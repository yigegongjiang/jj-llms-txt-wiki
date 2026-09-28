> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Deje que Claude coordine el trabajo en curso con Projects

> Proporcione a Claude un conjunto de trabajo relacionado en una conversación y deje que coordine sesiones en la nube paralelas que compartan repositorios, instrucciones y memoria.

<Note>
  Projects está en versión beta pública en los planes Pro y Max y se está implementando gradualmente, comenzando con cuentas que han utilizado [sesiones en la nube](/docs/es/claude-code-on-the-web) y no tienen proyectos existentes en el chat de claude.ai o en Cowork. Aún no está disponible en los planes Team o Enterprise. Si **Projects** no aparece en la barra lateral en [claude.ai/code](https://claude.ai/code) o en la pestaña Code de la [aplicación de escritorio](/docs/es/desktop), el despliegue aún no ha llegado a su cuenta, y puede [unirse a la lista de espera](https://claude.com/form/projects). [Ejecutar agentes en paralelo](/docs/es/agents) enumera lo que puede usar mientras tanto.
</Note>

Un proyecto es una conversación en curso donde Claude coordina un flujo de trabajo relacionado para usted. Usted le dice qué necesita hacerse y comienza un hilo para cada tarea.

Cada hilo es generalmente una [sesión en la nube](/docs/es/claude-code-on-the-web): Claude Code ejecutándose en la nube en lugar de en su máquina. Cuando una tarea necesita algo que solo su computadora tiene, puede pedirle a Claude que ejecute ese hilo en su computadora en su lugar a través de [Remote Control](/docs/es/remote-control). Los hilos se ejecutan en paralelo y puede verificarlos y dirigirlos desde su teléfono. Los hilos en la nube continúan después de cerrar la computadora portátil.

Sin un proyecto, ejecutar varias sesiones significa hacer la coordinación usted mismo: decide en qué trabaja cada una, repite el mismo contexto al inicio de cada una, y verifica cuál terminó o necesita una respuesta. Con un proyecto, en su lugar:

* **Envíe trabajo a un solo lugar**: pegue un informe de error, un seguimiento de pila o una lista de tareas en la conversación cada vez que surja uno. Claude comienza un hilo para cada pieza de trabajo o lo pasa al hilo que ya está trabajando en esa área, y responde preguntas rápidas en su lugar.
* **Establezca el contexto una sola vez**: cada nuevo hilo comienza con las instrucciones del proyecto, por lo que una regla que establezca una sola vez, como qué rama dirigirse, llega a todos ellos.
* **Aléjese y regrese al trabajo terminado**: cuando regrese una hora después o a la mañana siguiente, el panel **Overview** muestra qué hilos terminaron, qué solicitudes de extracción están listas para revisión y qué hilo está esperando su respuesta.

Si ya sabe el trabajo que desea que un proyecto ejecute, vaya directamente a [Crear un proyecto](#create-a-project).

<h2 id="when-to-use-a-project">
  Cuándo usar un proyecto
</h2>

Vale la pena crear un proyecto cuando el trabajo tiene un objetivo que perdura más allá de una sesión y sigue produciendo tareas. Estos tipos de trabajo se adaptan bien a un proyecto:

* **Un objetivo en muchos repositorios**: "Llevar cada servicio a la nueva configuración de lint." Claude puede ejecutar un hilo por repositorio, cada uno con su propia solicitud de extracción, y el panel [**Overview**](#see-what-needs-you-in-overview) muestra cuáles están listas para revisión.
* **Un área que sigues alimentando**: los errores, seguimientos de pila y solicitudes de revisión para un servicio, pegados en la conversación a medida que te llegan. Una trampa que le dices a Claude que recuerde después de una corrección está en [memoria del proyecto](#give-a-project-standing-context) para la siguiente.
* **Una compilación o migración más grande que una sesión**: "Construir lo que describe `docs/spec.md`" o "Mover la aplicación del ORM obsoleto." El trabajo se divide en hilos que cada uno toma una parte, las decisiones que le pides a Claude que recuerde al principio llegan a los hilos posteriores, y la especificación cambia y los errores que encuentras durante la compilación van a la misma conversación.
* **Trabajo que no es código**: una carpeta de contratos o una exportación de tickets de soporte a la que sigues regresando con nuevas preguntas, como "encontrar los diez errores de integración más comunes en estos tickets." Carga los documentos en lugar de agregar un repositorio, y los hilos entregan cada informe como un archivo en la pestaña [**Library**](#see-what-needs-you-in-overview) del proyecto.

En cualquiera de ellos puedes enviar un lote de tareas, decirle a Claude que comience sin pedirte que confirmes, alejarte, y encontrar los hilos que te necesitan bajo [**Waiting on you**](#see-what-needs-you-in-overview) cuando regreses, o pedirle a Claude que ponga parte del trabajo en un horario como una [rutina](/docs/es/routines). Si esta es tu situación, [crea un proyecto](#create-a-project).

<h3 id="when-something-else-fits-better">
  Cuándo algo más se adapta mejor
</h3>

Los hilos funcionan en repositorios de GitHub y en los archivos, carpetas y carpetas de Google Drive que cargues en el proyecto, no en archivos o herramientas que existan solo en tu máquina. Si una tarea necesita tu máquina, pídele a Claude que ejecute su hilo allí a través de [Remote Control](/docs/es/remote-control). [Limitaciones](#limitations) enumera lo que eso necesita. Algo más se adapta mejor en estos casos:

* **Una tarea que cabe en una sesión**: "Arreglar la prueba de inicio de sesión inestable." Comienza una [sesión en la nube](/docs/es/claude-code-on-the-web) tú mismo.
* **Trabajo donde cada tarea necesita tu máquina**: una base de datos local, un emulador de dispositivo, o una API detrás de tu VPN. Usa una sesión local, o [vista de agente](/docs/es/agent-view) para ejecutar varias a la vez. Si el trabajo solo necesita archivos locales, cárgalos en el proyecto en su lugar.
* **Una tarea que se repite en un horario sin conversación alrededor**: "Publicar un informe de dependencias cada lunes." Crea una [rutina](/docs/es/routines) por su cuenta.
* **Varias personas dándole trabajo a Claude y dirigiéndolo juntos en un canal de Slack**: ver [Claude Tag](https://claude.com/docs/claude-tag/overview).

Un proyecto utiliza los mismos límites de plan que tus otras sesiones de Claude Code y los utiliza más rápido. [Uso y costo](#usage-and-cost) cubre qué utiliza tu plan y cómo mantenerlo bajo.

<h2 id="how-a-project-is-organized">
  Cómo se organiza un proyecto
</h2>

Un proyecto es una conversación coordinadora con Claude más los hilos que comienza para hacer el trabajo. Estas son sus partes:

* **La conversación del proyecto**: una sesión de larga duración donde Claude actúa como coordinador. Toma lo que envías, decide qué se convierte en un hilo, y mantiene un registro de cada hilo que comenzó. Ve lo que los hilos reportan, no cada paso que toman.
* **Hilos**: los trabajadores. Cada uno es una sesión separada con su propia ventana de contexto que hace una pieza de trabajo e informa a la conversación cuando termina. Un hilo en la nube trabaja en su propia rama y abre una solicitud de extracción cuando el trabajo lo requiere.
* **Lo que cada hilo en la nube comienza con**:
  * Los repositorios y archivos del proyecto, más sus [instrucciones y memoria](#give-a-project-standing-context)
  * El `CLAUDE.md` y skills en [cada uno de los repositorios del proyecto](#what-threads-pick-up-from-your-repositories), y en un proyecto con un repositorio, las reglas de permisos y hooks de ese repositorio también
  * Los [conectores](#get-skills-plugins-connectors-and-tools-into-threads) en tu cuenta de claude.ai
  * Un [entorno en la nube](#choose-an-environment-for-threads) que establece su acceso a la red, variables de entorno, credenciales de API, y herramientas instaladas
* **El panel Overview**: donde [ves todos los hilos a la vez](#see-what-needs-you-in-overview) y cuáles de ellos te necesitan. Sus otras pestañas son **Library** para los archivos que agregaste y los archivos que produjeron los hilos, **Pull requests** para los que abrieron los hilos, y **Routines** para el trabajo programado en el proyecto.

Los hilos en la nube no recogen nada de la configuración de Claude Code en tu propia máquina. [Obtener skills, plugins, conectores y herramientas en hilos](#get-skills-plugins-connectors-and-tools-into-threads) cubre cómo darles lo que de otro modo les faltaría.

Así es como esas partes se conectan, desde ti a través de la conversación hasta los hilos haciendo el trabajo, con **Overview** rastreando su estado:

<Frame>
  <img src="https://mintcdn.com/claude-code/e8CLbxM17eD7cAiv/images/claude-projects-overview.svg?fit=max&auto=format&n=e8CLbxM17eD7cAiv&q=85&s=dbf446f69f0bbdb9961d21af207cb93b" className="dark:hidden" alt="Diagrama de un proyecto. Escribes en la conversación del proyecto, donde Claude responde o comienza un hilo. Cada hilo en la nube trabaja en su propia rama y solicitud de extracción. El panel Overview enumera hilos por estado, como listo para revisión, esperando por ti, y trabajando." width="600" height="250" data-path="images/claude-projects-overview.svg" />

  <img src="https://mintcdn.com/claude-code/e8CLbxM17eD7cAiv/images/claude-projects-overview-dark.svg?fit=max&auto=format&n=e8CLbxM17eD7cAiv&q=85&s=549a5ba9fea8433729babc37a1f6e9c8" className="hidden dark:block" alt="Diagrama de un proyecto. Escribes en la conversación del proyecto, donde Claude responde o comienza un hilo. Cada hilo en la nube trabaja en su propia rama y solicitud de extracción. El panel Overview enumera hilos por estado, como listo para revisión, esperando por ti, y trabajando." width="600" height="250" data-path="images/claude-projects-overview-dark.svg" />
</Frame>

<h2 id="create-a-project">
  Crear un proyecto
</h2>

Creas y usas proyectos en [claude.ai/code](https://claude.ai/code), en la pestaña Code de la aplicación de escritorio, o en la aplicación móvil de Claude para [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) y [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude). En el navegador y la aplicación de escritorio hay dos formas de comenzar un proyecto:

* **Desde cero**, cuando sabes el flujo de trabajo que deseas que ejecute Claude: abre el diálogo **New project** y nómbralo. [Comienza un nuevo proyecto desde cero](#start-a-new-project-from-scratch) te guía a través del diálogo.
* **Desde una sesión en la nube que ya está haciendo el trabajo**: elige **Continue as a project** del menú de esa sesión, y Claude propone la configuración del proyecto a partir de lo que la sesión estaba haciendo. Ver [Comenzar desde una sesión en la nube existente](#start-from-an-existing-cloud-session).

De cualquier forma, [verifica los requisitos previos](#check-the-prerequisites) primero.

<h3 id="check-the-prerequisites">
  Verifica los requisitos previos
</h3>

Antes de crear un proyecto, verifica tu plan, tu configuración de GitHub, y a qué necesita llegar el trabajo:

* **Plan**: estás en Pro o Max y **Projects** aparece en tu barra lateral.
* **GitHub, si el proyecto trabajará en código**: tu código está en github.com en lugar de GitHub Enterprise Server, GitLab o Bitbucket, tu cuenta de GitHub conectada tiene acceso de inserción a él, y la Claude GitHub App está instalada en él. Si conectaste GitHub con [`/web-setup`](/docs/es/web-quickstart#connect-from-your-terminal), ese token permite que tus otras sesiones en la nube accedan a un repositorio pero no es suficiente para hilos de proyecto, que necesitan la Claude GitHub App. [Configurar acceso a GitHub](#set-up-github-access) tiene los pasos.
* **Red, credenciales y herramientas**: para hilos en la nube, estos provienen del [entorno en la nube](#choose-an-environment-for-threads) del proyecto. El entorno predeterminado ya alcanza [registros de paquetes comunes](/docs/es/cloud-environments#default-allowed-domains), así que verifica esto solo si el trabajo necesita otros dominios, un secreto, o una herramienta que no esté preinstalada. Si el trabajo necesita un servidor MCP, verifica que aparezca como conectado en tus [conectores de claude.ai](https://claude.ai/customize/connectors).

<h3 id="start-a-new-project-from-scratch">
  Comienza un nuevo proyecto desde cero
</h3>

Comenzar un proyecto desde cero significa abrir el diálogo **New project**, nombrar el flujo de trabajo, y opcionalmente darle un objetivo y los repositorios y archivos en los que trabaja. Solo el nombre es requerido, así que puedes crear el proyecto primero y llenar el resto a medida que el trabajo toma forma.

<Steps>
  <Step title="Abre Projects">
    En [claude.ai/code](https://claude.ai/code) o en la pestaña Code de la aplicación de escritorio, selecciona **Projects** en la barra lateral izquierda, luego selecciona **New project**. En un navegador también puedes ir directamente a [claude.ai/code/projects/browse](https://claude.ai/code/projects/browse).
  </Step>

  <Step title="Completa el diálogo New project">
    Limita el proyecto a un flujo de trabajo que seguirás agregando, como todo lo que se necesita para mantener una API bajo su objetivo de latencia. [Cuándo usar un proyecto](#when-to-use-a-project) tiene más ejemplos. Luego completa los campos del diálogo:

    * **Name**: cómo aparece el proyecto en la lista **Projects**.
    * **Goal** (opcional): una línea de lo que estás tratando de lograr, como "Mantener la latencia p95 de la API por debajo de 200 ms". Claude en la conversación trabaja hacia él. Sin un objetivo, Claude trabaja a partir de las tareas que envías, y puedes agregar un objetivo más tarde en **Project settings > General**.
    * **Context** (opcional): los repositorios de GitHub en los que trabaja este proyecto, más cualquier archivo, carpeta o carpeta de Google Drive que los hilos deban leer. Haz clic en **Add** para cada uno. Agrega los repositorios que la mayoría de las tareas necesitan en lugar de cada uno que el trabajo podría tocar; [Decide qué repositorios agregar](#decide-which-repositories-to-add) cubre la elección, y puedes agregar más tarde en **Project settings > Environment**.

    Las reglas permanentes sobre cómo deben trabajar los hilos van en [instrucciones del proyecto](#give-a-project-standing-context), que estableces después de que el proyecto exista.
  </Step>

  <Step title="Crea el proyecto">
    Haz clic en **Create project**. La conversación del proyecto se abre con un cuadro de mensaje en la parte inferior, donde describes trabajo para Claude.

    En tu primer proyecto, Claude toma un turno por su cuenta tan pronto como se crea el proyecto, a menos que envíes un mensaje primero. Ese turno utiliza tu plan. En él, Claude puede:

    * Comenzar un hilo que explora el repositorio sin cambiar nada y propone los próximos pasos, si el proyecto tiene un repositorio que puede leer.
    * Publicar **Setup recommendations** extraídas de tus sesiones en la nube recientes: repositorios para agregar, rutinas para crear, e hilos que podría comenzar. Cada repositorio y rutina recomendados comienzan activados. Desactiva los que no deseas, luego haz clic en **Update setup** para agregar el resto, o ignora las recomendaciones y describe el trabajo tú mismo.
  </Step>
</Steps>

El proyecto ahora está listado bajo **Projects** en la barra lateral, y su conversación está abierta. [Tu primer lote](#your-first-batch) cubre qué configurar antes de enviarle trabajo.

<h3 id="start-from-an-existing-cloud-session">
  Comienza desde una sesión en la nube existente
</h3>

Si ya tienes una sesión en la nube haciendo trabajo que pertenece a un proyecto, abre el menú de la sesión en la barra lateral y elige **Continue as a project** o **Move to project**:

* **Continue as a project** crea un nuevo proyecto nombrado después de la sesión y lo abre. Claude lee la sesión y publica **Setup recommendations** en la conversación para que confirmes. La sesión original permanece en tu lista de sesiones, y si estaba en medio de un turno sigue ejecutándose, así que detenla tú mismo si no deseas que ambas trabajen a la vez. Si usas el banner **Set up project** que puede aparecer arriba del cuadro de mensaje de la sesión en la nube, el resultado es el mismo, excepto que el turno en ejecución de la sesión se detiene una vez que se abre el proyecto.
* **Move to project** trae el trabajo de la sesión a un proyecto existente. Publica un mensaje en la conversación de ese proyecto pidiéndole a Claude que lea la sesión y continúe desde donde se quedó, y el nuevo trabajo continúa en los propios hilos del proyecto. La sesión original permanece en tu lista de sesiones, sin cambios.

<h3 id="set-up-github-access">
  Configurar acceso a GitHub
</h3>

La mayoría de la configuración de GitHub sucede una vez, no por proyecto. Conectas tu cuenta de GitHub a Claude una vez, y la Claude GitHub App se instala una vez por repositorio, o una vez para toda una organización de GitHub si le das todos los repositorios. Vuelves a estos pasos cuando agregas un repositorio que la Claude GitHub App aún no cubre o uno en una organización de GitHub que aplica SSO.

<Steps>
  <Step title="Conecta tu cuenta de GitHub">
    Si no has usado claude.ai/code antes, tu primera visita te guía a través de conectar GitHub; ver [Conectar GitHub](/docs/es/web-quickstart#connect-github). De lo contrario, usa una de las [opciones de autenticación de GitHub](/docs/es/claude-code-on-the-web#github-authentication-options).
  </Step>

  <Step title="Instala la Claude GitHub App en los repositorios del proyecto">
    Instala la [Claude GitHub App](https://github.com/apps/claude) y otórgale los repositorios que el proyecto usará. En un repositorio propiedad de una organización de GitHub, solo un propietario de la organización puede completar la instalación; si no eres uno, GitHub envía al propietario una solicitud de instalación y el proyecto no puede usar el repositorio hasta que lo apruebe.
  </Step>

  <Step title="Autoriza SSO para organizaciones que lo aplican">
    Si una organización de GitHub aplica SAML SSO, reconecta GitHub y autoriza la aplicación Claude para esa organización. Hasta que lo hagas, los repositorios privados de esa organización no aparecen en el diálogo **New project** o **Project settings > Environment**.
  </Step>
</Steps>

Cuando uno de estos pasos está incompleto, el diálogo **New project** y la página del proyecto nombran el paso faltante y enlazan a donde lo terminas. Termina el paso allí, luego haz clic en **Check again** si el diálogo lo ofrece. Si un repositorio aún falta de la lista después, abre la instalación de la Claude GitHub App en GitHub, en [github.com/settings/installations](https://github.com/settings/installations) para una cuenta personal, y confirma que el repositorio está listado bajo **Repository access**. Para los mensajes de error que un hilo o el proyecto reportan cuando el acceso aún es incorrecto, ver [Errores de acceso al repositorio](#repository-access-errors).

<h2 id="work-in-a-project">
  Trabajar en un proyecto
</h2>

Dale trabajo a Claude a través de la conversación del proyecto: tareas una a la vez o varias a la vez, más actualizaciones y pensamientos sueltos conforme surjan. Claude enruta cada mensaje, y los hilos hacen el trabajo e informan.

<h3 id="your-first-batch">
  Su primer lote
</h3>

Antes de enviar a un nuevo proyecto un lote de trabajo, configúrelo para que los primeros hilos regresen de la manera que desea:

1. [Escribir instrucciones del proyecto](#write-project-instructions): el resumen que cada hilo comienza, como qué rama dirigirse, cómo un hilo verifica su trabajo, y qué necesita su aprobación.
2. Envíe una pequeña pieza del trabajo real, o inicie uno de los hilos que Claude sugirió si ofreció alguno, y abra el hilo cuando termine para ver cómo informa y qué hizo en su rama. Si asumió algo incorrecto o no pudo alcanzar lo que necesitaba, [Los hilos adivinaron o se estancaron en lugar de preguntar](#threads-guessed-or-stalled-instead-of-asking) cubre dónde arreglarlo.
3. Verifique **Thread model** y **Thread effort** en **Project settings > General**. Un nuevo proyecto ejecuta cada hilo en Opus con esfuerzo alto, que consume su plan más rápidamente; [Elegir modelos y dejar que Claude gestione el contexto](#choose-models-and-let-claude-manage-context) cubre las alternativas.
4. Pida a Claude que [proponga hilos antes de iniciarlos y ejecute algunos a la vez](#tune-how-claude-runs-a-project), y elimine esos límites una vez que algunos hilos regresen de la manera que desea.

<h3 id="send-work-and-read-results">
  Enviar trabajo y leer resultados
</h3>

Claude decide dónde va cada mensaje que envía en la conversación:

* Una pregunta rápida generalmente obtiene una respuesta en la conversación.
* El trabajo nuevo va a un hilo nuevo o a un hilo que ya está trabajando en esa área, y Claude le dice cuál. Cada hilo nuevo aparece bajo su mensaje como una tarjeta: una caja con el título y estado del hilo, que hace clic para abrir el hilo.
* Varias tareas no relacionadas en un mensaje se convierten en hilos separados.

Si Claude enruta algo diferente a lo que deseaba, dígalo. [Ajustar cómo Claude ejecuta un proyecto](#tune-how-claude-runs-a-project) enumera cosas que puede decirle, como reutilizar un hilo existente para seguimientos o responder en su lugar en lugar de iniciar un hilo.

Los resultados completos de un hilo permanecen en el hilo, y abre su tarjeta en la conversación para leerlos. Los archivos que produjo un hilo también están en la pestaña **Library** en **Overview**.

A veces Claude propone hilos en lugar de iniciarlos, en una lista de **Suggested threads**. Haga clic en la flecha en una sugerencia para iniciar ese hilo. Cuando se enumeran varios, un botón debajo de la lista inicia todos ellos.

<h3 id="review-a-thread’s-pull-request">
  Revisar la solicitud de extracción de un hilo
</h3>

Cuando un hilo en la nube cambia código, esto es lo que hace a menos que le diga lo contrario:

* **Branch**: trabaja en una rama nueva, iniciada desde la rama predeterminada del repositorio.
* **Pull request**: abre uno cuando lo solicita, y puede abrir uno por su cuenta para una corrección de errores u otro cambio concreto.
* **After it opens**: vigila la solicitud de extracción con [auto-fix](/docs/es/claude-code-on-the-web#auto-fix-pull-requests) activado, independientemente de si auto-fix está activado para sus otras sesiones en la nube. Envía correcciones cuando CI falla, aborda comentarios de revisión, e informa en el hilo cuando las comprobaciones pasan y la solicitud de extracción está lista para usted.

Cuando un hilo ha enviado una rama o abierto una solicitud de extracción, su tarjeta en la conversación puede mostrar un botón para el siguiente paso:

* **Resolve conflicts**, **Fix CI**, **Address comments**, y **Merge it** envían esa instrucción al hilo como un mensaje de usted, para que pueda indicar al hilo usted mismo en lugar de esperar a que reaccione a la solicitud de extracción.
* **Review PR** abre la solicitud de extracción en GitHub.
* **Create PR** aparece cuando un hilo inactivo ha enviado una rama pero no ha abierto una solicitud de extracción. Al hacer clic, se crea la solicitud de extracción desde esa rama directamente en lugar de enviar al hilo una instrucción para abrir una.

Para cambiar cuándo los hilos abren solicitudes de extracción, por ejemplo solo cuando lo solicita, o de qué rama comienzan, dígalo en la tarea o en [instrucciones del proyecto](#write-project-instructions).

<h3 id="see-what-needs-you-in-overview">
  Ver qué lo necesita en Overview
</h3>

El panel **Overview** junto a la conversación rastrea los hilos del proyecto. Ya está abierto la primera vez que abre un nuevo proyecto. El botón **Overview** en el encabezado del proyecto lo cierra y reabre, y muestra un punto cuando un hilo lo está esperando.

En la aplicación de escritorio, también obtiene una notificación de escritorio cuando Claude publica en la conversación, un hilo alcanza un error, o un hilo necesita su entrada, para que no tenga que mantener el proyecto abierto para enterarse. Para obtener también uno cada vez que un hilo termina un turno, o para desactivarlos para un proyecto, elija **Notifications** en el menú de la barra lateral del proyecto. Estas notificaciones son solo de escritorio: en un navegador, verifique el punto en el botón **Overview**.

La pestaña **Threads** del panel agrupa hilos por estado:

| Grupo                | Qué contiene                                                                                                                                                                                                                                               |
| :------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Ready for review** | Hilos cuya solicitud de extracción está abierta y esperando revisión                                                                                                                                                                                       |
| **Waiting on you**   | Hilos que necesitan su respuesta o aprobación, o que fallaron                                                                                                                                                                                              |
| **Working**          | Hilos aún en ejecución                                                                                                                                                                                                                                     |
| **Landing**          | Hilos cuya solicitud de extracción está aprobada o en cola para fusionar                                                                                                                                                                                   |
| **Idle**             | Hilos que terminaron y no están esperando nada                                                                                                                                                                                                             |
| **Resolved**         | Hilos marcados como completados: por usted desde el menú del hilo, por Claude una vez que ha tomado el último paso, como fusionar su solicitud de extracción, o automáticamente después de una semana sin actividad. Puede reabrir uno desde el mismo menú |

Las otras pestañas del panel son **Library** para los archivos y carpetas que agregó y los archivos que produjeron los hilos, **Pull requests** una vez que los hilos han abierto alguno, y **Routines** para las [rutinas](/docs/es/routines) que Claude configuró desde este proyecto.

<h3 id="open-a-thread-when-you-need-control">
  Abrir un hilo cuando necesita control
</h3>

Haga clic en la tarjeta de un hilo en la conversación o su fila en **Overview** para abrir su transcripción en el panel Overview. Desde allí puede:

* Leer qué hizo Claude, paso a paso.
* Dirigir la tarea escribiendo en el cuadro de mensaje del hilo. Un mensaje allí va directamente a ese hilo, mientras que un seguimiento en la conversación del proyecto lo alcanza solo cuando Claude coincide el seguimiento con ese hilo.
* Responder a un aviso de permiso que el hilo está esperando.
* Interrumpir el hilo con **Stop**, que reemplaza el botón enviar mientras el hilo está trabajando, o presionando Esc.

<h3 id="choose-models-and-let-claude-manage-context">
  Elegir modelos y dejar que Claude gestione el contexto
</h3>

Establezca modelos y esfuerzo en **Project settings > General**. Un nuevo proyecto ejecuta Opus en todas partes, con [esfuerzo](/docs/es/model-config#adjust-effort-level) alto para hilos y esfuerzo bajo para la conversación:

* **Thread model** y **Thread effort** se aplican a los hilos. Para usar un modelo diferente para una tarea, solicítelo en la tarea; para un hilo ya en ejecución, use el selector de modelo de ese hilo.
* **Coordinator model** y **Coordinator effort** se aplican a Claude en la conversación del proyecto.

No gestiona ventanas de contexto en un proyecto. Los hilos se compactan automáticamente, y la conversación funciona a partir de mensajes recientes, hilos recientes, y memoria del proyecto en lugar de su historial completo, para que continúe mientras el proyecto se ejecute. Ponga cualquier cosa que nunca deba descartarse en [memoria del proyecto](#give-a-project-standing-context). Si un hilo crece más que su contexto, muestra [Claude ran out of context on this turn](#context-limit).

<h3 id="tune-how-claude-runs-a-project">
  Ajustar cómo Claude ejecuta un proyecto
</h3>

Dígale a Claude en la conversación cuántos hilos ejecutar a la vez, cuándo publicar actualizaciones, y cuándo abrir solicitudes de extracción. Si Claude está coordinando de una manera que no desea, dígalo. Por ejemplo, puede decir:

* "Propose threads and wait for my go-ahead before starting them" o "Start these now without asking me to confirm"
* "Run at most two threads at a time" o "Reuse an existing thread for follow-ups in the same area"
* "Post shorter updates" o "Only post when something finishes or is blocked"
* "Give me a status update on every thread"
* "Do this task with a smaller model"
* "Don't open a pull request until I've seen the plan"
* "Tell me what's wrong in these repositories and don't fix anything yet", cuando desea revisar los hallazgos antes de que alguno se convierta en un hilo
* "Answer that here instead of starting a thread", cuando Claude inicia un hilo para algo que significaba como una pregunta rápida

Claude guarda preferencias como estas en [memoria del proyecto](#give-a-project-standing-context) por su cuenta y las sigue en hilos posteriores. Son instrucciones que Claude mantiene, no configuraciones aplicadas, por lo que un límite de hilos que da de esta manera no es un límite duro. Agregue uno a las instrucciones del proyecto cuando desee que se redacte exactamente y se aplique a cada hilo desde el principio.

<h3 id="unblock-a-thread-waiting-on-approval">
  Desbloquear un hilo esperando aprobación
</h3>

Los hilos se ejecutan en [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) cuando el modelo del hilo lo admite, por lo que la mayoría de las llamadas de herramientas se ejecutan sin pedirle. Cuando un hilo necesita su aprobación, el aviso está dentro de ese hilo y el hilo espera hasta que responda allí. Decirle a Claude en la conversación del proyecto que continúe no lo alcanza.

Cada aprobación cubre ese aviso, o el resto de ese hilo si elige la opción más amplia. Para permitir que cada hilo ejecute ciertos comandos sin preguntar, o para bloquear algunos, agregue [reglas de permiso](/docs/es/permissions) al `.claude/settings.json` del repositorio. Los hilos en la nube las aplican solo en un proyecto con un repositorio; vea [Qué recogen los hilos de sus repositorios](#what-threads-pick-up-from-your-repositories).

<h2 id="give-a-project-standing-context">
  Proporcione contexto permanente al proyecto
</h2>

La memoria del proyecto, las instrucciones del proyecto y los repositorios, archivos y entorno del proyecto llevan contexto entre hilos. Usted establece cada uno una sola vez.

| Contexto                         | Lo que lleva                                                                                                                                                                                                                           | Cómo lo establece                                                                                                                                                                                                                            |
| :------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Memoria del proyecto             | Notas que Claude mantiene sobre el proyecto, como requisitos, decisiones y trampas, almacenadas como archivos. Cada hilo en la nube lee el archivo de índice `MEMORY.md` cuando comienza y abre los otros archivos cuando los necesita | Pida a Claude en la conversación del proyecto o en cualquier hilo en la nube que recuerde un requisito, una decisión o una trampa, o que olvide uno. Lea, edite y elimine los archivos en **Configuración del proyecto > Memoria**           |
| Instrucciones del proyecto       | Texto enviado a cada nuevo hilo y a Claude en la conversación del proyecto, hasta 16.000 caracteres. [Escribir instrucciones del proyecto](#write-project-instructions) cubre qué poner en él                                          | **Configuración del proyecto > Memoria > Instrucciones del proyecto**, o pida a Claude que cambie las instrucciones                                                                                                                          |
| Repositorios, archivos y entorno | Los repositorios que cada hilo en la nube clona, las carpetas y archivos que puede leer en `/mnt/project-files`, y el entorno en la nube en el que se ejecuta                                                                          | Repositorios y entorno en **Configuración del proyecto > Entorno**, o pida a Claude en la conversación que agregue un repositorio al proyecto. Archivos y carpetas desde **Agregar** en la pestaña **Biblioteca** en **Descripción general** |

**Configuración del proyecto > Memoria** enumera estos archivos en **Memoria automática**, porque Claude los escribe a sí mismo mientras trabaja en el proyecto. Son separados de la [memoria automática](/docs/es/memory) que Claude Code mantiene en su máquina, aunque ambos usen un índice `MEMORY.md`. La memoria del proyecto también es separada de los archivos `CLAUDE.md` en los repositorios del proyecto. Cada hilo en la nube aún lee esos archivos `CLAUDE.md` de su clon cuando comienza, así que ponga instrucciones sobre un repositorio en su `CLAUDE.md` y notas sobre el proyecto en la memoria del proyecto.

<h3 id="write-project-instructions">
  Escribir instrucciones del proyecto
</h3>

Las instrucciones del proyecto son el resumen con el que comienza cada nuevo hilo. Haga clic en el icono de engranaje en el encabezado del proyecto para abrir **Configuración del proyecto**, luego vaya a **Memoria > Instrucciones del proyecto**. Un resumen útil cubre:

* Para qué es el proyecto
* Dónde ocurre el trabajo: qué repositorios, de qué rama comenzar, cómo nombrar solicitudes de extracción
* Cómo un hilo verifica su propio trabajo antes de considerarlo terminado
* Qué hacer cuando falta algo que necesita
* Qué necesita su aprobación primero

Por ejemplo:

```text theme={null}
Este proyecto mantiene la latencia p95 de la API de pagos por debajo de 200 ms: perfilado, correcciones de consultas y almacenamiento en caché, y las actualizaciones de dependencias que vienen con ellas, en el repositorio payments-api.

- Rama desde main y abre una solicitud de extracción de borrador por hilo.
- Antes de considerar el trabajo terminado, ejecuta `make test` y `make lint` y pega las líneas de resumen en tu mensaje final.
- Si no puedes acceder a algo que necesitas, como un repositorio, un secreto, una API o un conector, di exactamente qué falta en tu primer mensaje y detente. No sustituyas, simules ni adivines.
- No fusiones, no hagas push forzado ni cambies la configuración de CI sin preguntarme en el hilo.
```

Las reglas sobre un repositorio, como sus comandos de compilación, pertenecen al `CLAUDE.md` de ese repositorio, que cada hilo en la nube lee cuando el repositorio es parte del proyecto. Una vez que el trabajo está en marcha, cuando corrija un hilo, también dígale a Claude que recuerde la corrección: va a la [memoria del proyecto](#give-a-project-standing-context) y los hilos posteriores comienzan con ella.

<h3 id="decide-which-repositories-to-add">
  Decidir qué repositorios agregar
</h3>

Los repositorios que agrega a un proyecto vienen con todo lo que contienen, su código, `CLAUDE.md` y skills, en cada hilo en la nube. Los repositorios que no agrega aún están al alcance: un hilo en la nube puede agregar uno a sí mismo cuando su tarea lo necesita. La mayoría de los proyectos usan ambos:

* **Agréguelo al proyecto**, en el diálogo **Nuevo proyecto**, en **Configuración del proyecto > Entorno**, o pidiendo a Claude en la conversación que lo agregue al proyecto. Cada hilo a partir de entonces lo clona y comienza con su `CLAUDE.md` y skills cargados, independientemente de si la tarea lo toca. Pasar de un repositorio a varios también cambia lo que los hilos toman de cada `.claude/settings.json` del repositorio; vea [Lo que los hilos recogen de sus repositorios](#what-threads-pick-up-from-your-repositories).
* **Déjelo fuera y deje que los hilos lo agreguen cuando sea necesario.** Un hilo en la nube cuya tarea necesita un repositorio que el proyecto no tiene puede agregarlo a sí mismo, y una nota en el hilo dice que fue agregado solo a este hilo. El clon ocurre a mitad de la tarea, por lo que el `CLAUDE.md` y skills de ese repositorio no estaban allí cuando el hilo comenzó. El siguiente hilo comienza sin él. Un repositorio agregado de esta manera necesita los mismos [requisitos previos](#check-the-prerequisites) que un repositorio del proyecto: la aplicación GitHub de Claude instalada en él y acceso de push desde su cuenta de GitHub.

Un proyecto no necesita un repositorio en absoluto. Sus hilos en la nube aún pueden investigar, escribir documentos, y escribir y ejecutar código en su propio sandbox, y entregan archivos a la pestaña **Biblioteca**. Cualquiera de sus hilos en la nube aún puede agregar un repositorio a sí mismo cuando una tarea lo requiere.

Una vez que el proyecto tiene repositorios, Claude solo puede agregar repositorios de un propietario de GitHub que el proyecto ya usa, ya sea que agregue uno al proyecto o un hilo agregue uno a sí mismo. Para traer un repositorio de un propietario diferente, agréguelo al proyecto usted mismo en **Configuración del proyecto > Entorno**.

Para un proyecto que abarca muchos repositorios, como una característica con código de servidor, web, móvil y escritorio, agregue uno o dos repositorios que casi todas las tareas tocan y nombre los otros en [instrucciones del proyecto](#write-project-instructions) para que Claude sepa dónde vive el resto del código. Los hilos en la nube entonces comienzan pequeños y traen los otros repositorios solo para las tareas que los necesitan.

<h3 id="what-threads-pick-up-from-your-repositories">
  Lo que los hilos recogen de sus repositorios
</h3>

Cada hilo en la nube clona cada repositorio en el proyecto y carga `CLAUDE.md` y skills de todos ellos. Las reglas de permisos, hooks y `env` vienen solo del `.claude/settings.json` en el directorio en el que comienza el hilo: dentro del repositorio cuando el proyecto tiene uno, y encima de los clones cuando tiene varios, donde no se lee el archivo de ningún repositorio para ellos.

| En cada repositorio                                                    | Un repositorio                                                                                                                               | Varios repositorios                                                                     |
| :--------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------- |
| `CLAUDE.md`                                                            | Se carga cuando comienza el hilo                                                                                                             | Se carga desde cada repositorio cuando comienza el hilo                                 |
| Skills, agentes y comandos bajo `.claude/`                             | Se cargan                                                                                                                                    | Se cargan desde cada repositorio                                                        |
| Plugins habilitados en `.claude/settings.json`                         | No se cargan. Agregue el plugin en **Configuración del proyecto > Plugins** en su lugar                                                      | No se cargan. Agregue el plugin en **Configuración del proyecto > Plugins** en su lugar |
| Reglas de permisos, hooks y `env` definidos en `.claude/settings.json` | Se aplican al hilo, excepto las claves `env` que [ninguna sesión en la nube honra](/docs/es/cloud-environments#what-carries-over-from-your-setup) | No se aplican                                                                           |

En un proyecto con varios repositorios, cada clon se adjunta al hilo como un [directorio adicional](/docs/es/memory#load-from-additional-directories) con carga de `CLAUDE.md` activada, por lo que el `CLAUDE.md` y skills de cada repositorio se cargan al inicio aunque el hilo comience encima de ellos. En un proyecto con varios repositorios, ponga reglas permanentes en instrucciones del proyecto y proporcione a los hilos variables de entorno a través del [entorno en la nube](#choose-an-environment-for-threads).

<h3 id="choose-an-environment-for-threads">
  Elegir un entorno para los hilos
</h3>

Cada nuevo hilo en la nube comienza en el [entorno en la nube](/docs/es/cloud-environments) del proyecto. El entorno establece a qué dominios pueden acceder los hilos, qué variables de entorno tienen, qué credenciales de API se agregan a sus solicitudes, y qué instala el script de configuración antes de que Claude comience. Los hilos en la nube usan un entorno predeterminado alojado por Anthropic hasta que elige uno en **Configuración del proyecto > Entorno**.

Si los hilos en la nube necesitan acceder a una API interna o a un registro de paquetes privado, o necesitan un token que su máquina normalmente contiene, cambie el entorno en lugar del proyecto: vea [Acceso de red](/docs/es/cloud-environments#network-access), [Agregar credenciales de API](/docs/es/cloud-environments#add-api-credentials) y [Scripts de configuración](/docs/es/cloud-environments#setup-scripts).

<h3 id="get-skills-plugins-connectors-and-tools-into-threads">
  Obtener skills, plugins, conectores y herramientas en los hilos
</h3>

Los hilos en la nube no tienen los skills, servidores MCP, plugins y herramientas instalados solo en su máquina. Un hilo que Claude ejecuta en su máquina a través de [Remote Control](/docs/es/remote-control) usa lo que está instalado allí. Para que cada uno de estos esté disponible para los hilos en la nube:

* Skills, subagentes y comandos: confirme que están en un repositorio que agregó al proyecto, por ejemplo un skill en `.claude/skills/<skill-name>/SKILL.md`. Cada hilo en la nube clona cada repositorio en el proyecto y carga `.claude/skills/`, `.claude/agents/` y `.claude/commands/` de cada uno de ellos, por lo que un skill confirmado en un repositorio está disponible en cada hilo en la nube. Los hilos en la nube también cargan los skills que habilita para su cuenta de claude.ai.
* Plugins: agréguelos en **Configuración del proyecto > Plugins**; se cargan en cada nuevo hilo en la nube. Los plugins que un repositorio declara en su `.claude/settings.json` [no se cargan en los hilos en la nube](/docs/es/cloud-environments#what-carries-over-from-your-setup).
* Servidores MCP: los hilos en la nube obtienen sus herramientas MCP de los conectores en su cuenta de claude.ai, que son servidores MCP que conecta una sola vez en [claude.ai/customize/connectors](https://claude.ai/customize/connectors) o a través del enlace **Administrar conectores** en **Configuración del proyecto > Entorno**. Cada hilo en la nube puede usar todos ellos sin configuración por proyecto. La conversación del proyecto en sí no tiene conectores, así que envíe el trabajo que necesita uno como una tarea para un hilo en la nube. En un proyecto con un repositorio, los hilos en la nube también cargan servidores MCP del [`.mcp.json`](/docs/es/cloud-environments#what-carries-over-from-your-setup) de ese repositorio. [Cómo los conectores llegan a Claude Code](/docs/es/mcp#how-connectors-reach-claude-code) enumera las reglas para sesiones en la nube y la configuración que desactiva los conectores.
* Herramientas de línea de comandos y paquetes: instálelos en el [script de configuración](/docs/es/cloud-environments#setup-scripts) del entorno.

Para ver qué conectores tiene un hilo en la nube en ejecución en claude.ai/code, abra el hilo y seleccione **Conectores** en el menú **+** junto a su cuadro de mensaje. Desactivar un conector allí lo elimina de ese hilo y guarda eso como su predeterminado de cuenta, por lo que los nuevos hilos en la nube y chats de claude.ai comienzan sin él hasta que lo vuelva a activar. Un hilo en la nube recoge un conector que agrega o reconecta después del siguiente mensaje que le envía.

<h2 id="project-settings-reference">
  Referencia de configuración del proyecto
</h2>

Cambias la configuración del proyecto en claude.ai/code o en la aplicación de escritorio, no en `settings.json`. Abre **Project settings** desde **Settings** en el menú de la barra lateral del proyecto o desde el icono de engranaje en el encabezado del proyecto.

Los cambios se guardan a medida que los realizas; un campo de texto que estés editando, como el objetivo o las instrucciones, muestra **Save changes** y **Discard** hasta que lo dejes. Los cambios en instrucciones, repositorios, plugins, y entorno en **Project settings** llegan a hilos nuevos, no a hilos ya en ejecución.

| Configuración                 | Sección     | Qué controla                                                                                                               |
| :---------------------------- | :---------- | :------------------------------------------------------------------------------------------------------------------------- |
| Nombre, icono y objetivo      | General     | El nombre e icono del proyecto en la barra lateral, y su objetivo de una línea                                             |
| Modelo coordinador y esfuerzo | General     | El modelo y [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) para Claude en la conversación del proyecto          |
| Modelo de hilo y esfuerzo     | General     | El modelo y nivel de esfuerzo para hilos                                                                                   |
| Instrucciones del proyecto    | Memory      | [Reglas permanentes](#give-a-project-standing-context) que cada hilo nuevo recibe                                          |
| Repositorios del proyecto     | Environment | Los repositorios que los hilos nuevos clonan                                                                               |
| Entorno en la nube            | Environment | El [entorno en la nube](#choose-an-environment-for-threads) en el que se ejecutan los hilos nuevos                         |
| Conectores                    | Environment | Un enlace para administrar los conectores de claude.ai que obtienen los hilos                                              |
| Plugins                       | Plugins     | Los plugins que se cargan en cada hilo nuevo                                                                               |
| Uso                           | Usage       | [Uso de tokens](#usage-and-cost) por hilo y por modelo                                                                     |
| Memory                        | Memory      | Los [archivos de memoria](#give-a-project-standing-context) del proyecto                                                   |
| Reiniciar Claude              | General     | Reinicia la conversación del proyecto cuando [Claude deja de responder allí](#claude-hasnt-responded)                      |
| Pausar, Archivar, Eliminar    | General     | Detiene, oculta, o elimina el proyecto; ver [Pausar, archivar, o eliminar un proyecto](#pause-archive-or-delete-a-project) |

<h3 id="pause-archive-or-delete-a-project">
  Pausar, archivar, o eliminar un proyecto
</h3>

Los tres controles están en la parte inferior de **Project settings > General**:

* **Pause**: detiene todo a la vez. Cada hilo en ejecución y la conversación se interrumpen, no comienzan hilos nuevos, las rutinas no se ejecutan, y el proyecto no acepta mensajes hasta que lo reanudes. Haz clic en **Resume** en el mismo lugar o en el banner arriba del cuadro de mensaje del proyecto; un hilo pausado continúa cuando le envías un mensaje después de eso.
* **Archive**: oculta el proyecto de la barra lateral y archiva sus hilos, que detiene cualquier hilo que estaba en ejecución o observando una solicitud de extracción. Las rutinas en el proyecto no se ejecutan mientras está archivado. Para traer el proyecto de vuelta, ábrelo desde la página Projects y haz clic en **Unarchive**. Sus hilos permanecen archivados hasta que los desarchives individualmente desde la lista de sesiones.
* **Delete**: elimina permanentemente el proyecto junto con sus hilos, su memoria, y sus archivos, y desactiva las rutinas del proyecto. Esto no se puede deshacer. Las ramas y solicitudes de extracción que los hilos insertaron en GitHub no se ven afectadas.

<h2 id="usage-and-cost">
  Uso y costo
</h2>

El uso del proyecto cuenta contra los mismos [límites de plan](/docs/es/errors#youve-hit-your-session-limit) que tus otras sesiones de Claude Code, y un proyecto no puede gastar más allá de esos límites por su cuenta.

Un hilo que alcanza el límite de tu plan espera y continúa por su cuenta cuando el límite se reinicia, así que el trabajo que dejaste en ejecución comienza a usar tu siguiente ventana de uso sin un mensaje de ti. [Un hilo alcanzó el límite de uso](#usage-limit-reached) cubre lo que ves, cómo detenerlo, y el único caso que no espera.

El trabajo va más allá de los límites de tu plan solo si has activado [créditos de uso](/docs/es/costs#add-usage-credits-to-your-subscription) para tu cuenta. Un hilo no puede activarlos para ti.

<h3 id="what-draws-on-your-plan">
  Qué utiliza tu plan
</h3>

Un proyecto utiliza tus límites más rápido que una sola sesión, y en un plan Pro en particular deberías esperar alcanzar tu límite más pronto en días que ejecutes uno. Estas son las partes de un proyecto que utilizan tu plan:

* **Hilos en ejecución**: cada uno es una sesión completa, y varios pueden ejecutarse a la vez. No hay un número fijo; Claude comienza tantos como el trabajo requiera, y un límite que [pidas](#tune-how-claude-runs-a-project) es una preferencia en lugar de un límite. El límite aplicado es 200 hilos nuevos por día en todos tus proyectos.
* **La conversación**: Claude utiliza tokens propios leyendo lo que los hilos reportan y decidiendo qué hacer a continuación.
* **Hilos observando una solicitud de extracción**: un hilo inactivo se despierta y utiliza tu plan de nuevo cuando CI falla o llega un comentario de revisión en su solicitud de extracción. Para detener eso, pídele en el hilo que deje de observar la solicitud de extracción.

Un proyecto sin hilos en ejecución, sin solicitudes de extracción observadas, y sin mensajes nuevos no utiliza tu plan mientras se sienta inactivo, y tampoco un proyecto archivado.

<h3 id="see-and-reduce-a-project’s-usage">
  Ve y reduce el uso de un proyecto
</h3>

Abre **Usage** en **Project settings** para ver el uso de tokens por hilo y por modelo, y cuánto fue a la conversación del proyecto. Para reducirlo:

* Un seguimiento enrutado a un hilo que ha estado inactivo más tiempo que la [vida útil del caché](/docs/es/prompt-caching#cache-lifetime), una hora en Pro y Max dentro de los límites de tu plan, vuelve a leer toda la conversación de ese hilo antes de hacer nada. Para trabajo nuevo, pedirle a Claude que comience un hilo fresco puede usar menos que revivir uno antiguo grande.
* Para trabajo que no necesita el modelo más grande, [elige un modelo más pequeño o un nivel de esfuerzo más bajo](#choose-models-and-let-claude-manage-context) para hilos, la conversación, o ambos.
* Pídele a Claude en la conversación del proyecto que ejecute menos hilos a la vez, o que responda pequeñas preguntas él mismo en lugar de comenzar un hilo.

<h2 id="how-projects-relate-to-other-claude-code-features">
  Cómo los proyectos se relacionan con otras características de Claude Code
</h2>

Varias características de Claude Code permiten que más de una sesión trabaje al mismo tiempo, así que ejecutar trabajo en paralelo no es por sí solo para qué es un proyecto. En un proyecto, Claude comienza y rastrea las sesiones en lugar de usted, y cada una comienza desde las mismas instrucciones. Así es como cada característica vecina se conecta a un proyecto:

* **Claude Tag**: [Claude Tag](https://claude.com/docs/claude-tag/overview) es Claude en los canales de Slack de tu equipo, en planes Team y Enterprise. Cualquiera en un canal puede darle trabajo, todos en el canal lo ven y lo dirigen, y utiliza conexiones que un administrador configuró para ese canal. Un proyecto es solo tuyo: eres el único que le envía trabajo o ve sus hilos, utiliza tu propio acceso a GitHub y conectores, y está en Pro y Max. [Cómo Claude Tag difiere de Cowork y Claude Code](https://claude.com/docs/claude-tag/concepts/how-it-works#how-claude-tag-differs-from-cowork-and-claude-code) tiene el lado a lado.
* **Cloud sessions**: cada hilo es una [sesión en la nube](/docs/es/claude-code-on-the-web) a menos que pida a Claude que la ejecute en su máquina. De cualquier forma, Claude la comienza y rastrea en lugar de usted. Una sesión en la nube que comenzó usted mismo puede convertirse en un proyecto o alimentar uno a través de [**Continue as a project** o **Move to project**](#start-from-an-existing-cloud-session).
* **Routines**: cuando pides trabajo programado en un proyecto, Claude crea una [rutina](/docs/es/routines) que se ejecuta como hilos en ese proyecto y aparece en su pestaña **Routines**. Las rutinas que creas fuera de un proyecto siguen funcionando por su cuenta.
* **Local sessions and agent view**: una sesión que comienza usted mismo en su terminal, IDE, o el entorno local de la aplicación de escritorio no puede ser agregada a un proyecto. Un proyecto llega a su máquina solo ejecutando un hilo allí a través de [Remote Control](/docs/es/remote-control). [Agent view](/docs/es/agent-view) es una pantalla para rastrear varias sesiones locales que comenzó usted mismo; no tiene coordinador.
* **Worktrees**: un [worktree](/docs/es/worktrees) le da a cada sesión local su propia copia de trabajo de un repositorio para que las sesiones paralelas en su máquina no se sobrescriban entre sí. Los hilos en la nube no los necesitan: cada uno clona sus repositorios en su propia sandbox en la nube y trabaja en su propia rama.
* **Equipos de agentes**: un [equipo de agentes](/docs/es/agent-teams) es una sesión que comienza sesiones de compañeros para una sola tarea, en tu máquina o dentro de una sesión en la nube, y termina con esa tarea.
* **Projects en claude.ai chat y Cowork**: la [experiencia anterior de Projects](https://support.claude.com/en/articles/9517075-what-are-projects), que agrupa conversaciones y archivos de referencia sin hilos o coordinador. Esos proyectos siguen funcionando como lo hacen hoy hasta que la experiencia rediseñada los alcance.

[Ejecutar agentes en paralelo](/docs/es/agents) compara estas opciones lado a lado.

<h2 id="limitations">
  Limitaciones
</h2>

* Los proyectos están disponibles en claude.ai/code, en la aplicación de escritorio y en la aplicación móvil de Claude, no en el CLI de terminal ni a través de Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry. El comando [`claude project`](/docs/es/cli-reference) del CLI, que administra el estado local de Claude Code para un directorio, no está relacionado.
* Los hilos del proyecto son [sesiones en la nube](/docs/es/claude-code-on-the-web), o sesiones en su propia máquina a través de [Remote Control](/docs/es/remote-control), con Anthropic como proveedor de modelo en ambos casos. [Seguridad](/docs/es/security) y [Uso de datos](/docs/es/data-usage) cubren cómo se aíslan las sesiones en la nube y qué se retiene, y [Connection and security](/docs/es/remote-control#connection-and-security) cubre cómo un hilo en su máquina se conecta y qué se almacena.
* No puede agregar una sesión que inició usted mismo en su máquina a un proyecto. Para permitir que un proyecto ejecute un hilo en su máquina, conecte la carpeta en la que debe trabajar a través de [Remote Control](/docs/es/remote-control#requirements): active Remote Control en **Settings > Claude Code** en la aplicación de escritorio de Claude, o ejecute `claude remote-control` en la carpeta y déjela ejecutándose. Esa máquina necesita Claude Code v2.1.280 o posterior. Un proyecto tampoco puede ejecutar un hilo en su máquina mientras **Require trusted devices** esté activado en su configuración de claude.ai.
* La sandbox de un hilo en la nube se pausa entre turnos y se reanuda cuando el hilo continúa. Si la sandbox no se puede reanudar, el hilo continúa desde un clon fresco, así que los cambios no confirmados pueden perderse. En tareas largas, pídele a Claude que confirme e inserte trabajo en progreso.
* Un proyecto pertenece a un usuario. No puedes compartir un proyecto o sus hilos con otro usuario, y las transcripciones de hilos no tienen la opción de compartir que tienen otras sesiones en la nube. No hay controles a nivel de organización para proyectos durante la beta.
* Un hilo pertenece al único proyecto que lo comenzó. No puedes mover o copiar un hilo a otro proyecto, o sacarlo para que esté solo. [**Move to project**](#start-from-an-existing-cloud-session) va solo en la otra dirección: trae el trabajo de una sesión en la nube a un proyecto.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

Para los avisos de configuración de GitHub en el diálogo **New project**, ver [Configurar acceso a GitHub](#set-up-github-access).

<h3 id="a-thread-looks-stuck">
  Un hilo parece estar atascado
</h3>

Claude no publica cada paso que toma un hilo, así que un hilo que se muestra como en ejecución sin mensajes nuevos en la conversación del proyecto generalmente aún está trabajando. Un hilo nuevo también aprovisiona su [entorno en la nube](/docs/es/cloud-environments) antes de que Claude comience, así que su primera actualización toma un momento. Abre el hilo para leer su transcripción. Si el hilo está esperando un aviso de permiso, respóndelo allí.

<h3 id="threads-guessed-or-stalled-instead-of-asking">
  Los hilos adivinaron o se estancaron en lugar de preguntar
</h3>

Cuando varios hilos regresan habiendo asumido algo incorrecto, trabajado alrededor del acceso faltante, o detenido con "bloqueado", la causa generalmente es la misma brecha en la configuración del proyecto en lugar de un problema con cada tarea. Ordena qué hilos son sólidos antes de arreglar nada:

1. Pídele a Claude en la conversación: "Para cada hilo abierto, enumera lo que le pediste que hiciera, qué asumió o no pudo alcanzar, y en qué está esperando." Claude lee cada hilo y responde en la conversación.
2. Para hilos que comenzaron desde una suposición incorrecta, abre el hilo desde **Overview** y márcalo resuelto desde su menú, o dile qué hacer en su lugar en su cuadro de mensaje. Su rama y cualquier solicitud de extracción permanecen en GitHub hasta que las elimines.
3. Arregla la brecha una vez, en [instrucciones del proyecto](#give-a-project-standing-context) o el [entorno](#choose-an-environment-for-threads), luego envía un hilo antes de enviar el resto del trabajo de nuevo como hilos nuevos.

<h3 id="claude-hasnt-responded">
  Claude no ha respondido
</h3>

La conversación del proyecto muestra un banner "Claude hasn't responded" cuando Claude está en ejecución pero sus respuestas no llegan al proyecto. Haz clic en **Restart Claude** en el banner, o ve a **Project settings > General** y haz clic en **Restart** en la fila **Restart Claude**. Claude se reconecta a la conversación; cualquier respuesta que estaba en medio de escribir se pierde, y los hilos no se ven afectados.

<h3 id="repository-access-errors">
  Errores de acceso al repositorio
</h3>

Tres mensajes significan que un hilo o el proyecto no puede alcanzar uno de sus repositorios. Un hilo del proyecto necesita los [requisitos previos de GitHub](#check-the-prerequisites) incluso cuando tus otras sesiones en la nube clonan el mismo repositorio sin problemas.

* **"Couldn't start the session — Claude doesn't have GitHub access to this project's repository"**, reportado antes de que comience el hilo, cuando la Claude GitHub App no está instalada en ese repositorio, está suspendida, o no está vinculada a la cuenta de GitHub que conectaste.
* **"Unable to access your repository"**, reportado por un hilo cuando su clon falla: GitHub rechazó el clon, el repositorio no se encontró bajo el nombre que tiene el proyecto, o la rama desde la que se le pidió al hilo que comenzara no existe.
* **"Claude can't access"** un repositorio, mostrado cuando guardas repositorios en el diálogo **New project** o **Project settings**. El mensaje continúa con un enlace de instalación y un enlace de reconexión. Usa el enlace de instalación si la Claude GitHub App no está en ese repositorio, y el enlace de reconexión si lo está, ya que la Claude GitHub App puede estar instalada en GitHub sin estar vinculada a la cuenta que conectaste a Claude. Si el mensaje dice que la Claude GitHub App está suspendida o no incluye este repositorio, sigue su enlace a GitHub para arreglarlo.

Para arreglar cualquiera de ellos, haz clic en el botón que ofrece el mensaje, como **Install GitHub App** o **Select repositories on GitHub**, luego **Check again**. Cuando el bloqueo está del lado de la organización de GitHub, como un propietario que no ha aprobado la aplicación o una lista de permitidos de IP que excluye a Claude, el mensaje muestra un enlace **See how to fix** en su lugar. Si no hay botón, sigue [Configurar acceso a GitHub](#set-up-github-access), luego envía otro mensaje para reintentar.

<h3 id="usage-limit-reached">
  Un hilo alcanzó el límite de uso
</h3>

Cuando un hilo o la conversación del proyecto alcanza el límite de cinco horas o semanal de tu plan, sigue reintentando por su cuenta y continúa cuando el límite se reinicia. Mientras espera, el hilo muestra **Service is busy** con "Claude is still retrying and will continue automatically." No necesitas hacer nada para que el trabajo continúe. Si prefieres que no use tu siguiente ventana de uso, haz clic en **Stop** en el hilo, o [pausa el proyecto](#pause-archive-or-delete-a-project) para mantener cada hilo. Un hilo que una rutina comenzó no espera: su turno se detiene con un error de límite, y le envías un mensaje después de que el límite se reinicia.

[Errores de límite de uso](/docs/es/errors#youve-hit-your-session-limit) explican los límites y cuándo se reinician.

<h3 id="additional-usage-credits-are-required">
  Se requieren créditos de uso adicionales
</h3>

Un hilo o la conversación del proyecto hizo una solicitud que tu plan cubre solo con créditos de uso, como una a un modelo o tamaño de contexto que tu plan no incluye, y los créditos de uso no están activados para tu cuenta. [Agregar créditos de uso a tu suscripción](/docs/es/costs#add-usage-credits-to-your-subscription) cubre quién puede activarlos o comprarlos en cada plan. Una vez que los créditos estén disponibles, envía otro mensaje para reintentar.

<h3 id="context-limit">
  Otros mensajes
</h3>

Estos mensajes nombran su propia causa. La tabla da el próximo paso para cada uno.

| Mensaje                                                                                       | Qué hacer                                                                                                                                                                                                                                      |
| :-------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| "Unable to connect to repository" con "Claude couldn't reach GitHub to fetch your repository" | Espera un momento, luego envía otro mensaje para reintentar                                                                                                                                                                                    |
| "Unable to connect to repository" con "Claude couldn't access your repository or environment" | Tu cuenta de GitHub necesita acceso de inserción al repositorio, y el entorno aún debe existir. Verifica ambos en **Project settings > Environment**, luego reintentar                                                                         |
| "Couldn't show the setup proposal"                                                            | La aplicación que tienes abierta es más antigua que las **Setup recommendations** que Claude envió. Actualiza la página o reinicia la aplicación de escritorio, o pídele a Claude que proponga la configuración de nuevo                       |
| "The project's environment was removed"                                                       | Elige un entorno diferente en **Project settings > Environment**; el cambio se aplica a hilos nuevos                                                                                                                                           |
| "Setup script failed"                                                                         | Haz clic en **Edit setup script** en el error, arregla el script en el entorno, luego envía otro mensaje. [Setup script failed](/docs/es/web-quickstart#setup-script-failed) enumera las causas comunes                                             |
| "Claude ran out of context on this turn"                                                      | El hilo llenó su ventana de contexto. Si el mensaje dice que el hilo continúa en una sesión fresca, continúa por sí solo; de lo contrario, pídele a Claude en la conversación del proyecto que comience un hilo nuevo para el trabajo restante |
| "Reached the turn limit"                                                                      | El hilo alcanzó el límite de pasos agenticos que [`CLAUDE_CODE_MAX_TURNS`](/docs/es/env-vars) establece. Envía otro mensaje para continuar, o aumenta o elimina esa variable donde esté establecida                                                 |

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Usa Claude Code en la nube](/docs/es/claude-code-on-the-web): cómo funcionan las sesiones en la nube detrás de cada hilo, incluidas las opciones de acceso a GitHub y auto-fix en solicitudes de extracción
* [Configura entornos en la nube](/docs/es/cloud-environments): cambia a qué pueden llegar los hilos en la red, dale variables de entorno y credenciales de API, e instala herramientas con un script de configuración
* [Automatiza trabajo con rutinas](/docs/es/routines): horarios, disparadores, y administración para rutinas, incluidas las que Claude crea desde un proyecto
* [Administra múltiples agentes con vista de agente](/docs/es/agent-view): ejecuta y rastrea varias sesiones en tu propia máquina cuando el trabajo necesita herramientas o servicios que solo tu máquina puede alcanzar
* [Proyectos rediseñados: de carpeta a conversación](https://claude.com/blog/projects-redesigned): el anuncio de lanzamiento, con el pensamiento detrás de hacer que un proyecto sea una conversación con Claude
