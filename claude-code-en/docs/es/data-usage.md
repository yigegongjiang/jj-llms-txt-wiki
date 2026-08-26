> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Uso de datos

> Conozca las políticas de uso de datos de Anthropic para Claude

<h2 id="data-policies">
  Políticas de datos
</h2>

<h3 id="data-training-policy">
  Política de entrenamiento de datos
</h3>

**Usuarios de consumidor (planes Free, Pro y Max)**:
Le damos la opción de permitir que sus datos se utilicen para mejorar futuros modelos de Claude. Entrenaremos nuevos modelos utilizando datos de cuentas Free, Pro y Max cuando esta configuración esté activada (incluso cuando utiliza Claude Code desde estas cuentas).

**Usuarios comerciales**: (planes Team y Enterprise, API, plataformas de terceros y Claude Gov) mantienen políticas existentes: Anthropic no entrena modelos generativos utilizando código o indicaciones enviados a Claude Code bajo términos comerciales, a menos que el cliente haya elegido proporcionarnos sus datos para mejorar el modelo (por ejemplo, el [Development Partner Program](https://support.claude.com/es/articles/11174108-about-the-development-partner-program)).

<h3 id="development-partner-program">
  Development Partner Program
</h3>

Si opta explícitamente por métodos para proporcionarnos materiales para entrenar, como a través del [Development Partner Program](https://support.claude.com/es/articles/11174108-about-the-development-partner-program), podemos utilizar esos materiales proporcionados para entrenar nuestros modelos. Un administrador de la organización puede optar explícitamente por el Development Partner Program para su organización. Tenga en cuenta que este programa está disponible solo para API de primera parte de Anthropic, y no para usuarios de Amazon Bedrock o Google Cloud's Agent Platform.

<h3 id="feedback-using-the-/feedback-command">
  Comentarios usando el comando `/feedback`
</h3>

Si elige enviarnos comentarios sobre Claude Code usando el comando `/feedback`, podemos utilizar sus comentarios para mejorar nuestros productos y servicios. Las transcripciones compartidas a través de `/feedback` se retienen durante 5 años.

<h3 id="session-quality-surveys">
  Encuestas de calidad de sesión
</h3>

Cuando ve el mensaje "¿Cómo está funcionando Claude en esta sesión?" en Claude Code, responder a esta encuesta, incluyendo seleccionar "Descartar", registra solo su calificación. No recopilamos ni almacenamos transcripciones de conversación, entradas, salidas u otros datos de sesión como parte de la solicitud de calificación en sí. A diferencia de los comentarios de pulgar hacia arriba/abajo o los informes `/feedback`, esta encuesta de calidad de sesión es una métrica simple de satisfacción del producto.

Después de la solicitud de calificación, puede ver una pregunta de seguimiento separada que pregunta "¿Puede Anthropic ver su transcripción de sesión para ayudarnos a mejorar Claude Code?". Este es un segundo paso opcional distinto de la calificación:

* **Sí**: carga su transcripción de conversación, cualquier transcripción de subagente y el archivo de registro de sesión sin procesar del disco a Anthropic. Los patrones de clave API y token conocidos se redactan antes de la carga. El código fuente, el contenido del archivo y otro contenido de conversación se cargan tal cual. Las transcripciones compartidas se retienen hasta 6 meses. En sesiones de Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry y [Claude apps gateway](/docs/es/claude-apps-gateway) con sesión iniciada, Sí escribe la misma carga en un archivo local bajo `~/.claude/feedback-bundles/` en lugar de cargar; nada sale de su máquina hasta que reenvíe ese archivo.
* **No**: rechaza sin enviar nada
* **No preguntar de nuevo**: rechaza y evita que este seguimiento aparezca en futuras sesiones

Nada se carga a menos que seleccione explícitamente **Sí**. Las organizaciones con [zero data retention](/docs/es/zero-data-retention), o donde los comentarios del producto están deshabilitados por política de la organización, o donde `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` está configurado, nunca ven este seguimiento. Sus respuestas a esta encuesta, incluyendo transcripciones de sesión enviadas después de la solicitud de calificación, no afectan sus preferencias de entrenamiento de datos y no se pueden utilizar para entrenar nuestros modelos de IA.

Para desactivar estas encuestas, establezca `CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1`. La encuesta también se desactiva cuando se establece `DISABLE_TELEMETRY`, `DO_NOT_TRACK`, o `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`. Las organizaciones que bloquean tráfico no esencial pero capturan respuestas de encuestas a través de su propio [OpenTelemetry collector](/docs/es/monitoring-usage) pueden optar por volver a activar la encuesta estableciendo `CLAUDE_CODE_ENABLE_FEEDBACK_SURVEY_FOR_OTEL=1`. La encuesta luego registra calificaciones solo en el recopilador configurado. El seguimiento de compartir transcripción y todo el otro tráfico de comentarios vinculado a Anthropic permanecen deshabilitados. Para controlar la frecuencia en lugar de desactivar, establezca [`feedbackSurveyRate`](/docs/es/settings#available-settings) en su archivo de configuración a una probabilidad entre `0` y `1`.

<h3 id="data-retention">
  Retención de datos
</h3>

Anthropic retiene datos de Claude Code según su tipo de cuenta y preferencias.

**Usuarios de consumidor (planes Free, Pro y Max)**:

* Usuarios que permiten el uso de datos para mejorar el modelo: período de retención de 5 años para apoyar el desarrollo del modelo y mejoras de seguridad
* Usuarios que no permiten el uso de datos para mejorar el modelo: período de retención de 30 días
* La configuración de privacidad se puede cambiar en cualquier momento en [claude.ai/settings/data-privacy-controls](https://claude.ai/settings/data-privacy-controls).

**Usuarios comerciales (Team, Enterprise y API)**:

* Estándar: período de retención de 30 días
* [Zero data retention](/docs/es/zero-data-retention): disponible para Claude Code en Claude for Enterprise. ZDR no está incluido en el plan Enterprise estándar; se habilita por organización después de que su equipo de cuenta confirme la elegibilidad
* Almacenamiento en caché local: los clientes de Claude Code almacenan transcripciones de sesión localmente en texto sin formato bajo `~/.claude/projects/` durante 30 días de forma predeterminada para permitir la reanudación de sesiones. Ajuste el período con `cleanupPeriodDays`. Consulte [application data](/docs/es/claude-directory#application-data) para ver qué se almacena y cómo borrarlo.

Puede eliminar sesiones individuales de Claude Code en la web en cualquier momento. Eliminar una sesión elimina permanentemente los datos de eventos de la sesión. Para obtener instrucciones sobre cómo eliminar sesiones, consulte [Delete sessions](/docs/es/claude-code-on-the-web#delete-sessions).

Obtenga más información sobre las prácticas de retención de datos en nuestro [Privacy Center](https://privacy.anthropic.com/).

Para obtener todos los detalles, consulte nuestros [Commercial Terms of Service](https://www.anthropic.com/legal/commercial-terms) (para usuarios de Team, Enterprise y API) o [Consumer Terms](https://www.anthropic.com/legal/consumer-terms) (para usuarios de Free, Pro y Max) y [Privacy Policy](https://www.anthropic.com/legal/privacy).

<h2 id="data-access">
  Acceso a datos
</h2>

Para todos los usuarios de primera parte, puede obtener más información sobre qué datos se registran para [Claude Code local](#local-claude-code-data-flow-and-dependencies) y [Claude Code remoto](#cloud-execution-data-flow-and-dependencies). Las sesiones de [Remote Control](/docs/es/remote-control) siguen el flujo de datos local ya que toda la ejecución ocurre en su máquina; mientras está conectado, la transcripción de la sesión también se almacena en los servidores de Anthropic para sincronizar la conversación entre dispositivos, como se describe en [Conexión y seguridad](/docs/es/remote-control#connection-and-security). Tenga en cuenta que para Claude Code remoto, Claude accede al repositorio donde inicia su sesión de Claude Code. Claude no accede a repositorios que ha conectado pero en los que no ha iniciado una sesión.

<h2 id="local-claude-code-data-flow-and-dependencies">
  Local Claude Code: Flujo de datos y dependencias
</h2>

El diagrama a continuación muestra cómo Claude Code se conecta a servicios externos durante la instalación y operación normal. Las líneas sólidas indican conexiones requeridas, mientras que las líneas punteadas representan flujos de datos opcionales o iniciados por el usuario.

<img src="https://mintcdn.com/claude-code/YR4DRZyI3CdsXkiT/images/claude-code-data-flow.svg?fit=max&auto=format&n=YR4DRZyI3CdsXkiT&q=85&s=2846ea92cfc2297b8620c31c82b482ad" alt="Diagrama que muestra las conexiones externas de Claude Code: instalar/actualizar se conecta al servidor de distribución, y las solicitudes del usuario se conectan a la consola de autenticación y API pública de Anthropic, con flujos de telemetría opcionales que transportan métricas e informes de errores a Anthropic y servicios de terceros. Los comentarios enviados con /feedback van a Google Cloud Storage y opcionalmente crean un problema de GitHub" width="720" height="520" data-path="images/claude-code-data-flow.svg" />

Claude Code se ejecuta localmente. Para interactuar con el LLM, Claude Code envía datos a través de la red. Estos datos incluyen todos los indicadores del usuario y salidas del modelo, cifrados en tránsito a través de TLS 1.2+. Claude Code es compatible con la mayoría de VPN y proxies LLM populares.

El cifrado en reposo depende de su proveedor de modelo:

| Proveedor                     | Cifrado en reposo                                                                                                                                          |
| ----------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Anthropic API                 | Cifrado de disco a nivel de infraestructura (AES-256). Habilite [Zero Data Retention](/docs/es/zero-data-retention) para no persistencia del lado del servidor. |
| Amazon Bedrock                | AES-256 con claves administradas por AWS. Claves administradas por el cliente disponibles a través de AWS KMS.                                             |
| Google Cloud's Agent Platform | Claves de cifrado administradas por Google. CMEK disponible.                                                                                               |
| Microsoft Foundry             | Las solicitudes se enrutan a la infraestructura de Anthropic con cifrado de disco AES-256.                                                                 |

Claude Code se construye sobre las API de Anthropic. Para obtener detalles sobre los controles de seguridad de la API, incluyendo procedimientos de registro de API, consulte los artefactos de cumplimiento en el [Anthropic Trust Center](https://trust.anthropic.com).

<h3 id="cloud-execution-data-flow-and-dependencies">
  Cloud execution: Flujo de datos y dependencias
</h3>

Cuando se utiliza [Claude Code en la web](/docs/es/claude-code-on-the-web), las sesiones se ejecutan en máquinas virtuales administradas por Anthropic en lugar de localmente. En entornos en la nube:

* **Almacenamiento de código y datos:** Su repositorio se clona en una VM aislada. El código y los datos de sesión están sujetos a las políticas de retención y uso para su tipo de cuenta (consulte la sección Retención de datos anterior)
* **Credenciales:** La autenticación de GitHub se maneja a través de un proxy seguro; sus credenciales de GitHub nunca ingresan al sandbox
* **Tráfico de red:** Todo el tráfico saliente pasa a través de un proxy de seguridad para registro de auditoría y prevención de abuso
* **Datos de sesión:** Los indicadores, cambios de código y salidas siguen las mismas políticas de datos que el uso local de Claude Code

Para obtener detalles de seguridad sobre la ejecución en la nube, consulte [Security](/docs/es/security#cloud-execution-security).

<h2 id="telemetry-services">
  Servicios de telemetría
</h2>

Claude Code envía dos tipos de telemetría operativa: métricas de uso e informes de errores. Puede desactivar cada una individualmente con las variables de entorno que se indican a continuación, o desactivar todo el tráfico no esencial de una vez configurando `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`.

**Métricas**: latencia, confiabilidad y patrones de uso, enviados a Anthropic y a infraestructura de registro de terceros a través de TLS. Las métricas nunca incluyen su código, indicaciones o rutas de archivo. Configure `DISABLE_TELEMETRY=1` para optar por no participar.

**Informes de errores**: mensajes de error y seguimientos de pila de los internos propios de Claude Code, enviados a un servicio de seguimiento de errores de terceros a través de TLS. Claude Code redacta patrones conocidos de secretos, rutas de archivo, direcciones de correo electrónico y otra información personal antes de que nada salga de su máquina. Configure `DISABLE_ERROR_REPORTING=1` para optar por no participar.

El informe de errores está activado solo cuando se aplican todos estos:

* inicia sesión con una suscripción de Claude Pro o Max
* está ejecutando Claude Code v2.1.198 o posterior
* se está conectando directamente a la API de Claude
* su organización no tiene un acuerdo de retención cero de datos o HIPAA

Cuando usted ejecuta el comando `/feedback`, se envía una copia de su historial de conversación completo incluyendo código a Anthropic. Antes de enviar, usted elige cuánto historial incluir: solo la sesión actual, que es la predeterminada, u también otras sesiones del mismo proyecto durante los últimos 24 horas o 7 días. Los datos se cifran en tránsito mediante TLS y se almacenan en Google Cloud Storage, que cifra los datos almacenados en reposo de forma predeterminada. Opcionalmente, se crea un problema de GitHub en el repositorio público. Para optar por no participar, establezca la variable de entorno `DISABLE_FEEDBACK_COMMAND` a `1`.

Cuando usted utiliza un proveedor de terceros como Amazon Bedrock o Google Cloud's Agent Platform, o no tiene credenciales de Anthropic configuradas, `/feedback` escribe el informe en un archivo local bajo `~/.claude/feedback-bundles/` en lugar de enviarlo a Anthropic. Los patrones conocidos de clave API y token se redactan antes de que se escriba el archivo. Nada sale de su máquina hasta que usted envíe ese archivo a su representante de cuenta de Anthropic o lo adjunte a una solicitud de soporte.

<h2 id="default-behaviors-by-api-provider">
  Comportamientos predeterminados por proveedor de API
</h2>

De forma predeterminada, los informes de errores, la telemetría y los informes de errores se desactivan cuando se utiliza Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry o Claude Platform en AWS. Las encuestas de calidad de sesión y la verificación de seguridad del dominio WebFetch son excepciones y se ejecutan independientemente del proveedor. En una sesión de [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) con sesión iniciada, los análisis de uso, los informes de errores y las calificaciones de encuestas a Anthropic se desactivan mediante la credencial de la puerta de enlace en sí, sin ninguna configuración para volver a habilitarlos. Puede optar por no participar en todo el tráfico no esencial, incluyendo encuestas, a la vez estableciendo `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`. Esta variable no afecta la verificación de WebFetch, que tiene su propio opt-out. Aquí están los comportamientos predeterminados completos:

| Servicio                                           | Claude API                                                                                                                             | Google Cloud's Agent Platform API                                                                                 | Amazon Bedrock API                                                                                                | Microsoft Foundry API                                                                                             | Claude Platform en AWS                                                                                            |
| -------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| **Métricas**                                       | Activado de forma predeterminada.<br />`DISABLE_TELEMETRY=1` para desactivar.                                                          | Desactivado de forma predeterminada.<br />`CLAUDE_CODE_USE_VERTEX` debe ser 1.                                    | Desactivado de forma predeterminada.<br />`CLAUDE_CODE_USE_BEDROCK` debe ser 1.                                   | Desactivado de forma predeterminada.<br />`CLAUDE_CODE_USE_FOUNDRY` debe ser 1.                                   | Desactivado de forma predeterminada.<br />`CLAUDE_CODE_USE_ANTHROPIC_AWS` debe ser 1.                             |
| **Informes de errores**                            | Activado para inicios de sesión Pro y Max en v2.1.198+, de lo contrario desactivado.<br />`DISABLE_ERROR_REPORTING=1` para desactivar. | Desactivado de forma predeterminada.<br />`CLAUDE_CODE_USE_VERTEX` debe ser 1.                                    | Desactivado de forma predeterminada.<br />`CLAUDE_CODE_USE_BEDROCK` debe ser 1.                                   | Desactivado de forma predeterminada.<br />`CLAUDE_CODE_USE_FOUNDRY` debe ser 1.                                   | Desactivado de forma predeterminada.<br />`CLAUDE_CODE_USE_ANTHROPIC_AWS` debe ser 1.                             |
| **Claude API (informes `/feedback`)**              | Activado de forma predeterminada.<br />`DISABLE_FEEDBACK_COMMAND=1` para desactivar.                                                   | Desactivado de forma predeterminada.<br />`CLAUDE_CODE_USE_VERTEX` debe ser 1.                                    | Desactivado de forma predeterminada.<br />`CLAUDE_CODE_USE_BEDROCK` debe ser 1.                                   | Desactivado de forma predeterminada.<br />`CLAUDE_CODE_USE_FOUNDRY` debe ser 1.                                   | Desactivado de forma predeterminada.<br />`CLAUDE_CODE_USE_ANTHROPIC_AWS` debe ser 1.                             |
| **Encuestas de calidad de sesión**                 | Activado de forma predeterminada.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` para desactivar.                                        | Activado de forma predeterminada.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` para desactivar.                   | Activado de forma predeterminada.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` para desactivar.                   | Activado de forma predeterminada.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` para desactivar.                   | Activado de forma predeterminada.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` para desactivar.                   |
| **Verificación de seguridad del dominio WebFetch** | Activado de forma predeterminada.<br />`skipWebFetchPreflight: true` en [settings](/docs/es/settings) para desactivar.                      | Activado de forma predeterminada.<br />`skipWebFetchPreflight: true` en [settings](/docs/es/settings) para desactivar. | Activado de forma predeterminada.<br />`skipWebFetchPreflight: true` en [settings](/docs/es/settings) para desactivar. | Activado de forma predeterminada.<br />`skipWebFetchPreflight: true` en [settings](/docs/es/settings) para desactivar. | Activado de forma predeterminada.<br />`skipWebFetchPreflight: true` en [settings](/docs/es/settings) para desactivar. |

Todas las variables de entorno se pueden verificar en `settings.json` (consulte [referencia de configuración](/docs/es/settings)).

A partir de v2.1.126, cuando una plataforma host establece `CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`, las métricas se activan de forma predeterminada para Google Cloud's Agent Platform, Amazon Bedrock y Microsoft Foundry, y siguen el opt-out estándar de `DISABLE_TELEMETRY`. Los informes de errores y los informes `/feedback` permanecen desactivados de forma predeterminada en esos proveedores.

<h3 id="webfetch-domain-safety-check">
  Verificación de seguridad del dominio WebFetch
</h3>

Antes de obtener una URL, la herramienta WebFetch envía el nombre de host solicitado a `api.anthropic.com` para verificarlo contra una lista de bloqueo de seguridad mantenida por Anthropic. Solo se envía el nombre de host, no la URL completa, la ruta o el contenido de la página. Los resultados se almacenan en caché por nombre de host durante cinco minutos.

Esta verificación se ejecuta independientemente de qué proveedor de modelo utilice y no se ve afectada por `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`. Si su red bloquea `api.anthropic.com`, las solicitudes de WebFetch fallan hasta que permita el dominio o establezca `skipWebFetchPreflight: true` en [settings](/docs/es/settings). Desactivar la verificación significa que WebFetch intenta recuperar cualquier URL sin consultar la lista de bloqueo, así que combínelo con [reglas de permisos de `WebFetch`](/docs/es/permissions#webfetch) si necesita restringir qué dominios puede alcanzar Claude.
