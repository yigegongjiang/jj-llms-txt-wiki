> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Comienza con Claude Code en la nube

> Ejecuta Claude Code en la nube desde tu navegador o teléfono. Conecta un repositorio de GitHub, envía una tarea y revisa el PR sin configuración local.

<Note>
  Las sesiones en la nube están disponibles en los planes Pro, Max y Team, y para usuarios Enterprise con asientos premium o asientos de Chat + Claude Code.
</Note>

Una sesión en la nube ejecuta Claude Code en la infraestructura en la nube en lugar de en tu máquina, administrada por Anthropic de forma predeterminada. Este inicio rápido inicia una desde [claude.ai/code](https://claude.ai/code) en tu navegador. También puedes iniciar una desde la aplicación móvil de Claude, la aplicación de Desktop, o tu terminal con `claude --cloud`.

Necesitarás un repositorio de GitHub para [comenzar](#connect-github). Claude lo clona en una máquina virtual aislada, realiza cambios e impulsa una rama para que la revises. Las sesiones persisten entre dispositivos, por lo que una tarea que comiences en tu portátil está lista para revisar desde tu teléfono más tarde.

Las sesiones en la nube funcionan bien para:

* **Tareas paralelas**: ejecuta varias tareas independientes a la vez, cada una en su propia sesión y rama, sin necesidad de gestionar múltiples worktrees
* **Repositorios que no tienes localmente**: Claude clona el repositorio nuevo en cada sesión, por lo que no necesitas tenerlo descargado
* **Tareas que no necesitan dirección frecuente**: envía una tarea bien definida, haz otra cosa y revisa el resultado cuando Claude haya terminado
* **Preguntas sobre código y exploración**: comprende una base de código o rastrea cómo se implementa una función sin una descarga local

Para trabajos que necesitan tu configuración local, herramientas o entorno, ejecutar Claude Code localmente o usar [Remote Control](/docs/es/remote-control) es una mejor opción.

<h2 id="how-sessions-run">
  Cómo se ejecutan las sesiones
</h2>

Los pasos a continuación describen sesiones alojadas en Anthropic. En un [entorno autohospedado](/docs/es/self-hosted-environments), el clon y todo lo que sigue se ejecutan en los ejecutores propios de tu organización, donde los límites de red, la configuración y el comportamiento de inserción están configurados por el operador. Cuando envías una tarea:

1. **Clonar y preparar**: tu repositorio se clona en una VM administrada por Anthropic, y tu [script de configuración](/docs/es/cloud-environments#setup-scripts) se ejecuta si está configurado.
2. **Configurar red**: el acceso a internet se establece según el [nivel de acceso](/docs/es/cloud-environments#access-levels) de tu entorno.
3. **Trabajar**: Claude analiza el código, realiza cambios, ejecuta pruebas y verifica su trabajo. Puedes observar y dirigir en todo momento, o alejarte y volver cuando haya terminado.
4. **Impulsar la rama**: cuando Claude alcanza un punto de parada, impulsa su rama a GitHub. Revisa el diff, deja comentarios en línea, crea un PR o envía otro mensaje para continuar.

La sesión no se cierra cuando se impulsa la rama. La creación de PR y ediciones adicionales ocurren dentro de la misma conversación.

<h2 id="compare-ways-to-run-claude-code">
  Compara formas de ejecutar Claude Code
</h2>

Claude Code se comporta igual en todas partes. Lo que cambia es dónde se ejecuta la sesión y si tu configuración local está disponible:

|                                              | Sesión en la nube                                                                                                         | Sesión local                                                                                                                                          | Sesión local con [Remote Control](/docs/es/remote-control)                   |
| :------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------- |
| **El código se ejecuta en**                  | VM en la nube, administrada por Anthropic de forma predeterminada                                                         | Tu máquina                                                                                                                                            | Tu máquina                                                              |
| **La inicias desde**                         | claude.ai/code, la aplicación móvil de Claude, la aplicación Desktop con **Cloud** seleccionado, o `claude --cloud`       | Tu terminal, tu IDE, o la aplicación Desktop con **Local** seleccionado                                                                               | Tu terminal, la extensión de VS Code, o la aplicación Desktop           |
| **Chateas desde**                            | claude.ai, la aplicación móvil, o la aplicación Desktop                                                                   | Donde la iniciaste                                                                                                                                    | claude.ai o la aplicación móvil, así como donde la iniciaste            |
| **Usa tu configuración local**               | No, solo repositorio                                                                                                      | Sí                                                                                                                                                    | Sí                                                                      |
| **Requiere GitHub**                          | Sí, o [agrupa un repositorio local](/docs/es/claude-code-on-the-web#send-local-repositories-without-github) mediante `--cloud` | No                                                                                                                                                    | No                                                                      |
| **Sigue ejecutándose si te desconectas**     | Sí                                                                                                                        | No                                                                                                                                                    | Mientras la sesión permanezca abierta en tu máquina                     |
| **[Modos de permiso](/docs/es/permission-modes)** | Aceptar ediciones, Plan, Auto                                                                                             | Todos los modos en la terminal; consulta [Cambiar modos de permiso](/docs/es/permission-modes#switch-permission-modes) para la IDE y la aplicación Desktop | Manual, Aceptar ediciones, o Plan desde claude.ai y la aplicación móvil |
| **Acceso a la red**                          | Configurable por entorno                                                                                                  | Red de tu máquina                                                                                                                                     | Red de tu máquina                                                       |

Consulta los documentos de [inicio rápido de terminal](/docs/es/quickstart), [aplicación Desktop](/docs/es/desktop), o [Remote Control](/docs/es/remote-control) para configurar sesiones locales.

<h2 id="connect-github">
  Conecta GitHub
</h2>

Conectar GitHub es un paso único. Si ya usas la CLI de GitHub, puedes [hacer esto desde tu terminal](#connect-from-your-terminal) en lugar del navegador.

<Note>
  En planes Team y Enterprise, el paso **Inicia sesión con GitHub** funciona solo después de que un [Propietario](/docs/es/server-managed-settings#access-control) de tu organización Claude active el conector de GitHub en [**Configuración de administrador > Conectores**](https://claude.ai/admin-settings/connectors). Hasta entonces, ese paso muestra "Se requiere acceso a GitHub para Claude Code en la web" en lugar de un botón de inicio de sesión. Después de que el conector esté activado, recarga [claude.ai/code](https://claude.ai/code) y comienza de nuevo desde el primer paso. Un segundo interruptor, [Configuración rápida de web](/docs/es/claude-code-on-the-web#github-authentication-options) en [**Configuración de administrador > Claude Code**](https://claude.ai/admin-settings/claude-code), es opcional: con él activado, `/web-setup` funciona e incorporación crea el entorno para los miembros.
</Note>

<Steps>
  <Step title="Visita claude.ai/code">
    Ve a [claude.ai/code](https://claude.ai/code) e inicia sesión con tu cuenta de claude.ai.
  </Step>

  <Step title="Inicia sesión con GitHub">
    Después de iniciar sesión, claude.ai/code te solicita que conectes GitHub. Sigue el mensaje, y claude.ai/code te envía a la página de autorización de GitHub. Aprueba la solicitud de autorización, y GitHub te devuelve a claude.ai/code. Las sesiones en la nube funcionan con repositorios de GitHub existentes. Para iniciar un nuevo proyecto, [crea un repositorio vacío en GitHub](https://github.com/new) primero.

    Con esta conexión, una sesión puede clonar cualquier repositorio público, pero puede trabajar en un repositorio privado solo cuando la aplicación Claude GitHub está instalada en él. [Instala la aplicación](https://github.com/apps/claude/installations/new) en cada cuenta de GitHub u organización cuyos repositorios privados desees usar. En una organización de GitHub, es posible que el propietario de la organización deba aprobar la instalación. Instalar la aplicación también habilita [Auto-fix](/docs/es/claude-code-on-the-web#auto-fix-pull-requests), que permite a Claude responder a fallos de CI y comentarios de revisión en solicitudes de extracción en esos repositorios.

    Si la incorporación te solicita instalar la aplicación en este punto y prefieres hacerlo más tarde, haz clic en **Omitir**.
  </Step>

  <Step title="Configura tu entorno predeterminado">
    Un [entorno en la nube](/docs/es/cloud-environments) es la configuración guardada que controla qué acceso a la red tiene Claude durante las sesiones y qué se ejecuta cuando se inicia una sesión. Lo que sucede después de conectar GitHub depende de tu plan:

    * **Pro y Max**: la incorporación crea un entorno llamado **Default** para ti.
    * **Team y Enterprise**: la incorporación muestra un formulario **Crea tu primer entorno en la nube**. Deja el nombre rellenado previamente y el acceso a la red sin cambios y haz clic en **Crear y finalizar** para crear el entorno **Default**. Si un Propietario ha activado [Configuración rápida de web](/docs/es/claude-code-on-the-web#github-authentication-options), la incorporación crea **Default** para ti en su lugar.

    **Default** utiliza [acceso a red `Trusted`](/docs/es/cloud-environments#access-levels): las sesiones alcanzan [registros de paquetes comunes](/docs/es/cloud-environments#default-allowed-domains) y otros dominios en la lista blanca, y nada más a través de la red de la sesión. Consulta [Herramientas instaladas](/docs/es/cloud-environments#installed-tools) para ver qué está disponible sin ninguna configuración.

    Para un primer proyecto, el entorno **Default** funciona tal como está. Para cambiar su acceso a la red, agregar variables de entorno o ejecutar un [script de configuración](/docs/es/cloud-environments#setup-scripts) antes de que se inicien las sesiones, [edítalo o crea entornos adicionales](/docs/es/cloud-environments#configure-your-environment).
  </Step>
</Steps>

<h3 id="connect-from-your-terminal">
  Conecta desde tu terminal
</h3>

Si ya usas la CLI de GitHub (`gh`), puedes conectar GitHub para sesiones en la nube desde tu terminal. Esto requiere la [CLI de Claude Code](/docs/es/quickstart). En planes Team y Enterprise, `/web-setup` está disponible solo después de que un Propietario active [Configuración rápida de web](/docs/es/claude-code-on-the-web#github-authentication-options).

Cuando ejecutas `/web-setup`, Claude Code lee el token que `gh auth token` imprime, te pide que confirmes y envía el token a Anthropic. Anthropic lo almacena cifrado con tu cuenta de claude.ai, y tus sesiones en la nube lo usan para acceso a GitHub hasta que [lo elimines](#remove-the-web-setup-token). Una sesión en la nube que inicies tú mismo puede entonces acceder a cualquier repositorio que ese token pueda acceder, sin instalación de la aplicación Claude GitHub. Los hilos en un [proyecto](/docs/es/claude-projects#set-up-github-access) aún necesitan la aplicación.

Si ya conectaste GitHub en el navegador, `/web-setup` te advierte que continuar reemplaza esa conexión para tus sesiones en la nube.

<Note>
  Las organizaciones con [Retención de datos cero](/docs/es/zero-data-retention) habilitada no pueden usar `/web-setup` u otras características de sesión en la nube. Si la CLI de GitHub no está instalada o no está autenticada, Claude Code abre el flujo de incorporación del navegador en su lugar.
</Note>

<Steps>
  <Step title="Autentica con la CLI de GitHub">
    En tu shell, autentica la CLI de GitHub si aún no lo has hecho:

    ```bash theme={null}
    gh auth login
    ```
  </Step>

  <Step title="Inicia sesión en Claude">
    En la CLI de Claude Code, ejecuta `/login` para iniciar sesión con tu cuenta de claude.ai. Omite este paso si ya has iniciado sesión con una cuenta de claude.ai. Autenticarse con una clave de API no cuenta. Para verificar, ejecuta `/status` y confirma que la fila **Método de inicio de sesión** muestra una cuenta de claude.ai.
  </Step>

  <Step title="Ejecuta /web-setup">
    En la CLI de Claude Code, ejecuta:

    ```text theme={null}
    /web-setup
    ```

    Confirma el mensaje para enviar tu token de `gh` a tu cuenta de Claude. Si tiene éxito, Claude Code imprime `Connected as <tu-nombre-de-usuario-github>` y abre [claude.ai/code](https://claude.ai/code) en tu navegador. Si aún no tienes un entorno en la nube, `/web-setup` crea uno con acceso a red Trusted y sin script de configuración. Puedes [editar el entorno o agregar variables](/docs/es/cloud-environments#configure-your-environment) después. Una vez que `/web-setup` se complete, puedes iniciar sesiones en la nube desde tu terminal con [`--cloud`](/docs/es/claude-code-on-the-web#from-terminal-to-cloud) o configurar tareas recurrentes con [`/schedule`](/docs/es/routines).
  </Step>
</Steps>

<h4 id="remove-the-web-setup-token">
  Elimina el token de `/web-setup`
</h4>

Para eliminar el token de tu cuenta de Claude, desconecta GitHub en [claude.ai/customize/connectors](https://claude.ai/customize/connectors). Desconectar elimina las credenciales de GitHub que tus sesiones en la nube usan, ya sea que provengan del navegador o de `/web-setup`, por lo que las sesiones en la nube pierden acceso a GitHub hasta que conectes de nuevo. Tu `gh` local permanece conectado, y el token sigue siendo válido en GitHub.

Para invalidar el token en sí, revócalo en GitHub. Si iniciaste sesión en `gh` a través del navegador, el token pertenece a la entrada **GitHub CLI** en [**Configuración > Aplicaciones > Aplicaciones OAuth autorizadas**](https://github.com/settings/applications) en GitHub, y revocar esa entrada también cierra la sesión de la CLI de GitHub en tus máquinas. Las sesiones en la nube entonces pierden acceso a GitHub hasta que ejecutes `gh auth login` y `/web-setup` de nuevo.

<h2 id="start-a-task">
  Inicia una tarea
</h2>

Con GitHub conectado y un entorno creado, estás listo para enviar tareas.

<Steps>
  <Step title="Selecciona un repositorio y rama">
    Desde [claude.ai/code](https://claude.ai/code) o la pestaña Code en la aplicación móvil de Claude, haz clic en el selector de repositorio debajo del cuadro de entrada y elige un repositorio en el que Claude pueda trabajar. Cada repositorio muestra un selector de rama. Cámbialo para que Claude comience desde una rama de función en lugar de la predeterminada. Puedes agregar múltiples repositorios para trabajar en ellos en una sesión.
  </Step>

  <Step title="Elige un modo de permiso">
    El menú desplegable de modo junto a la entrada muestra el modo en el que se ejecutará la sesión:

    * **Auto**: un clasificador revisa las acciones de Claude en lugar de pedirte. Aparece cuando tu organización permite el modo auto y el modelo seleccionado lo admite
    * **Aceptar ediciones**: Claude realiza cambios e impulsa una rama sin detenerse para aprobación
    * **Plan**: Claude propone un enfoque y espera tu aprobación antes de editar archivos

    Las sesiones en la nube no ofrecen permisos Manual o Bypass. Consulta la [lista completa de modos de permiso](/docs/es/permission-modes#available-modes) para ver qué permite cada uno.
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
| `environment`  | Nombre o ID del [entorno](#connect-github) para preseleccionar.                                                                                                                                                         |

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

    El diff compara los cambios de la sesión contra su rama base de forma predeterminada. Para comparar contra una rama diferente, selecciona **Comparar contra** y elige una.
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

Si conectó GitHub en el navegador, las sesiones pueden clonar cualquier repositorio público, pero un repositorio privado aparece solo cuando la aplicación Claude GitHub está instalada en la cuenta u organización que lo posee y el acceso a repositorios de la instalación lo incluye. [Instale la aplicación Claude GitHub](https://github.com/apps/claude/installations/new) allí, o pida a un propietario de la organización que la instale o la apruebe.

Si conectó con `/web-setup`, las sesiones alcanzan cada repositorio que su token `gh` puede acceder. Ejecute `gh repo view OWNER/REPO` en su shell para verificar que su inicio de sesión de CLI de GitHub puede ver el repositorio, y ejecute `/web-setup` nuevamente si ha cambiado de cuentas `gh` desde que se conectó.

<h3 id="the-page-only-shows-a-github-login-button">
  La página solo muestra un botón de inicio de sesión de GitHub
</h3>

Las sesiones en la nube requieren una cuenta de GitHub conectada. Conecte a través del flujo del navegador anterior, o ejecute `/web-setup` desde su terminal si usa la CLI de GitHub. Si prefiere no conectar GitHub en absoluto, consulte [Remote Control](/docs/es/remote-control) para ejecutar Claude Code en su propia máquina y monitorearlo desde su navegador o teléfono.

<h3 id="not-available-for-the-selected-organization">
  "No disponible para la organización seleccionada"
</h3>

Las organizaciones Enterprise pueden necesitar que un propietario habilite sesiones en la nube. Contacte a su equipo de cuenta de Anthropic.

<h3 id="/web-setup-says-not-signed-in-to-claude">
  `/web-setup` dice "Not signed in to Claude"
</h3>

Si `/web-setup` responde con "Not signed in to Claude. Run /login first.", la CLI no tiene un inicio de sesión válido en claude.ai. Esto también puede ocurrir cuando un inicio de sesión anterior ha expirado. Ejecute `/login`, inicie sesión con su cuenta de claude.ai, luego ejecute `/web-setup` nuevamente.

<h3 id="/web-setup-warns-that-your-token-doesn’t-have-the-workflow-scope">
  `/web-setup` advierte que su token no tiene el alcance `workflow`
</h3>

Si `/web-setup` dice que su token de CLI de GitHub no tiene el alcance `workflow`, puede continuar, pero GitHub puede rechazar algunos pushes realizados con ese token, como los pushes que cambian archivos de flujo de trabajo de GitHub Actions. Para agregar el alcance, ejecute `gh auth refresh -s workflow` en su shell, luego ejecute `/web-setup` nuevamente.

<h3 id="web-setup-shows-no-commands-match-or-unknown-command">
  `/web-setup` muestra "No commands match" o "Unknown command"
</h3>

`/web-setup` se ejecuta dentro de la CLI de Claude Code, no en su shell. Inicie `claude` primero, luego escriba `/web-setup` en el mensaje.

Si lo escribió dentro de Claude Code y el menú de comandos muestra `No commands match "/web-setup"`, o enviarlo devuelve `Unknown command: /web-setup`, el comando está oculto porque no se cumple un requisito. La causa generalmente es que está autenticado con una clave API o proveedor de terceros en lugar de una suscripción de claude.ai. Ejecute `/login` para iniciar sesión con su cuenta de claude.ai.

En planes Team y Enterprise, el comando está oculto de forma predeterminada: el [interruptor de configuración rápida de web](/docs/es/claude-code-on-the-web#github-authentication-options) está desactivado hasta que un propietario lo active. Mientras esté desactivado, [conecte GitHub desde el navegador](#connect-github) en su lugar.

El comando también está oculto en otros dos casos:

* Un administrador ha deshabilitado sesiones en la nube para su organización. En este caso, enviar `/web-setup` devuelve [`Cloud sessions are disabled by your organization's policy`](/docs/es/errors#cloud-sessions-are-disabled-by-your-organizations-policy). Antes de v2.1.268, este caso también devolvía `Unknown command: /web-setup`.
* Su organización Enterprise tiene [Zero Data Retention](/docs/es/zero-data-retention) habilitado, lo que hace que las sesiones en la nube no estén disponibles.

<h3 id="could-not-create-a-cloud-environment-or-no-cloud-environment-available-when-using-cloud">
  "No se pudo crear un entorno en la nube" o "No hay entorno en la nube disponible" al usar `--cloud`
</h3>

Las características de sesión en la nube crean un entorno en la nube predeterminado automáticamente si no tiene uno. Si ve "No se pudo crear un entorno en la nube", la creación automática falló. Si ve "No hay entorno en la nube disponible", su CLI es anterior a la creación automática. En cualquier caso, ejecute `/web-setup` en la CLI de Claude Code, o agregue un entorno desde el [selector de entorno](/docs/es/cloud-environments#configure-your-environment) en [claude.ai/code](https://claude.ai/code).

<h3 id="setup-script-failed">
  El script de configuración falló
</h3>

El script de configuración salió con un estado distinto de cero, lo que bloquea el inicio de la sesión. Las causas comunes son:

* Una instalación de paquete falló porque el registro no está en su [nivel de acceso a la red](/docs/es/cloud-environments#access-levels). `Trusted` cubre la mayoría de los administradores de paquetes; `None` los bloquea todos.
* El script hace referencia a un archivo o ruta que no existe en un clon nuevo.
* Un comando que funciona localmente necesita una invocación diferente en Ubuntu.

Para depurar, agregue `set -x` en la parte superior del script para ver qué comando falló. Para comandos no críticos, agregue `|| true` para que no bloqueen el inicio de la sesión.

<h3 id="new-sessions-hang-or-time-out-during-setup">
  Las nuevas sesiones se cuelgan o agotan el tiempo de espera durante la configuración
</h3>

Si las nuevas sesiones se estancan en el paso del script de configuración o fallan con un error genérico del contenedor antes de que el script termine, el script probablemente está excediendo el presupuesto de tiempo de aproximadamente cinco minutos para construir el [caché del entorno](/docs/es/cloud-environments#environment-caching). Los pasos pesados como extraer imágenes grandes de Docker, sincronizar árboles de dependencias completos o descargar pesos de modelos a menudo empujan el total por encima del límite, especialmente cuando se ejecutan uno tras otro.

Para solucionar esto, recorte el script para que se complete de manera confiable en menos de cinco minutos:

* Ejecute instalaciones independientes en paralelo con `&` y un `wait` final en lugar de ejecutarlas en serie.
* Mueva las descargas más grandes fuera del script de configuración y hacia un [hook SessionStart](/docs/es/cloud-environments#setup-scripts-vs-sessionstart-hooks) que las lance en segundo plano, para que la sesión sea utilizable mientras se completan.
* Elimine los reintentos de sueño largo del script de configuración, ya que un bucle de reintento estancado cuenta contra el presupuesto.

<h3 id="session-keeps-running-after-closing-the-tab">
  La sesión sigue ejecutándose después de cerrar la pestaña
</h3>

Esto es por diseño. Cerrar la pestaña o navegar lejos no detiene la sesión. Continúa ejecutándose en segundo plano hasta que Claude termine la tarea actual, luego se queda inactiva. Desde la barra lateral, puede [archivar una sesión](/docs/es/claude-code-on-the-web#archive-sessions) para ocultarla de su lista, o [eliminarla](/docs/es/claude-code-on-the-web#delete-sessions) para eliminarla permanentemente.

<h2 id="next-steps">
  Próximos pasos
</h2>

Ahora que puedes enviar y revisar tareas, estas páginas cubren lo que viene después: iniciar sesiones en la nube desde tu terminal, programar trabajo recurrente y dar a Claude instrucciones permanentes.

* [Usa Claude Code en la web](/docs/es/claude-code-on-the-web): la referencia completa, incluyendo teletransportar sesiones a tu terminal, compartir sesiones y corregir automáticamente solicitudes de extracción
* [Configura entornos en la nube](/docs/es/cloud-environments): niveles de acceso de red, variables de entorno y scripts de configuración para sesiones en la nube
* [Routines](/docs/es/routines): automatiza el trabajo en un horario, mediante llamada API o en respuesta a eventos de GitHub
* [CLAUDE.md](/docs/es/memory): da a Claude instrucciones y contexto persistentes que se cargan al inicio de cada sesión
* Instala la aplicación móvil de Claude para [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) o [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) para monitorear sesiones desde tu teléfono. Desde la CLI de Claude Code, `/mobile` muestra un código QR para [claude.ai/mobile](https://claude.ai/mobile) que abre la tienda de aplicaciones correcta para tu teléfono.
