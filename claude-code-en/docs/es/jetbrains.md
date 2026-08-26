> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# JetBrains IDEs

> Utiliza Claude Code con JetBrains IDEs incluyendo IntelliJ, PyCharm, WebStorm y más

Claude Code se integra con JetBrains IDEs a través de un plugin dedicado, proporcionando características como visualización de diferencias interactivas, compartición de contexto de selección y más.

<h2 id="supported-ides">
  IDEs Compatibles
</h2>

El plugin de Claude Code funciona con la mayoría de JetBrains IDEs, incluyendo:

* IntelliJ IDEA
* PyCharm
* Android Studio
* WebStorm
* PhpStorm
* GoLand

<h2 id="features">
  Características
</h2>

* **Lanzamiento rápido**: utiliza `Cmd+Esc` (Mac) o `Ctrl+Esc` (Windows/Linux) para abrir Claude Code directamente desde tu editor, o haz clic en el botón de Claude Code en la interfaz
* **Visualización de diferencias**: los cambios de código se pueden mostrar directamente en el visor de diferencias del IDE en lugar de la terminal
* **Contexto de selección**: la selección actual o pestaña en el IDE se comparte automáticamente con Claude Code. Las [reglas de denegación de `Read`](/docs/es/permissions#read-and-edit) bloquean este intercambio para los archivos coincidentes
* **Atajos de referencia de archivos**: utiliza `Cmd+Option+K` (Mac) o `Alt+Ctrl+K` (Linux/Windows) para insertar referencias de archivos como `@src/auth.ts#L1-99`
* **Compartición de diagnósticos**: los errores de diagnóstico del IDE, como errores de lint y sintaxis, se comparten automáticamente con Claude mientras trabajas

<h2 id="installation">
  Instalación
</h2>

El plugin ejecuta el comando `claude` en la terminal integrada de su IDE y se conecta a él. No incluye su propia copia de la CLI, así que instale ambos componentes:

<Steps>
  <Step title="Instalar Claude Code CLI">
    Siga la [guía de inicio rápido](/docs/es/quickstart) para instalar la CLI si aún no lo ha hecho. El plugin muestra una notificación "No se puede iniciar Claude Code" cuando `claude` no está en su PATH.
  </Step>

  <Step title="Instalar el plugin de JetBrains">
    Instale el [plugin de Claude Code](https://plugins.jetbrains.com/plugin/27310-claude-code-beta-) desde el Marketplace de JetBrains y reinicie su IDE.
  </Step>
</Steps>

Si `claude` está instalado en algún lugar que su IDE no puede encontrar, establezca la ruta completa en la [configuración del comando Claude](#general-settings) del plugin.

Claude Code funciona con cualquier suscripción de Claude de pago (Pro, Max, Team o Enterprise) o una cuenta de Claude Console, y no se requiere ninguna clave API. Se le pedirá que [inicie sesión](/docs/es/authentication#log-in-to-claude-code) la primera vez que ejecute `claude`.

<Note>
  Después de instalar el plugin, es posible que necesite reiniciar completamente su IDE para que surta efecto.
</Note>

<h2 id="usage">
  Uso
</h2>

<h3 id="from-your-ide">
  Desde tu IDE
</h3>

Ejecuta `claude` desde la terminal integrada de tu IDE, y todas las características de integración estarán activas.

<h3 id="from-external-terminals">
  Desde terminales externos
</h3>

Utiliza el comando `/ide` en cualquier terminal externo para conectar Claude Code a tu JetBrains IDE y activar todas las características:

```bash theme={null}
claude
```

```text theme={null}
/ide
```

Si deseas que Claude tenga acceso a los mismos archivos que tu IDE, inicia Claude Code desde el mismo directorio que la raíz del proyecto de tu IDE.

<h2 id="configuration">
  Configuración
</h2>

<h3 id="claude-code-settings">
  Configuración de Claude Code
</h3>

Configura la integración del IDE a través de la configuración de Claude Code:

1. Ejecuta `claude`
2. Ingresa el comando `/config`
3. Establece la herramienta de diferencias en `auto` para mostrar diferencias en el IDE, o `terminal` para mantenerlas en la terminal

<h3 id="plugin-settings">
  Ajustes de plugins
</h3>

Configura el plugin de Claude Code yendo a **Settings → Tools → Claude Code \[Beta]**:

<h4 id="general-settings">
  Configuración general
</h4>

* **Comando Claude**: especifica un comando personalizado para ejecutar Claude, por ejemplo `claude`, `/usr/local/bin/claude`, o `npx @anthropic-ai/claude-code`
* **Suprimir notificación para comando Claude no encontrado**: omite notificaciones sobre no encontrar el comando Claude
* **Habilitar usar Option+Enter para indicadores de varias líneas**: solo en macOS. Cuando está habilitado, Option+Enter inserta nuevas líneas en los indicadores de Claude Code. Desactívalo si la tecla Option se captura inesperadamente. Requiere reinicio de terminal.
* **Habilitar actualizaciones automáticas**: verifica automáticamente e instala actualizaciones del plugin, aplicadas al reiniciar

<Tip>
  Para usuarios de WSL: establece `wsl -d Ubuntu -- bash -lic "claude"` como tu comando Claude (reemplaza `Ubuntu` con el nombre de tu distribución WSL)
</Tip>

<h4 id="esc-key-configuration">
  Configuración de la tecla ESC
</h4>

Si la tecla ESC no interrumpe las operaciones de Claude Code en terminales de JetBrains:

1. Ve a **Settings → Tools → Terminal**
2. Cualquiera de:
   * Desactiva "Move focus to the editor with Escape", o
   * Haz clic en "Configure terminal keybindings" y elimina el atajo "Switch focus to Editor"
3. Aplica los cambios

Esto permite que la tecla ESC interrumpa correctamente las operaciones de Claude Code.

<h2 id="special-configurations">
  Configuraciones especiales
</h2>

<h3 id="remote-development">
  Desarrollo remoto
</h3>

<Warning>
  Cuando utilices JetBrains Remote Development, debes instalar el plugin en el host remoto a través de **Settings → Plugin (Host)**.
</Warning>

El plugin debe instalarse en el host remoto, no en tu máquina cliente local.

<h3 id="wsl-configuration">
  Configuración de WSL
</h3>

Si estás utilizando Claude Code en WSL2 con un JetBrains IDE y ves "No available IDEs detected", la causa generalmente es el enrutamiento NAT de WSL2 o el Firewall de Windows bloqueando la conexión entre WSL2 y el IDE ejecutándose en el host de Windows. WSL1 utiliza la red del host directamente y no se ve afectado.

<h4 id="allow-wsl2-traffic-through-windows-firewall">
  Permitir tráfico de WSL2 a través del Firewall de Windows
</h4>

Esta es la solución recomendada porque mantiene tu modo de red WSL2 existente.

<Steps>
  <Step title="Encuentra tu dirección IP de WSL2">
    Desde dentro de tu shell de WSL, ejecuta:

    ```bash theme={null}
    hostname -I
    ```

    Anota la subred, por ejemplo `172.21.123.45` está en `172.21.0.0/16`.
  </Step>

  <Step title="Crea una regla de firewall">
    Abre PowerShell como Administrador y ejecuta lo siguiente, ajustando el rango de IP para que coincida con tu subred:

    ```powershell theme={null}
    New-NetFirewallRule -DisplayName "Allow WSL2 Internal Traffic" -Direction Inbound -Protocol TCP -Action Allow -RemoteAddress 172.21.0.0/16 -LocalAddress 172.21.0.0/16
    ```
  </Step>

  <Step title="Reinicia tu IDE y Claude Code">
    Cierra y reabre ambos para que la nueva regla surta efecto.
  </Step>
</Steps>

<h4 id="switch-wsl2-to-mirrored-networking">
  Cambiar WSL2 a redes espejadas
</h4>

Las redes espejadas requieren Windows 11 22H2 o posterior. Si estás en Windows 10, utiliza la regla de firewall anterior.

Añade esto a `.wslconfig` en tu directorio de usuario de Windows:

```ini theme={null}
[wsl2]
networkingMode=mirrored
```

Luego reinicia WSL con `wsl --shutdown` desde PowerShell.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="plugin-not-working">
  Plugin no funciona
</h3>

Si el plugin está instalado pero las características de Claude Code no aparecen en tu IDE:

* Asegúrate de que estés ejecutando Claude Code desde el directorio raíz del proyecto
* Verifica que el plugin de JetBrains esté habilitado en la configuración del IDE
* Reinicia completamente el IDE (es posible que necesites hacerlo varias veces)
* Para Desarrollo Remoto, asegúrate de que el plugin esté instalado en el host remoto

<h3 id="ide-not-detected">
  IDE no detectado
</h3>

Si ejecutar `claude` muestra "No available IDEs detected":

* Verifica que el plugin esté instalado y habilitado
* Reinicia completamente el IDE
* Comprueba que estés ejecutando Claude Code desde la terminal integrada
* Para usuarios de WSL, consulta la [configuración de WSL](#wsl-configuration) anterior

<h3 id="command-not-found">
  Comando no encontrado
</h3>

Si hacer clic en el icono de Claude muestra "command not found":

1. Verifica que Claude Code esté instalado ejecutando `claude --version` en una terminal
2. Configura la ruta del comando Claude en la configuración del plugin
3. Para usuarios de WSL, utiliza el formato de comando WSL mencionado en la sección de configuración

<h2 id="security-considerations">
  Consideraciones de seguridad
</h2>

Cuando Claude Code se ejecuta en un JetBrains IDE en modo de permiso [`acceptEdits`](/docs/es/permission-modes#auto-approve-file-edits-with-acceptedits-mode), puede ser capaz de modificar archivos de configuración del IDE que pueden ser ejecutados automáticamente por su IDE. Esto puede aumentar el riesgo de ejecutar Claude Code en modo `acceptEdits` y permitir eludir los indicadores de permiso de Claude Code para la ejecución de bash.

Cuando se ejecuta en JetBrains IDEs, considere:

* Usar el modo de aprobación manual para ediciones
* Tener especial cuidado para asegurar que Claude solo se use con indicadores de confianza
* Ser consciente de qué archivos Claude Code tiene acceso para modificar

Para problemas de instalación o inicio de sesión de Claude Code fuera del IDE, consulte [Solucionar problemas de instalación e inicio de sesión](/docs/es/troubleshoot-install).

<h3 id="the-built-in-ide-mcp-server">
  El servidor MCP integrado del IDE
</h3>

Cuando el plugin está activo, ejecuta un servidor MCP local al que la CLI se conecta automáticamente. Así es como la CLI abre diffs en el visor de diffs nativo del IDE, lee su selección actual para menciones `@`, e incorpora diagnósticos de inspección en la conversación.

El servidor se llama `ide` y está oculto de `/mcp` porque no hay nada que configurar. Sin embargo, si su organización utiliza un [hook `PreToolUse`](/docs/es/hooks#pretooluse) para crear una lista de herramientas MCP permitidas, necesitará saber que existe.

**Contexto de selección y archivo abierto.** Mientras está conectado, la CLI incluye su selección actual del editor y la ruta del archivo activo como contexto en cada indicador que envía. La transcripción muestra una línea `⧉ Selected N lines from <file>` cuando esto sucede. Para excluir un archivo sensible como `.env`, agregue una [regla de denegación `Read`](/docs/es/permissions#read-and-edit) para su ruta. Una regla de denegación coincidente previene tanto el texto seleccionado como el aviso de archivo abierto para ese archivo de llegar a Claude.

**Transporte y autenticación.** El servidor escucha en un puerto efímero asignado por el sistema operativo, y el puerto no es configurable. El transporte es `ws://` sin cifrar; en loopback, cualquier proceso que pudiera capturar el tráfico también puede leer el token del archivo de bloqueo, por lo que TLS no agregaría protección contra un atacante local. Cada inicio del IDE genera un token de autenticación aleatorio nuevo, lo escribe en un archivo de bloqueo en `~/.claude/ide/<port>.lock`, y la CLI debe presentarlo como el encabezado `X-Claude-Code-Ide-Authorization` para conectarse. Si se establece `CLAUDE_CONFIG_DIR`, el archivo de bloqueo se escribe en `$CLAUDE_CONFIG_DIR/ide/` en su lugar.

**Herramientas expuestas al modelo.** El servidor aloja varias herramientas, pero solo una es visible para el modelo. El resto son RPC internas que la CLI utiliza para su propia interfaz de usuario, como abrir diffs y leer selecciones, y se filtran antes de que la lista de herramientas llegue a Claude.

| Nombre de herramienta (como se ve en hooks) | Qué hace                                                                                                                                 | Solo lectura |
| ------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- | ------------ |
| `mcp__ide__getDiagnostics`                  | Devuelve los diagnósticos de inspección del IDE, los errores y advertencias mostrados en el editor. Opcionalmente limitado a un archivo. | Sí           |

El plugin de JetBrains no expone una herramienta de ejecución de código al modelo.

**Interfaz de escucha.** Qué interfaz de red el servidor se vincula está controlado por **Accept connections from all network interfaces** bajo **Settings → Tools → Claude Code \[Beta] → Networking (Advanced)**. Con la configuración deshabilitada, el servidor escucha solo en `127.0.0.1` y no es accesible desde otros hosts. Con ella habilitada, el puerto es accesible desde su red local. La configuración existe para casos donde la CLI no puede alcanzar el IDE sobre loopback, como WSL2 con redes NAT predeterminadas o una configuración de IDE remoto; consulte [Configuración de WSL](#wsl-configuration) para ese escenario.

<Warning>
  Habilitar **Accept connections from all network interfaces** hace que el puerto MCP del IDE sea accesible desde su red local. Las conexiones aún requieren el token de autenticación del archivo de bloqueo, pero debido a que el transporte es `ws://` sin cifrar, tanto el tráfico de sesión como ese token cruzan la red en texto plano cuando la configuración está activada. Solo actívela cuando loopback genuinamente no pueda funcionar. Para WSL2, prefiera [redes espejadas](#switch-wsl2-to-mirrored-networking) para que la interfaz de loopback de Windows se comparta con la máquina virtual Linux y el socket pueda permanecer en loopback.
</Warning>
