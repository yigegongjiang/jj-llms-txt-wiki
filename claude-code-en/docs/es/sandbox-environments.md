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

Los dos primeros enfoques en la tabla a continuación se ejecutan en el sistema operativo host sin contenedores. El resto coloca Claude Code dentro de un contenedor o máquina virtual.

| Enfoque                                          | Qué se aísla                                                                              | Requiere Docker | Esfuerzo de configuración                           |
| :----------------------------------------------- | :---------------------------------------------------------------------------------------- | :-------------- | :-------------------------------------------------- |
| [Herramienta Bash aislada](#sandboxed-bash-tool) | Comandos Bash y sus procesos secundarios                                                  | No              | Mínimo en macOS; bajo en Linux y WSL2               |
| [Tiempo de ejecución sandbox](#sandbox-runtime)  | Todo el proceso de Claude Code, incluidas herramientas de archivo, servidores MCP y hooks | No              | Bajo                                                |
| [Contenedor de desarrollo](#dev-containers)      | Entorno de desarrollo completo                                                            | Sí              | Medio                                               |
| [Contenedor personalizado](#custom-container)    | Entorno de desarrollo completo                                                            | Sí              | Medio a alto                                        |
| [Máquina virtual](#virtual-machine)              | Sistema operativo completo                                                                | No              | Alto                                                |
| [Claude Code en la web](#claude-code-on-the-web) | Sistema operativo completo, alojado por Anthropic                                         | No              | Ninguno; requiere una suscripción a Claude y GitHub |

La [herramienta Bash aislada](/docs/es/sandboxing) está integrada en Claude Code y restringe solo comandos Bash. Las herramientas de archivo integradas, los servidores MCP y los hooks aún se ejecutan directamente en su host. Todos los demás enfoques en la tabla colocan todo el proceso de Claude Code dentro del límite de aislamiento, por lo que las herramientas de archivo, los servidores MCP y los hooks también están restringidos.

<Warning>
  El aislamiento sandbox reduce el impacto de una violación, pero no elimina el riesgo. Cualquier enfoque que permita egreso de red aún puede filtrar datos que el agente puede leer, y cualquier enfoque que monte su directorio de proyecto escribible aún puede modificar ese código. Revise las [limitaciones de seguridad](/docs/es/sandboxing#security-limitations) antes de confiar en un sandbox como un control duro.

  El aislamiento tampoco cambia lo que se envía al modelo. Sus indicaciones y los archivos que Claude lee se transmiten a la API de Anthropic o a su proveedor configurado con o sin sandbox. Consulte [Uso de datos](/docs/es/data-usage) para ver qué envía Claude Code y cómo reducirlo.
</Warning>

<h2 id="choose-an-approach">
  Elegir un enfoque
</h2>

Haga coincidir su objetivo con una fila a continuación, luego lea la sección de detalle que sigue.

| Usted quiere                                                                                       | Comience con                                                                                                                                                 |
| :------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Reducir solicitudes de permiso durante el trabajo diario en su propia máquina                      | La [herramienta Bash aislada](/docs/es/sandboxing), habilitada con `/sandbox`                                                                                     |
| Permitir que Claude trabaje sin supervisión con `--dangerously-skip-permissions` o modo automático | El [contenedor de desarrollo](/docs/es/devcontainer) preconfigurado, cualquier contenedor o máquina virtual, o el [tiempo de ejecución sandbox](#sandbox-runtime) |
| Aislar servidores MCP y hooks así como Bash, sin Docker                                            | El tiempo de ejecución sandbox                                                                                                                               |
| Trabajar en un repositorio no confiable                                                            | Una máquina virtual dedicada, o [Claude Code en la web](/docs/es/claude-code-on-the-web) si tiene una suscripción a Claude y una cuenta de GitHub conectada       |
| Estandarizar un entorno aislado en un equipo                                                       | El [contenedor de desarrollo](/docs/es/devcontainer) preconfigurado, copiado en su repositorio                                                                    |
| Usar Claude Code desde un dispositivo sin configuración local                                      | [Claude Code en la web](/docs/es/claude-code-on-the-web), que requiere una suscripción a Claude y una cuenta de GitHub conectada                                  |
| Requerir aislamiento para cada desarrollador en su organización                                    | [Aplicar aislamiento en toda la organización](#enforce-isolation-across-an-organization)                                                                     |
| Trabajar en un host Windows nativo                                                                 | Un contenedor o máquina virtual, o ejecutar el sandbox Bash dentro de WSL2                                                                                   |

<h3 id="how-isolation-relates-to-permission-modes">
  Cómo se relaciona el aislamiento con los modos de permiso
</h3>

Los [modos de permiso](/docs/es/permission-modes) deciden si se ejecuta una llamada de herramienta y si se le solicita primero. El aislamiento restringe lo que un comando puede acceder una vez que se ejecuta. Los dos funcionan juntos: cuando un modo de permiso permite que las acciones se ejecuten sin preguntarle, un límite de aislamiento limita lo que esas acciones pueden alcanzar.

Cuando pasa `--dangerously-skip-permissions`, Claude actúa sin preguntarle primero; solo se le solicita para [reglas de solicitud](/docs/es/permissions#manage-permissions) explícitas, herramientas de conector [que su organización configuró para `ask`](/docs/es/mcp#organization-controls-on-connector-tools), herramientas MCP marcadas [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool), y eliminaciones dirigidas a `/` o su directorio de inicio. Sin solicitudes para detectar errores, el límite de aislamiento que elija es lo que protege su sistema. Siempre ejecute sesiones `--dangerously-skip-permissions` dentro de un contenedor, una máquina virtual, o el [tiempo de ejecución sandbox](#sandbox-runtime), para que las herramientas de archivo, los servidores MCP y los hooks también estén dentro del límite.

El [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) reemplaza la solicitud con un clasificador que revisa acciones y bloquea las que escalan más allá de la solicitud, apuntan a infraestructura no reconocida o parecen impulsadas por contenido hostil que Claude leyó. El clasificador es un control por acción, no un límite de aislamiento, por lo que un límite de aislamiento aún agrega defensa en profundidad para ejecuciones sin supervisión, y no es requerido como lo es para `--dangerously-skip-permissions`.

La [herramienta Bash aislada](#sandboxed-bash-tool) por sí sola restringe solo Bash, por lo que no es suficiente para ejecuciones completamente sin supervisión en ninguno de los modos. Puede superponer enfoques: ejecutar la herramienta Bash aislada dentro de un contenedor o máquina virtual le da restricciones de comando a nivel de SO además del límite del entorno externo. Para cómo el sandbox Bash en sí interactúa con reglas de permiso y modos de permiso, consulte [Cómo el sandboxing se relaciona con permisos y modos de permiso](/docs/es/sandboxing#how-sandboxing-relates-to-permissions-and-permission-modes).

<h2 id="sandboxed-bash-tool">
  Herramienta Bash aislada
</h2>

<Note>
  Esta opción no admite Windows nativo. En hosts Windows, use WSL2 o uno de los enfoques de contenedor o máquina virtual a continuación.
</Note>

La herramienta Bash aislada está integrada en Claude Code. Utiliza primitivas del sistema operativo para restringir el acceso al sistema de archivos y la red de cada comando Bash que ejecuta Claude: Seatbelt, el sandbox integrado de macOS, y [bubblewrap](https://github.com/containers/bubblewrap) en Linux y WSL2. Por defecto, permite escrituras en el directorio de trabajo y solicita la primera vez que un comando necesita un nuevo dominio de red.

Habilítelo con el comando `/sandbox`. La guía [Sandboxing](/docs/es/sandboxing) cubre los modos de aprobación, el límite predeterminado y cómo ampliarlo o estrecharlo.

El sandbox por comando no cubre todo lo que se ejecuta en una sesión:

* Otras [herramientas integradas](/docs/es/tools-reference) como Read, Edit y WebFetch se ejecutan dentro del proceso de Claude Code y no generan código arbitrario. Las [reglas de permiso](/docs/es/permissions) para ruta o dominio las controlan en su lugar.
* Los servidores [MCP](/docs/es/mcp) y hooks son procesos separados que se ejecutan sin restricciones en el host.

Para poner herramientas integradas, servidores MCP y hooks todos detrás de un límite de SO, ejecute todo el proceso de Claude Code dentro del [tiempo de ejecución sandbox](#sandbox-runtime), el [contenedor de desarrollo](#dev-containers) o un [contenedor personalizado](#custom-container).

<h2 id="sandbox-runtime">
  Tiempo de ejecución sandbox
</h2>

El paquete [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime) envuelve un proceso completo en el mismo aislamiento Seatbelt o bubblewrap que usa el sandbox Bash integrado. Ejecutar Claude Code a través de él restringe cada herramienta, hook y servidor MCP en la sesión, no solo Bash. El tiempo de ejecución es una vista previa de investigación beta, y su formato de configuración puede cambiar a medida que el paquete evoluciona.

El tiempo de ejecución niega todo acceso de escritura y red por defecto, así que configúrelo antes de lanzar Claude Code a través de él. En `~/.srt-settings.json`, o un archivo que pase con `--settings`, permita acceso de escritura a al menos su directorio de proyecto y las rutas de configuración de Claude Code `~/.claude` y `~/.claude.json`. Permita los dominios de red que su sesión necesita, incluido `api.anthropic.com` o el punto final de su proveedor configurado. Consulte el [README](https://github.com/anthropic-experimental/sandbox-runtime) del paquete para el esquema de configuración completo.

Una vez que el archivo de configuración esté en su lugar, lance Claude Code con `npx` y pase `claude` como el comando a envolver:

```bash theme={null}
npx @anthropic-ai/sandbox-runtime claude
```

Claude Code se inicia dentro del sandbox con los límites de sistema de archivos y red que configuró. El mismo comando funciona para aislar servidores MCP independientes u otros procesos auxiliares.

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

Una máquina virtual dedicada proporciona la separación más fuerte, con su propio kernel y, en implementaciones en la nube o microVM, su propio hardware virtualizado. Las opciones incluyen instancias en la nube, hipervisores locales y microVMs como Firecracker.

Use este enfoque cuando esté evaluando código no confiable, cuando su política de seguridad requiera separación a nivel de kernel entre el agente y el host, o cuando ningún enfoque a nivel de host cumpla con sus requisitos de cumplimiento. La [característica de sandboxes](https://docs.docker.com/ai/sandboxes/) de Docker Desktop proporciona una microVM con su propio daemon de Docker y sincronización de espacio de trabajo, que puede ejecutar Claude Code en hosts que ya tienen Docker Desktop.

<h2 id="claude-code-on-the-web">
  Claude Code en la web
</h2>

[Claude Code en la web](/docs/es/claude-code-on-the-web) ejecuta cada sesión en una máquina virtual aislada y administrada por Anthropic. Un proxy de red aplica una lista de permitidos predeterminada, y un proxy separado mantiene su token de GitHub fuera del sandbox mientras emite credenciales con alcance para acceso al repositorio dentro de él.

Use este enfoque cuando desee aislamiento completo de máquina virtual sin aprovisionar infraestructura usted mismo, o cuando esté delegando tareas desde un dispositivo que no tiene un entorno de desarrollo local. Requiere una suscripción a Claude y una cuenta de GitHub conectada, y las sesiones clonan su repositorio desde GitHub. Consulte [Claude Code en la web](/docs/es/claude-code-on-the-web) para disponibilidad de planes y opciones de autenticación de GitHub.

<h2 id="enforce-isolation-across-an-organization">
  Aplicar aislamiento en toda la organización
</h2>

Los desarrolladores individuales pueden optar por cualquiera de los enfoques anteriores. Lo que una organización puede aplicar, y con qué herramientas, depende del enfoque:

* **Sandbox Bash integrado**: el único enfoque que Claude Code aplica a sí mismo. Entregue las claves de configuración `sandbox` a través de [configuración administrada](/docs/es/settings#settings-files), ya sea como un archivo administrado por su MDM o a través de [configuración administrada por servidor](/docs/es/server-managed-settings) en Claude.ai. Consulte [Aplicar sandboxing con configuración administrada](/docs/es/sandboxing#enforce-sandboxing-with-managed-settings) para las claves a implementar y cómo evitar que los desarrolladores amplíen la política.
* **Contenedores de desarrollo**: confirme el [contenedor de desarrollo de ejemplo](/docs/es/devcontainer) en sus repositorios para estandarizar el entorno en un equipo. Esta es una convención en lugar de un límite de aplicación, porque Claude Code no requiere un contenedor. Si los desarrolladores no deberían poder ejecutar Claude Code fuera de él, aplique eso con las herramientas de administración de dispositivos de su organización o herramientas de lista de permitidos de software.
* **Contenedores personalizados y máquinas virtuales**: distribuya Claude Code a través de la imagen aprobada y use las herramientas de administración de dispositivos de su organización o herramientas de lista de permitidos de software para evitar la instalación fuera de ella.

<h2 id="see-also">
  Ver también
</h2>

Estas páginas cubren detalles de configuración y política para los enfoques anteriores.

* [Sandboxing](/docs/es/sandboxing): configure la herramienta Bash aislada integrada
* [Contenedor de desarrollo](/docs/es/devcontainer): el contenedor de desarrollo Docker preconfigurado
* [Seguridad](/docs/es/security): el modelo de seguridad completo de Claude Code
* [Implementación segura](/docs/es/agent-sdk/secure-deployment): orientación de aislamiento para aplicaciones de Agent SDK
* [Configuración](/docs/es/settings#sandbox-settings): todas las claves de configuración de sandbox, incluida la entrega de configuración administrada
