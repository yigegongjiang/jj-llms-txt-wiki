> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referencia de hooks

> Referencia para eventos de hooks de Claude Code, esquema de configuración, formatos de entrada/salida JSON, códigos de salida, hooks asincronos, hooks HTTP, hooks de prompt y hooks de herramientas MCP.

<Tip>
  Para una guía de inicio rápido con ejemplos, consulte [Automatizar acciones con hooks](/docs/es/hooks-guide).
</Tip>

Los hooks son comandos de shell definidos por el usuario, puntos finales HTTP, llamadas a herramientas MCP, prompts de LLM o subagentes que se ejecutan automáticamente en puntos específicos del ciclo de vida de Claude Code. Claude Code dispara los mismos eventos de hooks dondequiera que se ejecute: sesiones en la terminal, extensiones de IDE, la [aplicación de escritorio](/docs/es/desktop-quickstart) y [Claude Code en la web](/docs/es/claude-code-on-the-web). Utilice esta referencia para buscar esquemas de eventos, opciones de configuración, formatos de entrada/salida JSON y características avanzadas como hooks asincronos, hooks HTTP y hooks de herramientas MCP.

<h2 id="hook-lifecycle">
  Ciclo de vida de los hooks
</h2>

Claude Code ejecuta hooks en puntos específicos durante una sesión. Cuando se activa un evento y un matcher coincide, Claude Code pasa contexto JSON sobre el evento a su controlador de hook. Para hooks de comando, la entrada llega en stdin. Para hooks HTTP, llega como el cuerpo de la solicitud POST. Su controlador puede entonces inspeccionar la entrada, tomar medidas y opcionalmente devolver una decisión.

Los eventos se dividen en tres cadencias:

* por sesión: `SessionStart` y `SessionEnd`
* por turno: `UserPromptSubmit`, `Stop` y `StopFailure`
* en cada llamada a herramienta dentro del bucle agentico: `PreToolUse` y `PostToolUse`, excepto las llamadas [`EndConversation`](/docs/es/tools-reference#endconversation-tool-behavior), que omiten ambas

<div style={{maxWidth: "500px", margin: "0 auto"}}>
  <Frame>
    <img src="https://mintcdn.com/claude-code/x7pO8l4XcvAXCoVc/images/hooks-lifecycle.svg?fit=max&auto=format&n=x7pO8l4XcvAXCoVc&q=85&s=81b9256c1bbe8832553485f5d9e9c746" className="dark:hidden" alt="Diagrama del ciclo de vida de hooks que muestra Setup opcional alimentando a SessionStart, luego un bucle por turno que contiene UserPromptSubmit, UserPromptExpansion para slash commands, el bucle agentico anidado (PreToolUse, PermissionRequest, PostToolUse, PostToolUseFailure, PostToolBatch, SubagentStart/Stop, TaskCreated, TaskCompleted), y Stop o StopFailure, seguido de TeammateIdle, PreCompact, PostCompact y SessionEnd, con Elicitation y ElicitationResult anidados dentro de la ejecución de herramientas MCP, PermissionDenied como una rama lateral de PermissionRequest para denegaciones en modo automático, WorktreeCreate, WorktreeRemove, Notification, ConfigChange, InstructionsLoaded, CwdChanged, FileChanged y DirectoryAdded como eventos asincronos independientes, PreModelSwitch como un evento secuencial independiente que se ejecuta antes de un cambio de modelo solicitado, PostModelSwitch como un evento asincrónico independiente que se ejecuta después de que cambia el modelo de la sesión, y MessageDisplay como un evento de solo visualización que se ejecuta mientras el texto del mensaje del asistente se transmite" width="520" height="1336" data-path="images/hooks-lifecycle.svg" />

    <img src="https://mintcdn.com/claude-code/x7pO8l4XcvAXCoVc/images/hooks-lifecycle-dark.svg?fit=max&auto=format&n=x7pO8l4XcvAXCoVc&q=85&s=c9b3d88487335f58cce0b52e2f9e7531" className="hidden dark:block" alt="Diagrama del ciclo de vida de hooks que muestra Setup opcional alimentando a SessionStart, luego un bucle por turno que contiene UserPromptSubmit, UserPromptExpansion para slash commands, el bucle agentico anidado (PreToolUse, PermissionRequest, PostToolUse, PostToolUseFailure, PostToolBatch, SubagentStart/Stop, TaskCreated, TaskCompleted), y Stop o StopFailure, seguido de TeammateIdle, PreCompact, PostCompact y SessionEnd, con Elicitation y ElicitationResult anidados dentro de la ejecución de herramientas MCP, PermissionDenied como una rama lateral de PermissionRequest para denegaciones en modo automático, WorktreeCreate, WorktreeRemove, Notification, ConfigChange, InstructionsLoaded, CwdChanged, FileChanged y DirectoryAdded como eventos asincronos independientes, PreModelSwitch como un evento secuencial independiente que se ejecuta antes de un cambio de modelo solicitado, PostModelSwitch como un evento asincrónico independiente que se ejecuta después de que cambia el modelo de la sesión, y MessageDisplay como un evento de solo visualización que se ejecuta mientras el texto del mensaje del asistente se transmite" width="520" height="1336" data-path="images/hooks-lifecycle-dark.svg" />
  </Frame>
</div>

La tabla a continuación resume cuándo se activa cada evento. La sección [Hook events](#hook-events) documenta el esquema de entrada completo y las opciones de control de decisión para cada uno.

| Evento                | Cuándo se dispara                                                                                                                                                                                                                                                                                                          |
| :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `SessionStart`        | Cuando una sesión comienza o se reanuda                                                                                                                                                                                                                                                                                    |
| `Setup`               | Cuando inicia Claude Code con `--init-only`, o con `--init` o `--maintenance` en modo `-p`. Para preparación única en CI o scripts                                                                                                                                                                                         |
| `UserPromptSubmit`    | Cuando envía un prompt, antes de que Claude lo procese                                                                                                                                                                                                                                                                     |
| `UserPromptExpansion` | Cuando un comando escrito por el usuario se expande en un prompt, antes de que llegue a Claude. Puede bloquear la expansión                                                                                                                                                                                                |
| `PreToolUse`          | Antes de que se ejecute una llamada a herramienta. Puede bloquearlo                                                                                                                                                                                                                                                        |
| `PermissionRequest`   | Cuando una llamada a herramienta necesita una decisión de permiso                                                                                                                                                                                                                                                          |
| `PermissionDenied`    | Cuando el modo automático deniega una llamada a herramienta, incluidas las denegaciones sin un veredicto del clasificador. Use JSON `hookSpecificOutput.retry: true` para indicar al modelo que puede reintentar la llamada a herramienta denegada. Claude Code ignora `retry` cuando el clasificador no produjo veredicto |
| `PostToolUse`         | Después de que una llamada a herramienta se ejecuta correctamente                                                                                                                                                                                                                                                          |
| `PostToolUseFailure`  | Después de que una llamada a herramienta falla                                                                                                                                                                                                                                                                             |
| `PostToolBatch`       | Después de que se resuelve un lote completo de llamadas a herramientas paralelas, antes de la siguiente llamada al modelo                                                                                                                                                                                                  |
| `Notification`        | Cuando Claude Code envía una notificación                                                                                                                                                                                                                                                                                  |
| `MessageDisplay`      | Mientras se muestra el texto del mensaje del asistente                                                                                                                                                                                                                                                                     |
| `SubagentStart`       | Cuando se genera un subagente                                                                                                                                                                                                                                                                                              |
| `SubagentStop`        | Cuando un subagente finaliza                                                                                                                                                                                                                                                                                               |
| `TaskCreated`         | Cuando se está creando una tarea a través de `TaskCreate`                                                                                                                                                                                                                                                                  |
| `TaskCompleted`       | Cuando se marca una tarea como completada                                                                                                                                                                                                                                                                                  |
| `Stop`                | Cuando Claude termina de responder                                                                                                                                                                                                                                                                                         |
| `StopFailure`         | Cuando el turno termina debido a un error de API                                                                                                                                                                                                                                                                           |
| `TeammateIdle`        | Cuando un compañero de [equipo de agentes](/docs/es/agent-teams) está a punto de quedarse inactivo                                                                                                                                                                                                                              |
| `InstructionsLoaded`  | Cuando se carga un archivo CLAUDE.md o `.claude/rules/*.md` en el contexto. Se dispara al inicio de la sesión y cuando los archivos se cargan de forma diferida durante una sesión                                                                                                                                         |
| `ConfigChange`        | Cuando un archivo de configuración cambia durante una sesión                                                                                                                                                                                                                                                               |
| `CwdChanged`          | Cuando el directorio de trabajo cambia, por ejemplo cuando Claude ejecuta un comando `cd`. Útil para la gestión reactiva del entorno con herramientas como direnv                                                                                                                                                          |
| `DirectoryAdded`      | Cuando se agrega un directorio de trabajo a mitad de sesión a través de `/add-dir` o la solicitud de control `register_repo_root` del SDK                                                                                                                                                                                  |
| `FileChanged`         | Cuando un archivo observado cambia en el disco. El campo `matcher` especifica qué nombres de archivo observar                                                                                                                                                                                                              |
| `WorktreeCreate`      | Cuando se está creando un worktree a través de `--worktree`, `isolation: "worktree"`, o para una sesión en segundo plano. Reemplaza el comportamiento predeterminado de git                                                                                                                                                |
| `WorktreeRemove`      | Cuando se está eliminando un worktree al salir de la sesión, cuando un subagente finaliza, o cuando elimina una sesión en segundo plano                                                                                                                                                                                    |
| `PreCompact`          | Antes de la compactación de contexto                                                                                                                                                                                                                                                                                       |
| `PostCompact`         | Después de que se completa la compactación de contexto                                                                                                                                                                                                                                                                     |
| `PreModelSwitch`      | Antes de que Claude Code aplique un cambio de modelo que usted o un cliente solicitó. Puede bloquear el cambio                                                                                                                                                                                                             |
| `PostModelSwitch`     | Después de que cambia el modelo de la sesión, incluidos los cambios que Claude Code realiza por su cuenta, como restaurar el modelo cuando reanuda una sesión                                                                                                                                                              |
| `Elicitation`         | Cuando un servidor MCP solicita entrada del usuario durante una llamada a herramienta                                                                                                                                                                                                                                      |
| `ElicitationResult`   | Después de que un usuario responde a una solicitud de MCP, antes de que la respuesta se envíe de vuelta al servidor                                                                                                                                                                                                        |
| `SessionEnd`          | Cuando una sesión termina                                                                                                                                                                                                                                                                                                  |

<h3 id="how-a-hook-resolves">
  Cómo se resuelve un hook
</h3>

Para ver cómo encajan el evento, el matcher y el controlador, considere este hook `PreToolUse` que bloquea comandos de shell destructivos.

<Tabs>
  <Tab title="macOS/Linux">
    El `matcher` se reduce a llamadas a herramientas Bash y la condición `if` se reduce aún más a subcomandos Bash que coinciden con `rm *`, por lo que `block-rm.sh` solo se genera cuando ambos filtros coinciden:

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

    El script lee la entrada JSON desde stdin, extrae el comando y devuelve una `permissionDecision` de `"deny"` si contiene `rm -rf`. Guárdelo en `.claude/hooks/block-rm.sh` en su proyecto y hágalo ejecutable con `chmod +x .claude/hooks/block-rm.sh` para que Claude Code pueda ejecutarlo:

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

    Este script, como los otros ejemplos de Bash en esta página que analizan entrada JSON, utiliza `jq`, así que instale `jq` y asegúrese de que esté en su `PATH` antes de intentarlos.
  </Tab>

  <Tab title="Windows (PowerShell)">
    El matcher `Bash|PowerShell` cubre la [herramienta PowerShell](#powershell) así como Bash. Una única regla `if` coincide solo con las llamadas de una herramienta, por lo que cada herramienta obtiene su propio controlador: el primero se reduce a subcomandos Bash que coinciden con `rm *`, el segundo a comandos PowerShell que coinciden con `Remove-Item *`. Ambos ejecutan el mismo script a través de `powershell.exe`:

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash|PowerShell",
            "hooks": [
              {
                "type": "command",
                "if": "Bash(rm *)",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.ps1"
                ]
              },
              {
                "type": "command",
                "if": "PowerShell(Remove-Item *)",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.ps1"
                ]
              }
            ]
          }
        ]
      }
    }
    ```

    La bandera `-NoProfile` omite cargar su perfil de PowerShell para que el hook se inicie rápidamente, y `-ExecutionPolicy Bypass` permite que PowerShell ejecute el archivo de script local.

    El script lee la entrada JSON desde stdin, extrae el comando y devuelve una `permissionDecision` de `"deny"` si contiene `rm -rf` o `Remove-Item` seguido de `-Recurse`. Guárdelo en `.claude/hooks/block-rm.ps1` en su proyecto:

    ```powershell theme={null}
    # .claude/hooks/block-rm.ps1
    $callInput = [Console]::In.ReadToEnd() | ConvertFrom-Json
    $command = $callInput.tool_input.command

    if ($command -match 'rm -rf|Remove-Item.*-Recurse') {
      @{
        hookSpecificOutput = @{
          hookEventName = "PreToolUse"
          permissionDecision = "deny"
          permissionDecisionReason = "Destructive command blocked by hook"
        }
      } | ConvertTo-Json
    } else {
      exit 0  # no decision; normal permission flow applies
    }
    ```
  </Tab>
</Tabs>

Ahora suponga que Claude Code decide ejecutar `Bash "rm -rf /tmp/build"` contra la configuración de macOS/Linux. Esto es lo que sucede:

<Frame>
  <img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/hook-resolution.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=be0bf3053550c26de5f54cd64674c197" className="dark:hidden" alt="Diagrama de resolución de hooks: PreToolUse se activa, el matcher verifica la coincidencia de Bash, luego la condición if verifica la coincidencia de Bash(rm *). Si ambos coinciden, el comando del hook se ejecuta y devuelve permissionDecision deny, por lo que la llamada a la herramienta se bloquea y Claude Code continúa. Si alguna verificación no coincide, el hook se omite y la llamada a la herramienta se permite continuar." width="930" height="270" data-path="images/hook-resolution.svg" />

  <img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/hook-resolution-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=e80af91f8507cee6bd51ac3c2dd92f63" className="hidden dark:block" alt="Diagrama de resolución de hooks: PreToolUse se activa, el matcher verifica la coincidencia de Bash, luego la condición if verifica la coincidencia de Bash(rm *). Si ambos coinciden, el comando del hook se ejecuta y devuelve permissionDecision deny, por lo que la llamada a la herramienta se bloquea y Claude Code continúa. Si alguna verificación no coincide, el hook se omite y la llamada a la herramienta se permite continuar." width="930" height="270" data-path="images/hook-resolution-dark.svg" />
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

1. Elige un [evento de hook](#hook-events) al que responder, como `PreToolUse` o `Stop`
2. Añade un [grupo de matcher](#matcher-patterns) para filtrar cuándo se activa, como "solo para la herramienta Bash"
3. Define uno o más [manejadores de hook](#hook-handler-fields) para ejecutar cuando coincida

Consulta [Cómo se resuelve un hook](#how-a-hook-resolves) arriba para un recorrido completo con un ejemplo anotado.

<Note>
  Esta página utiliza términos específicos para cada nivel: **evento de hook** para el punto del ciclo de vida, **grupo de matcher** para el filtro, y **manejador de hook** para el comando de shell, punto final HTTP, herramienta MCP, prompt o agente que se ejecuta. "Hook" por sí solo se refiere a la característica general.
</Note>

<h3 id="hook-locations">
  Ubicaciones de hooks
</h3>

Dónde definas un hook determina su alcance:

| Ubicación                                         | Alcance                                                                                                                 | Compartible                                                            |
| :------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------- |
| `~/.claude/settings.json`                         | Todos tus proyectos                                                                                                     | No, local en tu máquina                                                |
| `.claude/settings.json`                           | Proyecto único                                                                                                          | Sí, puede confirmarse en el repositorio                                |
| `.claude/settings.local.json`                     | Proyecto único                                                                                                          | No, ignorado por git cuando Claude Code guarda una configuración en él |
| Configuración de política administrada            | Toda la organización                                                                                                    | Sí, controlado por administrador                                       |
| [Plugin](/docs/es/plugins/overview) `hooks/hooks.json` | Cuando el plugin está habilitado                                                                                        | Sí, incluido con el plugin                                             |
| [Skill](/docs/es/skills) frontmatter                   | El resto de la sesión una vez que se invoca la skill. Consulta [Hooks en skills y agentes](#hooks-in-skills-and-agents) | Sí, definido en el archivo de skill                                    |
| [Subagente](/docs/es/sub-agents) frontmatter           | Mientras ese subagente se está ejecutando                                                                               | Sí, definido en el archivo del subagente                               |

Las [sesiones en la nube](/docs/es/claude-code-on-the-web) no leen tu `~/.claude/settings.json` local. En un [entorno autohospedado](/docs/es/self-hosted-environments-configuration#permissions-and-tool-approval), Claude Code también ejecuta los hooks que el operador sembró desde `~/.claude/` del host del ejecutor, y ejecuta los hooks en el archivo de configuración administrada de la imagen del ejecutor cuando ese archivo está entre las [fuentes administradas que Claude Code aplica](/docs/es/managed-settings#how-claude-code-combines-managed-sources), lo que por defecto significa solo cuando ni la configuración administrada por servidor ni una política de Claude Code entregada por MDM suministran el nivel administrado. Consulta [qué se transfiere de tu configuración](/docs/es/cloud-environments#what-carries-over-from-your-setup) para saber qué archivos de configuración y plugins, y por lo tanto qué hooks, llegan a una sesión en la nube.

Para obtener detalles sobre la resolución de archivos de configuración, consulta [configuración](/docs/es/settings).

Los hooks de archivos de configuración, configuración de política administrada y plugins también se ejecutan dentro de [subagentes](/docs/es/sub-agents). Cuando un subagente llama a una herramienta, eventos de herramienta como `PreToolUse` y `PostToolUse` activan los mismos hooks configurados que en la conversación principal, y la entrada lleva los campos de entrada comunes `agent_id` y `agent_type` [](#common-input-fields) que identifican al subagente.

Los administradores empresariales pueden usar `allowManagedHooksOnly` para restringir qué hooks se ejecutan:

* Tus hooks de usuario, proyecto, local y plugin están bloqueados. Los hooks de plugins forzados a habilitarse en la configuración administrada `enabledPlugins` están exentos
* Claude Code también reduce tu configuración [`statusLine`](/docs/es/statusline), [`fileSuggestion`](/docs/es/settings-reference#filesuggestion) y [`subagentStatusLine`](/docs/es/statusline#subagent-status-lines) a la configuración administrada
* Claude Code también deshabilita plugins con una [fuente `command`](/docs/es/plugins/marketplace-reference#command-plugin-source), incluidos los plugins forzados a habilitarse en la configuración administrada `enabledPlugins`, a menos que [`disableCommandPluginSources`](/docs/es/settings-reference#disablecommandpluginsources) esté explícitamente establecido en `false`. Las fuentes `command` requieren Claude Code v2.1.229 o posterior
* Claude Code también bloquea los comandos [`headersHelper`](/docs/es/plugins/host-marketplace#authenticate-archive-downloads) del marketplace a menos que [`disableCommandPluginSources`](/docs/es/settings-reference#disablecommandpluginsources) esté explícitamente establecido en `false`, excepto para un marketplace que la propia configuración administrada declare

Consulta [qué se ejecuta bajo `allowManagedHooksOnly`](/docs/es/settings-reference#what-runs-under-allowmanagedhooksonly).

Las entradas de hook se fusionan entre niveles de configuración en lugar de reemplazarse entre sí: la configuración de usuario, proyecto y local añaden sus propios hooks sin eliminar los administrados, y la configuración [`disableAllHooks`](#disable-or-remove-hooks) no puede deshabilitar hooks administrados desde fuera de la configuración administrada.

Las [listas de permitidos de hooks HTTP](/docs/es/settings-reference#hook-and-skill-settings) se aplican a hooks de todas las fuentes, incluida la configuración de política administrada:

* `allowedHttpHookUrls`: cuando se define en cualquier nivel de configuración, Claude Code ejecuta un manejador de hook HTTP solo si su URL coincide con la lista de permitidos fusionada
* `httpHookAllowedEnvVars`: cuando se define, Claude Code interpola solo las variables de entorno en esa lista en los encabezados de hook

<h3 id="matcher-patterns">
  Patrones de matcher
</h3>

El campo `matcher` filtra cuándo se activan los hooks. Cómo se evalúa un matcher depende de los caracteres que contiene:

| Valor de matcher                                     | Evaluado como                                                                                                  | Ejemplo                                                                                                                                                                                            |
| :--------------------------------------------------- | :------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"*"`, `""` u omitido                                | Coincidir con todo                                                                                             | se activa en cada ocurrencia del evento                                                                                                                                                            |
| Solo letras, dígitos, `_`, `-`, espacios, `,` y `\|` | Cadena exacta, o lista de cadenas exactas separadas por `\|` o `,` con espacios en blanco opcionales alrededor | `Bash` coincide solo con la herramienta Bash; `Edit\|Write` y `Edit, Write` cada una coincide con cualquiera de las herramientas exactamente; `code-reviewer` coincide solo con ese tipo de agente |
| Contiene cualquier otro carácter                     | Expresión regular de JavaScript, sin anclar                                                                    | `^Notebook` coincide con cualquier herramienta cuyo nombre comienza con `Notebook`; `mcp__memory__.*` coincide con cada herramienta del servidor `memory`                                          |

Un matcher en la ruta de expresión regular se prueba con `RegExp.prototype.test` de JavaScript, que tiene éxito en una coincidencia en cualquier lugar del valor. `Edit.*` coincide tanto con `Edit` como con `NotebookEdit`; envuelve el patrón en `^` y `$`, como en `^Edit$`, cuando necesites una coincidencia de cadena completa.

Los guiones en el conjunto de coincidencia exacta requieren Claude Code v2.1.195 o posterior. En versiones anteriores, un nombre con guiones como `code-reviewer` se evalúa como una expresión regular sin anclar, por lo que también se activa para `senior-code-reviewer`; anclalo como `^code-reviewer$` en esas versiones para coincidir solo con ese nombre.

`FileChanged` y `StopFailure` utilizan un conjunto de coincidencia exacta más estrecho de solo letras, dígitos, `_` y `|`. Un guión, espacio o coma en un matcher para esos dos eventos lo mantiene en la ruta de expresión regular, y solo `|` separa alternativas. Todos los demás eventos con soporte de matcher en la tabla que sigue aceptan `|` o `,`.

El evento `FileChanged` no sigue estas reglas al construir su lista de vigilancia. Consulta [FileChanged](#filechanged).

Cada tipo de evento coincide en un campo diferente:

| Evento                                                                                                                                            | Qué filtra el matcher                                                                                     | Valores de matcher de ejemplo                                                                                                                                                                                                                                                  |
| :------------------------------------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`                                                        | nombre de la herramienta                                                                                  | `Bash`, `Edit\|Write`, `mcp__.*`                                                                                                                                                                                                                                               |
| `SessionStart`                                                                                                                                    | cómo comenzó la sesión                                                                                    | `startup`, `resume`, `clear`, `compact`, `fork`                                                                                                                                                                                                                                |
| `Setup`                                                                                                                                           | qué bandera CLI activó la configuración                                                                   | `init`, `maintenance`                                                                                                                                                                                                                                                          |
| `SessionEnd`                                                                                                                                      | por qué terminó la sesión                                                                                 | `clear`, `resume`, `logout`, `prompt_input_exit`, `other`                                                                                                                                                                                                                      |
| `Notification`                                                                                                                                    | tipo de notificación                                                                                      | `permission_prompt`, `idle_prompt`, `auth_success`, `elicitation_dialog`, `elicitation_url_dialog`, `elicitation_complete`, `elicitation_response`, `agent_needs_input`, `agent_completed`, `quota_auto_resume_fired`, `quota_auto_resume_stale`, `quota_auto_resume_disabled` |
| `SubagentStart`                                                                                                                                   | tipo de agente                                                                                            | `general-purpose`, `Explore`, `Plan`, nombres de agentes personalizados, o nombres con alcance de plugin como `^my-plugin:reviewer$`                                                                                                                                           |
| `PreCompact`, `PostCompact`                                                                                                                       | qué activó la compactación                                                                                | `manual`, `auto`                                                                                                                                                                                                                                                               |
| `PreModelSwitch`, `PostModelSwitch`                                                                                                               | nombre canónico del modelo al que cambia la sesión, como se describe en [PreModelSwitch](#premodelswitch) | `claude-opus-5`, `claude-opus-4-6\|claude-opus-5`, `.*opus.*`                                                                                                                                                                                                                  |
| `SubagentStop`                                                                                                                                    | tipo de agente                                                                                            | los mismos valores que `SubagentStart`                                                                                                                                                                                                                                         |
| `ConfigChange`                                                                                                                                    | fuente de configuración                                                                                   | `user_settings`, `project_settings`, `local_settings`, `policy_settings`, `skills`                                                                                                                                                                                             |
| `CwdChanged`                                                                                                                                      | sin soporte de matcher                                                                                    | siempre se activa en cada ocurrencia                                                                                                                                                                                                                                           |
| `DirectoryAdded`                                                                                                                                  | cómo se añadió el directorio                                                                              | `slash_command`, `register_repo_root`                                                                                                                                                                                                                                          |
| `FileChanged`                                                                                                                                     | nombres de archivo literales a vigilar (consulta [FileChanged](#filechanged))                             | `.envrc\|.env`                                                                                                                                                                                                                                                                 |
| `StopFailure`                                                                                                                                     | tipo de error                                                                                             | `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `account_on_hold`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `cloud_credential_error`, `unknown`                                               |
| `InstructionsLoaded`                                                                                                                              | razón de carga                                                                                            | `session_start`, `nested_traversal`, `path_glob_match`, `include`, `compact`                                                                                                                                                                                                   |
| `UserPromptExpansion`                                                                                                                             | nombre del comando                                                                                        | tus nombres de skill o comando                                                                                                                                                                                                                                                 |
| `Elicitation`                                                                                                                                     | nombre del servidor MCP                                                                                   | tus nombres de servidor MCP configurados                                                                                                                                                                                                                                       |
| `ElicitationResult`                                                                                                                               | nombre del servidor MCP                                                                                   | los mismos valores que `Elicitation`                                                                                                                                                                                                                                           |
| `UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `MessageDisplay` | sin soporte de matcher                                                                                    | siempre se activa en cada ocurrencia                                                                                                                                                                                                                                           |

Hacer coincidir `StopFailure` en `cloud_credential_error` requiere Claude Code v2.1.267 o posterior, la primera versión que reporta fallos de carga de credenciales bajo ese valor en lugar de `server_error` o `unknown`.

Para la mayoría de eventos, Claude Code evalúa el matcher contra un campo de la [entrada JSON](#hook-input-and-output) que envía a tu hook en stdin. Para eventos de herramienta, ese campo es `tool_name`. Para `PreModelSwitch` y `PostModelSwitch`, Claude Code evalúa el matcher contra el nombre canónico que deriva de `to_model`, como se describe en [PreModelSwitch](#premodelswitch). Cada sección de [evento de hook](#hook-events) lista el conjunto completo de valores de matcher y el esquema de entrada para ese evento.

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

Si añades un campo `matcher` a un evento sin soporte de matcher, se ignora silenciosamente.

Para eventos de herramienta, puedes filtrar más estrictamente estableciendo el campo [`if`](#common-fields) en manejadores de hook individuales. `if` utiliza [sintaxis de regla de permisos](/docs/es/permissions) para coincidir contra el nombre de la herramienta y los argumentos juntos, por lo que `"Bash(git *)"` se ejecuta cuando cualquier subcomando de la entrada de Bash coincide con `git *` y `"Edit(*.ts)"` se ejecuta solo para archivos TypeScript.

<h4 id="match-mcp-tools">
  Coincidir herramientas MCP
</h4>

Las herramientas del servidor [MCP](/docs/es/mcp) aparecen como herramientas regulares en eventos de herramienta (`PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`), por lo que puedes hacerlas coincidir de la misma manera que cualquier otro nombre de herramienta.

Las herramientas MCP siguen el patrón de nomenclatura `mcp__<server>__<tool>`, por ejemplo:

* `mcp__memory__create_entities`: herramienta crear entidades del servidor Memory
* `mcp__filesystem__read_file`: herramienta leer archivo del servidor Filesystem
* `mcp__github__search_repositories`: herramienta de búsqueda del servidor GitHub

Para coincidir con cada herramienta de un servidor, añade `.*` al prefijo del servidor. El `.*` es obligatorio: un matcher como `mcp__memory` o `mcp__brave-search` contiene solo caracteres de coincidencia exacta, por lo que se compara como una cadena exacta y no coincide con ninguna herramienta.

* `mcp__memory__.*` coincide con todas las herramientas del servidor `memory`
* `mcp__brave-search__.*` coincide con todas las herramientas de un servidor cuyo nombre contiene un guión
* `mcp__.*__write.*` coincide con cualquier herramienta cuyo nombre comienza con `write` de cualquier servidor

Los guiones en el conjunto de coincidencia exacta requieren Claude Code v2.1.195 o posterior. En versiones anteriores, un prefijo con guiones desnudo como `mcp__brave-search` se evalúa como una expresión regular sin anclar y coincide con cada herramienta de ese servidor. La forma `mcp__brave-search__.*` funciona en cada versión.

Las herramientas de un [servidor MCP incluido en plugin](/docs/es/mcp#plugin-provided-mcp-servers) utilizan un segmento de servidor con alcance que incluye el nombre del plugin: `mcp__plugin_<plugin-name>_<server-name>__<tool>`. Un matcher escrito contra la clave del servidor desnuda nunca se activa para estas herramientas. Para un plugin llamado `my-plugin` que incluye un servidor bajo la clave `db`, una herramienta `query` aparece como `mcp__plugin_my-plugin_db__query`, por lo que el matcher para cada herramienta de ese servidor es `mcp__plugin_my-plugin_db__.*`. Utiliza el mismo nombre de herramienta con alcance en el campo [`if`](#common-fields) de un manejador. Consulta [Servidores MCP incluidos en plugin](/docs/es/mcp#plugin-provided-mcp-servers) para saber cómo se construye el nombre con alcance.

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
  Campos de manejador de hook
</h3>

Cada objeto en el array `hooks` interno es un manejador de hook: el comando de shell, punto final HTTP, herramienta MCP, prompt LLM o agente que se ejecuta cuando el matcher coincide. Hay cinco tipos:

* **[Hooks de comando](#command-hook-fields)** (`type: "command"`): ejecutan un comando de shell. Tu script recibe la [entrada JSON](#hook-input-and-output) del evento en stdin y comunica resultados de vuelta a través de códigos de salida y stdout.
* **[Hooks HTTP](#http-hook-fields)** (`type: "http"`): envían la entrada JSON del evento como una solicitud HTTP POST a una URL. El punto final comunica resultados de vuelta a través del cuerpo de respuesta utilizando el mismo [formato de salida JSON](#json-output) que los hooks de comando.
* **[Hooks de herramienta MCP](#mcp-tool-hook-fields)** (`type: "mcp_tool"`): llaman a una herramienta en un [servidor MCP](/docs/es/mcp) ya conectado. La salida de texto de la herramienta se trata como stdout de hook de comando.
* **[Hooks de prompt](#prompt-and-agent-hook-fields)** (`type: "prompt"`): envían un prompt a un modelo Claude para evaluación de un solo turno. El modelo devuelve su decisión como JSON. Consulta [Hooks basados en prompt](#prompt-based-hooks).
* **[Hooks de agente](#prompt-and-agent-hook-fields)** (`type: "agent"`): generan un subagente que puede usar herramientas como Read, Grep y Glob para verificar condiciones antes de devolver una decisión. Los hooks de agente son experimentales y pueden cambiar. Consulta [Hooks basados en agente](#agent-based-hooks).

Todos los hooks coincidentes se ejecutan en paralelo. Si defines el mismo manejador en más de un archivo de configuración, se ejecuta una vez. Una copia del mismo manejador de un plugin o skill se mantiene separada.

Los manejadores se ejecutan en el directorio actual con el entorno de Claude Code. Si el directorio actual ya no existe, por ejemplo un worktree o directorio temporal que otro shell eliminó a mitad de sesión, Claude Code ejecuta hooks de comando desde el primero de estos que aún existe: el directorio en el que comenzó la sesión, la raíz del proyecto, tu directorio de inicio o el directorio temporal del sistema. Claude Code registra una advertencia nombrando el directorio de respaldo en el [registro de depuración](#debug-hooks).

La variable de entorno `$CLAUDE_CODE_REMOTE` es `"true"` en entornos web remotos y no está establecida en la CLI local. Claude Code v2.1.199 y posterior establece [`$CLAUDE_CODE_BRIDGE_SESSION_ID`](/docs/es/env-vars) en el ID de sesión de [Control Remoto](/docs/es/remote-control) mientras la sesión local tiene una conexión activa de Control Remoto.

<h4 id="common-fields">
  Campos comunes
</h4>

Estos campos se aplican a todos los tipos de hook:

| Campo           | Requerido | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| :-------------- | :-------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`          | sí        | `"command"`, `"http"`, `"mcp_tool"`, `"prompt"` o `"agent"`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `if`            | no        | Sintaxis de regla de permisos para filtrar cuándo se ejecuta este hook, como `"Bash(git *)"` o `"Edit(*.ts)"`. El comando de hook solo se ejecuta si la llamada de herramienta coincide con el patrón. Consulta la tabla [Bash matching](#bash-if-matching) a continuación para saber cómo los patrones de Bash se evalúan contra subcomandos, `$()` y backticks. Solo se evalúa en eventos de herramienta: `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` y `PermissionDenied`. En otros eventos, un hook con `if` establecido nunca se ejecuta. Utiliza la misma sintaxis que [reglas de permisos](/docs/es/permissions)                                                                                |
| `timeout`       | no        | Segundos antes de cancelar. Claude Code no lo aplica en un hook de comando que ejecutas con [`async: true`](#run-hooks-in-the-background). Valores por defecto: 600 para `command`, `http` y `mcp_tool`; 30 para `prompt`; 60 para `agent`. Claude Code reduce el valor por defecto de `command`, `http` y `mcp_tool` a 30 en [`UserPromptSubmit`](#userpromptsubmit), [`PreModelSwitch`](#premodelswitch) y [`PostModelSwitch`](#postmodelswitch), y a 10 en [`MessageDisplay`](#messagedisplay). Los hooks de [`SessionEnd`](#sessionend) comparten un presupuesto de 1.5 segundos; si tu configuración establece un `timeout` por hook más largo, Claude Code aumenta el presupuesto para que coincida, hasta 60 segundos |
| `statusMessage` | no        | Mensaje de spinner personalizado mostrado mientras se ejecuta el hook                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `once`          | no        | Si es `true`, Claude Code elimina el hook después de su primera ejecución exitosa. Una ejecución que falla, bloquea con código de salida 2 o agota el tiempo de espera deja el hook en su lugar, por lo que se ejecuta de nuevo en el siguiente evento coincidente. Solo se respeta para hooks declarados en [frontmatter de skill](#hooks-in-skills-and-agents); se ignora en archivos de configuración y frontmatter de agente                                                                                                                                                                                                                                                                                             |

El campo `if` contiene exactamente una regla de permisos. No hay sintaxis `&&`, `||` o de lista para combinar reglas; para aplicar múltiples condiciones, define un manejador de hook separado para cada una.

En una condición `if` para una herramienta de archivo, un patrón de directorio de un solo segmento como `"Edit(src/**)"` coincide solo con el directorio `src` en el directorio de trabajo y los archivos bajo él. Para coincidir con un directorio llamado `src` a cualquier profundidad, escribe `"Edit(**/src/**)"`. Antes de v2.1.214, `"Edit(src/**)"` coincidía con un directorio llamado `src` a cualquier profundidad bajo el directorio de trabajo.

<span id="bash-if-matching" />Para patrones de Bash, si tu comando de hook se ejecuta depende de la forma del patrón y del comando de Bash que Claude está invocando. Las asignaciones `VAR=value` iniciales se eliminan antes de hacer coincidir.

| Patrón `if`        | Comando de Bash             | ¿Se ejecuta el hook? | Por qué                                                                                                                                            |
| :----------------- | :-------------------------- | :------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Bash(git *)`      | `FOO=bar git push`          | sí                   | las asignaciones iniciales se eliminan; `git push` coincide                                                                                        |
| `Bash(git *)`      | `npm test && git push`      | sí                   | cada subcomando se verifica; `git push` coincide                                                                                                   |
| `Bash(rm *)`       | `echo $(rm -rf /)`          | sí                   | los comandos dentro de `$()` y backticks se verifican; `rm -rf /` coincide                                                                         |
| `Bash(rm *)`       | `echo $(date)`              | no                   | ningún subcomando coincide con `rm *`                                                                                                              |
| `Bash(cat *)`      | `echo before $(date) after` | no                   | una sustitución puede estar en cualquier posición de argumento, por lo que se verifican el comando completo y `date`; ninguno coincide con `cat *` |
| `Bash(git *)`      | `$TOOL git push`            | sí                   | Claude Code no puede saber a qué se expande el nombre del comando, por lo que ejecuta el hook                                                      |
| `Bash(git push *)` | `echo $(date)`              | sí                   | los patrones que especifican más que el nombre del comando ejecutan el hook de todas formas en `$()`, backticks o `$VAR`                           |

Cuando Claude Code no puede determinar qué comandos ejecuta la entrada de Bash, ejecuta tu hook independientemente del patrón. Porque el filtro `if` es de mejor esfuerzo, utiliza el [sistema de permisos](/docs/es/permissions) en lugar de un hook para aplicar una autorización o denegación dura.

<h4 id="command-hook-fields">
  Campos de hook de comando
</h4>

Además de los [campos comunes](#common-fields), los hooks de comando aceptan estos campos:

| Campo         | Requerido | Descripción                                                                                                                                                                                                                                                                                                                                                                     |
| :------------ | :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `command`     | sí        | Comando de shell a ejecutar. Con `args`, el ejecutable a generar directamente. Consulta [Forma exec y forma shell](#exec-form-and-shell-form)                                                                                                                                                                                                                                   |
| `args`        | no        | Lista de argumentos. Cuando está presente, `command` se resuelve como un ejecutable y se genera directamente con `args` como el vector de argumentos, sin shell involucrado. Consulta [Forma exec y forma shell](#exec-form-and-shell-form)                                                                                                                                     |
| `async`       | no        | Si es `true`, se ejecuta en segundo plano sin bloquear. Consulta [Ejecutar hooks en segundo plano](#run-hooks-in-the-background)                                                                                                                                                                                                                                                |
| `asyncRewake` | no        | Si es `true`, se ejecuta en segundo plano y despierta a Claude en código de salida 2. El stderr del hook, o stdout si stderr está vacío, se muestra a Claude como un recordatorio del sistema para que pueda reaccionar a un fallo de fondo de larga duración                                                                                                                   |
| `shell`       | no        | Shell a usar para este hook. Acepta `"bash"` o `"powershell"`. Por defecto es `"bash"`, o `"powershell"` en Windows cuando Git Bash no está instalado. Establecer `"powershell"` ejecuta el comando a través de PowerShell en Windows. No requiere `CLAUDE_CODE_USE_POWERSHELL_TOOL` ya que los hooks generan PowerShell directamente. Se ignora cuando `args` está establecido |

<a id="exec-form-and-shell-form" />

<h5 id="exec-form-and-shell-form">
  Forma exec y forma shell
</h5>

Un hook de comando se ejecuta como forma exec cuando `args` está establecido, y forma shell cuando `args` se omite. Establece `args` siempre que el hook haga referencia a un [marcador de posición de ruta](#reference-scripts-by-path), ya que cada elemento se pasa como un argumento. Omite `args` cuando necesites características de shell como pipes o `&&`, o cuando ninguna preocupación se aplique.

**Forma exec** se ejecuta cuando `args` está presente. Claude Code resuelve `command` como un ejecutable en `PATH` y lo genera directamente con `args` como el vector de argumentos. No hay shell, por lo que cada elemento de `args` es exactamente un argumento tal como está escrito, y los marcadores de posición de ruta como `${CLAUDE_PLUGIN_ROOT}` se sustituyen en `command` y en cada elemento de `args` como cadenas simples. Los caracteres especiales como apóstrofes, `$` y backticks pasan sin cambios porque no hay shell para interpretarlos. No ocurre tokenización de shell en ninguna plataforma.

**Forma shell** se ejecuta cuando `args` se omite. La cadena `command` se pasa a un shell: `sh -c` en macOS y Linux, Git Bash en Windows, o PowerShell cuando Git Bash no está instalado. Establece el campo `shell` para elegir explícitamente. El shell tokeniza la cadena, expande variables e interpreta pipes, `&&`, redirecciones y globs.

<Note>
  En Windows, la forma exec requiere que `command` se resuelva en un ejecutable real como `.exe`. Los shims `.cmd` y `.bat` que npm, npx, eslint y otras herramientas instalan en `node_modules/.bin` no son ejecutables y no se pueden generar sin un shell. Para ejecutarlos en forma exec, invoca el script subyacente con `node` directamente, por ejemplo `"command": "node", "args": ["${CLAUDE_PLUGIN_ROOT}/node_modules/eslint/bin/eslint.js"]`. El patrón `node` más ruta de script funciona en cada plataforma porque `node.exe` es un binario real. Para ejecutar un shim `.cmd` o `.bat` por nombre, usa forma shell.
</Note>

Este ejemplo ejecuta un script de Node incluido con un plugin. La forma exec pasa la ruta de script resuelta como un argumento sin comillas:

```json theme={null}
{
  "type": "command",
  "command": "node",
  "args": ["${CLAUDE_PLUGIN_ROOT}/scripts/format.js", "--fix"]
}
```

La forma shell equivalente necesita comillas para manejar rutas con espacios o caracteres especiales:

```json theme={null}
{
  "type": "command",
  "command": "node \"${CLAUDE_PLUGIN_ROOT}\"/scripts/format.js --fix"
}
```

Ambas formas soportan los mismos [marcadores de posición de ruta](#reference-scripts-by-path), y ambas los exportan como variables de entorno `CLAUDE_PROJECT_DIR`, `CLAUDE_PLUGIN_ROOT` y `CLAUDE_PLUGIN_DATA` en el proceso generado, por lo que un script puede leer `process.env.CLAUDE_PLUGIN_ROOT` independientemente de cómo se haya lanzado.

Los hooks de plugin además sustituyen valores [`${user_config.*}`](/docs/es/plugins/manifest-reference#user-configuration), solo en forma exec: el valor se sustituye en `command` y en cada elemento de `args` como una cadena simple, por lo que ningún shell lo re-analiza.

Un hook de plugin en forma shell cuyo `command` hace referencia a `${user_config.*}` falla con un [error](/docs/es/errors#plugin-command-references-user-config) en lugar de ejecutarse. Para usar un valor de opción de un hook en forma shell, lee la variable de entorno `$CLAUDE_PLUGIN_OPTION_<KEY>`, como `$CLAUDE_PLUGIN_OPTION_WEBHOOK_URL` para una opción `webhook_url`, o establece `args` para cambiar el hook a forma exec. Antes de v2.1.207, los comandos de hook de plugin en forma shell también sustituían `${user_config.*}`.

<Note>
  En forma exec, `command` es solo el nombre o ruta del ejecutable. Si `command` es un nombre desnudo sin separador de ruta y contiene espacios junto con `args`, Claude Code registra una advertencia porque la generación fallará: no hay un ejecutable llamado `node script.js`. Mueve los tokens adicionales a `args`. Las rutas absolutas con espacios, como `C:\Program Files\nodejs\node.exe`, son un ejecutable válido único y no activan la advertencia.
</Note>

<h4 id="http-hook-fields">
  Campos de hook HTTP
</h4>

Además de los [campos comunes](#common-fields), los hooks HTTP aceptan estos campos:

| Campo            | Requerido | Descripción                                                                                                                                                                                                                                     |
| :--------------- | :-------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url`            | sí        | URL a la que enviar la solicitud POST                                                                                                                                                                                                           |
| `headers`        | no        | Encabezados HTTP adicionales como pares clave-valor. Los valores soportan interpolación de variables de entorno usando sintaxis `$VAR_NAME` o `${VAR_NAME}`. Solo se resuelven las variables listadas en `allowedEnvVars`                       |
| `allowedEnvVars` | no        | Lista de nombres de variables de entorno que pueden interpolarse en valores de encabezado. Las referencias a variables no listadas se reemplazan con cadenas vacías. Requerido para que funcione cualquier interpolación de variable de entorno |

Claude Code envía la [entrada JSON](#hook-input-and-output) del hook como el cuerpo de solicitud POST con `Content-Type: application/json`. El cuerpo de respuesta utiliza el mismo [formato de salida JSON](#json-output) que los hooks de comando.

El manejo de errores difiere de los hooks de comando; consulta [Manejo de respuesta HTTP](#http-response-handling).

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

| Campo    | Requerido | Descripción                                                                                                                                                                                                                                                                                                                                 |
| :------- | :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `server` | sí        | Nombre de un servidor MCP configurado. Para un [servidor incluido en plugin](/docs/es/mcp#plugin-provided-mcp-servers), este es el nombre con alcance `plugin:<plugin-name>:<server-name>`, como `plugin:my-plugin:db`, no la clave del servidor desnuda. El servidor debe estar ya conectado; el hook nunca activa un flujo OAuth o de conexión |
| `tool`   | sí        | Nombre de la herramienta a llamar en ese servidor                                                                                                                                                                                                                                                                                           |
| `input`  | no        | Argumentos pasados a la herramienta. Los valores de cadena soportan sustitución de `${path}` de la [entrada JSON](#hook-input-and-output) del hook, como `"${tool_input.file_path}"`                                                                                                                                                        |

Claude Code lee el contenido de texto de la herramienta de la misma manera que lee stdout de hook de comando, siguiendo la [regla de análisis bajo código de salida 0](#exit-code-0). Si el servidor nombrado no está conectado, o la herramienta devuelve `isError: true`, el hook produce un error sin bloqueo y la ejecución continúa.

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

Un hook `mcp_tool` puede ejecutarse solo después de que Claude Code haya puesto los servidores MCP de la sesión disponibles para los hooks. `SessionStart` y `Setup` pueden activarse antes de ese punto:

* **Al lanzar**: `SessionStart` se activa antes de que los servidores estén disponibles, incluso cuando lanzas con `--continue` o `--resume`. Claude Code omite los hooks `mcp_tool` del evento sin llamar a sus herramientas, y el [registro de depuración](#debug-hooks) registra `mcp_tool hooks are not available for the 'SessionStart' hook event (no MCP client context)`.
* **Más tarde en una sesión en ejecución**: después de `/clear` o una compactación, `SessionStart` se activa de nuevo con los servidores ya disponibles, y sus hooks `mcp_tool` se ejecutan.
* **En `Setup`**: `Setup` siempre se activa antes de que los servidores estén disponibles, por lo que Claude Code omite sus hooks `mcp_tool` cada vez y registra el mismo mensaje nombrando `Setup`.

Por ejemplo, esta configuración llama a la herramienta `load_context` en el servidor MCP `my_server` desde un hook `SessionStart` sin matcher, por lo que se aplica a cada fuente de `SessionStart`:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "mcp_tool",
            "server": "my_server",
            "tool": "load_context"
          }
        ]
      }
    ]
  }
}
```

Cuando ejecutas `claude`, Claude Code omite este hook, nunca llama a `load_context` y escribe el mensaje `no MCP client context` en el registro de depuración. Ejecuta `/clear` en esa misma sesión y el hook se ejecuta y llama a `load_context`. Un hook `type: "command"` en `SessionStart` se ejecuta al lanzar, por lo que usa uno para cualquier cosa que la sesión necesite desde su primer turno.

<h4 id="prompt-and-agent-hook-fields">
  Campos de hook de prompt y agente
</h4>

Además de los [campos comunes](#common-fields), los hooks de prompt y agente aceptan estos campos:

| Campo    | Requerido | Descripción                                                                                                                                                                                                 |
| :------- | :-------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt` | sí        | Texto de prompt a enviar al modelo. Usa `$ARGUMENTS` como marcador de posición para la entrada JSON del hook. Escapa con una barra invertida para incluir texto literal: `\$1.00` se renderiza como `$1.00` |
| `model`  | no        | Modelo a usar para evaluación. Por defecto es un modelo rápido                                                                                                                                              |

<h3 id="reference-scripts-by-path">
  Referencia de scripts por ruta
</h3>

Utiliza estos marcadores de posición para hacer referencia a scripts de hook relativos a la raíz del proyecto o plugin, independientemente del directorio de trabajo cuando se ejecuta el hook:

* `${CLAUDE_PROJECT_DIR}`: la raíz del proyecto donde comenzó la sesión. Claude Code también establece esta variable en el entorno de [servidores MCP stdio](/docs/es/mcp#option-3-add-a-local-stdio-server) y servidores LSP de plugin.
* `${CLAUDE_PLUGIN_ROOT}`: el directorio de instalación del plugin, para scripts incluidos con un [plugin](/docs/es/plugins/overview). Consulta [variables de entorno de plugin](/docs/es/plugins/manifest-reference#environment-variables) para saber cómo se comporta la ruta entre actualizaciones.
* `${CLAUDE_PLUGIN_DATA}`: el [directorio de datos persistentes](/docs/es/plugins/components#path-variables-and-persistent-data) del plugin, para dependencias y estado que deben sobrevivir a las actualizaciones del plugin.

<Note>
  **Los worktrees son diferentes.** Si Claude entra en un [worktree](/docs/es/worktrees) durante la sesión, Claude Code mantiene `${CLAUDE_PROJECT_DIR}` donde estaba y pasa la ruta del worktree a tus hooks de una manera diferente:

  * **`${CLAUDE_PROJECT_DIR}` se queda en su lugar**: aún apunta a la raíz del proyecto donde comenzó la sesión, por lo que un comando como `${CLAUDE_PROJECT_DIR}/.claude/hooks/check-style.sh` aún ejecuta el script en el checkout principal.
  * **`cwd` sigue a Claude**: el campo `cwd` en la [entrada JSON](#common-input-fields) del hook es la raíz del worktree después de que Claude entra en un worktree, y el nuevo directorio después de que Claude ejecuta `cd`. Léelo cuando un hook necesite saber en qué directorio está trabajando Claude.
</Note>

Prefiere [forma exec](#exec-form-and-shell-form) para cualquier hook que haga referencia a un marcador de posición de ruta. En forma shell, envuelve cada marcador de posición en comillas dobles.

<Tabs>
  <Tab title="Project scripts">
    Este ejemplo usa `${CLAUDE_PROJECT_DIR}` para ejecutar un verificador de estilo desde el directorio `.claude/hooks/` del proyecto después de cualquier llamada de herramienta `Write` o `Edit`:

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

  <Tab title="Plugin scripts">
    Define hooks de plugin en `hooks/hooks.json` con un campo `description` opcional de nivel superior. Cuando un plugin está habilitado, sus hooks se fusionan con tus hooks de usuario y proyecto.

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

    Consulta la [referencia de componentes de plugin](/docs/es/plugins/components#hooks) para obtener detalles sobre cómo crear hooks de plugin.
  </Tab>
</Tabs>

<h3 id="hooks-in-skills-and-agents">
  Hooks en skills y agentes
</h3>

Además de archivos de configuración y plugins, los hooks pueden definirse directamente en [skills](/docs/es/skills) y [subagentes](/docs/es/sub-agents) usando frontmatter, en el mismo formato de configuración que los hooks basados en configuración. Cuánto tiempo Claude Code los mantiene registrados depende del componente:

* **Hooks de subagente**: Claude Code los ejecuta solo mientras ese subagente se está ejecutando y los elimina cuando termina. Claude Code convierte un hook `Stop` aquí a `SubagentStop`, el evento que se activa cuando un subagente se completa.
* **Hooks de skill**: Claude Code los registra cuando tú o Claude invocas la skill y los mantiene ejecutándose durante el resto de la sesión, en turnos después del turno propio de la skill también. Para que Claude Code elimine un hook después de su primera ejecución exitosa en su lugar, establece [`once: true`](#common-fields) en él.

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

Los subagentes utilizan el mismo formato en su frontmatter YAML.

Los hooks de frontmatter en una skill de proyecto siguen la misma [regla de confianza de espacio de trabajo que los hooks en archivos de configuración](#workspace-trust). Claude Code los registra cuando tú o Claude invocas la skill, incluso en una ejecución `-p` en una carpeta que no has confiado.

Los hooks de frontmatter en un subagente de proyecto se ejecutan solo después de que aceptes el [diálogo de confianza de espacio de trabajo](/docs/es/permissions#project-allow-rules-and-workspace-trust) para la carpeta de la que proviene el archivo del agente. Una sesión `-p` no cuenta como aceptarlo. [Lo que se ejecuta antes de confiar en una carpeta](/docs/es/permissions#what-runs-before-you-trust-a-folder) compara esto con la regla del archivo de configuración, y la página de subagentes lista [qué alcances están exentos](/docs/es/sub-agents#hooks-in-subagent-frontmatter). Antes de v2.1.218, estos hooks podían ejecutarse desde carpetas que no habías confiado.

<h3 id="the-/hooks-menu">
  El menú `/hooks`
</h3>

Escribe `/hooks` en Claude Code para abrir un navegador de solo lectura para tus hooks configurados. El menú muestra cada evento de hook con un recuento de hooks configurados, te permite profundizar en matchers y muestra los detalles completos de cada manejador de hook. Úsalo para verificar la configuración, comprobar desde qué archivo de configuración proviene un hook o inspeccionar el comando, prompt o URL de un hook.

El menú muestra los cinco tipos de hook: `command`, `prompt`, `agent`, `http` y `mcp_tool`. Cada hook está etiquetado con un prefijo `[type]` y una fuente que indica dónde se definió:

* `User Settings`: de `~/.claude/settings.json`
* `Project Settings`: de `.claude/settings.json`
* `Local Settings`: de `.claude/settings.local.json`
* `Plugin Hooks`: de `hooks/hooks.json` de un plugin
* `Session Hooks`: registrado en memoria para la sesión actual

Seleccionar un hook abre una vista de detalle mostrando su evento, matcher, tipo, archivo de origen y el comando, prompt o URL completo. El menú es de solo lectura: para añadir, modificar o eliminar hooks, edita el JSON de configuración directamente o pide a Claude que haga el cambio.

<h3 id="disable-or-remove-hooks">
  Deshabilitar o eliminar hooks
</h3>

Para eliminar un hook, elimina su entrada del archivo JSON de configuración.

Para deshabilitar temporalmente todos los hooks sin eliminarlos, establece `"disableAllHooks": true` en tu archivo de configuración. Claude Code lee el valor que queda después de que se aplica la [precedencia de configuración](/docs/es/settings#settings-precedence), por lo que un `"disableAllHooks": false` en `.claude/settings.json` de un proyecto anula un `true` en tu configuración de usuario. Para desactivar los hooks para una ejecución sin importar lo que diga la configuración del proyecto, pasa `--settings '{"disableAllHooks": true}'`, que tiene precedencia sobre la configuración de proyecto y local. No hay forma de deshabilitar un hook individual mientras lo mantienes en la configuración.

La configuración `disableAllHooks` respeta la jerarquía de configuración administrada. Si un administrador ha configurado hooks a través de la configuración de política administrada, `disableAllHooks` establecido en la configuración de usuario, proyecto o local no puede deshabilitar esos hooks administrados. Solo `disableAllHooks` establecido en el nivel de configuración administrada puede deshabilitar hooks administrados. Para el alcance completo de cada nivel, consulta [`disableAllHooks`](/docs/es/settings-reference#disableallhooks).

Las ediciones directas a hooks en archivos de configuración normalmente se recogen automáticamente por el observador de archivos.

<h2 id="hook-input-and-output">
  Entrada y salida de hooks
</h2>

Los hooks de comando reciben datos JSON a través de stdin y comunican resultados a través de códigos de salida, stdout y stderr. Los hooks HTTP reciben el mismo JSON que el cuerpo de la solicitud POST y comunican resultados a través del cuerpo de la respuesta HTTP. Esta sección cubre campos y comportamiento comunes a todos los eventos. Cada sección de evento bajo [Hook events](#hook-events) incluye su esquema de entrada específico y opciones de control de decisión.

En macOS y Linux, los hooks de comando se ejecutan en su propia sesión sin una terminal de control. El proceso de hook y cualquier proceso secundario no pueden abrir `/dev/tty` o enviar secuencias de escape directamente a la interfaz de Claude Code. Windows no tiene `/dev/tty`.

Para mostrar un mensaje al usuario en cualquier plataforma, devuelva [`systemMessage`](#json-output) en la salida JSON. Algunos eventos lo descartan o lo entregan en otro lugar, y cada [sección de evento](#hook-events) lo indica. Para activar una notificación de escritorio, establecer un título de ventana o sonar la campana, devuelva [`terminalSequence`](#emit-terminal-notifications) en su lugar.

<h3 id="common-input-fields">
  Campos de entrada comunes
</h3>

Los eventos de hook reciben estos campos como JSON, además de campos específicos del evento documentados en cada sección [hook event](#hook-events). Para hooks de comando, este JSON llega a través de stdin. Para hooks HTTP, llega como el cuerpo de la solicitud POST.

| Campo             | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :---------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `session_id`      | Identificador de sesión actual                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `prompt_id`       | UUID que identifica el prompt del usuario que se está procesando actualmente. Coincide con el [atributo `prompt.id` en eventos de OpenTelemetry](/docs/es/monitoring-usage#event-correlation-attributes), para que pueda correlacionar la salida del hook con la telemetría de un único prompt. Ausente hasta la primera entrada del usuario. Requiere Claude Code v2.1.196 o posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `transcript_path` | Ruta al JSON de conversación. El archivo de transcripción se escribe de forma asincrónica y puede rezagarse con respecto a la conversación en memoria, por lo que es posible que aún no incluya los mensajes más recientes del turno actual cuando se activa un hook. Los hooks que necesitan el texto del asistente final del turno actual deben usar `last_assistant_message` en [Stop](#stop) y [SubagentStop](#subagentstop) en lugar de leer la transcripción                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `cwd`             | Directorio de trabajo actual cuando se invoca el hook                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `scratchpad_dir`  | Ruta al directorio de scratchpad de la sesión, donde Claude mantiene archivos de trabajo temporales. Ausente cuando la sesión no tiene scratchpad o el directorio temporal no está disponible. Requiere Claude Code v2.1.257 o posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `permission_mode` | [Modo de permiso](/docs/es/permissions#permission-modes) actual: `"default"`, `"plan"`, `"acceptEdits"`, `"auto"`, `"dontAsk"` o `"bypassPermissions"`. El modo etiquetado como **Manual** llega como `"default"`, nunca como `"manual"`, por lo que los scripts que coinciden con `"default"` siguen funcionando. No todos los eventos reciben este campo. Consulte el ejemplo JSON en cada sección [hook event](#hook-events)                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `effort`          | Objeto con un campo `level` que contiene el [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) en vigor cuando se ejecuta el hook: `"low"`, `"medium"`, `"high"`, `"xhigh"` o `"max"`. Si establece un nivel que el modelo activo no admite, `level` reporta el nivel que Claude Code ejecutó en su lugar; [Adjust effort level](/docs/es/model-config#adjust-effort-level) dice cómo lo elige. Ultracode no es un nivel distinto y se reporta como `"xhigh"`. El objeto coincide con el campo `effort` de la [línea de estado](/docs/es/statusline#available-data). Presente para eventos que se activan dentro de un contexto de uso de herramienta, como `PreToolUse`, `PostToolUse`, `Stop` y `SubagentStop`, cuando el modelo actual admite el parámetro de esfuerzo. El nivel también está disponible para comandos de hook y la herramienta Bash como la variable de entorno `$CLAUDE_EFFORT`. |
| `hook_event_name` | Nombre del evento que se activó                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |

Cuando se ejecuta con `--agent` o dentro de un subagente, se incluyen dos campos adicionales:

| Campo        | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :----------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `agent_id`   | Identificador único para el subagente. Presente solo cuando el hook se activa dentro de una llamada de subagente. Use esto para distinguir llamadas de hook de subagente de llamadas de hilo principal.                                                                                                                                                                                                                                       |
| `agent_type` | Nombre del agente (por ejemplo, `"Explore"` o `"security-reviewer"`). Presente cuando la sesión usa `--agent` o el hook se activa dentro de un subagente. Para subagentes, el tipo del subagente tiene precedencia sobre el valor `--agent` de la sesión. Consulte [SubagentStart](#subagentstart) para los valores que los subagentes personalizados y de plugin reportan y cómo escribir un matcher contra un nombre con alcance de plugin. |

Solo los hooks [`SessionStart`](#sessionstart) pueden recibir un campo `model`, y Claude Code no siempre lo incluye. Los hooks [`PreModelSwitch`](#premodelswitch) y [`PostModelSwitch`](#postmodelswitch) reciben `from_model` y `to_model` en su lugar, así que use un hook PostModelSwitch para seguir el modelo mientras cambia durante una sesión.

No hay variable de entorno `$CLAUDE_MODEL`. El hook puede leer `$ANTHROPIC_MODEL` si lo establece en su shell, pero ese valor no cambia cuando cambia de modelos con `/model` durante una sesión.

Un proceso de hook hereda el entorno principal, aparte de las variables exportadoras `OTEL_*` que Claude Code [elimina de cada subproceso que genera](/docs/es/monitoring-usage#administrator-configuration) y, cuando [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/es/env-vars#variables) se establece en `1`, las variables que elimina.

Por ejemplo, un hook `PreToolUse` para un comando Bash recibe esto en stdin:

```json theme={null}
{
  "session_id": "abc123",
  "prompt_id": "550e8400-e29b-41d4-a716-446655440000",
  "transcript_path": "/home/user/.claude/projects/.../transcript.jsonl",
  "cwd": "/home/user/my-project",
  "scratchpad_dir": "/tmp/claude-1000/-home-user-my-project/abc123/scratchpad",
  "permission_mode": "default",
  "hook_event_name": "PreToolUse",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test",
    "description": "Run test suite",
    "timeout": 120000,
    "run_in_background": false
  },
  "tool_use_id": "toolu_01ABC123..."
}
```

Los campos `tool_name`, `tool_input` y `tool_use_id` son específicos del evento. Cada sección [hook event](#hook-events) documenta los campos adicionales para ese evento.

<h3 id="exit-code-output">
  Salida de código de salida
</h3>

El código de salida de su comando de hook le dice a Claude Code si la acción debe proceder, ser bloqueada o ser ignorada. El código de salida no actúa solo. Claude Code lee campos [JSON output](#json-output) desde stdout en cada código de salida, no solo en 0, y para eventos que usan el modelo de decisión estándar, un objeto analizado que pasa la validación del esquema tiene efecto junto con el código. El bloqueo de Exit 2 es el único resultado que JSON no puede anular.

Dos tablas poseen las excepciones por evento: [Exit code 2 behavior per event](#exit-code-2-behavior-per-event) dice qué hacen los códigos de salida para cada evento, y [Decision control](#decision-control) dice qué campos de decisión honra cada evento. Los campos universales como `systemMessage` funcionan en la mayoría de eventos y se enumeran en la tabla [JSON output](#json-output).

<h4 id="exit-code-0">
  Exit code 0
</h4>

Exit 0 significa éxito, y es el código de salida previsto cuando imprime JSON para control estructurado.

Para la mayoría de eventos, Claude Code escribe stdout en el registro de depuración y no lo muestra en la transcripción. Las excepciones son `UserPromptSubmit`, `UserPromptExpansion`, `SessionStart` y `PostModelSwitch`, donde Claude Code agrega stdout de texto plano como contexto que Claude puede ver y actuar.

Si Claude Code lee su stdout como [JSON output](#json-output) o como texto plano depende de cómo comienza y termina, ignorando espacios en blanco circundantes:

* **Comienza con `{` y termina con `}`**: Claude Code lo analiza como JSON. Cuando la salida son dos o más líneas que cada una se analiza como JSON por su cuenta, y ninguna línea es un objeto [JSON output](#json-output) que establece un campo, Claude Code trata toda la salida como texto plano. Cuando una de esas líneas sí establece un campo, toda la salida es un fallo de análisis, descrito a continuación.
* **Comienza con `{` pero no termina con `}`**: Claude Code lo trata como texto plano.
* **Comienza con cualquier otra cosa**: Claude Code lo trata como texto plano, una matriz JSON o una cadena JSON entrecomillada incluida.

Para eventos que usan el modelo de decisión estándar, exit 0 con un objeto analizado que falla la validación del esquema es un error sin bloqueo: la acción procede, y la transcripción muestra un aviso `<hook name> hook error` con el mensaje de validación. Lo mismo sucede en cualquier código de salida que no sea 2, mientras que [exit 2 aún bloquea](#exit-code-2).

Para eventos que usan el modelo de decisión estándar, cuando Claude Code intenta analizar su stdout como JSON y no puede, reporta un error sin bloqueo en cada código de salida que no sea 2. La transcripción muestra un aviso `<hook name> hook error` con el mensaje de análisis. En los eventos que agregan stdout de texto plano como contexto, Claude Code no agrega el texto. Antes de v2.1.248, Claude Code trataba ese stdout como texto plano.

Stderr de un hook que sale 0 va solo al registro de depuración, nunca a la transcripción, y Claude nunca lo ve. Para leerlo usted mismo, habilite [debug logging](#debug-hooks). Para mostrar una advertencia a Claude desde un hook `PostToolUse` o `PostToolUseFailure`, salga 2 en su lugar para que [Claude vea stderr](#exit-code-2-behavior-per-event) aunque la herramienta ya se haya ejecutado.

<h4 id="exit-code-2">
  Exit code 2
</h4>

Exit 2 significa un error de bloqueo. En [eventos que pueden bloquear](#exit-code-2-behavior-per-event), exit 2 bloquea independientemente de si imprime JSON: incluso un `permissionDecision` JSON de `"allow"` no puede anularlo. Claude Code aún lee cualquier [JSON output](#json-output) válido en stdout. En `Elicitation` y `ElicitationResult`, el `hookSpecificOutput` de un hook exit-2 se ignora.

El mensaje de bloqueo es la razón de la decisión de bloqueo de su JSON cuando hace una, y su texto stderr en caso contrario. Lo que el bloqueo hace varía según el evento: `PreToolUse` bloquea la llamada a herramienta, `UserPromptSubmit` rechaza el prompt, y así sucesivamente. [Exit code 2 behavior per event](#exit-code-2-behavior-per-event) enumera el efecto para cada evento, y cada sección de evento dice dónde va el mensaje.

Un hook que sale 2 mientras imprime JSON que falla la validación del esquema [JSON output](#json-output) aún bloquea: Claude Code usa stderr como la razón de bloqueo y registra el fallo de validación en el registro de depuración. Antes de v2.1.214, Claude Code trataba esa combinación como un error sin bloqueo y la acción procedía.

Este script bloquea comandos `rm` saliendo 2 y deja cada otro comando al flujo de permiso normal:

```bash theme={null}
#!/bin/bash
# Lee entrada JSON desde stdin, verifica el comando
input=$(cat)
command=$(jq -r '.tool_input.command' <<<"$input")

if [[ "$command" == rm* ]]; then
  echo "Blocked: rm commands are not allowed" >&2
  exit 2  # Blocking error: tool call is prevented
fi

exit 0  # No decision: the normal permission flow applies
```

<h4 id="other-exit-codes">
  Otros códigos de salida
</h4>

Cualquier otro código de salida no bloquea por su cuenta para la mayoría de eventos de hook. Lo que sucede depende de su stdout:

* Con un objeto analizado que pasa la validación del esquema, para eventos que usan el modelo de decisión estándar, Claude Code ignora el código de salida y solo el JSON decide el resultado:
  * Cada campo que el evento admite se honra, incluyendo `permissionDecision`, `additionalContext`, `updatedInput` y `systemMessage`, y el hook no se reporta como un error.
  * [Decision control](#decision-control) enumera los campos de decisión por evento; campos universales como `systemMessage` siguen la tabla [JSON output](#json-output).
* Con un objeto analizado que falla la validación del esquema, para eventos que usan el modelo de decisión estándar, es el mismo error sin bloqueo que [en exit 0](#exit-code-0): la acción procede, y el aviso `<hook name> hook error` lleva el mensaje de validación.
* Con stdout que Claude Code [intenta analizar como JSON](#exit-code-0) y no puede, Claude Code reporta el mismo error sin bloqueo que en exit 0 para eventos que usan el modelo de decisión estándar. La acción procede, y el aviso lleva el mensaje de análisis.
* Con stdout que Claude Code [trata como texto plano](#exit-code-0), o con stdout vacío, es un error sin bloqueo para la mayoría de eventos de hook: la acción procede, y la transcripción muestra un aviso `<hook name> hook error` seguido de la primera línea de stderr, prefijado con `Failed with non-blocking status code:`. Para capturar el stderr completo, habilite [debug logging](#debug-hooks).

Los eventos fuera del modelo de decisión estándar mantienen sus propias filas en la [tabla por evento](#exit-code-2-behavior-per-event): `WorktreeCreate` falla la creación en cualquier salida distinta de cero sin importar lo que diga su JSON, y eventos que descartan la salida del hook completamente, como `StopFailure`, ignoran su JSON en cada código de salida, aparte de campos de efecto secundario como `terminalSequence`, que aún se activan.

Un hook que no puede iniciarse cae en el mismo cubo sin bloqueo. Cuando la ruta del script no existe o no es ejecutable, el shell sale con un código como 127 y ve el mismo aviso con el mensaje del intérprete, por ejemplo `Failed with non-blocking status code: /bin/sh: /path/to/hook.sh: No such file or directory`. Para la mayoría de eventos de hook, la acción procede. Cuando configura un hook de política, observe este aviso en su primera ejecución: una ruta mal escrita en `settings.json` deja la puerta silenciosamente deshabilitada.

<Warning>
  Para la mayoría de eventos de hook, el código de salida 2 es el único código de salida que bloquea solo a través del código. Sin JSON válido en stdout, Claude Code trata el código de salida 1 como un error sin bloqueo y procede con la acción, aunque 1 es el código de fallo convencional de Unix. Si su hook está destinado a aplicar una política, use `exit 2`. Los eventos de worktree difieren: cualquier código de salida distinto de cero de `WorktreeCreate` aborta la creación de worktree, y cualquier código de salida distinto de cero de `WorktreeRemove` hace que la eliminación de worktree falle si el directorio aún existe después.
</Warning>

<h4 id="timeouts">
  Tiempos de espera
</h4>

Aparte de un hook de comando que ejecuta con [`async: true`](#run-hooks-in-the-background), Claude Code cancela un hook `command`, `http` o `mcp_tool` que alcanza su [`timeout`](#common-fields), descartando la salida del hook, por lo que en la mayoría de eventos un hook agotado no renderiza decisión.

En [`PreModelSwitch`](#premodelswitch), un hook cancelado en su timeout bloquea el cambio de modelo. En `PreToolUse`, las dos familias de hooks difieren:

* Un hook `command`, `http` o `mcp_tool` agotado no bloquea la llamada a herramienta. La llamada continúa a través del [flujo de permiso](/docs/es/permissions) normal, así que no cuente con un hook estancado para actuar como puerta.
* Un hook de callback [Agent SDK](/docs/es/agent-sdk/hooks) que excede su timeout [bloquea la llamada a herramienta](#pretooluse).

<h4 id="exit-code-2-behavior-per-event">
  Comportamiento del código de salida 2 por evento
</h4>

Exit code 2 es la forma en que un hook señala "detente, no hagas esto". El efecto depende del evento, porque algunos eventos representan acciones que pueden bloquearse (como una llamada a herramienta que aún no ha sucedido) y otros representan cosas que ya sucedieron o no pueden prevenirse.

| Evento de hook        | ¿Puede bloquear? | Qué sucede en exit 2                                                                                                                                                                                                                                                   |
| :-------------------- | :--------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`          | Sí               | Bloquea la llamada a herramienta                                                                                                                                                                                                                                       |
| `PermissionRequest`   | No               | El código de salida 2 no se honra para este evento y el flujo de permiso procede sin cambios. Deniegue a través del objeto [`decision`](#permissionrequest-decision-control) en su lugar                                                                               |
| `UserPromptSubmit`    | Sí               | Bloquea el procesamiento del prompt y borra el prompt                                                                                                                                                                                                                  |
| `UserPromptExpansion` | Sí               | Bloquea la expansión                                                                                                                                                                                                                                                   |
| `Stop`                | Sí               | Evita que Claude se detenga, continúa la conversación                                                                                                                                                                                                                  |
| `SubagentStop`        | Sí               | Evita que el subagente se detenga                                                                                                                                                                                                                                      |
| `TeammateIdle`        | Sí               | Evita que el compañero se quede inactivo, por lo que continúa trabajando                                                                                                                                                                                               |
| `TaskCreated`         | Sí               | Revierte la creación de la tarea                                                                                                                                                                                                                                       |
| `TaskCompleted`       | Sí               | Evita que la tarea se marque como completada                                                                                                                                                                                                                           |
| `ConfigChange`        | Sí               | Bloquea que el cambio de configuración tenga efecto (excepto `policy_settings`)                                                                                                                                                                                        |
| `StopFailure`         | No               | La salida y el código de salida se ignoran, excepto `terminalSequence`                                                                                                                                                                                                 |
| `PostToolUse`         | No               | Muestra stderr a Claude; la herramienta ya se ejecutó                                                                                                                                                                                                                  |
| `PostToolUseFailure`  | No               | Muestra stderr a Claude; la herramienta ya falló                                                                                                                                                                                                                       |
| `PostToolBatch`       | Sí               | Detiene el bucle agentico antes de la siguiente llamada al modelo                                                                                                                                                                                                      |
| `PermissionDenied`    | No               | El código de salida y stderr se ignoran porque la denegación ya ocurrió. Use JSON `hookSpecificOutput.retry: true` para decirle al modelo que puede reintentar; Claude Code ignora `retry: true` para [denegaciones sin veredicto](#permissiondenied-decision-control) |
| `Notification`        | No               | El código de salida y stderr se ignoran                                                                                                                                                                                                                                |
| `SubagentStart`       | No               | Muestra stderr solo al usuario                                                                                                                                                                                                                                         |
| `SessionStart`        | No               | Muestra stderr solo al usuario                                                                                                                                                                                                                                         |
| `Setup`               | No               | El código de salida y stderr se ignoran                                                                                                                                                                                                                                |
| `SessionEnd`          | No               | Muestra stderr solo al usuario                                                                                                                                                                                                                                         |
| `CwdChanged`          | No               | Muestra stderr solo al usuario                                                                                                                                                                                                                                         |
| `DirectoryAdded`      | No               | Stderr va al registro de depuración; el directorio ya está agregado                                                                                                                                                                                                    |
| `FileChanged`         | No               | Muestra stderr solo al usuario                                                                                                                                                                                                                                         |
| `PreCompact`          | Sí               | Bloquea la compactación                                                                                                                                                                                                                                                |
| `PostCompact`         | No               | Muestra stderr solo al usuario                                                                                                                                                                                                                                         |
| `PreModelSwitch`      | Sí               | Bloquea el cambio de modelo y muestra stderr al usuario                                                                                                                                                                                                                |
| `PostModelSwitch`     | No               | Muestra stderr solo al usuario; el modelo ya cambió                                                                                                                                                                                                                    |
| `Elicitation`         | Sí               | Deniega la elicitación                                                                                                                                                                                                                                                 |
| `ElicitationResult`   | Sí               | Bloquea la respuesta (la acción se convierte en decline)                                                                                                                                                                                                               |
| `WorktreeCreate`      | Sí               | Cualquier código de salida distinto de cero causa que la creación de worktree falle                                                                                                                                                                                    |
| `WorktreeRemove`      | Sí               | Cualquier código de salida distinto de cero causa que la eliminación de worktree falle si el directorio aún existe después. Consulte [WorktreeRemove](#worktreeremove) para saber qué sucede con el directorio                                                         |
| `InstructionsLoaded`  | No               | El código de salida se ignora                                                                                                                                                                                                                                          |
| `MessageDisplay`      | No               | Se muestra el texto original                                                                                                                                                                                                                                           |

Para `SessionStart`, `SubagentStart` y `PostModelSwitch`, Claude Code renderiza el stderr del código de salida 2 en la transcripción como un aviso `<hook name> hook error`, de la misma manera que renderiza un [error sin bloqueo](#exit-code-output). Claude no lo ve, y la sesión o subagente procede. Para `SubagentStart`, el aviso aparece en la propia transcripción del subagente, no en la conversación principal.

<h3 id="http-response-handling">
  Manejo de respuesta HTTP
</h3>

Los hooks HTTP usan códigos de estado HTTP y cuerpos de respuesta en lugar de códigos de salida y stdout. Los resultados a continuación se aplican a la mayoría de eventos; un evento con su propio contrato de fallo en la [tabla por evento](#exit-code-2-behavior-per-event), como `WorktreeCreate`, aplica ese contrato a un hook HTTP fallido también:

* **2xx con un cuerpo vacío**: éxito, equivalente a código de salida 0 sin salida
* **2xx con un cuerpo de objeto JSON**: analizado usando el mismo esquema [JSON output](#json-output) que los hooks de comando. Un cuerpo que falla la validación del esquema es un error sin bloqueo
* **2xx con cualquier otro cuerpo, como texto plano**: error sin bloqueo, manejado igual que un estado que no es 2xx. Claude Code no agrega el texto al contexto de Claude
* **Estado que no es 2xx**: error sin bloqueo, la ejecución continúa
* **Fallo de conexión**: error sin bloqueo, la ejecución continúa
* **Tiempo de espera**: el hook se cancela, como se describe bajo [Timeouts](#timeouts)

A diferencia de los hooks de comando, los hooks HTTP no pueden señalar un error de bloqueo solo a través de códigos de estado. Para bloquear una llamada a herramienta o denegar un permiso, devuelva una respuesta 2xx con un cuerpo JSON que contenga los campos de decisión apropiados.

<h3 id="json-output">
  Salida JSON
</h3>

Los códigos de salida solo le permiten bloquear o permanecer en silencio, pero la salida JSON le da un control más granular. En lugar de salir con código 2 para bloquear, salga 0 e imprima un objeto JSON en stdout. Claude Code lee campos específicos de ese JSON para controlar el comportamiento, incluyendo [decision control](#decision-control) para bloquear, permitir o escalar al usuario.

<Note>
  Elija un enfoque por hook: use códigos de salida solos para señalizar, o salga 0 e imprima JSON para control estructurado. Si los mezcla, exit 2 mantiene su [efecto de bloqueo](#exit-code-2-behavior-per-event), y Claude Code aún lee los campos JSON, con la excepción de elicitación única anotada bajo [Exit code 2](#exit-code-2).
</Note>

El stdout de su hook debe contener solo el objeto JSON. Si su perfil de shell imprime texto al inicio, puede interferir con el análisis JSON. Consulte [Hook JSON has no effect](/docs/es/hooks-guide#hook-json-has-no-effect) en la guía de solución de problemas.

Las cadenas de salida de hook, incluyendo `additionalContext`, `systemMessage` e `initialUserMessage`, y su stdout plano, están limitadas a 10.000 caracteres:

* **Alcance**: Claude Code mide cada cadena por su cuenta, incluso cuando varios hooks se ejecutan para el mismo evento. Para salida JSON, cada campo se mide por separado; stdout plano se mide en su totalidad.
* **Sobre el límite**: Claude Code guarda la salida en un archivo en el directorio de sesión y la reemplaza con la ruta del archivo y una vista previa de hasta los primeros 2.000 caracteres. Un resultado de Bash válido grande se maneja de la misma manera, descrito bajo [Output limits](/docs/es/tools-reference#output-limits). A diferencia de ese techo de Bash, este límite no tiene configuración o variable de entorno para aumentarlo.
* **Lectura del archivo**: Claude Code no le pide a Claude que lea el archivo, así que mantenga cualquier cosa que Claude siempre deba ver dentro del límite.

El objeto JSON admite tres tipos de campos:

* **Campos universales** como `continue` se enumeran en la tabla a continuación. Cada evento los acepta, pero algunos eventos los descartan o entregan `systemMessage` en otro lugar que no sea la transcripción. Cada sección de evento lo indica. `terminalSequence` funciona en esos eventos también, con las excepciones enumeradas bajo [Emit terminal notifications](#emit-terminal-notifications).
* **`decision` y `reason` de nivel superior** son utilizados por algunos eventos para bloquear o proporcionar retroalimentación.
* **`hookSpecificOutput`** es un objeto anidado para eventos que necesitan control más rico. Requiere un campo `hookEventName` establecido en el nombre del evento.

| Campo              | Predeterminado | Descripción                                                                                                                                                                                                                                                                                                                                                      |
| :----------------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `continue`         | `true`         | Si es `false`, Claude detiene el procesamiento completamente después de que se ejecuta el hook. Tiene precedencia sobre cualquier campo de decisión específico del evento                                                                                                                                                                                        |
| `stopReason`       | ninguno        | Mensaje mostrado al usuario cuando `continue` es `false`. Se queda en la conversación, por lo que Claude lo ve si la conversación continúa                                                                                                                                                                                                                       |
| `suppressOutput`   | `false`        | No tiene efecto: Claude Code acepta el campo pero no actúa sobre él. El stdout de un hook exitoso nunca se muestra en la transcripción y se registra en el registro de depuración                                                                                                                                                                                |
| `systemMessage`    | ninguno        | Mensaje de advertencia mostrado al usuario. En [Agent SDK](/docs/es/agent-sdk/overview) y salida [`--output-format stream-json`](/docs/es/headless), puede llegar como un [`SDKInformationalMessage`](/docs/es/agent-sdk/typescript#sdkinformationalmessage)                                                                                                                    |
| `terminalSequence` | ninguno        | Una secuencia de escape de terminal para que Claude Code emita en su nombre, como una notificación de escritorio, título de ventana o campana. Restringido a OSC `0`/`1`/`2`/`9`/`99`/`777` y BEL. Si el valor contiene algo fuera de la lista de permitidos, el campo se ignora. Use esto en lugar de escribir en `/dev/tty`, que no está disponible para hooks |

Para detener Claude completamente:

```json theme={null}
{ "continue": false, "stopReason": "Build failed, fix errors before continuing" }
```

Para hooks `PreToolUse` y `PostToolUse`, la parada se aplica incluso cuando la llamada a herramienta falla o se completa mientras Claude aún está transmitiendo una respuesta.

<h4 id="emit-terminal-notifications">
  Emitir notificaciones de terminal
</h4>

Los hooks se ejecutan sin una terminal de control, por lo que escribir secuencias de escape directamente en `/dev/tty` falla. En su lugar, devuelva la secuencia de escape en el campo `terminalSequence` y Claude Code la emite por usted a través de su propia ruta de escritura de terminal. Esto es libre de carreras, funciona dentro de tmux y GNU screen, y funciona en Windows donde no hay `/dev/tty`.

El campo acepta una cadena de una o más secuencias de escape permitidas:

* OSC `0`, `1`, `2`: títulos de ventana e icono
* OSC `9`: notificaciones de iTerm2, ConEmu, Windows Terminal y WezTerm, incluyendo progreso de barra de tareas `9;4`
* OSC `99`: notificaciones de Kitty
* OSC `777`: notificaciones de urxvt, Ghostty y Warp
* BEL desnudo

Las secuencias pueden terminarse con BEL o con ST. Cualquier cosa fuera de la lista de permitidos, incluyendo secuencias de cursor y color CSI, secuencias de paleta OSC, hipervínculos OSC 8, escrituras de portapapeles OSC 52 y OSC 1337, se rechaza y el campo se ignora.

Claude Code escribe la secuencia en sí cuando procesa la salida de su hook, por lo que el campo funciona en eventos que descartan `systemMessage` y `continue`, como `Notification` y `StopFailure`. Tiene dos límites:

* Claude Code escribe la secuencia solo en una sesión interactiva, y solo mientras su interfaz está en pantalla. En modo no interactivo con la bandera `-p` y en Agent SDK, ignora el campo.
* Un hook de comando `WorktreeCreate` no puede devolver JSON, porque Claude Code lee su stdout como la ruta de worktree. Un hook HTTP `WorktreeCreate` devuelve JSON y puede incluir el campo.

El ejemplo a continuación dispara una notificación de escritorio desde un hook `Notification`. La secuencia de escape se construye con escapes octales `printf` para que los bytes de control nunca aparezcan en la línea de comandos del shell, y `jq -n --arg` construye la salida JSON para que las comillas, barras invertidas y saltos de línea en el mensaje de notificación se escapen correctamente:

```bash theme={null}
#!/bin/bash
# Hook de notificación: ping al escritorio cuando Claude Code necesita atención.
input=$(cat)
title="Claude Code"
body=$(jq -r '.message // "Needs your attention"' <<<"$input")
seq=$(printf '\033]777;notify;%s;%s\007' "$title" "$body")
jq -nc --arg seq "$seq" '{terminalSequence: $seq}'
```

La forma `{ "terminalSequence": "..." }` es la misma desde cualquier shell o lenguaje.

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

* [SessionStart](#sessionstart) y [SubagentStart](#subagentstart): al inicio de la conversación, antes del primer prompt
* [UserPromptSubmit](#userpromptsubmit) y [UserPromptExpansion](#userpromptexpansion): junto al prompt enviado
* [PreToolUse](#pretooluse), [PostToolUse](#posttooluse), [PostToolUseFailure](#posttoolusefailure) y [PostToolBatch](#posttoolbatch): junto al resultado de la herramienta
* [Stop](#stop) y [SubagentStop](#subagentstop): al final del turno. La conversación continúa para que Claude pueda actuar sobre la retroalimentación. Consulte [Stop decision control](#stop-decision-control)
* [PostModelSwitch](#postmodelswitch): con la siguiente solicitud después del cambio. Consulte [PostModelSwitch decision control](#postmodelswitch-decision-control) para el tiempo

Cuando varios hooks devuelven `additionalContext` para el mismo evento, Claude recibe todos los valores.

Si un valor excede 10.000 caracteres, Claude Code escribe el texto en un archivo en el directorio de sesión y pasa a Claude la ruta del archivo con una vista previa de hasta los primeros 2.000 caracteres en su lugar. Claude puede leer el archivo, pero Claude Code no se lo pide.

Use `additionalContext` para información que Claude debe conocer sobre el estado actual de su entorno o la operación que acaba de ejecutarse:

* **Estado del entorno**: la rama actual, destino de implementación o banderas de características activas
* **Reglas de proyecto condicionales**: qué comando de prueba se aplica al archivo que acaba de editar, qué directorios son de solo lectura en este worktree
* **Datos externos**: problemas abiertos asignados a usted, resultados recientes de CI, contenido obtenido de un servicio interno

Para instrucciones que nunca cambian, prefiera [CLAUDE.md](/docs/es/memory). Se carga sin ejecutar un script y es el lugar estándar para convenciones de proyecto estáticas.

Escriba el texto como declaraciones factuales en lugar de instrucciones de sistema imperativas. Frases como "El destino de implementación es producción" o "Este repositorio usa `bun test`" se leen como información del proyecto. El texto enmarcado como comandos de sistema fuera de banda puede activar las defensas de inyección de prompts de Claude, lo que hace que Claude le muestre el texto en lugar de tratarlo como contexto.

Claude Code guarda el texto inyectado en la transcripción de sesión. Para eventos a mitad de sesión como `PostToolUse` o `UserPromptSubmit`, cuando reanuda con `--continue` o `--resume`, Claude Code reproduce el texto guardado en lugar de volver a ejecutar el hook para turnos anteriores, por lo que valores como marcas de tiempo o SHAs de commit se vuelven obsoletos. Los hooks `SessionStart` se ejecutan nuevamente al reanudar con `source` establecido en `"resume"`, o `"fork"` si agregó `--fork-session`, por lo que pueden actualizar su contexto.

<h4 id="decision-control">
  Control de decisión
</h4>

No todos los eventos admiten bloqueo o control de comportamiento a través de JSON. Los eventos que lo hacen cada uno usan un conjunto diferente de campos para expresar esa decisión. Use esta tabla como referencia rápida antes de escribir un hook:

| Eventos                                                                                                                             | Patrón de decisión                                  | Campos clave                                                                                                                                                                                                                                                                                                                            |
| :---------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| UserPromptSubmit, UserPromptExpansion, PostToolUse, PostToolUseFailure, PostToolBatch, Stop, SubagentStop, ConfigChange, PreCompact | `decision` de nivel superior                        | `decision: "block"`, `reason`. Stop y SubagentStop también aceptan `hookSpecificOutput.additionalContext` para [retroalimentación sin error que continúa la conversación](#stop-decision-control)                                                                                                                                       |
| TeammateIdle, TaskCompleted                                                                                                         | Código de salida o `continue: false`                | El código de salida 2 bloquea la acción con retroalimentación de stderr. JSON `{"continue": false, "stopReason": "..."}` también detiene al compañero completamente, coincidiendo con el comportamiento del hook `Stop`; [TaskCompleted lo ignora cuando la herramienta `TaskUpdate` activó el evento](#taskcompleted-decision-control) |
| TaskCreated                                                                                                                         | Código de salida o `decision` de nivel superior     | El código de salida 2 o `decision: "block"` [cancela la tarea](#taskcreated-decision-control) y devuelve el mensaje a Claude. `continue: false` se ignora                                                                                                                                                                               |
| PreToolUse                                                                                                                          | `hookSpecificOutput`                                | `permissionDecision` (allow/deny/ask/defer), `permissionDecisionReason`                                                                                                                                                                                                                                                                 |
| PreModelSwitch                                                                                                                      | `hookSpecificOutput` o `decision` de nivel superior | `permissionDecision` (allow/deny/ask), `permissionDecisionReason`. `decision: "block"` también [cancela el cambio](#premodelswitch-decision-control)                                                                                                                                                                                    |
| PermissionRequest                                                                                                                   | `hookSpecificOutput`                                | `decision.behavior` (allow/deny)                                                                                                                                                                                                                                                                                                        |
| PermissionDenied                                                                                                                    | `hookSpecificOutput`                                | `retry: true` le dice al modelo que puede reintentar la llamada a herramienta denegada; Claude Code ignora `retry: true` para [denegaciones sin veredicto](#permissiondenied-decision-control)                                                                                                                                          |
| WorktreeCreate                                                                                                                      | ruta return                                         | El hook de comando imprime la ruta en stdout; el hook HTTP devuelve `hookSpecificOutput.worktreePath`. El fallo del hook o la ruta faltante falla la creación                                                                                                                                                                           |
| WorktreeRemove                                                                                                                      | Código de salida                                    | Cualquier código de salida distinto de cero hace que la eliminación falle si el directorio aún existe después. La salida JSON se descarta                                                                                                                                                                                               |
| Elicitation                                                                                                                         | `hookSpecificOutput`                                | `action` (accept/decline/cancel), `content` (valores de campo de formulario para accept)                                                                                                                                                                                                                                                |
| ElicitationResult                                                                                                                   | `hookSpecificOutput`                                | `action` (accept/decline/cancel), `content` (valores de campo de formulario override)                                                                                                                                                                                                                                                   |
| MessageDisplay                                                                                                                      | `hookSpecificOutput`                                | `displayContent` reemplaza el texto mostrado en pantalla. Solo visualización: la transcripción y lo que Claude ve mantienen el original                                                                                                                                                                                                 |
| SessionStart, SubagentStart, PostModelSwitch                                                                                        | Solo contexto                                       | `hookSpecificOutput.additionalContext` agrega contexto para Claude. SessionStart también acepta [`initialUserMessage`, `watchPaths`, `sessionTitle` y `reloadSkills`](#sessionstart-decision-control). Sin bloqueo o control de decisión                                                                                                |
| Setup, Notification, SessionEnd, PostCompact, InstructionsLoaded, StopFailure, CwdChanged, DirectoryAdded, FileChanged              | Ninguno                                             | Sin control de decisión. Se usa para efectos secundarios como registro o limpieza                                                                                                                                                                                                                                                       |

Algunos eventos también pueden reescribir contenido en lugar de solo permitir o bloquearlo:

* `PreToolUse`: `updatedInput` directamente bajo `hookSpecificOutput` reemplaza los argumentos de una herramienta antes de que se ejecute. Consulte [PreToolUse decision control](#pretooluse-decision-control)
* `PermissionRequest`: `updatedInput` dentro del objeto `decision`. Consulte [PermissionRequest decision control](#permissionrequest-decision-control)
* `PostToolUse`: `updatedToolOutput` reemplaza el resultado de la herramienta. Consulte [PostToolUse decision control](#posttooluse-decision-control)
* `UserPromptSubmit`: no puede reemplazar el prompt; solo inyecta `additionalContext` junto a él

Para casos de uso de redacción o transformación, intercepte en `PreToolUse` para entradas de herramientas salientes y `PostToolUse` para resultados de herramientas entrantes.

Aquí hay ejemplos de cada patrón en acción:

<Tabs>
  <Tab title="Decisión de nivel superior">
    El único valor para `decision` es `"block"`. Para permitir que la acción continúe, omita `decision` de su JSON, o salga 0 sin ningún JSON en absoluto:

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
  Eventos de hooks
</h2>

Cada evento corresponde a un punto en el ciclo de vida de Claude Code donde los hooks pueden ejecutarse. Las secciones siguientes están ordenadas para coincidir con el ciclo de vida: desde la configuración de la sesión a través del bucle agéntico hasta el final de la sesión. Cada sección describe cuándo se dispara el evento, qué matchers admite, la entrada JSON que recibe y cómo controlar el comportamiento a través de la salida.

<h3 id="sessionstart">
  SessionStart
</h3>

Se ejecuta cuando Claude Code inicia una nueva sesión o reanuda una sesión existente. Útil para cargar contexto de desarrollo como problemas existentes o cambios recientes en su base de código, o configurar variables de entorno. Para contexto estático que no requiere un script, use [CLAUDE.md](/docs/es/memory) en su lugar.

SessionStart se ejecuta en cada sesión, así que mantenga estos hooks rápidos. Solo se admiten hooks `type: "command"` y `type: "mcp_tool"`. Consulte [Campos de hook de herramienta MCP](#mcp-tool-hook-fields) para saber cuándo se ejecutan los hooks `mcp_tool`.

El valor del matcher corresponde a cómo se inició la sesión:

| Matcher   | Cuándo se dispara                                                                                                                      |
| :-------- | :------------------------------------------------------------------------------------------------------------------------------------- |
| `startup` | Nueva sesión                                                                                                                           |
| `resume`  | `--resume`, `--continue`, o `/resume`                                                                                                  |
| `clear`   | `/clear`                                                                                                                               |
| `compact` | Compactación automática o manual                                                                                                       |
| `fork`    | Una nueva sesión bifurcada desde una existente: `--fork-session` con `--resume` o `--continue`, la copia de fondo `/fork`, o `/branch` |

Antes de v2.1.214, las sesiones bifurcadas reportaban la fuente `"resume"`.

Cuando inicia una sesión interactiva, reanuda una conversación al iniciar con `--continue` o `--resume`, o ejecuta `/clear`, los hooks SessionStart se ejecutan en segundo plano. Puede escribir de inmediato, y una conversación que reanudó aparece sin esperar a que se completen los hooks. La primera respuesta de Claude aún espera a que se completen los hooks, por lo que su contexto llega a Claude.

Cuando cambia de conversación con `/resume` dentro de una sesión, el cambio espera a que se completen los hooks. Si ejecuta `/clear` o cambia a otra conversación mientras los hooks de fondo aún se están ejecutando, nada de lo que devuelven se aplica a la sesión.

La misma espera se aplica al iniciar, incluida una sesión reanudada: un prompt que envía mientras los hooks SessionStart aún se están ejecutando no llega a Claude hasta que se completen.

Durante cualquiera de estas esperas, presione `Esc` para recuperar el prompt en la entrada sin enviarlo. Los hooks continúan ejecutándose.

<h4 id="sessionstart-input">
  Entrada de SessionStart
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks SessionStart reciben `source` y opcionalmente `model`, `agent_type`, y `session_title`:

| Campo           | Descripción                                                                                                                                                                                                                                           |
| :-------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `source`        | Cómo se inició la sesión: `"startup"` para nuevas sesiones, `"resume"` para sesiones reanudadas, `"clear"` después de `/clear`, `"compact"` después de compactación, o `"fork"` para una nueva sesión bifurcada desde una existente                   |
| `model`         | El identificador del modelo activo. Puede omitirse, por ejemplo después de `/clear` o cuando se restaura una sesión a través de recuperación de conversación, así que verifique el campo antes de leerlo                                              |
| `agent_type`    | El nombre del agente, presente cuando inicia Claude Code con `claude --agent <name>`                                                                                                                                                                  |
| `session_title` | El título de sesión actual si ya está establecido, por ejemplo a través de `--name` o `/rename`. Un hook que emite `sessionTitle` puede verificar `session_title` primero para evitar sobrescribir un título que el usuario estableció explícitamente |

Cuando `source` es `"resume"` o `"fork"` y la transcripción contiene al menos una respuesta de Claude, los hooks SessionStart también reciben los cuatro campos siguientes. Su hook puede usarlos para reportar qué cuesta reanudar una conversación obsoleta antes de la primera solicitud, por ejemplo en un [`systemMessage`](#json-output). Estos campos requieren Claude Code v2.1.251 o posterior.

| Campo                         | Descripción                                                                                                                                                                                                    |
| :---------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `seconds_since_last_response` | Segundos de reloj de pared desde la última respuesta en la transcripción reanudada                                                                                                                             |
| `context_tokens`              | Tokens que la primera solicitud de la sesión reanudada reenvía como su prompt                                                                                                                                  |
| `prompt_cache_likely_expired` | `true` cuando la última respuesta es más antigua que la [duración de vida del caché de prompt](/docs/es/prompt-caching#cache-lifetime) de la sesión o una compactación posterior reemplazó la conversación en caché |
| `estimated_cache_write_usd`   | Costo estimado en dólares estadounidenses de escribir `context_tokens` en el caché de prompt en el modelo de la sesión, excluyendo la respuesta                                                                |

Este ejemplo muestra la entrada para una sesión reanudada 90 minutos después de su última respuesta:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionStart",
  "source": "resume",
  "model": "claude-opus-5",
  "seconds_since_last_response": 5400,
  "context_tokens": 182340,
  "prompt_cache_likely_expired": true,
  "estimated_cache_write_usd": 1.1396
}
```

<h4 id="sessionstart-decision-control">
  Control de decisión de SessionStart
</h4>

Claude Code agrega stdout que [trata como texto plano](#exit-code-0) al contexto de Claude. Además de los [campos de salida JSON](#json-output) disponibles para todos los hooks, puede devolver estos campos específicos del evento:

| Campo                | Descripción                                                                                                                                                                                                                                                                                                                                                                        |
| :------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext`  | Cadena agregada al contexto de Claude al inicio de la conversación, antes del primer prompt. Consulte [Agregar contexto para Claude](#add-context-for-claude) para saber cómo se entrega el texto y qué poner en él                                                                                                                                                                |
| `initialUserMessage` | Cadena utilizada como el primer mensaje del usuario de la sesión. Se aplica en [modo no interactivo](/docs/es/headless) con la bandera `-p`, donde se convierte en el primer turno incluso si no se proporciona ningún prompt. Si se proporciona un prompt, sigue como el siguiente turno. A diferencia de `additionalContext`, que se adjunta a un turno existente, esto crea el turno |
| `sessionTitle`       | Establece el título de la sesión, con el mismo efecto que `/rename`. Úselo para nombrar sesiones automáticamente desde la carpeta de inicio, rama de git o nombre de worktree. Se aplica cuando `source` es `"startup"`, `"resume"`, o `"fork"`; se ignora en `"clear"` y `"compact"`                                                                                              |
| `watchPaths`         | Matriz de rutas absolutas para observar eventos [FileChanged](#filechanged) durante esta sesión                                                                                                                                                                                                                                                                                    |
| `reloadSkills`       | Booleano. Cuando es `true`, Claude Code vuelve a escanear los directorios de [skill](/docs/es/skills) y comando después de que se completen los hooks SessionStart, por lo que las skills que instaló el hook están disponibles en la misma sesión, comenzando con el primer prompt                                                                                                     |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "SessionStart",
    "additionalContext": "Current branch: feat/auth-refactor\nUncommitted changes: src/auth.ts, src/login.tsx\nActive issue: #4211 Migrate to OAuth2",
    "sessionTitle": "auth-refactor"
  }
}
```

Dado que stdout plano ya llega a Claude para este evento, un hook que solo carga contexto puede imprimir a stdout directamente sin construir JSON. Use la forma JSON cuando necesite combinar contexto con otros campos como `sessionTitle`.

Use `reloadSkills` cuando un hook SessionStart instala o actualiza skills. El descubrimiento de skills normalmente se ejecuta antes de que se completen los hooks SessionStart, por lo que los archivos que el hook escribe en `~/.claude/skills/` o `.claude/skills/` de otro modo solo aparecerían en la siguiente sesión. Este ejemplo sincroniza un repositorio de skills compartido y solicita el re-escaneo:

```bash theme={null}
#!/bin/bash

git -C ~/.claude/skills/team-skills pull --quiet 2>/dev/null || \
  git clone --quiet https://git.example.com/your-org/team-skills.git ~/.claude/skills/team-skills

echo '{"hookSpecificOutput": {"hookEventName": "SessionStart", "reloadSkills": true}}'
```

La URL del repositorio es un marcador de posición; reemplácela con su propio repositorio de skills. Con el marcador de posición, el clon falla e imprime un mensaje `fatal:` a stderr. Stderr de un hook SessionStart que sale con 0 es solo informativo, por lo que la solicitud `reloadSkills` aún se aplica.

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

# Run your setup commands that modify the environment
source ~/.nvm/nvm.sh
nvm use 20

if [ -n "$CLAUDE_ENV_FILE" ]; then
  ENV_AFTER=$(export -p | sort)
  comm -13 <(echo "$ENV_BEFORE") <(echo "$ENV_AFTER") >> "$CLAUDE_ENV_FILE"
fi

exit 0
```

<Note>
  `CLAUDE_ENV_FILE` está disponible para hooks SessionStart, [Setup](#setup), [CwdChanged](#cwdchanged), y [FileChanged](#filechanged). Otros tipos de hooks no tienen acceso a esta variable.
</Note>

<h3 id="setup">
  Setup
</h3>

Se dispara solo cuando inicia Claude Code con `--init-only`, o con `--init` o `--maintenance` en [modo no interactivo](/docs/es/headless) con la bandera `-p`. No se dispara al iniciar normalmente. Úselo para instalación de dependencias única o limpieza programada que dispara explícitamente desde CI o scripts, separado del inicio de sesión normal. Para inicialización por sesión, use [SessionStart](#sessionstart) en su lugar.

El valor del matcher corresponde a la bandera CLI que disparó el hook:

| Matcher       | Cuándo se dispara                         |
| :------------ | :---------------------------------------- |
| `init`        | `claude --init-only` o `claude -p --init` |
| `maintenance` | `claude -p --maintenance`                 |

Cuando ejecuta `claude --init-only`, Claude Code ejecuta hooks Setup y hooks `SessionStart` con el matcher `startup`, luego sale sin iniciar una conversación.

Cuando inicia o continúa una conversación con `-p`, también necesita proporcionar un prompt, como argumento o canalizando en stdin. Puede omitir el prompt cuando un hook `SessionStart` proporciona [`initialUserMessage`](#sessionstart-decision-control) o cuando reanuda una sesión con una [llamada de herramienta diferida](#defer-a-tool-call-for-later).

En caso de éxito, `--init-only` no imprime nada en la terminal. Para confirmar que los hooks se ejecutaron, comience con `claude --debug-file <path> --init-only`, reemplazando `<path>` con una ubicación de archivo de registro, y verifique el registro para las entradas de hook Setup y SessionStart.

Debido a que Setup no se dispara en cada inicio, un plugin que necesita una dependencia instalada no puede confiar solo en Setup. El patrón práctico es verificar la dependencia en el primer uso e instalar si falta, por ejemplo un hook o skill que prueba `${CLAUDE_PLUGIN_DATA}/node_modules` y ejecuta `npm install` si está ausente. Consulte el [directorio de datos persistentes](/docs/es/plugins/components#path-variables-and-persistent-data) para saber dónde almacenar las dependencias instaladas. Si distribuye su plugin a través de un marketplace, es posible que no necesite este patrón: Claude Code [instala automáticamente las dependencias del paquete Node.js elegibles](/docs/es/plugins/loading#node-js-package-dependencies) cuando almacena en caché el plugin.

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

Los hooks Setup no pueden bloquear; la ejecución continúa en cualquier código de salida. En cada código de salida, Claude Code descarta los [campos de salida JSON](#json-output) de un hook Setup, como `systemMessage`, `continue`, y `hookSpecificOutput.additionalContext`. Con `-p`, la salida stdout, stderr y código de salida de un hook Setup aparecen en la salida de la ejecución solo como [eventos `hook_response`](/docs/es/headless#read-session-metadata) cuando inicia con `--output-format stream-json --verbose`.

Los hooks Setup tienen acceso a `CLAUDE_ENV_FILE`. Las variables escritas en ese archivo persisten en comandos Bash posteriores para la sesión, al igual que en [hooks SessionStart](#persist-environment-variables). Solo se ejecutan hooks `type: "command"` en `Setup`. Un hook `type: "mcp_tool"` en `Setup` siempre se omite, como se describe en [Campos de hook de herramienta MCP](#mcp-tool-hook-fields).

<h3 id="instructionsloaded">
  InstructionsLoaded
</h3>

Se dispara cuando se carga un archivo `CLAUDE.md` o `.claude/rules/*.md` en el contexto. Este evento se dispara al inicio de la sesión para archivos cargados con entusiasmo y nuevamente más tarde cuando se cargan archivos de forma perezosa, por ejemplo cuando Claude accede a un subdirectorio que contiene un `CLAUDE.md` anidado o cuando las reglas condicionales con frontmatter `paths:` coinciden. El hook no admite bloqueo o control de decisión. Se ejecuta de forma asincrónica con fines de observabilidad.

Este evento no se dispara cuando Claude [lee `AGENTS.md` directamente](/docs/es/memory#agents-md) a través de la configuración **Project instructions**. Se dispara cuando un `CLAUDE.md` importa su `AGENTS.md`, con `load_reason` establecido en `include` como para cualquier otro archivo importado, y cuando `CLAUDE.md` es un symlink a él, como una carga normal de `CLAUDE.md`.

El matcher se ejecuta contra `load_reason`. Por ejemplo, use `"matcher": "session_start"` para dispararse solo para archivos cargados al inicio de la sesión, o `"matcher": "path_glob_match|nested_traversal"` para dispararse solo para cargas perezosas.

<h4 id="instructionsloaded-input">
  Entrada de InstructionsLoaded
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks InstructionsLoaded reciben estos campos:

| Campo               | Descripción                                                                                                                                                                                                                                  |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `file_path`         | Ruta absoluta al archivo de instrucciones que se cargó                                                                                                                                                                                       |
| `memory_type`       | Alcance del archivo: `"User"`, `"Project"`, `"Local"`, o `"Managed"`                                                                                                                                                                         |
| `load_reason`       | Por qué se cargó el archivo: `"session_start"`, `"nested_traversal"`, `"path_glob_match"`, `"include"`, o `"compact"`. El valor `"compact"` se dispara cuando los archivos de instrucciones se recargan después de un evento de compactación |
| `globs`             | Patrones de glob de ruta del frontmatter `paths:` del archivo, si los hay. Presente solo para cargas `path_glob_match`                                                                                                                       |
| `trigger_file_path` | Ruta al archivo cuyo acceso disparó esta carga, para cargas perezosas                                                                                                                                                                        |
| `parent_file_path`  | Ruta al archivo de instrucciones padre que incluyó este, para cargas `include`                                                                                                                                                               |

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

Los hooks InstructionsLoaded no tienen control de decisión. No pueden bloquear o modificar la carga de instrucciones. Claude Code descarta sus [campos de salida JSON](#json-output), como `systemMessage` y `continue`. Use este evento para auditoría de registros, seguimiento de cumplimiento u observabilidad.

<h3 id="userpromptsubmit">
  UserPromptSubmit
</h3>

Se ejecuta cuando el usuario envía un prompt, antes de que Claude lo procese. Esto le permite agregar contexto adicional basado en el prompt/conversación, validar prompts o bloquear ciertos tipos de prompts.

Los hooks `UserPromptSubmit` tienen un tiempo de espera predeterminado de 30 segundos para tipos `command`, `http` y `mcp_tool`, más corto que el predeterminado de 600 segundos para esos tipos en la mayoría de otros eventos. Debido a que este hook se ejecuta antes de cada prompt y bloquea el procesamiento del modelo hasta que se completa, un hook atascado detiene la sesión. Si su hook necesita más tiempo, establezca el campo `timeout` en la entrada del hook.

Aparte de un hook de comando que ejecuta con [`async: true`](#run-hooks-in-the-background), un hook de comando, HTTP o herramienta MCP `UserPromptSubmit` que alcanza su tiempo de espera se cancela y su salida, incluido cualquier `additionalContext`, se descarta. El prompt aún llega a Claude sin ese contexto. La transcripción muestra un aviso que nombra el hook, el tiempo de espera que se disparó y que la salida se descartó.

Un hook de devolución de llamada [Agent SDK](/docs/es/agent-sdk/hooks) en `UserPromptSubmit` que alcanza su tiempo de espera bloquea el prompt con un mensaje que nombra el hook y el tiempo de espera, porque una devolución de llamada allí puede actuar como una puerta de política que no debe fallar abierta. La sesión continúa. Antes de v2.1.208, un tiempo de espera de devolución de llamada en ese evento terminaba el turno con un error de ejecución.

<h4 id="userpromptsubmit-input">
  Entrada de UserPromptSubmit
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks UserPromptSubmit reciben el campo `prompt` que contiene el texto que el usuario envió. El contenido pegado que se colapsó en un marcador de posición `[Pasted text #N]` llega expandido en su lugar. En sesiones donde Claude Code [marca el texto pegado para Claude](/docs/es/terminal-config#how-claude-treats-pasted-text), ese contenido expandido se encuentra entre una línea `<pasted_content id="…">` y una línea `</pasted_content id="…">`, así que tenga en cuenta esas líneas si su hook analiza el prompt.

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

* **Stdout de texto plano**: Claude Code agrega stdout que [trata como texto plano](#exit-code-0) al contexto de Claude
* **JSON con `additionalContext`**: use el formato JSON a continuación para más control. El campo `additionalContext` se agrega como contexto

Ningún canal produce una entrada de transcripción visible. El stdout plano y el valor `additionalContext` se inyectan cada uno como un recordatorio del sistema que comienza con el nombre del hook; Claude lee ambos. Para confirmar la entrega, verifique el [registro de depuración](#debug-hooks).

Para bloquear un prompt, devuelva un objeto JSON con `decision` establecido en `"block"`:

| Campo                    | Descripción                                                                                                                         |
| :----------------------- | :---------------------------------------------------------------------------------------------------------------------------------- |
| `decision`               | `"block"` evita que se procese el prompt y lo borra del contexto. Omita para permitir que el prompt continúe                        |
| `reason`                 | Se muestra al usuario cuando `decision` es `"block"`. No se agrega al contexto                                                      |
| `additionalContext`      | Cadena agregada al contexto de Claude junto con el prompt enviado. Consulte [Agregar contexto para Claude](#add-context-for-claude) |
| `sessionTitle`           | Establece el título de la sesión. Úselo para nombrar sesiones automáticamente basándose en el contenido del prompt                  |
| `suppressOriginalPrompt` | Si es `true` cuando `decision` es `"block"`, omite el texto del prompt original del mensaje de bloqueo mostrado al usuario          |

Un hook que bloquea saliendo con 2 se enruta de la misma manera que `reason`: el mensaje de bloqueo muestra el texto stderr al usuario y no se agrega al contexto.

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

Se ejecuta cuando un comando escrito por el usuario se expande en un prompt antes de llegar a Claude. Úselo para bloquear comandos específicos de invocación directa, inyectar contexto para una skill particular o registrar qué comandos invocan los usuarios. Por ejemplo, un hook que coincide con `deploy` puede bloquear `/deploy` a menos que esté presente un archivo de aprobación, o un hook que coincide con una skill de revisión puede agregar la lista de verificación de revisión del equipo como `additionalContext`.

Este evento cubre la ruta que `PreToolUse` no cubre: un hook `PreToolUse` que coincide con la herramienta `Skill` se dispara solo cuando Claude llama a la herramienta, pero escribir `/skillname` directamente omite `PreToolUse`. `UserPromptExpansion` se dispara en esa ruta directa.

Coincide en `command_name`. Deje el matcher vacío para dispararse en cada comando de tipo prompt.

<h4 id="userpromptexpansion-input">
  Entrada de UserPromptExpansion
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks UserPromptExpansion reciben `expansion_type`, `command_name`, `command_args`, `command_source` y la cadena `prompt` original. El campo `expansion_type` es `slash_command` para skills y comandos personalizados, o `mcp_prompt` para prompts del servidor MCP.

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
| `decision`          | `"block"` evita que el comando se expanda. Omita para permitir que continúe                                                           |
| `reason`            | Se muestra al usuario cuando `decision` es `"block"`                                                                                  |
| `additionalContext` | Cadena agregada al contexto de Claude junto con el prompt expandido. Consulte [Agregar contexto para Claude](#add-context-for-claude) |

Un hook que bloquea saliendo con 2 se enruta de la misma manera que `reason`: el mensaje de bloqueo muestra el texto stderr al usuario.

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
* transformar el texto que una aplicación Agent SDK muestra a sus usuarios
* redactar claves API o nombres de host internos de las respuestas de Claude

Claude Code retiene cada lote hasta que su hook devuelve, así que mantenga el hook rápido. Si el hook falla o agota el tiempo de espera, Claude Code muestra el texto original. El tiempo de espera predeterminado para este evento es 10 segundos; si su hook necesita más tiempo, establezca el campo `timeout` en la entrada del hook.

MessageDisplay es solo para visualización: el texto de reemplazo cambia solo lo que se renderiza en pantalla. La transcripción y lo que Claude ve mantienen el texto original, por lo que Claude nunca ve el reemplazo, y el modo detallado muestra el original. El hook recibe solo texto de mensaje del asistente, por lo que los resultados de herramientas y el texto que escribe se renderiza sin cambios.

MessageDisplay no admite matchers y se dispara para cada mensaje del asistente que transmite texto; los mensajes sin texto, como respuestas de solo llamada de herramienta, no lo disparan.

En ejecuciones no interactivas, incluidas consultas de Agent SDK y `claude -p`, MessageDisplay se ejecuta una vez por mensaje del asistente en lugar de una vez por lote de líneas. La llamada única llega después de que se completa el mensaje y lleva el texto del mensaje completo: `index` es `0`, `final` es `true`, y `delta` contiene el mensaje completo. Un hook que recopila el texto `delta` para cada mensaje recibe el mismo texto total en ambos modos.

<h4 id="messagedisplay-input">
  Entrada de MessageDisplay
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks MessageDisplay reciben identificadores para el turno y el mensaje, la posición de esta llamada dentro del mensaje y el nuevo texto en `delta`. Los límites de lotes dependen de cómo se transmite el texto, así que use `index` y `final` para rastrear el progreso a través de un mensaje en lugar de esperar que las líneas se agrupen de una manera particular.

| Campo        | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :----------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `turn_id`    | UUID del turno actual                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `message_id` | UUID del mensaje del asistente que se muestra. Estable en cada lote del mismo mensaje. Este no es el ID de API `msg_…`, por lo que no se puede correlacionar con IDs de mensaje de transcripción                                                                                                                                                                                                                                                                      |
| `index`      | Índice basado en cero de este lote dentro del mensaje                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `final`      | `true` en el último lote del mensaje. Cada mensaje tiene exactamente un lote final                                                                                                                                                                                                                                                                                                                                                                                    |
| `delta`      | Las líneas recién completadas desde el lote anterior, incluidas las saltos de línea finales. Siempre líneas completas, excepto el lote final que puede terminar a mitad de línea. En ejecuciones interactivas, el delta del lote final está vacío cuando el mensaje termina en un salto de línea, así que trate `final`, no un delta no vacío, como la señal de fin de mensaje. En ejecuciones de Agent SDK y `claude -p`, la llamada única lleva el mensaje completo |

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

Los hooks MessageDisplay no tienen control de decisión. No pueden bloquear el mensaje o cambiar lo que se almacena en la transcripción o se envía a Claude. Claude Code actúa sobre `displayContent` de su salida JSON y descarta `systemMessage` y `continue`.

Este ejemplo elimina el formato markdown de las respuestas de Claude para una visualización de texto plano. El script lee cada lote de stdin, elimina marcadores en negrita y comillas invertidas de código en línea de `delta`, y devuelve el resultado como `displayContent`.

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

Los lotes sin markdown pasan sin cambios. Si el script falla, por ejemplo porque falta `jq`, Claude Code muestra el texto original y solo nota el fallo en [salida de depuración](#debug-hooks), no en la sesión.

<h3 id="pretooluse">
  PreToolUse
</h3>

Se ejecuta después de que Claude crea parámetros de herramienta y antes de procesar la llamada de herramienta. Coincide con cualquier nombre de herramienta excepto `EndConversation`: herramientas integradas como `Bash`, `PowerShell`, `Edit`, `Write`, `Read`, `Glob`, `Grep`, `Agent`, `Workflow`, `WebFetch`, `WebSearch`, `AskUserQuestion`, y `ExitPlanMode`, y cualquier [nombre de herramienta MCP](#match-mcp-tools).

Para ejecutar un hook cuando un archivo específico cambia en el disco, sin importar qué lo escribió, use [FileChanged](#filechanged) en lugar de hacer coincidir herramientas de edición de archivos por nombre. A diferencia de PreToolUse, Claude Code ejecuta hooks FileChanged después del cambio y no tienen control de decisión, por lo que no pueden bloquear la escritura.

<Warning>
  PreToolUse se ejecuta solo cuando Claude llama a una herramienta. Los archivos que [referencia con `@` en su prompt](/docs/es/common-workflows#reference-files-and-directories) se agregan sin ninguna llamada de herramienta: Claude Code inserta su contenido mientras construye el prompt, por lo que ningún hook PreToolUse se dispara para ellos, incluidos los hooks que coinciden con `Read`. Para bloquear rutas específicas de referencias `@`, use una [regla de denegación `Read`](/docs/es/permissions#read-and-edit) en su lugar.

  PreToolUse tampoco se dispara para [`EndConversation`](/docs/es/tools-reference#endconversation-tool-behavior).
</Warning>

Use [control de decisión PreToolUse](#pretooluse-decision-control) para permitir, denegar, preguntar o diferir la llamada de herramienta.

Un hook de devolución de llamada [Agent SDK](/docs/es/agent-sdk/hooks) en `PreToolUse` que excede su tiempo de espera bloquea la llamada de herramienta, y Claude recibe un resultado de error que nombra el tiempo de espera. Una denegación explícita devuelta por otro hook aún tiene prioridad.

<h4 id="pretooluse-input">
  Entrada de PreToolUse
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks PreToolUse reciben `tool_name`, `tool_input`, y `tool_use_id`.

Para una [herramienta MCP](#match-mcp-tools), la entrada también lleva `mcp_server`, un objeto con el `name` del servidor y un `source` que dice de dónde vino la definición del servidor. Los valores `source` incluyen `plugin`, `sdk`, y alcances de configuración como `user` y `project`. [`McpServerProvenance`](/docs/es/agent-sdk/typescript#mcpserverprovenance) en la referencia de Agent SDK los enumera todos y dice cómo tratar uno que no reconozca. Base las decisiones de confianza en `source` en lugar de en `name` o el prefijo de nombre de herramienta `mcp__<server>__`. El campo `mcp_server` requiere Claude Code v2.1.274 o posterior.

Para las herramientas de archivo `Write`, `Edit`, y `Read`, `tool_input.file_path` siempre es absoluto:

* Claude Code expande `~` y rutas relativas antes de que se ejecuten los hooks, por lo que un hook que coincide con rutas no puede ser eludido a través de `~` o un deletreo relativo de la misma ruta
* En Windows, la ruta llega con separadores de barra invertida, incluso cuando su hook se ejecuta bajo Git Bash donde `$PWD` se ve como `/c/project`
* Una comparación escrita con barras diagonales, como una verificación `/src/`, nunca coincide con una ruta de barra invertida, y la llamada de herramienta continúa como si el hook no tuviera nada que bloquear
* Normalice separadores antes de comparar: `FILE_PATH="${FILE_PATH//\\//}"` en Bash, o `file_path.replace("\\", "/")` en Python, luego coincida con un segmento de ruta como `/src/` en lugar de anclar con `^`, ya que la ruta es absoluta

Una llamada `Write` en Windows entrega:

```json theme={null}
{
  "hook_event_name": "PreToolUse",
  "tool_name": "Write",
  "tool_input": {
    "file_path": "C:\\project\\src\\index.ts",
    "content": "..."
  },
  ...
}
```

Los campos `tool_input` dependen de la herramienta:

<a id="bash" />

<h5 id="bash">
  Bash
</h5>

Ejecuta comandos de shell.

| Campo               | Tipo    | Ejemplo            | Descripción                                                                                                                                                            |
| :------------------ | :------ | :----------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | string  | `"npm test"`       | El comando de shell a ejecutar                                                                                                                                         |
| `description`       | string  | `"Run test suite"` | Descripción opcional de lo que hace el comando                                                                                                                         |
| `timeout`           | number  | `120000`           | Tiempo de espera opcional en milisegundos. Los valores por encima del [máximo](/docs/es/tools-reference#bash-tool-behavior) se reducen al máximo en lugar de ser rechazados |
| `run_in_background` | boolean | `false`            | Si ejecutar el comando en segundo plano                                                                                                                                |

Cuando un comando Bash cambia archivos en un repositorio Git, Claude Code puede registrar qué cambió. Registra los cambios en cada modo de permiso cuando la configuración [`bashEditDiffEnabled`](/docs/es/settings-reference#basheditdiffenabled) activa el registro; la entrada de esa configuración dice qué archivos pueden establecerla. De lo contrario, solo los registra en modo automático y modo `bypassPermissions`, y solo cuando Claude Code dirige a Claude a editar archivos a través de Bash. Establezca `bashEditDiffEnabled` en `false` para desactivar el registro. Los comandos de fondo y los comandos de solo lectura no llevan diff.

Su hook [PostToolUse](#posttooluse) luego recibe los archivos cambiados en `tool_response.bashEditDiff`. La lista cubre qué cambió bajo el repositorio mientras se ejecutaba el comando. Los archivos que Git ignora y los archivos en submódulos no se enumeran. Requiere Claude Code v2.1.269 o posterior.

<Note>
  La lista es mejor esfuerzo y en beta pública. Claude Code puede perder un cambio, incluir un archivo que otro proceso cambió al mismo tiempo, o detenerse en sus límites de tamaño. La forma del campo puede cambiar. Use la lista para encontrar qué revisar, no para hacer cumplir una política.
</Note>

`changedFiles` y `files` enumeran qué cambió el comando; los campos restantes dicen qué tan completa y confiable es esa lista.

| Campo          | Tipo    | Ejemplo                                                 | Descripción                                                                                                                                                                                        |
| :------------- | :------ | :------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `changedFiles` | array   | `["/path/to/src/app.ts"]`                               | Rutas absolutas de los archivos que el comando cambió, como máximo 200. Presente siempre que `files` contenga un diff o `moreFiles` esté por encima de cero                                        |
| `files`        | array   | `[{"filePath": "/path/to/src/app.ts", "hunks": [...]}]` | Diffs de hasta 5 archivos cambiados, para visualización. `created` o `deleted` es `true` para un archivo que el comando agregó o eliminó                                                           |
| `moreFiles`    | number  | `2`                                                     | Recuento de archivos cambiados sin diff en `files`                                                                                                                                                 |
| `unavailable`  | boolean | `true`                                                  | Se establece cuando el diff está incompleto o no se pudo tomar                                                                                                                                     |
| `skipped`      | boolean | `true`                                                  | Se establece para un comando Git que mueve el árbol de trabajo, como `git checkout` o `git stash`, por lo que Claude Code no toma diff                                                             |
| `shared`       | boolean | `true`                                                  | Se establece cuando otra llamada de herramienta Bash, como la de un subagente, se ejecutó en el mismo repositorio al mismo tiempo, por lo que algunos cambios enumerados pueden ser de ese comando |

<a id="powershell" />

<h5 id="powershell">
  PowerShell
</h5>

Ejecuta comandos de PowerShell. Consulte la [herramienta PowerShell](/docs/es/tools-reference#powershell-tool) para disponibilidad por plataforma.

Los campos coinciden con la herramienta Bash, con la cadena de comando en `command`:

| Campo               | Tipo    | Ejemplo                    | Descripción                                    |
| :------------------ | :------ | :------------------------- | :--------------------------------------------- |
| `command`           | string  | `"Get-ChildItem -Recurse"` | El comando de PowerShell a ejecutar            |
| `description`       | string  | `"List files recursively"` | Descripción opcional de lo que hace el comando |
| `timeout`           | number  | `120000`                   | Tiempo de espera opcional en milisegundos      |
| `run_in_background` | boolean | `false`                    | Si ejecutar el comando en segundo plano        |

Coincida con `Bash|PowerShell` en hooks que inspeccionen comandos de shell, para que cubran ambas herramientas:

* En Windows, dondequiera que la herramienta PowerShell esté habilitada, Claude trata PowerShell como el shell principal y enruta comandos de shell a través de él.
* En Windows sin Git Bash, la herramienta se habilita automáticamente y Claude Code no registra la herramienta Bash en absoluto.
* Un hook que coincide solo con `Bash` nunca se dispara allí.

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

| Campo         | Tipo    | Ejemplo               | Descripción                         |
| :------------ | :------ | :-------------------- | :---------------------------------- |
| `file_path`   | string  | `"/path/to/file.txt"` | Ruta absoluta al archivo a editar   |
| `old_string`  | string  | `"original text"`     | Texto a encontrar y reemplazar      |
| `new_string`  | string  | `"replacement text"`  | Texto de reemplazo                  |
| `replace_all` | boolean | `false`               | Si reemplazar todas las ocurrencias |

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

| Campo     | Tipo   | Ejemplo          | Descripción                                                                        |
| :-------- | :----- | :--------------- | :--------------------------------------------------------------------------------- |
| `pattern` | string | `"**/*.ts"`      | Patrón glob para hacer coincidir archivos contra                                   |
| `path`    | string | `"/path/to/dir"` | Directorio opcional para buscar en. Por defecto es el directorio de trabajo actual |

<h5 id="grep">
  Grep
</h5>

Busca contenidos de archivo con expresiones regulares.

| Campo         | Tipo    | Ejemplo          | Descripción                                                                             |
| :------------ | :------ | :--------------- | :-------------------------------------------------------------------------------------- |
| `pattern`     | string  | `"TODO.*fix"`    | Patrón de expresión regular a buscar                                                    |
| `path`        | string  | `"/path/to/dir"` | Archivo o directorio opcional para buscar en                                            |
| `glob`        | string  | `"*.ts"`         | Patrón glob opcional para filtrar archivos                                              |
| `output_mode` | string  | `"content"`      | `"content"`, `"files_with_matches"`, o `"count"`. Por defecto es `"files_with_matches"` |
| `-i`          | boolean | `true`           | Búsqueda insensible a mayúsculas y minúsculas                                           |
| `multiline`   | boolean | `false`          | Habilitar coincidencia multilínea                                                       |

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

Cuando una llamada de Agent en primer plano se completa, su hook [PostToolUse](#posttooluse) recibe el resultado del subagente y telemetría de ejecución en `tool_response`. Lea estos campos para inspeccionar la ejecución; para resúmenes de tokens y costos en subagentes, use los [contadores de tokens y costos](/docs/es/monitoring-usage#token-counter) filtrados a `query_source` `"subagent"`, ya que `totalTokens` y `usage` cubren solo la solicitud final:

| Campo               | Tipo   | Ejemplo                                               | Descripción                                                                                                                                                                                                                                                                 |
| :------------------ | :----- | :---------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`            | string | `"completed"`                                         | `"completed"` para subagentes en primer plano, `"async_launched"` para subagentes en segundo plano. A partir de v2.1.198, los subagentes se ejecutan en segundo plano de forma predeterminada, por lo que un `run_in_background` omitido también produce `"async_launched"` |
| `agentId`           | string | `"a4d2c8f1e0b3a297"`                                  | Identificador para la ejecución del subagente                                                                                                                                                                                                                               |
| `content`           | array  | `[{"type": "text", "text": "Found 12 endpoints..."}]` | Los bloques de texto finales del subagente, o, para un subagente cuyo informe pasa a través de `SubagentHandback`, una nota breve sobre esa entrega en su lugar                                                                                                             |
| `resolvedModel`     | string | `"claude-sonnet-4-5"`                                 | Modelo en el que comenzó el subagente, que puede diferir del modelo solicitado                                                                                                                                                                                              |
| `modelsUsed`        | array  | `["claude-sonnet-4-5", "claude-haiku-4-5"]`           | Modelos utilizados en orden, con repeticiones consecutivas colapsadas; se establece solo cuando el modelo fue intercambiado a mitad de ejecución. Requiere Claude Code v2.1.212 o posterior                                                                                 |
| `totalTokens`       | number | `12450`                                               | Recuento de tokens de la solicitud final de API del subagente: tokens de entrada, salida y caché combinados. Este no es un total en toda la ejecución                                                                                                                       |
| `totalDurationMs`   | number | `48211`                                               | Duración de reloj de pared de la ejecución del subagente                                                                                                                                                                                                                    |
| `totalToolUseCount` | number | `7`                                                   | Recuento de llamadas de herramienta que realizó el subagente                                                                                                                                                                                                                |
| `usage`             | object | `{"input_tokens": 8320, ...}`                         | Desglose de tokens por tipo de la solicitud final de API: `input_tokens`, `output_tokens`, `cache_creation_input_tokens`, `cache_read_input_tokens`                                                                                                                         |

En Claude Code v2.1.271 o posterior, un subagente que se ejecuta con la herramienta [`SubagentHandback`](/docs/es/tools-reference) que Claude Code proporciona en [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode), entrega su informe a través de esa herramienta en lugar de devolverlo como texto. El campo `content` de su resultado `completed` luego lleva una nota breve sobre esa entrega en lugar del informe en sí. Para leer el informe, coincida un hook `PreToolUse` o `PostToolUse` en `SubagentHandback` y lea `tool_input.message`.

Para subagentes en segundo plano, la herramienta devuelve cuando la tarea se mueve al segundo plano, por lo que `tool_response` no lleva campos de uso: un lanzamiento en segundo plano devuelve inmediatamente, y una tarea en primer plano que Claude Code pone en segundo plano a mitad de ejecución devuelve en esa transición. Tiene `status: "async_launched"`, `agentId`, `description`, `prompt`, `outputFile`, y `resolvedModel`.

En una respuesta `completed`, `resolvedModel` nombra el modelo en el que comenzó el subagente, que puede diferir del valor `model` en `tool_input`, como cuando `availableModels` u otra anulación se aplica. En una respuesta `async_launched`, `resolvedModel` nombra el modelo en uso cuando el agente se movió al segundo plano, por lo que un intercambio que ocurrió antes de pasar al segundo plano se refleja allí. El comportamiento de `modelsUsed` y `resolvedModel` en tiempo de segundo plano requieren Claude Code v2.1.212 o posterior.

<a id="askuserquestion" />

<h5 id="askuserquestion">
  AskUserQuestion
</h5>

Hace al usuario una a cuatro preguntas de opción múltiple.

| Campo       | Tipo   | Ejemplo                                                                                                            | Descripción                                                                                                                                                                                                                                       |
| :---------- | :----- | :----------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `questions` | array  | `[{"question": "Which framework?", "header": "Framework", "options": [{"label": "React"}], "multiSelect": false}]` | Preguntas a presentar, cada una con una cadena `question`, `header` corto, matriz `options`, y bandera `multiSelect` opcional                                                                                                                     |
| `answers`   | object | `{"Which framework?": "React"}`                                                                                    | Opcional. Asigna texto de pregunta a la etiqueta de opción seleccionada. Las respuestas de selección múltiple unen etiquetas con comas. Claude no establece este campo; suministrarlo a través de `updatedInput` para responder programáticamente |

<h5 id="exitplanmode">
  ExitPlanMode
</h5>

Presenta un plan y pide al usuario que lo apruebe antes de que Claude salga del [modo de plan](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode). Claude escribe el plan en un archivo en el disco antes de llamar a la herramienta, por lo que la `tool_input` literal del modelo es típicamente vacía. Claude Code inyecta el contenido del plan y la ruta del archivo antes de pasar la entrada a los hooks.

| Campo            | Tipo   | Ejemplo                                     | Descripción                                                                                                                                                |
| :--------------- | :----- | :------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plan`           | string | `"## Refactor auth\n1. Extract..."`         | Contenido del plan en Markdown. Inyectado desde el archivo del plan en el disco                                                                            |
| `planFilePath`   | string | `"/Users/.../plans/refactor-auth.md"`       | Ruta al archivo del plan. Inyectado                                                                                                                        |
| `allowedPrompts` | array  | `[{"tool": "Bash", "prompt": "run tests"}]` | Deprecado. Claude Code acepta el campo pero lo ignora. Antes de v2.1.205, llevaba permisos basados en prompts que Claude solicitó para implementar el plan |

En `PostToolUse`, `tool_response` es un objeto con campos `plan` y `filePath` que contienen el plan aprobado, más banderas de estado internas. Lea `tool_response.plan` para el contenido del plan en lugar de releer el archivo del disco.

<h4 id="pretooluse-decision-control">
  Control de decisión de PreToolUse
</h4>

Los hooks `PreToolUse` pueden controlar si procede una llamada de herramienta. A diferencia de otros hooks que usan un campo `decision` de nivel superior, PreToolUse devuelve su decisión dentro de un objeto `hookSpecificOutput`. Esto le da un control más rico: cuatro resultados (permitir, denegar, preguntar o diferir) más la capacidad de modificar la entrada de herramienta antes de la ejecución.

| Campo                      | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissionDecision`       | `"allow"` omite el aviso de permiso, excepto para las [acciones que ningún modo aprueba automáticamente](/docs/es/permission-modes#actions-no-mode-auto-approves) y para `AskUserQuestion` y `ExitPlanMode`, que necesitan [`updatedInput` emparejado con él](#allow-with-updatedinput). `"deny"` evita la llamada de herramienta. `"ask"` solicita al usuario que confirme. `"defer"` sale correctamente para que la herramienta pueda reanudarse más tarde. Las [reglas de denegación y pregunta](/docs/es/permissions#manage-permissions) aún se evalúan independientemente de lo que devuelva el hook |
| `permissionDecisionReason` | Para `"allow"` y `"ask"`, se muestra al usuario pero no a Claude. Para `"deny"`, se muestra a Claude. Para `"defer"`, se ignora                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `updatedInput`             | Modifica los parámetros de entrada de la herramienta antes de la ejecución. Reemplaza el objeto de entrada completo, así que incluya campos sin cambios junto con los modificados. Claude Code evalúa reglas de permiso y la elegibilidad de [segundo plano automático](/docs/es/tools-reference#background-commands) de un comando Bash contra la entrada que devuelve su hook, no la entrada que envió Claude. Combine con `"allow"` para aprobar automáticamente, o `"ask"` para mostrar la entrada modificada al usuario. Para `"defer"`, se ignora                                              |
| `additionalContext`        | Cadena agregada al contexto de Claude junto con el resultado de la herramienta. Se ignora cuando `permissionDecision` es `"defer"`. Consulte [Agregar contexto para Claude](#add-context-for-claude)                                                                                                                                                                                                                                                                                                                                                                                            |

Cuando múltiples hooks PreToolUse devuelven decisiones diferentes, la precedencia es `deny` > `defer` > `ask` > `allow`.

Un hook que bloquea saliendo con 2 se enruta de la misma manera que `"deny"`: Claude ve el mensaje stderr como la razón de la denegación.

Cuando un hook devuelve `"ask"`, el aviso de permiso mostrado al usuario incluye una etiqueta que identifica de dónde vino el hook: `[settings]` para un hook de cualquier archivo de configuración o del frontmatter del agente, `[plugin:<name>]` para el hook de un plugin, o `[skill]` para un hook del frontmatter de skill. Esto ayuda a los usuarios a entender qué fuente de configuración solicita confirmación.

Un `"ask"` de un hook también fuerza un aviso de permiso en [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode): el clasificador aún puede denegar la llamada de herramienta, pero no puede aprobar la llamada silenciosamente. Antes de v2.1.211, el clasificador podría aprobar un comando Bash ejecutándose fuera del [sandbox](/docs/es/sandboxing) sin mostrar el aviso que solicitó el hook; el clasificador aún aplicaba sus propias reglas de seguridad a ese comando, y una denegación de hook `"deny"` siempre se honraba.

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

<span id="allow-with-updatedinput" />

En [modo no interactivo](/docs/es/headless) con la bandera `-p`, Claude Code ofrece `AskUserQuestion` y `ExitPlanMode` solo cuando la ejecución tiene un [anfitrión de permiso](/docs/es/headless#turn-off-permission-prompts-in-unattended-runs) para recibir el aviso, como una devolución de llamada `canUseTool` de Agent SDK. Estas herramientas requieren interacción del usuario. Devolver `permissionDecision: "allow"` junto con `updatedInput` satisface ese requisito: el hook lee la entrada de la herramienta de stdin, recopila la respuesta a través de su propia interfaz de usuario, y la devuelve en `updatedInput` para que la herramienta se ejecute sin solicitar. Devolver `"allow"` solo no es suficiente para estas herramientas. Para `AskUserQuestion`, devuelva la matriz `questions` original y agregue un objeto [`answers`](#askuserquestion) que asigne el texto de cada pregunta a la respuesta elegida.

A partir de v2.1.199, una herramienta MCP cuyo servidor la marca con [`_meta["anthropic/requiresUserInteraction"]`](/docs/es/mcp#require-approval-for-a-specific-tool) es más estricta: un hook no puede omitir su aviso de aprobación con `"allow"`, con o sin `updatedInput`, porque Claude Code no puede confirmar que el hook recopiló la interacción que la herramienta necesita.

<Note>
  PreToolUse previamente usaba campos `decision` y `reason` de nivel superior, pero estos están deprecados para este evento. Use `hookSpecificOutput.permissionDecision` y `hookSpecificOutput.permissionDecisionReason` en su lugar. Los valores deprecados `"approve"` y `"block"` se asignan a `"allow"` y `"deny"` respectivamente. Otros eventos como PostToolUse y Stop continúan usando `decision` y `reason` de nivel superior como su formato actual.
</Note>

<h4 id="defer-a-tool-call-for-later">
  Diferir una llamada de herramienta para más tarde
</h4>

`"defer"` es para integraciones que ejecutan `claude -p` como un subproceso y leen su salida JSON, como una aplicación Agent SDK o una interfaz de usuario personalizada construida sobre Claude Code. Permite que ese proceso de llamada pause Claude en una llamada de herramienta, recopile entrada a través de su propia interfaz, y reanude donde se quedó. Claude Code honra este valor solo en [modo no interactivo](/docs/es/headless) con la bandera `-p`. En sesiones interactivas registra una advertencia e ignora el resultado del hook.

La herramienta `AskUserQuestion` es el caso típico: Claude quiere hacer una pregunta al usuario, pero no hay terminal para responder. Una ejecución `-p` ofrece `AskUserQuestion` solo cuando tiene un [anfitrión de permiso](/docs/es/headless#turn-off-permission-prompts-in-unattended-runs), como una herramienta MCP que pasa con `--permission-prompt-tool`, así que comience la ejecución con una. El viaje de ida y vuelta funciona así:

1. Claude llama a `AskUserQuestion`. Se dispara el hook `PreToolUse`.
2. El hook devuelve `permissionDecision: "defer"`. La herramienta no se ejecuta. El proceso sale con `stop_reason: "tool_deferred"` y la llamada de herramienta pendiente preservada en la transcripción.
3. El proceso de llamada lee `deferred_tool_use` del resultado de SDK, muestra la pregunta en su propia interfaz de usuario, y espera una respuesta.
4. El proceso de llamada ejecuta `claude -p --resume <session-id>` con el mismo anfitrión de permiso. Se dispara la misma llamada de herramienta `PreToolUse` nuevamente.
5. El hook devuelve `permissionDecision: "allow"` con la respuesta en `updatedInput`. La herramienta se ejecuta y Claude continúa.

El campo `deferred_tool_use` lleva el `id`, `name`, e `input` de la herramienta. El `input` son los parámetros que Claude generó para la llamada de herramienta, capturados antes de la ejecución:

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

No hay límite de tiempo de espera o reintento. La sesión permanece en el disco hasta que la reanude, sujeta al barrido de retención [`cleanupPeriodDays`](/docs/es/settings-reference#cleanupperioddays), que elimina archivos de sesión después de 30 días de forma predeterminada, siguiendo las [reglas de barrido de retención](/docs/es/claude-directory#cleaned-up-automatically). Si la respuesta no está lista cuando reanuda, el hook puede devolver `"defer"` nuevamente y el proceso sale de la misma manera. El proceso de llamada controla cuándo romper el bucle devolviendo finalmente `"allow"` o `"deny"` del hook.

`"defer"` solo funciona cuando Claude realiza una única llamada de herramienta en el turno. Si Claude realiza varias llamadas de herramienta a la vez, `"defer"` se ignora con una advertencia y la herramienta procede a través del flujo de permiso normal. La restricción existe porque reanudar solo puede re-ejecutar una herramienta: no hay forma de diferir una llamada de un lote sin dejar las otras sin resolver.

Si la herramienta diferida ya no está disponible cuando reanuda, el proceso sale con `stop_reason: "tool_deferred_unavailable"` e `is_error: true` antes de que se dispare el hook. Esto sucede cuando un servidor MCP que proporcionó la herramienta no está conectado para la sesión reanudada. La carga `deferred_tool_use` aún se incluye para que pueda identificar qué herramienta desapareció.

<Note>
  Para reanudar una sesión diferida en modo de plan, pase [`--permission-prompt-tool`](/docs/es/cli-reference#cli-flags) junto con `--resume` para que Claude Code pueda presentar el plan para aprobación. Sin él, Claude Code no restaura el modo de plan. Requiere Claude Code v2.1.246 o posterior.

  Cuando reanuda con `-p`, Claude Code no restaura ningún otro modo de permiso almacenado. Comienza la ejecución en el modo de permiso que comenzaría una nueva ejecución `claude -p`, así que pase `--permission-mode` o `--dangerously-skip-permissions` nuevamente si la sesión diferida usó uno. Cuando reanuda con `claude --resume <session-id>` sin `-p`, Claude Code restaura el modo de permiso almacenado, con las excepciones enumeradas en [modo de permiso al reanudar](/docs/es/sessions#permission-mode-on-resume).
</Note>

<h3 id="permissionrequest">
  PermissionRequest
</h3>

Se ejecuta cuando Claude Code está a punto de pedirle permiso para usar una herramienta. En sesiones que no pueden mostrar un aviso, como subagentes en segundo plano en [modo no interactivo](/docs/es/headless), Claude Code aún ejecuta estos hooks, y si ningún hook devuelve una decisión, deniega la llamada de herramienta.
Use [control de decisión PermissionRequest](#permissionrequest-decision-control) para permitir o denegar en nombre del usuario.

Use este evento cuando necesite una señal en el momento en que Claude solicita permiso para usar una herramienta. Claude Code ejecuta un hook [Notification](#notification) con el tipo `permission_prompt` solo después de que el aviso ha esperado aproximadamente seis segundos.

Claude Code no ejecuta hooks PermissionRequest para la [solicitud de red](/docs/es/sandboxing#network-isolation) de un comando en sandbox. Para obtener una señal para ese aviso, use el tipo de notificación `permission_prompt`.

Coincide en el nombre de la herramienta, los mismos valores que PreToolUse.

<h4 id="permissionrequest-input">
  Entrada de PermissionRequest
</h4>

Los hooks PermissionRequest reciben campos `tool_name` e `tool_input` como los hooks PreToolUse, pero sin `tool_use_id`. Para una herramienta MCP, también reciben el objeto [`mcp_server`](#pretooluse-input). Una matriz `permission_suggestions` opcional contiene las [actualizaciones de permiso](#permission-update-entries) que Claude Code sugiere para esta solicitud, como agregar una regla de permiso o cambiar el modo de permiso.

La matriz `permission_suggestions` no es una lista exacta de las opciones que ve, porque cada diálogo de permiso construye sus propias opciones. Algunos diálogos, como el de ediciones de archivo, no leen la matriz en absoluto y derivan sus opciones de la solicitud en sí. Un diálogo que sí la lee aún puede retener una opción cuya sugerencia permanece en la matriz, por ejemplo cuando [`allowManagedPermissionRulesOnly`](/docs/es/settings-reference#allowmanagedpermissionrulesonly) oculta opciones de guardado de reglas. También puede ofrecer opciones que no tienen entrada de sugerencia, como [**Sí, y cambiar a modo automático**](/docs/es/permission-modes#switch-permission-modes), que cambia el modo de permiso directamente en lugar de a través de una actualización de permiso.

Los hooks PreToolUse se ejecutan antes de cada llamada de herramienta, independientemente de si necesita permiso. Los hooks PermissionRequest se ejecutan solo cuando Claude Code está a punto de pedirle permiso, o cuando de otro modo denegaría automáticamente una llamada que no puede solicitar. Ninguno de los dos eventos se dispara para [`EndConversation`](/docs/es/tools-reference#endconversation-tool-behavior).

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
| `behavior`           | `"allow"` otorga el permiso, `"deny"` lo deniega. Las [reglas de denegación y pregunta](/docs/es/permissions#manage-permissions) aún se evalúan, por lo que un hook que devuelve `"allow"` no anula una regla de denegación coincidente                                                |
| `updatedInput`       | Solo para `"allow"`: modifica los parámetros de entrada de la herramienta antes de la ejecución. Reemplaza el objeto de entrada completo, así que incluya campos sin cambios junto con los modificados. La entrada modificada se re-evalúa contra reglas de denegación y pregunta |
| `updatedPermissions` | Solo para `"allow"`: matriz de [entradas de actualización de permiso](#permission-update-entries) a aplicar, como agregar una regla de permiso o cambiar el modo de permiso de la sesión                                                                                          |
| `message`            | Solo para `"deny"`: dice a Claude por qué se denegó el permiso                                                                                                                                                                                                                    |
| `interrupt`          | Solo para `"deny"`: si es `true`, detiene a Claude                                                                                                                                                                                                                                |

Un hook que sale con 2 sin un objeto `decision` deja el flujo de permiso sin cambios, y su stderr se descarta. Solo el objeto `decision` puede otorgar o denegar la solicitud.

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

El campo de salida `updatedPermissions` y el campo de entrada [`permission_suggestions`](#permissionrequest-input) ambos usan la misma matriz de objetos de entrada. Cada entrada tiene un `type` que determina sus otros campos, y un `destination` que controla dónde se escribe el cambio.

| `type`              | Campos                             | Efecto                                                                                                                                                                                                                       |
| :------------------ | :--------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `addRules`          | `rules`, `behavior`, `destination` | Agrega reglas de permiso. `rules` es una matriz de objetos `{toolName, ruleContent?}`. Omita `ruleContent` para hacer coincidir toda la herramienta. `behavior` es `"allow"`, `"deny"`, o `"ask"`                            |
| `replaceRules`      | `rules`, `behavior`, `destination` | Reemplaza todas las reglas del `behavior` dado en el `destination` con las `rules` proporcionadas                                                                                                                            |
| `removeRules`       | `rules`, `behavior`, `destination` | Elimina reglas coincidentes del `behavior` dado                                                                                                                                                                              |
| `setMode`           | `mode`, `destination`              | Cambia el modo de permiso. Los modos válidos son `default`, `auto`, `acceptEdits`, `dontAsk`, `bypassPermissions`, `plan`, y `manual` como alias para `default`. El alias `manual` requiere Claude Code v2.1.200 o posterior |
| `addDirectories`    | `directories`, `destination`       | Agrega directorios de trabajo. `directories` es una matriz de cadenas de ruta                                                                                                                                                |
| `removeDirectories` | `directories`, `destination`       | Elimina directorios de trabajo                                                                                                                                                                                               |

<Note>
  `setMode` con `bypassPermissions` solo toma efecto si inició la sesión con modo de bypass ya disponible: `--dangerously-skip-permissions`, `--permission-mode bypassPermissions`, `--allow-dangerously-skip-permissions`, o `permissions.defaultMode: "bypassPermissions"` en [configuración de usuario, `--settings`, o configuración administrada](/docs/es/settings-reference#permissions-defaultmode). De lo contrario, la actualización es una no-op. La actualización también es una no-op cuando [`permissions.disableBypassPermissionsMode`](/docs/es/permissions#managed-settings) deshabilita el modo, o cuando la sesión comienza en [modo restringido](/docs/es/cli-reference#cli-flags).

  `bypassPermissions` nunca se persiste como `defaultMode` independientemente de `destination`.
</Note>

El campo `destination` en cada entrada determina si el cambio permanece en memoria o persiste en un archivo de configuración.

| `destination`     | Escribe en                                           |
| :---------------- | :--------------------------------------------------- |
| `session`         | solo en memoria, descartado cuando termina la sesión |
| `localSettings`   | `.claude/settings.local.json`                        |
| `projectSettings` | `.claude/settings.json`                              |
| `userSettings`    | `~/.claude/settings.json`                            |

Un hook puede devolver una de las `permission_suggestions` que recibió como su propia salida `updatedPermissions`.

<h3 id="posttooluse">
  PostToolUse
</h3>

Se ejecuta inmediatamente después de que una herramienta se completa exitosamente.

Coincide en el nombre de la herramienta, los mismos valores que PreToolUse.

Coincida más ampliamente cuando el nombre de la herramienta no es el filtro correcto:

* Para ejecutar un hook después de que cualquier herramienta se complete exitosamente, omita el `matcher` o establézcalo en `"*"`. Su hook puede entonces descubrir qué cambió por sí mismo, por ejemplo ejecutando `git status --porcelain`, que también enumera archivos sin seguimiento que `git diff` pierde. Para llamadas de herramienta que fallan, agregue el mismo hook bajo [PostToolUseFailure](#posttoolusefailure).
* Para ejecutar un hook cuando un archivo específico cambia en el disco, sin importar qué lo escribió, use [FileChanged](#filechanged). Claude Code no ejecuta un hook `PostToolUse` que coincida con `Edit|Write` cuando un comando `Bash` o un proceso fuera de Claude Code reescribe el mismo archivo.

<h4 id="posttooluse-input">
  Entrada de PostToolUse
</h4>

Los hooks `PostToolUse` se disparan después de que una herramienta ya se ha ejecutado exitosamente. La entrada incluye tanto `tool_input`, los argumentos enviados a la herramienta, como `tool_response`, el resultado que devolvió. El esquema exacto para ambos depende de la herramienta. Las rutas de herramienta de archivo `tool_input` llegan en el mismo formato que para [PreToolUse](#pretooluse-input): siempre absoluto, con los separadores nativos de la plataforma, así que barras invertidas en Windows. Para una herramienta MCP, la entrada también lleva el objeto [`mcp_server`](#pretooluse-input).

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
    "type": "create"
  },
  "tool_use_id": "toolu_01ABC123...",
  "duration_ms": 12
}
```

| Campo         | Descripción                                                                                                                        |
| :------------ | :--------------------------------------------------------------------------------------------------------------------------------- |
| `duration_ms` | Opcional. Tiempo de ejecución de la herramienta en milisegundos. Excluye el tiempo dedicado a avisos de permiso y hooks PreToolUse |

<h4 id="posttooluse-decision-control">
  Control de decisión de PostToolUse
</h4>

Los hooks `PostToolUse` pueden proporcionar retroalimentación a Claude después de la ejecución de la herramienta. Además de los [campos de salida JSON](#json-output) disponibles para todos los hooks, su script de hook puede devolver estos campos específicos del evento:

| Campo                  | Descripción                                                                                                                                                                                                                                                                                                                                |
| :--------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `decision`             | `"block"` agrega el `reason` junto al resultado de la herramienta. Claude aún ve la salida original; para reemplazarla, use `updatedToolOutput`                                                                                                                                                                                            |
| `reason`               | Explicación mostrada a Claude cuando `decision` es `"block"`                                                                                                                                                                                                                                                                               |
| `additionalContext`    | Cadena agregada al contexto de Claude junto con el resultado de la herramienta. Consulte [Agregar contexto para Claude](#add-context-for-claude)                                                                                                                                                                                           |
| `classifierContext`    | Nota breve sobre el resultado de esta llamada para el clasificador de [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) en lugar de para Claude. Consulte [Anotar un resultado para el clasificador de modo automático](#annotate-a-result-for-the-auto-mode-classifier). Requiere Claude Code v2.1.236 o posterior |
| `updatedToolOutput`    | Reemplaza la salida de la herramienta con el valor proporcionado antes de que se envíe a Claude. El valor debe coincidir con la forma de salida de la herramienta                                                                                                                                                                          |
| `updatedMCPToolOutput` | Reemplaza la salida solo para [herramientas MCP](#match-mcp-tools). Prefiera `updatedToolOutput`, que funciona para todas las herramientas                                                                                                                                                                                                 |

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
  `updatedToolOutput` solo cambia lo que Claude ve. La herramienta ya se ha ejecutado en el momento en que se dispara el hook, por lo que cualquier archivo escrito, comando ejecutado o solicitud de red enviada ya ha tenido efecto. La telemetría como tramos de herramientas OpenTelemetry y eventos de análisis también captura la salida original antes de que se ejecute el hook. Para evitar o modificar una llamada de herramienta antes de que se ejecute, use un hook [PreToolUse](#pretooluse) en su lugar.

  El valor de reemplazo debe coincidir con la forma de salida de la herramienta. Las herramientas integradas devuelven objetos estructurados en lugar de cadenas simples. Por ejemplo, `Bash` devuelve un objeto con campos `stdout`, `stderr`, `interrupted`, e `isImage`. Para herramientas integradas, un valor que no coincida con el esquema de salida de la herramienta se ignora y se usa la salida original. La salida de herramientas MCP se pasa sin validación de esquema. Eliminar detalles de error que Claude necesita puede hacer que continúe con una suposición falsa.
</Warning>

<h4 id="annotate-a-result-for-the-auto-mode-classifier">
  Anotar un resultado para el clasificador de modo automático
</h4>

Devuelva `classifierContext` para enviar una nota breve sobre el resultado de la llamada de herramienta al clasificador de [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) en lugar de a Claude. El clasificador [nunca recibe resultados de herramientas en sí](/docs/es/permission-modes#how-the-classifier-evaluates-actions), por lo que este campo es la forma admitida de decirle algo sobre lo que devolvió una llamada antes de que revise acciones posteriores. El campo requiere Claude Code v2.1.236 o posterior.

El ejemplo a continuación dice al clasificador de dónde vino la salida de una consulta:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "classifierContext": "This query ran against the staging database, not production."
  }
}
```

Cuánto peso da el clasificador a la nota depende de dónde configuró el hook:

* **Hooks configurados en Claude Code**: para hooks de archivos de configuración, plugins, skills y frontmatter del agente, el clasificador trata la nota como contexto no verificado proporcionado por la aplicación. La nota nunca establece la intención del usuario, y si afirma que aprobó o solicitó algo, el clasificador verifica esa afirmación contra sus propios mensajes en la conversación
* **Devoluciones de llamada de Agent SDK en proceso**: cuando una aplicación que integra Claude Code registra el hook como una [devolución de llamada de SDK de TypeScript](/docs/es/agent-sdk/hooks) y devuelve la nota durante la sesión en vivo, el clasificador puede pesar una declaración del usuario retransmitida en la nota como intención del usuario. Tal declaración puede satisfacer un requisito de consentimiento que el clasificador aceptaría de un mensaje que envía, pero nunca levanta un bloqueo que su propio mensaje tampoco podría levantar. Después de que se reanuda una sesión, Claude Code trata las notas restauradas como contexto no verificado. Cuando hooks de ambos grupos anotan la misma llamada, el clasificador trata la nota combinada como contexto no verificado

Claude Code aplica estos límites al entregar la nota:

* **Longitud**: Claude Code limita las notas para una llamada de herramienta a 2,000 caracteres y trunca el resto. El límite se comparte en cada hook que responde a esa llamada
* **Solo respuestas sincrónicas**: Claude Code ignora el campo en la respuesta de un hook que [se ejecuta en segundo plano](#run-hooks-in-the-background), porque esa respuesta llega después de que Claude Code registra el resultado de la herramienta
* **Llamadas que el clasificador no registra**: la transcripción del clasificador omite búsquedas de solo lectura como lecturas de archivo y búsquedas. Claude Code descarta una nota adjunta a una de esas llamadas
* **Interacción con reescrituras**: cuando la nota describe salida que está reemplazando con `updatedToolOutput`, devuelva ambos campos en la misma respuesta del hook. Claude Code descarta la nota si esa reescritura se rechaza u otro hook la reemplaza. Claude Code entrega una nota que devuelve sin una reescritura incluso cuando otro hook reescribe la salida

<Warning>
  El clasificador lee el contenido que coloca en `classifierContext` como información del anfitrión de la aplicación de la sesión, así que no copie salida de herramienta no confiable o texto de terceros en él. Mantenga la nota a una afirmación breve sobre esta única llamada, como un hecho sobre su origen o una declaración del usuario sobre ella; no use el campo para entregar mensajes no relacionados o un flujo de eventos.
</Warning>

<h3 id="posttoolusefailure">
  PostToolUseFailure
</h3>

Se ejecuta cuando una herramienta que comenzó a ejecutarse falla: la herramienta lanzó un error, o una herramienta MCP devolvió un resultado de error. Úselo para registrar fallas, enviar alertas o proporcionar retroalimentación correctiva a Claude.

Coincide en el nombre de la herramienta, los mismos valores que PreToolUse.

<Note>
  Este evento no se dispara para llamadas de herramienta rechazadas antes de la ejecución: un nombre de herramienta desconocido, entrada que falla en validación de esquema o específica de herramienta, o una denegación de permiso. Los rechazos de validación se devuelven como resultados `tool_use_error` y ocurren antes de que se ejecuten los hooks, por lo que no disparan ni `PreToolUse` ni `PostToolUseFailure`. Las denegaciones de permiso disparan `PreToolUse` pero no este evento; consulte [PermissionDenied](#permissiondenied).
</Note>

<h4 id="posttoolusefailure-input">
  Entrada de PostToolUseFailure
</h4>

Los hooks PostToolUseFailure reciben los mismos campos `tool_name` e `tool_input` que PostToolUse, junto con información de error como campos de nivel superior. Para una herramienta MCP, también reciben el objeto [`mcp_server`](#pretooluse-input). Por ejemplo, un comando `npm test` fallido podría entregar:

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
  "error": "Exit code 1\nError: Cannot find module 'express'",
  "is_interrupt": false,
  "duration_ms": 4187
}
```

| Campo          | Descripción                                                                                                                                                                                                                                                                          |
| :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `error`        | Cadena que describe qué salió mal. El formato depende de la herramienta que falló                                                                                                                                                                                                    |
| `is_interrupt` | Booleano opcional. Verdadero cuando el fallo llegó a Claude Code como una interrupción en lugar de como un error que reportó la herramienta. Cancelar una herramienta en ejecución no dispara este hook; el resultado de la herramienta lleva el mensaje de interrupción en su lugar |
| `duration_ms`  | Opcional. Tiempo de ejecución de la herramienta en milisegundos. Excluye el tiempo dedicado a avisos de permiso y hooks PreToolUse                                                                                                                                                   |

La cadena `error` es generalmente el mismo texto que Claude recibe como resultado fallido de la herramienta. Su formato varía según la herramienta y el fallo. Clave su hook en `tool_name`, `is_interrupt`, y la primera línea `Exit code N`; trate el resto de la cadena como texto de visualización, no como un formato estable.

* Para Bash y PowerShell, un comando que se ejecutó y salió produce una primera línea `Exit code N`, luego cualquier salida que el comando produjo como un bloque con stdout y stderr intercalados
* Una carga también puede llevar un mensaje de fallo desnudo sin línea de código de salida, cuando Claude Code no pudo iniciar el proceso de shell en sí
* Claude Code trunca a mitad de cadenas largas alrededor de un marcador `... [N characters truncated] ...`, e puede insertar líneas propias, como `Command timed out after 2m 0s`

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

Se ejecuta una vez después de que cada llamada de herramienta en un lote se haya resuelto, antes de que Claude Code envíe la siguiente solicitud al modelo. `PostToolUse` se dispara una vez por herramienta, lo que significa que se dispara simultáneamente cuando Claude realiza llamadas de herramienta paralelas. `PostToolBatch` se dispara exactamente una vez con el lote completo, por lo que es el lugar correcto para inyectar contexto que dependa del conjunto de herramientas que se ejecutaron en lugar de en cualquier herramienta única. No hay matcher para este evento.

<h4 id="posttoolbatch-input">
  Entrada de PostToolBatch
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks PostToolBatch reciben `tool_calls`, una matriz que describe cada llamada de herramienta en el lote:

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

`tool_response` contiene el mismo contenido que el modelo recibe en el bloque `tool_result` correspondiente. El valor es una cadena serializada o matriz de bloques de contenido, exactamente como lo emitió la herramienta. Para `Read`, eso significa texto con prefijo de número de línea en lugar de contenidos de archivo sin procesar. Las respuestas pueden ser grandes, así que analice solo los campos que necesita.

<Note>
  La forma `tool_response` difiere de la de `PostToolUse`. `PostToolUse` pasa el objeto `Output` estructurado de la herramienta, como `{filePath: "...", type: "create"}` para `Write`; `PostToolBatch` pasa el contenido `tool_result` serializado que ve el modelo.
</Note>

<h4 id="posttoolbatch-decision-control">
  Control de decisión de PostToolBatch
</h4>

Los hooks `PostToolBatch` pueden inyectar contexto para Claude. Además de los [campos de salida JSON](#json-output) disponibles para todos los hooks, su script de hook puede devolver estos campos específicos del evento:

| Campo               | Descripción                                                                                                                                                                                                                                      |
| :------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | Cadena de contexto inyectada una vez antes de la siguiente llamada del modelo. Consulte [Agregar contexto para Claude](#add-context-for-claude) para detalles de entrega, qué poner en él y cómo las sesiones reanudadas manejan valores pasados |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolBatch",
    "additionalContext": "These files are part of the ledger module. Run pytest before marking the task complete."
  }
}
```

Devolver `decision: "block"` o `continue: false` detiene el bucle agéntico antes de la siguiente llamada del modelo. El mensaje de bloqueo viene del JSON `reason` o `stopReason`, o de stderr en salida 2. Lo ve como una advertencia en la transcripción, y permanece en la conversación, por lo que Claude lo ve cuando la conversación continúa.

<h3 id="permissiondenied">
  PermissionDenied
</h3>

Se ejecuta cuando [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) deniega una llamada de herramienta, incluido cuando deniega sin un veredicto del clasificador porque [una verificación de seguridad separada del modo automático rechazó la propia solicitud del clasificador](/docs/es/errors#auto-mode-cannot-determine-the-safety-of-an-action) o su respuesta no se analizó. Este hook solo se dispara en modo automático: no se ejecuta cuando deniega manualmente un diálogo de permiso, cuando un hook `PreToolUse` bloquea una llamada, o cuando una regla `deny` coincide. Úselo para registrar denegaciones, ajustar configuración o decirle al modelo que puede reintentar la llamada de herramienta.

Coincide en el nombre de la herramienta, los mismos valores que PreToolUse.

<h4 id="permissiondenied-input">
  Entrada de PermissionDenied
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks PermissionDenied reciben `tool_name`, `tool_input`, `tool_use_id`, y `reason`. Para una herramienta MCP, también reciben el objeto [`mcp_server`](#pretooluse-input).

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
  "reason": "[Irreversible Local Destruction]"
}
```

| Campo    | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| :------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `reason` | La razón de la denegación. Para un veredicto del clasificador, en la mayoría de sesiones nombra la regla coincidente entre corchetes, como `[Data Exfiltration]`; consulte [Revisar denegaciones](/docs/es/auto-mode-config#review-denials) para las otras formas. Para una [denegación sin veredicto](#permissiondenied-decision-control), comienza con `Auto mode could not evaluate this action and is blocking it for safety`. Para una denegación porque el modelo del clasificador no estaba disponible, es el texto fijo `Classifier unavailable` |

<h4 id="permissiondenied-decision-control">
  Control de decisión de PermissionDenied
</h4>

Los hooks PermissionDenied pueden decirle al modelo que puede reintentar la llamada de herramienta denegada. Devuelva un objeto JSON con `hookSpecificOutput.retry` establecido en `true`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionDenied",
    "retry": true
  }
}
```

Cuando `retry` es `true`, Claude Code agrega un mensaje a la conversación diciéndole al modelo que puede reintentar la llamada de herramienta. Claude Code no revierte la denegación en sí. Si su hook no devuelve JSON, o devuelve `retry: false`, la denegación se mantiene y el modelo recibe el mensaje de rechazo original.

Claude Code ignora `retry: true` cuando el clasificador produjo [ningún veredicto sobre la acción](/docs/es/errors#auto-mode-cannot-determine-the-safety-of-an-action): su respuesta no se analizó, o una verificación de seguridad separada del modo automático rechazó la propia solicitud del clasificador. Para esas denegaciones, Claude Code ya le dice al modelo en el mensaje de rechazo si reintentar más tarde o continuar.

<h3 id="notification">
  Notification
</h3>

Se ejecuta cuando Claude Code envía notificaciones. Coincide en el tipo de notificación. Omita el matcher para ejecutar hooks para todos los tipos de notificación.

Recibe estos eventos de hook incluso con notificaciones de escritorio desactivadas: la configuración `preferredNotifChannel`, incluida `notifications_disabled`, cambia solo cómo se le alerta, no si su hook se ejecuta.

| Matcher                      | Cuándo se dispara                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :--------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permission_prompt`          | Claude necesita su permiso para usar una herramienta o una [solicitud de red](/docs/es/sandboxing#network-isolation) de un comando en sandbox, y el aviso ha esperado aproximadamente seis segundos                                                                                                                                                                                                                                                                                                       |
| `idle_prompt`                | Claude terminó de responder hace aproximadamente 60 segundos y no ha escrito desde entonces                                                                                                                                                                                                                                                                                                                                                                                                          |
| `auth_success`               | La autenticación se completa                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `elicitation_dialog`         | Un servidor MCP abre un formulario de elicitación y no ha escrito durante aproximadamente seis segundos                                                                                                                                                                                                                                                                                                                                                                                              |
| `elicitation_url_dialog`     | Un servidor MCP le pide que abra una URL de navegador y no ha escrito durante aproximadamente seis segundos                                                                                                                                                                                                                                                                                                                                                                                          |
| `elicitation_complete`       | Un servidor MCP reporta que una [elicitación de modo URL](#elicitation-input) está completa                                                                                                                                                                                                                                                                                                                                                                                                          |
| `elicitation_response`       | Se envía una respuesta de elicitación de MCP de vuelta al servidor                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `agent_needs_input`          | Una sesión de fondo comienza a esperar su entrada mientras [vista de agente](/docs/es/agent-view) está abierta en una terminal, o la sesión actual le hace una pregunta de configuración de terminal de [compañero de equipo del agente](/docs/es/agent-teams#choose-a-display-mode) y no ha escrito durante aproximadamente seis segundos                                                                                                                                                                     |
| `agent_completed`            | Una sesión de fondo se completa o falla. Se dispara solo mientras [vista de agente](/docs/es/agent-view) está abierta en una terminal                                                                                                                                                                                                                                                                                                                                                                     |
| `quota_auto_resume_fired`    | Claude Code continúa su tarea después de que un límite de uso de claude.ai la pausó: en el reinicio, o antes cuando algo que hace en Claude Code durante la espera, como agregar créditos de uso, actualizar su plan o cambiar modelos, hace que el uso esté disponible nuevamente, con la [excepción de configuración de modelo](/docs/es/interactive-mode#wait-for-a-usage-limit-to-reset)                                                                                                              |
| `quota_auto_resume_stale`    | Un límite de uso de claude.ai se reinició mientras su computadora dormía durante más de aproximadamente 30 minutos. Claude Code espera a que presione `Enter` en lugar de continuar. Después de un sueño más corto continúa y dispara `quota_auto_resume_fired` en su lugar                                                                                                                                                                                                                          |
| `quota_auto_resume_disabled` | Claude Code termina su espera por un límite de uso de claude.ai sin continuar su tarea: [`autoContinueAtUsageLimit`](/docs/es/settings-reference#autocontinueatusagelimit) se desactivó o el reinicio se movió más de 24 horas en el futuro durante una espera que Claude Code comenzó por su cuenta, la tarea continuada siguió golpeando el límite, o la continuación fue bloqueada antes de llegar al modelo. No se dispara cuando presiona `Esc` o `Ctrl+C`, o elige **No continuar automáticamente** |

Los tipos `agent_needs_input` y `agent_completed` requieren Claude Code v2.1.198 o posterior.

Los tipos `quota_auto_resume_fired`, `quota_auto_resume_stale`, y `quota_auto_resume_disabled` requieren Claude Code v2.1.234 o posterior.

En sesiones de terminal, `permission_prompt` para una solicitud de red de un comando en sandbox requiere Claude Code v2.1.246 o posterior.

`agent_needs_input` para una pregunta de configuración de terminal de compañero requiere Claude Code v2.1.248 o posterior.

<Note>
  Los tipos `permission_prompt`, `idle_prompt`, `elicitation_dialog`, y `elicitation_url_dialog` comparten su tiempo con notificaciones de escritorio, así que en sesiones de terminal solo los ve cuando parece que está lejos de la terminal:

  * Espere `permission_prompt` una vez que no haya escrito durante aproximadamente seis segundos. El temporizador comienza cuando aparece el aviso de permiso, y cada pulsación de tecla lo difiere. Para ejecutar un hook inmediatamente cuando Claude solicita permiso para usar una herramienta, use [PermissionRequest](#permissionrequest) en su lugar.
  * Espere `idle_prompt` aproximadamente 60 segundos después de que Claude termine de responder, y solo si no ha escrito desde entonces. Claude Code no envía `idle_prompt` mientras espera a que se reinicie un límite de uso de claude.ai. Cuando la espera termina por sí sola, uno de los tipos `quota_auto_resume_*` se dispara en su lugar.
  * Espere `elicitation_dialog` para un formulario de elicitación, o `elicitation_url_dialog` para una solicitud de URL de navegador, una vez que no haya escrito durante aproximadamente seis segundos. Ambos comparten la misma puerta de seis segundos que `permission_prompt`: el temporizador comienza cuando aparece el diálogo, y cada pulsación de tecla lo difiere.

  Una solicitud de permiso o elicitación que llega mientras otro diálogo está en pantalla mantiene la misma puerta de seis segundos, cronometrada desde cuando llega la solicitud. Su notificación puede llegar a usted mientras la solicitud aún espera detrás del diálogo abierto.
</Note>

Claude Code cronometra `permission_prompt` diferente en sesiones donde envía solicitudes de permiso a la devolución de llamada [`canUseTool`](/docs/es/agent-sdk/user-input) de Agent SDK, que es cómo Claude Desktop y la extensión VS Code alojan Claude Code:

* Espere `permission_prompt` aproximadamente seis segundos después de que Claude solicita permiso. Claude Code no lo difiere mientras escribe.
* Si usted o un hook [PermissionRequest](#permissionrequest) responden antes, Claude Code no ejecuta `permission_prompt`.
* Establezca [`CLAUDE_CODE_DISABLE_PERMISSION_PROMPT_NOTIFY_HOOKS`](/docs/es/env-vars) en `1` para desactivar `permission_prompt` en estas sesiones.

Antes de v2.1.233, `permission_prompt` no se disparaba en estas sesiones.

Use matchers separados para ejecutar diferentes manejadores dependiendo del tipo de notificación. Esta configuración dispara un script de alerta específico de permiso cuando Claude necesita aprobación de permiso y una notificación diferente cuando Claude ha estado inactivo:

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

Además de los [campos de entrada comunes](#common-input-fields), los hooks Notification reciben `message` con el texto de notificación, un `title` opcional, y `notification_type` que indica qué tipo se disparó.

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

Los hooks Notification no pueden bloquear o modificar notificaciones. Claude Code descarta sus campos `systemMessage` y `continue` pero aún emite [`terminalSequence`](#emit-terminal-notifications), en el que se basa el ejemplo de notificación de escritorio. Los hooks Notification están destinados a efectos secundarios como reenviar la notificación a un servicio externo.

<h3 id="subagentstart">
  SubagentStart
</h3>

Se ejecuta cuando Claude genera un subagente con la herramienta Agent, cuando Claude [reanuda un subagente](/docs/es/sub-agents#resume-subagents), y cada vez que un compañero de [equipo de agente](/docs/es/agent-teams) en proceso maneja un nuevo mensaje. Admite matchers para filtrar por nombre de tipo de agente. Para agentes integrados, este es el nombre del agente como `general-purpose`, `Explore`, o `Plan`. Para [subagentes personalizados](/docs/es/sub-agents), este es el campo `name` del frontmatter del agente, no el nombre del archivo.

Para subagentes enviados por un [plugin](/docs/es/plugins), el tipo de agente es el identificador con alcance de plugin como `my-plugin:reviewer`, no el nombre de frontmatter desnudo. Los dos puntos colocan un nombre con alcance de plugin en la ruta de expresión regular, así que ancle el matcher con `^` y `$` para una coincidencia exacta: `^my-plugin:reviewer$`.

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

Los hooks SubagentStart no pueden bloquear la creación del subagente, pero pueden inyectar contexto en el subagente. Además de los [campos de salida JSON](#json-output) disponibles para todos los hooks, puede devolver:

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

Cuando el hook se ejecuta nuevamente para el mismo subagente, Claude Code inyecta el contexto devuelto solo cuando el contexto del subagente no contiene ya la copia de una ejecución anterior. La copia inyectada al inicio permanece en su lugar, dejando el [caché de prompt](/docs/es/prompt-caching#subagents-and-the-cache) del subagente intacto. Después de que la [compactación automática](/docs/es/sub-agents#auto-compaction) descarta esa copia, Claude Code inyecta el contexto de la siguiente ejecución nuevamente.

<h3 id="subagentstop">
  SubagentStop
</h3>

Se ejecuta cuando un subagente de Claude Code ha terminado de responder. Coincide en el tipo de agente, los mismos valores que SubagentStart.

<h4 id="subagentstop-input">
  Entrada de SubagentStop
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks SubagentStop reciben `stop_hook_active`, `agent_id`, `agent_type`, `agent_transcript_path`, y `last_assistant_message`. El campo `agent_type` es el valor utilizado para filtrado de matcher. El `transcript_path` es la transcripción de la sesión principal, mientras que `agent_transcript_path` es la propia transcripción del subagente almacenada en una carpeta `subagents/` anidada. El campo `last_assistant_message` contiene el contenido de texto de la respuesta final del subagente, por lo que los hooks pueden acceder a él sin analizar el archivo de transcripción.

No cada evento SubagentStop proviene de un subagente que Claude generó. Claude Code también ejecuta agentes internos para algunas de sus propias características, como [sugerencias de prompt](/docs/es/interactive-mode#prompt-suggestions) y [preguntas secundarias `/btw`](/docs/es/interactive-mode#side-questions-with-%2Fbtw), y SubagentStop se dispara cuando uno de esos termina también. Para esos eventos, `agent_type` es el nombre del agente que la sesión en sí ejecuta, como uno establecido con [`--agent`](/docs/es/cli-reference#cli-flags) o la configuración [`agent`](/docs/es/settings-reference#agent), y una cadena vacía cuando la sesión se ejecuta sin uno.

Un `matcher` que nombra tipos de agente no coincide con un `agent_type` vacío. Un hook cuyo matcher está omitido, `""`, o `"*"`, o es una expresión regular que coincide con una cadena vacía, se ejecuta para eventos con un `agent_type` vacío también.

En Claude Code v2.1.271 o posterior, un subagente que se ejecuta con la herramienta [`SubagentHandback`](/docs/es/tools-reference) entrega su informe a través de esa herramienta antes de que se detenga. El campo `last_assistant_message` luego contiene el texto de cierre del subagente, si lo hay, que no es el informe entregado. El informe es la entrada `message` de esa llamada, que un hook `PreToolUse` o `PostToolUse` que coincida en `SubagentHandback` recibe como `tool_input.message`.

Los hooks SubagentStop también reciben las matrices `background_tasks` y `session_crons` descritas en [Entrada de Stop](#stop-input). Ambas matrices tienen alcance a la sesión padre, no al subagente.

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

Los hooks SubagentStop usan el mismo formato de control de decisión que [hooks Stop](#stop-decision-control), incluido `hookSpecificOutput.additionalContext` con `hookEventName` establecido en `"SubagentStop"`, para retroalimentación sin error que mantiene el subagente ejecutándose. Devolver `decision: "block"` con un `reason` mantiene el subagente ejecutándose y entrega `reason` al subagente como su siguiente instrucción. Un hook que bloquea saliendo con 2 entrega su mensaje stderr de la misma manera. Para inyectar contexto en la sesión padre después de que un subagente devuelve, use un hook [`PostToolUse`](#posttooluse) en la herramienta `Agent` en su lugar.

<h3 id="taskcreated">
  TaskCreated
</h3>

Se ejecuta cuando se está creando una tarea a través de la herramienta `TaskCreate`. Úselo para hacer cumplir convenciones de nomenclatura, requerir descripciones de tareas o evitar que se creen ciertas tareas. En una [sesión sin las herramientas Task](/docs/es/tools-reference#task-tool-availability), este evento no se dispara.

Los hooks TaskCreated no admiten matchers y se disparan en cada ocurrencia.

<h4 id="taskcreated-input">
  Entrada de TaskCreated
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks TaskCreated reciben `task_id`, `task_subject`, y opcionalmente `task_description`, `teammate_name`, y `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "TaskCreated",
  "task_id": "task-001",
  "task_subject": "Implement user authentication",
  "task_description": "Add login and signup endpoints",
  "teammate_name": "implementer",
  "team_name": "session-a1b2c3d4"
}
```

| Campo              | Descripción                                                                        |
| :----------------- | :--------------------------------------------------------------------------------- |
| `task_id`          | Identificador de la tarea que se está creando                                      |
| `task_subject`     | Título de la tarea                                                                 |
| `task_description` | Descripción detallada de la tarea. Puede estar ausente                             |
| `teammate_name`    | Nombre del compañero que está creando la tarea. Puede estar ausente                |
| `team_name`        | Deprecado. Nombre de equipo derivado de sesión; se eliminará en una versión futura |

<h4 id="taskcreated-decision-control">
  Control de decisión de TaskCreated
</h4>

Un hook TaskCreated puede bloquear la creación de dos formas. De cualquier manera, Claude Code elimina la tarea y devuelve su mensaje a Claude como el error de la herramienta. Claude Code ignora `continue: false` de este evento y Claude continúa trabajando.

* **Código de salida 2**: Claude Code devuelve el texto stderr como el mensaje.
* **JSON `{"decision": "block", "reason": "..."}`**: Claude Code devuelve `reason` como el mensaje.

Este ejemplo bloquea tareas cuyos asuntos no siguen el formato requerido:

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

Se ejecuta cuando se está marcando una tarea como completada. Esto se dispara en dos situaciones: cuando cualquier agente marca explícitamente una tarea como completada a través de la herramienta TaskUpdate, o cuando un compañero de [equipo de agente](/docs/es/agent-teams) termina su turno con tareas en progreso. Úselo para hacer cumplir criterios de finalización como pasar pruebas o verificaciones de lint antes de que una tarea pueda cerrarse.

Los hooks TaskCompleted no admiten matchers y se disparan en cada ocurrencia.

<h4 id="taskcompleted-input">
  Entrada de TaskCompleted
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks TaskCompleted reciben `task_id`, `task_subject`, y opcionalmente `task_description`, `teammate_name`, y `team_name`.

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

| Campo              | Descripción                                                                        |
| :----------------- | :--------------------------------------------------------------------------------- |
| `task_id`          | Identificador de la tarea que se está completando                                  |
| `task_subject`     | Título de la tarea                                                                 |
| `task_description` | Descripción detallada de la tarea. Puede estar ausente                             |
| `teammate_name`    | Nombre del compañero que está completando la tarea. Puede estar ausente            |
| `team_name`        | Deprecado. Nombre de equipo derivado de sesión; se eliminará en una versión futura |

<h4 id="taskcompleted-decision-control">
  Control de decisión de TaskCompleted
</h4>

Los hooks TaskCompleted admiten dos formas de controlar la finalización de tareas:

* **Código de salida 2**: la tarea no se marca como completada y el mensaje stderr se devuelve al modelo como retroalimentación.
* **JSON `{"continue": false, "stopReason": "..."}`**: cuando un compañero que termina su turno disparó el evento, detiene completamente al compañero, coincidiendo con el comportamiento del hook `Stop`. El `stopReason` se muestra al usuario. Cuando la herramienta `TaskUpdate` disparó el evento, Claude Code ignora `continue: false`; el código de salida 2 aún bloquea la finalización.

Este ejemplo ejecuta pruebas y bloquea la finalización de tareas si fallan:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
TASK_SUBJECT=$(echo "$INPUT" | jq -r '.task_subject')

# Run the test suite
if ! npm test 2>&1; then
  echo "Tests not passing. Fix failing tests before completing: $TASK_SUBJECT" >&2
  exit 2
fi

exit 0
```

<h3 id="stop">
  Stop
</h3>

Se ejecuta cuando el agente principal de Claude Code ha terminado de responder. No se ejecuta si la detención ocurrió debido a una interrupción del usuario. Los errores de API disparan [StopFailure](#stopfailure) en su lugar.

<Tip>
  El comando [`/goal`](/docs/es/goal) es un atajo integrado para un hook Stop basado en prompt con alcance de sesión. Úselo cuando quiera que Claude continúe trabajando hacia una condición sin escribir configuración de hook.
</Tip>

<h4 id="stop-input">
  Entrada de Stop
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks Stop reciben `stop_hook_active`, `last_assistant_message`, `background_tasks`, y `session_crons`. El campo `stop_hook_active` es `true` cuando Claude Code ya está continuando como resultado de un hook stop. Verifique este valor o procese la transcripción para evitar bloquear en una condición que nunca se resolverá. Claude Code anula el hook y termina el turno después de 8 bloqueos consecutivos.

El campo `last_assistant_message` contiene el contenido de texto de la respuesta final de Claude, por lo que los hooks pueden acceder a él sin analizar el archivo de transcripción. Para hooks que actúan en el turno recién completado, como hooks de lectura en voz alta o notificación, use este campo en lugar de leer `transcript_path`: el archivo de transcripción no se garantiza que incluya el mensaje final en el tiempo de Stop en todas las versiones.

Las matrices `background_tasks` y `session_crons` permiten que los hooks distingan "sesión hecha" de "sesión pausada esperando que el trabajo de fondo la despierte". Ambas matrices están presentes cuando el registro de tareas es accesible y están vacías cuando nada está en vuelo o programado.

Cada entrada en `background_tasks` describe una tarea en vuelo y usa estos campos:

| Campo         | Descripción                                                                                                                                                                                                                                                             |
| :------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`          | Identificador de tarea                                                                                                                                                                                                                                                  |
| `type`        | Etiqueta de tipo de tarea amigable como `shell`, `subagent`, `monitor`, `workflow`, `teammate`, `cloud session`, o `MCP task`. Cada etiqueta identifica qué característica de Claude Code creó la tarea. Vuelve al discriminante sin procesar para tipos no reconocidos |
| `status`      | Estado actual de la tarea                                                                                                                                                                                                                                               |
| `description` | Descripción de texto libre, limitada a 1000 caracteres con un marcador `… [+N chars]` en cadena cuando se recorta                                                                                                                                                       |
| `command`     | Línea de comando de shell, limitada a 1000 caracteres. Presente solo para tareas `shell`                                                                                                                                                                                |
| `agent_type`  | Nombre de tipo de subagente. Presente solo para tareas `subagent`                                                                                                                                                                                                       |
| `server`      | Nombre del servidor MCP. Presente solo para tareas `monitor` y `MCP task`                                                                                                                                                                                               |
| `tool`        | Nombre de herramienta MCP. Presente solo para tareas `monitor` y `MCP task`                                                                                                                                                                                             |
| `name`        | Nombre del flujo de trabajo. Presente solo para tareas `workflow`                                                                                                                                                                                                       |

Cada entrada en `session_crons` describe un despertar programado con alcance de sesión, originado de `CronCreate`, `ScheduleWakeup`, y `/loop`:

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

Un hook que bloquea saliendo con 2 se enruta de la misma manera que `reason`: Claude recibe el mensaje stderr como la explicación de por qué debe continuar.

```json theme={null}
{
  "decision": "block",
  "reason": "Must be provided when Claude is blocked from stopping"
}
```

Use `additionalContext` cuando el hook está funcionando como se diseñó y dando orientación a Claude, como "ejecutar el conjunto de pruebas antes de terminar". Mantiene la conversación a través de las mismas protecciones de bucle que `decision: "block"`, es decir, la entrada `stop_hook_active` y el límite de 8 continuaciones consecutivas, pero la transcripción la etiqueta como `Stop hook feedback` y no se muestra ninguna notificación de error de hook:

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

Se ejecuta en lugar de [Stop](#stop) cuando el turno termina debido a un error de API. Claude Code ignora la salida y el código de salida del hook, aparte de [`terminalSequence`](#emit-terminal-notifications). Úselo para registrar fallas, enviar alertas o tomar acciones de recuperación cuando Claude no puede completar una respuesta debido a límites de velocidad, problemas de autenticación u otros errores de API.

<h4 id="stopfailure-input">
  Entrada de StopFailure
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks StopFailure reciben `error`, `error_details` opcional, y `last_assistant_message` opcional. El campo `error` identifica el tipo de error y se utiliza para filtrado de matcher.

| Campo                    | Descripción                                                                                                                                                                                                                                                           |
| :----------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `error`                  | Tipo de error: `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `account_on_hold`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `cloud_credential_error`, o `unknown`                     |
| `error_details`          | Detalles adicionales sobre el error, cuando esté disponible                                                                                                                                                                                                           |
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

Se ejecuta cuando un compañero de [equipo de agente](/docs/es/agent-teams) está a punto de quedarse inactivo después de terminar su turno. Úselo para hacer cumplir puertas de calidad antes de que un compañero deje de trabajar, como requerir que pasen verificaciones de lint o verificar que existan archivos de salida.

Los hooks TeammateIdle no admiten matchers y se disparan en cada ocurrencia.

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

| Campo           | Descripción                                                                        |
| :-------------- | :--------------------------------------------------------------------------------- |
| `teammate_name` | Nombre del compañero que está a punto de quedarse inactivo                         |
| `team_name`     | Deprecado. Nombre de equipo derivado de sesión; se eliminará en una versión futura |

<h4 id="teammateidle-decision-control">
  Control de decisión de TeammateIdle
</h4>

Los hooks TeammateIdle admiten dos formas de controlar el comportamiento del compañero:

* **Código de salida 2**: el compañero recibe el mensaje stderr como retroalimentación y continúa trabajando en lugar de quedarse inactivo.
* **JSON `{"continue": false, "stopReason": "..."}`**: detiene completamente al compañero, coincidiendo con el comportamiento del hook `Stop`. El `stopReason` se muestra al usuario.

Este ejemplo verifica que existe un artefacto de compilación antes de permitir que un compañero se quede inactivo:

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

Se ejecuta cuando un archivo de configuración cambia durante una sesión. Úselo para auditar cambios de configuración, hacer cumplir políticas de seguridad o bloquear modificaciones no autorizadas en archivos de configuración.

Claude Code ejecuta hooks ConfigChange cuando un archivo de configuración, un archivo de política administrada o un archivo de skill cambia. Para política administrada, solo los ejecuta cuando `managed-settings.json` o un archivo en `managed-settings.d/` cambia. Aplica [configuración administrada por servidor](/docs/es/server-managed-settings) y cambios en preferencias administradas de macOS o política de registro de Windows sin ejecutarlos. En WSL con [`wslInheritsWindowsSettings`](/docs/es/settings-reference#wslinheritswindowssettings), también aplica un archivo de configuración administrada de Windows modificado en su sondeo de política sin ejecutarlos.

El matcher filtra en la fuente de configuración:

| Matcher            | Cuándo se dispara                                                    |
| :----------------- | :------------------------------------------------------------------- |
| `user_settings`    | `~/.claude/settings.json` cambia                                     |
| `project_settings` | `.claude/settings.json` cambia                                       |
| `local_settings`   | `.claude/settings.local.json` cambia                                 |
| `policy_settings`  | `managed-settings.json` o un archivo en `managed-settings.d/` cambia |
| `skills`           | Un archivo de skill en `.claude/skills/` cambia                      |

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

Los hooks ConfigChange pueden bloquear cambios de configuración para que no surtan efecto. Use código de salida 2 o un JSON `decision` para evitar el cambio. Cuando se bloquea, la nueva configuración no se aplica a la sesión en ejecución.

| Campo      | Descripción                                                                              |
| :--------- | :--------------------------------------------------------------------------------------- |
| `decision` | `"block"` evita que se aplique el cambio de configuración. Omita para permitir el cambio |
| `reason`   | Aceptado pero nunca mostrado                                                             |

```json theme={null}
{
  "decision": "block",
  "reason": "Configuration changes to project settings require admin approval"
}
```

Los cambios `policy_settings` no se pueden bloquear. Los hooks aún se disparan para fuentes `policy_settings` cuando un archivo de configuración administrada en la máquina cambia, para que pueda usarlos para registrar esas ediciones, pero cualquier decisión de bloqueo se ignora. Esto asegura que la configuración administrada por empresa siempre surta efecto. Claude Code no ejecuta hooks `ConfigChange` cuando llegan o se actualizan [configuración administrada por servidor](/docs/es/server-managed-settings).

Claude Code actúa sobre la decisión de bloqueo de la salida JSON de un hook ConfigChange y descarta `systemMessage` y `continue`. Un cambio bloqueado no muestra ningún mensaje a usted o a Claude, ya sea que bloquee con `reason` o con stderr en salida 2. Claude Code solo escribe una línea en el registro de depuración.

<h3 id="cwdchanged">
  CwdChanged
</h3>

Se ejecuta cuando un comando de shell en la conversación principal cambia el directorio de trabajo, por ejemplo cuando Claude ejecuta un comando `cd`. Úselo para reaccionar a cambios de directorio: recargar variables de entorno, activar cadenas de herramientas específicas del proyecto o ejecutar scripts de configuración automáticamente. Se empareja con [FileChanged](#filechanged) para herramientas como [direnv](https://direnv.net/) que administran el entorno por directorio.

Los hooks CwdChanged tienen acceso a [`CLAUDE_ENV_FILE`](#persist-environment-variables). Las variables escritas en ese archivo persisten en comandos Bash posteriores hasta el siguiente evento CwdChanged, cuando Claude Code las borra.

CwdChanged no admite matchers y se dispara en cada ocurrencia.

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

Además de los [campos de salida JSON](#json-output) disponibles para todos los hooks, los hooks CwdChanged pueden devolver `watchPaths` para establecer dinámicamente qué rutas de archivo [FileChanged](#filechanged) observa:

| Campo        | Descripción                                                                                                                                                                                                                                  |
| :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `watchPaths` | Matriz de rutas absolutas. Reemplaza la lista de observación dinámica actual. Las rutas de su configuración `matcher` siempre se observan. Devolver una matriz vacía borra la lista dinámica, que es típico al entrar en un nuevo directorio |

Los hooks CwdChanged no tienen control de decisión. No pueden bloquear el cambio de directorio.

Claude Code lee `watchPaths` y `systemMessage` de su salida JSON y descarta `continue`. En sesiones interactivas, muestra el `systemMessage` como una breve notificación de terminal. El mensaje no llega al flujo de mensajes de SDK.

<h3 id="directoryadded">
  DirectoryAdded
</h3>

Se ejecuta después de agregar un directorio de trabajo a mitad de sesión con el comando `/add-dir`, o después de que un cliente de SDK agregue uno con la solicitud de control `register_repo_root`. Úselo para preparar un repositorio recién agregado, por ejemplo instalando sus dependencias.

Claude Code no dispara este evento cuando:

* Pasa un directorio con la bandera de inicio `--add-dir`; [SessionStart](#sessionstart) cubre esos directorios
* Agrega un directorio en la pestaña Workspace `/permissions`
* Agrega un directorio que ya es un directorio de trabajo o está dentro de uno

Claude Code dispara DirectoryAdded después de actualizar el estado de sandbox y permiso, por lo que las herramientas en sandbox ya ven el nuevo directorio cuando se ejecuta su hook. Los comandos del hook en sí se ejecutan sin sandbox.

Claude Code no espera el hook: la adición se completa inmediatamente, y el hook se ejecuta en segundo plano con el tiempo de espera predeterminado de 600 segundos.

El matcher filtra en cómo se agregó el directorio:

| Matcher              | Cuándo se dispara                                                                       |
| :------------------- | :-------------------------------------------------------------------------------------- |
| `slash_command`      | Agrega un directorio con `/add-dir`                                                     |
| `register_repo_root` | Un cliente de SDK agrega un directorio con la solicitud de control `register_repo_root` |

<h4 id="directoryadded-input">
  Entrada de DirectoryAdded
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks DirectoryAdded reciben `directory` y `source`.

| Campo       | Descripción                                                                                                                  |
| :---------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `directory` | Ruta absoluta del directorio que se agregó                                                                                   |
| `source`    | Cómo se agregó el directorio, `"slash_command"` para `/add-dir` o `"register_repo_root"` para la solicitud de control de SDK |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "DirectoryAdded",
  "directory": "/Users/my-other-repo",
  "source": "slash_command"
}
```

Los hooks DirectoryAdded no tienen control de decisión. No pueden bloquear la adición, que ya se ha completado cuando se ejecuta el hook. Claude Code descarta el campo `continue` de su salida JSON y muestra el resto diferente por fuente:

* `slash_command`: Claude Code entrega el `systemMessage` del hook a Claude como contexto en el siguiente turno de conversación, en lugar de mostrárselo. Un recuento de hooks fallidos aparece en la transcripción. La salida de fallo completo va al registro de depuración
* `register_repo_root`: Claude Code escribe la salida `systemMessage` y la salida de fallo solo en el registro de depuración

<h3 id="filechanged">
  FileChanged
</h3>

Se ejecuta cuando un archivo observado cambia en el disco. Claude Code detecta cambios con un observador del sistema de archivos, no inspeccionando llamadas de herramientas, por lo que ejecuta el hook sin importar qué cambió el archivo: una llamada de herramienta `Edit` o `Write`, un script que Claude ejecuta con `Bash`, o un proceso fuera de Claude Code completamente. Un uso común es recargar variables de entorno cuando cambian archivos de configuración del proyecto.

El `matcher` para este evento sirve dos propósitos:

* **Construir la lista de observación**: el valor se divide en `|` y cada segmento se registra como un nombre de archivo literal en el directorio de trabajo, por lo que `".envrc|.env"` observa exactamente esos dos archivos. Los patrones regex no son útiles aquí: un valor como `^\.env` observaría un archivo literalmente nombrado `^\.env`.
* **Filtrar qué hooks se ejecutan**: cuando un archivo observado cambia, el mismo valor filtra qué grupos de hooks se ejecutan usando las [reglas de matcher](#matcher-patterns) estándar contra el nombre base del archivo cambiado.

Este ejemplo normaliza los finales de línea en `data.csv` después de cualquier cambio, incluido un comando `Bash` o un script externo reescribiendo el archivo:

```json theme={null}
{
  "hooks": {
    "FileChanged": [
      {
        "matcher": "data.csv",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/normalize-line-endings.sh"
          }
        ]
      }
    ]
  }
}
```

El hook lee la ruta absoluta del archivo cambiado del campo `file_path` de la [entrada JSON](#filechanged-input) en stdin. Su guardia `grep` prueba lo mismo que `perl` elimina, un CR al final de una línea, por lo que la ejecución después de una normalización sale sin tocar el archivo. Una guardia más suelta se repite para siempre, porque `perl -i` reescribe el archivo incluso cuando no sustituye nada y Claude Code ejecuta el hook nuevamente después de cada reescritura. Guarde este script en `/path/to/normalize-line-endings.sh` y hágalo ejecutable:

```bash theme={null}
#!/bin/bash
FILE=$(jq -r .file_path)
if grep -q $'\r$' "$FILE"; then
  perl -pi -e 's/\r$//' "$FILE"
fi
```

Para confirmar que el hook funciona, pida a Claude que agregue una línea CRLF a `data.csv` con un comando `Bash`. Claude Code ejecuta el hook y el archivo termina con finales LF.

Para observar archivos que no puede nombrar por adelantado, devuelva [`watchPaths`](#filechanged-output) de un hook para actualizar la lista de observación dinámicamente. Claude Code comienza el observador solo cuando algo nombra un archivo para observar, por lo que inicie la lista con un grupo FileChanged cuyo matcher nombre al menos un archivo, o con un hook [SessionStart](#sessionstart-decision-control) o [CwdChanged](#cwdchanged) que devuelva `watchPaths`. El matcher aún filtra qué grupos de hooks se ejecutan cuando un archivo observado cambia, así que dé al grupo que maneja rutas dinámicas un matcher omitido, que coincide con cada archivo observado y no agrega nada a la lista de observación. Un matcher `"*"` también coincide con cada archivo, pero Claude Code lo registra en la lista de observación como un archivo literal nombrado `*`.

Los hooks FileChanged tienen acceso a [`CLAUDE_ENV_FILE`](#persist-environment-variables). Las variables escritas en ese archivo persisten en comandos Bash posteriores hasta el siguiente evento [CwdChanged](#cwdchanged), cuando Claude Code las borra.

<h4 id="filechanged-input">
  Entrada de FileChanged
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks FileChanged reciben `file_path` y `event`.

| Campo       | Descripción                                                                                                                |
| :---------- | :------------------------------------------------------------------------------------------------------------------------- |
| `file_path` | Ruta absoluta al archivo que cambió                                                                                        |
| `event`     | Qué sucedió: `"change"` para un archivo modificado, `"add"` para un archivo creado, o `"unlink"` para un archivo eliminado |

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

Además de los [campos de salida JSON](#json-output) disponibles para todos los hooks, los hooks FileChanged pueden devolver `watchPaths` para actualizar dinámicamente qué rutas de archivo se observan:

| Campo        | Descripción                                                                                                                                                                                                                                            |
| :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `watchPaths` | Matriz de rutas absolutas. Reemplaza la lista de observación dinámica actual. Las rutas de su configuración `matcher` siempre se observan. Úselo cuando su script de hook descubra archivos adicionales para observar basándose en el archivo cambiado |

Los hooks FileChanged no tienen control de decisión. No pueden bloquear el cambio de archivo que ocurre.

Claude Code lee `watchPaths` y `systemMessage` de su salida JSON y descarta `continue`. En sesiones interactivas, muestra el `systemMessage` como una breve notificación de terminal. El mensaje no llega al flujo de mensajes de SDK.

<h3 id="worktreecreate">
  WorktreeCreate
</h3>

Se ejecuta cuando se está creando un worktree, ya sea desde `claude --worktree`, desde un [subagente usando `isolation: "worktree"`](/docs/es/sub-agents#choose-the-subagent-scope), o para una [sesión de fondo](/docs/es/agent-view#how-file-edits-are-isolated) que Claude Code aísla en su propio worktree. De forma predeterminada, Claude Code crea la copia de trabajo aislada con `git worktree`. Configurar un hook WorktreeCreate reemplaza ese comportamiento de git predeterminado, permitiéndole usar un sistema de control de versiones diferente como SVN, Perforce o Mercurial.

Debido a que el hook reemplaza el comportamiento predeterminado completamente, [`.worktreeinclude`](/docs/es/worktrees#copy-gitignored-files-into-worktrees) no se procesa. Si necesita copiar archivos de configuración local como `.env` en el nuevo worktree, hágalo dentro de su script de hook.

El hook debe devolver la ruta al directorio del worktree creado. Claude Code usa esta ruta como el directorio de trabajo para la sesión aislada. Consulte [Salida de WorktreeCreate](#worktreecreate-output) para saber cómo cada tipo de hook devuelve la ruta.

Claude Code actúa sobre el éxito del hook y la ruta devuelta, y descarta `systemMessage` y `continue`.

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

Además de los [campos de entrada comunes](#common-input-fields), los hooks WorktreeCreate reciben el campo `name`. Este es un identificador slug para el nuevo worktree, ya sea especificado por el usuario o generado automáticamente, por ejemplo `bold-oak-a3f2`.

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

Los hooks WorktreeCreate no usan el modelo de decisión de permitir/bloquear estándar. En su lugar, el éxito o fallo del hook determina el resultado. El hook debe devolver la ruta al directorio del worktree creado:

* **Hooks de comando** (`type: "command"`): imprima la ruta como la última línea no vacía de stdout. Claude Code elimina códigos de escape ANSI antes de leer esa línea, por lo que los banners de inicio de shell impresos antes de su `echo` se ignoran. Redirija cualquier otra salida de hook a stderr.
* **Hooks HTTP** (`type: "http"`): devuelva `{ "hookSpecificOutput": { "hookEventName": "WorktreeCreate", "worktreePath": "/absolute/path" } }` en el cuerpo de la respuesta.

Si el hook falla o no produce una ruta, la creación del worktree falla con un error.

Claude Code resuelve una ruta relativa contra el directorio en el que se ejecutó el hook, colapsando cualquier segmento `.` o `..` en él. Si la ruta resultante no es un directorio en el que Claude Code pueda entrar, la sesión imprime un error que nombra la ruta y sale con código 1.

Claude Code rechaza una ruta absoluta que contiene segmentos `.` o `..`, y cualquier ruta que pase a través de un symlink debajo de la raíz del repositorio, porque un symlink comprometido en el repositorio podría redirigir el worktree fuera de él. El error nombra el componente rechazado. Devuelva una ruta normalizada que no pase a través de un symlink dentro del repositorio. Antes de v2.1.216, la creación del worktree seguía la ruta del hook sin este cribado.

<h3 id="worktreeremove">
  WorktreeRemove
</h3>

Se ejecuta cuando se está eliminando un worktree. Este es el homólogo de limpieza de [WorktreeCreate](#worktreecreate). El evento se dispara cuando:

* sale de una sesión `--worktree` y elige eliminarla
* un subagente con `isolation: "worktree"` se completa
* elimina una [sesión de fondo](/docs/es/agent-view#what-deleting-a-session-removes) cuyo worktree creó el hook

Para worktrees basados en git, Claude Code maneja la limpieza automáticamente con `git worktree remove`. Si configuró un hook WorktreeCreate para un sistema de control de versiones que no es git, emparéjelo con un hook WorktreeRemove para manejar la limpieza. Sin uno, el directorio del worktree se deja en el disco.

Claude Code descarta los [campos de salida JSON](#json-output) de un hook WorktreeRemove, como `systemMessage` y `continue`.

Para una eliminación de sesión de fondo, Claude Code verifica la ruta del worktree almacenada antes de ejecutar el hook y rechaza una ruta que es un symlink o pasa a través de uno debajo de la raíz del repositorio. El hook se ejecuta para un worktree que aún contiene archivos solo cuando confirma la eliminación en [vista de agente](/docs/es/agent-view#what-deleting-a-session-removes); para tal worktree, [`claude rm`](/docs/es/agent-view#manage-sessions-from-the-shell) mantiene la sesión y el worktree en su lugar. Antes de v2.1.216, el hook se ejecutaba en la ruta almacenada sin estas verificaciones.

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

El código de salida de un hook WorktreeRemove decide el resultado. Cuando un hook sale con código distinto de cero y el directorio en `worktree_path` aún existe después, la eliminación falla:

* El worktree permanece en el disco, y el comando del hook y stderr van al [registro de depuración](#debug-hooks).
* Si estaba eliminando una sesión de fondo, la sesión también permanece. El mensaje de rechazo en [vista de agente](/docs/es/agent-view#what-deleting-a-session-removes) reporta cómo terminó el hook, como `exited 1`, cita el inicio de su stderr, y dice si eliminar la sesión nuevamente elimina el directorio de todas formas.

<h3 id="precompact">
  PreCompact
</h3>

Se ejecuta antes de que Claude Code esté a punto de ejecutar una operación de compactación.

El valor del matcher indica si la compactación fue disparada manualmente o automáticamente:

| Matcher  | Cuándo se dispara                                                                                                                            |
| :------- | :------------------------------------------------------------------------------------------------------------------------------------------- |
| `manual` | `/compact`                                                                                                                                   |
| `auto`   | Compactación automática cuando la conversación alcanza la [ventana de compactación automática](/docs/es/model-config#set-the-auto-compact-window) |

Salga con código 2 para bloquear la compactación. Para un `/compact` manual, el mensaje stderr se muestra al usuario. También puede bloquear devolviendo JSON con `"decision": "block"`.

Bloquear la compactación automática tiene diferentes efectos dependiendo de cuándo se dispare. Si la compactación fue disparada de forma proactiva antes del límite de contexto, Claude Code la omite y la conversación continúa sin compactar. Si la compactación fue disparada para recuperarse de un error de límite de contexto ya devuelto por la API, el error subyacente surge y la solicitud actual falla.

Claude Code descarta los campos `systemMessage` y `continue` de un hook PreCompact.

<h4 id="precompact-input">
  Entrada de PreCompact
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks PreCompact reciben `trigger` e `custom_instructions`. Para `manual`, `custom_instructions` contiene lo que el usuario pasa a `/compact` y es `null` cuando no pasa nada. Para `auto`, `custom_instructions` es `null`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PreCompact",
  "trigger": "manual",
  "custom_instructions": null
}
```

<h3 id="postcompact">
  PostCompact
</h3>

Se ejecuta después de que Claude Code completa una operación de compactación. Úselo para reaccionar al nuevo estado compactado, por ejemplo para registrar el resumen generado o actualizar el estado externo. Claude Code descarta los campos `systemMessage` y `continue` de un hook PostCompact.

Los mismos valores de matcher se aplican que para `PreCompact`:

| Matcher  | Cuándo se dispara                                                                                                                                       |
| :------- | :------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `manual` | Después de `/compact`                                                                                                                                   |
| `auto`   | Después de compactación automática cuando la conversación alcanza la [ventana de compactación automática](/docs/es/model-config#set-the-auto-compact-window) |

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

Los hooks PostCompact no tienen control de decisión. No pueden afectar el resultado de la compactación pero pueden realizar tareas de seguimiento.

<h3 id="premodelswitch">
  PreModelSwitch
</h3>

Se ejecuta antes de que Claude Code aplique un cambio de modelo que solicitó usted o un cliente. Úselo para bloquear un cambio, requerir confirmación o mostrar cuánto costará el cambio antes de que suceda.

PreModelSwitch requiere Claude Code v2.1.251 o posterior. Claude Code lo ejecuta para estas solicitudes:

* `/model <name>` y el selector `/model`
* El selector de modelo `Option+P` o `Alt+P`
* La configuración Model en `/config`
* Activar [modo rápido](/docs/es/fast-mode) cuando eso cambia el modelo de la sesión
* Una solicitud `set_model`, o un cambio de modelo en una solicitud `apply_flag_settings`, desde un anfitrión [Agent SDK](/docs/es/agent-sdk/typescript#query-object) o [Control Remoto](/docs/es/remote-control)

Claude Code no ejecuta hooks PreModelSwitch para cambios que realiza por su cuenta, como un [fallback de modelo automático](/docs/es/model-config#automatic-model-fallback) o restaurar el modelo cuando reanuda una sesión. Esos cambios llegan a [PostModelSwitch](#postmodelswitch) solo.

Claude Code compara el matcher contra el nombre canónico del modelo al que la sesión está cambiando, ignorando cualquier sufijo `[1m]`. Un alias como `opus`, un ID de modelo fechado y un ID específico del proveedor como un ID de modelo de Amazon Bedrock todos coinciden con el único nombre canónico al que se resuelven, por lo que `claude-opus-5` cubre cada deletreo de Opus 5.

Cuando Claude Code no puede determinar un nombre canónico para el objetivo, por ejemplo un ID de modelo personalizado que solo su [puerta de enlace LLM](/docs/es/llm-gateway) conoce, ejecuta cada hook PreModelSwitch independientemente del matcher. Un hook que bloquea debe verificar `to_model` de su entrada en lugar de confiar solo en el matcher.

Escriba el matcher como un nombre exacto, una lista separada por `|` como `claude-opus-4-6|claude-opus-5`, o una expresión regular como `.*opus.*`. Este ejemplo usa un matcher de nombre exacto y también verifica `to_model` de la entrada del hook, por lo que rechaza un cambio a Opus 4.6 saliendo con código 2 y permite cualquier otro objetivo:

<Tabs>
  <Tab title="macOS/Linux">
    El comando verifica `to_model` con `jq`:

    ```json theme={null}
    {
      "hooks": {
        "PreModelSwitch": [
          {
            "matcher": "claude-opus-4-6",
            "hooks": [
              {
                "type": "command",
                "command": "jq -e '.to_model | test(\"opus-4-6\")' > /dev/null && { echo 'Opus 4.6 is retired for this project. Use a newer model.' >&2; exit 2; }; exit 0"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Windows (PowerShell)">
    Registre un hook de comando que ejecute un script a través de PowerShell:

    ```json theme={null}
    {
      "hooks": {
        "PreModelSwitch": [
          {
            "matcher": "claude-opus-4-6",
            "hooks": [
              {
                "type": "command",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-opus-46.ps1"
                ]
              }
            ]
          }
        ]
      }
    }
    ```

    Guarde este script en `.claude/hooks/block-opus-46.ps1` en su proyecto:

    ```powershell theme={null}
    $hookInput = [Console]::In.ReadToEnd() | ConvertFrom-Json
    if ($hookInput.to_model -match 'opus-4-6') {
      [Console]::Error.WriteLine('Opus 4.6 is retired for this project. Use a newer model.')
      exit 2
    }
    exit 0
    ```
  </Tab>
</Tabs>

Para confirmar que el hook funciona, ejecute `/model claude-opus-4-6` desde una sesión que ejecuta un modelo diferente. Claude Code mantiene el modelo actual e informa que un hook PreModelSwitch bloqueó el cambio, con su mensaje como la razón.

<h4 id="premodelswitch-input">
  Entrada de PreModelSwitch
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks PreModelSwitch reciben los campos en esta tabla. Los últimos cinco describen qué cuesta reenviar la conversación al nuevo modelo, para que un hook pueda mostrar esa cifra antes de que suceda el cambio.

| Campo                       | Tipo            | Descripción                                                                                                                                                                                                                                                                                                          |
| :-------------------------- | :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `from_model`                | string          | ID de modelo del que cambia el cambio                                                                                                                                                                                                                                                                                |
| `to_model`                  | string          | ID de modelo al que cambia el cambio. El matcher compara contra el nombre canónico de este modelo                                                                                                                                                                                                                    |
| `requested_model`           | string o `null` | El modelo que la solicitud nombró: un alias como `opus`, un ID de modelo completo, o `null` cuando la solicitud fue para el modelo predeterminado                                                                                                                                                                    |
| `source`                    | string          | De dónde vino la solicitud: `"command"` para `/model <name>`, la configuración Model en `/config`, o activar modo rápido; `"picker"` para un selector de modelo; `"sdk"` para una solicitud `set_model`, o un cambio de modelo en una solicitud `apply_flag_settings`, desde un anfitrión Agent SDK o Control Remoto |
| `context_tokens`            | number          | Tokens que la siguiente solicitud reenvía como su prompt: los tokens de entrada, lectura de caché, creación de caché y salida de la última respuesta en la conversación principal, combinados. `0` antes de la primera respuesta                                                                                     |
| `prompt_cache_warm`         | boolean         | Si el caché de prompt del modelo actual probablemente aún esté caliente, lo que significa que el cambio lo pierde                                                                                                                                                                                                    |
| `cache_ttl`                 | string          | [Duración de vida del caché de prompt](/docs/es/prompt-caching#cache-lifetime) que Claude Code solicita para esta sesión: `"5m"` o `"1h"`                                                                                                                                                                                 |
| `estimated_cache_write_usd` | number          | Costo estimado en dólares estadounidenses de escribir `context_tokens` en el caché de prompt en `to_model` a la tasa `cache_ttl`, excluyendo la siguiente respuesta. El servidor puede no necesitar re-cachear todo el contexto, así que trátelo como una estimación                                                 |
| `pricing`                   | string          | Cómo Claude Code fijó el precio de `estimated_cache_write_usd`: `"configured"` a sus propias tasas de la organización cuando las ha configurado, `"catalog"` a precio de lista, o `"default"` cuando `to_model` no tiene precio conocido y Claude Code asumió una tasa predeterminada                                |

Este ejemplo muestra la entrada para `/model opus` en una sesión que ejecuta Sonnet 5:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PreModelSwitch",
  "from_model": "claude-sonnet-5",
  "to_model": "claude-opus-5",
  "requested_model": "opus",
  "source": "command",
  "context_tokens": 182340,
  "prompt_cache_warm": true,
  "cache_ttl": "5m",
  "estimated_cache_write_usd": 1.1396,
  "pricing": "catalog"
}
```

<h4 id="premodelswitch-decision-control">
  Control de decisión de PreModelSwitch
</h4>

Los hooks `PreModelSwitch` pueden cancelar el cambio, pedir al usuario que lo confirme, o permitir que continúe. El código de salida 2 o un `decision: "block"` de nivel superior cancela el cambio.

Para un control más fino, devuelva `permissionDecision` y `permissionDecisionReason` en un objeto `hookSpecificOutput`, como en [PreToolUse](#pretooluse-decision-control). `PreModelSwitch` acepta `"allow"`, `"deny"`, y `"ask"`. No acepta `"defer"`, `updatedInput`, o `additionalContext`. La tabla a continuación describe ambos campos:

| Campo                      | Descripción                                                                                                                                                                                                                    |
| :------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissionDecision`       | `"allow"` continúa y omite la [confirmación que Claude Code muestra mientras el caché de prompt está caliente](/docs/es/prompt-caching#switching-models). `"deny"` cancela el cambio. `"ask"` solicita al usuario que lo confirme   |
| `permissionDecisionReason` | Para `"deny"`, se muestra al usuario como la razón por la que se bloqueó el cambio, o se devuelve como el error para una solicitud `set_model`. Para `"ask"`, se muestra en el aviso de confirmación. Se ignora para `"allow"` |

Solo `/model` en una sesión interactiva puede mostrar el aviso `"ask"`. En todas las otras superficies, incluido modo no interactivo con la bandera `-p`, `/config`, y solicitudes `set_model`, Claude Code trata `"ask"` como un rechazo.

Este ejemplo pide al usuario que confirme y cita el recuento de tokens de `context_tokens`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreModelSwitch",
    "permissionDecision": "ask",
    "permissionDecisionReason": "Switching now re-sends about 180k tokens to the new model. Continue?"
  }
}
```

Cuando múltiples hooks PreModelSwitch devuelven decisiones diferentes, la precedencia es `deny` > `ask` > `allow`.

Claude Code muestra al usuario cualquier `systemMessage` que devuelva su hook independientemente de la decisión, por lo que un hook de informe de costos puede devolver `{"systemMessage": "..."}` y salir con 0.

Un hook PreModelSwitch que no responde antes de su tiempo de espera bloquea el cambio. En [PreToolUse](#timeouts), por el contrario, un hook de comando que agota el tiempo de espera permite que la llamada de herramienta continúe. El tiempo de espera predeterminado para este evento es 30 segundos. `PreModelSwitch` ejecuta solo hooks `command`, `http`, y `mcp_tool`, por lo que los valores predeterminados `prompt` y `agent` no se aplican.

Un hook que sale con un código distinto de 0 o 2 y no imprime ninguna decisión JSON no bloquea: Claude Code muestra su stderr y aplica el cambio, como se describe en [Otros códigos de salida](#other-exit-codes).

<h3 id="postmodelswitch">
  PostModelSwitch
</h3>

Se ejecuta después de que cambia el modelo de la sesión. Úselo para dar orientación específica del modelo a Claude sin editar cada CLAUDE.md, por ejemplo una instrucción de toda la organización que se aplica en ciertos modelos.

PostModelSwitch requiere Claude Code v2.1.251 o posterior. No puede bloquear, porque el modelo ya ha cambiado. Claude Code ejecuta hooks PostModelSwitch después de cualquiera de estos cambios:

* Un cambio que solicitó usted o un cliente
* Un [fallback de modelo automático](/docs/es/model-config#automatic-model-fallback), que cambia el modelo de la sesión
* Una configuración como [`opusplan`](/docs/es/model-config#opusplan-model-setting) entrando o saliendo del modo de plan
* Claude Code restaurando el modelo cuando reanuda una sesión

Claude Code no ejecuta hooks PostModelSwitch cuando un modelo de una [cadena de modelo de fallback](/docs/es/model-config#fallback-model-chains) sirve un turno, porque esa sustitución dura un turno y deja el modelo de la sesión sin cambios.

El matcher sigue las mismas reglas que [PreModelSwitch](#premodelswitch): Claude Code compara contra el nombre canónico del modelo al que la sesión cambió.

Este ejemplo agrega orientación siempre que el modelo de la sesión cambia a cualquier modelo Opus:

```json theme={null}
{
  "hooks": {
    "PostModelSwitch": [
      {
        "matcher": ".*opus.*",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'On Opus, delegate implementation work to subagents and keep this conversation for planning and review.'"
          }
        ]
      }
    ]
  }
}
```

Para confirmar que el hook funciona, cambie a un modelo Opus desde una sesión que ejecuta un modelo diferente, por ejemplo ejecute `/model opus` desde una sesión Sonnet, luego pregunte a Claude qué orientación tiene sobre el modelo actual.

<h4 id="postmodelswitch-input">
  Entrada de PostModelSwitch
</h4>

Los hooks PostModelSwitch reciben los mismos campos que [PreModelSwitch](#premodelswitch-input), con `hook_event_name` establecido en `"PostModelSwitch"` y dos valores `source` más: `"auto"` para un fallback automático u otro cambio que Claude Code realizó por su cuenta, y `"resume"` para el modelo restaurado cuando reanuda una sesión.

`requested_model` es `null` cuando `source` es `"auto"`. Cuando `source` es `"resume"`, es la configuración de modelo guardada que Claude Code restauró.

<h4 id="postmodelswitch-decision-control">
  Control de decisión de PostModelSwitch
</h4>

Claude Code toma su stdout de texto plano del hook [](#exit-code-0) en salida 0, o `additionalContext` de salida JSON, y lo entrega a Claude con la siguiente solicitud después del cambio. Además de los [campos de salida JSON](#json-output) disponibles para todos los hooks, puede devolver:

| Campo               | Descripción                                                                                                                        |
| :------------------ | :--------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | Cadena agregada al contexto de Claude con la siguiente solicitud. Consulte [Agregar contexto para Claude](#add-context-for-claude) |

Si el hook no se ha completado dentro de cinco segundos después de que envía la siguiente solicitud, Claude Code envía esa solicitud sin la salida y la adjunta a la siguiente solicitud en su lugar. Si el modelo cambia varias veces antes de la siguiente solicitud, Claude Code entrega solo la salida para el modelo de destino del último cambio.

<h3 id="sessionend">
  SessionEnd
</h3>

Se ejecuta cuando termina una sesión de Claude Code. Útil para tareas de limpieza, registro de estadísticas de sesión o guardado del estado de sesión. Admite matchers para filtrar por razón de salida.

El campo `reason` en la entrada del hook indica por qué terminó la sesión:

| Razón                         | Descripción                                                                          |
| :---------------------------- | :----------------------------------------------------------------------------------- |
| `clear`                       | Sesión borrada con comando `/clear`                                                  |
| `resume`                      | Sesión cambiada a través de `/resume` interactivo                                    |
| `logout`                      | Usuario cerró sesión                                                                 |
| `prompt_input_exit`           | Usuario salió mientras la entrada de prompt era visible                              |
| `other`                       | Otras razones de salida                                                              |
| `bypass_permissions_disabled` | Eliminado en v2.1.234; Claude Code no lo envía. Elimine de sus matchers `SessionEnd` |

<h4 id="sessionend-input">
  Entrada de SessionEnd
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks SessionEnd reciben un campo `reason` que indica por qué terminó la sesión. Consulte la [tabla de razones](#sessionend) anterior para todos los valores.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionEnd",
  "reason": "other"
}
```

Los hooks SessionEnd no tienen control de decisión. No pueden bloquear la terminación de la sesión pero pueden realizar tareas de limpieza. Claude Code descarta sus [campos de salida JSON](#json-output), como `systemMessage`.

Los hooks SessionEnd tienen un tiempo de espera predeterminado de 1.5 segundos. Se aplica cuando sale, ejecuta `/clear`, o cambia sesiones con `/resume` interactivo. Puede dar a un hook más tiempo de dos formas:

* **`timeout` por hook**: establezca `timeout` en la configuración de ese hook. El presupuesto general se eleva automáticamente para coincidir con el `timeout` por hook más alto en sus archivos de configuración, hasta 60 segundos. Si eleva el presupuesto de esta manera, un hook sin su propio `timeout` aún mantiene el predeterminado. Los tiempos de espera establecidos en hooks proporcionados por plugins no elevan el presupuesto.
* **`CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS`**: establezca esta variable de entorno en milisegundos para anular el presupuesto explícitamente. El valor que establezca también se convierte en el tiempo de espera para cada hook sin su propio `timeout`.

Este ejemplo establece el presupuesto en 5 segundos:

```bash theme={null}
CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS=5000 claude
```

Antes de v2.1.268, `CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS` solo elevaba el presupuesto general, y un hook sin su propio `timeout` aún se cancelaba después de 1.5 segundos.

<h3 id="elicitation">
  Elicitation
</h3>

Se ejecuta cuando un servidor MCP solicita entrada del usuario a mitad de tarea. De forma predeterminada, Claude Code muestra un diálogo interactivo para que el usuario responda. Los hooks pueden interceptar esta solicitud y responder programáticamente, omitiendo completamente el diálogo.

El campo matcher coincide contra el nombre del servidor MCP.

<h4 id="elicitation-input">
  Entrada de Elicitation
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks Elicitation reciben `mcp_server_name`, `message`, y campos opcionales `mode`, `url`, `elicitation_id`, y `requested_schema`.

Para elicitación de modo de formulario, el caso más común:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
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

Para elicitación de modo URL, utilizada para autenticación basada en navegador:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
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

El código de salida 2 deniega la elicitación. Claude Code no muestra su mensaje stderr en ningún lugar.

Claude Code actúa sobre `hookSpecificOutput` de la salida JSON de un hook Elicitation y descarta `systemMessage` y `continue`.

<h3 id="elicitationresult">
  ElicitationResult
</h3>

Se ejecuta después de que un usuario responde a una elicitación de MCP. Los hooks pueden observar, modificar o bloquear la respuesta antes de que se devuelva al servidor MCP.

El campo matcher coincide contra el nombre del servidor MCP.

<h4 id="elicitationresult-input">
  Entrada de ElicitationResult
</h4>

Además de los [campos de entrada comunes](#common-input-fields), los hooks ElicitationResult reciben `mcp_server_name`, `action`, y campos opcionales `mode`, `elicitation_id`, y `content`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
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

| Campo     | Valores                       | Descripción                                                                               |
| :-------- | :---------------------------- | :---------------------------------------------------------------------------------------- |
| `action`  | `accept`, `decline`, `cancel` | Anula la acción del usuario                                                               |
| `content` | object                        | Anula los valores del campo de formulario. Solo significativo cuando `action` es `accept` |

El código de salida 2 bloquea la respuesta, cambiando la acción efectiva a `decline`. Claude Code no muestra su mensaje stderr en ningún lugar.

Claude Code actúa sobre `hookSpecificOutput` de la salida JSON de un hook ElicitationResult y descarta `systemMessage` y `continue`.

<h2 id="prompt-based-hooks">
  Hooks basados en prompts
</h2>

Además de hooks de comando, HTTP y herramientas MCP, Claude Code admite hooks basados en prompts (`type: "prompt"`) que usan un LLM para evaluar si permitir o bloquear una acción, y hooks de agente (`type: "agent"`) que generan un verificador agentico con acceso a herramientas. No todos los eventos admiten todos los tipos de hooks.

Eventos que admiten los cinco tipos de hooks (`command`, `http`, `mcp_tool`, `prompt` y `agent`):

* `PermissionDenied`
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

`PermissionRequest` admite hooks `command`, `http`, `mcp_tool` y `prompt` pero no hooks `agent`. Si configura un hook de agente en este evento, Claude Code lo omite y el flujo de permisos continúa sin cambios. Para permitir o denegar desde un hook, devuelva el [objeto de decisión](#permissionrequest-decision-control) desde un hook de comando o HTTP.

Eventos que admiten hooks `command`, `http` y `mcp_tool` pero no `prompt` o `agent`:

* `ConfigChange`
* `CwdChanged`
* `DirectoryAdded`
* `Elicitation`
* `ElicitationResult`
* `FileChanged`
* `InstructionsLoaded`
* `MessageDisplay`
* `Notification`
* `PostCompact`
* `PostModelSwitch`
* `PreCompact`
* `PreModelSwitch`
* `SessionEnd`
* `StopFailure`
* `SubagentStart`
* `WorktreeCreate`
* `WorktreeRemove`

`SessionStart` y `Setup` admiten hooks `command` y `mcp_tool`, y [MCP tool hook fields](#mcp-tool-hook-fields) describe cuándo se ejecutan sus hooks `mcp_tool`. No admiten hooks `http`, `prompt` o `agent`.

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

Establezca `type` en `"prompt"` y proporcione una cadena `prompt` en lugar de un `command`. Use el marcador de posición `$ARGUMENTS` para inyectar datos de entrada JSON del hook en su texto de prompt.

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

| Campo             | Requerido | Descripción                                                                                                                                                                                                                                    |
| :---------------- | :-------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`            | sí        | Debe ser `"prompt"`                                                                                                                                                                                                                            |
| `prompt`          | sí        | El texto del prompt a enviar al LLM. Use `$ARGUMENTS` como marcador de posición para la entrada JSON del hook. Si `$ARGUMENTS` no está presente, la entrada JSON se agrega al prompt                                                           |
| `model`           | no        | Modelo a usar para evaluación. Por defecto es un modelo rápido                                                                                                                                                                                 |
| `timeout`         | no        | Tiempo de espera en segundos. Predeterminado: 30                                                                                                                                                                                               |
| `continueOnBlock` | no        | En los eventos a los que se aplica, `true` retroalimenta una razón `ok: false` a Claude y continúa en lugar de terminar el turno. Predeterminado: `false`. Consulte [Esquema de respuesta](#response-schema) para el comportamiento por evento |

<h3 id="response-schema">
  Esquema de respuesta
</h3>

El LLM debe responder con JSON que contenga:

```json theme={null}
{
  "ok": true | false,
  "reason": "Explanation for the decision",
  "impossible": true | false
}
```

| Campo        | Descripción                                                                                                                                                                                                                                                      |
| :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ok`         | `true` para permitir. Para `false`, consulte el comportamiento por evento a continuación                                                                                                                                                                         |
| `reason`     | Requerido cuando `ok` es `false`                                                                                                                                                                                                                                 |
| `impossible` | Opcional. El modelo lo devuelve con `ok: false` cuando juzga que la condición nunca puede satisfacerse. En `Stop` y `SubagentStop`, Claude Code permite que el turno termine en lugar de retroalimentar la razón. Los hooks de agente y otros eventos lo ignoran |

Lo que sucede en `ok: false` depende del evento:

* `Stop` y `SubagentStop`: la razón se retroalimenta a Claude como su siguiente instrucción y el turno continúa, a menos que la respuesta también establezca `impossible: true`, en cuyo caso Claude Code permite la detención y el turno termina
* `PreToolUse`: la llamada de herramienta se deniega; por defecto el turno termina y la razón de denegación aparece en el chat como una línea de advertencia. Establezca `continueOnBlock: true` para devolver la razón a Claude como el error de la herramienta para que pueda ajustarse y continuar, equivalente a un hook de comando con `permissionDecision: "deny"`. Antes de v2.1.210, la razón de denegación se devolvía a Claude como el error de la herramienta y el turno continuaba
* `PostToolUse`: por defecto el turno termina y la razón aparece en el chat como una línea de advertencia. Establezca `continueOnBlock: true` para retroalimentar la razón a Claude y continuar el turno en lugar de detener
* `PostToolBatch`, `UserPromptSubmit` y `UserPromptExpansion`: el turno termina y la razón aparece como una línea de advertencia. Estos eventos terminan el turno en `decision: "block"` independientemente de `continue`
* `PostToolUseFailure` y `TaskCreated`: la razón se devuelve a Claude como un error de herramienta y el turno continúa, independientemente de `continueOnBlock`
* `TaskCompleted`: cuando se activa porque una tarea se marca como completada durante un turno, la razón se devuelve a Claude como un error de herramienta y el turno continúa, independientemente de `continueOnBlock`. Cuando se activa porque un compañero se detiene, se comporta como `TeammateIdle` y detiene al compañero por defecto
* `TeammateIdle`: por defecto el compañero se detiene y la razón aparece como una línea de advertencia. Establezca `continueOnBlock: true` para retroalimentar la razón al compañero y mantenerlo trabajando en su lugar
* `PermissionRequest`: `ok: false` no tiene efecto. Para denegar una aprobación desde un hook, use un [hook de comando](#command-hook-fields) que devuelva `hookSpecificOutput.decision.behavior: "deny"`
* `PermissionDenied`: `ok: false` no tiene efecto porque la denegación ya sucedió. La única salida que este evento lee es `hookSpecificOutput.retry`, que los hooks de prompt y agente no pueden establecer. Se ejecutan en este evento, pero su salida se descarta. Use un [hook de comando](#command-hook-fields) para devolver `retry`

Si necesita un control más fino en cualquier evento, use un [hook de comando](#command-hook-fields) con los campos por evento descritos en [Control de decisión](#decision-control).

<h3 id="check-multiple-conditions-before-stopping">
  Verificar múltiples condiciones antes de detener
</h3>

Este hook `Stop` usa un prompt detallado para verificar tres condiciones antes de permitir que Claude se detenga. Los hooks `SubagentStop` usan el mismo formato para evaluar si un [subagente](/docs/es/sub-agents) debe detenerse. Si el modelo devuelve `"ok": false` porque la condición aún no se cumple, Claude continúa trabajando con la razón proporcionada como su siguiente instrucción:

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

Los hooks basados en agentes (`type: "agent"`) son como hooks basados en prompts pero con acceso a herramientas de múltiples turnos. En lugar de una única llamada LLM, un hook de agente genera un subagente que puede leer archivos, buscar código e inspeccionar la base de código para verificar condiciones. Los hooks de agente admiten los mismos eventos que los [hooks basados en prompts](#prompt-based-hooks), excepto `PermissionRequest`.

<h3 id="how-agent-hooks-work">
  Cómo funcionan los hooks de agente
</h3>

Cuando se activa un hook de agente:

1. Claude Code genera un subagente con su prompt y la entrada JSON del hook
2. El subagente puede usar herramientas como Read, Grep y Glob para investigar
3. Después de hasta 50 turnos, el subagente devuelve una decisión estructurada `{ "ok": true/false }`
4. Claude Code permite la acción si `ok` es `true`. Si `ok` es `false`, Claude Code maneja el bloqueo de la misma manera que un hook de prompt con `continueOnBlock: true` en ese evento, como se indica en [Esquema de respuesta](#response-schema)

Los hooks de agente son útiles cuando la verificación requiere inspeccionar archivos reales o salida de prueba, no solo evaluar los datos de entrada del hook solos.

<h3 id="agent-hook-configuration">
  Configuración de hook de agente
</h3>

Establezca `type` en `"agent"` y proporcione una cadena `prompt`, usando `$ARGUMENTS` como marcador de posición para la entrada JSON del hook. Los campos de configuración son los mismos que los [hooks de prompt](#prompt-hook-configuration), excepto que los hooks de agente tienen un tiempo de espera predeterminado más largo de 60 segundos y no tienen campo `continueOnBlock`.

El esquema de respuesta es `{ "ok": true }` para permitir o `{ "ok": false, "reason": "..." }` para bloquear. En `ok: false`, Claude Code maneja un hook de agente de la manera que maneja un [hook de prompt con `continueOnBlock: true`](#response-schema) en el mismo evento; los hooks de agente no tienen campo `continueOnBlock` y no admiten el campo `impossible` del hook de prompt.

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

Este hook ejecuta un script de prueba después de cada llamada a herramienta `Write`. Claude continúa trabajando inmediatamente mientras `run-tests.sh` se ejecuta. Cuando el script finaliza, su salida se entrega en el siguiente turno de conversación:

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
            "async": true
          }
        ]
      }
    ]
  }
}
```

Una vez que un hook asincrónico se está ejecutando en segundo plano, Claude Code no aplica `timeout` en él. Claude Code aún aplica `timeout` en un hook que ejecuta con `asyncRewake`.

Claude Code entrega los resultados de un hook asincrónico solo mientras se ejecuta la sesión:

* En [modo no interactivo](/docs/es/headless) con la bandera `-p`, Claude Code mata cualquier hook asincrónico que aún se esté ejecutando al finalizar y lo finaliza con resultado `cancelled`
* Si el trabajo de su hook debe sobrevivir a una sesión `claude -p`, inicie un proceso completamente desacoplado desde él

<h3 id="how-async-hooks-execute">
  Cómo se ejecutan los hooks asincronos
</h3>

Cuando se activa un hook asincrónico, Claude Code inicia el proceso del hook e inmediatamente continúa sin esperar a que finalice. El hook recibe la misma entrada JSON a través de stdin que un hook sincrónico.

Después de que el proceso de fondo sale, Claude Code entrega los campos `additionalContext` y `systemMessage` de la respuesta JSON del hook a Claude en el siguiente turno de conversación. A diferencia del `systemMessage` de un hook sincrónico, ninguno de estos campos se le muestra a usted.

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
            "async": true
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

Los hooks asincronos tienen restricciones adicionales en comparación con los hooks sincronos:

* La salida del hook se entrega en el siguiente turno de conversación. Si la sesión está inactiva, la respuesta espera hasta la siguiente interacción del usuario. Excepción: un hook `asyncRewake` que sale con código 2 despierta a Claude inmediatamente incluso cuando la sesión está inactiva.
* Cada ejecución crea un proceso de fondo separado. No hay deduplicación en múltiples activaciones del mismo hook asincrónico.

<h2 id="security-considerations">
  Consideraciones de seguridad
</h2>

<h3 id="disclaimer">
  Descargo de responsabilidad
</h3>

<Warning>
  Los hooks de comando ejecutan comandos de shell con sus permisos de usuario completos. Pueden modificar, eliminar o acceder a cualquier archivo al que su cuenta de usuario pueda acceder. Revise y pruebe todos los comandos de hook antes de agregarlos a su configuración.
</Warning>

<h3 id="workspace-trust">
  Confianza del espacio de trabajo
</h3>

Claude Code verifica la confianza del espacio de trabajo antes de ejecutar cualquier hook desde un archivo de configuración. Lo que cuenta como confiable depende del tipo de sesión:

* **Sesión interactiva**: Claude Code retiene los hooks de todos los archivos de configuración, incluido su propio `~/.claude/settings.json`, hasta que acepte el [diálogo de confianza del espacio de trabajo](/docs/es/permissions#project-allow-rules-and-workspace-trust) para la carpeta, o para un directorio principal cuya confianza se extienda a ella
* **Sesión `-p` o SDK**: Claude Code nunca muestra el diálogo y trata la carpeta como confiable, por lo que los hooks confirmados en el `.claude/settings.json` de un repositorio se ejecutan en una carpeta que nunca ha confiado

Antes de ejecutar `claude -p` en un repositorio que no escribió, revise sus archivos de configuración `.claude/`, comience con [`--bare`](/docs/es/headless#start-faster-with-bare-mode), o [desactive los hooks para esa ejecución](#disable-or-remove-hooks) con `--settings '{"disableAllHooks": true}'`. Los hooks de frontmatter en un subagente de proyecto siguen una regla más estricta que los hooks de archivo de configuración. [Lo que se ejecuta antes de confiar en una carpeta](/docs/es/permissions#what-runs-before-you-trust-a-folder) enumera cada tipo de contenido de repositorio por tipo de sesión.

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

En Windows, puede ejecutar hooks individuales en PowerShell estableciendo `"shell": "powershell"` en un hook de comando. Claude Code detecta automáticamente `pwsh.exe`, el ejecutable de PowerShell 7 y posterior, y recurre a `powershell.exe` para Windows PowerShell 5.1.

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

Los detalles de ejecución de hooks se escriben en el archivo de registro de depuración. Inicie Claude Code con `claude --debug-file <path>` para escribir el registro en una ubicación conocida, o ejecute `claude --debug` y lea el registro en `~/.claude/debug/<session-id>.txt`. La bandera `--debug` no imprime en la terminal.

Por ejemplo, un hook `PostToolUse` en `Write` cuyo comando imprime `hook-ran` produce entradas como:

```text theme={null}
2026-07-19T02:03:24.382Z [DEBUG] Hook output does not start with {, treating as plain text
2026-07-19T02:03:24.382Z [DEBUG] "Hook PostToolUse:Write (PostToolUse) success:\nhook-ran"
```

Para detalles de coincidencia de hooks más granulares, establezca `CLAUDE_CODE_DEBUG_LOG_LEVEL=verbose` para ver líneas de registro adicionales como recuentos de matchers de hooks y coincidencia de consultas.

Para solucionar problemas comunes como hooks que no se activan, hooks Stop que siguen bloqueando, o errores de configuración, consulte [Limitaciones y solución de problemas](/docs/es/hooks-guide#limitations-and-troubleshooting) en la guía. Para un recorrido de diagnóstico más amplio que cubra `/context`, `/doctor` y precedencia de configuración, consulte [Depure su configuración](/docs/es/debug-your-config).
