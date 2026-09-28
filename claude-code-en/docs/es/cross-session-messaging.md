> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Mensajería entre tus otras sesiones de Claude Code

> Permite que Claude liste y envíe mensajes a tus otras sesiones de Claude Code en esta máquina, y alcance tus sesiones en otras máquinas o en la web.

<Note>
  La mensajería entre sesiones requiere Claude Code v2.1.224 o posterior en macOS y Linux, incluido Linux dentro de WSL 2. En Windows nativo, requiere Claude Code v2.1.234 o posterior. Cuando una sesión cumple los requisitos, la mensajería está activada sin necesidad de habilitar nada. Consulta [Disponibilidad](#availability) para conocer los requisitos del proveedor y cómo confirmar que una sesión la tiene.
</Note>

La mensajería entre sesiones permite que Claude entregue un mensaje de una de tus sesiones de Claude Code a otra. Cuando un cambio en una sesión rompe lo que otra está construyendo, Claude puede advertir a esa sesión antes de que lo notes. Cuando una sesión resuelve una pregunta en la que otra está bloqueada, Claude puede enviar la respuesta entre sesiones.

Un mensaje es un fragmento de texto que un Claude escribe a otro, nunca el historial de conversación o archivos del remitente. Para mover una conversación completa o su contexto, [reanuda la sesión](/docs/es/sessions#resume-a-session) en su lugar.

Claude utiliza dos herramientas para esto: `ListAgents` para descubrir qué agentes puede alcanzar, y `SendMessage` para entregar un mensaje a uno de ellos por nombre. Con la misma herramienta `SendMessage`, Claude también puede enviar mensajes a [subagentes](/docs/es/sub-agents#resume-subagents) y compañeros de [equipo de agentes](/docs/es/agent-teams) dentro de una única sesión o equipo. Esta página cubre mensajes entre tus sesiones independientes.

<h2 id="when-to-use-cross-session-messaging">
  Cuándo usar mensajería entre sesiones
</h2>

Usa mensajería cuando una de tus sesiones tiene algo que otra sesión necesita a mitad de la tarea. Claude puede enviar un mensaje por su cuenta cuando ve la necesidad, por ejemplo después de hacer un cambio que afecta el trabajo que otra sesión está haciendo, o puedes pedirle que envíe uno. Los casos comunes son:

* **Entregar un hallazgo**: cuando una sesión descubre un cambio que rompe algo o toma una decisión, Claude lo resume para la sesión que trabaja en el área afectada, en lugar de que lo re-expliques allí.
* **Coordinar worktrees paralelos**: cuando las sesiones trabajan el mismo repositorio en [worktrees](/docs/es/worktrees) separados, Claude puede decirle a las otras sesiones qué se implementó.
* **Obtener estado del trabajo de larga duración**: haz que una migración o ejecución de prueba informe a la sesión que estás observando, o pregúntale desde allí. Si esa sesión está en esta máquina, Claude también puede [pedirle un aviso cuando la sesión se quede inactiva o salga](#get-a-notice-when-another-session-goes-idle).
* **Enviar mensajes entre máquinas**: alcanza una de tus sesiones en otra máquina o en la web.

Usa mensajería entre sesiones independientes que inicias y diriges tú mismo. Claude Code tiene una característica dedicada para cada una de las otras formas de ejecutar o alcanzar múltiples sesiones, así que usa la construida para lo que estás haciendo en su lugar:

* Para continuar una conversación en otra terminal, o compartir su contexto con una nueva sesión, [reanuda la sesión](/docs/es/sessions#resume-a-session)
* Para un equipo coordinado de sesiones que Claude genera y supervisa, usa [equipos de agentes](/docs/es/agent-teams)
* Para observar y dirigir muchas sesiones desde un lugar, usa [vista de agentes](/docs/es/agent-view)
* Para dirigir una sesión tú mismo desde tu teléfono u otro dispositivo, en lugar de que las sesiones se envíen mensajes entre sí, usa [Control Remoto](/docs/es/remote-control)
* Para insertar eventos externos, como resultados de CI o mensajes de chat, en una sesión, usa [canales](/docs/es/channels)

<h2 id="message-another-session">
  Enviar un mensaje a otra sesión
</h2>

Cuando una de tus sesiones aprende algo que otra sesión necesita, como un hallazgo, un estado o una decisión, Claude lo pasa en lugar de que copies y pegues entre terminales. Claude descubre el destino con `ListAgents` y envía con `SendMessage`, así que nunca llamas a ninguna herramienta tú mismo. Claude puede decidir enviar un mensaje sin ser preguntado, y también puedes solicitar uno.

Para solicitar uno tú mismo, dile a Claude qué quieres que la otra sesión sepa o haga. Este ejemplo es un mensaje que escribes, no un mensaje que Claude envía:

```text wrap theme={null}
Pregunta a la sesión que se ejecuta en mi otra terminal si la migración terminó
```

Claude escribe el mensaje real en sí, así que tu mensaje puede dejar el contenido a Claude. Este mensaje solicita un resumen sin dictar su redacción, y lo que Claude envía varía:

```text wrap theme={null}
Explica lo que acabamos de hacer a la sesión que trabaja en la API de pagos
```

Para nombrar el destino tú mismo, menciona la sesión en tu mensaje: escribe `@` seguido de las primeras letras del nombre de la sesión y elige la sesión del typeahead, de la misma manera que [@-mencionas un subagente](/docs/es/sub-agents#invoke-subagents-explicitly). Requiere Claude Code v2.1.232 o posterior. Claude Code inserta la mención, como `@api-worker`, y le dice a Claude qué sesión nombra, para que Claude pueda enviar un mensaje a esa sesión sin listar primero tus sesiones. Este mensaje nombra el destino con una mención:

```text wrap theme={null}
Hazle saber a @api-worker que la migración de esquema terminó
```

El typeahead lista tus otras sesiones activas en esta máquina. Dos casos necesitan más que las primeras letras de un nombre:

* **Una sesión más allá de esta máquina**: una sesión en la nube o Control Remoto aparece en el typeahead solo después de que Claude haya listado o enviado mensajes a tus sesiones más allá de esta máquina, así que pídele a Claude que las liste primero.
* **Un nombre con un espacio u otros caracteres fuera de letras, dígitos, guiones e guiones bajos**: escríbelo entre comillas dobles, como `@"release notes"`. Cuando eliges la sesión del typeahead, Claude Code inserta las comillas por ti.

También puedes escribir la mención sin el selector. Cuando más de una sesión activa responde al nombre mencionado, Claude te pregunta cuál quieres decir antes de enviar.

Para ver cómo se ve el mensaje que Claude escribe cuando llega, incluido un ejemplo de uno, consulta [cómo se ve un mensaje](#what-a-message-looks-like).

<h3 id="message-delivery">
  Entrega de mensajes
</h3>

El Claude receptor lee el mensaje entre llamadas de herramientas durante un turno activo, así que una herramienta en ejecución nunca se interrumpe. Cuando la sesión receptora está inactiva, Claude Code inicia un nuevo turno con el mensaje.

Un mensaje de otra sesión llega como texto plano. Si menciona un archivo o un [recurso MCP](/docs/es/mcp#use-mcp-resources) con `@`, Claude ve la mención tal como está escrita y Claude Code no adjunta nada, ya sea que el mensaje inicie un nuevo turno o llegue durante uno. Claude aún puede abrir una ruta mencionada en la máquina receptora con sus propias herramientas, sujeto a los permisos de esa sesión. Antes de v2.1.251, una mención `@` en un mensaje que inició un nuevo turno adjuntaba el archivo o recurso MCP en el lado receptor.

Claude Code rechaza un mensaje en los siguientes casos:

* El mensaje está [por encima del límite de tamaño](#limitations). Claude Code lo rechaza en la sesión de envío, antes de que se vaya.
* Una ráfaga rápida a una sesión en esta máquina ha alcanzado [lo que esa bandeja de entrada de sesión acepta](#limitations). Claude Code rechaza más mensajes a esa sesión.
* El destino de respuesta en esta máquina falla una verificación de seguridad, como un destino con enlace simbólico o un punto final que no es el proceso esperado. [Rechazar enviar un mensaje entre sesiones](/docs/es/errors#refusing-to-send-a-cross-session-message) lista estas verificaciones.
* Claude dirige el mensaje al nombre de esta propia sesión, como se describe en [Ver qué sesiones Claude puede alcanzar](#see-which-sessions-claude-can-reach).

La sesión receptora verifica cada mensaje que llega contra sus propios [controles de entrada](#control-inbound-messages), y la verificación termina en uno de tres resultados:

* **Entregado**: Claude Code pasa el mensaje al Claude receptor.
* **Retenido**: Claude Code deja el mensaje de lado sin entregar. Un mensaje retenido llega a Claude solo cuando lo apruebas o un modo o cambio de configuración posterior lo permite.
* **Rechazado**: Claude Code descarta el mensaje sin entregarlo.

Una vez entregado, el mensaje cuenta hacia [uso](/docs/es/costs) como un mensaje que escribes, y el Claude receptor puede responder al remitente de la misma manera, excepto en el [caso de una sola dirección entre máquinas](#message-sessions-on-other-machines).

Los límites de permisos permanecen por sesión. Claude recibe instrucciones de nunca pedir a otra sesión una acción que fue denegada o bloqueada en su propia sesión, o que sus propias configuraciones de permisos bloquearían, y de enrutar ese trabajo de vuelta a ti. En el lado receptor, los [propios mensajes de permiso y reglas de la sesión receptora aún se aplican](#how-a-session-treats-an-incoming-message) a cualquier cosa que el mensaje solicite.

<h3 id="get-a-notice-when-another-session-goes-idle">
  Obtener un aviso cuando otra sesión se queda inactiva
</h3>

Claude puede pedirle a una de tus sesiones en esta máquina que envíe un aviso cuando esa sesión se quede inactiva o salga. Inactivo aquí significa que la sesión terminó un turno sin nada en cola. Úsalo cuando estés esperando una tarea larga en otra sesión y quieras saber cuándo termina en lugar de verificar. Requiere Claude Code v2.1.236 o posterior en ambas sesiones.

<h4 id="ask-for-a-notice">
  Solicitar un aviso
</h4>

Dile a Claude qué estás esperando. Este mensaje solicita un aviso de la sesión de migración:

```text wrap theme={null}
Dime cuándo la sesión de migración termina lo que está haciendo
```

Claude se suscribe con la entrada `notify_when_idle` de la herramienta `SendMessage`, ya sea adjunta a un mensaje que está enviando de todas formas o por su cuenta. Por su cuenta, Claude Code se suscribe sin iniciar un turno o gastar tokens en la sesión observada, y envía el aviso de inmediato si esa sesión ya está inactiva. Adjunto a un mensaje, Claude Code entrega el mensaje primero y envía el aviso después.

<h4 id="what-each-session-shows">
  Lo que cada sesión muestra
</h4>

La sesión observada muestra una línea diciendo que otro proceso pidió ser notificado cuando la sesión esté inactiva. La sesión que pregunta muestra el aviso como una línea que nombra la sesión observada. La línea puede incluir la hora en que terminó el turno de esa sesión y un estado de una línea de ese turno. Si la sesión que pregunta está inactiva, Claude Code inicia un nuevo turno con el aviso.

<h4 id="limits">
  Límites
</h4>

El aviso es de una sola vez: Claude Code lo envía una vez desde la sesión observada, y ninguna sesión sondea la otra. Si no llega ningún aviso dentro de 12 horas, Claude Code descarta la suscripción y le dice a Claude, para que no siga esperando.

Los [controles de entrada](#control-inbound-messages) de cada lado se aplican a un aviso como un mensaje:

* **`refuse` en cualquier lado**: nada llega. La sesión observada descarta la solicitud sin registrar o responder, así que la suscripción expira sin respuesta después de 12 horas, y una sesión que pregunta con `refuse` nunca se suscribe.
* **`hold` en cualquier lado**: el aviso llega con menos. La sesión observada deja fuera el estado de una línea, y la sesión que pregunta muestra el aviso en tu transcripción sin entregarlo a Claude.

Solo el Claude en tu conversación principal puede suscribirse, y solo a tus sesiones en esta máquina. Cuando un subagente o un compañero de equipo de agentes establece `notify_when_idle`, Claude Code no hace suscripción y le dice así. Cuando Claude pide un aviso de cualquier otro agente, como un compañero, un subagente o una sesión más allá de esta máquina, Claude Code rechaza toda la llamada, incluido cualquier mensaje adjunto, e informa el rechazo a Claude para que pueda reenviar el mensaje sin la solicitud.

<h3 id="see-which-sessions-claude-can-reach">
  Ver qué sesiones Claude puede alcanzar
</h3>

Claude encuentra el destino de un mensaje por su cuenta, así que no necesitas ejecutar nada antes de pedirle que envíe. Para ver por ti mismo qué sesiones Claude puede alcanzar, ejecuta el comando `/list-agents`. La primera línea, cuando está presente, es el nombre de esta propia sesión, el que tus otras sesiones usan para enviarle mensajes. Las filas debajo son las sesiones que Claude puede alcanzar:

* **Subagentes**: agentes que se ejecutan dentro de la sesión actual.
* **Compañeros**: los propios compañeros de [equipo de agentes](/docs/es/agent-teams) de esta sesión. Antes de v2.1.239, los compañeros no aparecían en el listado, aunque Claude ya podía enviarles mensajes por nombre.
* **Tus otras sesiones locales**: sesiones de Claude Code que se ejecutan en la misma máquina, incluidas [sesiones en segundo plano](/docs/es/agent-view). Una sesión aparece solo cuando vincula un [socket de bandeja de entrada](#the-sessions-inbox-socket).
* **Tus sesiones en la nube**: tus sesiones de [Claude Code en la web](/docs/es/claude-code-on-the-web), mostradas mientras esta sesión está conectada a [Control Remoto](/docs/es/remote-control). Claude Code las etiqueta como `cloud` en el listado.
* **Tus sesiones de Control Remoto en otras máquinas**: mostradas mientras esta sesión está conectada a [Control Remoto](/docs/es/remote-control), y etiquetadas como `Remote Control`. Claude Code muestra `offline` como el estado de una sesión cuya conexión de Control Remoto se ha caído.

Esta sesión no es una de las filas. Si Claude dirige un mensaje al nombre de esta propia sesión, Claude Code lo rechaza y le dice a Claude que el destino es la sesión actual. Antes de v2.1.239, el listado no mostraba el nombre de esta sesión, y Claude Code reportaba un mensaje enviado a él como un agente que no podía encontrar.

Mientras esta sesión está conectada a [Control Remoto](/docs/es/remote-control), Claude Code retiene algunos detalles de tus sesiones locales de la salida `/list-agents`, sin cambiar lo que Claude mismo ve cuando busca una sesión para enviar un mensaje:

* **Directorios de trabajo**: deja fuera el directorio de trabajo de cada sesión local.
* **Nombres de sesión**: deja fuera cualquier nombre de sesión que no pueda atribuir a una persona, así que una fila dejada sin nombre lee `(unnamed session)`.
* **La primera línea**: deja fuera la línea con el nombre de esta propia sesión a menos que escribas ese nombre en esta terminal, con `--name` o con `/rename` y el nombre, desde que lanzaste o reanudaste la sesión por última vez.

Cuando la salida lista algo, termina con una nota diciendo que los detalles fueron retenidos. Ejecutar `/rename` seguido de un nombre no utilizado en el teclado de una sesión propia le da a esa sesión un nombre que aparece en la salida.

Claude Code lee tus listas de sesiones en la nube y Control Remoto más nuevas primero y se detiene después de un número acotado de páginas para cada una. Si tu cuenta tiene más de esas sesiones de las que caben, Claude Code no lista las más antiguas, y Claude no puede enviarles mensajes por nombre. Cuando esto sucede, Claude Code lo dice en el listado, y Claude ve la misma nota cuando envía un mensaje.

Claude dirige una sesión más allá de esta máquina por nombre, igual que una sesión local. Consulta [Enviar mensajes a sesiones en otras máquinas](#message-sessions-on-other-machines) para ver cómo viajan esos mensajes.

Una sesión responde al nombre que estableces con el comando [`/rename`](/docs/es/commands) o la bandera [`--name`](/docs/es/cli-reference#cli-flags). Cuando no estableces uno, Claude Code nombra la sesión en sí. Para una sesión interactiva, ese es el nombre mostrado en [listados de sesiones en ejecución](/docs/es/sessions#name-your-sessions).

Cuando renombras una sesión, Claude Code también actualiza el registro compartido que tus otras sesiones usan para buscar el nombre de la sesión. Si no puede actualizar ese registro, te advierte en la salida `/rename` que otras sesiones aún pueden mostrar el nombre antiguo. Ejecuta la sesión con [`--debug`](/docs/es/cli-reference#cli-flags), y Claude Code registra la causa de la actualización fallida.

Cuando renombras una sesión, o inicias o reanudas una interactiva, con un nombre que otra sesión activa en esta máquina ya usa, Claude Code deja el nombre con la sesión que ya lo tiene y [renombra el tuyo a una variante](/docs/es/sessions#name-your-sessions). Las sesiones aún pueden compartir un nombre, por ejemplo cuando una de ellas ejecuta una versión anterior de Claude Code o el nombre compartido es uno que Claude Code generó. A menos que esta sesión esté conectada a Control Remoto, Claude Code muestra el directorio de trabajo de cada sesión local en la salida `/list-agents`, así que puedes distinguir sesiones con el mismo nombre cuando se ejecutan en directorios diferentes. Claude dirige el mensaje de una de dos maneras, dependiendo de cuántas sesiones activas respondan al nombre:

* **Una sesión responde al nombre**: Claude Code entrega el mensaje solo en el nombre.
* **Varias sesiones comparten el nombre, o Claude Code no pudo verificar en todas partes donde se ejecutan tus sesiones**: Claude agrega un identificador corto a cada fila de su listado y usa el identificador en la dirección.

<h3 id="message-sessions-on-other-machines">
  Enviar mensajes a sesiones en otras máquinas
</h3>

Cómo viaja un mensaje, y si pasa por servidores de Anthropic, depende de dónde se ejecuta la sesión de destino:

| Dónde se ejecuta la otra sesión                        | Cómo viaja el mensaje                                                                                                                         |
| :----------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------- |
| En esta máquina                                        | Sobre un socket por sesión en macOS y Linux, o una tubería con nombre por sesión en Windows nativo, nunca a través de servidores de Anthropic |
| En otra de tus máquinas                                | A través de servidores de Anthropic, llegando sobre la conexión [Control Remoto](/docs/es/remote-control) de esa máquina                           |
| En [Claude Code en la web](/docs/es/claude-code-on-the-web) | A través de servidores de Anthropic, directamente a la sesión en la nube                                                                      |

Iniciar una conversación con una sesión en otra de tus máquinas requiere Claude Code v2.1.225 o posterior y un destino que [aparezca en el listado](#see-which-sessions-claude-can-reach). Antes de v2.1.225, Claude solo podía responder a un mensaje que llegaba de uno.

Puedes enviar un mensaje a una sesión mostrada como `offline` en [el listado](#see-which-sessions-claude-can-reach), una cuya conexión de Control Remoto se ha caído. El envío se realiza, pero el mensaje llega solo después de que la máquina de esa sesión se reconecte. Claude es informado de esto cuando envía.

La entrega en la misma máquina funciona dondequiera que la característica esté habilitada. Cada sesión se registra en archivos en el disco. Cuando Claude lista o envía mensajes a tus sesiones locales, Claude Code lee esos archivos para encontrar las sesiones, así que dos sesiones pueden alcanzarse mutuamente solo cuando pueden ver los mismos archivos.

Un contenedor tiene su propio sistema de archivos, así que una sesión dentro de él y una sesión en el host no pueden alcanzarse mutuamente. Dos sesiones dentro del mismo contenedor aún pueden enviar mensajes entre sí, incluido en un [ejecutor auto-hospedado](/docs/es/self-hosted-environments). Una sesión dentro de WSL 2 y una sesión nativa de Windows en la misma computadora tampoco pueden alcanzarse mutuamente, porque se registran bajo directorios de inicio diferentes y escuchan en tipos de socket diferentes.

Mientras esta sesión está conectada a Control Remoto, cuando envías un mensaje a una sesión en otra de tus máquinas, Claude Code muestra el mensaje en la conversación de esa sesión bajo el nombre de Control Remoto de esta sesión. El Claude en esa máquina puede responder a ese nombre. Por ejemplo, cuando esta sesión está conectada a Control Remoto como `laptop-graceful-unicorn` y envías un mensaje a tu escritorio, ves el mensaje en la sesión de escritorio bajo `laptop-graceful-unicorn`.

Si esta sesión no está conectada a Control Remoto cuando Claude envía a una sesión más allá de esta máquina, el mensaje aún se va, pero sin una [dirección de respuesta](#what-a-message-looks-like), así que el Claude receptor no puede responder. Claude es informado de esto cuando envía.

Para requerir tu aprobación antes de que cualquier mensaje vaya más allá de esta máquina, establece [`isolatePeerMachines`](#require-approval-for-cross-machine-messages).

<h2 id="how-a-session-treats-an-incoming-message">
  Cómo una sesión trata un mensaje entrante
</h2>

Cuando la sesión A envía un mensaje a la sesión B, Claude Code le dice al Claude de B que el mensaje vino de otra sesión, no de ti, y limita lo que el mensaje puede hacer:

* **No puede aprobar nada**: un mensaje de otra sesión nunca cuenta como tu consentimiento, así que no puede responder a un mensaje de permiso pendiente en tu nombre.
* **No puede cambiar configuración**: Claude Code instruye al Claude receptor a nunca cambiar configuraciones de permisos, `CLAUDE.md` u otra configuración porque otra sesión lo pidió.
* **Los comandos no se ejecutan**: un comando en el texto del mensaje, como `/compact`, llega como texto plano. Claude Code nunca lo ejecuta.
* **Los mensajes de permiso aún se disparan**: si actuar sobre el mensaje requiere un permiso que la sesión receptora no tiene, ves el mismo mensaje que verías para cualquier otro trabajo.

<h3 id="what-a-message-looks-like">
  Cómo se ve un mensaje
</h3>

Cuando llega un mensaje, Claude Code lo muestra en la conversación como una vista previa de una línea atenuada, y la línea de vista previa permanece en la conversación después. La vista previa lleva el nombre del remitente y la primera línea del mensaje, cortada con `…` cuando es larga, como `› Message from @api-worker: Schema migration finished (ctrl+o to expand)`. Antes de v2.1.247, Claude Code mostraba el mensaje que llegaba en su totalidad en lugar de una vista previa.

Cualquiera de estos muestra el texto completo:

* Presiona `Ctrl+O` para abrir el [visor de transcripción](/docs/es/interactive-mode#transcript-viewer) y lee el texto completo bajo el nombre de sesión del remitente.
* En una sesión iniciada con [`--verbose`](/docs/es/cli-reference#cli-flags), Claude Code muestra el texto completo en lugar de la vista previa.

La vista previa acorta solo lo que ves. Ya sea que la expandas o no, Claude lee el mensaje completo.

Claude recibe el mensaje con el nombre del remitente y una dirección de respuesta, excepto para un [mensaje de una sola dirección entre máquinas](#message-sessions-on-other-machines), que no lleva dirección de respuesta. Más allá del nombre y la dirección de respuesta, el Claude receptor obtiene el texto del mensaje, nunca el historial de conversación o archivos del remitente. [Entrega de mensajes](#message-delivery) cubre menciones `@` en el texto.

Un mensaje que un [subagente](/docs/es/sub-agents) escribió llega bajo el nombre de la sesión de envío, con el subagente identificado en el texto del mensaje. Una respuesta a él llega a la conversación principal de esa sesión, no al subagente.

Este ejemplo es un mensaje que un Claude escribió a otro, tal como su texto completo se lee cuando lo expandes:

```text wrap theme={null}
Schema migration finished
The new column is tenant_id, and rebasing on main is safe now.
```

<h3 id="control-inbound-messages">
  Controlar mensajes entrantes
</h3>

Establece [`crossSessionInbound`](/docs/es/settings-reference#crosssessioninbound) para elegir qué hace una sesión con los mensajes que llegan de tus otras sesiones:

| Valor    | Comportamiento                                                                                                                                                                                                             |
| :------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `accept` | Claude Code entrega cada mensaje a Claude                                                                                                                                                                                  |
| `hold`   | Claude Code muestra un aviso para cada mensaje y no lo entrega. Si un `accept` se aplica después, según las [reglas de precedencia](/docs/es/settings-reference#crosssessioninbound), Claude Code libera los mensajes retenidos |
| `refuse` | Claude Code descarta cada mensaje sin entregarlo                                                                                                                                                                           |

Más allá de editar un archivo de configuración, puedes seleccionar el valor en la fila `/config` **Mensajes de tus otras sesiones**. Claude Code escribe el valor que seleccionas en tu configuración de usuario. La fila requiere Claude Code v2.1.232 o posterior y no aparece mientras la configuración administrada o la bandera `--settings` establece la clave, ya que un valor de configuración de usuario no se aplicaría entonces. Claude Code rechaza el atajo `/config crossSessionInbound=value` para esta clave.

Para ver qué valor se aplica, sigue las reglas de precedencia `crossSessionInbound` en la [referencia de configuración](/docs/es/settings-reference#crosssessioninbound).

Cuando no se aplica ningún valor, Claude Code decide por mensaje de las dos clases de modo de permiso de las sesiones. Agrupa sesiones que [omiten mensajes de permiso](/docs/es/permission-modes#skip-all-checks-with-bypasspermissions-mode) en una clase, y todas las otras sesiones en la otra. El modo Plan cuenta como omisión en sesiones con permisos de omisión disponibles, y [auto](/docs/es/permission-modes#eliminate-prompts-with-auto-mode), `acceptEdits` y `dontAsk` cuentan como mensajes:

* **La sesión receptora solicita permisos**: Claude Code entrega cada mensaje. Retiene uno para tu aprobación solo cuando la sesión de envío se identifica como omitiendo mensajes de permiso.
* **La sesión receptora omite mensajes de permiso**: Claude Code retiene cada mensaje para tu aprobación. Entrega uno solo cuando la sesión de envío se identifica como también omitiendo.

Cuando el valor predeterminado retiene un mensaje, Claude Code abre un diálogo de aprobación en la sesión receptora. El diálogo muestra el remitente y una vista previa:

* **Aprobar** entrega ese mensaje a Claude.
* **Denegar**, o descartar el diálogo, lo descarta.
* Cuando el diálogo permanece sin respuesta pasado el plazo [`dialogExpiry`](/docs/es/settings-reference#dialogexpiry), Claude Code lo cierra y descarta el mensaje. El plazo predeterminado es cinco minutos.
* Mientras ninguna terminal está adjunta a una [sesión en segundo plano](/docs/es/agent-view), Claude Code deja el diálogo abierto pasado el plazo. Después de que adjuntes, si el diálogo permanece sin respuesta durante un período de plazo completo, Claude Code lo cierra y descarta el mensaje.
* Si la clase de modo de permiso de esta sesión cambia mientras los mensajes están retenidos, Claude Code re-aplica las reglas de entrada, entrega los mensajes que ahora acepta, y muestra un aviso.
* Si un cambio de configuración hace que `refuse` se aplique mientras los mensajes están retenidos, Claude Code descarta cada mensaje retenido e informa un rechazo a cada remitente que puede alcanzar.

Cuando el remitente es una sesión en la misma máquina, Claude Code envía un aviso de vuelta a ella cuando el receptor retiene el mensaje, y un seguimiento cuando el receptor después lo entrega, lo deniega, o expira. El aviso llega al Claude de envío, así que sabe que no debe seguir esperando un mensaje que la otra sesión no ha leído.

En una sesión de envío interactiva, el aviso aparece en la transcripción. Un remitente [`claude -p`](/docs/es/headless) lo recibe en [salida transmitida](/docs/es/headless#stream-responses) como un [mensaje `system` informativo](/docs/es/agent-sdk/typescript#sdkinformationalmessage). Los avisos a remitentes `claude -p` requieren Claude Code v2.1.271 o posterior.

Si el receptor rechaza el mensaje, el aviso del remitente dice que el receptor no está aceptando mensajes entre sesiones y le dice al Claude del remitente que no espere o reenvíe.

Claude Code retiene como máximo 100 mensajes, separados de la cola de entrega, y pasado eso descarta los más antiguos.

<h3 id="non-interactive-sessions">
  Sesiones no interactivas
</h3>

Claude Code vincula un socket de bandeja de entrada para una sesión [`claude -p`](/docs/es/headless) como una interactiva, así que un trabajador `-p` de larga duración puede recibir mensajes y aparece en el listado. Cuando inicias una sesión en [modo desnudo](/docs/es/headless#start-faster-with-bare-mode), Claude Code no vincula el socket, así que esa sesión no puede recibir mensajes y no aparece en la lista de agentes.

Una sesión `-p` no puede mostrar el diálogo de aprobación. Cuando el [valor predeterminado de entrada](#control-inbound-messages) retiene un mensaje allí, Claude Code lo mantiene para el mismo plazo [`dialogExpiry`](/docs/es/settings-reference#dialogexpiry) que el diálogo usa, cinco minutos por defecto:

* **Antes del plazo**: si un cambio de modo o configuración permite el mensaje, Claude Code lo entrega.
* **Pasado el plazo**: Claude Code descarta el mensaje e informa que expiró a un remitente que puede alcanzar.

Establece `dialogExpiry` a `"never"` para mantener los mensajes retenidos por defecto hasta que la sesión termine. Un mensaje retenido por una configuración explícita `hold` no expira; Claude Code lo entrega solo cuando un `accept` se aplica después.

Cuando la sesión termina con mensajes aún retenidos, Claude Code los informa como expirados a cada remitente que puede alcanzar. Antes de v2.1.225, no se aplicaba plazo en una sesión `-p`: un mensaje retenido permanecía retenido a menos que un cambio de modo de permiso durante la ejecución lo entregara, y una sesión que terminaba con mensajes retenidos no reportaba nada a sus remitentes.

Para permitir que un trabajador `-p` tome mensajes desatendido, inicia con `crossSessionInbound` establecido a `accept` en su valor `--settings`. Un `accept` en tu configuración de usuario también funciona pero se aplica a cada sesión que ejecutas.

<h3 id="the-sessions-inbox-socket">
  El socket de bandeja de entrada de la sesión
</h3>

Lee esta sección cuando una sesión que esperas no está en la lista de agentes, cuando quieres que un script o hook publique en una sesión, o cuando un comando sandboxed no puede alcanzar el socket.

Claude Code vincula un socket de bandeja de entrada para cada sesión con mensajería entre sesiones habilitada, donde otras sesiones en la máquina entregan mensajes. El socket es un socket de dominio Unix en macOS y Linux, incluido Linux dentro de WSL 2, y una tubería con nombre en Windows nativo. Para qué tipos de sesión vinculan uno, consulta [Sesiones no interactivas](#non-interactive-sessions).

Puedes encontrar la ruta del socket en dos lugares:

* `/status` la muestra en la fila `Peer address`. La ruta tiene el prefijo `uds:`.
* Claude Code la exporta a [hooks](/docs/es/hooks) y comandos Bash como la variable de entorno [`CLAUDE_CODE_MESSAGING_SOCKET`](/docs/es/env-vars#variables):
  * En una sesión que inicia con mensajería activada, Claude Code exporta la variable antes de que cualquier hook se ejecute, incluido `SessionStart`.
  * Cada sesión exporta su propio socket, nunca uno heredado de una sesión padre.

En macOS y Linux, Claude Code restringe el socket a tu usuario del sistema operativo. En Windows nativo, en su lugar requiere que cada conexión se autentique primero con una clave que solo tu usuario del sistema operativo puede leer. De cualquier manera, en una máquina compartida las sesiones de otro usuario no pueden entregar a él.

En macOS y Linux, Claude Code también rechaza crear el socket en un directorio que no puede aceptar, por ejemplo uno que otro usuario posee, y usa un directorio privado por usuario, `/tmp/cc-socks-<uid>`, en su lugar. Cuando no puede aceptar ningún directorio, la sesión se ejecuta sin una bandeja de entrada: Claude Code muestra un aviso, `/status` muestra `unavailable` y la razón en su fila `Peer address`, y el registro [`--debug`](/docs/es/cli-reference#cli-flags) registra el rechazo completo.

Junto a la ruta del socket, Claude Code exporta un token por sesión como [`CLAUDE_CODE_MESSAGING_TOKEN`](/docs/es/env-vars#variables). Un script que publica en el socket de su propia sesión puede enviar `{"type":"auth","token":"<token>"}` como la primera línea de su conexión, donde `<token>` es el valor de `CLAUDE_CODE_MESSAGING_TOKEN`. Si Claude Code requiere la línea depende de la plataforma:

* **macOS y Linux, incluido WSL 2**: la línea es opcional. Claude Code acepta una conexión con o sin ella.
* **Windows nativo**: la línea es requerida. Claude Code cierra cualquier conexión cuya primera línea no sea una línea de autenticación válida y no entrega nada de esa conexión.

Abre la conexión solo cuando el mensaje que estás publicando esté listo. Claude Code cierra una conexión que no ha enviado una línea completa dentro de 30 segundos, así que captura la salida de un comando lento primero y luego abre la conexión para enviarlo.

Las [reglas de propio-hijo](#own-child-messages) abajo dicen cuándo Claude Code consulta el token y cómo trata un mensaje que no puede verificar.

<span id="own-child-messages" />Claude Code ejecuta mensajes que llegan en el socket a través de los mismos [controles de entrada](#control-inbound-messages) que cualquier otro mensaje de par, con una excepción y un requisito previo:

* **Mensajes de propio-hijo**: cuando no se aplica ningún valor `crossSessionInbound`, Claude Code entrega un mensaje que verifica vino de los procesos hijo de la sesión, como un hook o comando Bash que publica de vuelta al socket de su propia sesión.
  * En Linux, incluido dentro de WSL 2, Claude Code puede verificar por evidencia de proceso incluso para un hijo que ya ha salido. En macOS puede verificar de esa manera solo mientras el proceso de publicación aún se está ejecutando, y en un contenedor donde Claude Code se ejecuta como ID de proceso 1 no tiene evidencia de proceso en absoluto. En Windows nativo tampoco la tiene.
  * En macOS después de que el proceso de publicación ha salido y en contenedores donde Claude Code se ejecuta como ID de proceso 1, esa evidencia de proceso falta, y Claude Code en su lugar verifica un hijo que envió el [`CLAUDE_CODE_MESSAGING_TOKEN`](/docs/es/env-vars#variables) exportado de la sesión en la línea de autenticación que abrió su conexión. En Windows nativo, ese token es la única forma en que Claude Code verifica un mensaje de propio-hijo.
  * Cuando Claude Code no puede verificar de ninguna manera, trata el mensaje como cualquier otro que no afirma ninguna clase de permiso, así que una sesión que omite mensajes de permiso lo retiene para tu aprobación.
* **Sesiones sandboxed**: controla si un comando Bash puede alcanzar el socket desde dentro del [sandbox](/docs/es/sandboxing) con la configuración de socket Unix del sandbox, [`sandbox.network.allowAllUnixSockets` y `sandbox.network.allowUnixSockets`](/docs/es/settings-reference#sandbox-settings).

<h2 id="restrict-cross-session-messaging">
  Restringir mensajería entre sesiones
</h2>

Más allá de los valores predeterminados por mensaje, puedes estrechar la mensajería de dos maneras. Requiere tu aprobación antes de que cualquier mensaje salga de la máquina, o desactiva la mensajería para una sesión u organización.

<h3 id="require-approval-for-cross-machine-messages">
  Requerir aprobación para mensajes entre máquinas
</h3>

Establece [`isolatePeerMachines`](/docs/es/settings-reference#isolatepeermachines) a `true` para requerir tu aprobación explícita antes de que cualquier `SendMessage` alcance una sesión más allá de esta máquina:

```json theme={null}
{
  "isolatePeerMachines": true
}
```

Con esto establecido, Claude Code pide tu aprobación antes de que el mensaje de Claude a una sesión más allá de esta máquina se vaya, incluso en modo `bypassPermissions`, que omite mensajes de permiso ordinarios. Un `true` de cualquier ámbito de configuración se aplica, así que un archivo de proyecto registrado puede activar el requisito pero no desactivarlo. Claude Code no solicita mensajes entre sesiones en la misma máquina.

<h3 id="turn-off-cross-session-messaging">
  Desactivar mensajería entre sesiones
</h3>

Recibir y enviar son controles separados, así que desactiva la dirección que necesites, o ambas. Usa `crossSessionInbound` para mensajes que llegan, y reglas de permisos para lo que Claude aquí puede enviar o listar:

* **Dejar de recibir**: establece `crossSessionInbound` a `refuse`, y Claude Code descarta mensajes de par entrantes sin entregarlos. Desde configuración de proyecto o local, `refuse` se aplica sobre todas las otras fuentes, y desde tu configuración de usuario se aplica a menos que la configuración administrada o la bandera `--settings` establezcan un valor.
* **Dejar de enviar y listar**: agrega [reglas de permiso de denegación específicas de herramienta](/docs/es/permissions#tool-specific-permission-rules) que nombren `SendMessage` y `ListAgents`. Ambas toman el nombre de herramienta desnudo sin especificador.

Los administradores pueden desactivar ambos lados para una organización en [configuración administrada](/docs/es/managed-settings), combinando las reglas de denegación con el `refuse`:

```json theme={null}
{
  "permissions": {
    "deny": ["SendMessage", "ListAgents"]
  },
  "crossSessionInbound": "refuse"
}
```

Con esto en su lugar, Claude Code aún vincula el socket de bandeja de entrada de cada sesión, pero descarta cada mensaje que llega en él sin entregar nada a Claude. Denegar `SendMessage` también elimina la mensajería a subagentes y compañeros de equipo de agentes, ya que la misma herramienta sirve a ambos. Una sesión que rechaza no muestra cambio visible, en su propio `/status` o en los listados de otras sesiones en la misma máquina, así que para confirmarlo, verifica los archivos de configuración que se aplican a esa sesión en lugar de su estado.

<h2 id="availability">
  Disponibilidad
</h2>

La mensajería entre sesiones requiere Claude Code v2.1.224 o posterior en macOS, Linux y WSL 2, y v2.1.234 o posterior en Windows nativo. La disponibilidad, y qué sesiones Claude puede enviar mensajes, también dependen de su sistema operativo, proveedor y configuración:

* **Sistema operativo**: disponible en macOS, Windows y Linux, incluido Linux dentro de WSL 2.

* **Sesiones en esta máquina**: disponible en cada proveedor, incluido Amazon Bedrock, Claude Platform en AWS, Google Cloud's Agent Platform y Microsoft Foundry, y en sesiones que se ejecutan con [obtención de bandera de característica](/docs/es/env-vars#features-that-need-feature-flag-fetching) desactivada. En esos proveedores, y con obtención de bandera desactivada, la mensajería en la misma máquina requiere Claude Code v2.1.248 o posterior. Claude Code entrega estos mensajes sobre un [socket por sesión en su máquina](#the-sessions-inbox-socket), nunca a través de servidores de Anthropic.

  Para evitar que una sesión los reciba, establezca [`crossSessionInbound`](#turn-off-cross-session-messaging) a `refuse`.

* **Sesiones más allá de esta máquina**: Claude encuentra sus [sesiones en la nube](/docs/es/claude-code-on-the-web) y sus sesiones en otras máquinas desde una sesión que está conectada a Control Remoto, que necesita un inicio de sesión en claude.ai como autenticación activa de esta sesión y los otros [requisitos de Control Remoto](/docs/es/remote-control#requirements). Claude no puede encontrar esas sesiones con una clave API o en Amazon Bedrock, Claude Platform en AWS, Google Cloud's Agent Platform y Microsoft Foundry.

Para verificar una sesión, escriba `/list-agents`, también disponible como `/peers`. El resultado separa una sesión que no tiene la característica de una sesión donde algo más estrecho bloqueó un mensaje, como una herramienta `SendMessage` faltante o un envío rechazado:

* **`/list-agents` no se reconoce**: la sesión no tiene mensajería entre sesiones. Trabaje a través de los requisitos anteriores, comenzando con `claude --version` para el requisito de versión.
* **`/list-agents` funciona pero un envío no llegó**: la mensajería está activada, y algo más estrecho se aplica:
  * **Reglas de denegación**: una [regla de permiso de denegación](#turn-off-cross-session-messaging) elimina las herramientas `SendMessage` y `ListAgents`.
  * **Controles de entrada**: los [controles de entrada de la sesión receptora](#control-inbound-messages) pueden retener o descartar lo que envía.
  * **Sesión en la nube faltante**: una sesión en la nube aparece solo mientras esta sesión está conectada a [Control Remoto](/docs/es/remote-control).
  * **Sesión en otra máquina faltante**: una sesión en otra de sus máquinas aparece solo cuando se ejecuta con [Control Remoto](/docs/es/remote-control) y esta sesión también está conectada.
  * **Sesión en otra máquina `offline`**: un mensaje a una sesión listada como `offline` se envía, pero [llega solo después de que la máquina de esa sesión se reconecte](#message-sessions-on-other-machines).
  * **Sesión en la nube o en otra máquina más antigua faltante**: Claude Code [lee esas listas de sesiones más nuevas primero y se detiene después de un número acotado de páginas](#see-which-sessions-claude-can-reach), así que Claude no puede enviar un mensaje a una sesión que cayó pasado ellas por nombre.
  * **Iniciando una conversación**: [Enviar mensajes a sesiones en otras máquinas](#message-sessions-on-other-machines) cubre iniciar una conversación con una sesión más allá de esta máquina.

En una sesión con mensajería, `/status` también muestra una fila `Peer address` con la dirección de bandeja de entrada propia de la sesión, o `unavailable` y la razón cuando Claude Code [no pudo configurar una bandeja de entrada](#the-sessions-inbox-socket).

<h2 id="limitations">
  Limitaciones
</h2>

Los límites aquí son propiedades del canal de mensajería en sí y se aplican dondequiera que la característica se ejecute. Para brechas de plataforma y proveedor, consulta [Disponibilidad](#availability) en su lugar.

* **Solo texto plano**: Claude envía solo texto plano entre sesiones. Los mensajes de protocolo de [equipo de agentes](/docs/es/agent-teams) estructurados permanecen dentro de un equipo.
* **El tamaño del mensaje en la misma máquina está limitado**: Claude Code rechaza un mensaje a una sesión en esta máquina una vez que su forma serializada pasa aproximadamente un millón de caracteres. El rechazo [nombra los tamaños exactos](/docs/es/errors#message-too-large-for-cross-session-delivery). Nada llega a la sesión receptora.
* **Las ráfagas rápidas a una sesión se rechazan en el remitente**: una vez que una ráfaga rápida de mensajes a una sesión en esta máquina alcanza lo que esa bandeja de entrada acepta, Claude Code rechaza más envíos en la sesión de envío. El [rechazo nombra la ráfaga](/docs/es/errors#too-many-messages-to-this-session-just-now) y le dice a Claude que agrupe el resto en un mensaje o espere. Antes de v2.1.236, Claude Code reportaba esos envíos como enviados mientras la sesión receptora los descartaba.
* **Los bucles de mensajes se limitan**: en la sesión receptora, Claude Code limita la velocidad de mensajes repetidos por remitente, descarta repeticiones idénticas que llegan dentro de una ventana corta, y pone en cola como máximo 50 mensajes aceptados para que Claude lea. Un bucle de mensajes entre dos sesiones por lo tanto se detiene por sí solo. Cuando el límite de velocidad, verificación de repetición o límite de cola descarta un mensaje de una sesión interactiva en esta máquina, Claude Code le dice a esa sesión cuál lo descartó y le dice a su Claude que no reenvíe de inmediato.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Subagentes](/docs/es/sub-agents#resume-subagents) y [equipos de agentes](/docs/es/agent-teams#messages-between-agents): mensajería dentro de una única sesión o equipo
* [Agentes en segundo plano](/docs/es/agent-view): despacha y monitorea las sesiones paralelas que podrías enviar mensajes
* [Control Remoto](/docs/es/remote-control): conecta esta sesión para alcanzar tus sesiones en otras máquinas
* [Configuración](/docs/es/settings-reference#all-settings): `crossSessionInbound`, `isolatePeerMachines` y `dialogExpiry`
* [Modos de permiso](/docs/es/permission-modes): los modos detrás de las dos clases del valor predeterminado de entrada
* [Referencia de herramientas](/docs/es/tools-reference): las filas `ListAgents` y `SendMessage` en la tabla de herramientas
* [Ejecutar agentes en paralelo](/docs/es/agents): compara las formas en que Claude Code ejecuta múltiples agentes
