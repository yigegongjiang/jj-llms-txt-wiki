> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Implementar configuración administrada

> Implementar configuración administrada en la máquina de cada desarrollador: mecanismos de entrega por SO, cómo Claude Code combina fuentes administradas y cómo verificar la aplicación.

La configuración administrada es la configuración que su organización implementa en la máquina de cada desarrollador. Claude Code la aplica por encima de todos los demás niveles, por lo que ningún valor de usuario, proyecto, local o `--settings` la anula, excepto por algunas [excepciones sensibles a la seguridad](/docs/es/settings#exceptions-to-managed-settings-precedence) donde un valor más restrictivo de un nivel inferior aún cuenta.

Esta página es para el administrador que implementa la configuración administrada o depura por qué una no se está aplicando. Para decidir qué aplicar, comience con la tabla [Decidir qué aplicar](/docs/es/admin-setup#decide-what-to-enforce). Para la ruta de la consola claude.ai, consulte [Configuración administrada por servidor](/docs/es/server-managed-settings). Para saber en qué archivo van los valores propios de un desarrollador, consulte [Configuración](/docs/es/settings).

<h2 id="deploy-a-managed-settings-file">
  Implementar un archivo de configuración administrada
</h2>

Esta es la forma más rápida de poner una política en cada máquina: un archivo `managed-settings.json`. Si aún no ha elegido cómo entregar la configuración administrada, o sus dispositivos están bajo MDM o los desarrolladores ejecutan sesiones en la nube, lea primero [Elegir un mecanismo de entrega](#choose-a-delivery-mechanism).

<Steps>
  <Step title="Escribir managed-settings.json">
    Escriba un `managed-settings.json` que contenga las claves que ha decidido aplicar, en la misma forma JSON que `settings.json`. La tabla [Decidir qué aplicar](/docs/es/admin-setup#decide-what-to-enforce) enumera las claves detrás de cada control, y cada entrada en la [referencia de configuración](/docs/es/settings-reference) dice si una fuente administrada puede establecerla. Este archivo bloquea dos lecturas de archivo, desactiva el modo de omisión y hace que Claude Code ignore las reglas de permisos de archivos de usuario, proyecto y local y de `--allowedTools`:

    ```json managed-settings.json theme={null}
    {
      "permissions": {
        "deny": [
          "Read(./.env)",
          "Read(./secrets/**)"
        ],
        "disableBypassPermissionsMode": "disable"
      },
      "allowManagedPermissionRulesOnly": true
    }
    ```

    Para un ejemplo más completo que muestre la forma de más claves administradas, incluido el método de inicio de sesión, modelos, servidores MCP y mercados, consulte [Configuración administrada de una organización](/docs/es/settings-example#an-organizations-managed-settings).
  </Step>

  <Step title="Colocar el archivo en cada máquina">
    Guarde el archivo como `managed-settings.json` en el directorio del sistema para el sistema operativo, utilizando cualquier herramienta que ya coloque archivos en su flota:

    * **macOS**: `/Library/Application Support/ClaudeCode/managed-settings.json`
    * **Linux y WSL**: `/etc/claude-code/managed-settings.json`
    * **Windows**: `C:\Program Files\ClaudeCode\managed-settings.json`
  </Step>

  <Step title="Confirmar que la política se aplicó">
    En una máquina, ejecute `/status` dentro de Claude Code. La línea `Setting sources` muestra `Enterprise managed settings (file)`. Implemente en el resto de la flota después de eso; [Verificar que una política está en vigor](#check-that-a-policy-is-in-force) cubre qué mirar cuando falta la línea.
  </Step>
</Steps>

<span id="managed-settings-delivery" />

<span id="delivery-mechanisms" />

<h2 id="choose-a-delivery-mechanism">
  Elegir un mecanismo de entrega
</h2>

El archivo en los pasos anteriores es una de cuatro formas de obtener la configuración administrada en una máquina. Cada mecanismo lleva las mismas claves de política que un archivo `settings.json`, por lo que la [referencia de configuración](/docs/es/settings-reference) se aplica a todos ellos. Algunas claves están vinculadas a fuentes particulares, y la línea Scope de cada entrada dice cuál:

* **Controles de entrega**: [`policyHelper`](/docs/es/settings-reference#policyhelper), [`wslInheritsWindowsSettings`](/docs/es/settings-reference#wslinheritswindowssettings) y [`managedSourcesBehavior`](/docs/es/settings-reference#managedsourcesbehavior)
* **Claves de inicio de sesión de puerta de enlace**: [`forceLoginGatewayUrl`](/docs/es/settings-reference#forcelogingatewayurl), [`gatewayInternalNetworks`](/docs/es/settings-reference#gatewayinternalnetworks) y el valor `"gateway"` de [`forceLoginMethod`](/docs/es/settings-reference#forceloginmethod)

Un archivo de configuración administrada, un perfil MDM o la consola claude.ai aplica una política a todos los que alcanza. Para dar a un grupo de desarrolladores una política diferente, implemente un archivo o perfil diferente en ese grupo; la consola claude.ai [aún no puede dirigirse a un grupo](/docs/es/server-managed-settings#current-limitations), mientras que una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) autohospedada entrega la configuración administrada por grupo de IdP.

Cuando más de un mecanismo entrega una política a la misma máquina, Claude Code por defecto usa uno e ignora los otros. [Cómo Claude Code combina fuentes administradas](#how-claude-code-combines-managed-sources) da el orden y la opción de participación que aplica cada fuente.

Las filas MDM y archivo se llaman juntas configuración administrada en el punto final, porque la política se almacena en el dispositivo del desarrollador, a diferencia de la fila administrada por servidor, donde Claude Code la obtiene.

Elija un mecanismo según cómo ya administre dispositivos, usando la tabla a continuación.

| Mecanismo                                                              | Cómo lo entrega                                                                                                                                                                                                                                    | Cuándo Claude Code lo lee                                                                                                                                                                                                                                                                      | Úselo cuando                                                                                    |
| :--------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------- |
| [Configuración administrada por servidor](/docs/es/server-managed-settings) | En la consola de administración de claude.ai, o en una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) autohospedada                                                                                                            | Se obtiene al inicio y se sondea cada hora; consulte [cambios que necesitan aprobación](#where-and-when-a-policy-applies)                                                                                                                                                                      | Desea un lugar para cambiar la política de una organización de claude.ai sin tocar cada máquina |
| Política MDM o a nivel de SO                                           | Como un perfil de configuración de macOS o un valor de registro `HKLM` de Windows, a través de Jamf, Intune, Group Policy o una herramienta similar; consulte [dónde cada mecanismo almacena la política](#where-each-mechanism-stores-the-policy) | Se lee al inicio y se verifica si hay cambios cada 30 minutos                                                                                                                                                                                                                                  | Ya administra dispositivos con MDM o Group Policy                                               |
| Basado en archivo                                                      | Como `managed-settings.json` en un directorio del sistema en cada máquina; consulte [dónde cada mecanismo almacena la política](#where-each-mechanism-stores-the-policy)                                                                           | Se lee al inicio y se recarga cuando cambia un archivo                                                                                                                                                                                                                                         | Máquinas sin MDM, hosts Linux o imágenes que construye usted mismo                              |
| Registro HKCU, Windows y WSL                                           | Como un valor de registro `HKCU` de Windows; consulte [dónde cada mecanismo almacena la política](#where-each-mechanism-stores-the-policy)                                                                                                         | Se lee al inicio y se verifica si hay cambios cada 30 minutos; Claude Code lo usa solo cuando ninguna otra fuente administrada entrega una clave de política y ninguna [configuración principal suministrada por el host](#let-an-embedding-host-add-policy) proporciona una clave restrictiva | No puede escribir la clave `HKLM` a nivel de máquina                                            |

Las plantillas de inicio para Jamf, Iru, Intune y Group Policy están en el [repositorio de ejemplos de MDM](https://github.com/anthropics/claude-code/tree/main/examples/mdm).

Para servidores MCP administrados, que implementa junto con cualquiera de estos a través de `managed-mcp.json` o proporciona a través de la clave [`managedMcpServers`](/docs/es/settings-reference#managedmcpservers), consulte [Configuración MCP administrada](/docs/es/managed-mcp).

<h3 id="where-and-when-a-policy-applies">
  Dónde y cuándo se aplica una política
</h3>

Una política implementada llega a las sesiones del desarrollador de la siguiente manera:

* **Superficies**: en la máquina del desarrollador, la terminal, las extensiones de VS Code y JetBrains, la pestaña Code de la aplicación de escritorio y las sesiones de [Agent SDK](/docs/es/agent-sdk/typescript) leen todas estas fuentes. Las sesiones de Agent SDK cargan la configuración administrada incluso cuando `settingSources` excluye los archivos de usuario, proyecto y local.
* **Sesiones en la nube**: una sesión en un entorno alojado por Anthropic no lee un perfil MDM o archivo de dispositivo, por lo que la política para ella debe provenir de la configuración administrada por servidor. Una sesión en un [entorno autohospedado](/docs/es/self-hosted-environments) también lee el archivo de configuración administrada en su imagen de ejecutor, por defecto solo cuando la configuración administrada por servidor no entrega una clave de política, aparte de las [claves que Claude Code lee de cada fuente de administrador](#keys-read-from-every-admin-source). [Cómo Claude Code combina fuentes administradas](#how-claude-code-combines-managed-sources) cubre la opción de participación que aplica ambas.
* **Sesiones de Cowork**: [Cowork](https://claude.com/docs/cowork/overview) en la aplicación Claude Desktop ejecuta sus sesiones en Claude Code. En una sesión de Cowork, Claude Code nunca obtiene la configuración administrada por servidor de la consola de administración de claude.ai, incluso cuando el usuario inicia sesión con una cuenta de Team o Enterprise, por lo que la política que se aplica depende de dónde se ejecute la sesión:

  * **En la máquina del usuario**: por defecto, Claude Code en una sesión de Cowork lee la política MDM o a nivel de SO y el archivo de configuración administrada en ese dispositivo, así que implemente la política allí.
  * **En un sandbox de VM completa**: cuando su configuración administrada de Claude Desktop establece [`requireCoworkFullVmSandbox`](https://claude.com/docs/third-party/claude-desktop/configuration#requirecoworkfullvmsandbox), Claude Code se ejecuta dentro de una máquina virtual donde la política MDM del dispositivo y el archivo de configuración administrada no están presentes.
  * **Sesiones remotas de Cowork**: se ejecutan en máquinas virtuales administradas por Anthropic, donde Claude Code no tiene política de dispositivo para leer.

  Dondequiera que se ejecute la sesión, claude.ai aplica las listas [`strictKnownMarketplaces`](/docs/es/settings-reference#strictknownmarketplaces) y [`blockedMarketplaces`](/docs/es/settings-reference#blockedmarketplaces) de la consola de administración cuando alguien agrega un marketplace desde un repositorio de git en claude.ai o desde **Personalizar** en la pestaña Cowork. [Cómo funcionan las restricciones](/docs/es/plugins/org#restrict-what-users-can-install) describe esa verificación. La tabla [cobertura de superficie](/docs/es/model-config#surface-coverage) compara Cowork con las otras superficies.
* **Sesiones en ejecución**: la mayoría de los cambios llegan a una sesión en ejecución en el cronograma de la [tabla de mecanismo de entrega](#choose-a-delivery-mechanism), sin un reinicio.
  * Los cambios a [`forceRemoteSettingsRefresh`](/docs/es/settings-reference#forceremotesettingsrefresh), [`requiredMinimumVersion`](/docs/es/settings-reference#requiredminimumversion) y [algunas claves editables por el usuario](/docs/es/settings#when-edits-take-effect) surten efecto en el próximo inicio de sesión.
  * Una entrada [`policyHelper`](/docs/es/settings-reference#policyhelper) nueva o cambiada surte efecto en el próximo lanzamiento. Si la configuración administrada por servidor sombrea el asistente en ese lanzamiento, el asistente se ejecuta tan pronto como una obtención informa que esa configuración se ha eliminado.
* **Cambios que necesitan aprobación**: aparte de las [actualizaciones que esperan el próximo lanzamiento](/docs/es/server-managed-settings#fetch-and-caching-behavior), un cambio administrado por servidor en una configuración que [necesita aprobación](/docs/es/server-managed-settings#security-approval-dialogs), como un hook o una variable `env`, espera a que el desarrollador acepte el diálogo en una sesión interactiva, y se aplica para la ejecución actual en una sesión que una extensión IDE o el Agent SDK aloja. Otros cambios administrados por servidor se aplican en el próximo sondeo.
* **Sesiones de larga duración**: una sesión dejada abierta durante semanas aún puede retrasarse en un despliegue. [`requiredMinimumVersion`](/docs/es/settings-reference#requiredminimumversion) bloquea un binario desactualizado de iniciarse y no termina una sesión que ya se está ejecutando.

<span id="format-the-policy-for-each-platform" />

<h3 id="where-each-mechanism-stores-the-policy">
  Dónde cada mecanismo almacena la política
</h3>

Las claves son las mismas en todas partes, pero cada mecanismo las almacena en un lugar y forma diferentes:

* **Administrada por servidor**: los servidores de Anthropic, o su puerta de enlace, mantienen la política. Claude Code mantiene una caché local que aplica al inicio y [reemplaza en cada obtención exitosa](/docs/es/server-managed-settings#security-considerations).
* **Perfil de configuración de macOS**: el dominio de preferencias administradas `com.anthropic.claudecode`. Use las mismas claves de nivel superior que `managed-settings.json`, con configuraciones anidadas como diccionarios y listas como matrices plist.
* **Registro HKLM de Windows**: el JSON como un valor `REG_SZ` o `REG_EXPAND_SZ` llamado `Settings` bajo `HKLM\SOFTWARE\Policies\ClaudeCode`.
* **Basado en archivo**: `managed-settings.json`, un directorio opcional `managed-settings.d/` y `managed-mcp.json` en el directorio del sistema: `/Library/Application Support/ClaudeCode/` en macOS, `/etc/claude-code/` en Linux y WSL, y `C:\Program Files\ClaudeCode\` en Windows. Claude Code no lee la ruta heredada de Windows `C:\ProgramData\ClaudeCode\managed-settings.json`.
* **Registro HKCU de Windows**: el mismo valor `Settings` bajo `HKCU\SOFTWARE\Policies\ClaudeCode`.

<h3 id="split-a-file-based-policy-across-teams">
  Dividir una política basada en archivo entre equipos
</h3>

Si varios equipos poseen partes de una política, coloque cada parte en su propio archivo en `managed-settings.d/`, junto a `managed-settings.json` en el mismo directorio del sistema, en lugar de editar un archivo compartido.

Claude Code fusiona `managed-settings.json` primero, luego cada archivo `*.json` en el directorio en orden alfabético. Nombre los archivos con prefijos numéricos para controlar el orden, como `10-telemetry.json` y `20-security.json`. Claude Code ignora archivos ocultos y archivos que no terminan en `.json`.

Cuando dos archivos establecen la misma clave, Claude Code los combina por estas reglas:

* **Valores únicos**, como `"model": "opus"` o `"cleanupPeriodDays": 7`: el valor del archivo posterior reemplaza al anterior
* **Listas**, como `permissions.deny` o `sandbox.network.allowedDomains`: las dos listas se combinan, con duplicados eliminados
* **Bloques anidados**, como `env` o `sandbox`: los dos bloques se fusionan clave por clave, y cada clave dentro sigue estas mismas reglas
* **`fallbackModel`**: la cadena posterior reemplaza la anterior completamente
* **[`extraKnownMarketplaces`](/docs/es/settings-reference#extraknownmarketplaces) y [`managedMcpServers`](/docs/es/settings-reference#managedmcpservers)**: una entrada posterior con el mismo nombre reemplaza la anterior completamente
* **[`modelPicker`](/docs/es/settings-reference#modelpicker)**: la alineación posterior reemplaza la anterior completamente

<span id="precedence-within-the-managed-tier" />

<span id="which-managed-source-claude-code-uses" />

<h2 id="how-claude-code-combines-managed-sources">
  Cómo Claude Code combina fuentes administradas
</h2>

Cuando su organización entrega más de una fuente administrada a la misma máquina, la clave [`managedSourcesBehavior`](/docs/es/settings-reference#managedsourcesbehavior) decide qué hace Claude Code con las demás:

* **`"first-wins"`, el valor predeterminado**: Claude Code utiliza la fuente de mayor rango que entrega al menos una clave de política e ignora el resto en lugar de fusionarlas, aparte de las claves en [Claves leídas de cada fuente de administrador](#keys-read-from-every-admin-source). Claude Code no muestra ninguna advertencia para las fuentes que omite; `/status` [nombra la fuente que utilizó y las que omitió](#read-the-source-in-/status).
* **`"merge"`**: Claude Code aplica cada fuente de administrador que entrega una clave de política y las combina por tipo de clave: en la mayoría de las claves se aplica el valor de la fuente de mayor rango, las listas se unen y los bloqueos toman el valor más restrictivo. [Componer cada fuente administrada](#compose-every-managed-source) dice dónde establecer la clave y cómo se combina cada tipo de clave. Requiere Claude Code v2.1.242 o posterior.

Ambas configuraciones clasifican las fuentes de la misma manera. Estos términos se repiten en esta sección:

* **Clave de política**: cualquier clave de configuración que no sea las dos claves de control, [`wslInheritsWindowsSettings`](/docs/es/settings-reference#wslinheritswindowssettings) y [`managedSourcesBehavior`](/docs/es/settings-reference#managedsourcesbehavior). Un archivo de configuración administrada o una política MDM que contenga solo esas no cuenta, y Claude Code pasa a la siguiente fuente.
* **Fuente de administrador**: una de las tres primeras fuentes a continuación. El registro HKCU es escribible por el usuario y no es una.

Claude Code comprueba las fuentes en este orden, con la máxima prioridad primero:

1. Configuración remota, entregada desde claude.ai como [configuración administrada por servidor](/docs/es/server-managed-settings) o por una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway). Claude Code obtiene esta fuente solo cuando la sesión se autentica en la API de Anthropic directamente con un [inicio de sesión o clave elegible](/docs/es/server-managed-settings#platform-availability), o inicia sesión en una puerta de enlace con `/login`. En otros proveedores, o cuando `ANTHROPIC_BASE_URL` apunta a algún lugar que no sea la API de Anthropic, comienza en la siguiente fuente
2. Políticas de MDM o a nivel del SO: la clave de registro plist de macOS o HKLM
3. Archivos de configuración administrada, `managed-settings.d/*.json` y `managed-settings.json` fusionados juntos
4. El registro HKCU, en Windows, y en WSL una vez que el registro HKLM o el archivo de configuración administrada de Windows activa [`wslInheritsWindowsSettings`](/docs/es/settings-reference#wslinheritswindowssettings) y el valor HKCU también lo establece. Claude Code lo lee solo cuando ninguna fuente anterior entrega una clave de política y ninguna [configuración principal suministrada por el host](#let-an-embedding-host-add-policy) suministra una clave restrictiva

Este diagrama muestra la clasificación, con ejemplos de las claves entre fuentes que Claude Code lee de las tres primeras fuentes bajo cualquier configuración:

<img src="https://mintcdn.com/claude-code/zuWID2B-Rxm8DEC8/images/managed-source-precedence.svg?fit=max&auto=format&n=zuWID2B-Rxm8DEC8&q=85&s=53f6be49f06eff48e01422c8ae1bc2e6" className="dark:hidden" alt="Diagrama que muestra las cuatro fuentes de configuración administrada clasificadas desde la configuración remota en la parte superior a través de MDM, archivos de configuración administrada y el registro HKCU en la parte inferior. De forma predeterminada, la primera fuente con una clave de política suministra la política y el resto se omiten; con managedSourcesBehavior establecido en merge, cada fuente de administrador con una clave de política contribuye, combinada por tipo de clave, y el registro HKCU se mantiene fuera. Un panel lateral muestra que las claves entre fuentes como los bloqueos de sandbox, forceRemoteSettingsRefresh y la fusión env por variable se leen de cada fuente de administrador, que excluye el registro HKCU." width="680" height="330" data-path="images/managed-source-precedence.svg" />

<img src="https://mintcdn.com/claude-code/zuWID2B-Rxm8DEC8/images/managed-source-precedence-dark.svg?fit=max&auto=format&n=zuWID2B-Rxm8DEC8&q=85&s=ae407a9a08a3d680e80cf1a2af845d71" className="hidden dark:block" alt="Diagrama que muestra las cuatro fuentes de configuración administrada clasificadas desde la configuración remota en la parte superior a través de MDM, archivos de configuración administrada y el registro HKCU en la parte inferior. De forma predeterminada, la primera fuente con una clave de política suministra la política y el resto se omiten; con managedSourcesBehavior establecido en merge, cada fuente de administrador con una clave de política contribuye, combinada por tipo de clave, y el registro HKCU se mantiene fuera. Un panel lateral muestra que las claves entre fuentes como los bloqueos de sandbox, forceRemoteSettingsRefresh y la fusión env por variable se leen de cada fuente de administrador, que excluye el registro HKCU." width="680" height="330" data-path="images/managed-source-precedence-dark.svg" />

<h3 id="keys-read-from-every-admin-source">
  Claves leídas de cada fuente de administrador
</h3>

Bajo la configuración predeterminada `"first-wins"`, Claude Code lee la mayoría de las claves solo de la [fuente que seleccionó](#how-claude-code-combines-managed-sources), e ignora un valor en una fuente de menor rango incluso cuando la fuente seleccionada deja esa clave sin establecer.

Algunas claves funcionan de manera diferente. Claude Code las lee de cada fuente de administrador, por lo que una política MDM de menor rango o un archivo de configuración administrada aún pueden establecerlas cuando la fuente seleccionada no lo hace. Claude Code deja fuera el registro HKCU escribible por el usuario de ese escaneo; cuando HKCU es la única fuente y ningún host suministra configuración principal, HKCU se aplica como cualquier fuente seleccionada.

Las claves entre fuentes incluyen:

* `sandbox.network.allowManagedDomainsOnly` y `sandbox.filesystem.allowManagedReadPathsOnly`: un `true` en cualquier fuente de administrador activa el bloqueo. Mientras un bloqueo está activo, Claude Code une la lista de permitidos que bloquea, `sandbox.network.allowedDomains` junto con reglas de permiso `WebFetch(domain:...)`, o `sandbox.filesystem.allowRead`, en cada fuente de administrador. Sin el bloqueo, Claude Code trata la lista de permitidos como cualquier otra clave, por lo que bajo `"first-wins"` la lista de permitidos de una fuente de administrador no seleccionada se ignora
* `allowAllClaudeAiMcps`
* `allowManagedMcpServersOnly`: un `true` en cualquier fuente de administrador activa el bloqueo de lista de permitidos de MCP. Mientras el bloqueo está activo, la lista `allowedMcpServers` administrada proviene de la fuente de administrador de mayor rango que establece una. Una lista administrada por servidor reemplaza la lista de una fuente inferior en lugar de combinarse con ella.

  Si ninguna fuente de administrador establece una lista, cada servidor que pase la lista de denegación se carga, a menos que la [configuración principal](#let-an-embedding-host-add-policy) suministre una lista.

  Sin el bloqueo, Claude Code lee `allowedMcpServers` de la fuente administrada que aplica, por lo que bajo `"first-wins"` la lista de una fuente de administrador no seleccionada se ignora. Requiere Claude Code v2.1.273 o posterior
* `deniedMcpServers` y [`disableClaudeAiConnectors`](/docs/es/settings-reference#disableclaudeaiconnectors): una entrada o un `true` en cualquier fuente de administrador se aplica. Requiere Claude Code v2.1.273 o posterior
* Las rutas binarias de sandbox `sandbox.bwrapPath` y `sandbox.socatPath`
* El binario `ripgrep` de sandbox, [`sandbox.ripgrep`](/docs/es/settings-reference#sandbox-ripgrep)
* `sandbox.filesystem.disabled` y `sandbox.network.strictAllowlist`
* [`useAutoModeDuringPlan`](/docs/es/settings-reference#useautomodeduringplan), [`syncClaudeAiSkills`](/docs/es/settings-reference#syncclaudeaiskills) y [`syncClaudeAiPlugins`](/docs/es/settings-reference#syncclaudeaiplugins), donde un `false` de cualquier fuente de administrador desactiva el comportamiento. Un `false` en la configuración de usuario o local del desarrollador también lo desactiva; cada clave solo puede negar
* [`enableArtifact`](/docs/es/settings-reference#enableartifact), donde un `false` de cualquier fuente de administrador desactiva la [herramienta Artifact](/docs/es/artifacts). Un `false` en la configuración de usuario, proyecto o local del desarrollador también lo desactiva, y ninguna fuente lo vuelve a activar; vea [qué valores de nivel inferior aún cuentan](/docs/es/settings#exceptions-to-managed-settings-precedence). Requiere Claude Code v2.1.242 o posterior
* [`maxEffortLevel`](/docs/es/settings-reference#maxeffortlevel), donde se aplica el límite más bajo en cualquier fuente de administrador. Si un desarrollador establece un límite más bajo en su propia configuración o con `--settings`, Claude Code aplica ese; ninguna fuente puede aumentar el límite. Requiere Claude Code v2.1.267 o posterior
* Una exclusión de remolque de confirmación en `attribution`, o en el `includeCoAuthoredBy` deprecado, de cualquier nivel
* [`forceRemoteSettingsRefresh`](/docs/es/server-managed-settings)
* `env`, fusionado por variable en las fuentes de administrador: cada variable proviene de la fuente de mayor prioridad que la define, por lo que las fuentes inferiores rellenan las variables que las superiores dejan sin establecer. Algunas variables siguen sus propias reglas; [Excepciones por clave en fuentes administradas](/docs/es/server-managed-settings#per-key-exceptions-across-managed-sources) nombra cada una. Requiere Claude Code v2.1.223 o posterior. Antes de v2.1.223, Claude Code aplicaba solo el bloque `env` completo de la fuente seleccionada

Las [claves de inicio de sesión de puerta de enlace](#choose-a-delivery-mechanism) siguen una regla separada. Claude Code nunca las lee de la configuración administrada por servidor, por lo que mientras la configuración administrada por servidor es la fuente seleccionada, la fuente de administrador de mayor rango en la máquina que lleva una clave de política aún las suministra. Se ignora un valor en una fuente de administrador clasificada por debajo de esa, o en el registro HKCU.

Cuando una fuente de administrador establece `allowManagedMcpServersOnly` o una lista `allowedMcpServers` y ese valor no es el que está en vigor, `/status` y `claude doctor` nombran esa fuente y clave.

<h3 id="compose-every-managed-source">
  Componer cada fuente administrada
</h3>

Para que Claude Code aplique cada fuente de administrador que su organización entrega, establezca [`managedSourcesBehavior`](/docs/es/settings-reference#managedsourcesbehavior) en `"merge"` en la fuente de mayor rango que implemente. Claude Code lee la clave solo de la fuente de mayor rango que lleva la clave o una clave de política, por lo que una fuente inferior no puede optar por fusionarse con la fuente anterior, y una máquina que nunca recibe configuración administrada por servidor necesita la clave en su perfil MDM también. El registro HKCU escribible por el usuario nunca se fusiona con otra fuente. Requiere Claude Code v2.1.242 o posterior.

Bajo `"merge"`, Claude Code agrega las entradas de lista de una fuente inferior, como reglas `permissions.allow` y hooks, a la política, por lo que actívelo solo cuando cada fuente clasificada por debajo de la más alta esté bajo el control de un administrador.

Esta tabla muestra cómo Claude Code combina cada tipo de clave bajo `"merge"`. La [entrada `managedSourcesBehavior`](/docs/es/settings-reference#managedsourcesbehavior) nombra cada clave en tres de las filas: listas de permitidos de restricción, valores tomados completos y claves leídas solo de la fuente de mayor rango.

| Tipo de clave                                  | Cómo Claude Code la combina                                                                                                                                                         | Ejemplos                                                                                                                                    |
| :--------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| Listas                                         | Combina las entradas de cada fuente                                                                                                                                                 | `permissions.allow`, `hooks`, `sandbox.network.allowedDomains`, `deniedMcpServers`                                                          |
| Bloqueos                                       | Aplica el valor más restrictivo que establece cualquier fuente; un valor más flexible se aplica solo de la fuente de mayor rango                                                    | `allowManagedHooksOnly`, `permissions.disableBypassPermissionsMode`, `crossSessionInbound`                                                  |
| Listas de permitidos de restricción            | Toma la lista completa de la fuente de mayor rango que la establece, sin agregar entradas de fuentes inferiores                                                                     | `availableModels`, `allowedMcpServers`, `strictKnownMarketplaces`, `allowedChannelPlugins` y la cadena `fallbackModel`                      |
| Valores tomados completos                      | Toma el valor completo de la fuente de mayor rango que lo establece, sin combinar entradas o campos de fuentes inferiores                                                           | `sandbox.credentials.awsPairs`, `sandbox.ripgrep`                                                                                           |
| Servidores MCP proporcionados                  | Combina los nombres de servidor de cada fuente; cuando dos fuentes establecen el mismo nombre, aplica la entrada completa de la fuente de mayor rango                               | `managedMcpServers`                                                                                                                         |
| Claves leídas solo de la fuente de mayor rango | Ignora la clave en cada fuente inferior, incluso cuando la fuente de mayor rango la deja sin establecer                                                                             | Ayudantes de credenciales como `apiKeyHelper`, pines de inicio de sesión como `forceLoginOrgUUID`, `modelPicker`, `permissions.defaultMode` |
| `env`                                          | Se fusiona por variable en fuentes de administrador bajo cualquier configuración, como [Claves leídas de cada fuente de administrador](#keys-read-from-every-admin-source) describe |                                                                                                                                             |
| Cualquier otra clave                           | Toma el valor de la fuente de mayor rango que lo establece                                                                                                                          | `model`, `cleanupPeriodDays`                                                                                                                |

Para confirmar qué fuentes se combinaron en una máquina, [lea la línea `Setting sources` en `/status`](#read-the-source-in-/status); esa sección dice qué significa cada etiqueta.

<h3 id="compute-the-policy-with-a-helper-program">
  Calcular la política con un programa auxiliar
</h3>

Un [`policyHelper`](/docs/es/settings-reference#policyhelper) es un ejecutable que su política MDM o archivo de configuración administrada nombra, y Claude Code lo ejecuta para calcular la configuración administrada al inicio. Cuando la fuente seleccionada configura uno y el auxiliar emite un objeto `managedSettings`, esa salida cambia lo que Claude Code lee:

* **El objeto `managedSettings` emitido es la única configuración administrada para la sesión**, incluyendo para las [claves que de otro modo lee de cada fuente de administrador](#keys-read-from-every-admin-source), aparte de [`forceRemoteSettingsRefresh`, que tiene su propia regla de inicio](/docs/es/settings-reference#forceremotesettingsrefresh)

Para qué fallos de auxiliar se ejecutan y qué hace Claude Code cuando uno lo hace, vea [Fallos de auxiliar](/docs/es/settings-reference#helper-failures).

<span id="parent-settings-from-embedding-hosts" />

<span id="control-policy-from-an-embedding-host" />

<span id="merge-policy-from-an-embedding-host" />

<h3 id="let-an-embedding-host-add-policy">
  Permitir que un host de incrustación agregue política
</h3>

Cuando otra aplicación inicia Claude Code, como Claude Desktop, una extensión IDE o una aplicación Agent SDK, ese host puede pasar su propia configuración administrada a través de la opción SDK `managedSettings`. Claude Code llama a estas configuraciones principales.

De forma predeterminada, Claude Code ignora la configuración principal siempre que una fuente de administrador esté presente: configuración administrada por servidor, una política MDM o a nivel del SO, o un archivo de configuración administrada.

Para que Claude Code fusione la configuración principal junto con una fuente de administrador, establezca [`parentSettingsBehavior`](/docs/es/settings-reference#parentsettingsbehavior) en `"merge"` en la fuente administrada de mayor prioridad; Claude Code lee la clave solo de esa fuente.

Claude Code luego mantiene solo los valores del host que restringen lo que Claude puede hacer, con una brecha a tener en cuenta: a menos que también establezca los bloqueos `allowManaged*Only`, las reglas de permiso de permitidos del host y las listas de permitidos de sandbox aún se aplican. Vea [Restringir configuración principal](/docs/es/claude-apps-gateway#restrict-parent-settings) para los bloqueos.

Un [`policyHelper`](/docs/es/settings-reference#policyhelper) puede desactivar la fusión principal independientemente de esta clave; su entrada dice cuándo.

Claude Code también aplica estas comprobaciones a valores suministrados por el principal por su cuenta:

* Cuando cualquier fuente de administrador establece `allowManagedPermissionRulesOnly`, Claude Code descarta [reglas de permiso de permitidos suministradas por el principal](/docs/es/claude-apps-gateway#restrict-parent-settings) y `additionalDirectories` mientras las lee, incluso cuando una fuente de mayor prioridad deja la clave sin establecer. El efecto de la clave en sus propias reglas de permiso proviene de la configuración administrada que Claude Code aplica, o de la configuración principal que ha elegido fusionar
* Claude Code aplica el valor `forceLoginOrgUUID` o `allowedMcpServers` en la configuración administrada que aplica y bloquea uno suministrado por el principal. Fuera del bloqueo de lista de permitidos de MCP, un valor en una fuente de administrador inferior que Claude Code no aplica ni se aplica ni bloquea el del principal.

  En Claude Code v2.1.273 o posterior, mientras `allowManagedMcpServersOnly` está activo, la lista `allowedMcpServers` de la fuente de administrador de mayor rango que establece una se aplica y bloquea la del principal, como una [clave entre fuentes](#keys-read-from-every-admin-source). La lista del principal se aplica solo cuando ninguna fuente de administrador establece una. La entrada [`managedSourcesBehavior`](/docs/es/settings-reference#managedsourcesbehavior) dice qué fuente suministra cada clave bajo `"merge"`. Antes de v2.1.223, un valor en cualquier fuente de administrador bloqueaba el del principal
* Para `availableModels`, Claude Code aplica el valor en la configuración administrada que aplica y bloquea una lista suministrada por el principal
* Para `strictKnownMarketplaces`, Claude Code asimismo aplica la lista en la configuración administrada que aplica y bloquea una suministrada por el principal. La lista del principal se aplica solo cuando ninguna fuente administrada aplicada establece una. Requiere Claude Code v2.1.282 o posterior
* Un `blockedMarketplaces` suministrado por el principal se aplica además de cualquier lista de denegación que una fuente administrada establece. Requiere Claude Code v2.1.282 o posterior

<h4 id="keep-cowork-folder-access-when-only-managed-rules-apply">
  Mantener el acceso a la carpeta Cowork cuando solo se aplican reglas administradas
</h4>

[Cowork](https://claude.com/docs/cowork/overview) en la aplicación Claude Desktop ejecuta sus sesiones en Claude Code y otorga a cada sesión acceso a sus carpetas de trabajo, como la carpeta que el usuario conecta, a través de reglas de permitidos que suministra cuando inicia la sesión. Cuando su política administrada establece [`allowManagedPermissionRulesOnly`](/docs/es/settings-reference#allowmanagedpermissionrulesonly), Claude Code mantiene solo las reglas de permitidos en la política administrada: descarta reglas de permitidos que un host suministra como configuración principal, como `--allowedTools`, o en un archivo de configuración, por lo que las escrituras en esas carpetas pierden su preaprobación. En una sesión de Cowork que pregunta antes de ediciones, Cowork no puede mostrar el aviso, y Claude reporta cada escritura como bloqueada porque la ruta se resuelve en una ubicación protegida o una ruta fuera de la carpeta conectada.

Para restaurar las escrituras, agregue reglas de permitidos para esas carpetas a la fuente administrada que Claude Code [selecciona](#precedence-within-the-managed-tier) en esas máquinas: en una flota administrada por MDM, esa es la política MDM en lugar de un archivo de configuración administrada separado. Este ejemplo usa la forma de archivo, y una política MDM toma las mismas claves. Mantiene `allowManagedPermissionRulesOnly` establecido y permite ediciones bajo una carpeta `CoworkProjects` en el directorio de inicio de cada usuario; reemplace la ruta con las carpetas que sus usuarios conectan:

```json managed-settings.json theme={null}
{
  "allowManagedPermissionRulesOnly": true,
  "permissions": {
    "allow": [
      "Edit(~/CoworkProjects/**)"
    ]
  }
}
```

Después de implementar la política, Claude puede guardar archivos bajo esa carpeta en una nueva sesión de Cowork. [Reglas de lectura y edición](/docs/es/permissions#read-and-edit) cubren la sintaxis de ruta, incluyendo la forma `//` para rutas absolutas.

<h3 id="what-a-developer-can-change">
  Lo que un desarrollador puede cambiar
</h3>

Los archivos de configuración propios de un desarrollador, valores `--settings` y archivos de proyecto nunca anulan un valor administrado; las [excepciones](/docs/es/settings#exceptions-to-managed-settings-precedence) solo permiten que un valor de nivel inferior más restrictivo cuente. Estos casos se encuentran fuera de esa regla:

* **El modelo para una sesión**: un `model` administrado es un valor predeterminado, no un bloqueo. `--model` y `ANTHROPIC_MODEL` aún seleccionan el modelo para esa sesión, por lo que implemente [`availableModels`](/docs/es/settings-reference#availablemodels) para restringir la opción.
* **Derechos de administrador local**: un desarrollador que es administrador en la máquina puede editar la fuente administrada en sí, por lo que las herramientas MDM pueden reimplementar el perfil o archivo en un cronograma y por qué existen el registro HKLM y el dominio de preferencias administradas de macOS.
* **La caché de configuración administrada por servidor**: la configuración administrada por servidor proviene de los servidores de Anthropic, y una edición en la caché local [dura solo hasta la siguiente obtención exitosa](/docs/es/server-managed-settings#security-considerations).
* **Otras herramientas**: la configuración administrada vincula solo Claude Code. Un desarrollador que llama a la API desde otra herramienta no está bajo ellas.

<span id="verify-enforcement" />

<span id="verify-that-a-policy-is-in-force" />

<h2 id="check-that-a-policy-is-in-force">
  Verificar que una política está en vigor
</h2>

Un desarrollador reporta que una política no se está aplicando, o desea confirmar que un despliegue llegó antes de empujarlo a la flota. Dos comandos en esa máquina lo responden: `/status` muestra qué fuente administrada seleccionó Claude Code, y `claude doctor` enumera lo que descartó.

<h3 id="read-the-source-in-/status">
  Leer la fuente en /status
</h3>

En la máquina del desarrollador, ejecute `/status` dentro de Claude Code y lea la línea `Setting sources`. Cuando una fuente administrada está en vigor, la línea enumera `Enterprise managed settings` con la fuente que Claude Code seleccionó entre paréntesis:

* `(remote)`: configuración administrada por servidor desde claude.ai o una puerta de enlace
* `(plist)` o `(HKLM)`: una política MDM u OS
* `(file)`, `(drop-ins)` o `(file + drop-ins)`: `managed-settings.json`, el directorio de complementos o ambos
* `(remote + file, merged)` u otra lista que termina en `, merged`: su organización [compone cada fuente administrada](#compose-every-managed-source), y Claude Code fusionó las fuentes enumeradas en la política. Una fuente inferior aún puede proporcionar variables `env` sin aparecer en la lista. Requiere Claude Code v2.1.242 o posterior
* `(HKCU)`: el registro de reserva que se puede escribir por el usuario
* `(parent process)`: un [host de incrustación](#let-an-embedding-host-add-policy) proporcionó configuración restrictiva
* `(helper)`: un [`policyHelper`](/docs/es/settings-reference#policyhelper) configurado por la fuente MDM o de archivo seleccionada

Cuando Claude Code encontró una fuente administrada en la máquina y no la seleccionó, una segunda línea, `Skipped sources`, nombra cada fuente de este tipo. Léala para distinguir una política que nunca llegó a la máquina de una que llegó y que una fuente de mayor prioridad anuló. Requiere Claude Code v2.1.242 o posterior.

Cuando la política no se está aplicando, la línea `Setting sources` le dice cuál de dos problemas tiene:

* **La línea falta**: Claude Code no encontró ninguna fuente administrada que entregue una clave de política.

  Si implementó un archivo de configuración administrada, verifique que se encuentre en la ruta del SO y que contenga una [clave de política](#how-claude-code-combines-managed-sources) en lugar de solo las claves de control. Un archivo que no es JSON válido no produce este estado; Claude Code [se niega a iniciar](#find-entries-claude-code-dropped) en su lugar.

  Cuando implementó a través de configuración administrada por servidor en su lugar, ejecute `claude doctor`, que reporta el [resultado de obtención](/docs/es/server-managed-settings#verify-settings-delivery).
* **La línea nombra una fuente que no es la que implementó**: una fuente de mayor prioridad está presente y Claude Code ignoró la suya, y `Skipped sources` la enumera. [Cómo Claude Code combina fuentes administradas](#how-claude-code-combines-managed-sources) da el orden.

<span id="invalid-entries-in-managed-settings" />

<h3 id="find-entries-claude-code-dropped">
  Encontrar entradas que Claude Code descartó
</h3>

Cuando un archivo de configuración administrada, perfil MDM, valor de registro o carga administrada por servidor falla la validación del esquema, Claude Code primero omite las entradas individuales que puede reparar, como una regla de permiso inválida, con una advertencia para cada una, luego descarta cualquier clave de nivel superior cuyo valor aún falla y continúa aplicando cada clave válida restante.

Claude Code es más estricto con el `managedSettings` que un [`policyHelper`](/docs/es/settings-reference#policyhelper) emite: realiza las mismas reparaciones de entrada, pero cualquier violación de esquema que sobreviva falla toda la ejecución del auxiliar, y al inicio Claude Code se niega a iniciar, lo mismo que para un auxiliar que sale con código distinto de cero.

Cuando un archivo de configuración administrada, archivo de complemento, plist MDM o valor de registro HKLM está presente pero no se puede analizar como un objeto JSON, Claude Code se niega a iniciar e imprime [un error que nombra la fuente](/docs/es/errors#managed-settings-document-could-not-be-parsed), incluso cuando otra fuente de administrador entrega una política válida. Cada fuente falla de esta manera cuando:

* **Archivo de configuración administrada o archivo de complemento**: el archivo no es JSON válido, o su nivel superior no es un objeto
* **Plist MDM**: `plutil` de macOS reporta la plist malformada, o su contenido convertido no es un objeto JSON
* **Valor de registro HKLM**: el valor `Settings` no es una cadena, está vacío o no contiene un objeto JSON

Tres estados de fuente no causan este rechazo:

* Un archivo, perfil o valor de registro ausente no es un fallo; Claude Code se ejecuta sin esa fuente.
* Un archivo de configuración administrada vacío cuenta como `{}`.
* Un valor malformado en la clave de registro HKCU que se puede escribir por el usuario nunca bloquea el lanzamiento. Claude Code lo reporta como un aviso en `/status` y `claude doctor` en su lugar.

Si un archivo de configuración administrada, archivo de complemento o directorio `managed-settings.d/` no se puede leer y ninguna fuente de administrador proporciona una política, las sesiones que inician sesión con credenciales de claude.ai o Claude Console salen al inicio con un mensaje para contactar a un administrador.

Para encontrar una entrada descartada, busque en uno de tres lugares:

* Las sesiones interactivas muestran un diálogo al inicio que enumera las entradas inválidas.
* Las ejecuciones no interactivas con `-p` imprimen un resumen a stderr.
* [`claude doctor`](/docs/es/debug-your-config) enumera cada entrada inválida con su fuente y campo.

<h4 id="keys-that-fail-closed">
  Claves que fallan cerradas
</h4>

Algunas claves de aplicación no se descartan cuando son inválidas. Claude Code aplica una reserva más restrictiva hasta que se corrija el valor; la tabla muestra qué aplica para cada clave:

| Campo                         | Comportamiento cuando está presente pero es inválido                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :---------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedMcpServers`           | Se aplica como una lista de permitidos vacía hasta que se corrija el valor, por lo que ningún servidor MCP que los usuarios agreguen se admite. Los servidores que su organización entrega a través de [`managedMcpServers`](/docs/es/settings-reference#managedmcpservers) aún se cargan, y los servidores `managed-mcp.json` se cargan según [Cómo se evalúa un servidor](/docs/es/managed-mcp#how-a-server-is-evaluated). Una entrada individual inválida se elimina y el subconjunto válido se aplica.                                        |
| `allowedHttpHookUrls`         | Claude Code aplica una [lista de permitidos](/docs/es/settings-reference#allowedhttphookurls) administrada vacía hasta que corrija el valor, por lo que un hook HTTP se ejecuta solo si otro archivo de configuración enumera su URL. Si solo una entrada individual es inválida, Claude Code elimina esa entrada y aplica el resto.                                                                                                                                                                                                         |
| `httpHookAllowedEnvVars`      | Claude Code aplica una [lista de permitidos](/docs/es/settings-reference#httphookallowedenvvars) administrada vacía hasta que corrija el valor, por lo que una variable de encabezado se interpola solo si otro archivo de configuración la nombra. Si solo una entrada individual es inválida, Claude Code elimina esa entrada y aplica el resto.                                                                                                                                                                                           |
| `allowedChannelPlugins`       | Claude Code aplica una lista de permitidos vacía hasta que corrija el valor, por lo que ningún plugin de canal pasado a `--channels` se admite. Si solo una entrada individual es inválida, elimina esa entrada y aplica el resto.                                                                                                                                                                                                                                                                                                      |
| `strictKnownMarketplaces`     | Se aplica como una lista de permitidos vacía hasta que se corrija el valor, por lo que ninguna [fuente de marketplace](/docs/es/plugins/org#restrict-what-users-can-install) se admite. Una entrada individual que es inválida o no se puede aplicar, como una expresión regular `hostPattern` que no se compila, se elimina y el subconjunto válido se aplica.                                                                                                                                                                              |
| `allowManagedHooksOnly`       | Se trata como `true` hasta que se corrija: se aplican las [restricciones de hook](/docs/es/settings-reference#allowmanagedhooksonly) y, a menos que `disableCommandPluginSources` sea explícitamente `false`, los plugins de origen de comando se desactivan.                                                                                                                                                                                                                                                                                |
| `allowManagedMcpServersOnly`  | Se trata como `true`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `disableCommandPluginSources` | Se trata como `true`, por lo que los plugins de origen de comando permanecen desactivados hasta que se corrija el valor.                                                                                                                                                                                                                                                                                                                                                                                                                |
| `disableSideloadFlags`        | Se trata como `true` hasta que se corrija el valor, con los efectos enumerados para [`disableSideloadFlags`](/docs/es/settings-reference#disablesideloadflags).                                                                                                                                                                                                                                                                                                                                                                              |
| `availableModels`             | Se aplica como una lista de permitidos vacía hasta que se corrija, por lo que solo el modelo predeterminado está disponible; una entrada que no es cadena se elimina y el subconjunto válido se aplica.                                                                                                                                                                                                                                                                                                                                 |
| `enforceAvailableModels`      | Se trata como `true`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `syncClaudeAiPlugins`         | Se trata como `false`, por lo que la sincronización de [plugins de claude.ai](/docs/es/settings-reference#syncclaudeaiplugins) está desactivada hasta que se corrija el valor.                                                                                                                                                                                                                                                                                                                                                               |
| `forceLoginOrgUUID`           | Ninguna organización puede iniciar sesión hasta que se corrija el valor.                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `gatewayInternalNetworks`     | Cuando el valor inválido proviene de la fuente administrada más alta en la máquina, `/login` rechaza cada nuevo inicio de sesión de [puerta de enlace en la nube](/docs/es/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) en esa máquina hasta que se corrija el valor.                                                                                                                                                                                                                                                |
| `crossSessionInbound`         | Se trata como `refuse`, el valor más restrictivo, por lo que los [mensajes entre sesiones](/docs/es/cross-session-messaging#control-inbound-messages) entrantes se rechazan hasta que se corrija el valor. El desarrollador ve [una advertencia](/docs/es/errors#crosssessioninbound-must-be-one-of-accept-hold-refuse).                                                                                                                                                                                                                          |
| `deniedMcpServers`            | Una entrada individual inválida se elimina y el subconjunto válido se aplica. Un valor completamente inválido se descarta con una advertencia, ya que negar cada servidor bloquearía servidores que la política nunca nombró.                                                                                                                                                                                                                                                                                                           |
| `blockedMarketplaces`         | Una entrada individual inválida se elimina y el subconjunto válido se aplica. Una entrada que se analiza pero nunca puede coincidir, como una expresión regular `hostPattern` que no se compila, se mantiene con una advertencia. No bloquea nada hasta que se corrija, pero las [restricciones de marketplace](/docs/es/plugins/org#restrict-what-users-can-install) permanecen activas. Un valor completamente inválido se descarta con una advertencia, ya que bloquear cada marketplace bloquearía fuentes que la política nunca nombró. |
| `sandbox.credentials`         | Una entrada inválida recuperable se degrada a `mode: "deny"` con una advertencia; una irrecuperable se elimina; las entradas válidas permanecen aplicadas. Consulte [entradas de credenciales inválidas](/docs/es/settings-reference#invalid-credential-entries-in-managed-settings)                                                                                                                                                                                                                                                         |

`allowedHttpHookUrls` y `httpHookAllowedEnvVars` se fusionan en archivos de configuración, por lo que las entradas en su configuración de usuario, proyecto o local aún se aplican mientras la lista administrada está vacía.

Las reservas para esas dos claves y para `allowedChannelPlugins` requieren Claude Code v2.1.267 o posterior; las versiones anteriores descartan la clave completa cuando su valor o cualquier entrada es inválida. Las reservas para `strictKnownMarketplaces`, `blockedMarketplaces` y `disableSideloadFlags` requieren Claude Code v2.1.277 o posterior; las versiones anteriores descartan la clave completa cuando su valor o cualquier entrada es inválida.

`requiredMinimumVersion` y `requiredMaximumVersion` fallan abiertos por diseño: un valor inválido se descarta en lugar de aplicarse.

Esta tolerancia se aplica solo a la configuración administrada. Los archivos de configuración de usuario, proyecto y local permanecen estrictos: un archivo cuyo JSON o forma de nivel superior falla la validación se rechaza completamente y se reporta, y una entrada individual que falla, como una regla de permiso malformada, se omite con una advertencia mientras el resto del archivo se aplica.

<span id="managed-only-settings" />

<h2 id="keys-only-a-managed-source-can-set">
  Claves que solo una fuente administrada puede establecer
</h2>

Claude Code lee las siguientes claves solo de una fuente administrada; colocarlas en archivos de configuración de usuario o proyecto no tiene efecto.

La mayoría de ellas son bloqueos: el valor que un bloqueo rige, como reglas de permisos o `sandbox.network.allowedDomains`, es una clave ordinaria que cualquier nivel puede establecer, y el bloqueo le dice a Claude Code que honre solo el valor administrado.

La tabla cubre los controles de permisos, plugins y entrega. Para cualquier clave no enumerada aquí, la columna Scope de la [referencia de configuración](/docs/es/settings-reference#all-settings) dice si es solo administrada; las claves solo administradas restantes allí incluyen la URL de inicio de sesión de puerta de enlace, versión, navegador, simulador móvil, host SSH, sesión local de Desktop, ruta binaria de sandbox, precios de modelo y controles CLAUDE.md.

| Configuración                                                                                                         | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| :-------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`allowAllClaudeAiMcps`](/docs/es/settings-reference#allowallclaudeaimcps)                                                 | Cargue los conectores de claude.ai que Claude Code obtiene a sí mismo junto con un `managed-mcp.json` implementado en lugar de suprimirlos                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| [`allowedChannelPlugins`](/docs/es/settings-reference#allowedchannelplugins)                                               | Lista de permitidos de plugins de canal que pueden enviar mensajes. Reemplaza la lista de permitidos predeterminada de Anthropic cuando se establece. Requiere `channelsEnabled: true`. Consulte [Restringir qué plugins de canal pueden ejecutarse](/docs/es/channels#restrict-which-channel-plugins-can-run)                                                                                                                                                                                                                                                                   |
| [`allowManagedHooksOnly`](/docs/es/settings-reference#allowmanagedhooksonly)                                               | Cuando es `true`, restringe qué hooks se ejecutan; consulte [qué se ejecuta bajo `allowManagedHooksOnly`](/docs/es/settings-reference#what-runs-under-allowmanagedhooksonly) para la lista de efectos completa                                                                                                                                                                                                                                                                                                                                                                   |
| [`allowManagedMcpServersOnly`](/docs/es/settings-reference#allowmanagedmcpserversonly)                                     | Cuando es `true`, solo se respetan `allowedMcpServers` de la configuración administrada. `deniedMcpServers` aún se fusiona de todas las fuentes. Consulte [Claves leídas de cada fuente de administrador](#keys-read-from-every-admin-source) para saber qué fuentes administradas pueden establecerla, y [Configuración MCP administrada](/docs/es/managed-mcp)                                                                                                                                                                                                                 |
| [`allowManagedPermissionRulesOnly`](/docs/es/settings-reference#allowmanagedpermissionrulesonly)                           | Hace que la configuración administrada sea la única fuente de configuración de reglas de permisos. La entrada enumera cada fuente que ignora                                                                                                                                                                                                                                                                                                                                                                                                                                |
| [`blockedMarketplaces`](/docs/es/settings-reference#blockedmarketplaces)                                                   | Lista de bloqueo de fuentes de mercado. Las fuentes bloqueadas se verifican antes de descargar, por lo que nunca tocan el sistema de archivos. Consulte [restricciones de mercado administradas](/docs/es/plugins/org#restrict-what-users-can-install)                                                                                                                                                                                                                                                                                                                           |
| [`channelsEnabled`](/docs/es/settings-reference#channelsenabled)                                                           | Permitir [canales](/docs/es/channels) para la organización. Consulte [controles empresariales](/docs/es/channels#enterprise-controls) para el predeterminado en cada plan                                                                                                                                                                                                                                                                                                                                                                                                             |
| [`disableCommandPluginSources`](/docs/es/settings-reference#disablecommandpluginsources)                                   | Cuando es `true`, bloquea completamente las [fuentes de plugins de `command`](/docs/es/plugins/marketplace-reference#command-plugin-source), por lo que el comando declarado por el mercado nunca se ejecuta. También bloquea los comandos [`headersHelper`](/docs/es/plugins/host-marketplace#authenticate-archive-downloads) del mercado, excepto para un mercado que la configuración administrada declara a sí misma. Cuando no se establece, sigue `allowManagedHooksOnly`. Requiere Claude Code v2.1.229 o posterior, y el bloque `headersHelper` requiere v2.1.238 o posterior |
| [`disableSideloadFlags`](/docs/es/settings-reference#disablesideloadflags)                                                 | Rechace los indicadores `--plugin-dir`, `--plugin-url`, `--agents` y `--mcp-config` al inicio. En sesiones en la nube, Claude Code descarta los servidores MCP que el servidor entregó a través de `--mcp-config`, que no sean entradas `type: "sdk"` en proceso, e inicia la sesión. Requiere Claude Code v2.1.193 o posterior                                                                                                                                                                                                                                             |
| [`forceRemoteSettingsRefresh`](/docs/es/settings-reference#forceremotesettingsrefresh)                                     | Cuando es `true`, bloquea el inicio de CLI hasta que la configuración administrada remota se obtenga recientemente y sale si la obtención falla. Consulte [aplicación de inicio cerrado](/docs/es/server-managed-settings#enforce-fail-closed-startup)                                                                                                                                                                                                                                                                                                                           |
| [`managedMcpServers`](/docs/es/settings-reference#managedmcpservers)                                                       | Servidores MCP remotos proporcionados a cada usuario junto con los suyos propios. Proporciona servidores en lugar de bloquear nada. Consulte [Proporcionar servidores a través de configuración administrada](/docs/es/managed-mcp#provide-servers-through-managed-settings). Requiere Claude Code v2.1.259 o posterior                                                                                                                                                                                                                                                          |
| [`managedSourcesBehavior`](/docs/es/settings-reference#managedsourcesbehavior)                                             | Si Claude Code aplica solo la fuente administrada de mayor prioridad o [compone cada una de ellas](#compose-every-managed-source)                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| [`parentSettingsBehavior`](/docs/es/settings-reference#parentsettingsbehavior)                                             | Si la configuración principal suministrada por el host se fusiona bajo la política administrada                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| [`pluginSuggestionMarketplaces`](/docs/es/settings-reference#pluginsuggestionmarketplaces)                                 | Mercados cuyos plugins Claude Code puede sugerir a los usuarios                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| [`pluginTrustMessage`](/docs/es/settings-reference#plugintrustmessage)                                                     | Mensaje personalizado agregado a la advertencia de confianza de plugin mostrada antes de la instalación                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| [`policyHelper`](/docs/es/settings-reference#policyhelper)                                                                 | Ejecutable que calcula la configuración administrada al inicio; consulte [Calcular configuración administrada con un auxiliar de política](/docs/es/settings-reference#policyhelper)                                                                                                                                                                                                                                                                                                                                                                                             |
| [`sandbox.filesystem.allowManagedReadPathsOnly`](/docs/es/settings-reference#sandbox-filesystem-allowmanagedreadpathsonly) | Cuando es `true`, solo se respetan las rutas `filesystem.allowRead` de la configuración administrada. `denyRead` aún se fusiona de todas las fuentes                                                                                                                                                                                                                                                                                                                                                                                                                        |
| [`sandbox.network.allowManagedDomainsOnly`](/docs/es/settings-reference#sandbox-network-allowmanageddomainsonly)           | Honre solo los dominios `allowedDomains` administrados y las reglas de permitidos `WebFetch(domain:...)`; bloquee otros dominios sin preguntar                                                                                                                                                                                                                                                                                                                                                                                                                              |
| [`strictKnownMarketplaces`](/docs/es/settings-reference#strictknownmarketplaces)                                           | Controla qué fuentes de mercado de plugins pueden agregar los usuarios e instalar plugins. Consulte [restricciones de mercado administradas](/docs/es/plugins/org#restrict-what-users-can-install)                                                                                                                                                                                                                                                                                                                                                                               |
| [`strictPluginOnlyCustomization`](/docs/es/settings-reference#strictpluginonlycustomization)                               | Bloquee skills, agentes, hooks y servidores MCP de fuentes de usuario y proyecto; `true` bloquea los cuatro, una matriz nombra cuál                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| [`wslInheritsWindowsSettings`](/docs/es/settings-reference#wslinheritswindowssettings)                                     | Cuando se establece en el registro HKLM o un archivo bajo `C:\Program Files\ClaudeCode`, haga que WSL lea la cadena de política de Windows, y lea `/etc/claude-code` solo cuando ningún archivo de configuración administrada o complemento bajo ese directorio entregue una [clave de política](#how-claude-code-combines-managed-sources); la entrada da el orden                                                                                                                                                                                                         |

<Note>
  En planes de Team y Enterprise, un Owner habilita o deshabilita [Remote Control](/docs/es/remote-control) y [sesiones web](/docs/es/claude-code-on-the-web) en toda la organización en [configuración de administrador de Claude Code](https://claude.ai/admin-settings/claude-code). Remote Control también se puede deshabilitar por dispositivo con la configuración [`disableRemoteControl`](/docs/es/settings-reference#disableremotecontrol). Las sesiones web no tienen clave de configuración administrada por dispositivo.

  Para verificar si estas configuraciones de organización llegaron a una máquina determinada, ejecute `claude doctor` allí y lea la línea `Organization policy`, que dice dónde Claude Code cargó la política o por qué no la cargó. Requiere Claude Code v2.1.261 o posterior. En una sesión en ejecución, `/status` muestra la misma línea cuando la política no se cargó.
</Note>

<h2 id="turn-telemetry-off-for-your-organization">
  Desactivar la telemetría para su organización
</h2>

Claude Code envía [telemetría](/docs/es/data-usage#telemetry-services) operativa de Anthropic por defecto en sesiones que usan la API de Anthropic, ya sea directamente, a través de una puerta de enlace LLM o a través de un `ANTHROPIC_BASE_URL` personalizado; [Comportamientos predeterminados por proveedor de API](/docs/es/data-usage#default-behaviors-by-api-provider) dice qué proveedores la envían. Para desactivarla para cada desarrollador sin depender de la shell de cada persona, entregue `DISABLE_TELEMETRY` a través del bloque `env` de su configuración administrada. Este ejemplo establece `DISABLE_TELEMETRY` para todos los que la política alcanza:

```json theme={null}
{
  "env": {
    "DISABLE_TELEMETRY": "1"
  }
}
```

Claude Code aplica un valor de `1` sin mostrar al usuario el [diálogo de aprobación](/docs/es/server-managed-settings#environment-variables-and-the-approval-dialog).

Si desactiva la telemetría, Claude Code deja de enviar los datos de uso que alimentan el [panel de análisis](/docs/es/analytics) de su organización para los desarrolladores que la política alcanza. La variable también desactiva la obtención de indicadores de características, lo que hace que Remote Control, modo automático predeterminado y las otras [características que necesitan obtención de indicadores de características](/docs/es/env-vars#features-that-need-feature-flag-fetching) no estén disponibles para esos desarrolladores.

[Dónde y cuándo se aplica una política](#where-and-when-a-policy-applies) dice qué mecanismo de entrega alcanza cada superficie, y [Disponibilidad de plataforma](/docs/es/server-managed-settings#platform-availability) dice qué sesiones omiten la obtención de configuración administrada por servidor.

Si su organización usa claves de cifrado administradas por el cliente y enruta Claude Code a través de una puerta de enlace, [Configurar proxies y puertas de enlace](/docs/es/third-party-integrations#configure-proxies-and-gateways) dice por qué esas sesiones necesitan esta variable.

<h2 id="see-also">
  Ver también
</h2>

* [Configurar Claude Code para su organización](/docs/es/admin-setup): decidir qué aplicar y cómo
* [Configuración administrada por servidor](/docs/es/server-managed-settings): entregar política desde la consola de claude.ai o una puerta de enlace
* [Configuración MCP administrada](/docs/es/managed-mcp): controlar qué servidores MCP pueden usar los desarrolladores
* [Toda la configuración](/docs/es/settings-reference): cada clave, con si una fuente administrada puede establecerla
* [Archivos de configuración de ejemplo](/docs/es/settings-example#an-organizations-managed-settings): un `managed-settings.json` completo que muestra la forma de las claves administradas
