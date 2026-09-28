> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Cómo Claude Code utiliza el almacenamiento en caché de prompts

> Claude Code gestiona automáticamente el almacenamiento en caché de prompts. Vea por qué un cambio de modelo desencadena un turno lento sin caché, qué cuesta `/compact`, por qué las ediciones de CLAUDE.md no se aplican a mitad de sesión, y cómo verificar su tasa de aciertos de caché.

El almacenamiento en caché de prompts hace que Claude Code sea más rápido y eficiente en costos. Sin almacenamiento en caché, la API reprocesaría su historial completo en cada turno. Con almacenamiento en caché, reutiliza lo que ya procesó, factura la relectura a la [tasa de token en caché](https://platform.claude.com/docs/en/about-claude/pricing), y solo procesa completamente lo que cambió.

Claude Code gestiona el almacenamiento en caché de prompts automáticamente, a menos que lo [desactive](#disable-prompt-caching). Aún es útil saber cómo funciona el almacenamiento en caché de prompts, porque algunas acciones invalidan el caché y hacen que la siguiente respuesta sea más lenta y costosa mientras se reconstruye. Esta página cubre qué acciones son esas, por qué algunos ajustes esperan un reinicio para aplicarse, y cómo verificar el rendimiento del caché cuando el uso parece alto.

<h2 id="how-the-cache-is-organized">
  Cómo se organiza la caché
</h2>

Cada vez que envía un mensaje en Claude Code, realiza una nueva solicitud de API. El modelo no recuerda nada entre solicitudes, por lo que Claude Code reenvía el contexto completo: el aviso del sistema, el contexto de su proyecto, cada mensaje anterior y resultado de herramienta, y su nuevo mensaje. El contenido nuevo se añade al final, lo que significa que la mayoría de cada solicitud es idéntica a la anterior. El almacenamiento en caché de indicaciones es cómo la API evita reprocesar la parte que no cambió.

La API almacena en caché haciendo coincidir el inicio de cada solicitud, llamado prefijo, con el contenido que procesó recientemente. En un turno normal, el prefijo es la solicitud anterior completa y solo el intercambio más reciente es nuevo. La coincidencia es exacta, por lo que un cambio en cualquier lugar del prefijo recalcula todo lo que viene después. No hay almacenamiento en caché por archivo o por segmento. Consulte [cómo funciona el almacenamiento en caché de indicaciones](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#how-prompt-caching-works) en la referencia de API para el mecanismo subyacente.

<img src="https://mintcdn.com/claude-code/VbDJw--l6T9a9Wvm/images/prompt-caching-prefix.svg?fit=max&auto=format&n=VbDJw--l6T9a9Wvm&q=85&s=f2e8f0b8298a50305fe428ca3f1d1594" className="dark:hidden" alt="Cuatro turnos mostrados como barras horizontales crecientes. La solicitud de cada turno contiene todo del turno anterior más el intercambio más reciente añadido al final. En los turnos dos y tres, el prefijo sin cambios se lee de la caché y solo se procesa el nuevo intercambio. En el turno cuatro, el aviso del sistema cambió, por lo que el prefijo ya no coincide y toda la solicitud se reprocesa y se escribe." width="720" height="454" data-path="images/prompt-caching-prefix.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/prompt-caching-prefix-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=297dc1c639f0915cae858d0c4b6f3be5" className="hidden dark:block" alt="Cuatro turnos mostrados como barras horizontales crecientes. La solicitud de cada turno contiene todo del turno anterior más el intercambio más reciente añadido al final. En los turnos dos y tres, el prefijo sin cambios se lee de la caché y solo se procesa el nuevo intercambio. En el turno cuatro, el aviso del sistema cambió, por lo que el prefijo ya no coincide y toda la solicitud se reprocesa y se escribe." width="720" height="454" data-path="images/prompt-caching-prefix-dark.svg" />

Para aprovechar al máximo la coincidencia de prefijos, Claude Code ordena cada solicitud para que el contenido que rara vez cambia entre turnos venga primero:

| Capa                  | Contenido                                                      | Cambia cuando                                               |
| --------------------- | -------------------------------------------------------------- | ----------------------------------------------------------- |
| Aviso del sistema     | Instrucciones principales, definiciones de herramientas        | El conjunto de definiciones de herramientas cargadas cambia |
| Contexto del proyecto | CLAUDE.md, memoria automática, reglas sin alcance              | La sesión comienza, o después de `/clear` o `/compact`      |
| Conversación          | Sus mensajes, respuestas de Claude, resultados de herramientas | Cada turno                                                  |

Un cambio en la capa de conversación deja el aviso del sistema y el contexto del proyecto en caché. Un cambio en el aviso del sistema invalida todo, porque todo el contenido posterior ahora se encuentra detrás de un prefijo diferente. La tercera columna proporciona desencadenantes comunes en lugar de una lista exhaustiva, y las secciones a continuación cubren el conjunto completo.

La regla de coincidencia de prefijos explica la mayoría de los comportamientos en esta página. [Plan mode](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode) y [carga de skills](/docs/es/skills), por ejemplo, añaden sus instrucciones como mensajes de conversación, por lo que el prefijo en caché permanece intacto.

Dos configuraciones no aparecen en la tabla de capas pero aún afectan lo que permanece en caché:

* **Modelo**: cada modelo tiene su propia caché. Cambiar de modelo recalcula toda la solicitud incluso cuando el contenido es idéntico. Consulte [Cambiar de modelo](#switching-models) a continuación.
* **Nivel de esfuerzo**: en la mayoría de los modelos, cada nivel de esfuerzo tiene su propia caché, por lo que cambiar el esfuerzo a mitad de sesión recalcula toda la solicitud. En Opus 5.5 y Fable 5.1 con una clave de API o una suscripción de Claude, la caché permanece intacta de forma predeterminada. Consulte [Cambiar el nivel de esfuerzo](#changing-effort-level) a continuación.

<Tip>
  Elija su modelo y nivel de esfuerzo al principio de una sesión, luego guarde `/compact` para descansos naturales entre tareas. Cuantos menos cambios realice a mitad de tarea, mayor será su tasa de aciertos de caché.
</Tip>

<h3 id="where-the-cache-lives">
  Dónde vive la caché
</h3>

El almacenamiento en caché ocurre del lado del servidor, en cualquier infraestructura que sirva su modelo. Dónde es eso depende de cómo se autentique:

* **Clave de API, suscripción de Claude, o [Claude Platform on AWS](/docs/es/claude-platform-on-aws)**: la caché vive en la infraestructura de Anthropic, accesible a través de la [Claude API](https://platform.claude.com/docs)
* **Amazon Bedrock o Agent Platform de Google Cloud**: la caché vive en la infraestructura de servicio de su proveedor de nube
* **Microsoft Foundry**: depende de la [opción de alojamiento](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options) del despliegue. Los despliegues alojados en Azure se sirven en infraestructura de Azure; los despliegues alojados en Anthropic se sirven en la infraestructura de Anthropic
* **`ANTHROPIC_BASE_URL` personalizado o [LLM gateway](/docs/es/llm-gateway)**: la caché vive donde se reenvíen sus solicitudes, y si el almacenamiento en caché funciona depende de la puerta de enlace

Claude Code también añade contexto del sistema a mitad de la conversación, como notificaciones de cambios de archivo, y marca ese bloque para almacenamiento en caché en cada proveedor y conexión a menos que establezca [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/es/llm-gateway-protocol#disable-pre-release-capabilities), en cuyo caso ese bloque se envía sin caché.

En el punto final propio del proveedor, Amazon Bedrock y su [punto final de Mantle](/docs/es/amazon-bedrock#use-the-mantle-endpoint), Agent Platform de Google Cloud, y Microsoft Foundry almacenan en caché el bloque de la misma manera que lo hace la Claude API.

Cuando sus solicitudes pasan a través de una [LLM gateway](/docs/es/llm-gateway), un `ANTHROPIC_BASE_URL` personalizado, o una anulación de URL base del proveedor de nube como [`ANTHROPIC_BEDROCK_BASE_URL`](/docs/es/env-vars), lo que permanece en caché depende de cómo la puerta de enlace maneja los [marcadores de `cache_control`](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#explicit-cache-breakpoints) que Claude Code envía:

* **Los reenvía sin cambios**: el bloque y su conversación se almacenan en caché de la misma manera que en el punto final propio del proveedor.
* **Rechaza la solicitud marcada con un error `400` que nombra `cache_control`**: Claude Code reenvía la solicitud con el marcador movido fuera del bloque y hacia su último mensaje de conversación, y lo mantiene allí para el resto de la conversación. El bloque se factura como entrada sin caché; su conversación permanece en caché.
* **Elimina los marcadores mientras devuelve éxito**: todo su historial de conversación se factura como entrada sin caché en cada turno. Una puerta de enlace que convierte contenido del sistema en forma de bloque a una cadena simple elimina el marcador de la misma manera.

Para lo que cada proveedor almacena y procesa, consulte [uso de datos](/docs/es/data-usage). Dondequiera que viva la caché, las entradas expiran después de un período de inactividad, y [Duración de la caché](#cache-lifetime) a continuación cubre el TTL y cómo extenderlo.

<h2 id="actions-that-invalidate-the-cache">
  Acciones que invalidan la caché
</h2>

Estas acciones hacen que la siguiente solicitud pierda parte o toda la caché. Verá un turno más lento y costoso de una sola vez, después del cual el nuevo prefijo se almacena en caché. La mayoría de ellas se pueden evitar durante la tarea una vez que sabe que tienen un costo. Un cambio de modelo puede parecer gratuito hasta que note el turno más lento que sigue.

* [Cambiar modelos](#switching-models)
* [Cambiar el nivel de esfuerzo](#changing-effort-level)
* [Activar el modo rápido](#turning-on-fast-mode)
* [Conectar o desconectar un servidor MCP](#connecting-or-disconnecting-an-mcp-server)
* [Habilitar o deshabilitar un plugin](#enabling-or-disabling-a-plugin)
* [Denegar una herramienta completa](#denying-an-entire-tool)
* [Compactar la conversación](#compacting-the-conversation)
* [Acumular muchas imágenes](#accumulating-many-images)
* [Actualizar Claude Code](#upgrading-claude-code)

<h3 id="switching-models">
  Cambiar modelos
</h3>

Cada modelo tiene su propia caché. Cambiar con [`/model`](/docs/es/model-config#setting-your-model) significa que la siguiente solicitud lee todo el historial de conversación sin aciertos de caché, aunque el contenido sea idéntico.

Cuando ejecuta `/model` en la terminal, Claude Code le pide que confirme el cambio solo mientras la caché aún está activa y el nuevo modelo no es el que produjo la última respuesta. La caché permanece activa durante un [TTL de caché](#cache-lifetime) después de que Claude Code envió por última vez una solicitud en esta conversación o Claude respondió por última vez. Una vez que pasa ese tiempo, la caché ha expirado, por lo que Claude Code cambia sin preguntar.

Antes de v2.1.238, Claude Code no verificaba el TTL de caché y preguntaba incluso después de que la caché había expirado.

También puede requerir esta confirmación u omitirla con un [hook PreModelSwitch](/docs/es/hooks#premodelswitch-decision-control).

La [configuración del modelo `opusplan`](/docs/es/model-config#opusplan-model-setting) se resuelve a Opus durante el modo de plan y Sonnet durante la ejecución, por lo que cada alternancia del modo de plan es un cambio de modelo e inicia una caché nueva.

[El fallback automático del modelo](/docs/es/model-config#automatic-model-fallback) en modelos Fable, Opus 5.5 y Opus 5 también es un cambio de modelo. Cuando un clasificador de seguridad marca una solicitud en una categoría que tiene un modelo de fallback, Claude Code vuelve a ejecutar la solicitud en ese modelo y la sesión continúa allí.

Cuando la portada de una skill o comando nombra un [`model`](/docs/es/skills#frontmatter-reference) diferente del modelo actual de la sesión, ese turno también es un cambio de modelo: la siguiente solicitud lee todo el historial de conversación sin aciertos de caché. El modelo de sesión se reanuda en su siguiente solicitud. Una skill `context: fork` establece el [modelo del subagente bifurcado](/docs/es/skills#run-skills-in-a-subagent) en su lugar.

<h3 id="changing-effort-level">
  Cambiar el nivel de esfuerzo
</h3>

En la mayoría de los modelos, cambiar el [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) a mitad de sesión significa que la siguiente solicitud lee todo el historial de conversación sin aciertos de caché. Mientras la caché aún está activa, Claude Code le pide que confirme el cambio primero.

En Opus 5.5 y Fable 5.1 con una clave API o una suscripción a Claude, cambiar el esfuerzo mantiene la caché, y Claude Code aplica el nuevo nivel sin preguntar. Esto no se aplica en Amazon Bedrock, en la plataforma de agentes de Google Cloud, ni en una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway), ni cuando establece [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/es/llm-gateway-protocol#disable-pre-release-capabilities) o su organización tiene una configuración HIPAA.

Antes de v2.1.260, cambiar el esfuerzo en Fable 5.1 con una clave API o una suscripción a Claude también invalidaba la caché.

<h3 id="turning-on-fast-mode">
  Activar el modo rápido
</h3>

Habilitar el [modo rápido](/docs/es/fast-mode) agrega un encabezado de solicitud que es parte de la clave de caché, por lo que la primera solicitud que Claude Code envía con el modo rápido activado lee todo el historial de conversación sin aciertos de caché. Claude Code establece ese encabezado una vez cuando comienza un turno y lo mantiene durante todo el turno, por lo que cuando activa el modo rápido mientras Claude está trabajando, la pérdida de caché del encabezado ocurre en la primera solicitud de su siguiente turno. Esos tokens de entrada sin caché se facturan a [tasas de modo rápido](/docs/es/fast-mode#understand-the-cost-tradeoff), por lo que activarlo al inicio de una sesión cuesta menos que activarlo profundamente en una larga. Si su modelo actual no admite el modo rápido, habilitar el modo rápido también [cambia su modelo](#switching-models), y ese cambio inicia una caché nueva por sí solo desde la siguiente solicitud en el turno en ejecución.

El costo se aplica una vez por conversación. Después del primer turno de modo rápido, Claude Code sigue enviando el encabezado y varía solo la configuración de velocidad de la solicitud, que no es parte de la clave de caché. Desactivar el modo rápido, el [fallback automático a velocidad estándar](/docs/es/fast-mode#handle-rate-limits) después de un límite de velocidad, y activarlo nuevamente más tarde mantienen la caché. Si [se queda sin créditos de uso](/docs/es/fast-mode#handle-rate-limits) a mitad de sesión, Claude Code reintenta cada solicitud de modo rápido rechazada a velocidad estándar de la misma manera, por lo que este fallback también mantiene la caché. `/clear` y `/compact` restablecen esto, ya que reconstruyen la caché en esos puntos de todos modos.

<h3 id="connecting-or-disconnecting-an-mcp-server">
  Conectar o desconectar un servidor MCP
</h3>

Las definiciones de herramientas se encuentran en la capa del mensaje del sistema, por lo que la caché se invalida cuando el conjunto de definiciones de herramientas en la solicitud cambia entre turnos. Alternar la [herramienta de asesor](/docs/es/advisor) es una excepción: su definición se encuentra después del punto de ruptura de caché, por lo que habilitar o deshabilitar `/advisor` mantiene el prefijo almacenado en caché intacto. Si un cambio de [servidor MCP](/docs/es/mcp) hace esto depende de si sus herramientas se difieren por [búsqueda de herramientas](/docs/es/mcp#scale-with-mcp-tool-search) o se cargan en el prefijo:

* **Herramientas diferidas**, el valor predeterminado en modelos compatibles: un servidor que se conecta, desconecta o cambia su lista de herramientas solo agrega contenido nuevo y no perturba nada ya almacenado en caché.
* **Herramientas cargadas en el prefijo**: cualquier cambio en ellas invalida la caché. Esto ocurre cuando [la búsqueda de herramientas no está disponible o está deshabilitada](/docs/es/mcp#configure-tool-search), como en los modelos de la plataforma de agentes de Google Cloud anteriores a la generación Claude 4.5, con una puerta de enlace `ANTHROPIC_BASE_URL` personalizada, o en una [implementación de Microsoft Foundry alojada en Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options) una vez que Claude Code detecta que la implementación rechaza la búsqueda de herramientas. También ocurre para un servidor o herramienta marcada [`alwaysLoad`](/docs/es/mcp#exempt-a-server-from-deferral), y para definiciones mantenidas al frente por [carga basada en umbral](/docs/es/mcp#configure-tool-search).

Cuando las herramientas se cargan en el prefijo, la causa más común de una invalidación es un servidor que se conecta o desconecta a mitad de sesión, lo que puede ocurrir sin ninguna acción de su parte: el proceso de un servidor stdio sale, una sesión HTTP expira, o un servidor [se reconecta automáticamente después de una falla transitoria](/docs/es/mcp#automatic-reconnection). Un servidor conectado también puede enviar una [actualización de herramienta dinámica](/docs/es/mcp#dynamic-tool-updates) que cambia su lista de herramientas.

Editar su configuración de MCP no cambia la caché por sí solo. La nueva configuración entra en vigor solo después de un reinicio, que es cuando el servidor se conecta o desconecta.

<h3 id="enabling-or-disabling-a-plugin">
  Habilitar o deshabilitar un plugin
</h3>

Cuando habilita o deshabilita un [plugin](/docs/es/plugins/overview), lo que el cambio cuesta depende de qué tipos de componentes proporciona el plugin. Los casos a continuación cubren cada tipo de componente, cuándo Claude Code aplica el cambio y qué sucede cuando deshabilita un plugin nuevamente en la misma sesión.

<h4 id="plugin-components-that-keep-the-cache">
  Componentes de plugin que mantienen la caché
</h4>

Claude Code nunca invalida la caché para las skills, comandos, agentes, hooks, monitores o temas de un plugin. Agrega su contenido después de la conversación existente, por lo que la siguiente solicitud paga por ese contenido y aún lee todo lo anterior desde la caché.

<h4 id="plugins-that-provide-mcp-servers">
  Plugins que proporcionan servidores MCP
</h4>

Cuando habilita o deshabilita un plugin que proporciona [servidores MCP](/docs/es/plugins/components#mcp-servers), Claude Code sigue las mismas reglas que cuando [conecta o desconecta un servidor MCP](#connecting-or-disconnecting-an-mcp-server):

* Si Claude Code difiere las herramientas del servidor, mantiene la caché.
* Si Claude Code las carga en el prefijo, la siguiente solicitud vuelve a leer toda la conversación.

<h4 id="code-intelligence-plugins">
  Plugins de inteligencia de código
</h4>

Cuando habilita un [plugin de inteligencia de código](/docs/es/plugins/code-intelligence), Claude obtiene la [herramienta LSP](/docs/es/tools-reference#lsp-tool-behavior).

<h4 id="when-plugin-changes-apply">
  Cuándo se aplican los cambios de plugin
</h4>

Un cambio que realiza en el menú `/plugin` se realiza a través de [`/reload-plugins`](/docs/es/plugins/cli-reference#reload-plugins), que Claude Code ejecuta por usted cuando cierra el menú. Paga el costo, ya sean anuncios anexados o una relectura completa, en el primer turno después de que se aplique el cambio. Claude Code también puede aplicar un cambio por su cuenta:

* Para un plugin con una fuente `command`, Claude Code [puede recargar el plugin por sí solo](/docs/es/plugins/loading#when-a-command-source-re-runs).
* Cuando [instala un plugin desde la interfaz `/plugin`](/docs/es/plugins/install#install-a-plugin), Claude Code puede activarlo durante la instalación. El resumen de instalación le dice si lo hizo.
* Cuando [mueve la sesión con `/cd`](/docs/es/permissions#move-the-session-to-another-directory) en v2.1.246 o posterior, Claude Code aplica los plugins que la configuración del nuevo directorio habilita como parte del movimiento, sin la advertencia de relectura completa que retiene un `/reload-plugins`.
* En sesiones interactivas, cuando agrega o elimina un plugin en una [carpeta de plugins](/docs/es/plugins/create#load-a-directory-or-archive-for-one-session) que pasó con `--plugin-dir`, el cambio se aplica de inmediato. Si aplicarlo desencadenaría una relectura completa, Claude Code retiene el cambio en su lugar y muestra un aviso para ejecutar `/reload-plugins`. Requiere Claude Code v2.1.265 o posterior.

Cuando `/reload-plugins` se ejecuta y la recarga desencadenaría una relectura completa, Claude Code muestra una advertencia y no aplica la recarga. Ejecute `/reload-plugins --force` para aplicarla de todos modos.

`/reload-plugins` también se ejecuta en sesiones sin una terminal interactiva, como la aplicación de escritorio, el SDK del agente y [modo no interactivo](/docs/es/headless) con `-p`, cuando lo escribe directamente en la sesión. Requiere Claude Code v2.1.260 o posterior.

En esas sesiones, la recarga aplica todo excepto los cambios del servidor MCP del plugin, que [entran en vigor en su próxima sesión](/docs/es/plugins/cli-reference#reload-plugins) y por lo tanto nunca cuestan una relectura completa a mitad de sesión.

<h4 id="plugins-you-enable-and-then-disable-in-one-session">
  Plugins que habilita y luego deshabilita en una sesión
</h4>

Cuando deshabilita un plugin que habilitó anteriormente en la sesión, Claude Code restaura la forma de solicitud anterior. Si ese prefijo aún está dentro de su [duración de caché](#cache-lifetime), la siguiente solicitud lee la entrada de caché anterior en lugar de reconstruir.

<h3 id="denying-an-entire-tool">
  Denegar una herramienta completa
</h3>

Si agrega un nombre de herramienta simple como `Bash` o `WebFetch` como una [regla de denegación](/docs/es/permissions#manage-permissions), Claude no puede llamar a esa herramienta desde su siguiente solicitud en adelante, ya sea que agregue la regla a través de `/permissions` o [editando un archivo de configuración directamente](/docs/es/settings#when-edits-take-effect). Eso incluye una regla que agrega a través de `/permissions` en medio de un turno.

Cuando [la búsqueda de herramientas](/docs/es/mcp#scale-with-mcp-tool-search) está activa, que es el valor predeterminado en modelos compatibles, las definiciones de herramientas de la solicitud no cambian y el prefijo almacenado en caché sobrevive. Cuando la búsqueda de herramientas no está disponible o está deshabilitada, Claude Code elimina la definición de la siguiente solicitud, lo que invalida la caché, y lo mismo ocurre al eliminar la regla más tarde.

Solo una regla de denegación que coincida en la posición del nombre de la herramienta bloquea una herramienta de esta manera: un nombre de herramienta simple, la forma equivalente `Bash(*)`, o un [glob de nombre de herramienta](/docs/es/permissions#tool-name-wildcards) como `"*"`. Un glob que coincida solo con herramientas MCP, como `"mcp__*"`, bloquea esas herramientas de la misma manera. Las reglas de denegación con alcance como `Bash(rm *)`, y todas las reglas de permitir y preguntar, no cambian qué herramientas ve Claude. Claude Code las verifica cuando Claude intenta una llamada, dejando el prefijo intacto.

<h3 id="compacting-the-conversation">
  Compactar la conversación
</h3>

[La compactación](/docs/es/context-window#what-survives-compaction) reemplaza su historial de mensajes con un resumen. Por diseño, esto invalida la capa de conversación, ya que la siguiente solicitud tiene un historial nuevo y más corto que no comparte un prefijo con el anterior. Claude Code reutiliza la capa del mensaje del sistema a menos que la conversación se [reanudara mientras se mantiene un mensaje del sistema que de otro modo habría cambiado](#resuming-a-session); en ese caso, la primera compactación cambia al mensaje actual y esa capa se reconstruye una vez. Recarga el contexto del proyecto desde el disco, que solo obtiene aciertos de caché si CLAUDE.md y la memoria no han cambiado desde que comenzó la sesión.

Para producir el resumen, Claude Code envía una solicitud separada con el mismo mensaje del sistema, herramientas e historial que su conversación, más una instrucción de resumen anexada como un mensaje de usuario final. Mientras la caché está activa, esa solicitud lee su prefijo desde la caché, por lo que un `/compact` a mitad de sesión cuesta una fracción de lo que sugiere el tamaño del contexto y pasa la mayor parte de su tiempo generando el resumen.

Después de una pausa más larga que la [duración de caché](#cache-lifetime), no hay caché para leer, por lo que la solicitud de resumen reprocesa el historial completo como entrada sin caché. Por eso `/compact` cuesta más cuando [reanuda una sesión anterior](/docs/es/sessions#resume-from-a-summary). En ambos casos, cálido y frío, el turno después de la compactación reconstruye la caché de conversación solo para el resumen mucho más corto, por lo que ese turno no es la parte lenta.

<Tip>
  La compactación funciona a su favor cuando el contexto que descarta es contenido que ya no necesita. Para elegir cuándo ocurre su sobrecarga, ejecute `/compact` en un descanso natural en su trabajo, como entre tareas, en lugar de esperar a que la compactación automática se active a mitad de tarea. Si ha seguido un camino que desea abandonar por completo, [use `/rewind`](#rewinding-the-conversation) en su lugar para volver a un turno anterior. Rewind trunca a un prefijo que ya está almacenado en caché, en lugar de construir uno nuevo como lo hace la compactación.
</Tip>

<h3 id="accumulating-many-images">
  Acumular muchas imágenes
</h3>

La API limita cuántas imágenes y PDF puede llevar cada solicitud. Para los números actuales, consulte [Límites de solicitud](https://platform.claude.com/docs/en/build-with-claude/vision#request-limits) en los documentos de la API. Claude Code también limita el tamaño total de las imágenes y PDF en una solicitud, por lo que las capturas de pantalla grandes alcanzan el límite con menos imágenes que las pequeñas.

Cuando la siguiente solicitud pasaría cualquiera de los límites, Claude Code elimina un lote de las imágenes y PDF más antiguas de lo que envía, lo que deja espacio para más antes de que necesite eliminar alguna nuevamente. Claude ya no puede ver las imágenes eliminadas. Si Claude necesita una de ellas nuevamente, compártala nuevamente.

Eliminar imágenes cambia los mensajes que las contenían, por lo que la siguiente solicitud reprocesa la conversación desde la más antigua de esos mensajes en adelante. Debido a que Claude Code elimina un lote a la vez, ve un turno más lento por lote en lugar de uno con cada nueva captura de pantalla.

<h3 id="upgrading-claude-code">
  Actualizar Claude Code
</h3>

Una nueva versión de Claude Code típicamente actualiza el mensaje del sistema o las definiciones de herramientas, por lo que la primera conversación que inicia después de una actualización construye su caché desde el principio. [La actualización automática](/docs/es/setup#auto-updates) descarga nuevas versiones en segundo plano pero las aplica en el siguiente lanzamiento, nunca a mitad de sesión, por lo que ve esto como un primer turno sin caché después de reiniciar en lugar de una sorpresa durante una sesión. Establezca `DISABLE_AUTOUPDATER=1` para controlar cuándo se aplican las actualizaciones.

<Note>
  Para saber qué cuesta reanudar una conversación que inició antes de la actualización, consulte [Reanudar una sesión](#resuming-a-session).
</Note>

<h2 id="actions-that-keep-the-cache">
  Acciones que mantienen la caché
</h2>

Estas acciones o bien se añaden al final de la conversación o no tocan la solicitud en absoluto. Algunas de ellas, como editar CLAUDE.md, mantienen la caché por la misma razón que el cambio no llega a la sesión en ejecución hasta `/clear`, `/compact` o un reinicio.

* [Editar archivos en su repositorio](#editing-files-in-your-repository)
* [Editar CLAUDE.md durante la sesión](#editing-claude-md-mid-session)
* [Cambiar modo de permisos](#changing-permission-mode)
* [Cambiar estilo de salida](#changing-output-style)
* [Invocar skills y comandos](#invoking-skills-and-commands)
* [Ejecutar `/recap`](#running-%2Frecap)
* [Rebobinar la conversación](#rewinding-the-conversation)
* [Generar un subagente](#subagents-and-the-cache)

<h3 id="editing-files-in-your-repository">
  Editar archivos en su repositorio
</h3>

El contenido de los archivos entra en contexto solo cuando Claude los lee, y las lecturas se añaden a la conversación. Editar un archivo que Claude leyó previamente no cambia retroactivamente la lectura anterior en el historial. En su lugar, Claude Code añade un `<system-reminder>` notando que el archivo cambió, y Claude lo vuelve a leer si es necesario.

<h3 id="editing-claude-md-mid-session">
  Editar CLAUDE.md durante la sesión
</h3>

Sus archivos CLAUDE.md a nivel de raíz del proyecto y a nivel de usuario se leen una vez al inicio de la sesión y se mantienen en memoria. Editarlos durante la sesión no invalida la caché, pero la edición tampoco se aplica. Claude continúa trabajando con la versión que se cargó al inicio de la sesión. El nuevo contenido se carga en el próximo `/clear`, `/compact` o reinicio.

[Los archivos CLAUDE.md anidados en subdirectorios](/docs/es/memory) y [las reglas con frontmatter `paths:`](/docs/es/memory#path-specific-rules) se cargan más tarde, cuando Claude lee por primera vez un archivo coincidente. Editar uno antes de que se cargue sí tiene efecto. Después de que se carga, el contenido es parte del historial de conversación, por lo que una edición durante la sesión no lo cambia retroactivamente.

<h3 id="changing-permission-mode">
  Cambiar modo de permisos
</h3>

Cambiar entre [modos de permisos](/docs/es/permission-modes), como de Manual a aceptar ediciones, no cambia el prompt del sistema ni las definiciones de herramientas, por lo que los cambios de modo son seguros para la caché. La excepción es el modo plan con la configuración de modelo [`opusplan`](/docs/es/model-config#opusplan-model-setting), que cambia el modelo entre Opus y Sonnet cuando entra o sale del modo plan. Eso hace que el cambio de modo sea un [cambio de modelo](#switching-models).

<h3 id="changing-output-style">
  Cambiar estilo de salida
</h3>

Cuando cambia [estilos de salida](/docs/es/output-styles) durante la sesión con [`/output-style`](/docs/es/output-styles#change-your-output-style), `/config` o la configuración `outputStyle`, Claude usa el nuevo estilo a partir de su próximo mensaje. Claude Code entrega las instrucciones del nuevo estilo como un mensaje en la conversación, por lo que esa solicitud aún lee el prompt del sistema y la conversación anterior desde la caché.

Antes de v2.1.251, un cambio de estilo durante la sesión mantenía la caché pero no se aplicaba hasta que ejecutaba `/clear` o iniciaba una nueva sesión.

<h3 id="invoking-skills-and-commands">
  Invocar skills y comandos
</h3>

[Skills](/docs/es/skills) y [comandos](/docs/es/commands) inyectan sus instrucciones como mensajes de usuario en el punto de invocación. Nada anterior en la conversación cambia. Un skill o comando cuyo frontmatter nombra un `model` puede ser un [cambio de modelo](#switching-models) para ese turno.

<h3 id="running-/recap">
  Ejecutar `/recap`
</h3>

[`/recap`](/docs/es/interactive-mode#session-recap) genera un resumen para mostrar en su terminal. A diferencia de `/compact`, añade el resumen como salida de comando en lugar de reemplazar su historial de mensajes, por lo que el prefijo en caché permanece intacto.

<h3 id="rewinding-the-conversation">
  Rebobinar la conversación
</h3>

[`/rewind`](/docs/es/checkpointing) trunca su conversación hasta un turno anterior. El historial restante es el mismo contenido del que se construyó la caché en ese punto, y el prompt del sistema y las capas de contexto del proyecto no cambian, por lo que la siguiente solicitud accede a la entrada de caché anterior. Cada turno desde entonces ha leído a través de ese prefijo, lo que mantuvo la entrada activa incluso si el turno original fue hace más tiempo que el TTL.

Restaurar puntos de control de archivos junto con la conversación no tiene un efecto separado en la caché. El contenido de los archivos entra en contexto solo cuando Claude los lee, igual que [editar archivos en su repositorio](#editing-files-in-your-repository).

<h2 id="resuming-a-session">
  Reanudación de una sesión
</h2>

Cuando [reanuda una sesión](/docs/es/sessions#resume-a-session), Claude Code envía toda la conversación nuevamente, y la solicitud lee de la caché cualquier parte de su prefijo que no haya cambiado y que siga estando dentro de la [duración de la caché](#cache-lifetime). La tabla de capas en la parte superior de esta página indica qué cambios cada capa.

El mensaje del sistema cambiaría después de una [actualización de Claude Code](#upgrading-claude-code) o con un texto [`--append-system-prompt`](/docs/es/cli-reference#system-prompt-flags) diferente en la reanudación. De forma predeterminada, la conversación reanudada mantiene el mensaje del sistema con el que comenzó, por lo que su historial sigue estando detrás del mismo mensaje, y el cambio entra en vigor una vez que la conversación se compacta o en una nueva conversación. [Indicadores de mensaje del sistema en conversaciones reanudadas](/docs/es/cli-reference#system-prompt-flags-in-resumed-conversations) cubre los casos donde Claude Code reconstruye el mensaje en cada solicitud en su lugar.

<h2 id="cache-lifetime">
  Duración del caché
</h2>

Los prefijos en caché expiran después de un período de inactividad. Cada solicitud que acierta el caché reinicia el temporizador, por lo que el caché permanece activo mientras continúe trabajando. Después de una brecha lo suficientemente larga, la siguiente solicitud recalcula la entrada completa y restablece el caché, que es por qué el primer turno después de alejarse puede ser notablemente más lento.

En un plan Pro o Max, cuando reanuda una sesión grande después de un descanso prolongado, Claude Code [ofrece reanudar desde un resumen](/docs/es/sessions#resume-from-a-summary) para que las solicitudes posteriores no lleven el historial completo.

El tiempo de vida (TTL) controla cuánto tiempo la brecha el caché sobrevive. La API ofrece dos: un TTL de cinco minutos, y un [TTL de una hora](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#1-hour-cache-duration) que mantiene el caché activo a través de descansos más largos pero [factura escrituras de caché a una tasa más alta](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing). El TTL más largo ayuda cuando deja una sesión inactiva y vuelve a ella, porque omite el reprocesamiento que cuesta un prefijo expirado. Cuesta más en ráfagas cortas de trabajo que nunca se quedan inactivas más de cinco minutos, donde se aplica la tasa de escritura más alta y la duración de caché más larga no se utiliza.

<h3 id="which-ttl-each-request-gets">
  Qué TTL obtiene cada solicitud
</h3>

Claude Code decide el TTL por solicitud, y cada solicitud cae en uno de dos depósitos fijos:

* **Conversación principal**: sus turnos interactivos, ejecuciones `-p` no interactivas, y turnos del SDK de Agent, más los ayudantes que Claude Code ejecuta en línea con ellos
* **Todo lo demás**: las solicitudes que Claude Code realiza fuera de esa conversación, como [subagentes](/docs/es/sub-agents), [flujos de trabajo](/docs/es/workflows), [compañeros de equipo](/docs/es/agent-teams) en proceso, bifurcaciones, compactación, y títulos de sesión

A menos que elija un TTL usted mismo, Claude Code solicita el TTL de una hora solo en una suscripción de Claude dentro del uso incluido en su plan. Allí solicita la hora para la conversación principal, más un pequeño conjunto de solicitudes de ayuda que Anthropic controla del lado del servidor. Esta tabla proporciona el TTL predeterminado de cada depósito bajo ambos tipos de facturación.

| Depósito de solicitud  | Suscripción de Claude, dentro del uso del plan                                                     | Créditos de uso, clave de API, o proveedor de nube |
| ---------------------- | -------------------------------------------------------------------------------------------------- | -------------------------------------------------- |
| Conversación principal | Una hora                                                                                           | Cinco minutos                                      |
| Todo lo demás          | Cinco minutos, excepto las solicitudes de ayuda controladas por el servidor, que obtienen una hora | Cinco minutos                                      |

Una vez que supera el límite de uso de su plan y Claude Code utiliza [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans), se le factura por ese uso, por lo que Claude Code reduce la conversación principal al TTL de cinco minutos más económico. Para mantener el TTL de una hora allí, [elija el TTL usted mismo](#choose-the-ttl-yourself).

<h3 id="choose-the-ttl-yourself">
  Elija el TTL usted mismo
</h3>

Puede establecer un TTL para cualquier depósito. Cada control toma `5m` o `1h`, y Claude Code ignora cualquier otro valor.

* **Conversación principal**: la configuración [`promptCacheTtl`](/docs/es/settings-reference#promptcachettl), o la variable de entorno `CLAUDE_CODE_PROMPT_CACHE_TTL` [environment variable](/docs/es/env-vars)
* **Todo lo demás**: la configuración [`subagentPromptCacheTtl`](/docs/es/settings-reference#subagentpromptcachettl), o la variable de entorno `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`

Ambas configuraciones y ambas variables de entorno requieren Claude Code v2.1.242 o posterior. Si inicia sesión con una clave de API o utiliza un proveedor de nube, establezca `promptCacheTtl` en `1h` para dar a la conversación principal un caché de una hora. Las solicitudes fuera de ella mantienen el predeterminado de cinco minutos hasta que elija un TTL para ese depósito también.

Cuando se aplica más de un control, Claude Code toma la primera coincidencia en este orden:

1. `FORCE_PROMPT_CACHING_5M=1`, que fuerza cinco minutos para ambos depósitos
2. La variable de entorno del depósito
3. La configuración del depósito
4. Para las solicitudes de un subagente, el valor `cacheTtl` en el campo frontmatter [`experimental`](/docs/es/sub-agents#supported-frontmatter-fields) del subagente, que requiere Claude Code v2.1.248 o posterior. Claude Code ignora un `1h` allí mientras su suscripción de Claude está utilizando créditos de uso
5. `ENABLE_PROMPT_CACHING_1H=1`, que solicita una hora para ambos depósitos
6. El [predeterminado para el depósito de la solicitud](#which-ttl-each-request-gets)

Establezca `FORCE_PROMPT_CACHING_5M=1` cuando esté depurando el comportamiento del caché, comparando los dos TTL, o anulando un TTL más largo establecido en [configuración administrada](/docs/es/managed-settings).

Para confirmar qué TTL utilizaron las escrituras de caché de su conversación principal, ejecute `claude -p "hello" --output-format json` y lea `usage.cache_creation` en el resultado. Claude Code reporta escrituras de caché de una hora bajo `ephemeral_1h_input_tokens` y escrituras de caché de cinco minutos bajo `ephemeral_5m_input_tokens`.

A través de una puerta de enlace LLM que establece con `ANTHROPIC_BASE_URL`, parte de la solicitud de una hora viaja en el encabezado `anthropic-beta`, por lo que configure la puerta de enlace para [reenviar ese encabezado sin cambios](/docs/es/llm-gateway-protocol#request-headers). El TTL de una hora no está disponible a través de la [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway#availability-and-limitations). En Amazon Bedrock, el soporte de almacenamiento en caché de prompts, la longitud mínima de prefijo almacenable en caché, y la disponibilidad de TTL de una hora varían según el modelo. Si los recuentos de tokens de caché permanecen en cero, verifique [modelos soportados, regiones y límites](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models) en la documentación de Amazon Bedrock.

<h2 id="cache-scope">
  Alcance del caché
</h2>

En Claude Code, el caché está efectivamente limitado a una máquina y directorio. Cada conversación lleva consigo el directorio de trabajo, plataforma, shell, y versión del SO, y el prompt del sistema nombra sus rutas de memoria automática, por lo que dos sesiones en directorios diferentes construyen prefijos diferentes y se pierden el caché del otro. Eso incluye worktrees del mismo repositorio, ya que cada worktree tiene su propio directorio de trabajo.

Las sesiones que ejecuta en paralelo en el mismo directorio construyen prefijos coincidentes y leen el caché del otro. Las sesiones secuenciales comparten el prefijo solo cuando la instantánea de estado de git al inicio coincide, ya que cada conversación también lleva la rama y commits recientes de esa instantánea.

El caché de API subyacente es más amplio. Los cachés están aislados entre organizaciones, y en algunos proveedores, [entre espacios de trabajo dentro de una organización](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#cache-storage-and-sharing). Dentro de esos límites, cualquier dos solicitudes con el mismo modelo y prefijo leen el mismo caché. Para llamadores de Agent SDK que ejecutan flotas de procesos automatizados, vea [mejorar el almacenamiento en caché de prompts entre usuarios y máquinas](/docs/es/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) para suprimir las secciones por máquina del prompt del sistema y compartir el caché entre máquinas.

<h2 id="check-cache-performance">
  Verificar el rendimiento del caché
</h2>

El rendimiento del caché se muestra como dos recuentos de tokens que la API reporta en cada respuesta. La forma más directa de verlos en vivo es un [script de statusline](/docs/es/statusline) que lee el objeto `current_usage`:

| Campo                         | Significado                                                                                                                                                                                         |
| ----------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cache_creation_input_tokens` | Tokens escritos en el caché en este turno, facturados a la tasa de escritura de caché                                                                                                               |
| `cache_read_input_tokens`     | Tokens servidos desde caché en este turno, facturados a la [tasa de token en caché](https://platform.claude.com/docs/en/about-claude/pricing) del modelo, por debajo de la tasa de entrada estándar |

Una alta relación de lectura a creación significa que el almacenamiento en caché está funcionando bien. Si la creación permanece alta turno tras turno, algo está cambiando en su prefijo. La sección [acciones que invalidan el caché](#actions-that-invalidate-the-cache) enumera las causas usuales.

Para un resumen por sesión, ejecute `/usage`. Después de la primera respuesta de la conversación principal, Claude Code añade una [línea `Prompt cache (main)`](/docs/es/costs#prompt-cache-statistics) al bloque de sesión, mostrando la relación de aciertos de la sesión, el recuento de fallos y si el caché está activo en este momento. Un script de statusline puede leer los mismos números desde el [objeto `prompt_cache`](/docs/es/statusline#prompt-cache-fields). Ambos requieren Claude Code v2.1.251 o posterior.

La línea `Prompt cache (main)` también nombra la causa probable del último fallo cuando Claude Code puede identificar una, por ejemplo `likely cause: tool definitions changed`. El texto de causa probable requiere Claude Code v2.1.260 o posterior.

Para visibilidad en toda una organización, el exportador de OpenTelemetry reporta tokens de lectura y creación de caché por usuario y sesión. Vea [Monitor usage](/docs/es/monitoring-usage) para la referencia de métrica y atributo de evento.

<h2 id="subagents-and-the-cache">
  Subagentes y el caché
</h2>

Un [subagent](/docs/es/sub-agents) inicia su propia conversación con su propio prompt del sistema y conjunto de herramientas, separado del padre. Su primera solicitud no lee el caché del padre, porque los dos prefijos difieren, y calienta un caché propio a través de sus turnos. Los subagentes caen fuera del [bucket TTL](#which-ttl-each-request-gets) de la conversación principal, por lo que obtienen cinco minutos incluso en una suscripción hasta que [elija uno más largo](#choose-the-ttl-yourself).

El caché del padre no se ve afectado. Desde el lado del padre, la llamada y resultado del subagente se añaden a la conversación, dejando el prefijo del padre intacto.

Un [fork](/docs/es/sub-agents#fork-the-current-conversation), por el contrario, hereda el prompt del sistema del padre, herramientas e historial de conversación exactamente, por lo que su primera solicitud lee el caché del padre.

Otras solicitudes también pueden leer un prefijo que una solicitud anterior almacenó en caché:

* **Copias de sesión**: una sesión que [copia con `/fork`](/docs/es/agent-view#copy-the-session-with-%2Ffork) recibe su instrucción de aislamiento como un mensaje al final de la conversación copiada, por lo que el caché que la conversación original construyó permanece intacto.
* **Compactación**: la llamada de resumen descrita en [Compactar la conversación](#compacting-the-conversation) utiliza el mismo enfoque de compartir prefijo.
* **Subagentes reanudados**: cuando Claude [reanuda un subagent](/docs/es/sub-agents#resume-subagents), la primera solicitud de la ejecución reanudada puede leer el caché que la ejecución original calentó.
* **Fan-outs de flujo de trabajo**: en un [fan-out de flujo de trabajo](/docs/es/workflows#prompt-caching-in-a-fan-out) de agentes con el mismo prefijo, Claude Code retiene todos excepto el primero durante hasta 5 segundos por defecto, por lo que sus primeras solicitudes pueden leer el prefijo que el primer agente almacenó en caché.

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

Para establecer la política de almacenamiento en caché en toda una organización, coloque cualquiera de estas o las [variables de TTL](#cache-lifetime) en el bloque `env` de [configuración administrada](/docs/es/managed-settings). Para uso normal, deje el almacenamiento en caché habilitado.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Lecciones de construir Claude Code: El almacenamiento en caché de prompts lo es todo](https://claude.com/blog/lessons-from-building-claude-code-prompt-caching-is-everything): la justificación del diseño para el modo de plan, carga de herramientas diferida, y compactación
* [Explorar la ventana de contexto](/docs/es/context-window): qué se carga en contexto y cuándo
* [Reducir el uso de tokens](/docs/es/costs#reduce-token-usage): estrategias más allá del almacenamiento en caché para gestionar el tamaño del contexto
* [Rastrear y reducir costos](/docs/es/agent-sdk/cost-tracking): seguimiento de tokens de caché y configuración de TTL para llamadores de Agent SDK
* [Almacenamiento en caché de prompts](https://platform.claude.com/docs/es/build-with-claude/prompt-caching): el mecanismo de API subyacente, puntos de interrupción, y precios
