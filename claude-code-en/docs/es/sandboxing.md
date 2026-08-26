> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurar la herramienta Bash aislada

> Aprenda cómo la herramienta Bash aislada de Claude Code proporciona aislamiento del sistema de archivos y la red para una ejecución de agentes más segura y autónoma.

El sandbox de Bash permite que Claude ejecute la mayoría de comandos de shell sin detenerse para pedir permiso. En lugar de aprobar cada comando, usted define qué archivos y dominios de red pueden tocar los comandos, y el sistema operativo aplica ese límite para cada comando Bash y sus procesos secundarios.

<Note>
  Para comparar otros enfoques de aislamiento como contenedores de desarrollo, contenedores personalizados y máquinas virtuales, consulte [Entornos sandbox](/docs/es/sandbox-environments). Para reducir solicitudes de permiso para herramientas distintas de Bash, consulte [modos de permiso](/docs/es/permission-modes).
</Note>

<h2 id="get-started">
  Primeros pasos
</h2>

El sandbox está integrado en Claude Code y se ejecuta en macOS, Linux y WSL2. Windows nativo no es compatible. En Windows, ejecute Claude Code dentro de una distribución WSL2.

En macOS, no hay nada que instalar: el sandboxing utiliza el marco Seatbelt integrado. En Linux y WSL2, el sandbox se basa en dos paquetes, cubiertos en [Configurar Linux y WSL2](#set-up-linux-and-wsl2). Incluso si aún no los ha instalado, puede comenzar con `/sandbox`, porque su panel muestra si falta algo.

<Steps>
  <Step title="Ejecutar /sandbox">
    Inicie una sesión de Claude Code y ejecute el comando `/sandbox`:

    ```text theme={null}
    /sandbox
    ```

    Esto abre el panel de sandbox con tres pestañas:

    * **Mode**: elija cómo se aprueban los comandos aislados, cubierto en el siguiente paso
    * **Overrides**: elija si los comandos que fallan bajo el sandbox pueden volver a ejecutarse sin aislar. Esta es la configuración [`allowUnsandboxedCommands`](/docs/es/settings#sandbox-settings)
    * **Config**: vea la configuración de sandbox resuelta

    Si el panel muestra solo una pestaña Dependencies, falta un paquete requerido. Instálelo como se describe en [Configurar Linux y WSL2](#set-up-linux-and-wsl2), reinicie Claude Code y ejecute `/sandbox` nuevamente.
  </Step>

  <Step title="Elegir un modo">
    En la pestaña Mode, seleccione auto-allow o permisos regulares. Auto-allow ejecuta comandos aislados sin solicitar, y permisos regulares mantiene las solicitudes de permiso regulares incluso cuando los comandos están aislados. Consulte [Modos de sandbox](#sandbox-modes) para ver qué comandos aún solicitan en modo auto-allow.
  </Step>

  <Step title="Ejecutar un comando Bash">
    Pida a Claude que ejecute un comando, como una compilación o un conjunto de pruebas. De forma predeterminada, los comandos dentro del sandbox solo pueden escribir en el directorio de trabajo y el directorio temporal de la sesión. La primera vez que un comando necesita un nuevo dominio de red, Claude Code solicita aprobación.

    Los comandos que no pueden ejecutarse aislados vuelven al flujo de permiso regular. Para ampliar o reducir estos límites, consulte [Configurar sandboxing](#configure-sandboxing).
  </Step>
</Steps>

Seleccionar un modo en el panel escribe en la configuración local de su proyecto en `.claude/settings.local.json`, que se aplica al proyecto actual y no se verifica en git. Para habilitar el sandbox en todos sus proyectos, establezca [`sandbox.enabled`](/docs/es/settings#sandbox-settings) en `true` en su configuración de usuario en `~/.claude/settings.json`. Para aplicar sandboxing para cada desarrollador en una organización, use [configuración administrada](#enforce-sandboxing-with-managed-settings).

<Warning>
  De forma predeterminada, si el sandbox no puede iniciarse porque faltan dependencias o la plataforma no es compatible, Claude Code muestra una advertencia y ejecuta comandos sin sandboxing. Para hacer que esto sea un error grave en su lugar, establezca [`sandbox.failIfUnavailable`](/docs/es/settings#sandbox-settings) en `true`. Esto está destinado a implementaciones administradas que requieren sandboxing como una puerta de seguridad.
</Warning>

<h3 id="set-up-linux-and-wsl2">
  Configurar Linux y WSL2
</h3>

En Linux y WSL2, el sandbox se basa en dos paquetes:

* [`bubblewrap`](https://github.com/containers/bubblewrap): la herramienta de sandboxing sin privilegios que aplica aislamiento del sistema de archivos
* [`socat`](http://www.dest-unreach.org/socat/): el relé utilizado para enrutar el tráfico de red a través del proxy del sandbox

Instálelos con el gestor de paquetes de su distribución:

<Tabs>
  <Tab title="Ubuntu/Debian">
    ```bash theme={null}
    sudo apt-get install bubblewrap socat
    ```
  </Tab>

  <Tab title="Fedora">
    ```bash theme={null}
    sudo dnf install bubblewrap socat
    ```
  </Tab>
</Tabs>

Después de instalar, la pestaña Dependencies en `/sandbox` muestra si `ripgrep`, `bubblewrap`, `socat` y el filtro seccomp están disponibles en su plataforma. Ripgrep se incluye con el binario nativo de Claude Code. El filtro seccomp es opcional y agrega bloqueo de socket de dominio Unix. Instálelo con `npm install -g @anthropic-ai/sandbox-runtime` si falta.

Cuando falta una dependencia requerida, la pestaña Dependencies es la única pestaña mostrada hasta que la instale. La verificación de dependencia se ejecuta al inicio, así que reinicie Claude Code después de instalar paquetes para que `/sandbox` los detecte.

<AccordionGroup>
  <Accordion title="Ubuntu 24.04 y posterior: permitir que bubblewrap cree espacios de nombres de usuario">
    En Ubuntu 24.04 y posterior, la política de AppArmor predeterminada impide que bubblewrap cree los espacios de nombres de usuario que necesita para el aislamiento.

    Para verificar si su entorno aplica esta restricción, incluso dentro de WSL2, ejecute `sysctl kernel.apparmor_restrict_unprivileged_userns`. Si la clave no existe o devuelve `0`, omita este paso. Si devuelve `1`, agregue un perfil de AppArmor que otorgue a `bwrap` esta capacidad:

    ```bash theme={null}
    sudo tee /etc/apparmor.d/bwrap > /dev/null <<'EOF'
    abi <abi/4.0>,
    include <tunables/global>

    profile bwrap /usr/bin/bwrap flags=(unconfined) {
      userns,
      include if exists <local/bwrap>
    }
    EOF
    ```

    El perfil se aplica solo a `bwrap` en sí, no a los comandos que se ejecutan dentro del sandbox. Recargue AppArmor para aplicarlo:

    ```bash theme={null}
    sudo systemctl reload apparmor
    ```
  </Accordion>

  <Accordion title="Notas de WSL2">
    Verifique su versión de WSL con `wsl -l -v` desde PowerShell. Si ve `Sandboxing requires WSL2`, su distribución está ejecutando WSL1. Actualícela a WSL2 o ejecute Claude Code sin sandboxing.

    En WSL2, los comandos aislados no pueden lanzar binarios de Windows como `cmd.exe`, `powershell.exe` o cualquier cosa bajo `/mnt/c/`. WSL entrega estos al host de Windows a través de un socket Unix, que el sandbox bloquea. Si un comando necesita invocar un binario de Windows, agréguelo a [`excludedCommands`](/docs/es/settings#sandbox-settings) para que se ejecute fuera del sandbox.
  </Accordion>
</AccordionGroup>

<h3 id="sandbox-modes">
  Modos de sandbox
</h3>

Claude Code ofrece dos modos de sandbox:

**Modo auto-allow**: Los comandos Bash intentarán ejecutarse dentro del sandbox y se permitirán automáticamente sin requerir permiso. Los comandos que no pueden ser aislados, como aquellos que necesitan acceso a la red a hosts no permitidos, vuelven al flujo de permiso regular, donde Claude Code verifica sus [reglas de permiso](/docs/es/permissions) y le solicita cualquier comando que esas reglas no permitan, con una solicitud en modo predeterminado o el clasificador en [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode).

Incluso en modo auto-allow, lo siguiente sigue siendo válido:

* Las [reglas de denegación](/docs/es/permissions) explícitas siempre se respetan
* Los comandos `rm` o `rmdir` que apunten a `/`, su directorio de inicio u otras rutas críticas del sistema aún desencadenan una solicitud de permiso
* Las [reglas de solicitud](/docs/es/permissions) con alcance de contenido como `Bash(git push *)` aún fuerzan una solicitud incluso para comandos aislados
* Una regla de solicitud `Bash` simple, o la forma equivalente `Bash(*)`, se omite para comandos que se ejecutan aislados; aún se aplica a comandos que vuelven al flujo de permiso regular

**Modo de permisos regulares**: Todos los comandos Bash pasan por el flujo de permiso regular, incluso cuando están aislados. Esto proporciona más control pero requiere más aprobaciones.

En ambos modos, el sandbox aplica las mismas restricciones de sistema de archivos y red. La diferencia es solo si los comandos aislados se aprueban automáticamente o requieren permiso explícito.

El directorio temporal de la sesión es escribible dentro del sandbox de forma predeterminada, junto con el directorio de trabajo. Claude Code establece `$TMPDIR` en este directorio para comandos aislados, por lo que las herramientas que escriben archivos temporales funcionan sin configuración adicional. Los comandos no aislados heredan su `$TMPDIR` de shell sin cambios, lo que significa que los comandos aislados y no aislados resuelven `$TMPDIR` a directorios diferentes. Para pasar archivos temporales entre los dos, escríbalos en el directorio de trabajo en su lugar.

Algunos comandos no pueden ejecutarse dentro del sandbox en absoluto, como herramientas que son incompatibles con él o que necesitan un host que no ha permitido. En lugar de fallar la tarea o requerirle que apague el sandboxing, Claude Code incluye una salida de emergencia: cuando un comando falla debido a restricciones del sandbox, Claude analiza la falla y puede reintentar el comando con el parámetro `dangerouslyDisableSandbox`. El comando reintentado se ejecuta fuera del sandbox, por lo que pasa por el flujo de permiso regular: en modo predeterminado obtiene una solicitud de confirmación; en [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) el clasificador evalúa el comando subyacente en lugar de solicitarle. Para que se le solicite en cada reintento sin sandbox incluso en modo automático, agregue una [regla de solicitud](/docs/es/permissions#match-by-input-parameter) para `Bash(dangerouslyDisableSandbox:true)`.

Puede deshabilitar esta salida de emergencia estableciendo `"allowUnsandboxedCommands": false` en su [configuración de sandbox](/docs/es/settings#sandbox-settings). Cuando está deshabilitado, que la pestaña Overrides de `/sandbox` muestra como **Modo de sandbox estricto**, el parámetro `dangerouslyDisableSandbox` se ignora completamente y todos los comandos deben ejecutarse aislados o estar explícitamente listados en `excludedCommands`.

<Info>
  El modo auto-allow funciona independientemente de su configuración de modo de permiso. Incluso si no está en modo "aceptar ediciones", los comandos Bash aislados se ejecutarán automáticamente cuando auto-allow esté habilitado. Esto significa que los comandos Bash que modifican archivos dentro de los límites del sandbox se ejecutarán sin solicitar, incluso cuando las herramientas de edición de archivos normalmente requerirían aprobación.
</Info>

<h2 id="configure-sandboxing">
  Configurar sandboxing
</h2>

Personalice el comportamiento del sandbox a través de su archivo `settings.json`. Consulte [Configuración](/docs/es/settings#sandbox-settings) para obtener la referencia de configuración completa.

De forma predeterminada, los comandos aislados solo pueden escribir en el directorio de trabajo actual y el directorio temporal de la sesión. Si comandos de subproceso como `kubectl`, `terraform` o `npm` necesitan escribir fuera de esos directorios, use `sandbox.filesystem.allowWrite` para otorgar acceso a rutas específicas:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "allowWrite": ["~/.kube", "/tmp/build"]
    }
  }
}
```

Estas rutas se aplican a nivel del sistema operativo, por lo que todos los comandos que se ejecutan dentro del sandbox, incluidos sus procesos secundarios, las respetan. Este es el enfoque recomendado cuando una herramienta necesita acceso de escritura a una ubicación específica, en lugar de excluir la herramienta del sandbox por completo con `excludedCommands`.

Cuando el mismo array del sistema de archivos se define en múltiples [ámbitos de configuración](/docs/es/settings#settings-precedence), los arrays se fusionan: las rutas de cada ámbito se combinan, no se reemplazan.

Los prefijos de ruta controlan cómo se resuelven las rutas:

| Prefijo            | Significado                                                                                                   | Ejemplo                                                                     |
| :----------------- | :------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------- |
| `/`                | Ruta absoluta desde la raíz del sistema de archivos                                                           | `/tmp/build` se mantiene como `/tmp/build`                                  |
| `~/`               | Relativo al directorio de inicio                                                                              | `~/.kube` se convierte en `$HOME/.kube`                                     |
| `./` o sin prefijo | Relativo a la raíz del proyecto para configuración de proyecto, o a `~/.claude` para configuración de usuario | `./output` en `.claude/settings.json` se resuelve a `<project-root>/output` |

Esta sintaxis difiere de las [reglas de permiso Read y Edit](/docs/es/permissions#read-and-edit), que usan `//path` para absoluto y `/path` para relativo al proyecto. Las rutas del sistema de archivos del sandbox usan convenciones estándar: `/tmp/build` es absoluto.

También puede denegar acceso de escritura o lectura usando `sandbox.filesystem.denyWrite` y `sandbox.filesystem.denyRead`, y permitir nuevamente rutas específicas dentro de una región denegada usando `sandbox.filesystem.allowRead`. Cuando las reglas de lectura se superponen, la ruta más específica gana:

| Reglas de ejemplo                                      | Resultado                                                                                                                                                                                                                  |
| :----------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"denyRead": ["~/"]` con `"allowRead": ["~/projects"]` | `~/projects` es legible y el resto del directorio de inicio permanece bloqueado. El permiso más estrecho reabre esa parte de la región denegada                                                                            |
| `"allowRead": ["~/"]` con `"denyRead": ["~/.env"]`     | `~/.env` permanece bloqueado y el resto del directorio de inicio es legible. Una denegación exacta se mantiene dentro de un permiso más amplio, por lo que un permiso amplio no puede reexponer silenciosamente un secreto |

El ejemplo a continuación bloquea la lectura de todo el directorio de inicio mientras aún permite lecturas del proyecto actual. Colóquelo en el `.claude/settings.json` de su proyecto, porque la ruta relativa `.` se resuelve a la raíz del proyecto solo cuando la configuración se encuentra en la configuración del proyecto:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "denyRead": ["~/"],
      "allowRead": ["."]
    }
  }
}
```

El `.` en `allowRead` se resuelve a la raíz del proyecto porque esta configuración se encuentra en la configuración del proyecto. Si colocara la misma configuración en `~/.claude/settings.json`, `.` se resolvería a `~/.claude` en su lugar, y los archivos del proyecto permanecerían bloqueados por la regla `denyRead`.

<h3 id="protect-credentials">
  Proteger credenciales
</h3>

La configuración `sandbox.credentials` declara archivos de credenciales y variables de entorno a proteger de los comandos aislados. Cada entrada nombra una ruta de archivo o una variable de entorno y un `mode`. El bloque dedicado `credentials` mantiene las reglas de credenciales agrupadas juntas y separadas de las reglas generales del sistema de archivos. Requiere Claude Code v2.1.187 o posterior.

Para entradas con `"mode": "deny"`, las rutas de archivo se deniegan para lecturas dentro del sandbox, la misma restricción que aplica `filesystem.denyRead`, y las variables de entorno se desactivan antes de que se ejecute cada comando aislado.

El ejemplo a continuación bloquea las lecturas del archivo de credenciales de AWS y el directorio SSH y elimina `GITHUB_TOKEN` y `NPM_TOKEN` del entorno de los comandos aislados:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "credentials": {
      "files": [
        { "path": "~/.aws/credentials", "mode": "deny" },
        { "path": "~/.ssh", "mode": "deny" }
      ],
      "envVars": [
        { "name": "GITHUB_TOKEN", "mode": "deny" },
        { "name": "NPM_TOKEN", "mode": "deny" }
      ]
    }
  }
}
```

Las entradas de archivo admiten solo `"mode": "deny"`. Las entradas de variables de entorno también aceptan `"mode": "mask"`, descrito a continuación.

Las rutas de archivo siguen las mismas [reglas de prefijo](/docs/es/settings#sandbox-path-prefixes) que las configuraciones `sandbox.filesystem.*`, y las entradas `deny` de cada [ámbito de configuración](/docs/es/settings#settings-precedence) se fusionan. Una entrada `deny` solo estrecha el acceso, por lo que cualquier ámbito puede agregar una, pero ningún ámbito puede eliminar una que otro ámbito haya agregado.

No hay una lista de denegación de credenciales integrada, por lo que solo los archivos y variables que enumere están restringidos. La configuración afecta solo a los comandos Bash aislados. Para eliminar las credenciales de Anthropic y del proveedor de nube de todos los subprocesos independientemente del sandboxing, establezca [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/es/env-vars).

<h4 id="mask-environment-variables">
  Enmascarar variables de entorno
</h4>

`"mode": "mask"` protege una credencial mientras mantiene funcionando las herramientas que se autentican con ella. `deny` elimina la variable por completo, lo que también rompe las herramientas que la necesitan, como `gh` o `npm`. Requiere Claude Code v2.1.199 o posterior.

Con `mask`, el comando aislado ve un valor centinela por sesión en lugar del real. Cuando una solicitud sale del sandbox para uno de los `injectHosts` de la credencial, el [proxy del sandbox](#network-isolation) reemplaza el centinela con el valor real. El comando y cualquier cosa que registre nunca contienen la credencial real, pero sus solicitudes aún se autentican.

El proxy sustituye la credencial dentro del contenido de la solicitud, por lo que tiene que verla. Establezca [`network.tlsTerminate`](/docs/es/settings#sandbox-settings) para que el proxy termine TLS por sí mismo. Sin él, el enmascaramiento falla cerrado: el comando aún ve solo el centinela, pero el centinela llega al servidor sin cambios y la autenticación falla. Claude Code reporta esta configuración incorrecta al inicio.

El ejemplo a continuación enmascara dos tokens. `GH_TOKEN` se sustituye solo en solicitudes a `api.github.com`, mientras que `NPM_TOKEN` no tiene `injectHosts` y se sustituye en solicitudes a cada host en `network.allowedDomains`. Cada entrada `injectHosts` debe estar cubierta por `network.allowedDomains`.

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "network": {
      "tlsTerminate": {},
      "allowedDomains": ["*.github.com", "registry.npmjs.org"]
    },
    "credentials": {
      "envVars": [
        { "name": "GH_TOKEN", "mode": "mask", "injectHosts": ["api.github.com"] },
        { "name": "NPM_TOKEN", "mode": "mask" }
      ]
    }
  }
}
```

A diferencia de `deny`, el enmascaramiento autoriza al proxy a enviar su credencial real a los hosts enumerados, por lo que se honra solo desde configuraciones que usted o su administrador controlan: configuración de usuario, configuración administrada y la bandera CLI `--settings`. Las entradas `mask`, `network.tlsTerminate` y [`credentials.allowPlaintextInject`](/docs/es/settings#sandbox-settings) en el `.claude/settings.json` o `.claude/settings.local.json` de un repositorio se ignoran.

Cuando la misma variable se enumera con `deny` en cualquier ámbito, `deny` tiene prioridad.

<h2 id="how-sandboxing-works">
  Cómo funciona el sandboxing
</h2>

<h3 id="filesystem-isolation">
  Aislamiento del sistema de archivos
</h3>

La herramienta Bash aislada restringe el acceso al sistema de archivos a directorios específicos:

* **Comportamiento de escritura predeterminado**: acceso de lectura y escritura al directorio de trabajo actual y sus subdirectorios, más el directorio temporal de sesión al que apunta `$TMPDIR`
* **Comportamiento de lectura predeterminado**: acceso de lectura a toda la computadora, excepto ciertos directorios denegados. Tenga en cuenta que este comportamiento predeterminado aún permite leer archivos de credenciales como `~/.aws/credentials` y `~/.ssh/`. Utilice [`sandbox.credentials`](#protect-credentials) para bloquear lecturas de estos archivos y desconfigurar variables de entorno secretas, o agregue las rutas a `denyRead`.
* **Acceso bloqueado**: no puede modificar archivos fuera del directorio de trabajo actual y el directorio temporal de sesión sin permiso explícito, incluidos archivos de configuración de shell como `~/.bashrc` y binarios del sistema en `/bin/`
* **Git worktrees**: cuando el directorio de trabajo es un [git worktree vinculado](/docs/es/worktrees), el sandbox también permite escrituras en el directorio compartido `.git` del repositorio principal para que comandos como `git commit` puedan actualizar referencias e índices. Las escrituras a `hooks/` y `config` dentro de ese directorio permanecen denegadas.
* **Configurable**: defina rutas permitidas y denegadas personalizadas a través de la configuración

Puede otorgar acceso de escritura a rutas adicionales usando `sandbox.filesystem.allowWrite` en su configuración. Estas restricciones se aplican a nivel del sistema operativo, por lo que se aplican a todos los comandos de subproceso, incluidas herramientas como `kubectl`, `terraform` y `npm`, no solo a las herramientas de archivo de Claude.

<h3 id="network-isolation">
  Aislamiento de red
</h3>

El acceso a la red se controla a través de un servidor proxy que se ejecuta fuera del sandbox:

* **Restricciones de dominio**: no hay dominios pre-permitidos. La primera vez que un comando necesita un nuevo dominio, Claude Code solicita aprobación. A partir de v2.1.191, elegir Sí permite el host para el resto de la sesión actual, por lo que las conexiones posteriores al mismo host no solicitan nuevamente. Pre-permita dominios con [`allowedDomains`](/docs/es/settings#sandbox-settings) para evitar la solicitud por completo.
* **Bloqueo administrado**: si [`allowManagedDomainsOnly`](/docs/es/settings#sandbox-settings) se establece en la configuración administrada, los dominios no permitidos se bloquean automáticamente en lugar de solicitar, y solo se honran `allowedDomains` de la configuración administrada.
* **Soporte de proxy personalizado**: los usuarios avanzados pueden implementar reglas personalizadas en el tráfico saliente
* **Cobertura integral**: las restricciones se aplican a todos los scripts, programas y subprocesos generados por comandos

<Note>
  El proxy integrado aplica la lista de permitidos basada en el nombre de host solicitado y, de forma predeterminada, no termina ni inspecciona el tráfico TLS. La configuración experimental [`network.tlsTerminate`](/docs/es/settings#sandbox-settings), disponible en Claude Code v2.1.199 y posterior, hace que el proxy integrado termine TLS por sí mismo, lo que requieren las entradas de credenciales [`mask`](#protect-credentials). Consulte [Limitaciones de seguridad](#security-limitations) para las implicaciones del comportamiento predeterminado, y [Configuración de proxy personalizado](#custom-proxy-configuration) si su modelo de amenaza requiere inspección de TLS.
</Note>

<h3 id="os-level-enforcement">
  Aplicación a nivel del sistema operativo
</h3>

La herramienta Bash aislada aprovecha las primitivas de seguridad del sistema operativo:

* **macOS**: utiliza Seatbelt para la aplicación del sandbox
* **Linux**: utiliza [bubblewrap](https://github.com/containers/bubblewrap) para el aislamiento
* **WSL2**: utiliza bubblewrap, igual que Linux

WSL1 no es compatible porque bubblewrap requiere características del kernel solo disponibles en WSL2. Estas restricciones a nivel del sistema operativo garantizan que todos los procesos secundarios generados por los comandos de Claude Code hereden los mismos límites de seguridad.

Estas mismas primitivas están disponibles como el paquete independiente [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime), que la página [Entornos sandbox](/docs/es/sandbox-environments#sandbox-runtime) cubre como un enfoque separado para envolver todo el proceso de Claude Code.

<h2 id="how-sandboxing-relates-to-permissions-and-permission-modes">
  Cómo se relaciona el sandboxing con los permisos y modos de permiso
</h2>

El sandboxing, las [reglas de permiso](/docs/es/permissions) y los [modos de permiso](/docs/es/permission-modes) son capas complementarias. Las secciones a continuación cubren cómo el sandbox interactúa con cada una.

<h3 id="permission-rules">
  Reglas de permiso
</h3>

Las reglas de permiso y el sandboxing controlan cosas diferentes:

* **Reglas de permiso** controlan qué herramientas puede usar Claude Code y se evalúan antes de que se ejecute cualquier herramienta. Se aplican a todas las herramientas: Bash, Read, Edit, WebFetch, MCP y otras.
* **Sandboxing** proporciona aplicación a nivel del sistema operativo que restringe lo que los comandos Bash pueden acceder a nivel del sistema de archivos y la red. Se aplica solo a comandos Bash y sus procesos secundarios.

Las dos capas también difieren en cómo se aplican. Claude Code evalúa las decisiones de permiso antes de que se ejecute un comando, basándose en la cadena de comando y, en modo automático, el juicio de un clasificador separado sobre si el comando es seguro. El sistema operativo aplica el límite del sandbox en el proceso en ejecución, por lo que se mantiene independientemente de lo que el modelo eligió ejecutar e incluso si un comando permitido hace más de lo que su nombre sugiere.

Las restricciones del sistema de archivos y la red se configuran tanto a través de la configuración del sandbox como de las reglas de permiso:

| Configuración o regla                                          | Qué hace                                                                                                         |
| :------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------- |
| `sandbox.filesystem.allowWrite`                                | Otorga acceso de escritura de subproceso a rutas fuera del directorio de trabajo                                 |
| `sandbox.filesystem.denyWrite` y `sandbox.filesystem.denyRead` | Bloquea el acceso de subproceso a rutas específicas                                                              |
| `sandbox.filesystem.allowRead`                                 | Permite nuevamente la lectura de rutas específicas dentro de una región `denyRead`                               |
| Reglas de permitir `Edit`                                      | Otorga acceso de escritura a rutas específicas, de la misma manera que `sandbox.filesystem.allowWrite`           |
| Reglas de denegar `Read` y `Edit`                              | Bloquea el acceso a archivos o directorios específicos                                                           |
| Reglas de permitir y denegar `WebFetch`                        | Controla el acceso al dominio                                                                                    |
| `allowedDomains` del sandbox                                   | Controla qué dominios pueden alcanzar los comandos Bash                                                          |
| `deniedDomains` del sandbox                                    | Bloquea dominios específicos incluso cuando un comodín `allowedDomains` más amplio de otra manera los permitiría |

Las rutas de ambas configuraciones `sandbox.filesystem` y reglas de permiso se fusionan en la configuración final del sandbox.

El [directorio de ejemplos del repositorio claude-code](https://github.com/anthropics/claude-code/tree/main/examples/settings) incluye configuraciones de configuración de inicio para escenarios de implementación comunes, incluidos ejemplos específicos del sandbox. Úselos como puntos de partida y ajústelos para que se adapten a sus necesidades.

<h3 id="permission-modes">
  Modos de permiso
</h3>

`/sandbox` no es un [modo de permiso](/docs/es/permission-modes). Los modos de permiso deciden si se ejecuta una llamada de herramienta y si se le solicita primero, mientras que el sandbox restringe lo que un comando Bash puede acceder una vez que se ejecuta. Difieren en lo que controlan y qué reemplaza la solicitud por acción:

|                                                                          | Qué controla                                                | Qué reemplaza la solicitud                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :----------------------------------------------------------------------- | :---------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/sandbox`                                                               | Lo que un comando Bash puede acceder una vez que se ejecuta | El límite del sandbox en sí, en [modo auto-allow](#sandbox-modes)                                                                                                                                                                                                                                                                                                                                                                                                        |
| [Modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) | Si se ejecuta cada llamada de herramienta                   | Un clasificador que revisa acciones                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `--dangerously-skip-permissions`                                         | Si se ejecuta cada llamada de herramienta                   | Nada. Las verificaciones de [ruta protegida](/docs/es/permission-modes#protected-paths) también se omiten; solo las [reglas ask](/docs/es/permissions#manage-permissions) explícitas, las herramientas de conector [que su organización configuró en `ask`](/docs/es/mcp#organization-controls-on-connector-tools), las herramientas MCP marcadas como [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool) y eliminar `/` o su directorio de inicio aún solicitan |

El [modo auto-allow](#sandbox-modes) del sandbox es separado del [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode): auto-allow aprueba comandos Bash porque el límite del sandbox los contiene, mientras que el modo automático usa un clasificador para revisar acciones. Los dos funcionan independientemente y pueden combinarse. Para elegir un límite de aislamiento para ejecuciones desatendidas, consulte [Entornos sandbox](/docs/es/sandbox-environments#how-isolation-relates-to-permission-modes).

<h2 id="configure-the-sandbox-for-your-organization">
  Configurar el sandbox para su organización
</h2>

Los administradores pueden requerir sandboxing para cada usuario, evitar que los desarrolladores amplíen la política y enrutar el tráfico del sandbox a través de un proxy corporativo.

<h3 id="enforce-sandboxing-with-managed-settings">
  Aplicar sandboxing con configuración administrada
</h3>

Para requerir el sandbox para cada desarrollador, entregue las claves `sandbox` a través de [configuración administrada](/docs/es/settings#settings-files), ya sea como un archivo administrado por su MDM o a través de [configuración administrada por servidor](/docs/es/server-managed-settings) en Claude.ai.

La siguiente configuración de configuración administrada habilita el sandbox, se niega a iniciar Claude Code si el sandbox no puede inicializarse y evita que el modelo reintente comandos fuera del sandbox:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "failIfUnavailable": true,
    "allowUnsandboxedCommands": false
  }
}
```

Las dos claves más allá de `enabled` controlan qué sucede cuando el sandbox no puede ejecutar un comando:

* **`failIfUnavailable`**: una dependencia faltante como bubblewrap en Linux bloquea que Claude Code se inicie en lugar de mostrar una advertencia y volver a la ejecución sin aislar
* **`allowUnsandboxedCommands: false`**: la salida de emergencia `dangerouslyDisableSandbox` se ignora, por lo que los comandos que fallan bajo el sandbox no pueden reintentarse fuera de él

Dos adiciones vale la pena considerar junto con ellas. Agregue `excludedCommands` para cualquier herramienta aprobada por la organización que deba ejecutarse sin aislamiento. Agregue entradas [`sandbox.credentials`](#protect-credentials) para directorios de credenciales como `~/.aws` y `~/.ssh` y para variables de entorno secretas, ya que la política de lectura predeterminada aún las permite.

El sandbox no se ejecuta en Windows nativo, por lo que si su flota incluye hosts de Windows, limite esta configuración a macOS y Linux o haga que esos usuarios ejecuten Claude Code dentro de WSL2 o un contenedor.

<h3 id="keep-developers-from-widening-the-policy">
  Evitar que los desarrolladores amplíen la política
</h3>

Para claves booleanas como `enabled` y `failIfUnavailable`, Claude Code usa el valor administrado e ignora cualquier cosa que un desarrollador establezca localmente. Para claves de array como `excludedCommands` y `allowRead`, Claude Code fusiona entradas de cada ámbito, por lo que un desarrollador puede agregar entradas que amplíen la política.

Establezca `allowManagedReadPathsOnly` en `true` en la configuración administrada para que solo se honren las entradas `allowRead` de la configuración administrada. Las entradas `allowRead` de usuario, proyecto y local se ignoran. Esto evita que los desarrolladores amplíen el acceso de lectura más allá de las rutas aprobadas por la organización. Para bloquear dominios de red a los valores administrados de la misma manera, establezca [`allowManagedDomainsOnly`](/docs/es/settings#sandbox-settings).

`excludedCommands` no tiene un bloqueo equivalente solo administrado, por lo que un desarrollador siempre puede agregar entradas que ejecuten comandos adicionales fuera del sandbox. Mantenga la lista administrada estrecha.

<h3 id="custom-proxy-configuration">
  Configuración de proxy personalizado
</h3>

Para organizaciones que requieren seguridad de red avanzada, puede implementar un proxy personalizado para:

* Descifrar e inspeccionar tráfico HTTPS
* Aplicar reglas de filtrado personalizadas
* Registrar todas las solicitudes de red
* Integrar con infraestructura de seguridad existente

Para apuntar Claude Code a su proxy, establezca los puertos del proxy en [configuración de sandbox](/docs/es/settings#sandbox-settings):

```json theme={null}
{
  "sandbox": {
    "network": {
      "httpProxyPort": 8080,
      "socksProxyPort": 8081
    }
  }
}
```

<h2 id="troubleshooting">
  Solución de problemas
</h2>

Algunos comandos fallan dentro del sandbox aunque funcionen fuera de él. Las correcciones a continuación cubren los casos más comunes.

* **Los comandos fallan con un error de host no permitido**: muchas herramientas CLI necesitan alcanzar hosts específicos. Otorgar permiso cuando se solicita agrega el host a su lista de permitidos para que la herramienta se ejecute dentro del sandbox en el futuro.
* **`jest` se cuelga o falla**: `watchman` es incompatible con el sandbox. Ejecute `jest --no-watchman` en su lugar.
* **Las CLI basadas en Go fallan en la verificación de TLS en macOS**: herramientas como `gh`, `gcloud` y `terraform` pueden fallar en la verificación de TLS bajo Seatbelt. Liste estas herramientas en `excludedCommands` para ejecutarlas fuera del sandbox. Si está usando `httpProxyPort` con un proxy MITM y CA personalizado, establezca [`enableWeakerNetworkIsolation`](/docs/es/settings#sandbox-settings) en `true` en su lugar.
* **Los comandos `open`, `osascript` o los flujos de autenticación basados en navegador fallan con el error `-600` en macOS**: el sandbox bloquea Apple Events de forma predeterminada. Establezca [`allowAppleEvents`](/docs/es/settings#sandbox-settings) en `true` en su configuración de usuario, administrada o CLI para permitirlos. La configuración del proyecto se ignora para esta clave. Habilitarlo elimina el aislamiento de ejecución de código, ya que los comandos aislados pueden entonces lanzar otras aplicaciones sin aislar sin solicitud del usuario y enviar comandos AppleScript a aplicaciones en ejecución, sujeto a la solicitud de consentimiento de automatización de macOS (TCC). Alternativamente, agregue el comando a `excludedCommands` para ejecutarlo fuera del sandbox.
* **Los comandos `docker` fallan**: `docker` es incompatible con el sandbox. Agregue `docker *` a `excludedCommands` para ejecutarlo fuera del sandbox.
* **Bubblewrap falla al iniciarse dentro de un contenedor**: en un contenedor sin privilegios, bubblewrap no puede montar un sistema de archivos `/proc` nuevo. Establezca [`enableWeakerNestedSandbox`](/docs/es/settings#sandbox-settings) en `true` para que el sandbox interno monte el `/proc` existente del contenedor en su lugar. Solo use esta configuración cuando el contenedor externo ya proporcione el límite de aislamiento que necesita, ya que expone información de proceso a comandos aislados que un montaje `/proc` nuevo ocultaría.
* **Filtro seccomp en Linux**: el filtro seccomp es necesario para bloquear sockets de dominio Unix. La pestaña Dependencies en `/sandbox` muestra si está disponible. Si falta, ejecute `npm install -g @anthropic-ai/sandbox-runtime` para instalar el asistente.
* **`--dangerously-skip-permissions` falla como root**: esta bandera se bloquea cuando se ejecuta como root o a través de sudo en Linux y macOS, porque el acceso root combinado sin solicitudes de permiso puede modificar cualquier archivo o servicio en el sistema. La verificación se omite automáticamente dentro de un sandbox reconocido. Para ejecutar de manera autónoma en un contenedor, use la configuración [contenedor de desarrollo](/docs/es/devcontainer), que ejecuta Claude Code como un usuario no root.

<h2 id="limitations">
  Limitaciones
</h2>

El sandboxing reduce el riesgo pero no es un límite de aislamiento completo. Revise las limitaciones a continuación antes de confiar en él como un control de seguridad duro.

<h3 id="security-limitations">
  Limitaciones de seguridad
</h3>

* **Filtrado de red**: el sandbox restringe los dominios a los que se permite que se conecten los procesos. De forma predeterminada, el proxy integrado no termina ni inspecciona TLS en el tráfico saliente, por lo que el contenido de las conexiones cifradas no se examina. La configuración experimental [`network.tlsTerminate`](/docs/es/settings#sandbox-settings) termina TLS en el proxy para [sustitución de credenciales `mask`](#protect-credentials) pero no añade filtrado de contenido. Usted es responsable de asegurarse de que solo se permitan dominios confiables en su política.

<Warning>
  Permitir dominios amplios como `github.com` puede crear caminos para exfiltración de datos. Porque el proxy toma su decisión de permitir del nombre de host suministrado por el cliente sin inspeccionar TLS, el código que se ejecuta dentro del sandbox puede potencialmente usar [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) u técnicas similares para alcanzar hosts fuera de la lista de permitidos. Si su modelo de amenaza requiere garantías más fuertes, configure un [proxy personalizado](#custom-proxy-configuration) que termine TLS e inspeccione tráfico, e instale su certificado CA dentro del sandbox. El aislamiento de red consciente de TLS más fuerte es un área activa de desarrollo.
</Warning>

* **Escalada de privilegios a través de sockets Unix**: la configuración `allowUnixSockets` puede otorgar inadvertidamente acceso a servicios del sistema poderosos que podrían llevar a omisiones del sandbox. Por ejemplo, permitir acceso a `/var/run/docker.sock` efectivamente otorga acceso al sistema host a través del socket de Docker. Considere cuidadosamente cualquier socket Unix que permita a través del sandbox.
* **Escalada de permisos del sistema de archivos**: los permisos de escritura del sistema de archivos demasiado amplios pueden permitir ataques de escalada de privilegios. Permitir escrituras en directorios que contienen ejecutables en `$PATH`, directorios de configuración del sistema o archivos de configuración de shell del usuario como `.bashrc` o `.zshrc` puede llevar a ejecución de código en diferentes contextos de seguridad cuando otros usuarios o procesos del sistema acceden a estos archivos.
* **Fortaleza del sandbox de Linux**: la implementación de Linux proporciona un fuerte aislamiento del sistema de archivos y la red pero incluye un modo `enableWeakerNestedSandbox` que le permite funcionar dentro de entornos Docker sin espacios de nombres privilegiados, o en hosts Linux donde los espacios de nombres de usuario sin privilegios están deshabilitados por sysctl. Esta opción debilita considerablemente la seguridad y solo debe usarse cuando se aplica aislamiento adicional de otra manera.
* **Apple Events en macOS**: el sandbox de macOS bloquea Apple Events de forma predeterminada. La configuración `allowAppleEvents` levanta esta restricción para que herramientas como `open` y `osascript` funcionen, pero elimina el aislamiento de ejecución de código: los comandos aislados pueden lanzar otras aplicaciones sin aislar sin solicitud del usuario, y pueden enviar comandos AppleScript a aplicaciones en ejecución, sujeto al aviso de consentimiento de automatización por aplicación de macOS (TCC). Solo se honra desde configuración de usuario, administrada o CLI. La configuración del proyecto no puede habilitarla.
* **Archivos de configuración protegidos**: el sandbox automáticamente deniega acceso de escritura a los archivos `settings.json` de Claude Code en cada ámbito y al directorio de configuración administrada, por lo que un comando aislado no puede modificar su propia política.

<h3 id="platform-and-tool-compatibility">
  Compatibilidad de plataforma y herramienta
</h3>

* **Soporte de plataforma**: admite macOS, Linux y WSL2. WSL1 y Windows nativo no son compatibles.
* **Sobrecarga de rendimiento**: mínima, pero algunas operaciones del sistema de archivos pueden ser ligeramente más lentas.
* **Compatibilidad de herramienta**: algunas herramientas que requieren patrones de acceso específicos del sistema pueden necesitar ajustes de configuración, o pueden necesitar ejecutarse fuera del sandbox.

<h3 id="scope">
  Alcance
</h3>

El sandbox aísla subprocesos Bash. Otras herramientas operan bajo límites diferentes:

* **Herramientas de archivo integradas**: Read, Edit y Write usan el sistema de permisos directamente en lugar de ejecutarse a través del sandbox. Consulte [permisos](/docs/es/permissions).
* **Uso de computadora**: cuando Claude abre aplicaciones y controla su pantalla, se ejecuta en su escritorio real en lugar de en un entorno aislado. Las solicitudes de permiso por aplicación controlan cada aplicación. Consulte [uso de computadora en CLI](/docs/es/computer-use) o [uso de computadora en Desktop](/docs/es/desktop#let-claude-use-your-computer).
* **Variables de entorno**: los comandos Bash aislados heredan el entorno del proceso padre de forma predeterminada, incluidas las credenciales establecidas allí. Use [`sandbox.credentials`](#protect-credentials) para desactivar o enmascarar variables específicas para comandos aislados, o establezca [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/es/env-vars) para eliminar credenciales de Anthropic y proveedores de nube de todos los subprocesos.
* **Subagentes**: los [subagentes](/docs/es/sub-agents) se ejecutan en el mismo proceso que la sesión padre y usan la misma configuración de sandbox. Los comandos Bash dentro de un subagente están aislados cuando el sandboxing está habilitado en la sesión padre.

<Warning>
  El sandboxing efectivo requiere tanto aislamiento del sistema de archivos como de la red. Sin aislamiento de red, un agente comprometido podría exfiltrar archivos sensibles como claves SSH. Sin aislamiento del sistema de archivos, un agente comprometido podría instalar una puerta trasera en recursos del sistema para obtener acceso a la red. Cuando amplíe los valores predeterminados, verifique que una ruta `allowWrite`, una entrada `allowedDomains` amplia o una excepción `excludedCommands` no deshaga una restricción en el otro lado.
</Warning>

<h2 id="see-also">
  Ver también
</h2>

* [Entornos sandbox](/docs/es/sandbox-environments): compare el sandbox integrado con contenedores de desarrollo, contenedores y máquinas virtuales
* [Seguridad](/docs/es/security): características de seguridad integral y mejores prácticas
* [Permisos](/docs/es/permissions): configuración de permisos y control de acceso
* [Configuración](/docs/es/settings): referencia de configuración completa
* [Referencia de CLI](/docs/es/cli-reference): opciones de línea de comandos
