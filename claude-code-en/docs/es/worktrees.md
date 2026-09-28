> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ejecutar sesiones paralelas con worktrees

> Aisle sesiones paralelas de Claude Code en worktrees de git separados para que los cambios no colisionen. Cubre la bandera `--worktree`, aislamiento de subagentes, `.worktreeinclude`, limpieza y hooks de VCS no-git.

Un [git worktree](https://git-scm.com/docs/git-worktree) es un directorio de trabajo separado con sus propios archivos y rama, compartiendo el mismo historial de repositorio y remoto que su checkout principal. Ejecutar cada sesión de Claude Code en su propio worktree significa que las ediciones en una sesión nunca tocan archivos en otra, por lo que una sesión puede construir una característica mientras una segunda corrige un error.

<Note>
  Los worktrees requieren un repositorio de git; para otros sistemas de control de versiones, [configure hooks para reemplazar la lógica de git](#non-git-version-control). En la [aplicación de escritorio](/docs/es/desktop#work-in-parallel-with-sessions), seleccione la opción **worktree** cuando inicie una sesión para darle su propio worktree.
</Note>

Los worktrees son una de varias formas de ejecutar Claude en paralelo. Aíslan ediciones de archivos. Los [subagentes](/docs/es/sub-agents) dividen el trabajo dentro de una sesión, y la [mensajería entre sesiones](/docs/es/cross-session-messaging) permite que Claude pase hallazgos entre las sesiones en sus worktrees. Consulte [Ejecutar agentes en paralelo](/docs/es/agents) para comparar los enfoques, o salte directamente a [Aislar subagentes con worktrees](#isolate-subagents-with-worktrees) para usar worktrees y subagentes juntos.

La mayoría de sesiones necesitan solo las dos primeras secciones: [inicie Claude en un worktree](#start-claude-in-a-worktree), luego [limpie cuando salga](#clean-up-worktrees). Vuelva al resto de la página cuando necesite [reanudar una sesión](#resume-a-worktree-session), [cambiar cómo se crean los worktrees](#customize-worktree-creation), o [depurar un fallo](#troubleshooting).

<h2 id="start-claude-in-a-worktree">
  Inicie Claude en un worktree
</h2>

Pase `--worktree` o `-w` con un nombre para crear un worktree aislado e iniciar Claude en él. De forma predeterminada, el worktree se crea bajo `.claude/worktrees/<name>/` en la raíz de su repositorio, en una nueva rama llamada `worktree-<name>`:

```bash theme={null}
claude --worktree feature-auth
```

Ejecute el comando nuevamente con un nombre diferente en otra terminal para iniciar una segunda sesión aislada. Si omite el nombre, Claude genera uno como `bright-running-fox`.

Las ejecuciones interactivas requieren [confianza del espacio de trabajo](/docs/es/security): si no ha ejecutado Claude en el directorio antes, ejecute `claude` una vez allí para aceptar el diálogo de confianza, o `--worktree` sale con un error pidiéndole que lo haga. Las ejecuciones no interactivas con `-p` omiten la verificación de confianza, por lo que `claude -p --worktree` procede sin ella.

<Tip>
  Agregue `.claude/worktrees/` a su `.gitignore` para que el contenido del worktree no aparezca como archivos sin seguimiento en su checkout principal.
</Tip>

<h3 id="set-up-the-worktree-environment">
  Configure el entorno del worktree
</h3>

Un worktree es un checkout fresco, por lo que inicialice su entorno de desarrollo allí: pida a Claude que instale dependencias, o ejecute la configuración de su proyecto usted mismo en el directorio del worktree bajo `.claude/worktrees/`. Para llevar archivos ignorados por git como `.env` a cada nuevo worktree automáticamente, agregue un [archivo `.worktreeinclude`](#copy-gitignored-files-into-worktrees).

<h3 id="ask-claude-to-create-a-worktree">
  Pida a Claude que cree un worktree
</h3>

También puede pedirle a Claude que "trabaje en un worktree" durante una sesión, y creará uno con la herramienta [`EnterWorktree`](/docs/es/tools-reference). Una vez en un worktree, Claude puede cambiar directamente a otro bajo `.claude/worktrees/` llamando a `EnterWorktree` con la ruta de destino; el worktree anterior permanece en el disco sin cambios.

Cuando Claude entra en una ruta fuera del directorio `.claude/worktrees/` del repositorio, Claude Code solicita su aprobación primero, porque el movimiento toma el directorio de trabajo de la sesión, acceso de escritura, y configuración del proyecto como `CLAUDE.md` y configuración a esa ubicación. Una regla de [permiso](/docs/es/permissions) de `EnterWorktree` o elegir "no preguntar de nuevo" no suprime este aviso; solo el modo `bypassPermissions` lo omite. Antes de v2.1.206, Claude podía entrar en cualquier ruta de worktree existente sin preguntar.

<Note>
  **Las rutas de hooks no siguen el worktree.** Después de que Claude entra en un worktree, Claude Code mantiene `${CLAUDE_PROJECT_DIR}` en sus [hooks](/docs/es/hooks#reference-scripts-by-path) donde estaba y pasa la ruta del worktree a ellos de una manera diferente:

  * **`${CLAUDE_PROJECT_DIR}` permanece en su lugar**: todavía apunta a la raíz del proyecto donde comenzó la sesión, por lo que un comando de hook como `${CLAUDE_PROJECT_DIR}/.claude/hooks/check-style.sh` todavía ejecuta el script en el checkout principal.
  * **`cwd` sigue a Claude**: el campo `cwd` en el [JSON de entrada](/docs/es/hooks#common-input-fields) del hook es la raíz del worktree, y se mueve nuevamente cuando Claude ejecuta `cd`. Léalo cuando un hook necesite la ruta del worktree.
</Note>

<h2 id="clean-up-worktrees">
  Limpie worktrees
</h2>

Cuando sale de una sesión de worktree interactiva, Claude verifica el worktree para el trabajo que la eliminación eliminaría: archivos cambiados o sin seguimiento, trabajo sin confirmar dentro de submódulos extraídos y nuevos commits.

* **El worktree está limpio**: para una sesión sin nombre, Claude elimina el worktree y su rama automáticamente. Una sesión [nombrada](/docs/es/sessions#name-your-sessions) le solicita primero para que pueda mantener el worktree para más tarde
* **El worktree tiene trabajo en él**: Claude le solicita que mantenga o elimine el worktree. Mantener preserva el directorio y la rama para que pueda regresar más tarde. Eliminar borra el directorio del worktree y su rama, junto con todo el trabajo en ellos
* **El estado del worktree no se puede verificar**: cuando Claude Code no puede contar los cambios del worktree o no puede inspeccionar los checkouts de sus submódulos, le solicita en lugar de eliminar el worktree automáticamente. La solicitud nombra lo que no pudo verificar

Las ejecuciones no interactivas con `-p` no tienen solicitud de salida, por lo que Claude no limpia sus worktrees, y Claude Code deja el bloqueo que tomó en cada uno en la creación en su lugar hasta que una [limpieza de bloqueo obsoleto](#clean-up-subagent-and-background-session-worktrees) posterior lo libere. Para eliminar uno, ejecute `git worktree remove`; si git se niega porque el worktree está bloqueado, ejecute `git worktree unlock` en él primero.

En Windows, eliminar un worktree no elimina archivos fuera de él. Si una carpeta dentro del worktree es un enlace a otro lugar, como una unión NTFS o un enlace simbólico de directorio, Claude Code elimina solo el enlace y mantiene la carpeta a la que apunta. Antes de v2.1.205, eliminar un worktree con un enlace anidado en un subdirectorio podría eliminar la carpeta a la que apuntaba.

<h2 id="resume-a-worktree-session">
  Reanude una sesión de worktree
</h2>

Cuando reanuda una sesión que estaba dentro de un worktree, Claude Code devuelve la sesión a ese worktree. Esto se aplica a reanudaciones interactivas, a `--continue` y `--resume` en [modo no interactivo](/docs/es/headless) con `-p`, y al Agent SDK. De vuelta dentro del worktree, Claude aún puede salir de él con la herramienta [`ExitWorktree`](/docs/es/tools-reference).

Antes de devolver la sesión a su worktree, Claude Code verifica que el worktree siga siendo un checkout separado del principal, y se niega a reingresar a un worktree que falla la verificación. Para un git worktree, la verificación lee sus metadatos de git. Un worktree sin metadatos de git, como uno que un [hook `WorktreeCreate`](#non-git-version-control) creó, puede pasar la verificación; los casos que Claude Code aún se niega están listados con sus recuperaciones bajo [Claude Code se niega a usar un worktree](#claude-code-refuses-to-use-a-worktree). Para los mensajes y cómo recuperarse de cada uno, consulte [La sesión se reanuda fuera de su worktree](#the-session-resumes-outside-its-worktree).

Dónde lanza y cómo reanuda, cambian lo que Claude Code reingresa:

* **Directorio de lanzamiento**: reanude desde el checkout principal u otro directorio del repositorio. Claude Code reingresa a un worktree que creó con git bajo `.claude/worktrees/` incluso cuando lanza desde dentro de él. Cuando lanza desde dentro de cualquier otro worktree, Claude Code lo reingresa solo si puede respaldarlo desde allí: un worktree que es su propio repositorio, uno sin metadatos de git, o un lanzamiento desde un subdirectorio de un worktree que creó con `git worktree add` se niega, por lo que lance esos desde el checkout principal.
* **`--fork-session`**: la sesión bifurcada comienza en el directorio desde el que lanzó Claude, y Claude Code deja el worktree de la sesión original sin cambios.
* **Worktree eliminado**: si el directorio del worktree ya no existe, Claude Code reanuda la sesión en el directorio desde el que lanzó Claude. Le dice que el worktree se ha ido y borra el enlace del worktree de la sesión.

<Note>
  Antes de v2.1.212, una reanudación no interactiva permanecía en el directorio de inicio y `ExitWorktree` reportaba que no había una sesión de worktree activa para salir.
</Note>

Cuando Claude entra o sale de un worktree que Claude Code creó con git, la transcripción sigue: Claude Code registra la sesión bajo el nuevo directorio de trabajo de la sesión, de la misma manera que [`/cd`](/docs/es/commands) lo hace, por lo que `/desktop` y `--resume` la encuentran allí. Salir la mueve de la misma manera. Un worktree creado por un [hook `WorktreeCreate`](#non-git-version-control) mantiene su transcripción en el directorio de lanzamiento. Requiere Claude Code v2.1.198 o posterior.

<h2 id="how-claude-code-enforces-isolation">
  Cómo Claude Code aplica el aislamiento
</h2>

Mientras una sesión está aislada en un worktree, Claude Code bloquea las llamadas de herramientas que las verificaciones a continuación definen. Las mismas reglas se aplican si inició la sesión con `--worktree`, Claude entró en un worktree con `EnterWorktree`, o reanudó una sesión de worktree.

El mismo cumplimiento cubre cada subagente que Claude genera desde la sesión aislada. Se aplica si la sesión es interactiva o se ejecuta en el [fondo](/docs/es/agent-view#how-file-edits-are-isolated). Los [subagentes que se ejecutan en su propio worktree](#isolate-subagents-with-worktrees) llevan las mismas verificaciones. Su historial de versiones está bajo [Escribir archivos de subagentes](/docs/es/sub-agents#write-subagent-files).

Claude Code aplica cuatro verificaciones:

* **Ediciones de archivos**: Claude Code bloquea un `Edit`, `Write`, o `NotebookEdit` que apunta a una ruta en el checkout principal.
* **Directorio de trabajo del comando**: Claude Code bloquea un comando Bash, PowerShell, o Monitor cuyo directorio de trabajo se resuelve al checkout principal, o cuyo directorio de trabajo no puede verificar que permanece fuera de él.
* **Redirecciones de git**: Claude Code bloquea un comando Bash o Monitor que redirige git al checkout principal. La redirección puede venir a través de `git -C`, `--git-dir`, una variable `GIT_DIR` o `GIT_WORK_TREE`, o un `cd` al checkout principal antes de ejecutar git.
* **Forma del comando**: Claude Code bloquea un comando Bash o Monitor cuando no puede verificar desde el texto del comando que cualquier git que ejecute el comando permanece dentro del worktree. Eso sucede, por ejemplo, cuando el nombre del comando se calcula en tiempo de ejecución, cuando la sintaxis no se puede analizar, o cuando una expansión como `${!name}` o `${ command; }` podría ejecutar un comando que el texto no especifica. Claude Code le dice a Claude cómo reescribir el comando rechazado, como dividirlo en comandos simples y separados. No puede desactivar esta verificación.

Las verificaciones se aplican al repositorio desde el que lanzó Claude Code. También cubren el checkout principal desde el que está vinculado un worktree vinculado. Para comandos PowerShell, Claude Code aplica solo la verificación del directorio de trabajo.

Claude ve cada rechazo como un error de herramienta que nombra el worktree y dice cómo proceder. Para un comando rechazado, consulte [qué significa el mensaje de rechazo y cómo borrarlo](/docs/es/errors#command-blocked-by-the-worktree-isolation-checks).

<h2 id="isolate-subagents-with-worktrees">
  Aisle subagentes con worktrees
</h2>

Los subagentes pueden ejecutarse en sus propios worktrees para que las ediciones paralelas no entren en conflicto. Pida a Claude que "use worktrees para sus agentes", o hágalo permanente para un [subagente personalizado](/docs/es/sub-agents#supported-frontmatter-fields) agregando `isolation: worktree` a su frontmatter.

Este subagente en `.claude/agents/` siempre se ejecuta en su propio worktree:

```markdown theme={null}
---
name: refactorer
description: Applies mechanical refactors across many files
isolation: worktree
---

Apply the requested refactor across every affected file, then run the tests
and report the results.
```

Cada subagente obtiene un worktree temporal que Claude Code elimina automáticamente cuando el subagente termina sin cambios; un worktree con cambios permanece en el disco hasta que el [barrido periódico a continuación](#clean-up-subagent-and-background-session-worktrees) pueda eliminarlo sin perder trabajo.

Los worktrees de subagentes utilizan la misma [rama base](#choose-the-base-branch) que `--worktree`, por lo que se ramifican desde la rama predeterminada de su repositorio a menos que `worktree.baseRef` esté configurado en `"head"`.

<h3 id="clean-up-subagent-and-background-session-worktrees">
  Limpie worktrees de subagentes y sesiones en segundo plano
</h3>

Claude Code ejecuta un barrido periódico que elimina worktrees que Claude creó para subagentes y [sesiones en segundo plano](/docs/es/agent-view#how-file-edits-are-isolated) una vez que son más antiguos que su configuración [`cleanupPeriodDays`](/docs/es/settings-reference#cleanupperioddays), siguiendo las [reglas de barrido de retención](/docs/es/claude-directory#cleaned-up-automatically).

Cuando [envía al fondo](/docs/es/agent-view#send-the-session-to-the-background) una sesión `--worktree`, su worktree se convierte en un worktree de sesión en segundo plano que el barrido puede eliminar. El barrido deja un worktree en su lugar en estos casos:

* El worktree aún contiene trabajo: archivos cambiados o sin seguimiento, o commits no enviados.
* Un submódulo extraído en el worktree contiene archivos cambiados o sin seguimiento, o Claude Code no puede inspeccionar los submódulos del worktree. Esta verificación requiere Claude Code v2.1.274 o posterior.
* Uno de los [cuatro casos que también bloquean la creación de worktree](#git-lfs-content-is-missing-from-a-worktree-claude-code-created) se aplica: Claude Code no puede determinar qué controladores de filtro define la configuración del repositorio, o encuentra una configuración allí que no puede desactivar.
* El worktree pertenece a una sesión `--worktree` que no ha enviado al fondo, sin importar su antigüedad.
* Creó el worktree usted mismo con `git worktree add`, incluso si luego ejecutó una sesión `--worktree <name>` en él y envió esa sesión al fondo.

Claude Code escribe un marcador en los metadatos de git de cada worktree que crea con git, y el barrido mantiene cualquier worktree sin uno, incluido un worktree que un [hook `WorktreeCreate`](#non-git-version-control) creó. Antes de v2.1.246, el barrido no verificaba el marcador, y podría eliminar un worktree que creó usted mismo cuando un registro de sesión en segundo plano antiguo apuntaba a él.

Mientras un agente se está ejecutando, Claude Code mantiene un `git worktree lock` en su worktree para que la limpieza concurrente no pueda eliminarlo, y libera el bloqueo cuando el agente termina. Claude Code mantiene el mismo bloqueo en el worktree que creó para una sesión enviada al fondo mientras la sesión se ejecuta, por lo que el barrido deja el worktree en su lugar y `git worktree remove` se niega a eliminarlo.

El barrido también libera un bloqueo que Claude Code estableció para una sesión cuyo proceso ha salido, por lo que una sesión en segundo plano eliminada no deja su worktree permanentemente bloqueado. El barrido nunca libera un bloqueo que estableció usted mismo con `git worktree lock`. Antes de v2.1.210, un bloqueo dejado por una sesión eliminada permanecía en su lugar hasta que ejecutaba `git worktree unlock`.

Para limpiar un worktree que el barrido mantiene, ejecute `git worktree remove`, agregando `--force` si el worktree tiene cambios no confirmados o archivos sin seguimiento. Si git se niega porque el worktree está bloqueado, ejecute `git worktree unlock` en él primero.

<h2 id="customize-worktree-creation">
  Personalice la creación de worktrees
</h2>

Los valores predeterminados de Claude Code para crear worktrees cubren la mayoría de sesiones: los crea bajo `.claude/worktrees/`, los ramifica desde la rama predeterminada de su repositorio, y verifica solo archivos rastreados. Las opciones en esta sección cambian esos valores predeterminados.

<h3 id="choose-the-base-branch">
  Elija la rama base
</h3>

Los nuevos worktrees se ramifican desde la rama predeterminada del repositorio, por lo que la mayoría de sesiones no necesitan esta configuración. Establezca `worktree.baseRef` en [configuración](/docs/es/settings-reference#worktree) para ramificarse desde su trabajo actual en su lugar. La configuración acepta dos valores:

* `"fresh"` (predeterminado): ramifique desde la rama predeterminada del repositorio en el remoto, generalmente `main`, para que el worktree comience desde un árbol limpio que coincida con el remoto.
* `"head"`: ramifique desde su `HEAD` local actual, para que el worktree lleve sus commits no enviados y estado de rama de característica. Use esto cuando aisle subagentes que necesiten operar en trabajo en progreso. Dentro de un worktree, `"head"` se resuelve a ese `HEAD` del worktree, no al del checkout principal.

No puede establecer `worktree.baseRef` en un nombre de rama. Para iniciar un worktree desde una rama existente específica, [créelo con git directamente](#manage-worktrees-manually).

Para una base `"fresh"`, Claude Code mantiene `origin/HEAD` actual: cuando el repositorio no ha sido obtenido en las últimas 24 horas, obtiene la rama predeterminada, limitado a cinco segundos, y usa la ref almacenada localmente en caché si la obtención falla. Si no hay remoto configurado, u `origin/HEAD` no está almacenado en caché localmente y no se puede obtener, el worktree vuelve a su `HEAD` local actual. Antes de v2.1.208, un worktree fresco usaba lo que `origin/HEAD` ya estaba almacenado en caché localmente.

Este ejemplo hace que cada nuevo worktree se ramifique desde su trabajo actual:

```json theme={null}
{
  "worktree": {
    "baseRef": "head"
  }
}
```

<h3 id="branch-from-a-pull-request">
  Ramifique desde una solicitud de extracción
</h3>

Para ramificarse desde una solicitud de extracción o solicitud de fusión específica, pase `--worktree` el número prefijado con `#`, una URL de solicitud de extracción de GitHub, o una URL de solicitud de fusión de GitLab como `https://gitlab.com/group/repo/-/merge_requests/123`. Claude Code obtiene el commit de cabeza de ese cambio de `origin` y crea el worktree en `.claude/worktrees/pr-<number>`. Entrecomille el argumento para que su shell no trate `#` como el inicio de un comentario:

```bash theme={null}
claude --worktree "#1234"
```

Claude Code lee solo el número de la URL. Siempre obtiene de su remoto `origin` del repositorio, y elige la ruta de obtención por el host de `origin`:

* **github.com**: obtiene `pull/<number>/head`
* **gitlab.com**: obtiene `merge-requests/<number>/head`
* **GitHub Enterprise, GitLab autogestionado, u otro host**: intenta `pull/<number>/head` primero, luego `merge-requests/<number>/head`

Antes de v2.1.233, Claude Code aceptaba solo `#<number>` y URLs de solicitud de extracción de estilo GitHub para `--worktree`, y siempre obtenía `pull/<number>/head`.

<h3 id="copy-gitignored-files-into-worktrees">
  Copie archivos ignorados por git en worktrees
</h3>

Un worktree es un checkout fresco, por lo que archivos sin seguimiento como `.env` o `.env.local` de su repositorio principal no están presentes. Para copiarlos automáticamente cuando Claude crea un worktree, agregue un archivo `.worktreeinclude` a la raíz de su proyecto.

El archivo utiliza la sintaxis de `.gitignore`. Solo se copian los archivos que coinciden con un patrón y también están ignorados por git, por lo que los archivos rastreados nunca se duplican.

Si escribe un patrón que comienza con `**/` y los archivos que desea están dentro de un directorio que está ignorado por git en su totalidad, Claude Code los copia solo cuando ese directorio en sí coincide con el patrón, o cuando el primer nombre después de `**/` es uno de los nombres en la ruta del directorio. Por ejemplo, si escribe `**/.claude/skills/*.md`, ese primer nombre es `.claude`, por lo que Claude Code copia los archivos coincidentes de un directorio `.claude/` ignorado. Para copiar archivos de un directorio ignorado que un patrón `**/` no alcanza, nombre el directorio en el patrón en su lugar: escriba `vendor/**/config.json` en lugar de `**/config.json`. Antes de v2.1.239, Claude Code copiaba archivos de un directorio completamente ignorado para un patrón `**/` solo cuando el directorio en sí coincidía con el patrón.

Este `.worktreeinclude` copia dos archivos env y una configuración de secretos en cada nuevo worktree:

```text .worktreeinclude theme={null}
.env
.env.local
config/secrets.json
```

Esto se aplica a cada worktree que Claude Code crea con git: worktrees `--worktree`, [worktrees de subagentes](#isolate-subagents-with-worktrees), y sesiones paralelas en la [aplicación de escritorio](/docs/es/desktop#work-in-parallel-with-sessions). Con un [hook `WorktreeCreate`](#non-git-version-control), copie los archivos dentro del script del hook.

<h3 id="reuse-a-worktree-name">
  Reutilice un nombre de worktree
</h3>

Pasar `--worktree` un nombre cuyo directorio ya existe abre ese worktree existente en lugar de crear uno nuevo.

Con la [base](#choose-the-base-branch) predeterminada `"fresh"`, un worktree reabierto se reinicia a la rama predeterminada del repositorio en lugar de continuar en su punta anterior cuando se cumplen todas las siguientes condiciones:

* No tiene cambios sin confirmar ni archivos sin seguimiento.
* Todavía está en la rama que Claude Code creó para él.
* No tiene commits propios, o su solicitud de extracción o solicitud de fusión fue fusionada y su rama remota fue eliminada.

Claude Code detecta el caso fusionado solo desde el estado de git: la rama remota a la que se envió el worktree ya no existe, y cada commit en el worktree ya está en la rama predeterminada.

En todos los otros casos, Claude Code reabre el worktree en su punta anterior:

* El worktree falla cualquiera de las condiciones.
* Claude Code no puede verificar el estado del worktree.
* `worktree.baseRef` es `"head"`.
* El nombre es una referencia de solicitud de extracción o solicitud de fusión.

Antes de v2.1.208, cuando reutilizaba un nombre, Claude Code siempre reabrió el worktree anterior en su punta anterior.

<h3 id="replace-worktree-creation-with-a-hook">
  Reemplace la creación de worktree con un hook
</h3>

Configure un [hook `WorktreeCreate`](/docs/es/hooks#worktreecreate) para reemplazar completamente la lógica predeterminada de `git worktree`, incluida la colocación de worktrees en algún lugar que no sea `.claude/worktrees/`. Para un ejemplo completo, consulte [Control de versiones no-git](#non-git-version-control).

<h2 id="what-worktrees-share-with-the-main-checkout">
  Qué comparten los worktrees con el checkout principal
</h2>

Un worktree obtiene sus propios archivos y rama, pero comparte lo siguiente con el checkout principal:

* **El directorio `.git` del repositorio**: los comandos de git en un worktree escriben en el directorio `.git` compartido del repositorio principal, y el [sandboxing](/docs/es/sandboxing#filesystem-isolation) permite esas escrituras, por lo que comandos como `git commit` funcionan desde dentro de un worktree con el sandbox habilitado.
* **Plugins**: los plugins instalados en [ámbito de proyecto](/docs/es/plugins/loading#find-where-a-plugin-is-enabled) desde el checkout principal también se cargan en worktrees del mismo repositorio, por lo que no necesita reinstalarlos por worktree. Requiere Claude Code v2.1.200 o posterior.
* **Aprobaciones de permisos**: elegir "Sí, y no preguntar de nuevo" para un comando Bash en una sesión de worktree guarda la regla en el `.claude/settings.local.json` del checkout principal, por lo que se aplica en el checkout principal y en cada otro worktree del repositorio, y sobrevive a la eliminación del worktree. En Windows y en los otros casos donde Claude Code [no usa la raíz del repositorio](/docs/es/settings#where-claude-code-looks-for-each-file), la regla permanece con ese worktree. Antes de v2.1.211, una aprobación otorgada en un worktree se guardaba dentro de ese worktree, no se aplicaba en otro lugar, y se perdía cuando se eliminaba el worktree. Consulte [dónde se guardan las aprobaciones](/docs/es/permissions#permission-system).
* **Skills, agentes y comandos sin seguimiento**: cuando el checkout del worktree no tiene un directorio `.claude/skills` en su raíz, por ejemplo porque su `.claude/skills` está en gitignore, Claude Code carga los [skills de proyecto](/docs/es/skills#where-skills-live) del checkout principal en la sesión del worktree. En un worktree con su propio directorio `.claude/skills`, solo se carga esa copia.

  La misma lectura de paso cubre `.claude/agents` y `.claude/commands`. Para skills, la lectura de paso requiere Claude Code v2.1.277 o posterior.

Todos estos se aplican si crea el worktree con `--worktree`, con `git worktree add`, o a través de la [aplicación de escritorio](/docs/es/desktop#work-in-parallel-with-sessions).

<h2 id="manage-worktrees-manually">
  Administre worktrees manualmente
</h2>

Cree worktrees con Git directamente cuando necesite verificar una rama existente específica o colocar el worktree fuera del repositorio.

Cree un worktree en una nueva rama:

```bash theme={null}
git worktree add ../project-feature-a -b feature-a
```

Cree un worktree desde una rama existente, reemplazando `fix-issue-456` con una rama que ya existe en su repositorio:

```bash theme={null}
git worktree add ../project-bugfix fix-issue-456
```

Inicie Claude en el worktree:

```bash theme={null}
cd ../project-feature-a
claude
```

Liste sus worktrees:

```bash theme={null}
git worktree list
```

Elimine uno cuando haya terminado con él:

```bash theme={null}
git worktree remove ../project-feature-a
```

Consulte la [documentación de git worktree](https://git-scm.com/docs/git-worktree) para la referencia completa de comandos.

<h2 id="non-git-version-control">
  Control de versiones no-git
</h2>

El aislamiento de worktree usa git de forma predeterminada. Para SVN, Perforce, Mercurial u otros sistemas, configure [hooks `WorktreeCreate` y `WorktreeRemove`](/docs/es/hooks#worktreecreate) para proporcionar lógica de creación y limpieza personalizada. Debido a que el hook reemplaza el comportamiento predeterminado de git, [`.worktreeinclude`](#copy-gitignored-files-into-worktrees) no se procesa cuando usa `--worktree`. Copie cualquier archivo de configuración local dentro de su script de hook en su lugar.

Este hook `WorktreeCreate` lee el nombre del worktree desde JSON en stdin con `jq`, verifica una copia de trabajo fresca de SVN, e imprime la ruta del directorio para que Claude Code pueda usarla como el directorio de trabajo de la sesión. Agregue la configuración a su [`settings.json`](/docs/es/settings#where-settings-live):

```json theme={null}
{
  "hooks": {
    "WorktreeCreate": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'NAME=$(jq -r .name); DIR=\"$HOME/.claude/worktrees/$NAME\"; svn checkout https://svn.example.com/repo/trunk \"$DIR\" >&2 && echo \"$DIR\"'"
          }
        ]
      }
    ]
  }
}
```

Emparéjelo con un hook `WorktreeRemove` para limpiar cuando la sesión termina. Consulte la [referencia de hooks](/docs/es/hooks#worktreecreate) para el esquema de entrada y un ejemplo de eliminación.

Un hook `WorktreeCreate` también le permite ejecutar [`/batch`](/docs/es/commands#all-commands) fuera de un repositorio git. Cada subagente `/batch` luego publica su cambio con los comandos de control de versiones de su proyecto y, cuando no puede abrir una solicitud de extracción, informa qué publicó en su lugar. Ejecutar `/batch` fuera de un repositorio git requiere Claude Code v2.1.281 o posterior.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

Claude Code reporta los errores a continuación cuando crea un worktree, entra en uno al iniciar, o devuelve una sesión reanudada a uno.

<h3 id="claude-code-can’t-enter-the-worktree-at-startup">
  Claude Code no puede entrar en el worktree al iniciar
</h3>

Cuando Claude Code no puede entrar en el directorio del worktree al iniciar, imprime un error nombrando la ruta y sale con código 1. Esto puede suceder cuando un [hook `WorktreeCreate`](/docs/es/hooks#worktreecreate) imprime algo que no sea el directorio que creó, o cuando el directorio fue eliminado después de que fue configurado.

<h3 id="worktree-creation-fails-on-a-symlinked-path">
  La creación de worktree falla en una ruta con enlace simbólico
</h3>

Claude Code se niega a crear un worktree cuando `.claude`, `.claude/worktrees`, o el directorio del worktree en sí es un enlace simbólico, y el error nombra la ruta con enlace simbólico. Elimine el enlace simbólico e intente de nuevo. Antes de v2.1.212, si el repositorio ya contenía un enlace simbólico confirmado en una de esas rutas, la creación de worktree lo seguía y podría crear archivos fuera del repositorio.

<h3 id="git-lfs-content-is-missing-from-a-worktree-claude-code-created">
  Los archivos Git LFS son archivos de puntero en un worktree que Claude Code creó
</h3>

Si configura [Git LFS](https://git-lfs.com) con `git lfs install --local`, un worktree que Claude Code crea contiene archivos de puntero de LFS en lugar de los archivos reales. La bandera `--local` escribe el filtro de LFS en el `.git/config` del repositorio en lugar de su configuración de git global. Un `git lfs install` simple escribe en su configuración global y no se ve afectado. Lo mismo se aplica a cualquier otro [controlador de filtro](https://git-scm.com/docs/gitattributes) definido en la configuración del repositorio en sí.

Claude Code omite los controladores de filtro del repositorio en sí cuando crea un worktree porque un controlador de filtro es un comando de shell, y cualquier cosa que pueda escribir en el repositorio, incluido Claude, podría haber puesto uno allí. Antes de v2.1.247, Claude Code ejecutaba esos controladores durante la creación de worktree.

Para obtener los archivos reales, ejecute `git lfs pull` dentro del worktree.

En cuatro casos raros, Claude Code no crea ningún worktree en absoluto: no puede determinar qué controladores de filtro define la configuración del repositorio, o encuentra una configuración allí que no puede desactivar. Haga coincidir el error con su corrección:

* **`Could not read the repository git config to neutralize filter drivers`**: Claude Code no pudo leer el `.git/config` del repositorio, por ejemplo debido a sus permisos. Corrija eso e intente de nuevo.
* **`The repository git config defines a filter driver whose name cannot be neutralized (contains "=" or a newline)`**: cambie el nombre o elimine ese controlador de filtro en `.git/config` e intente de nuevo.
* **`The repository git config has a conditional include (includeIf)`**: mueva la configuración que el `includeIf` en `.git/config` extrae directamente a ese archivo, elimine el `includeIf`, e intente de nuevo. Un `includeIf` en su configuración de git global no activa esto.
* **`Git was not run: the repository's own git config sets <key>`**: el mensaje nombra una clave que apunta Git LFS a un programa para ejecutar, como `lfs.customtransfer.<name>.path` o `lfs.standalonetransferagent`. Si esa configuración es suya, muévala a su configuración de git global. Si no la reconoce, elimínela de la configuración de git del repositorio, ya que una herramienta o checkout que no confía puede haberla escrito. Reintente una vez que la clave se haya ido de la configuración del repositorio.

<h3 id="claude-code-refuses-to-use-a-worktree">
  Claude Code se niega a usar un worktree
</h3>

Un error que comienza con `Refusing to use <path> as an isolation worktree` significa que Claude Code verificó la identidad de git del directorio antes de adoptarlo como un checkout aislado de sesión o subagente, y se negó. La verificación se ejecuta si Claude Code está creando el worktree, entrando en uno existente, o reutilizando uno de una ejecución anterior.

En la mayoría de casos, el resto del mensaje dice que los metadatos de git del directorio se resuelven en el checkout principal: por ejemplo, su archivo `.git` apunta al directorio `.git` del repositorio principal en sí, o git resuelve su árbol de trabajo al checkout principal a través de una redirección `core.worktree`. Desde tal directorio, un comando de git ordinario como `git reset --hard` actuaría en el checkout principal en lugar del worktree. Claude Code también se niega cuando el directorio tiene una entrada `.git` que no puede leer, en lugar de asumir que el worktree es seguro.

Un directorio sin metadatos de git en absoluto, como uno que su [hook `WorktreeCreate`](#non-git-version-control) crea, pasa la verificación solo cuando ningún repositorio de git lo contiene. Si el hook crea el directorio dentro de un repositorio, git lo resuelve al checkout de ese repositorio y Claude Code se niega con el mensaje `git resolves its working tree to`, así que haga que el hook cree sus directorios fuera de cualquier repositorio.

Claude Code deja el directorio rechazado en su lugar, ya que puede contener trabajo. Haga coincidir el mensaje con su recuperación, si sigue a `Refusing to use <path>` o aparece en un [mensaje de reanudación](#the-session-resumes-outside-its-worktree); algunos finales ocurren solo en mensajes de reanudación:

* **Dice `launch from the parent checkout` o `Run the resume from the project checkout`**: lanzó Claude Code desde dentro del worktree. Lance desde el checkout principal en su lugar; el worktree no necesita recreación.
* **Dice `it cannot be resumed or re-entered`**: nada en esta sesión respalda el worktree desde donde lanzó. Recréelo; el directorio y su trabajo permanecen en el disco para recuperación manual, y cuando el worktree tiene un checkout principal, reanudar desde allí también funciona.
* **Dice `it contains the protected checkout`**: el directorio rechazado es un padre de su checkout principal, como su directorio de inicio. No lo elimine. Cambie la ruta del worktree, como la ruta que devuelve su hook `WorktreeCreate` o el destino de `EnterWorktree`, para que el worktree no contenga el checkout.
* **Dice `the protected checkout <path> has a .git entry that could not be examined` o `has git metadata that could not be resolved`**: el problema es los metadatos de git del checkout principal, no los del worktree. No elimine el worktree, e ignore el consejo final del mensaje para recrearlo, que no se aplica a estos dos finales. Repare el checkout principal, por ejemplo un problema de permisos o un rechazo de git `dubious ownership` en su `.git`, e intente de nuevo.
* **Dice `its recorded path has a network spelling`**: Claude Code nunca reanuda en un worktree en una ruta de red. Recree el worktree en una ruta local.
* **Cualquier otro final**: el mensaje nombra el problema y su corrección, como eliminar una redirección `core.worktree` o recrear el worktree; sígalo. Antes de eliminar un directorio cuyo mensaje dice que su identidad de git no pudo ser verificada, aborde la causa nombrada primero, por ejemplo un enlace simbólico en la ruta del worktree o git en sí fallando al ejecutarse, ya que el directorio puede estar saludable. Cuando recree, rescate cualquier cambio que necesite del directorio anterior primero; permanece en el disco.

<h3 id="the-session-resumes-outside-its-worktree">
  La sesión se reanuda fuera de su worktree
</h3>

Cuando reanuda una sesión de forma interactiva y Claude Code no puede devolverla a su worktree, Claude Code lo dice con uno de los mensajes a continuación. Cuando Claude Code borra el enlace del worktree, registra la limpieza en la transcripción de la sesión. Si [suprime escrituras de transcripción](/docs/es/sessions#where-transcripts-are-stored), el mensaje dice en su lugar que el enlace no pudo ser borrado y que Claude Code volverá a verificar el worktree en una reanudación posterior.

| El mensaje comienza con                           | Qué sucedió y qué hacer                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Your worktree <path> no longer exists`           | El directorio del worktree fue eliminado. La sesión continúa en el directorio actual sin aislamiento, y Claude Code borra el enlace del worktree. No se requiere acción.                                                                                                                                                                                                                                                                                                                    |
| `Could not verify your worktree <path> this time` | Claude Code no pudo verificar el worktree, generalmente por una razón transitoria; el enlace se mantiene, y la sesión continúa en el directorio actual sin aislamiento. Reanude de nuevo para reintentar; si sigue sucediendo, entre en el worktree en una nueva sesión y haga coincidir el mensaje de rechazo bajo [Claude Code se niega a usar un worktree](#claude-code-refuses-to-use-a-worktree), que puede nombrar los metadatos del checkout principal en lugar de los del worktree. |
| `Did not re-enter your worktree <path>`           | Claude Code rechazó el enlace del worktree como inseguro; borra el enlace y la sesión continúa sin aislamiento. El mensaje incluye el rechazo específico: haga coincidir bajo [Claude Code se niega a usar un worktree](#claude-code-refuses-to-use-a-worktree), ya que la corrección es recreación para algunos rechazos y un cambio de ruta para otros.                                                                                                                                   |
| `Could not re-enter your worktree <path>`         | Claude Code no pudo respaldar el worktree desde donde lanzó, más comúnmente porque lanzó desde dentro de él; el enlace se mantiene. El resto del mensaje nombra la corrección; haga coincidir bajo [Claude Code se niega a usar un worktree](#claude-code-refuses-to-use-a-worktree).                                                                                                                                                                                                       |

En [modo no interactivo](/docs/es/headless) con `-p`, y en reanudaciones que el [Agent SDK](/docs/es/agent-sdk/sessions) ejecuta, Claude Code detiene la reanudación con un error stderr para cada rechazo excepto un worktree desaparecido, en lugar de continuar sin aislamiento.

Con `--output-format stream-json`, el rechazo también llega en stdout como un mensaje `result` con subtipo `error_during_execution` cuyo array `errors` lleva el mismo texto, así que una aplicación Agent SDK recibe la razón en lugar de solo una salida distinta de cero. Antes de v2.1.260, un rechazo de reanudación de worktree no producía ningún mensaje `result`.

Los mensajes toman formas diferentes de los mensajes interactivos en la tabla:

* `Error: cannot resume into worktree <path>: ...This session was not started.` para un rechazo que la tabla muestra como `Did not re-enter`. Claude Code borra el enlace del worktree antes de salir, y el error lo dice; la próxima vez que reanude la conversación, la sesión continúa en el directorio actual sin aislamiento de worktree. Antes de v2.1.260, Claude Code no escribía el enlace borrado, así que cada reintento de la misma reanudación fallaba con el mismo error.

  Si [suprime escrituras de transcripción](/docs/es/sessions#where-transcripts-are-stored), la limpieza no puede ser guardada. El error entonces dice que el mismo comando será rechazado de nuevo, y nombra `--fork-session` e iniciar una nueva conversación como formas de continuar sin el worktree.
* `Error: could not verify worktree <path> for this resume, so the resume was aborted...` para `Could not verify`
* `Error: ...The worktree binding is kept.` para `Could not re-enter`
* `Notice: the worktree <path> for this session no longer exists...` para un worktree desaparecido; Claude Code lo imprime y continúa la sesión, como lo hace una reanudación interactiva

El final de rechazo incrustado en cada error se comparte con los avisos interactivos, así que aún coincide con su entrada bajo [Claude Code se niega a usar un worktree](#claude-code-refuses-to-use-a-worktree).

En el resultado stream-json, [`startup_failure_reason`](/docs/es/agent-sdk/typescript#startup_failure_reason) es `worktree_unverified` para el error `could not verify worktree` y `worktree_resume_refused` para los errores `cannot resume into worktree` y `The worktree binding is kept`. Una aplicación puede ramificarse en él en lugar de hacer coincidir el texto del error. Antes de v2.1.274, el resultado no llevaba ningún campo `startup_failure_reason`.

<h2 id="see-also">
  Véase también
</h2>

Los worktrees manejan el aislamiento de archivos. Las páginas relacionadas a continuación cubren la delegación de trabajo en esos checkouts aislados, el paso de hallazgos entre ellos, y el cambio entre las sesiones que crea:

* [Subagentes](/docs/es/sub-agents): delegue trabajo a agentes aislados dentro de una sesión
* [Mensajería entre sesiones](/docs/es/cross-session-messaging): deje que las sesiones en sus worktrees pasen hallazgos entre sí
* [Equipos de agentes](/docs/es/agent-teams): coordine múltiples sesiones de Claude automáticamente
* [Administrar sesiones](/docs/es/sessions): nombre, reanude y cambie entre conversaciones
* [Sesiones paralelas de escritorio](/docs/es/desktop#work-in-parallel-with-sessions): sesiones respaldadas por worktree en la aplicación de escritorio
