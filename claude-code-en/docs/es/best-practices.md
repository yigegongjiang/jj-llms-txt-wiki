> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Mejores prácticas para Claude Code

> Consejos y patrones para aprovechar al máximo Claude Code, desde configurar su entorno hasta escalar entre sesiones paralelas.

Claude Code es un entorno de codificación agencial. A diferencia de un chatbot que responde preguntas y espera, Claude Code puede leer sus archivos, ejecutar comandos, hacer cambios y trabajar autónomamente a través de problemas mientras usted observa, redirige o se aleja completamente.

Esto cambia cómo trabaja. En lugar de escribir código usted mismo y pedirle a Claude que lo revise, describe lo que desea y Claude descubre cómo construirlo. Claude explora, planifica e implementa.

Pero esta autonomía aún viene con una curva de aprendizaje. Claude trabaja dentro de ciertas restricciones que necesita entender.

Esta guía cubre patrones que han demostrado ser efectivos en los equipos internos de Anthropic y para ingenieros que usan Claude Code en varios códigos base, lenguajes y entornos. Para saber cómo funciona el bucle agencial, consulte [Cómo funciona Claude Code](/docs/es/how-claude-code-works).

***

La mayoría de las mejores prácticas se basan en una restricción: la ventana de contexto de Claude se llena rápidamente y el rendimiento se degrada a medida que se llena.

La ventana de contexto de Claude contiene toda su conversación, incluido cada mensaje, cada archivo que Claude lee y cada salida de comando. Sin embargo, esto puede llenarse rápidamente. Una única sesión de depuración o exploración de código base podría generar y consumir decenas de miles de tokens.

Esto importa porque el rendimiento del LLM se degrada a medida que se llena el contexto. Cuando la ventana de contexto se está llenando, Claude puede comenzar a "olvidar" instrucciones anteriores o cometer más errores. La ventana de contexto es el recurso más importante a gestionar. Para ver cómo se llena una sesión en la práctica, [vea un recorrido interactivo](/docs/es/context-window) de lo que se carga al inicio y cuánto cuesta cada lectura de archivo. Rastree el uso de contexto continuamente con una [línea de estado personalizada](/docs/es/statusline), y consulte [Reducir el uso de tokens](/docs/es/costs#reduce-token-usage) para estrategias sobre cómo reducir el uso de tokens.

***

<h2 id="give-claude-a-way-to-verify-its-work">
  Dé a Claude una forma de verificar su trabajo
</h2>

<Tip>
  Dé a Claude una verificación que pueda ejecutar: pruebas, una compilación, una captura de pantalla para comparar. Es la diferencia entre una sesión que observa y una de la que se aleja.
</Tip>

Claude se detiene cuando el trabajo parece estar hecho. Sin una verificación que pueda ejecutar, "parece estar hecho" es la única señal disponible, y usted se convierte en el bucle de verificación: cada error espera a que lo note. Dé a Claude algo que produzca un resultado de aprobado o reprobado, y el bucle se cierra por sí solo. Claude realiza el trabajo, ejecuta la verificación, lee el resultado e itera hasta que la verificación pase.

La verificación es cualquier cosa que devuelva una señal que Claude pueda leer en la conversación: un conjunto de pruebas, un código de salida de compilación, un linter, un script que compara la salida con una fixture, o una [captura de pantalla del navegador](/docs/es/chrome) comparada con un diseño. Ejecute [`/verify`](/docs/es/skills#run-and-verify-your-app) usted mismo después de que la verificación de Claude pase para confirmar el cambio contra la aplicación en ejecución.

| Estrategia                                 | Antes                                                                    | Después                                                                                                                                                                                                                              |
| ------------------------------------------ | ------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Proporcionar criterios de verificación** | *"implementar una función que valide direcciones de correo electrónico"* | *"escribir una función validateEmail. casos de prueba de ejemplo: [user@example.com](mailto:user@example.com) es verdadero, inválido es falso, [user@.com](mailto:user@.com) es falso. ejecutar las pruebas después de implementar"* |
| **Verificar cambios de UI visualmente**    | *"hacer que el panel de control se vea mejor"*                           | *"\[pegar captura de pantalla] implementar este diseño. tomar una captura de pantalla del resultado y compararla con la original. listar diferencias y corregirlas"*                                                                 |
| **Abordar causas raíz, no síntomas**       | *"la compilación está fallando"*                                         | *"la compilación falla con este error: \[pegar error]. corregirlo y verificar que la compilación tenga éxito. abordar la causa raíz, no suprimir el error"*                                                                          |

Una vez que la verificación existe, decida qué tan estrictamente detiene el trabajo:

* **En un solo mensaje**: pida a Claude que ejecute la verificación e itere en el mismo mensaje, como en la tabla anterior.
* **En toda una sesión**: establezca la verificación como una [condición `/goal`](/docs/es/goal). Un evaluador separado la vuelve a verificar después de cada turno y Claude continúa trabajando hasta que se cumpla el objetivo. Si Claude se estanca, Claude Code eventualmente detiene la ejecución con el objetivo aún establecido — vea [cómo funciona la evaluación de /goal](/docs/es/goal#how-evaluation-works).
* **Como una puerta determinista**: un [hook Stop](/docs/es/hooks#stop) ejecuta su verificación como un script y bloquea el final del turno hasta que pase. Claude Code anula el hook y finaliza el turno después de 8 bloqueos consecutivos.
* **Por una segunda opinión**: un [subagente de verificación](/docs/es/sub-agents) o un [flujo de trabajo dinámico](/docs/es/workflows) que verifica sus propios hallazgos tiene un modelo fresco que intenta refutar el resultado, por lo que el agente que realiza el trabajo no es el que lo califica.

Cada paso intercambia configuración por atención. La versión de mensaje funciona en cualquier tarea hoy. Las versiones `/goal` y Stop hook son las que permiten que una ejecución desatendida se complete correctamente sin usted.

Haga que Claude muestre evidencia en lugar de afirmar el éxito: la salida de la prueba, el comando que ejecutó y lo que devolvió, o una captura de pantalla del resultado. Revisar evidencia es más rápido que volver a ejecutar la verificación usted mismo, y funciona para sesiones que no estaba observando.

***

<h2 id="explore-first-then-plan-then-code">
  Explore primero, luego planifique, luego codifique
</h2>

<Tip>
  Separe la investigación y la planificación de la implementación para evitar resolver el problema incorrecto.
</Tip>

Dejar que Claude salte directamente a la codificación puede producir código que resuelve el problema incorrecto. Use [plan mode](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode) para separar la exploración de la ejecución.

El flujo de trabajo recomendado tiene cuatro fases:

<Steps>
  <Step title="Explorar">
    Ingrese plan mode presionando `Shift+Tab` hasta que la barra de estado muestre `⏸ plan mode on`, o inicie la sesión con `claude --permission-mode plan`. Claude lee archivos y responde preguntas sin hacer cambios.

    ```txt title="claude (plan mode)" wrap theme={null}
    read /src/auth and understand how we handle sessions and login.
    also look at how we manage environment variables for secrets.
    ```
  </Step>

  <Step title="Planificar">
    Pida a Claude que cree un plan de implementación detallado.

    ```txt title="claude (plan mode)" wrap theme={null}
    I want to add Google OAuth. What files need to change?
    What's the session flow? Create a plan.
    ```

    Presione `Ctrl+G` para abrir el plan en su editor de texto para edición directa antes de que Claude continúe.
  </Step>

  <Step title="Implementar">
    Salga de plan mode aprobando el plan o presionando `Shift+Tab`, luego deje que Claude codifique, verificando contra su plan.

    ```txt title="claude" wrap theme={null}
    implement the OAuth flow from your plan. write tests for the
    callback handler, run the test suite and fix any failures.
    ```
  </Step>

  <Step title="Confirmar">
    Pida a Claude que confirme con un mensaje descriptivo y cree un PR.

    ```txt title="claude" wrap theme={null}
    commit with a descriptive message and open a PR
    ```
  </Step>
</Steps>

<Callout>
  Plan mode es útil, pero también agrega sobrecarga.

  Para tareas donde el alcance es claro y la corrección es pequeña (como corregir un error tipográfico, agregar una línea de registro o renombrar una variable) pida a Claude que lo haga directamente.

  La planificación es más útil cuando no está seguro del enfoque, cuando el cambio modifica múltiples archivos o cuando no está familiarizado con el código que se está modificando. Si pudiera describir el diff en una oración, omita el plan.
</Callout>

***

<h2 id="provide-specific-context-in-your-prompts">
  Proporcione contexto específico en sus indicaciones
</h2>

<Tip>
  Cuanto más precisas sean sus instrucciones, menos correcciones necesitará.
</Tip>

Claude puede inferir intención, pero no puede leer su mente. Haga referencia a archivos específicos, mencione restricciones y señale patrones de ejemplo.

| Estrategia                                                                                           | Antes                                                    | Después                                                                                                                                                                                                                                                                                                                                                                                       |
| ---------------------------------------------------------------------------------------------------- | -------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Delimitar la tarea.** Especifique qué archivo, qué escenario y preferencias de prueba.             | *"agregar pruebas para foo.py"*                          | *"escribir una prueba para foo.py cubriendo el caso extremo donde el usuario ha cerrado sesión. evitar mocks."*                                                                                                                                                                                                                                                                               |
| **Señalar fuentes.** Dirija a Claude a la fuente que puede responder una pregunta.                   | *"¿por qué ExecutionFactory tiene una API tan extraña?"* | *"revisar el historial de git de ExecutionFactory y resumir cómo su API llegó a ser así"*                                                                                                                                                                                                                                                                                                     |
| **Hacer referencia a patrones existentes.** Señale a Claude los patrones en su código base.          | *"agregar un widget de calendario"*                      | *"ver cómo se implementan los widgets existentes en la página de inicio para entender los patrones. HotDogWidget.php es un buen ejemplo. seguir el patrón para implementar un nuevo widget de calendario que permita al usuario seleccionar un mes y paginar hacia adelante/atrás para elegir un año. construir desde cero sin bibliotecas que no sean las ya utilizadas en el código base."* |
| **Describir el síntoma.** Proporcione el síntoma, la ubicación probable y qué significa "corregido". | *"corregir el error de inicio de sesión"*                | *"los usuarios informan que el inicio de sesión falla después del agotamiento de la sesión. verificar el flujo de autenticación en src/auth/, especialmente la actualización de tokens. escribir una prueba fallida que reproduzca el problema, luego corregirlo"*                                                                                                                            |

Las indicaciones vagas pueden ser útiles cuando está explorando y puede permitirse corregir el curso. Una indicación como `"¿qué mejoraría en este archivo?"` puede revelar cosas en las que no habría pensado en preguntar.

<h3 id="provide-rich-content">
  Proporcionar contenido enriquecido
</h3>

<Tip>
  Use `@` para hacer referencia a archivos, pegue capturas de pantalla/imágenes o canalice datos directamente.
</Tip>

Puede proporcionar datos enriquecidos a Claude de varias maneras:

* **Haga referencia a archivos con `@`** en lugar de describir dónde vive el código. Claude lee el archivo antes de responder.
* **Pegue imágenes directamente**. Copie/pegue o arrastre y suelte imágenes en la indicación.
* **Proporcione URLs** para documentación y referencias de API. Use `/permissions` para permitir dominios de uso frecuente.
* **Canalice datos** ejecutando `cat error.log | claude` para enviar contenidos de archivo directamente.
* **Deje que Claude obtenga lo que necesita**. Diga a Claude que extraiga contexto por sí mismo usando comandos Bash, herramientas MCP o leyendo archivos.

***

<h2 id="configure-your-environment">
  Configura tu entorno
</h2>

Algunos pasos de configuración hacen que Claude Code sea significativamente más efectivo en todas tus sesiones. Para una descripción completa de las características de la extensión y cuándo usar cada una, consulta [Extend Claude Code](/docs/es/features-overview).

<h3 id="write-an-effective-claude-md">
  Escribe un CLAUDE.md efectivo
</h3>

<Tip>
  Ejecuta `/init` para generar un archivo CLAUDE.md inicial basado en la estructura actual de tu proyecto, luego refina con el tiempo.
</Tip>

CLAUDE.md es un archivo especial que Claude lee al inicio de cada conversación. Incluye comandos Bash, estilo de código y reglas de flujo de trabajo. Esto le da a Claude un contexto persistente que no puede inferir solo del código.

No hay un formato requerido para los archivos CLAUDE.md, pero mantenlos cortos y legibles para humanos. Por ejemplo:

```markdown CLAUDE.md theme={null}
# Code style
- Use ES modules (import/export) syntax, not CommonJS (require)
- Destructure imports when possible (eg. import { foo } from 'bar')

# Workflow
- Be sure to typecheck when you're done making a series of code changes
- Prefer running single tests, and not the whole test suite, for performance
```

Ejecuta `/context` para confirmar que Claude cargó el archivo. CLAUDE.md se carga en cada sesión, así que solo incluye cosas que se apliquen ampliamente. Para conocimiento de dominio o flujos de trabajo que solo son relevantes a veces, usa [skills](/docs/es/skills) en su lugar. Claude los carga bajo demanda sin saturar cada conversación.

Mantenlo conciso. Para cada línea, pregúntate: *"¿Causaría que Claude cometa errores si elimino esto?"* Si no, elimínalo. Los archivos CLAUDE.md inflados hacen que Claude ignore tus instrucciones reales.

| ✅ Incluir                                                                  | ❌ Excluir                                                        |
| -------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| Comandos Bash que Claude no puede adivinar                                 | Cualquier cosa que Claude pueda descubrir leyendo código         |
| Reglas de estilo de código que difieren de los valores predeterminados     | Convenciones de lenguaje estándar que Claude ya conoce           |
| Instrucciones de prueba y ejecutores de prueba preferidos                  | Documentación detallada de API (enlaza a documentos en su lugar) |
| Etiqueta del repositorio (nomenclatura de ramas, convenciones de PR)       | Información que cambia frecuentemente                            |
| Decisiones arquitectónicas específicas de tu proyecto                      | Explicaciones largas o tutoriales                                |
| Peculiaridades del entorno de desarrollo (variables de entorno requeridas) | Descripciones archivo por archivo de la base de código           |
| Errores comunes o comportamientos no obvios                                | Prácticas evidentes como "escribir código limpio"                |

Si Claude sigue haciendo algo que no quieres a pesar de tener una regla en contra, el archivo probablemente es demasiado largo y la regla se está perdiendo. Si Claude te hace preguntas que están respondidas en CLAUDE.md, la redacción podría ser ambigua. Trata CLAUDE.md como código: revísalo cuando las cosas salgan mal, pódalo regularmente y prueba cambios observando si el comportamiento de Claude realmente cambia. Para un CLAUDE.md registrado, ejecuta [`/doctor`](/docs/es/commands#all-commands) y Claude propone cortes para contenido que puede derivar de la base de código.

Si Claude sigue omitiendo una instrucción, añade énfasis como "IMPORTANTE" solo a esa línea. Si enfatizas muchas líneas, ninguna destaca. Registra CLAUDE.md en git para que tu equipo pueda contribuir. El archivo aumenta en valor con el tiempo.

Los archivos CLAUDE.md pueden importar archivos adicionales usando la sintaxis `@path/to/import`. Para reglas de importación y dónde pueden vivir los archivos CLAUDE.md, consulta [CLAUDE.md files](/docs/es/memory#claude-md-files).

<h3 id="configure-permissions">
  Configura permisos
</h3>

<Tip>
  Para obtener menos solicitudes sin renunciar al control, pre-aprueba las herramientas en las que confías con `/permissions` y deja que los comandos en sandbox se ejecuten sin preguntar con `/sandbox`. Cambia a modo Manual cuando quieras aprobar ediciones y comandos tú mismo.
</Tip>

En los planes Pro, Max y Team, el modo automático es el [modo de permiso inicial integrado](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) para sesiones interactivas de terminal y VS Code: un modelo clasificador separado revisa la mayoría de acciones en lugar de ti y bloquea solo lo que parece arriesgado, como escalada de alcance, infraestructura desconocida o acciones impulsadas por contenido hostil.

En modo Manual, el modo de permiso inicial integrado en otros planes, Claude Code pregunta antes de acciones que podrían modificar tu sistema: escrituras de archivos, comandos Bash, herramientas MCP. Eso es seguro pero tedioso. Después de la décima aprobación estás haciendo clic sin revisar realmente. Dos herramientas reducen esas interrupciones en modo Manual y se aplican también en modo automático:

* **Listas de permisos**: permiten herramientas específicas que sabes que son seguras, como `npm run lint` o `git commit`
* **Sandboxing**: habilita aislamiento a nivel del SO que restringe el acceso al sistema de archivos y red, permitiendo que Claude trabaje más libremente dentro de límites definidos

Lee más sobre [permission modes](/docs/es/permission-modes), [permission rules](/docs/es/permissions) y [sandboxing](/docs/es/sandboxing).

<h3 id="use-cli-tools">
  Usa herramientas CLI
</h3>

<Tip>
  Dile a Claude Code que use herramientas CLI como `gh`, `aws`, `gcloud` y `sentry-cli` cuando interactúe con servicios externos.
</Tip>

Las herramientas CLI son la forma más eficiente en contexto de interactuar con servicios externos. Si usas GitHub, instala el CLI `gh`. Claude sabe cómo usarlo para crear problemas, abrir solicitudes de extracción y leer comentarios. Sin `gh`, Claude aún puede usar la API de GitHub, pero las solicitudes no autenticadas a menudo alcanzan límites de velocidad.

Claude también es efectivo aprendiendo herramientas CLI que no conoce. Intenta solicitudes como `Use 'foo-cli-tool --help' to learn about foo tool, then use it to solve A, B, C.`

<h3 id="connect-mcp-servers">
  Conecta servidores MCP
</h3>

<Tip>
  Ejecuta `claude mcp add` con un nombre de servidor y URL o comando para conectar herramientas externas como Notion, Figma o tu base de datos. Por ejemplo: `claude mcp add --transport http notion https://mcp.notion.com/mcp`.
</Tip>

Con [MCP servers](/docs/es/mcp), puedes pedirle a Claude que implemente características de rastreadores de problemas, consulte bases de datos, analice datos de monitoreo, integre diseños de Figma y automatice flujos de trabajo.

<h3 id="set-up-hooks">
  Configura hooks
</h3>

<Tip>
  Usa hooks para acciones que deben ocurrir cada vez sin excepciones.
</Tip>

[Hooks](/docs/es/hooks-guide) ejecutan scripts automáticamente en puntos específicos del flujo de trabajo de Claude. A diferencia de las instrucciones CLAUDE.md que son consultivas, los hooks son deterministas y garantizan que la acción ocurra.

Claude puede escribir hooks para ti. Intenta solicitudes como *"Write a hook that runs eslint after every file edit"* o *"Write a hook that blocks writes to the migrations folder."* Edita `.claude/settings.json` directamente para configurar hooks manualmente, y ejecuta `/hooks` para explorar lo que está configurado.

<h3 id="create-skills">
  Crea skills
</h3>

<Tip>
  Crea archivos `SKILL.md` en `.claude/skills/` para darle a Claude conocimiento de dominio y flujos de trabajo reutilizables.
</Tip>

[Skills](/docs/es/skills) extienden el conocimiento de Claude con información específica de tu proyecto, equipo o dominio. Claude los aplica automáticamente cuando son relevantes, o puedes invocarlos directamente con `/skill-name`.

Crea una skill añadiendo un directorio con un `SKILL.md` a `.claude/skills/`:

```markdown .claude/skills/api-conventions/SKILL.md theme={null}
---
name: api-conventions
description: REST API design conventions for our services
---
# API Conventions
- Use kebab-case for URL paths
- Use camelCase for JSON properties
- Always include pagination for list endpoints
- Version APIs in the URL path (/v1/, /v2/)
```

Las skills también pueden definir flujos de trabajo repetibles que invocas directamente:

```markdown .claude/skills/fix-issue/SKILL.md theme={null}
---
name: fix-issue
description: Fix a GitHub issue
disable-model-invocation: true
---
Analyze and fix the GitHub issue: $ARGUMENTS.

1. Use `gh issue view` to get the issue details
2. Understand the problem described in the issue
3. Search the codebase for relevant files
4. Implement the necessary changes to fix the issue
5. Write and run tests to verify the fix
6. Ensure code passes linting and type checking
7. Create a descriptive commit message
8. Push and create a PR
```

Ejecuta `/fix-issue 1234` para invocarlo. Usa `disable-model-invocation: true` para flujos de trabajo con efectos secundarios que quieres activar manualmente.

<h3 id="create-custom-subagents">
  Crea subagentes personalizados
</h3>

<Tip>
  Define asistentes especializados en `.claude/agents/` que Claude pueda delegar para tareas aisladas.
</Tip>

[Subagents](/docs/es/sub-agents) se ejecutan en su propio contexto con su propio conjunto de herramientas permitidas. Son útiles para tareas que leen muchos archivos o necesitan enfoque especializado sin saturar tu conversación principal.

```markdown .claude/agents/security-reviewer.md theme={null}
---
name: security-reviewer
description: Reviews code for security vulnerabilities
tools: Read, Grep, Glob, Bash
model: opus
---
You are a senior security engineer. Review code for:
- Injection vulnerabilities (SQL, XSS, command injection)
- Authentication and authorization flaws
- Secrets or credentials in code
- Insecure data handling

Provide specific line references and suggested fixes.
```

Dile a Claude que use subagentes explícitamente: *"Use a subagent to review this code for security issues."*

<h3 id="install-plugins">
  Instala plugins
</h3>

<Tip>
  Ejecuta `/plugin` para explorar el marketplace. Los plugins añaden skills, herramientas e integraciones sin configuración.
</Tip>

[Plugins](/docs/es/plugins/overview) agrupan skills, hooks, subagentes y servidores MCP en una única unidad instalable de la comunidad y Anthropic. Si trabajas con un lenguaje tipado, instala un [code intelligence plugin](/docs/es/plugins/code-intelligence) para darle a Claude navegación de símbolos precisa y detección automática de errores después de ediciones.

Para orientación sobre cómo elegir entre skills, subagentes, hooks y MCP, consulta [Extend Claude Code](/docs/es/features-overview#match-features-to-your-goal).

***

<h2 id="communicate-effectively">
  Comuníquese efectivamente
</h2>

Haga a Claude las preguntas que haría a otro ingeniero, y para características más grandes, deje que Claude lo entreviste y escriba una especificación antes de que comience a implementar.

<h3 id="ask-codebase-questions">
  Haga preguntas sobre el código base
</h3>

<Tip>
  Haga a Claude preguntas que haría a un ingeniero senior.
</Tip>

Al incorporarse a un nuevo código base, use Claude Code para aprender y explorar. Puede hacer a Claude el mismo tipo de preguntas que haría a otro ingeniero:

* ¿Cómo funciona el registro?
* ¿Cómo hago un nuevo punto final de API?
* ¿Qué hace `async move { ... }` en la línea 134 de `foo.rs`?
* ¿Qué casos extremos maneja `CustomerOnboardingFlowImpl`?
* ¿Por qué este código llama a `foo()` en lugar de `bar()` en la línea 333?

Usar Claude Code de esta manera es un flujo de trabajo de incorporación efectivo, mejorando el tiempo de rampa y reduciendo la carga en otros ingenieros. No se requiere indicación especial: haga preguntas directamente.

<h3 id="let-claude-interview-you">
  Deje que Claude lo entreviste
</h3>

<Tip>
  Para características más grandes, deje que Claude lo entreviste primero. Comience con una indicación mínima y pida a Claude que lo entreviste usando la herramienta `AskUserQuestion`.
</Tip>

Claude hace preguntas sobre cosas que podría no haber considerado, incluyendo implementación técnica, UI/UX, casos extremos y compensaciones. Reemplace `[brief description]` con su característica antes de enviar la indicación.

```text wrap theme={null}
I want to build [brief description]. Interview me in detail using the AskUserQuestion tool.

Ask about technical implementation, UI/UX, edge cases, concerns, and tradeoffs. Don't ask obvious questions, dig into the hard parts I might not have considered.

Keep interviewing until we've covered everything, then write a complete spec to SPEC.md.
```

Una vez que la especificación esté completa, inicie una sesión nueva para ejecutarla. La nueva sesión tiene contexto limpio enfocado completamente en la implementación, y tiene una especificación escrita para hacer referencia.

Las especificaciones más útiles son autónomas: nombran los archivos e interfaces involucrados, establecen qué está fuera del alcance, y terminan con un paso de verificación de extremo a extremo que demuestra que la característica funciona. El tiempo dedicado a hacer la especificación precisa se compensa más que el tiempo dedicado a observar la implementación.

***

<h2 id="manage-your-session">
  Gestione su sesión
</h2>

Las conversaciones son persistentes y reversibles. ¡Úselo a su favor!

<h3 id="course-correct-early-and-often">
  Corrija el curso temprano y a menudo
</h3>

<Tip>
  Corrija a Claude tan pronto como note que se desvía del camino.
</Tip>

Los mejores resultados provienen de bucles de retroalimentación ajustados. Aunque Claude ocasionalmente resuelve problemas perfectamente en el primer intento, corregirlo rápidamente generalmente produce mejores soluciones más rápido.

* **`Esc`**: detener a Claude a mitad de acción con la tecla `Esc`. El contexto se preserva, para que pueda redirigir.
* **`Esc + Esc` o `/rewind`**: presione `Esc` dos veces o ejecute `/rewind` para abrir el menú de rebobinado y restaurar la conversación anterior y el estado del código, o resumir desde un mensaje seleccionado.
* **`"Undo that"`**: haga que Claude revierta sus cambios.
* **`/clear`**: restablecer contexto entre tareas no relacionadas. Las sesiones largas con contexto irrelevante pueden reducir el rendimiento.

Si ha corregido a Claude más de dos veces en el mismo problema en una sesión, el contexto está saturado de enfoques fallidos. Ejecute `/clear` e inicie de nuevo con una indicación más específica que incorpore lo que aprendió. Una sesión limpia con una indicación mejor casi siempre supera una sesión larga con correcciones acumuladas.

<h3 id="manage-context-aggressively">
  Gestione el contexto agresivamente
</h3>

<Tip>
  Ejecute `/clear` entre tareas no relacionadas para restablecer el contexto.
</Tip>

Claude Code compacta automáticamente el historial de conversación cuando se acerca a los límites de contexto, lo que preserva código importante y decisiones mientras libera espacio.

Durante sesiones largas, la ventana de contexto de Claude puede llenarse con conversación irrelevante, contenidos de archivo y comandos. Esto puede reducir el rendimiento y a veces distraer a Claude.

* Use `/clear` frecuentemente entre tareas para restablecer completamente la ventana de contexto
* Cuando se activa la compactación automática, Claude resume lo que más importa, incluyendo patrones de código, estados de archivo y decisiones clave
* Para más control, ejecute `/compact <instructions>`, como `/compact Focus on the API changes`
* Para compactar solo parte de la conversación, use `Esc + Esc` o `/rewind`, seleccione un punto de control de mensaje y elija **Summarize from here** o **Summarize up to here**. El primero condensa mensajes desde ese punto hacia adelante mientras mantiene el contexto anterior intacto; el segundo condensa mensajes anteriores mientras mantiene los recientes en su totalidad. Consulte [las opciones de resumición del menú de rebobinado](/docs/es/checkpointing#rewind-and-summarize).
* Personalice el comportamiento de compactación en CLAUDE.md con instrucciones como `"When compacting, always preserve the full list of modified files and any test commands"` para asegurar que el contexto crítico sobreviva a la resumición
* Para preguntas que no necesitan permanecer en contexto, use [`/btw`](/docs/es/interactive-mode#side-questions-with-%2Fbtw). La respuesta nunca entra en el historial de conversación, para que pueda verificar un detalle sin aumentar el contexto.

<h3 id="use-subagents-for-investigation">
  Use subagents para investigación
</h3>

<Tip>
  Delegue investigación con `"use subagents to investigate X"`. Exploran en un contexto separado, manteniendo su conversación principal limpia para la implementación.
</Tip>

Dado que el contexto es su restricción fundamental, use subagents para mantener la investigación fuera de él. Cuando Claude investiga un código base, lee muchos archivos, todos los cuales consumen su contexto. Los subagents se ejecutan en ventanas de contexto separadas e informan resúmenes:

```text wrap theme={null}
Use subagents to investigate how our authentication system handles token
refresh, and whether we have any existing OAuth utilities I should reuse.
```

También puede usar subagents para verificación después de que Claude implemente algo. Consulte [Agregue un paso de revisión adversarial](#add-an-adversarial-review-step).

<h3 id="rewind-with-checkpoints">
  Rebobine con puntos de control
</h3>

<Tip>
  Cada indicación que envía que inicia un turno crea un punto de control. Puede restaurar conversación, código o ambos a cualquier punto de control anterior.
</Tip>

Claude automáticamente crea instantáneas de archivos antes de cada cambio para que un punto de control pueda restaurarlos. Presione Escape dos veces o ejecute `/rewind` para abrir el menú de rebobinado. Puede restaurar solo conversación, restaurar solo código, restaurar ambos o resumir desde un mensaje seleccionado. Consulte [Checkpointing](/docs/es/checkpointing) para detalles.

En lugar de planificar cuidadosamente cada movimiento, puede decirle a Claude que intente algo arriesgado. Si no funciona, rebobine e intente un enfoque diferente. Los puntos de control se guardan con la conversación, para que pueda cerrar su terminal, reanudar la sesión más tarde y aún rebobinar.

<Warning>
  Los puntos de control solo rastrean cambios realizados a través de las herramientas de edición de archivos de Claude. Los cambios realizados a través de comandos Bash o procesos externos no se capturan. Esto no es un reemplazo para git.
</Warning>

<h3 id="resume-conversations">
  Reanudar conversaciones
</h3>

<Tip>
  Nombre sesiones con `/rename` y trate las como ramas: cada flujo de trabajo obtiene su propio contexto persistente.
</Tip>

Claude Code guarda conversaciones localmente, por lo que cuando una tarea abarca múltiples sesiones no tiene que re-explicar el contexto. Ejecute [`claude --continue`](/docs/es/sessions#resume-a-session) para continuar donde lo dejó, o `claude --resume` para elegir de una lista. Dé a las sesiones nombres descriptivos como `oauth-migration` para que pueda encontrarlas más tarde. Consulte [Gestione sesiones](/docs/es/sessions) para el conjunto completo de controles de reanudación, ramificación y denominación.

***

<h2 id="automate-and-scale">
  Automatizar y escalar
</h2>

Una vez que sea efectivo con un Claude, multiplique su salida con sesiones paralelas, modo no interactivo y patrones de distribución.

<h3 id="run-non-interactive-mode">
  Ejecutar modo no interactivo
</h3>

<Tip>
  Use `claude -p "prompt"` en CI, hooks de pre-commit o scripts. Agregue `--output-format stream-json --verbose` para salida JSON en streaming.
</Tip>

Con `claude -p "su prompt"`, puede ejecutar Claude de forma no interactiva, sin un prompt interactivo. La ejecución aún crea una sesión reanudable a menos que pase `--no-session-persistence`. [El modo no interactivo](/docs/es/headless) es cómo integra Claude en canalizaciones de CI, hooks de pre-commit o cualquier flujo de trabajo automatizado. Los formatos de salida le permiten analizar resultados mediante programación: texto sin formato, JSON o JSON en streaming.

```bash theme={null}
# Consultas puntuales
claude -p "Explain what this project does"

# Salida estructurada para scripts
claude -p "List all API endpoints" --output-format json

# Streaming para procesamiento en tiempo real
claude -p "Analyze this log file" --output-format stream-json --verbose
```

El primer comando imprime texto sin formato. El formato `json` devuelve un único objeto JSON con un campo `result`. El formato `stream-json` imprime un objeto JSON por línea, comenzando con un evento de inicialización.

<h3 id="run-multiple-claude-sessions">
  Ejecutar múltiples sesiones de Claude
</h3>

<Tip>
  Ejecute múltiples sesiones de Claude en paralelo para acelerar el desarrollo, ejecutar experimentos aislados o iniciar flujos de trabajo complejos.
</Tip>

Elija el enfoque paralelo que se ajuste a cuánta coordinación desea hacer usted mismo, y agregue mensajería cuando las sesiones necesiten pasar hallazgos entre ellas:

* [Git Worktrees](/docs/es/worktrees): ejecute sesiones de CLI separadas en checkouts de git aislados para que las ediciones no choquen
* [Mensajería entre sesiones](/docs/es/cross-session-messaging): permita que las sesiones que ejecuta usted mismo pasen hallazgos entre sí
* [Aplicación de escritorio](/docs/es/desktop#work-in-parallel-with-sessions): administre múltiples sesiones locales visualmente, opcionalmente cada una en su propio worktree
* [Claude Code en la web](/docs/es/claude-code-on-the-web): ejecute sesiones en la nube, en infraestructura administrada por Anthropic de forma predeterminada
* [Vista de agente](/docs/es/agent-view): vista previa de investigación. Ejecute `claude agents` para enviar sesiones que sigan ejecutándose en segundo plano y obsérvelas desde una pantalla
* [Equipos de agentes](/docs/es/agent-teams): experimental y deshabilitado de forma predeterminada. Coordinación automatizada de múltiples sesiones con tareas compartidas, mensajería y un líder de equipo

Más allá de paralelizar el trabajo, múltiples sesiones permiten flujos de trabajo enfocados en la calidad. Un contexto fresco mejora la revisión de código ya que Claude no estará sesgado hacia el código que acaba de escribir.

Por ejemplo, use un patrón Escritor/Revisor:

| Sesión A (Escritor)                                                     | Sesión B (Revisor)                                                                                                                                                       |
| ----------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Implement a rate limiter for our API endpoints`                        |                                                                                                                                                                          |
|                                                                         | `Review the rate limiter implementation in @src/middleware/rateLimiter.ts. Look for edge cases, race conditions, and consistency with our existing middleware patterns.` |
| `Here's the review feedback: [Session B output]. Address these issues.` |                                                                                                                                                                          |

Puede hacer algo similar con pruebas: haga que un Claude escriba pruebas, luego otro escriba código para pasarlas.

<h3 id="fan-out-across-files">
  Distribuir entre archivos
</h3>

<Tip>
  Recorra tareas llamando a `claude -p` para cada una. Use `--allowedTools` para limitar permisos para operaciones por lotes.
</Tip>

Para migraciones o análisis grandes, puede distribuir el trabajo entre muchas invocaciones paralelas de Claude. Ejecute [`/batch <instruction>`](/docs/es/commands#all-commands) para que Claude divida el cambio entre 5 a 30 subagentes. Cada subagente trabaja en su propio worktree. Para impulsar la distribución desde su propio script en su lugar, recorra `claude -p`:

<Steps>
  <Step title="Generar una lista de tareas">
    Haga que Claude escriba la lista de archivos que necesitan migración en un archivo, para que el bucle en el siguiente paso pueda leerlo, con un prompt como `list all 2,000 Python files that need migrating and save the list to files.txt`
  </Step>

  <Step title="Escribir un script para recorrer la lista">
    ```bash theme={null}
    for file in $(cat files.txt); do
      claude -p "Migrate $file from Python 2 to Python 3. Return OK or FAIL." \
        --allowedTools "Edit,Bash(git commit *)"
    done
    ```
  </Step>

  <Step title="Probar en algunos archivos, luego ejecutar en todos ellos">
    Refine su prompt basándose en lo que sale mal con los primeros 2-3 archivos, luego ejecute en el conjunto completo. La bandera `--allowedTools` restringe lo que Claude puede hacer, lo que importa cuando está ejecutando sin supervisión.
  </Step>
</Steps>

También puede integrar Claude en canalizaciones de datos/procesamiento existentes:

```bash theme={null}
claude -p "<your prompt>" --output-format json | your_command
```

<h3 id="run-autonomously-with-auto-mode">
  Ejecutar de forma autónoma con modo automático
</h3>

Para ejecución ininterrumpida con comprobaciones de seguridad en segundo plano, use [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode). Un modelo clasificador revisa los comandos antes de que se ejecuten, bloqueando escalada de alcance, infraestructura desconocida y acciones impulsadas por contenido hostil mientras permite que el trabajo rutinario continúe sin prompts.

```bash theme={null}
claude --permission-mode auto -p "fix all lint errors"
```

Cuando el clasificador bloquea repetidamente acciones en una ejecución no interactiva con la bandera `-p`, Claude Code no detiene la ejecución. Vea [cuándo el modo automático retrocede](/docs/es/permission-modes#when-auto-mode-falls-back) para saber qué sucede en su lugar y para los umbrales.

<h3 id="add-an-adversarial-review-step">
  Agregar un paso de revisión adversarial
</h3>

<Tip>
  Antes de tratar una tarea como completada, haga que un subagente revise el diff en un contexto fresco e informe sobre brechas.
</Tip>

Cuanto más tiempo trabaje Claude sin supervisión, más importa una verificación independiente antes de contar el trabajo como completado. Un revisor que se ejecuta en un contexto de [subagente](/docs/es/sub-agents) fresco ve solo el diff y los criterios que le proporciona, no el razonamiento que produjo el cambio, por lo que evalúa el resultado en sus propios términos.

Para una verificación de corrección, ejecute la [skill `/code-review`](/docs/es/commands) incluida, que revisa el diff actual en busca de errores en un subagente fresco y devuelve hallazgos a la sesión. Para verificar el diff contra su plan en su lugar, escriba el prompt de revisión usted mismo. Nombre el trabajo a verificar, el plan a verificar contra él y qué cuenta como un hallazgo:

```text wrap theme={null}
Use a subagent to review the rate limiter diff against PLAN.md. Check that
every requirement is implemented, the listed edge cases have tests, and
nothing outside the task's scope changed. Report gaps, not style preferences.
```

Debido a que el revisor se ejecuta como un subagente, la sesión de implementación recibe las brechas directamente y puede corregirlas y revisar nuevamente sin que usted copie hallazgos entre ventanas.

<Callout>
  Un revisor indicado para encontrar brechas generalmente reportará algunas, incluso cuando el trabajo es sólido, porque eso es lo que se le pidió que hiciera. Perseguir cada hallazgo conduce a sobre-ingeniería: capas de abstracción adicionales, código defensivo y pruebas para casos que no pueden suceder. Dígale al revisor que marque solo brechas que afecten la corrección o los requisitos establecidos, y trate el resto como opcional.
</Callout>

***

<h2 id="avoid-common-failure-patterns">
  Evite patrones de falla comunes
</h2>

Estos son errores comunes. Reconocerlos temprano ahorra tiempo:

* **La sesión de todo incluido.** Comienza con una tarea, luego pregunta a Claude algo no relacionado, luego vuelve a la primera tarea. El contexto está lleno de información irrelevante.
  > **Solución**: `/clear` entre tareas no relacionadas.
* **Corrección una y otra vez.** Claude hace algo mal, lo corrige, sigue siendo incorrecto, lo corrige de nuevo. El contexto está contaminado con enfoques fallidos.
  > **Solución**: Después de dos correcciones fallidas, `/clear` y escriba una indicación inicial mejor incorporando lo que aprendió.
* **El CLAUDE.md sobre especificado.** Si su CLAUDE.md es demasiado largo, Claude ignora la mitad porque las reglas importantes se pierden en el ruido.
  > **Solución**: Elimine sin piedad. Si Claude ya hace algo correctamente sin la instrucción, elimínelo o conviértalo en un hook.
* **La brecha de confianza-luego-verificación.** Claude produce una implementación que se ve plausible pero no maneja casos extremos.
  > **Solución**: Siempre proporcione verificación (pruebas, scripts, capturas de pantalla). Si no puede verificarlo, no lo envíe.
* **La exploración infinita.** Pide a Claude que "investigue" algo sin delimitarlo. Claude lee cientos de archivos, llenando el contexto.
  > **Solución**: Delimite investigaciones estrechamente o use subagents para que la exploración no consuma su contexto principal.

***

<h2 id="develop-your-intuition">
  Desarrolle su intuición
</h2>

Los patrones en esta guía no están grabados en piedra. Son puntos de partida que funcionan bien en general, pero podrían no ser óptimos para cada situación.

A veces *debería* dejar que el contexto se acumule porque está profundo en un problema complejo y el historial es valioso. A veces debería omitir la planificación y dejar que Claude lo descubra porque la tarea es exploratoria. A veces una indicación vaga es exactamente lo correcto porque desea ver cómo Claude interpreta el problema antes de limitarlo.

Preste atención a lo que funciona. Cuando Claude produce una salida excelente, note lo que hizo: la estructura de la indicación, el contexto que proporcionó, el modo en que estaba. Cuando Claude lucha, pregúntese por qué. ¿Fue el contexto demasiado ruidoso? ¿La indicación demasiado vaga? ¿La tarea demasiado grande para un pase?

Con el tiempo, desarrollará intuición que ninguna guía puede capturar. Sabrá cuándo ser específico y cuándo ser abierto, cuándo planificar y cuándo explorar, cuándo limpiar contexto y cuándo dejarlo acumular.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Cómo funciona Claude Code](/docs/es/how-claude-code-works): el bucle agencial, herramientas y gestión de contexto
* [Extender Claude Code](/docs/es/features-overview): skills, hooks, MCP, subagents y plugins
* [Flujos de trabajo comunes](/docs/es/common-workflows): recetas paso a paso para depuración, pruebas, PRs y más
* [CLAUDE.md](/docs/es/memory): almacenar convenciones de proyecto y contexto persistente
