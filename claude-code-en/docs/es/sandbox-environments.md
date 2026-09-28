> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Elegir un entorno sandbox

> Compare las opciones de sandbox de Claude Code: la herramienta Bash aislada integrada, el tiempo de ejecución sandbox, contenedores de desarrollo, Docker y máquinas virtuales. Elija el aislamiento adecuado para su modelo de amenaza.

Aislar Claude Code limita lo que una sesión puede leer, escribir y alcanzar en la red. Esto es más importante cuando permite que Claude trabaje con menos solicitudes de permiso, lo ejecuta sin supervisión o lo apunta a código en el que no confía completamente.

Claude Code puede ejecutarse en varios tipos de entornos aislados, que van desde un sandbox ligero por comando hasta una máquina virtual completamente separada. Esta página compara estos entornos por lo que aíslan y lo que requieren, le ayuda a elegir uno para su modelo de amenaza, y muestra cómo aplicar esa opción en toda una organización.

<Info>
  Para el modelo de seguridad más amplio, consulte [Seguridad](/docs/es/security). Para implementaciones de Agent SDK, consulte [Implementación segura](/docs/es/agent-sdk/secure-deployment).
</Info>

<h2 id="compare-sandboxing-approaches">
  Comparar enfoques de sandboxing
</h2>

Los dos primeros enfoques en la tabla siguiente se ejecutan en el sistema operativo host sin contenedores. El resto coloca Claude Code dentro de un contenedor o máquina virtual.

| Enfoque                                            | Qué se aísla                                                                                  | Requiere Docker | Esfuerzo de configuración                                                                                           |
| :------------------------------------------------- | :-------------------------------------------------------------------------------------------- | :-------------- | :------------------------------------------------------------------------------------------------------------------ |
| [Herramienta Bash sandboxed](#sandboxed-bash-tool) | Comandos Bash, PowerShell y Monitor y sus procesos secundarios                                | No              | Mínimo en macOS; bajo en Linux y WSL2                                                                               |
| [Sandbox runtime](#sandbox-runtime)                | Todo el proceso de Claude Code, incluidas las herramientas de archivo, servidores MCP y hooks | No              | Bajo                                                                                                                |
| [Dev container](#dev-containers)                   | Entorno de desarrollo completo                                                                | Sí              | Medio                                                                                                               |
| [Contenedor personalizado](#custom-container)      | Entorno de desarrollo completo                                                                | Sí              | Medio a alto                                                                                                        |
| [Máquina virtual](#virtual-machine)                | Sistema operativo completo                                                                    | No              | Alto                                                                                                                |
| [Cloud sessions](#cloud-sessions)                  | Sistema operativo completo, alojado por Anthropic                                             | No              | Ninguno; requiere una suscripción a Claude y una cuenta de GitHub conectada a menos que inicie con `claude --cloud` |

La [herramienta Bash sandboxed](/docs/es/sandboxing) está integrada en Claude Code y restringe los comandos Bash. Las herramientas de archivo integradas, los servidores MCP y los hooks aún se ejecutan directamente en su host. Todos los demás enfoques en la tabla colocan todo el proceso de Claude Code dentro del límite de aislamiento, por lo que las herramientas de archivo, los servidores MCP y los hooks también se restringen.

<Warning>
  El aislamiento de sandbox reduce el impacto de una brecha, pero no elimina el riesgo. Cualquier enfoque que permita egreso de red aún puede filtrar datos que el agente puede leer, y cualquier enfoque que monte su directorio de proyecto escribible aún puede modificar ese código. Revise las [limitaciones de seguridad](/docs/es/sandboxing#security-limitations) antes de confiar en un sandbox como un control estricto.

  El aislamiento tampoco cambia lo que se envía al modelo. Sus indicaciones y los archivos que Claude lee se transmiten a la API de Anthropic o a su proveedor configurado con o sin un sandbox. Consulte [Uso de datos](/docs/es/data-usage) para saber qué envía Claude Code y cómo reducirlo.
</Warning>

<h2 id="choose-an-approach">
  Elegir un enfoque
</h2>

Haga coincidir su objetivo con una fila a continuación, luego lea la sección de detalle que sigue.

| Usted quiere                                                                                       | Comience con                                                                                                                                                                     |
| :------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Reducir solicitudes de permiso durante el trabajo diario en su propia máquina                      | La [herramienta Bash aislada](/docs/es/sandboxing), configurada con `/sandbox`                                                                                                        |
| Permitir que Claude trabaje sin supervisión con `--dangerously-skip-permissions` o modo automático | El [contenedor de desarrollo](/docs/es/devcontainer) preconfigurado, cualquier contenedor o máquina virtual, o el [tiempo de ejecución sandbox](#sandbox-runtime)                     |
| Aislar servidores MCP y hooks así como Bash, sin Docker                                            | El tiempo de ejecución sandbox                                                                                                                                                   |
| Trabajar en un repositorio no confiable                                                            | Una máquina virtual dedicada, o una [sesión en la nube](/docs/es/claude-code-on-the-web) si tiene una suscripción a Claude; GitHub no es necesario cuando inicia con `claude --cloud` |
| Estandarizar un entorno aislado en un equipo                                                       | El [contenedor de desarrollo](/docs/es/devcontainer) preconfigurado, copiado en su repositorio                                                                                        |
| Usar Claude Code desde un dispositivo sin configuración local                                      | Una [sesión en la nube](/docs/es/claude-code-on-the-web), que requiere una suscripción a Claude y una cuenta de GitHub conectada                                                      |
| Requerir aislamiento para cada desarrollador en su organización                                    | [Aplicar aislamiento en toda la organización](#enforce-isolation-across-an-organization)                                                                                         |
| Trabajar en un host Windows nativo                                                                 | Un contenedor o máquina virtual, o ejecutar el sandbox Bash dentro de WSL2                                                                                                       |

<h3 id="how-isolation-relates-to-permission-modes">
  Cómo se relaciona el aislamiento con los modos de permiso
</h3>

Los [modos de permiso](/docs/es/permission-modes) deciden si se ejecuta una llamada de herramienta y si se le solicita primero. El aislamiento restringe lo que un comando puede acceder una vez que se ejecuta. Los dos funcionan juntos: cuando un modo de permiso permite que las acciones se ejecuten sin preguntarle, un límite de aislamiento limita lo que esas acciones pueden alcanzar.

Cuando pasa `--dangerously-skip-permissions`, Claude actúa sin preguntarle primero. Las [acciones que ningún modo aprueba automáticamente](/docs/es/permission-modes#actions-no-mode-auto-approves) aún se aplican.

Sin solicitudes para detectar errores, el límite de aislamiento que elija es lo que protege su sistema. Siempre ejecute sesiones `--dangerously-skip-permissions` dentro de un contenedor, una máquina virtual, o el [tiempo de ejecución sandbox](#sandbox-runtime), para que las herramientas de archivo, los servidores MCP y los hooks también estén dentro del límite. En Linux y macOS, Claude Code se niega a iniciarse con esta marca cuando se ejecuta como root, así que ejecute el contenedor, la máquina virtual, o el tiempo de ejecución sandbox como un usuario no root.

El [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) reemplaza la solicitud con un clasificador que revisa acciones. El clasificador es un control por acción, no un límite de aislamiento, por lo que un límite de aislamiento aún agrega defensa en profundidad para ejecuciones sin supervisión, y no es requerido como lo es para `--dangerously-skip-permissions`.

La [herramienta Bash aislada](#sandboxed-bash-tool) por sí sola restringe solo comandos shell, por lo que no es suficiente para ejecuciones completamente sin supervisión en ninguno de los modos. Puede superponer enfoques: ejecutar la herramienta Bash aislada dentro de un contenedor o máquina virtual le da restricciones de comando a nivel de SO además del límite del entorno externo. Para cómo el sandbox Bash en sí interactúa con reglas de permiso y modos de permiso, consulte [Cómo el sandboxing se relaciona con permisos y modos de permiso](/docs/es/sandboxing#how-sandboxing-relates-to-permissions-and-permission-modes).

<h2 id="sandboxed-bash-tool">
  Herramienta Bash aislada
</h2>

<Note>
  Esta opción no admite Windows nativo. En hosts Windows, use WSL2 o uno de los enfoques de contenedor o máquina virtual a continuación.
</Note>

La herramienta Bash aislada está integrada en Claude Code. Utiliza primitivas del sistema operativo para restringir el acceso al sistema de archivos y la red de cada comando Bash, PowerShell o Monitor que ejecuta Claude.

Ejecute el comando `/sandbox` para abrir el panel de sandbox y elegir un modo. La guía [Sandboxing](/docs/es/sandboxing) cubre los modos de aprobación, el límite predeterminado y cómo ampliarlo o estrecharlo.

El sandbox por comando no cubre todo lo que se ejecuta en una sesión:

* Otras [herramientas integradas](/docs/es/tools-reference) como Read, Edit y WebFetch se ejecutan dentro del proceso de Claude Code y no generan código arbitrario. Las [reglas de permiso](/docs/es/permissions) para ruta o dominio las controlan en su lugar.
* Los servidores [MCP](/docs/es/mcp) y [hooks de comando](/docs/es/hooks#command-hook-fields) son procesos separados que se ejecutan sin restricciones en el host.

Para poner herramientas integradas, servidores MCP y hooks todos detrás de un límite de SO, ejecute todo el proceso de Claude Code dentro del [tiempo de ejecución sandbox](#sandbox-runtime), el [contenedor de desarrollo](#dev-containers) o un [contenedor personalizado](#custom-container).

<h2 id="sandbox-runtime">
  Tiempo de ejecución sandbox
</h2>

El paquete [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime) envuelve un proceso completo en el mismo aislamiento Seatbelt o bubblewrap que usa el sandbox Bash integrado. Ejecutar Claude Code a través del tiempo de ejecución restringe cada herramienta, hook y servidor MCP en la sesión, no solo comandos de shell. El tiempo de ejecución es una vista previa de investigación beta, y su formato de configuración puede cambiar a medida que el paquete evoluciona.

Esta sección cubre qué configura usted y qué el tiempo de ejecución aplica por su cuenta. Para implementar el tiempo de ejecución en aplicaciones del Agent SDK, consulte la [guía de implementación segura](/docs/es/agent-sdk/secure-deployment#sandbox-runtime).

<h3 id="set-up-and-launch-the-runtime">
  Configurar e iniciar el tiempo de ejecución
</h3>

En Linux y WSL2, el tiempo de ejecución depende de los mismos paquetes `bubblewrap` y `socat` que usa el sandbox integrado, más `ripgrep`, que Claude Code incluye pero el tiempo de ejecución independiente resuelve desde su PATH. Instale `bubblewrap` y `socat` como se describe en [Configurar Linux y WSL2](/docs/es/sandboxing#set-up-linux-and-wsl2), y `ripgrep` desde el gestor de paquetes de su distribución. En macOS no necesita paquetes adicionales. El tiempo de ejecución usa el sandbox Seatbelt integrado allí.

Por defecto el tiempo de ejecución niega acceso de red y confina escrituras a un pequeño conjunto de rutas de tiempo de ejecución integradas, así que configúrelo antes de lanzar Claude Code a través de él. Coloque su configuración en `~/.srt-settings.json`, o en un archivo que pase con `--settings`. El [README](https://github.com/anthropic-experimental/sandbox-runtime) del paquete documenta el esquema de configuración completo.

Permita acceso de escritura a al menos:

* Su directorio de proyecto.
* Las rutas de configuración de Claude Code `~/.claude` y `~/.claude.json`.
* `/tmp`, donde Claude Code escribe archivos de tiempo de ejecución.

Permita los dominios de red que su sesión necesita:

* `api.anthropic.com`, o el punto final de su proveedor configurado. En un proveedor de terceros, mantenga `api.anthropic.com` también: la verificación de seguridad del dominio WebFetch aún lo llama por defecto a menos que establezca `skipWebFetchPreflight: true`.
* `claude.ai` y `platform.claude.com`, que requieren [inicio de sesión OAuth y actualización de token](/docs/es/network-config#network-access-requirements). Las ejecuciones autenticadas con una clave API pueden descartar estos dos.

En Linux y WSL2, el tiempo de ejecución aplica permisos de escritura solo a rutas que ya existen. En un entorno nuevo, cree las rutas de configuración de Claude Code antes del primer lanzamiento:

```bash theme={null}
mkdir -p ~/.claude && echo '{}' > ~/.claude.json
```

Una vez que el archivo de configuración esté en su lugar, lance Claude Code con `npx` y pase `claude` como el comando a envolver:

```bash theme={null}
npx @anthropic-ai/sandbox-runtime claude
```

Claude Code se inicia dentro del sandbox con los límites de sistema de archivos y red que configuró. El mismo comando funciona para aislar servidores MCP independientes u otros procesos auxiliares.

<h3 id="what-the-runtime-blocks-on-its-own">
  Qué bloquea el tiempo de ejecución por su cuenta
</h3>

El tiempo de ejecución bloquea las escrituras de mayor riesgo sin ninguna configuración de su parte:

* `denyWrite` tiene precedencia sobre `allowWrite`.
* En la raíz del proyecto, el tiempo de ejecución niega `.git/hooks`, niega `.git/config` a menos que establezca `filesystem.allowGitConfig: true`, y niega `.mcp.json`, `.claude/commands`, `.claude/agents`, y archivos de inicio de shell.
* En macOS, estas negaciones se verifican cuando ocurre una escritura, así que también cubren archivos anidados y repositorios creados durante la sesión.
* En Linux y WSL2, el tiempo de ejecución construye la lista de negación una vez al lanzamiento. Cubre de manera confiable la raíz del proyecto, realiza un escaneo superficial de mejor esfuerzo para copias anidadas que existen en ese momento, y no cubre nada que la sesión cree después, como `git init`, `git clone`, o scaffolding. La sección `mandatoryDenySearchDepth` del README describe la semántica exacta del escaneo.
* Sin un `~/.srt-settings.json` válido, el tiempo de ejecución se inicia de todas formas, bloquea acceso de red, y confina escrituras a rutas de tiempo de ejecución integradas como `/tmp/claude`, `~/.npm/_logs`, y `~/.claude/debug`. No tome un inicio limpio como prueba de que su configuración se cargó.
* Cuando pasa `--settings`, el tiempo de ejecución se niega a iniciar si el archivo falla al cargar.

Sus permisos de escritura aún incluyen otras rutas desde las que Claude Code carga configuración, así que niegue esas con `denyWrite`. Una sesión en sandbox que puede escribirlas puede persistir hooks, reglas de permiso, o servidores MCP que se ejecutan sin sandbox la próxima vez que lance Claude Code.

<h3 id="after-unattended-runs">
  Después de ejecuciones desatendidas
</h3>

Revise las rutas que mantuvo escribibles. En Linux y WSL2, también revise cualquier cosa que la sesión creó.

<h2 id="dev-containers">
  Contenedores de desarrollo
</h2>

Un contenedor de desarrollo ejecuta Claude Code dentro de un contenedor Docker que VS Code o un editor compatible gestiona, con su proyecto montado. Puede definir el suyo propio con un directorio `.devcontainer/` en su repositorio.

El repositorio claude-code publica un [contenedor de desarrollo de ejemplo](/docs/es/devcontainer) con un firewall iptables de negación predeterminada como punto de partida. Cópielo en su repositorio y ajuste la lista de permitidos del firewall, la imagen base y la versión de Claude Code fijada para que se ajuste a su entorno. Debido a que el firewall bloquea el egreso no aprobado, una configuración como esta admite ejecutar Claude Code con `--dangerously-skip-permissions` para trabajo sin supervisión.

<h2 id="custom-container">
  Contenedor personalizado
</h2>

Puede ejecutar Claude Code en cualquier imagen de contenedor Docker u OCI con sus propias políticas de red, volúmenes montados y perfiles seccomp. Este es el camino más común para organizaciones con infraestructura de contenedor existente o ejecutores de CI.

Varios servicios de sandbox administrados y ejecución remota pueden alojar el contenedor para usted. La misma lista de verificación se aplica como para cualquier contenedor que opere: revise qué está montado escribible, qué credenciales y tokens son accesibles dentro de él y qué permite la política de egreso de red.

Puede superponer el sandbox Bash integrado dentro del contenedor para restricciones por comando. Los contenedores sin privilegios necesitan la configuración de sandbox anidado descrita en [Solución de problemas de Sandboxing](/docs/es/sandboxing#troubleshooting).

<h2 id="virtual-machine">
  Máquina virtual
</h2>

Una máquina virtual dedicada proporciona la separación más fuerte, con su propio kernel y, en implementaciones en la nube o microVM, su propio hardware virtualizado. Las opciones incluyen instancias en la nube, hipervisores locales y microVMs como Firecracker. Use este enfoque cuando esté evaluando código no confiable, cuando su política de seguridad requiera separación a nivel de kernel entre el agente y el host, o cuando ningún enfoque a nivel de host cumpla con sus requisitos de cumplimiento.

[Docker Sandboxes](https://docs.docker.com/ai/sandboxes/) proporciona una microVM con su propio daemon de Docker y sincronización de espacio de trabajo, que puede ejecutar Claude Code en cualquier host con Docker Sandboxes instalado. Es un producto gratuito e independiente de Docker que no requiere Docker Desktop.

<h2 id="cloud-sessions">
  Sesiones en la nube
</h2>

Una [sesión en la nube](/docs/es/claude-code-on-the-web) se ejecuta en una máquina virtual aislada administrada por Anthropic. Un proxy de red aplica una lista de permitidos predeterminada, y un proxy separado mantiene su token de GitHub fuera del sandbox mientras emite credenciales con alcance para acceso al repositorio dentro de él. Las sesiones que su organización enruta a un [entorno autohospedado](/docs/es/self-hosted-environments) se ejecutan en la infraestructura que usted aprovisiona en su lugar, donde el aislamiento, el control de salida y las credenciales de git son responsabilidad de su implementación.

Utilice este enfoque cuando desee aislamiento completo de máquina virtual sin aprovisionar infraestructura usted mismo, o cuando esté delegando tareas desde un dispositivo que no tiene un entorno de desarrollo local. Requiere una suscripción a Claude. A menos que inicie desde la CLI, también necesita una cuenta de GitHub conectada para que el sandbox pueda clonar su repositorio. Cuando inicia desde la CLI con `--cloud`, Claude Code puede [agrupar y cargar su repositorio local](/docs/es/claude-code-on-the-web#send-local-repositories-without-github) en su lugar. Consulte [Usar Claude Code en la nube](/docs/es/claude-code-on-the-web) para disponibilidad de planes y opciones de autenticación de GitHub.

<h2 id="enforce-isolation-across-an-organization">
  Aplicar aislamiento en toda la organización
</h2>

Los desarrolladores individuales pueden optar por cualquiera de los enfoques de sandboxing en esta página. Lo que una organización puede aplicar, y con qué herramientas, depende del enfoque:

* **Sandbox Bash integrado**: el único enfoque que Claude Code aplica a sí mismo. Entregue las claves de configuración `sandbox` a través de [configuración administrada](/docs/es/managed-settings#delivery-mechanisms), ya sea como un archivo administrado por su MDM o a través de [configuración administrada por servidor](/docs/es/server-managed-settings) en Claude.ai. Consulte [Aplicar sandboxing con configuración administrada](/docs/es/sandboxing#enforce-sandboxing-with-managed-settings) para las claves a implementar y cómo evitar que los desarrolladores amplíen la política.
* **Contenedores de desarrollo**: confirme el [contenedor de desarrollo de ejemplo](/docs/es/devcontainer) en sus repositorios para estandarizar el entorno en un equipo. Esta es una convención en lugar de un límite de aplicación, porque Claude Code no requiere un contenedor. Si los desarrolladores no deberían poder ejecutar Claude Code fuera de él, aplique eso con las herramientas de administración de dispositivos de su organización o herramientas de lista de permitidos de software.
* **Contenedores personalizados y máquinas virtuales**: distribuya Claude Code a través de la imagen aprobada y use las herramientas de administración de dispositivos de su organización o herramientas de lista de permitidos de software para evitar la instalación fuera de ella.

<h2 id="see-also">
  Ver también
</h2>

Estas páginas cubren detalles de configuración y política para los enfoques de sandboxing en esta página.

* [Sandboxing](/docs/es/sandboxing): configure la herramienta Bash aislada integrada
* [Contenedor de desarrollo](/docs/es/devcontainer): el contenedor de desarrollo Docker preconfigurado
* [Seguridad](/docs/es/security): el modelo de seguridad completo de Claude Code
* [Implementación segura](/docs/es/agent-sdk/secure-deployment): orientación de aislamiento para aplicaciones de Agent SDK
* [Configuración](/docs/es/settings-reference#sandbox-settings): todas las claves de configuración de sandbox, incluida la entrega de configuración administrada
