> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ejecutar Claude Code a través de una puerta de enlace

> Enrute Claude Code a través de una puerta de enlace autohospedada para credenciales centralizadas, seguimiento de uso y controles de costos. Cubre la arquitectura, la puerta de enlace de aplicaciones Claude de Anthropic y el uso de otros productos de puerta de enlace.

Una puerta de enlace es un proxy que su organización ejecuta entre Claude Code y un proveedor de modelos. Claude Code envía tráfico de API a la puerta de enlace en lugar de directamente al proveedor, y la puerta de enlace lo reenvía usando una credencial que su organización posee. Los desarrolladores se autentican en la puerta de enlace en lugar de poseer credenciales del proveedor, por lo que la autenticación, el seguimiento de uso, los presupuestos y el registro de auditoría ocurren en un único lugar que usted controla.

Claude Code incluye una puerta de enlace autohospedada, [Claude apps gateway](/docs/es/claude-apps-gateway), en el binario `claude`, por lo que no tiene que adoptar un producto de puerta de enlace separado para ejecutar uno. Si su organización ya ejecuta una [puerta de enlace LLM](/docs/es/llm-gateway), Claude Code también funciona con esa.

Esta página cubre:

* [Cómo una puerta de enlace se sitúa entre Claude Code y su proveedor](#how-a-gateway-works)
* [Elegir entre la puerta de enlace de aplicaciones Claude y una puerta de enlace que ya ejecuta](#choose-a-gateway)
* [Cómo las puertas de enlace interactúan con las suscripciones de claude.ai](#subscriptions-and-gateways)
* [Qué se configura por separado de la puerta de enlace](#configure-separately-from-the-gateway)

<h2 id="how-a-gateway-works">
  Cómo funciona una puerta de enlace
</h2>

Cada Claude Code del desarrollador se apunta a la dirección de la puerta de enlace y se autentica con una credencial emitida por la puerta de enlace.

La puerta de enlace autentica al desarrollador, aplica las reglas de acceso y presupuesto que configure, y reenvía la solicitud a su proveedor con la credencial de la organización. El proveedor puede ser la API de Anthropic o un [proveedor de nube](/docs/es/third-party-integrations) como Amazon Bedrock, Agent Platform de Google Cloud o Microsoft Foundry; la configuración de la puerta de enlace lo decide. Con la puerta de enlace de aplicaciones Claude, u otra puerta de enlace que exponga un único punto de conexión en formato Anthropic, cambiar de proveedor no requiere tocar máquinas de desarrolladores.

<Frame>
  <img src="https://mintcdn.com/claude-code/-uq-4JE0W_JO5Er5/images/llm-gateway-flow.svg?fit=max&auto=format&n=-uq-4JE0W_JO5Er5&q=85&s=1c1a8dcc0cfcc3a58652cc8e28cd3e20" alt="Diagrama que muestra Claude Code enrutado a través de una puerta de enlace. En una zona de máquinas de desarrolladores, la CLI de Claude Code y la extensión de VS Code envían solicitudes a la dirección de la puerta de enlace con una credencial por desarrollador. En una zona etiquetada como su infraestructura, la puerta de enlace maneja la autenticación, el seguimiento de uso, los presupuestos y el enrutamiento, y reenvía solicitudes con la credencial de su organización. En una zona de proveedores de modelos, una flecha sólida conduce al proveedor que configura, mostrado como la API de Anthropic, y flechas punteadas conducen a otras opciones de proveedor, ilustradas con Amazon Bedrock, Google Cloud y Microsoft Foundry como ejemplos." width="780" height="322" data-path="images/llm-gateway-flow.svg" />
</Frame>

Hay dos tipos de credencial involucrados:

* **Credencial del desarrollador**: cada desarrollador posee la suya propia, emitida por la puerta de enlace. Los autentica en la puerta de enlace e los identifica en el seguimiento de uso
* **Credencial del proveedor**: la puerta de enlace posee una credencial para su cuenta de proveedor, compartida por todo el tráfico reenviado

<h2 id="choose-a-gateway">
  Elegir una puerta de enlace
</h2>

Claude Code funciona con la puerta de enlace propia de Anthropic o con una puerta de enlace que su organización ya ejecuta.

<h3 id="claude-apps-gateway">
  Puerta de enlace de aplicaciones Claude
</h3>

La puerta de enlace de aplicaciones Claude es la puerta de enlace autohospedada propia de Anthropic, incluida en el binario `claude`. Enruta a Amazon Bedrock, Claude Platform en AWS, Google Cloud, Microsoft Foundry o la API de Anthropic como el proveedor ascendente. Los desarrolladores inician sesión con su proveedor de identidad corporativa a través de `/login`, la puerta de enlace aplica el acceso a modelos y [configuración administrada](/docs/es/permissions#managed-settings) por grupo de IdP, y emite métricas de uso del [Protocolo OpenTelemetry (OTLP)](/docs/es/monitoring-usage) a su propia pila de observabilidad.

Debido a que se construye y prueba junto con cada versión de Claude Code, reenvía los encabezados y campos de solicitud que Claude Code envía. Una puerta de enlace mantenida por separado necesita que sus [reglas de reenvío se actualicen](/docs/es/llm-gateway-protocol#forward-as-open-lists) a medida que esos encabezados y campos cambian con cada versión; la puerta de enlace de aplicaciones Claude se lanza con la CLI, por lo que no hay lista que mantener actualizada. Consulte [Disponibilidad y limitaciones](/docs/es/claude-apps-gateway#availability-and-limitations) para el pequeño conjunto de características que se comportan de manera diferente en una sesión de puerta de enlace.

El inicio de sesión de la puerta de enlace es un paso de SSO del navegador, y no hay flujo de token de servicio, por lo que una canalización de CI sin un desarrollador para aprobar el inicio de sesión no puede autenticarse a través de ella; configure esos contra su proveedor directamente. Las sesiones del SDK de Agent y las ejecuciones de `claude -p` en una máquina donde un desarrollador ha iniciado sesión usan la sesión de puerta de enlace de esa máquina y se rigen por sus políticas. Consulte [Canalizaciones de CI y máquinas remotas](/docs/es/claude-apps-gateway#ci-pipelines-and-remote-machines).

Consulte [Puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) para implementarla.

<h3 id="other-gateways">
  Otras puertas de enlace
</h3>

Si su organización ya ejecuta una puerta de enlace LLM o puerta de enlace de API, puede usarla en su lugar. Anthropic no respalda, mantiene ni audita otros productos de puerta de enlace, y no admite enrutar Claude Code a modelos que no sean de Claude a través de ninguna puerta de enlace. Consulte [Otras puertas de enlace LLM](/docs/es/llm-gateway) para la lista de verificación de implementación del administrador, lo que una puerta de enlace debe implementar y cómo apuntar Claude Code a ella.

<h2 id="subscriptions-and-gateways">
  Suscripciones y puertas de enlace
</h2>

Cuando los desarrolladores se conectan a través de una puerta de enlace con una credencial de puerta de enlace, el uso se factura a la cuenta de proveedor de su organización a tasas de API, y sus suscripciones de claude.ai no se usan ni se cobran. Configurar [`ANTHROPIC_AUTH_TOKEN`](/docs/es/env-vars) para una puerta de enlace que ejecuta, o iniciar sesión en una puerta de enlace de aplicaciones Claude con `/login`, desactiva el inicio de sesión de suscripción para esa sesión. Cada solicitud reenviada bajo esa credencial se cobra a la cuenta detrás de la credencial del proveedor de la puerta de enlace.

La excepción es configurar solo `ANTHROPIC_BASE_URL`, sin credencial de puerta de enlace. Las solicitudes aún se enrutan a través de la puerta de enlace, pero un inicio de sesión de claude.ai guardado permanece como la credencial activa, por lo que los límites de uso y la facturación de la suscripción se aplican. [Otras puertas de enlace LLM](/docs/es/llm-gateway#subscriptions-and-gateways) cubre esa configuración y lo que la puerta de enlace tiene que reenviar para que funcione.

<h2 id="configure-separately-from-the-gateway">
  Configurar por separado de la puerta de enlace
</h2>

Una puerta de enlace enruta solicitudes de API de modelos. Algunas cosas que podría esperar que maneje se configuran en otro lugar:

* **Qué modelo responde**: seleccione el modelo con el comando `/model` o [variables de entorno de modelo](/docs/es/model-config#setting-your-model). La puerta de enlace decide dónde van las solicitudes, no qué modelo selecciona el desarrollador. La puerta de enlace de aplicaciones Claude puede limitar la opción con una lista de permitidos `availableModels` por grupo, pero el desarrollador aún elige dentro de ella.
* **Otro tráfico de red**: Claude Code en sí envía comprobaciones de versión y descargas directamente a Anthropic, separado de la ruta de la puerta de enlace. Si la secuencia de telemetría del cliente opcional también está activada depende de su proveedor; la [tabla de valores predeterminados de telemetría](/docs/es/data-usage#telemetry-services) cubre cada caso. En una sesión de puerta de enlace de aplicaciones Claude con sesión iniciada, la credencial de la puerta de enlace desactiva el análisis vinculado a Anthropic y, cuando se configura el [reenvío de telemetría](/docs/es/claude-apps-gateway-config#telemetry), fija la exportación de OTLP a la puerta de enlace. Su red aún necesita salida a los [dominios requeridos](/docs/es/network-config), o configure [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/es/env-vars) para desactivar las secuencias opcionales.
* **Proxies HTTP corporativos**: un `HTTPS_PROXY` se sitúa entre Claude Code y cada servidor con el que habla, incluida la puerta de enlace. Si su red requiere uno, [configure el proxy](/docs/es/network-config) además de la puerta de enlace. Para una puerta de enlace de aplicaciones Claude que usted aloja, [el inicio de sesión verifica que el host del proxy también esté en una red privada](/docs/es/claude-apps-gateway#prerequisites); si no es así, agregue el host de la puerta de enlace a `NO_PROXY` para que la CLI se conecte a él directamente.

<h2 id="next-steps">
  Próximos pasos
</h2>

La siguiente página depende de quién ejecute la puerta de enlace. La puerta de enlace de Anthropic se ejecuta desde el binario `claude` y tiene su propia guía de configuración; una puerta de enlace que su organización ya ejecuta tiene un protocolo que implementar y una lista de verificación de implementación del administrador.

* [Puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) para implementar la puerta de enlace autohospedada propia de Anthropic con inicio de sesión SSO y telemetría OTLP
* [Otras puertas de enlace LLM](/docs/es/llm-gateway) para lo que una puerta de enlace que su organización ya ejecuta debe implementar, y cómo apuntar Claude Code a ella
* [Configurar Claude Code para su organización](/docs/es/admin-setup) para las decisiones de implementación más amplias de las que una puerta de enlace es una parte
