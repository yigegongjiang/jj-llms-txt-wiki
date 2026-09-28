> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gestionar costos de manera efectiva

> Realice un seguimiento del uso de tokens, establezca límites de gasto del equipo y reduzca los costos de Claude Code con la gestión del contexto, la selección de modelos, la configuración del pensamiento extendido y los hooks de preprocesamiento.

Claude Code cobra por consumo de tokens de API. Para precios de planes de suscripción (Pro, Max, Team, Enterprise), consulte [claude.com/pricing](https://claude.com/pricing). Los costos por desarrollador varían ampliamente según la selección del modelo, el tamaño de la base de código y los patrones de uso, como ejecutar múltiples instancias o automatización.

En implementaciones empresariales, el costo promedio es de alrededor de \$13 por desarrollador por día activo y \$150-250 por desarrollador por mes, con costos que se mantienen por debajo de \$30 por día activo para el 90% de los usuarios. Para estimar el gasto de su equipo, comience con un pequeño grupo piloto y use las herramientas de seguimiento a continuación para establecer una línea base antes de un despliegue más amplio.

Esta página cubre cómo [realizar un seguimiento de sus costos](#track-your-costs), [gestionar costos para su organización](#manage-costs-for-your-organization) y [reducir el uso de tokens](#reduce-token-usage).

<h2 id="track-your-costs">
  Realice un seguimiento de sus costos
</h2>

<h3 id="using-the-/usage-command">
  Uso del comando `/usage`
</h3>

<Note>
  El bloque Session en `/usage` muestra el uso de tokens de API y está destinado a usuarios de API. Los suscriptores de Claude Max y Pro tienen el uso incluido en su suscripción, por lo que la cifra de costo de sesión no es relevante para fines de facturación. Los suscriptores ven barras de uso del plan, estadísticas de actividad y un desglose de uso en la misma pantalla.
</Note>

El bloque Session en la parte superior de `/usage` muestra estadísticas detalladas de uso de tokens para su sesión actual. Claude Code calcula la cifra en dólares localmente a partir de conteos de tokens al precio de lista, a menos que una tabla [`modelPricing`](/docs/es/settings-reference#modelpricing) esté en vigor. Un administrador establece una en la configuración administrada de su organización para que la cifra utilice sus tasas contratadas, y mientras una tabla esté en vigor, la línea `Total cost` lleva la nota `at your organization's configured rates`. La cifra es una estimación, por lo que para facturación autorizada consulte la página de Uso en la [Consola de Claude](https://platform.claude.com/usage).

```text theme={null}
Total cost:            $0.55
Total duration (API):  6m 20s
Total duration (wall): 6h 33m 10s
Total code changes:    0 lines added, 0 lines removed
Usage by model:
   claude-sonnet-4-6:  1.2k input, 5.3k output, 940.0k cache read, 50.0k cache write ($0.55)
```

Estos totales se restablecen cuando `/clear` inicia una nueva sesión, por lo que el costo total de la siguiente sesión comienza en \$0. Antes de v2.1.211, continuaban acumulándose en `/clear` durante la vida útil del proceso de Claude Code.

Para una respuesta de la API de Claude facturada a la [tasa de residencia de datos](https://platform.claude.com/docs/en/about-claude/pricing#data-residency-pricing) de 1.1×, Claude Code multiplica el precio de lista de los tokens de esa respuesta por 1.1 en la cifra de costo de sesión. El mismo total aparece en el [campo de costo de la línea de estado](/docs/es/statusline#cost-and-duration-tracking), y la cifra multiplicada también cuenta hacia [`--max-budget-usd`](/docs/es/cli-reference#cli-flags). Antes de v2.1.239, Claude Code no aplicaba el 1.1× a esas respuestas, por lo que la cifra de costo de sesión era menor que la factura.

<h4 id="prompt-cache-statistics">
  Estadísticas de caché de indicaciones
</h4>

Después de la primera respuesta de API de la conversación principal, Claude Code también agrega una línea `Prompt cache (main)` al bloque Session, resumiendo el uso de [caché de indicaciones](/docs/es/prompt-caching) de la sesión: el recuento de solicitudes, la proporción de tokens de entrada servidos desde caché, fallos de caché, y si el caché está caliente en este momento. Requiere Claude Code v2.1.251 o posterior.

```text theme={null}
Prompt cache (main):   14 requests · 91% of input tokens from cache · 2 misses (last 6m 10s ago, 310.2k tokens re-cached) · 1 expected rebuild (compaction or tool-result clearing) · warm (1h TTL, last activity 40s ago)
```

Los fallos, reconstrucciones esperadas, y partes cálidas o frías de la línea significan lo siguiente:

* **Misses (Fallos)**: solicitudes que reprocesaron contenido que el caché ya contenía, con la hora del último fallo y cuántos tokens esas solicitudes escribieron de nuevo en el caché. Claude Code cuenta una solicitud como un fallo cuando la solicitud reprocesó más del 5% y al menos 2,000 tokens de lo que podría haber leído desde caché. [Las acciones que invalidan el caché](/docs/es/prompt-caching#actions-that-invalidate-the-cache) enumeran las causas habituales. Cuando Claude Code puede identificar una causa probable para el último fallo, la línea también la nombra, por ejemplo `likely cause: tool definitions changed`. El texto de causa probable requiere Claude Code v2.1.260 o posterior.
* **Expected rebuilds (Reconstrucciones esperadas)**: cuando Claude Code ha reescrito la conversación, por [compactación](/docs/es/prompt-caching#compacting-the-conversation) o por borrar resultados de herramientas antiguos del contexto, cuenta el mismo tipo de fallo como una reconstrucción esperada en su lugar. Esta parte aparece solo después de que ha ocurrido al menos una reconstrucción esperada.
* **Warm or cold (Cálido o frío)**: si el prefijo en caché aún está dentro de su [vida útil de caché](/docs/es/prompt-caching#cache-lifetime), con el TTL en vigor. Cuando el caché está frío, la línea muestra cuánto tiempo la sesión ha estado inactiva. Cuando ninguna respuesta ha reportado tokens de caché, la línea termina con `no prompt caching reported by the API` en su lugar.

Los conteos provienen de los campos de token de caché en las respuestas de la API, por lo que la línea funciona en cada proveedor y puerta de enlace. Cubre solo la conversación principal, no subagentes. `/clear` lo restablece junto con el resto del bloque Session.

Los scripts de línea de estado pueden leer los mismos números desde el [objeto `prompt_cache`](/docs/es/statusline#prompt-cache-fields).

<h4 id="plan-usage-breakdown">
  Desglose de uso del plan
</h4>

En un plan Pro, Max, Team o Enterprise, `/usage` también muestra un desglose de lo que cuenta contra los límites de su plan:

* **Attribution (Atribución)**: uso reciente atribuido a skills, subagentes, plugins y servidores MCP individuales, cada uno mostrado como un porcentaje del total. La participación de un servidor MCP cuenta solo las solicitudes que consumieron uno de sus resultados de herramientas. Antes de v2.1.222, después de una llamada a un servidor MCP, Claude Code atribuía cada solicitud posterior a ese servidor, sobrestimando su participación.
* **Behavior flags (Indicadores de comportamiento)**: comportamientos como contexto largo o fallos de caché, marcados cuando uno representa el 10% o más del uso reciente.
* **Loops (Bucles)**: una fila para cada una de las tareas [`/loop` u otras tareas programadas](/docs/es/scheduled-tasks) más pesadas que se ejecutaron recientemente, ordenadas por tokens totales, con un recuento del resto. Claude Code informa con qué frecuencia se ejecuta cada tarea, cuántas veces se ejecutó, sus tokens totales y por ejecución, y cuándo se ejecutó por última vez. Claude Code clave una fila por el indicador de la tarea, por lo que un bucle que detiene y recrea permanece como una fila. Requiere Claude Code v2.1.242 o posterior.

Presione `d` o `w` para cambiar entre las últimas 24 horas y los últimos 7 días. Las cifras son aproximadas y se calculan a partir del historial de sesión local en esta máquina, por lo que el uso de otros dispositivos o claude.ai no se incluye.

En la [extensión de VS Code](/docs/es/vs-code#check-account-and-usage), las participaciones de atribución y los indicadores de comportamiento aparecen en el diálogo Cuenta y uso con un botón de alternancia Día y Semana, sin las filas de Loops.

<h4 id="check-your-usage-credits-spend">
  Verifique su gasto en créditos de uso
</h4>

`/usage` también muestra una fila de créditos de uso mientras [los créditos de uso](#add-usage-credits-to-your-subscription) estén activados. Lo que muestra la fila depende de su plan:

* **Pro y Max**: su gasto para el mes actual, medido contra su límite de gasto mensual cuando ha establecido uno. Cuando no ha establecido un límite, la fila muestra `Unlimited` y ninguna cifra de gasto.
* **Team y Enterprise**: su propio gasto para el mes actual, medido contra cualquier [límite que su organización estableció](#claude-for-teams-and-enterprise) que se le aplique. Un límite que cubre toda la organización no aparece en la fila. Cuando no tiene su propio límite, la fila muestra su gasto sin límite junto a él. Mientras los créditos de uso estén desactivados para usted, `/usage` no muestra ninguna fila de créditos de uso.

Cuando tiene un límite de gasto, la fila aparece tan pronto como los créditos de uso estén activados y muestra 0% hasta que gaste créditos de uso por primera vez. Antes de v2.1.236, `/usage` mostraba la fila solo en planes Pro y Max, y una fila con un límite de gasto permanecía oculta hasta que hubiera gastado algo.

<h4 id="when-the-usage-request-fails">
  Cuando la solicitud de uso falla
</h4>

Cuando la solicitud de sus límites de plan falla, la mayoría de las veces porque el punto final de uso tiene límite de velocidad, `/usage` muestra las últimas barras de uso que cargó en esta máquina dentro de los últimos 60 minutos, junto con una nota `Showing last-known usage` que indica cuánto tiempo hace que se obtuvieron esos datos. Presione `r` para reintentar; un reintento exitoso reemplaza las últimas barras conocidas con datos frescos. Sin una instantánea de los últimos 60 minutos, `/usage` informa que el punto final de uso tiene límite de velocidad y ofrece el mismo atajo de reintento. Antes de v2.1.208, una solicitud con límite de velocidad en una sesión que aún no había cargado uso siempre mostraba el error sin barras.

<h3 id="analyze-your-usage-patterns">
  Analice sus patrones de uso
</h3>

Ejecute [`/insights`](/docs/es/commands#all-commands) para un informe sobre cómo trabaja en lugar de cuántos tokens ha utilizado. Analiza sus sesiones recientes en esta máquina y escribe un informe HTML que cubre en qué trabaja, puntos de fricción como solicitudes mal entendidas o código defectuoso, y sugerencias para usar Claude Code de manera más efectiva. Una única ejecución analiza hasta 200 sesiones que no ha visto antes y omite las muy cortas. Cuando se omiten sesiones, el encabezado del informe muestra el recuento analizado con el total entre paréntesis, por ejemplo `200 sessions (412 total)`.

Claude Code escribe el informe más reciente en `~/.claude/usage-data/report.html` y guarda una copia con marca de tiempo de cada ejecución en el mismo directorio, por lo que los informes anteriores no se sobrescriben. Claude Code elimina informes en el mismo cronograma que el resto de sus datos de sesión: al iniciar, elimina archivos más antiguos que [`cleanupPeriodDays`](/docs/es/claude-directory#cleaned-up-automatically), 30 días por defecto.

Puede ejecutar `/insights` en cualquier plan y con cualquier proveedor. El análisis se ejecuta a través del mismo proveedor y cuenta que sus sesiones regulares, y los tokens cuentan contra su plan o uso de API. Las sesiones de otros dispositivos y claude.ai no se incluyen.

<h3 id="add-usage-credits-to-your-subscription">
  Agregue créditos de uso a su suscripción
</h3>

[Los créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) le permiten continuar trabajando después del límite de uso de su plan. Para administrarlos, ejecute `/usage-credits` después de iniciar sesión con su suscripción de claude.ai a través de `/login`; el comando no está disponible con autenticación de clave de API. En organizaciones Enterprise de autoservicio, pruebas de Enterprise y organizaciones Enterprise facturadas a través de AWS Marketplace, el comando requiere Claude Code v2.1.248 o posterior; las versiones anteriores lo rechazan con [`Unknown command: /usage-credits`](/docs/es/errors#unknown-command). Lo que abre depende de su rol:

| Su rol                                                 | Lo que hace `/usage-credits`                                                                                                                                                                                                                              |
| :----------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Suscriptor de Pro o Max                                | Abre [**Settings > Usage**](https://claude.ai/settings/usage) en claude.ai en el navegador. En su sección **Usage credits** puede activar o desactivar créditos de uso y verificar su saldo de crédito, el gasto de este mes y su límite de gasto mensual |
| Miembro de Team o Enterprise con acceso de facturación | Abre la configuración de uso de su organización, [**Admin settings > Usage**](https://claude.ai/admin-settings/usage), en el navegador                                                                                                                    |
| Miembro de Team o Enterprise sin acceso de facturación | Le pide que confirme, luego envía una solicitud a los administradores de su organización. Antes de v2.1.211, Claude Code enviaba la solicitud sin un paso de confirmación                                                                                 |

Para miembros de Team y Enterprise sin acceso de facturación, la confirmación aparece solo en sesiones interactivas: en modo no interactivo con la bandera `-p` y desde [Control Remoto](/docs/es/remote-control), el comando no envía ninguna solicitud y le dice que la ejecute en una sesión interactiva en su lugar.

Si ejecuta `/usage-credits` nuevamente mientras su solicitud anterior está esperando a un administrador, Claude Code le dice que ya se ha enviado una solicitud en lugar de enviar un duplicado. Después de que un administrador descarta su solicitud, ejecutar el comando nuevamente envía una nueva. Antes de v2.1.222, una solicitud descartada también bloqueaba nuevas solicitudes.

En planes Pro y Max, cuando alcanza su límite de gasto con créditos de uso aún disponibles, Claude Code le solicita que aumente o elimine el límite sin abandonar la CLI. Si el servidor rechaza el cambio, consulte [Could not update your spend limit](/docs/es/errors#could-not-update-your-spend-limit).

<h2 id="manage-costs-for-your-organization">
  Gestión de costos para su organización
</h2>

Los controles que tiene dependen de cómo su organización accede a Claude Code: un plan Claude for Teams o Enterprise, la Consola de Claude, o un proveedor de nube. En los planes Teams y Enterprise, el uso se extrae de la asignación de cada miembro. En la Consola y en proveedores de nube, el uso se factura por token a su organización. Si su organización mezcla métodos de inicio de sesión, cada desarrollador se mide según el que autenticó.

La tabla asigna cada configuración a dónde ve el gasto, dónde lo limita y cómo extrae números por usuario. En un plan individual Pro o Max no tiene organización que administrar, así que rastree su propio gasto de créditos de uso, incluido [modo rápido](/docs/es/fast-mode#see-where-fast-mode-spend-appears), en [Agregar créditos de uso a su suscripción](#add-usage-credits-to-your-subscription).

| Su configuración                                                                               | Ver gasto                                                                                                                                 | Limitar gasto                                      | Informes por usuario                                                                                                                                                                                                               |
| :--------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Claude for Teams o Enterprise](#claude-for-teams-and-enterprise)                              | [Informe de gasto en análisis de org](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) | Límites de gasto en configuración de administrador | [CSV de informe de gasto](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans); [API de Análisis Enterprise](https://platform.claude.com/docs/en/api/admin/analytics) en Enterprise |
| [Consola de Claude (API)](#claude-console)                                                     | [Página de uso de Consola](https://platform.claude.com/usage)                                                                             | Límites de gasto del espacio de trabajo            | [Panel de Consola](https://platform.claude.com/claude-code), [API de Análisis de Claude Code](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api)                                                     |
| [Amazon Bedrock, Plataforma de Agentes de Google Cloud, o Microsoft Foundry](#cloud-providers) | Su consola de facturación en la nube                                                                                                      | Controles de presupuesto de su nube                | [OpenTelemetry](/docs/es/monitoring-usage) o una [puerta de enlace LLM](/docs/es/llm-gateway)                                                                                                                                                |

[La exportación de OpenTelemetry](/docs/es/monitoring-usage) funciona en cada configuración y es la única opción que transmite métricas de tokens y costos por usuario a su propia pila de observabilidad en tiempo casi real.

<h3 id="report-spend-at-your-contracted-rates">
  Informar gasto a sus tasas contratadas
</h3>

Por defecto, Claude Code calcula cada cifra de costo que muestra a los desarrolladores al precio de lista, así que si su organización paga tasas contratadas, las cifras en `/usage`, la línea de estado y OpenTelemetry no coinciden con su factura. Para hacerlas coincidir, establezca la configuración administrada [`modelPricing`](/docs/es/settings-reference#modelpricing) a sus tasas. La configuración cambia lo que Claude Code reporta, no lo que Anthropic cobra. Requiere Claude Code v2.1.242 o posterior.

<Steps>
  <Step title="Tome las tasas de su contrato">
    Ingrese las tasas por millón de tokens de su contrato. Claude Code no las obtiene de la Consola de Claude, así que actualice la configuración cuando el contrato cambie.
  </Step>

  <Step title="Escriba la configuración">
    Establezca `multiplier` por debajo de 1 para un descuento fijo o por encima de 1 para un margen, enumere las cuatro tasas por token de cada modelo en `overrides`, o haga ambas. Un margen requiere Claude Code v2.1.271 o posterior. La [entrada `modelPricing`](/docs/es/settings-reference#modelpricing) tiene la forma y un ejemplo listo para pegar.
  </Step>

  <Step title="Implántelo a través de configuración administrada">
    Entréguela como [configuración administrada](/docs/es/managed-settings): configuración administrada por servidor, una política MDM, `managed-settings.json`, o un [asistente de política](/docs/es/managed-settings#compute-the-policy-with-a-helper-program). Claude Code ignora la clave en configuración de usuario, proyecto y local y en `--settings`.
  </Step>
</Steps>

Para confirmar que las tasas están en vigor, ejecute `/usage` en una sesión que haya [recibido la configuración administrada](/docs/es/managed-settings#read-the-source-in-%2Fstatus): el bloque de sesión de la línea `Total cost` lleva la nota `at your organization's configured rates`. Las cifras siguen siendo estimaciones, no una factura. Los precios por millón de tokens en el selector `/model` permanecen al precio de lista.

<h3 id="claude-for-teams-and-enterprise">
  Claude for Teams y Enterprise
</h3>

En los planes Claude for Teams y Enterprise, el uso de Claude Code de cada miembro se extrae de una asignación por puesto que se reinicia en una ventana de cinco horas móvil y una ventana semanal. La asignación se comparte con Claude chat y Cowork, y su tamaño depende del [nivel de puesto](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan) (Standard o Premium). Sus controles se encuentran en la consola de administrador de claude.ai, no en la Consola de Claude.

* **Ver gasto**: el [informe de gasto en análisis de org](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) muestra el gasto estimado por usuario y por modelo, con exportación CSV, actualizado diariamente. El informe cubre el gasto de créditos de uso y aparece una vez que se activan los créditos de uso. El uso dentro de la asignación de puesto no se mide en dólares.
* **Ver adopción**: el [panel de análisis](https://claude.ai/analytics/claude-code) muestra usuarios activos diarios, sesiones y métricas de contribución, con exportación CSV de datos de contribución. Consulte [rastrear el uso del equipo con análisis](/docs/es/analytics).
* **Limitar gasto**: la asignación de puesto es el techo predeterminado. Para permitir que los miembros continúen más allá, active [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) y establezca límites de gasto a nivel de organización, grupo o miembro individual.
* **Extraer números por usuario**: en el plan Enterprise, la [API de Análisis Enterprise](https://platform.claude.com/docs/en/api/admin/analytics) devuelve informes de uso y costo por usuario en todas las superficies de Claude, incluido Claude Code. Un Propietario Principal crea una clave con el alcance `read:analytics` en [claude.ai/analytics/api-keys](https://claude.ai/analytics/api-keys). En el plan Teams, exporte el [CSV de informe de gasto](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans), que enumera el uso de tokens y el gasto estimado por usuario y por modelo.

La [Guía de consumo de Claude Enterprise](https://support.claude.com/en/articles/14782391-claude-enterprise-consumption-guide) es la referencia de planificación para administradores. Explica cómo difiere el consumo en Claude chat, Claude Code y Cowork, y proporciona puntos de partida en dólares por usuario para presupuestar. Presupueste más para un puesto de codificación que para un puesto de chat: cada turno de Claude Code lleva contenidos de archivo, llamadas de herramientas y razonamiento de múltiples pasos, por lo que una sesión de depuración puede consumir más que un día de chat.

<h3 id="claude-console">
  Consola de Claude
</h3>

Las organizaciones de API administran el gasto de Claude Code a través de [espacios de trabajo](https://platform.claude.com/docs/en/build-with-claude/workspaces). Puede [establecer límites de gasto del espacio de trabajo](https://platform.claude.com/docs/en/build-with-claude/workspaces#workspace-limits) en el gasto total de Claude Code y [ver informes de costos y uso](https://platform.claude.com/docs/en/build-with-claude/workspaces#usage-and-cost-tracking) en la Consola.

<Note>
  Cuando autentica por primera vez Claude Code con su cuenta de Claude Console, se crea automáticamente un espacio de trabajo llamado "Claude Code" para usted. Este espacio de trabajo proporciona seguimiento y gestión centralizada de costos para todo el uso de Claude Code en su organización. No puede crear claves de API para este espacio de trabajo; es exclusivamente para autenticación y uso de Claude Code.

  Para organizaciones con límites de velocidad personalizados, el tráfico de Claude Code en este espacio de trabajo cuenta hacia los límites de velocidad de API generales de su organización. Puede establecer un [límite de velocidad del espacio de trabajo](https://platform.claude.com/docs/en/api/rate-limits#setting-lower-limits-for-workspaces) en la página Limits de este espacio de trabajo en la Consola de Claude para limitar la parte de Claude Code y proteger otras cargas de trabajo de producción.
</Note>

Para informes por usuario, el [panel de Consola](https://platform.claude.com/claude-code) muestra gasto y líneas aceptadas por miembro, y la [API de Análisis de Claude Code](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) devuelve las mismas métricas diarias por usuario mediante programación con una [clave de API de administrador](https://platform.claude.com/settings/admin-keys). Consulte [análisis para clientes de API](/docs/es/analytics#access-analytics-for-api-customers).

<h4 id="rate-limit-recommendations">
  Recomendaciones de límite de velocidad
</h4>

Al configurar Claude Code para equipos, considere estas recomendaciones de Tokens Por Minuto (TPM) y Solicitudes Por Minuto (RPM) por usuario según el tamaño de su organización:

| Tamaño del equipo | TPM por usuario | RPM por usuario |
| ----------------- | --------------- | --------------- |
| 1-5 usuarios      | 200k-300k       | 5-7             |
| 5-20 usuarios     | 100k-150k       | 2.5-3.5         |
| 20-50 usuarios    | 50k-75k         | 1.25-1.75       |
| 50-100 usuarios   | 25k-35k         | 0.62-0.87       |
| 100-500 usuarios  | 15k-20k         | 0.37-0.47       |
| 500+ usuarios     | 10k-15k         | 0.25-0.35       |

Por ejemplo, si tiene 200 usuarios, podría solicitar 20k TPM para cada usuario, o 4 millones de TPM totales (200\*20,000 = 4 millones).

El TPM por usuario disminuye a medida que crece el tamaño del equipo porque menos usuarios tienden a usar Claude Code simultáneamente en organizaciones más grandes. Estos límites de velocidad se aplican a nivel de organización, no por usuario individual, lo que significa que los usuarios individuales pueden consumir temporalmente más que su parte calculada cuando otros no están usando activamente el servicio.

<Note>
  Si anticipa escenarios con uso concurrente inusualmente alto (como sesiones de capacitación en vivo con grupos grandes), es posible que necesite asignaciones de TPM más altas por usuario.
</Note>

<h3 id="cloud-providers">
  Proveedores de nube
</h3>

En Amazon Bedrock, Plataforma de Agentes de Google Cloud y Microsoft Foundry, Claude Code se factura por token a su cuenta en la nube, y los controles de gasto se encuentran en la consola de facturación de su proveedor de nube. Claude Code no envía métricas desde su nube a Anthropic, por lo que los [paneles de análisis](/docs/es/analytics) y la API de Análisis de Claude Code no cubren este uso.

Para la atribución de costos por usuario, tiene tres opciones:

* **OpenTelemetry**: [exporte métricas](/docs/es/monitoring-usage) desde la máquina de cada desarrollador a su propia pila de observabilidad. Esto le proporciona conteos de tokens por usuario, costos y actividad de herramientas independientemente del proveedor.
* **Una puerta de enlace de aplicaciones Claude**: una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) autohospedada proporciona atribución de uso por usuario, métricas OTLP con conteos de tokens, y [límites de gasto por usuario](/docs/es/claude-apps-gateway-spend-limits) en estos proveedores.
* **Una puerta de enlace LLM**: enrute todo el tráfico de Claude Code a través de un proxy que rastree el gasto por clave. Varios grandes empresas informaron usar [LiteLLM](/docs/es/llm-gateway), una herramienta de código abierto que [rastrea el gasto por clave](https://docs.litellm.ai/docs/proxy/virtual_keys#tracking-spend). Este proyecto no está afiliado con Anthropic y no ha sido auditado por seguridad.

<h3 id="when-a-developer-asks-about-a-limit">
  Cuando un desarrollador pregunta sobre un límite
</h3>

Los desarrolladores generalmente llevan preguntas sobre límites a su administrador, por lo que es útil saber qué techo alcanzaron. Estas situaciones significan cosas diferentes:

* **"Ha alcanzado su límite de sesión" o "Ha alcanzado su límite semanal"**: una ventana de uso basada en puesto en un plan de suscripción, compartida en todos los modelos, por lo que el desarrollador no puede restaurar el acceso cambiando modelos con `/model`. El mensaje muestra cuándo se reinicia la ventana. Después del mensaje específico del modelo "Ha alcanzado su límite de Opus" o "Ha alcanzado su límite de Sonnet", cambiar a un modelo fuera de esa familia con `/model` sí mantiene al desarrollador trabajando. Consulte [errores de límite de uso](/docs/es/errors#youve-hit-your-session-limit). Lo que el desarrollador puede hacer mientras tanto:
  * Ejecute `/usage-credits` para solicitar uso más allá de la asignación, si tiene [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) activados.
  * En Claude Code v2.1.234 o posterior, [espere y continúe la tarea interrumpida automáticamente después del reinicio](/docs/es/interactive-mode#wait-for-a-usage-limit-to-reset); esa sección enumera cuándo Claude Code inicia la espera por su cuenta y cuándo el desarrollador la elige de `/rate-limit-options`. Para controlar en su flota si Claude Code inicia esa espera por su cuenta, establezca [`autoContinueAtUsageLimit`](/docs/es/settings-reference#autocontinueatusagelimit) en [configuración administrada](/docs/es/settings#settings-precedence).
* **"Ha alcanzado su límite de gasto individual", "límite de gasto mensual de la org", o "presupuesto compartido del equipo"**: la solicitud del desarrollador se facturaría a créditos de uso, y esos créditos han alcanzado un límite de gasto que estableció. Para permitir que el desarrollador continúe, vaya a [**Configuración de administrador > Uso**](https://claude.ai/admin-settings/usage) y aumente el límite que menciona el mensaje. Cuando el mensaje también menciona un tiempo de reinicio del plan, el desarrollador puede esperar hasta entonces. Consulte [la referencia de errores](/docs/es/errors#youve-hit-your-monthly-spend-limit) para cada variante.
* **Un mensaje de límite de gasto de una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway)**: el desarrollador pasó un límite de gasto que estableció en su puerta de enlace autohospedada, y la puerta de enlace bloquea sus solicitudes hasta que el período se reinicia o usted aumenta el límite. Consulte [límites de gasto de puerta de enlace](/docs/es/claude-apps-gateway-spend-limits) para límites, cronogramas de reinicio y el mensaje que ve el desarrollador.
* **Una advertencia de contexto o auto-compact**: no es un límite de uso. La conversación ha crecido cerca de la [ventana de auto-compact](/docs/es/model-config#set-the-auto-compact-window) de la sesión, el umbral donde Claude Code resume el historial anterior para liberar espacio. Dirija al desarrollador a [reducir el uso de tokens](#reduce-token-usage).
* **Gasto inesperadamente alto en un plan de API o proveedor de nube**: generalmente se remonta a sesiones largas que nunca se borraron o a Opus dejado como modelo predeterminado. Los hábitos de mayor impacto para compartir son borrar entre tareas no relacionadas y hacer coincidir el modelo con el trabajo, ambos cubiertos en [reducir el uso de tokens](#reduce-token-usage).

<h3 id="agent-team-token-costs">
  Costos de tokens del equipo de agentes
</h3>

[Los equipos de agentes](/docs/es/agent-teams) generan múltiples instancias de Claude Code, cada una con su propia ventana de contexto. El uso de tokens se escala con el número de compañeros de equipo activos y cuánto tiempo se ejecuta cada uno.

Para mantener los costos del equipo de agentes manejables:

* Use Sonnet para compañeros de equipo. Equilibra capacidad y costo para tareas de coordinación.
* Mantenga los equipos pequeños. Cada compañero de equipo ejecuta su propia ventana de contexto, por lo que el uso de tokens es aproximadamente proporcional al tamaño del equipo.
* Mantenga los prompts de generación enfocados. Los compañeros de equipo cargan CLAUDE.md, servidores MCP y skills automáticamente, pero todo en el prompt de generación se suma a su contexto desde el principio.
* Cierre los compañeros de equipo cuando su trabajo esté hecho. Cada compañero de equipo activo continúa consumiendo tokens hasta que se cierre o la sesión finalice.
* Los equipos de agentes están deshabilitados por defecto. Establezca `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` en su [settings.json](/docs/es/settings) o entorno para habilitarlos. Consulte [habilitar equipos de agentes](/docs/es/agent-teams#enable-agent-teams).

<h2 id="reduce-token-usage">
  Reducir el uso de tokens
</h2>

Los costos de tokens se escalan con el tamaño del contexto: cuanto más contexto procesa Claude, más tokens utiliza. Claude Code optimiza automáticamente los costos a través del [almacenamiento en caché de prompts](/docs/es/prompt-caching), que reduce costos para contenido repetido como prompts del sistema, y auto-compactación, que resume el historial de conversación cuando se acerca a los límites del contexto.

Las siguientes estrategias lo ayudan a mantener el contexto pequeño y reducir los costos por mensaje.

<h3 id="manage-context-proactively">
  Gestione el contexto de manera proactiva
</h3>

Use `/usage` para verificar su uso actual de tokens, o [configure su línea de estado](/docs/es/statusline#context-window-usage) para mostrarla continuamente.

* **Limpie entre tareas**: Use `/clear` para comenzar de nuevo cuando cambie a trabajo no relacionado. El contexto obsoleto desperdicia tokens en cada mensaje posterior. Use `/rename` antes de limpiar para que pueda encontrar fácilmente la sesión más tarde, luego `/resume` para volver a ella.
* **Agregue instrucciones de compactación personalizadas**: `/compact Focus on code samples and API usage` le dice a Claude qué preservar durante la summarización. En una sesión nueva, `/compact` imprime `Not enough messages to compact.` porque aún no hay historial de conversación para resumir.

También puede personalizar el comportamiento de compactación en su archivo CLAUDE.md en la raíz de su proyecto:

```markdown theme={null}
# Compact instructions

When you are using compact, please focus on test output and code changes
```

<h3 id="choose-the-right-model">
  Elija el modelo correcto
</h3>

Sonnet maneja bien la mayoría de tareas de codificación y cuesta menos que Opus. Reserve Opus para decisiones arquitectónicas complejas o razonamiento de múltiples pasos. Use `/model` para cambiar modelos a mitad de sesión, o establezca un valor predeterminado en `/config`. Un cambio a Opus también se aplica a los [subagentes que heredan el modelo de su sesión](/docs/es/model-config#setting-your-model). Para tareas simples de subagent, especifique `model: haiku` en su [configuración de subagent](/docs/es/sub-agents#choose-a-model).

<h3 id="reduce-mcp-server-overhead">
  Reduzca la sobrecarga del servidor MCP
</h3>

Las definiciones de herramientas MCP se [difieren por defecto](/docs/es/mcp#scale-with-mcp-tool-search), por lo que solo los nombres de herramientas y las instrucciones del servidor entran en contexto hasta que Claude usa una herramienta específica. Ejecute `/context` para ver qué está consumiendo espacio.

* **Prefiera herramientas CLI cuando estén disponibles**: Herramientas como `gh`, `aws`, `gcloud` y `sentry-cli` son más eficientes en contexto que los servidores MCP porque no agregan ningún listado por herramienta. Claude puede ejecutar comandos CLI directamente.
* **Deshabilite servidores no utilizados**: Ejecute `/mcp` para ver servidores configurados y deshabilite cualquiera que no esté usando activamente.

<h3 id="install-code-intelligence-plugins-for-typed-languages">
  Instale plugins de inteligencia de código para lenguajes tipados
</h3>

[Los plugins de inteligencia de código](/docs/es/plugins/code-intelligence) le dan a Claude navegación de símbolos precisa en lugar de búsqueda basada en texto, reduciendo lecturas de archivos innecesarias al explorar código desconocido. Una única llamada "ir a definición" reemplaza lo que de otro modo sería un grep seguido de lectura de múltiples archivos candidatos. Los servidores de lenguaje instalados también reportan errores de tipo automáticamente después de ediciones, por lo que Claude detecta errores sin ejecutar un compilador.

<h3 id="offload-processing-to-hooks-and-skills">
  Descargue el procesamiento en hooks y skills
</h3>

Los [hooks](/docs/es/hooks) personalizados pueden preprocesar datos antes de que Claude los vea. En lugar de que Claude lea un archivo de registro de 10,000 líneas para encontrar errores, un hook puede buscar `ERROR` y devolver solo las líneas coincidentes, reduciendo el contexto de decenas de miles de tokens a cientos.

Una [skill](/docs/es/skills) puede darle a Claude conocimiento de dominio para que no tenga que explorar. Por ejemplo, una skill "codebase-overview" podría describir la arquitectura de su proyecto, directorios clave y convenciones de nomenclatura. Cuando Claude invoca la skill, obtiene este contexto inmediatamente en lugar de gastar tokens leyendo múltiples archivos para entender la estructura.

Por ejemplo, este hook PreToolUse filtra la salida de prueba para mostrar solo fallos:

<Tabs>
  <Tab title="settings.json">
    Agregue esto a su [settings.json](/docs/es/settings#where-settings-live) para ejecutar el hook antes de cada comando Bash:

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "~/.claude/hooks/filter-test-output.sh"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="filter-test-output.sh">
    El hook llama a este script. Cree la carpeta con `mkdir -p ~/.claude/hooks`, guarde el script a continuación como `~/.claude/hooks/filter-test-output.sh` y hágalo ejecutable con `chmod +x ~/.claude/hooks/filter-test-output.sh`. Verifica si el comando es un ejecutor de pruebas y lo modifica para mostrar solo fallos:

    ```bash theme={null}
    #!/bin/bash
    input=$(cat)
    cmd=$(echo "$input" | jq -r '.tool_input.command')

    # If running tests, filter to show only failures
    if [[ "$cmd" =~ ^(npm test|pytest|go test) ]]; then
      filtered_cmd="$cmd 2>&1 | grep -A 5 -E '(FAIL|ERROR|error:)' | head -100"
      echo "$input" | jq --arg filtered "$filtered_cmd" \
        '{hookSpecificOutput: {hookEventName: "PreToolUse", permissionDecision: "allow", updatedInput: (.tool_input + {command: $filtered})}}'
    else
      echo "{}"
    fi
    ```
  </Tab>
</Tabs>

Para verificar la configuración, ejecute `/hooks` y verifique que el hook aparezca bajo PreToolUse. También puede iniciar Claude Code con `claude --debug-file ./claude-debug.txt` y pedirle a Claude que ejecute `npm test`. Cuando el hook reescribe el comando, ese archivo de registro contiene una línea `modified tool input keys` que enumera `command` y los otros campos de entrada de Bash.

<h3 id="move-instructions-from-claude-md-to-skills">
  Mueva instrucciones de CLAUDE.md a skills
</h3>

Su archivo [CLAUDE.md](/docs/es/memory) se carga en contexto al inicio de la sesión. Si contiene instrucciones detalladas para flujos de trabajo específicos (como revisiones de PR o migraciones de bases de datos), esos tokens están presentes incluso cuando está haciendo trabajo no relacionado. [Skills](/docs/es/skills) se cargan bajo demanda solo cuando se invocan, por lo que mover instrucciones especializadas a skills mantiene su contexto base más pequeño. Apunte a mantener CLAUDE.md bajo 200 líneas incluyendo solo lo esencial.

<h3 id="adjust-extended-thinking">
  Ajuste el pensamiento extendido
</h3>

El pensamiento extendido está habilitado por defecto porque mejora significativamente el rendimiento en tareas complejas de planificación y razonamiento. Los tokens de pensamiento se facturan como tokens de salida, y el presupuesto predeterminado puede ser decenas de miles de tokens por solicitud dependiendo del modelo.

Para tareas más simples donde el razonamiento profundo no es necesario, puede reducir costos bajando el [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) con `/effort` o en `/model`, o deshabilitando el pensamiento en `/config`. No puede desactivar el pensamiento en Opus 5.5 o los modelos Fable, que siempre usan pensamiento extendido.

En modelos con un [presupuesto de pensamiento fijo](/docs/es/model-config#adaptive-reasoning-and-fixed-thinking-budgets), también puede bajar el presupuesto estableciendo la [variable de entorno](/docs/es/env-vars) `MAX_THINKING_TOKENS`, por ejemplo `MAX_THINKING_TOKENS=8000`. Los modelos de razonamiento adaptativo ignoran presupuestos distintos de cero, así que use niveles de esfuerzo en su lugar.

<h3 id="delegate-verbose-operations-to-subagents">
  Delegue operaciones detalladas a subagentes
</h3>

Ejecutar pruebas, obtener documentación o procesar archivos de registro puede consumir contexto significativo. Delegue estos a [subagentes](/docs/es/sub-agents#isolate-high-volume-operations) para que la salida detallada permanezca en el contexto del subagente mientras solo un resumen regresa a su conversación principal.

<h3 id="manage-agent-team-costs">
  Gestione los costos del equipo de agentes
</h3>

Los equipos de agentes usan aproximadamente 7 veces más tokens que sesiones estándar cuando los compañeros de equipo se ejecutan en plan mode, porque cada compañero de equipo mantiene su propia ventana de contexto y se ejecuta como una instancia separada de Claude. Mantenga las tareas del equipo pequeñas y autónomas para limitar el uso de tokens por compañero de equipo. Consulte [equipos de agentes](/docs/es/agent-teams) para obtener detalles.

<h3 id="write-specific-prompts">
  Escriba prompts específicos
</h3>

Solicitudes vagas como "mejorar esta base de código" desencadenan escaneo amplio. Solicitudes específicas como "agregar validación de entrada a la función de inicio de sesión en auth.ts" permiten que Claude trabaje eficientemente con lecturas de archivos mínimas.

<h3 id="work-efficiently-on-complex-tasks">
  Trabaje eficientemente en tareas complejas
</h3>

Para trabajo más largo o más complejo, estos hábitos ayudan a evitar tokens desperdiciados por tomar el camino equivocado:

* **Use plan mode para tareas complejas**: Presione Shift+Tab para entrar en [plan mode](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode) antes de la implementación. Claude explora la base de código y propone un enfoque para su aprobación, previniendo re-trabajo costoso cuando la dirección inicial es incorrecta.
* **Corrija el curso temprano**: Si Claude comienza a ir en la dirección equivocada, presione Escape para detener inmediatamente. Use `/rewind` o presione Escape dos veces para restaurar la conversación y el código a un checkpoint anterior.
* **Proporcione objetivos de verificación**: Incluya casos de prueba, pegue capturas de pantalla o defina la salida esperada en su prompt. Cuando Claude puede verificar su propio trabajo, detecta problemas antes de que necesite solicitar correcciones.
* **Pruebe incrementalmente**: Escriba un archivo, pruébelo, luego continúe. Esto detecta problemas temprano cuando son baratos de arreglar.

<h2 id="background-token-usage">
  Uso de tokens en segundo plano
</h2>

Claude Code usa tokens para algunas funcionalidades en segundo plano incluso cuando está inactivo:

* **Summarización de conversación**: Trabajos en segundo plano que resumen conversaciones anteriores para la característica `claude --resume`
* **Procesamiento de comandos**: Algunos comandos como `/usage` pueden generar solicitudes para verificar el estado

Estos procesos en segundo plano consumen una pequeña cantidad de tokens (típicamente menos de \$0.04 por sesión) incluso sin interacción activa.

Cuando las sugerencias de prompt están activadas, Claude Code también envía una solicitud breve al modelo que utiliza su sesión después de que Claude responde, para [sugerir su próximo prompt](/docs/es/interactive-mode#prompt-suggestions). Esa solicitud reutiliza la caché de prompt de la conversación, por lo que es principalmente lecturas de caché más algunos tokens de salida. Claude Code [omite esto cuando su cuenta está cerca o en su límite de uso](/docs/es/interactive-mode#when-claude-code-skips-suggestions). Para detener estas solicitudes, [desactive las sugerencias de prompt](/docs/es/interactive-mode#turn-prompt-suggestions-off).

<h2 id="why-usage-climbs-in-a-long-session">
  Por qué el uso aumenta en una sesión larga
</h2>

Una sesión que ha estado abierta durante horas puede usar mucho más de los límites de su plan de lo que su actividad sugiere, generalmente por una de estas razones:

* **Contexto largo**: Claude Code envía su conversación completa con cada solicitud, y cada vez que Claude utiliza herramientas envía otra solicitud que lleva ese lote de resultados de herramientas. Con [almacenamiento en caché de solicitudes](/docs/es/prompt-caching), Claude Code relee ese historial a la [tasa de token almacenado en caché](https://platform.claude.com/docs/en/about-claude/pricing), por lo que una pregunta de una línea en una sesión que ha estado abierta todo el día sigue consumiendo uso para toda la conversación. Consulte [Gestionar el contexto de forma proactiva](#manage-context-proactively) para conocer formas de mantener su contexto pequeño
* **Fallos de caché**: su primer mensaje después de una pausa más larga que la [duración del caché](/docs/es/prompt-caching#cache-lifetime) falla en el caché y reprocesa su contexto completo. La duración es de una hora en una suscripción y se reduce a cinco minutos una vez que está utilizando [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans); en una clave API o proveedor de nube, es de cinco minutos por defecto. Para mantener la duración de una hora mientras utiliza créditos de uso, [elija el TTL usted mismo](/docs/es/prompt-caching#choose-the-ttl-yourself). En los planes Pro y Max, cuando reanuda una sesión grande después de una pausa larga, Claude Code [ofrece reanudar desde un resumen](/docs/es/sessions#resume-from-a-summary) para que las solicitudes posteriores no lleven el historial completo
* **Tareas programadas**: una [tarea programada](/docs/es/scheduled-tasks) se ejecuta en su intervalo incluso mientras la sesión está inactiva, enviando su contexto completo cada vez
* **Mensajes entre sesiones**: Claude Code entrega un [mensaje de otra de sus sesiones](/docs/es/cross-session-messaging) como un nuevo turno cuando esta sesión está inactiva, enviando su contexto completo cada vez. Para retener los mensajes entrantes en lugar de entregarlos, establezca [`crossSessionInbound`](/docs/es/settings-reference#crosssessioninbound) en `hold`
* **Verificaciones de objetivos**: mientras el trabajo en segundo plano mantiene un [objetivo](/docs/es/goal) activo en espera, Claude Code [le pide a Claude que verifique ese trabajo](/docs/es/goal#background-work-defers-evaluation) incluso cuando la sesión está inactiva, iniciando un nuevo turno que envía su contexto completo. Claude Code inicia como máximo tres verificaciones inactivas por objetivo entre sus solicitudes. Antes de v2.1.246, las verificaciones inactivas eran ilimitadas. Para desactivar las verificaciones, establezca [`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](/docs/es/env-vars) en `0`. Las verificaciones inactivas requieren Claude Code v2.1.236 o posterior
* **Compañeros agentes**: cada [compañero](/docs/es/agent-team-token-costs) activo sigue consumiendo tokens hasta que sale
* **Compactación**: `/compact` lee la conversación que resume, por lo que [compactar un contexto grande](/docs/es/prompt-caching#compacting-the-conversation) es en sí una solicitud grande. Cuando desea un nuevo comienzo en lugar de continuidad, `/clear` no cuesta nada

En un plan Pro, Max, Team o Enterprise, el desglose de `/usage` marca comportamientos que representan el 10% o más de su uso reciente, como contexto largo o fallos de caché, cada uno con un consejo para reducirlo.

<h2 id="understanding-changes-in-claude-code-behavior">
  Entender los cambios en el comportamiento de Claude Code
</h2>

Claude Code recibe actualizaciones regularmente que pueden cambiar cómo funcionan las características, incluida la información de costos. Ejecute `claude --version` para verificar su versión actual.

Para preguntas sobre facturación de su cuenta específica, póngase en contacto con el soporte de Anthropic a través del mensajero integrado en el producto:

* **Planes de suscripción** (Pro, Max, Team, Enterprise): inicie sesión en [claude.ai](https://claude.ai), haga clic en sus iniciales en la esquina inferior izquierda y seleccione **Obtener ayuda**
* **Facturación de Console (API)**: inicie sesión en [platform.claude.com](https://platform.claude.com), haga clic en sus iniciales y seleccione **Obtener ayuda**

Consulte [Cómo obtener soporte](https://support.claude.com/en/articles/9015913-how-to-get-support) para conocer el flujo completo, incluida información sobre quién puede comunicarse con un agente humano en cada plan.
