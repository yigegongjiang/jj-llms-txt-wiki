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

El bloque Session en la parte superior de `/usage` muestra estadísticas detalladas de uso de tokens para su sesión actual. La cifra en dólares es una estimación calculada localmente a partir de conteos de tokens y puede diferir de su factura real. Para facturación autorizada, consulte la página de Uso en la [Consola de Claude](https://platform.claude.com/usage).

```text theme={null}
Total cost:            $0.55
Total duration (API):  6m 19.7s
Total duration (wall): 6h 33m 10.2s
Total code changes:    0 lines added, 0 lines removed
```

En un plan Pro, Max, Team o Enterprise, `/usage` también muestra un desglose de lo que cuenta contra los límites de su plan. Atribuye el uso reciente a skills, subagentes, plugins y servidores MCP individuales, cada uno mostrado como un porcentaje del total. Presione `d` o `w` para cambiar entre las últimas 24 horas y los últimos 7 días. Las cifras son aproximadas y se calculan a partir del historial de sesión local en esta máquina, por lo que el uso de otros dispositivos o claude.ai no se incluye.

Cuando la solicitud de sus límites de plan falla, la mayoría de las veces porque el punto final de uso tiene límite de velocidad, `/usage` muestra las últimas barras de uso que cargó en esta máquina dentro de los últimos 60 minutos, junto con una nota `Showing last-known usage` que indica cuánto tiempo hace que se obtuvieron esos datos. Presione `r` para reintentar; un reintento exitoso reemplaza las últimas barras conocidas con datos frescos. Sin una instantánea de los últimos 60 minutos, `/usage` informa que el punto final de uso tiene límite de velocidad y ofrece el mismo atajo de reintento. Antes de v2.1.208, una solicitud con límite de velocidad en una sesión que aún no había cargado uso siempre mostraba el error sin barras.

En la [extensión de VS Code](/docs/es/vs-code#check-account-and-usage), el mismo desglose aparece en el diálogo Cuenta y uso con un botón de alternancia Día y Semana. Requiere Claude Code v2.1.174 o posterior.

<h3 id="set-a-spend-limit-on-pro-and-max">
  Establecer un límite de gasto en Pro y Max
</h3>

En los planes Pro y Max, el comando `/usage-credits` abre un diálogo en la CLI donde puede administrar [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans). Desde el diálogo puede:

* Activar créditos de uso para su cuenta
* Comprar más créditos de uso, ya sea un paquete listado o una cantidad personalizada
* Establecer, cambiar o eliminar su límite de gasto mensual
* Configurar recarga automática, que compra más créditos de uso automáticamente cuando su saldo cae por debajo de un umbral que establece

En versiones de Claude Code anteriores a v2.1.207 y en cuentas donde el diálogo en la CLI no está disponible, `/usage-credits` abre la página de facturación de créditos de uso en su navegador en su lugar. En planes Team y Enterprise, los miembros con acceso de facturación obtienen la misma página del navegador, y los miembros sin acceso de facturación envían una solicitud desde la CLI pidiendo a su administrador que active los créditos de uso o aumente el límite.

Cambiar el límite de gasto mensual requiere acceso de facturación en la cuenta. Si alcanza el límite mientras aún tiene créditos de uso disponibles, Claude Code le solicita que aumente o elimine el límite para que pueda continuar sin abandonar la CLI.

Los montos que escribe en el diálogo, como una cantidad de compra personalizada, el límite de gasto mensual, o el umbral y objetivo de recarga automática, deben ser dígitos, opcionalmente seguidos de un período y uno o dos dígitos decimales, por ejemplo `20` o `20.50`. Cualquier otra entrada, incluidas comas, muestra un error en línea y no se guarda. Las versiones anteriores a v2.1.207 no muestran el diálogo y abren la página de facturación en su lugar.

Claude Code le solicita que escriba `yes` para confirmar cada compra y cada cambio de recarga automática, sin importar la cantidad, y la confirmación de compra muestra el total después de impuestos que está aprobando. Cambiar el límite de gasto mensual solicita la misma confirmación escrita solo por encima de \$1,000, o por encima de 1,000 unidades de una moneda de facturación que no sea dólar estadounidense. Antes de v2.1.208, las compras y los cambios de recarga automática usaban ese umbral también, por lo que los montos más pequeños pasaban por el flujo de diálogo estándar sin el paso adicional de `yes` escrito.

Los campos de cantidad se abren rellenados previamente con un valor sugerido, y el primer dígito que escribe reemplaza la sugerencia en lugar de agregarse a ella. La pantalla que activa los créditos de uso se abre con Cancelar seleccionado, por lo que activarlos requiere una selección deliberada en lugar de un Enter accidental. Ambos requieren Claude Code v2.1.208 o posterior.

<h2 id="manage-costs-for-your-organization">
  Gestión de costos para su organización
</h2>

Los controles que tiene dependen de cómo su organización accede a Claude Code: un plan Claude for Teams o Enterprise, la Consola de Claude, o un proveedor de nube. En los planes Teams y Enterprise, el uso se extrae de la asignación de cada miembro. En la Consola y en proveedores de nube, el uso se factura por token a su organización. Si su organización mezcla métodos de inicio de sesión, cada desarrollador se mide según el que autenticó.

La tabla asigna cada configuración a dónde ve el gasto, dónde lo limita y cómo extrae números por usuario.

| Su configuración                                                                               | Ver gasto                                                                                                                                 | Limitar gasto                                      | Informes por usuario                                                                                                                                                                                                               |
| :--------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Claude for Teams o Enterprise](#claude-for-teams-and-enterprise)                              | [Informe de gasto en análisis de org](https://support.claude.com/es/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) | Límites de gasto en configuración de administrador | [CSV de informe de gasto](https://support.claude.com/es/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans); [API de Análisis Enterprise](https://platform.claude.com/docs/es/api/admin/analytics) en Enterprise |
| [Consola de Claude (API)](#claude-console)                                                     | [Página de uso de Consola](https://platform.claude.com/usage)                                                                             | Límites de gasto del espacio de trabajo            | [Panel de Consola](https://platform.claude.com/claude-code), [API de Análisis de Claude Code](https://platform.claude.com/docs/es/build-with-claude/claude-code-analytics-api)                                                     |
| [Amazon Bedrock, Plataforma de Agentes de Google Cloud, o Microsoft Foundry](#cloud-providers) | Su consola de facturación en la nube                                                                                                      | Controles de presupuesto de su nube                | [OpenTelemetry](/docs/es/monitoring-usage) o una [puerta de enlace LLM](/docs/es/llm-gateway)                                                                                                                                                |

[La exportación de OpenTelemetry](/docs/es/monitoring-usage) funciona en cada configuración y es la única opción que transmite métricas de tokens y costos por usuario a su propia pila de observabilidad en tiempo casi real.

<h3 id="claude-for-teams-and-enterprise">
  Claude for Teams y Enterprise
</h3>

En los planes Claude for Teams y Enterprise, el uso de Claude Code de cada miembro se extrae de una asignación por puesto que se reinicia en una ventana de cinco horas móvil y una ventana semanal. La asignación se comparte con Claude chat y Cowork, y su tamaño depende del [nivel de puesto](https://support.claude.com/es/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan) (Standard o Premium). Sus controles se encuentran en la consola de administrador de claude.ai, no en la Consola de Claude.

* **Ver gasto**: el [informe de gasto en análisis de org](https://support.claude.com/es/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) muestra el gasto estimado por usuario y por modelo, con exportación CSV, actualizado diariamente. El informe cubre el gasto de créditos de uso y aparece una vez que se activan los créditos de uso. El uso dentro de la asignación de puesto no se mide en dólares.
* **Ver adopción**: el [panel de análisis](https://claude.ai/analytics/claude-code) muestra usuarios activos diarios, sesiones y métricas de contribución, con exportación CSV de datos de contribución. Consulte [rastrear el uso del equipo con análisis](/docs/es/analytics).
* **Limitar gasto**: la asignación de puesto es el techo predeterminado. Para permitir que los miembros continúen más allá, active [créditos de uso](https://support.claude.com/es/articles/12429409-extra-usage-for-paid-claude-plans) y establezca límites de gasto a nivel de organización, grupo o miembro individual.
* **Extraer números por usuario**: en el plan Enterprise, la [API de Análisis Enterprise](https://platform.claude.com/docs/es/api/admin/analytics) devuelve informes de uso y costo por usuario en todas las superficies de Claude, incluido Claude Code. Un Propietario Principal crea una clave con el alcance `read:analytics` en [claude.ai/analytics/api-keys](https://claude.ai/analytics/api-keys). En el plan Teams, exporte el [CSV de informe de gasto](https://support.claude.com/es/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans), que enumera el uso de tokens y el gasto estimado por usuario y por modelo.

La [Guía de consumo de Claude Enterprise](https://support.claude.com/es/articles/14782391-claude-enterprise-consumption-guide) es la referencia de planificación para administradores. Explica cómo difiere el consumo en Claude chat, Claude Code y Cowork, y proporciona puntos de partida en dólares por usuario para presupuestar. Presupueste más para un puesto de codificación que para un puesto de chat: cada turno de Claude Code lleva contenidos de archivo, llamadas de herramientas y razonamiento de múltiples pasos, por lo que una sesión de depuración puede consumir más que un día de chat.

<h3 id="claude-console">
  Consola de Claude
</h3>

Las organizaciones de API administran el gasto de Claude Code a través de [espacios de trabajo](https://platform.claude.com/docs/es/build-with-claude/workspaces). Puede [establecer límites de gasto del espacio de trabajo](https://platform.claude.com/docs/es/build-with-claude/workspaces#workspace-limits) en el gasto total de Claude Code y [ver informes de costos y uso](https://platform.claude.com/docs/es/build-with-claude/workspaces#usage-and-cost-tracking) en la Consola.

<Note>
  Cuando autentica por primera vez Claude Code con su cuenta de Claude Console, se crea automáticamente un espacio de trabajo llamado "Claude Code" para usted. Este espacio de trabajo proporciona seguimiento y gestión centralizada de costos para todo el uso de Claude Code en su organización. No puede crear claves de API para este espacio de trabajo; es exclusivamente para autenticación y uso de Claude Code.

  Para organizaciones con límites de velocidad personalizados, el tráfico de Claude Code en este espacio de trabajo cuenta hacia los límites de velocidad de API generales de su organización. Puede establecer un [límite de velocidad del espacio de trabajo](https://platform.claude.com/docs/es/api/rate-limits#setting-lower-limits-for-workspaces) en la página Limits de este espacio de trabajo en la Consola de Claude para limitar la parte de Claude Code y proteger otras cargas de trabajo de producción.
</Note>

Para informes por usuario, el [panel de Consola](https://platform.claude.com/claude-code) muestra gasto y líneas aceptadas por miembro, y la [API de Análisis de Claude Code](https://platform.claude.com/docs/es/build-with-claude/claude-code-analytics-api) devuelve las mismas métricas diarias por usuario mediante programación con una [clave de API de administrador](https://platform.claude.com/settings/admin-keys). Consulte [análisis para clientes de API](/docs/es/analytics#access-analytics-for-api-customers).

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

Los desarrolladores generalmente llevan preguntas sobre límites a su administrador, por lo que es útil saber qué techo alcanzaron. Las tres situaciones significan cosas diferentes:

* **"Ha alcanzado su límite de sesión" o "Ha alcanzado su límite semanal"**: una ventana de uso basada en puesto en un plan de suscripción. Estas ventanas se comparten en todos los modelos, por lo que cambiar modelos con `/model` no restaura el acceso, aunque permite que el desarrollador continúe trabajando después del mensaje específico del modelo "Ha alcanzado su límite de Opus". El mensaje muestra cuándo se reinicia la ventana, y el desarrollador puede ejecutar `/usage-credits` para solicitar uso más allá de la asignación si tiene [créditos de uso](https://support.claude.com/es/articles/12429409-extra-usage-for-paid-claude-plans) activados. Consulte [errores de límite de uso](/docs/es/errors#youve-hit-your-session-limit).
* **Una advertencia de contexto o auto-compact**: no es un límite de uso. La conversación ha crecido cerca del tamaño máximo de entrada del modelo, y Claude Code resume el historial anterior para liberar espacio. Dirija al desarrollador a [reducir el uso de tokens](#reduce-token-usage).
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
* **Agregue instrucciones de compactación personalizadas**: `/compact Focus on code samples and API usage` le dice a Claude qué preservar durante la summarización.

También puede personalizar el comportamiento de compactación en su archivo CLAUDE.md en la raíz de su proyecto:

```markdown theme={null}
# Compact instructions

When you are using compact, please focus on test output and code changes
```

<h3 id="choose-the-right-model">
  Elija el modelo correcto
</h3>

Sonnet maneja bien la mayoría de tareas de codificación y cuesta menos que Opus. Reserve Opus para decisiones arquitectónicas complejas o razonamiento de múltiples pasos. Use `/model` para cambiar modelos a mitad de sesión, o establezca un valor predeterminado en `/config`. Para tareas simples de subagent, especifique `model: haiku` en su [configuración de subagent](/docs/es/sub-agents#choose-a-model).

<h3 id="reduce-mcp-server-overhead">
  Reduzca la sobrecarga del servidor MCP
</h3>

Las definiciones de herramientas MCP se [difieren por defecto](/docs/es/mcp#scale-with-mcp-tool-search), por lo que solo los nombres de herramientas entran en contexto hasta que Claude usa una herramienta específica. Ejecute `/context` para ver qué está consumiendo espacio.

* **Prefiera herramientas CLI cuando estén disponibles**: Herramientas como `gh`, `aws`, `gcloud` y `sentry-cli` son más eficientes en contexto que los servidores MCP porque no agregan ningún listado por herramienta. Claude puede ejecutar comandos CLI directamente.
* **Deshabilite servidores no utilizados**: Ejecute `/mcp` para ver servidores configurados y deshabilite cualquiera que no esté usando activamente.

<h3 id="install-code-intelligence-plugins-for-typed-languages">
  Instale plugins de inteligencia de código para lenguajes tipados
</h3>

[Los plugins de inteligencia de código](/docs/es/discover-plugins#code-intelligence) le dan a Claude navegación de símbolos precisa en lugar de búsqueda basada en texto, reduciendo lecturas de archivos innecesarias al explorar código desconocido. Una única llamada "ir a definición" reemplaza lo que de otro modo sería un grep seguido de lectura de múltiples archivos candidatos. Los servidores de lenguaje instalados también reportan errores de tipo automáticamente después de ediciones, por lo que Claude detecta errores sin ejecutar un compilador.

<h3 id="offload-processing-to-hooks-and-skills">
  Descargue el procesamiento en hooks y skills
</h3>

Los [hooks](/docs/es/hooks) personalizados pueden preprocesar datos antes de que Claude los vea. En lugar de que Claude lea un archivo de registro de 10,000 líneas para encontrar errores, un hook puede buscar `ERROR` y devolver solo las líneas coincidentes, reduciendo el contexto de decenas de miles de tokens a cientos.

Una [skill](/docs/es/skills) puede darle a Claude conocimiento de dominio para que no tenga que explorar. Por ejemplo, una skill "codebase-overview" podría describir la arquitectura de su proyecto, directorios clave y convenciones de nomenclatura. Cuando Claude invoca la skill, obtiene este contexto inmediatamente en lugar de gastar tokens leyendo múltiples archivos para entender la estructura.

Por ejemplo, este hook PreToolUse filtra la salida de prueba para mostrar solo fallos:

<Tabs>
  <Tab title="settings.json">
    Agregue esto a su [settings.json](/docs/es/settings#settings-files) para ejecutar el hook antes de cada comando Bash:

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
      echo "{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"permissionDecision\":\"allow\",\"updatedInput\":{\"command\":\"$filtered_cmd\"}}}"
    else
      echo "{}"
    fi
    ```
  </Tab>
</Tabs>

<h3 id="move-instructions-from-claude-md-to-skills">
  Mueva instrucciones de CLAUDE.md a skills
</h3>

Su archivo [CLAUDE.md](/docs/es/memory) se carga en contexto al inicio de la sesión. Si contiene instrucciones detalladas para flujos de trabajo específicos (como revisiones de PR o migraciones de bases de datos), esos tokens están presentes incluso cuando está haciendo trabajo no relacionado. [Skills](/docs/es/skills) se cargan bajo demanda solo cuando se invocan, por lo que mover instrucciones especializadas a skills mantiene su contexto base más pequeño. Apunte a mantener CLAUDE.md bajo 200 líneas incluyendo solo lo esencial.

<h3 id="adjust-extended-thinking">
  Ajuste el pensamiento extendido
</h3>

El pensamiento extendido está habilitado por defecto porque mejora significativamente el rendimiento en tareas complejas de planificación y razonamiento. Los tokens de pensamiento se facturan como tokens de salida, y el presupuesto predeterminado puede ser decenas de miles de tokens por solicitud dependiendo del modelo. Para tareas más simples donde el razonamiento profundo no es necesario, puede reducir costos bajando el [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) con `/effort` o en `/model`, deshabilitando el pensamiento en `/config`, o, en modelos con un [presupuesto de pensamiento fijo](/docs/es/model-config#adaptive-reasoning-and-fixed-thinking-budgets), bajando el presupuesto estableciendo la [variable de entorno](/docs/es/env-vars) `MAX_THINKING_TOKENS`, por ejemplo `MAX_THINKING_TOKENS=8000`. Los modelos de razonamiento adaptativo ignoran presupuestos distintos de cero, así que use niveles de esfuerzo en su lugar. Deshabilitar el pensamiento no está disponible en Fable 5, que siempre usa pensamiento extendido.

<h3 id="delegate-verbose-operations-to-subagents">
  Delegue operaciones detalladas a subagents
</h3>

Ejecutar pruebas, obtener documentación o procesar archivos de registro puede consumir contexto significativo. Delegue estos a [subagents](/docs/es/sub-agents#isolate-high-volume-operations) para que la salida detallada permanezca en el contexto del subagent mientras solo un resumen regresa a su conversación principal.

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

<h2 id="understanding-changes-in-claude-code-behavior">
  Comprensión de cambios en el comportamiento de Claude Code
</h2>

Claude Code recibe actualizaciones regularmente que pueden cambiar cómo funcionan las características, incluido el reporte de costos. Ejecute `claude --version` para verificar su versión actual. Para preguntas específicas de facturación, contacte al soporte de Anthropic a través de su [cuenta de Consola](https://platform.claude.com/login).
