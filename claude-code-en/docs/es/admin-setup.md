> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurar Claude Code para su organización

> Un mapa de decisiones para administradores que implementan Claude Code, cubriendo proveedores de API, configuración administrada, aplicación de políticas, monitoreo de uso y manejo de datos.

Claude Code aplica la política de la organización a través de configuraciones administradas que tienen prioridad sobre la configuración local del desarrollador. Usted entrega esa configuración desde la consola de administración de Claude, su sistema de gestión de dispositivos móviles (MDM), o un archivo en disco. La configuración controla qué herramientas, comandos, servidores y destinos de red puede alcanzar Claude.

Esta página lo guía a través de las decisiones de implementación en orden. Cada fila se vincula a la sección a continuación y a la página de referencia para esa área.

<Note>
  SSO, aprovisionamiento SCIM y asignación de asientos se configuran a nivel de cuenta de Claude. Consulte la [Guía del administrador empresarial de Claude](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide) y [asignación de asientos](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan) para esos pasos.
</Note>

| Decisión                                                                                     | Lo que está eligiendo                                                     | Referencia                                                                                                                                                                    |
| :------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Elegir su proveedor de API](#choose-your-api-provider)                                      | Dónde Claude Code se autentica y cómo se factura                          | [Authentication](/docs/es/authentication), [Amazon Bedrock](/docs/es/amazon-bedrock), [Google Cloud's Agent Platform](/docs/es/google-vertex-ai), [Microsoft Foundry](/docs/es/microsoft-foundry) |
| [Decidir cómo llega la configuración a los dispositivos](#decide-how-settings-reach-devices) | Cómo la política administrada llega a las máquinas de los desarrolladores | [Server-managed settings](/docs/es/server-managed-settings), [Settings files](/docs/es/settings#settings-files)                                                                         |
| [Decidir qué aplicar](#decide-what-to-enforce)                                               | Qué herramientas, comandos e integraciones están permitidas               | [Permissions](/docs/es/permissions), [Sandboxing](/docs/es/sandboxing)                                                                                                                  |
| [Configurar visibilidad de uso](#set-up-usage-visibility)                                    | Cómo rastrear el gasto y la adopción                                      | [Analytics](/docs/es/analytics), [Monitoring](/docs/es/monitoring-usage), [Costs](/docs/es/costs)                                                                                            |
| [Revisar el manejo de datos](#review-data-handling)                                          | Retención de datos y postura de cumplimiento                              | [Data usage](/docs/es/data-usage), [Security](/docs/es/security)                                                                                                                        |

<h2 id="choose-your-api-provider">
  Elegir su proveedor de API
</h2>

Claude Code se conecta a Claude a través de uno de varios proveedores de API. Su elección afecta la facturación, la autenticación, qué postura de cumplimiento hereda y qué características de Claude Code pueden usar sus desarrolladores.

| Proveedor                     | Elija esto cuando                                                                                                                                  |
| :---------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude for Teams / Enterprise | Desea Claude Code y claude.ai bajo una suscripción por asiento con ninguna infraestructura para ejecutar. Esta es la recomendación predeterminada. |
| Claude Console                | Es API-first o desea facturación de pago por uso                                                                                                   |
| Amazon Bedrock                | Desea heredar controles de cumplimiento y facturación de AWS existentes                                                                            |
| Google Cloud's Agent Platform | Desea heredar controles de cumplimiento y facturación de GCP existentes                                                                            |
| Microsoft Foundry             | Desea heredar controles de cumplimiento y facturación de Azure existentes                                                                          |

Algunas características de Claude Code requieren una cuenta de claude.ai. [Claude Code en la web](/docs/es/claude-code-on-the-web), [Routines](/docs/es/routines), [Code Review](/docs/es/code-review), [Remote Control](/docs/es/remote-control) y la [extensión de Chrome](/docs/es/chrome) no están disponibles solo a través de claves de API de Console o credenciales de proveedores en la nube. Si implementa a través de Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry, planifique si los desarrolladores también necesitan asientos de Claude for Teams o Enterprise. Cada página de características enumera sus requisitos de plan.

Para la comparación completa del proveedor que cubre autenticación, regiones y paridad de características, consulte la [descripción general de implementación empresarial](/docs/es/third-party-integrations). La configuración de autenticación de cada proveedor está en [Authentication](/docs/es/authentication).

Los requisitos de proxy y firewall en [Network configuration](/docs/es/network-config) se aplican independientemente del proveedor. Si desea un único punto final frente a múltiples proveedores o registro de solicitudes centralizado, consulte [LLM gateway](/docs/es/llm-gateway).

<h2 id="decide-how-settings-reach-devices">
  Decidir cómo llega la configuración a los dispositivos
</h2>

La configuración administrada define la política que tiene prioridad sobre la configuración local del desarrollador. Claude Code comprueba las cuatro fuentes que se indican a continuación en orden de prioridad y aplica la primera que devuelve una configuración no vacía, con una excepción: un pequeño conjunto de [claves de bloqueo entre fuentes](/docs/es/settings#settings-precedence), como los bloqueos de lista de permitidos de sandbox, se respeta cuando cualquier fuente controlada por administrador los establece.

| Mecanismo               | Entrega                                                                                                                                                                                             | Prioridad | Plataformas    |
| :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------- | :------------- |
| Server-managed          | Consola de administración de claude.ai, o una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) autohospedada para inicios de sesión de puerta de enlace                           | Más alta  | Todas          |
| plist / registry policy | macOS: `com.anthropic.claudecode` plist<br />Windows: `HKLM\SOFTWARE\Policies\ClaudeCode`                                                                                                           | Alta      | macOS, Windows |
| File-based managed      | macOS: `/Library/Application Support/ClaudeCode/managed-settings.json`<br />Linux y WSL: `/etc/claude-code/managed-settings.json`<br />Windows: `C:\Program Files\ClaudeCode\managed-settings.json` | Media     | Todas          |
| Windows user registry   | `HKCU\SOFTWARE\Policies\ClaudeCode`                                                                                                                                                                 | Más baja  | Solo Windows   |

Un [`policyHelper`](/docs/es/settings#compute-managed-settings-with-a-policy-helper) configurado tiene prioridad sobre las cuatro fuentes: su salida se convierte en la única configuración administrada para la ejecución. Consulte [Settings precedence](/docs/es/settings#settings-precedence).

La configuración administrada por servidor llega a los dispositivos en el momento de la autenticación y se actualiza cada hora durante las sesiones activas, sin infraestructura de punto final. La entrega a través de la consola de administración de claude.ai requiere un plan Claude for Teams o Enterprise. Las implementaciones en Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry pueden obtener la misma entrega remota ejecutando una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway), o usar uno de los mecanismos basados en archivos o a nivel del SO en su lugar.

Si su organización mezcla proveedores, configure [server-managed settings](/docs/es/server-managed-settings) para usuarios de claude.ai más un [respaldo basado en archivos o plist/registry](/docs/es/settings#settings-files) para que otros usuarios aún reciban política administrada.

Las ubicaciones de plist y registro HKLM funcionan con cualquier proveedor y resisten la manipulación porque requieren privilegios de administrador para escribir. El registro de usuario de Windows en HKCU se puede escribir sin elevación, así que trátelo como un valor predeterminado de conveniencia en lugar de un canal de aplicación.

Por defecto, WSL lee solo la ruta de archivo de Linux en `/etc/claude-code`. Para extender su política de registro de Windows y `C:\Program Files\ClaudeCode` a WSL en la misma máquina, establezca [`wslInheritsWindowsSettings: true`](/docs/es/settings#available-settings) en cualquiera de esas fuentes de solo administrador de Windows.

<h3 id="wsl-sessions-in-claude-code-desktop">
  Sesiones de WSL en Claude Code Desktop
</h3>

En Windows, [Claude Code Desktop puede ejecutar sesiones de Code dentro de una distribución de WSL 2](/docs/es/desktop-wsl). El proceso de Claude Code de la sesión se ejecuta dentro de la distribución, por lo que resuelve la configuración administrada a través de la ruta de descubrimiento de WSL anterior: las fuentes solo de Windows no la alcanzan a menos que `wslInheritsWindowsSettings: true` esté implementado.

En dispositivos donde la configuración administrada está presente, las sesiones de Desktop WSL no están disponibles de forma predeterminada. Si su organización desea habilitarlas, póngase en contacto con su equipo de cuenta de Anthropic. Cuando estén habilitadas:

* Implemente `wslInheritsWindowsSettings: true` a través del registro HKLM o del archivo `C:\Program Files\ClaudeCode` para que las sesiones de WSL hereden la misma política que las sesiones del host.
* Verifique ejecutando `/status` dentro de una sesión de WSL: la línea `Setting sources` debe mostrar `Enterprise managed settings` con la fuente de Windows que implementó, `(HKLM)` o `(file)`.

Los procesos dentro de la máquina virtual de utilidad de WSL 2 no son visibles para los sensores de detección de puntos finales del lado de Windows. Si utiliza CrowdStrike Falcon, habilite el sensor de Falcon para Linux en WSL 2 con las dos exclusiones que requiere la documentación de WSL de CrowdStrike, para el proceso de máquina virtual de WSL y la imagen de disco de VM, para que la actividad de procesos y archivos dentro de la distribución sea observable. La [telemetría de ejecución de herramientas de OpenTelemetry](/docs/es/monitoring-usage) de Claude Code se emite de forma idéntica para sesiones de WSL y nativas.

Cualquiera que sea el mecanismo que elija, los valores administrados tienen prioridad sobre la configuración de usuario y proyecto. La configuración de matriz como `permissions.allow` y `permissions.deny` fusionan entradas de todas las fuentes, por lo que los desarrolladores pueden extender listas administradas pero no eliminar de ellas. Para [dos excepciones](/docs/es/settings#settings-precedence), `fallbackModel` y `availableModels`, el valor administrado reemplaza capas inferiores en lugar de fusionarse.

Consulte [Server-managed settings](/docs/es/server-managed-settings) y [Settings files and precedence](/docs/es/settings#settings-files).

<h2 id="decide-what-to-enforce">
  Decidir qué aplicar
</h2>

La configuración administrada puede bloquear herramientas, ejecución de sandbox, restringir servidores MCP y fuentes de plugins, y controlar qué hooks se ejecutan. Cada fila es una superficie de control con las claves de configuración que la impulsan.

| Control                                                                                | Lo que hace                                                                                                                                                                                                                                                                                       | Configuraciones clave                                                                                               |
| :------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------ |
| [Permission rules](/docs/es/permissions)                                                    | Permitir, preguntar o denegar herramientas y comandos específicos                                                                                                                                                                                                                                 | `permissions.allow`, `permissions.deny`                                                                             |
| [Permission lockdown](/docs/es/permissions#managed-only-settings)                           | Solo se aplican reglas de permisos administradas; deshabilitar `--dangerously-skip-permissions`                                                                                                                                                                                                   | `allowManagedPermissionRulesOnly`, `permissions.disableBypassPermissionsMode`                                       |
| [Sandboxing](/docs/es/sandboxing)                                                           | Aislamiento de sistema de archivos y red a nivel del SO con listas de permitidos de dominio                                                                                                                                                                                                       | `sandbox.enabled`, `sandbox.network.allowedDomains`                                                                 |
| [Managed policy CLAUDE.md](/docs/es/memory#deploy-organization-wide-claude-md)              | Instrucciones de toda la organización cargadas en cada sesión, no se pueden excluir                                                                                                                                                                                                               | Archivo en la ruta de política administrada                                                                         |
| [MCP server control](/docs/es/managed-mcp)                                                  | Restringir qué servidores MCP pueden agregar o conectar los usuarios, o implementar un conjunto fijo                                                                                                                                                                                              | `allowedMcpServers`, `deniedMcpServers`, `allowManagedMcpServersOnly`, o un archivo `managed-mcp.json` implementado |
| [Plugin marketplace control](/docs/es/plugin-marketplaces#managed-marketplace-restrictions) | Restringir qué fuentes de marketplace pueden agregar e instalar los usuarios, y rechazar las banderas CLI que cargan plugins, agents y servidores MCP para una única ejecución                                                                                                                    | `strictKnownMarketplaces`, `blockedMarketplaces`, `disableSideloadFlags`                                            |
| [Customization lockdown](/docs/es/settings#strictpluginonlycustomization)                   | Bloquear skills, agents, hooks y servidores MCP de fuentes de usuario y proyecto, para que solo provengan de plugins o configuración administrada                                                                                                                                                 | `strictPluginOnlyCustomization`                                                                                     |
| [Hook restrictions](/docs/es/settings#hook-configuration)                                   | Solo se cargan hooks administrados; restringir URLs de hooks HTTP                                                                                                                                                                                                                                 | `allowManagedHooksOnly`, `allowedHttpHookUrls`                                                                      |
| [Disable agent view](/docs/es/agent-view#how-background-sessions-are-hosted)                | Desactivar `claude agents`, `--bg`, `/background` y el supervisor bajo demanda                                                                                                                                                                                                                    | `disableAgentView`                                                                                                  |
| [Model restrictions](/docs/es/model-config#restrict-model-selection)                        | `availableModels` filtra qué modelos aparecen en el selector. Agregar `enforceAvailableModels` también restringe el modelo predeterminado seleccionado automáticamente. Consulte [surface coverage](/docs/es/model-config#surface-coverage) para ver cómo esta configuración llega a la CLI, web e IDE | `availableModels`, `enforceAvailableModels`                                                                         |
| [Version floor](/docs/es/settings)                                                          | Evitar que la actualización automática instale por debajo de un mínimo de toda la organización                                                                                                                                                                                                    | `minimumVersion`                                                                                                    |
| [Required version range](/docs/es/settings)                                                 | Rechazar iniciar completamente cuando la versión en ejecución está fuera de un rango aprobado por la organización. Más fuerte que `minimumVersion`, que solo bloquea degradaciones                                                                                                                | `requiredMinimumVersion`, `requiredMaximumVersion`                                                                  |

Las organizaciones cuyos miembros se autentican a través de claude.ai o la API de Anthropic también pueden gobernar modelos sin implementar configuración: [restricciones de modelo de organización](/docs/es/model-config#organization-model-restrictions) deshabilitan modelos individuales, un [modelo predeterminado de organización](/docs/es/model-config#organization-default-model) establece en qué modelo comienzan las nuevas sesiones, y [límites de esfuerzo de organización](/docs/es/model-config#organization-effort-limits) limitan los niveles de esfuerzo por rol. Los tres controles requieren un plan Claude Enterprise. Las restricciones de modelo y los límites de esfuerzo se aplican del lado del servidor; el modelo predeterminado es un punto de partida que los usuarios pueden cambiar, a menos que la organización lo aplique. La aplicación está disponible para un conjunto limitado de organizaciones; consulte con su equipo de cuenta de Anthropic sobre la disponibilidad. Ninguno de estos controles llega a sesiones en Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, o [Claude Platform on AWS](/docs/es/claude-platform-on-aws); en esos proveedores, use `availableModels` arriba para restricciones y la clave `model` en configuración administrada para un predeterminado.

Las reglas de permisos y el sandboxing cubren diferentes capas. Denegar WebFetch bloquea la herramienta de búsqueda de Claude, pero si Bash está permitido, `curl` y `wget` aún pueden alcanzar cualquier URL. El sandboxing cierra esa brecha con una lista de permitidos de dominio de red aplicada a nivel del SO.

Para el modelo de amenaza que estos controles defienden, consulte [Security](/docs/es/security).

<h2 id="set-up-usage-visibility">
  Configurar visibilidad de uso
</h2>

Elija monitoreo basado en lo que necesita reportar. Los paneles, las API y los controles de gasto difieren entre los planes Claude for Teams o Enterprise y las organizaciones de Claude Console, así que verifique la columna Disponibilidad antes de planificar su reporte alrededor de una capacidad.

| Capacidad              | Lo que obtiene                                                                                                                           | Disponibilidad                                                                                                                                                                                                                                                                              | Dónde comenzar                                        |
| :--------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------- |
| Usage monitoring       | Exportación de OpenTelemetry de sesiones, herramientas y tokens                                                                          | Todos los proveedores                                                                                                                                                                                                                                                                       | [Monitoring usage](/docs/es/monitoring-usage)              |
| Analytics dashboard    | Métricas de adopción y contribución con una tabla de clasificación en Teams / Enterprise; métricas de uso y gasto por usuario en Console | Teams / Enterprise en [claude.ai/analytics](https://claude.ai/analytics/claude-code), Console en [platform.claude.com/claude-code](https://platform.claude.com/claude-code)                                                                                                                 | [Analytics](/docs/es/analytics)                            |
| Programmatic reporting | Datos de uso y costo por usuario a través de una API                                                                                     | [Enterprise Analytics API](https://support.claude.com/en/articles/13703965-claude-enterprise-analytics-api-reference-guide) para Enterprise, [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) para Console                      | [Costs](/docs/es/costs#manage-costs-for-your-organization) |
| Spend controls         | Límites de gasto y límites de velocidad                                                                                                  | Configuración de administrador para Teams / Enterprise, límites de espacio de trabajo para Console; en nubes de terceros, controles de presupuesto en la nube o una [Claude apps gateway](/docs/es/claude-apps-gateway) con [límites de gasto](/docs/es/claude-apps-gateway-spend-limits) por usuario | [Costs](/docs/es/costs#manage-costs-for-your-organization) |

En Teams y Enterprise, los números de uso y gasto por usuario provienen del [informe de gasto](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) en la configuración de análisis de su organización, no del panel de análisis. Los proveedores de nube exponen el gasto a través de AWS Cost Explorer, GCP Billing o Azure Cost Management. Para planificar presupuestos empresariales en Claude chat, Claude Code y Cowork, consulte la [guía de consumo de Claude Enterprise](https://support.claude.com/en/articles/14782391-claude-enterprise-consumption-guide).

<h2 id="review-data-handling">
  Revisar el manejo de datos
</h2>

En planes de Team, Enterprise, Claude API y proveedores de nube, Anthropic no entrena modelos con su código o indicaciones. Su proveedor de API determina la retención y la postura de cumplimiento.

| Tema                      | Lo que debe saber                                                                            | Dónde comenzar                                 |
| :------------------------ | :------------------------------------------------------------------------------------------- | :--------------------------------------------- |
| Data usage policy         | Qué recopila Anthropic, cuánto tiempo se retiene, qué nunca se usa para entrenamiento        | [Data usage](/docs/es/data-usage)                   |
| Zero Data Retention (ZDR) | Nada almacenado después de que se completa la solicitud. Disponible en Claude for Enterprise | [Zero data retention](/docs/es/zero-data-retention) |
| Security architecture     | Modelo de red, cifrado, autenticación, pista de auditoría                                    | [Security](/docs/es/security)                       |

Si necesita registro de auditoría a nivel de solicitud o enrutar tráfico por sensibilidad de datos, coloque una puerta de enlace entre desarrolladores y su proveedor: una [Claude apps gateway](/docs/es/claude-apps-gateway) autohospedada registra un registro de auditoría por solicitud con identidad de IdP, o use otra [LLM gateway](/docs/es/llm-gateway). Para requisitos regulatorios y certificaciones, consulte [Legal and compliance](/docs/es/legal-and-compliance).

<h2 id="verify-and-onboard">
  Verificar e incorporar
</h2>

Después de configurar la configuración administrada, haga que un desarrollador ejecute `/status` dentro de Claude Code. En la pestaña **Status**, la línea `Setting sources` muestra `Enterprise managed settings` seguida de la fuente entre paréntesis, una de `(remote)`, `(plist)`, `(HKLM)`, `(HKCU)`, o `(file)`. Consulte [Verificar configuración activa](/docs/es/settings#verify-active-settings).

Comparta estos recursos para ayudar a los desarrolladores a comenzar:

* [Quickstart](/docs/es/quickstart): recorrido de primera sesión desde la instalación hasta trabajar con un proyecto
* [Common workflows](/docs/es/common-workflows): patrones para tareas cotidianas como revisión de código, refactorización y depuración
* [Claude 101](https://anthropic.skilljar.com/claude-101) y [Claude Code in Action](https://anthropic.skilljar.com/claude-code-in-action): cursos de Anthropic Academy a su propio ritmo

Para problemas de inicio de sesión, dirija a los desarrolladores a [solución de problemas de autenticación](/docs/es/troubleshoot-install#login-and-authentication). Las correcciones más comunes son:

* Ejecutar `/logout` luego `/login` para cambiar de cuenta
* Ejecutar `claude update` si falta la opción de autenticación empresarial
* Reiniciar la terminal después de actualizar

Si un desarrollador ve "You haven't been added to your organization yet," su asiento no incluye acceso a Claude Code y debe actualizarse en la consola de administración.

<h2 id="next-steps">
  Próximos pasos
</h2>

Con el proveedor y el mecanismo de entrega elegidos, continúe con la configuración detallada:

* [Server-managed settings](/docs/es/server-managed-settings): entregar política administrada desde la consola de administración de Claude
* [Settings reference](/docs/es/settings): cada clave de configuración, ubicación de archivo y regla de precedencia
* [Monorepos and large repos](/docs/es/large-codebases): patrones de configuración por directorio para organizaciones que implementan en un monorepo
* [Amazon Bedrock](/docs/es/amazon-bedrock), [Google Cloud's Agent Platform](/docs/es/google-vertex-ai), [Microsoft Foundry](/docs/es/microsoft-foundry): implementación específica del proveedor
* [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide): SSO, SCIM, gestión de asientos y guía de implementación
