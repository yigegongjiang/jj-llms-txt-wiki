> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Comienza con Claude Code en la web

> Ejecuta Claude Code en la nube desde tu navegador o teléfono. Conecta un repositorio de GitHub, envía una tarea y revisa el PR sin configuración local.

<Note>
  Claude Code en la web está en vista previa de investigación para usuarios Pro, Max y Team, y para usuarios Enterprise con asientos premium o asientos de Chat + Claude Code.
</Note>

Claude Code en la web se ejecuta en la infraestructura en la nube administrada por Anthropic en lugar de en tu máquina. Envía tareas desde [claude.ai/code](https://claude.ai/code) en tu navegador o en la aplicación móvil de Claude.

Necesitarás un repositorio de GitHub para [comenzar](#connect-github-and-create-an-environment). Claude lo clona en una máquina virtual aislada, realiza cambios e impulsa una rama para que la revises. Las sesiones persisten entre dispositivos, por lo que una tarea que comiences en tu portátil está lista para revisar desde tu teléfono más tarde.

Claude Code en la web funciona bien para:

* **Tareas paralelas**: ejecuta varias tareas independientes a la vez, cada una en su propia sesión y rama, sin necesidad de gestionar múltiples worktrees
* **Repositorios que no tienes localmente**: Claude clona el repositorio nuevo en cada sesión, por lo que no necesitas tenerlo descargado
* **Tareas que no necesitan dirección frecuente**: envía una tarea bien definida, haz otra cosa y revisa el resultado cuando Claude haya terminado
* **Preguntas sobre código y exploración**: comprende una base de código o rastrea cómo se implementa una función sin una descarga local

Para trabajos que necesitan tu configuración local, herramientas o entorno, ejecutar Claude Code localmente o usar [Remote Control](/docs/es/remote-control) es una mejor opción.

<h2 id="how-sessions-run">
  Cómo se ejecutan las sesiones
</h2>

Cuando envías una tarea:

1. **Clonar y preparar**: tu repositorio se clona en una VM administrada por Anthropic, y tu [script de configuración](/docs/es/claude-code-on-the-web#setup-scripts) se ejecuta si está configurado.
2. **Configurar red**: el acceso a internet se establece según el [nivel de acceso](/docs/es/claude-code-on-the-web#access-levels) de tu entorno.
3. **Trabajar**: Claude analiza el código, realiza cambios, ejecuta pruebas y verifica su trabajo. Puedes observar y dirigir en todo momento, o alejarte y volver cuando haya terminado.
4. **Impulsar la rama**: cuando Claude alcanza un punto de parada, impulsa su rama a GitHub. Revisa el diff, deja comentarios en línea, crea un PR o envía otro mensaje para continuar.

La sesión no se cierra cuando se impulsa la rama. La creación de PR y ediciones adicionales ocurren dentro de la misma conversación.

<h2 id="compare-ways-to-run-claude-code">
  Compara formas de ejecutar Claude Code
</h2>

Claude Code se comporta igual en todas partes. Lo que cambia es dónde se ejecuta el código y si tu configuración local está disponible. La aplicación Desktop ofrece sesiones locales y en la nube, por lo que las respuestas a continuación dependen de cuál elijas:

|                                              | En la web                                                                                                                 | Remote Control                                     | Terminal CLI      | Aplicación Desktop            |
| :------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------- | :---------------- | :---------------------------- |
| **El código se ejecuta en**                  | VM en la nube de Anthropic                                                                                                | Tu máquina                                         | Tu máquina        | Tu máquina o VM en la nube    |
| **Chateas desde**                            | claude.ai o aplicación móvil                                                                                              | claude.ai o aplicación móvil                       | Tu terminal       | La interfaz de Desktop        |
| **Usa tu configuración local**               | No, solo repositorio                                                                                                      | Sí                                                 | Sí                | Sí para local, no para nube   |
| **Requiere GitHub**                          | Sí, o [agrupa un repositorio local](/docs/es/claude-code-on-the-web#send-local-repositories-without-github) mediante `--cloud` | No                                                 | No                | Solo para sesiones en la nube |
| **Sigue ejecutándose si te desconectas**     | Sí                                                                                                                        | Mientras la terminal permanezca abierta            | No                | Depende del tipo de sesión    |
| **[Modos de permiso](/docs/es/permission-modes)** | Aceptar ediciones, Plan, Auto                                                                                             | Preguntar, Aceptar ediciones automáticamente, Plan | Todos los modos   | Depende del tipo de sesión    |
| **Acceso a la red**                          | Configurable por entorno                                                                                                  | Red de tu máquina                                  | Red de tu máquina | Depende del tipo de sesión    |

Consulta los documentos de [inicio rápido de terminal](/docs/es/quickstart), [aplicación Desktop](/docs/es/desktop) o [Remote Control](/docs/es/remote-control) para configurarlos.

<h2 id="connect-github-and-create-an-environment">
  Conecta GitHub y crea un entorno
</h2>

La configuración es un proceso único. Si ya usas la CLI de GitHub, puedes [hacer esto desde tu terminal](#connect-from-your-terminal) en lugar del navegador.

<Steps>
  <Step title="Visita claude.ai/code">
    Ve a [claude.ai/code](https://claude.ai/code) e inicia sesión con tu cuenta de Anthropic.
  </Step>

  <Step title="Instala la aplicación Claude GitHub">
    Después de iniciar sesión, claude.ai/code te solicita que conectes GitHub. Sigue el mensaje para instalar la aplicación Claude GitHub y otorgarle acceso a tus repositorios. Las sesiones en la nube funcionan con repositorios de GitHub existentes, por lo que para iniciar un nuevo proyecto, [crea un repositorio vacío en GitHub](https://github.com/new) primero.
  </Step>

  <Step title="Crea tu entorno">
    Después de conectar GitHub, se te pedirá que crees un entorno en la nube. El entorno controla qué acceso a la red tiene Claude durante las sesiones y qué se ejecuta cuando se crea una nueva sesión. Consulta [Herramientas instaladas](/docs/es/claude-code-on-the-web#installed-tools) para ver qué está disponible sin ninguna configuración.

    El formulario tiene estos campos:

    * **Nombre**: una etiqueta de visualización. Útil cuando tienes múltiples entornos para diferentes proyectos o niveles de acceso.
    * **Acceso a la red**: controla a qué puede acceder la sesión en internet. El valor predeterminado, `Trusted`, permite conexiones a [registros de paquetes comunes](/docs/es/claude-code-on-the-web#default-allowed-domains) como npm, PyPI y RubyGems mientras bloquea el acceso general a internet.
    * **Variables de entorno**: variables opcionales disponibles en cada sesión, en formato `.env`. No envuelvas los valores entre comillas, ya que las comillas se almacenan como parte del valor. Estas son visibles para cualquiera que pueda editar este entorno.
    * **Script de configuración**: un script Bash opcional que se ejecuta antes de que se lance Claude Code. Úsalo para instalar herramientas del sistema que la VM en la nube no incluye, como `apt install -y gh`. El resultado se [almacena en caché](/docs/es/claude-code-on-the-web#environment-caching), por lo que el script no se vuelve a ejecutar en cada sesión. Consulta [Scripts de configuración](/docs/es/claude-code-on-the-web#setup-scripts) para ver ejemplos y consejos de depuración.

    Para un primer proyecto, deja los valores predeterminados y haz clic en **Crear entorno**. Puedes [editarlo más tarde o crear entornos adicionales](/docs/es/claude-code-on-the-web#configure-your-environment) para diferentes proyectos.
  </Step>
</Steps>

<h3 id="connect-from-your-terminal">
  Conecta desde tu terminal
</h3>

Si ya usas la CLI de GitHub (`gh`), puedes configurar Claude Code en la web sin abrir un navegador. Esto requiere la [CLI de Claude Code](/docs/es/quickstart). `/web-setup` lee tu token local de `gh`, lo vincula a tu cuenta de Claude y crea un entorno en la nube predeterminado si no tienes uno.

<Note>
  Las organizaciones con [Retención de datos cero](/docs/es/zero-data-retention) habilitada no pueden usar `/web-setup` u otras características de sesión en la nube. Si la CLI de GitHub no está instalada o autenticada, `/web-setup` abre el flujo de incorporación del navegador en su lugar.
</Note>

<Steps>
  <Step title="Autentica con la CLI de GitHub">
    En tu shell, autentica la CLI de GitHub si aún no lo has hecho:

    ```bash theme={null}
    gh auth login
    ```
  </Step>

  <Step title="Inicia sesión en Claude">
    En la CLI de Claude Code, ejecuta `/login` para iniciar sesión con tu cuenta de claude.ai. Omite este paso si ya has iniciado sesión.
  </Step>

  <Step title="Ejecuta /web-setup">
    En la CLI de Claude Code, ejecuta:

    ```text theme={null}
    /web-setup
    ```

    Esto sincroniza tu token de `gh` con tu cuenta de Claude. Si aún no tienes un entorno en la nube, `/web-setup` crea uno con acceso a red Trusted y sin script de configuración. Puedes [editar el entorno o agregar variables](/docs/es/claude-code-on-the-web#configure-your-environment) después. Una vez que `/web-setup` se complete, puedes iniciar sesiones en la nube desde tu terminal con [`--cloud`](/docs/es/claude-code-on-the-web#from-terminal-to-web) o configurar tareas recurrentes con [`/schedule`](/docs/es/routines).
  </Step>
</Steps>

<h2 id="start-a-task">
  Inicia una tarea
</h2>

Con GitHub conectado y un entorno creado, estás listo para enviar tareas.

<Steps>
  <Step title="Selecciona un repositorio y rama">
    Desde [claude.ai/code](https://claude.ai/code) o la pestaña Code en la aplicación móvil de Claude, haz clic en el selector de repositorio debajo del cuadro de entrada y elige un repositorio en el que Claude pueda trabajar. Cada repositorio muestra un selector de rama. Cámbialo para que Claude comience desde una rama de función en lugar de la predeterminada. Puedes agregar múltiples repositorios para trabajar en ellos en una sesión.
  </Step>

  <Step title="Elige un modo de permiso">
    El menú desplegable de modo junto a la entrada tiene como valor predeterminado **Aceptar ediciones automáticamente**, donde Claude realiza cambios e impulsa una rama sin detenerse para aprobación. Cambia a **Plan Mode** si deseas que Claude proponga un enfoque y espere tu aprobación antes de editar archivos. Las sesiones en la nube no ofrecen permisos Manual o Bypass. Consulta la [lista completa de modos de permiso](/docs/es/permission-modes#available-modes) para ver qué permite cada uno.
  </Step>

  <Step title="Describe la tarea y envía">
    Escribe una descripción de lo que deseas y presiona Enter. Sé específico:

    * Nombra el archivo o función: "Agregar un README con instrucciones de configuración" o "Corregir la prueba de autenticación fallida en `tests/test_auth.py`" es mejor que "corregir pruebas"
    * Pega la salida de error si la tienes
    * Describe el comportamiento esperado, no solo el síntoma

    Claude clona los repositorios, ejecuta tu script de configuración si está configurado e inicia el trabajo. Cada tarea obtiene su propia sesión y su propia rama, por lo que no necesitas esperar a que una termine antes de iniciar otra.
  </Step>
</Steps>

<h2 id="pre-fill-sessions">
  Sesiones rellenadas previamente
</h2>

Puedes rellenar previamente el mensaje, los repositorios y el entorno para una nueva sesión agregando parámetros de consulta a la URL de [claude.ai/code](https://claude.ai/code). Úsalo para crear integraciones como un botón en tu rastreador de problemas que abre Claude Code con la descripción del problema como mensaje.

| Parámetro      | Descripción                                                                                                                                                                                                             |
| :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`       | Texto del mensaje para rellenar en el cuadro de entrada. También se acepta el alias `q`.                                                                                                                                |
| `prompt_url`   | URL para obtener el texto del mensaje, para mensajes demasiado largos para incrustar en una cadena de consulta. La URL debe permitir solicitudes de origen cruzado. Se ignora cuando `prompt` también está establecido. |
| `repositories` | Lista separada por comas de slugs `owner/repo` para preseleccionar. También se acepta el alias `repo`.                                                                                                                  |
| `environment`  | Nombre o ID del [entorno](#connect-github-and-create-an-environment) para preseleccionar.                                                                                                                               |

Codifica en URL cada valor. El ejemplo a continuación abre el formulario con un mensaje y un repositorio ya seleccionados:

```text theme={null}
https://claude.ai/code?prompt=Fix%20the%20login%20bug&repositories=acme/webapp
```

<h2 id="review-and-iterate">
  Revisa e itera
</h2>

Cuando Claude termina, revisa los cambios, deja comentarios en líneas específicas y continúa hasta que el diff se vea bien.

<Steps>
  <Step title="Abre la vista de diff">
    Un indicador de diff muestra líneas agregadas y eliminadas en toda la sesión, por ejemplo `+42 -18`. Selecciónalo para abrir la vista de diff, con una lista de archivos a la izquierda y cambios a la derecha.
  </Step>

  <Step title="Deja comentarios en línea">
    Selecciona cualquier línea en el diff, escribe tu comentario y presiona Enter. Los comentarios se colan hasta que envíes tu siguiente mensaje, luego se agrupan con él. Claude ve "en `src/auth.ts:47`, no captures el error aquí" junto a tu instrucción principal, por lo que no tienes que describir dónde está el problema.
  </Step>

  <Step title="Crea una solicitud de extracción">
    Cuando el diff se vea bien, selecciona **Crear PR** en la parte superior de la vista de diff. Puedes abrirlo como un PR completo, un borrador, o ir a la página de composición de GitHub con un título y descripción generados.
  </Step>

  <Step title="Continúa iterando después del PR">
    La sesión permanece activa después de que se crea el PR. Pega la salida de falla de CI o comentarios del revisor en el chat y pide a Claude que los aborde. Para que Claude monitoree el PR automáticamente, consulta [Corregir automáticamente solicitudes de extracción](/docs/es/claude-code-on-the-web#auto-fix-pull-requests).
  </Step>
</Steps>

<h2 id="troubleshoot-setup">
  Soluciona problemas de configuración
</h2>

<h3 id="no-repositories-appear-after-connecting-github">
  No aparecen repositorios después de conectar GitHub
</h3>

Una sesión en la nube puede usar cualquier repositorio que la cuenta de GitHub conectada pueda ver, independientemente de en qué repositorios esté instalada la aplicación Claude GitHub. Si falta un repositorio, verifica que la cuenta de GitHub conectada tenga acceso a él en GitHub. Si también deseas [Auto-fix](/docs/es/claude-code-on-the-web#auto-fix-pull-requests) para un repositorio, instala la aplicación en él: en github.com, abre **Configuración → Aplicaciones → Claude → Configurar** y verifica que el repositorio esté listado en **Acceso a repositorios**. Los repositorios privados necesitan la misma autorización que los públicos.

<h3 id="the-page-only-shows-a-github-login-button">
  La página solo muestra un botón de inicio de sesión de GitHub
</h3>

Las sesiones en la nube requieren una cuenta de GitHub conectada. Conecta a través del flujo del navegador anterior, o ejecuta `/web-setup` desde tu terminal si usas la CLI de GitHub. Si prefieres no conectar GitHub en absoluto, consulta [Remote Control](/docs/es/remote-control) para ejecutar Claude Code en tu propia máquina y monitorearlo desde la web.

<h3 id="not-available-for-the-selected-organization">
  "No disponible para la organización seleccionada"
</h3>

Las organizaciones Enterprise pueden necesitar que un administrador habilite Claude Code en la web. Contacta a tu equipo de cuenta de Anthropic.

<h3 id="/web-setup-shows-no-commands-match-or-unknown-command">
  `/web-setup` muestra "No commands match" o "Unknown command"
</h3>

`/web-setup` se ejecuta dentro de la CLI de Claude Code, no en tu shell. Inicia `claude` primero, luego escribe `/web-setup` en el mensaje.

Si lo escribiste dentro de Claude Code y el menú de comandos muestra `No commands match "/web-setup"`, o enviarlo devuelve `Unknown command: /web-setup`, el comando está oculto porque no se cumple un requisito. La causa generalmente es que estás autenticado con una clave API o proveedor de terceros en lugar de una suscripción de claude.ai. Ejecuta `/login` para iniciar sesión con tu cuenta de claude.ai.

<h3 id="could-not-create-a-cloud-environment-or-no-cloud-environment-available-when-using-cloud-or-ultraplan">
  "No se pudo crear un entorno en la nube" o "No hay entorno en la nube disponible" al usar `--cloud` o ultraplan
</h3>

Las características de sesión remota crean un entorno en la nube predeterminado automáticamente si no tienes uno. Si ves "No se pudo crear un entorno en la nube", la creación automática falló. Si ves "No hay entorno en la nube disponible", tu CLI es anterior a la creación automática. En cualquier caso, ejecuta `/web-setup` en la CLI de Claude Code para crear uno manualmente, o visita [claude.ai/code](https://claude.ai/code) y sigue el paso **Crea tu entorno** anterior.

<h3 id="setup-script-failed">
  El script de configuración falló
</h3>

El script de configuración salió con un estado distinto de cero, lo que bloquea el inicio de la sesión. Las causas comunes son:

* Una instalación de paquete falló porque el registro no está en tu [nivel de acceso a la red](/docs/es/claude-code-on-the-web#access-levels). `Trusted` cubre la mayoría de los administradores de paquetes; `None` los bloquea todos.
* El script hace referencia a un archivo o ruta que no existe en un clon nuevo.
* Un comando que funciona localmente necesita una invocación diferente en Ubuntu.

Para depurar, agrega `set -x` en la parte superior del script para ver qué comando falló. Para comandos no críticos, agrega `|| true` para que no bloqueen el inicio de la sesión.

<h3 id="new-sessions-hang-or-time-out-during-setup">
  Las nuevas sesiones se cuelgan o agotan el tiempo de espera durante la configuración
</h3>

Si las nuevas sesiones se estancan en el paso del script de configuración o fallan con un error genérico del contenedor antes de que el script termine, el script probablemente está excediendo el presupuesto de tiempo de aproximadamente cinco minutos para construir el [caché del entorno](/docs/es/claude-code-on-the-web#environment-caching). Los pasos pesados como extraer imágenes grandes de Docker, sincronizar árboles de dependencias completos o descargar pesos de modelos a menudo empujan el total por encima del límite, especialmente cuando se ejecutan uno tras otro.

Para solucionar esto, recorta el script para que se complete de manera confiable en menos de cinco minutos:

* Ejecuta instalaciones independientes en paralelo con `&` y un `wait` final en lugar de ejecutarlas en serie.
* Mueve las descargas más grandes fuera del script de configuración y hacia un [hook SessionStart](/docs/es/claude-code-on-the-web#setup-scripts-vs-sessionstart-hooks) que las lance en segundo plano, para que la sesión sea utilizable mientras se completan.
* Elimina los reintentos de sueño largo del script de configuración, ya que un bucle de reintento estancado cuenta contra el presupuesto.

<h3 id="session-keeps-running-after-closing-the-tab">
  La sesión sigue ejecutándose después de cerrar la pestaña
</h3>

Esto es por diseño. Cerrar la pestaña o navegar lejos no detiene la sesión. Continúa ejecutándose en segundo plano hasta que Claude termine la tarea actual, luego se queda inactiva. Desde la barra lateral, puedes [archivar una sesión](/docs/es/claude-code-on-the-web#archive-sessions) para ocultarla de tu lista, o [eliminarla](/docs/es/claude-code-on-the-web#delete-sessions) para eliminarla permanentemente.

<h2 id="next-steps">
  Próximos pasos
</h2>

Ahora que puedes enviar y revisar tareas, estas páginas cubren lo que viene después: iniciar sesiones en la nube desde tu terminal, programar trabajo recurrente y dar a Claude instrucciones permanentes.

* [Usa Claude Code en la web](/docs/es/claude-code-on-the-web): la referencia completa, incluyendo teletransportar sesiones a tu terminal, scripts de configuración, variables de entorno y configuración de red
* [Routines](/docs/es/routines): automatiza el trabajo en un horario, mediante llamada API o en respuesta a eventos de GitHub
* [CLAUDE.md](/docs/es/memory): da a Claude instrucciones y contexto persistentes que se cargan al inicio de cada sesión
* Instala la aplicación móvil de Claude para [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) o [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) para monitorear sesiones desde tu teléfono. Desde la CLI de Claude Code, `/mobile` muestra un código QR.
