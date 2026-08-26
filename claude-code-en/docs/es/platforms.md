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
| [Web](/docs/es/claude-code-on-the-web) | Tareas de larga duración que no necesitan mucha dirección, o trabajo que debe continuar cuando está desconectado | Nube administrada por Anthropic, continúa después de desconectarse                                                                                                                                   |
| Móvil                             | Iniciar y monitorear tareas mientras está lejos de su computadora                                                | Sesiones en la nube desde la aplicación Claude para iOS y Android, [Remote Control](/docs/es/remote-control) para sesiones locales, [Dispatch](/docs/es/desktop#sessions-from-dispatch) a Desktop en Pro y Max |

La CLI es la superficie más completa para el trabajo nativo de terminal: scripting y el Agent SDK son solo CLI. Los proveedores de terceros también funcionan en [VS Code](/docs/es/vs-code#use-third-party-providers). Las implementaciones empresariales de [Desktop](/docs/es/desktop) admiten Google Cloud's Agent Platform, y Desktop admite [proveedores de puerta de enlace](/docs/es/llm-gateway-connect#desktop-app); para Amazon Bedrock o Microsoft Foundry, use la CLI o VS Code, o [Claude Desktop en 3P](https://claude.com/docs/third-party/claude-desktop/overview), que ejecuta la pestaña Code en esos proveedores. Desktop y las extensiones de IDE intercambian algunas características solo de CLI por revisión visual e integración más estrecha del editor. La web se ejecuta en la nube de Anthropic, por lo que las tareas continúan después de desconectarse. Móvil es un cliente delgado en esas mismas sesiones en la nube o en una sesión local a través de Remote Control, y puede enviar tareas a Desktop con Dispatch.

Puede mezclar superficies en el mismo proyecto. La configuración, la memoria del proyecto y los servidores MCP se comparten entre las superficies locales.

<h2 id="connect-your-tools">
  Conecte sus herramientas
</h2>

Las integraciones permiten que Claude trabaje con servicios fuera de su base de código.

| Integración                          | Qué hace                                         | Úselo para                                                                          |
| :----------------------------------- | :----------------------------------------------- | :---------------------------------------------------------------------------------- |
| [Chrome](/docs/es/chrome)                 | Controla su navegador con sus sesiones iniciadas | Prueba de aplicaciones web, rellenar formularios, automatizar sitios sin una API    |
| [GitHub Actions](/docs/es/github-actions) | Ejecuta Claude en su canalización de CI          | Revisiones automáticas de PR, clasificación de problemas, mantenimiento programado  |
| [GitLab CI/CD](/docs/es/gitlab-ci-cd)     | Lo mismo que GitHub Actions para GitLab          | Automatización impulsada por CI en GitLab                                           |
| [Code Review](/docs/es/code-review)       | Revisa automáticamente cada PR                   | Detectar errores antes de la revisión humana                                        |
| [Slack](/docs/es/slack)                   | Responde a menciones de `@Claude` en sus canales | Convertir informes de errores en solicitudes de extracción desde el chat del equipo |

Para integraciones no listadas aquí, [servidores MCP](/docs/es/mcp) y [conectores](/docs/es/desktop#connect-external-tools) le permiten conectar casi cualquier cosa: Linear, Notion, Google Drive o sus propias API internas.

<h2 id="work-when-you-are-away-from-your-terminal">
  Trabaje cuando está lejos de su terminal
</h2>

Claude Code offers several ways to work when you're not at your terminal. They differ in what triggers the work, where Claude runs, and how much you need to set up.

|                                                          | Trigger                                                                                        | Claude runs on                                                                               | Setup                                                                                                                                | Best for                                                      |
| :------------------------------------------------------- | :--------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------ |
| [Dispatch](/docs/en/desktop#sessions-from-dispatch)           | Message a task from the Claude mobile app                                                      | Your machine (Desktop)                                                                       | [Pair the mobile app with Desktop](https://support.claude.com/en/articles/13947068)                                                  | Delegating work while you're away, minimal setup              |
| [Remote Control](/docs/en/remote-control)                     | Drive a running session from [claude.ai/code](https://claude.ai/code) or the Claude mobile app | Your machine (CLI or VS Code)                                                                | Run `claude remote-control`                                                                                                          | Steering in-progress work from another device                 |
| [Channels](/docs/en/channels)                                 | Push events from a chat app like Telegram or Discord, or your own server                       | Your machine (CLI)                                                                           | [Install a channel plugin](/docs/en/channels#quickstart) or [build your own](/docs/en/channels-reference)                                      | Reacting to external events like CI failures or chat messages |
| [Slack](/docs/en/slack)                                       | Mention `@Claude` in a team channel                                                            | Anthropic cloud                                                                              | [Install the Slack app](/docs/en/slack#setting-up-claude-code-in-slack) with [Claude Code on the web](/docs/en/claude-code-on-the-web) enabled | PRs and reviews from team chat                                |
| [Self-hosted environments](/docs/en/self-hosted-environments) | Start a [cloud session](/docs/en/claude-code-on-the-web) and pick your organization's environment   | Your organization's infrastructure                                                           | [Deploy runners](/docs/en/self-hosted-environments-quickstart), on Team and Enterprise plans                                              | Cloud sessions that must run inside your network              |
| [Scheduled tasks](/docs/en/scheduled-tasks)                   | Set a schedule                                                                                 | [CLI](/docs/en/scheduled-tasks), [Desktop](/docs/en/desktop-scheduled-tasks), or [cloud](/docs/en/routines) | Pick a frequency                                                                                                                     | Recurring automation like daily reviews                       |

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
* [Claude Code en la web](/docs/es/claude-code-on-the-web): sesiones en la nube que continúan ejecutándose cuando se desconecta
* Móvil: la aplicación Claude para [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) y [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) para iniciar y monitorear tareas mientras está lejos de su computadora

<h3 id="integrations">
  Integraciones
</h3>

* [Chrome](/docs/es/chrome): automatice tareas del navegador con sus sesiones iniciadas
* [Uso de computadora](/docs/es/computer-use): permita que Claude abra aplicaciones y controle su pantalla en macOS
* [GitHub Actions](/docs/es/github-actions): ejecute Claude en su canalización de CI
* [GitLab CI/CD](/docs/es/gitlab-ci-cd): lo mismo para GitLab
* [Code Review](/docs/es/code-review): revisión automática en cada solicitud de extracción
* [Slack](/docs/es/slack): envíe tareas desde el chat del equipo, obtenga PR de vuelta

<h3 id="remote-access">
  Acceso remoto
</h3>

* [Dispatch](/docs/es/desktop#sessions-from-dispatch): envíe un mensaje con una tarea desde su teléfono y puede generar una sesión de Desktop
* [Remote Control](/docs/es/remote-control): controle una sesión en ejecución desde su teléfono o navegador
* [Channels](/docs/es/channels): envíe eventos desde aplicaciones de chat o sus propios servidores a una sesión
* [Scheduled tasks](/docs/es/scheduled-tasks): ejecute indicaciones en un horario recurrente
