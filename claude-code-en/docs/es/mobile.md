> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code en dispositivos móviles

> Inicie, supervise y dirija tareas de Claude Code desde su teléfono con la aplicación Claude para iOS y Android.

La aplicación Claude para [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) y [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) es un cliente para sesiones de Claude Code en lugar de un lugar donde se ejecuta el código. Desde su teléfono accede a [sesiones en la nube](#start-and-monitor-cloud-sessions) y [proyectos](/docs/es/claude-projects) en la nube, una sesión que se ejecuta en su propia máquina a través de [Control Remoto](#continue-a-local-session-with-remote-control), o la aplicación de Escritorio a través de [Dispatch](/docs/es/desktop#sessions-from-dispatch).

<Note>
  Claude Code no tiene una aplicación móvil separada: las sesiones en la nube y Control Remoto se encuentran en la pestaña **Code** en la aplicación Claude, y Dispatch es una tarea a la que envía mensajes en la aplicación.
</Note>

<h2 id="get-the-app">
  Obtener la aplicación
</h2>

<Steps>
  <Step title="Descargar la aplicación Claude">
    Instale la aplicación Claude para [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) o [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude). En un iPad, instale la misma aplicación de iOS.

    <Tip>
      Ejecute `/mobile` en una sesión de Claude Code para mostrar un código QR para [claude.ai/mobile](https://claude.ai/mobile), que abre la tienda de aplicaciones correcta para su teléfono. `/ios` y `/android` hacen lo mismo.
    </Tip>
  </Step>

  <Step title="Iniciar sesión">
    Inicie sesión con la misma cuenta de claude.ai y organización que utiliza para Claude Code. Las sesiones en la nube y Control Remoto requieren una cuenta de claude.ai, por lo que no son accesibles con una clave API de Anthropic Console o desde un proveedor de terceros como Amazon Bedrock.
  </Step>

  <Step title="Abrir la pestaña Code">
    Toque **Code** en la navegación de la aplicación para acceder a sus sesiones, o abra [claude.ai/code/new](https://claude.ai/code/new) en su teléfono para iniciar una nueva sesión de Code en la aplicación. Si no ve la pestaña Code, su plan u organización puede no incluir estas características; consulte [disponibilidad por plan de suscripción](/docs/es/feature-availability#availability-by-subscription-plan).
  </Step>
</Steps>

<h2 id="work-from-your-phone">
  Trabajar desde su teléfono
</h2>

Desde la aplicación puede iniciar sesiones en la nube, abrir un proyecto, dirigir una sesión de Claude Code que se ejecuta en su computadora, o enviar un mensaje a Dispatch con una tarea. La aplicación es la misma para cada uno; difieren en dónde ocurre el trabajo.

| Característica                                 | A qué se conecta                                                                             | Cuándo usar                                                                                                                                                                    |
| :--------------------------------------------- | :------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Cloud sessions](/docs/es/claude-code-on-the-web)   | Una sesión en infraestructura en la nube, administrada por Anthropic de forma predeterminada | Su repositorio está en GitHub y la tarea debe continuar ejecutándose después de dejar su teléfono. Consulte el [inicio rápido en la nube](/docs/es/web-quickstart) para configurar. |
| [Projects](/docs/es/claude-projects)                | Una conversación donde Claude coordina sesiones en la nube paralelas como hilos              | Tiene un flujo de trabajo relacionado en lugar de una tarea y desea ver qué hilos finalizaron o lo necesitan.                                                                  |
| [Remote Control](/docs/es/remote-control)           | Una sesión de Claude Code que se ejecuta en su computadora                                   | El trabajo necesita su sistema de archivos local, herramientas o servidores MCP.                                                                                               |
| [Dispatch](/docs/es/desktop#sessions-from-dispatch) | La aplicación de Escritorio en su computadora                                                | Desea enviar un mensaje con una tarea y dejar que Dispatch decida cómo ejecutarla. Requiere un plan Pro o Max.                                                                 |

Si su computadora estará apagada, use cloud sessions o un proyecto, que se ejecutan en la nube y continúan con su portátil cerrado. Remote Control y Dispatch conducen su propia máquina, por lo que necesita mantenerse encendida con Claude Code o la aplicación de Escritorio en ejecución. Si su máquina entra en modo de suspensión durante una sesión de Remote Control, Claude Code se reconecta cuando la máquina vuelve a estar en línea.

Para una comparación más completa, consulte [trabajar cuando está lejos de su terminal](/docs/es/platforms#work-when-you-are-away-from-your-terminal).

Las cloud sessions y Remote Control se ejecutan desde la pestaña **Code**. Para Dispatch, que envía mensajes como una tarea en la aplicación, consulte [sesiones desde Dispatch](/docs/es/desktop#sessions-from-dispatch).

<h3 id="start-and-monitor-cloud-sessions">
  Iniciar y supervisar cloud sessions
</h3>

Las cloud sessions ejecutan tareas en infraestructura en la nube, administrada por Anthropic de forma predeterminada, por lo que una sesión continúa después de dejar su teléfono. Desde la pestaña Code, seleccione un repositorio y rama, describa la tarea y envíela. Las sesiones persisten en todos los dispositivos: una tarea que inicia en su portátil está lista para revisar desde su teléfono, y una que inicia desde su teléfono está esperando cuando regresa a su escritorio.

Abra una sesión en la aplicación para verificar el progreso, responder las preguntas de Claude o dirigirla en una nueva dirección. También puede indicarle a Claude que [observe una solicitud de extracción](/docs/es/claude-code-on-the-web#auto-fix-pull-requests) y corrija fallos de CI o revise comentarios a medida que llegan. Para conectar GitHub y configurar su entorno, siga el [inicio rápido en la nube](/docs/es/web-quickstart), y consulte [Use Claude Code in the cloud](/docs/es/claude-code-on-the-web) para todo lo que pueden hacer las cloud sessions.

<h3 id="continue-a-local-session-with-remote-control">
  Continuar una sesión local con Remote Control
</h3>

Remote Control conecta la aplicación Claude a una sesión de Claude Code que se ejecuta en su máquina, por lo que la ejecución de código y el acceso al sistema de archivos permanecen locales mientras dirige la sesión desde su teléfono. Inicie la sesión en su computadora con `claude remote-control`, o ejecute `/remote-control` en una sesión que ya está abierta. Luego escanee el código QR que la terminal puede mostrar, o abra la aplicación Claude, toque **Code**, y seleccione la sesión de la lista. Consulte [conectar desde otro dispositivo](/docs/es/remote-control#connect-from-another-device) para cada opción.

Cuando agrega un archivo adjunto en la aplicación Claude, también llega a la sesión local:

* **Fotos**: Claude ve las fotos adjuntas directamente como parte de su mensaje. Claude Code también guarda cada foto en `~/.claude/uploads/` y le dice a Claude la ruta del archivo guardado, por lo que Claude puede copiar la imagen en los archivos que crea.
* **Otros archivos**: Claude Code los descarga a su máquina y los pasa a Claude como referencias de archivo `@`.

Para requisitos, modos de invocación y solución de problemas, consulte la [descripción general de Remote Control](/docs/es/remote-control).

<h3 id="get-push-notifications">
  Obtener notificaciones push
</h3>

Cuando Remote Control está activo, Claude puede enviar notificaciones push a su teléfono, típicamente cuando una tarea de larga duración finaliza o cuando necesita una decisión de usted. También puede solicitar una en su indicación, como `notify me when the tests finish`. Consulte [notificaciones push móviles](/docs/es/remote-control#mobile-push-notifications) para los dos cambios `/config` y solución de problemas de entrega.

Dispatch envía su propia notificación cuando una sesión de Code que generó finaliza o necesita su aprobación, descrito en [sesiones desde Dispatch](/docs/es/desktop#sessions-from-dispatch).

<h2 id="limitations">
  Limitaciones
</h2>

El cliente móvil cubre la mayoría de lo que una sesión necesita, con algunas limitaciones:

* **Comandos solo locales**: comandos que solo se ejecutan en la interfaz de terminal, como `/plugin` y `/resume`, no funcionan desde la aplicación. Las [limitaciones de Control Remoto](/docs/es/remote-control#limitations) enumeran los comandos que funcionan desde dispositivos móviles y cómo difiere su comportamiento.
* **Modos de permisos**: las sesiones en la nube ofrecen Aceptar ediciones, Plan y Auto en el menú desplegable de modo, y las sesiones de Control Remoto ofrecen Manual, Aceptar ediciones y Plan. No puede seleccionar Bypass permissions desde la aplicación en ninguno de los casos, y no puede seleccionar Auto para una sesión de Control Remoto. Consulte [cambiar modos de permisos](/docs/es/permission-modes#switch-permission-modes).
* **Planes de Dispatch**: Dispatch requiere un plan Pro o Max y no está disponible en Team o Enterprise.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Plataformas e integraciones](/docs/es/platforms): compare todas las superficies en las que se ejecuta Claude Code
* [Claude Code en la web](/docs/es/claude-code-on-the-web): cómo se ejecutan las sesiones en la nube y cómo mover trabajo hacia y desde su terminal
* [Configurar entornos en la nube](/docs/es/cloud-environments): niveles de acceso de red, variables de entorno y scripts de configuración para sesiones en la nube
* [Control Remoto](/docs/es/remote-control): continuar una sesión local desde cualquier dispositivo
* [Sesiones desde Dispatch](/docs/es/desktop#sessions-from-dispatch): cómo las tareas de Dispatch se convierten en sesiones de Code en la aplicación de Escritorio
* [Channels](/docs/es/channels): pregunte algo a Claude desde su teléfono a través de Telegram, Discord o iMessage mientras el trabajo se ejecuta en su máquina
* [Claude Code en Slack](/docs/es/slack): delegue tareas de codificación desde su espacio de trabajo de Slack mencionando `@Claude`
