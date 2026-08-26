> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ejecutar prompts en un horario

> Utilice /loop y las herramientas de programación cron para ejecutar prompts repetidamente, sondear el estado o establecer recordatorios únicos dentro de una sesión de Claude Code.

Las tareas programadas permiten que Claude vuelva a ejecutar un prompt automáticamente en un intervalo. Úselas para sondear una implementación, supervisar un PR, verificar una compilación de larga duración o recordarse a sí mismo que debe hacer algo más adelante en la sesión. Para reaccionar a eventos a medida que ocurren en lugar de sondear, consulte [Channels](/docs/es/channels): su CI puede insertar el error directamente en la sesión. Para mantener la sesión funcionando turno tras turno hasta que se cumpla una condición en lugar de en un intervalo, consulte [`/goal`](/docs/es/goal).

Las tareas tienen alcance de sesión: viven en la conversación actual y se detienen cuando inicia una nueva. Reanudar con `--resume` o `--continue` trae de vuelta cualquier tarea que no haya [expirado](#seven-day-expiry): una tarea recurrente creada en los últimos 7 días, o una única cuyo tiempo programado aún no ha pasado. Para la programación que sobrevive independientemente de cualquier sesión, utilice [Routines](/docs/es/routines) para crear una rutina en la infraestructura administrada por Anthropic, configure una [tarea programada de Desktop](/docs/es/desktop-scheduled-tasks) o utilice [GitHub Actions](/docs/es/github-actions).

<h2 id="compare-scheduling-options">
  Comparar opciones de programación
</h2>

Claude Code offers three ways to schedule recurring or one-off work:

|                            | [Cloud](/docs/en/routines)               | [Desktop](/docs/en/desktop-scheduled-tasks) | [`/loop`](/docs/en/scheduled-tasks)      |
| :------------------------- | :---------------------------------- | :------------------------------------- | :---------------------------------- |
| Runs on                    | Cloud, Anthropic-managed by default | Your machine                           | Your machine                        |
| Requires machine on        | No                                  | Yes                                    | Yes                                 |
| Requires open session      | No                                  | No                                     | Yes                                 |
| Persistent across restarts | Yes                                 | Yes                                    | Restored on `--resume` if unexpired |
| Access to local files      | No (fresh clone)                    | Yes                                    | Yes                                 |
| MCP servers                | Connectors configured per task      | [Config files](/docs/en/mcp) and connectors | Inherits from session               |
| Permission prompts         | No (runs autonomously)              | Configurable per task                  | Inherits from session               |
| Customizable schedule      | Via `/schedule` in the CLI          | Yes                                    | Yes                                 |
| Minimum interval           | 1 hour                              | 1 minute                               | 1 minute                            |

<Tip>
  Use **cloud tasks** for work that should run reliably without your machine. Use **Desktop tasks** when you need access to local files and tools. Use **`/loop`** for quick polling during a session.
</Tip>

<h2 id="run-a-prompt-repeatedly-with-/loop">
  Ejecutar un prompt repetidamente con /loop
</h2>

El `/loop` [bundled skill](/docs/es/commands) es la forma más rápida de ejecutar un prompt repetidamente mientras la sesión permanece abierta. Tanto el intervalo como el prompt son opcionales, y lo que proporcione determina cómo se comporta el bucle.

| Lo que proporciona     | Ejemplo                     | Qué sucede                                                                                                            |
| :--------------------- | :-------------------------- | :-------------------------------------------------------------------------------------------------------------------- |
| Intervalo y prompt     | `/loop 5m check the deploy` | Su prompt se ejecuta en un [horario fijo](#run-on-a-fixed-interval)                                                   |
| Solo prompt            | `/loop check the deploy`    | Su prompt se ejecuta en un [intervalo que Claude elige](#let-claude-choose-the-interval) en cada iteración            |
| Solo intervalo, o nada | `/loop`                     | El [prompt de mantenimiento integrado](#run-the-built-in-maintenance-prompt) se ejecuta, o su `loop.md` si existe uno |

También puede pasar un skill como el prompt, por ejemplo `/loop 20m /review-pr 1234`, para volver a ejecutar ese skill en cada iteración. A partir de v2.1.196, un disparo programado solo ejecuta skills que Claude [está autorizado a invocar por su cuenta](/docs/es/skills#control-who-invokes-a-skill). Lo siguiente llega a Claude como texto sin formato en lugar de ejecutarse:

* comandos integrados como `/permissions`, `/model`, o `/clear`
* skills marcados [`disable-model-invocation: true`](/docs/es/skills#frontmatter-reference)
* skills retenidos de Claude por una configuración [`skillOverrides`](/docs/es/skills#override-skill-visibility-from-settings) o una regla de [denegación](/docs/es/skills#restrict-claude’s-skill-access) de `Skill`
* [prompts MCP](/docs/es/mcp#use-mcp-prompts-as-commands) como `/mcp__github__list_prs`; los skills que expone un servidor MCP aún se ejecutan

<h3 id="run-on-a-fixed-interval">
  Ejecutar en un intervalo fijo
</h3>

Cuando proporciona un intervalo, Claude lo convierte en una expresión cron, programa el trabajo y confirma la cadencia y el ID del trabajo.

```text theme={null}
/loop 5m check if the deployment finished and tell me what happened
```

El intervalo puede encabezar el prompt como un token simple como `30m`, o seguirlo como una cláusula como `every 2 hours`. Las unidades admitidas son `s` para segundos, `m` para minutos, `h` para horas y `d` para días.

Los segundos se redondean al minuto más cercano ya que cron tiene una granularidad de un minuto. Los intervalos que no se asignan a un paso cron limpio, como `7m` o `90m`, se redondean al intervalo más cercano que sí lo hace y Claude le dice cuál eligió.

<h3 id="let-claude-choose-the-interval">
  Dejar que Claude elija el intervalo
</h3>

Cuando omite el intervalo, Claude elige uno dinámicamente en lugar de ejecutarse en un horario cron fijo. Después de cada iteración, elige un retraso entre un minuto y una hora según lo que observó: esperas cortas mientras una compilación se está terminando o un PR está activo, esperas más largas cuando no hay nada pendiente. El retraso elegido y la razón del mismo se imprimen al final de cada iteración.

El ejemplo a continuación verifica CI y comentarios de revisión, con Claude esperando más tiempo entre iteraciones una vez que el PR se queda en silencio:

```text theme={null}
/loop check whether CI passed and address any review comments
```

Cuando solicita un horario `/loop` dinámico, Claude puede usar la [herramienta Monitor](/docs/es/tools-reference#monitor-tool) directamente. Monitor ejecuta un script de fondo y transmite cada línea de salida, lo que evita el sondeo por completo y a menudo es más eficiente en tokens y más receptivo que volver a ejecutar un prompt en un intervalo.

Un bucle programado dinámicamente aparece en su [lista de tareas programadas](#manage-scheduled-tasks) como cualquier otra tarea, por lo que puede enumerarla o cancelarla de la misma manera. Las [reglas de jitter](#jitter) no se aplican a ella, pero el [vencimiento de siete días](#seven-day-expiry) sí: el bucle termina automáticamente siete días después de iniciarlo.

<Note>
  En Amazon Bedrock, Claude Platform en AWS, Google Cloud's Agent Platform y Microsoft Foundry, un prompt sin intervalo se ejecuta en un horario fijo de 10 minutos en su lugar.
</Note>

<h3 id="run-the-built-in-maintenance-prompt">
  Ejecutar el prompt de mantenimiento integrado
</h3>

Cuando omite el prompt, Claude usa un prompt de mantenimiento integrado en lugar de uno que proporcione. En cada iteración, trabaja a través de lo siguiente, en orden:

* continuar cualquier trabajo inacabado de la conversación
* cuidar el pull request de la rama actual: comentarios de revisión, ejecuciones de CI fallidas, conflictos de fusión
* ejecutar pasadas de limpieza como búsquedas de errores o simplificación cuando no hay nada más pendiente

Claude no inicia nuevas iniciativas fuera de ese alcance, y las acciones irreversibles como insertar o eliminar solo proceden cuando continúan algo que la transcripción ya autorizó.

```text theme={null}
/loop
```

Un `/loop` simple ejecuta este prompt en un [intervalo elegido dinámicamente](#let-claude-choose-the-interval). Agregue un intervalo, por ejemplo `/loop 15m`, para ejecutarlo en un horario fijo en su lugar. Para reemplazar el prompt integrado con el suyo propio, consulte [Personalizar el prompt predeterminado con loop.md](#customize-the-default-prompt-with-loop-md).

<Note>
  En Amazon Bedrock, Claude Platform en AWS, Google Cloud's Agent Platform y Microsoft Foundry, `/loop` sin prompt imprime el mensaje de uso en su lugar en lugar de ejecutar el prompt de mantenimiento.
</Note>

<h3 id="customize-the-default-prompt-with-loop-md">
  Personalizar el prompt predeterminado con loop.md
</h3>

Un archivo `loop.md` reemplaza el prompt de mantenimiento integrado con sus propias instrucciones. Define un único prompt predeterminado para `/loop` simple, no una lista de tareas programadas separadas, e se ignora siempre que proporcione un prompt en la línea de comandos. Para programar prompts adicionales junto a él, use `/loop <prompt>` o [pida a Claude directamente](#manage-scheduled-tasks).

Claude busca el archivo en dos ubicaciones y usa el primero que encuentra.

| Ruta                | Alcance                                                                         |
| :------------------ | :------------------------------------------------------------------------------ |
| `.claude/loop.md`   | Nivel de proyecto. Tiene precedencia cuando ambos archivos existen.             |
| `~/.claude/loop.md` | Nivel de usuario. Se aplica en cualquier proyecto que no defina el suyo propio. |

El archivo es Markdown simple sin estructura requerida. Escríbalo como si estuviera escribiendo el prompt `/loop` directamente. El siguiente ejemplo mantiene una rama de lanzamiento saludable:

```markdown title=".claude/loop.md" theme={null}
Check the `release/next` PR. If CI is red, pull the failing job log,
diagnose, and push a minimal fix. If new review comments have arrived,
address each one and resolve the thread. If everything is green and
quiet, say so in one line.
```

Las ediciones a `loop.md` tienen efecto en la siguiente iteración, por lo que puede refinar las instrucciones mientras un bucle se está ejecutando. Cuando no existe `loop.md` en ninguna ubicación, el bucle vuelve al prompt de mantenimiento integrado. Mantenga el archivo conciso: el contenido más allá de 25,000 bytes se trunca.

<Note>
  En Amazon Bedrock, Claude Platform en AWS, Google Cloud's Agent Platform y Microsoft Foundry, `loop.md` no se lee y `/loop` sin prompt imprime el mensaje de uso en su lugar.
</Note>

<h3 id="stop-a-loop">
  Detener un bucle
</h3>

Para detener un `/loop` mientras espera la siguiente iteración, presione `Esc`. Esto borra el despertar pendiente para que el bucle no se ejecute nuevamente. Las tareas que programó [pidiendo a Claude directamente](#manage-scheduled-tasks) no se ven afectadas por `Esc` y permanecen en su lugar hasta que las elimine.

En [modo de ritmo propio](#let-claude-choose-the-interval), Claude también puede terminar el bucle por su cuenta una vez que la tarea se complete. Claude llama a la herramienta [`ScheduleWakeup`](/docs/es/tools-reference) con `stop: true`, que cancela el despertar pendiente inmediatamente. Si una iteración termina sin reprogramar ni detener, Claude Code programa un despertar de respaldo aproximadamente 20 minutos después y termina el bucle cuando esa iteración tampoco se reprograma. Antes de v2.1.202, no reprogramar era la única forma en que Claude podía terminar un bucle por su cuenta.

Los bucles en un intervalo fijo siguen ejecutándose hasta que los detenga o [transcurran siete días](#seven-day-expiry).

<h2 id="set-a-one-time-reminder">
  Establecer un recordatorio único
</h2>

Para recordatorios únicos, describa lo que desea en lenguaje natural en lugar de usar `/loop`. Claude programa una tarea de un solo disparo que se elimina a sí misma después de ejecutarse.

```text theme={null}
remind me at 3pm to push the release branch
```

```text theme={null}
in 45 minutes, check whether the integration tests passed
```

Claude fija la hora de disparo a un minuto y hora específicos usando una expresión cron y confirma cuándo se ejecutará.

<h2 id="manage-scheduled-tasks">
  Gestionar tareas programadas
</h2>

Pida a Claude en lenguaje natural que enumere o cancele tareas, o haga referencia directamente a las herramientas subyacentes.

```text theme={null}
what scheduled tasks do I have?
```

```text theme={null}
cancel the deploy check job
```

Bajo el capó, Claude utiliza estas herramientas:

| Herramienta  | Propósito                                                                                                                        |
| :----------- | :------------------------------------------------------------------------------------------------------------------------------- |
| `CronCreate` | Programar una nueva tarea. Acepta una expresión cron de 5 campos, el prompt a ejecutar y si se repite o se ejecuta una sola vez. |
| `CronList`   | Enumerar todas las tareas programadas con sus IDs, horarios y prompts.                                                           |
| `CronDelete` | Cancelar una tarea por ID.                                                                                                       |

Cada tarea programada tiene un ID de 8 caracteres que puede pasar a `CronDelete`. Una sesión puede contener hasta 50 tareas programadas a la vez.

<h2 id="how-scheduled-tasks-run">
  Cómo se ejecutan las tareas programadas
</h2>

El programador verifica cada segundo si hay tareas vencidas y las encola con baja prioridad. Un prompt programado se ejecuta entre sus turnos, no mientras Claude está en medio de una respuesta. Si Claude está ocupado cuando vence una tarea, el prompt espera hasta que termine el turno actual.

Todos los tiempos se interpretan en su zona horaria local. Una expresión cron como `0 9 * * *` significa las 9am donde está ejecutando Claude Code, no UTC.

<h3 id="jitter">
  Jitter
</h3>

Para evitar que cada sesión golpee la API en el mismo momento de reloj de pared, el programador agrega un desplazamiento determinista a los tiempos de disparo:

* Las tareas recurrentes se ejecutan hasta 30 minutos después de la hora programada (o hasta la mitad del intervalo, para tareas que se ejecutan más frecuentemente que cada hora). Un trabajo por hora programado para `:00` puede ejecutarse en cualquier momento hasta `:30`.
* Las tareas únicas programadas para la parte superior o inferior de la hora se ejecutan hasta 90 segundos antes.

El desplazamiento se deriva del ID de la tarea, por lo que la misma tarea siempre obtiene el mismo desplazamiento. Si el tiempo exacto es importante, elija un minuto que no sea `:00` o `:30`, por ejemplo `3 9 * * *` en lugar de `0 9 * * *`, y el jitter único no se aplicará.

<h3 id="seven-day-expiry">
  Vencimiento de siete días
</h3>

Las tareas recurrentes expiran automáticamente 7 días después de su creación. La tarea se ejecuta una última vez, luego se elimina a sí misma. Esto limita cuánto tiempo puede ejecutarse un bucle olvidado. Si necesita que una tarea recurrente dure más, cancele y recree antes de que expire, o utilice [Routines](/docs/es/routines) o [tareas programadas de Desktop](/docs/es/desktop-scheduled-tasks) para programación duradera.

<h2 id="cron-expression-reference">
  Referencia de expresión cron
</h2>

`CronCreate` acepta expresiones cron estándar de 5 campos: `minute hour day-of-month month day-of-week`. Todos los campos admiten comodines (`*`), valores únicos (`5`), pasos (`*/15`), rangos (`1-5`) y listas separadas por comas (`1,15,30`).

| Ejemplo        | Significado                    |
| :------------- | :----------------------------- |
| `*/5 * * * *`  | Cada 5 minutos                 |
| `0 * * * *`    | Cada hora en punto             |
| `7 * * * *`    | Cada hora a los 7 minutos      |
| `0 9 * * *`    | Todos los días a las 9am local |
| `0 9 * * 1-5`  | Días de semana a las 9am local |
| `30 14 15 3 *` | 15 de marzo a las 2:30pm local |

El día de la semana usa `0` o `7` para domingo hasta `6` para sábado. La sintaxis extendida como `L`, `W`, `?` y alias de nombres como `MON` o `JAN` no se admiten.

Cuando tanto el día del mes como el día de la semana están restringidos, una fecha coincide si cualquiera de los campos coincide. Esto sigue la semántica estándar de vixie-cron.

<h2 id="disable-scheduled-tasks">
  Deshabilitar tareas programadas
</h2>

Establezca `CLAUDE_CODE_DISABLE_CRON=1` en su entorno para deshabilitar completamente el programador. Las herramientas cron y `/loop` dejan de estar disponibles, y cualquier tarea ya programada deja de ejecutarse. Consulte [Variables de entorno](/docs/es/env-vars) para la lista completa de banderas de deshabilitación.

<h2 id="limitations">
  Limitaciones
</h2>

La programación con alcance de sesión tiene limitaciones inherentes:

* Las tareas solo se ejecutan mientras Claude Code está ejecutándose e inactivo. Cerrar la terminal o dejar que la sesión salga detiene su ejecución. [Poner la sesión en segundo plano](/docs/es/agent-view#from-inside-a-session) traslada las tareas `/loop` a una sesión en segundo plano, que sigue ejecutándose sin una terminal.
* Sin recuperación de disparos perdidos. Si el tiempo programado de una tarea pasa mientras Claude está ocupado en una solicitud de larga duración, se ejecuta una vez cuando Claude queda inactivo, no una vez por intervalo perdido.
* Iniciar una conversación nueva borra todas las tareas con alcance de sesión. Reanudar con `claude --resume` o `claude --continue` restaura tareas que no han expirado: tareas recurrentes dentro de siete días de creación, y tareas únicas cuyo tiempo programado aún no ha pasado. Las tareas de Bash de fondo y monitor nunca se restauran al reanudar.

Para la automatización impulsada por cron que necesita ejecutarse sin supervisión:

* [Routines](/docs/es/routines): se ejecutan en infraestructura administrada por Anthropic en un horario, mediante llamada a API o en eventos de GitHub
* [GitHub Actions](/docs/es/github-actions): utilice un disparador `schedule` en CI
* [tareas programadas de Desktop](/docs/es/desktop-scheduled-tasks): se ejecutan localmente en su máquina
