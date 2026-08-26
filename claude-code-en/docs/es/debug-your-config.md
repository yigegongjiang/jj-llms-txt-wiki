> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Depura tu configuración

> Diagnostica por qué CLAUDE.md, configuración, hooks, servidores MCP o skills no están surtiendo efecto. Usa /context, /doctor, /hooks y /mcp para ver qué se cargó realmente.

Cuando Claude ignora una instrucción o una característica que configuró no aparece, la causa suele ser que el archivo no se cargó, se cargó desde una ubicación diferente a la que esperaba, u otro archivo la anuló. Esta guía muestra cómo inspeccionar qué cargó realmente Claude Code para que pueda reducir cuál se aplica.

Para problemas de instalación, autenticación y conectividad, consulte [Troubleshooting installation and login](/docs/es/troubleshoot-install) en su lugar.

<h2 id="see-what-loaded-into-context">
  Ver qué se cargó en el contexto
</h2>

El comando `/context` muestra todo lo que ocupa la ventana de contexto para la sesión actual, desglosado por categoría: indicación del sistema, archivos de memoria, skills, subagentes personalizados con la fuente desde la que se cargó cada uno, herramientas MCP y mensajes de conversación. Ejecútelo primero para confirmar si su `CLAUDE.md`, reglas o descripciones de skills están presentes en absoluto.

Para obtener detalles sobre una categoría específica, continúe con el comando dedicado:

| Comando          | Muestra                                                                                                                                                                                                                                                                                                            |
| :--------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/memory`        | Qué archivos `CLAUDE.md` y rules se cargaron, más entradas de memoria automática                                                                                                                                                                                                                                   |
| `/skills`        | Skills disponibles de fuentes de proyecto, usuario y plugin                                                                                                                                                                                                                                                        |
| `/hooks`         | Configuraciones de hook activas                                                                                                                                                                                                                                                                                    |
| `/mcp`           | Servidores MCP conectados y su estado                                                                                                                                                                                                                                                                              |
| `/permissions`   | Reglas de permitir y denegar resueltas actualmente en vigor                                                                                                                                                                                                                                                        |
| `/doctor`        | Diagnóstico de configuración: salud de la instalación, archivos de configuración inválidos, extensiones no utilizadas, nombres de [subagentes](/docs/es/sub-agents) duplicados en el mismo directorio, y contenido de `CLAUDE.md` registrado que Claude puede derivar de la base de código, con correcciones propuestas |
| `/debug [issue]` | Habilita el registro de depuración para la sesión e indica a Claude que diagnostique usando la salida del registro y las rutas de configuración                                                                                                                                                                    |
| `/status`        | Fuentes de configuración activas, incluido si la configuración administrada está en vigor                                                                                                                                                                                                                          |

Si falta un archivo de memoria en `/memory`, verifique su ubicación contra [cómo se cargan los archivos CLAUDE.md](/docs/es/memory#how-claude-md-files-load). Los archivos `CLAUDE.md` del subdirectorio se cargan bajo demanda cuando Claude lee un archivo en ese directorio con la herramienta Read, no al inicio de la sesión.

Si `/memory` confirma que el archivo se cargó pero Claude aún no sigue una instrucción particular, el problema probablemente sea cómo se escribe la instrucción en lugar de si se cargó. CLAUDE.md funciona bien para el tipo de orientación que daría a un nuevo compañero de equipo, como convenciones de proyecto, comandos de compilación y dónde pertenecen los archivos.

La adherencia disminuye cuando una instrucción es lo suficientemente vaga como para interpretarse de múltiples formas, cuando dos archivos dan direcciones conflictivas, o cuando el archivo ha crecido lo suficiente como para que las reglas individuales reciban menos atención. [Escribir instrucciones efectivas](/docs/es/memory#write-effective-instructions) cubre los patrones de especificidad, tamaño y estructura que mantienen la adherencia alta.

<Note>
  CLAUDE.md y los permisos resuelven problemas diferentes. CLAUDE.md le dice a Claude cómo funciona su proyecto para que tome buenas decisiones. [Permisos](/docs/es/permissions) y [hooks](/docs/es/hooks) aplican límites independientemente de lo que Claude decida. Use CLAUDE.md para "lo hacemos de esta manera aquí". Use permisos o hooks para límites de seguridad y cualquier cosa que nunca deba suceder, donde necesita una garantía en lugar de orientación.
</Note>

<h2 id="check-resolved-settings">
  Verificar configuración resuelta
</h2>

La configuración se fusiona en ámbitos administrados, de usuario, de proyecto y locales. La configuración administrada siempre gana cuando está presente. Entre el resto, el ámbito más cercano anula el más amplio en el orden local, luego proyecto, luego usuario. Algunos ajustes también se pueden establecer mediante banderas de línea de comandos o [variables de entorno](/docs/es/env-vars), que actúan como otra capa de anulación. Cuando una configuración no parece aplicarse, el valor que estableció generalmente se anula por otro ámbito o una variable de entorno.

Ejecute `/doctor` para verificar su configuración e instalación. Informa lo que encuentra, incluidos archivos de configuración inválidos, instalaciones duplicadas, extensiones no utilizadas y contenido de `CLAUDE.md` registrado que Claude puede derivar de la base de código, luego propone correcciones que aplica solo después de que usted confirme. La verificación de recorte de `CLAUDE.md` requiere Claude Code v2.1.206 o posterior. Antes de v2.1.205, `/doctor` abría una pantalla de diagnóstico de solo lectura y presionar `f` enviaba el informe a Claude para corregir.

Desde la terminal, `claude doctor` imprime diagnósticos de instalación y configuración de solo lectura sin iniciar una sesión.

Ejecute `/status` para ver qué fuentes de configuración están activas, incluido si la configuración administrada está en vigor. Para entender qué ámbito gana para una clave determinada, consulte [Cómo interactúan los ámbitos](/docs/es/settings#how-scopes-interact).

<h2 id="check-mcp-servers">
  Verificar servidores MCP
</h2>

Ejecute `/mcp` para ver cada servidor configurado, su estado de conexión y si lo ha aprobado para el proyecto actual. Un servidor puede estar definido correctamente pero aún no proporcionar herramientas por algunas razones comunes:

* Los servidores con ámbito de proyecto en `.mcp.json` requieren una aprobación única. Si se descartó el mensaje, el servidor permanece deshabilitado hasta que lo apruebe desde `/mcp`.
* Un servidor que no se inicia se muestra como fallido en `/mcp`. Las rutas de archivo relativas en `command` o `args` son una causa frecuente, ya que se resuelven contra el directorio desde el que lanzó Claude Code en lugar de la ubicación de `.mcp.json`.
* Un servidor que se muestra como conectado pero enumera cero herramientas se ha iniciado correctamente pero no devuelve una lista de herramientas. Seleccione **Reconnect** desde `/mcp`. Si el recuento permanece en cero, ejecute `claude --debug mcp` para ver la salida stderr del servidor.

Para ubicaciones de configuración y reglas de ámbito, consulte [MCP](/docs/es/mcp).

<h2 id="check-hooks">
  Verificar hooks
</h2>

Ejecute `/hooks` para enumerar cada hook registrado para la sesión actual, agrupado por evento. Si un hook que definió no aparece, no se está leyendo: los hooks van bajo la clave `"hooks"` en un archivo de configuración, no en un archivo independiente.

Si el hook aparece pero no se dispara, el matcher es la causa habitual. Compruébelo para estos errores:

* El campo `matcher` es una cadena única que usa `|` para coincidir con múltiples nombres de herramientas, por ejemplo `"Edit|Write"`. Un separador `,` es equivalente, por lo que `"Edit,Write"` coincide con las mismas herramientas. Antes de v2.1.191, una coma se evaluaba como expresión regular y el matcher nunca coincidía, así que use `|` si no está en v2.1.191 aún.
* Un nombre de herramienta mal escrito produce un matcher que no coincide con nada, por lo que el hook falla silenciosamente.
* Un valor de matriz es un error de esquema: Claude Code muestra un aviso de error de configuración y rechaza todo el archivo de configuración del usuario, proyecto o local, `claude doctor` informa del error de validación, y ningún hook de ese archivo aparece en `/hooks`. En [configuración administrada](/docs/es/settings#settings-files), solo se elimina la entrada inválida y los otros hooks del archivo aún se aplican.

Las ediciones en `settings.json` surten efecto en la sesión en ejecución después de un breve retraso de estabilidad de archivo. No necesita reiniciar. Si `/hooks` aún muestra la definición anterior unos segundos después de guardar, ejecute `/hooks` nuevamente para actualizar la vista.

Si `/hooks` muestra el hook pero aún no se dispara, el siguiente paso es ver la evaluación del hook en vivo. Inicie una sesión con `claude --debug hooks` y active la llamada de herramienta. El registro de depuración registra cada evento, qué matchers se verificaron, y el código de salida y la salida del hook. Consulte [Depurar hooks](/docs/es/hooks#debug-hooks) para el formato del registro y [solución de problemas de hooks](/docs/es/hooks-guide#limitations-and-troubleshooting) para patrones de fallo comunes.

<h2 id="test-against-a-clean-configuration">
  Probar contra una configuración limpia
</h2>

Comience con [`claude --safe-mode`](/docs/es/cli-reference#cli-flags), que inicia una sesión con todas las personalizaciones deshabilitadas, incluidos `CLAUDE.md`, skills, plugins, hooks, servidores MCP y comandos y agentes personalizados. La autenticación, la selección de modelo, las herramientas integradas y los permisos funcionan normalmente. Si el problema desaparece en modo seguro, una de esas superficies es la causa; use las comprobaciones dirigidas anteriores para encontrar cuál. El modo seguro aún aplica hooks administrados y la política de configuración de su organización. Los plugins administrados, skills, CLAUDE.md y servidores MCP están desactivados.

Si el problema persiste en modo seguro, o si su configuración en sí es sospechosa, compare contra una sesión que no carga nada de su configuración habitual. Apunte [`CLAUDE_CONFIG_DIR`](/docs/es/env-vars) a un directorio vacío para omitir todo bajo `~/.claude`, e inicie desde un directorio que no tenga carpeta `.claude`, `.mcp.json` o `CLAUDE.md` para que la configuración del proyecto también se omita.

```bash theme={null}
cd /tmp && CLAUDE_CONFIG_DIR=/tmp/claude-clean claude
```

La sesión limpia no tiene configuración de usuario o proyecto, hooks, servidores MCP, plugins o memoria.

* La configuración administrada aún se aplica si su organización la implementa, ya que vive en una ruta del sistema fuera de `~/.claude`
* En Linux y Windows, se le pedirá que inicie sesión nuevamente porque las credenciales se almacenan en el directorio de configuración
* En macOS, las credenciales están en el Keychain y se transfieren a la sesión limpia

Si el problema desaparece aquí, la causa está en algún lugar de sus archivos reales `~/.claude` o `.claude` del proyecto. Reintrodúzcalos uno a la vez, copiando archivos en el directorio temporal o iniciando desde su proyecto, para encontrar cuál es. Si persiste en la sesión limpia, la causa está fuera de su configuración de usuario y proyecto. Ejecute `/status` para verificar si la configuración administrada está en vigor, busque [variables de entorno](/docs/es/env-vars) que afecten a Claude Code, luego consulte [Solución de problemas](/docs/es/troubleshooting).

<h2 id="check-common-causes">
  Verificar causas comunes
</h2>

La mayoría de las sorpresas de configuración se remontan a un pequeño conjunto de reglas de ubicación y sintaxis. Verifique estos antes de asumir un error:

| Síntoma                                                                          | Causa                                                                                                                                              | Solución                                                                                                                                                                                                                                                                                                            |
| :------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Hook nunca se dispara                                                            | `matcher` es una matriz JSON en lugar de una cadena                                                                                                | Use una cadena única con `\|` para coincidir con múltiples herramientas, por ejemplo `"Edit\|Write"`. Consulte [patrones de matcher](/docs/es/hooks#matcher-patterns).                                                                                                                                                   |
| Hook nunca se dispara                                                            | `matcher` usa `,` como separador en una versión anterior a v2.1.191                                                                                | Claude Code v2.1.191 o posterior trata `,` como un separador de lista como `\|`. Las versiones anteriores evalúan una coma como un carácter literal, por lo que `"Edit,Write"` no coincide con nada. Use `\|` en su lugar, o actualice Claude Code.                                                                 |
| Hook nunca se dispara                                                            | El valor `matcher` está en minúsculas, por ejemplo `"bash"`                                                                                        | La coincidencia distingue mayúsculas de minúsculas. Los nombres de herramientas están capitalizados: `Bash`, `Edit`, `Write`, `Read`.                                                                                                                                                                               |
| Hook nunca se dispara                                                            | Los hooks están definidos en un archivo independiente en lugar de en `settings.json`                                                               | No hay archivo de hooks independiente para la configuración del proyecto o del usuario. Defina hooks bajo la clave `"hooks"` en `settings.json`. Solo los [plugins](/docs/es/plugins-reference#hooks) cargan un archivo `hooks/hooks.json` separado. Consulte [configuración de hook](/docs/es/hooks).                        |
| Los permisos, hooks o env establecidos globalmente se ignoran                    | La configuración se agregó a `~/.claude.json`                                                                                                      | `~/.claude.json` contiene el estado de la aplicación y los cambios de interfaz de usuario. `permissions`, `hooks` y `env` pertenecen a `~/.claude/settings.json`. Estos son dos archivos diferentes.                                                                                                                |
| Un valor `settings.json` parece ignorado                                         | La misma clave se establece en `settings.local.json`                                                                                               | `settings.local.json` anula `settings.json`, y ambos anulan `~/.claude/settings.json`. Consulte [precedencia de configuración](/docs/es/settings#how-scopes-interact).                                                                                                                                                   |
| Skill no aparece en `/skills`                                                    | El archivo de skill está en `.claude/skills/name.md` en lugar de en una carpeta                                                                    | Use una carpeta con `SKILL.md` dentro: `.claude/skills/name/SKILL.md`.                                                                                                                                                                                                                                              |
| Skill aparece en `/skills` pero Claude nunca lo invoca                           | Skill tiene `disable-model-invocation: true` en su frontmatter, o su descripción no coincide con cómo formula la solicitud                         | Verifique la insignia en `/skills`: una etiqueta "user-only" significa que Claude no lo activará por su cuenta. Consulte [invocación de skill](/docs/es/skills).                                                                                                                                                         |
| Las instrucciones de `CLAUDE.md` del subdirectorio parecen ignoradas             | Los archivos del subdirectorio se cargan bajo demanda, no al inicio de la sesión                                                                   | Se cargan cuando Claude lee un archivo en ese directorio con la herramienta Read, no al lanzar y no al escribir o crear archivos allí. Consulte [cómo se cargan los archivos CLAUDE.md](/docs/es/memory#how-claude-md-files-load).                                                                                       |
| El subagente ignora las instrucciones de `CLAUDE.md`                             | Los agentes Explore y Plan integrados omiten `CLAUDE.md`. Los subagentes personalizados lo cargan de la misma manera que la conversación principal | Para Explore o Plan, restate la instrucción en su indicador delegado. Para un subagente personalizado, coloque las instrucciones críticas en el cuerpo del archivo del agente, que se convierte en el indicador del sistema del subagente. Consulte [qué se carga al inicio](/docs/es/sub-agents#what-loads-at-startup). |
| La lógica de limpieza nunca se ejecuta al final de la sesión                     | No hay hook `SessionEnd` configurado                                                                                                               | Agregue un hook `SessionEnd` en `settings.json`. Consulte la [lista de eventos de hook](/docs/es/hooks#hook-events).                                                                                                                                                                                                     |
| Los servidores MCP en `.mcp.json` nunca se cargan                                | El archivo está bajo `.claude/` o usa el formato de configuración de Claude Desktop                                                                | La configuración de MCP del proyecto va en la raíz del repositorio como `.mcp.json`, no dentro de `.claude/`. Consulte [configuración de MCP](/docs/es/mcp).                                                                                                                                                             |
| Los servidores MCP agregados bajo `mcpServers` en `settings.json` nunca aparecen | `settings.json` no lee una clave `mcpServers`                                                                                                      | Defina servidores de proyecto en `.mcp.json` en la raíz del repositorio, o ejecute `claude mcp add --scope user` para servidores con ámbito de usuario. Consulte [configuración de MCP](/docs/es/mcp).                                                                                                                   |
| El servidor MCP del proyecto agregado pero no aparece                            | Se descartó el mensaje de aprobación única                                                                                                         | Los servidores con ámbito de proyecto requieren aprobación. Ejecute `/mcp` para ver el estado y aprobar.                                                                                                                                                                                                            |
| El servidor MCP no se inicia desde algunos directorios                           | `command` o `args` usa una ruta de archivo relativa                                                                                                | Use rutas absolutas para scripts locales. Los ejecutables en su `PATH` como `npx` o `uvx` funcionan tal cual.                                                                                                                                                                                                       |
| El servidor MCP se inicia sin las variables de entorno esperadas                 | Las variables están en `settings.json` `env`, que no se propaga a procesos secundarios de MCP                                                      | Establezca `env` por servidor dentro de `.mcp.json` en su lugar.                                                                                                                                                                                                                                                    |
| La regla de denegación `Bash(rm *)` no bloquea `/bin/rm` o `find -delete`        | Las reglas de prefijo coinciden con la cadena de comando literal, no con el ejecutable subyacente                                                  | Agregue patrones explícitos para cada variante, o use un [hook PreToolUse](/docs/es/hooks-guide) o el [sandbox](/docs/es/sandboxing) para una garantía dura.                                                                                                                                                                  |

<h2 id="related-resources">
  Recursos relacionados
</h2>

Para una referencia completa en cada superficie de configuración, consulte la página dedicada:

* **[Referencia del directorio `.claude`](/docs/es/claude-directory)**: cada ubicación de archivo de configuración y qué lo lee
* **[Configuración](/docs/es/settings)**: orden de precedencia y la lista completa de claves
* **[Referencia de hooks](/docs/es/hooks)**: nombres de eventos, cargas útiles y formato de salida `--debug hooks`
* **[MCP](/docs/es/mcp)**: configuración del servidor, aprobación y salida `/mcp`
* **[Solucionar problemas de instalación e inicio de sesión](/docs/es/troubleshoot-install)**: `comando no encontrado`, PATH y problemas de autenticación
* **[Solución de problemas](/docs/es/troubleshooting)**: rendimiento, bloqueos y problemas de búsqueda
