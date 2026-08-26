> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Recomienda tu plugin desde tu CLI

> Emite un marcador de una línea desde tu CLI para que Claude Code solicite a los usuarios instalar tu plugin oficial.

Si mantienes una CLI o SDK y tienes un plugin en el marketplace oficial de Anthropic, tu herramienta puede solicitar a los usuarios de Claude Code que instalen ese plugin. Tu CLI escribe un marcador de una línea en stderr cuando detecta que se está ejecutando dentro de Claude Code. Claude Code lee el marcador, lo elimina de la salida y muestra al usuario un mensaje de instalación de una sola vez.

Claude Code elimina la línea de sugerencia de la salida del comando antes de enviarla al modelo, por lo que el marcador nunca aparece en la conversación y no se cuenta hacia el uso de tokens. El protocolo no requiere comandos adicionales y no cambia lo que tu CLI imprime para usuarios fuera de Claude Code.

Esta página es para mantenedores de CLI y SDK. Si buscas instalar plugins, consulta [Descubre e instala plugins](/docs/es/discover-plugins).

<h2 id="how-it-works">
  Cómo funciona
</h2>

Claude Code establece la variable de entorno [`CLAUDECODE`](/docs/es/env-vars) en `1` para cada comando que ejecuta a través de las herramientas Bash y PowerShell, y para comandos de [hook](/docs/es/hooks). A partir de v2.1.172 también establece [`CLAUDE_CODE_CHILD_SESSION`](/docs/es/env-vars) en `1` en esos mismos subprocesos. Cuando tu CLI ve una de estas variables, escribe una etiqueta `<claude-code-hint />` que se cierra automáticamente en stderr. En comandos hook la etiqueta de sugerencia se elimina e ignora. Solo la salida de las herramientas Bash y PowerShell activa el mensaje de instalación.

Cuando Claude Code recibe la salida del comando, hace lo siguiente:

1. Busca líneas de sugerencia y las elimina antes de que la salida llegue al modelo
2. Verifica que la sugerencia apunte a un plugin en un marketplace oficial de Anthropic
3. Verifica que el plugin no esté ya instalado y no haya sido sugerido antes
4. Muestra al usuario un mensaje de instalación que nombra el comando que emitió la sugerencia

Claude Code nunca instala un plugin automáticamente. El usuario siempre confirma.

<h2 id="emit-the-hint">
  Emite la sugerencia
</h2>

Las sugerencias solo se activan para plugins listados en el marketplace oficial de Anthropic. Consulta [Obtén tu plugin en el marketplace oficial](#get-your-plugin-into-the-official-marketplace) antes de enviar la integración.

Condiciona la emisión en una variable de entorno para que el marcador sea poco probable que aparezca cuando un usuario humano ejecute tu CLI directamente, luego escribe la etiqueta en stderr en su propia línea. Elige qué variable verificar:

* `CLAUDECODE`: se establece en cada versión de Claude Code, por lo que llega a la mayoría de sesiones. También se establece en sesiones de tmux y subprocesos del servidor MCP de stdio que Claude Code inicia, y las extensiones de IDE la establecen en sus terminales integradas, donde un usuario humano puede estar ejecutando tu CLI directamente.
* `CLAUDE_CODE_CHILD_SESSION`: se establece solo en subprocesos que el propio Claude Code genera, como llamadas de herramientas, comandos de hook y comandos de [línea de estado](/docs/es/statusline), por lo que la etiqueta normalmente no llega a una terminal humana. Un proceso de larga duración que se inició dentro de una sesión, como un servidor tmux, captura la variable, por lo que los shells lanzados posteriormente desde ese proceso aún muestran la etiqueta sin procesar. Requiere Claude Code v2.1.172 o posterior, por lo que las sesiones en versiones anteriores pierden la sugerencia.

Los siguientes ejemplos condicionan `CLAUDECODE` para máximo alcance y emiten una sugerencia para un plugin llamado `example-cli` en el marketplace oficial:

<CodeGroup>
  ```javascript Node.js theme={null}
  if (process.env.CLAUDECODE) {
    process.stderr.write(
      '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />\n',
    )
  }
  ```

  ```python Python theme={null}
  import os, sys

  if os.environ.get("CLAUDECODE"):
      print(
          '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />',
          file=sys.stderr,
      )
  ```

  ```go Go theme={null}
  if os.Getenv("CLAUDECODE") != "" {
      fmt.Fprintln(os.Stderr,
          `<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />`)
  }
  ```

  ```shell Shell theme={null}
  [ -n "$CLAUDECODE" ] &&
    printf '%s\n' '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />' >&2
  ```
</CodeGroup>

Reemplaza `example-cli` con el nombre de tu plugin en el marketplace oficial.

<h2 id="choose-where-to-emit">
  Elige dónde emitir
</h2>

Controlas qué rutas de código emiten la sugerencia. Claude Code deduplica por plugin, por lo que emitir en cada invocación no tiene desventajas. Los puntos de contacto que funcionan bien incluyen:

| Ubicación                                  | Por qué funciona                                                 |
| :----------------------------------------- | :--------------------------------------------------------------- |
| Salida de `--help`                         | Claude a menudo ejecuta help al explorar una CLI desconocida     |
| Errores de subcomando desconocido          | Llega al momento en que Claude está confundido sobre tu interfaz |
| Éxito de inicio de sesión o autenticación  | El usuario ya está en una mentalidad de configuración            |
| Mensaje de bienvenida de primera ejecución | Un momento natural de incorporación                              |

<h2 id="what-the-user-sees">
  Lo que ve el usuario
</h2>

Cuando la sugerencia pasa todas las verificaciones, Claude Code muestra un mensaje como el siguiente:

```text theme={null}
─────────────────────────────────────────────────────────────
  Recomendación de Plugin

    El comando example-cli sugiere instalar un plugin.

    Plugin: example-cli
    Marketplace: claude-plugins-official
    Integración oficial para implementaciones de example-cli

    ¿Te gustaría instalarlo?
    ❯ 1. Sí, instalar example-cli
      2. No
      3. No, y no mostrar sugerencias de instalación de plugins nuevamente

─────────────────────────────────────────────────────────────
```

El mensaje nombra el comando que produjo la sugerencia para que los usuarios puedan detectar una discrepancia entre la herramienta y el plugin que recomienda. Si el usuario no responde dentro de 30 segundos, el mensaje se descarta como **No**.

La frecuencia del mensaje está limitada:

* **Una vez por plugin**: después de que se muestre el mensaje, Claude Code registra el plugin y nunca vuelve a solicitar para él, independientemente de la respuesta del usuario.
* **Una vez por sesión**: en todas las CLI de la máquina, como máximo aparece un mensaje de sugerencia por sesión de Claude Code.

Seleccionar **Sí** instala el plugin en el ámbito del usuario. Seleccionar **No, y no mostrar sugerencias de instalación de plugins nuevamente** desactiva todos los mensajes de sugerencia futuros para el usuario.

<h2 id="hint-format">
  Formato de sugerencia
</h2>

La sugerencia es una etiqueta que se cierra automáticamente con tres atributos requeridos.

```text theme={null}
<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />
```

| Atributo | Requerido | Descripción                                              |
| :------- | :-------- | :------------------------------------------------------- |
| `v`      | Sí        | Versión del protocolo. `1` es el único valor soportado   |
| `type`   | Sí        | Tipo de sugerencia. `plugin` es el único valor soportado |
| `value`  | Sí        | Identificador del plugin en forma `name@marketplace`     |

Los valores de atributo pueden estar entrecomillados con comillas dobles o dejarse sin comillas. Los valores sin comillas no pueden contener espacios en blanco. Las secuencias de escape no son compatibles.

<h2 id="requirements">
  Requisitos
</h2>

Claude Code aplica dos condiciones antes de actuar sobre una sugerencia. Las sugerencias que fallan en cualquiera de las verificaciones se descartan:

* **Línea propia**: la etiqueta debe ocupar su propia línea. Una etiqueta incrustada a mitad de línea, por ejemplo dentro de una declaración de registro, se ignora. Se permite espacios en blanco al principio y al final de la línea.
* **Marketplace oficial**: el `value` debe hacer referencia a un plugin en un marketplace controlado por Anthropic como `claude-plugins-official`. Las sugerencias que apuntan a otros marketplaces se descartan silenciosamente.

La línea de sugerencia siempre se elimina de la salida antes de que llegue al modelo, incluso cuando la versión o el tipo no se reconocen, por lo que el marcador nunca se cuenta hacia el uso de tokens.

La orientación restante se recomienda pero no se aplica. Claude Code no puede observar si tu CLI la sigue:

* **Escribe en stderr**: stderr mantiene la etiqueta fuera de tuberías de shell como `example-cli deploy | jq`. Claude Code escanea ambas secuencias, por lo que stdout también funciona.
* **Condiciona en una variable de entorno**: solo emite cuando `CLAUDECODE` o `CLAUDE_CODE_CHILD_SESSION` está establecida. Consulta [Emitir la sugerencia](#emit-the-hint) para saber cómo difieren las dos variables.

<h2 id="get-your-plugin-into-the-official-marketplace">
  Obtén tu plugin en el marketplace oficial
</h2>

El protocolo de sugerencia solo tiene efecto para plugins que se enumeran en el marketplace oficial de Anthropic, `claude-plugins-official`. Anthropic cura ese marketplace a su discreción, y los formularios de envío en la aplicación agregan plugins al [marketplace de la comunidad](/docs/es/plugins#submit-your-plugin-to-the-community-marketplace) en su lugar, que el protocolo de sugerencia no verifica. Si trabajas con un contacto de socio de Anthropic, comunícate con ellos para coordinar una enumeración en el marketplace oficial.

<h2 id="see-also">
  Ver también
</h2>

* [Crea plugins](/docs/es/plugins): construye el plugin que tu CLI recomienda
* [Crea y distribuye un marketplace de plugins](/docs/es/plugin-marketplaces): aloja plugins fuera del marketplace oficial
* [Variables de entorno](/docs/es/env-vars): referencia completa para `CLAUDECODE` y variables relacionadas
