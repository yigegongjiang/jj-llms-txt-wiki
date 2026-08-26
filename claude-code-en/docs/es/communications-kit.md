> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Kit de comunicaciones

> Anuncios de lanzamiento, mensajes de campaña de goteo y respuestas de preguntas frecuentes para implementar Claude Code en su organización de ingeniería.

Esta página es para administradores y líderes de ingeniería que implementan Claude Code en un equipo. Proporciona anuncios de lanzamiento listos para copiar, una campaña de goteo de consejos y trucos, y respuestas de una línea para las preguntas que le harán con más frecuencia.

<Note>
  Trate todo aquí como borrador, no como copia final. Reescriba cada mensaje con la voz de su organización, cambie las tareas de ejemplo por errores y módulos reales de su propio código, y reemplace los `[marcadores de posición entre corchetes]` antes de enviar. Los anuncios que impulsan la adopción son los que parecen escritos por alguien de su empresa.
</Note>

<h2 id="launch-communications">
  Comunicaciones de lanzamiento
</h2>

Un anuncio en dos formatos, más dos variantes opcionales. Elija el que mejor se ajuste a su implementación y reescriba a partir de ahí.

<h3 id="before-you-send">
  Antes de enviar
</h3>

Trabaje en esta lista de verificación antes de que salga el anuncio. Cada elemento cierra una brecha que de otro modo se convierte en un hilo de soporte en el día del lanzamiento.

| Elemento                                                                                              | Por qué importa                                                                                                                               |
| ----------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| Canal `#claude-code` creado y vinculado en el mensaje                                                 | Proporciona un lugar donde las preguntas pueden llegar                                                                                        |
| Comando de instalación probado en al menos una máquina en su entorno                                  | Detecta problemas de proxy o firewall antes de que todos los golpeen a la vez                                                                 |
| Enlace de seguridad y manejo de datos listo ([Uso de datos](/docs/es/data-usage) o su equivalente interno) | "¿Dónde va mi código?" será la primera respuesta                                                                                              |
| Una tarea concreta elegida, un error real o archivo en su código                                      | Los ejemplos genéricos no convierten; "arreglar la prueba inestable en `auth_test.go`" sí                                                     |
| Un propietario nombrado para el canal durante las primeras 48 horas                                   | Las preguntas sin respuesta en el día del lanzamiento matan el impulso                                                                        |
| Un patrocinador de la suite C alineado para enviar o co-firmar el anuncio                             | Los lanzamientos enviados por ejecutivos ven consistentemente una adopción más alta en la primera semana que los enviados por administradores |

<h3 id="the-announcement">
  El anuncio
</h3>

Use esto como su mensaje de implementación estándar en toda la organización. Cubre qué es Claude Code, proporciona una ruta de instalación de dos minutos, ofrece a los lectores una tarea concreta para probar y responde "¿dónde va mi código?" antes de que alguien tenga que preguntar.

<Tabs>
  <Tab title="Correo electrónico">
    ```text theme={null}
    Asunto: Claude Code está activo para [Ingeniería / su equipo]

    Equipo,

    A partir de hoy tiene acceso a Claude Code, un agente de codificación de IA que se ejecuta en
    su terminal, lee su código real y trabaja en tareas reales de principio a fin: depuración, refactores, pruebas, PRs. No es autocompletado y no es
    una ventana de chat. Edita archivos, ejecuta sus comandos y pide permiso
    antes de hacer algo arriesgado.

    Comience en dos minutos:

        curl -fsSL https://claude.ai/install.sh | bash
        cd <su-repositorio>
        claude

    Luego ejecute /init una vez. Claude lee su proyecto y escribe un CLAUDE.md con
    sus comandos de compilación y convenciones, para que deje de re-explicar lo básico.

    Luego intente uno de estos en el repositorio en el que ya está:

      - "La prueba en [archivo] es inestable. Averigua por qué y arréglalo"
      - "Camina conmigo a través de cómo [módulo] maneja [X]"
      - "Mira mi diferencia de trabajo y dime qué es arriesgado antes de que lo envíe"

    Dónde va su código: Claude Code se ejecuta en su terminal y habla directamente
    con la API de Anthropic, sin servidores de terceros en el medio. Pide permiso antes de
    editar archivos o ejecutar comandos. Bajo nuestro acuerdo Enterprise, Anthropic
    no utiliza su código o indicaciones para entrenar sus modelos.
    Detalles: https://code.claude.com/docs/es/data-usage
             https://code.claude.com/docs/es/security

    Dónde ir con preguntas: #claude-code. [Nombre del propietario] lo está monitoreando
    esta semana.

    - [Nombre]

    P.D. ¿Prefiere su editor? Hay una extensión de VS Code y un complemento de JetBrains.
    El mismo agente, sin terminal requerida.
    ```
  </Tab>

  <Tab title="Slack o Teams">
    ```markdown theme={null}
    🚀 *Claude Code está activo para [equipo]*

    Agente de codificación de IA, se ejecuta en su terminal, lee su repositorio, hace trabajo real:
    errores, refactores, pruebas, PRs. Pide permiso antes de tocar cualquier cosa.

    `curl -fsSL https://claude.ai/install.sh | bash` → `cd su-repositorio` → `claude`

    *Primera cosa a probar* → ejecute `/init`, luego: "la prueba en [archivo] es inestable,
    averigua por qué y arréglalo."

    🔒 Se ejecuta en su terminal, habla solo con la API de Anthropic. Bajo nuestro
    plan Enterprise su código e indicaciones no se utilizan para entrenar modelos.
    Uso de datos → https://code.claude.com/docs/es/data-usage

    📚 Inicio rápido · VS Code · Curso gratuito de 1 hora
       https://code.claude.com/docs/es/quickstart
       https://code.claude.com/docs/es/vs-code
       https://anthropic.skilljar.com/claude-code-in-action

    Preguntas → este hilo. [Propietario] está a cargo.
    ```
  </Tab>
</Tabs>

<h3 id="executive-sponsor-variant">
  Variante de patrocinador ejecutivo
</h3>

Envíe esto desde su ejecutivo patrocinador, como el CTO, CIO o SVP de Ingeniería, bajo su nombre y desde su cuenta. Los lanzamientos que salen bajo el nombre de un ejecutivo ven consistentemente tasas de apertura más altas y activación más rápida en la primera semana que el mismo mensaje de un administrador o equipo de herramientas. Señala una prioridad de la empresa en lugar de un experimento opcional.

Esta versión se reduce deliberadamente a una solicitud: instálelo y ejecútelo en una tarea real. El trabajo del ejecutivo es hacer que la solicitud llegue; el anuncio estándar y `#claude-code` manejan el cómo.

<Tabs>
  <Tab title="Correo electrónico">
    ```text theme={null}
    Asunto: Una cosa que me gustaría que cada ingeniero probara esta semana

    Equipo,

    Hemos activado Claude Code para toda la ingeniería. Es un agente de IA
    que funciona directamente en su terminal, en su código real, y los
    resultados tempranos de los equipos que ya lo usan son lo suficientemente sólidos como para que quiera
    que todos lo usen esta semana.

    Le pido diez minutos:

        curl -fsSL https://claude.ai/install.sh | bash
        cd <su-repositorio>
        claude

    Luego dele una tarea real: el error que ha estado postergando, o "camina conmigo
    a través de cómo funciona [módulo]."

    Esa es toda la solicitud. [Nombre del propietario] y el equipo están en #claude-code para
    cualquier cosa que encuentre en el camino.

    - [Nombre del ejecutivo]
      [Título]
    ```
  </Tab>

  <Tab title="Slack o Teams">
    ```markdown theme={null}
    📣 *De [Nombre del ejecutivo]: una cosa a probar esta semana*

    Hemos activado *Claude Code* para toda la ingeniería. Los resultados tempranos son
    lo suficientemente sólidos como para que le pida a todos que le dediquen diez minutos en trabajo real esta semana.

    `curl -fsSL https://claude.ai/install.sh | bash` → `cd su-repositorio` →
    `claude` → dele una tarea real.

    Eso es todo. Preguntas → #claude-code.
    ```
  </Tab>
</Tabs>

<h3 id="pilot-group-variant">
  Variante de grupo piloto
</h3>

Use para una implementación por fases. Envíe solo a la cohorte piloto.

```text theme={null}
Asunto: Está en el piloto de Claude Code

[Nombre / equipo],

Está en la primera ola de Claude Code en [empresa]. Elegimos este grupo
porque lo pondrá en problemas reales y nos dirá la verdad al respecto.

La solicitud: úselo en al menos una tarea real esta semana, luego deje una nota en
#claude-code-pilot cubriendo qué funcionó, qué fue molesto y qué
lo sorprendió. Esa retroalimentación decide cómo lo implementamos para todos los demás.

[Continúe con "Comience en dos minutos" del anuncio estándar]

Una cosa extra para pilotos: en su primer cambio de múltiples archivos, presione Shift+Tab
hasta que vea "plan". Claude establecerá exactamente qué intenta hacer
antes de tocar un archivo. Es la forma más rápida de calibrar cuánto
confiar en él.
```

<h3 id="champion-recruitment-dm">
  DM de reclutamiento de campeones
</h3>

Después del lanzamiento, envíe un DM a las dos o tres personas más activas en `#claude-code`.

```text theme={null}
Hola [nombre], tus publicaciones en #claude-code están haciendo más por la adopción que mi
anuncio. Un par de personas me dijeron que tu [hilo / captura de pantalla]
fue la razón por la que realmente lo intentaron.

¿Quieres hacer eso semi-oficial? Poco esfuerzo: principalmente sigue publicando lo que
estás publicando, más primer acceso a nuevas características y una línea directa al
equipo de Anthropic. Puedo compartir un pequeño manual si estás dentro.
```

<h2 id="tips-and-tricks-campaign">
  Campaña de consejos y trucos
</h2>

Mensajes de Slack o Teams listos para pegar diseñados para impulsar la activación de características después del lanzamiento. Cada uno sigue el mismo patrón: un gancho, la recompensa, un indicador "pruébalo ahora" y un enlace de documentación. Distribúyalos uno o dos a la semana en `#claude-code`, o elija los pocos que coincidan con las brechas de su equipo. Se mantienen solos sin orden requerido.

Copie el cuerpo del mensaje de cada bloque directamente en Slack o Teams. Reemplace `[marcadores de posición entre corchetes]` antes de enviar.

<h3 id="get-started">
  Comenzar
</h3>

**Elegir el modelo correcto**

```markdown theme={null}
🎯 *Consejo: Haga coincidir el modelo con el momento*

Usar Opus para arreglar una falta de ortografía quema computación. Usar Haiku para un refactor
de 12 archivos es pedir un re-hacer.

Claude Code se ejecuta en los mismos modelos que la aplicación Claude, y puede cambiar
a mitad de sesión. *Sonnet* es el caballo de batalla predeterminado para trabajo de características cotidianas,
errores, pruebas y revisiones. Recurra a *Opus* en refactores grandes, depuración complicada,
o cualquier cosa de alto riesgo. Baje a *Haiku* para preguntas rápidas,
formato y ediciones mecánicas donde la velocidad gana. *Fable 5* es el modelo más
capaz para sus tareas más difíciles y de mayor duración; no es el
predeterminado, así que selecciónelo con `/model fable`, y tenga en cuenta que el contenido de ciberseguridad y
biología se retrocede automáticamente a Opus.

*Pruébalo ahora:* escribe `/model` y elige Sonnet si aún no lo has hecho. Es
el predeterminado correcto para la mayoría de tareas.

📖 Configuración de modelo → https://code.claude.com/docs/es/model-config
```

| Modelo  | Mejor para                                                                                                                                                                                             |
| ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Fable 5 | Las tareas más difíciles y de mayor duración. Solo opción: selecciónelo con `/model fable`. El contenido de ciberseguridad o biología [se retrocede a Opus](/docs/es/model-config#automatic-model-fallback) |
| Opus    | Refactores a gran escala, depuración compleja, decisiones de arquitectura, cambios de alto riesgo                                                                                                      |
| Sonnet  | Trabajo de características cotidianas, corrección de errores, pruebas, documentación, revisión de código. Predeterminado recomendado.                                                                  |
| Haiku   | Preguntas rápidas, formato, ediciones mecánicas, iteración rápida                                                                                                                                      |

**Victorias rápidas para probar primero**

```markdown theme={null}
🚀 *Consejo: Tres cosas a probar en tus primeros 10 minutos*

¿Instaló Claude Code pero no está seguro de qué preguntarle realmente? Comience con lo
que lo ha estado molestando toda la semana.

  - Arregla algo molesto: "la prueba en [archivo] es inestable, averigua por qué"
  - Oriéntate en código que no escribiste: "camina conmigo a través de cómo funciona [módulo]"
  - Verifica la cordura antes de enviar: "mira mi diferencia de trabajo y dime qué
    se ve arriesgado"

Ninguno de estos necesita configuración. Solo `cd` en su repositorio y ejecute `claude`.

*Pruébalo ahora:* elige el error que has estado evitando y pega el mensaje de error.

📖 Inicio rápido → https://code.claude.com/docs/es/quickstart
```

<h3 id="project-memory">
  Memoria del proyecto
</h3>

**`/init` y CLAUDE.md**

```markdown theme={null}
📁 *Consejo: Deja de re-explicar tu repositorio cada sesión*

¿Diciéndole a Claude "usamos pnpm, no npm" por quinta vez? Hay una
solución única.

Ejecute `/init` una vez por repositorio. Claude lee la estructura de su proyecto y escribe un
archivo CLAUDE.md con sus comandos de compilación, arquitectura y convenciones.
Cada sesión futura en ese repositorio comienza desde este archivo automáticamente. Manténgalo
bajo dos pantallas. Es una hoja de trucos, no documentación.

*Pruébalo ahora:* abre tu repositorio principal, ejecuta `claude`, escribe `/init`. Treinta
segundos, se amortiza en cada sesión después.

📖 CLAUDE.md y memoria del proyecto → https://code.claude.com/docs/es/memory
```

**Referencias @**

```markdown theme={null}
📎 *Consejo: Deja de pegar contenidos de archivos en el chat*

¿Copiar 200 líneas de un componente en tu indicación para que Claude pueda "verlo"?
No tienes que hacerlo.

Escribe `@` luego una ruta de archivo. Claude extrae el archivo directamente al contexto.
También funciona para directorios completos.

> los estilos en @src/components/Button.tsx se ven mal, verifica contra
> @docs/design-system.md

*Pruébalo ahora:* escribe `@` luego Tab. El autocompletado te muestra cada archivo al alcance.

📖 Referenciación de archivos → https://code.claude.com/docs/es/common-workflows
```

<h3 id="control-and-safety">
  Control y seguridad
</h3>

**Modos de permiso**

```markdown theme={null}
🛡️ *Consejo: Una pulsación de tecla entre "mirar pero no tocar" y "simplemente hazlo"*

A veces quieres que Claude pregunte antes de cada edición. A veces solo quieres
que lo envíe. No deberías tener que elegir uno para siempre.

*Shift+Tab* cicla a través de cuánta libertad obtiene Claude: *Manual* (el
valor de configuración `default`) pregunta antes de cada acción, *acceptEdits* permite que las ediciones de archivos y comandos comunes del sistema de archivos
fluyan mientras aún verifica antes de otros comandos de shell, y *plan*
propone cambios para tu aprobación antes de que se toque nada. El modo plan es
el constructor de confianza, así que comienza allí para cualquier cosa que toque múltiples archivos.

*Pruébalo ahora:* en tu próximo refactor, presiona Shift+Tab hasta que veas "plan",
luego describe el cambio. Obtendrás una propuesta completa antes de que un solo archivo se mueva.

📖 Modos de permiso → https://code.claude.com/docs/es/permissions
```

**Checkpointing y `/rewind`**

```markdown theme={null}
⏪ *Consejo: Hay un botón de deshacer para toda la conversación*

Claude fue por el camino equivocado hace tres turnos y ahora estás desenredándolo?
No tienes que arreglarlo hacia adelante.

`/rewind` retrocede a un punto anterior en la conversación, incluidos los
cambios de archivo que Claude hizo en el camino. El checkpointing es automático; no
configuras nada.

*Pruébalo ahora:* presiona *Esc* dos veces para abrir el menú de retroceso, o escribe `/rewind`.
Elige el punto antes de que las cosas se salieran del camino.

📖 Checkpointing → https://code.claude.com/docs/es/checkpointing
```

<h3 id="connect-your-tools">
  Conecta tus herramientas
</h3>

**Conectores MCP**

```markdown theme={null}
🔌 *Consejo: Deja que Claude lea tu rastreador de problemas para que no tengas que pegar tickets*

Copiar y pegar tickets de Jira en la terminal se siente como un paso atrás.
Lo es.

Un archivo de configuración (`.mcp.json` en la raíz de tu proyecto) conecta Claude a GitHub,
Jira, Linear, o cualquier rastreador que uses. Luego "¿cuál es el problema de mayor prioridad
asignado a mí?" y "adelante y arréglalo" suceden en la misma
conversación.

*Pruébalo ahora:* pregúntale a Claude "configura un conector MCP para [GitHub/Jira/Linear]
en este repositorio". Escribirá la configuración para ti.

📖 Conectores MCP → https://code.claude.com/docs/es/mcp
```

<h3 id="automate-your-workflows">
  Automatiza tus flujos de trabajo
</h3>

**Skills**

```markdown theme={null}
⚡ *Consejo: Convierte ese indicador que sigues reescribiendo en un comando*

¿Escribiste "resumir en qué trabajé hoy desde git log, formatearlo para standup"
tres veces esta semana? Ese es un comando de barra inclinada esperando suceder.

Un archivo SKILL.md en `.claude/skills/<nombre>/` se convierte en un indicador reutilizable; escribe
`/nombre` para ejecutarlo. Haz uno la segunda vez que escribas un indicador de múltiples pasos
que hayas escrito antes. Camino más fácil: pídele a Claude que lo haga por ti.

*Pruébalo ahora:* escribe "hazme un skill /standup que resuma en qué trabajé
hoy desde git log", luego ejecuta `/standup` mañana por la mañana.

📖 Skills → https://code.claude.com/docs/es/skills
```

**Hooks**

```markdown theme={null}
🔔 *Consejo: Recibe una notificación cuando tu refactor termine*

¿Sentado en tu escritorio viendo a Claude trabajar en una tarea larga? Tienes
cosas mejores que hacer durante esos ocho minutos.

Los hooks son comandos de shell que se disparan en eventos de Claude Code. Un hook Stop que
envía una notificación de escritorio significa que puedes iniciar un refactor largo, irte,
y recibir una notificación en el momento en que termina.

*Pruébalo ahora:* pregúntale a Claude "agrega un hook Stop que envíe una notificación de escritorio
cuando termines". Escribirá el script y lo conectará.

📖 Guía de hooks → https://code.claude.com/docs/es/hooks-guide
```

<h3 id="day-to-day-development">
  Desarrollo día a día
</h3>

**Capturas de pantalla e imágenes**

```markdown theme={null}
📸 *Consejo: Deja de describir el diálogo de error. Solo muéstralo.*

¿Escribiendo "hay una caja roja que dice algo sobre una referencia nula
y está apuntando a la línea 47-ish"? Captura de pantalla.

Arrastra una captura de pantalla directamente a la terminal y Claude la ve: diálogos de error,
maquetas de UI, fotos de pizarra, exportaciones de Figma. *Ctrl+V* pega desde
el portapapeles (usa Ctrl+V en macOS también, no Cmd+V).

*Pruébalo ahora:* la próxima vez que algo visual se rompa, captura de pantalla y pégalo
directamente en el indicador. Luego solo escribe "¿qué está mal aquí?"

📖 Trabajar con imágenes → https://code.claude.com/docs/es/common-workflows
```

**Flujos de trabajo de Git**

```markdown theme={null}
🌿 *Consejo: Delega toda la ceremonia de git*

La corrección tomó 5 minutos. El mensaje de commit, rama y descripción de PR
tomaron 15. Esa proporción es incorrecta.

Claude maneja el flujo completo de git: commits con mensajes convencionales,
ramas, PRs con resúmenes adecuados. Una solicitud: "arregla el off-by-one, commit
con un mensaje de commit convencional, y abre un PR." ¿Revisando el trabajo de alguien más?
Pega la URL del PR y pídele a Claude que te camine a través del diff.

*Pruébalo ahora:* después de tu próxima corrección, en lugar de cambiar a tu cliente de git,
solo escribe "commit esto con un buen mensaje y abre un PR".

📖 Creación de solicitudes de extracción → https://code.claude.com/docs/es/common-workflows
```

<h3 id="share-and-scale">
  Compartir y escalar
</h3>

**Plugins**

```markdown theme={null}
📦 *Consejo: Alguien probablemente ya construyó esa skill*

¿A punto de pasar una hora construyendo un comando `/deploy`? Verifica si
ya existe.

Las skills se agrupan y comparten como plugins. `/plugin` explora lo que está
disponible e instala en un paso. Cinco minutos de exploración pueden ahorrar una hora de construcción.

*Pruébalo ahora:* escribe `/plugin` y desplázate. Encontrarás al menos una
cosa que no sabías que querías.

📖 Plugins → https://code.claude.com/docs/es/plugins
```

<h3 id="security-and-admin">
  Seguridad y administración
</h3>

**Arquitectura de seguridad**

```markdown theme={null}
🔐 *Consejo: La respuesta a "¿es esto seguro?" para la próxima vez que te lo pregunten*

Alguien en tu equipo va a preguntar "espera, ¿dónde va mi código?"
Aquí está la versión corta que puedes pegar.

Primero permiso por diseño. Cada edición de archivo, comando de shell y llamada externa
está controlada por tu aprobación. El CLI se ejecuta en tu terminal y habla
directamente con la API de Anthropic, sin servidores de terceros, y soporta
sandboxing opcional a nivel del SO para comandos de shell. Bajo nuestro plan Enterprise,
Anthropic no utiliza tu código o indicaciones para entrenar sus modelos.

*Pruébalo ahora:* guarda estos dos enlaces para la próxima vez que surja la pregunta.
Responden la mayoría de preguntas de revisión de seguridad.

📖 https://code.claude.com/docs/es/security
📖 https://code.claude.com/docs/es/data-usage
```

**Mejores prácticas**

```markdown theme={null}
✅ *Consejo: Los 4 hábitos que separan "lo intenté una vez" de "lo uso diariamente"*

La mayoría de las personas que rebotan en Claude Code se saltaron uno de estos. La mayoría de las personas
que se quedan hicieron los cuatro en la primera semana.

  - Comienza en modo plan para cualquier cosa que toque múltiples archivos
  - Ejecuta /init temprano; el contexto se compone
  - Revisa diffs antes de hacer commit; Claude puede estar confiadamente equivocado
  - Verifica cambios que toquen rutas críticas; trátalo como un junior afilado,
    no como un oráculo

*Pruébalo ahora:* si solo has hecho uno o dos de estos, elige el que te falta
y hazlo en tu próxima tarea. Publica qué cambió en #claude-code.

📖 Mejores prácticas → https://code.claude.com/docs/es/best-practices
```

<h2 id="quick-reference">
  Referencia rápida
</h2>

<h3 id="faq-responses">
  Respuestas de preguntas frecuentes
</h3>

Respuestas de una línea para las preguntas que le harán con más frecuencia.

| Pregunta                              | Respuesta                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| "¿Funciona en VS Code?"               | Sí. Hay una extensión de VS Code y un complemento de JetBrains con las mismas características, integradas en su editor. [VS Code →](/docs/es/vs-code)                                                                                                                                                                                                                                                                                                                                |
| "¿Tengo que configurar algo primero?" | No. Instale, luego ejecute `claude` en cualquier repositorio. Ejecute `/init` una vez y está listo. [Inicio rápido →](/docs/es/quickstart)                                                                                                                                                                                                                                                                                                                                           |
| "¿Dónde va mi código?"                | El CLI se ejecuta en su terminal y envía contexto a la API de Anthropic para inferencia, sin servidores de terceros. Bajo su plan Enterprise, su código e indicaciones no se utilizan para entrenar modelos. [Uso de datos →](/docs/es/data-usage)                                                                                                                                                                                                                                   |
| "¿Puede ver todo mi repositorio?"     | Lee lo que le da acceso. Las lecturas de archivos dentro de su directorio de trabajo no solicitan; los indicadores de permiso controlan ediciones, comandos de shell no solo lectura, y lecturas de herramientas de archivos fuera de ese directorio. Un conjunto integrado de comandos de shell de solo lectura como `ls` y `cat` se ejecuta sin solicitar; restrínjalo con [reglas de sandbox `denyRead`](/docs/es/sandboxing#filesystem-isolation). [Permisos →](/docs/es/permissions) |
| "¿Cómo es esto diferente de Copilot?" | Copilot autocompletea líneas. Claude Code es un agente que lee archivos, ejecuta comandos y realiza ediciones de múltiples archivos. [Descripción general →](/docs/es/overview)                                                                                                                                                                                                                                                                                                      |
| "¿Qué debería probar primero?"        | Un error que ha estado postergando porque es tedioso. "La prueba en \[archivo] es inestable, averigua por qué." [Inicio rápido →](/docs/es/quickstart)                                                                                                                                                                                                                                                                                                                               |

<h3 id="prompt-templates">
  Plantillas de indicaciones
</h3>

Comparta estos indicadores de inicio con ingenieros que han instalado pero no están seguros de qué preguntar. Cada uno está redactado de la manera en que se escribiría en una sesión real; reemplace las piezas entre corchetes con archivos de su propio repositorio.

| Tarea                         | Indicación                                                                                      |
| ----------------------------- | ----------------------------------------------------------------------------------------------- |
| Arreglar un error             | "las pruebas en \[archivo] están fallando, averigua por qué y arréglalo"                        |
| Entender código               | "camina conmigo a través de cómo funciona \[módulo], luego dime dónde está el punto de entrada" |
| Refactor seguro               | "refactoriza \[módulo] a \[objetivo], usa modo plan para que pueda revisar primero"             |
| Escribir pruebas              | "escribe pruebas para \[archivo] que cubran los casos extremos alrededor de \[escenario]"       |
| Revisar antes de hacer commit | "mira mi diferencia de trabajo y dime qué se ve arriesgado"                                     |
| Abrir un PR                   | "arregla \[problema], escribe un commit convencional, y abre un PR con un resumen"              |
| Hacer una skill               | "hazme una skill /ship que ejecute pruebas y lint antes de hacer commit"                        |
| Depurar un stack trace        | "aquí está el stack trace, encuentra la causa raíz, no solo lo cubras"                          |

<Tip>
  Claude Code se envía frecuentemente. Verifique los detalles específicos de la versión contra la [página de inicio de documentación](/docs/es/overview) antes de distribuir internamente.
</Tip>
