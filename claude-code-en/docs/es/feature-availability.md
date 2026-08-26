> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Disponibilidad de características

> Compare qué características de Claude Code están disponibles en los planes de suscripción de Anthropic, la Consola de Anthropic, Amazon Bedrock, Claude Platform en AWS, Google Cloud's Agent Platform y Microsoft Foundry.

El CLI de Claude Code y todo lo que se ejecuta localmente funciona de manera idéntica en cada proveedor. Para obtener instrucciones de configuración por proveedor, consulte la [descripción general de implementación empresarial](/docs/es/third-party-integrations). Para ir directamente a lo que falta en su proveedor, consulte las pestañas de [resumen por proveedor](#summary-by-provider).

En las tablas a continuación, ✓ significa disponible, ✗ significa no disponible, y "Ver nota" enlaza a una nota al pie para soporte parcial. Un calificador después de ✓ reduce la disponibilidad a ese subconjunto, y "Habilitado por administrador" significa que la característica está desactivada hasta que un administrador de la organización la active.

<h2 id="availability-by-model-provider">
  Disponibilidad por proveedor de modelo
</h2>

La forma en que se autentica determina a qué características puede acceder Claude Code. Para una lista única de lo que falta en su proveedor, consulte las pestañas de [resumen por proveedor](#summary-by-provider). Para encontrar su columna en las tablas:

* **Suscripción a Claude**: inicia sesión con una cuenta de claude.ai en el plan Pro, Max, Team o Enterprise
* **Consola de Anthropic**: se autentica con una clave API de Anthropic
* **Amazon Bedrock**: utiliza modelos de Claude del catálogo de modelos de Bedrock y establece `CLAUDE_CODE_USE_BEDROCK`. El [punto final de Mantle](/docs/es/amazon-bedrock#use-the-mantle-endpoint) (`CLAUDE_CODE_USE_MANTLE`) está cubierto por esta columna
* **Claude Platform en AWS**: compró Claude a través de AWS Marketplace pero llama a la API de Anthropic, y establece `CLAUDE_CODE_USE_ANTHROPIC_AWS`
* **Google Cloud's Agent Platform**: operado por Google; establece `CLAUDE_CODE_USE_VERTEX`
* **Microsoft Foundry**: operado por Anthropic en Azure; establece `CLAUDE_CODE_USE_FOUNDRY`

<h3 id="features-available-on-every-provider">
  Características disponibles en cada proveedor
</h3>

Estas funcionan en cada proveedor:

* [CLI](/docs/es/quickstart) y [Agent SDK](/docs/es/agent-sdk/overview)
* Extensiones de [VS Code](/docs/es/vs-code) y [JetBrains](/docs/es/jetbrains)
* [Subagentes](/docs/es/sub-agents), [hooks](/docs/es/hooks-guide), [comandos](/docs/es/commands) y [skills](/docs/es/skills)
* Memoria [CLAUDE.md](/docs/es/memory), [plugins](/docs/es/plugins) y [servidores MCP](/docs/es/mcp)
* [Checkpoints](/docs/es/checkpointing), [sandboxing](/docs/es/sandboxing) y [Workflows](/docs/es/workflows)
* Métricas [OpenTelemetry](/docs/es/monitoring-usage) y el [archivo de configuración administrado](/docs/es/settings#settings-files)

Tres de estos tienen diferencias específicas del proveedor:

* **Servidores MCP**: los [conectores de claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai) se cargan solo cuando su suscripción a claude.ai es el método de autenticación activo, y la [búsqueda de herramientas](/docs/es/mcp#configure-tool-search) está desactivada de forma predeterminada en Google Cloud's Agent Platform y cuando `ANTHROPIC_BASE_URL` apunta a un host que no es de primera parte
* **Subagentes**: el [subagente Explore integrado](/docs/es/sub-agents#built-in-subagents) limita su modelo heredado a Opus en la API de Claude, e hereda el modelo de la conversación principal directamente en cualquier otro proveedor, incluido Claude Platform en AWS
* **[Comandos](/docs/es/commands#all-commands)**: `/design-sync` y `/radio` no están disponibles en Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry y Claude Platform en AWS, y `/voice` requiere una cuenta de claude.ai

<h3 id="features-that-require-a-claude-subscription">
  Características que requieren una suscripción a Claude
</h3>

Estas requieren iniciar sesión con una cuenta de claude.ai y no son accesibles con una clave API de la Consola de Anthropic o desde un proveedor de terceros:

* [Claude Code en la web](/docs/es/claude-code-on-the-web), Claude Code en dispositivos móviles y [Claude Code en Slack](/docs/es/slack)
* [Claude Code Desktop](/docs/es/desktop)
* [Routines](/docs/es/routines) (`/schedule`)
* [Ultraplan](/docs/es/ultraplan) y [Ultrareview](/docs/es/ultrareview)
* [Code Review](/docs/es/code-review): planes Team y Enterprise
* [Remote Control](/docs/es/remote-control)
* [Extensión de Chrome](/docs/es/chrome)
* [Computer use](/docs/es/computer-use): planes Pro y Max
* [Artifacts](/docs/es/artifacts): planes Pro, Max, Team y Enterprise
* [Voice dictation](/docs/es/voice-dictation)

Desktop es la excepción parcial: el [enrutamiento de puerta de enlace se puede configurar en la aplicación o por un administrador](/docs/es/llm-gateway-connect#desktop-app), las implementaciones empresariales pueden enrutar Desktop a Google Cloud's Agent Platform o a un proveedor de puerta de enlace a través de [configuración administrada](https://claude.com/docs/third-party/claude-desktop/configuration), y [Claude Desktop en 3P](https://claude.com/docs/third-party/claude-desktop/overview) ejecuta la pestaña de Code en Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry o una puerta de enlace LLM autohospedada. Para la disponibilidad por plan de estas características, consulte [Disponibilidad por plan de suscripción](#availability-by-subscription-plan).

<h3 id="cli-capabilities-that-vary-by-provider">
  Capacidades de CLI que varían por proveedor
</h3>

Estas características funcionan en el CLI local pero dependen de una capacidad del lado del servidor que no todos los proveedores exponen.

<table>
  <thead>
    <tr>
      <th>Característica</th>
      <th>Suscripción a Claude</th>
      <th>Consola de Anthropic</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform en AWS</th>
      <th>Google Cloud's Agent Platform</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>[Búsqueda web](/docs/es/tools-reference#websearch-tool-behavior)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✓</td>
      <td>Ver nota <sup><a href="#fn1">1</a></sup></td>
      <td>✓</td>
    </tr>

    <tr>
      <td>[Fast mode](/docs/es/fast-mode)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Auto mode](/docs/es/auto-mode-config)</td>
      <td>✓</td>
      <td>✓</td>
      <td>Ver nota <sup><a href="#fn2">2</a></sup></td>
      <td>✓</td>
      <td>Ver nota <sup><a href="#fn2">2</a></sup></td>
      <td>Ver nota <sup><a href="#fn2">2</a></sup></td>
    </tr>

    <tr>
      <td>[Advisor](/docs/es/advisor)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Channels](/docs/es/channels)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[`/loop` tareas programadas](/docs/es/scheduled-tasks)</td>
      <td>✓</td>
      <td>✓</td>
      <td>Ver nota <sup><a href="#fn3">3</a></sup></td>
      <td>Ver nota <sup><a href="#fn3">3</a></sup></td>
      <td>Ver nota <sup><a href="#fn3">3</a></sup></td>
      <td>Ver nota <sup><a href="#fn3">3</a></sup></td>
    </tr>

    <tr>
      <td>[GitHub Actions](/docs/es/github-actions) y [GitLab CI/CD](/docs/es/gitlab-ci-cd)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
    </tr>
  </tbody>
</table>

<h3 id="admin-and-analytics">
  Admin y análisis
</h3>

Controles a nivel de organización y visibilidad de uso.

<table>
  <thead>
    <tr>
      <th>Característica</th>
      <th>Suscripción a Claude</th>
      <th>Consola de Anthropic</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform en AWS</th>
      <th>Google Cloud's Agent Platform</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>[Panel de análisis y API](/docs/es/analytics)</td>
      <td>✓ (panel: Team y Enterprise; API: Enterprise)</td>
      <td>✓ <sup><a href="#fn5">5</a></sup></td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Configuración administrada por servidor](/docs/es/server-managed-settings)</td>
      <td>✓ (Team y Enterprise)</td>
      <td>✓ (Team y Enterprise)</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Zero Data Retention](/docs/es/zero-data-retention)</td>
      <td>✓ (cuentas Enterprise calificadas)</td>
      <td>✓ (cuentas calificadas)</td>
      <td>Ver nota <sup><a href="#fn4">4</a></sup></td>
      <td>✓ (cuentas calificadas)</td>
      <td>Ver nota <sup><a href="#fn4">4</a></sup></td>
      <td>Ver nota <sup><a href="#fn4">4</a></sup></td>
    </tr>
  </tbody>
</table>

<span id="fn1" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>1</sup> En Google Cloud's Agent Platform, la búsqueda web está disponible para modelos Claude 4 y posteriores.<br />
<span id="fn2" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>2</sup> En estos proveedores, el modo automático admite solo Claude Sonnet 5, Opus 4.7 y Opus 4.8. Consulte [Configuración de Auto mode](/docs/es/auto-mode-config). En v2.1.158 a v2.1.206, el modo automático en estos proveedores también requería establecer `CLAUDE_CODE_ENABLE_AUTO_MODE=1`; v2.1.207 eliminó el requisito.<br />
<span id="fn3" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>3</sup> Los intervalos explícitos como `/loop every 2 hours` funcionan en cada proveedor. En Amazon Bedrock, Claude Platform en AWS, Google Cloud's Agent Platform y Microsoft Foundry, `/loop` no puede elegir su propio intervalo ni proporcionar el mensaje de mantenimiento predeterminado, por lo que un mensaje sin intervalo se ejecuta cada 10 minutos, y `/loop` sin argumentos muestra el mensaje de uso. Consulte [Tareas programadas](/docs/es/scheduled-tasks).<br />
<span id="fn4" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>4</sup> Sujeto a su acuerdo con el proveedor de nube.<br />
<span id="fn5" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>5</sup> Panel y API solamente. [Métricas de contribución](/docs/es/analytics#enable-contribution-metrics) requiere una organización Team o Enterprise de claude.ai.

<Note>
  Si se autentica a través de una [puerta de enlace LLM](/docs/es/llm-gateway), la disponibilidad de características coincide con el proveedor subyacente al que la puerta de enlace reenvía. Algunas características solo de Anthropic, como [Advisor](/docs/es/advisor), funcionan solo si la puerta de enlace reenvía solicitudes intactas a la API de Anthropic.
</Note>

<h3 id="summary-by-provider">
  Resumen por proveedor
</h3>

Cada pestaña enumera lo que no está disponible o tiene soporte parcial en ese proveedor, con alternativas donde exista una. Todo lo que no aparece en la lista funciona igual que en una suscripción a Claude, aparte de las [diferencias específicas del proveedor](#features-available-on-every-provider) señaladas anteriormente. En Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry y Claude Platform en AWS, el informe de errores y la telemetría a Anthropic están desactivados de forma predeterminada. Consulte [comportamientos predeterminados por proveedor de API](/docs/es/data-usage#default-behaviors-by-api-provider) para ver qué tráfico aún llega a Anthropic y cómo optar por no participar.

<Tabs>
  <Tab title="Amazon Bedrock">
    **No disponible:** todas las [características que requieren una suscripción a Claude](#features-that-require-a-claude-subscription), más [búsqueda web](/docs/es/tools-reference#websearch-tool-behavior), [fast mode](/docs/es/fast-mode), [Advisor](/docs/es/advisor), [Channels](/docs/es/channels), el [panel de análisis](/docs/es/analytics), [configuración administrada por servidor](/docs/es/server-managed-settings) y los [comandos `/design-sync` y `/radio`](/docs/es/commands#all-commands).

    **Soporte parcial:**

    * [Desktop](/docs/es/desktop): solo a través de [Claude Desktop en 3P](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Auto mode](/docs/es/auto-mode-config): solo Sonnet 5, Opus 4.7 y Opus 4.8
    * [`/loop`](/docs/es/scheduled-tasks): solo intervalos explícitos
    * [Zero Data Retention](/docs/es/zero-data-retention): sujeto a su acuerdo de AWS

    **Alternativas:** para programación, use [`/loop`](/docs/es/scheduled-tasks) con un intervalo explícito en lugar de `/schedule`. Para sesiones en la nube, use [GitHub Actions](/docs/es/github-actions) o [GitLab CI/CD](/docs/es/gitlab-ci-cd). Para búsquedas web, use la [herramienta WebFetch](/docs/es/tools-reference#webfetch-tool-behavior) con una URL específica.
  </Tab>

  <Tab title="Claude Platform en AWS">
    **No disponible:** todas las [características que requieren una suscripción a Claude](#features-that-require-a-claude-subscription), más [fast mode](/docs/es/fast-mode), [Advisor](/docs/es/advisor), [Channels](/docs/es/channels), el [panel de análisis](/docs/es/analytics), [configuración administrada por servidor](/docs/es/server-managed-settings) y los [comandos `/design-sync` y `/radio`](/docs/es/commands#all-commands).

    **Disponible donde Amazon Bedrock no lo es:** [búsqueda web](/docs/es/tools-reference#websearch-tool-behavior).

    **Soporte parcial:**

    * [`/loop`](/docs/es/scheduled-tasks): solo intervalos explícitos

    **Alternativas:** para programación, use [`/loop`](/docs/es/scheduled-tasks) con un intervalo explícito en lugar de `/schedule`. Para sesiones en la nube, use [GitHub Actions](/docs/es/github-actions) o [GitLab CI/CD](/docs/es/gitlab-ci-cd).
  </Tab>

  <Tab title="Google Cloud's Agent Platform">
    **No disponible:** todas las [características que requieren una suscripción a Claude](#features-that-require-a-claude-subscription), más [fast mode](/docs/es/fast-mode), [Advisor](/docs/es/advisor), [Channels](/docs/es/channels), el [panel de análisis](/docs/es/analytics), [configuración administrada por servidor](/docs/es/server-managed-settings) y los [comandos `/design-sync` y `/radio`](/docs/es/commands#all-commands).

    **Soporte parcial:**

    * [Desktop](/docs/es/desktop): a través de [configuración administrada](https://claude.com/docs/third-party/claude-desktop/configuration) o [Claude Desktop en 3P](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Búsqueda web](/docs/es/tools-reference#websearch-tool-behavior): modelos Claude 4 y posteriores
    * [Auto mode](/docs/es/auto-mode-config): solo Sonnet 5, Opus 4.7 y Opus 4.8
    * [`/loop`](/docs/es/scheduled-tasks): solo intervalos explícitos
    * [Zero Data Retention](/docs/es/zero-data-retention): sujeto a su acuerdo de Google Cloud

    **Alternativas:** para programación, use [`/loop`](/docs/es/scheduled-tasks) con un intervalo explícito en lugar de `/schedule`. Para sesiones en la nube, use [GitHub Actions](/docs/es/github-actions) o [GitLab CI/CD](/docs/es/gitlab-ci-cd).
  </Tab>

  <Tab title="Microsoft Foundry">
    **No disponible:** todas las [características que requieren una suscripción a Claude](#features-that-require-a-claude-subscription), más [fast mode](/docs/es/fast-mode), [Advisor](/docs/es/advisor), [Channels](/docs/es/channels), [GitHub Actions](/docs/es/github-actions) y [GitLab CI/CD](/docs/es/gitlab-ci-cd), el [panel de análisis](/docs/es/analytics), [configuración administrada por servidor](/docs/es/server-managed-settings) y los [comandos `/design-sync` y `/radio`](/docs/es/commands#all-commands).

    **Soporte parcial:**

    * [Desktop](/docs/es/desktop): solo a través de [Claude Desktop en 3P](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Auto mode](/docs/es/auto-mode-config): solo Sonnet 5, Opus 4.7 y Opus 4.8
    * [`/loop`](/docs/es/scheduled-tasks): solo intervalos explícitos
    * [Zero Data Retention](/docs/es/zero-data-retention): sujeto a su acuerdo de Azure

    **Alternativas:** para programación, use [`/loop`](/docs/es/scheduled-tasks) con un intervalo explícito en lugar de `/schedule`.
  </Tab>

  <Tab title="Consola de Anthropic">
    **No disponible:** todas las [características que requieren una suscripción a Claude](#features-that-require-a-claude-subscription).

    Todo en [Capacidades de CLI que varían por proveedor](#cli-capabilities-that-vary-by-provider) está disponible, así como [configuración administrada por servidor](/docs/es/server-managed-settings) cuando la clave API pertenece a una organización Team o Enterprise.
  </Tab>
</Tabs>

<h2 id="availability-by-subscription-plan">
  Disponibilidad por plan de suscripción
</h2>

Si se autentica a través de Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry o una clave API de la Consola de Anthropic, esta sección no se aplica a usted. Cuando inicia sesión con una cuenta de claude.ai, su plan determina cuál de las características a continuación está disponible.

| Característica                                                              | Pro | Max | Team                         | Enterprise                        |
| :-------------------------------------------------------------------------- | :-- | :-- | :--------------------------- | :-------------------------------- |
| [Claude Code en la web](/docs/es/claude-code-on-the-web)                         | ✓   | ✓   | ✓                            | ✓ <sup><a href="#fn6">6</a></sup> |
| [Routines](/docs/es/routines)                                                    | ✓   | ✓   | ✓                            | ✓                                 |
| [Remote Control](/docs/es/remote-control)                                        | ✓   | ✓   | Habilitado por administrador | Habilitado por administrador      |
| [Channels](/docs/es/channels)                                                    | ✓   | ✓   | Habilitado por administrador | Habilitado por administrador      |
| [Computer use](/docs/es/computer-use)                                            | ✓   | ✓   | ✗                            | ✗                                 |
| Dispatch ([Desktop](/docs/es/desktop#sessions-from-dispatch))                    | ✓   | ✓   | ✗                            | ✗                                 |
| [Code Review](/docs/es/code-review)                                              | ✗   | ✗   | ✓                            | ✓                                 |
| [Artifacts](/docs/es/artifacts)                                                  | ✓   | ✓   | ✓                            | Habilitado por administrador      |
| [Panel de análisis y métricas de contribución](/docs/es/analytics)               | ✗   | ✗   | ✓                            | ✓                                 |
| [API de análisis empresarial](/docs/es/analytics#access-data-programmatically)   | ✗   | ✗   | ✗                            | ✓                                 |
| [Configuración administrada por servidor](/docs/es/server-managed-settings)      | ✗   | ✗   | ✓                            | ✓                                 |
| [SSO](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) | ✗   | ✗   | ✓                            | ✓                                 |
| SCIM                                                                        | ✗   | ✗   | ✗                            | ✓                                 |
| [API de cumplimiento](https://platform.claude.com/docs/en/api/compliance)   | ✗   | ✗   | ✗                            | ✓                                 |
| [Zero Data Retention](/docs/es/zero-data-retention)                              | ✗   | ✗   | ✗                            | ✓ <sup><a href="#fn7">7</a></sup> |

<span id="fn6" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>6</sup> En Enterprise, requiere un asiento premium o un asiento de Chat + Claude Code. Consulte [Claude Code en la web](/docs/es/claude-code-on-the-web).<br />
<span id="fn7" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>7</sup> No incluido en el plan Enterprise estándar. Requiere habilitación separada por Anthropic para cuentas calificadas. Consulte [Zero Data Retention](/docs/es/zero-data-retention).

Para precios y la comparación completa de planes, consulte [Planes Team](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) y [Planes Enterprise](https://support.claude.com/en/articles/9797531-what-is-the-enterprise-plan).

<h2 id="model-availability">
  Disponibilidad de modelos
</h2>

Para ver qué modelos de Claude y tamaños de ventana de contexto están disponibles por proveedor y región, consulte [Configuración de modelos](/docs/es/model-config) y la [descripción general de modelos](https://platform.claude.com/docs/en/about-claude/models/overview). Vision, entrada de PDF y pensamiento extendido son capacidades de modelo en lugar de características de Claude Code y funcionan en cada proveedor que ofrece el modelo. [Prompt caching](/docs/es/prompt-caching) funciona de la misma manera en la mayoría de proveedores; en Amazon Bedrock, el soporte varía según el modelo.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Descripción general de implementación empresarial](/docs/es/third-party-integrations): compare autenticación, facturación y regiones entre proveedores
* Guías de configuración de proveedores: [Amazon Bedrock](/docs/es/amazon-bedrock), [Claude Platform en AWS](/docs/es/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/es/google-vertex-ai), [Microsoft Foundry](/docs/es/microsoft-foundry)
* [Plataformas e integraciones](/docs/es/platforms): dónde se ejecuta Claude Code, incluidos CLI, Desktop, extensiones de IDE, web, dispositivos móviles e CI/CD
