> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Implementar entornos autohospedados en producción

> Ejecutar runners autohospedados en producción: endurecimiento de seguridad, control de salida de red, credenciales de git, recetas de Kubernetes y Compose, y solución de problemas.

<Note>
  Los entornos autohospedados están en beta pública en planes Team y Enterprise; [Disponibilidad y limitaciones](/docs/es/self-hosted-environments#availability-and-limitations) cubre la ruta de habilitación. Esta página cubre la ejecución de la flota en producción; consulte el [inicio rápido](/docs/es/self-hosted-environments-quickstart) para su primer runner y sesión.
</Note>

Un [entorno autohospedado](/docs/es/self-hosted-environments) ejecuta [sesiones en la nube](/docs/es/claude-code-on-the-web) de Claude Code en runners que usted implementa dentro de su red, y en producción esas sesiones ejecutan código dirigido por el modelo en nombre de todos los que pueden enviar una sesión al entorno. Esta página es para el operador que lleva un entorno funcional a producción. Funciona a través de la implementación en orden: qué bloquear antes de conectar sistemas reales, la salida que necesita la flota, cómo las sesiones se autentican en su host de git, las recetas de implementación en sí, y qué verificar cuando las sesiones se comportan mal.

<h2 id="harden-your-deployment">
  Endurezca su implementación
</h2>

Un runner autohospedado ejecuta código arbitrario dirigido por el modelo en su infraestructura en nombre de todos los que pueden enviar una sesión a su entorno. Eso es cualquier miembro de su organización de Anthropic, y cualquiera que pueda iniciar una sesión de canal de [Claude Tag](https://claude.com/docs/claude-tag/overview) en un alcance que un Propietario enrutó al entorno. Trabaje en cada elemento antes de conectar un entorno a sistemas de producción:

* **Contenedores efímeros por sesión**: ejecute cada proceso de runner en un contenedor o VM nuevo que se destruya cuando el proceso salga, con `--capacity 1` y el `--drain-grace-sec 0` predeterminado para que cada contenedor sirva exactamente una sesión. Con una capacidad más alta, o con un drenaje de gracia positivo, un contenedor sirve múltiples sesiones del mismo [propietario bloqueado](/docs/es/self-hosted-environments#key-concepts); consulte [Ciclo de vida del runner](/docs/es/self-hosted-environments#runner-lifecycle). No reutilice un sistema de archivos entre reinicios de runner, excepto en la configuración deliberada de [checkout precalentado](#reuse-a-pre-warmed-checkout), y nunca entre propietarios.
* **Sin credenciales amplias en la imagen**: no incluya claves SSH de larga duración, credenciales de proveedor de nube, o tokens de acceso personal que otorguen más de lo que una sesión necesita. Genere credenciales utilizadas durante una sesión, como tokens de push o API, por sesión desde su [script de envoltura](/docs/es/self-hosted-environments-configuration#wrapper-scripts). Para el clon inicial, que ocurre antes de que se ejecute el envoltura, use un [hook de ciclo de vida `checkout`](/docs/es/self-hosted-environments-configuration#checkout) o [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy); consulte [Configurar git](#configure-git).
* **Mantenga el secreto del entorno fuera de los hosts que ejecutan sesiones**: el secreto del entorno puede registrar runners y recoger cualquier sesión en cola en el entorno. En una flota fija vive en cada host de runner, donde el código de cualquier sesión puede leer el archivo secreto. Prefiera [runners bajo demanda](/docs/es/self-hosted-environments-configuration#on-demand-runners), donde el secreto permanece en el host del orquestador, que nunca ejecuta código de usuario, y cada runner recibe una orden de trabajo de un solo uso que registra exactamente un runner. En una flota fija, trate el archivo de secreto del entorno como legible por cada sesión y rote el secreto después de cualquier compromiso de sesión sospechoso.
* **Salida de red de negación predeterminada**: restrinja el tráfico saliente del contenedor de runner y sesión en su propio límite de red en cada entorno; [Salida de negación predeterminada](#default-deny-egress) cubre qué permitir y por qué.
* **IAM de host con privilegios mínimos**: la identidad de cálculo adjunta al host del runner, como un perfil de instancia o una cuenta de servicio de nodo, debe otorgar solo lo que el runner en sí necesita. Las sesiones deben obtener sus propias credenciales a través de su script de envoltura en lugar de heredar las del host.
* **Bloquee el punto final de metadatos en la nube desde las sesiones**: mantener las sesiones fuera de la identidad del host requiere bloquear su acceso al punto final de metadatos en la nube, y las políticas de salida a nivel de subred no interceptan el tráfico de metadatos de enlace local, así que bloquéelo en el contenedor en sí:

  * IMDSv2 con un límite de salto de uno
  * GKE Workload Identity con ocultamiento de metadatos
  * Una denegación explícita para `169.254.169.254` en el espacio de nombres de red del contenedor de sesión

  El bloque se aplica a su script de envoltura y hooks de ciclo de vida también, ya que comparten el contenedor. Autentique cualquier intercambio de token con el [JWT de sesión](/docs/es/self-hosted-environments-identity) contra su propio servicio de token sobre salida permitida, o use una identidad web basada en archivos como IAM Roles for Service Accounts (IRSA) en Amazon EKS.
* **Aislamiento del sistema de archivos por runner**: cada proceso de runner obtiene su propio directorio de trabajo que ningún otro proceso en el host puede leer o escribir. Haga `--hooks-dir`, el script de envoltura, y el `~/.claude/` del host de solo lectura para la sesión, ya sea integrado en la imagen o montado como solo lectura.
* **El envío no tiene control de acceso por entorno**: cualquier miembro de su organización de Anthropic puede enviar una sesión a cualquiera de sus entornos. Si un Propietario [enruta canales de Claude Tag al entorno](/docs/es/cloud-environments#set-the-environment-a-claude-tag-channel-uses), cualquiera que la [configuración de acceso de Claude Tag](https://claude.com/docs/claude-tag/admins/restrict-access#restrict-who-can-use-claude) admita puede iniciar sesiones de canal que se ejecuten allí. Por defecto, eso es cualquiera en el espacio de trabajo de Slack conectado, con o sin una cuenta de Claude. Trate cada host de runner como alcanzable para la ejecución de código por todos los que pueden enviar al mismo, y coloque en un host de runner solo datos y credenciales que todas esas personas pueden leer. [`--lock-to-account`](/docs/es/self-hosted-environments-reference#runner-cli-flags) limita qué sesiones de cuenta ejecuta un host determinado, pero no reduce quién puede enviar al entorno. Para hacer que los entornos autohospedados sean la única opción de selector, un [Propietario](/docs/es/cloud-environments#organization-shared-environments) puede ocultar entornos alojados por Anthropic para toda la organización desde la página [**Entornos en la nube**](https://claude.ai/admin-settings/cloud-environments).
* **Aplique la protección de configuración del repositorio**: elija el modo de protección con [`--confine-repo-settings`](/docs/es/self-hosted-environments-reference#runner-cli-flags). El `warn` predeterminado registra una violación y aún genera la sesión, `enforce` rechaza la sesión, y `off` desactiva el escaneo. El runner escanea la configuración comprometida de cada repositorio para:

  * Una concesión que se resuelve fuera del espacio de trabajo de esa sesión: una entrada `additionalDirectories`, una regla `Edit`, `Write`, o `NotebookEdit` en `permissions.allow`, o una entrada `sandbox.filesystem.allowWrite` o `allowRead`
  * Un bloque `env` no vacío
  * Una anulación de postura del operador como `sandbox.enabled: false`

  La protección se ejecuta independientemente de [`--trust-workspace`](/docs/es/self-hosted-environments-reference#runner-cli-flags), y no cubre hooks de repositorio, `.mcp.json`, o reglas de Bash; consulte [Permisos y aprobación de herramientas](/docs/es/self-hosted-environments-configuration#permissions-and-tool-approval) para saber dónde pertenecen esas concesiones.

<Note>
  La lista de permitidos de IP de su organización no cubre el tráfico de runner autohospedado por defecto. No confíe en ella como control de red para el tráfico de runner o sesión; aplique salida de negación predeterminada en su propio límite de red en su lugar, y contacte a su equipo de cuenta de Anthropic si desea aplicación de lista de permitidos de IP para su organización.
</Note>

<h2 id="network-requirements">
  Requisitos de red
</h2>

El runner y los hijos de sesión que genera hacen conexiones salientes a los hosts a continuación. Restrinja la salida del contenedor de sesión a estos hosts y los servicios internos específicos que las sesiones necesitan alcanzar; [Salida de negación predeterminada](#default-deny-egress) cubre cómo y por qué.

Estos hosts siempre son requeridos:

| Host                                                             | Puerto                                    | Utilizado para                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :--------------------------------------------------------------- | :---------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `api.anthropic.com`                                              | 443, HTTPS; WSS solo para el conector SCM | Plano de control del runner y transmisión de sesión, inferencia de modelo, banderas de características, análisis de productos, obtenciones de claves [JWKS](/docs/es/self-hosted-environments-identity), firma de commits, el proxy de git cuando se establece `--use-anthropic-git-proxy`, y el túnel del [conector SCM](/docs/es/self-hosted-environments-reference#scm-connector-flags) del orquestador cuando se establece `--scm-connector-host` |
| Su host de git, como `github.com` o su host de GitHub Enterprise | 443 o 22                                  | Clonación e inserción de repositorios. No es necesario si el runner usa `--use-anthropic-git-proxy`, que enruta el tráfico de git a través de `api.anthropic.com`.                                                                                                                                                                                                                                                                          |

Si estos hosts son necesarios depende de su configuración:

| Host                                 | Puerto | Cuando es requerido                                                                                                                                                                                                                                                                                                                                                         |
| :----------------------------------- | :----- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `downloads.claude.ai`                | 443    | En el momento de la instalación, cuando instala o actualiza Claude Code en el host con el instalador nativo; el script `install.sh` en sí se sirve desde `claude.ai`. En tiempo de ejecución de sesión, solo cuando las sesiones instalan plugins del mercado oficial de Anthropic.                                                                                         |
| `storage.googleapis.com`             | 443    | En tiempo de ejecución de sesión, para los recuentos de instalación de plugins y metadatos mostrados en `/plugin`.                                                                                                                                                                                                                                                          |
| `code.claude.com` y `claude.com`     | 443    | Búsquedas de documentación por el agente integrado claude-code-guide y solicitudes de WebFetch preaprobadas durante sesiones. Bloquear estos hosts solo afecta las búsquedas de documentación.                                                                                                                                                                              |
| `*.frame.claudeusercontent.com`      | 443    | Solo cuando la [herramienta Artifact](/docs/es/artifacts#availability) está disponible para sesiones en su organización; los valores predeterminados varían según el plan, según la tabla de disponibilidad allí. Establezca `CLAUDE_CODE_DISABLE_ARTIFACT=1` en el runner para mantener la herramienta deshabilitada independientemente de la configuración de la organización. |
| `registry.npmjs.org`                 | 443    | Cuando una sesión instala un plugin, tanto para obtener paquetes de plugins de origen npm como para instalar las dependencias de Node.js de un plugin, o cuando se ejecuta un servidor MCP lanzado por `npx`                                                                                                                                                                |
| `http-intake.logs.us5.datadoghq.com` | 443    | Métricas operacionales de Anthropic. Solo cuando se establece `CLAUDE_CODE_BYOC_ENABLE_DATADOG=1`; desactivado por defecto en entornos autohospedados.                                                                                                                                                                                                                      |
| `browser-intake-us5-datadoghq.com`   | 443    | Cargas de informes de errores de Anthropic, enviadas solo cuando [informe de errores](/docs/es/data-usage#telemetry-services) está habilitado para la cuenta de la sesión. Suprimido por `DISABLE_ERROR_REPORTING=1` o `DISABLE_TELEMETRY=1`.                                                                                                                                    |

El runner no alcanza `statsig.anthropic.com`, `*.sentry.io`, `claude.ai`, o `platform.claude.com`. Estos hosts aparecen en algunas listas de verificación de red empresarial más antiguas, pero no necesita permitirlos para el tráfico de runner o sesión: las obtenciones de banderas de características van a `api.anthropic.com`, y el runner se autentica con el secreto del entorno en lugar de OAuth interactivo. Dos flujos del lado del host alcanzan `claude.ai`, así que ejecútelos desde un host cuya salida permite en lugar de ampliar la salida del contenedor de sesión: el instalador de una línea obtiene `install.sh` de `claude.ai` en el momento de la instalación, e interactivo `claude auth login`, que el [configuración guiada](/docs/es/self-hosted-environments-quickstart#set-up-an-environment-and-runner), el modo firmado del `doctor`, y [envío de CI](/docs/es/self-hosted-environments-testing#authenticate-from-ci) usan, inicia sesión a través de `claude.ai`, `claude.com`, y `platform.claude.com`. `mcp-proxy.anthropic.com` tampoco es requerido: las sesiones autohospedadas no lo usan, y la entrega de los conectores de claude.ai de su organización a sesiones, cuando está habilitada para su organización, se enruta a través de `api.anthropic.com`. Consulte [Servidores MCP](/docs/es/self-hosted-environments-configuration#mcp-servers).

<h3 id="default-deny-egress">
  Salida de negación predeterminada
</h3>

Implemente contenedores de runner y sesión en un segmento de red o espacio de nombres cuyo tráfico saliente se limita a los hosts en la [tabla de requisitos de red](#network-requirements), su host de git, y los servicios internos específicos que las sesiones necesitan alcanzar. El producto no puede verificar o aplicar esto, así que aplíquelo en su propio límite de red en cada entorno. El código de sesión está dirigido por el modelo e intenta conexiones a hosts arbitrarios; la salida de negación predeterminada a nivel de red limita dónde esos intentos pueden aterrizar. Esto se aplica independientemente del modo de permiso: el conjunto de herramientas preaprobadas predeterminadas ya incluye `Bash`, así que la salida de shell se ejecuta sin un aviso incluso sin [modo automático](/docs/es/self-hosted-environments-configuration#permissions-and-tool-approval).

Para detalles sobre qué telemetría emite cada sesión y cómo desactivarla, consulte [Telemetría](/docs/es/self-hosted-environments-reference#telemetry).

<h3 id="authenticate-to-an-egress-proxy">
  Autentíquese en un proxy de salida
</h3>

Algunos proxies de salida corporativos requieren un encabezado `Proxy-Authorization` en cada conexión. El token en ese encabezado a menudo rota demasiado rápido para escribir en la URL del proxy que establece en `HTTPS_PROXY`. Establezca `HTTPS_PROXY` o `HTTP_PROXY` en la URL de su proxy como de costumbre, luego establezca `--proxy-authorization-command` o `--proxy-authorization-file` para indicar al runner dónde leer el valor del encabezado. Ambas banderas requieren Claude Code v2.1.238 o posterior.

<h4 id="choose-where-the-proxy-authorization-value-comes-from">
  Elija de dónde viene el valor `Proxy-Authorization`
</h4>

Elija la bandera que coincida con cómo produce el token `Proxy-Authorization`:

* **[`--proxy-authorization-command <command>`](/docs/es/self-hosted-environments-reference#runner-cli-flags)**: elija esto para un token que genera bajo demanda. El runner ejecuta el comando de shell y usa su stdout recortado como el valor del encabezado, por ejemplo `Bearer <token>`.
* **[`--proxy-authorization-file <path>`](/docs/es/self-hosted-environments-reference#runner-cli-flags)**: elija esto para un token que otro proceso rota en su lugar. El runner lee el archivo y usa su contenido recortado como el valor del encabezado.

<h4 id="configurations-the-runner-refuses-to-start-with">
  Configuraciones que el runner se niega a iniciar con
</h4>

Cada bandera también tiene una forma de variable de entorno, listada junto a ella en la [referencia de banderas CLI del runner](/docs/es/self-hosted-environments-reference#runner-cli-flags). Antes de que el runner contacte su proxy o el plano de control, verifica las banderas y sus variables, y se niega a iniciar en tres casos:

* **Ambas banderas establecidas**: una bandera más la variable de entorno de la otra bandera cuenta como establecer ambas.
* **Sin URL de proxy**: ni `HTTPS_PROXY` ni `HTTP_PROXY` contiene una URL `http://` o `https://`. El runner lee ambas variables en mayúsculas o minúsculas, y no consulta `ALL_PROXY`.
* **Cualquiera de las banderas pasadas al subcomando del orquestador**: `self-hosted-runner orchestrator` no acepta las banderas o sus variables de entorno. Pase la bandera a cada runner que inicia el orquestador en su lugar.

<h4 id="what-the-runner-changes-while-a-proxy-authorization-flag-is-set">
  Lo que el runner cambia mientras se establece una bandera de autorización de proxy
</h4>

Con cualquiera de las banderas establecidas, el runner inicia un oyente propio y envía tráfico de proxy desde sí mismo, sus hooks de ciclo de vida, y sus sesiones a través de ese oyente. El oyente agrega el encabezado `Proxy-Authorization` en el camino a su proxy.

* **Oyente**: el oyente es un proxy directo en `127.0.0.1`. El runner inicia el oyente antes de registrarse con el plano de control, y sale al inicio si el oyente no puede iniciar.
* **Variables de proxy**: el runner reescribe cualquiera de `HTTPS_PROXY` e `HTTP_PROXY` que establezca para que apunte al oyente. Ese valor reescrito alcanza el runner en sí, sus hooks de ciclo de vida, y cada sesión que ejecuta.
* **Rotación de token**: un token rotado entra en vigor sin un reinicio. Para cada conexión que el oyente abre a su proxy, el runner ejecuta su comando o lee su archivo nuevamente y agrega el resultado como el encabezado.
* **Entorno de sesión**: una sesión alcanza su proxy solo a través del oyente. En el entorno de cada sesión, el runner elimina `ALL_PROXY`, elimina cualquier ortografía de `HTTPS_PROXY` o `HTTP_PROXY` que no haya establecido, y fija `NO_PROXY` al valor propio del runner.
* **Registros**: el runner nunca registra el valor del encabezado.

<h2 id="configure-git">
  Configurar git
</h2>

El runner gestiona checkouts de repositorio pero no configura la identidad de git o credenciales por defecto. Usted controla la imagen y el entorno de proceso del runner, así que controla la configuración de git. Elija uno de dos enfoques:

* **Deje que el runner configure git**: inicie el runner con `--configure-git` para que escriba la misma identidad y configuración de firma de commit que usan las sesiones alojadas por Anthropic
* **Envíe la configuración de git en su imagen**: establezca la identidad y las credenciales de push usted mismo, por ejemplo para hacer commits bajo su propia identidad de bot

Pisos de versión de Git en el host del runner: [`--configure-git`](#let-the-runner-configure-git) la firma de commit SSH requiere Git 2.34 o más reciente, [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy) requiere 2.32 o más reciente, y reanudar sesiones desde ramas empujadas por [`--push-outcome-on-release`](/docs/es/self-hosted-environments-reference#runner-cli-flags) requiere 2.29 o más reciente. Git 2.24 es suficiente si omite los tres y gestiona la identidad de git usted mismo.

<h3 id="let-the-runner-configure-git">
  Deje que el runner configure git
</h3>

Inicie el runner con `--configure-git`, o establezca `SELF_HOSTED_RUNNER_CONFIGURE_GIT=1`, para que escriba la configuración global de git al inicio:

* `user.name = Claude` y `user.email = noreply@anthropic.com`, coincidiendo con sesiones alojadas por Anthropic
* Firma de commit y etiqueta en formato SSH, enrutada a través de un shim gestionado por el runner que firma cada commit a través del servicio de firma de Anthropic usando las credenciales de la sesión. Las firmas son verificables en GitHub contra la clave de firma SSH publicada de Anthropic.
* `push.negotiate = true`, para que git pregunte a su host de git qué commits ya tiene antes de empacar un push. Requiere Claude Code v2.1.257 o posterior.
* `core.hooksPath` apuntando a un directorio de hooks gestionado por el runner. Sus hooks `commit-msg` y `prepare-commit-msg` añaden un tráiler `Co-authored-by:` para el creador de la sesión a cada commit, construido a partir del correo electrónico en [`CCR_SESSION_ACCOUNT_EMAIL`](/docs/es/self-hosted-environments-configuration#wrapper-scripts) y omitido cuando esa variable no está establecida. Si su imagen ya establece `core.hooksPath`, el runner deja su configuración en su lugar, omite instalar estos hooks, e imprime una advertencia `[runner:git]`.

La firma de commit requiere git 2.34 o más reciente; el runner verifica al inicio y sale con un error si su git es más antiguo. Esta bandera no configura credenciales de push, que aún proporciona en la imagen.

<h3 id="ship-git-config-in-your-image">
  Envíe la configuración de git en su imagen
</h3>

La identidad de Git es requerida para cualquier commit. Establézcala a nivel del sistema en su Dockerfile para que la configuración se aplique independientemente de qué usuario ejecute el proceso del runner:

```dockerfile theme={null}
RUN git config --system user.name "Claude" && \
    git config --system user.email "noreply@anthropic.com"
```

Sin una identidad, `git commit` falla con `Please tell me who you are` y las sesiones no pueden progresar. Puede usar su propia identidad de bot en su lugar; el runner no anula estos valores.

No hornee credenciales de push de larga duración o ampliamente alcanzadas en una imagen de runner compartida: una credencial en la imagen está disponible para cada sesión que ejecuta la imagen, quienquiera que la haya iniciado. En su lugar, genere un token de corta duración y alcance mínimo por sesión desde su [script de envoltura](/docs/es/self-hosted-environments-configuration#wrapper-scripts), usando la identidad del creador de la sesión decodificada del JWT de la sesión. Emparéjelo con un contenedor efímero por sesión, que requiere `--capacity 1`, para que ninguna credencial sobreviva a la sesión que la generó; consulte la [sección de endurecimiento](#harden-your-deployment).

Si debe configurar credenciales de push a nivel de imagen, por ejemplo para una clave de implementación de solo lectura, limítelas lo más posible que su host de git permita:

* Una clave de implementación SSH limitada a un repositorio con una reescritura `url.<base>.insteadOf`
* Un `credential.helper` que devuelve un token de alcance mínimo
* `GIT_SSH_COMMAND` apuntando a una clave de alcance estrecho

Cualquier mecanismo que configure debe funcionar sin un aviso, porque el clon integrado del runner y la obtención desactivan los avisos que git, SSH, y Git Credential Manager mostrarían de otra manera:

* El runner establece `GIT_TERMINAL_PROMPT=0`, por lo que git no pide un nombre de usuario o contraseña.
* El runner ejecuta SSH con `BatchMode=yes`, añadido a su `GIT_SSH_COMMAND` si establece uno, por lo que SSH no pide una frase de contraseña o confirmación de host.
* El runner establece `GCM_INTERACTIVE=never`, por lo que Git Credential Manager no abre un diálogo de inicio de sesión.
* El runner borra `core.askPass`, así que si usa un ayudante askpass, establézcalo a través de la variable de entorno `GIT_ASKPASS` en su lugar.

Si su host de git rechaza la credencial, o no configuró una, el runner reintenta algunas veces y luego falla la preparación del repositorio cuando el repositorio es el que la sesión empuja resultados a. Para un repositorio que la sesión solo lee, [Troubleshooting](#troubleshooting) cubre cuándo el runner lo omite en su lugar. El runner no pasa estas configuraciones al entorno de la sesión.

Si los directorios de checkout son propiedad de un uid diferente al del proceso del runner, git se niega a operar en ellos; agregue `safe.directory`:

```dockerfile theme={null}
RUN git config --system --add safe.directory '*'
```

<h3 id="use-the-anthropic-git-proxy">
  Use el proxy de git de Anthropic
</h3>

Inicie el runner con `--use-anthropic-git-proxy`, o establezca `CLAUDE_RUNNER_USE_GIT_PROXY=1`, para que clone a través del proxy de git de Anthropic, autenticado con el token de corta duración de la sesión. Para sesiones de usuario ordinarias, el proxy usa el token OAuth de GitHub o GitHub Enterprise almacenado para el creador de la sesión; para sesiones de bot y agente, usa el token de instalación de GitHub App de su organización. De cualquier manera, la imagen del runner no necesita credenciales de git en absoluto: sin claves SSH, sin ayudante de credenciales, sin `.netrc`. Esta es la misma ruta de autenticación que usan los entornos alojados por Anthropic.

El proxy requiere `--capacity 1` porque la URL del proxy es por sesión, y git 2.32 o más reciente porque git más antiguo ignora el mecanismo de configuración que el proxy usa para aislar sesiones entre sí. El runner se niega a iniciar si alguno de los requisitos no se cumple. Porque el proxy obtiene del lado de Anthropic, su host de git debe ser alcanzable desde la infraestructura de Anthropic, el mismo requisito que tienen las sesiones alojadas por Anthropic; para un host de git que solo es enrutable dentro de su red, use un [hook de ciclo de vida `checkout`](/docs/es/self-hosted-environments-configuration#checkout) en su lugar. Cada proceso de runner maneja una sesión a la vez, así que ejecute más réplicas para paralelismo. Cuando el proxy está habilitado, `--git-host-rewrite` y `--git-ssh-rewrite` no tienen efecto: la URL del proxy apunta a `api.anthropic.com`, no a su host de git.

El runner también reporta la opción de participación a Anthropic cuando se registra, imprimiendo `Registering as opted in to Anthropic-managed git (--use-anthropic-git-proxy)` al inicio. Reportar la opción de participación requiere Claude Code v2.1.267 o posterior, y versiones anteriores aceptan la bandera sin reportarla o imprimir esa línea. Cada sesión en un runner que ha optado por participar luego usa git gestionado por Anthropic o la URL del proxy por sesión. Cuando una sesión usa la URL del proxy por sesión, el runner registra una línea `[runner:warn]` diciendo así.

<h3 id="rewrite-git-urls-for-private-networks">
  Reescriba URLs de git para redes privadas
</h3>

Las URLs de repositorio llegan desde el plano de control como HTTPS, con el nombre de host de su host de git; para GitHub Enterprise, ese es el nombre de host que configuró para la [integración de GitHub Enterprise](/docs/es/github-enterprise-server) en la configuración de administrador de Claude Code en claude.ai. Dos banderas repetibles reescriben esas URLs antes del clon:

* `--git-host-rewrite <from>=<to>`: para DNS de horizonte dividido, donde Anthropic alcanza su host de git a través de un nombre de host externo pero los runners deben usar uno interno
* `--git-ssh-rewrite <host>`: para hosts de git que solo aceptan SSH, reescribiendo `https://<host>/owner/repo` a `git@<host>:owner/repo`

La reescritura de host se ejecuta primero, así que enumere el nombre de host interno en `--git-ssh-rewrite` si necesita ambos. Para control total sobre el checkout, use un [hook de ciclo de vida `checkout`](/docs/es/self-hosted-environments-configuration#checkout).

<h2 id="build-the-runner-image">
  Construya la imagen del runner
</h2>

Anthropic no publica una imagen de runner precompilada. Construya la suya alrededor del binario `claude`, agregando cualquier cadena de herramientas que sus repositorios necesiten: tiempos de ejecución de lenguaje, compiladores, gestores de paquetes, y sidecars de [MCP](/docs/es/mcp).

Las recetas a continuación usan `--capacity 4`, por lo que un contenedor sirve hasta cuatro sesiones concurrentes del mismo propietario bloqueado. Eso no proporciona el aislamiento del contenedor por sesión en la [sección de endurecimiento](#harden-your-deployment): antes de conectar un entorno a sistemas de producción, ejecute las recetas en `--capacity 1` con un contenedor por sesión, o use [runners bajo demanda](/docs/es/self-hosted-environments-configuration#on-demand-runners), que también mantienen el secreto del entorno fuera de los hosts que ejecutan sesiones.

Este Dockerfile es un punto de partida mínimo:

```dockerfile theme={null}
FROM debian:bookworm-slim
ARG CLAUDE_CODE_VERSION
RUN apt-get update && apt-get install -y --no-install-recommends git curl ca-certificates openssh-client \
 && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL "https://downloads.claude.ai/claude-code-releases/${CLAUDE_CODE_VERSION:?set with --build-arg CLAUDE_CODE_VERSION}/linux-x64/claude" \
      -o /usr/local/bin/claude && chmod +x /usr/local/bin/claude
RUN git config --system user.name "Claude" \
 && git config --system user.email "noreply@anthropic.com" \
 && git config --system --add safe.directory '*'
ENTRYPOINT ["claude"]
```

Intercambie `linux-x64` por `linux-arm64` si sus nodos son ARM, o por `linux-x64-musl` o `linux-arm64-musl` en una imagen basada en musl como Alpine; consulte [Configuración de Alpine Linux](/docs/es/setup#alpine-linux-and-musl-based-distributions) para los paquetes adicionales que las imágenes musl necesitan. La URL es la ubicación de lanzamiento estándar de Claude Code, por lo que puede verificar el binario descargado contra el manifiesto firmado del lanzamiento como se describe en [Integridad binaria y firma de código](/docs/es/setup#binary-integrity-and-code-signing). Construya la imagen con la versión 2.1.224 de Claude Code o posterior, luego empújela a su registro y hágale referencia en las recetas a continuación:

```bash theme={null}
docker build --build-arg CLAUDE_CODE_VERSION=2.1.267 -t <your-registry>/claude-runner:latest .
```

<h2 id="size-cpu-and-memory-for-sessions">
  Dimensione CPU y memoria para sesiones
</h2>

Dimensione el contenedor o host de un runner para las sesiones que ejecuta en lugar de para el proceso del runner. El runner en sí sondea trabajo, prepara el checkout de cada sesión, ejecuta sus [hooks de ciclo de vida](/docs/es/self-hosted-environments-configuration#lifecycle-hooks), e inicia y supervisa los procesos de sesión. La carga proviene de las sesiones: cada una es un proceso de Claude Code más lo que inicia, como compilaciones, suites de prueba, instalaciones de paquetes, y [servidores MCP](/docs/es/mcp).

Para una sesión, comience con los siguientes valores, indicados como solicitudes y límites de Kubernetes o el equivalente de su plataforma, y trate los como un punto de partida en lugar de un requisito:

* **Memoria**: una solicitud y un límite de 4 GiB cada uno, que cumple con el mínimo de 4 GB en los [requisitos del sistema](/docs/es/setup#system-requirements) de Claude Code. Mantenga los dos iguales para que el programador tenga en cuenta la memoria completa del contenedor. Cuando el contenedor alcanza su límite de memoria, el kernel mata procesos dentro de él, lo que puede terminar una sesión a mitad de la tarea.
* **CPU**: una solicitud de 2 CPUs y un límite de 4 CPUs, para que una sesión pueda aumentar por encima de la solicitud durante compilaciones. El kernel acelera un contenedor en su límite de CPU en lugar de matar procesos en él, por lo que las sesiones en el límite se ejecutan más lentamente pero siguen ejecutándose.

En una especificación de contenedor de Kubernetes, establezca esos valores iniciales con el siguiente bloque `resources`:

```yaml theme={null}
resources:
  requests:
    cpu: "2"
    memory: 4Gi
  limits:
    cpu: "4"
    memory: 4Gi
```

Las compilaciones y pruebas son generalmente la parte más grande y variable de la carga de una sesión, así que ejecute una compilación representativa de su repositorio, mida su CPU y memoria máximas, y aumente cualquier valor inicial que no deje espacio para el proceso de Claude Code en la parte superior de ese pico.

El runner usa `--capacity` para limitar cuántas sesiones ejecuta a la vez. No divide CPU o memoria entre ellas, por lo que las sesiones en un runner comparten la CPU y memoria del contenedor. Para limitar la parte de una sesión, aplique límites desde su [script de envoltura](/docs/es/self-hosted-environments-configuration#wrapper-scripts). Lo que dar a un contenedor depende de cuántas sesiones sirve a la vez:

* **Una sesión por runner**: dé a cada contenedor los valores de una sesión. Use este dimensionamiento en `--capacity 1`, que la [sección de endurecimiento](#harden-your-deployment) recomienda, y para [runners bajo demanda](/docs/es/self-hosted-environments-configuration#on-demand-runners), donde establece los valores en la carga de trabajo que su [hook `spawn-runner`](/docs/es/self-hosted-environments-configuration#the-spawn-runner-hook) envía, como la plantilla de pod de un Job de Kubernetes.
* **Varias sesiones por runner**: en un `--capacity` por encima de uno, multiplique los valores de una sesión por la capacidad, porque hasta esa cantidad de sesiones pueden ejecutarse en el contenedor a la vez. Las recetas de [Kubernetes](#kubernetes) y [Docker Compose](#docker-compose) ejecutan `--capacity 4` sin límites de CPU o memoria, así que agregue límites dimensionados para la capacidad que ejecuta.

<h2 id="kubernetes">
  Kubernetes
</h2>

El runner sirve `GET /healthz` en el puerto 8080 por defecto, configurable con `--health-port`, por lo que los sondeos de Kubernetes funcionan sin configuración adicional. El punto final devuelve `200` siempre que el proceso esté vivo, por lo que los sondeos a continuación detectan un proceso muerto, no uno atascado; para detectar un runner que dejó de sondear, alerte en la serie `last_poll_age_seconds` de [`/metrics`](/docs/es/self-hosted-environments-reference#prometheus-metrics). El Deployment a continuación monta el secreto del entorno desde un Secret de Kubernetes, apunta los sondeos de vivacidad y preparación a `/healthz`, y establece un período de gracia de terminación de 90 segundos. Consulte [Tiempo de apagado](#shutdown-timing) para saber por qué importa el período de gracia.

El manifiesto no establece `resources` de CPU o memoria en el contenedor del runner. Agregue un bloque dimensionado para la capacidad que ejecuta, como [Dimensione CPU y memoria para sesiones](#size-cpu-and-memory-for-sessions) describe.

```yaml theme={null}
apiVersion: apps/v1
kind: Deployment
metadata:
  name: claude-runner
  namespace: claude-runners
spec:
  replicas: 3
  selector:
    matchLabels:
      app: claude-runner
  template:
    metadata:
      labels:
        app: claude-runner
        app.kubernetes.io/part-of: claude-code-self-hosted-runner
    spec:
      terminationGracePeriodSeconds: 90
      containers:
        - name: runner
          image: <your-registry>/claude-runner:latest
          args:
            - self-hosted-runner
            - --environment-secret-file
            - /etc/claude/environment-secret
            - --capacity
            - "4"
          volumeMounts:
            - name: environment-secret
              mountPath: /etc/claude
              readOnly: true
          ports:
            - name: health
              containerPort: 8080
          readinessProbe:
            httpGet:
              path: /healthz
              port: 8080
            initialDelaySeconds: 5
            periodSeconds: 10
          livenessProbe:
            httpGet:
              path: /healthz
              port: 8080
            initialDelaySeconds: 30
            periodSeconds: 30
      volumes:
        - name: environment-secret
          secret:
            secretName: claude-runner-environment-secret
```

El Deployment anterior vive en un espacio de nombres `claude-runners`. Cree el espacio de nombres primero:

```bash theme={null}
kubectl create namespace claude-runners
```

Cree el Secret de respaldo desde un archivo local que contenga el valor que copió en el paso [**Copiar clave de entorno**](/docs/es/self-hosted-environments-quickstart#set-up-an-environment-and-runner) de la UI de administrador, para que el secreto nunca aparezca en su historial de shell. Ejecute `(umask 077 && cat > ./environment-secret)`, pegue el secreto, presione Enter, luego Ctrl-D. Luego cree el Secret y elimine el archivo:

```bash theme={null}
kubectl create secret generic claude-runner-environment-secret -n claude-runners --from-file=environment-secret=./environment-secret
```

<h2 id="docker-compose">
  Docker Compose
</h2>

El servicio Compose a continuación reinicia el runner siempre que sale, lo que cubre tanto bloqueos como la salida normal después del drenaje. Una política de reinicio de Docker reinicia el mismo contenedor con su capa escribible intacta, por lo que el runner regresa en un sistema de archivos reutilizado en lugar del nuevo que la [postura de endurecimiento](#harden-your-deployment) recomienda; use esta receta para evaluación, y para producción recrear el contenedor por ejecución o use un orquestador que lo haga.

```yaml theme={null}
services:
  claude-runner:
    image: <your-registry>/claude-runner:latest
    command:
      - self-hosted-runner
      - --environment-secret-file
      - /run/secrets/environment-secret
      - --capacity
      - "4"
    secrets:
      - environment-secret
    restart: always
    stop_grace_period: 90s

secrets:
  environment-secret:
    file: ./environment-secret
```

<h2 id="shutdown-timing">
  Tiempo de apagado
</h2>

En `SIGTERM`, el runner deja de tomar trabajo nuevo y, a menos que establezca [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal), espera hasta `--drain-wait-sec`, cero por defecto, para que los turnos en vuelo terminen, termina el árbol de procesos de cada sesión, y ejecuta el [hook de ciclo de vida `post-session`](/docs/es/self-hosted-environments-configuration#post-session). Ese árbol de procesos incluye comandos que Claude aún estaba ejecutando en la sesión.

La ruta de drenaje completa necesita hasta `--session-stop-grace-sec` + `--drain-wait-sec` + `--post-session-hook-timeout-sec`, más 15 segundos de sobrecarga fija para limpieza de procesos, más 30 segundos más cuando [`--push-outcome-on-release`](/docs/es/self-hosted-environments-reference#runner-cli-flags) está establecido. Eso es 80 segundos en valores predeterminados, y el runner registra el total al inicio. Las sesiones drenan en paralelo bajo este presupuesto, por lo que el total no crece con `--capacity`.

En el `--drain-wait-sec 0` predeterminado, un reinicio rodante interrumpe turnos en vuelo; cada sesión se reanuda en otro runner, perdiendo trabajo no empujado como se describe en [Problemas conocidos](#additional-limitations). Establezca `--drain-wait-sec`, y aumente el período de gracia para que coincida, para dejar que los turnos terminen primero.

A lo largo de toda esa ruta, el runner sigue latiendo al plano de control con capacidad cero, por lo que el arrendamiento de sesión no expira y se reencola a otro runner mientras el hook `post-session` aún está escribiendo trabajo no comprometido. El latido se detiene justo antes de que el runner se desregistre.

Dé al runner al menos el total que registra al inicio antes de que el host lo detenga. Dónde establece eso depende de cómo se detienen sus hosts:

* **Con un período de gracia `SIGTERM`**: establezca `terminationGracePeriodSeconds` en Kubernetes, `stop_grace_period` en Docker Compose, o el equivalente de su orquestador en al menos ese total. El valor predeterminado de Kubernetes de 30 segundos es más corto que la ruta de drenaje del runner, por lo que Kubernetes detiene el pod antes de que el runner termine de drenar.
* **Con [`--retire-at`](/docs/es/self-hosted-environments-reference#runner-cli-flags)**: dimensione el margen entre el tiempo de jubilación y el tiempo de parada del host para cubrir turnos típicos, más la retención de tarea de fondo que [Ciclo de vida del runner](/docs/es/self-hosted-environments#runner-lifecycle) describe, más ese mismo total. Calcule el tiempo de jubilación en cada lanzamiento, por ejemplo `date +%s` más la vida útil prevista del runner.
* **Con [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal)**: agregue dos partes más al total de la ruta de drenaje. La primera es los minutos que configura. La segunda es la gracia posterior a la liberación que [Diferir el drenaje más allá de la primera señal](#defer-the-drain-past-the-first-signal) describe, 75 segundos en valores predeterminados. Con la bandera establecida, el runner también imprime la cifra combinada al inicio, después del total de la ruta de drenaje.

<h3 id="defer-the-drain-past-the-first-signal">
  Diferir el drenaje más allá de la primera señal
</h3>

Establezca [`--defer-shutdown-max-min <n>`](/docs/es/self-hosted-environments-reference#runner-cli-flags) si desea que un runner que está reiniciando continúe sirviendo las sesiones que mantiene durante hasta `n` minutos, en lugar de drenarlas en la primera señal. En la primera `SIGTERM` o `SIGINT`, el runner deja de tomar trabajo nuevo y continúa sirviendo las sesiones que mantiene. Sigue sondeando para que el plano de control no reencole esas sesiones. Requiere Claude Code v2.1.238 o posterior.

<h4 id="what-happens-to-the-sessions-the-runner-holds-after-the-first-signal">
  Lo que sucede con las sesiones que el runner mantiene después de la primera señal
</h4>

En las primeras dos etapas que siguen a la señal, el runner libera sesiones, y una sesión liberada se reanuda en un runner nuevo cuando su usuario envía su siguiente mensaje. Contando desde la primera señal, el runner se mueve a través de tres etapas:

* **Durante los primeros `n` minutos**: el runner sirve sus sesiones normalmente y sigue aplicando `--startup-timeout-min` y `--kill-session-after-min`. Si también establece [`--release-idle-session-min`](/docs/es/self-hosted-environments-reference#runner-cli-flags), el runner libera cualquier sesión cuyo usuario ha estado inactivo ese tiempo; sin ella, las sesiones inactivas permanecen en el runner.
* **Cuando los `n` minutos se agotan**: el runner libera cada sesión que aún mantiene, inactiva o no. El runner espera a que la sesión de mitad de turno termine su turno, y hasta 60 segundos más para las tareas de fondo de un turno, antes de liberar esa sesión.
* **Cuando la gracia posterior a la liberación se agota**: el runner drena cualquier sesión que aún mantiene, y el plano de control reencola cada sesión drenada a otro runner de inmediato. La gracia posterior a la liberación comienza cuando los `n` minutos se agotan y es 75 segundos en valores predeterminados. Si establece `--drain-wait-sec` por encima de 60 segundos, la gracia posterior a la liberación es `--drain-wait-sec` más 15 segundos en su lugar.

En cualquier etapa, el runner sale 0 tan pronto como no mantiene sesiones. Una segunda señal acorta las etapas: el runner drena inmediatamente, como lo hace en la primera señal sin `--defer-shutdown-max-min`. Una vez que un drenaje está en marcha, la siguiente señal fuerza la salida del runner. Eso se mantiene si una segunda señal o la gracia posterior a la liberación que se agota inició el drenaje.

<h4 id="size-the-stop-timeout">
  Dimensione el tiempo de parada
</h4>

Dé al tiempo de parada de su host al menos la suma de tres partes: los `n` minutos que configura, la gracia posterior a la liberación, y la ruta de drenaje completa que [Tiempo de apagado](#shutdown-timing) describe. Con configuraciones predeterminadas, la gracia posterior a la liberación es 75 segundos y la ruta de drenaje es 80 segundos, así que permita `n` minutos más 155 segundos. El runner imprime esta suma al inicio siempre que `--defer-shutdown-max-min` está establecido.

Si el tiempo de parada se agota antes de que el runner termine, el host mata el runner. Las sesiones que aún mantiene no obtienen ningún hook `post-session`. El runner no se desregistra, y el plano de control reencola las sesiones aproximadamente un minuto después. Si no puede dar al tiempo de parada esa suma, deje `--defer-shutdown-max-min` sin establecer para que el runner drene en la primera señal en su lugar.

<h3 id="what-reaches-a-running-post-session-hook">
  Lo que alcanza un hook post-session en ejecución
</h3>

El hook `post-session` y el hijo de sesión de Claude cada uno se ejecutan en su propio grupo de procesos POSIX, separado del runner, por lo que los mecanismos de parada los alcanzan de manera diferente:

* **Un `SIGTERM` mientras el runner ya está drenando**: fuerza la salida del runner inmediatamente, omitiendo lo que queda de la ruta de drenaje. Sin [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal), eso es el segundo `SIGTERM` que recibe el runner. Nada señala un hook `post-session` en ejecución, por lo que en un host desnudo donde un proceso init adopta huérfanos, termina por su cuenta, pero sin supervisión: su presupuesto de tiempo ya no se aplica, y una escritura en la tubería de registro cerrada puede matarlo con `SIGPIPE`, así que un hook que necesita sobrevivir a una salida forzada allí debe redirigir su propia salida a un archivo. En las recetas de contenedor en esta página el runner es el PID 1 del contenedor y su salida termina el contenedor, y bajo el `KillMode=control-group` predeterminado de systemd la matanza de cgroup alcanza el hook también, como la entrada **Matanzas de cgroup** describe; en ambos, trate una salida forzada como fatal para el hook y confíe en el período de gracia en su lugar.
* **Señales de grupo de procesos**, como `kill -- -<pid>` en un script de envoltura, control de trabajo de shell, o un vigilante de grupo: alcanzan el runner y un subproceso de hook `checkout` en ejecución, que permanece adjunto al grupo deliberadamente, pero no un hook `post-session` en ejecución o el hijo de sesión.
* **Matanzas de cgroup**, como el `KillMode=control-group` predeterminado de systemd o el `SIGKILL` que Kubernetes entrega a todo el contenedor cuando `terminationGracePeriodSeconds` expira: alcanzan todo, incluido el hook. El aislamiento de grupo de procesos no protege contra estos, que es por qué el período de gracia debe cubrir la ruta de drenaje completa.
* **El tiempo de espera propio del hook**: cuando un hook excede `--post-session-hook-timeout-sec`, el runner envía `SIGTERM` a todo el grupo de procesos del hook, luego `SIGKILL` dos segundos después, por lo que un trabajador que el hook bifurcó, como tar, rsync, o git, termina con el shell de envoltura en lugar de sobrevivir como un huérfano. La supervisión del runner termina una vez que el stdio del hook se cierra: un trabajador que redirigió su propia salida a un archivo y sobrevive a la etapa `SIGTERM` está más allá del alcance del runner.

Cuando comienza el drenaje, y nuevamente en una salida forzada, el runner registra cuántos hooks `post-session` aún se están ejecutando, para que pueda distinguir un drenaje tranquilo de uno que está en mitad de una instantánea.

<h2 id="keep-the-base-directory-and-capacity-identical-across-runners">
  Mantenga el directorio base y la capacidad idénticos en todos los runners
</h2>

Si un runner muere en mitad de sesión, el servidor reencola la sesión y otro runner en el entorno la recoge. Ese runner deriva la ruta de checkout de su propio `--base-dir` y `--capacity`: `--capacity 1` verifica directamente bajo `--base-dir`, y un `--capacity` por encima de `1` usa worktrees por sesión en su lugar. Cuando los runners en el mismo entorno usan valores diferentes para cualquiera de las banderas, el directorio de trabajo de la sesión reanudada cambia, y las rutas absolutas que el agente registró anteriormente, en ediciones, llamadas de herramientas, o sus propias notas, apuntan a una ubicación que ya no existe.

Use el mismo `--base-dir` y `--capacity` en cada runner en un entorno, y no use un valor por host como un ID de instancia o nombre de host.

El directorio base tiene como valor predeterminado `/workspace`, con la excepción que la fila de referencia [`--base-dir`](/docs/es/self-hosted-environments-reference#runner-cli-flags) registra. El runner necesita acceso de escritura a él. Al inicio, antes de registrarse, el runner crea el directorio y confirma que puede escribir en él, y sale con `cannot create or write to base directory` cuando no puede. Un runner iniciado como root crea el `/workspace` predeterminado en sí. Para un runner que no es root, cree el directorio y dé al usuario del runner la propiedad antes de iniciar el runner, o apunte `--base-dir` a un directorio que ese usuario ya posee.

<h2 id="reuse-a-pre-warmed-checkout">
  Reutilice un checkout precalentado
</h2>

Para repositorios grandes, el clon puede dominar el inicio de sesión. En `--capacity 1` sin [hook `checkout`](/docs/es/self-hosted-environments-configuration#checkout), el runner mantiene un clon canónico por repositorio en `<base-dir>/<repo-owner>/<repo>` y lo reutiliza en sesiones: obtiene la ref solicitada, desasocia `HEAD`, y reinicia duro a ella, que es casi instantáneo cuando poco ha cambiado. Para omitir el clon frío, suministre el clon de una de dos maneras:

* **Clon en la imagen**: construya el clon en su imagen de runner en esa ruta. Cada contenedor nuevo comienza con el clon precalentado sin reutilizar un disco.
* **Clon en un volumen persistente**: en runners que prebloquea a la cuenta de un usuario con [`--lock-to-account`](/docs/es/self-hosted-environments-reference#runner-cli-flags), apunte `--base-dir` a un volumen persistente, para que el disco solo sirva esa cuenta. Un runner prebloquado nunca recoge sesiones de canal de Claude Tag, por lo que esta opción no se aplica a runners que las sirven.

Lo que la ruta de reutilización hace y no garantiza:

* **Cualquier forma de clon funciona**: un clon completo, superficial, o de rama única en la ruta se usa tal cual. El runner nunca pasa `--depth` cuando obtiene en un clon existente, por lo que un precalentamiento completo mantiene su historial completo y uno superficial permanece superficial. `CLAUDE_RUNNER_FETCH_DEPTH` (`full`, `0`, o un número; valor predeterminado 50) controla solo el clon frío que el runner hace cuando no existe clon aún.
* **Los cambios rastreados se reinician, los archivos sin rastrear persisten**: cada sesión comienza desde un reinicio duro que borra las modificaciones rastreadas de la sesión anterior, pero el runner nunca ejecuta `git clean`, por lo que los archivos sin rastrear de las sesiones anteriores del propietario bloqueado permanecen en el árbol.
* **Directorios por sesión persisten también**: junto al checkout, el runner crea entradas por sesión bajo `<base-dir>/_sessions/` para cada sesión que ejecuta. El directorio de configuración de Claude de la sesión contiene una copia local de la transcripción de la conversación. Junto a él se encuentran los archivos cargados de la sesión, cuando la sesión tiene alguno. El directorio de sesión también se encuentra allí: contiene cualquier worktree por sesión y checkouts de hook `checkout` mientras se ejecuta la sesión, y mantiene cualquier otra cosa que Claude escribió en él.

  Por defecto, el runner deja estos en su lugar cuando termina la sesión, por lo que en un disco que sobrevive al proceso del runner se acumulan. Cada sesión se ejecuta como el usuario del runner, por lo que cualquier sesión posterior que ese disco sirva puede leerlos. Si mantiene un `--base-dir` persistente, dimensione el volumen para ese crecimiento. Lo mismo se aplica a cualquier configuración que reinicie el runner en el mismo sistema de archivos, incluida la [receta de Docker Compose](#docker-compose).
* **Con `--remove-session-state`, los directorios por sesión no persisten**: inicie el runner con [`--remove-session-state`](/docs/es/self-hosted-environments-reference#runner-cli-flags) para que elimine los directorios por sesión de cada sesión cuando termina la sesión. La eliminación es de mejor esfuerzo: los directorios permanecen cuando el runner se mata antes de que se ejecute su limpieza. El clon canónico y los archivos que una sesión escribió en otro lugar del host, como el directorio temporal, permanecen independientemente.
* **Con el proxy de git, el reinicio se convierte en un checkout**: con [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy), el runner sanitiza el `.git/` del clon antes de cada sesión, manteniendo el almacén de objetos, refs, y estado superficial pero eliminando el índice, por lo que cada sesión paga un checkout de árbol de trabajo completo en lugar de un reinicio casi instantáneo; aún nunca vuelve a clonar. Los precalentamientos de submódulos no son compatibles con el proxy.
* **Los clones largos no necesitan solución alternativa**: el runner limita cada operación de git con un vigilante de 120 segundos sin progreso y un límite duro de 30 minutos, no un tiempo de espera plano, por lo que un clon frío lento que sigue reportando progreso se completa.

<h2 id="pin-the-version">
  Fije la versión
</h2>

El proceso hijo de Claude Code de cada sesión ejecuta el binario propio del runner, y el runner desactiva la actualización automática dentro de las sesiones que genera, por lo que cada sesión ejecuta la versión que instaló en el host o construyó en la imagen. Una actualización a nivel de host entra en vigor la próxima vez que el runner comienza.

* **Para mantener una flota en una versión**: construya la imagen con una versión fijada, o en un host desnudo instale una versión específica y [desactive las actualizaciones automáticas](/docs/es/setup#disable-auto-updates)
* **Para actualizar**: instale la versión más reciente o reconstruya la imagen, luego reinicie los runners
* **Plugins**: los mercados de plugins tampoco se actualizan automáticamente; establezca `FORCE_AUTOUPDATE_PLUGINS=1` en el entorno del runner para permitir que los plugins se actualicen automáticamente mientras el binario permanece fijado

<h2 id="scale-the-fleet">
  Escale la flota
</h2>

Su orquestador decide cuándo agregar o eliminar runners. Debido al [bloqueo de un propietario por runner](/docs/es/self-hosted-environments#runner-lifecycle), el recuento de réplicas mínimo es el número de usuarios y agentes de Claude Tag que espera que estén activos simultáneamente; `--capacity` controla el paralelismo dentro de las sesiones de un propietario, no entre propietarios.

Dos enfoques de escalado están disponibles:

* **Flota fija**: ejecute un conjunto estático de réplicas de runner y escale en las [métricas de Prometheus](/docs/es/self-hosted-environments-reference#prometheus-metrics) que cada runner sirve
* **Runners bajo demanda**: ejecute el subcomando `claude self-hosted-runner orchestrator`, que sondea Anthropic para sesiones que están en cola sin runner disponible e invoca su hook `spawn-runner` para arrancar uno por sesión. Consulte [Runners bajo demanda](/docs/es/self-hosted-environments-configuration#on-demand-runners).

<h2 id="known-issues-and-limitations">
  Problemas conocidos y limitaciones
</h2>

Las siguientes son las limitaciones en esta versión, con soluciones alternativas donde exista una.

<h3 id="connector-traffic-leaves-your-network">
  El tráfico del conector sale de su red
</h3>

Anthropic llama herramientas de conector desde su propia infraestructura en lugar de desde su runner. Las herramientas de conector son los conectores de claude.ai, como GitHub, Slack y Linear. Cuando Claude usa un conector en una sesión autohospedada, ese tráfico va a través de `api.anthropic.com` en lugar de originarse dentro de su límite de red.

Para mantener un conector fuera de sesiones autohospedadas, filtrelo con la [configuración de política `allowedMcpServers` y `deniedMcpServers`](/docs/es/managed-mcp#policy-based-control-with-allowlists-and-denylists). Claude Code aplica estas configuraciones a los conectores que Anthropic entrega así como a los servidores que configura desde el host del runner y los servidores que los usuarios agregan, por lo que si implementa una lista de permitidos para otros servidores, Claude Code bloquea conectores entregados también. Para mantener conectores disponibles junto con una lista de permitidos basada en URL, agregue entradas que coincidan con las rutas de proxy de Anthropic para conectores entregados:

* `https://api.anthropic.com/v2/ccr-sessions/*`
* `https://api.anthropic.com/v1/code/sessions/*`
* `https://api.anthropic.com/v1/code/mcp/*`

Si el tráfico de herramientas debe permanecer dentro de su red, ejecute las herramientas equivalentes como servidores MCP locales en la imagen del runner en su lugar. Consulte [Servidores MCP](/docs/es/self-hosted-environments-configuration#mcp-servers).

<h3 id="some-sessions-don’t-count-as-idle">
  Algunas sesiones no cuentan como inactivas
</h3>

Una sesión que mantiene una tarea de fondo que nunca termina no cuenta como inactiva, por lo que `--release-idle-session-min` no liberará la ranura de esa sesión. Una sesión que espera una aprobación solicitada desde dentro de una llamada de herramienta en ejecución tampoco cuenta como inactiva. Siempre establezca `--kill-session-after-min` junto con ella como un tope duro para que ninguna sesión pueda mantener una ranura indefinidamente.

`--kill-session-after-min` es un tope para sesiones descontroladas. En un runner en v2.1.260 o posterior, una sesión que alcanza el límite no se termina inmediatamente. El runner le da una ventana de gracia, 15 minutos por defecto, que puede cambiar con [`SELF_HOSTED_RUNNER_MAX_LIFETIME_GRACE_MS`](/docs/es/self-hosted-environments-reference#environment-variable-only-settings):

* Si la sesión está esperando a su usuario, el runner la libera. Si su turno ha terminado y solo mantiene tareas de fondo, el runner espera hasta 60 segundos a que esas tareas terminen y luego la libera. La sesión se reanuda cuando su usuario envía su siguiente mensaje.
* Si un turno aún se está ejecutando, el runner espera a que el turno termine, o a que la sesión espere a su usuario, y luego la libera.
* Si la sesión aún está en el runner cuando la ventana de gracia termina, el runner la termina, y se pierde el trabajo de cualquier turno en ejecución. Un turno esperando una aprobación solicitada desde dentro de una llamada de herramienta en ejecución es una forma en que una sesión supera la ventana.

Una sesión liberada se reanuda desde un clon nuevo, por lo que el trabajo que no había empujado se ha ido de cualquier forma; consulte [Las sesiones reanudadas pierden trabajo no empujado](#additional-limitations). Antes de v2.1.260, el runner terminaba cada sesión en el límite, después de esperar como máximo la ventana de gracia a que un turno en ejecución terminara.

Establezca la bandera anterior a su sesión más larga esperada, como `--kill-session-after-min 480` para 8 horas. Para liberar ranuras de conversaciones que se vuelven inactivas, use `--release-idle-session-min` en su lugar.

<h3 id="additional-limitations">
  Limitaciones adicionales
</h3>

* **Las sesiones reanudadas pierden trabajo no empujado**: cuando una sesión se libera o su runner se reinicia, y el usuario envía otro mensaje, la sesión se reanuda en un runner nuevo que clona el repositorio nuevamente desde su rama inicial, por lo que el trabajo que la sesión no había empujado se ha ido. Establezca [`--push-outcome-on-release`](/docs/es/self-hosted-environments-reference#runner-cli-flags) para que el runner haga un mejor esfuerzo para empujar las ramas de resultado de la sesión antes de liberarla, para que la sesión reanudada comience desde esos commits en su lugar; esto preserva trabajo comprometido, no un árbol de trabajo sucio. Antes de habilitarlo, restrinja quién puede empujar a refs `claude/*` en el remoto de origen, por ejemplo con un conjunto de reglas de rama: en la reanudación, el runner obtiene la rama previamente empujada sin verificar quién la empujó, por lo que cualquiera con acceso de push a esos refs puede colocar contenido en el espacio de trabajo reanudado. El runner también descarta la configuración por sesión en la reanudación, lo que significa el directorio de configuración de Claude de la sesión y cualquier estado de shell que la sesión escribió; `--push-outcome-on-release` no cubre esos.
* **Los repositorios privados no se pueden agregar en mitad de sesión**: un repositorio agregado a una sesión después de que ha comenzado no se clona con credenciales en un runner autohospedado, por lo que la adición falla. Seleccione cada repositorio que la sesión necesita cuando la crea.
* **Algunos conectores no aparecen en sesiones autohospedadas**: un conector que aún no ha conectado en la configuración de claude.ai no se enumera en una sesión autohospedada, y la sesión no le pedirá que lo conecte. Conéctelo en Configuración primero, luego inicie una sesión nueva. Agregar un conector a una sesión ya en ejecución tampoco hace que sus herramientas estén disponibles para Claude; inicie una sesión nueva para recoger un conector recién agregado.

<h3 id="report-an-issue">
  Reportar un problema
</h3>

Para problemas con entornos autohospedados, contacte a su equipo de cuenta de Anthropic.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

Para un diagnóstico guiado, ejecute el subcomando doctor en el host del runner. El subcomando doctor inicia una sesión interactiva de Claude Code con los registros y el estado del runner adjuntos. Inicie sesión con `claude auth login` en ese host primero para que la sesión pueda consultar su entorno, sus runners y sus sesiones en cola. Sin ese inicio de sesión, por ejemplo cuando el host se autentica con una clave API, se limita al punto final de salud local, las métricas y el registro del runner, y lee el registro solo si inició el runner con `--log-file`.

```bash theme={null}
claude self-hosted-runner doctor
```

Problemas comunes:

* **El runner no aparece en el entorno**: confirme que el host pueda alcanzar `api.anthropic.com` sobre HTTPS, que el secreto del entorno sea actual y que el reloj del host esté dentro de cinco minutos de la hora real; un sesgo mayor causa que la autenticación falle. El runner registra `[runner:fatal]` con el motivo del rechazo en caso de fallo de autenticación.
* **El runner se cierra al inicio con `cannot create or write to base directory`**: el runner no puede crear ni escribir en `--base-dir`, que por defecto es `/workspace`. Corrija la propiedad del directorio o apunte `--base-dir` a una ruta escribible, como se describe en [Mantener el directorio base y la capacidad idénticos en todos los runners](#keep-the-base-directory-and-capacity-identical-across-runners). Si el runner registra `[runner:fatal]` diciendo que la verificación del directorio base agotó el tiempo de espera, el directorio está en un montaje NFS o CSI colgado. Verifique la salud del montaje en lugar de los permisos. El runner imprime ambas fallas de inicio en stderr antes de abrir `--log-file`, así que búsquelas en la terminal o en los registros del contenedor de su plataforma en lugar del archivo de registro. Antes de v2.1.225, el runner no verificaba el directorio base al inicio, y esta configuración incorrecta fallaba en las sesiones después de la recogida.
* **Las sesiones permanecen en cola**: cada runner en línea puede estar bloqueado a un propietario diferente. Verifique la [métrica](/docs/es/self-hosted-environments-reference#prometheus-metrics) `claude_code_self_hosted_runner_locked_account` de cada runner o el campo `locked_account` de su línea de registro `[runner:health]` para ver quién la mantiene. Ambos muestran el correo electrónico del propietario solo después de que el runner haya recibido un token de sesión que lleve un reclamo `act.email`, que las sesiones de un agente Claude Tag nunca hacen. Sin el reclamo, el runner no emite ninguna serie `locked_account` y registra `locked_account=yes`, lo que le indica que el runner está bloqueado pero no a qué propietario. Agregue réplicas o espere a que un runner existente se drene y reinicie. Si el entorno usa runners bajo demanda, verifique el orquestador en su lugar; consulte [On-demand runners](/docs/es/self-hosted-environments-configuration#on-demand-runners).
* **Las sesiones fallan inmediatamente después de la recogida**: abra la sesión en claude.ai/code para ver el error. Las causas más comunes son las [credenciales de git](#configure-git) faltantes en la imagen del runner y las herramientas de compilación que no están instaladas. Un directorio base no escribible detiene el runner al inicio en lugar de fallar en las sesiones. Consulte la entrada **El runner se cierra al inicio con `cannot create or write to base directory`** en esta lista.
* **Las sesiones no pueden alcanzar la red a través de un proxy de salida autenticador**: cuando la fuente que estableció con [`--proxy-authorization-command` o `--proxy-authorization-file`](#authenticate-to-an-egress-proxy) falla, agota el tiempo de espera después de 30 segundos o produce un valor vacío, el runner responde esa conexión con `502 Bad Gateway` y registra por qué. El runner redacta stderr del comando en ese registro y nunca registra el valor del encabezado. Con `--proxy-authorization-command`, ejecute el comando usted mismo en el host para confirmar que imprime el valor de encabezado completo en stdout. Si el runner se cierra al inicio con `could not start the proxy-authorization listener`, no pudo abrir su oyente de loopback.
* **El runner registra líneas `Poll failed` que contienen `rejecting the malformed poll response`**: el runner recibió una respuesta de sondeo de trabajo cuyo cuerpo no es el JSON esperado de la cola, la mayoría de las veces porque algo entre el runner y `api.anthropic.com`, como un proxy interceptor o un portal cautivo, respondió con su propia página. El runner rechaza la respuesta, la cuenta bajo el tipo `transport` de la [métrica](/docs/es/self-hosted-environments-reference#prometheus-metrics) `claude_code_self_hosted_runner_poll_errors_total`, y reintenta en el cronograma de sondeo fallido descrito en [Session lifecycle](/docs/es/self-hosted-environments#session-lifecycle). El runner continúa sirviendo sus sesiones activas. Configure el proxy para pasar las respuestas de `api.anthropic.com` sin alterar. Antes de v2.1.246, el runner leía tal respuesta como una cola de trabajo vacía, lo que podría terminar sus sesiones activas o hacer que se cierre.
* **La rama de una sesión ya no existe en el remoto**: para una fuente de git que la sesión solo lee, el runner omite esa fuente y continúa con las restantes. Para la fuente a la que la sesión envía resultados, una rama eliminada, típicamente porque fue fusionada y auto-eliminada, falla la sesión con un error que nombra el repositorio y la rama y le pide que restaure la rama y reintente. El runner falla la sesión con el mismo error cuando omitir dejaría sin repositorio en absoluto. Antes de v2.1.228, tal sesión comenzaba en un directorio vacío.
* **Una sesión comienza sin uno de sus repositorios**: en un runner sin un [hook `checkout`](/docs/es/self-hosted-environments-configuration#checkout), el host de git puede rechazar la verificación de acceso del runner para un repositorio que la sesión solo lee. El runner entonces omite ese repositorio, registra una línea `[runner:warn] could not access context source` que nombra el rechazo, e inicia la sesión en los restantes.

  El runner omite solo un rechazo claro: el host responde que el repositorio no fue encontrado, git no encuentra credenciales para el host, o la autenticación falla. Una falla de red, un tiempo de espera agotado, o un HTTP `403` aún falla el inicio de sesión, al igual que un rechazo para un repositorio al que la sesión envía resultados. El runner aún falla una sesión que omitir dejaría sin repositorio en absoluto. Con [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy), el runner omite solo un repositorio que el proxy de git mismo deniega.

  La verificación de acceso se ejecuta nuevamente cada vez que la sesión comienza en un runner, así que una vez que la identidad de git del runner tiene acceso de lectura, el siguiente inicio clona el repositorio. Antes de v2.1.274, cada uno de estos rechazos fallaba el inicio de sesión.
* **Las sesiones tardan minutos en iniciarse**: el clon inicial generalmente domina. Observe la [métrica](/docs/es/self-hosted-environments-reference#prometheus-metrics) `claude_code_self_hosted_runner_session_init_duration_seconds` para confirmar, y corte el clon con un [pre-warmed checkout](#reuse-a-pre-warmed-checkout) o un `CLAUDE_RUNNER_FETCH_DEPTH` más pequeño.
* **Los turnos fallan con un 401**: cada sesión autentica llamadas de modelo con el [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/es/self-hosted-environments-configuration#wrapper-scripts) de corta duración que el runner obtiene de Anthropic y rota sobre stdin de la sesión. Cuando un turno termina con un 401 o 403 de la API del modelo, el runner obtiene un token fresco y lo pasa a la sesión. El turno fallido no se reintenta.

  Cuando una obtención falla, el runner registra una línea `inference_token refresh failed` que dice cuándo reintentará, y continúa reintentando mientras la sesión se ejecute.

  Si cada llamada comienza a fallar aproximadamente 30 minutos en una sesión, un script contenedor probablemente ha cortado stdin de la sesión, por lo que las rotaciones de token no pueden alcanzarlo; consulte [Keep stdin and file descriptor 3 attached](/docs/es/self-hosted-environments-configuration#keep-stdin-and-file-descriptor-3-attached).

  Antes de v2.1.274, el runner dejaba de reintentar una obtención fallida después de algunos intentos y esperaba la siguiente programada. Un turno fallido no desencadenaba una obtención, por lo que cada turno fallaba con un 401 hasta la siguiente obtención programada.
* **El pod se mata a mitad del drenaje**: aumente `terminationGracePeriodSeconds` al menos al valor que el runner registra al inicio. Consulte [Shutdown timing](#shutdown-timing).

Una vez que se inicializa el registro, el runner escribe su registro de ciclo de vida, incluidas las líneas `[runner:fatal]`, en stdout, y la salida de depuración en stderr, todo como líneas de texto sin formato en lugar de JSON. Las fallas de inicio descritas en las entradas de solución de problemas anteriores se imprimen en stderr antes de ese punto. Capture ambas secuencias con `--log-file`, que también permite que `self-hosted-runner doctor` las siga, o con la recopilación de registros de su plataforma.

Cada proceso secundario de sesión escribe un registro de depuración separado. En caso de fallo, el runner expone la cola del registro junto con la sesión en claude.ai/code. A menos que haya iniciado el runner con [`--remove-session-state`](/docs/es/self-hosted-environments-reference#runner-cli-flags), también mantiene el registro de una sesión fallida en el disco e imprime su ruta en el registro del runner.

<h2 id="what’s-next">
  Qué sigue
</h2>

* [Personalice sesiones](/docs/es/self-hosted-environments-configuration): scripts de envoltura, hooks de ciclo de vida, runners bajo demanda, servidores MCP, y permisos
* [Pruebe de extremo a extremo](/docs/es/self-hosted-environments-testing): verifique una nueva imagen de runner desde CI antes de promoverla
* [Referencia](/docs/es/self-hosted-environments-reference): cada bandera CLI, variable de entorno, y métrica
