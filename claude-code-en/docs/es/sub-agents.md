> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Crear subagentes personalizados

> Cree y utilice subagentes de IA especializados en Claude Code para flujos de trabajo específicos de tareas y una mejor gestión del contexto.

Los subagentes son asistentes de IA especializados que manejan tipos específicos de tareas. Utilice uno cuando una tarea secundaria inundaría su conversación principal con resultados de búsqueda, registros o contenidos de archivos que no volverá a consultar: el subagente realiza ese trabajo en su propio contexto y devuelve solo el resumen. Defina un subagente personalizado cuando siga generando el mismo tipo de trabajador con las mismas instrucciones.

Cada subagente se ejecuta en su propia ventana de contexto con un mensaje del sistema personalizado, acceso a herramientas específicas y permisos independientes. Cuando Claude encuentra una tarea que coincide con la descripción de un subagente, delega en ese subagente, que trabaja de forma independiente y devuelve resultados. Para ver el ahorro de contexto en la práctica, la [visualización de la ventana de contexto](/docs/es/context-window) muestra un recorrido por una sesión donde un subagente maneja la investigación en su propia ventana separada.

<Note>
  Los subagentes funcionan dentro de una única sesión. Para ejecutar muchas sesiones independientes en paralelo y supervisarlas desde un único lugar, consulte [agentes en segundo plano](/docs/es/agent-view). Para sesiones que se comunican entre sí, consulte [equipos de agentes](/docs/es/agent-teams).
</Note>

Los subagentes le ayudan a:

* **Preservar contexto** manteniendo la exploración e implementación fuera de su conversación principal
* **Aplicar restricciones** limitando qué herramientas puede usar un subagente
* **Reutilizar configuraciones** en proyectos con subagentes a nivel de usuario
* **Especializar comportamiento** con mensajes del sistema enfocados para dominios específicos
* **Controlar costos** enrutando tareas a modelos más rápidos y económicos como Haiku

Claude utiliza la descripción de cada subagente para decidir cuándo delegar tareas. Cuando crea un subagente, escriba una descripción clara para que Claude sepa cuándo usarlo.

Claude Code incluye varios subagentes integrados como Explore, Plan y general-purpose. También puede crear subagentes personalizados para manejar tareas específicas.

<h2 id="built-in-subagents">
  Subagentes integrados
</h2>

Claude Code incluye subagentes integrados que Claude utiliza automáticamente cuando es apropiado. Cada uno hereda los permisos de la conversación principal con restricciones de herramientas adicionales.

Explore y Plan omiten sus archivos CLAUDE.md y el estado de git de la sesión principal para mantener la investigación rápida y económica. Todos los demás subagentes integrados y [subagentes personalizados](#configure-subagents) cargan ambos. Para el desglose completo de lo que llega a un subagente, consulte [qué se carga al iniciar](#what-loads-at-startup).

<Tabs>
  <Tab title="Explore">
    Un agente rápido y de solo lectura optimizado para buscar y analizar bases de código.

    * **Modelo**: hereda de la conversación principal, limitado a Opus en la API de Claude, por lo que Explore nunca se ejecuta en un modelo más costoso que el que ya eligió para la sesión
    * **Herramientas**: herramientas de solo lectura; Write y Edit están denegados
    * **Propósito**: descubrimiento de archivos, búsqueda de código, exploración de base de código

    A partir de v2.1.198, Explore hereda el modelo de la conversación principal en lugar de ejecutarse siempre en Haiku. En la API de Claude, el modelo heredado está limitado a Opus: una conversación principal en un nivel superior ejecuta Explore en Opus, y una conversación principal en Sonnet o Haiku ejecuta Explore en ese mismo modelo. En cualquier otro proveedor, como [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, o Claude Platform en AWS](/docs/es/third-party-integrations), Explore hereda el modelo de la conversación principal directamente.

    Un [subagente de usuario o proyecto](#choose-the-subagent-scope) llamado `Explore` anula el integrado y mantiene su propio campo `model`, así que defina uno con `model: haiku` para mantener la exploración en un modelo de menor costo.

    Claude delega en Explore cuando necesita buscar o entender una base de código sin hacer cambios. Esto mantiene los resultados de exploración fuera del contexto de su conversación principal.

    Al invocar Explore, Claude especifica un nivel de minuciosidad: **quick** para búsquedas dirigidas, **medium** para exploración equilibrada, o **very thorough** para análisis exhaustivo.
  </Tab>

  <Tab title="Plan">
    Un agente de investigación utilizado durante [plan mode](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode) para recopilar contexto antes de presentar un plan.

    * **Modelo**: hereda de la conversación principal
    * **Herramientas**: herramientas de solo lectura; Write y Edit están denegados
    * **Propósito**: investigación de base de código para planificación

    Cuando está en plan mode y Claude necesita entender su base de código, delega la investigación al subagente Plan para que la salida de exploración permanezca en una ventana de contexto separada mientras la conversación principal sigue siendo de solo lectura.
  </Tab>

  <Tab title="General-purpose">
    Un agente capaz para tareas complejas de múltiples pasos que requieren tanto exploración como acción.

    * **Modelo**: hereda de la conversación principal
    * **Herramientas**: todas las herramientas
    * **Propósito**: investigación compleja, operaciones de múltiples pasos, modificaciones de código

    Claude delega en general-purpose cuando la tarea requiere tanto exploración como modificación, razonamiento complejo para interpretar resultados, o múltiples pasos dependientes.
  </Tab>

  <Tab title="Other">
    Claude Code incluye agentes auxiliares adicionales para tareas específicas. Estos se invocan típicamente automáticamente, por lo que no necesita usarlos directamente.

    | Agente            | Modelo | Cuándo Claude lo usa                                            |
    | :---------------- | :----- | :-------------------------------------------------------------- |
    | statusline-setup  | Sonnet | Cuando ejecuta `/statusline` para configurar su línea de estado |
    | claude-code-guide | Haiku  | Cuando hace preguntas sobre características de Claude Code      |
  </Tab>
</Tabs>

Los subagentes integrados se registran por defecto en sesiones interactivas. Para restringirlos:

* Para bloquear un tipo integrado específico, agréguelo a `permissions.deny` como se muestra en [Deshabilitar subagentes específicos](#disable-specific-subagents).
* Para evitar que Claude delegue a cualquier subagente, deniegue la herramienta `Agent` en sí con [`permissions.deny`](/docs/es/permissions#tool-specific-permission-rules).
* Para eliminar solo los subagentes integrados `Explore` y `Plan`, establezca [`CLAUDE_CODE_DISABLE_EXPLORE_PLAN_AGENTS=1`](/docs/es/env-vars). Claude lee y explora archivos directamente en lugar de delegarlos. Requiere Claude Code v2.1.198 o posterior.
* En [modo no interactivo](/docs/es/headless) y el [Agent SDK](/docs/es/agent-sdk/overview), establezca [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/es/env-vars) para eliminar todos los tipos integrados y proporcionar solo los suyos.

Más allá de estos subagentes integrados, puede crear los suyos propios con mensajes personalizados, restricciones de herramientas, modos de permisos, hooks y skills. Las siguientes secciones muestran cómo comenzar y personalizar subagentes.

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

    Claude delega en su nuevo subagente, que escanea la base de código y devuelve sugerencias de mejora.

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
| Directorio `agents/` del plugin | Donde el plugin está habilitado | 5 (más baja) | Instalado con [plugins](/docs/es/plugins)                                |

**Los subagentes de proyecto** (`.claude/agents/`) son ideales para subagentes específicos de una base de código. Verifíquelos en control de versiones para que su equipo pueda usarlos y mejorarlos colaborativamente.

Los subagentes de proyecto se descubren caminando hacia arriba desde el directorio de trabajo actual, por lo que cada `.claude/agents/` entre allí y la raíz del repositorio se escanea. A partir de v2.1.178, cuando más de uno de estos directorios anidados define el mismo `name`, Claude Code usa la definición más cercana al directorio de trabajo.

Los directorios agregados con `--add-dir` también se escanean: una carpeta `.claude/agents/` dentro de un directorio agregado se carga junto con subagentes de proyecto. Consulte [Directorios adicionales](/docs/es/permissions#additional-directories-grant-file-access-not-configuration) para ver qué otros tipos de configuración se cargan desde `--add-dir`. Para compartir subagentes entre proyectos sin `--add-dir`, use `~/.claude/agents/` o un [plugin](/docs/es/plugins).

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

La bandera `--agents` acepta JSON con los mismos campos de [frontmatter](#supported-frontmatter-fields) que los subagentes basados en archivos: `description`, `prompt`, `tools`, `disallowedTools`, `model`, `permissionMode`, `mcpServers`, `hooks`, `maxTurns`, `skills`, `initialPrompt`, `memory`, `effort`, `background`, `isolation` y `color`. Use `prompt` para el mensaje del sistema, equivalente al cuerpo markdown en subagentes basados en archivos.

**Los subagentes administrados** son implementados por administradores de la organización. Coloque archivos markdown en `.claude/agents/` dentro del [directorio de configuración administrada](/docs/es/settings#settings-files), usando el mismo formato de frontmatter que los subagentes de proyecto y usuario. Las definiciones administradas tienen precedencia sobre los subagentes de proyecto y usuario con el mismo nombre.

**Los subagentes de plugin** provienen de [plugins](/docs/es/plugins) que ha instalado. Se cargan junto a sus subagentes personalizados y aparecen en la lista de @-mention bajo su nombre con alcance. Consulte la [referencia de componentes de plugin](/docs/es/plugins-reference#agents) para obtener detalles sobre la creación de subagentes de plugin.

<Note>
  Por razones de seguridad, los subagentes de plugin no soportan los campos de frontmatter `hooks`, `mcpServers`, o `permissionMode`. Estos campos se ignoran al cargar agentes desde un plugin. Si los necesita, copie el archivo del agente en `.claude/agents/` o `~/.claude/agents/`. También puede agregar reglas a [`permissions.allow`](/docs/es/settings#permission-settings) en `settings.json` o `settings.local.json`, pero estas reglas se aplican a toda la sesión, no solo al subagente del plugin.
</Note>

Las definiciones de subagentes de cualquiera de estos alcances también están disponibles para [equipos de agentes](/docs/es/agent-teams#use-subagent-definitions-for-teammates): al generar un compañero de equipo, puede hacer referencia a un tipo de subagente y el compañero hereda sus `tools` y `model`, con el cuerpo de la definición anexado al mensaje del sistema del compañero como instrucciones adicionales. Consulte [equipos de agentes](/docs/es/agent-teams#use-subagent-definitions-for-teammates) para ver qué campos de frontmatter se aplican en esa ruta.

<h3 id="write-subagent-files">
  Escribir archivos de subagentes
</h3>

Los archivos de subagentes usan frontmatter YAML para configuración, seguido del mensaje del sistema en Markdown:

<Note>
  Claude Code observa `~/.claude/agents/` y `.claude/agents/`. Cuando agrega o edita un archivo de subagente en disco, o pide a Claude que escriba uno para usted, Claude Code detecta el cambio dentro de unos pocos segundos y la siguiente delegación usa la definición actualizada, sin necesidad de reinicio.

  Dos casos aún necesitan un reinicio:

  * El observador cubre solo directorios que existían cuando comenzó la sesión, por lo que después de crear el primer archivo de agente de un alcance en un nuevo directorio `agents`, reinicie para cargarlo.
  * Las sesiones iniciadas con `--disable-slash-commands` no observan estos directorios en absoluto.
</Note>

```markdown theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
tools: Read, Glob, Grep
model: sonnet
---

You are a code reviewer. When invoked, analyze the code and provide
specific, actionable feedback on quality, security, and best practices.
```

El frontmatter define los metadatos y la configuración del subagente. El cuerpo se convierte en el mensaje del sistema que guía el comportamiento del subagente. Los subagentes reciben solo este mensaje del sistema más detalles básicos del entorno como el directorio de trabajo, no el mensaje del sistema completo de Claude Code.

En [modo no interactivo](/docs/es/headless), la bandera [`--append-subagent-system-prompt`](/docs/es/cli-reference#cli-flags) añade el texto que proporciona al final del mensaje del sistema de cada subagente, incluyendo subagentes anidados. Requiere Claude Code v2.1.205 o posterior.

Un subagente comienza en el directorio de trabajo actual de la conversación principal. Dentro de un subagente, los comandos `cd` no persisten entre llamadas de herramientas Bash o PowerShell y no afectan el directorio de trabajo de la conversación principal. Para dar al subagente una copia aislada del repositorio en su lugar, establezca [`isolation: worktree`](#supported-frontmatter-fields).

Un subagente con `isolation: worktree` ejecuta sus comandos Bash y PowerShell dentro de su worktree. Un comando cuyo directorio de trabajo se resuelve a su checkout principal en su lugar, por ejemplo porque el directorio worktree fue eliminado mientras el subagente estaba ejecutándose, falla con un error. Antes de v2.1.203, tal comando podría ejecutarse en el checkout principal.

<h4 id="supported-frontmatter-fields">
  Campos de frontmatter soportados
</h4>

Los siguientes campos se pueden usar en el frontmatter YAML. Solo `name` y `description` son requeridos.

| Campo             | Requerido | Descripción                                                                                                                                                                                                                                                                                                                                                                                              |
| :---------------- | :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`            | Sí        | Identificador único usando letras minúsculas y guiones. [Hooks](/docs/es/hooks#subagentstart) reciben este valor como `agent_type`. El nombre del archivo no tiene que coincidir                                                                                                                                                                                                                              |
| `description`     | Sí        | Cuándo Claude debe delegar en este subagente                                                                                                                                                                                                                                                                                                                                                             |
| `tools`           | No        | [Herramientas](#available-tools) que el subagente puede usar. Hereda todas las herramientas si se omite. Si ninguna entrada en la lista se resuelve a una herramienta, el subagente falla al lanzarse con un error nombrando las entradas. Para precargar Skills en el contexto, use el campo `skills` en lugar de listar `Skill` aquí                                                                   |
| `disallowedTools` | No        | Herramientas a denegar, eliminadas de la lista heredada o especificada                                                                                                                                                                                                                                                                                                                                   |
| `model`           | No        | [Modelo](#choose-a-model) a usar: `sonnet`, `opus`, `haiku`, `fable`, un ID de modelo completo (por ejemplo, `claude-opus-4-8`), o `inherit`. Por defecto es `inherit`                                                                                                                                                                                                                                   |
| `permissionMode`  | No        | [Modo de permiso](#permission-modes): `default`, `acceptEdits`, `auto`, `dontAsk`, `bypassPermissions`, `plan`, o `manual` como alias para `default`. El alias `manual` requiere Claude Code v2.1.200 o posterior. Se ignora para [subagentes de plugin](#choose-the-subagent-scope)                                                                                                                     |
| `maxTurns`        | No        | Número máximo de turnos de agente antes de que el subagente se detenga                                                                                                                                                                                                                                                                                                                                   |
| `skills`          | No        | [Skills](/docs/es/skills) a precargar en el contexto del subagente al inicio. El contenido completo de la skill se inyecta, no solo la descripción. Los subagentes aún pueden invocar skills de proyecto, usuario y plugin no listadas a través de la herramienta Skill                                                                                                                                       |
| `mcpServers`      | No        | [Servidores MCP](/docs/es/mcp) disponibles para este subagente. Cada entrada es un nombre de servidor que hace referencia a un servidor ya configurado (por ejemplo, `"slack"`) o una definición en línea con el nombre del servidor como clave y una [configuración completa del servidor MCP](/docs/es/mcp#installing-mcp-servers) como valor. Se ignora para [subagentes de plugin](#choose-the-subagent-scope) |
| `hooks`           | No        | [Hooks de ciclo de vida](#define-hooks-for-subagents) limitados a este subagente. Se ignora para [subagentes de plugin](#choose-the-subagent-scope)                                                                                                                                                                                                                                                      |
| `memory`          | No        | [Alcance de memoria persistente](#enable-persistent-memory): `user`, `project`, o `local`. Habilita aprendizaje entre sesiones                                                                                                                                                                                                                                                                           |
| `background`      | No        | Establecer en `true` para ejecutar siempre este subagente como una [tarea de fondo](#run-subagents-in-foreground-or-background), incluso cuando Claude necesita su resultado de inmediato. Cuando no se establece, Claude elige, y a partir de v2.1.198 ejecuta subagentes en segundo plano por defecto                                                                                                  |
| `effort`          | No        | Nivel de esfuerzo cuando este subagente está activo. Anula el nivel de esfuerzo de la sesión. Por defecto: hereda de la sesión. Opciones: `low`, `medium`, `high`, `xhigh`, `max`; los niveles disponibles dependen del modelo                                                                                                                                                                           |
| `isolation`       | No        | Establecer en `worktree` para ejecutar el subagente en un [git worktree](/docs/es/worktrees) temporal, dándole una copia aislada del repositorio ramificada por defecto desde su [rama predeterminada](/docs/es/worktrees#choose-the-base-branch) en lugar del `HEAD` de la sesión principal. El worktree se limpia automáticamente si el subagente no realiza cambios                                             |
| `color`           | No        | Color de visualización para el subagente en la lista de tareas y transcripción. Acepta `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink`, o `cyan`                                                                                                                                                                                                                                            |
| `initialPrompt`   | No        | Se envía automáticamente como el primer turno de usuario cuando este agente se ejecuta como el agente de sesión principal (a través de `--agent` o la configuración `agent`). Se procesan [comandos](/docs/es/commands) y [skills](/docs/es/skills). Se antepone a cualquier mensaje proporcionado por el usuario                                                                                                  |

<h3 id="choose-a-model">
  Elegir un modelo
</h3>

El campo `model` controla qué [modelo de IA](/docs/es/model-config) usa el subagente:

* **Alias de modelo**: Use uno de los alias disponibles: `sonnet`, `opus`, `haiku`, o `fable`
* **ID de modelo completo**: Use un ID de modelo completo como `claude-opus-4-8` o `claude-sonnet-5`. Acepta los mismos valores que la bandera `--model`
* **inherit**: Use el mismo modelo que la conversación principal
* **Omitido**: Si no se especifica, por defecto es `inherit` (usa el mismo modelo que la conversación principal)

Cuando Claude invoca un subagente, también puede pasar un parámetro `model` para esa invocación específica. Claude Code resuelve el modelo del subagente en este orden:

1. La variable de entorno [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/es/model-config#environment-variables), cuando está establecida en un alias de modelo o ID de modelo
2. El parámetro `model` por invocación
3. El frontmatter `model` de la definición del subagente
4. El modelo de la conversación principal

A partir de v2.1.196, establecer `CLAUDE_CODE_SUBAGENT_MODEL` en `inherit` es lo mismo que dejarlo sin establecer: la resolución continúa con el parámetro `model` por invocación, luego el frontmatter. En versiones anteriores, `inherit` forzaba subagentes al modelo de la conversación principal e ignoraba ambas fuentes.

Claude Code verifica la variable de entorno, el parámetro por invocación y los valores de frontmatter contra la lista de permitidos [`availableModels`](/docs/es/model-config#restrict-model-selection) de su organización. Un valor que se resuelve a un modelo excluido no se usa y el subagente se ejecuta en el modelo heredado en su lugar.

A partir de v2.1.198, los subagentes también heredan la configuración de [pensamiento extendido](/docs/es/model-config#extended-thinking) de la conversación principal: si el pensamiento está activado en su sesión, está activado para el subagente, y si está desactivado, permanece desactivado. No hay una configuración de pensamiento por subagente. Antes de v2.1.198, los subagentes se ejecutaban con pensamiento extendido deshabilitado independientemente de la configuración de la conversación principal.

<h3 id="control-subagent-capabilities">
  Controlar capacidades de subagentes
</h3>

Puede controlar qué pueden hacer los subagentes a través del acceso a herramientas, modos de permisos y reglas condicionales.

<h4 id="available-tools">
  Herramientas disponibles
</h4>

Los subagentes heredan las [herramientas internas](/docs/es/tools-reference) y herramientas MCP disponibles en la conversación principal por defecto. Las siguientes herramientas dependen de la interfaz de usuario o estado de sesión de la conversación principal y no están disponibles para subagentes, incluso cuando se enumeran en el campo `tools`:

* `AskUserQuestion`
* `EnterPlanMode`
* `ExitPlanMode`, a menos que el [`permissionMode`](#permission-modes) del subagente sea `plan`
* `ScheduleWakeup`
* `WaitForMcpServers`

Para restringir herramientas, use el campo `tools` (lista blanca) o el campo `disallowedTools` (lista negra). Este ejemplo usa `tools` para permitir exclusivamente Read, Grep, Glob y Bash. El subagente no puede editar archivos, escribir archivos, o usar ninguna herramienta MCP:

```yaml theme={null}
---
name: safe-researcher
description: Research agent with restricted capabilities
tools: Read, Grep, Glob, Bash
---
```

Este ejemplo usa `disallowedTools` para heredar todas las herramientas de la conversación principal excepto Write y Edit. El subagente mantiene Bash, herramientas MCP y todo lo demás:

```yaml theme={null}
---
name: no-writes
description: Inherits every tool except file writes
disallowedTools: Write, Edit
---
```

Si ambos se establecen, `disallowedTools` se aplica primero, luego `tools` se resuelve contra el grupo restante. Una herramienta listada en ambos se elimina.

Cuando nada en la lista `tools` se resuelve a una herramienta, por ejemplo porque cada entrada está mal escrita o nombra una herramienta que no está disponible para subagentes, Claude Code se niega a lanzar el subagente y la herramienta Agent devuelve un error nombrando las entradas no resueltas. Antes de v2.1.208, ese subagente se lanzaba sin herramientas y podría devolver un resultado vacío o confuso.

Ambos campos aceptan patrones a nivel de servidor MCP además de nombres de herramientas exactos: `mcp__<server>` o `mcp__<server>__*` otorga o elimina todas las herramientas del servidor nombrado. En `disallowedTools`, `mcp__*` también elimina todas las herramientas MCP de cualquier servidor. Este ejemplo elimina todas las herramientas del servidor MCP `github` mientras mantiene herramientas de otros servidores y todas las herramientas integradas:

```yaml theme={null}
---
name: local-only
description: Inherits every tool except those from the github MCP server
disallowedTools: mcp__github
---
```

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

Si `Agent` se omite completamente de la lista `tools`, el agente no puede generar ningún subagente.

La sintaxis de lista blanca `Agent(agent_type)` se aplica solo a un agente que se ejecuta como el hilo principal con `claude --agent`. En una definición de subagente, listar `Agent` en `tools` permite que ese subagente [genere subagentes anidados](#spawn-nested-subagents), pero cualquier lista de tipos dentro de los paréntesis se ignora.

<h4 id="scope-mcp-servers-to-a-subagent">
  Alcance de servidores MCP a un subagente
</h4>

Use el campo `mcpServers` para dar a un subagente acceso a servidores [MCP](/docs/es/mcp) que no están disponibles en la conversación principal. Los servidores en línea definidos aquí se conectan cuando el subagente comienza y se desconectan cuando termina. Las referencias de cadena comparten la conexión de la sesión principal.

<Note>
  El campo `mcpServers` se aplica en ambos contextos donde un archivo de agente puede ejecutarse:

  * Como un subagente, generado a través de la herramienta Agent o una @-mención
  * Como la sesión principal, lanzada con [`--agent`](#invoke-subagents-explicitly) o la configuración `agent`

  Cuando el agente es la sesión principal, las definiciones de servidor en línea se conectan al inicio junto con servidores de [`.mcp.json`](/docs/es/mcp) y archivos de configuración.
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

A partir de v2.1.153, las restricciones de MCP que se aplican a la sesión principal también cubren servidores declarados en frontmatter de subagentes:

* [`--strict-mcp-config`](/docs/es/cli-reference) y [`--bare`](/docs/es/cli-reference)
* [Configuración de MCP administrada empresarial](/docs/es/managed-mcp)
* [Políticas `allowedMcpServers` y `deniedMcpServers`](/docs/es/managed-mcp#policy-based-control-with-allowlists-and-denylists)

Cuando uno de estos bloquea un servidor, Claude Code lo omite y muestra una advertencia nombrando los servidores bloqueados.

Las restricciones de configuración administrada se aplican a cada subagente independientemente de cómo se defina. `--strict-mcp-config` no filtra servidores que pase en línea a través de `--agents` o la opción `agents` del SDK, ya que esa es entrada explícita del llamador.

<h4 id="permission-modes">
  Modos de permiso
</h4>

El campo `permissionMode` controla cómo el subagente maneja solicitudes de permiso. Los subagentes heredan el contexto de permiso de la conversación principal y pueden anular el modo, excepto cuando el modo principal tiene precedencia como se describe a continuación.

| Modo                | Comportamiento                                                                                                                                                                                                                                                                                                                                                                                   |
| :------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Verificación de permiso estándar con solicitudes                                                                                                                                                                                                                                                                                                                                                 |
| `acceptEdits`       | Aceptar automáticamente ediciones de archivo y comandos comunes del sistema de archivos para rutas en el directorio de trabajo o `additionalDirectories`                                                                                                                                                                                                                                         |
| `auto`              | [Modo auto](/docs/es/permission-modes#eliminate-prompts-with-auto-mode): un clasificador de IA evalúa cada llamada de herramienta                                                                                                                                                                                                                                                                     |
| `dontAsk`           | Denegar automáticamente solicitudes de permiso. Las herramientas explícitamente permitidas aún funcionan; `AskUserQuestion`, herramientas de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools), y herramientas MCP marcadas [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool) se deniegan incluso si las ha permitido |
| `bypassPermissions` | Omitir solicitudes de permiso                                                                                                                                                                                                                                                                                                                                                                    |
| `plan`              | Modo plan (exploración de solo lectura)                                                                                                                                                                                                                                                                                                                                                          |

<Warning>
  Use `bypassPermissions` con cuidado. Omite solicitudes de permiso, permitiendo que el subagente ejecute operaciones sin aprobación, incluyendo escrituras en `.git`, `.config/git`, `.claude`, `.vscode`, `.idea`, `.husky`, `.cargo`, `.devcontainer`, `.yarn` y `.mvn`.

  Las reglas explícitas de [`ask`](/docs/es/permissions#manage-permissions), herramientas de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools), herramientas MCP marcadas [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool), y eliminaciones de directorio raíz y directorio de inicio como `rm -rf /` aún solicitan confirmación. Consulte [modos de permiso](/docs/es/permission-modes#skip-all-checks-with-bypasspermissions-mode) para detalles.
</Warning>

Si el principal usa `bypassPermissions` o `acceptEdits`, esto tiene precedencia y no puede ser anulado. Si el principal usa [modo auto](/docs/es/permission-modes#eliminate-prompts-with-auto-mode), el subagente hereda modo auto y cualquier `permissionMode` en su frontmatter se ignora: el clasificador evalúa las llamadas de herramientas del subagente con las mismas reglas de bloqueo y permiso que la sesión principal.

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

No puede precargar skills que establezcan [`disable-model-invocation: true`](/docs/es/skills#control-who-invokes-a-skill), ya que la precarga se extrae del mismo conjunto de skills que Claude puede invocar. Si una skill listada falta o está deshabilitada, Claude Code la omite y registra una advertencia en el registro de depuración.

<Note>
  Esto es lo inverso de [ejecutar una skill en un subagente](/docs/es/skills#run-skills-in-a-subagent). Con `skills` en un subagente, el subagente controla el mensaje del sistema y carga contenido de skill. Con `context: fork` en una skill, el contenido de la skill se inyecta en el agente que especifique. Ambos usan el mismo sistema subyacente.
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

Consulte [Hook input](/docs/es/hooks#pretooluse-input) para el esquema de entrada completo y [códigos de salida](/docs/es/hooks#exit-code-output) para cómo los códigos de salida afectan el comportamiento. En Windows, escriba scripts de hook en PowerShell y agregue `shell: powershell` a la entrada del hook como se muestra en [ejecutar hooks en PowerShell](/docs/es/hooks#windows-powershell-tool).

<h4 id="disable-specific-subagents">
  Deshabilitar subagentes específicos
</h4>

Puede evitar que Claude use subagentes específicos agregándolos a la matriz `deny` en su [configuración](/docs/es/settings#permission-settings). Use el formato `Agent(subagent-name)` donde `subagent-name` coincida con el campo name del subagente.

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
* **En `settings.json`**: defina hooks que se ejecuten en la sesión principal cuando los subagentes comienzan o se detienen

<h4 id="hooks-in-subagent-frontmatter">
  Hooks en frontmatter de subagentes
</h4>

Defina hooks directamente en el archivo markdown del subagente. Estos hooks solo se ejecutan mientras ese subagente específico está activo y se limpian cuando termina.

<Note>
  Los hooks de frontmatter se disparan cuando el agente se genera como un subagente a través de la herramienta Agent o una @-mención, y cuando el agente se ejecuta como la sesión principal a través de [`--agent`](#invoke-subagents-explicitly) o la configuración `agent`. En el caso de sesión principal, se ejecutan junto con cualquier hook definido en [`settings.json`](/docs/es/hooks).
</Note>

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

Ambos eventos soportan matchers para dirigirse a tipos de agentes específicos por nombre. El valor del matcher es el `name` del frontmatter del agente para subagentes a nivel de proyecto y usuario, o el identificador con alcance de plugin como `my-plugin:db-agent` para [subagentes de plugin](/docs/es/plugins). Un nombre con alcance contiene dos puntos, por lo que se evalúa como una [expresión regular sin anclar](/docs/es/hooks#matcher-patterns); anclarlo con `^` y `$`, como en `^my-plugin:db-agent$`, para coincidir solo con ese agente.

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
  Entender delegación automática
</h3>

Claude delega automáticamente tareas basadas en la descripción de la tarea en su solicitud, el campo `description` en configuraciones de subagentes y el contexto actual. Para alentar delegación proactiva, incluya frases como "use proactively" en el campo description de su subagente.

<h3 id="invoke-subagents-explicitly">
  Invocar subagentes explícitamente
</h3>

Cuando la delegación automática no es suficiente, puede solicitar un subagente usted mismo. Tres patrones escalan desde una sugerencia única a un valor predeterminado de sesión completa:

* **Lenguaje natural**: nombre el subagente en su solicitud; Claude decide si delegar
* **@-mention**: garantiza que el subagente se ejecute para una tarea
* **Sesión completa**: toda la sesión usa el mensaje del sistema del subagente, restricciones de herramientas y modelo a través de la bandera `--agent` o la configuración `agent`

Para lenguaje natural, no hay sintaxis especial. Nombre el subagente y Claude típicamente delega:

```text wrap theme={null}
Use the test-runner subagent to fix failing tests
Have the code-reviewer subagent look at my recent changes
```

**@-mention el subagente.** Escriba `@` y elija el subagente del typeahead, de la misma manera que @-menciona archivos. Esto asegura que ese subagente específico se ejecute en lugar de dejar la opción a Claude:

```text wrap theme={null}
@"code-reviewer (agent)" look at the auth changes
```

Su mensaje completo aún va a Claude, que escribe el mensaje de tarea del subagente basado en lo que pidió. El @-mention controla qué subagente Claude invoca, no qué mensaje recibe.

Los subagentes proporcionados por un [plugin](/docs/es/plugins) habilitado aparecen en el typeahead bajo su nombre con alcance, como `my-plugin:code-reviewer` o `my-plugin:review:security` cuando el plugin [organiza agentes en subcarpetas](#choose-the-subagent-scope). Los subagentes de fondo nombrados actualmente en ejecución en la sesión también aparecen en el typeahead, mostrando su estado junto al nombre.

Puede también escribir la mención manualmente sin usar el selector: `@agent-<name>` para subagentes locales, o `@agent-` seguido del nombre con alcance para subagentes de plugin, por ejemplo `@agent-my-plugin:code-reviewer`.

**Ejecute toda la sesión como un subagente.** Pase [`--agent <name>`](/docs/es/cli-reference) para iniciar una sesión donde el hilo principal en sí toma el mensaje del sistema del subagente, restricciones de herramientas y modelo:

```bash theme={null}
claude --agent code-reviewer
```

El mensaje del sistema del subagente reemplaza completamente el mensaje del sistema predeterminado de Claude Code, de la misma manera que [`--system-prompt`](/docs/es/cli-reference) lo hace. Los archivos `CLAUDE.md` y la memoria del proyecto aún se cargan a través del flujo de mensajes normal. El nombre del agente aparece como `@<name>` en el encabezado de inicio para que pueda confirmar que está activo.

Esto funciona con subagentes integrados y personalizados, y la opción persiste cuando reanuda la sesión.

Para un subagente proporcionado por plugin, puede pasar solo el nombre del agente y Claude Code lo encontrará:

```bash theme={null}
claude --agent security-reviewer
```

Si múltiples plugins proporcionan agentes con el mismo nombre, pase el nombre con alcance para desambiguar:

```bash theme={null}
claude --agent my-plugin:security-reviewer
```

Si el plugin coloca el agente en una subcarpeta de su directorio `agents/`, incluya la subcarpeta en el nombre con alcance, por ejemplo `claude --agent my-plugin:review:security`.

Para hacerlo el predeterminado para cada sesión en un proyecto, establezca `agent` en `.claude/settings.json`:

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

* **Subagentes en primer plano** bloquean la conversación principal hasta completarse. Las solicitudes de permiso se le pasan a usted a medida que surgen.
* **Subagentes en fondo** se ejecutan concurrentemente mientras continúa trabajando. A partir de v2.1.186, cuando un subagente en fondo alcanza una llamada de herramienta que necesita permiso, la solicitud aparece en su sesión principal y nombra el subagente que está preguntando. Apruebe para permitir que el subagente continúe, o presione Esc para denegar esa llamada de herramienta sin detener el subagente. Antes de v2.1.186, los subagentes en fondo denegaban automáticamente cualquier llamada de herramienta que habría solicitado.

A partir de v2.1.198, los subagentes se ejecutan en el fondo de forma predeterminada. Claude ejecuta un subagente en primer plano cuando necesita el resultado antes de continuar. El valor predeterminado cambia dónde se ejecuta un subagente, no qué se le permite hacer: los subagentes en fondo aún muestran cada solicitud de permiso en su sesión principal. Antes de v2.1.198, Claude elegía entre primer plano y fondo basado en la tarea.

También puede dirigir esto usted mismo:

* Pida a Claude que ejecute una tarea en el fondo o en primer plano
* Presione **Ctrl+B** para poner en fondo una tarea en ejecución

Un subagente en fondo que se completa permanece listado en [`/tasks`](/docs/es/commands), marcado como hecho y ordenado debajo del trabajo en ejecución, hasta que la sesión limpie su lista de tareas. Su vista de detalle permanece abierta cuando el subagente termina. Los subagentes que fallan o que usted detiene dejan la lista. Antes de v2.1.208, un subagente completado dejaba la lista en el momento en que terminaba y su vista de detalle se cerraba.

Para deshabilitar toda la funcionalidad de tareas en fondo, establezca la variable de entorno `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` en `1`. Consulte [Variables de entorno](/docs/es/env-vars).

Cuando [`CLAUDE_CODE_FORK_SUBAGENT`](#fork-the-current-conversation) está establecido en `1`, cada generación de subagente se ejecuta en el fondo y el campo frontmatter `background` no tiene efecto, porque el modo fork elimina el parámetro `run_in_background` de la herramienta `Agent`. `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` tiene precedencia sobre el modo fork y mantiene las generaciones de subagentes en primer plano.

<h3 id="api-errors-in-subagents">
  Errores de API en subagentes
</h3>

A partir de v2.1.199, un subagente cuya ejecución termina en un error de API, como un límite de uso o un error de servidor repetido, reporta esa falla de vuelta a Claude en lugar de devolver el texto de error como si fueran los hallazgos del subagente. Lo que Claude recibe depende de dónde se ejecutó el subagente:

* **Primer plano**: si un límite de velocidad, sobrecarga o error de servidor corta un subagente que ya produjo salida, la herramienta Agent devuelve esa salida parcial con una nota de que el subagente fue cortado y no completó su tarea. Un subagente que no produjo nada, o cuya única salida fueron llamadas de herramientas, falla con [`Agent terminated early due to an API error`](/docs/es/errors#agent-terminated-early-due-to-an-api-error), seguido del detalle del error. En v2.1.199, un límite de velocidad, sobrecarga o error de servidor que cortó la forma de solo llamadas de herramientas devolvió un resultado parcial vacío que contenía solo la nota de corte en su lugar.
* **Fondo**: el subagente se marca como fallido, y el mensaje que Claude recibe cuando termina nombra el error de API e incluye la última salida del subagente, por lo que el trabajo parcial no se pierde.

Una vez que el error de API subyacente se resuelve, pida a Claude que reintente la tarea o [reanude el subagente](#resume-subagents).

<h3 id="common-patterns">
  Patrones comunes
</h3>

<h4 id="isolate-high-volume-operations">
  Aislar operaciones de alto volumen
</h4>

Uno de los usos más efectivos para subagentes es aislar operaciones que producen grandes cantidades de salida. Ejecutar pruebas, obtener documentación o procesar archivos de registro puede consumir contexto significativo. Al delegar estos a un subagente, la salida detallada permanece en el contexto del subagente mientras solo el resumen relevante regresa a su conversación principal.

```text wrap theme={null}
Use a subagent to run the test suite and report only the failing tests with their error messages
```

<h4 id="run-parallel-research">
  Ejecutar investigación en paralelo
</h4>

Para investigaciones independientes, genere múltiples subagentes para trabajar simultáneamente:

```text wrap theme={null}
Research the authentication, database, and API modules in parallel using separate subagents
```

Cada subagente explora su área independientemente, luego Claude sintetiza los hallazgos. Esto funciona mejor cuando las rutas de investigación no dependen una de la otra.

<Warning>
  Cuando los subagentes se completan, sus resultados regresan a su conversación principal. Ejecutar muchos subagentes que cada uno devuelve resultados detallados puede consumir contexto significativo.
</Warning>

Para tareas que necesitan paralelismo sostenido o exceden su ventana de contexto, [equipos de agentes](/docs/es/agent-teams) dan a cada trabajador su propio contexto independiente.

<h4 id="chain-subagents">
  Encadenar subagentes
</h4>

Para flujos de trabajo de múltiples pasos, pida a Claude que use subagentes en secuencia. Cada subagente completa su tarea y devuelve resultados a Claude, que luego pasa contexto relevante al siguiente subagente.

```text wrap theme={null}
Use the code-reviewer subagent to find performance issues, then use the optimizer subagent to fix them
```

<h3 id="choose-between-subagents-and-main-conversation">
  Elegir entre subagentes y conversación principal
</h3>

Use la **conversación principal** cuando:

* La tarea necesita ida y vuelta frecuente o refinamiento iterativo
* Múltiples fases comparten contexto significativo, como planificación, implementación y prueba
* Está haciendo un cambio rápido y dirigido
* La latencia importa. Los subagentes comienzan frescos y pueden necesitar tiempo para recopilar contexto

Use **subagentes** cuando:

* La tarea produce salida detallada que no necesita en su contexto principal
* Desea aplicar restricciones de herramientas específicas o permisos
* El trabajo es autónomo y puede devolver un resumen

Considere [Skills](/docs/es/skills) en su lugar cuando desee mensajes reutilizables o flujos de trabajo que se ejecuten en el contexto de conversación principal en lugar de contexto de subagente aislado.

Para una pregunta rápida sobre algo ya en su conversación, use [`/btw`](/docs/es/interactive-mode#side-questions-with-%2Fbtw) en lugar de un subagente. Ve su contexto completo pero no tiene acceso a herramientas, y la respuesta se descarta en lugar de agregarse al historial.

<h3 id="spawn-nested-subagents">
  Generar subagentes anidados
</h3>

A partir de Claude Code v2.1.172, un subagente puede generar sus propios subagentes. Use esto cuando una tarea delegada se divide en subtareas paralelas, como un subagente revisor que distribuye un verificador por hallazgo, de modo que la salida intermedia nunca llegue a su conversación principal. Solo el resumen del subagente de nivel superior regresa a usted.

Un subagente anidado se configura de la misma manera que uno de nivel superior y se resuelve desde los mismos [alcances](#choose-the-subagent-scope). El panel de subagentes debajo de la entrada de solicitud muestra el árbol completo: cada fila muestra un recuento `(+N)` de descendientes, y a partir de v2.1.193, abrir una fila muestra los hermanos de ese subagente e hijos directos con una ruta de regreso a `main`.

La profundidad se cuenta como el número de niveles de subagentes debajo de la conversación principal, independientemente de si cada nivel se ejecuta en [primer plano o fondo](#run-subagents-in-foreground-or-background). Un subagente a profundidad cinco no recibe la herramienta Agent y no puede generar más. El límite es fijo y no configurable.

A partir de Claude Code v2.1.187, la profundidad de un subagente en fondo se fija cuando se genera por primera vez, y [reanudar](#resume-subagents) más tarde no cambia esa profundidad. Por ejemplo, si su conversación principal genera el subagente A, y A genera un subagente en fondo B a profundidad dos, B sigue siendo a profundidad dos cuando lo reanuda directamente desde la conversación principal. Reanudar un subagente desde un contexto más superficial no le permite generar niveles adicionales que el límite de profundidad ya impidió.

Para prevenir que un subagente específico genere otros, omita `Agent` de su lista [`tools`](#available-tools) o añádalo a `disallowedTools`.

Un [fork](#fork-the-current-conversation) aún no puede generar otro fork. Puede generar otros tipos de subagentes, y esos cuentan hacia el límite de profundidad.

<h3 id="manage-subagent-context">
  Administrar contexto de subagentes
</h3>

<h4 id="what-loads-at-startup">
  Qué se carga al inicio
</h4>

Cada subagente comienza con una ventana de contexto fresca e aislada. No ve su historial de conversación, las habilidades que ya ha invocado, o los archivos que Claude ya ha leído. Claude compone un mensaje de delegación que resume la tarea, y el subagente trabaja a partir de ahí. La excepción es un [fork](#fork-the-current-conversation), que hereda la conversación principal en lugar de comenzar de nuevo.

El contexto inicial de un subagente que no es fork contiene:

* **Mensaje del sistema**: el mensaje del agente propio más detalles de entorno que Claude Code añade, no el mensaje del sistema completo de Claude Code. Los subagentes personalizados definen el suyo en el [cuerpo markdown](#write-subagent-files) o campo `prompt`. Los agentes integrados tienen mensajes predefinidos.
* **Mensaje de tarea**: el mensaje de delegación que Claude escribe cuando entrega el trabajo.
* **CLAUDE.md y memoria**: cada nivel de la [jerarquía de memoria](/docs/es/memory#how-claude-md-files-load) que la conversación principal carga, incluyendo `~/.claude/CLAUDE.md`, reglas del proyecto, `CLAUDE.local.md` y archivos de política administrados. Los agentes Explore y Plan integrados omiten esto.
* **Estado de Git**: una instantánea tomada al inicio de la sesión principal. Ausente cuando el directorio de trabajo no es un repositorio de Git o cuando [`includeGitInstructions`](/docs/es/settings#available-settings) es `false`. Explore y Plan lo omiten de todas formas.
* **Habilidades precargadas**: contenido completo de cualquier habilidad nombrada en el campo [`skills`](#preload-skills-into-subagents) del agente. Los agentes integrados no precargan habilidades.
* **Roster de hermanos**: un recordatorio del sistema que enumera `main` y cada otro agente nombrado en la sesión, cada uno un valor `to` válido para [`SendMessage`](#resume-subagents). Requiere Claude Code v2.1.206 o posterior. El roster aparece solo cuando las herramientas del subagente incluyen `SendMessage` y al menos otro agente tiene un nombre, ya sea que Claude lo nombró cuando lo generó o se ejecuta como un compañero de [equipo de agentes](/docs/es/agent-teams). Es una instantánea tomada cuando el subagente comienza, por lo que los agentes nombrados más tarde no aparecen.

Explore y Plan son los únicos subagentes que omiten CLAUDE.md y estado de Git. No hay campo de frontmatter o configuración por agente para cambiar qué agentes los omiten.

La conversación principal lee resultados de Explore y Plan con contexto completo de CLAUDE.md, por lo que la mayoría de reglas no necesitan llegar al subagente en sí. Si una regla debe, como "ignore el directorio `vendor/`", restate la en el mensaje que da a Claude cuando delega.

<h4 id="resume-subagents">
  Reanudar subagentes
</h4>

Cada invocación de subagente crea una nueva instancia con contexto fresco. Para continuar el trabajo de un subagente existente en lugar de comenzar de nuevo, pida a Claude que lo reanude.

Los subagentes reanudados retienen su historial de conversación completo, incluyendo todas las llamadas de herramientas anteriores, resultados y razonamiento. El subagente continúa exactamente donde se detuvo en lugar de comenzar de nuevo.

Cuando un subagente se completa, Claude recibe su ID de agente. Los agentes integrados Explore y Plan son de una sola ejecución y no devuelven ID de agente, por lo que no pueden reanudarse; use `general-purpose` o un subagente personalizado cuando necesite continuar el trabajo.

Claude usa la herramienta `SendMessage` con el ID del agente o nombre como campo `to` para reanudarlo. `SendMessage` no requiere que [equipos de agentes](/docs/es/agent-teams) estén habilitados; solo los mensajes de protocolo de equipo estructurados como `shutdown_request` y `plan_approval_response` lo hacen.

Para reanudar un subagente, pida a Claude que continúe el trabajo anterior:

```text wrap theme={null}
Use the code-reviewer subagent to review the authentication module
[Agent completes]

Continue that code review and now analyze the authorization logic
[Claude resumes the subagent with full context from previous conversation]
```

Un subagente completado que recibe un `SendMessage` se reanuda automáticamente en el fondo sin una nueva invocación de `Agent`. Lo mismo aplica a un subagente que Claude detuvo con la herramienta `TaskStop`.

A partir de v2.1.191, un subagente que usted detuvo, con `x` en `/tasks` o una solicitud SDK `stop_task`, no se reanuda automáticamente. La llamada `SendMessage` devuelve un rechazo diciéndole a Claude que el agente fue cancelado. Escriba en la transcripción de ese subagente en el panel de subagentes para reanudarlo usted mismo, lo que borra la parada para que llamadas `SendMessage` posteriores puedan reanudarlo automáticamente de nuevo.

Reanudar inicia una nueva ejecución del agente bajo el mismo ID, por lo que un subagente que ya había fallado o completado se muestra como ejecutándose de nuevo en la lista de tareas y en los eventos de tareas del SDK del Agent. Antes de v2.1.205, seguía mostrando su estado anterior fallido o completado mientras la ejecución reanudada estaba funcionando.

A partir de v2.1.199, `SendMessage` verifica que un nombre aún se refiera al mismo agente que alcanzó anteriormente en la conversación. Si un agente más nuevo ha tomado el nombre, como un subagente en fondo re-generado que lo reutilizó, Claude Code rechaza el envío en lugar de entregarlo al agente incorrecto, y el error reporta qué agente el nombre ahora alcanza para que Claude pueda redirigirse. Para alcanzar el agente anterior mientras aún se está ejecutando, Claude lo dirige por el ID del agente del resultado de generación. La verificación se limita a la conversación actual y se reinicia en `/clear`.

A partir de v2.1.198, un subagente trata los mensajes del agente que lo lanzó como dirección de tarea normal, incluyendo correcciones de curso a mitad de tarea, y actúa sobre ellos dentro de su propia configuración de permisos. Dos límites aún se mantienen independientemente de quién envió el mensaje: ningún mensaje de ningún agente cuenta como su aprobación para una solicitud de permiso pendiente, y ningún mensaje de agente puede cambiar la configuración de permisos de un subagente, `CLAUDE.md` o configuración. Solo el sistema de permisos o sus propios mensajes pueden otorgar aprobación.

También puede pedir a Claude el ID del agente si desea referenciarlo explícitamente, o encontrar IDs en los archivos de transcripción en `~/.claude/projects/{project}/{sessionId}/subagents/`. Cada transcripción se almacena como `agent-{agentId}.jsonl`.

Las transcripciones de subagentes persisten independientemente de la conversación principal:

* **Compactación de conversación principal**: Cuando la conversación principal se compacta, las transcripciones de subagentes no se ven afectadas. Se almacenan en archivos separados.
* **Persistencia de sesión**: Las transcripciones de subagentes persisten dentro de su sesión. Puede [reanudar un subagente](#resume-subagents) después de reiniciar Claude Code reanudando la misma sesión.
* **Limpieza automática**: Las transcripciones se limpian basadas en la configuración `cleanupPeriodDays`, que por defecto es 30 días.

<h4 id="auto-compaction">
  Auto-compactación
</h4>

Los subagentes soportan compactación automática usando la misma lógica que la conversación principal. La compactación se dispara bajo las mismas condiciones, y `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` se aplica a subagentes también. Consulte [variables de entorno](/docs/es/env-vars) para cuándo entra en vigor el override.

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
  Los subagentes bifurcados requieren Claude Code v2.1.117 o posterior. A partir de v2.1.161, el comando `/fork` está habilitado de forma predeterminada; en versiones anteriores requiere establecer la variable de entorno [`CLAUDE_CODE_FORK_SUBAGENT`](/docs/es/env-vars) en `1`. Permitir que Claude mismo genere bifurcaciones es experimental y puede cambiar en futuras versiones. Esta capacidad también puede habilitarse en sesiones interactivas como parte de un lanzamiento por fases.
</Note>

Un fork es un subagente que hereda toda la conversación hasta ahora en lugar de comenzar de nuevo. Esto elimina el aislamiento de entrada que los subagentes de otra manera proporcionan: un fork ve el mismo mensaje del sistema, herramientas, modelo e historial de mensajes que la sesión principal, para que pueda entregarle una tarea secundaria sin re-explicar la situación. Las propias llamadas de herramientas del fork aún permanecen fuera de su conversación y solo su resultado final regresa, por lo que su ventana de contexto principal permanece limpia. Use un fork cuando un subagente nombrado necesitaría demasiado contexto para ser útil, o cuando desee probar varios enfoques en paralelo desde el mismo punto de partida.

Para controlar el modo fork independientemente del lanzamiento por fases, establezca [`CLAUDE_CODE_FORK_SUBAGENT`](/docs/es/env-vars) en `1` para habilitarlo explícitamente o en `0` para deshabilitarlo. La variable se respeta en modo interactivo y a través del SDK o `claude -p`.

Habilitar el modo fork cambia Claude Code de dos maneras:

* Claude puede generar un fork solicitando explícitamente el tipo de subagente `fork`. Los spawns sin un tipo de subagente aún utilizan el subagente [general-purpose](#built-in-subagents), y los subagentes nombrados como Explore aún se generan como antes.
* Cada generación de subagente se ejecuta en el [fondo](#run-subagents-in-foreground-or-background), ya sea un fork o un subagente nombrado. Establezca `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` en `1` para mantener los spawns síncronos.

Puede iniciar un fork usted mismo con `/fork` seguido de una directiva, con o sin la variable establecida. Claude Code nombra el fork a partir de las primeras palabras de la directiva. El siguiente ejemplo bifurca la conversación para redactar casos de prueba mientras continúa con la implementación en la sesión principal:

```text wrap theme={null}
/fork draft unit tests for the parser changes so far
```

El fork aparece en un panel debajo de su solicitud y se ejecuta en el fondo mientras continúa trabajando. Cuando termina, su resultado llega como un mensaje en su conversación principal. La siguiente sección cubre los controles del panel para observar y dirigir forks mientras se ejecutan.

<h3 id="observe-and-steer-running-forks">
  Observar y dirigir forks en ejecución
</h3>

Los forks en ejecución aparecen en un panel debajo de la entrada de solicitud, con una fila para la sesión principal y una para cada fork. Use estas teclas para interactuar con el panel:

| Tecla     | Acción                                                                          |
| :-------- | :------------------------------------------------------------------------------ |
| `↑` / `↓` | Moverse entre filas                                                             |
| `Enter`   | Abrir la transcripción del fork seleccionado y enviarle mensajes de seguimiento |
| `x`       | Descartar un fork terminado o detener uno en ejecución                          |
| `Esc`     | Devolver el enfoque a la entrada de solicitud                                   |

Con la transcripción de un fork o subagente abierta, los mensajes de seguimiento y las [skills](/docs/es/skills) van a ese agente, pero los comandos integrados aún se ejecutan en su conversación principal. A partir de v2.1.199, escribir `/model` o `/fast` en esa vista muestra un aviso de que cambia el modelo de la conversación principal o el modo rápido, no el del agente visto, en lugar de ejecutarlo silenciosamente.

<h3 id="how-forks-differ-from-named-subagents">
  Cómo los forks difieren de los subagentes nombrados
</h3>

Un fork hereda todo lo que la sesión principal tiene en el momento en que se genera. Un subagente nombrado comienza desde su propia definición.

|                                    | Fork                                    | Subagente nombrado                                                                                                           |
| :--------------------------------- | :-------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| Contexto                           | Historial de conversación completo      | Contexto fresco con la solicitud que pasa                                                                                    |
| Mensaje del sistema y herramientas | Igual que la sesión principal           | Del [archivo de definición](#write-subagent-files) del subagente                                                             |
| Modelo                             | Igual que la sesión principal           | Del campo `model` del subagente                                                                                              |
| Permisos                           | Las solicitudes aparecen en su terminal | [Las solicitudes aparecen en su sesión principal](#run-subagents-in-foreground-or-background) cuando se ejecutan en el fondo |
| Caché de solicitud                 | Compartido con la sesión principal      | Caché separado                                                                                                               |

Porque el mensaje del sistema del fork y las definiciones de herramientas son idénticas al principal, su primera solicitud reutiliza la caché de solicitud del principal. Esto hace que bifurcar sea más económico que generar un subagente fresco para tareas que necesitan el mismo contexto.

Cuando Claude genera un fork a través de la herramienta Agent, puede pasar `isolation: "worktree"` para que las ediciones de archivo del fork se escriban en un git worktree separado en lugar de su checkout.

<h3 id="limitations">
  Limitaciones
</h3>

Establecer `CLAUDE_CODE_FORK_SUBAGENT=1` habilita el modo fork en sesiones interactivas, [modo no interactivo](/docs/es/headless), y el Agent SDK; establecerlo en `0` deshabilita el modo fork en todas partes, incluido cualquier lanzamiento del lado del servidor. Un fork no puede generar más forks.

<h2 id="example-subagents">
  Subagentes de ejemplo
</h2>

Estos ejemplos demuestran patrones efectivos para construir subagentes. Úselos como puntos de partida, o genere una versión personalizada con Claude.

<Tip>
  **Mejores prácticas:**

  * **Diseñe subagentes enfocados:** cada subagente debe sobresalir en una tarea específica
  * **Escriba descripciones detalladas:** Claude usa la descripción para decidir cuándo delegar
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

<h2 id="next-steps">
  Próximos pasos
</h2>

Ahora que entiende subagentes, explore estas características relacionadas:

* [Distribuir subagentes con plugins](/docs/es/plugins) para compartir subagentes entre equipos o proyectos
* [Ejecutar Claude Code programáticamente](/docs/es/headless) con el Agent SDK para CI/CD y automatización
* [Usar servidores MCP](/docs/es/mcp) para dar a los subagentes acceso a herramientas y datos externos
