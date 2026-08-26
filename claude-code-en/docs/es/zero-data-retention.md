> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Retención cero de datos

> Obtenga información sobre la Retención Cero de Datos (ZDR) para Claude Code, disponible para cuentas calificadas en Claude for Enterprise, incluido el alcance, las características deshabilitadas y cómo solicitar la habilitación.

La Retención Cero de Datos (ZDR) para Claude Code está disponible para cuentas calificadas en Claude for Enterprise. Cuando ZDR está habilitado, los prompts y las respuestas del modelo generadas durante las sesiones de Claude Code se procesan en tiempo real y no se almacenan por Anthropic después de que se devuelve la respuesta, excepto cuando es necesario para cumplir con la ley o combatir el uso indebido.

<Note>
  ZDR no está incluido en el plan estándar de Claude for Enterprise y no se puede habilitar desde la configuración de administrador. Está disponible para cuentas calificadas y requiere habilitación separada por Anthropic. Si su organización requiere ZDR, [póngase en contacto con ventas](https://www.anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=zero_data_retention_request) o con su equipo de cuenta de Anthropic para confirmar la elegibilidad.
</Note>

ZDR en Claude for Enterprise proporciona a los clientes empresariales la capacidad de usar Claude Code con retención cero de datos y acceso a capacidades administrativas:

* Controles de costos por usuario
* Panel de [Analytics](/docs/es/analytics)
* [Configuración administrada por servidor](/docs/es/server-managed-settings)
* Registros de auditoría

ZDR para Claude Code en Claude for Enterprise se aplica solo a la plataforma directa de Anthropic. Para implementaciones de Claude en Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry, consulte las políticas de retención de datos de esas plataformas.

<h2 id="zdr-scope">
  Alcance de ZDR
</h2>

ZDR cubre la inferencia de Claude Code en Claude for Enterprise.

<Warning>
  ZDR se habilita por organización. Cada nueva organización requiere que ZDR sea habilitado por separado por su equipo de cuenta de Anthropic. ZDR no se aplica automáticamente a las nuevas organizaciones creadas bajo la misma cuenta. Póngase en contacto con su equipo de cuenta para habilitar ZDR para cualquier nueva organización.
</Warning>

<h3 id="what-zdr-covers">
  Qué cubre ZDR
</h3>

ZDR cubre las llamadas de inferencia del modelo realizadas a través de Claude Code en Claude for Enterprise. Cuando utiliza Claude Code en su terminal, los prompts que envía y las respuestas que genera Claude no se retienen por Anthropic. Esto se aplica a todos los modelos disponibles para organizaciones ZDR. Algunos modelos requieren retención de datos y no están disponibles bajo ZDR; consulte [Disponibilidad de modelos bajo ZDR](#model-availability-under-zdr).

<h3 id="what-zdr-does-not-cover">
  Qué no cubre ZDR
</h3>

ZDR no se extiende a lo siguiente, incluso para organizaciones con ZDR habilitado. Estas características siguen [políticas estándar de retención de datos](/docs/es/data-usage#data-retention):

| Característica                 | Detalles                                                                                                                                                                                                                                                                                         |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Chat en claude.ai              | Las conversaciones de chat a través de la interfaz web de Claude for Enterprise no están cubiertas por ZDR.                                                                                                                                                                                      |
| Cowork                         | Las sesiones de Cowork no están cubiertas por ZDR.                                                                                                                                                                                                                                               |
| Claude Code Analytics          | No almacena prompts o respuestas del modelo, pero recopila metadatos de productividad como correos electrónicos de cuenta y estadísticas de uso. Las métricas de contribución no están disponibles para organizaciones ZDR; el [panel de analytics](/docs/es/analytics) muestra solo métricas de uso. |
| Gestión de usuarios y asientos | Los datos administrativos como correos electrónicos de cuenta y asignaciones de asientos se retienen bajo políticas estándar.                                                                                                                                                                    |
| Integraciones de terceros      | Los datos procesados por herramientas de terceros, MCP servers u otras integraciones externas no están cubiertos por ZDR. Revise las prácticas de manejo de datos de esos servicios de forma independiente.                                                                                      |

<h2 id="features-disabled-under-zdr">
  Características deshabilitadas bajo ZDR
</h2>

Cuando ZDR está habilitado para una organización de Claude Code en Claude for Enterprise, ciertas características que requieren almacenar prompts o completaciones se deshabilitan automáticamente a nivel de backend:

| Característica                                                             | Razón                                                                                                                  |
| -------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| [Claude Code en la Web](/docs/es/claude-code-on-the-web)                        | Requiere almacenamiento del lado del servidor del historial de conversaciones.                                         |
| [Sesiones remotas](/docs/es/desktop#cloud-sessions) desde la aplicación Desktop | Requiere datos de sesión persistentes que incluyen prompts y completaciones.                                           |
| [Artefactos](/docs/es/artifacts)                                                | Requiere almacenar contenido de página publicado en infraestructura operada por Anthropic.                             |
| Envío de comentarios (`/feedback`)                                         | Enviar comentarios envía datos de conversación a Anthropic.                                                            |
| [Control remoto](/docs/es/remote-control)                                       | Almacena la transcripción de la sesión en servidores de Anthropic para sincronizar la conversación entre dispositivos. |

Estas características se bloquean en el backend independientemente de la visualización del lado del cliente. Si ve una característica deshabilitada en la terminal de Claude Code durante el inicio, intentar usarla devuelve un error indicando que las políticas de la organización no permiten esa acción.

Las características futuras también pueden deshabilitarse si requieren almacenar prompts o completaciones.

<h3 id="model-availability-under-zdr">
  Disponibilidad de modelos bajo ZDR
</h3>

Claude Fable 5 no está disponible para organizaciones con retención de datos cero habilitada. Esta clase de modelo [requiere retención de datos](https://platform.claude.com/docs/en/manage-claude/api-and-data-retention#model-specific-data-retention-requirements), por lo que las solicitudes de organizaciones ZDR no pueden ser servidas por ella. El modelo está ausente del selector `/model` para organizaciones ZDR o se muestra como deshabilitado con un aviso de que se requiere deshabilitar ZDR, y el servidor rechaza las solicitudes para él independientemente de la configuración del cliente.

Otros modelos permanecen disponibles bajo ZDR. Fable 5 no es el modelo predeterminado, y el alias `best`, que se resuelve a Fable 5 donde está disponible, se resuelve a Opus para organizaciones donde no lo está, incluidas las organizaciones ZDR.

<h2 id="data-retention-for-policy-violations">
  Retención de datos para violaciones de políticas
</h2>

Incluso con ZDR habilitado, Anthropic puede retener datos cuando sea requerido por ley o para abordar violaciones de la Política de Uso. Si una sesión se marca por una violación de política, Anthropic puede retener las entradas y salidas asociadas hasta 2 años, consistente con la política estándar de ZDR de Anthropic.

<h2 id="request-zdr">
  Solicitar ZDR
</h2>

Para solicitar ZDR para Claude Code en Claude for Enterprise, [póngase en contacto con ventas](https://www.anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=zero_data_retention_request) o su equipo de cuenta de Anthropic. Su equipo de cuenta presentará la solicitud internamente, y Anthropic revisará y habilitará ZDR en su organización después de confirmar la elegibilidad. Todas las acciones de habilitación se registran en auditoría.

Si actualmente está utilizando ZDR para Claude Code a través de claves API de pago por uso, puede hacer la transición a Claude for Enterprise para obtener acceso a características administrativas mientras mantiene ZDR para Claude Code. Póngase en contacto con su equipo de cuenta para coordinar la migración.
