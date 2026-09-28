> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Recomienda tu plugin desde tu CLI

> Solicita a los usuarios de Claude Code que instalen tu plugin del marketplace oficial emitiendo una etiqueta claude-code-hint desde tu CLI o SDK.

Si mantienes una CLI o SDK, tu herramienta puede solicitar a los usuarios de Claude Code que instalen tu plugin. Cuando tu CLI detecta que se está ejecutando dentro de Claude Code, debe escribir una etiqueta `<claude-code-hint />` de una sola línea en stderr. Claude Code elimina la línea de la salida de las herramientas Bash y PowerShell antes de que el modelo vea la salida, y luego muestra al usuario un mensaje de instalación de una sola vez.

Esta página se aplica solo si tu plugin está listado en `claude-plugins-official` u otro marketplace con uno de los [nombres de marketplace oficiales](/docs/es/plugins/security#official-marketplace-names) de Anthropic. El marketplace comunitario, `claude-community`, no es uno de ellos.

<Note>
  Para publicar un plugin, consulta [Publicar y distribuir un plugin](/docs/es/plugins/publish).
</Note>

<h2 id="emit-the-hint">
  Emitir la sugerencia
</h2>

Emite la etiqueta solo cuando `CLAUDECODE` o `CLAUDE_CODE_CHILD_SESSION` esté establecido, para que no aparezca cuando una persona ejecute tu CLI directamente.

Claude Code establece `CLAUDECODE=1` en los comandos que ejecuta a través de las herramientas Bash y PowerShell y en comandos hook. En v2.1.172 y posteriores también establece `CLAUDE_CODE_CHILD_SESSION=1` allí. Las variables difieren en qué procesos las llevan:

* **`CLAUDECODE`**: establecido por cada versión de Claude Code. Las extensiones IDE también lo establecen en sus terminales integradas, por lo que una puerta en `CLAUDECODE` solo también emite la etiqueta cuando una persona ejecuta tu CLI directamente en una de esas terminales
* **`CLAUDE_CODE_CHILD_SESSION`**: establecido solo en subprocesos que inicia Claude Code. Úsalo cuando puedas requerir v2.1.172 o posterior

La [referencia de variables de entorno](/docs/es/env-vars) tiene los detalles.

Los siguientes ejemplos usan `CLAUDECODE` para el alcance más amplio y emiten una sugerencia para un plugin llamado `example-cli` en el marketplace oficial:

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
  if [ -n "$CLAUDECODE" ]; then
    printf '%s\n' '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />' >&2
  fi
  ```
</CodeGroup>

Reemplaza `example-cli` con el nombre de tu plugin en el marketplace oficial.

Puedes emitir la sugerencia en cada invocación, porque Claude Code solicita cada plugin una sola vez.

Para verificar el emisor, ejecuta `CLAUDECODE=1 example-cli` en una terminal y confirma que la línea de etiqueta aparece en stderr, luego ejecuta `example-cli` sin la variable y confirma que no se imprime nada extra.

<h2 id="hint-format">
  Formato de la sugerencia
</h2>

La etiqueta debe ocupar su propia línea; Claude Code ignora una etiqueta incrustada a mitad de línea.

La etiqueta toma tres atributos, todos requeridos:

| Atributo | Descripción                                               |
| :------- | :-------------------------------------------------------- |
| `v`      | Versión del protocolo. `1` es el único valor compatible   |
| `type`   | Tipo de sugerencia. `plugin` es el único valor compatible |
| `value`  | Identificador del plugin en forma `name@marketplace`      |

Los valores pueden estar entre comillas dobles o sin comillas; un valor sin comillas no puede contener espacios en blanco.

Claude Code elimina la línea de la salida incluso cuando `v` o `type` no se reconoce.

<h2 id="check-when-the-prompt-appears">
  Verificar cuándo aparece el mensaje
</h2>

El mensaje aparece solo en sesiones de terminal interactivas. En ejecuciones de `claude -p`, en ejecuciones de subagentes y en la salida de comandos hook, la etiqueta se elimina y no se muestra ningún mensaje. Todas estas comprobaciones también deben pasar:

* **Oficial e instalable**: `value` nombra un plugin que Claude Code encuentra en su copia local de un marketplace oficial, que aún no está instalado y que ninguna política bloquea
* **Análisis activado**: una sesión donde los análisis de Claude Code están desactivados nunca solicita, por ejemplo una con `DISABLE_TELEMETRY`, `DO_NOT_TRACK` o `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` establecidos, o una en un proveedor de terceros como Amazon Bedrock, donde se aplica la [exclusión automática de telemetría](/docs/es/data-usage#default-behaviors-by-api-provider)
* **Límites de frecuencia**: un mensaje por sesión, un mensaje en total por plugin independientemente de la respuesta del usuario, y ninguno una vez que se han solicitado 100 plugins en esa máquina
* **No desactivado**: el usuario no ha elegido **No, y no vuelvas a mostrar sugerencias de instalación de plugins**
* **Sesión local y atendida**: el espacio de trabajo de la sesión es local en lugar de estar en una máquina en la nube o remota, y la sesión no se ejecuta sin supervisión. Por ejemplo, una sesión iniciada con `--cloud`, una que sirve Control Remoto, o un compañero de equipo de agentes nunca solicita

<h2 id="preview-what-the-user-sees">
  Previsualizar lo que ve el usuario
</h2>

Cuando las comprobaciones en [Verificar cuándo aparece el mensaje](#check-when-the-prompt-appears) pasan, Claude Code muestra un diálogo de **Recomendación de plugin** como el siguiente:

```text theme={null}
─────────────────────────────────────────────────────────────
  Recomendación de plugin

    El comando example-cli sugiere instalar un plugin.

    Plugin: example-cli
    Marketplace: claude-plugins-official
    Descripción: Integración oficial para implementaciones de example-cli

    ¿Te gustaría instalarlo?
    ❯ 1. Sí, instalar
      2. No
      3. No, y no vuelvas a mostrar sugerencias de instalación de plugins

─────────────────────────────────────────────────────────────
```

El diálogo nombra la primera palabra del comando shell que ejecutó Claude, para que los usuarios puedan detectar una discrepancia. Cada respuesta tiene un efecto:

* **Sí, instalar**: instala el plugin en [ámbito de usuario](/docs/es/plugins/install)
* **No, y no vuelvas a mostrar sugerencias de instalación de plugins**: desactiva futuros mensajes de sugerencia para ese usuario
* **Sin respuesta durante 30 segundos**: cuenta como **No**

<h2 id="next-steps">
  Próximos pasos
</h2>

* [Publicar y distribuir un plugin](/docs/es/plugins/publish): las rutas hacia cada marketplace, incluido el marketplace oficial, que la sugerencia requiere
* [Referencia de comandos de plugins](/docs/es/plugins/cli-reference#plugin-install): el comando shell que instala el mismo plugin fuera de una sesión
