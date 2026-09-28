> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referencia de Marketplace

> Referencia completa de los campos marketplace.json, entradas de plugins y los objetos de origen de plugin y marketplace, con dónde es válido cada uno.

`marketplace.json` es el archivo que define un marketplace de plugins. Contiene el nombre del marketplace, su propietario y una entrada por cada plugin. La fuente de plugin de cada entrada indica dónde Claude Code obtiene ese plugin.

Una fuente de marketplace es un objeto separado que indica dónde Claude Code obtiene el archivo marketplace en sí. Usted escribe uno en la configuración, o Claude Code construye uno cuando ejecuta `claude plugin marketplace add`.

Esta referencia es para los mantenedores de marketplace que necesitan un nombre de campo o valor exacto, y para los administradores que necesitan saber qué valores de `source` son válidos en [`extraKnownMarketplaces`](/docs/es/settings-reference#extraknownmarketplaces), [`strictKnownMarketplaces`](/docs/es/settings-reference#strictknownmarketplaces) y [`blockedMarketplaces`](/docs/es/plugins/org#restrict-what-users-can-install).

<Note>
  Estos casos se cubren en otras páginas:

  * **Construir u alojar un marketplace**: consulte [Crear un marketplace](/docs/es/plugins/create-marketplace) y [Alojar y mantener un marketplace](/docs/es/plugins/host-marketplace)
  * **Recetas de lista de permitidos y lista de bloqueados**: consulte [Administrar plugins para su organización](/docs/es/plugins/org)
</Note>

Encuentre la sección para lo que está escribiendo o leyendo:

* **El archivo marketplace**: [Campos de nivel superior](#top-level-fields) y [Entradas de plugins](#plugin-entries)
* **El `source` de una entrada**: [Fuentes de plugins](#plugin-sources)
* **Un objeto `source` en la configuración**: [Fuentes de marketplace](#marketplace-sources)
* **Salida de [`claude plugin validate <path>`](/docs/es/plugins/cli-reference)**: [Mensajes de validación](#validation-messages), que asigna cada mensaje al campo que nombra

<h2 id="marketplace-file">
  Archivo marketplace
</h2>

Guarde el archivo marketplace en `.claude-plugin/marketplace.json` en el directorio de su marketplace. Si mantiene el archivo en otro lugar del repositorio, los usuarios tienen que declarar el marketplace en [`extraKnownMarketplaces`](/docs/es/settings-reference#extraknownmarketplaces) con `path` establecido en su fuente, porque `claude plugin marketplace add` no tiene opción para ello.

El directorio que contiene `.claude-plugin/` se llama raíz del marketplace, y cada fuente de plugin relativa se resuelve desde él, no desde `.claude-plugin/`.

Cada usuario registra un marketplace por `name`, por lo que un usuario no puede tener dos marketplaces con el mismo nombre registrados a la vez.

Claude Code ignora una clave de nivel superior desconocida o una clave de entrada de plugin en lugar de rechazarla, por lo que un error tipográfico se carga silenciosamente. `claude plugin validate` reporta cada clave desconocida como una advertencia.

<h3 id="reserved-names">
  Nombres reservados
</h3>

No puede dar a su marketplace ninguno de los siguientes nombres:

* **Nombres de marketplace oficial**: `claude-code-marketplace`, `claude-code-plugins`, `claude-plugins-official`, `anthropic-marketplace`, `anthropic-plugins`, `agent-skills`, `anthropic-agent-skills`, `life-sciences`, `knowledge-work-plugins`, `claude-for-legal`, `claude-for-financial-services`, `financial-services-plugins`, `first-party-plugins` y `claude-tag-plugins`. Reservado a menos que el marketplace provenga de una [fuente de marketplace](#marketplace-sources) `github` o `git` bajo `github.com/anthropics/`.
* **Nombres de marketplace comunitario**: `claude-community`, `claude-plugins-community` y `healthcare`. Reservado bajo la misma regla que los nombres oficiales.
* **Nombres de directorio de plugins**: `anthropic-plugin-directory` y `claude-plugin-directory`. Reservado bajo la misma regla que los nombres oficiales.
* **Nombres que suplanten un marketplace oficial**: nombres como `official-claude-plugins` o `claude-plugins-v2`, y cualquier nombre que contenga un carácter no ASCII. El error es `Marketplace name impersonates an official Anthropic/Claude marketplace`. Un carácter de control o de formato bidireccional en un nombre también reporta `Marketplace name cannot contain control or bidirectional-formatting characters`.
* <span id="reserved-name-spellings" />**Otra ortografía de un nombre reservado**: un nombre que difiere de un nombre reservado solo por un punto final, o por un símbolo distinto de un guión en lugar de un guión, por lo que `claude.code.plugins` cuenta como `claude-code-plugins`. `claude plugin validate` acepta tal nombre; agregar el marketplace falla con [`is another spelling of "<reserved>", a reserved marketplace name`](/docs/es/errors#marketplace-name-is-another-spelling-of-a-reserved-name), y un marketplace ya registrado bajo uno deja de cargarse. Esta verificación requiere Claude Code v2.1.280 o posterior.
* **Nombres que Claude Code usa para plugins que no provienen de un marketplace**: `inline` para plugins cargados con [`--plugin-dir`](/docs/es/cli-reference), `builtin` para plugins integrados, `skills-dir` para plugins cargados automáticamente desde [`.claude/skills/`](/docs/es/skills) y `synced` para plugins sincronizados desde su cuenta claude.ai. `claude-plugin-test` también está reservado. `skills-dir` también aparece como `{"source": "skills-dir"}` en `strictKnownMarketplaces` y `blockedMarketplaces`, descrito bajo [Valores de fuente válidos solo en listas de políticas](#source-values-valid-only-in-policy-lists).
* **`npm`, `pip`, `uv`, `cargo`, `github` y `gh`**: reservado en cualquier mayúscula. Esta verificación requiere Claude Code v2.1.275 o posterior.
* **Nombres que comienzan con `claudeai-`**: reservado para marketplaces alojados en claude.ai. `claude plugin marketplace add` rechaza cualquier otro marketplace que use uno con `Cannot add marketplace "<name>": names starting with "claudeai-" are reserved for marketplaces hosted on claude.ai`.

<h2 id="top-level-fields">
  Campos de nivel superior
</h2>

La tabla enumera cada clave que Claude Code lee de `marketplace.json`. `name`, `owner` y `plugins` son obligatorios.

| Campo                                      | Tipo             | Descripción                                                                                                                                                                                                                                                                                           |
| :----------------------------------------- | :--------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                                     | string           | Identificador de marketplace. Sin espacios, caracteres de control o caracteres de formato bidireccional, sin `/` o `\`, sin `..` y no `.`. Consulte [Nombres reservados](#reserved-names). Los usuarios lo escriben después de `@` cuando instalan un plugin                                          |
| `owner`                                    | object           | Información del mantenedor. `name` es obligatorio; `email` y `url` son opcionales                                                                                                                                                                                                                     |
| `plugins`                                  | array            | [Entradas de plugins](#plugin-entries). Cada entrada se valida por sí sola, por lo que una entrada inválida no falla el marketplace                                                                                                                                                                   |
| `$schema`                                  | string           | URL de JSON Schema para autocompletado del editor. Se ignora en tiempo de carga                                                                                                                                                                                                                       |
| `description`                              | string           | Descripción del marketplace mostrada a los usuarios. `claude plugin validate` advierte cuando falta                                                                                                                                                                                                   |
| `version`                                  | string           | Versión del manifiesto del marketplace                                                                                                                                                                                                                                                                |
| `metadata.description`, `metadata.version` | string           | Ubicación alternativa para `description` y `version`                                                                                                                                                                                                                                                  |
| `metadata.pluginRoot`                      | string           | Directorio bajo el cual se resuelven los nombres de fuente de plugin desnudos. Consulte [Fuente de plugin de ruta relativa](#relative-path-plugin-source). Requiere Claude Code v2.1.239 o posterior                                                                                                  |
| `forceRemoveDeletedPlugins`                | boolean          | Cuando es `true`, un plugin que elimina de `plugins` se desinstala en las máquinas de los usuarios. Consulte [Alojar y mantener un marketplace](/docs/es/plugins/host-marketplace)                                                                                                                         |
| `allowCrossMarketplaceDependenciesOn`      | array of strings | Nombres de marketplace cuyos plugins pueden instalarse como dependencias de los plugins de este marketplace. Cuando instala un plugin, solo se aplica la lista en el propio marketplace del plugin, para toda su cadena de dependencias. Consulte [Dependencias de plugins](/docs/es/plugins/dependencies) |
| `renames`                                  | object           | Mapa de un `name` de plugin anterior a su nombre actual, o a `null` para un plugin que eliminó. Requiere Claude Code v2.1.193 o posterior. Consulte [Alojar y mantener un marketplace](/docs/es/plugins/host-marketplace)                                                                                  |

<h2 id="plugin-entries">
  Entradas de plugins
</h2>

Cada objeto en el array `plugins` de nivel superior de `marketplace.json` nombra un plugin e indica dónde obtenerlo. `name` y `source` son obligatorios.

Una entrada también acepta cada campo de [`plugin.json`](/docs/es/plugins/manifest-reference), como `description`, `version`, `author`, `commands` y `hooks`. Para cuándo se aplican esos campos, consulte [Cómo una entrada se combina con plugin.json](#entry-and-plugin-json).

La tabla enumera los campos propios de la entrada y los campos del manifiesto cuyo significado cambia en una entrada.

| Campo            | Tipo             | Descripción                                                                                                                                                                                                                                                                                                                                               |
| :--------------- | :--------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`           | string           | Identificador de plugin, sin espacios, caracteres de control o caracteres de formato bidireccional. Los usuarios lo escriben antes de `@` cuando instalan, incluso cuando el propio `plugin.json` del plugin establece un `name` diferente                                                                                                                |
| `source`         | string or object | Dónde obtener el plugin. Consulte [Fuentes de plugins](#plugin-sources)                                                                                                                                                                                                                                                                                   |
| `description`    | string           | Se muestra en los listados y detalles de [`/plugin`](/docs/es/plugins/install)                                                                                                                                                                                                                                                                                 |
| `version`        | string           | Cadena de versión para el plugin. Cuando `plugin.json` también establece `version`, `plugin.json` tiene precedencia y `claude plugin validate` advierte. Consulte [Referencia de carga de plugins](/docs/es/plugins/loading)                                                                                                                                   |
| `category`       | string           | Categoría de forma libre para organizar el catálogo                                                                                                                                                                                                                                                                                                       |
| `tags`           | array of strings | Etiquetas de forma libre para búsqueda                                                                                                                                                                                                                                                                                                                    |
| `strict`         | boolean          | Por defecto `true`. Si `plugin.json` es la fuente definitiva para los componentes del plugin. Consulte [Modo estricto](#strict-mode)                                                                                                                                                                                                                      |
| `relevance`      | object           | Señales que indican a Claude Code cuándo sugerir el plugin. Consulte [Recomendar plugins para su organización](/docs/es/plugins/relevance)                                                                                                                                                                                                                     |
| `dependencies`   | array            | Plugins que deben estar habilitados para que este funcione. Cada elemento es `"name"`, `"name@marketplace"` u un objeto. Consulte [Dependencias de plugins](/docs/es/plugins/dependencies)                                                                                                                                                                     |
| `defaultEnabled` | boolean          | Por defecto `true`. Si el plugin comienza habilitado cuando el usuario no lo ha establecido en [`enabledPlugins`](/docs/es/settings-reference#enabledplugins). El valor de la entrada tiene precedencia sobre `plugin.json`                                                                                                                                    |
| `displayName`    | string           | Nombre legible por humanos mostrado en la interfaz de usuario. Cuando ni la entrada ni el `plugin.json` del plugin establece uno, los usuarios ven el `name` del plugin                                                                                                                                                                                   |
| `metadata`       | object           | Objeto de forma libre para sus propios campos. Claude Code no lo lee. Requiere Claude Code v2.1.222 o posterior                                                                                                                                                                                                                                           |
| `headers`        | object           | Encabezados HTTP que Claude Code envía cuando descarga el [archivo](#archive-plugin-source) de esta entrada. Un encabezado establecido aquí reemplaza un encabezado del mismo nombre de los [`headers`](#fields-by-type) de la fuente del marketplace. Requiere Claude Code v2.1.238 o posterior                                                          |
| `headersHelper`  | string           | Comando que imprime los encabezados de descarga de archivo de esta entrada como un objeto JSON, para una credencial que expira. La entrada también debe establecer [`"strict": false`](#strict-mode). Requiere Claude Code v2.1.238 o posterior. Consulte [Autenticar descargas de archivos](/docs/es/plugins/host-marketplace#authenticate-archive-downloads) |

<h3 id="entry-and-plugin-json">
  Cómo una entrada se combina con plugin.json
</h3>

Los campos de la entrada se aplican de manera diferente a un plugin obtenido que tiene su propio `.claude-plugin/plugin.json` y a uno que no:

* **Sin `plugin.json`**: la entrada es el manifiesto independientemente de `strict`. Cada campo de manifiesto en la entrada se aplica, incluidos [`mcpServers`, `lspServers`, `userConfig` y `channels`](/docs/es/plugins/manifest-reference).
* **`plugin.json` presente**: `plugin.json` es el manifiesto. [Modo estricto](#strict-mode) decide si los seis campos de componente de la entrada, `commands`, `agents`, `skills`, `hooks`, `outputStyles` y `themes`, se combinan con él o se rechazan como un conflicto. La entrada `mcpServers`, `lspServers`, `userConfig` y `channels` no se aplican. Declárelos en `plugin.json`.

<h4 id="hooks-in-an-entry">
  Hooks en una entrada
</h4>

Escriba `hooks` de entrada como un objeto en línea que asigne nombres de eventos de hook a arrays de coincidencia. Si escribe una ruta de archivo o un array en su lugar, `claude plugin validate` lo aprueba. Esos hooks nunca se ejecutan, y Claude Code reporta un error `not yet supported in a marketplace entry` para el plugin. Coloque hooks basados en archivos en el propio [`hooks/hooks.json`](/docs/es/plugins/components) del plugin o `plugin.json`.

<h4 id="display-fields">
  Campos de visualización
</h4>

Tanto la entrada como el propio `plugin.json` del plugin pueden establecer los campos de visualización `displayName`, `description`, `author`, `homepage`, `repository`, `license` y `keywords`. Los usuarios ven estos valores en los listados y detalles de plugins, antes y después de instalar:

* Para un campo que establece en la entrada, los usuarios ven el valor de la entrada, incluso cuando `plugin.json` establece uno diferente.
* Para un campo que la entrada deja sin establecer, los usuarios ven el valor de `plugin.json`.

Antes de instalar, Claude Code solo puede leer `plugin.json` para entradas con una [fuente de ruta relativa](#relative-path-plugin-source), cuyos archivos de plugin están dentro del marketplace en sí. Para una entrada con cualquier otro tipo de fuente, los usuarios ven solo los campos propios de la entrada hasta que instalen el plugin.

<h3 id="strict-mode">
  Modo estricto
</h3>

`strict` decide qué sucede cuando el plugin obtenido tiene su propio `plugin.json` y la entrada también declara cualquiera de los [campos de componente](#entry-and-plugin-json): `commands`, `agents`, `skills`, `hooks`, `outputStyles` o `themes`. Con `strict: true`, el predeterminado, Claude Code añade los campos de componente de la entrada a `plugin.json`, excepto `hooks`, cuyos coincidentes reemplazan los del manifiesto por evento. Con `strict: false`, una entrada que declara cualquier campo de componente es un conflicto, y el plugin falla al cargar. La tabla muestra cada combinación de `strict`, `plugin.json` y los campos de componente de la entrada.

| `strict`                  | `plugin.json` | Campos de componente de entrada | Resultado                                                                                                                                                                                                                                              |
| :------------------------ | :------------ | :------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| any                       | absent        | any                             | La entrada es el manifiesto                                                                                                                                                                                                                            |
| `true`, el predeterminado | present       | any                             | `plugin.json` es la autoridad. Claude Code añade los campos de componente de la entrada a él, excepto `hooks`, cuyos coincidentes [reemplazan los del manifiesto por evento](/docs/es/plugins/manifest-reference#how-entry-fields-combine-with-plugin-json) |
| `false`                   | present       | none                            | `plugin.json` es el manifiesto, como con `true`                                                                                                                                                                                                        |
| `false`                   | present       | one or more                     | Conflicto. El plugin falla al cargar con `Plugin <name> has conflicting manifests: both plugin.json and marketplace entry specify components`                                                                                                          |

<h2 id="plugin-sources">
  Fuentes de plugins
</h2>

El `source` de una entrada de plugin indica dónde Claude Code obtiene ese plugin. Es una cadena de ruta relativa u un objeto cuya propia clave `source` nombra el tipo, por lo que una entrada se ve como `"source": { "source": "github", "repo": "your-org/formatter" }`.

La tabla enumera cada tipo de fuente de plugin y sus campos.

| Tipo          | Campos                           | Notas                                                                                                                                                                                                                                             |
| :------------ | :------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Ruta relativa | la cadena en sí                  | Un directorio dentro del marketplace, resuelto desde la raíz del marketplace. Debe comenzar con `./`, a menos que escriba un [nombre desnudo bajo `metadata.pluginRoot`](#relative-path-plugin-source). `"."` por sí solo significa la raíz en sí |
| `github`      | `repo`, `ref`, `sha`             | Repositorio de GitHub en forma `owner/repo`                                                                                                                                                                                                       |
| `url`         | `url`, `ref`, `sha`              | Cualquier repositorio git por URL                                                                                                                                                                                                                 |
| `git-subdir`  | `url`, `path`, `ref`, `sha`      | Un subdirectorio de un repositorio git, obtenido con un clon parcial disperso                                                                                                                                                                     |
| `npm`         | `package`, `version`, `registry` | Paquete npm, obtenido con su cliente npm y desempaquetado sin ejecutar scripts de instalación                                                                                                                                                     |
| `archive`     | `url`, `sha256`                  | Archivo Zip sobre HTTPS. Requiere Claude Code v2.1.224 o posterior                                                                                                                                                                                |
| `command`     | `command`, `timeout`, `mode`     | Directorio impreso por un comando que Claude Code ejecuta en la máquina del usuario. Requiere Claude Code v2.1.229 o posterior                                                                                                                    |

Los nombres `url` y `github` también son tipos de [fuente de marketplace](#marketplace-sources), donde `url` significa un enlace directo a un archivo `marketplace.json` en lugar de un repositorio git. `git` existe solo como una fuente de marketplace, y `npm` existe como ambas. `git-subdir`, `archive` y `command` existen solo como fuentes de plugins.

Use una ruta relativa para un plugin en un subdirectorio del repositorio del marketplace en sí. Use `git-subdir` para un subdirectorio de algún otro repositorio.

Las fuentes `github`, `url` y `git-subdir` comparten los campos `ref` y `sha`:

* **`ref`**: una rama o etiqueta. Por defecto es la rama predeterminada del repositorio.
* **`sha`**: un SHA de commit completo de 40 caracteres en minúsculas. Cuando establece tanto `ref` como `sha`, Claude Code verifica `sha`. En la mayoría de hosts git, incluidos GitHub, GitLab y Bitbucket, esto significa que la instalación tiene éxito incluso si la rama o etiqueta nombrada por `ref` ha sido eliminada posteriormente, siempre que el commit aún sea alcanzable desde el repositorio. Algunos servidores, como AWS CodeCommit, no admiten obtener commits por SHA. En esos servidores, el `ref` aún debe existir y el commit fijado debe ser alcanzable desde él.

Para cómo se obtiene, almacena en caché y versiona cada tipo, consulte [Referencia de carga de plugins](/docs/es/plugins/loading).

<h3 id="relative-path-plugin-source">
  Fuente de plugin de ruta relativa
</h3>

La ruta se resuelve desde la raíz del marketplace. `./plugins/formatter` es `<root>/plugins/formatter` aunque el archivo marketplace esté en `<root>/.claude-plugin/`.

Una ruta que contiene `..` falla la validación. En macOS y Linux, Claude Code rechaza una ruta de entrada que contiene una barra invertida en cualquier lugar después del `./` inicial, por lo que escriba la ruta con barras diagonales.

```json theme={null}
{ "name": "formatter", "source": "./plugins/formatter" }
```

Una ruta relativa se resuelve solo cuando Claude Code tiene los archivos del marketplace, así que verifique el tipo de [fuente de marketplace](#marketplace-sources):

* **`github`, `git`, `file` y `directory`**: Claude Code tiene los archivos del marketplace.
* **`url`**: Claude Code obtiene solo `marketplace.json`, por lo que las rutas relativas no pueden resolverse. Dé a cada plugin una fuente de objeto en su lugar, como `github` o `git-subdir`.
* **`settings`**: las rutas relativas se rechazan directamente.

<h4 id="bare-names-under-pluginroot">
  Nombres desnudos bajo pluginRoot
</h4>

Un nombre desnudo es un nombre de directorio único sin `/`, como `"formatter"`. Para escribir nombres desnudos en lugar de rutas `./`, establezca [`metadata.pluginRoot`](#top-level-fields) en el directorio bajo el cual se resuelven. Con `"pluginRoot": "./plugins"`, `"source": "formatter"` se resuelve a `./plugins/formatter`. Requiere Claude Code v2.1.239 o posterior.

`metadata.pluginRoot` tiene estos límites:

* Debe ser en sí una ruta relativa dentro del marketplace.
* No tiene efecto en una fuente que ya comienza con `./`.
* Una fuente que contiene un `/`, como `team-a/formatter`, no es un nombre desnudo y aún necesita el prefijo `./`, incluso cuando `metadata.pluginRoot` está establecido.

<h3 id="github-plugin-source">
  Fuente de plugin github
</h3>

`repo` toma `owner/repo`. `ref` y `sha` son opcionales.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "github",
    "repo": "your-org/formatter",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

<h3 id="url-plugin-source">
  Fuente de plugin url
</h3>

`url` es una URL git completa: `https://`, `http://`, `file://` o `git@`. Un sufijo `.git` no es obligatorio, por lo que las URL de Azure DevOps y AWS CodeCommit funcionan tal como están escritas. Este tipo no toma el atajo `owner/repo`.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "url",
    "url": "https://gitlab.example.com/your-group/formatter.git",
    "ref": "main"
  }
}
```

<h3 id="git-subdir-plugin-source">
  Fuente de plugin git-subdir
</h3>

`url` acepta una URL git completa o el atajo `owner/repo` de GitHub. `path` es el subdirectorio que contiene el plugin, y Claude Code descarga solo ese subdirectorio.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/your-org/monorepo.git",
    "path": "tools/formatter"
  }
}
```

<h3 id="npm-plugin-source">
  Fuente de plugin npm
</h3>

Una fuente `npm` toma estos campos:

* `package`: un nombre de paquete, o un nombre con alcance como `@your-org/formatter`
* `version`: una versión o rango
* `registry`: una URL de registro para un paquete que no está en el registro predeterminado

Claude Code obtiene el paquete con su cliente npm. Los scripts de instalación del paquete, como `preinstall` o `postinstall`, nunca se ejecutan, y sus dependencias no se instalan durante la obtención. Si el paquete tiene un archivo de bloqueo compatible junto a su `package.json`, Claude Code instala esas [dependencias de paquetes Node.js](/docs/es/plugins/loading#node-js-package-dependencies) en un paso separado, también con scripts deshabilitados.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "npm",
    "package": "@your-org/formatter",
    "version": "^2.0.0",
    "registry": "https://npm.example.com"
  }
}
```

<h3 id="archive-plugin-source">
  Fuente de plugin archive
</h3>

`url` debe usar `https://` y no puede apuntar a un host de loopback, link-local o cloud-metadata.

La raíz del plugin puede estar en la parte superior del zip o un directorio hacia abajo.

`sha256` es el resumen del archivo como 64 caracteres hexadecimales, mayúsculas o minúsculas. Cuando lo establece, Claude Code rechaza una descarga que no coincida.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "archive",
    "url": "https://artifacts.example.com/formatter-2.0.0.zip",
    "sha256": "6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1"
  }
}
```

<h3 id="command-plugin-source">
  Fuente de plugin command
</h3>

Use una fuente `command` cuando una herramienta instalada en la máquina del usuario produce el directorio del plugin, como un IDE que renderiza su plugin para la cadena de herramientas que el usuario ha seleccionado. Claude Code ejecuta el comando cuando el usuario instala o actualiza el plugin, y [nuevamente una vez por sesión](/docs/es/plugins/loading#when-a-command-source-re-runs), por lo que los usuarios obtienen la salida cambiada de la herramienta sin reinstalar.

Una fuente `command` toma estos campos:

* `command`: un comando de shell que imprime la ruta absoluta del directorio del plugin como una línea y sale con 0. Claude Code muestra a los usuarios la cadena completa para revisión antes de ejecutarla. Escríbala como ASCII imprimible, como máximo 500 caracteres, sin una ejecución de cuatro o más espacios.
* `timeout`: un número entero de segundos de 1 a 600. Por defecto es 60.
* `mode`: `copy`, el predeterminado, o `link`. Consulte [Modo de copia y modo de enlace](#copy-mode-and-link-mode).

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "command",
    "command": "my-tool claude-plugin-path",
    "timeout": 120
  }
}
```

Para cómo los usuarios aceptan el comando, consulte [Instalar desde su shell](/docs/es/plugins/install#install-from-your-shell). Para lo que los usuarios ven después de que lo cambia, consulte [Cambiar el comando de una fuente de comando](/docs/es/plugins/host-marketplace#change-the-command-of-a-command-source). Los administradores desactivan las fuentes de comando con [`disableCommandPluginSources`](/docs/es/settings-reference#disablecommandpluginsources).

<h4 id="what-the-command-must-do">
  Qué debe hacer el comando
</h4>

Escriba el comando para cumplir con estos requisitos:

* **Shell y directorio de trabajo**: Claude Code ejecuta el comando a través de `sh`, o a través de `cmd.exe` en Windows, desde el directorio de inicio del usuario. Dé una ruta absoluta o un comando en `PATH`.
* **Salida**: imprima exactamente una línea en stdout, la ruta absoluta del directorio del plugin, y salga con 0 dentro de `timeout` segundos.
* **Contenido del directorio**: el directorio contiene el plugin completo en el momento en que el comando sale. La ruta puede diferir de una ejecución a la siguiente.

<h4 id="output-that-fails-the-install-or-update">
  Salida que falla la instalación o actualización
</h4>

La instalación o actualización falla cuando el comando sale con un código distinto de cero, se ejecuta más tiempo que `timeout`, o imprime algo que no sea una ruta absoluta. También falla cuando el directorio impreso es uno de estos:

* **Sin contenido de plugin**: el directorio impreso no tiene contenido de plugin en su nivel superior, como un directorio `.claude-plugin/` o un directorio `skills/`, `commands/`, `agents/` o `hooks/`.
* **El directorio de la propia sesión**: el directorio impreso es el en el que se inició Claude Code, o uno de sus padres.
* **Una ruta de red**: en Windows, la ruta impresa es una ruta UNC.
* **Demasiado grande para copiar**: en modo de copia, el directorio es más grande que 256 MiB o tiene más de 20,000 entradas.

<h4 id="copy-mode-and-link-mode">
  Modo de copia y modo de enlace
</h4>

`mode` decide si Claude Code copia el directorio impreso o lo usa en su lugar:

* **`copy`**: Claude Code copia el directorio en el caché de plugins y deriva la [versión del plugin](/docs/es/plugins/loading#how-claude-code-computes-the-version) de un hash de los archivos copiados. Su herramienta puede eliminar o reescribir el directorio después de que el comando salga. Una re-ejecución que produce archivos idénticos cuenta como actualizado.
* **`link`**: Claude Code llena la entrada de caché del plugin con un enlace a cada entrada de nivel superior del directorio impreso y carga los archivos en su lugar. Nada se copia, los contenidos de los archivos no se hashean, y los límites de tamaño no se aplican. Úselo para un directorio demasiado grande para copiar, como una exportación de SDK renderizada.

Un plugin en modo de enlace tiene estos requisitos:

* **Mantenga el directorio en su lugar**: Claude Code carga el plugin a través de los enlaces en cada inicio, por lo que el directorio impreso debe permanecer donde está mientras el plugin permanezca instalado.
* **Imprima una ruta diferente para señalar contenido nuevo**: la versión proviene de la ruta real del directorio impreso y sus entradas de nivel superior, no de los archivos dentro de ellas.
* **Mantenga los enlaces simbólicos de nivel superior dentro del directorio**: la instalación falla si una entrada de nivel superior es un enlace simbólico que apunta fuera del directorio impreso.
* **Incluya `node_modules`**: Claude Code omite la [instalación de dependencias de paquetes Node.js](/docs/es/plugins/loading#node-js-package-dependencies) para un plugin en modo de enlace, por lo que imprima un directorio que ya contenga los paquetes que el plugin necesita.
* **Sesiones iniciadas dentro del directorio**: una sesión iniciada en el directorio impreso o en cualquier lugar debajo de él no carga el plugin.
* **No en Windows**: Claude Code rechaza instalar un plugin en modo de enlace en Windows. Declare `"mode": "copy"` allí.

<h2 id="marketplace-sources">
  Fuentes de marketplace
</h2>

Una fuente de marketplace indica dónde Claude Code obtiene un `marketplace.json`. La CLI construye uno para usted cuando agrega un marketplace, y usted escribe uno en la configuración:

* **[`claude plugin marketplace add`](/docs/es/plugins/cli-reference)**: Claude Code construye la fuente a partir de la cadena que pasa.
* **[`extraKnownMarketplaces`](/docs/es/settings-reference#extraknownmarketplaces)**: usted escribe la fuente en sí como el objeto `source`.
* **[`strictKnownMarketplaces`](/docs/es/settings-reference#strictknownmarketplaces) y [`blockedMarketplaces`](/docs/es/plugins/org#restrict-what-users-can-install)**: los administradores escriben fuentes en estas dos listas de políticas. `strictKnownMarketplaces` es la lista de permitidos y `blockedMarketplaces` es la lista de bloqueados.

Los nombres de tipo `url`, `git` y `github` significan algo diferente en una fuente de marketplace que en una [fuente de plugin](#plugin-sources):

| Nombre de tipo | Como fuente de marketplace                                                                       | Como fuente de plugin                                                    |
| :------------- | :----------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------- |
| `url`          | Un enlace directo a un archivo `marketplace.json`, con campos `url`, `headers` y `headersHelper` | Un repositorio git para clonar, con campos `url`, `ref` y `sha`          |
| `git`          | Un repositorio git para clonar, con campos `url`, `ref`, `path` y `sparsePaths`                  | No existe                                                                |
| `github`       | Un repositorio de GitHub, con campos `repo`, `ref`, `path` y `sparsePaths`                       | Un repositorio de GitHub, con campos `repo`, `ref` y `sha`, y sin `path` |

La tabla enumera cada tipo de fuente de marketplace con sus campos, la entrada de `claude plugin marketplace add` que lo produce, y qué hace en cada una de las tres claves de configuración.

| Tipo          | Campos                               | entrada `marketplace add`                                                                                                                                    | `extraKnownMarketplaces`                                       | `strictKnownMarketplaces`                                                                                                                                                                                                                                                    | `blockedMarketplaces`                                                  |
| :------------ | :----------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------- |
| `url`         | `url`, `headers`, `headersHelper`    | Una URL `http://` o `https://` que no coincide con una forma git                                                                                             | Se carga                                                       | Permite la misma URL                                                                                                                                                                                                                                                         | Bloquea la misma URL                                                   |
| `github`      | `repo`, `ref`, `path`, `sparsePaths` | `owner/repo`, `owner/repo@ref` u `owner/repo#ref`                                                                                                            | Se carga                                                       | Permite el mismo `repo`, `ref` y `path`. `repo` puede ser `owner/*`                                                                                                                                                                                                          | Bloquea lo mismo, y una URL `git` al mismo repositorio                 |
| `git`         | `url`, `ref`, `path`, `sparsePaths`  | Una URL `user@host:path`, o una URL `https://` que termina en `.git`, contiene `/_git/`, o nombra un repositorio github.com o gitlab.com. `#ref` fija un ref | Se carga                                                       | Permite la misma URL, `ref` y `path`                                                                                                                                                                                                                                         | Bloquea lo mismo, y otras ortografías del mismo repositorio github.com |
| `npm`         | `package`                            | No producido                                                                                                                                                 | Falla al cargar: `NPM marketplace sources not yet implemented` | Se analiza pero no coincide con nada, porque nada registra un marketplace `npm`                                                                                                                                                                                              | Se analiza pero no coincide con nada                                   |
| `file`        | `path`                               | Una ruta a un archivo `.json`                                                                                                                                | Se carga                                                       | Permite la misma ruta                                                                                                                                                                                                                                                        | Bloquea la misma ruta                                                  |
| `directory`   | `path`                               | Una ruta a un directorio                                                                                                                                     | Se carga                                                       | Permite la misma ruta                                                                                                                                                                                                                                                        | Bloquea la misma ruta                                                  |
| `settings`    | `name`, `plugins`, `owner`           | No producido                                                                                                                                                 | Se carga                                                       | Permite una entrada con el mismo `name` y `plugins` idénticos                                                                                                                                                                                                                | Bloquea el mismo `name`                                                |
| `skills-dir`  | none                                 | No producido                                                                                                                                                 | Falla al cargar: `Unsupported marketplace source type`         | Mantiene los [plugins del directorio de skills](/docs/es/plugins/org#keep-skills-directory-plugins-loading) cargándose mientras se establece una lista de permitidos. Consulte [Valores de fuente válidos solo en listas de políticas](#source-values-valid-only-in-policy-lists) | Detiene los plugins del directorio de skills de cargarse               |
| `hostPattern` | `hostPattern`                        | No producido                                                                                                                                                 | Falla al cargar: `Unsupported marketplace source type`         | Permite fuentes `github`, `git` y `url` cuyo host coincida                                                                                                                                                                                                                   | Bloquea esas fuentes                                                   |
| `pathPattern` | `pathPattern`                        | No producido                                                                                                                                                 | Falla al cargar: `Unsupported marketplace source type`         | Permite fuentes `file` y `directory` cuya `path` coincida                                                                                                                                                                                                                    | Bloquea esas fuentes                                                   |

<h3 id="fields-by-type">
  Campos por tipo
</h3>

La tabla enumera cada campo de fuente de marketplace que tiene un predeterminado, una restricción o un significado específico de su tipo.

| Campo           | Tipos           | Descripción                                                                                                                                                                                                                                                               |
| :-------------- | :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `url`           | `url`           | Enlace al archivo `marketplace.json`. Claude Code descarga solo ese archivo, por lo que los plugins del marketplace no pueden usar [fuentes de ruta relativa](#relative-path-plugin-source)                                                                               |
| `url`           | `git`           | El repositorio git para clonar                                                                                                                                                                                                                                            |
| `headers`       | `url`           | Mapa de encabezados HTTP que Claude Code envía con la obtención, para hosts autenticados                                                                                                                                                                                  |
| `headersHelper` | `url`           | Comando que imprime encabezados cuyos valores son demasiado efímeros para listar en `headers`. Requiere Claude Code v2.1.238 o posterior. Consulte [Autenticar descargas de archivos](/docs/es/plugins/host-marketplace#authenticate-archive-downloads)                        |
| `repo`          | `github`        | En `marketplace add` y `extraKnownMarketplaces`, `repo` debe nombrar un repositorio. `marketplace add` rechaza `owner/*` como no un atajo `owner/repo` válido; en `extraKnownMarketplaces` Claude Code lo toma literalmente y el clon falla                               |
| `ref`           | `github`, `git` | Rama o etiqueta. Por defecto es la rama predeterminada del repositorio                                                                                                                                                                                                    |
| `path`          | `github`, `git` | La ruta del archivo marketplace dentro del repositorio. Por defecto es `.claude-plugin/marketplace.json`                                                                                                                                                                  |
| `path`          | `file`          | El archivo marketplace en sí. Claude Code lo lee en su lugar y toma el directorio dos niveles arriba como la raíz del marketplace, así que mantenga el archivo en `<root>/.claude-plugin/marketplace.json`                                                                |
| `path`          | `directory`     | La raíz del marketplace, el directorio que contiene `.claude-plugin/marketplace.json`                                                                                                                                                                                     |
| `sparsePaths`   | `github`, `git` | Array de directorios para un checkout disperso, como `[".claude-plugin", "plugins"]`. `claude plugin marketplace add --sparse` lo establece                                                                                                                               |
| `skipLfs`       | `github`, `git` | Aceptado y no tiene efecto. Consulte [Mantener archivos de plugin fuera de Git LFS](/docs/es/plugins/host-marketplace#keep-plugin-files-out-of-git-lfs)                                                                                                                        |
| `name`          | `settings`      | Debe ser igual a la clave `extraKnownMarketplaces` y no puede ser un [nombre reservado](#reserved-names)                                                                                                                                                                  |
| `plugins`       | `settings`      | El catálogo en línea, sin archivo alojado. Cada elemento toma `name`, `source`, `description`, `version`, `strict`, `headers` y `headersHelper`. Escriba el `source` de cada elemento como un tipo de objeto, porque una ruta relativa no tiene repositorio para resolver |

<h3 id="source-values-valid-only-in-policy-lists">
  Valores de fuente válidos solo en listas de políticas
</h3>

`hostPattern`, `pathPattern`, `skills-dir` y la forma `owner/*` de `repo` son válidos solo en las dos listas de políticas, `strictKnownMarketplaces` y `blockedMarketplaces`:

* **`hostPattern` y `pathPattern`**: expresiones regulares que Claude Code prueba contra una fuente antes de obtener de ella.
* **`skills-dir`**: no es una fuente. Si establece `strictKnownMarketplaces` en absoluto, los [plugins del directorio de skills](/docs/es/plugins/org#keep-skills-directory-plugins-loading) dejan de cargarse hasta que agregue `{"source": "skills-dir"}` a esa lista.
* **`owner/*`**: como un valor `repo` de `github`, coincide con cada repositorio bajo exactamente ese propietario de GitHub. Requiere Claude Code v2.1.223 o posterior.

Para el orden de coincidencia, la semántica exacta de `ref` y recetas, consulte [Administrar plugins para su organización](/docs/es/plugins/org).

<h3 id="source-objects-in-settings">
  Objetos de fuente en la configuración
</h3>

Un valor `extraKnownMarketplaces` es un mapa de nombre de marketplace a un objeto con `source`. Esta entrada registra un marketplace desde un repositorio git en su rama `main`:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": {
        "source": "git",
        "url": "https://git.example.com/your-org/your-marketplace.git",
        "ref": "main"
      }
    }
  }
}
```

`strictKnownMarketplaces` y `blockedMarketplaces` son arrays de objetos de fuente. Esta lista de permitidos admite un propietario de GitHub y un host interno:

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "your-org/*" },
    { "source": "hostPattern", "hostPattern": "^git\\.example\\.com$" }
  ]
}
```

<h2 id="validation-messages">
  Mensajes de validación
</h2>

`claude plugin validate <path>` toma la raíz del marketplace o el archivo marketplace en sí. Imprime errores y advertencias. Para códigos de salida y `--strict`, consulte [plugin validate](/docs/es/plugins/cli-reference#plugin-validate).

Un mensaje nombra una entrada de plugin por su índice, escrito como `plugins.1.source` o `plugins[1].source`.

Un mensaje prefijado con un índice de entrada y `plugin.json →`, como `plugins[2] plugin.json →`, es sobre los propios archivos de ese plugin. [`claude plugin validate` reporta errores](/docs/es/plugins/troubleshooting#claude-plugin-validate-reports-errors) enumera esos mensajes con sus correcciones.

Las advertencias que mencionan nombres de banderas de Claude Desktop señalan nombres que Claude Code acepta pero Claude Desktop rechaza, porque las reglas de nombre de Claude Desktop son más estrictas.

La tabla asigna mensajes de nivel de marketplace al campo sobre el que trata cada uno.

| Mensaje                                                                                                                                                                                         | Nivel       | Campo                                                                                                                        |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `Marketplace must have a name`                                                                                                                                                                  | Error       | `name` está vacío                                                                                                            |
| `Marketplace name cannot contain spaces. Use kebab-case (e.g., "my-marketplace")`                                                                                                               | Error       | `name`                                                                                                                       |
| `Marketplace name cannot contain path separators (/ or \), ".." sequences, or be "."`                                                                                                           | Error       | `name`                                                                                                                       |
| `Marketplace name impersonates an official Anthropic/Claude marketplace`                                                                                                                        | Error       | `name`. Consulte [Nombres reservados](#reserved-names)                                                                       |
| `Marketplace name cannot contain control or bidirectional-formatting characters`                                                                                                                | Error       | `name` contiene un carácter de control, como un escape o una nueva línea, o un carácter de formato bidireccional Unicode     |
| `Marketplace name "inline" is reserved for --plugin-dir session plugins`, y las variantes `builtin`, `skills-dir`, `synced`, `claude-plugin-test`, `npm`, `pip`, `uv`, `cargo`, `github` y `gh` | Error       | `name`                                                                                                                       |
| `Author name cannot be empty`                                                                                                                                                                   | Error       | `owner.name`                                                                                                                 |
| `Plugin name cannot contain spaces. Use kebab-case (e.g., "my-plugin")`                                                                                                                         | Error       | `plugins[i].name`                                                                                                            |
| `Plugin name cannot contain control or bidirectional-formatting characters`                                                                                                                     | Error       | `plugins[i].name`                                                                                                            |
| `Duplicate plugin name "x" found in marketplace`                                                                                                                                                | Error       | Dos entradas comparten un `name`                                                                                             |
| `plugins.i.source: Invalid input`                                                                                                                                                               | Error       | El `source` de la entrada no coincide con ningún tipo. Consulte [Entrada inválida en una fuente](#invalid-input-on-a-source) |
| `plugins[i].source: Path contains "..": <path>`                                                                                                                                                 | Error       | Un `source` relativo que escapa de la raíz del marketplace                                                                   |
| `source.source: 'unsupported' is a parse-time placeholder and cannot be authored`                                                                                                               | Error       | `plugins[i].source`                                                                                                          |
| `Plugin "x" sets headersHelper but is not "strict": false`                                                                                                                                      | Error       | `plugins[i].headersHelper`, en una entrada `archive`                                                                         |
| `chain does not resolve (<reason>) — target must be a name in plugins[], a key in renames, or null`                                                                                             | Error       | `renames.<old>`                                                                                                              |
| `target "x" is not a valid plugin name (PluginIdSchema)`                                                                                                                                        | Error       | `renames.<old>`                                                                                                              |
| `Unknown field 'x'. Claude Code ignores it at load time.`                                                                                                                                       | Advertencia | La clave nombrada en el nivel superior, bajo `metadata`, en una entrada, o bajo el `relevance` de una entrada                |
| `Marketplace has no plugins defined`                                                                                                                                                            | Advertencia | `plugins` está vacío                                                                                                         |
| `Plugin "x" sets headers/headersHelper, which only apply to "archive" sources; they have no effect on this entry.`                                                                              | Advertencia | `plugins[i].headers` o `plugins[i].headersHelper`, en una entrada cuyo `source` no es `archive`                              |
| `Plugin "x" fetches its archive with a headersHelper but sets no sha256 pin`                                                                                                                    | Advertencia | `plugins[i].source.sha256`                                                                                                   |
| `Header "x" is a request-routing/identity header that catalog entries may not set; Claude Code drops it at download time.`                                                                      | Advertencia | `plugins[i].headers.<name>`                                                                                                  |
| `Local source "x" is or traverses a symlink, so <path> was not read`                                                                                                                            | Advertencia | `plugins[i].source`                                                                                                          |
| `No marketplace description provided. Adding a description helps users understand what this marketplace offers`                                                                                 | Advertencia | `description`                                                                                                                |
| `Entry declares version "x" but <path>/plugin.json says "y". At install time, plugin.json wins`                                                                                                 | Advertencia | `plugins[i].version`, en una entrada de ruta relativa                                                                        |
| `'relevance' must be an object containing topic and signals; got <type>. It will be ignored at load time.`                                                                                      | Advertencia | `plugins[i].relevance`                                                                                                       |
| `'metadata' must be a free-form object; got <type>. It will be ignored at load time.`                                                                                                           | Advertencia | `plugins[i].metadata`                                                                                                        |
| `'experimental' must be an object containing component declarations; got <type>. It will be ignored at load time.`                                                                              | Advertencia | `plugins[i].experimental`                                                                                                    |
| `Marketplace name "x" is reserved in Claude Desktop`                                                                                                                                            | Advertencia | `name` es `org`, `org-provisioned` u `unknown`. Claude Desktop rechaza el marketplace                                        |
| `Marketplace name "x" is not accepted by Claude Desktop (letters, digits, ".", "_", "-"; must start alphanumeric; max 128 chars)`                                                               | Advertencia | `name`. Claude Desktop rechaza el marketplace                                                                                |
| `Plugin name "x" is not accepted by Claude Desktop (letters, digits, ".", "_", "-"; must start alphanumeric; max 128 chars)`                                                                    | Advertencia | `plugins[i].name`. Claude Desktop descarta la entrada                                                                        |

<h3 id="invalid-input-on-a-source">
  Entrada inválida en una fuente
</h3>

`Invalid input` en un `source` significa que el objeto no coincidió con ningún tipo de fuente. Verifique estas causas:

* Una ruta relativa que no comienza con `./`, que no sea `"."` o un [nombre desnudo bajo `metadata.pluginRoot`](#relative-path-plugin-source)
* Un `package` npm que contiene `..`
* Un tipo de `source` que no es uno de las [fuentes de plugins](#plugin-sources)
* Un tipo conocido con un campo obligatorio faltante o de tipo incorrecto, como `github` sin `repo`

<h3 id="failures-that-validation-doesn’t-catch">
  Fallos que la validación no detecta
</h3>

`claude plugin validate` no reporta cada fallo. Un `hooks` de entrada escrito como una ruta de archivo o array aprueba la validación, y el error aparece solo cuando el plugin se carga, como describe [Hooks en una entrada](#hooks-in-an-entry). Los errores al obtener un `source` también aparecen solo después de instalar, no en la validación.

[`claude plugin list`](/docs/es/plugins/cli-reference) muestra un plugin que falló al cargar con su error, y [Solucionar problemas de plugins](/docs/es/plugins/troubleshooting) cubre las cadenas de tiempo de carga.

<h2 id="next-steps">
  Próximos pasos
</h2>

* [Crear un marketplace](/docs/es/plugins/create-marketplace): construya un marketplace a partir de estos campos e instale desde él localmente
* [Alojar y mantener un marketplace](/docs/es/plugins/host-marketplace): dónde poner el archivo y cómo los usuarios reciben cambios
* [Referencia de manifiesto de plugin](/docs/es/plugins/manifest-reference): los campos `plugin.json` que una entrada puede anular
* [Administrar plugins para su organización](/docs/es/plugins/org): recetas de lista de permitidos y lista de bloqueados que usan estos valores de fuente
