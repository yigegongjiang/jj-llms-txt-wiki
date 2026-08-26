> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ejecutar sesiones paralelas con worktrees

> Aisle sesiones paralelas de Claude Code en worktrees de git separados para que los cambios no colisionen. Cubre la bandera `--worktree`, aislamiento de subagentes, `.worktreeinclude`, limpieza y hooks de VCS no-git.

Un [git worktree](https://git-scm.com/docs/git-worktree) es un directorio de trabajo separado con sus propios archivos y rama, compartiendo el mismo historial de repositorio y remoto que su checkout principal. Ejecutar cada sesión de Claude Code en su propio worktree significa que las ediciones en una sesión nunca tocan archivos en otra, por lo que puede tener a Claude construyendo una característica en una terminal mientras corrige un error en una segunda.

Esta página cubre el aislamiento de worktree en la CLI. Todo lo siguiente asume un repositorio de git. Para otros sistemas de control de versiones, consulte [Control de versiones no-git](#non-git-version-control). La [aplicación de escritorio](/docs/es/desktop#work-in-parallel-with-sessions) crea un worktree para cada nueva sesión automáticamente.

Los worktrees son una de varias formas de ejecutar Claude en paralelo. Aíslan ediciones de archivos, mientras que [subagentes](/docs/es/sub-agents) y [equipos de agentes](/docs/es/agent-teams) coordinan el trabajo en sí. Consulte [Ejecutar agentes en paralelo](/docs/es/agents) para comparar los enfoques, o salte directamente a [Aislar subagentes con worktrees](#isolate-subagents-with-worktrees) para usar worktrees y subagentes juntos.

<h2 id="start-claude-in-a-worktree">
  Inicie Claude en un worktree
</h2>

Pase `--worktree` o `-w` para crear un worktree aislado e iniciar Claude en él. De forma predeterminada, el worktree se crea bajo `.claude/worktrees/<value>/` en la raíz de su repositorio, en una nueva rama llamada `worktree-<value>`:

```bash theme={null}
claude --worktree feature-auth
```

Para poner worktrees en otro lugar, configure un [hook `WorktreeCreate`](#non-git-version-control). Ejecute el comando nuevamente con un nombre diferente en otra terminal para iniciar una segunda sesión aislada:

```bash theme={null}
claude --worktree bugfix-123
```

Si omite el nombre, Claude genera uno como `bright-running-fox`:

```bash theme={null}
claude --worktree
```

También puede pedirle a Claude que "trabaje en un worktree" durante una sesión, y creará uno con la herramienta [`EnterWorktree`](/docs/es/tools-reference). Una vez en un worktree, Claude puede cambiar directamente a otro bajo `.claude/worktrees/` llamando a `EnterWorktree` con la ruta de destino. El worktree anterior permanece en el disco sin cambios.

Entrar en una ruta fuera del directorio `.claude/worktrees/` del repositorio solicita su aprobación primero, porque mueve el directorio de trabajo de la sesión, acceso de escritura, y configuración del proyecto como `CLAUDE.md` y configuración a esa ubicación. Una regla de [permiso](/docs/es/permissions) de `EnterWorktree` o elegir "no preguntar de nuevo" no suprime este aviso; solo el modo `bypassPermissions` lo omite. Antes de v2.1.206, Claude podía entrar en cualquier ruta de worktree existente sin preguntar.

A partir de v2.1.198, entrar o salir de un worktree también traslada la transcripción de la sesión al almacenamiento del proyecto de ese directorio, de la misma manera que [`/cd`](/docs/es/commands) lo hace, por lo que `/desktop` y `--resume` encuentran la sesión allí después. Los worktrees creados por un [hook `WorktreeCreate`](#non-git-version-control) se excluyen y mantienen la transcripción en el directorio de lanzamiento.

Los worktrees funcionan con [sandboxing](/docs/es/sandboxing#filesystem-isolation) habilitado: el sandbox permite escrituras al directorio compartido `.git` del repositorio principal para que comandos como `git commit` puedan actualizar refs e índice desde dentro de un worktree vinculado.

Antes de usar `--worktree` interactivamente en un directorio por primera vez, acepte el diálogo de confianza del espacio de trabajo ejecutando `claude` una vez en ese directorio. Si la confianza aún no ha sido aceptada, `--worktree` sale con un error y le solicita que ejecute `claude` en el directorio primero. Las ejecuciones no interactivas con `-p` omiten la [verificación de confianza](/docs/es/security), por lo que `claude -p --worktree` procede sin ella.

Si Claude Code no puede entrar en el directorio del worktree al iniciar, por ejemplo porque un [hook `WorktreeCreate`](/docs/es/hooks#worktreecreate) imprimió algo diferente al directorio que creó, o porque el directorio fue eliminado después de que fue configurado, Claude Code imprime un error nombrando la ruta y sale con código 1. Antes de v2.1.205, esto causaba un bloqueo de la sesión, y con `-p` se estancaba durante aproximadamente 30 segundos antes de salir con código 0.

Los plugins instalados en [ámbito de proyecto](/docs/es/plugins-reference#plugin-installation-scopes) desde el checkout principal también se cargan en worktrees del mismo repositorio, por lo que no necesita reinstalarlos por worktree. Esto se aplica tanto si crea el worktree con `--worktree` como con `git worktree add`. Requiere Claude Code v2.1.200 o posterior.

<Tip>
  Agregue `.claude/worktrees/` a su `.gitignore` para que el contenido del worktree no aparezca como archivos sin seguimiento en su checkout principal.
</Tip>

<h3 id="choose-the-base-branch">
  Elija la rama base
</h3>

Los worktrees se ramifican desde la rama predeterminada de su repositorio, `origin/HEAD`, por lo que comienzan desde un árbol limpio que coincide con el remoto. Cuando nada ha obtenido el repositorio en las últimas 24 horas, Claude Code actualiza `origin/HEAD` con una obtención de la rama predeterminada, limitada a cinco segundos, y utiliza la ref almacenada localmente en caché si la obtención falla. Si no hay remoto configurado, o `origin/HEAD` no está almacenado en caché localmente y no se puede obtener, el worktree vuelve a su `HEAD` local actual.

La actualización requiere Claude Code v2.1.208 o posterior; antes de eso, un worktree nuevo usaba lo que `origin/HEAD` ya estaba almacenado en caché localmente.

Para siempre ramificarse desde `HEAD` local en su lugar, establezca `worktree.baseRef` en `"head"` en [configuración](/docs/es/settings#worktree-settings). Establecer `baseRef` en `"head"` hace que los nuevos worktrees lleven sus commits no enviados y estado de rama de característica, lo cual es útil cuando se aíslan subagentes que necesitan operar en trabajo en progreso. Cuando la sesión se ejecuta dentro de un worktree vinculado, `"head"` se resuelve a ese `HEAD` del worktree, no al del checkout principal. La configuración acepta solo `"fresh"` o `"head"`, no refs de git arbitrarios:

```json theme={null}
{
  "worktree": {
    "baseRef": "head"
  }
}
```

Para ramificarse desde una solicitud de extracción específica, pase el número de PR prefijado con `#`, o una URL completa de solicitud de extracción de GitHub. Claude Code obtiene `pull/<number>/head` de `origin` y crea el worktree en `.claude/worktrees/pr-<number>`:

```bash theme={null}
claude --worktree "#1234"
```

Para control total sobre cómo se crean los worktrees, configure un [hook `WorktreeCreate`](/docs/es/hooks#worktreecreate), que reemplaza completamente la lógica predeterminada de `git worktree`.

<h3 id="reuse-a-worktree-name">
  Reutilice un nombre de worktree
</h3>

Reutilizar un nombre de worktree cuyo directorio ya existe reanuda ese worktree.

Un worktree reanudado se reinicia a la [base actual](#choose-the-base-branch) en lugar de reanudar en su punta anterior cuando se cumplen todas las siguientes condiciones:

* No tiene cambios sin confirmar ni archivos sin seguimiento.
* Todavía está en la rama que Claude Code creó para él.
* Nunca confirmó, o su solicitud de extracción fue fusionada y su rama remota fue eliminada.

Antes de v2.1.208, un nombre reutilizado siempre reanudaba el worktree anterior en su punta anterior.

<h2 id="copy-gitignored-files-into-worktrees">
  Copie archivos ignorados por git en worktrees
</h2>

Un worktree es un checkout fresco, por lo que archivos sin seguimiento como `.env` o `.env.local` de su repositorio principal no están presentes. Para copiarlos automáticamente cuando Claude crea un worktree, agregue un archivo `.worktreeinclude` a la raíz de su proyecto.

El archivo utiliza la sintaxis de `.gitignore`. Solo se copian los archivos que coinciden con un patrón y también están ignorados por git, por lo que los archivos rastreados nunca se duplican.

Este `.worktreeinclude` copia dos archivos env y una configuración de secretos en cada nuevo worktree:

```text .worktreeinclude theme={null}
.env
.env.local
config/secrets.json
```

Esto se aplica a worktrees creados con `--worktree`, [worktrees de subagentes](#isolate-subagents-with-worktrees), y sesiones paralelas en la [aplicación de escritorio](/docs/es/desktop#work-in-parallel-with-sessions).

<h2 id="isolate-subagents-with-worktrees">
  Aisle subagentes con worktrees
</h2>

Los subagentes pueden ejecutarse en sus propios worktrees para que las ediciones paralelas no entren en conflicto. Pida a Claude que "use worktrees para sus agentes", o establézcalo permanentemente en un [subagente personalizado](/docs/es/sub-agents#supported-frontmatter-fields) agregando `isolation: worktree` al frontmatter. Cada subagente obtiene un worktree temporal que se elimina automáticamente cuando el subagente termina sin cambios.

Los worktrees de subagentes utilizan la misma [rama base](#choose-the-base-branch) que `--worktree`, por lo que se ramifican desde la rama predeterminada de su repositorio a menos que `worktree.baseRef` esté configurado en `"head"`.

<h2 id="clean-up-worktrees">
  Limpie worktrees
</h2>

Cuando sale de una sesión de worktree, la limpieza depende de si realizó cambios:

* **Sin cambios no confirmados, sin archivos sin seguimiento y sin nuevos commits**: el worktree y su rama se eliminan automáticamente. Si la sesión tiene un [nombre](/docs/es/sessions#name-your-sessions), Claude le solicita en su lugar para que pueda mantener el worktree para más tarde
* **Existen cambios no confirmados, archivos sin seguimiento o nuevos commits**: Claude le solicita que mantenga o elimine el worktree. Mantener preserva el directorio y la rama para que pueda regresar más tarde. Eliminar borra el directorio del worktree y su rama, descartando todos los cambios no confirmados, archivos sin seguimiento y commits
* **Ejecuciones no interactivas**: los worktrees creados con `--worktree` junto con `-p` no se limpian automáticamente ya que no hay solicitud de salida. Elimínelos con `git worktree remove`

Los worktrees que Claude creó para subagentes y [sesiones en segundo plano](/docs/es/agent-view#how-file-edits-are-isolated) se eliminan automáticamente una vez que son más antiguos que su configuración [`cleanupPeriodDays`](/docs/es/settings#available-settings), siempre que no tengan cambios no confirmados, archivos sin seguimiento ni commits no enviados. Los worktrees que crea con `--worktree` nunca se eliminan por este barrido.

Mientras un agente se está ejecutando, Claude ejecuta `git worktree lock` en su worktree para que la limpieza concurrente no pueda eliminarlo. El bloqueo se libera cuando el agente termina. Para limpiar un worktree que el barrido mantiene, ejecute `git worktree remove`, agregando `--force` si el worktree tiene cambios no confirmados o archivos sin seguimiento.

En Windows, antes de eliminar un worktree, Claude Code elimina cualquier unión NTFS o enlace simbólico de directorio en cualquier profundidad dentro de él como una entrada de enlace, por lo que eliminar el worktree no borra los archivos a los que apunta un enlace. Antes de v2.1.205, Claude Code eliminaba solo enlaces de nivel superior como entradas de enlace, y eliminar un worktree con una unión anidada en un subdirectorio podría eliminar el contenido del directorio al que apuntaba el enlace fuera del worktree.

<h2 id="manage-worktrees-manually">
  Administre worktrees manualmente
</h2>

Para control total sobre la ubicación del worktree y la configuración de rama, cree worktrees directamente con Git. Esto es útil cuando necesita verificar una rama existente específica o colocar el worktree fuera del repositorio.

Cree un worktree en una nueva rama:

```bash theme={null}
git worktree add ../project-feature-a -b feature-a
```

Cree un worktree desde una rama existente:

```bash theme={null}
git worktree add ../project-bugfix bugfix-123
```

Inicie Claude en el worktree:

```bash theme={null}
cd ../project-feature-a && claude
```

Liste sus worktrees:

```bash theme={null}
git worktree list
```

Elimine uno cuando haya terminado con él:

```bash theme={null}
git worktree remove ../project-feature-a
```

Consulte la [documentación de git worktree](https://git-scm.com/docs/git-worktree) para la referencia completa de comandos. Recuerde inicializar su entorno de desarrollo en cada nuevo worktree: instale dependencias, configure entornos virtuales, o ejecute lo que requiera la configuración de su proyecto.

<h2 id="non-git-version-control">
  Control de versiones no-git
</h2>

El aislamiento de worktree usa git de forma predeterminada. Para SVN, Perforce, Mercurial u otros sistemas, configure [hooks `WorktreeCreate` y `WorktreeRemove`](/docs/es/hooks#worktreecreate) para proporcionar lógica de creación y limpieza personalizada. Debido a que el hook reemplaza el comportamiento predeterminado de git, [`.worktreeinclude`](#copy-gitignored-files-into-worktrees) no se procesa cuando usa `--worktree`. Copie cualquier archivo de configuración local dentro de su script de hook en su lugar.

Este hook `WorktreeCreate` lee el nombre del worktree desde stdin, verifica una copia de trabajo fresca de SVN e imprime la ruta del directorio para que Claude Code pueda usarla como el directorio de trabajo de la sesión:

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

<h2 id="see-also">
  Véase también
</h2>

Los worktrees manejan el aislamiento de archivos. Las páginas relacionadas a continuación cubren la delegación de trabajo en esos checkouts aislados y el cambio entre las sesiones que crea:

* [Subagentes](/docs/es/sub-agents): delegue trabajo a agentes aislados dentro de una sesión
* [Equipos de agentes](/docs/es/agent-teams): coordine múltiples sesiones de Claude automáticamente
* [Administrar sesiones](/docs/es/sessions): nombre, reanude y cambie entre conversaciones
* [Sesiones paralelas de escritorio](/docs/es/desktop#work-in-parallel-with-sessions): sesiones respaldadas por worktree en la aplicación de escritorio
