> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Automatizar acciones con hooks

> Ejecuta comandos de shell automáticamente cuando Claude Code edita archivos, finaliza tareas o necesita entrada. Formatea código, envía notificaciones, valida comandos y aplica reglas del proyecto.

Los hooks son comandos de shell definidos por el usuario. Claude Code los ejecuta en puntos específicos de su ciclo de vida, lo que le proporciona control determinista: ciertas acciones siempre ocurren en lugar de depender de que el LLM elija ejecutarlas. Utilice hooks para aplicar reglas del proyecto, automatizar tareas repetitivas e integrar Claude Code con sus herramientas existentes.

Para decisiones que requieren criterio en lugar de reglas deterministas, también puedes usar [hooks basados en prompts](#prompt-based-hooks) o [hooks basados en agentes](#agent-based-hooks) que utilizan un modelo Claude para evaluar condiciones.

Para otras formas de extender Claude Code, consulte [skills](/docs/es/skills) para dar a Claude instrucciones adicionales y comandos ejecutables, [subagents](/docs/es/sub-agents) para ejecutar tareas en contextos aislados, y [plugins](/docs/es/plugins/overview) para empaquetar extensiones para compartir entre proyectos.

<Tip>
  Esta guía cubre casos de uso comunes y cómo comenzar. Para esquemas de eventos completos, formatos de entrada/salida JSON y características avanzadas como hooks asincronos y hooks de herramientas MCP, consulta la [referencia de Hooks](/docs/es/hooks).
</Tip>

<h2 id="set-up-your-first-hook">
  Configura tu primer hook
</h2>

Para crear un hook, añade un bloque `hooks` a un [archivo de configuración](#configure-hook-location). Este tutorial crea un hook de notificación de escritorio, para que recibas una alerta cada vez que Claude esté esperando tu entrada en lugar de ver la terminal.

<Steps>
  <Step title="Añade el hook a tu configuración">
    Abre `~/.claude/settings.json` y añade un hook `Notification`. Si el archivo no existe, créalo. El ejemplo a continuación usa `osascript` para macOS; consulta [Recibe notificaciones cuando Claude necesita entrada](#get-notified-when-claude-needs-input) para comandos de Linux y Windows.

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
    Presiona `Esc` para volver a la CLI. Presiona `Shift+Tab` hasta que la barra de estado muestre `⏸ manual mode on`, pídele a Claude que haga algo que requiera permiso, luego cambia de la terminal. Deberías recibir una notificación de escritorio.
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

Este hook usa el evento `Notification`, que Claude Code activa cuando Claude está esperando entrada o permiso. Consulta [cuándo se activa cada tipo de notificación](/docs/es/hooks#notification) para el momento exacto. Cada pestaña a continuación usa el comando de notificación nativo de la plataforma. Añade esto a `~/.claude/settings.json`:

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

    <Accordion title="Si no aparece ninguna notificación">
      `notify-send` necesita un demonio de notificación de escritorio, que los servidores sin interfaz gráfica, sesiones SSH y la mayoría de contenedores no tienen. Prueba el comando directamente primero:

      ```bash theme={null}
      notify-send 'Claude Code' 'test'
      ```

      Si el comando no se encuentra, instala el paquete `libnotify-bin` en Debian y Ubuntu, o el equivalente de tu distribución.
    </Accordion>
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

    <Accordion title="Si no aparece ningún diálogo">
      Este comando abre un cuadro de diálogo en lugar de una notificación en la esquina de tu pantalla, por lo que el diálogo puede abrirse detrás de tu ventana de terminal. Prueba el comando directamente en PowerShell primero. Si ejecutas Claude Code dentro de WSL, `powershell.exe` debe estar disponible en tu `PATH` a través de interoperabilidad de Windows.
    </Accordion>
  </Tab>
</Tabs>

El matcher vacío se activa en todos los tipos de notificación. Para activarse solo en eventos específicos, establécelo en uno de estos valores:

| Matcher                      | Se activa cuando                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :--------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permission_prompt`          | Claude necesita que apruebes un uso de herramienta o una [solicitud de red](/docs/es/sandboxing#network-isolation) de un comando en sandbox, y el aviso ha esperado aproximadamente seis segundos                                                                                                                                                                                                                                                                                               |
| `idle_prompt`                | Claude terminó de responder hace aproximadamente 60 segundos y no has escrito desde entonces                                                                                                                                                                                                                                                                                                                                                                                               |
| `auth_success`               | La autenticación se completa                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `elicitation_dialog`         | Un servidor MCP abre un formulario de elicitación y no has escrito durante aproximadamente seis segundos                                                                                                                                                                                                                                                                                                                                                                                   |
| `elicitation_url_dialog`     | Un servidor MCP te pide que abras una URL en el navegador y no has escrito durante aproximadamente seis segundos                                                                                                                                                                                                                                                                                                                                                                           |
| `elicitation_complete`       | Un servidor MCP reporta que una [elicitación en modo URL](/docs/es/hooks#elicitation-input) está completa                                                                                                                                                                                                                                                                                                                                                                                       |
| `elicitation_response`       | Una respuesta de elicitación de MCP se envía de vuelta al servidor                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `agent_needs_input`          | Una sesión en segundo plano comienza a esperar tu entrada mientras la [vista de agente](/docs/es/agent-view) está abierta, o la sesión actual te hace una [pregunta de configuración de terminal de un compañero de equipo de agente](/docs/es/agent-teams#choose-a-display-mode) y no has escrito durante aproximadamente seis segundos                                                                                                                                                             |
| `agent_completed`            | Una sesión en segundo plano se completa o falla. Se activa solo mientras la [vista de agente](/docs/es/agent-view) está abierta                                                                                                                                                                                                                                                                                                                                                                 |
| `quota_auto_resume_fired`    | Claude Code continúa tu tarea después de que un límite de uso de claude.ai la pausó: en el reinicio, o antes cuando algo que haces en Claude Code durante la espera, como añadir créditos de uso, actualizar tu plan o cambiar modelos, hace que el uso esté disponible de nuevo, con la [excepción de configuración de modelo](/docs/es/interactive-mode#wait-for-a-usage-limit-to-reset)                                                                                                      |
| `quota_auto_resume_stale`    | Un límite de uso de claude.ai se reinició mientras tu computadora dormía durante más de aproximadamente 30 minutos. Claude Code espera a que presiones `Enter` en lugar de continuar. Después de un sueño más corto continúa y activa `quota_auto_resume_fired` en su lugar                                                                                                                                                                                                                |
| `quota_auto_resume_disabled` | Claude Code termina su espera por un límite de uso de claude.ai sin continuar tu tarea: [`autoContinueAtUsageLimit`](/docs/es/settings-reference#autocontinueatusagelimit) se desactivó o el reinicio se movió más de 24 horas durante una espera que Claude Code inició por sí solo, la tarea continuada siguió golpeando el límite, o la continuación fue bloqueada antes de llegar al modelo. No se activa cuando presionas `Esc` o `Ctrl+C`, o seleccionas **No continuar automáticamente** |

Claude Code cronometra `permission_prompt` de manera diferente en una terminal y en Claude Desktop, la extensión de VS Code y otros hosts que responden solicitudes de permiso a través del Agent SDK. Consulta [cuándo se activa cada tipo de notificación](/docs/es/hooks#notification) para ambos tiempos.

Los matchers `agent_needs_input` y `agent_completed` requieren Claude Code v2.1.198 o posterior.

Los matchers `quota_auto_resume_fired`, `quota_auto_resume_stale` y `quota_auto_resume_disabled` requieren Claude Code v2.1.234 o posterior.

En sesiones de terminal, `permission_prompt` para una solicitud de red de un comando en sandbox requiere Claude Code v2.1.246 o posterior.

`agent_needs_input` para una pregunta de configuración de terminal de un compañero de equipo requiere Claude Code v2.1.248 o posterior.

Escribe `/hooks` y selecciona `Notification` para confirmar que el hook está registrado. Para el esquema de evento completo, consulta la [referencia de Notification](/docs/es/hooks#notification).

<h3 id="auto-format-code-after-edits">
  Formatea automáticamente el código después de ediciones
</h3>

Ejecuta automáticamente [Prettier](https://prettier.io/) en cada archivo que Claude edita, para que el formato se mantenga consistente sin intervención manual.

Este hook usa el evento `PostToolUse` con un matcher `Edit|Write`, por lo que se ejecuta solo después de herramientas de edición de archivos. El comando extrae la ruta del archivo editado con [`jq`](https://jqlang.org/) y la pasa a Prettier. Añade esto a `.claude/settings.json` en la raíz de tu proyecto:

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

Para probar el hook, pídele a Claude que añada una línea con cadenas entre comillas simples a un archivo JavaScript, luego abre el archivo: con la configuración predeterminada de Prettier, el hook las reescribe a comillas dobles.

Cuando el hook tiene éxito, Claude Code no muestra nada en la conversación. Para confirmar que el hook se ejecutó, verifica que el archivo editado esté reformateado, o consulta [Técnicas de depuración](#debug-techniques).

Para reformatear un archivo específico sin importar cómo cambie, incluyendo cuando un comando `Bash` lo reescribe, usa un hook [FileChanged](/docs/es/hooks#filechanged) en su lugar.

<Note>
  Los ejemplos de Bash en esta página usan `jq` para análisis JSON. Instálalo con `brew install jq` en macOS, `apt-get install jq` en Debian y Ubuntu, o consulta [descargas de `jq`](https://jqlang.org/download/).
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

    # Normaliza separadores de barra invertida de Windows para que los patrones a continuación coincidan
    FILE_PATH="${FILE_PATH//\\//}"

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

  <Step title="Prueba el hook">
    Pídele a Claude que añada un comentario a tu archivo `.env`. Claude Code bloquea la edición antes de que se ejecute y pasa el mensaje `Blocked:` del script a Claude como retroalimentación.
  </Step>
</Steps>

<h3 id="re-inject-context-after-compaction">
  Reinyecta contexto después de compactación
</h3>

Cuando la ventana de contexto de Claude se llena, la compactación resume la conversación para liberar espacio. Esto puede perder detalles importantes. Usa un hook `SessionStart` con un matcher `compact` para reinyectar contexto crítico después de cada compactación.

Claude Code añade texto plano que tu comando escribe en stdout al contexto de Claude. Este ejemplo recuerda a Claude las convenciones del proyecto y el trabajo reciente. Añade esto a `.claude/settings.json` en la raíz de tu proyecto:

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

Para confirmar que el hook registra cambios, edita un archivo de configuración en otro editor mientras una sesión está en ejecución, luego abre `~/claude-config-audit.log`: el hook añade una línea JSON por cambio con la marca de tiempo, fuente y ruta del archivo.

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

A diferencia de los ejemplos de código de salida anteriores, la aprobación automática requiere que tu hook escriba una decisión JSON en stdout. Claude Code ejecuta hooks `PermissionRequest` cuando está a punto de pedirte permiso, y si tu hook devuelve `"behavior": "allow"`, Claude Code responde la solicitud en tu nombre.

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
  `bypassPermissions` solo se aplica si iniciaste la sesión con modo bypass ya disponible: `--dangerously-skip-permissions`, `--permission-mode bypassPermissions`, `--allow-dangerously-skip-permissions`, o `permissions.defaultMode: "bypassPermissions"` en [configuración de usuario, `--settings`, o configuración administrada](/docs/es/settings-reference#permissions-defaultmode). No se aplica si el modo bypass está deshabilitado por [`permissions.disableBypassPermissionsMode`](/docs/es/permissions#managed-settings), o si iniciaste la sesión en [modo restringido](/docs/es/cli-reference#cli-flags).

  Claude Code nunca lo guarda como `defaultMode`.
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

Mantén el matcher lo más estrecho posible. Coincidir con `.*` o dejar el matcher vacío aprobaría automáticamente cada aviso de permiso de herramienta, incluyendo escrituras de archivos y comandos de shell. Consulta la [referencia de PermissionRequest](/docs/es/hooks#permissionrequest-decision-control) para el conjunto completo de campos de decisión.

<h2 id="how-hooks-work">
  Cómo funcionan los hooks
</h2>

Claude Code dispara eventos de hook en puntos específicos de su ciclo de vida. Cuando se dispara un evento, Claude Code ejecuta todos los hooks coincidentes en paralelo; consulta [Campos del manejador de hooks](/docs/es/hooks#hook-handler-fields) para saber cómo se tratan los manejadores duplicados. La tabla a continuación muestra cada evento y cuándo se dispara:

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

Cada evento incluye campos comunes como `session_id`, un ID único para la sesión, y `cwd`, el directorio de trabajo cuando se disparó el evento, pero cada tipo de evento añade datos diferentes. Cuando Claude ejecuta un comando Bash, un hook `PreToolUse` recibe estos campos en stdin:

* `hook_event_name`: el evento que activó el hook
* `tool_name`: la herramienta que Claude está a punto de usar
* `tool_input`: los argumentos que Claude pasó a la herramienta. Para Bash, su campo `command` contiene el comando de shell.

Por ejemplo, la entrada del hook para un comando `npm test` se ve así:

```json theme={null}
{
  "session_id": "abc123",
  "cwd": "/Users/sarah/myproject",
  "hook_event_name": "PreToolUse",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test"
  }
}
```

Tu script puede analizar ese JSON y actuar sobre cualquiera de esos campos. Los hooks `UserPromptSubmit` obtienen el texto `prompt` en su lugar, los hooks `SessionStart` obtienen una `source` de `startup`, `resume`, `clear`, `compact`, o `fork`, y así sucesivamente. Consulta [Campos de entrada comunes](/docs/es/hooks#common-input-fields) en la referencia para campos compartidos, y la sección de cada evento para esquemas específicos del evento.

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

* **Exit 0**: tu hook reporta sin objeción a través de su código de salida.
  * Para un hook `PreToolUse` esto no aprueba la llamada a herramienta: el [flujo de permiso](/docs/es/permissions) normal aún se aplica.
  * Para hooks `UserPromptSubmit`, `UserPromptExpansion`, `SessionStart`, y `PostModelSwitch`, Claude Code añade stdout que [trata como texto plano](/docs/es/hooks#exit-code-0) al contexto de Claude.
* **Exit 2**: Claude Code bloquea la acción. Escribe una razón en stderr. Dónde llega depende del evento: algunos eventos lo alimentan a Claude como retroalimentación para que pueda ajustar, otros lo muestran al usuario, y algunos, como `ConfigChange` y `Elicitation`, no muestran ningún mensaje. Algunos eventos no pueden ser bloqueados: para `SessionStart` y otros, exit 2 muestra stderr al usuario y la ejecución continúa. Consulta [comportamiento del código de salida 2 por evento](/docs/es/hooks#exit-code-2-behavior-per-event) para la lista completa.
* **Cualquier otro código de salida**: para la mayoría de eventos, el resultado depende de lo que tu hook imprimió en stdout:
  * Un objeto analizado que pasa validación de esquema: Claude Code ignora el código de salida, el JSON solo decide el resultado, y el hook no se reporta como un error. Las excepciones por evento, como `WorktreeCreate` fallando en cualquier salida distinta de cero, se enumeran en la sección [Salida del código de salida](/docs/es/hooks#exit-code-output) de la referencia.
  * Un objeto analizado que falla validación de esquema, o stdout que Claude Code [intenta analizar como JSON](/docs/es/hooks#exit-code-0) pero que no es JSON válido: un error sin bloqueo; el aviso lleva el mensaje de validación o análisis.
  * Stdout que Claude Code [trata como texto plano](/docs/es/hooks#exit-code-0), o stdout vacío: la acción procede como un error sin bloqueo. La transcripción muestra un aviso `<hook name> hook error`, luego la primera línea de stderr prefijada con `Failed with non-blocking status code:`. Para capturar el stderr completo, habilita [registro de depuración](/docs/es/hooks#debug-hooks) con `claude --debug` o ejecutando `/debug` en medio de la sesión.

<h4 id="structured-json-output">
  Salida JSON estructurada
</h4>

Los códigos de salida solo te permiten bloquear o permanecer en silencio. Para más control, sal con 0 e imprime un objeto JSON a stdout en su lugar.

<Note>
  Usa exit 2 para bloquear con un mensaje de stderr, o exit 0 con JSON para control estructurado. Elige un enfoque por hook. Para lo que sucede cuando los mezclas, consulta [Salida del código de salida](/docs/es/hooks#exit-code-output).
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

Con `"deny"`, Claude Code cancela la llamada a herramienta y alimenta `permissionDecisionReason` de vuelta a Claude.

En `PreToolUse`, Claude Code maneja cada valor `permissionDecision` de la siguiente manera:

* `"allow"`: omite el aviso de permiso interactivo. Las reglas de negación y solicitud, incluyendo listas de negación gestionadas empresariales, aún se aplican, al igual que los avisos para herramientas MCP marcadas [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool) y para herramientas de conector [que tu organización configuró como `ask`](/docs/es/mcp#organization-controls-on-connector-tools) en sesiones donde esa configuración llega a Claude Code
* `"deny"`: cancela la llamada a herramienta y envía la razón a Claude
* `"ask"`: muestra el aviso de permiso al usuario como es normal

Un cuarto valor, `"defer"`, está disponible en [modo no interactivo](/docs/es/headless) con la bandera `-p`. Sale del proceso con la llamada a herramienta preservada para que un envoltorio del SDK del Agente pueda recopilar entrada y reanudar. Consulta [Defer a tool call for later](/docs/es/hooks#defer-a-tool-call-for-later) en la referencia.

Un hook `PreModelSwitch` devuelve el mismo campo `permissionDecision`: `"allow"` permite que un cambio de modelo proceda, y `"deny"` lo cancela. `"ask"` te hace confirmar el cambio cuando ejecutas `/model` en una sesión interactiva; en cualquier otro lugar, Claude Code trata `"ask"` como un rechazo. Consulta [PreModelSwitch decision control](/docs/es/hooks#premodelswitch-decision-control).

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

El matcher `"Edit|Write"` se activa solo cuando Claude usa la herramienta `Edit` o `Write`, no cuando usa `Bash`, `Read`, u otra herramienta. Una coma separa alternativas de la misma manera, por lo que `"Edit, Write"` es equivalente. Consulta [Matcher patterns](/docs/es/hooks#matcher-patterns) para cómo se evalúan los nombres simples y las expresiones regulares.

<Note>
  Claude también puede crear o modificar archivos ejecutando comandos de shell. Si tu hook debe ver cada cambio de archivo, como para escaneo de cumplimiento o registro de auditoría, añade un hook [`Stop`](/docs/es/hooks#stop) que escanee el árbol de trabajo una vez por turno. Para cobertura por llamada en su lugar, también coincide con `Bash|PowerShell` y haz que tu script liste archivos modificados y sin seguimiento con `git status --porcelain`. La sección [Entrada del hook PowerShell](/docs/es/hooks#powershell) explica por qué coincidir solo con `Bash` no es suficiente. Para ejecutar un hook cuando un archivo específico cambia en disco, sin importar qué lo escribió, usa un hook [FileChanged](/docs/es/hooks#filechanged).
</Note>

Cada tipo de evento coincide en un campo específico:

| Evento                                                                                                                                                          | En qué filtra el matcher                                                                                           | Valores de matcher de ejemplo                                                                                                                                                                                                                                                  |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`                                                                      | nombre de herramienta                                                                                              | `Bash`, `Edit\|Write`, `mcp__.*`                                                                                                                                                                                                                                               |
| `SessionStart`                                                                                                                                                  | cómo comenzó la sesión                                                                                             | `startup`, `resume`, `clear`, `compact`, `fork`                                                                                                                                                                                                                                |
| `Setup`                                                                                                                                                         | qué bandera CLI activó la configuración                                                                            | `init`, `maintenance`                                                                                                                                                                                                                                                          |
| `SessionEnd`                                                                                                                                                    | por qué terminó la sesión                                                                                          | `clear`, `resume`, `logout`, `prompt_input_exit`, `other`                                                                                                                                                                                                                      |
| `Notification`                                                                                                                                                  | tipo de notificación                                                                                               | `permission_prompt`, `idle_prompt`, `auth_success`, `elicitation_dialog`, `elicitation_url_dialog`, `elicitation_complete`, `elicitation_response`, `agent_needs_input`, `agent_completed`, `quota_auto_resume_fired`, `quota_auto_resume_stale`, `quota_auto_resume_disabled` |
| `SubagentStart`                                                                                                                                                 | tipo de agente                                                                                                     | `general-purpose`, `Explore`, `Plan`, o nombres de agentes personalizados                                                                                                                                                                                                      |
| `PreCompact`, `PostCompact`                                                                                                                                     | qué activó la compactación                                                                                         | `manual`, `auto`                                                                                                                                                                                                                                                               |
| `PreModelSwitch`, `PostModelSwitch`                                                                                                                             | nombre canónico del modelo al que la sesión cambia, como se describe en [PreModelSwitch](/docs/es/hooks#premodelswitch) | `claude-opus-5`, `claude-opus-4-6\|claude-opus-5`, `.*opus.*`                                                                                                                                                                                                                  |
| `SubagentStop`                                                                                                                                                  | tipo de agente                                                                                                     | los mismos valores que `SubagentStart`                                                                                                                                                                                                                                         |
| `ConfigChange`                                                                                                                                                  | fuente de configuración                                                                                            | `user_settings`, `project_settings`, `local_settings`, `policy_settings`, `skills`                                                                                                                                                                                             |
| `DirectoryAdded`                                                                                                                                                | cómo se añadió el directorio                                                                                       | `slash_command`, `register_repo_root`                                                                                                                                                                                                                                          |
| `StopFailure`                                                                                                                                                   | tipo de error                                                                                                      | `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `account_on_hold`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `cloud_credential_error`, `unknown`                                               |
| `InstructionsLoaded`                                                                                                                                            | razón de carga                                                                                                     | `session_start`, `nested_traversal`, `path_glob_match`, `include`, `compact`                                                                                                                                                                                                   |
| `Elicitation`                                                                                                                                                   | nombre del servidor MCP                                                                                            | tus nombres de servidor MCP configurados                                                                                                                                                                                                                                       |
| `ElicitationResult`                                                                                                                                             | nombre del servidor MCP                                                                                            | los mismos valores que `Elicitation`                                                                                                                                                                                                                                           |
| `FileChanged`                                                                                                                                                   | nombres de archivo literales a observar (consulta [FileChanged](/docs/es/hooks#filechanged))                            | `.envrc\|.env`                                                                                                                                                                                                                                                                 |
| `UserPromptExpansion`                                                                                                                                           | nombre del comando                                                                                                 | tus nombres de skill o comando                                                                                                                                                                                                                                                 |
| `UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `CwdChanged`, `MessageDisplay` | sin soporte de matcher                                                                                             | siempre se activa en cada ocurrencia                                                                                                                                                                                                                                           |

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

Cuando Claude Code no puede determinar qué comandos ejecuta la entrada Bash, ejecuta tu hook independientemente del patrón. La [tabla de coincidencia Bash](/docs/es/hooks#bash-if-matching) cubre las formas de comando que Claude Code puede y no puede estrecharse por subcomando. Porque el filtro es mejor esfuerzo, usa el [sistema de permisos](/docs/es/permissions) en lugar de un hook para aplicar un allow o deny duro.

El campo `if` acepta los mismos patrones que las reglas de permiso: `"Bash(git *)"`, `"Edit(*.ts)"`, y así sucesivamente. Para coincidir con múltiples nombres de herramienta, usa manejadores separados cada uno con su propio valor `if`, o coincide a nivel de `matcher` donde se soporta alternancia de tuberías.

`if` solo funciona en eventos de herramienta: `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, y `PermissionDenied`. Añadirlo a cualquier otro evento evita que el hook se ejecute.

<h3 id="configure-hook-location">
  Configura la ubicación del hook
</h3>

Dónde añadas un hook determina su ámbito:

| Ubicación                                         | Ámbito                                                                                                                            | Compartible                                                      |
| :------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------- |
| `~/.claude/settings.json`                         | Todos tus proyectos                                                                                                               | No, local a tu máquina                                           |
| `.claude/settings.json`                           | Proyecto único                                                                                                                    | Sí, puede ser confirmado en el repositorio                       |
| `.claude/settings.local.json`                     | Proyecto único                                                                                                                    | No, gitignored cuando Claude Code guarda una configuración en él |
| Configuración de política gestionada              | Organización completa                                                                                                             | Sí, controlado por administrador                                 |
| [Plugin](/docs/es/plugins/overview) `hooks/hooks.json` | Cuando el plugin está habilitado                                                                                                  | Sí, incluido con el plugin                                       |
| [Skill](/docs/es/skills) frontmatter                   | El resto de la sesión una vez que el skill se invoca. Consulta [Hooks in skills and agents](/docs/es/hooks#hooks-in-skills-and-agents) | Sí, definido en el archivo del skill                             |
| [Subagent](/docs/es/sub-agents) frontmatter            | Mientras ese subagente está ejecutándose                                                                                          | Sí, definido en el archivo del subagente                         |

Ejecuta [`/hooks`](/docs/es/hooks#the-%2Fhooks-menu) en Claude Code para examinar todos los hooks configurados agrupados por evento.

Para desactivar hooks, establece `"disableAllHooks": true` en tu archivo de configuración. Claude Code lee el valor que queda después de que se aplica la [precedencia de configuración](/docs/es/hooks#disable-or-remove-hooks), por lo que el archivo de configuración de un proyecto puede anular el tuyo. Los hooks configurados en configuración gestionada aún se ejecutan a menos que `disableAllHooks` también esté establecido allí. Para el alcance completo de cada nivel, consulta [`disableAllHooks`](/docs/es/settings-reference#disableallhooks).

Si editas archivos de configuración directamente mientras Claude Code está ejecutándose, el observador de archivos normalmente recoge cambios de hooks automáticamente.

<h2 id="prompt-based-hooks">
  Hooks basados en prompts
</h2>

Para decisiones que requieren criterio en lugar de reglas deterministas, usa hooks `type: "prompt"`. En lugar de ejecutar un comando de shell, Claude Code envía tu prompt y los datos de entrada del hook a un modelo Claude (Haiku por defecto) para tomar la decisión. Puedes especificar un modelo diferente con el campo `model` si necesitas más capacidad.

El único trabajo del modelo es devolver su decisión como JSON:

* `"ok": true`: la acción procede
* `"ok": false`: lo que sucede depende del evento:
  * `Stop` y `SubagentStop`: la `reason` se alimenta de vuelta a Claude para que siga trabajando, a menos que la respuesta también establezca `"impossible": true` para marcar la condición como una que nunca puede ser satisfecha, en cuyo caso Claude Code permite la parada y el turno termina
  * `PreToolUse`: la llamada de herramienta se deniega; por defecto el turno termina y la `reason` de denegación aparece en el chat como una línea de advertencia. Establece `continueOnBlock: true` en el hook para devolver la `reason` a Claude como el error de la herramienta, para que pueda ajustarse y continuar. Antes de v2.1.210, la `reason` de denegación se devolvía a Claude como el error de la herramienta y el turno continuaba
  * `PostToolUse`: por defecto el turno termina y la `reason` aparece en el chat como una línea de advertencia. Establece `continueOnBlock: true` para alimentar la `reason` de vuelta a Claude y continuar el turno en su lugar
  * `PostToolBatch`, `UserPromptSubmit` y `UserPromptExpansion`: el turno termina y la `reason` aparece en el chat como una línea de advertencia

Este ejemplo usa un hook `Stop` para preguntarle al modelo si todas las tareas solicitadas están completas. Si el modelo devuelve `"ok": false` porque la condición aún no se cumple, Claude sigue trabajando y usa la `reason` como su siguiente instrucción:

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

Los hooks de agente usan el formato de respuesta `"ok"` / `"reason"` con un tiempo de espera predeterminado más largo de 60 segundos y hasta 50 turnos de uso de herramientas. No admiten el campo `impossible` de los hooks de prompt. En `ok: false`, Claude Code maneja un hook de agente de la misma manera que maneja un hook de prompt con `continueOnBlock: true` en el mismo evento, por lo que en `PreToolUse` y `PostToolUse` el turno continúa; los hooks de agente no tienen campo `continueOnBlock`. Consulta [configuración de hooks de agente](/docs/es/hooks#agent-hook-configuration) para los campos, incluido el marcador de posición `$ARGUMENTS` que Claude Code reemplaza con la entrada JSON del hook.

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
  * `command`, `http`, `mcp_tool`: 10 minutos. Claude Code reduce estos valores predeterminados a 30 segundos para los hooks `UserPromptSubmit`, `PreModelSwitch` y `PostModelSwitch`, y a 10 segundos para `MessageDisplay`.
  * `prompt`: 30 segundos.
  * `agent`: 60 segundos.
  * Los hooks [`SessionEnd`](/docs/es/hooks#sessionend) de cualquier tipo comparten un presupuesto de 1,5 segundos. Si su configuración establece un `timeout` por hook más largo, Claude Code aumenta el presupuesto para que coincida, hasta 60 segundos.
* Los hooks `PostToolUse` no pueden deshacer acciones ya que la herramienta ya se ha ejecutado.
* Los hooks `PermissionRequest` se activan cuando Claude Code está a punto de pedirle permiso.
  * En [modo no interactivo](/docs/es/headless) con la bandera `-p`, esa solicitud solo existe cuando la devolución de llamada [`canUseTool`](/docs/es/agent-sdk/permissions) del SDK del Agent la proporciona. En ejecuciones simples de `-p` o con `--permission-prompt-tool`, use hooks `PreToolUse` para decisiones de permiso automatizadas en su lugar.
  * Los subagentes de fondo no pueden mostrar una solicitud en modo no interactivo. Claude Code aún ejecuta los hooks para sus llamadas a herramientas, y si ningún hook devuelve una decisión, deniega la llamada. En una sesión interactiva, las solicitudes de subagentes de fondo aparecen en su sesión principal y los hooks se activan como de costumbre.
* Los hooks `Stop` se activan cada vez que Claude termina de responder, no solo en la finalización de tareas. No se activan en interrupciones del usuario. Los errores de API activan [StopFailure](/docs/es/hooks#stopfailure) en su lugar.
* Cuando múltiples hooks `PreToolUse` devuelven [`updatedInput`](/docs/es/hooks#pretooluse) para reescribir los argumentos de una herramienta, el último en terminar gana. Como los hooks se ejecutan en paralelo, el orden es no determinista. Evite tener más de un hook modificando la entrada de la misma herramienta.

<h3 id="hooks-and-permission-modes">
  Hooks y modos de permiso
</h3>

Los hooks `PreToolUse` se activan antes de cualquier verificación de modo de permiso, en cada [modo de permiso](/docs/es/permission-modes), incluyendo `dontAsk`. Un hook que devuelve `permissionDecision: "deny"` bloquea la herramienta incluso en modo `bypassPermissions` o con `--dangerously-skip-permissions`. Esto le permite aplicar política que los usuarios no pueden eludir cambiando su modo de permiso.

Lo inverso no es cierto: un hook que devuelve `"allow"` no elude reglas de negación de configuración, y no puede suprimir la solicitud de herramientas MCP marcadas [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool) o de herramientas conectoras [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools) en sesiones donde esa configuración llega a Claude Code. Los hooks pueden endurecer restricciones pero no relajarlas más allá de lo que las reglas de permiso permiten.

<h3 id="hook-not-firing">
  Hook no se activa
</h3>

El hook está configurado pero nunca se ejecuta.

* Ejecuta `/hooks` y confirma que el hook aparece bajo el evento correcto
* Verifique que el patrón del matcher coincida exactamente con el nombre de la herramienta. Los matchers distinguen mayúsculas de minúsculas
* Verifique que esté activando el tipo de evento correcto: `PreToolUse` se activa antes de la ejecución de la herramienta, `PostToolUse` se activa después. Un hook `PermissionRequest` se activa cuando Claude Code está a punto de pedirle permiso; consulte las [limitaciones](#limitations) para los casos no interactivos

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
* Si el aviso muestra un mensaje de validación JSON, la salida estándar de su hook se analizó como JSON pero falló la validación del esquema. Si muestra un mensaje de análisis JSON, la salida estándar parecía un objeto JSON pero no era JSON válido. Ambos suceden incluso en salida 0.

  Para corregir una falla de análisis, construya la carga útil con un codificador JSON como `jq` en lugar de concatenación de cadenas, para que las comillas y barras invertidas dentro de los valores se escapen. La sección [Exit code output](/docs/es/hooks#exit-code-output) de la referencia cubre las combinaciones de código de salida y JSON
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

<h3 id="hook-json-has-no-effect">
  Hook JSON no tiene efecto
</h3>

Su hook imprime JSON válido, pero la decisión no surte efecto y no aparece ningún error en la transcripción. Verifique cuál es la causa que se aplica:

* **Salida adicional antes del JSON**: algo más escribe en stdout primero, generalmente un `echo` incondicional en su perfil de shell, por lo que la salida ya no comienza con `{` y Claude Code no la analiza como JSON. La causa y la solución siguen esta lista.
* **Un campo en el nivel incorrecto**: compare la ubicación de cada campo con el formato [JSON output](/docs/es/hooks#json-output). Por ejemplo, `permissionDecision` pertenece dentro de `hookSpecificOutput`, no en el nivel superior.

Cuando Claude Code ejecuta un hook de comando en forma de shell, uno sin `args`, genera `sh -c` en macOS y Linux, Git Bash en Windows, o PowerShell cuando Git Bash no está instalado por defecto. Este shell es no interactivo, pero Git Bash y algunas configuraciones, como `BASH_ENV` apuntando a `~/.bashrc`, aún obtienen su perfil. Si ese perfil contiene declaraciones `echo` incondicionales, la salida se antepone a su JSON del hook:

```text theme={null}
Shell ready on arm64
{"decision": "block", "reason": "Not allowed"}
```

La salida combinada ya no comienza con `{`, por lo que Claude Code trata toda la salida estándar como texto plano e ignora el JSON. En salida 0 nada se reporta en la transcripción; el intento de análisis se registra solo en el [registro de depuración](/docs/es/hooks#debug-hooks). Para corregir esto, envuelva las declaraciones echo en su perfil de shell para que solo se ejecuten en shells interactivos:

```bash theme={null}
# En ~/.zshrc o ~/.bashrc
if [[ $- == *i* ]]; then
  echo "Shell ready"
fi
```

La variable `$-` contiene banderas de shell, e `i` significa interactivo. Los hooks se ejecutan en shells no interactivos, por lo que el echo se omite.

Cuando su hook devuelve `permissionDecision` o `additionalContext` en el nivel superior en lugar de dentro de `hookSpecificOutput`, el JSON aún se analiza, y Claude Code ignora los campos mal colocados sin reportar un error. Para ver qué campos ignoró, inicie Claude Code con `claude --debug` y busque en el [registro de depuración](/docs/es/hooks#debug-hooks) `Hook JSON output had unrecognized keys`.

<h3 id="debug-techniques">
  Técnicas de depuración
</h3>

Presione `Ctrl+O` para abrir la vista de transcripción para verificar el resultado de una ejecución de hook:

* **Ejecución exitosa**: no ve nada, a menos que el JSON del hook muestre algo, como `systemMessage` o retroalimentación del hook Stop.
  * Para confirmar que un hook se ejecutó, verifique su efecto, como un archivo reformateado, o active el registro de depuración como se describe a continuación y active el hook nuevamente
* **Error de bloqueo**: en la mayoría de eventos ve la retroalimentación del hook. Cuando el JSON del hook tomó una decisión de bloqueo, la retroalimentación es la razón de esa decisión; de lo contrario es el stderr del hook. En algunos eventos, como `ConfigChange` y `Elicitation`, un bloqueo no muestra ningún mensaje.
* **Error sin bloqueo**: la acción procedió, y ve un aviso `<hook name> hook error` con una explicación breve, como la primera línea de stderr prefijada con `Failed with non-blocking status code:`, o un mensaje de validación o análisis JSON.

Qué combinaciones de código de salida y JSON producen cada resultado, incluyendo las excepciones por evento, se define en la sección [Exit code output](/docs/es/hooks#exit-code-output) de la referencia.

Para detalles de ejecución completos incluyendo qué hooks coincidieron, sus códigos de salida, stdout y stderr, lee el registro de depuración. Inicia Claude Code con `claude --debug-file /tmp/claude.log` para escribir en una ruta conocida, luego `tail -f /tmp/claude.log` en otra terminal. Si iniciaste sin esa bandera, ejecuta `/debug` a mitad de sesión para habilitar el registro y encontrar la ruta del registro.

<h2 id="learn-more">
  Aprende más
</h2>

* [Referencia de Hooks](/docs/es/hooks): esquemas de eventos completos, formato de salida JSON, hooks asincronos y hooks de herramientas MCP
* [Consideraciones de seguridad](/docs/es/hooks#security-considerations): revisa antes de desplegar hooks en entornos compartidos o de producción
* [Ejemplo de validador de comandos Bash](https://github.com/anthropics/claude-code/blob/main/examples/hooks/bash_command_validator_example.py): implementación de referencia completa
