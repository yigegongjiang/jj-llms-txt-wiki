> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Desktop en Linux (beta)

> Instala y actualiza la aplicación de escritorio Claude en Ubuntu y Debian

<Note>
  La compatibilidad con Linux para la aplicación de escritorio Claude está en beta. Las pestañas Chat, Cowork y Code están todas disponibles.
</Note>

La aplicación de escritorio en Linux te proporciona la misma experiencia de Chat, Cowork y Claude Code que en macOS y Windows: sesiones paralelas, revisión de diferencias visuales, un terminal y editor integrados, y vista previa de la aplicación en vivo. Consulta [Usar Claude Code Desktop](/docs/es/desktop) para la referencia completa de características.

<h2 id="requirements">
  Requisitos
</h2>

* Ubuntu 22.04 o posterior, o Debian 12 o posterior
* x86\_64 o arm64

Otras distribuciones basadas en Debian que cumplan con estos requisitos pueden funcionar pero no se han probado oficialmente.

<h2 id="install">
  Instalar
</h2>

Instala desde el repositorio apt de Anthropic para que las actualizaciones lleguen a través de las actualizaciones regulares de paquetes de tu sistema. Abre una terminal y ejecuta los comandos en cada paso.

<Steps>
  <Step title="Agregar el repositorio apt de Anthropic">
    Este paso descarga la clave de firma con `curl`, que las instalaciones nuevas de Debian y Ubuntu pueden no incluir. Si el comando de descarga falla con `sudo: curl: command not found`, instala curl primero:

    ```bash theme={null}
    sudo apt install curl
    ```

    Descarga la clave de firma de Anthropic:

    ```bash theme={null}
    sudo curl -fsSLo /usr/share/keyrings/claude-desktop-archive-keyring.asc https://downloads.claude.ai/claude-desktop/key.asc
    ```

    Registra el repositorio:

    ```bash theme={null}
    echo "deb [arch=amd64,arm64 signed-by=/usr/share/keyrings/claude-desktop-archive-keyring.asc] https://downloads.claude.ai/claude-desktop/apt/stable stable main" | sudo tee /etc/apt/sources.list.d/claude-desktop.list
    ```
  </Step>

  <Step title="Instalar el paquete">
    ```bash theme={null}
    sudo apt update && sudo apt install claude-desktop
    ```
  </Step>

  <Step title="Lanzar e iniciar sesión">
    Lanza **Claude** desde tu lanzador de aplicaciones, o ejecuta `claude-desktop` desde una terminal, e inicia sesión con tu cuenta de Anthropic.

    La aplicación de Linux inicia sesión de la misma manera que en macOS y Windows: con una suscripción a claude.ai, o a través del SSO de tu organización. Desktop no acepta una clave de API de Claude Console directamente; usa la [CLI](/docs/es/quickstart) para la autenticación con clave de API. Para implementaciones empresariales que enrutan Desktop a la Plataforma de Agentes de Google Cloud o a una puerta de enlace de LLM, consulta [Claude Desktop en 3P](https://claude.com/docs/third-party/claude-desktop/overview) y [configuración de red](/docs/es/network-config).
  </Step>
</Steps>

<Accordion title="Verificar la clave de firma">
  Puedes confirmar que la clave de firma descargada pertenece a Anthropic:

  ```bash theme={null}
  gpg --show-keys /usr/share/keyrings/claude-desktop-archive-keyring.asc
  ```

  La huella digital debe ser `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE`.
</Accordion>

<h3 id="install-from-a-downloaded-file">
  Instalar desde un archivo descargado
</h3>

Si no puedes instalar a través del repositorio apt, descarga el paquete `.deb` directamente desde el grupo de paquetes del repositorio. Este comando busca el paquete más nuevo para tu arquitectura en el índice del repositorio, luego lo descarga en el directorio actual:

```bash theme={null}
curl -fLO "https://downloads.claude.ai/claude-desktop/apt/stable/$(curl -s "https://downloads.claude.ai/claude-desktop/apt/stable/dists/stable/main/binary-$(dpkg --print-architecture)/Packages" | grep '^Filename: pool/main/c/claude-desktop/claude-desktop_' | sort -V | tail -n 1 | cut -d' ' -f2)"
```

Si el comando falla con `Remote file name has no length`, la búsqueda no devolvió ninguna ruta de paquete. Esto puede significar que el índice del repositorio no se pudo obtener, por ejemplo cuando tu red bloquea `downloads.claude.ai`, o que no existe ningún paquete para tu arquitectura. Confirma que tu red puede alcanzar `downloads.claude.ai` y que `dpkg --print-architecture` imprime `amd64` o `arm64`; el repositorio no publica paquetes para otras arquitecturas.

Luego abre el archivo descargado con tu instalador de software, como GNOME Software, o instálalo con apt desde el directorio que contiene el archivo descargado:

```bash theme={null}
sudo apt install ./claude-desktop_*.deb
```

Si apt reporta `E: Unsupported file ./claude-desktop_*.deb given on commandline`, el patrón no coincidió con un archivo `.deb` en el directorio actual. Confirma que la descarga se completó, luego ejecuta el comando nuevamente desde el directorio que contiene el archivo.

Un `.deb` instalado de esta manera no recibe actualizaciones. Para obtener actualizaciones a través de apt, registra el repositorio desde el paso [Agregar el repositorio apt de Anthropic](#install). El paquete también escribe una entrada de repositorio comentada en `/etc/apt/sources.list.d/claude-desktop.list`; descomenta su línea `deb` es equivalente.

<h2 id="update">
  Actualizar
</h2>

La aplicación de escritorio no se actualiza a sí misma en Linux. Las actualizaciones llegan con las actualizaciones regulares de paquetes de tu sistema:

```bash theme={null}
sudo apt update && sudo apt upgrade
```

El actualizador de software gráfico de tu distribución también detectará nuevas versiones.

<h2 id="uninstall">
  Desinstalar
</h2>

```bash theme={null}
sudo apt remove claude-desktop
```

Esto elimina la clave de firma junto con la aplicación, así que si agregaste la entrada del repositorio durante la instalación, elimínala también:

```bash theme={null}
sudo rm /etc/apt/sources.list.d/claude-desktop.list
```

<h2 id="troubleshoot">
  Solución de problemas
</h2>

<h3 id="unable-to-locate-package-claude-desktop">
  No se puede localizar el paquete claude-desktop
</h3>

Si `sudo apt install claude-desktop` falla con `E: Unable to locate package claude-desktop`, apt no encontró el repositorio que agregó. Verifique lo siguiente:

* Confirme que la entrada del repositorio se escribió. `cat /etc/apt/sources.list.d/claude-desktop.list` debe mostrar la línea `deb` del paso [Agregar el repositorio apt de Anthropic](#install). Si el archivo está vacío o falta, ejecute ese paso nuevamente.
* Confirme que su arquitectura es compatible. `dpkg --print-architecture` debe imprimir `amd64` o `arm64`. El repositorio no publica paquetes para otras arquitecturas.
* Ejecute `sudo apt update` nuevamente y verifique su salida para detectar errores relacionados con `downloads.claude.ai`. Un error de red o clave allí significa que el repositorio se agregó pero no se pudo alcanzar o verificar.

Si el repositorio está en su lugar y es accesible y el paquete aún no se encuentra, [instale desde un archivo descargado](#install-from-a-downloaded-file) en su lugar.

<h2 id="what’s-not-in-the-linux-beta-yet">
  Lo que aún no está en la beta de Linux
</h2>

* **Computer Use**: [el control de aplicaciones y pantalla](/docs/es/desktop#let-claude-use-your-computer) no está disponible en Linux.
* **Dictation**: la entrada de voz no está disponible en la aplicación de escritorio Linux. Usa [dictación de voz](/docs/es/voice-dictation) en la CLI en su lugar.
* **Quick Entry global hotkey**: funciona en X11. En Wayland nativo requiere el portal GlobalShortcuts de tu entorno de escritorio.
* **Fedora y RHEL**: solo se admiten distribuciones basadas en Debian hoy en día. La compatibilidad con distribuciones adicionales llegará en el futuro.

Para cualquier cosa que aún no esté disponible en la aplicación de escritorio, la [CLI](/docs/es/quickstart) ejecuta el mismo motor de Claude Code y admite un rango más amplio de distribuciones de Linux; consulta los [requisitos del sistema](/docs/es/setup#system-requirements).
