> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Desktop en Linux (beta)

> Instala y actualiza la aplicación de escritorio Claude en Ubuntu y Debian

<Note>
  La compatibilidad con Linux para la aplicación de escritorio Claude está en beta.
</Note>

La aplicación de escritorio en Linux te proporciona la misma experiencia de Chat, Cowork y Claude Code que en macOS y Windows: sesiones paralelas, revisión de diferencias visuales, un terminal y editor integrados, y vista previa de la aplicación en vivo. Consulta [Usar Claude Code Desktop](/docs/es/desktop) para la referencia de características.

<h2 id="requirements">
  Requisitos
</h2>

* Una distribución basada en Debian: Ubuntu 22.04 o posterior, o Debian 12 o posterior
* x86\_64 o arm64

Otras distribuciones basadas en Debian que cumplan con estos requisitos pueden funcionar pero no se han probado oficialmente. En distribuciones que no estén basadas en Debian, como Fedora o Arch, ejecute la [CLI](/docs/es/setup#system-requirements) en su lugar. Si trabaja en Windows con WSL 2, instale la aplicación de escritorio de Windows y ejecute sesiones dentro de su distribución; consulte [Claude Code Desktop en WSL](/docs/es/desktop-wsl).

<h3 id="cowork-requirements">
  Requisitos de Cowork
</h3>

Cowork es la pestaña de escritorio para [Dispatch y trabajo agéntico más largo](https://claude.com/docs/cowork/overview). En Linux, Cowork ejecuta esas tareas en una máquina virtual que la aplicación de escritorio aloja con QEMU y KVM. Para usar Cowork, su máquina necesita:

* **Virtualización de hardware**: activada en la configuración del firmware. Sin ella, la pestaña Cowork reporta "Cowork requires hardware virtualization (KVM)".
* **Firmware QEMU y UEFI**: `qemu-system-x86`, `ovmf`, y `virtiofsd` en x86\_64, o `qemu-system-arm`, `qemu-efi-aarch64`, y `virtiofsd` en arm64. `apt install claude-desktop` los instala por defecto como paquetes recomendados. Si instaló con `--no-install-recommends`, o su sistema es una imagen mínima que omite paquetes recomendados, la pestaña Cowork reporta "Cowork requires QEMU" y muestra el comando `apt install` a ejecutar. Ubuntu 22.04 no tiene paquete `virtiofsd`; la aplicación usa una copia incluida allí.
* **Acceso a `/dev/kvm`**: agregue su usuario al grupo `kvm` con `sudo usermod -aG kvm $USER`, luego cierre sesión e inicie sesión nuevamente. Algunos entornos de escritorio otorgan acceso al usuario conectado a `/dev/kvm` sin el grupo, pero Cowork también necesita `/dev/vhost-vsock`, que solo los miembros del grupo `kvm` pueden abrir. Únase al grupo incluso si `/dev/kvm` ya funciona para usted.

La aplicación verifica estos requisitos una vez al iniciar: reiníciela después de instalar paquetes, y cierre sesión e inicie sesión nuevamente después de unirse al grupo. Si `/dev/vhost-vsock` falta y su kernel en ejecución no tiene directorio de módulos bajo `/lib/modules`, la pestaña Cowork reporta que el kernel no incluye el soporte de virtualización que Cowork necesita y que no se puede agregar manualmente. Esta combinación es común en ChromeOS y en entornos Linux basados en contenedores.

<h2 id="install">
  Instalar
</h2>

Instala desde el repositorio apt de Anthropic para que las actualizaciones lleguen a través de las actualizaciones regulares de paquetes de tu sistema. Abre una terminal y ejecuta los comandos en cada paso.

<Steps>
  <Step title="Agregar el repositorio apt de Anthropic">
    Este paso descarga la clave de firma con `curl` y la verifica con `gpg`, que las instalaciones nuevas de Debian y Ubuntu pueden no incluir. Si alguno de los comandos reporta `command not found`, instala ambos primero:

    ```bash theme={null}
    sudo apt install curl gnupg
    ```

    Descarga la clave de firma de Anthropic:

    ```bash theme={null}
    sudo curl -fsSLo /usr/share/keyrings/claude-desktop-archive-keyring.asc https://downloads.claude.ai/claude-desktop/key.asc
    ```

    El comando no imprime nada cuando tiene éxito y un error `curl:` cuando no. Una clave faltante o incorrecta hace que `apt update` falle más tarde con `NO_PUBKEY BAA929FF1A7ECACE`, así que confirma que la clave se descargó y pertenece a Anthropic antes de continuar:

    ```bash theme={null}
    gpg --show-keys /usr/share/keyrings/claude-desktop-archive-keyring.asc
    ```

    La huella digital que gpg imprime debe ser `31DDDE24DDFAB679F42D7BD2BAA929FF1A7ECACE`. Si gpg reporta que el archivo no se puede abrir o no contiene datos OpenPGP válidos, la descarga falló o devolvió contenido incorrecto: confirma que tu red puede alcanzar `downloads.claude.ai`, luego vuelve a ejecutar el comando de descarga.

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

<h3 id="install-from-a-downloaded-file">
  Instalar desde un archivo descargado
</h3>

Si no puedes instalar a través del repositorio apt, descarga el paquete `.deb` directamente desde el grupo de paquetes del repositorio. Este comando busca el paquete más nuevo para tu arquitectura en el índice del repositorio, luego lo descarga en el directorio actual:

```bash theme={null}
curl -fLO "https://downloads.claude.ai/claude-desktop/apt/stable/$(curl -s "https://downloads.claude.ai/claude-desktop/apt/stable/dists/stable/main/binary-$(dpkg --print-architecture)/Packages" | grep '^Filename: pool/main/c/claude-desktop/claude-desktop_' | sort -V | tail -n 1 | cut -d' ' -f2)"
```

Si el comando falla con `Remote file name has no length`, la búsqueda no devolvió ninguna ruta de paquete. Esto puede significar que el índice del repositorio no se pudo obtener, por ejemplo cuando tu red bloquea `downloads.claude.ai`, o que no existe ningún paquete para tu arquitectura. Confirma que tu red puede alcanzar `downloads.claude.ai` y que `dpkg --print-architecture` imprime `amd64` o `arm64`; el repositorio no publica paquetes para otras arquitecturas.

Para instalar sin registrar el repositorio apt de Anthropic, primero crea `/etc/default/claude-desktop` con la línea `CLAUDE_DESKTOP_ADD_REPO="false"`. Sin el repositorio, apt no entrega nuevas versiones; para actualizar, vuelve a ejecutar el comando de descarga e reinstala, o [registra el repositorio](#install) más tarde.

Luego abre el archivo descargado con tu instalador de software, como GNOME Software, o instálalo con apt desde el directorio que contiene el archivo descargado:

```bash theme={null}
sudo apt install ./claude-desktop_*.deb
```

Si apt reporta `E: Unsupported file ./claude-desktop_*.deb given on commandline`, el patrón no coincidió con un archivo `.deb` en el directorio actual. Confirma que la descarga se completó, luego ejecuta el comando nuevamente desde el directorio que contiene el archivo.

Instalar el `.deb` también registra el repositorio apt de Anthropic en `/etc/apt/sources.list.d/claude-desktop.list`, así que las futuras actualizaciones llegan con las [actualizaciones regulares de paquetes](#update) de tu sistema.

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

Desinstalar el paquete también elimina la entrada del repositorio y la clave de firma que registró. Si agregó la entrada del repositorio usted mismo con el paso [Agregar el repositorio apt de Anthropic](#install), elimínela también:

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

* Ejecute `sudo apt update` después de agregar el repositorio. `apt install` por sí solo no ve un repositorio que agregó después de la última vez que ejecutó `apt update`.
* Confirme que la entrada del repositorio se escribió. `cat /etc/apt/sources.list.d/claude-desktop.list` debe mostrar la línea `deb` del paso [Agregar el repositorio apt de Anthropic](#install). Si el archivo está vacío o falta, ejecute ese paso nuevamente.
* Confirme que su arquitectura es compatible. `dpkg --print-architecture` debe imprimir `amd64` o `arm64`. El repositorio no publica paquetes para otras arquitecturas.
* Ejecute `sudo apt update` nuevamente y verifique su salida para detectar errores relacionados con `downloads.claude.ai`. Un error de red o clave allí significa que el repositorio se agregó pero no se pudo alcanzar o verificar.

Si el repositorio está en su lugar y es accesible y el paquete aún no se encuentra, [instale desde un archivo descargado](#install-from-a-downloaded-file) en su lugar.

<h3 id="unmet-dependencies">
  Dependencias no satisfechas
</h3>

Si `apt` se detiene con `The following packages have unmet dependencies` o `Unsatisfied dependencies`, lea qué dependencia nombra:

* `libc6 (>= 2.34)`: su distribución es más antigua de lo que el paquete admite. Ubuntu 20.04 incluye `libc6` 2.31. Actualice a Ubuntu 22.04 o posterior, o Debian 12 o posterior.
* Todas las dependencias faltantes muestran `not installable` con un sufijo `:amd64` o `:arm64`: descargó el `.deb` para una arquitectura diferente a la de su máquina. Ejecute `dpkg --print-architecture` y descargue el `.deb` coincidente, o [instale desde el repositorio apt](#install), que selecciona el paquete para su arquitectura.

<h3 id="running-as-root-without-no-sandbox-is-not-supported">
  La ejecución como root sin --no-sandbox no es compatible
</h3>

Si `claude-desktop` sale con este mensaje, lo inició como root. Inicie sesión como usuario normal e inícielo desde allí.

<h3 id="cowork-isn’t-available">
  Cowork no está disponible
</h3>

Si la pestaña Cowork muestra uno de estos mensajes, corrija el requisito que nombra y luego reinicie la aplicación:

* **Cowork requiere QEMU**: instale los [paquetes QEMU y firmware UEFI](#cowork-requirements) que el mensaje enumera.
* **Cowork requiere virtualización de hardware (KVM)**: active la [virtualización de hardware](#cowork-requirements) en la configuración del firmware.
* **Claude no tiene permiso para usar virtualización (/dev/kvm)**: agregue su usuario al [grupo `kvm`](#cowork-requirements), luego cierre sesión e inicie sesión nuevamente.
* **Cowork requiere el módulo del kernel `vhost_vsock`**: ejecute `sudo modprobe vhost_vsock`, luego reinicie la aplicación. Eso carga el módulo solo para el arranque actual. Para cargarlo en cada arranque, ejecute `echo vhost_vsock | sudo tee /etc/modules-load.d/vhost_vsock.conf`.

<h2 id="what’s-not-in-the-linux-beta-yet">
  Lo que aún no está en la beta de Linux
</h2>

* **Computer Use**: [el control de aplicaciones y pantalla](/docs/es/desktop#let-claude-use-your-computer) no está disponible en Linux.
* **Dictation**: la entrada de voz no está disponible en la aplicación de escritorio Linux. Usa [dictación de voz](/docs/es/voice-dictation) en la CLI en su lugar.
* **Quick Entry global hotkey**: funciona en X11. En Wayland nativo requiere el portal GlobalShortcuts de tu entorno de escritorio.
* **Fedora y RHEL**: solo se admiten distribuciones basadas en Debian hoy en día. La compatibilidad con distribuciones adicionales llegará en el futuro.

Para cualquier cosa que aún no esté disponible en la aplicación de escritorio, la [CLI](/docs/es/quickstart) ejecuta el mismo motor de Claude Code y admite un rango más amplio de distribuciones de Linux; consulta los [requisitos del sistema](/docs/es/setup#system-requirements).
