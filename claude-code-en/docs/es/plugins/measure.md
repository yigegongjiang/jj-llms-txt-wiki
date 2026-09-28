> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Medir el costo y el uso del plugin

> Mida el costo de tokens de un plugin de Claude Code, descubra si la gente aún lo usa y seleccione los eventos de telemetría para preguntas de plugins en toda la organización.

Cada sesión en la que un plugin está habilitado incluye los nombres y descripciones de sus skills, agentes y comandos en el contexto de Claude, y esos tokens se cuentan contra el uso del usuario independientemente de si el plugin se utiliza o no. Esta página muestra cómo ver ese número para un plugin, cómo reducirlo si mantiene el plugin, y dónde aparece el uso para que pueda saber si un plugin aún se está utilizando.

Esta página es para autores y mantenedores de plugins. Si administra Claude Code para una organización, [Medir en toda una flota](#measure-across-a-fleet) cubre las mismas preguntas en cada máquina.

<Note>
  Estos casos se cubren en otras páginas:

  * **Probar qué tan confiablemente el plugin cambia el comportamiento de Claude**: consulte [Test plugins with evals](/docs/es/plugin-evals)
  * **Reducir el contexto de su propia sesión**: consulte [Manage installed plugins](/docs/es/plugins/install#manage-installed-plugins) y la página [context window](/docs/es/context-window)
</Note>

Comience con [Medir qué cuesta un plugin](#measure-what-a-plugin-costs).

<h2 id="measure-what-a-plugin-costs">
  Medir qué cuesta un plugin
</h2>

Para ver qué agrega un plugin al contexto de Claude, ejecute [`claude plugin details`](/docs/es/plugins/cli-reference#plugin-details) con el nombre del plugin. Lo ejecuta en su shell, no en el prompt de una sesión de Claude Code en ejecución. El plugin debe estar cargado: instalado, en un directorio de skills, o pasado con `--plugin-dir` en el mismo comando, como en `claude --plugin-dir ./formatter plugin details formatter`.

Este ejemplo lee un plugin instalado llamado `formatter` que tiene dos skills, un comando, un agente, un hook y un servidor MCP:

```bash theme={null}
claude plugin details formatter
```

```text theme={null}
formatter 1.0.0
  Description: Formats and lints code on save
  Source: formatter@my-marketplace

Component inventory
  Skills (3)  format-all, format-code, lint-fix
  Agents (1)  style-reviewer
  Hooks (1)  PostToolUse  (harness-only — no model context cost)
  MCP servers (1)  formatter-tools  (tool schemas resolved at runtime; not counted)
  LSP servers (0)

Projected token cost
  Always-on:   ~146 tok   added to every session

Per-component (rounded)
  component       always-on  on-invoke
  format-code           ~40        ~30
  lint-fix              ~50        ~30
  style-reviewer        ~40        ~40
  format-all           < 20        ~30

  On-invoke cost is paid each time a skill or agent fires.
  Token counts are estimates and may differ from actual usage.
```

Cada parte de la salida responde una pregunta diferente:

* **Component inventory**: lo que Claude Code encontró en el plugin. Los comandos se cuentan con skills, por lo que `format-all` aparece bajo `Skills`. Los hooks y servidores MCP no obtienen estimación de costo ni fila por componente; para ver qué agregan las herramientas MCP de un plugin, ejecute `/context` en una sesión con el plugin habilitado y lea la categoría `MCP tools`.
* **Always-on**: los tokens que los nombres y descripciones de los skills, agentes y comandos del plugin agregan a cada sesión donde el plugin está habilitado, independientemente de si algo se ejecuta o no. Este es el número que cada usuario lleva, y el que debe reducir.
* **Per-component**: cada fila divide un skill, agente o comando en su parte siempre activa y su costo de invocación, que es el cuerpo que se carga solo cuando ese componente se ejecuta. Use la columna always-on para encontrar qué componente contribuye más.

<h3 id="lower-the-always-on-figure">
  Reducir la cifra always-on
</h3>

Si mantiene el plugin, estos cambios reducen lo que agrega a cada sesión. Si solo lo usa, sus opciones son deshabilitarlo o desinstalarlo; consulte [Manage installed plugins](/docs/es/plugins/install#manage-installed-plugins).

La cifra always-on cuenta el nombre de cada componente más su `description` y frontmatter `when_to_use`. Para reducirla:

* Acorte las descripciones de skills y agentes.
* Divida un plugin grande para que los usuarios instalen solo los componentes que necesitan.

La descripción de un skill también es lo que Claude compara con una solicitud, por lo que una más corta puede evitar que el skill se active. Después de recortar descripciones, verifique la activación con un [calificador `tool_used: Skill`](/docs/es/plugin-evals#create-your-first-eval-suite) en su suite de evaluación.

Para ver qué contribuye cada tipo de componente, consulte [plugin components](/docs/es/plugins/components).

<h3 id="cost-shown-to-users-before-install">
  Costo mostrado a los usuarios antes de instalar
</h3>

Los plugins en el marketplace oficial muestran su costo a los usuarios antes de instalar. En `/plugin`, cuando un usuario explora la lista de plugins de un marketplace y selecciona un plugin, el panel de detalles muestra una sección **Context cost** con una línea `Every turn:` y una línea `When invoked:`. Cuando la cifra always-on es de 2.000 tokens o más, la línea `Every turn:` aparece resaltada.

Un plugin en su propio marketplace no tiene sección **Context cost**.

<h2 id="check-whether-a-plugin-is-used">
  Verificar si se utiliza un plugin
</h2>

Claude Code no informa el uso de un plugin a su autor. El uso se registra en la máquina de cada persona que instaló el plugin, por lo que lo que puede aprender depende de su relación con esas personas:

* **Administra Claude Code para su organización**: los eventos de OpenTelemetry y la API de Analytics cuentan instalaciones y activaciones de skills en cada máquina. Consulte [Medir en toda una flota](#measure-across-a-fleet).
* **Son compañeros de equipo a los que puede preguntar**: el propio Claude Code de cada usuario les muestra si aún usan el plugin, en cuatro lugares: el [panel `/plugin`](#not-used-recently-in-/plugin), [`/skill-doctor`](#find-skills-that-never-run), [`/doctor`](#unused-plugins-in-/doctor), y [`/usage`](#usage-share-in-/usage). Los cuatro son comandos que el usuario ejecuta en el prompt de Claude Code en una sesión en su propia máquina.
* **Ninguno**: no tiene señal de uso de Claude Code para ese plugin.

<h3 id="not-used-recently-in-/plugin">
  No utilizado recientemente en `/plugin`
</h3>

En la pestaña **Installed** de `/plugin`, un plugin que el usuario instaló desde un marketplace se mueve bajo un encabezado **Not used recently** una vez que ha estado sin usar durante al menos 14 días y 10 sesiones. Los detalles del plugin también muestran una línea `Last used:`. Para ver qué hacen los usuarios con ese encabezado y línea, consulte [Find plugins you no longer use](/docs/es/plugins/install#find-plugins-you-no-longer-use).

El encabezado **Not used recently** nunca aparece para:

* Plugins cargados con `--plugin-dir` o desde un directorio de skills
* Plugins habilitados a través de configuración administrada, o montados desde un [directorio seed](/docs/es/plugins/org#seed-containers-and-ci)
* Plugins que incluyen un tema, estilo de salida, monitor o flujo de trabajo, porque están en uso sin una invocación rastreada

El [servidor de lenguaje](/docs/es/plugins/components#lsp-servers) de un plugin se cuenta como utilizado cuando entrega diagnósticos o responde una solicitud de navegación de código, por lo que un plugin LSP cuyo servidor está activo en sus sesiones no se enumera como sin usar.

Cuando la organización del usuario establece [`strictKnownMarketplaces`](/docs/es/plugins/org#restrict-what-users-can-install), ni el encabezado ni la línea `Last used:` aparecen.

<h3 id="find-skills-that-never-run">
  Encontrar skills que nunca se ejecutan
</h3>

Ejecute `/skill-doctor` para ver qué cuesta cada uno de sus skills y con qué frecuencia se utiliza. Marca skills que están en la lista de skills de Claude pero nunca han sido invocados, incluidos skills de plugins.

En una sesión interactiva, el informe se abre en la pestaña **Stats** del administrador `/plugin`. Consulte [Find unused skills](/docs/es/skills#find-unused-skills) para ver qué cubre el informe y dónde está disponible.

<h3 id="unused-plugins-in-/doctor">
  Plugins sin usar en `/doctor`
</h3>

El chequeo `/doctor` enumera cada skill instalado por el usuario, servidor MCP y plugin, y recomienda deshabilitar los que no se utilizaron. Consulte [`/doctor` en la referencia de comandos](/docs/es/commands#all-commands).

<h3 id="usage-share-in-/usage">
  Compartir uso en `/usage`
</h3>

En un plan Pro, Max, Team o Enterprise, el desglose `/usage` atribuye el uso reciente a skills, subagentes, plugins y servidores MCP como una parte del total. Consulte [Using the `/usage` command](/docs/es/costs#using-the-/usage-command).

<h2 id="measure-across-a-fleet">
  Medir en toda una flota
</h2>

Si administra Claude Code para una organización, puede medir el costo y el uso del plugin en cada máquina desde cualquiera de estas fuentes:

* **Eventos de OpenTelemetry**: Claude Code los exporta a su propio backend una vez que [configura un exportador](/docs/es/monitoring-usage). Consulte [OpenTelemetry events for plugin installs and use](#pick-the-opentelemetry-event-for-each-question).
* **API de Analytics**: servida desde los registros de Anthropic, sin necesidad de exportador. Consulte [Query the Analytics API](#query-the-analytics-api).

<h3 id="pick-the-opentelemetry-event-for-each-question">
  Eventos de OpenTelemetry para instalaciones y uso de plugins
</h3>

Estos eventos y atributos de OpenTelemetry responden cada pregunta de plugin desde su backend:

| Pregunta                                      | Evento o atributo de OpenTelemetry                                                                                                                                    |
| :-------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Qué plugins se instalan y de dónde            | [`claude_code.plugin_installed`](/docs/es/monitoring-usage#plugin-installed-event), uno por instalación                                                                    |
| Qué plugins están activos en cuántas sesiones | [`claude_code.plugin_loaded`](/docs/es/monitoring-usage#plugin-loaded-event), uno por plugin habilitado al inicio de sesión                                                |
| Qué skills se activan y qué plugin los posee  | [`claude_code.skill_activated`](/docs/es/monitoring-usage#skill-activated-event), con `plugin.name` y `marketplace.name` para skills de plugin                             |
| Qué reportan los hooks de un plugin           | [`claude_code.hook_plugin_metrics`](/docs/es/monitoring-usage#hook-plugin-metrics-event), emitido solo para hooks en plugins del marketplace oficial                       |
| Qué cuesta un plugin en gasto de API          | `plugin.name` y `marketplace.name` en el [contador de costo](/docs/es/monitoring-usage#cost-counter), establecido cuando el skill activo o subagente pertenece a un plugin |

<h3 id="redacted-plugin-names-in-your-backend">
  Nombres de plugins redactados en su backend
</h3>

Los plugins del marketplace oficial reportan su nombre de plugin y nombre de marketplace a su backend literalmente. El nombre de todos los demás plugins se redacta u omite por defecto, incluido un plugin del propio marketplace de su organización. El [nivel de confianza](/docs/es/plugins/security#find-plugins-in-telemetry) del plugin decide cuál.

Para obtener nombres reales en algunos eventos, establezca la variable de entorno [`OTEL_LOG_TOOL_DETAILS`](/docs/es/monitoring-usage#common-configuration-variables) en `1` en las máquinas que exportan telemetría, por ejemplo en el bloque `env` de la misma [configuración administrada](/docs/es/monitoring-usage#administrator-configuration) que configura el exportador:

| Evento                                | Predeterminado                                                                                    | Con `OTEL_LOG_TOOL_DETAILS=1`                      |
| :------------------------------------ | :------------------------------------------------------------------------------------------------ | :------------------------------------------------- |
| `plugin_loaded`                       | `plugin.name` y `marketplace.name` son la cadena literal `third-party`                            | Nombres reales                                     |
| `plugin_installed`, `skill_activated` | `plugin.name` y `marketplace.name` omitidos; en `skill_activated`, `skill.name` es `custom_skill` | Nombres reales                                     |
| Contador de costo                     | `plugin.name` es `third-party`; `marketplace.name` ausente                                        | `plugin.name` real; `marketplace.name` aún ausente |

En `plugin_loaded`, `plugin_id_hash` aún identifica cada plugin por defecto, por lo que puede contar plugins de terceros distintos.

<h3 id="query-the-analytics-api">
  Consultar la API de Analytics
</h3>

En el plan Enterprise, la API de Analytics responde "qué plugins instala e invoca mi organización" desde los registros de Anthropic, sin necesidad de exportador. [`GET /v1/organizations/analytics/plugins`](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) devuelve recuentos de instalación e invocación por plugin, por día en Claude Code y Cowork, que puede agrupar por usuario, grupo RBAC o producto.

La actividad de plugin que llega a Anthropic sin un nombre de plugin aparece en una fila agregada `third-party`. [Find plugins in telemetry](/docs/es/plugins/security#find-plugins-in-telemetry) dice qué plugins reporta Claude Code por nombre.

Autentique la solicitud con una clave de API que tenga el alcance `read:analytics`, que un Propietario Principal crea como se describe en [Access data programmatically](/docs/es/analytics#access-data-programmatically).

Consulte la [referencia del endpoint](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) para los parámetros y campos de respuesta.

<h2 id="next-steps">
  Próximos pasos
</h2>

* [Test plugins with evals](/docs/es/plugin-evals): mida qué tan confiablemente el plugin dirige a Claude, no solo qué cuesta
* [Reducir la cifra always-on](#lower-the-always-on-figure): qué cambiar en el plugin para reducir su costo por turno
* [Plugin security and trust](/docs/es/plugins/security#find-plugins-in-telemetry): qué campos de telemetría llevan nombres de plugins y cuándo se redactan
* [Monitoring usage](/docs/es/monitoring-usage): la referencia completa de eventos de OpenTelemetry
