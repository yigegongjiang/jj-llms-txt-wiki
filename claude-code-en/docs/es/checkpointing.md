> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Checkpointing

> Realiza un seguimiento, revierte y resume las ediciones y conversaciones de Claude para gestionar el estado de la sesión.

Claude Code realiza un seguimiento automático de las ediciones de archivos de Claude mientras trabaja, permitiéndole deshacer rápidamente cambios y revertir a estados anteriores si algo se sale de control.

<h2 id="how-checkpoints-work">
  Cómo funciona el checkpointing
</h2>

Mientras trabaja con Claude, el checkpointing captura automáticamente el estado de su código antes de cada solicitud que envía y que inicia un turno.

<h3 id="automatic-tracking">
  Seguimiento automático
</h3>

Claude Code realiza un seguimiento de todos los cambios realizados por sus herramientas de edición de archivos:

* Cada solicitud que envía y que inicia un turno crea un nuevo checkpoint
* Claude Code mantiene snapshots de archivos para los 100 checkpoints más recientes en una sesión. Descartar un checkpoint anterior elimina los archivos de snapshot que ningún checkpoint restante referencia, excepto el primer snapshot de cada archivo, que la extensión de VS Code utiliza como línea base para sus diffs de sesión.
* Claude Code guarda checkpoints con la conversación, por lo que aún puede ejecutar `/rewind` después de reanudar una sesión
* Claude Code elimina los snapshots de archivos de una sesión en el [barrido de retención](/docs/es/claude-directory#cleaned-up-automatically), por defecto aproximadamente 30 días después de que la sesión guardó uno por última vez. Revertir a un checkpoint cuyos snapshots se han eliminado puede fallar con [`No files were restored`](/docs/es/errors#no-files-were-restored). Para mantener snapshots más tiempo, establezca [`cleanupPeriodDays`](/docs/es/settings-reference#cleanupperioddays).

<h3 id="rewind-and-summarize">
  Revertir y resumir
</h3>

Ejecute `/rewind`, o presione `Esc` dos veces cuando el campo de entrada de solicitud esté vacío, para abrir el menú de rewind.

<Note>
  Si el campo de entrada de solicitud contiene texto, presionar `Esc` dos veces lo borra en lugar de abrir el menú. El texto borrado se guarda en su historial de entrada, por lo que presione `Arriba` para recuperarlo después de terminar en el menú de rewind.
</Note>

El menú de rewind enumera cada solicitud que envió durante la sesión, excepto [mensajes que se unieron a un turno en ejecución](#messages-sent-mid-turn-not-checkpointed). Seleccione el punto en el que desea actuar y luego elija una acción:

* **Restaurar código y conversación**: revierte tanto el código como la conversación a ese punto
* **Restaurar conversación**: revierte a ese mensaje mientras mantiene el código actual
* **Restaurar código**: revierte los cambios de archivo mientras mantiene la conversación
* **Resumir desde aquí**: comprime la conversación desde este punto en adelante en un resumen, liberando espacio de context window
* **Resumir hasta aquí**: comprime la conversación antes de este punto en un resumen, manteniendo los mensajes posteriores intactos
* **Cancelar**: regresa a la lista de mensajes sin hacer cambios

Las dos opciones de restauración de código aparecen solo cuando el checkpoint seleccionado tiene cambios de archivo rastreados para revertir. Si no se capturaron ediciones de archivo después de ese punto, el menú ofrece solo **Restaurar conversación**, las opciones de resumir y **Cancelar**.

Después de restaurar la conversación o elegir Resumir desde aquí, la solicitud original del mensaje seleccionado se restaura en el campo de entrada para que pueda reenviarlo o editarlo.

Al elegir Resumir hasta aquí, se queda al final de la conversación con la entrada vacía. Con cualquiera de las opciones de resumir, aparece un marcador de **Conversación resumida** en la conversación donde se comprimieron los mensajes.

<h4 id="rewind-past-a-cleared-conversation">
  Revertir una conversación borrada
</h4>

Si ejecutó `/clear` anteriormente en el mismo proceso de Claude Code, el menú de rewind muestra una entrada adicional en la parte superior de la lista etiquetada como `/resume <session-id> (sesión anterior)`. Selecciónela para reanudar la conversación que estaba activa antes de que se ejecutara `/clear`. La entrada está disponible hasta que salga de Claude Code o reanude una sesión diferente.

<h4 id="guide-a-summary">
  Guiar un resumen
</h4>

Resumir no cambia archivos en el disco, y los mensajes originales permanecen en la transcripción de la sesión, por lo que Claude aún puede hacer referencia a los detalles. Para guiar en qué se enfoca el resumen, resalte una opción de **Resumir** con las teclas de flecha e ingrese instrucciones donde la fila dice **agregar contexto (opcional)**, luego presione `Intro`. Seleccionar la opción con su tecla numérica resume inmediatamente sin instrucciones.

<Note>
  Resumir lo mantiene en la misma sesión y comprime el contexto, como un `/compact` dirigido. Para ramificarse e intentar un enfoque diferente mientras preserva la sesión original intacta, use [`/branch`](/docs/es/sessions#branch-a-session) o `claude --continue --fork-session` en su lugar.
</Note>

<h2 id="common-use-cases">
  Casos de uso comunes
</h2>

Los checkpoints son particularmente útiles cuando:

* **Explorar alternativas**: pruebe diferentes enfoques de implementación sin perder su punto de partida
* **Recuperarse de errores**: deshaga rápidamente cambios que introdujeron errores o rompieron la funcionalidad
* **Iterar en características**: experimente con variaciones sabiendo que puede revertir a estados que funcionan
* **Liberar espacio de contexto**: resuma una sesión de depuración detallada desde el punto medio en adelante, manteniendo sus instrucciones iniciales intactas

<h2 id="limitations">
  Limitaciones
</h2>

<h3 id="bash-command-changes-not-tracked">
  Los cambios de comandos Bash no se rastrean
</h3>

El checkpointing no rastrea archivos modificados por comandos Bash. Por ejemplo, si Claude Code ejecuta:

```bash theme={null}
rm file.txt
mv old.txt new.txt
cp source.txt dest.txt
```

Estas modificaciones de archivo no se pueden deshacer a través de rewind. Solo se rastrean las ediciones de archivo directo realizadas a través de las herramientas de edición de archivos de Claude.

<h3 id="subagent-edits-not-restored">
  Los cambios de subagentes no se restauran
</h3>

Un [subagente](/docs/es/sub-agents) realiza ediciones con las herramientas de edición de archivos de Claude, pero Claude Code normalmente no captura esas ediciones en los checkpoints de su sesión. Si el rewind las restaura depende de cómo se ejecute el subagente:

* **Skill forked en primer plano**: un [skill con `context: fork`](/docs/es/skills#run-skills-in-a-subagent) que se ejecuta en primer plano edita su árbol de trabajo durante su propio turno, por lo que el rewind restaura sus ediciones como de costumbre. Establezca `background: false` para ejecutar un fork en primer plano; algunas situaciones, [listadas en la página de skills](/docs/es/skills#run-skills-in-a-subagent), lo ejecutan allí independientemente de la configuración.
* **Cualquier otro subagente**: el rewind no restaura las ediciones. Use git para revertirlas. Esto incluye un skill forked que se ejecuta en segundo plano, el predeterminado, y una ejecución de [`/code-review --fix`](/docs/es/code-review) en segundo plano.

<h3 id="external-changes-not-tracked">
  Los cambios externos no se rastrean
</h3>

El checkpointing solo rastrea archivos que han sido editados dentro de la sesión actual. Los cambios manuales que realiza en archivos fuera de Claude Code y las ediciones de otras sesiones concurrentes normalmente no se capturan, a menos que modifiquen los mismos archivos que la sesión actual.

<h3 id="messages-sent-mid-turn-not-checkpointed">
  Los mensajes enviados a mitad de turno no se someten a checkpoint
</h3>

Cuando un mensaje que [pone en cola mientras Claude trabaja](/docs/es/interactive-mode#queue-messages-while-claude-works) llega a Claude dentro del turno en ejecución, se une a ese turno en lugar de iniciar uno nuevo. El mensaje aparece en la conversación, pero Claude Code no crea un checkpoint para él, y el menú de rewind no lo enumera. Un mensaje en cola que Claude Code envía como su propio turno obtiene un checkpoint como de costumbre.

Para eliminar tal mensaje, o deshacer las ediciones que Claude realizó después de que llegara, retroceda al prompt que inició el turno. Eso retrocede todo el turno, incluido el trabajo que Claude realizó antes de que llegara su mensaje.

<h3 id="symlinked-and-hard-linked-paths-not-restored">
  Las rutas con enlaces simbólicos y enlaces duros no se restauran
</h3>

El checkpointing no revierte archivos con enlaces simbólicos o enlaces duros. Cuando selecciona **Restore code** o **Restore code and conversation** del menú `/rewind`, Claude Code omite cualquier ruta rastreada que sea un enlace simbólico o un enlace duro y muestra una advertencia `Restored the code, but skipped N files`. Los archivos omitidos mantienen su contenido actual. Para deshacer los cambios de la sesión en uno de ellos, pida a Claude que revierta la edición o edite el archivo usted mismo. Los archivos de configuración que un gestor de dotfiles enlaza simbólicamente en su proyecto y los archivos que pnpm enlaza duramente en su lugar caen en esta categoría.

Para ver qué rutas omite una restauración, active el registro de depuración con `/debug` antes de restaurar: el registro de depuración en `~/.claude/debug/<session-id>.txt` nombra cada ruta omitida. Para cada razón de omisión y los pasos de recuperación, consulte [la entrada skipped-files en la referencia de errores](/docs/es/errors#restored-the-code-but-skipped-files).

<h3 id="not-a-replacement-for-version-control">
  No es un reemplazo para el control de versiones
</h3>

Los checkpoints están diseñados para recuperación rápida a nivel de sesión. Para historial de versiones permanente y colaboración, continúe usando control de versiones, como Git, para commits, ramas e historial a largo plazo.

<h2 id="see-also">
  Ver también
</h2>

* [Modo interactivo](/docs/es/interactive-mode) - Atajos de teclado y controles de sesión
* [Comandos](/docs/es/commands) - Acceso a checkpoints usando `/rewind`
* [Referencia de CLI](/docs/es/cli-reference) - Opciones de línea de comandos
