> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Mantener a Claude trabajando hacia un objetivo

> Establezca una condición de finalización con /goal y Claude seguirá trabajando hasta que se cumpla, un modelo juzgue que es imposible, o un error que deba corregir borre el objetivo.

El comando `/goal` establece una condición de finalización y Claude sigue trabajando hacia ella sin que usted solicite cada paso. Después de cada turno, un modelo pequeño y rápido verifica si se cumple la condición. Si el modelo juzga que aún no se cumple, Claude inicia otro turno en lugar de devolver el control a usted. El objetivo se borra automáticamente una vez que se cumple la condición, si el modelo juzga que la condición es imposible de satisfacer, o si un turno falla en [un error que deba corregir](#errors-you-have-to-fix-clear-the-goal).

Utilice un objetivo para trabajo sustancial con un estado final verificable:

* Migrar un módulo a una nueva API hasta que cada sitio de llamada se compile y las pruebas pasen
* Implementar un documento de diseño hasta que se cumplan todos los criterios de aceptación
* Dividir un archivo grande en módulos enfocados hasta que cada uno esté dentro de un presupuesto de tamaño
* Trabajar a través de un backlog de problemas etiquetados hasta que la cola esté vacía

<h2 id="compare-ways-to-keep-a-session-running">
  Comparar formas de mantener una sesión en ejecución
</h2>

Tres enfoques mantienen la sesión actual en ejecución entre solicitudes. Elija según lo que deba iniciar el siguiente turno:

| Enfoque                                                             | El siguiente turno comienza cuando                                                                                                                                                                       | Se detiene cuando                                                                                                                                                                                      |
| :------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/goal`                                                             | El turno anterior finaliza, o, en una sesión interactiva, una [verificación de inactividad](#background-work-defers-evaluation) o un [reintento automático](#other-errors-retry-or-pause-the-goal) vence | Un modelo confirma que se cumple la condición o la juzga imposible, o un turno falla en [un error que debe corregir](#errors-you-have-to-fix-clear-the-goal), o ejecuta [`/goal clear`](#clear-a-goal) |
| [`/loop`](/docs/es/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop) | Transcurre un intervalo de tiempo                                                                                                                                                                        | Usted lo detiene, o Claude decide que el trabajo está hecho                                                                                                                                            |
| [Stop hook](/docs/es/hooks-guide#prompt-based-hooks)                     | El turno anterior finaliza                                                                                                                                                                               | Su propio script o solicitud decide                                                                                                                                                                    |

`/goal` y un Stop hook se activan después de cada turno. `/goal` es un atajo con alcance de sesión: escribe una condición y está activa solo para la sesión actual. Un Stop hook vive en su archivo de configuración, se aplica a cada sesión en su alcance y puede ejecutar un script para verificaciones deterministas o una solicitud para evaluaciones basadas en modelos.

[El modo automático](/docs/es/auto-mode-config) por sí solo aprueba llamadas de herramientas dentro de un único turno pero no inicia uno nuevo. Claude se detiene cuando juzga que el trabajo está hecho. `/goal` añade un evaluador separado que verifica su condición después de cada turno, por lo que la finalización es decidida por un modelo nuevo en lugar del que realiza el trabajo. Los dos son complementarios: el modo automático elimina solicitudes por herramienta, y `/goal` elimina solicitudes por turno.

<Tip>
  Los enfoques anteriores mantienen la sesión actual en ejecución. También puede programar trabajo que se ejecute independientemente de cualquier sesión abierta, como pruebas nocturnas o triaje matutino. Consulte [opciones de programación](/docs/es/scheduled-tasks#compare-scheduling-options) para rutinas en la nube y tareas programadas de escritorio.
</Tip>

<h2 id="use-/goal">
  Usar `/goal`
</h2>

Solo un objetivo puede estar activo por sesión. El mismo comando establece, verifica y lo borra según el argumento.

<h3 id="set-a-goal">
  Establecer un objetivo
</h3>

Ejecute `/goal` seguido de la condición que desea que se cumpla. Si ya hay un objetivo activo, el nuevo lo reemplaza.

```text theme={null}
/goal all tests in test/auth pass and the lint step is clean
```

Establecer un objetivo inicia un turno inmediatamente, con la condición misma como directiva. No necesita enviar un mensaje separado. Mientras el objetivo está activo, un indicador `◎ /goal active` muestra cuánto tiempo lleva ejecutándose el objetivo.

Un objetivo no cambia su modo de permisos. Para permitir que los turnos de objetivo se ejecuten sin supervisión, ejecute `/goal` en [modo automático](/docs/es/auto-mode-config). En [modo Manual](/docs/es/permission-modes), Claude aún pregunta antes de llamadas a herramientas que su configuración no permita, como el comando de prueba anterior.

Mientras el objetivo está activo, la transcripción muestra cada veredicto que devuelve el evaluador, y puede presionar Ctrl+O para ver la razón detrás del mismo. La vista de estado también muestra la razón más reciente, para que pueda ver hacia dónde está trabajando Claude a continuación.

<h3 id="write-an-effective-condition">
  Escribir una condición efectiva
</h3>

El [evaluador](#how-evaluation-works) juzga su condición contra lo que Claude ha expuesto en la conversación. No ejecuta comandos ni lee archivos de forma independiente, así que escriba la condición como algo que la propia salida de Claude pueda demostrar. "Todas las pruebas en `test/auth` pasan" funciona porque Claude ejecuta las pruebas y el resultado aparece en la transcripción para que el evaluador lo lea.

Una condición que se mantiene a lo largo de muchos turnos generalmente tiene:

* **Un estado final medible**: un resultado de prueba, un código de salida de compilación, un recuento de archivos, una cola vacía
* **Una verificación establecida**: cómo Claude debe probarlo, como "`npm test` sale con 0" o "`git status` está limpio"
* **Restricciones que importan**: cualquier cosa que no debe cambiar en el camino, como "ningún otro archivo de prueba se modifica"

La condición puede tener hasta 4.000 caracteres.

Para limitar cuánto tiempo se ejecuta un objetivo, incluya una cláusula de turno o tiempo en la condición, como `or stop after 20 turns`. Claude informa el progreso contra esa cláusula cada turno y el evaluador la juzga desde la conversación.

<h3 id="check-status">
  Verificar estado
</h3>

Ejecute `/goal` sin argumentos para ver el estado actual.

```text theme={null}
/goal
```

Si un objetivo está activo, el estado muestra:

* La condición
* Cuánto tiempo lleva ejecutándose
* Cuántos turnos se han evaluado
* El gasto de tokens actual
* La razón más reciente del evaluador

El recuento de turnos y la razón más reciente aparecen después de que se haya ejecutado la primera evaluación.

Si no hay un objetivo activo pero uno se logró anteriormente en la sesión, el estado muestra la condición lograda junto con su duración, recuento de turnos y gasto de tokens.

<h3 id="clear-a-goal">
  Borrar un objetivo
</h3>

Ejecute `/goal clear` para eliminar un objetivo activo antes de que se resuelva.

```text theme={null}
/goal clear
```

Claude imprime `Goal cleared:` seguido de la condición para confirmar, o `No goal set` si nada estaba activo.

`stop`, `off`, `reset`, `none` y `cancel` se aceptan como alias para `clear`. Ejecutar `/clear` para iniciar una nueva conversación también elimina cualquier objetivo activo.

<h3 id="resume-with-an-active-goal">
  Reanudar con un objetivo activo
</h3>

Cuando reanuda una sesión, Claude Code restaura un objetivo que aún estaba activo cuando terminó la sesión. Claude Code lo restaura en cada ruta de reanudación: `--continue`, `--resume` con un ID de sesión, nombre o [ruta de archivo de transcripción](/docs/es/sessions#resume-a-session), y el [selector de sesión](/docs/es/sessions#use-the-session-picker). Antes de v2.1.239, Claude Code restauraba el objetivo en cada ruta excepto el selector `claude --resume`.

Claude Code lleva la condición pero reinicia el recuento de turnos, el temporizador y la línea de base de gasto de tokens. No restaura un objetivo que ya se logró o se borró.

<h3 id="run-non-interactively">
  Ejecutar de forma no interactiva
</h3>

`/goal` funciona en [modo no interactivo](/docs/es/headless), en la [aplicación de escritorio](/docs/es/desktop) y a través de [Control Remoto](/docs/es/remote-control). Establecer un objetivo con `-p` ejecuta el bucle hasta su finalización en una sola invocación:

```bash theme={null}
claude -p "/goal CHANGELOG.md has an entry for every PR merged this week"
```

Con la salida de texto predeterminada, nada se imprime hasta que se termine la ejecución, por lo que un objetivo que se ejecuta muchos turnos puede parecer atascado. Agregue `--output-format stream-json --verbose` para emitir cada mensaje mientras se ejecuta el bucle.

Interrumpa el proceso con Ctrl+C para detener un objetivo no interactivo antes de que se resuelva.

<h2 id="how-evaluation-works">
  Cómo funciona la evaluación
</h2>

`/goal` es un envoltorio alrededor de un [Stop hook basado en solicitud](/docs/es/hooks#prompt-based-hooks) con alcance de sesión. Cada vez que Claude termina un turno, Claude Code envía la condición y la conversación hasta ahora a su [modelo pequeño y rápido](/docs/es/model-config) configurado, que por defecto es Haiku en la API de Claude; en un proveedor de terceros, consulte su [página de proveedor](/docs/es/third-party-integrations) para conocer el valor predeterminado de la plataforma. El modelo devuelve uno de tres veredictos, cada uno con una breve razón:

* **Aún no se cumple**: Claude sigue trabajando y toma la razón como orientación para el siguiente turno.
* **Cumplido**: Claude Code borra el objetivo y registra una entrada lograda en la transcripción.
* **Imposible**: el evaluador juzgó que la condición nunca puede satisfacerse. Claude Code borra el objetivo y registra una entrada fallida en la transcripción junto con la razón. No necesita borrarla usted mismo.

Si Claude sigue respondiendo al evaluador sin hacer progreso (sin uso de herramientas durante varios turnos seguidos), Claude Code detiene el bucle, imprime una advertencia y devuelve el control a usted con el objetivo aún establecido. La evaluación se reanuda después de su siguiente solicitud. La [guía de hooks](/docs/es/hooks-guide#stop-hook-hits-the-block-cap) explica el mecanismo subyacente.

<h3 id="when-a-turn-fails">
  Cuando un turno falla
</h3>

Cuando un turno falla, Claude Code borra el objetivo si el error es uno que usted tiene que corregir. Después de cualquier otro error, el objetivo permanece establecido.

<h4 id="errors-you-have-to-fix-clear-the-goal">
  Los errores que debe corregir borran el objetivo
</h4>

Si un turno falla en un error que no se borrará hasta que lo corrija, Claude Code borra el objetivo e imprime una advertencia nombrando la causa. La advertencia comienza con `Goal cleared after an unrecoverable error` y termina con `Run /goal again to continue`. Corrija la causa, luego [establezca el objetivo nuevamente](#set-a-goal) con `/goal <condition>`. Cuatro tipos de fallo borran el objetivo:

* Un fallo de autenticación, cuando Claude Code gestiona sus propias credenciales. Cuando un host las gestiona por usted, como la aplicación de escritorio, la extensión de VS Code o una [sesión en la nube](/docs/es/claude-code-on-the-web), Claude Code deja el objetivo activo porque el host restaura el acceso por su cuenta.
* Un saldo de crédito agotado
* Un desbordamiento de contexto que [auto-compact](/docs/es/model-config#set-the-auto-compact-window) no pudo borrar
* Un modelo que no está disponible

<h4 id="other-errors-retry-or-pause-the-goal">
  Otros errores reintentan o pausan el objetivo
</h4>

Después de cualquier otro fallo, el objetivo permanece establecido. En una sesión interactiva en Claude Code v2.1.269 o posterior, Claude Code también imprime una línea nombrando la causa e intenta de nuevo por su cuenta o espera a usted:

* **Reintentar**: después de un fallo que tiende a resolverse por sí solo, como un servidor sobrecargado o una conexión perdida, un aviso que comienza con `Goal still active` muestra la espera antes del siguiente intento. Después de tres reintentos automáticos, el objetivo se pausa en su lugar.
* **Pausar**: después de un fallo que un reintento solo repetiría, como un límite de velocidad de API, un [límite de uso](/docs/es/errors#youve-hit-your-session-limit) de claude.ai, o un hook que terminó el turno, un aviso que comienza con `Goal paused` nombra la causa. Si la sesión está [esperando continuar automáticamente cuando se restablece un límite de uso](/docs/es/interactive-mode#wait-for-a-usage-limit-to-reset), Claude reanuda el trabajo hacia el objetivo entonces.

Envíe un mensaje en cualquier momento para iniciar el siguiente turno inmediatamente. Para desactivar los reintentos automáticos, establezca [`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](/docs/es/env-vars) en `0`, lo que también desactiva las [verificaciones](#background-work-defers-evaluation).

<h3 id="background-work-defers-evaluation">
  El trabajo en segundo plano difiere la evaluación
</h3>

Si un subagente o un comando de shell en segundo plano aún se está ejecutando cuando termina un turno, Claude Code omite la evaluación para ese turno. Evalúa al final del siguiente turno que termina sin trabajo en segundo plano ejecutándose. Cuando el trabajo en segundo plano termina, Claude Code entrega el resultado a Claude como un nuevo turno, por lo que no tiene que solicitar.

Una vez que el trabajo en segundo plano ha mantenido el objetivo esperando durante 30 minutos, se debe una verificación. En la verificación, Claude Code enumera las tareas en ejecución y le pide a Claude que lea su salida, siga esperando si están progresando, y corrija o detenga cualquiera que esté atascada. Después de la primera verificación, Claude Code espera el doble de tiempo antes de cada verificación posterior, hasta cuatro veces el primer intervalo: con el valor predeterminado, 1 hora después de la primera verificación, luego cada 2 horas. Claude Code entrega una verificación vencida, la primera incluida, de una de dos maneras:

* **Cuando termina un turno**: Claude Code entrega la verificación al final del siguiente turno que termina con el trabajo aún en ejecución. En una sesión no interactiva, como una iniciada con `-p`, esta es la única forma en que Claude Code entrega verificaciones.
* **Mientras la sesión está inactiva**: en una sesión interactiva, Claude Code también inicia un turno por su cuenta para entregar la verificación en lugar de esperar su siguiente solicitud. Si el trabajo en segundo plano se ha detenido sin reportar un resultado, Claude Code le pide a Claude que continúe hacia el objetivo. Claude Code inicia como máximo tres verificaciones inactivas por objetivo entre sus solicitudes. En la tercera verificación inactiva, Claude Code dice que las verificaciones inactivas se pausan hasta que envíe otra solicitud. Antes de v2.1.246, las verificaciones inactivas eran ilimitadas. Las verificaciones inactivas requieren Claude Code v2.1.236 o posterior.

Antes de v2.1.239, solo las verificaciones inactivas se retrasaban de esta manera; una verificación entregada al final de un turno se repetía en el primer intervalo.

Para cambiar el primer intervalo, establezca [`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](/docs/es/env-vars). Claude Code usa su valor en lugar del intervalo de 30 minutos y escala los intervalos posteriores con él. Establézcalo en `0` para desactivar las verificaciones y los [reintentos automáticos](#other-errors-retry-or-pause-the-goal).

Las verificaciones requieren Claude Code v2.1.234 o posterior.

<h3 id="evaluation-model-and-cost">
  Modelo de evaluación y costo
</h3>

Para evaluar en un modelo diferente, establezca [`ANTHROPIC_DEFAULT_HAIKU_MODEL`](/docs/es/model-config#environment-variables).

<Warning>
  Claude Code lee `ANTHROPIC_DEFAULT_HAIKU_MODEL` en todas partes donde usa el modelo pequeño y rápido, no solo para la evaluación de `/goal`. Cuando lo establece, Claude Code también resuelve el [alias `haiku`](/docs/es/model-config#model-aliases) a ese modelo y ejecuta [funcionalidad en segundo plano](/docs/es/costs#background-token-usage), como la resumición de conversaciones, en él.
</Warning>

El evaluador se ejecuta en cualquier proveedor para el que esté configurada su sesión. No llama a herramientas, por lo que solo puede juzgar lo que Claude ya ha presentado en la conversación.

<Note>
  Los tokens de evaluación se facturan en el modelo pequeño y rápido configurado para su proveedor y son típicamente insignificantes en comparación con el gasto de turno principal.
</Note>

<h2 id="requirements">
  Requisitos
</h2>

Claude Code pone `/goal` disponible bajo la misma [regla de confianza del espacio de trabajo que los hooks en archivos de configuración](/docs/es/permissions#what-runs-before-you-trust-a-folder), porque el evaluador es parte del sistema de hooks. `/goal` también no está disponible cuando [`disableAllHooks`](/docs/es/hooks#disable-or-remove-hooks) es `true` después de que se aplica la precedencia de configuración, o cuando [`allowManagedHooksOnly`](/docs/es/settings-reference#allowmanagedhooksonly) se establece en la configuración administrada. En cada caso, el comando le indica por qué en lugar de no hacer nada silenciosamente.

<h2 id="see-also">
  Ver también
</h2>

* [Ejecutar una solicitud repetidamente con `/loop`](/docs/es/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop): volver a ejecutar en un intervalo de tiempo en lugar de hasta que se cumpla una condición
* [Hooks basados en solicitud](/docs/es/hooks-guide#prompt-based-hooks): escriba su propio Stop hook cuando necesite lógica de evaluación personalizada
* [Modo automático](/docs/es/auto-mode-config): apruebe llamadas de herramientas automáticamente para que cada turno de objetivo se ejecute sin supervisión
* [Comparación de programación](/docs/es/scheduled-tasks#compare-scheduling-options): ejecute trabajo en un horario independiente de cualquier sesión abierta
