> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurar la herramienta Bash aislada

> Aprenda cómo la herramienta Bash aislada de Claude Code proporciona aislamiento del sistema de archivos y la red para una ejecución de agentes más segura y autónoma.

El sandbox de Bash permite que Claude ejecute la mayoría de comandos de shell sin detenerse para pedir permiso. En lugar de aprobar cada comando, usted define qué archivos y dominios de red pueden tocar los comandos, y el sistema operativo aplica ese límite para cada comando Bash, PowerShell o Monitor y sus procesos secundarios.

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

    Esto abre el panel de sandbox con tres pestañas, más una pestaña Dependencies en Linux cuando falta el filtro seccomp opcional:

    * **Mode**: elija cómo se aprueban los comandos aislados, cubierto en el siguiente paso
    * **Overrides**: elija si los comandos que fallan bajo el sandbox pueden volver a ejecutarse sin aislar. Esta es la configuración [`allowUnsandboxedCommands`](/docs/es/settings-reference#sandbox-allowunsandboxedcommands)
    * **Config**: vea la configuración de sandbox resuelta

    Si el panel muestra solo una pestaña Dependencies, falta un paquete requerido. Instálelo como se describe en [Configurar Linux y WSL2](#set-up-linux-and-wsl2), reinicie Claude Code y ejecute `/sandbox` nuevamente.
  </Step>

  <Step title="Elegir un modo">
    En la pestaña Mode, seleccione auto-allow o permisos regulares. Auto-allow ejecuta comandos aislados sin solicitar, y permisos regulares mantiene las solicitudes de permiso regulares incluso cuando los comandos están aislados. Consulte [Modos de sandbox](#sandbox-modes) para ver qué comandos aún solicitan en modo auto-allow.
  </Step>

  <Step title="Ejecutar un comando Bash">
    Pida a Claude que ejecute un comando, como una compilación o un conjunto de pruebas. De forma predeterminada, los comandos dentro del sandbox pueden escribir en el directorio de trabajo, el directorio temporal de la sesión y cualquier [directorio que haya agregado](/docs/es/permissions#additional-directories-grant-file-access-not-configuration) con `--add-dir`, `/add-dir` o `permissions.additionalDirectories`.

    La primera vez que un comando necesita un nuevo dominio de red, Claude Code solicita aprobación; en [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode), Claude en su lugar nombra los hosts que un comando necesita [en el comando mismo](#per-command-allowed-domains-in-auto-mode) para que el clasificador lo revise con él.

    Los comandos que no pueden ejecutarse aislados vuelven al flujo de permiso regular. Claude Code titula su solicitud de permiso "Bash command (unsandboxed)" en lugar de "Bash command", para que pueda saber qué comandos se ejecutaron fuera del sandbox. Para ampliar o reducir lo que permite el sandbox, consulte [Configurar sandboxing](#configure-sandboxing).

    Si los comandos aislados fallan con `Operation not permitted` dentro de un contenedor, consulte la entrada Bubblewrap en [Solución de problemas](#troubleshooting).
  </Step>
</Steps>

Cuando selecciona un modo en el panel, Claude Code lo guarda en la configuración local de su proyecto en `.claude/settings.local.json`, que se aplica al proyecto actual. Claude Code agrega ese archivo a su gitignore global cuando guarda una configuración allí. Para habilitar el sandbox en todos sus proyectos, establezca [`sandbox.enabled`](/docs/es/settings-reference#sandbox-enabled) en `true` en su configuración de usuario en `~/.claude/settings.json`. Para aplicar sandboxing para cada desarrollador en una organización, use [configuración administrada](#enforce-sandboxing-with-managed-settings).

Para cambiar el sandbox para una sesión sin escribir en un archivo de configuración, inicie Claude Code con [`--settings`](/docs/es/settings#change-a-setting-for-one-session). Por ejemplo, este comando inicia una sesión aislada en la que Claude no puede reintentar un comando bloqueado fuera del sandbox:

```bash theme={null}
claude --settings '{"sandbox": {"enabled": true, "allowUnsandboxedCommands": false}}'
```

<Warning>
  De forma predeterminada, si el sandbox no puede iniciarse porque faltan dependencias o la plataforma no es compatible, Claude Code muestra una advertencia y ejecuta comandos sin sandboxing. Para hacer que esto sea un error grave en su lugar, establezca [`sandbox.failIfUnavailable`](/docs/es/settings-reference#sandbox-failifunavailable) en `true`. Esto está destinado a implementaciones administradas que requieren sandboxing como una puerta de seguridad.
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

Cuando falta una dependencia, la pestaña Dependencies en `/sandbox` enumera cuál de `ripgrep`, `bubblewrap`, `socat` y el filtro seccomp le falta a su plataforma. Si no ve la pestaña después de instalar y reiniciar Claude Code, todas las dependencias están presentes.

Ripgrep se incluye con el binario nativo de Claude Code. El filtro seccomp es opcional y agrega bloqueo de socket de dominio Unix. Instálelo con `npm install -g @anthropic-ai/sandbox-runtime` si falta.

Cuando falta una dependencia requerida, la pestaña Dependencies es la única pestaña mostrada hasta que la instale. Cuando solo falta el filtro seccomp opcional, la pestaña Dependencies aparece junto con las otras pestañas. La verificación de dependencia se ejecuta al inicio, así que reinicie Claude Code después de instalar paquetes para que `/sandbox` los detecte.

<AccordionGroup>
  <Accordion title="Ubuntu 24.04 y posterior: permitir que bubblewrap cree espacios de nombres de usuario">
    En Ubuntu 24.04 y posterior, la política de AppArmor predeterminada impide que bubblewrap cree los espacios de nombres de usuario que necesita para el aislamiento.

    Para verificar si su entorno aplica esta restricción, incluso dentro de WSL2, ejecute `sysctl kernel.apparmor_restrict_unprivileged_userns`. Si el comando devuelve `0`, omita este paso. Si imprime un error `No such file or directory`, la clave no existe y puede omitir este paso. Si devuelve `1`, agregue un perfil de AppArmor que otorgue a `bwrap` esta capacidad:

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

    En WSL2, WSL entrega un lanzamiento de un binario de Windows como `cmd.exe`, `powershell.exe` o cualquier cosa bajo `/mnt/c/` al host de Windows a través de un socket Unix, por lo que si un comando aislado puede lanzar uno sigue la configuración [Unix-socket](/docs/es/settings-reference#sandbox-network-allowunixsockets) del sandbox: el filtro seccomp opcional tiene que estar instalado para bloquear el socket en primer lugar. Para permitir estos lanzamientos, establezca `allowAllUnixSockets`; para mantenerlos fuera del sandbox completamente, agregue el comando a [`excludedCommands`](/docs/es/settings-reference#sandbox-excludedcommands).
  </Accordion>
</AccordionGroup>

<h3 id="sandbox-modes">
  Modos de sandbox
</h3>

Claude Code ofrece dos modos de sandbox. En ambos, el sandbox aplica las mismas restricciones de sistema de archivos y red; la diferencia es solo si los comandos aislados se aprueban automáticamente o requieren permiso explícito.

<h4 id="auto-allow-mode">
  Modo auto-allow
</h4>

Cuando un comando puede ser aislado, Claude Code lo ejecuta dentro del sandbox y lo aprueba automáticamente, sin pedirle permiso. Los comandos que no pueden ser aislados, como aquellos que necesitan acceso a la red a hosts no permitidos, vuelven al flujo de permiso regular, donde Claude Code verifica sus [reglas de permiso](/docs/es/permissions) y bloquea cualquier comando que esas reglas no permitan, con una solicitud en modo Manual.

Incluso en modo auto-allow, lo siguiente sigue siendo válido:

* Las [reglas de denegación](/docs/es/permissions) explícitas siempre se respetan
* Los comandos `rm` o `rmdir` que apunten a una [ruta crítica](/docs/es/permission-modes#critical-paths) aún pasan por el flujo de permiso regular
* Las [reglas de solicitud](/docs/es/permissions) con alcance de contenido como `Bash(git push *)` aún fuerzan una solicitud incluso para comandos aislados
* Una regla de solicitud `Bash` simple, o la forma equivalente `Bash(*)`, se omite para comandos que se ejecutan aislados; aún se aplica a comandos que vuelven al flujo de permiso regular. En [modo de plan](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode), la regla no se omite: solicita comandos aislados también, incluyendo los de solo lectura. Antes de v2.1.212, la omisión se aplicaba en modo de plan también

<Info>
  El modo auto-allow funciona independientemente de su configuración de modo de permiso, con tres excepciones: [modo de plan](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode), un comando en modo automático que lleva [dominios permitidos por comando](#per-command-allowed-domains-in-auto-mode), y [revisión del clasificador del lado del servidor](/docs/es/permission-modes#how-the-classifier-evaluates-actions) de comandos aislados en modo automático. Incluso si no está en modo "aceptar ediciones", los comandos Bash aislados se ejecutan automáticamente cuando auto-allow está habilitado. Esto significa que los comandos Bash que modifican archivos dentro de los límites del sandbox se ejecutan sin solicitar, incluso en modo Manual, donde las herramientas de edición de archivos solicitarían.

  En modo de plan, auto-allow no amplía las aprobaciones; consulte [modo de plan](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode) para ver cómo Claude Code bloquea comandos mientras planifica. Antes de v2.1.212, auto-allow ejecutaba comandos aislados sin solicitud en modo de plan también.
</Info>

<h4 id="regular-permissions-mode">
  Modo de permisos regulares
</h4>

Todos los comandos Bash pasan por el flujo de permiso regular, incluso cuando están aislados. Esto proporciona más control pero requiere más aprobaciones.

<h4 id="the-unsandboxed-retry-escape-hatch">
  La salida de emergencia de reintento sin sandbox
</h4>

Algunos comandos no pueden ejecutarse dentro del sandbox en absoluto, como herramientas que son incompatibles con él o que necesitan un host que no ha permitido. Claude Code reporta violaciones de sandbox en el resultado del comando bloqueado, nombrando la ruta u host que el sandbox denegó, para que Claude vea qué bloqueó el sandbox. En lugar de fallar la tarea o requerirle que apague el sandboxing, Claude Code incluye una salida de emergencia: Claude analiza la violación y puede reintentar el comando con el parámetro `dangerouslyDisableSandbox`.

El comando reintentado se ejecuta fuera del sandbox, por lo que pasa por el flujo de permiso regular. En modo Manual obtiene una solicitud de confirmación. En [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode), el clasificador evalúa el comando subyacente. Mientras [`permissions.blockReadsOutsideWorkingDirectories`](/docs/es/settings-reference#permissions-blockreadsoutsideworkingdirectories) esté activado, un reintento que necesita aprobación para ejecutarse fuera del sandbox le solicita en su lugar. Para que se le solicite en cada reintento sin sandbox incluso en modo automático, agregue una [regla de solicitud](/docs/es/permissions#match-by-input-parameter) para `Bash(dangerouslyDisableSandbox:true)`.

Puede deshabilitar esta salida de emergencia estableciendo `"allowUnsandboxedCommands": false` en su [configuración de sandbox](/docs/es/settings-reference#sandbox-settings). Con la salida de emergencia deshabilitada, Claude Code ignora el parámetro `dangerouslyDisableSandbox`, y cada comando que Claude ejecuta debe ejecutarse aislado a menos que lo haya listado en `excludedCommands`. La pestaña **Overrides** de `/sandbox` muestra esta configuración como **Strict sandbox mode**.

El modo de sandbox estricto se aplica a los comandos que Claude ejecuta. Los comandos que usted escribe en el símbolo del sistema de modo shell [`!`](/docs/es/interactive-mode#shell-mode-with-prefix) se ejecutan fuera del sandbox a menos que la sesión sea una de estas:

* **Una [sesión de fondo](/docs/es/agent-view)**: el modo de sandbox estricto cubre comandos de modo shell también
* **Una sesión de Linux con [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/es/env-vars#variables) establecido**: cada comando se ejecuta aislado, comandos de modo shell incluidos

Antes de v2.1.260, el modo de sandbox estricto aislaba comandos de modo shell en cada sesión.

<h4 id="temporary-directories">
  Directorios temporales
</h4>

El directorio temporal de la sesión es escribible dentro del sandbox de forma predeterminada, junto con el directorio de trabajo. A menos que [deshabilite el aislamiento del sistema de archivos](#disable-filesystem-isolation), Claude Code establece `$TMPDIR` en este directorio para comandos aislados, por lo que las herramientas que escriben archivos temporales funcionan sin configuración adicional.

Los comandos sin aislar heredan su `$TMPDIR` de shell cuando está establecido, por lo que mientras el aislamiento del sistema de archivos esté activado, los comandos aislados y sin aislar resuelven `$TMPDIR` a directorios diferentes. Si su shell deja `$TMPDIR` sin establecer o vacío, un comando sin aislar que hace referencia a `$TMPDIR` recibe su anulación [`CLAUDE_CODE_TMPDIR`](/docs/es/env-vars), o el directorio temporal del sistema operativo cuando no ha establecido uno o la anulación es una ruta larga, por lo que la variable no se expande a una cadena vacía. Para pasar archivos temporales entre los dos, escríbalos en el directorio de trabajo en su lugar.

<h2 id="configure-sandboxing">
  Configurar sandboxing
</h2>

Personalice el comportamiento del sandbox a través de su archivo `settings.json`. Consulte [Configuración](/docs/es/settings-reference#sandbox-settings) para obtener la referencia de configuración completa.

De forma predeterminada, los comandos aislados pueden escribir en el directorio de trabajo actual, el directorio temporal de la sesión y cualquier [directorio que haya agregado](/docs/es/permissions#additional-directories-grant-file-access-not-configuration) con `--add-dir`, `/add-dir` o `permissions.additionalDirectories`. Si comandos de subproceso como `kubectl`, `terraform` o `npm` necesitan escribir fuera de esos directorios, use `sandbox.filesystem.allowWrite` para otorgar acceso a rutas específicas:

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

Cuando define el mismo array del sistema de archivos en múltiples [ámbitos de configuración](/docs/es/settings#settings-precedence), Claude Code los fusiona, combinando rutas de cada ámbito en lugar de reemplazar el array de un ámbito con otro.

Si excluye una fuente con [`--setting-sources`](/docs/es/cli-reference) en la CLI o [`settingSources`](/docs/es/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) en el Agent SDK, Claude Code ignora sus entradas `sandbox.filesystem`, sus reglas de permiso `Edit` y sus reglas de denegación `Read` al construir la configuración del sandbox. Requiere Claude Code v2.1.246 o posterior.

Cuando edita estas listas del sistema de archivos durante una sesión, Claude Code [aplica el cambio a la sesión en ejecución](/docs/es/settings#when-edits-take-effect), por lo que el siguiente comando aislado se ejecuta bajo las nuevas rutas.

Los prefijos de ruta controlan cómo se resuelven las rutas:

| Prefijo            | Significado                                                                                                   | Ejemplo                                                                     |
| :----------------- | :------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------- |
| `/`                | Ruta absoluta desde la raíz del sistema de archivos                                                           | `/tmp/build` se mantiene como `/tmp/build`                                  |
| `~/`               | Relativo al directorio de inicio                                                                              | `~/.kube` se convierte en `$HOME/.kube`                                     |
| `./` o sin prefijo | Relativo a la raíz del proyecto para configuración de proyecto, o a `~/.claude` para configuración de usuario | `./output` en `.claude/settings.json` se resuelve a `<project-root>/output` |

Esta sintaxis difiere de las [reglas de permiso Read y Edit](/docs/es/permissions#read-and-edit), que usan `//path` para absoluto y `/path` para relativo al proyecto. Las rutas del sistema de archivos del sandbox usan convenciones estándar: `/tmp/build` es absoluto. Para saber cómo Claude Code trata una barra diagonal final o un comodín en estas rutas, consulte [Prefijos de ruta del sandbox](/docs/es/settings-reference#sandbox-path-prefixes).

También puede denegar acceso de escritura o lectura usando `sandbox.filesystem.denyWrite` y `sandbox.filesystem.denyRead`, y permitir nuevamente rutas específicas dentro de una región denegada usando `sandbox.filesystem.allowRead`. Cuando las reglas de lectura se superponen, la ruta más específica gana:

| Reglas de ejemplo                                      | Resultado                                                                                                                                                                                                                                       |
| :----------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"denyRead": ["~/"]` con `"allowRead": ["~/projects"]` | `~/projects` es legible y el resto del directorio de inicio permanece bloqueado. El permiso más estrecho reabre esa parte de la región denegada                                                                                                 |
| `"allowRead": ["~/"]` con `"denyRead": ["~/.env"]`     | `~/.env` permanece bloqueado y el resto del directorio de inicio es legible. La denegación se mantiene dentro de un permiso más amplio, por lo que un permiso amplio no puede reexponer silenciosamente un secreto                              |
| `"allowRead": ["~/"]` con `"denyRead": ["~/**/.env"]`  | Cada `.env` bajo el directorio de inicio permanece bloqueado y el resto es legible. Un [comodín de denegación](/docs/es/settings-reference#sandbox-path-prefixes) se mantiene dentro de un permiso más amplio de la misma manera que una ruta exacta |

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

Si colocara la misma configuración en `~/.claude/settings.json`, `.` se resolvería a `~/.claude` en su lugar, y los archivos del proyecto permanecerían bloqueados por la regla `denyRead`.

Para denegar a los comandos aislados acceso de lectura a directorios de inicio y volúmenes montados mientras se mantienen los directorios de trabajo legibles, establezca [`permissions.blockReadsOutsideWorkingDirectories`](/docs/es/settings-reference#permissions-blockreadsoutsideworkingdirectories) en su lugar de escribir reglas de ruta.

<h3 id="disable-filesystem-isolation">
  Deshabilitar aislamiento del sistema de archivos
</h3>

Establezca `sandbox.filesystem.disabled` en `true` para omitir el aislamiento del sistema de archivos mientras se mantiene el aislamiento de red. El ejemplo a continuación desactiva el aislamiento del sistema de archivos mientras se mantiene una lista de permitidos de dominios de red:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "disabled": true
    },
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org"]
    }
  }
}
```

El sandbox tiene dos capas independientes: el [aislamiento del sistema de archivos](#filesystem-isolation) controla qué rutas pueden leer y escribir los comandos aislados, y el [aislamiento de red](#network-isolation) controla qué dominios pueden alcanzar. Con la capa del sistema de archivos desactivada, los comandos aislados obtienen acceso de lectura y escritura sin restricciones al sistema de archivos del host, mientras que su salida de red permanece confinada a sus dominios permitidos. Desactive la capa cuando aisle para controlar dónde se conectan los comandos en lugar de lo que escriben.

La configuración está desactivada de forma predeterminada y se aplica en las plataformas donde se ejecuta el sandbox: macOS, Linux y WSL2. Requiere Claude Code v2.1.216 o posterior.

<Warning>
  Con el aislamiento del sistema de archivos desactivado y los comandos permitidos automáticamente, un comando aislado puede escribir archivos que comandos posteriores ejecuten o lean, como archivos de inicio de shell, ejecutables en `$PATH` o `~/.claude/settings.json`, y usarlos para ampliar su propio acceso en la siguiente ejecución. Establezca `filesystem.disabled` en `true` solo para cargas de trabajo en las que confía que no escalen su propio acceso. Bloquear dominios de red con [`allowManagedDomainsOnly`](#keep-developers-from-widening-the-policy) reduce el riesgo pero no lo elimina, ya que ese bloqueo se aplica solo a los comandos que se ejecutan dentro del sandbox.
</Warning>

<h4 id="which-settings-can-disable-it">
  Qué configuraciones pueden deshabilitarlo
</h4>

Debido a que desactivar el aislamiento del sistema de archivos amplía lo que pueden hacer los comandos aislados, Claude Code honra `filesystem.disabled` solo desde estas fuentes de configuración:

* La configuración de usuario, la configuración administrada y la bandera CLI `--settings` pueden establecerla. La configuración del proyecto en `.claude/settings.json` y `.claude/settings.local.json` no puede, por lo que un proyecto extraído no puede desactivar el aislamiento del sistema de archivos.
* Cuando la configuración administrada configura `sandbox.filesystem` en absoluto, o enumera cualquier entrada `sandbox.credentials.files` con `"mode": "deny"`, solo la configuración administrada puede establecer la clave. Esto mantiene las restricciones del sistema de archivos implementadas por el administrador en vigor; para relajar tal implementación, establezca `"disabled": true` en la configuración administrada.
* Cuando [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/es/env-vars) está establecido, Claude Code ignora `filesystem.disabled` de cada fuente, incluida la configuración administrada, y mantiene el aislamiento del sistema de archivos activado.

Si una entrada de credenciales administrada `credentials.files` fija `filesystem.disabled`, bloqueando la clave a la configuración administrada para que los desarrolladores no puedan desactivar el aislamiento del sistema de archivos, depende del `mode` de la entrada y de lo que sucede con la entrada cuando se inicia el sandbox:

| Entrada administrada                                                                                            | Fija `filesystem.disabled`            | Qué protege el archivo cuando el aislamiento está desactivado                                                                                      |
| --------------------------------------------------------------------------------------------------------------- | ------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"mode": "deny"`                                                                                                | Sí                                    | Nada: el bloqueo de lectura es parte de la capa del sistema de archivos                                                                            |
| `"mode": "mask"`, aplicado como máscara                                                                         | No                                    | El enmascaramiento en sí: la [copia centinela y proxy](#mask-credential-files) en Linux y WSL2, las propias reglas de lectura del sandbox en macOS |
| `"mode": "mask"`, [retrocedido a `deny`](#mask-credential-files) en la configuración                            | No                                    | Nada, igual que `deny`. Enumere una ruta que no se pueda enmascarar, como un directorio, como una entrada explícita `deny`, que fija la clave      |
| `"mode": "mask"`, [degradado a `deny` por validación](/docs/es/managed-settings#invalid-entries-in-managed-settings) | Sí, como una entrada explícita `deny` | Nada, igual que `deny`                                                                                                                             |

Un retroceso ocurre cuando se inicia el sandbox, después de que Claude Code ya haya leído la configuración en la que se ejecuta la verificación de fijación, por lo que una entrada retrocedida nunca fija. La validación reescribe una entrada inválida a `deny` mientras se carga la configuración, por lo que una entrada degradada fija como una que escribió como `deny`.

<h4 id="what-changes-when-filesystem-isolation-is-off">
  Qué cambia cuando el aislamiento del sistema de archivos está desactivado
</h4>

Establecer `filesystem.disabled` levanta las protecciones que la capa del sistema de archivos en sí misma aplica. Las protecciones que otras capas aplican siguen aplicándose:

| Protección                                                                                    | Con aislamiento del sistema de archivos desactivado                                                                                                                                          |
| --------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `filesystem.denyRead` y [`credentials.files`](#protect-credentials) bloques de lectura `deny` | No se aplica. La capa del sistema de archivos aplica ambos                                                                                                                                   |
| `credentials.envVars` entradas `deny` y `mask`                                                | Se aplica. El raspado de variables de entorno es independiente de la capa del sistema de archivos                                                                                            |
| Entradas [`credentials.files` `mask`](#mask-credential-files) aplicadas como máscaras         | Se aplica: el enmascaramiento es independiente de la capa del sistema de archivos. Una entrada que [retrocedió a `deny`](#mask-credential-files) no se aplica, como cualquier entrada `deny` |

Dos otras cosas cambian:

* Los comandos aislados heredan el `$TMPDIR` de su shell en lugar del directorio temporal de la sesión, porque cada directorio temporal es escribible y Claude Code ya no redirige los comandos al de la sesión.

  En Linux la variable a menudo no está establecida en el shell padre. La orientación de herramienta Bash le dice a Claude que cree directorios de trabajo con `mktemp -d` en lugar de confiar en `$TMPDIR`.
* [`autoAllowBashIfSandboxed`](/docs/es/settings-reference#sandbox-autoallowbashifsandboxed) aún tiene como valor predeterminado `true`, por lo que los comandos aislados siguen ejecutándose sin indicadores. Establézcalo en `false` para solicitar comandos aislados.

<h3 id="protect-credentials">
  Proteger credenciales
</h3>

La configuración `sandbox.credentials` declara archivos de credenciales y variables de entorno a proteger de los comandos aislados. Cada entrada nombra una ruta de archivo o una variable de entorno y un `mode`. El bloque dedicado `credentials` mantiene las reglas de credenciales agrupadas juntas y separadas de las reglas generales del sistema de archivos.

Para entradas con `"mode": "deny"`, las rutas de archivo se deniegan para lecturas dentro del sandbox, la misma restricción que aplica `filesystem.denyRead`, y las variables de entorno se desactivan antes de que se ejecute cada comando aislado. La protección de archivo es parte de la capa del sistema de archivos, por lo que no se aplica si [deshabilita el aislamiento del sistema de archivos](#disable-filesystem-isolation); la protección de variable de entorno aún lo hace.

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

Las entradas de variables de entorno y archivo también aceptan `"mode": "mask"`, descrito bajo [Enmascarar credenciales](#mask-credentials).

Las rutas de archivo siguen las mismas [reglas de prefijo](/docs/es/settings-reference#sandbox-path-prefixes) que las configuraciones `sandbox.filesystem.*`.

Claude Code fusiona las entradas `deny` de cada [ámbito de configuración](/docs/es/settings#settings-precedence) que carga la sesión. Una entrada `deny` solo estrecha el acceso, por lo que cualquier ámbito puede agregar una, pero ningún ámbito puede eliminar una que otro ámbito haya agregado.

Cuando [excluye una fuente de configuración](#configure-sandboxing):

* **Configuración de proyecto o local**: Claude Code no aplica ninguna de sus entradas `credentials`. Requiere Claude Code v2.1.246 o posterior.
* **Configuración de usuario**: Claude Code aún aplica las entradas `deny` en `~/.claude/settings.json` y mantiene sus entradas `mask` de [archivo](#mask-credential-files) como restricciones, pero descarta sus entradas `mask` de [variable de entorno](#mask-environment-variables).

No hay una lista de denegación de credenciales integrada, por lo que solo los archivos y variables que enumere están restringidos.

`sandbox.credentials` afecta solo a los comandos Bash aislados. Para eliminar las credenciales de todos los subprocesos independientemente del sandboxing, establezca [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/es/env-vars).

<h3 id="mask-credentials">
  Enmascarar credenciales
</h3>

El enmascaramiento va más allá de una entrada `deny` bajo [Proteger credenciales](#protect-credentials). En lugar de bloquear una credencial, Claude Code muestra a los comandos aislados un marcador de posición, el centinela, y el [proxy del sandbox](#network-isolation) intercambia el valor real en solicitudes salientes a hosts que permite. Para archivos, la sustitución es comportamiento de Linux y WSL2; [macOS bloquea el archivo en su lugar](#mask-credential-files).

<h4 id="mask-environment-variables">
  Enmascarar variables de entorno
</h4>

`"mode": "mask"` protege una credencial mientras mantiene funcionando las herramientas que se autentican con ella. `deny` elimina la variable por completo, lo que también rompe las herramientas que la necesitan, como `gh` o `npm`. Requiere Claude Code v2.1.199 o posterior.

Con `mask`, el comando aislado ve un valor centinela por sesión en lugar del real. Cada entrada `mask` puede enumerar `injectHosts`, los hosts a los que se permite que el valor real llegue. Cuando una solicitud sale del sandbox para uno de ellos, el [proxy del sandbox](#network-isolation) reemplaza el centinela con el valor real. El comando y cualquier cosa que registre nunca contienen la credencial real, pero sus solicitudes aún se autentican.

El proxy sustituye la credencial dentro del contenido de la solicitud, por lo que tiene que verla. Establezca [`network.tlsTerminate`](/docs/es/settings-reference#sandbox-network-tlsterminate) para que el proxy termine TLS por sí mismo.

Sin él, el enmascaramiento falla sin exponer nada: el comando aún ve solo el centinela, pero el centinela llega al servidor sin cambios y la autenticación falla. Claude Code reporta esta configuración incorrecta al inicio.

La sustitución cubre encabezados y cuerpos de solicitud. Las solicitudes que se autentican con una firma derivada de la credencial, en lugar de la credencial en sí, necesitan re-firmarse en el proxy; [Re-firmar solicitudes de AWS](#re-sign-aws-requests) cubre cómo funciona eso para AWS.

El proxy inyecta solo en conexiones que la [lista de permitidos de dominio](#network-isolation) admite, por lo que cada destino `injectHosts` también debe ser alcanzable a través de `network.allowedDomains`.

El ejemplo a continuación enmascara dos tokens. `GH_TOKEN` se sustituye solo en solicitudes a `api.github.com`, mientras que `NPM_TOKEN` no tiene `injectHosts` y se sustituye en solicitudes a cada host en `network.allowedDomains`.

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

<span id="ipv6-destinations-in-injecthosts" />Deletree un destino IPv6 de manera diferente en las dos listas, porque cada lista tiene su propio comparador:

* **`network.allowedDomains`**: la [forma entre corchetes que usan las listas de dominios](#ipv6-addresses-in-domain-lists), como `"[::1]"`. El proxy verifica esta lista para admitir la conexión.
* **`injectHosts`**: la dirección desnuda en su forma canónica comprimida, como `"::1"` o `"2001:db8::1"`. El proxy compara cada entrada contra la dirección de destino desnuda de la conexión, ignorando puertos, por lo que una ortografía entre corchetes, con ID de zona o comprimida de manera diferente nunca coincide y el proxy nunca inyecta la credencial allí.

`claude doctor` marca entradas `injectHosts` que nunca pueden coincidir con la advertencia `Sandbox credential injectHosts entries can never match their destination`. Esta verificación requiere Claude Code v2.1.229 o posterior.

A diferencia de `deny`, el enmascaramiento autoriza al proxy a enviar su credencial real a los hosts enumerados, por lo que Claude Code lo honra solo desde configuraciones que usted o su administrador controlan: configuración de usuario, configuración administrada y la bandera CLI `--settings`. Claude Code ignora entradas `mask` en el `.claude/settings.json` o `.claude/settings.local.json` de un repositorio. En esos archivos también ignora `network.tlsTerminate` y [`credentials.allowPlaintextInject`](/docs/es/settings-reference#sandbox-credentials-allowplaintextinject), la configuración que permite al proxy inyectar credenciales en solicitudes sin cifrar. Si [excluye la configuración de usuario](#configure-sandboxing), Claude Code descarta las entradas `mask` de variable de entorno en `~/.claude/settings.json` también.

Cuando su administrador entrega entradas `mask`, `network.tlsTerminate` o `credentials.allowPlaintextInject` a través de configuración administrada por servidor, cuentan como [configuraciones que necesitan aprobación](/docs/es/server-managed-settings#security-approval-dialogs).

Cuando la misma variable se enumera con `deny` en cualquier ámbito, `deny` tiene prioridad.

El enmascaramiento reemplaza el valor completo de la variable de forma predeterminada, lo que se adapta a un token desnudo. Los campos de entrada opcionales, que requieren Claude Code v2.1.224 o posterior, manejan valores con estructura:

* `extract`: una expresión regular que Claude Code aplica en todo el valor, reemplazando solo el texto capturado por el grupo 1 de cada coincidencia, por lo que una herramienta que analiza el valor, como una cadena de conexión `DATABASE_URL`, aún funciona dentro del sandbox. El patrón debe contener al menos un grupo de captura.
* `onExtractNoMatch` controla qué sucede cuando el patrón no coincide con nada:
  * `warn`, el valor predeterminado, advierte y pasa la variable sin enmascarar
  * `deny` desactiva la variable dentro del sandbox
  * `error` detiene la configuración del sandbox hasta que corrija la configuración
* `decode: "jwt"`: para una variable que contiene un JSON Web Token (JWT). Claude Code verifica que el valor sea un JWT y lo reemplaza con un token falso estructuralmente válido, por lo que el código dentro del sandbox que decodifica el token sigue funcionando. Agregue `maskClaims` para enumerar reclamaciones de carga útil de nivel superior para enmascarar individualmente en lugar de reemplazar el token completo; las otras reclamaciones permanecen legibles. Cuando el valor no se verifica como JWT, o ninguna reclamación enumerada coincide, Claude Code pasa la variable sin enmascarar con una advertencia. `decode` no se puede combinar con `extract`.

Consulte las [filas `credentials.envVars[]` en la referencia de configuración](/docs/es/settings-reference#sandbox-settings) para la lista de campos completa.

<h4 id="re-sign-aws-requests">
  Re-firmar solicitudes de AWS
</h4>

Las solicitudes de AWS llevan firmas SigV4 sobre el contenido de la solicitud, por lo que enmascare `AWS_ACCESS_KEY_ID` y `AWS_SECRET_ACCESS_KEY` juntos. El proxy detecta una solicitud SigV4 por el centinela de la clave de acceso y la re-firma después de sustituir los valores reales. Enmascarar solo el secreto deja solicitudes firmadas con el marcador de posición, que el proxy no puede detectar, por lo que fallan en AWS; Claude Code advierte sobre este caso al inicio, pero no cuando solo se enmascara el ID de clave de acceso. Una solicitud detectada que el proxy no puede re-firmar, como una que falta su encabezado `x-amz-date`, falla con un error de proxy en lugar de llegar al servidor con una firma rota.

Claude Code vincula las variables convencionales `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` y `AWS_SESSION_TOKEN` en una credencial automáticamente cuando enmascara sus valores completos. Si su credencial de AWS vive en variables con otros nombres, agrúpelas usted mismo con [`credentials.awsPairs`](/docs/es/settings-reference#sandbox-credentials-awspairs), que requiere Claude Code v2.1.224 o posterior. Este ejemplo agrega el emparejamiento a una configuración que ya enmascara `MY_KEY_ID`, `MY_SECRET_KEY` y `MY_SESSION_TOKEN` completos, como en la [configuración de enmascaramiento anterior](#mask-environment-variables):

```json theme={null}
{
  "sandbox": {
    "credentials": {
      "awsPairs": [
        {
          "accessKeyIdVar": "MY_KEY_ID",
          "secretAccessKeyVar": "MY_SECRET_KEY",
          "sessionTokenVar": "MY_SESSION_TOKEN"
        }
      ]
    }
  }
}
```

Cada entrada sigue estas reglas:

* `accessKeyIdVar` y `secretAccessKeyVar` nombran las entradas `envVars` enmascaradas que contienen el ID de clave de acceso y la clave secreta. El `sessionTokenVar` opcional nombra la entrada que contiene el token de sesión para credenciales temporales; cuando se establece, el proxy envía el token real como `x-amz-security-token` en solicitudes re-firmadas.
* Cada variable nombrada debe ser una entrada `mask` que enmascara su valor completo, sin `extract` o `decode`.
* El proxy re-firma solicitudes en los hosts enumerados en la entrada `injectHosts` del ID de clave de acceso.
* Nombrar cualquiera de las variables convencionales en un par reemplaza el emparejamiento automático.

Como entradas `mask`, `awsPairs` se honra solo desde configuración de usuario, configuración administrada y la bandera CLI `--settings`.

Tres formas de solicitud de AWS llevan firmas que el proxy no puede recomputar. Cuando tal solicitud se firma con el marcador de posición de un par enmascarado, el proxy la falla en lugar de reenviar una firma rota; las solicitudes firmadas con credenciales sin enmascarar nunca se ven afectadas. La configuración [`credentials.sigv4`](/docs/es/settings-reference#sandbox-credentials-sigv4), que requiere Claude Code v2.1.224 o posterior, relaja esto por forma: establecer la clave de una forma en `passthrough` reenvía la solicitud con su firma derivada de marcador de posición, por lo que la herramienta que llama recibe el rechazo propio de AWS en lugar de un error de proxy. Como `awsPairs`, `sigv4` se honra solo desde configuración de usuario, configuración administrada y la bandera CLI `--settings`.

| Forma de solicitud                | Clave `sigv4` | Por qué el proxy no puede re-firmarla                                                                                 |
| :-------------------------------- | :------------ | :-------------------------------------------------------------------------------------------------------------------- |
| cargas de transmisión aws-chunked | `streaming`   | Las firmas por fragmento se encadenan desde la firma de semilla, por lo que re-firmar requeriría reescribir el cuerpo |
| URLs pre-firmadas                 | `presigned`   | La firma vive en la URL en sí, sin encabezado `Authorization`                                                         |
| Firmas asimétricas SigV4A         | `sigv4a`      | No hay HMAC de clave compartida para recomputar                                                                       |

<h4 id="mask-credential-files">
  Enmascarar archivos de credenciales
</h4>

Las entradas de archivo también aceptan `"mode": "mask"`, que requiere Claude Code v2.1.221 o posterior. Lo que ve un comando aislado depende de la plataforma:

* **Linux y WSL2**: los comandos aislados leen una copia centinela del archivo, un sustituto cuyo secreto se reemplaza con un valor de marcador de posición, y el [proxy del sandbox](#network-isolation) sustituye el valor real en salida.
* **macOS**: los comandos aislados no pueden leer el archivo enumerado en absoluto. Claude Code no construye ninguna copia centinela y no sustituye nada en salida, por lo que las herramientas que se autentican con el archivo no funcionan dentro del sandbox, el mismo efecto que `deny`. A diferencia de una entrada `deny`, el bloqueo de lectura se mantiene incluso cuando [deshabilita el aislamiento del sistema de archivos](#disable-filesystem-isolation).

En cada plataforma, Claude Code aplica el requisito [`network.tlsTerminate`](/docs/es/settings-reference#sandbox-network-tlsterminate) e `injectHosts` de la misma manera que para [variables de entorno enmascaradas](#mask-environment-variables), e ignora configuración de repositorio de la misma manera. Si [excluye la configuración de usuario](#configure-sandboxing), Claude Code mantiene las entradas `mask` de archivo en `~/.claude/settings.json` como restricciones, pero las entradas ya no autorizan al proxy a sustituir el valor real.

El ejemplo a continuación enmascara un token de GitHub almacenado en `~/.config/gh/hosts.yml`; el patrón `extract`, cubierto a continuación, le dice a Claude Code qué parte del archivo es el secreto. En Linux y WSL2, los comandos aislados que leen el archivo obtienen un centinela en lugar del token, y el proxy sustituye el token real en solicitudes a `api.github.com`:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "network": {
      "tlsTerminate": {},
      "allowedDomains": ["*.github.com"]
    },
    "credentials": {
      "files": [
        {
          "path": "~/.config/gh/hosts.yml",
          "mode": "mask",
          "extract": "oauth_token:\\s*(\\S+)",
          "injectHosts": ["api.github.com"]
        }
      ]
    }
  }
}
```

Para confirmar que la máscara está activa, pida a Claude que ejecute `cat ~/.config/gh/hosts.yml` en un comando aislado: en Linux y WSL2 la salida muestra un valor centinela en lugar del token, y en macOS la lectura falla en su lugar.

En Linux y WSL2, el patrón `extract` es lo que mantiene el resto de `hosts.yml` legible. Claude Code aplica la expresión regular en todo el archivo y reemplaza solo el texto capturado por el grupo 1 de cada coincidencia, por lo que `gh` aún analiza su configuración y solo el token es un marcador de posición. Use `extract` para cualquier archivo estructurado que las herramientas analicen, como `.netrc`, JSON o YAML; el patrón debe contener al menos un grupo de captura. Sin `extract`, Claude Code reemplaza todo el contenido del archivo con un valor centinela, que se adapta a un archivo que contiene un único secreto desnudo y nada más.

Para un archivo que contiene un JSON Web Token (JWT), establezca `decode: "jwt"` en lugar de, o junto con, `extract`. `decode` requiere Claude Code v2.1.224 o posterior. Claude Code encuentra candidatos JWT con un patrón integrado, o con su patrón `extract` cuando se establece, verifica que cada candidato sea un JWT y lo reemplaza con un token falso estructuralmente válido, por lo que el código que decodifica el token dentro del sandbox sigue funcionando. Agregue `maskClaims` para enmascarar solo las reclamaciones de carga útil de nivel superior nombradas dentro de cada token verificado y deje las otras reclamaciones legibles. Cuando ningún candidato se verifica, o ninguna reclamación nombrada coincide, el campo `onExtractNoMatch` a continuación rige el resultado, como lo hace para un patrón que no coincide con nada.

Dos campos opcionales refinan cómo se comporta la coincidencia. Ambos se aplican solo cuando `mode` es `mask` y `extract` o `decode` está establecido. En macOS, Claude Code aplica entradas `mask` como `deny` antes de que se ejecute el patrón siempre que el aislamiento del sistema de archivos esté activado, por lo que estos campos, y los resultados de no coincidencia a continuación, tienen efecto allí solo cuando [deshabilita el aislamiento del sistema de archivos](#disable-filesystem-isolation):

* `onExtractNoMatch` controla qué sucede cuando la coincidencia no encuentra nada para enmascarar en el archivo:

  * `warn`, el valor predeterminado, advierte y omite la entrada, por lo que los comandos aislados pueden leer el archivo real sin enmascarar. El valor predeterminado se adapta a credenciales que pueden estar legítimamente ausentes; si el secreto podría estar presente pero el patrón podría no detectarlo, use `deny`
  * `deny` hace que el archivo sea ilegible en su lugar
  * `error` detiene la configuración del sandbox hasta que corrija la configuración

  Claude Code trata `deny` como `error` siempre que el bloqueo de lectura no se aplicaría: cuando [deshabilita el aislamiento del sistema de archivos](#disable-filesystem-isolation), y cuando una entrada `filesystem.allowRead` de cualquier fuente de configuración reabre la ruta del archivo.
* `maskDuplicates` también reemplaza copias verbatim de cada valor de credencial enmascarado, una captura `extract` o un token verificado `decode`, encontrado fuera de los intervalos coincidentes, para un secreto repetido donde la coincidencia no llega. Coincide con subcadenas sin procesar, por lo que un valor corto o común se reemplazaría en todas partes donde aparece; resérvelo para secretos largos y de alta entropía. Valor predeterminado: false.

`mask` se aplica a un único archivo, por lo que enumere cada archivo de credencial individualmente. Claude Code retrocede a `deny` para una entrada `mask` que no puede enmascarar de forma segura: una ruta de directorio, un patrón glob, un archivo más grande que 8 MiB o un archivo que no es texto UTF-8. Escriba directorios como entradas explícitas `deny` en su lugar; la tabla bajo [Qué configuraciones pueden deshabilitarlo](#which-settings-can-disable-it) cubre si cada forma fija `filesystem.disabled` y cómo se comporta con el aislamiento del sistema de archivos desactivado.

<h2 id="how-sandboxing-works">
  Cómo funciona el sandboxing
</h2>

<h3 id="filesystem-isolation">
  Aislamiento del sistema de archivos
</h3>

La herramienta Bash aislada restringe el acceso al sistema de archivos a directorios específicos:

* **Comportamiento de escritura predeterminado**: acceso de lectura y escritura al directorio de trabajo actual y sus subdirectorios, cualquier directorio que haya agregado con `--add-dir`, `/add-dir`, o [`permissions.additionalDirectories`](/docs/es/settings-reference#permissions-additionaldirectories), más el directorio temporal de sesión al que apunta `$TMPDIR`
* **Comportamiento de lectura predeterminado**: acceso de lectura a toda la computadora, excepto ciertos directorios denegados. Tenga en cuenta que este comportamiento predeterminado aún permite leer archivos de credenciales como `~/.aws/credentials` y `~/.ssh/`. Utilice [`sandbox.credentials`](#protect-credentials) para bloquear lecturas de estos archivos y desconfigurar variables de entorno secretas, o agregue las rutas a `denyRead`.
* **Acceso bloqueado**: no puede modificar archivos fuera del directorio de trabajo, directorios agregados y directorio temporal de sesión sin permiso explícito, incluidos archivos de configuración de shell como `~/.bashrc` y binarios del sistema en `/bin/`
* **Git worktrees**: cuando el directorio de trabajo es un [git worktree vinculado](/docs/es/worktrees), el sandbox también permite escrituras en el directorio compartido `.git` del repositorio principal para que comandos como `git commit` puedan actualizar referencias e índices. Las escrituras a `hooks/` y `config` dentro de ese directorio permanecen denegadas.
* **Configurable**: defina rutas permitidas y denegadas personalizadas a través de la configuración

Para omitir completamente el aislamiento del sistema de archivos mientras se mantiene el aislamiento de red, establezca [`sandbox.filesystem.disabled`](#disable-filesystem-isolation).

<h3 id="protected-paths">
  Rutas protegidas
</h3>

Dentro de los directorios en los que los comandos aislados pueden escribir, el sandbox aún deniega escrituras en los archivos desde los que Claude Code carga configuración y código. Un comando que pudiera editar esos archivos podría otorgarse permisos a sí mismo, o agregar un hook o servidor MCP que Claude Code ejecuta fuera del sandbox. El sistema de permisos tiene sus propias [rutas protegidas](/docs/es/permission-modes#protected-paths), que controlan lo que Claude Code aprueba antes de que se ejecute una herramienta; la lista del sandbox se aplica a un comando que ya está en ejecución. Cubre cuatro grupos de rutas:

* **En su directorio de trabajo y los directorios por encima de él**: los archivos de configuración `.claude`, los directorios `.claude/skills`, `.claude/agents`, `.claude/commands` y `.claude/hooks`, `.mcp.json`, y los archivos que Claude Code ejecuta por su cuenta, como `.claude/workflows` y `.claude/scheduled_tasks.json`
* **Solo en su directorio de trabajo**: archivos de inicio de shell como `.bashrc` y `.zshrc`, `.gitconfig`, los directorios `.vscode` e `.idea`, y `hooks` y `config` dentro de `.git`
* **Archivos que convertirían su directorio de trabajo en un repositorio git desnudo**: `HEAD`, `objects` y `refs` en el nivel superior, más `config` y `hooks` allí cuando un `HEAD` se sienta junto a ellos. Un archivo denominado `config` se deniega incluso sin `HEAD`. En Linux y WSL2, el sandbox elimina un archivo `HEAD` de nivel superior o un directorio `objects` o `refs` que aparece mientras se ejecuta un comando aislado
* **En `~/.claude`, o el directorio al que apunta `CLAUDE_CONFIG_DIR`**: la mayoría de su contenido, más `~/.claude.json` y el almacén de credenciales `.credentials.json`

Si aparece un enlace simbólico en la ruta de un archivo de configuración protegido durante la sesión, el sandbox también deniega escrituras en el archivo al que apunta, comenzando con el siguiente comando.

No hay forma de eximir una de estas rutas: una entrada `allowWrite` o una regla de permiso `Edit` que cubra la ruta no levanta la protección. La única forma de desactivar la protección es [`filesystem.disabled`](#disable-filesystem-isolation), que desactiva el aislamiento del sistema de archivos para cada ruta. Para ver la mayoría de estas rutas resueltas para su máquina, ejecute `/sandbox` y abra la pestaña **Config**, que las enumera bajo **Denied within allowed**, mezcladas con sus propias entradas `denyWrite`.

Si `git merge` o `git checkout` falla con `unable to unlink old` en una de estas rutas, consulte [Troubleshooting](#troubleshooting).

<h3 id="network-isolation">
  Aislamiento de red
</h3>

El acceso a la red se controla a través de un servidor proxy que se ejecuta fuera del sandbox:

* **Restricciones de dominio**: Claude Code no pre-permite dominios de forma predeterminada. La primera vez que un comando necesita un nuevo dominio, Claude Code solicita aprobación; en [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode), Claude en su lugar nombra los hosts que un comando necesita en el comando mismo, según [Dominios permitidos por comando](#per-command-allowed-domains-in-auto-mode).
* **Opciones de aprobación**: si elige Sí cuando se le solicite, Claude Code permite el host para el resto de la sesión actual y no solicita nuevamente para conexiones posteriores al mismo host. Si elige "Sí, y no vuelvas a preguntar", Claude Code guarda una regla de permiso `WebFetch(domain:...)` en su [configuración local](/docs/es/permissions#permission-system), por lo que el host permanece permitido en sesiones futuras.
* **Dominios pre-permitidos**: pre-permita dominios con [`allowedDomains`](/docs/es/settings-reference#sandbox-network-alloweddomains) para evitar completamente la solicitud. Claude Code también pre-permite dominios de reglas de permiso `WebFetch(domain:...)`, como se describe en [Reglas de permisos](#permission-rules).
* **Lista de permitidos estricta**: si establece [`strictAllowlist`](/docs/es/settings-reference#sandbox-network-strictallowlist) en `true` en la configuración de usuario, administrada o CLI `--settings`, Claude Code deniega a los comandos aislados el acceso a cualquier host fuera de la lista de permitidos en lugar de solicitar. La lista de permitidos es la misma contra la que el sandbox solicita de otra manera: `allowedDomains` más dominios de reglas de permiso `WebFetch(domain:...)`, o solo las entradas de configuración administrada cuando se establece `allowManagedDomainsOnly`. Claude Code aplica esto solo para comandos aislados; las herramientas en proceso como `WebFetch` aún siguen sus [reglas de permiso](#permission-rules). Establecerlo en `.claude/settings.json` o `.claude/settings.local.json` de un repositorio no tiene efecto. Requiere Claude Code v2.1.219 o posterior.
* **Bloqueo administrado**: si [`allowManagedDomainsOnly`](/docs/es/settings-reference#sandbox-network-allowmanageddomainsonly) se establece en la configuración administrada, los dominios no permitidos se bloquean automáticamente en lugar de solicitar, y solo se honran `allowedDomains` y reglas de permiso `WebFetch(domain:...)` de la configuración administrada.
* **Proxy corporativo**: cuando su red requiere que el tráfico saliente pase a través de un proxy corporativo, establezca `HTTPS_PROXY`, `HTTP_PROXY` y `NO_PROXY` como describe [proxy configuration](/docs/es/network-config#proxy-configuration), en el bloque `env` de su configuración para que [agentes de fondo](/docs/es/network-config#set-network-variables-in-settings-not-the-shell) también los obtengan, o en el entorno desde el que inicia Claude Code. Claude Code aplica la lista de permitidos de dominio y luego canaliza las conexiones permitidas a través de ese proxy ascendente.
* **Soporte de proxy personalizado**: los usuarios avanzados pueden implementar reglas personalizadas en el tráfico saliente
* **Cobertura integral**: las restricciones se aplican a todos los scripts, programas y subprocesos generados por comandos

En una regla `WebFetch(domain:...)`, el sandbox honra dos formas de comodín: un `*.` inicial, como `*.example.com`, y un `*` desnudo. La forma `*` desnuda requiere Claude Code v2.1.186 o posterior. Un comodín en cualquier otra posición, como `WebFetch(domain:example.*)`, aún coincide con búsquedas pero no tiene efecto en comandos aislados.

<Note>
  El proxy integrado aplica la lista de permitidos basada en el nombre de host solicitado y, de forma predeterminada, no termina ni inspecciona el tráfico TLS. La configuración experimental [`network.tlsTerminate`](/docs/es/settings-reference#sandbox-network-tlsterminate), disponible en Claude Code v2.1.199 y posterior, hace que el proxy integrado termine TLS por sí mismo, lo que requieren las entradas de credenciales [`mask`](#mask-credentials). Consulte [Limitaciones de seguridad](#security-limitations) para las implicaciones del comportamiento predeterminado, y [Configuración de proxy personalizado](#custom-proxy-configuration) si su modelo de amenaza requiere inspección de TLS.
</Note>

<h4 id="per-command-allowed-domains-in-auto-mode">
  Dominios permitidos por comando en modo automático
</h4>

En [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) con sandboxing activado, Claude nombra los hosts que un comando necesita en el comando mismo en lugar de desencadenar una aprobación de red para cada conexión. Cada comando Bash, PowerShell o [Monitor](/docs/es/tools-reference#monitor-tool) que se ejecuta en el sandbox puede llevar una lista de hosts más allá de la lista de permitidos del sandbox: un dominio como `registry.npmjs.org`, un comodín como `*.pythonhosted.org`, o una dirección IP, cada uno con un `:port` opcional. El clasificador revisa los hosts junto con el comando. Requiere Claude Code v2.1.271 o posterior.

Una lista aprobada abre esos hosts solo para ese comando, mientras se ejecuta. Nada se agrega a los hosts permitidos de su sesión o su configuración; el siguiente comando nombra sus propios hosts.

Un comando que lleva hosts va al clasificador en lugar de ser aprobado por una regla de permiso o el [modo de auto-permitir](#sandbox-modes) del sandbox. Si una [regla ask](/docs/es/permissions#manage-permissions) fuerza una solicitud para el comando, el diálogo de permiso en su terminal enumera los hosts junto a él, y aprobar allí cubre ambos.

Una lista por comando amplía solo lo que el sandbox deniega de forma predeterminada. Las entradas [`deniedDomains`](/docs/es/settings-reference#sandbox-network-denieddomains) aún bloquean. Cuando [`strictAllowlist`](/docs/es/settings-reference#sandbox-network-strictallowlist) o [`allowManagedDomainsOnly`](/docs/es/settings-reference#sandbox-network-allowmanageddomainsonly) bloquea la lista de permitidos, Claude Code rechaza listas por comando.

Mientras se aplican listas por comando, Claude Code rechaza una conexión a un host que ningún comando aprobado listó, sin una solicitud o una verificación del clasificador. El rechazo nombra el host en el resultado del comando, y Claude vuelve a ejecutar el comando con el host agregado.

<h4 id="ipv6-addresses-in-domain-lists">
  Direcciones IPv6 en listas de dominios
</h4>

Las listas de dominios del sandbox son `allowedDomains`, `deniedDomains`, y las reglas `WebFetch(domain:...)` que las alimentan. Para coincidir con una dirección IPv6 en cualquiera de ellas, escriba el literal entre corchetes: `"[::1]"` coincide con esa dirección en cada puerto, y `"[::1]:443"` coincide con ella solo en el puerto 443. Escriba el puerto como un número del 1 al 65535 sin ceros iniciales. La forma entre corchetes requiere Claude Code v2.1.229 o posterior. Antes de v2.1.229, cuando el texto después de los dos puntos finales de una entrada sin corchetes era un número de puerto, Claude Code lo leía como uno, por lo que `::1:443` nombraba la dirección `::1` en el puerto 443.

Cuando elige "Sí, y no vuelvas a preguntar" en la solicitud de aprobación de red para una dirección IPv6, Claude Code guarda la regla `WebFetch(domain:...)` con la dirección entre corchetes, por lo que la regla sigue coincidiendo con la dirección en sesiones futuras.

Una entrada sin corchetes con dos o más dos puntos es ambigua: `::1:443` es tanto una dirección IPv6 completa como una dirección seguida de un puerto. Claude Code aplica deletreos ambiguos de forma conservadora en lugar de adivinar cuál lectura pretendía:

* **Listas de denegación**: Claude Code deniega cada lectura que la entrada analiza, por lo que cualquiera que sea la lectura que pretendía está bloqueada. Para una entrada sin lectura analizable, Claude Code no bloquea nada.
* **Listas de permitidos**: Claude Code nunca permite más de lo que escribió. Reescribe una entrada ambigua a su lectura de host-y-puerto cuando esa lectura se analiza limpiamente, y puede descartar la entrada completamente en lugar de ampliar la lista de permitidos.

Ejecute `claude doctor` en su terminal para encontrar las entradas afectadas: la advertencia `Sandbox network domain entries have unreliable spellings` nombra hasta tres de ellas y cuenta el resto. Reescriba cada una en la forma entre corchetes para borrar la advertencia. La advertencia también nombra entradas cuyo deletreo es poco confiable por otras razones, como `@`, caracteres de ruta o consulta, o comodines dentro de corchetes.

<h3 id="os-level-enforcement">
  Aplicación a nivel del sistema operativo
</h3>

La herramienta Bash aislada utiliza primitivas de seguridad del sistema operativo:

* **macOS**: utiliza Seatbelt para la aplicación del sandbox
* **Linux**: utiliza [bubblewrap](https://github.com/containers/bubblewrap) para el aislamiento
* **WSL2**: utiliza bubblewrap, igual que Linux

WSL1 no es compatible porque bubblewrap requiere características del kernel solo disponibles en WSL2.

Estas mismas primitivas están disponibles como el paquete independiente [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime), que la página [Sandbox environments](/docs/es/sandbox-environments#sandbox-runtime) cubre como un enfoque separado para envolver todo el proceso de Claude Code.

<h2 id="how-sandboxing-relates-to-permissions-and-permission-modes">
  Cómo se relaciona el sandboxing con los permisos y modos de permiso
</h2>

El sandboxing, las [reglas de permiso](/docs/es/permissions) y los [modos de permiso](/docs/es/permission-modes) son capas complementarias. Las secciones a continuación cubren cómo el sandbox interactúa con cada una.

<h3 id="permission-rules">
  Reglas de permiso
</h3>

Las reglas de permiso y el sandboxing controlan cosas diferentes:

* **Reglas de permiso** controlan qué herramientas puede usar Claude Code y se evalúan antes de que se ejecute cualquier herramienta. Se aplican a todas las herramientas: Bash, Read, Edit, WebFetch, MCP y otras, excepto que una regla de denegación o solicitud no puede bloquear [`EndConversation`](/docs/es/tools-reference#endconversation-tool-behavior) mientras que cualquier otra herramienta permanezca.
* **Sandboxing** proporciona aplicación a nivel del sistema operativo que restringe lo que los comandos Bash pueden acceder a nivel del sistema de archivos y la red. Se aplica solo a comandos Bash, PowerShell y [Monitor](/docs/es/tools-reference#monitor-tool) y sus procesos secundarios.

Las dos capas también difieren en cómo se aplican. Claude Code evalúa las decisiones de permiso antes de que se ejecute un comando, basándose en la cadena de comando y, en modo automático, el juicio de un clasificador separado sobre si el comando es seguro. El sistema operativo aplica el límite del sandbox en el proceso en ejecución, por lo que se mantiene independientemente de lo que el modelo eligió ejecutar e incluso si un comando permitido hace más de lo que su nombre sugiere.

Las restricciones del sistema de archivos y la red se configuran tanto a través de la configuración del sandbox como de las reglas de permiso:

| Configuración o regla                                          | Qué hace                                                                                                         |
| :------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------- |
| `sandbox.filesystem.allowWrite`                                | Otorga acceso de escritura de subproceso a rutas fuera del directorio de trabajo                                 |
| `sandbox.filesystem.denyWrite` y `sandbox.filesystem.denyRead` | Bloquea el acceso de subproceso a rutas específicas                                                              |
| `sandbox.filesystem.allowRead`                                 | Permite nuevamente la lectura de rutas específicas dentro de una región `denyRead`                               |
| [`sandbox.filesystem.disabled`](#disable-filesystem-isolation) | Desactiva la capa del sistema de archivos por completo mientras mantiene el aislamiento de red                   |
| Reglas de permitir `Edit`                                      | Otorga acceso de escritura a rutas específicas, de la misma manera que `sandbox.filesystem.allowWrite`           |
| Reglas de denegar `Read` y `Edit`                              | Bloquea el acceso a archivos o directorios específicos                                                           |
| Reglas de permitir y denegar `WebFetch(domain:...)`            | Controla el acceso al dominio                                                                                    |
| `allowedDomains` del sandbox                                   | Controla qué dominios pueden alcanzar los comandos Bash                                                          |
| `deniedDomains` del sandbox                                    | Bloquea dominios específicos incluso cuando un comodín `allowedDomains` más amplio de otra manera los permitiría |

Las rutas y dominios de ambas configuraciones del sandbox y reglas de permiso se fusionan en la configuración final del sandbox.

El [directorio de ejemplos del repositorio claude-code](https://github.com/anthropics/claude-code/tree/main/examples/settings) incluye configuraciones de configuración de inicio para escenarios de implementación comunes, incluidos ejemplos específicos del sandbox. Úselos como puntos de partida y ajústelos para que se adapten a sus necesidades.

<h3 id="permission-modes">
  Modos de permiso
</h3>

`/sandbox` no es un [modo de permiso](/docs/es/permission-modes). Los modos de permiso deciden si se ejecuta una llamada de herramienta y si se le solicita primero, mientras que el sandbox restringe lo que un comando Bash puede acceder una vez que se ejecuta. Difieren en lo que controlan y qué reemplaza la solicitud por acción:

|                                                                          | Qué controla                                                | Qué reemplaza la solicitud                                                                                                                                                                                           |
| :----------------------------------------------------------------------- | :---------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/sandbox`                                                               | Lo que un comando Bash puede acceder una vez que se ejecuta | El límite del sandbox en sí, en [modo auto-allow](#sandbox-modes)                                                                                                                                                    |
| [Modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) | Si se ejecuta cada llamada de herramienta                   | Un clasificador que revisa acciones                                                                                                                                                                                  |
| `--dangerously-skip-permissions`                                         | Si se ejecuta cada llamada de herramienta                   | Nada. Las verificaciones de [ruta protegida](/docs/es/permission-modes#protected-paths) también se omiten; las [acciones que ningún modo auto-aprueba](/docs/es/permission-modes#actions-no-mode-auto-approves) aún se aplican |

El [modo auto-allow](#sandbox-modes) del sandbox es separado del [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode): auto-allow aprueba comandos Bash porque el límite del sandbox los contiene, mientras que el modo automático usa un clasificador para revisar acciones. Los dos funcionan independientemente y pueden combinarse, con las excepciones enumeradas en [Modos sandbox](#sandbox-modes). Para elegir un límite de aislamiento para ejecuciones desatendidas, consulte [Entornos sandbox](/docs/es/sandbox-environments#how-isolation-relates-to-permission-modes). Para una tabla de emparejamientos comunes de modo de permiso y sandbox con los indicadores que inician cada uno, consulte [Configuraciones comunes](/docs/es/permission-modes#common-setups).

<h2 id="configure-the-sandbox-for-your-organization">
  Configurar el sandbox para su organización
</h2>

Los administradores pueden requerir sandboxing para cada usuario, evitar que los desarrolladores amplíen la política y enrutar el tráfico del sandbox a través de un proxy corporativo.

<h3 id="enforce-sandboxing-with-managed-settings">
  Aplicar sandboxing con configuración administrada
</h3>

Para requerir el sandbox para cada desarrollador, entregue las claves `sandbox` a través de [configuración administrada](/docs/es/managed-settings#delivery-mechanisms), ya sea como un archivo administrado por su MDM o a través de [configuración administrada por servidor](/docs/es/server-managed-settings) en claude.ai.

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
* **`allowUnsandboxedCommands: false`**: Claude Code ignora la salida de emergencia `dangerouslyDisableSandbox`, por lo que cuando un comando falla bajo el sandbox, Claude no puede reintentarlo sin aislar

Dos adiciones vale la pena considerar junto con ellas. Agregue `excludedCommands` para cualquier herramienta aprobada por la organización que deba ejecutarse sin aislamiento. Agregue entradas [`sandbox.credentials`](#protect-credentials) para directorios de credenciales como `~/.aws` y `~/.ssh` y para variables de entorno secretas, ya que la política de lectura predeterminada aún las permite.

Esta configuración aísla los comandos que ejecuta Claude. Un desarrollador aún puede escribir un comando en el [símbolo del sistema del modo shell `!`](/docs/es/interactive-mode#shell-mode-with-prefix) y ejecutarlo fuera del sandbox, con el mismo acceso que ya tienen en cualquier terminal fuera de Claude Code. Consulte [La salida de emergencia de reintento sin aislar](#the-unsandboxed-retry-escape-hatch) para las sesiones donde los comandos escritos se ejecutan aislados.

El sandbox no se ejecuta en Windows nativo, por lo que si su flota incluye hosts de Windows, limite esta configuración a macOS y Linux o haga que esos usuarios ejecuten Claude Code dentro de WSL2 o un contenedor.

<h3 id="keep-developers-from-widening-the-policy">
  Evitar que los desarrolladores amplíen la política
</h3>

Para claves booleanas como `enabled` y `failIfUnavailable`, Claude Code usa el valor administrado e ignora cualquier cosa que un desarrollador establezca localmente. Para claves de array como `excludedCommands` y `allowRead`, Claude Code fusiona entradas de cada ámbito que carga la sesión, por lo que un desarrollador puede agregar entradas que amplíen la política.

Establezca `allowManagedReadPathsOnly` en `true` en la configuración administrada para que solo se honren las entradas `allowRead` de la configuración administrada. Esto evita que los desarrolladores amplíen el acceso de lectura más allá de las rutas aprobadas por la organización. Para bloquear dominios de red a los valores administrados de la misma manera, establezca [`allowManagedDomainsOnly`](/docs/es/settings-reference#sandbox-network-allowmanageddomainsonly).

Cuando la configuración administrada configura `sandbox.filesystem` o enumera cualquier entrada `sandbox.credentials.files` con `"mode": "deny"`, solo la configuración administrada puede establecer [`filesystem.disabled`](#disable-filesystem-isolation), por lo que los desarrolladores no pueden desactivar las restricciones de aislamiento del sistema de archivos implementadas por el administrador. Si una entrada `mask` fija la clave depende de cómo se resuelva; la tabla bajo [Qué configuración puede desactivarla](#which-settings-can-disable-it) cubre los cuatro casos.

`excludedCommands` no tiene un bloqueo equivalente solo administrado, por lo que un desarrollador siempre puede agregar entradas que ejecuten comandos adicionales fuera del sandbox. Mantenga la lista administrada estrecha.

<h3 id="custom-proxy-configuration">
  Configuración de proxy personalizado
</h3>

Para organizaciones que requieren seguridad de red avanzada, puede implementar un proxy personalizado para:

* Descifrar e inspeccionar tráfico HTTPS
* Aplicar reglas de filtrado personalizadas
* Registrar todas las solicitudes de red
* Integrar con infraestructura de seguridad existente

Para apuntar Claude Code a su proxy, establezca los puertos del proxy en [configuración de sandbox](/docs/es/settings-reference#sandbox-settings):

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
* **Las CLI basadas en Go fallan en la verificación de TLS en macOS**: herramientas como `gh`, `gcloud` y `terraform` pueden fallar en la verificación de TLS bajo Seatbelt. Liste estas herramientas en [`excludedCommands`](/docs/es/settings-reference#sandbox-excludedcommands). Si está usando `httpProxyPort` con un proxy MITM y CA personalizado, establezca [`enableWeakerNetworkIsolation`](/docs/es/settings-reference#sandbox-enableweakernetworkisolation) en `true` en su lugar.
* **`open`, `osascript` o los flujos de autenticación basados en navegador fallan con el error `-600` en macOS**: el sandbox bloquea Apple Events de forma predeterminada. Establezca [`allowAppleEvents`](/docs/es/settings-reference#sandbox-allowappleevents) en `true` en su configuración de usuario, administrada o CLI para permitirlos. La configuración del proyecto se ignora para esta clave. Habilitarlo elimina el aislamiento de ejecución de código, ya que los comandos aislados pueden entonces lanzar otras aplicaciones sin aislar sin solicitud del usuario y enviar comandos AppleScript a aplicaciones en ejecución, sujeto a la solicitud de consentimiento de automatización de macOS (TCC). Alternativamente, agregue el comando a [`excludedCommands`](/docs/es/settings-reference#sandbox-excludedcommands).
* **Los comandos `docker` fallan**: `docker` es incompatible con el sandbox. Agregue `docker *` a [`excludedCommands`](/docs/es/settings-reference#sandbox-excludedcommands).
* **`pbcopy`, `xclip` o `wl-copy` no actualiza el portapapeles**: estas utilidades de portapapeles pueden fallar al alcanzar el portapapeles del sistema desde dentro del sandbox, en cuyo caso el texto canalizado hacia ellas no llega.

  Para poner la salida de Claude en su portapapeles, pida a Claude que la imprima en su respuesta, luego ejecute [`/copy`](/docs/es/commands). `/copy` escribe en el portapapeles desde el proceso Claude Code en lugar de desde un comando aislado.

  Cuando Claude canaliza texto a una de estas herramientas, agregar la herramienta a [`excludedCommands`](/docs/es/settings-reference#sandbox-excludedcommands) no saca esa llamada del sandbox por sí sola.
* **Un comando git falla con `unable to unlink old`**: `git merge`, `git checkout` y comandos similares fallan de esta manera cuando necesitan reemplazar un archivo al que el sandbox deniega escrituras, ya sea que ese archivo esté bajo una [ruta protegida](#protected-paths) como `.claude/skills`, bajo una de sus entradas `denyWrite`, o fuera de los directorios en los que el sandbox permite que los comandos escriban en absoluto. En Linux y WSL2 el error termina con `Read-only file system`.

  Después de la falla, Claude puede [ofrecer ejecutar nuevamente el comando fuera del sandbox](#the-unsandboxed-retry-escape-hatch); apruebe ese reintento o ejecute el comando git usted mismo en otra terminal. Si ha establecido `allowUnsandboxedCommands` en `false`, Claude no puede ofrecer el reintento, así que ejecute el comando usted mismo. Si el mismo comando git falla a menudo, agréguelo a [`excludedCommands`](/docs/es/settings-reference#sandbox-excludedcommands).
* **Bubblewrap falla al iniciarse dentro de un contenedor**: en un contenedor sin privilegios, bubblewrap no puede montar un sistema de archivos `/proc` nuevo, por lo que los comandos aislados fallan con un error `bwrap` como `Can't mount proc on /newroot/proc: Operation not permitted`. Establezca [`enableWeakerNestedSandbox`](/docs/es/settings-reference#sandbox-enableweakernestedsandbox) en `true` para que el sandbox interno monte el `/proc` existente del contenedor en su lugar. Solo use esta configuración cuando el contenedor externo ya proporcione el límite de aislamiento que necesita, ya que expone información de proceso a comandos aislados que un montaje `/proc` nuevo ocultaría.
* **Los archivos de solo lectura de 0 bytes aparecen en las rutas de configuración `.claude`, y "Sí, y no preguntar de nuevo" no guarda**: en Linux y WSL2, el sandbox mantiene una denegación de escritura en un archivo que aún no existe creando un marcador de posición de solo lectura de 0 bytes allí mientras se ejecuta un comando aislado. El sandbox elimina el marcador de posición después. Si una sesión se mata antes de que se ejecute esa limpieza, por ejemplo por SIGKILL, los marcadores de posición permanecen. Las sesiones posteriores los vinculan de solo lectura nuevamente en cada inicio, por lo que una escritura de configuración como guardar una opción de permiso falla donde se encuentra uno.

  Ejecute `claude doctor` para enumerar los archivos de marcador de posición restantes. La advertencia [`Stale sandbox mask files left by a killed session`](/docs/es/errors#stale-sandbox-mask-files-left-by-a-killed-session) nombra hasta tres de ellos y cuenta el resto. Elimine cada archivo con `rm` mientras no se ejecute ninguna otra sesión de Claude Code en ese proyecto. Antes de v2.1.257, Claude Code dejaba los mismos marcadores de posición sin marcarlos.
* **`--dangerously-skip-permissions` falla como root**: esta bandera se bloquea cuando se ejecuta como root o a través de sudo en Linux y macOS, porque el acceso root combinado sin solicitudes de permiso puede modificar cualquier archivo o servicio en el sistema. La verificación se omite automáticamente dentro de un sandbox reconocido. Para ejecutar de manera autónoma en un contenedor, use la configuración [contenedor de desarrollo](/docs/es/devcontainer), que ejecuta Claude Code como un usuario no root.

<h2 id="limitations">
  Limitaciones
</h2>

El sandboxing reduce el riesgo pero no es un límite de aislamiento completo. Revise las limitaciones a continuación antes de confiar en él como un control de seguridad duro.

<h3 id="security-limitations">
  Limitaciones de seguridad
</h3>

* **Filtrado de red**: el sandbox restringe los dominios a los que se permite que se conecten los procesos. De forma predeterminada, el proxy integrado no termina ni inspecciona TLS en el tráfico saliente, por lo que el contenido de las conexiones cifradas no se examina. La configuración experimental [`network.tlsTerminate`](/docs/es/settings-reference#sandbox-network-tlsterminate) termina TLS en el proxy para [sustitución de credenciales `mask`](#mask-credentials) pero no añade filtrado de contenido. Usted es responsable de asegurarse de que solo se permitan dominios confiables en su política.

<Warning>
  Permitir dominios amplios como `github.com` puede crear caminos para exfiltración de datos. Porque el proxy toma su decisión de permitir del nombre de host suministrado por el cliente sin inspeccionar TLS, el código que se ejecuta dentro del sandbox puede potencialmente usar [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) u técnicas similares para alcanzar hosts fuera de la lista de permitidos. Si su modelo de amenaza requiere garantías más fuertes, configure un [proxy personalizado](#custom-proxy-configuration) que termine TLS e inspeccione tráfico, e instale su certificado CA dentro del sandbox. El aislamiento de red consciente de TLS más fuerte es un área activa de desarrollo.
</Warning>

* **Escalada de privilegios a través de sockets Unix**: la configuración `allowUnixSockets` puede otorgar inadvertidamente acceso a servicios del sistema que podrían llevar a omisiones del sandbox. Por ejemplo, permitir acceso a `/var/run/docker.sock` efectivamente otorga acceso al sistema host a través del socket de Docker. Considere cuidadosamente cualquier socket Unix que permita a través del sandbox.
* **Escalada de permisos del sistema de archivos**: los permisos de escritura del sistema de archivos demasiado amplios pueden permitir ataques de escalada de privilegios. Permitir escrituras en directorios que contienen ejecutables en `$PATH`, directorios de configuración del sistema o archivos de configuración de shell del usuario como `.bashrc` o `.zshrc` puede llevar a ejecución de código en diferentes contextos de seguridad cuando otros usuarios o procesos del sistema acceden a estos archivos.
* **Fortaleza del sandbox de Linux**: la implementación de Linux proporciona un fuerte aislamiento del sistema de archivos y la red pero incluye un modo `enableWeakerNestedSandbox` que le permite funcionar dentro de entornos Docker sin espacios de nombres privilegiados, o en hosts Linux donde los espacios de nombres de usuario sin privilegios están deshabilitados por sysctl. Esta opción debilita considerablemente la seguridad y solo debe usarse cuando se aplica aislamiento adicional de otra manera.
* **Apple Events en macOS**: el sandbox de macOS bloquea Apple Events de forma predeterminada. La configuración `allowAppleEvents` levanta esta restricción para que herramientas como `open` y `osascript` funcionen, pero elimina el aislamiento de ejecución de código: los comandos aislados pueden lanzar otras aplicaciones sin aislar sin solicitud del usuario, y pueden enviar comandos AppleScript a aplicaciones en ejecución, sujeto al aviso de consentimiento de automatización por aplicación de macOS (TCC). Solo se honra desde configuración de usuario, administrada o CLI. La configuración del proyecto no puede habilitarla.

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
* **Variables de entorno**: los comandos Bash aislados heredan el entorno del proceso padre de forma predeterminada, incluidas las credenciales establecidas allí. Use [`sandbox.credentials`](#protect-credentials) para desactivar o enmascarar variables específicas para comandos aislados, o establezca [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/es/env-vars) para eliminar credenciales de todos los subprocesos.
* **Subagentes**: los [subagentes](/docs/es/sub-agents) se ejecutan en el mismo proceso que la sesión padre y usan la misma configuración de sandbox. Los comandos Bash dentro de un subagente están aislados cuando el sandboxing está habilitado en la sesión padre.

<Warning>
  El sandboxing efectivo requiere tanto aislamiento del sistema de archivos como de la red. Sin aislamiento de red, un agente comprometido podría exfiltrar archivos sensibles como claves SSH. Sin aislamiento del sistema de archivos, ya sea por una política permisiva o por [deshabilitar la capa del sistema de archivos](#disable-filesystem-isolation), un agente comprometido podría instalar una puerta trasera en recursos del sistema para obtener acceso a la red. Cuando amplíe los valores predeterminados, verifique que una ruta `allowWrite`, una entrada `allowedDomains` amplia o una excepción `excludedCommands` no deshaga una restricción en el otro lado.
</Warning>

<h2 id="see-also">
  Ver también
</h2>

* [Entornos sandbox](/docs/es/sandbox-environments): comparar el sandbox integrado con contenedores de desarrollo, contenedores y máquinas virtuales
* [Seguridad](/docs/es/security): características de seguridad integral y mejores prácticas
* [Permisos](/docs/es/permissions): configuración de permisos y control de acceso
* [Toda la configuración](/docs/es/settings-reference): cada clave de configuración
* [Referencia de CLI](/docs/es/cli-reference): opciones de línea de comandos
