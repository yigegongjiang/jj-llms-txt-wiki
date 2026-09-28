> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Compartir salida de sesión como artefactos

> Los artefactos convierten el trabajo de Claude Code en páginas interactivas en vivo en claude.ai que puede mantener privadas, compartir con su organización o publicar en un enlace público.

<Note>
  Los artefactos están disponibles en los planes Pro, Max, Team y Enterprise y requieren una sesión iniciada con [`/login`](/docs/es/setup#authenticate). Consulte [Disponibilidad](#availability) para ver el conjunto completo de requisitos.
</Note>

Un [artefacto](https://claude.com/features/artifacts) es una página web interactiva en vivo que Claude Code publica desde su sesión en una URL privada en claude.ai. La abre en un navegador y se actualiza en su lugar a medida que continúa la sesión. Compártala desde el encabezado de la página cuando desee que otra persona también la vea.

<Frame>
  <img src="https://mintcdn.com/claude-code/kaHIYYMIYMYPxQg9/images/artifacts-viewer.png?fit=max&auto=format&n=kaHIYYMIYMYPxQg9&q=85&s=dbfd671cdb0d15f49f808b9e89778fe1" alt="Un artefacto abierto en un navegador en claude.ai/code/artifact. El encabezado del visor muestra el título del artefacto acme-funnel-fix, un botón Compartir y el avatar del autor. El menú Compartir está abierto con el botón de alternancia Siempre compartir la versión más reciente, un selector de versión que lee Compartiendo versión 2, un selector de audiencia Todos en Acme y un botón Copiar enlace. Debajo del encabezado, la página del artefacto muestra dos maquetas móviles una al lado de la otra, un gráfico de embudo y una fila de tarjetas de métricas." width="2511" height="1890" data-path="images/artifacts-viewer.png" />
</Frame>

<h2 id="when-to-use-an-artifact">
  Cuándo usar un artefacto
</h2>

Use un artefacto cuando el texto de terminal es el medio incorrecto para lo que Claude produjo: salida que es más fácil de ver e interactuar que de leer línea por línea. Claude construye la página a partir de cualquier cosa que su sesión pueda alcanzar, incluido su base de código y los datos que extrae a través de sus [herramientas conectadas](/docs/es/mcp), por lo que la página puede mostrar cosas que tomaría párrafos describir. Por ejemplo, pida a Claude que:

* Guíe a un revisor a través de una solicitud de extracción con diffs anotados
* Represente un panel de control a partir de datos que la sesión ya extrajo
* Distribuya varias opciones de diseño o implementación una al lado de la otra
* Mantenga una línea de tiempo de investigación que se complete mientras se ejecuta una tarea larga
* Envíe a un compañero de equipo un enlace en lugar de pegar la salida en Slack
* Publique un tablero de estado que [extrae datos frescos a través de conectores MCP](#pull-live-data-with-mcp-connectors) cada vez que alguien lo abre

Consulte [Lo que puede construir](#what-you-can-build) para ver indicaciones que coincidan con estas, y [Extraer datos en vivo con conectores MCP](#pull-live-data-with-mcp-connectors) para la indicación del tablero respaldado por conectores.

<h3 id="what-an-artifact-is-not">
  Lo que un artefacto no es
</h3>

Un artefacto es una captura de trabajo: una página independiente sin backend, por lo que no puede servir múltiples rutas. Para una herramienta interna alojada con un backend, impleméntela en su propia infraestructura. Consulte [Restricciones de página](#page-constraints) para el conjunto completo de límites.

<h2 id="create-an-artifact">
  Crear un artefacto
</h2>

Claude puede publicar un artefacto por su cuenta cuando la salida se adapta a una página, o puede solicitar uno directamente. Para solicitar uno, nombre la característica o describa la salida visual que desea en lenguaje natural. Un buen candidato es cualquier cosa más fácil de ver que de leer como texto, como un diff anotado, un gráfico o un conjunto de opciones para comparar. Los prompts a continuación son dos ejemplos; consulte [Lo que puede crear](#what-you-can-build) para obtener más patrones.

```text wrap theme={null}
Make an artifact that walks through this PR with the diff annotated inline.
```

```text wrap theme={null}
Build a dashboard artifact of last week's deploy failures by service and keep it updated as you investigate.
```

A menos que nombre una ubicación, Claude escribe la página en un archivo HTML o Markdown en un directorio temporal fuera de su proyecto y luego la publica. Publicar un nuevo artefacto pasa por el [modo de permiso](/docs/es/permission-modes) de su sesión:

* **Modo Auto**: el clasificador revisa la publicación en lugar de solicitarle, por lo que Claude puede publicar una página sin que usted vea un prompt. El modo en el que comienzan sus sesiones depende de su plan; consulte [el modo de permiso inicial](/docs/es/permission-modes#eliminate-prompts-with-auto-mode).
* **Modos Manual y Aceptar ediciones**: Claude Code solicita permiso; podría decir algo como `Claude wants to publish deploy-failures.html, uploading it to claude.ai (Anthropic's servers) to host as the page "Deploy failures by service", private to you until you share it`. Seleccione **Yes** para publicar.

Después de que apruebe un artefacto una vez, Claude Code lo republica sin preguntar, y vuelve a preguntar en algunos casos, incluyendo cuando:

* Claude declara una capacidad de tiempo de ejecución para la página, como [llamadas de conector](#pull-live-data-with-mcp-connectors) o [descargas de archivos](#offer-a-file-download)
* Ha [compartido públicamente](#share-an-artifact)
* Ha compartido con personas específicas u su organización con la versión más reciente elegida como la versión que ven los espectadores

Después de la primera publicación, Claude imprime la URL y su navegador se abre a la nueva página. Si envió el prompt a través de [Remote Control](/docs/es/remote-control) desde claude.ai, Claude Desktop o la aplicación móvil de Claude, no se abre ninguna pestaña en la máquina que ejecuta la sesión. El navegador se abre allí la próxima vez que Claude publica el artefacto desde un prompt que escribe en la terminal. Presione `Ctrl+]` en cualquier momento para reabrir el artefacto más reciente de la sesión.

Claude elige el título del artefacto y un emoji, y ambos aparecen en su [galería de artefactos](#share-an-artifact) en claude.ai y en enlaces compartidos. Claude también puede elegir un icono de pestaña del navegador que coincida con lo que es la página, como un gráfico o un calendario. Pida a Claude un título, emoji o icono de pestaña específico si desea uno.

Para evitar que el navegador se abra automáticamente cuando se publica un nuevo artefacto, establezca `CLAUDE_CODE_ARTIFACT_AUTO_OPEN=0` en su entorno.

Si Claude responde que no puede publicar, o escribe un archivo HTML local sin un enlace, la herramienta no está habilitada para su sesión. Verifique los requisitos de [Disponibilidad](#availability).

<h2 id="update-an-artifact">
  Actualizar un artefacto
</h2>

Pida a Claude que revise la página, o permita que una tarea de larga duración se republique a medida que avanza. Claude edita el archivo subyacente y publica nuevamente en la misma URL.

```text wrap theme={null}
Agregue un desglose por región debajo del gráfico de resumen y republique.
```

Cualquiera que tenga la página abierta verá la actualización en su lugar. Cada publicación se convierte en una versión, y desde el control **Compartir** en el encabezado de la página puede elegir qué versión ven los espectadores.

Para actualizar un artefacto desde una sesión diferente, proporcione a Claude su URL, o adjúntelo con [`/artifacts`](#find-an-artifact-again). Sin ninguno de los dos, una nueva sesión crea un nuevo artefacto en lugar de actualizar uno.

```text wrap theme={null}
Actualizar https://claude.ai/code/artifact/5fbea6f3-... con los números de hoy.
```

<h2 id="find-an-artifact-again">
  Encontrar un artefacto nuevamente
</h2>

Ejecute `/artifacts` en Claude Code para enumerar todos los artefactos que posee y todos los artefactos compartidos con usted. Seleccione uno y presione `o` para abrirlo en su navegador o `c` para copiar su enlace. Presione `Enter` para adjuntarlo a la sesión actual; antes de v2.1.216, `Enter` lo abría en su navegador. Claude Code lee la lista de su cuenta claude.ai, por lo que funciona en una nueva sesión y después de `/clear`, cuando el enlace se ha desplazado fuera de la terminal. Requiere Claude Code v2.1.208 o posterior.

<h2 id="share-an-artifact">
  Compartir un artefacto
</h2>

Un nuevo artefacto es visible solo para usted. Para compartirlo, abra el artefacto en su navegador y use el control **Compartir** en el encabezado de la página. El encabezado también vincula a su galería en [claude.ai/code/artifacts](https://claude.ai/code/artifacts), que enumera todos los artefactos que ha creado.

Los espectadores en su organización pueden ver quién publicó la página: en un artefacto compartido dentro de su organización, su nombre está en el menú de título, y en un artefacto público está en el encabezado de la página para los espectadores que han iniciado sesión en su organización. Un espectador que abre un enlace público sin iniciar sesión, o desde fuera de su organización, ve la etiqueta `El contenido es generado por el usuario y no verificado.` en lugar de su nombre.

Con quién puede compartir depende de su plan:

* **Dentro de su organización**: en los planes Team y Enterprise, otorgue acceso a personas específicas en su organización, o a todos en ella. Los espectadores inician sesión en claude.ai como miembros de su organización para ver la página.
* **Públicamente**: comparta un enlace que cualquiera en internet pueda abrir, sin necesidad de iniciar sesión en claude.ai. En los planes Pro y Max, un enlace público es la única forma de compartir un artefacto. En los planes Team y Enterprise, el uso compartido público está desactivado hasta que un propietario [lo habilite para la organización](#control-public-sharing).

<h3 id="let-someone-edit-with-you">
  Permitir que alguien edite con usted
</h3>

Las personas con las que comparte son espectadores de forma predeterminada: ven cada versión que publica pero no pueden cambiar la página. En los planes Team y Enterprise, también puede hacer que alguien sea un editor. En el diálogo de uso compartido, agregue una persona y cambie su rol de **espectador** a **editor**.

Un editor publica nuevas versiones de la misma manera que usted [actualiza el artefacto desde otra sesión](#update-an-artifact): le proporciona a Claude la URL del artefacto, o lo adjunta desde [`/artifacts`](#find-an-artifact-again), y Claude extrae el contenido actual y lo republica con sus cambios. Todos los que tengan la página abierta ven cada actualización en vivo.

<h2 id="read-an-artifact-shared-with-you">
  Leer un artefacto compartido con usted
</h2>

Cuando alguien comparte un artefacto con usted, puede hacer que Claude lo lea: proporcione a Claude su URL, o adjúntelo desde [`/artifacts`](#find-an-artifact-again).

Claude lee una página que escribió otra persona de la manera en que lee una página web con [WebFetch](/docs/es/tools-reference#webfetch-tool-behavior): obtiene un resumen de lo que preguntó en lugar de la página sin procesar, y el resumen reporta instrucciones escritas en la página en lugar de transmitirlas. Claude Code también guarda el código fuente completo de la página en un archivo local, que Claude puede abrir cuando necesita el contenido exacto, como para republicar el artefacto como un [editor](#let-someone-edit-with-you).

<h2 id="collect-comments-on-an-artifact">
  Recopilar comentarios en un artefacto
</h2>

Cuando comparte un artefacto dentro de su organización, las personas con las que lo comparte pueden dejar comentarios en la página, y usted puede hacer que Claude lea esos comentarios y responda a ellos. Necesita Claude Code v2.1.221 o posterior y un plan de Team o Enterprise, porque solo un artefacto que [comparte dentro de su organización](#share-an-artifact) recibe comentarios. Claude lee los comentarios en dos casos:

* **Usted le pide a Claude que los lea**: proporcione a Claude la URL del artefacto y pida los comentarios. Claude enumera cada hilo y marca los comentarios que alguien que puede editar el artefacto le envió.
* **Alguien que puede editar el artefacto envía un comentario a Claude**: en un hilo en la página, envía un comentario con **Send to Claude**, o menciona `@claude` en uno. De cualquier manera, activan el hilo.

Claude puede responder o resolver solo un hilo activado. Otros hilos permanecen abiertos hasta que una persona los resuelve en la página. Los espectadores ven cada respuesta atribuida a Claude, a través de usted.

Si comparte un artefacto públicamente, los espectadores no pueden comentar en él: la página dice `Comments aren't available while this Artifact is shared publicly.` Para cambiar un artefacto que ya tiene hilos de comentarios a un enlace público, elimine los hilos primero.

Para pedir los comentarios usted mismo, proporcione a Claude la URL:

```text wrap theme={null}
Read the comments on https://claude.ai/code/artifact/5fbea6f3-... and make the changes the commenters ask for.
```

Si Claude le dice que no puede leer comentarios, confirme su versión, su sesión y su configuración de indicador de características:

* Está ejecutando Claude Code v2.1.221 o posterior.
* No está en su primera sesión desde que instaló Claude Code o actualizó desde una versión anterior a v2.1.221. En esa [primera sesión después de una instalación o actualización](/docs/es/env-vars#first-session-after-an-install-or-upgrade), Claude podría no ser capaz de leer comentarios aún; inicie una nueva sesión y pregunte de nuevo.
* No ha desactivado la obtención de indicadores de características.

<h3 id="let-claude-reply-to-comments-on-its-own">
  Permitir que Claude responda a comentarios por su cuenta
</h3>

Después de que su sesión publica un artefacto, Claude Code observa ese artefacto en busca de comentarios mientras la sesión se ejecute. Cuando alguien que puede editar el artefacto envía un comentario a Claude, llega a su sesión de inmediato, y Claude puede leer el hilo y responder sin que usted lo pida.

Necesita Claude Code v2.1.228 o posterior. Si desactivó la [obtención de indicadores de características](/docs/es/env-vars#features-that-need-feature-flag-fetching), Claude Code no observa comentarios.

Su [modo de permiso](/docs/es/permission-modes) decide qué hace Claude cuando llega un comentario enviado:

* **Claude responde por su cuenta**: cuando su modo de permiso permite que Claude publique la respuesta sin pedirle, Claude lee el hilo y responde, y edita el artefacto cuando el comentario pide un cambio. Usted ve `Auto-replied to comment thread on Artifact: <name>` o `Auto-edited Artifact: <name> in response to a comment thread`.
* **Claude espera por usted**: fuera del Plan Mode, cuando publicar la respuesta necesitaría su aprobación, usted ve `Comments are waiting on Artifact: <name>`. Claude entonces le pide aprobación para leer el hilo, y nuevamente para publicar la respuesta.
* **Claude pausa en Plan Mode**: usted ve `Comments are waiting on Artifact: <name>`, y Claude no responde hasta que salga del Plan Mode y le pida que lea y responda.

Claude también deja de responder por su cuenta a un artefacto después de manejar 60 comentarios enviados o activaciones de hilos en ese artefacto dentro de una hora. Usted ve `Comments are waiting on Artifact: <name>` una vez, y Claude retoma cuando los comentarios de esa hora envejecen.

Ejecute `/tasks` para ver cada artefacto que su sesión está observando, listado como una tarea de actualizaciones en vivo. Puede evitar que Claude responda por su cuenta de cualquiera de estas maneras:

* **Presione Ctrl+C una vez en un símbolo del sistema inactivo**: Claude pausa la respuesta en cada artefacto que su sesión está observando. Las respuestas comienzan de nuevo después de que envíe su siguiente mensaje.
* **Detenga la tarea en `/tasks`**: Claude deja de responder en ese artefacto hasta que le pida que reanude las respuestas allí. Publicar el artefacto de nuevo no inicia las respuestas de nuevo, y la parada aún se aplica cuando reanuda la sesión más tarde.
* **Presione `Ctrl+X Ctrl+K` dos veces dentro de 3 segundos**: el acorde que [detiene cada subagente de fondo en ejecución](/docs/es/interactive-mode#general-controls) también detiene a Claude de responder en cada artefacto para el resto de la sesión. Pedirle a Claude que reanude las respuestas no deshace esta parada.

Si el servicio que entrega comentarios no está disponible o deja de responder, Claude Code sigue intentando reconectarse por un tiempo, luego deja de observar cada artefacto que su sesión estaba observando.

<h2 id="pull-live-data-with-mcp-connectors">
  Extraer datos en vivo con conectores MCP
</h2>

Un artefacto puede llamar a [conectores MCP](/docs/es/mcp#use-mcp-servers-from-claude-ai) cada vez que alguien lo visualiza, de modo que la página muestra datos actuales en lugar de una instantánea de la sesión que la construyó. Las llamadas de conectores desde artefactos están disponibles en los planes Pro, Max, Team y Enterprise y requieren Claude Code v2.1.209 o posterior. En versiones anteriores, Claude publica la página con los datos que la sesión recopiló mientras la construía.

Para crear una página respaldada por un conector, nombre el conector y los datos que desea en su indicación:

```text wrap theme={null}
Build a dashboard artifact of our open pull requests that pulls the live list through my GitHub connector when the page loads.
```

Claude declara qué conectores puede llamar la página como parte de la publicación, y la página no puede llamar a conectores fuera de esa declaración. Solo califican los conectores de su cuenta claude.ai: Claude los nombra en la declaración, y cuando alguien visualiza la página, cada llamada [se ejecuta a través de la conexión propia de la cuenta que visualiza](#how-connector-calls-work-for-viewers) a ese conector. Los servidores MCP locales que configura en Claude Code, como servidores de `.mcp.json`, pueden proporcionar datos mientras Claude construye la página, pero la página publicada no puede llamarlos.

La página obtiene datos cuando se carga y puede actualizarse en un intervalo o cuando un visualizador utiliza un control de actualización en la página. Las respuestas se almacenan en caché en el navegador del visualizador, de modo que una página reabierta se representa a partir de las respuestas en caché inmediatamente, luego se actualiza con resultados frescos.

<h3 id="how-connector-calls-work-for-viewers">
  Cómo funcionan las llamadas de conectores para los visualizadores
</h3>

Cuando una página publicada llama a un conector, la llamada utiliza la cuenta de la persona que visualiza la página, no la cuenta de la persona que la publicó:

* **Cada visualizador utiliza sus propios conectores**: las llamadas se realizan a través de las herramientas conectadas de la cuenta que visualiza, de modo que dos personas que abran el mismo panel pueden ver datos diferentes según lo que sus cuentas puedan acceder. La página nunca ve las credenciales de nadie; claude.ai realiza las llamadas en nombre de la página.
* **Los visualizadores aprueban el acceso primero**: claude.ai pide permiso a cada visualizador antes de la primera llamada de conector de la página. Un visualizador que rechace, o que no haya conectado un conector que la página utiliza, aún ve la página sin sus secciones en vivo.
* **Las acciones también utilizan la cuenta del visualizador**: una página puede ofrecer controles que invoquen herramientas de conectores con efectos secundarios, como publicar un mensaje o actualizar un problema. La acción se realiza a través de la cuenta de quien selecciona el control.

Cuando planee compartir una página respaldada por un conector, pida a Claude que incluya un mensaje de respaldo en cada sección en vivo que nombre el conector que necesita. Un visualizador que no tenga la conexión entonces ve qué conectar en lugar de una sección vacía.

Un artefacto que llama a conectores no se puede compartir a un enlace público en ningún plan. En los planes Team y Enterprise, puede mantenerlo privado o [compartirlo dentro de su organización](#share-an-artifact). En los planes Pro y Max, donde un enlace público es la única forma de compartir, un artefacto respaldado por un conector permanece privado para usted.

<h3 id="the-page-shows-no-live-data-for-a-viewer">
  La página no muestra datos en vivo para un visualizador
</h3>

Cuando una página respaldada por un conector se representa pero sus secciones en vivo permanecen vacías para alguien con quien la compartió, trabaje a través de estas causas:

* **El visualizador no ha conectado el conector**: los conectores son por cuenta, de modo que cada visualizador necesita su propia conexión a cada conector que la página llama. Pueden agregar uno en **Configuración > Conectores** en claude.ai, luego recargar la página.
* **El visualizador rechazó la solicitud de permiso**: un rechazo dura el resto de esa carga de página. Recargar la página trae la solicitud de permiso de vuelta.
* **Las llamadas de conectores están desactivadas para la organización**: un Propietario controla el [conmutador **Habilitar conectores de artefactos**](#control-connector-calls-from-artifacts) en la configuración de administración.
* **La página llama a nombres de herramientas que el conector no expone**: las secciones afectadas permanecen vacías para todos, incluyéndolo a usted. Esto puede suceder cuando una página nombra las herramientas individuales detrás de un conector de estilo puerta de enlace que expone solo algunas de sus propias herramientas. Pida a Claude que corrija los nombres de herramientas que la página llama y publíquela de nuevo.

  Cuando Claude publica la página y las herramientas de ese conector están disponibles en su sesión, Claude Code verifica los nombres de herramientas que la página declara contra ellos, advierte a Claude sobre nombres que no coinciden, y rechaza la publicación cuando ninguno coincide. Antes de v2.1.265, publicaba la página sin verificarlos.

<h2 id="offer-a-file-download">
  Ofrecer una descarga de archivo
</h2>

Un artefacto puede ofrecer a los espectadores un archivo que genera la página, como una exportación CSV de una tabla o un PNG de un gráfico. El espectador lo guarda a través de un control de descarga en la página, como un botón. Las descargas de archivos son una capacidad en tiempo de ejecución que claude.ai habilita por cuenta, por lo que Claude verifica si su cuenta la tiene antes de construir el control.

Los espectadores no pueden guardar un archivo desde un enlace de descarga ordinario o un script en la página, porque el visor de artefactos en claude.ai bloquea cualquier descarga que inicie la página, incluidos los enlaces a URLs `data:` o `blob:`. Si una página tiene botones de descarga construidos de esa manera, pida a Claude que los reconstruya con la capacidad de descargas.

Para ofrecer un archivo, solicite el control y el formato de archivo en su indicación:

```text wrap theme={null}
Add a button that downloads this table as a CSV file.
```

Claude declara la capacidad de descargas como parte de la publicación, de la misma manera que [declara conectores](#pull-live-data-with-mcp-connectors).

<h2 id="what-you-can-build">
  Qué puede construir
</h2>

Un artefacto es una única página HTML, por lo que cualquier cosa que pueda expresar en HTML, CSS y JavaScript en línea está dentro del alcance. Los patrones a continuación son los más comunes.

<h3 id="walk-through-a-change">
  Recorrer un cambio
</h3>

Solicite una página que represente un diff o un cambio de diseño con anotaciones al lado de las líneas relevantes, para que los revisores puedan leer su razonamiento junto al código en lugar de reconstruirlo a partir de una descripción.

```text wrap theme={null}
Haga un artefacto que recorra este PR. Represente el diff con anotaciones de margen y codifique por colores los hallazgos por severidad.
```

<h3 id="compare-alternatives">
  Comparar alternativas
</h3>

Solicite varias variantes en una página para que pueda evaluarlas entre sí. Esto funciona para diseños, copias, formas de API o planes de implementación.

```text wrap theme={null}
Haga un artefacto con cuatro diseños distintamente diferentes para el panel de configuración. Varíe la densidad y la agrupación, y distribúyalos como una cuadrícula con un intercambio de una línea debajo de cada uno.
```

<h3 id="tune-with-interactive-controls">
  Ajustar con controles interactivos
</h3>

Solicite controles deslizantes, alternancias o campos de entrada vinculados a lo que está ajustando, para que pueda explorar valores directamente en lugar de describirlos.

```text wrap theme={null}
Construya un artefacto con controles deslizantes para la curva de suavizado, duración y retraso para que pueda probar valores en esta transición. Muestre la animación en vivo mientras los mueve.
```

<h3 id="bring-the-result-back-to-your-session">
  Traer el resultado de vuelta a su sesión
</h3>

Un artefacto puede actuar como un editor ligero para una decisión que luego devuelve a Claude. Solicite un control de exportación que produzca texto que pueda pegar en la terminal, para que el resultado de interactuar con la página fluya de vuelta a la sesión en lugar de permanecer en la página.

```text wrap theme={null}
Haga un artefacto de tablero de triaje con cada problema abierto como una tarjeta arrastrable en las columnas Ahora, Siguiente, Más tarde y Cortar. Agregue un botón "Copiar como indicación" que me dé el orden final para pegar aquí.
```

<h3 id="track-work-in-progress">
  Rastrear el trabajo en progreso
</h3>

Pida a Claude que mantenga un artefacto actualizado mientras se ejecuta una tarea larga, para que cualquiera con el enlace pueda seguir sin leer la terminal.

```text wrap theme={null}
Convierta este plan de migración en un artefacto de lista de verificación. Marque los elementos a medida que los complete y agregue una nota para cualquier cosa que omita.
```

<h2 id="improve-the-visual-design">
  Mejorar el diseño visual
</h2>

Claude aplica una skill de diseño integrada cuando construye un artefacto, por lo que las páginas obtienen una paleta deliberada, tipografía y diseño sin necesidad de indicaciones adicionales. Esa skill también busca un sistema de diseño existente en su proyecto antes de elegir el suyo propio. Los design tokens son los valores de color, tipografía y espaciado nombrados que su sistema de diseño reutiliza. Para mantener los artefactos consistentes con la marca de su producto, regístrelos donde Claude pueda encontrarlos, como el [CLAUDE.md](/docs/es/memory) del proyecto o un archivo de tema en su repositorio:

```markdown theme={null}
## Design system

- Colors: primary #1a4d8f, accent #f59e0b, surface #f8fafc
- Typography: Inter for body, JetBrains Mono for code
- Spacing: 8px scale, 6px border radius
```

Claude trata su sistema de diseño con mayor precedencia que sus propias opciones, y su prompt con mayor precedencia que ambas. El encabezado y el formato anterior son un ejemplo; cualquier lista clara de colores, fuentes y espaciado funciona.

Para la tipografía, Claude puede cargar una fuente tipográfica desde Google Fonts, la única fuente de fuente externa que una página de artefacto puede cargar. Claude inserta cualquier otra fuente tipográfica como un data URI `@font-face` y proporciona a cada fuente tipográfica una pila de alternativas, por lo que la página se sigue renderizando si una fuente no se carga. Para usar una fuente tipográfica específica, nómbrela en su prompt o en su sistema de diseño.

<h2 id="draft-a-design-canvas">
  Elaborar un lienzo de diseño
</h2>

Para crear un prototipo de una interfaz de usuario, un flujo de pantalla, una página de destino o un póster en lugar de construir una página, ejecute `/design` con un resumen. Claude elabora el diseño como artículos en un lienzo y publica el lienzo como un artefacto de diseño. El resumen nombra lo que desea que se dibuje:

```text wrap theme={null}
/design a settings screen for a mobile banking app
```

Abra el artefacto publicado en un navegador de escritorio para revisar los artículos. Seleccione un elemento en un artículo y cámbielo, y sus ediciones se guardan automáticamente. Puede exportar cada artículo como PNG o PDF.

`/design` requiere una sesión donde [los artefactos estén disponibles](#availability) y Claude Code v2.1.265 o posterior.

<h2 id="page-constraints">
  Restricciones de página
</h2>

Cada artefacto es una página independiente y autocontenida. Claude Code envuelve el archivo que publica en un shell de documento HTML y lo sirve bajo una Política de Seguridad de Contenido (CSP) estricta, que determina lo que la página puede hacer.

| Restricción                | Efecto                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Solicitudes externas       | La página puede cargar tipografías desde Google Fonts y scripts desde [cinco hosts CDN públicos](#allowlist-the-viewer-domain): cdnjs, unpkg, los CDN de Tailwind y jQuery, y rutas seleccionadas en jsDelivr como `/npm/`. La CSP bloquea todas las imágenes externas y todos los demás scripts, hojas de estilo y tipografías externas, y permite que las llamadas `fetch`, XHR y WebSocket lleguen solo al origen de la página y a los hosts de Google Fonts. Claude carga cualquier biblioteca que la página necesite desde uno de esos CDN, inserta en línea todo el CSS y JavaScript restante, e incrusta imágenes como URI de datos. Las [llamadas de Connector](#pull-live-data-with-mcp-connectors) se realizan a través de claude.ai, que realiza la llamada de red por sí misma. |
| Sin backend                | Un artefacto es una página estática. No puede autenticar espectadores por sí mismo.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| Descargas                  | La página no puede iniciar una descarga por sí misma. Para permitir que los espectadores guarden un archivo que genera la página, Claude declara la capacidad de descargas. Consulte [Ofrecer una descarga de archivo](#offer-a-file-download).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| Página única               | Los enlaces relativos no se resuelven, porque nada se implementa junto a la página. Para contenido de múltiples secciones, Claude utiliza anclajes dentro de la página en lugar de archivos separados.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| Tipos de archivo de origen | El archivo publicado debe ser `.html`, `.htm` o `.md`, y debe decodificarse como UTF-8, o como UTF-16 little-endian por su marca de orden de bytes. Los archivos Markdown se renderizan como páginas de documento con estilo con código resaltado por sintaxis. Un archivo que no se decodifica, o que contiene el carácter de reemplazo `U+FFFD`, se [rechaza con la línea y columna a corregir](/docs/es/errors#the-source-file-is-not-valid-utf-8-text).                                                                                                                                                                                                                                                                                                                                      |
| Tamaño renderizado         | La página renderizada debe tener 16 MiB o menos. Las imágenes incrustadas grandes son la causa habitual cuando una publicación falla por tamaño.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |

Generar un artefacto utiliza tokens de salida como cualquier otra respuesta, y una página con estilo es más intensiva en tokens que el mismo contenido como texto de terminal. CSS en línea, JavaScript para controles interactivos e imágenes incrustadas como URI de datos son los principales contribuyentes. Para reducir el costo de tokens de un artefacto:

* Prefiera SVG, o HTML y CSS, para diagramas sobre imágenes raster incrustadas
* Omita la interactividad que no necesita
* Haga que la página resuma grandes conjuntos de datos en lugar de incluirlos en su totalidad

<h2 id="availability">
  Disponibilidad
</h2>

Los artefactos requieren todas las condiciones a continuación. Cuando una no se cumple, Claude escribe un archivo HTML local o dice que no puede publicar.

| Requisito                | Disponible cuando                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :----------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Plan                     | Pro, Max, Team o Enterprise. En planes Pro y Max, los artefactos son privados para usted hasta que los comparta, y no se aplica ninguna gestión de administrador. En planes de Team, los artefactos están habilitados de forma predeterminada. En planes de Enterprise, un propietario [los habilita](#manage-artifacts-for-your-organization) en la configuración de administrador de claude.ai.                                                                                                     |
| Autenticación            | La sesión está respaldada por una cuenta de claude.ai: inicie sesión con `/login` en la CLI o la aplicación de escritorio. Las sesiones de Claude Tag están autenticadas a través de la identidad del agente, por lo que no se requiere ningún paso. Las sesiones que usan una clave API, [token de puerta de enlace](/docs/es/llm-gateway) o credencial de proveedor de nube no pueden publicar.                                                                                                          |
| Proveedor de modelo      | API de Anthropic. No disponible en [Amazon Bedrock](/docs/es/amazon-bedrock), [Google Cloud's Agent Platform](/docs/es/google-vertex-ai) o [Microsoft Foundry](/docs/es/microsoft-foundry).                                                                                                                                                                                                                                                                                                                          |
| Política de organización | Las claves de cifrado administradas por el cliente (CMEK), HIPAA y [Retención de datos cero](/docs/es/zero-data-retention) no están habilitadas para la organización.                                                                                                                                                                                                                                                                                                                                      |
| Superficie               | CLI de Claude Code, o la aplicación de escritorio Claude versión 1.13576.0 o posterior. Las sesiones de [Claude Tag](https://claude.com/docs/claude-tag/overview) también pueden publicar artefactos cuando tanto Claude Tag como los artefactos están habilitados para la organización. Deshabilitado de forma predeterminada en contextos de [Agent SDK](/docs/es/agent-sdk/overview), GitHub Action y MCP-server, y cuando [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/es/env-vars) está establecido. |

Si los artefactos están permitidos para su organización proviene de la política de su organización, que Claude Code carga desde `api.anthropic.com`. Cuando Claude Code no puede cargar la política, los artefactos no están disponibles. Cuando usted solicita uno, Claude explica por qué.

Si hay un proxy, VPN o filtro web involucrado, pida a su administrador de TI que permita `api.anthropic.com`. Claude Code sigue reintentando en segundo plano, y los artefactos se vuelven disponibles una vez que la política se carga y los permite.

<h2 id="disable-artifacts">
  Deshabilitar artefactos
</h2>

Para desactivar los artefactos en sus propias sesiones independientemente de la configuración de su organización, use cualquiera de:

| Dónde                                    | Qué hacer                                                                                                                                      |
| :--------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| [`/config`](/docs/es/commands)                | Desactive la fila **Artifacts**, que escribe [`"enableArtifact": false`](/docs/es/settings-reference#enableartifact) en su configuración de usuario |
| [Archivo de configuración](/docs/es/settings) | Establezca `"enableArtifact": false`. El deprecated `"disableArtifact": true` también desactiva los artefactos                                 |
| [Variable de entorno](/docs/es/env-vars)      | Establezca `CLAUDE_CODE_DISABLE_ARTIFACT=1`                                                                                                    |
| [Regla de permiso](/docs/es/permissions)      | Agregue `Artifact` a `permissions.deny`                                                                                                        |

Una vez que desactive los artefactos en un archivo [`--settings`](/docs/es/cli-reference#cli-flags) o con `CLAUDE_CODE_DISABLE_ARTIFACT`, o su administrador los desactiva en [configuración administrada](/docs/es/server-managed-settings), ningún archivo de configuración los vuelve a activar. Antes de v2.1.242, un archivo más alto en la [pila de precedencia](/docs/es/settings#settings-precedence) podría volver a activar los artefactos incluso cuando un archivo de precedencia más baja establecía `"enableArtifact": false`.

También puede establecer `"enableArtifact": false` en `.claude/settings.json` o `.claude/settings.local.json` de un proyecto para desactivar los artefactos en sesiones en ese proyecto. Un `"enableArtifact": true` en cualquiera de los archivos no los vuelve a activar. Honrar la clave en la configuración del proyecto y local requiere Claude Code v2.1.242 o posterior.

Si agrega una regla de negación o solicitud de `WebFetch` sin una parte `domain:`, no desactiva los artefactos ni bloquea las lecturas de artefactos. Una [regla `WebFetch(domain:claude.ai)` en `deny` o `ask` sí se aplica a las lecturas de artefactos](/docs/es/permissions#allow-or-deny-every-fetch).

<h2 id="manage-artifacts-for-your-organization">
  Gestionar artefactos para su organización
</h2>

Los administradores en planes de Team y Enterprise controlan los artefactos desde [la configuración de administrador de claude.ai](https://claude.ai/admin-settings/claude-code). El contenido del artefacto se almacena en la infraestructura operada por Anthropic y es visible solo para miembros autenticados de la organización publicadora, a menos que el artefacto sea [compartido públicamente](#control-public-sharing).

<h3 id="enable-or-disable-artifacts">
  Habilitar o deshabilitar artefactos
</h3>

Para habilitar o deshabilitar artefactos para toda la organización, vaya a [**Configuración > Claude Code > Capacidades**](https://claude.ai/admin-settings/claude-code) y use el botón de alternancia **Artefactos**. En planes de Enterprise con control de acceso basado en roles, también puede limitar los artefactos a roles específicos: vaya a [**Configuración > Roles**](https://claude.ai/admin-settings/roles), edite un rol y establezca el permiso **Artefactos** bajo el grupo **Claude Code**.

<h3 id="control-connector-calls-from-artifacts">
  Controlar llamadas de conectores desde artefactos
</h3>

[Las llamadas de conectores desde artefactos](#pull-live-data-with-mcp-connectors) tienen su propio botón de alternancia, separado del botón de alternancia **Artefactos** que activa o desactiva los artefactos. Vaya a [**Configuración > Capacidades**](https://claude.ai/admin-settings/capabilities) y use el botón de alternancia **Habilitar conectores de artefactos**. El mismo botón de alternancia rige las llamadas de conectores desde artefactos creados en conversaciones de claude.ai, por lo que se encuentra bajo **Configuración > Capacidades** en lugar de **Configuración > Claude Code**.

<h3 id="control-public-sharing">
  Controlar el compartir público
</h3>

El compartir público está desactivado de forma predeterminada en planes de Team y Enterprise, por lo que los miembros pueden compartir artefactos solo dentro de la organización hasta que un administrador lo active. Para permitir que los miembros publiquen artefactos en enlaces públicos que cualquiera pueda ver sin iniciar sesión, vaya a **Configuración > Claude Code > Capacidades** y active **Compartir externo** bajo el botón de alternancia **Artefactos**. Desactivarlo bloquea el acceso a través de enlaces públicos existentes sin cambiar la audiencia de cada artefacto; el acceso se reanuda si lo vuelve a habilitar.

<h3 id="set-a-retention-policy">
  Establecer una política de retención
</h3>

Para establecer cuánto tiempo se conservan los artefactos antes de la eliminación automática, vaya a [**Configuración > Controles de datos y privacidad**](https://claude.ai/admin-settings/data-privacy-controls). Puede establecer períodos de retención separados para artefactos que aún son privados para su autor y artefactos que han sido compartidos.

<h3 id="review-the-audit-log">
  Revisar el registro de auditoría
</h3>

Publicar, compartir y eliminar un artefacto aparecen en el registro de auditoría de su organización bajo los tipos de evento `claude_artifact_*`, la misma familia utilizada para artefactos creados en conversaciones de claude.ai.

<h3 id="allowlist-the-viewer-domain">
  Permitir el dominio del visor
</h3>

El visor en claude.ai carga cada artefacto desde un origen `*.claudeusercontent.com` aislado. Si su organización restringe el acceso a la red saliente, agregue ese dominio a su lista de permitidos junto con `claude.ai`. Consulte [Requisitos de acceso a la red](/docs/es/network-config#network-access-requirements) para la lista completa.

Un artefacto que carga una fuente tipográfica desde [Google Fonts](#improve-the-visual-design) también solicita `fonts.googleapis.com` y `fonts.gstatic.com`. Ambos hosts son opcionales. Si los bloquea, los artefactos se renderizan en fuentes tipográficas alternativas. Bloquee con un rechazo rápido en lugar de una caída silenciosa para que la solicitud de fuente falle inmediatamente en lugar de retrasar la primera representación de la página.

Los artefactos también pueden cargar bibliotecas de JavaScript, como React o un paquete de gráficos, desde `cdnjs.cloudflare.com`, `cdn.jsdelivr.net`, `cdn.tailwindcss.com`, `code.jquery.com` y `unpkg.com`, y desde ningún otro host externo. Si bloquea esos hosts, las partes de un artefacto que dependen de una biblioteca no funcionan, y a diferencia de una fuente bloqueada, una biblioteca bloqueada no tiene alternativa. Bloquee con un rechazo rápido aquí también, para que una solicitud de biblioteca bloqueada falle de inmediato en lugar de quedarse colgada hasta que se agote el tiempo de espera.

<h3 id="list-and-delete-artifacts-with-the-compliance-api">
  Enumerar y eliminar artefactos con la API de Cumplimiento
</h3>

La [API de Cumplimiento](https://docs.claude.com/en/api/compliance) proporciona puntos finales para enumerar los artefactos de una organización, recuperar el contenido de una versión específica y eliminar un artefacto:

| Método   | Punto final                                                         |
| :------- | :------------------------------------------------------------------ |
| `GET`    | `/v1/compliance/code/artifacts`                                     |
| `GET`    | `/v1/compliance/code/artifacts/{artifact_id}/versions/{version_id}` |
| `DELETE` | `/v1/compliance/code/artifacts/{artifact_id}`                       |

Para los esquemas de solicitud y respuesta, consulte la [referencia de la API de Cumplimiento](https://docs.claude.com/en/api/compliance/code/artifacts).

<h2 id="related-resources">
  Recursos relacionados
</h2>

* Explore [patrones de indicaciones y flujos de trabajo](/docs/es/prompt-library) que se emparejan con artefactos
* Convierta un indicador de artefacto que reutiliza en una [skill](/docs/es/skills) para que pueda invocarlo como un comando
* [Conecte servidores MCP](/docs/es/mcp) para que Claude pueda extraer datos en un artefacto mientras lo construye
