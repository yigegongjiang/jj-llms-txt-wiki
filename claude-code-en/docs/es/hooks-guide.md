> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Automatizar acciones con hooks

> Ejecuta comandos de shell automáticamente cuando Claude Code edita archivos, finaliza tareas o necesita entrada. Formatea código, envía notificaciones, valida comandos y aplica reglas del proyecto.

Los hooks son comandos de shell definidos por el usuario que se ejecutan en puntos específicos del ciclo de vida de Claude Code. Proporcionan control determinista sobre el comportamiento de Claude Code, asegurando que ciertas acciones siempre ocurran en lugar de depender de que el LLM elija ejecutarlas. Usa hooks para aplicar reglas del proyecto, automatizar tareas repetitivas e integrar Claude Code con tus herramientas existentes.

Para decisiones que requieren criterio en lugar de reglas deterministas, también puedes usar [hooks basados en prompts](#prompt-based-hooks) o [hooks basados en agentes](#agent-based-hooks) que utilizan un modelo Claude para evaluar condiciones.

Para otras formas de extender Claude Code, consulta [skills](/docs/es/skills) para dar a Claude instrucciones adicionales y comandos ejecutables, [subagents](/docs/es/sub-agents) para ejecutar tareas en contextos aislados, y [plugins](/docs/es/plugins) para empaquetar extensiones para compartir entre proyectos.

<Tip>
  Esta guía cubre casos de uso comunes y cómo comenzar. Para esquemas de eventos completos, formatos de entrada/salida JSON y características avanzadas como hooks asincronos y hooks de herramientas MCP, consulta la [referencia de Hooks](/docs/es/hooks).
</Tip>

<h2 id="set-up-your-first-hook">
  Configura tu primer hook
</h2>

Para crear un hook, añade un bloque `hooks` a un [archivo de configuración](#configure-hook-location). Este tutorial crea un hook de notificación de escritorio, para que recibas una alerta cada vez que Claude esté esperando tu entrada en lugar de ver la terminal.

<Steps>
  <Step title="Añade el hook a tu configuración">
    Abre `~/.claude/settings.json` y añade un hook `Notification`. El ejemplo a continuación usa `osascript` para macOS; consulta [Recibe notificaciones cuando Claude necesita entrada](#get-notified-when-claude-needs-input) para comandos de Linux y Windows.

    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'"
              }
            ]
          }
        ]
      }
    }
    ```

    Si tu archivo de configuración ya tiene una clave `hooks`, añade `Notification` como hermano de las claves de evento existentes en lugar de reemplazar el objeto completo. Cada nombre de evento es una clave dentro del único objeto `hooks`:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write" }]
          }
        ],
        "Notification": [
          {
            "matcher": "",
            "hooks": [{ "type": "command", "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'" }]
          }
        ]
      }
    }
    ```

    También puedes pedirle a Claude que escriba el hook por ti describiendo lo que quieres en la CLI.
  </Step>

  <Step title="Verifica la configuración">
    Escribe `/hooks` para abrir el navegador de hooks. Verás una lista de todos los eventos de hook disponibles, con un contador junto a cada evento que tiene hooks configurados. Selecciona `Notification` para confirmar que tu nuevo hook aparece en la lista. Seleccionar el hook muestra sus detalles: el evento, matcher, tipo, archivo de origen y comando.
  </Step>

  <Step title="Prueba el hook">
    Presiona `Esc` para volver a la CLI. Pídele a Claude que haga algo que requiera permiso, luego cambia de la terminal. Deberías recibir una notificación de escritorio.
  </Step>
</Steps>

<Tip>
  El menú `/hooks` es de solo lectura. Para añadir, modificar o eliminar hooks, edita tu JSON de configuración directamente o pídele a Claude que haga el cambio.
</Tip>

<h2 id="what-you-can-automate">
  Qué puedes automatizar
</h2>

Los hooks te permiten ejecutar código en puntos clave del ciclo de vida de Claude Code: formatear archivos después de ediciones, bloquear comandos antes de que se ejecuten, enviar notificaciones cuando Claude necesita entrada, inyectar contexto al inicio de la sesión, y más. Para la lista completa de eventos de hook, consulta la [referencia de Hooks](/docs/es/hooks#hook-lifecycle).

Cada ejemplo incluye un bloque de configuración listo para usar que añades a un [archivo de configuración](#configure-hook-location).

Para un ejemplo de producción de hooks que ejecutan una revisión de modelo separada y alimentan los hallazgos de vuelta a la sesión, consulta [cómo el plugin `security-guidance` se integra con Claude Code](/docs/es/security-guidance#how-the-plugin-integrates-with-claude-code).

<h3 id="get-notified-when-claude-needs-input">
  Recibe notificaciones cuando Claude necesita entrada
</h3>

Obtén una notificación de escritorio cada vez que Claude termine de trabajar y necesite tu entrada, para que puedas cambiar a otras tareas sin verificar la terminal.

Este hook usa el evento `Notification`, que se activa cuando Claude está esperando entrada o permiso. Cada pestaña a continuación usa el comando de notificación nativo de la plataforma. Añade esto a `~/.claude/settings.json`:

<Tabs>
  <Tab title="macOS">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'"
              }
            ]
          }
        ]
      }
    }
    ```

    <Accordion title="Si no aparece ninguna notificación">
      `osascript` enruta notificaciones a través de la aplicación Script Editor integrada. Si Script Editor no tiene permiso de notificación, el comando falla silenciosamente, y macOS no te pedirá que lo otorgues. Ejecuta esto en Terminal una vez para que Script Editor aparezca en tu configuración de notificaciones:

      ```bash theme={null}
      osascript -e 'display notification "test"'
      ```

      Nada aparecerá aún. Abre **Configuración del Sistema > Notificaciones**, encuentra **Script Editor** en la lista, y activa **Permitir notificaciones**. Ejecuta el comando de nuevo para confirmar que aparece la notificación de prueba.
    </Accordion>
  </Tab>

  <Tab title="Linux">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "notify-send 'Claude Code' 'Claude Code needs your attention'"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Windows (PowerShell)">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "powershell.exe -Command \"[System.Reflection.Assembly]::LoadWithPartialName('System.Windows.Forms'); [System.Windows.Forms.MessageBox]::Show('Claude Code needs your attention', 'Claude Code')\""
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>
</Tabs>

El matcher vacío se activa en todos los tipos de notificación. Para activarse solo en eventos específicos, establécelo en uno de estos valores:

| Matcher                | Se activa cuando                                                                                                                     |
| :--------------------- | :----------------------------------------------------------------------------------------------------------------------------------- |
| `permission_prompt`    | Claude necesita que apruebes un uso de herramienta                                                                                   |
| `idle_prompt`          | Claude ha terminado y está esperando tu siguiente solicitud                                                                          |
| `auth_success`         | La autenticación se completa                                                                                                         |
| `elicitation_dialog`   | Un servidor MCP abre un formulario de elicitación                                                                                    |
| `elicitation_complete` | Un formulario de elicitación de MCP se envía o se descarta                                                                           |
| `elicitation_response` | Una respuesta de elicitación de MCP se envía de vuelta al servidor                                                                   |
| `agent_needs_input`    | Una sesión en segundo plano comienza a esperar tu entrada. Se activa solo mientras la [vista de agente](/docs/es/agent-view) está abierta |
| `agent_completed`      | Una sesión en segundo plano se completa o falla. Se activa solo mientras la [vista de agente](/docs/es/agent-view) está abierta           |

Los matchers `agent_needs_input` y `agent_completed` requieren Claude Code v2.1.198 o posterior.

Escribe `/hooks` y selecciona `Notification` para confirmar que el hook está registrado. Para el esquema de evento completo, consulta la [referencia de Notification](/docs/es/hooks#notification).

<h3 id="auto-format-code-after-edits">
  Formatea automáticamente el código después de ediciones
</h3>

Ejecuta automáticamente [Prettier](https://prettier.io/) en cada archivo que Claude edita, para que el formato se mantenga consistente sin intervención manual.

Este hook usa el evento `PostToolUse` con un matcher `Edit|Write`, por lo que se ejecuta solo después de herramientas de edición de archivos. El comando extrae la ruta del archivo editado con [`jq`](https://jqlang.github.io/jq/) y la pasa a Prettier. Añade esto a `.claude/settings.json` en la raíz de tu proyecto:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write"
          }
        ]
      }
    ]
  }
}
```

En Claude Code v2.1.191 o posterior también puedes escribir el matcher como `Edit,Write`, ya que `|` y `,` son separadores de lista intercambiables para matchers de nombre de herramienta en esas versiones.

<Note>
  Los ejemplos de Bash en esta página usan `jq` para análisis JSON. Instálalo con `brew install jq` en macOS, `apt-get install jq` en Debian y Ubuntu, o consulta [descargas de `jq`](https://jqlang.github.io/jq/download/).
</Note>

<h3 id="block-edits-to-protected-files">
  Bloquea ediciones a archivos protegidos
</h3>

Evita que Claude modifique archivos sensibles como `.env`, `package-lock.json`, o cualquier cosa en `.git/`. Claude recibe retroalimentación explicando por qué se bloqueó la edición, para que pueda ajustar su enfoque.

Este ejemplo usa un archivo de script separado que el hook llama. El script verifica la ruta del archivo de destino contra una lista de patrones protegidos y sale con código 2 para bloquear la edición.

<Steps>
  <Step title="Crea el script del hook">
    Guarda esto en `.claude/hooks/protect-files.sh`:

    ```bash theme={null}
    #!/bin/bash
    # protect-files.sh

    INPUT=$(cat)
    FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

    PROTECTED_PATTERNS=(".env" "package-lock.json" ".git/")

    for pattern in "${PROTECTED_PATTERNS[@]}"; do
      if [[ "$FILE_PATH" == *"$pattern"* ]]; then
        echo "Blocked: $FILE_PATH matches protected pattern '$pattern'" >&2
        exit 2
      fi
    done

    exit 0
    ```
  </Step>

  <Step title="Haz el script ejecutable en macOS y Linux">
    Los scripts de hook deben ser ejecutables para que Claude Code los ejecute:

    ```bash theme={null}
    chmod +x .claude/hooks/protect-files.sh
    ```
  </Step>

  <Step title="Registra el hook">
    Añade un hook `PreToolUse` a `.claude/settings.json` que ejecute el script antes de cualquier llamada a herramienta `Edit` o `Write`:

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [
              {
                "type": "command",
                "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/protect-files.sh"
              }
            ]
          }
        ]
      }
    }
    ```
  </Step>
</Steps>

<h3 id="re-inject-context-after-compaction">
  Reinyecta contexto después de compactación
</h3>

Cuando la ventana de contexto de Claude se llena, la compactación resume la conversación para liberar espacio. Esto puede perder detalles importantes. Usa un hook `SessionStart` con un matcher `compact` para reinyectar contexto crítico después de cada compactación.

Cualquier texto que tu comando escriba en stdout se añade al contexto de Claude. Este ejemplo recuerda a Claude las convenciones del proyecto y el trabajo reciente. Añade esto a `.claude/settings.json` en la raíz de tu proyecto:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "compact",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'Reminder: use Bun, not npm. Run bun test before committing. Current sprint: auth refactor.'"
          }
        ]
      }
    ]
  }
}
```

Puedes reemplazar el `echo` con cualquier comando que produzca salida dinámica, como `git log --oneline -5` para mostrar commits recientes. Para inyectar contexto en cada inicio de sesión, considera usar [CLAUDE.md](/docs/es/memory) en su lugar. Para variables de entorno, consulta [`CLAUDE_ENV_FILE`](/docs/es/hooks#persist-environment-variables) en la referencia.

<h3 id="audit-configuration-changes">
  Audita cambios de configuración
</h3>

Realiza un seguimiento de cuándo los archivos de configuración o skills cambian durante una sesión. El evento `ConfigChange` se activa cuando un proceso externo o editor modifica un archivo de configuración, para que puedas registrar cambios para cumplimiento o bloquear modificaciones no autorizadas.

Este ejemplo añade cada cambio a un registro de auditoría. Añade esto a `~/.claude/settings.json`:

```json theme={null}
{
  "hooks": {
    "ConfigChange": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "jq -c '{timestamp: now | todate, source: .source, file: .file_path}' >> ~/claude-config-audit.log"
          }
        ]
      }
    ]
  }
}
```

El matcher filtra por tipo de configuración: `user_settings`, `project_settings`, `local_settings`, `policy_settings`, o `skills`. Para bloquear que un cambio tenga efecto, sal con código 2 o devuelve `{"decision": "block"}`. Consulta la [referencia de ConfigChange](/docs/es/hooks#configchange) para el esquema de entrada completo.

<h3 id="reload-environment-when-directory-or-files-change">
  Recarga el entorno cuando el directorio o los archivos cambian
</h3>

Algunos proyectos establecen diferentes variables de entorno dependiendo de en qué directorio estés. Herramientas como [direnv](https://direnv.net/) hacen esto automáticamente en tu shell, pero la herramienta Bash de Claude no recoge esos cambios por sí sola.

Emparejar un hook `SessionStart` con un hook `CwdChanged` arregla esto. `SessionStart` carga las variables para el directorio en el que lanzas, y `CwdChanged` las recarga cada vez que Claude cambia de directorio. Ambos escriben en `CLAUDE_ENV_FILE`, que Claude Code ejecuta como un preámbulo de script antes de cada comando Bash. Añade esto a `~/.claude/settings.json`:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ],
    "CwdChanged": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ]
  }
}
```

Ejecuta `direnv allow` una vez en cada directorio que tenga un `.envrc` para que direnv tenga permiso de cargarlo. Si usas devbox o nix en lugar de direnv, el mismo patrón funciona con `devbox shellenv` o `devbox global shellenv` en lugar de `direnv export bash`.

Para reaccionar a archivos específicos en lugar de cada cambio de directorio, usa `FileChanged` con un `matcher` listando los nombres de archivo a observar, separados por `|`. Cuando construyas la lista de observación, Claude Code divide este valor en nombres de archivo literales en lugar de evaluarlo como una expresión regular. Consulta [FileChanged](/docs/es/hooks#filechanged) para cómo el mismo valor también filtra qué grupos de hooks se ejecutan cuando un archivo cambia. Este ejemplo observa `.envrc` y `.env` en el directorio de trabajo:

```json theme={null}
{
  "hooks": {
    "FileChanged": [
      {
        "matcher": ".envrc|.env",
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ]
  }
}
```

Consulta las entradas de referencia [CwdChanged](/docs/es/hooks#cwdchanged) y [FileChanged](/docs/es/hooks#filechanged) para esquemas de entrada, salida `watchPaths`, y detalles de `CLAUDE_ENV_FILE`.

<h3 id="auto-approve-specific-permission-prompts">
  Aprueba automáticamente avisos de permiso específicos
</h3>

Omite el diálogo de aprobación para llamadas a herramientas que siempre permites. Este ejemplo aprueba automáticamente `ExitPlanMode`, la herramienta que Claude llama cuando termina de presentar un plan y pide proceder, para que no se te solicite cada vez que un plan esté listo.

A diferencia de los ejemplos de código de salida anteriores, la aprobación automática requiere que tu hook escriba una decisión JSON en stdout. Un hook `PermissionRequest` se activa cuando Claude Code está a punto de mostrar un diálogo de permiso, y devolver `"behavior": "allow"` lo responde en tu nombre.

El matcher limita el hook a `ExitPlanMode` solamente, para que ningún otro aviso se vea afectado. Añade esto a `~/.claude/settings.json`:

```json theme={null}
{
  "hooks": {
    "PermissionRequest": [
      {
        "matcher": "ExitPlanMode",
        "hooks": [
          {
            "type": "command",
            "command": "echo '{\"hookSpecificOutput\": {\"hookEventName\": \"PermissionRequest\", \"decision\": {\"behavior\": \"allow\"}}}'"
          }
        ]
      }
    ]
  }
}
```

Cuando el hook aprueba, Claude Code sale del modo plan y restaura cualquier modo de permiso que estuviera activo antes de entrar en modo plan. La transcripción muestra "Allowed by PermissionRequest hook" donde habría aparecido el diálogo. La ruta del hook siempre mantiene la conversación actual: no puede limpiar contexto e iniciar una sesión de implementación fresca de la manera que el diálogo puede.

Para establecer un modo de permiso específico en su lugar, la salida de tu hook puede incluir un array `updatedPermissions` con una entrada `setMode`. El valor `mode` es cualquier modo de permiso como `default`, `acceptEdits`, o `bypassPermissions`, y `destination: "session"` lo aplica solo para la sesión actual.

<Note>
  `bypassPermissions` solo se aplica si la sesión se lanzó con modo bypass ya disponible: `--dangerously-skip-permissions`, `--permission-mode bypassPermissions`, `--allow-dangerously-skip-permissions`, o `permissions.defaultMode: "bypassPermissions"` en configuración, y no deshabilitado por [`permissions.disableBypassPermissionsMode`](/docs/es/permissions#managed-settings). Nunca se persiste como `defaultMode`.
</Note>

Para cambiar la sesión a `acceptEdits`, tu hook escribe este JSON en stdout:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "allow",
      "updatedPermissions": [
        { "type": "setMode", "mode": "acceptEdits", "destination": "session" }
      ]
    }
  }
}
```

Mantén el matcher lo más estrecho posible. Coincidir con `.*` o dejar el matcher vacío aprobaría automáticamente cada aviso de permiso, incluyendo escrituras de archivos y comandos de shell. Consulta la [referencia de PermissionRequest](/docs/es/hooks#permissionrequest-decision-control) para el conjunto completo de campos de decisión.

<h2 id="how-hooks-work">
  Cómo funcionan los hooks
</h2>

Los eventos de hook se activan en puntos específicos del ciclo de vida de Claude Code. Cuando se activa un evento, todos los hooks coincidentes se ejecutan en paralelo, y los comandos de hook idénticos se deduplicarán automáticamente. La tabla a continuación muestra cada evento y cuándo se activa:

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

Cada hook tiene un `type` que determina cómo se ejecuta. La mayoría de los hooks usan `"type": "command"`, que ejecuta un comando de shell. Hay otros cuatro tipos disponibles:

* `"type": "http"`: POST de datos de evento a una URL. Consulta [HTTP hooks](#http-hooks).
* `"type": "mcp_tool"`: llamar a una herramienta en un servidor MCP ya conectado. Consulta [MCP tool hooks](/docs/es/hooks#mcp-tool-hook-fields).
* `"type": "prompt"`: evaluación LLM de un solo turno. Consulta [Prompt-based hooks](#prompt-based-hooks).
* `"type": "agent"`: verificación multi-turno con acceso a herramientas. Los hooks de agente son experimentales y pueden cambiar. Consulta [Agent-based hooks](#agent-based-hooks).

<h3 id="combine-results-from-multiple-hooks">
  Combina resultados de múltiples hooks
</h3>

Cuando múltiples hooks coinciden con el mismo evento, el comando de cada hook se ejecuta hasta completarse antes de que Claude Code fusione los resultados. Un hook que devuelve `deny` no detiene la ejecución de hooks hermanos. No confíes en que el `deny` de un hook suprima efectos secundarios en otro hook.

Después de que todos los hooks coincidentes terminen, Claude Code combina sus salidas. Para decisiones de permiso `PreToolUse`, la respuesta más restrictiva gana, en el orden `deny`, `defer`, `ask`, `allow`. El texto de `additionalContext` se mantiene de cada hook y se pasa a Claude junto.

El ejemplo a continuación registra dos hooks `PreToolUse` en `Bash`. El primero añade cada comando a un archivo de registro y sale con 0. El segundo ejecuta un script que sale con 2 para negar cuando el comando contiene `rm -rf`:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r .tool_input.command >> ~/.claude/bash.log"
          },
          {
            "type": "command",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/block-rm-rf.sh"
          }
        ]
      }
    ]
  }
}
```

Cuando Claude intenta ejecutar `rm -rf /tmp/build`, ambos hooks se ejecutan en paralelo. El hook de registro escribe el comando en `~/.claude/bash.log` y sale con 0, lo que no reporta ninguna decisión. El hook de protección sale con 2, lo que niega la llamada a herramienta. El deny gana, por lo que Claude Code bloquea el comando y muestra a Claude el stderr del guardrail. La entrada de registro se sigue escribiendo porque el hook de registro ya se ejecutó.

<h3 id="read-input-and-return-output">
  Lee entrada y devuelve salida
</h3>

Los hooks se comunican con Claude Code a través de stdin, stdout, stderr y códigos de salida. Cuando se activa un evento, Claude Code pasa datos específicos del evento como JSON a stdin de tu script. Tu script lee esos datos, hace su trabajo, y le dice a Claude Code qué hacer a continuación a través del código de salida.

<h4 id="hook-input">
  Entrada del hook
</h4>

Cada evento incluye campos comunes como `session_id` y `cwd`, pero cada tipo de evento añade datos diferentes. Por ejemplo, cuando Claude ejecuta un comando Bash, un hook `PreToolUse` recibe algo como esto en stdin:

```json theme={null}
{
  "session_id": "abc123",          // ID único para esta sesión
  "cwd": "/Users/sarah/myproject", // directorio de trabajo cuando se activó el evento
  "hook_event_name": "PreToolUse", // qué evento activó este hook
  "tool_name": "Bash",             // la herramienta que Claude está a punto de usar
  "tool_input": {                  // los argumentos que Claude pasó a la herramienta
    "command": "npm test"          // para Bash, este es el comando de shell
  }
}
```

Tu script puede analizar ese JSON y actuar sobre cualquiera de esos campos. Los hooks `UserPromptSubmit` obtienen el texto `prompt` en su lugar, los hooks `SessionStart` obtienen la `source` (startup, resume, clear, compact), y así sucesivamente. Consulta [Campos de entrada comunes](/docs/es/hooks#common-input-fields) en la referencia para campos compartidos, y la sección de cada evento para esquemas específicos del evento.

<h4 id="hook-output">
  Salida del hook
</h4>

Tu script le dice a Claude Code qué hacer a continuación escribiendo en stdout o stderr y saliendo con un código específico. El siguiente hook `PreToolUse` bloquea un comando:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command')

if echo "$COMMAND" | grep -q "drop table"; then
  echo "Blocked: dropping tables is not allowed" >&2  # stderr se convierte en retroalimentación de Claude
  exit 2 # exit 2 = bloquea la acción
fi

exit 0  # exit 0 = no hay objeción; el flujo de permiso normal se aplica
```

El código de salida determina qué sucede a continuación:

* **Exit 0**: el hook reporta sin objeción y la acción procede normalmente. Para un hook `PreToolUse` esto no aprueba la llamada a herramienta: el [flujo de permiso](/docs/es/permissions) normal aún se aplica. Para hooks `UserPromptSubmit`, `UserPromptExpansion`, y `SessionStart`, cualquier cosa que escribas en stdout se añade al contexto de Claude.
* **Exit 2**: la acción se bloquea. Escribe una razón en stderr, y Claude la recibe como retroalimentación para que pueda ajustar. Algunos eventos no pueden ser bloqueados: para `SessionStart`, `Setup`, `Notification`, y otros, exit 2 muestra stderr al usuario y la ejecución continúa. Consulta [comportamiento del código de salida 2 por evento](/docs/es/hooks#exit-code-2-behavior-per-event) para la lista completa.
* **Cualquier otro código de salida**: la acción procede. La transcripción muestra un aviso `<hook name> hook error` seguido de la primera línea de stderr; el stderr completo va al [registro de depuración](/docs/es/hooks#debug-hooks).

<h4 id="structured-json-output">
  Salida JSON estructurada
</h4>

Los códigos de salida solo te permiten bloquear o permanecer en silencio. Para más control, sal con 0 e imprime un objeto JSON a stdout en su lugar.

<Note>
  Usa exit 2 para bloquear con un mensaje de stderr, o exit 0 con JSON para control estructurado. No los mezcles: Claude Code ignora JSON cuando sales con 2.
</Note>

Por ejemplo, un hook `PreToolUse` puede negar una llamada a herramienta y decirle a Claude por qué, o escalarla al usuario para aprobación:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "deny",
    "permissionDecisionReason": "Use rg instead of grep for better performance"
  }
}
```

Con `"deny"`, Claude Code cancela la llamada a herramienta y alimenta `permissionDecisionReason` de vuelta a Claude. Estos valores `permissionDecision` son específicos de `PreToolUse`:

* `"allow"`: omite el aviso de permiso interactivo. Las reglas de negación y solicitud, incluyendo listas de negación gestionadas empresariales, aún se aplican, al igual que los avisos para herramientas de conector [que tu organización configuró como `ask`](/docs/es/mcp#organization-controls-on-connector-tools) y herramientas MCP marcadas como [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool)
* `"deny"`: cancela la llamada a herramienta y envía la razón a Claude
* `"ask"`: muestra el aviso de permiso al usuario como es normal

Un cuarto valor, `"defer"`, está disponible en [modo no interactivo](/docs/es/headless) con la bandera `-p`. Sale del proceso con la llamada a herramienta preservada para que un envoltorio del SDK del Agente pueda recopilar entrada y reanudar. Consulta [Defer a tool call for later](/docs/es/hooks#defer-a-tool-call-for-later) en la referencia.

Devolver `"allow"` omite el aviso interactivo pero no anula [reglas de permiso](/docs/es/permissions#manage-permissions). Si una regla de negación coincide con la llamada a herramienta, la llamada se bloquea incluso cuando tu hook devuelve `"allow"`. Si una regla de solicitud coincide, el usuario sigue siendo solicitado, al igual que las herramientas de conector [que tu organización configuró como `ask`](/docs/es/mcp#organization-controls-on-connector-tools) y herramientas MCP marcadas como [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool). Esto significa que las reglas de negación de cualquier ámbito de configuración, incluyendo [configuración gestionada](/docs/es/settings#settings-files), siempre tienen prioridad sobre las aprobaciones de hooks.

Otros eventos usan patrones de decisión diferentes. Por ejemplo, los hooks `PostToolUse` y `Stop` usan un campo `decision: "block"` de nivel superior, mientras que `PermissionRequest` usa `hookSpecificOutput.decision.behavior`. Consulta la [tabla de resumen](/docs/es/hooks#decision-control) en la referencia para un desglose completo por evento.

Para hooks `UserPromptSubmit`, usa `hookSpecificOutput.additionalContext` en su lugar para inyectar texto en el contexto de Claude. Anida `additionalContext` dentro de `hookSpecificOutput`; si lo colocas en el nivel superior del JSON, Claude Code lo ignora silenciosamente. Por ejemplo, esta salida añade el estado de rama actual a cada aviso:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "UserPromptSubmit",
    "additionalContext": "Current branch: release-42. Deploy freeze until Friday."
  }
}
```

Consulta [UserPromptSubmit decision control](/docs/es/hooks#userpromptsubmit-decision-control) para la forma de salida completa, incluyendo bloqueo de avisos y configuración del título de sesión.

Los hooks con `type: "prompt"` manejan la salida de manera diferente: consulta [Prompt-based hooks](#prompt-based-hooks).

<h3 id="filter-hooks-with-matchers">
  Filtra hooks con matchers
</h3>

Sin un matcher, un hook se activa en cada ocurrencia de su evento. Los matchers te permiten estrecharlo. Por ejemplo, si quieres ejecutar un formateador solo después de ediciones de archivos, no después de cada llamada a herramienta, añade un matcher a tu hook `PostToolUse`:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          { "type": "command", "command": "prettier --write ..." }
        ]
      }
    ]
  }
}
```

El matcher `"Edit|Write"` se activa solo cuando Claude usa la herramienta `Edit` o `Write`, no cuando usa `Bash`, `Read`, u otra herramienta. En Claude Code v2.1.191 o posterior, una coma separa alternativas de la misma manera, por lo que `"Edit, Write"` es equivalente. Consulta [Matcher patterns](/docs/es/hooks#matcher-patterns) para cómo se evalúan los nombres simples y las expresiones regulares.

<Note>
  Claude también puede crear o modificar archivos ejecutando comandos de shell a través de la herramienta `Bash`. Si tu hook debe ver cada cambio de archivo, como para escaneo de cumplimiento o registro de auditoría, añade un hook [`Stop`](/docs/es/hooks#stop) que escanee el árbol de trabajo una vez por turno. Para cobertura por llamada en su lugar, también coincide con `Bash` y haz que tu script liste archivos modificados y sin seguimiento con `git status --porcelain`.
</Note>

Cada tipo de evento coincide en un campo específico:

| Evento                                                                                                                                                          | En qué filtra el matcher                                                                | Valores de matcher de ejemplo                                                                                                                                                       |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`                                                                      | nombre de herramienta                                                                   | `Bash`, `Edit\|Write`, `mcp__.*`                                                                                                                                                    |
| `SessionStart`                                                                                                                                                  | cómo comenzó la sesión                                                                  | `startup`, `resume`, `clear`, `compact`                                                                                                                                             |
| `Setup`                                                                                                                                                         | qué bandera CLI activó la configuración                                                 | `init`, `maintenance`                                                                                                                                                               |
| `SessionEnd`                                                                                                                                                    | por qué terminó la sesión                                                               | `clear`, `resume`, `logout`, `prompt_input_exit`, `bypass_permissions_disabled`, `other`                                                                                            |
| `Notification`                                                                                                                                                  | tipo de notificación                                                                    | `permission_prompt`, `idle_prompt`, `auth_success`, `elicitation_dialog`, `elicitation_complete`, `elicitation_response`, `agent_needs_input`, `agent_completed`                    |
| `SubagentStart`                                                                                                                                                 | tipo de agente                                                                          | `general-purpose`, `Explore`, `Plan`, o nombres de agentes personalizados                                                                                                           |
| `PreCompact`, `PostCompact`                                                                                                                                     | qué activó la compactación                                                              | `manual`, `auto`                                                                                                                                                                    |
| `SubagentStop`                                                                                                                                                  | tipo de agente                                                                          | los mismos valores que `SubagentStart`                                                                                                                                              |
| `ConfigChange`                                                                                                                                                  | fuente de configuración                                                                 | `user_settings`, `project_settings`, `local_settings`, `policy_settings`, `skills`                                                                                                  |
| `StopFailure`                                                                                                                                                   | tipo de error                                                                           | `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `unknown` |
| `InstructionsLoaded`                                                                                                                                            | razón de carga                                                                          | `session_start`, `nested_traversal`, `path_glob_match`, `include`, `compact`                                                                                                        |
| `Elicitation`                                                                                                                                                   | nombre del servidor MCP                                                                 | tus nombres de servidor MCP configurados                                                                                                                                            |
| `ElicitationResult`                                                                                                                                             | nombre del servidor MCP                                                                 | los mismos valores que `Elicitation`                                                                                                                                                |
| `FileChanged`                                                                                                                                                   | nombres de archivo literales a observar (consulta [FileChanged](/docs/es/hooks#filechanged)) | `.envrc\|.env`                                                                                                                                                                      |
| `UserPromptExpansion`                                                                                                                                           | nombre del comando                                                                      | tus nombres de skill o comando                                                                                                                                                      |
| `UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `CwdChanged`, `MessageDisplay` | sin soporte de matcher                                                                  | siempre se activa en cada ocurrencia                                                                                                                                                |

Las pestañas a continuación muestran algunos matchers más en diferentes tipos de eventos.

<Tabs>
  <Tab title="Registra cada comando Bash">
    Coincide solo con llamadas a herramienta `Bash` y registra cada comando en un archivo. El evento `PostToolUse` se activa después de que el comando se completa, por lo que `tool_input.command` contiene lo que se ejecutó. El hook recibe los datos del evento como JSON en stdin, y `jq -r '.tool_input.command'` extrae solo la cadena de comando, que `>>` añade al archivo de registro:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "jq -r '.tool_input.command' >> ~/.claude/command-log.txt"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Coincide con herramientas MCP">
    Las herramientas MCP usan una convención de nombres diferente a las herramientas integradas: `mcp__<server>__<tool>`, donde `<server>` es el nombre del servidor MCP y `<tool>` es la herramienta que proporciona. Por ejemplo, `mcp__github__search_repositories` o `mcp__filesystem__read_file`. Las herramientas de un [servidor MCP proporcionado por plugin](/docs/es/mcp#plugin-provided-mcp-servers) usan un segmento de servidor con ámbito en su lugar, como `mcp__plugin_my-plugin_db__query`. Usa un matcher regex para dirigirse a todas las herramientas de un servidor específico, o coincide entre servidores con un patrón como `mcp__.*__write.*`. Consulta [Match MCP tools](/docs/es/hooks#match-mcp-tools) en la referencia para la lista completa de ejemplos.

    El comando a continuación extrae el nombre de la herramienta de la entrada JSON del hook con `jq` y lo escribe en stderr. Escribir en stderr mantiene stdout limpio para salida JSON y envía el mensaje al [registro de depuración](/docs/es/hooks#debug-hooks):

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "mcp__github__.*",
            "hooks": [
              {
                "type": "command",
                "command": "echo \"GitHub tool called: $(jq -r '.tool_name')\" >&2"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Limpia al final de la sesión">
    El evento `SessionEnd` soporta matchers en la razón por la que terminó la sesión. Este hook solo se activa en `clear` (cuando ejecutas `/clear`), no en salidas normales:

    ```json theme={null}
    {
      "hooks": {
        "SessionEnd": [
          {
            "matcher": "clear",
            "hooks": [
              {
                "type": "command",
                "command": "rm -f /tmp/claude-scratch-*.txt"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>
</Tabs>

Para sintaxis de matcher completa, consulta la [referencia de Hooks](/docs/es/hooks#configuration).

<h4 id="filter-by-tool-name-and-arguments-with-the-if-field">
  Filtra por nombre de herramienta y argumentos con el campo `if`
</h4>

El campo `if` usa [sintaxis de regla de permiso](/docs/es/permissions) para filtrar hooks por nombre de herramienta y argumentos juntos, para que el proceso del hook solo se genere cuando la llamada a herramienta coincida. Esto va más allá de `matcher`, que filtra a nivel de grupo solo por nombre de herramienta.

Por ejemplo, esta configuración ejecuta un hook solo cuando Claude usa comandos `git` en lugar de todos los comandos Bash:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "if": "Bash(git *)",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/check-git-policy.sh"
          }
        ]
      }
    ]
  }
}
```

Si tu hook debe ejecutarse depende de la forma de tu patrón `if` y del comando Bash que Claude está invocando:

| Patrón `if`        | Comando Bash           | ¿Se ejecuta el hook? | Por qué                                                                                                                   |
| :----------------- | :--------------------- | :------------------- | :------------------------------------------------------------------------------------------------------------------------ |
| `Bash(git *)`      | `git push`             | sí                   | el nombre del comando coincide                                                                                            |
| `Bash(git *)`      | `npm test && git push` | sí                   | cada subcomando se verifica; `git push` coincide                                                                          |
| `Bash(git *)`      | `echo $(git log)`      | sí                   | los comandos dentro de `$()` y backticks se verifican; `git log` coincide                                                 |
| `Bash(git *)`      | `echo $(date)`         | no                   | ningún subcomando coincide con `git *`                                                                                    |
| `Bash(git push *)` | `echo $(date)`         | sí                   | los patrones que especifican más que el nombre del comando ejecutan el hook de todas formas en `$()`, backticks, o `$VAR` |

El filtro también falla abierto, ejecutando tu hook independientemente del patrón, cuando el comando Bash no puede ser analizado. Porque el filtro es mejor esfuerzo, usa el [sistema de permisos](/docs/es/permissions) en lugar de un hook para aplicar un allow o deny duro.

El campo `if` acepta los mismos patrones que las reglas de permiso: `"Bash(git *)"`, `"Edit(*.ts)"`, y así sucesivamente. Para coincidir con múltiples nombres de herramienta, usa manejadores separados cada uno con su propio valor `if`, o coincide a nivel de `matcher` donde se soporta alternancia de tuberías.

`if` solo funciona en eventos de herramienta: `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, y `PermissionDenied`. Añadirlo a cualquier otro evento evita que el hook se ejecute.

<h3 id="configure-hook-location">
  Configura la ubicación del hook
</h3>

Dónde añadas un hook determina su ámbito:

| Ubicación                                                  | Ámbito                                 | Compartible                                |
| :--------------------------------------------------------- | :------------------------------------- | :----------------------------------------- |
| `~/.claude/settings.json`                                  | Todos tus proyectos                    | No, local a tu máquina                     |
| `.claude/settings.json`                                    | Proyecto único                         | Sí, puede ser confirmado en el repositorio |
| `.claude/settings.local.json`                              | Proyecto único                         | No, gitignored cuando Claude Code lo crea  |
| Configuración de política gestionada                       | Organización completa                  | Sí, controlado por administrador           |
| [Plugin](/docs/es/plugins) `hooks/hooks.json`                   | Cuando el plugin está habilitado       | Sí, incluido con el plugin                 |
| [Skill](/docs/es/skills) o [agente](/docs/es/sub-agents) frontmatter | Mientras el skill o agente está activo | Sí, definido en el archivo del componente  |

Ejecuta [`/hooks`](/docs/es/hooks#the-%2Fhooks-menu) en Claude Code para examinar todos los hooks configurados agrupados por evento.

Para desactivar hooks, establece `"disableAllHooks": true` en tu archivo de configuración. Los hooks configurados en configuración gestionada aún se ejecutan a menos que `disableAllHooks` también esté establecido allí.

Si editas archivos de configuración directamente mientras Claude Code está ejecutándose, el observador de archivos normalmente recoge cambios de hooks automáticamente.

<h2 id="prompt-based-hooks">
  Hooks basados en prompts
</h2>

Para decisiones que requieren criterio en lugar de reglas deterministas, usa hooks `type: "prompt"`. En lugar de ejecutar un comando de shell, Claude Code envía tu prompt y los datos de entrada del hook a un modelo Claude (Haiku por defecto) para tomar la decisión. Puedes especificar un modelo diferente con el campo `model` si necesitas más capacidad.

El único trabajo del modelo es devolver una decisión sí/no como JSON:

* `"ok": true`: la acción procede
* `"ok": false`: lo que sucede depende del evento:
  * `Stop` y `SubagentStop`: la `reason` se alimenta de vuelta a Claude para que siga trabajando
  * `PreToolUse`: la llamada de herramienta se deniega y la `reason` se devuelve a Claude como el error de la herramienta, para que pueda ajustarse y continuar
  * `PostToolUse`, `PostToolBatch`, `UserPromptSubmit` y `UserPromptExpansion`: el turno termina y la `reason` aparece en el chat como una línea de advertencia

Este ejemplo usa un hook `Stop` para preguntarle al modelo si todas las tareas solicitadas están completas. Si el modelo devuelve `"ok": false`, Claude sigue trabajando y usa la `reason` como su siguiente instrucción:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "Check if all tasks are complete. If not, respond with {\"ok\": false, \"reason\": \"what remains to be done\"}."
          }
        ]
      }
    ]
  }
}
```

Para opciones de configuración completas, consulta [Hooks basados en prompts](/docs/es/hooks#prompt-based-hooks) en la referencia.

<h2 id="agent-based-hooks">
  Hooks basados en agentes
</h2>

<Warning>
  Los hooks de agente son experimentales. El comportamiento y la configuración pueden cambiar en futuras versiones. Para flujos de trabajo de producción, prefiere [hooks de comando](/docs/es/hooks#command-hook-fields).
</Warning>

Cuando la verificación requiere inspeccionar archivos o ejecutar comandos, usa hooks `type: "agent"`. A diferencia de los hooks de prompt que hacen una sola llamada LLM, los hooks de agente generan un subagente que puede leer archivos, buscar código y usar otras herramientas para verificar condiciones antes de devolver una decisión.

Los hooks de agente usan el mismo formato de respuesta `"ok"` / `"reason"` que los hooks de prompt, pero con un tiempo de espera predeterminado más largo de 60 segundos y hasta 50 turnos de uso de herramientas.

Este ejemplo verifica que las pruebas pasen antes de permitir que Claude se detenga:

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

Usa hooks de prompt cuando los datos de entrada del hook por sí solos son suficientes para tomar una decisión. Usa hooks de agente cuando necesites verificar algo contra el estado real del código base.

Para opciones de configuración completas, consulta [Hooks basados en agentes](/docs/es/hooks#agent-based-hooks) en la referencia.

<h2 id="http-hooks">
  HTTP hooks
</h2>

Usa hooks `type: "http"` para POST de datos de evento a un punto final HTTP en lugar de ejecutar un comando de shell. El punto final recibe el mismo JSON que un hook de comando recibiría en stdin, y devuelve resultados a través del cuerpo de respuesta HTTP usando el mismo formato JSON.

Los HTTP hooks son útiles cuando quieres que un servidor web, función en la nube o servicio externo maneje la lógica del hook: por ejemplo, un servicio de auditoría compartido que registra eventos de uso de herramientas en un equipo.

Este ejemplo publica cada uso de herramienta a un servicio de registro local:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "http",
            "url": "http://localhost:8080/hooks/tool-use",
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

El punto final debe devolver un cuerpo de respuesta JSON usando el mismo [formato de salida](/docs/es/hooks#json-output) que los hooks de comando. Para bloquear una llamada a herramienta, devuelve una respuesta 2xx con los campos `hookSpecificOutput` apropiados. Los códigos de estado HTTP por sí solos no pueden bloquear acciones.

Los valores de encabezado soportan interpolación de variables de entorno usando la sintaxis `$VAR_NAME` o `${VAR_NAME}`. Solo las variables listadas en el array `allowedEnvVars` se resuelven; todas las otras referencias `$VAR` permanecen vacías.

Para opciones de configuración completas y manejo de respuestas, consulta [HTTP hooks](/docs/es/hooks#http-hook-fields) en la referencia.

<h2 id="limitations-and-troubleshooting">
  Limitaciones y solución de problemas
</h2>

<h3 id="limitations">
  Limitaciones
</h3>

Tenga en cuenta estas restricciones al diseñar hooks:

* Los hooks de comando se comunican solo a través de stdout, stderr y códigos de salida. No pueden activar comandos `/` o llamadas a herramientas. El texto devuelto a través de `additionalContext` se inyecta como un recordatorio del sistema que Claude lee como texto plano. Los HTTP hooks se comunican a través del cuerpo de respuesta en su lugar.
* Los tiempos de espera del hook varían según el tipo. Anule por hook con el campo `timeout` en segundos.
  * `command`, `http`, `mcp_tool`: 10 minutos. `UserPromptSubmit` reduce estos a 30 segundos, y `MessageDisplay` los reduce a 10 segundos.
  * `prompt`: 30 segundos.
  * `agent`: 60 segundos.
* Los hooks `PostToolUse` no pueden deshacer acciones ya que la herramienta ya se ha ejecutado.
* Los hooks `PermissionRequest` no se activan en [modo no interactivo](/docs/es/headless) con la bandera `-p`. Use hooks `PreToolUse` para decisiones de permiso automatizadas.
* Los hooks `Stop` se activan cada vez que Claude termina de responder, no solo en la finalización de tareas. No se activan en interrupciones del usuario. Los errores de API activan [StopFailure](/docs/es/hooks#stopfailure) en su lugar.
* Cuando múltiples hooks `PreToolUse` devuelven [`updatedInput`](/docs/es/hooks#pretooluse) para reescribir los argumentos de una herramienta, el último en terminar gana. Como los hooks se ejecutan en paralelo, el orden es no determinista. Evite tener más de un hook modificando la entrada de la misma herramienta.

<h3 id="hooks-and-permission-modes">
  Hooks y modos de permiso
</h3>

Los hooks `PreToolUse` se activan antes de cualquier verificación de modo de permiso. Un hook que devuelve `permissionDecision: "deny"` bloquea la herramienta incluso en modo `bypassPermissions` o con `--dangerously-skip-permissions`. Esto le permite aplicar política que los usuarios no pueden eludir cambiando su modo de permiso.

Lo inverso no es cierto: un hook que devuelve `"allow"` no elude reglas de negación de configuración, y no puede suprimir la solicitud de herramientas conectoras [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools) o herramientas MCP marcadas [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool). Los hooks pueden endurecer restricciones pero no relajarlas más allá de lo que las reglas de permiso permiten.

<h3 id="hook-not-firing">
  Hook no se activa
</h3>

El hook está configurado pero nunca se ejecuta.

* Ejecuta `/hooks` y confirma que el hook aparece bajo el evento correcto
* Verifique que el patrón del matcher coincida exactamente con el nombre de la herramienta. Los matchers distinguen mayúsculas de minúsculas
* Verifique que esté activando el tipo de evento correcto: `PreToolUse` se activa antes de la ejecución de la herramienta, `PostToolUse` se activa después
* Si usa hooks `PermissionRequest` en modo no interactivo con la bandera `-p`, cambie a `PreToolUse` en su lugar

<h3 id="hook-error-in-output">
  Error de hook en la salida
</h3>

Ves un mensaje como "PreToolUse hook error: ..." en la transcripción.

* Tu script salió con un código no cero inesperadamente. Pruébalo manualmente canalizando JSON de muestra:
  ```bash theme={null}
  echo '{"tool_name":"Bash","tool_input":{"command":"ls"}}' | ./my-hook.sh
  echo $?  # Verifica el código de salida
  ```
* Si ves "command not found", usa rutas absolutas o `${CLAUDE_PROJECT_DIR}` para referenciar scripts. Para evitar entrecomillado de shell por completo, añade `"args": []` para cambiar a [forma exec](/docs/es/hooks#exec-form-and-shell-form), que genera el script directamente sin un shell
* Si ves "jq: command not found", instala `jq` o usa Python/Node.js para análisis JSON
* Si el script no se ejecuta en absoluto, hazlo ejecutable: `chmod +x ./my-hook.sh`

<h3 id="/hooks-shows-no-hooks-configured">
  `/hooks` no muestra hooks configurados
</h3>

Editaste un archivo de configuración pero los hooks no aparecen en el menú.

* Las ediciones de archivos normalmente se recogen automáticamente. Si no han aparecido después de unos segundos, el observador de archivos puede haber perdido el cambio: reinicia tu sesión para forzar una recarga.
* Verifique que su JSON sea válido: las comas finales y comentarios no están permitidos
* Confirma que el archivo de configuración está en la ubicación correcta: `.claude/settings.json` para hooks de proyecto, `~/.claude/settings.json` para hooks globales

<h3 id="stop-hook-hits-the-block-cap">
  El hook Stop alcanza el límite de bloqueo
</h3>

Claude sigue trabajando en lugar de detenerse, luego termina el turno con una advertencia de que el hook Stop bloqueó demasiadas veces consecutivas.

Claude Code anula un hook Stop después de que bloquea ocho veces seguidas sin progreso. Su script de hook necesita verificar si ya activó una continuación. Analice el campo `stop_hook_active` de la entrada JSON y salga temprano si es `true`:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
if [ "$(echo "$INPUT" | jq -r '.stop_hook_active')" = "true" ]; then
  exit 0  # Permite que Claude se detenga
fi
# ... resto de tu lógica de hook
```

Si tu hook legítimamente necesita más de ocho iteraciones para converger, aumenta el límite con [`CLAUDE_CODE_STOP_HOOK_BLOCK_CAP`](/docs/es/env-vars).

<h3 id="json-validation-failed">
  Falló la validación JSON
</h3>

Claude Code muestra un error de análisis JSON aunque tu script de hook produzca JSON válido.

Cuando Claude Code ejecuta un hook de comando en forma de shell (uno sin `args`), genera `sh -c` en macOS y Linux o Git Bash en Windows por defecto. Este shell es no interactivo, pero Git Bash y algunas configuraciones, como `BASH_ENV` apuntando a `~/.bashrc`, aún obtienen su perfil. Si ese perfil contiene declaraciones `echo` incondicionales, la salida se antepone a su JSON del hook:

```text theme={null}
Shell ready on arm64
{"decision": "block", "reason": "Not allowed"}
```

Claude Code intenta analizar esto como JSON y falla. Para arreglarlo, envuelve las declaraciones echo en tu perfil de shell para que solo se ejecuten en shells interactivos:

```bash theme={null}
# En ~/.zshrc o ~/.bashrc
if [[ $- == *i* ]]; then
  echo "Shell ready"
fi
```

La variable `$-` contiene banderas de shell, e `i` significa interactivo. Los hooks se ejecutan en shells no interactivos, por lo que el echo se omite.

<h3 id="debug-techniques">
  Técnicas de depuración
</h3>

La vista de transcripción, alternada con `Ctrl+O`, muestra un resumen de una línea para cada hook que se activó: el éxito es silencioso, los errores de bloqueo muestran stderr, y los errores sin bloqueo muestran un aviso `<hook name> hook error` seguido de la primera línea de stderr.

Para detalles de ejecución completos incluyendo qué hooks coincidieron, sus códigos de salida, stdout y stderr, lee el registro de depuración. Inicia Claude Code con `claude --debug-file /tmp/claude.log` para escribir en una ruta conocida, luego `tail -f /tmp/claude.log` en otra terminal. Si iniciaste sin esa bandera, ejecuta `/debug` a mitad de sesión para habilitar el registro y encontrar la ruta del registro.

<h2 id="learn-more">
  Aprende más
</h2>

* [Referencia de Hooks](/docs/es/hooks): esquemas de eventos completos, formato de salida JSON, hooks asincronos y hooks de herramientas MCP
* [Consideraciones de seguridad](/docs/es/hooks#security-considerations): revisa antes de desplegar hooks en entornos compartidos o de producción
* [Ejemplo de validador de comandos Bash](https://github.com/anthropics/claude-code/blob/main/examples/hooks/bash_command_validator_example.py): implementación de referencia completa
