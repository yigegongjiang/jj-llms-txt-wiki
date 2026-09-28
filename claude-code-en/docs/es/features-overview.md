> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Extender Claude Code

> Comprenda cuándo usar CLAUDE.md, Skills, subagents, hooks, MCP y plugins.

Claude Code combina un modelo que razona sobre su código con [herramientas integradas](/docs/es/how-claude-code-works#tools) para operaciones de archivos, búsqueda, ejecución y acceso web. Las herramientas integradas cubren la mayoría de las tareas de codificación. Esta guía cubre la capa de extensión: características que agrega para personalizar lo que Claude sabe, conectarlo a servicios externos y automatizar flujos de trabajo.

<Note>
  Para saber cómo funciona el bucle agentico central, consulte [Cómo funciona Claude Code](/docs/es/how-claude-code-works).
</Note>

**¿Nuevo en Claude Code?** Comience con [CLAUDE.md](/docs/es/memory) para convenciones de proyecto, luego agregue otras extensiones [según surjan desencadenantes específicos](#build-your-setup-over-time).

<h2 id="overview">
  Descripción general
</h2>

Las extensiones se conectan a diferentes partes del bucle agentico:

* **[CLAUDE.md](/docs/es/memory)** agrega contexto persistente que Claude ve en cada sesión
* **[Output styles](/docs/es/output-styles)** establecen el rol, tono y formato de respuesta de Claude para cada respuesta en una sesión
* **[Skills](/docs/es/skills)** agregan conocimiento reutilizable y flujos de trabajo invocables
* **[Code intelligence](/docs/es/tools-reference#lsp-tool-behavior)** conecta Claude a un servidor de lenguaje para navegación a nivel de símbolo y errores de tipo en vivo
* **[MCP](/docs/es/mcp)** conecta Claude a servicios y herramientas externas
* **[Subagents](/docs/es/sub-agents)** ejecutan sus propios bucles en contexto aislado, devolviendo resúmenes
* **[Dynamic workflows](/docs/es/workflows)** ejecutan muchos subagentes desde un script que Claude escribe, devolviendo un resultado
* **[Cross-session messaging](/docs/es/cross-session-messaging)** permite que Claude pase un mensaje de una de sus sesiones a otra
* **[Hooks](/docs/es/hooks-guide)** ejecutan su script, solicitud HTTP, llamada a herramienta MCP, prompt o subagente cuando Claude Code alcanza un evento del ciclo de vida
* **[Plugins](/docs/es/plugins/overview)** y **[marketplaces](/docs/es/plugins/overview)** empaquetan y distribuyen estas características

[Skills](/docs/es/skills) son la extensión más flexible. Una skill es un archivo markdown que contiene conocimiento, flujos de trabajo o instrucciones. Puede invocar skills con un comando como `/deploy`, o Claude puede cargarlas automáticamente cuando sea relevante. Las skills pueden ejecutarse en su conversación actual o en un contexto aislado a través de subagents.

<h2 id="match-features-to-your-goal">
  Hacer coincidir características con su objetivo
</h2>

Las características van desde contexto siempre activo que Claude ve en cada sesión, hasta capacidades bajo demanda que usted o Claude pueden invocar, hasta automatización en segundo plano que se ejecuta en eventos específicos. La tabla a continuación muestra qué está disponible y cuándo tiene sentido cada uno.

| Característica                                                 | Qué hace                                                                                        | Cuándo usarlo                                                                                                               | Ejemplo                                                                                                                            |
| -------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| **CLAUDE.md**                                                  | Contexto persistente cargado en cada conversación                                               | Convenciones de proyecto, reglas "siempre haz X"                                                                            | "Usa pnpm, no npm. Ejecuta pruebas antes de hacer commit."                                                                         |
| **[Output style](/docs/es/output-styles)**                          | Instrucciones que establecen el rol, tono y formato de respuesta de Claude para toda una sesión | Una voz, longitud o formato que desea en cada respuesta, o Claude trabajando como algo diferente a un ingeniero de software | El estilo Concise integrado para respuestas más cortas; un estilo personalizado que responde cada pregunta con un diagrama primero |
| **Skill**                                                      | Instrucciones, conocimiento y flujos de trabajo que Claude puede usar                           | Contenido reutilizable, documentos de referencia, tareas repetibles                                                         | `/deploy` ejecuta su lista de verificación de implementación; skill de documentos API con patrones de endpoint                     |
| **Subagent**                                                   | Contexto de ejecución aislado que devuelve resultados resumidos                                 | Aislamiento de contexto, tareas paralelas, trabajadores especializados                                                      | Tarea de investigación que lee muchos archivos pero devuelve solo hallazgos clave                                                  |
| **[Dynamic workflow](/docs/es/workflows)**                          | Script que Claude escribe que ejecuta muchos subagents en segundo plano                         | Trabajo que crece más allá de un puñado de subagents, o hallazgos que desea verificar cruzadamente                          | Auditar una base de código completa, con un segundo conjunto de agentes verificando cada hallazgo                                  |
| **[Cross-session messaging](/docs/es/cross-session-messaging)**     | Claude entrega un mensaje de una de sus sesiones a otra                                         | Sesiones que ejecuta usted mismo que necesitan los hallazgos de las otras a mitad de la tarea                               | Una sesión advierte a otra que un cambio que hizo rompe lo que la otra está construyendo                                           |
| **[Code intelligence](/docs/es/tools-reference#lsp-tool-behavior)** | Navegación y diagnósticos del servidor de lenguaje                                              | Lenguajes tipados, bases de código grandes donde grep es lento o impreciso                                                  | Saltar a la definición de un símbolo en lugar de leer todo el archivo                                                              |
| **MCP**                                                        | Conectar a servicios externos                                                                   | Datos o acciones externas                                                                                                   | Consultar su base de datos, publicar en Slack, controlar un navegador                                                              |
| **Hook**                                                       | Script, solicitud HTTP, llamada a herramienta MCP, prompt o subagent desencadenado por eventos  | Automatización que debe ejecutarse en cada evento coincidente                                                               | Ejecutar ESLint después de cada edición de archivo                                                                                 |
| **[Artifact](/docs/es/artifacts)**                                  | Publicar salida de sesión como una página web privada e interactiva                             | Salida que desea ver o compartir visualmente en lugar de como texto de terminal                                             | Una línea de tiempo de incidentes que se actualiza mientras Claude investiga                                                       |

**[Plugins](/docs/es/plugins/overview)** son la capa de empaquetamiento. Un plugin agrupa skills, hooks, subagents y servidores MCP en una única unidad instalable. Las skills de plugin tienen espacios de nombres (como `/my-plugin:review`) para que múltiples plugins puedan coexistir. Use plugins cuando desee reutilizar la misma configuración en múltiples repositorios o distribuir a otros a través de un **[marketplace](/docs/es/plugins/overview)**.

<h3 id="build-your-setup-over-time">
  Construir su configuración con el tiempo
</h3>

No necesita configurar todo de antemano. Cada característica tiene un desencadenante reconocible, y la mayoría de los equipos las agregan en aproximadamente este orden:

| Desencadenante                                                                                  | Agregar                                                                                  |
| :---------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------- |
| Claude se equivoca en una convención o comando dos veces                                        | Agréguelo a [CLAUDE.md](/docs/es/memory)                                                      |
| Sigue pidiendo a Claude que sea más breve, que explique más, o que responda en el mismo formato | Establezca un [output style](/docs/es/output-styles)                                          |
| Sigue escribiendo el mismo prompt para iniciar una tarea                                        | Guárdelo como una [skill](/docs/es/skills) invocable por el usuario                           |
| Pega el mismo manual o procedimiento de varios pasos en el chat por tercera vez                 | Capturarlo como una [skill](/docs/es/skills)                                                  |
| Sigue copiando datos de una pestaña del navegador que Claude no puede ver                       | Conecte ese sistema como un [servidor MCP](/docs/es/mcp)                                      |
| Claude lee muchos archivos para encontrar dónde se define o usa un símbolo                      | Instale un [plugin de code intelligence](/docs/es/plugins/code-intelligence) para su lenguaje |
| Una tarea secundaria inunda su conversación con salida que no volverá a consultar               | Enrutarlo a través de un [subagent](/docs/es/sub-agents)                                      |
| Desea que algo suceda cada vez sin preguntar                                                    | Escriba un [hook](/docs/es/hooks-guide)                                                       |
| Un segundo repositorio necesita la misma configuración                                          | Empaquételo como un [plugin](/docs/es/plugins/overview)                                       |

Los mismos desencadenantes le dicen cuándo actualizar lo que ya tiene. Un error repetido o un comentario de revisión recurrente es una edición de CLAUDE.md, no una corrección única en el chat. Un flujo de trabajo que sigue ajustando manualmente es una skill que necesita otra revisión.

<h3 id="compare-similar-features">
  Comparar características similares
</h3>

Algunas características pueden parecer similares. Para un recorrido más profundo sobre cómo elegir entre ellas, consulte [Steering Claude Code: when to use CLAUDE.md, skills, hooks, and subagents](https://claude.com/blog/steering-claude-code-skills-hooks-rules-subagents-and-more) en el blog. Aquí se explica cómo distinguirlas.

<Tabs>
  <Tab title="Skill vs Subagent">
    Las skills y los subagents resuelven problemas diferentes:

    * **Skills** son contenido reutilizable que puede cargar en cualquier contexto
    * **Subagents** son trabajadores aislados que se ejecutan separadamente de su conversación principal

    | Aspecto                                                  | Skill                                                         | Subagent                                                                       |
    | -------------------------------------------------------- | ------------------------------------------------------------- | ------------------------------------------------------------------------------ |
    | **Qué es**                                               | Instrucciones, conocimiento o flujos de trabajo reutilizables | Trabajador aislado con su propio contexto                                      |
    | **Beneficio clave**                                      | Compartir contenido entre contextos                           | Aislamiento de contexto. El trabajo ocurre por separado, solo devuelve resumen |
    | **Impacto de [ventana de contexto](/docs/es/context-window)** | Se agrega a su ventana principal                              | Usa una ventana separada con sus propios tokens de entrada y salida            |
    | **Mejor para**                                           | Material de referencia, flujos de trabajo invocables          | Tareas que leen muchos archivos, trabajo paralelo, trabajadores especializados |

    **Las skills pueden ser de referencia o acción.** Las skills de referencia proporcionan conocimiento que Claude usa en toda su sesión (como su guía de estilo de API). Las skills de acción le dicen a Claude que haga algo específico (como `/deploy` que ejecuta su flujo de trabajo de implementación).

    **Use un subagent** cuando necesite aislamiento de contexto o cuando su ventana de contexto se esté llenando. El subagent podría leer docenas de archivos o ejecutar búsquedas extensas, pero su conversación principal solo recibe un resumen. Dado que el trabajo del subagent no consume su contexto principal, esto también es útil cuando no necesita que el trabajo intermedio permanezca visible. Los subagents personalizados pueden tener sus propias instrucciones y pueden precargar skills.

    **Pueden combinarse.** Un subagent puede precargar skills específicas (campo `skills:`). Una skill puede ejecutarse en contexto aislado usando `context: fork`. Consulte [Skills](/docs/es/skills) para obtener detalles.
  </Tab>

  <Tab title="CLAUDE.md vs Skill">
    Ambos almacenan instrucciones, pero se cargan de manera diferente y sirven propósitos diferentes.

    | Aspecto                                  | CLAUDE.md                     | Skill                                                |
    | ---------------------------------------- | ----------------------------- | ---------------------------------------------------- |
    | **Se carga**                             | Cada sesión, automáticamente  | Bajo demanda                                         |
    | **Puede incluir archivos**               | Sí, con importaciones `@path` | Sí, con importaciones `@path`                        |
    | **Puede desencadenar flujos de trabajo** | No                            | Sí, con `/<name>`                                    |
    | **Mejor para**                           | Reglas "siempre haz X"        | Material de referencia, flujos de trabajo invocables |

    **Póngalo en CLAUDE.md** si Claude siempre debe saberlo: convenciones de codificación, comandos de compilación, estructura del proyecto, reglas "nunca hagas X".

    **Póngalo en una skill** si es material de referencia que Claude necesita a veces (documentos de API, guías de estilo) o un flujo de trabajo que desencadena con `/<name>` (implementar, revisar, lanzar).

    **Regla general:** Mantenga CLAUDE.md bajo 200 líneas. Si está creciendo, mueva contenido de referencia a skills o divida en archivos [`.claude/rules/`](/docs/es/memory#organize-rules-with-claude/rules/).
  </Tab>

  <Tab title="CLAUDE.md vs Output style">
    Ambos dan a Claude instrucciones permanentes. CLAUDE.md contiene lo que Claude debe saber, y un output style establece cómo Claude responde.

    | Aspecto        | CLAUDE.md                                                     | Output style                                                                                    |
    | -------------- | ------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
    | **Contiene**   | Hechos y reglas sobre su proyecto                             | Un rol, tono y formato de respuesta                                                             |
    | **Cambio**     | Siempre cargado                                               | Uno activo a la vez; [cambie estilos](/docs/es/output-styles#change-your-output-style) cuando quiera |
    | **Mejor para** | Comandos de compilación, convenciones, reglas "nunca hagas X" | Respuestas más cortas, explicaciones junto con código, un rol no ingenieril                     |

    **Póngalo en CLAUDE.md** si es verdad del proyecto sea cual sea el estilo que esté usando: convenciones de codificación, comandos de compilación, estructura del proyecto.

    **Use un output style** si se trata de la respuesta en sí y podría querer desactivarlo nuevamente: longitud, formato, cuánto explica Claude, o un rol diferente como asistente de escritura. Claude Code incluye [estilos integrados](/docs/es/output-styles#built-in-output-styles), y puede escribir los suyos propios.

    **Se combinan.** CLAUDE.md permanece cargado sea cual sea el estilo que elija. Claude sigue ambos como instrucciones, por lo que ninguno se aplica. Para cualquier cosa que deba suceder cada vez, use un [hook](/docs/es/hooks-guide).
  </Tab>

  <Tab title="CLAUDE.md vs Rules vs Skills">
    Los tres almacenan instrucciones, pero se cargan de manera diferente:

    | Aspecto        | CLAUDE.md                                        | `.claude/rules/`                                     | Skill                                                |
    | -------------- | ------------------------------------------------ | ---------------------------------------------------- | ---------------------------------------------------- |
    | **Se carga**   | Cada sesión                                      | Cada sesión, o cuando se abren archivos coincidentes | Bajo demanda, cuando se invoca o es relevante        |
    | **Alcance**    | Proyecto completo                                | Puede estar limitado a rutas de archivo              | Específico de tarea                                  |
    | **Mejor para** | Convenciones y comandos de compilación centrales | Directrices específicas del idioma o directorio      | Material de referencia, flujos de trabajo repetibles |

    **Use CLAUDE.md** para instrucciones que cada sesión necesita: comandos de compilación, convenciones de prueba, arquitectura del proyecto.

    **Use rules** para mantener CLAUDE.md enfocado. Las rules con [frontmatter `paths`](/docs/es/memory#path-specific-rules) solo se cargan cuando Claude trabaja con archivos coincidentes, ahorrando contexto.

    **Use skills** para contenido que Claude solo necesita a veces, como documentación de API o una lista de verificación de implementación que desencadena con `/<name>`.
  </Tab>

  <Tab title="Subagent vs Dynamic workflow">
    Ambos hacen trabajo fuera de su conversación principal. Con subagents, Claude decide turno a turno qué se ejecuta a continuación. En un flujo de trabajo, el script decide:

    * **Subagents** son trabajadores que Claude genera, cada uno devolviendo un resumen a la conversación que lo generó
    * **[Dynamic workflows](/docs/es/workflows)** son scripts que Claude escribe que ejecutan muchos subagents en segundo plano y devuelven un resultado

    **Use un subagent** cuando necesite un trabajador rápido y enfocado: investigar una pregunta, verificar una afirmación, revisar un archivo. El subagent hace el trabajo y devuelve un resumen, por lo que su conversación principal se mantiene limpia. Los subagents que Claude nombró cuando los generó también pueden [enviarse mensajes entre sí](/docs/es/sub-agents#what-loads-at-startup).

    **Use un dynamic workflow** cuando un trabajo [crece más allá de un puñado de subagents](/docs/es/workflows#when-to-use-a-workflow), o cuando desea que los hallazgos se verifiquen cruzadamente antes de verlos, como una auditoría de base de código completa, una migración grande o un plan elaborado desde varios ángulos. Para iniciar uno, [pida un flujo de trabajo en su prompt](/docs/es/workflows#ask-for-a-workflow-in-your-prompt).

    **Para pasar un hallazgo de una de sus sesiones a otra**, pida a la Claude de la primera sesión que lo envíe. Claude lo entrega con [cross-session messaging](/docs/es/cross-session-messaging). [Run agents in parallel](/docs/es/agents) compara las otras formas de ejecutar más de una Claude a la vez, incluyendo sesiones que entrega y verifica más tarde.
  </Tab>

  <Tab title="MCP vs Skill">
    MCP conecta Claude a servicios externos. Las skills extienden lo que Claude sabe, incluyendo cómo usar esos servicios de manera efectiva.

    | Aspecto         | MCP                                                                    | Skill                                                                                                  |
    | --------------- | ---------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
    | **Qué es**      | Protocolo para conectar a servicios externos                           | Conocimiento, flujos de trabajo y material de referencia                                               |
    | **Proporciona** | Herramientas y acceso a datos                                          | Conocimiento, flujos de trabajo, material de referencia                                                |
    | **Ejemplos**    | Integración de Slack, consultas de base de datos, control de navegador | Lista de verificación de revisión de código, flujo de trabajo de implementación, guía de estilo de API |

    Estos resuelven problemas diferentes y funcionan bien juntos:

    **MCP** le da a Claude herramientas especialmente diseñadas para un sistema externo, con la conexión y autenticación manejadas por el servidor.

    **Skills** le dan a Claude conocimiento sobre cómo usar esas herramientas de manera efectiva, además de flujos de trabajo que puede desencadenar con `/<name>`. Una skill podría incluir el esquema de base de datos de su equipo y patrones de consulta, o un flujo de trabajo `/post-to-slack` con las reglas de formato de mensaje de su equipo.
  </Tab>

  <Tab title="Hook vs Skill">
    Claude Code ejecuta un hook en un evento del ciclo de vida; carga una skill en contexto para que Claude la aplique.

    | Aspecto               | Hook                                                                                   | Skill                                                                                        |
    | --------------------- | -------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- |
    | **Se ejecuta**        | Un comando shell, solicitud HTTP, llamada a herramienta MCP, prompt LLM o subagent     | Instrucciones que Claude lee y sigue                                                         |
    | **Desencadenado por** | [Eventos del ciclo de vida](/docs/es/hooks#hook-events) como `PostToolUse` o `SessionStart` | Usted escribiendo `/<name>`, o Claude haciendo coincidir la descripción con su tarea         |
    | **Determinismo**      | Siempre se dispara en su evento; el desencadenante está garantizado                    | Claude interpreta las instrucciones; el resultado puede variar                               |
    | **Costo de contexto** | Cero a menos que el hook devuelva salida                                               | La descripción se carga cada sesión; el contenido completo se carga cuando se usa            |
    | **Mejor para**        | Linting después de ediciones, bloquear comandos inseguros, logging, notificaciones     | Flujos de trabajo que necesitan razonamiento, material de referencia, tareas de varios pasos |

    **Use un hook** cuando la acción debe suceder de la misma manera cada vez y no necesita que Claude piense. Por ejemplo: formatear al guardar, rechazar `rm -rf /`, publicar un mensaje de Slack cuando termina una sesión.

    **Use una skill** cuando Claude debe decidir cómo aplicar los pasos, o cuando el contenido es conocimiento en lugar de un script. Por ejemplo: una lista de verificación `/release`, su guía de estilo de API, un manual de depuración.

    **Ponga guardrails en hooks.** Una instrucción como "nunca edite `.env`" en CLAUDE.md o una skill es una solicitud, no una garantía. Un hook `PreToolUse` que bloquea la edición es cumplimiento. Si una regla debe mantenerse cada vez, hágala un hook en lugar de una instrucción de prompt.

    **La salida del hook aterriza en contexto.** Un hook `PostToolUse` que ejecuta su linter alimenta resultados de vuelta como texto que Claude lee; una skill `/fix-lint` le dice a Claude cómo resolverlos.
  </Tab>
</Tabs>

<h3 id="understand-how-features-layer">
  Entender cómo se superponen las características
</h3>

Las características se pueden definir en múltiples niveles: en todo el usuario, por proyecto, a través de plugins o mediante políticas administradas. También puede anidar archivos CLAUDE.md en subdirectorios o colocar skills en paquetes específicos de un monorepo. Cuando la misma característica existe en múltiples niveles, así es como se superponen:

* **Los archivos CLAUDE.md** son aditivos: todos los niveles contribuyen contenido al contexto de Claude simultáneamente. Los archivos de su directorio de trabajo y superior se cargan al iniciar; los subdirectorios se cargan mientras trabaja en ellos. Cuando las instrucciones entran en conflicto, Claude usa el juicio para reconciliarlas. Consulte [cómo se cargan los archivos CLAUDE.md](/docs/es/memory#how-claude-md-files-load).
* **Las skills y subagents** se anulan por nombre: cuando el mismo nombre existe en múltiples niveles, una definición gana según la prioridad (administrado > usuario > proyecto para skills; administrado > bandera CLI > proyecto > usuario > plugin para subagents). Las skills de plugin tienen [espacios de nombres](/docs/es/plugins/components#skills) para evitar conflictos. Consulte [descubrimiento de skills](/docs/es/skills#resolve-skills-that-share-a-name) y [alcance de subagent](/docs/es/sub-agents#choose-the-subagent-scope).
* **Los servidores MCP** se anulan por nombre: local > proyecto > usuario. Consulte [alcance de MCP](/docs/es/mcp#scope-hierarchy-and-precedence).
* **Los hooks** se fusionan: todos los hooks registrados se disparan para sus eventos coincidentes independientemente de la fuente. Consulte [hooks](/docs/es/hooks).

<h3 id="combine-features">
  Combinar características
</h3>

Cada extensión resuelve un problema diferente: CLAUDE.md maneja contexto siempre activo, las skills manejan conocimiento bajo demanda y flujos de trabajo, MCP maneja conexiones externas, los subagents manejan aislamiento y los hooks manejan automatización. Las configuraciones reales las combinan según su flujo de trabajo.

Por ejemplo, podría usar CLAUDE.md para convenciones de proyecto, una skill para su flujo de trabajo de implementación, MCP para conectar a su base de datos y un hook para ejecutar linting después de cada edición. Cada característica maneja lo que hace mejor.

| Patrón                 | Cómo funciona                                                                                               | Ejemplo                                                                                                   |
| ---------------------- | ----------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| **Skill + MCP**        | MCP proporciona la conexión; una skill enseña a Claude cómo usarla bien                                     | MCP se conecta a su base de datos, una skill documenta su esquema y patrones de consulta                  |
| **Skill + Subagent**   | Una skill genera subagents para trabajo paralelo                                                            | La skill `/audit` inicia subagents de seguridad, rendimiento y estilo que trabajan en contexto aislado    |
| **CLAUDE.md + Skills** | CLAUDE.md contiene reglas siempre activas; las skills contienen material de referencia cargado bajo demanda | CLAUDE.md dice "sigue nuestras convenciones de API," una skill contiene la guía de estilo de API completa |
| **Hook + MCP**         | Un hook desencadena acciones externas a través de MCP                                                       | El hook post-edición envía una notificación de Slack cuando Claude modifica archivos críticos             |

<h2 id="understand-context-costs">
  Entender costos de contexto
</h2>

Cada característica que agrega consume algo del contexto de Claude. Demasiado puede llenar su ventana de contexto, pero también puede agregar ruido que hace que Claude sea menos efectivo; las skills pueden no desencadenarse correctamente, o Claude puede perder de vista sus convenciones. Entender estos compromisos lo ayuda a construir una configuración efectiva. Para una vista interactiva de cómo estas características se combinan en una sesión en ejecución, consulte [Explorar la ventana de contexto](/docs/es/context-window).

<h3 id="context-cost-by-feature">
  Costo de contexto por característica
</h3>

Cada característica tiene una estrategia de carga y costo de contexto diferentes:

| Característica             | Cuándo se carga                                         | Qué se carga                                                                                                                       | Costo de contexto                                     |
| -------------------------- | ------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------- |
| **CLAUDE.md**              | Inicio de sesión                                        | Contenido completo                                                                                                                 | Cada solicitud                                        |
| **Estilos de salida**      | Inicio de sesión, y nuevamente cuando cambia de estilos | Las instrucciones completas del estilo activo; nada para el estilo Predeterminado                                                  | Cada solicitud                                        |
| **Skills**                 | Inicio de sesión + cuando se usa                        | Descripciones al inicio, contenido completo cuando se usa                                                                          | Bajo (descripciones cada solicitud)\*                 |
| **Servidores MCP**         | Inicio de sesión                                        | Nombres de herramientas; esquemas completos bajo demanda                                                                           | Bajo hasta que se usa una herramienta                 |
| **Inteligencia de código** | Después de ediciones de archivo y bajo demanda          | Diagnósticos después de ediciones; ubicaciones de símbolos en búsqueda                                                             | Bajo; reduce lecturas de archivo en otros lugares     |
| **Subagents**              | Cuando se generan                                       | Contexto fresco con skills especificadas, o la conversación principal para un [fork](/docs/es/sub-agents#fork-the-current-conversation) | Aislado de la sesión principal                        |
| **Hooks**                  | Al desencadenar                                         | Nada (se ejecuta externamente)                                                                                                     | Cero, a menos que el hook devuelva contexto adicional |

\*Por defecto, las descripciones de skills se cargan al inicio de sesión para que Claude pueda decidir cuándo usarlas. Establezca `disable-model-invocation: true` en el frontmatter de una skill para ocultarla de Claude completamente hasta que la invoque manualmente. Para una skill que no escribió, establezca [`skillOverrides`](/docs/es/skills#override-skill-visibility-from-settings) en la configuración para hacer lo mismo sin editar su archivo.

<h3 id="understand-how-features-load">
  Entender cómo se cargan las características
</h3>

Cada característica se carga en diferentes puntos de su sesión. Las pestañas a continuación explican cuándo se carga cada una y qué entra en contexto.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/context-loading.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=aab139e750494a237ae2e0c8f9139b0a" className="dark:hidden" alt="Carga de contexto: CLAUDE.md se carga al inicio de sesión y permanece en cada solicitud. Los nombres de herramientas MCP se cargan al inicio con esquemas completos diferidos hasta el uso. Las skills cargan descripciones al inicio, contenido completo al invocar. Los subagents obtienen contexto aislado. Los hooks se ejecutan externamente." width="720" height="382" data-path="images/context-loading.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/context-loading-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=b274089ef9612d9c760bca9838557626" className="hidden dark:block" alt="Carga de contexto: CLAUDE.md se carga al inicio de sesión y permanece en cada solicitud. Los nombres de herramientas MCP se cargan al inicio con esquemas completos diferidos hasta el uso. Las skills cargan descripciones al inicio, contenido completo al invocar. Los subagents obtienen contexto aislado. Los hooks se ejecutan externamente." width="720" height="382" data-path="images/context-loading-dark.svg" />

<Tabs>
  <Tab title="CLAUDE.md">
    **Cuándo:** Inicio de sesión

    **Qué se carga:** Contenido completo de todos los archivos CLAUDE.md (niveles administrado, usuario y proyecto).

    **Herencia:** Claude lee archivos CLAUDE.md de su directorio de trabajo hasta la raíz, y descubre los anidados en subdirectorios mientras accede a esos archivos. Consulte [Cómo se cargan los archivos CLAUDE.md](/docs/es/memory#how-claude-md-files-load) para obtener detalles.

    <Tip>Mantenga CLAUDE.md bajo 200 líneas. Mueva material de referencia a skills, que se cargan bajo demanda. Para obtener [propuestas de recorte para un CLAUDE.md registrado](/docs/es/memory#my-claude-md-is-too-large), ejecute `/doctor`.</Tip>
  </Tab>

  <Tab title="Skills">
    Las skills son capacidades adicionales en el kit de herramientas de Claude. Pueden ser material de referencia (como una guía de estilo de API) o flujos de trabajo invocables que desencadena con `/<name>` (como `/deploy`). Claude Code incluye [skills incluidas](/docs/es/commands) como `/code-review`, `/batch` y `/debug` que funcionan de inmediato. También puede crear las suyas propias.

    **Cuándo:** Depende de la configuración de la skill. Por defecto, las descripciones se cargan al inicio de sesión y el contenido completo se carga cuando se usa. Para skills solo de usuario (`disable-model-invocation: true`), nada se carga hasta que las invoque.

    **Qué se carga:** Para skills invocables por modelo, Claude ve nombres y descripciones en cada solicitud. Cuando invoca una skill con `/<name>` o Claude la carga automáticamente, el contenido completo se carga en su conversación.

    **Cómo Claude elige skills:** Claude hace coincidir su tarea contra descripciones de skills para decidir cuáles son relevantes. Si las descripciones son vagas u se superponen, Claude puede cargar la skill incorrecta o perder una que ayudaría. Para decirle a Claude que use una skill específica, invóquela con `/<name>`. Las skills con `disable-model-invocation: true` son invisibles para Claude hasta que las invoque.

    **Costo de contexto:** Bajo hasta que se use. Las skills solo de usuario tienen costo cero hasta que se invoquen.

    **En subagents:** Las skills funcionan de manera diferente en subagents. En lugar de carga bajo demanda, las skills listadas en el campo `skills` del subagent se precarga completamente en su contexto al iniciar. Los subagents aún pueden descubrir e invocar skills de proyecto, usuario y plugin no listadas a través de la herramienta Skill.

    <Tip>Use `disable-model-invocation: true` para skills con efectos secundarios. Esto ahorra contexto y asegura que solo usted las desencadene.</Tip>
  </Tab>

  <Tab title="Servidores MCP">
    **Cuándo:** Inicio de sesión.

    **Qué se carga:** Nombres de herramientas e instrucciones del servidor desde servidores conectados. Los esquemas JSON completos permanecen diferidos hasta que Claude necesita una herramienta específica.

    **Costo de contexto:** [Búsqueda de herramientas](/docs/es/mcp#scale-with-mcp-tool-search) está habilitada por defecto, por lo que las herramientas MCP inactivas consumen contexto mínimo.

    <Tip>Ejecute `/mcp` para ver estado de conexión de cada servidor. Ejecute `/context all` para ver cuántos tokens usa cada herramienta MCP cargada. Claude Code [se reconecta automáticamente a servidores remotos](/docs/es/mcp#automatic-reconnection) si se desconectan, y puede desconectar servidores que no esté usando activamente.</Tip>
  </Tab>

  <Tab title="Inteligencia de código">
    **Cuándo:** Después de ediciones de archivo, y bajo demanda cuando Claude navega código.

    **Qué se carga:** Errores de tipo y advertencias después de cada edición de archivo. Información de definición, referencia y tipo cuando Claude busca un símbolo.

    **Costo de contexto:** Bajo. Las búsquedas de símbolos a menudo reemplazan lecturas amplias de archivos, por lo que el uso neto de contexto puede disminuir.

    <Tip>La herramienta LSP está inactiva hasta que instale un [plugin de inteligencia de código](/docs/es/plugins/code-intelligence) para su lenguaje.</Tip>
  </Tab>

  <Tab title="Subagents">
    **Cuándo:** Bajo demanda, cuando usted o Claude genera uno para una tarea.

    **Qué se carga:** Contexto fresco y aislado que contiene:

    * El prompt del sistema del agente, no el prompt del sistema de Claude Code
    * Contenido completo de skills listadas en el campo `skills:` del agente
    * CLAUDE.md y estado de git, excepto los agentes Explore y Plan integrados [omiten ambos](/docs/es/sub-agents#what-loads-at-startup), y un agente cuya definición establece [`omitClaudeMd`](/docs/es/sub-agents#supported-frontmatter-fields) omite los archivos CLAUDE.md de usuario, proyecto y local
    * Cualquier contexto que el agente principal pase en el prompt

    Para un [fork](/docs/es/sub-agents#fork-the-current-conversation), Claude Code carga la conversación del principal hasta ahora, el prompt del sistema y las herramientas en su lugar.

    **Costo de contexto:** Aislado de la sesión principal.

    <Tip>Use subagents para trabajo que no necesita su contexto de conversación completo. Su aislamiento previene inflar su sesión principal.</Tip>
  </Tab>

  <Tab title="Hooks">
    **Cuándo:** Al desencadenar. Claude Code ejecuta hooks en eventos de ciclo de vida específicos como ejecución de herramientas, límites de sesión, envío de prompt, solicitudes de permiso y compactación. Consulte [Hooks](/docs/es/hooks) para la lista completa.

    **Qué se carga:** Nada por defecto. Los hooks se ejecutan fuera de la conversación principal.

    **Costo de contexto:** Cero, a menos que el hook devuelva salida que se agregue como mensajes a su conversación.

    <Tip>Los hooks son ideales para efectos secundarios (linting, logging) que no necesitan afectar el contexto de Claude.</Tip>
  </Tab>
</Tabs>

<h2 id="learn-more">
  Aprender más
</h2>

Cada característica tiene su propia guía con instrucciones de configuración, ejemplos y opciones de configuración.

<CardGroup cols={2}>
  <Card title="CLAUDE.md" icon="file-lines" href="/docs/es/memory">
    Almacenar contexto de proyecto, convenciones e instrucciones
  </Card>

  <Card title="Skills" icon="brain" href="/docs/es/skills">
    Dar a Claude experiencia de dominio y flujos de trabajo reutilizables
  </Card>

  <Card title="Subagents" icon="users" href="/docs/es/sub-agents">
    Descargar trabajo a contexto aislado
  </Card>

  <Card title="Dynamic workflows" icon="network" href="/docs/es/workflows">
    Ejecutar muchos subagentes desde un script
  </Card>

  <Card title="Cross-session messaging" icon="terminal" href="/docs/es/cross-session-messaging">
    Permitir que Claude envíe mensajes a sus otras sesiones
  </Card>

  <Card title="MCP" icon="plug" href="/docs/es/mcp">
    Conectar Claude a servicios externos
  </Card>

  <Card title="Hooks" icon="bolt" href="/docs/es/hooks-guide">
    Automatizar acciones con hooks
  </Card>

  <Card title="Plugins" icon="puzzle-piece" href="/docs/es/plugins/overview">
    Empaquetar y compartir conjuntos de características
  </Card>

  <Card title="Marketplaces" icon="store" href="/docs/es/plugins/create-marketplace">
    Alojar y distribuir colecciones de plugins
  </Card>
</CardGroup>
