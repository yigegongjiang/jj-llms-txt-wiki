> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Personaliza tu línea de estado

> Configura una barra de estado personalizada para monitorear el uso de la ventana de contexto, costos y estado de git en Claude Code

La línea de estado es una barra personalizable en la parte inferior de Claude Code que ejecuta cualquier script de shell que configure. Recibe datos de sesión JSON en stdin y muestra lo que su script imprime, dándole una vista persistente y de un vistazo del uso de contexto, costos, estado de git, o cualquier otra cosa que desee rastrear.

Las líneas de estado son útiles cuando:

* Desea monitorear el uso de la ventana de contexto mientras trabaja
* Necesita rastrear los costos de la sesión
* Trabaja en múltiples sesiones y necesita distinguirlas
* Desea que la rama de git y el estado siempre sean visibles

La línea de estado se renderiza en su propia fila por encima de las insignias de pie de página integradas y no las reemplaza. Con una línea de estado personalizada configurada, Claude Code deja de mostrar la mayoría de las sugerencias de teclado del pie de página, incluyendo `esc para interrumpir`, la alternativa `? para atajos de teclado`, y la sugerencia de `mantener espacio para hablar` [dictado por voz](/docs/es/voice-dictation). Para agregar insignias de enlaces interactivos al pie de página cuando aparece una ID en la conversación, sin escribir un script, configure [`footerLinksRegexes`](/docs/es/settings-reference#footerlinksregexes) en su lugar.

Aquí hay un ejemplo de una [línea de estado de múltiples líneas](#display-multiple-lines) que muestra información de git en la primera línea y una barra de contexto codificada por colores en la segunda.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-multiline.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=60f11387658acc9ff75158ae85f2ac87" alt="Una línea de estado de múltiples líneas que muestra el nombre del modelo, directorio, rama de git en la primera línea, y una barra de progreso de uso de contexto con costo y duración en la segunda línea" width="776" height="212" data-path="images/statusline-multiline.png" />
</Frame>

Esta página le guía a través de [configurar una línea de estado básica](#set-up-a-status-line), explica [cómo fluyen los datos](#how-status-lines-work) desde Claude Code a su script, enumera [todos los campos que puede mostrar](#available-data), y proporciona [ejemplos listos para usar](#examples) para patrones comunes como estado de git, seguimiento de costos y barras de progreso.

<h2 id="set-up-a-status-line">
  Configurar una línea de estado
</h2>

Usa el [comando `/statusline`](#use-the-%2Fstatusline-command) para que Claude Code genere un script para ti, o [crea manualmente un script](#manually-configure-a-status-line) y agrégalo a tu configuración.

<h3 id="use-the-/statusline-command">
  Usar el comando /statusline
</h3>

El comando `/statusline` acepta instrucciones en lenguaje natural que describen lo que deseas mostrar. Claude Code genera un archivo de script en `~/.claude/` y actualiza tu configuración automáticamente:

```text theme={null}
/statusline show model name and context percentage with a progress bar
```

Aprueba los mensajes de edición de archivo si Claude Code solicita permiso durante la configuración.

<h3 id="manually-configure-a-status-line">
  Configurar manualmente una línea de estado
</h3>

Agrega un campo `statusLine` a tu configuración de usuario (`~/.claude/settings.json`, donde `~` es tu directorio de inicio) o [configuración del proyecto](/docs/es/settings#where-settings-live). Establece `type` en `"command"` y apunta `command` a una ruta de script o un comando de shell en línea. Para un tutorial completo sobre cómo crear un script, consulta [Construir una línea de estado paso a paso](#build-a-status-line-step-by-step).

```json theme={null}
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/statusline.sh",
    "padding": 2
  }
}
```

El campo `command` se ejecuta en un shell, por lo que también puedes usar comandos en línea en lugar de un archivo de script. Este ejemplo usa `jq` para analizar la entrada JSON y mostrar el nombre del modelo y el porcentaje de contexto:

```json theme={null}
{
  "statusLine": {
    "type": "command",
    "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'"
  }
}
```

El campo `padding` opcional agrega espaciado horizontal adicional (en caracteres) al contenido de la línea de estado. Por defecto es `0`. Este relleno se suma al espaciado integrado de la interfaz, por lo que controla la indentación relativa en lugar de la distancia absoluta desde el borde de la terminal.

El campo `refreshInterval` opcional vuelve a ejecutar tu comando cada N segundos además de las [actualizaciones impulsadas por eventos](#how-status-lines-work). El mínimo es `1`. Establece esto cuando tu línea de estado muestra datos basados en tiempo, como un reloj, o cuando los subagentes de fondo cambian el estado de git mientras la sesión principal está inactiva. Déjalo sin establecer para ejecutar solo en eventos.

El campo `hideVimModeIndicator` opcional suprime el texto integrado `-- INSERT --` debajo del prompt. Establece esto en `true` cuando tu script renderiza [`vim.mode`](#available-data) por sí mismo, para que el modo no se muestre dos veces.

<h3 id="disable-the-status-line">
  Desactivar la línea de estado
</h3>

Ejecuta `/statusline` y pídele que elimine o borre tu línea de estado (por ejemplo, `/statusline delete`, `/statusline clear`, `/statusline remove it`). También puedes eliminar manualmente el campo `statusLine` de tu settings.json.

<h2 id="build-a-status-line-step-by-step">
  Construir una línea de estado paso a paso
</h2>

Este tutorial muestra lo que `/statusline` configura para ti creando manualmente una línea de estado que muestra el modelo actual, el directorio de trabajo y el porcentaje de uso de la ventana de contexto.

<Note>Ejecutar [`/statusline`](#use-the-%2Fstatusline-command) con una descripción de lo que deseas configura todo esto automáticamente para ti.</Note>

Estos ejemplos usan scripts de Bash, que funcionan en macOS y Linux. En Windows, consulta [Configuración de Windows](#windows-configuration) para ejemplos de PowerShell y Git Bash.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-quickstart.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=696445e59ca0059213250651ad23db6b" alt="Una línea de estado que muestra el nombre del modelo, directorio y porcentaje de contexto" width="726" height="164" data-path="images/statusline-quickstart.png" />
</Frame>

<Steps>
  <Step title="Crear un script que lea JSON e imprima salida">
    Claude Code envía datos JSON a tu script a través de stdin. Este script usa [`jq`](https://jqlang.org/), un analizador JSON de línea de comandos que es posible que necesites instalar, para extraer el nombre del modelo, el directorio y el porcentaje de contexto, luego imprime una línea formateada.

    Guarda esto en `~/.claude/statusline.sh` (donde `~` es tu directorio de inicio, como `/Users/username` en macOS o `/home/username` en Linux):

    ```bash theme={null}
    #!/bin/bash
    # Read JSON data that Claude Code sends to stdin
    input=$(cat)

    # Extract fields using jq
    MODEL=$(echo "$input" | jq -r '.model.display_name')
    DIR=$(echo "$input" | jq -r '.workspace.current_dir')
    # The "// 0" provides a fallback if the field is null
    PCT=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)

    # Output the status line - ${DIR##*/} extracts just the folder name
    echo "[$MODEL] 📁 ${DIR##*/} | ${PCT}% context"
    ```
  </Step>

  <Step title="Hacerlo ejecutable">
    Marca el script como ejecutable para que tu shell pueda ejecutarlo:

    ```bash theme={null}
    chmod +x ~/.claude/statusline.sh
    ```
  </Step>

  <Step title="Agregar a la configuración">
    Dile a Claude Code que ejecute tu script como la línea de estado. Agrega esta configuración a `~/.claude/settings.json`, que establece `type` en `"command"` (lo que significa "ejecutar este comando de shell") y apunta `command` a tu script:

    ```json theme={null}
    {
      "statusLine": {
        "type": "command",
        "command": "~/.claude/statusline.sh"
      }
    }
    ```

    Tu línea de estado aparece en la parte inferior de la interfaz. Claude Code recarga la configuración automáticamente y ejecuta tu script tan pronto como guardes el archivo.
  </Step>
</Steps>

<h2 id="how-status-lines-work">
  Cómo funcionan las líneas de estado
</h2>

Claude Code ejecuta tu script con [datos de sesión JSON](#available-data) en stdin y muestra lo que el script imprime en stdout.

**Cuándo se actualiza**

Tu script se ejecuta una vez cuando comienza una sesión, incluyendo cuando reanudas una. Después de eso, se ejecuta nuevamente cuando:

* Llega un nuevo mensaje del asistente
* `/compact` finaliza
* El modo de permiso cambia
* El modo Vim se activa o desactiva
* Cambias el `command` en tu configuración de `statusLine`
* Un temporizador [`refreshInterval`](#manually-configure-a-status-line) se agota, si estableces uno
* Una ventana de [límite de velocidad](#rate-limit-usage) en los datos que tu script recibió por última vez alcanza su tiempo `resets_at`
* Un [caché de prompt](#prompt-cache-fields) cálido en los datos que tu script recibió por última vez alcanza su tiempo `expires_at`

Claude Code debounce las actualizaciones en 300ms, por lo que los cambios rápidos se agrupan y tu script se ejecuta una vez después de que los cambios se detienen. Un cambio en el `command` mismo omite el debounce: Claude Code ejecuta el nuevo comando de inmediato. Si una nueva actualización se activa mientras tu script aún se está ejecutando, Claude Code cancela el script en vuelo. Si editas tu script, los cambios aparecen la próxima vez que un disparador de actualización lo vuelve a ejecutar.

Los disparadores impulsados por eventos pueden quedarse en silencio cuando la sesión principal está inactiva, por ejemplo mientras un coordinador espera en subagentes de fondo. Para mantener segmentos basados en tiempo o de fuentes externas actuales durante períodos inactivos, establece [`refreshInterval`](#manually-configure-a-status-line) para también volver a ejecutar el comando en un temporizador fijo.

**Lo que tu script puede generar**

* **Múltiples líneas**: cada declaración `echo` o `print` se muestra como una fila separada. Consulta el [ejemplo de múltiples líneas](#display-multiple-lines).
* **Colores**: usa [códigos de escape ANSI](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors) como `\033[32m` para verde (la terminal debe admitirlos). Consulta el [ejemplo de estado de git](#git-status-with-colors).
* **Enlaces**: usa [secuencias de escape OSC 8](https://en.wikipedia.org/wiki/ANSI_escape_code#OSC) para hacer que el texto sea clickeable (Cmd+clic en macOS, Ctrl+clic en Windows/Linux). Requiere una terminal que admita hipervínculos como iTerm2, Kitty o WezTerm. Consulta el [ejemplo de enlaces clickeables](#clickable-links).

**Ajustar el tamaño de la salida a la terminal**

Claude Code captura la salida de tu script en lugar de conectarla directamente a la terminal, por lo que `tput cols` y la detección de ancho a nivel de lenguaje no pueden leer el tamaño de la terminal desde dentro del script. Lee las variables de entorno `COLUMNS` y `LINES` en su lugar. Claude Code establece estas variables con las dimensiones actuales de la terminal antes de ejecutar tu script.

<Note>La línea de estado se ejecuta localmente y no consume tokens de API. Se oculta temporalmente durante ciertas interacciones de la interfaz, incluidas sugerencias de autocompletado, el menú de ayuda y solicitudes de permiso.</Note>

<h2 id="available-data">
  Datos disponibles
</h2>

Claude Code envía los siguientes campos JSON a tu script a través de stdin:

| Campo                                                                            | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| -------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `model.id`, `model.display_name`                                                 | Identificador del modelo actual y nombre para mostrar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `cwd`, `workspace.current_dir`                                                   | Directorio de trabajo actual. Ambos campos contienen el mismo valor; `workspace.current_dir` es preferido para consistencia con `workspace.project_dir`.                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `workspace.project_dir`                                                          | Directorio donde se lanzó Claude Code, que puede diferir de `cwd` si el directorio de trabajo cambia durante una sesión                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `workspace.added_dirs`                                                           | Directorios adicionales agregados a través de `/add-dir` o `--add-dir`. Array vacío si no se ha agregado ninguno                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `workspace.git_worktree`                                                         | Nombre de git worktree cuando el directorio actual está dentro de un worktree vinculado creado con `git worktree add`. Ausente en el árbol de trabajo principal. Poblado para cualquier git worktree, a diferencia de `worktree.*`, que está presente solo mientras la sesión está en una [sesión de worktree](/docs/es/worktrees)                                                                                                                                                                                                                                                                    |
| `workspace.repo.host`, `workspace.repo.owner`, `workspace.repo.name`             | Identidad del repositorio analizada desde el remoto `origin`, por ejemplo, `"github.com"`, `"anthropics"`, `"claude-code"`. Ausente fuera de un repositorio git o cuando no hay un remoto `origin` configurado. Para un proyecto de gitlab.com anidado en subgrupos, `owner` es la ruta de espacio de nombres completa con barras, como `"group/subgroup"`. Antes de v2.1.260, `workspace.repo` estaba ausente para estos proyectos                                                                                                                                                              |
| `cost.total_cost_usd`                                                            | Costo total estimado de la sesión en USD, calculado del lado del cliente al precio de lista a menos que una tabla [`modelPricing`](/docs/es/settings-reference#modelpricing) esté en vigor. Puede diferir de tu factura real. Se reinicia a \$0 cuando `/clear` inicia una nueva sesión. Antes de v2.1.211, el total se mantenía después de `/clear`                                                                                                                                                                                                                                                  |
| `cost.total_duration_ms`                                                         | Tiempo total transcurrido desde que comenzó la sesión, en milisegundos. Se acumula entre reanudaciones y no incluye el tiempo mientras la sesión no está en ejecución                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `cost.total_api_duration_ms`                                                     | Tiempo total dedicado a esperar respuestas de API en milisegundos                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `cost.total_lines_added`, `cost.total_lines_removed`                             | Líneas de código cambiadas                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `context_window.total_input_tokens`, `context_window.total_output_tokens`        | Conteos de tokens actualmente en la ventana de contexto, de la respuesta de API más reciente. La entrada incluye lecturas y escrituras de caché                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `context_window.context_window_size`                                             | Tamaño máximo de la ventana de contexto en tokens. 200000 por defecto, o 1000000 para modelos con contexto extendido.                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `context_window.used_percentage`                                                 | Porcentaje precalculado de ventana de contexto utilizada                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `context_window.remaining_percentage`                                            | Porcentaje precalculado de ventana de contexto restante                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `context_window.current_usage`                                                   | Conteos de tokens de la última llamada a API, descritos en [campos de ventana de contexto](#context-window-fields)                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `exceeds_200k_tokens`                                                            | Si el conteo total de tokens (tokens de entrada, caché y salida combinados) de la respuesta de API más reciente excede 200k. Este es un umbral fijo independientemente del tamaño real de la ventana de contexto.                                                                                                                                                                                                                                                                                                                                                                                |
| `fast_mode`                                                                      | Si [fast mode](/docs/es/fast-mode) está habilitado para la sesión                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `effort.level`                                                                   | Nivel de esfuerzo de razonamiento actual (`low`, `medium`, `high`, `xhigh`, o `max`). Refleja el valor de sesión en vivo, incluidos cambios de `/effort` a mitad de sesión. Ultracode no es un nivel distinto y se reporta como `xhigh`. Ausente cuando el modelo actual no admite el parámetro de esfuerzo                                                                                                                                                                                                                                                                                      |
| `thinking.enabled`                                                               | Si el pensamiento extendido está habilitado para la sesión                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `rate_limits.five_hour.used_percentage`, `rate_limits.seven_day.used_percentage` | Porcentaje del límite de velocidad de 5 horas o 7 días consumido, de 0 a 100                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `rate_limits.five_hour.resets_at`, `rate_limits.seven_day.resets_at`             | Segundos de época Unix cuando se reinicia la ventana de límite de velocidad de 5 horas o 7 días                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `rate_limits.spend_limit.used_percentage`, `rate_limits.spend_limit.resets_at`   | Detrás de una [puerta de aplicaciones Claude](/docs/es/claude-apps-gateway-spend-limits#usage-warnings-in-claude-code), el porcentaje utilizado del límite de gasto que se aplica a ti, y los segundos de época Unix cuando se reinicia su período. El porcentaje va de 0 a 100, o por encima de 100 una vez que excedas el límite. Requiere Claude Code v2.1.251 o posterior                                                                                                                                                                                                                         |
| `prompt_cache`                                                                   | Las estadísticas de [prompt cache](/docs/es/prompt-caching) de la sesión para la conversación principal: relación de aciertos, fallos, y si el caché está caliente. Consulta [campos de prompt cache](#prompt-cache-fields) para cada campo. Ausente hasta la primera respuesta de API de la conversación principal. Requiere Claude Code v2.1.251 o posterior                                                                                                                                                                                                                                        |
| `session_id`                                                                     | Identificador único de sesión                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `session_name`                                                                   | Nombre de sesión. Utiliza el nombre personalizado establecido con la bandera `--name` o `/rename` cuando existe uno, de lo contrario el título de sesión generado por IA. El [nombre para mostrar predeterminado](/docs/es/sessions#name-your-sessions), como `my-app-3f`, no completa este campo. Ausente cuando la sesión no tiene ni un nombre personalizado ni un título generado por IA                                                                                                                                                                                                          |
| `prompt_id`                                                                      | UUID que identifica el prompt del usuario que se está procesando actualmente. Coincide con el atributo [`prompt.id` en eventos de OpenTelemetry](/docs/es/monitoring-usage#event-correlation-attributes). Ausente hasta la primera entrada del usuario. Requiere Claude Code v2.1.196 o posterior                                                                                                                                                                                                                                                                                                     |
| `transcript_path`                                                                | Ruta al archivo de transcripción de conversación                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `version`                                                                        | Versión de Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `output_style.name`                                                              | Nombre del estilo de salida actual                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `vim.mode`                                                                       | Modo vim actual (`NORMAL`, `INSERT`, `VISUAL`, o `VISUAL LINE`) cuando [el modo vim](/docs/es/interactive-mode#vim-editor-mode) está habilitado                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `agent.name`                                                                     | Nombre del agente cuando se ejecuta con la bandera `--agent` o configuración de agente configurada                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `pr.number`, `pr.url`                                                            | Solicitud de extracción abierta para la rama actual. Refleja la insignia de PR en la barra de estado inferior. En un repositorio con un remoto de GitLab, Claude Code completa estos campos desde la [solicitud de fusión](/docs/es/interactive-mode#gitlab-merge-requests) abierta de la rama, por lo que `pr.number` es el número de solicitud de fusión. Los datos de solicitud de fusión requieren Claude Code v2.1.234 o posterior. Ausente cuando no está en un repositorio git, hasta que se encuentre una solicitud de extracción o solicitud de fusión, o una vez que se fusiona o se cierra |
| `pr.review_state`                                                                | Estado de revisión de la PR abierta: `approved`, `pending`, `changes_requested`, o `draft`. Puede estar independientemente ausente incluso cuando `pr` está presente                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `pr.kind`                                                                        | `mr` cuando `pr` describe una [solicitud de fusión de GitLab](/docs/es/interactive-mode#gitlab-merge-requests). Ausente para solicitudes de extracción de GitHub, por lo que los scripts escritos antes de este campo siguen funcionando. Para una solicitud de fusión, Claude Code establece `review_state` en `approved` cuando GitLab informa que es fusionable, `pending` para cualquier otro estado abierto, y `draft` para un borrador. Requiere Claude Code v2.1.234 o posterior                                                                                                               |
| `worktree.name`                                                                  | Nombre del worktree activo. Presente solo mientras la sesión está en una [sesión de worktree](/docs/es/worktrees)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `worktree.path`                                                                  | Ruta absoluta al directorio del worktree                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `worktree.branch`                                                                | Nombre de rama de Git para el worktree (por ejemplo, `"worktree-my-feature"`). Ausente para worktrees basados en hooks                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `worktree.original_cwd`                                                          | El directorio en el que estaba Claude antes de entrar en el worktree                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `worktree.original_branch`                                                       | Rama de Git extraída antes de entrar en el worktree. Ausente para worktrees basados en hooks                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |

<Accordion title="Esquema JSON completo">
  Tu comando de línea de estado recibe esta estructura JSON a través de stdin:

  ```json theme={null}
  {
    "cwd": "/current/working/directory",
    "session_id": "abc123...",
    "session_name": "my-session",
    "prompt_id": "550e8400-e29b-41d4-a716-446655440000",
    "transcript_path": "/path/to/transcript.jsonl",
    "model": {
      "id": "claude-opus-5-5",
      "display_name": "Opus"
    },
    "workspace": {
      "current_dir": "/current/working/directory",
      "project_dir": "/original/project/directory",
      "added_dirs": [],
      "git_worktree": "feature-xyz",
      "repo": {
        "host": "github.com",
        "owner": "anthropics",
        "name": "claude-code"
      }
    },
    "version": "2.1.90",
    "output_style": {
      "name": "default"
    },
    "cost": {
      "total_cost_usd": 0.01234,
      "total_duration_ms": 45000,
      "total_api_duration_ms": 2300,
      "total_lines_added": 156,
      "total_lines_removed": 23
    },
    "context_window": {
      "total_input_tokens": 15500,
      "total_output_tokens": 1200,
      "context_window_size": 200000,
      "used_percentage": 8,
      "remaining_percentage": 92,
      "current_usage": {
        "input_tokens": 8500,
        "output_tokens": 1200,
        "cache_creation_input_tokens": 5000,
        "cache_read_input_tokens": 2000
      }
    },
    "exceeds_200k_tokens": false,
    "prompt_cache": {
      "warm": true,
      "caching_observed": true,
      "ttl": "1h",
      "expires_at": 1738429200,
      "requests": 14,
      "misses": 2,
      "expected_rebuilds": 1,
      "hit_ratio": 0.91,
      "cache_write_tokens": 352000,
      "miss_recache_tokens": 310200,
      "last_miss_at": 1738425230,
      "last_miss_cause": {
        "causes": ["tools_changed"],
        "tools_added": 2,
        "tools_removed": 0
      },
      "miss_causes": {
        "tools_changed": 2
      },
      "recache_tokens_if_cold": 45000
    },
    "fast_mode": false,
    "effort": {
      "level": "high"
    },
    "thinking": {
      "enabled": true
    },
    "rate_limits": {
      "five_hour": {
        "used_percentage": 23.5,
        "resets_at": 1738425600
      },
      "seven_day": {
        "used_percentage": 41.2,
        "resets_at": 1738857600
      },
      "spend_limit": {
        "used_percentage": 62.8,
        "resets_at": 1740787200
      }
    },
    "vim": {
      "mode": "NORMAL"
    },
    "agent": {
      "name": "security-reviewer"
    },
    "pr": {
      "number": 1234,
      "url": "https://github.com/anthropics/claude-code/pull/1234",
      "review_state": "pending"
    },
    "worktree": {
      "name": "my-feature",
      "path": "/path/to/.claude/worktrees/my-feature",
      "branch": "worktree-my-feature",
      "original_cwd": "/path/to/project",
      "original_branch": "main"
    }
  }
  ```

  **Campos que pueden estar ausentes** (no presentes en JSON):

  * `session_name`: aparece cuando se ha establecido un nombre personalizado con `--name` o `/rename`, o una vez que existe un título de sesión generado por IA. El nombre para mostrar predeterminado, como `my-app-3f`, no lo completa
  * `prompt_id`: aparece solo después de la primera entrada del usuario
  * `workspace.git_worktree`: aparece solo cuando el directorio actual está dentro de un git worktree vinculado
  * `workspace.repo`: aparece solo dentro de un repositorio git con un remoto `origin` configurado
  * `effort`: aparece solo cuando el modelo actual admite el parámetro de esfuerzo de razonamiento
  * `vim`: aparece solo cuando el modo vim está habilitado
  * `agent`: aparece solo cuando se ejecuta con la bandera `--agent` o configuración de agente configurada
  * `pr`: aparece solo mientras se encuentra una PR abierta o una solicitud de fusión de GitLab para la rama actual, y se elimina una vez que se fusiona o se cierra. `pr.review_state` y `pr.kind` pueden estar independientemente ausentes
  * `worktree`: aparece solo mientras la sesión está en una [sesión de worktree](/docs/es/worktrees). Cuando está presente, `branch` y `original_branch` también pueden estar ausentes para worktrees basados en hooks
  * `rate_limits`: aparece solo para suscriptores de Claude.ai Pro y Max, o detrás de una puerta de aplicaciones Claude que establece un límite de gasto para ti, y solo después de la primera respuesta de API en la sesión. Cada ventana (`five_hour`, `seven_day`, `spend_limit`) puede estar independientemente ausente, y Claude Code elimina una ventana una vez que pasa su tiempo `resets_at`. Usa `jq -r '.rate_limits.five_hour.used_percentage // empty'` para manejar la ausencia con elegancia.
  * `prompt_cache`: aparece después de la primera respuesta de API de la conversación principal. Consulta [campos de prompt cache](#prompt-cache-fields)

  **Campos que pueden ser `null`**:

  * `context_window.current_usage`: `null` antes de la primera llamada a API en una sesión, y nuevamente después de `/compact` hasta que la siguiente llamada a API lo repuebla
  * `context_window.used_percentage`, `context_window.remaining_percentage`: pueden ser `null` al principio de la sesión

  Maneja campos faltantes con acceso condicional y valores nulos con valores predeterminados de respaldo en tus scripts.
</Accordion>

<h3 id="context-window-fields">
  Campos de ventana de contexto
</h3>

El objeto `context_window` describe la ventana de contexto en vivo de la respuesta de API más reciente.

* **Totales combinados** (`total_input_tokens`, `total_output_tokens`): tokens actualmente en la ventana de contexto. `total_input_tokens` es la suma de `input_tokens`, `cache_creation_input_tokens`, y `cache_read_input_tokens`; `total_output_tokens` son los tokens de salida de la respuesta más reciente. Ambos son `0` antes de la primera respuesta de API.
* **Uso por componente** (`current_usage`): los mismos conteos de tokens desglosados por categoría. Usa esto cuando necesites separar los aciertos de caché de la entrada fresca.

El objeto `current_usage` contiene:

* `input_tokens`: tokens de entrada en contexto actual
* `output_tokens`: tokens de salida generados
* `cache_creation_input_tokens`: tokens escritos en caché
* `cache_read_input_tokens`: tokens leídos del caché

Para saber qué significan los campos de caché y cómo se facturan, consulta [verificar rendimiento de caché](/docs/es/prompt-caching#check-cache-performance).

El campo `used_percentage` se calcula solo a partir de tokens de entrada: `input_tokens + cache_creation_input_tokens + cache_read_input_tokens`. No incluye `output_tokens`.

Si calculas el porcentaje de contexto manualmente desde `current_usage`, usa la misma fórmula de solo entrada para coincidir con `used_percentage`.

El objeto `current_usage` es `null` antes de la primera llamada a API en una sesión, y nuevamente inmediatamente después de `/compact` hasta que la siguiente llamada a API lo repuebla.

<h3 id="prompt-cache-fields">
  Campos de prompt cache
</h3>

El objeto `prompt_cache` resume cómo la conversación principal de la sesión está utilizando el [prompt cache](/docs/es/prompt-caching). Claude Code lo calcula a partir de los conteos de tokens de caché en las respuestas de la API, por lo que funciona en cada proveedor.

El objeto aparece después de la primera respuesta de API de la conversación principal. Claude Code no cuenta las solicitudes de subagente en estas estadísticas. Requiere Claude Code v2.1.251 o posterior.

La tabla enumera cada campo con su significado. Las marcas de tiempo son segundos de época Unix, la misma unidad que `rate_limits.*.resets_at`. Una línea de estado corta generalmente muestra uno o dos de estos; `warm` y `hit_ratio` resumen el estado del caché más directamente.

| Campo                    | Descripción                                                                                                                                                                                                                                                                              |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `warm`                   | Si el prefijo en caché aún está dentro de su TTL. `false` cuando la última respuesta no reportó tokens de caché, incluso mientras `caching_observed` es `true`                                                                                                                           |
| `caching_observed`       | Si alguna respuesta esta sesión reportó tokens de caché. `false` significa que el prompt caching está desactivado, o tu proveedor o puerta de enlace no lo reporta                                                                                                                       |
| `ttl`                    | [Vida útil del caché](/docs/es/prompt-caching#cache-lifetime) del prefijo en caché actual: `"5m"` o `"1h"`                                                                                                                                                                                    |
| `expires_at`             | Cuándo el prefijo en caché sale de su TTL y se vuelve frío, en segundos de época. `null` cuando la última respuesta no reportó tokens de caché                                                                                                                                           |
| `requests`               | Solicitudes de API registradas para la conversación principal esta sesión                                                                                                                                                                                                                |
| `misses`                 | Solicitudes que reprocesaron contenido que el caché ya tenía: más del 5% y al menos 2,000 tokens de lo que la solicitud podría haber leído del caché, sin compactación o limpieza de resultados de herramientas para explicar el déficit en lecturas de caché                            |
| `expected_rebuilds`      | Reconstrucciones de caché que siguieron a una compactación o una limpieza de resultados de herramientas antiguas                                                                                                                                                                         |
| `hit_ratio`              | Tokens de lectura de caché como una fracción de todos los tokens de entrada esta sesión, de 0 a 1. El denominador cuenta lecturas de caché, escrituras de caché, e entrada sin caché. `null` mientras esos conteos sean todos cero                                                       |
| `cache_write_tokens`     | Todos los tokens escritos en el caché esta sesión, la escritura inicial de la primera solicitud incluida                                                                                                                                                                                 |
| `miss_recache_tokens`    | Tokens escritos en el caché por las solicitudes contadas como fallos                                                                                                                                                                                                                     |
| `last_miss_at`           | Cuándo ocurrió el último fallo, en segundos de época. `null` mientras la sesión no tenga fallos                                                                                                                                                                                          |
| `last_miss_cause`        | Lo que Claude Code identificó como la causa probable del último fallo, descrito bajo [Causa del último fallo](#last-miss-cause). Requiere Claude Code v2.1.260 o posterior                                                                                                               |
| `miss_causes`            | Cuántos de los fallos diagnosticados de esta sesión tuvieron cada causa, indexados por los mismos nombres de causa que `last_miss_cause`. Requiere Claude Code v2.1.260 o posterior                                                                                                      |
| `recache_tokens_if_cold` | Tokens que la siguiente solicitud vuelve a almacenar en caché si el caché se ha enfriado para entonces. `null` justo después de una compactación o una limpieza de resultados de herramientas antiguas, hasta que la siguiente solicitud registre el tamaño de la conversación reescrita |

Claude Code muestra las mismas estadísticas en la terminal, en la línea `Prompt cache (main)` del comando [`/usage`](/docs/es/costs#prompt-cache-statistics).

<h4 id="last-miss-cause">
  Causa del último fallo
</h4>

El objeto `last_miss_cause` reporta lo que Claude Code identificó como la causa probable del fallo más reciente. Su array `causes` contiene uno o más nombres de causa, como `tools_changed`, `system_prompt_changed`, `ttl_expired_5m`, o `likely_server_side`. El objeto es `null` hasta el primer fallo de la sesión, y nuevamente siempre que Claude Code no pueda identificar una causa para el fallo más reciente. Requiere Claude Code v2.1.260 o posterior.

Dos causas agregan conteos al objeto:

* `tools_added` y `tools_removed`: con `tools_changed`, cuántas herramientas se agregaron o se eliminaron de la solicitud
* `system_char_delta`: con `system_prompt_changed`, el cambio en la longitud del prompt del sistema, en caracteres

<h2 id="examples">
  Ejemplos
</h2>

Estos ejemplos muestran patrones comunes de línea de estado. Para usar cualquier ejemplo:

1. Guarda el script en un archivo como `~/.claude/statusline.sh` (o `.py`/`.js`)
2. Hazlo ejecutable: `chmod +x ~/.claude/statusline.sh`
3. Agrega la ruta a tu [configuración](#manually-configure-a-status-line)

Los ejemplos de Bash usan [`jq`](https://jqlang.org/) para analizar JSON. Python y Node.js tienen análisis JSON integrado.

<h3 id="context-window-usage">
  Uso de ventana de contexto
</h3>

Muestra el modelo actual y el uso de la ventana de contexto con una barra de progreso visual. Cada script lee JSON desde stdin, extrae el campo `used_percentage` y construye una barra de 10 caracteres donde los bloques rellenos (▓) representan el uso:

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-context-window-usage.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=15b58ab3602f036939145dde3165c6f7" alt="Una línea de estado que muestra el nombre del modelo y una barra de progreso con porcentaje" width="448" height="152" data-path="images/statusline-context-window-usage.png" />
</Frame>

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  # Read all of stdin into a variable
  input=$(cat)

  # Extract fields with jq, "// 0" provides fallback for null
  MODEL=$(echo "$input" | jq -r '.model.display_name')
  PCT=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)

  # Build progress bar: printf -v creates a run of spaces, then
  # ${var// /▓} replaces each space with a block character
  BAR_WIDTH=10
  FILLED=$((PCT * BAR_WIDTH / 100))
  EMPTY=$((BAR_WIDTH - FILLED))
  BAR=""
  [ "$FILLED" -gt 0 ] && printf -v FILL "%${FILLED}s" && BAR="${FILL// /▓}"
  [ "$EMPTY" -gt 0 ] && printf -v PAD "%${EMPTY}s" && BAR="${BAR}${PAD// /░}"

  echo "[$MODEL] $BAR $PCT%"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys

  # json.load reads and parses stdin in one step
  data = json.load(sys.stdin)
  model = data['model']['display_name']
  # "or 0" handles null values
  pct = int(data.get('context_window', {}).get('used_percentage', 0) or 0)

  # String multiplication builds the bar
  filled = pct * 10 // 100
  bar = '▓' * filled + '░' * (10 - filled)

  print(f"[{model}] {bar} {pct}%")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  // Node.js reads stdin asynchronously with events
  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      // Optional chaining (?.) safely handles null fields
      const pct = Math.floor(data.context_window?.used_percentage || 0);

      // String.repeat() builds the bar
      const filled = Math.floor(pct * 10 / 100);
      const bar = '▓'.repeat(filled) + '░'.repeat(10 - filled);

      console.log(`[${model}] ${bar} ${pct}%`);
  });
  ```
</CodeGroup>

<h3 id="git-status-with-colors">
  Estado de git con colores
</h3>

Muestra la rama de git con indicadores codificados por colores para archivos preparados y modificados. Este script usa [códigos de escape ANSI](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors) para colores de terminal: `\033[32m` es verde, `\033[33m` es amarillo, y `\033[0m` restablece al predeterminado.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-git-context.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=e656f34f90d1d9a1d0e220988914345f" alt="Una línea de estado que muestra modelo, directorio, rama de git e indicadores codificados por colores para archivos preparados y modificados" width="742" height="178" data-path="images/statusline-git-context.png" />
</Frame>

Cada script verifica si el directorio actual es un repositorio de git, cuenta archivos preparados y modificados, y muestra indicadores codificados por colores:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  DIR=$(echo "$input" | jq -r '.workspace.current_dir')

  GREEN='\033[32m'
  YELLOW='\033[33m'
  RESET='\033[0m'

  if git rev-parse --git-dir > /dev/null 2>&1; then
      BRANCH=$(git branch --show-current 2>/dev/null)
      STAGED=$(git diff --cached --numstat 2>/dev/null | wc -l | tr -d ' ')
      MODIFIED=$(git diff --numstat 2>/dev/null | wc -l | tr -d ' ')

      GIT_STATUS=""
      [ "$STAGED" -gt 0 ] && GIT_STATUS="${GREEN}+${STAGED}${RESET}"
      [ "$MODIFIED" -gt 0 ] && GIT_STATUS="${GIT_STATUS}${YELLOW}~${MODIFIED}${RESET}"

      echo -e "[$MODEL] 📁 ${DIR##*/} | 🌿 $BRANCH $GIT_STATUS"
  else
      echo "[$MODEL] 📁 ${DIR##*/}"
  fi
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, os

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  directory = os.path.basename(data['workspace']['current_dir'])

  GREEN, YELLOW, RESET = '\033[32m', '\033[33m', '\033[0m'

  try:
      subprocess.check_output(['git', 'rev-parse', '--git-dir'], stderr=subprocess.DEVNULL)
      branch = subprocess.check_output(['git', 'branch', '--show-current'], text=True).strip()
      staged_output = subprocess.check_output(['git', 'diff', '--cached', '--numstat'], text=True).strip()
      modified_output = subprocess.check_output(['git', 'diff', '--numstat'], text=True).strip()
      staged = len(staged_output.split('\n')) if staged_output else 0
      modified = len(modified_output.split('\n')) if modified_output else 0

      git_status = f"{GREEN}+{staged}{RESET}" if staged else ""
      git_status += f"{YELLOW}~{modified}{RESET}" if modified else ""

      print(f"[{model}] 📁 {directory} | 🌿 {branch} {git_status}")
  except:
      print(f"[{model}] 📁 {directory}")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const dir = path.basename(data.workspace.current_dir);

      const GREEN = '\x1b[32m', YELLOW = '\x1b[33m', RESET = '\x1b[0m';

      try {
          execSync('git rev-parse --git-dir', { stdio: 'ignore' });
          const branch = execSync('git branch --show-current', { encoding: 'utf8' }).trim();
          const staged = execSync('git diff --cached --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;
          const modified = execSync('git diff --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;

          let gitStatus = staged ? `${GREEN}+${staged}${RESET}` : '';
          gitStatus += modified ? `${YELLOW}~${modified}${RESET}` : '';

          console.log(`[${model}] 📁 ${dir} | 🌿 ${branch} ${gitStatus}`);
      } catch {
          console.log(`[${model}] 📁 ${dir}`);
      }
  });
  ```
</CodeGroup>

<h3 id="cost-and-duration-tracking">
  Seguimiento de costos y duración
</h3>

Rastrea los costos de API de tu sesión y el tiempo transcurrido. El campo `cost.total_cost_usd` acumula el costo estimado de todas las llamadas a API en la sesión actual. El campo `cost.total_duration_ms` mide el tiempo total transcurrido desde que comenzó la sesión, mientras que `cost.total_api_duration_ms` rastrea solo el tiempo dedicado a esperar respuestas de API.

Cada script formatea el costo como moneda y convierte milisegundos a minutos y segundos:

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-cost-tracking.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=e3444a51fe6f3440c134bd5f1f08ad29" alt="Una línea de estado que muestra el nombre del modelo, costo de sesión y duración" width="588" height="180" data-path="images/statusline-cost-tracking.png" />
</Frame>

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  COST=$(echo "$input" | jq -r '.cost.total_cost_usd // 0')
  DURATION_MS=$(echo "$input" | jq -r '.cost.total_duration_ms // 0')

  COST_FMT=$(printf '$%.2f' "$COST")
  DURATION_SEC=$((DURATION_MS / 1000))
  MINS=$((DURATION_SEC / 60))
  SECS=$((DURATION_SEC % 60))

  echo "[$MODEL] 💰 $COST_FMT | ⏱️ ${MINS}m ${SECS}s"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  cost = data.get('cost', {}).get('total_cost_usd', 0) or 0
  duration_ms = data.get('cost', {}).get('total_duration_ms', 0) or 0

  duration_sec = duration_ms // 1000
  mins, secs = duration_sec // 60, duration_sec % 60

  print(f"[{model}] 💰 ${cost:.2f} | ⏱️ {mins}m {secs}s")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const cost = data.cost?.total_cost_usd || 0;
      const durationMs = data.cost?.total_duration_ms || 0;

      const durationSec = Math.floor(durationMs / 1000);
      const mins = Math.floor(durationSec / 60);
      const secs = durationSec % 60;

      console.log(`[${model}] 💰 $${cost.toFixed(2)} | ⏱️ ${mins}m ${secs}s`);
  });
  ```
</CodeGroup>

<h3 id="display-multiple-lines">
  Mostrar múltiples líneas
</h3>

Tu script puede generar múltiples líneas para crear una pantalla más rica.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-multiline.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=60f11387658acc9ff75158ae85f2ac87" alt="Una línea de estado de múltiples líneas que muestra el nombre del modelo, directorio, rama de git en la primera línea, y una barra de progreso de uso de contexto con costo y duración en la segunda línea" width="776" height="212" data-path="images/statusline-multiline.png" />
</Frame>

Este ejemplo combina varias técnicas: colores basados en umbrales (verde por debajo del 70%, amarillo 70-89%, rojo 90%+), una barra de progreso e información de rama de git. Cada declaración `print` o `echo` crea una fila separada:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  DIR=$(echo "$input" | jq -r '.workspace.current_dir')
  COST=$(echo "$input" | jq -r '.cost.total_cost_usd // 0')
  PCT=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)
  DURATION_MS=$(echo "$input" | jq -r '.cost.total_duration_ms // 0')

  CYAN='\033[36m'; GREEN='\033[32m'; YELLOW='\033[33m'; RED='\033[31m'; RESET='\033[0m'

  # Pick bar color based on context usage
  if [ "$PCT" -ge 90 ]; then BAR_COLOR="$RED"
  elif [ "$PCT" -ge 70 ]; then BAR_COLOR="$YELLOW"
  else BAR_COLOR="$GREEN"; fi

  FILLED=$((PCT / 10)); EMPTY=$((10 - FILLED))
  printf -v FILL "%${FILLED}s"; printf -v PAD "%${EMPTY}s"
  BAR="${FILL// /█}${PAD// /░}"

  MINS=$((DURATION_MS / 60000)); SECS=$(((DURATION_MS % 60000) / 1000))

  BRANCH=""
  git rev-parse --git-dir > /dev/null 2>&1 && BRANCH=" | 🌿 $(git branch --show-current 2>/dev/null)"

  echo -e "${CYAN}[$MODEL]${RESET} 📁 ${DIR##*/}$BRANCH"
  COST_FMT=$(printf '$%.2f' "$COST")
  echo -e "${BAR_COLOR}${BAR}${RESET} ${PCT}% | ${YELLOW}${COST_FMT}${RESET} | ⏱️ ${MINS}m ${SECS}s"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, os

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  directory = os.path.basename(data['workspace']['current_dir'])
  cost = data.get('cost', {}).get('total_cost_usd', 0) or 0
  pct = int(data.get('context_window', {}).get('used_percentage', 0) or 0)
  duration_ms = data.get('cost', {}).get('total_duration_ms', 0) or 0

  CYAN, GREEN, YELLOW, RED, RESET = '\033[36m', '\033[32m', '\033[33m', '\033[31m', '\033[0m'

  bar_color = RED if pct >= 90 else YELLOW if pct >= 70 else GREEN
  filled = pct // 10
  bar = '█' * filled + '░' * (10 - filled)

  mins, secs = duration_ms // 60000, (duration_ms % 60000) // 1000

  try:
      branch = subprocess.check_output(['git', 'branch', '--show-current'], text=True, stderr=subprocess.DEVNULL).strip()
      branch = f" | 🌿 {branch}" if branch else ""
  except:
      branch = ""

  print(f"{CYAN}[{model}]{RESET} 📁 {directory}{branch}")
  print(f"{bar_color}{bar}{RESET} {pct}% | {YELLOW}${cost:.2f}{RESET} | ⏱️ {mins}m {secs}s")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const dir = path.basename(data.workspace.current_dir);
      const cost = data.cost?.total_cost_usd || 0;
      const pct = Math.floor(data.context_window?.used_percentage || 0);
      const durationMs = data.cost?.total_duration_ms || 0;

      const CYAN = '\x1b[36m', GREEN = '\x1b[32m', YELLOW = '\x1b[33m', RED = '\x1b[31m', RESET = '\x1b[0m';

      const barColor = pct >= 90 ? RED : pct >= 70 ? YELLOW : GREEN;
      const filled = Math.floor(pct / 10);
      const bar = '█'.repeat(filled) + '░'.repeat(10 - filled);

      const mins = Math.floor(durationMs / 60000);
      const secs = Math.floor((durationMs % 60000) / 1000);

      let branch = '';
      try {
          branch = execSync('git branch --show-current', { encoding: 'utf8', stdio: ['pipe', 'pipe', 'ignore'] }).trim();
          branch = branch ? ` | 🌿 ${branch}` : '';
      } catch {}

      console.log(`${CYAN}[${model}]${RESET} 📁 ${dir}${branch}`);
      console.log(`${barColor}${bar}${RESET} ${pct}% | ${YELLOW}$${cost.toFixed(2)}${RESET} | ⏱️ ${mins}m ${secs}s`);
  });
  ```
</CodeGroup>

<h3 id="clickable-links">
  Enlaces clickeables
</h3>

Este ejemplo crea un enlace clickeable a tu repositorio de GitHub. Mantén presionado Cmd (macOS) o Ctrl (Windows/Linux) y haz clic para abrir el enlace en tu navegador.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-links.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=4bcc6e7deb7cf52f41ab85a219b52661" alt="Una línea de estado que muestra un enlace clickeable a un repositorio de GitHub" width="726" height="198" data-path="images/statusline-links.png" />
</Frame>

Cada script obtiene la URL remota de git, convierte el formato SSH a HTTPS, y envuelve el nombre del repositorio en códigos de escape OSC 8. La versión de Bash usa `printf '%b'` que interpreta escapes de barra invertida de manera más confiable que `echo -e` en diferentes shells:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')

  # Convert git SSH URL to HTTPS
  REMOTE=$(git remote get-url origin 2>/dev/null | sed 's/git@github.com:/https:\/\/github.com\//' | sed 's/\.git$//')

  if [ -n "$REMOTE" ]; then
      REPO_NAME=$(basename "$REMOTE")
      # OSC 8 format: \e]8;;URL\a then TEXT then \e]8;;\a
      # printf %b interprets escape sequences reliably across shells
      printf '%b' "[$MODEL] 🔗 \e]8;;${REMOTE}\a${REPO_NAME}\e]8;;\a\n"
  else
      echo "[$MODEL]"
  fi
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, re, os

  data = json.load(sys.stdin)
  model = data['model']['display_name']

  # Get git remote URL
  try:
      remote = subprocess.check_output(
          ['git', 'remote', 'get-url', 'origin'],
          stderr=subprocess.DEVNULL, text=True
      ).strip()
      # Convert SSH to HTTPS format
      remote = re.sub(r'^git@github\.com:', 'https://github.com/', remote)
      remote = re.sub(r'\.git$', '', remote)
      repo_name = os.path.basename(remote)
      # OSC 8 escape sequences
      link = f"\033]8;;{remote}\a{repo_name}\033]8;;\a"
      print(f"[{model}] 🔗 {link}")
  except:
      print(f"[{model}]")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;

      try {
          let remote = execSync('git remote get-url origin', { encoding: 'utf8', stdio: ['pipe', 'pipe', 'ignore'] }).trim();
          // Convert SSH to HTTPS format
          remote = remote.replace(/^git@github\.com:/, 'https://github.com/').replace(/\.git$/, '');
          const repoName = path.basename(remote);
          // OSC 8 escape sequences
          const link = `\x1b]8;;${remote}\x07${repoName}\x1b]8;;\x07`;
          console.log(`[${model}] 🔗 ${link}`);
      } catch {
          console.log(`[${model}]`);
      }
  });
  ```
</CodeGroup>

<h3 id="rate-limit-usage">
  Uso de límite de velocidad
</h3>

Muestra el uso del límite de velocidad de suscripción de Claude.ai en la línea de estado. El objeto `rate_limits` contiene una ventana móvil `five_hour` y una ventana semanal `seven_day`. Cada ventana proporciona `used_percentage`, de 0 a 100, y `resets_at`, los segundos de época Unix cuando se reinicia la ventana.

Detrás de una puerta de enlace de aplicaciones Claude con límites de gasto, `rate_limits` lleva `spend_limit` con los mismos dos campos para el límite de gasto que se aplica a ti, excepto que su `used_percentage` puede superar 100 una vez que excedas el límite. Requiere Claude Code v2.1.251 o posterior.

El objeto `rate_limits` solo está presente para suscriptores de Claude.ai Pro y Max, o detrás de una puerta de enlace de aplicaciones Claude con límites de gasto, y solo después de la primera respuesta de API. Cada script maneja el campo ausente con elegancia:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  # "// empty" produces no output when rate_limits is absent
  FIVE_H=$(echo "$input" | jq -r '.rate_limits.five_hour.used_percentage // empty')
  WEEK=$(echo "$input" | jq -r '.rate_limits.seven_day.used_percentage // empty')

  LIMITS=""
  [ -n "$FIVE_H" ] && LIMITS="5h: $(printf '%.0f' "$FIVE_H")%"
  [ -n "$WEEK" ] && LIMITS="${LIMITS:+$LIMITS }7d: $(printf '%.0f' "$WEEK")%"

  [ -n "$LIMITS" ] && echo "[$MODEL] | $LIMITS" || echo "[$MODEL]"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys

  data = json.load(sys.stdin)
  model = data['model']['display_name']

  parts = []
  rate = data.get('rate_limits', {})
  five_h = rate.get('five_hour', {}).get('used_percentage')
  week = rate.get('seven_day', {}).get('used_percentage')

  if five_h is not None:
      parts.append(f"5h: {five_h:.0f}%")
  if week is not None:
      parts.append(f"7d: {week:.0f}%")

  if parts:
      print(f"[{model}] | {' '.join(parts)}")
  else:
      print(f"[{model}]")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;

      const parts = [];
      const fiveH = data.rate_limits?.five_hour?.used_percentage;
      const week = data.rate_limits?.seven_day?.used_percentage;

      if (fiveH != null) parts.push(`5h: ${Math.round(fiveH)}%`);
      if (week != null) parts.push(`7d: ${Math.round(week)}%`);

      console.log(parts.length ? `[${model}] | ${parts.join(' ')}` : `[${model}]`);
  });
  ```
</CodeGroup>

<h3 id="cache-expensive-operations">
  Cachear operaciones costosas
</h3>

Tu script de línea de estado se ejecuta frecuentemente durante sesiones activas. Comandos como `git status` o `git diff` pueden ser lentos, especialmente en repositorios grandes. Este ejemplo cachea información de git en un archivo temporal y solo la actualiza cada 5 segundos.

El nombre del archivo de caché debe ser estable en las invocaciones de línea de estado dentro de una sesión, pero único en sesiones para que las sesiones concurrentes en diferentes repositorios no lean el estado de git cacheado de cada una. Los identificadores basados en procesos como `$$`, `os.getpid()`, o `process.pid` cambian en cada invocación y anulan el caché. Usa el `session_id` de la entrada JSON en su lugar: es estable durante la vida útil de una sesión y único por sesión.

Cada script verifica si el archivo de caché falta o es más antiguo que 5 segundos antes de ejecutar comandos de git:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  DIR=$(echo "$input" | jq -r '.workspace.current_dir')
  SESSION_ID=$(echo "$input" | jq -r '.session_id')

  CACHE_FILE="/tmp/statusline-git-cache-$SESSION_ID"
  CACHE_MAX_AGE=5  # seconds

  cache_is_stale() {
      [ ! -f "$CACHE_FILE" ] || \
      # stat -c %Y (Linux) or stat -f %m (macOS) prints the file's last-modified
      # time. The Linux form must run first: on Linux, the macOS form prints a
      # filesystem report to stdout before failing, and that output would be
      # captured by the command substitution and break the arithmetic.
      [ $(($(date +%s) - $(stat -c %Y "$CACHE_FILE" 2>/dev/null || stat -f %m "$CACHE_FILE" 2>/dev/null || echo 0))) -gt $CACHE_MAX_AGE ]
  }

  if cache_is_stale; then
      if git rev-parse --git-dir > /dev/null 2>&1; then
          BRANCH=$(git branch --show-current 2>/dev/null)
          STAGED=$(git diff --cached --numstat 2>/dev/null | wc -l | tr -d ' ')
          MODIFIED=$(git diff --numstat 2>/dev/null | wc -l | tr -d ' ')
          echo "$BRANCH|$STAGED|$MODIFIED" > "$CACHE_FILE"
      else
          echo "||" > "$CACHE_FILE"
      fi
  fi

  IFS='|' read -r BRANCH STAGED MODIFIED < "$CACHE_FILE"

  if [ -n "$BRANCH" ]; then
      echo "[$MODEL] 📁 ${DIR##*/} | 🌿 $BRANCH +$STAGED ~$MODIFIED"
  else
      echo "[$MODEL] 📁 ${DIR##*/}"
  fi
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, os, time

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  directory = os.path.basename(data['workspace']['current_dir'])
  session_id = data['session_id']

  CACHE_FILE = f"/tmp/statusline-git-cache-{session_id}"
  CACHE_MAX_AGE = 5  # seconds

  def cache_is_stale():
      if not os.path.exists(CACHE_FILE):
          return True
      return time.time() - os.path.getmtime(CACHE_FILE) > CACHE_MAX_AGE

  if cache_is_stale():
      try:
          subprocess.check_output(['git', 'rev-parse', '--git-dir'], stderr=subprocess.DEVNULL)
          branch = subprocess.check_output(['git', 'branch', '--show-current'], text=True).strip()
          staged = subprocess.check_output(['git', 'diff', '--cached', '--numstat'], text=True).strip()
          modified = subprocess.check_output(['git', 'diff', '--numstat'], text=True).strip()
          staged_count = len(staged.split('\n')) if staged else 0
          modified_count = len(modified.split('\n')) if modified else 0
          with open(CACHE_FILE, 'w') as f:
              f.write(f"{branch}|{staged_count}|{modified_count}")
      except:
          with open(CACHE_FILE, 'w') as f:
              f.write("||")

  with open(CACHE_FILE) as f:
      branch, staged, modified = f.read().strip().split('|')

  if branch:
      print(f"[{model}] 📁 {directory} | 🌿 {branch} +{staged} ~{modified}")
  else:
      print(f"[{model}] 📁 {directory}")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const fs = require('fs');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const dir = path.basename(data.workspace.current_dir);
      const sessionId = data.session_id;

      const CACHE_FILE = `/tmp/statusline-git-cache-${sessionId}`;
      const CACHE_MAX_AGE = 5; // seconds

      const cacheIsStale = () => {
          if (!fs.existsSync(CACHE_FILE)) return true;
          return (Date.now() / 1000) - fs.statSync(CACHE_FILE).mtimeMs / 1000 > CACHE_MAX_AGE;
      };

      if (cacheIsStale()) {
          try {
              execSync('git rev-parse --git-dir', { stdio: 'ignore' });
              const branch = execSync('git branch --show-current', { encoding: 'utf8' }).trim();
              const staged = execSync('git diff --cached --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;
              const modified = execSync('git diff --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;
              fs.writeFileSync(CACHE_FILE, `${branch}|${staged}|${modified}`);
          } catch {
              fs.writeFileSync(CACHE_FILE, '||');
          }
      }

      const [branch, staged, modified] = fs.readFileSync(CACHE_FILE, 'utf8').trim().split('|');

      if (branch) {
          console.log(`[${model}] 📁 ${dir} | 🌿 ${branch} +${staged} ~${modified}`);
      } else {
          console.log(`[${model}] 📁 ${dir}`);
      }
  });
  ```
</CodeGroup>

<h3 id="windows-configuration">
  Configuración de Windows
</h3>

En Windows, Claude Code ejecuta comandos de línea de estado a través de Git Bash cuando Git Bash está instalado, o a través de PowerShell cuando Git Bash está ausente.

Git Bash trata las barras invertidas sin comillas como caracteres de escape, por lo que una ruta de estilo Windows como `C:\Users\username\script.mjs` llega al ejecutor de scripts con sus separadores eliminados y el comando falla sin un error visible. Escribe rutas de archivo en la cadena `command` con barras diagonales, como se muestra en los ejemplos a continuación. El atajo `~` también funciona y se expande a tu directorio de inicio de Windows.

Para ejecutar un script de PowerShell como tu línea de estado, invócalo mediante `powershell`. Esto funciona ya sea que Claude Code enrute el comando a través de Git Bash o PowerShell:

<CodeGroup>
  ```json settings.json theme={null}
  {
    "statusLine": {
      "type": "command",
      "command": "powershell -NoProfile -File C:/Users/username/.claude/statusline.ps1"
    }
  }
  ```

  ```powershell statusline.ps1 theme={null}
  $input_json = $input | Out-String | ConvertFrom-Json
  $cwd = $input_json.cwd
  $model = $input_json.model.display_name
  $used = $input_json.context_window.used_percentage
  $dirname = Split-Path $cwd -Leaf

  if ($used) {
      Write-Host "$dirname [$model] ctx: $used%"
  } else {
      Write-Host "$dirname [$model]"
  }
  ```
</CodeGroup>

O, cuando Git Bash está instalado, ejecuta un script de Bash directamente:

<CodeGroup>
  ```json settings.json theme={null}
  {
    "statusLine": {
      "type": "command",
      "command": "~/.claude/statusline.sh"
    }
  }
  ```

  ```bash statusline.sh theme={null}
  #!/usr/bin/env bash
  input=$(cat)
  cwd=$(echo "$input" | grep -o '"cwd":"[^"]*"' | cut -d'"' -f4)
  model=$(echo "$input" | grep -o '"display_name":"[^"]*"' | cut -d'"' -f4)
  dirname="${cwd##*[/\\]}"
  echo "$dirname [$model]"
  ```
</CodeGroup>

<h2 id="subagent-status-lines">
  Líneas de estado de subagentes
</h2>

La configuración `subagentStatusLine` renderiza un cuerpo de fila personalizado para cada [subagente](/docs/es/sub-agents) mostrado en el panel de agentes debajo del prompt. Úsalo para reemplazar la fila predeterminada `name · description · token count` con tu propio formato.

```json theme={null}
{
  "subagentStatusLine": {
    "type": "command",
    "command": "~/.claude/subagent-statusline.sh"
  }
}
```

El comando se ejecuta una vez por tick de actualización con todas las filas de subagentes visibles pasadas como un único objeto JSON en stdin. La entrada incluye los [campos de hook base](/docs/es/hooks#common-input-fields), un campo `columns` con el ancho de fila utilizable, y un array `tasks`. Cada tarea tiene `id`, `name`, `type`, `status`, `description`, `label`, `startTime`, `model`, `effort`, `contextWindowSize`, `tokenCount`, `tokenSamples`, y `cwd`.

El campo `model` por tarea es el ID de modelo resuelto en el que se ejecuta la tarea. `contextWindowSize` es la ventana de contexto de ese modelo en tokens, calculada de la misma manera que `context_window.context_window_size` de la línea de estado principal, por lo que puedes renderizar un porcentaje por fila desde `tokenCount`. Ambos campos requieren Claude Code v2.1.205 o posterior y se omiten para una tarea cuyo modelo aún no está resuelto.

El campo `effort` por tarea es el esfuerzo de razonamiento establecido para ese subagente, en su [frontmatter de definición](/docs/es/sub-agents#supported-frontmatter-fields) o en la invocación individual. El valor es uno de los strings de nivel de esfuerzo `low`, `medium`, `high`, `xhigh`, o `max`, o un presupuesto de tokens numérico. El campo reporta el valor configurado tal como está escrito: si el modelo no admite ese nivel, el esfuerzo que Claude Code realmente aplica puede diferir. El campo requiere Claude Code v2.1.214 o posterior y está ausente cuando el subagente hereda el nivel de esfuerzo de la sesión.

Escribe una línea JSON a stdout por cada fila que desees anular, en la forma `{"id": "<task id>", "content": "<row body>"}`. La cadena `content` se renderiza tal cual, incluidos colores ANSI e hipervínculos OSC 8. Omite el `id` de una tarea para mantener el renderizado predeterminado para esa fila; emite una cadena `content` vacía para ocultarla.

Las mismas puertas de confianza, `disableAllHooks`, y [`allowManagedHooksOnly`](/docs/es/settings-reference#allowmanagedhooksonly) que se aplican a `statusLine` se aplican aquí. Los plugins pueden enviar una `subagentStatusLine` predeterminada en su [`settings.json`](/docs/es/plugins/manifest-reference#standard-layout), pero a diferencia de los hooks, los valores de plugins no se ejecutan bajo `allowManagedHooksOnly` incluso cuando el plugin está forzadamente habilitado en la configuración gestionada `enabledPlugins`.

<h2 id="tips">
  Consejos
</h2>

* **Prueba con entrada simulada**: `echo '{"model":{"display_name":"Opus"},"workspace":{"current_dir":"/home/user/project"},"context_window":{"used_percentage":25},"session_id":"test-session-abc"}' | ./statusline.sh`
* **Mantén la salida corta**: la barra de estado tiene un ancho limitado, por lo que la salida larga puede truncarse o ajustarse de manera incómoda
* **Cachea operaciones lentas**: tu script se ejecuta frecuentemente durante sesiones activas, por lo que comandos como `git status` pueden causar retrasos. Consulta el [ejemplo de caché](#cache-expensive-operations) para saber cómo manejar esto.

Proyectos comunitarios como [ccstatusline](https://github.com/sirmalloc/ccstatusline) y [starship-claude](https://github.com/martinemde/starship-claude) proporcionan configuraciones preconstruidas con temas y características adicionales.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

**La línea de estado no aparece**

* Verifica que tu script sea ejecutable: `chmod +x ~/.claude/statusline.sh`
* Comprueba que tu script genere salida a stdout, no stderr
* Ejecuta tu script manualmente para verificar que produce salida
* En Windows con Git Bash instalado, es probable que las barras invertidas en la ruta del `command` se consuman como caracteres de escape antes de que se ejecute el script. Usa barras diagonales en la ruta. Consulta [Configuración de Windows](#windows-configuration).
* Si `disableAllHooks` es `true` fuera de la configuración administrada después de que se aplica la [precedencia de configuración](/docs/es/hooks#disable-or-remove-hooks), Claude Code ejecuta solo un `statusLine` de la configuración administrada, y sin un `statusLine` administrado la línea de estado está deshabilitada. Elimina la configuración o establécela en `false` en el archivo que la establece para volver a habilitarla. Consulta [`disableAllHooks`](/docs/es/settings-reference#disableallhooks).
* Si tu organización establece `allowManagedHooksOnly` en la configuración administrada, tu línea de estado personalizada desaparece sin advertencia: solo puedes obtener una línea de estado de un valor `statusLine` en esa configuración administrada. Consulta [qué se ejecuta bajo `allowManagedHooksOnly`](/docs/es/settings-reference#what-runs-under-allowmanagedhooksonly) para el comportamiento completo, y pregunta a tu administrador si esta configuración se aplica a ti.
* Ejecuta `claude --debug` para registrar el código de salida y stderr de la primera invocación de línea de estado en una sesión
* Pídele a Claude que lea tu archivo de configuración y ejecute el comando `statusLine` directamente para exponer errores

**La línea de estado muestra `--` o valores vacíos**

* Los campos pueden ser `null` antes de que se complete la primera respuesta de API
* Maneja valores nulos en tu script con valores predeterminados de respaldo como `// 0` en jq
* Reinicia Claude Code si los valores permanecen vacíos después de múltiples mensajes

**El porcentaje de contexto muestra valores inesperados**

* Usa `used_percentage` para el estado de contexto más simple y preciso
* El porcentaje de contexto puede diferir de la salida `/context` debido a cuándo se calcula cada uno

**Los enlaces OSC 8 no son clickeables**

* Verifica que tu terminal admita hipervínculos OSC 8 (iTerm2, Kitty, WezTerm)

* Terminal.app no admite enlaces clickeables

* Si el texto del enlace aparece pero no es clickeable, Claude Code puede no haber detectado soporte de hipervínculos en tu terminal. Establece la variable de entorno `FORCE_HYPERLINK` para anular la detección antes de lanzar Claude Code:

  ```bash theme={null}
  FORCE_HYPERLINK=1 claude
  ```

  En PowerShell, establece la variable en la sesión actual primero:

  ```powershell theme={null}
  $env:FORCE_HYPERLINK = "1"; claude
  ```

* Las sesiones SSH y tmux pueden eliminar secuencias OSC dependiendo de la configuración

* Si las secuencias de escape aparecen como texto literal como `\e]8;;`, usa `printf '%b'` en lugar de `echo -e` para un manejo más confiable de escapes

**Problemas de visualización con secuencias de escape**

* Las secuencias de escape complejas (colores ANSI, enlaces OSC 8) pueden ocasionalmente causar salida garbled si se superponen con otras actualizaciones de la interfaz
* Si ves texto corrupto, intenta simplificar tu script a salida de texto plano
* Las líneas de estado de múltiples líneas con códigos de escape son más propensas a problemas de renderizado que el texto plano de una sola línea

**Confianza del espacio de trabajo requerida**

* Debido a que `statusLine` ejecuta un comando de shell, Claude Code lo ejecuta bajo la misma [regla de confianza del espacio de trabajo que los hooks en archivos de configuración](/docs/es/permissions#what-runs-before-you-trust-a-folder). Aceptar el diálogo para la carpeta, o para un directorio padre cuya confianza se extiende a ella, es suficiente.
* Hasta entonces, la línea de estado permanece en blanco, y `claude --debug` registra `Status line command skipped: workspace trust not accepted`. Reinicia Claude Code y acepta el diálogo de confianza para habilitarlo.

**Errores de script o bloqueos**

* Los scripts que salen con códigos distintos de cero o no producen salida hacen que la línea de estado se quede en blanco
* Los scripts lentos bloquean la línea de estado de actualizar hasta que se completen. Mantén los scripts rápidos para evitar salida obsoleta.
* Si una nueva actualización se activa mientras un script lento se está ejecutando, el script en vuelo se cancela
* Prueba tu script de forma independiente con entrada simulada antes de configurarlo

**Las notificaciones comparten la fila de la línea de estado**

Fuera de la [representación a pantalla completa](/docs/es/fullscreen), Claude Code muestra notificaciones en la misma fila que tu línea de estado. En la representación a pantalla completa, Claude Code asigna a las notificaciones una fila propia.

* Las notificaciones del sistema como errores de servidor MCP y actualizaciones automáticas se muestran en el lado derecho de la fila. Las notificaciones transitorias como la advertencia de contexto bajo también ciclan a través de esta área.
* Habilitar el modo verbose agrega un contador de tokens a esta área
* En terminales estrechas, estas notificaciones pueden truncar tu salida de línea de estado
