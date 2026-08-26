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
| `API Error: 5xx`, `529 Overloaded`, `429`, errores de validación de solicitudes                                                                                                 | [Referencia de errores](/docs/es/errors)                                                                         |
| `model not found` o `you may not have access to it`                                                                                                                             | [Referencia de errores](/docs/es/errors#theres-an-issue-with-the-selected-model)                                 |
| La extensión de VS Code no se conecta o no detecta Claude                                                                                                                       | [Integración de VS Code](/docs/es/vs-code#fix-common-issues)                                                     |
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

1. Utilice `/compact` regularmente para reducir el tamaño del contexto
2. Cierre y reinicie Claude Code entre tareas principales
3. Considere añadir directorios de compilación grandes a su archivo `.gitignore`
4. Reinicie con [`claude --safe-mode`](/docs/es/cli-reference#cli-flags) para verificar si un plugin, servidor MCP, o hook es la fuente. Desactiva todas las personalizaciones para la sesión; si el uso disminuye, consulte [Depurar su configuración](/docs/es/debug-your-config#test-against-a-clean-configuration) para encontrar cuál es

Si el uso de memoria se mantiene alto después de estos pasos, ejecute `/heapdump` para escribir una instantánea de montón de JavaScript y un desglose de memoria a `~/Desktop`. En Linux sin una carpeta Desktop, los archivos se escriben en su directorio de inicio.

El desglose muestra el tamaño del conjunto residente, el montón de JS, los búferes de matriz y la memoria nativa no contabilizada, lo que ayuda a identificar si el crecimiento está en objetos de JavaScript o en código nativo. Para inspeccionar los retenedores, abra el archivo `.heapsnapshot` en Chrome DevTools en Memory → Load; el desglose es el archivo que termina en `-diagnostics.json`.

<Warning>
  El archivo `.heapsnapshot` contiene cada cadena en el proceso. No lo adjunte a un problema público ni lo comparta. Adjunte solo el archivo `-diagnostics.json` al informar un problema de memoria en [GitHub](https://github.com/anthropics/claude-code/issues). Ese archivo contiene estadísticas de memoria y ningún contenido de conversación ni credenciales.
</Warning>

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

Luego establezca `USE_BUILTIN_RIPGREP=0` en su [entorno](/docs/es/env-vars).

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
