> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Acelera las respuestas con el modo rápido

> Obtén respuestas más rápidas de Opus en Claude Code al activar el modo rápido.

<Note>
  El modo rápido está en [vista previa de investigación](#research-preview). La función, los precios y la disponibilidad pueden cambiar según los comentarios.
</Note>

El modo rápido es una configuración de alta velocidad para Claude Opus, haciendo que el modelo sea hasta 2.5x más rápido a un costo más alto por token. Actívalo con `/fast` cuando necesites velocidad para trabajo interactivo como iteración rápida o depuración en vivo, y desactívalo cuando el costo sea más importante que la latencia.

El modo rápido no es un modelo diferente. Utiliza Claude Opus con una configuración de API diferente que prioriza la velocidad sobre la eficiencia de costos. Obtienes la misma calidad y capacidades con respuestas más rápidas. El modo rápido es compatible con Opus 4.8 y Opus 4.7. No está disponible en Sonnet, Haiku u otros modelos.

<Warning>
  El modo rápido para Opus 4.7 está deprecado a partir del 25 de junio de 2026 y se eliminará el 24 de julio de 2026. Después de la eliminación, las solicitudes de modo rápido en Opus 4.7 devuelven un error y no vuelven a Opus 4.7 estándar. Migra a Opus 4.8 para mantener la aceleración.
</Warning>

Lo que debes saber:

* Usa `/fast` para activar o desactivar el modo rápido en Claude Code CLI. El modo rápido no es compatible con la extensión VS Code.
* Los precios del modo rápido por MTok de entrada/salida son \$10/\$50 en Opus 4.8 y \$30/\$150 en Opus 4.7.
* Disponible para todos los usuarios de Claude Code en planes de suscripción (Pro/Max/Team/Enterprise) y Claude Console.
* Para los usuarios de Claude Code en planes de suscripción (Pro/Max/Team/Enterprise), el modo rápido está disponible solo a través de créditos de uso y no está incluido en los límites de velocidad de la suscripción.

<h2 id="toggle-fast-mode">
  Activar el modo rápido
</h2>

Activa el modo rápido de cualquiera de estas formas:

* Escribe `/fast` y presiona Tab para activar o desactivar
* Establece `"fastMode": true` en tu [archivo de configuración de usuario](/docs/es/settings)

De forma predeterminada, el modo rápido que activas en una sesión interactiva persiste entre sesiones. En [modo no interactivo](/docs/es/headless), con la bandera `-p`, `/fast` funciona solo en una sesión iniciada con el modo rápido en su valor [`--settings`](/docs/es/cli-reference#cli-flags), por ejemplo `claude -p --settings '{"fastMode": true}'`; el cambio se aplica solo a esa sesión y no se guarda como tu valor predeterminado, y en cualquier otra sesión no interactiva el comando reporta que el modo rápido no está disponible. Puedes configurar el modo rápido para que se reinicie cada sesión. Consulta [opción de participación por sesión](#require-per-session-opt-in) para obtener más detalles.

Para la mejor eficiencia de costos, habilita el modo rápido al inicio de una sesión en lugar de cambiar a mitad de la conversación. Consulta [comprender la compensación de costos](#understand-the-cost-tradeoff) para obtener más detalles.

Cuando habilitas el modo rápido:

* Si estás en un modelo diferente, Claude Code cambia automáticamente a Opus
* Verás un mensaje de confirmación: "Fast mode ON"
* Un pequeño icono `↯` aparece junto al prompt mientras el modo rápido está activo
* Ejecuta `/fast` nuevamente en cualquier momento para verificar si el modo rápido está activado o desactivado

Cuando desactivas el modo rápido con `/fast` nuevamente, permaneces en Opus. El modelo no revierte a tu modelo anterior. Para cambiar a un modelo diferente, usa `/model`.

Cambiar a un modelo que no admite el modo rápido desactiva el modo rápido. Cambiar de nuevo a un modelo Opus compatible lo activa nuevamente cuando tu preferencia de modo rápido guardada está activada, la misma preferencia con la que una nueva sesión comienza de forma predeterminada. Con [opción de participación por sesión](#require-per-session-opt-in) configurada, cambiar de nuevo no activa el modo rápido nuevamente; ejecuta `/fast` para reactivarlo. El modo rápido nunca se activa para una sesión cuya preferencia guardada está desactivada, y el icono `↯` y la confirmación `Fast mode ON` aparecen siempre que se activa. Antes de v2.1.208, el modo rápido permanecía desactivado después de que cambiaras de nuevo hasta que ejecutaras `/fast` nuevamente.

Opus 4.8 es el valor predeterminado del modo rápido en Claude Code v2.1.154 y posterior. En v2.1.142 a v2.1.153, el modo rápido utiliza Opus 4.7 de forma predeterminada.

<h2 id="understand-the-cost-tradeoff">
  Comprender la compensación de costos
</h2>

El modo rápido tiene precios por token más altos que el Opus estándar, con el multiplicador variando según el modelo:

| Modelo   | Entrada (MTok) | Salida (MTok) |
| -------- | -------------- | ------------- |
| Opus 4.8 | \$10           | \$50          |
| Opus 4.7 | \$30           | \$150         |

Los precios del modo rápido son fijos en toda la ventana de contexto de 1M tokens. Para la tarifa estándar de Opus con la que comparar, consulte la [referencia de precios de Claude](https://platform.claude.com/docs/es/about-claude/pricing).

La primera vez que habilita el modo rápido en una conversación, paga el precio completo del token de entrada sin caché del modo rápido para todo el contexto de la conversación. Cuanto más profundo esté en una conversación, más cuesta esto, por lo que habilitar el modo rápido desde el inicio es más económico. El costo se aplica una vez por conversación, por lo que desactivar y activar el modo rápido nuevamente más tarde no lo repite. Para el mecanismo, consulte [cómo el modo rápido interactúa con el caché de indicaciones](/docs/es/prompt-caching#turning-on-fast-mode).

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

* **Solo API de Anthropic o suscripción**: el modo rápido está disponible a través de la API de Anthropic Console y para planes de suscripción de Claude usando créditos de uso. No está disponible en Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry o Claude Platform en AWS.
* **Créditos de uso activados**: su cuenta debe tener los créditos de uso activados, lo que permite facturación más allá del uso incluido en su plan. Para cuentas individuales, active esto en su [configuración de facturación de Console](https://platform.claude.com/settings/billing). Para Team y Enterprise, un administrador debe activar los créditos de uso para la organización.

<Note>
  El uso del modo rápido se extrae directamente de los créditos de uso, incluso si tiene uso restante en su plan. Esto significa que los tokens del modo rápido no cuentan contra el uso incluido en su plan y se cobran a la tarifa del modo rápido desde el primer token.
</Note>

* **Habilitación del propietario para Team y Enterprise**: el modo rápido está deshabilitado de forma predeterminada para organizaciones Team y Enterprise. Un propietario debe [habilitar explícitamente el modo rápido](#enable-fast-mode-for-your-organization) antes de que los usuarios puedan acceder a él.

<Note>
  Si el modo rápido no ha sido habilitado para su organización, el comando `/fast` mostrará "Fast mode has been disabled by your organization." Si la lista de permitidos [`availableModels`](/docs/es/model-config#restrict-model-selection) de su organización excluye el modelo Opus del modo rápido, `/fast` se rechaza con "is not in your organization's allowed models". La excepción es una sesión ya en ejecución en un modelo Opus permitido que admita modo rápido: `/fast` habilita el modo rápido en su modelo actual en lugar de cambiar de modelos.
</Note>

<h3 id="enable-fast-mode-for-your-organization">
  Habilitar el modo rápido para su organización
</h3>

Dónde habilita el modo rápido depende de qué producto usa su organización:

* **Console** (clientes de API): un administrador lo habilita en [Preferencias de Claude Code](https://platform.claude.com/claude-code/preferences)
* **Claude AI** (Team y Enterprise): un propietario lo habilita en [Admin Settings > Claude Code](https://claude.ai/admin-settings/claude-code)

Otra opción para desactivar completamente el modo rápido es establecer `CLAUDE_CODE_DISABLE_FAST_MODE=1`. Consulte [Variables de entorno](/docs/es/env-vars).

<h3 id="require-per-session-opt-in">
  Requerir opción de participación por sesión
</h3>

De forma predeterminada, el modo rápido que un usuario activa en una sesión interactiva persiste entre sesiones: permanece activado en futuras sesiones. Para cambiar esto, establezca `fastModePerSessionOptIn` en `true` en cualquier [archivo de configuración](/docs/es/settings#settings-files), lo que hace que cada sesión comience con el modo rápido desactivado y requiere que los usuarios lo habiliten explícitamente con `/fast`. Los propietarios en planes [Team](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_teams#team-&-enterprise) o [Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_enterprise) pueden implementarlo en toda la organización a través de [configuración administrada por servidor](/docs/es/server-managed-settings).

```json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

Esto es útil para controlar costos en organizaciones donde los usuarios ejecutan múltiples sesiones concurrentes. Los usuarios aún pueden habilitar el modo rápido con `/fast` cuando necesiten velocidad, pero se reinicia al inicio de cada nueva sesión. La preferencia del modo rápido del usuario aún se guarda, por lo que eliminar esta configuración restaura el comportamiento persistente predeterminado.

<h2 id="handle-rate-limits">
  Manejar límites de velocidad
</h2>

El modo rápido tiene límites de velocidad separados del Opus estándar. El modo rápido en Opus 4.8 y Opus 4.7 comparten el mismo grupo de límite de velocidad: el uso en cualquiera de ellos se extrae de los mismos límites. Cuando alcanzas el límite de velocidad del modo rápido o se agotan tus créditos de uso:

1. El modo rápido automáticamente vuelve a velocidad estándar
2. El icono `↯` se vuelve gris para indicar enfriamiento
3. Continúas trabajando a velocidad y precios estándar
4. Cuando expira el enfriamiento, el modo rápido se vuelve a habilitar automáticamente

Para desactivar el modo rápido manualmente en lugar de esperar el enfriamiento, ejecuta `/fast` nuevamente.

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
