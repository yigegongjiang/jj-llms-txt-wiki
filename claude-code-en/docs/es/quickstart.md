> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Inicio rápido

> ¡Bienvenido a Claude Code!

Esta guía de inicio rápido le permitirá usar asistencia de codificación impulsada por IA en pocos minutos. Al final, comprenderá cómo usar Claude Code para tareas comunes de desarrollo.

<h2 id="before-you-begin">
  Antes de comenzar
</h2>

Asegúrese de tener:

* Una terminal o símbolo del sistema abiertos
  * Si nunca ha usado la terminal antes, consulte la [guía de terminal](/docs/es/terminal-guide)
* Un proyecto de código con el que trabajar
* Una [suscripción a Claude](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_prereq) (Pro, Max, Team o Enterprise), una cuenta de [Claude Console](https://platform.claude.com/), o acceso a través de un [proveedor de nube compatible](/docs/es/third-party-integrations)

<Note>
  Esta guía cubre la CLI de terminal. Claude Code también está disponible en la [web](https://claude.ai/code), como una [aplicación de escritorio](/docs/es/desktop), en [VS Code](/docs/es/vs-code) e [IDEs de JetBrains](/docs/es/jetbrains), en [Slack](/docs/es/slack), y en CI/CD con [GitHub Actions](/docs/es/github-actions) y [GitLab](/docs/es/gitlab-ci-cd). Consulte [todas las interfaces](/docs/es/overview#use-claude-code-everywhere).
</Note>

<h2 id="step-1-install-claude-code">
  Paso 1: Instalar Claude Code
</h2>

Para instalar Claude Code, utilice uno de los siguientes métodos:

<Tabs>
  <Tab title="Instalación nativa (recomendado)">
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

    Si ve `The token '&&' is not a valid statement separator`, está en PowerShell, no en CMD. Si ve `'irm' is not recognized as an internal or external command`, está en CMD, no en PowerShell. Su símbolo del sistema muestra `PS C:\` cuando está en PowerShell y `C:\` sin el `PS` cuando está en CMD.

    Si el comando de instalación falla con `syntax error near unexpected token '<'`, un `403`, u otro error de curl, consulte [Solucionar problemas de instalación](/docs/es/troubleshoot-install#find-your-error) para hacer coincidir el error con una solución y para conocer métodos de instalación alternativos.

    Se recomienda [Git for Windows](https://git-scm.com/downloads/win) en Windows nativo para que Claude Code pueda utilizar la herramienta Bash. Si Git for Windows no está instalado, Claude Code utiliza PowerShell como herramienta de shell en su lugar. Las configuraciones de WSL no necesitan Git for Windows.

    <Info>
      Las instalaciones nativas se actualizan automáticamente en segundo plano para mantenerlo en la versión más reciente.
    </Info>
  </Tab>

  <Tab title="Homebrew">
    ```bash theme={null}
    brew install --cask claude-code
    ```

    Homebrew ofrece dos casks. `claude-code` rastrea el canal de versión estable, que típicamente está aproximadamente una semana atrás y omite versiones con regresiones importantes. `claude-code@latest` rastrea el canal más reciente y recibe nuevas versiones tan pronto como se lanzan.

    <Info>
      Las instalaciones de Homebrew no se actualizan automáticamente. Ejecute `brew upgrade claude-code` o `brew upgrade claude-code@latest`, dependiendo de cuál cask haya instalado, para obtener las características más recientes y correcciones de seguridad.
    </Info>
  </Tab>

  <Tab title="WinGet">
    ```powershell theme={null}
    winget install Anthropic.ClaudeCode
    ```

    <Info>
      Las instalaciones de WinGet no se actualizan automáticamente. Ejecute `winget upgrade Anthropic.ClaudeCode` periódicamente para obtener las características más recientes y correcciones de seguridad.
    </Info>
  </Tab>
</Tabs>

También puede instalar con [apt, dnf, o apk](/docs/es/setup#install-with-linux-package-managers) en Debian, Fedora, RHEL y Alpine.

Para confirmar que la instalación funcionó, ejecute:

```bash theme={null}
claude --version
```

El comando imprime un número de versión seguido de `(Claude Code)`.

<h2 id="step-2-log-in-to-your-account">
  Paso 2: Inicie sesión en su cuenta
</h2>

Claude Code requiere una cuenta para usarse. Inicie una sesión interactiva con el comando `claude` y se le pedirá que inicie sesión en el primer uso:

```bash theme={null}
claude
```

Para cuentas de suscripción de Claude o Console, siga las indicaciones para completar la autenticación en su navegador. Si ha establecido la variable de entorno `ANTHROPIC_API_KEY`, Claude Code omite la solicitud de inicio de sesión y le pide que apruebe la clave en su lugar. Para cambiar de cuenta más tarde o volver a autenticarse, escriba `/login` dentro de la sesión en ejecución:

```text wrap theme={null}
/login
```

Puede iniciar sesión usando cualquiera de estos tipos de cuenta:

* [Claude Pro, Max, Team o Enterprise](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_login) (recomendado)
* [Claude Console](https://platform.claude.com/) (acceso a API con créditos prepagados). En el primer inicio de sesión, se crea automáticamente un espacio de trabajo "Claude Code" en la Console para el seguimiento centralizado de costos.
* [Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry](/docs/es/third-party-integrations) (proveedores de nube empresariales)
* Una puerta de enlace [Claude apps gateway](/docs/es/claude-apps-gateway) autohospedada, si su organización ejecuta una: su administrador preconfiguración la URL de la puerta de enlace, y `/login` abre directamente en la pantalla **Cloud gateway** para que inicie sesión con SSO corporativo

Una vez que haya iniciado sesión, sus credenciales se almacenan y no necesitará iniciar sesión nuevamente. Obtenga más información en [Gestión de credenciales](/docs/es/authentication#credential-management).

<h2 id="step-3-start-your-first-session">
  Paso 3: Inicie su primera sesión
</h2>

Abra su terminal en cualquier directorio de proyecto e inicie Claude Code:

```bash theme={null}
cd /path/to/your/project
claude
```

Reemplace `/path/to/your/project` con la ruta del proyecto en el que desea trabajar.

Verá la pantalla de Claude Code con la versión, el modelo actual y el directorio de trabajo mostrados arriba. Escriba `/help` para ver los comandos disponibles o `/resume` para continuar una conversación anterior.

<h2 id="step-4-ask-your-first-question">
  Paso 4: Haga su primera pregunta
</h2>

Comencemos por entender su base de código. Intente uno de estos comandos:

```text wrap theme={null}
¿qué hace este proyecto?
```

Claude analizará sus archivos y proporcionará un resumen. También puede hacer preguntas más específicas:

```text wrap theme={null}
¿qué tecnologías usa este proyecto?
```

```text wrap theme={null}
¿dónde está el punto de entrada principal?
```

```text wrap theme={null}
explique la estructura de carpetas
```

También puede preguntarle a Claude sobre sus propias capacidades:

```text wrap theme={null}
¿qué puede hacer Claude Code?
```

```text wrap theme={null}
¿cómo creo skills personalizados en Claude Code?
```

```text wrap theme={null}
¿puede Claude Code trabajar con Docker?
```

<Note>
  Claude Code lee los archivos de su proyecto según sea necesario. No tiene que agregar contexto manualmente.
</Note>

<h2 id="step-5-make-your-first-code-change">
  Paso 5: Realice su primer cambio de código
</h2>

Ahora hagamos que Claude Code haga algo de codificación real. Intente una tarea simple:

```text wrap theme={null}
agrega una función hello world al archivo principal
```

Claude Code encuentra el archivo apropiado y le muestra el cambio. Si pregunta antes de realizar el cambio, seleccione **Sí** para aprobar.

El modo Auto es el [modo de permiso de inicio integrado](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) para sesiones de terminal interactivas en planes Pro, Max y Team: un clasificador revisa las acciones en lugar de usted, y Claude edita la mayoría de los archivos y ejecuta la mayoría de los comandos sin preguntar. En otros planes, el modo Manual es el modo de permiso de inicio integrado. Para la sesión que inicia justo después de instalar, consulte [Primera sesión después de una instalación o actualización](/docs/es/env-vars#first-session-after-an-install-or-upgrade).

<Note>
  Su configuración u organización puede establecer un modo de permiso de inicio diferente. [Qué modo de permiso inicia una sesión](/docs/es/permission-modes#which-mode-a-session-starts-in) enumera lo que hace. Presione `Shift+Tab` en cualquier momento para cambiar el modo de permiso de la sesión en la que se encuentra.
</Note>

<h2 id="step-6-use-git-with-claude-code">
  Paso 6: Use Git con Claude Code
</h2>

Claude Code hace que las operaciones de Git sean conversacionales:

```text wrap theme={null}
¿qué archivos he cambiado?
```

```text wrap theme={null}
confirma mis cambios con un mensaje descriptivo
```

También puede solicitar operaciones de Git más complejas:

```text wrap theme={null}
crea una nueva rama llamada feature/quickstart
```

```text wrap theme={null}
muéstrame los últimos 5 commits
```

```text wrap theme={null}
ayúdame a resolver conflictos de fusión
```

<h2 id="step-7-fix-a-bug-or-add-a-feature">
  Paso 7: Corrija un error o agregue una función
</h2>

Claude es competente en depuración e implementación de funciones.

Describa lo que desea en lenguaje natural:

```text wrap theme={null}
agrega validación de entrada al formulario de registro de usuarios
```

O corrija problemas existentes:

```text wrap theme={null}
hay un error donde los usuarios pueden enviar formularios vacíos - corrígelo
```

Claude Code hará lo siguiente:

* Localizará el código relevante
* Comprenderá el contexto
* Implementará una solución
* Ejecutará pruebas si están disponibles

<h2 id="step-8-test-out-other-common-workflows">
  Paso 8: Pruebe otros flujos de trabajo comunes
</h2>

Hay varias formas de trabajar con Claude:

**Refactorizar código**

```text wrap theme={null}
refactoriza el módulo de autenticación para usar async/await en lugar de callbacks
```

**Escribir pruebas**

```text wrap theme={null}
escribe pruebas unitarias para las funciones de calculadora
```

**Actualizar documentación**

```text wrap theme={null}
actualiza el README con instrucciones de instalación
```

**Revisión de código**

```text wrap theme={null}
revisa mis cambios y sugiere mejoras
```

<Tip>
  Hable con Claude como lo haría con un colega útil. Describa lo que desea lograr y le ayudará a llegar allí.
</Tip>

<h2 id="essential-commands">
  Comandos esenciales
</h2>

Aquí están los comandos más importantes para el uso diario. Los comandos de shell se ejecutan desde su terminal para iniciar o reanudar Claude Code. Los comandos de sesión se ejecutan dentro de Claude Code después de que se inicia.

**Comandos de shell**

| Comando             | Qué hace                                                      | Ejemplo                             |
| ------------------- | ------------------------------------------------------------- | ----------------------------------- |
| `claude`            | Inicia el modo interactivo                                    | `claude`                            |
| `claude "task"`     | Inicia el modo interactivo con un mensaje inicial             | `claude "fix the build error"`      |
| `claude -p "query"` | Ejecuta una consulta única y luego sale                       | `claude -p "explain this function"` |
| `claude -c`         | Continúa la conversación más reciente en el directorio actual | `claude -c`                         |
| `claude -r`         | Reanuda una conversación anterior                             | `claude -r`                         |

**Comandos de sesión**

| Comando                    | Qué hace                           | Ejemplo  |
| -------------------------- | ---------------------------------- | -------- |
| `/clear`                   | Borra el historial de conversación | `/clear` |
| `/help`                    | Muestra los comandos disponibles   | `/help`  |
| `/exit` o Ctrl+D dos veces | Salir de Claude Code               | `/exit`  |

Consulte la [referencia de CLI](/docs/es/cli-reference) para obtener una lista completa de comandos de shell y la [referencia de comandos](/docs/es/commands) para obtener una lista completa de comandos de sesión.

<h2 id="pro-tips-for-beginners">
  Consejos profesionales para principiantes
</h2>

Para más información, consulte [mejores prácticas](/docs/es/best-practices) y [flujos de trabajo comunes](/docs/es/common-workflows).

<AccordionGroup>
  <Accordion title="Sea específico con sus solicitudes">
    En lugar de: "corrige el error"

    Intente: "corrige el error de inicio de sesión donde los usuarios ven una pantalla en blanco después de ingresar credenciales incorrectas"
  </Accordion>

  <Accordion title="Utilice instrucciones paso a paso">
    Divida tareas complejas en pasos:

    ```text wrap theme={null}
    1. crea una nueva tabla de base de datos para perfiles de usuario
    2. crea un endpoint de API para obtener y actualizar perfiles de usuario
    3. construye una página web que permita a los usuarios ver y editar su información
    ```
  </Accordion>

  <Accordion title="Deje que Claude explore primero">
    Antes de hacer cambios, deje que Claude entienda su código:

    ```text wrap theme={null}
    analiza el esquema de la base de datos
    ```

    ```text wrap theme={null}
    construye un panel que muestre los productos que nuestros clientes del Reino Unido devuelven con más frecuencia
    ```
  </Accordion>

  <Accordion title="Ahorre tiempo con atajos de teclado">
    * Escriba `/` para ver todos los comandos y skills disponibles
    * Utilice Tab para completar comandos
    * Presione ↑ para el historial de comandos
    * Presione `Shift+Tab` para ciclar a través de los modos de permisos
  </Accordion>
</AccordionGroup>

<h2 id="what’s-next">
  ¿Qué sigue?
</h2>

Ahora que ha aprendido lo básico, explore funciones más avanzadas:

<CardGroup cols={2}>
  <Card title="Cómo funciona Claude Code" icon="microchip" href="/docs/es/how-claude-code-works">
    Comprenda el bucle de agente, las herramientas integradas y cómo Claude Code interactúa con su proyecto
  </Card>

  <Card title="Mejores prácticas" icon="star" href="/docs/es/best-practices">
    Obtenga mejores resultados con indicaciones efectivas y configuración de proyecto
  </Card>

  <Card title="Flujos de trabajo comunes" icon="graduation-cap" href="/docs/es/common-workflows">
    Guías paso a paso para tareas comunes
  </Card>

  <Card title="Extiende Claude Code" icon="puzzle-piece" href="/docs/es/features-overview">
    Personalice con CLAUDE.md, skills, hooks, MCP y más
  </Card>
</CardGroup>

<h2 id="getting-help">
  Obtener ayuda
</h2>

* **En Claude Code**: Escriba `/help` o pregunte "¿cómo..."
* **Documentación**: ¡Está aquí! Explore otras guías
* **Cursos**: Tome [Claude Code 101](https://academy.claude.com/courses/claude-code-101) y otros cursos gratuitos a su propio ritmo en [Claude Academy](https://academy.claude.com/)
* **Comunidad**: Únase a nuestro [Discord](https://www.anthropic.com/discord) para consejos y soporte
