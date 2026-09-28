> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurar entornos en la nube

> Configure entornos en la nube para sesiones en la nube de Claude Code: niveles de acceso a la red, variables de entorno, scripts de configuración y almacenamiento en caché de entornos.

<Note>
  Los entornos en la nube se aplican a [sesiones en la nube](/docs/es/claude-code-on-the-web), que están disponibles en planes Pro, Max y Team, y para usuarios de Enterprise con [asientos premium o asientos de Chat + Claude Code](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan).
</Note>

Cada [sesión en la nube](/docs/es/claude-code-on-the-web) se ejecuta en un entorno en la nube. Puede configurar un entorno para permitir o denegar [acceso a la red](#access-levels), [establecer variables de entorno](#set-environment-variables) para la sesión, en planes Pro y Max almacenar [credenciales de API](#add-api-credentials) que las sesiones utilizan sin verlas, y ejecutar un [script de configuración](#setup-scripts) antes de que Claude comience a trabajar.

Los mismos entornos se aplican dondequiera que inicie una sesión en la nube: la [aplicación de escritorio](/docs/es/desktop), la [aplicación móvil de Claude](/docs/es/mobile), su navegador en [claude.ai/code](https://claude.ai/code), la terminal con [`claude --cloud`](/docs/es/claude-code-on-the-web#from-terminal-to-cloud), [routines](/docs/es/routines) y [Claude Tag](https://claude.com/docs/claude-tag/overview). Cada una de estas superficies también puede enrutar a un [entorno autohospedado](/docs/es/self-hosted-environments). [Disponibilidad y limitaciones](/docs/es/self-hosted-environments#availability-and-limitations) cubre lo que Claude aún no puede usar cuando una sesión de Claude Tag se ejecuta en uno.

<Info>
  Las sesiones de [Remote Control](/docs/es/remote-control) conectan las interfaces web y móvil a una sesión en su propia máquina, que utiliza la red y los archivos de su máquina, no un entorno en la nube. Las sesiones de canales de Claude Tag utilizan entornos a nivel de organización únicamente, ya sean [entornos compartidos](#organization-shared-environments) o [entornos autohospedados](/docs/es/self-hosted-environments).
</Info>

<h2 id="the-default-environment">
  El entorno Default
</h2>

Si aún no tiene un entorno, la incorporación configura el entorno **Default** para usted. Cómo depende de dónde se incorpore:

* **Flujos CLI como `/web-setup`**: crean **Default** para usted
* **Incorporación web en Pro y Max**: crea **Default** para usted
* **Incorporación web en Team y Enterprise**: muestra un formulario **Crear su primer entorno en la nube** a menos que un Propietario haya activado [Configuración rápida de web](/docs/es/claude-code-on-the-web#github-authentication-options); mantenga los valores predeterminados del formulario y haga clic en **Crear y finalizar** para obtener el mismo entorno **Default**

**Default** no lleva ninguna configuración propia:

* [Acceso a la red **Trusted**](#access-levels): las sesiones alcanzan registros de paquetes y otros [dominios en la lista permitida](#default-allowed-domains), y nada más a través de la red de la sesión.
* Sin otra configuración: **Default** no define variables de entorno ni script de configuración, por lo que las sesiones comienzan con solo las [herramientas preinstaladas](#installed-tools).

Con solo **Default** disponible, cada sesión se ejecuta en él. Cuando tiene más de un entorno, las sesiones eligen uno por superficie:

* En la aplicación de escritorio, la aplicación móvil y en claude.ai/code, las sesiones que inicia usted mismo utilizan el entorno que se muestra en el [selector](#configure-your-environment). Un [valor predeterminado de la organización](#organization-shared-environments) establecido por un Propietario completa la selección cuando no ha elegido uno. Los hilos en un [proyecto](/docs/es/claude-projects#project-settings-reference) utilizan el entorno establecido en la configuración del proyecto en su lugar.
* Desde la CLI, Claude Code utiliza su selección de [`/remote-env`](#select-an-environment-from-the-cli), o recurre al entorno alojado por Anthropic cuando su lista tiene uno, y de lo contrario al primer entorno en su lista que no sea un entorno puente, una entrada [Remote Control](/docs/es/remote-control) que registra para representar su propia máquina en lugar de un entorno en la nube. Para un [entorno autohospedado](/docs/es/self-hosted-environments), pasar `--environment <environment-id>` con su ID `ccpool_` [cuando distribuya una sesión](/docs/es/self-hosted-environments-testing#run-the-test-loop) anula la selección de `/remote-env` y el respaldo para esa invocación. Claude Code rechaza los IDs `env_` alojados por Anthropic pasados a la bandera, así que use `/remote-env` para dirigirse a esos. La bandera requiere Claude Code v2.1.224 o posterior.

Configure un entorno cuando el valor predeterminado no sea suficiente: cuando Claude necesita alcanzar dominios fuera de la [lista permitida predeterminada](#default-allowed-domains), necesita variables de entorno establecidas para sus sesiones, o necesita dependencias instaladas antes de que comience a trabajar.

<h2 id="configure-your-environment">
  Configurar su entorno
</h2>

Cree, edite y archive entornos desde el selector de entornos, al que accede en [claude.ai/code](https://claude.ai/code) después de la [incorporación web](/docs/es/web-quickstart), o desde el cuadro de mensaje en la [aplicación de escritorio](/docs/es/desktop#cloud-sessions). Los entornos que crea son personales para su cuenta; los [entornos compartidos](#organization-shared-environments) creados por un Propietario aparecen en el mismo selector. Consulte [Herramientas instaladas](#installed-tools) para ver qué está disponible sin ninguna configuración.

<Steps>
  <Step title="Abrir el selector de entornos">
    En [claude.ai/code](https://claude.ai/code), seleccione el icono de nube que muestra el nombre del entorno actual, en la fila encima del cuadro de mensaje. No hay página de configuración ni URL directa para el selector.

    <Frame>
      <img src="https://mintcdn.com/claude-code/ZFId6l95856c5LSw/images/cloud-environment-selector.png?fit=max&auto=format&n=ZFId6l95856c5LSw&q=85&s=cc2813a5664519eaf5a89d793ce5af26" alt="El selector de entornos abierto encima del cuadro de mensaje en claude.ai/code. El botón de nube que muestra el nombre del entorno Default se encuentra en la fila encima del cuadro de mensaje. El menú abierto enumera una fila Local con etiquetas de Descargar y Solo escritorio, una sección Cloud donde el entorno Default está seleccionado con una marca de verificación y muestra un icono de engranaje de configuración al pasar el ratón, una opción Agregar entorno en la nube y una sección Remote Control con instrucciones de configuración." width="1672" height="682" data-path="images/cloud-environment-selector.png" />
    </Frame>
  </Step>

  <Step title="Agregar o editar un entorno">
    Seleccione **Agregar entorno en la nube**, o pase el ratón sobre un entorno existente y seleccione el icono de configuración que aparece a la derecha. El diálogo incluye el nombre, nivel de acceso a la red, variables de entorno y script de configuración. Cuando edita un entorno en la nube existente en un plan Pro o Max, el diálogo también incluye [credenciales de API](#add-api-credentials).

    <Frame>
      <img src="https://mintcdn.com/claude-code/ZFId6l95856c5LSw/images/cloud-environment-dialog.png?fit=max&auto=format&n=ZFId6l95856c5LSw&q=85&s=30d4478b31d1f879f7ee287ddab32505" alt="El diálogo Nuevo entorno en la nube. Un campo Nombre con el texto de marcador de posición Default, un selector de Acceso a la red establecido en Trusted con enlaces a la política de red y niveles de acceso, un cuadro Variables de entorno que muestra texto de marcador de posición en formato .env con una nota de que los valores son visibles para cualquiera que use el entorno, un cuadro Script de configuración descrito como un script Bash que se ejecuta cuando comienza una nueva sesión antes de que se lance Claude Code, y botones Cancelar y Crear entorno." width="874" height="1372" data-path="images/cloud-environment-dialog.png" />
    </Frame>
  </Step>
</Steps>

<h3 id="set-environment-variables">
  Establecer variables de entorno
</h3>

Las variables de entorno utilizan formato `.env`, un par `KEY=value` por línea. Los valores simples no necesitan comillas, y si cita un valor con un par coincidente, las comillas no se convierten en parte del valor. Cite un valor que abarque varias líneas o contenga un `#`: en un valor sin comillas, `#` inicia un comentario y el resto de la línea se descarta.

El siguiente ejemplo define tres variables.

```text theme={null}
NODE_ENV=development
LOG_LEVEL=debug
DATABASE_URL=postgres://localhost:5432/myapp
```

Cada sesión copia los valores del entorno una vez, al inicio, en variables de entorno ordinarias que cualquier comando que ejecute Claude puede leer. Debido a que las sesiones en ejecución no vuelven a leer la configuración, editar o agregar variables afecta a las sesiones que inicia después; las sesiones ya en ejecución mantienen los valores con los que comenzaron.

Una sesión en la nube también establece algunas variables por sí misma cuando inicia. Para [`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`](/docs/es/claude-code-on-the-web#manage-context), el valor que la sesión establece anula uno que agregue aquí, por lo que agregar esa clave aquí no tiene efecto.

Cualquiera que use el entorno puede leer los valores. En planes Pro y Max, use una [credencial de API](#add-api-credentials) en su lugar para una clave que el proxy del agente pueda adjuntar a una solicitud. Las [solicitudes que nunca obtienen una credencial](#requests-that-never-get-the-credential) se enumeran allí.

<h3 id="add-api-credentials">
  Agregar credenciales de API
</h3>

Una credencial de API es una clave de API o token que almacena en un entorno en la nube para que Claude pueda llamar a esa API desde cualquier sesión en el entorno sin ver la clave. El proxy del agente de Anthropic agrega la clave a las solicitudes de los hosts que enumera, después de que cada solicitud sale de la VM de la sesión. La clave nunca llega a Claude, los comandos que ejecuta, o las variables de entorno de la sesión.

Las credenciales de API están disponibles en planes Pro y Max. No están disponibles en planes Team o Enterprise aún, por lo que la sección **Credenciales de API** no aparece en el diálogo de entorno en esos planes.

<h4 id="requirements">
  Requisitos
</h4>

Dos de estos deciden si puede agregar una credencial, y dos deciden si el proxy del agente puede usarla una vez agregada:

* **Rol**: un rol de administrador de la organización en su organización claude.ai
  * En Team y Enterprise, los Propietarios lo tienen y los Administradores no
  * En Pro y Max, lo tiene en su propia organización
  * Sin él, ve una nota en lugar de la lista de credenciales, incluso en sus propios entornos. Pida a un Propietario que agregue la credencial a un entorno compartido y ejecute sus sesiones allí
* **Tipo de entorno**: un entorno en la nube alojado por Anthropic que ya existe. Un [entorno autohospedado](/docs/es/self-hosted-environments) no tiene credenciales de API
* **Accesibilidad de API**: la API acepta conexiones desde internet, porque las solicitudes salen de la red de Anthropic
* **Claves de cifrado**: si su organización utiliza claves de cifrado administradas por el cliente, no puede guardar credenciales

<h4 id="add-a-credential">
  Agregar una credencial
</h4>

Agrega credenciales una a la vez desde el editor de un entorno que ya existe. El diálogo para un nuevo entorno no las ofrece. Tampoco hay edición. Para cambiar los hosts o el valor de una credencial, elimínela y agréguela de nuevo.

<Steps>
  <Step title="Abrir las credenciales de API del entorno">
    [Abra el entorno para editar](#configure-your-environment) en [claude.ai/code](https://claude.ai/code). En el diálogo **Actualizar entorno en la nube**, encuentre **Credenciales de API** debajo de **Variables de entorno**. Ve las credenciales ya en el entorno, cada una con los hosts a los que se aplica.
  </Step>

  <Step title="Agregar la credencial">
    Seleccione **Agregar credencial** y complete el formulario. Mantenga el **Tipo de credencial** predeterminado, **Bearer**, para una clave de API que viaja en un encabezado de solicitud, y complete estos campos:

    * **Nombre**: una etiqueta para la credencial, como `API de facturación interna`
    * **Sitios web permitidos**: los hosts de la API, como `api.example.com`. Un `*.` inicial coincide con cada subdominio
    * **Encabezados personalizados**: una fila para el encabezado que lleva la clave. La fila comienza con `Authorization` como el **Nombre** del encabezado y `Bearer` como su **Prefijo**; pegue la clave misma como el **Valor**. Para un encabezado como `X-Api-Key` que toma el valor desnudo, cambie el nombre y borre el prefijo

    Para una API que se autentica de otra manera, elija un **Tipo de credencial** diferente. La lista es la misma que [Claude Tag](https://claude.com/docs/claude-tag/overview), la integración de Slack para planes Team y Enterprise, ofrece para [conexiones](https://claude.com/docs/claude-tag/admins/add-connections).
  </Step>

  <Step title="Guardar la credencial">
    Seleccione **Conectar**. La credencial aparece en la lista con sus hosts, guardada sin el botón **Guardar cambios** del diálogo. No puede ver el valor nuevamente después de guardar.
  </Step>
</Steps>

Para confirmar que la credencial funciona, inicie una sesión en el entorno y pida a Claude que llame a la API, por ejemplo con `curl`. La API responde como si la clave estuviera en la solicitud, y la clave no aparece en las variables de entorno de la sesión ni en ningún archivo. Si la lista marca una credencial **No enviada** en su lugar, la nota debajo dice por qué y qué hacer. Dos credenciales cuyos hosts se superponen sin coincidir exactamente no obtienen marcador, y el proxy del agente envía solo una de ellas.

<h4 id="which-requests-get-the-credential">
  Qué solicitudes obtienen la credencial
</h4>

El proxy del agente adjunta una credencial a una solicitud cuando el host de la solicitud coincide con uno que enumera en esa credencial. Las sesiones pueden alcanzar esos hosts incluso cuando el [nivel de acceso a la red](#access-levels) del entorno no lo permitiría de otra manera, excepto los [hosts que nunca obtienen la credencial](#requests-that-never-get-the-credential). La credencial se aplica en cada sesión que se ejecuta en el entorno, quienquiera que la haya iniciado, hasta que la elimine.

<h4 id="requests-that-never-get-the-credential">
  Solicitudes que nunca obtienen la credencial
</h4>

El proxy del agente nunca adjunta una credencial que agregue a estas solicitudes:

* **GitHub**: el [proxy de GitHub](#github-proxy) autentica las solicitudes a GitHub en su lugar, por lo que no necesita una credencial de API para él
* **La API de Anthropic y registros de paquetes públicos**: `api.anthropic.com`, `registry.npmjs.org`, `jsr.io`, `npm.jsr.io`, `pypi.org`, `files.pythonhosted.org`, `index.crates.io` y `proxy.golang.org`
* **Solicitudes de script de configuración**: Claude Code se conecta al proxy del agente cuando se lanza, después de que el [script de configuración](#setup-scripts) ha ejecutado

<h3 id="select-an-environment-from-the-cli">
  Seleccionar un entorno desde la CLI
</h3>

Ejecute `/remote-env` en su terminal para elegir el entorno predeterminado para sesiones en la nube que crea desde la CLI, como [`claude --cloud`](/docs/es/claude-code-on-the-web#from-terminal-to-cloud). El comando abre un selector de sus entornos existentes y guarda su elección en la clave `remote.defaultEnvironmentId` en su [configuración de usuario](/docs/es/settings#where-settings-live), por lo que se aplica en cada proyecto en su máquina hasta que lo cambie, a menos que la misma clave esté establecida en una [capa de configuración](/docs/es/settings#settings-precedence) de mayor precedencia, como la configuración del proyecto de un repositorio.

Un ID de [entorno autohospedado](/docs/es/self-hosted-environments), que tiene la forma `ccpool_...`, sigue una regla de origen más estricta. Consulte [`remote.defaultEnvironmentId`](/docs/es/settings-reference#remote-defaultenvironmentid) para las capas de configuración que Claude Code honra.

`/remote-env` solo establece el valor predeterminado: no inicia una sesión, y no puede agregar o editar entornos. Adminístrelos desde el [selector de entornos](#configure-your-environment).

<h3 id="archive-an-environment">
  Archivar un entorno
</h3>

Para archivar uno de sus propios entornos, ábralo para editar y seleccione **Archivar**. Un Propietario archiva un [entorno compartido](#organization-shared-environments) desde la página **Entornos en la nube** en la configuración de administrador. No puede eliminar un entorno, solo archivarlo.

El archivado afecta a las nuevas sesiones, no a las que se están ejecutando:

* Las sesiones ya en ejecución en el entorno continúan funcionando.
* El entorno desaparece del selector y de `/remote-env`, por lo que no puede elegirlo para nuevas sesiones.
* Las credenciales de API en el entorno permanecen adjuntas en sus sesiones en ejecución. Elimine las que ya no desee antes de archivar.
* Ninguna sesión nueva puede iniciarse en un entorno archivado, en ninguna superficie. Si el entorno era su [valor predeterminado de CLI](#select-an-environment-from-the-cli) guardado, Claude Code inicia sesiones en la nube de CLI en el entorno alojado por Anthropic cuando su lista tiene uno, y de lo contrario en el primer entorno en su lista que no sea un [entorno puente de Remote Control](#the-default-environment). Cualquier cosa configurada con el entorno explícitamente, como una [routine](/docs/es/routines#environments-and-network-access), no puede iniciar nuevas sesiones en él. Apúntela a otro entorno.

<h3 id="organization-shared-environments">
  Entornos compartidos de la organización
</h3>

En planes Team y Enterprise, un Propietario puede crear entornos en la nube que se comparten con cada miembro de la organización. El mismo rol administra todo lo demás en la página **Entornos en la nube** del administrador, incluyendo [entornos autohospedados](/docs/es/self-hosted-environments); el rol Administrador no puede abrir la página. La lista completa de roles que pueden abrirla es la de [administración de configuración administrada por servidor](/docs/es/server-managed-settings#access-control).

Los entornos compartidos aparecen en el [selector de entornos](#configure-your-environment) de cada miembro bajo un encabezado **Organización**, después de los entornos propios del miembro bajo **Personal**, por lo que un equipo puede estandarizar una configuración en lugar de que cada miembro la recree. Seleccionar el icono de configuración de un entorno compartido allí abre un resumen de solo lectura de su configuración para cada miembro, incluyendo Propietarios.

Un Propietario pone un entorno a disposición de la organización de una de dos formas:

* **Crear un entorno compartido**: use la página **Entornos en la nube** en [configuración de administrador](https://claude.ai/admin-settings), que es también donde los Propietarios editan y archivan entornos compartidos. Cada uno tiene un nombre, un [nivel de acceso a la red](#access-levels), [variables de entorno](#set-environment-variables) en formato `.env` y un [script de configuración](#setup-scripts).
* **Compartir un entorno personal**: abra uno de sus propios entornos para editar en el selector de entornos, luego compártalo desde la fila **Quién puede usarlo**. El entorno mantiene su ID, por lo que las sesiones y routines que ya lo usan no se ven afectadas, y cada miembro puede entonces verlo e iniciar sesiones en él.

Los Propietarios eligen el [entorno predeterminado](#the-default-environment) de la organización por separado, en [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code).

Cada sesión de miembro en un entorno compartido lee sus variables, por lo que no incluya secretos en ellas. Las [credenciales de API](#add-api-credentials), que dan a las sesiones una clave que no pueden leer, aún no están disponibles en planes Team o Enterprise.

<h3 id="set-the-environment-a-claude-tag-channel-uses">
  Establecer el entorno que usa un canal de Claude Tag
</h3>

En canales de [Claude Tag](https://claude.com/docs/claude-tag/overview), Claude trabaja como la identidad compartida de su organización, no como ningún miembro, por lo que las sesiones de canal utilizan entornos a nivel de organización únicamente, ya sean entornos compartidos o [entornos autohospedados](/docs/es/self-hosted-environments). Para dar a un canal una cadena de herramientas que no esté [preinstalada](#installed-tools), como .NET, un Propietario puede crear un [entorno compartido](#organization-shared-environments) desde la página **Entornos en la nube** del administrador con un [script de configuración](#setup-scripts) que lo instale. Apunte el canal a un entorno de una de dos formas:

* Establezca un entorno compartido o autohospedado como el [entorno predeterminado](#the-default-environment) de la organización en [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code).
* [Fije uno a un canal](https://claude.com/docs/claude-tag/admins/troubleshooting#channel-sessions-use-the-wrong-environment-or-can%E2%80%99t-find-one) en la configuración de administrador de Claude Tag.

<h2 id="network-access">
  Acceso a la red
</h2>

Cada entorno establece un nivel de acceso a la red, que controla las conexiones salientes que pueden hacer sus sesiones. El nivel predeterminado, **Trusted**, permite registros de paquetes y otros [dominios en la lista permitida](#default-allowed-domains); **Custom** toma su propia lista de dominios.

Para cambiar el acceso a la red de un entorno, [ábralo para editar](#configure-your-environment) y use el selector **Network access** en el diálogo. Un [entorno compartido](#organization-shared-environments) se abre como solo lectura allí, por lo que un Propietario cambia su acceso a la red desde la página **Cloud environments** en [configuración de administrador](https://claude.ai/admin-settings) en su lugar. El icono de nube que abre el selector aparece en las superficies de la aplicación enumeradas bajo [El entorno Default](#the-default-environment) y en el [editor de routines](/docs/es/routines#environments-and-network-access); los entornos personales no tienen una página separada en la configuración de su cuenta claude.ai.

<Note>
  Los conectores MCP que habilita en una sesión o routine funcionan sin agregar sus hosts a **Allowed domains**, porque el tráfico del conector viaja a través de los servidores de Anthropic en lugar de la red de la sesión. Esto se basa en el mismo canal vinculado a Anthropic anotado bajo [Seguridad y aislamiento](/docs/es/claude-code-on-the-web#security-and-isolation). Desactive cualquier conector que no necesite para limitar qué herramientas puede alcanzar Claude.
</Note>

<h3 id="access-levels">
  Niveles de acceso
</h3>

El campo **Network access** en el [diálogo de entorno](#configure-your-environment) toma uno de cuatro niveles:

| Nivel       | Conexiones salientes                                                                                           |
| :---------- | :------------------------------------------------------------------------------------------------------------- |
| **None**    | Sin acceso a la red saliente a través de la red de la sesión                                                   |
| **Trusted** | Solo [dominios en la lista permitida](#default-allowed-domains): registros de paquetes, GitHub, SDK en la nube |
| **Full**    | Cualquier dominio                                                                                              |
| **Custom**  | Su propia lista permitida, opcionalmente incluyendo los valores predeterminados                                |

Cualquiera que sea el nivel que elija, las sesiones aún pueden alcanzar estos, porque cada uno toma una ruta que no pasa por la lista permitida de red de la sesión:

* GitHub, a través de su [proxy separado](#github-proxy)
* [Conectores MCP](#network-access) que habilita, cuyo tráfico viaja a través de los servidores de Anthropic
* Los hosts que enumera en las [credenciales de API](#add-api-credentials) del entorno, excepto los [hosts que nunca reciben la credencial](#requests-that-never-get-the-credential)
* La API de Anthropic, para las propias solicitudes de Claude Code, incluso en **None**, como se señala bajo [Seguridad y aislamiento](/docs/es/claude-code-on-the-web#security-and-isolation)

<h3 id="allow-specific-domains">
  Permitir dominios específicos
</h3>

Para permitir dominios que no están en la lista Trusted, seleccione **Custom** en la configuración de acceso a la red del entorno, luego enumere un dominio por línea en el campo **Allowed domains**. Este ejemplo permite tres hosts que un proyecto interno podría necesitar.

```text theme={null}
api.example.com
*.internal.example.com
registry.example.com
```

Las sesiones en este entorno ahora pueden alcanzar `api.example.com`, cualquier subdominio de `internal.example.com` y `registry.example.com`, y ningún otro dominio a través de la red de la sesión. El [tráfico de GitHub](#github-proxy), el [tráfico del conector MCP](#network-access) y las solicitudes a los hosts de las [credenciales de API](#add-api-credentials) del entorno, excepto los [hosts que nunca reciben la credencial](#requests-that-never-get-the-credential), no pasan por esta lista permitida. Un `*.` inicial coincide con cada subdominio. Para mantener también los [dominios Trusted](#default-allowed-domains), marque **Also include default list of common package managers**; déjelo sin marcar para permitir solo lo que enumera.

Si su organización utiliza [artefactos](/docs/es/artifacts#availability), no necesita `*.frame.claudeusercontent.com` en la lista para que las sesiones los lean. Cuando la lista deja ese host fuera, Claude Code lee el contenido del artefacto a través de la conexión de la sesión a Anthropic en su lugar. Mantenga el host en una lista permitida en dos situaciones:

* **Las sesiones en este entorno abren artefactos públicos de otra organización**: Claude Code obtiene esos del host directamente, así que agréguelo a esta lista.
* **Está configurando la CLI local o un ejecutor autohospedado**: mantenga el host en esa lista permitida. Consulte [requisitos de acceso a la red](/docs/es/network-config#network-access-requirements) y los [requisitos de red](/docs/es/self-hosted-environments-deploy#network-requirements) autohospedados.

Cada entorno tiene su propia lista de dominios permitidos; no hay una lista permitida a nivel de organización que los administradores puedan enviar a los entornos de cada miembro. La [configuración administrada por servidor](/docs/es/server-managed-settings) aún se aplica dentro de sesiones en la nube, pero ninguna de ellas agrega dominios a la lista permitida de red del entorno. Para dar a un equipo una lista estándar, un Propietario puede crear un [entorno compartido de la organización](#organization-shared-environments) con acceso a la red **Custom** y esa lista.

<h3 id="github-proxy">
  Proxy de GitHub
</h3>

En entornos alojados por Anthropic, todas las operaciones de GitHub pasan por un proxy dedicado que mantiene sus credenciales reales de GitHub fuera de la VM de la sesión, independientemente del [nivel de acceso](#access-levels) del entorno. Las sesiones en un entorno autohospedado autentican operaciones de git con credenciales que su implementación proporciona; [Configurar git](/docs/es/self-hosted-environments-deploy#configure-git) cubre las opciones, incluyendo credenciales acuñadas por sesión y una opción de participación en este mismo proxy. El proxy proporciona:

* **Credenciales de Git**: el cliente git dentro de la VM utiliza una credencial con alcance, que el proxy verifica e intercambia por su token real de GitHub.
* **Solicitudes de API**: las solicitudes de las herramientas integradas de GitHub y de `gh` bajo el [marcador de posición `proxy-injected`](#work-with-github-issues-and-pull-requests), se envían con sus credenciales reales sustituidas.
* **Protección de push**: `git push` funciona solo contra la rama de trabajo actual de la sesión; la clonación, la obtención y las operaciones de PR funcionan normalmente.
* **Alcance del repositorio**: las solicitudes de API de GitHub y de activos de lanzamiento alcanzan solo repositorios adjuntos a la sesión, por lo que un script de configuración que descarga activos de lanzamiento de un repositorio no adjunto obtiene un 403.
* **Restricciones de GraphQL**: el proxy sirve solo un conjunto fijado de operaciones de GraphQL para flujos de trabajo de solicitud de extracción. El proxy rechaza todo lo demás en el punto final de GraphQL con un 403 que dice `This GraphQL query is not enabled for this session` y nombra el respaldo de REST, `gh api repos/{owner}/{repo}/...`. La restricción se aplica a cada solicitud a través del proxy independientemente de las credenciales que suministre, por lo que un `GH_TOKEN` que establezca obtiene el mismo 403. Claude no puede alcanzar APIs de GitHub que existen solo en GraphQL, como Projects v2, a través del proxy.

Los archivos confirmados de repositorios públicos llegan a través de `raw.githubusercontent.com`, que el [proxy de seguridad](#security-proxy) maneja en su lugar. Ese dominio está en la [lista Trusted](#default-allowed-domains) predeterminada, por lo que esos archivos permanecen accesibles a menos que el [nivel de acceso](#access-levels) del entorno los excluya.

<h3 id="security-proxy">
  Proxy de seguridad
</h3>

Las sesiones en la nube en entornos alojados por Anthropic se ejecutan detrás de un proxy de red HTTP/HTTPS para fines de seguridad y prevención de abuso; en un [entorno autohospedado](/docs/es/self-hosted-environments-deploy#default-deny-egress), el tráfico saliente sale a través de su propio límite de red en su lugar. Todo el tráfico de internet saliente de una sesión alojada por Anthropic pasa por este proxy, que proporciona:

* Protección contra solicitudes maliciosas
* Limitación de velocidad y prevención de abuso
* Filtrado de contenido para mayor seguridad
* Un registro de auditoría a nivel de DNS de nombres de host solicitados

<h2 id="what’s-available-in-cloud-sessions">
  Qué está disponible en sesiones en la nube
</h2>

En entornos alojados por Anthropic, cada sesión obtiene una máquina virtual (VM) nueva ejecutando Ubuntu 24.04 en x86\_64, independientemente de su propio sistema operativo y arquitectura de CPU, con su repositorio clonado y cadenas de herramientas comunes preinstaladas. Cuando una dependencia proporciona binarios precompilados, como gemas de Ruby con extensiones nativas o ruedas de Python precompiladas, use su compilación de Linux x86\_64 para coincidir con la VM. Esta sección cubre los valores predeterminados alojados por Anthropic, las herramientas integradas de GitHub, cómo [ejecutar pruebas y servicios](#run-tests-start-services-and-add-packages) y los [límites de recursos](#resource-limits) que obtiene cada VM.

<Note>
  Las sesiones que su organización enruta a un [entorno autohospedado](/docs/es/self-hosted-environments) se ejecutan en sus propios ejecutores en su lugar, con las herramientas que proporciona su imagen de ejecutor.
</Note>

<h3 id="what-carries-over-from-your-setup">
  Qué se transfiere de su configuración
</h3>

Las sesiones en la nube comienzan desde un clon nuevo de su repositorio. Cualquier cosa que confirme en el repositorio está disponible. Cualquier cosa que haya instalado o configurado solo en su propia máquina no está disponible en la sesión. La política de su organización llega por separado a través de [configuración administrada por servidor](/docs/es/server-managed-settings).

|                                                                                                                                                                                                              | Disponible en sesiones en la nube                                     | Por qué                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Su `CLAUDE.md` del repositorio                                                                                                                                                                               | Sí                                                                    | Parte del clon                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Sus hooks `.claude/settings.json` del repositorio y reglas de permisos                                                                                                                                       | Sí, en una sesión con un repositorio                                  | Parte del clon. Una sesión con varios repositorios, incluido un hilo de [proyecto](/docs/es/claude-projects#what-threads-pick-up-from-your-repositories), comienza por encima de los clones y no los lee                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| Sus servidores MCP `.mcp.json` del repositorio                                                                                                                                                               | Sí, en una sesión con un repositorio                                  | Parte del clon, encontrado desde el directorio de trabajo de la sesión                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| Su `.claude/rules/` del repositorio                                                                                                                                                                          | Sí                                                                    | Parte del clon                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Su `.claude/skills/`, `.claude/agents/`, `.claude/commands/` del repositorio                                                                                                                                 | Sí                                                                    | Parte del clon                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Plugins y marketplaces declarados en el `.claude/settings.json` de su repositorio                                                                                                                            | No                                                                    | Una sesión en la nube no instala los plugins que un repositorio activa bajo [`enabledPlugins`](/docs/es/settings-reference#enabledplugins), incluidos los de los marketplaces que enumera bajo [`extraKnownMarketplaces`](/docs/es/settings-reference#extraknownmarketplaces)                                                                                                                                                                                                                                                                                                                                                                                                                  |
| La [configuración administrada por servidor](/docs/es/server-managed-settings) de su organización                                                                                                                 | Sí                                                                    | Obtenida de los servidores de Anthropic cuando comienza la sesión. Consulte [Cobertura de superficie](/docs/es/model-config#surface-coverage) para ver cómo se aplica `availableModels` en sesiones en la nube. La configuración implementada en su dispositivo a través de MDM o archivos de configuración administrada no se aplica, porque la sesión se ejecuta en una VM administrada por Anthropic; en un [entorno autohospedado](/docs/es/self-hosted-environments), las sesiones también leen el archivo de configuración administrada en la imagen del ejecutor, según [cómo Claude Code combina fuentes administradas](/docs/es/managed-settings#how-claude-code-combines-managed-sources) |
| Su `~/.claude/CLAUDE.md` de usuario                                                                                                                                                                          | No                                                                    | Vive en su máquina, no en el repositorio                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| Su `~/.claude/skills/`, `~/.claude/agents/`, `~/.claude/commands/` de usuario                                                                                                                                | No                                                                    | Viven en su máquina, no en el repositorio. Confirme los en el directorio `.claude/` del repositorio en su lugar. Las sesiones en la nube cargan automáticamente las skills que habilita en claude.ai                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| Plugins habilitados solo en su configuración de usuario                                                                                                                                                      | No                                                                    | El `enabledPlugins` con alcance de usuario vive en `~/.claude/settings.json` en su máquina                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Servidores MCP que agregó con `claude mcp add` en el alcance local predeterminado o el alcance de usuario                                                                                                    | No                                                                    | Esos escriben en `~/.claude.json` en su máquina, no en el repositorio. Agregue el servidor con `claude mcp add --scope project`, que escribe el [`.mcp.json`](/docs/es/mcp#project-scope) del repositorio, y confirme ese archivo. Una sesión con un repositorio lo carga                                                                                                                                                                                                                                                                                                                                                                                                                 |
| Variables de transporte en el bloque `env` de `.claude/settings.json` de su repositorio, como `NODE_EXTRA_CA_CERTS` y las [variables de certificado de cliente mTLS](/docs/es/network-config#mtls-authentication) | No                                                                    | El entorno de alojamiento administra la conexión de API de la sesión, por lo que Claude Code ignora estas claves y anota cada clave ignorada en el registro de depuración de la sesión                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| Claves de API y tokens para servicios que Claude llama                                                                                                                                                       | En planes Pro y Max, como [credenciales de API](#add-api-credentials) | Agrega la clave una vez en el entorno y el proxy del agente la adjunta a las solicitudes de los hosts que enumera. Una clave que el proxy del agente [no puede adjuntar](#requests-that-never-get-the-credential), o cualquier clave en un plan Team o Enterprise, permanece en una variable de entorno                                                                                                                                                                                                                                                                                                                                                                              |
| Autenticación interactiva como AWS SSO                                                                                                                                                                       | No                                                                    | No compatible. SSO requiere inicio de sesión basado en navegador que no puede ejecutarse en una sesión en la nube                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |

Para que su propia configuración esté disponible en sesiones en la nube, confirme la en el repositorio.

Cualquiera que use el entorno puede leer sus variables de entorno y script de configuración. La nota del diálogo bajo **Variables de entorno** lo dice y advierte contra poner secretos allí. En planes Pro y Max, almacene una clave que el proxy del agente pueda adjuntar como una [credencial de API](#add-api-credentials) en su lugar.

<h3 id="installed-tools">
  Herramientas instaladas
</h3>

Las sesiones en la nube vienen con tiempos de ejecución de lenguaje comunes, herramientas de compilación y bases de datos preinstaladas. La tabla a continuación resume lo que se incluye por categoría.

| Categoría          | Incluido                                                               |
| :----------------- | :--------------------------------------------------------------------- |
| **Python**         | Python 3.x con pip, poetry, uv, black, mypy, pytest, ruff              |
| **Node.js**        | 20, 21 y 22, con npm, yarn, pnpm, bun¹, eslint, prettier, chromedriver |
| **Ruby**           | 3.1, 3.2, 3.3 con gem, bundler, rbenv                                  |
| **PHP**            | 8.3 con Composer                                                       |
| **Java**           | OpenJDK 21 con Maven y Gradle                                          |
| **Go**             | Go con soporte de módulos                                              |
| **Rust**           | rustc y cargo                                                          |
| **C/C++**          | GCC, Clang, cmake, ninja, conan                                        |
| **Docker**         | docker, dockerd, docker compose                                        |
| **Bases de datos** | PostgreSQL 16, Redis 7.0                                               |
| **Utilidades**     | git, gh, jq, yq, ripgrep, tmux, vim, nano                              |

¹ Bun está instalado pero tiene [problemas de compatibilidad](#install-dependencies-with-a-sessionstart-hook) conocidos con proxy para obtención de paquetes.

Para obtener las versiones de la mayoría de las herramientas en esta tabla, pida a Claude que ejecute `check-tools` en una sesión en la nube. Es un comando de shell instalado en la VM de la sesión, no un comando que escriba con `/`; pide a Claude porque [Claude ejecuta todos los comandos de VM para usted](#run-tests-start-services-and-add-packages). Para una herramienta que no reporta, como Ruby, PHP, bun, PostgreSQL o Redis, pida a Claude que ejecute el comando de versión propia de la herramienta, por ejemplo `psql --version`.

Las versiones de Node.js se instalan en `/opt/node20`, `/opt/node21` y `/opt/node22`, con 22 en `PATH` de forma predeterminada. Para trabajar con una versión diferente, pida a Claude que anteponga el directorio `bin` de esa versión, como `/opt/node20/bin`, a `PATH`.

Las cadenas de herramientas fuera de esta lista, como el SDK de .NET, no están preinstaladas incluso cuando sus registros de paquetes están en la [lista permitida predeterminada](#default-allowed-domains). Instálelas con un [script de configuración](#setup-scripts).

<h3 id="work-with-github-issues-and-pull-requests">
  Trabajar con problemas y solicitudes de extracción de GitHub
</h3>

Las sesiones en la nube incluyen herramientas integradas de GitHub que permiten a Claude leer problemas, enumerar solicitudes de extracción, obtener diffs y publicar comentarios sin ninguna configuración. Estas herramientas se autentican a través del [proxy de GitHub](#github-proxy) utilizando cualquier método que configuró bajo [Opciones de autenticación de GitHub](/docs/es/claude-code-on-the-web#github-authentication-options), por lo que su token nunca entra en el contenedor.

Puede establecer `GH_TOKEN` o `GITHUB_TOKEN` usted mismo en [configuración de entorno](#set-environment-variables), o dejar ambos sin establecer y dejar que el [proxy de GitHub](#github-proxy) se autentique por usted:

* Si establece un token, pasa al contenedor sin cambios, por lo que sus scripts y el [`gh` CLI](https://cli.github.com) de GitHub lo usan directamente.
* Si no establece ninguno y el [proxy de GitHub](#github-proxy) está manejando la autenticación para su sesión, ambas variables se leen como la cadena de marcador de posición `proxy-injected` en los comandos que ejecuta Claude, y el proxy sustituye sus credenciales reales en solicitudes salientes de GitHub. `gh` funciona sin un token propio, pero un script que lee `GITHUB_TOKEN` directamente obtiene el marcador de posición, no un token utilizable.

Un token que establece es una variable de entorno ordinaria, por lo que cualquiera que use el entorno puede leerlo; la ruta del proxy mantiene la credencial fuera de la configuración del entorno y la VM de la sesión.

Para verificar qué caso se aplica a su sesión, pida a Claude que ejecute `echo $GH_TOKEN`.

El [`gh` CLI](https://cli.github.com) de GitHub está preinstalado. Si necesita un comando `gh` que las herramientas integradas no cubran, como `gh release` o `gh workflow run`, pida a Claude que lo ejecute. `gh` lee `GH_TOKEN` automáticamente, por lo que no necesita ejecutar `gh auth login`.

<h3 id="link-output-back-to-the-session">
  Vincular salida de nuevo a la sesión
</h3>

Cada sesión en la nube tiene una URL de transcripción en claude.ai, y la sesión puede leer su propio ID desde la variable de entorno `CLAUDE_CODE_REMOTE_SESSION_ID`. Úselo para poner un enlace rastreable en cuerpos de PR, mensajes de confirmación, publicaciones de Slack o informes generados para que un revisor pueda abrir la ejecución que los produjo.

Las confirmaciones que Claude crea en una sesión en la nube incluyen un remolque de git `Claude-Session: <url>`, y los cuerpos de PR incluyen la URL de la sesión en su propia línea. Para omitir el remolque y el enlace del cuerpo de PR, establezca [`attribution.sessionUrl`](/docs/es/settings-reference#attribution-sessionurl) en `false`.

Para incluir el enlace de la sesión en algo que no sea una confirmación o PR, como un mensaje de Slack que Claude publica o un archivo de informe que escribe, pida a Claude que ejecute el siguiente comando y use su salida. El comando convierte el prefijo `cse_` en el valor de la variable de entorno al prefijo `session_` que espera la URL de transcripción:

```bash theme={null}
echo "https://claude.ai/code/${CLAUDE_CODE_REMOTE_SESSION_ID/#cse_/session_}"
```

<h3 id="run-tests-start-services-and-add-packages">
  Ejecutar pruebas, iniciar servicios y agregar paquetes
</h3>

No obtiene un shell en la VM de la sesión. Claude ejecuta cada comando para usted, por lo que exprese las tareas en esta sección como solicitudes en su indicación.

<h4 id="run-tests">
  Ejecutar pruebas
</h4>

Claude ejecuta pruebas como parte del trabajo en una tarea. Solicítelo en su indicación, como "corregir las pruebas fallidas en `tests/`" o "ejecutar pytest después de cada cambio". Los ejecutores de pruebas que vienen con las [cadenas de herramientas preinstaladas](#installed-tools), como pytest y cargo test, funcionan sin configuración adicional. Un ejecutor que su proyecto declara como una dependencia, como jest, se instala con sus dependencias.

<h4 id="start-services">
  Iniciar servicios
</h4>

PostgreSQL y Redis están preinstalados pero no se ejecutan de forma predeterminada. Pida a Claude que inicie el que necesite; los comandos que ejecuta son:

```bash theme={null}
service postgresql start
```

```bash theme={null}
service redis-server start
```

Docker está disponible para ejecutar servicios en contenedores. Pida a Claude que ejecute `docker compose up` para iniciar los servicios de su proyecto. El acceso a la red para extraer imágenes sigue su [nivel de acceso](#access-levels) del entorno, y los [valores predeterminados Trusted](#default-allowed-domains) incluyen Docker Hub y otros registros comunes.

Si sus imágenes son grandes o lentas de extraer, agregue `docker compose pull` o `docker compose build` a su [script de configuración](#setup-scripts). El [almacenamiento en caché del entorno](#environment-caching) mantiene las imágenes extraídas, por lo que cada sesión nueva las tiene en el disco. El caché almacena solo archivos, no procesos en ejecución, por lo que Claude aún inicia los contenedores cada sesión.

<h4 id="add-packages">
  Agregar paquetes
</h4>

Para agregar paquetes que no están preinstalados, use un [script de configuración](#setup-scripts). El [almacenamiento en caché del entorno](#environment-caching) mantiene lo que instala el script, por lo que los paquetes que instala allí están disponibles al inicio de cada sesión sin reinstalar cada vez. También puede pedir a Claude que instale paquetes a mitad de sesión, pero esas instalaciones no se transfieren a otras sesiones.

<h3 id="resource-limits">
  Límites de recursos
</h3>

Las sesiones en la nube en entornos alojados por Anthropic se ejecutan con límites de recursos aproximados que pueden cambiar con el tiempo:

* 4 vCPU
* 16 GB de RAM
* 30 GB de disco

La VM puede detener tareas que necesitan significativamente más memoria, como trabajos de compilación grandes o pruebas que consumen mucha memoria. Para cargas de trabajo más allá de estos límites, use [Remote Control](/docs/es/remote-control) para ejecutar Claude Code en su propio hardware, o ejecute sesiones en la nube en un [entorno autohospedado](/docs/es/self-hosted-environments) en computación que su organización opera.

<h2 id="setup-scripts">
  Scripts de configuración
</h2>

Un script de configuración es un script Bash que se ejecuta cuando comienza una nueva sesión en la nube, antes de que se lance Claude Code. Use scripts de configuración para instalar dependencias, configurar herramientas o obtener cualquier cosa que la sesión necesite que no esté preinstalada.

Los scripts se ejecutan como root en Ubuntu 24.04, por lo que `apt install` y la mayoría de los administradores de paquetes de lenguaje funcionan.

Para agregar un script de configuración, abra el diálogo de configuración del entorno e ingrese su script en el campo **Setup script**.

Este ejemplo instala [ShellCheck](https://www.shellcheck.net/), que no está preinstalado.

```bash theme={null}
#!/bin/bash
apt update && apt install -y shellcheck
```

<h3 id="script-requirements">
  Requisitos del script
</h3>

Un script de configuración tiene tres restricciones para escribir:

* **Salir con cero**: si el script sale con un valor distinto de cero, la sesión no se inicia. Agregue `|| true` a comandos no críticos para que una falla de instalación intermitente no bloquee la sesión.
* **Terminar dentro de cinco minutos**: mantenga el tiempo de ejecución total del script por debajo de aproximadamente cinco minutos para que el [almacenamiento en caché del entorno](#environment-caching) pueda compilarse. Ejecute instalaciones independientes en paralelo con `&` y `wait`, y mueva cualquier descarga única que no quepa a un hook [SessionStart](#setup-scripts-vs-sessionstart-hooks) que lo inicie en segundo plano.
* **Acceso a la red para instalaciones**: las instalaciones de paquetes necesitan alcanzar registros. El nivel **Trusted** predeterminado cubre [registros de paquetes comunes](#default-allowed-domains) incluyendo npm, PyPI, RubyGems y crates.io; con acceso a la red **None**, las instalaciones fallan.

<h3 id="environment-caching">
  Almacenamiento en caché del entorno
</h3>

El script de configuración se ejecuta la primera vez que inicia una sesión en un entorno. Después de que se completa, Anthropic toma una instantánea del sistema de archivos y reutiliza esa instantánea como punto de partida para sesiones posteriores. Las nuevas sesiones comienzan con sus dependencias, herramientas e imágenes de Docker ya en el disco, y omiten el paso del script de configuración. Esto mantiene el inicio rápido incluso cuando el script instala cadenas de herramientas grandes o extrae imágenes de contenedor.

El caché es una instantánea del sistema de archivos, por lo que mantiene lo que el script de configuración escribe en el disco y pierde cualquier cosa que solo estaba en ejecución. Los paquetes que instala, las imágenes de Docker que extrae y los archivos que escribe se transfieren. Una base de datos que inició el script, una pila `docker compose up` o cualquier otro proceso en segundo plano no; inicie esos por sesión pidiendo a Claude o con un hook [SessionStart](#setup-scripts-vs-sessionstart-hooks).

El script de configuración se ejecuta nuevamente para reconstruir el caché cuando cambia el script de configuración del entorno u hosts de red permitidos, y cuando el caché alcanza su vencimiento después de aproximadamente siete días. Reanudar una sesión existente nunca vuelve a ejecutar el script de configuración.

No necesita habilitar el almacenamiento en caché ni administrar instantáneas usted mismo.

<h3 id="setup-scripts-vs-sessionstart-hooks">
  Scripts de configuración vs. hooks SessionStart
</h3>

Use un script de configuración para aprovisionar la VM misma: cadenas de herramientas y herramientas CLI que no están [preinstaladas](#installed-tools). Use un hook [SessionStart](/docs/es/hooks#sessionstart) para configuración del proyecto que debe ejecutarse en todas partes, en la nube y localmente, como `npm install`.

Los scripts de configuración y los hooks SessionStart se ejecutan en un orden fijo cuando comienza una sesión en la nube. La tabla compara dónde los configura, cuándo se ejecutan y dónde se ejecutan.

|                         | Scripts de configuración                                                                                                                                                                  | Hooks SessionStart                                                                                                                                                                                                                                         |
| ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Dónde los configura** | El diálogo de entorno en [claude.ai/code](https://claude.ai/code), más la página **Entornos en la nube** del administrador para [entornos compartidos](#organization-shared-environments) | Un [archivo de configuración](/docs/es/settings#where-settings-live) como su `.claude/settings.json` del repositorio; consulte [Qué se transfiere de su configuración](#what-carries-over-from-your-setup) para ver qué archivos llegan a una sesión en la nube |
| **Cuándo se ejecutan**  | Antes de que se lance Claude Code, omitido cuando existe un [entorno en caché](#environment-caching)                                                                                      | Después de que se lance Claude Code, en cada sesión incluyendo reanudadas                                                                                                                                                                                  |
| **Dónde se ejecutan**   | Solo sesiones en la nube                                                                                                                                                                  | Sesiones locales y en la nube                                                                                                                                                                                                                              |

Si tiene hooks SessionStart en su `~/.claude/settings.json` a nivel de usuario, no espere que estén en la nube. La configuración a nivel de usuario permanece en su máquina. Qué otros hooks se ejecutan depende de dónde se ejecute la sesión:

* **Entorno alojado por Anthropic**: Claude Code ejecuta hooks del repositorio y de la [configuración administrada por servidor](/docs/es/server-managed-settings) de su organización.
* **[Entorno autohospedado](/docs/es/self-hosted-environments-configuration#permissions-and-tool-approval)**: Claude Code también ejecuta los hooks que el operador sembró desde el `~/.claude/` del host del ejecutor, y los hooks en el archivo de configuración administrada de la imagen del ejecutor cuando ese archivo es uno de los [orígenes administrados que Claude Code aplica](/docs/es/managed-settings#how-claude-code-combines-managed-sources).

<h3 id="install-dependencies-with-a-sessionstart-hook">
  Instalar dependencias con un hook SessionStart
</h3>

Para instalar dependencias solo en sesiones en la nube, empareje un hook SessionStart con un script que verifique dónde se está ejecutando.

Primero, agregue un hook SessionStart a su `.claude/settings.json` del repositorio. Esta configuración le dice a Claude Code que ejecute `scripts/install_pkgs.sh` de su repositorio cada vez que comienza o se reanuda una sesión:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "startup|resume",
        "hooks": [
          {
            "type": "command",
            "command": "bash \"$CLAUDE_PROJECT_DIR\"/scripts/install_pkgs.sh"
          }
        ]
      }
    ]
  }
}
```

El `matcher` limita el hook a los eventos `startup` y `resume`, y `$CLAUDE_PROJECT_DIR` se resuelve a la raíz del repositorio, por lo que el hook encuentra el script independientemente del directorio de trabajo de la sesión.

A continuación, cree el script en `scripts/install_pkgs.sh`. Sale inmediatamente fuera de la nube, luego instala sus dependencias:

```bash theme={null}
#!/bin/bash

if [ "$CLAUDE_CODE_REMOTE" != "true" ]; then
  exit 0
fi

npm install
pip install -r requirements.txt
exit 0
```

La verificación `CLAUDE_CODE_REMOTE` es lo que limita la instalación a sesiones en la nube: la VM de la sesión lleva esa variable como `true`, nunca es `true` localmente, por lo que en su portátil el script sale antes de instalar cualquier cosa.

Juntos, los dos archivos dan a cada sesión en la nube un `npm install` y `pip install` nuevo al inicio mientras dejan las sesiones locales sin tocar.

<h4 id="limitations-in-cloud-sessions">
  Limitaciones en sesiones en la nube
</h4>

Los hooks SessionStart se comportan igual en la nube que localmente, con estas advertencias:

* **Un repositorio por sesión**: una sesión con varios repositorios no carga hooks de ningún `.claude/settings.json` del repositorio, por lo que un hook SessionStart que defina allí no se ejecuta. Instale dependencias para esas sesiones con un [script de configuración](#setup-scripts) en su lugar.
* **Sin alcance solo en la nube**: los hooks se ejecutan en sesiones locales y en la nube. Para omitir la ejecución local, salga temprano a menos que la variable de entorno `CLAUDE_CODE_REMOTE` sea `true`, de la manera que lo hace el [script de instalación de dependencias](#install-dependencies-with-a-sessionstart-hook).
* **Requiere acceso a la red**: los comandos de instalación necesitan alcanzar registros de paquetes. Si su entorno usa acceso a la red **None**, estos hooks fallan. La [lista permitida predeterminada](#default-allowed-domains) bajo **Trusted** cubre npm, PyPI, RubyGems y crates.io.
* **Compatibilidad con proxy**: en entornos alojados por Anthropic, todo el tráfico saliente pasa por un [proxy de seguridad](#security-proxy), y algunos administradores de paquetes no funcionan correctamente con él; Bun es un ejemplo conocido. En un [entorno autohospedado](/docs/es/self-hosted-environments-deploy#default-deny-egress), el tráfico saliente va a través de su propio límite de red en su lugar.
* **Agrega latencia de inicio**: los hooks se ejecutan cada vez que comienza o se reanuda una sesión, a diferencia de los scripts de configuración que se benefician del [almacenamiento en caché del entorno](#environment-caching). Mantenga los scripts de instalación rápidos verificando si las dependencias ya están presentes antes de reinstalar.

Para personalizar la imagen base, use un script de configuración para instalar lo que necesita encima de la [imagen proporcionada](#installed-tools), o ejecute su propia imagen como un contenedor junto a Claude con `docker compose`. Reemplazar la imagen base completamente aún no es compatible.

<h2 id="default-allowed-domains">
  Dominios permitidos predeterminados
</h2>

Con acceso a la red **Trusted**, las sesiones pueden alcanzar los siguientes dominios de forma predeterminada. Los dominios marcados con `*` indican coincidencia de subdominio comodín, por lo que `*.gcr.io` permite cualquier subdominio de `gcr.io`.

<AccordionGroup>
  <Accordion title="Servicios de Anthropic">
    * api.anthropic.com
    * docs.claude.com
    * platform.claude.com
    * code.claude.com
    * claude.ai
  </Accordion>

  <Accordion title="Control de versiones">
    * github.com
    * [www.github.com](http://www.github.com)
    * api.github.com
    * npm.pkg.github.com
    * raw\.githubusercontent.com
    * pkg-npm.githubusercontent.com
    * objects.githubusercontent.com
    * release-assets.githubusercontent.com
    * codeload.github.com
    * avatars.githubusercontent.com
    * camo.githubusercontent.com
    * gist.github.com
    * gitlab.com
    * [www.gitlab.com](http://www.gitlab.com)
    * registry.gitlab.com
    * bitbucket.org
    * [www.bitbucket.org](http://www.bitbucket.org)
    * api.bitbucket.org
  </Accordion>

  <Accordion title="Registros de contenedores">
    * registry-1.docker.io
    * auth.docker.io
    * index.docker.io
    * hub.docker.com
    * [www.docker.com](http://www.docker.com)
    * production.cloudflare.docker.com
    * download.docker.com
    * gcr.io
    * \*.gcr.io
    * ghcr.io
    * mcr.microsoft.com
    * \*.data.mcr.microsoft.com
    * public.ecr.aws
  </Accordion>

  <Accordion title="Plataformas en la nube">
    * cloud.google.com
    * accounts.google.com
    * gcloud.google.com
    * \*.googleapis.com
    * storage.googleapis.com
    * compute.googleapis.com
    * container.googleapis.com
    * azure.com
    * portal.azure.com
    * microsoft.com
    * [www.microsoft.com](http://www.microsoft.com)
    * \*.microsoftonline.com
    * packages.microsoft.com
    * dotnet.microsoft.com
    * dot.net
    * visualstudio.com
    * dev.azure.com
    * \*.amazonaws.com
    * \*.api.aws
    * oracle.com
    * [www.oracle.com](http://www.oracle.com)
    * java.com
    * [www.java.com](http://www.java.com)
    * java.net
    * [www.java.net](http://www.java.net)
    * download.oracle.com
    * yum.oracle.com
    * \*.r2.cloudflarestorage.com
  </Accordion>

  <Accordion title="Administradores de paquetes JavaScript y Node">
    * registry.npmjs.org
    * [www.npmjs.com](http://www.npmjs.com)
    * [www.npmjs.org](http://www.npmjs.org)
    * npmjs.com
    * npmjs.org
    * yarnpkg.com
    * registry.yarnpkg.com
    * jsr.io
    * npm.jsr.io
  </Accordion>

  <Accordion title="Administradores de paquetes Python">
    * pypi.org
    * [www.pypi.org](http://www.pypi.org)
    * files.pythonhosted.org
    * pythonhosted.org
    * test.pypi.org
    * pypi.python.org
    * pypa.io
    * [www.pypa.io](http://www.pypa.io)
  </Accordion>

  <Accordion title="Administradores de paquetes Ruby">
    * rubygems.org
    * [www.rubygems.org](http://www.rubygems.org)
    * api.rubygems.org
    * index.rubygems.org
    * ruby-lang.org
    * [www.ruby-lang.org](http://www.ruby-lang.org)
    * rubyforge.org
    * [www.rubyforge.org](http://www.rubyforge.org)
    * rubyonrails.org
    * [www.rubyonrails.org](http://www.rubyonrails.org)
    * rvm.io
    * get.rvm.io
  </Accordion>

  <Accordion title="Administradores de paquetes Rust">
    * crates.io
    * [www.crates.io](http://www.crates.io)
    * index.crates.io
    * static.crates.io
    * rustup.rs
    * static.rust-lang.org
    * [www.rust-lang.org](http://www.rust-lang.org)
  </Accordion>

  <Accordion title="Administradores de paquetes Go">
    * proxy.golang.org
    * sum.golang.org
    * index.golang.org
    * golang.org
    * [www.golang.org](http://www.golang.org)
    * goproxy.io
    * pkg.go.dev
  </Accordion>

  <Accordion title="Administradores de paquetes JVM">
    * maven.org
    * repo.maven.org
    * central.maven.org
    * repo1.maven.org
    * repo.maven.apache.org
    * maven.google.com
    * jcenter.bintray.com
    * gradle.org
    * [www.gradle.org](http://www.gradle.org)
    * services.gradle.org
    * plugins.gradle.org
    * plugins-artifacts.gradle.org
    * kotlinlang.org
    * [www.kotlinlang.org](http://www.kotlinlang.org)
    * spring.io
    * repo.spring.io
  </Accordion>

  <Accordion title="Otros administradores de paquetes">
    * packagist.org (PHP Composer)
    * [www.packagist.org](http://www.packagist.org)
    * repo.packagist.org
    * nuget.org (.NET NuGet)
    * [www.nuget.org](http://www.nuget.org)
    * api.nuget.org
    * pub.dev (Dart/Flutter)
    * api.pub.dev
    * hex.pm (Elixir/Erlang)
    * [www.hex.pm](http://www.hex.pm)
    * cpan.org (Perl CPAN)
    * [www.cpan.org](http://www.cpan.org)
    * metacpan.org
    * [www.metacpan.org](http://www.metacpan.org)
    * api.metacpan.org
    * cocoapods.org (iOS/macOS)
    * [www.cocoapods.org](http://www.cocoapods.org)
    * cdn.cocoapods.org
    * haskell.org
    * [www.haskell.org](http://www.haskell.org)
    * hackage.haskell.org
    * swift.org
    * [www.swift.org](http://www.swift.org)
  </Accordion>

  <Accordion title="Distribuciones de Linux">
    * archive.ubuntu.com
    * security.ubuntu.com
    * ubuntu.com
    * [www.ubuntu.com](http://www.ubuntu.com)
    * \*.ubuntu.com
    * ppa.launchpad.net
    * launchpad.net
    * [www.launchpad.net](http://www.launchpad.net)
    * \*.nixos.org
  </Accordion>

  <Accordion title="Herramientas de desarrollo y plataformas">
    * dl.k8s.io (Kubernetes)
    * pkgs.k8s.io
    * k8s.io
    * [www.k8s.io](http://www.k8s.io)
    * releases.hashicorp.com (HashiCorp)
    * apt.releases.hashicorp.com
    * rpm.releases.hashicorp.com
    * archive.releases.hashicorp.com
    * hashicorp.com
    * [www.hashicorp.com](http://www.hashicorp.com)
    * repo.anaconda.com (Anaconda/Conda)
    * conda.anaconda.org
    * anaconda.org
    * [www.anaconda.com](http://www.anaconda.com)
    * anaconda.com
    * continuum.io
    * apache.org (Apache)
    * [www.apache.org](http://www.apache.org)
    * archive.apache.org
    * downloads.apache.org
    * eclipse.org (Eclipse)
    * [www.eclipse.org](http://www.eclipse.org)
    * download.eclipse.org
    * nodejs.org (Node.js)
    * [www.nodejs.org](http://www.nodejs.org)
    * developer.apple.com
    * developer.android.com
    * pkg.stainless.com
    * binaries.prisma.sh
  </Accordion>

  <Accordion title="Servicios en la nube y monitoreo">
    * http-intake.logs.datadoghq.com
    * \*.datadoghq.com
    * \*.datadoghq.eu
    * api.honeycomb.io
  </Accordion>

  <Accordion title="Entrega de contenido y espejos">
    * sourceforge.net
    * \*.sourceforge.net
    * packagecloud.io
    * \*.packagecloud.io
    * fonts.googleapis.com
    * fonts.gstatic.com
  </Accordion>

  <Accordion title="Esquema y configuración">
    * json-schema.org
    * [www.json-schema.org](http://www.json-schema.org)
    * json.schemastore.org
    * [www.schemastore.org](http://www.schemastore.org)
  </Accordion>

  <Accordion title="Model Context Protocol">
    * \*.modelcontextprotocol.io
  </Accordion>
</AccordionGroup>

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Cloud sessions reference](/docs/es/claude-code-on-the-web): inicie, administre y comparta sesiones en la nube
* [Cloud sessions quickstart](/docs/es/web-quickstart): conecte GitHub e inicie su primera sesión en la nube
* [Claude Tag](https://claude.com/docs/claude-tag/overview): las sesiones que Claude inicia desde Slack se ejecutan en los mismos entornos
* [Routines](/docs/es/routines): las ejecuciones programadas utilizan los mismos entornos y niveles de acceso a la red
* [Remote Control](/docs/es/remote-control): ejecute sesiones en la red y archivos de su propia máquina en su lugar
* [Entornos autohospedados](/docs/es/self-hosted-environments): ejecute sesiones en la nube en la infraestructura propia de su organización
* [SessionStart hooks](/docs/es/hooks#sessionstart): configuración confirmada en el repositorio que se ejecuta en sesiones locales y en la nube
* [Configuración administrada por servidor](/docs/es/server-managed-settings): política de la organización que llega a sesiones en la nube
