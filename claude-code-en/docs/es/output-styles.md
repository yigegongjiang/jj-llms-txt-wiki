> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Estilos de salida

> Cambie el rol, tono y formato de respuesta de Claude Code con un estilo de salida integrado como Conciso o Explicativo, o escriba un estilo personalizado.

Un estilo de salida es un conjunto de instrucciones que establece el rol, tono y formato de respuesta de Claude para cada respuesta en una sesión. Claude Code incluye cuatro estilos integrados además de su predeterminado, y usted puede escribir el suyo propio.

Utilice un estilo de salida para cambiar la forma en que Claude responde y trabaja con usted durante toda una sesión, de modo que no tenga que repetir la solicitud en cada indicación. Por ejemplo, un estilo integrado puede hacer que las respuestas sean más cortas, agregar una explicación de cada cambio, o hacer que Claude comience a trabajar sin hacer preguntas rutinarias. Un estilo personalizado también puede convertir Claude en algo diferente a un ingeniero de software, como un asistente de escritura o un analista de datos.

* Para usar un estilo integrado, elija uno de los [estilos de salida integrados](#built-in-output-styles) y [cambie a él](#change-your-output-style).
* Para escribir sus propias instrucciones, [cree un estilo de salida personalizado](#create-a-custom-output-style).

<Note>
  Un estilo de salida proporciona instrucciones a Claude para que las siga. No garantiza que algo siempre suceda o nunca suceda. Algunas necesidades se ajustan mejor a una característica diferente:

  * Para lo que Claude debe saber sobre su proyecto, use [CLAUDE.md](/docs/es/memory).
  * Para algo que tiene que suceder cada vez, como formatear después de cada edición o bloquear un comando, use un [hook](/docs/es/hooks-guide).
  * Para skills, subagentes y las otras opciones, consulte [Elegir entre un estilo de salida y otras características](#choose-between-an-output-style-and-other-features).
</Note>

<h2 id="built-in-output-styles">
  Estilos de salida integrados
</h2>

Claude Code comienza en el estilo [**Default**](#default), sus instrucciones estándar para completar tareas de ingeniería de software. Cada uno de los otros cuatro estilos integrados mantiene esas instrucciones y añade las suyas propias.

Esta tabla muestra qué cambia cada estilo en una sesión y cuándo es apropiado:

| Estilo                      | Qué cambia                                                                                                                  | Úselo cuando                                                                                                                 |
| :-------------------------- | :-------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| [Proactive](#proactive)     | Claude comienza el trabajo de inmediato y realiza suposiciones razonables en lugar de preguntar sobre decisiones rutinarias | Desea que Claude continúe trabajando a través de decisiones rutinarias, y corregirá el rumbo si una suposición es incorrecta |
| [Concise](#concise)         | Las respuestas comienzan con el resultado y omiten el preámbulo, la narración y los resúmenes                               | Las respuestas predeterminadas son más largas de lo que desea                                                                |
| [Explanatory](#explanatory) | Claude añade bloques cortos de `Insight` que explican las opciones detrás del código que escribe                            | Está conociendo una base de código o desea el razonamiento junto con el cambio                                               |
| [Learning](#learning)       | Claude explica sus opciones y deja pequeños fragmentos de código para que usted escriba                                     | Desea práctica de codificación práctica mientras la tarea aún se completa                                                    |

<h3 id="default">
  Default
</h3>

Default significa que no se selecciona ningún estilo de salida. Claude Code no añade instrucciones de estilo, y Claude trabaja desde el símbolo del sistema estándar de Claude Code, que está escrito para tareas de ingeniería de software.

`default` aparece en la lista `/output-style` con los otros estilos, por lo que [lo selecciona de la misma manera](#change-your-output-style).

<h3 id="proactive">
  Proactive
</h3>

En el estilo Proactive, Claude comienza a implementar tan pronto como envía una tarea. Realiza suposiciones razonables sobre decisiones rutinarias en lugar de detenerse para preguntar, y no cambia al modo de plan a menos que solicite un plan. Puede redirigirlo en cualquier momento.

Las instrucciones del estilo también le dicen a Claude que se comunique con usted en la conversación antes de una acción que elimine datos o cambie un sistema compartido o de producción. Esa verificación es una instrucción que Claude sigue y es separada de los mensajes de permiso.

Cambiar al estilo Proactive no cambia su [modo de permiso](/docs/es/permission-modes). Su modo de permiso aún decide qué llamadas de herramienta se ejecutan sin preguntarle, por lo que los mensajes de permiso aparecen de la misma manera que antes de cambiar.

<h3 id="concise">
  Concise
</h3>

En el estilo Concise, la primera oración de una respuesta indica qué sucedió o cuál es la respuesta. Claude omite la introducción, la narración paso a paso y el resumen de cierre, y responde una pregunta simple en una a tres oraciones. Realiza el trabajo de ingeniería de manera tan exhaustiva como en el estilo Default. Requiere Claude Code v2.1.237 o posterior.

Claude aún escribe a toda extensión en estos casos:

* **Cualquier cosa que solicite**: cuando solicita una explicación o más detalle, Claude responde completamente.
* **Cualquier cosa que necesite para actuar de manera segura**: los informes de errores, la salida de pruebas fallidas, las advertencias de seguridad y las confirmaciones de acciones destructivas mantienen su contenido completo.

<h3 id="explanatory">
  Explanatory
</h3>

En el estilo Explanatory, Claude realiza la tarea de la manera que lo hace en el estilo Default y añade explicaciones breves de por qué hizo las opciones que hizo. Cada explicación aparece en la conversación, antes o después del código sobre el que trata, en un bloque etiquetado como `Insight`. Las explicaciones no se escriben en sus archivos como comentarios.

Un bloque `Insight` contiene dos o tres puntos sobre su base de código o el código que escribió Claude, como este después de añadir un punto final de API:

```text theme={null}
★ Insight ─────────────────────────────────────
- Every route in this repo goes through the withAuth wrapper, so the new endpoint gets session checks without its own middleware.
- Rate limits are set per route in limits.ts, which is why this change adds an entry there rather than a global default.
─────────────────────────────────────────────────
```

<h3 id="learning">
  Learning
</h3>

En el estilo Learning, Claude añade los mismos bloques `Insight` que el [estilo Explanatory](#explanatory) y también le pide que escriba parte del código. Claude maneja la implementación rutinaria por sí mismo. Cuando llega a una pieza con una decisión de diseño real, como manejo de errores, una estructura de datos o lógica empresarial con más de un enfoque válido, deja algunas líneas para usted.

Claude marca el lugar con un comentario `TODO(human)` en el archivo, luego envía una solicitud que dice qué ya está construido, qué escribir y qué considerar:

```text theme={null}
● Learn by Doing

Context: The upload form is in place and calls validateFile() before accepting a file. Size and type checks work for images, but the switch statement has no handling for documents yet.

Your Task: In upload.js, implement the case "document" branch inside validateFile(). Look for TODO(human).

Guidance: Decide on a size limit for documents and whether the file extension has to match the MIME type. Return {valid: boolean, error?: string}.
```

Claude luego se detiene y espera. Escriba su código en el comentario `TODO(human)` y dígale a Claude cuándo haya terminado. Claude responde con un `Insight` sobre su código y continúa la tarea.

<h2 id="change-your-output-style">
  Cambiar su estilo de salida
</h2>

Elija un estilo con el comando, un menú o un archivo de configuración. El comando y ambos menús guardan su selección en `.claude/settings.local.json` en el [nivel de proyecto local](/docs/es/settings).

* **Comando `/output-style`**: ejecute `/output-style <style>` para cambiar, por ejemplo `/output-style concise`. Sin argumentos, el comando enumera los estilos que puede elegir y marca el actual.

  El comando también funciona en [modo no interactivo](/docs/es/headless) y sesiones de Agent SDK, y desde la aplicación móvil o web a través de [Control Remoto](/docs/es/remote-control#limitations), donde puede enumerar y seleccionar solo [estilos integrados](#built-in-output-styles). Requiere Claude Code v2.1.269 o posterior.
* **Menú de terminal**: ejecute `/config` y seleccione **Output style** para elegir un estilo de un menú.
* **Extensión de VS Code**: abra el [menú de comandos](/docs/es/vs-code#use-the-prompt-box) con `/` y seleccione **Output styles** para elegir un estilo, incluidos sus estilos personalizados. Requiere Claude Code v2.1.257 o posterior.
* **Aplicación de escritorio**: establezca el campo `outputStyle` en un archivo de configuración, por ejemplo `.claude/settings.local.json`, el archivo que escribe el menú de terminal. Cuando ejecute `/config` allí, Claude Code [abre **Settings > Claude Code**](/docs/es/desktop#what%E2%80%99s-not-available-in-desktop) en lugar de un menú.

Para establecer un estilo sin el menú, edite el campo `outputStyle` directamente en un archivo de configuración:

```json theme={null}
{
  "outputStyle": "Explanatory"
}
```

El valor distingue mayúsculas de minúsculas, así que escriba los nombres integrados como `Proactive`, `Concise`, `Explanatory` y `Learning`. Un valor que no coincida exactamente con un nombre de estilo, como `explanatory`, le proporciona el estilo Predeterminado. El comando `/output-style` ignora mayúsculas y minúsculas.

Para hacer que un estilo sea su predeterminado en todos los proyectos, establezca `outputStyle` en `~/.claude/settings.json`. Los archivos de configuración propios de un proyecto [tienen precedencia](/docs/es/settings#settings-precedence) sobre ese valor.

Cuando cambia de estilos a mitad de sesión, Claude utiliza el nuevo estilo a partir de su siguiente mensaje. Para lo que cuesta ese primer mensaje en almacenamiento en caché de prompts, consulte [Cambiar el estilo de salida](/docs/es/prompt-caching#changing-output-style). Antes de v2.1.251, el nuevo estilo se aplicaba solo después de ejecutar `/clear` o iniciar una nueva sesión.

<h2 id="create-a-custom-output-style">
  Crear un estilo de salida personalizado
</h2>

Un estilo de salida personalizado es un archivo Markdown: frontmatter para metadatos, luego las instrucciones para Claude.

En la extensión de VS Code, también puede crear el archivo desde el [menú **Output styles**](/docs/es/vs-code#use-the-prompt-box) en lugar de escribirlo manualmente. Esto requiere Claude Code v2.1.261 o posterior.

<Steps>
  <Step title="Crear un archivo Markdown">
    Guárdelo en uno de tres niveles. El nombre del archivo se convierte en el nombre del estilo a menos que establezca `name` en el frontmatter.

    * Usuario: `~/.claude/output-styles`
    * Proyecto: `.claude/output-styles`
    * Política administrada: `.claude/output-styles` dentro del [directorio de configuración administrada](/docs/es/managed-settings#delivery-mechanisms)

    Los estilos de salida del proyecto se cargan desde cada `.claude/output-styles/` entre el directorio de trabajo y la raíz del repositorio. Cuando más de uno de estos directorios anidados define un estilo con el mismo nombre, Claude Code utiliza el más cercano al directorio de trabajo.
  </Step>

  <Step title="Agregar frontmatter e instrucciones">
    Decida si desea mantener las instrucciones de ingeniería de software de Claude Code. Establezca `keep-coding-instructions: true` si está cambiando cómo Claude se comunica pero aún desea que codifique de la misma manera. Déjelo fuera si Claude no estará haciendo ingeniería de software.

    Este ejemplo encabeza cada explicación con un diagrama mientras mantiene el comportamiento de codificación de Claude:

    ```markdown theme={null}
    ---
    name: Diagrams first
    description: Lead every explanation with a diagram
    keep-coding-instructions: true
    ---

    When explaining code, architecture, or data flow, start with a Mermaid diagram showing the structure, then explain in prose.

    ## Diagram conventions

    Use `flowchart TD` for control flow and `sequenceDiagram` for request paths. Keep diagrams under 15 nodes.
    ```
  </Step>

  <Step title="Cambiar a su estilo">
    Ejecute `/output-style <style>` en la terminal, o ejecute `/config` y seleccione su estilo bajo **Output style**. Claude utiliza el nuevo estilo a partir de su próximo mensaje. En la terminal, Claude Code lee archivos de estilo cuando se inicia, por lo que si crea o edita uno durante una sesión en ejecución, reinicie Claude Code para aplicar el cambio.
  </Step>
</Steps>

[Plugins](/docs/es/plugins/manifest-reference) también pueden enviar estilos de salida en un directorio `output-styles/`.

<h3 id="frontmatter">
  Referencia de frontmatter
</h3>

Configure un estilo de salida con [frontmatter](/docs/es/glossary#frontmatter) YAML entre marcadores `---` en la parte superior del archivo. Todos los campos son opcionales, y los nombres de los campos utilizan palabras en minúsculas separadas por guiones. Un campo mal escrito se ignora sin un error. Si el YAML no se analiza, el estilo aún se carga bajo su nombre de archivo sin campos establecidos; ejecute `claude --debug` para ver el error de análisis.

| Campo                      | Requerido | Descripción                                                                                                                                                                                                                                                                                                                                            |
| :------------------------- | :-------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                     | No        | Nombre del estilo de salida, mostrado en el selector `/config`. Predeterminado: el nombre del archivo                                                                                                                                                                                                                                                  |
| `description`              | No        | Descripción del estilo de salida, mostrada en el selector `/config`                                                                                                                                                                                                                                                                                    |
| `keep-coding-instructions` | No        | Establezca en `true` para mantener las instrucciones integradas de ingeniería de software de Claude Code junto con su estilo. Predeterminado: `false`                                                                                                                                                                                                  |
| `force-for-plugin`         | No        | Solo estilos de salida de plugins. Establezca en `true` para aplicar este estilo automáticamente siempre que el plugin esté habilitado, sin requerir que los usuarios lo seleccionen. Anula la configuración `outputStyle` del usuario. Si varios plugins habilitados establecen esto, Claude Code utiliza el primero cargado. Predeterminado: `false` |

<span id="comparisons-to-related-features" />

<h2 id="choose-between-an-output-style-and-other-features">
  Elige entre un estilo de salida y otras características
</h2>

Un estilo de salida se aplica a cada respuesta en una sesión. Es una instrucción que Claude sigue, por lo que nada la impone. Cuando lo que desea es más específico que cada respuesta, u tiene que suceder sin falta, otra característica se ajusta mejor.

Esta tabla hace coincidir lo que desea con la característica que lo hace:

| Lo que desea                                                                                                      | Usar                                                              | Por qué se ajusta                                                                                                                |
| :---------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| Cada respuesta en una voz, longitud o formato determinados, o Claude en un rol diferente                          | Un estilo de salida                                               | Se aplica a toda la sesión, y cambia estilos con un comando                                                                      |
| Claude debe conocer las convenciones, comandos y estructura de su proyecto                                        | [CLAUDE.md](/docs/es/memory)                                           | Contiene lo que Claude debe saber sobre la base de código, y permanece cargado cualquiera que sea el estilo que elija            |
| Instrucciones para un tipo de tarea, como una lista de verificación de lanzamiento o un procedimiento de revisión | Una [skill](/docs/es/skills)                                           | Claude la carga solo cuando la invoca o la tarea coincide, por lo que no da forma a respuestas no relacionadas                   |
| Algo que suceda cada vez sin excepción, como formatear después de cada edición o bloquear un comando              | Un [hook](/docs/es/hooks-guide)                                        | Claude Code ejecuta un hook en sí mismo en un evento del ciclo de vida, por lo que no depende de que Claude siga una instrucción |
| Un asistente con sus propias instrucciones, modelo y herramientas para una tarea enfocada                         | Un [subagent](/docs/es/sub-agents)                                     | Se ejecuta en un contexto separado con su propio mensaje del sistema y devuelve un resumen a su conversación                     |
| Una adición a las instrucciones de Claude que pasa cuando inicia Claude Code                                      | [`--append-system-prompt`](/docs/es/cli-reference#system-prompt-flags) | Se añade al mensaje del sistema sin eliminar nada                                                                                |

Estas características se combinan. Por ejemplo, puede usar CLAUDE.md para lo que Claude debe saber, un estilo de salida para cómo responde, y un hook para cualquier cosa que tenga que ser garantizada. [Extienda Claude Code](/docs/es/features-overview) compara el resto de las características de la extensión.

<h2 id="how-output-styles-work">
  Cómo funcionan los estilos de salida
</h2>

Un estilo de salida cambia las instrucciones que Claude Code le da a Claude.

* Claude Code envía las instrucciones del estilo activo con cada solicitud.
* Los estilos de salida personalizados omiten las instrucciones de ingeniería de software integradas de Claude Code, como cómo delimitar cambios, escribir comentarios y verificar el trabajo, a menos que `keep-coding-instructions` esté configurado en `true`.

Los estilos de salida se aplican a la conversación principal y a un [fork](/docs/es/sub-agents#fork-the-current-conversation), que hereda la conversación completa y el mensaje del sistema del padre. Otros [subagentes ejecutan su propio mensaje del sistema](/docs/es/sub-agents#what-loads-at-startup), por lo que los estilos no cambian cómo responden.

El uso de tokens depende del estilo. Las instrucciones de un estilo agregan tokens de entrada, aunque el almacenamiento en caché de solicitudes reduce este costo después de la primera solicitud en una sesión.

Los estilos Explanatory y Learning integrados producen respuestas más largas que Default por diseño, lo que aumenta los tokens de salida. El estilo Concise hace lo opuesto al instruir a Claude que mantenga las respuestas cortas por defecto. Para estilos personalizados, el uso de tokens de salida depende de lo que sus instrucciones le digan a Claude que produzca.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Settings](/docs/es/settings): donde vive el campo `outputStyle` y cómo funciona la precedencia de configuración
* [Permission modes](/docs/es/permission-modes): cómo el estilo Proactive se compara con el modo automático
* [Plugins](/docs/es/plugins/overview): empaquete y distribuya estilos de salida junto con skills, hooks y agents
* [Debug your configuration](/docs/es/debug-your-config): diagnostique por qué un estilo de salida no está surtiendo efecto
