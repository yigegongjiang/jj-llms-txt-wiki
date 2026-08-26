> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Usar Claude Code en la web

> Configura entornos en la nube, scripts de configuración, acceso a la red y Docker en el sandbox de Anthropic. Mueve sesiones entre web y terminal con `--cloud` y `--teleport`.

<Note>
  Claude Code en la web está en vista previa de investigación para usuarios Pro, Max y Team, y para usuarios Enterprise con asientos premium o asientos Chat + Claude Code.
</Note>

Claude Code en la web ejecuta tareas en infraestructura en la nube administrada por Anthropic en [claude.ai/code](https://claude.ai/code). Las sesiones persisten incluso si cierra su navegador, y puede monitorearlas desde la aplicación móvil Claude.

<Tip>
  ¿Nuevo en Claude Code en la web? Comience con [Primeros pasos](/docs/es/web-quickstart) para conectar su cuenta de GitHub y enviar su primera tarea.
</Tip>

Esta página cubre:

* [Opciones de autenticación de GitHub](#github-authentication-options): dos formas de conectar GitHub
* [El entorno en la nube](#the-cloud-environment): qué configuración se transfiere, qué herramientas están instaladas y cómo configurar entornos
* [Scripts de configuración](#setup-scripts) y gestión de dependencias
* [Acceso a la red](#network-access): niveles, proxies y la lista de permitidos predeterminada
* [Mover tareas entre web y terminal](#move-tasks-between-web-and-terminal) con `--cloud` y `--teleport`
* [Trabajar con sesiones](#work-with-sessions): revisar, compartir, archivar, eliminar
* [Correcciones automáticas de solicitudes de extracción](#auto-fix-pull-requests): responder automáticamente a fallos de CI y comentarios de revisión
* [Seguridad y aislamiento](#security-and-isolation): cómo se aíslan las sesiones
* [Limitaciones](#limitations): límites de velocidad y restricciones de plataforma

<h2 id="github-authentication-options">
  Opciones de autenticación de GitHub
</h2>

Las sesiones en la nube necesitan acceso a sus repositorios de GitHub para clonar código e insertar ramas. Puede otorgar acceso de dos formas:

| Método           | Cómo funciona                                                                                       | Mejor para                                                                                         |
| :--------------- | :-------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------- |
| **GitHub App**   | Autorice la aplicación Claude GitHub durante la [incorporación web](/docs/es/web-quickstart).            | Incorporación en navegador; equipos que desean [Correcciones automáticas](#auto-fix-pull-requests) |
| **`/web-setup`** | Ejecute `/web-setup` en su terminal para sincronizar su token local de CLI `gh` a su cuenta Claude. | Desarrolladores individuales que ya usan `gh`                                                      |

<Note>
  Con cualquiera de los dos métodos, una sesión en la nube puede acceder a cualquier repositorio que la cuenta de GitHub conectada pueda ver, no solo los repositorios en los que está instalada la aplicación Claude GitHub. La instalación de la aplicación habilita webhooks de PR para [Correcciones automáticas](#auto-fix-pull-requests); no es un control de acceso a nivel de sesión. Para restringir qué repositorios puede alcanzar su equipo desde sesiones en la nube, restrinja el acceso en GitHub mismo, por ejemplo limitando la membresía de equipo o repositorio para las cuentas de GitHub conectadas.
</Note>

Cualquiera de los dos métodos funciona. [`/schedule`](/docs/es/routines) verifica cualquiera de las dos formas de acceso y le solicita que ejecute `/web-setup` si ninguna está configurada. Consulte [Conectar desde su terminal](/docs/es/web-quickstart#connect-from-your-terminal) para el tutorial de `/web-setup`.

La aplicación GitHub es necesaria para [Correcciones automáticas](#auto-fix-pull-requests), que usa la aplicación para recibir webhooks de PR. Si se conecta con `/web-setup` y luego desea correcciones automáticas, instale la aplicación en esos repositorios.

Los administradores de Team y Enterprise pueden deshabilitar `/web-setup` con el interruptor de configuración web rápida en [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code).

<Note>
  Las organizaciones con [Retención de datos cero](/docs/es/zero-data-retention) habilitada no pueden usar `/web-setup` u otras características de sesión en la nube.
</Note>

<h2 id="the-cloud-environment">
  El entorno en la nube
</h2>

Cada sesión se ejecuta en una VM nueva administrada por Anthropic con su repositorio clonado. Esta sección cubre qué está disponible cuando comienza una sesión y cómo personalizarlo.

<h3 id="what’s-available-in-cloud-sessions">
  Qué está disponible en sesiones en la nube
</h3>

Las sesiones en la nube comienzan desde un clon nuevo de su repositorio. Cualquier cosa comprometida con el repositorio está disponible. Cualquier cosa que haya instalado o configurado solo en su propia máquina no está disponible en la sesión. La política de su organización llega por separado a través de [configuración administrada por el servidor](/docs/es/server-managed-settings).

|                                                                                                 | Disponible en sesiones en la nube | Por qué                                                                                                                                                                                                                                                                                                                                                                                              |
| :---------------------------------------------------------------------------------------------- | :-------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Su `CLAUDE.md` del repositorio                                                                  | Sí                                | Parte del clon                                                                                                                                                                                                                                                                                                                                                                                       |
| Sus hooks `.claude/settings.json` del repositorio                                               | Sí                                | Parte del clon                                                                                                                                                                                                                                                                                                                                                                                       |
| Sus servidores MCP `.mcp.json` del repositorio                                                  | Sí                                | Parte del clon                                                                                                                                                                                                                                                                                                                                                                                       |
| Su `.claude/rules/` del repositorio                                                             | Sí                                | Parte del clon                                                                                                                                                                                                                                                                                                                                                                                       |
| Su `.claude/skills/`, `.claude/agents/`, `.claude/commands/` del repositorio                    | Sí                                | Parte del clon                                                                                                                                                                                                                                                                                                                                                                                       |
| Plugins declarados en `.claude/settings.json`                                                   | Sí                                | Instalados al inicio de la sesión desde el [marketplace](/docs/es/plugin-marketplaces) que declaró. Requiere acceso a la red para llegar a la fuente del marketplace                                                                                                                                                                                                                                      |
| Su [configuración administrada por el servidor](/docs/es/server-managed-settings) de la organización | Sí                                | Se obtiene de los servidores de Anthropic cuando comienza la sesión. Consulte [Cobertura de superficie](/docs/es/model-config#surface-coverage) para ver cómo se aplica `availableModels` en sesiones en la nube. La configuración implementada en su dispositivo a través de MDM o archivos de configuración administrada no se aplica, porque la sesión se ejecuta en una VM administrada por Anthropic |
| Su `~/.claude/CLAUDE.md` de usuario                                                             | No                                | Vive en su máquina, no en el repositorio                                                                                                                                                                                                                                                                                                                                                             |
| Su `~/.claude/skills/`, `~/.claude/agents/`, `~/.claude/commands/` de usuario                   | No                                | Viven en su máquina, no en el repositorio. Comprométalos en el directorio `.claude/` del repositorio en su lugar. Las skills que habilita en claude.ai se cargan automáticamente en sesiones en la nube                                                                                                                                                                                              |
| Plugins habilitados solo en su configuración de usuario                                         | No                                | El `enabledPlugins` con alcance de usuario vive en `~/.claude/settings.json`. Declárelos en el `.claude/settings.json` del repositorio en su lugar                                                                                                                                                                                                                                                   |
| Servidores MCP que agregó con `claude mcp add`                                                  | No                                | Esos escriben en su configuración de usuario local, no en el repositorio. Declare el servidor en [`.mcp.json`](/docs/es/mcp#project-scope) en su lugar                                                                                                                                                                                                                                                    |
| Tokens de API estáticos y credenciales                                                          | No                                | Aún no existe un almacén de secretos dedicado. Vea a continuación                                                                                                                                                                                                                                                                                                                                    |
| Autenticación interactiva como AWS SSO                                                          | No                                | No compatible. SSO requiere inicio de sesión basado en navegador que no puede ejecutarse en una sesión en la nube                                                                                                                                                                                                                                                                                    |

Para que su configuración esté disponible en sesiones en la nube, comprométala en el repositorio; la política de su organización llega por separado a través de [configuración administrada por el servidor](/docs/es/server-managed-settings).

Un almacén de secretos dedicado aún no está disponible. Tanto las variables de entorno como los scripts de configuración se almacenan en la configuración del entorno, visible para cualquiera que pueda editar ese entorno. Si necesita secretos en una sesión en la nube, agréguelos como variables de entorno con esa visibilidad en mente.

<h3 id="installed-tools">
  Herramientas instaladas
</h3>

Las sesiones en la nube vienen con tiempos de ejecución de lenguaje comunes, herramientas de compilación y bases de datos preinstaladas. La tabla a continuación resume lo que se incluye por categoría.

| Categoría          | Incluido                                                                       |
| :----------------- | :----------------------------------------------------------------------------- |
| **Python**         | Python 3.x con pip, poetry, uv, black, mypy, pytest, ruff                      |
| **Node.js**        | 20, 21 y 22 vía nvm, con npm, yarn, pnpm, bun¹, eslint, prettier, chromedriver |
| **Ruby**           | 3.1, 3.2, 3.3 con gem, bundler, rbenv                                          |
| **PHP**            | 8.4 con Composer                                                               |
| **Java**           | OpenJDK 21 con Maven y Gradle                                                  |
| **Go**             | última versión estable con soporte de módulos                                  |
| **Rust**           | rustc y cargo                                                                  |
| **C/C++**          | GCC, Clang, cmake, ninja, conan                                                |
| **Docker**         | docker, dockerd, docker compose                                                |
| **Bases de datos** | PostgreSQL 16, Redis 7.0                                                       |
| **Utilidades**     | git, jq, yq, ripgrep, tmux, vim, nano                                          |

¹ Bun está instalado pero tiene [problemas de compatibilidad de proxy](#install-dependencies-with-a-sessionstart-hook) conocidos para obtención de paquetes.

Para versiones exactas, pida a Claude que ejecute `check-tools` en una sesión en la nube. Este comando solo existe en sesiones en la nube.

<h3 id="work-with-github-issues-and-pull-requests">
  Trabajar con problemas y solicitudes de extracción de GitHub
</h3>

Las sesiones en la nube incluyen herramientas de GitHub integradas que permiten a Claude leer problemas, listar solicitudes de extracción, obtener diffs y publicar comentarios sin ninguna configuración. Estas herramientas se autentican a través del [proxy de GitHub](#github-proxy) usando cualquier método que configuró en [Opciones de autenticación de GitHub](#github-authentication-options), por lo que su token nunca entra en el contenedor.

Puede establecer `GH_TOKEN` o `GITHUB_TOKEN` usted mismo en [configuración de entorno](#configure-your-environment), o dejar ambos sin establecer y dejar que el [proxy de GitHub](#github-proxy) se autentique por usted:

* Si establece un token, se pasa al contenedor sin cambios, por lo que `gh` y sus scripts lo usan directamente.
* Si no establece ninguno, el contenedor establece ambas variables en la cadena de marcador de posición `proxy-injected` y el proxy sustituye sus credenciales reales en solicitudes salientes de GitHub. `gh` funciona sin un token propio, pero un script que lee `GITHUB_TOKEN` directamente obtiene el marcador de posición, no un token utilizable.

Para verificar cuál es el caso en su sesión, pida a Claude que ejecute `echo $GH_TOKEN`.

La CLI `gh` no está preinstalada. Si necesita un comando `gh` que las herramientas integradas no cubran, como `gh release` o `gh workflow run`, instálelo y auténtiquese usted mismo:

<Steps>
  <Step title="Instale gh en su script de configuración">
    Agregue `apt update && apt install -y gh` a su [script de configuración](#setup-scripts).
  </Step>

  <Step title="Proporcione un token si el proxy no está manejando la autenticación">
    Si `echo $GH_TOKEN` imprime `proxy-injected`, el [proxy de GitHub](#github-proxy) autentica `gh` para usted y este paso es innecesario. De lo contrario, agregue una variable de entorno `GH_TOKEN` a su [configuración de entorno](#configure-your-environment) con un token de acceso personal de GitHub. `gh` lee `GH_TOKEN` automáticamente, por lo que no se necesita un paso `gh auth login`.
  </Step>
</Steps>

<h3 id="link-output-back-to-the-session">
  Vincule la salida de vuelta a la sesión
</h3>

Cada sesión en la nube tiene una URL de transcripción en claude.ai, y la sesión puede leer su propio ID desde la variable de entorno `CLAUDE_CODE_REMOTE_SESSION_ID`. Use esto para poner un enlace rastreable en cuerpos de PR, mensajes de confirmación, publicaciones de Slack o informes generados para que un revisor pueda abrir la ejecución que los produjo.

A partir de v2.1.179, los commits que Claude crea en una sesión web incluyen un trailer de git `Claude-Session: <url>`, y los cuerpos de PR incluyen la URL de la sesión en su propia línea. Desde v2.1.182, establezca [`attribution.sessionUrl`](/docs/es/settings#attribution-settings) en `false` para omitir el trailer y el enlace del cuerpo de PR.

Para incluir el enlace de sesión en algo que no sea un commit o PR, como un mensaje de Slack que Claude publica o un archivo de informe que escribe, haga que Claude ejecute el siguiente comando y use su salida. El comando convierte el prefijo `cse_` en el valor de la variable de entorno al prefijo `session_` que la URL de transcripción espera:

```bash theme={null}
echo "https://claude.ai/code/${CLAUDE_CODE_REMOTE_SESSION_ID/#cse_/session_}"
```

<h3 id="run-tests-start-services-and-add-packages">
  Ejecute pruebas, inicie servicios y agregue paquetes
</h3>

Claude ejecuta pruebas como parte del trabajo en una tarea. Pídalo en su solicitud, como "corregir las pruebas fallidas en `tests/`" o "ejecutar pytest después de cada cambio". Los ejecutores de pruebas como pytest, jest y cargo test están preinstalados y funcionan sin configuración adicional.

PostgreSQL y Redis están preinstalados pero no se ejecutan de forma predeterminada. Pida a Claude que inicie cada uno durante la sesión:

```bash theme={null}
service postgresql start
```

```bash theme={null}
service redis-server start
```

Docker está disponible para ejecutar servicios en contenedores. Pida a Claude que ejecute `docker compose up` para iniciar los servicios de su proyecto. El acceso a la red para extraer imágenes sigue el [nivel de acceso](#access-levels) de su entorno, y los [Valores predeterminados confiables](#default-allowed-domains) incluyen Docker Hub y otros registros comunes.

Si sus imágenes son grandes o lentas de extraer, agregue `docker compose pull` o `docker compose build` a su [script de configuración](#setup-scripts). Las imágenes extraídas se guardan en el [entorno en caché](#environment-caching), por lo que cada nueva sesión las tiene en el disco. El caché almacena solo archivos, no procesos en ejecución, por lo que Claude aún inicia los contenedores cada sesión.

Para agregar paquetes que no están preinstalados, use un [script de configuración](#setup-scripts). La salida del script se [almacena en caché](#environment-caching), por lo que los paquetes que instale allí están disponibles al inicio de cada sesión sin reinstalar cada vez. También puede pedir a Claude que instale paquetes durante la sesión, pero esas instalaciones no persisten entre sesiones.

<h3 id="resource-limits">
  Límites de recursos
</h3>

Las sesiones en la nube se ejecutan con límites de recursos aproximados que pueden cambiar con el tiempo:

* 4 vCPUs
* 16 GB de RAM
* 30 GB de disco

Las tareas que requieren significativamente más memoria, como trabajos de compilación grandes o pruebas que consumen mucha memoria, pueden fallar o ser terminadas. Para cargas de trabajo más allá de estos límites, use [Control Remoto](/docs/es/remote-control) para ejecutar Claude Code en su propio hardware.

<h3 id="configure-your-environment">
  Configure su entorno
</h3>

Los entornos controlan [acceso a la red](#network-access), variables de entorno y el [script de configuración](#setup-scripts) que se ejecuta antes de que comience una sesión. Consulte [Herramientas instaladas](#installed-tools) para ver qué está disponible sin ninguna configuración. Puede administrar entornos desde la interfaz web o desde la terminal:

| Acción                                                       | Cómo                                                                                                                                                                                                                     |
| :----------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Agregar un entorno                                           | Seleccione el entorno actual para abrir el selector, luego seleccione **Agregar entorno**. El diálogo incluye nombre, nivel de acceso a la red, variables de entorno y script de configuración.                          |
| Editar un entorno                                            | Seleccione el icono de nube que muestra el nombre del entorno actual para abrir el selector, pase el cursor sobre un entorno y haga clic en el icono de configuración que aparece a la derecha.                          |
| Archivar un entorno                                          | Abra el entorno para editar y seleccione **Archivar**. Los entornos archivados se ocultan del selector pero las sesiones existentes continúan ejecutándose.                                                              |
| Establecer el predeterminado para sesiones en la nube de CLI | Ejecute `/remote-env` en su terminal. Si tiene un único entorno, este comando muestra su configuración actual. `/remote-env` solo selecciona el predeterminado; agregue, edite y archive entornos desde la interfaz web. |

Las variables de entorno usan formato `.env` con un par `KEY=value` por línea. No envuelva valores entre comillas, ya que las comillas se almacenan como parte del valor. Este ejemplo define tres variables:

```text theme={null}
NODE_ENV=development
LOG_LEVEL=debug
DATABASE_URL=postgres://localhost:5432/myapp
```

<h3 id="organization-shared-environments">
  Entornos compartidos por la organización
</h3>

Los propietarios y administradores en planes Team y Enterprise pueden crear entornos en la nube que se comparten con cada miembro de la organización. Los entornos compartidos aparecen en el selector de entorno de cada miembro junto con los suyos personales, por lo que un equipo puede estandarizar en una configuración en lugar de que cada miembro la recree.

Administre entornos compartidos desde la página **Entornos en la nube** en [configuración de administrador](https://claude.ai/admin-settings). Desde allí puede:

* Crear, editar y archivar entornos compartidos. Cada uno tiene los mismos campos que un entorno personal: un nombre, un [nivel de acceso a la red](#access-levels), [variables de entorno](#configure-your-environment) en formato `.env` y un [script de configuración](#setup-scripts).
* Establecer el entorno predeterminado para la organización.

Los valores en un entorno compartido llegan a las sesiones de cada miembro en ese entorno. Como los entornos personales, los entornos compartidos no tienen un almacén de secretos dedicado, por lo que no incluya secretos.

<h2 id="setup-scripts">
  Scripts de configuración
</h2>

Un script de configuración es un script Bash que se ejecuta cuando comienza una nueva sesión en la nube, antes de que se lance Claude Code. Use scripts de configuración para instalar dependencias, configurar herramientas o obtener cualquier cosa que la sesión necesite que no esté preinstalada.

Los scripts se ejecutan como root en Ubuntu 24.04, por lo que `apt install` y la mayoría de los administradores de paquetes de lenguaje funcionan.

Para agregar un script de configuración, abra el diálogo de configuración del entorno e ingrese su script en el campo **Script de configuración**.

Este ejemplo instala la CLI `gh`, que no está preinstalada:

```bash theme={null}
#!/bin/bash
apt update && apt install -y gh
```

Si el script sale con un código distinto de cero, la sesión no se inicia. Agregue `|| true` a comandos no críticos para evitar bloquear la sesión en una instalación intermitente fallida.

Mantenga el tiempo de ejecución total del script por debajo de aproximadamente cinco minutos para que el [caché del entorno](#environment-caching) pueda compilarse. Ejecute instalaciones independientes en paralelo con `&` y `wait`. Si una única descarga no cabe en el límite de cinco minutos, muévala a un [hook SessionStart](#setup-scripts-vs-sessionstart-hooks) que la inicie en segundo plano.

<Note>
  Los scripts de configuración que instalan paquetes necesitan acceso a la red para llegar a los registros. El acceso a la red predeterminado **Confiable** permite conexiones a [dominios comunes en la lista de permitidos](#default-allowed-domains) incluyendo npm, PyPI, RubyGems y crates.io. Los scripts fallarán al instalar paquetes si su entorno usa acceso a la red **Ninguno**.
</Note>

<h3 id="environment-caching">
  Almacenamiento en caché del entorno
</h3>

El script de configuración se ejecuta la primera vez que inicia una sesión en un entorno. Después de que se completa, Anthropic toma una instantánea del sistema de archivos y reutiliza esa instantánea como punto de partida para sesiones posteriores. Las nuevas sesiones comienzan con sus dependencias, herramientas e imágenes de Docker ya en el disco, y se omite el paso del script de configuración. Esto mantiene el inicio rápido incluso cuando el script instala cadenas de herramientas grandes o extrae imágenes de contenedor.

El caché captura archivos, no procesos en ejecución. Cualquier cosa que el script de configuración escriba en el disco se transfiere. Los servicios o contenedores que inicia no, por lo que inicie esos por sesión pidiendo a Claude o con un [hook SessionStart](#setup-scripts-vs-sessionstart-hooks).

El script de configuración se ejecuta nuevamente para reconstruir el caché cuando cambia el script de configuración del entorno o los hosts de red permitidos, y cuando el caché alcanza su vencimiento después de aproximadamente siete días. Reanudar una sesión existente nunca vuelve a ejecutar el script de configuración.

No necesita habilitar el almacenamiento en caché ni administrar instantáneas usted mismo.

<h3 id="setup-scripts-vs-sessionstart-hooks">
  Scripts de configuración vs. hooks SessionStart
</h3>

Use un script de configuración para instalar cosas que la nube necesita pero su portátil ya tiene, como un tiempo de ejecución de lenguaje o herramienta CLI. Use un [hook SessionStart](/docs/es/hooks#sessionstart) para la configuración del proyecto que debe ejecutarse en todas partes, nube y local, como `npm install`.

Ambos se ejecutan al inicio de una sesión, pero pertenecen a diferentes lugares:

|                | Scripts de configuración                                                                             | Hooks SessionStart                                                        |
| -------------- | ---------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| Adjunto a      | El entorno en la nube                                                                                | Su repositorio                                                            |
| Configurado en | Interfaz de usuario del entorno en la nube                                                           | `.claude/settings.json` en su repositorio                                 |
| Se ejecuta     | Antes de que se lance Claude Code, cuando no hay [entorno en caché](#environment-caching) disponible | Después de que se lance Claude Code, en cada sesión incluyendo reanudadas |
| Alcance        | Solo entornos en la nube                                                                             | Tanto local como nube                                                     |

Los hooks SessionStart también se pueden definir en su `~/.claude/settings.json` a nivel de usuario localmente, pero la configuración a nivel de usuario no se transfiere a sesiones en la nube. En la nube, los hooks provienen del repositorio y de la [configuración administrada por el servidor](/docs/es/server-managed-settings) de su organización.

<h3 id="install-dependencies-with-a-sessionstart-hook">
  Instale dependencias con un hook SessionStart
</h3>

Para instalar dependencias solo en sesiones en la nube, agregue un hook SessionStart a su `.claude/settings.json` del repositorio:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "startup|resume",
        "hooks": [
          {
            "type": "command",
            "command": "\"$CLAUDE_PROJECT_DIR\"/scripts/install_pkgs.sh"
          }
        ]
      }
    ]
  }
}
```

Cree el script en `scripts/install_pkgs.sh` y hágalo ejecutable con `chmod +x`. La variable de entorno `CLAUDE_CODE_REMOTE` se establece en `true` en sesiones en la nube, por lo que puede usarla para omitir la ejecución local:

```bash theme={null}
#!/bin/bash

if [ "$CLAUDE_CODE_REMOTE" != "true" ]; then
  exit 0
fi

npm install
pip install -r requirements.txt
exit 0
```

Los hooks SessionStart tienen algunas limitaciones en sesiones en la nube:

* **Sin alcance solo en la nube**: los hooks se ejecutan en sesiones locales y en la nube. Para omitir la ejecución local, verifique la variable de entorno `CLAUDE_CODE_REMOTE` como se muestra arriba.
* **Requiere acceso a la red**: los comandos de instalación necesitan llegar a los registros de paquetes. Si su entorno usa acceso a la red **Ninguno**, estos hooks fallan. La [lista de permitidos predeterminada](#default-allowed-domains) bajo **Confiable** cubre npm, PyPI, RubyGems y crates.io.
* **Compatibilidad de proxy**: todo el tráfico saliente pasa a través de un [proxy de seguridad](#security-proxy). Algunos administradores de paquetes no funcionan correctamente con este proxy. Bun es un ejemplo conocido.
* **Agrega latencia de inicio**: los hooks se ejecutan cada vez que comienza o se reanuda una sesión, a diferencia de los scripts de configuración que se benefician del [almacenamiento en caché del entorno](#environment-caching). Mantenga los scripts de instalación rápidos verificando si las dependencias ya están presentes antes de reinstalar.

Para persistir variables de entorno para comandos Bash posteriores, escriba en el archivo en `$CLAUDE_ENV_FILE`. Consulte [hooks SessionStart](/docs/es/hooks#sessionstart) para obtener detalles.

Reemplazar la imagen base con su propia imagen Docker aún no es compatible. Use un script de configuración para instalar lo que necesita en la [imagen proporcionada](#installed-tools), o ejecute su imagen como un contenedor junto a Claude con `docker compose`.

<h2 id="network-access">
  Acceso a la red
</h2>

El acceso a la red controla las conexiones salientes desde el entorno en la nube. Cada entorno especifica un nivel de acceso, y puede extenderlo con dominios permitidos personalizados. El predeterminado es **Confiable**, que permite registros de paquetes y otros [dominios en la lista de permitidos](#default-allowed-domains).

Para cambiar el acceso a la red de un entorno, [abra el entorno para editar](#configure-your-environment) y use el selector **Acceso a la red** en el diálogo. No hay una página de Entornos separada. El icono de nube aparece dondequiera que inicie una sesión en la nube o configure una [rutina](/docs/es/routines#environments-and-network-access).

<Note>
  El tráfico del conector MCP se enruta a través de los servidores de Anthropic, por lo que los conectores que habilita en una sesión o rutina funcionan sin agregar sus hosts a **Dominios permitidos**. Los conectores se configuran por sesión o por rutina; elimine cualquiera que no necesite para limitar qué herramientas puede alcanzar Claude. Esto se basa en el mismo canal vinculado a Anthropic mencionado en [Seguridad y aislamiento](#security-and-isolation).
</Note>

<h3 id="access-levels">
  Niveles de acceso
</h3>

Elija un nivel de acceso cuando cree o edite un entorno:

| Nivel             | Conexiones salientes                                                                                                |
| :---------------- | :------------------------------------------------------------------------------------------------------------------ |
| **Ninguno**       | Sin acceso a la red saliente                                                                                        |
| **Confiable**     | [Dominios en la lista de permitidos](#default-allowed-domains) solo: registros de paquetes, GitHub, SDKs en la nube |
| **Completo**      | Cualquier dominio                                                                                                   |
| **Personalizado** | Su propia lista de permitidos, opcionalmente incluyendo los predeterminados                                         |

Las operaciones de GitHub usan un [proxy separado](#github-proxy) que es independiente de esta configuración.

<h3 id="allow-specific-domains">
  Permita dominios específicos
</h3>

Para permitir dominios que no están en la lista Confiable, seleccione **Personalizado** en la configuración de acceso a la red del entorno. Aparece un campo **Dominios permitidos**. Ingrese un dominio por línea:

```text theme={null}
api.example.com
*.internal.example.com
registry.example.com
```

Use `*.` para coincidencia de subdominio comodín. Marque **También incluir lista predeterminada de administradores de paquetes comunes** para mantener los [dominios Confiables](#default-allowed-domains) junto con sus entradas personalizadas, o déjelo sin marcar para permitir solo lo que enumera.

Los dominios permitidos se configuran por entorno. No hay una lista de permitidos a nivel de organización que los Propietarios puedan enviar a los entornos de todos los usuarios; la [configuración administrada por servidor](/docs/es/server-managed-settings) puede restringir sesiones en la nube pero no puede agregar dominios permitidos.

<h3 id="github-proxy">
  Proxy de GitHub
</h3>

Por seguridad, todas las operaciones de GitHub pasan a través de un servicio de proxy dedicado que mantiene sus credenciales reales de GitHub fuera del sandbox. El proxy autentica dos tipos de tráfico:

* Interacciones de Git: el cliente de git dentro del sandbox usa una credencial de alcance personalizada, que el proxy verifica y traduce a su token de autenticación real de GitHub
* Solicitudes de API de GitHub: el proxy sustituye sus credenciales reales en solicitudes de las herramientas de GitHub integradas, y de `gh` cuando su sesión establece el marcador de posición `proxy-injected` descrito en [Trabajar con problemas y solicitudes de extracción de GitHub](#work-with-github-issues-and-pull-requests)

El proxy también restringe las operaciones de inserción de git a la rama de trabajo actual por seguridad, y permite operaciones de clonación, obtención y PR mientras mantiene límites de seguridad.

El proxy limita las solicitudes de API de GitHub y de activos de lanzamiento a repositorios adjuntos a la sesión, independientemente del [nivel de acceso a la red](#access-levels) del entorno. Los scripts de configuración que descargan activos de lanzamiento de repositorios no adjuntos devuelven un 403. Los archivos confirmados de repositorios públicos se obtienen a través de `raw.githubusercontent.com`, que el [proxy de seguridad](#security-proxy) maneja en su lugar. Ese dominio está en la [lista Confiable](#default-allowed-domains) predeterminada, por lo que los archivos permanecen accesibles a menos que el [nivel de acceso](#access-levels) del entorno lo excluya.

<h3 id="security-proxy">
  Proxy de seguridad
</h3>

Los entornos se ejecutan detrás de un proxy de red HTTP/HTTPS para propósitos de seguridad y prevención de abuso. Todo el tráfico de Internet saliente pasa a través de este proxy, que proporciona:

* Protección contra solicitudes maliciosas
* Limitación de velocidad y prevención de abuso
* Filtrado de contenido para mayor seguridad
* Un registro de auditoría a nivel de DNS de los nombres de host solicitados

<h3 id="default-allowed-domains">
  Dominios permitidos predeterminados
</h3>

Cuando se usa acceso a la red **Confiable**, los siguientes dominios están permitidos de forma predeterminada. Los dominios marcados con `*` indican coincidencia de subdominio comodín, por lo que `*.gcr.io` permite cualquier subdominio de `gcr.io`.

<AccordionGroup>
  <Accordion title="Servicios Anthropic">
    * api.anthropic.com
    * statsig.anthropic.com
    * docs.claude.com
    * platform.claude.com
    * code.claude.com
    * claude.ai
  </Accordion>

  <Accordion title="Control de versiones">
    * github.com
    * [www.github.com](http://www.github.com)
    * api.github.com
    * npm.pkg.github.com
    * raw\.githubusercontent.com
    * pkg-npm.githubusercontent.com
    * objects.githubusercontent.com
    * release-assets.githubusercontent.com
    * codeload.github.com
    * avatars.githubusercontent.com
    * camo.githubusercontent.com
    * gist.github.com
    * gitlab.com
    * [www.gitlab.com](http://www.gitlab.com)
    * registry.gitlab.com
    * bitbucket.org
    * [www.bitbucket.org](http://www.bitbucket.org)
    * api.bitbucket.org
  </Accordion>

  <Accordion title="Registros de contenedores">
    * registry-1.docker.io
    * auth.docker.io
    * index.docker.io
    * hub.docker.com
    * [www.docker.com](http://www.docker.com)
    * production.cloudflare.docker.com
    * download.docker.com
    * gcr.io
    * \*.gcr.io
    * ghcr.io
    * mcr.microsoft.com
    * \*.data.mcr.microsoft.com
    * public.ecr.aws
  </Accordion>

  <Accordion title="Plataformas en la nube">
    * cloud.google.com
    * accounts.google.com
    * gcloud.google.com
    * \*.googleapis.com
    * storage.googleapis.com
    * compute.googleapis.com
    * container.googleapis.com
    * azure.com
    * portal.azure.com
    * microsoft.com
    * [www.microsoft.com](http://www.microsoft.com)
    * \*.microsoftonline.com
    * packages.microsoft.com
    * dotnet.microsoft.com
    * dot.net
    * visualstudio.com
    * dev.azure.com
    * \*.amazonaws.com
    * \*.api.aws
    * oracle.com
    * [www.oracle.com](http://www.oracle.com)
    * java.com
    * [www.java.com](http://www.java.com)
    * java.net
    * [www.java.net](http://www.java.net)
    * download.oracle.com
    * yum.oracle.com
  </Accordion>

  <Accordion title="Administradores de paquetes JavaScript y Node">
    * registry.npmjs.org
    * [www.npmjs.com](http://www.npmjs.com)
    * [www.npmjs.org](http://www.npmjs.org)
    * npmjs.com
    * npmjs.org
    * yarnpkg.com
    * registry.yarnpkg.com
  </Accordion>

  <Accordion title="Administradores de paquetes Python">
    * pypi.org
    * [www.pypi.org](http://www.pypi.org)
    * files.pythonhosted.org
    * pythonhosted.org
    * test.pypi.org
    * pypi.python.org
    * pypa.io
    * [www.pypa.io](http://www.pypa.io)
  </Accordion>

  <Accordion title="Administradores de paquetes Ruby">
    * rubygems.org
    * [www.rubygems.org](http://www.rubygems.org)
    * api.rubygems.org
    * index.rubygems.org
    * ruby-lang.org
    * [www.ruby-lang.org](http://www.ruby-lang.org)
    * rubyforge.org
    * [www.rubyforge.org](http://www.rubyforge.org)
    * rubyonrails.org
    * [www.rubyonrails.org](http://www.rubyonrails.org)
    * rvm.io
    * get.rvm.io
  </Accordion>

  <Accordion title="Administradores de paquetes Rust">
    * crates.io
    * [www.crates.io](http://www.crates.io)
    * index.crates.io
    * static.crates.io
    * rustup.rs
    * static.rust-lang.org
    * [www.rust-lang.org](http://www.rust-lang.org)
  </Accordion>

  <Accordion title="Administradores de paquetes Go">
    * proxy.golang.org
    * sum.golang.org
    * index.golang.org
    * golang.org
    * [www.golang.org](http://www.golang.org)
    * goproxy.io
    * pkg.go.dev
  </Accordion>

  <Accordion title="Administradores de paquetes JVM">
    * maven.org
    * repo.maven.org
    * central.maven.org
    * repo1.maven.org
    * repo.maven.apache.org
    * jcenter.bintray.com
    * gradle.org
    * [www.gradle.org](http://www.gradle.org)
    * services.gradle.org
    * plugins.gradle.org
    * kotlinlang.org
    * [www.kotlinlang.org](http://www.kotlinlang.org)
    * spring.io
    * repo.spring.io
  </Accordion>

  <Accordion title="Otros administradores de paquetes">
    * packagist.org (PHP Composer)
    * [www.packagist.org](http://www.packagist.org)
    * repo.packagist.org
    * nuget.org (.NET NuGet)
    * [www.nuget.org](http://www.nuget.org)
    * api.nuget.org
    * pub.dev (Dart/Flutter)
    * api.pub.dev
    * hex.pm (Elixir/Erlang)
    * [www.hex.pm](http://www.hex.pm)
    * cpan.org (Perl CPAN)
    * [www.cpan.org](http://www.cpan.org)
    * metacpan.org
    * [www.metacpan.org](http://www.metacpan.org)
    * api.metacpan.org
    * cocoapods.org (iOS/macOS)
    * [www.cocoapods.org](http://www.cocoapods.org)
    * cdn.cocoapods.org
    * haskell.org
    * [www.haskell.org](http://www.haskell.org)
    * hackage.haskell.org
    * swift.org
    * [www.swift.org](http://www.swift.org)
  </Accordion>

  <Accordion title="Distribuciones de Linux">
    * archive.ubuntu.com
    * security.ubuntu.com
    * ubuntu.com
    * [www.ubuntu.com](http://www.ubuntu.com)
    * \*.ubuntu.com
    * ppa.launchpad.net
    * launchpad.net
    * [www.launchpad.net](http://www.launchpad.net)
    * \*.nixos.org
  </Accordion>

  <Accordion title="Herramientas de desarrollo y plataformas">
    * dl.k8s.io (Kubernetes)
    * pkgs.k8s.io
    * k8s.io
    * [www.k8s.io](http://www.k8s.io)
    * releases.hashicorp.com (HashiCorp)
    * apt.releases.hashicorp.com
    * rpm.releases.hashicorp.com
    * archive.releases.hashicorp.com
    * hashicorp.com
    * [www.hashicorp.com](http://www.hashicorp.com)
    * repo.anaconda.com (Anaconda/Conda)
    * conda.anaconda.org
    * anaconda.org
    * [www.anaconda.com](http://www.anaconda.com)
    * anaconda.com
    * continuum.io
    * apache.org (Apache)
    * [www.apache.org](http://www.apache.org)
    * archive.apache.org
    * downloads.apache.org
    * eclipse.org (Eclipse)
    * [www.eclipse.org](http://www.eclipse.org)
    * download.eclipse.org
    * nodejs.org (Node.js)
    * [www.nodejs.org](http://www.nodejs.org)
    * developer.apple.com
    * developer.android.com
    * pkg.stainless.com
    * binaries.prisma.sh
  </Accordion>

  <Accordion title="Servicios en la nube y monitoreo">
    * statsig.com
    * [www.statsig.com](http://www.statsig.com)
    * api.statsig.com
    * sentry.io
    * \*.sentry.io
    * downloads.sentry-cdn.com
    * http-intake.logs.datadoghq.com
    * browser-intake-us5-datadoghq.com
    * \*.datadoghq.com
    * \*.datadoghq.eu
    * api.honeycomb.io
  </Accordion>

  <Accordion title="Entrega de contenido y espejos">
    * sourceforge.net
    * \*.sourceforge.net
    * packagecloud.io
    * \*.packagecloud.io
    * fonts.googleapis.com
    * fonts.gstatic.com
  </Accordion>

  <Accordion title="Esquema y configuración">
    * json-schema.org
    * [www.json-schema.org](http://www.json-schema.org)
    * json.schemastore.org
    * [www.schemastore.org](http://www.schemastore.org)
  </Accordion>

  <Accordion title="Protocolo de contexto de modelo">
    * \*.modelcontextprotocol.io
  </Accordion>
</AccordionGroup>

<h2 id="move-tasks-between-web-and-terminal">
  Mover tareas entre web y terminal
</h2>

Estos flujos de trabajo requieren la [CLI de Claude Code](/docs/es/quickstart) conectada a la misma cuenta de claude.ai. Puede iniciar nuevas sesiones en la nube desde su terminal, o extraer sesiones en la nube en su terminal para continuar localmente. Las sesiones en la nube persisten incluso si cierra su portátil, y puede monitorearlas desde cualquier lugar, incluyendo la aplicación móvil Claude.

<Note>
  Desde la CLI, la transferencia de sesión es unidireccional: puede extraer sesiones en la nube en su terminal con `--teleport`, pero no puede insertar una sesión de terminal existente en la web. La bandera `--cloud` crea una nueva sesión en la nube para su repositorio actual. La [aplicación de escritorio](/docs/es/desktop#continue-in-another-surface) proporciona un menú Continuar en que puede enviar una sesión local a la web.
</Note>

<h3 id="from-terminal-to-web">
  De terminal a web
</h3>

Inicie una sesión en la nube desde la línea de comandos con la bandera `--cloud`:

```bash theme={null}
claude --cloud "Fix the authentication bug in src/auth/login.ts"
```

Esto crea una nueva sesión en la nube en claude.ai. La sesión clona el remoto de GitHub de su directorio actual en su rama actual, por lo que inserte primero si tiene confirmaciones locales, ya que la VM clona desde GitHub en lugar de su máquina. `--cloud` funciona con un único repositorio a la vez. La tarea se ejecuta en la nube mientras continúa trabajando localmente. La ortografía anterior `--remote` aún funciona como un alias deprecado para `--cloud`.

A partir de v2.1.195, la CLI muestra una lista de verificación en vivo de pasos de configuración, como clonar el repositorio y ejecutar su [script de configuración](#setup-scripts), mientras se inicia el contenedor en la nube. Los mensajes que escribe mientras se aprovisiona el contenedor se ponen en cola y se envían una vez que la sesión está lista.

<Note>
  `--cloud` crea sesiones en la nube. `--remote-control` no está relacionado: expone una sesión de CLI local para monitoreo desde la web. Consulte [Control Remoto](/docs/es/remote-control).
</Note>

Use `/tasks` en la CLI de Claude Code para verificar el progreso, o abra la sesión en claude.ai o la aplicación móvil Claude para interactuar directamente. Desde allí puede dirigir Claude, proporcionar retroalimentación o responder preguntas como en cualquier otra conversación.

<h4 id="tips-for-cloud-tasks">
  Consejos para tareas en la nube
</h4>

**Planifique localmente, ejecute remotamente**: para tareas complejas, inicie Claude en modo de plan para colaborar en el enfoque, luego envíe el trabajo a la nube:

```bash theme={null}
claude --permission-mode plan
```

En modo de plan, Claude lee archivos, ejecuta comandos para explorar y propone un plan sin editar código fuente. Una vez que esté satisfecho, guarde el plan en el repositorio, comprométalo e insértelo para que la VM en la nube pueda clonarlo. Luego inicie una sesión en la nube para ejecución autónoma:

```bash theme={null}
claude --cloud "Execute the migration plan in docs/migration-plan.md"
```

Este patrón le da control sobre la estrategia mientras permite que Claude ejecute de forma autónoma en la nube.

**Planifique en la nube con ultraplan**: para redactar y revisar el plan en una sesión web, use [ultraplan](/docs/es/ultraplan). Claude genera el plan en Claude Code en la web mientras continúa trabajando, luego comenta sobre secciones en su navegador y elige ejecutar remotamente o enviar el plan de vuelta a su terminal.

**Ejecute tareas en paralelo**: cada comando `--cloud` crea su propia sesión en la nube que se ejecuta de forma independiente. Puede iniciar múltiples tareas y todas se ejecutarán simultáneamente en sesiones separadas:

```bash theme={null}
claude --cloud "Fix the flaky test in auth.spec.ts"
claude --cloud "Update the API documentation"
claude --cloud "Refactor the logger to use structured output"
```

Monitoree todas las sesiones con `/tasks` en la CLI de Claude Code. Cuando una sesión se completa, puede crear una PR desde la interfaz web o [teleportar](#from-web-to-terminal) la sesión a su terminal para continuar trabajando.

<h4 id="send-local-repositories-without-github">
  Envíe repositorios locales sin GitHub
</h4>

Cuando ejecuta `claude --cloud` desde un repositorio que no está conectado a GitHub, Claude Code agrupa su repositorio local y lo carga directamente a la sesión en la nube. El paquete incluye su historial de repositorio completo en todas las ramas, más cualquier cambio sin confirmar en archivos rastreados.

Este respaldo se activa automáticamente cuando el acceso a GitHub no está disponible. Para forzarlo incluso cuando GitHub está conectado, establezca `CCR_FORCE_BUNDLE=1`:

```bash theme={null}
CCR_FORCE_BUNDLE=1 claude --cloud "Run the test suite and fix any failures"
```

Los repositorios agrupados deben cumplir estos límites:

* El directorio debe ser un repositorio de git con al menos una confirmación
* El repositorio agrupado debe ser menor de 100 MB. Los repositorios más grandes se replieguen a agrupar solo la rama actual, luego a una instantánea única comprimida del árbol de trabajo, y fallan solo si la instantánea aún es demasiado grande
* Los archivos sin rastrear no se incluyen; ejecute `git add` en archivos que desea que la sesión en la nube vea
* Las sesiones creadas desde un paquete no pueden insertar de vuelta a un remoto a menos que también tenga [autenticación de GitHub](#github-authentication-options) configurada

<h3 id="from-web-to-terminal">
  De web a terminal
</h3>

Extraiga una sesión en la nube en su terminal usando cualquiera de estos:

* **Usando `--teleport`**: desde la línea de comandos, ejecute `claude --teleport` para un selector de sesión interactivo, o `claude --teleport <session-id>` para reanudar una sesión específica directamente. Si tiene cambios sin confirmar, se le pedirá que los guarde primero.
* **Usando `/teleport`**: dentro de una sesión de CLI existente, ejecute `/teleport` o `/tp` para abrir el mismo selector de sesión sin reiniciar Claude Code.
* **Desde `/tasks`**: ejecute `/tasks` para ver sus sesiones de fondo, luego presione `t` para teleportarse a una.
* **Desde la interfaz web**: seleccione **Abrir en CLI** para copiar un comando que puede pegar en su terminal.

Cuando teleporta una sesión, Claude verifica que esté en el repositorio correcto, obtiene y verifica la rama de la sesión en la nube, y carga el historial de conversación completo en su terminal.

`--teleport` es distinto de `--resume`. `--resume` reabre una conversación del historial local de esta máquina y no enumera sesiones en la nube; `--teleport` extrae una sesión en la nube y su rama.

<h4 id="teleport-requirements">
  Requisitos de teleportación
</h4>

Teleport verifica estos requisitos antes de reanudar una sesión. Si algún requisito no se cumple, verá un error o se le pedirá que resuelva el problema.

| Requisito            | Detalles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Estado de git limpio | Su directorio de trabajo no debe tener cambios sin confirmar. Teleport le pide que guarde los cambios si es necesario.                                                                                                                                                                                                                                                                                                                                                                           |
| Repositorio correcto | Debe ejecutar `--teleport` desde una desprotección del mismo repositorio, no desde una bifurcación. A partir de v2.1.199, Claude Code acepta una desprotección incluso cuando no puede analizar el remoto en un nombre de host, como un alias de host SSH como `git@work:owner/repo.git` o una forma corta reescrita por `insteadOf`. Muestra un mensaje de confirmación primero, y solo cuando el propietario del remoto y el nombre del repositorio coinciden con el repositorio de la sesión. |
| Rama disponible      | La rama de la sesión en la nube debe haber sido insertada en el remoto. Teleport la obtiene y verifica automáticamente.                                                                                                                                                                                                                                                                                                                                                                          |
| Misma cuenta         | Debe estar autenticado en la misma cuenta de claude.ai utilizada en la sesión en la nube.                                                                                                                                                                                                                                                                                                                                                                                                        |

<h4 id="teleport-is-unavailable">
  `--teleport` no está disponible
</h4>

Teleport requiere autenticación de suscripción de claude.ai. Si está autenticado a través de clave de API, Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry, ejecute `/login` para iniciar sesión con su cuenta de claude.ai en su lugar. Si ya está conectado a través de claude.ai y `--teleport` aún no está disponible, su organización puede haber deshabilitado sesiones en la nube.

<h2 id="work-with-sessions">
  Trabajar con sesiones
</h2>

Las sesiones aparecen en la barra lateral en claude.ai/code. Desde allí puede revisar cambios, compartir con compañeros de equipo, archivar trabajo terminado o eliminar sesiones permanentemente.

<h3 id="manage-context">
  Administrar contexto
</h3>

Las sesiones en la nube admiten [comandos integrados](/docs/es/commands) que producen salida de texto. Los comandos que solo se ejecutan en la interfaz de terminal, como `/plugin` o `/resume`, no están disponibles. Los comandos que abren un selector o panel en la terminal se comportan de manera diferente en sesiones en la nube:

* **`/model`, `/effort`, `/fast`, `/color` y `/rename`**: pase el valor como argumento, por ejemplo `/model sonnet`, en lugar de abrir el selector de terminal o el control deslizante. Los formularios de argumento requieren Claude Code v2.1.205 o posterior en el entorno de la sesión y siguen las [notas de disponibilidad](/docs/es/commands#all-commands) de cada comando: `/effort` reporta `Not applied` mientras que el [esfuerzo predeterminado de lanzamiento](/docs/es/model-config#adjust-effort-level) de un modelo está en vigor, y `/fast` funciona solo en una sesión que comenzó con el modo rápido activado.
* **`/config`**: en la web, abre la sección Claude Code de su configuración en lugar de establecer un valor, y el texto después del comando, incluyendo `key=value`, se ignora. Para cambiar la configuración de una sesión en la nube, use [variables de entorno](#configure-your-environment) o confirme [archivos de configuración](/docs/es/settings) en el repositorio.

Para administración de contexto específicamente:

| Comando    | Funciona en sesiones en la nube | Notas                                                                                                                         |
| :--------- | :------------------------------ | :---------------------------------------------------------------------------------------------------------------------------- |
| `/compact` | Sí                              | Resume la conversación para liberar contexto. Acepta instrucciones de enfoque opcionales como `/compact keep the test output` |
| `/context` | Sí                              | Muestra qué está actualmente en la ventana de contexto                                                                        |
| `/clear`   | No                              | Inicie una nueva sesión desde la barra lateral en su lugar                                                                    |

La compactación automática se ejecuta automáticamente cuando la ventana de contexto se acerca a la capacidad. Para activarla antes, establezca [`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`](/docs/es/env-vars) en sus [variables de entorno](#configure-your-environment). Por ejemplo, `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=70` compacta al 70% de capacidad en lugar de esperar hasta que la ventana esté casi llena. Para cambiar el tamaño de ventana efectivo para cálculos de compactación, use [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/es/env-vars).

Los [subagentes](/docs/es/sub-agents) funcionan de la misma manera que lo hacen localmente. Claude puede generarlos con la herramienta Task para descargar investigación o trabajo paralelo en una ventana de contexto separada, manteniendo la conversación principal más ligera. Los subagentes definidos en su `.claude/agents/` del repositorio se recogen automáticamente.

Los [equipos de agentes](/docs/es/agent-teams) están deshabilitados de forma predeterminada pero se pueden habilitar agregando `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` a sus [variables de entorno](#configure-your-environment).

<h3 id="review-changes">
  Revisar cambios
</h3>

Cada sesión muestra un indicador de diferencias con líneas agregadas y eliminadas, como `+42 -18`. Selecciónelo para abrir la vista de diferencias, deje comentarios en línea en líneas específicas y envíelos a Claude con su siguiente mensaje. Consulte [Revisar e iterar](/docs/es/web-quickstart#review-and-iterate) para el tutorial completo incluyendo creación de PR. Para que Claude monitoree la PR para fallos de CI y comentarios de revisión automáticamente, consulte [Correcciones automáticas de solicitudes de extracción](#auto-fix-pull-requests).

<h3 id="share-sessions">
  Compartir sesiones
</h3>

Para compartir una sesión, alterne su visibilidad según los tipos de cuenta a continuación. Después de eso, comparta el enlace de sesión tal como está. Los destinatarios ven el estado más reciente cuando abren el enlace, pero su vista no se actualiza en tiempo real.

<h4 id="share-from-an-enterprise-or-team-account">
  Compartir desde una cuenta Enterprise o Team
</h4>

Para cuentas Enterprise y Team, las dos opciones de visibilidad son **Privada** y **Team**. La visibilidad de Team hace que la sesión sea visible para otros miembros de su organización de claude.ai. Las sesiones de [Claude en Slack](/docs/es/slack) se comparten automáticamente con visibilidad de Team.

La verificación de acceso al repositorio está habilitada de forma predeterminada, según la cuenta de GitHub conectada a la cuenta del destinatario. El nombre para mostrar de su cuenta es visible para todos los destinatarios con acceso.

<h4 id="share-from-a-max-or-pro-account">
  Compartir desde una cuenta Max o Pro
</h4>

Para cuentas Max y Pro, las dos opciones de visibilidad son **Privada** y **Pública**. La visibilidad pública hace que la sesión sea visible para cualquier usuario que haya iniciado sesión en claude.ai.

Verifique su sesión para contenido sensible antes de compartir. Las sesiones pueden contener código y credenciales de repositorios privados de GitHub. La verificación de acceso al repositorio no está habilitada de forma predeterminada.

Para requerir que los destinatarios tengan acceso al repositorio, o para ocultar su nombre de sesiones compartidas, vaya a Configuración > Claude Code > Configuración de uso compartido.

<h3 id="archive-sessions">
  Archivar sesiones
</h3>

Puede archivar sesiones para mantener su lista de sesiones organizada. Las sesiones archivadas se ocultan de la lista de sesiones predeterminada pero se pueden ver filtrando sesiones archivadas.

Para archivar una sesión, pase el cursor sobre la sesión en la barra lateral y seleccione el icono de archivo.

<h3 id="delete-sessions">
  Eliminar sesiones
</h3>

Eliminar una sesión elimina permanentemente la sesión y sus datos. Esta acción no se puede deshacer. Puede eliminar una sesión de dos formas:

* **Desde la barra lateral**: filtre sesiones archivadas, luego pase el cursor sobre la sesión que desea eliminar y seleccione el icono de eliminar
* **Desde el menú de sesión**: abra una sesión, seleccione el menú desplegable junto al título de la sesión y seleccione **Eliminar**

Se le pedirá que confirme antes de que se elimine una sesión.

<h2 id="auto-fix-pull-requests">
  Correcciones automáticas de solicitudes de extracción
</h2>

Claude puede observar una solicitud de extracción y responder automáticamente a fallos de CI y comentarios de revisión. Claude se suscribe a la actividad de GitHub en la PR, y cuando falla una verificación o un revisor deja un comentario, Claude investiga e inserta una corrección si es clara.

<Note>
  Las correcciones automáticas requieren que la aplicación Claude GitHub esté instalada en su repositorio. Si aún no lo ha hecho, instálela desde la [página de la aplicación GitHub](https://github.com/apps/claude) o cuando se le solicite durante la [configuración](/docs/es/web-quickstart#connect-github-and-create-an-environment).
</Note>

Hay algunas formas de activar correcciones automáticas dependiendo de dónde provenga la PR y qué dispositivo esté usando:

* **PRs creadas en Claude Code en la web**: abra la barra de estado de CI y seleccione **Correcciones automáticas**
* **Desde su terminal**: ejecute [`/autofix-pr`](/docs/es/commands) mientras está en la rama de la PR. Claude Code detecta la PR abierta con `gh`, genera una sesión web y activa correcciones automáticas en un paso
* **Desde la aplicación móvil**: dígale a Claude que corrija automáticamente la PR, por ejemplo "observa esta PR y corrige cualquier fallo de CI o comentario de revisión"
* **Cualquier PR existente**: pegue la URL de la PR en una sesión y dígale a Claude que la corrija automáticamente

Las correcciones automáticas son un control por PR. Para dejar de monitorear, abra la barra de estado de CI en la sesión web y desactive el control **Correcciones automáticas**, o dígale a Claude que deje de observar la PR.

<h3 id="how-claude-responds-to-pr-activity">
  Cómo Claude responde a la actividad de PR
</h3>

Cuando las correcciones automáticas están activas, Claude recibe eventos de GitHub para la PR incluyendo nuevos comentarios de revisión y fallos de verificación de CI. Para cada evento, Claude investiga y decide cómo proceder:

* **Correcciones claras**: si Claude está seguro de una corrección y no entra en conflicto con instrucciones anteriores, Claude realiza el cambio, lo inserta y explica qué se hizo en la sesión
* **Solicitudes ambiguas**: si el comentario de un revisor podría interpretarse de múltiples formas o implica algo arquitectónicamente significativo, Claude le pregunta antes de actuar
* **Eventos duplicados o sin acción**: si un evento es un duplicado o no requiere cambio, Claude lo anota en la sesión y continúa

GitHub no emite un webhook cuando la rama base avanza y crea un conflicto de fusión, por lo que las correcciones automáticas no pueden reaccionar a conflictos por sí solas. Para resolver un conflicto, abra la sesión y pídele a Claude que haga un rebase.

Claude puede responder a hilos de comentarios de revisión en GitHub como parte de resolverlos. Estas respuestas se publican usando su cuenta de GitHub, por lo que aparecen bajo su nombre de usuario, pero cada respuesta está etiquetada como proveniente de Claude Code para que los revisores sepan que fue escrita por el agente y no por usted directamente.

<Warning>
  Si su repositorio utiliza automatización activada por comentarios como Atlantis, Terraform Cloud o GitHub Actions personalizadas que se ejecutan en eventos `issue_comment`, tenga en cuenta que Claude puede responder en su nombre, lo que puede activar esos flujos de trabajo. Revise la automatización de su repositorio antes de habilitar correcciones automáticas y considere deshabilitar correcciones automáticas para repositorios donde un comentario de PR puede implementar infraestructura o ejecutar operaciones privilegiadas.
</Warning>

<h2 id="security-and-isolation">
  Seguridad y aislamiento
</h2>

Cada sesión en la nube se separa de su máquina y de otras sesiones a través de varias capas:

* **Máquinas virtuales aisladas**: cada sesión se ejecuta en una VM aislada administrada por Anthropic
* **Controles de acceso a la red**: el acceso a la red se limita de forma predeterminada y puede deshabilitarse. Cuando se ejecuta con acceso a la red deshabilitado, Claude Code aún puede comunicarse con la API de Anthropic, lo que puede permitir que los datos salgan de la VM.
* **Protección de credenciales**: las credenciales sensibles como credenciales de git o claves de firma nunca están dentro del sandbox con Claude Code. La autenticación se maneja a través de un proxy seguro usando credenciales de alcance.
* **Análisis seguro**: el código se analiza y modifica dentro de VMs aisladas antes de crear PRs

<h2 id="troubleshooting">
  Solución de problemas
</h2>

Para errores de API en tiempo de ejecución que aparecen en la conversación como `API Error: 500`, `529 Overloaded`, `429` o `Prompt is too long`, consulte la [referencia de errores](/docs/es/errors). Esos errores y sus soluciones se comparten con la CLI y la aplicación de escritorio. Las secciones a continuación cubren problemas específicos de sesiones en la nube.

<h3 id="session-creation-failed">
  Falló la creación de sesión
</h3>

Si una nueva sesión no se inicia con `Session creation failed` o se detiene en el aprovisionamiento, Claude Code no pudo asignar un entorno en la nube.

* Verifique [status.claude.com](https://status.claude.com) para incidentes de sesión en la nube
* Reintente después de un minuto, ya que la capacidad se aprovisiona bajo demanda
* Confirme que su repositorio es accesible. La cuenta de GitHub que se conecta debe tener acceso al repositorio en GitHub, ya sea a través de la autorización de la aplicación Claude GitHub o un token `gh` sincronizado a través de `/web-setup`. No es necesario instalar la aplicación en el repositorio. Consulte [Opciones de autenticación de GitHub](#github-authentication-options).

<h3 id="remote-control-session-expired-or-access-denied">
  Sesión de Control Remoto expirada o acceso denegado
</h3>

`--teleport` se conecta a través de la misma infraestructura de sesión de Control Remoto que usan las sesiones en la nube, por lo que los errores de autenticación y vencimiento de sesión aparecen con la redacción de Control Remoto. Puede ver `Remote Control session expired` o `Access denied`. El token de conexión es de corta duración y está limitado a su cuenta.

* Ejecute `/login` localmente para actualizar sus credenciales, luego reconecte
* Confirme que está conectado a la misma cuenta que posee la sesión
* Si ve `Remote Control may not be available for this organization`, un propietario no ha habilitado sesiones en la nube para su organización

<h3 id="environment-expired">
  Entorno expirado
</h3>

Las sesiones en la nube se detienen después de un período de inactividad y el entorno subyacente se reclama. Desde una terminal local, esto aparece como `Could not resume session ... its environment has expired. Creating a fresh session instead.` En la web, la sesión está marcada como expirada en la lista de sesiones.

Reabra la sesión desde [claude.ai/code](https://claude.ai/code) para aprovisionar un entorno nuevo con su historial de conversación restaurado.

<h2 id="limitations">
  Limitaciones
</h2>

Antes de confiar en sesiones en la nube para un flujo de trabajo, tenga en cuenta estas restricciones:

* **Límites de velocidad**: Claude Code en la web comparte límites de velocidad con todo otro uso de Claude y Claude Code dentro de su cuenta. Ejecutar múltiples tareas en paralelo consume más límites de velocidad proporcionalmente. No hay cargo de computación separado para la VM en la nube.
* **Autenticación de repositorio**: solo puede mover sesiones de web a local cuando está autenticado en la misma cuenta
* **Restricciones de plataforma**: la clonación de repositorio y la creación de solicitudes de extracción requieren GitHub. Las instancias de [GitHub Enterprise Server](/docs/es/github-enterprise-server) autohospedadas son compatibles con planes de Team y Enterprise. GitLab, Bitbucket y otros repositorios que no sean GitHub se pueden enviar a sesiones en la nube como un [paquete local](#send-local-repositories-without-github), pero la sesión no puede insertar resultados de vuelta al remoto
* **Lista de permitidos de IP de la organización**: las sesiones en la nube llaman a la API de Anthropic desde infraestructura administrada por Anthropic, no desde su red. Si su organización tiene [lista de permitidos de IP](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting) habilitada, cada sesión en la nube falla con un error de autenticación. Lo mismo se aplica a [Revisión de código](/docs/es/code-review) y [Rutinas](/docs/es/routines). Contacte al [soporte de Anthropic](https://support.claude.com/) para eximir los servicios alojados por Anthropic de la lista de permitidos de IP de su organización.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Ultraplan](/docs/es/ultraplan): redacte un plan en una sesión en la nube y revíselo en su navegador
* [Ultrareview](/docs/es/ultrareview): ejecute una revisión de código profunda de múltiples agentes en un sandbox en la nube
* [Routines](/docs/es/routines): automatice el trabajo en un cronograma, a través de llamada de API o en respuesta a eventos de GitHub
* [Configuración de hooks](/docs/es/hooks): ejecute scripts en eventos del ciclo de vida de la sesión
* [Referencia de configuración](/docs/es/settings): todas las opciones de configuración
* [Seguridad](/docs/es/security): garantías de aislamiento y manejo de datos
* [Uso de datos](/docs/es/data-usage): qué retiene Anthropic de sesiones en la nube
* [Claude Tag](https://claude.com/docs/claude-tag/overview): una @Claude administrada por la organización en Slack que se ejecuta en el mismo entorno en la nube
