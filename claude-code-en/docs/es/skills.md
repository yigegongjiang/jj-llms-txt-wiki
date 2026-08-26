> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ampliar Claude con skills

> Crear, gestionar y compartir skills para ampliar las capacidades de Claude en Claude Code. Incluye comandos personalizados y skills agrupados.

Los skills amplían lo que Claude puede hacer. Cree un archivo `SKILL.md` con instrucciones, y Claude lo añade a su kit de herramientas. Claude utiliza skills cuando es relevante, o puede invocar uno directamente con `/skill-name`.

Cree un skill cuando siga pegando el mismo manual, lista de verificación o procedimiento de varios pasos en el chat, o cuando una sección de CLAUDE.md se haya convertido en un procedimiento en lugar de un hecho. A diferencia del contenido de CLAUDE.md, el cuerpo de un skill se carga solo cuando se usa, por lo que el material de referencia largo cuesta casi nada hasta que lo necesita.

<Note>
  Para comandos integrados como `/help` y `/compact`, y skills agrupados como `/debug` y `/code-review`, consulte la [referencia de comandos](/docs/es/commands).

  **Los comandos personalizados se han fusionado con los skills.** Un archivo en `.claude/commands/deploy.md` y un skill en `.claude/skills/deploy/SKILL.md` crean ambos `/deploy` y funcionan de la misma manera. Sus archivos existentes en `.claude/commands/` siguen funcionando. Los skills añaden características opcionales: un directorio para archivos de apoyo, frontmatter para [controlar si usted o Claude los invoca](#control-who-invokes-a-skill), y la capacidad de que Claude los cargue automáticamente cuando sea relevante.
</Note>

Los skills de Claude Code siguen el estándar abierto [Agent Skills](https://agentskills.io), que funciona en múltiples herramientas de IA. Claude Code extiende el estándar con características adicionales como [control de invocación](#control-who-invokes-a-skill), [ejecución de subagent](#run-skills-in-a-subagent), e [inyección de contexto dinámico](#inject-dynamic-context).

<h2 id="bundled-skills">
  Skills agrupados
</h2>

Claude Code incluye un conjunto de skills agrupados que están disponibles en cada sesión a menos que se desactiven con la configuración [`disableBundledSkills`](/docs/es/settings#available-settings), incluyendo `/doctor`, `/code-review`, `/batch`, `/debug`, `/loop` y `/claude-api`. A diferencia de la mayoría de comandos integrados, que ejecutan lógica fija directamente, los skills agrupados se basan en prompts: dan a Claude instrucciones detalladas y le permiten orquestar el trabajo utilizando sus herramientas. Los invoca de la misma manera que cualquier otro skill, escribiendo `/` seguido del nombre del skill.

La verificación de configuración [`/doctor`](/docs/es/commands#all-commands) es la única excepción a `disableBundledSkills` en Claude Code v2.1.205 y posterior: permanece escribible cuando la configuración está activada. Para ocultarla, establezca la variable de entorno `DISABLE_DOCTOR_COMMAND` o una entrada [`skillOverrides`](#override-skill-visibility-from-settings) de `"doctor": "off"`. Antes de v2.1.205, `/doctor` era un comando integrado en lugar de un skill agrupado.

Los skills agrupados se enumeran junto con los comandos integrados en la [referencia de comandos](/docs/es/commands), marcados como **Skill** en la columna Propósito.

<h3 id="run-and-verify-your-app">
  Ejecutar y verificar su aplicación
</h3>

Tres skills agrupados trabajan juntos para lanzar su aplicación y confirmar cambios contra la aplicación en ejecución en lugar de solo pruebas:

| Skill                  | Propósito                                                                                                                                     |
| :--------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------- |
| `/run`                 | Lanzar y ejecutar su aplicación para ver un cambio funcionando                                                                                |
| `/verify`              | Compilar y ejecutar su aplicación para confirmar que un cambio de código hace lo que debería, sin recurrir a pruebas o verificaciones de tipo |
| `/run-skill-generator` | Enseñar a `/run` y `/verify` cómo compilar e iniciar su proyecto                                                                              |

Los tres skills requieren Claude Code v2.1.145 o posterior.

`/run` y `/verify` funcionan sin configuración. Deducen el lanzamiento de su tipo de proyecto (CLI, servidor, TUI, impulsado por navegador) y de lo que hay en su README, `package.json` o `Makefile`. Esa deducción se vuelve poco confiable para proyectos que necesitan algo más allá de un lanzamiento estándar: una base de datos, un archivo env, una sesión gráfica, una compilación de varios pasos.

`/run-skill-generator` registra la receta en su lugar. Consigue que su aplicación se ejecute desde un entorno limpio, captura lo que funcionó (los comandos de instalación, las variables de entorno, el script de lanzamiento) y lo confirma como un skill por proyecto en `.claude/skills/run-<name>/`. Después de eso, `/run`, `/verify` y cualquier otro agente en el repositorio siguen la receta registrada en lugar de redescubrirla. Ejecute `/run-skill-generator` una vez por proyecto, y nuevamente si el proceso de compilación o lanzamiento cambia.

<h2 id="getting-started">
  Primeros pasos
</h2>

<h3 id="create-your-first-skill">
  Crear su primer skill
</h3>

Este ejemplo crea un skill que resume los cambios sin confirmar en su repositorio git e identifica cualquier cosa arriesgada. Extrae el diff en vivo en el prompt antes de que Claude lo lea, por lo que la respuesta se basa en su árbol de trabajo real en lugar de lo que Claude puede adivinar a partir de archivos abiertos. Claude carga el skill automáticamente cuando pregunta sobre sus cambios, o puede invocarlo directamente con `/summarize-changes`.

<Steps>
  <Step title="Crear el directorio del skill">
    Cree un directorio para el skill en su carpeta de skills personales. Los skills personales están disponibles en todos sus proyectos.

    ```bash theme={null}
    mkdir -p ~/.claude/skills/summarize-changes
    ```
  </Step>

  <Step title="Escribir SKILL.md">
    Cada skill necesita un archivo `SKILL.md` con dos partes: frontmatter YAML entre marcadores `---` que le dice a Claude cuándo usar el skill, y contenido markdown con las instrucciones que Claude sigue cuando se ejecuta el skill. El nombre del directorio se convierte en el comando que escribe, y la `description` ayuda a Claude a decidir cuándo cargar el skill automáticamente.

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

    La línea `` !`git diff HEAD` `` utiliza [inyección de contexto dinámico](#inject-dynamic-context): Claude Code ejecuta el comando y reemplaza la línea con su salida antes de que Claude vea el contenido del skill, por lo que las instrucciones llegan con el diff actual ya insertado.
  </Step>

  <Step title="Probar el skill">
    Abra un proyecto git, realice una pequeña edición en cualquier archivo e inicie Claude Code ejecutando `claude`. Puede probar el skill de dos maneras.

    **Dejar que Claude lo invoque automáticamente** haciendo una pregunta que coincida con la descripción:

    ```text theme={null}
    What did I change?
    ```

    **O invocarlo directamente** con el nombre del skill:

    ```text theme={null}
    /summarize-changes
    ```

    De cualquier manera, Claude debe responder con un breve resumen de su edición y una lista de riesgos.
  </Step>
</Steps>

<h3 id="where-skills-live">
  Dónde viven los skills
</h3>

Dónde almacena un skill determina quién puede usarlo:

| Ubicación  | Ruta                                                             | Se aplica a                           |
| :--------- | :--------------------------------------------------------------- | :------------------------------------ |
| Enterprise | Consulte [configuración gestionada](/docs/es/settings#settings-files) | Todos los usuarios de su organización |
| Personal   | `~/.claude/skills/<skill-name>/SKILL.md`                         | Todos sus proyectos                   |
| Proyecto   | `.claude/skills/<skill-name>/SKILL.md`                           | Solo este proyecto                    |
| Plugin     | `<plugin>/skills/<skill-name>/SKILL.md`                          | Donde el plugin está habilitado       |

Cuando los skills comparten el mismo nombre en diferentes niveles, enterprise anula personal, y personal anula proyecto. Un skill en cualquiera de estos niveles también anula un skill incluido con el mismo nombre. Por ejemplo, un skill `code-review` en el `.claude/skills/` de su proyecto reemplaza el `/code-review` incluido. Los skills de plugin utilizan un espacio de nombres `plugin-name:skill-name`, por lo que no pueden entrar en conflicto con otros niveles. Si tiene archivos en `.claude/commands/`, funcionan de la misma manera, pero si un skill y un comando comparten el mismo nombre, el skill tiene prioridad.

Los skills también se cargan desde directorios `.claude/skills/` anidados por debajo de su directorio de trabajo. Cuando Claude lee o edita un archivo en un subdirectorio, los skills del `.claude/skills/` de ese subdirectorio se vuelven disponibles. Esto permite que un paquete de monorepo proporcione sus propios skills que se apliquen cuando se trabaja en ese paquete, incluso si la sesión comenzó en la raíz del repositorio.

Si un skill anidado comparte un nombre con otro skill, ambos permanecen disponibles. Por ejemplo, con un skill `deploy` en la raíz del proyecto y otro en `apps/web/.claude/skills/`:

* El anidado aparece bajo un nombre calificado por directorio, `apps/web:deploy`.
* Su descripción dice a qué directorio se aplica.
* Claude elige la variante que coincide con los archivos en los que está trabajando.

Escribir `/deploy` ejecuta el skill de la raíz del proyecto. Escriba el nombre calificado `/apps/web:deploy` para ejecutar explícitamente la variante anidada.

Cuando usted o Claude invoca el nombre sin calificar, se carga el skill de la raíz del proyecto, y Claude Code añade una lista de las variantes calificadas por directorio a su contenido con una instrucción para también invocar cualquier variante cuyo directorio contenga los archivos en los que Claude está trabajando. Un skill anidado, por lo tanto, sigue siendo aplicable al trabajo en su directorio cuando solo se invoca el nombre sin calificar. Requiere Claude Code v2.1.203 o posterior.

Una entrada `<skill-name>` en las ubicaciones enterprise, personal o proyecto puede ser un enlace simbólico a un directorio en otro lugar del disco. Claude Code sigue el enlace simbólico y lee `SKILL.md` del directorio de destino, y si el mismo destino es accesible desde más de una ubicación, Claude Code carga el skill una sola vez. Los skills de plugin manejan los enlaces simbólicos de manera diferente; consulte [Compartir archivos dentro de un marketplace con enlaces simbólicos](/docs/es/plugins-reference#share-files-within-a-marketplace-with-symlinks).

<Note>
  Agregue un `.claude-plugin/plugin.json` a una carpeta de skill y se carga como un [plugin](/docs/es/plugins-reference#skills-directory-plugins) llamado `<name>@skills-dir`, por lo que puede agrupar agentes, hooks y servidores MCP. En un `.claude/skills/` de proyecto, esto requiere aceptar primero el diálogo de confianza del espacio de trabajo.
</Note>

<h4 id="live-change-detection">
  Detección de cambios en vivo
</h4>

Claude Code observa los directorios de skills para detectar cambios de archivos. Añadir, editar o eliminar un skill bajo `~/.claude/skills/`, el proyecto `.claude/skills/`, o un `.claude/skills/` dentro de un directorio `--add-dir` surte efecto dentro de la sesión actual sin reiniciar. Crear un directorio de skills de nivel superior que no existía cuando se inició la sesión requiere reiniciar Claude Code para que el nuevo directorio pueda ser observado.

<Note>
  La detección de cambios en vivo cubre solo el texto de `SKILL.md`. Para una carpeta de skill que también es un [plugin](/docs/es/plugins-reference#skills-directory-plugins), los cambios en `hooks/`, `.mcp.json`, `agents/` y `output-styles/` necesitan `/reload-plugins` para surtir efecto.
</Note>

<h4 id="automatic-discovery-from-parent-and-nested-directories">
  Descubrimiento automático desde directorios anidados y padres
</h4>

Los skills del proyecto se cargan desde `.claude/skills/` en su directorio de inicio y en cada directorio padre hasta la raíz del repositorio, por lo que iniciar Claude en un subdirectorio sigue recogiendo skills definidos en la raíz. Cuando trabaja con archivos en subdirectorios por debajo de su directorio de inicio, Claude Code también descubre skills desde directorios `.claude/skills/` anidados bajo demanda. Por ejemplo, si está editando un archivo en `packages/frontend/`, Claude Code también busca skills en `packages/frontend/.claude/skills/`. Esto admite configuraciones de monorepo donde los paquetes tienen sus propios skills.

Cada skill es un directorio con `SKILL.md` como punto de entrada:

```text theme={null}
my-skill/
├── SKILL.md           # Main instructions (required)
├── template.md        # Template for Claude to fill in
├── examples/
│   └── sample.md      # Example output showing expected format
└── scripts/
    └── validate.sh    # Script Claude can execute
```

El `SKILL.md` contiene las instrucciones principales y es obligatorio. Otros archivos son opcionales y le permiten crear skills más potentes: plantillas para que Claude las complete, salidas de ejemplo que muestren el formato esperado, scripts que Claude pueda ejecutar o documentación de referencia detallada. Haga referencia a estos archivos desde su `SKILL.md` para que Claude sepa qué contienen y cuándo cargarlos. Consulte [Añadir archivos de apoyo](#add-supporting-files) para más detalles.

<Note>
  Los archivos en `.claude/commands/` siguen funcionando y admiten el mismo [frontmatter](#frontmatter-reference). Los skills se recomiendan ya que admiten características adicionales como archivos de apoyo.
</Note>

<h4 id="skills-from-additional-directories">
  Skills de directorios adicionales
</h4>

La bandera `--add-dir` y el comando `/add-dir` [otorgan acceso a archivos](/docs/es/permissions#additional-directories-grant-file-access-not-configuration) en lugar de descubrimiento de configuración, pero los skills son una excepción: `.claude/skills/` dentro de un directorio añadido se carga automáticamente. Esta excepción se aplica solo a `--add-dir` y `/add-dir`. La configuración `permissions.additionalDirectories` en `settings.json` otorga solo acceso a archivos y no carga skills. Consulte [Detección de cambios en vivo](#live-change-detection) para ver cómo se detectan las ediciones durante una sesión.

Otra configuración de `.claude/` como comandos y estilos de salida no se carga desde directorios adicionales. Consulte la [tabla de excepciones](/docs/es/permissions#additional-directories-grant-file-access-not-configuration) para la lista completa de qué se carga y qué no, y las formas recomendadas de compartir configuración entre proyectos.

<Note>
  Los archivos CLAUDE.md de directorios `--add-dir` no se cargan de forma predeterminada. Para cargarlos, establezca `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1`. Consulte [Cargar desde directorios adicionales](/docs/es/memory#load-from-additional-directories).
</Note>

<h2 id="configure-skills">
  Configurar skills
</h2>

Los skills se configuran a través de frontmatter YAML en la parte superior de `SKILL.md` y el contenido markdown que sigue.

<h3 id="types-of-skill-content">
  Tipos de contenido de skill
</h3>

Los archivos de skill pueden contener cualquier instrucción, pero pensar en cómo desea invocarlos ayuda a guiar qué incluir:

**Contenido de referencia** añade conocimiento que Claude aplica a su trabajo actual. Convenciones, patrones, guías de estilo, conocimiento del dominio. Este contenido se ejecuta en línea para que Claude pueda usarlo junto con el contexto de su conversación.

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

**Contenido de tarea** da a Claude instrucciones paso a paso para una acción específica, como despliegues, commits o generación de código. Estas son a menudo acciones que desea invocar directamente con `/skill-name` en lugar de dejar que Claude decida cuándo ejecutarlas. Añada `disable-model-invocation: true` para evitar que Claude la active automáticamente.

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

Su `SKILL.md` puede contener cualquier cosa, pero pensar en cómo desea que se invoque el skill (por usted, por Claude, o ambos) y dónde desea que se ejecute (en línea o en un subagent) ayuda a guiar qué incluir. Para skills complejos, también puede [añadir archivos de apoyo](#add-supporting-files) para mantener el skill principal enfocado.

Mantenga el cuerpo en sí conciso. Una vez que se carga un skill, su contenido [permanece en contexto entre turnos](#skill-content-lifecycle), por lo que cada línea es un costo de token recurrente. Indique qué hacer en lugar de narrar cómo o por qué, y aplique la misma prueba de concisión que haría para [contenido de CLAUDE.md](/docs/es/best-practices#write-an-effective-claude-md).

<h3 id="frontmatter-reference">
  Referencia de frontmatter
</h3>

Más allá del contenido markdown, puede configurar el comportamiento del skill utilizando campos de frontmatter YAML entre marcadores `---` en la parte superior de su archivo `SKILL.md`:

```yaml theme={null}
---
name: my-skill
description: What this skill does
disable-model-invocation: true
allowed-tools: Read Grep
---

Your skill instructions here...
```

Todos los campos son opcionales. Solo se recomienda `description` para que Claude sepa cuándo usar el skill.

| Campo                      | Requerido   | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :------------------------- | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                     | No          | Nombre para mostrar mostrado en listados de skills. Por defecto es el nombre del directorio. Consulte [Cómo un skill obtiene su nombre de comando](#how-a-skill-gets-its-command-name) para ver cómo esto difiere del nombre que escribe para invocar el skill.                                                                                                                                                                                                                      |
| `description`              | Recomendado | Qué hace el skill y cuándo usarlo. Claude utiliza esto para decidir cuándo aplicar el skill. Si se omite, utiliza el primer párrafo del contenido markdown. Coloque el caso de uso clave primero: el texto combinado de `description` y `when_to_use` se trunca en 1.536 caracteres en el listado de skills para reducir el uso de contexto.                                                                                                                                         |
| `when_to_use`              | No          | Contexto adicional para cuándo Claude debe invocar el skill, como frases desencadenantes o solicitudes de ejemplo. Se añade a `description` en el listado de skills y cuenta hacia el límite de 1.536 caracteres.                                                                                                                                                                                                                                                                    |
| `argument-hint`            | No          | Sugerencia mostrada durante el autocompletado para indicar argumentos esperados. Ejemplo: `[issue-number]` o `[filename] [format]`.                                                                                                                                                                                                                                                                                                                                                  |
| `arguments`                | No          | Argumentos posicionales nombrados para [sustitución de `$name`](#available-string-substitutions) en el contenido del skill. Acepta una cadena separada por espacios o una lista YAML. Los nombres se asignan a posiciones de argumentos en orden.                                                                                                                                                                                                                                    |
| `disable-model-invocation` | No          | Establezca en `true` para evitar que Claude cargue automáticamente este skill. Utilice para flujos de trabajo que desea activar manualmente con `/name`. También evita que el skill sea [precargado en subagents](/docs/es/sub-agents#preload-skills-into-subagents). A partir de v2.1.196, también evita que el skill se ejecute cuando una [tarea programada](/docs/es/scheduled-tasks) se dispara con el skill como su prompt. Predeterminado: `false`.                                     |
| `user-invocable`           | No          | Establezca en `false` para ocultar del menú `/`. Utilice para conocimiento de fondo que los usuarios no deberían invocar directamente. Predeterminado: `true`.                                                                                                                                                                                                                                                                                                                       |
| `allowed-tools`            | No          | Herramientas que Claude puede usar sin pedir permiso cuando este skill está activo. Acepta una cadena separada por espacios o comas, o una lista YAML.                                                                                                                                                                                                                                                                                                                               |
| `disallowed-tools`         | No          | Herramientas eliminadas del grupo disponible de Claude mientras este skill está activo. Utilice para skills autónomos que nunca deben llamar a ciertas herramientas, como `AskUserQuestion` para un bucle de fondo. Acepta una cadena separada por espacios o comas, o una lista YAML. La restricción se borra cuando envía su siguiente mensaje.                                                                                                                                    |
| `model`                    | No          | Modelo a usar cuando este skill está activo. La anulación se aplica al resto del turno actual y no se guarda en la configuración; el modelo de sesión se reanuda en su siguiente prompt. Acepta los mismos valores que [`/model`](/docs/es/model-config), o `inherit` para mantener el modelo activo. Un valor excluido por la lista de permitidos [`availableModels`](/docs/es/model-config#restrict-model-selection) de su organización no se utiliza y la sesión mantiene su modelo actual. |
| `effort`                   | No          | [Nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) cuando este skill está activo. Anula el nivel de esfuerzo de la sesión. Predeterminado: hereda de la sesión. Opciones: `low`, `medium`, `high`, `xhigh`, `max`; los niveles disponibles dependen del modelo.                                                                                                                                                                                                               |
| `context`                  | No          | Establezca en `fork` para ejecutar en un contexto de subagent bifurcado.                                                                                                                                                                                                                                                                                                                                                                                                             |
| `agent`                    | No          | Qué tipo de subagent usar cuando `context: fork` está establecido.                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `hooks`                    | No          | Hooks limitados al ciclo de vida de este skill. Consulte [Hooks en skills y agents](/docs/es/hooks#hooks-in-skills-and-agents) para el formato de configuración.                                                                                                                                                                                                                                                                                                                          |
| `paths`                    | No          | Patrones glob que limitan cuándo se activa este skill. Acepta una cadena separada por comas o una lista YAML. Cuando se establece, Claude carga el skill automáticamente solo cuando trabaja con archivos que coinciden con los patrones. Utiliza el mismo formato que [reglas específicas de ruta](/docs/es/memory#path-specific-rules).                                                                                                                                                 |
| `shell`                    | No          | Shell a usar para `` !`command` `` y bloques ` ```! ` en este skill. Acepta `bash` (predeterminado) o `powershell`. Establecer `powershell` ejecuta comandos de shell en línea a través de PowerShell en Windows. Requiere `CLAUDE_CODE_USE_POWERSHELL_TOOL=1`.                                                                                                                                                                                                                      |

<h4 id="how-a-skill-gets-its-command-name">
  Cómo un skill obtiene su nombre de comando
</h4>

El comando que escribe para invocar un skill proviene de dónde vive el archivo de skill. El campo frontmatter `name` establece la etiqueta de visualización mostrada en listados de skills y, excepto para un `SKILL.md` de raíz de plugin, no cambia lo que escribe después de `/`.

La tabla a continuación muestra de dónde proviene el nombre del comando para cada diseño:

| Ubicación del skill                                                                                            | Fuente del nombre del comando                                                                     | Ejemplo                                                                                                                                               |
| :------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------- |
| Directorio de skill bajo `~/.claude/skills/` o `.claude/skills/`                                               | Nombre del directorio                                                                             | `.claude/skills/deploy-staging/SKILL.md` → `/deploy-staging`                                                                                          |
| [Directorio `.claude/skills/` anidado](#where-skills-live), cuando el nombre entra en conflicto con otro skill | Ruta del subdirectorio relativa al directorio de trabajo, luego el nombre del directorio de skill | `apps/web/.claude/skills/deploy/SKILL.md` → `/apps/web:deploy`                                                                                        |
| Archivo bajo `.claude/commands/`                                                                               | Nombre del archivo sin extensión                                                                  | `.claude/commands/deploy.md` → `/deploy`                                                                                                              |
| Subdirectorio `skills/` del plugin                                                                             | Nombre del directorio, con espacio de nombres por plugin                                          | `my-plugin/skills/review/SKILL.md` → `/my-plugin:review`                                                                                              |
| `SKILL.md` de raíz del plugin                                                                                  | Frontmatter `name`, con el nombre del directorio del plugin como alternativa                      | `my-plugin/SKILL.md` con `name: review` → `/my-plugin:review`. Consulte [Reglas de comportamiento de ruta](/docs/es/plugins-reference#path-behavior-rules) |

El caso de raíz del plugin es el único lugar donde `name` establece el nombre del comando, porque no hay directorio de skill del que tomarlo. Si `name` no se establece en el frontmatter, se utiliza el nombre del directorio del plugin en su lugar.

<h4 id="available-string-substitutions">
  Sustituciones de cadena disponibles
</h4>

Los skills admiten sustitución de cadena para valores dinámicos en el contenido del skill:

| Variable                | Descripción                                                                                                                                                                                                                                                                                                                                          |
| :---------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `$ARGUMENTS`            | Todos los argumentos pasados al invocar el skill. Si `$ARGUMENTS` no está presente en el contenido, los argumentos se añaden como `ARGUMENTS: <value>`.                                                                                                                                                                                              |
| `$ARGUMENTS[N]`         | Acceda a un argumento específico por índice basado en 0, como `$ARGUMENTS[0]` para el primer argumento.                                                                                                                                                                                                                                              |
| `$N`                    | Abreviatura para `$ARGUMENTS[N]`, como `$0` para el primer argumento o `$1` para el segundo.                                                                                                                                                                                                                                                         |
| `$name`                 | Argumento nombrado declarado en la lista de frontmatter [`arguments`](#frontmatter-reference). Los nombres se asignan a posiciones en orden, por lo que con `arguments: [issue, branch]` el marcador de posición `$issue` se expande al primer argumento y `$branch` al segundo.                                                                     |
| `${CLAUDE_SESSION_ID}`  | El ID de sesión actual. Útil para registro, creación de archivos específicos de sesión o correlación de salida de skill con sesiones.                                                                                                                                                                                                                |
| `${CLAUDE_EFFORT}`      | El nivel de esfuerzo actual: `low`, `medium`, `high`, `xhigh`, o `max`. Ultracode no es un nivel distinto e informa como `xhigh`. Utilice esto para adaptar instrucciones de skill a la configuración de esfuerzo activa.                                                                                                                            |
| `${CLAUDE_SKILL_DIR}`   | El directorio que contiene el archivo `SKILL.md` del skill. Para skills de plugin, este es el subdirectorio del skill dentro del plugin, no la raíz del plugin. Utilice esto en comandos de inyección bash para hacer referencia a scripts o archivos incluidos con el skill, independientemente del directorio de trabajo actual.                   |
| `${CLAUDE_PROJECT_DIR}` | El directorio raíz del proyecto. Esta es la misma ruta que [hooks](/docs/es/hooks#reference-scripts-by-path) y servidores MCP reciben como `CLAUDE_PROJECT_DIR`. Utilice esto para hacer referencia a scripts o archivos locales del proyecto, como `${CLAUDE_PROJECT_DIR}/.claude/hooks/helper.sh`, independientemente de dónde esté instalado el skill. |

La sustitución `${CLAUDE_PROJECT_DIR}` requiere Claude Code v2.1.196 o posterior. Se aplica tanto al cuerpo del skill como al frontmatter [`allowed-tools`](#frontmatter-reference), por lo que una regla de permiso como `Bash(${CLAUDE_PROJECT_DIR}/scripts/lint.sh *)` se resuelve a la misma ruta que usa el cuerpo del skill.

Los argumentos indexados utilizan entrecomillado de estilo shell, por lo que envuelva valores de varias palabras entre comillas para pasarlos como un único argumento. Por ejemplo, `/my-skill "hello world" second` hace que `$0` se expanda a `hello world` y `$1` a `second`. El marcador de posición `$ARGUMENTS` siempre se expande a la cadena de argumento completa tal como se escribió.

Para incluir un `$` literal antes de un dígito, `ARGUMENTS`, o un nombre de argumento declarado, como `$1.00` en prosa, escápelo con una barra invertida: `\$1.00`. Una barra invertida antes de cualquier otro `$` se deja sin cambios. Solo una barra invertida directamente antes del token lo escapa. Una barra invertida duplicada como `\\$1` deja ambas barras invertidas en su lugar, y `$1` sigue expandiéndose al valor del argumento.

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

Haga referencia a archivos de apoyo desde `SKILL.md` para que Claude sepa qué contiene cada archivo y cuándo cargarlo:

```markdown theme={null}
## Additional resources

- For complete API details, see [reference.md](reference.md)
- For usage examples, see [examples.md](examples.md)
```

<Tip>Mantenga `SKILL.md` por debajo de 500 líneas. Mueva material de referencia detallado a archivos separados.</Tip>

<h3 id="control-who-invokes-a-skill">
  Controlar quién invoca un skill
</h3>

De forma predeterminada, tanto usted como Claude pueden invocar cualquier skill. Puede escribir `/skill-name` para invocarlo directamente, y Claude puede cargarlo automáticamente cuando sea relevante para su conversación. Dos campos de frontmatter le permiten restringir esto:

* **`disable-model-invocation: true`**: Solo usted puede invocar el skill. Utilice esto para flujos de trabajo con efectos secundarios o que desea controlar el tiempo, como `/commit`, `/deploy` o `/send-slack-message`. No desea que Claude decida desplegar porque su código se ve listo.

* **`user-invocable: false`**: Solo Claude puede invocar el skill. Utilice esto para conocimiento de fondo que no es accionable como comando. Un skill `legacy-system-context` explica cómo funciona un sistema antiguo. Claude debe saber esto cuando sea relevante, pero `/legacy-system-context` no es una acción significativa para que los usuarios realicen.

Este ejemplo crea un skill de despliegue que solo usted puede activar. Si establece `disable-model-invocation: true`, Claude no puede ejecutar el skill automáticamente:

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

Aquí se muestra cómo los dos campos afectan la invocación y la carga de contexto:

| Frontmatter                      | Puede invocar | Claude puede invocar | Cuándo se carga en contexto                                                     |
| :------------------------------- | :------------ | :------------------- | :------------------------------------------------------------------------------ |
| (predeterminado)                 | Sí            | Sí                   | La descripción siempre en contexto, el skill completo se carga cuando se invoca |
| `disable-model-invocation: true` | Sí            | No                   | La descripción no está en contexto, el skill completo se carga cuando lo invoca |
| `user-invocable: false`          | No            | Sí                   | La descripción siempre en contexto, el skill completo se carga cuando se invoca |

<Note>
  En una sesión regular, las descripciones de skills se cargan en contexto para que Claude sepa qué está disponible, pero el contenido completo del skill solo se carga cuando se invoca. Los [subagents con skills precargados](/docs/es/sub-agents#preload-skills-into-subagents) funcionan de manera diferente: el contenido completo del skill se inyecta al inicio.
</Note>

<h3 id="skill-content-lifecycle">
  Ciclo de vida del contenido del skill
</h3>

Cuando usted o Claude invoca un skill, el contenido `SKILL.md` renderizado entra en la conversación como un único mensaje y permanece allí durante el resto de la sesión. Claude Code no vuelve a leer el archivo de skill en turnos posteriores, por lo que escriba la orientación que debe aplicarse durante una tarea como instrucciones permanentes en lugar de pasos únicos.

Cuando Claude vuelve a invocar un skill cuyo contenido renderizado es idéntico a la copia ya en contexto, Claude Code añade una nota breve de que el skill ya está cargado en lugar de una segunda copia del contenido. Cuando el contenido renderizado difiere, porque los argumentos cambiaron o un comando de [contexto dinámico](#inject-dynamic-context) produjo una nueva salida, Claude Code añade el contenido completo nuevamente. Antes de v2.1.202, cada re-invocación añadía otra copia completa de las instrucciones del skill.

[Auto-compactación](/docs/es/how-claude-code-works#when-context-fills-up) lleva skills invocados hacia adelante dentro de un presupuesto de tokens. Cuando la conversación se resume para liberar contexto, Claude Code vuelve a adjuntar la invocación más reciente de cada skill después del resumen, manteniendo los primeros 5.000 tokens de cada uno. Los skills reajustados comparten un presupuesto combinado de 25.000 tokens. Claude Code llena este presupuesto comenzando desde el skill invocado más recientemente, por lo que los skills más antiguos pueden eliminarse completamente después de la compactación si ha invocado muchos en una sesión.

Si un skill parece dejar de influir en el comportamiento después de la primera respuesta, el contenido generalmente sigue presente y el modelo está eligiendo otras herramientas o enfoques. Fortalezca la `description` del skill e instrucciones para que el modelo siga prefiriéndolo, o use [hooks](/docs/es/hooks) para aplicar comportamiento de manera determinista. Si el skill es grande o invocó varios otros después de él, vuelva a invocarlo después de la compactación para restaurar el contenido completo.

<h3 id="pre-approve-tools-for-a-skill">
  Pre-aprobar herramientas para un skill
</h3>

El campo `allowed-tools` otorga permiso para las herramientas enumeradas mientras el skill está activo, por lo que Claude puede usarlas sin solicitarle aprobación. No restringe qué herramientas están disponibles: cada herramienta sigue siendo invocable, y su [configuración de permisos](/docs/es/permissions) sigue rigiendo las herramientas que no están enumeradas.

Para skills verificados en el directorio `.claude/skills/` de un proyecto, `allowed-tools` entra en vigor después de que acepte el diálogo de confianza del espacio de trabajo para esa carpeta, igual que las reglas de permisos en `.claude/settings.json`. Revise los skills del proyecto antes de confiar en un repositorio, ya que un skill puede otorgarse a sí mismo acceso amplio a herramientas.

Este skill permite que Claude ejecute comandos git sin aprobación por uso cada vez que lo invoca:

```yaml theme={null}
---
name: commit
description: Stage and commit the current changes
disable-model-invocation: true
allowed-tools: Bash(git add *) Bash(git commit *) Bash(git status *)
---
```

Para eliminar herramientas del grupo disponible de Claude mientras un skill está activo, enumérelas en `disallowed-tools` en el frontmatter del skill. La restricción se borra cuando envía su siguiente mensaje. Para bloquear herramientas en todos los skills y prompts, añada reglas de denegación en su [configuración de permisos](/docs/es/permissions).

<h3 id="pass-arguments-to-skills">
  Pasar argumentos a skills
</h3>

Tanto usted como Claude pueden pasar argumentos al invocar un skill. Los argumentos están disponibles a través del marcador de posición `$ARGUMENTS`.

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

Cuando ejecuta `/fix-issue 123`, Claude recibe "Fix GitHub issue 123 following our coding standards..."

Si invoca un skill con argumentos pero el skill no incluye `$ARGUMENTS`, Claude Code añade `ARGUMENTS: <your input>` al final del contenido del skill para que Claude siga viendo lo que escribió.

También puede apilar varios skills al inicio de un mensaje. A partir de v2.1.199, escribir `/code-review /fix-issue 123` carga ambos skills y pasa el texto final `123` como `$ARGUMENTS` a cada uno de ellos. En versiones anteriores, solo el primer skill se cargaba y recibía `/fix-issue 123` como texto de argumento literal.

Claude Code expande el primer skill más hasta cinco más apilados después de él. La expansión se detiene en el primer token que no es un skill invocable en línea por el usuario, por lo que un skill que se ejecuta como un [subagent bifurcado](#run-skills-in-a-subagent) o uno cuyos argumentos pueden comenzar con un comando de barra, como `/loop`, también termina allí; ese token y todo lo que viene después se convierte en el texto de argumento para cada skill expandido.

Para acceder a argumentos individuales por posición, utilice `$ARGUMENTS[N]` o la forma más corta `$N`:

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one framework to another
---

Migrate the $ARGUMENTS[0] component from $ARGUMENTS[1] to $ARGUMENTS[2].
Preserve all existing behavior and tests.
```

Ejecutar `/migrate-component SearchBar React Vue` reemplaza `$ARGUMENTS[0]` con `SearchBar`, `$ARGUMENTS[1]` con `React` y `$ARGUMENTS[2]` con `Vue`. El mismo skill usando la abreviatura `$N`:

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one framework to another
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

La sintaxis `` !`<command>` `` ejecuta comandos de shell antes de que el contenido del skill se envíe a Claude. La salida del comando reemplaza el marcador de posición, por lo que Claude recibe datos reales, no el comando en sí.

Este skill resume una solicitud de extracción obteniendo datos de PR en vivo con la CLI de GitHub. Los comandos `` !`gh pr diff` `` y otros se ejecutan primero, y su salida se inserta en el prompt:

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

Cuando se ejecuta este skill:

1. Cada `` !`<command>` `` se ejecuta inmediatamente (antes de que Claude vea algo)
2. La salida reemplaza el marcador de posición en el contenido del skill
3. Claude recibe el prompt completamente renderizado con datos de PR reales

Esto es preprocesamiento, no algo que Claude ejecute. Claude solo ve el resultado final.

La sustitución se ejecuta una sola vez sobre el archivo original. La salida del comando se inserta como texto sin formato y no se vuelve a escanear para buscar más marcadores de posición `` !`<command>` ``, por lo que un comando no puede emitir un marcador de posición para que una pasada posterior lo expanda.

La forma en línea solo se reconoce cuando `!` aparece al inicio de una línea o inmediatamente después de espacios en blanco. Si `!` sigue a otro carácter, como en `` KEY=!`cmd` ``, el marcador de posición se deja como texto literal y el comando no se ejecuta.

Para comandos de varias líneas, utilice un bloque de código cercado abierto con ` ```! ` en lugar de la forma en línea:

````markdown theme={null}
## Environment
```!
node --version
npm --version
git status --short
```
````

Para deshabilitar este comportamiento para skills y comandos personalizados de fuentes de usuario, proyecto, plugin o [directorio adicional](#skills-from-additional-directories), establezca `"disableSkillShellExecution": true` en [settings](/docs/es/settings). Cada comando se reemplaza con `[shell command execution disabled by policy]` en lugar de ejecutarse. Los skills agrupados y gestionados no se ven afectados. Esta configuración es más útil en [managed settings](/docs/es/permissions#managed-settings), donde los usuarios no pueden anularla.

<Tip>
  Para solicitar un razonamiento más profundo cuando se ejecuta un skill, incluya `ultrathink` en cualquier lugar en el contenido del skill. Consulte [Use ultrathink for one-off deep reasoning](/docs/es/model-config#use-ultrathink-for-one-off-deep-reasoning).
</Tip>

<h3 id="run-skills-in-a-subagent">
  Ejecutar skills en un subagent
</h3>

Añada `context: fork` a su frontmatter cuando desee que un skill se ejecute en aislamiento. El contenido del skill se convierte en el prompt que impulsa el subagent. No tendrá acceso a su historial de conversación.

<Warning>
  `context: fork` solo tiene sentido para skills con instrucciones explícitas. Si su skill contiene directrices como "use estas convenciones de API" sin una tarea, el subagent recibe las directrices pero sin un prompt accionable, y regresa sin salida significativa.
</Warning>

Los skills y los [subagents](/docs/es/sub-agents) funcionan juntos en dos direcciones:

| Enfoque                     | Prompt del sistema           | Tarea                           | También carga                                        |
| :-------------------------- | :--------------------------- | :------------------------------ | :--------------------------------------------------- |
| Skill con `context: fork`   | Del tipo de agent            | Contenido de SKILL.md           | CLAUDE.md, excepto cuando el agent es Explore o Plan |
| Subagent con campo `skills` | Cuerpo markdown del subagent | Mensaje de delegación de Claude | Skills precargados + CLAUDE.md                       |

Con `context: fork`, escribe la tarea en tu skill y elige un tipo de agent para ejecutarla. Los agents integrados Explore y Plan [omiten CLAUDE.md y git status](/docs/es/sub-agents#what-loads-at-startup) para mantener su contexto pequeño, por lo que un skill bifurcado usando `agent: Explore` solo ve el contenido de SKILL.md y el prompt del sistema del agent. Para lo inverso, donde define un subagent personalizado que usa skills como material de referencia, consulte [Subagents](/docs/es/sub-agents#preload-skills-into-subagents).

<h4 id="example-research-skill-using-explore-agent">
  Ejemplo: Skill de investigación usando agent Explore
</h4>

Este skill ejecuta investigación en un agent Explore bifurcado. El contenido del skill se convierte en la tarea, y el agent proporciona herramientas de solo lectura optimizadas para exploración de base de código:

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

Cuando se ejecuta este skill:

1. Se crea un nuevo contexto aislado
2. El subagent recibe el contenido del skill como su prompt ("Research \$ARGUMENTS thoroughly...")
3. El campo `agent` determina el entorno de ejecución (modelo, herramientas y permisos)
4. Los resultados se resumen y se devuelven a su conversación principal

El campo `agent` especifica qué configuración de subagent usar. Las opciones incluyen agents integrados (`Explore`, `Plan`, `general-purpose`) o cualquier subagent personalizado de `.claude/agents/`. Si se omite, utiliza `general-purpose`.

<h3 id="restrict-claude’s-skill-access">
  Restringir el acceso de Claude a skills
</h3>

De forma predeterminada, Claude puede invocar cualquier skill que no tenga `disable-model-invocation: true` establecido. Los skills que definen `allowed-tools` otorgan a Claude acceso a esas herramientas sin aprobación por uso cuando el skill está activo. Su [configuración de permisos](/docs/es/permissions) sigue rigiendo el comportamiento de aprobación de línea base para todas las demás herramientas. Algunos comandos integrados también están disponibles a través de la herramienta Skill, incluyendo `/init`, `/review` y `/security-review`. Otros comandos integrados como `/compact` no lo están.

Tres formas de controlar qué skills puede invocar Claude:

**Deshabilitar todos los skills** negando la herramienta Skill en `/permissions`:

```text theme={null}
# Add to deny rules:
Skill
```

**Permitir o denegar skills específicos** usando [reglas de permisos](/docs/es/permissions):

```text theme={null}
# Allow only specific skills
Skill(commit)
Skill(review-pr *)

# Deny specific skills
Skill(deploy *)
```

Sintaxis de permisos: `Skill(name)` para coincidencia exacta, `Skill(name *)` para coincidencia de prefijo con cualquier argumento.

**Ocultar skills individuales** añadiendo `disable-model-invocation: true` a su frontmatter. Esto elimina el skill del contexto de Claude por completo.

<Note>
  El campo `user-invocable` solo controla la visibilidad del menú, no el acceso a la herramienta Skill. Utilice `disable-model-invocation: true` para bloquear la invocación programática.
</Note>

<h3 id="override-skill-visibility-from-settings">
  Anular la visibilidad del skill desde la configuración
</h3>

La configuración `skillOverrides` controla la visibilidad del skill desde su [settings](/docs/es/settings) en lugar del frontmatter del skill. Úselo para skills cuyo SKILL.md no desea editar, como los que se registran en un repositorio de proyecto compartido o proporcionados por un servidor MCP. El menú `/skills` lo escribe por usted: resalte un skill y presione `Space` para ciclar entre estados, luego `Enter` para guardar en `.claude/settings.local.json`.

Cada clave es un nombre de skill y cada valor es uno de cuatro estados:

| Valor                   | Listado para Claude  | En menú `/` |
| :---------------------- | :------------------- | :---------- |
| `"on"`                  | Nombre y descripción | Sí          |
| `"name-only"`           | Solo nombre          | Sí          |
| `"user-invocable-only"` | Oculto               | Sí          |
| `"off"`                 | Oculto               | Oculto      |

A partir de v2.1.199, `"off"` también oculta el skill de las listas de comandos anunciadas a clientes de [Remote Control](/docs/es/remote-control) y a llamadores de [Agent SDK](/docs/es/agent-sdk/slash-commands), no solo el menú `/` de terminal. Invocar un skill oculto por su nombre completo aún devuelve el error `skillOverrides` en lugar de ejecutarlo.

Un skill que está ausente de `skillOverrides` se trata como `"on"`. El ejemplo a continuación colapsa un skill a su nombre y desactiva otro por completo:

```json theme={null}
{
  "skillOverrides": {
    "legacy-context": "name-only",
    "deploy": "off"
  }
}
```

Los skills de plugin no se ven afectados por `skillOverrides`. Gestione esos a través de `/plugin` en su lugar.

<h2 id="evaluate-and-iterate-on-a-skill">
  Evaluar e iterar en un skill
</h2>

Ver que un skill se activa le dice que Claude lo encontró, no que hizo lo que pretendía. Para saber que un skill funciona, mida dos cosas por separado: si Claude lo invoca en los prompts que debería, y si la salida coincide con lo que espera cuando lo hace.

La verificación de ambos es una comparación de línea base. Recopile algunos prompts realistas, ejecute cada uno en una sesión nueva con el skill disponible y nuevamente con [deshabilitado](#override-skill-visibility-from-settings), y compare los resultados. Una sesión nueva es importante porque el contexto sobrante de la autoría del skill enmascarará las brechas en las instrucciones escritas.

<h3 id="run-evals-with-skill-creator">
  Ejecutar evals con skill-creator
</h3>

El [plugin `skill-creator`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/skill-creator) automatiza el bucle de comparación dentro de Claude Code. Instálelo desde el marketplace oficial:

```text theme={null}
/plugin install skill-creator@claude-plugins-official
```

Si Claude Code informa que el plugin no se encuentra en ningún marketplace, su marketplace está ausente o desactualizado. Ejecute `/plugin marketplace update claude-plugins-official` para actualizarlo, o `/plugin marketplace add anthropics/claude-plugins-official` si no lo ha añadido antes. Luego reintente la instalación.

Después de instalar, ejecute `/reload-plugins` para que los skills del plugin estén disponibles en la sesión actual. Luego pida a Claude que evalúe un skill existente, por ejemplo `evaluate my summarize-changes skill with skill-creator`. El plugin lo guía a través de la escritura de casos de prueba y ejecuta el bucle:

* **Casos de prueba**: almacena prompts, archivos de entrada y comportamiento esperado en `evals/evals.json` dentro del directorio de skill
* **Ejecuciones aisladas**: genera un [subagent](/docs/es/sub-agents) por caso de prueba para que cada ejecución comience con un contexto limpio, y registra el recuento de tokens y la duración
* **Calificación**: verifica cada aserción contra la salida y escribe aprobado o reprobado con evidencia en `grading.json`
* **Benchmark**: agrega la tasa de aprobación, tiempo y tokens para con-skill versus sin-skill en `benchmark.json` para que pueda comparar la mejora de la tasa de aprobación contra la sobrecarga de tokens y tiempo
* **Comparación de versiones**: ejecuta una prueba A/B ciega entre dos versiones del skill para que pueda confirmar que una edición es una mejora antes de confirmarla
* **Visor de revisión**: abre un informe HTML donde puede inspeccionar cada salida y registrar comentarios cualitativos que la siguiente iteración lee

Para el formato del archivo eval y el flujo de trabajo de iteración completo, consulte [Evaluating skill output quality](https://agentskills.io/skill-creation/evaluating-skills) en agentskills.io. Para obtener información sobre el benchmark y los modos de comparación, consulte el [anuncio de skill-creator](https://claude.com/blog/improving-skill-creator-test-measure-and-refine-agent-skills).

<h2 id="share-skills">
  Compartir skills
</h2>

Los skills se pueden distribuir en diferentes ámbitos dependiendo de su audiencia:

* **Skills de proyecto**: Confirme `.claude/skills/` en el control de versiones
* **Plugins**: Cree un directorio `skills/` en su [plugin](/docs/es/plugins)
* **Gestionado**: Implemente en toda la organización a través de [configuración gestionada](/docs/es/settings#settings-files)

<h3 id="generate-visual-output">
  Generar salida visual
</h3>

Los skills pueden agrupar y ejecutar scripts en cualquier idioma, dando a Claude capacidades más allá de lo que es posible en un único prompt. Un patrón poderoso es generar salida visual: archivos HTML interactivos que se abren en su navegador para explorar datos, depurar o crear informes.

Este ejemplo crea un explorador de base de código: una vista de árbol interactiva donde puede expandir y contraer directorios, ver tamaños de archivo de un vistazo e identificar tipos de archivo por color.

Cree el directorio Skill:

```bash theme={null}
mkdir -p ~/.claude/skills/codebase-visualizer/scripts
```

Guarde esto en `~/.claude/skills/codebase-visualizer/SKILL.md`. La descripción le dice a Claude cuándo activar este Skill, y las instrucciones le dicen a Claude que ejecute el script incluido. La ruta del script utiliza [`${CLAUDE_SKILL_DIR}`](#available-string-substitutions) para que se resuelva correctamente si el skill está instalado a nivel personal, de proyecto o de plugin:

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

El script requiere Python 3 pero utiliza solo bibliotecas integradas, por lo que no hay paquetes para instalar:

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

Para probar, abra Claude Code en cualquier proyecto y pregunte "Visualize this codebase." Claude ejecuta el script, genera `codebase-map.html` y lo abre en su navegador.

Este patrón funciona para cualquier salida visual: gráficos de dependencias, informes de cobertura de pruebas, documentación de API o visualizaciones de esquema de base de datos. El script incluido hace el trabajo pesado mientras Claude maneja la orquestación.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="skill-not-triggering">
  Skill no se activa
</h3>

Si Claude no usa su skill cuando se espera:

1. Verifique que la descripción incluya palabras clave que los usuarios dirían naturalmente
2. Verifique que el skill aparezca en `What skills are available?`
3. Intente reformular su solicitud para que coincida más estrechamente con la descripción
4. Invóquelo directamente con `/skill-name` si el skill es invocable por el usuario

Si el YAML del frontmatter está mal formado, Claude Code carga el cuerpo del skill con metadatos vacíos, por lo que `/skill-name` sigue funcionando pero Claude no tiene `description` para coincidir. Ejecute con `--debug` para ver el error de análisis.

<h3 id="skill-triggers-too-often">
  Skill se activa demasiado a menudo
</h3>

Si Claude usa su skill cuando no desea:

1. Haga la descripción más específica
2. Añada `disable-model-invocation: true` si solo desea invocación manual

<h3 id="skill-descriptions-are-cut-short">
  Las descripciones de skills se cortan
</h3>

Claude Code carga un listado de nombres de skills y descripciones en contexto para que Claude sepa qué está disponible. El listado siempre contiene todos los nombres de skills, pero si tiene muchos skills, Claude Code acorta las descripciones para ajustarse al presupuesto de caracteres del listado, lo que puede eliminar las palabras clave que Claude necesita para coincidir con su solicitud. El presupuesto se escala al 1% de la ventana de contexto del modelo. Cuando el listado se desborda, Claude Code elimina descripciones comenzando con los skills que invoca menos, por lo que los skills que usa más mantienen su texto completo.

Ejecute `/doctor` para obtener una estimación del costo de contexto del listado y sus mayores contribuyentes. Cuando el listado excede su presupuesto, Claude Code también escribe una advertencia en el registro de depuración, visible con [`--debug`](/docs/es/cli-reference#cli-flags).

La fila Skills en `/context` reporta el tamaño del listado después de que se aplica el presupuesto, por lo que coincide con lo que recibe el modelo. Antes de v2.1.196, la fila contaba el texto completo de cada descripción y podría mostrar un valor varias veces mayor que el presupuesto configurado.

Para aumentar el presupuesto, establezca la configuración [`skillListingBudgetFraction`](/docs/es/settings#available-settings) (por ejemplo, `0.02` = 2%) o la variable de entorno `SLASH_COMMAND_TOOL_CHAR_BUDGET` a un recuento de caracteres fijo. Para liberar presupuesto para otros skills, establezca las entradas de baja prioridad en `"name-only"` en [`skillOverrides`](#override-skill-visibility-from-settings) para que se enumeren sin descripción. También puede recortar el texto de `description` y `when_to_use` en la fuente: coloque el caso de uso clave primero, ya que el texto combinado de cada entrada está limitado a 1.536 caracteres independientemente del presupuesto. El límite es configurable con [`skillListingMaxDescChars`](/docs/es/settings#available-settings).

<h2 id="related-resources">
  Recursos relacionados
</h2>

* **[Depura tu configuración](/docs/es/debug-your-config)**: diagnostica por qué una skill no aparece o no se activa
* **[Evaluating skill output quality](https://agentskills.io/skill-creation/evaluating-skills)**: el formato del archivo eval y el flujo de trabajo de iteración en agentskills.io
* **[Skill authoring best practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices)**: orientación de escritura que se aplica en productos Claude
* **[Subagents](/docs/es/sub-agents)**: delega tareas a agents especializados
* **[Plugins](/docs/es/plugins)**: empaqueta y distribuye skills con otras extensiones
* **[Hooks](/docs/es/hooks)**: automatiza flujos de trabajo alrededor de eventos de herramientas
* **[Memory](/docs/es/memory)**: gestiona archivos CLAUDE.md para contexto persistente
* **[Comandos](/docs/es/commands)**: referencia para comandos integrados y skills agrupados
* **[Permisos](/docs/es/permissions)**: controla el acceso a herramientas y skills
* **[Claude Tag skills](https://claude.com/docs/claude-tag/admins/skills-repo)**: skills de proyecto comprometidas en un repositorio también se cargan cuando ese repositorio se utiliza en un canal de Claude Tag
