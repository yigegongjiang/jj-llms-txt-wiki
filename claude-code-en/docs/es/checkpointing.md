> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Checkpointing

> Realiza un seguimiento, revierte y resume las ediciones y conversaciones de Claude para gestionar el estado de la sesión.

Claude Code realiza un seguimiento automático de las ediciones de archivos de Claude mientras trabaja, permitiéndole deshacer rápidamente cambios y revertir a estados anteriores si algo se sale de control.

<h2 id="how-checkpoints-work">
  Cómo funciona el checkpointing
</h2>

Mientras trabaja con Claude, el checkpointing captura automáticamente el estado de su código antes de cada solicitud del usuario. Esta red de seguridad le permite realizar tareas ambiciosas y a gran escala sabiendo que siempre puede volver a un estado de código anterior.

<h3 id="automatic-tracking">
  Seguimiento automático
</h3>

Claude Code realiza un seguimiento de todos los cambios realizados por sus herramientas de edición de archivos:

* Cada solicitud del usuario crea un nuevo checkpoint
* Claude Code mantiene snapshots de archivos para los 100 checkpoints más recientes en una sesión. Descartar un checkpoint anterior elimina los archivos de snapshot que ningún checkpoint restante referencia, excepto el primer snapshot de cada archivo, que la extensión de VS Code utiliza como línea base para sus diffs de sesión. Antes de v2.1.208, esos archivos de snapshot reemplazados permanecían en el disco hasta que la sesión se limpiaba.
* Los checkpoints se guardan con la conversación, por lo que una sesión reanudada aún puede `/rewind` a ellos
* Se limpian automáticamente junto con las sesiones después de 30 días (configurable)

<h3 id="rewind-and-summarize">
  Revertir y resumir
</h3>

Ejecute `/rewind`, o presione `Esc` dos veces cuando el campo de entrada de solicitud esté vacío, para abrir el menú de rewind.

<Note>
  Si el campo de entrada de solicitud contiene texto, presionar `Esc` dos veces lo borra en lugar de abrir el menú. El texto borrado se guarda en su historial de entrada, por lo que presione `Arriba` para recuperarlo después de terminar en el menú de rewind.
</Note>

El menú de rewind enumera cada solicitud que envió durante la sesión. Seleccione el punto en el que desea actuar y luego elija una acción:

* **Restaurar código y conversación**: revierte tanto el código como la conversación a ese punto
* **Restaurar conversación**: revierte a ese mensaje mientras mantiene el código actual
* **Restaurar código**: revierte los cambios de archivo mientras mantiene la conversación
* **Resumir desde aquí**: comprime la conversación desde este punto en adelante en un resumen, liberando espacio de context window
* **Resumir hasta aquí**: comprime la conversación antes de este punto en un resumen, manteniendo los mensajes posteriores intactos
* **Cancelar**: regresa a la lista de mensajes sin hacer cambios

Después de restaurar la conversación o elegir Resumir desde aquí, la solicitud original del mensaje seleccionado se restaura en el campo de entrada para que pueda reenviarlo o editarlo.

Al elegir Resumir hasta aquí, se queda al final de la conversación con la entrada vacía.

<h4 id="rewind-past-a-cleared-conversation">
  Revertir una conversación borrada
</h4>

Si ejecutó `/clear` anteriormente en el mismo proceso de Claude Code, el menú de rewind muestra una entrada adicional en la parte superior de la lista etiquetada como `/resume <session-id> (sesión anterior)`. Selecciónela para reanudar la conversación que estaba activa antes de que se ejecutara `/clear`. La entrada está disponible hasta que salga de Claude Code o reanude una sesión diferente, y requiere Claude Code v2.1.191 o posterior. En versiones anteriores, ejecute `/resume` y seleccione la sesión anterior de la lista.

<h4 id="restore-vs-summarize">
  Restaurar vs. resumir
</h4>

Las opciones de restauración revierten el estado: deshacen cambios de código, historial de conversación, o ambos. Las opciones de resumir comprimen parte de la conversación en un resumen generado por IA sin cambiar archivos en el disco:

* **Resumir desde aquí**: los mensajes anteriores al mensaje seleccionado permanecen intactos. El mensaje seleccionado y todo lo que viene después se reemplazan con un resumen. Utilice esto para descartar una discusión secundaria mientras mantiene el contexto inicial en detalle completo.
* **Resumir hasta aquí**: los mensajes anteriores al mensaje seleccionado se reemplazan con un resumen. El mensaje seleccionado y todo lo que viene después permanecen intactos, y usted permanece al final de la conversación. Utilice esto para comprimir la discusión de configuración inicial mientras mantiene el trabajo reciente en detalle completo.

En ambos casos, los mensajes originales se conservan en la transcripción de la sesión, por lo que Claude puede hacer referencia a los detalles si es necesario. Puede escribir instrucciones opcionales para guiar en qué se enfoca el resumen. Esto es similar a `/compact`, pero dirigido: en lugar de resumir toda la conversación, elige qué lado del mensaje seleccionado comprimir.

<Note>
  Resumir lo mantiene en la misma sesión y comprime el contexto. Si desea ramificarse e intentar un enfoque diferente mientras preserva la sesión original intacta, use [fork](/docs/es/sessions#branch-a-session) en su lugar (`claude --continue --fork-session`).
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

El checkpointing no rastrea archivos modificados por comandos bash. Por ejemplo, si Claude Code ejecuta:

```bash theme={null}
rm file.txt
mv old.txt new.txt
cp source.txt dest.txt
```

Estas modificaciones de archivo no se pueden deshacer a través de rewind. Solo se rastrean las ediciones de archivo directo realizadas a través de las herramientas de edición de archivos de Claude.

<h3 id="external-changes-not-tracked">
  Los cambios externos no se rastrean
</h3>

El checkpointing solo rastrea archivos que han sido editados dentro de la sesión actual. Los cambios manuales que realiza en archivos fuera de Claude Code y las ediciones de otras sesiones concurrentes normalmente no se capturan, a menos que modifiquen los mismos archivos que la sesión actual.

<h3 id="not-a-replacement-for-version-control">
  No es un reemplazo para el control de versiones
</h3>

Los checkpoints están diseñados para recuperación rápida a nivel de sesión. Para historial de versiones permanente y colaboración:

* Continúe usando control de versiones (por ejemplo, Git) para commits, ramas e historial a largo plazo
* Los checkpoints complementan pero no reemplazan el control de versiones adecuado
* Piense en los checkpoints como "deshacer local" y Git como "historial permanente"

<h2 id="see-also">
  Ver también
</h2>

* [Modo interactivo](/docs/es/interactive-mode) - Atajos de teclado y controles de sesión
* [Comandos](/docs/es/commands) - Acceso a checkpoints usando `/rewind`
* [Referencia de CLI](/docs/es/cli-reference) - Opciones de línea de comandos
