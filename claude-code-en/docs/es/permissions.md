> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurar permisos

> Controle lo que Claude Code puede acceder y hacer con reglas de permisos granulares, modos y políticas administradas.

Claude Code admite permisos granulares para que pueda especificar exactamente qué puede hacer el agente y qué no puede hacer. La configuración de permisos se puede registrar en el control de versiones y distribuir a todos los desarrolladores de su organización, así como personalizarse por desarrolladores individuales.

<h2 id="permission-system">
  Sistema de permisos
</h2>

Claude Code utiliza un sistema de permisos escalonado para equilibrar potencia y seguridad:

| Tipo de herramienta      | Ejemplo                    | Se requiere aprobación                                                                 | Comportamiento de "Sí, no preguntar de nuevo"        |
| :----------------------- | :------------------------- | :------------------------------------------------------------------------------------- | :--------------------------------------------------- |
| Solo lectura             | Lecturas de archivos, Grep | No, dentro del [directorio de trabajo y directorios adicionales](#working-directories) | N/A                                                  |
| Comandos Bash            | Ejecución de shell         | Sí, excepto un conjunto integrado de [comandos de solo lectura](#read-only-commands)   | Permanentemente por directorio de proyecto y comando |
| Modificación de archivos | Editar/escribir archivos   | Sí                                                                                     | Hasta el final de la sesión                          |

En un mensaje de permiso de Bash o PowerShell, presione `Ctrl+E` para mostrar una explicación del comando: qué hace, por qué Claude lo está ejecutando y qué podría salir mal, etiquetado como **Riesgo bajo**, **Riesgo medio** o **Riesgo alto**. Claude Code envía el comando y la propia descripción de Claude de la llamada al modelo para generar la explicación solo cuando presiona `Ctrl+E`, no en cada mensaje. Mostrar la explicación no ejecuta el comando; presione `Ctrl+E` de nuevo para ocultarla.

Para desactivar el atajo, establezca [`permissionExplainerEnabled`](/docs/es/settings#global-config-settings) en `false` en `~/.claude.json`.

<h2 id="manage-permissions">
  Administrar permisos
</h2>

Puede ver y administrar los permisos de herramientas de Claude Code con `/permissions`. Esta interfaz de usuario enumera todas las reglas de permisos y el archivo `settings.json` del que se obtienen.

* Las reglas **Allow** permiten que Claude Code use la herramienta especificada sin aprobación manual.
* Las reglas **Ask** solicitan confirmación cada vez que Claude Code intenta usar la herramienta especificada.
* Las reglas **Deny** impiden que Claude Code use la herramienta especificada.

Las reglas se evalúan en orden: deny, luego ask, luego allow. La primera coincidencia en ese orden determina el resultado, y la especificidad de la regla no cambia el orden.

Una regla deny amplia como `Bash(aws *)` bloquea cada llamada coincidente, incluidas las llamadas que también coinciden con una regla allow más específica como `Bash(aws s3 ls)`, por lo que una regla deny no puede llevar excepciones de lista de permitidos. La misma precedencia se aplica entre ask y allow: una regla ask coincidente solicita confirmación incluso cuando una regla allow más específica también coincide con la misma llamada.

Las reglas deny se comportan de manera diferente dependiendo de si nombran una herramienta o delimitan un patrón dentro de una. Un nombre de herramienta simple como `Bash` elimina la herramienta del contexto de Claude por completo, por lo que Claude nunca la ve. Una regla delimitada como `Bash(rm *)` deja la herramienta disponible y bloquea las llamadas coincidentes cuando Claude intenta usarlas.

<Note>
  Las reglas de permisos se aplican mediante Claude Code, no por el modelo. Las instrucciones en su prompt o `CLAUDE.md` determinan lo que Claude intenta hacer, pero no cambian lo que Claude Code permite. Para otorgar o revocar acceso, use `/permissions`, las reglas descritas aquí, un [modo de permisos](/docs/es/permission-modes), o un [hook PreToolUse](#extend-permissions-with-hooks).
</Note>

<h2 id="permission-modes">
  Modos de permisos
</h2>

Claude Code admite varios modos de permisos que controlan cómo se aprueban las llamadas de herramientas. Consulte [Modos de permisos](/docs/es/permission-modes) para saber cuándo usar cada uno. Establezca `defaultMode` en sus [archivos de configuración](/docs/es/settings#settings-files):

| Modo                | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Comportamiento estándar: solicita permiso en el primer uso de cada herramienta. Etiquetado como Manual en la CLI, en las extensiones de VS Code y JetBrains, y en la aplicación de escritorio, y Claude Code acepta `manual` como un alias. La etiqueta y el alias requieren Claude Code v2.1.200 o posterior. La etiqueta de la aplicación de escritorio no depende de su versión de CLI                                                    |
| `acceptEdits`       | Acepta automáticamente ediciones de archivos y comandos comunes del sistema de archivos como `mkdir`, `touch`, `mv` y `cp` para rutas en el directorio de trabajo o `additionalDirectories`                                                                                                                                                                                                                                                  |
| `plan`              | Claude lee archivos y ejecuta comandos de shell de solo lectura para explorar pero no edita sus archivos de origen. Etiquetado como Plan en la CLI y en la extensión de VS Code                                                                                                                                                                                                                                                              |
| `auto`              | Auto-aprueba llamadas de herramientas con comprobaciones de seguridad en segundo plano que verifican que las acciones se alineen con su solicitud                                                                                                                                                                                                                                                                                            |
| `dontAsk`           | Deniega automáticamente las herramientas a menos que estén preaprobadas a través de `/permissions` o reglas `permissions.allow`. `AskUserQuestion`, herramientas de conector [que su organización configuró en `ask`](/docs/es/mcp#organization-controls-on-connector-tools), y herramientas MCP marcadas [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool) se deniegan incluso si las ha permitido                       |
| `bypassPermissions` | Omite avisos de permisos, excepto aquellos forzados por reglas `ask` explícitas, herramientas de conector [que su organización configuró en `ask`](/docs/es/mcp#organization-controls-on-connector-tools), y herramientas MCP marcadas [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool). Las eliminaciones de directorio raíz y directorio de inicio como `rm -rf /` también aún solicitan como un disyuntor de circuito |

<Warning>
  El modo `bypassPermissions` omite avisos de permisos, incluyendo escrituras en `.git`, `.config/git`, `.claude`, `.vscode`, `.idea`, `.husky`, `.cargo`, `.devcontainer`, `.yarn` y `.mvn`. Use este modo solo en entornos aislados como contenedores o máquinas virtuales donde Claude Code no pueda causar daño.

  Algunos avisos aún se activan en este modo. Las reglas `ask` explícitas, herramientas de conector [que su organización configuró en `ask`](/docs/es/mcp#organization-controls-on-connector-tools), y herramientas MCP marcadas [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool) aún solicitan. Las eliminaciones dirigidas al directorio raíz del sistema de archivos o al directorio de inicio, como `rm -rf /` y `rm -rf ~`, también solicitan como un disyuntor de circuito contra errores del modelo, incluyendo cuando el comando contiene sustitución de comandos con `$(...)` o comillas invertidas, o sustitución de procesos con `<(...)`. Antes de v2.1.208, solo la forma simple, como `rm -rf ~` escrito como su propio comando, solicitaba; los comandos que llegaban a la eliminación a través de una sustitución no lo hacían.
</Warning>

Para evitar que se use el modo `bypassPermissions` o `auto`, establezca `permissions.disableBypassPermissionsMode` o `permissions.disableAutoMode` en `"disable"` en cualquier [archivo de configuración](/docs/es/settings#settings-files). Estos son más útiles en [configuración administrada](#managed-settings) donde no pueden ser anulados.

<h2 id="permission-rule-syntax">
  Sintaxis de reglas de permisos
</h2>

Las reglas de permisos siguen el formato `Tool` o `Tool(specifier)`.

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

Las reglas de denegación y solicitud pueden coincidir con un parámetro de entrada de nivel superior en cualquier herramienta con `Tool(param:value)`. La regla coincide cuando Claude llama a la herramienta con ese parámetro establecido en ese valor exacto. Una regla de permiso para un valor de parámetro no establecería que la llamada sea segura en general, por lo que las reglas de permiso continúan usando la sintaxis de especificador propia de cada herramienta. Esto funciona para cualquier parámetro escalar que acepte la herramienta:

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

Los campos que una herramienta ya coincide con sus propias reglas de canonicalización no son coincidentes de esta manera: `command` para Bash y PowerShell, `file_path` para Read, Edit y Write, `path` para Grep y Glob, `notebook_path` para NotebookEdit, y `url` para WebFetch. Una regla como `Bash(command:rm *)` sería eludible por un comando compuesto, por lo que Claude Code la ignora y emite una advertencia de inicio. Use `Bash(rm *)`, `Read(./path)`, o `WebFetch(domain:host)` en su lugar.

<h3 id="wildcard-patterns">
  Patrones de comodín
</h3>

Las reglas de Bash admiten patrones glob con `*`. Los comodines pueden aparecer en cualquier posición del comando. Esta configuración permite comandos npm y git commit mientras bloquea git push:

```json theme={null}
{
  "permissions": {
    "allow": [
      "Bash(npm run *)",
      "Bash(git commit *)",
      "Bash(git * main)",
      "Bash(* --version)",
      "Bash(* --help *)"
    ],
    "deny": [
      "Bash(git push *)"
    ]
  }
}
```

El espacio antes de `*` importa: `Bash(ls *)` coincide con `ls -la` pero no con `lsof`, mientras que `Bash(ls*)` coincide con ambos. El sufijo `:*` es una forma equivalente de escribir un comodín final, por lo que `Bash(ls:*)` coincide con los mismos comandos que `Bash(ls *)`.

El diálogo de permisos escribe la forma separada por espacios cuando selecciona "Sí, no preguntar de nuevo" para un prefijo de comando. La forma `:*` solo se reconoce al final de un patrón. En un patrón como `Bash(git:* push)`, los dos puntos se tratan como un carácter literal y no coincidirán con comandos git.

<h3 id="tool-name-wildcards">
  Comodines de nombre de herramienta
</h3>

Las reglas de denegación y solicitud también aceptan patrones glob en la posición del nombre de la herramienta. El patrón debe coincidir con el nombre completo de la herramienta: `"*"` coincide con todas las herramientas, y `"mcp__*"` coincide con todas las herramientas MCP en todos los servidores. Una herramienta coincidida por una regla de denegación de nombre simple se elimina del contexto de Claude, igual que un nombre de herramienta simple. Esta configuración deniega todas las herramientas MCP:

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

Una regla de denegación o solicitud cuyo nombre de herramienta no coincide con ninguna herramienta conocida produce una advertencia de inicio para detectar errores tipográficos. Los nombres de herramientas que contienen `_` o `*` están exentos de la verificación.

La etiqueta mostrada para una herramienta en la transcripción y el diálogo de permisos puede diferir de su nombre canónico. Por ejemplo, la herramienta etiquetada como `Stop Task` en la transcripción tiene el nombre canónico `TaskStop`. Las reglas de permisos y los [coincidentes de hooks](/docs/es/hooks) coinciden solo con el nombre canónico, por lo que una regla escrita como `Stop Task` no coincide. Para reglas de denegación y solicitud, la advertencia de inicio anterior detecta la discrepancia. Use los nombres canónicos enumerados en la [referencia de herramientas](/docs/es/tools-reference).

<h2 id="tool-specific-permission-rules">
  Reglas de permisos específicas de herramientas
</h2>

<h3 id="bash">
  Bash
</h3>

Las reglas de permisos de Bash admiten coincidencia de comodines con `*`. Los comodines pueden aparecer en cualquier posición del comando, incluyendo al principio, en el medio o al final:

* `Bash(npm run build)` coincide con el comando Bash exacto `npm run build`
* `Bash(npm run test *)` coincide con comandos Bash que comienzan con `npm run test`
* `Bash(npm *)` coincide con cualquier comando que comience con `npm `
* `Bash(* install)` coincide con cualquier comando que termine con ` install`
* `Bash(git * main)` coincide con comandos como `git checkout main` y `git log --oneline main`

Un único `*` coincide con cualquier secuencia de caracteres incluyendo espacios, por lo que un comodín puede abarcar múltiples argumentos. `Bash(git *)` coincide con `git log --oneline --all`, y `Bash(git * main)` coincide con `git push origin main` así como con `git merge main`.

Cuando `*` aparece al final con un espacio antes (como `Bash(ls *)`), aplica un límite de palabra, requiriendo que el prefijo sea seguido por un espacio o fin de cadena. Por ejemplo, `Bash(ls *)` coincide con `ls -la` pero no con `lsof`. En contraste, `Bash(ls*)` sin espacio coincide con ambos `ls -la` y `lsof` porque no hay restricción de límite de palabra.

<h4 id="compound-commands">
  Comandos compuestos
</h4>

<Tip>
  Claude Code es consciente de los operadores de shell, por lo que una regla como `Bash(safe-cmd *)` no le dará permiso para ejecutar el comando `safe-cmd && other-cmd`. Los separadores de comando reconocidos son `&&`, `||`, `;`, `|`, `|&`, `&` y saltos de línea. Una regla debe coincidir con cada subcomando de forma independiente.
</Tip>

Cuando aprueba un comando compuesto con "Sí, no preguntar de nuevo", Claude Code guarda una regla separada para cada subcomando que requiere aprobación, en lugar de una sola regla para la cadena completa. Por ejemplo, aprobar `git status && npm test` guarda una regla para `npm test`, por lo que futuras invocaciones de `npm test` se reconocen independientemente de lo que preceda a `&&`. Los subcomandos como `cd` en un subdirectorio generan su propia regla Read para esa ruta. Se pueden guardar hasta 5 reglas para un solo comando compuesto.

<h4 id="process-wrappers">
  Envoltorios de procesos
</h4>

Antes de coincidir con reglas de Bash, Claude Code elimina un conjunto fijo de envoltorios de procesos para que una regla como `Bash(npm test *)` también coincida con `timeout 30 npm test`. Los envoltorios reconocidos son `timeout`, `time`, `nice`, `nohup` y `stdbuf`.

`xargs` desnudo también se elimina, por lo que `Bash(grep *)` coincide con `xargs grep pattern`. La eliminación solo se aplica cuando `xargs` no tiene banderas: una invocación como `xargs -n1 grep pattern` se coincide como un comando `xargs`, por lo que las reglas escritas para el comando interno no la cubren.

Esta lista de envoltorios está integrada y no es configurable. Los ejecutores de entorno de desarrollo como `direnv exec`, `devbox run`, `mise exec`, `npx` y `docker exec` no están en la lista. Porque estas herramientas ejecutan sus argumentos como un comando, una regla como `Bash(devbox run *)` coincide con lo que viene después de `run`, incluyendo `devbox run rm -rf .`. Para aprobar trabajo dentro de un ejecutor de entorno, escriba una regla específica que incluya tanto el ejecutor como el comando interno, como `Bash(devbox run npm test)`. Agregue una regla por comando interno que desee permitir.

Los envoltorios exec como `watch`, `setsid`, `ionice` y `flock` siempre solicitan y no pueden ser auto-aprobados por una regla de prefijo como `Bash(watch *)`. Lo mismo se aplica a `find` con `-exec` o `-delete`: una regla `Bash(find *)` no cubre estas formas. Para aprobar una invocación específica, escriba una regla de coincidencia exacta para la cadena de comando completa.

<h4 id="read-only-commands">
  Comandos de solo lectura
</h4>

Claude Code reconoce un conjunto integrado de comandos Bash como de solo lectura y los ejecuta sin un aviso de permisos en cada modo. Estos incluyen `ls`, `cat`, `echo`, `pwd`, `head`, `tail`, `grep`, `find`, `wc`, `which`, `diff`, `stat`, `du`, `cd` y formas de solo lectura de `git`. El conjunto no es configurable; para requerir un aviso para uno de estos comandos, agregue una regla `ask` o `deny` para él.

Los patrones glob sin comillas se permiten para comandos cuya cada bandera es de solo lectura, por lo que `ls *.ts` y `wc -l src/*.py` se ejecutan sin un aviso. Los comandos con banderas capaces de escritura o ejecución, como `find`, `sort`, `sed` y `git`, aún solicitan cuando un glob sin comillas está presente porque el glob podría expandirse a una bandera como `-delete`.

Un `cd` en una ruta dentro de su directorio de trabajo o un [directorio adicional](#working-directories) también es de solo lectura. Un comando compuesto como `cd packages/api && ls` se ejecuta sin un aviso cuando cada parte se califica por su cuenta. Combinar `cd` con `git` en un comando compuesto solicita cuando el `cd` cambia a un directorio diferente, ya que ejecutar `git` en un nuevo directorio puede ejecutar los hooks de ese directorio. Un `cd` cuyo destino se resuelve al directorio de trabajo actual es una no-operación y no activa este aviso.

Combinar `cd` con una redirección de salida en un comando compuesto también solicita cuando Claude Code no puede determinar a qué directorio se resuelve el objetivo de redirección después de que se ejecuta `cd`. Un comando cuyo único objetivo de redirección es `/dev/null`, como `cd app; grep -r pattern . 2>/dev/null`, no activa este aviso, porque `/dev/null` no depende del directorio de trabajo. Antes de v2.1.207, un comando compuesto que contenía `cd` solicitaba cualquier redirección de salida, incluyendo una cuyo único objetivo era `/dev/null`.

<Warning>
  Los patrones de permisos de Bash que intentan restringir argumentos de comando son frágiles. Por ejemplo, `Bash(curl http://github.com/ *)` intenta restringir curl a URLs de GitHub, pero no coincidirá con variaciones como:

  * Opciones antes de URL: `curl -X GET http://github.com/...`
  * Protocolo diferente: `curl https://github.com/...`
  * Redirecciones: `curl -L http://bit.ly/xyz`, que redirige a GitHub
  * Variables: `URL=http://github.com && curl $URL`
  * Espacios adicionales: `curl  http://github.com`

  Para un filtrado de URL más confiable, considere:

  * **Restringir herramientas de red de Bash**: use reglas de negación para bloquear `curl`, `wget` y comandos similares, luego use la herramienta WebFetch con permiso `WebFetch(domain:github.com)` para dominios permitidos
  * **Usar hooks PreToolUse**: implemente un hook que valide URLs en comandos Bash y bloquee dominios no permitidos
  * **Agregar orientación CLAUDE.md**: describa sus patrones curl permitidos en `CLAUDE.md`. Esto forma lo que Claude intenta pero no aplica un límite, así que emparéjelo con una de las opciones anteriores

  Tenga en cuenta que usar WebFetch solo no previene el acceso a la red. Si se permite Bash, Claude aún puede usar `curl`, `wget` u otras herramientas para alcanzar cualquier URL.
</Warning>

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

Claude Code analiza el AST de PowerShell y verifica cada comando en un comando compuesto de forma independiente. Los operadores de tubería `|`, separadores de declaración `;` y en PowerShell 7+ los operadores de cadena `&&` y `||` dividen un comando compuesto en subcomandos. Una regla debe coincidir con cada subcomando para que se permita el comando compuesto.

<h3 id="read-and-edit">
  Read y Edit
</h3>

Las reglas `Edit` se aplican a todas las herramientas integradas que editan archivos. Claude hace un esfuerzo de mejor intento para aplicar reglas `Read` a todas las herramientas integradas que leen archivos como Grep y Glob, a menciones `@file` en sus indicaciones, y a la selección y contexto de archivo abierto que un [IDE](/docs/es/vs-code#the-built-in-ide-mcp-server) conectado comparte con Claude.

Una regla de negación `Read` también bloquea la [herramienta Edit](/docs/es/errors#file-is-covered-by-a-read-deny-rule) en la misma ruta, incluyendo crear un archivo nuevo allí. Write y NotebookEdit no están cubiertos, así que agregue una regla de negación `Edit` para rutas que ninguna herramienta pueda cambiar. Requiere Claude Code v2.1.208 o posterior.

<Warning>
  Las reglas de negación Read y Edit se aplican a las herramientas de archivo integradas de Claude y a los comandos de archivo que Claude Code reconoce en Bash, como `cat`, `head`, `tail` y `sed`. No se aplican a subprocesos arbitrarios que leen o escriben archivos indirectamente, como un script de Python o Node que abre archivos por sí mismo. Para aplicación a nivel del SO que bloquea todos los procesos de acceder a una ruta, [habilite el sandbox](/docs/es/sandboxing).
</Warning>

Las reglas Read y Edit siguen la especificación [gitignore](https://git-scm.com/docs/gitignore) con cuatro tipos de patrones distintos:

| Patrón            | Significado                                         | Ejemplo                          | Coincide                                                  |
| ----------------- | --------------------------------------------------- | -------------------------------- | --------------------------------------------------------- |
| `//path`          | Ruta absoluta desde la raíz del sistema de archivos | `Read(//Users/alice/secrets/**)` | `/Users/alice/secrets/**`                                 |
| `~/path`          | Ruta desde el directorio home                       | `Read(~/Documents/*.pdf)`        | `/Users/alice/Documents/*.pdf`                            |
| `/path`           | Ruta relativa a la fuente de configuración          | `Edit(/src/**/*.ts)`             | `<project root>/src/**/*.ts` en configuración de proyecto |
| `path` o `./path` | Ruta relativa al directorio actual                  | `Read(*.env)`                    | `<cwd>/*.env`                                             |

<Warning>
  Un patrón como `/Users/alice/file` no es una ruta absoluta. La barra diagonal inicial única se ancla en la fuente de configuración, no en la raíz del sistema de archivos. Use `//Users/alice/file` para rutas absolutas.
</Warning>

Un patrón `/path` se ancla en el directorio asociado con el archivo de configuración que lo define, por lo que la misma regla coincide con diferentes ubicaciones dependiendo de dónde la coloque:

| Regla definida en                                               | `/path` se resuelve a      |
| :-------------------------------------------------------------- | :------------------------- |
| Configuración de proyecto o local, como `.claude/settings.json` | `<project root>/path`      |
| Configuración de usuario en `~/.claude/settings.json`           | `~/.claude/path`           |
| Un archivo pasado con `--settings <file>`                       | `<directory of file>/path` |
| Banderas CLI, `/permissions` o reglas de sesión                 | `<original cwd>/path`      |

Una regla de negación como `Read(/secrets/**)` en configuración de usuario bloquea `~/.claude/secrets/**`, no un directorio `secrets` en su proyecto. Para escribir una regla en configuración de usuario que se aplique dentro de cada proyecto, use una ruta absoluta `//` o una ruta relativa a home `~/` en su lugar.

En Windows, las rutas se normalizan a forma POSIX antes de coincidir. `C:\Users\alice` se convierte en `/c/Users/alice`, así que use `//c/**/.env` para coincidir con archivos `.env` en cualquier lugar de esa unidad. Para coincidir en todas las unidades, use `//**/.env`.

Ejemplos:

* `Edit(/docs/**)`: edita en `<project>/docs/`, no `/docs/` o `<project>/.claude/docs/`
* `Read(~/.zshrc)`: lee el `.zshrc` de su directorio home
* `Edit(//tmp/scratch.txt)`: edita la ruta absoluta `/tmp/scratch.txt`
* `Read(src/**)`: lee desde `<current-directory>/src/`

Una regla solo coincide con archivos bajo su anclaje, por lo que el anclaje determina cuán lejos llega una regla de negación. Los nombres de archivo desnudos siguen la semántica de gitignore y coinciden en cualquier profundidad, por lo que `Read(.env)` y `Read(**/.env)` son equivalentes:

| Regla de negación              | Bloquea                                                     | No bloquea                                                     |
| ------------------------------ | ----------------------------------------------------------- | -------------------------------------------------------------- |
| `Read(.env)` o `Read(**/.env)` | cualquier `.env` en o bajo el directorio actual             | `.env` en un directorio padre u otro proyecto                  |
| `Read(//**/.env)`              | cualquier `.env` en cualquier lugar del sistema de archivos | nada; la regla está anclada en la raíz del sistema de archivos |

<Note>
  En patrones gitignore, `*` coincide dentro de un solo segmento de ruta y puede aparecer en cualquier posición en el patrón, mientras que `**` coincide en directorios. Para permitir todo acceso a archivos, use solo el nombre de la herramienta sin paréntesis: `Read`, `Edit` o `Write`.
</Note>

Cuando aprueba una ruta de archivo con "Sí, no preguntar de nuevo", Claude Code escapa caracteres de patrón gitignore en esa ruta, como `[`, `]` y `*`, por lo que la regla generada coincide solo con la ruta literal que aprobó. Las reglas que escribe usted mismo no se escapan. Antes de v2.1.202, Claude Code guardaba la ruta sin escapar, por lo que una regla generada para un directorio llamado `[2024-06] Reports` podría fallar en coincidir con su propia ruta o coincidir con directorios hermanos no deseados.

Cuando Claude accede a un symlink, las reglas de permisos verifican dos rutas: el symlink mismo y el archivo al que se resuelve. Las reglas de permiso y negación tratan ese par de manera diferente: las reglas de permiso recurren a solicitarle, mientras que las reglas de negación bloquean directamente.

* **Reglas de permiso**: se aplican solo cuando tanto la ruta del symlink como su destino coinciden. Un symlink dentro de un directorio permitido que apunta fuera de él aún le solicita.
* **Reglas de negación**: se aplican cuando la ruta del symlink o su destino coincide. Un symlink que apunta a un archivo denegado está denegado.

Por ejemplo, con `Read(./project/**)` permitido y `Read(~/.ssh/**)` denegado, un symlink en `./project/key` que apunta a `~/.ssh/id_rsa` está bloqueado: el destino falla la regla de permiso y coincide con la regla de negación.

<h3 id="webfetch">
  WebFetch
</h3>

Las reglas WebFetch usan un prefijo `domain:` y coinciden con el nombre de host de la URL solicitada. La coincidencia no distingue mayúsculas de minúsculas, admite comodines `*` y elimina un punto final de ambas la regla y el nombre de host para que `example.com.` y `example.com` se traten igual.

* `WebFetch(domain:example.com)` coincide con solicitudes a `example.com`
* `WebFetch(domain:*.example.com)` coincide con cualquier subdominio en cualquier profundidad, como `api.example.com` o `a.b.example.com`, pero no con `example.com` mismo
* `WebFetch(domain:*)` coincide con cada dominio y es equivalente a una regla `WebFetch` desnuda

En cualquier posición que no sea un `*.` inicial o un `*` desnudo, el comodín coincide solo con el texto entre dos puntos. `WebFetch(domain:example.*)` coincide con `example.org`, donde `*` se convierte en `org`, pero no con `example.evil.com`, donde `*` tendría que convertirse en `evil.com` y cruzar un punto. Esto evita que un comodín final coincida con dominios que un atacante podría registrar.

<h3 id="mcp">
  MCP
</h3>

Las reglas MCP usan el nombre del servidor tal como se configura en Claude Code, opcionalmente seguido del nombre de una herramienta de ese servidor.

* `mcp__puppeteer` coincide con cualquier herramienta proporcionada por el servidor `puppeteer`
* `mcp__puppeteer__*` usa sintaxis de comodín y también coincide con todas las herramientas del servidor `puppeteer`
* `mcp__puppeteer__puppeteer_navigate` coincide con la herramienta `puppeteer_navigate` proporcionada por el servidor `puppeteer`

Si su organización ha establecido una herramienta de conector [claude.ai](/docs/es/mcp#organization-controls-on-connector-tools) en `ask`, las reglas de permiso para esa herramienta no tienen efecto: Claude Code solicita en cada llamada, incluso en modos `auto` y `bypassPermissions`. En modo `dontAsk`, que nunca solicita, Claude Code niega la llamada en su lugar. Las herramientas de conector aparecen como `mcp__claude_ai_<server>__<tool>`.

<h3 id="agent-subagents">
  Agent (subagents)
</h3>

Use reglas `Agent(AgentName)` para controlar qué [subagents](/docs/es/sub-agents) puede usar Claude:

* `Agent(Explore)` coincide con el subagent Explore
* `Agent(Plan)` coincide con el subagent Plan
* `Agent(my-custom-agent)` coincide con un subagent personalizado llamado `my-custom-agent`

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

Las reglas `Cd` controlan a qué directorios el comando [`/cd`](/docs/es/commands) puede mover la sesión. `Cd` no es una herramienta invocable por el modelo: Claude no puede llamarla, y las reglas se aplican solo cuando ejecuta `/cd` usted mismo.

Una regla de negación `Cd` desnuda deshabilita `/cd` completamente. Una regla de negación `Cd(<path-pattern>)` bloquea objetivos coincidentes. Las reglas de negación verifican cada ortografía del objetivo, incluyendo cada salto de symlink que resuelve, por lo que una regla escrita para una ruta también bloquea objetivos que se resuelven a ella.

Agregar cualquier regla de permiso `Cd` cambia `/cd` al modo de lista de permitidos: el directorio objetivo resuelto debe coincidir con una de sus reglas de permiso, o `/cd` se niega. Sin reglas `Cd` configuradas, `/cd` mantiene su comportamiento predeterminado y le solicita confiar en un directorio desconocido.

Los patrones de ruta comparten los anclajes `//`, `~/` y `/` de [reglas Read y Edit](#read-and-edit), pero la coincidencia está anclada a la ruta del directorio completo en lugar de estilo gitignore. `*` coincide exactamente con un segmento de ruta y `**` coincide en segmentos. Un `/**` final también coincide con su raíz nombrada.

| Regla                 | Coincide                                                     | No coincide                   |
| --------------------- | ------------------------------------------------------------ | ----------------------------- |
| `Cd(~/code/*)`        | `~/code/app`                                                 | `~/code/app/src`, `~/code`    |
| `Cd(~/code/**)`       | `~/code` y cualquier directorio bajo él                      | directorios fuera de `~/code` |
| `Cd(**/node_modules)` | cualquier directorio `node_modules` en cualquier profundidad | `node_modules/pkg`            |

<h2 id="extend-permissions-with-hooks">
  Extender permisos con hooks
</h2>

Los [hooks de Claude Code](/docs/es/hooks-guide) proporcionan una forma de registrar comandos de shell personalizados para realizar evaluación de permisos en tiempo de ejecución. Cuando Claude Code realiza una llamada de herramienta, los hooks PreToolUse se ejecutan antes del aviso de permisos. La salida del hook puede denegar la llamada de herramienta, forzar un aviso u omitir el aviso para permitir que la llamada continúe.

Las decisiones del hook no omiten las reglas de permisos. Claude Code evalúa las reglas de negación y solicitud independientemente de lo que devuelva un hook PreToolUse: una regla de negación coincidente bloquea la llamada, y una regla de solicitud coincidente aún solicita incluso cuando el hook devolvió `"allow"` u `"ask"`. Esto preserva la precedencia de negación primero descrita en [Administrar permisos](#manage-permissions), incluyendo reglas de negación establecidas en configuración administrada.

Las herramientas de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools) y las herramientas MCP marcadas como [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool) también aún solicitan cuando un hook devuelve `"allow"`.

Un hook de bloqueo también tiene precedencia sobre las reglas de permiso. Un hook que sale con código 2 detiene la llamada de herramienta antes de que se evalúen las reglas de permisos, por lo que el bloqueo se aplica incluso cuando una regla de permiso permitiría que la llamada continúe. Para ejecutar todos los comandos Bash sin avisos excepto algunos que desea bloquear, agregue `"Bash"` a su lista de permiso y registre un hook PreToolUse que rechace esos comandos específicos. Consulte [Bloquear ediciones a archivos protegidos](/docs/es/hooks-guide#block-edits-to-protected-files) para un script de hook que puede adaptar.

<h2 id="working-directories">
  Directorios de trabajo
</h2>

Por defecto, Claude tiene acceso a archivos en el directorio donde fue lanzado. Puede extender este acceso:

* **Durante el inicio**: use el argumento CLI `--add-dir <path>`
* **Durante la sesión**: use el comando `/add-dir`
* **Configuración persistente**: agregue a `additionalDirectories` en [archivos de configuración](/docs/es/settings#settings-files)

Los archivos en directorios adicionales siguen las mismas reglas de permisos que el directorio de trabajo original: se vuelven legibles sin avisos, y los permisos de edición de archivos siguen el modo de permisos actual.

En sesiones en segundo plano en macOS, el host de la sesión solicita acceso a carpetas protegidas como `~/Desktop`, `~/Documents` y `~/Downloads` por separado de su terminal cuando Claude necesita leer o escribir archivos allí; si las lecturas allí fallan con `Operation not permitted`, consulte [cómo otorgar acceso a carpetas a sesiones en segundo plano](/docs/es/agent-view#background-sessions-can't-read-desktop-documents-or-downloads-on-macos).

Para cambiar el directorio de trabajo principal de la sesión en lugar de agregar otro, use [`/cd`](/docs/es/commands). El comando `/cd` requiere Claude Code v2.1.169 o posterior. A diferencia de `/add-dir`, reubica la sesión: se carga el `CLAUDE.md` del nuevo directorio y `--resume` encuentra la sesión desde allí.

<h3 id="additional-directories-grant-file-access-not-configuration">
  Los directorios adicionales otorgan acceso a archivos, no configuración
</h3>

Agregar un directorio extiende dónde Claude puede leer y editar archivos. No hace que ese directorio sea una raíz de configuración completa: la mayoría de la configuración `.claude/` no se descubre desde directorios adicionales, aunque algunos tipos se cargan como excepciones.

Estas excepciones se aplican solo a directorios agregados con la bandera `--add-dir` o el comando `/add-dir`. Los directorios listados en `permissions.additionalDirectories` en un archivo de configuración otorgan acceso a archivos solamente y no cargan ninguna de la configuración a continuación.

Los siguientes tipos de configuración se cargan desde directorios `--add-dir`:

| Configuración                                                                            | Cargado desde `--add-dir`                                                                                                                                                            |
| :--------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Skills](/docs/es/skills) en `.claude/skills/`                                                | Sí, con recarga en vivo                                                                                                                                                              |
| [Subagentes](/docs/es/sub-agents) en `.claude/agents/`                                        | Sí                                                                                                                                                                                   |
| [Configuración](/docs/es/settings) en `.claude/settings.json` y `.claude/settings.local.json` | Solo las claves `enabledPlugins` y `extraKnownMarketplaces`                                                                                                                          |
| Archivos [CLAUDE.md](/docs/es/memory), `.claude/rules/` y `CLAUDE.local.md`                   | Solo cuando `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1` está establecido. `CLAUDE.local.md` además requiere la fuente de configuración `local`, que está habilitada por defecto |

Los comandos y estilos de salida se descubren desde el directorio de trabajo actual y sus directorios padres, su directorio de usuario en `~/.claude/`, y configuración administrada. Los hooks y otras claves de `settings.json` se cargan desde la carpeta `.claude/` del directorio de trabajo actual sin recurso a directorios padres, junto con su `~/.claude/settings.json` de usuario y configuración administrada. Para compartir esa configuración entre proyectos, use uno de estos enfoques:

* **Configuración a nivel de usuario**: coloque archivos en `~/.claude/agents/`, `~/.claude/output-styles/`, o `~/.claude/settings.json` para hacerlos disponibles en cada proyecto
* **Plugins**: empaquete y distribuya configuración como un [plugin](/docs/es/plugins) que los equipos pueden instalar
* **Lanzar desde el directorio de configuración**: ejecute Claude Code desde el directorio que contiene la configuración `.claude/` que desea

<h2 id="how-permissions-interact-with-sandboxing">
  Cómo interactúan los permisos con el sandboxing
</h2>

Los permisos y el [sandboxing](/docs/es/sandboxing) son capas de seguridad complementarias:

* **Permisos** controlan qué herramientas puede usar Claude Code y qué archivos o dominios puede acceder. Se aplican a todas las herramientas, incluyendo Bash, Read, Edit, WebFetch, y MCP.
* **Sandboxing** proporciona aplicación a nivel del SO que restringe el acceso del sistema de archivos y red de la herramienta Bash. Se aplica solo a comandos Bash y sus procesos secundarios.

Use ambos para defensa en profundidad:

* Las reglas de negación de permisos bloquean que Claude intente acceder a recursos restringidos
* Las restricciones de sandbox previenen que comandos Bash alcancen recursos fuera de límites definidos, incluso si una inyección de solicitud omite la toma de decisiones de Claude
* Las restricciones del sistema de archivos en el sandbox combinan la configuración [`sandbox.filesystem`](/docs/es/sandboxing) con reglas de negación Read y Edit; ambas se fusionan en el límite final del sandbox
* Las restricciones de red combinan reglas de permisos WebFetch con las listas `allowedDomains` y `deniedDomains` del sandbox

Cuando el sandboxing está habilitado con `autoAllowBashIfSandboxed: true`, que es el valor predeterminado, los comandos Bash en sandbox se ejecutan sin solicitar incluso si sus permisos incluyen una regla ask `Bash` simple, o la [forma equivalente `Bash(*)`](#match-all-uses-of-a-tool): el límite del sandbox sustituye ese aviso de herramienta completa. Estas comprobaciones aún se aplican:

* Las reglas ask con alcance de contenido como `Bash(git push *)` aún fuerzan un aviso
* Las reglas de negación explícitas aún se aplican
* Los comandos `rm` o `rmdir` que apunten a `/`, su directorio de inicio u otras rutas críticas del sistema aún desencadenan un aviso

Los comandos que no se ejecutarán en sandbox, como comandos excluidos, respetan la regla ask `Bash` simple como es habitual. Consulte [modos de sandbox](/docs/es/sandboxing#sandbox-modes) para cambiar este comportamiento.

<h2 id="managed-settings">
  Configuración administrada
</h2>

Para organizaciones que necesitan control centralizado sobre la configuración de Claude Code, los administradores pueden implementar configuración administrada que no puede ser anulada por configuración de usuario o proyecto. Estas configuraciones de política siguen el mismo formato que archivos de configuración regulares y se pueden entregar a través de políticas MDM/a nivel del SO, archivos de configuración administrada, [configuración administrada por servidor](/docs/es/server-managed-settings), o una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) autohospedada. Consulte [archivos de configuración](/docs/es/settings#settings-files) para mecanismos de entrega y ubicaciones de archivos.

<h3 id="managed-only-settings">
  Configuración solo administrada
</h3>

Las siguientes configuraciones solo se leen desde configuración administrada. Colocarlas en archivos de configuración de usuario o proyecto no tiene efecto.

| Configuración                                  | Descripción                                                                                                                                                                                                                                                                                                                                               |
| :--------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowAllClaudeAiMcps`                         | Cuando es `true`, los conectores de claude.ai se cargan junto con un `managed-mcp.json` implementado en lugar de ser suprimidos por su control exclusivo. Consulte [Configuración MCP administrada](/docs/es/managed-mcp)                                                                                                                                      |
| `allowedChannelPlugins`                        | Lista de permitidos de plugins de canal que pueden enviar mensajes. Reemplaza la lista de permitidos predeterminada de Anthropic cuando se establece. Requiere `channelsEnabled: true`. Consulte [Restringir qué plugins de canal pueden ejecutarse](/docs/es/channels#restrict-which-channel-plugins-can-run)                                                 |
| `allowManagedHooksOnly`                        | Cuando es `true`, solo se cargan hooks administrados, hooks SDK y hooks de plugins forzados habilitados en la configuración administrada `enabledPlugins`. Los hooks de usuario, proyecto y todos los demás plugins están bloqueados                                                                                                                      |
| `allowManagedMcpServersOnly`                   | Cuando es `true`, solo se respetan `allowedMcpServers` de configuración administrada. `deniedMcpServers` aún se fusiona de todas las fuentes. Consulte [Configuración MCP administrada](/docs/es/managed-mcp)                                                                                                                                                  |
| `allowManagedPermissionRulesOnly`              | Cuando es `true`, evita que la configuración de usuario y proyecto defina reglas de permisos `allow`, `ask` o `deny`. Solo se aplican las reglas en configuración administrada. No afecta la lista de permitidos del servidor MCP; para eso, establezca `allowManagedMcpServersOnly`                                                                      |
| `blockedMarketplaces`                          | Lista de bloqueo de fuentes de marketplace. Las fuentes bloqueadas se verifican antes de descargar, por lo que nunca tocan el sistema de archivos. Consulte [restricciones de marketplace administradas](/docs/es/plugin-marketplaces#managed-marketplace-restrictions)                                                                                        |
| `channelsEnabled`                              | Permitir [channels](/docs/es/channels) para la organización. Consulte [controles empresariales](/docs/es/channels#enterprise-controls) para el valor predeterminado en cada plan                                                                                                                                                                                    |
| `disableSideloadFlags`                         | Rechazar los indicadores CLI `--plugin-dir`, `--plugin-url`, `--agents` y `--mcp-config` al inicio. Sin esto, los usuarios pueden eludir `strictKnownMarketplaces` para una única ejecución pasando estos indicadores. Consulte [`disableSideloadFlags`](/docs/es/settings#available-settings). Requiere Claude Code v2.1.193 o posterior                      |
| `forceRemoteSettingsRefresh`                   | Cuando es `true`, bloquea el inicio de CLI hasta que la configuración administrada remota se obtenga recientemente y sale si la obtención falla. Consulte [aplicación de cierre de falla](/docs/es/server-managed-settings#enforce-fail-closed-startup)                                                                                                        |
| `pluginTrustMessage`                           | Mensaje personalizado agregado a la advertencia de confianza de plugin mostrada antes de la instalación                                                                                                                                                                                                                                                   |
| `sandbox.filesystem.allowManagedReadPathsOnly` | Cuando es `true`, solo se respetan rutas `filesystem.allowRead` de configuración administrada. `denyRead` aún se fusiona de todas las fuentes                                                                                                                                                                                                             |
| `sandbox.network.allowManagedDomainsOnly`      | Cuando es `true`, solo se respetan `allowedDomains` y reglas de permiso `WebFetch(domain:...)` de configuración administrada. Los dominios no permitidos se bloquean automáticamente sin solicitar al usuario. Los dominios denegados aún se fusionan de todas las fuentes                                                                                |
| `strictKnownMarketplaces`                      | Controla qué marketplaces de plugins pueden agregar los usuarios e instalar plugins desde. Consulte [restricciones de marketplace administradas](/docs/es/plugin-marketplaces#managed-marketplace-restrictions)                                                                                                                                                |
| `strictPluginOnlyCustomization`                | Bloquea skills, agents, hooks y servidores MCP de fuentes de usuario y proyecto, por lo que solo pueden provenir de plugins o configuración administrada. `true` bloquea las cuatro superficies; una matriz como `["skills", "hooks"]` bloquea solo las nombradas. Consulte [`strictPluginOnlyCustomization`](/docs/es/settings#strictpluginonlycustomization) |
| `wslInheritsWindowsSettings`                   | Cuando es `true` en la clave del registro HKLM de Windows o `C:\Program Files\ClaudeCode\managed-settings.json`, WSL lee configuración administrada de la cadena de política de Windows además de `/etc/claude-code`. Consulte [Archivos de configuración](/docs/es/settings#settings-files)                                                                   |

`disableBypassPermissionsMode` generalmente se coloca en configuración administrada para aplicar la política organizacional, pero funciona desde cualquier alcance. Un usuario puede establecerlo en su propia configuración para bloquearse a sí mismo del modo de bypass.

<Note>
  En planes Team y Enterprise, un propietario habilita o deshabilita [Remote Control](/docs/es/remote-control) y [sesiones web](/docs/es/claude-code-on-the-web) en toda la organización en [configuración de administrador de Claude Code](https://claude.ai/admin-settings/claude-code). Remote Control puede deshabilitarse adicionalmente por dispositivo con la configuración [`disableRemoteControl`](/docs/es/settings#available-settings). Las sesiones web no tienen clave de configuración administrada por dispositivo.
</Note>

<h2 id="settings-precedence">
  Precedencia de configuración
</h2>

Las reglas de permisos siguen la misma [precedencia de configuración](/docs/es/settings#settings-precedence) que todas las demás configuraciones de Claude Code:

1. **Configuración administrada**: no puede ser anulada por ningún otro nivel, incluyendo argumentos de línea de comandos
2. **Argumentos de línea de comandos**: anulaciones de sesión temporal
3. **Configuración de proyecto local** (`.claude/settings.local.json`)
4. **Configuración de proyecto compartida** (`.claude/settings.json`)
5. **Configuración de usuario** (`~/.claude/settings.json`)

Si una herramienta se deniega en cualquier nivel, ningún otro nivel puede permitirla. Por ejemplo, una negación de configuración administrada no puede ser anulada por `--allowedTools`, y `--disallowedTools` puede agregar restricciones más allá de lo que define la configuración administrada.

Lo mismo se aplica en todos los ámbitos de configuración: si la configuración de usuario permite un permiso y la configuración de proyecto lo deniega, la regla de negación lo bloquea. Lo contrario también es cierto: una negación a nivel de usuario bloquea un permiso a nivel de proyecto, porque las reglas de negación de cualquier ámbito se evalúan antes que las reglas de permiso.

Los hosts de inserción pueden proporcionar política administrada adicional a través de la opción `managedSettings` del SDK cuando [`parentSettingsBehavior`](/docs/es/settings#settings-precedence) se establece en `"merge"`; los valores del integrador pueden restringir la política pero no flexibilizarla.

<h2 id="project-allow-rules-and-workspace-trust">
  Reglas de permiso del proyecto y confianza del espacio de trabajo
</h2>

Las reglas `permissions.allow` y las entradas `permissions.additionalDirectories` en el archivo `.claude/settings.json` de un proyecto otorgan capacidad, por lo que Claude Code las aplica solo después de que acepte el [diálogo de confianza del espacio de trabajo](/docs/es/security#additional-safeguards) para ese espacio de trabajo. Hasta entonces, Claude Code lee las reglas pero no las aplica. El diálogo de confianza enumera las reglas de permiso y los directorios adicionales que la carpeta otorgaría para que pueda revisarlos antes de aceptar. Las reglas `deny` y `ask` no se ven afectadas, ya que solo restringen.

Claude Code guarda la confianza por espacio de trabajo, identificada por la raíz del repositorio de git o, fuera de un repositorio, el directorio desde el que inició Claude Code. Cuando comienza en su directorio de inicio, la confianza se mantiene solo para la sesión actual y no se escribe en el disco; consulte la nota sobre [salvaguardas adicionales](/docs/es/security#additional-safeguards). Confiar en un directorio principal no aplica las reglas de permiso de un proyecto anidado.

`.claude/settings.local.json` es su propio archivo, por lo que la verificación de confianza del espacio de trabajo generalmente no se aplica a él. Cuando un repositorio podría haber proporcionado el archivo, como cuando se confirma en git o `.claude` es un enlace simbólico, sus reglas de permiso y directorios adicionales se someten a la verificación de confianza como la configuración del proyecto.

Claude Code ejecuta git para verificar si el repositorio proporcionó el archivo, y ejecuta esa verificación solo en una carpeta cubierta por un diálogo de confianza aceptado, para esa carpeta o para uno de sus directorios principales. En una sesión interactiva en una carpeta que aún no ha confiado, las reglas de permiso y los directorios adicionales en `.claude/settings.local.json` se someten a la verificación de confianza como la configuración del proyecto hasta que acepte el diálogo, a menos que la sesión se ejecute en su directorio de configuración personal como se describe a continuación. De las dos excepciones a continuación, solo la excepción del directorio de configuración se aplica antes del diálogo, porque no necesita ejecutar git. Determinar que un directorio no está dentro de un repositorio de git utiliza la misma verificación de git, por lo que la excepción de no estar dentro de un repositorio entra en vigor una vez que se acepta un diálogo de confianza que cubre la carpeta. Antes de v2.1.207, un `.claude/settings.local.json` sin seguimiento aplicaba sus reglas de permiso en esa carpeta antes de que aceptara el diálogo.

Las reglas de permiso y los directorios adicionales en `.claude/settings.local.json` también se aplican sin confianza del espacio de trabajo en dos casos:

* El directorio desde el que inició Claude Code no está dentro de un repositorio de git.
* La sesión se ejecuta en su directorio de configuración personal: su directorio de inicio o cualquier directorio cuyo subdirectorio `.claude` haya establecido como [`CLAUDE_CONFIG_DIR`](/docs/es/env-vars).

En ambos casos, el archivo es uno que creó en lugar de uno que un repositorio podría haber proporcionado, y un `.claude/settings.local.json` confirmado en el repositorio aún requiere confianza del espacio de trabajo. Las versiones 2.1.196 a 2.1.199 trataban el archivo como proporcionado por el repositorio en esos espacios de trabajo, ignoraban sus reglas de permiso e imprimían una advertencia [`this workspace has not been trusted`](/docs/es/errors#workspace-has-not-been-trusted) en stderr. Las dos excepciones anteriores coinciden con v2.1.195 y versiones anteriores y se restauraron en v2.1.200.

También a partir de v2.1.200, un espacio de trabajo cuyas reglas de permiso o directorios adicionales aún no se aplican, pero que nunca mostró el diálogo de confianza porque un directorio principal ya era de confianza, muestra el diálogo la próxima vez que inicia Claude Code allí de forma interactiva. El diálogo ofrece dos opciones:

* **Yes, I trust this folder**: guarda la confianza para ese espacio de trabajo y aplica las reglas en la misma sesión.
* **No, continue without these permissions**: continúa funcionando con esas reglas ignoradas. El diálogo aparece nuevamente en la próxima sesión.

En [modo no interactivo](/docs/es/headless) con `-p`, no aparece ningún diálogo y las reglas permanecen ignoradas.

<h2 id="example-configurations">
  Configuraciones de ejemplo
</h2>

Este [repositorio](https://github.com/anthropics/claude-code/tree/main/examples/settings) incluye configuraciones de configuración inicial para escenarios de implementación comunes. Use estos como puntos de partida y ajústelos para que se adapten a sus necesidades.

<h2 id="see-also">
  Ver también
</h2>

* [Settings](/docs/es/settings): referencia de configuración completa incluyendo la tabla de configuración de permisos
* [Configure auto mode](/docs/es/auto-mode-config): indique al clasificador del modo auto qué infraestructura confía su organización
* [Sandboxing](/docs/es/sandboxing): aislamiento del sistema de archivos y red a nivel del SO para comandos Bash
* [Authentication](/docs/es/authentication): configure el acceso de usuario a Claude Code
* [Security](/docs/es/security): salvaguardas de seguridad y mejores prácticas
* [Hooks](/docs/es/hooks-guide): automatice flujos de trabajo y extienda la evaluación de permisos
