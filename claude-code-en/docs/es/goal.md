> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Mantener a Claude trabajando hacia un objetivo

> Establezca una condición de finalización con /goal y Claude seguirá trabajando entre turnos hasta que se cumpla la condición.

<Note>
  `/goal` requiere Claude Code v2.1.139 o posterior.
</Note>

El comando `/goal` establece una condición de finalización y Claude sigue trabajando hacia ella sin que usted solicite cada paso. Después de cada turno, un modelo pequeño y rápido verifica si se cumple la condición. Si no es así, Claude inicia otro turno en lugar de devolver el control a usted. El objetivo se borra automáticamente una vez que se cumple la condición.

Utilice un objetivo para trabajo sustancial con un estado final verificable:

* Migrar un módulo a una nueva API hasta que cada sitio de llamada se compile y las pruebas pasen
* Implementar un documento de diseño hasta que se cumplan todos los criterios de aceptación
* Dividir un archivo grande en módulos enfocados hasta que cada uno esté dentro de un presupuesto de tamaño
* Trabajar a través de un backlog de problemas etiquetados hasta que la cola esté vacía

<h2 id="compare-ways-to-keep-a-session-running">
  Comparar formas de mantener una sesión en ejecución
</h2>

Tres enfoques mantienen la sesión actual en ejecución entre solicitudes. Elija según lo que deba iniciar el siguiente turno:

| Enfoque                                                             | El siguiente turno comienza cuando | Se detiene cuando                                           |
| :------------------------------------------------------------------ | :--------------------------------- | :---------------------------------------------------------- |
| `/goal`                                                             | El turno anterior finaliza         | Un modelo confirma que se cumple la condición               |
| [`/loop`](/docs/es/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop) | Transcurre un intervalo de tiempo  | Usted lo detiene, o Claude decide que el trabajo está hecho |
| [Stop hook](/docs/es/hooks-guide#prompt-based-hooks)                     | El turno anterior finaliza         | Su propio script o solicitud decide                         |

`/goal` y un Stop hook se activan después de cada turno. `/goal` es un atajo con alcance de sesión: escribe una condición y está activa solo para la sesión actual. Un Stop hook vive en su archivo de configuración, se aplica a cada sesión en su alcance y puede ejecutar un script para verificaciones deterministas o una solicitud para evaluaciones basadas en modelos.

[El modo automático](/docs/es/auto-mode-config) por sí solo aprueba llamadas de herramientas dentro de un único turno pero no inicia uno nuevo. Claude se detiene cuando juzga que el trabajo está hecho. `/goal` añade un evaluador separado que verifica su condición después de cada turno, por lo que la finalización es decidida por un modelo nuevo en lugar del que realiza el trabajo. Los dos son complementarios: el modo automático elimina solicitudes por herramienta, y `/goal` elimina solicitudes por turno.

<Tip>
  Los enfoques anteriores mantienen la sesión actual en ejecución. También puede programar trabajo que se ejecute independientemente de cualquier sesión abierta, como pruebas nocturnas o triaje matutino. Consulte [opciones de programación](/docs/es/scheduled-tasks#compare-scheduling-options) para rutinas en la nube y tareas programadas de escritorio.
</Tip>

<h2 id="use-/goal">
  Usar `/goal`
</h2>

Un objetivo puede estar activo por sesión. El mismo comando lo establece, verifica y borra según el argumento.

<h3 id="set-a-goal">
  Establecer un objetivo
</h3>

Ejecute `/goal` seguido de la condición que desea que se cumpla. Si ya hay un objetivo activo, el nuevo lo reemplaza.

```text theme={null}
/goal all tests in test/auth pass and the lint step is clean
```

Establecer un objetivo inicia un turno inmediatamente, con la condición misma como directiva. No necesita enviar una solicitud separada. Mientras el objetivo está activo, un indicador `◎ /goal active` muestra cuánto tiempo ha estado ejecutándose el objetivo.

Un objetivo no cambia los permisos. En el modo de permiso predeterminado, Claude aún pregunta antes de las llamadas a herramientas que su configuración no permite, como el comando de prueba anterior. Para permitir que los turnos de objetivo se ejecuten sin supervisión, combine `/goal` con [modo automático](/docs/es/auto-mode-config).

Después de cada turno, el evaluador devuelve una breve razón explicando por qué se cumple o no se cumple la condición. La razón más reciente aparece en la vista de estado y en la transcripción para que pueda ver hacia qué está trabajando Claude a continuación.

<Note>
  Un objetivo sigue ejecutándose hasta que se cumpla la condición o ejecute `/goal clear`. Ejecute `/goal` sin argumento para ver los turnos y tokens gastados hasta ahora.
</Note>

<h3 id="write-an-effective-condition">
  Escribir una condición efectiva
</h3>

El [evaluador](#how-evaluation-works) juzga su condición contra lo que Claude ha presentado en la conversación. No ejecuta comandos ni lee archivos de forma independiente, así que escriba la condición como algo que la salida propia de Claude pueda demostrar. "Todas las pruebas en `test/auth` pasan" funciona porque Claude ejecuta las pruebas y el resultado aparece en la transcripción para que el evaluador lo lea.

Una condición que se mantiene en muchos turnos generalmente tiene:

* **Un estado final medible**: un resultado de prueba, un código de salida de compilación, un recuento de archivos, una cola vacía
* **Una verificación establecida**: cómo Claude debe probarlo, como "`npm test` sale 0" o "`git status` está limpio"
* **Restricciones que importan**: cualquier cosa que no deba cambiar en el camino, como "ningún otro archivo de prueba se modifica"

La condición puede tener hasta 4.000 caracteres.

Para limitar cuánto tiempo se ejecuta un objetivo, incluya una cláusula de turno o tiempo en la condición, como `or stop after 20 turns`. Claude reporta progreso contra esa cláusula cada turno y el evaluador la juzga desde la conversación.

<h3 id="check-status">
  Verificar estado
</h3>

Ejecute `/goal` sin argumentos para ver el estado actual.

```text theme={null}
/goal
```

Si un objetivo está activo, el estado muestra:

* La condición
* Cuánto tiempo ha estado ejecutándose
* Cuántos turnos han sido evaluados
* El gasto de tokens actual
* La razón más reciente del evaluador

El recuento de turnos y la razón más reciente aparecen después de que se haya ejecutado la primera evaluación.

Si no hay un objetivo activo pero se logró uno anteriormente en la sesión, el estado muestra la condición lograda junto con su duración, recuento de turnos y gasto de tokens.

<h3 id="clear-a-goal">
  Borrar un objetivo
</h3>

Ejecute `/goal clear` para eliminar un objetivo activo antes de que se cumpla su condición.

```text theme={null}
/goal clear
```

Claude imprime `Goal cleared:` seguido de la condición para confirmar, o `No goal set` si nada estaba activo.

`stop`, `off`, `reset`, `none` y `cancel` se aceptan como alias para `clear`. Ejecutar `/clear` para iniciar una nueva conversación también elimina cualquier objetivo activo.

<h3 id="resume-with-an-active-goal">
  Reanudar con un objetivo activo
</h3>

Un objetivo que aún estaba activo cuando terminó una sesión se restaura cuando reanuda esa sesión con `--resume` o `--continue`. La condición se mantiene, pero el recuento de turnos, el temporizador y la línea de base de gasto de tokens se reinician al reanudar. Un objetivo que ya se logró o se borró no se restaura.

<h3 id="run-non-interactively">
  Ejecutar de forma no interactiva
</h3>

`/goal` funciona en [modo no interactivo](/docs/es/headless), en la [aplicación de escritorio](/docs/es/desktop) y a través de [Control Remoto](/docs/es/remote-control). Establecer un objetivo con `-p` ejecuta el bucle hasta completarse en una única invocación:

```bash theme={null}
claude -p "/goal CHANGELOG.md has an entry for every PR merged this week"
```

Con el formato de salida de texto predeterminado, nada se imprime hasta que se cumpla la condición, por lo que un objetivo que se ejecuta muchos turnos puede parecer atascado. Agregue `--output-format stream-json --verbose` para emitir cada mensaje mientras se ejecuta el bucle.

Interrumpa el proceso con Ctrl+C para detener un objetivo no interactivo antes de que se cumpla la condición.

<h2 id="how-evaluation-works">
  Cómo funciona la evaluación
</h2>

`/goal` es un envoltorio alrededor de un [Stop hook basado en solicitud](/docs/es/hooks#prompt-based-hooks) con alcance de sesión. Cada vez que Claude termina un turno, la condición y la conversación hasta ahora se envían a su [modelo pequeño y rápido](/docs/es/model-config) configurado, que por defecto es Haiku. El modelo devuelve una decisión sí o no y una breve razón. Un "no" le dice a Claude que siga trabajando e incluye la razón como orientación para el siguiente turno. Un "sí" borra el objetivo y registra una entrada lograda en la transcripción.

El evaluador se ejecuta en cualquier proveedor para el que esté configurada su sesión. No llama a herramientas, por lo que solo puede juzgar lo que Claude ya ha presentado en la conversación.

<Note>
  Los tokens de evaluación se facturan en el modelo pequeño y rápido configurado para su proveedor y son típicamente insignificantes en comparación con el gasto de turno principal.
</Note>

<h2 id="requirements">
  Requisitos
</h2>

`/goal` se ejecuta solo en espacios de trabajo donde ha aceptado el diálogo de confianza, porque el evaluador es parte del sistema de hooks. `/goal` también no está disponible cuando [`disableAllHooks`](/docs/es/hooks#disable-or-remove-hooks) se establece en cualquier nivel de configuración o cuando [`allowManagedHooksOnly`](/docs/es/settings#hook-configuration) se establece en la configuración administrada. En cada caso, el comando le indica por qué en lugar de no hacer nada silenciosamente.

<h2 id="see-also">
  Ver también
</h2>

* [Ejecutar una solicitud repetidamente con `/loop`](/docs/es/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop): volver a ejecutar en un intervalo de tiempo en lugar de hasta que se cumpla una condición
* [Hooks basados en solicitud](/docs/es/hooks-guide#prompt-based-hooks): escriba su propio Stop hook cuando necesite lógica de evaluación personalizada
* [Modo automático](/docs/es/auto-mode-config): apruebe llamadas de herramientas automáticamente para que cada turno de objetivo se ejecute sin supervisión
* [Comparación de programación](/docs/es/scheduled-tasks#compare-scheduling-options): ejecute trabajo en un horario independiente de cualquier sesión abierta
