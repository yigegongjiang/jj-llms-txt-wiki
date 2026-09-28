> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ejecutar Claude Code detrás de un lanzador corporativo

> Enrute los procesos que Claude Code inicia desde su propio binario, incluido el servicio de fondo y cada sesión de vista de agente, a través de un lanzador requerido con CLAUDE_CODE_PROCESS_WRAPPER o la configuración processWrapper.

Algunas organizaciones requieren que cada proceso en una estación de trabajo se inicie a través de un lanzador obligatorio. El lanzador aplica la zona de pruebas, los controles de red o la inyección de credenciales en las que depende la postura de seguridad de la empresa, y un binario que se inicia sin ella es una violación de política.

`CLAUDE_CODE_PROCESS_WRAPPER` inicia cada proceso que Claude Code lanza desde su propio binario a través de su lanzador: el servicio de fondo, cada sesión que aloja en [vista de agente](/docs/es/agent-view), y los relanzamientos de Claude Code después de una actualización. Establézcalo en la ruta absoluta de su lanzador, y Claude Code ejecuta el lanzador con el comando de Claude Code como sus argumentos.

Un lanzador que envuelve el comando `claude` en su `PATH` no puede alcanzar el servicio de fondo o las sesiones que aloja, porque se inician desde la ruta directa del binario sin buscar `claude`.

<Note>
  `CLAUDE_CODE_PROCESS_WRAPPER` requiere Claude Code v2.1.208 o posterior. Las versiones anteriores ignoran la variable e inician cada proceso sin envolver. La configuración equivalente [`processWrapper`](/docs/es/settings-reference#processwrapper) requiere v2.1.210 o posterior. Las versiones anteriores la ignoran como una clave desconocida, no aplican ningún lanzador y no reportan ningún error.

  Después de implementar cualquiera de las dos formas, utilice el [paso Verificar](#set-up-the-launcher) para confirmar que la versión en ejecución la aplica.
</Note>

<h2 id="what-the-launcher-covers">
  Qué cubre el lanzador
</h2>

Con `CLAUDE_CODE_PROCESS_WRAPPER` establecido, Claude Code inicia cada uno de los siguientes procesos a través de su lanzador:

* El servicio de fondo que `claude agents` y las sesiones de fondo inician bajo demanda.
* El host de terminal y la sesión de Claude Code dentro de cada fila de vista de agente, incluidas las sesiones de espera en caliente que el servicio mantiene listas.
* Las sesiones que el servicio reinicia después de una actualización o un bloqueo.
* El relanzamiento que Claude Code realiza de sí mismo para terminar de instalar una actualización, incluida la acción de reinicio para actualización de la vista de agente.
* Los procesos de sesión que [Control Remoto](/docs/es/remote-control) inicia. Requiere Claude Code v2.1.210 o posterior.
* Las sesiones de compañero de panel dividido que [equipos de agentes](/docs/es/agent-teams) inician en tmux o iTerm2. Los paneles de compañero son interactivos en lugar de procesos de fondo, pero Claude Code los inicia desde su propio binario, por lo que el lanzador los cubre. Requiere Claude Code v2.1.210 o posterior.

En Windows, la variable se ignora: el contrato del lanzador depende de `exec`, que Windows no admite. Una máquina Windows con la variable establecida ejecuta cada proceso sin envolver y continúa funcionando, y la única señal es una advertencia en el [registro de depuración](/docs/es/troubleshooting). Si su política de lanzador cubre Windows, la variable no la satisface allí: cuente las máquinas Windows como sin envolver cuando planifique el despliegue.

<h3 id="processes-that-start-outside-the-launcher">
  Procesos que se inician fuera del lanzador
</h3>

Los siguientes procesos no se inician a través del lanzador:

* Un [servicio de fondo instalado](/docs/es/agent-view#the-supervisor-process) cuya unidad se escribió antes de que se configurara el lanzador: `launchd` o `systemd` inicia ese proceso desde su archivo de unidad. `/status` y `claude daemon status` advierten cuando el servicio en ejecución y el lanzador configurado no coinciden, y las sesiones que el servicio genera aún se inician a través del lanzador una vez que el servicio se reinicia con la variable en su configuración.
* Una sesión que usted inicia usted mismo en una terminal, que se ejecuta como la invocó. Para cubrir estas sesiones, coloque un script llamado `claude` en un directorio anterior en `PATH` que ejecute su lanzador con el binario real; no reemplace el enlace simbólico administrado. El servicio de fondo y sus sesiones se inician sin una búsqueda de `PATH`, por lo que los dos lanzadores no se apilan allí.
* El primer proceso de un enlace profundo `claude-cli://`, que el controlador de protocolo del sistema operativo inicia directamente. Todo lo que esa sesión inicia en el fondo después se ejecuta a través del lanzador. Para cerrar completamente este camino, [evite el registro del controlador](/docs/es/deep-links#registration-and-supported-platforms) con la configuración `disableDeepLinkRegistration`.
* El relanzamiento que `--worktree` combinado con `--tmux` realiza: el multiplexor de terminal inicia ese panel, no el binario de Claude Code.
* El host de mensajería nativa que [Claude en Chrome](/docs/es/chrome) registra: el navegador lo inicia, no el binario de Claude Code.

<h3 id="helper-process-names-in-process-monitors">
  Nombres de procesos auxiliares en monitores de procesos
</h3>

Con un lanzador configurado, `ps` y Activity Monitor ya no muestran las etiquetas `claude bg-pty-host` y `claude bg-spare` de Claude Code para los procesos auxiliares de fondo, porque el `exec` del lanzador reconstruye la lista de argumentos. Perder las etiquetas es un efecto secundario, no un ocultamiento: los procesos son de otra manera sin cambios, y Claude Code identifica sus propios procesos por ruta binaria, nunca por nombre de visualización.

<h2 id="set-up-the-launcher">
  Configurar el lanzador
</h2>

<Steps>
  <Step title="Escribir el script del lanzador">
    Cree un script ejecutable en una ruta absoluta, como `/opt/corp/launcher`. Claude Code lo ejecuta con el comando completo de Claude Code como sus argumentos, y el script debe terminar llamando a `exec "$@"` para que se reemplace a sí mismo con Claude Code:

    ```bash theme={null}
    #!/bin/sh
    # La configuración de su organización: ingrese a la zona de pruebas, aplique
    # controles de red o inyecte credenciales.
    exec "$@"
    ```

    Hágalo ejecutable con `chmod +x`. La porción de configuración es lo que su lanzador debe hacer antes de que Claude Code se ejecute; [el contrato del lanzador](#the-launcher-contract) a continuación enumera las reglas que el script debe seguir.

    <Note>
      Si anteriormente reemplazó el enlace simbólico `~/.local/bin/claude` con su lanzador, restaure el enlace simbólico original en el mismo cambio. Un enlace simbólico reemplazado hace que la primera sesión envuelta inicie el servicio de fondo a través de ambos lanzadores a la vez, y coloca la instalación en un estado administrado externamente: `/doctor` lo reporta, la actualización automática deja el archivo en su lugar, y la limpieza de versiones antiguas permanece deshabilitada hasta que el instalador administre esa ruta nuevamente.
    </Note>
  </Step>

  <Step title="Establecer CLAUDE_CODE_PROCESS_WRAPPER en configuración">
    Establezca la variable en el bloque `env` de un archivo de configuración para que el servicio de fondo desacoplado la herede. Una `export` de shell no es suficiente: el servicio de fondo se inicia bajo demanda, sobrevive a su shell y nunca vuelve a leer perfiles de shell.

    Para una máquina, agréguelo a `~/.claude/settings.json`. Para implementarlo en cada máquina de su organización, coloque el mismo bloque en [configuración administrada](/docs/es/managed-settings):

    ```json theme={null}
    {
      "env": {
        "CLAUDE_CODE_PROCESS_WRAPPER": "/opt/corp/launcher"
      }
    }
    ```

    Cuando más de una fuente establece la variable, el valor de configuración administrada anula tanto `~/.claude/settings.json` como un valor exportado en el shell, por lo que los usuarios no pueden apuntar auto-generados a un lanzador diferente.

    La configuración [`processWrapper`](/docs/es/settings-reference#processwrapper) lleva el mismo valor que una clave de configuración nombrada de nivel superior. Establézcala cuando su organización envíe configuración como claves individuales en lugar de un bloque `env`. La configuración `processWrapper` requiere Claude Code v2.1.210 o posterior. El siguiente archivo de configuración establece el mismo lanzador a través de la clave:

    ```json theme={null}
    {
      "processWrapper": "/opt/corp/launcher"
    }
    ```

    `CLAUDE_CODE_PROCESS_WRAPPER` tiene precedencia cuando ambas están establecidas.

    Debido a que `processWrapper` es una configuración nombrada, una organización que la entrega a través de [configuración administrada remota](/docs/es/managed-settings#delivery-mechanisms) la ve listada en el [diálogo de aprobación de seguridad](/docs/es/server-managed-settings#security-approval-dialogs) junto con las otras configuraciones que ejecutan ejecutables suministrados por el administrador.

    La configuración de proyecto y local no puede configurar el lanzador. Un archivo confirmado en un repositorio no debe poder poner un binario frente a cada proceso de Claude Code en la máquina, por lo que Claude Code ignora `CLAUDE_CODE_PROCESS_WRAPPER` en `.claude/settings.json` o `.claude/settings.local.json` con una advertencia en el [registro de depuración](/docs/es/troubleshooting), y nunca lee la clave `processWrapper` de esos archivos.
  </Step>

  <Step title="Reiniciar el servicio de fondo y sus sesiones">
    Un servicio de fondo en ejecución y cualquier sesión `claude` abierta leen la variable una vez al inicio, por lo que continúan lanzando procesos sin envolver hasta que se reinician. Ejecute `claude daemon stop --any` para detener el servicio bajo demanda; el siguiente comando que lo necesite, como `claude agents`, inicia uno envuelto. Un [servicio instalado](/docs/es/agent-view#the-supervisor-process) toma `claude daemon stop` sin `--any`. Luego reinicie sus sesiones `claude` abiertas.

    En máquinas que no puede reiniciar manualmente, la primera sesión iniciada después del empuje de configuración retira automáticamente un servicio bajo demanda sin envolver restante. Una máquina donde no se inicia ninguna sesión nueva mantiene su servicio sin envolver hasta que lo haga, y un servicio instalado siempre necesita el reinicio en este paso.
  </Step>

  <Step title="Verificar">
    Ejecute `/status` en una sesión: la entrada Self-exec muestra el comando de lanzamiento resuelto y advierte cuando el servicio de fondo en ejecución no coincide con él. `claude daemon status` imprime la misma información desde el shell, incluido después de que desestablezca la variable, cuando `/status` ya no muestra la entrada.
  </Step>
</Steps>

<h2 id="the-launcher-contract">
  El contrato del lanzador
</h2>

Cuando el lanzador no puede ejecutarse, Claude Code se niega a iniciar el proceso en lugar de iniciarlo sin envolver. En Windows, [la variable se ignora](#what-the-launcher-covers) y los procesos se inician sin envolver. Claude Code mantiene el script a estas reglas:

* **Terminar con `exec "$@"`.** Un lanzador que bifurca un hijo y sale deja un proceso de Claude Code huérfano que el servicio de fondo no puede rastrear. La vista de agente marca tal sesión como fallida con un mensaje que nombra el lanzador, y el servicio recoge lo que el lanzador dejó atrás.
* **No reordene, absorba o anteponga argumentos.** El primer argumento es el binario de Claude Code y todo después de él es su argv.
* **Pase cada variable de entorno heredada a través de `exec`.** Agregar variables, como credenciales inyectadas, está bien; descartar las heredadas no.
  * Los tokens de autenticación por sesión, la selección de modelo y proveedor, y `CLAUDE_CODE_PROCESS_WRAPPER` en sí viajan en el entorno heredado, por lo que un lanzador que lo reconstruye desde una lista de permitidos rompe las sesiones que inicia, y `/status` reporta una falta de coincidencia del lanzador.
  * Si el lanzador debe entrar en un espacio de nombres o zona de pruebas que reinicia el entorno, vuelva a exportar el entorno heredado dentro de él textualmente.
* **Alcance `exec` dentro de aproximadamente tres segundos cada vez que se ejecute el lanzador.** Un envío de fondo frío ejecuta el lanzador dos veces en serie antes del primer byte de salida, así que haga trabajo lento como un intercambio de inicio de sesión único de manera perezosa o desde un caché.
* **Tolere ser invocado desde dentro de sí mismo.** Claude Code aplica el lanzador a cada auto-generado anidado, por lo que un lanzador que adquiere un recurso exclusivo debe detectar que ya lo posee.
* **No escriba en la terminal antes de que Claude Code se inicie.** Cualquier cosa impresa antes del `exec` se reporta como la causa del bloqueo si la sesión muere antes de inicializarse.

<h3 id="format-of-the-launcher-value">
  Formato del valor del lanzador
</h3>

`CLAUDE_CODE_PROCESS_WRAPPER` y la configuración `processWrapper` toman el mismo formato. Para la mayoría de los lanzadores, el valor es la ruta absoluta del script, como `/opt/corp/launcher`.

Para pasar argumentos de su lanzador, escríbalos después de la ruta. Claude Code analiza el valor como una lista de argumentos, no como un comando de shell:

* El espacio en blanco separa tokens, y las comillas dobles agrupan un token que contiene espacios.
* Un valor que comienza con `[` se lee como una matriz de cadenas JSON, como `["/opt/corp/launcher", "--profile", "cc"]`.
* La sintaxis de shell no funciona: no hay expansión de variables ni globbing, y un operador sin comillas como `;`, `|`, `&`, o `$(` se rechaza como un error de configuración en lugar de reinterpretarse.

Cuando el valor no se puede usar, Claude Code se niega a iniciar el proceso afectado y [reporta la razón](/docs/es/errors#claude_code_process_wrapper-launcher-errors).

<h2 id="relationship-to-claude_code_shell_prefix">
  Relación con `CLAUDE_CODE_SHELL_PREFIX`
</h2>

`CLAUDE_CODE_PROCESS_WRAPPER` envuelve los propios procesos de Claude Code y pasa el comando como tokens argv separados para que el lanzador `exec`. [`CLAUDE_CODE_SHELL_PREFIX`](/docs/es/env-vars) envuelve los comandos de shell que Claude Code ejecuta en su nombre, como llamadas de herramienta Bash, hooks, y los comandos que inician servidores MCP de stdio, y pasa cada uno como una única cadena entrecomillada de shell en `$1` para que el contenedor vuelva a evaluar. Un lanzador escrito para uno no funciona como el otro.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Vista de agente](/docs/es/agent-view): las sesiones de fondo y el proceso supervisor que cubre el lanzador
* [Variables de entorno](/docs/es/env-vars): la entrada de referencia `CLAUDE_CODE_PROCESS_WRAPPER`
* [Configuración administrada](/docs/es/permissions#managed-settings): entregar el bloque `env` en toda una flota
* [Referencia de errores del lanzador](/docs/es/errors#claude_code_process_wrapper-launcher-errors): los mensajes de rechazo y cómo recuperarse
