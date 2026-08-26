> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Crear y distribuir un marketplace de plugins

> Cree y aloje marketplaces de plugins para distribuir extensiones de Claude Code en equipos y comunidades.

Un **marketplace de plugins** es un catálogo que le permite distribuir plugins a otros. Los marketplaces proporcionan descubrimiento centralizado, seguimiento de versiones, actualizaciones automáticas y soporte para múltiples tipos de fuentes, incluyendo repositorios git y rutas locales. Esta guía le muestra cómo crear su propio marketplace para compartir plugins con su equipo o comunidad.

¿Busca instalar plugins desde un marketplace existente? Consulte [Descubrir e instalar plugins precompilados](/docs/es/discover-plugins).

<h2 id="overview">
  Descripción general
</h2>

Crear y distribuir un marketplace implica:

1. **Crear plugins**: construya uno o más plugins con skills, agentes, hooks, servidores MCP o servidores LSP. Esta guía asume que ya tiene plugins para distribuir; consulte [Crear plugins](/docs/es/plugins) para obtener detalles sobre cómo crearlos.
2. **Crear el archivo de marketplace**: defina un `marketplace.json` que enumere sus plugins y dónde encontrarlos. Consulte [Crear el archivo de marketplace](#create-the-marketplace-file).
3. **Alojar el marketplace**: envíe a GitHub, GitLab u otro host git. Consulte [Alojar y distribuir marketplaces](#host-and-distribute-marketplaces).
4. **Compartir con usuarios**: los usuarios agregan su marketplace con `/plugin marketplace add` e instalan plugins individuales. Consulte [Descubrir e instalar plugins](/docs/es/discover-plugins).

Una vez que su marketplace esté activo, puede actualizarlo enviando cambios a su repositorio. Los usuarios actualizan su copia local con `/plugin marketplace update`.

<h2 id="walkthrough-create-a-local-marketplace">
  Tutorial: crear un marketplace local
</h2>

Este ejemplo crea un marketplace con un plugin: una skill `quality-review` para revisiones de código. Creará la estructura de directorios, agregará una skill, creará el manifiesto del plugin y el catálogo del marketplace, luego lo instalará y probará.

<Steps>
  <Step title="Crear la estructura de directorios">
    ```bash theme={null}
    mkdir -p my-marketplace/.claude-plugin
    mkdir -p my-marketplace/plugins/quality-review-plugin/.claude-plugin
    mkdir -p my-marketplace/plugins/quality-review-plugin/skills/quality-review
    ```
  </Step>

  <Step title="Crear la skill">
    Cree un archivo `SKILL.md` que defina qué hace la skill `quality-review`.

    ```markdown my-marketplace/plugins/quality-review-plugin/skills/quality-review/SKILL.md theme={null}
    ---
    description: Review code for bugs, security, and performance
    ---

    Review the code I've selected or the recent changes for:
    - Potential bugs or edge cases
    - Security concerns
    - Performance issues
    - Readability improvements

    Be concise and actionable.
    ```
  </Step>

  <Step title="Crear el manifiesto del plugin">
    Cree un archivo `plugin.json` que describa el plugin. El manifiesto va en el directorio `.claude-plugin/`.

    ```json my-marketplace/plugins/quality-review-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "quality-review-plugin",
      "description": "Adds a quality-review skill for quick code reviews",
      "version": "1.0.0"
    }
    ```

    <Note>
      Establecer `version` significa que los usuarios solo reciben actualizaciones cuando cambia este campo, así que incremente la versión en cada lanzamiento. Si omite `version` y aloja este marketplace en git, cada commit cuenta automáticamente como una nueva versión. Consulte [Resolución de versiones](#version-resolution-and-release-channels) para elegir el enfoque correcto.
    </Note>
  </Step>

  <Step title="Crear el archivo de marketplace">
    Cree el catálogo de marketplace que enumera su plugin.

    ```json my-marketplace/.claude-plugin/marketplace.json theme={null}
    {
      "name": "my-plugins",
      "owner": {
        "name": "Your Name"
      },
      "plugins": [
        {
          "name": "quality-review-plugin",
          "source": "./plugins/quality-review-plugin",
          "description": "Adds a quality-review skill for quick code reviews"
        }
      ]
    }
    ```
  </Step>

  <Step title="Agregar e instalar">
    Agregue el marketplace e instale el plugin.

    ```shell theme={null}
    /plugin marketplace add ./my-marketplace
    /plugin install quality-review-plugin@my-plugins
    ```
  </Step>

  <Step title="Pruébelo">
    Seleccione algo de código en su editor y ejecute su nueva skill. Las skills del plugin tienen un espacio de nombres con el nombre del plugin.

    ```shell theme={null}
    /quality-review-plugin:quality-review
    ```
  </Step>
</Steps>

Para obtener más información sobre lo que los plugins pueden hacer, incluidos hooks, agentes, servidores MCP y servidores LSP, consulte [Plugins](/docs/es/plugins).

<Note>
  **Cómo se instalan los plugins**: Cuando los usuarios instalan un plugin, Claude Code copia el directorio del plugin a una ubicación de caché. Esto significa que los plugins no pueden hacer referencia a archivos fuera de su directorio usando rutas como `../shared-utils`, porque esos archivos no se copiarán.

  Si necesita compartir archivos entre plugins, use enlaces simbólicos. Consulte [Plugin caching and file resolution](/docs/es/plugins-reference#plugin-caching-and-file-resolution) para obtener detalles.
</Note>

<h2 id="create-the-marketplace-file">
  Crear el archivo de marketplace
</h2>

Cree `.claude-plugin/marketplace.json` en la raíz de su repositorio. Este archivo define el nombre de su marketplace, información del propietario y una lista de plugins con sus fuentes.

Cada entrada de plugin necesita como mínimo un `name` y un `source` que le indique a Claude Code dónde obtenerlo. Consulte el [esquema completo](#marketplace-schema) a continuación para todos los campos disponibles.

```json theme={null}
{
  "name": "company-tools",
  "owner": {
    "name": "DevTools Team",
    "email": "devtools@example.com"
  },
  "plugins": [
    {
      "name": "code-formatter",
      "source": "./plugins/formatter",
      "description": "Automatic code formatting on save",
      "version": "2.1.0",
      "author": {
        "name": "DevTools Team"
      }
    },
    {
      "name": "deployment-tools",
      "source": {
        "source": "github",
        "repo": "company/deploy-plugin"
      },
      "description": "Deployment automation tools"
    }
  ]
}
```

<h2 id="marketplace-schema">
  Esquema de marketplace
</h2>

<h3 id="required-fields">
  Campos requeridos
</h3>

| Campo     | Tipo   | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Ejemplo            |
| :-------- | :----- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------- |
| `name`    | string | Identificador de marketplace (kebab-case, sin espacios). Esto es público: los usuarios lo ven al instalar plugins (por ejemplo, `/plugin install my-tool@your-marketplace`). Cada usuario puede registrar solo un marketplace por nombre: agregar un segundo marketplace con el mismo nombre reemplaza el primero. Para publicar múltiples plugins bajo un nombre de marketplace, enumérelos todos en un único [`marketplace.json`](#create-the-marketplace-file). | `"acme-tools"`     |
| `owner`   | object | Información del mantenedor del marketplace ([consulte los campos a continuación](#owner-fields))                                                                                                                                                                                                                                                                                                                                                                   |                    |
| `plugins` | array  | Lista de plugins disponibles                                                                                                                                                                                                                                                                                                                                                                                                                                       | Ver a continuación |

<Note>
  **Nombres reservados**: Los siguientes nombres de marketplace están reservados para uso oficial de Anthropic y no pueden ser utilizados por marketplaces de terceros: `claude-code-marketplace`, `claude-code-plugins`, `claude-plugins-official`, `claude-plugins-community`, `claude-community`, `anthropic-marketplace`, `anthropic-plugins`, `agent-skills`, `anthropic-agent-skills`, `knowledge-work-plugins`, `life-sciences`, `claude-for-legal`, `claude-for-financial-services`, `financial-services-plugins`, `first-party-plugins`, `healthcare`. Los nombres que se hacen pasar por marketplaces oficiales, como `official-claude-plugins` o `anthropic-plugins-v2`, también están bloqueados. Reservar estos nombres evita que un marketplace de terceros se presente como una fuente publicada por Anthropic.

  Claude Code vuelve a verificar los nombres reservados cada vez que carga un marketplace, no solo cuando agrega uno. Un marketplace que fue registrado bajo uno de estos nombres antes de que el nombre se reservara deja de cargar e informa que está [registrado desde una fuente no confiable](/docs/es/errors#marketplace-is-registered-from-an-untrusted-source). Elimine ese marketplace y vuelva a agregarlo desde la fuente oficial de Anthropic. Un marketplace de terceros afectado por un nombre recién reservado se carga nuevamente tan pronto como lo vuelva a agregar bajo un nombre diferente. Antes de v2.1.205, `first-party-plugins` y `healthcare` no estaban reservados, y un marketplace ya registrado bajo un nombre reservado seguía cargándose.
</Note>

<h3 id="owner-fields">
  Campos del propietario
</h3>

| Campo   | Tipo   | Requerido | Descripción                                   |
| :------ | :----- | :-------- | :-------------------------------------------- |
| `name`  | string | Sí        | Nombre del mantenedor o equipo                |
| `email` | string | No        | Correo electrónico de contacto del mantenedor |

<h3 id="optional-fields">
  Campos opcionales
</h3>

| Campo                                 | Tipo   | Descripción                                                                                                                                                                                                                                                                                                                               |
| :------------------------------------ | :----- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `$schema`                             | string | URL del esquema JSON para autocompletado y validación del editor. Claude Code ignora este campo al cargar.                                                                                                                                                                                                                                |
| `description`                         | string | Descripción breve del marketplace                                                                                                                                                                                                                                                                                                         |
| `version`                             | string | Versión del manifiesto del marketplace                                                                                                                                                                                                                                                                                                    |
| `metadata.pluginRoot`                 | string | Directorio base antepuesto a rutas de fuente de plugin relativas (por ejemplo, `"./plugins"` le permite escribir `"source": "formatter"` en lugar de `"source": "./plugins/formatter"`)                                                                                                                                                   |
| `allowCrossMarketplaceDependenciesOn` | array  | Otros marketplaces en los que los plugins en este marketplace pueden depender. Las dependencias de un marketplace no listado aquí se bloquean en la instalación. Consulte [Depender de un plugin de otro marketplace](/docs/es/plugin-dependencies#depend-on-a-plugin-from-another-marketplace).                                               |
| `renames`                             | object | Mapa del anterior `name` de un plugin a su nombre actual, o a `null` si el plugin fue eliminado. Permite que los usuarios existentes migren automáticamente cuando cambia el nombre o elimina una entrada en `plugins`. Consulte [Renombrar o eliminar un plugin](#rename-or-remove-a-plugin). Requiere Claude Code v2.1.193 o posterior. |

`description` y `version` también se aceptan bajo `metadata` para compatibilidad con versiones anteriores.

<h2 id="plugin-entries">
  Entradas de plugins
</h2>

Cada entrada de plugin en el array `plugins` describe un plugin y dónde encontrarlo. Puede incluir cualquier campo del [esquema de manifiesto de plugin](/docs/es/plugins-reference#plugin-manifest-schema), como `description`, `version`, `author`, `commands` y `hooks`, más estos campos específicos del marketplace: `source`, `category`, `tags`, `strict` y `relevance`.

<h3 id="required-fields-2">
  Campos requeridos
</h3>

| Campo    | Tipo           | Descripción                                                                                                                                                  |
| :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`   | string         | Identificador de plugin (kebab-case, sin espacios). Esto es público: los usuarios lo ven al instalar (por ejemplo, `/plugin install my-plugin@marketplace`). |
| `source` | string\|object | Dónde obtener el plugin (consulte [Fuentes de plugins](#plugin-sources) a continuación)                                                                      |

<h3 id="optional-plugin-fields">
  Campos de plugin opcionales
</h3>

**Campos de metadatos estándar:**

| Campo            | Tipo    | Descripción                                                                                                                                                                                                                                                                                                                                                                            |
| :--------------- | :------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `displayName`    | string  | Nombre legible mostrado en superficies de interfaz de usuario. Recurre a `name` cuando se omite. Puede contener espacios y cualquier capitalización. No se utiliza para espacios de nombres o búsqueda. Requiere Claude Code v2.1.143 o posterior.                                                                                                                                     |
| `description`    | string  | Descripción breve del plugin                                                                                                                                                                                                                                                                                                                                                           |
| `version`        | string  | Versión del plugin. Si se establece (aquí o en `plugin.json`), el plugin se fija a esta cadena y los usuarios solo reciben actualizaciones cuando cambia. Omita para recurrir al SHA del commit de git. Consulte [Resolución de versiones](#version-resolution-and-release-channels).                                                                                                  |
| `author`         | object  | Información del autor del plugin (`name` requerido, `email` opcional)                                                                                                                                                                                                                                                                                                                  |
| `homepage`       | string  | URL de página de inicio o documentación del plugin                                                                                                                                                                                                                                                                                                                                     |
| `repository`     | string  | URL del repositorio de código fuente                                                                                                                                                                                                                                                                                                                                                   |
| `license`        | string  | Identificador de licencia SPDX (por ejemplo, MIT, Apache-2.0)                                                                                                                                                                                                                                                                                                                          |
| `keywords`       | array   | Etiquetas para descubrimiento y categorización de plugins                                                                                                                                                                                                                                                                                                                              |
| `category`       | string  | Categoría del plugin para organización                                                                                                                                                                                                                                                                                                                                                 |
| `tags`           | array   | Etiquetas para búsqueda                                                                                                                                                                                                                                                                                                                                                                |
| `strict`         | boolean | Controla si `plugin.json` es la autoridad para definiciones de componentes (predeterminado: true). Consulte [Modo estricto](#strict-mode) a continuación.                                                                                                                                                                                                                              |
| `relevance`      | object  | Señales que indican a Claude Code cuándo sugerir este plugin a los usuarios. Solo tiene efecto para marketplaces que un administrador incluye en la lista de permitidos en la configuración administrada. Consulte [Recomendar plugins para su organización](/docs/es/plugin-relevance). Requiere Claude Code v2.1.152 o posterior.                                                         |
| `defaultEnabled` | boolean | Si el plugin está habilitado después de la instalación (predeterminado: true). Establezca en `false` para instalar el plugin deshabilitado hasta que el usuario opte por participar. Tiene prioridad sobre el mismo campo en el `plugin.json` del plugin. Consulte [Habilitación predeterminada](/docs/es/plugins-reference#default-enablement). Requiere Claude Code v2.1.154 o posterior. |

**Campos de configuración de componentes:**

| Campo        | Tipo           | Descripción                                                                  |
| :----------- | :------------- | :--------------------------------------------------------------------------- |
| `skills`     | string\|array  | Rutas personalizadas a directorios de skills que contienen `<name>/SKILL.md` |
| `commands`   | string\|array  | Rutas personalizadas a archivos de skills planos o directorios               |
| `agents`     | string\|array  | Rutas personalizadas a archivos de agentes                                   |
| `hooks`      | string\|object | Configuración de hooks personalizada o ruta a archivo de hooks               |
| `mcpServers` | string\|object | Configuraciones de servidor MCP o ruta a configuración de MCP                |
| `lspServers` | string\|object | Configuraciones de servidor LSP o ruta a configuración de LSP                |

<h2 id="plugin-sources">
  Fuentes de plugins
</h2>

Las fuentes de plugins le indican a Claude Code dónde obtener cada plugin individual listado en su marketplace. Estos se establecen en el campo `source` de cada entrada de plugin en `marketplace.json`.

Después de que Claude Code clona o descarga un plugin en la máquina local, lo copia en el caché de plugins versionado local en `~/.claude/plugins/cache`.

| Fuente        | Tipo                              | Campos                             | Notas                                                                                                                                                              |
| ------------- | --------------------------------- | ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Ruta relativa | `string` (p. ej. `"./my-plugin"`) | ninguno                            | Directorio local dentro del repositorio de marketplace. Debe comenzar con `./`. Se resuelve relativo a la raíz del marketplace, no al directorio `.claude-plugin/` |
| `github`      | object                            | `repo`, `ref?`, `sha?`             |                                                                                                                                                                    |
| `url`         | object                            | `url`, `ref?`, `sha?`              | Fuente de URL de Git                                                                                                                                               |
| `git-subdir`  | object                            | `url`, `path`, `ref?`, `sha?`      | Subdirectorio dentro de un repositorio git. Clona escasamente para minimizar el ancho de banda para monorepos                                                      |
| `npm`         | object                            | `package`, `version?`, `registry?` | Instalado vía `npm install`                                                                                                                                        |

<Note>
  **Fuentes de marketplace vs fuentes de plugins**: Estos son conceptos diferentes que controlan cosas diferentes.

  * **Fuente de marketplace**: dónde obtener el catálogo `marketplace.json` en sí. Se establece cuando los usuarios ejecutan `/plugin marketplace add` o en la configuración `extraKnownMarketplaces`. Soporta `ref` (rama/etiqueta) pero no `sha`.
  * **Fuente de plugin**: dónde obtener un plugin individual listado en el marketplace. Se establece en el campo `source` de cada entrada de plugin dentro de `marketplace.json`. Soporta tanto `ref` (rama/etiqueta) como `sha` (commit exacto).

  Por ejemplo, un marketplace alojado en `acme-corp/plugin-catalog` (fuente de marketplace) puede listar un plugin obtenido de `acme-corp/code-formatter` (fuente de plugin). La fuente de marketplace y la fuente de plugin apuntan a diferentes repositorios y se fijan independientemente.
</Note>

Los tipos de fuente basados en git que se muestran a continuación son `github`, `url` y `git-subdir`. Cuando tanto `ref` como `sha` se establecen en cualquiera de ellos, `sha` es el pin efectivo. Claude Code obtiene y verifica el commit fijado directamente.

En la mayoría de los hosts de git, incluidos GitHub, GitLab y Bitbucket, esto significa que la instalación tiene éxito incluso si la rama o etiqueta nombrada por `ref` ha sido eliminada posteriormente, siempre que el commit aún sea alcanzable desde el repositorio. Algunos servidores, como AWS CodeCommit, no soportan la obtención de commits por SHA. En esos servidores, `ref` aún debe existir y el commit fijado debe ser alcanzable desde él.

<h3 id="relative-paths">
  Rutas relativas
</h3>

Para plugins en el mismo repositorio, use una ruta que comience con `./`:

```json theme={null}
{
  "name": "my-plugin",
  "source": "./plugins/my-plugin"
}
```

Las rutas se resuelven relativas a la raíz del marketplace, que es el directorio que contiene `.claude-plugin/`. En el ejemplo anterior, `./plugins/my-plugin` apunta a `<repo>/plugins/my-plugin`, aunque `marketplace.json` vive en `<repo>/.claude-plugin/marketplace.json`. No use `../` para hacer referencia a rutas fuera de la raíz del marketplace.

<Note>
  Las rutas relativas se resuelven contra una copia local del marketplace, por lo que funcionan cuando los usuarios agregan su marketplace desde una fuente de git o un directorio local. Si los usuarios agregan su marketplace a través de una URL directa al archivo `marketplace.json`, las rutas relativas no se resolverán, porque solo se descarga ese archivo. Para distribución basada en URL, use fuentes de GitHub, npm o URL de git en su lugar. Consulte [Solución de problemas](#plugins-with-relative-paths-fail-in-url-based-marketplaces) para obtener detalles.
</Note>

<h3 id="github-repositories">
  Repositorios de GitHub
</h3>

```json theme={null}
{
  "name": "github-plugin",
  "source": {
    "source": "github",
    "repo": "owner/plugin-repo"
  }
}
```

Puede fijar a una rama, etiqueta o commit específico:

```json theme={null}
{
  "name": "github-plugin",
  "source": {
    "source": "github",
    "repo": "owner/plugin-repo",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

| Campo  | Tipo   | Descripción                                                                              |
| :----- | :----- | :--------------------------------------------------------------------------------------- |
| `repo` | string | Requerido. Repositorio de GitHub en formato `owner/repo`                                 |
| `ref`  | string | Opcional. Rama o etiqueta de Git (por defecto es la rama predeterminada del repositorio) |
| `sha`  | string | Opcional. SHA de commit de git completo de 40 caracteres para fijar a una versión exacta |

<h3 id="git-repositories">
  Repositorios de Git
</h3>

```json theme={null}
{
  "name": "git-plugin",
  "source": {
    "source": "url",
    "url": "https://gitlab.com/team/plugin.git"
  }
}
```

Puede fijar a una rama, etiqueta o commit específico:

```json theme={null}
{
  "name": "git-plugin",
  "source": {
    "source": "url",
    "url": "https://gitlab.com/team/plugin.git",
    "ref": "main",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

| Campo | Tipo   | Descripción                                                                                                                                                                      |
| :---- | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url` | string | Requerido. URL completa del repositorio de git (`https://` o `git@`). El sufijo `.git` es opcional, por lo que las URLs de Azure DevOps y AWS CodeCommit sin el sufijo funcionan |
| `ref` | string | Opcional. Rama o etiqueta de Git (por defecto es la rama predeterminada del repositorio)                                                                                         |
| `sha` | string | Opcional. SHA de commit de git completo de 40 caracteres para fijar a una versión exacta                                                                                         |

<h3 id="git-subdirectories">
  Subdirectorios de Git
</h3>

Use `git-subdir` para apuntar a un plugin que vive dentro de un subdirectorio de un repositorio de git. Claude Code usa un clon parcial y escaso para obtener solo el subdirectorio, minimizando el ancho de banda para monorepos grandes.

```json theme={null}
{
  "name": "my-plugin",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/acme-corp/monorepo.git",
    "path": "tools/claude-plugin"
  }
}
```

Puede fijar a una rama, etiqueta o commit específico:

```json theme={null}
{
  "name": "my-plugin",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/acme-corp/monorepo.git",
    "path": "tools/claude-plugin",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

El campo `url` también acepta una abreviatura de GitHub (`owner/repo`) o URLs SSH (`git@github.com:owner/repo.git`).

| Campo  | Tipo   | Descripción                                                                                                            |
| :----- | :----- | :--------------------------------------------------------------------------------------------------------------------- |
| `url`  | string | Requerido. URL del repositorio de Git, abreviatura de GitHub `owner/repo` o URL SSH                                    |
| `path` | string | Requerido. Ruta del subdirectorio dentro del repositorio que contiene el plugin (por ejemplo, `"tools/claude-plugin"`) |
| `ref`  | string | Opcional. Rama o etiqueta de Git (por defecto es la rama predeterminada del repositorio)                               |
| `sha`  | string | Opcional. SHA de commit de git completo de 40 caracteres para fijar a una versión exacta                               |

<h3 id="npm-packages">
  Paquetes npm
</h3>

Los plugins distribuidos como paquetes npm se instalan usando `npm install`. Esto funciona con cualquier paquete en el registro npm público o un registro privado que su equipo aloje.

```json theme={null}
{
  "name": "my-npm-plugin",
  "source": {
    "source": "npm",
    "package": "@acme/claude-plugin"
  }
}
```

Para fijar a una versión específica, agregue el campo `version`:

```json theme={null}
{
  "name": "my-npm-plugin",
  "source": {
    "source": "npm",
    "package": "@acme/claude-plugin",
    "version": "2.1.0"
  }
}
```

Para instalar desde un registro privado o interno, agregue el campo `registry`:

```json theme={null}
{
  "name": "my-npm-plugin",
  "source": {
    "source": "npm",
    "package": "@acme/claude-plugin",
    "version": "^2.0.0",
    "registry": "https://npm.example.com"
  }
}
```

| Campo      | Tipo   | Descripción                                                                                                     |
| :--------- | :----- | :-------------------------------------------------------------------------------------------------------------- |
| `package`  | string | Requerido. Nombre del paquete o paquete con alcance (por ejemplo, `@org/plugin`)                                |
| `version`  | string | Opcional. Versión o rango de versión (por ejemplo, `2.1.0`, `^2.0.0`, `~1.5.0`)                                 |
| `registry` | string | Opcional. URL de registro npm personalizado. Por defecto es el registro npm del sistema (típicamente npmjs.org) |

<h3 id="advanced-plugin-entries">
  Entradas de plugins avanzadas
</h3>

Este ejemplo muestra una entrada de plugin usando muchos de los campos opcionales, incluidas rutas personalizadas para commands, agents, hooks y MCP servers:

```json theme={null}
{
  "name": "enterprise-tools",
  "source": {
    "source": "github",
    "repo": "company/enterprise-plugin"
  },
  "description": "Enterprise workflow automation tools",
  "version": "2.1.0",
  "author": {
    "name": "Enterprise Team",
    "email": "enterprise@example.com"
  },
  "homepage": "https://docs.example.com/plugins/enterprise-tools",
  "repository": "https://github.com/company/enterprise-plugin",
  "license": "MIT",
  "keywords": ["enterprise", "workflow", "automation"],
  "category": "productivity",
  "commands": [
    "./commands/core/",
    "./commands/enterprise/",
    "./commands/experimental/preview.md"
  ],
  "agents": ["./agents/security-reviewer.md", "./agents/compliance-checker.md"],
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PLUGIN_ROOT}/scripts/validate.sh"
          }
        ]
      }
    ]
  },
  "mcpServers": {
    "enterprise-db": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"]
    }
  },
  "strict": false
}
```

Cosas clave a notar:

* **`commands` y `agents`**: puede especificar múltiples directorios o archivos individuales. Las rutas son relativas a la raíz del plugin.
* **`${CLAUDE_PLUGIN_ROOT}`**: Use esta variable en hooks y configuraciones de MCP server para hacer referencia a archivos dentro del directorio de instalación del plugin. Esto es necesario porque los plugins se copian a una ubicación de caché cuando se instalan.
  * Consulte la [tabla de sustitución](/docs/es/plugins-reference#environment-variables) para ver qué campos de configuración la sustituyen por tipo de servidor
  * Para dependencias o estado que deben sobrevivir a las actualizaciones de plugins, use [`${CLAUDE_PLUGIN_DATA}`](/docs/es/plugins-reference#persistent-data-directory) en su lugar
* **`strict: false`**: dado que esto se establece en false, el plugin no necesita su propio `plugin.json`. La entrada del marketplace define todo. Consulte [Modo estricto](#strict-mode) a continuación.

Por defecto, las skills de un plugin se cargan desde el directorio `skills/` bajo su `source`. Las rutas listadas en el campo `skills` se agregan a ese escaneo:

```json theme={null}
"skills": ["./skills/", "./extra-skills/"]
```

Cuando varias entradas de plugin comparten una carpeta `skills/` en la raíz del marketplace (`source: "./"`), liste subdirectorios específicos en su lugar para que cada entrada cargue solo sus propias skills:

```json theme={null}
"source": "./",
"skills": ["./skills/code-review", "./skills/docs"]
```

Con una fuente de raíz de marketplace, las rutas listadas son el conjunto completo para esa entrada, y otros directorios en la carpeta `skills/` compartida no se cargan. Listar `./skills/` en sí, o la raíz del plugin, mantiene el escaneo completo. Si ninguna de las rutas listadas existe, se ejecuta el escaneo predeterminado en su lugar.

<h3 id="strict-mode">
  Modo estricto
</h3>

El campo `strict` controla si `plugin.json` es la autoridad para definiciones de componentes (skills, agents, hooks, MCP servers, estilos de salida).

| Valor                   | Comportamiento                                                                                                                                                              |
| :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `true` (predeterminado) | `plugin.json` es la autoridad. La entrada del marketplace puede complementarla con componentes adicionales, y ambas fuentes se fusionan.                                    |
| `false`                 | La entrada del marketplace es la definición completa. Si el plugin también tiene un `plugin.json` que declara componentes, eso es un conflicto y el plugin falla al cargar. |

**Cuándo usar cada modo:**

* **`strict: true`**: el plugin tiene su propio `plugin.json` y gestiona sus propios componentes. La entrada del marketplace puede agregar skills o hooks adicionales encima. Este es el predeterminado y funciona para la mayoría de los plugins.
* **`strict: false`**: el operador del marketplace quiere control total. El repositorio del plugin proporciona archivos sin procesar, y la entrada del marketplace define cuáles de esos archivos se exponen como skills, agents, hooks, etc. Útil cuando el marketplace reestructura o cura los componentes de un plugin de manera diferente a la que el autor del plugin pretendía.

<h2 id="host-and-distribute-marketplaces">
  Alojar y distribuir marketplaces
</h2>

<h3 id="host-on-github-recommended">
  Alojar en GitHub (recomendado)
</h3>

GitHub es la forma recomendada para alojar y distribuir un marketplace:

1. **Crear un repositorio**: configure un nuevo repositorio para su marketplace
2. **Agregar archivo de marketplace**: cree `.claude-plugin/marketplace.json` con sus definiciones de plugins
3. **Compartir con equipos**: los usuarios agregan su marketplace con `/plugin marketplace add owner/repo`

**Beneficios**: control de versiones integrado, seguimiento de problemas y características de colaboración en equipo.

<h3 id="host-on-other-git-services">
  Alojar en otros servicios de git
</h3>

Cualquier servicio de alojamiento de git funciona, como GitLab, Bitbucket y servidores autohospedados. Los usuarios agregan con la URL completa del repositorio:

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git
```

<h3 id="private-repositories">
  Repositorios privados
</h3>

Claude Code soporta instalar plugins desde repositorios privados. Para instalación manual y actualizaciones, Claude Code usa sus ayudantes de credenciales de git existentes, por lo que el acceso HTTPS a través de `gh auth login`, Keychain de macOS o `git-credential-store` funciona igual que en su terminal. El acceso SSH funciona siempre que el host ya esté en su archivo `known_hosts` y la clave esté cargada en `ssh-agent`, ya que Claude Code suprime los mensajes interactivos de SSH para la huella digital del host y la contraseña de la clave. Los atajos de teclado de GitHub `owner/repo` clonan sobre SSH de forma predeterminada; establezca [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/es/env-vars#variables) para clonarlos sobre HTTPS en su lugar.

Las actualizaciones automáticas en segundo plano funcionan de manera diferente. De forma predeterminada, la actualización en segundo plano deshabilita los ayudantes de credenciales de git para su `git pull`, por lo que la extracción no puede autenticarse en repositorios privados sobre HTTPS incluso cuando un ayudante está configurado. Los remotos SSH no se ven afectados: una clave cargada en `ssh-agent` autentica las extracciones en segundo plano de la misma manera que las operaciones manuales. Cuando la extracción en segundo plano falla, Claude Code vuelve a clonar el marketplace desde cero. El re-clonado sí usa sus credenciales de git almacenadas, pero puede [agotar el tiempo de espera en repositorios grandes](#git-operations-time-out), por lo que las actualizaciones automáticas de marketplace privado pueden fallar intermitentemente.

Dos configuraciones hacen que los marketplaces privados se comporten de manera predecible:

* Establezca `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1` para mantener el clon existente cuando la extracción en segundo plano falla, en lugar de eliminar y re-clonar. Sus plugins siguen funcionando desde el último estado sincronizado, y las actualizaciones manuales con `/plugin marketplace update` aún extraen con sus credenciales.
* Configure un ayudante de credenciales de git, por ejemplo con `gh auth setup-git` para GitHub, para que la alternativa de re-clonado pueda autenticarse sin solicitar.

Establecer un token de proveedor como `GITHUB_TOKEN` en su entorno no habilita por sí solo la autenticación en segundo plano. Los tokens tienen efecto solo a través de un ayudante de credenciales configurado, por ejemplo el ayudante de CLI `gh`, que lee `GH_TOKEN` y `GITHUB_TOKEN`.

Para hacer que la extracción en segundo plano se autentique sobre HTTPS, configure una reescritura de URL de git global. La reescritura incrusta un token en la URL remota, por lo que tiene efecto aunque la extracción en segundo plano deshabilite los ayudantes de credenciales, y una extracción exitosa omite la alternativa de re-clonado. El siguiente ejemplo reescribe la URL del repositorio del marketplace para incluir un token de acceso:

```bash theme={null}
git config --global url."https://x-access-token:YOUR_TOKEN@github.com/acme-corp/plugins".insteadOf "https://github.com/acme-corp/plugins"
```

Limite el alcance de la reescritura al repositorio del marketplace o ruta de la organización. Una reescritura cuya base es solo el host se aplica a cada extracción e inserción a ese host en la máquina e invalida sus credenciales normales, incluidas las inserciones a sus propios repositorios.

Cada proveedor espera un nombre de usuario diferente en la URL reescrita, y el mismo alcance de ruta se aplica a cada proveedor. Para servidores autohospedados, reemplace el nombre de host con el nombre de host de su servidor:

| Proveedor | Forma de URL reescrita                                            |
| :-------- | :---------------------------------------------------------------- |
| GitHub    | `https://x-access-token:YOUR_TOKEN@github.com/acme-corp/plugins`  |
| GitLab    | `https://oauth2:YOUR_TOKEN@gitlab.com/acme-corp/plugins`          |
| Bitbucket | `https://x-token-auth:YOUR_TOKEN@bitbucket.org/acme-corp/plugins` |

La reescritura almacena el token en texto plano en su gitconfig, por lo que use un token con acceso de solo lectura al repositorio del marketplace.

<Note>
  En entornos de CI/CD, configure un ayudante de credenciales de git antes de instalar plugins desde repositorios privados. En GitHub Actions, exporte un token con acceso de lectura al repositorio del marketplace como `GH_TOKEN`, luego ejecute `gh auth setup-git`. El token de flujo de trabajo predeterminado solo puede acceder al repositorio del flujo de trabajo, por lo que un marketplace privado en otro repositorio necesita un token de acceso personal o token de aplicación. Una reescritura de URL global configurada en la canalización también autentica la extracción en segundo plano directamente.
</Note>

<h3 id="test-locally-before-distribution">
  Probar localmente antes de la distribución
</h3>

Pruebe su marketplace localmente antes de compartirlo:

```shell theme={null}
/plugin marketplace add ./my-marketplace
/plugin install quality-review-plugin@my-plugins
```

Para el rango completo de comandos add (GitHub, URLs de Git, rutas locales, URLs remotas), consulte [Agregar marketplaces](/docs/es/discover-plugins#add-marketplaces).

<h3 id="require-marketplaces-for-your-team">
  Requerir marketplaces para su equipo
</h3>

Puede configurar su repositorio para que los miembros del equipo sean automáticamente solicitados para instalar su marketplace cuando confíen en la carpeta del proyecto. Agregue su marketplace a `.claude/settings.json`:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "company-tools": {
      "source": {
        "source": "github",
        "repo": "your-org/claude-plugins"
      }
    }
  }
}
```

También puede especificar qué plugins deben estar habilitados de forma predeterminada:

```json theme={null}
{
  "enabledPlugins": {
    "code-formatter@company-tools": true,
    "deployment-tools@company-tools": true
  }
}
```

Para opciones de configuración completas, consulte [Configuración de plugins](/docs/es/settings#plugin-settings).

<Note>
  Si usa una fuente local `directory` o `file` con una ruta relativa, la ruta se resuelve contra el checkout principal de su repositorio. Cuando ejecuta Claude Code desde un git worktree, la ruta aún apunta al checkout principal, por lo que todos los worktrees comparten la misma ubicación de marketplace. El estado del marketplace se almacena una vez por usuario en `~/.claude/plugins/known_marketplaces.json`, no por proyecto.
</Note>

<h3 id="pre-populate-plugins-for-containers">
  Precargar plugins para contenedores
</h3>

Para imágenes de contenedor y entornos de CI, puede precargar un directorio de plugins en tiempo de compilación para que Claude Code comience con marketplaces y plugins ya disponibles, sin clonar nada en tiempo de ejecución. Establezca la variable de entorno `CLAUDE_CODE_PLUGIN_SEED_DIR` para apuntar a este directorio.

Para superponer múltiples directorios seed, separe las rutas con `:` en Unix o `;` en Windows. Claude Code busca cada directorio en orden y usa el primer seed que contiene un marketplace o caché de plugin dado.

El directorio seed refleja la estructura de `~/.claude/plugins`:

```
$CLAUDE_CODE_PLUGIN_SEED_DIR/
  known_marketplaces.json
  marketplaces/<name>/...
  cache/<marketplace>/<plugin>/<version>/...
```

Para construir un directorio seed, ejecute Claude Code una vez durante la compilación de la imagen, instale los plugins que necesita, luego copie el directorio `~/.claude/plugins` resultante en su imagen y apunte `CLAUDE_CODE_PLUGIN_SEED_DIR` a él.

Para omitir el paso de copia, establezca `CLAUDE_CODE_PLUGIN_CACHE_DIR` en su ruta de seed de destino durante la compilación para que los plugins se instalen directamente allí:

```bash theme={null}
CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin marketplace add your-org/plugins
CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin install my-tool@your-plugins
```

Luego establezca `CLAUDE_CODE_PLUGIN_SEED_DIR=/opt/claude-seed` en el entorno de tiempo de ejecución de su contenedor para que Claude Code lea desde el seed al inicio.

Al inicio, Claude Code registra los marketplaces encontrados en el `known_marketplaces.json` del seed en la configuración principal, y usa cachés de plugins encontrados bajo `cache/` en su lugar sin re-clonar. Esto funciona tanto en modo interactivo como en modo no interactivo con la bandera `-p`.

Detalles de comportamiento:

* **Solo lectura**: el directorio seed nunca se escribe. Las actualizaciones automáticas están deshabilitadas para marketplaces seed ya que git pull fallaría en un sistema de archivos de solo lectura.
* **Las entradas seed tienen precedencia**: los marketplaces declarados en el seed sobrescriben cualquier entrada coincidente en la configuración del usuario en cada inicio. Para optar por no participar en un plugin seed, use `/plugin disable` en lugar de eliminar el marketplace.
* **Resolución de rutas**: Claude Code localiza contenido de marketplace sondeando `$CLAUDE_CODE_PLUGIN_SEED_DIR/marketplaces/<name>/` en tiempo de ejecución, no confiando en rutas almacenadas dentro del JSON del seed. Esto significa que el seed funciona correctamente incluso cuando se monta en una ruta diferente a donde fue construido.
* **Se bloquea la mutación**: ejecutar `/plugin marketplace remove` o `/plugin marketplace update` contra un marketplace administrado por seed falla con orientación para pedir a su administrador que actualice la imagen seed.
* **Se compone con configuración**: si `extraKnownMarketplaces` o `enabledPlugins` declaran un marketplace que ya existe en el seed, Claude Code usa la copia del seed en lugar de clonar.

<h3 id="managed-marketplace-restrictions">
  Restricciones de marketplace administrado
</h3>

Para organizaciones que requieren control estricto sobre las fuentes de plugins, los administradores pueden restringir qué marketplaces de plugins se permite a los usuarios agregar usando la configuración [`strictKnownMarketplaces`](/docs/es/settings#strictknownmarketplaces) en configuración administrada. Para también rechazar las banderas de CLI que cargan plugins, agentes y servidores MCP para una única ejecución, emparéjelo con [`disableSideloadFlags`](/docs/es/settings#available-settings). Para permitir qué plugins de marketplaces pueden aparecer como sugerencias de instalación contextual, establezca [`pluginSuggestionMarketplaces`](/docs/es/settings#available-settings).

Cuando `strictKnownMarketplaces` se configura en configuración administrada, el comportamiento de restricción depende del valor:

| Valor                       | Comportamiento                                                                                     |
| --------------------------- | -------------------------------------------------------------------------------------------------- |
| Indefinido (predeterminado) | Sin restricciones. Los usuarios pueden agregar cualquier marketplace                               |
| Array vacío `[]`            | Bloqueo completo. Los usuarios no pueden agregar nuevos marketplaces                               |
| Lista de fuentes            | Los usuarios solo pueden agregar marketplaces que coincidan exactamente con la lista de permitidos |

<h4 id="common-configurations">
  Configuraciones comunes
</h4>

Deshabilitar todas las adiciones de marketplace:

```json theme={null}
{
  "strictKnownMarketplaces": []
}
```

Permitir solo marketplaces específicos:

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "github",
      "repo": "acme-corp/approved-plugins"
    },
    {
      "source": "github",
      "repo": "acme-corp/security-tools",
      "ref": "v2.0"
    },
    {
      "source": "url",
      "url": "https://plugins.example.com/marketplace.json"
    }
  ]
}
```

Permitir todos los marketplaces desde un servidor git interno usando coincidencia de patrón regex en el host. Este es el enfoque recomendado para [GitHub Enterprise Server](/docs/es/github-enterprise-server#plugin-marketplaces-on-ghes) o instancias de GitLab autohospedadas:

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "hostPattern",
      "hostPattern": "^github\\.example\\.com$"
    }
  ]
}
```

Permitir marketplaces basados en sistema de archivos desde un directorio específico usando coincidencia de patrón regex en la ruta:

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "pathPattern",
      "pathPattern": "^/opt/approved/"
    }
  ]
}
```

Use `".*"` como `pathPattern` para permitir cualquier ruta del sistema de archivos mientras aún controla fuentes de red con `hostPattern`.

<Note>
  `strictKnownMarketplaces` restringe lo que los usuarios pueden agregar, pero no registra marketplaces por sí solo. Para hacer que los marketplaces permitidos estén disponibles automáticamente sin que los usuarios ejecuten `/plugin marketplace add`, emparéjelo con [`extraKnownMarketplaces`](/docs/es/settings#extraknownmarketplaces) en el mismo `managed-settings.json`. Consulte [Usar ambos juntos](/docs/es/settings#strictknownmarketplaces).
</Note>

<h4 id="how-restrictions-work">
  Cómo funcionan las restricciones
</h4>

Las restricciones se validan antes de cualquier operación de red o del sistema de archivos. La verificación se ejecuta al agregar marketplace y al instalar, actualizar, actualizar y auto-actualizar plugins. Si un marketplace se agregó antes de que se configurara la política y su fuente ya no coincide con la lista de permitidos, Claude Code se niega a instalar o actualizar plugins desde él. La misma aplicación se aplica a `blockedMarketplaces`.

La lista de permitidos usa coincidencia exacta para la mayoría de tipos de fuente. Para que un marketplace sea permitido, todos los campos especificados deben coincidir exactamente:

* Para fuentes de GitHub: `repo` es requerido, y `ref` o `path` también deben coincidir si se especifican en la lista de permitidos
* Para fuentes de URL: la URL completa debe coincidir exactamente
* Para fuentes `hostPattern`: el host del marketplace se compara contra el patrón regex
* Para fuentes `pathPattern`: la ruta del sistema de archivos del marketplace se compara contra el patrón regex

La coincidencia exacta no normaliza URLs: una barra diagonal final, sufijo `.git` o forma `ssh://` versus `https://` se tratan como valores diferentes. Si el marketplace de su organización se puede clonar por más de una forma de URL, prefiera una entrada `hostPattern` sobre una URL literal para que todas las formas coincidan.

Debido a que `strictKnownMarketplaces` se establece en [configuración administrada](/docs/es/settings#settings-files), los usuarios individuales y las configuraciones del proyecto no pueden anular estas restricciones.

Para detalles de configuración completos incluyendo todos los tipos de fuente soportados y comparación con `extraKnownMarketplaces`, consulte la [referencia de strictKnownMarketplaces](/docs/es/settings#strictknownmarketplaces).

<h3 id="version-resolution-and-release-channels">
  Resolución de versiones y canales de lanzamiento
</h3>

Las versiones de plugins determinan rutas de caché y detección de actualizaciones: si la versión resuelta coincide con lo que un usuario ya tiene, `/plugin update` y auto-actualización omiten el plugin.

Claude Code resuelve la versión de un plugin desde el primero de estos que esté establecido:

1. `version` en el `plugin.json` del plugin
2. `version` en la entrada del marketplace del plugin
3. El SHA del commit de git de la fuente del plugin

Para los tipos de fuente basados en git `github`, `url`, `git-subdir` y rutas relativas dentro de un marketplace alojado en git, puede omitir `version` completamente y cada nuevo commit se trata como una nueva versión. Esta es la configuración más simple para plugins internos o en desarrollo activo.

<Warning>
  Establecer `version` fija el plugin. Si `plugin.json` declara `"version": "1.0.0"`, empujar nuevos commits sin cambiar esa cadena no hace nada para usuarios existentes, porque Claude Code ve la misma versión y mantiene la copia en caché. Aumente el campo en cada lanzamiento, u omítalo para usar el SHA del commit.

  Evite establecer `version` en ambos `plugin.json` y la entrada del marketplace. El valor de `plugin.json` siempre gana silenciosamente, por lo que una versión de manifiesto obsoleta puede enmascarar una versión que estableció en `marketplace.json`.
</Warning>

<h4 id="set-up-release-channels">
  Configurar canales de lanzamiento
</h4>

Para soportar canales de lanzamiento "estable" y "último" para sus plugins, puede configurar dos marketplaces que apunten a diferentes refs o SHAs del mismo repositorio. Luego puede asignar los dos marketplaces a diferentes grupos de usuarios a través de [configuración administrada](/docs/es/settings#settings-files).

<Warning>
  Cada canal debe resolver a una versión diferente. Si usa versiones explícitas, `plugin.json` debe declarar una `version` diferente en cada ref fijado. Si omite `version`, los SHAs de commit distintos ya distinguen los canales. Si dos refs resuelven a la misma cadena de versión, Claude Code los trata como idénticos y omite la actualización.
</Warning>

<h5 id="example">
  Ejemplo
</h5>

```json theme={null}
{
  "name": "stable-tools",
  "plugins": [
    {
      "name": "code-formatter",
      "source": {
        "source": "github",
        "repo": "acme-corp/code-formatter",
        "ref": "stable"
      }
    }
  ]
}
```

```json theme={null}
{
  "name": "latest-tools",
  "plugins": [
    {
      "name": "code-formatter",
      "source": {
        "source": "github",
        "repo": "acme-corp/code-formatter",
        "ref": "latest"
      }
    }
  ]
}
```

<h5 id="assign-channels-to-user-groups">
  Asignar canales a grupos de usuarios
</h5>

Asigne cada marketplace al grupo de usuarios apropiado a través de configuración administrada. Por ejemplo, el grupo estable recibe:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "stable-tools": {
      "source": {
        "source": "github",
        "repo": "acme-corp/stable-tools"
      }
    }
  }
}
```

El grupo de acceso temprano recibe `latest-tools` en su lugar:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "latest-tools": {
      "source": {
        "source": "github",
        "repo": "acme-corp/latest-tools"
      }
    }
  }
}
```

<h4 id="pin-dependency-versions">
  Fijar versiones de dependencias
</h4>

Un plugin puede restringir sus dependencias a un rango semver para que las actualizaciones de una dependencia no rompan el plugin dependiente. Consulte [Restringir versiones de dependencias de plugins](/docs/es/plugin-dependencies) para la convención de etiqueta de git `{plugin-name}--v{version}`, sintaxis de rango y cómo se combinan múltiples restricciones en la misma dependencia.

<h3 id="rename-or-remove-a-plugin">
  Renombrar o eliminar un plugin
</h3>

El `name` de un plugin es su identificador estable. Los usuarios lo referencian en `enabledPlugins`, `pluginConfigs` y comandos `/plugin install`, por lo que cambiarlo rompe cada instalación existente. Para cambiar la etiqueta mostrada en la interfaz de usuario sin romper instalaciones, establezca [`displayName`](#optional-plugin-fields) y mantenga `name` sin cambios.

Si debe cambiar el `name` de un plugin, o elimina un plugin del array `plugins`, agregue una entrada de nivel superior `renames` para que los usuarios existentes migren en lugar de ver un error `plugin-not-found`. La migración automática requiere Claude Code v2.1.193 o posterior. Asigne cada nombre anterior a su nombre actual, o a `null` si el plugin ya no existe. El siguiente ejemplo renombra `formatter` a `code-formatter` y registra que `legacy-linter` fue eliminado:

```json theme={null}
{
  "name": "acme-tools",
  "owner": { "name": "Acme" },
  "plugins": [
    { "name": "code-formatter", "source": "./plugins/code-formatter" }
  ],
  "renames": {
    "formatter": "code-formatter",
    "legacy-linter": null
  }
}
```

Cuando un usuario inicia Claude Code con el nombre anterior aún en su configuración, Claude Code sigue el mapa `renames`:

* Si la entrada apunta a un nuevo nombre, Claude Code carga el plugin bajo su nuevo nombre y muestra un aviso de una línea como `Renamed to "code-formatter" in the "acme-tools" marketplace`. Luego reescribe la clave anterior a la nueva clave en los ámbitos de configuración del usuario, proyecto y local para `enabledPlugins` y `pluginConfigs`, por lo que el aviso aparece una vez.
* Para una entrada `null`, Claude Code elimina la clave anterior y el aviso reporta que el plugin fue eliminado del marketplace.
* Si el plugin renombrado usa una fuente remota como `github` o `npm`, Claude Code reporta `plugin-cache-miss` después del renombramiento y el usuario debe ejecutar `/plugin install` una vez para obtenerlo bajo el nuevo nombre.

Trate `renames` como historial de solo anexión: mantenga las entradas antiguas en su lugar incluso después de que espere que cada usuario haya migrado. Claude Code sigue cadenas, por lo que si más tarde renombra `code-formatter` a `formatter-pro`, agregue una segunda entrada en lugar de editar la primera. Un usuario que aún tiene el `formatter` original habilitado luego se resuelve a través de ambas entradas a `formatter-pro`.

Ejecute `claude plugin validate .` después de editar el mapa; rechaza cualquier entrada cuya cadena forme un ciclo o no termine en `null` o un nombre listado en `plugins`.

<Note>
  La configuración administrada y de política es de solo lectura para Claude Code, por lo que los plugins habilitados allí no pueden ser reescritos automáticamente. El plugin renombrado aún se carga cada sesión, pero el aviso de renombramiento recurrirá hasta que un administrador actualice `enabledPlugins` en el archivo de configuración administrada para usar el nuevo nombre. Lo mismo se aplica a los plugins habilitados a través de otras fuentes de solo lectura como `--add-dir`.
</Note>

Las versiones anteriores de Claude Code ignoran el campo `renames` y reportan `plugin-not-found` para el nombre anterior.

<h2 id="validation-and-testing">
  Validación y pruebas
</h2>

Pruebe su marketplace antes de compartirlo.

Valide la sintaxis JSON de su marketplace:

```bash theme={null}
claude plugin validate .
```

O desde dentro de Claude Code:

```shell theme={null}
/plugin validate .
```

Agregue el marketplace para pruebas:

```shell theme={null}
/plugin marketplace add ./path/to/marketplace
```

Instale un plugin de prueba para verificar que todo funciona:

```shell theme={null}
/plugin install test-plugin@marketplace-name
```

Para flujos de trabajo completos de prueba de plugins, consulte [Pruebe sus plugins localmente](/docs/es/plugins#test-your-plugins-locally). Para solución de problemas técnicos, consulte [Referencia de plugins](/docs/es/plugins-reference).

<h2 id="manage-marketplaces-from-the-cli">
  Administrar marketplaces desde la CLI
</h2>

Claude Code proporciona subcomandos no interactivos `claude plugin marketplace` para scripting y automatización. Estos son equivalentes a los comandos `/plugin marketplace` disponibles dentro de una sesión interactiva.

<h3 id="plugin-marketplace-add">
  Plugin marketplace add
</h3>

Agregue un marketplace desde un repositorio de GitHub, URL de git, URL remota o ruta local.

```bash theme={null}
claude plugin marketplace add <source> [options]
```

**Argumentos:**

* `<source>`: Abreviatura de GitHub `owner/repo`, URL de git, URL remota a un archivo `marketplace.json` o ruta de directorio local. Para fijar a una rama o etiqueta, agregue `@ref` a la abreviatura de GitHub o `#ref` a una URL de git

Una URL debe incluir su esquema. A partir de Claude Code v2.1.196, un host escrito sin uno, como `gitlab.example.com/team/plugins`, se rechaza como una abreviatura `owner/repo` inválida y el error le indica que agregue `https://` o use `./` para una ruta local. Las versiones anteriores lo malinterpretaban como una ruta de repositorio de GitHub y fallan en el momento del clon con un error de no encontrado de GitHub.

**Opciones:**

| Opción                | Descripción                                                                                                                                         | Predeterminado |
| :-------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------- | :------------- |
| `--scope <scope>`     | Dónde declarar el marketplace: `user`, `project` o `local`. Consulte [Plugin installation scopes](/docs/es/plugins-reference#plugin-installation-scopes) | `user`         |
| `--sparse <paths...>` | Limitar el checkout a directorios específicos a través de git sparse-checkout. Útil para monorepos                                                  |                |

Agregue un marketplace desde GitHub usando la abreviatura `owner/repo`:

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins
```

Fije a una rama o etiqueta específica con `@ref`:

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins@v2.0
```

Agregue desde una URL de git en un host que no sea GitHub:

```bash theme={null}
claude plugin marketplace add https://gitlab.example.com/team/plugins.git
```

Agregue desde una URL remota que sirva el archivo `marketplace.json` directamente:

```bash theme={null}
claude plugin marketplace add https://example.com/marketplace.json
```

Agregue desde un directorio local para pruebas:

```bash theme={null}
claude plugin marketplace add ./my-marketplace
```

Declare el marketplace en alcance de proyecto para que se comparta con su equipo a través de `.claude/settings.json`:

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins --scope project
```

Para un monorepo, limite el checkout a los directorios que contienen contenido de plugins:

```bash theme={null}
claude plugin marketplace add acme-corp/monorepo --sparse .claude-plugin plugins
```

<h3 id="plugin-marketplace-list">
  Plugin marketplace list
</h3>

Enumere todos los marketplaces configurados.

```bash theme={null}
claude plugin marketplace list [options]
```

**Opciones:**

| Opción   | Descripción      |
| :------- | :--------------- |
| `--json` | Salida como JSON |

Con `--json`, cada entrada incluye `name`, `source` y campos específicos de la fuente: `repo` para fuentes de GitHub, `url` para fuentes de git y URL, y `path` para fuentes locales. Las fuentes de GitHub y git también incluyen un campo `ref` cuando el marketplace se agregó con una rama o etiqueta fija.

<h3 id="plugin-marketplace-remove">
  Plugin marketplace remove
</h3>

Elimine un marketplace configurado. El alias `rm` también se acepta.

```bash theme={null}
claude plugin marketplace remove <name> [options]
```

**Argumentos:**

* `<name>`: nombre del marketplace a eliminar, como se muestra en `claude plugin marketplace list`. Este es el `name` de `marketplace.json`, no la fuente que pasó a `add`

**Opciones:**

| Opción            | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Predeterminado       |
| :---------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------- |
| `--scope <scope>` | Restringir la eliminación a un único alcance de configuración: `user`, `project` o `local`. Consulte [Plugin installation scopes](/docs/es/plugins-reference#plugin-installation-scopes). Cuando se omite, la declaración se elimina de cada alcance editable. Cuando se proporciona, solo se elimina la declaración de ese alcance; el estado compartido, la caché y los datos de plugins instalados se conservan cuando el marketplace aún se declara en otro alcance | (todos los alcances) |

<Warning>
  Eliminar un marketplace de su último alcance restante también desinstala cualquier plugin que haya instalado desde él. Para actualizar un marketplace sin perder plugins instalados, use `claude plugin marketplace update` en su lugar.
</Warning>

<h3 id="plugin-marketplace-update">
  Plugin marketplace update
</h3>

Actualice marketplaces desde sus fuentes para recuperar nuevos plugins y cambios de versión. Un marketplace agregado con una rama o etiqueta `ref` se actualiza a la confirmación más reciente de esa ref, no a la rama predeterminada del repositorio.

```bash theme={null}
claude plugin marketplace update [name]
```

**Argumentos:**

* `[name]`: nombre del marketplace a actualizar, como se muestra en `claude plugin marketplace list`. Actualiza todos los marketplaces si se omite

Tanto `remove` como `update` fallan cuando se ejecutan contra un marketplace administrado por seed, que es de solo lectura. Al actualizar todos los marketplaces, las entradas administradas por seed se omiten y otros marketplaces aún se actualizan. Para cambiar plugins proporcionados por seed, pida a su administrador que actualice la imagen seed. Consulte [Precargar plugins para contenedores](#pre-populate-plugins-for-containers).

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="marketplace-not-loading">
  Marketplace no se carga
</h3>

**Síntomas**: No puede agregar marketplace o ver plugins de él

**Soluciones**:

* Verifique que la URL del marketplace sea accesible
* Compruebe que `.claude-plugin/marketplace.json` existe en la ruta especificada
* Asegúrese de que la sintaxis JSON sea válida usando `claude plugin validate` o `/plugin validate`. Para verificar el frontmatter de skill, agente y comando, ejecute el comando contra cada directorio de plugin
* Para repositorios privados, confirme que tiene permisos de acceso

<h3 id="marketplace-validation-errors">
  Errores de validación de marketplace
</h3>

Ejecute `claude plugin validate .` o `/plugin validate .` desde su directorio de marketplace para verificar problemas. Cuando se apunta a un directorio de marketplace, el validador verifica `marketplace.json` para errores de esquema, nombres de plugins duplicados y traversal de ruta de fuente. Para cada entrada cuya `source` es una ruta local, también valida el `plugin.json` de ese plugin y advierte cuando la `version` de la entrada no coincide con la de `plugin.json`. Los problemas encontrados en el `plugin.json` de un plugin tienen el prefijo del índice de entrada, en la forma `plugins[2] plugin.json →`.

A partir de Claude Code v2.1.196, el pase por entrada también:

* incluye plugins cuya `source` es `.`
* se ejecuta cuando `marketplace.json` está fuera de un directorio `.claude-plugin`, resolviendo fuentes contra el directorio del archivo en sí
* reporta los problemas de cada entrada incluso cuando otra parte del archivo tiene errores de esquema

Las versiones anteriores omiten plugins en la raíz del marketplace y solo descienden desde un `.claude-plugin/marketplace.json`.

Para validar el `plugin.json` de un plugin individual y sus archivos de skill, agente, comando y hook, ejecute el comando contra el directorio del plugin en sí, por ejemplo `claude plugin validate ./plugins/my-plugin`. Errores comunes:

| Error                                             | Causa                                                  | Solución                                                                                                                                                                |
| :------------------------------------------------ | :----------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `File not found: .claude-plugin/marketplace.json` | Manifiesto faltante                                    | Cree `.claude-plugin/marketplace.json` con campos requeridos                                                                                                            |
| `Invalid JSON syntax: Unexpected token...`        | Error de sintaxis JSON en marketplace.json             | Verifique comas faltantes, comas extra o cadenas sin comillas                                                                                                           |
| `Duplicate plugin name "x" found in marketplace`  | Dos plugins comparten el mismo nombre                  | Dé a cada plugin un valor `name` único                                                                                                                                  |
| `plugins[0].source: Path contains ".."`           | La ruta de fuente contiene `..`                        | Use rutas relativas a la raíz del marketplace sin `..`. Consulte [Rutas relativas](#relative-paths)                                                                     |
| `YAML frontmatter failed to parse: ...`           | YAML inválido en un archivo de skill, agente o comando | Corrija la sintaxis YAML en el bloque frontmatter. En tiempo de ejecución este archivo se carga sin metadatos. Se reporta solo cuando se valida un directorio de plugin |
| `Invalid JSON syntax: ...` (hooks.json)           | `hooks/hooks.json` malformado                          | Corrija la sintaxis JSON. Un `hooks/hooks.json` malformado previene que todo el plugin se cargue. Se reporta solo cuando se valida un directorio de plugin              |

**Advertencias** (no bloqueantes):

* `Marketplace has no plugins defined`: agregue al menos un plugin al array `plugins`
* `No marketplace description provided`: agregue una `description` de nivel superior para ayudar a los usuarios a entender su marketplace
* `Plugin name "x" is not kebab-case`: el nombre del plugin contiene letras mayúsculas, espacios o caracteres especiales. Renombre a letras minúsculas, dígitos y guiones solamente (por ejemplo, `my-plugin`). Claude Code acepta otras formas, pero la sincronización del marketplace de claude.ai las rechaza.

<h3 id="plugin-installation-failures">
  Fallos de instalación de plugins
</h3>

**Síntomas**: El marketplace aparece pero la instalación del plugin falla

**Soluciones**:

* Verifique que las URLs de fuente del plugin sean accesibles
* Compruebe que los directorios de plugins contengan archivos requeridos
* Para fuentes de GitHub, asegúrese de que los repositorios sean públicos o tenga acceso
* Pruebe las fuentes de plugins manualmente clonando/descargando
* Si la fuente fija tanto `ref` como `sha`, una rama o etiqueta ascendente eliminada no bloquea la instalación en la mayoría de los hosts de git, incluyendo GitHub, GitLab y Bitbucket. En servidores que no soportan obtener commits por SHA, como AWS CodeCommit, el `ref` aún debe existir y el commit fijado debe ser alcanzable desde él. Si la instalación aún falla, confirme que el commit fijado aún existe en el repositorio

<h3 id="private-repository-authentication-fails">
  La autenticación del repositorio privado falla
</h3>

**Síntomas**: Errores de autenticación al instalar plugins desde repositorios privados

**Soluciones**:

Para instalación manual y actualizaciones:

* Verifique que esté autenticado con su proveedor de git (por ejemplo, ejecute `gh auth status` para GitHub)
* Compruebe que su ayudante de credenciales esté configurado correctamente: `git config --global credential.helper`
* Intente clonar el repositorio manualmente para verificar que sus credenciales funcionan

Para actualizaciones automáticas en segundo plano:

* Por defecto, las actualizaciones en segundo plano desactivan los ayudantes de credenciales de git para la extracción, por lo que la extracción no puede autenticarse sobre HTTPS. Los remotos SSH con una clave cargada en `ssh-agent` aún se autentican. Una extracción fallida desencadena un re-clonado desde cero, que usa sus credenciales almacenadas pero puede agotar el tiempo de espera en repositorios grandes
* Establezca `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1` para mantener el clon existente cuando la extracción en segundo plano falla
* Configure un ayudante de credenciales de git, por ejemplo `gh auth setup-git`, para que el re-clonado fallback pueda autenticarse
* Si el re-clonado agota el tiempo de espera en un repositorio grande, aumente el límite con [`CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS`](#git-operations-time-out)
* Configure una [reescritura de URL de git](#private-repositories) limitada al repositorio del marketplace para que la extracción en segundo plano se autentique directamente
* O actualice marketplaces privados manualmente con `/plugin marketplace update <name>`, que usa sus credenciales

<h3 id="marketplace-updates-fail-in-offline-environments">
  Las actualizaciones del marketplace fallan en entornos sin conexión
</h3>

**Síntomas**: El `git pull` del marketplace falla en segundo plano y Claude Code intenta repetidamente un re-clonado que no puede tener éxito.

**Causa**: Por defecto, cuando un `git pull` falla, Claude Code intenta un re-clonado desde cero. En entornos sin conexión o aislados, el re-clonado falla de la misma manera, y la restauración del caché anterior después es de mejor esfuerzo. La actualización se ejecuta en segundo plano después del inicio, por lo que no retrasa el inicio, pero cada sesión repite los intentos fallidos y cada operación de git puede esperar el [tiempo de espera de 120 segundos](#git-operations-time-out).

**Solución**: Establezca `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1` para omitir el intento de re-clonado y mantener el uso del caché existente cuando la extracción falla:

```bash theme={null}
export CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1
```

Con esta variable establecida, Claude Code retiene el clon obsoleto del marketplace en fallo de `git pull` y continúa usando el último estado conocido bueno. Para implementaciones completamente sin conexión donde el repositorio nunca será alcanzable, use [`CLAUDE_CODE_PLUGIN_SEED_DIR`](#pre-populate-plugins-for-containers) para precargar el directorio de plugins en tiempo de compilación en su lugar.

<h3 id="git-operations-time-out">
  Las operaciones de Git agotan el tiempo de espera
</h3>

**Síntomas**: La instalación del plugin o las actualizaciones del marketplace fallan con un error de tiempo de espera como "Git clone timed out after 120s" o "Git pull timed out after 120s".

**Causa**: Claude Code usa un tiempo de espera de 120 segundos para todas las operaciones de git, incluida la clonación de repositorios de plugins y la extracción de actualizaciones de marketplace. Los repositorios grandes o las conexiones de red lentas pueden exceder este límite.

**Solución**: Aumente el tiempo de espera usando la variable de entorno `CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS`. El valor está en milisegundos:

```bash theme={null}
export CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS=300000  # 5 minutos
```

<h3 id="plugins-with-relative-paths-fail-in-url-based-marketplaces">
  Los plugins con rutas relativas fallan en marketplaces basados en URL
</h3>

**Síntomas**: Agregó un marketplace a través de URL (como `https://example.com/marketplace.json`), pero los plugins con fuentes de ruta relativa como `"./plugins/my-plugin"` fallan al instalar con errores "path not found".

**Causa**: Los marketplaces basados en URL solo descargan el archivo `marketplace.json` en sí. No descargan archivos de plugins del servidor. Las rutas relativas en la entrada del marketplace hacen referencia a archivos en el servidor remoto que no fueron descargados.

**Soluciones**:

* **Use fuentes externas**: Cambie las entradas de plugins para usar fuentes de GitHub, npm o URL de git en lugar de rutas relativas:
  ```json theme={null}
  { "name": "my-plugin", "source": { "source": "github", "repo": "owner/repo" } }
  ```
* **Use un marketplace basado en Git**: Aloje su marketplace en un repositorio de Git y agréguelo con la URL de git. Los marketplaces basados en Git clonan el repositorio completo, haciendo que las rutas relativas funcionen correctamente.

<h3 id="files-not-found-after-installation">
  Archivos no encontrados después de la instalación
</h3>

**Síntomas**: El plugin se instala pero las referencias a archivos fallan, especialmente archivos fuera del directorio del plugin

**Causa**: Los plugins se copian a un directorio de caché en lugar de usarse en el lugar. Las rutas que hacen referencia a archivos fuera del directorio del plugin (como `../shared-utils`) no funcionarán porque esos archivos no se copian.

**Soluciones**: Consulte [Plugin caching and file resolution](/docs/es/plugins-reference#plugin-caching-and-file-resolution) para soluciones alternativas incluyendo enlaces simbólicos y reestructuración de directorios.

Para herramientas de depuración adicionales y problemas comunes, consulte [Debugging and development tools](/docs/es/plugins-reference#debugging-and-development-tools).

<h2 id="see-also">
  Ver también
</h2>

* [Descubrir e instalar plugins precompilados](/docs/es/discover-plugins) - Instalación de plugins desde marketplaces existentes
* [Plugins](/docs/es/plugins) - Creación de sus propios plugins
* [Referencia de plugins](/docs/es/plugins-reference) - Especificaciones técnicas completas y esquemas
* [Configuración de plugins](/docs/es/settings#plugin-settings) - Opciones de configuración de plugins
* [Referencia de strictKnownMarketplaces](/docs/es/settings#strictknownmarketplaces) - Restricciones de marketplace administrado
