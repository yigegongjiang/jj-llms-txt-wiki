> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Cómo Claude Code utiliza el almacenamiento en caché de prompts

> Claude Code gestiona automáticamente el almacenamiento en caché de prompts. Vea por qué un cambio de modelo desencadena un turno lento sin caché, qué cuesta `/compact`, por qué las ediciones de CLAUDE.md no se aplican a mitad de sesión, y cómo verificar su tasa de aciertos de caché.

El almacenamiento en caché de prompts hace que Claude Code sea más rápido y eficiente en costos. Sin almacenamiento en caché, la API reprocesaría su historial completo en cada turno. Con almacenamiento en caché, reutiliza lo que ya procesó y solo realiza trabajo nuevo para lo que cambió.

Claude Code gestiona el almacenamiento en caché de prompts automáticamente, a menos que lo [desactive](#disable-prompt-caching). Aún es útil saber cómo funciona el almacenamiento en caché de prompts, porque algunas acciones invalidan el caché y hacen que la siguiente respuesta sea más lenta y costosa mientras se reconstruye. Esta página cubre qué acciones son esas, por qué algunos ajustes esperan un reinicio para aplicarse, y cómo verificar el rendimiento del caché cuando el uso parece alto.

<h2 id="how-the-cache-is-organized">
  Cómo se organiza el caché
</h2>

Cada vez que envía un mensaje en Claude Code, realiza una nueva solicitud de API. El modelo no recuerda nada entre solicitudes, por lo que Claude Code reenvía el contexto completo: el prompt del sistema, el contexto de su proyecto, cada mensaje anterior y resultado de herramienta, y su nuevo mensaje. El contenido nuevo se añade al final, lo que significa que la mayoría de cada solicitud es idéntica a la anterior. El almacenamiento en caché de prompts es cómo la API evita reprocesar la parte que no cambió.

La API almacena en caché haciendo coincidir el inicio de cada solicitud, llamado el prefijo, con el contenido que procesó recientemente. En un turno normal, el prefijo es la solicitud anterior completa y solo el intercambio más reciente es nuevo. La coincidencia es exacta, por lo que un cambio en cualquier lugar del prefijo recalcula todo después de él. No hay almacenamiento en caché por archivo o por segmento. Vea [cómo funciona el almacenamiento en caché de prompts](https://platform.claude.com/docs/es/build-with-claude/prompt-caching#how-prompt-caching-works) en la referencia de API para el mecanismo subyacente.

<img src="https://mintcdn.com/claude-code/VbDJw--l6T9a9Wvm/images/prompt-caching-prefix.svg?fit=max&auto=format&n=VbDJw--l6T9a9Wvm&q=85&s=f2e8f0b8298a50305fe428ca3f1d1594" className="dark:hidden" alt="Cuatro turnos mostrados como barras horizontales crecientes. La solicitud de cada turno contiene todo del turno anterior más el intercambio más reciente añadido al final. En los turnos dos y tres, el prefijo sin cambios se lee del caché y solo se procesa el nuevo intercambio. En el turno cuatro, el prompt del sistema cambió, por lo que el prefijo ya no coincide y toda la solicitud se reprocesa y se escribe." width="720" height="454" data-path="images/prompt-caching-prefix.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/prompt-caching-prefix-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=297dc1c639f0915cae858d0c4b6f3be5" className="hidden dark:block" alt="Cuatro turnos mostrados como barras horizontales crecientes. La solicitud de cada turno contiene todo del turno anterior más el intercambio más reciente añadido al final. En los turnos dos y tres, el prefijo sin cambios se lee del caché y solo se procesa el nuevo intercambio. En el turno cuatro, el prompt del sistema cambió, por lo que el prefijo ya no coincide y toda la solicitud se reprocesa y se escribe." width="720" height="454" data-path="images/prompt-caching-prefix-dark.svg" />

Para aprovechar al máximo la coincidencia de prefijos, Claude Code ordena cada solicitud para que el contenido que rara vez cambia entre turnos venga primero:

| Capa                  | Contenido                                                                 | Cambia cuando                                                                           |
| --------------------- | ------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| Prompt del sistema    | Instrucciones principales, definiciones de herramientas, estilo de salida | El conjunto de definiciones de herramientas cargadas cambia, o Claude Code se actualiza |
| Contexto del proyecto | CLAUDE.md, memoria automática, reglas sin alcance                         | La sesión comienza, o después de `/clear` o `/compact`                                  |
| Conversación          | Sus mensajes, respuestas de Claude, resultados de herramientas            | Cada turno                                                                              |

Un cambio en la capa de conversación deja el prompt del sistema y el contexto del proyecto en caché. Un cambio en el prompt del sistema invalida todo, porque todo el contenido posterior ahora se encuentra detrás de un prefijo diferente. La tercera columna proporciona desencadenantes comunes en lugar de una lista exhaustiva, y las secciones a continuación cubren el conjunto completo, incluido contenido como el estilo de salida que se fija al inicio de la sesión.

La regla de coincidencia de prefijos explica la mayoría de los comportamientos en esta página. [Plan mode](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode) y [skill loading](/docs/es/skills), por ejemplo, añaden sus instrucciones como mensajes de conversación, por lo que el prefijo en caché permanece intacto.

Dos ajustes no son parte del texto del prompt en absoluto, por lo que no aparecen en la tabla de capas, pero ambos son parte de la clave de caché:

* **Model**: cada modelo tiene su propio caché. Cambiar de modelo recalcula toda la solicitud incluso cuando el contenido es idéntico. Vea [Cambiar de modelo](#switching-models) a continuación.
* **Effort level**: cada nivel de esfuerzo tiene su propio caché para el mismo modelo. Cambiarlo a mitad de sesión recalcula toda la solicitud, y Claude Code le pide que confirme antes de aplicar el cambio. Vea [Cambiar nivel de esfuerzo](#changing-effort-level) a continuación.

<Tip>
  Elija su modelo y nivel de esfuerzo al principio de una sesión, luego guarde `/compact` para descansos naturales entre tareas. Cuantos menos cambios realice a mitad de tarea, mayor será su tasa de aciertos de caché.
</Tip>

<h3 id="where-the-cache-lives">
  Dónde vive el caché
</h3>

El almacenamiento en caché ocurre del lado del servidor, en cualquier infraestructura que sirva su modelo. Dónde es eso depende de cómo se autentique:

* **Clave de API, suscripción de Claude, o [Claude Platform on AWS](/docs/es/claude-platform-on-aws)**: el caché vive en la infraestructura de Anthropic, accedido a través de la [Claude API](https://platform.claude.com/docs)
* **Amazon Bedrock o Google Cloud's Agent Platform**: el caché vive en la infraestructura de servicio de su proveedor de nube
* **Microsoft Foundry**: las solicitudes se enrutan a la infraestructura de Anthropic
* **`ANTHROPIC_BASE_URL` personalizado o [LLM gateway](/docs/es/llm-gateway)**: el caché vive donde se reenvíen sus solicitudes, y si el almacenamiento en caché funciona depende de la puerta de enlace

Para lo que cada proveedor almacena y procesa, vea [data usage](/docs/es/data-usage). Dondequiera que viva el caché, las entradas expiran después de un período de inactividad, y [Cache lifetime](#cache-lifetime) a continuación cubre el TTL y cómo extenderlo.

<h2 id="actions-that-invalidate-the-cache">
  Acciones que invalidan el caché
</h2>

Estas acciones hacen que la siguiente solicitud pierda parte o todo el caché. Verá un turno más lento y costoso de una sola vez, después del cual el nuevo prefijo se almacena en caché. La mayoría de ellas se pueden evitar a mitad de tarea una vez que sabe que tienen un costo. Un cambio de modelo puede parecer gratuito hasta que note el turno más lento que sigue.

* [Cambiar de modelo](#switching-models)
* [Cambiar el nivel de esfuerzo](#changing-effort-level)
* [Activar el modo rápido](#turning-on-fast-mode)
* [Conectar o desconectar un servidor MCP](#connecting-or-disconnecting-an-mcp-server)
* [Habilitar o deshabilitar un plugin](#enabling-or-disabling-a-plugin)
* [Denegar una herramienta completa](#denying-an-entire-tool)
* [Compactar la conversación](#compacting-the-conversation)
* [Actualizar Claude Code](#upgrading-claude-code)

<h3 id="switching-models">
  Cambiar de modelo
</h3>

Cada modelo tiene su propio caché. Cambiar con [`/model`](/docs/es/model-config#setting-your-model) significa que la siguiente solicitud lee todo el historial de conversación sin aciertos de caché, aunque el contenido sea idéntico.

La [configuración de modelo `opusplan`](/docs/es/model-config#opusplan-model-setting) se resuelve a Opus durante el modo de plan y Sonnet durante la ejecución, por lo que cada alternancia de modo de plan es un cambio de modelo e inicia un caché nuevo.

El [respaldo automático de modelo](/docs/es/model-config#automatic-model-fallback) en Fable 5 también es un cambio de modelo. Cuando un clasificador de seguridad marca una solicitud, Claude Code la vuelve a ejecutar en el modelo Opus predeterminado y la sesión continúa allí.

<h3 id="changing-effort-level">
  Cambiar el nivel de esfuerzo
</h3>

El caché se indexa por [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) así como por modelo, por lo que cambiar con `/effort` significa que la siguiente solicitud lee todo el historial de conversación sin aciertos de caché. Una vez que una conversación ha comenzado, Claude Code muestra un diálogo de confirmación antes de aplicar un cambio de esfuerzo que invalidaría el caché. Un cambio que se resuelve al mismo nivel ya en vigor, como establecer explícitamente el valor predeterminado del modelo, omite el diálogo y mantiene el caché.

<h3 id="turning-on-fast-mode">
  Activar el modo rápido
</h3>

Habilitar [modo rápido](/docs/es/fast-mode) añade un encabezado de solicitud que forma parte de la clave de caché, por lo que la siguiente solicitud lee todo el historial de conversación sin aciertos de caché. Esos tokens de entrada sin caché se facturan a [tasas de modo rápido](/docs/es/fast-mode#understand-the-cost-tradeoff), por lo que activarlo al inicio de una sesión cuesta menos que activarlo profundamente en una larga. Habilitar el modo rápido desde un modelo que no es Opus también [cambia su modelo](#switching-models), lo que inicia un caché nuevo por sí solo.

El costo se aplica una vez por conversación. Después del primer turno de modo rápido, Claude Code sigue enviando el encabezado y varía solo la configuración de velocidad de la solicitud, que no forma parte de la clave de caché. Desactivar el modo rápido, la [reversión automática a velocidad estándar](/docs/es/fast-mode#handle-rate-limits) después de un límite de velocidad, y activarlo nuevamente más tarde mantienen el caché. `/clear` y `/compact` restablecen esto, ya que reconstruyen el caché en esos puntos de todas formas.

<h3 id="connecting-or-disconnecting-an-mcp-server">
  Conectar o desconectar un servidor MCP
</h3>

Las definiciones de herramientas se encuentran en la capa del prompt del sistema, por lo que el caché se invalida cuando el conjunto de definiciones de herramientas en la solicitud cambia entre turnos. Alternar la [herramienta de asesor](/docs/es/advisor) es una excepción: su definición se encuentra después del punto de ruptura de caché, por lo que habilitar o deshabilitar `/advisor` mantiene el prefijo en caché intacto. Si un cambio de [servidor MCP](/docs/es/mcp) hace esto depende de si sus herramientas se difieren por [búsqueda de herramientas](/docs/es/mcp#scale-with-mcp-tool-search) o se cargan en el prefijo:

* **Herramientas diferidas**, el valor predeterminado en modelos compatibles: un servidor que se conecta, desconecta o cambia su lista de herramientas solo añade contenido nuevo y no perturba nada ya almacenado en caché.
* **Herramientas cargadas en el prefijo**: cualquier cambio en ellas invalida el caché. Esto sucede cuando [la búsqueda de herramientas no está disponible o está deshabilitada](/docs/es/mcp#configure-tool-search), como en Google Cloud's Agent Platform o con una puerta de enlace `ANTHROPIC_BASE_URL` personalizada. También sucede para un servidor o herramienta marcada [`alwaysLoad`](/docs/es/mcp#exempt-a-server-from-deferral), y para definiciones mantenidas al frente por [carga basada en umbral](/docs/es/mcp#configure-tool-search).

Cuando las herramientas se cargan en el prefijo, la causa más común de una invalidación es un servidor que se conecta o desconecta a mitad de sesión, lo que puede suceder sin ninguna acción de su parte: el proceso de un servidor stdio sale, una sesión HTTP expira, o un servidor [se reconecta automáticamente después de una falla transitoria](/docs/es/mcp#automatic-reconnection). Un servidor conectado también puede enviar una [actualización de herramienta dinámica](/docs/es/mcp#dynamic-tool-updates) que cambia su lista de herramientas.

Editar su configuración de MCP no cambia el caché por sí solo. La nueva configuración entra en vigor solo después de un reinicio, que es cuando el servidor se conecta o desconecta.

<h3 id="enabling-or-disabling-a-plugin">
  Habilitar o deshabilitar un plugin
</h3>

Los [plugins](/docs/es/plugins) agrupan varios tipos de componentes, y el costo de un cambio depende de qué componentes proporciona el plugin. Skills, commands, agents, hooks, servidores LSP, monitores y temas nunca invalidan el caché: cualquier cosa que agreguen a la solicitud se añade después de la conversación existente, por lo que la siguiente solicitud paga por el contenido nuevo pero aún lee todo lo anterior desde el caché.

La excepción es un plugin que proporciona [servidores MCP](/docs/es/plugins-reference#mcp-servers). Habilitar o deshabilitar uno sigue las mismas reglas que [conectar o desconectar un servidor MCP](#connecting-or-disconnecting-an-mcp-server): el caché sobrevive cuando las herramientas del servidor se difieren, y la siguiente solicitud vuelve a leer toda la conversación cuando se cargan en el prefijo.

Los cambios de plugin se aplican cuando ejecuta [`/reload-plugins`](/docs/es/discover-plugins#apply-plugin-changes-without-restarting) o inicia una nueva sesión. El costo, ya sean anuncios añadidos o una relectura completa, se muestra en el primer turno después de la recarga, no cuando ejecuta `/plugin install`, `/plugin enable`, o `/plugin disable`. A partir de v2.1.163, cuando una recarga activaría la relectura completa, `/reload-plugins` muestra una advertencia y no aplica la recarga. Pase `--force` para aplicar de todas formas.

Deshabilitar un plugin que habilitó anteriormente en la sesión restaura la forma de solicitud anterior. Si ese prefijo aún está dentro de su [vida útil de caché](#cache-lifetime), la siguiente solicitud lee la entrada de caché más antigua en lugar de reconstruir.

<h3 id="denying-an-entire-tool">
  Denegar una herramienta completa
</h3>

Agregar un nombre de herramienta simple como `Bash` o `WebFetch` como una [regla de denegación](/docs/es/permissions#manage-permissions) elimina esa herramienta del contexto de Claude por completo. Las definiciones de herramientas integradas se cargan en la capa del prompt del sistema, por lo que agregar o eliminar una de estas reglas a mitad de sesión invalida el caché. El cambio entra en vigor en el siguiente turno, ya sea que lo agregue a través de `/permissions` o [editando un archivo de configuración directamente](/docs/es/settings#when-edits-take-effect).

Solo una regla de denegación que coincida en la posición del nombre de la herramienta tiene este efecto: un nombre de herramienta simple, la forma equivalente `Bash(*)`, o un [glob de nombre de herramienta](/docs/es/permissions#tool-name-wildcards) como `"*"`. Un glob que coincida solo con herramientas MCP, como `"mcp__*"`, elimina esas herramientas de la misma manera pero deja el caché intacto cuando las herramientas coincidentes se [difieren](#connecting-or-disconnecting-an-mcp-server), el valor predeterminado, ya que las definiciones diferidas nunca estuvieron en el prefijo en caché. Las reglas de denegación con alcance como `Bash(rm *)`, y todas las reglas de permitir y preguntar, no cambian qué herramientas ve Claude. Claude Code las verifica cuando Claude intenta una llamada, dejando el prefijo intacto.

<h3 id="compacting-the-conversation">
  Compactar la conversación
</h3>

[Compaction](/docs/es/context-window#what-survives-compaction) reemplaza su historial de mensajes con un resumen. Por diseño, esto invalida la capa de conversación, ya que la siguiente solicitud tiene un historial nuevo y más corto que no comparte un prefijo con el anterior. Claude Code reutiliza la capa del prompt del sistema y recarga el contexto del proyecto desde el disco, que solo tiene aciertos de caché si CLAUDE.md y la memoria no han cambiado desde que comenzó la sesión.

Para producir el resumen, Claude Code envía una solicitud única con el mismo prompt del sistema, herramientas e historial que su conversación, más una instrucción de resumen añadida como un mensaje de usuario final. Porque comparte su prefijo, esa solicitud lee el caché existente en lugar de reprocesar el historial completo. La mayoría del tiempo de compactación se dedica a generar el resumen, no a una pérdida de caché. El turno que sigue reconstruye el caché de conversación solo para el resumen mucho más corto, por lo que el turno posterior a la compactación no es la parte lenta.

<Tip>
  La compactación funciona a su favor cuando el contexto que descarta es contenido que ya no necesita. Para elegir cuándo ocurre su sobrecarga, ejecute `/compact` en un descanso natural en su trabajo, como entre tareas, en lugar de esperar a que la compactación automática se active a mitad de tarea. Si ha seguido un camino que desea abandonar completamente, [`/rewind`](#rewinding-the-conversation) a un turno anterior en su lugar. Rewind trunca de vuelta a un prefijo que ya está en caché, en lugar de construir uno nuevo como lo hace la compactación.
</Tip>

<h3 id="upgrading-claude-code">
  Actualizar Claude Code
</h3>

Una nueva versión de Claude Code típicamente actualiza el prompt del sistema o las definiciones de herramientas, por lo que la primera solicitud después de una actualización reconstruye el caché desde el principio. [Auto-update](/docs/es/setup#auto-updates) descarga nuevas versiones en segundo plano pero las aplica en el siguiente lanzamiento, nunca a mitad de sesión, por lo que ve esto como un primer turno sin caché después de reiniciar en lugar de una sorpresa durante una sesión. Establezca `DISABLE_AUTOUPDATER=1` para controlar cuándo se aplican las actualizaciones.

<Note>
  [Reanudar una sesión](/docs/es/sessions#resume-a-session) después de una actualización reprocesa todo el historial de conversación sin aciertos de caché, ya que el historial ahora se encuentra detrás de un prompt del sistema diferente. El costo se escala con la duración de la conversación reanudada, por lo que el primer turno de vuelta a una sesión larga puede ser la solicitud más costosa que envíe.
</Note>

<h2 id="actions-that-keep-the-cache">
  Acciones que mantienen el caché
</h2>

Estas acciones ya sea se añaden al final de la conversación o no tocan la solicitud en absoluto. Algunas de ellas, como editar CLAUDE.md o cambiar el estilo de salida, también son por qué un cambio de ajuste espera un reinicio para aplicarse.

* [Editar archivos en su repositorio](#editing-files-in-your-repository)
* [Editar CLAUDE.md a mitad de sesión](#editing-claude-md-mid-session)
* [Cambiar el estilo de salida](#changing-output-style)
* [Cambiar el modo de permiso](#changing-permission-mode)
* [Invocar skills y comandos](#invoking-skills-and-commands)
* [Ejecutar `/recap`](#running-%2Frecap)
* [Rewind de la conversación](#rewinding-the-conversation)
* [Generar un subagente](#subagents-and-the-cache)

<h3 id="editing-files-in-your-repository">
  Editar archivos en su repositorio
</h3>

El contenido del archivo entra en contexto solo cuando Claude lo lee, y las lecturas se añaden a la conversación. Editar un archivo que Claude leyó anteriormente no cambia retroactivamente la lectura anterior en el historial. En su lugar, Claude Code añade un `<system-reminder>` notando que el archivo cambió, y Claude lo relee si es necesario.

<h3 id="editing-claude-md-mid-session">
  Editar CLAUDE.md a mitad de sesión
</h3>

Sus archivos CLAUDE.md de raíz de proyecto y nivel de usuario se leen una vez al inicio de la sesión y se mantienen en memoria. Editarlos a mitad de sesión no invalida el caché, pero la edición tampoco se aplica. Claude continúa trabajando con la versión que se cargó al inicio de la sesión. El nuevo contenido se carga en el siguiente `/clear`, `/compact`, o reinicio.

[Archivos CLAUDE.md anidados en subdirectorios](/docs/es/memory) y [reglas con frontmatter `paths:`](/docs/es/memory#path-specific-rules) se cargan más tarde, cuando Claude lee por primera vez un archivo coincidente. Editar uno antes de que se cargue sí tiene efecto. Después de que se carga, el contenido es parte del historial de conversación, por lo que una edición a mitad de sesión no lo cambia retroactivamente.

<h3 id="changing-output-style">
  Cambiar el estilo de salida
</h3>

[Output style](/docs/es/output-styles) es parte del prompt del sistema, que Claude Code lee una vez al inicio de la sesión. Cambiarlo a través de `/config` o la configuración `outputStyle` a mitad de sesión no invalida el caché, pero el cambio tampoco se aplica. Claude continúa usando el estilo que se cargó al inicio de la sesión. El nuevo estilo se carga en el siguiente `/clear` o reinicio.

<h3 id="changing-permission-mode">
  Cambiar el modo de permiso
</h3>

Cambiar entre [permission modes](/docs/es/permission-modes), como de predeterminado a aceptar ediciones, no cambia el prompt del sistema o las definiciones de herramientas, por lo que los cambios de modo son seguros para el caché. La excepción es el modo de plan con la configuración de modelo [`opusplan`](/docs/es/model-config#opusplan-model-setting), que cambia el modelo entre Opus y Sonnet cuando entra o sale del modo de plan. Eso hace que el cambio de modo sea un [cambio de modelo](#switching-models).

<h3 id="invoking-skills-and-commands">
  Invocar skills y comandos
</h3>

[Skills](/docs/es/skills) y [commands](/docs/es/commands) inyectan sus instrucciones como mensajes de usuario en el punto de invocación. Nada anterior en la conversación cambia.

<h3 id="running-/recap">
  Ejecutar `/recap`
</h3>

[`/recap`](/docs/es/interactive-mode#session-recap) genera un resumen para mostrar en su terminal. A diferencia de `/compact`, añade el resumen como salida de comando en lugar de reemplazar su historial de mensajes, por lo que el prefijo en caché permanece intacto.

<h3 id="rewinding-the-conversation">
  Rewind de la conversación
</h3>

[`/rewind`](/docs/es/checkpointing) trunca su conversación de vuelta a un turno anterior. El historial restante es el mismo contenido del que se construyó el caché en ese punto, y las capas del prompt del sistema y contexto del proyecto no cambian, por lo que la siguiente solicitud acierta la entrada de caché anterior. Cada turno desde entonces ha leído a través de ese prefijo, que mantuvo la entrada activa incluso si el turno original fue hace más tiempo que el TTL.

Restaurar puntos de control de archivo junto con la conversación no tiene efecto separado en el caché. El contenido del archivo entra en contexto solo cuando Claude lo lee, igual que [editar archivos en su repositorio](#editing-files-in-your-repository).

<h2 id="cache-lifetime">
  Duración del caché
</h2>

Los prefijos en caché expiran después de un período de inactividad. Cada solicitud que acierta el caché reinicia el temporizador, por lo que el caché permanece activo mientras continúe trabajando. Después de una brecha lo suficientemente larga, la siguiente solicitud recalcula la entrada completa y restablece el caché, que es por qué el primer turno después de alejarse puede ser notablemente más lento.

El tiempo de vida (TTL) controla cuánto tiempo la brecha el caché sobrevive. La API ofrece dos: un TTL de cinco minutos, y un [TTL de una hora](https://platform.claude.com/docs/es/build-with-claude/prompt-caching#1-hour-cache-duration) que mantiene el caché activo a través de descansos más largos pero [factura escrituras de caché a una tasa más alta](https://platform.claude.com/docs/es/build-with-claude/prompt-caching#pricing). Claude Code elige el TTL para usted según cómo se autentique, y puede anularlo con variables de entorno.

<h3 id="on-a-claude-subscription">
  En una suscripción de Claude
</h3>

En una suscripción de Claude, Claude Code solicita automáticamente el TTL de una hora. El uso se incluye en su plan en lugar de facturarse por token, por lo que el TTL más largo no le cuesta nada extra y solo afecta cuánto tiempo su caché permanece activo.

Si ha superado el límite de uso de su plan y Claude Code está utilizando [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans), se le factura por ese uso, por lo que Claude Code automáticamente reduce el TTL a cinco minutos.

<h3 id="on-an-api-key-or-third-party-provider">
  En una clave de API o proveedor de terceros
</h3>

En una clave de API, Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, o Claude Platform on AWS, paga las tasas por token, por lo que el TTL permanece en los cinco minutos más baratos por defecto. Para optar por el [TTL de una hora](https://platform.claude.com/docs/es/build-with-claude/prompt-caching#1-hour-cache-duration), establezca `ENABLE_PROMPT_CACHING_1H=1`.

En Amazon Bedrock, el soporte de almacenamiento en caché de prompts, la longitud mínima de prefijo almacenable en caché, y la disponibilidad de TTL de una hora varían según el modelo. Si los recuentos de tokens de caché permanecen en cero, verifique [modelos soportados, regiones y límites](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models) en la documentación de Amazon Bedrock.

<h3 id="override-the-ttl">
  Anular el TTL
</h3>

Establezca `FORCE_PROMPT_CACHING_5M=1` para forzar el TTL de cinco minutos independientemente de la autenticación. Esto es útil cuando está depurando el comportamiento del caché, comparando los dos TTL, o anulando un `ENABLE_PROMPT_CACHING_1H` establecido en [managed settings](/docs/es/settings#settings-files).

<h2 id="cache-scope">
  Alcance del caché
</h2>

En Claude Code, el caché está efectivamente limitado a una máquina y directorio. El prompt del sistema incorpora el directorio de trabajo, plataforma, shell, versión del SO, y rutas de memoria automática, por lo que dos sesiones en directorios diferentes construyen prefijos diferentes y se pierden el caché del otro. Eso incluye worktrees del mismo repositorio, ya que cada worktree tiene su propio directorio de trabajo.

Las sesiones que ejecuta en paralelo en el mismo directorio construyen prefijos coincidentes y leen el caché del otro. Las sesiones secuenciales comparten el prefijo solo cuando la instantánea de estado de git al inicio coincide, ya que el prompt del sistema también captura rama y commits recientes.

El caché de API subyacente es más amplio. Los cachés están aislados entre organizaciones, y en algunos proveedores, [entre espacios de trabajo dentro de una organización](https://platform.claude.com/docs/es/build-with-claude/prompt-caching#cache-storage-and-sharing). Dentro de esos límites, cualquier dos solicitudes con el mismo modelo y prefijo leen el mismo caché. Para llamadores de Agent SDK que ejecutan flotas de procesos automatizados, vea [mejorar el almacenamiento en caché de prompts entre usuarios y máquinas](/docs/es/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) para suprimir las secciones por máquina del prompt del sistema y compartir el caché entre máquinas.

<h2 id="check-cache-performance">
  Verificar el rendimiento del caché
</h2>

El rendimiento del caché se muestra como dos recuentos de tokens que la API reporta en cada respuesta. La forma más directa de verlos en vivo es un [script de statusline](/docs/es/statusline) que lee el objeto `current_usage`:

| Campo                         | Significado                                                                                                   |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `cache_creation_input_tokens` | Tokens escritos en el caché en este turno, facturados a la tasa de escritura de caché                         |
| `cache_read_input_tokens`     | Tokens servidos desde caché en este turno, facturados a aproximadamente el 10% de la tasa de entrada estándar |

Una alta relación de lectura a creación significa que el almacenamiento en caché está funcionando bien. Si la creación permanece alta turno tras turno, algo está cambiando en su prefijo. La sección [acciones que invalidan el caché](#actions-that-invalidate-the-cache) enumera las causas usuales.

Para visibilidad en toda una organización, el exportador de OpenTelemetry reporta tokens de lectura y creación de caché por usuario y sesión. Vea [Monitor usage](/docs/es/monitoring-usage) para la referencia de métrica y atributo de evento.

<h2 id="subagents-and-the-cache">
  Subagentes y el caché
</h2>

Un [subagent](/docs/es/sub-agents) inicia su propia conversación con su propio prompt del sistema y conjunto de herramientas, separado del padre. Construye su propio caché, comenzando sin aciertos de caché en su primera llamada y calentándose a través de sus propios turnos. Los subagentes usan el TTL de cinco minutos incluso en una suscripción, ya que el TTL automático de una hora se aplica a la conversación principal.

El caché del padre no se ve afectado. Desde el lado del padre, la llamada y resultado del subagente se añaden a la conversación, dejando el prefijo del padre intacto.

Un [fork](/docs/es/sub-agents#fork-the-current-conversation), por el contrario, hereda el prompt del sistema del padre, herramientas e historial de conversación exactamente, por lo que su primera solicitud lee el caché del padre. La llamada de resumen de compactación descrita en [Compactar la conversación](#compacting-the-conversation) usa el mismo enfoque de compartir prefijo.

<h2 id="disable-prompt-caching">
  Desactivar el almacenamiento en caché de prompts
</h2>

Desactivar el almacenamiento en caché es ocasionalmente útil cuando se depura el comportamiento del almacenamiento en caché con un modelo o proveedor específico. Para desactivarlo, establezca una de estas variables de entorno a `1`:

| Variable                        | Efecto                            |
| ------------------------------- | --------------------------------- |
| `DISABLE_PROMPT_CACHING`        | Desactivar para todos los modelos |
| `DISABLE_PROMPT_CACHING_HAIKU`  | Desactivar solo para Haiku        |
| `DISABLE_PROMPT_CACHING_SONNET` | Desactivar solo para Sonnet       |
| `DISABLE_PROMPT_CACHING_OPUS`   | Desactivar solo para Opus         |
| `DISABLE_PROMPT_CACHING_FABLE`  | Desactivar solo para Fable        |

Para establecer la política de almacenamiento en caché en toda una organización, coloque cualquiera de estas o las [variables de TTL](#cache-lifetime) en el bloque `env` de [configuración administrada](/docs/es/settings#settings-files). Para uso normal, deje el almacenamiento en caché habilitado.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Lecciones de construir Claude Code: El almacenamiento en caché de prompts lo es todo](https://claude.com/blog/lessons-from-building-claude-code-prompt-caching-is-everything): la justificación del diseño para el modo de plan, carga de herramientas diferida, y compactación
* [Explorar la ventana de contexto](/docs/es/context-window): qué se carga en contexto y cuándo
* [Reducir el uso de tokens](/docs/es/costs#reduce-token-usage): estrategias más allá del almacenamiento en caché para gestionar el tamaño del contexto
* [Rastrear y reducir costos](/docs/es/agent-sdk/cost-tracking): seguimiento de tokens de caché y configuración de TTL para llamadores de Agent SDK
* [Almacenamiento en caché de prompts](https://platform.claude.com/docs/es/build-with-claude/prompt-caching): el mecanismo de API subyacente, puntos de interrupción, y precios
