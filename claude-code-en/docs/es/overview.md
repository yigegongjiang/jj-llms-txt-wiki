> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Descripción general

> Claude Code es una herramienta de codificación agencial que lee tu base de código, edita archivos, ejecuta comandos e integra con tus herramientas de desarrollo. Disponible en tu terminal, IDE, aplicación de escritorio y navegador.

Claude Code es un asistente de codificación impulsado por IA que te ayuda a construir características, corregir errores y automatizar tareas de desarrollo. Entiende tu base de código completa y puede trabajar en múltiples archivos y herramientas para lograr las cosas.

<h2 id="get-started">
  Comenzar
</h2>

Claude Code se ejecuta en varias superficies: la terminal, extensiones de IDE, una aplicación de escritorio y la web. Elige una de las pestañas a continuación para comenzar. La mayoría de las superficies requieren una [suscripción a Claude](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=overview_pricing) o una cuenta de [Anthropic Console](https://console.anthropic.com/). La CLI de Terminal y VS Code también admiten [proveedores de terceros](/docs/es/third-party-integrations).

<Tabs>
  <Tab title="Terminal">
    La CLI completa para trabajar con Claude Code directamente en tu terminal. Edita archivos, ejecuta comandos y gestiona tu proyecto completo desde la línea de comandos.

    To install Claude Code, use one of the following methods:

    <Tabs>
      <Tab title="Native Install (Recommended)">
        **macOS, Linux, WSL:**

        ```bash theme={null}
        curl -fsSL https://claude.ai/install.sh | bash
        ```

        **Windows PowerShell:**

        ```powershell theme={null}
        irm https://claude.ai/install.ps1 | iex
        ```

        **Windows CMD:**

        ```batch theme={null}
        curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
        ```

        If you see `The token '&&' is not a valid statement separator`, you're in PowerShell, not CMD. If you see `'irm' is not recognized as an internal or external command`, you're in CMD, not PowerShell. Your prompt shows `PS C:\` when you're in PowerShell and `C:\` without the `PS` when you're in CMD.

        If the install command fails with `syntax error near unexpected token '<'`, a `403`, or another curl error, see [Troubleshoot installation](/docs/en/troubleshoot-install#find-your-error) to match the error to a fix and for alternative install methods.

        [Git for Windows](https://git-scm.com/downloads/win) is recommended on native Windows so Claude Code can use the Bash tool. If Git for Windows is not installed, Claude Code uses PowerShell as the shell tool instead. WSL setups do not need Git for Windows.

        <Info>
          Native installations automatically update in the background to keep you on the latest version.
        </Info>
      </Tab>

      <Tab title="Homebrew">
        ```bash theme={null}
        brew install --cask claude-code
        ```

        Homebrew offers two casks. `claude-code` tracks the stable release channel, which is typically about a week behind and skips releases with major regressions. `claude-code@latest` tracks the latest channel and receives new versions as soon as they ship.

        <Info>
          Homebrew installations do not auto-update. Run `brew upgrade claude-code` or `brew upgrade claude-code@latest`, depending on which cask you installed, to get the latest features and security fixes.
        </Info>
      </Tab>

      <Tab title="WinGet">
        ```powershell theme={null}
        winget install Anthropic.ClaudeCode
        ```

        <Info>
          WinGet installations do not auto-update. Run `winget upgrade Anthropic.ClaudeCode` periodically to get the latest features and security fixes.
        </Info>
      </Tab>
    </Tabs>

    You can also install with [apt, dnf, or apk](/docs/en/setup#install-with-linux-package-managers) on Debian, Fedora, RHEL, and Alpine.

    Luego inicia Claude Code en cualquier proyecto:

    ```bash theme={null}
    cd your-project
    claude
    ```

    Se te pedirá que inicies sesión en el primer uso. ¡Eso es todo! [Continúa con la Guía de inicio rápido →](/docs/es/quickstart)

    <Tip>
      Consulta [configuración avanzada](/docs/es/setup) para opciones de instalación, actualizaciones manuales o instrucciones de desinstalación. Visita [solución de problemas de instalación](/docs/es/troubleshoot-install) si encuentras problemas.
    </Tip>
  </Tab>

  <Tab title="VS Code">
    La extensión de VS Code proporciona diffs en línea, menciones @, revisión de planes e historial de conversación directamente en tu editor.

    * [Instalar para VS Code](vscode:extension/anthropic.claude-code)
    * [Instalar para Cursor](cursor:extension/anthropic.claude-code)

    O busca "Claude Code" en la vista de Extensiones (`Cmd+Shift+X` en Mac, `Ctrl+Shift+X` en Windows/Linux). Después de instalar, abre la Paleta de comandos (`Cmd+Shift+P` / `Ctrl+Shift+P`), escribe "Claude Code" y selecciona **Abrir en Nueva Pestaña**.

    [Comenzar con VS Code →](/docs/es/vs-code#get-started)
  </Tab>

  <Tab title="Aplicación de escritorio">
    Una aplicación independiente para ejecutar Claude Code fuera de tu IDE o terminal. Revisa diffs visualmente, ejecuta múltiples sesiones lado a lado, programa tareas recurrentes e inicia sesiones en la nube.

    Descarga e instala:

    * [macOS](https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code\&utm_medium=docs) (Intel y Apple Silicon)
    * [Windows](https://claude.ai/api/desktop/win32/x64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs) (x64)
    * [Windows ARM64](https://claude.ai/api/desktop/win32/arm64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs)

    Después de instalar, lanza Claude, inicia sesión y haz clic en la pestaña **Code** para comenzar a codificar. Se requiere una [suscripción de pago](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=overview_desktop_pricing).

    [Obtén más información sobre la aplicación de escritorio →](/docs/es/desktop-quickstart)
  </Tab>

  <Tab title="Web">
    Ejecuta Claude Code en tu navegador sin configuración local. Inicia tareas de larga duración y vuelve cuando estén listas, trabaja en repositorios que no tienes localmente o ejecuta múltiples tareas en paralelo. Disponible en navegadores de escritorio y la aplicación Claude iOS.

    Comienza a codificar en [claude.ai/code](https://claude.ai/code).

    [Comenzar en la web →](/docs/es/web-quickstart)
  </Tab>

  <Tab title="JetBrains">
    Un plugin para IntelliJ IDEA, PyCharm, WebStorm y otros IDEs de JetBrains con visualización de diff interactiva y compartición de contexto de selección.

    Instala el [plugin Claude Code](https://plugins.jetbrains.com/plugin/27310-claude-code-beta-) desde el Marketplace de JetBrains y reinicia tu IDE. El plugin requiere la CLI de Claude Code, instalada por separado; consulta los [pasos de configuración de JetBrains](/docs/es/jetbrains#installation).

    [Comenzar con JetBrains →](/docs/es/jetbrains)
  </Tab>
</Tabs>

<h2 id="what-you-can-do">
  Lo que puedes hacer
</h2>

Aquí hay algunas de las formas en que puedes usar Claude Code:

<AccordionGroup>
  <Accordion title="Automatiza el trabajo que sigues posponiendo" icon="wand-magic-sparkles">
    Claude Code maneja las tareas tediosas que consumen tu día: escribir pruebas para código sin probar, corregir errores de lint en un proyecto, resolver conflictos de fusión, actualizar dependencias y escribir notas de lanzamiento.

    ```bash theme={null}
    claude "write tests for the auth module, run them, and fix any failures"
    ```
  </Accordion>

  <Accordion title="Construye características y corrige errores" icon="hammer">
    Describe lo que quieres en lenguaje natural. Claude Code planifica el enfoque, escribe el código en múltiples archivos y verifica que funcione.

    Para errores, pega un mensaje de error o describe el síntoma. Claude Code rastrea el problema a través de tu base de código, identifica la causa raíz e implementa una corrección. Consulta [flujos de trabajo comunes](/docs/es/common-workflows) para más ejemplos.
  </Accordion>

  <Accordion title="Crea commits y solicitudes de extracción" icon="code-branch">
    Claude Code funciona directamente con git. Prepara cambios, escribe mensajes de commit, crea ramas y abre solicitudes de extracción.

    ```bash theme={null}
    claude "commit my changes with a descriptive message"
    ```

    En CI, puedes automatizar la revisión de código y la clasificación de problemas con [GitHub Actions](/docs/es/github-actions) o [GitLab CI/CD](/docs/es/gitlab-ci-cd).
  </Accordion>

  <Accordion title="Conecta tus herramientas con MCP" icon="plug">
    El [Protocolo de Contexto de Modelo (MCP)](/docs/es/mcp) es un estándar abierto para conectar herramientas de IA a fuentes de datos externas. Con MCP, Claude Code puede leer tus documentos de diseño en Google Drive, actualizar tickets en Jira, extraer datos de Slack o usar tu propia herramienta personalizada. El [inicio rápido de MCP](/docs/es/mcp-quickstart) conecta tu primer servidor de extremo a extremo.
  </Accordion>

  <Accordion title="Personaliza con instrucciones, skills y hooks" icon="sliders">
    [`CLAUDE.md`](/docs/es/memory) es un archivo markdown que añades a la raíz de tu proyecto que Claude Code lee al inicio de cada sesión. Úsalo para establecer estándares de codificación, decisiones de arquitectura, librerías preferidas y listas de verificación de revisión. Claude también construye [memoria automática](/docs/es/memory#auto-memory) mientras trabaja, guardando aprendizajes como comandos de compilación e insights de depuración en sesiones sin que escribas nada.

    Crea [skills](/docs/es/skills) para empaquetar flujos de trabajo repetibles que tu equipo pueda compartir, como `/review-pr` o `/deploy-staging`.

    [Hooks](/docs/es/hooks) te permiten ejecutar comandos de shell antes o después de acciones de Claude Code, como formateo automático después de cada edición de archivo o ejecución de lint antes de un commit.
  </Accordion>

  <Accordion title="Ejecuta equipos de agentes y construye agentes personalizados" icon="users">
    Genera [múltiples agentes de Claude Code](/docs/es/sub-agents) que trabajen en diferentes partes de una tarea simultáneamente. Un agente líder coordina el trabajo, asigna subtareas y fusiona resultados.

    Para ejecutar varias sesiones completas en paralelo y observarlas desde una pantalla, usa [agentes en segundo plano](/docs/es/agent-view). Para flujos de trabajo completamente personalizados, el [Agent SDK](/docs/es/agent-sdk/overview) te permite construir tus propios agentes impulsados por las herramientas y capacidades de Claude Code, con control total sobre orquestación, acceso a herramientas y permisos.
  </Accordion>

  <Accordion title="Canaliza, secuencia y automatiza con la CLI" icon="terminal">
    Claude Code es componible y sigue la filosofía de Unix. Canaliza registros en él, ejecútalo en CI o encadénalo con otras herramientas:

    ```bash theme={null}
    # Analiza la salida de registros recientes
    tail -200 app.log | claude -p "Slack me if you see any anomalies"

    # Automatiza traducciones en CI
    claude -p "translate new strings into French and raise a PR for review"

    # Operaciones en masa en archivos
    git diff main --name-only | claude -p "review these changed files for security issues"
    ```

    Consulta la [referencia de CLI](/docs/es/cli-reference) para el conjunto completo de comandos y banderas.
  </Accordion>

  <Accordion title="Programa tareas recurrentes" icon="clock">
    Ejecuta Claude en un horario para automatizar el trabajo que se repite: revisiones de PR matutinas, análisis de fallos de CI durante la noche, auditorías de dependencias semanales o sincronización de documentos después de que se fusionen los PR.

    * [Routines](/docs/es/routines) se ejecutan en infraestructura administrada por Anthropic, por lo que siguen ejecutándose incluso cuando tu computadora está apagada. También pueden activarse en llamadas de API o eventos de GitHub. Créalas desde la web, la aplicación de escritorio o ejecutando `/schedule` en la CLI.
    * [Tareas programadas de escritorio](/docs/es/desktop-scheduled-tasks) se ejecutan en tu máquina, con acceso directo a tus archivos y herramientas locales
    * [`/loop`](/docs/es/scheduled-tasks) repite un prompt dentro de una sesión de CLI para sondeo rápido
  </Accordion>

  <Accordion title="Trabaja desde cualquier lugar" icon="globe">
    Las sesiones no están vinculadas a una única superficie. Mueve el trabajo entre entornos a medida que cambia tu contexto:

    * Aléjate de tu escritorio y sigue trabajando desde tu teléfono o cualquier navegador con [Remote Control](/docs/es/remote-control)
    * Envía un mensaje a [Dispatch](/docs/es/desktop#sessions-from-dispatch) con una tarea desde tu teléfono y abre la sesión de escritorio que crea
    * Inicia una tarea de larga duración en la [web](/docs/es/claude-code-on-the-web) o [aplicación iOS](https://apps.apple.com/app/claude-by-anthropic/id6473753684), luego extráela a tu terminal con `claude --teleport`. Teleport requiere una suscripción a claude.ai.
    * Transfiere una sesión de terminal a la [aplicación de escritorio](/docs/es/desktop) con `/desktop` para revisión visual de diff
    * Enruta tareas desde el chat del equipo: menciona `@Claude` en [Slack](/docs/es/slack) con un informe de error y obtén una solicitud de extracción de vuelta
  </Accordion>
</AccordionGroup>

<h2 id="use-claude-code-everywhere">
  Usa Claude Code en todas partes
</h2>

Cada [superficie](/docs/es/glossary#surface) se conecta al mismo motor subyacente de Claude Code, por lo que tus archivos CLAUDE.md, configuración y servidores MCP funcionan en todos ellos.

Más allá de los entornos [Terminal](/docs/es/quickstart), [VS Code](/docs/es/vs-code), [JetBrains](/docs/es/jetbrains), [Desktop](/docs/es/desktop) y [Web](/docs/es/claude-code-on-the-web) anteriores, Claude Code se integra con flujos de trabajo de CI/CD, chat y navegador:

| Quiero...                                                                            | Mejor opción                                                                                                             |
| ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------ |
| Continuar una sesión local desde mi teléfono u otro dispositivo                      | [Remote Control](/docs/es/remote-control)                                                                                     |
| Enviar eventos desde Telegram, Discord, iMessage o mis propios webhooks a una sesión | [Channels](/docs/es/channels)                                                                                                 |
| Iniciar una tarea localmente, continuar en móvil                                     | [Web](/docs/es/claude-code-on-the-web) o [aplicación Claude iOS](https://apps.apple.com/app/claude-by-anthropic/id6473753684) |
| Ejecutar Claude en un horario recurrente                                             | [Routines](/docs/es/routines) o [Tareas programadas de escritorio](/docs/es/desktop-scheduled-tasks)                               |
| Automatizar revisiones de PR y clasificación de problemas                            | [GitHub Actions](/docs/es/github-actions) o [GitLab CI/CD](/docs/es/gitlab-ci-cd)                                                  |
| Obtener revisión de código automática en cada PR                                     | [GitHub Code Review](/docs/es/code-review)                                                                                    |
| Enrutar informes de errores de Slack a solicitudes de extracción                     | [Slack](/docs/es/slack)                                                                                                       |
| Depurar aplicaciones web en vivo                                                     | [Chrome](/docs/es/chrome)                                                                                                     |
| Construir agentes personalizados para tus propios flujos de trabajo                  | [Agent SDK](/docs/es/agent-sdk/overview)                                                                                      |

<h2 id="next-steps">
  Próximos pasos
</h2>

Una vez que hayas instalado Claude Code, estas guías te ayudan a profundizar.

* [Guía de inicio rápido](/docs/es/quickstart): recorre tu primera tarea real, desde explorar una base de código hasta confirmar una corrección
* [Almacena instrucciones y memorias](/docs/es/memory): proporciona a Claude instrucciones persistentes con archivos CLAUDE.md y memoria automática
* [Flujos de trabajo comunes](/docs/es/common-workflows) y [mejores prácticas](/docs/es/best-practices): patrones para obtener lo máximo de Claude Code
* [Un arnés para cada tarea](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code): cómo el equipo de Claude Code utiliza [flujos de trabajo dinámicos](/docs/es/workflows) para orquestar subagentes a escala
* [Configuración](/docs/es/settings): personaliza Claude Code para tu flujo de trabajo
* [Solución de problemas](/docs/es/troubleshooting): soluciones para problemas comunes
* [code.claude.com](https://code.claude.com/): demostraciones, precios y detalles del producto
