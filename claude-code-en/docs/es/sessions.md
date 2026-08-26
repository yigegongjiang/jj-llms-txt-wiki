> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gestionar sesiones

> Nombre, reanude, ramifique y cambie entre conversaciones de Claude Code. Cubre `--continue`, `--resume`, `--from-pr`, el selector `/resume`, nombres de sesión, exportación de transcripciones y dónde se almacenan las transcripciones.

Una sesión es una conversación guardada vinculada a un directorio de proyecto. Claude Code la almacena localmente mientras trabaja, para que pueda reanudar donde lo dejó, ramificarse para probar un enfoque diferente o cambiar entre tareas.

La [aplicación de escritorio](/docs/es/desktop#work-in-parallel-with-sessions), [Claude Code en la web](/docs/es/claude-code-on-the-web) y la [extensión de VS Code](/docs/es/vs-code#resume-past-conversations) mantienen cada una su propio historial de sesiones. Esta página cubre la CLI.

<h2 id="resume-a-session">
  Reanude una sesión
</h2>

Las sesiones se guardan continuamente en [archivos de transcripción locales](#export-and-locate-session-data) mientras trabaja, para que pueda volver a una después de salir o ejecutar `/clear`. Use estos puntos de entrada:

| Comando                     | Qué hace                                                              |
| :-------------------------- | :-------------------------------------------------------------------- |
| `claude --continue`         | Reanuda la sesión más reciente en el directorio actual                |
| `claude --resume`           | Abre el [selector de sesiones](#use-the-session-picker)               |
| `claude --resume <name>`    | Reanuda la sesión nombrada directamente                               |
| `claude --from-pr <number>` | Reanuda la sesión vinculada a esa solicitud de extracción             |
| `/resume`                   | Cambia a una conversación diferente desde dentro de una sesión activa |

Las sesiones creadas con [`claude -p`](/docs/es/headless) o el [Agent SDK](/docs/es/agent-sdk/overview) no aparecen en el selector de sesiones, pero aún puede reanudar una pasando su ID de sesión a `claude --resume <session-id>`. Ejecute esto desde el directorio en el que se inició la sesión: la búsqueda de ID de sesión se limita al directorio del proyecto actual y sus git worktrees, por lo que una sesión creada en otro lugar reporta `No conversation found with session ID: <session-id>`.

<h3 id="where-the-session-picker-looks">
  Dónde busca el selector de sesiones
</h3>

Las sesiones se almacenan por directorio de proyecto. De forma predeterminada, el selector de sesiones muestra sesiones interactivas del worktree actual, más sesiones iniciadas en otro lugar que agregaron el directorio actual con `/add-dir`. Use `Ctrl+W` para ampliar a todos los worktrees del repositorio o `Ctrl+A` para ampliar a cada proyecto en esta máquina.

A partir de v2.1.169, mover una sesión con [`/cd`](/docs/es/commands) la traslada al almacenamiento del proyecto del nuevo directorio, por lo que aparece en el selector de ese directorio después. A partir de v2.1.196, una sesión movida se mantiene fuera del selector del directorio anterior incluso después de un bloqueo o salida forzada. En versiones anteriores, también podría reaparecer en la lista del directorio anterior después de una salida que no fue limpia cuando la ruta anterior contenía caracteres especiales como guiones bajos.

Seleccionar una sesión de otro worktree del mismo repositorio la reanuda en su lugar. Seleccionar una sesión de un proyecto no relacionado copia un comando `cd` y reanuda a su portapapeles en su lugar.

Reanudar por nombre se resuelve en el repositorio actual y sus worktrees. Ambas formas buscan una coincidencia exacta y la reanudan directamente incluso si vive en un worktree diferente:

| Comando                  | Coincidencia exacta  | Nombre ambiguo                                                                            |
| :----------------------- | :------------------- | :---------------------------------------------------------------------------------------- |
| `claude --resume <name>` | Reanuda directamente | Abre el selector de sesiones con el nombre rellenado previamente como término de búsqueda |
| `/resume <name>`         | Reanuda directamente | Reporta un error; ejecute `/resume` sin argumentos para abrir el selector de sesiones     |

<h2 id="name-your-sessions">
  Nombre sus sesiones
</h2>

Dé a las sesiones nombres descriptivos para que sean encontrables en el selector de sesiones y reanudables por nombre. Esto es más importante cuando está trabajando en varias tareas en paralelo.

| Cuándo                        | Cómo establecer el nombre                                                                                                                                                       |
| :---------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Al inicio                     | `claude -n auth-refactor`                                                                                                                                                       |
| Durante una sesión            | `/rename auth-refactor`. El nombre también aparece en la barra de indicaciones                                                                                                  |
| Desde el selector de sesiones | Resalte una sesión y presione `Ctrl+R`                                                                                                                                          |
| Al aceptar un plan            | Aceptar un plan en [modo de plan](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode) nombra la sesión desde el contenido del plan a menos que ya haya establecido uno |

Una vez que una sesión está nombrada, vuelva a ella con `claude --resume <name>` o `/resume <name>`. Vea [Reanude una sesión](#resume-a-session) para saber cómo se comporta la resolución de nombres en worktrees.

Las sesiones interactivas que nunca nombra aún obtienen un nombre de visualización predeterminado cuando se inician. Requiere Claude Code v2.1.196 o posterior. El valor predeterminado combina el nombre del directorio de trabajo con un sufijo de dos caracteres, por ejemplo `my-app-3f`, e identifica la sesión en listados de sesiones en ejecución, como [vista de agente](/docs/es/agent-view) y salida de `claude agents --json`.

El valor predeterminado no es un identificador de reanudación: `claude --resume <name>`, `/resume <name>` y el selector de sesiones coinciden solo con nombres que usted establece. Nombrar la sesión reemplaza el valor predeterminado.

<h2 id="use-the-session-picker">
  Usar el selector de sesiones
</h2>

Ejecute `/resume` dentro de una sesión, o `claude --resume` sin argumentos, para abrir el selector de sesiones interactivo. Use estos atajos de teclado para navegar, buscar y ampliar la lista:

| Atajo                                                  | Acción                                                                                                                                                                                 |
| :----------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `↑` / `↓`                                              | Navegar entre sesiones                                                                                                                                                                 |
| `→` / `←`                                              | Expandir o contraer sesiones agrupadas                                                                                                                                                 |
| `Enter`                                                | Reanuda la sesión resaltada                                                                                                                                                            |
| `Space`                                                | Previsualiza el contenido de la sesión. `Ctrl+V` también funciona en terminales que no lo capturan como pegado                                                                         |
| `Ctrl+R`                                               | Renombra la sesión resaltada                                                                                                                                                           |
| `/` o cualquier carácter imprimible que no sea `Space` | Ingrese al modo de búsqueda y filtre sesiones. Pegue una URL de solicitud de extracción o fusión de GitHub, GitHub Enterprise, GitLab o Bitbucket para encontrar la sesión que la creó |
| `Ctrl+A`                                               | Muestra sesiones de todos los proyectos en esta máquina. Presione nuevamente para volver al repositorio actual                                                                         |
| `Ctrl+W`                                               | Muestra sesiones de todos los worktrees del repositorio actual. Presione nuevamente para volver al worktree actual. Solo se muestra en repositorios con múltiples worktrees            |
| `Ctrl+B`                                               | Filtra a sesiones de la rama git actual. Presione nuevamente para mostrar todas las ramas                                                                                              |
| `Esc`                                                  | Salga del selector de sesiones o del modo de búsqueda                                                                                                                                  |

Cada fila muestra el nombre de la sesión si está establecido, de lo contrario el resumen de la conversación o el primer indicador, junto con el tiempo desde la última actividad, el recuento de mensajes y la rama git. La ruta del proyecto aparece después de ampliar a todos los proyectos con `Ctrl+A`.

Las sesiones bifurcadas creadas con `/branch`, `/rewind` o `--fork-session` se agrupan bajo su sesión raíz. Presione `→` para expandir un grupo.

<h2 id="branch-a-session">
  Ramifique una sesión
</h2>

La ramificación crea una copia de la conversación hasta ahora y lo cambia a ella, dejando el original intacto. Úselo para probar un enfoque diferente sin perder el camino en el que estaba.

Desde dentro de una sesión, ejecute `/branch` con un nombre opcional:

```text theme={null}
/branch try-streaming-approach
```

Si omite el nombre, Claude Code nombra la nueva rama después del primer mensaje en la conversación. A partir de v2.1.198 esto también se aplica después de [compactación](/docs/es/how-claude-code-works#when-context-fills-up); las versiones anteriores recurrieron al nombre literal `Branched conversation` en lugar de mirar más allá del resumen de compactación al mensaje original.

Desde la línea de comandos, combine `--continue` o `--resume` con `--fork-session`:

```bash theme={null}
claude --continue --fork-session
```

La sesión original no se modifica y permanece disponible en el selector de sesiones. La confirmación de `/branch` imprime dos IDs de sesión: la nueva rama en la que se encuentra ahora y la original. Para volver a la original, pase su ID a `/resume`, use el selector de sesiones o ejecute `/resume <original-name>`. Los permisos que aprobó con "permitir para esta sesión" no se transfieren a la nueva rama. Si reanuda la misma sesión en dos terminales sin bifurcar, los mensajes de ambos se intercalan en una transcripción.

Para rewind basado en puntos de control dentro de una sola sesión, vea [Checkpointing](/docs/es/checkpointing).

<h2 id="manage-context-within-a-session">
  Gestione el contexto dentro de una sesión
</h2>

Estos comandos controlan qué hay en la ventana de contexto sin dejar la sesión:

* **`/clear`**: comience de nuevo con un contexto vacío. La conversación anterior se guarda y es reanudable con `/resume`, o, en el mismo proceso de Claude Code, desde [la entrada de sesión anterior del menú de rewind](/docs/es/checkpointing#rewind-past-a-cleared-conversation)
* **`/compact [instructions]`**: reemplace el historial con un resumen, opcionalmente enfocado en lo que especifique
* **`/context`**: muestre qué está consumiendo actualmente el contexto

Para saber cómo la compactación interactúa con CLAUDE.md, skills y reglas, vea la [guía de ventana de contexto](/docs/es/context-window). Para estrategias sobre cuándo limpiar versus compactar, vea [Mejores prácticas](/docs/es/best-practices#manage-your-session).

<h2 id="export-and-locate-session-data">
  Exporte y localice datos de sesión
</h2>

Ejecute `/export` para abrir un menú que le permita copiar la conversación actual a su portapapeles o guardarla como un archivo de texto sin formato, con mensajes y salidas de herramientas renderizadas como texto legible. Pase un nombre de archivo para omitir el menú y escribir directamente en ese archivo.

<h3 id="access-conversations-from-scripts">
  Acceda a conversaciones desde scripts
</h3>

`/export` produce una transcripción renderizada para que una persona la lea. Las interfaces a continuación producen datos estructurados para que un script analice: un resultado JSON de una ejecución, la ruta al archivo de transcripción de una sesión, o un flujo en vivo de eventos. Elija según lo que active el script:

* **Ejecute Claude una vez y capture el resultado**: invoque `claude -p` con [`--output-format json` o `stream-json`](/docs/es/headless#get-structured-output) para capturar el resultado, ID de sesión, uso y costo de una ejecución no interactiva como JSON estructurado.
* **Haga una pregunta a una sesión existente**: pase un ID de sesión a [`claude -p --resume`](/docs/es/headless#continue-conversations) para enviar un mensaje de seguimiento, como una solicitud de resumen, y capture la respuesta estructurada.
* **Reaccione a eventos de sesión**: lea el campo `transcript_path` que [hooks](/docs/es/hooks#common-input-fields) y [comandos de línea de estado](/docs/es/statusline#available-data) reciben como entrada. Un hook `SessionEnd` puede archivar la transcripción cuando finaliza una sesión.
* **Integre Claude en una aplicación TypeScript o Python**: use el [Agent SDK](/docs/es/agent-sdk/overview) para recibir cada mensaje mediante programación.

El ejemplo a continuación utiliza la segunda interfaz. Envía un mensaje de seguimiento a una sesión existente y lee la respuesta con `jq`:

```bash theme={null}
claude -p --resume <session-id> --output-format json "summarize what we changed" | jq -r '.result'
```

<h3 id="where-transcripts-are-stored">
  Dónde se almacenan las transcripciones
</h3>

De forma predeterminada, las transcripciones se almacenan como JSONL en `~/.claude/projects/<project>/<session-id>.jsonl`, donde `<project>` es la ruta de su directorio de trabajo con caracteres no alfanuméricos reemplazados por `-`. Cada línea es un objeto JSON para un mensaje, uso de herramienta o entrada de metadatos. El formato de entrada es interno de Claude Code y cambia entre versiones, por lo que los scripts que analizan estos archivos directamente pueden romperse en cualquier versión. Para construir sobre datos de sesión, use `/export` o las [interfaces de script](#access-conversations-from-scripts) en su lugar.

La ubicación, retención y comportamiento de escritura son configurables:

| Para                                                    | Establecer                                             | Dónde                       |
| ------------------------------------------------------- | ------------------------------------------------------ | --------------------------- |
| Mover almacenamiento fuera de `~/.claude`               | [`CLAUDE_CONFIG_DIR`](/docs/es/env-vars)                    | Variable de entorno         |
| Cambiar la retención de 30 días                         | [`cleanupPeriodDays`](/docs/es/settings#available-settings) | `settings.json`             |
| Suprimir escrituras de transcripción en todos los modos | [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/es/env-vars)      | Variable de entorno         |
| Suprimir escrituras para una ejecución no interactiva   | [`--no-session-persistence`](/docs/es/cli-reference)        | Bandera CLI con `claude -p` |

<h2 id="see-also">
  Ver también
</h2>

Estas páginas cubren mecánicas relacionadas de sesión y paralelismo:

* [Worktrees](/docs/es/worktrees): ejecute sesiones paralelas aisladas en ramas separadas
* [Checkpointing](/docs/es/checkpointing): rebobine código y conversación a un punto anterior
* [Ventana de contexto](/docs/es/context-window): qué llena el contexto y qué sobrevive a la compactación
* [Modo no interactivo](/docs/es/headless): comportamiento de sesión bajo `claude -p`
