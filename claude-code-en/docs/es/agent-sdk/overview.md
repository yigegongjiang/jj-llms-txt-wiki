> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Descripción general del Agent SDK

> Construya agentes de IA en producción con Claude Code como una biblioteca

Un agente es una aplicación que completa una tarea planificando sus propios pasos y llamando a herramientas que leen archivos, ejecutan comandos o editan código. El Agent SDK le proporciona las mismas herramientas, [bucle de agente](/docs/es/agent-sdk/agent-loop), y gestión de contexto que potencian Claude Code, programable en Python y TypeScript.

<h2 id="compare-the-agent-sdk-to-other-claude-tools">
  Comparar el Agent SDK con otras herramientas de Claude
</h2>

El Agent SDK, la CLI, el Client SDK y los Managed Agents difieren en quién ejecuta el agente, qué viene integrado y cómo acceder a él. Encuentre la fila que coincida con cómo desea construir y ejecutar el suyo.

| Desea                                                                                                        | Utilice                                                                           | Lo que obtiene                                                                                                                                                                                                                                                                                                                                                                                                       |
| ------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Integrar el agente de Claude Code en su propia aplicación Python o TypeScript, en un proceso que usted opera | **Agent SDK**                                                                     | Una biblioteca que ejecuta el binario de Claude Code, con las [capacidades](#capabilities) de Claude Code, como herramientas integradas, permisos, sesiones y hooks.                                                                                                                                                                                                                                                 |
| Realizar desarrollo interactivo o ejecutar tareas puntuales desde una terminal                               | [**Claude Code CLI**](/docs/es/overview)                                               | La interfaz de terminal, construida para uso interactivo diario.                                                                                                                                                                                                                                                                                                                                                     |
| Llamar a la API de Claude directamente desde su propio código                                                | [**Client SDK**](https://platform.claude.com/docs/en/cli-sdks-libraries/overview) | Acceso directo a la API de Claude desde cualquiera de los lenguajes del Client SDK. Usted escribe el bucle de herramientas usted mismo, o deja que el [ejecutor de herramientas](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-runner) beta del Client SDK lo impulse.                                                                                                                          |
| Tener que Anthropic aloje el agente, configurado a través de la API de Claude                                | [**Managed Agents**](https://platform.claude.com/docs/en/managed-agents/overview) | Un arnés de agente alojado que ejecuta el bucle del agente, con sesiones en un sandbox en la nube administrado por Anthropic o un [sandbox autohospedado](https://platform.claude.com/docs/en/managed-agents/self-hosted-sandboxes) en su propia infraestructura. Úselo desde el [SDK para su lenguaje](https://platform.claude.com/docs/en/managed-agents/quickstart#install-the-sdk), la CLI `ant`, o la API REST. |

Para impulsar el mismo bucle de agente desde un lenguaje distinto a Python o TypeScript, [ejecute la CLI como un subproceso](/docs/es/headless) con la bandera `-p` y `--output-format json`.

<h2 id="capabilities">
  Capacidades
</h2>

Estas capacidades de Claude Code están disponibles en el SDK:

| Capacidad                  | Qué hace                                                                                           | Más información                                                                                                                                                                                                  |
| -------------------------- | -------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Herramientas integradas    | Leer, escribir, editar archivos, ejecutar comandos y buscar en la web                              | [Referencia de herramientas](/docs/es/tools-reference)                                                                                                                                                                |
| Hooks                      | Ejecutar código personalizado en puntos clave del ciclo de vida del agente                         | [Hooks](/docs/es/agent-sdk/hooks)                                                                                                                                                                                     |
| Subagentes                 | Generar agentes especializados para subtareas enfocadas                                            | [Subagentes](/docs/es/agent-sdk/subagents)                                                                                                                                                                            |
| MCP                        | Conectar herramientas y fuentes de datos externas a través del Protocolo de Contexto del Modelo    | [MCP](/docs/es/agent-sdk/mcp)                                                                                                                                                                                         |
| Permisos                   | Controlar qué herramientas se ejecutan automáticamente, cuáles necesitan aprobación                | [Permisos](/docs/es/agent-sdk/permissions)                                                                                                                                                                            |
| Sesiones                   | Mantener contexto en múltiples intercambios, reanudar o dividir más tarde                          | [Sesiones](/docs/es/agent-sdk/sessions)                                                                                                                                                                               |
| Skills, comandos y memoria | Cargar automáticamente desde `.claude/` de su proyecto y desde `~/.claude/`, igual que Claude Code | [Skills](/docs/es/agent-sdk/skills), [Comandos](/docs/es/agent-sdk/skills#commands-in-agent-sdk-sessions), [Memoria](/docs/es/agent-sdk/modifying-system-prompts), [Carga de configuración](/docs/es/agent-sdk/claude-code-features) |
| Plugins                    | Empaquetar skills, agentes, hooks y servidores MCP, y cargarlos por ruta local                     | [Plugins](/docs/es/agent-sdk/plugins)                                                                                                                                                                                 |

<h2 id="get-started">
  Comenzar
</h2>

Siga el [Quickstart](/docs/es/agent-sdk/quickstart) para instalar el SDK, establecer su clave de API y construir su primer agente, uno que encuentre y corrija errores en código existente.

<Note>
  A menos que haya sido previamente aprobado, Anthropic no permite que desarrolladores de terceros ofrezcan inicio de sesión en claude.ai o límites de velocidad para sus productos, incluidos los agentes construidos en el Agent SDK de Claude. En su lugar, utilice los métodos de autenticación de clave de API descritos en el [Quickstart](/docs/es/agent-sdk/quickstart).
</Note>

<h2 id="changelog">
  Registro de cambios
</h2>

Vea el registro de cambios completo para actualizaciones del SDK, correcciones de errores y nuevas características:

* **TypeScript SDK**: [ver CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md)
* **Python SDK**: [ver CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md)

<h2 id="report-bugs">
  Reportar errores
</h2>

Si encuentra errores o problemas con el Agent SDK:

* **TypeScript SDK**: [reportar problemas en GitHub](https://github.com/anthropics/claude-agent-sdk-typescript/issues)
* **Python SDK**: [reportar problemas en GitHub](https://github.com/anthropics/claude-agent-sdk-python/issues)

<h2 id="branding-guidelines">
  Directrices de marca
</h2>

Para socios que integran el Claude Agent SDK, el uso de la marca Claude es opcional. Al hacer referencia a Claude en su producto:

**Permitido:**

* "Claude Agent", preferido para menús desplegables
* "Claude", cuando ya está dentro de un menú etiquetado como "Agents"
* "\{YourAgentName} Powered by Claude", si tiene un nombre de agente existente

**No permitido:**

* "Claude Code" o "Claude Code Agent"
* Arte ASCII de marca Claude Code o elementos visuales que imiten Claude Code

Su producto debe mantener su propia marca y no parecer ser Claude Code o ningún producto de Anthropic. Para preguntas sobre cumplimiento de marca, póngase en contacto con el [equipo de ventas](https://www.anthropic.com/contact-sales) de Anthropic.

<h2 id="license-and-terms">
  Licencia y términos
</h2>

El uso del Claude Agent SDK se rige por los [Términos de Servicio Comerciales de Anthropic](https://www.anthropic.com/legal/commercial-terms), incluso cuando lo utiliza para potenciar productos y servicios que pone a disposición de sus propios clientes y usuarios finales, excepto en la medida en que un componente específico o dependencia esté cubierto por una licencia diferente como se indica en el archivo LICENSE de ese componente.

<h2 id="next-steps">
  Próximos pasos
</h2>

Estos recursos cubren detalles técnicos más profundos y proyectos de ejemplo para construir con el Agent SDK.

* [Inicio rápido](/docs/es/agent-sdk/quickstart): construya su primer agente que encuentre y corrija errores
* [Guía de migración](/docs/es/agent-sdk/migration-guide): migre desde los paquetes del Claude Code SDK al Agent SDK
* [Bucle de agente](/docs/es/agent-sdk/agent-loop): cómo Claude planifica, llama herramientas y decide cuándo se completa una tarea
* [Agentes de ejemplo](https://github.com/anthropics/claude-agent-sdk-demos): aplicaciones de demostración para desarrollo local
* [SDK de TypeScript](/docs/es/agent-sdk/typescript): referencia completa de API de TypeScript y ejemplos
* [SDK de Python](/docs/es/agent-sdk/python): referencia completa de API de Python y ejemplos
* [Diseño del arnés de agente](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code): cómo el equipo de Claude Code utiliza flujos de trabajo dinámicos para orquestar muchos subagentes a la vez
