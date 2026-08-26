> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Compartir la salida de la sesión como artefactos

> Los artefactos convierten el trabajo de Claude Code en páginas interactivas en vivo en claude.ai que puede mantener privadas, compartir con su organización o publicar en un enlace público.

<Note>
  Los artefactos están disponibles en los planes Pro, Max, Team y Enterprise y requieren una sesión iniciada con [`/login`](/docs/es/setup#authenticate). Consulte [Disponibilidad](#availability) para ver el conjunto completo de requisitos.
</Note>

Un artefacto es una página web interactiva en vivo que Claude Code publica desde su sesión en una URL privada en claude.ai. Puede abrirla en un navegador y se actualiza en el lugar mientras la sesión continúa. Compártala desde el encabezado de la página cuando desee que otra persona también la vea. Por ejemplo, use un artefacto para guiar a un revisor a través de una solicitud de extracción con diffs anotados, crear un panel de control a partir de datos de sesión, o mantener una línea de tiempo de investigación que se complete mientras Claude trabaja.

<Frame>
  <img src="https://mintcdn.com/claude-code/kaHIYYMIYMYPxQg9/images/artifacts-viewer.png?fit=max&auto=format&n=kaHIYYMIYMYPxQg9&q=85&s=dbfd671cdb0d15f49f808b9e89778fe1" alt="Un artefacto abierto en un navegador en claude.ai/code/artifact. El encabezado del visor muestra el título del artefacto acme-funnel-fix, un botón Compartir y el avatar del autor. El menú Compartir está abierto con el botón de alternancia Siempre compartir la versión más reciente, un selector de versión que dice Compartiendo versión 2, un selector de audiencia Todos en Acme, y un botón Copiar enlace. Debajo del encabezado, la página del artefacto muestra dos maquetas móviles una al lado de la otra, un gráfico de embudo y una fila de tarjetas de métricas." width="2511" height="1890" data-path="images/artifacts-viewer.png" />
</Frame>

<h2 id="when-to-use-an-artifact">
  Cuándo usar un artefacto
</h2>

Use un artefacto cuando el texto de terminal sea el medio incorrecto para lo que Claude produjo: salida que es más fácil de ver e interactuar que de leer línea por línea. Claude construye la página a partir de cualquier cosa a la que su sesión pueda acceder, incluida su base de código y los datos que extrae a través de sus [herramientas conectadas](/docs/es/mcp), por lo que la página puede mostrar cosas que tomaría párrafos describir. Por ejemplo, pida a Claude que:

* Guíe a un revisor a través de una solicitud de extracción con diffs anotados
* Represente un panel de control a partir de datos que la sesión ya extrajo
* Distribuya varias opciones de diseño o implementación una al lado de la otra
* Mantenga una línea de tiempo de investigación que se complete mientras se ejecuta una tarea larga
* Envíe a un compañero de equipo un enlace en lugar de pegar la salida en Slack
* Publique un tablero de estado que [extrae datos frescos a través de conectores MCP](#pull-live-data-with-mcp-connectors) cada vez que alguien lo abre

Consulte [Qué puede construir](#what-you-can-build) para ver indicaciones que coincidan con estas, y [Extraer datos en vivo con conectores MCP](#pull-live-data-with-mcp-connectors) para la indicación del tablero respaldado por conectores.

<h3 id="what-an-artifact-is-not">
  Qué no es un artefacto
</h3>

Un artefacto es una captura del trabajo, no una aplicación. Es una página única y autónoma sin backend, por lo que no puede almacenar entrada de formulario o servir múltiples rutas, y su única ruta a datos externos cuando alguien lo visualiza es [llamar a conectores MCP](#pull-live-data-with-mcp-connectors). Para una herramienta interna alojada con un backend, impleméntela en su propia infraestructura. Consulte [Restricciones de página](#page-constraints) para ver el conjunto completo de límites.

<h2 id="create-an-artifact">
  Crear un artefacto
</h2>

Claude puede publicar un artefacto por su cuenta cuando la salida se adapta a una página, o puede solicitar uno directamente. Para solicitar uno, nombre la característica o describa la salida visual que desea en lenguaje natural. Un buen candidato es cualquier cosa más fácil de ver que de leer como texto, como un diff anotado, un gráfico o un conjunto de opciones para comparar. Los indicadores a continuación son dos ejemplos; consulte [Qué puede construir](#what-you-can-build) para más patrones.

```text wrap theme={null}
Haga un artefacto que recorra este PR con el diff anotado en línea.
```

```text wrap theme={null}
Construya un artefacto de panel de control de los fallos de implementación de la semana pasada por servicio y manténgalo actualizado mientras investiga.
```

Claude escribe la página en un archivo HTML o Markdown en su proyecto y luego la publica. Antes de publicar un nuevo artefacto, Claude Code solicita permiso; podría decir algo como `Claude quiere publicar "Fallos de implementación por servicio" (deploy-failures.html) en una página privada en claude.ai`. Volver a publicar un artefacto que ya ha aprobado no solicita permiso nuevamente.

Seleccione **Sí** para publicar. Claude imprime la URL y su navegador se abre a la nueva página. Presione `Ctrl+]` en cualquier momento para volver a abrir el artefacto más reciente desde la terminal.

Claude elige el título del artefacto y un emoji para su icono de pestaña del navegador. Ambos aparecen en su [galería de artefactos](#share-an-artifact) en claude.ai y en enlaces compartidos, así que pida a Claude que use un título o icono específico si desea uno.

Para evitar que el navegador se abra automáticamente cuando se publica un nuevo artefacto, establezca `CLAUDE_CODE_ARTIFACT_AUTO_OPEN=0` en su entorno.

Si Claude responde que no puede publicar, o escribe un archivo HTML local sin un enlace, la herramienta no está habilitada para su sesión. Verifique los requisitos de [Disponibilidad](#availability).

<h2 id="update-an-artifact">
  Actualizar un artefacto
</h2>

Pida a Claude que revise la página, o deje que una tarea de larga duración se vuelva a publicar mientras hace progreso. Claude edita el archivo subyacente y publica nuevamente en la misma URL.

```text wrap theme={null}
Agregue un desglose por región debajo del gráfico de resumen y vuelva a publicar.
```

Cualquiera que tenga la página abierta ve la actualización en el lugar. Cada publicación se convierte en una versión, y desde el control **Compartir** en el encabezado de la página puede elegir qué versión ven los espectadores.

Para actualizar un artefacto desde una sesión diferente, proporcione a Claude la URL del artefacto y pida que lo revise. Sin la URL, una nueva sesión siempre crea un nuevo artefacto en lugar de actualizar uno existente.

```text wrap theme={null}
Actualice https://claude.ai/code/artifact/5fbea6f3-... con los números de hoy.
```

<h2 id="share-an-artifact">
  Compartir un artefacto
</h2>

Un nuevo artefacto es visible solo para usted. Para compartirlo, abra el artefacto en su navegador y use el control **Compartir** en el encabezado de la página. El encabezado lo identifica como el autor del artefacto, por lo que cualquier persona con la que lo comparta puede ver quién publicó la página. También vincula a su galería en [claude.ai/code/artifacts](https://claude.ai/code/artifacts), que enumera todos los artefactos que ha creado.

Con quién puede compartir depende de su plan:

* **Dentro de su organización**: en los planes Team y Enterprise, otorgue acceso a personas específicas en su organización, o a todos en ella. Los espectadores inician sesión en claude.ai como miembros de su organización para ver la página.
* **Públicamente**: comparta un enlace que cualquier persona en Internet pueda abrir, sin necesidad de iniciar sesión en claude.ai. En los planes Pro y Max, un enlace público es la única forma de compartir un artefacto. En los planes Team y Enterprise, el uso compartido público está desactivado hasta que un Propietario [lo habilite para la organización](#control-public-sharing).

<h3 id="let-someone-edit-with-you">
  Permitir que alguien edite con usted
</h3>

Las personas con las que comparte son espectadores de forma predeterminada: ven cada versión que publica pero no pueden cambiar la página. En los planes Team y Enterprise, también puede convertir a alguien en editor. En el diálogo de uso compartido, agregue una persona y cambie su rol de **espectador** a **editor**.

Un editor publica nuevas versiones de la misma manera que usted [actualiza el artefacto desde otra sesión](#update-an-artifact): le proporciona a Claude la URL del artefacto en su propia sesión, y Claude extrae el contenido actual y lo republica con sus cambios. Todos los que tengan la página abierta ven cada actualización en vivo.

<h2 id="pull-live-data-with-mcp-connectors">
  Extraer datos en vivo con conectores MCP
</h2>

Un artefacto puede llamar a [conectores MCP](/docs/es/mcp#use-mcp-servers-from-claude-ai) cada vez que alguien lo visualiza, de modo que la página muestre datos actuales en lugar de una instantánea de la sesión que la construyó. Las llamadas de conectores desde artefactos están disponibles en los planes Pro, Max, Team y Enterprise y requieren Claude Code v2.1.209 o posterior. En versiones anteriores, Claude publica la página con los datos que la sesión recopiló mientras la construía.

Para crear una página respaldada por un conector, nombre el conector y los datos que desea en su indicación:

```text wrap theme={null}
Build a dashboard artifact of our open pull requests that pulls the live list through my GitHub connector when the page loads.
```

Claude declara qué conectores puede llamar la página como parte de la publicación, y la página no puede llamar a conectores fuera de esa declaración. Solo califican los conectores de su cuenta claude.ai: Claude los nombra en la declaración, y cuando alguien visualiza la página, cada llamada [se ejecuta a través de la conexión propia de la cuenta que visualiza](#how-connector-calls-work-for-viewers) a ese conector. Los servidores MCP locales que configure en Claude Code, como servidores de `.mcp.json`, pueden proporcionar datos mientras Claude construye la página, pero la página publicada no puede llamarlos.

La página obtiene datos cuando se carga y puede actualizarse en un intervalo o cuando un visualizador utiliza un control de actualización en la página. Las respuestas se almacenan en caché en el navegador del visualizador, por lo que una página reabierta se renderiza desde las respuestas en caché inmediatamente, luego se actualiza con resultados frescos.

<h3 id="how-connector-calls-work-for-viewers">
  Cómo funcionan las llamadas de conectores para los visualizadores
</h3>

Cuando una página publicada llama a un conector, la llamada utiliza la cuenta de la persona que visualiza la página, no la cuenta de la persona que la publicó:

* **Cada visualizador utiliza sus propios conectores**: las llamadas se ejecutan a través de las herramientas conectadas de la cuenta que visualiza, por lo que dos personas que abran el mismo panel pueden ver datos diferentes según lo que sus cuentas puedan acceder. La página nunca ve las credenciales de nadie; claude.ai realiza las llamadas en nombre de la página.
* **Los visualizadores aprueban el acceso primero**: claude.ai solicita permiso a cada visualizador antes de la primera llamada de conector de la página. Un visualizador que rechace, o que no haya conectado un conector que la página utiliza, aún ve la página sin sus secciones en vivo.
* **Las acciones también utilizan la cuenta del visualizador**: una página puede ofrecer controles que invoquen herramientas de conectores con efectos secundarios, como publicar un mensaje o actualizar un problema. La acción se ejecuta a través de la cuenta de quien selecciona el control.

Cuando planee compartir una página respaldada por un conector, pida a Claude que incluya un mensaje de respaldo en cada sección en vivo que nombre el conector que necesita. Un visualizador que no tenga la conexión entonces ve qué conectar en lugar de una sección vacía.

Un artefacto que llama a conectores no se puede compartir a un enlace público en ningún plan. En los planes Team y Enterprise, puede mantenerlo privado o [compartirlo dentro de su organización](#share-an-artifact). En los planes Pro y Max, donde un enlace público es la única forma de compartir, un artefacto respaldado por un conector permanece privado para usted.

<h3 id="the-page-shows-no-live-data-for-a-viewer">
  La página no muestra datos en vivo para un visualizador
</h3>

Cuando una página respaldada por un conector se renderiza pero sus secciones en vivo permanecen vacías para alguien con quien la compartió, trabaje a través de estas causas:

* **El visualizador no ha conectado el conector**: los conectores son por cuenta, por lo que cada visualizador necesita su propia conexión a cada conector que la página llama. Pueden agregar uno en **Configuración > Conectores** en claude.ai, luego recargar la página.
* **El visualizador rechazó la solicitud de permiso**: un rechazo dura el resto de esa carga de página. Recargar la página trae de vuelta la solicitud de permiso.
* **Las llamadas de conectores están desactivadas para la organización**: un Propietario controla el [alternar **Habilitar conectores de artefactos**](#control-connector-calls-from-artifacts) en la configuración de administración.

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

A partir de Claude Code v2.1.183, Claude aplica una skill de diseño integrada cuando construye un artefacto, por lo que las páginas obtienen una paleta deliberada, tipografía y diseño sin indicaciones adicionales. Esa skill también busca un sistema de diseño existente en su proyecto antes de elegir el suyo. Para mantener los artefactos consistentes con la marca de su producto, registre sus tokens de diseño donde Claude pueda encontrarlos, como el [CLAUDE.md](/docs/es/memory) del proyecto o un archivo de tema en su repositorio:

```markdown theme={null}
## Design system

- Colors: primary #1a4d8f, accent #f59e0b, surface #f8fafc
- Typography: Inter for body, JetBrains Mono for code
- Spacing: 8px scale, 6px border radius
```

Claude trata su sistema de diseño como una precedencia más alta que sus propias opciones, y su indicación como una precedencia más alta que ambas. El encabezado y el formato anterior son un ejemplo; cualquier lista clara de colores, fuentes y espaciado funciona.

<h2 id="page-constraints">
  Restricciones de página
</h2>

Cada artefacto es una página única y autónoma. Claude Code envuelve el archivo que publica en un shell de documento HTML y lo sirve bajo una Política de Seguridad de Contenido (CSP) estricta, que da forma a lo que la página puede hacer.

| Restricción                | Efecto                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| :------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Sin solicitudes externas   | El CSP bloquea scripts, hojas de estilo, fuentes e imágenes cargadas desde cualquier otro host, junto con llamadas `fetch`, XHR y WebSocket. Claude inserta CSS y JavaScript e incrusta imágenes como URIs de datos para que la página se represente sin ninguna solicitud externa. [Las llamadas de Connector](#pull-live-data-with-mcp-connectors) son la excepción: la página las entrega a claude.ai, que realiza la llamada de red por sí misma. |
| Sin backend                | Un artefacto es una página estática. No puede almacenar datos enviados a través de un formulario ni autenticar espectadores por sí mismo. Su única forma de obtener datos cuando alguien lo visualiza es [llamando a conectores MCP](#pull-live-data-with-mcp-connectors), no una API propia.                                                                                                                                                         |
| Página única               | Los enlaces relativos no se resuelven, porque nada se implementa junto a la página. Para contenido de múltiples secciones, Claude usa anclajes en la página en lugar de archivos separados.                                                                                                                                                                                                                                                           |
| Tipos de archivo de origen | El archivo publicado debe ser `.html`, `.htm` o `.md`. Los archivos Markdown se representan como HTML con estilo.                                                                                                                                                                                                                                                                                                                                     |
| Tamaño representado        | La página representada debe ser de 16 MiB o menor. Las imágenes incrustadas grandes son la causa habitual cuando una publicación falla por tamaño.                                                                                                                                                                                                                                                                                                    |

Generar un artefacto usa tokens de salida como cualquier otra respuesta, y una página con estilo es más intensiva en tokens que el mismo contenido como texto de terminal. CSS en línea, JavaScript para controles interactivos e imágenes incrustadas como URIs de datos son los principales contribuyentes. Para reducir el costo de tokens de un artefacto:

* Prefiera SVG, o HTML y CSS, para diagramas sobre imágenes rasterizadas incrustadas
* Omita la interactividad que no necesita
* Haga que la página resuma grandes conjuntos de datos en lugar de incrustarlos en su totalidad

<h2 id="availability">
  Disponibilidad
</h2>

Los artefactos requieren todas las condiciones a continuación. Cuando una no se cumple, Claude escribe un archivo HTML local o dice que no puede publicar.

| Requisito                | Disponible cuando                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| :----------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Plan                     | Pro, Max, Team o Enterprise. En planes Pro y Max, los artefactos son privados para usted hasta que los comparta, y no se aplica ninguna gestión de administrador. En planes de Team, los artefactos están habilitados de forma predeterminada. En planes de Enterprise, un propietario [los habilita](#manage-artifacts-for-your-organization) en la configuración de administrador de claude.ai.                                                                                                                                 |
| Autenticación            | La sesión está respaldada por una cuenta de claude.ai: inicie sesión con `/login` en la CLI o la aplicación de escritorio. Las sesiones de Claude Tag están autenticadas a través de la identidad del agente, por lo que no se requiere ningún paso. Las sesiones que usan una clave API, [token de puerta de enlace](/docs/es/llm-gateway) o credencial de proveedor de nube no pueden publicar.                                                                                                                                      |
| Proveedor de modelo      | API de Anthropic. No disponible en [Amazon Bedrock](/docs/es/amazon-bedrock), [Google Cloud's Agent Platform](/docs/es/google-vertex-ai) o [Microsoft Foundry](/docs/es/microsoft-foundry).                                                                                                                                                                                                                                                                                                                                                      |
| Política de organización | Las claves de cifrado administradas por el cliente (CMEK), HIPAA y [Retención de datos cero](/docs/es/zero-data-retention) no están habilitadas para la organización.                                                                                                                                                                                                                                                                                                                                                                  |
| Superficie               | CLI de Claude Code versión 2.1.183 o posterior, o la aplicación de escritorio Claude versión 1.13576.0 o posterior. Las sesiones de [Claude Tag](https://claude.com/docs/claude-tag/overview) también pueden publicar artefactos cuando tanto Claude Tag como los artefactos están habilitados para la organización. Deshabilitado de forma predeterminada en contextos de [Agent SDK](/docs/es/agent-sdk/overview), GitHub Action y MCP-server, y cuando [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/es/env-vars) está establecido. |

<h2 id="disable-artifacts">
  Deshabilitar artefactos
</h2>

Para desactivar los artefactos en sus propias sesiones independientemente de la configuración de su organización, use cualquiera de:

| Método                                   | Configuración                           |
| :--------------------------------------- | :-------------------------------------- |
| [Archivo de configuración](/docs/es/settings) | `"disableArtifact": true`               |
| [Variable de entorno](/docs/es/env-vars)      | `CLAUDE_CODE_DISABLE_ARTIFACT=1`        |
| [Regla de permiso](/docs/es/permissions)      | Agregue `Artifact` a `permissions.deny` |

<h2 id="manage-artifacts-for-your-organization">
  Gestionar artefactos para su organización
</h2>

Los administradores en planes de Team y Enterprise controlan los artefactos desde [la configuración de administrador de claude.ai](https://claude.ai/admin-settings/claude-code). El contenido del artefacto se almacena en la infraestructura operada por Anthropic y es visible solo para miembros autenticados de la organización publicadora, a menos que el artefacto sea [compartido públicamente](#control-public-sharing).

<h3 id="enable-or-disable-artifacts">
  Habilitar o deshabilitar artefactos
</h3>

Para habilitar o deshabilitar artefactos para toda la organización, vaya a **Configuración > Claude Code > Capacidades** y use el botón de alternancia **Artefactos**. En planes de Enterprise con control de acceso basado en roles, también puede limitar los artefactos a roles específicos: vaya a **Configuración > Roles**, edite un rol y establezca el permiso **Artefactos** bajo el grupo **Claude Code**.

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

Para establecer cuánto tiempo se conservan los artefactos antes de la eliminación automática, vaya a **Configuración > Controles de datos y privacidad**. Puede establecer períodos de retención separados para artefactos que aún son privados para su autor y artefactos que han sido compartidos.

<h3 id="review-the-audit-log">
  Revisar el registro de auditoría
</h3>

Publicar, compartir y eliminar un artefacto aparecen en el registro de auditoría de su organización bajo los tipos de evento `claude_artifact_*`, la misma familia utilizada para artefactos creados en conversaciones de claude.ai.

<h3 id="allowlist-the-viewer-domain">
  Permitir el dominio del visor
</h3>

El visor en claude.ai carga cada artefacto desde un origen `*.claudeusercontent.com` aislado. Si su organización restringe el acceso a la red saliente, agregue ese dominio a su lista de permitidos junto con `claude.ai`. Consulte [Requisitos de acceso a la red](/docs/es/network-config#network-access-requirements) para la lista completa.

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
