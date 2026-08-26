> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Flujos de trabajo comunes

> Guías paso a paso para explorar bases de código, corregir errores, refactorizar, probar y otras tareas cotidianas con Claude Code.

Esta página recopila recetas cortas para el desarrollo cotidiano. Para orientación de nivel superior sobre indicaciones y gestión de contexto, consulte [Mejores prácticas](/docs/es/best-practices).

Esta página cubre:

* [Recetas de indicaciones](#prompt-recipes) para explorar código, corregir errores, refactorizar, probar, PRs y documentación
* [Reanudar conversaciones anteriores](#resume-previous-conversations) para que una tarea pueda abarcar múltiples sesiones
* [Ejecutar sesiones paralelas con worktrees](#run-parallel-sessions-with-worktrees) para que las ediciones concurrentes no choquen
* [Planificar antes de editar](#plan-before-editing) para revisar cambios antes de que toquen el disco
* [Delegar investigación a subagentes](#delegate-research-to-subagents) para mantener su contexto principal limpio
* [Canalizar Claude en scripts](#pipe-claude-into-scripts) para CI y procesamiento por lotes

<h2 id="prompt-recipes">
  Recetas de indicaciones
</h2>

Estos son patrones de indicaciones para tareas cotidianas como explorar código desconocido, depuración, refactorización, escritura de pruebas y creación de PRs. Cada uno funciona en cualquier superficie de Claude Code; adapte la redacción a su proyecto.

<h3 id="understand-new-codebases">
  Comprender nuevas bases de código
</h3>

Para configurar Claude Code en un monorepo o base de código grande, consulte [Monorepos y repositorios grandes](/docs/es/large-codebases).

<h4 id="get-a-quick-codebase-overview">
  Obtener una descripción general rápida de la base de código
</h4>

Supongamos que acaba de unirse a un nuevo proyecto y necesita comprender su estructura rápidamente.

<Steps>
  <Step title="Navegue al directorio raíz del proyecto">
    ```bash theme={null}
    cd /path/to/project 
    ```
  </Step>

  <Step title="Inicie Claude Code">
    ```bash theme={null}
    claude 
    ```
  </Step>

  <Step title="Solicite una descripción general de alto nivel">
    ```text theme={null}
    dame una descripción general de esta base de código
    ```
  </Step>

  <Step title="Profundice en componentes específicos">
    ```text theme={null}
    explica los patrones de arquitectura principales utilizados aquí
    ```

    ```text theme={null}
    ¿cuáles son los modelos de datos clave?
    ```

    ```text theme={null}
    ¿cómo se maneja la autenticación?
    ```
  </Step>
</Steps>

<Tip>
  Consejos:

  * Comience con preguntas amplias, luego reduzca a áreas específicas
  * Pregunte sobre convenciones de codificación y patrones utilizados en el proyecto
  * Solicite un glosario de términos específicos del proyecto
</Tip>

<h4 id="find-relevant-code">
  Encontrar código relevante
</h4>

Supongamos que necesita localizar código relacionado con una característica o funcionalidad específica.

<Steps>
  <Step title="Pida a Claude que encuentre archivos relevantes">
    ```text theme={null}
    encuentra los archivos que manejan la autenticación de usuarios
    ```
  </Step>

  <Step title="Obtenga contexto sobre cómo interactúan los componentes">
    ```text theme={null}
    ¿cómo funcionan juntos estos archivos de autenticación?
    ```
  </Step>

  <Step title="Comprenda el flujo de ejecución">
    ```text theme={null}
    rastrear el proceso de inicio de sesión de front-end a base de datos
    ```
  </Step>
</Steps>

<Tip>
  Consejos:

  * Sea específico sobre lo que está buscando
  * Utilice el lenguaje del dominio del proyecto
  * Instale un [plugin de inteligencia de código](/docs/es/discover-plugins#code-intelligence) para su lenguaje para dar a Claude una navegación precisa de "ir a definición" y "buscar referencias"
</Tip>

***

<h3 id="fix-bugs-efficiently">
  Corregir errores de manera eficiente
</h3>

Supongamos que ha encontrado un mensaje de error y necesita encontrar y corregir su origen.

<Steps>
  <Step title="Comparta el error con Claude">
    ```text theme={null}
    estoy viendo un error cuando ejecuto npm test
    ```
  </Step>

  <Step title="Solicite recomendaciones de corrección">
    ```text theme={null}
    sugiere algunas formas de corregir el @ts-ignore en user.ts
    ```
  </Step>

  <Step title="Aplique la corrección">
    ```text theme={null}
    actualiza user.ts para agregar la verificación nula que sugeriste
    ```
  </Step>
</Steps>

<Tip>
  Consejos:

  * Dígale a Claude el comando para reproducir el problema y obtener un seguimiento de pila
  * Mencione cualquier paso para reproducir el error
  * Hágale saber a Claude si el error es intermitente o consistente
</Tip>

***

<h3 id="refactor-code">
  Refactorizar código
</h3>

Supongamos que necesita actualizar código antiguo para utilizar patrones y prácticas modernas.

<Steps>
  <Step title="Identifique código heredado para refactorización">
    ```text theme={null}
    encuentra el uso de API obsoleta en nuestra base de código
    ```
  </Step>

  <Step title="Obtenga recomendaciones de refactorización">
    ```text theme={null}
    sugiere cómo refactorizar utils.js para usar características modernas de JavaScript
    ```
  </Step>

  <Step title="Aplique los cambios de manera segura">
    ```text theme={null}
    refactoriza utils.js para usar características de ES2024 manteniendo el mismo comportamiento
    ```
  </Step>

  <Step title="Verifique la refactorización">
    ```text theme={null}
    ejecuta pruebas para el código refactorizado
    ```
  </Step>
</Steps>

<Tip>
  Consejos:

  * Pida a Claude que explique los beneficios del enfoque moderno
  * Solicite que los cambios mantengan la compatibilidad hacia atrás cuando sea necesario
  * Realice la refactorización en incrementos pequeños y comprobables
</Tip>

***

<h3 id="work-with-tests">
  Trabajar con pruebas
</h3>

Supongamos que necesita agregar pruebas para código no cubierto.

<Steps>
  <Step title="Identifique código no probado">
    ```text theme={null}
    encuentra funciones en NotificationsService.swift que no están cubiertas por pruebas
    ```
  </Step>

  <Step title="Genere andamiaje de prueba">
    ```text theme={null}
    agrega pruebas para el servicio de notificaciones
    ```
  </Step>

  <Step title="Agregue casos de prueba significativos">
    ```text theme={null}
    agrega casos de prueba para condiciones de borde en el servicio de notificaciones
    ```
  </Step>

  <Step title="Ejecute y verifique las pruebas">
    ```text theme={null}
    ejecuta las nuevas pruebas y corrige cualquier fallo
    ```
  </Step>
</Steps>

Claude puede generar pruebas que sigan los patrones y convenciones existentes de su proyecto. Al solicitar pruebas, sea específico sobre qué comportamiento desea verificar. Claude examina sus archivos de prueba existentes para coincidir con el estilo, marcos y patrones de afirmación ya en uso.

Para una cobertura integral, pida a Claude que identifique casos extremos que podría haber perdido. Claude puede analizar sus rutas de código y sugerir pruebas para condiciones de error, valores límite e entradas inesperadas que son fáciles de pasar por alto.

***

<h3 id="create-pull-requests">
  Crear solicitudes de extracción
</h3>

Puede crear solicitudes de extracción pidiendo a Claude directamente ("crear una pr para mis cambios"), o guiar a Claude a través de ella paso a paso:

<Steps>
  <Step title="Resuma sus cambios">
    ```text theme={null}
    resume los cambios que he hecho en el módulo de autenticación
    ```
  </Step>

  <Step title="Genere una solicitud de extracción">
    ```text theme={null}
    crear una pr
    ```
  </Step>

  <Step title="Revise y refine">
    ```text theme={null}
    mejora la descripción de la PR con más contexto sobre las mejoras de seguridad
    ```
  </Step>
</Steps>

Cuando crea una PR usando `gh pr create`, la sesión se vincula automáticamente a esa PR. Para regresar a ella más tarde, ejecute `claude --from-pr 123`, reemplazando 123 con el número de PR, o pegue la URL de la PR en el selector [`/resume`](/docs/es/sessions#use-the-session-picker).

<Tip>
  Revise la PR generada por Claude antes de enviarla y pida a Claude que destaque los riesgos potenciales o consideraciones.
</Tip>

<h3 id="handle-documentation">
  Manejar documentación
</h3>

Supongamos que necesita agregar o actualizar documentación para su código.

<Steps>
  <Step title="Identifique código sin documentar">
    ```text theme={null}
    encuentra funciones sin comentarios JSDoc adecuados en el módulo de autenticación
    ```
  </Step>

  <Step title="Genere documentación">
    ```text theme={null}
    agrega comentarios JSDoc a las funciones sin documentar en auth.js
    ```
  </Step>

  <Step title="Revise y mejore">
    ```text theme={null}
    mejora la documentación generada con más contexto y ejemplos
    ```
  </Step>

  <Step title="Verifique la documentación">
    ```text theme={null}
    verifica si la documentación sigue nuestros estándares de proyecto
    ```
  </Step>
</Steps>

<Tip>
  Consejos:

  * Especifique el estilo de documentación que desea (JSDoc, docstrings, etc.)
  * Solicite ejemplos en la documentación
  * Solicite documentación para API públicas, interfaces y lógica compleja
</Tip>

***

<h3 id="work-in-notes-and-non-code-folders">
  Trabajar en notas y carpetas que no son código
</h3>

Claude Code funciona en cualquier directorio. Ejecútelo dentro de una bóveda de notas, una carpeta de documentación o cualquier colección de archivos markdown para buscar, editar y reorganizar contenido de la misma manera que lo haría con código.

El directorio `.claude/` y `CLAUDE.md` se encuentran junto a los directorios de configuración de otras herramientas sin conflicto. Claude lee archivos nuevos en cada llamada de herramienta, por lo que ve las ediciones que realiza en otra aplicación la próxima vez que lee ese archivo.

***

<h3 id="work-with-images">
  Trabajar con imágenes
</h3>

Supongamos que necesita trabajar con imágenes en su base de código y desea la ayuda de Claude para analizar el contenido de la imagen.

<Steps>
  <Step title="Agregue una imagen a la conversación">
    Puede usar cualquiera de estos métodos:

    1. Arrastre y suelte una imagen en la ventana de Claude Code
    2. Copie una imagen y péguela en la CLI con Ctrl+V. En macOS, Cmd+V también funciona en iTerm2.
    3. Proporcione una ruta de imagen a Claude. Por ejemplo, "Analiza esta imagen: /path/to/your/image.png"
  </Step>

  <Step title="Pida a Claude que analice la imagen">
    ```text theme={null}
    ¿Qué muestra esta imagen?
    ```

    ```text theme={null}
    Describe los elementos de la interfaz de usuario en esta captura de pantalla
    ```

    ```text theme={null}
    ¿Hay algún elemento problemático en este diagrama?
    ```
  </Step>

  <Step title="Usar imágenes para contexto">
    ```text theme={null}
    Aquí hay una captura de pantalla del error. ¿Qué lo está causando?
    ```

    ```text theme={null}
    Este es nuestro esquema de base de datos actual. ¿Cómo deberíamos modificarlo para la nueva característica?
    ```
  </Step>

  <Step title="Obtenga sugerencias de código del contenido visual">
    ```text theme={null}
    Generar CSS para coincidir con este mockup de diseño
    ```

    ```text theme={null}
    ¿Qué estructura HTML recrearía este componente?
    ```
  </Step>
</Steps>

<Tip>
  Consejos:

  * Use imágenes cuando las descripciones de texto serían poco claras o engorrosas
  * Incluya capturas de pantalla de errores, diseños de interfaz de usuario o diagramas para mejor contexto
  * Puede trabajar con múltiples imágenes en una conversación
  * El análisis de imágenes funciona con diagramas, capturas de pantalla, mockups y más
  * Cuando Claude hace referencia a imágenes (por ejemplo, `[Image #1]`), `Cmd+Click` (Mac) o `Ctrl+Click` (Windows/Linux) el enlace para abrir la imagen en su visor predeterminado
</Tip>

***

<h3 id="reference-files-and-directories">
  Archivos y directorios de referencia
</h3>

Use @ para incluir rápidamente archivos o directorios sin esperar a que Claude los lea.

<Steps>
  <Step title="Haga referencia a un archivo único">
    ```text theme={null}
    Explica la lógica en @src/utils/auth.js
    ```

    Esto incluye el contenido completo del archivo en la conversación.
  </Step>

  <Step title="Haga referencia a un directorio">
    ```text theme={null}
    ¿Cuál es la estructura de @src/components?
    ```

    Esto proporciona un listado de directorio con información de archivo.
  </Step>

  <Step title="Haga referencia a recursos MCP">
    ```text theme={null}
    Muéstrame los datos de @github:repos/owner/repo/issues
    ```

    Esto obtiene datos de servidores MCP conectados usando el formato @server:resource. Consulte [recursos MCP](/docs/es/mcp#use-mcp-resources) para más detalles.
  </Step>
</Steps>

<Tip>
  Consejos:

  * Las rutas de archivo pueden ser relativas o absolutas
  * Las referencias de archivo @ agregan `CLAUDE.md` en el directorio del archivo y directorios principales al contexto
  * Las referencias de directorio muestran listados de archivos, no contenidos
  * Puede hacer referencia a múltiples archivos en un solo mensaje (por ejemplo, "@file1.js y @file2.js")
</Tip>

***

<h3 id="run-claude-on-a-schedule">
  Ejecutar Claude en un horario
</h3>

Supongamos que desea que Claude maneje una tarea automáticamente de forma recurrente, como revisar PRs abiertas cada mañana, auditar dependencias semanalmente o verificar fallas de CI durante la noche.

Elija una opción de programación según dónde desee que se ejecute la tarea:

| Opción                                                          | Dónde se ejecuta                                    | Mejor para                                                                                                                                                                                                                         |
| :-------------------------------------------------------------- | :-------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Routines](/docs/es/routines)                                        | Infraestructura administrada por Anthropic          | Tareas que deben ejecutarse incluso cuando su computadora está apagada. También puede activarse en llamadas API o eventos de GitHub además de un horario. Configure en [claude.ai/code/routines](https://claude.ai/code/routines). |
| [Tareas programadas de escritorio](/docs/es/desktop-scheduled-tasks) | Su máquina, a través de la aplicación de escritorio | Tareas que necesitan acceso directo a archivos locales, herramientas o cambios sin confirmar.                                                                                                                                      |
| [GitHub Actions](/docs/es/github-actions)                            | Su canalización de CI                               | Tareas vinculadas a eventos de repositorio como PRs abiertos, o horarios cron que deben vivir junto con su configuración de flujo de trabajo.                                                                                      |
| [`/loop`](/docs/es/scheduled-tasks)                                  | La sesión CLI actual                                | Sondeo rápido mientras una sesión está abierta. Las tareas se cancelan cuando comienza una nueva conversación; `--resume` y `--continue` restauran las no expiradas.                                                               |

<Tip>
  Al escribir indicaciones para tareas programadas, sea explícito sobre qué se ve como éxito y qué hacer con los resultados. La tarea se ejecuta de forma autónoma, por lo que no puede hacer preguntas aclaratorias. Por ejemplo: "Revise PRs abiertas etiquetadas con `needs-review`, deje comentarios en línea sobre cualquier problema y publique un resumen en el canal `#eng-reviews` de Slack."
</Tip>

***

<h3 id="ask-claude-about-its-capabilities">
  Pregunte a Claude sobre sus capacidades
</h3>

Claude tiene acceso integrado a su documentación y puede responder preguntas sobre sus propias características y limitaciones.

<h4 id="example-questions">
  Preguntas de ejemplo
</h4>

```text theme={null}
¿puede Claude Code crear solicitudes de extracción?
```

```text theme={null}
¿cómo maneja Claude Code los permisos?
```

```text theme={null}
¿qué skills están disponibles?
```

```text theme={null}
¿cómo uso MCP con Claude Code?
```

```text theme={null}
¿cómo configuro Claude Code para Amazon Bedrock?
```

```text theme={null}
¿cuáles son las limitaciones de Claude Code?
```

<Note>
  Claude proporciona respuestas basadas en documentación a estas preguntas. Para demostraciones prácticas, ejecute `/powerup` para lecciones interactivas con demostraciones animadas, o consulte las secciones de flujo de trabajo específicas anteriores.
</Note>

<Tip>
  Consejos:

  * Claude siempre tiene acceso a la documentación más reciente de Claude Code, independientemente de la versión que esté utilizando
  * Haga preguntas específicas para obtener respuestas detalladas
  * Claude puede explicar características complejas como integración MCP, configuraciones empresariales y flujos de trabajo avanzados
</Tip>

***

<h2 id="resume-previous-conversations">
  Reanudar conversaciones anteriores
</h2>

Cuando una tarea abarca múltiples sesiones, continúe donde lo dejó en lugar de volver a explicar el contexto. Claude Code guarda cada conversación localmente.

```bash theme={null}
claude --continue
```

Esto reanuda la sesión más reciente en el directorio actual; si aún no existe una, imprime `No conversation found to continue` y sale. Use `claude --resume` para elegir de una lista, o `/resume` desde dentro de una sesión en ejecución. Consulte [Gestionar sesiones](/docs/es/sessions) para nombrar, ramificar y la referencia completa del selector.

<h2 id="run-parallel-sessions-with-worktrees">
  Ejecutar sesiones paralelas con worktrees
</h2>

Trabaje en una característica en una terminal mientras Claude corrige un error en otra, sin que los cambios choquen. Cada worktree es un checkout separado en su propia rama.

```bash theme={null}
claude --worktree feature-auth
```

Ejecute el mismo comando con un nombre diferente en una segunda terminal para iniciar una sesión paralela aislada. Consulte [Worktrees](/docs/es/worktrees) para limpieza, `.worktreeinclude` y soporte de VCS que no sea git. Para monitorear sesiones paralelas desde una pantalla en lugar de terminales separadas, consulte [agentes de fondo](/docs/es/agent-view).

<h2 id="plan-before-editing">
  Planificar antes de editar
</h2>

Para cambios que desea revisar antes de que toquen el disco, cambie al modo de plan. Claude lee archivos y propone un plan pero no realiza ediciones hasta que apruebe.

```bash theme={null}
claude --permission-mode plan
```

También puede presionar `Shift+Tab` durante una sesión para cambiar al modo de plan. Consulte [Plan mode](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode) para el flujo de aprobación y editar el plan en su editor de texto.

<h2 id="delegate-research-to-subagents">
  Delegar investigación a subagentes
</h2>

Explorar una base de código grande llena su contexto con lecturas de archivos. Delegue la exploración para que solo los hallazgos regresen.

```text theme={null}
usa un subagente para investigar cómo nuestro sistema de autenticación maneja la actualización de tokens
```

El subagente lee archivos en su propia ventana de contexto e informa un resumen. Consulte [Subagentes](/docs/es/sub-agents) para definir agentes personalizados con sus propias herramientas e indicaciones.

<h2 id="pipe-claude-into-scripts">
  Canalizar Claude en scripts
</h2>

Ejecute Claude de forma no interactiva para CI, hooks de pre-commit o procesamiento por lotes. Stdin y stdout funcionan como cualquier herramienta Unix.

```bash theme={null}
git log --oneline -20 | claude -p "summarize these recent commits"
```

Consulte [Modo no interactivo](/docs/es/headless) para formatos de salida, banderas de permiso y patrones de distribución.

<h2 id="next-steps">
  Próximos pasos
</h2>

<CardGroup cols={2}>
  <Card title="Mejores prácticas" icon="lightbulb" href="/docs/es/best-practices">
    Patrones para obtener lo máximo de Claude Code
  </Card>

  <Card title="Gestionar sesiones" icon="rotate-left" href="/docs/es/sessions">
    Reanudar, nombrar y ramificar conversaciones
  </Card>

  <Card title="Worktrees" icon="code-branch" href="/docs/es/worktrees">
    Ejecutar sesiones paralelas aisladas
  </Card>

  <Card title="Extender Claude Code" icon="puzzle-piece" href="/docs/es/features-overview">
    Agregar skills, hooks, MCP, subagentes y plugins
  </Card>
</CardGroup>
