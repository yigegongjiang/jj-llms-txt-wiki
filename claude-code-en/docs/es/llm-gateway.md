> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Otras puertas de enlace LLM

> Enrute Claude Code a través de una puerta de enlace LLM que su organización ya ejecuta. Cubre la conexión de Claude Code a una puerta de enlace, el despliegue de una para su organización, y qué envía Claude Code a una puerta de enlace.

Esta sección cubre el uso de un producto de puerta de enlace que su organización ya ejecuta, en lugar de [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway). Para saber qué es una puerta de enlace, cómo se sitúa entre Claude Code y su proveedor, y cómo elegir entre puerta de enlace de aplicaciones Claude y otro producto, consulte la [descripción general de puertas de enlace](/docs/es/gateways).

<Note>
  * Si es un desarrollador que se conecta a una puerta de enlace existente: [conecte Claude Code a su puerta de enlace](/docs/es/llm-gateway-connect)
  * Si es un administrador que despliega una puerta de enlace para su organización: [despliegue y distribuya una puerta de enlace](/docs/es/llm-gateway-rollout)
  * Si está configurando un producto de puerta de enlace: la [referencia del protocolo de puerta de enlace](/docs/es/llm-gateway-protocol)
</Note>

Cualquier puerta de enlace que exponga un [formato de API compatible](/docs/es/llm-gateway-protocol#api-formats) funciona. Anthropic no respalda, mantiene ni audita productos de puerta de enlace de terceros, y no admite enrutar Claude Code a modelos que no sean de Claude a través de ninguna puerta de enlace. Despliegue la puerta de enlace siguiendo su propia documentación, luego complete el lado de Claude Code con los [pasos de despliegue a continuación](#roll-out-a-gateway).

<h2 id="what-a-gateway-provides">
  Qué proporciona una puerta de enlace
</h2>

Una puerta de enlace le da a su organización un lugar para gestionar:

* **Credenciales**: la clave del proveedor permanece del lado del servidor; los desarrolladores tienen credenciales de puerta de enlace en su lugar
* **Seguimiento de uso**: atribuya el uso por desarrollador o equipo, independientemente de qué proveedor sirva la solicitud
* **Controles de costos**: aplique presupuestos y límites de velocidad en un solo lugar
* **Registro de auditoría**: registre cada solicitud de modelo para cumplimiento normativo
* **Cambio de proveedor**: cambie el proveedor en la configuración de la puerta de enlace, sin tocar máquinas de desarrolladores

Todos estos excepto el cambio de proveedor se aplican si el upstream es la API de Anthropic o un [proveedor en la nube](/docs/es/third-party-integrations). El cambio de proveedor sin reconfigurar máquinas de desarrolladores también depende de que la puerta de enlace exponga un único [punto final en formato Anthropic](/docs/es/llm-gateway-protocol#api-formats) independientemente del upstream; una puerta de enlace que exponga el formato propio de un proveedor vincula la configuración del cliente a ese proveedor.

El compromiso es que la puerta de enlace se convierte en infraestructura que su organización opera. Claude Code añade capacidades con cada lanzamiento, y una puerta de enlace que no las reenvía rompe las características correspondientes, por lo que el producto de puerta de enlace necesita mantenerse actualizado a medida que Claude Code evoluciona. La [referencia del protocolo de puerta de enlace](/docs/es/llm-gateway-protocol) cubre qué reenviar.

<h2 id="roll-out-a-gateway">
  Desplegar una puerta de enlace
</h2>

Cuando esté listo para desplegar una puerta de enlace LLM a su organización, la secuencia es la misma sin importar qué producto de puerta de enlace elija:

1. Despliegue la puerta de enlace y déle su credencial de proveedor, para que pueda autenticar las solicitudes que reenvía.
2. Emita a cada desarrollador una credencial de puerta de enlace, para que el uso se atribuya al desarrollador y la desvinculación revoque una credencial.
3. Distribuya la configuración a través de un [archivo de configuración administrada](/docs/es/settings#settings-files) y su herramienta de secretos, para que cada máquina reciba la URL base y una credencial. Cuando ambas se distribuyen, los desarrolladores no configuran nada. Si no tiene distribución de configuración en su lugar, los desarrolladores siguen la [página de conexión](/docs/es/llm-gateway-connect) para establecer las variables ellos mismos.
4. Haga que cada desarrollador [verifique la configuración en Claude Code](/docs/es/llm-gateway-connect#check-for-an-existing-configuration), para que los problemas de distribución salgan a la superficie antes de que dependan de la puerta de enlace.

[Despliegue una puerta de enlace LLM para su organización](/docs/es/llm-gateway-rollout) camina cada paso y muestra los archivos de configuración a distribuir en cada uno. La puerta de enlace es una parte de la configuración de la organización; para aplicación de políticas, visibilidad de uso y decisiones de manejo de datos, consulte [Configure Claude Code para su organización](/docs/es/admin-setup).

<h2 id="subscriptions-and-gateways">
  Suscripciones y puertas de enlace
</h2>

Mientras una [variable de credencial de puerta de enlace](/docs/es/llm-gateway-connect#set-the-credential-variable) o `apiKeyHelper` está activa, la suscripción de claude.ai de un desarrollador no se usa: la credencial reemplaza el inicio de sesión de suscripción para esa sesión, y los límites de uso de la suscripción no se aplican. Ese tráfico se factura por token a quien sea propietario de la credencial que la puerta de enlace reenvía, como la cuenta de Consola de Anthropic de su organización, o su cuenta de Amazon Bedrock, Agent Platform de Google Cloud o Foundry de Microsoft cuando la puerta de enlace enruta allí.

[`ANTHROPIC_BASE_URL`](/docs/es/llm-gateway-connect#set-the-base-url-and-credential) es la variable que apunta Claude Code a la puerta de enlace. Establecer solo esa variable, sin una credencial de puerta de enlace, no reemplaza la suscripción. Las solicitudes aún se enrutan a través de la puerta de enlace, pero un inicio de sesión de claude.ai guardado permanece como la credencial activa, por lo que sus límites de uso y facturación se aplican. Las puertas de enlace que pasan este tráfico a Anthropic deben reenviar la capacidad OAuth en `anthropic-beta`; consulte la [referencia de encabezados de solicitud](/docs/es/llm-gateway-protocol#request-headers).

<h2 id="related-pages">
  Páginas relacionadas
</h2>

* [Descripción general de puertas de enlace](/docs/es/gateways): cómo funciona una puerta de enlace y cómo elegir entre puerta de enlace de aplicaciones Claude y otro producto
* [Puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway): puerta de enlace autohospedada de Anthropic con inicio de sesión SSO y telemetría OTLP
* [Conecte Claude Code a una puerta de enlace LLM](/docs/es/llm-gateway-connect): establezca la URL base y la credencial en su propia máquina, con configuración por superficie y una tabla de solución de problemas
* [Despliegue una puerta de enlace LLM para su organización](/docs/es/llm-gateway-rollout): la lista de verificación del administrador para desplegar una puerta de enlace, emitir credenciales de desarrollador y distribuir configuración administrada
* [Referencia del protocolo de puerta de enlace](/docs/es/llm-gateway-protocol): qué envía Claude Code a una puerta de enlace, para operadores que configuran una, cubriendo puntos finales, encabezados a reenviar y paso a través de características
