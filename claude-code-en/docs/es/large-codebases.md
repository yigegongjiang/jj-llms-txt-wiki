> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurar Claude Code en un monorepo o codebase grande

> Configure Claude Code para monorepos y codebases de árbol único grande con archivos CLAUDE.md anidados, worktrees dispersos, inteligencia de código y skills por paquete para que Claude se mantenga enfocado en el código en el que está trabajando.

Un codebase grande puede ser un repositorio con millones de líneas o un monorepo con muchos paquetes. Claude Code funciona en cualquier tamaño, pero a medida que el codebase crece, los valores predeterminados ajustados para proyectos más pequeños pueden llenar la ventana de contexto con instrucciones y lecturas de archivos no relacionadas con la tarea, costando tokens y degradando el rendimiento de Claude.

Esta guía muestra a desarrolladores individuales y equipos de ingeniería cómo limitar Claude a la parte del codebase que toca una tarea. Cada sección indica si una configuración es personal para su máquina o comprometida con el repositorio.

<h2 id="what-this-guide-covers">
  Lo que cubre esta guía
</h2>

La [tabla a continuación](#settings-on-this-page) enumera cada configuración y lo que logra. El [árbol de archivos después de ella](#the-example-monorepo) es el monorepo de ejemplo al que se refieren todos los ejemplos de código en esta página.

<h3 id="settings-on-this-page">
  Configuraciones en esta página
</h3>

Cada configuración a continuación es independiente. Se superponen en lugar de reemplazarse entre sí, así que aplique las que se ajusten a su repositorio. [Elija dónde iniciar Claude](#choose-where-to-start-claude) determina dónde viven sus archivos de configuración, así que léalo primero. [Póngalo todo junto](#put-it-together) muestra todos ellos combinados.

| Quiero                                                                                                             | Usar                                                                                           |
| :----------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------- |
| Cargar solo las convenciones para el código que toca, en lugar de un archivo raíz que cubra cada subsistema        | Archivos [CLAUDE.md](#layer-claude-md-files-by-directory) por directorio                       |
| Excluir archivos CLAUDE.md para paquetes en los que nunca trabajo                                                  | [`claudeMdExcludes`](#exclude-irrelevant-claude-md-files)                                      |
| Bloquear a Claude de abrir salida de compilación, código generado y dependencias vendidas                          | Reglas de negación [`Read`](#block-reads-of-generated-and-vendored-code) en `permissions.deny` |
| Encontrar la definición de un símbolo o llamadores a través del servidor de lenguaje en lugar de escanear archivos | Un [plugin de inteligencia de código](#reduce-file-reads-with-code-intelligence)               |
| Verificar solo los directorios que necesita una tarea cuando Claude crea un worktree                               | [`worktree.sparsePaths`](#check-out-only-the-directories-you-need)                             |
| Leer y editar un paquete hermano u otro repositorio desde la misma sesión                                          | [`--add-dir`](#grant-access-across-packages-or-repositories) o `additionalDirectories`         |
| Dar a Claude procedimientos específicos de un área que se cargan solo cuando son relevantes                        | [Skills](#add-per-directory-skills) por directorio                                             |
| Reemplazar muchos archivos CLAUDE.md por directorio con un conjunto de convenciones que todos instalan             | Un [plugin](#centralize-conventions-when-layering-stops-scaling) en un marketplace interno     |

<Tip>
  Para técnicas de flujo de trabajo que mantengan el contexto pequeño en cualquier repositorio, como [ejecutar exploración en un subagente](/docs/es/best-practices#use-subagents-for-investigation) para que las lecturas de archivos se mantengan fuera de la conversación principal, consulte [Mejores prácticas para Claude Code](/docs/es/best-practices). Para implementar una configuración de línea base para cada desarrollador en su organización, consulte [Configurar Claude Code para su organización](/docs/es/admin-setup).
</Tip>

<h3 id="the-example-monorepo">
  El monorepo de ejemplo
</h3>

Los ejemplos en toda esta página se refieren a un monorepo con tres paquetes. Los mismos patrones funcionan en un codebase de árbol único grande: donde un ejemplo usa `packages/api/`, sustituya su propio directorio de subsistema como `src/backend/` o `lib/core/`.

```text theme={null}
monorepo/
  CLAUDE.md                     # instrucciones raíz
  packages/
    api/
      CLAUDE.md                 # instrucciones específicas de API
      .claude/skills/
      src/
    web/
      CLAUDE.md                 # instrucciones específicas de frontend
      .claude/skills/
      src/
    shared/
      CLAUDE.md                 # instrucciones de biblioteca compartida
      src/
```

<h2 id="choose-where-to-start-claude">
  Elija dónde iniciar Claude
</h2>

Dónde inicia `claude` determina qué archivos puede leer y editar Claude sin una concesión de permiso adicional, qué archivos CLAUDE.md se cargan en el contexto al inicio, y qué configuraciones de proyecto se aplican.

| Iniciar desde        | Acceso a archivos                        | CLAUDE.md cargado al inicio                                                            | Usar cuando                                         |
| :------------------- | :--------------------------------------- | :------------------------------------------------------------------------------------- | :-------------------------------------------------- |
| Raíz del repositorio | Cada archivo                             | Solo raíz; los archivos de subdirectorio se cargan bajo demanda cuando Claude lee allí | Las tareas abarcan múltiples paquetes o subsistemas |
| Un subdirectorio     | Solo ese subárbol, hasta que otorgue más | El de ese directorio más el de cada antecesor                                          | El trabajo se limita a un paquete o subsistema      |

Las configuraciones de proyecto en `.claude/settings.json` se cargan solo desde su directorio de inicio y no se heredan de directorios padres de la manera que lo hacen los archivos CLAUDE.md: un `.claude/settings.json` en la raíz del repositorio se aplica solo cuando inicia desde la raíz.

Cada sección a continuación indica si su archivo de configuración pertenece a la raíz del repositorio o al subdirectorio desde el que inicia, y si se compromete o se mantiene localmente.

<h2 id="layer-claude-md-files-by-directory">
  Capa de archivos CLAUDE.md por directorio
</h2>

En un codebase grande, un único CLAUDE.md en la raíz del repositorio tiende a crecer para cubrir las convenciones de cada subsistema, costando contexto en instrucciones no relacionadas con la tarea actual, o permanecer demasiado genérico para ser útil. Dividir instrucciones en archivos por directorio significa que Claude carga reglas de todo el repositorio más solo las convenciones para el código en el que está trabajando.

Claude Code carga cada archivo [CLAUDE.md](/docs/es/memory) desde su directorio de trabajo y cada directorio padre al inicio, luego carga el archivo de cada subdirectorio bajo demanda cuando lee archivos allí. Un archivo raíz establece reglas de todo el repositorio y cada subdirectorio agrega las suyas propias.

Una división común es dos niveles:

* **CLAUDE.md raíz**: instrucciones que se aplican en todas partes, como estándares de codificación, convenciones de commit y diseño del repositorio
* **CLAUDE.md por subdirectorio**: convenciones específicas del stack de esa área. En un monorepo eso es uno por paquete. En un árbol único grande es uno por subsistema como `src/db/` o `src/api/`

Comprometa estos archivos al repositorio para que los compañeros de equipo los hereden. El propietario de cada directorio típicamente mantiene su archivo.

El CLAUDE.md raíz orienta a Claude sobre la estructura del repositorio:

```markdown CLAUDE.md theme={null}
Este es un monorepo con tres paquetes bajo packages/:

- packages/api: API REST de Node.js con Express, TypeScript y PostgreSQL
- packages/web: frontend React con Vite, TypeScript y TailwindCSS
- packages/shared: utilidades TypeScript compartidas utilizadas por api y web

Ejecute comandos desde el directorio del paquete, no desde la raíz del monorepo.
Cada paquete tiene su propio tsconfig.json, package.json y suite de pruebas.
```

El CLAUDE.md de cada subdirectorio, aquí `packages/api/CLAUDE.md`, agrega contexto específico del stack de esa área:

```markdown packages/api/CLAUDE.md theme={null}
Este paquete es el servidor de API REST.

- Ejecutar pruebas: `npm test` (usa Vitest)
- Ejecutar servidor de desarrollo: `npm run dev` (puerto 3001)
- Migraciones de base de datos: `npm run migrate`
- Variables de entorno: copie `.env.example` a `.env`

Las rutas de API están en src/routes/. Cada archivo de ruta exporta un enrutador Express.
Las consultas de base de datos usan Knex en src/db/. Nunca escriba cadenas SQL sin procesar en manejadores de rutas.
```

Cuando inicia Claude desde `packages/api/`, carga tanto `packages/api/CLAUDE.md` como el CLAUDE.md raíz. Claude ve las instrucciones locales junto con las reglas de todo el repositorio, sin instrucciones de `packages/web/` en el contexto. Lo mismo se aplica a cualquier subdirectorio en un árbol no monorepo.

Algunas formas de mantener los archivos actuales a medida que el codebase y los modelos cambian:

* **Revisar en solicitudes de extracción**: trate las ediciones de CLAUDE.md como cualquier otro cambio de documentación para que las convenciones sigan el código
* **Revisar después de lanzamientos de modelos principales**: las instrucciones que funcionaban alrededor de una limitación de un modelo anterior pueden convertirse en sobrecarga una vez que un modelo más nuevo maneja el caso por sí solo. Por ejemplo, una regla que fuerza refactores de un solo archivo puede eliminarse una vez que se haya ido la limitación
* **Agregue un hook Stop que proponga actualizaciones**: un [hook `Stop`](/docs/es/hooks#stop) recibe la ruta a la transcripción de la sesión cuando Claude termina de responder, por lo que un script puede revisar la sesión y proponer actualizaciones de CLAUDE.md mientras la brecha que expuso es fresca

Para más información sobre cómo se cargan e interactúan los archivos CLAUDE.md, consulte [Memoria e instrucciones de proyecto](/docs/es/memory).

<h3 id="choose-between-per-directory-claude-md-and-path-scoped-rules">
  Elija entre CLAUDE.md por directorio y reglas con alcance de ruta
</h3>

Los archivos `CLAUDE.md` por directorio y [reglas con alcance de ruta](/docs/es/memory#path-specific-rules) bajo `.claude/rules/` ambos le permiten dirigir instrucciones a parte del árbol. Difieren en dónde vive el archivo y cuándo se carga.

| Enfoque                                       | Ubicación del archivo                         | Se carga cuando                                                                                   | Usar cuando                                                                                                      |
| :-------------------------------------------- | :-------------------------------------------- | :------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------- |
| CLAUDE.md por directorio                      | Dentro del directorio, junto a su código      | Al inicio cuando se inicia desde ese directorio, o bajo demanda cuando Claude lee un archivo allí | Los propietarios del directorio mantienen sus propias convenciones; las instrucciones se versionan con el código |
| Regla con alcance de ruta en `.claude/rules/` | `.claude/` central en la raíz del repositorio | Cuando Claude trabaja con un archivo que coincide con el glob `paths:` de la regla                | Desea todas las convenciones en un lugar, o la misma regla se aplica a muchas rutas dispersas                    |

Para una comparación que también cubre skills, consulte [Comparar características similares](/docs/es/features-overview#compare-similar-features).

<h3 id="exclude-irrelevant-claude-md-files">
  Excluir archivos CLAUDE.md irrelevantes
</h3>

Cuando inicia Claude desde la raíz del repositorio, el CLAUDE.md de cada subdirectorio se carga tan pronto como Claude lee un archivo en ese directorio. La configuración `claudeMdExcludes` omite archivos específicos por ruta o patrón glob para que nunca se carguen.

Úselo para directorios en los que nunca trabaja, como paquetes de otros equipos, código heredado o subárboles vendidos. La lista de exclusión es estática, no un interruptor por tarea. Para enfocarse en un paquete hoy y otro mañana, [inicie Claude desde el directorio de ese paquete](#choose-where-to-start-claude) en lugar de editar exclusiones.

Si solo desea estas exclusiones para usted, coloque la configuración en `.claude/settings.local.json`. Claude Code ignora ese archivo cuando lo crea; como lo está creando a mano aquí, agréguelo a su gitignore. Los patrones usan sintaxis glob coincidida contra rutas de archivo absolutas, así que comience patrones de estilo relativo con `**/` para coincidir en cualquier lugar del árbol. El ejemplo a continuación excluye paquetes propiedad de otros equipos:

```json .claude/settings.local.json theme={null}
{
  "claudeMdExcludes": [
    "**/packages/admin-dashboard/**",
    "**/packages/legacy-*/**"
  ]
}
```

Esto omite cada CLAUDE.md y archivo de reglas bajo esos paquetes. El CLAUDE.md raíz y los paquetes en los que sí trabaja aún se cargan normalmente.

Estos patrones cubren otros casos comunes:

* `"**/packages/*/CLAUDE.md"`: excluye el CLAUDE.md de cada paquete mientras mantiene la raíz
* `"**/packages/web/**"`: excluye todo bajo el paquete web, incluidas las reglas
* `"/home/user/monorepo/legacy/CLAUDE.md"`: excluye un archivo específico por ruta absoluta

Los archivos CLAUDE.md de política administrada no pueden excluirse, por lo que las instrucciones de toda la organización siempre se aplican. Puede establecer `claudeMdExcludes` en cualquier [alcance de configuración](/docs/es/settings#configuration-scopes): usuario, proyecto, local o administrado. Los arreglos se fusionan entre alcances, por lo que un equipo puede establecer valores predeterminados a nivel de proyecto mientras los individuos agregan anulaciones locales.

Para la documentación completa de exclusión, consulte [Excluir archivos CLAUDE.md específicos](/docs/es/memory#exclude-specific-claude-md-files).

<h2 id="reduce-what-claude-reads">
  Reducir lo que Claude lee
</h2>

Las instrucciones son solo parte de lo que termina en el contexto de Claude. Las lecturas de archivos son otro costo que crece con el codebase. Las configuraciones a continuación bloquean lecturas de rutas irrelevantes y reemplazan escaneos exhaustivos con búsquedas de servidor de lenguaje.

<h3 id="block-reads-of-generated-and-vendored-code">
  Bloquear lecturas de código generado y vendido
</h3>

Las búsquedas de contenido de Claude respetan `.gitignore` de forma predeterminada, por lo que las rutas ya enumeradas allí, como `node_modules/`, `dist/` y `build/`, se mantienen fuera de los resultados de búsqueda sin configuración adicional.

Para rutas que se verifican, como un SDK vendido o código generado comprometido, agregue reglas de negación `Read` en `permissions.deny` para bloquear a Claude de abrir esos archivos incluso cuando una búsqueda los enumera.

Para aplicar estas exclusiones para todos los que trabajan en el repositorio, comprométalas a `.claude/settings.json`. Para mantenerlas personales, use `.claude/settings.local.json` en su lugar. Como otras configuraciones de proyecto en esta página, estos archivos se cargan solo desde su directorio de inicio. Colóquelos en la raíz del repositorio si inicia Claude allí, o en cada `.claude/` del paquete si inicia desde subdirectorios. Para aplicar las mismas reglas de negación en cada sesión independientemente del directorio de inicio, establézcalas en [configuraciones administradas](/docs/es/settings#settings-files), que las configuraciones de usuario y proyecto no pueden anular.

El ejemplo a continuación bloquea artefactos de compilación y un SDK vendido:

```json .claude/settings.json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./**/dist/**)",
      "Read(./**/build/**)",
      "Read(./**/*.generated.*)",
      "Read(./vendor/**)"
    ]
  }
}
```

Las reglas de negación cubren las herramientas de archivo integradas de Claude y los comandos Bash reconocidos, incluidos `cat`, `head`, `grep` y `find`, cuando se pasa una ruta denegada como argumento. No filtran rutas denegadas de la salida de una búsqueda recursiva, y no cubren subprocesos arbitrarios que abren archivos por sí solos. Para la sintaxis de patrón completa, consulte [Reglas de permiso Read y Edit](/docs/es/permissions#read-and-edit).

<h3 id="reduce-file-reads-with-code-intelligence">
  Reducir lecturas de archivos con inteligencia de código
</h3>

En un codebase grande, encontrar dónde se define o usa un símbolo puede costar muchas lecturas de archivos y llamadas grep. Los [plugins de inteligencia de código](/docs/es/discover-plugins#code-intelligence) conectan Claude a un servidor de lenguaje para que pueda saltar a definiciones, encontrar referencias y mostrar errores de tipo directamente en lugar de escanear el árbol.

El marketplace oficial tiene plugins para TypeScript, Python, Go, Rust y otros lenguajes comunes. El ejemplo a continuación instala el plugin de TypeScript:

```shell theme={null}
/plugin install typescript-lsp@claude-plugins-official
```

Para habilitar un plugin para todos en el repositorio en lugar de instalarlo usted mismo, agréguelo a la configuración de proyecto [`enabledPlugins`](/docs/es/settings#plugin-settings).

Los plugins de inteligencia de código requieren el binario del servidor de lenguaje del lenguaje en la máquina de cada desarrollador. Consulte [qué binario requiere cada lenguaje](/docs/es/discover-plugins#code-intelligence). La instalación desde el marketplace oficial requiere acceso de red a GitHub, donde se aloja el marketplace. En una red restringida, [agregue el marketplace desde un host Git interno o ruta local](/docs/es/discover-plugins#add-from-other-git-hosts) en su lugar.

Esto se empareja bien con `claudeMdExcludes` y las reglas de negación `Read` anteriores. Esos mantienen contenido irrelevante fuera del contexto, e inteligencia de código mantiene a Claude de leer a través de lo que queda para ubicar una definición.

<h2 id="scope-worktrees-and-file-access">
  Alcance de worktrees y acceso a archivos
</h2>

Estas configuraciones controlan qué está en disco en worktrees y qué directorios puede leer y escribir Claude más allá de su punto de inicio.

<h3 id="check-out-only-the-directories-you-need">
  Verificar solo los directorios que necesita
</h3>

La bandera `--worktree` inicia una sesión en un nuevo worktree de git para que los cambios se mantengan aislados de su checkout principal. De forma predeterminada, verifica todo el repositorio. En un repositorio grande, la configuración `worktree.sparsePaths` usa git sparse-checkout para escribir solo los directorios enumerados más archivos de nivel raíz en disco, para que los worktrees se inicien más rápido y usen menos espacio.

Si todos los que trabajan en este directorio necesitan las mismas rutas, comprometa la configuración a `.claude/settings.json`. Para agregar rutas para usted, use `.claude/settings.local.json`: las listas se fusionan entre alcances, por lo que un archivo local puede agregar rutas a la lista comprometida pero no eliminarlas. El ejemplo a continuación muestra el archivo comprometido:

```json .claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ]
  }
}
```

Cuando Claude crea un worktree, verifica solo `.claude/`, `packages/api/` y `packages/shared/` en lugar del árbol completo. Las rutas en `sparsePaths` son relativas a la raíz del repositorio, independientemente de qué subdirectorio inicie Claude desde. Cualquier ruta de directorio funciona aquí, no solo raíces de paquete.

Esto es particularmente útil para [aislamiento de worktree de subagente](/docs/es/worktrees#isolate-subagents-with-worktrees). Los subagentes son instancias paralelas de Claude generadas para subtareas, y cada una que se ejecuta en un worktree obtiene un checkout ligero en lugar del árbol completo. Todos los worktrees en una sesión comparten el mismo `sparsePaths`, por lo que si un subagente necesita `packages/api/` y otro necesita `packages/web/`, enumere ambos.

Enumere directorios en `sparsePaths`, no archivos individuales. Los archivos de nivel raíz como `package.json`, `tsconfig.base.json` y archivos de bloqueo siempre se verifican junto con los directorios que enumera. Los directorios de nivel raíz no, así que incluya `.claude` en la lista si desea que el `.claude/settings.json`, `.claude/rules/` o `.claude/skills/` de la raíz del repositorio estén disponibles dentro del worktree.

El sparse checkout requiere que git habilite `extensions.worktreeConfig` en el `.git/config` compartido del repositorio mientras existe un worktree disperso. Claude Code elimina esa entrada después de que se elimina el último worktree, pero solo si Claude Code la agregó. Nunca elimina un valor que usted estableció. Antes de v2.1.207, la entrada permanecía después de que se eliminaba el último worktree, y las herramientas basadas en go-git como `tea` no podían abrir el repositorio hasta que ejecutaba `git config --unset extensions.worktreeConfig`.

Para evitar duplicar directorios grandes como `node_modules` entre worktrees, empareje `sparsePaths` con `symlinkDirectories` en el mismo `.claude/settings.json`:

```json .claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ],
    "symlinkDirectories": [
      "node_modules"
    ]
  }
}
```

Esto crea un enlace simbólico desde el `node_modules/` de cada worktree de vuelta a la copia del repositorio principal en lugar de duplicarlo en disco.

<Note>
  Las configuraciones `sparsePaths` y `symlinkDirectories` se leen desde su directorio de inicio antes de que se cree el worktree. Después de la creación, el directorio de trabajo de la sesión es la raíz del worktree, no el subdirectorio desde el que inició. Las configuraciones de proyecto dentro del worktree por lo tanto se cargan desde el `.claude/settings.json` de la raíz del worktree, la copia verificada del archivo de raíz del repositorio. Coloque cualquier otra configuración que necesite dentro de worktrees, como reglas de permiso u hooks, en el `.claude/settings.json` de la raíz del repositorio.
</Note>

Para la referencia completa de configuraciones de worktree, consulte [Configuraciones de Worktree](/docs/es/settings#worktree-settings).

<h3 id="grant-access-across-packages-or-repositories">
  Otorgar acceso entre paquetes o repositorios
</h3>

Esta sección se aplica cuando inicia Claude desde un subdirectorio, o cuando una tarea abarca múltiples checkouts. Si inicia desde la raíz del repositorio en un árbol único grande, Claude ya tiene acceso a cada archivo y puede omitir esto.

Cuando inicia Claude desde `packages/api/`, puede leer y escribir archivos dentro de ese directorio. Si una tarea requiere cambios entre paquetes, como actualizar un tipo compartido que tanto `api` como `web` importan, necesita otorgar acceso al directorio hermano. El mismo mecanismo otorga acceso a un repositorio verificado por separado.

La configuración `additionalDirectories` en `.claude/settings.json` da a Claude acceso a directorios fuera del directorio de trabajo. El ejemplo a continuación otorga acceso a dos paquetes hermanos:

```json .claude/settings.json theme={null}
{
  "permissions": {
    "additionalDirectories": [
      "../shared",
      "../web"
    ]
  }
}
```

Las rutas relativas se resuelven contra el directorio desde el que inicia Claude. Con esta configuración, Claude puede leer y editar archivos en `packages/shared/` y `packages/web/` mientras trabaja desde `packages/api/`.

También puede otorgar acceso en tiempo de ejecución sin editar configuraciones pasando `--add-dir` cuando inicia Claude:

```bash theme={null}
claude --add-dir ../shared
```

Sin embargo agregue un directorio, Claude puede leer y editar archivos en él. Si el CLAUDE.md del directorio, archivos `.claude/rules/` y skills también se cargan depende de cómo lo agregó:

| Agregado con                             | Carga CLAUDE.md y reglas                       | Carga skills |
| :--------------------------------------- | :--------------------------------------------- | :----------- |
| Configuración `additionalDirectories`    | Nunca                                          | Nunca        |
| Bandera `--add-dir` o comando `/add-dir` | Solo con la variable de entorno a continuación | Sí           |

Para cargar archivos CLAUDE.md y reglas desde un directorio agregado con `--add-dir` o `/add-dir`, establezca la variable de entorno `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD`:

```bash theme={null}
CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1 claude --add-dir ../shared
```

La variable de entorno no tiene efecto en directorios enumerados en la configuración `additionalDirectories`. Consulte [Cargar desde directorios adicionales](/docs/es/memory#load-from-additional-directories) para detalles.

Para directorios hermanos que todos en esta área necesitan, comprometa `additionalDirectories` a `.claude/settings.json`. Para una selección personal o acceso único, use `.claude/settings.local.json` o pase `--add-dir` al inicio.

<h2 id="add-per-directory-skills">
  Agregar skills por directorio
</h2>

Cualquier subdirectorio puede definir [skills](/docs/es/skills) limitados a su propio stack. Un skill se carga bajo demanda cuando Claude determina que es relevante, por lo que las herramientas específicas de API no consumen contexto durante el trabajo de frontend.

Los skills viven bajo `.claude/skills/` dentro del directorio. Comprométalos junto con el código de esa área para que cualquiera que clone el repositorio los obtenga. En un monorepo esto puede ser un conjunto de skills por paquete. En un codebase de árbol único grande es uno por subsistema como `src/db/.claude/skills/`.

Cree un directorio de skill dentro del subdirectorio:

```bash theme={null}
mkdir -p packages/api/.claude/skills/api-testing
```

Luego escriba `SKILL.md` dentro de ese directorio, aquí `packages/api/.claude/skills/api-testing/SKILL.md`. Este ejemplo enseña a Claude los patrones de prueba del paquete API:

```markdown packages/api/.claude/skills/api-testing/SKILL.md theme={null}
---
name: api-testing
description: Patrones de prueba para el paquete API. Usar cuando escriba o modifique pruebas en packages/api/.
---

## Estructura de prueba

Las pruebas están en `src/__tests__/` reflejando la estructura del directorio `src/`.
Cada archivo de ruta tiene un archivo `.test.ts` correspondiente.

## Ejecutar pruebas

- Todas las pruebas: `npm test`
- Archivo único: `npm test -- src/__tests__/routes/users.test.ts`
- Modo watch: `npm test -- --watch`

## Utilidades de prueba

- `src/__tests__/helpers/db.ts`: proporciona `setupTestDb()` y `teardownTestDb()` para pruebas de base de datos
- `src/__tests__/helpers/auth.ts`: proporciona `createTestUser()` y `getAuthToken()` para endpoints autenticados

## Patrones

- Use `supertest` para aserciones HTTP, no fetch sin procesar
- Siempre envuelva pruebas de base de datos en una transacción que se revierta
- Simule servicios externos en `src/__tests__/mocks/`
```

Un subdirectorio diferente contiene diferentes skills de la misma manera: `packages/web/.claude/skills/component-patterns/` describe las convenciones de componentes del frontend en lugar de pruebas. Cuando Claude trabaja en un archivo en `packages/api/`, carga el skill api-testing. Cuando trabaja en `packages/web/`, carga component-patterns en su lugar. Los skills de ningún directorio se cargan durante las tareas del otro.

También puede limitar un skill por patrón de archivo en lugar de por colocación. El [campo frontmatter `paths`](/docs/es/skills#frontmatter-reference) toma patrones glob, y Claude carga el skill automáticamente solo cuando trabaja con archivos coincidentes. Úselo para un skill que vive en el `.claude/skills/` de la raíz del repositorio pero se aplica solo a ciertos archivos dondequiera que aparezcan, como un skill de migración de base de datos limitado a `**/migrations/**`.

Para más información sobre crear y organizar skills, consulte [Skills](/docs/es/skills).

<h3 id="keep-skills-discoverable">
  Mantener skills descubribles
</h3>

Con skills dispersos en muchos directorios, la lista de la que Claude elige puede crecer mucho. Claude elige un skill leyendo el nombre y descripción de cada skill descubierto, y solo el contenido completo del skill elegido se carga en el contexto. Esta sección cubre cómo mantener esa lista pequeña y escribir descripciones que sobrevivan al acortamiento.

Qué skills están en alcance depende de dónde inicie Claude:

* **Desde un subdirectorio como `packages/api/`**: skills de ese directorio, cada padre hasta la raíz del repositorio, y los niveles de usuario y empresa
* **Desde la raíz del repositorio**: skills de cada subdirectorio que Claude toca durante la sesión, que pueden acumularse en los cientos
* **Después de agregar un hermano con [`--add-dir`](#grant-access-across-packages-or-repositories)**: los skills de ese hermano también se cargan. La configuración `additionalDirectories` otorga solo acceso a archivos y no carga skills

Los nombres siempre se cargan, pero [las descripciones se acortan cuando hay muchas](/docs/es/skills#skill-descriptions-are-cut-short), lo que puede eliminar las palabras clave que Claude usa para decidir si un skill se aplica. Mantenga las descripciones cortas y comience con palabras que una solicitud contendría, como "escribir o modificar pruebas en `packages/api/`".

Para skills que muchos directorios comparten, como convenciones de PR o una lista de verificación de implementación, colóquelos en el `.claude/skills/` de la raíz del repositorio para que se carguen desde cualquier directorio de inicio. Cuando los skills compartidos necesitan su propio historial de versiones o deben funcionar entre repositorios, empaquételos como un [plugin](/docs/es/plugins) en su lugar. Los skills de plugin usan un espacio de nombres `plugin-name:skill-name`, por lo que nunca chocan con skills por directorio. Un equipo de plataforma puede versionarlos y actualizarlos en un lugar.

Para encontrar qué skills no se usan, habilite el exportador de OpenTelemetry [logs](/docs/es/monitoring-usage) y establezca `OTEL_LOG_TOOL_DETAILS=1` para que los nombres de skills se registren textualmente en lugar de redactarse. El evento [`skill_activated`](/docs/es/monitoring-usage#skill-activated-event) registra cada invocación en su atributo `skill.name`, e `invocation_trigger` registra si un comando, Claude o un skill anidado lo invocó, lo que le dice qué consolidar o retirar.

<h2 id="centralize-conventions-when-layering-stops-scaling">
  Centralizar convenciones cuando la estratificación deja de escalar
</h2>

Los archivos CLAUDE.md por directorio pueden volverse difíciles de gobernar a medida que el codebase crece. Las convenciones se desvían, los archivos se vuelven obsoletos, y nadie es propietario de la raíz. Resolver eso típicamente recae en el equipo que mantiene la configuración de Claude Code del repositorio en lugar de en cada desarrollador trabajando en su propia área.

Mueva convenciones y contenido de referencia fuera de CLAUDE.md siempre cargado y hacia mecanismos que se cargan bajo demanda:

* [Skills](/docs/es/skills): material de referencia que Claude carga solo cuando es relevante para la tarea
* [Plugins](/docs/es/plugins): paquetes versionados de skills, hooks y comandos que un equipo de plataforma posee centralmente
* [Servidores MCP](/docs/es/mcp): si su organización ya ejecuta una búsqueda de código o índice RAG sobre el repositorio, expóngalo como una herramienta MCP para que Claude lo consulte en lugar de leer archivos directamente

Consulte [configuraciones administradas por servidor o punto final](/docs/es/server-managed-settings#choose-between-server-managed-and-endpoint-managed-settings) para cómo los equipos de plataforma pueden aplicar esto centralmente.

<h3 id="recommend-the-right-plugin-at-session-start">
  Recomendar el plugin correcto al inicio de la sesión
</h3>

Una vez que las convenciones viven en plugins, un compañero de equipo que inicia Claude en una parte desconocida del árbol no tiene señal sobre qué plugin mantienen los propietarios de esa área. Un [hook `SessionStart`](/docs/es/hooks#sessionstart) puede cerrar esa brecha, ya que cualquier cosa que el hook imprima en stdout se agrega al contexto de Claude antes de la primera solicitud.

Por ejemplo, puede escribir un script que lea el directorio de inicio desde la [entrada del hook](/docs/es/hooks#common-input-fields), lo busque en un mapa de ruta a plugin comprometido al repositorio, e imprima la recomendación para que Claude la retransmita en su primera respuesta. Consulte [Automatizar acciones con hooks](/docs/es/hooks-guide) para escribir y registrar el hook.

<h2 id="put-it-together">
  Póngalo todo junto
</h2>

La configuración combinada a continuación usa el diseño del monorepo. Los mismos archivos funcionan para cualquier subdirectorio en un árbol único grande. Las configuraciones de proyecto se cargan solo desde el directorio desde el que inicia Claude, por lo que el `.claude/settings.json` de cada subdirectorio debe ser autónomo en lugar de estratificado en un archivo raíz.

El ejemplo compromete `worktree`, `additionalDirectories` y las reglas de negación `Read` en `.claude/settings.json` para que cada desarrollador en `packages/api/` obtenga el mismo acceso hermano, rutas dispersas y exclusiones. El archivo a continuación es la configuración por área comprometida para `packages/api/`:

```json packages/api/.claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ],
    "symlinkDirectories": [
      "node_modules"
    ]
  },
  "permissions": {
    "additionalDirectories": [
      "../shared"
    ],
    "deny": [
      "Read(./**/dist/**)",
      "Read(./**/build/**)"
    ]
  }
}
```

Porque esta sesión inicia desde `packages/api/`, los archivos CLAUDE.md de paquetes hermanos ya están fuera de alcance, por lo que `claudeMdExcludes` no es necesario aquí. Agréguelo al `.claude/settings.local.json` de la raíz del repositorio en su lugar si también inicia sesiones desde la raíz.

La entrada `additionalDirectories` se aplica cuando inicia Claude desde `packages/api/` directamente. Dentro de un worktree creado desde esta sesión, el directorio de trabajo es la raíz del worktree, por lo que este archivo de configuración no se carga. Los paquetes hermanos ya son alcanzables dentro del worktree sin él, pero las reglas de negación necesitan una segunda copia en el `.claude/settings.json` de la raíz del repositorio para que las sesiones de worktree las recojan, como la [nota de configuraciones de worktree](#check-out-only-the-directories-you-need) describe:

```json .claude/settings.json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./**/dist/**)",
      "Read(./**/build/**)"
    ]
  }
}
```

Después de la configuración, el repositorio tiene este diseño:

```text theme={null}
monorepo/
  CLAUDE.md
  .claude/settings.json                           # reglas de negación para sesiones de worktree
  packages/
    api/
      CLAUDE.md
      .claude/settings.json                       # worktree, additionalDirectories, reglas de negación
      .claude/skills/api-testing/SKILL.md
    web/
      CLAUDE.md
      .claude/skills/component-patterns/SKILL.md
    shared/
      CLAUDE.md
```

Con esta configuración, iniciando Claude desde `packages/api/`:

* Carga el CLAUDE.md raíz y `packages/api/CLAUDE.md`, omite `packages/web/CLAUDE.md`
* Puede leer y editar archivos en `packages/api/` y `packages/shared/`
* Omite lecturas de salida de compilación bajo `dist/` y `build/` en `packages/api/`
* Tiene el skill api-testing disponible bajo demanda
* Crea worktrees que contienen `.claude/`, `packages/api/`, `packages/shared/` y archivos de nivel raíz, con las reglas de negación aplicadas en todo el worktree desde el archivo de configuración raíz

<h2 id="scope-and-plan-changes-that-span-packages">
  Alcance y planifique cambios que abarquen paquetes
</h2>

La configuración anterior controla lo que Claude ve. Cuando un único cambio toca varios paquetes, como actualizar un tipo compartido junto con cada sitio de llamada que lo usa, cómo usted limita y secuencia la tarea también afecta el resultado.

Dos técnicas ayudan a mantener un cambio entre paquetes consistente:

* **Dé a Claude el cambio completo en una sesión**: entregar la edición compartida y sus sitios de llamada juntos mantiene las decisiones detrás de cada edición consistentes, en lugar de rederivarse por paquete
* **Guarde el plan en un archivo antes de editar**: [planifique primero](/docs/es/best-practices#explore-first-then-plan-then-code) y pida a Claude que escriba el plan en un archivo markdown en el repositorio. Una sesión larga entre paquetes [compacta su contexto](/docs/es/context-window#what-survives-compaction) en el camino, y el plan guardado sobrevive donde el historial de conversación puede no hacerlo

<h2 id="next-steps">
  Próximos pasos
</h2>

Una vez que esta configuración esté en su lugar, puede refinarla:

* Use [hooks](/docs/es/hooks-guide) para ejecutar linters o verificadores de tipo por directorio después de que Claude edite archivos
* Revise [Administrar costos de manera efectiva](/docs/es/costs) para entender cómo el tamaño del codebase afecta el uso de tokens y cómo establecer límites de gasto antes de un despliegue más amplio
* Lea [Cómo funciona Claude Code en codebases grandes](https://claude.com/blog/how-claude-code-works-in-large-codebases-best-practices-and-where-to-start) en el blog de Claude para patrones de despliegue organizacional y modelos de propiedad que se encuentran por encima de la configuración por repositorio en esta página
