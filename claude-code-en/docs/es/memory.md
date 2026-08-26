> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Cómo Claude recuerda su proyecto

> Proporcione a Claude instrucciones persistentes con archivos CLAUDE.md, y permita que Claude acumule aprendizajes automáticamente con auto memory.

Cada sesión de Claude Code comienza con una ventana de contexto nueva. Dos mecanismos llevan el conocimiento entre sesiones:

* **Archivos CLAUDE.md**: instrucciones que usted escribe para dar a Claude contexto persistente
* **Auto memory**: notas que Claude escribe por sí mismo basadas en sus correcciones y preferencias

Esta página cubre cómo:

* [Escribir y organizar archivos CLAUDE.md](#claude-md-files)
* [Limitar reglas a tipos de archivo específicos](#organize-rules-with-claude/rules/) con `.claude/rules/`
* [Configurar auto memory](#auto-memory) para que Claude tome notas automáticamente
* [Solucionar problemas](#troubleshoot-memory-issues) cuando las instrucciones no se siguen

<h2 id="claude-md-vs-auto-memory">
  CLAUDE.md vs auto memory
</h2>

Claude Code tiene dos sistemas de memoria complementarios. Ambos se cargan al inicio de cada conversación. Claude los trata como contexto, no como configuración forzada. Para bloquear una acción independientemente de lo que Claude decida, use un [hook PreToolUse](/docs/es/hooks-guide) en su lugar. Cuanto más específicas y concisas sean sus instrucciones, más consistentemente Claude las seguirá.

|                      | Archivos CLAUDE.md                                                       | Auto memory                                                                          |
| :------------------- | :----------------------------------------------------------------------- | :----------------------------------------------------------------------------------- |
| **Quién lo escribe** | Usted                                                                    | Claude                                                                               |
| **Qué contiene**     | Instrucciones y reglas                                                   | Aprendizajes y patrones                                                              |
| **Alcance**          | Proyecto, usuario u organización                                         | Por repositorio, compartido entre worktrees                                          |
| **Se carga en**      | Cada sesión                                                              | Cada sesión (primeras 200 líneas o 25KB)                                             |
| **Usar para**        | Estándares de codificación, flujos de trabajo, arquitectura del proyecto | Comandos de compilación, información de depuración, preferencias que Claude descubre |

Use archivos CLAUDE.md cuando quiera guiar el comportamiento de Claude. Auto memory permite que Claude aprenda de sus correcciones sin esfuerzo manual.

Los subagents también pueden mantener su propia auto memory. Consulte [configuración de subagent](/docs/es/sub-agents#enable-persistent-memory) para obtener detalles.

<h2 id="claude-md-files">
  Archivos CLAUDE.md
</h2>

Los archivos CLAUDE.md son archivos markdown que dan a Claude instrucciones persistentes para un proyecto, su flujo de trabajo personal o toda su organización. Usted escribe estos archivos en texto plano; Claude los lee al inicio de cada sesión.

<h3 id="when-to-add-to-claude-md">
  Cuándo agregar a CLAUDE.md
</h3>

Trate CLAUDE.md como el lugar donde escribe lo que de otro modo tendría que re-explicar. Agregue a él cuando:

* Claude comete el mismo error una segunda vez
* Una revisión de código detecta algo que Claude debería haber sabido sobre esta base de código
* Usted escribe la misma corrección o aclaración en el chat que escribió la sesión anterior
* Un nuevo compañero de equipo necesitaría el mismo contexto para ser productivo

Manténgalo en hechos que Claude debe retener en cada sesión: comandos de compilación, convenciones, diseño del proyecto, reglas "siempre haz X". Si una entrada es un procedimiento de múltiples pasos o solo importa para una parte de la base de código, muévala a un [skill](/docs/es/skills) o una [regla con alcance de ruta](#organize-rules-with-claude/rules/) en su lugar. La [descripción general de extensiones](/docs/es/features-overview#build-your-setup-over-time) cubre cuándo usar cada mecanismo.

<h3 id="choose-where-to-put-claude-md-files">
  Elija dónde colocar los archivos CLAUDE.md
</h3>

Los archivos CLAUDE.md pueden vivir en varios lugares, cada uno con un alcance diferente. La tabla a continuación los enumera en orden de carga, desde el alcance más amplio hasta el más específico, por lo que una instrucción de proyecto aparece en contexto después de una instrucción de usuario.

| Alcance                        | Ubicación                                                                                                                                                             | Propósito                                                                | Ejemplos de casos de uso                                                                     | Compartido con                                        |
| ------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------- | ----------------------------------------------------- |
| **Política gestionada**        | • macOS: `/Library/Application Support/ClaudeCode/CLAUDE.md`<br />• Linux y WSL: `/etc/claude-code/CLAUDE.md`<br />• Windows: `C:\Program Files\ClaudeCode\CLAUDE.md` | Instrucciones de toda la organización gestionadas por TI/DevOps          | Estándares de codificación de la empresa, políticas de seguridad, requisitos de cumplimiento | Todos los usuarios de la organización                 |
| **Instrucciones del usuario**  | `~/.claude/CLAUDE.md`                                                                                                                                                 | Preferencias personales para todos los proyectos                         | Preferencias de estilo de código, atajos de herramientas personales                          | Solo usted (todos los proyectos)                      |
| **Instrucciones del proyecto** | `./CLAUDE.md` o `./.claude/CLAUDE.md`                                                                                                                                 | Instrucciones compartidas por el equipo para el proyecto                 | Arquitectura del proyecto, estándares de codificación, flujos de trabajo comunes             | Miembros del equipo a través del control de versiones |
| **Instrucciones locales**      | `./CLAUDE.local.md`                                                                                                                                                   | Preferencias personales específicas del proyecto; agregue a `.gitignore` | Sus URLs de sandbox, datos de prueba preferidos                                              | Solo usted (proyecto actual)                          |

Los archivos CLAUDE.md y CLAUDE.local.md en la jerarquía de directorios por encima del directorio de trabajo se cargan completamente al iniciar. Los archivos en subdirectorios se cargan bajo demanda cuando Claude lee archivos en esos directorios. Consulte [Cómo se cargan los archivos CLAUDE.md](#how-claude-md-files-load) para el orden de resolución completo.

Para proyectos grandes, puede dividir las instrucciones en archivos específicos de temas usando [reglas de proyecto](#organize-rules-with-claude/rules/). Las reglas le permiten limitar las instrucciones a tipos de archivo específicos o subdirectorios.

<h3 id="set-up-a-project-claude-md">
  Configure un CLAUDE.md de proyecto
</h3>

Un CLAUDE.md de proyecto puede almacenarse en `./CLAUDE.md` o `./.claude/CLAUDE.md`. Cree este archivo y agregue instrucciones que se apliquen a cualquiera que trabaje en el proyecto: comandos de compilación y prueba, estándares de codificación, decisiones arquitectónicas, convenciones de nomenclatura y flujos de trabajo comunes. Estas instrucciones se comparten con su equipo a través del control de versiones, así que enfóquese en estándares a nivel de proyecto en lugar de preferencias personales.

<Tip>
  Ejecute `/init` para generar un CLAUDE.md inicial automáticamente. Claude analiza su base de código y crea un archivo con comandos de compilación, instrucciones de prueba y convenciones de proyecto que descubre. Si ya existe un CLAUDE.md, `/init` sugiere mejoras en lugar de sobrescribirlo. Refine desde allí con instrucciones que Claude no descubriría por sí solo.

  Establezca `CLAUDE_CODE_NEW_INIT=1` para habilitar un flujo interactivo de múltiples fases. `/init` pregunta qué artefactos configurar: archivos CLAUDE.md, skills y hooks. Luego explora su base de código con un subagent, llena los vacíos mediante preguntas de seguimiento y presenta una propuesta revisable antes de escribir cualquier archivo.
</Tip>

<h3 id="write-effective-instructions">
  Escriba instrucciones efectivas
</h3>

Los archivos CLAUDE.md se cargan en la ventana de contexto al inicio de cada sesión, consumiendo tokens junto con su conversación. La [visualización de la ventana de contexto](/docs/es/context-window) muestra dónde se carga CLAUDE.md en relación con el resto del contexto de inicio. Debido a que son contexto en lugar de configuración forzada, cómo escribe las instrucciones afecta qué tan confiablemente Claude las sigue. Las instrucciones específicas, concisas y bien estructuradas funcionan mejor.

**Tamaño**: apunte a menos de 200 líneas por archivo CLAUDE.md. Los archivos más largos consumen más contexto y reducen la adherencia. Si sus instrucciones están creciendo mucho, use [reglas con alcance de ruta](#path-specific-rules) para que las instrucciones se carguen solo cuando Claude trabaje con archivos coincidentes. También puede dividir contenido en [importaciones](#import-additional-files) para organización, aunque los archivos importados aún se cargan e ingresan a la ventana de contexto al iniciar.

**Estructura**: use encabezados y viñetas de markdown para agrupar instrucciones relacionadas. Claude escanea la estructura de la misma manera que los lectores: las secciones organizadas son más fáciles de seguir que los párrafos densos.

**Especificidad**: escriba instrucciones que sean lo suficientemente concretas para verificar. Por ejemplo:

* "Usar indentación de 2 espacios" en lugar de "Formatear código correctamente"
* "Ejecutar `npm test` antes de hacer commit" en lugar de "Probar sus cambios"
* "Los controladores de API viven en `src/api/handlers/`" en lugar de "Mantener los archivos organizados"

**Consistencia**: si dos reglas se contradicen entre sí, Claude puede elegir una arbitrariamente. Revise sus archivos CLAUDE.md, archivos CLAUDE.md anidados en subdirectorios y archivos [`.claude/rules/`](#organize-rules-with-claude/rules/) periódicamente para eliminar instrucciones desactualizadas o conflictivas. En monorepos, use [`claudeMdExcludes`](#exclude-specific-claude-md-files) para omitir archivos CLAUDE.md de otros equipos que no sean relevantes para su trabajo.

<h3 id="import-additional-files">
  Importar archivos adicionales
</h3>

Los archivos CLAUDE.md pueden importar archivos adicionales usando la sintaxis `@path/to/import`. Los archivos importados se expanden y se cargan en contexto al iniciar junto con el CLAUDE.md que los referencia.

Se permiten rutas relativas y absolutas. Las rutas relativas se resuelven en relación con el archivo que contiene la importación, no con el directorio de trabajo. Los archivos importados pueden importar recursivamente otros archivos, con una profundidad máxima de cuatro saltos.

El análisis de importación omite espacios de código Markdown y bloques de código delimitados. Para mencionar una ruta en su CLAUDE.md sin importarla, envuélvala en backticks: escribir `` `@README` `` mantiene el texto literal, mientras que `@README` fuera de backticks importa el archivo.

Para incluir un README, package.json y una guía de flujo de trabajo, haga referencia a ellos con la sintaxis `@` en cualquier lugar de su CLAUDE.md:

```text theme={null}
Consulte @README para obtener una descripción general del proyecto y @package.json para los comandos npm disponibles para este proyecto.

# Instrucciones adicionales
- flujo de trabajo git @docs/git-instructions.md
```

Para preferencias personales privadas por proyecto que no desea registrar en el control de versiones, cree un `CLAUDE.local.md` en la raíz del proyecto. Se carga junto con `CLAUDE.md` y se trata de la misma manera. Agregue `CLAUDE.local.md` a su `.gitignore` para que no se confirme; ejecutar `/init` y elegir la opción personal hace esto por usted.

Si trabaja en múltiples git worktrees del mismo repositorio, un `CLAUDE.local.md` ignorado por git solo existe en el worktree donde lo creó. Para compartir instrucciones personales entre worktrees, importe un archivo desde su directorio de inicio en su lugar:

```text theme={null}
# Preferencias individuales
- @~/.claude/my-project-instructions.md
```

<Warning>
  La primera vez que Claude Code encuentra importaciones externas en un proyecto, muestra un diálogo de aprobación que enumera los archivos. Si rechaza, las importaciones permanecen deshabilitadas y el diálogo no aparece nuevamente.
</Warning>

Para un enfoque más estructurado para organizar instrucciones, consulte [`.claude/rules/`](#organize-rules-with-claude/rules/).

<h3 id="agents-md">
  AGENTS.md
</h3>

Claude Code lee `CLAUDE.md`, no `AGENTS.md`. Si su repositorio ya usa `AGENTS.md` para otros agentes de codificación, cree un `CLAUDE.md` que lo importe para que ambas herramientas lean las mismas instrucciones sin duplicarlas. También puede agregar instrucciones específicas de Claude Code debajo de la importación. Claude carga el archivo importado al inicio de la sesión, luego agrega el resto:

```markdown CLAUDE.md theme={null}
@AGENTS.md

## Claude Code

Use plan mode para cambios bajo `src/billing/`.
```

Un enlace simbólico también funciona si no necesita agregar contenido específico de Claude Code:

```bash theme={null}
ln -s AGENTS.md CLAUDE.md
```

En Windows, crear un enlace simbólico requiere privilegios de administrador o modo de desarrollador, así que use la importación `@AGENTS.md` en su lugar.

Ejecutar [`/init`](/docs/es/commands) en un repositorio que ya tiene un `AGENTS.md` lo lee e incorpora las partes relevantes en el `CLAUDE.md` generado. También lee otras configuraciones de herramientas como `.cursorrules`, `.devin/rules/` y `.windsurfrules`.

<h3 id="how-claude-md-files-load">
  Cómo se cargan los archivos CLAUDE.md
</h3>

Claude Code lee los archivos CLAUDE.md caminando hacia arriba en el árbol de directorios desde su directorio de trabajo actual, verificando cada directorio en el camino para archivos `CLAUDE.md` y `CLAUDE.local.md`. Esto significa que si ejecuta Claude Code en `foo/bar/`, carga instrucciones desde `foo/bar/CLAUDE.md`, `foo/CLAUDE.md` y cualquier archivo `CLAUDE.local.md` junto a ellos.

Todos los archivos descubiertos se concatenan en contexto en lugar de anularse entre sí. Dentro de la jerarquía de directorios, el contenido se ordena desde la raíz del sistema de archivos hasta su directorio de trabajo. Para el ejemplo `foo/bar/`, `foo/CLAUDE.md` aparece en contexto antes de `foo/bar/CLAUDE.md`, por lo que las instrucciones más cercanas a donde lanzó Claude se leen al final. Dentro de cada directorio, `CLAUDE.local.md` se agrega después de `CLAUDE.md`, por lo que sus notas personales son lo último que Claude lee en ese nivel.

Claude también descubre archivos `CLAUDE.md` y `CLAUDE.local.md` en subdirectorios bajo su directorio de trabajo actual. En lugar de cargarlos al iniciar, se incluyen cuando Claude lee archivos en esos subdirectorios.

Si trabaja en un monorepo grande donde se recogen archivos CLAUDE.md de otros equipos, use [`claudeMdExcludes`](#exclude-specific-claude-md-files) para omitirlos. Para el diseño completo de archivos CLAUDE.md de raíz y por directorio y reglas, consulte [Monorepos y repositorios grandes](/docs/es/large-codebases).

Los comentarios HTML a nivel de bloque (`<!-- notas de mantenimiento -->`) en archivos CLAUDE.md se eliminan antes de que el contenido se inyecte en el contexto de Claude. Úselos para dejar notas para los mantenedores humanos sin gastar tokens de contexto en ellas. Los comentarios dentro de bloques de código se conservan. Cuando abre un archivo CLAUDE.md directamente con la herramienta Read, los comentarios permanecen visibles.

<h4 id="load-from-additional-directories">
  Cargar desde directorios adicionales
</h4>

La bandera `--add-dir` da a Claude acceso a directorios adicionales fuera de su directorio de trabajo principal. De forma predeterminada, los archivos CLAUDE.md de estos directorios no se cargan.

Para cargar también archivos de memoria desde directorios adicionales, establezca la variable de entorno `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD`:

```bash theme={null}
CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1 claude --add-dir ../shared-config
```

Esto carga `CLAUDE.md`, `.claude/CLAUDE.md`, `.claude/rules/*.md` y `CLAUDE.local.md` desde el directorio adicional. `CLAUDE.local.md` se omite si excluye `local` de [`--setting-sources`](/docs/es/cli-reference).

<h3 id="organize-rules-with-claude/rules/">
  Organizar reglas con `.claude/rules/`
</h3>

Para proyectos más grandes, puede organizar instrucciones en múltiples archivos usando el directorio `.claude/rules/`. Esto mantiene las instrucciones modulares y más fáciles de mantener para los equipos. Las reglas también pueden ser [limitadas a rutas de archivo específicas](#path-specific-rules), por lo que solo se cargan en contexto cuando Claude trabaja con archivos coincidentes, reduciendo ruido y ahorrando espacio de contexto.

<Note>
  Las reglas se cargan en contexto cada sesión o cuando se abren archivos coincidentes. Para instrucciones específicas de tareas que no necesitan estar en contexto todo el tiempo, use [skills](/docs/es/skills) en su lugar, que solo se cargan cuando las invoca o cuando Claude determina que son relevantes para su prompt.
</Note>

<h4 id="set-up-rules">
  Configurar reglas
</h4>

Coloque archivos markdown en el directorio `.claude/rules/` de su proyecto. Cada archivo debe cubrir un tema, con un nombre de archivo descriptivo como `testing.md` o `api-design.md`. Todos los archivos `.md` se descubren recursivamente, por lo que puede organizar reglas en subdirectorios como `frontend/` o `backend/`:

```text theme={null}
your-project/
├── .claude/
│   ├── CLAUDE.md           # Instrucciones principales del proyecto
│   └── rules/
│       ├── code-style.md   # Directrices de estilo de código
│       ├── testing.md      # Convenciones de prueba
│       └── security.md     # Requisitos de seguridad
```

Las reglas sin [frontmatter `paths`](#path-specific-rules) se cargan al iniciar con la misma prioridad que `.claude/CLAUDE.md`.

<h4 id="path-specific-rules">
  Reglas específicas de ruta
</h4>

Las reglas pueden limitarse a archivos específicos usando frontmatter YAML con el campo `paths`. Estas reglas condicionales solo se aplican cuando Claude trabaja con archivos que coinciden con los patrones especificados.

```markdown theme={null}
---
paths:
  - "src/api/**/*.ts"
---

# Reglas de desarrollo de API

- Todos los puntos finales de API deben incluir validación de entrada
- Usar el formato de respuesta de error estándar
- Incluir comentarios de documentación OpenAPI
```

Las reglas sin un campo `paths` se cargan incondicionalmente y se aplican a todos los archivos. Las reglas con alcance de ruta se activan cuando Claude lee archivos que coinciden con el patrón, no en cada uso de herramienta. A partir de v2.1.198, la coincidencia también funciona cuando Claude alcanza un archivo a través de una ruta de proyecto vinculada simbólicamente, por ejemplo en un checkout vinculado simbólicamente.

Use patrones glob en el campo `paths` para hacer coincidir archivos por extensión, directorio o cualquier combinación:

| Patrón                 | Coincide con                                          |
| ---------------------- | ----------------------------------------------------- |
| `**/*.ts`              | Todos los archivos TypeScript en cualquier directorio |
| `src/**/*`             | Todos los archivos bajo el directorio `src/`          |
| `*.md`                 | Archivos Markdown en la raíz del proyecto             |
| `src/components/*.tsx` | Componentes React en un directorio específico         |

Puede especificar múltiples patrones y usar expansión de llaves para hacer coincidir múltiples extensiones en un patrón:

```markdown theme={null}
---
paths:
  - "src/**/*.{ts,tsx}"
  - "lib/**/*.ts"
  - "tests/**/*.test.ts"
---
```

La sintaxis glob trata `[` como el inicio de una expresión de corchetes como `[abc]`. Un patrón con un `[` que no se puede leer como una expresión de corchetes, como `photos [2024/**`, es inválido: no coincide con nada, y los otros patrones de la regla siguen funcionando. Para hacer coincidir un `[` literal en un nombre de archivo, escápelo como `photos \[2024/**`. Antes de v2.1.207, un patrón inválido hacía que la herramienta Read fallara para cada archivo contra el cual se evaluaba la regla, en lugar de no coincidir con nada.

<h4 id="share-rules-across-projects-with-symlinks">
  Compartir reglas entre proyectos con enlaces simbólicos
</h4>

El directorio `.claude/rules/` admite enlaces simbólicos, por lo que puede mantener un conjunto compartido de reglas y vincularlas en múltiples proyectos. Los enlaces simbólicos se resuelven y se cargan normalmente, y los enlaces simbólicos circulares se detectan y se manejan correctamente.

Este ejemplo vincula tanto un directorio compartido como un archivo individual:

```bash theme={null}
ln -s ~/shared-claude-rules .claude/rules/shared
ln -s ~/company-standards/security.md .claude/rules/security.md
```

<h4 id="user-level-rules">
  Reglas a nivel de usuario
</h4>

Las reglas personales en `~/.claude/rules/` se aplican a cada proyecto en su máquina. Úselas para preferencias que no son específicas del proyecto:

```text theme={null}
~/.claude/rules/
├── preferences.md    # Sus preferencias personales de codificación
└── workflows.md      # Sus flujos de trabajo preferidos
```

Las reglas a nivel de usuario se cargan antes que las reglas del proyecto, dando a las reglas del proyecto mayor prioridad.

<h3 id="manage-claude-md-for-large-teams">
  Gestionar CLAUDE.md para equipos grandes
</h3>

Para organizaciones que implementan Claude Code en equipos, puede centralizar instrucciones y controlar qué archivos CLAUDE.md se cargan.

<h4 id="deploy-organization-wide-claude-md">
  Implementar CLAUDE.md en toda la organización
</h4>

Las organizaciones pueden implementar un CLAUDE.md gestionado centralmente que se aplique a todos los usuarios en una máquina. Este archivo no puede ser excluido por configuraciones individuales.

<Steps>
  <Step title="Crear el archivo en la ubicación de política gestionada">
    * macOS: `/Library/Application Support/ClaudeCode/CLAUDE.md`
    * Linux y WSL: `/etc/claude-code/CLAUDE.md`
    * Windows: `C:\Program Files\ClaudeCode\CLAUDE.md`
  </Step>

  <Step title="Implementar con su sistema de gestión de configuración">
    Use MDM, Group Policy, Ansible o herramientas similares para distribuir el archivo en máquinas de desarrolladores. Consulte [configuración gestionada](/docs/es/permissions#managed-settings) para otras opciones de configuración de toda la organización.
  </Step>
</Steps>

La clave `claudeMd` le permite poner contenido CLAUDE.md gestionado directamente dentro de `managed-settings.json` en lugar de implementar un archivo separado.

**Alcance**: cada sesión de Claude Code en la máquina, en cada repositorio. Para orientación específica del repositorio, confirme un CLAUDE.md de proyecto en su lugar.

**Precedencia**: igual que un archivo CLAUDE.md gestionado. Se carga antes que CLAUDE.md de usuario y proyecto.

**Dónde se respeta**: solo configuración gestionada y de política. Establecer `claudeMd` en configuración de usuario, proyecto o local no tiene efecto.

El ejemplo a continuación agrega instrucciones conductuales directamente en un archivo de configuración gestionada:

```json theme={null}
{
  "claudeMd": "Always run `make lint` before committing.\nNever push directly to main."
}
```

Un CLAUDE.md gestionado y [configuración gestionada](/docs/es/settings#settings-files) sirven para propósitos diferentes. Use configuración para aplicación técnica y CLAUDE.md para orientación conductual:

| Preocupación                                                   | Configurar en                                                     |
| :------------------------------------------------------------- | :---------------------------------------------------------------- |
| Bloquear herramientas, comandos o rutas de archivo específicas | Configuración gestionada: `permissions.deny`                      |
| Aplicar aislamiento de sandbox                                 | Configuración gestionada: `sandbox.enabled`                       |
| Variables de entorno y enrutamiento de proveedor de API        | Configuración gestionada: `env`                                   |
| Método de autenticación y bloqueo de organización              | Configuración gestionada: `forceLoginMethod`, `forceLoginOrgUUID` |
| Directrices de estilo de código y calidad                      | CLAUDE.md gestionado                                              |
| Recordatorios de manejo de datos y cumplimiento                | CLAUDE.md gestionado                                              |
| Instrucciones conductuales para Claude                         | CLAUDE.md gestionado                                              |

Las reglas de configuración se aplican por el cliente independientemente de lo que Claude decida hacer. Las instrucciones de CLAUDE.md moldean el comportamiento de Claude pero no son una capa de aplicación forzada.

<h4 id="exclude-specific-claude-md-files">
  Excluir archivos CLAUDE.md específicos
</h4>

En monorepos grandes, los archivos CLAUDE.md ancestros pueden contener instrucciones que no son relevantes para su trabajo. La configuración `claudeMdExcludes` le permite omitir archivos específicos por ruta o patrón glob.

Este ejemplo excluye un CLAUDE.md de nivel superior y un directorio de reglas de una carpeta principal. Agréguelo a `.claude/settings.local.json` para que la exclusión permanezca local en su máquina:

```json theme={null}
{
  "claudeMdExcludes": [
    "**/monorepo/CLAUDE.md",
    "/home/user/monorepo/other-team/.claude/rules/**"
  ]
}
```

Los patrones se comparan contra rutas de archivo absolutas usando sintaxis glob. Puede configurar `claudeMdExcludes` en cualquier [capa de configuración](/docs/es/settings#settings-files): usuario, proyecto, local o política gestionada. Los arrays se fusionan entre capas.

Los archivos CLAUDE.md de política gestionada no pueden ser excluidos. Esto asegura que las instrucciones de toda la organización siempre se apliquen independientemente de la configuración individual.

<h2 id="auto-memory">
  Auto memory
</h2>

Auto memory permite que Claude acumule conocimiento entre sesiones sin que usted escriba nada. Claude guarda notas para sí mismo mientras trabaja: comandos de compilación, información de depuración, notas de arquitectura, preferencias de estilo de código y hábitos de flujo de trabajo. Claude no guarda algo cada sesión. Decide qué vale la pena recordar basándose en si la información sería útil en una conversación futura.

<h3 id="enable-or-disable-auto-memory">
  Habilitar o deshabilitar auto memory
</h3>

Auto memory está habilitado de forma predeterminada. Para alternarlo, abra `/memory` en una sesión y use el botón de alternancia de auto memory, o establezca `autoMemoryEnabled` en la configuración de su proyecto:

```json theme={null}
{
  "autoMemoryEnabled": false
}
```

Para deshabilitar auto memory a través de variable de entorno, establezca `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1`.

<h3 id="storage-location">
  Ubicación de almacenamiento
</h3>

Cada proyecto obtiene su propio directorio de memoria en `~/.claude/projects/<project>/memory/`. La ruta `<project>` se deriva del repositorio git, por lo que todos los worktrees y subdirectorios dentro del mismo repositorio comparten un directorio de auto memory. Fuera de un repositorio git, se usa la raíz del proyecto en su lugar.

Para almacenar auto memory en una ubicación diferente, establezca `autoMemoryDirectory` en su `settings.json`. Se lee desde cualquier [ámbito de configuración](/docs/es/settings#settings-precedence): usuario, proyecto, local, política, o `--settings`.

```json theme={null}
{
  "autoMemoryDirectory": "~/my-custom-memory-dir"
}
```

El valor debe ser una ruta absoluta o comenzar con `~/`. Cuando se establece en `.claude/settings.json` o `.claude/settings.local.json` de un proyecto, el valor se respeta solo después de que acepte el diálogo de confianza del espacio de trabajo para esa carpeta, la misma puerta que rige los hooks.

El directorio contiene un punto de entrada `MEMORY.md` y archivos de tema opcionales:

```text theme={null}
~/.claude/projects/<project>/memory/
├── MEMORY.md          # Índice conciso, cargado en cada sesión
├── debugging.md       # Notas detalladas sobre patrones de depuración
├── api-conventions.md # Decisiones de diseño de API
└── ...                # Cualquier otro archivo de tema que Claude cree
```

`MEMORY.md` actúa como un índice del directorio de memoria. Claude lee y escribe archivos en este directorio durante su sesión, usando `MEMORY.md` para mantener un registro de lo que se almacena dónde.

Auto memory es local de la máquina. Todos los worktrees y subdirectorios dentro del mismo repositorio git comparten un directorio de auto memory. Los archivos no se comparten entre máquinas o entornos en la nube.

<h3 id="how-it-works">
  Cómo funciona
</h3>

Las primeras 200 líneas de `MEMORY.md`, o los primeros 25KB, lo que sea menor, se cargan al inicio de cada conversación. El contenido más allá de ese umbral no se carga al inicio de la sesión. Claude mantiene `MEMORY.md` conciso moviendo notas detalladas a archivos de tema separados.

Este límite se aplica solo a `MEMORY.md`. Los archivos CLAUDE.md se cargan completamente independientemente de la longitud, aunque los archivos más cortos producen mejor adherencia.

Los archivos de tema como `debugging.md` o `patterns.md` no se cargan al iniciar. Claude los lee bajo demanda usando sus herramientas de archivo estándar cuando necesita la información.

Claude lee y escribe archivos de memoria durante su sesión. Cuando ve "Writing memory" o "Recalled memory" en la interfaz de Claude Code, Claude está actualizando o leyendo activamente desde `~/.claude/projects/<project>/memory/`.

<h3 id="audit-and-edit-your-memory">
  Auditar y editar su memoria
</h3>

Los archivos de auto memory son markdown plano que puede editar o eliminar en cualquier momento. Ejecute [`/memory`](#view-and-edit-with-%2Fmemory) para examinar y abrir archivos de memoria desde dentro de una sesión.

<h2 id="view-and-edit-with-/memory">
  Ver y editar con `/memory`
</h2>

El comando `/memory` enumera todos los archivos CLAUDE.md, CLAUDE.local.md y rules cargados en su sesión actual, le permite alternar auto memory activado o desactivado, y proporciona un enlace para abrir la carpeta de auto memory. Seleccione cualquier archivo para abrirlo en su editor.

Cuando le pide a Claude que recuerde algo, como "siempre usar pnpm, no npm" o "recuerde que las pruebas de API requieren una instancia local de Redis", Claude lo guarda en auto memory. Para agregar instrucciones a CLAUDE.md en su lugar, pídale a Claude directamente, como "agregue esto a CLAUDE.md", o edite el archivo usted mismo a través de `/memory`.

<h2 id="troubleshoot-memory-issues">
  Solucionar problemas de memoria
</h2>

Estos son los problemas más comunes con CLAUDE.md y auto memory, junto con pasos para depurarlos.

<h3 id="claude-isn’t-following-my-claude-md">
  Claude no está siguiendo mi CLAUDE.md
</h3>

El contenido de CLAUDE.md se entrega como un mensaje de usuario después del prompt del sistema, no como parte del prompt del sistema en sí. Claude lo lee e intenta seguirlo, pero no hay garantía de cumplimiento estricto, especialmente para instrucciones vagas o conflictivas.

Para depurar:

* Ejecute `/memory` para verificar que sus archivos CLAUDE.md y CLAUDE.local.md se están cargando. Si un archivo no aparece en la lista, Claude no puede verlo.
* Verifique que el CLAUDE.md relevante esté en una ubicación que se cargue para su sesión (consulte [Elija dónde colocar los archivos CLAUDE.md](#choose-where-to-put-claude-md-files)).
* Haga instrucciones más específicas. "Usar indentación de 2 espacios" funciona mejor que "formatear código bien".
* Busque instrucciones conflictivas en archivos CLAUDE.md. Si dos archivos dan orientación diferente para el mismo comportamiento, Claude puede elegir uno arbitrariamente.

Si la instrucción es algo que debe ejecutarse en un punto específico, como antes de cada commit o después de cada edición de archivo, escríbala como un [hook](/docs/es/hooks-guide) en su lugar. Los hooks se ejecutan como comandos de shell en eventos de ciclo de vida fijos y se aplican independientemente de lo que Claude decida hacer.

Para instrucciones que desea a nivel de prompt del sistema, use [`--append-system-prompt`](/docs/es/cli-reference#system-prompt-flags). Esto debe pasarse en cada invocación, por lo que es más adecuado para scripts y automatización que para uso interactivo.

<Tip>
  Use el hook [`InstructionsLoaded`](/docs/es/hooks#instructionsloaded) para registrar exactamente qué archivos de instrucciones se cargan, cuándo se cargan y por qué. Esto es útil para depurar reglas específicas de ruta o archivos cargados perezosamente en subdirectorios.
</Tip>

<h3 id="i-don’t-know-what-auto-memory-saved">
  No sé qué guardó auto memory
</h3>

Ejecute `/memory` y seleccione la carpeta de auto memory para examinar lo que Claude ha guardado. Todo es markdown plano que puede leer, editar o eliminar.

<h3 id="my-claude-md-is-too-large">
  Mi CLAUDE.md es demasiado grande
</h3>

Los archivos de más de 200 líneas consumen más contexto y pueden reducir la adherencia. Use [reglas con alcance de ruta](#path-specific-rules) para cargar instrucciones solo cuando Claude trabaja con archivos coincidentes, o recorte contenido que no sea necesario en cada sesión. Dividir en [importaciones `@path`](#import-additional-files) ayuda a la organización pero no reduce el contexto, ya que los archivos importados se cargan al iniciar.

La revisión [`/doctor`](/docs/es/commands#all-commands) propone recortes para un CLAUDE.md registrado: elimina contenido que Claude puede derivar de la base de código, como diseños de directorios, listas de dependencias y descripción general de la arquitectura, y mantiene trampas, justificación y convenciones que difieren de los valores predeterminados de las herramientas. La verificación de recorte requiere Claude Code v2.1.206 o posterior.

<h3 id="instructions-seem-lost-after-/compact">
  Las instrucciones parecen perdidas después de `/compact`
</h3>

CLAUDE.md de raíz de proyecto sobrevive a la compactación: después de `/compact`, Claude vuelve a leer desde el disco e lo reinyecta en la sesión. Los archivos CLAUDE.md anidados en subdirectorios no se reinyectan automáticamente; se recargan la próxima vez que Claude lee un archivo en ese subdirectorio.

Si una instrucción desapareció después de la compactación, se dio solo en la conversación o vive en un CLAUDE.md anidado que aún no se ha recargado. Agregue instrucciones solo de conversación a CLAUDE.md para que persistan. Consulte [Qué sobrevive a la compactación](/docs/es/context-window#what-survives-compaction) para el desglose completo.

Consulte [Escriba instrucciones efectivas](#write-effective-instructions) para obtener orientación sobre tamaño, estructura y especificidad.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Depurar su configuración](/docs/es/debug-your-config): diagnosticar por qué CLAUDE.md o configuración no están surtiendo efecto
* [Skills](/docs/es/skills): empaquetar flujos de trabajo repetibles que se cargan bajo demanda
* [Settings](/docs/es/settings): configurar el comportamiento de Claude Code con archivos de configuración
* [Subagent memory](/docs/es/sub-agents#enable-persistent-memory): permitir que los subagents mantengan su propia auto memory
