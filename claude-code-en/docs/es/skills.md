> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Extender Claude con skills

> Cree, gestione y comparta skills para extender las capacidades de Claude en Claude Code. Incluye comandos personalizados y skills agrupados.

Los skills extienden lo que Claude puede hacer. Cree un archivo `SKILL.md` con instrucciones, y Claude lo añade a su kit de herramientas. Claude utiliza skills cuando es relevante, o puede invocar uno directamente con `/skill-name`.

Cree un skill cuando siga pegando las mismas instrucciones, lista de verificación o procedimiento de varios pasos en el chat, o cuando una sección de CLAUDE.md haya crecido hasta convertirse en un procedimiento en lugar de un hecho. A diferencia del contenido de CLAUDE.md, el cuerpo de un skill se carga solo cuando se utiliza, por lo que el material de referencia largo cuesta casi nada hasta que lo necesite.

<Note>
  Para comandos integrados como `/help` y `/compact`, y skills agrupados como `/debug` y `/code-review`, consulte la [referencia de comandos](/docs/es/commands).

  **Los comandos personalizados se han fusionado en skills.** Un archivo en `.claude/commands/deploy.md` y un skill en `.claude/skills/deploy/SKILL.md` crean ambos `/deploy` y funcionan de la misma manera. Sus archivos existentes en `.claude/commands/` siguen funcionando. Los skills añaden características opcionales: un directorio para archivos de apoyo, frontmatter para [controlar si usted o Claude los invoca](#control-who-invokes-a-skill), y la capacidad de que Claude los cargue automáticamente cuando sea relevante.
</Note>

Los skills de Claude Code siguen el estándar abierto [Agent Skills](https://agentskills.io), que funciona en múltiples herramientas de IA. Claude Code extiende el estándar con características adicionales como [control de invocación](#control-who-invokes-a-skill), [ejecución de subagentes](#run-skills-in-a-subagent), e [inyección de contexto dinámico](#inject-dynamic-context). Consulte [Usar frontmatter de skills fuera de Claude Code](#using-skill-frontmatter-outside-claude-code) para saber qué campos de frontmatter forman parte del estándar y cuáles son extensiones de Claude Code.

<h2 id="bundled-skills">
  Skills incluidas
</h2>

Claude Code incluye un conjunto de skills incluidas, como `/doctor`, `/code-review`, `/batch`, `/debug`, `/loop` y `/claude-api`. Las skills incluidas se basan en prompts: proporcionan a Claude instrucciones detalladas y le permiten orquestar el trabajo utilizando sus herramientas. La mayoría de los comandos integrados en su lugar ejecutan lógica fija directamente.

Usted invoca una skill incluida de la misma manera que cualquier otra skill, escribiendo `/` seguido del nombre de la skill. Claude invoca algunas skills incluidas automáticamente cuando es relevante; otras, incluyendo `/verify`, se ejecutan solo cuando usted las invoca, lo que le mantiene en control de cuándo estas comprobaciones de ejecución más larga gastan tiempo y tokens.

La mayoría de las skills incluidas están disponibles en cada sesión. Algunas dependen de una característica específica: `/workflow-authoring`, por ejemplo, está disponible solo cuando [los flujos de trabajo dinámicos](/docs/es/workflows) están habilitados.

Para desactivar las skills incluidas, use la configuración [`disableBundledSkills`](/docs/es/settings-reference#disablebundledskills).

<Note>
  La comprobación de configuración [`/doctor`](/docs/es/commands#all-commands) permanece escribible cuando `disableBundledSkills` está activado, en Claude Code v2.1.205 y posterior. Para ocultarla, establezca la variable de entorno `DISABLE_DOCTOR_COMMAND` o una entrada de [`skillOverrides`](#override-skill-visibility-from-settings) de `"doctor": "off"`. Antes de v2.1.205, `/doctor` era un comando integrado en lugar de una skill incluida.
</Note>

Las skills incluidas se enumeran junto con los comandos integrados en la [referencia de comandos](/docs/es/commands), marcadas como **Skill** en la columna Propósito.

<h3 id="run-and-verify-your-app">
  Ejecute y verifique su aplicación
</h3>

Tres skills incluidas funcionan juntas para lanzar su aplicación y confirmar cambios contra la aplicación en ejecución en lugar de solo pruebas:

| Skill                  | Propósito                                                                                                                                   |
| :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| `/run`                 | Inicie y maneje su aplicación para ver un cambio funcionando                                                                                |
| `/verify`              | Compile y ejecute su aplicación para confirmar que un cambio de código hace lo que debería, sin recurrir a pruebas o comprobaciones de tipo |
| `/run-skill-generator` | Enseñe a `/run` y `/verify` cómo compilar e iniciar su proyecto                                                                             |

`/run` y `/verify` funcionan sin configuración. Deducen el lanzamiento de su tipo de proyecto (CLI, servidor, TUI, impulsado por navegador) y de lo que hay en su README, `package.json` o `Makefile`. Esa deducción se vuelve poco confiable para proyectos que necesitan algo más allá de un lanzamiento estándar: una base de datos, un archivo env, una sesión gráfica, una compilación de varios pasos.

`/run-skill-generator` registra la receta en su lugar. Consigue que su aplicación se ejecute desde un entorno limpio, captura lo que funcionó (los comandos de instalación, las variables de entorno, el script de lanzamiento) y lo confirma como una skill por proyecto en `.claude/skills/run-<name>/`. Después de eso, `/run`, `/verify` y cualquier otro agente en el repositorio siguen la receta registrada en lugar de redescubrirla. Ejecute `/run-skill-generator` una vez por proyecto, y nuevamente si el proceso de compilación o lanzamiento cambia.

`/verify` también puede registrar su propia receta. Cuando tiene que compilar e impulsar su aplicación sin una receta registrada, escribe lo que funcionó en `.claude/skills/verify/SKILL.md` en la raíz del repositorio, o en el directorio del paquete tocado en un monorepo, para que las ejecuciones posteriores y otros agentes sigan los mismos pasos. En la raíz del repositorio, la skill registrada reemplaza la `/verify` incluida. Esto requiere Claude Code v2.1.200 o posterior.

Claude edita el archivo registrado solo cuando dirigió una ejecución incorrectamente, como un comando que falló o un paso faltante, para que pueda confirmar el archivo sin diffs por sesión. Antes de v2.1.205, la skill incluida le indicaba a Claude que incorporara cualquier cosa que una ejecución aprendiera, lo que causaba conflictos de fusión frecuentes.

<h2 id="getting-started">
  Primeros pasos
</h2>

<h3 id="create-your-first-skill">
  Crear su primera skill
</h3>

Este ejemplo crea una skill que resume los cambios sin confirmar en su repositorio de git e identifica cualquier cosa arriesgada. Extrae el diff en vivo en el prompt antes de que Claude lo lea, por lo que la respuesta se basa en su árbol de trabajo real en lugar de lo que Claude puede adivinar a partir de archivos abiertos. Claude carga la skill automáticamente cuando pregunta sobre sus cambios, o puede invocarla directamente con `/summarize-changes`.

<Steps>
  <Step title="Create the skill directory">
    Cree un directorio para la skill en su carpeta de skills personales. Las skills personales están disponibles en todos sus proyectos.

    ```bash theme={null}
    mkdir -p ~/.claude/skills/summarize-changes
    ```
  </Step>

  <Step title="Write SKILL.md">
    Cada skill necesita un archivo `SKILL.md` con dos partes: frontmatter YAML entre marcadores `---` que le dice a Claude cuándo usar la skill, y contenido markdown con las instrucciones que Claude sigue cuando se ejecuta la skill. El nombre del directorio se convierte en el comando que escribe, y la `description` ayuda a Claude a decidir cuándo cargar la skill automáticamente.

    Guarde esto en `~/.claude/skills/summarize-changes/SKILL.md`:

    ```yaml theme={null}
    ---
    description: Summarizes uncommitted changes and flags anything risky. Use when the user asks what changed, wants a commit message, or asks to review their diff.
    ---

    ## Current changes

    !`git diff HEAD`

    ## Instructions

    Summarize the changes above in two or three bullet points, then list any risks you notice such as missing error handling, hardcoded values, or tests that need updating. If the diff is empty, say there are no uncommitted changes.
    ```

    La línea `` !`git diff HEAD` `` utiliza [inyección de contexto dinámico](#inject-dynamic-context): Claude Code ejecuta el comando y reemplaza la línea con su salida antes de que Claude vea el contenido de la skill, por lo que las instrucciones llegan con el diff actual ya insertado.
  </Step>

  <Step title="Test the skill">
    Abra un proyecto de git, realice una pequeña edición en cualquier archivo e inicie Claude Code ejecutando `claude`. Puede probar la skill de dos formas.

    **Deje que Claude la invoque automáticamente** preguntando algo que coincida con la descripción:

    ```text theme={null}
    What did I change?
    ```

    **O invóquela directamente** con el nombre de la skill:

    ```text theme={null}
    /summarize-changes
    ```

    De cualquier forma, Claude debe responder con un breve resumen de su edición y una lista de riesgos.
  </Step>
</Steps>

<h2 id="where-skills-live">
  Elige dónde cargan las skills
</h2>

Dónde guardes una skill decide qué sesiones la cargan. Guárdala en tu directorio de inicio para obtenerla en cada proyecto, confirma su cambio en un repositorio para compartirla con todos los que trabajan allí, o distribúyela a través de un plugin o configuración administrada para llegar a todo un equipo.

| Ubicación            | Ruta                                                                                                                              | Se carga en                                                                                                                                                                                                                           |
| :------------------- | :-------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Enterprise           | `.claude/skills/<skill-name>/SKILL.md` en el [directorio de configuración administrada](/docs/es/managed-settings#delivery-mechanisms) | Todos los usuarios en máquinas donde tu organización la implementa                                                                                                                                                                    |
| Personal             | `~/.claude/skills/<skill-name>/SKILL.md`                                                                                          | Todos tus proyectos en esta máquina, pero no en [sesiones de Cowork o en la nube](#skills-in-cowork-and-cloud-sessions)                                                                                                               |
| Proyecto             | `.claude/skills/<skill-name>/SKILL.md`                                                                                            | Sesiones en este repositorio. Confirma su cambio para que tu equipo también la obtenga                                                                                                                                                |
| Anidada              | `<subdir>/.claude/skills/<skill-name>/SKILL.md`                                                                                   | Sesiones iniciadas en o debajo de `<subdir>`. Una sesión iniciada por encima de ella carga la skill una vez que Claude trabaja en archivos allí. Consulta [monorepos y subdirectorios](#discovery-from-parent-and-nested-directories) |
| Directorio adicional | `.claude/skills/<skill-name>/SKILL.md` en un directorio que pasas con `--add-dir`                                                 | Esa sesión. Consulta [directorios fuera del proyecto](#skills-from-additional-directories)                                                                                                                                            |
| Plugin               | `<plugin>/skills/<skill-name>/SKILL.md`                                                                                           | Dondequiera que el [plugin](/docs/es/plugins/overview) esté habilitado, como `/plugin-name:skill-name`                                                                                                                                     |
| Cuenta de claude.ai  | Skills habilitadas para tu cuenta de claude.ai                                                                                    | Sesiones de Cowork, sesiones en la nube y sesiones de terminal donde inicias sesión con esa cuenta. Consulta [Skills sincronizadas desde claude.ai](#how-synced-skills-behave)                                                        |

Las carpetas de skills también siguen estas reglas:

* **Carpetas con enlaces simbólicos**: una entrada `<skill-name>` en la ubicación enterprise, personal o de proyecto puede ser un enlace simbólico a un directorio en otro lugar del disco. Claude Code lee `SKILL.md` del destino y carga la skill una sola vez incluso si varias ubicaciones apuntan al mismo destino. Las skills de plugins [manejan los enlaces simbólicos de manera diferente](/docs/es/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks).
* **Nombre reservado**: no nombres una carpeta de skill `synced`, en ninguna capitalización. Claude Code usa `~/.claude/skills/synced/` para [skills descargadas desde claude.ai](#where-synced-skills-load) y omite una skill que crees con ese nombre en las ubicaciones enterprise, personal y de proyecto.
* **Archivos de comando**: un archivo Markdown en `.claude/commands/` es el formato más antiguo y aún funciona. Admite el mismo [frontmatter](#frontmatter-reference) excepto `name` y `paths`. Para encontrar el nombre que escribes para invocarlo, consulta [Cómo una skill obtiene su nombre de comando](#how-a-skill-gets-its-command-name). Prefiere una skill para trabajo nuevo, ya que las skills también admiten [archivos de apoyo](#add-supporting-files).
* **Carpeta de skill como plugin**: agrega un `.claude-plugin/plugin.json` a una carpeta de skill y se carga como un [plugin](/docs/es/plugins/loading#plugins-shared-through-a-repository) llamado `<name>@skills-dir`, para que pueda agrupar agentes, hooks y servidores MCP. En un `.claude/skills/` de un proyecto, esto requiere aceptar primero el diálogo de confianza del espacio de trabajo.

<h3 id="discovery-from-parent-and-nested-directories">
  Carga skills en monorepos y subdirectorios
</h3>

Claude Code carga skills de proyecto desde `.claude/skills/` en el directorio donde lo inicias y en cada directorio padre hasta la raíz del repositorio, por lo que iniciar en `packages/frontend/` aún recoge skills definidas en la raíz. Cuando [mueves la sesión con `/cd`](/docs/es/permissions#move-the-session-to-another-directory) en v2.1.246 o posterior, Claude Code agrega las skills de proyecto del nuevo directorio.

En una sesión que se ejecuta en un [git worktree](/docs/es/worktrees) vinculado, Claude Code busca directorios padre solo hasta la raíz del worktree. En Claude Code v2.1.277 o posterior, cuando el checkout del worktree no tiene un directorio `.claude/skills` en su raíz, Claude Code carga las skills de proyecto del checkout principal en su lugar. Consulta [Qué comparten los worktrees con el checkout principal](/docs/es/worktrees#what-worktrees-share-with-the-main-checkout).

Las skills en un directorio `.claude/skills/` debajo de donde iniciaste no se cargan al inicio. Se cargan la primera vez que Claude lee o edita un archivo en ese subdirectorio y permanecen disponibles para el resto de la sesión. Hasta entonces no aparecen en el menú `/` y no puedes invocarlas por nombre. Para cargarlas antes, ejecuta `/add-dir` con la ruta del subdirectorio, lo que requiere Claude Code v2.1.257 o posterior.

Cuando una skill anidada comparte un nombre con otra skill, ambas permanecen disponibles. Con una skill `deploy` en la raíz del repositorio y otra en `apps/web/.claude/skills/`:

* `/deploy` ejecuta la skill raíz. Claude Code también enumera las variantes calificadas por directorio para Claude, con una instrucción para invocar la cuya carpeta contiene los archivos en los que está trabajando, por lo que la skill anidada aún se aplica al trabajo en `apps/web/`.
* `/apps/web:deploy` ejecuta la skill anidada por sí sola. Su descripción nombra el directorio al que se aplica.

<h3 id="skills-from-additional-directories">
  Carga skills desde un directorio fuera del proyecto
</h3>

Cuando agregas un directorio con `--add-dir` o `/add-dir`, Claude Code carga las skills en el `.claude/skills/` de ese directorio, junto con su `.claude/commands/` y `.claude/agents/`. Los directorios que el Agent SDK agrega a través de [`additionalDirectories`](/docs/es/agent-sdk/typescript#options) en TypeScript o [`add_dirs`](/docs/es/agent-sdk/python#claudeagentoptions) en Python se cargan de la misma manera, porque el SDK los pasa como `--add-dir`. La configuración `permissions.additionalDirectories` en `settings.json` otorga solo acceso a archivos y no carga ninguno de estos.

Claude Code observa `.claude/skills/` en un directorio que pasas con `--add-dir` al iniciar, como describe [Edita una skill durante una sesión](#live-change-detection). No observa el `.claude/commands/` o `.claude/agents/` del directorio agregado, por lo que reinicia la sesión después de cambiar un archivo allí.

Estas cargas dependen de la [fuente de configuración](/docs/es/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) `project`, que está activada de forma predeterminada. Una política [`strictPluginOnlyCustomization`](/docs/es/settings-reference#strictpluginonlycustomization), [modo bare](/docs/es/headless#start-faster-with-bare-mode) y [`--safe-mode`](/docs/es/cli-reference#cli-flags) cada una las restringe aún más, como describen esas páginas. Consulta [Los directorios adicionales otorgan acceso a archivos, no configuración](/docs/es/permissions#additional-directories-grant-file-access-not-configuration) para la tabla completa de lo que carga un directorio agregado, incluyendo `CLAUDE.md` y configuración de plugins.

<h3 id="resolve-skills-that-share-a-name">
  Resuelve skills que comparten un nombre
</h3>

Cuando dos skills comparten un nombre, de dónde vino cada una decide cuál ejecuta `/name`. La tabla cubre las ubicaciones enterprise, personal, proyecto, anidada, plugin y claude.ai, skills agrupadas y archivos de comando:

| Mismo nombre en                                                                                                 | Cuál se ejecuta                                                                                                                                                                                                               |
| :-------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Dos de enterprise, personal y proyecto                                                                          | Enterprise sobre personal, y personal sobre proyecto. Con `deploy` en ambos `~/.claude/skills/` y el `.claude/skills/` del proyecto, `/deploy` ejecuta la personal                                                            |
| Cualquiera de esas ubicaciones y una [skill agrupada](#bundled-skills)                                          | Tu skill reemplaza el comando agrupado, pero no sus alias. Una skill `code-review` de proyecto reemplaza `/code-review`, y el alias agrupado `/review` nunca ejecuta tu skill                                                 |
| Una skill y un archivo en `.claude/commands/`                                                                   | La skill                                                                                                                                                                                                                      |
| Una skill raíz de proyecto y una skill anidada                                                                  | Ambas se cargan. Consulta [monorepos y subdirectorios](#discovery-from-parent-and-nested-directories)                                                                                                                         |
| Una skill de plugin y una skill en cualquiera de las ubicaciones anteriores                                     | Ambas se cargan, porque las skills de plugin tienen espacios de nombres como `/plugin-name:skill-name`                                                                                                                        |
| Cualquiera de las anteriores y una skill [sincronizada desde tu cuenta de claude.ai](#how-synced-skills-behave) | La otra skill o comando. La skill sincronizada aún se ejecuta como `/anthropic-skills:<name>`. Consulta [Cuando un nombre de skill sincronizada coincide con otro comando](#when-a-synced-skill-name-matches-another-command) |

<h3 id="skills-in-cowork-and-cloud-sessions">
  Usa skills en sesiones de Cowork y en la nube
</h3>

Las sesiones de [Cowork](https://claude.com/product/cowork) y [sesiones en la nube](/docs/es/cloud-environments#what-carries-over-from-your-setup), incluyendo [rutinas](/docs/es/routines), no leen `~/.claude/skills/` en tu máquina. Tanto las sesiones de Cowork interactivas como las programadas cargan las skills habilitadas para tu cuenta de claude.ai, sincronizadas al inicio de la sesión; adminístralas desde **Customize** en la barra lateral de la aplicación de escritorio o desde la configuración de skills en claude.ai. Las sesiones en la nube cargan adicionalmente las skills de proyecto confirmadas en el `.claude/skills/` del repositorio clonado.

Si una skill existe solo en `~/.claude/skills/` en tu máquina, Claude Code reporta que la skill no fue encontrada cuando una [rutina](/docs/es/routines) la invoca, porque cada ejecución de rutina comienza como una sesión en la nube nueva. Para hacer que una skill personal esté disponible en estas sesiones:

* Para sesiones de Cowork y en la nube, habilita la skill para tu cuenta de claude.ai.
* Para sesiones en la nube, puedes en su lugar confirmar la skill en el `.claude/skills/` del repositorio. Los plugins declarados en el `.claude/settings.json` del repositorio [se instalan al inicio de la sesión](/docs/es/cloud-environments#what-carries-over-from-your-setup); los plugins habilitados solo en tu configuración de usuario no se transfieren.

Las [tareas programadas de escritorio](/docs/es/desktop-scheduled-tasks) se ejecutan localmente en tu máquina, por lo que cargan `~/.claude/skills/`.

<h3 id="how-synced-skills-behave">
  Skills sincronizadas desde claude.ai
</h3>

Esta sección se aplica a ti si usas sesiones de Cowork o en la nube, o inicias sesión en Claude Code en tu terminal con una cuenta de claude.ai. En esas sesiones, Claude Code carga las skills habilitadas para tu cuenta de claude.ai, sin ninguna configuración de tu parte, como describe [Dónde se cargan las skills sincronizadas](#where-synced-skills-load). Esas skills incluyen las que creas o actives en tu configuración de claude.ai, skills que tu organización proporciona allí, y las skills integradas de Anthropic como `pdf` y `xlsx`.

Claude Code descarga una skill sincronizada desde tu cuenta en lugar de leer un archivo que escribiste en la máquina donde se ejecuta la sesión, por lo que aplica reglas a las skills sincronizadas que no se aplican a las skills que almacenas en las [ubicaciones de skills](#where-skills-live).

<h4 id="where-synced-skills-load">
  Dónde se cargan las skills sincronizadas
</h4>

En una sesión de Cowork o en la nube, Claude Code carga las skills habilitadas para tu cuenta de claude.ai, y [Skills en sesiones de Cowork y en la nube](#skills-in-cowork-and-cloud-sessions) dice cómo elegir qué skills obtienen esas sesiones.

En tu terminal, Claude Code sincroniza esas skills en sesiones donde inicias sesión con tu cuenta de claude.ai. Cuando la sesión comienza, Claude Code descarga las skills de tu cuenta en `~/.claude/skills/synced/` en segundo plano, luego verifica claude.ai para cambios aproximadamente cada 10 minutos mientras se ejecuta la sesión. Cuando una verificación encuentra que una skill fue agregada, editada o desactivada en claude.ai, Claude Code la agrega, actualiza o elimina en la sesión en ejecución sin un reinicio. La sincronización en sesiones de terminal requiere Claude Code v2.1.273 o posterior.

La sincronización nunca retrasa el inicio, porque Claude espera la descarga de una skill solo cuando la invoca. Una ejecución corta [no interactiva](/docs/es/headless) puede por lo tanto terminar antes de que una skill recién agregada se descargue, en cuyo caso una sesión posterior la descarga. Para hacer que una ejecución no interactiva descargue tus skills y espere la lista antes de responder el prompt, establece [`CLAUDE_CODE_SYNC_SKILLS`](/docs/es/env-vars#variables) en `1`.

Claude Code sincroniza solo en una sesión que inicia sesión con tu cuenta de claude.ai y [obtiene banderas de características de Anthropic](/docs/es/env-vars#features-that-need-feature-flag-fetching). No sincroniza en estas sesiones:

* Una sesión que no usa un inicio de sesión almacenado por `/login`, como una que se autentica con una clave API, o una donde `ANTHROPIC_AUTH_TOKEN`, `CLAUDE_CODE_OAUTH_TOKEN`, o un script `apiKeyHelper` proporciona la credencial
* Una sesión que no obtiene banderas de características, como una en Amazon Bedrock o una donde estableces `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`
* Una sesión en [modo bare](/docs/es/headless#start-faster-with-bare-mode) o una que inicias con `--safe-mode`
* Una sesión donde la configuración administrada de tu organización [bloquea skills a fuentes de plugins](/docs/es/settings-reference#strictpluginonlycustomization-skills), o una que inicias con una lista [`--setting-sources`](/docs/es/cli-reference#cli-flags) que deja fuera `user`

Si inicias sesión con `/login` durante una sesión, reinicia Claude Code para comenzar a sincronizar.

Las skills que una sesión anterior sincronizó permanecen en el disco. Claude Code las carga en sesiones posteriores iniciadas en la misma cuenta, incluso cuando no puede alcanzar claude.ai.

Claude Code descarga skills sincronizadas y nunca las carga. Si tú o Claude editan un archivo bajo `~/.claude/skills/synced/`, el cambio no se guarda en tu cuenta de claude.ai, y una sincronización posterior puede sobrescribirlo o eliminarlo. Para cambiar una skill sincronizada, actualízala en claude.ai; la próxima sincronización descarga la nueva versión.

Para ver qué skills sincronizaron, ejecuta `/skills`. El menú las enumera bajo `claude.ai sync`.

Algunas de las skills de Anthropic, como `pdf` y `xlsx`, siempre se sincronizan. Para el resto, activa o desactiva una skill en tu configuración de skills en claude.ai para cambiar si se sincroniza.

Para dejar de sincronizar en una máquina, establece [`syncClaudeAiSkills`](/docs/es/settings-reference#syncclaudeaiskills) en `false` en tu configuración de usuario. Claude Code deja de descargar, y la próxima vez que inicie mueve las skills que ya sincronizó a `~/.claude/skills/.trash/` y ya no las carga. Tu organización puede desactivar la sincronización para todos desactivando Skills en claude.ai. Para dejar de sincronizar mientras mantienes Skills activado, puede establecer la misma clave en [configuración administrada](/docs/es/managed-settings).

Si tu organización desactiva Skills en claude.ai, Claude Code elimina las skills descargadas y dejan de cargarse. Las skills eliminadas se mueven a `~/.claude/skills/.trash/`, donde puedes recuperar los archivos hasta que el [barrido de retención](/docs/es/claude-directory#cleaned-up-automatically) los elimine. Una vez que tu organización activa Skills nuevamente, Claude Code descarga las skills que habilitaste en la próxima sincronización.

<h4 id="when-a-synced-skill-name-matches-another-command">
  Cuando un nombre de skill sincronizada coincide con otro comando
</h4>

Puedes invocar una skill sincronizada por su nombre completo, `/anthropic-skills:<name>`, o por su nombre corto, `/<name>`. Cuando otro comando usa ese nombre corto, `/<name>` ejecuta el otro comando, y la skill sincronizada se ejecuta solo como `/anthropic-skills:<name>`. Con una skill local `deploy` y una sincronizada `deploy`, `/deploy` ejecuta la skill local y `/anthropic-skills:deploy` ejecuta la sincronizada. Antes de v2.1.269, una skill sincronizada tenía solo su nombre corto.

El otro comando puede ser cualquiera de estos:

* Un comando integrado o una [skill agrupada](#bundled-skills), incluyendo una que no esté disponible en tu sesión, por ejemplo después de que desactives las skills agrupadas
* Una skill en cualquier [nivel local](#where-skills-live) o un archivo en `.claude/commands/`
* Una skill de plugin
* Un [prompt de MCP](/docs/es/mcp#use-mcp-prompts-as-commands)

Claude Code etiqueta las skills sincronizadas para que puedas saber de dónde vinieron. El menú `/skills` y `/context` agrupan las skills sincronizadas bajo `claude.ai sync`, y el menú de comando `/` las marca como provenientes de claude.ai.

Cuando compara nombres, Claude Code ignora mayúsculas, espacios y caracteres invisibles, y trata formas de compatibilidad como letras de ancho completo y variantes de guiones como sus equivalentes simples. Por ejemplo, una skill sincronizada llamada `Commit` y una skill local llamada `commit` cuentan como el mismo nombre, por lo que `/commit` sigue ejecutando tu skill local.

Un nombre que difiere solo por una letra que se parece a otra del alfabeto cuenta como un nombre diferente, y la etiqueta `claude.ai sync` es cómo distingues los dos. Estas comprobaciones y etiquetas requieren Claude Code v2.1.228 o posterior.

<h4 id="how-claude-code-handles-the-frontmatter-of-a-synced-skill">
  Cómo Claude Code maneja el frontmatter de una skill sincronizada
</h4>

Claude Code aplica dos reglas al frontmatter de una skill sincronizada:

* Claude Code honra el frontmatter en cada tipo de sesión, por lo que una concesión `allowed-tools` pasa por el [flujo de permisos](/docs/es/permissions) normal.
* Claude Code sanitiza el texto de visualización que proporciona la skill, como su descripción. Elimina caracteres de control, y en texto que llega a Claude, como la descripción, también escapa los corchetes angulares para que el texto no pueda imitar el formato interno de Claude Code. Esta sanitización requiere Claude Code v2.1.228 o posterior.

<h4 id="how-claude-code-handles-the-body-of-a-synced-skill">
  Cómo Claude Code maneja el cuerpo de una skill sincronizada
</h4>

Lo que Claude Code hace con el cuerpo de una skill sincronizada depende de dónde se ejecute la sesión:

* En una sesión en la nube, el cuerpo mantiene el comportamiento que tiene una skill local, porque la sesión se ejecuta en un contenedor aislado.
* En una sesión de Cowork en tu escritorio, el cuerpo mantiene el comportamiento que tiene una skill local, excepto que Claude Code reemplaza cada línea de comando `!` con el [marcador de posición `disableSkillShellExecution`](#inject-dynamic-context), como lo hace para cada skill que proporcionas allí.
* En cualquier otra sesión en tu máquina, Claude Code no ejecuta [comandos `!`](#inject-dynamic-context), no adjunta los archivos que las referencias `@` nombran de la manera que lo hace para una skill local, y no sustituye los marcadores de posición `${CLAUDE_PROJECT_DIR}` y `${CLAUDE_SESSION_ID}`, por lo que las referencias `@` y ambos marcadores de posición llegan a Claude como texto literal. Una línea de comando `!` también llega a Claude como texto literal, o como ese marcador de posición cuando `disableSkillShellExecution` está activado. Este manejo requiere Claude Code v2.1.228 o posterior.

<h3 id="live-change-detection">
  Edita una skill durante una sesión
</h3>

Claude Code observa directorios de skills para cambios de archivo, excepto en [modo bare](/docs/es/headless#start-faster-with-bare-mode). Cuando agregas, editas o eliminas una skill bajo `~/.claude/skills/`, el `.claude/skills/` del proyecto, o un `.claude/skills/` dentro de un directorio `--add-dir`, Claude Code recoge el cambio dentro de la sesión actual, sin un reinicio. Si creas un directorio de skills de nivel superior que no existía cuando se inició la sesión, reinicia Claude Code para que pueda observar el nuevo directorio.

La detección de cambios en vivo cubre solo el texto `SKILL.md`. Para una carpeta de skill que también es un [plugin](/docs/es/plugins/loading#plugins-shared-through-a-repository), los cambios en `hooks/`, `.mcp.json`, `agents/` y `output-styles/` necesitan `/reload-plugins` para tomar efecto.

<h3 id="remove-a-skill">
  Elimina una skill
</h3>

Cómo eliminas una skill depende de dónde vino:

* **Skill personal o de proyecto**: elimina el directorio de la skill, `~/.claude/skills/<skill-name>/` o `.claude/skills/<skill-name>/`. Claude Code [la elimina de `/skills` en la sesión actual](#live-change-detection); el contenido que Claude Code ya cargó de ella sigue el [ciclo de vida del contenido de la skill](#skill-content-lifecycle).
* **Skill enterprise**: un administrador elimina el directorio de la skill desde `.claude/skills/` dentro del [directorio de configuración administrada](/docs/es/managed-settings#delivery-mechanisms), por ejemplo `/etc/claude-code/.claude/skills/<skill-name>/` en Linux.
* **Skill de plugin**: deshabilita o desinstala el plugin que la proporciona, desde el menú `/plugin` o con `/plugin uninstall <plugin-name>@<marketplace-name>`. Claude Code descarga las skills del plugin cuando [el cambio se aplica](/docs/es/plugins/cli-reference#reload-plugins) o cuando reinicies.
* **Skill sincronizada desde claude.ai**: desactiva la skill para tu cuenta de claude.ai, en el mismo lugar donde la [habilitaste](#skills-in-cowork-and-cloud-sessions). Claude Code la elimina de `~/.claude/skills/synced/` la próxima vez que [sincroniza tus skills](#where-synced-skills-load). Si eliminas el directorio manualmente en su lugar, la próxima sincronización lo descarga nuevamente mientras la skill permanece habilitada en claude.ai.
* **Skill agrupada**: establece [`disableBundledSkills`](#bundled-skills) en `true` para desactivar las skills agrupadas, o establece una skill en `"off"` en [`skillOverrides`](#override-skill-visibility-from-settings) para ocultarla.

Para mantener una skill personal o de proyecto pero evitar que Claude la invoque por sí sola, establece [`disable-model-invocation: true`](#control-who-invokes-a-skill) en su frontmatter, o `"user-invocable-only"` en [`skillOverrides`](#override-skill-visibility-from-settings) cuando no desees editar el archivo.

<h2 id="configure-skills">
  Configurar skills
</h2>

Los skills se configuran a través de frontmatter YAML en la parte superior de `SKILL.md` y el contenido markdown que sigue.

<h3 id="types-of-skill-content">
  Tipos de contenido de skill
</h3>

Los archivos de skill pueden contener cualquier instrucción, pero pensar en cómo deseas invocarlos ayuda a guiar qué incluir:

**Contenido de referencia** añade conocimiento que Claude aplica a tu trabajo actual. Convenciones, patrones, guías de estilo, conocimiento del dominio. Este contenido se ejecuta en línea para que Claude pueda usarlo junto con el contexto de tu conversación.

```yaml theme={null}
---
name: api-conventions
description: API design patterns for this codebase
---

When writing API endpoints:
- Use RESTful naming conventions
- Return consistent error formats
- Include request validation
```

**Contenido de tarea** proporciona a Claude instrucciones paso a paso para una acción específica, como implementaciones, commits o generación de código. Estas son a menudo acciones que deseas invocar directamente con `/skill-name` en lugar de dejar que Claude decida cuándo ejecutarlas. Añade `disable-model-invocation: true` para evitar que Claude la active automáticamente. El ejemplo a continuación añade `context: fork`, que ejecuta el skill en su propio contexto de subagente; consulta [Ejecutar skills en un subagente](#run-skills-in-a-subagent).

```yaml theme={null}
---
name: deploy
description: Deploy the application to production
context: fork
disable-model-invocation: true
---

Deploy the application:
1. Run the test suite
2. Build the application
3. Push to the deployment target
```

Mantén el cuerpo en sí conciso. Una vez que se carga un skill, su contenido [permanece en contexto entre turnos](#skill-content-lifecycle), por lo que cada línea es un costo de token recurrente. Indica qué hacer en lugar de narrar cómo o por qué, y aplica la misma prueba de concisión que harías para [contenido de CLAUDE.md](/docs/es/best-practices#write-an-effective-claude-md).

<h3 id="frontmatter-reference">
  Referencia de frontmatter
</h3>

Configura un skill con YAML [frontmatter](/docs/es/glossary#frontmatter) entre marcadores `---` en la parte superior de `SKILL.md`, y escribe las instrucciones del skill como Markdown después del cierre `---`. Los nombres de campo usan palabras en minúsculas separadas por guiones, excepto `when_to_use`. Un [archivo de comando](#where-skills-live) en `.claude/commands/` acepta los mismos campos excepto `name` y `paths`. Este ejemplo establece cuatro campos:

```yaml theme={null}
---
name: my-skill
description: What this skill does
disable-model-invocation: true
allowed-tools: Read Grep
---

Your skill instructions here...
```

Todos los campos son opcionales. Solo se recomienda `description` para que Claude sepa cuándo usar el skill. Un nombre de campo debe coincidir exactamente con la tabla, guiones incluidos: Claude Code ignora un campo que no reconoce sin reportar un error.

Claude Code lee el frontmatter solo cuando la apertura `---` es la primera línea del archivo. De lo contrario, trata todo el archivo, incluidos los marcadores `---`, como contenido de skill. Si el YAML entre los marcadores no se analiza, el skill aún se carga sin campos establecidos; consulta [Skill no se activa](#skill-not-triggering) para encontrar y corregir el error.

Los campos booleanos aceptan `yes`, `no`, `on`, `off`, `1` y `0` en cualquier caso de letra, además de `true` y `false`. Antes de v2.1.218, Claude Code reconocía solo `true` y `false`.

| Campo                      | Requerido   | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| :------------------------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`                     | No          | Nombre para mostrar en listados de skills. Por defecto es el nombre del directorio. Consulta [Cómo un skill obtiene su nombre de comando](#how-a-skill-gets-its-command-name) para ver cómo el campo interactúa con el nombre que escribes para invocar el skill.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `description`              | Recomendado | Qué hace el skill y cuándo usarlo. Claude usa esto para decidir cuándo aplicar el skill. Si se omite, usa la primera línea no vacía del contenido markdown. Pon el caso de uso clave primero: el texto combinado de `description` y `when_to_use` se trunca en 1.536 caracteres en el listado de skills para reducir el uso de contexto.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `when_to_use`              | No          | Contexto adicional para cuándo Claude debe invocar el skill, como frases desencadenantes o solicitudes de ejemplo. Se añade a `description` en el listado de skills y cuenta hacia el límite de 1.536 caracteres.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `argument-hint`            | No          | Sugerencia mostrada durante el autocompletado para indicar argumentos esperados. Ejemplo: `[issue-number]` o `[filename] [format]`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `arguments`                | No          | Argumentos posicionales nombrados para [`$name` substitution](#available-string-substitutions) en el contenido del skill. Acepta una cadena separada por espacios o una lista YAML. Los nombres se asignan a posiciones de argumentos en orden.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `disable-model-invocation` | No          | Establece en `true` para evitar que Claude cargue automáticamente este skill. Úsalo para flujos de trabajo que deseas activar manualmente con `/name`. También evita que el skill sea [precargado en subagentes](/docs/es/sub-agents#preload-skills-into-subagents). A partir de v2.1.196, también evita que el skill se ejecute cuando una [tarea programada](/docs/es/scheduled-tasks) se activa con el skill como su prompt. Por defecto: `false`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `user-invocable`           | No          | Establece en `false` cuando solo Claude debe invocar el skill: Claude Code lo oculta del menú `/` y no lo ejecuta cuando escribes `/name`. Úsalo para conocimiento de fondo que los usuarios no deben invocar directamente. Por defecto: `true`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `allowed-tools`            | No          | Herramientas que Claude puede usar sin pedir permiso durante el turno que invoca este skill. La concesión se borra cuando envías tu siguiente mensaje. Acepta una cadena separada por espacios o comas, o una lista YAML. Consulta [Pre-aprobar herramientas para un skill](#pre-approve-tools-for-a-skill).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `disallowed-tools`         | No          | Herramientas eliminadas del grupo disponible de Claude mientras este skill está activo. Úsalo para skills autónomos que nunca deben llamar a ciertas herramientas, como `AskUserQuestion` para un bucle de fondo. Acepta una cadena separada por espacios o comas, o una lista YAML. La restricción se borra cuando envías tu siguiente mensaje. Como las reglas de denegación, el campo no puede eliminar [`EndConversation`](/docs/es/tools-reference#endconversation-tool-behavior) mientras cualquier otra herramienta permanezca.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `model`                    | No          | Modelo a usar cuando este skill está activo. La anulación se aplica por el resto del turno actual y no se guarda en la configuración; el modelo de sesión se reanuda en tu siguiente prompt. Acepta los mismos valores que [`/model`](/docs/es/model-config), o `inherit` para mantener el modelo activo. Un valor excluido por la lista de permitidos [`availableModels`](/docs/es/model-config#restrict-model-selection) de tu organización no se usa y la sesión mantiene su modelo actual. En [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode), y en [modo plan mientras el clasificador revisa comandos](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode), un modelo que el modo automático no admite tampoco se usa y la sesión mantiene su modelo actual. Con `context: fork`, el valor establece el [modelo del subagente bifurcado](#run-skills-in-a-subagent) en su lugar, y un valor excluido sigue las [mismas reglas que una anulación de modelo de subagente](/docs/es/model-config#restrict-model-selection). |
| `effort`                   | No          | [Nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) cuando este skill está activo. Anula el nivel de esfuerzo de la sesión. Por defecto: hereda de la sesión. Opciones: `low`, `medium`, `high`, `xhigh`, `max`; los niveles disponibles dependen del modelo.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `context`                  | No          | Establece en `fork` para ejecutar en un contexto de subagente bifurcado. Consulta [Ejecutar skills en un subagente](#run-skills-in-a-subagent).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `agent`                    | No          | Qué tipo de subagente usar cuando `context: fork` está establecido.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `background`               | No          | Solo se aplica con `context: fork`. Establece en `false` para esperar el resultado del subagente bifurcado en el turno que invocó el skill, en lugar de [ejecutarlo en segundo plano](#run-skills-in-a-subagent). Por defecto: `true`. Requiere Claude Code v2.1.218 o posterior.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `hooks`                    | No          | Hooks que Claude Code registra cuando se invoca el skill y mantiene ejecutándose por el resto de la sesión. Consulta [Hooks en skills y agentes](/docs/es/hooks#hooks-in-skills-and-agents) para el formato de configuración y la opción `once`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `paths`                    | No          | Patrones Glob que limitan cuándo se activa este skill. Acepta una cadena separada por comas o una lista YAML. Cuando se establece, Claude carga el skill automáticamente solo cuando trabaja con archivos que coinciden con los patrones. Usa el mismo formato que [reglas específicas de ruta](/docs/es/memory#path-specific-rules).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `shell`                    | No          | Shell a usar para `` !`command` `` y bloques ` ```! ` en este skill. Acepta `bash` (por defecto) o `powershell`. Establecer `powershell` ejecuta comandos shell en línea a través de PowerShell cuando la [herramienta PowerShell](/es/tools-reference#powershell-tool) está habilitada: está activada por defecto en Windows sin Git Bash, activada por defecto con Git Bash para cuentas de claude.ai y Console, y necesita `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` en sesiones de Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry y en macOS, Linux y WSL. Establécelo en `0` para desactivar la herramienta.                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `metadata`                 | No          | Mapa YAML de forma libre para tus propios datos clave-valor, como campos de derechos o catálogo, leídos por tu propia herramienta desde `SKILL.md`. Claude Code no actúa sobre su contenido y descarta un valor que no sea un mapa. No reutilices nombres de campos de frontmatter como `paths` como claves.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `license`                  | No          | Licencia que cubre el skill. Parte de la especificación [Agent Skills](https://agentskills.io); consulta [Usar frontmatter de skill fuera de Claude Code](#using-skill-frontmatter-outside-claude-code). Claude Code acepta el campo pero no actúa sobre él.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `compatibility`            | No          | Requisitos de entorno para el skill, como productos previstos o requisitos previos del sistema, según se define en la especificación [Agent Skills](https://agentskills.io); consulta [Usar frontmatter de skill fuera de Claude Code](#using-skill-frontmatter-outside-claude-code). Acepta una cadena de hasta 500 caracteres. Claude Code acepta el campo pero no actúa sobre él.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |

<h4 id="using-skill-frontmatter-outside-claude-code">
  Usar frontmatter de skill fuera de Claude Code
</h4>

Claude Code acepta cada campo en la tabla anterior. Fuera de Claude Code, solo puedes usar los campos en la especificación [Agent Skills](https://agentskills.io):

| Ruta de distribución                                                                                                                                  | Campos de frontmatter que puedes usar                                          |
| :---------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------- |
| Skills de Claude Code en [cualquier nivel](#where-skills-live), incluidos skills de [plugin](/docs/es/plugins/overview)                                    | Cada campo en la tabla anterior                                                |
| Cargas de skills de claude.ai, la API de Skills y empaquetamiento con `package_skill.py` de [anthropics/skills](https://github.com/anthropics/skills) | `name`, `description`, `license`, `compatibility`, `metadata`, `allowed-tools` |

Cuando habilitas un skill personal para tu cuenta de claude.ai, por ejemplo para usarlo en [sesiones de Cowork y nube](#skills-in-cowork-and-cloud-sessions) e incluidas rutinas, lo cargas en claude.ai, por lo que se aplican las mismas reglas.

Si incluyes algún campo que la especificación no permite, el empaquetamiento o la carga falla con un error grave en lugar de ignorar el campo:

```
Unexpected key(s) in SKILL.md frontmatter: argument-hint. Allowed properties are: allowed-tools, compatibility, description, license, metadata, name
```

Restringir el frontmatter a los seis campos de la especificación evita el error de clave inesperada anterior. La [especificación de Agent Skills](https://agentskills.io) y los [requisitos de la API de Skills](https://docs.claude.com/en/api/skills-guide) definen todo lo demás que esas rutas validan. Las características del cuerpo solo de Claude Code, como [inyección de contexto dinámico](#inject-dynamic-context), no funcionan en el chat de claude.ai o a través de la API. Claude Code acepta los seis campos, por lo que el frontmatter que sigue la especificación se carga en Claude Code sin cambios.

<h4 id="how-a-skill-gets-its-command-name">
  Cómo un skill obtiene su nombre de comando
</h4>

El comando que escribes para invocar un skill proviene de dónde vive el archivo de skill y, para skills de plugin, también del campo de frontmatter `name`. En un skill personal o de proyecto, `name` establece solo la etiqueta de visualización mostrada en listados de skills, y el comando aún proviene del nombre del directorio. En un skill de plugin, `name` establece el último segmento del comando y el prefijo del plugin permanece en su lugar.

La tabla a continuación muestra de dónde proviene el nombre del comando para cada diseño:

| Ubicación del skill                                                                               | Fuente del nombre del comando                                                                                             | Ejemplo                                                                                                                                       |
| :------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------- |
| Directorio de skill bajo `~/.claude/skills/` o `.claude/skills/`                                  | Nombre del directorio                                                                                                     | `.claude/skills/deploy-staging/SKILL.md` → `/deploy-staging`                                                                                  |
| Directorio [anidado](#where-skills-live) `.claude/skills/`, cuando el nombre choca con otro skill | Ruta del subdirectorio relativa al directorio de trabajo, luego el nombre del directorio de skill                         | `apps/web/.claude/skills/deploy/SKILL.md` → `/apps/web:deploy`                                                                                |
| Archivo bajo `.claude/commands/`                                                                  | Nombre del archivo sin extensión                                                                                          | `.claude/commands/deploy.md` → `/deploy`                                                                                                      |
| Archivo en un subdirectorio de `.claude/commands/`                                                | Ruta del subdirectorio relativa a `commands/` con cada `/` reemplazado por `:`, luego el nombre del archivo sin extensión | `.claude/commands/frontend/component.md` → `/frontend:component`                                                                              |
| Subdirectorio `skills/` del plugin                                                                | Frontmatter `name` o el nombre del directorio, con espacio de nombres por plugin                                          | `my-plugin/skills/review/SKILL.md` → `/my-plugin:review`, o `/my-plugin:fancy` con `name: fancy`                                              |
| `SKILL.md` raíz del plugin                                                                        | Frontmatter `name`, con el nombre del directorio del plugin como alternativa                                              | `my-plugin/SKILL.md` con `name: review` → `/my-plugin:review`. Consulta [un skill único en la raíz del plugin](/docs/es/plugins/components#skills) |
| Skill [sincronizado desde claude.ai](#how-synced-skills-behave)                                   | El nombre del skill en tu cuenta de claude.ai, con prefijo `anthropic-skills:`                                            | Skill de cuenta `deploy` → `/anthropic-skills:deploy`, o `/deploy` mientras ningún otro comando use ese nombre                                |

En un skill de plugin, el frontmatter `name` reemplaza el nombre del directorio en el último segmento del comando, por lo que `my-plugin/skills/review/SKILL.md` con `name: fancy` se convierte en `/my-plugin:fancy`. El comando desnudo `/fancy` también invoca el skill a menos que otro comando ya use ese nombre. Si el `name` que escribes ya comienza con el prefijo propio del plugin, Claude Code no añade el prefijo nuevamente en v2.1.246 o posterior. Por ejemplo, `name: my-plugin:fancy` sigue siendo `/my-plugin:fancy`. De v2.1.216 a v2.1.245, Claude Code duplicaba el prefijo cuando el `name` ya lo llevaba.

En [sesiones no interactivas](/docs/es/headless), los nombres `help` y `feedback` no están reservados para sus comandos integrados solo de terminal, por lo que un skill de plugin con uno de esos nombres mantiene su comando desnudo allí. Todos los demás nombres de integrados solo de terminal, como `/login`, permanecen reservados aunque el comando no pueda ejecutarse en esas sesiones.

Para un `SKILL.md` raíz de plugin, no hay directorio de skill del que tomar el nombre, por lo que `name` proporciona el segmento final completo. Sin un campo `name`, Claude Code recurre al nombre del directorio del plugin.

<h4 id="available-string-substitutions">
  Sustituciones de cadena disponibles
</h4>

Los skills admiten sustitución de cadena para valores dinámicos en el contenido del skill:

| Variable                | Descripción                                                                                                                                                                                                                                                                                                                                   |
| :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `$ARGUMENTS`            | Todos los argumentos pasados al invocar el skill. Cuando ningún marcador de posición recibe un argumento, Claude Code los añade como `ARGUMENTS: <value>`. Consulta [Pasar argumentos a skills](#pass-arguments-to-skills).                                                                                                                   |
| `$ARGUMENTS[N]`         | Accede a un argumento específico por índice basado en 0, como `$ARGUMENTS[0]` para el primer argumento.                                                                                                                                                                                                                                       |
| `$N`                    | Abreviatura para `$ARGUMENTS[N]`, como `$0` para el primer argumento o `$1` para el segundo.                                                                                                                                                                                                                                                  |
| `$name`                 | Argumento nombrado declarado en la lista de frontmatter [`arguments`](#frontmatter-reference). Los nombres se asignan a posiciones en orden, por lo que con `arguments: [issue, branch]` el marcador de posición `$issue` se expande al primer argumento y `$branch` al segundo.                                                              |
| `${CLAUDE_SESSION_ID}`  | El ID de sesión actual. Útil para registro, creación de archivos específicos de sesión o correlación de salida de skill con sesiones.                                                                                                                                                                                                         |
| `${CLAUDE_EFFORT}`      | El nivel de esfuerzo actual: `low`, `medium`, `high`, `xhigh` o `max`. Ultracode no es un nivel distinto e informa como `xhigh`. Úsalo para adaptar instrucciones de skill a la configuración de esfuerzo activa.                                                                                                                             |
| `${CLAUDE_SKILL_DIR}`   | El directorio que contiene el archivo `SKILL.md` del skill. Para skills de plugin, este es el subdirectorio del skill dentro del plugin, no la raíz del plugin. Úsalo en comandos de inyección bash para hacer referencia a scripts o archivos incluidos con el skill, independientemente del directorio de trabajo actual.                   |
| `${CLAUDE_PROJECT_DIR}` | El directorio raíz del proyecto. Esta es la misma ruta que [hooks](/docs/es/hooks#reference-scripts-by-path) y servidores MCP reciben como `CLAUDE_PROJECT_DIR`. Úsalo para hacer referencia a scripts o archivos locales del proyecto, como `${CLAUDE_PROJECT_DIR}/.claude/hooks/helper.sh`, independientemente de dónde esté instalado el skill. |
| `${CLAUDE_PLUGIN_ROOT}` | El directorio de instalación del plugin. Se sustituye solo en skills de plugin. Úsalo para hacer referencia a scripts o archivos incluidos en cualquier lugar del plugin, incluidos recursos compartidos entre los skills del plugin. Consulta [variables de entorno del plugin](/docs/es/plugins/manifest-reference#environment-variables).       |
| `${CLAUDE_PLUGIN_DATA}` | El [directorio de datos persistentes](/docs/es/plugins/components#path-variables-and-persistent-data) del plugin, que sobrevive a las actualizaciones del plugin. Se sustituye solo en skills de plugin. Úsalo para hacer referencia a dependencias instaladas, archivos generados o cachés que deben sobrevivir a una actualización.              |

Claude Code sustituye `${CLAUDE_SKILL_DIR}` y `${CLAUDE_PROJECT_DIR}` en dos lugares: el contenido markdown del skill y las reglas Bash en el frontmatter [`allowed-tools`](#frontmatter-reference). En un skill de plugin, Claude Code sustituye `${CLAUDE_PLUGIN_ROOT}` y `${CLAUDE_PLUGIN_DATA}` en los mismos dos lugares. Usar la misma variable en ambos lugares permite que un skill ejecute un script incluido sin un aviso de permiso. El siguiente skill muestra el patrón:

```yaml theme={null}
---
name: render-chart
description: Render a chart from a CSV file
allowed-tools: Bash(${CLAUDE_SKILL_DIR}/scripts/render.sh *)
---

Run `${CLAUDE_SKILL_DIR}/scripts/render.sh <csv-file>` to render the chart.
```

Si este skill está instalado en `~/.claude/skills/render-chart/`, ambas ocurrencias de `${CLAUDE_SKILL_DIR}` se expanden a ese directorio. La regla `allowed-tools` luego coincide con el comando exacto que el cuerpo del skill le dice a Claude que ejecute, por lo que el script se ejecuta sin solicitar.

La sustitución `${CLAUDE_PROJECT_DIR}` requiere Claude Code v2.1.196 o posterior.

Los argumentos indexados usan comillas de estilo shell, por lo que envuelve valores de varias palabras entre comillas para pasarlos como un único argumento. Por ejemplo, `/my-skill "hello world" second` hace que `$0` se expanda a `hello world` y `$1` a `second`. El marcador de posición `$ARGUMENTS` siempre se expande a la cadena de argumento completa tal como se escribió.

Un marcador de posición indexado sin argumento correspondiente, como `$2` cuando solo se pasó un argumento, permanece en el contenido sin cambios. Un marcador de posición nombrado del frontmatter [`arguments`](#frontmatter-reference) sin argumento coincidente se expande a una cadena vacía.

Si pasas un valor de argumento que en sí contiene texto como `$1` o `$ARGUMENTS`, Claude Code lo inserta como texto literal y no lo expande. Por ejemplo, si el cuerpo de un skill contiene `Summarize $0` y ejecutas `/summarize "$ARGUMENTS from yesterday"`, Claude recibe `Summarize $ARGUMENTS from yesterday`. Claude Code aún reemplaza variables `${CLAUDE_*}` como `${CLAUDE_SKILL_DIR}` después de insertar los argumentos.

Para incluir un `$` literal antes de un dígito, `ARGUMENTS` o un nombre de argumento declarado, como `$1.00` en prosa, escápalo con una barra invertida: `\$1.00`. Una barra invertida antes de cualquier otro `$` se deja sin cambios. Solo una barra invertida directamente antes del token lo escapa. Una barra invertida duplicada como `\\$1` deja ambas barras invertidas en su lugar, y `$1` aún se expande al valor del argumento. El escape de barra invertida cubre solo estos marcadores de posición de argumento. Una barra invertida no evita la sustitución de una variable `${CLAUDE_*}` donde se aplica la variable.

**Ejemplo usando sustituciones:**

```yaml theme={null}
---
name: session-logger
description: Log activity for this session
---

Log the following to logs/${CLAUDE_SESSION_ID}.log:

$ARGUMENTS
```

<h3 id="add-supporting-files">
  Añadir archivos de apoyo
</h3>

Los skills pueden incluir múltiples archivos en su directorio. Esto mantiene `SKILL.md` enfocado en lo esencial mientras permite que Claude acceda a material de referencia detallado solo cuando sea necesario. Documentos de referencia grandes, especificaciones de API o colecciones de ejemplos no necesitan cargarse en contexto cada vez que se ejecuta el skill.

```text theme={null}
my-skill/
├── SKILL.md (required - overview and navigation)
├── reference.md (detailed API docs - loaded when needed)
├── examples.md (usage examples - loaded when needed)
└── scripts/
    └── helper.py (utility script - executed, not loaded)
```

Haz referencia a archivos de apoyo desde `SKILL.md` para que Claude sepa qué contiene cada archivo y cuándo cargarlo:

```markdown theme={null}
## Additional resources

- For complete API details, see [reference.md](reference.md)
- For usage examples, see [examples.md](examples.md)
```

<Tip>Mantén `SKILL.md` bajo 500 líneas. Mueve material de referencia detallado a archivos separados.</Tip>

<h3 id="control-who-invokes-a-skill">
  Controlar quién invoca un skill
</h3>

Por defecto, tanto tú como Claude pueden invocar cualquier skill. Puedes escribir `/skill-name` para invocarlo directamente, y Claude puede cargarlo automáticamente cuando sea relevante para tu conversación. Dos campos de frontmatter te permiten restringir esto:

* **`disable-model-invocation: true`**: Solo tú puedes invocar el skill. Úsalo para flujos de trabajo con efectos secundarios o que deseas controlar el tiempo, como `/commit`, `/deploy` o `/send-slack-message`. No quieres que Claude decida implementar porque tu código se ve listo.

* **`user-invocable: false`**: Solo Claude puede invocar el skill. Úsalo para conocimiento de fondo que no es accionable como comando. Un skill `legacy-system-context` explica cómo funciona un sistema antiguo. Claude debe saber esto cuando sea relevante, pero `/legacy-system-context` no es una acción significativa para que los usuarios realicen.

Este ejemplo crea un skill de implementación que solo tú puedes activar. Si estableces `disable-model-invocation: true`, Claude no puede ejecutar el skill automáticamente:

```yaml theme={null}
---
name: deploy
description: Deploy the application to production
disable-model-invocation: true
---

Deploy $ARGUMENTS to production:

1. Run the test suite
2. Build the application
3. Push to the deployment target
4. Verify the deployment succeeded
```

Si Claude lo intenta de todas formas, Claude Code bloquea la llamada e instruye a Claude a no reproducir los pasos de implementación de otra manera, así que espera que Claude sugiera ejecutar `/deploy` tú mismo.

Aquí está cómo los dos campos afectan la invocación y la carga de contexto:

| Frontmatter                      | Puedes invocar | Claude puede invocar | Cuándo se carga en contexto                                               |
| :------------------------------- | :------------- | :------------------- | :------------------------------------------------------------------------ |
| (por defecto)                    | Sí             | Sí                   | Descripción siempre en contexto, skill completo se carga cuando se invoca |
| `disable-model-invocation: true` | Sí             | No                   | Descripción no en contexto, skill completo se carga cuando lo invocas     |
| `user-invocable: false`          | No             | Sí                   | Descripción siempre en contexto, skill completo se carga cuando se invoca |

<Note>
  En una sesión regular, las descripciones de skills se cargan en contexto para que Claude sepa qué está disponible, pero el contenido completo del skill solo se carga cuando se invoca. [Subagentes con skills precargados](/docs/es/sub-agents#preload-skills-into-subagents) funcionan diferente: el contenido completo del skill se inyecta al inicio.
</Note>

<h3 id="skill-content-lifecycle">
  Ciclo de vida del contenido del skill
</h3>

Cuando tú o Claude invocas un skill, el contenido `SKILL.md` renderizado entra en la conversación como un único mensaje y permanece allí en turnos posteriores. Esta persistencia se aplica a las instrucciones del skill, no a sus permisos: una concesión [`allowed-tools`](#pre-approve-tools-for-a-skill) se borra cuando envías tu siguiente mensaje. Claude Code no vuelve a leer el archivo de skill en turnos posteriores, por lo que escribe orientación que debe aplicarse durante una tarea como instrucciones permanentes en lugar de pasos únicos.

Cuando Claude reinvoca un skill cuyo contenido renderizado es idéntico a la copia ya en contexto, Claude Code añade una nota breve de que el skill ya está cargado en lugar de una segunda copia del contenido. Cuando el contenido renderizado difiere, porque los argumentos cambiaron o un comando [contexto dinámico](#inject-dynamic-context) produjo nueva salida, Claude Code añade el contenido completo nuevamente.

[Auto-compactación](/docs/es/how-claude-code-works#when-context-fills-up) lleva skills invocados hacia adelante dentro de un presupuesto de tokens. Cuando la conversación se resume para liberar contexto, Claude Code vuelve a adjuntar la invocación más reciente de cada skill después del resumen, manteniendo los primeros 5.000 tokens de cada uno. Los skills reajuntados comparten un presupuesto combinado de 25.000 tokens. Claude Code llena este presupuesto comenzando desde el skill invocado más recientemente, por lo que los skills más antiguos pueden ser eliminados completamente después de la compactación si has invocado muchos en una sesión.

Si un skill parece dejar de influir en el comportamiento después de la primera respuesta, el contenido generalmente aún está presente y el modelo está eligiendo otras herramientas o enfoques. Fortalece la `description` del skill e instrucciones para que el modelo siga prefiriéndolo, o usa [hooks](/docs/es/hooks) para aplicar comportamiento de manera determinista. Si el skill es grande o invocaste varios otros después de él, reinvócalo después de la compactación para restaurar el contenido completo.

<h3 id="pre-approve-tools-for-a-skill">
  Pre-aprobar herramientas para un skill
</h3>

El campo `allowed-tools` otorga permiso para las herramientas listadas durante el turno que invoca el skill, para que Claude pueda usarlas sin solicitarte aprobación. La concesión se borra cuando envías tu siguiente mensaje, aunque el contenido del skill [permanece en contexto](#skill-content-lifecycle); invocar el skill nuevamente vuelve a aplicarlo para ese turno. No restringe qué herramientas están disponibles: cada herramienta sigue siendo invocable, y tu [configuración de permisos](/docs/es/permissions) aún rige herramientas que no están listadas. Para pre-aprobar herramientas para toda la sesión en lugar de un único turno, añade reglas de permitir a esa configuración de permisos en su lugar.

La confianza del espacio de trabajo no controla este campo. Claude Code aplica el `allowed-tools` de un skill de proyecto siempre que tú o Claude invoquen el skill, incluido en una ejecución `-p` en una carpeta que nunca has confiado. Un skill puede otorgarse a sí mismo acceso amplio a herramientas, por lo que revisa el `allowed-tools` de skills verificados en un repositorio antes de ejecutar Claude Code allí.

Este skill permite que Claude ejecute comandos git sin aprobación por uso siempre que lo invokes:

```yaml theme={null}
---
name: commit
description: Stage and commit the current changes
disable-model-invocation: true
allowed-tools: Bash(git add *) Bash(git commit *) Bash(git status *)
---
```

Para eliminar herramientas del grupo disponible de Claude mientras un skill está activo, listalás en `disallowed-tools` en el frontmatter del skill. La restricción se borra cuando envías tu siguiente mensaje. Como las reglas de denegación, el campo no puede eliminar [`EndConversation`](/docs/es/tools-reference#endconversation-tool-behavior) mientras cualquier otra herramienta permanezca. Para bloquear herramientas en todos los skills y prompts, añade reglas de denegación en tu [configuración de permisos](/docs/es/permissions).

<h3 id="pass-arguments-to-skills">
  Pasar argumentos a skills
</h3>

Tanto tú como Claude pueden pasar argumentos al invocar un skill. Los argumentos están disponibles a través del marcador de posición `$ARGUMENTS`.

Este skill corrige un problema de GitHub por número. El marcador de posición `$ARGUMENTS` se reemplaza con lo que sigue al nombre del skill:

```yaml theme={null}
---
name: fix-issue
description: Fix a GitHub issue
disable-model-invocation: true
---

Fix GitHub issue $ARGUMENTS following our coding standards.

1. Read the issue description
2. Understand the requirements
3. Implement the fix
4. Write tests
5. Create a commit
```

Cuando ejecutas `/fix-issue 123`, Claude recibe "Fix GitHub issue 123 following our coding standards..."

Si invocas un skill con argumentos pero ningún marcador de posición en el contenido del skill recibe uno, Claude Code añade `ARGUMENTS: <your input>` al final del contenido del skill para que Claude aún vea lo que escribiste. Un marcador de posición es `$ARGUMENTS`, una forma indexada como `$1`, o un argumento nombrado. Un marcador de posición indexado sin argumento en su posición permanece como texto literal y no cuenta como recibiendo uno. Un marcador de posición nombrado cuenta incluso cuando su posición no tiene argumento, porque se expande a una cadena vacía.

También puedes apilar varios skills al inicio de un mensaje. Escribir `/write-tests /fix-issue 123` carga ambos skills y pasa el texto final `123` como `$ARGUMENTS` a cada uno de ellos. Antes de v2.1.199, solo el primer skill se cargaba y recibía `/fix-issue 123` como texto de argumento literal.

Claude Code expande el primer skill más hasta cinco más apilados después de él. La expansión se detiene en el primer token que no es un skill invocable por el usuario en línea, por lo que un skill que se ejecuta como un [subagente bifurcado](#run-skills-in-a-subagent), como [`/code-review`](/docs/es/code-review#review-a-diff-locally), o uno cuyos argumentos pueden en sí mismos comenzar con un comando slash, como `/loop`, también termina la ejecución allí. Ese token y todo lo que viene después se convierte en el texto de argumento para cada skill expandido. `/code-review` se ejecuta como un subagente bifurcado desde v2.1.218; en versiones anteriores se ejecutaba en línea y se apilaba.

Para acceder a argumentos individuales por posición, usa `$ARGUMENTS[N]` o la forma más corta `$N`:

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one language to another
---

Migrate the $ARGUMENTS[0] component from $ARGUMENTS[1] to $ARGUMENTS[2].
Preserve all existing behavior and tests.
```

Ejecutar `/migrate-component SearchBar JavaScript TypeScript` reemplaza `$ARGUMENTS[0]` con `SearchBar`, `$ARGUMENTS[1]` con `JavaScript` y `$ARGUMENTS[2]` con `TypeScript`. El mismo skill usando la abreviatura `$N`:

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one language to another
---

Migrate the $0 component from $1 to $2.
Preserve all existing behavior and tests.
```

<h2 id="advanced-patterns">
  Patrones avanzados
</h2>

<h3 id="inject-dynamic-context">
  Inyectar contexto dinámico
</h3>

La sintaxis `` !`<command>` `` ejecuta comandos de shell antes de que el contenido de la skill se envíe a Claude. La salida del comando reemplaza el marcador de posición, por lo que Claude recibe datos reales, no el comando en sí. Claude Code no ejecuta estos comandos en su máquina cuando la skill está [sincronizada desde su cuenta de claude.ai](#how-claude-code-handles-the-body-of-a-synced-skill). Esta restricción requiere Claude Code v2.1.228 o posterior.

Esta skill resume una solicitud de extracción obteniendo datos de PR en vivo con la CLI de GitHub. Los comandos `` !`gh pr diff` `` y otros se ejecutan primero, y su salida se inserta en el prompt:

```yaml theme={null}
---
name: pr-summary
description: Summarize changes in a pull request
context: fork
agent: Explore
allowed-tools: Bash(gh *)
---

## Pull request context
- PR diff: !`gh pr diff`
- PR comments: !`gh pr view --comments`
- Changed files: !`gh pr diff --name-only`

## Your task
Summarize this pull request...
```

La sustitución se ejecuta una sola vez sobre el archivo original. La salida del comando se inserta como texto sin formato y no se vuelve a escanear para buscar más marcadores de posición `` !`<command>` ``, por lo que un comando no puede emitir un marcador de posición para que una pasada posterior lo expanda.

La forma en línea solo se reconoce cuando `!` aparece al inicio de una línea o inmediatamente después de espacios en blanco. Si `!` sigue a otro carácter, como en `` KEY=!`cmd` ``, el marcador de posición se deja como texto literal y el comando no se ejecuta.

Para comandos de varias líneas, use un bloque de código delimitado abierto con ` ```! ` en lugar de la forma en línea:

````markdown theme={null}
## Environment
```!
node --version
git status --short
```
````

Para desactivar este comportamiento para skills y comandos personalizados de fuentes de usuario, proyecto, plugin o [directorio adicional](#skills-from-additional-directories), establezca `"disableSkillShellExecution": true` en [settings](/docs/es/settings). Cada comando se reemplaza con `[shell command execution disabled by policy]` en lugar de ejecutarse. Las skills agrupadas y administradas no se ven afectadas. Esta configuración es más útil en [managed settings](/docs/es/managed-settings), donde los usuarios no pueden anularla.

Claude Code nunca ejecuta estos comandos en su máquina cuando aparecen en skills [sincronizadas desde su cuenta de claude.ai](#how-synced-skills-behave), independientemente de esta configuración. Esta restricción requiere Claude Code v2.1.228 o posterior. [How Claude Code handles the body of a synced skill](#how-claude-code-handles-the-body-of-a-synced-skill) dice qué recibe Claude en lugar del comando en cada tipo de sesión.

<Tip>
  Para solicitar un razonamiento más profundo cuando se ejecuta una skill, incluya `ultrathink` en cualquier lugar del contenido de la skill. Consulte [Use ultrathink for one-off deep reasoning](/docs/es/model-config#use-ultrathink-for-one-off-deep-reasoning).
</Tip>

<h4 id="how-injected-commands-run">
  Cómo se ejecutan los comandos inyectados
</h4>

Claude Code elige la herramienta que ejecuta los comandos inyectados de una skill desde la clave `shell` en el frontmatter de la skill y su entorno. Cada combinación ejecuta los comandos a través de la herramienta Bash o la herramienta PowerShell, excepto una que falla la invocación directamente:

* `shell: powershell`, con la [herramienta PowerShell](/docs/es/tools-reference#powershell-tool) habilitada: los comandos se ejecutan a través de la herramienta PowerShell.
* `shell: bash` cuando bash no está disponible: la invocación falla antes de que se ejecute ningún comando. Esto sucede en Windows sin Git Bash. Claude Code muestra ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found``.
* Cualquier otra combinación: los comandos se ejecutan a través de la herramienta Bash cuando bash está disponible. Cuando no lo está, se ejecutan a través de la herramienta PowerShell.

Cualquiera de las herramientas ejecuta los comandos de la misma manera que ejecuta los propios comandos de shell de Claude. Comparten el directorio de trabajo, el tiempo de espera y el manejo de salida:

* **Directorio de trabajo**: Claude Code ejecuta cada comando en el directorio de trabajo actual del shell de la sesión. Ese directorio se mueve cuando Claude ejecuta `cd`. Use [`${CLAUDE_SKILL_DIR}` o `${CLAUDE_PROJECT_DIR}`](#available-string-substitutions) en rutas que deben resolverse de la misma manera cada vez.
* **stderr**: con el shell `bash` predeterminado, Claude Code fusiona stderr en stdout. Cualquier cosa que el comando escriba en stderr aparece en el texto inyectado.
* **Tiempo de espera**: cada comando se ejecuta bajo el [tiempo de espera](/docs/es/tools-reference#timeout-and-output-limits) predeterminado de 2 minutos de la herramienta Bash. Cuando la herramienta Bash [mueve un comando que agotó el tiempo de espera al fondo](/docs/es/tools-reference#background-commands), la skill aún se renderiza. El texto inyectado informa el movimiento y nombra la tarea de fondo y el archivo que recopila la salida del comando. Cuando el comando es uno que la herramienta Bash nunca pone automáticamente en segundo plano, Claude Code lo mata en el tiempo de espera. Esa falla [aborta la invocación](#when-an-injected-command-fails).
* **Tamaño de salida**: la salida más allá del techo en línea de la herramienta Bash llega como una ruta de archivo más una vista previa corta, no texto truncado. [Output limits](/docs/es/tools-reference#output-limits) cubre el techo y cómo ajustar cada límite.

La herramienta PowerShell aplica el mismo comportamiento de tiempo de espera, puesta en segundo plano y techo de salida a los comandos que ejecuta. Consulte la sección [herramienta PowerShell](/docs/es/tools-reference#powershell-tool) para obtener sus detalles específicos.

<h4 id="when-an-injected-command-fails">
  Cuando un comando inyectado falla
</h4>

Un comando fallido aborta toda la invocación de la skill, no solo su propio marcador de posición. Claude nunca ve el contenido de la skill para esa invocación. El aborto muestra `Shell command failed for pattern "..."`. El mensaje de error incluye la salida del comando bajo `[stderr]`.

Con el shell `bash` predeterminado, cualquier código de salida distinto de cero cuenta como un fallo. Se aplica una excepción: Claude Code trata el código de salida 1 de [comandos de búsqueda y comparación](/docs/es/tools-reference#output-limits) como un resultado normal e inyecta su salida. Los códigos de salida de 2 o superior fallan incluso para esos comandos.

Qué comandos obtienen la excepción depende del shell:

* Shell `bash` predeterminado: los comandos enumerados en [Output limits](/docs/es/tools-reference#output-limits)
* `shell: powershell`, cuando la herramienta PowerShell está habilitada: un [conjunto diferente](/docs/es/tools-reference#shell-selection-in-settings-hooks-and-skills) que incluye `grep` y `git diff` pero no `find` o `diff`

Con el shell `bash` predeterminado, agregue `|| true` a cualquier otro comando que espere que salga con un código distinto de cero. Un script de verificación que sale con 1 cuando encuentra problemas es un ejemplo.

<h4 id="permission-checks-on-injected-commands">
  Verificaciones de permisos en comandos inyectados
</h4>

Los comandos inyectados nunca solicitan permiso mientras se renderiza la skill. Claude Code verifica cada uno contra sus [reglas de permisos](/docs/es/permissions) primero. Un comando que una regla de denegación coincide aborta la invocación con `Shell command permission check failed for pattern "..."`.

Fuera del [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode), cuando la verificación de permisos de un comando devuelve algo que no sea permitir, Claude Code aborta la invocación con el mismo error. Esto incluye una regla que normalmente le preguntaría. Para evitar que un comando sin coincidencia aborte aquí, apruébelo previamente con [`allowed-tools`](#pre-approve-tools-for-a-skill). Las reglas de denegación y solicitud aún anulan `allowed-tools`. Consulte [Manage permissions](/docs/es/permissions#manage-permissions).

En modo automático, un comando que de otro modo necesitaría su aprobación no aborta la invocación. La skill se carga con una instrucción que le dice a Claude que ejecute el comando primero, y la llamada propia de Claude luego pasa por las [verificaciones habituales del modo automático](/docs/es/permission-modes#how-the-classifier-evaluates-actions). La invocación aún aborta en una [skill bifurcada](#run-skills-in-a-subagent) que establece `agent`, y en una sesión donde Claude no tiene la [herramienta de shell que ejecuta comandos inyectados](#how-injected-commands-run).

<h3 id="run-skills-in-a-subagent">
  Ejecutar skills en un subagente
</h3>

Agregue `context: fork` a su frontmatter cuando desee que una skill se ejecute en aislamiento. Claude Code inicia un nuevo subagente del tipo establecido en el campo `agent` y le da el contenido de la skill como su prompt. El subagente no ve su historial de conversación, por lo que las instrucciones de la skill tienen que ser autosuficientes.

<Note>
  A pesar del nombre, una skill con `context: fork` no se ejecuta en un [fork de la conversación actual](/docs/es/sub-agents#fork-the-current-conversation), que entregaría al subagente todo lo que ha discutido hasta ahora. Cuando la tarea depende de ese historial, bifurque la conversación en lugar de usar `context: fork`.
</Note>

El subagente bifurcado se ejecuta en [segundo plano](/docs/es/sub-agents#run-subagents-in-foreground-or-background): usted continúa trabajando mientras se ejecuta, y su resultado llega a su conversación cuando se completa. Establezca `background: false` en el frontmatter para esperar el resultado en el turno que invocó la skill. Antes de v2.1.218, las skills bifurcadas siempre bloqueaban el turno hasta que terminaban.

Claude Code también espera el resultado, incluso cuando la skill no establece `background: false`, en casos como estos:

* En modo no interactivo, con la bandera `-p` o el Agent SDK
* Cuando establece [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](/docs/es/env-vars) en `1`, que también desactiva todas las otras características de tareas en segundo plano
* Cuando invoca una skill bifurcada mientras una invocación anterior de la misma skill aún se está ejecutando
* Cuando una [tarea programada](/docs/es/scheduled-tasks) se dispara con la skill como su prompt

Un fork en segundo plano también se ejecuta con el [conjunto de herramientas más estrecho que se aplica a los subagentes en segundo plano](/docs/es/sub-agents#run-subagents-in-foreground-or-background): el subagente de la skill es un tipo de agente regular, por lo que la exención para subagentes que bifurcan la conversación no lo cubre. Si los pasos de su skill dependen de una herramienta fuera de ese conjunto, establezca `background: false` para mantener el conjunto completo de herramientas.

Una skill bifurcada que se ejecuta en segundo plano aplica sus ediciones fuera de los [checkpoints](/docs/es/checkpointing) de su sesión, por lo que `/rewind` no las deshace; use git para revertirlas.

<Warning>
  `context: fork` solo tiene sentido para skills con instrucciones explícitas. Si su skill contiene directrices como "use estas convenciones de API" sin una tarea, el subagente recibe las directrices pero sin un prompt procesable, y regresa sin salida significativa.
</Warning>

Las skills y los [subagentes](/docs/es/sub-agents) funcionan juntos en dos direcciones:

| Enfoque                      | Prompt del sistema            | Tarea                           | También carga                                                                                              |
| :--------------------------- | :---------------------------- | :------------------------------ | :--------------------------------------------------------------------------------------------------------- |
| Skill con `context: fork`    | Del tipo de agente            | Contenido de SKILL.md           | CLAUDE.md, per the agent's [startup context](/docs/es/sub-agents#what-loads-at-startup)                         |
| Subagente con campo `skills` | Cuerpo markdown del subagente | Mensaje de delegación de Claude | Skills precargadas + CLAUDE.md, per the subagent's [startup context](/docs/es/sub-agents#what-loads-at-startup) |

Con `context: fork`, escribe la tarea en su skill y elige un tipo de agente para ejecutarla. Los agentes Explore y Plan integrados [omiten CLAUDE.md y el estado de git](/docs/es/sub-agents#what-loads-at-startup) para mantener su contexto pequeño, por lo que una skill bifurcada que usa `agent: Explore` solo ve el contenido de SKILL.md y el prompt del sistema del agente. Para lo inverso, donde define un subagente personalizado que usa skills como material de referencia, consulte [Subagentes](/docs/es/sub-agents#preload-skills-into-subagents).

<h4 id="example-research-skill-using-explore-agent">
  Ejemplo: Skill de investigación usando agente Explore
</h4>

Esta skill ejecuta investigación en un agente Explore bifurcado. El contenido de la skill se convierte en la tarea, y el agente proporciona herramientas de solo lectura optimizadas para la exploración de bases de código:

```yaml theme={null}
---
name: deep-research
description: Research a topic thoroughly
context: fork
agent: Explore
---

Research $ARGUMENTS thoroughly:

1. Find relevant files using Glob and Grep
2. Read and analyze the code
3. Summarize findings with specific file references
```

Cuando se ejecuta esta skill:

1. Se crea un nuevo contexto aislado
2. El subagente recibe el contenido de la skill como su prompt (las instrucciones "Research \$ARGUMENTS thoroughly")
3. El campo `agent` determina el entorno de ejecución (modelo, herramientas y permisos)
4. El subagente resume sus resultados y los devuelve a su conversación principal cuando termina

El campo `agent` especifica qué configuración de subagente usar. Las opciones incluyen agentes integrados (`Explore`, `Plan`, `general-purpose`) o cualquier subagente personalizado de `.claude/agents/`. Si se omite, usa `general-purpose`.

<h3 id="restrict-claude’s-skill-access">
  Restringir el acceso de Claude a las skills
</h3>

De forma predeterminada, Claude puede invocar cualquier skill que no tenga `disable-model-invocation: true` establecido. Las skills que definen `allowed-tools` otorgan a Claude acceso a esas herramientas sin aprobación por uso durante el turno que invoca la skill; la concesión se borra cuando envía su siguiente mensaje. Su [configuración de permisos](/docs/es/permissions) aún rige el comportamiento de aprobación de línea base para todas las otras herramientas. Algunos comandos integrados también están disponibles a través de la herramienta Skill, incluidos `/init` y `/security-review`. Otros comandos integrados como `/compact` no lo están.

Tres formas de controlar qué skills puede invocar Claude:

**Desactivar todas las skills** denegando la herramienta Skill en `/permissions`:

```text theme={null}
# Add to deny rules:
Skill
```

**Permitir o denegar skills específicas** usando [reglas de permisos](/docs/es/permissions):

```text theme={null}
# Allow only specific skills
Skill(commit)
Skill(review-pr *)

# Deny specific skills
Skill(deploy *)
```

Sintaxis de permisos: `Skill(name)` para coincidencia exacta, `Skill(name *)` para coincidencia de prefijo con cualquier argumento.

Si su regla `deny` nombra un alias o un nombre no calificado en lugar del nombre de la skill, Claude Code aún bloquea la skill: con `Skill(review)` bloquea el `/code-review` agrupado a través de su alias `/review`, y con `Skill(deploy)` bloquea una [skill anidada](#where-skills-live) listada como `apps/web:deploy` a través de su nombre no calificado. Antes de v2.1.260, Claude Code no bloqueaba una skill anidada listada bajo su nombre calificado cuando la regla de denegación solo nombraba el nombre no calificado.

Claude Code coincide una regla `allow` solo contra el nombre de la skill y el nombre en la invocación de Claude.

**Ocultar skills individuales** agregando `disable-model-invocation: true` a su frontmatter. Esto elimina la skill del contexto de Claude por completo.

<Note>
  Con `user-invocable: false`, no puede invocar la skill, pero Claude aún puede. Para evitar que Claude la invoque a través de la herramienta Skill, establezca `disable-model-invocation: true`.
</Note>

<h3 id="override-skill-visibility-from-settings">
  Anular la visibilidad de la skill desde la configuración
</h3>

La configuración `skillOverrides` controla la visibilidad de la skill desde su [configuración](/docs/es/settings) en lugar del frontmatter de la skill. Úsela para skills cuyo SKILL.md no desea editar, como las que se registran en un repositorio de proyecto compartido. El menú `/skills` lo escribe por usted: resalte una skill y presione `Space` para ciclar estados, luego `Esc` para guardar en `.claude/settings.local.json`.

Cada clave es un nombre de skill y cada valor es uno de cuatro estados:

| Valor                   | Listado a Claude     | En menú `/` |
| :---------------------- | :------------------- | :---------- |
| `"on"`                  | Nombre y descripción | Sí          |
| `"name-only"`           | Solo nombre          | Sí          |
| `"user-invocable-only"` | Oculto               | Sí          |
| `"off"`                 | Oculto               | Oculto      |

El menú `/skills` etiqueta el estado `"user-invocable-only"` como `user-only`.

A partir de v2.1.199, `"off"` también oculta la skill de las listas de comandos anunciadas a clientes de [Remote Control](/docs/es/remote-control) y a llamadores de [Agent SDK](/docs/es/agent-sdk/skills#discover-available-commands), además del menú `/` del terminal. Invocar una skill oculta por su nombre completo aún devuelve el error `skillOverrides` en lugar de ejecutarla.

Una skill que está ausente de `skillOverrides` se trata como `"on"`. El ejemplo a continuación colapsa una skill a su nombre y desactiva otra por completo:

```json theme={null}
{
  "skillOverrides": {
    "legacy-context": "name-only",
    "deploy": "off"
  }
}
```

Algunas skills agrupadas tienen alias, como `checkup` para `/doctor`. Si establece una entrada `skillOverrides` bajo un alias en [managed settings](/docs/es/managed-settings) o en un archivo que pasa con la bandera `--settings`, Claude Code la aplica a la skill detrás del alias. Solo puede restringir una skill más a través de un alias, nunca hacerla más visible, y si también establece una entrada bajo el nombre de la skill en managed settings, esa entrada tiene prioridad. Antes de v2.1.260, Claude Code no aplicaba una entrada bajo un alias a la skill en ninguna fuente de configuración.

En configuración de usuario, proyecto y local, Claude Code coincide entradas solo contra nombres de skill. Si establece una entrada para `review` allí, se aplica a una skill nombrada `review`, no al `/code-review` agrupado a través de su alias `/review`.

Las skills de plugins no se ven afectadas por `skillOverrides`. Administre esas a través de `/plugin` en su lugar.

<h3 id="find-unused-skills">
  Encontrar skills no utilizadas
</h3>

Cada skill en el [listado de skills](#skill-descriptions-are-cut-short) se suma a su contexto en cada turno, independientemente de si Claude la usa o no. Ejecute `/skill-doctor` para ver qué cuesta cada una de sus skills y con qué frecuencia se usa, para que pueda decidir cuáles desactivar. En una sesión interactiva, el informe se abre en la pestaña **Stats** del administrador `/plugin`. En [modo no interactivo](/docs/es/headless) con `-p`, Claude Code lo imprime como texto.

El informe cubre las skills en su sesión que no sean skills agrupadas ni skills empresariales. Marca las skills en el listado que nunca se han invocado y dice dónde desactivarlas. De las skills que le dice dónde desactivar, comience con las que tienen el costo de contexto más alto. El informe también enumera los plugins que no ha usado recientemente.

`/skill-doctor` requiere Claude Code v2.1.252 o posterior y no está disponible en sesiones que omiten [obtención de banderas de características](/docs/es/env-vars#features-that-need-feature-flag-fetching). Si ejecuta `/skill-doctor` sobre [Remote Control](/docs/es/remote-control) desde su teléfono o navegador, Claude Code responde [`Skill usage reports are not available on this connection.`](/docs/es/errors#skill-usage-reports-are-not-available-on-this-connection) en su lugar. Ejecute `/skill-doctor` en el terminal en la máquina donde se ejecuta la sesión.

<h2 id="evaluate-and-iterate-on-a-skill">
  Evaluar e iterar sobre una skill
</h2>

Ver que una skill se activa le indica que Claude la encontró, no que haya hecho lo que usted pretendía. Para saber que una skill funciona, mida dos cosas por separado: si Claude la invoca en los prompts que debería, y si la salida coincide con lo que espera cuando lo hace.

La verificación de ambas es una comparación de línea base. Recopile algunos prompts realistas, ejecute cada uno en una sesión nueva con la skill disponible y nuevamente con ella [deshabilitada](#override-skill-visibility-from-settings), y compare los resultados. Una sesión nueva es importante porque el contexto residual de la creación de la skill enmascarará las brechas en las instrucciones escritas.

Dos herramientas automatizan esa comparación. Para una skill que se distribuye en un [plugin](/docs/es/plugins/overview), [`claude plugin eval`](/docs/es/plugin-evals) ejecuta cada prompt en una sesión aislada con y sin el plugin, la califica con evaluadores que usted define o que escribe por usted, y sale con un código distinto de cero por debajo de un umbral para que pueda controlar su CI. Para iterar sobre una única skill dentro de una conversación de Claude Code, el plugin skill-creator a continuación ejecuta un bucle similar con su propio formato `evals/evals.json`. Los dos formatos no son intercambiables.

<h3 id="run-evals-with-skill-creator">
  Ejecutar evals con skill-creator
</h3>

El [plugin `skill-creator`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/skill-creator) automatiza el bucle de comparación dentro de Claude Code. Instálelo desde el marketplace oficial:

```text theme={null}
/plugin install skill-creator@claude-plugins-official
```

Si la instalación falla, haga coincidir el mensaje que Claude Code reporta:

* `Marketplace "claude-plugins-official" not found`: agregue el marketplace con `/plugin marketplace add anthropics/claude-plugins-official`, luego reintente la instalación.
* El plugin [no se encuentra en el marketplace](/docs/es/plugins/install#install-a-plugin): verifique el nombre del plugin.

Si el resumen de instalación reporta `Run /reload-plugins to activate.`, Claude Code entonces ejecuta esa recarga para usted. Si la recarga advierte que su próximo mensaje volvería a leer la conversación, ejecute `/reload-plugins --force` para que las skills del plugin estén disponibles en la sesión actual. Luego pida a Claude que evalúe una skill existente, por ejemplo `evaluate my summarize-changes skill with skill-creator`. El plugin lo guía a través de la escritura de casos de prueba y ejecuta el bucle:

* **Casos de prueba**: almacena prompts, archivos de entrada y comportamiento esperado en `evals/evals.json` dentro del directorio de la skill
* **Ejecuciones aisladas**: genera una [subagente](/docs/es/sub-agents) por caso de prueba para que cada ejecución comience con un contexto limpio, y registra el recuento de tokens y la duración
* **Calificación**: verifica cada aserción contra la salida y escribe aprobado o reprobado con evidencia en `grading.json`
* **Benchmark**: agrega la tasa de aprobación, tiempo y tokens para con-skill versus sin-skill en `benchmark.json` para que pueda comparar la mejora de la tasa de aprobación contra la sobrecarga de tokens y tiempo
* **Comparación de versiones**: ejecuta una prueba A/B ciega entre dos versiones de la skill para que pueda confirmar que una edición es una mejora antes de confirmarla
* **Ajuste de descripción**: genera prompts de debe-activarse y no-debe-activarse, mide la tasa de aciertos, y propone ediciones de descripción cuando la skill se activa en solicitudes incorrectas
* **Visor de revisión**: abre un informe HTML donde puede inspeccionar cada salida y registrar comentarios cualitativos que la siguiente iteración lee

Para el formato del archivo eval y el flujo de trabajo de iteración completo, consulte [Evaluating skill output quality](https://agentskills.io/skill-creation/evaluating-skills) en agentskills.io. Para obtener información sobre los modos de benchmark y comparación, consulte el [anuncio de skill-creator](https://claude.com/blog/improving-skill-creator-test-measure-and-refine-agent-skills).

<h2 id="share-skills">
  Compartir skills
</h2>

Los skills se pueden distribuir en diferentes ámbitos según su audiencia:

* **Skills de proyecto**: Confirmar `.claude/skills/` en el control de versiones
* **Plugins**: Crear un directorio `skills/` en su [plugin](/docs/es/plugins/overview)
* **Administrado**: Implementar en toda la organización a través de [configuración administrada](/docs/es/managed-settings)

<h3 id="generate-visual-output">
  Generar salida visual
</h3>

Los skills pueden agrupar y ejecutar scripts en cualquier lenguaje, dando a Claude capacidades más allá de lo que es posible en un único prompt. Un patrón es generar salida visual: archivos HTML interactivos que se abren en su navegador para explorar datos, depurar o crear informes.

Este ejemplo crea un explorador de base de código: una vista de árbol interactiva donde puede expandir y contraer directorios, ver tamaños de archivo de un vistazo e identificar tipos de archivo por color.

Cree el directorio Skill:

```bash theme={null}
mkdir -p ~/.claude/skills/codebase-visualizer/scripts
```

Guarde esto en `~/.claude/skills/codebase-visualizer/SKILL.md`. La descripción le dice a Claude cuándo activar este Skill, y las instrucciones le dicen a Claude que ejecute el script incluido. La ruta del script utiliza [`${CLAUDE_SKILL_DIR}`](#available-string-substitutions) para que se resuelva correctamente si el skill se instala a nivel personal, de proyecto o de plugin:

````yaml theme={null}
---
name: codebase-visualizer
description: Generate an interactive collapsible tree visualization of your codebase. Use when exploring a new repo, understanding project structure, or identifying large files.
allowed-tools: Bash(python3 *)
---

# Codebase Visualizer

Generate an interactive HTML tree view that shows your project's file structure with collapsible directories.

## Usage

Run the visualization script from your project root:

```bash
python3 ${CLAUDE_SKILL_DIR}/scripts/visualize.py .
```

This creates `codebase-map.html` in the current directory and opens it in your default browser.

## What the visualization shows

- **Collapsible directories**: Click folders to expand/collapse
- **File sizes**: Displayed next to each file
- **Colors**: Different colors for different file types
- **Directory totals**: Shows aggregate size of each folder
````

Guarde esto en `~/.claude/skills/codebase-visualizer/scripts/visualize.py`. Este script escanea un árbol de directorios y genera un archivo HTML independiente con:

* Una **barra lateral de resumen** que muestra el recuento de archivos, recuento de directorios, tamaño total y número de tipos de archivo
* Un **gráfico de barras** que desglosa la base de código por tipo de archivo (los 8 principales por tamaño)
* Un **árbol contraíble** donde puede expandir y contraer directorios, con indicadores de tipo de archivo codificados por color

El script requiere Python 3 pero utiliza solo bibliotecas integradas, por lo que no hay paquetes que instalar:

```python expandable theme={null}
#!/usr/bin/env python3
"""Generate an interactive collapsible tree visualization of a codebase."""

import json
import sys
import webbrowser
from html import escape
from pathlib import Path
from collections import Counter

IGNORE = {'.git', 'node_modules', '__pycache__', '.venv', 'venv', 'dist', 'build'}

def scan(path: Path, stats: dict) -> dict:
    result = {"name": path.name, "children": [], "size": 0}
    try:
        for item in sorted(path.iterdir()):
            if item.name in IGNORE or item.name.startswith('.'):
                continue
            if item.is_file():
                size = item.stat().st_size
                ext = item.suffix.lower() or '(no ext)'
                result["children"].append({"name": item.name, "size": size, "ext": ext})
                result["size"] += size
                stats["files"] += 1
                stats["extensions"][ext] += 1
                stats["ext_sizes"][ext] += size
            elif item.is_dir():
                stats["dirs"] += 1
                child = scan(item, stats)
                if child["children"]:
                    result["children"].append(child)
                    result["size"] += child["size"]
    except PermissionError:
        pass
    return result

def generate_html(data: dict, stats: dict, output: Path) -> None:
    ext_sizes = stats["ext_sizes"]
    total_size = sum(ext_sizes.values()) or 1
    sorted_exts = sorted(ext_sizes.items(), key=lambda x: -x[1])[:8]
    colors = {
        '.js': '#f7df1e', '.ts': '#3178c6', '.py': '#3776ab', '.go': '#00add8',
        '.rs': '#dea584', '.rb': '#cc342d', '.css': '#264de4', '.html': '#e34c26',
        '.json': '#6b7280', '.md': '#083fa1', '.yaml': '#cb171e', '.yml': '#cb171e',
        '.mdx': '#083fa1', '.tsx': '#3178c6', '.jsx': '#61dafb', '.sh': '#4eaa25',
    }
    lang_bars = "".join(
        f'<div class="bar-row"><span class="bar-label">{ext}</span>'
        f'<div class="bar" style="width:{(size/total_size)*100}%;background:{colors.get(ext,"#6b7280")}"></div>'
        f'<span class="bar-pct">{(size/total_size)*100:.1f}%</span></div>'
        for ext, size in sorted_exts
    )
    def fmt(b):
        if b < 1024: return f"{b} B"
        if b < 1048576: return f"{b/1024:.1f} KB"
        return f"{b/1048576:.1f} MB"

    html = f'''<!DOCTYPE html>
<html><head>
  <meta charset="utf-8"><title>Codebase Explorer</title>
  <style>
    body {{ font: 14px/1.5 system-ui, sans-serif; margin: 0; background: #1a1a2e; color: #eee; }}
    .container {{ display: flex; height: 100vh; }}
    .sidebar {{ width: 280px; background: #252542; padding: 20px; border-right: 1px solid #3d3d5c; overflow-y: auto; flex-shrink: 0; }}
    .main {{ flex: 1; padding: 20px; overflow-y: auto; }}
    h1 {{ margin: 0 0 10px 0; font-size: 18px; }}
    h2 {{ margin: 20px 0 10px 0; font-size: 14px; color: #888; text-transform: uppercase; }}
    .stat {{ display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid #3d3d5c; }}
    .stat-value {{ font-weight: bold; }}
    .bar-row {{ display: flex; align-items: center; margin: 6px 0; }}
    .bar-label {{ width: 55px; font-size: 12px; color: #aaa; }}
    .bar {{ height: 18px; border-radius: 3px; }}
    .bar-pct {{ margin-left: 8px; font-size: 12px; color: #666; }}
    .tree {{ list-style: none; padding-left: 20px; }}
    details {{ cursor: pointer; }}
    summary {{ padding: 4px 8px; border-radius: 4px; }}
    summary:hover {{ background: #2d2d44; }}
    .folder {{ color: #ffd700; }}
    .file {{ display: flex; align-items: center; padding: 4px 8px; border-radius: 4px; }}
    .file:hover {{ background: #2d2d44; }}
    .size {{ color: #888; margin-left: auto; font-size: 12px; }}
    .dot {{ width: 8px; height: 8px; border-radius: 50%; margin-right: 8px; }}
  </style>
</head><body>
  <div class="container">
    <div class="sidebar">
      <h1>📊 Summary</h1>
      <div class="stat"><span>Files</span><span class="stat-value">{stats["files"]:,}</span></div>
      <div class="stat"><span>Directories</span><span class="stat-value">{stats["dirs"]:,}</span></div>
      <div class="stat"><span>Total size</span><span class="stat-value">{fmt(data["size"])}</span></div>
      <div class="stat"><span>File types</span><span class="stat-value">{len(stats["extensions"])}</span></div>
      <h2>By file type</h2>
      {lang_bars}
    </div>
    <div class="main">
      <h1>📁 {escape(data["name"])}</h1>
      <ul class="tree" id="root"></ul>
    </div>
  </div>
  <script>
    const data = {json.dumps(data)};
    const colors = {json.dumps(colors)};
    function fmt(b) {{ if (b < 1024) return b + ' B'; if (b < 1048576) return (b/1024).toFixed(1) + ' KB'; return (b/1048576).toFixed(1) + ' MB'; }}
    function esc(s) {{ return s.replace(/[&<>"']/g, c => ({{"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}}[c])); }}
    function render(node, parent) {{
      if (node.children) {{
        const det = document.createElement('details');
        det.open = parent === document.getElementById('root');
        det.innerHTML = `<summary><span class="folder">📁 ${{esc(node.name)}}</span><span class="size">${{fmt(node.size)}}</span></summary>`;
        const ul = document.createElement('ul'); ul.className = 'tree';
        node.children.sort((a,b) => (b.children?1:0)-(a.children?1:0) || a.name.localeCompare(b.name));
        node.children.forEach(c => render(c, ul));
        det.appendChild(ul);
        const li = document.createElement('li'); li.appendChild(det); parent.appendChild(li);
      }} else {{
        const li = document.createElement('li'); li.className = 'file';
        li.innerHTML = `<span class="dot" style="background:${{colors[node.ext]||'#6b7280'}}"></span>${{esc(node.name)}}<span class="size">${{fmt(node.size)}}</span>`;
        parent.appendChild(li);
      }}
    }}
    data.children.forEach(c => render(c, document.getElementById('root')));
  </script>
</body></html>'''
    output.write_text(html)

if __name__ == '__main__':
    target = Path(sys.argv[1] if len(sys.argv) > 1 else '.').resolve()
    stats = {"files": 0, "dirs": 0, "extensions": Counter(), "ext_sizes": Counter()}
    data = scan(target, stats)
    out = Path('codebase-map.html')
    generate_html(data, stats, out)
    print(f'Generated {out.absolute()}')
    webbrowser.open(f'file://{out.absolute()}')
```

Para probar, abra Claude Code en cualquier proyecto y pregunte "Visualize this codebase." Claude ejecuta el script, que imprime la ruta del archivo generado, como `Generated /path/to/codebase-map.html`, y lo abre en su navegador. Si trabaja en un entorno sin interfaz gráfica donde no se abre ningún navegador, la ruta impresa confirma que el script se ejecutó correctamente.

Este patrón funciona para cualquier salida visual: gráficos de dependencias, informes de cobertura de pruebas, documentación de API o visualizaciones de esquemas de bases de datos. El script incluido realiza el trabajo mientras Claude maneja la orquestación.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="skill-not-triggering">
  Skill no se activa
</h3>

Si Claude no utiliza su skill cuando se espera:

1. Verifique que la descripción incluya palabras clave que los usuarios dirían naturalmente
2. Confirme que el skill aparece en `¿Qué skills están disponibles?`
3. Intente reformular su solicitud para que coincida más estrechamente con la descripción
4. Invóquelo directamente con `/skill-name` si el skill es invocable por el usuario

Si el YAML del frontmatter está mal formado, Claude Code carga el cuerpo del skill con metadatos vacíos, por lo que `/skill-name` sigue funcionando pero Claude no puede comparar contra su `description`. Ejecute con `--debug` para ver el error de análisis.

Si el skill se distribuye en un plugin, puede medir con qué frecuencia se activa en solicitudes realistas en lugar de verificar una por una: escriba un caso de evaluación con un [calificador `tool_used: Skill`](/docs/es/plugin-evals#create-your-first-eval-suite) y ejecútelo con `claude plugin eval` después de cada cambio de descripción.

Para encontrar archivos `SKILL.md` cuyo frontmatter no se analiza, ejecute [`claude plugin validate`](/docs/es/plugins/cli-reference#validate-a-directory) en el directorio de skills, por ejemplo `claude plugin validate .claude/skills` para skills de proyecto o `claude plugin validate ~/.claude/skills` para skills personales. Requiere Claude Code v2.1.233 o posterior.

<h3 id="skill-triggers-too-often">
  Skill se activa demasiado a menudo
</h3>

Si Claude utiliza su skill cuando no lo desea:

1. Haga la descripción más específica
2. Agregue `disable-model-invocation: true` si solo desea invocación manual

<h3 id="skill-descriptions-are-cut-short">
  Las descripciones de skills se cortan
</h3>

Claude Code carga un listado de nombres y descripciones de skills en el contexto para que Claude sepa qué está disponible. El listado siempre contiene todos los nombres de skills, pero si tiene muchos skills, Claude Code acorta las descripciones para ajustarse al presupuesto de caracteres del listado, lo que puede eliminar las palabras clave que Claude necesita para comparar con su solicitud. El presupuesto se escala al 1% de la ventana de contexto del modelo. Cuando el listado se desborda, Claude Code elimina descripciones comenzando con los skills que invoca menos, por lo que los skills que utiliza más mantienen su texto completo.

Ejecute `/doctor` para obtener una estimación del costo de contexto del listado y sus mayores contribuyentes. Para encontrar skills que vale la pena desactivar, ejecute [`/skill-doctor`](#find-unused-skills). Cuando el listado excede su presupuesto, Claude Code también escribe una advertencia en el registro de depuración, visible con [`--debug`](/docs/es/cli-reference#cli-flags).

La fila Skills en `/context` reporta el tamaño del listado después de que se aplica el presupuesto, por lo que coincide con lo que recibe el modelo. Antes de v2.1.196, la fila contaba el texto completo de cada descripción y podría mostrar un valor varias veces mayor que el presupuesto configurado.

Para aumentar el presupuesto, establezca la configuración [`skillListingBudgetFraction`](/docs/es/settings-reference#skilllistingbudgetfraction) (por ejemplo, `0.02` = 2%) o la variable de entorno `SLASH_COMMAND_TOOL_CHAR_BUDGET` a un recuento de caracteres fijo. Para liberar presupuesto para otros skills, establezca entradas de baja prioridad en `"name-only"` en [`skillOverrides`](#override-skill-visibility-from-settings) para que se listen sin descripción. También puede recortar el texto de `description` y `when_to_use` en la fuente: coloque el caso de uso clave primero, ya que el texto combinado de cada entrada está limitado a 1.536 caracteres independientemente del presupuesto. El límite es configurable con [`skillListingMaxDescChars`](/docs/es/settings-reference#skilllistingmaxdescchars).

<h3 id="personal-skills-disappeared">
  Skills personales desaparecieron
</h3>

Si las carpetas de skills que creó en `~/.claude/skills/` desaparecieron, busque en `~/.claude/skills/.trash/`. Cuando Claude Code [sincroniza skills desde claude.ai](#how-synced-skills-behave), las descarga en la subcarpeta separada `synced` y no mueve ni elimina las carpetas que crea.

Antes de v2.1.280, un archivo llamado `manifest.json` en `~/.claude/skills/` causaba que Claude Code moviera las carpetas de skills que ese archivo listaba a una carpeta con marca de tiempo bajo `~/.claude/skills/.trash/`, y esos skills dejaban de cargarse.

Para restaurar un skill, mueva su carpeta de la carpeta con marca de tiempo de vuelta a `~/.claude/skills/`. Haga esto antes de que la [limpieza de retención](/docs/es/claude-directory#cleaned-up-automatically) elimine las entradas de papelera, por defecto 30 días después de que se movieron a la papelera.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* **[Depura tu configuración](/docs/es/debug-your-config)**: diagnostica por qué una skill no aparece o no se activa
* **[Evaluating skill output quality](https://agentskills.io/skill-creation/evaluating-skills)**: el formato del archivo eval y el flujo de trabajo de iteración en agentskills.io
* **[Skill authoring best practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices)**: orientación de escritura que se aplica en productos Claude
* **[Subagents](/docs/es/sub-agents)**: delega tareas a agents especializados
* **[Plugins](/docs/es/plugins/overview)**: empaqueta y distribuye skills con otras extensiones
* **[Hooks](/docs/es/hooks)**: automatiza flujos de trabajo alrededor de eventos de herramientas
* **[Memory](/docs/es/memory)**: gestiona archivos CLAUDE.md para contexto persistente
* **[Comandos](/docs/es/commands)**: referencia para comandos integrados y skills agrupados
* **[Permisos](/docs/es/permissions)**: controla el acceso a herramientas y skills
* **[Claude Tag skills](https://claude.com/docs/claude-tag/admins/skills-repo)**: skills de proyecto comprometidas en un repositorio también se cargan cuando ese repositorio se utiliza en un canal de Claude Tag
