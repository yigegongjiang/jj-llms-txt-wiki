> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Solucionar problemas de instalación e inicio de sesión

> Corrija errores de comando no encontrado, PATH, permisos, red y autenticación al instalar o iniciar sesión en Claude Code.

Si la instalación falla o no puede iniciar sesión, encuentre su error a continuación. Para problemas en tiempo de ejecución después de que Claude Code esté funcionando, consulte [Solución de problemas](/docs/es/troubleshooting). Para problemas de configuración como configuraciones que no se aplican o hooks que no se disparan, consulte [Depurar su configuración](/docs/es/debug-your-config).

<h2 id="find-your-error">
  Encuentre su error
</h2>

Haga coincidir el mensaje de error o síntoma que está viendo con una solución:

| Lo que ve                                                                                                    | Solución                                                                                                                                        |
| :----------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------- |
| `command not found: claude` o `'claude' is not recognized`                                                   | [Corrija su PATH](#command-not-found-claude-after-installation)                                                                                 |
| `syntax error near unexpected token '<'`                                                                     | [El script de instalación devuelve HTML](#install-script-returns-html-instead-of-a-shell-script)                                                |
| `curl: (22) The requested URL returned error: 403`                                                           | [El script de instalación devolvió 403](#install-script-returns-html-instead-of-a-shell-script)                                                 |
| `curl: (23)` o `curl: (56) Failure writing output to destination`                                            | [Verifique la conectividad o use un instalador alternativo](#curl-56-failure-writing-output-to-destination)                                     |
| `Killed` durante la instalación en Linux, o `Installation was killed before it could finish (exit code 137)` | [Libere memoria o agregue espacio de intercambio](#install-killed-on-low-memory-linux-servers)                                                  |
| `TLS connect error` o `SSL/TLS secure channel`                                                               | [Actualice los certificados CA](#tls-or-ssl-connection-errors)                                                                                  |
| `Failed to fetch version` o no puede alcanzar el servidor de descarga                                        | [Verifique la configuración de red y proxy](#check-network-connectivity)                                                                        |
| `irm is not recognized` o `&& is not valid`                                                                  | [Use el comando correcto para su shell](#wrong-install-command-on-windows)                                                                      |
| `Cask 'claude-code' is unavailable: No Cask with this name exists`                                           | [Actualice Homebrew](#homebrew-cask-unavailable-or-outdated)                                                                                    |
| `'bash' is not recognized as the name of a cmdlet`                                                           | [Use el comando del instalador de Windows](#wrong-install-command-on-windows)                                                                   |
| `A parameter cannot be found that matches parameter name 'fsSL'`                                             | [Use el comando del instalador de Windows](#wrong-install-command-on-windows)                                                                   |
| `Claude Code on Windows requires either Git for Windows (for bash) or PowerShell`                            | [Instale un shell](#claude-code-on-windows-requires-either-git-for-windows-for-bash-or-powershell)                                              |
| `Claude Code does not support 32-bit Windows`                                                                | [Abra Windows PowerShell, no la entrada x86](#claude-code-does-not-support-32-bit-windows)                                                      |
| `The process cannot access the file ... because it is being used by another process`                         | [Borre la carpeta de descargas e intente de nuevo](#the-process-cannot-access-the-file-during-windows-install)                                  |
| `Error loading shared library`                                                                               | [Variante binaria incorrecta para su sistema](#linux-musl-or-glibc-binary-mismatch)                                                             |
| `Illegal instruction`                                                                                        | [Desajuste de arquitectura o conjunto de instrucciones de CPU](#illegal-instruction)                                                            |
| `cannot execute binary file: Exec format error` en WSL                                                       | [Regresión binaria nativa de WSL1](#exec-format-error-on-wsl1)                                                                                  |
| El instalador de PowerShell se completa pero `claude` no se encuentra o muestra una versión anterior         | [Agregue el directorio de instalación a su PATH](#verify-your-path), luego abra una nueva terminal                                              |
| `dyld: cannot load`, `dyld: Symbol not found`, o `Abort trap` en macOS                                       | [Incompatibilidad binaria](#dyld-cannot-load-on-macos)                                                                                          |
| `Invoke-Expression: Missing argument in parameter list`                                                      | [El script de instalación devuelve HTML](#install-script-returns-html-instead-of-a-shell-script)                                                |
| `App unavailable in region`                                                                                  | Claude Code no está disponible en su país. Consulte [países admitidos](https://www.anthropic.com/supported-countries).                          |
| `unable to get local issuer certificate`                                                                     | [Configure certificados CA corporativos](#tls-or-ssl-connection-errors)                                                                         |
| `OAuth error` u `403 Forbidden`                                                                              | [Corrija la autenticación](#login-and-authentication)                                                                                           |
| `Could not load the default credentials` o `Could not load credentials from any providers`                   | [Credenciales de Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry](#bedrock-agent-platform-or-foundry-credentials-not-loading) |
| `ChainedTokenCredential authentication failed` o `CredentialUnavailableError`                                | [Credenciales de Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry](#bedrock-agent-platform-or-foundry-credentials-not-loading) |
| `API Error: 500`, `529 Overloaded`, `429`, u otros errores 4xx y 5xx no listados arriba                      | Consulte la [referencia de errores](/docs/es/errors)                                                                                                 |

Si su problema no está listado, trabaje a través de las verificaciones de diagnóstico a continuación para reducir la causa.

<Tip>
  Si prefiere omitir la terminal por completo, la [aplicación de escritorio Claude Code](/docs/es/desktop-quickstart) le permite instalar y usar Claude Code a través de una interfaz gráfica. Descárguela para [macOS](https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code\&utm_medium=docs) o [Windows](https://claude.com/download?utm_source=claude_code\&utm_medium=docs) y comience a codificar sin ninguna configuración de línea de comandos. En Linux, instale la aplicación con apt siguiendo las [instrucciones de instalación de Linux](/docs/es/desktop-linux).
</Tip>

<h2 id="run-diagnostic-checks">
  Ejecute verificaciones de diagnóstico
</h2>

<h3 id="check-network-connectivity">
  Verifique la conectividad de red
</h3>

El instalador descarga desde `downloads.claude.ai`. Verifique que pueda alcanzarlo:

```bash theme={null}
curl -sI https://downloads.claude.ai/claude-code-releases/latest
```

En PowerShell, ejecute `curl.exe -sI` en su lugar. PowerShell crea un alias de `curl` a `Invoke-WebRequest`, que rechaza los indicadores `-sI`.

Una línea `HTTP/2 200` significa que alcanzó el servidor. Si no ve salida, `Could not resolve host`, o un tiempo de espera de conexión, su red está bloqueando la conexión. Las causas comunes incluyen:

* Firewalls corporativos o proxies bloqueando `downloads.claude.ai`
* Restricciones de red regional: intente una VPN o red alternativa
* Problemas de TLS/SSL: actualice los certificados CA de su sistema, o verifique si `HTTPS_PROXY` está configurado

Si está detrás de un proxy corporativo, establezca `HTTPS_PROXY` y `HTTP_PROXY` en la dirección de su proxy antes de instalar. Pregunte a su equipo de TI por la URL del proxy si no la conoce, o verifique la configuración del proxy de su navegador.

Este ejemplo establece ambas variables de proxy, luego ejecuta el instalador a través de su proxy:

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    export HTTP_PROXY=http://proxy.example.com:8080
    export HTTPS_PROXY=http://proxy.example.com:8080
    curl -fsSL https://claude.ai/install.sh | bash
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    $env:HTTP_PROXY = 'http://proxy.example.com:8080'
    $env:HTTPS_PROXY = 'http://proxy.example.com:8080'
    irm https://claude.ai/install.ps1 | iex
    ```
  </Tab>
</Tabs>

<h3 id="verify-your-path">
  Verifique su PATH
</h3>

Si la instalación fue exitosa pero obtiene un error `command not found` o `not recognized` al ejecutar `claude`, el directorio de instalación no está en su PATH. Su shell busca programas en directorios listados en PATH, y el instalador coloca `claude` en `~/.local/bin/claude` en macOS/Linux o `%USERPROFILE%\.local\bin\claude.exe` en Windows.

<Note>
  La [extensión de VS Code](/docs/es/vs-code) no coloca `claude` en esta ubicación. Agrupa una copia privada de la CLI dentro del directorio de la extensión para su propio panel de chat y no la agrega a PATH. Si solo ha instalado la extensión, `~/.local/bin/claude` no existirá. Ejecute la [instalación independiente](/docs/es/setup) para usar `claude` desde una terminal, luego continúe a continuación.
</Note>

Verifique si el directorio de instalación está en su PATH listando sus entradas de PATH y filtrando por `local/bin`:

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    echo $PATH | tr ':' '\n' | grep -Fx "$HOME/.local/bin"
    ```

    Si esto imprime `/Users/you/.local/bin` o `/home/you/.local/bin`, el directorio está en su PATH y puede saltar a [Verifique instalaciones conflictivas](#check-for-conflicting-installations). Si no hay salida, agréguelo a su configuración de shell.

    Para Zsh, el predeterminado en macOS:

    ```bash theme={null}
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
    source ~/.zshrc
    ```

    Para Bash, el predeterminado en la mayoría de distribuciones de Linux:

    ```bash theme={null}
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
    source ~/.bashrc
    ```

    Alternativamente, cierre y vuelva a abrir su terminal.

    Para otros shells como fish o Nushell, agregue `~/.local/bin` a su PATH usando la sintaxis de configuración propia de su shell, luego reinicie su terminal.

    Verifique que la corrección funcionó:

    ```bash theme={null}
    claude --version
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    $env:PATH -split ';' | Select-String '\.local\\bin'
    ```

    Si no hay salida, agregue el directorio de instalación a su PATH de usuario:

    ```powershell theme={null}
    $currentPath = [Environment]::GetEnvironmentVariable('PATH', 'User')
    [Environment]::SetEnvironmentVariable('PATH', "$currentPath;$env:USERPROFILE\.local\bin", 'User')
    ```

    Reinicie su terminal para que el cambio surta efecto.

    Verifique que la corrección funcionó:

    ```powershell theme={null}
    claude --version
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    echo %PATH% | findstr /i "local\bin"
    ```

    Si no hay salida, abra Configuración del sistema, vaya a Variables de entorno, y agregue `%USERPROFILE%\.local\bin` a su variable PATH de usuario. Reinicie su terminal.

    Verifique que la corrección funcionó:

    ```batch theme={null}
    claude --version
    ```
  </Tab>
</Tabs>

<h3 id="check-for-conflicting-installations">
  Verifique instalaciones conflictivas
</h3>

Múltiples instalaciones de Claude Code pueden causar desajustes de versión o comportamiento inesperado. Verifique qué está instalado:

<Tabs>
  <Tab title="macOS/Linux">
    Liste todos los binarios `claude` encontrados en su PATH:

    ```bash theme={null}
    which -a claude
    ```

    Si esto no imprime nada, ningún `claude` está en su PATH aún. Vuelva a [Verifique su PATH](#verify-your-path).

    Verifique las tres ubicaciones de donde puede venir un binario `claude`. `~/.local/bin/claude` es el instalador nativo, `~/.claude/local/` es una instalación npm local heredada creada por versiones anteriores de Claude Code, y la lista npm global muestra una instalación `-g`:

    ```bash theme={null}
    ls -la ~/.local/bin/claude
    ```

    Una instalación nativa muestra un enlace simbólico en `~/.local/share/claude/versions/`. Un script o un enlace simbólico que creó usted mismo en esta ruta es un iniciador personalizado, que [la actualización automática deja en su lugar](/docs/es/setup#auto-updates).

    Si algún comando `ls` imprime `No such file or directory`, eso no es un error. Significa que nada está instalado en esa ubicación, así que continúe con la siguiente verificación.

    ```bash theme={null}
    ls -la ~/.claude/local/
    ```

    ```bash theme={null}
    npm -g ls @anthropic-ai/claude-code 2>/dev/null
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    Liste todos los binarios `claude` encontrados en su PATH:

    ```powershell theme={null}
    where.exe claude
    ```

    Verifique si el instalador nativo colocó un binario:

    ```powershell theme={null}
    Test-Path "$env:USERPROFILE\.local\bin\claude.exe"
    ```
  </Tab>
</Tabs>

Si encuentra múltiples instalaciones, mantenga solo una. La instalación nativa en `~/.local/bin/claude` en macOS/Linux o `%USERPROFILE%\.local\bin\claude.exe` en Windows es recomendada. Elimine las extras:

Desinstale una instalación npm global:

```bash theme={null}
npm uninstall -g @anthropic-ai/claude-code
```

Elimine la instalación npm local heredada:

```bash theme={null}
rm -rf ~/.claude/local
```

En Windows, use PowerShell:

```powershell theme={null}
Remove-Item -Recurse -Force "$env:USERPROFILE\.claude\local"
```

Elimine una instalación de Homebrew en macOS. Si instaló el cask `claude-code@latest`, sustituya ese nombre:

```bash theme={null}
brew uninstall --cask claude-code
```

Elimine una instalación de WinGet en Windows:

```powershell theme={null}
winget uninstall Anthropic.ClaudeCode
```

<h3 id="check-directory-permissions">
  Verifique permisos de directorio
</h3>

El instalador necesita acceso de escritura a `~/.local/bin/` y `~/.claude/` en macOS y Linux. En Windows la ubicación de instalación está bajo `%USERPROFILE%`, que es escribible por su usuario de forma predeterminada, por lo que esta sección rara vez se aplica allí.

Verifique si los directorios son escribibles:

```bash theme={null}
test -w ~/.local/bin && echo "writable" || echo "not writable"
test -w ~/.claude && echo "writable" || echo "not writable"
```

Si algún directorio no es escribible, cree el directorio de instalación y establezca su usuario como propietario:

```bash theme={null}
sudo mkdir -p ~/.local/bin
sudo chown -R $(whoami) ~/.local
```

<h3 id="verify-the-binary-works">
  Verifique que el binario funciona
</h3>

Si `claude --version` imprime una versión pero `claude` se bloquea o cuelga al iniciar, ejecute estas verificaciones para reducir la causa. Si `claude --version` dice comando no encontrado, vaya a [Verifique su PATH](#verify-your-path) primero; los comandos a continuación asumen que `claude` está en su PATH.

Confirme que el binario existe y es ejecutable:

```bash theme={null}
ls -la "$(command -v claude)"
```

En Windows, use PowerShell:

```powershell theme={null}
Get-Command claude | Select-Object Source
```

En Linux, verifique bibliotecas compartidas faltantes. Si `ldd` muestra bibliotecas faltantes, es posible que deba instalar paquetes del sistema. En Alpine Linux y otras distribuciones basadas en musl, consulte [Configuración de Alpine Linux](/docs/es/setup#alpine-linux-and-musl-based-distributions).

```bash theme={null}
ldd "$(command -v claude)" | grep "not found"
```

Confirme que el binario puede ejecutarse:

```bash theme={null}
claude --version
```

<h2 id="common-installation-issues">
  Problemas comunes de instalación
</h2>

Estos son los problemas de instalación más frecuentes y sus soluciones.

<h3 id="install-script-returns-html-instead-of-a-shell-script">
  El script de instalación devuelve HTML en lugar de un script de shell
</h3>

Al ejecutar el comando de instalación, puede ver uno de estos errores:

```text theme={null}
bash: line 1: syntax error near unexpected token `<'
bash: line 1: `<!DOCTYPE html>'
```

En PowerShell, el mismo problema aparece como:

```text theme={null}
Invoke-Expression: Missing argument in parameter list.
```

Dependiendo de cómo se enrutó la solicitud, en su lugar puede ver un 403 sin cuerpo HTML:

```text theme={null}
curl: (22) The requested URL returned error: 403
```

Todos estos significan que la URL de instalación devolvió una página HTML o un estado de error en lugar del script de instalación. Si la página HTML dice "App unavailable in region", Claude Code no está disponible en su país. Consulte [países admitidos](https://www.anthropic.com/supported-countries).

Un 403 desnudo sin cuerpo a menudo tiene la misma causa, pero también puede provenir de un proxy corporativo o firewall que bloquea la descarga. Si está en un país admitido y aún ve el 403, trabaje a través de [Verifique la conectividad de red](#check-network-connectivity) antes de intentar los instaladores alternativos a continuación, ya que esos alcanzan los mismos hosts.

De lo contrario, esto puede ocurrir debido a problemas de red, enrutamiento regional o una interrupción temporal del servicio.

**Soluciones:**

1. **Use un método de instalación alternativo**:

   En macOS, instale a través de Homebrew:

   ```bash theme={null}
   brew install --cask claude-code
   ```

   En Windows, instale a través de WinGet:

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

2. **Reinténtelo después de unos minutos**: el problema suele ser temporal. Espere e intente el comando original nuevamente.

<h3 id="command-not-found-claude-after-installation">
  `command not found: claude` después de la instalación
</h3>

La instalación finalizó pero `claude` no funciona. El error exacto varía según la plataforma:

| Plataforma  | Mensaje de error                                                       |
| :---------- | :--------------------------------------------------------------------- |
| macOS       | `zsh: command not found: claude`                                       |
| Linux       | `bash: claude: command not found`                                      |
| Windows CMD | `'claude' is not recognized as an internal or external command`        |
| PowerShell  | `claude : The term 'claude' is not recognized as the name of a cmdlet` |

Esto significa que el directorio de instalación no está en la ruta de búsqueda de su shell. Consulte [Verifique su PATH](#verify-your-path) para la corrección en cada plataforma.

<h3 id="curl-56-failure-writing-output-to-destination">
  `curl: (56) Failure writing output to destination`
</h3>

El comando `curl ... | bash` descarga el script y lo canaliza a Bash para su ejecución. Este error, y el relacionado `curl: (23) Failure writing output to destination`, significa que Bash no recibió el script completo. El código de salida 56 indica que la descarga en sí fue interrumpida, y el código de salida 23 indica que curl no pudo escribir lo que recibió en la tubería, generalmente porque Bash salió temprano.

**Soluciones:**

1. **Verifique la estabilidad de la red**: Los binarios de Claude Code se alojan en `downloads.claude.ai`. Pruebe que pueda alcanzarlo:
   ```bash theme={null}
   curl -sI https://downloads.claude.ai/claude-code-releases/latest
   ```
   Una línea `HTTP/2 200` significa que alcanzó el servidor y el fallo original probablemente fue intermitente; reintente el comando de instalación. Si ve `Could not resolve host` o un tiempo de espera de conexión, su red está bloqueando la descarga.

2. **Intente un método de instalación alternativo**:

   En macOS:

   ```bash theme={null}
   brew install --cask claude-code
   ```

   En Windows:

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

<h3 id="homebrew-cask-unavailable-or-outdated">
  Cask de Homebrew no disponible u obsoleto
</h3>

Homebrew reporta `Error: Cask 'claude-code' is unavailable: No Cask with this name exists` cuando su copia local del índice de cask de Homebrew es anterior a la publicación del cask. Actualice el índice e intente nuevamente:

```bash theme={null}
brew update
brew install --cask claude-code
```

Si Homebrew instala una versión anterior de Claude Code de la que espera, el mismo índice obsoleto suele ser la causa. El cask `claude-code` rastrea el canal estable y típicamente está aproximadamente una semana detrás de la última versión; para la versión más reciente ejecute `brew install --cask claude-code@latest` en su lugar. Consulte [Configurar canal de versión](/docs/es/setup#configure-release-channel) para la diferencia entre los dos casks.

<h3 id="tls-or-ssl-connection-errors">
  Errores de conexión TLS o SSL
</h3>

Errores como `curl: (35) TLS connect error`, `schannel: next InitializeSecurityContext failed`, o el `Could not establish trust relationship for the SSL/TLS secure channel` de PowerShell indican fallos de protocolo de enlace TLS.

**Soluciones:**

1. **Actualice sus certificados CA del sistema**:

   En Ubuntu/Debian:

   ```bash theme={null}
   sudo apt-get update && sudo apt-get install ca-certificates
   ```

   En macOS, el curl del sistema usa el almacén de confianza de Keychain; actualizar macOS en sí actualiza los certificados raíz.

2. **En Windows, habilite TLS 1.2** en PowerShell antes de ejecutar el instalador:
   ```powershell theme={null}
   [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
   irm https://claude.ai/install.ps1 | iex
   ```

3. **Verifique la interferencia de proxy o firewall**: los proxies corporativos que realizan inspección TLS pueden causar estos errores, incluidos `unable to get local issuer certificate` y `SELF_SIGNED_CERT_IN_CHAIN`. Para el paso de instalación, apunte curl a su paquete CA corporativo con `--cacert`:
   ```bash theme={null}
   curl --cacert /path/to/corporate-ca.pem -fsSL https://claude.ai/install.sh | bash
   ```
   Para Claude Code en sí una vez instalado, establezca `NODE_EXTRA_CA_CERTS` para que las solicitudes de API confíen en el mismo paquete:
   ```bash theme={null}
   export NODE_EXTRA_CA_CERTS=/path/to/corporate-ca.pem
   ```
   Pregunte a su equipo de TI por el archivo de certificado si no lo tiene. También puede intentar en una conexión directa para confirmar que el proxy es la causa.

4. **En Windows, cambie instaladores si su red bloquea verificaciones de revocación**. Los errores `CRYPT_E_NO_REVOCATION_CHECK (0x80092012)` y `CRYPT_E_REVOCATION_OFFLINE (0x80092013)` significan que curl alcanzó el servidor pero su red bloquea la búsqueda de revocación de certificados, que es común detrás de firewalls corporativos. Agregar la bandera `--ssl-revoke-best-effort` de curl no soluciona esto: la bandera solo se aplica a descargar `install.cmd` en sí, y las descargas propias del script se ejecutan sin ella, por lo que la instalación falla con el mismo error. Use un método de instalación que tolere la búsqueda bloqueada en su lugar. Abra PowerShell y ejecute el instalador de PowerShell, que descarga a través de .NET y no falla cuando el servidor de revocación es inaccesible:
   ```powershell theme={null}
   irm https://claude.ai/install.ps1 | iex
   ```
   También puede instalar con `winget install Anthropic.ClaudeCode`, que evita curl por completo.

<h3 id="failed-to-fetch-version-from-downloads-claude-ai">
  `Failed to fetch version from downloads.claude.ai`
</h3>

El instalador no pudo alcanzar el servidor de descarga. Esto típicamente significa que `downloads.claude.ai` está bloqueado en su red.

**Soluciones:**

1. **Pruebe la conectividad directamente**:
   ```bash theme={null}
   curl -sI https://downloads.claude.ai/claude-code-releases/latest
   ```

2. **Si está detrás de un proxy**, establezca `HTTPS_PROXY` para que el instalador pueda enrutarse a través de él. Consulte [configuración de proxy](/docs/es/network-config#proxy-configuration) para detalles.
   ```bash theme={null}
   export HTTPS_PROXY=http://proxy.example.com:8080
   curl -fsSL https://claude.ai/install.sh | bash
   ```

3. **Si está en una red restringida**, intente una red diferente o VPN, o use un método de instalación alternativo:

   En macOS:

   ```bash theme={null}
   brew install --cask claude-code
   ```

   En Windows:

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

<h3 id="wrong-install-command-on-windows">
  Comando de instalación incorrecto en Windows
</h3>

Si ve `'irm' is not recognized`, `The token '&&' is not valid`, `A parameter cannot be found that matches parameter name 'fsSL'`, o `'bash' is not recognized as the name of a cmdlet`, copió el comando de instalación para un shell o sistema operativo diferente.

* **`irm` no reconocido**: está en CMD, no en PowerShell. Tiene dos opciones:

  Abra PowerShell buscando "PowerShell" en el menú Inicio, luego ejecute el comando de instalación original:

  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

  O permanezca en CMD y use el instalador de CMD en su lugar:

  ```batch theme={null}
  curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
  ```

* **`&&` no válido**: está en PowerShell pero ejecutó el comando del instalador de CMD. Use el instalador de PowerShell:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

* **`A parameter cannot be found that matches parameter name 'fsSL'`**: ejecutó el instalador de macOS/Linux `curl -fsSL ... | bash` en Windows PowerShell, donde `curl` es un alias para `Invoke-WebRequest` y rechaza los indicadores `-fsSL`. Use el instalador de PowerShell en su lugar:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

* **`bash` no reconocido**: ejecutó el instalador de macOS/Linux en Windows. Use el instalador de PowerShell en su lugar:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

<h3 id="the-process-cannot-access-the-file-during-windows-install">
  `The process cannot access the file` durante la instalación en Windows
</h3>

Si el instalador de PowerShell falla con `Failed to download binary: The process cannot access the file ... because it is being used by another process`, el instalador no pudo escribir en `%USERPROFILE%\.claude\downloads`. Esto generalmente significa que un intento de instalación anterior aún se está ejecutando, o el software antivirus está escaneando un binario descargado parcialmente en esa carpeta.

Cierre cualquier otra ventana de PowerShell ejecutando el instalador y espere a que los escaneos de antivirus liberen el archivo. Luego elimine la carpeta de descargas y ejecute el instalador nuevamente:

```powershell theme={null}
Remove-Item -Recurse -Force "$env:USERPROFILE\.claude\downloads"
irm https://claude.ai/install.ps1 | iex
```

<h3 id="install-killed-on-low-memory-linux-servers">
  La instalación se cuelga en servidores Linux con poca memoria
</h3>

Un mensaje `Killed` durante la instalación generalmente significa que el asesino de falta de memoria (OOM) de Linux terminó el paso `claude install` porque el sistema se quedó sin memoria libre. Esto es común en VPS pequeños e instancias en la nube. El script de instalación reporta la causa y sale con código 137:

```text theme={null}
Setting up Claude Code...
bash: line 142: 34803 Killed    "$binary_path" install ${TARGET:+"$TARGET"}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

Antes de v2.1.200, el script salía solo con la línea `Killed` desnuda del shell y sin explicación.

La instalación necesita aproximadamente 512 MB de memoria libre, y ejecutar Claude Code necesita más. Consulte los [requisitos del sistema](/docs/es/setup#system-requirements).

**Soluciones:**

1. **Agregue espacio de intercambio** si su servidor tiene RAM limitada. El intercambio usa espacio en disco como memoria de desbordamiento, permitiendo que la instalación se complete incluso con RAM física baja.

   Cree un archivo de intercambio de 2 GB y habilítelo:

   ```bash theme={null}
   sudo fallocate -l 2G /swapfile
   sudo chmod 600 /swapfile
   sudo mkswap /swapfile
   sudo swapon /swapfile
   ```

   Luego reintente la instalación:

   ```bash theme={null}
   curl -fsSL https://claude.ai/install.sh | bash
   ```

2. **Cierre otros procesos** para liberar memoria antes de instalar.

3. **Use una instancia más grande** si es posible. Claude Code requiere al menos 4 GB de RAM.

<h3 id="install-hangs-in-docker">
  La instalación se cuelga en Docker
</h3>

Al instalar Claude Code en un contenedor Docker, instalar como root en `/` puede causar cuelgues.

**Soluciones:**

1. **Establezca un directorio de trabajo** antes de ejecutar el instalador. Cuando se ejecuta desde `/`, el instalador escanea todo el sistema de archivos, lo que causa un uso excesivo de memoria. Establecer `WORKDIR` limita el escaneo a un directorio pequeño:
   ```dockerfile theme={null}
   WORKDIR /tmp
   RUN curl -fsSL https://claude.ai/install.sh | bash
   ```

2. **Aumente los límites de memoria de Docker** si usa Docker Desktop:
   ```bash theme={null}
   docker build --memory=4g .
   ```

<h3 id="claude-desktop-overrides-the-claude-command-on-windows">
  Claude Desktop anula el comando `claude` en Windows
</h3>

Si instaló una versión anterior de Claude Desktop, puede registrar un `Claude.exe` en el directorio `WindowsApps` que toma prioridad de PATH sobre Claude Code CLI. Ejecutar `claude` abre la aplicación de escritorio en lugar de la CLI.

Actualice Claude Desktop a la versión más reciente para corregir este problema.

<h3 id="claude-code-on-windows-requires-either-git-for-windows-for-bash-or-powershell">
  Claude Code en Windows requiere Git para Windows (para bash) o PowerShell
</h3>

Git para Windows es opcional. Claude Code usa la [herramienta PowerShell](/docs/es/tools-reference#powershell-tool) cuando Git Bash está ausente, por lo que este error significa que ninguno de los dos shells fue encontrado.

**Si PowerShell falta de su PATH**, su ubicación predeterminada es `C:\Windows\System32\WindowsPowerShell\v1.0\`. Agregue ese directorio a su `PATH`, o instale [PowerShell 7](https://aka.ms/powershell), que proporciona `pwsh`.

**Para instalar Git para Windows en su lugar**, descárguelo desde [git-scm.com/downloads/win](https://git-scm.com/downloads/win). Durante la configuración, seleccione "Add to PATH." Reinicie su terminal después de instalar. Instalarlo habilita la herramienta Bash, útil cuando se trabaja con scripts y herramientas basadas en Bash.

**Si Git ya está instalado** pero Claude Code no puede encontrarlo, establezca la ruta en su [archivo settings.json](/docs/es/settings):

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_GIT_BASH_PATH": "C:\\Program Files\\Git\\bin\\bash.exe"
  }
}
```

Si su Git está instalado en otro lugar, encuentre la ruta ejecutando `where.exe git` en PowerShell y use la ruta `bin\bash.exe` de ese directorio.

**Si la ruta es correcta y el archivo existe** pero Claude Code aún lo reporta como no encontrado, el software de seguridad de punto final como AppLocker, políticas de restricción de software de Directiva de grupo, o agentes EDR pueden estar interfiriendo. En versiones anteriores a v2.1.116, Claude Code generaba un proceso hijo (`cmd.exe`) para verificar la ruta, que estas políticas pueden bloquear — una señal común es que `cmd.exe /c dir "C:\Program Files\Git\bin\bash.exe"` funciona cuando lo ejecuta directamente en PowerShell pero falla silenciosamente cuando lo inicia `claude.exe`.

Claude Code v2.1.116 y posterior verifican el sistema de archivos directamente, así que actualice primero. Si el error persiste en una versión actual, pida a su equipo de TI que agregue a la lista blanca `claude.exe` y los procesos que genera, incluidos `cmd.exe` y `bash.exe`, en su política de protección de punto final.

<h3 id="claude-code-does-not-support-32-bit-windows">
  Claude Code no admite Windows de 32 bits
</h3>

Windows incluye dos entradas de PowerShell en el menú Inicio: `Windows PowerShell` y `Windows PowerShell (x86)`. La entrada x86 se ejecuta como un proceso de 32 bits y desencadena este error incluso en una máquina de 64 bits. Para verificar en qué caso está, ejecute esto en la misma ventana que produjo el error:

```powershell theme={null}
[Environment]::Is64BitOperatingSystem
```

Si esto imprime `True`, su sistema operativo está bien. Cierre la ventana, abra `Windows PowerShell` sin el sufijo x86, y ejecute el comando de instalación nuevamente.

Si esto imprime `False`, está en una edición de Windows de 32 bits. Claude Code requiere un sistema operativo de 64 bits. Consulte los [requisitos del sistema](/docs/es/setup#system-requirements).

<h3 id="linux-musl-or-glibc-binary-mismatch">
  Desajuste binario musl o glibc de Linux
</h3>

Si ve errores sobre bibliotecas compartidas faltantes como `libstdc++.so.6` o `libgcc_s.so.1` después de la instalación, el instalador puede haber descargado la variante binaria incorrecta para su sistema.

```text theme={null}
Error loading shared library libstdc++.so.6: No such file or directory
```

Esto puede ocurrir en sistemas basados en glibc que tienen paquetes de compilación cruzada musl instalados, causando que el instalador detecte incorrectamente el sistema como musl.

**Soluciones:**

1. **Verifique qué libc usa su sistema**:
   ```bash theme={null}
   ldd --version 2>&1 | head -1
   ```
   La salida que menciona `GNU libc` o `GLIBC` significa glibc. La salida que menciona `musl` significa musl.

2. **Si está en glibc pero obtuvo el binario musl**, elimine la instalación y reinstale. También puede descargar manualmente el binario correcto usando el manifiesto en `https://downloads.claude.ai/claude-code-releases/{VERSION}/manifest.json`. Presente un [problema de GitHub](https://github.com/anthropics/claude-code/issues) con la salida de `ldd --version` y `ls /lib/libc.musl*`.

3. **Si realmente está en musl**, como Alpine Linux, instale los paquetes requeridos:
   ```bash theme={null}
   apk add libgcc libstdc++ ripgrep
   ```

<h3 id="illegal-instruction">
  `Illegal instruction`
</h3>

Si ejecutar `claude` o el instalador imprime `Illegal instruction`, el binario nativo usa instrucciones de CPU que su procesador no admite. Hay dos causas distintas.

**Desajuste de arquitectura.** El instalador descargó el binario incorrecto, por ejemplo x86 en un servidor ARM. Verifique con `uname -m` en macOS o Linux, o `$env:PROCESSOR_ARCHITECTURE` en PowerShell. Si el resultado no coincide con el binario que recibió, [presente un problema de GitHub](https://github.com/anthropics/claude-code/issues) con la salida.

**Conjunto de instrucciones AVX faltante.** Si su arquitectura es correcta pero aún ve `Illegal instruction`, su CPU probablemente carece de AVX u otra instrucción que requiere el binario. Esto afecta aproximadamente a procesadores Intel y AMD anteriores a 2013, y máquinas virtuales donde el hipervisor no pasa AVX al invitado.

En un VPS o VM, ejecute `grep -m1 -ow avx /proc/cpuinfo`; un resultado vacío significa que AVX no está disponible para el invitado.

No hay solución de binario nativo; siga el [problema #50384](https://github.com/anthropics/claude-code/issues/50384) para el estado, e incluya su modelo de CPU de `grep -m1 "model name" /proc/cpuinfo` en Linux o `sysctl -n machdep.cpu.brand_string` en macOS al reportar.

Los métodos de instalación alternativos descargan el mismo binario nativo y no resolverán ninguna de las causas.

<h3 id="dyld-cannot-load-on-macos">
  `dyld: cannot load` en macOS
</h3>

Si ve `dyld: cannot load`, `dyld: Symbol not found`, o `Abort trap: 6` durante la instalación, el binario es incompatible con su versión de macOS o hardware.

```text theme={null}
dyld: cannot load 'claude-2.1.42-darwin-x64' (load command 0x80000034 is unknown)
Abort trap: 6
```

Un error `Symbol not found` que hace referencia a `libicucore` también indica que su versión de macOS es más antigua que la que admite el binario:

```text theme={null}
dyld: Symbol not found: _ubrk_clone
  Referenced from: claude-darwin-x64 (which was built for Mac OS X 13.0)
  Expected in: /usr/lib/libicucore.A.dylib
```

**Soluciones:**

1. **Verifique su versión de macOS**: Claude Code requiere macOS 13.0 o posterior. Abra el menú Apple y seleccione Acerca de esta Mac para verificar su versión.

2. **Actualice macOS** si está en una versión anterior. El binario usa comandos de carga y bibliotecas del sistema que las versiones anteriores de macOS no admiten. Los métodos de instalación alternativos como Homebrew descargan el mismo binario y no resolverán este error.

<h3 id="exec-format-error-on-wsl1">
  `Exec format error` en WSL1
</h3>

Si ejecutar `claude` en WSL imprime `cannot execute binary file: Exec format error`, está en WSL1 y está experimentando una regresión binaria nativa conocida rastreada en el [problema #38788](https://github.com/anthropics/claude-code/issues/38788). Los encabezados del programa del binario cambiaron de una manera que el cargador de WSL1 no puede manejar.

La corrección más limpia es convertir su distribución a WSL2 desde PowerShell:

```powershell theme={null}
wsl --set-version <DistroName> 2
```

Si necesita permanecer en WSL1, invoque el binario a través del enlazador dinámico. Agregue esta función a `~/.bashrc` dentro de WSL, reemplazando la ruta si su directorio de inicio es diferente:

```bash theme={null}
claude() {
  /lib64/ld-linux-x86-64.so.2 "$(readlink -f "$HOME/.local/bin/claude")" "$@"
}
```

Luego ejecute `source ~/.bashrc` e intente `claude` nuevamente.

<h3 id="npm-install-errors-in-wsl">
  Errores de instalación de npm en WSL
</h3>

Estos problemas se aplican si instaló Claude Code con `npm install -g` dentro de WSL. Si usó el [instalador nativo](/docs/es/setup), omita esta sección.

**Problemas de detección de SO o plataforma.** Si npm reporta un desajuste de plataforma durante la instalación, WSL probablemente está recogiendo el `npm` de Windows. Ejecute `npm config set os linux` primero, luego instale con `npm install -g @anthropic-ai/claude-code --force`. No use `sudo`.

**`exec: node: not found` al ejecutar `claude`.** Su entorno WSL probablemente está usando la instalación de Node.js de Windows. Confirme con `which npm` y `which node`: las rutas que comienzan con `/mnt/c/` son binarios de Windows, mientras que las rutas de Linux comienzan con `/usr/`. Para corregir esto, instale Node a través del administrador de paquetes de su distribución de Linux o a través de [`nvm`](https://github.com/nvm-sh/nvm).

**Conflictos de versión de nvm.** Si tiene nvm instalado tanto en WSL como en Windows, cambiar versiones de Node en WSL puede romper porque WSL importa el PATH de Windows de forma predeterminada y el nvm de Windows toma prioridad. La causa más común es que nvm no está cargado en su shell. Agregue el cargador de nvm a `~/.bashrc` o `~/.zshrc`:

```bash theme={null}
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"
```

O cárguelo en su sesión actual:

```bash theme={null}
source ~/.nvm/nvm.sh
```

Si nvm está cargado pero las rutas de Windows aún toman prioridad, anteponga explícitamente su ruta de Node de Linux:

```bash theme={null}
export PATH="$HOME/.nvm/versions/node/$(node -v)/bin:$PATH"
```

<Warning>
  Evite deshabilitar la importación de PATH de Windows a través de `appendWindowsPath = false` ya que esto rompe la capacidad de llamar ejecutables de Windows desde WSL. De manera similar, evite desinstalar Node.js de Windows si lo usa para desarrollo de Windows.
</Warning>

<h3 id="permission-errors-during-installation">
  Errores de permisos durante la instalación
</h3>

Si el instalador nativo falla con errores de permisos, el directorio de destino puede no ser escribible. Consulte [Verifique permisos de directorio](#check-directory-permissions).

Si instaló previamente con npm y está experimentando errores de permisos específicos de npm, cambie al instalador nativo:

```bash theme={null}
curl -fsSL https://claude.ai/install.sh | bash
```

<h3 id="native-binary-not-found-after-npm-install">
  Binario nativo no encontrado después de la instalación de npm
</h3>

El paquete npm `@anthropic-ai/claude-code` obtiene el binario nativo a través de una dependencia opcional por plataforma como `@anthropic-ai/claude-code-darwin-arm64`. Si ejecutar `claude` después de instalar imprime `Could not find native binary package "@anthropic-ai/claude-code-<platform>"`, verifique las siguientes causas:

* **Las dependencias opcionales están deshabilitadas.** Elimine `--omit=optional` de su comando de instalación de npm, `--no-optional` de pnpm, o `--ignore-optional` de yarn, y verifique que `.npmrc` no establezca `optional=false`. Luego reinstale. El binario nativo se entrega solo como una dependencia opcional, por lo que no hay alternativa de JavaScript si se omite.
* **Plataforma no admitida.** Los binarios precompilados se publican para `darwin-arm64`, `darwin-x64`, `linux-x64`, `linux-arm64`, `linux-x64-musl`, `linux-arm64-musl`, `win32-x64`, y `win32-arm64`. Claude Code no envía un binario para otras plataformas; consulte los [requisitos del sistema](/docs/es/setup#system-requirements). En FreeBSD, el instalador reporta la plataforma como no admitida. Antes de v2.1.205, trataba FreeBSD como Linux y descargaba un binario que no podía ejecutarse.
* **El espejo npm corporativo carece de los paquetes de plataforma.** Asegúrese de que su registro refleje los ocho paquetes `@anthropic-ai/claude-code-*` de plataforma además del paquete meta.

Instalar con `--ignore-scripts` no desencadena este error. El paso de postinstalación que vincula el binario en su lugar se omite, por lo que Claude Code recurre a un contenedor que localiza e inicia el binario de plataforma en cada lanzamiento. Esto funciona pero se inicia más lentamente; reinstale con scripts habilitados para ejecución directa.

<h2 id="login-and-authentication">
  Inicio de sesión y autenticación
</h2>

Estas secciones abordan fallos de inicio de sesión, errores de OAuth y problemas de tokens.

<h3 id="reset-your-login">
  Reinicie su inicio de sesión
</h3>

Cuando el inicio de sesión falla y la causa no es obvia, una reautenticación limpia resuelve la mayoría de los casos:

1. Ejecute `/logout` para cerrar sesión completamente
2. Cierre Claude Code
3. Reinicie con `claude` y complete el proceso de autenticación nuevamente

Si el navegador no se abre automáticamente durante el inicio de sesión, presione `c` para copiar la URL de OAuth a su portapapeles, luego péguelo en un navegador manualmente. Esto también funciona cuando la URL se envuelve en varias líneas en una terminal estrecha o SSH y no se puede hacer clic directamente.

<h3 id="oauth-error-invalid-code">
  Error de OAuth: Código inválido
</h3>

Si ve `OAuth error: Invalid code. Please make sure the full code was copied`, el código de inicio de sesión expiró o fue truncado durante la copia y pegado.

**Soluciones:**

* Presione Intro para reintentar y complete el inicio de sesión rápidamente después de que se abra el navegador
* Escriba `c` para copiar la URL completa si el navegador no se abre automáticamente
* Si usa una sesión remota/SSH, el navegador puede abrirse en la máquina incorrecta. Copie la URL mostrada en la terminal y ábrala en su navegador local en su lugar.

<h3 id="403-forbidden-after-login">
  403 Forbidden después del inicio de sesión
</h3>

Si ve `API Error: 403 {"error":{"type":"forbidden","message":"Request not allowed"}}` después de iniciar sesión:

* **Usuarios de Claude Pro/Max**: verifique que su suscripción esté activa en [claude.ai/settings](https://claude.ai/settings)
* **Usuarios de Anthropic Console**: confirme que su cuenta tiene el rol "Claude Code" o "Developer". Los administradores asignan esto en Anthropic Console bajo Settings → Members.
* **Detrás de un proxy**: los proxies corporativos pueden interferir con las solicitudes de API. Consulte [configuración de red](/docs/es/network-config) para la configuración de proxy.

<h3 id="this-organization-has-been-disabled-with-an-active-subscription">
  Esta organización ha sido deshabilitada con una suscripción activa
</h3>

Si ve `API Error: 400 ... "This organization has been disabled"` a pesar de tener una suscripción activa de Claude, una variable de entorno `ANTHROPIC_API_KEY` está anulando su suscripción. Esto comúnmente ocurre cuando una clave API antigua de un empleador anterior o proyecto anterior aún está configurada en su perfil de shell.

Cuando `ANTHROPIC_API_KEY` está presente y lo ha aprobado, Claude Code usa esa clave en lugar de las credenciales de OAuth de su suscripción. En modo no interactivo con la bandera `-p`, la clave siempre se usa cuando está presente. Consulte [precedencia de autenticación](/docs/es/authentication#authentication-precedence) para el orden de resolución completo.

Para usar su suscripción en su lugar, desestablezca la variable de entorno y elimínela de su perfil de shell:

```bash theme={null}
unset ANTHROPIC_API_KEY
claude
```

Verifique `~/.zshrc`, `~/.bashrc`, o `~/.profile` para líneas `export ANTHROPIC_API_KEY=...` y elimínelas para hacer el cambio permanente. En Windows, verifique su perfil de PowerShell en `$PROFILE` y sus variables de entorno de usuario para `ANTHROPIC_API_KEY`. Ejecute `/status` dentro de Claude Code para confirmar qué método de autenticación está activo.

<h3 id="oauth-login-fails-in-wsl2-ssh-or-containers">
  El inicio de sesión de OAuth falla en WSL2, SSH o contenedores
</h3>

Cuando Claude Code se ejecuta en WSL2, en una máquina remota a través de SSH, o dentro de un contenedor, el navegador generalmente se abre en un host diferente y su redirección no puede alcanzar el servidor de devolución de llamada local de Claude Code. Después de que inicie sesión, el navegador muestra un código de inicio de sesión en lugar de redirigirse automáticamente. Pegue ese código en la terminal en el indicador `Paste code here if prompted` para completar el inicio de sesión.

Si el navegador no se abre en absoluto desde WSL2, establezca la variable de entorno `BROWSER` en la ruta de su navegador de Windows:

```bash theme={null}
export BROWSER="/mnt/c/Program Files/Google/Chrome/Application/chrome.exe"
claude
```

Alternativamente, presione `c` en el indicador de inicio de sesión interactivo para copiar la URL de OAuth, o copie la URL que `claude auth login` imprime, y ábrala en un navegador en su máquina local.

Si pegar el código en el indicador interactivo no hace nada, el enlace de pegado de su terminal probablemente no está llegando al campo de entrada. Intente el atajo de pegado alternativo de su terminal, a menudo clic derecho o Shift+Insert en Windows Terminal, o use `claude auth login` en su lugar, que lee el código pegado desde la entrada estándar:

```bash theme={null}
claude auth login
```

Esta alternativa también se aplica en Windows nativo o cualquier terminal donde pegar en el indicador interactivo falla.

<h3 id="not-logged-in-or-token-expired">
  No ha iniciado sesión o el token ha expirado
</h3>

Si Claude Code le solicita que inicie sesión nuevamente después de una sesión, su token de OAuth puede haber expirado.

Ejecute `/login` para reautenticarse. Si esto ocurre frecuentemente, verifique que su reloj del sistema sea preciso, ya que la validación de tokens depende de marcas de tiempo correctas.

En macOS, el inicio de sesión también puede fallar cuando Keychain está bloqueado o su contraseña está fuera de sincronización con su contraseña de cuenta, lo que impide que Claude Code guarde credenciales. Ejecute `claude doctor` para verificar el acceso a Keychain. Para desbloquear Keychain manualmente, ejecute `security unlock-keychain ~/Library/Keychains/login.keychain-db`. Si desbloquear no ayuda, abra Keychain Access, seleccione el keychain `login`, y elija Edit > Change Password for Keychain "login" para resincronizarlo con su contraseña de cuenta.

<h3 id="bedrock-agent-platform-or-foundry-credentials-not-loading">
  Las credenciales de Bedrock, Agent Platform o Foundry no se cargan
</h3>

Si configuró Claude Code para usar un proveedor en la nube y ve `Could not load credentials from any providers` en Amazon Bedrock, `Could not load the default credentials` en Google Cloud's Agent Platform, o `ChainedTokenCredential authentication failed` en Microsoft Foundry, su CLI del proveedor en la nube probablemente no está autenticado en el shell actual.

Para Amazon Bedrock, confirme que sus credenciales de AWS son válidas:

```bash theme={null}
aws sts get-caller-identity
```

Para Google Cloud's Agent Platform, confirme que `ANTHROPIC_VERTEX_PROJECT_ID` y `CLOUD_ML_REGION` están configurados en su shell, luego establezca credenciales predeterminadas de aplicación:

```bash theme={null}
gcloud auth application-default login
```

Para Microsoft Foundry, confirme que `ANTHROPIC_FOUNDRY_API_KEY` está configurado, o inicie sesión con la CLI de Azure para que la cadena de credenciales predeterminada pueda encontrar su cuenta:

```bash theme={null}
az login
```

Si las credenciales funcionan en su terminal pero no en la extensión de VS Code o JetBrains, el proceso del IDE probablemente no heredó su entorno de shell. Establezca las variables de entorno del proveedor en la configuración propia del IDE, o inicie el IDE desde una terminal donde ya estén exportadas.

Consulte [Amazon Bedrock](/docs/es/amazon-bedrock), [Google Cloud's Agent Platform](/docs/es/google-vertex-ai), o [Microsoft Foundry](/docs/es/microsoft-foundry) para la configuración completa del proveedor.

<h2 id="still-stuck">
  Aún atrapado
</h2>

Si ninguno de los anteriores resuelve su problema:

1. Verifique el [repositorio de GitHub](https://github.com/anthropics/claude-code/issues) para problemas conocidos, o abra uno nuevo con su sistema operativo, el comando de instalación que ejecutó, y la salida de error completa
2. Si `claude --version` funciona pero algo más está mal, ejecute `claude doctor` para un informe de diagnóstico automatizado
3. Si puede iniciar una sesión, use `/feedback` dentro de Claude Code para reportar el problema
