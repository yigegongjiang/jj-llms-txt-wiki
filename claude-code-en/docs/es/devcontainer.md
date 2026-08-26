> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Contenedores de desarrollo

> Ejecuta Claude Code dentro de un contenedor de desarrollo para entornos consistentes e aislados en todo tu equipo.

Un [contenedor de desarrollo](https://containers.dev/), o dev container, te permite definir un entorno idéntico e aislado que cada ingeniero en tu equipo puede ejecutar. Con Claude Code instalado en ese contenedor, los comandos que Claude ejecuta se ejecutan dentro de él en lugar de en la máquina host, mientras que las ediciones a tus archivos de proyecto aparecen en tu repositorio local mientras trabajas.

Esta página cubre [instalar Claude Code en un contenedor de desarrollo](#add-claude-code-to-your-dev-container), luego un conjunto de temas de configuración independientes: persistir autenticación entre reconstrucciones, aplicar política organizacional, restringir salida de red y ejecutar sin solicitudes de permiso. Lee los que coincidan con tu configuración.

<Warning>
  Aunque el contenedor de desarrollo proporciona protecciones sustanciales, ningún sistema es completamente inmune a todos los ataques.
  Cuando se ejecuta con `--dangerously-skip-permissions`, los contenedores de desarrollo no previenen que un proyecto malicioso exfiltre cualquier cosa accesible dentro del contenedor, incluyendo las credenciales de Claude Code almacenadas en [`~/.claude`](/docs/es/claude-directory).
  Solo usa contenedores de desarrollo cuando desarrolles con repositorios de confianza, y monitorea las actividades de Claude.
  Evita montar secretos del host como `~/.ssh` o archivos de credenciales en la nube en el contenedor; prefiere tokens con alcance de repositorio o de corta duración.
</Warning>

<Accordion title="Cómo funcionan los contenedores de desarrollo con tu editor">
  <img src="https://mintcdn.com/claude-code/YvJyjZfd9yMihr0i/images/devcontainer-architecture.svg?fit=max&auto=format&n=YvJyjZfd9yMihr0i&q=85&s=9017b1d16a446c6cc37ba562f35b9aae" className="dark:hidden" alt="Diagrama que muestra un editor en el host conectándose a un contenedor de desarrollo Docker. Claude Code, la terminal y las herramientas de compilación se ejecutan dentro del contenedor. El repositorio del host está montado en bind en el contenedor como el espacio de trabajo." width="640" height="300" data-path="images/devcontainer-architecture.svg" />

  <img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/devcontainer-architecture-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=a0a340b1f2afc6a590696102c8acaaca" className="hidden dark:block" alt="Diagrama que muestra un editor en el host conectándose a un contenedor de desarrollo Docker. Claude Code, la terminal y las herramientas de compilación se ejecutan dentro del contenedor. El repositorio del host está montado en bind en el contenedor como el espacio de trabajo." width="640" height="300" data-path="images/devcontainer-architecture-dark.svg" />

  Un contenedor de desarrollo se ejecuta como un contenedor Docker, ya sea en tu máquina o en un host en la nube como GitHub Codespaces. Un editor que admita la especificación Dev Containers, como VS Code, GitHub Codespaces, un IDE de JetBrains o Cursor, se conecta a ese contenedor: navegas y editas archivos en el editor como de costumbre, pero la terminal integrada, los servidores de lenguaje y las herramientas de compilación se ejecutan dentro del contenedor en lugar de en tu host. Los editores sin soporte para contenedores de desarrollo, como Vim simple, no son parte de este flujo de trabajo.

  Claude Code se ejecuta dentro del contenedor, por lo que ve los mismos archivos, dependencias y herramientas que el resto de la cadena de herramientas de tu proyecto. En VS Code puedes usar el [panel de extensión de Claude Code](/docs/es/vs-code) o ejecutar `claude` en la terminal integrada; ambos se ejecutan dentro del contenedor y comparten la misma configuración de `~/.claude`.
</Accordion>

<h2 id="add-claude-code-to-your-dev-container">
  Agregar Claude Code a tu contenedor de desarrollo
</h2>

Claude Code se instala en cualquier contenedor de desarrollo a través de la [Característica Claude Code Dev Container](https://github.com/anthropics/devcontainer-features/tree/main/src/claude-code).

La configuración funciona con cualquier herramienta que admita la especificación Dev Containers, como VS Code, GitHub Codespaces o IDEs de JetBrains. Los pasos a continuación usan VS Code como ejemplo.

Cuando abres el contenedor en VS Code o Codespaces, la característica también agrega la extensión Claude Code VS Code; otros editores ignoran esa parte.

<Tip>
  ¿Nuevo en contenedores de desarrollo? El [tutorial de Dev Containers de VS Code](https://code.visualstudio.com/docs/devcontainers/tutorial) te guía a través de la instalación de Docker, la extensión y la apertura de tu primer contenedor. Para un ejemplo más completo y endurecido con un firewall y volúmenes persistentes, consulta [Prueba el contenedor de referencia](#try-the-reference-container).
</Tip>

<Steps>
  <Step title="Crear o actualizar devcontainer.json">
    Guarda lo siguiente como `.devcontainer/devcontainer.json` en tu repositorio, o agrega el bloque `features` a tu archivo existente.

    La etiqueta de versión al final, como `:1.0`, fija el script de instalación de la característica, no la versión de Claude Code. La característica instala la última versión de Claude Code, y Claude Code se actualiza automáticamente dentro del contenedor de forma predeterminada.

    Para fijar la versión de CLI o deshabilitar la actualización automática, consulta [Aplicar política organizacional](#enforce-organization-policy).

    ```json .devcontainer/devcontainer.json theme={null}
    {
      "image": "mcr.microsoft.com/devcontainers/base:ubuntu",
      "features": {
        "ghcr.io/anthropics/devcontainer-features/claude-code:1.0": {}
      }
    }
    ```

    Reemplaza la línea `image` con la imagen base de tu proyecto o elimínala si tu archivo existente usa un Dockerfile.
  </Step>

  <Step title="Reconstruir el contenedor">
    Abre la Paleta de Comandos de VS Code con `Cmd+Shift+P` en Mac o `Ctrl+Shift+P` en Windows y Linux, y ejecuta **Dev Containers: Rebuild Container**.

    Para otras herramientas, sigue la acción de reconstrucción de esa herramienta: consulta [reconstruir en GitHub Codespaces](https://docs.github.com/en/codespaces/developing-in-a-codespace/rebuilding-the-container-in-a-codespace), la [CLI de Dev Containers](https://github.com/devcontainers/cli), o la documentación de contenedor de desarrollo de tu IDE.
  </Step>

  <Step title="Iniciar sesión en Claude Code">
    Abre una terminal en el contenedor reconstruido y ejecuta `claude`, luego sigue la solicitud de autenticación.
  </Step>
</Steps>

Lo que ves en la solicitud de autenticación depende de tu proveedor:

* **Anthropic**: inicia sesión a través de un navegador con tu cuenta de Claude o Anthropic Console
* **[Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry](/docs/es/third-party-integrations)**: Claude Code usa tus credenciales del proveedor de nube, sin solicitud de navegador

Para proveedores de nube, pasa credenciales al contenedor como variables de entorno a través de `containerEnv`, un secreto de Codespaces, o la identidad de carga de trabajo de tu nube en lugar de montar archivos de credenciales desde el host. Consulta [Amazon Bedrock](/docs/es/amazon-bedrock), [Google Cloud's Agent Platform](/docs/es/google-vertex-ai) o [Microsoft Foundry](/docs/es/microsoft-foundry) para la cadena de credenciales que Claude Code lee.

Consulta [Elige tu proveedor de API](/docs/es/admin-setup#choose-your-api-provider) para decidir qué camino se ajusta a tu organización.

<Note>
  Si el inicio de sesión del navegador se completa pero la devolución de llamada nunca llega al contenedor, copia el código mostrado en el navegador y pégalo en la solicitud `Paste code here if prompted` en la terminal. Esto puede suceder cuando el reenvío de puertos del editor no enruta la devolución de llamada de localhost.
</Note>

<h2 id="persist-authentication-and-settings-across-rebuilds">
  Persistir autenticación y configuración entre reconstrucciones
</h2>

De forma predeterminada, el directorio de inicio del contenedor se descarta en la reconstrucción, por lo que los ingenieros deben iniciar sesión nuevamente cada vez. Claude Code almacena su token de autenticación, configuración de usuario e historial de sesión en [`~/.claude`](/docs/es/claude-directory). Monta un volumen nombrado en esa ruta para mantener este estado entre reconstrucciones.

El siguiente ejemplo monta un volumen en el directorio de inicio del usuario `node`:

```json devcontainer.json theme={null}
"mounts": [
  "source=claude-code-config,target=/home/node/.claude,type=volume"
]
```

Reemplaza `/home/node` con el directorio de inicio del `remoteUser` de tu contenedor. Si montas el volumen en algún lugar que no sea `~/.claude`, establece [`CLAUDE_CONFIG_DIR`](/docs/es/env-vars) en la ruta de montaje para que Claude Code lea y escriba allí.

Para aislar el estado por proyecto en lugar de compartir un volumen en todos los repositorios, incluye la variable `${devcontainerId}` en el nombre de la fuente. La [configuración de referencia](https://github.com/anthropics/claude-code/blob/main/.devcontainer/devcontainer.json) usa `source=claude-code-config-${devcontainerId}` para este propósito.

En GitHub Codespaces, `~/.claude` persiste entre detener e iniciar un codespace, pero aún se borra cuando reconstruyes el contenedor, por lo que el montaje de volumen anterior también se aplica allí. Para llevar la autenticación entre codespaces, almacena `ANTHROPIC_API_KEY` o un `CLAUDE_CODE_OAUTH_TOKEN` de [`claude setup-token`](/docs/es/authentication#generate-a-long-lived-token) como un [secreto de Codespaces](https://docs.github.com/en/codespaces/managing-your-codespaces/managing-your-account-specific-secrets-for-github-codespaces); Codespaces hace que los secretos estén disponibles como variables de entorno dentro del contenedor automáticamente.

<h2 id="enforce-organization-policy">
  Aplicar política organizacional
</h2>

Un contenedor de desarrollo es un lugar conveniente para aplicar la política organizacional, porque la misma imagen y configuración se ejecutan en la máquina de cada ingeniero.

Claude Code lee `/etc/claude-code/managed-settings.json` en Linux y lo aplica con la máxima precedencia en la [jerarquía de configuración](/docs/es/settings#how-scopes-interact), por lo que los valores allí anulan cualquier cosa que un ingeniero establezca en `~/.claude` o en el directorio `.claude/` del proyecto. Copia el archivo en su lugar desde tu Dockerfile:

```dockerfile Dockerfile theme={null}
RUN mkdir -p /etc/claude-code
COPY managed-settings.json /etc/claude-code/managed-settings.json
```

Debido a que el Dockerfile vive en el repositorio, cualquiera con acceso de escritura puede cambiar o eliminar este paso. Para la política que los ingenieros no pueden eludir editando archivos del repositorio, entrega la configuración administrada a través de [configuración administrada por servidor](/docs/es/server-managed-settings) o tu MDM en su lugar. Consulta [archivos de configuración administrada](/docs/es/settings#settings-files) para las claves disponibles y las otras rutas de entrega.

Para establecer [variables de entorno](/docs/es/env-vars) que se apliquen a cada sesión de Claude Code en el contenedor, agrégalas a `containerEnv` en tu `devcontainer.json`. El siguiente ejemplo rechaza la telemetría y el informe de errores e impide que Claude Code se actualice automáticamente después de la instalación:

```json devcontainer.json theme={null}
"containerEnv": {
  "CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC": "1",
  "DISABLE_AUTOUPDATER": "1"
}
```

La Característica Dev Container siempre instala la última versión de Claude Code. Para fijar una versión específica de Claude Code para compilaciones reproducibles, instálala desde tu Dockerfile con `npm install -g @anthropic-ai/claude-code@X.Y.Z` en lugar de usar la característica, y establece `DISABLE_AUTOUPDATER` como se muestra arriba.

Para la lista completa de controles de política incluyendo reglas de permiso, restricciones de herramientas y listas blancas de servidores MCP, consulta [Configurar Claude Code para tu organización](/docs/es/admin-setup).

Para hacer que [servidores MCP](/docs/es/mcp) estén disponibles dentro del contenedor, defínelos en [alcance de proyecto](/docs/es/mcp#mcp-installation-scopes) en un archivo `.mcp.json` en la raíz del repositorio para que se verifiquen junto con tu configuración de contenedor de desarrollo. Instala cualquier binario del que dependan los servidores stdio locales en tu Dockerfile, y agrega dominios de servidor remoto a tu lista blanca de red.

<h2 id="restrict-network-egress">
  Restringir salida de red
</h2>

Puedes limitar el tráfico saliente del contenedor solo a los dominios que Claude Code necesita. Consulta [Requisitos de acceso de red](/docs/es/network-config#network-access-requirements) para los dominios de inferencia y autenticación, y [Servicios de telemetría](/docs/es/data-usage#telemetry-services) para las conexiones opcionales de telemetría e informe de errores y cómo deshabilitarlas.

El contenedor de referencia incluye un script [`init-firewall.sh`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/init-firewall.sh) que bloquea todo el tráfico saliente excepto los dominios que Claude Code y tus herramientas de desarrollo necesitan. Ejecutar un firewall dentro de un contenedor requiere permisos adicionales, por lo que la referencia agrega las capacidades `NET_ADMIN` y `NET_RAW` a través de `runArgs`. El script de firewall y estas capacidades no son requeridas para Claude Code en sí: puedes dejarlas fuera y confiar en tus propios controles de red en su lugar.

<h2 id="run-without-permission-prompts">
  Ejecutar sin solicitudes de permiso
</h2>

Debido a que el contenedor ejecuta Claude Code como un usuario no root y confina la ejecución de comandos al contenedor, puedes pasar `--dangerously-skip-permissions` para operación desatendida. La CLI rechaza esta bandera cuando se lanza como root, así que confirma que `remoteUser` está establecido en una cuenta no root.

Omitir solicitudes de permiso elimina tu oportunidad de revisar llamadas de herramientas antes de que se ejecuten. Claude aún puede modificar cualquier archivo en el espacio de trabajo montado en bind, que aparece directamente en tu host, y alcanzar cualquier cosa que la política de red del contenedor permita. Empareja esta bandera con las [restricciones de salida de red](#restrict-network-egress) anteriores para limitar lo que una sesión omitida puede alcanzar.

Si deseas menos solicitudes sin deshabilitar las comprobaciones de seguridad, considera [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) en su lugar, que tiene un clasificador que revisa las acciones antes de que se ejecuten. Para prevenir que los ingenieros usen `--dangerously-skip-permissions` en absoluto, establece `permissions.disableBypassPermissionsMode` en `"disable"` en [configuración administrada](/docs/es/settings#permission-settings).

<h2 id="try-the-reference-container">
  Prueba el contenedor de referencia
</h2>

El repositorio [`anthropics/claude-code`](https://github.com/anthropics/claude-code/tree/main/.devcontainer) incluye un contenedor de desarrollo de ejemplo que combina la CLI, el firewall de salida, volúmenes persistentes y un shell basado en Zsh. Se proporciona como un ejemplo funcional en lugar de una imagen base mantenida; úsalo para ver cómo encajan las piezas antes de aplicarlas a tu propia configuración.

<Steps>
  <Step title="Instalar requisitos previos">
    Instala VS Code y la [extensión Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers).
  </Step>

  <Step title="Clonar la referencia">
    Clona el [repositorio de Claude Code](https://github.com/anthropics/claude-code) y ábrelo en VS Code.
  </Step>

  <Step title="Reabrir en contenedor">
    Cuando se te solicite, haz clic en **Reopen in Container**, o ejecuta **Dev Containers: Reopen in Container** desde la Paleta de Comandos.
  </Step>

  <Step title="Iniciar Claude Code">
    Una vez que el contenedor termine de compilarse, abre una terminal con `` Ctrl+` `` y ejecuta `claude` para iniciar sesión y comenzar tu primera sesión.
  </Step>
</Steps>

Para usar esta configuración con tu propio proyecto, copia el directorio `.devcontainer/` en tu repositorio y ajusta el Dockerfile para tu cadena de herramientas, o vuelve a [Agregar Claude Code a tu contenedor de desarrollo](#add-claude-code-to-your-dev-container) para agregar solo la característica a una configuración que ya tienes.

La configuración de referencia consta de tres archivos. Ninguno de ellos es requerido cuando agregas Claude Code a tu propio contenedor de desarrollo a través de la característica, pero muestran una forma de combinar las piezas.

| Archivo                                                                                                    | Propósito                                                                           |
| ---------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| [`devcontainer.json`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/devcontainer.json) | Montajes de volumen, capacidades `runArgs`, extensiones de VS Code y `containerEnv` |
| [`Dockerfile`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/Dockerfile)               | Imagen base, herramientas de desarrollo e instalación de Claude Code                |
| [`init-firewall.sh`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/init-firewall.sh)   | Bloquea todo el tráfico de red saliente excepto los dominios permitidos             |

<h2 id="next-steps">
  Próximos pasos
</h2>

Una vez que Claude Code se ejecuta en tu contenedor de desarrollo, las páginas a continuación cubren el resto de un despliegue organizacional: elegir una ruta de autenticación, entregar política administrada fuera del repositorio, monitorear el uso y entender qué almacena y envía Claude Code.

* [Configurar Claude Code para tu organización](/docs/es/admin-setup): elige un proveedor de autenticación, decide cómo la política llega a los dispositivos y planifica el despliegue
* [Configuración administrada por servidor](/docs/es/server-managed-settings): entrega política administrada desde la consola de administrador de Claude.ai para que los ingenieros no puedan eludirla editando archivos del repositorio
* [Monitorear el uso y auditar la actividad](/docs/es/monitoring-usage): exporta métricas de OpenTelemetry y revisa lo que tu equipo está ejecutando
* [Requisitos de acceso de red](/docs/es/network-config#network-access-requirements): la lista completa de dominios para proxies y firewalls
* [Servicios de telemetría y opción de exclusión](/docs/es/data-usage#telemetry-services): qué envía Claude Code de forma predeterminada y las variables de entorno que lo deshabilitan
* [Explorar el directorio `.claude`](/docs/es/claude-directory): qué contiene el montaje de volumen, incluyendo credenciales, configuración e historial de sesión
* [Entornos sandbox](/docs/es/sandbox-environments): compara contenedores de desarrollo con el sandbox Bash integrado, contenedores personalizados y máquinas virtuales
* [Modelo de seguridad](/docs/es/security): cómo encajan el sistema de permisos de Claude Code, el sandboxing y las protecciones contra inyección de solicitudes
* [Modos de permiso](/docs/es/permission-modes): el rango completo desde modo de plan hasta modo automático hasta omisión, y cuándo usar cada uno
