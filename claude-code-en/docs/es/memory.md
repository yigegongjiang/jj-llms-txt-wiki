> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Cómo Claude recuerda su proyecto

> Proporcione a Claude instrucciones persistentes con archivos CLAUDE.md o AGENTS.md, y permita que Claude acumule aprendizajes automáticamente con auto memory.

Cada sesión de Claude Code comienza con una ventana de contexto nueva. Dos mecanismos llevan el conocimiento entre sesiones:

* **Archivos CLAUDE.md**: instrucciones que usted escribe para dar a Claude contexto persistente. Claude también puede leer archivos [`AGENTS.md`](#agents-md) de un repositorio, por sí solos o junto con CLAUDE.md
* **Auto memory**: notas que Claude escribe por sí mismo basadas en sus correcciones y preferencias

Esta página cubre cómo:

* [Escribir y organizar archivos CLAUDE.md](#claude-md-files)
* [Usar un AGENTS.md existente](#agents-md) como sus instrucciones de proyecto, por sí solo o junto con CLAUDE.md
* [Limitar reglas a tipos de archivo específicos](#organize-rules-with-claude/rules/) con `.claude/rules/`
* [Configurar auto memory](#auto-memory) para que Claude tome notas automáticamente
* [Solucionar problemas](#troubleshoot-memory-issues) cuando las instrucciones no se siguen

<h2 id="claude-md-vs-auto-memory">
  CLAUDE.md vs auto memory
</h2>

Claude Code tiene dos sistemas de memoria complementarios. Ambos se cargan al inicio de cada conversación. Claude los trata como contexto, no como configuración forzada. Para bloquear una acción independientemente de lo que Claude decida, use un [hook PreToolUse](/docs/es/hooks-guide) en su lugar. Cuanto más específicas y concisas sean sus instrucciones, más consistentemente Claude las seguirá.

|                      | Archivos CLAUDE.md                                                       | Auto memory                                                                                                     |
| :------------------- | :----------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- |
| **Quién lo escribe** | Usted                                                                    | Claude                                                                                                          |
| **Qué contiene**     | Instrucciones y reglas                                                   | Aprendizajes y patrones                                                                                         |
| **Alcance**          | Proyecto, usuario u organización                                         | Por repositorio, compartido entre worktrees                                                                     |
| **Se carga en**      | Cada sesión                                                              | Cada sesión (primeras 200 líneas o 25KB)                                                                        |
| **Usar para**        | Estándares de codificación, flujos de trabajo, arquitectura del proyecto | Sus preferencias, correcciones que le da a Claude, contexto del proyecto que Claude no puede derivar del código |

Use archivos CLAUDE.md cuando quiera guiar el comportamiento de Claude. Auto memory permite que Claude aprenda de sus correcciones sin esfuerzo manual.

Los subagents también pueden mantener su propia auto memory. Consulte [configuración de subagent](/docs/es/sub-agents#enable-persistent-memory) para obtener detalles.

<h2 id="claude-md-files">
  Archivos CLAUDE.md
</h2>

Los archivos CLAUDE.md son archivos markdown que proporcionan a Claude instrucciones persistentes para un proyecto, su flujo de trabajo personal o toda su organización. Usted escribe estos archivos en texto plano; Claude los lee al inicio de cada sesión. Si su repositorio utiliza `AGENTS.md` en su lugar, consulte [AGENTS.md](#agents-md).

<h3 id="when-to-add-to-claude-md">
  Cuándo agregar a CLAUDE.md
</h3>

Trate CLAUDE.md como el lugar donde escribe lo que de otro modo tendría que re-explicar. Agregue contenido cuando:

* Claude comete el mismo error una segunda vez
* Una revisión de código detecta lo que sí llega al PR que Claude debería haber sabido sobre esta base de código
* Usted escribe la misma corrección o aclaración en el chat que escribió en la sesión anterior
* Un nuevo compañero de equipo necesitaría el mismo contexto para ser productivo

Manténgalo en hechos que Claude debe retener en cada sesión: comandos de compilación, convenciones, diseño del proyecto, reglas "siempre haga X". Si una entrada es un procedimiento de varios pasos o solo importa para una parte de la base de código, muévala a un [skill](/docs/es/skills) o a una [regla con alcance de ruta](#organize-rules-with-claude/rules/) en su lugar. La [descripción general de extensiones](/docs/es/features-overview#build-your-setup-over-time) cubre cuándo usar cada mecanismo.

<h3 id="choose-where-to-put-claude-md-files">
  Elija dónde colocar los archivos CLAUDE.md
</h3>

Los archivos CLAUDE.md pueden vivir en varios lugares, cada uno con un alcance diferente. La tabla a continuación los enumera en orden de carga, desde el alcance más amplio hasta el más específico, por lo que una instrucción de proyecto aparece en contexto después de una instrucción de usuario.

| Alcance                       | Ubicación                                                                                                                                                             | Propósito                                                                | Ejemplos de casos de uso                                                                     | Compartido con                                        |
| ----------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------- | ----------------------------------------------------- |
| **Política administrada**     | • macOS: `/Library/Application Support/ClaudeCode/CLAUDE.md`<br />• Linux y WSL: `/etc/claude-code/CLAUDE.md`<br />• Windows: `C:\Program Files\ClaudeCode\CLAUDE.md` | Instrucciones de toda la organización administradas por TI/DevOps        | Estándares de codificación de la empresa, políticas de seguridad, requisitos de cumplimiento | Todos los usuarios de la organización                 |
| **Instrucciones de usuario**  | `~/.claude/CLAUDE.md`                                                                                                                                                 | Preferencias personales para todos los proyectos                         | Preferencias de estilo de código, atajos de herramientas personales                          | Solo usted (todos los proyectos)                      |
| **Instrucciones de proyecto** | `./CLAUDE.md` o `./.claude/CLAUDE.md`. Consulte [AGENTS.md](#agents-md) para saber cuándo `./AGENTS.md` se carga en su lugar o junto con ellos                        | Instrucciones compartidas por el equipo para el proyecto                 | Arquitectura del proyecto, estándares de codificación, flujos de trabajo comunes             | Miembros del equipo a través del control de versiones |
| **Instrucciones locales**     | `./CLAUDE.local.md`                                                                                                                                                   | Preferencias personales específicas del proyecto; agregue a `.gitignore` | Sus URLs de sandbox, datos de prueba preferidos                                              | Solo usted (proyecto actual)                          |

Los archivos CLAUDE.md y CLAUDE.local.md en la jerarquía de directorios por encima del directorio de trabajo se cargan al iniciar. Los archivos en subdirectorios se cargan bajo demanda cuando Claude lee archivos en esos directorios. Consulte [Cómo se cargan los archivos CLAUDE.md](#how-claude-md-files-load) para el orden de resolución completo.

Para proyectos grandes, puede dividir las instrucciones en archivos específicos de temas utilizando [reglas de proyecto](#organize-rules-with-claude/rules/). Las reglas le permiten limitar el alcance de las instrucciones a tipos de archivo específicos o subdirectorios.

<h3 id="set-up-a-project-claude-md">
  Configure un CLAUDE.md de proyecto
</h3>

Un CLAUDE.md de proyecto puede almacenarse en `./CLAUDE.md` o `./.claude/CLAUDE.md`. Cree este archivo y agregue instrucciones que se apliquen a cualquiera que trabaje en el proyecto: comandos de compilación y prueba, estándares de codificación, decisiones arquitectónicas, convenciones de nomenclatura y flujos de trabajo comunes. Estas instrucciones se comparten con su equipo a través del control de versiones, por lo que enfóquese en estándares a nivel de proyecto en lugar de preferencias personales. Para confirmar que el archivo se cargó, ejecute `/context` en una sesión y verifique la lista bajo **Memory files**.

<Tip>
  Ejecute `/init` para generar un CLAUDE.md inicial automáticamente. Claude analiza su base de código y crea un archivo con comandos de compilación, instrucciones de prueba y convenciones de proyecto que descubre. Si ya existe un CLAUDE.md, `/init` sugiere mejoras en lugar de sobrescribirlo. Refine desde allí con instrucciones que Claude no descubriría por sí solo.

  Para un flujo interactivo de múltiples fases en su lugar, configure la variable de entorno `CLAUDE_CODE_NEW_INIT` a `1` antes de ejecutar `/init`. Configúrela en su shell o en el bloque `env` de un archivo de configuración, como se muestra en [Establecer variables de entorno](/docs/es/env-vars#set-environment-variables). Con ella configurada, `/init` pregunta qué artefactos configurar: archivos CLAUDE.md, skills y hooks. Luego explora su base de código con un subagente, llena los vacíos mediante preguntas de seguimiento y presenta una propuesta revisable antes de escribir cualquier archivo. La variable solo cambia cómo se ejecuta `/init`, por lo que puede dejarla configurada.
</Tip>

<h3 id="write-effective-instructions">
  Escriba instrucciones efectivas
</h3>

Los archivos CLAUDE.md se cargan en la ventana de contexto al inicio de cada sesión, consumiendo tokens junto con su conversación. La [visualización de la ventana de contexto](/docs/es/context-window) muestra dónde se carga CLAUDE.md en relación con el resto del contexto de inicio. Debido a que son contexto en lugar de configuración forzada, la forma en que escribe las instrucciones afecta la confiabilidad con la que Claude las sigue. Las instrucciones específicas, concisas y bien estructuradas funcionan mejor.

**Tamaño**: apunte a menos de 200 líneas por archivo CLAUDE.md. Los archivos más largos consumen más contexto y reducen la adherencia. Si sus instrucciones están creciendo mucho, use [reglas con alcance de ruta](#path-specific-rules) para que las instrucciones se carguen solo cuando Claude trabaje con archivos coincidentes. También puede dividir el contenido en [importaciones](#import-additional-files) para organización, aunque los archivos importados aún se cargan e ingresan a la ventana de contexto al iniciar.

**Estructura**: use encabezados markdown y viñetas para agrupar instrucciones relacionadas. Claude escanea la estructura de la misma manera que los lectores: las secciones organizadas son más fáciles de seguir que los párrafos densos.

**Especificidad**: escriba instrucciones que sean lo suficientemente concretas para verificar. Por ejemplo:

* "Usar indentación de 2 espacios" en lugar de "Formatear código correctamente"
* "Ejecutar `npm test` antes de hacer commit" en lugar de "Probar sus cambios"
* "Los controladores de API viven en `src/api/handlers/`" en lugar de "Mantener los archivos organizados"

**Consistencia**: si dos reglas se contradicen entre sí, Claude puede elegir una arbitrariamente. Revise sus archivos CLAUDE.md, archivos CLAUDE.md anidados en subdirectorios y [`.claude/rules/`](#organize-rules-with-claude/rules/) periódicamente para eliminar instrucciones obsoletas o conflictivas. En monorepos, use [`claudeMdExcludes`](#exclude-specific-claude-md-files) para omitir archivos CLAUDE.md de otros equipos que no sean relevantes para su trabajo.

<h3 id="import-additional-files">
  Importe archivos adicionales
</h3>

Los archivos CLAUDE.md pueden importar archivos adicionales usando la sintaxis `@path/to/import`. Los archivos importados se expanden y se cargan en contexto al iniciar junto con el CLAUDE.md que los referencia.

Se permiten rutas relativas y absolutas. Las rutas relativas se resuelven en relación con el archivo que contiene la importación, no con el directorio de trabajo. Los archivos importados pueden importar recursivamente otros archivos, con una profundidad máxima de cuatro saltos.

El análisis de importación omite espacios de código Markdown y bloques de código delimitados. Para mencionar una ruta en su CLAUDE.md sin importarla, envuélvala en backticks: escribir `` `@README` `` mantiene el texto literal, mientras que `@README` fuera de backticks importa el archivo.

Para incluir un README, package.json y una guía de flujo de trabajo, haga referencia a ellos con la sintaxis `@` en cualquier lugar de su CLAUDE.md:

```text theme={null}
Consulte @README para obtener una descripción general del proyecto y @package.json para los comandos npm disponibles para este proyecto.

# Instrucciones adicionales
- flujo de trabajo de git @docs/git-instructions.md
```

Para preferencias personales privadas por proyecto que no deben verificarse en el control de versiones, cree un `CLAUDE.local.md` en la raíz del proyecto. Se carga junto con `CLAUDE.md` y se trata de la misma manera. Agregue `CLAUDE.local.md` a su `.gitignore` para que no se confirme. Con `CLAUDE_CODE_NEW_INIT=1` configurado, ejecutar `/init` y elegir la opción personal lo hace por usted.

Si trabaja en múltiples git worktrees del mismo repositorio, un `CLAUDE.local.md` ignorado por git solo existe en el worktree donde lo creó. Para compartir instrucciones personales entre worktrees, importe un archivo desde su directorio de inicio en su lugar:

```text theme={null}
# Preferencias individuales
- @~/.claude/my-project-instructions.md
```

<Warning>
  Una importación en un archivo de memoria a nivel de proyecto es externa cuando su ruta se resuelve fuera de su directorio de trabajo, como la importación del directorio de inicio anterior. La primera vez que Claude Code encuentra importaciones externas en un proyecto, muestra un diálogo de aprobación que enumera los archivos. Si rechaza, las importaciones permanecen deshabilitadas y el diálogo no aparece nuevamente.

  Claude Code muestra el diálogo para protegerlo de archivos que otras personas confirmen en un proyecto compartido. Los archivos de memoria con alcance de usuario, como `~/.claude/CLAUDE.md` y `~/.claude/rules/`, son archivos que usted escribió. Excepto en sesiones de [Cowork](https://claude.com/product/cowork) en su escritorio, Claude Code carga sus importaciones sin el diálogo y las confía como el resto de su configuración personal.

  En sesiones de Cowork en su escritorio, Claude Code omite cualquier importación en un archivo con alcance de usuario que se resuelva a una ruta fuera del directorio de trabajo de la sesión y carga el resto del archivo. En esas sesiones también omite un `~/.claude/CLAUDE.md` que es en sí mismo un enlace simbólico o un enlace duro, y un directorio `~/.claude/rules/` enlazado simbólicamente o un archivo de regla que apunta fuera del directorio de trabajo.
</Warning>

<h3 id="how-claude-md-files-load">
  Cómo se cargan los archivos CLAUDE.md
</h3>

Claude Code carga `CLAUDE.md` y `CLAUDE.local.md` desde su directorio de trabajo actual y cada directorio por encima de él. Ejecute Claude Code en `foo/bar/` y cargará instrucciones desde `foo/bar/CLAUDE.md`, `foo/CLAUDE.md` y cualquier archivo `CLAUDE.local.md` junto a ellos.

Todos los archivos descubiertos se concatenan en contexto en lugar de anularse entre sí. En el árbol de directorios, el contenido se ordena desde la raíz del sistema de archivos hasta su directorio de trabajo. Para el ejemplo `foo/bar/`, `foo/CLAUDE.md` aparece en contexto antes de `foo/bar/CLAUDE.md`, por lo que las instrucciones más cercanas a donde lanzó Claude se leen al final. Dentro de cada directorio, `CLAUDE.local.md` se añade después de `CLAUDE.md`, por lo que sus notas personales son lo último que Claude lee en ese nivel.

Claude también descubre archivos `CLAUDE.md` y `CLAUDE.local.md` en subdirectorios bajo su directorio de trabajo actual. En lugar de cargarlos al iniciar, se incluyen cuando Claude lee archivos en esos subdirectorios.

Si trabaja en un monorepo grande donde se recogen archivos CLAUDE.md de otros equipos, use [`claudeMdExcludes`](#exclude-specific-claude-md-files) para omitirlos. Para el diseño completo de archivos CLAUDE.md raíz y por directorio y reglas, consulte [Monorepos y repositorios grandes](/docs/es/large-codebases).

Los comentarios HTML a nivel de bloque (`<!-- maintainer notes -->`) en archivos CLAUDE.md se eliminan antes de que el contenido se inyecte en el contexto de Claude. Úselos para dejar notas para los mantenedores humanos sin gastar tokens de contexto en ellas. Los comentarios dentro de bloques de código se conservan. Cuando abre un archivo CLAUDE.md directamente con la herramienta Read, los comentarios permanecen visibles.

<h4 id="load-from-additional-directories">
  Cargue desde directorios adicionales
</h4>

La bandera `--add-dir` proporciona a Claude acceso a directorios adicionales fuera de su directorio de trabajo principal. De forma predeterminada, los archivos CLAUDE.md de estos directorios no se cargan.

Para cargar también archivos de memoria desde directorios adicionales, configure la variable de entorno `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD`:

```bash theme={null}
CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1 claude --add-dir ../shared-config
```

La forma en línea configura la variable para ese único lanzamiento en Bash o Zsh. Para mantenerla activada para cada sesión, agréguela al bloque `env` en `~/.claude/settings.json` como se muestra en [Establecer variables de entorno](/docs/es/env-vars#set-environment-variables).

Esto carga `CLAUDE.md`, `.claude/CLAUDE.md`, `.claude/rules/*.md` y `CLAUDE.local.md` desde el directorio adicional. `CLAUDE.local.md` se omite si excluye `local` de [`--setting-sources`](/docs/es/cli-reference).

<h3 id="organize-rules-with-claude/rules/">
  Organice reglas con `.claude/rules/`
</h3>

Para proyectos más grandes, puede organizar instrucciones en múltiples archivos usando el directorio `.claude/rules/`. Esto mantiene las instrucciones modulares y más fáciles de mantener para los equipos. Las reglas también pueden ser [limitadas a rutas de archivo específicas](#path-specific-rules), por lo que solo se cargan en contexto cuando Claude trabaja con archivos coincidentes, reduciendo el ruido y ahorrando espacio de contexto.

<Note>
  Las reglas se cargan en contexto cada sesión o cuando se abren archivos coincidentes. Para instrucciones específicas de tareas que no necesitan estar en contexto todo el tiempo, use [skills](/docs/es/skills) en su lugar, que solo se cargan cuando las invoca o cuando Claude determina que son relevantes para su solicitud.
</Note>

<h4 id="set-up-rules">
  Configure reglas
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

Las reglas de proyecto se omiten si excluye `project` de [`--setting-sources`](/docs/es/cli-reference). Antes de v2.1.211, las reglas que se cargan bajo demanda, incluidas las reglas con alcance de ruta y las reglas en directorios `.claude/rules/` anidados, se cargaban incluso cuando `project` estaba excluido.

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

Las reglas sin un campo `paths` se cargan incondicionalmente y se aplican a todos los archivos. Las reglas con alcance de ruta se activan cuando Claude lee archivos que coinciden con el patrón, no en cada uso de herramienta. A partir de v2.1.198, la coincidencia también funciona cuando Claude alcanza un archivo a través de una ruta enlazada simbólicamente al directorio del proyecto, por ejemplo en un checkout enlazado simbólicamente.

Use patrones glob en el campo `paths` para hacer coincidir archivos por extensión, directorio o cualquier combinación:

| Patrón                 | Coincide                                              |
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

Cada grupo de llaves multiplica el número de patrones expandidos: `src/*.{ts,tsx}` se expande a dos patrones, y `{a,b}/{c,d}/*.{ts,tsx}` a ocho. Para mantener la expansión acotada, la lista `paths` completa de una regla comparte un presupuesto de 1.000 patrones expandidos y 4 MiB, y los patrones sin llaves no cuentan contra él.

Claude Code usa cualquier patrón que excedería el presupuesto sin expandir, y sus llaves literales no coinciden con ningún archivo. Antes de v2.1.217, un valor `paths` con muchos grupos de llaves detenía o bloqueaba la CLI al iniciar.

La sintaxis Glob trata `[` como el inicio de una expresión de corchetes como `[abc]`. Un patrón con un `[` que no se puede leer como una expresión de corchetes, como `photos [2024/**`, es inválido: no coincide con nada, y los otros patrones de la regla siguen funcionando. Para hacer coincidir un `[` literal en un nombre de archivo, escápelo como `photos \[2024/**`. Antes de v2.1.207, un patrón inválido hacía que la herramienta Read fallara para cada archivo contra el cual se evaluaba la regla, en lugar de no coincidir con nada.

<h4 id="rules-frontmatter-reference">
  Referencia de frontmatter de reglas
</h4>

Configure una regla con [frontmatter](/docs/es/glossary#frontmatter) YAML entre marcadores `---` en la parte superior del archivo. `paths` es el único campo que Claude Code lee de una regla; cualquier otro campo se ignora sin un error. Claude Code elimina el frontmatter antes de cargar la regla en contexto.

| Campo   | Requerido | Descripción                                                                                                                               |
| :------ | :-------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| `paths` | No        | Patrones glob que [limitan la regla a archivos coincidentes](#path-specific-rules). Acepta una lista YAML o una cadena separada por comas |

Si el YAML entre los marcadores no se analiza, Claude Code ignora el frontmatter y carga la regla como si no tuviera `paths`. Ejecute `claude --debug` para ver el error de análisis.

<h4 id="share-rules-across-projects-with-symlinks">
  Comparta reglas entre proyectos con enlaces simbólicos
</h4>

El directorio `.claude/rules/` admite enlaces simbólicos, por lo que puede mantener un conjunto compartido de reglas y vincularlas en múltiples proyectos. Los enlaces simbólicos circulares se detectan y se manejan correctamente.

Claude Code trata un enlace simbólico cuyo destino está fuera de su directorio de trabajo como una [importación externa](#import-additional-files). Las reglas vinculadas no se cargan hasta que apruebe importaciones externas para el proyecto, y después de eso solo se cargan las que no tienen un [campo `paths`](#path-specific-rules). Claude Code solo solicita esa aprobación cuando un archivo de memoria de proyecto importa un archivo fuera del directorio de trabajo con `@path`, no para enlaces simbólicos solos. Para cargar reglas compartidas sin esa aprobación, manténgalas en [`~/.claude/rules/`](#user-level-rules), donde se aplican a cada proyecto en su máquina.

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

Claude Code carga reglas a nivel de usuario antes que reglas de proyecto, por lo que una regla de proyecto aparece más tarde en el contexto de Claude que una regla de usuario. Ninguno de los dos conjuntos anula al otro: si una regla de usuario y una regla de proyecto entran en conflicto, Claude puede seguir cualquiera de ellas, por lo que mantenga los dos consistentes.

<h3 id="manage-claude-md-for-large-teams">
  Administre CLAUDE.md para equipos grandes
</h3>

Para organizaciones que implementan Claude Code en equipos, puede centralizar instrucciones y controlar qué archivos CLAUDE.md se cargan.

<h4 id="deploy-organization-wide-claude-md">
  Implemente CLAUDE.md en toda la organización
</h4>

Las organizaciones pueden implementar un CLAUDE.md administrado centralmente que se aplique a todos los usuarios en una máquina. Este archivo no puede ser excluido por configuraciones individuales.

<Steps>
  <Step title="Cree el archivo en la ubicación de política administrada">
    * macOS: `/Library/Application Support/ClaudeCode/CLAUDE.md`
    * Linux y WSL: `/etc/claude-code/CLAUDE.md`
    * Windows: `C:\Program Files\ClaudeCode\CLAUDE.md`
  </Step>

  <Step title="Implemente con su sistema de gestión de configuración">
    Use MDM, Group Policy, Ansible o herramientas similares para distribuir el archivo en máquinas de desarrolladores. Consulte [configuración administrada](/docs/es/managed-settings) para otras opciones de configuración de toda la organización.
  </Step>
</Steps>

La clave `claudeMd` le permite poner contenido CLAUDE.md administrado directamente dentro de `managed-settings.json` en lugar de implementar un archivo separado.

**Alcance**: cada sesión de Claude Code en la máquina, en cada repositorio. Para orientación específica del repositorio, confirme un CLAUDE.md de proyecto en su lugar.

**Precedencia**: igual que un archivo CLAUDE.md administrado. Se carga antes que CLAUDE.md de usuario y proyecto.

**Dónde se respeta**: solo configuración administrada y de política. Configurar `claudeMd` en configuración de usuario, proyecto o local no tiene efecto.

El ejemplo a continuación agrega instrucciones de comportamiento directamente en un archivo de configuración administrada:

```json theme={null}
{
  "claudeMd": "Always run `make lint` before committing.\nNever push directly to main."
}
```

Un CLAUDE.md administrado y [configuración administrada](/docs/es/managed-settings) sirven para propósitos diferentes. Use configuración para aplicación técnica y CLAUDE.md para orientación de comportamiento:

| Preocupación                                                   | Configurar en                                                       |
| :------------------------------------------------------------- | :------------------------------------------------------------------ |
| Bloquear herramientas, comandos o rutas de archivo específicas | Configuración administrada: `permissions.deny`                      |
| Aplicar aislamiento de sandbox                                 | Configuración administrada: `sandbox.enabled`                       |
| Variables de entorno y enrutamiento de proveedores de API      | Configuración administrada: `env`                                   |
| Método de inicio de sesión y restricciones de organización     | Configuración administrada: `forceLoginMethod`, `forceLoginOrgUUID` |
| Directrices de estilo de código y calidad                      | CLAUDE.md administrado                                              |
| Recordatorios de manejo de datos y cumplimiento                | CLAUDE.md administrado                                              |
| Instrucciones de comportamiento para Claude                    | CLAUDE.md administrado                                              |

Las reglas de configuración se aplican por el cliente independientemente de lo que Claude decida hacer. Las instrucciones CLAUDE.md moldean el comportamiento de Claude pero no son una capa de aplicación forzada.

<h4 id="exclude-specific-claude-md-files">
  Excluya archivos CLAUDE.md específicos
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

Los patrones se comparan contra rutas de archivo absolutas usando sintaxis glob. Puede configurar `claudeMdExcludes` en cualquier [capa de configuración](/docs/es/settings#where-settings-live): usuario, proyecto, local o política administrada. Los arreglos se fusionan entre capas.

Para excluir un archivo de reglas que alcanza a través de un [enlace simbólico](#share-rules-across-projects-with-symlinks), ya sea que el archivo o su directorio sea el enlace, escriba el patrón contra cualquiera de las rutas: la ruta del archivo bajo `.claude/rules/` o su destino de enlace. Un patrón que coincida con cualquiera de las rutas excluye el archivo. Antes de v2.1.239, solo un patrón que coincidiera con el destino del enlace excluía el archivo.

Los archivos CLAUDE.md de política administrada no pueden ser excluidos. Esto asegura que las instrucciones de toda la organización siempre se apliquen independientemente de la configuración individual.

<h2 id="agents-md">
  AGENTS.md
</h2>

Claude Code puede leer [`AGENTS.md`](/docs/es/glossary#agents-md) como sus instrucciones de proyecto, por lo que un repositorio ya configurado para otros agentes de codificación funciona sin agregar un `CLAUDE.md`, una importación o una configuración. Esta tabla muestra lo que Claude lee de forma predeterminada para cada combinación de archivos de instrucciones en su repositorio:

| Su repositorio tiene                                                                            | Claude lee                                                          |
| :---------------------------------------------------------------------------------------------- | :------------------------------------------------------------------ |
| Un `AGENTS.md`, y ningún `CLAUDE.md` o `CLAUDE.local.md` en su directorio de trabajo o superior | Su `AGENTS.md`                                                      |
| Un `AGENTS.md` y un `CLAUDE.md` o `CLAUDE.local.md` en su directorio de trabajo o superior      | Solo sus archivos `CLAUDE.md`                                       |
| Un `CLAUDE.md` que ya [importa `AGENTS.md`](#share-one-file-with-other-coding-tools)            | Su `CLAUDE.md`, con `AGENTS.md` incluido a través de la importación |

Para cambiar el comportamiento predeterminado, por ejemplo para que Claude siempre lea ambos archivos, lea solo `CLAUDE.md`, o lea solo las instrucciones administradas de su organización, [cambie la configuración **Project instructions**](#choose-which-instruction-files-load).

<Note>
  La lectura de `AGENTS.md` directamente requiere Claude Code v2.1.277 o posterior. En algunas sesiones Claude [no puede leer `AGENTS.md`](#when-agents-md-support-is-unavailable), así que [impórtelo desde un `CLAUDE.md`](#share-one-file-with-other-coding-tools) en su lugar.
</Note>

<h3 id="when-claude-code-reads-agents-md">
  Cuándo Claude Code lee AGENTS.md
</h3>

De forma predeterminada, Claude lee `AGENTS.md` solo cuando no tiene `CLAUDE.md` en su directorio de trabajo o superior. Aquí están los archivos que cuentan para esa verificación:

* **Cuentan, por lo que Claude los lee en lugar de `AGENTS.md`**: un `CLAUDE.md`, `.claude/CLAUDE.md`, o `CLAUDE.local.md` en su directorio de trabajo o cualquier directorio superior
* **No cuentan, y continúan cargándose junto con `AGENTS.md`**: su `~/.claude/CLAUDE.md`, el `CLAUDE.md` administrado de su organización, y los archivos `.claude/rules/`

Cuando ninguno cuenta, aquí está lo que Claude lee y cómo puede saberlo:

* **Al inicio de la sesión**: cada `AGENTS.md` y `.claude/AGENTS.md` en su directorio de trabajo y los directorios superiores. En una sesión interactiva verá una línea como `no CLAUDE.md found; AGENTS.md loaded: /home/you/repo/AGENTS.md` en la conversación
* **Mientras Claude trabaja en subdirectorios**: el `AGENTS.md` de un subdirectorio, cuando Claude abre un archivo allí con la herramienta Read y ese subdirectorio no tiene ninguno de los tres archivos `CLAUDE.md` propios
* **Dentro de cada `AGENTS.md`**: las importaciones [`@path`](#import-additional-files) se expanden, los patrones [`claudeMdExcludes`](#exclude-specific-claude-md-files) se aplican, y los subagentes que [omiten instrucciones de proyecto](/docs/es/sub-agents#what-loads-at-startup) también omiten estos archivos
* **No se lee**: `AGENTS.local.md`, `AGENTS.override.md`, o cualquier cosa bajo un directorio `.agents/`

<Note>
  Debido a que `CLAUDE.local.md` cuenta, agregar uno para mantener sus propias instrucciones no comprometidas en un proyecto que depende de `AGENTS.md` detiene que Claude lea `AGENTS.md` para usted. Para mantener su `CLAUDE.local.md` y aún tener Claude leyendo `AGENTS.md`, establezca **Project instructions** en [`claude-md-and-agents-md`](#choose-which-instruction-files-load).
</Note>

<h3 id="choose-which-instruction-files-load">
  Elija qué archivos de instrucciones se cargan
</h3>

Para cambiar qué archivos lee Claude, escriba `/config` en una sesión de Claude Code para abrir el panel de configuración, luego establezca **Project instructions** en uno de estos valores:

| Valor                     | Lo que Claude lee                                                                                                                                                                                                                                                                                                                                                                                    |
| :------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `claude-md-or-agents-md`  | Sus archivos `CLAUDE.md`, o sus archivos `AGENTS.md` cuando no tiene `CLAUDE.md` o `CLAUDE.local.md` en su directorio de trabajo o superior. Este es el valor predeterminado                                                                                                                                                                                                                         |
| `claude-md-and-agents-md` | Sus archivos `CLAUDE.md` y `AGENTS.md` juntos, los archivos `CLAUDE.md` de cada directorio primero y su `AGENTS.md` después. Claude Code omite un `AGENTS.md` que ya ha cargado, por lo que uno que su `CLAUDE.md` importa o vincula simbólicamente no se lee dos veces                                                                                                                              |
| `claude-md`               | Solo sus archivos `CLAUDE.md`                                                                                                                                                                                                                                                                                                                                                                        |
| `managed-only`            | Solo el `CLAUDE.md` administrado de su organización y [auto memory](#auto-memory) al iniciar. Sus archivos `CLAUDE.md` de proyecto, local y usuario, sus archivos `.claude/rules/`, y cada `AGENTS.md` se dejan fuera. El `CLAUDE.md` de un subdirectorio y los archivos `.claude/rules/`, y las [reglas con alcance de ruta](#path-specific-rules), aún se cargan cuando Claude lee un archivo allí |

También puede establecer el valor en un archivo de configuración en lugar de `/config`. Agréguelo bajo el ID del plugin `agents-md` integrado en [`pluginConfigs`](/docs/es/settings-reference#pluginconfigs), en `~/.claude/settings.json`, un archivo `--settings` o [configuración administrada](/docs/es/managed-settings). Claude Code lo ignora en archivos de configuración de proyecto y local. Este ejemplo hace que Claude lea ambos archivos:

```json settings.json theme={null}
{
  "pluginConfigs": {
    "agents-md@builtin": {
      "options": { "instructionFiles": "claude-md-and-agents-md" }
    }
  }
}
```

Su cambio se aplica desde el siguiente mensaje que envíe y en cada nueva sesión.

<h3 id="when-agents-md-support-is-unavailable">
  Cuándo el soporte de AGENTS.md no está disponible
</h3>

En estas sesiones Claude lee solo archivos `CLAUDE.md`, y **Project instructions** no aparece en el panel de configuración `/config`:

* Está en una versión de Claude Code anterior a v2.1.277
* Usted deshabilitó el plugin `agents-md` integrado en `/plugin`
* En algunos casos, es su [primera sesión después de actualizar](/docs/es/env-vars#first-session-after-an-install-or-upgrade) desde v2.1.276 o anterior. Claude lee `AGENTS.md` desde su siguiente sesión

Antes de v2.1.281, algunas sesiones, como las en Amazon Bedrock o con telemetría deshabilitada, leían solo archivos `CLAUDE.md`. En esas versiones, actualice Claude Code. Para darle a Claude su `AGENTS.md` en cualquiera de estas sesiones, [impórtelo desde un `CLAUDE.md`](#share-one-file-with-other-coding-tools).

<h3 id="where-agents-md-differs-from-claude-md">
  Dónde AGENTS.md difiere de CLAUDE.md
</h3>

Un `AGENTS.md` que Claude lee a través de la configuración **Project instructions** difiere de un `CLAUDE.md` en estos lugares:

|                                                                                                                                                      | `CLAUDE.md`                                                                        | `AGENTS.md` leído a través de la configuración                                                                      |
| :--------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------ |
| [Hooks `InstructionsLoaded`](/docs/es/hooks#instructionsloaded)                                                                                           | Se activan                                                                         | No se activan. Se activan como de costumbre para un `AGENTS.md` que un `CLAUDE.md` importa o vincula simbólicamente |
| Directorios que agrega con `--add-dir` mientras [`CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD`](#load-from-additional-directories) está establecido | Su `CLAUDE.md` se carga                                                            | Su `AGENTS.md` no se carga                                                                                          |
| Una importación `@path` de un archivo fuera de su directorio de trabajo                                                                              | Claude Code le pide que apruebe [importaciones externas](#import-additional-files) | Se carga solo si ya aprobó importaciones externas para este proyecto, sin solicitud                                 |

<h3 id="remove-an-earlier-agents-md-workaround">
  Elimine una solución anterior de AGENTS.md
</h3>

Si configuró Claude Code para leer `AGENTS.md` antes de que lo hiciera por su cuenta, aquí está lo que debe hacer con cada configuración común:

* **Un `CLAUDE.md` que contiene `@AGENTS.md`**: puede dejarlo. Mantener la importación nunca hace que Claude lea `AGENTS.md` dos veces, cualquiera que sea el valor de **Project instructions** que use. Elimine el `CLAUDE.md` si no contiene nada más, o manténgalo si algunas de sus sesiones [no pueden cargar `AGENTS.md` directamente](#when-agents-md-support-is-unavailable).
* **Un `CLAUDE.md` que le dice a Claude en palabras que lea `AGENTS.md`**: Claude ve `AGENTS.md` solo si decide abrir el archivo. Elimine el `CLAUDE.md` para que Claude lea `AGENTS.md` directamente, o reemplace la oración con una importación `@AGENTS.md`.
* **Un `CLAUDE.md` vinculado simbólicamente a `AGENTS.md`**: nada, o elimine el vínculo simbólico. De cualquier forma, Claude lee el contenido una vez.
* **Un hook `SessionStart` que imprime `AGENTS.md`**: elimínelo. Una vez que Claude lee `AGENTS.md` directamente, el hook agrega una segunda copia al contexto.

<h3 id="share-one-file-with-other-coding-tools">
  Comparta un archivo con otras herramientas de codificación
</h3>

Cuando Claude no está leyendo su `AGENTS.md` directamente, aún puede mantenerlo como el archivo único que comparten todas las herramientas poniendo una importación `@AGENTS.md` en un `CLAUDE.md` junto a él. Haga esto cuando su proyecto también tenga un `CLAUDE.md`, cuando haya establecido **Project instructions** en `claude-md`, o en sesiones que [no puedan cargar `AGENTS.md`](#when-agents-md-support-is-unavailable). Agregue cualquier instrucción específica de Claude debajo de la importación, y Claude lee el archivo importado primero, luego el resto:

```markdown CLAUDE.md theme={null}
@AGENTS.md

## Claude Code

Use plan mode for changes under `src/billing/`.
```

Si no necesita contenido específico de Claude, un vínculo simbólico también funciona:

```bash theme={null}
ln -s AGENTS.md CLAUDE.md
```

El comando no imprime salida en caso de éxito. Antes de elegir el vínculo simbólico sobre la importación, verifique estas restricciones:

* **Edición**: Claude lee `CLAUDE.md` a través del vínculo, pero las herramientas Edit y Write [se niegan a escribir a través de un vínculo simbólico](/docs/es/errors#refusing-after-a-symlink-changed), y la negativa dirige a Claude a editar el destino del vínculo, `AGENTS.md`, en su lugar
* **Windows**: si usted o alguien que clona el repositorio trabaja en Windows, use la importación `@AGENTS.md` en su lugar. Crear un vínculo simbólico allí requiere privilegios de administrador o modo de desarrollador, y Git verifica un vínculo simbólico comprometido como un archivo de texto sin formato a menos que `core.symlinks` esté habilitado, lo que deja ese clon con un `CLAUDE.md` de una línea en lugar de sus instrucciones

Con cualquiera de los enfoques, ejecute `/context` en su siguiente sesión y confirme que `CLAUDE.md` aparece bajo **Memory files**.

<h3 id="migrate-instructions-from-other-tools">
  Migre instrucciones de otras herramientas
</h3>

Ejecutar [`/init`](/docs/es/commands) lee los archivos de instrucciones de otras herramientas e incorpora las partes relevantes en el `CLAUDE.md` generado:

* Reglas de Cursor en `.cursor/rules/` o `.cursorrules`
* Reglas de Copilot en `.github/copilot-instructions.md`
* Con `CLAUDE_CODE_NEW_INIT=1` establecido: `AGENTS.md`, `.devin/rules/`, `.windsurf/rules/` o `.windsurfrules`, y `.clinerules`

También puede ejecutar [`/import`](/docs/es/commands) para traer la configuración de un agente de codificación compatible a Claude Code, que agrega una copia única de archivos de instrucciones como `AGENTS.md` al `CLAUDE.md` coincidente y traslada servidores MCP, comandos, subagentes y skills. Requiere Claude Code v2.1.213 o posterior.

<h2 id="auto-memory">
  Auto memory
</h2>

Auto memory permite que Claude acumule conocimiento entre sesiones sin que usted escriba nada. Mientras trabaja, Claude guarda cuatro tipos de notas para sí mismo. Claude registra el tipo como un campo `type` en el frontmatter del archivo de memoria:

* `user`: su rol, experiencia y preferencias de trabajo
* `feedback`: correcciones que usted le da a Claude y enfoques que usted confirma
* `project`: trabajo en curso, plazos y decisiones que Claude no puede derivar del código o del historial de git
* `reference`: dónde encontrar información fuera del proyecto, como un rastreador de problemas o un panel de control

Claude omite cualquier cosa que pueda derivar del código base, como arquitectura, rutas de archivo o correcciones de depuración. También omite cualquier cosa que sus archivos CLAUDE.md ya digan.

Claude no guarda algo cada sesión. Decide qué vale la pena recordar basándose en si la información sería útil en una conversación futura.

<h3 id="enable-or-disable-auto-memory">
  Habilitar o deshabilitar auto memory
</h3>

Auto memory está habilitado de forma predeterminada. Para alternarlo, abra `/memory` en una sesión y use el botón de alternancia de auto memory, que guarda `autoMemoryEnabled` en la configuración de usuario en `~/.claude/settings.json`. Para desactivarlo en un único proyecto, establezca `autoMemoryEnabled` en la configuración de ese proyecto:

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

Si establece [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/es/sessions#name-the-project-directory-yourself) junto a `CLAUDE_CONFIG_DIR`, Claude Code usa ese nombre como el directorio `<project>` bajo `<config dir>/projects/` en lugar de cualquier repositorio que inicie, por lo que los proyectos iniciados con ese directorio de configuración comparten un directorio de auto memory. Requiere Claude Code v2.1.234 o posterior.

Para almacenar auto memory en una ubicación diferente, establezca `autoMemoryDirectory` en su `settings.json`. Se lee desde cualquier [ámbito de configuración](/docs/es/settings#settings-precedence): usuario, proyecto, local, política, o `--settings`.

```json theme={null}
{
  "autoMemoryDirectory": "~/my-custom-memory-dir"
}
```

El valor debe ser una ruta absoluta o comenzar con `~/`.

Cuando se establece en `.claude/settings.json` o `.claude/settings.local.json` de un proyecto, Claude Code lo respeta bajo la misma [regla de confianza del espacio de trabajo que los hooks en archivos de configuración](/docs/es/permissions#what-runs-before-you-trust-a-folder). Mientras [`permissions.blockReadsOutsideWorkingDirectories`](/docs/es/settings-reference#permissions-blockreadsoutsideworkingdirectories) está activado, Claude Code no carga auto memory desde un directorio que un [archivo de configuración suministrado por el repositorio](/docs/es/permissions#when-your-local-settings-file-needs-trust) elige y no guarda nada en él, dondequiera que se encuentre ese directorio.

El directorio contiene un índice `MEMORY.md` y un archivo de tema por memoria:

```text theme={null}
~/.claude/projects/<project>/memory/
├── MEMORY.md           # Índice, una línea por memoria, cargado en cada sesión
├── user_role.md        # Una memoria
├── feedback_testing.md # Una memoria
└── ...                 # Cualquier otro archivo de tema que Claude cree
```

`MEMORY.md` actúa como un índice del directorio de memoria. Claude lee y escribe archivos en este directorio durante su sesión, usando `MEMORY.md` para mantener un registro de lo que se almacena dónde.

Auto memory es local de la máquina. Todos los worktrees y subdirectorios dentro del mismo repositorio git comparten un directorio de auto memory. Los archivos no se comparten entre máquinas o entornos en la nube.

Claude Code elimina transcripciones de sesiones antiguas después del período de retención [`cleanupPeriodDays`](/docs/es/settings-reference#cleanupperioddays), pero excluye los archivos de memoria en el directorio de memoria de ese [barrido de retención](/docs/es/claude-directory#cleaned-up-automatically). `MEMORY.md` y los archivos de tema permanecen hasta que usted o Claude los edite o elimine.

<h3 id="how-it-works">
  Cómo funciona
</h3>

Las primeras 200 líneas de `MEMORY.md`, o los primeros 25KB, lo que sea menor, se cargan al inicio de cada conversación. El contenido más allá de ese umbral no se carga al inicio de la sesión. Claude mantiene `MEMORY.md` conciso moviendo notas detalladas a archivos de tema separados.

Después de que Claude escribe en `MEMORY.md`, Claude Code mide el archivo contra los límites de lectura de 200 líneas y 25KB. Si el archivo está cerca de un límite, Claude Code le recuerda a Claude que lo acorte: mantenga una línea por entrada, mueva detalles a archivos de tema y combine o elimine entradas obsoletas. Si el archivo supera un límite, la escritura aún tiene éxito, pero Claude Code devuelve un [error indicándole a Claude que reescriba el índice](/docs/es/errors#memory-index-is-over-its-read-limit), porque todo lo que está más allá del límite se descarta en la siguiente carga.

Este límite se aplica solo a `MEMORY.md`. Claude Code carga un archivo CLAUDE.md de hasta 4 MiB en su totalidad y omite un archivo más grande. Los archivos más cortos producen mejor adherencia.

Claude Code no carga archivos de tema como `user_role.md` o `feedback_testing.md` al iniciar. Claude los lee bajo demanda usando sus herramientas de archivo estándar cuando necesita la información.

La auto memory de la conversación principal no se carga en [subagentes](/docs/es/sub-agents#what-loads-at-startup); la excepción es un [fork](/docs/es/sub-agents#fork-the-current-conversation), que hereda la conversación principal y el prompt del sistema. La auto memory propia de un subagente, habilitada con el campo `memory` del subagente, es un directorio separado.

Claude lee y escribe archivos de memoria durante su sesión. Cuando ve mensajes como "Saved 2 memories" o "Recalled 2 memories" en la interfaz de Claude Code, Claude está actualizando o leyendo activamente desde `~/.claude/projects/<project>/memory/`.

Cuando Claude escribe un archivo de memoria que comienza con frontmatter YAML, Claude Code registra la hora de escritura en un campo `modified` del frontmatter como una marca de tiempo ISO 8601. La marca de tiempo muestra cuán actual es el hecho, tanto para usted como para Claude cuando lo vuelve a leer. Cualquier archivo que tenga frontmatter obtiene el campo la próxima vez que Claude lo escribe, incluidos los archivos creados en versiones anteriores; Claude Code nunca agrega frontmatter a un archivo que no lo tenga. El campo `modified` requiere Claude Code v2.1.214 o posterior.

<h3 id="audit-and-edit-your-memory">
  Auditar y editar su memoria
</h3>

Los archivos de auto memory son markdown plano que puede editar o eliminar en cualquier momento. Ejecute [`/memory`](#view-and-edit-with-%2Fmemory) para examinar y abrir archivos de memoria desde dentro de una sesión.

<h2 id="view-and-edit-with-/memory">
  Ver y editar con `/memory`
</h2>

El comando `/memory` enumera sus archivos CLAUDE.md, CLAUDE.local.md y otros archivos de memoria en los ámbitos de usuario y proyecto, incluidas las entradas CLAUDE.md de usuario y proyecto para archivos que aún no existen. También le permite alternar auto memory activado o desactivado y proporciona una opción para abrir la carpeta de auto memory. Seleccione cualquier archivo para abrirlo en su editor; seleccionar uno que aún no existe lo crea primero. Para verificar qué archivos `CLAUDE.md` y archivos de reglas se cargaron en la sesión actual, ejecute `/context`.

Los editores GUI como VS Code abren el archivo en una ventana separada, y puede seguir usando la sesión mientras está abierto. Antes de v2.1.216, `/memory` esperaba a que cerrara el archivo antes de responder. Los editores de terminal como Vim toman el control del terminal hasta que salga.

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

* Ejecute `/context` y verifique la lista bajo **Memory files** para verificar que sus archivos CLAUDE.md y CLAUDE.local.md se cargaron. Si un archivo `CLAUDE.md` no aparece allí, Claude no puede verlo. Use `/memory` para abrir y editar los archivos.
* Verifique que el CLAUDE.md relevante esté en una ubicación que se cargue para su sesión (consulte [Elija dónde colocar los archivos CLAUDE.md](#choose-where-to-put-claude-md-files)).
* Haga instrucciones más específicas. "Usar indentación de 2 espacios" funciona mejor que "formatear código bien".
* Busque instrucciones conflictivas en archivos CLAUDE.md. Si dos archivos dan orientación diferente para el mismo comportamiento, Claude puede elegir uno arbitrariamente.

Si la instrucción es algo que debe ejecutarse en un punto específico, como antes de cada commit o después de cada edición de archivo, escríbala como un [hook](/docs/es/hooks-guide) en su lugar. Los hooks se ejecutan como comandos de shell en eventos de ciclo de vida fijos y se aplican independientemente de lo que Claude decida hacer.

Para instrucciones que desea a nivel de prompt del sistema, use [`--append-system-prompt`](/docs/es/cli-reference#system-prompt-flags). Esto debe pasarse en el lanzamiento, por lo que es más adecuado para scripts y automatización que para uso interactivo. Para saber cómo se comporta cuando reanuda una conversación, consulte [System prompt flags in resumed conversations](/docs/es/cli-reference#system-prompt-flags-in-resumed-conversations).

<Tip>
  Use el hook [`InstructionsLoaded`](/docs/es/hooks#instructionsloaded) para registrar exactamente qué archivos de instrucciones se cargan, cuándo se cargan y por qué. Esto es útil para depurar reglas específicas de ruta o archivos cargados perezosamente en subdirectorios.
</Tip>

<h3 id="my-agents-md-isn’t-loading">
  Mi AGENTS.md no se está cargando
</h3>

Si su repositorio tiene un `AGENTS.md` y Claude no parece saber qué dice, la causa habitual es un `CLAUDE.md` en algún lugar de la ruta del proyecto. Por defecto, Claude lee `AGENTS.md` solo cuando no tiene `CLAUDE.md` o `CLAUDE.local.md` en su directorio de trabajo o por encima de él. Verifique estos en orden:

1. Busque un `CLAUDE.md`, `.claude/CLAUDE.md`, o `CLAUDE.local.md` en su directorio de trabajo o en cualquier directorio por encima de él, excepto su `~/.claude/CLAUDE.md`. Si encuentra uno, Claude lo lee en lugar de `AGENTS.md` a menos que establezca **Project instructions** en `claude-md-and-agents-md`.
2. Ejecute `claude --version` y confirme v2.1.277 o posterior. Antes de v2.1.281, algunas sesiones, como las en Amazon Bedrock o con telemetría deshabilitada, [no podían cargar `AGENTS.md`](#when-agents-md-support-is-unavailable) tampoco, por lo que en esas versiones actualice a v2.1.281 o posterior.
3. Escriba `/config` en su sesión para abrir el panel de configuración y confirme que **Project instructions** no está establecido en `claude-md` o `managed-only`. Si no ve la configuración allí en absoluto, su sesión es una que [no puede cargar `AGENTS.md`](#when-agents-md-support-is-unavailable).

Para verificar si Claude leyó su `AGENTS.md`, ejecute `/memory` y busque su ruta en la lista.

Antes de v2.1.280, `/memory` y `/context` no listaban un `AGENTS.md` que Claude leyera directamente. En esas versiones, pregúntele a Claude qué dicen sus instrucciones de proyecto en su lugar.

Si desea mantener el `CLAUDE.md` que encontró, o su sesión no puede cargar `AGENTS.md`, [agregue un `CLAUDE.md` junto a su `AGENTS.md` que lo importe](#share-one-file-with-other-coding-tools).

<h3 id="i-don’t-know-what-auto-memory-saved">
  No sé qué guardó auto memory
</h3>

Ejecute `/memory` y seleccione la carpeta de auto memory para examinar lo que Claude ha guardado. Todo es markdown plano que puede leer, editar o eliminar.

<h3 id="my-claude-md-is-too-large">
  Mi CLAUDE.md es demasiado grande
</h3>

Los archivos de más de 200 líneas consumen más contexto y pueden reducir la adherencia. Claude Code omite un archivo de más de 4 MiB. Use [reglas con alcance de ruta](#path-specific-rules) para cargar instrucciones solo cuando Claude trabaja con archivos coincidentes, o recorte contenido que no sea necesario en cada sesión. Dividir en [importaciones `@path`](#import-additional-files) ayuda a la organización pero no reduce el contexto, ya que los archivos importados se cargan al iniciar.

La revisión [`/doctor`](/docs/es/commands#all-commands) propone recortes para un CLAUDE.md registrado: elimina contenido que Claude puede derivar de la base de código, como diseños de directorios, listas de dependencias y descripción general de la arquitectura, y mantiene trampas, justificación y convenciones que difieren de los valores predeterminados de las herramientas. La verificación de recorte requiere Claude Code v2.1.206 o posterior.

<h3 id="instructions-seem-lost-after-/compact">
  Las instrucciones parecen perdidas después de `/compact`
</h3>

CLAUDE.md de raíz de proyecto sobrevive a la compactación: después de `/compact`, Claude vuelve a leer desde el disco e lo reinyecta en la sesión. Los archivos CLAUDE.md anidados en subdirectorios y reglas con [frontmatter `paths:`](#path-specific-rules) se recargan cuando Claude lee archivos a los que se aplican.

Si una instrucción desapareció después de la compactación, se dio solo en la conversación, vive en un CLAUDE.md anidado que aún no se ha recargado, o es una regla con alcance de ruta que no ha coincidido con un archivo desde entonces. Agregue instrucciones solo de conversación a CLAUDE.md para que persistan. Consulte [Qué sobrevive a la compactación](/docs/es/context-window#what-survives-compaction) para el desglose completo.

Consulte [Escriba instrucciones efectivas](#write-effective-instructions) para obtener orientación sobre tamaño, estructura y especificidad.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Depurar su configuración](/docs/es/debug-your-config): diagnosticar por qué CLAUDE.md o configuración no están surtiendo efecto
* [Skills](/docs/es/skills): empaquetar flujos de trabajo repetibles que se cargan bajo demanda
* [Settings](/docs/es/settings): configurar el comportamiento de Claude Code con archivos de configuración
* [Subagent memory](/docs/es/sub-agents#enable-persistent-memory): permitir que los subagents mantengan su propia auto memory
