> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Elegir un modo de permisos

> Controle si Claude solicita aprobación antes de actuar. Cambie de modo de permisos con Mayús+Tab en la CLI, el indicador de modo en VS Code, o el selector de modo en Desktop.

Un modo de permisos establece qué acciones puede realizar Claude en una sesión sin pedirle permiso primero. En modo Manual, Claude Code se detiene y le solicita aprobación antes de la mayoría de acciones que editan archivos, ejecutan comandos de shell o alcanzan la red. En [modo automático](#eliminate-prompts-with-auto-mode), un segundo modelo, el clasificador, revisa acciones en su lugar; [cómo el clasificador evalúa acciones](#how-the-classifier-evaluates-actions) enumera qué acciones revisa y cuáles se omiten.

En planes Pro, Max y Team, el modo de permisos inicial integrado es modo automático. [Qué modo inicia una sesión](#which-mode-a-session-starts-in) cubre las superficies y configuraciones que cambian el modo de permisos inicial. También puede cambiar el modo de permisos de una sesión en ejecución en cualquier momento.

<h2 id="available-modes">
  Modos disponibles
</h2>

Cada modo hace un compromiso diferente entre conveniencia y supervisión. La tabla a continuación muestra qué puede hacer Claude sin un aviso de permiso en cada modo. El modo manual aparece bajo su valor de configuración, `default`.

| Modo                                                                | Qué se ejecuta sin preguntar                                                                                                  | Mejor para                                        |
| :------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------ |
| `default`                                                           | Solo lecturas                                                                                                                 | Revisar cada acción usted mismo, trabajo sensible |
| [`acceptEdits`](#auto-approve-file-edits-with-acceptedits-mode)     | Lecturas, ediciones de archivos y comandos comunes del sistema de archivos (`mkdir`, `touch`, `mv`, `cp`, etc.)               | Iterar sobre código que está revisando            |
| [`plan`](#analyze-before-you-edit-with-plan-mode)                   | Lecturas, más comandos aprobados por clasificador cuando [modo automático](#eliminate-prompts-with-auto-mode) está disponible | Explorar una base de código antes de cambiarla    |
| [`auto`](#eliminate-prompts-with-auto-mode)                         | Todo, con verificaciones de seguridad en segundo plano                                                                        | Tareas largas, reducir fatiga de avisos           |
| [`dontAsk`](#allow-only-pre-approved-tools-with-dontask-mode)       | Lecturas y herramientas preaprobadas; cualquier cosa que generaría un aviso se deniega                                        | CI bloqueado y scripts                            |
| [`bypassPermissions`](#skip-all-checks-with-bypasspermissions-mode) | Todo                                                                                                                          | Solo contenedores aislados y máquinas virtuales   |

El modo que revisa cada acción se llama **Manual** en la CLI, en `claude --help`, en las extensiones de VS Code y JetBrains, y en la aplicación de escritorio. Su valor de configuración es `default`, que es lo que usan los hooks e integraciones de SDK. La CLI acepta `manual` como un alias dondequiera que escriba el valor, por ejemplo `claude --permission-mode manual` o `"defaultMode": "manual"`. La etiqueta Manual y el alias `manual` requieren Claude Code v2.1.200 o posterior. La etiqueta de la aplicación de escritorio no depende de su versión de CLI.

Las escrituras en [rutas protegidas](#protected-paths) nunca se aprueban automáticamente excepto en modo `bypassPermissions` y en sesiones de modo plan donde los permisos de omisión están disponibles, lo que significa sesiones de terminal interactivas iniciadas de una manera que [pone `bypassPermissions` en el ciclo de modo](#switch-permission-modes).

Los modos establecen la línea base. Superponga [reglas de permiso](/docs/es/permissions#manage-permissions) encima para preaprobación o bloqueo de herramientas específicas. Las reglas de denegación bloquean en cada modo, incluido `bypassPermissions`. Las reglas de denegación y pregunta no se aplican a [`EndConversation`](/docs/es/tools-reference#endconversation-tool-behavior) siempre que Claude aún tenga al menos otra herramienta que pueda llamar. Las reglas de permiso no tienen efecto en `bypassPermissions`.

<h3 id="actions-no-mode-auto-approves">
  Acciones que ningún modo aprueba automáticamente
</h3>

Claude Code no aprueba automáticamente lo siguiente en ningún modo, incluido `bypassPermissions`. Cada viñeta enlaza a la sección que dice qué sucede en su lugar en cada modo:

* Herramientas coincidentes por una [regla de pregunta](/docs/es/permissions#manage-permissions) explícita
* Herramientas de conector que su organización [estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools), en sesiones donde esa configuración llega a Claude Code
* Herramientas que requieren interacción del usuario: la herramienta integrada `AskUserQuestion` y herramientas MCP marcadas [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool)
* Eliminaciones de `rm` y `rmdir` dirigidas a una [ruta crítica](#critical-paths), que ninguna regla de permiso o hook `PreToolUse` `"allow"` aprueba
* Las [salvaguardas de mensajería entre sesiones](#skip-all-checks-with-bypasspermissions-mode)
* Lecturas fuera de los directorios de trabajo mientras [`permissions.blockReadsOutsideWorkingDirectories`](/docs/es/settings-reference#permissions-blockreadsoutsideworkingdirectories) está activado: los comandos Bash reconocidos de lectura de archivos generan avisos incluso en modo automático y modo `bypassPermissions`, y también lo hace cualquier [reintento sin sandbox](/docs/es/sandboxing#the-unsandboxed-retry-escape-hatch) que necesita aprobación para ejecutarse fuera del sandbox. Requiere Claude Code v2.1.257 o posterior.

  Un comando que el analizador de shell no puede rastrear, como uno que cambia de directorio más de una vez o ejecuta un subshell, genera avisos de la misma manera incluso cuando no nombra ninguna ruta externa. Este aviso no se aplica cuando el comando se ejecuta en el [sandbox](/docs/es/sandboxing) y el sandbox aplica el bloqueo.

<h2 id="common-setups">
  Configuraciones comunes
</h2>

Los modos de permisos deciden si Claude solicita antes de una acción, y el [sandbox Bash](/docs/es/sandboxing) y los [límites de aislamiento](/docs/es/sandbox-environments) externos deciden qué puede alcanzar una acción una vez que se ejecuta. Cada fila a continuación empareja un objetivo con las banderas o configuraciones que lo logran y el aislamiento que necesita, como punto de partida. [Modos disponibles](#available-modes) enumera qué se ejecuta sin un aviso en cada modo.

| Usted quiere                                               | Comience con                                                                                                                                                                                  | Aislamiento necesario                                                                                                                                                                                                           | Notas                                                                                                                                                                                                                                                                                      |
| :--------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Revisar cada acción usted mismo                            | Modo Manual: `claude --permission-mode default`                                                                                                                                               | Ninguno                                                                                                                                                                                                                         | Trabajo sensible, código desconocido                                                                                                                                                                                                                                                       |
| Iterar localmente con menos avisos, sin un clasificador    | Modo Manual más el sandbox Bash en [modo de permitir automático](/docs/es/sandboxing#sandbox-modes): `claude --permission-mode default`, luego ejecute `/sandbox` y seleccione permitir automático | El sandbox Bash integrado, en macOS, Linux y WSL2                                                                                                                                                                               | Las reglas de denegación aún se aplican, y las reglas de solicitud que nombran un comando, como `Bash(git push *)`, aún solicitan. Para activar el sandbox desde un archivo de configuración en su lugar, establezca [`sandbox.enabled`](/docs/es/settings-reference#sandbox-enabled) en `true` |
| Explorar antes de cambiar nada                             | `claude --permission-mode plan`                                                                                                                                                               | Ninguno                                                                                                                                                                                                                         | Claude Code bloquea ediciones hasta que [apruebe un plan](#review-and-approve-a-plan)                                                                                                                                                                                                      |
| Trabajar sin intervención en modo automático               | `claude --permission-mode auto`, el [modo de permisos inicial integrado](#which-mode-a-session-starts-in) en Pro, Max y Team                                                                  | Ninguno; un sandbox o contenedor agrega defensa en profundidad                                                                                                                                                                  | Requiere un [modelo compatible](#eliminate-prompts-with-auto-mode), y su organización puede [desactivar modo automático](#eliminate-prompts-with-auto-mode)                                                                                                                                |
| Ejecutar en CI con una lista de permitidos exacta          | `claude -p "run the test suite" --permission-mode dontAsk --allowedTools "Bash(npm test)" "Read"`                                                                                             | Ninguno más allá de lo que proporciona su ejecutor de CI                                                                                                                                                                        | [Cloud sessions](/docs/es/claude-code-on-the-web) ignora `dontAsk` de archivos de configuración                                                                                                                                                                                                 |
| Ejecutar completamente desatendido dentro de un contenedor | `claude -p "<prompt>" --dangerously-skip-permissions`                                                                                                                                         | Requerido: un contenedor, máquina virtual o el [tiempo de ejecución del sandbox](/docs/es/sandbox-environments#sandbox-runtime); en Linux y macOS, ejecútelo como un [usuario no root](#skip-all-checks-with-bypasspermissions-mode) | Cloud sessions ignora este modo de archivos de configuración. En esta ejecución `-p`, las [pocas llamadas que aún solicitarían](#skip-all-checks-with-bypasspermissions-mode) se deniegan en su lugar                                                                                      |

El sandbox Bash y el modo automático funcionan independientemente y se combinan, con las excepciones enumeradas en [Modos de sandbox](/docs/es/sandboxing#sandbox-modes). Para la interacción completa, consulte [Cómo el sandboxing se relaciona con permisos y modos de permisos](/docs/es/sandboxing#how-sandboxing-relates-to-permissions-and-permission-modes) y [Cómo el aislamiento se relaciona con modos de permisos](/docs/es/sandbox-environments#how-isolation-relates-to-permission-modes).

<h2 id="which-mode-a-session-starts-in">
  Qué modo inicia una sesión
</h2>

Cuando inicia una nueva sesión en una terminal, Claude Code toma el modo de permisos del primero de estos que se aplique:

1. La bandera `--permission-mode` o `--dangerously-skip-permissions`

2. `permissions.defaultMode` en un [archivo de configuración](/docs/es/settings#where-settings-live)

   Si establece `"auto"` en `.claude/settings.json` o `.claude/settings.local.json`, el valor no entra en vigor, y Claude Code luego usa el valor predeterminado integrado en lugar de un `defaultMode` de `~/.claude/settings.json`. Si establece `"bypassPermissions"` en esos dos archivos, tampoco entra en vigor, y la sesión comienza en modo Manual. Los otros valores se aplican desde cualquier archivo de configuración.

3. El valor predeterminado integrado

Las conversaciones que inicia la extensión de VS Code siguen la lista propia de la extensión en [Cambiar modos de permisos](#switch-permission-modes). Para el modo de permisos en el que Claude Code inicia una sesión reanudada, consulte [modo de permisos al reanudar](/docs/es/sessions#permission-mode-on-resume).

El valor predeterminado integrado `auto` requiere Claude Code v2.1.228 o posterior en macOS, Linux y WSL, y v2.1.233 o posterior en Windows nativo. En versiones anteriores, el valor predeterminado integrado es Manual.

El valor predeterminado integrado depende de cómo ejecute Claude Code, de su plan y de si Claude Code pudo obtener sus banderas de características. La primera fila que coincida con su sesión se aplica. La tabla cubre sesiones que inicia en una terminal o a través de la extensión de VS Code; para la aplicación de escritorio y claude.ai, consulte las pestañas Desktop y Web en [Cambiar modos de permisos](#switch-permission-modes).

| Cómo ejecuta Claude Code                                                                                                                                                                                                                                              | Modo de permisos inicial integrado |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------- |
| Cualquier archivo de configuración establece `disableAutoMode` en `"disable"`                                                                                                                                                                                         | `default`                          |
| La [obtención de banderas de características](/docs/es/env-vars#features-that-need-feature-flag-fetching) está desactivada                                                                                                                                                 | `default`                          |
| Su [primera sesión después de instalar Claude Code o actualizar](/docs/es/env-vars#first-session-after-an-install-or-upgrade) a una versión que agrega este valor predeterminado, a menos que, después de una instalación nueva, Claude Code obtenga las banderas a tiempo | `default`                          |
| `claude -p` o el [SDK de Agent](/docs/es/agent-sdk/permissions)                                                                                                                                                                                                            | `default`                          |
| Amazon Bedrock, Agent Platform de Google Cloud, Microsoft Foundry, [Claude Platform en AWS](/docs/es/claude-platform-on-aws) o una sesión de [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) con sesión iniciada                                        | `default`                          |
| Un plan Pro, Max o Team, en una terminal o a través de la [extensión de VS Code](/docs/es/vs-code)                                                                                                                                                                         | `auto`                             |
| Un plan Enterprise o una clave API de Claude Console                                                                                                                                                                                                                  | `default`                          |

Cuando la obtención de banderas de características está desactivada, o en una [primera sesión después de una instalación o actualización](/docs/es/env-vars#first-session-after-an-install-or-upgrade) donde las banderas aún no han llegado, la extensión de VS Code ignora todos los archivos de configuración al elegir el modo de permisos inicial.

Cuando la bandera, un archivo de configuración o el valor predeterminado integrado selecciona `auto` pero el modo automático no está disponible para la sesión, Claude Code inicia la sesión en Manual en su lugar. El modo automático no está disponible cuando la sesión no cumple con los [requisitos de disponibilidad](#eliminate-prompts-with-auto-mode), como un archivo de configuración desactivándolo o un modelo que no lo admite, o cuando Anthropic lo ha desactivado temporalmente del lado del servidor.

La primera vez que el valor predeterminado integrado inicia una de sus sesiones en modo automático, Claude Code muestra un aviso que enlaza a esta página:

* En una terminal, una vez, en la parte superior de la sesión
* En la extensión de VS Code, como una tarjeta en la pantalla de nueva conversación que permanece hasta que la descarte

En planes Pro, Max y Team, si su `~/.claude/settings.json` establece un `defaultMode` distinto de `auto` y ningún otro archivo de configuración establece uno, sus sesiones siguen iniciándose en ese modo. Claude Code pregunta una vez, en la terminal o en la extensión de VS Code, si desea cambiar la configuración a modo automático. Si rechaza, su configuración permanece como está.

<h3 id="start-in-a-different-mode">
  Comience en un modo de permisos diferente
</h3>

Puede establecer el modo de permisos inicial para una sesión, o como predeterminado para cada sesión en una máquina, proyecto u organización. Cuando más de un archivo de configuración establece `permissions.defaultMode`, [precedencia de configuración](/docs/es/settings#settings-precedence) decide, por lo que un valor de proyecto o administrado supera `~/.claude/settings.json`. Para cambiar el modo de permisos de una sesión que ya se está ejecutando, consulte [Cambiar modos de permisos](#switch-permission-modes).

| Para establecer el modo de permisos inicial para   | Haga esto                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Una sesión que está a punto de iniciar             | Pase el modo de permisos como una bandera, por ejemplo `claude --permission-mode default`                                                                                                                                                                                                                                                                                                                                                      |
| Cada sesión de terminal que inicia en esta máquina | Establezca `permissions.defaultMode` en `~/.claude/settings.json`. Para lo que lee la extensión de VS Code, consulte [Cambiar modos de permisos](#switch-permission-modes)                                                                                                                                                                                                                                                                     |
| Cada sesión de terminal que inicia en un proyecto  | Establezca `permissions.defaultMode` en `.claude/settings.json` del proyecto. Las sesiones que inicia en una terminal respetan cada valor excepto `auto` y `bypassPermissions`; las sesiones que inicia la extensión de VS Code no leen configuración de proyecto para el modo de permisos inicial                                                                                                                                             |
| Cada sesión de terminal en su organización         | Establezca `permissions.defaultMode` en [configuración administrada](/docs/es/managed-settings). Las sesiones de terminal comienzan en ese modo y las personas aún pueden cambiar a modo automático; para lo que lee la extensión de VS Code, consulte [Cambiar modos de permisos](#switch-permission-modes). Para eliminar modo automático para que nadie pueda seleccionarlo, establezca `permissions.disableAutoMode` en `"disable"` en su lugar |

Este ejemplo hace que cada sesión de terminal en su máquina comience en modo Manual, cuyo valor de configuración es `default`. Guárdelo en `~/.claude/settings.json`:

```json theme={null}
{
  "permissions": {
    "defaultMode": "default"
  }
}
```

La siguiente sesión que inicia muestra `⏸ manual mode on` en la barra de estado.

<h2 id="switch-permission-modes">
  Cambiar modos de permisos
</h2>

Cada interfaz tiene su propio control para cambiar modos de permisos durante una sesión y su propia forma de elegir el modo de permisos que inician las nuevas sesiones. Seleccione su interfaz para ver sus controles.

<Tabs>
  <Tab title="CLI">
    **Durante una sesión**: presione `Shift+Tab` para ciclar modos de permisos. Desde `auto`, el primer presionamiento cambia a `default`, y el ciclo luego ejecuta `default` → `acceptEdits` → `plan` → de vuelta a `default`. Los modos opcionales, descritos a continuación, se insertan después de `plan`. La barra de estado muestra el modo activo como un `⏸ manual mode on` gris para `default`, o como `⏵⏵ accept edits on`, `⏸ plan mode on`, `⏵⏵ auto mode on`, `⏵⏵ don't ask on`, o `⏵⏵ bypass permissions on`.

    No todos los modos están en el ciclo predeterminado:

    * `auto`: aparece cuando [modo automático está disponible](#eliminate-prompts-with-auto-mode); cambiar a él cambia modos de permisos sin un aviso de confirmación
    * `bypassPermissions`: aparece después de que inicia con `--permission-mode bypassPermissions`, `--dangerously-skip-permissions`, `--allow-dangerously-skip-permissions` o `permissions.defaultMode: "bypassPermissions"` en [configuración de usuario, `--settings` o administrada](/docs/es/settings-reference#permissions-defaultmode). La variante `--allow-` agrega el modo de permisos al ciclo sin activarlo
    * `dontAsk`: nunca aparece en el ciclo; establézcalo con `--permission-mode dontAsk`

    Los modos opcionales habilitados se insertan después de `plan`, con `bypassPermissions` primero y `auto` último. Si tiene ambos habilitados, ciclará a través de `bypassPermissions` en el camino a `auto`.

    **Desde un aviso de permisos Bash**: en los modos de permisos Manual y `acceptEdits`, cuando [modo automático](#eliminate-prompts-with-auto-mode) está disponible, Claude Code agrega **Sí, y cambiar a modo automático** al aviso de permisos de un comando Bash. Selecciónelo para aprobar el comando y cambiar la sesión a modo automático. Los avisos de la [herramienta PowerShell](/docs/es/tools-reference#powershell-tool) no ofrecen la opción. Requiere Claude Code v2.1.247 o posterior.

    Claude Code no agrega la opción a avisos forzados por una de sus [reglas `ask`](/docs/es/permissions#manage-permissions) o por un [hook](/docs/es/hooks#pretooluse-decision-control), porque el modo automático aún le muestra esos avisos, por lo que cambiar no los eliminaría.

    **Al inicio**: pase el modo de permisos como una bandera.

    ```bash theme={null}
    claude --permission-mode plan
    ```

    **Como predeterminado**: establezca `permissions.defaultMode` en el alcance que desee, como se describe en [Comience en un modo de permisos diferente](#start-in-a-different-mode).

    La misma bandera `--permission-mode` funciona con `-p` para [ejecuciones no interactivas](/docs/es/headless).
  </Tab>

  <Tab title="VS Code">
    **Durante una sesión**: haga clic en el indicador de modo en la parte inferior del cuadro de solicitud. Utiliza estas etiquetas para los modos en esta página:

    | Etiqueta de interfaz   | Modo                |
    | :--------------------- | :------------------ |
    | Manual                 | `default`           |
    | Editar automáticamente | `acceptEdits`       |
    | Plan                   | `plan`              |
    | Auto                   | `auto`              |
    | Omitir permisos        | `bypassPermissions` |

    **Como predeterminado**: para fijar el modo de permisos en el que comienzan las conversaciones, establezca `claudeCode.initialPermissionMode` en la configuración de usuario de VS Code en `default`, `manual`, `acceptEdits`, `plan` o `bypassPermissions`. La configuración no acepta `auto`; para comenzar en Auto, déjela sin establecer y seleccione **Auto** del indicador de modo una vez, como describe el elemento 2 a continuación. La extensión comienza cada nueva conversación en el primero de estos que se aplique:

    1. `claudeCode.initialPermissionMode`
    2. El modo que seleccionó por última vez del indicador de modo, si fue Manual, Editar automáticamente o Auto. Seleccionar Plan u Omitir permisos se aplica solo a esa conversación
    3. `permissions.defaultMode` de [configuración administrada](/docs/es/managed-settings) o `~/.claude/settings.json`, en planes Pro, Max y Team con [obtención de banderas de características](#which-mode-a-session-starts-in) disponible
    4. El [valor predeterminado integrado](#which-mode-a-session-starts-in) para su plan, proveedor y configuración de organización

    La extensión nunca lee `.claude/settings.json` o `.claude/settings.local.json` de un proyecto para el modo de permisos inicial, y en conversaciones que no cumplen con las condiciones del elemento 3 no lee ningún archivo de configuración en absoluto. Cuando `claudeCode.claudeProcessWrapper` está establecido, los elementos 3 y 4 tampoco se aplican: esas conversaciones comienzan en Manual a menos que el elemento 1 o el elemento 2 establezca un modo de permisos.

    Auto aparece en el indicador de modo cuando [modo automático está disponible](#eliminate-prompts-with-auto-mode).

    Omitir permisos requiere el toggle **Allow dangerously skip permissions** en la configuración de la extensión. Sin él, el modo de permisos no aparece en el indicador, y un valor `bypassPermissions` del elemento 1 o elemento 3 comienza la conversación en Manual en su lugar. Auto de cualquier elemento asimismo comienza la conversación en Manual cuando el modo automático no está disponible.

    Consulte la [guía de VS Code](/docs/es/vs-code) para obtener detalles específicos de la extensión.
  </Tab>

  <Tab title="JetBrains">
    El plugin de JetBrains ejecuta Claude Code en la terminal del IDE, por lo que cambiar modos de permisos funciona igual que en la CLI: presione `Shift+Tab` para ciclar, o pase `--permission-mode` al iniciar.
  </Tab>

  <Tab title="Desktop">
    **Durante una sesión**: en la pestaña Code, use el selector de modo junto al botón de envío. No todos los modos aparecen en el selector:

    * **Auto**: aparece cuando [modo automático está disponible](#eliminate-prompts-with-auto-mode)
    * **Omitir permisos**: requiere el toggle **Allow bypass permissions mode** en la configuración de Desktop en planes Pro y Max; en planes Team y Enterprise, la política de la organización lo controla en su lugar

    La pestaña Cowork no utiliza estos modos. Cowork tiene sus propios modos de permisos, habilitados por separado, y la pestaña Cowork no muestra ningún selector de modo en absoluto hasta que un modo más allá de su predeterminado está habilitado para su cuenta. Consulte la [documentación de Cowork](https://claude.com/docs/cowork/overview).

    Para detalles específicos de desktop, consulte [Elegir un modo de permisos](/docs/es/desktop#choose-a-permission-mode) en la guía de Desktop.

    **Como predeterminado**: establezca `defaultMode` en [configuración](/docs/es/settings#where-settings-live). La aplicación de desktop lee los mismos archivos de configuración que la CLI y aplica el modo de permisos a nuevas sesiones locales.

    Un modo que selecciona en el selector de modo se recuerda por carpeta y tiene prioridad sobre `defaultMode` para esa carpeta. Plan es la excepción: seleccionarlo se aplica solo a la sesión actual.

    Para dónde va `defaultMode` en un archivo de configuración, consulte el ejemplo bajo [Comience en un modo de permisos diferente](#start-in-a-different-mode).
  </Tab>

  <Tab title="Web and mobile">
    Use el menú desplegable de modo junto al cuadro de solicitud en [claude.ai/code](https://claude.ai/code) o en la aplicación móvil. Los avisos de permisos aparecen en claude.ai para aprobación. Qué modos aparecen depende de dónde se ejecute la sesión:

    * **Sesiones en la nube** en [Claude Code en la web](/docs/es/claude-code-on-the-web): Aceptar ediciones, Plan y Auto. Aceptar ediciones corresponde al modo `default`: las sesiones en la nube aprueban previamente ediciones de archivos independientemente del modo, por lo que el menú desplegable muestra Aceptar ediciones en lugar de Manual. Las sesiones en la nube aún respetan `defaultMode: "acceptEdits"` de la configuración. El modo Auto aparece solo cuando su organización lo permite y el modelo seleccionado lo admite. Omitir permisos no está disponible.
    * **Sesiones de [Control Remoto](/docs/es/remote-control)** en su máquina local: Manual, Aceptar ediciones y Plan. No puede seleccionar Auto u Omitir permisos desde la aplicación.
      * Excepto por Omitir permisos, el menú desplegable muestra el modo de permisos en el que se encuentra la sesión local, incluido uno establecido desde la terminal. Se actualiza cuando el modo de permisos cambia en la aplicación o en la terminal. La sesión nunca reporta Omitir permisos a claude.ai, por lo que cambiar a él desde la terminal no cambia lo que muestra el menú desplegable.
      * Las sesiones alojadas por la [aplicación de desktop](/docs/es/desktop) o la [extensión de VS Code](/docs/es/vs-code) reportan cambios de modo de permisos a claude.ai a medida que suceden, igual que las sesiones alojadas en una terminal.
      * Antes de v2.1.202, las sesiones conectadas con `/remote-control` o `claude --remote-control` no reportaban su modo de permisos en absoluto, por lo que claude.ai y la aplicación móvil podrían mostrar un modo de permisos en el que la sesión no estaba. La discrepancia afectó solo la etiqueta. Claude Code generó avisos de permisos desde el modo de permisos real de la sesión, y aún aparecieron en la aplicación para aprobación.

    Para Control Remoto, la máquina local que ejecuta la sesión debe estar conectada con su cuenta de claude.ai; las claves API no son compatibles. También puede establecer el modo de permisos inicial al iniciar esa sesión local:

    ```bash theme={null}
    claude remote-control --permission-mode acceptEdits
    ```
  </Tab>
</Tabs>

<h2 id="auto-approve-file-edits-with-acceptedits-mode">
  Aprobar automáticamente ediciones de archivos con modo acceptEdits
</h2>

El modo `acceptEdits` permite que Claude cree y edite archivos en su directorio de trabajo sin solicitar. La barra de estado muestra `⏵⏵ accept edits on` mientras este modo está activo.

Además de ediciones de archivos, el modo `acceptEdits` aprueba automáticamente comandos Bash comunes del sistema de archivos: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp` y `sed`. Estos comandos también se aprueban automáticamente cuando van precedidos de variables de entorno seguras como `LANG=C` o `NO_COLOR=1`, o envoltorios de procesos como `timeout`, `nice` o `nohup`. Al igual que las ediciones de archivos, la aprobación automática se aplica solo a rutas dentro de su directorio de trabajo o `additionalDirectories`. Las rutas fuera de ese alcance, las escrituras en [rutas protegidas](#protected-paths), las eliminaciones `rm` y `rmdir` dirigidas a una [ruta crítica](#critical-paths) y todos los demás comandos Bash excepto el [conjunto integrado de solo lectura](/docs/es/permissions#read-only-commands) aún solicitan.

Cuando la [herramienta PowerShell](/docs/es/tools-reference#powershell-tool) está habilitada, el modo `acceptEdits` también aprueba automáticamente `Set-Content`, `Add-Content`, `Clear-Content` y `Remove-Item` en rutas dentro del alcance, junto con sus alias comunes. Se aplican las mismas reglas de alcance y ruta protegida, y `Remove-Item` obtiene [su propia verificación](#remove-item-in-powershell). Un argumento posicional que contiene un carácter de comilla, como el apóstrofo en `Set-Content .\notes.txt "It's done"`, aún solicita incluso en rutas dentro del alcance, porque Claude Code no puede validar estáticamente un argumento cuyas lecturas entrecomilladas y sin entrecomillar difieren. Pase el contenido a través de un parámetro nombrado como `-Value` para evitar el aviso.

Utilice `acceptEdits` cuando desee revisar cambios en su editor o mediante `git diff` después del hecho en lugar de aprobar cada edición en línea.

Presione `Shift+Tab` una vez desde modo Manual para entrar en él, o comience directamente con él:

```bash theme={null}
claude --permission-mode acceptEdits
```

<h2 id="analyze-before-you-edit-with-plan-mode">
  Analice antes de editar con modo plan
</h2>

El modo plan le indica a Claude que investigue y proponga cambios sin realizarlos. Claude lee archivos, ejecuta comandos de shell para explorar y escribe un plan, pero no edita su fuente. Excepto en sesiones con [permisos de omisión disponibles](#skip-all-checks-with-bypasspermissions-mode), las ediciones permanecen bloqueadas hasta que apruebe el plan.

Cuando [modo automático](/docs/es/auto-mode-config) está disponible y la configuración `useAutoModeDuringPlan` está activada, que lo está de forma predeterminada, el clasificador revisa comandos de shell durante la planificación en lugar de solicitarle. Los comandos aprobados se ejecutan, y los rechazados se bloquean. De lo contrario, los comandos fuera del [conjunto integrado de solo lectura](/docs/es/permissions#read-only-commands) solicitan aprobación, incluido cuando el [modo de permitir automático](/docs/es/sandboxing#sandbox-modes) del sandbox está habilitado. En sesiones con permisos de omisión disponibles, ni el clasificador ni un aviso se aplican a comandos de planificación; [Omitir todas las comprobaciones con modo bypassPermissions](#skip-all-checks-with-bypasspermissions-mode) cubre las pocas cosas que aún solicitan allí. En v2.1.212 a v2.1.217, las sesiones sin permisos de omisión solicitaban cada comando fuera del conjunto de solo lectura, independientemente de si el modo automático estaba disponible.

Ingrese al modo plan presionando `Shift+Tab` o prefijando un único aviso con `/plan`. También puede comenzar en modo plan desde la CLI:

```bash theme={null}
claude --permission-mode plan
```

Presione `Shift+Tab` nuevamente para salir del modo plan sin aprobar un plan.

<h3 id="review-and-approve-a-plan">
  Revise y apruebe un plan
</h3>

Cuando el plan esté listo, Claude lo presenta y pregunta cómo proceder. Desde ese aviso puede elegir:

* **Sí, y usar modo automático**: apruebe e inicie en [modo automático](#eliminate-prompts-with-auto-mode). Si el modo automático no está [disponible para su sesión](#eliminate-prompts-with-auto-mode), por ejemplo porque su organización lo desactivó, esta opción dice **Sí, aceptar automáticamente ediciones**. Si inició la sesión con permisos de omisión habilitados, la opción dice **Sí, y cambiar a OMITIR PERMISOS (sin más avisos) para esta sesión** en su lugar.
* **Sí, aprobar manualmente ediciones**: apruebe y revise cada edición individualmente.
* **No, seguir planificando**: permanezca en modo plan y dígale a Claude qué cambiar.

Aprobar un plan sale del modo plan y cambia la sesión al modo de permisos que describe cada opción de aprobación, por lo que Claude comienza a editar. Para planificar nuevamente, cicle de vuelta al modo plan con `Shift+Tab`, o prefije su próximo aviso con `/plan`.

Presione `Ctrl+G` para abrir el plan propuesto en su editor de texto predeterminado y edítelo directamente antes de que Claude continúe. Cuando [`showClearContextOnPlanAccept`](/docs/es/settings-reference#showclearcontextonplanaccept) está habilitado, la lista gana una primera opción que aprueba el plan y borra el contexto de planificación.

Aprobar un plan también da a la sesión un [título generado](/docs/es/sessions#name-your-sessions) basado en el plan, a menos que ya haya nombrado la sesión.

<h3 id="set-plan-mode-as-the-default">
  Establezca el modo plan como predeterminado
</h3>

Para hacer que el modo plan sea el predeterminado para las sesiones de terminal de un proyecto, establezca `defaultMode` en `plan` en `.claude/settings.json`, colocado como muestra el ejemplo bajo [Comience en un modo de permisos diferente](#start-in-a-different-mode). Las conversaciones que inicia la [extensión de VS Code](/docs/es/vs-code) no leen configuración de proyecto para el modo de permisos inicial. Allí, establezca `claudeCode.initialPermissionMode` en `plan` en su configuración de usuario de VS Code en su lugar.

<h2 id="eliminate-prompts-with-auto-mode">
  Eliminar solicitudes de permiso con modo automático
</h2>

El modo automático permite que Claude se ejecute sin solicitudes de permiso rutinarias. Un modelo clasificador separado revisa las acciones antes de que se ejecuten, bloqueando cualquier cosa que escale más allá de su solicitud, apunte a infraestructura no reconocida o parezca impulsada por contenido hostil que Claude leyó. Las [reglas de solicitud](/docs/es/permissions#manage-permissions) explícitas aún fuerzan una solicitud.

En los planes Pro, Max y Team, el modo automático es el [modo de permiso de inicio integrado](#which-mode-a-session-starts-in).

El clasificador también revisa cada mensaje que Claude envía a otro agente con [`SendMessage`](/docs/es/tools-reference), ya sea texto sin formato o un mensaje de [equipo de agentes](/docs/es/agent-teams) estructurado, antes de que Claude Code lo entregue, tanto en modo automático como en [modo de plan mientras el clasificador revisa comandos](#analyze-before-you-edit-with-plan-mode); la revisión de envío requiere Claude Code v2.1.222 o posterior.

El clasificador también revisa y aprueba o bloquea las eliminaciones `rm` y `rmdir` dirigidas a una [ruta crítica](#critical-paths), como `rm -rf /` y `rm -rf ~`, incluso cuando la eliminación está dentro de sustitución de comandos o procesos.

El modo automático también anima a Claude a seguir trabajando sin detenerse para hacer preguntas aclaratorias, aunque Claude aún pregunta cuando su solicitud o una skill depende explícitamente de ello. Para un comportamiento más autónomo en un modo que aún le solicita, establezca el [estilo de salida proactivo](/docs/es/output-styles) en su lugar.

<Warning>
  El modo automático reduce las solicitudes de permiso pero no garantiza seguridad. Úselo para tareas donde confía en la dirección general, no como reemplazo de revisión en operaciones sensibles.
</Warning>

El modo automático está disponible solo cuando su cuenta cumple con todos estos requisitos:

* **Plan**: Todos los planes.
* **Organización**: en Team y Enterprise, el modo automático está disponible de forma predeterminada. Los administradores pueden desactivarlo para la organización estableciendo `permissions.disableAutoMode` en `"disable"` en [configuración administrada](/docs/es/managed-settings).
* **Modelo**: en la API de Anthropic y [Claude Platform en AWS](/docs/es/claude-platform-on-aws), Claude Opus 4.6 o posterior, Sonnet 4.6 o posterior, o un [modelo Fable](/docs/es/model-config#work-with-fable). En Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry y sesiones de [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) con sesión iniciada, solo Claude Sonnet 5, Opus 4.7 o posterior y los modelos Fable. Los modelos más antiguos, incluidos Sonnet 4.5, Opus 4.5, Haiku y modelos claude-3, no son compatibles en ningún proveedor.
* **Proveedor**: disponible de forma predeterminada en la API de Anthropic, Claude Platform en AWS, Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry y sesiones de puerta de enlace de aplicaciones Claude con sesión iniciada.

Si Claude Code informa que el modo automático no está disponible, primero verifique estos requisitos y si algún archivo de configuración establece [`disableAutoMode`](/docs/es/settings-reference#disableautomode). Anthropic también puede haber desactivado el modo automático del lado del servidor, o el servidor puede haber rechazado el modo automático para su cuenta. Una sesión que recibió cualquiera de estas respuestas mantiene el modo automático desactivado hasta que finaliza la sesión, así que inicie una nueva sesión más tarde.

Un mensaje separado que nombra un modelo y dice que el modo automático "no puede determinar la seguridad" de una acción significa que una solicitud del clasificador falló. Ese fallo suele ser transitorio, pero en Amazon Bedrock puede repetirse hasta que su cuenta pueda invocar el modelo nombrado. Consulte la [referencia de errores](/docs/es/errors#auto-mode-cannot-determine-the-safety-of-an-action) para conocer las causas y qué hacer.

Si establece `defaultMode: "auto"` en [configuración](/docs/es/settings-reference#all-settings) y una sesión de terminal comienza en modo Manual sin error, la configuración probablemente esté en `.claude/settings.json` o `.claude/settings.local.json`. `auto` no entra en vigor desde esos archivos. Muévalo a `~/.claude/settings.json`. Para una conversación que la extensión VS Code inició, verifique la lista propia de la extensión en [Cambiar modos de permiso](#switch-permission-modes) en su lugar.

<h3 id="enable-auto-mode-on-bedrock-agent-platform-or-foundry">
  Modo automático en Bedrock, Agent Platform o Foundry
</h3>

En sesiones de [Amazon Bedrock](/docs/es/amazon-bedrock), [Google Cloud's Agent Platform](/docs/es/google-vertex-ai), [Microsoft Foundry](/docs/es/microsoft-foundry) y [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) con sesión iniciada, el modo automático aparece en el ciclo `Shift+Tab` de forma predeterminada. Aparecer en el ciclo no cambia el modo de permiso en el que comienza una sesión: en estos proveedores, las sesiones de terminal comienzan en su [`defaultMode`](/docs/es/settings-reference#permissions-defaultmode), que es Manual a menos que lo cambie, y las conversaciones en la [extensión VS Code](/docs/es/vs-code) comienzan en Manual a menos que `claudeCode.initialPermissionMode` o un modo que eligió en la extensión establezca uno. Solo Claude Sonnet 5, Opus 4.7 o posterior y los modelos Fable son compatibles en estos proveedores.

Para hacer que el modo automático sea el modo de permiso de inicio predeterminado, establezca `"permissions": {"defaultMode": "auto"}` en la configuración de usuario o administrada. En sesiones que la extensión VS Code inicia, seleccione **Auto** del indicador de modo en su lugar. [Cambiar modos de permiso](#switch-permission-modes) cubre qué anula esa selección.

El checkup [`/doctor`](/docs/es/commands#all-commands) propone este valor predeterminado de configuración de usuario en estos proveedores de la misma manera que lo hace en la API de Anthropic.

Para evitar que los desarrolladores usen el modo automático, establezca `disableAutoMode` en `"disable"` en [configuración administrada](/docs/es/managed-settings). Esto elimina `auto` del ciclo `Shift+Tab`, y una sesión iniciada con `--permission-mode auto` comienza en Manual en su lugar. Una sesión ya en ejecución en modo automático lo abandona cuando la configuración llega a esa sesión desde una [fuente implementada por administrador](/docs/es/managed-settings#which-managed-source-claude-code-uses), y muestra `auto mode disabled by settings`. Antes de v2.1.251, una sesión en ejecución mantenía el modo automático hasta que finalizaba.

En v2.1.158 a v2.1.206, el modo automático estaba desactivado en estos proveedores hasta que establecía `CLAUDE_CODE_ENABLE_AUTO_MODE=1`, y Claude Code ignoraba `defaultMode: "auto"` en estos proveedores a menos que la variable también estuviera establecida. La variable aún se acepta por compatibilidad y no tiene efecto desde v2.1.207 en adelante.

<h3 id="server-side-classifier-review">
  Revisión del clasificador del lado del servidor
</h3>

En modo automático, Claude Code puede solicitar al servidor que revise las acciones que [el orden de decisión](#how-the-classifier-evaluates-actions) envía para revisión, como parte de las solicitudes del modelo de la sesión, en lugar de enviar sus propias solicitudes del clasificador. Estas sesiones solicitan:

* **Una conexión directa a la API de Anthropic**: en una sesión de terminal interactiva, en todos los planes de claude.ai y en cuentas que usan la API de Claude, a medida que Anthropic lo implementa. Requiere Claude Code v2.1.271 o posterior en planes Pro, Max y Team, y v2.1.278 o posterior en planes Enterprise y cuentas de API de Claude. Desde v2.1.282, una sesión que [no obtiene banderas de características](/docs/es/env-vars#features-that-need-feature-flag-fetching), por ejemplo porque desactivó la telemetría, solicita al servidor de forma predeterminada en cualquier tipo de sesión.
* **Un proveedor de nube, o una puerta de enlace LLM o proxy**: en [Claude Platform en AWS](/docs/es/claude-platform-on-aws), Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry, y siempre que apunte `ANTHROPIC_BASE_URL` a una [puerta de enlace LLM o proxy](/docs/es/llm-gateway), sea cual sea su plan. Solicitar de forma predeterminada requiere Claude Code v2.1.278 o posterior.
* **Una sesión de [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) con sesión iniciada**: requiere Claude Code v2.1.280 o posterior

Donde el servidor revisa las acciones, sus veredictos las deciden. Otros dos resultados son posibles:

* **El servidor no revisa la sesión**: una respuesta se completa sin resultados de revisión, o el servidor responde que no revisa esta sesión. Las causas más comunes son una puerta de enlace LLM o proxy que descarta la solicitud de revisión o los resultados, y una plataforma, región o credencial que aún no tiene verificaciones del lado del servidor. Claude Code retrocede a sus propias solicitudes del clasificador. Una vez que ese retroceso se mantiene para el resto de la sesión, muestra un [aviso sobre cargos de solicitud del clasificador](/docs/es/auto-mode-classifier-billing) en cuentas donde esas solicitudes se facturan.
* **El servidor no da veredicto para una acción**: Claude Code niega la acción en lugar de ejecutarla sin revisar. En cualquier conexión, esto sucede cuando la respuesta termina antes de que lleguen los resultados de revisión o los resultados llegan en una forma que Claude Code no puede leer. Una puerta de enlace LLM o proxy que corta respuestas o reescribe los resultados puede causar cualquiera de los dos. En una conexión directa a la API de Anthropic, también sucede cuando la verificación del servidor falla para la acción, por ejemplo al agotarse el tiempo. [El servidor no devolvió veredicto de seguridad](/docs/es/errors#the-server-returned-no-safety-verdict) cubre el mensaje de denegación, qué sucede cuando las denegaciones se repiten y qué hacer.

Para omitir solicitar al servidor y siempre usar las propias solicitudes del clasificador de Claude Code, establezca [`CLAUDE_CODE_AUTO_MODE_SERVER=0`](/docs/es/env-vars). En una conexión directa a la API de Anthropic, la variable requiere Claude Code v2.1.281 o posterior. Establecerlo en `1` allí activa la revisión del servidor en una sesión que no la tiene aún, como una sesión `-p` o Agent SDK, a menos que también haya establecido `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`. Si establece `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` y deja `CLAUDE_CODE_AUTO_MODE_SERVER` sin establecer, Claude Code también deja de solicitar al servidor.

<h3 id="what-the-classifier-blocks-by-default">
  Qué bloquea el clasificador de forma predeterminada
</h3>

El clasificador confía en su directorio de trabajo y en los remotos que se configuraron para él cuando comenzó la sesión. Un remoto agregado o reapuntado durante la sesión con `git remote add` o `git remote set-url` no es de confianza, y todo lo demás se trata como externo hasta que [configure infraestructura de confianza](/docs/es/auto-mode-config). Antes de v2.1.200, los remotos agregados a mitad de sesión también eran de confianza.

**Bloqueado de forma predeterminada**:

* Descargar y ejecutar código, como `curl | bash`
* Enviar datos sensibles a puntos finales externos
* Implementaciones y migraciones de producción
* Eliminación masiva en almacenamiento en la nube
* Otorgamiento de permisos de IAM o repositorio
* Modificación de infraestructura compartida
* Destrucción irreversible de archivos que existían antes de la sesión
* Inserción forzada
* Confirmar o insertar un cambio que enviaría secretos o datos sensibles fuera del repositorio cuando se ejecuta, o ampliar lo que expone una implementación. Esto cubre un flujo de trabajo de CI o configuración de implementación que entrega un secreto a un destino que no lo recibe ya, un script o paso de configuración que lee un almacén de secretos y envía los datos, y un cambio de configuración que amplía lo que publica una implementación, como un registro, visibilidad, artefacto o configuración de sourcemap. La verificación se aplica en cualquier rama, se aplica incluso cuando el repositorio es público y se activa cuando el cambio se confirma o inserta, independientemente de si esa confirmación o inserción desencadena la canalización; limpiarla requiere nombrar el efecto de ejecución, no solo la confirmación o inserción. Antes de v2.1.211, esta verificación se limitaba a la rama predeterminada: una inserción allí se bloqueaba cuando llevaba contenido sensible, cambios ocultos o mal descritos en relación con lo que pidió, contenido portado desde fuera del repositorio o enrutado alrededor de una revisión que pidió
* `git reset --hard`, `git checkout -- .`, `git restore .`, `git clean -fd`, `git stash drop` o `git stash clear`, que el clasificador presume descartaría cambios sin confirmar
* `git commit --amend` cuando la confirmación en HEAD no fue creada en esta sesión
* Desde v2.1.198, `git commit --amend` cuando la confirmación en HEAD ya ha sido insertada. Una reescritura solo de mensaje no se bloquea: `--amend -m` sin nada recién preparado, en una confirmación que Claude creó durante esta sesión
* `terraform destroy`, `pulumi destroy`, `cdk destroy` o `terragrunt destroy`, e implementar un plan que destruye recursos

Claude Code v2.1.195 y posterior bloquean más categorías de forma predeterminada. Varias dependen de entradas de [entorno](/docs/es/auto-mode-config#define-trusted-infrastructure), como destinos remotos sensibles y alcances de IaC protegidos, que puede reducir a nombres concretos.

* Escritura en un administrador de secretos, o cambio de registros DNS o certificados TLS
* Fusión de una solicitud de extracción que ningún humano ha aprobado, aprobación de la propia solicitud de extracción de Claude o deshabilitación de verificaciones de CI
* Publicación de un comentario que es en sí mismo un comando para automatización, como `atlantis apply` o `/deploy` o `/merge` de un bot
* Alternancia, ramificación o eliminación de una bandera de característica de producción
* Aplicación de cambios de infraestructura a un alcance de IaC protegido, o drenaje y eliminación de nodos de clúster
* Escrituras en un clúster de cómputo compartido que van más allá del recurso que nombró, como un selector de etiqueta o `--all` que captura trabajos de otros usuarios
* Creación de recursos de Kubernetes que se ejecutan en cada nodo o interceptan tráfico de clúster, como DaemonSets y webhooks de admisión
* Shells interactivos o reenvíos de puertos a un destino remoto sensible
* Apertura de un túnel o shell inverso que hace que un servicio local sea accesible desde Internet público
* Impresión de una credencial o token en vivo en la transcripción o un archivo
* Acceso a una ubicación listada como ubicación de datos sensibles en su [entorno](/docs/es/auto-mode-config#define-trusted-infrastructure), o copia de datos de una. A partir de v2.1.198, esto también bloquea el envío de datos de uno a una audiencia que la entrada excluye
* Enrutamiento de una instalación de paquete alrededor de su registro de paquetes interno a un registro público. A partir de v2.1.198, esto también se aplica cuando le ha dicho a Claude que existe un registro interno o espejo en la conversación, no solo cuando uno está listado en su entorno
* Ejecución de un comando con una bandera que desactiva una protección de seguridad, como `--insecure`
* Lanzamiento de un bucle de agente autónomo que se ejecuta sin aprobación humana o sandbox, como uno iniciado con `--dangerously-skip-permissions` o `--no-sandbox`. A partir de v2.1.198, esto también cubre la ejecución de un agente de terceros o arnés de evaluación con aislamiento y aprobación por acción deshabilitados, como un ejecutor iniciado con `--yes-always`
* Acciones del navegador [Claude en Chrome](/docs/es/chrome) que podrían enviar contenido de página, cookies o credenciales fuera de origen

Claude Code v2.1.198 y posterior también bloquean estos de forma predeterminada:

* Eliminación de archivos en `/tmp`, `$TMPDIR` u otro directorio compartido de scratch o caché por comodín, glob o filtro de edad en lugar de por una ruta nombrada específica
* Inclusión de detalles sensibles en contenido enviado, cargado, publicado o escrito a otras personas o sistemas compartidos, cuando su propio mensaje no autorizó esos detalles para ese destinatario. Los cuerpos de PR y problemas, mensajes de confirmación y comentarios cuentan como este tipo de contenido saliente cuando el repositorio está fuera del límite de confianza o es público, incluidos los repositorios públicos de su propia organización; las rutas de archivo internas, nombres de código, datos de respuesta de API en vivo como correos electrónicos o identificadores de cuenta e identificadores de infraestructura cuentan como detalles sensibles. El alcance de PR, problema y mensaje de confirmación requiere Claude Code v2.1.200 o posterior. Los datos personales en vivo de una respuesta de API en un cuerpo de PR o problema, como una dirección de correo electrónico, un identificador de cuenta u organización o una métrica de uso, requieren que nombre esos detalles y el destinatario independientemente de la visibilidad o límite de confianza del repositorio. Esa verificación requiere Claude Code v2.1.203 o posterior
* Envío de pulsaciones de teclas al propio panel tmux de Claude Code para impulsar su propia interfaz, que el clasificador trata como Claude cambiando sus propios permisos o supervisión

Claude Code v2.1.200 y posterior también bloquean estos de forma predeterminada:

* Comentario, eliminación o paso forzado de una prueba o afirmación que protege el comportamiento de seguridad, como autenticación, control de acceso, validación de entrada o sandboxing
* Eliminación o desmantelamiento de un recurso con estado que Claude no creó en la sesión, cuando no se aplica una regla de eliminación más específica y no nombró ese recurso
* Reapuntamiento de una URL base de API, punto final de proxy, receptor de webhook o espejo de registro a un host de terceros que no se ajusta a la tarea, incluso en archivos de ejemplo como `.env.example`
* Cambio de dónde van las inserciones con `git remote set-url` o `git remote add`, a menos que nombre el nuevo remoto
* Inserción de secretos o datos personales o confiados a un repositorio conocido como público, o inserción de material confidencial allí que no es parte del trabajo propio de ese repositorio. El asunto propio de un repositorio de dotfiles es la única excepción para datos personales o confiados, y el contenido de un repositorio privado que llega a cualquier superficie pública se bloquea de la misma manera; ambos refinamientos requieren Claude Code v2.1.203 o posterior. Antes de v2.1.203, los datos personales se agrupaban con material confidencial y se bloqueaban solo cuando no eran parte del trabajo propio de ese repositorio. Cuando la visibilidad de un repositorio no está establecida, el clasificador no bloquea solo por eso; juzga el contenido contra las otras reglas en su lugar
* Apertura de una solicitud de extracción contra un repositorio u organización diferente, bifurcación con `gh repo fork` o inserción a un repositorio de terceros, a menos que nombre ese destino externo

Claude Code v2.1.203 y posterior también bloquean estos de forma predeterminada:

* Contenido de un almacén local sensible, o de un archivo cuyo nombre, ruta o tipo lo marca como sensible, entrando en una confirmación, una inserción, texto de PR o problema, una esencia o pegado, o una publicación de paquete, a menos que nombre tanto la fuente como el destino. Las transcripciones de sesión y registros de conversación, carpetas de puntos de credencial y configuración como claves SSH, credenciales en la nube, perfiles de navegador e historial de shell, y exportaciones de datos de usuario cuentan, y el repositorio siendo privado no lo aclara

Claude Code v2.1.205 y posterior también bloquean estos de forma predeterminada:

* Escritura en transcripciones de sesión de Claude Code, los archivos de historial `.jsonl` bajo `~/.claude/projects/` o su directorio de configuración configurado, ya sea directamente o a través de un comando de shell. La regla también cubre las líneas de metadatos que Claude Code añade a cada entrada de transcripción para sus propias verificaciones. Leer una transcripción no se bloquea
* Una eliminación forzada recursiva como `rm -rf "$VAR"` o `Remove-Item -Recurse -Force $dir` cuyo destino es una variable de shell, o un glob enraizado en una, que no está asignado en ningún lugar de la conversación que el clasificador ve. El valor provino solo de la salida de comando anterior, que el clasificador nunca recibe, por lo que el clasificador no puede verificar el destino de eliminación contra las otras reglas de eliminación. El bloqueo se aclara cuando nombra la ruta exacta que se está eliminando, o cuando Claude vuelve a ejecutar la eliminación con la ruta literal resuelta escrita en el comando. Las eliminaciones cuyo destino el clasificador puede resolver no se ven afectadas. Los destinos `Remove-Item` que son un `*` desnudo o terminan en `/*` o `\*` nunca llegan al clasificador: Claude Code [los niega directamente](#remove-item-in-powershell)

Claude Code v2.1.257 y posterior también bloquean estos de forma predeterminada:

* Solicitud de credenciales del punto final de metadatos de instancia en la nube, como `169.254.169.254`, o autenticación explícita de una llamada de nube, clúster o registro con la identidad de cuenta de servicio o nodo de la máquina
* Alcance de un host público por una ruta que no sea una solicitud directa, como un túnel, un shell inverso, o una configuración de resolutor o proxy reescrita para apuntar fuera
* Lectura de credenciales que pertenecen al host en lugar de a su tarea, como certificados de nodo o la autenticación de registro de contenedor del nodo
* Conexión a o escaneo de contenedores, pods o máquinas virtuales hermanas que Claude no inició, o el nodo debajo del contenedor

Si Claude Code se ejecuta en algún lugar que se supone que permite uno de estos, describa esa configuración en una entrada [Host containment](/docs/es/auto-mode-config#define-trusted-infrastructure) en `autoMode.environment`.

Claude Code v2.1.261 y posterior también bloquean estos de forma predeterminada:

* Publicación o escritura de un enlace a un servicio público de pegado, diagrama o intercambio de datos en un mensaje, texto de PR o problema, un documento, o en cualquier otro lugar donde se abrirá o recuperará el enlace, cuando la URL misma lleva el contenido que se comparte, a menos que nombre ese servicio

**Permitido de forma predeterminada**:

* Operaciones de archivo local en su directorio de trabajo
* Instalación de dependencias declaradas en sus archivos de bloqueo o manifiestos
* Lectura de `.env` y envío de credenciales a su API coincidente
* Solicitudes HTTP de solo lectura
* Inserción a cualquier rama del repositorio en el que está trabajando, incluida la rama predeterminada. Una rama no predeterminada cuyo nombre la marca como destino de implementación o publicación, como `production` o `gh-pages`, no está cubierta: el clasificador juzga una inserción allí en sus propios términos. El contenido de la inserción aún se verifica contra las otras reglas, las reglas [`permissions.deny`](/docs/es/permissions#manage-permissions) aún pueden bloquear comandos de inserción [tal como están escritos](/docs/es/permissions#bash-rule-limits) en todos los modos, y la protección de rama propia del remoto aún se aplica. Antes de v2.1.211, solo las inserciones a la rama en la que comenzó, ramas que Claude creó e inserciones rutinarias a la rama predeterminada se permitían de forma predeterminada, y antes de v2.1.203 cualquier inserción directa a la rama predeterminada se bloqueaba

Claude Code v2.1.195 y posterior también permiten estos de forma predeterminada:

* Eliminación de los trabajos exactos que Claude creó anteriormente en la misma sesión
* Lectura, revisión o escritura de código, configuraciones y modelos de amenaza relacionados con seguridad como parte de su tarea
* Mensajes entre agentes que trabajan juntos en la misma sesión multiagente
* Envío de datos a los dominios, cubos y servicios de confianza que lista en [`environment`](/docs/es/auto-mode-config#define-trusted-infrastructure). Esto cubre solo el flujo de datos, no operaciones destructivas o de credencial en la misma infraestructura
* [Claude en Chrome](/docs/es/chrome) navegación a un dominio interno de confianza, localhost o una URL que nombró

Los comandos en sandbox no obtienen acceso a la red de forma predeterminada. Claude nombra los hosts que un comando necesita en el comando mismo, el clasificador los revisa con el comando, y una lista aprobada abre esos hosts solo para ese comando. [Dominios permitidos por comando](/docs/es/sandboxing#per-command-allowed-domains-in-auto-mode) cubre qué puede y qué no puede abrir una lista y qué sucede cuando un comando llega a un host no listado.

Ejecute `claude auto-mode defaults` para imprimir las listas de reglas completas como JSON. Si las acciones rutinarias se bloquean, un administrador puede agregar repositorios, cubos y servicios de confianza a través de la configuración `autoMode.environment`: consulte [Configurar modo automático](/docs/es/auto-mode-config).

La inserción a cualquier rama del repositorio en el que está trabajando y la creación de una solicitud de extracción que coincida con su solicitud se ejecutan sin solicitud, a menos que la inserción o solicitud de extracción caiga bajo la [lista bloqueada](#what-the-classifier-blocks-by-default), como secretos o datos sensibles que salen del repositorio, o una solicitud de extracción que apunta a un repositorio u organización diferente. Para requerir un punto de control humano antes de estos comandos mientras permanece en modo automático, agregue reglas `permissions.ask`, que coinciden con el comando [tal como está escrito](/docs/es/permissions#bash-rule-limits): consulte [Límites comunes](/docs/es/auto-mode-config#common-boundaries).

<h3 id="first-read-outside-the-working-directories">
  La primera lectura fuera de los directorios de trabajo
</h3>

Mientras [`permissions.blockReadsOutsideWorkingDirectories`](/docs/es/settings-reference#permissions-blockreadsoutsideworkingdirectories) está desactivado, las lecturas de archivo se ejecutan sin solicitud en modo automático, incluidas las lecturas fuera de los [directorios de trabajo](/docs/es/permissions#working-directories). La primera vez que Claude usa la herramienta Read, Grep o Glob en una ruta fuera de ellos, Claude Code le pregunta si desea seguir permitiendo esas lecturas.

La solicitud no aparece en ejecuciones `-p` no interactivas o sesiones en segundo plano; las lecturas allí se ejecutan como antes.

Sea cual sea su respuesta, Claude sigue trabajando:

* **Seguir permitiendo**: la lectura se ejecuta, las lecturas posteriores fuera de los directorios de trabajo se ejecutan como antes, y Claude Code registra su respuesta para que la solicitud no aparezca nuevamente
* **Bloquear de ahora en adelante**: la lectura se rechaza, y Claude Code establece [`permissions.blockReadsOutsideWorkingDirectories`](/docs/es/settings-reference#permissions-blockreadsoutsideworkingdirectories) en `true` en su configuración de usuario, lo que hace que las herramientas de archivo rechacen tales lecturas en cada sesión posterior y cada modo de permiso. Para permitir que Claude lea tal ruta más tarde, agregue su directorio con `/add-dir` o elimine la configuración.
* **Preguntar nuevamente la próxima vez**: la lectura se rechaza, y la próxima lectura fuera de los directorios de trabajo solicita nuevamente

<h3 id="boundaries-you-state-in-conversation">
  Límites que establece en la conversación
</h3>

El clasificador trata los límites que establece en la conversación como una señal de bloqueo. Si le dice a Claude "no insertes" o "espera hasta que revise antes de implementar", el clasificador bloquea acciones coincidentes incluso cuando las reglas predeterminadas las permitirían. Un límite permanece en vigor hasta que lo levanta en un mensaje posterior. El propio juicio de Claude de que se cumplió una condición no lo levanta.

Los límites no se almacenan como reglas. El clasificador los vuelve a leer de la transcripción en cada verificación, por lo que un límite puede perderse si [compactación de contexto](/docs/es/costs#reduce-token-usage) elimina el mensaje que lo estableció. Para una garantía dura, agregue una [regla de denegación](/docs/es/permissions#permission-rule-syntax) en su lugar.

<h3 id="approvals-you-state-in-conversation">
  Aprobaciones que establece en la conversación
</h3>

Si le dice a Claude que una acción bloqueada está permitida, el clasificador lee eso como su aprobación y puede despejar el bloqueo. La forma en que lo expresó decide si la acción se ejecuta y qué tan lejos llega la aprobación:

* **Nombre la acción y sus especificidades**: su mensaje tiene que nombrar la acción y la cosa específica que la hace peligrosa, como la rama de una inserción forzada. Nombrar solo el verbo no aclara nada, así que "puede hacer una inserción forzada" deja el bloqueo en su lugar.
* **Espere que cubra una acción**: una aprobación cubre la acción destructiva que nombró, así que una acción posterior se bloquea nuevamente a menos que haya otorgado la aprobación como permanente. Para dejar de aprobar un patrón rutinario una acción a la vez, agréguelo a [`autoMode.allow`](/docs/es/auto-mode-config#override-the-block-and-allow-rules).
* **Algunos bloqueos permanecen en su lugar**: [el orden de precedencia del clasificador](/docs/es/auto-mode-config#override-the-block-and-allow-rules) establece qué bloqueos puede alcanzar su aprobación. Para ejecutar un paso que no despejará, [salga del modo automático](#switch-permission-modes) y responda la solicitud de permiso.

<h3 id="when-auto-mode-falls-back">
  Cuando el modo automático retrocede
</h3>

Cuando el modo automático no puede aprobar las acciones de su sesión, lo que sucede depende del caso:

* **Una acción bloqueada**: Claude Code muestra una notificación y enumera la acción en `/permissions` bajo la pestaña **Recently denied**, donde puede presionar `r` para reintentar con una aprobación manual. Cuando el clasificador produce [sin veredicto en la acción](/docs/es/errors#auto-mode-cannot-determine-the-safety-of-an-action), porque una verificación de seguridad separada del modo automático rechazó la solicitud propia del clasificador o su respuesta no se analizó, Claude Code niega la acción sin la notificación o la entrada **Recently denied**.
* **Bloqueos repetidos**: si el clasificador bloquea una acción 3 veces seguidas o 20 veces en total, el modo automático se pausa y Claude Code reanuda la solicitud. Aprobar la acción solicitada reanuda el modo automático. Estos umbrales no son configurables. Cualquier acción permitida reinicia el contador consecutivo, mientras que el contador total persiste para la sesión y se reinicia solo cuando su propio límite desencadena un retroceso. Claude Code no cuenta una denegación hacia ninguno de los umbrales cuando [una verificación de seguridad separada del modo automático rechaza la solicitud propia del clasificador](/docs/es/errors#auto-mode-cannot-determine-the-safety-of-an-action); la entrada vinculada cubre cómo Claude Code maneja esas denegaciones.
* **Sesiones que no pueden solicitar**: una ejecución `-p` [no interactiva](/docs/es/headless) sin un [`--permission-prompt-tool`](/docs/es/cli-reference#cli-flags) no tiene solicitud a la que retroceder. Cuando los bloqueos repetidos alcanzan un umbral, la acción no se ejecuta y Claude sigue trabajando. Lo mismo se aplica cuando [una verificación de seguridad separada del modo automático rechaza la solicitud del clasificador](/docs/es/errors#auto-mode-cannot-determine-the-safety-of-an-action). Claude Code no detiene la ejecución en ninguno de los casos.
* **No hay veredicto del servidor**: bajo [revisión del clasificador del lado del servidor](#server-side-classifier-review), Claude Code niega una acción para la que el servidor no da veredicto, y detiene el turno después de diez respuestas seguidas sin veredicto. Consulte [El servidor no devolvió veredicto de seguridad](/docs/es/errors#the-server-returned-no-safety-verdict).
* **Un cambio de modo durante una verificación**: si cambia modos de permiso mientras una verificación del clasificador está pendiente, Claude Code descarta un veredicto que el nuevo modo no habría solicitado en lugar de aplicarlo: se le solicita aprobación en su lugar, o la acción se niega automáticamente en [modo `dontAsk`](#allow-only-pre-approved-tools-with-dontask-mode).

Los bloqueos repetidos generalmente significan que el clasificador carece de contexto sobre su infraestructura. Use `/feedback` para reportar falsos positivos, o haga que un administrador [configure infraestructura de confianza](/docs/es/auto-mode-config).

<span id="how-the-classifier-evaluates-actions" />

<AccordionGroup>
  <Accordion title="Cómo el clasificador evalúa acciones">
    Cada acción pasa por un orden de decisión fijo. El primer paso coincidente gana:

    1. Las acciones que coinciden con sus [reglas de permitir, solicitar o denegar](/docs/es/permissions#manage-permissions) se resuelven inmediatamente, con estas excepciones:
       * Escrituras en [rutas protegidas](#protected-paths) se enrutan al clasificador incluso cuando una regla de permitir coincide, y también lo hacen las eliminaciones `rm` y `rmdir` dirigidas a una [ruta crítica](#critical-paths) en Claude Code v2.1.218 y posterior
       * Las herramientas MCP marcadas [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool) le solicitan directamente incluso cuando una regla de permitir coincide, y también lo hacen las herramientas de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools) en sesiones donde esa configuración llega a Claude Code
       * Un comando de shell que lleva [dominios permitidos por comando](/docs/es/sandboxing#per-command-allowed-domains-in-auto-mode) también se enruta al clasificador incluso cuando una regla de permitir coincide, porque una regla aprueba el comando, no sus hosts
       * Las reglas de solicitud que coinciden en el contenido de un comando, como `Bash(git push *)`, retroceden a una solicitud de permiso
    2. Las acciones de solo lectura y ediciones de archivo en su directorio de trabajo se aprueban automáticamente, excepto escrituras en [rutas protegidas](#protected-paths) y [la primera lectura fuera de los directorios de trabajo](#first-read-outside-the-working-directories), que le solicita
       * En una sesión con [revisión del clasificador del lado del servidor](#server-side-classifier-review), las acciones de solo lectura y comandos de shell [en sandbox](/docs/es/sandboxing#sandbox-modes) esperan esa revisión y se bloquean si la marcan
    3. Todo lo demás va al clasificador. Las herramientas de conector y herramientas MCP `requiresUserInteraction` que le solicitan directamente en el paso 1 nunca llegan al clasificador, por lo que ni una aprobación requerida por la organización ni un paso de consentimiento se aprueban automáticamente
    4. Si el clasificador bloquea, Claude recibe la razón e intenta una alternativa. En la mayoría de las sesiones la razón nombra la regla con la que coincidió el clasificador, como `[Data Exfiltration]`, en lugar de dar una explicación escrita; consulte [Revisar denegaciones](/docs/es/auto-mode-config#review-denials)

    Al entrar en modo automático, se descartan las reglas de permitir amplias que otorgan ejecución de código arbitraria:

    * `Bash(*)` o `PowerShell(*)` sin restricciones
    * Intérpretes con comodín como `Bash(python*)`
    * Comandos de ejecución del administrador de paquetes
    * Reglas de permitir `Agent`
    * Reglas de permitir [`Monitor`](/docs/es/tools-reference#monitor-tool), porque Claude Code ejecuta comandos Monitor a través del shell

    Las reglas estrechas como `Bash(npm test)` permanecen en vigor. Claude Code restaura las reglas descartadas cuando sale del modo automático. Antes de v2.1.236, Claude Code dejaba las reglas de permitir `Monitor` en vigor en modo automático, por lo que una regla que coincidía con la herramienta completa aprobaba comandos Monitor sin revisión del clasificador.

    Claude Code también ejecuta `git status` a sí mismo antes de un comando que descartaría trabajo sin confirmar, como `git reset --hard` o `rm -rf`, y muestra al clasificador si hay trabajo preparado, modificado o sin seguimiento presente. Claude Code reporta archivos sin seguimiento en esa verificación incluso cuando la configuración de git del repositorio establece `status.showUntrackedFiles=no`.

    En las solicitudes del clasificador enviadas por Claude Code mismo, el clasificador ve mensajes de usuario, llamadas de herramienta que no sean búsquedas de solo lectura como lecturas de archivo y búsquedas, y su contenido CLAUDE.md. Los resultados de herramientas se eliminan, por lo que el contenido hostil en un archivo o página web no puede manipularlo directamente. Puede anotar el resultado de una llamada con el campo `classifierContext` del hook [PostToolUse](/docs/es/hooks#annotate-a-result-for-the-auto-mode-classifier), que el clasificador lee como contexto proporcionado por la aplicación. El campo requiere Claude Code v2.1.236 o posterior.

    Una sonda separada del lado del servidor escanea los resultados de herramientas entrantes y marca contenido sospechoso antes de que Claude lo lea. Para más información sobre cómo funcionan juntas estas capas, consulte el [anuncio de modo automático](https://claude.com/blog/auto-mode) y la [inmersión profunda de ingeniería](https://www.anthropic.com/engineering/claude-code-auto-mode).
  </Accordion>

  <Accordion title="Cómo el modo automático maneja subagentes">
    El clasificador verifica el trabajo de [subagentes](/docs/es/sub-agents) en tres puntos:

    1. Antes de que comience un subagente, se evalúa la descripción de tarea delegada, por lo que una tarea de aspecto peligroso se bloquea en el momento del desove.
    2. Mientras se ejecuta el subagente, cada una de sus acciones pasa por el clasificador con las mismas reglas que la sesión principal, y cualquier `permissionMode` en el frontmatter del subagente se ignora.
    3. Cuando el subagente termina, el clasificador revisa su trabajo y su informe final antes de que el padre lea el informe. Cuando el clasificador marca el trabajo o informe del subagente, o una verificación de seguridad de API separada rechaza la revisión, el informe aún se entrega, antepuesto con una advertencia de seguridad. Cuando el clasificador no está disponible para la revisión, el informe llega con una nota para verificar el trabajo del subagente antes de actuar en consecuencia.
  </Accordion>

  <Accordion title="Costo y latencia">
    El clasificador se ejecuta en Claude Sonnet 5 de forma predeterminada en lugar de en su selección `/model`. Un modelo clasificador que Anthropic configura del lado del servidor tiene prioridad sobre ese valor predeterminado. Cuando el modelo de su sesión es Claude Sonnet 4.6, o cuando [`availableModels`](/docs/es/model-config#restrict-model-selection) excluye Sonnet 5, el clasificador se ejecuta en el modelo de su sesión en su lugar, o en un modelo Opus cuando la sesión se ejecuta en un [modelo Fable](/docs/es/model-config#work-with-fable); en proveedores que no sean la API de Anthropic, ese respaldo de Opus es el modelo Opus predeterminado del proveedor.

    La primera solicitud de modo automático de la sesión valida el valor predeterminado de Sonnet 5: si la solicitud tiene éxito, Sonnet 5 permanece como el modelo clasificador de la sesión, y si falla porque el modelo no está disponible, la sesión usa el respaldo en su lugar. Después de que esa validación se resuelve, el modelo del clasificador no cambia para la sesión.

    En planes Enterprise y en cuentas que usan la API de Claude, [Claude Platform en AWS](/docs/es/claude-platform-on-aws), Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry, las llamadas del clasificador cuentan hacia su uso de tokens. Cada verificación envía una porción de la transcripción más la acción pendiente, agregando un viaje de ida y vuelta antes de la ejecución. Las lecturas y ediciones de directorio de trabajo fuera de rutas protegidas omiten el clasificador, por lo que la sobrecarga proviene principalmente de comandos de shell y operaciones de red. Donde el servidor revisa las acciones como parte de las solicitudes del modelo de la sesión, no hay solicitudes del clasificador separadas para contar; consulte [Revisión del clasificador del lado del servidor](#server-side-classifier-review).

    El acceso a la red de sandbox no agrega solicitudes del clasificador por conexión. El clasificador juzga [los hosts que un comando nombra](/docs/es/sandboxing#per-command-allowed-domains-in-auto-mode) junto con el comando en una revisión, y Claude Code verifica cada conexión contra la lista aprobada sin llamar al clasificador nuevamente.
  </Accordion>
</AccordionGroup>

<h2 id="allow-only-pre-approved-tools-with-dontask-mode">
  Permitir solo herramientas preaprobadas con modo dontAsk
</h2>

Si establece el modo `dontAsk`, Claude Code deniega automáticamente cada llamada de herramienta que de otro modo le solicitaría. Claude aún ejecuta acciones que no necesitan aprobación en modo Manual, como lecturas de archivos dentro de sus directorios de trabajo y [comandos Bash de solo lectura](/docs/es/permissions#read-only-commands), además de acciones que coinciden con sus reglas `permissions.allow` y llamadas aprobadas por un [hook PreToolUse](/docs/es/permissions#extend-permissions-with-hooks). Utilice este modo para canalizaciones de CI o entornos restringidos donde predefine qué puede hacer Claude; la sesión nunca espera entrada. La barra de estado muestra `⏵⏵ don't ask on` mientras este modo está activo.

Claude Code deniega llamadas que coincidan con sus [reglas `ask`](/docs/es/permissions#manage-permissions) explícitas en lugar de solicitar. También deniega la herramienta integrada `AskUserQuestion` incluso si sus reglas de permitir coinciden, y hace lo mismo con las herramientas de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools) en sesiones donde esa configuración llega a Claude Code. Deniega las herramientas MCP marcadas [`_meta["anthropic/requiresUserInteraction"]`](/docs/es/mcp#require-approval-for-a-specific-tool) de la misma manera, porque su tarjeta de aprobación necesita una respuesta que este modo nunca recopila; esto requiere Claude Code v2.1.199 o posterior.

Las eliminaciones `rm` y `rmdir` dirigidas a una [ruta crítica](#critical-paths), como `rm -rf /` y `rm -rf ~`, se deniegan incluso cuando una regla de permitir coincide con ellas o un hook `PreToolUse` las permite.

Las sesiones en la nube en [Claude Code en la web](/docs/es/claude-code-on-the-web) ignoran `defaultMode: "dontAsk"`; consulte [bypassPermissions](#skip-all-checks-with-bypasspermissions-mode) para obtener detalles.

Establézcalo al inicio con la bandera:

```bash theme={null}
claude --permission-mode dontAsk
```

<h2 id="skip-all-checks-with-bypasspermissions-mode">
  Omitir todas las comprobaciones con modo bypassPermissions
</h2>

El modo `bypassPermissions` desactiva los avisos de permisos y las comprobaciones de seguridad para que las llamadas a herramientas se ejecuten inmediatamente, incluidas las escrituras en [rutas protegidas](#protected-paths).

Las [acciones que ningún modo aprueba automáticamente](#actions-no-mode-auto-approves) aún solicitan en este modo.

Dos [salvaguardas de mensajería entre sesiones](/docs/es/cross-session-messaging) aún se aplican en este modo, y en sesiones de modo plan de terminal interactivo donde los permisos de omisión están disponibles:

* El aviso de aprobación [`isolatePeerMachines`](/docs/es/settings-reference#isolatepeermachines) para mensajes a sus sesiones más allá de esta máquina aún aparece.
* Cuando no se aplica ningún valor [`crossSessionInbound`](/docs/es/cross-session-messaging#control-inbound-messages), Claude Code retiene un mensaje entrante de otra de sus sesiones para su aprobación, y entrega sin preguntar solo cuando la sesión de envío se identifica a sí misma como también omitiendo avisos de permisos. Si deja el modo de permisos mientras se retienen mensajes, Claude Code reaaplica las reglas entrantes y entrega cualquier mensaje retenido que ahora aceptan.

En sesiones de terminal interactivo con permisos de omisión disponibles, Claude Code tampoco aplica los [bloqueos del modo plan](#analyze-before-you-edit-with-plan-mode). Claude aún recibe instrucciones de planificar sin editar, pero una edición de archivo o comando de shell que intenta durante la planificación se ejecuta sin solicitar. Las [reglas `ask`](/docs/es/permissions#manage-permissions) explícitas y las eliminaciones `rm` y `rmdir` dirigidas a una [ruta crítica](#critical-paths) aún solicitan.

El modo plan mantiene sus bloqueos dondequiera que Claude Code se ejecute sin un terminal interactivo, incluidas [ejecuciones no interactivas](/docs/es/headless) con `-p`, sesiones de [Agent SDK](/docs/es/agent-sdk/permissions#plan-mode-plan), y conversaciones en el panel de chat de la [extensión VS Code](/docs/es/vs-code). Allí, `--allow-dangerously-skip-permissions` hace que `bypassPermissions` sea seleccionable más tarde.

<Warning>
  Use este modo solo en entornos aislados como contenedores, máquinas virtuales o dev containers sin acceso a Internet, donde Claude Code no pueda dañar su sistema anfitrión.
</Warning>

No puede entrar en `bypassPermissions` desde una sesión que inició sin él habilitado. Habilítelo al inicio con [`permissions.defaultMode: "bypassPermissions"`](/docs/es/settings-reference#permissions-defaultmode) o con una bandera de habilitación:

```bash theme={null}
claude --permission-mode bypassPermissions
```

La bandera `--dangerously-skip-permissions` es equivalente.

Claude Code rechaza `bypassPermissions` en una sesión que inicia con [`--restricted`](/docs/es/cli-reference#cli-flags). `--restricted` requiere Claude Code v2.1.248 o posterior.

La primera vez que inicia una sesión interactiva con este modo habilitado, Claude Code muestra un diálogo de advertencia pidiéndole que acepte la responsabilidad por acciones tomadas sin comprobaciones de permisos. Claude Code guarda su aceptación en la configuración de usuario, por lo que el diálogo aparece solo una vez. Si rechaza, Claude Code sale. En [modo no interactivo](/docs/es/headless) no se muestra diálogo, y una [sesión en segundo plano](/docs/es/agent-view) iniciada con `--bg` se rechaza hasta que haya aceptado el diálogo en una sesión interactiva.

En Linux y macOS, Claude Code se niega a iniciarse en este modo cuando se ejecuta como root o bajo `sudo`:

```text theme={null}
--dangerously-skip-permissions cannot be used with root/sudo privileges for security reasons
```

La verificación se omite automáticamente dentro de un sandbox reconocido. Para ejecutarse de forma autónoma en un contenedor, use la configuración de [dev container](/docs/es/devcontainer), que ejecuta Claude Code como un usuario no root.

[Claude Code en la web](/docs/es/claude-code-on-the-web) no respeta `defaultMode: "bypassPermissions"` o `"dontAsk"` de sus archivos de configuración, por lo que la configuración registrada en un repositorio no puede iniciar una sesión en la nube en modo bypass-permissions. La configuración se ignora silenciosamente y la sesión comienza en el modo de permisos mostrado en el menú desplegable de modo en su lugar. Consulte [Cambiar modos de permisos](#switch-permission-modes) para ver qué modos ofrecen las sesiones en la nube.

<Warning>
  `bypassPermissions` no ofrece protección contra inyección de solicitudes o acciones no intencionadas. Para comprobaciones de seguridad de fondo con muchos menos avisos de permisos, use [modo automático](#eliminate-prompts-with-auto-mode) en su lugar. Los administradores pueden bloquear este modo estableciendo `permissions.disableBypassPermissionsMode` en `"disable"` en [configuración administrada](/docs/es/managed-settings).
</Warning>

<h2 id="protected-paths">
  Rutas protegidas
</h2>

Las escrituras en un pequeño conjunto de rutas nunca se aprueban automáticamente, excepto en modo `bypassPermissions` y en sesiones de terminal interactivas en modo plan con [permisos de omisión](#skip-all-checks-with-bypasspermissions-mode) disponibles. Esto previene la corrupción accidental del estado del repositorio y la configuración de Claude.

| Modo                     | Escrituras en rutas protegidas                                                                                                                                                                                                                                                                                        |
| :----------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`, `acceptEdits` | Solicitadas                                                                                                                                                                                                                                                                                                           |
| `plan`                   | Permitidas en sesiones de terminal interactivas con [permisos de omisión](#skip-all-checks-with-bypasspermissions-mode) disponibles. De lo contrario, enrutadas al clasificador cuando [modo automático](#eliminate-prompts-with-auto-mode) está disponible durante la planificación, y solicitadas cuando no lo está |
| `auto`                   | Enrutadas al clasificador                                                                                                                                                                                                                                                                                             |
| `dontAsk`                | Denegadas                                                                                                                                                                                                                                                                                                             |
| `bypassPermissions`      | Permitidas                                                                                                                                                                                                                                                                                                            |

En una sesión iniciada con [`--restricted`](/docs/es/cli-reference#cli-flags), que requiere Claude Code v2.1.248 o posterior, el clasificador no puede aprobar escrituras en rutas protegidas.

Las reglas [`permissions.allow`](/docs/es/permissions#manage-permissions) en archivos de configuración no pre-aprueban escrituras en rutas protegidas. La verificación de seguridad se ejecuta antes de que Claude Code evalúe las reglas de permitir desde la configuración, por lo que una entrada como `Edit(.claude/**)` en `~/.claude/settings.json` o `.claude/settings.json` no cambia el resultado por modo en la tabla anterior. En modos que solicitan, el aviso para una escritura en `.claude/` ofrece **Sí, y permitir que Claude edite su propia configuración para esta sesión**, que aprueba escrituras posteriores en `.claude/` en esa sesión sin solicitar nuevamente.

Directorios protegidos:

* `.git`
* `.config/git`
* `.vscode`
* `.idea`
* `.husky`
* `.cargo`
* `.devcontainer`
* `.yarn`
* `.mvn`
* `.claude`, excepto por `.claude/worktrees` donde Claude almacena sus propios git worktrees

Archivos protegidos:

* `.gitconfig`, `.gitmodules`
* `.bashrc`, `.bash_profile`, `.bash_login`, `.bash_aliases`, `.bash_logout`, `.zshrc`, `.zprofile`, `.zshenv`, `.zlogin`, `.zlogout`, `.profile`, `.envrc`
* `.npmrc`, `.yarnrc`, `.yarnrc.yml`, `.pnp.cjs`, `.pnp.loader.mjs`, `.pnpmfile.cjs`, `bunfig.toml`, `.bunfig.toml`
* `.bazelrc`, `.bazelversion`, `.bazeliskrc`
* `.pre-commit-config.yaml`, `lefthook.yml`, `lefthook.yaml`, `.lefthook.yml`, `.lefthook.yaml`
* `gradle-wrapper.properties`, `maven-wrapper.properties`
* `.devcontainer.json`
* `.ripgreprc`, `pyrightconfig.json`
* `.mcp.json`, `.claude.json`

<h2 id="critical-paths">
  Rutas críticas
</h2>

Claude Code nunca permite que una regla [`permissions.allow`](/docs/es/permissions#manage-permissions) o un [hook `PreToolUse`](/docs/es/permissions#extend-permissions-with-hooks) que devuelve `"allow"` apruebe un comando `rm` o `rmdir` que se dirija a una ruta crítica, incluso en modos que omiten otros avisos. Este cortacircuitos protege contra errores del modelo. Una regla de denegación coincidente aún bloquea el comando completamente.

Qué sucede en su lugar depende de su modo de permisos:

| Modo                     | Qué hace Claude Code con una eliminación de ruta crítica                                                                                                                                                   |
| :----------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`, `acceptEdits` | Le solicita que la apruebe                                                                                                                                                                                 |
| `plan`                   | Le solicita que la apruebe. Con [modo automático disponible durante la planificación](#analyze-before-you-edit-with-plan-mode) y sin permisos de omisión disponibles, la envía al clasificador en su lugar |
| `auto`                   | La envía al [clasificador](#eliminate-prompts-with-auto-mode)                                                                                                                                              |
| `dontAsk`                | La deniega                                                                                                                                                                                                 |
| `bypassPermissions`      | Le solicita que la apruebe                                                                                                                                                                                 |

Si una [regla `ask`](/docs/es/permissions#manage-permissions) explícita coincide con el comando, Claude Code le solicita incluso en modo `auto`. En modos que solicitan, un [hook `PermissionRequest`](/docs/es/hooks#permissionrequest) puede responder el aviso de la manera que responde cualquier otro.

Claude Code trata un objetivo `rm` o `rmdir` como una ruta crítica cuando es cualquiera de los siguientes:

* La raíz del sistema de archivos
* Directorios de nivel superior, lo que significa cualquier hijo directo de la raíz, como `/usr`, `/etc` o `/data`
* Su directorio de inicio
* Raíces de unidad de Windows y sus directorios de nivel superior, como `C:\` y `C:\Windows`
* Su directorio de trabajo y sus padres
* Sus directorios de trabajo adicionales y sus padres, pero solo cuando la eliminación es un glob bajo uno de ellos, como `rm -rf <dir>/*`. `rm -rf <dir>` en el directorio en sí no desencadena esta verificación

Claude Code también trata un glob o barra diagonal final directamente bajo una variable de shell, como `rm -rf "$DIR"/*`, como una eliminación de ruta crítica, porque el comando se convierte en una eliminación desde la raíz del sistema de archivos cuando la variable está vacía.

El aviso para este caso de variable nombra el `rm` marcado y dice cómo reescribirlo para que la verificación pase:

* Para una variable como `$DIR`, proteja cada expansión para que el shell se detenga con un error cuando la variable no esté configurada o esté vacía, como en `rm -rf "${DIR:?}"/*`, o use una ruta literal
* Para una variable que normalmente está configurada, como `$HOME`, use una ruta literal

Una eliminación cuyas expansiones están todas protegidas de esa manera no es una eliminación de ruta crítica, por lo que en modo `bypassPermissions` se ejecuta sin un aviso.

Ocultar la eliminación dentro de una subshell con `(...)`, un grupo de llaves con `{ ...; }`, sustitución de comandos con `$(...)` o comillas invertidas, o sustitución de procesos con `<(...)`, no omite la verificación. Claude Code encuentra una eliminación de ruta crítica ya sea que esté dentro de la forma anidada, como en `(rm -rf ~)` o `echo "$(rm -rf ~)"`, o en otro lugar del mismo comando.

<h3 id="remove-item-in-powershell">
  Remove-Item en PowerShell
</h3>

Cuando habilita la [herramienta PowerShell](/docs/es/tools-reference#powershell-tool), Claude Code da a `Remove-Item` su propia verificación, separada de la lista de rutas críticas `rm`. El resultado depende del objetivo, y se aplica el primer caso coincidente:

* **Rutas del sistema**: la raíz del sistema de archivos y sus directorios de nivel superior, raíces de unidad y sus directorios de nivel superior, y su directorio de inicio. Claude Code deniega el comando en todos los modos, sin solicitarle.
* **Comodines**: un `*` desnudo, o cualquier objetivo que termine en `/*` o `\*`, incluido un glob bajo una variable de shell como `$dir/*`. Claude Code deniega el comando en todos los modos, sin solicitarle, antes de que el [clasificador](#eliminate-prompts-with-auto-mode) lo vea.
* **Su directorio de trabajo o uno de sus padres, con `-Recurse`**: Claude Code trata el comando como cualquier otro que necesita aprobación en su modo de permisos, por lo que le solicita en modos que solicitan, lo envía al clasificador en modo `auto` y lo deniega en modo `dontAsk`. El modo `bypassPermissions` omite esta verificación.

<h2 id="see-also">
  Véase también
</h2>

* [Permisos](/docs/es/permissions): reglas de permitir, solicitar y denegar; políticas administradas
* [Configurar modo automático](/docs/es/auto-mode-config): indique al clasificador qué infraestructura confía su organización
* [Hooks](/docs/es/hooks): lógica de permisos personalizada mediante hooks `PreToolUse` y `PermissionRequest`
* [Seguridad](/docs/es/security): salvaguardas y mejores prácticas
* [Sandboxing](/docs/es/sandboxing): aislamiento del sistema de archivos y red para comandos Bash
* [Modo no interactivo](/docs/es/headless): ejecutar Claude Code con la bandera `-p`
