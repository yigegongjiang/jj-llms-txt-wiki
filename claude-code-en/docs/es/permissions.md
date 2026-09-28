> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurar permisos

> Controle lo que Claude Code puede acceder y hacer con reglas de permisos granulares, modos y políticas administradas.

Claude Code admite permisos granulares para que pueda especificar exactamente qué puede hacer el agente y qué no puede hacer. La configuración de permisos se puede registrar en el control de versiones y distribuir a todos los desarrolladores de su organización, así como personalizarse por desarrolladores individuales.

<h2 id="permission-system">
  Sistema de permisos
</h2>

Claude Code utiliza un sistema de permisos escalonado para equilibrar potencia y seguridad. La tabla muestra, para cada tipo de herramienta, si el modo Manual solicita aprobación antes de que se ejecute la acción. Los otros [modos de permisos](#permission-modes) cambian cuál de estos le solicita; en modo automático un clasificador revisa las acciones en lugar de usted, y [cómo el clasificador evalúa las acciones](/docs/es/permission-modes#how-the-classifier-evaluates-actions) enumera cuáles ve.

| Tipo de herramienta      | Ejemplo                    | Se requiere aprobación                                                                                                    | Comportamiento de "Sí, no preguntar de nuevo" |
| :----------------------- | :------------------------- | :------------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------- |
| Solo lectura             | Lecturas de archivos, Grep | No, dentro del [directorio de trabajo y directorios adicionales](#working-directories)                                    | N/A                                           |
| Comandos Bash            | Ejecución de shell         | Sí, excepto un conjunto integrado de [comandos de solo lectura](#read-only-commands)                                      | Permanentemente por repositorio y comando     |
| Modificación de archivos | Editar/escribir archivos   | Sí                                                                                                                        | Hasta el final de la sesión                   |
| Obtención web            | WebFetch                   | Sí, excepto un conjunto integrado de [dominios de documentación preaprobados](/docs/es/tools-reference#webfetch-tool-behavior) | Permanentemente por repositorio y dominio     |
| Búsqueda web             | WebSearch                  | Sí                                                                                                                        | Permanentemente por repositorio               |

Cuando elige "Sí, no preguntar de nuevo" y la aprobación se guarda permanentemente, como para un comando Bash o un dominio WebFetch, Claude Code guarda la regla en `.claude/settings.local.json` en la raíz del repositorio de git, resuelto a través de [worktrees](/docs/es/worktrees) al checkout principal. La regla se aplica a futuras sesiones en cualquier lugar de ese repositorio, incluidas las sesiones iniciadas en subdirectorios y en worktrees. Una aprobación de modificación de archivo no se guarda en el archivo: como muestra la tabla, dura hasta que termina la sesión. En algunos casos, como fuera de un repositorio de git o en Windows, Claude Code no utiliza la raíz del repositorio; [Dónde Claude Code busca cada archivo](/docs/es/settings#where-claude-code-looks-for-each-file) enumera esos casos y dónde guarda la regla en su lugar.

Antes de v2.1.211, Claude Code siempre guardaba la regla en el directorio de inicio, por lo que una aprobación otorgada en un worktree o subdirectorio no se aplicaba al resto del repositorio. Las reglas que las versiones anteriores guardaron en un subdirectorio o worktree aún se aplican a las sesiones iniciadas allí.

A veces un mensaje de permiso ofrece solo una aprobación única, sin opción de "no preguntar de nuevo" y sin opción de permitir la acción para el resto de la sesión. Claude Code ofrece esas opciones solo cuando el mensaje puede mostrarle todo lo que permitirían, por lo que una regla que guarde desde un mensaje cubre solo lo que su opción nombrada. Cuando el mensaje ofrece solo la aprobación única, apruebe la acción una vez, o agregue la regla usted mismo en [`/permissions`](#manage-permissions).

<h3 id="add-a-comment-when-you-answer-a-permission-prompt">
  Agregue un comentario cuando responda a un mensaje de permiso
</h3>

Puede adjuntar una nota a Claude cuando aprueba o deniega una única acción. En la mayoría de los mensajes de permiso, incluidos los mensajes de Bash, PowerShell, archivo y herramienta MCP, muévase a **Sí** o **No** y presione `Tab` para abrir un campo de comentario en esa opción. Los mensajes de WebFetch y navegador no ofrecen el campo. Las opciones que permiten la acción para el resto de la sesión o guardan una regla tampoco toman una.

Con el campo abierto, escriba el comentario y luego presione una de estas teclas:

* `Enter`: envía su respuesta con el comentario adjunto. Si deja el campo vacío, Claude Code envía la respuesta sin un comentario.
* `Tab`: cierra el campo sin responder. Claude Code mantiene el texto que escribió y aún lo envía si responde con esa opción.
* `Shift+Tab`: en un mensaje de archivo, como un mensaje de Edit o Write, cierra el campo igual que `Tab`. Antes de v2.1.235, presionar `Shift+Tab` dentro del campo en su lugar seleccionaba la opción que permite la acción para el resto de la sesión, por lo que Claude Code aprobaba la acción para el resto de la sesión y descartaba el comentario.

Claude Code entrega el comentario de manera diferente según cómo haya respondido:

* **Sí**: Claude Code ejecuta la acción, luego envía su comentario a Claude después del resultado.
* **No**: Claude Code envía su comentario a Claude como la razón de la denegación, y Claude continúa trabajando. Si selecciona **No** sin un comentario en un mensaje de la conversación principal, Claude Code detiene el turno.

<h2 id="manage-permissions">
  Administrar permisos
</h2>

Puede ver y administrar los permisos de herramientas de Claude Code con `/permissions`. Esta interfaz de usuario enumera todas las reglas de permisos y el archivo `settings.json` del que se obtienen. Puede abrir esta interfaz mientras Claude está trabajando: cuando agrega o elimina una regla, Claude Code aplica el cambio a partir de la siguiente llamada de herramienta de Claude en el mismo turno. Antes de v2.1.234, Claude Code ponía en cola el comando hasta que el turno finalizaba.

* Las reglas **Allow** permiten que Claude Code use la herramienta especificada sin aprobación manual.
* Las reglas **Ask** solicitan confirmación cada vez que Claude Code intenta usar la herramienta especificada.
* Las reglas **Deny** impiden que Claude Code use la herramienta especificada.

Las reglas se evalúan en orden: deny, luego ask, luego allow. La primera coincidencia en ese orden determina el resultado, y la especificidad de la regla no cambia el orden.

Una regla deny amplia como `Bash(aws *)` bloquea cada llamada coincidente, incluidas las llamadas que también coinciden con una regla allow más específica como `Bash(aws s3 ls)`. Una regla allow no puede crear una excepción dentro de una regla deny. La misma precedencia se aplica entre ask y allow: una regla ask coincidente solicita confirmación incluso cuando una regla allow más específica también coincide con la misma llamada.

Las reglas deny se comportan de manera diferente dependiendo de si nombran una herramienta o delimitan un patrón dentro de una. Un nombre de herramienta simple como `Bash` elimina la herramienta del contexto de Claude por completo, por lo que Claude nunca la ve. Si agrega tal regla a mitad de sesión, Claude no puede llamar a la herramienta desde su siguiente llamada de herramienta en adelante; [Denying an entire tool](/docs/es/prompt-caching#denying-an-entire-tool) cubre lo que sucede con una definición que Claude ya ha visto. Una regla delimitada como `Bash(rm *)` deja la herramienta disponible y bloquea las llamadas coincidentes cuando Claude intenta usarlas.

La eliminación de nombre simple se aplica a todas las herramientas excepto [`EndConversation`](/docs/es/tools-reference#endconversation-tool-behavior): una regla deny no puede eliminarla mientras permanezca cualquier otra herramienta, y una regla ask nunca solicita confirmación para ella.

<Note>
  Las reglas de permisos se aplican mediante Claude Code, no por el modelo. Las instrucciones en su prompt o `CLAUDE.md` determinan lo que Claude intenta hacer, pero no cambian lo que Claude Code permite. Para otorgar o revocar acceso, use `/permissions`, las reglas descritas aquí, un [modo de permisos](/docs/es/permission-modes), o un [hook PreToolUse](#extend-permissions-with-hooks).
</Note>

Cuando el [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) está disponible para su sesión, la interfaz de usuario también incluye las [reglas del clasificador de modo automático](/docs/es/auto-mode-config#edit-rules-from-permissions). Seleccione la pestaña **Auto mode** para verlas.

<h2 id="permission-modes">
  Modos de permisos
</h2>

Claude Code admite varios modos de permisos que controlan cómo se aprueban las llamadas de herramientas. Consulte [Modos de permisos](/docs/es/permission-modes) para saber cuándo usar cada uno. Para cambiar el modo en el que comienzan las sesiones, establezca `defaultMode` en sus [archivos de configuración](/docs/es/settings#where-settings-live). [Qué modo comienza una sesión](/docs/es/permission-modes#which-mode-a-session-starts-in) cubre el valor predeterminado integrado para cada plan y lo que lee la extensión de VS Code.

| Modo                | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Solicita permiso en el primer uso de cada herramienta. Etiquetado como Manual en la CLI, en las extensiones de VS Code y JetBrains, y en la aplicación de escritorio, y Claude Code acepta `manual` como un alias. La etiqueta y el alias requieren Claude Code v2.1.200 o posterior. La etiqueta de la aplicación de escritorio no depende de su versión de CLI                                                                                                                                                                                                                                                                       |
| `acceptEdits`       | Acepta automáticamente ediciones de archivos y comandos comunes del sistema de archivos como `mkdir`, `touch`, `mv` y `cp` para rutas en el directorio de trabajo o `additionalDirectories`                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `plan`              | Claude lee archivos y ejecuta comandos de shell de solo lectura para explorar pero no edita sus archivos de origen; con [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) disponible, los comandos aprobados por clasificador también se ejecutan. Etiquetado como Plan en la CLI y en la extensión de VS Code                                                                                                                                                                                                                                                                                                  |
| `auto`              | Auto-aprueba llamadas de herramientas con comprobaciones de seguridad en segundo plano que verifican que las acciones se alineen con su solicitud                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `dontAsk`           | Auto-deniega cada llamada que de otro modo solicitaría permiso; las lecturas de archivos en sus directorios de trabajo y otras acciones que no necesitan aprobación aún se ejecutan, al igual que las herramientas preaprobadas a través de `/permissions` o reglas `permissions.allow`. `AskUserQuestion`, herramientas MCP marcadas [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool), y herramientas de conector [que su organización configuró en `ask`](/docs/es/mcp#organization-controls-on-connector-tools) en sesiones donde esa configuración llega a Claude Code se deniegan incluso si las ha permitido |
| `bypassPermissions` | Omite avisos de permisos, excepto para las [acciones que ningún modo auto-aprueba](/docs/es/permission-modes#actions-no-mode-auto-approves)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |

<Warning>
  En modo `bypassPermissions`, Claude Code omite avisos de permisos, incluyendo escrituras en [rutas protegidas](/docs/es/permission-modes#protected-paths) como `.git` y `.claude`. Las [salvaguardas de mensajería entre sesiones](/docs/es/permission-modes#skip-all-checks-with-bypasspermissions-mode) aún se aplican. Use este modo solo en entornos aislados como contenedores o máquinas virtuales donde Claude Code no pueda causar daño.
</Warning>

Para evitar que se use el modo `bypassPermissions` o `auto`, establezca `permissions.disableBypassPermissionsMode` o `permissions.disableAutoMode` en `"disable"` en cualquier [archivo de configuración](/docs/es/settings#where-settings-live). Estos son más útiles en [configuración administrada](#managed-settings) donde no pueden ser anulados.

<h2 id="permission-rule-syntax">
  Sintaxis de reglas de permisos
</h2>

Las reglas de permisos siguen el formato `Tool` o `Tool(specifier)`. Los paréntesis dentro del especificador son literales, por lo que un comando o ruta que los contiene no necesita escape.

<h3 id="match-all-uses-of-a-tool">
  Coincidir con todos los usos de una herramienta
</h3>

Para coincidir con todos los usos de una herramienta, use solo el nombre de la herramienta sin paréntesis:

| Regla      | Efecto                                              |
| :--------- | :-------------------------------------------------- |
| `Bash`     | Coincide con todos los comandos Bash                |
| `WebFetch` | Coincide con todas las solicitudes de obtención web |
| `Read`     | Coincide con todas las lecturas de archivos         |

`Bash(*)` es equivalente a `Bash` y coincide con todos los comandos Bash. Como regla de denegación, ambas formas eliminan la herramienta del contexto de Claude.

<h3 id="use-specifiers-for-fine-grained-control">
  Usar especificadores para control granular
</h3>

Agregue un especificador entre paréntesis para coincidir con usos específicos de herramientas:

| Regla                          | Efecto                                                             |
| :----------------------------- | :----------------------------------------------------------------- |
| `Bash(npm run build)`          | Coincide con el comando exacto `npm run build`                     |
| `Read(./.env)`                 | Coincide con la lectura del archivo `.env` en el directorio actual |
| `WebFetch(domain:example.com)` | Coincide con solicitudes de obtención a example.com                |

<h3 id="match-by-input-parameter">
  Coincidir por parámetro de entrada
</h3>

Las reglas de denegación y solicitud pueden coincidir con un parámetro de entrada de nivel superior en cualquier herramienta integrada con `Tool(param:value)`.

Para coincidir con un parámetro en una herramienta MCP, pase una regla de denegación con [`--disallowedTools`](/docs/es/cli-reference#cli-flags). Cuando Claude Code carga un archivo de configuración, omite cualquier regla `mcp__` que tenga paréntesis. Claude Code enumera la regla omitida en el diálogo de configuración inválida cuando comienza una sesión interactiva, y en la salida de [`claude doctor`](/docs/es/debug-your-config#check-resolved-settings).

Una regla de parámetro coincide cuando Claude llama a la herramienta con ese parámetro establecido en ese valor exacto. Una regla de permiso para un valor de parámetro no establecería que la llamada sea segura en general, por lo que las reglas de permiso continúan usando la sintaxis de especificador propia de cada herramienta. Esto funciona para cualquier parámetro escalar que acepte la herramienta:

| Regla                          | Coincide                                                |
| :----------------------------- | :------------------------------------------------------ |
| `Agent(model:opus)`            | Llamadas de Agent que solicitan el nivel de modelo Opus |
| `Agent(isolation:worktree)`    | Llamadas de Agent que solicitan un git worktree         |
| `Bash(run_in_background:true)` | Llamadas de Bash que se ejecutan en segundo plano       |

La coincidencia de parámetros sigue estas reglas:

* El nombre del parámetro debe ser un campo directo de la entrada de la herramienta, como `model` en la herramienta Agent. Los campos anidados dentro de un objeto o matriz no son coincidentes
* Cada regla nombra un parámetro. Para controlar tanto `model` como `isolation`, escriba dos reglas, `Agent(model:opus)` y `Agent(isolation:worktree)`, en lugar de combinarlas en una regla
* El valor admite `*` como un comodín que coincide con cualquier secuencia de caracteres, por lo que `Agent(isolation:*)` coincide con cualquier valor de aislamiento explícito. Sin `*` la coincidencia es exacta
* Un parámetro que el modelo omite nunca se coincide, por lo que `Agent(model:*)` no coincide con una llamada que deja `model` sin establecer
* El valor se compara con la entrada literal que Claude envía, antes de cualquier normalización. `Agent(model:opus)` coincide con el alias `opus` pero no con un ID de modelo completo. Ejecute con [`--verbose`](/docs/es/cli-reference) para ver los nombres y valores exactos de los parámetros en cada llamada de herramienta
* Se ignora el espacio en blanco alrededor de los dos puntos

No puede coincidir con el campo de contenido principal de una herramienta de esta manera: `command` para Bash y PowerShell, `file_path` para Read, Edit y Write, `path` para Grep y Glob, `notebook_path` para NotebookEdit, y `url` para WebFetch. Una regla como `Bash(command:rm *)` sería eludible por un comando compuesto, por lo que Claude Code la ignora y emite una advertencia de inicio. Use `Bash(rm *)`, `Read(./path)`, o `WebFetch(domain:host)` en su lugar.

<h3 id="wildcard-patterns">
  Patrones de comodín
</h3>

Un `*` en una regla de Bash coincide con cualquier texto, incluidos espacios, por lo que una regla cubre una familia de comandos. Una regla sin `*` coincide con un comando exacto.

<Warning>
  Ponga el `*` después del subcomando. En `git log --oneline main`, `git` es el programa y `log` es el subcomando, la palabra que determina qué hace el programa. Claude Code coincide con todo lo anterior al primer `*` tal como está escrito, por lo que esas palabras son lo que limita la regla: `Bash(git log *)` permite solo comandos `git log`, y `Bash(git *)` permite todos los comandos git. Claude Code [advierte al inicio](/docs/es/errors#has-a-wildcard-before-the-rest-of-the-command) sobre una regla de permiso con un `*` antes del subcomando, como `Bash(git * main)`.
</Warning>

Escriba el comando que desea que Claude ejecute sin preguntar, y reemplace las partes que varían con `*`. Con esta configuración, Claude Code ejecuta scripts npm y confirmaciones git sin preguntar y rechaza comandos que comienzan con `git push`. Un push escrito de otra manera, como `git -C . push`, no coincide; vea [lo que una regla de Bash no coincide](#bash-rule-limits).

```json theme={null}
{
  "permissions": {
    "allow": [
      "Bash(npm run *)",
      "Bash(git commit *)"
    ],
    "deny": [
      "Bash(git push *)"
    ]
  }
}
```

Un `*` puede ir en cualquier lugar de la regla: al inicio, en el medio o al final. Cada fila muestra una regla, comandos que coincide, y comandos cercanos que no coincide:

| Usted escribe          | Coincide                                                                             | No coincide                            |
| :--------------------- | :----------------------------------------------------------------------------------- | :------------------------------------- |
| `Bash(npm run build)`  | `npm run build`                                                                      | `npm run build --watch`                |
| `Bash(npm run *)`      | `npm run build`, `npm run test --watch`, `npm run`                                   | `npm install`                          |
| `Bash(git log * main)` | `git log --oneline main`, `git log -5 main`, `git log --output=<file> main`          | `git log main`, `git push origin main` |
| `Bash(git * main)`     | `git merge main`, `git push origin main`, `git -c core.fsmonitor=<script> diff main` | `git log`                              |
| `Bash(* --version)`    | `node --version`, `bash -c 'echo hi' --version`                                      | `node -v`                              |
| `Bash(ls *)`           | `ls -la`, `ls`                                                                       | `lsof`                                 |
| `Bash(ls*)`            | `ls -la`, `lsof`                                                                     |                                        |
| `Bash(* --help *)`     | `npm --help x`                                                                       | `npm --help`                           |

Tres reglas de coincidencia producen esas filas:

* **El `*` representa cualquier texto que esté en su lugar.** En `Bash(git * main)`, representa el subcomando, por lo que Claude Code coincide con todos los subcomandos git y todas las opciones anteriores. Esto incluye `-c`, que hace que git ejecute un programa que usted nombre. En `Bash(* --version)`, el `*` representa el programa, por lo que cualquier programa coincide.
* **Un `*` al final, con un espacio antes, también coincide con el comando desnudo.** `Bash(ls *)` coincide con `ls`, y `Bash(git log *)` coincide con `git log`. Esto se mantiene solo cuando el `*` final es el único comodín de la regla: `Bash(* --help *)` coincide con `npm --help x` pero no con `npm --help`.
* **El espacio antes de un `*` final es parte de la regla.** `Bash(ls *)` requiere un espacio después de `ls`, por lo que `lsof` no coincide. `Bash(ls*)` no tiene espacio, por lo que también coincide con `lsof`.

El sufijo `:*` es una forma equivalente de escribir un comodín final, por lo que `Bash(ls:*)` coincide con los mismos comandos que `Bash(ls *)`.

El diálogo de permisos escribe la forma separada por espacios cuando selecciona "Sí, no preguntar de nuevo" para un prefijo de comando. La forma `:*` solo se reconoce al final de un patrón. En un patrón como `Bash(git:* push)`, los dos puntos se tratan como un carácter literal y no coincidirán con comandos git.

<h3 id="tool-name-wildcards">
  Comodines de nombre de herramienta
</h3>

Las reglas de denegación y solicitud también aceptan patrones glob en la posición del nombre de la herramienta. El patrón debe coincidir con el nombre completo de la herramienta: `"*"` coincide con todas las herramientas, y `"mcp__*"` coincide con todas las herramientas MCP en todos los servidores. Una herramienta coincidida por una regla de denegación de nombre simple se elimina del contexto de Claude, igual que un nombre de herramienta simple, incluida la excepción [`EndConversation`](/docs/es/tools-reference#endconversation-tool-behavior): una denegación glob no puede eliminarla mientras permanezca cualquier otra herramienta, y una solicitud glob nunca solicita por ella. Esta configuración deniega todas las herramientas MCP:

```json theme={null}
{
  "permissions": {
    "deny": [
      "mcp__*"
    ]
  }
}
```

Las reglas de permiso aceptan comodines de nombre de herramienta solo después de un prefijo literal `mcp__<server>__`. El segmento del servidor debe estar libre de comodines para que la regla nombre un servidor específico que haya configurado. `mcp__puppeteer__*` coincide con todas las herramientas del servidor `puppeteer`, y `mcp__github__get_*` coincide con sus herramientas `get_`. Un comodín de permiso sin ancla como `"*"`, `"B*"`, o `"mcp__*"` se omite con una advertencia y no aprueba automáticamente nada.

Una regla de denegación o solicitud cuyo nombre de herramienta no coincide con ninguna herramienta conocida produce una advertencia de inicio para detectar errores tipográficos. Los nombres de herramientas que contienen `_` o `*` están exentos de la verificación, y también lo están los nombres de herramientas que Claude Code ha eliminado, como `TaskOutput`.

La etiqueta mostrada para una herramienta en la transcripción y el diálogo de permisos puede diferir de su nombre canónico. Por ejemplo, la herramienta etiquetada como `Stop Task` en la transcripción tiene el nombre canónico `TaskStop`. Las reglas de permisos y los [coincidentes de hooks](/docs/es/hooks) no coinciden con la etiqueta, por lo que una regla escrita como `Stop Task` no coincide. Para reglas de denegación y solicitud, la advertencia de inicio anterior detecta la discrepancia. Use los nombres canónicos enumerados en la [referencia de herramientas](/docs/es/tools-reference).

<h2 id="tool-specific-permission-rules">
  Reglas de permisos específicas de herramientas
</h2>

<h3 id="bash">
  Bash
</h3>

Las reglas de Bash coinciden con el texto completo del comando, siendo `*` un comodín para cualquier texto. [Wildcard patterns](#wildcard-patterns) muestra qué comandos coinciden con cada forma de regla y dónde poner el `*`. El resto de esta sección cubre cómo Claude Code coincide con comandos compuestos y envoltorios, qué no coincide una regla, comandos de solo lectura y redirecciones.

<h4 id="compound-commands">
  Comandos compuestos
</h4>

<Tip>
  Claude Code es consciente de los operadores de shell, por lo que una regla como `Bash(safe-cmd *)` no le dará permiso para ejecutar el comando `safe-cmd && other-cmd`. Los separadores de comandos reconocidos son `&&`, `||`, `;`, `|`, `|&`, `&` y saltos de línea. Una regla debe coincidir con cada subcomando de forma independiente.
</Tip>

Las reglas de negación y solicitud se aplican cuando cualquier subcomando coincide con ellas, incluido un comando anidado dentro de un subshell, una sustitución de comandos o un cuerpo de flujo de control como un bucle `for`. Una regla de solicitud como `Bash(git clean *)` aún le solicita aprobación para `cd /tmp && git clean -f` o `echo "$(git clean -f)"`, incluso en [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode).

Cuando `&&` o `||` no tiene nada después, como en `npm test &&`, Claude Code trata el comando como no analizable y no lo divide en subcomandos para coincidencia de reglas de permiso de solo lectura, por lo que una regla como `Bash(npm *)` no la aprueba.

Cuando aprueba un comando compuesto con "Sí, y no vuelvas a preguntar", Claude Code guarda una regla separada para cada subcomando que requiere aprobación, en lugar de una única regla para la cadena completa. Por ejemplo, aprobar `git status && npm test` guarda una regla para `npm test`, por lo que las invocaciones futuras de `npm test` se reconocen independientemente de lo que preceda a `&&`. Los subcomandos como `cd` en un subdirectorio generan su propia regla Read para esa ruta. Se pueden guardar hasta 5 reglas para un único comando compuesto.

<h4 id="process-wrappers">
  Envoltorios
</h4>

Antes de coincidir con las reglas de Bash, Claude Code elimina un conjunto fijo de envoltorios, por lo que una regla como `Bash(npm test *)` también coincide con `timeout 30 npm test`. Los envoltorios eliminados son `timeout`, `time`, `nice`, `nohup` y `stdbuf`, más los builtins de shell `command` y `builtin`, y `noglob` de zsh. Cada uno ejecuta su argumento como el comando real. Dos formas relacionadas no se eliminan: la forma de consulta `command -v`, que busca un comando en lugar de ejecutarlo, y `nocorrect` de zsh.

Claude Code también elimina una asignación inicial de ciertas variables de entorno conocidas como seguras, por lo que `Bash(npm test *)` coincide con `NODE_ENV=test npm test`. Una regla de permiso no coincidirá más allá de una asignación de cualquier otra variable. Una regla de negación o solicitud coincide más allá de cualquier asignación inicial, por lo que `Bash(rm *)` en negación aún coincide con `FOO=bar rm -rf tmp/`.

`xargs` desnudo también se elimina, por lo que `Bash(grep *)` coincide con `xargs grep pattern`. La eliminación se aplica solo cuando `xargs` no tiene banderas: una invocación como `xargs -n1 grep pattern` se coincide como un comando `xargs`, por lo que las reglas escritas para el comando interno no la cubren.

Esta lista de envoltorios está integrada y no es configurable. Los ejecutores de entorno de desarrollo como `direnv exec`, `devbox run`, `mise exec`, `npx` y `docker exec` no están en la lista. Debido a que estas herramientas ejecutan sus argumentos como un comando, una regla como `Bash(devbox run *)` coincide con lo que viene después de `run`, incluido `devbox run rm -rf .`. Para aprobar el trabajo dentro de un ejecutor de entorno, escriba una regla específica que incluya tanto el ejecutor como el comando interno, como `Bash(devbox run npm test)`. Agregue una regla por cada comando interno que desee permitir.

Los envoltorios Exec como `watch`, `setsid`, `ionice` y `flock` no pueden ser aprobados automáticamente por una regla de prefijo como `Bash(watch *)`, por lo que en modo Manual siempre solicitan aprobación. Lo mismo se aplica a `find` con `-exec` o `-delete`: una regla `Bash(find *)` no cubre estas formas. Para aprobar una invocación específica, escriba una regla de coincidencia exacta para la cadena de comando completa.

<h4 id="bash-rule-limits">
  Qué no coincide una regla de Bash
</h4>

Una regla de Bash coincide con el texto del comando que Claude escribe, después de que Claude Code divide [comandos compuestos](#compound-commands) y elimina [envoltorios](#process-wrappers). No coincide con el mismo programa invocado de una forma diferente, por lo que una regla de negación o solicitud cubre la invocación que Claude generalmente produce y no es un límite de seguridad alrededor del programa. Estas reglas en `deny` o `ask` detienen la primera forma y no las otras:

| Regla              | Detiene                    | No detiene                                                                                            |
| :----------------- | :------------------------- | :---------------------------------------------------------------------------------------------------- |
| `Bash(curl *)`     | `curl https://example.com` | `/usr/bin/curl https://example.com`, `sh -c 'curl https://example.com'`                               |
| `Bash(rm *)`       | `rm -rf build/`            | `/bin/rm -rf build/`, `bash -c 'rm -rf build/'`                                                       |
| `Bash(git push *)` | `git push origin main`     | `git -C . push origin main`, `git -c push.default=current push origin main`, `git 'push' origin main` |

Sus otras reglas y el modo de permiso deciden los comandos en la última columna.

Para la aplicación de sistema de archivos y red que no depende del texto del comando, use [sandboxing](/docs/es/sandboxing). Para inspeccionar el texto completo del comando con su propia lógica antes de que se ejecute, use un hook [PreToolUse](#extend-permissions-with-hooks).

<h4 id="read-only-commands">
  Comandos de solo lectura
</h4>

Claude Code reconoce un conjunto integrado de comandos de Bash como de solo lectura y los ejecuta sin una solicitud de permiso en todos los modos, excepto para una ruta que [`permissions.blockReadsOutsideWorkingDirectories`](/docs/es/settings-reference#permissions-blockreadsoutsideworkingdirectories) cierra. El conjunto incluye `ls`, `cat`, `echo`, `pwd`, `head`, `tail`, `grep`, `find`, `wc`, `which`, `diff`, `stat`, `du`, `cd` y formas de solo lectura de `git`. El conjunto no es configurable; para requerir una solicitud para uno de estos comandos, agregue una regla `ask` o `deny` para él. En modo automático, estos comandos también pueden esperar la revisión del clasificador; vea [cómo el clasificador evalúa acciones](/docs/es/permission-modes#how-the-classifier-evaluates-actions).

Una redirección como `ls > out.txt` agrega una verificación en el destino. Vea [Redirections](#redirections).

Los patrones glob sin comillas se permiten para comandos cuya bandera es de solo lectura, por lo que `ls *.ts` y `wc -l src/*.py` se ejecutan sin una solicitud.

En modo Manual, los comandos de este conjunto aún solicitan aprobación en estos casos:

* **Globs sin comillas para comandos con banderas capaces de escritura**: comandos con banderas capaces de escritura o ejecución, como `find`, `sort`, `sed` y `git`, solicitan aprobación cuando hay un glob sin comillas presente, porque el glob podría expandirse a una bandera como `-delete`.
* **`docker` apuntado a otro daemon**: formas de solo lectura de `docker` solicitan aprobación cuando el comando lleva una bandera que selecciona un daemon diferente, como `-H`, `--context` o `--url` y `--connection` de Podman.
* **`file` con banderas de apertura de ruta**: `file` solicita aprobación cuando pasa `-m`/`--magic-file` o `-f`/`--files-from`, porque esas banderas hacen que `file` abra las rutas nombradas en el valor de la bandera.
* **Rutas de red en Windows**: un comando cuyos argumentos incluyen una ruta de red (UNC), como `\\server\share\file`, solicita aprobación porque acceder a una ruta de red puede enviar sus credenciales de Windows al host que nombra. La misma verificación se aplica a comandos de [herramienta PowerShell](/docs/es/tools-reference#powershell-tool).
* **Comandos que el análisis no puede analizar**: cuando Claude Code no puede analizar completamente un comando, solicita aprobación en lugar de tratar el comando como de solo lectura. Los comandos más largos que 10,000 caracteres siempre solicitan aprobación porque exceden lo que el análisis analiza.

Un `cd` en una ruta dentro de su directorio de trabajo o un [directorio adicional](#working-directories) también es de solo lectura, y un comando compuesto como `cd packages/api && ls` se ejecuta sin una solicitud cuando cada parte se califica por sí sola. Estas combinaciones solicitan aprobación incluso cuando cada parte es de solo lectura:

* **`cd` con `git`**: solicita aprobación cuando el `cd` cambia a un directorio diferente, ya que ejecutar `git` en un nuevo directorio puede ejecutar los hooks de ese directorio. Un `cd` cuyo destino se resuelve al directorio de trabajo actual es una no-operación y no desencadena la solicitud.
* **`cd` con una redirección**: solicita aprobación cuando Claude Code no puede determinar contra qué directorio se resuelve el destino de redirección después de que se ejecuta el `cd`. Un comando cuyo único destino de redirección es `/dev/null`, como `cd app; grep -r pattern . 2>/dev/null`, no solicita aprobación, porque `/dev/null` no depende del directorio de trabajo.

<Warning>
  Los patrones de permisos de Bash que intentan restringir argumentos de comandos son frágiles. Por ejemplo, `Bash(curl http://github.com/ *)` intenta restringir curl a URLs de GitHub, pero no coincidirá con variaciones como:

  * Opciones antes de URL: `curl -X GET http://github.com/...`
  * Protocolo diferente: `curl https://github.com/...`
  * Redirecciones: `curl -L http://short.example.com/xyz`, que redirige a GitHub
  * Variables: `URL=http://github.com && curl $URL`

  Para un filtrado de URL más confiable, considere:

  * **Restringir herramientas de red de Bash**: use reglas de negación para detener `curl`, `wget` y comandos similares, luego use la herramienta WebFetch con permiso `WebFetch(domain:github.com)` para dominios permitidos. Una regla de negación no coincide con el mismo programa por ruta o dentro de `sh -c`, por lo que emparéjela con la [lista de permitidos de red del sandbox](/docs/es/sandboxing#network-isolation) cuando la restricción debe mantenerse; vea [qué no coincide una regla de Bash](#bash-rule-limits)
  * **Usar hooks PreToolUse**: implemente un hook que valide URLs en comandos de Bash y bloquee dominios no permitidos
  * **Agregar orientación CLAUDE.md**: describa sus patrones de curl permitidos en `CLAUDE.md`. Esto forma lo que Claude intenta pero no aplica un límite, por lo que emparéjelo con una de las opciones anteriores

  Tenga en cuenta que usar WebFetch solo no previene el acceso a la red. Si se permite Bash, Claude aún puede usar `curl`, `wget` u otras herramientas para alcanzar cualquier URL.
</Warning>

<h4 id="redirections">
  Redirecciones
</h4>

Cuando un comando redirige salida o entrada, Claude Code verifica el destino de redirección contra sus reglas de archivo como si Claude escribiera o leyera ese archivo directamente:

* **Redirecciones de salida**: para `> file`, `>> file` o `2> file`, la verificación cubre sus reglas de permiso `Edit` de permitir y negar, [rutas protegidas](/docs/es/permission-modes#protected-paths) y los [directorios de trabajo](#working-directories). Una regla como `Bash(git commit *)` permite el comando, no el destino. Un destino que comienza con `~` o contiene un carácter glob necesita su aprobación.
* **Redirecciones de entrada**: para `< file`, la verificación cubre sus reglas de permiso `Read` de permitir y negar y los directorios de trabajo. Un destino fuera de los directorios de trabajo necesita su aprobación a menos que una regla de permiso lo cubra. Un destino que contiene un patrón glob, o una ruta relativa que sigue a un `cd` en el mismo comando, necesita su aprobación incluso cuando una regla de permiso lo cubre. Claude Code verifica destinos de entrada en v2.1.257 y posterior.

Los destinos sin archivo detrás no se verifican: `/dev/null`, formas de descriptor de archivo como `2>&1` y `<&3`, y here-docs y here-strings.

Claude Code también verifica los archivos que escribe un comando `tee`, incluido en un pipeline como `make | tee build.log`. La verificación cubre sus reglas de permiso `Edit` de permitir y negar, [rutas protegidas](/docs/es/permission-modes#protected-paths) y los [directorios de trabajo](#working-directories). Una regla de permiso como `Bash(tee *)` no cubre un destino fuera de los directorios de trabajo. Claude Code verifica destinos de `tee` en v2.1.269 y posterior.

<h3 id="powershell">
  PowerShell
</h3>

Las reglas de permisos de PowerShell usan la misma forma que las reglas de Bash. Los comodines con `*` coinciden en cualquier posición, el sufijo `:*` es equivalente a un ` *` final, y un `PowerShell` desnudo o `PowerShell(*)` coincide con cada comando. Esta configuración permite comandos `Get-ChildItem` y `git commit` mientras bloquea `Remove-Item`:

```json theme={null}
{
  "permissions": {
    "allow": [
      "PowerShell(Get-ChildItem *)",
      "PowerShell(git commit *)"
    ],
    "deny": [
      "PowerShell(Remove-Item *)"
    ]
  }
}
```

Los alias comunes se canonicalizan antes de coincidir. Una regla escrita para el nombre del cmdlet también coincide con sus alias, por lo que `PowerShell(Get-ChildItem *)` coincide con `gci`, `ls` y `dir` también. La coincidencia no distingue mayúsculas de minúsculas.

Claude Code analiza el AST de PowerShell y verifica cada comando en un comando compuesto de forma independiente. Los operadores de pipeline `|`, separadores de declaración `;` y en PowerShell 7+ los operadores de cadena `&&` y `||` dividen un comando compuesto en subcomandos. Una regla debe coincidir con cada subcomando para que se permita el comando compuesto.

<h3 id="read-and-edit">
  Read y Edit
</h3>

Para bloquear las herramientas de archivo de Claude de leer un archivo o directorio, agregue una regla de negación `Read` para su ruta, como `Read(./.env)` o `Read(./secrets/**)`; [Exclude sensitive files](/docs/es/settings-reference#exclude-sensitive-files) tiene un ejemplo listo para pegar.

Las reglas `Edit` se aplican a todas las herramientas integradas que editan archivos. Claude hace un esfuerzo de mejor intento para aplicar reglas `Read` a todas las herramientas integradas que leen archivos como Grep y Glob, a menciones `@file` en sus solicitudes y a la selección y contexto de archivo abierto que un [IDE](/docs/es/vs-code#the-built-in-ide-mcp-server) conectado comparte con Claude.

Una regla de negación `Read` también bloquea las herramientas [Edit y Write](/docs/es/errors#file-is-covered-by-a-read-deny-rule) en la misma ruta, incluida la creación de un nuevo archivo allí. NotebookEdit no está cubierto, por lo que agregue una regla de negación `Edit` para rutas que ninguna herramienta pueda cambiar. La verificación requiere Claude Code v2.1.208 o posterior en ediciones, y v2.1.228 o posterior en escrituras.

Claude Code verifica permisos de archivo solo contra reglas `Edit(path)` y `Read(path)`. Si escribe una regla de ruta para `Write`, `NotebookEdit`, `Glob` o la herramienta `MultiEdit` heredada en su lugar, Claude Code acepta la regla pero nunca la consulta, y [advierte al inicio](/docs/es/errors#is-not-matched-by-file-permission-checks), excepto para una regla `Glob` pasada en `--allowedTools`. Use `Edit(docs/**)` en lugar de `Write(docs/**)`, `NotebookEdit(docs/**)` o `MultiEdit(docs/**)`, y `Read(docs/**)` en lugar de `Glob(docs/**)`. Claude Code no advierte sobre una regla de nombre de herramienta sin ruta, como una regla de negación para `Write`; coincide esa regla a nivel de herramienta en todas partes. Requiere Claude Code v2.1.210 o posterior.

<Warning>
  Las reglas de negación Read y Edit se aplican a las herramientas de archivo integradas de Claude, a comandos de archivo que Claude Code reconoce en Bash, como `cat`, `head`, `tail`, `sed` y `tee`, y a los destinos de [redirecciones](#redirections) de Bash como `> file` y `< file`. No se aplican a un comando que lee archivos sin nombrarlos, como `grep -r pattern .` ejecutado desde el directorio que contiene el archivo, o a subprocesos arbitrarios que leen o escriben archivos indirectamente, como un script de Python o Node que abre archivos por sí mismo. Para la aplicación a nivel del SO que bloquea todos los procesos de acceder a una ruta, [habilite el sandbox](/docs/es/sandboxing).
</Warning>

Las reglas Read y Edit usan ambas sintaxis de patrón [gitignore](https://git-scm.com/docs/gitignore) con cuatro tipos de patrón distintos; para patrones de directorio de un solo segmento, la profundidad de coincidencia también depende del tipo de regla, descrito más adelante en esta sección:

| Patrón            | Significado                                         | Ejemplo                          | Coincide                                                               |
| ----------------- | --------------------------------------------------- | -------------------------------- | ---------------------------------------------------------------------- |
| `//path`          | Ruta absoluta desde la raíz del sistema de archivos | `Read(//Users/alice/secrets/**)` | `/Users/alice/secrets/**`                                              |
| `~/path`          | Ruta desde el directorio de inicio                  | `Read(~/Documents/*.pdf)`        | `/Users/alice/Documents/*.pdf`                                         |
| `/path`           | Ruta relativa a la fuente de configuración          | `Edit(/src/**/*.ts)`             | `<primary working directory>/src/**/*.ts` en configuración de proyecto |
| `path` o `./path` | Ruta relativa al directorio actual                  | `Read(*.env)`                    | `<cwd>/*.env`                                                          |

<Warning>
  Un patrón como `/Users/alice/file` no es una ruta absoluta. La barra inclinada inicial única se ancla en la fuente de configuración, no en la raíz del sistema de archivos. Use `//Users/alice/file` para rutas absolutas.
</Warning>

Un patrón `/path` se ancla en un directorio asociado con la fuente de configuración que lo define, por lo que la misma regla coincide con ubicaciones diferentes dependiendo de dónde la coloque:

| Regla definida en                                     | `/path` se resuelve a              |
| :---------------------------------------------------- | :--------------------------------- |
| Configuración de proyecto en `.claude/settings.json`  | `<primary working directory>/path` |
| Configuración local en `.claude/settings.local.json`  | `<primary working directory>/path` |
| Configuración de usuario en `~/.claude/settings.json` | `~/.claude/path`                   |
| Un archivo pasado con `--settings <file>`             | `<directory of file>/path`         |
| Banderas CLI o reglas de sesión                       | `<primary working directory>/path` |

Una regla que agrega a través de `/permissions` sigue la fila para el archivo de configuración en el que la guarda.

Las reglas de configuración local se anclan en el [directorio de trabajo principal](#working-directories) de la sesión, no en la raíz del repositorio donde Claude Code [almacena el archivo](#permission-system) en v2.1.211 y posterior. En una sesión iniciada en la raíz del repositorio, los dos directorios son iguales; en una sesión de [worktree](/docs/es/worktrees), una regla compartida como `Edit(/src/**)` coincide con el directorio `src/` propio de ese worktree.

Una regla de negación como `Read(/secrets/**)` en configuración de usuario bloquea `~/.claude/secrets/**`, no un directorio `secrets` en su proyecto. Para escribir una regla en configuración de usuario que se aplique dentro de cada proyecto, use una ruta `//` absoluta o una ruta `~/` relativa a inicio en su lugar.

En Windows, las rutas se normalizan a forma POSIX antes de coincidir. `C:\Users\alice` se convierte en `/c/Users/alice`, por lo que use `//c/**/.env` para coincidir con archivos `.env` en cualquier lugar de esa unidad. Para coincidir en todas las unidades, use `//**/.env`.

Ejemplos:

* `Edit(/docs/**)`: ediciones en `<primary working directory>/docs/`, no `/docs/` o `<primary working directory>/.claude/docs/`
* `Read(~/.zshrc)`: lee el `.zshrc` de su directorio de inicio
* `Edit(//tmp/scratch.txt)`: edita la ruta absoluta `/tmp/scratch.txt`
* `Read(src/**)`: como regla de permiso, lee desde `<current-directory>/src/` solo; como regla de negación o solicitud, coincide con un directorio `src` a cualquier profundidad bajo el directorio actual

Una regla solo coincide con archivos bajo su ancla; dentro de ese límite, la profundidad de coincidencia depende de la forma del patrón y, para patrones de directorio de un solo segmento, del tipo de regla, descrito a continuación. Los nombres de archivo desnudos siguen la semántica de gitignore y coinciden a cualquier profundidad, por lo que `Read(.env)` y `Read(**/.env)` son equivalentes:

| Regla de negación              | Bloquea                                                     | No bloquea                                                 |
| ------------------------------ | ----------------------------------------------------------- | ---------------------------------------------------------- |
| `Read(.env)` o `Read(**/.env)` | cualquier `.env` en o bajo el directorio actual             | `.env` en un directorio padre u otro proyecto              |
| `Read(//**/.env)`              | cualquier `.env` en cualquier lugar del sistema de archivos | nada; la regla se ancla en la raíz del sistema de archivos |

Un patrón relativo con un segmento de directorio único, como `src/**`, coincide a diferentes profundidades dependiendo del tipo de regla:

* **Reglas de permiso**: `Edit(src/**)` coincide solo con `<cwd>/src` y los archivos bajo él. Para permitir un nombre de directorio a cualquier profundidad, escriba `Edit(**/src/**)`.
* **Reglas de negación y solicitud**: `Read(secrets/**)` coincide con un directorio llamado `secrets` a cualquier profundidad bajo el directorio actual, por lo que la regla también se aplica a copias anidadas.

Cada otra forma de patrón coincide a la misma profundidad en cada tipo de regla: `Edit(/src/**)` y `Edit(src/components/**)` coinciden solo en su ubicación anclada, mientras que `Edit(**/src/**)` coincide a cualquier profundidad.

El siguiente ejemplo muestra cada forma de patrón contra un proyecto con un directorio `src/` de nivel superior y una copia anidada bajo `vendor/`:

```text theme={null}
<current-directory>/
├── src/
│   └── app.ts
└── vendor/
    └── pkg/
        └── src/
            └── lib.js
```

| Regla                                             | Coincide `src/app.ts` | Coincide `vendor/pkg/src/lib.js` |
| :------------------------------------------------ | :-------------------- | :------------------------------- |
| `Edit(src/**)` como regla de permiso              | Sí                    | No                               |
| `Edit(src/**)` como regla de negación o solicitud | Sí                    | Sí                               |
| `Edit(/src/**)` en cualquier tipo de regla        | Sí                    | No                               |
| `Edit(**/src/**)` en cualquier tipo de regla      | Sí                    | Sí                               |

<Note>
  En patrones de gitignore, `*` coincide dentro de un segmento de ruta único y puede aparecer en cualquier posición en el patrón, mientras que `**` coincide en directorios.
</Note>

Cuando aprueba una ruta de archivo con "Sí, y no vuelvas a preguntar", Claude Code escapa caracteres de patrón de gitignore en esa ruta, como `[`, `]` y `*`, por lo que la regla generada coincide solo con la ruta literal que aprobó. Las reglas que escribe usted mismo no se escapan. Antes de v2.1.202, Claude Code guardaba la ruta sin escapar, por lo que una regla generada para un directorio llamado `[2024-06] Reports` podría no coincidir con su propia ruta o coincidir con directorios hermanos no deseados.

No necesita escapar paréntesis en una ruta, por lo que `Edit(./Finance (2024)/**)` coincide con la carpeta `Finance (2024)` tal como está escrita.

Una regla de negación o solicitud cuya ruta no es utilizable como patrón de gitignore aún protege esa ruta exacta. Una regla de permiso con un patrón no utilizable no aprueba nada.

Un patrón de negación o solicitud que comienza con `!` es una negación de gitignore. Extrae las rutas que coincide de las reglas `path` o `./path` enumeradas antes. En la lista `deny` de un archivo de configuración, `Read(*.env)` seguido de `Read(!sample.env)` bloquea cada archivo cuyo nombre termina en `.env` a cualquier profundidad, excepto archivos nombrados `sample.env`. Una regla `!` enumerada primero no extrae nada.

La extracción solo alcanza reglas de la misma fuente. Un `Read(!.env)` en configuración de proyecto o en `--disallowedTools` no cancela un `Read(./.env)` de negación de configuración administrada o cualquier otro archivo de configuración.

Dos límites estrechan lo que un patrón `!` puede extraer:

* Claude Code lee un patrón `!` relativo al directorio actual incluso cuando `/`, `~/` o `//` sigue a `!`, por lo que el patrón no puede alcanzar una regla anclada con uno de esos prefijos. `Read(!~/notes/public/**)` no extrae nada de `Read(~/notes/**)`.
* Una extracción no puede reabrir un archivo dentro de un directorio que una regla bloquea como un todo. Con `Read(secrets/**)` y `Read(!secrets/public/**)`, Claude Code aún bloquea `secrets/public` junto con el resto de `secrets`.

Cuando Claude accede a un symlink, las reglas de permiso verifican dos rutas: el symlink en sí y el archivo al que se resuelve. Las reglas de permiso y negación tratan ese par de manera diferente: las reglas de permiso vuelven a solicitar aprobación, mientras que las reglas de negación bloquean directamente.

* **Reglas de permiso**: se aplican solo cuando tanto la ruta del symlink como su destino coinciden. Un symlink dentro de un directorio permitido que apunta fuera de él aún le solicita aprobación.
* **Reglas de negación**: se aplican cuando la ruta del symlink o su destino coincide. Un symlink que apunta a un archivo negado está negado en sí mismo. Por ejemplo, con `Read(./project/**)` permitido y `Read(~/.ssh/**)` negado, un symlink en `./project/key` que apunta a `~/.ssh/id_rsa` está bloqueado: el destino falla la regla de permiso y coincide con la regla de negación.

En macOS y Linux, una regla de negación o solicitud escrita a través de un directorio con symlink con un patrón `//`, `~/` o `/` también se aplica en la ubicación real del directorio. Por ejemplo, en macOS, donde `/etc` se resuelve a `/private/etc`, `Read(//etc/**)` también bloquea `/private/etc/hosts`. Antes de v2.1.268, una regla de negación o solicitud escrita a través de un directorio con symlink no se aplicaba a una ruta dada por su ubicación real.

Cuando una herramienta abre un archivo aprobado, Claude Code [confirma que la ruta aún se resuelve a la ubicación que la verificación de permiso aprobó](/docs/es/errors#refusing-after-a-symlink-changed).

Grep y Glob buscan el directorio al que se resuelve el argumento `path`. Claude Code aplica reglas de negación `Read` a ese directorio.

<h3 id="webfetch">
  WebFetch
</h3>

Las reglas de WebFetch usan un prefijo `domain:` y coinciden contra el nombre de host de la URL solicitada. La coincidencia no distingue mayúsculas de minúsculas, admite comodines `*` y elimina un `.` final de la regla y el nombre de host para que `example.com.` y `example.com` se traten igual.

* `WebFetch(domain:example.com)` coincide con solicitudes a `example.com`
* `WebFetch(domain:*.example.com)` coincide con cualquier subdominio a cualquier profundidad, como `api.example.com` o `a.b.example.com`, pero no `example.com` en sí
* `WebFetch(domain:*)` coincide con cada dominio. No es lo mismo que una regla `WebFetch` desnuda; vea [Allow or deny every fetch](#allow-or-deny-every-fetch)

En cualquier posición que no sea un `*.` inicial o un `*` desnudo, el comodín coincide solo con el texto entre dos puntos. `WebFetch(domain:example.*)` coincide con `example.org`, donde `*` se convierte en `org`, pero no `example.evil.com`, donde `*` tendría que convertirse en `evil.com` y cruzar un punto. Esto evita que un comodín final coincida con dominios que un atacante podría registrar.

Los comodines en reglas `WebFetch` requieren Claude Code v2.1.172 o posterior para coincidir con búsquedas.

<h4 id="allow-or-deny-every-fetch">
  Permitir o negar cada búsqueda
</h4>

Una regla `WebFetch` desnuda es el nombre de la herramienta sin una parte `domain:`, como `"deny": ["WebFetch"]`. Tanto ella como `WebFetch(domain:*)` cubren cada URL, pero Claude Code las aplica de manera diferente, y solo la forma `domain:` también agrega su dominio a la [lista de dominios permitidos o negados](/docs/es/sandboxing#network-isolation) del sandbox. Esa sección enumera las formas de comodín que el sandbox honra y la versión que agregó `*` desnudo.

Cada fila muestra qué hace una regla en la lista `allow` y en la lista `deny`:

| Regla                | En `allow`                                                                                            | En `deny`                                                                                                                                                  |
| :------------------- | :---------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `WebFetch`           | Claude busca sin solicitarle aprobación. No cambia qué hosts pueden alcanzar los comandos en sandbox. | Claude Code elimina la herramienta `WebFetch`, por lo que Claude no puede buscar en absoluto. No cambia qué hosts pueden alcanzar los comandos en sandbox. |
| `WebFetch(domain:*)` | Claude busca sin solicitarle aprobación, y los comandos en sandbox pueden alcanzar cualquier host.    | Claude Code mantiene la herramienta y rechaza cada búsqueda, y los comandos en sandbox no pueden alcanzar ningún host.                                     |

Las dos formas también difieren en lecturas de [artefactos](/docs/es/artifacts), las páginas que la herramienta Artifact publica en claude.ai. Una regla de negación o solicitud `WebFetch` desnuda no se aplica a esas lecturas. Una regla `domain:` que cubre `claude.ai` o el host de contenido `*.claudeusercontent.com`, como `WebFetch(domain:claude.ai)` o `WebFetch(domain:*)`, niega cada lectura o solicita aprobación antes. Una regla [`Artifact`](/docs/es/artifacts#disable-artifacts) hace lo mismo.

Cuando una regla bloquea una lectura, la negación nombra la regla. Antes de v2.1.268, una regla de negación `WebFetch` desnuda bloqueaba cada lectura de artefacto, y una regla de solicitud desnuda solicitaba aprobación antes de cada una.

Para permitir que Claude busque libremente mientras mantiene la lista de permitidos del sandbox como está, use la forma desnuda. Este `settings.json` hace eso:

```json theme={null}
{
  "permissions": {
    "allow": ["WebFetch"]
  }
}
```

Cuando le pide a Claude que busque una página, busca sin una solicitud. Cuando le pide que ejecute un `curl` [en sandbox](/docs/es/sandboxing) contra un host fuera de la lista de permitidos del sandbox, Claude Code aún le solicita aprobación para ese host, porque la regla desnuda no agregó el host a la lista de permitidos.

En [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode), Claude en su lugar nombra el host en los [dominios permitidos por comando](/docs/es/sandboxing#per-command-allowed-domains-in-auto-mode) del comando para que el clasificador revise.

<h3 id="mcp">
  MCP
</h3>

Las reglas de MCP usan el nombre del servidor tal como está configurado en Claude Code, opcionalmente seguido del nombre de una herramienta de ese servidor.

* `mcp__puppeteer` coincide con cualquier herramienta proporcionada por el servidor `puppeteer`
* `mcp__puppeteer__*` usa sintaxis de comodín y también coincide con todas las herramientas del servidor `puppeteer`
* `mcp__puppeteer__puppeteer_navigate` coincide con la herramienta `puppeteer_navigate` proporcionada por el servidor `puppeteer`

Si su organización ha establecido una herramienta de [conector claude.ai](/docs/es/mcp#organization-controls-on-connector-tools) en `ask` y esa configuración llega a Claude Code en su sesión, las reglas de permiso para esa herramienta no tienen efecto: Claude Code solicita aprobación en cada llamada, incluso en modos `auto` y `bypassPermissions`. En modo `dontAsk`, que nunca solicita aprobación, Claude Code niega la llamada en su lugar. Las herramientas de conectores que Claude Code obtiene por sí mismo aparecen como `mcp__claude_ai_<server>__<tool>`.

En una sesión de [Cowork](https://claude.com/docs/cowork/overview) en la aplicación Claude Desktop, Claude ejecuta comandos de shell a través de la herramienta `mcp__workspace__bash` de Cowork en lugar de la herramienta `Bash` integrada, y Cowork igualmente proporciona `mcp__workspace__web_fetch` para búsquedas web. Claude Code también aplica reglas de negación que nombran la herramienta `Bash` o `WebFetch` completa a estas herramientas de Cowork, por lo que una regla de negación `Bash` administrada detiene a Claude de ejecutar comandos de shell en Cowork. Cuando Claude Code bloquea tal llamada, el mensaje nombra la herramienta de Cowork: `Permission to use mcp__workspace__bash has been denied.` Las reglas de permiso no se transfieren: Claude Code nunca aplica una regla de permiso `Bash` a `mcp__workspace__bash`.

<h3 id="agent-subagents">
  Agent (subagentes)
</h3>

Use reglas `Agent(AgentName)` para controlar qué [subagentes](/docs/es/sub-agents) puede usar Claude:

* `Agent(Explore)` coincide con el subagente Explore
* `Agent(Plan)` coincide con el subagente Plan
* `Agent(my-custom-agent)` coincide con un subagente personalizado llamado `my-custom-agent`

Agregue estas reglas a la matriz `deny` en su configuración o use la bandera CLI `--disallowedTools` para deshabilitar agentes específicos. Para deshabilitar el agente Explore:

```json theme={null}
{
  "permissions": {
    "deny": ["Agent(Explore)"]
  }
}
```

<h3 id="cd">
  Cd
</h3>

Las reglas `Cd` controlan a qué directorios el comando [`/cd`](/docs/es/commands) puede mover la sesión. `Cd` no es una herramienta invocable por modelo: Claude no puede llamarla, y las reglas se aplican solo cuando ejecuta `/cd` usted mismo.

Una regla de negación `Cd` desnuda deshabilita `/cd` completamente. Una regla de negación `Cd(<path-pattern>)` bloquea destinos coincidentes. Las reglas de negación verifican cada ortografía del destino, incluido cada salto de symlink que se resuelve, por lo que una regla escrita para una ruta también bloquea destinos que se resuelven a ella.

Agregar cualquier regla de permiso `Cd` cambia `/cd` a modo de lista de permitidos: el directorio de destino resuelto debe coincidir con una de sus reglas de permiso, o `/cd` rechaza. Sin reglas `Cd` configuradas, `/cd` mantiene su comportamiento predeterminado y le solicita que confíe en un directorio desconocido.

Los patrones de ruta comparten los anclajes `//`, `~/` y `/` de [reglas Read y Edit](#read-and-edit), pero la coincidencia se ancla a la ruta de directorio completa en lugar de estilo gitignore. `*` coincide exactamente con un segmento de ruta y `**` coincide en segmentos. Un `/**` final también coincide con su raíz nombrada.

| Regla                 | Coincide                                                                              | No coincide                   |
| --------------------- | ------------------------------------------------------------------------------------- | ----------------------------- |
| `Cd(~/code/*)`        | `~/code/app`                                                                          | `~/code/app/src`, `~/code`    |
| `Cd(~/code/**)`       | `~/code` y cualquier directorio bajo él                                               | directorios fuera de `~/code` |
| `Cd(**/node_modules)` | cualquier directorio `node_modules` a cualquier profundidad bajo el directorio actual | `node_modules/pkg`            |

<h2 id="extend-permissions-with-hooks">
  Extender permisos con hooks
</h2>

Los [hooks de Claude Code](/docs/es/hooks-guide) le permiten registrar comandos de shell personalizados que evalúan permisos en tiempo de ejecución. Cuando Claude Code realiza una llamada de herramienta, los hooks PreToolUse se ejecutan antes del aviso de permisos, para cada herramienta excepto [`EndConversation`](/docs/es/tools-reference#endconversation-tool-behavior). La salida del hook puede denegar la llamada de herramienta, forzar un aviso u omitir el aviso para permitir que la llamada continúe.

Las decisiones del hook no omiten las reglas de permisos. Claude Code evalúa las reglas de negación y solicitud independientemente de lo que devuelva un hook PreToolUse: una regla de negación coincidente bloquea la llamada, y una regla de solicitud coincidente aún solicita incluso cuando el hook devolvió `"allow"` u `"ask"`. Esto preserva la precedencia de negación primero descrita en [Administrar permisos](#manage-permissions), incluyendo reglas de negación establecidas en configuración administrada.

Las herramientas MCP marcadas como [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool) también aún solicitan cuando un hook devuelve `"allow"`, al igual que las herramientas de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools) en sesiones donde esa configuración llega a Claude Code.

Un hook de bloqueo también tiene precedencia sobre las reglas de permiso. Un hook que sale con código 2 detiene la llamada de herramienta antes de que se evalúen las reglas de permisos, por lo que el bloqueo se aplica incluso cuando una regla de permiso permitiría que la llamada continúe. Para ejecutar todos los comandos Bash sin avisos excepto algunos que desea bloquear, agregue `"Bash"` a su lista de permiso y registre un hook PreToolUse que rechace esos comandos específicos. Consulte [Bloquear ediciones a archivos protegidos](/docs/es/hooks-guide#block-edits-to-protected-files) para un script de hook que puede adaptar.

<h2 id="working-directories">
  Directorios de trabajo
</h2>

Por defecto, Claude tiene acceso a archivos en el directorio donde fue lanzado. Ese directorio es el directorio de trabajo principal de la sesión hasta que [mueva la sesión con `/cd`](#move-the-session-to-another-directory). Puede extender este acceso:

* **Durante el inicio**: use el argumento CLI `--add-dir <path>`
* **Durante la sesión**: use el comando `/add-dir`
* **Configuración persistente**: agregue a `additionalDirectories` en [archivos de configuración](/docs/es/settings#where-settings-live)

Los archivos en directorios adicionales siguen las mismas reglas de permisos que el directorio de trabajo original: se vuelven legibles sin avisos, y los permisos de edición de archivos siguen el modo de permisos actual.

No puede agregar la mayoría de [rutas de red](/docs/es/errors#working-directory-is-a-network-path), como el recurso compartido UNC `\\server\share`, como directorios de trabajo, porque buscar uno puede contactar al host que nombra. En Windows, asigne el recurso compartido a una letra de unidad en su lugar y pase la unidad con `--add-dir` al iniciar.

Establezca [`permissions.blockReadsOutsideWorkingDirectories`](/docs/es/settings-reference#permissions-blockreadsoutsideworkingdirectories) para hacer que las herramientas de archivo rechacen las rutas que delimita en cada modo de permisos. En modo automático, Claude Code ofrece activarlo la primera vez que Claude [lee fuera de los directorios de trabajo](/docs/es/permission-modes#first-read-outside-the-working-directories).

En sesiones en segundo plano en macOS, el host de la sesión solicita acceso a carpetas protegidas como `~/Desktop`, `~/Documents` y `~/Downloads` por separado de su terminal cuando Claude necesita leer o escribir archivos allí; si las lecturas allí fallan con `Operation not permitted`, consulte [cómo otorgar acceso a carpetas a sesiones en segundo plano](/docs/es/agent-view#background-sessions-can%E2%80%99t-read-desktop-documents-or-downloads-on-macos).

<h3 id="move-the-session-to-another-directory">
  Mover la sesión a otro directorio
</h3>

Para mover la sesión a un directorio de trabajo principal diferente, en lugar de [agregar un directorio](#working-directories) junto al actual, ejecute `/cd <path>`. Claude Code mantiene la conversación, carga el `CLAUDE.md` del nuevo directorio, y le solicita que [confíe en el espacio de trabajo](#project-allow-rules-and-workspace-trust) si no ha trabajado en él antes. Después, Claude Code [encuentra la sesión movida](/docs/es/sessions#resume-a-session) cuando ejecuta `--resume` desde el nuevo directorio.

Tan pronto como se mueva, Claude Code aplica la configuración del proyecto del nuevo directorio:

* Su configuración de proyecto, incluidas sus reglas de permisos y [hooks](/docs/es/hooks)
* Sus servidores [`.mcp.json`](/docs/es/mcp#project-scope), sujetos a la misma [aprobación de servidor](/docs/es/mcp#project-server-approvals-and-workspace-trust) que al iniciar, y los servidores MCP de [alcance local](/docs/es/mcp#local-scope) que registró en él
* Los [plugins](/docs/es/plugins/overview) que su configuración habilita, sus [skills](/docs/es/skills#discovery-from-parent-and-nested-directories), y sus [subagentes](/docs/es/sub-agents)
* Sus valores [`env`](/docs/es/settings-reference#env), aplicados sobre las variables de entorno de la configuración del directorio anterior, que permanecen en vigor

Claude Code también desconecta los servidores MCP del proyecto del directorio anterior y de [alcance local](/docs/es/mcp#local-scope), y los servidores de [plugins](/docs/es/mcp#plugin-provided-mcp-servers) que ya no están habilitados después del movimiento. Toma [directorios adicionales](#working-directories) de la configuración del nuevo directorio en lugar del anterior, y mantiene los directorios que agregó con `--add-dir` o `/add-dir`. Los hooks que el movimiento activa aún reciben [`${CLAUDE_PROJECT_DIR}`](/docs/es/hooks#reference-scripts-by-path) establecido en la raíz del proyecto donde comenzó la sesión.

Cuando el nuevo directorio aún no es de confianza, Claude Code enumera en el aviso de confianza las reglas de permiso, directorios adicionales, hooks y comandos auxiliares que la configuración del directorio activaría, para que pueda revisarlos antes de aceptar. Si rechaza, la sesión permanece donde está. Antes de v2.1.246, `/cd` no aplicaba la configuración, hooks, servidores MCP o skills del nuevo directorio hasta que reanudaba la sesión, y su aviso de confianza no enumeraba lo que la configuración del directorio activaría.

Restrinja o deshabilite los destinos de `/cd` con [reglas de permisos `Cd`](#cd).

<h3 id="additional-directories-grant-file-access-not-configuration">
  Los directorios adicionales otorgan acceso a archivos, no configuración
</h3>

Agregar un directorio extiende dónde Claude puede leer y editar archivos. No hace que ese directorio sea una raíz de configuración completa: la mayoría de la configuración `.claude/` no se descubre desde directorios adicionales, aunque algunos tipos se cargan como excepciones.

Estas excepciones se aplican solo a directorios agregados con la bandera `--add-dir` o el comando `/add-dir`, incluidos los directorios que el Agent SDK agrega a través de la bandera. Los directorios listados en `permissions.additionalDirectories` en un archivo de configuración otorgan acceso a archivos solamente y no cargan ninguna de la configuración a continuación.

La opción [`additionalDirectories`](/docs/es/agent-sdk/typescript#options) del Agent SDK en TypeScript y la opción [`add_dirs`](/docs/es/agent-sdk/python#claudeagentoptions) en Python reciben las excepciones también, aunque la opción de TypeScript comparte su nombre con la clave de configuración. El SDK pasa cada entrada a Claude Code como `--add-dir`, por lo que esos directorios se comportan como directorios agregados por bandera. Los skills, comandos y subagentes de cualquier directorio agregado por bandera se cargan a través de la [fuente de configuración](/docs/es/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) `project`, por lo que no se cargan cuando excluye esa fuente con [`--setting-sources`](/docs/es/cli-reference) en la CLI o `settingSources` en el SDK, y [modo bare](/docs/es/headless#start-faster-with-bare-mode) omite los comandos y subagentes entre ellos.

Los siguientes tipos de configuración se cargan desde directorios `--add-dir`:

| Configuración                                                                            | Cargado desde `--add-dir`                                                                                                                                                            |
| :--------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Skills](/docs/es/skills) en `.claude/skills/`                                                | Sí, con recarga en vivo                                                                                                                                                              |
| [Archivos de comando](/docs/es/skills#where-skills-live) en `.claude/commands/`               | Sí, sin recarga en vivo. Cuando el directorio agregado y su proyecto definen un comando con el mismo nombre, Claude Code ejecuta el comando de su proyecto                           |
| [Subagentes](/docs/es/sub-agents) en `.claude/agents/`                                        | Sí, sin recarga en vivo                                                                                                                                                              |
| [Configuración](/docs/es/settings) en `.claude/settings.json` y `.claude/settings.local.json` | Solo las claves `enabledPlugins` y [`extraKnownMarketplaces`](/docs/es/settings-reference#extraknownmarketplaces)                                                                         |
| Archivos [CLAUDE.md](/docs/es/memory), `.claude/rules/` y `CLAUDE.local.md`                   | Solo cuando `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1` está establecido. `CLAUDE.local.md` además requiere la fuente de configuración `local`, que está habilitada por defecto |

Para cargar los skills, comandos y subagentes desde un subdirectorio de su [directorio de trabajo principal](#working-directories) a mitad de sesión, ejecute `/add-dir` con la ruta de ese subdirectorio. Claude Code los carga para el resto de la sesión sin solicitarle que agregue un directorio de trabajo, porque el subdirectorio ya es legible. Esto requiere Claude Code v2.1.257 o posterior.

Claude Code descubre estilos de salida desde el directorio de trabajo actual y sus directorios padres, su directorio de usuario en `~/.claude/`, y configuración administrada. Los hooks y otras claves de `.claude/settings.json` se cargan desde la carpeta `.claude/` del directorio de trabajo actual sin recurso a directorios padres, junto con su `~/.claude/settings.json` de usuario y configuración administrada. `.claude/settings.local.json` se carga desde la raíz del repositorio git en su lugar, incluso cuando inicia Claude Code en un subdirectorio, excepto en los casos donde Claude Code [no usa la raíz del repositorio](/docs/es/settings#where-claude-code-looks-for-each-file), como en Windows; antes de v2.1.211, también se cargaba solo desde el directorio de trabajo actual. Las sesiones de [Agent SDK](/docs/es/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) la cargan desde el directorio de trabajo en todas las versiones.

Para compartir esa configuración entre proyectos, use uno de estos enfoques:

* **Configuración a nivel de usuario**: coloque archivos en `~/.claude/agents/`, `~/.claude/output-styles/`, o `~/.claude/settings.json` para hacerlos disponibles en cada proyecto
* **Plugins**: empaquete y distribuya configuración como un [plugin](/docs/es/plugins/overview) que los equipos pueden instalar
* **Lanzar desde el directorio de configuración**: ejecute Claude Code desde el directorio que contiene la configuración `.claude/` que desea

<h2 id="how-permissions-interact-with-sandboxing">
  Cómo interactúan los permisos con el sandboxing
</h2>

Los permisos y el [sandboxing](/docs/es/sandboxing) son capas de seguridad complementarias:

* **Permisos** controlan qué herramientas puede usar Claude Code y qué archivos o dominios puede acceder. Se aplican a Bash, Read, Edit, WebFetch, MCP y todas las demás herramientas, excepto que una regla de negación o ask no puede bloquear [`EndConversation`](/docs/es/tools-reference#endconversation-tool-behavior) mientras cualquier otra herramienta permanezca.
* **Sandboxing** proporciona aplicación a nivel del SO que restringe el acceso del sistema de archivos y red de los comandos Bash, PowerShell y [Monitor](/docs/es/tools-reference#monitor-tool) y sus procesos secundarios.

Use ambos para defensa en profundidad, ya que las restricciones de sandbox aún se aplican incluso si una inyección de solicitud omite la toma de decisiones de Claude. Las rutas y dominios de la configuración de sandbox y las reglas de permisos se [fusionan en la configuración final del sandbox](/docs/es/sandboxing#permission-rules).

Cuando el sandboxing está habilitado y deja `autoAllowBashIfSandboxed` en su valor predeterminado de `true`, los comandos Bash en sandbox se ejecutan sin solicitar incluso si sus permisos incluyen una regla ask `Bash` simple, o la [forma equivalente `Bash(*)`](#match-all-uses-of-a-tool): el límite del sandbox sustituye ese aviso de herramienta completa.

En [plan mode](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode), Claude Code omite esta sustitución. Sin una regla ask, los [comandos de solo lectura integrados](#read-only-commands) aún se ejecutan sin solicitar, y cualquier otro comando de shell pasa por el flujo de permisos regular mientras aún está planificando; consulte [plan mode](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode) para saber cómo Claude Code controla los comandos allí. Con una regla ask `Bash` simple, cada comando Bash solicita, incluyendo comandos de solo lectura en sandbox, igual que fuera del sandboxing. Antes de v2.1.212, la sustitución se aplicaba en plan mode también.

Estas comprobaciones aún se aplican:

* Las reglas ask con alcance de contenido como `Bash(git push *)` aún fuerzan un aviso
* Las reglas de negación explícitas aún se aplican
* Los comandos `rm` o `rmdir` que apunten a una [ruta crítica](/docs/es/permission-modes#critical-paths) aún pasan por el flujo de permisos regular

Los comandos que no se ejecutarán en sandbox, como comandos excluidos, respetan la regla ask `Bash` simple como es habitual. Consulte [modos de sandbox](/docs/es/sandboxing#sandbox-modes) para cambiar este comportamiento.

<span id="managed-only-settings" />

<h2 id="managed-settings">
  Configuración administrada
</h2>

Para organizaciones que necesitan control centralizado, los administradores implementan configuración administrada que la configuración de usuario y proyecto no pueden anular, excepto por algunas [claves sensibles a la seguridad](/docs/es/settings#exceptions-to-managed-settings-precedence). [Implementar configuración administrada](/docs/es/managed-settings) cubre los mecanismos de entrega, la precedencia dentro del nivel administrado, y las [claves que solo la configuración administrada puede establecer](/docs/es/managed-settings#managed-only-settings).

Una de esas claves, [`allowManagedPermissionRulesOnly`](/docs/es/settings-reference#allowmanagedpermissionrulesonly), hace que la configuración administrada sea la única fuente de configuración de reglas de permisos. Su entrada enumera todas las fuentes que Claude Code luego ignora.

`disableBypassPermissionsMode` generalmente se coloca en configuración administrada para aplicar la política organizacional, pero funciona desde cualquier alcance. Un usuario puede establecerlo en su propia configuración para bloquearse a sí mismo del modo de bypass.

<h2 id="settings-precedence">
  Precedencia de configuración
</h2>

Las reglas de permisos siguen la misma [precedencia de configuración](/docs/es/settings#settings-precedence) que todas las demás configuraciones de Claude Code, con la configuración administrada siendo la más alta: ningún otro nivel, incluyendo argumentos de línea de comandos, puede anular una regla de permiso administrada.

Si una herramienta se deniega en cualquier nivel, ningún otro nivel puede permitirla. Por ejemplo, una negación de configuración administrada no puede ser anulada por `--allowedTools`, y `--disallowedTools` puede agregar restricciones más allá de lo que define la configuración administrada.

Lo mismo se aplica en todos los ámbitos de configuración: si la configuración de usuario permite un permiso y la configuración de proyecto lo deniega, la regla de negación lo bloquea. Lo contrario también es cierto: una negación a nivel de usuario bloquea un permiso a nivel de proyecto, porque las reglas de negación de cualquier ámbito se evalúan antes que las reglas de permiso.

Los hosts de inserción pueden proporcionar política administrada adicional a través de la opción `managedSettings` del SDK, incluyendo reglas de permiso de permitir a menos que el administrador establezca los bloqueos `allowManaged*Only`; [Entregar política a sesiones de Claude Desktop](/docs/es/claude-apps-gateway#deliver-policy-to-claude-desktop-sessions) cubre cuándo se aplica la política del integrador en absoluto.

<h2 id="project-allow-rules-and-workspace-trust">
  Reglas de permiso del proyecto y confianza del espacio de trabajo
</h2>

Las reglas `permissions.allow` y las entradas `permissions.additionalDirectories` en el archivo `.claude/settings.json` de un proyecto otorgan capacidad, por lo que Claude Code las aplica solo después de que acepte el [diálogo de confianza del espacio de trabajo](/docs/es/security#additional-safeguards) para esa carpeta. El diálogo enumera las reglas y directorios que la carpeta otorgaría para que pueda revisarlos primero. Las reglas `deny` y `ask` no se ven afectadas, ya que solo restringen.

Claude Code guarda la confianza que acepta según dónde la inicie:

* En un repositorio, Claude Code guarda la confianza en la raíz del repositorio de git, por lo que la confianza cubre todo el repositorio excepto cualquier repositorio de git anidado dentro de él, como un submódulo. En un [worktree](/docs/es/worktrees), utiliza la raíz del checkout principal, como lo hace para [reglas guardadas](#permission-system).
* Fuera de un repositorio, Claude Code guarda la confianza en el directorio desde el que la inició, y la confianza cubre cualquier subdirectorio de ese directorio excepto un repositorio de git anidado dentro de él, como un clon. Cada subdirectorio cubierto cuenta entonces como una carpeta cuyo padre confió.
* Cuando comienza en su directorio de inicio, Claude Code mantiene la confianza solo para la sesión actual y no la escribe en el disco; consulte la nota sobre [salvaguardas adicionales](/docs/es/security#additional-safeguards).

Claude Code muestra el diálogo de confianza solo en sesiones interactivas. Una ejecución `claude -p` o una sesión SDK nunca lo muestra, y confiar en una carpeta principal no cuenta para estas reglas, por lo que [Lo que se ejecuta antes de confiar en una carpeta](#what-runs-before-you-trust-a-folder) dice qué contenido del repositorio Claude Code aún utiliza en cada una de esas dos situaciones.

<h3 id="when-your-local-settings-file-needs-trust">
  Cuándo su archivo de configuración local necesita confianza
</h3>

`.claude/settings.local.json` es normalmente su propio archivo, por lo que Claude Code aplica sus reglas de permiso y directorios adicionales sin el paso de confianza. Cuando el archivo se rastrea en git, o `.claude` es un enlace simbólico, Claude Code lo trata como proporcionado por el repositorio en su lugar y retiene sus reglas sin aplicarlas hasta que confíe en la carpeta.

Claude Code ejecuta git para distinguir los dos, y ejecuta git solo después de que haya confiado en la carpeta: aceptó el diálogo de confianza para ella o para un directorio principal cuya confianza se extiende a ella, o está en una sesión `-p` o SDK, que cuenta como aceptada. Hasta entonces, dónde inició Claude Code decide qué sucede con las reglas del archivo:

* **En su directorio de configuración personal:** Claude Code aplica el `.claude/settings.local.json` de esa carpeta de inmediato sin ejecutar git. Su directorio de configuración personal es su directorio de inicio, o un directorio cuyo subdirectorio `.claude` ha establecido como [`CLAUDE_CONFIG_DIR`](/docs/es/env-vars#variables). Si ese directorio `CLAUDE_CONFIG_DIR` se encuentra dentro de un repositorio de git y Claude Code [mantiene su configuración local en la raíz del repositorio](/docs/es/settings#where-claude-code-looks-for-each-file) en su lugar, retiene las reglas sin aplicarlas, como en cualquier otro lugar.
* **En cualquier otro lugar:** Claude Code retiene las reglas del archivo sin aplicarlas, como las de la configuración del proyecto. Una vez que la verificación ha ejecutado, Claude Code aplica las reglas de un archivo sin rastrear, o de un archivo en un directorio fuera de cualquier repositorio de git, aunque no haya confiado en esa carpeta exacta.

<Note>
  La excepción del directorio de configuración omite solo el paso de confianza. `~/.claude/settings.local.json` sigue siendo [alcance local](/docs/es/settings#compare-the-scope-of-each-settings-file), por lo que Claude Code lo lee solo en sesiones que inicia en su directorio de inicio mismo, no en cada proyecto. Para aplicar reglas de permiso en todos sus proyectos, agréguelas a su configuración de usuario en su lugar: `~/.claude/settings.json`, o `$CLAUDE_CONFIG_DIR/settings.json` cuando `CLAUDE_CONFIG_DIR` está establecido.
</Note>

En las versiones 2.1.196 a 2.1.199, Claude Code mantenía las reglas del archivo en su directorio de configuración personal y fuera de repositorios de git también, e imprimía la advertencia [`this workspace has not been trusted`](/docs/es/errors#workspace-has-not-been-trusted) allí. Antes de v2.1.207, Claude Code aplicaba las reglas de un archivo sin rastrear antes de que aceptara el diálogo.

<h3 id="what-runs-before-you-trust-a-folder">
  Lo que se ejecuta antes de confiar en una carpeta
</h3>

Cada fila es un tipo de contenido que un repositorio puede proporcionar. Las columnas son las dos situaciones en las que no ha confiado en la carpeta misma: confió solo en una carpeta principal, o ejecutó `claude -p` o el SDK allí, que nunca muestra el diálogo de confianza. La columna de carpeta principal no se aplica dentro de un [repositorio anidado](#project-allow-rules-and-workspace-trust): en una sesión interactiva Claude Code muestra el diálogo de confianza para ella, y una ejecución `claude -p` o SDK allí sigue la columna `claude -p`.

| Lo que proporciona el repositorio                                                                                                                                                                                                                                                                                                    | Confió solo en una carpeta principal                                                                                                                                                                   | `claude -p` o el SDK, carpeta nunca confiada                                                                                                                                                            |
| :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [Hooks](/docs/es/hooks) en archivos de configuración, el bloque [`env`](/docs/es/settings-reference#env) y comandos auxiliares como [`apiKeyHelper`](/docs/es/settings-reference#apikeyhelper), y los [hooks](/docs/es/hooks#hooks-in-skills-and-agents) de una skill del proyecto y [`allowed-tools`](/docs/es/skills#pre-approve-tools-for-a-skill)         | Utilizado                                                                                                                                                                                              | Utilizado. La confianza del espacio de trabajo nunca bloquea `allowed-tools` de una skill en ninguna sesión                                                                                             |
| Reglas `permissions.allow` y `additionalDirectories` en `.claude/settings.json`                                                                                                                                                                                                                                                      | No se utiliza hasta que acepte el diálogo de confianza, que aparece nuevamente enumerándolas                                                                                                           | No se utiliza. Claude Code imprime una advertencia [`this workspace has not been trusted`](/docs/es/errors#workspace-has-not-been-trusted) a stderr                                                          |
| Hooks de frontmatter en un [subagente](/docs/es/sub-agents#hooks-in-subagent-frontmatter) del proyecto, un plugin [`@skills-dir`](/docs/es/plugins/loading#plugins-shared-through-a-repository) del proyecto, y entradas [`extraKnownMarketplaces`](/docs/es/settings-reference#extraknownmarketplaces) del repositorio o un directorio `--add-dir` | No se utiliza, y no se ofrece diálogo                                                                                                                                                                  | No se utiliza                                                                                                                                                                                           |
| [`mcpServers`](/docs/es/sub-agents#scope-mcp-servers-to-a-subagent) en línea en el frontmatter de un subagente del repositorio o un directorio `--add-dir`. Antes de v2.1.238, Claude Code cargaba estos servidores en ambas situaciones                                                                                                  | No se utiliza, y no se ofrece diálogo                                                                                                                                                                  | No se utiliza                                                                                                                                                                                           |
| Servidores en `.mcp.json`, incluidos los que el repositorio [aprueba en su propia configuración](/docs/es/mcp#project-server-approvals-and-workspace-trust)                                                                                                                                                                               | Claude Code le pregunta antes de conectarlos. Las aprobaciones propias del repositorio no cuentan                                                                                                      | Conectado sin preguntar, aprobado o no. El SDK los carga solo cuando `settingSources` incluye configuración del proyecto. `claude mcp list` en la misma carpeta aún reporta tal servidor como pendiente |
| Un [`headersHelper`](/docs/es/mcp#trust-a-folder-before-its-headershelper-runs) en un servidor en `.mcp.json`. Antes de v2.1.238, Claude Code ejecutaba el auxiliar en ambas situaciones                                                                                                                                                  | No se ejecuta hasta que acepte el diálogo de confianza, que aparece nuevamente nombrando dónde se declara el auxiliar. Claude Code conecta el servidor solo con sus `headers` estáticos hasta entonces | No se ejecuta. Claude Code conecta el servidor con sus `headers` estáticos y imprime una línea [`headersHelper not run`](/docs/es/errors#headershelper-not-run) por servidor a stderr                        |

Para las filas que necesitan que esta carpeta exacta sea confiada, confíe en ella manualmente: establezca `projects["<path>"].hasTrustDialogAccepted` en `true` en `~/.claude.json`, donde `<path>` es la raíz del repositorio, o la carpeta misma fuera de un repositorio. Claude Code imprime la clave exacta en la línea de registro de depuración para un hook de subagente omitido o servidor MCP en línea, en la advertencia de stderr para reglas de permiso omitidas, y en la línea `headersHelper not run` para un auxiliar omitido.

Antes de ejecutar `claude -p` en un repositorio que no escribió, decida qué puede ejecutar en su máquina:

* Pase `--setting-sources user`, o establezca el `settingSources` del SDK sin configuración del proyecto, para que Claude Code no lea ni los archivos de configuración del proyecto ni su `.mcp.json`
* Comience con [`--bare`](/docs/es/headless#start-faster-with-bare-mode) para que Claude Code no lea hooks, skills, comandos personalizados, subagentes, plugins, o servidores `.mcp.json` del proyecto. El bloque `env` del proyecto y auxiliares como `awsAuthRefresh` en sus archivos de configuración aún se aplican, y Claude Code lee `apiKeyHelper` solo desde `--settings`
* Pase `--settings '{"disableAllHooks": true}'` para [desactivar hooks](/docs/es/hooks#disable-or-remove-hooks) para esa ejecución. Establecerlo solo en su configuración de usuario no es suficiente, porque la configuración del proyecto del repositorio tiene prioridad sobre la suya y puede establecerlo nuevamente en `false`
* Agregue una entrada [`disabledMcpjsonServers`](/docs/es/settings-reference#disabledmcpjsonservers) para rechazar un servidor `.mcp.json` por nombre en cada tipo de sesión

<h2 id="example-configurations">
  Configuraciones de ejemplo
</h2>

Este [repositorio](https://github.com/anthropics/claude-code/tree/main/examples/settings) incluye configuraciones de configuración inicial para escenarios de implementación comunes. Use estos como puntos de partida y ajústelos para que se adapten a sus necesidades.

<h2 id="see-also">
  Ver también
</h2>

* [Todos los ajustes](/docs/es/settings-reference#permission-settings): cada clave de configuración, incluyendo las claves de permisos
* [Configurar modo auto](/docs/es/auto-mode-config): indique al clasificador del modo auto qué infraestructura confía su organización
* [Sandboxing](/docs/es/sandboxing): aislamiento del sistema de archivos y red a nivel del SO para comandos Bash
* [Autenticación](/docs/es/authentication): configure el acceso de usuario a Claude Code
* [Seguridad](/docs/es/security): salvaguardas de seguridad y mejores prácticas
* [Hooks](/docs/es/hooks-guide): automatice flujos de trabajo y extienda la evaluación de permisos
