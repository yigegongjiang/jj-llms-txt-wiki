> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Acelera las respuestas con el modo rápido

> Obtén respuestas más rápidas de Opus en Claude Code al activar el modo rápido.

<Note>
  El modo rápido está en [vista previa de investigación](#research-preview). La función, los precios y la disponibilidad pueden cambiar según los comentarios.
</Note>

El modo rápido es una configuración de alta velocidad para Claude Opus, haciendo que el modelo sea hasta 2.5x más rápido a un costo más alto por token. Actívalo con `/fast` cuando necesites velocidad para trabajo interactivo como iteración rápida o depuración en vivo, y desactívalo cuando el costo sea más importante que la latencia.

El modo rápido no es un modelo diferente. Utiliza Claude Opus con una configuración de API diferente que prioriza la velocidad sobre la eficiencia de costos. Obtienes la misma calidad y capacidades con respuestas más rápidas. El modo rápido es compatible con Opus 5.5, Opus 5 y Opus 4.8. No está disponible en Sonnet, Haiku u otros modelos.

Opus 4.7 no admite el modo rápido, por lo que cambiar a él desactiva el modo rápido. El modo rápido para Opus 4.7 fue deprecado el 25 de junio de 2026 y se eliminó el 24 de julio de 2026.

Lo que debes saber:

* Usa `/fast` para activar o desactivar el modo rápido en Claude Code CLI. La [extensión VS Code](/docs/es/vs-code) ofrece un comando **Toggle fast mode** cuando el modelo seleccionado admite el modo rápido. Claude Code guarda ese cambio en tu configuración [`fastMode`](#toggle-fast-mode).
* Los precios del modo rápido por MTok de entrada/salida son \$8/\$40 en Opus 5.5 y \$10/\$50 en Opus 5 y Opus 4.8.
* Disponible para usuarios de Claude Code en planes de suscripción (Pro/Max/Team/Enterprise) y en Claude Console. Las organizaciones Team y Enterprise necesitan que un Propietario lo habilite primero, y las organizaciones Console necesitan que se aprovisione el acceso primero, ambos descritos en [Requisitos](#requirements).
* Para los usuarios de Claude Code en planes de suscripción (Pro/Max/Team/Enterprise), el modo rápido está disponible solo a través de créditos de uso y no está incluido en los límites de velocidad de la suscripción.

<h2 id="toggle-fast-mode">
  Activar el modo rápido
</h2>

En la CLI, activa el modo rápido de cualquiera de estas formas:

* Ejecuta `/fast`, presiona Space para activar o desactivar, luego presiona Enter para confirmar
* Establece `"fastMode": true` en tu [archivo de configuración de usuario](/docs/es/settings)

De forma predeterminada, el modo rápido que activas en una sesión interactiva persiste entre sesiones. Puedes configurar el modo rápido para que se reinicie cada sesión. Consulta [opción de participación por sesión](#require-per-session-opt-in) para obtener más detalles.

Fuera de una [sesión en la nube](#use-fast-mode-in-cloud-sessions), en [modo no interactivo](/docs/es/headless) con la bandera `-p`, `/fast` funciona solo en una sesión iniciada con el modo rápido en su valor [`--settings`](/docs/es/cli-reference#cli-flags), por ejemplo `claude -p --settings '{"fastMode": true}'`; el cambio se aplica solo a esa sesión y no se guarda como tu valor predeterminado. El formulario `-p` requiere Claude Code v2.1.205 o posterior. En cualquier otro lugar en modo no interactivo, el comando reporta que el modo rápido no está disponible.

Puedes ejecutar `/fast` mientras Claude está trabajando, y Claude Code activa o desactiva el modo rápido sin esperar a que termine el turno. Claude Code termina el turno en ejecución a su velocidad original, por lo que el cambio de velocidad entra en vigor a partir de tu siguiente turno. Si tu modelo actual no admite el modo rápido, activarlo también cambia tu modelo, y Claude Code utiliza el nuevo modelo a partir de su siguiente solicitud en ese turno.

Para la mejor eficiencia de costos, habilita el modo rápido al inicio de una sesión en lugar de cambiar a mitad de la conversación. Consulta [comprender la compensación de costos](#understand-the-cost-tradeoff) para obtener más detalles.

Cuando habilitas el modo rápido:

* Si tu modelo actual no admite el modo rápido, Claude Code cambia a Opus
* Verás un mensaje de confirmación: "Fast mode ON"
* Un pequeño icono `↯` aparece junto al prompt mientras el modo rápido está activo
* Ejecuta `/fast` nuevamente en cualquier momento para verificar si el modo rápido está activado o desactivado

Opus 5.5 es el valor predeterminado del modo rápido en Claude Code v2.1.280 y posterior. Antes de v2.1.280, el modo rápido utilizaba Opus 5 de forma predeterminada a partir de v2.1.219, Opus 4.8 en v2.1.154 a v2.1.218, y Opus 4.7 en v2.1.142 a v2.1.153.

Cuando desactivas el modo rápido con `/fast` nuevamente, permaneces en Opus. Para cambiar a un modelo diferente, usa `/model`.

<h3 id="switch-models-while-fast-mode-is-on">
  Cambiar modelos mientras el modo rápido está activado
</h3>

El modo rápido sigue tus cambios de modelo en ambas direcciones:

* **Cambiar a otro**: cuando cambias a un modelo que no admite el modo rápido, Claude Code desactiva el modo rápido. Esto incluye Opus 4.7; antes de v2.1.221, el modo rápido permanecía activado después de un cambio a Opus 4.7 y la API rechazaba las solicitudes.
* **Cambiar de vuelta**: cambiar de vuelta a un modelo Opus compatible activa el modo rápido nuevamente cuando tu preferencia de modo rápido guardada está activada, la misma preferencia con la que una nueva sesión comienza de forma predeterminada. Un cambio de modelo nunca activa el modo rápido para una sesión cuya preferencia guardada está desactivada, y con [opción de participación por sesión](#require-per-session-opt-in) configurada, cambiar de vuelta tampoco lo activa; ejecuta `/fast` para reactivarlo.

Siempre que un cambio de modelo activa o desactiva el modo rápido, Claude Code muestra una confirmación `Fast mode ON` o `Fast mode OFF`, y el icono `↯` aparece mientras el modo rápido está activado. Esto se aplica tanto si cambias con `/model`, con [`/config model=<model>`](/docs/es/settings), como desde un dispositivo conectado a través de [Control Remoto](/docs/es/remote-control).

Claude Code reenvía el estado del modo rápido de la sesión a los dispositivos conectados a través de Control Remoto después de un cambio de modelo, una reconexión, o una [verificación de disponibilidad](#use-fast-mode-behind-proxies-and-llm-gateways) fallida.

<h3 id="use-fast-mode-in-cloud-sessions">
  Usar el modo rápido en sesiones en la nube
</h3>

El modo rápido funciona en [sesiones en la nube](/docs/es/claude-code-on-the-web) cuando está disponible en tu cuenta, ya sea que la sesión se ejecute en infraestructura administrada por Anthropic o en un [ejecutor autohospedado](/docs/es/self-hosted-environments). Requiere Claude Code v2.1.271 o posterior en el entorno de la sesión.

Escribe `/fast on` en la sesión para activar el modo rápido. Se mantiene activado solo para esa sesión y no se guarda como tu valor predeterminado. Los [requisitos](#requirements) también se aplican en sesiones en la nube.

<h2 id="understand-the-cost-tradeoff">
  Comprender la compensación de costos
</h2>

El modo rápido tiene precios por token más altos que el Opus estándar:

| Modelo   | Entrada (MTok) | Salida (MTok) |
| -------- | -------------- | ------------- |
| Opus 5.5 | \$8            | \$40          |
| Opus 5   | \$10           | \$50          |
| Opus 4.8 | \$10           | \$50          |

Los precios del modo rápido son fijos en toda la ventana de contexto de 1M tokens. Para la tarifa estándar de Opus con la que comparar, consulte la [referencia de precios de Claude](https://platform.claude.com/docs/es/about-claude/pricing).

La primera vez que habilita el modo rápido en una conversación, paga el precio completo del token de entrada sin caché del modo rápido para todo el contexto de la conversación. Cuanto más profundo esté en una conversación, más cuesta esto, por lo que habilitar el modo rápido desde el inicio es más económico. El costo se aplica una vez por conversación, por lo que desactivar y activar el modo rápido nuevamente más tarde no lo repite. Para el mecanismo, consulte [cómo el modo rápido interactúa con el caché de indicaciones](/docs/es/prompt-caching#turning-on-fast-mode).

<h3 id="see-where-fast-mode-spend-appears">
  Vea dónde aparece el gasto del modo rápido
</h3>

Usted ve el gasto del modo rápido en un lugar diferente dependiendo de cómo inició sesión, así que primero ejecute [`/status`](/docs/es/commands) para verificar. Si muestra una fila `Login method` como `Claude Max account`, inició sesión con una suscripción de Claude. Si muestra una fila `API key` en su lugar, sus solicitudes se facturan a una organización de Claude Console.

* **Pro y Max**: usted paga el modo rápido desde sus créditos de uso. Vaya a [**Settings > Usage**](https://claude.ai/settings/usage) en claude.ai, donde la sección **Usage credits** muestra cuánto ha gastado en créditos de uso este mes. Esa cifra incluye el modo rápido pero no lo desglosa por separado.
* **Team y Enterprise**: su organización paga el uso del modo rápido desde sus créditos de uso. Para ver su propio gasto de créditos de uso, ejecute [`/usage`](/docs/es/costs#check-your-usage-credits-spend). Para ver dónde su organización ve ese gasto, consulte [Claude para Teams y Enterprise](/docs/es/costs#claude-for-teams-and-enterprise).
* **Claude Console**: su organización paga el modo rápido junto con el resto de su uso de API. En las páginas [Usage](https://platform.claude.com/usage) y [Cost](https://platform.claude.com/cost) de Console, seleccione **Speed (Research Preview)** en el menú **Group by** para separar el modo rápido del uso de velocidad estándar. Usted ve esa opción solo cuando el rango de fechas seleccionado incluye uso del modo rápido.

<h2 id="decide-when-to-use-fast-mode">
  Decidir cuándo usar el modo rápido
</h2>

El modo rápido es mejor para trabajo interactivo donde la latencia de respuesta es más importante que el costo:

* Iteración rápida en cambios de código
* Sesiones de depuración en vivo
* Trabajo sensible al tiempo con plazos ajustados

El modo estándar es mejor para:

* Tareas autónomas largas donde la velocidad importa menos
* Procesamiento por lotes o canalizaciones CI/CD
* Cargas de trabajo sensibles al costo

<h3 id="fast-mode-vs-effort-level">
  Modo rápido versus nivel de esfuerzo
</h3>

El modo rápido y el nivel de esfuerzo afectan la velocidad de respuesta, pero de manera diferente:

| Configuración                  | Efecto                                                                                                   |
| ------------------------------ | -------------------------------------------------------------------------------------------------------- |
| **Modo rápido**                | Misma calidad de modelo, latencia más baja, costo más alto                                               |
| **Nivel de esfuerzo más bajo** | Menos tiempo de pensamiento, respuestas más rápidas, calidad potencialmente más baja en tareas complejas |

Puedes combinar ambos: usa el modo rápido con un [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) más bajo para máxima velocidad en tareas sencillas.

<h2 id="requirements">
  Requisitos
</h2>

El modo rápido requiere todos los siguientes:

* **Solo API de Anthropic o suscripción**: el modo rápido está disponible a través de la API de Anthropic Console y para planes de suscripción de Claude usando créditos de uso. No está disponible en Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry o Claude Platform en AWS. Las organizaciones de Console también deben tener [acceso al modo rápido aprovisionado](#enable-fast-mode-for-your-organization).
* **Créditos de uso activados para planes de suscripción**: en un plan Pro, Max, Team o Enterprise, su cuenta debe tener [créditos de uso](/docs/es/costs#add-usage-credits-to-your-subscription) activados, lo que permite facturación más allá del uso incluido en su plan. Hasta que estén activados, `/fast` muestra "Fast mode requires usage credits". Cómo los activa depende de su plan:
  * En Pro y Max, actívelos en la sección **Usage credits** de [**Settings > Usage**](https://claude.ai/settings/usage) en claude.ai, o ejecute `/usage-credits` para abrir esa página.
  * En Team y Enterprise, un miembro con acceso de facturación los activa para la organización en [**Admin settings > Usage**](https://claude.ai/admin-settings/usage), y un miembro sin acceso ejecuta `/usage-credits` para enviar una solicitud a los administradores de la organización.

<Note>
  El uso del modo rápido se extrae directamente de los créditos de uso, incluso si tiene uso restante en su plan.
</Note>

* **Organización de Console pagada**: las cuentas de Claude Console no utilizan créditos de uso, y su organización paga por el modo rápido por token junto con el resto de su uso de API. En el plan de Evaluación gratuito de Console, `/fast` muestra "Fast mode unavailable during evaluation. Please purchase credits." Para borrarlo, compre créditos en su [configuración de facturación de Console](https://platform.claude.com/settings/billing).
* **Habilitación del propietario para Team y Enterprise**: el modo rápido está deshabilitado de forma predeterminada para organizaciones Team y Enterprise. Un propietario debe [habilitar explícitamente el modo rápido](#enable-fast-mode-for-your-organization) antes de que los usuarios puedan acceder a él.

<Note>
  Cuatro configuraciones de organización pueden bloquear la activación del modo rápido con `/fast`:

  * **Modo rápido no habilitado**: si el modo rápido no ha sido habilitado para su organización, activar el modo rápido con `/fast` muestra "Fast mode has been disabled by your organization."
  * **Modo rápido desactivado por configuración administrada**: si su organización implementa [configuración administrada](/docs/es/managed-settings) que establece [`fastMode: false`](/docs/es/settings-reference#fastmode), activar el modo rápido con `/fast` muestra el mismo mensaje "Fast mode has been disabled by your organization".
  * **Opción de participación requerida por sesión**: la configuración administrada que establece [`fastModePerSessionOptIn: true`](#require-per-session-opt-in) rechaza `/fast on` con el mismo mensaje en todas partes excepto en una sesión de terminal interactiva.
  * **Modelo de modo rápido no permitido**: si la lista de permitidos [`availableModels`](/docs/es/model-config#restrict-model-selection) de su organización excluye el modelo Opus del modo rápido, activarlo se rechaza con "is not in your organization's allowed models". En una sesión ya en ejecución en un modelo Opus permitido que admita modo rápido, `/fast` en su lugar habilita el modo rápido en su modelo actual sin cambiar de modelos.
</Note>

<h3 id="enable-fast-mode-for-your-organization">
  Habilitar el modo rápido para su organización
</h3>

Dónde habilita el modo rápido depende de qué producto usa su organización:

* **Console** (clientes de API): un administrador lo habilita en [Preferencias de Claude Code](https://platform.claude.com/claude-code/preferences). El modo rápido está en [vista previa de investigación](#research-preview), por lo que su organización también debe tener acceso al modo rápido aprovisionado antes de que las solicitudes de modo rápido tengan éxito. Para obtener acceso, póngase en contacto con su gerente de cuenta o únase a la lista de espera, como se describe en [modo rápido en la API de Claude](https://platform.claude.com/docs/en/build-with-claude/fast-mode).

  Sin acceso aprovisionado, la API rechaza cada solicitud de modo rápido con un 429, y Claude Code trata cada rechazo como un [límite de velocidad del modo rápido](#handle-rate-limits). A diferencia del tiempo de espera de un límite de velocidad, los rechazos continúan hasta que se aprovisione el acceso.
* **Claude AI** (Team y Enterprise): un propietario lo habilita en [Admin Settings > Claude Code](https://claude.ai/admin-settings/claude-code)

Otra opción para desactivar completamente el modo rápido es establecer `CLAUDE_CODE_DISABLE_FAST_MODE=1`. Consulte [Variables de entorno](/docs/es/env-vars).

<h3 id="use-fast-mode-behind-proxies-and-llm-gateways">
  Usar el modo rápido detrás de proxies y puertas de enlace LLM
</h3>

Antes de ofrecer el modo rápido, Claude Code verifica la disponibilidad del modo rápido de su organización con una solicitud directa a `api.anthropic.com`. La verificación no sigue [`ANTHROPIC_BASE_URL`](/docs/es/llm-gateway-connect#set-the-base-url-and-credential), por lo que en una red que enruta el tráfico de Claude a través de una [puerta de enlace LLM](/docs/es/llm-gateway) y bloquea la salida directa a `api.anthropic.com`, la verificación falla aunque las solicitudes de inferencia funcionen. La verificación utiliza un [proxy HTTP](/docs/es/network-config#proxy-configuration) configurado, por lo que un bloqueo de red falla la verificación solo donde `api.anthropic.com` es inaccesible incluso a través del proxy.

Cuando la verificación falla, `/fast` reporta "Fast mode unavailable due to network connectivity issues", y las solicitudes se ejecutan a velocidad estándar, incluso cuando su organización tiene el modo rápido habilitado. Una verificación que tuvo éxito en el pasado sigue funcionando desde su resultado en caché, por lo que una verificación bloqueada afecta principalmente a las nuevas instalaciones.

El mismo mensaje de conectividad aparece en una red abierta cuando la verificación llega a `api.anthropic.com` pero presenta una credencial que Anthropic rechaza. Una sesión cuya clave resuelta es una credencial emitida por la puerta de enlace, mantenida en [`ANTHROPIC_API_KEY`](/docs/es/llm-gateway-connect#set-the-base-url-and-credential) o producida por un [`apiKeyHelper`](/docs/es/settings-reference#apikeyhelper), envía la verificación con esa clave, y la solicitud rechazada se reporta como un fallo de conectividad.

Para restaurar el modo rápido, permita la salida directa a `api.anthropic.com` donde un bloqueo de red es la causa, o establezca cualquier variable que coincida con cómo falla la verificación:

* `CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS=1` trata una verificación fallida como disponible y aún respeta una respuesta "disabled by your organization". Úselo cuando su red rechace la conexión, o cuando Anthropic rechace una credencial de puerta de enlace; la lista de permitidos no ayuda en el caso de credencial, ya que nada está bloqueado.
* `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK=1` omite la verificación por completo. Úselo cuando su red intercepte la solicitud en lugar de rechazarla.

Dos configuraciones de puerta de enlace reportan "Fast mode has been disabled by your organization" en lugar del mensaje de conectividad, incluso cuando su organización tiene el modo rápido habilitado:

* Una sesión que se autentica con [`ANTHROPIC_AUTH_TOKEN`](/docs/es/llm-gateway-connect#set-the-base-url-and-credential) únicamente omite la verificación: sin un inicio de sesión en claude.ai o una clave de API de Anthropic, y sin una verificación exitosa en caché, Claude Code trata el modo rápido como deshabilitado por su organización sin enviar la solicitud.
* Un proxy que intercepta la verificación y responde con su propia página, por ejemplo un proxy que inspecciona TLS devolviendo una página de bloqueo HTTP 200, se lee como una respuesta que dice que su organización tiene el modo rápido deshabilitado.

En ambos casos, establezca `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK=1` para restaurar el modo rápido. `CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS` no se aplica a ninguno de estos casos, ya que solo omite verificaciones fallidas y ambos producen una respuesta deshabilitada en su lugar. La lista de permitidos de salida directa no ayuda en el caso de token portador, que nunca envía la solicitud.

Las variables afectan solo la verificación del lado del cliente. Cuando su organización tiene el modo rápido deshabilitado, la API rechaza solicitudes de modo rápido independientemente de si están configuradas. Un rechazo de la API se mantiene incluso con una variable de omisión configurada. Claude Code reintenta la solicitud rechazada a velocidad estándar, desactiva el modo rápido, y `/fast` reporta que su organización ha deshabilitado el modo rápido.

Establecer `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` también suprime la verificación de disponibilidad. Sin una verificación exitosa previamente en caché, `/fast` reporta "Fast mode is currently unavailable"; ambas variables de omisión restauran el modo rápido en esa configuración también.

<h3 id="require-per-session-opt-in">
  Requerir opción de participación por sesión
</h3>

De forma predeterminada, el modo rápido que un usuario activa en una sesión interactiva persiste entre sesiones. Para cambiar esto, establezca `fastModePerSessionOptIn` en `true` en cualquier [archivo de configuración](/docs/es/settings#where-settings-live), lo que hace que cada sesión comience con el modo rápido desactivado y requiere que los usuarios lo habiliten explícitamente con `/fast`. Los propietarios en planes [Team](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_teams#team-&-enterprise) o [Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_enterprise) pueden implementarlo en toda la organización a través de [configuración administrada por servidor](/docs/es/server-managed-settings).

```json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

Esto es útil para controlar costos en organizaciones donde los usuarios ejecutan múltiples sesiones concurrentes. La preferencia del modo rápido del usuario aún se guarda, por lo que eliminar esta configuración restaura el comportamiento persistente predeterminado.

Cuando la configuración administrada establece la clave, `/fast on` funciona solo en una sesión de terminal interactiva. En todas partes, incluyendo [modo no interactivo](/docs/es/headless), la [extensión de VS Code](/docs/es/vs-code), y [sesiones en la nube](#use-fast-mode-in-cloud-sessions), se rechaza con un mensaje que su organización ha deshabilitado el modo rápido.

<h2 id="handle-rate-limits">
  Manejar límites de velocidad
</h2>

El modo rápido tiene límites de velocidad separados del Opus estándar. Todos los modelos Opus compatibles comparten un grupo de límite de velocidad del modo rápido: el uso en cualquiera de ellos se extrae de los mismos límites. Cuando alcanzas el límite de velocidad del modo rápido:

1. El modo rápido automáticamente vuelve a velocidad estándar
2. El icono `↯` se vuelve gris para indicar enfriamiento
3. Continúas trabajando a velocidad y precios estándar
4. Cuando expira el enfriamiento, el modo rápido se vuelve a habilitar automáticamente

Para desactivar el modo rápido manualmente en lugar de esperar el enfriamiento, ejecuta `/fast` nuevamente.

Si se agotan tus créditos de uso a mitad de la sesión, Claude Code reintenta cada solicitud de modo rápido rechazada a velocidad y precios estándar, por lo que continúas trabajando y no hay enfriamiento. La forma en que ves el rechazo depende del tipo de sesión:

* En una sesión interactiva, Claude Code muestra una notificación "Fast mode disabled · usage credits exhausted" (Modo rápido desactivado · créditos de uso agotados) y desactiva el modo rápido para el resto de la sesión. Tu preferencia de modo rápido guardada no cambia; ejecuta `/fast` para volver a activar el modo rápido.
* En [modo no interactivo](/docs/es/headless) con `--output-format stream-json`, y a través del Agent SDK, Claude Code emite el mismo texto en el flujo de mensajes como un mensaje `system` con subtipo `notification`, una vez por turno mientras se te acaban los créditos de uso. El modo rápido permanece activado. Requiere Claude Code v2.1.221 o posterior.

<h2 id="research-preview">
  Vista previa de investigación
</h2>

El modo rápido es una función de vista previa de investigación. Esto significa:

* La función puede cambiar según los comentarios
* La disponibilidad y los precios están sujetos a cambios
* La configuración de API subyacente puede evolucionar

Reporta problemas o comentarios a través de tus canales de soporte habituales de Anthropic.

<h2 id="see-also">
  Ver también
</h2>

* [Configuración de modelo](/docs/es/model-config): cambiar modelos y ajustar niveles de esfuerzo
* [Gestionar costos de manera efectiva](/docs/es/costs): rastrear el uso de tokens y reducir costos
* [Configuración de línea de estado](/docs/es/statusline): mostrar información de modelo y contexto
