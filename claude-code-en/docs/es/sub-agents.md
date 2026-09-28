> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Crear subagentes personalizados

> Cree y utilice subagentes de IA especializados en Claude Code para flujos de trabajo específicos de tareas y una mejor gestión del contexto.

Los subagentes son asistentes de IA especializados que manejan tipos específicos de tareas. Utilice uno cuando una tarea secundaria inundaría su conversación principal con resultados de búsqueda, registros o contenidos de archivos que no volverá a consultar: el subagente realiza ese trabajo en su propio contexto y devuelve solo el resumen. Defina un subagente personalizado cuando siga generando el mismo tipo de trabajador con las mismas instrucciones.

Cada subagente se ejecuta en su propia ventana de contexto con un mensaje del sistema personalizado, acceso a herramientas específicas y permisos independientes. Cuando Claude encuentra una tarea que coincide con la descripción de un subagente, delega en ese subagente, que trabaja de forma independiente y devuelve resultados. Para ver el ahorro de contexto en la práctica, la [visualización de la ventana de contexto](/docs/es/context-window) muestra un recorrido por una sesión donde un subagente maneja la investigación en su propia ventana separada.

<Note>
  Los subagentes funcionan dentro de una única sesión. Para ejecutar muchas sesiones independientes en paralelo y supervisarlas desde un único lugar, consulte [agentes en segundo plano](/docs/es/agent-view). Para sesiones separadas que se pasen mensajes entre sí, consulte [mensajería entre sesiones](/docs/es/cross-session-messaging). Para un equipo coordinado de sesiones que Claude genera y supervisa, consulte [equipos de agentes](/docs/es/agent-teams).
</Note>

Los subagentes le ayudan a:

* **Preservar contexto** manteniendo la exploración e implementación fuera de su conversación principal
* **Aplicar restricciones** limitando qué herramientas puede usar un subagente
* **Reutilizar configuraciones** en proyectos con subagentes a nivel de usuario
* **Especializar comportamiento** con mensajes del sistema enfocados para dominios específicos
* **Controlar costos** enrutando tareas a modelos más rápidos y económicos como Haiku

Claude utiliza la descripción de cada subagente para decidir cuándo delegar tareas. Cuando crea un subagente, escriba una descripción clara para que Claude sepa cuándo usarlo.

Esas descripciones ocupan contexto, así que manténgalas breves. Cuando las descripciones combinadas de sus subagentes, excepto los integrados, superan 15.000 tokens, Claude Code muestra una [advertencia al inicio con el recuento total de tokens](/docs/es/errors#agent-descriptions-are-over-the-15000-token-limit). Recorte los campos `description` de sus subagentes y traslade los detalles al mensaje del sistema de cada subagente, que solo se carga cuando ese subagente se ejecuta.

<h2 id="built-in-subagents">
  Subagentes integrados
</h2>

Claude Code incluye subagentes integrados que Claude utiliza automáticamente cuando es apropiado. Cada uno hereda los permisos de la conversación principal; la mayoría se ejecuta con un conjunto de herramientas restringido.

Explore y Plan omiten sus archivos CLAUDE.md y la instantánea del estado de git para mantener la investigación rápida y económica. Todos los demás subagentes integrados y [subagentes personalizados](#configure-subagents) cargan ambos, a menos que su definición establezca el campo [`omitClaudeMd`](#supported-frontmatter-fields) para omitir los archivos CLAUDE.md del usuario, proyecto y local. Para obtener el desglose completo de lo que llega a un subagente, consulte [qué se carga al iniciar](#what-loads-at-startup).

<Tabs>
  <Tab title="Explore">
    Un agente rápido y de solo lectura optimizado para buscar y analizar bases de código.

    * **Modelo**: hereda de la conversación principal, limitado a Opus en la API de Claude, por lo que Explore nunca se ejecuta en un modelo más costoso que el que ya eligió para la sesión, a menos que establezca `CLAUDE_CODE_SUBAGENT_MODEL` y [lo fuerce en todos los subagentes](#run-every-subagent-on-one-model)
    * **Herramientas**: herramientas de solo lectura; Write y Edit están denegadas
    * **Propósito**: descubrimiento de archivos, búsqueda de código, exploración de base de código

    A partir de v2.1.198, Explore hereda el modelo de la conversación principal en lugar de ejecutarse siempre en Haiku. En la API de Claude, el modelo heredado se limita a Opus: una conversación principal en un nivel superior ejecuta Explore en Opus, y una conversación principal en Sonnet o Haiku ejecuta Explore en ese mismo modelo. En cualquier otro proveedor, como [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, o Claude Platform en AWS](/docs/es/third-party-integrations), Explore hereda el modelo de la conversación principal directamente.

    Un subagente de [usuario o proyecto](#choose-the-subagent-scope) llamado `Explore` anula el integrado y mantiene su propio campo `model`, así que defina uno con `model: haiku` para mantener la exploración en un modelo de menor costo.

    Claude delega en Explore cuando necesita buscar o entender una base de código sin hacer cambios. Esto mantiene los resultados de exploración fuera del contexto de su conversación principal.

    Al invocar Explore, Claude especifica un nivel de minuciosidad: **quick** para búsquedas dirigidas, **medium** para exploración equilibrada, o **very thorough** para análisis exhaustivo.
  </Tab>

  <Tab title="Plan">
    Un agente de investigación utilizado durante el [modo plan](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode) para recopilar contexto antes de presentar un plan.

    * **Modelo**: hereda de la conversación principal, a menos que establezca `CLAUDE_CODE_SUBAGENT_MODEL` y [lo fuerce en todos los subagentes](#run-every-subagent-on-one-model)
    * **Herramientas**: herramientas de solo lectura; Write y Edit están denegadas
    * **Propósito**: investigación de base de código para planificación

    Cuando está en modo plan y Claude necesita entender su base de código, delega la investigación al subagente Plan para que la salida de exploración permanezca en una ventana de contexto separada mientras la conversación principal permanece en solo lectura.
  </Tab>

  <Tab title="General-purpose">
    Un agente capaz para tareas complejas y de múltiples pasos que requieren tanto exploración como acción.

    * **Modelo**: el modelo [`CLAUDE_CODE_SUBAGENT_MODEL`](#choose-a-model) si establece uno y nada asigna un modelo de otra manera, de lo contrario el modelo de la conversación principal; [Choose a model](#choose-a-model) establece el orden completo, y [Run every subagent on one model](#run-every-subagent-on-one-model) muestra cómo hacer que la variable anule esas fuentes
    * **Herramientas**: todas las herramientas [disponibles para subagentes](#available-tools)
    * **Propósito**: investigación compleja, operaciones de múltiples pasos, modificaciones de código

    Claude delega a general-purpose cuando la tarea requiere tanto exploración como modificación, razonamiento complejo para interpretar resultados, o múltiples pasos dependientes.
  </Tab>

  <Tab title="Other">
    Claude Code incluye agentes auxiliares adicionales para tareas específicas. Estos se invocan típicamente automáticamente, por lo que no necesita usarlos directamente.

    | Agente            | Modelo                                                                                             | Cuándo Claude lo utiliza                                                                                                                                                                                                                                                                                                                                      |
    | :---------------- | :------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
    | claude            | Ninguno propio; sigue el [orden de modelo](#choose-a-model) cuando Claude lo genera como subagente | Cuando una tarea no se ajusta a un agente más especializado. Un comodín con todas las herramientas [disponibles para subagentes](#available-tools). También el agente predeterminado para una [sesión de fondo](/docs/es/agent-view) despachada; [qué modo de permiso inicia](/docs/es/agent-view#permission-mode-model-and-effort) depende de cómo se inició la sesión |
    | statusline-setup  | Sonnet                                                                                             | Cuando ejecuta `/statusline` para configurar su línea de estado                                                                                                                                                                                                                                                                                               |
    | claude-code-guide | Haiku                                                                                              | Cuando hace preguntas sobre características de Claude Code                                                                                                                                                                                                                                                                                                    |
  </Tab>
</Tabs>

Los subagentes integrados se registran de forma predeterminada en sesiones interactivas. Para restringirlos:

* Para bloquear un tipo integrado específico, agréguelo a `permissions.deny` como se muestra en [Disable specific subagents](#disable-specific-subagents).
* Para evitar que Claude delegue en ningún subagente, niegue la herramienta `Agent` misma con [`permissions.deny`](/docs/es/permissions#tool-specific-permission-rules).
* Para eliminar solo los subagentes integrados `Explore` y `Plan`, establezca [`CLAUDE_CODE_DISABLE_EXPLORE_PLAN_AGENTS=1`](/docs/es/env-vars). Claude lee y explora archivos directamente en lugar de delegar en ellos. Requiere Claude Code v2.1.198 o posterior.
* En [modo no interactivo](/docs/es/headless) y el [Agent SDK](/docs/es/agent-sdk/overview), establezca [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/es/env-vars) para eliminar todos los tipos integrados y proporcionar solo los suyos.

Una llamada a la herramienta Agent que omite `subagent_type` falla con [`subagent_type is required`](/docs/es/errors#subagent-type-is-required) cuando la sesión no tiene ningún subagente `general-purpose` en el que recurrir.

Más allá de estos subagentes integrados, puede crear los suyos propios con indicaciones personalizadas, restricciones de herramientas, modos de permiso, hooks y skills. Las siguientes secciones muestran cómo comenzar y personalizar subagentes.

<h2 id="quickstart-create-your-first-subagent">
  Inicio rápido: crear su primer subagente
</h2>

Los subagentes son archivos Markdown con frontmatter YAML. Para crear uno, pida a Claude que lo escriba por usted, o [escriba el archivo usted mismo](#write-subagent-files).

A partir de v2.1.198, el comando `/agents` ya no abre el asistente de creación interactivo; ejecutarlo imprime un recordatorio para pedir a Claude o editar `.claude/agents/` directamente. Los archivos de subagentes, los campos de frontmatter y las ubicaciones `.claude/agents/` y `~/.claude/agents/` no cambian; solo se elimina el asistente de terminal.

Este tutorial crea un subagente a nivel de usuario que revisa código y sugiere mejoras.

<Steps>
  <Step title="Pida a Claude que cree el subagente">
    En Claude Code, describa el subagente que desea y dónde guardarlo:

    ```text wrap theme={null}
    Create a personal code-improver subagent in ~/.claude/agents/ that scans
    files and suggests improvements for readability, performance, and best
    practices. It should explain each issue, show the current code, and
    provide an improved version. Make it read-only and have it use Sonnet.
    ```

    Claude escribe el archivo con un `name`, una `description`, una lista de `tools`, un `model` y un mensaje del sistema.
  </Step>

  <Step title="Revise el archivo">
    Abra `~/.claude/agents/code-improver.md` y confirme que el frontmatter coincida con lo que pidió. El resultado se ve así:

    ```markdown theme={null}
    ---
    name: code-improver
    description: Scans files and suggests improvements for readability, performance, and best practices. Use after writing or modifying code.
    tools: Read, Grep, Glob
    model: sonnet
    ---

    You are a code improvement specialist. For each issue you find, explain
    the problem, show the current code, and provide an improved version.
    ```

    Debido a que el archivo se encuentra en `~/.claude/agents/`, el subagente está disponible en cada proyecto en su máquina. Para limitarlo a un proyecto, muévalo al directorio `.claude/agents/` de ese proyecto. [Elija el alcance del subagente](#choose-the-subagent-scope) compara los dos.
  </Step>

  <Step title="Pruébelo">
    Pida a Claude que delegue en el nuevo subagente:

    ```text wrap theme={null}
    Use the code-improver agent to suggest improvements in this project
    ```

    Claude delega en su nuevo subagente, que escanea la base de código y devuelve sugerencias de mejora. En la transcripción, la delegación aparece como una fila de llamada de herramienta que muestra el nombre del subagente seguido de una breve descripción de la tarea, como `code-improver(Suggest code improvements)`.

    Si Claude no puede encontrar el nuevo subagente, reinicie Claude Code e intente de nuevo. Esto sucede solo cuando `~/.claude/agents/` no existía antes de que la sesión comenzara, porque una sesión en ejecución no detecta un directorio `agents` recién creado.
  </Step>
</Steps>

Ahora tiene un subagente que puede usar en cualquier proyecto en su máquina para analizar bases de código y sugerir mejoras.

También puede escribir archivos de subagentes a mano, definirlos mediante banderas CLI, o distribuirlos a través de plugins. Las siguientes secciones cubren todas las opciones de configuración.

<Note>
  En Claude Code v2.1.197 y anteriores, `/agents` abre un asistente interactivo con una pestaña **Running** que enumera los subagentes activos y una pestaña **Library** para crearlos, editarlos y eliminarlos.&#x20;
</Note>

<h2 id="configure-subagents">
  Configurar subagentes
</h2>

La ubicación del archivo de un subagente determina quién tiene acceso a él, y su frontmatter determina qué puede hacer. Esta sección cubre dónde viven los archivos de subagentes y cada campo que soportan.

<h3 id="choose-the-subagent-scope">
  Elegir el alcance del subagente
</h3>

Almacene archivos de subagentes en diferentes ubicaciones según el alcance. Cuando múltiples subagentes comparten el mismo nombre, Claude Code usa el de la ubicación de mayor prioridad.

| Ubicación                       | Alcance                         | Prioridad    | Cómo crear                                                          |
| :------------------------------ | :------------------------------ | :----------- | :------------------------------------------------------------------ |
| Configuración administrada      | Toda la organización            | 1 (más alta) | Implementado a través de [configuración administrada](/docs/es/settings) |
| Bandera CLI `--agents`          | Sesión actual                   | 2            | Pasar JSON al lanzar Claude Code                                    |
| `.claude/agents/`               | Proyecto actual                 | 3            | Pedir a Claude, o crear el archivo manualmente                      |
| `~/.claude/agents/`             | Todos sus proyectos             | 4            | Pedir a Claude, o crear el archivo manualmente                      |
| Directorio `agents/` del plugin | Donde el plugin está habilitado | 5 (más baja) | Instalado con [plugins](/docs/es/plugins/overview)                       |

**Los subagentes de proyecto** (`.claude/agents/`) son ideales para subagentes específicos de una base de código. Verifíquelos en control de versiones para que su equipo pueda usarlos y mejorarlos colaborativamente.

Los subagentes de proyecto se descubren caminando hacia arriba desde el directorio de trabajo actual, por lo que cada `.claude/agents/` entre allí y la raíz del repositorio se escanea. Cuando más de uno de estos directorios anidados define el mismo `name`, Claude Code usa la definición más cercana al directorio de trabajo.

Cuando agrega un directorio con `--add-dir` o `/add-dir`, Claude Code también carga su carpeta `.claude/agents/`, junto con sus subagentes de proyecto. Consulte [Directorios adicionales](/docs/es/permissions#additional-directories-grant-file-access-not-configuration) para ver qué otros tipos de configuración se cargan desde `--add-dir`. Para compartir subagentes entre proyectos sin `--add-dir`, use `~/.claude/agents/` o un [plugin](/docs/es/plugins/overview).

**Los subagentes de usuario** (`~/.claude/agents/`) son subagentes personales disponibles en todos sus proyectos.

Claude Code escanea `.claude/agents/` y `~/.claude/agents/` recursivamente, por lo que puede organizar definiciones en subcarpetas como `agents/review/` o `agents/research/`. La ruta del subdirectorio no afecta cómo se identifica o invoca un subagente, porque la identidad proviene solo del campo `name` del frontmatter.

Mantenga los valores de `name` únicos en todo el árbol: si dos archivos bajo el mismo directorio `.claude/agents/`, incluyendo sus subcarpetas, declaran el mismo nombre, Claude Code carga solo uno de ellos, elegido por orden de lectura del sistema de archivos en lugar de una precedencia documentada. En directorios de proyecto anidados, la definición más cercana al directorio de trabajo gana, como se describe arriba. El chequeo de configuración [`/doctor`](/docs/es/commands#all-commands) reporta archivos en el mismo directorio que comparten un nombre y propone renombrar o eliminar todos excepto uno. Antes de v2.1.205, `/doctor` abría una pantalla de diagnósticos que listaba duplicados y mostraba qué definición estaba activa.

Los directorios `agents/` de plugins también se escanean recursivamente. A diferencia de los alcances de proyecto y usuario, una subcarpeta dentro del directorio `agents/` de un plugin se convierte en parte del [identificador con alcance](#invoke-subagents-explicitly): un archivo en `agents/review/security.md` en el plugin `my-plugin` se registra como `my-plugin:review:security`.

**Los subagentes definidos por CLI** se pasan como JSON al lanzar Claude Code. Existen solo para esa sesión y no se guardan en disco, lo que los hace útiles para pruebas rápidas o scripts de automatización. Puede definir múltiples subagentes en una única llamada `--agents`:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    claude --agents '{
      "code-reviewer": {
        "description": "Expert code reviewer. Use proactively after code changes.",
        "prompt": "You are a senior code reviewer. Focus on code quality, security, and best practices.",
        "tools": ["Read", "Grep", "Glob", "Bash"],
        "model": "sonnet"
      },
      "debugger": {
        "description": "Debugging specialist for errors and test failures.",
        "prompt": "You are an expert debugger. Analyze errors, identify root causes, and provide fixes."
      }
    }'
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    claude --agents @'
    {
      "code-reviewer": {
        "description": "Expert code reviewer. Use proactively after code changes.",
        "prompt": "You are a senior code reviewer. Focus on code quality, security, and best practices.",
        "tools": ["Read", "Grep", "Glob", "Bash"],
        "model": "sonnet"
      },
      "debugger": {
        "description": "Debugging specialist for errors and test failures.",
        "prompt": "You are an expert debugger. Analyze errors, identify root causes, and provide fixes."
      }
    }
    '@
    ```
  </Tab>
</Tabs>

La bandera `--agents` acepta JSON con un campo `prompt` más estos campos de [frontmatter](#supported-frontmatter-fields): `description`, `tools`, `disallowedTools`, `model`, `permissionMode`, `mcpServers`, `hooks`, `maxTurns`, `skills`, `initialPrompt`, `memory`, `effort`, `background`, `omitClaudeMd`, e `isolation`. Use `prompt` para el mensaje del sistema, equivalente al cuerpo markdown en subagentes basados en archivos. `color` y `experimental` no se aceptan aquí y se ignoran en lugar de rechazarse.

Cada clave de nivel superior en el JSON es el nombre del agente. No comience un nombre con `-`.

Para lo que Claude Code hace con un valor que no puede cargar, y las banderas y variable de entorno que omiten esa verificación, consulte [`Configuración de --agents inválida`](/docs/es/errors#invalid-agents-configuration).

**Los subagentes administrados** son implementados por administradores de la organización. Coloque archivos markdown en `.claude/agents/` dentro del [directorio de configuración administrada](/docs/es/managed-settings#delivery-mechanisms), usando el mismo formato de frontmatter que los subagentes de proyecto y usuario. Las definiciones administradas tienen precedencia sobre los subagentes de proyecto y usuario con el mismo nombre.

**Los subagentes de plugin** provienen de [plugins](/docs/es/plugins/overview) que ha instalado. Se cargan automáticamente junto a sus subagentes personalizados y aparecen en la lista de @-mention bajo su nombre con alcance. Consulte la [referencia de componentes de plugin](/docs/es/plugins/components#agents) para obtener detalles sobre la creación de subagentes de plugin.

<Note>
  Por razones de seguridad, los subagentes de plugin no soportan los campos de frontmatter `hooks`, `mcpServers`, o `permissionMode`. Estos campos se ignoran al cargar agentes desde un plugin. Si los necesita, copie el archivo del agente en `.claude/agents/` o `~/.claude/agents/`. También puede agregar reglas a [`permissions.allow`](/docs/es/settings-reference#permissions-allow) en `settings.json` o `settings.local.json`, pero estas reglas se aplican a toda la sesión, no solo al subagente del plugin.
</Note>

Las definiciones de subagentes de cualquiera de estos alcances también están disponibles para [equipos de agentes](/docs/es/agent-teams#use-subagent-definitions-for-teammates): al generar un compañero de equipo, puede hacer referencia a un tipo de subagente, y Claude Code aplica partes de esa definición al compañero. Consulte [equipos de agentes](/docs/es/agent-teams#use-subagent-definitions-for-teammates) para ver qué partes se aplican en cada modo de visualización.

<h3 id="write-subagent-files">
  Escribir archivos de subagentes
</h3>

Los archivos de subagentes usan frontmatter YAML para configuración, seguido del mensaje del sistema en Markdown:

<Note>
  Claude Code observa `~/.claude/agents/` y `.claude/agents/`. Cuando agrega o edita un archivo de subagente en disco, o pide a Claude que escriba uno para usted, Claude Code detecta el cambio dentro de unos pocos segundos y la siguiente delegación usa la definición actualizada, sin necesidad de reinicio.

  Tres casos aún necesitan un reinicio:

  * El observador cubre solo directorios que existían cuando comenzó la sesión, por lo que después de crear el primer archivo de agente de un alcance en un nuevo directorio `agents`, reinicie para cargarlo.
  * Claude Code no observa `.claude/agents/` dentro de directorios agregados con `--add-dir` o `/add-dir`, por lo que después de agregar o editar un subagente allí, reinicie para cargar el cambio.
  * Las sesiones iniciadas con `--disable-slash-commands` no observan estos directorios en absoluto.
</Note>

```markdown .claude/agents/code-reviewer.md theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
tools: Read, Glob, Grep
model: sonnet
---

You are a code reviewer. When invoked, analyze the code and provide
specific, actionable feedback on quality, security, and best practices.
```

El frontmatter define los metadatos y la configuración del subagente. El cuerpo se convierte en el mensaje del sistema que guía el comportamiento del subagente. Los subagentes reciben solo este mensaje del sistema más detalles básicos del entorno como el directorio de trabajo, no el mensaje del sistema de Claude Code.

En [modo no interactivo](/docs/es/headless), pase [`--append-subagent-system-prompt`](/docs/es/cli-reference#cli-flags) para anexar su texto al final del mensaje del sistema de cada subagente, incluyendo subagentes anidados, aparte de un [subagente bifurcado](#fork-the-current-conversation), que reutiliza el mensaje del sistema de la conversación. Requiere Claude Code v2.1.205 o posterior. Si su texto es demasiado largo para pasar en la línea de comandos, guárdelo en un archivo y pase la ruta con `--append-subagent-system-prompt-file` en su lugar. La bandera de archivo requiere Claude Code v2.1.261 o posterior.

Un subagente comienza en el directorio de trabajo actual de la conversación principal. Dentro de un subagente, los comandos `cd` no persisten entre llamadas de herramientas Bash o PowerShell y no afectan el directorio de trabajo de la conversación principal. Para dar al subagente una copia aislada del repositorio en su lugar, establezca [`isolation: worktree`](#supported-frontmatter-fields).

Un subagente con `isolation: worktree` ejecuta sus comandos Bash y PowerShell dentro de su worktree. Un comando cuyo directorio de trabajo se resuelve a su checkout principal en su lugar, por ejemplo porque el directorio worktree fue eliminado mientras el subagente estaba ejecutándose, falla con un error. Antes de v2.1.203, tal comando podría ejecutarse en el checkout principal.

Esta verificación de directorio de trabajo cubre todo el repositorio que contiene el directorio desde el que lanzó Claude Code. Cuando su sesión se ejecuta en un [worktree](/docs/es/worktrees) vinculado de su propiedad, la verificación también cubre el checkout principal desde el que ese worktree está vinculado. Antes de v2.1.210, la verificación cubría solo el directorio de lanzamiento en sí. Un comando cuyo directorio de trabajo se resolvía en otro lugar en el mismo repositorio, como la raíz del repositorio cuando lanzó Claude Code desde un subdirectorio de monorepo, se ejecutaba allí en lugar de fallar.

Para comandos Bash, Claude Code también verifica el comando en sí de dos maneras:

* Bloquea un comando que redirige git al checkout principal.
* Se niega a un comando cuando no puede verificar desde el texto del comando que cualquier git que el comando ejecute permanece dentro del worktree, por ejemplo cuando el nombre del comando se calcula en tiempo de ejecución.

Los vectores de redirección y las reglas de forma se enumeran en [Cómo Claude Code aplica el aislamiento](/docs/es/worktrees#how-claude-code-enforces-isolation). Los comandos PowerShell obtienen solo la verificación de directorio de trabajo.

Los comandos [Monitor](/docs/es/tools-reference#monitor-tool) pasan por las mismas verificaciones de directorio de trabajo y contenido de comando que los comandos Bash.

Cuando la conversación principal en sí se ejecuta aislada en un worktree, Claude Code aplica las mismas verificaciones a la sesión y a cada subagente que genera, incluyendo subagentes sin `isolation: worktree`; consulte [Cómo Claude Code aplica el aislamiento](/docs/es/worktrees#how-claude-code-enforces-isolation).

<h3 id="supported-frontmatter-fields">
  Referencia de frontmatter
</h3>

Configure un subagente con [frontmatter](/docs/es/glossary#frontmatter) YAML entre marcadores `---` en la parte superior de su archivo, y escriba su mensaje del sistema como Markdown después del `---` de cierre. Solo `name` y `description` son requeridos.

Los nombres de campo de varias palabras usan camelCase, como `maxTurns` y `disallowedTools`, y deben coincidir exactamente con la tabla: Claude Code ignora un campo que no reconoce sin reportar un error. Para averiguar por qué un archivo de subagente no se cargó, consulte [Archivos de subagente que Claude Code omite](#subagent-files-claude-code-skips).

| Campo             | Requerido | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :---------------- | :-------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`            | Sí        | Identificador único, como `code-reviewer` o `reviewer-v2`. [Hooks](/docs/es/hooks#subagentstart) reciben este valor como `agent_type`. El nombre del archivo no tiene que coincidir. Los nombres no pueden contener `:`, que está reservado para [identificadores con alcance de plugin](/docs/es/plugins/overview) como `my-plugin:reviewer`. Claude Code no carga un archivo cuyo nombre contiene uno y registra un error en el registro de depuración. Antes de v2.1.218, tales nombres fueron aceptados                                                                  |
| `description`     | Sí        | Cuándo Claude debe delegar en este subagente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `tools`           | No        | [Herramientas](#available-tools) que el subagente puede usar, como una cadena separada por comas como `Read, Grep, Bash` o una lista YAML. Hereda todas las herramientas disponibles para subagentes si se omite. Si ninguna entrada en la lista se resuelve a una herramienta, el subagente generalmente [falla al lanzarse](/docs/es/errors#agent-would-be-spawned-with-zero-tools) con un error nombrando las entradas. Para precargar Skills en el contexto, use el campo `skills` en lugar de listar `Skill` aquí                                                  |
| `disallowedTools` | No        | Herramientas a denegar, eliminadas de la lista heredada o especificada. Mismo formato que `tools`. Una entrada con un especificador, como `Bash(git push *)`, aún [elimina la herramienta completa](#available-tools)                                                                                                                                                                                                                                                                                                                                              |
| `model`           | No        | [Modelo](#choose-a-model) a usar: `sonnet`, `opus`, `haiku`, `fable`, un ID de modelo completo como `claude-opus-5-5`, o `inherit`. Cuando lo omite, Claude Code elige el modelo en el [orden de modelo de subagente](#choose-a-model)                                                                                                                                                                                                                                                                                                                             |
| `permissionMode`  | No        | [Modo de permiso](#permission-modes): `default`, `acceptEdits`, `auto`, `dontAsk`, `bypassPermissions`, `plan`, o `manual` como alias para `default`. El alias `manual` requiere Claude Code v2.1.200 o posterior. Se ignora para [subagentes de plugin](#choose-the-subagent-scope)                                                                                                                                                                                                                                                                               |
| `maxTurns`        | No        | Número máximo de turnos de agente antes de que el subagente se detenga. Cuando el subagente alcanza el límite, Claude Code devuelve su salida marcada como parcial, y Claude puede [reanudarlo](#resume-subagents) para continuar. La marca parcial requiere Claude Code v2.1.246 o posterior                                                                                                                                                                                                                                                                      |
| `skills`          | No        | [Skills](/docs/es/skills) a precargar en el contexto del subagente al inicio. El contenido completo de la skill se inyecta, no solo la descripción. Los subagentes aún pueden invocar skills de proyecto, usuario y plugin no listadas a través de la herramienta Skill                                                                                                                                                                                                                                                                                                 |
| `mcpServers`      | No        | [Servidores MCP](/docs/es/mcp) disponibles para este subagente. Cada entrada es un nombre de servidor que hace referencia a un servidor ya configurado (por ejemplo, `"slack"`) o una definición en línea con el nombre del servidor como clave y una [configuración completa del servidor MCP](/docs/es/mcp#installing-mcp-servers) como valor. Se ignora para [subagentes de plugin](#choose-the-subagent-scope)                                                                                                                                                           |
| `hooks`           | No        | [Hooks de ciclo de vida](#define-hooks-for-subagents) limitados a este subagente. Se ignora para [subagentes de plugin](#choose-the-subagent-scope)                                                                                                                                                                                                                                                                                                                                                                                                                |
| `memory`          | No        | [Alcance de memoria persistente](#enable-persistent-memory): `user`, `project`, o `local`. Habilita aprendizaje entre sesiones                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `background`      | No        | Establecer en `true` para mantener este subagente en segundo plano incluso cuando Claude pide ejecutarlo en primer plano. Donde [fork mode](#turn-fork-mode-on-or-off) está activado, Claude Code ya ejecuta los subagentes que Claude genera [en segundo plano](#run-subagents-in-foreground-or-background)                                                                                                                                                                                                                                                       |
| `omitClaudeMd`    | No        | Establecer en `true` para lanzar este subagente sin los archivos CLAUDE.md de usuario, proyecto y local; [archivos de política administrada](/docs/es/memory#how-claude-md-files-load) aún se cargan, excepto para [subagentes administrados](#choose-the-subagent-scope). Úselo para subagentes que toman todo lo que necesitan del [mensaje de delegación](#what-loads-at-startup). Se ignora cuando el agente se ejecuta como el agente de sesión principal a través de `--agent` o la configuración `agent`. Requiere Claude Code v2.1.271 o posterior              |
| `effort`          | No        | Nivel de esfuerzo cuando este subagente está activo. Anula el nivel de esfuerzo de la sesión. Por defecto: hereda de la sesión. Opciones: `low`, `medium`, `high`, `xhigh`, `max`; los niveles disponibles dependen del modelo                                                                                                                                                                                                                                                                                                                                     |
| `isolation`       | No        | Establecer en `worktree` para ejecutar el subagente en un [git worktree](/docs/es/worktrees) temporal, dándole una copia aislada del repositorio ramificada por defecto desde su [rama predeterminada](/docs/es/worktrees#choose-the-base-branch) en lugar del `HEAD` de la sesión principal. El worktree se limpia automáticamente si el subagente no realiza cambios                                                                                                                                                                                                       |
| `color`           | No        | Color de visualización para el subagente en la lista de tareas y transcripción. Acepta `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink`, o `cyan`                                                                                                                                                                                                                                                                                                                                                                                                      |
| `initialPrompt`   | No        | Se envía automáticamente como el primer turno de usuario cuando este agente se ejecuta como el agente de sesión principal (a través de `--agent` o la configuración `agent`). Se procesan [comandos](/docs/es/commands) y [skills](/docs/es/skills). Se antepone a cualquier mensaje proporcionado por el usuario. Se ignora para [subagentes de plugin](#choose-the-subagent-scope)                                                                                                                                                                                         |
| `experimental`    | No        | Mapa de opciones experimentales. Establezca su clave `cacheTtl` en `5m` o `1h` para elegir la [duración de vida del caché de prompt](/docs/es/prompt-caching#choose-the-ttl-yourself) para las solicitudes de este subagente, en el lugar del frontmatter en la [precedencia de duración de vida del caché](/docs/es/prompt-caching#choose-the-ttl-yourself). Claude Code ignora cualquier otro valor, ignora `1h` mientras su suscripción de Claude está usando créditos de uso, y lee el campo solo desde archivos de subagente. Requiere Claude Code v2.1.248 o posterior |

Escriba `cacheTtl` dentro del mapa `experimental`, no en el nivel superior del frontmatter.

```yaml theme={null}
---
name: repo-auditor
description: Audits a large repository and reports what it finds
experimental:
  cacheTtl: 1h
---
```

<h4 id="subagent-files-claude-code-skips">
  Archivos de subagente que Claude Code omite
</h4>

Claude Code omite un archivo en un directorio `agents` de proyecto, usuario o administrado, o en uno bajo un directorio que agrega con `--add-dir`, sin reportarlo en la sesión, cuando el frontmatter tiene alguno de estos problemas:

* **Sin `name`**: Claude Code trata el archivo como documentación guardada junto a sus agentes.
* **Un `---` de apertura que no es la primera línea del archivo**: Claude Code lee el archivo como si no tuviera frontmatter y lo trata como documentación.
* **Un `name` que comienza con `-` o contiene `:`**: Claude Code omite el archivo y escribe un error en el registro de depuración. Consulte la fila `name` en la tabla anterior.
* **Un `name` pero sin `description`**: Claude Code omite el archivo y escribe la razón en el registro de depuración.
* **YAML que no se analiza**: Claude Code no lee campos del archivo, lo omite y escribe el error de análisis en el registro de depuración.

Para ver el registro de depuración, ejecute Claude Code con `--debug`.

Un [subagente de plugin](/docs/es/plugins/components#agents) cuyo frontmatter no tiene `name` o no se analiza aún se carga, bajo su nombre de archivo.

<h5 id="check-an-agents-directory-before-a-session">
  Verificar un directorio `agents` antes de una sesión
</h5>

Para encontrar archivos en un directorio `agents` cuyo frontmatter no se analiza, ejecute `claude plugin validate` contra el directorio, por ejemplo `.claude/agents` o `~/.claude/agents`. Claude Code verifica solo [el directorio que nombra](/docs/es/plugins/cli-reference#validate-a-directory), y no marca un archivo cuyo frontmatter se analiza pero no tiene `name`. Requiere Claude Code v2.1.233 o posterior.

<h3 id="choose-a-model">
  Elegir un modelo
</h3>

El campo `model` controla qué modelo usa el subagente:

* **Alias de modelo**: use uno de los alias disponibles: `sonnet`, `opus`, `haiku`, o `fable`
* **ID de modelo completo**: use un ID de modelo completo como `claude-opus-5-5` o `claude-sonnet-5`. Acepta los mismos valores que la bandera `--model`
* **inherit**: use el mismo modelo que la conversación principal

Cuando Claude invoca un subagente, también puede pasar un parámetro `model` para esa invocación específica. Claude Code resuelve el modelo del subagente en este orden:

1. El parámetro `model` por invocación
2. El frontmatter `model` de la definición del subagente, donde `inherit` selecciona el modelo de la conversación principal
3. La variable de entorno [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/es/model-config#environment-variables), cuando la establece en un alias de modelo o ID de modelo
4. El modelo de la conversación principal

En dos casos, un alias de familia como `opus` en el parámetro por invocación o el frontmatter se resuelve al modelo de la conversación principal en lugar de la [versión a la que apunta el alias](/docs/es/model-config#model-aliases):

* **El modelo de la conversación principal pertenece a esa familia**: el subagente se ejecuta en el modelo exacto de la conversación principal, incluyendo cualquier sufijo `[1m]`, por lo que obtiene la misma ventana de [contexto extendido](/docs/es/model-config#extended-context) que la conversación principal.
* **Claude Code no puede determinar la familia del modelo de la conversación principal, en [un proveedor distinto de la API de Anthropic](/docs/es/third-party-integrations)**: esto puede suceder con un [ARN de perfil de inferencia de aplicación](/docs/es/amazon-bedrock#iam-configuration) en Amazon Bedrock que Claude Code no ha resuelto a un modelo de respaldo. Este caso cubre solo el alias `opus`, y no se aplica cuando establece [`ANTHROPIC_DEFAULT_OPUS_MODEL`](/docs/es/model-config#environment-variables), ya que `opus` entonces se resuelve al modelo que establece.

Un alias en `CLAUDE_CODE_SUBAGENT_MODEL` siempre se resuelve a la versión a la que apunta el alias, incluso cuando nombra la familia de la conversación principal.

Establecer `CLAUDE_CODE_SUBAGENT_MODEL` por sí solo no cambia el modelo en el que se ejecutan los subagentes integrados Explore y Plan. Para cambiarlo, consulte [Ejecutar cada subagente en un modelo](#run-every-subagent-on-one-model).

Antes de v2.1.251, `CLAUDE_CODE_SUBAGENT_MODEL` venía primero en este orden y anulaba tanto el parámetro por invocación como el frontmatter, incluyendo `model: inherit`.

Establecer la variable en `inherit` es lo mismo que dejarla sin establecer. Antes de v2.1.196, ese valor forzaba subagentes al modelo de la conversación principal e ignoraba las otras fuentes.

Claude Code verifica el parámetro por invocación, frontmatter y valores de variable de entorno contra la lista de permitidos [`availableModels`](/docs/es/model-config#restrict-model-selection) de su organización. Para un valor bloqueado, sustituye otro modelo:

* Cuando el valor bloqueado es un alias de familia como `opus`, Claude Code ejecuta el subagente en la versión más nueva de esa familia que la lista de permitidos permite, siguiendo las mismas [reglas de sustitución y alcance de proveedor](/docs/es/model-config#restrict-model-selection) que `/model`. Antes de v2.1.222, Claude Code ejecutaba el subagente en el modelo heredado para un alias de familia bloqueado también.
* Para cualquier otro valor bloqueado, en proveedores donde esa sustitución no opera, o cuando la lista de permitidos no permite ninguna versión de la familia, Claude Code ejecuta el subagente en el modelo heredado en su lugar. Si establece `CLAUDE_CODE_SUBAGENT_MODEL`, Claude Code intenta ese modelo primero, bajo estas mismas reglas.

En sesiones interactivas, Claude Code muestra una advertencia nombrando el modelo solicitado y el modelo en el que se ejecuta el subagente, para cualquiera de las sustituciones.

Para verificar qué modelo está ejecutando un subagente, ejecute [`/tasks`](/docs/es/commands). Claude Code nombra el modelo en la fila del subagente, y agrega el [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) cuando la definición del subagente, o la skill de la que se bifurcó, establece [`effort`](#supported-frontmatter-fields). Requiere Claude Code v2.1.242 o posterior.

Un parámetro `model` por invocación también se aplica cuando el subagente se [reanuda o se le envía un mensaje de seguimiento](#resume-subagents), por lo que el subagente permanece en ese modelo. Antes de v2.1.211, reanudar eliminaba el valor por invocación y el subagente revertía al campo `model` de su definición o, sin uno, al modelo de la conversación principal.

A partir de v2.1.198, los subagentes también heredan la configuración de [pensamiento extendido](/docs/es/model-config#extended-thinking) de la conversación principal: si el pensamiento está activado en su sesión, está activado para el subagente, y si está desactivado, permanece desactivado. No hay una configuración de pensamiento por subagente. Antes de v2.1.198, los subagentes se ejecutaban con pensamiento extendido deshabilitado independientemente de la configuración de la conversación principal.

<h4 id="run-every-subagent-on-one-model">
  Ejecutar cada subagente en un modelo
</h4>

`CLAUDE_CODE_SUBAGENT_MODEL` es un valor predeterminado, por lo que la definición de un subagente o un modelo que Claude pasa aún tiene precedencia sobre él. Para aplicar un modelo a cada subagente, [compañero de equipo](/docs/es/agent-teams#specify-teammates-and-models), y [agente de flujo de trabajo](/docs/es/workflows), también establezca `CLAUDE_CODE_SUBAGENT_MODEL_FORCE` en `1`. Requiere Claude Code v2.1.257 o posterior.

* Si establece ambas variables, los subagentes se ejecutan en el modelo en `CLAUDE_CODE_SUBAGENT_MODEL`.
* Si establece solo `CLAUDE_CODE_SUBAGENT_MODEL_FORCE`, los subagentes se ejecutan en el modelo de la conversación principal.

Por ejemplo, para ejecutar cada subagente en Haiku, establezca ambas variables en el bloque `env` de un [archivo de configuración](/docs/es/settings):

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_SUBAGENT_MODEL": "haiku",
    "CLAUDE_CODE_SUBAGENT_MODEL_FORCE": "1"
  }
}
```

Para verificar que la configuración surtió efecto, ejecute [`/tasks`](/docs/es/commands) mientras se ejecuta un subagente. La fila del subagente muestra el modelo en el que se ejecuta.

Mientras `CLAUDE_CODE_SUBAGENT_MODEL_FORCE` está [activado](/docs/es/env-vars), Claude Code ignora el campo `model` de cada definición de subagente, incluyendo los subagentes integrados Explore y Plan, y Claude no puede pasar un modelo cuando inicia un subagente. Dos tipos de subagente aún se ejecutan en el modelo de la conversación principal:

* Una [bifurcación](#fork-the-current-conversation)
* Una [skill que se ejecuta en un subagente](/docs/es/skills#run-skills-in-a-subagent) con `model: inherit`

Cuando establece solo `CLAUDE_CODE_SUBAGENT_MODEL_FORCE`, el subagente integrado Explore mantiene su [límite de modelo](#built-in-subagents).

<h3 id="control-subagent-capabilities">
  Controlar capacidades de subagentes
</h3>

Puede controlar qué pueden hacer los subagentes a través del acceso a herramientas, modos de permisos y reglas condicionales.

<h4 id="available-tools">
  Herramientas disponibles
</h4>

Los subagentes heredan las [herramientas integradas](/docs/es/tools-reference) y herramientas MCP disponibles en la conversación principal, reducidas por dos filtros: el primero elimina una lista corta de herramientas de cada subagente, y el segundo reduce el conjunto de herramientas integradas para subagentes que se ejecutan en [segundo plano](#run-subagents-in-foreground-or-background), que es el predeterminado. En macOS, Linux y WSL, un subagente también puede recibir las herramientas Glob y Grep cuando la conversación principal no las tiene, como se describe en [Comportamiento de la herramienta Glob](/docs/es/tools-reference#glob-tool-behavior). [Las bifurcaciones](#fork-the-current-conversation) omiten ambos filtros y reciben el grupo de herramientas exacto de la conversación principal. El primer filtro elimina estas herramientas, incluso cuando se enumeran en el campo `tools`:

* `Agent`, cuando el subagente está en el [límite de profundidad](#let-subagents-spawn-their-own-subagents); en una [bifurcación](#fork-the-current-conversation) la herramienta permanece listada pero devuelve un error en lugar de generar
* `AskUserQuestion`
* `EndConversation`, que solo puede terminar la conversación principal; consulte [Comportamiento de la herramienta EndConversation](/docs/es/tools-reference#endconversation-tool-behavior)
* `EnterPlanMode`
* `ExitPlanMode`, a menos que el [`permissionMode`](#permission-modes) del subagente sea `plan`
* `ScheduleWakeup`
* `WaitForMcpServers`
* `Workflow`

El segundo filtro se aplica a subagentes que se ejecutan en segundo plano. Aparte de `Agent` y `ExitPlanMode`, que siguen las condiciones del primer filtro dondequiera que se ejecute el subagente, un subagente en segundo plano mantiene todas las herramientas MCP pero solo estas herramientas integradas: `Read`, `Grep`, `Glob`, `LSP`, `Bash`, `PowerShell`, `Edit`, `Write`, `NotebookEdit`, `WebFetch`, `WebSearch`, `TodoWrite`, `Skill`, `ToolSearch`, `EnterWorktree`, `ExitWorktree`, `Monitor`, `TaskStop`, `SendMessage`, y `Artifact`, más [`SubagentHandback`](/docs/es/tools-reference) para un subagente que reporta a través de él. Claude Code elimina todas las otras herramientas integradas de un subagente en segundo plano, ya sean heredadas o listadas en el campo `tools`, por lo que la misma definición puede resolver a diferentes herramientas en primer plano y en segundo plano. La eliminación no reporta error a menos que deje la lista `tools` [resolviendo a nada](/docs/es/errors#agent-would-be-spawned-with-zero-tools).

Antes de v2.1.280, los subagentes en segundo plano no podían usar `LSP`.

[`ListAgents`](/docs/es/cross-session-messaging) sigue estos filtros como cualquier herramienta integrada: un subagente en primer plano la hereda en sesiones donde la mensajería entre sesiones está habilitada, y un subagente en segundo plano no la mantiene.

Los compañeros de equipo en [equipos de agentes](/docs/es/agent-teams) además mantienen las herramientas de tareas y herramientas cron: `TaskCreate`, `TaskGet`, `TaskList`, `TaskUpdate`, `CronCreate`, `CronDelete`, y `CronList`.

En una [sesión sin las herramientas Task](/docs/es/tools-reference#task-tool-availability), Claude Code no proporciona las herramientas de tareas a los subagentes tampoco, incluso cuando el subagente ejecuta un modelo diferente. Un compañero en proceso sigue su sesión de la misma manera, mientras que un compañero en su propio [panel dividido](/docs/es/agent-teams#choose-a-display-mode) se ejecuta como un proceso Claude Code separado, por lo que su propio modelo decide.

Para restringir herramientas, use el campo `tools` como una lista blanca o el campo `disallowedTools` como una lista negra. Este ejemplo usa `tools` para permitir solo Read, Grep, Glob y Bash. El subagente no puede editar archivos, escribir archivos, o usar ninguna herramienta MCP:

```yaml theme={null}
---
name: safe-researcher
description: Research agent with restricted capabilities
tools: Read, Grep, Glob, Bash
---
```

Este ejemplo usa `disallowedTools` para heredar el grupo de herramientas del subagente excepto Write y Edit. El subagente mantiene Bash, herramientas MCP y el resto de su grupo:

```yaml theme={null}
---
name: no-writes
description: Inherits the available tools except file writes
disallowedTools: Write, Edit
---
```

Si ambos se establecen, `disallowedTools` se aplica primero, luego `tools` se resuelve contra el grupo restante. Una herramienta listada en ambos se elimina.

Cuando nada en la lista `tools` se resuelve a una herramienta, por ejemplo porque cada entrada está mal escrita o nombra una herramienta que no está disponible para subagentes, Claude Code generalmente se niega a lanzar el subagente y la herramienta Agent devuelve un error nombrando las entradas no resueltas; consulte [Agent sería generado con cero herramientas](/docs/es/errors#agent-would-be-spawned-with-zero-tools) para el mensaje y cómo corregir cada entrada. Antes de v2.1.208, ese subagente se lanzaba sin herramientas y podría devolver un resultado vacío o confuso.

Ambos campos aceptan patrones a nivel de servidor MCP además de nombres de herramientas exactos: `mcp__<server>` o `mcp__<server>__*` otorga o elimina todas las herramientas del servidor nombrado. En `disallowedTools`, `mcp__*` también elimina todas las herramientas MCP de cualquier servidor. Este ejemplo elimina todas las herramientas del servidor MCP `github` mientras mantiene herramientas de otros servidores y las herramientas integradas en su grupo:

```yaml theme={null}
---
name: local-only
description: Inherits every tool except those from the github MCP server
disallowedTools: mcp__github
---
```

Una entrada `disallowedTools` con un especificador, como `Bash(git push *)`, aún elimina la herramienta completa del subagente, no solo los comandos coincidentes. Para mantener Bash y bloquear comandos específicos, agregue una [regla de negación de Bash](/docs/es/permissions#bash) como `Bash(git push *)` a `permissions.deny` en su configuración. La regla se aplica a la conversación principal y a los subagentes.

<h4 id="restrict-which-subagents-can-be-spawned">
  Restringir qué subagentes pueden ser generados
</h4>

Cuando un agente se ejecuta como el hilo principal con `claude --agent`, puede generar subagentes usando la herramienta Agent. Para restringir qué tipos de subagentes puede generar, use la sintaxis `Agent(agent_type)` en el campo `tools`.

<Note>En la versión 2.1.63, la herramienta Task fue renombrada a Agent. Las referencias existentes a `Task(...)` en configuraciones y definiciones de agentes aún funcionan como alias.</Note>

```yaml theme={null}
---
name: coordinator
description: Coordinates work across specialized agents
tools: Agent(worker, researcher), Read, Bash
---
```

Esta es una lista blanca: solo los subagentes `worker` y `researcher` pueden ser generados. Si el agente intenta generar cualquier otro tipo, la solicitud falla y el agente solo ve los tipos permitidos en su mensaje. Para bloquear agentes específicos mientras se permiten todos los demás, use [`permissions.deny`](#disable-specific-subagents) en su lugar.

Para permitir generar cualquier subagente sin restricciones, use `Agent` sin paréntesis:

```yaml theme={null}
tools: Agent, Read, Bash
```

Si `Agent` se omite completamente de la lista `tools`, el agente no puede generar ningún subagente con la herramienta Agent.

La sintaxis de lista blanca `Agent(agent_type)` se aplica solo a un agente que se ejecuta como el hilo principal con `claude --agent`. En una definición de subagente, listar `Agent` en `tools` permite que ese subagente genere subagentes de su propio mientras el [límite de profundidad](#let-subagents-spawn-their-own-subagents) lo permite, pero cualquier lista de tipos dentro de los paréntesis se ignora.

<h4 id="scope-mcp-servers-to-a-subagent">
  Alcance de servidores MCP a un subagente
</h4>

Use el campo `mcpServers` para dar a un subagente acceso a servidores [MCP](/docs/es/mcp) que no están disponibles en la conversación principal. Los servidores en línea definidos aquí se conectan cuando el subagente comienza, sujetos a la [regla de confianza para la carpeta del archivo del agente](#inline-server-trust), y se desconectan cuando termina. Las referencias de cadena comparten la conexión de la sesión principal.

<Note>
  El campo `mcpServers` se aplica en ambos contextos donde un archivo de agente puede ejecutarse:

  * Como un subagente, generado a través de la herramienta Agent o una @-mención
  * Como la sesión principal, lanzada con [`--agent`](#invoke-subagents-explicitly) o la configuración `agent`

  Cuando el agente es la sesión principal, las definiciones de servidor en línea se conectan al inicio junto con servidores de [`.mcp.json`](/docs/es/mcp) y archivos de configuración, bajo la misma [regla de confianza para la carpeta del archivo del agente](#inline-server-trust). En `/mcp`, un servidor remoto (HTTP o SSE) que ha usado antes puede mostrar el estado [`cached`](/docs/es/mcp#managing-your-servers) en su lugar; Claude Code lo conecta cuando Claude primero llama a una de sus herramientas.
</Note>

Cada entrada en la lista es una definición de servidor en línea o una cadena que hace referencia a un servidor MCP ya configurado en su sesión:

```yaml theme={null}
---
name: browser-tester
description: Tests features in a real browser using Playwright
mcpServers:
  # Inline definition: scoped to this subagent only
  - playwright:
      type: stdio
      command: npx
      args: ["-y", "@playwright/mcp@latest"]
  # Reference by name: reuses an already-configured server
  - github
---

Use the Playwright tools to navigate, screenshot, and interact with pages.
```

Las definiciones en línea usan el mismo esquema que las entradas del servidor `.mcp.json`, con clave del nombre del servidor, y soportan los tipos `stdio`, `http`, `sse` y `ws`.

Para mantener un servidor MCP fuera de la conversación principal por completo y evitar que sus descripciones de herramientas consuman contexto allí, defínalo en línea aquí en lugar de en `.mcp.json`. El subagente obtiene las herramientas; la conversación principal no.

<span id="inline-server-trust" />Claude Code carga un servidor en línea desde un archivo de agente en el directorio `.claude/agents/` de su proyecto, o en el directorio `.claude/agents/` de un directorio agregado con `--add-dir`, solo después de que [confíe en la carpeta de la que provino el archivo del agente](/docs/es/permissions#what-runs-before-you-trust-a-folder). Antes de v2.1.238, Claude Code cargaba estos servidores sin verificar confianza.

* **Confianza que no cuenta**: la confianza de una carpeta principal, y la confianza automática que una sesión `-p` o SDK obtiene para [hooks en archivos de configuración](/docs/es/permissions#what-runs-before-you-trust-a-folder)
* **Hasta entonces**: Claude Code omite cada servidor en línea en ese archivo de agente y escribe la clave exacta `projects["<path>"].hasTrustDialogAccepted` para `~/.claude.json` en el registro de depuración
* **Directorios `--add-dir`**: un directorio fuera del repositorio del espacio de trabajo de confianza necesita su propia entrada de confianza, ya que sus archivos `.claude/agents/` no heredan la confianza del espacio de trabajo

Claude Code carga dos tipos de servidor sin verificar confianza para la carpeta de la que provino el archivo del agente:

* Un nombre que hace referencia a un servidor que ya configuró
* Un servidor en línea en un archivo de agente desde `~/.claude/agents/`, en uno que pasa con `--agents` o la opción `agents` del SDK, o en uno que la configuración administrada proporciona

Las restricciones de MCP que se aplican a la sesión principal también cubren servidores declarados en frontmatter de subagentes:

* [`--strict-mcp-config`](/docs/es/cli-reference) y [`--bare`](/docs/es/cli-reference)
* [Configuración de MCP administrada empresarial](/docs/es/managed-mcp)
* [Políticas `allowedMcpServers` y `deniedMcpServers`](/docs/es/managed-mcp#policy-based-control-with-allowlists-and-denylists)

Cuando uno de estos bloquea un servidor, Claude Code lo omite y muestra una advertencia nombrando los servidores bloqueados.

Las restricciones de configuración administrada se aplican a cada subagente independientemente de cómo se defina. `--strict-mcp-config` no filtra servidores que pase en línea a través de `--agents` o la opción `agents` del SDK, ya que esa es entrada explícita del llamador.

<h4 id="permission-modes">
  Modos de permiso
</h4>

Establezca `permissionMode` para elegir el modo de permiso en el que se ejecuta un subagente. Use los valores de configuración de los modos, por lo que el modo Manual es `default`. Si lo deja sin establecer, el subagente hereda el modo de la conversación principal, que comienza como [modo auto](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) en planes Pro, Max y Team a menos que su configuración u su organización lo cambien.

El modo de permiso de la conversación principal decide si Claude Code usa el valor que establece:

* Cuando la conversación principal está en `bypassPermissions`, `acceptEdits`, o [modo auto](/docs/es/permission-modes#eliminate-prompts-with-auto-mode), el subagente se ejecuta en ese mismo modo y Claude Code ignora el `permissionMode` que establece. Bajo modo auto, el clasificador evalúa las llamadas de herramientas del subagente con las reglas de bloqueo y permiso de la conversación principal. Cuando el subagente termina, el clasificador también revisa su trabajo y su informe final antes de que el informe se entregue, como [Cómo el modo auto maneja subagentes](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) describe.
* Cuando la conversación principal está en modo `default`, `dontAsk`, o `plan`, el subagente se ejecuta en el modo de permiso que establece, excepto `bypassPermissions`. Un subagente que declara `bypassPermissions` mantiene el modo de la conversación principal en su lugar. La excepción `bypassPermissions` requiere Claude Code v2.1.267 o posterior.

`permissionMode` acepta estos valores, y `manual` como alias para `default`:

| Modo                | Comportamiento                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Modo Manual: solicita permiso                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `acceptEdits`       | Aceptar automáticamente ediciones de archivo y comandos comunes del sistema de archivos para rutas en el directorio de trabajo o `additionalDirectories`                                                                                                                                                                                                                                                                                                 |
| `auto`              | [Modo auto](/docs/es/permission-modes#eliminate-prompts-with-auto-mode): un clasificador de fondo revisa comandos y escrituras de directorio protegido                                                                                                                                                                                                                                                                                                        |
| `dontAsk`           | Denegar automáticamente solicitudes de permiso. Las herramientas explícitamente permitidas aún funcionan; `AskUserQuestion`, herramientas MCP marcadas [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool), y herramientas de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools) en sesiones donde esa configuración llega a Claude Code se deniegan incluso si las ha permitido |
| `bypassPermissions` | [Omitir solicitudes de permiso](/docs/es/permission-modes#skip-all-checks-with-bypasspermissions-mode). Un subagente se ejecuta en este modo solo cuando la conversación principal lo hace                                                                                                                                                                                                                                                                    |
| `plan`              | Modo plan (exploración de solo lectura)                                                                                                                                                                                                                                                                                                                                                                                                                  |

<h4 id="preload-skills-into-subagents">
  Precargar skills en subagentes
</h4>

Use el campo `skills` para inyectar contenido de skill en el contexto de un subagente al inicio. Esto da al subagente conocimiento de dominio sin requerir que descubra y cargue skills durante la ejecución.

```yaml theme={null}
---
name: api-developer
description: Implement API endpoints following team conventions
skills:
  - api-conventions
  - error-handling-patterns
---

Implement API endpoints. Follow the conventions and patterns from the preloaded skills.
```

El contenido completo de cada skill listada se inyecta en el contexto del subagente al inicio. Este campo controla qué skills se precargan, no qué skills el subagente puede acceder: sin él, el subagente aún puede descubrir e invocar skills de proyecto, usuario y plugin a través de la herramienta Skill durante la ejecución. Para evitar que un subagente invoque skills en absoluto, omita `Skill` de la lista [`tools`](#available-tools) o agréguelo a `disallowedTools`.

No puede precargar skills que establezcan [`disable-model-invocation: true`](/docs/es/skills#control-who-invokes-a-skill), ya que la precarga se extrae del mismo conjunto de skills que Claude puede invocar. Esto incluye la skill integrada `/verify`: solo usted puede ejecutarla, por lo que tampoco puede ser precargada.

Si una skill listada falta o está deshabilitada, por ejemplo por la política de su organización, Claude Code la omite y registra una advertencia en el registro de depuración.

<Note>
  Esto es lo inverso de [ejecutar una skill en un subagente](/docs/es/skills#run-skills-in-a-subagent). Con `skills` en un subagente, el subagente controla el mensaje del sistema y carga contenido de skill. Con `context: fork` en una skill, el contenido de la skill se inyecta en el agente que especifique. En ambos casos el subagente comienza sin su historial de conversación.
</Note>

<h4 id="enable-persistent-memory">
  Habilitar memoria persistente
</h4>

El campo `memory` da al subagente un directorio persistente que sobrevive entre conversaciones. El subagente usa este directorio para acumular conocimiento con el tiempo, como patrones de base de código, insights de depuración y decisiones arquitectónicas.

```yaml theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
memory: user
---

You are a code reviewer. As you review code, update your agent memory with
patterns, conventions, and recurring issues you discover.
```

Elija un alcance basado en qué tan ampliamente debe aplicarse la memoria:

| Alcance   | Ubicación                                     | Usar cuando                                                                                                  |
| :-------- | :-------------------------------------------- | :----------------------------------------------------------------------------------------------------------- |
| `user`    | `~/.claude/agent-memory/<name-of-agent>/`     | el subagente debe recordar aprendizajes en todos los proyectos                                               |
| `project` | `.claude/agent-memory/<name-of-agent>/`       | el conocimiento del subagente es específico del proyecto y compartible a través de control de versiones      |
| `local`   | `.claude/agent-memory-local/<name-of-agent>/` | el conocimiento del subagente es específico del proyecto pero no debe ser verificado en control de versiones |

La memoria del subagente es parte de [memoria automática](/docs/es/memory#auto-memory): si desactiva la memoria automática, con la configuración `autoMemoryEnabled` o `CLAUDE_CODE_DISABLE_AUTO_MEMORY`, el campo `memory` no tiene efecto y el subagente se lanza sin las instrucciones de memoria o el acceso a la herramienta de memoria descrito a continuación.

Cuando la memoria está habilitada:

* El mensaje del sistema del subagente incluye instrucciones para leer y escribir en el directorio de memoria.
* El mensaje del sistema del subagente también incluye las primeras 200 líneas o 25KB de `MEMORY.md` en el directorio de memoria, lo que sea menor, con instrucciones para curar `MEMORY.md` si excede ese límite.
* Las herramientas Read, Write y Edit se habilitan automáticamente para que el subagente pueda administrar sus archivos de memoria.

<h5 id="persistent-memory-tips">
  Consejos de memoria persistente
</h5>

* `project` es el alcance predeterminado recomendado. Hace que el conocimiento del subagente sea compartible a través de control de versiones.
* Pida al subagente que consulte su memoria antes de comenzar el trabajo: "Review this PR, and check your memory for patterns you've seen before."
* Pida al subagente que actualice su memoria después de completar una tarea: "Now that you're done, save what you learned to your memory." Con el tiempo, esto construye una base de conocimiento que hace que el subagente sea más efectivo.
* Incluya instrucciones de memoria directamente en el archivo markdown del subagente para que mantenga proactivamente su propia base de conocimiento:

  ```markdown theme={null}
  Update your agent memory as you discover codepaths, patterns, library
  locations, and key architectural decisions. This builds up institutional
  knowledge across conversations. Write concise notes about what you found
  and where.
  ```

<h4 id="conditional-rules-with-hooks">
  Reglas condicionales con hooks
</h4>

Para un control más dinámico sobre el uso de herramientas, use hooks `PreToolUse` para validar operaciones antes de que se ejecuten. Esto es útil cuando necesita permitir algunas operaciones de una herramienta mientras bloquea otras.

Este ejemplo crea un subagente que solo permite consultas de base de datos de solo lectura. El hook `PreToolUse` ejecuta el script especificado en `command` antes de que se ejecute cada comando Bash:

```yaml theme={null}
---
name: db-reader
description: Execute read-only database queries
tools: Bash
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-readonly-query.sh"
---
```

Claude Code [pasa la entrada del hook como JSON](/docs/es/hooks#pretooluse-input) a través de stdin a comandos de hook. El script de validación lee este JSON, extrae el comando Bash y [sale con código 2](/docs/es/hooks#exit-code-2-behavior-per-event) para bloquear operaciones de escritura:

```bash theme={null}
#!/bin/bash
# ./scripts/validate-readonly-query.sh

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

# Block SQL write operations (case-insensitive)
if echo "$COMMAND" | grep -iE '\b(INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|TRUNCATE)\b' > /dev/null; then
  echo "Blocked: Only SELECT queries are allowed" >&2
  exit 2
fi

exit 0
```

En macOS y Linux, haga el script ejecutable, o el hook falla en lugar de bloquear nada:

```bash theme={null}
chmod +x ./scripts/validate-readonly-query.sh
```

Para probar la regla, pida al subagente que ejecute una declaración `UPDATE`: el script sale con código 2, Claude Code bloquea el comando, y el subagente ve el mensaje `Blocked: Only SELECT queries are allowed`.

Consulte [Hook input](/docs/es/hooks#pretooluse-input) para el esquema de entrada completo y [códigos de salida](/docs/es/hooks#exit-code-output) para cómo los códigos de salida afectan el comportamiento. En Windows, escriba scripts de hook en PowerShell y agregue `shell: powershell` a la entrada del hook como se muestra en [ejecutar hooks en PowerShell](/docs/es/hooks#windows-powershell-tool).

<h4 id="disable-specific-subagents">
  Deshabilitar subagentes específicos
</h4>

Puede evitar que Claude use subagentes específicos agregándolos a la matriz `deny` en su [configuración](/docs/es/settings-reference#permission-settings). Use el formato `Agent(subagent-name)` donde `subagent-name` coincida con el campo name del subagente.

```json theme={null}
{
  "permissions": {
    "deny": ["Agent(Explore)", "Agent(my-custom-agent)"]
  }
}
```

Esto funciona para subagentes integrados y personalizados. También puede usar la bandera CLI `--disallowedTools`:

```bash theme={null}
claude --disallowedTools "Agent(Explore)"
```

Consulte la [documentación de Permisos](/docs/es/permissions#tool-specific-permission-rules) para más detalles sobre reglas de permisos.

<h3 id="define-hooks-for-subagents">
  Definir hooks para subagentes
</h3>

Los subagentes pueden definir [hooks](/docs/es/hooks) que se ejecutan durante el ciclo de vida del subagente. Hay dos formas de configurar hooks:

* **En el frontmatter del subagente**: defina hooks que se ejecuten solo mientras ese subagente está activo
* **En `settings.json`**: defina hooks a nivel de sesión que también se disparen dentro de subagentes. Los eventos de herramientas como `PreToolUse` y `PostToolUse` se disparan para las llamadas de herramientas del subagente de la misma manera que en la conversación principal, y `SubagentStart` y `SubagentStop` se disparan cuando un subagente comienza o termina

Los hooks de [archivos de configuración, configuración de política administrada y plugins](/docs/es/hooks#hook-locations) todos se aplican dentro de subagentes, por lo que un hook `PreToolUse` en `settings.json` también se ejecuta antes de cada herramienta que usa un subagente.

<h4 id="hooks-in-subagent-frontmatter">
  Hooks en frontmatter de subagentes
</h4>

Defina hooks directamente en el archivo markdown del subagente. Estos hooks solo se ejecutan mientras ese subagente específico está activo y se limpian cuando termina.

<Note>
  Los hooks de frontmatter se disparan cuando el agente se genera como un subagente a través de la herramienta Agent o una @-mención, y cuando el agente se ejecuta como la sesión principal a través de [`--agent`](#invoke-subagents-explicitly) o la configuración `agent`. En el caso de sesión principal, se ejecutan junto con cualquier hook definido en [`settings.json`](/docs/es/hooks).
</Note>

Para permitir que los hooks de frontmatter de un subagente a nivel de proyecto se ejecuten, acepte el [diálogo de confianza del espacio de trabajo](/docs/es/permissions#project-allow-rules-and-workspace-trust) para la carpeta que contiene el archivo del agente. Los hooks de subagentes a nivel de usuario en `~/.claude/agents/` y de definiciones que pasa con `--agents` se ejecutan sin este paso. Si agregó una carpeta con `--add-dir` desde fuera del repositorio del espacio de trabajo de confianza, confíe en esa carpeta por separado: sus hooks `.claude/agents/` no heredan la concesión del espacio de trabajo.

Hasta que confíe en la carpeta, el subagente aún se ejecuta, pero Claude Code omite sus hooks de frontmatter y registra un error en el registro de depuración explicando cómo confiar en la carpeta. Esta es una regla más estricta que la de los hooks en archivos de configuración: confiar en una carpeta principal no es suficiente, y una sesión `-p` no cuenta como de confianza. [Lo que se ejecuta antes de confiar en una carpeta](/docs/es/permissions#what-runs-before-you-trust-a-folder) compara los dos. Antes de v2.1.218, los hooks de frontmatter podían ejecutarse desde carpetas que no había confiado, incluyendo en sesiones no interactivas.

Se soportan todos los [eventos de hook](/docs/es/hooks#hook-events). Los eventos más comunes para subagentes son:

| Evento        | Entrada del matcher   | Cuándo se dispara                                                                |
| :------------ | :-------------------- | :------------------------------------------------------------------------------- |
| `PreToolUse`  | Nombre de herramienta | Antes de que el subagente use una herramienta                                    |
| `PostToolUse` | Nombre de herramienta | Después de que el subagente usa una herramienta                                  |
| `Stop`        | (ninguno)             | Cuando el subagente termina (convertido a `SubagentStop` en tiempo de ejecución) |

Este ejemplo valida comandos Bash con el hook `PreToolUse` y ejecuta un linter después de ediciones de archivo con `PostToolUse`:

```yaml theme={null}
---
name: code-reviewer
description: Review code changes with automatic linting
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-command.sh $TOOL_INPUT"
  PostToolUse:
    - matcher: "Edit|Write"
      hooks:
        - type: command
          command: "./scripts/run-linter.sh"
---
```

Cuando el agente se invoca como un subagente, los hooks `Stop` en frontmatter se convierten automáticamente a eventos `SubagentStop`.

<h4 id="project-level-hooks-for-subagent-events">
  Hooks a nivel de proyecto para eventos de subagentes
</h4>

Configure hooks en `settings.json` que respondan a eventos de ciclo de vida de subagentes en la sesión principal.

| Evento          | Entrada del matcher      | Cuándo se dispara                         |
| :-------------- | :----------------------- | :---------------------------------------- |
| `SubagentStart` | Nombre de tipo de agente | Cuando un subagente comienza la ejecución |
| `SubagentStop`  | Nombre de tipo de agente | Cuando un subagente se completa           |

Ambos eventos soportan matchers para dirigirse a tipos de agentes específicos por nombre. El valor del matcher es el `name` del frontmatter del agente para subagentes a nivel de proyecto y usuario, o el identificador con alcance de plugin como `my-plugin:db-agent` para [subagentes de plugin](/docs/es/plugins/components#agents). Un nombre con alcance contiene dos puntos, por lo que se evalúa como una [expresión regular sin anclar](/docs/es/hooks#matcher-patterns); anclarlo con `^` y `$`, como en `^my-plugin:db-agent$`, para coincidir solo con ese agente.

Este ejemplo ejecuta un script de configuración solo cuando el subagente `db-agent` comienza, y un script de limpieza cuando cualquier subagente se detiene:

```json theme={null}
{
  "hooks": {
    "SubagentStart": [
      {
        "matcher": "db-agent",
        "hooks": [
          { "type": "command", "command": "./scripts/setup-db-connection.sh" }
        ]
      }
    ],
    "SubagentStop": [
      {
        "hooks": [
          { "type": "command", "command": "./scripts/cleanup-db-connection.sh" }
        ]
      }
    ]
  }
}
```

Un matcher con guiones como `db-agent` coincide exactamente en Claude Code v2.1.195 o posterior. En versiones anteriores se evalúa como una expresión regular sin anclar y también se dispara para cualquier tipo de agente que lo contenga, como `prod-db-agent`; anclarlo como `^db-agent$` en esas versiones.

Consulte [Hooks](/docs/es/hooks) para el formato de configuración de hook completo.

<h2 id="work-with-subagents">
  Trabajar con subagentes
</h2>

<h3 id="understand-automatic-delegation">
  Entender la delegación automática
</h3>

Claude delega tareas automáticamente en función de la descripción de la tarea en tu solicitud, el campo `description` en las configuraciones de subagentes y el contexto actual. Para fomentar la delegación proactiva, incluye frases como "use proactively" en el campo de descripción de tu subagente.

Mantén las descripciones breves: Claude Code muestra una advertencia de inicio cuando las descripciones combinadas de tus subagentes superan el [límite de 15,000 tokens](/docs/es/errors#agent-descriptions-are-over-the-15000-token-limit), y aún así carga todos los subagentes.

Si el subagente se distribuye en un [plugin](/docs/es/plugins/overview), puedes medir con qué fiabilidad Claude delega a él en solicitudes realistas en lugar de verificar una a la vez: [`claude plugin eval`](/docs/es/plugin-evals) ejecuta cada solicitud con y sin el plugin y califica los resultados.

<h3 id="invoke-subagents-explicitly">
  Invocar subagentes explícitamente
</h3>

Cuando la delegación automática no es suficiente, puedes solicitar un subagente tú mismo. Tres patrones escalan desde una sugerencia única hasta un valor predeterminado de toda la sesión:

* **Lenguaje natural**: nombra el subagente en tu solicitud; Claude decide si delega
* **@-mention**: garantiza que el subagente se ejecute para una tarea
* **Sesión completa**: toda la sesión utiliza el símbolo del sistema del subagente, las restricciones de herramientas y el modelo a través de la bandera `--agent` o la configuración `agent`

Para lenguaje natural, no hay sintaxis especial. Nombra el subagente y Claude típicamente delega:

```text wrap theme={null}
Use the test-runner subagent to fix failing tests
Have the code-reviewer subagent look at my recent changes
```

**@-mention el subagente.** Escribe `@` y elige el subagente del typeahead, de la misma manera que @-mencionas archivos. Esto asegura que se ejecute ese subagente específico en lugar de dejar la elección a Claude:

```text wrap theme={null}
@"code-reviewer (agent)" look at the auth changes
```

Tu mensaje completo aún va a Claude, que escribe el símbolo del sistema de la tarea del subagente en función de lo que solicitaste. El @-mention controla qué subagente invoca Claude, no qué símbolo del sistema recibe.

Los subagentes proporcionados por un [plugin](/docs/es/plugins/overview) habilitado aparecen en el typeahead bajo su nombre con alcance, como `my-plugin:code-reviewer` o `my-plugin:review:security` cuando el plugin [organiza agentes en subcarpetas](#choose-the-subagent-scope). Los subagentes de fondo nombrados que se ejecutan actualmente en la sesión también aparecen en el typeahead, mostrando su estado junto al nombre.

También puedes escribir la mención manualmente sin usar el selector: `@agent-<name>` para subagentes locales, o `@agent-` seguido del nombre con alcance para subagentes de plugin, por ejemplo `@agent-my-plugin:code-reviewer`. Mientras escribes esta forma, el typeahead muestra coincidencias de archivos en lugar de agentes. La mención del agente aún se resuelve cuando envías.

**Ejecuta toda la sesión como un subagente.** Pasa [`--agent <name>`](/docs/es/cli-reference) para iniciar una sesión donde el hilo principal en sí toma el símbolo del sistema del subagente, las restricciones de herramientas y el modelo:

```bash theme={null}
claude --agent code-reviewer
```

El símbolo del sistema del subagente reemplaza completamente el símbolo del sistema predeterminado de Claude Code, de la misma manera que [`--system-prompt`](/docs/es/cli-reference) lo hace. Los archivos `CLAUDE.md` y la memoria del proyecto aún se cargan a través del flujo de mensajes normal, incluso cuando la definición del agente establece [`omitClaudeMd`](#supported-frontmatter-fields).

El nombre del agente aparece como `@<name>` en el encabezado de inicio para que puedas confirmar que está activo.

Esto funciona con subagentes integrados y personalizados, y la elección persiste cuando reanudas la sesión: Claude Code restaura las restricciones de herramientas y el modelo del agente junto con la conversación. Si el agente ya no existe cuando reanudas, la sesión continúa con las herramientas predeterminadas y muestra una [advertencia que nombra el agente](/docs/es/errors#session-agent-no-longer-available). Para el símbolo del sistema en cualquier caso, consulta [Banderas de símbolo del sistema en conversaciones reanudadas](/docs/es/cli-reference#system-prompt-flags-in-resumed-conversations).

Para un subagente proporcionado por un plugin, puedes pasar solo el nombre del agente y Claude Code lo encuentra:

```bash theme={null}
claude --agent security-reviewer
```

Si varios plugins proporcionan agentes con el mismo nombre, pasa el nombre con alcance para desambiguar:

```bash theme={null}
claude --agent my-plugin:security-reviewer
```

Si el plugin coloca el agente en una subcarpeta de su directorio `agents/`, incluye la subcarpeta en el nombre con alcance, por ejemplo `claude --agent my-plugin:review:security`.

Para hacerlo el predeterminado para cada sesión en un proyecto, establece `agent` en `.claude/settings.json`:

```json theme={null}
{
  "agent": "code-reviewer"
}
```

La bandera CLI anula la configuración si ambas están presentes.

<h3 id="run-subagents-in-foreground-or-background">
  Ejecutar subagentes en primer plano o fondo
</h3>

Los subagentes pueden ejecutarse en primer plano o en fondo:

* **Subagentes en primer plano** bloquean la conversación principal hasta completarse. Los avisos de permiso se te pasan a medida que surgen.
* **Subagentes en fondo** se ejecutan simultáneamente mientras continúas trabajando. Cuando un subagente en fondo llega a una llamada de herramienta que necesita permiso, Claude Code muestra el aviso en tu sesión principal y nombra el subagente que está pidiendo. Aprueba para permitir que el subagente continúe, o presiona Esc para negar esa llamada de herramienta sin detener el subagente.

Para cada subagente que Claude genera con la herramienta Agent, Claude Code elige primer plano o fondo del primero de estos casos que se aplique:

* Si un compañero de [equipo de agentes](/docs/es/agent-teams#limitations) en proceso generó el subagente, Claude Code lo ejecuta en primer plano. Claude Code rechaza con un error generar un subagente de un compañero cuya definición establece [`background: true`](#supported-frontmatter-fields). Donde el [modo fork](#turn-fork-mode-on-or-off) está desactivado y no has [desactivado tareas en fondo](/docs/es/env-vars), Claude Code también rechaza con un error cuando un compañero establece `run_in_background: true`.
* Si estableces [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](/docs/es/env-vars) en `1`, Claude Code ejecuta el subagente en primer plano, en todo tipo de sesión y si el modo fork está activado o no.
* Donde el [modo fork](#turn-fork-mode-on-or-off) está activado, como lo está por defecto en una sesión interactiva, Claude Code ejecuta el subagente en fondo, subagentes fork y no-fork por igual, y Claude no puede pedir el primer plano.
* Donde el modo fork está desactivado, Claude ejecuta el subagente en fondo por defecto y en primer plano cuando necesita el resultado antes de continuar. El modo fork está desactivado en [modo no interactivo](/docs/es/headless) con `-p` y en el Agent SDK a menos que lo actives. Para mantener un subagente particular en fondo incluso cuando Claude quiere el resultado, establece su campo frontmatter [`background`](#supported-frontmatter-fields) en `true`.

Para una skill con `context: fork`, Claude Code sigue las reglas en [Ejecutar skills en un subagente](/docs/es/skills#run-skills-in-a-subagent) en su lugar, independientemente de si el modo fork está activado o no.

Los subagentes en fondo se ejecutan con un [conjunto de herramientas integradas más pequeño](#available-tools) que los subagentes en primer plano, excepto para forks de conversación y subagentes en primer plano [reanudados](#resume-subagents).

Los subagentes en fondo muestran cada aviso de permiso en tu sesión principal. Cuando respondes a uno de esos avisos con una opción que dura más allá de esa llamada de herramienta, como una concesión que dura el resto de la sesión, Claude Code aplica tu respuesta a toda la sesión, incluyendo tu conversación principal.

Un subagente en fondo puede dejar un comando [Bash o PowerShell](/docs/es/tools-reference#background-commands) en fondo [ejecutándose más allá del final de su turno](/docs/es/interactive-mode#how-backgrounding-works). Cuando ese comando termina, Claude Code envía al subagente una notificación.

Los resultados de un subagente en fondo llegan a Claude como una notificación de finalización en un turno posterior. Claude espera esa notificación antes de reportar los resultados del subagente, y si preguntas sobre el progreso primero, reporta que el subagente aún se está ejecutando. Antes de v2.1.211, Claude a veces reportaba resultados para un subagente en fondo que no había terminado.

También puedes dirigir esto tú mismo:

* Donde el modo fork está desactivado, pide a Claude que ejecute una tarea en fondo o en primer plano
* Presiona **Ctrl+B** para poner en fondo una tarea en ejecución

Claude Code borra la fila de un subagente en fondo del panel de subagentes debajo de la entrada de solicitud de dos maneras, dependiendo de cómo terminó el subagente:

* Cuando un subagente termina exitosamente, Claude Code elimina su fila inmediatamente y, excepto en [modo lector de pantalla](/docs/es/accessibility), muestra `/tasks to see subagents` en el pie de página durante 30 segundos. Durante esos 30 segundos, ejecuta [`/tasks`](/docs/es/commands) y presiona `Enter` en el subagente para abrir su transcripción. Antes de v2.1.232, Claude Code mantenía la fila durante 30 segundos después de que el subagente terminaba, igual que uno fallido, y no mostraba ninguna pista de pie de página.
* Cuando un subagente falla o lo detienes, Claude Code mantiene su fila durante 30 segundos. Para borrar la fila más rápido, selecciónala y presiona `x`.

Un subagente en fondo que se completa permanece listado en [`/tasks`](/docs/es/commands), marcado como hecho y ordenado debajo del trabajo en ejecución, durante los mismos 30 segundos que la pista de pie de página. Su vista de detalle permanece abierta cuando el subagente termina. Los subagentes que fallan o que detienes dejan la lista. Antes de v2.1.208, un subagente completado dejaba la lista en el momento en que terminaba y su vista de detalle se cerraba.

<h3 id="subagent-names">
  Nombres de subagentes
</h3>

Claude puede dar a un subagente un nombre pasando un parámetro `name` en la llamada de herramienta Agent, y puede hacerlo por su cuenta, sin pedirte permiso primero. El nombre hace que el subagente sea direccionable: Claude puede [enviarle un mensaje o reanudarlo por nombre](#resume-subagents) después de que termine.

En una sesión interactiva con [equipos de agentes](/docs/es/agent-teams) habilitados, un subagente que Claude genera desde la conversación principal con un `name` se lanza como un compañero en su lugar, a menos que la llamada sea un [fork](#fork-the-current-conversation) o pase `isolation` en la llamada misma. Un valor `isolation` en el frontmatter del subagente no lo previene, y el compañero entonces se ejecuta en el directorio de trabajo de la sesión principal. Consulta [Cómo Claude inicia equipos de agentes](/docs/es/agent-teams#how-claude-starts-agent-teams).

<h3 id="api-errors-in-subagents">
  Errores de API en subagentes
</h3>

Cuando algo [corta la respuesta de un subagente a mitad de la transmisión](/docs/es/errors#the-response-above-may-be-incomplete), y la respuesta parcial contiene texto pero sin llamadas de herramienta, Claude Code solicita al subagente que continúe en lugar de terminar la ejecución. Esto sucede también en sesiones interactivas. La ejecución termina en el error solo una vez que esas continuaciones se agotan.

A partir de v2.1.199, un subagente cuya ejecución termina en un error de API, como un límite de uso o un error de servidor repetido, reporta esa falla de vuelta a Claude en lugar de devolver el texto de error como si fueran los hallazgos del subagente. Lo que Claude recibe depende de dónde se ejecutó el subagente:

* **Primer plano**: si un límite de velocidad, sobrecarga o error de servidor corta un subagente que ya produjo salida de texto, la herramienta Agent devuelve esa salida parcial con una nota de que el subagente fue cortado y no completó su tarea. Un subagente que no produjo nada, o cuya única salida fueron llamadas de herramienta, falla con [`Agent terminated early due to an API error`](/docs/es/errors#agent-terminated-early-due-to-an-api-error), seguido del detalle del error. En v2.1.199, un límite de velocidad, sobrecarga o error de servidor que cortó la forma de solo llamadas de herramienta devolvió un resultado parcial vacío que contenía solo la nota de corte en su lugar.
* **Fondo**: el subagente se marca como fallido, y el mensaje que Claude recibe cuando termina nombra el error de API e incluye la última salida del subagente, por lo que el trabajo parcial no se pierde.

Cuando configuras una [cadena de modelo de respaldo](/docs/es/model-config#fallback-model-chains) y un subagente encuentra una falla que la cadena cubre, como que su modelo no esté disponible, Claude Code cambia el subagente al primer modelo en la cadena que acepta la solicitud. El subagente continúa trabajando en lugar de terminar en el error.

Una vez que el error de API subyacente se resuelve, pide a Claude que reintente la tarea o [reanude el subagente](#resume-subagents).

<h3 id="subagent-output-scanning">
  Escaneo de salida de subagentes
</h3>

Claude Code escanea el informe final de cada subagente antes de que Claude lo lea. Un subagente puede haber leído archivos, páginas web o salida de comandos que nunca revisaste, y el texto de esas fuentes puede llevar instrucciones dirigidas a la conversación principal. El escaneo nunca elimina ni reformula nada; hace dos tipos de cambios que puedes notar en un informe:

* **Inserción de barra invertida**: el escaneo inserta una barra invertida en el texto que imita la salida propia de Claude Code, como una etiqueta `<system-reminder>` o una línea que comienza con `Human:` o `Assistant:`, para que la imitación se lea como texto ordinario en lugar de ser confundida con parte de la conversación.
* **Línea de marcador**: el escaneo antepone una línea que comienza con `[harness: subagent output matched instruction-shaped pattern(s):` cuando el informe imita una etiqueta como `<system-reminder>` o menciona configuraciones de permiso como `bypassPermissions` o `--dangerously-skip-permissions`. Las menciones de configuración de permiso obtienen la línea de marcador, pero el texto en sí permanece como está escrito.

El escaneo no juzga si el contenido es malicioso, y no cambia lo que una instrucción en un informe puede hacer: una llamada de herramienta que el informe lleva a Claude a hacer aún pasa por los [controles de permiso](/docs/es/permissions) y [sandboxing](/docs/es/sandboxing) de la sesión. No es un sustituto para [restringir lo que un subagente puede alcanzar](#control-subagent-capabilities).

Un informe que regresa a Claude como el resultado del subagente también llega bajo un encabezado que lo marca como salida de subagente. El encabezado establece que las instrucciones o afirmaciones de aprobación dentro del informe son las palabras del subagente y no llevan autoridad de tu parte.

Un [informe de subagente en fondo](#run-subagents-in-foreground-or-background) llega dentro de una notificación de finalización, que se marca como un evento automatizado en lugar de un mensaje de tu parte.

<Note>
  El escaneo de salida de subagentes requiere Claude Code v2.1.210 o posterior.
</Note>

<h3 id="common-patterns">
  Patrones comunes
</h3>

<h4 id="isolate-high-volume-operations">
  Aislar operaciones de alto volumen
</h4>

Uno de los usos más efectivos para subagentes es aislar operaciones que producen grandes cantidades de salida. Ejecutar pruebas, obtener documentación o procesar archivos de registro puede consumir contexto significativo. Al delegar estos a un subagente, la salida detallada permanece en el contexto del subagente mientras solo el resumen relevante regresa a tu conversación principal.

```text wrap theme={null}
Use a subagent to run the test suite and report only the failing tests with their error messages
```

<h4 id="run-parallel-research">
  Ejecutar investigación en paralelo
</h4>

Para investigaciones independientes, genera múltiples subagentes para trabajar simultáneamente:

```text wrap theme={null}
Research the authentication, database, and API modules in parallel using separate subagents
```

Cada subagente explora su área independientemente, luego Claude sintetiza los hallazgos. Esto funciona mejor cuando las rutas de investigación no dependen una de la otra.

<Warning>
  Cuando los subagentes se completan, sus resultados regresan a tu conversación principal. Ejecutar muchos subagentes que cada uno devuelve resultados detallados puede consumir contexto significativo.
</Warning>

Para trabajo que necesita seguir ejecutándose en paralelo o no cabe en una ventana de contexto, ejecútalo en [sesiones separadas](/docs/es/agents) y deja que Claude [pase hallazgos entre ellas](/docs/es/cross-session-messaging).

<h4 id="chain-subagents">
  Encadenar subagentes
</h4>

Para flujos de trabajo de múltiples pasos, pide a Claude que use subagentes en secuencia. Cada subagente completa su tarea y devuelve resultados a Claude, que luego pasa contexto relevante al siguiente subagente.

```text wrap theme={null}
Use the code-reviewer subagent to find performance issues, then use the optimizer subagent to fix them
```

<h3 id="choose-between-subagents-and-main-conversation">
  Elegir entre subagentes y conversación principal
</h3>

Usa la **conversación principal** cuando:

* La tarea necesita ida y vuelta frecuente o refinamiento iterativo
* Múltiples fases comparten contexto significativo, como planificación, implementación y pruebas
* Estás haciendo un cambio rápido y dirigido
* La latencia importa. Un subagente que no es un [fork](#fork-the-current-conversation) comienza de nuevo y puede necesitar tiempo para reunir contexto

Usa **subagentes** cuando:

* La tarea produce salida detallada que no necesitas en tu contexto principal
* Quieres aplicar restricciones de herramientas o permisos específicos
* El trabajo es autónomo y puede devolver un resumen

Considera [Skills](/docs/es/skills) en su lugar cuando quieras solicitudes reutilizables o flujos de trabajo que se ejecuten en el contexto de conversación principal en lugar de contexto de subagente aislado.

Para una pregunta sobre algo ya en tu conversación, usa [`/btw`](/docs/es/interactive-mode#side-questions-with-%2Fbtw) en lugar de un subagente. Ve tu contexto completo pero no tiene acceso a herramientas, y la respuesta no se añade al historial.

<h3 id="let-subagents-spawn-their-own-subagents">
  Permitir que los subagentes generen sus propios subagentes
</h3>

Por defecto, un subagente puede generar subagentes propios, hasta tres capas por debajo de la conversación principal. En el límite de profundidad, Claude Code retiene la herramienta `Agent` de cada subagente excepto un [fork](#fork-the-current-conversation), por lo que un subagente en el límite hace su trabajo delegado a sí mismo y devuelve un resumen. Un fork en el límite mantiene `Agent` en su lista de herramientas heredada, pero la herramienta devuelve un error en lugar de generar.

Los subagentes anidados se adaptan a una tarea delegada que a su vez se divide en subtareas paralelas, como un subagente revisor que envía un verificador por hallazgo. En una sesión interactiva, solo el resumen del subagente de nivel superior regresa a ti y la salida intermedia permanece fuera de tu conversación principal: un subagente que lanza subagentes en fondo espera sus resultados antes de terminar. En [modo no interactivo](/docs/es/headless) y el Agent SDK, el subagente de lanzamiento no espera, por lo que un subagente en fondo anidado que termina después de que su lanzador ha terminado reporta a tu conversación principal en su lugar.

Para cambiar el límite, establece [`CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH`](/docs/es/env-vars) al número de capas de subagentes que quieres por debajo de tu conversación principal. Por ejemplo, esta entrada en [`settings.json`](/docs/es/settings) limita el anidamiento a dos capas:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH": "2"
  }
}
```

Con este valor, tus subagentes pueden delegar a una segunda capa de los suyos, y esa segunda capa no puede delegar más. Establece `1` para desactivar el anidamiento.

Un subagente anidado se configura de la misma manera que uno de nivel superior y se resuelve desde los mismos [alcances](#choose-the-subagent-scope). Para mantener un subagente sin generar mientras el anidamiento está activado, como un revisor que debe permanecer de solo lectura, omite `Agent` de su lista [`tools`](#available-tools) o añádelo a `disallowedTools`.

En el terminal, Claude Code muestra subagentes anidados como un árbol en el panel de subagentes debajo de la entrada de solicitud y marca cada fila que aún tiene descendientes en el panel con un conteo `(+N)` de ellos. Abre una fila para ver los hermanos y hijos directos de ese subagente con una ruta de vuelta a `main`.

<Note>
  Las versiones anteriores usaban diferentes valores predeterminados:

  * **v2.1.172 a v2.1.216**: los subagentes podían anidar por defecto, hasta cinco capas de profundidad, y el límite no podía cambiarse.
  * **v2.1.217 a v2.1.218**: el límite predeterminado era uno, por lo que un subagente no podía generar el suyo a menos que lo aumentaras; v2.1.219 aumentó el predeterminado a tres.
</Note>

<h3 id="concurrent-subagent-limit">
  Límite de subagentes concurrentes
</h3>

Dos límites controlan el uso de subagentes, cada uno con su propia variable: este detiene a Claude de generar más subagentes mientras demasiados se están ejecutando, y el [límite de profundidad](#let-subagents-spawn-their-own-subagents) limita cuán profundamente se anidan los subagentes. No hay límite en el número total de subagentes que Claude puede generar durante una sesión.

Por defecto, cuando 20 subagentes se están ejecutando en una sesión, generar otro con la herramienta Agent falla con `Concurrent subagent limit reached`, y el error le dice a Claude que no reintente. La generación tiene éxito de nuevo cuando el conteo en ejecución cae por debajo del límite. Para cambiar el límite, establece [`CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS`](/docs/es/env-vars) a cualquier número entero positivo. Las sesiones con [ultracode](/docs/es/model-config#adjust-effort-level) activo están exentas: el límite no se aplica allí. Requiere Claude Code v2.1.217 o posterior.

El límite bloquea solo subagentes que Claude genera con la herramienta Agent, pero otras ejecuciones ocupan los mismos espacios:

* Un fork en sesión que inicias con [`/subtask`](#fork-the-current-conversation) toma un espacio mientras se ejecuta y nunca es bloqueado por el límite.
* [Reanudar un subagente](#resume-subagents) que ya terminó toma un espacio nuevo sin verificar el límite, por lo que las reanudaciones pueden empujar el conteo en ejecución más allá de él.

Los agentes que otras características ejecutan, como agentes de [workflow](/docs/es/workflows) y compañeros de [equipo de agentes](/docs/es/agent-teams), siguen sus propios límites en su lugar.

<h3 id="manage-subagent-context">
  Gestionar contexto de subagentes
</h3>

<h4 id="what-loads-at-startup">
  Qué se carga al inicio
</h4>

Cada subagente comienza con una ventana de contexto fresca y aislada. No ve tu historial de conversación, las skills que ya has invocado, o los archivos que Claude ya ha leído. Claude compone un mensaje de delegación que resume la tarea, y el subagente trabaja a partir de ahí. La excepción es un [fork](#fork-the-current-conversation), que hereda la conversación padre en lugar de comenzar de nuevo.

El contexto inicial de un subagente que no es fork contiene:

* **Símbolo del sistema**: el símbolo del sistema propio del agente más detalles del entorno que Claude Code añade, no el símbolo del sistema de Claude Code. Los subagentes personalizados definen el suyo en el [cuerpo markdown](#write-subagent-files) o campo `prompt`. Los agentes integrados tienen símbolos del sistema predefinidos.
* **Mensaje de tarea**: el símbolo del sistema de delegación que Claude escribe cuando entrega el trabajo.
* **Archivos CLAUDE.md**: cada nivel de la [jerarquía CLAUDE.md](/docs/es/memory#how-claude-md-files-load) que la conversación principal carga, incluyendo `~/.claude/CLAUDE.md`, reglas del proyecto, `CLAUDE.local.md`, archivos de política gestionada, y cualquier archivo [`AGENTS.md`](/docs/es/memory#agents-md) cargado como instrucciones del proyecto. Los agentes Explore y Plan integrados omiten esto. Un subagente cuya definición establece [`omitClaudeMd`](#supported-frontmatter-fields) carga solo los archivos de política gestionada, o ninguno en absoluto cuando la definición viene de [configuración gestionada](#choose-the-subagent-scope).
* **Estado de Git**: una instantánea que Claude Code lee de tu repositorio cuando el subagente comienza. Ausente fuera de un repositorio de Git o siempre que la instantánea esté desactivada; consulta [`includeGitInstructions`](/docs/es/settings-reference#includegitinstructions). Explore y Plan lo omiten independientemente.
* **Skills precargadas**: contenido completo de cualquier skill nombrada en el campo [`skills`](#preload-skills-into-subagents) del agente. Los agentes integrados no precargan skills.
* **Roster de hermanos**: un recordatorio del sistema listando `main` y cada otro agente nombrado en la sesión, cada uno un valor `to` válido para [`SendMessage`](#resume-subagents). Requiere Claude Code v2.1.206 o posterior. El roster aparece solo cuando las herramientas del subagente incluyen `SendMessage` y al menos otro agente tiene un nombre, ya sea que Claude lo nombró al generarlo o se ejecuta como un compañero de [equipo de agentes](/docs/es/agent-teams). Es una instantánea tomada cuando el subagente comienza, por lo que los agentes nombrados después no aparecen.

Para lanzar uno de tus propios subagentes sin los archivos CLAUDE.md del usuario, proyecto y local, establece [`omitClaudeMd: true`](#supported-frontmatter-fields) en su frontmatter o `--agents` JSON.

La conversación principal aún tiene tu CLAUDE.md completo cuando lee los resultados de estos subagentes, por lo que la mayoría de reglas no necesitan llegar al subagente mismo. Si una regla debe, como "ignora el directorio `vendor/`," reafírmala en el símbolo del sistema que das a Claude cuando delegas.

No puedes cambiar qué subagentes reciben estado de git. Solo Explore y Plan lo omiten.

Algún estado de conversación principal nunca llega a un subagente que no es fork:

* **Estilo de salida**: un subagente ejecuta su propio símbolo del sistema, por lo que tu [estilo de salida](/docs/es/output-styles) no forma sus respuestas, excepto en un [fork](#fork-the-current-conversation).
* **Memoria automática**: la [memoria automática](/docs/es/memory#auto-memory) de la conversación principal no se carga. Para dar a un subagente memoria persistente propia, usa el campo [`memory`](#enable-persistent-memory).
* **Tamaño de ventana de contexto**: la ventana de contexto de un subagente se dimensiona por su propio modelo, no por el del padre. Delegar a un modelo con una ventana más pequeña da a ese subagente la ventana más pequeña.

<h4 id="resume-subagents">
  Reanudar subagentes
</h4>

Cada invocación de subagente crea una nueva instancia en lugar de continuar una anterior. Para continuar el trabajo de un subagente existente en lugar de comenzar de nuevo, pide a Claude que lo reanude.

Los subagentes reanudados retienen su historial de conversación completo, incluyendo todas las llamadas de herramienta anteriores, resultados y razonamiento. Si el subagente generó [subagentes en fondo propios](#let-subagents-spawn-their-own-subagents), ese historial incluye los resultados que entregaron mientras se ejecutaba. El subagente continúa exactamente donde se detuvo en lugar de comenzar de nuevo.

* Cuando un subagente se completa, Claude recibe su ID de agente.
* Los agentes Explore y Plan integrados son de una sola vez y no devuelven un ID de agente, por lo que Claude no puede reanudarlo. Usa `general-purpose` o un subagente personalizado cuando necesites continuar el trabajo.
* Cuando un subagente se detiene en su límite [`maxTurns`](#supported-frontmatter-fields), Claude Code marca la salida devuelta como parcial. Para subagentes que devuelven un ID de agente, Claude Code también nota en el resultado que Claude puede enviar un mensaje al subagente para continuar desde donde se detuvo.

Claude usa la herramienta `SendMessage` con el ID o nombre del agente como campo `to` para reanudarlo. `SendMessage` no requiere que [equipos de agentes](/docs/es/agent-teams) estén habilitados; solo mensajes de protocolo de equipo estructurados como `shutdown_request` y `plan_approval_response` lo hacen. Más allá de subagentes y compañeros, en sesiones donde la mensajería entre sesiones está habilitada, Claude puede usar la misma herramienta para enviar mensajes a [tus otras sesiones de Claude Code](/docs/es/cross-session-messaging), en esta máquina o [más allá de ella](/docs/es/cross-session-messaging#message-sessions-on-other-machines).

Para reanudar un subagente, pide a Claude que continúe el trabajo anterior:

```text wrap theme={null}
Use the code-reviewer subagent to review the authentication module
[Agent completes]

Continue that code review and now analyze the authorization logic
[Claude resumes the subagent with full context from previous conversation]
```

Cuando Claude envía un mensaje a un subagente completado con la herramienta `SendMessage`, el subagente se reanuda en fondo sin una nueva invocación de `Agent`. Lo mismo se aplica a un subagente que Claude detuvo con la herramienta `TaskStop`, una vez que su ejecución detenida ha salido. La ejecución reanudada mantiene el [conjunto de herramientas de donde el subagente se ejecutó primero](#run-subagents-in-foreground-or-background) y puede seguir leyendo el [caché de solicitud que la ejecución original calentó](/docs/es/prompt-caching#subagents-and-the-cache).

Un subagente que tiene la herramienta `SendMessage` puede enviar ese mensaje también. En una sesión interactiva, el agente reanudado entonces reporta de vuelta al subagente que lo reanudó, no a tu conversación principal. Ese subagente espera el resultado antes de terminar su propio trabajo. Cuando un subagente envía un mensaje a un agente al que reporta, como su propio lanzador, Claude Code reanuda ese agente sin redirigir sus resultados.

Un subagente que detuviste tú mismo, con `x` en `/tasks` o una solicitud SDK `stop_task`, no se reanuda automáticamente. Si Claude le envía un mensaje, el mensaje es rechazado y Claude es informado de que el agente fue cancelado.

Mientras [la fila de ese subagente aún está en el panel de subagentes](#run-subagents-in-foreground-or-background), escribe en su transcripción para reanudarlo tú mismo. Después de eso, un mensaje de Claude puede reanudarlo automáticamente de nuevo.

Reanudar comienza una nueva ejecución del agente bajo el mismo ID, por lo que un subagente que ya había fallado o se completó muestra como ejecutándose de nuevo en la lista de tareas y en los eventos de tareas del Agent SDK. Antes de v2.1.205, mantenía su estado anterior fallido o completado mientras la ejecución reanudada estaba trabajando.

A partir de v2.1.199, `SendMessage` verifica que un nombre aún se refiera al mismo agente que alcanzó anteriormente en la conversación. Si un agente más nuevo ha tomado el nombre, como un agente en fondo re-generado que lo reutilizó, Claude Code rechaza el envío en lugar de entregarlo al agente incorrecto, y el error reporta qué agente el nombre ahora alcanza para que Claude pueda redirigir. Para alcanzar el agente anterior mientras aún se está ejecutando, Claude lo dirige por el ID de agente que recibió cuando generó ese agente. La verificación está limitada a la conversación actual y se reinicia en `/clear`.

A partir de v2.1.198, un subagente trata mensajes del agente que lo lanzó como dirección de tarea normal, incluyendo correcciones de curso a mitad de tarea, y actúa sobre ellos dentro de sus propias configuraciones de permiso. Dos límites aún se mantienen independientemente de quién envió el mensaje: ningún mensaje de ningún agente cuenta como tu aprobación para un aviso de permiso pendiente, y ningún mensaje de agente puede cambiar las configuraciones de permiso, `CLAUDE.md`, o configuración de un subagente. Solo el sistema de permiso o tus propios mensajes pueden otorgar aprobación.

También puedes pedir a Claude el ID de agente si quieres referenciarlo explícitamente, o encontrar IDs en los archivos de transcripción en `~/.claude/projects/{project}/{sessionId}/subagents/`. Cada transcripción se almacena como `agent-{agentId}.jsonl`.

Las transcripciones de subagentes persisten independientemente de la conversación principal:

* **Compactación de conversación principal**: cuando la conversación principal se compacta, las transcripciones de subagentes no se ven afectadas. Se almacenan en archivos separados.
* **Persistencia de sesión**: las transcripciones de subagentes persisten dentro de su sesión. Puedes [reanudar un subagente](#resume-subagents) después de reiniciar Claude Code reanudando la misma sesión.
* **Limpieza automática**: Claude Code elimina las transcripciones de subagentes después del período de retención `cleanupPeriodDays`, 30 días por defecto, siguiendo las [reglas de barrido de retención](/docs/es/claude-directory#cleaned-up-automatically).

<h4 id="auto-compaction">
  Auto-compactación
</h4>

Los subagentes soportan compactación automática usando la misma lógica que la conversación principal. La compactación se dispara bajo las mismas condiciones, y `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` se aplica a subagentes también. Consulta [variables de entorno](/docs/es/env-vars) para cuándo entra en vigor el override.

Los eventos de compactación se registran en archivos de transcripción de subagentes:

```json theme={null}
{
  "type": "system",
  "subtype": "compact_boundary",
  "compactMetadata": {
    "trigger": "auto",
    "preTokens": 167189
  }
}
```

El valor `preTokens` muestra cuántos tokens se usaron antes de que ocurriera la compactación.

<h2 id="fork-the-current-conversation">
  Bifurcar la conversación actual
</h2>

<Note>
  Ejecute un subagente bifurcado con `/subtask`, que requiere Claude Code v2.1.212 o posterior. Cuando [la vista de agente está desactivada](/docs/es/agent-view#turn-off-agent-view), `/subtask` no está disponible y `/fork` inicia el subagente bifurcado en su lugar; de lo contrario, `/fork` copia toda la sesión en una nueva [sesión de fondo](/docs/es/agent-view#from-inside-a-session).
</Note>

Un fork es un subagente que hereda toda la conversación hasta ahora en lugar de comenzar de nuevo. Esto elimina el aislamiento de entrada que los subagentes de otra manera proporcionan: un fork ve el mismo mensaje del sistema, herramientas, modelo e historial de mensajes que la sesión principal, para que pueda entregarle una tarea secundaria sin re-explicar la situación. Las propias llamadas de herramientas del fork aún permanecen fuera de su conversación y solo su resultado final regresa, por lo que su ventana de contexto principal permanece limpia. Use un fork cuando cualquier otro subagente necesitaría demasiado contexto para ser útil, o cuando desee probar varios enfoques en paralelo desde el mismo punto de partida.

Claude inicia un fork solicitando el tipo de subagente `fork` a través de la herramienta Agent. Usted controla si puede hacerlo con [modo fork](#turn-fork-mode-on-or-off), que está activado de forma predeterminada en sesiones interactivas.

Puede iniciar un fork usted mismo con `/subtask` seguido de una tarea, independientemente de si el modo fork está activado o no. En v2.1.161 a v2.1.211, el comando es `/fork`. Claude Code nombra el fork a partir de las primeras palabras de la tarea. El siguiente ejemplo bifurca la conversación para redactar casos de prueba mientras continúa con la implementación en la sesión principal:

```text wrap theme={null}
/subtask draft unit tests for the parser changes so far
```

El fork aparece en un panel debajo de su solicitud y se ejecuta en el fondo mientras continúa trabajando. Cuando termina, su resultado llega como un mensaje en su conversación principal. La siguiente sección cubre los controles del panel para observar y dirigir forks mientras se ejecutan.

<h3 id="observe-and-steer-running-forks">
  Observar y dirigir forks en ejecución
</h3>

Los forks en ejecución aparecen en un panel debajo de la entrada de solicitud, con una fila para la sesión principal y una para cada fork.

Cuando un fork termina exitosamente, Claude Code elimina su fila. Claude Code mantiene la fila de un fork que falló o que usted detuvo durante 30 segundos, [lo mismo que para cualquier otro subagente de fondo](#run-subagents-in-foreground-or-background). Antes de v2.1.232, Claude Code también mantenía la fila de un fork terminado durante 30 segundos.

Use estas teclas para interactuar con el panel:

| Tecla     | Acción                                                                                                                                                                                                                                      |
| :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `↑` / `↓` | Moverse entre filas                                                                                                                                                                                                                         |
| `Enter`   | Abrir la transcripción del fork seleccionado y enviarle mensajes de seguimiento                                                                                                                                                             |
| `x`       | Detener el fork seleccionado si se está ejecutando, o descartar su fila si ya no se está ejecutando. En la fila de la sesión principal, o en la fila del fork cuya transcripción abrió con `Enter`, `x` escribe en la solicitud en su lugar |
| `Esc`     | Devolver el enfoque a la entrada de solicitud                                                                                                                                                                                               |

Con la transcripción de un fork o subagente abierta, los mensajes de seguimiento y las [skills](/docs/es/skills) van a ese agente, pero los comandos integrados aún se ejecutan en su conversación principal. A partir de v2.1.199, escribir `/model` o `/fast` en esa vista muestra un aviso de que cambia el modelo de la conversación principal o el modo rápido, no el del agente visto, en lugar de ejecutarlo silenciosamente.

<h3 id="how-forks-differ-from-other-subagents">
  Cómo los forks difieren de otros subagentes
</h3>

Un fork hereda todo lo que la sesión principal tiene en el momento en que se genera. Cualquier otro subagente comienza desde cero a partir de su definición.

|                                    | Fork                                    | Subagente que no es fork                                                                                                     |
| :--------------------------------- | :-------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| Contexto                           | Historial de conversación completo      | Contexto fresco con la solicitud que pasa                                                                                    |
| Mensaje del sistema y herramientas | Igual que la sesión principal           | Del [archivo de definición](#write-subagent-files) del subagente, [filtrado para ejecuciones de fondo](#available-tools)     |
| Modelo                             | Igual que la sesión principal           | Del campo `model` del subagente                                                                                              |
| Permisos                           | Las solicitudes aparecen en su terminal | [Las solicitudes aparecen en su sesión principal](#run-subagents-in-foreground-or-background) cuando se ejecutan en el fondo |
| Caché de solicitud                 | Compartido con la sesión principal      | Caché separado                                                                                                               |

Porque el mensaje del sistema del fork y las definiciones de herramientas son idénticas al principal, su primera solicitud reutiliza la [caché de solicitud](/docs/es/prompt-caching#subagents-and-the-cache) del principal. Esto hace que bifurcar sea más económico que generar un subagente fresco para tareas que necesitan el mismo contexto.

Cuando Claude genera un fork a través de la herramienta Agent, puede pasar `isolation: "worktree"` para que las ediciones de archivo del fork se escriban en un git worktree separado en lugar de su checkout. Un fork no puede generar más forks.

<h3 id="turn-fork-mode-on-or-off">
  Activar o desactivar el modo fork
</h3>

Claude Code activa el modo fork de forma predeterminada en sesiones interactivas y lo deja desactivado de forma predeterminada en [modo no interactivo](/docs/es/headless) con `-p` y en el Agent SDK. El valor predeterminado interactivo requiere Claude Code v2.1.232 o posterior. En versiones anteriores, establezca `CLAUDE_CODE_FORK_SUBAGENT` en `1` para activar el modo fork.

Puede saber que el modo fork está activado por cómo Claude Code maneja la herramienta Agent:

* Claude puede generar un fork solicitando el tipo de subagente `fork`. Cuando Claude no solicita un tipo, obtiene el subagente [general-purpose](#built-in-subagents), si la sesión aún tiene ese tipo. Los subagentes generados a partir de una definición, como Explore, funcionan como de costumbre.
* Claude Code ejecuta los subagentes que Claude genera en el fondo, forks y subagentes que no son fork por igual, aparte de los [casos que permanecen en primer plano](#run-subagents-in-foreground-or-background). Claude Code también elimina el parámetro `run_in_background` de la herramienta Agent, por lo que Claude no puede solicitar el primer plano.

Establezca la variable de entorno [`CLAUDE_CODE_FORK_SUBAGENT`](/docs/es/env-vars) para anular los valores predeterminados:

* `1` activa el modo fork en modo no interactivo y el Agent SDK también
* `0` desactiva el modo fork en todo tipo de sesión

Para mantener el modo fork activado pero evitar que Claude genere forks, [deniegue el tipo de subagente `fork`](#disable-specific-subagents) con una regla `Agent(fork)`. Claude Code aún ejecuta los subagentes que Claude genera en el fondo, aparte de los mismos [casos que permanecen en primer plano](#run-subagents-in-foreground-or-background).

<h2 id="example-subagents">
  Subagentes de ejemplo
</h2>

Estos ejemplos demuestran patrones efectivos para construir subagentes. Úselos como puntos de partida, o genere una versión personalizada con Claude.

<Tip>
  **Mejores prácticas:**

  * **Diseñe subagentes enfocados:** cada subagente debe sobresalir en una tarea específica
  * **Escriba descripciones que destaquen un subagente:** Claude usa la descripción para decidir cuándo delegar. Haga cada descripción lo suficientemente específica para enrutar al subagente correcto, y mantenga el conjunto combinado dentro del [presupuesto de descripción de 15,000 tokens](#understand-automatic-delegation)
  * **Limite el acceso a herramientas:** otorgue solo permisos necesarios para seguridad y enfoque
  * **Verifique en control de versiones:** comparta subagentes de proyecto con su equipo
</Tip>

<h3 id="code-reviewer">
  Revisor de código
</h3>

Un subagente de solo lectura que revisa código sin modificarlo. Este ejemplo muestra cómo diseñar un subagente enfocado con acceso limitado a herramientas que excluye Edit y Write, y un mensaje detallado que especifica exactamente qué buscar y cómo formatear la salida.

```markdown theme={null}
---
name: code-reviewer
description: Expert code review specialist. Proactively reviews code for quality, security, and maintainability. Use immediately after writing or modifying code.
tools: Read, Grep, Glob, Bash
model: inherit
---

You are a senior code reviewer ensuring high standards of code quality and security.

When invoked:
1. Run git diff to see recent changes
2. Focus on modified files
3. Begin review immediately

Review checklist:
- Code is clear and readable
- Functions and variables are well-named
- No duplicated code
- Proper error handling
- No exposed secrets or API keys
- Input validation implemented
- Good test coverage
- Performance considerations addressed

Provide feedback organized by priority:
- Critical issues (must fix)
- Warnings (should fix)
- Suggestions (consider improving)

Include specific examples of how to fix issues.
```

<h3 id="debugger">
  Depurador
</h3>

Un subagente que puede analizar y corregir problemas. A diferencia del revisor de código, este incluye Edit porque corregir errores requiere modificar código. El mensaje proporciona un flujo de trabajo claro desde diagnóstico hasta verificación.

```markdown theme={null}
---
name: debugger
description: Debugging specialist for errors, test failures, and unexpected behavior. Use proactively when encountering any issues.
tools: Read, Edit, Bash, Grep, Glob
---

You are an expert debugger specializing in root cause analysis.

When invoked:
1. Capture error message and stack trace
2. Identify reproduction steps
3. Isolate the failure location
4. Implement minimal fix
5. Verify solution works

Debugging process:
- Analyze error messages and logs
- Check recent code changes
- Form and test hypotheses
- Add strategic debug logging
- Inspect variable states

For each issue, provide:
- Root cause explanation
- Evidence supporting the diagnosis
- Specific code fix
- Testing approach
- Prevention recommendations

Focus on fixing the underlying issue, not the symptoms.
```

<h3 id="data-scientist">
  Científico de datos
</h3>

Un subagente específico de dominio para trabajo de análisis de datos. Este ejemplo muestra cómo crear subagentes para flujos de trabajo especializados fuera de tareas de codificación típicas. Establece explícitamente `model: sonnet` para análisis más capaz.

```markdown theme={null}
---
name: data-scientist
description: Data analysis expert for SQL queries, BigQuery operations, and data insights. Use proactively for data analysis tasks and queries.
tools: Bash, Read, Write
model: sonnet
---

You are a data scientist specializing in SQL and BigQuery analysis.

When invoked:
1. Understand the data analysis requirement
2. Write efficient SQL queries
3. Use BigQuery command line tools (bq) when appropriate
4. Analyze and summarize results
5. Present findings clearly

Key practices:
- Write optimized SQL queries with proper filters
- Use appropriate aggregations and joins
- Include comments explaining complex logic
- Format results for readability
- Provide data-driven recommendations

For each analysis:
- Explain the query approach
- Document any assumptions
- Highlight key findings
- Suggest next steps based on data

Always ensure queries are efficient and cost-effective.
```

<h3 id="database-query-validator">
  Validador de consultas de base de datos
</h3>

Un subagente que permite acceso a Bash pero valida comandos para permitir solo consultas SQL de solo lectura. Este ejemplo muestra cómo usar hooks `PreToolUse` para validación condicional cuando necesita control más fino que el campo `tools` proporciona.

```markdown theme={null}
---
name: db-reader
description: Execute read-only database queries. Use when analyzing data or generating reports.
tools: Bash
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-readonly-query.sh"
---

You are a database analyst with read-only access. Execute SELECT queries to answer questions about the data.

When asked to analyze data:
1. Identify which tables contain the relevant data
2. Write efficient SELECT queries with appropriate filters
3. Present results clearly with context

You cannot modify data. If asked to INSERT, UPDATE, DELETE, or modify schema, explain that you only have read access.
```

Claude Code [pasa la entrada del hook como JSON](/docs/es/hooks#pretooluse-input) a través de stdin a comandos de hook. El script de validación lee este JSON, extrae el comando siendo ejecutado, y lo verifica contra una lista de operaciones de escritura SQL. Si se detecta una operación de escritura, el script [sale con código 2](/docs/es/hooks#exit-code-2-behavior-per-event) para bloquear la ejecución y devuelve un mensaje de error a Claude a través de stderr.

Cree el script de validación en cualquier lugar en su proyecto. La ruta debe coincidir con el campo `command` en su configuración de hook:

```bash theme={null}
#!/bin/bash
# Blocks SQL write operations, allows SELECT queries

# Read JSON input from stdin
INPUT=$(cat)

# Extract the command field from tool_input using jq
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

if [ -z "$COMMAND" ]; then
  exit 0
fi

# Block write operations (case-insensitive)
if echo "$COMMAND" | grep -iE '\b(INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|TRUNCATE|REPLACE|MERGE)\b' > /dev/null; then
  echo "Blocked: Write operations not allowed. Use SELECT queries only." >&2
  exit 2
fi

exit 0
```

En macOS y Linux, haga el script ejecutable:

```bash theme={null}
chmod +x ./scripts/validate-readonly-query.sh
```

En Windows, escriba el script de validación en PowerShell y agregue `shell: powershell` a la entrada del hook. Consulte [ejecutar hooks en PowerShell](/docs/es/hooks#windows-powershell-tool).

El hook recibe JSON a través de stdin con el comando Bash en `tool_input.command`. El código de salida 2 bloquea la operación y alimenta el mensaje de error de vuelta a Claude. Consulte [Hooks](/docs/es/hooks#exit-code-output) para detalles sobre códigos de salida e [Hook input](/docs/es/hooks#pretooluse-input) para el esquema de entrada completo.

El mensaje del sistema le dice al subagente que rechace solicitudes de escritura, por lo que el hook es una red de seguridad: si el subagente intenta una escritura de todas formas, Claude Code bloquea el comando y el subagente ve el mensaje `Blocked: Write operations not allowed. Use SELECT queries only.`.

<h2 id="next-steps">
  Próximos pasos
</h2>

Ahora que entiende subagentes, explore estas características relacionadas:

* [Distribuir subagentes con plugins](/docs/es/plugins/components#agents) para compartir subagentes entre equipos o proyectos
* [Ejecutar Claude Code programáticamente](/docs/es/headless) con el Agent SDK para CI/CD y automatización
* [Usar servidores MCP](/docs/es/mcp) para dar a los subagentes acceso a herramientas y datos externos
