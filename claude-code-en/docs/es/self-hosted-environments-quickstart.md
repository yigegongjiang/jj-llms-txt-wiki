> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Guía de inicio rápido de entornos autohospedados

> Configure su primer entorno autohospedado: instale Claude Code, cree el entorno, inicie un runner y enrute una sesión hacia él.

<Note>
  Los entornos autohospedados están en versión beta pública en planes Team y Enterprise; [Disponibilidad y limitaciones](/docs/es/self-hosted-environments#availability-and-limitations) cubre la ruta de habilitación. Esta página pone en marcha su primera sesión; consulte [Entornos autohospedados](/docs/es/self-hosted-environments) para saber qué son y [Implementar en producción](/docs/es/self-hosted-environments-deploy) para endurecimiento y recetas de flota.
</Note>

Un [entorno autohospedado](/docs/es/self-hosted-environments) ejecuta [sesiones en la nube](/docs/es/claude-code-on-the-web) de Claude Code en infraestructura que opera su organización, ejecutadas por procesos runner que usted implementa. Este inicio rápido configura el primero, el más pequeño que funciona: un runner en un único host, ejecutando una sesión de prueba. Hay dos pasos: [crear el entorno, iniciar un runner y enrutar una sesión hacia él](#set-up-an-environment-and-runner), luego [enviar un mensaje de seguimiento a esa sesión desde su terminal](#send-a-follow-up-message-to-a-running-session). Se moverá entre dos superficies: claude.ai para crear el entorno, verificar su estado y enrutar una sesión, y una terminal en el host para todo lo que hace el runner.

Al final tendrá un entorno en la [página de administración **Cloud environments**](https://claude.ai/admin-settings/cloud-environments), un runner consultando trabajo, y una sesión ejecutándose en su host. Antes de conectar repositorios reales o sistemas internos, trabaje a través de [Implementar en producción](/docs/es/self-hosted-environments-deploy), que cubre la postura de seguridad, control de egreso, credenciales de git y orquestación.

<h2 id="prerequisites">
  Requisitos previos
</h2>

<h3 id="organization-and-roles">
  Organización y roles
</h3>

El lado de claude.ai necesita:

* **Allow self-hosted environments** activado por un [Owner](/docs/es/cloud-environments#organization-shared-environments) en la [página de administración **Cloud environments**](https://claude.ai/admin-settings/cloud-environments); el botón **New** no aparece hasta que esté activado. Si no tiene el rol, alguien que lo tenga puede crear el entorno y entregarle su secreto; los pasos de runner y terminal en esta página no necesitan ningún rol de claude.ai, y donde un paso verifica el estado en la interfaz de administración, las propias líneas de registro del runner le dan la misma señal.
* Una [conexión de GitHub](/docs/es/claude-code-on-the-web#github-authentication-options) para su organización, para que los desarrolladores puedan seleccionar repositorios cuando inicien sesiones.

<h3 id="host-and-network">
  Host y red
</h3>

El host del runner necesita:

* Un host o contenedor Linux o macOS con HTTPS saliente a `api.anthropic.com`, a `claude.ai` y a los hosts de descarga a los que redirige para el paso de instalación a continuación, y a su host de git para el clon; la [tabla de requisitos de red](/docs/es/self-hosted-environments-deploy#network-requirements) tiene la lista completa. Windows no se admite como host de runner; ejecute el runner en un contenedor Linux en su lugar. Las estaciones de trabajo de desarrolladores no se ven afectadas, ya que las sesiones se inician desde claude.ai en un navegador.
* Un reloj sincronizado con la hora real, por ejemplo con NTP. La autenticación falla cuando el reloj está más de cinco minutos desincronizado; consulte [Troubleshooting](/docs/es/self-hosted-environments-deploy#troubleshooting).

<h3 id="software-on-the-runner-host">
  Software en el host del runner
</h3>

Instale en el host antes de comenzar:

* **Claude Code v2.1.224 o posterior**, con cualquiera de los [métodos de instalación estándar](/docs/es/setup). El runner es parte del binario `claude` estándar, y las versiones anteriores no reconocen el subcomando `self-hosted-runner`. El canal `latest` del instalador nativo predeterminado lleva cada versión tan pronto como se publica; el canal `stable`, el cask `claude-code` de Homebrew, y los repositorios estables apt, dnf y apk se retrasan aproximadamente una semana. Para fijar la versión exacta que ejecuta su flota, consulte [Instalar una versión específica](/docs/es/setup#install-a-specific-version). Para imágenes de contenedor, consulte el Dockerfile en [Implementar en producción](/docs/es/self-hosted-environments-deploy#build-the-runner-image).
* **Git 2.24 o más reciente**. Algunas opciones de git en la página de implementación necesitan versiones más recientes; [Configurar git](/docs/es/self-hosted-environments-deploy#configure-git) establece cada piso.

Confirme que el host está listo:

```bash theme={null}
claude self-hosted-runner --help
```

Un host listo imprime el texto de uso del runner, listando banderas como `--environment-secret-file`. En versiones anteriores a 2.1.224, el comando imprime la salida general de `claude --help` en su lugar; actualice con `claude update` o reinstale desde el canal `latest`.

<h2 id="set-up-an-environment-and-runner">
  Configurar un entorno y runner
</h2>

Claude Code incluye una configuración guiada: una sesión interactiva de Claude Code que lo guía a través de crear el entorno en la interfaz de administración, inicia un runner local con el archivo secreto que guarda, confirma que el runner se registra, y escribe una hoja de trucos en `./runner-setup/CHEAT-SHEET.md`. Ejecútelo en una máquina donde haya iniciado sesión con `claude auth login` usando una cuenta que tenga un rol Owner; no está disponible con claves API o proveedores de modelos de terceros. En hosts donde una sesión interactiva no es posible, use los pasos manuales a continuación en su lugar. Confirme que la [verificación de versión](#software-on-the-runner-host) pasó primero: en versiones anteriores a 2.1.224, este comando inicia una sesión ordinaria de Claude con las palabras como el prompt en lugar de la configuración guiada. Para iniciar la configuración guiada, ejecute el subcomando setup y siga los prompts:

```bash theme={null}
claude self-hosted-runner setup
```

Para configurar manualmente en su lugar:

<Steps>
  <Step title="Crear un entorno">
    Vaya a la [página **Cloud environments**](https://claude.ai/admin-settings/cloud-environments) en la configuración de administración. Bajo **Self-hosted environments**, seleccione **New**, nombre el entorno, y seleccione **Create**. En el segundo paso del asistente, seleccione **Copy environment key** para copiar el secreto del entorno, que la interfaz de administración etiqueta como una clave de entorno. claude.ai muestra el secreto una vez, y no puede recuperarlo más tarde; expira 365 días después de la creación. El ID `ccpool_...` del entorno permanece visible en su diálogo de detalle; lo necesitará para la verificación `aud` en [verificación de token](/docs/es/self-hosted-environments-identity) y para enviar [sesiones de prueba desde CI](/docs/es/self-hosted-environments-testing#run-the-test-loop).

    Si pierde el secreto o necesita rotarlo, cree un nuevo secreto desde la pestaña **Configuration** del entorno, implemente el nuevo secreto en sus runners, luego revoque el antiguo. Los runners que tengan un secreto revocado fallan su siguiente sondeo autenticado y salen, registrando `poll auth failed`, y su orquestador los reinicia con el nuevo secreto.
  </Step>

  <Step title="Iniciar un runner">
    Cree el directorio secreto. Este paso y el siguiente necesitan root para la ruta `/etc/claude`; cualquier ruta que el proceso runner pueda leer funciona, así que ajuste ambos comandos y el valor `--environment-secret-file` juntos si usa uno diferente.

    ```bash theme={null}
    mkdir -p /etc/claude
    ```

    Escriba el secreto del entorno en un archivo. El comando a continuación lee desde su terminal para que el secreto se mantenga fuera del historial de shell: pegue el valor que copió, presione Enter, luego Ctrl-D, y el `umask` del subshell hace que el archivo sea legible solo por su propietario.

    ```bash theme={null}
    (umask 077 && cat > /etc/claude/environment-secret)
    ```

    Elija un directorio base, reemplazando `<writable-dir>` en el comando runner a continuación con una ruta absoluta que el runner pueda escribir o crear. El runner crea el directorio al inicio, luego verifica repositorios y crea directorios por sesión bajo él. Sin `--base-dir` usa `/workspace`, que solo funciona si ese directorio ya existe y es escribible o inicia el runner como root.

    Si el runner no puede crear o escribir en la ruta, sale al inicio con un error nombrando el directorio en lugar de registrarse. Consulte [Troubleshooting](/docs/es/self-hosted-environments-deploy#troubleshooting).

    Luego inicie el runner con `--environment-secret-file` y `--base-dir`. El runner se registra con su entorno y comienza a sondear trabajo. Si el runner sale, reinícielo manualmente. Las implementaciones de producción ejecutan el runner bajo un orquestador que reinicia runners salidos, normalmente con un sistema de archivos fresco por reinicio; [Reutilizar un checkout precalentado](/docs/es/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout) cubre la configuración de disco persistente admitida.

    ```bash theme={null}
    claude self-hosted-runner --environment-secret-file '/etc/claude/environment-secret' --base-dir '<writable-dir>'
    ```
  </Step>

  <Step title="Verificar que el runner aparece">
    Regrese a la [página **Cloud environments**](https://claude.ai/admin-settings/cloud-environments). El estado de su entorno cambia de **No runners deployed** a **Healthy** dentro de unos pocos segundos de que el runner inicie; abra el entorno y seleccione **Activity** para ver el runner mismo.
  </Step>

  <Step title="Enrutar una sesión al entorno">
    Inicie una sesión en claude.ai/code y seleccione su entorno del selector de entorno, donde los entornos autohospedados aparecen junto a los alojados por Anthropic. El runner clona con las credenciales de git que el host ya tiene, así que seleccione un repositorio que este host ya pueda clonar, o uno público; las opciones de credenciales para repositorios privados en producción están en [Configurar git](/docs/es/self-hosted-environments-deploy#configure-git). El siguiente runner disponible recoge la sesión en cola y registra `Picked up session <session-id>` junto con su recuento activo y capacidad, para que pueda confirmar desde la propia salida del runner qué host tomó la sesión. Vea la sesión trabajar y lea las respuestas de Claude en [claude.ai/code](https://claude.ai/code). Si la sesión permanece en cola en su lugar, consulte [Troubleshooting](/docs/es/self-hosted-environments-deploy#troubleshooting).
  </Step>
</Steps>

El runner sale por diseño una vez que sus sesiones activas terminan; consulte [Runner lifecycle](/docs/es/self-hosted-environments#runner-lifecycle). Para producción, implántelo bajo un orquestador que lo reinicie al salir. Consulte [Implementar en producción](/docs/es/self-hosted-environments-deploy).

<h2 id="send-a-follow-up-message-to-a-running-session">
  Enviar un mensaje de seguimiento a una sesión en ejecución
</h2>

Una vez que una sesión se ejecuta en su entorno, envíele un seguimiento desde la CLI de `claude` en cualquier máquina donde haya iniciado sesión con `claude auth login`; el comando no necesita ejecutarse desde la máquina que inició la sesión. El comando publica un mensaje:

```bash theme={null}
claude -p "your message" --cloud <session-id>
```

Para `<session-id>`, pase el ID desnudo `session_...` o `cse_...` o la URL claude.ai/code de la sesión. Un envío exitoso imprime `Sent to cloud session.` con el ID de sesión y un enlace de vista. Las formas de ID aceptadas, salida JSON, los requisitos de cuenta y política, y la referencia de error están en [Enviar seguimientos desde la CLI](/docs/es/claude-code-on-the-web#send-follow-ups-from-the-cli), ya que el comando funciona igual contra sesiones alojadas por Anthropic.

<h2 id="what’s-next">
  Qué sigue
</h2>

* [Implementar en producción](/docs/es/self-hosted-environments-deploy): endurezca la implementación, controle el egreso, configure credenciales de git, y ejecute la flota bajo Kubernetes o Compose
* [Personalizar sesiones](/docs/es/self-hosted-environments-configuration): scripts de envoltura, hooks de ciclo de vida, runners bajo demanda, servidores MCP, y permisos
* [Probar de extremo a extremo](/docs/es/self-hosted-environments-testing): una prueba de humo de CI que envía una sesión y lee las respuestas de Claude
