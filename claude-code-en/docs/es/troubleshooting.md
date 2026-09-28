> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Solución de problemas

> Corrige el alto uso de CPU o memoria, cuelgues, thrashing de auto-compact, y problemas de búsqueda en Claude Code, y encuentra la página correcta para otros problemas.

Esta página cubre problemas de rendimiento, estabilidad y búsqueda una vez que Claude Code está en ejecución. Para otros problemas, comienza con la página que coincida con dónde estés atrapado:

| Síntoma                                                                                                                                                                         | Ir a                                                                                                        |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------- |
| `command not found`, falla de instalación, problemas de PATH, `EACCES`, errores de TLS                                                                                          | [Solucionar problemas de instalación e inicio de sesión](/docs/es/troubleshoot-install)                          |
| Actualización o falla de descarga de instalación con `The connection dropped while downloading the update` o `aborted`                                                          | [Referencia de errores](/docs/es/errors#the-connection-dropped-while-downloading-the-update)                     |
| Bucles de inicio de sesión, errores de OAuth, `403 Forbidden`, "organización deshabilitada", credenciales de Amazon Bedrock, Google Cloud's Agent Platform, o Microsoft Foundry | [Solucionar problemas de instalación e inicio de sesión](/docs/es/troubleshoot-install#login-and-authentication) |
| La configuración no se aplica, hooks no se disparan, servidores MCP no se cargan                                                                                                | [Depurar tu configuración](/docs/es/debug-your-config)                                                           |
| Sesión iniciada en modo automático, o Claude edita archivos y ejecuta comandos sin preguntar                                                                                    | [En qué modo se inicia una sesión](/docs/es/permission-modes#which-mode-a-session-starts-in)                     |
| `API Error: 5xx`, `529 Overloaded`, `429`, errores de validación de solicitudes                                                                                                 | [Referencia de errores](/docs/es/errors)                                                                         |
| `model not found` o `you may not have access to it`                                                                                                                             | [Referencia de errores](/docs/es/errors#theres-an-issue-with-the-selected-model)                                 |
| La extensión de VS Code no se conecta o no detecta Claude                                                                                                                       | [Integración de VS Code](/docs/es/vs-code#fix-common-issues)                                                     |
| `Claude Code process exited with code 1` en VS Code o una aplicación SDK                                                                                                        | [Referencia de errores](/docs/es/errors#claude-code-process-exited-with-code-n)                                  |
| Plugin de JetBrains o IDE no detectado                                                                                                                                          | [Integración de JetBrains](/docs/es/jetbrains#troubleshooting)                                                   |
| Alto uso de CPU o memoria, respuestas lentas, cuelgues, búsqueda no encuentra archivos                                                                                          | [Rendimiento y estabilidad](#performance-and-stability) abajo                                               |

Si no estás seguro de cuál aplica, ejecuta `/doctor` dentro de Claude Code para una verificación automatizada de tu instalación, configuración, extensiones, y uso de contexto; propone correcciones que puede aplicar después de que confirmes. Si `claude` no inicia en absoluto, ejecuta `claude doctor` desde tu shell en su lugar. Ejecuta `/mcp` para verificar el estado del servidor MCP.

<h2 id="performance-and-stability">
  Rendimiento y estabilidad
</h2>

Estas secciones cubren problemas relacionados con el uso de recursos, capacidad de respuesta, y comportamiento de búsqueda.

<h3 id="high-cpu-or-memory-usage">
  Alto uso de CPU o memoria
</h3>

Claude Code está diseñado para funcionar con la mayoría de entornos de desarrollo, pero puede consumir recursos significativos al procesar bases de código grandes. Si está experimentando problemas de rendimiento:

1. Utilice `/compact` regularmente para reducir el tamaño del contexto. Si devuelve `Not enough messages to compact.`, la conversación tiene muy pocos turnos para resumir; eso puede suceder incluso con un contexto completo cuando un único pegado grande lo llenó
2. Cierre y reinicie Claude Code entre tareas principales
3. Considere añadir directorios de compilación grandes a su archivo `.gitignore`
4. Reinicie con [`claude --safe-mode`](/docs/es/cli-reference#cli-flags) para verificar si un plugin, servidor MCP, o hook es la fuente. Desactiva todas las personalizaciones para la sesión; si el uso disminuye, consulte [Depurar su configuración](/docs/es/debug-your-config#test-against-a-clean-configuration) para encontrar cuál es

Si la memoria de sesión supera 2.5GB, aparece una advertencia crítica de uso de memoria. Para liberar la memoria, reinicie Claude Code y ejecute [`claude --continue`](/docs/es/cli-reference#cli-flags) para reanudar la conversación en un proceso nuevo.

Fuera de [renderizado a pantalla completa](/docs/es/fullscreen), ejecutar `/compact` también libera memoria. La advertencia desaparece una vez que el uso de memoria cae por debajo de 2.5GB.

Si el uso de memoria se mantiene alto después de estos pasos, ejecute `/heapdump` para escribir dos archivos a `~/Desktop`: una instantánea de montón de JavaScript denominada `<session-id>.heapsnapshot` y un desglose de memoria denominado `<session-id>-diagnostics.json`. Claude Code [oculta el comando del menú de comandos](/docs/es/commands#how-the-command-menu-matches-what-you-type); escriba el comando completo. En Linux sin una carpeta Desktop, los archivos se escriben en su directorio de inicio.

<Warning>
  El archivo `.heapsnapshot` contiene cada cadena en el proceso, incluyendo su conversación completa y credenciales. No lo adjunte a un problema público ni lo comparta.
</Warning>

El comando también imprime un resumen en la conversación, mostrando el tamaño del conjunto residente, montón de JS, búferes de matriz, y memoria nativa no contabilizada, más cualquier indicador de fuga que detecte, como una tasa de crecimiento de memoria alta o un número inusualmente alto de identificadores abiertos. El resumen indica si la mayoría de la memoria está en el montón de JS, que la instantánea captura, o en memoria nativa, que no lo hace.

Haga una de dos cosas con la salida:

* **Infórmelo**: abra un [problema de GitHub](https://github.com/anthropics/claude-code/issues) y adjunte solo el archivo `-diagnostics.json`, que contiene las estadísticas detrás del resumen impreso y ningún contenido de conversación ni credenciales
* **Investíguelo usted mismo**: si el resumen dice que la mayoría de la memoria es montón de JS, abra el archivo `.heapsnapshot` en Chrome DevTools en Memory → Load y ordene por tamaño retenido para ver qué está reteniendo la memoria

Si el resumen dice que la mayoría de la memoria es nativa, la instantánea no puede mostrarlo; incluya los indicadores de fuga del resumen en su informe en su lugar.

<h3 id="large-tables-are-cut-off-in-the-terminal">
  Las tablas grandes se cortan en la terminal
</h3>

Una tabla Markdown con más de 200 filas renderiza sus primeras 200 filas seguidas de una línea `… N more rows not shown`. Solo la visualización está limitada: la tabla completa permanece en la conversación, y [`/copy`](/docs/es/commands) copia cada fila. Para una tabla demasiado grande para leer en la terminal, pida a Claude que la escriba en un archivo en su lugar. Antes de v2.1.208, Claude Code renderizaba cada fila, por lo que reanudar una sesión que contenía una tabla muy grande podría estancarse mientras se re-renderizaba.

<h3 id="auto-compaction-stops-with-a-thrashing-error">
  Auto-compaction se detiene con un error de thrashing
</h3>

Si ve `Autocompact is thrashing: the context refilled to the limit...`, la compactación automática fue exitosa pero un archivo o salida de herramienta rellenó inmediatamente la ventana de contexto varias veces seguidas. Claude Code deja de reintentar para evitar desperdiciar llamadas de API en un bucle que no está haciendo progreso.

Para recuperarse:

1. Pida a Claude que lea el archivo de gran tamaño en fragmentos más pequeños, como un rango de línea específico o función, en lugar de todo el archivo
2. Ejecute `/compact` con un enfoque que elimine la salida grande, por ejemplo `/compact keep only the plan and the diff`
3. Mueva el trabajo de archivo grande a un [subagente](/docs/es/sub-agents) para que se ejecute en una ventana de contexto separada
4. Ejecute `/clear` si la conversación anterior ya no es necesaria

<h3 id="command-hangs-or-freezes">
  El comando se cuelga o congela
</h3>

Si Claude Code parece no responder:

1. Presione Ctrl+C para intentar cancelar la operación actual
2. Si no responde, es posible que necesite cerrar la terminal y reiniciar

Reiniciar no pierde su conversación. Ejecute `claude --resume` en el mismo directorio para retomar la sesión.

<h3 id="garbled-or-corrupted-text-in-an-editor’s-integrated-terminal">
  Texto garbled o corrupto en la terminal integrada de un editor
</h3>

Si los caracteres se renderizan como cuadros, manchas, o glifos incorrectos al ejecutar Claude Code en la terminal integrada de VS Code, Cursor, o Devin Desktop, el renderizador GPU de la terminal es probablemente la causa. Ejecute `/terminal-setup` dentro de Claude Code para establecer `terminal.integrated.gpuAcceleration` a `"off"`, o establézcalo manualmente en la configuración de su editor y recargue la ventana. Consulte [Configuración de terminal](/docs/es/terminal-config) para las otras configuraciones que `/terminal-setup` escribe.

<h3 id="mouse-wheel-scrolls-one-line-at-a-time-in-fullscreen-rendering">
  La rueda del ratón se desplaza una línea a la vez en renderizado a pantalla completa
</h3>

En [renderizado a pantalla completa](/docs/es/fullscreen), Claude Code desplaza la conversación en sí en lugar de dejarlo a su terminal. Si cada muesca de rueda mueve menos líneas de las que desea, ejecute `/scroll-speed` para aumentar el número de líneas por muesca y guárdelo, o establezca la variable de entorno `CLAUDE_CODE_SCROLL_SPEED`, excepto en la terminal del IDE JetBrains, donde Claude Code aplica su propio manejo de desplazamiento y ninguno de los dos tiene efecto. Consulte [Desplazamiento de rueda del ratón](/docs/es/fullscreen#mouse-wheel-scrolling) para los valores que cada uno acepta.

Para moverse más rápido sin cambiar la velocidad, presione `PgUp` y `PgDn` para desplazarse media pantalla a la vez. Para usar el desplazamiento nativo de su terminal en su lugar, ejecute `/tui default` para cambiar al renderizador clásico.

<h3 id="clipboard-commands-such-as-pbcopy-fail-inside-the-sandbox">
  Los comandos del portapapeles como `pbcopy` fallan dentro del sandbox
</h3>

Cuando [sandboxing](/docs/es/sandboxing) está activado, las utilidades del portapapeles como `pbcopy`, `xclip`, y `wl-copy` pueden fallar al alcanzar el portapapeles del sistema desde dentro de un comando Bash en sandbox, dejando su portapapeles sin cambios después de que Claude canaliza texto a ellos.

Para poner la salida de Claude en su portapapeles, pida a Claude que imprima el contenido en su respuesta, luego ejecute [`/copy`](/docs/es/commands). `/copy` escribe en el portapapeles desde el proceso de Claude Code en sí en lugar de desde un comando en sandbox, por lo que el sandboxing no lo bloquea. Puede copiar un único bloque de código en lugar de toda la respuesta, y también escribe lo que copió en un archivo e imprime la ruta, lo que le da una alternativa cuando la escritura del portapapeles no alcanza su terminal, por ejemplo sobre SSH.

Cuando Claude canaliza texto a una de estas herramientas, añadir `pbcopy *`, `wl-copy *`, o `xclip *` a [`excludedCommands`](/docs/es/settings-reference#sandbox-excludedcommands) no saca esa llamada del sandbox por sí sola.

<h3 id="copied-text-doesn’t-reach-your-local-clipboard-over-ssh">
  Texto copiado no llega a su portapapeles local sobre SSH
</h3>

Cuando Claude Code se ejecuta en una máquina remota sobre SSH, no puede ejecutar una herramienta de portapapeles en su máquina local. Fuera de tmux, cuando selecciona texto en [renderizado a pantalla completa](/docs/es/fullscreen) o ejecuta `/copy`, Claude Code envía el texto a su terminal como una secuencia de escape OSC 52 en su lugar. Su terminal decide si lo pone en su portapapeles. `/copy` informa `Copied to clipboard` independientemente de si el texto llegó, y fuera de tmux el aviso de selección lee `sent N chars via OSC 52`.

Algunos terminales no actúan sobre OSC 52. iTerm2 lo ignora hasta que activa **Settings > General > Selection > Applications in terminal may access clipboard**, y macOS Terminal.app no lo admite.

Para obtener el texto sin OSC 52:

* Mantenga presionada la tecla de selección nativa de su terminal mientras arrastra, luego copie con el atajo habitual de su terminal, como `Cmd+C`. La tecla es `Fn` en Terminal.app e `Option` en iTerm2. [Mantener selección de texto nativa](/docs/es/fullscreen#keep-native-text-selection) la enumera para otros terminales.
* Establezca [`CLAUDE_CODE_DISABLE_MOUSE=1`](/docs/es/env-vars) en la máquina remota para que su terminal maneje la selección para toda la sesión.

<h3 id="search-and-discovery-issues">
  Problemas de búsqueda y descubrimiento
</h3>

Si la herramienta Search, menciones `@file`, agentes personalizados, o skills personalizados no encuentran archivos, el binario `ripgrep` incluido puede no ejecutarse en su sistema. Instale el paquete `ripgrep` de su plataforma e indique a Claude Code que lo use en su lugar:

<Tabs>
  <Tab title="macOS">
    ```bash theme={null}
    brew install ripgrep
    ```
  </Tab>

  <Tab title="Ubuntu/Debian">
    ```bash theme={null}
    sudo apt install ripgrep
    ```
  </Tab>

  <Tab title="Alpine">
    ```bash theme={null}
    apk add ripgrep
    ```

    `ripgrep` está en el repositorio de comunidad de Alpine. Si `apk` informa que el paquete falta, consulte [Configuración de Alpine Linux](/docs/es/setup#alpine-linux-and-musl-based-distributions).
  </Tab>

  <Tab title="Arch">
    ```bash theme={null}
    pacman -S ripgrep
    ```
  </Tab>

  <Tab title="Windows">
    ```powershell theme={null}
    winget install BurntSushi.ripgrep.MSVC
    ```
  </Tab>
</Tabs>

Luego establezca `USE_BUILTIN_RIPGREP` a `0`, ya sea en su [entorno](/docs/es/env-vars) de shell o en el bloque `env` de su [`settings.json`](/docs/es/settings-reference#all-settings):

```json theme={null}
{
  "env": {
    "USE_BUILTIN_RIPGREP": "0"
  }
}
```

Para confirmar que el cambio tuvo efecto, ejecute `claude doctor` en su terminal y verifique que la línea Search muestre la ruta de su ripgrep del sistema en lugar de `OK (bundled)`.

<h3 id="slow-or-incomplete-search-results-on-wsl">
  Resultados de búsqueda lentos o incompletos en WSL
</h3>

Las penalizaciones de rendimiento de lectura de disco al [trabajar entre sistemas de archivos en WSL](https://learn.microsoft.com/en-us/windows/wsl/filesystems) pueden resultar en menos coincidencias de las esperadas al usar Claude Code en WSL. La búsqueda aún funciona, pero devuelve menos resultados que en un sistema de archivos nativo.

<Note>
  `claude doctor` muestra Search como OK en este caso.
</Note>

**Soluciones:**

1. **Envíe búsquedas más específicas**: reduzca el número de archivos buscados especificando directorios o tipos de archivo: "Search for JWT validation logic in the auth-service package" o "Find use of md5 hash in JS files".

2. **Mueva el proyecto al sistema de archivos de Linux**: si es posible, asegúrese de que su proyecto esté ubicado en el sistema de archivos de Linux (`/home/`) en lugar del sistema de archivos de Windows (`/mnt/c/`).

3. **Utilice Windows nativo en su lugar**: considere ejecutar Claude Code nativamente en Windows en lugar de a través de WSL, para mejor rendimiento del sistema de archivos.

<h2 id="get-more-help">
  Obtén más ayuda
</h2>

Si estás experimentando problemas no cubiertos aquí:

1. Ejecuta `/doctor` para una verificación de configuración y `/mcp` para verificar el estado del servidor MCP
2. Usa el comando `/feedback` dentro de Claude Code para reportar problemas directamente a Anthropic
3. Verifica el [repositorio de GitHub](https://github.com/anthropics/claude-code) para problemas conocidos
4. Pregunta a Claude directamente sobre sus capacidades y características. Claude tiene acceso integrado a su documentación.

Para problemas de cuenta, facturación o suscripción, contacta al soporte de Anthropic en su lugar: inicia sesión en [claude.ai](https://claude.ai) (Usuarios de Console: [platform.claude.com](https://platform.claude.com)), haz clic en tus iniciales en la esquina inferior izquierda y selecciona **Obtener ayuda**. Consulta [Cómo obtener soporte](https://support.claude.com/en/articles/9015913-how-to-get-support) para el flujo completo, incluyendo quién puede comunicarse con un agente humano en cada plan.
