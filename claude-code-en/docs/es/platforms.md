> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plataformas e integraciones

> Elija dónde ejecutar Claude Code y qué conectar. Compare la CLI, Desktop, VS Code, JetBrains, web, móvil e integraciones como Chrome, Slack e CI/CD.

Claude Code ejecuta el mismo motor subyacente en todas partes, pero cada superficie está optimizada para una forma diferente de trabajar. Esta página le ayuda a elegir la plataforma adecuada para su flujo de trabajo y conectar las herramientas que ya utiliza.

<h2 id="where-to-run-claude-code">
  Dónde ejecutar Claude Code
</h2>

Elija una plataforma según cómo le guste trabajar y dónde viva su proyecto.

| Plataforma                        | Mejor para                                                                                                       | Lo que obtiene                                                                                                                                                                                       |
| :-------------------------------- | :--------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [CLI](/docs/es/quickstart)             | Flujos de trabajo de terminal, scripting, servidores remotos                                                     | Conjunto completo de características, [Agent SDK](/docs/es/headless), [uso de computadora](/docs/es/computer-use) en macOS (Pro y Max), proveedores de terceros                                                |
| [Desktop](/docs/es/desktop)            | Revisión visual, sesiones paralelas, configuración administrada                                                  | Visor de diferencias, vista previa de aplicaciones, [uso de computadora](/docs/es/desktop#let-claude-use-your-computer) y [Dispatch](/docs/es/desktop#sessions-from-dispatch) en Pro y Max                     |
| [VS Code](/docs/es/vs-code)            | Trabajar dentro de VS Code sin cambiar a una terminal                                                            | Diferencias en línea, terminal integrada, contexto de archivo                                                                                                                                        |
| [JetBrains](/docs/es/jetbrains)        | Trabajar dentro de IntelliJ, PyCharm, WebStorm u otros IDE de JetBrains                                          | Visor de diferencias, intercambio de selección, sesión de terminal                                                                                                                                   |
| [Web](/docs/es/claude-code-on-the-web) | Tareas de larga duración que no necesitan mucha dirección, o trabajo que debe continuar cuando está desconectado | Nube, administrada por Anthropic de forma predeterminada; continúa después de desconectarse                                                                                                          |
| [Móvil](/docs/es/mobile)               | Iniciar y monitorear tareas mientras está lejos de su computadora                                                | Sesiones en la nube desde la aplicación Claude para iOS y Android, [Remote Control](/docs/es/remote-control) para sesiones locales, [Dispatch](/docs/es/desktop#sessions-from-dispatch) a Desktop en Pro y Max |

La CLI es la superficie más completa para el trabajo nativo de terminal: scripting y el Agent SDK son solo CLI. Los proveedores de terceros también funcionan en [VS Code](/docs/es/vs-code#use-third-party-providers) y en [JetBrains](/docs/es/feature-availability#features-available-on-every-provider), que ejecuta la CLI en la terminal de su IDE. Las implementaciones empresariales de [Desktop](/docs/es/desktop) admiten Google Cloud's Agent Platform, y Desktop admite [proveedores de puerta de enlace](/docs/es/llm-gateway-connect#desktop-app); para Amazon Bedrock o Microsoft Foundry, use la CLI o una extensión de IDE, o [Claude Desktop en 3P](https://claude.com/docs/third-party/claude-desktop/overview), que ejecuta la pestaña Code en esos proveedores. Desktop y las extensiones de IDE intercambian algunas características solo de CLI por revisión visual e integración más estrecha del editor. La web se ejecuta en la nube, por lo que las tareas continúan después de desconectarse. Móvil es un cliente delgado en esas mismas sesiones en la nube o en una sesión local a través de Remote Control, y puede enviar tareas a Desktop con Dispatch.

Puede mezclar superficies en el mismo proyecto. La configuración, la memoria del proyecto y los servidores MCP se comparten entre las superficies locales.

<h2 id="connect-your-tools">
  Conecte sus herramientas
</h2>

Las integraciones permiten que Claude trabaje con servicios fuera de su base de código.

| Integración                                      | Qué hace                                                                                                      | Úselo para                                                                                          |
| :----------------------------------------------- | :------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------- |
| [Chrome](/docs/es/chrome)                             | Controla su navegador con sus sesiones iniciadas                                                              | Prueba de aplicaciones web, rellenar formularios, automatizar sitios sin una API                    |
| [GitHub Actions](/docs/es/github-actions)             | Ejecuta Claude en su canalización de CI                                                                       | Revisiones automáticas de PR, clasificación de problemas, mantenimiento programado                  |
| [GitLab CI/CD](/docs/es/gitlab-ci-cd)                 | Lo mismo que GitHub Actions para GitLab                                                                       | Automatización impulsada por CI en GitLab                                                           |
| [Code Review](/docs/es/code-review)                   | Revisa automáticamente cada PR                                                                                | Detectar errores antes de la revisión humana                                                        |
| [Slack](/docs/es/slack)                               | Responde a menciones de `@Claude` en sus canales                                                              | Convertir informes de errores en solicitudes de extracción desde el chat del equipo                 |
| [Claude Tag](https://claude.com/docs/claude-tag) | Ejecuta `@Claude` como la identidad compartida de su organización con acceso configurado por el administrador | Acceso compartido del equipo en planes Team y Enterprise, en lugar de sesiones de Slack por usuario |

Para integraciones no listadas aquí, [servidores MCP](/docs/es/mcp) y [conectores](/docs/es/desktop#connect-external-tools) le permiten conectar casi cualquier cosa: Linear, Notion, Google Drive o sus propias API internas.

<h2 id="work-when-you-are-away-from-your-terminal">
  Trabaje cuando está lejos de su terminal
</h2>

Claude Code ofrece varias formas de trabajar cuando no está en su terminal. Difieren en lo que desencadena el trabajo, dónde se ejecuta Claude y cuánta configuración necesita.

|                                                         | Desencadenante                                                                                                  | Claude se ejecuta en                                                                       | Configuración                                                                                                                                  | Mejor para                                                         |
| :------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------- |
| [Dispatch](/docs/es/desktop#sessions-from-dispatch)          | Envíe un mensaje con una tarea desde la aplicación móvil de Claude                                              | Su máquina (Desktop)                                                                       | [Empareje la aplicación móvil con Desktop](https://support.claude.com/en/articles/13947068)                                                    | Delegar trabajo mientras está fuera, configuración mínima          |
| [Remote Control](/docs/es/remote-control)                    | Controle una sesión en ejecución desde [claude.ai/code](https://claude.ai/code) o la aplicación móvil de Claude | Su máquina (CLI o VS Code)                                                                 | Ejecute `claude remote-control`                                                                                                                | Dirigir el trabajo en progreso desde otro dispositivo              |
| [Channels](/docs/es/channels)                                | Envíe eventos desde una aplicación de chat como Telegram o Discord, o su propio servidor                        | Su máquina (CLI)                                                                           | [Instale un plugin de canal](/docs/es/channels#quickstart) o [cree el suyo propio](/docs/es/channels-reference)                                          | Reaccionar a eventos externos como fallos de CI o mensajes de chat |
| [Slack](/docs/es/slack)                                      | Mencione `@Claude` en un canal de equipo                                                                        | Nube de Anthropic                                                                          | [Instale la aplicación de Slack](/docs/es/slack#setting-up-claude-code-in-slack) con [Claude Code en la web](/docs/es/claude-code-on-the-web) habilitado | PRs y revisiones desde el chat del equipo                          |
| [Entornos autohospedados](/docs/es/self-hosted-environments) | Inicie una [sesión en la nube](/docs/es/claude-code-on-the-web) y seleccione el entorno de su organización           | La infraestructura de su organización                                                      | [Implemente ejecutores](/docs/es/self-hosted-environments-quickstart), en planes Team y Enterprise                                                  | Sesiones en la nube que deben ejecutarse dentro de su red          |
| [Tareas programadas](/docs/es/scheduled-tasks)               | Establezca una programación                                                                                     | [CLI](/docs/es/scheduled-tasks), [Desktop](/docs/es/desktop-scheduled-tasks), o [nube](/docs/es/routines) | Seleccione una frecuencia                                                                                                                      | Automatización recurrente como revisiones diarias                  |

Si no está seguro de por dónde empezar, [instale la CLI](/docs/es/quickstart) y ejecútela en un directorio de proyecto. Si prefiere no usar una terminal, [Desktop](/docs/es/desktop-quickstart) le proporciona el mismo motor con una interfaz gráfica.

<h2 id="related-resources">
  Recursos relacionados
</h2>

<h3 id="platforms">
  Plataformas
</h3>

* [Inicio rápido de CLI](/docs/es/quickstart): instale y ejecute su primer comando en la terminal
* [Desktop](/docs/es/desktop): revisión visual de diferencias, sesiones paralelas, uso de computadora y Dispatch
* [VS Code](/docs/es/vs-code): la extensión Claude Code dentro de su editor
* [JetBrains](/docs/es/jetbrains): la extensión para IntelliJ, PyCharm y otros IDE de JetBrains
* [Web](/docs/es/claude-code-on-the-web): sesiones en la nube desde su navegador en claude.ai/code que continúan ejecutándose cuando se desconecta
* [Proyectos](/docs/es/claude-projects): una conversación donde Claude coordina muchas sesiones en la nube para un cuerpo de trabajo e informa de vuelta
* [Móvil](/docs/es/mobile): la aplicación Claude para [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) y [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) para iniciar y monitorear tareas mientras está lejos de su computadora

<h3 id="integrations">
  Integraciones
</h3>

* [Chrome](/docs/es/chrome): automatice tareas del navegador con sus sesiones iniciadas
* [Uso de computadora](/docs/es/computer-use): permita que Claude abra aplicaciones y controle su pantalla en macOS
* [GitHub Actions](/docs/es/github-actions): ejecute Claude en su canalización de CI
* [GitLab CI/CD](/docs/es/gitlab-ci-cd): lo mismo para GitLab
* [Code Review](/docs/es/code-review): revisión automática en cada solicitud de extracción
* [Slack](/docs/es/slack): envíe tareas desde el chat del equipo, obtenga PR de vuelta
* [Claude Tag](https://claude.com/docs/claude-tag): ejecute `@Claude` como la identidad compartida de su organización en planes Team y Enterprise

<h3 id="remote-access">
  Acceso remoto
</h3>

* [Dispatch](/docs/es/desktop#sessions-from-dispatch): envíe un mensaje con una tarea desde su teléfono y puede generar una sesión de Desktop
* [Remote Control](/docs/es/remote-control): controle una sesión en ejecución desde su teléfono o navegador
* [Channels](/docs/es/channels): envíe eventos desde aplicaciones de chat o sus propios servidores a una sesión
* [Scheduled tasks](/docs/es/scheduled-tasks): ejecute indicaciones en un horario recurrente
