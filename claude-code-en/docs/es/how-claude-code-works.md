> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Cómo funciona Claude Code

> Comprenda el bucle agentico, las herramientas integradas y cómo Claude Code interactúa con su proyecto.

Claude Code es un asistente agentico que se ejecuta en su terminal. Aunque destaca en codificación, puede ayudarle con cualquier cosa que pueda hacer desde la línea de comandos: escribir documentación, ejecutar compilaciones, buscar archivos, investigar temas y más.

Esta guía cubre la arquitectura principal, las capacidades integradas y [consejos para trabajar efectivamente](#work-effectively-with-claude-code). Para tutoriales paso a paso, consulte [Flujos de trabajo comunes](/docs/es/common-workflows). Para características de extensibilidad como skills, MCP y hooks, consulte [Extender Claude Code](/docs/es/features-overview).

<h2 id="the-agentic-loop">
  El bucle agentico
</h2>

Cuando le da una tarea a Claude, trabaja a través de tres fases: **recopilar contexto**, **tomar acción** y **verificar resultados**. Estas fases se mezclan entre sí. Claude utiliza herramientas en todo momento, ya sea buscando archivos para entender su código, editando para hacer cambios o ejecutando pruebas para verificar su trabajo.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agentic-loop.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=4a30fb7ce2815012a9f27c955e2c6bb0" alt="Diagrama del bucle agentico: Su indicación lleva a Claude a recopilar contexto, tomar acción, verificar resultados y repetir hasta completar la tarea. Puede interrumpir en cualquier momento." width="720" height="280" data-path="images/agentic-loop.svg" />

El bucle se adapta a lo que pregunta. Una pregunta sobre su base de código podría necesitar solo recopilación de contexto. Una corrección de errores cicla a través de las tres fases repetidamente. Una refactorización podría implicar una verificación extensa. Claude decide qué requiere cada paso basándose en lo que aprendió del paso anterior, encadenando docenas de acciones juntas y corrigiendo el curso en el camino.

Usted también es parte de este bucle. Puede interrumpir en cualquier momento para dirigir a Claude en una dirección diferente, proporcionar contexto adicional o pedirle que intente un enfoque diferente. Claude trabaja de forma autónoma pero permanece receptivo a su entrada.

El bucle agentico está impulsado por dos componentes: [modelos](#models) que razonan y [herramientas](#tools) que actúan. Claude Code sirve como el **arnés agentico** alrededor de Claude: proporciona las herramientas, la gestión del contexto y el entorno de ejecución que convierten un modelo de lenguaje en un agente de codificación capaz.

<h3 id="models">
  Modelos
</h3>

Claude Code utiliza modelos Claude para entender su código y razonar sobre tareas. Claude puede leer código en cualquier idioma, entender cómo se conectan los componentes y determinar qué necesita cambiar para lograr su objetivo. Para tareas complejas, divide el trabajo en pasos, los ejecuta y se ajusta basándose en lo que aprende.

[Múltiples modelos](/docs/es/model-config) están disponibles con diferentes compensaciones. Sonnet maneja bien la mayoría de tareas de codificación. Opus proporciona un razonamiento más fuerte para decisiones arquitectónicas complejas. Cambie con `/model` durante una sesión o comience con `claude --model <name>`.

Cuando esta guía dice "Claude elige" o "Claude decide", es el modelo el que está haciendo el razonamiento.

<h3 id="tools">
  Herramientas
</h3>

Las herramientas son lo que hace que Claude Code sea agentico. Sin herramientas, Claude solo puede responder con texto. Con herramientas, Claude puede actuar: leer su código, editar archivos, ejecutar comandos, buscar en la web e interactuar con servicios externos. Cada uso de herramienta devuelve información que se retroalimenta en el bucle, informando la siguiente decisión de Claude.

Las herramientas integradas generalmente se dividen en cinco categorías, cada una representando un tipo diferente de agencia.

| Categoría                  | Lo que Claude puede hacer                                                                                                                                                                    |
| -------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Operaciones de archivo** | Leer archivos, editar código, crear nuevos archivos, renombrar y reorganizar                                                                                                                 |
| **Búsqueda**               | Encontrar archivos por patrón, buscar contenido con regex, explorar bases de código                                                                                                          |
| **Ejecución**              | Ejecutar comandos de shell, iniciar servidores, ejecutar pruebas, usar git                                                                                                                   |
| **Web**                    | Buscar en la web, obtener documentación, buscar mensajes de error                                                                                                                            |
| **Inteligencia de código** | Ver errores de tipo y advertencias después de ediciones, saltar a definiciones, encontrar referencias (requiere [plugins de inteligencia de código](/docs/es/discover-plugins#code-intelligence)) |

Estas son las capacidades principales. Claude también tiene herramientas para generar subagents, hacerle preguntas y otras tareas de orquestación. Consulte [Herramientas disponibles para Claude](/docs/es/tools-reference) para la lista completa.

Claude elige qué herramientas usar basándose en su indicación y lo que aprende en el camino. Cuando dice "arreglar las pruebas fallidas", Claude podría:

1. Ejecutar el conjunto de pruebas para ver qué está fallando
2. Leer la salida de error
3. Buscar los archivos de código fuente relevantes
4. Leer esos archivos para entender el código
5. Editar los archivos para arreglar el problema
6. Ejecutar las pruebas nuevamente para verificar

Cada uso de herramienta le da a Claude nueva información que informa el siguiente paso. Este es el bucle agentico en acción.

**Extender las capacidades base:** Las herramientas integradas son la base. Puede extender lo que Claude sabe con [skills](/docs/es/skills), conectarse a servicios externos con [MCP](/docs/es/mcp), automatizar flujos de trabajo con [hooks](/docs/es/hooks) y delegar tareas a [subagents](/docs/es/sub-agents). Estas extensiones forman una capa encima del bucle agentico principal. Consulte [Extender Claude Code](/docs/es/features-overview) para orientación sobre cómo elegir la extensión correcta para sus necesidades.

<h2 id="what-claude-can-access">
  A qué puede acceder Claude
</h2>

Esta guía se enfoca en la terminal. Claude Code también se ejecuta en [VS Code](/docs/es/vs-code), [IDEs de JetBrains](/docs/es/jetbrains) y otros entornos.

Cuando ejecuta `claude` en un directorio, Claude Code obtiene acceso a:

* **Su proyecto.** Archivos en su directorio y subdirectorios, más archivos en otros lugares con su permiso.
* **Su terminal.** Cualquier comando que pueda ejecutar: herramientas de compilación, git, gestores de paquetes, utilidades del sistema, scripts. Si puede hacerlo desde la línea de comandos, Claude también puede.
* **Su estado de git.** Rama actual, cambios sin confirmar e historial de confirmaciones recientes.
* **Su [CLAUDE.md](/docs/es/memory).** Un archivo markdown donde almacena instrucciones específicas del proyecto, convenciones y contexto que Claude debe conocer en cada sesión.
* **[Auto memory](/docs/es/memory#auto-memory).** Aprendizajes que Claude guarda automáticamente mientras trabaja, como patrones de proyecto y sus preferencias. Las primeras 200 líneas o 25KB de MEMORY.md, lo que sea menor, se cargan al inicio de cada sesión.
* **Extensiones que configure.** [Servidores MCP](/docs/es/mcp) para servicios externos, [skills](/docs/es/skills) para flujos de trabajo, [subagents](/docs/es/sub-agents) para trabajo delegado y [Claude en Chrome](/docs/es/chrome) para interacción del navegador.

Debido a que Claude ve todo su proyecto, puede trabajar en él. Cuando le pide a Claude que "arregle el error de autenticación", busca archivos relevantes, lee múltiples archivos para entender el contexto, realiza ediciones coordinadas en ellos, ejecuta pruebas para verificar la corrección y confirma los cambios si lo solicita. Esto es diferente de los asistentes de código en línea que solo ven el archivo actual.

<h2 id="environments-and-interfaces">
  Entornos e interfaces
</h2>

El bucle agentico, las herramientas y las capacidades descritas anteriormente son iguales en todas partes donde use Claude Code. Lo que cambia es dónde se ejecuta el código y cómo interactúa con él.

<h3 id="execution-environments">
  Entornos de ejecución
</h3>

Claude Code se ejecuta en tres entornos, cada uno con diferentes compensaciones para dónde se ejecuta su código.

| Entorno            | Dónde se ejecuta el código                | Caso de uso                                                            |
| ------------------ | ----------------------------------------- | ---------------------------------------------------------------------- |
| **Local**          | Su máquina                                | Predeterminado. Acceso completo a sus archivos, herramientas y entorno |
| **Cloud**          | VMs administradas por Anthropic           | Delegar tareas, trabajar en repositorios que no tiene localmente       |
| **Control remoto** | Su máquina, controlada desde un navegador | Usar la interfaz web mientras mantiene todo local                      |

<h3 id="interfaces">
  Interfaces
</h3>

Puede acceder a Claude Code a través de la terminal, la [aplicación de escritorio](/docs/es/desktop), [extensiones de IDE](/docs/es/vs-code), [claude.ai/code](https://claude.ai/code), [Control remoto](/docs/es/remote-control), [Slack](/docs/es/slack) y [canalizaciones CI/CD](/docs/es/github-actions). La interfaz determina cómo ve e interactúa con Claude, pero el bucle agentico subyacente es idéntico. Consulte [Usar Claude Code en todas partes](/docs/es/overview#use-claude-code-everywhere) para la lista completa.

<h2 id="work-with-sessions">
  Trabajar con sesiones
</h2>

Claude Code guarda su conversación localmente mientras trabaja. Cada mensaje, uso de herramienta y resultado se escribe en un archivo JSONL de texto plano bajo `~/.claude/projects/`, lo que permite [rebobinar](#undo-changes-with-checkpoints), [reanudar y bifurcar](#resume-or-fork-sessions) sesiones. Antes de que Claude realice cambios de código, también toma una instantánea de los archivos afectados para que pueda revertir si es necesario. Para rutas, retención y cómo borrar estos datos, consulte [datos de aplicación en `~/.claude`](/docs/es/claude-directory#application-data).

**Las sesiones son independientes.** Cada nueva sesión comienza con una ventana de contexto nueva, sin el historial de conversación de sesiones anteriores. Claude puede persistir aprendizajes entre sesiones usando [auto memory](/docs/es/memory#auto-memory), y puede agregar sus propias instrucciones persistentes en [CLAUDE.md](/docs/es/memory).

<h3 id="work-across-branches">
  Trabajar entre ramas
</h3>

Cada conversación de Claude Code es una sesión vinculada a su directorio actual. El selector `/resume` muestra sesiones del worktree actual de forma predeterminada, con atajos de teclado para ampliar la lista a otros worktrees o proyectos. Consulte [Gestionar sesiones](/docs/es/sessions#use-the-session-picker) para la lista completa de atajos de teclado del selector y cómo funciona la resolución de nombres.

Claude ve los archivos de su rama actual. Cuando cambia de rama, Claude ve los archivos de la nueva rama, pero el historial de conversación permanece igual. Claude recuerda lo que discutió incluso después de cambiar de rama.

Dado que las sesiones están vinculadas a directorios, puede ejecutar sesiones paralelas de Claude Code usando [git worktrees](/docs/es/worktrees), que crean directorios separados para ramas individuales.

<h3 id="resume-or-fork-sessions">
  Reanudar o bifurcar sesiones
</h3>

Reanudar una sesión con `claude --continue` o `claude --resume` la reabre bajo el mismo ID de sesión y agrega nuevos mensajes a la conversación existente. Bifurcar con `--fork-session` o `/branch` copia el historial en un nuevo ID de sesión, dejando el original sin cambios.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/session-continuity.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=04ed0984a58e4127e05b3640265241a3" alt="Continuidad de sesión: reanudar continúa la misma sesión, bifurcar crea una nueva rama con un nuevo ID." width="560" height="280" data-path="images/session-continuity.svg" />

Para las banderas de reanudación, el selector `/resume`, nombres y qué sucede cuando la misma sesión está abierta en dos terminales, consulte [Gestionar sesiones](/docs/es/sessions).

<h3 id="the-context-window">
  La ventana de contexto
</h3>

La ventana de contexto de Claude contiene el historial de su conversación, contenidos de archivos, salidas de comandos, [CLAUDE.md](/docs/es/memory), [auto memory](/docs/es/memory#auto-memory), skills cargadas e instrucciones del sistema. A medida que trabaja, el contexto se llena. Claude se compacta automáticamente, pero las instrucciones del principio de la conversación pueden perderse. Coloque reglas persistentes en CLAUDE.md y ejecute `/context` para ver qué está usando espacio.

Para un recorrido interactivo de qué se carga y cuándo, consulte [Explorar la ventana de contexto](/docs/es/context-window).

<h4 id="when-context-fills-up">
  Cuando el contexto se llena
</h4>

Claude Code gestiona el contexto automáticamente a medida que se acerca al límite. Primero borra salidas de herramientas más antiguas, luego resume la conversación si es necesario. Sus solicitudes y fragmentos de código clave se preservan; las instrucciones detalladas del principio de la conversación pueden perderse. Coloque reglas persistentes en CLAUDE.md en lugar de depender del historial de conversación.

Para controlar qué se preserva durante la compactación, agregue una sección "Compact Instructions" a CLAUDE.md o ejecute `/compact` con un enfoque (como `/compact focus on the API changes`).

Si un archivo único o salida de herramienta es tan grande que el contexto se vuelve a llenar inmediatamente después de cada resumen, Claude Code deja de compactarse automáticamente después de algunos intentos y muestra un error en lugar de hacer un bucle. Consulte [Auto-compaction stops with a thrashing error](/docs/es/troubleshooting#auto-compaction-stops-with-a-thrashing-error) para pasos de recuperación.

Ejecute `/context` para ver qué está usando espacio. Las definiciones de herramientas MCP se difieren por defecto y se cargan bajo demanda a través de [búsqueda de herramientas](/docs/es/mcp#scale-with-mcp-tool-search), por lo que solo los nombres de herramientas consumen contexto hasta que Claude use una herramienta específica. Ejecute `/mcp` para verificar costos por servidor.

<h4 id="manage-context-with-skills-and-subagents">
  Gestionar contexto con skills y subagents
</h4>

Más allá de la compactación, puede usar otras características para controlar qué se carga en el contexto.

[Skills](/docs/es/skills) se cargan bajo demanda. Claude ve descripciones de skills al inicio de la sesión, pero el contenido completo solo se carga cuando se usa una skill. Para skills que invoca manualmente, establezca `disable-model-invocation: true` para mantener descripciones fuera del contexto hasta que las necesite. Para skills que no escribió, use [`skillOverrides`](/docs/es/skills#override-skill-visibility-from-settings) para hacer lo mismo desde la configuración.

[Subagents](/docs/es/sub-agents) obtienen su propio contexto nuevo, completamente separado de su conversación principal. Su trabajo no infla su contexto. Cuando terminan, devuelven un resumen. Este aislamiento es por qué los subagents ayudan con sesiones largas.

Consulte [costos de contexto](/docs/es/features-overview#understand-context-costs) para lo que cuesta cada característica y [reducir el uso de tokens](/docs/es/costs#reduce-token-usage) para consejos sobre cómo gestionar el contexto.

<h2 id="stay-safe-with-checkpoints-and-permissions">
  Manténgase seguro con checkpoints y permisos
</h2>

Claude tiene dos mecanismos de seguridad: los checkpoints le permiten deshacer cambios de archivo y los permisos controlan qué puede hacer Claude sin preguntar.

<h3 id="undo-changes-with-checkpoints">
  Deshacer cambios con checkpoints
</h3>

**Cada edición de archivo es reversible.** Antes de que Claude edite cualquier archivo, toma una instantánea del contenido actual. Si algo sale mal, presione `Esc` dos veces para rebobinar a un estado anterior, o pida a Claude que deshaga.

Los checkpoints son separados de git y permanecen disponibles cuando reanuda una conversación. Solo cubren cambios de archivo. Las acciones que afectan sistemas remotos (bases de datos, APIs, implementaciones) no pueden ser checkpointed, por lo que Claude pregunta antes de ejecutar comandos con efectos secundarios externos.

<h3 id="control-what-claude-can-do">
  Controle qué puede hacer Claude
</h3>

Presione `Shift+Tab` para ciclar a través de modos de permiso:

* **Manual**: Claude pregunta antes de ediciones de archivo y comandos de shell
* **Accept edits**: Claude edita archivos y ejecuta comandos comunes del sistema de archivos como `mkdir` y `mv` sin preguntar, aún pregunta por otros comandos
* **Plan**: Claude explora y propone un plan sin editar sus archivos fuente
* **Auto**: Claude evalúa todas las acciones con verificaciones de seguridad en segundo plano

También puede permitir comandos específicos en `.claude/settings.json` para que Claude no pregunte cada vez. Esto es útil para comandos confiables como `npm test` o `git status`. La configuración puede tener alcance desde políticas de toda la organización hasta preferencias personales. Consulte [Permisos](/docs/es/permissions) para detalles.

***

<h2 id="work-effectively-with-claude-code">
  Trabajar efectivamente con Claude Code
</h2>

Estos consejos le ayudan a obtener mejores resultados de Claude Code.

<h3 id="ask-claude-code-for-help">
  Pida ayuda a Claude Code
</h3>

Claude Code puede enseñarle cómo usarlo. Haga preguntas como "¿cómo configuro hooks?" o "¿cuál es la mejor manera de estructurar mi CLAUDE.md?" y Claude explicará.

Los comandos integrados también lo guían a través de la configuración:

* `/init` lo guía a través de la creación de un CLAUDE.md para su proyecto
* `/doctor` ejecuta una verificación de configuración que diagnostica problemas de instalación y configuración y puede solucionarlos

<h3 id="it’s-a-conversation">
  Es una conversación
</h3>

Claude Code es conversacional. No necesita indicaciones perfectas. Comience con lo que desea, luego refine:

```text theme={null}
Arreglar el error de inicio de sesión
```

\[Claude investiga, intenta algo]

```text theme={null}
Eso no es del todo correcto. El problema está en el manejo de sesiones.
```

\[Claude ajusta el enfoque]

Cuando el primer intento no es correcto, no comienza de nuevo. Itera.

<h4 id="interrupt-and-steer">
  Interrumpir y dirigir
</h4>

Puede redirigir a Claude en cualquier momento sin esperar a que termine el turno o comenzar de nuevo:

* **Presione `Esc`** para detener a Claude inmediatamente. La llamada de herramienta en ejecución se cancela y Claude espera su siguiente instrucción.
* **Escriba una corrección y presione `Enter`** para enviarla sin detener la herramienta en ejecución. Claude la lee tan pronto como se completa la acción actual y se ajusta antes de decidir su siguiente paso.

<h3 id="be-specific-upfront">
  Sea específico desde el principio
</h3>

Cuanto más precisa sea su indicación inicial, menos correcciones necesitará. Haga referencia a archivos específicos, mencione restricciones y señale patrones de ejemplo.

```text theme={null}
El flujo de pago está roto para usuarios con tarjetas vencidas.
Verifique src/payments/ para el problema, especialmente la actualización de tokens.
Escriba una prueba fallida primero, luego arréglela.
```

Las indicaciones vagas funcionan, pero pasará más tiempo dirigiendo. Las indicaciones específicas como la anterior a menudo tienen éxito en el primer intento.

<h3 id="give-claude-something-to-verify-against">
  Dé a Claude algo contra lo que verificar
</h3>

Claude funciona mejor cuando puede verificar su propio trabajo. Incluya casos de prueba, pegue capturas de pantalla de la interfaz de usuario esperada o defina la salida que desea.

```text theme={null}
Implementar validateEmail. Casos de prueba: 'user@example.com' → true,
'invalid' → false, 'user@.com' → false. Ejecute las pruebas después.
```

Para trabajo visual, pegue una captura de pantalla del diseño y pida a Claude que compare su implementación con ella.

<h3 id="explore-before-implementing">
  Explorar antes de implementar
</h3>

Para problemas complejos, separe la investigación de la codificación. Use plan mode (`Shift+Tab` dos veces) para analizar la base de código primero:

```text theme={null}
Lea src/auth/ y entienda cómo manejamos sesiones.
Luego cree un plan para agregar soporte OAuth.
```

Revise el plan, refínelo a través de la conversación, luego deje que Claude implemente. Este enfoque de dos fases produce mejores resultados que saltar directamente al código.

<h3 id="delegate-don’t-dictate">
  Delegue, no dicte
</h3>

Piense en delegar a un colega capaz. Dé contexto y dirección, luego confíe en que Claude descubra los detalles:

```text theme={null}
El flujo de pago está roto para usuarios con tarjetas vencidas.
El código relevante está en src/payments/. ¿Puede investigar y arreglarlo?
```

No necesita especificar qué archivos leer o qué comandos ejecutar. Claude lo descubre.

<h2 id="what’s-next">
  Qué sigue
</h2>

<CardGroup cols={2}>
  <Card title="Extender con características" icon="puzzle-piece" href="/docs/es/features-overview">
    Agregue Skills, conexiones MCP y comandos personalizados
  </Card>

  <Card title="Flujos de trabajo comunes" icon="graduation-cap" href="/docs/es/common-workflows">
    Guías paso a paso para tareas típicas
  </Card>
</CardGroup>
