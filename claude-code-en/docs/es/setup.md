> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configuración avanzada

> Requisitos del sistema, instalación específica de plataforma, gestión de versiones y desinstalación para Claude Code.

Esta página cubre requisitos del sistema, detalles de instalación específicos de plataforma, actualizaciones y desinstalación. Para un recorrido guiado de su primera sesión, consulte el [inicio rápido](/docs/es/quickstart). Si nunca ha utilizado una terminal antes, consulte la [guía de terminal](/docs/es/terminal-guide).

<h2 id="system-requirements">
  Requisitos del sistema
</h2>

Claude Code se ejecuta en las siguientes plataformas y configuraciones:

* **Sistema operativo**:
  * macOS 13.0+
  * Windows 10 1809+ o Windows Server 2019+
  * Ubuntu 20.04+
  * Debian 10+
  * Alpine Linux 3.19+
* **Hardware**: 4 GB+ de RAM, procesador x64 o ARM64
* **Red**: se requiere conexión a Internet. Consulte [configuración de red](/docs/es/network-config#network-access-requirements).
* **Shell**: Bash, Zsh, PowerShell o CMD.
* **Ubicación**: [países compatibles con Anthropic](https://www.anthropic.com/supported-countries)

<h3 id="additional-dependencies">
  Dependencias adicionales
</h3>

* **ripgrep**: generalmente incluido con Claude Code. Si la búsqueda falla, consulte [solución de problemas de búsqueda](/docs/es/troubleshooting#search-and-discovery-issues).

<h2 id="install-claude-code">
  Instalar Claude Code
</h2>

<Tip>
  ¿Prefiere una interfaz gráfica? La [aplicación de escritorio](/docs/es/desktop-quickstart) le permite usar Claude Code sin la terminal. Descárguela para [macOS](https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code\&utm_medium=docs), [Windows](https://claude.com/download?utm_source=claude_code\&utm_medium=docs) o [Linux](/docs/es/desktop-linux).

  ¿Nuevo en la terminal? Consulte la [guía de terminal](/docs/es/terminal-guide) para obtener instrucciones paso a paso.
</Tip>

To install Claude Code, use one of the following methods:

<Tabs>
  <Tab title="Native Install (Recommended)">
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

    If you see `The token '&&' is not a valid statement separator`, you're in PowerShell, not CMD. If you see `'irm' is not recognized as an internal or external command`, you're in CMD, not PowerShell. Your prompt shows `PS C:\` when you're in PowerShell and `C:\` without the `PS` when you're in CMD.

    If the install command fails with `syntax error near unexpected token '<'`, a `403`, or another curl error, see [Troubleshoot installation](/docs/en/troubleshoot-install#find-your-error) to match the error to a fix and for alternative install methods.

    [Git for Windows](https://git-scm.com/downloads/win) is recommended on native Windows so Claude Code can use the Bash tool. If Git for Windows is not installed, Claude Code uses PowerShell as the shell tool instead. WSL setups do not need Git for Windows.

    <Info>
      Native installations automatically update in the background to keep you on the latest version.
    </Info>
  </Tab>

  <Tab title="Homebrew">
    ```bash theme={null}
    brew install --cask claude-code
    ```

    Homebrew offers two casks. `claude-code` tracks the stable release channel, which is typically about a week behind and skips releases with major regressions. `claude-code@latest` tracks the latest channel and receives new versions as soon as they ship.

    <Info>
      Homebrew installations do not auto-update. Run `brew upgrade claude-code` or `brew upgrade claude-code@latest`, depending on which cask you installed, to get the latest features and security fixes.
    </Info>
  </Tab>

  <Tab title="WinGet">
    ```powershell theme={null}
    winget install Anthropic.ClaudeCode
    ```

    <Info>
      WinGet installations do not auto-update. Run `winget upgrade Anthropic.ClaudeCode` periodically to get the latest features and security fixes.
    </Info>
  </Tab>
</Tabs>

You can also install with [apt, dnf, or apk](/docs/en/setup#install-with-linux-package-managers) on Debian, Fedora, RHEL, and Alpine.

Después de que se complete la instalación, abra una terminal en el proyecto en el que desea trabajar e inicie Claude Code:

```bash theme={null}
claude
```

Si encuentra algún problema durante la instalación, consulte [Solucionar problemas de instalación e inicio de sesión](/docs/es/troubleshoot-install).

<h3 id="set-up-on-windows">
  Configurar en Windows
</h3>

Puede ejecutar Claude Code de forma nativa en Windows o dentro de WSL. Elija según dónde se encuentren sus proyectos y qué características necesite:

| Opción         | Requiere                                                                  | [Sandboxing](/docs/es/sandboxing) | Cuándo usar                                                         |
| -------------- | ------------------------------------------------------------------------- | ---------------------------- | ------------------------------------------------------------------- |
| Windows nativo | Ninguno; [Git for Windows](https://git-scm.com/downloads/win) es opcional | No compatible                | Proyectos y herramientas nativas de Windows                         |
| WSL 2          | WSL 2 habilitado                                                          | Compatible                   | Cadenas de herramientas de Linux o ejecución de comandos en sandbox |
| WSL 1          | WSL 1 habilitado                                                          | No compatible                | Si WSL 2 no está disponible                                         |

**Opción 1: Windows nativo**

Ejecute el comando de instalación desde PowerShell o CMD. No necesita ejecutar como Administrador. Instalar [Git for Windows](https://git-scm.com/downloads/win) es opcional. Habilita la [herramienta Bash](/docs/es/tools-reference#bash-tool-behavior) proporcionando Git Bash.

Ya sea que instale desde PowerShell o CMD solo afecta qué comando de instalación ejecuta. Su indicador muestra `PS C:\Users\YourName>` en PowerShell y `C:\Users\YourName>` sin el `PS` en CMD. Si es nuevo en la terminal, la [guía de terminal](/docs/es/terminal-guide#windows) le guía a través de cada paso.

Después de la instalación, inicie `claude` desde cualquier terminal.

* **Sin Git for Windows**, Claude Code ejecuta comandos de shell a través de la [herramienta PowerShell](/docs/es/tools-reference#powershell-tool).
* **Con Git for Windows**, Claude Code utiliza Git Bash para la [herramienta Bash](/docs/es/tools-reference#bash-tool-behavior). Si Claude Code no puede encontrar Git Bash, establezca la ruta en su [archivo settings.json](/docs/es/settings):

  ```json theme={null}
  {
    "env": {
      "CLAUDE_CODE_GIT_BASH_PATH": "C:\\Program Files\\Git\\bin\\bash.exe"
    }
  }
  ```

Cuando Git for Windows está instalado, la herramienta PowerShell se está implementando progresivamente como una opción adicional junto a Bash. Establezca `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` para participar o `0` para no participar. Consulte [herramienta PowerShell](/docs/es/tools-reference#powershell-tool) para configuración y limitaciones.

**Opción 2: WSL**

Abra su distribución de WSL y ejecute el instalador de Linux desde las [instrucciones de instalación](#install-claude-code) anteriores. Instala e inicia `claude` dentro del terminal de WSL, no desde PowerShell o CMD.

<h3 id="alpine-linux-and-musl-based-distributions">
  Alpine Linux y distribuciones basadas en musl
</h3>

El instalador nativo en Alpine y otras distribuciones basadas en musl/uClibc requiere `libgcc`, `libstdc++` y `ripgrep`. Instale estos usando el gestor de paquetes de su distribución y luego establezca `USE_BUILTIN_RIPGREP=0`.

Este ejemplo instala los paquetes requeridos en Alpine:

```bash theme={null}
apk add libgcc libstdc++ ripgrep
```

Luego establezca `USE_BUILTIN_RIPGREP` en `0` en su archivo [`settings.json`](/docs/es/settings#available-settings):

```json theme={null}
{
  "env": {
    "USE_BUILTIN_RIPGREP": "0"
  }
}
```

<h2 id="verify-your-installation">
  Verificar su instalación
</h2>

Después de instalar, confirme que Claude Code está funcionando:

```bash theme={null}
claude --version
```

Si esto falla con `command not found` u otro error, consulte [Solucionar problemas de instalación e inicio de sesión](/docs/es/troubleshoot-install).

Para una verificación más detallada de su instalación y configuración, ejecute [`claude doctor`](/docs/es/troubleshooting#get-more-help):

```bash theme={null}
claude doctor
```

<h2 id="authenticate">
  Autenticar
</h2>

Claude Code requiere una cuenta Pro, Max, Team, Enterprise o Console. El plan gratuito de Claude.ai no incluye acceso a Claude Code. También puede usar Claude Code con un proveedor de API de terceros como [Amazon Bedrock](/docs/es/amazon-bedrock), [Google Cloud's Agent Platform](/docs/es/google-vertex-ai) o [Microsoft Foundry](/docs/es/microsoft-foundry).

Después de instalar, inicie sesión ejecutando `claude` y siguiendo las indicaciones del navegador. Consulte [Autenticación](/docs/es/authentication) para todos los tipos de cuenta y opciones de configuración de equipo.

<h2 id="update-claude-code">
  Actualizar Claude Code
</h2>

Las instalaciones nativas se actualizan automáticamente en segundo plano. Puede [configurar el canal de lanzamiento](#configure-release-channel) para controlar si recibe actualizaciones inmediatamente o en un cronograma estable retrasado, o [deshabilitar las actualizaciones automáticas](#disable-auto-updates) completamente. Las instalaciones de Homebrew, WinGet y [gestor de paquetes de Linux](#install-with-linux-package-managers) requieren actualizaciones manuales de forma predeterminada.

<h3 id="auto-updates">
  Actualizaciones automáticas
</h3>

Claude Code busca actualizaciones al iniciar y periódicamente mientras se ejecuta. Las actualizaciones se descargan e instalan en segundo plano y luego surten efecto la próxima vez que inicie Claude Code.

Ejecute `claude doctor` para ver el resultado del intento de actualización más reciente.

En macOS y Linux, el instalador nativo gestiona el lanzador en `~/.local/bin/claude` como un enlace simbólico en `~/.local/share/claude/versions/`. Si reemplaza ese lanzador con su propio script o enlace simbólico, la actualización automática y `claude update` lo dejan en su lugar: las nuevas versiones aún se instalan en el directorio `versions/`, y su lanzador decide qué versión se ejecuta. Antes de v2.1.207, el actualizador automático reemplazaba un lanzador personalizado en esa ruta con su propio enlace simbólico en cada actualización.

Con un lanzador personalizado, Claude Code también mantiene todas las versiones instaladas en el disco porque no puede determinar qué versión necesita el lanzador. `claude doctor` informa de un lanzador que el instalador nativo no creó.

Para permitir que Claude Code gestione el lanzador nuevamente, elimine `~/.local/bin/claude` y ejecute `claude update`.

Si una instalación global de npm no puede actualizarse automáticamente porque el directorio global de npm no es escribible, Claude Code muestra un aviso único al iniciar, y `claude doctor` enumera las correcciones disponibles. Consulte [errores de permisos durante la instalación](/docs/es/troubleshoot-install#permission-errors-during-installation) para obtener detalles.

<Note>
  Las instalaciones de Homebrew, WinGet, apt, dnf y apk no se actualizan automáticamente de forma predeterminada; consulte a continuación para optar por Homebrew y WinGet. Para actualizar Homebrew manualmente, ejecute `brew upgrade claude-code` o `brew upgrade claude-code@latest`, dependiendo de qué cask instaló. Para WinGet, ejecute `winget upgrade Anthropic.ClaudeCode`. Para gestores de paquetes de Linux, consulte los comandos de actualización en [Instalar con gestores de paquetes de Linux](#install-with-linux-package-managers).

  Para que Claude Code ejecute el comando de actualización por usted en Homebrew o WinGet, establezca [`CLAUDE_CODE_PACKAGE_MANAGER_AUTO_UPDATE`](/docs/es/env-vars) en `1`. Claude Code luego ejecuta la actualización en segundo plano cuando una nueva versión está disponible y muestra un aviso de reinicio en caso de éxito. La actualización se dirige solo al paquete Claude Code y no afecta otro software que tenga instalado.

  En WinGet, la actualización puede fallar mientras Claude Code se está ejecutando porque Windows bloquea el ejecutable. En ese caso, Claude Code muestra el comando manual en su lugar. apt, dnf y apk continúan requiriendo una actualización manual porque esos comandos necesitan privilegios elevados.

  **Problema conocido:** Claude Code puede notificarle sobre actualizaciones antes de que la nueva versión esté disponible en estos gestores de paquetes. Si una actualización falla, espere e intente más tarde.

  Homebrew mantiene versiones antiguas en el disco después de las actualizaciones. Ejecute `brew cleanup` periódicamente para recuperar espacio en disco.
</Note>

<h3 id="configure-release-channel">
  Configurar canal de lanzamiento
</h3>

Controle qué canal de lanzamiento sigue Claude Code para actualizaciones automáticas y `claude update` con la configuración `autoUpdatesChannel`:

* `"latest"`, el predeterminado: reciba nuevas características tan pronto como se lancen
* `"stable"`: use una versión que típicamente tiene aproximadamente una semana de antigüedad, omitiendo lanzamientos con regresiones importantes

Configure esto a través de `/config` → **Canal de actualización automática**, o agréguelo a su [archivo settings.json](/docs/es/settings):

```json theme={null}
{
  "autoUpdatesChannel": "stable"
}
```

Para implementaciones empresariales, puede aplicar un canal de lanzamiento consistente en toda su organización usando [configuración administrada](/docs/es/permissions#managed-settings).

Las instalaciones de Homebrew eligen un canal por nombre de cask en lugar de esta configuración: `claude-code` rastrea estable y `claude-code@latest` rastrea latest.

<h3 id="pin-a-minimum-version">
  Fijar una versión mínima
</h3>

La configuración `minimumVersion` establece un piso. Las actualizaciones automáticas en segundo plano y `claude update` se niegan a instalar cualquier versión por debajo de este valor, por lo que cambiar al canal `"stable"` no lo degrada si ya está en una compilación `"latest"` más nueva.

Cambiar de `"latest"` a `"stable"` a través de `/config` le solicita que permanezca en la versión actual o permita la degradación. Elegir permanecer establece `minimumVersion` en esa versión. Cambiar de nuevo a `"latest"` lo borra.

Agréguelo a su [archivo settings.json](/docs/es/settings) para fijar un piso explícitamente:

```json theme={null}
{
  "autoUpdatesChannel": "stable",
  "minimumVersion": "2.1.100"
}
```

En [configuración administrada](/docs/es/permissions#managed-settings), esto aplica un mínimo en toda la organización que la configuración de usuario y proyecto no puede anular.

El pin `minimumVersion` solo restringe las actualizaciones. Para hacer que Claude Code se niegue a iniciar fuera de un rango de versión, use la configuración administrada `requiredMinimumVersion` y `requiredMaximumVersion` en su lugar. Las actualizaciones también respetan el techo `requiredMaximumVersion`. Consulte [configuración disponible](/docs/es/settings#available-settings).

<h3 id="disable-auto-updates">
  Deshabilitar actualizaciones automáticas
</h3>

Establezca `DISABLE_AUTOUPDATER` en `"1"` en la clave `env` de su archivo [`settings.json`](/docs/es/settings#available-settings):

```json theme={null}
{
  "env": {
    "DISABLE_AUTOUPDATER": "1"
  }
}
```

`DISABLE_AUTOUPDATER` solo detiene la verificación en segundo plano; `claude update` e `claude install` aún funcionan. Para bloquear todas las rutas de actualización, incluidas las actualizaciones manuales, establezca [`DISABLE_UPDATES`](/docs/es/env-vars) en su lugar. Úselo cuando distribuya Claude Code a través de sus propios canales y necesite que los usuarios permanezcan en la versión que proporciona.

<h3 id="update-manually">
  Actualizar manualmente
</h3>

Para aplicar una actualización inmediatamente sin esperar la próxima verificación en segundo plano, ejecute:

```bash theme={null}
claude update
```

<h2 id="advanced-installation-options">
  Opciones de instalación avanzadas
</h2>

Estas opciones son para fijación de versiones, gestores de paquetes de Linux, npm y verificación de integridad binaria.

<h3 id="install-a-specific-version">
  Instalar una versión específica
</h3>

El instalador nativo acepta un número de versión específico o un canal de lanzamiento (`latest` o `stable`). El canal que elija en el momento de la instalación se convierte en su predeterminado para actualizaciones automáticas. Consulte [configurar canal de lanzamiento](#configure-release-channel) para más información.

Para instalar la versión más reciente (predeterminada):

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    irm https://claude.ai/install.ps1 | iex
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
    ```
  </Tab>
</Tabs>

Para instalar la versión estable:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash -s stable
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    & ([scriptblock]::Create((irm https://claude.ai/install.ps1))) stable
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd stable && del install.cmd
    ```
  </Tab>
</Tabs>

Para instalar un número de versión específico:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash -s 2.1.89
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    & ([scriptblock]::Create((irm https://claude.ai/install.ps1))) 2.1.89
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd 2.1.89 && del install.cmd
    ```
  </Tab>
</Tabs>

<h3 id="install-with-linux-package-managers">
  Instalar con gestores de paquetes de Linux
</h3>

Claude Code publica repositorios apt, dnf y apk firmados. Cada repositorio ofrece dos canales: `stable` sirve una versión que típicamente tiene aproximadamente una semana de antigüedad, omitiendo lanzamientos con regresiones mayores, y `latest` sirve cada lanzamiento tan pronto como se envía. Los comandos a continuación configuran el canal `stable`, que se ajusta a la mayoría de usuarios; cada pestaña también muestra la URL del repositorio `latest`. Las instalaciones del gestor de paquetes no se actualizan automáticamente a través de Claude Code; las actualizaciones llegan a través de su flujo de trabajo de actualización del sistema normal.

Todos los repositorios están firmados con la [clave de firma de lanzamiento de Claude Code](#binary-integrity-and-code-signing). Antes de confiar en la clave, verifíquela como se describe en cada pestaña.

<Tabs>
  <Tab title="apt">
    Para Debian y Ubuntu. Los comandos de instalación a continuación descargan la clave de firma con `curl`, que las instalaciones nuevas de Debian y Ubuntu pueden no incluir. Si la descarga falla con `sudo: curl: command not found`, instale curl primero:

    ```bash theme={null}
    sudo apt install curl
    ```

    Los siguientes comandos configuran el canal `stable`:

    ```bash theme={null}
    sudo install -d -m 0755 /etc/apt/keyrings
    sudo curl -fsSL https://downloads.claude.ai/keys/claude-code.asc \
      -o /etc/apt/keyrings/claude-code.asc
    echo "deb [signed-by=/etc/apt/keyrings/claude-code.asc] https://downloads.claude.ai/claude-code/apt/stable stable main" \
      | sudo tee /etc/apt/sources.list.d/claude-code.list
    sudo apt update
    sudo apt install claude-code
    ```

    Para usar el canal `latest` en su lugar, tanto la ruta de URL como el nombre de la suite cambian. Use esta línea `deb`:

    ```bash theme={null}
    echo "deb [signed-by=/etc/apt/keyrings/claude-code.asc] https://downloads.claude.ai/claude-code/apt/latest latest main" \
      | sudo tee /etc/apt/sources.list.d/claude-code.list
    ```

    Verifique la huella digital de la clave GPG antes de confiar en ella: `gpg --show-keys /etc/apt/keyrings/claude-code.asc` debe reportar `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE`.

    Para actualizar más tarde, ejecute `sudo apt update && sudo apt upgrade claude-code`.
  </Tab>

  <Tab title="dnf">
    Para Fedora y RHEL. Los siguientes comandos configuran el canal `stable`:

    ```bash theme={null}
    sudo tee /etc/yum.repos.d/claude-code.repo <<'EOF'
    [claude-code]
    name=Claude Code
    baseurl=https://downloads.claude.ai/claude-code/rpm/stable
    enabled=1
    gpgcheck=1
    gpgkey=https://downloads.claude.ai/keys/claude-code.asc
    EOF
    sudo dnf install claude-code
    ```

    Para usar el canal `latest` en su lugar, establezca `baseurl` en el repositorio `latest`:

    ```ini theme={null}
    baseurl=https://downloads.claude.ai/claude-code/rpm/latest
    ```

    dnf descarga la clave en la primera instalación y le solicita que confirme la huella digital. Verifique que coincida con `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE` antes de aceptar.

    Para actualizar más tarde, ejecute `sudo dnf upgrade claude-code`.
  </Tab>

  <Tab title="apk">
    Para Alpine Linux. Los siguientes comandos configuran el canal `stable`:

    ```sh theme={null}
    wget -O /etc/apk/keys/claude-code.rsa.pub \
      https://downloads.claude.ai/keys/claude-code.rsa.pub
    echo "https://downloads.claude.ai/claude-code/apk/stable" >> /etc/apk/repositories
    apk add claude-code
    ```

    Para cambiar al canal `latest`, elimine la línea del repositorio `stable` y agregue el repositorio `latest`:

    ```sh theme={null}
    sed -i '\|downloads.claude.ai/claude-code/apk/stable|d' /etc/apk/repositories
    echo "https://downloads.claude.ai/claude-code/apk/latest" >> /etc/apk/repositories
    ```

    Verifique la clave descargada con `sha256sum /etc/apk/keys/claude-code.rsa.pub`, que debe reportar `395759c1f7449ef4cdef305a42e820f3c766d6090d142634ebdb049f113168b6`.

    Para actualizar más tarde, ejecute `apk update && apk upgrade claude-code`.
  </Tab>
</Tabs>

<h3 id="install-with-npm">
  Instalar con npm
</h3>

También puede instalar Claude Code como un paquete npm global. A partir de v2.1.198, el paquete npm requiere [Node.js 22 o posterior](https://nodejs.org/en/download). En una versión anterior de Node.js, npm imprime una advertencia `EBADENGINE` durante la instalación en lugar de fallar; la instalación se completa y `claude` aún se ejecuta, ya que el paquete descarga un binario nativo que no utiliza su Node.js en tiempo de ejecución.

```bash theme={null}
npm install -g @anthropic-ai/claude-code
```

El paquete npm instala el mismo binario nativo que el instalador independiente. npm extrae el binario a través de una dependencia opcional por plataforma como `@anthropic-ai/claude-code-darwin-arm64`, y un paso postinstall lo vincula en su lugar. El binario `claude` instalado no invoca Node en sí mismo.

Las plataformas de instalación npm compatibles son `darwin-arm64`, `darwin-x64`, `linux-x64`, `linux-arm64`, `linux-x64-musl`, `linux-arm64-musl`, `win32-x64` y `win32-arm64`. Su gestor de paquetes debe permitir dependencias opcionales. Consulte [solución de problemas](/docs/es/troubleshoot-install#native-binary-not-found-after-npm-install) si falta el binario después de la instalación.

Para actualizar una instalación de npm, ejecute `npm install -g @anthropic-ai/claude-code@latest`. Evite `npm update -g`, que respeta el rango semver de la instalación original y puede no llevarlo a la versión más reciente.

<Warning>
  NO use `sudo npm install -g` ya que esto puede causar problemas de permisos y riesgos de seguridad. Si encuentra errores de permisos, consulte [solución de problemas de errores de permisos](/docs/es/troubleshoot-install#permission-errors-during-installation).
</Warning>

<h3 id="binary-integrity-and-code-signing">
  Integridad binaria y firma de código
</h3>

Cada lanzamiento publica un `manifest.json` que contiene sumas de verificación SHA256 para cada binario de plataforma. El manifiesto está firmado con una clave GPG de Anthropic, por lo que verificar la firma en el manifiesto verifica transitivamente cada binario que enumera.

<h4 id="verify-the-manifest-signature">
  Verificar la firma del manifiesto
</h4>

Los pasos 1-3 requieren un shell POSIX con `gpg` y `curl`. En Windows, ejecútelos en Git Bash o WSL. El paso 4 incluye una opción de PowerShell.

<Steps>
  <Step title="Descargar e importar la clave pública">
    La clave de firma de lanzamiento se publica en una URL fija.

    ```bash theme={null}
    curl -fsSL https://downloads.claude.ai/keys/claude-code.asc | gpg --import
    ```

    Muestre la huella digital de la clave importada.

    ```bash theme={null}
    gpg --fingerprint security@anthropic.com
    ```

    Confirme que la salida incluye esta huella digital:

    ```text theme={null}
    31DD DE24 DDFA B679 F42D  7BD2 BAA9 29FF 1A7E CACE
    ```
  </Step>

  <Step title="Descargar el manifiesto y la firma">
    Establezca `VERSION` en el lanzamiento que desea verificar.

    ```bash theme={null}
    REPO=https://downloads.claude.ai/claude-code-releases
    VERSION=2.1.89
    curl -fsSLO "$REPO/$VERSION/manifest.json"
    curl -fsSLO "$REPO/$VERSION/manifest.json.sig"
    ```
  </Step>

  <Step title="Verificar la firma">
    Verifique la firma separada contra el manifiesto.

    ```bash theme={null}
    gpg --verify manifest.json.sig manifest.json
    ```

    Un resultado válido reporta `Good signature from "Anthropic Claude Code Release Signing <security@anthropic.com>"`.

    `gpg` también imprime `WARNING: This key is not certified with a trusted signature!` para cualquier clave recién importada. Esto es esperado. La línea `Good signature` confirma que la verificación criptográfica pasó. La comparación de huella digital en el Paso 1 confirma que la clave en sí es auténtica.
  </Step>

  <Step title="Verificar el binario contra el manifiesto">
    Compare la suma de verificación SHA256 del binario con el valor listado bajo `platforms.<platform>.checksum` en `manifest.json`. Los comandos a continuación asumen un binario `claude` en el directorio actual. Para verificar un binario nativo instalado en su lugar, ejecute el comando contra `~/.local/share/claude/versions/VERSION`, reemplazando VERSION con el lanzamiento que estableció en el Paso 2.

    <Tabs>
      <Tab title="Linux">
        ```bash theme={null}
        sha256sum claude
        ```
      </Tab>

      <Tab title="macOS">
        ```bash theme={null}
        shasum -a 256 claude
        ```
      </Tab>

      <Tab title="Windows PowerShell">
        ```powershell theme={null}
        (Get-FileHash claude.exe -Algorithm SHA256).Hash.ToLower()
        ```
      </Tab>
    </Tabs>
  </Step>
</Steps>

<Note>
  Las firmas de manifiesto están disponibles para lanzamientos desde `2.1.89` en adelante. Los lanzamientos anteriores publican sumas de verificación en `manifest.json` sin una firma separada.
</Note>

<h4 id="platform-code-signatures">
  Firmas de código de plataforma
</h4>

Además del manifiesto firmado, los binarios individuales llevan firmas de código nativas de plataforma donde se admiten.

* **macOS**: firmado por "Anthropic PBC" y notarizado por Apple. Verifique con `codesign --verify --verbose ./claude`.
* **Windows**: firmado por "Anthropic, PBC". Verifique con `Get-AuthenticodeSignature .\claude.exe`.
* **Linux**: los binarios no están firmados individualmente con código. Si descarga directamente del bucket `claude-code-releases` o usa el instalador nativo, verifique la integridad con la firma de manifiesto anterior. Si instala con [apt, dnf o apk](#install-with-linux-package-managers), su gestor de paquetes verifica las firmas automáticamente usando la clave de firma del repositorio.

<h2 id="uninstall-claude-code">
  Desinstalar Claude Code
</h2>

Para eliminar Claude Code, siga las instrucciones para su método de instalación. Si `claude` aún se ejecuta después, probablemente tenga una segunda instalación o un alias de shell residual de un instalador anterior. Consulte [Verificar instalaciones conflictivas](/docs/es/troubleshoot-install#check-for-conflicting-installations) para encontrarlo y eliminarlo.

<h3 id="native-installation">
  Instalación nativa
</h3>

Elimine el binario de Claude Code y los archivos de versión:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    rm -f ~/.local/bin/claude
    rm -rf ~/.local/share/claude
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    Remove-Item -Path "$env:USERPROFILE\.local\bin\claude.exe" -Force
    Remove-Item -Path "$env:USERPROFILE\.local\share\claude" -Recurse -Force
    ```
  </Tab>
</Tabs>

<h3 id="homebrew-installation">
  Instalación de Homebrew
</h3>

Elimine el cask de Homebrew que instaló. Si instaló el cask estable:

```bash theme={null}
brew uninstall --cask claude-code
```

Si instaló el cask latest:

```bash theme={null}
brew uninstall --cask claude-code@latest
```

<h3 id="winget-installation">
  Instalación de WinGet
</h3>

Elimine el paquete de WinGet:

```powershell theme={null}
winget uninstall Anthropic.ClaudeCode
```

<h3 id="apt-/-dnf-/-apk">
  apt / dnf / apk
</h3>

Elimine el paquete y la configuración del repositorio:

<Tabs>
  <Tab title="apt">
    ```bash theme={null}
    sudo apt remove claude-code
    sudo rm /etc/apt/sources.list.d/claude-code.list /etc/apt/keyrings/claude-code.asc
    ```
  </Tab>

  <Tab title="dnf">
    ```bash theme={null}
    sudo dnf remove claude-code
    sudo rm /etc/yum.repos.d/claude-code.repo
    ```
  </Tab>

  <Tab title="apk">
    ```sh theme={null}
    apk del claude-code
    sed -i '\|downloads.claude.ai/claude-code/apk|d' /etc/apk/repositories
    rm /etc/apk/keys/claude-code.rsa.pub
    ```
  </Tab>
</Tabs>

<h3 id="npm">
  npm
</h3>

Elimine el paquete npm global:

```bash theme={null}
npm uninstall -g @anthropic-ai/claude-code
```

<h3 id="remove-configuration-files">
  Eliminar archivos de configuración
</h3>

<Warning>
  Eliminar archivos de configuración eliminará toda su configuración, herramientas permitidas, configuraciones de servidor MCP e historial de sesiones.
</Warning>

La extensión de VS Code, el plugin de JetBrains y la aplicación de escritorio también escriben en `~/.claude/`. Si alguno de ellos aún está instalado, el directorio se recrea la próxima vez que se ejecuta. Para eliminar Claude Code completamente, desinstale la [extensión de VS Code](/docs/es/vs-code#uninstall-the-extension), el plugin de JetBrains y la aplicación de escritorio antes de eliminar estos archivos.

Para eliminar la configuración y datos en caché de Claude Code:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    # Eliminar configuración de usuario y estado
    rm -rf ~/.claude
    rm ~/.claude.json

    # Eliminar configuración específica del proyecto (ejecutar desde su directorio de proyecto)
    rm -rf .claude
    rm -f .mcp.json
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    # Eliminar configuración de usuario y estado
    Remove-Item -Path "$env:USERPROFILE\.claude" -Recurse -Force
    Remove-Item -Path "$env:USERPROFILE\.claude.json" -Force

    # Eliminar configuración específica del proyecto (ejecutar desde su directorio de proyecto)
    Remove-Item -Path ".claude" -Recurse -Force
    Remove-Item -Path ".mcp.json" -Force
    ```
  </Tab>
</Tabs>
