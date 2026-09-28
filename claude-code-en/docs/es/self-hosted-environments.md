> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Entornos autohospedados

> Ejecute sesiones en la nube de Claude Code en la infraestructura que controla: configure un entorno autohospedado, implemente ejecutores y enrute sesiones a su propio cómputo.

<Note>
  Los entornos autohospedados están en versión beta pública en planes Team y Enterprise y están deshabilitados de forma predeterminada. Consulte [Disponibilidad y limitaciones](#availability-and-limitations) para conocer la ruta de habilitación y qué está excluido.
</Note>

Un entorno autohospedado ejecuta sesiones en la nube de Claude Code en la infraestructura que opera su organización. Una [sesión en la nube](/docs/es/claude-code-on-the-web) es cualquier sesión que se ejecuta en algún lugar que no sea la máquina del desarrollador: los desarrolladores las inician desde claude.ai, las aplicaciones móviles y de escritorio, la terminal con [`claude --cloud`](/docs/es/claude-code-on-the-web#from-terminal-to-cloud), y [rutinas programadas](/docs/es/routines), y de forma predeterminada se ejecutan en la infraestructura de Anthropic. En un entorno autohospedado, esas mismas sesiones se ejecutan dentro de su red, y la experiencia del desarrollador es la misma excepto por las diferencias en [Disponibilidad y limitaciones](#availability-and-limitations) y los [problemas conocidos](/docs/es/self-hosted-environments-deploy#known-issues-and-limitations) de la página de implementación.

Si su equipo no utiliza sesiones en la nube, no hay nada que configurar aquí: las sesiones en una terminal o IDE siempre se ejecutan en la máquina del desarrollador. Si desea ejecutar Claude Code en su propia máquina siempre activa e impulsarla desde otros dispositivos, use [Control remoto](/docs/es/remote-control), que también está disponible en planes Pro y Max. Cuando esté listo para configurar, vaya directamente al [inicio rápido](/docs/es/self-hosted-environments-quickstart); para revisar primero la postura de seguridad, comience con [Implementar en producción](/docs/es/self-hosted-environments-deploy). El resto de esta página explica cómo funciona el autohospedaje y cuándo elegirlo.

<h2 id="how-self-hosted-environments-work">
  Cómo funcionan los entornos autohospedados
</h2>

El autohospedaje tiene tres partes:

* **Entorno**: un destino nombrado al que se pueden enviar sesiones en la nube. Su organización crea entornos en la configuración de administrador de claude.ai, y cada uno agrupa un conjunto de ejecutores.
* **Ejecutor**: un programa que se ejecuta en hosts dentro de su red. Los ejecutores ejecutan las sesiones; la idea es la misma que un ejecutor de CI autohospedado.
* **Sesión**: una tarea de Claude Code que un desarrollador inició.

Cuando un desarrollador inicia una sesión en la nube, la interfaz de inicio de sesión muestra un selector de entorno que enumera los entornos alojados por Anthropic junto con los que su organización ha creado. Si elige el suyo, el plano de control de Anthropic coloca la sesión en la cola de su entorno, donde un ejecutor la reclama, clona el repositorio que el desarrollador eligió e inicia un proceso de Claude Code en su host para ejecutarlo. El ejecutor se autentica en su host de git con credenciales que usted configura; [Configurar git](/docs/es/self-hosted-environments-deploy#configure-git) cubre las opciones. Las sesiones alcanzan sus servicios internos desde dentro de su red, y su host de git de la misma manera cuando es interno; el tráfico a Anthropic, el sondeo de colas, el flujo de eventos de la sesión e inferencia de modelos, es HTTPS saliente a `api.anthropic.com`, con la breve lista de hosts adicionales que las sesiones pueden alcanzar en [Requisitos de red](/docs/es/self-hosted-environments-deploy#network-requirements). Anthropic nunca se conecta a su red.

<div style={{maxWidth: "640px", margin: "0 auto"}}>
  <Frame>
    <img src="https://mintcdn.com/claude-code/Y0sJ2uDoOVbOVZrQ/images/self-hosted-network-paths.svg?fit=max&auto=format&n=Y0sJ2uDoOVbOVZrQ&q=85&s=8056103fc1c5564c7f0ef219d260b99d" className="dark:hidden" alt="Diagrama de arquitectura de un entorno autohospedado: el límite de su red contiene un ejecutor, dos procesos de sesión de Claude Code dentro de él, y su host de git, con api.anthropic.com afuera sosteniendo cola, flujo de sesión e inferencia. El ejecutor sondea la cola y alcanza el host de git, cada proceso de sesión abre sus propias conexiones de flujo, inferencia y git, y cada conexión es saliente desde su red, sin ninguna entrante." width="680" height="320" data-path="images/self-hosted-network-paths.svg" />

    <img src="https://mintcdn.com/claude-code/Y0sJ2uDoOVbOVZrQ/images/self-hosted-network-paths-dark.svg?fit=max&auto=format&n=Y0sJ2uDoOVbOVZrQ&q=85&s=fec6aef3b0740d80eaf6d6a7000a2233" className="hidden dark:block" alt="Diagrama de arquitectura de un entorno autohospedado: el límite de su red contiene un ejecutor, dos procesos de sesión de Claude Code dentro de él, y su host de git, con api.anthropic.com afuera sosteniendo cola, flujo de sesión e inferencia. El ejecutor sondea la cola y alcanza el host de git, cada proceso de sesión abre sus propias conexiones de flujo, inferencia y git, y cada conexión es saliente desde su red, sin ninguna entrante." width="680" height="320" data-path="images/self-hosted-network-paths-dark.svg" />
  </Frame>
</div>

Los dos cuadros de Claude Code en el diagrama son procesos de sesión: un ejecutor ejecutando dos sesiones a la vez, hasta su capacidad configurada. Un ejecutor sirve a un [propietario](#key-concepts) a la vez y se bloquea a ese propietario cuando reclama su primera sesión, por lo que el código extraído nunca se mezcla entre propietarios; [Ciclo de vida del ejecutor](#runner-lifecycle) cubre la regla.

Puede iniciar ejecutores usted mismo y mantenerlos en ejecución, o ejecutar el [orquestador de escalado automático](/docs/es/self-hosted-environments-configuration#on-demand-runners), un segundo proceso que hospeda, que inicia ejecutores a medida que las sesiones se colan; cada ejecutor sale por su cuenta cuando su trabajo termina. De cualquier manera, configura el entorno una vez, y aparece en el selector en cada superficie compatible.

<h2 id="availability-and-limitations">
  Disponibilidad y limitaciones
</h2>

Verifique esto antes de planificar un despliegue:

* **Planes**: versión beta pública para organizaciones Team y Enterprise. Los entornos autohospedados están deshabilitados de forma predeterminada; un [Propietario](/docs/es/cloud-environments#organization-shared-environments) activa **Permitir entornos autohospedados** en la [página de administrador de **Entornos en la nube**](https://claude.ai/admin-settings/cloud-environments), que requiere que [sesiones en la nube](/docs/es/claude-code-on-the-web) estén habilitadas para la organización.
* **Retención cero de datos**: no disponible para organizaciones con [Retención cero de datos](/docs/es/zero-data-retention) habilitada.
* **Inferencia de modelos**: las sesiones utilizan la API de Anthropic, y la inferencia no se puede enrutar a través de [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry](/docs/es/third-party-integrations), o una [puerta de enlace LLM](/docs/es/llm-gateway).
* **Superficies**: sesiones iniciadas desde [claude.ai/code](https://claude.ai/code), las aplicaciones móviles y de escritorio, [rutinas programadas](/docs/es/routines), y la terminal, con [`claude --cloud`](/docs/es/claude-code-on-the-web#from-terminal-to-cloud) o un [envío `--environment`](/docs/es/self-hosted-environments-testing#run-the-test-loop), pueden ejecutarse en entornos autohospedados. Las sesiones de [Claude Tag](https://claude.com/docs/claude-tag/overview) también pueden ejecutarse en ellas, pero Claude aún no puede usar [Paquetes de acceso](https://claude.com/docs/claude-tag/concepts/glossary#access-bundle) en esas sesiones. Las sesiones de [Claude Security](/docs/es/claude-security) y [Code Review](/docs/es/code-review) aún no se enrutan a ellas. El soporte para esas dos superficies sigue por separado.
* **Repositorios**: las sesiones clonan repositorios desde GitHub; consulte [Opciones de autenticación de GitHub](/docs/es/claude-code-on-the-web#github-authentication-options).
* **Facturación**: las sesiones en un entorno autohospedado consumen el uso de Claude Code de su organización de la misma manera que las sesiones en entornos alojados por Anthropic.

<h2 id="why-self-host">
  Por qué autohospedar
</h2>

La mayoría de los equipos se benefician mejor de los entornos alojados por Anthropic, que no requieren infraestructura para ejecutar o mantener. El autohospedaje es para equipos cuya red, herramientas o requisitos de cumplimiento requieren mantener la ejecución de sesiones en la infraestructura que controlan. Si ese es su caso, planifique la propiedad operativa que conlleva: construye y mantiene la imagen del ejecutor, opera la flota y controla su red.

A cambio, el autohospedaje le proporciona acceso a la red, herramientas personalizadas y control de cumplimiento:

* **Acceso a la red**: las sesiones se ejecutan dentro de su red y pueden alcanzar servicios internos, bases de datos y registros sin exponerlos a Internet público
* **Herramientas personalizadas**: preinstale compiladores, SDK y CLI internos en su imagen de ejecutor para que cada sesión comience lista para compilar
* **Cumplimiento**: los clones de repositorio y artefactos de compilación permanecen en la infraestructura que controla. El contenido de la sesión aún va a `api.anthropic.com` para inferencia de modelos.

<h2 id="environments-runners-and-sessions">
  Entornos, ejecutores y sesiones
</h2>

Los entornos se administran en la página **Entornos en la nube** en la configuración de administrador de claude.ai; los ejecutores son procesos que inicia y administra en su propia infraestructura.

<h3 id="key-concepts">
  Conceptos clave
</h3>

Estos términos aparecen en todas las páginas autohospedadas:

| Término            | Qué es                                                                                                                                                                                                                                  |
| :----------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Entorno            | Un grupo nombrado de sus ejecutores, creado en la configuración de claude.ai. Las sesiones se enrutan a un entorno, no a un ejecutor individual.                                                                                        |
| Secreto de entorno | La credencial compartida única que los ejecutores utilizan para autenticarse y registrarse con el entorno. Se muestra una vez en la creación del entorno, etiquetado como **clave de entorno** en la interfaz de administrador.         |
| Ejecutor           | El proceso de larga duración que implementa. Un ejecutor se registra con el entorno, recibe un token de ejecutor y sondea sesiones.                                                                                                     |
| Sesión             | Una tarea de Claude Code, iniciada desde claude.ai, la aplicación móvil u otra superficie de Anthropic como una rutina programada o un agente. Cada sesión se ejecuta como un proceso secundario de Claude Code que el ejecutor genera. |

En campos de API, reclamaciones de token y nombres de métricas, el entorno aparece como `pool`, y el ID del entorno es el `pool_id`. La [referencia](/docs/es/self-hosted-environments-reference) asigna los dos términos, incluidos los nombres de bandera `pool` deprecados.

Un ejecutor sirve a un propietario a la vez. La primera sesión que un ejecutor recoge bloquea el ejecutor al propietario de esa sesión, y el ejecutor luego ejecuta sesiones solo para ese propietario, hasta una capacidad configurada. Quién es el propietario depende de cómo se inició la sesión:

* **Sesiones que inicia un usuario**: el propietario es la cuenta de ese usuario.
* **Sesiones de canal de Claude Tag**: Claude las ejecuta sin cuenta de usuario adjunta, por lo que el propietario es el [agente de Claude Tag](https://claude.com/docs/claude-tag/concepts/glossary#agent-identity) que inició la sesión. Cada sesión de canal que ese agente inicia tiene el mismo propietario, quienquiera que haya enviado el mensaje de Slack, por lo que un ejecutor bloqueado a él sirve sesiones que diferentes personas iniciaron cuando lo ejecuta en una `--capacity` superior a uno o con un `--drain-grace-sec` positivo. Un ejecutor bloqueado a un usuario nunca recoge estos, y un ejecutor bloqueado a un agente de Claude Tag nunca recoge sesiones de un usuario.

El tamaño mínimo de la flota es, por lo tanto, el número de propietarios que espera que estén activos a la vez, contando usuarios y agentes de Claude Tag.

<h3 id="session-lifecycle">
  Ciclo de vida de la sesión
</h3>

Cuando un desarrollador inicia una sesión y selecciona su entorno, el plano de control de Anthropic coloca la sesión en la cola del entorno. Desde allí:

1. Un ejecutor con capacidad libre reclama la sesión y mantiene un arrendamiento en ella.
2. El ejecutor clona el repositorio en su directorio de trabajo e genera un proceso secundario de Claude Code.
3. El secundario transmite eventos de vuelta sobre HTTPS mientras el ejecutor sigue sondeando; cada sondeo actualiza el arrendamiento y funciona como el latido del corazón.
4. Si el ejecutor deja de sondear durante aproximadamente 60 segundos, el servidor vuelve a encolar la sesión para otro ejecutor.

El ejecutor da a cada solicitud de sondeo 10 segundos. Cuando una solicitud agota el tiempo de espera, se pierde o recibe una respuesta que el ejecutor no puede analizar, el ejecutor sigue sirviendo sus sesiones activas y reintenta después de un segundo o dos en lugar de esperar al siguiente sondeo programado. Por ejemplo, un proxy de interceptación que responde al sondeo con su propia página produce una respuesta que el ejecutor no puede analizar. Cada vez que otra solicitud falla de una de esas maneras, el ejecutor duplica la brecha antes del siguiente reintento, hasta 20 segundos, y acorta la brecha siempre que el arrendamiento esté cerca de expirar.

<h3 id="runner-lifecycle">
  Ciclo de vida del ejecutor
</h3>

La primera sesión que un ejecutor recoge bloquea el ejecutor al propietario de esa sesión, y el ejecutor ejecuta hasta `--capacity` sesiones concurrentes para ese propietario. Mientras el ejecutor tiene sesiones activas y no ha recibido una señal de apagado o alcanzado su tiempo de jubilación, el ejecutor sigue reclamando el trabajo en cola del propietario bloqueado. Lo que sucede una vez que terminan depende de [`--drain-grace-sec`](/docs/es/self-hosted-environments-reference#runner-cli-flags):

* **En el valor predeterminado de `0`**: el ejecutor sale tan pronto como sus sesiones activas terminan, sin sondear más, por lo que el orquestador en el que lo implementa, como Kubernetes, puede reiniciarlo con un disco fresco, listo para servir a cualquier propietario.
* **En un valor positivo**: el ejecutor sigue sondeando la cola del propietario bloqueado durante esa cantidad de segundos antes de salir.

Este ciclo de vida aísla el código extraído de cada propietario sin requerir que el ejecutor elimine el estado del disco entre propietarios.

La forma en que su infraestructura detiene un ejecutor decide si necesita `--retire-at`. Una eliminación que entrega `SIGTERM` no necesita bandera: el ejecutor drena como [Tiempo de apagado](/docs/es/self-hosted-environments-deploy#shutdown-timing) describe, o sigue sirviendo las sesiones que ya tiene cuando establece [`--defer-shutdown-max-min`](/docs/es/self-hosted-environments-deploy#defer-the-drain-past-the-first-signal). Si su infraestructura en su lugar destruye hosts en un tiempo de reloj de pared conocido sin una señal, o con un período de gracia demasiado corto para drenar, como un límite de vida útil de sandbox o reclamación de instancia spot, pase `--retire-at <epoch-seconds>` establecido a unos minutos antes de ese tiempo. En el tiempo de jubilación:

1. El ejecutor deja de aceptar trabajo nuevo.
2. El ejecutor libera cada sesión activa a través de la misma ruta de liberación que usa la bandera [`--release-idle-session-min`](/docs/es/self-hosted-environments-reference#runner-cli-flags), por lo que la sesión se reanuda en un ejecutor fresco cuando el usuario envía su siguiente mensaje. Cuándo el ejecutor libera cada sesión depende de su estado:
   * El ejecutor libera una sesión que está en medio de un turno tan pronto como ese turno termina.
   * Cuando un turno termina y deja tareas en segundo plano ejecutándose, el ejecutor espera hasta 60 segundos para ellas, luego libera la sesión incluso si aún se están ejecutando. Si las tareas han terminado pero el turno de seguimiento que lee sus resultados aún no se ha ejecutado, el ejecutor mantiene la sesión hasta que ese turno termina, y espera no más de [`SELF_HOSTED_RUNNER_BG_RESULT_GRACE_MS`](/docs/es/self-hosted-environments-reference#environment-variable-only-settings) para que ese turno comience.
3. El ejecutor sale 0 una vez que todas sus sesiones se liberan.

Un turno que sobrevive a la eliminación aún se pierde; [Tiempo de apagado](/docs/es/self-hosted-environments-deploy#shutdown-timing) cubre el tamaño del margen. Sin `--retire-at`, una eliminación de host sin señal es indistinguible de un bloqueo: el plano de control registra un trabajador perdido en lugar de una liberación limpia, y la sesión se vuelve a encolar a otro ejecutor.

<h3 id="network-paths">
  Rutas de red
</h3>

El ejecutor y sus sesiones hacen varios tipos de conexión saliente, y no se requiere conectividad entrante desde Anthropic:

* **Plano de control**: el ejecutor sondea `api.anthropic.com` para trabajo y publica eventos de progreso de configuración y falla, todo HTTPS saliente. El sondeo funciona como el latido del corazón del ejecutor.
* **Conector SCM**: el orquestador opcional [conector SCM](/docs/es/self-hosted-environments-reference#scm-connector-flags) tunnel es la única conexión WebSocket.
* **Git**: el ejecutor clona desde y empuja a su host de git sobre HTTPS o SSH, autenticado con credenciales que su implementación proporciona; [Configurar git](/docs/es/self-hosted-environments-deploy#configure-git) cubre las opciones, incluidas credenciales acuñadas por sesión y el [proxy de git de Anthropic](/docs/es/self-hosted-environments-deploy#use-the-anthropic-git-proxy), que enruta git a través de `api.anthropic.com` en su lugar.
* **Secundario de sesión**: el proceso secundario de Claude Code mantiene el flujo de eventos de la sesión a `api.anthropic.com`, y realiza sus propias llamadas salientes para inferencia de modelos y para comandos de git ejecutados durante la sesión. Consulte [Requisitos de red](/docs/es/self-hosted-environments-deploy#network-requirements) para la lista completa de salida. El [diagrama anterior](#how-self-hosted-environments-work) muestra estas rutas, aparte del conector SCM opcional.

La inferencia de modelos utiliza la API de Anthropic. El plano de control entrega el punto final de la API a cada sesión, y la sesión se autentica con un token OAuth emitido por Anthropic con alcance de sesión, por lo que la inferencia no se puede enrutar a través de [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry](/docs/es/third-party-integrations), o una [puerta de enlace LLM](/docs/es/llm-gateway) en entornos autohospedados.

Los proxies de salida corporativos son compatibles. El ejecutor y el [orquestador de escalado automático](/docs/es/self-hosted-environments-configuration#on-demand-runners) opcional honran el proxy y las variables de entorno mTLS descritas en [Configuración de red](/docs/es/network-config), como `HTTPS_PROXY` y `NO_PROXY`; establézcalas en el entorno de cada proceso. Las variables cubren llamadas de plano de control, el WebSocket del [conector SCM](/docs/es/self-hosted-environments-reference#scm-connector-flags) del orquestador, y el clon integrado para remotos HTTPS, y las sesiones las heredan del ejecutor. El flujo de sesión utiliza eventos enviados por servidor sobre HTTPS, por lo que un proxy en la ruta no debe almacenar en búfer respuestas.

Si su proxy también requiere un encabezado `Proxy-Authorization`, el ejecutor puede agregarlo a cada conexión que abre al proxy; consulte [Autenticarse en un proxy de salida](/docs/es/self-hosted-environments-deploy#authenticate-to-an-egress-proxy).

<h2 id="what-stays-on-your-infrastructure">
  Qué permanece en su infraestructura
</h2>

Los clones de repositorio, artefactos de compilación, secretos y cualquier archivo que una sesión cree o modifique permanecen en las máquinas que aprovisiona. La conversación en sí, incluidas indicaciones, respuestas y resultados de herramientas, va a `api.anthropic.com` para inferencia de modelos, y Anthropic almacena la transcripción de la sesión para que pueda reanudar la sesión desde otra [superficie compatible](#availability-and-limitations).

Un entorno autohospedado mueve la ejecución de sesiones a su red. El plano de control sigue siendo alojado por Anthropic: la orquestación de sesiones, el encolamiento y la interfaz de claude.ai continúan ejecutándose en la infraestructura de Anthropic.

<h2 id="get-started">
  Empezar
</h2>

Las páginas de entornos autohospedados se organizan por lo que está haciendo:

* [Inicio rápido](/docs/es/self-hosted-environments-quickstart): instale Claude Code, cree un entorno, inicie un ejecutor y enrute su primera sesión
* [Implementar en producción](/docs/es/self-hosted-environments-deploy): endurecimiento de seguridad, salida de red, credenciales de git, recetas de Kubernetes y Compose, problemas conocidos y solución de problemas
* [Personalizar sesiones](/docs/es/self-hosted-environments-configuration): scripts de contenedor para credenciales por sesión, hooks de ciclo de vida, ejecutores bajo demanda, servidores MCP y permisos
* [Prueba de extremo a extremo](/docs/es/self-hosted-environments-testing): una prueba de humo de CI que verifica una imagen de ejecutor antes de promoverla
* [Referencia](/docs/es/self-hosted-environments-reference): cada bandera de CLI, variable de entorno, métrica y el punto final de salud
* [Verificar identidad de sesión](/docs/es/self-hosted-environments-identity): valide el token de sesión desde sus propios servicios antes de otorgar acceso
