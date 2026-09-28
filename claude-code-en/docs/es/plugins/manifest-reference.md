> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referencia del manifiesto de plugins

> Referencia completa de plugin.json: cada campo con su tipo y valor predeterminado, formas de ruta aceptadas, y los esquemas de userConfig y variables de entorno.

Un manifiesto de plugin es el archivo `plugin.json` en el directorio `.claude-plugin/` de un plugin. Contiene los metadatos del plugin y los valores de [`userConfig`](#user-configuration) que Claude Code solicita al usuario. También declara cualquier componente que defina en línea o que mantenga fuera de su [ubicación predeterminada](#standard-layout).

Esta referencia es para creadores de plugins y para propietarios de mercados que colocan campos de componentes en una entrada del mercado.

<Note>
  Estos casos se tratan en otras páginas:

  * **Aprender a crear un plugin**: comience con [Crear un plugin](/docs/es/plugins/create)
  * **Qué hace cada componente en tiempo de ejecución**: consulte [Componentes de plugins](/docs/es/plugins/components)
</Note>

Comience en la sección que coincida con lo que está buscando:

* Un campo: la [tabla de campos](#fields) proporciona el tipo de cada campo, si es obligatorio, su valor predeterminado y qué acepta. [Reglas de ruta](#path-rules) cubre el prefijo `./` y la contención para cada ruta de componente
* Una opción `userConfig` o una entrada `channels`: los esquemas de [Configuración del usuario](#user-configuration) y [Canales](#channels)
* `${CLAUDE_PLUGIN_ROOT}` u otra variable que un plugin pueda referenciar: [Variables de entorno](#environment-variables)
* Dónde van los archivos de cada componente: [Diseño estándar](#standard-layout)
* Un mensaje de `claude plugin validate`: la [página de solución de problemas](/docs/es/plugins/troubleshooting) enumera cada mensaje con su solución y enlaces a las secciones relevantes en esta página

<h2 id="manifest-file">
  Archivo de manifiesto
</h2>

El manifiesto es opcional. Sin él, Claude Code carga los componentes que encuentra en el [diseño estándar](#standard-layout). El nombre del plugin proviene de la entrada del mercado o del nombre del directorio cuando carga el plugin con `--plugin-dir`.

Escriba un manifiesto cuando desee metadatos, un componente fuera de su directorio predeterminado, `userConfig`, o una definición de componente en línea.

Guarde el manifiesto en `.claude-plugin/plugin.json` bajo la raíz del plugin. Coloque todos los demás archivos del plugin en la raíz del plugin, no dentro de `.claude-plugin/`. Esto incluye `skills/`, `commands/` y `hooks/`.

El siguiente ejemplo establece la mayoría de las claves en la [tabla de campos](#fields). Pasa la validación en un directorio de plugin que contiene cada ruta referenciada.

```json theme={null}
{
  "name": "deploy-tools",
  "displayName": "Deploy Tools",
  "version": "1.2.0",
  "description": "Deployment commands, a review agent, and a status monitor",
  "author": {
    "name": "Example Team",
    "email": "dev@example.com",
    "url": "https://example.com"
  },
  "homepage": "https://example.com/docs/deploy-tools",
  "repository": "https://github.com/example/deploy-tools",
  "license": "MIT",
  "keywords": ["deployment", "ci"],
  "defaultEnabled": true,
  "dependencies": ["secrets-vault"],
  "metadata": { "catalogId": "cat-123" },
  "skills": ["./extra-skills/"],
  "commands": {
    "status": {
      "source": "./commands/status.md",
      "description": "Show the current deployment status"
    },
    "about": {
      "content": "Explain what the deploy-tools plugin provides.",
      "description": "Describe this plugin"
    }
  },
  "agents": ["./agents/reviewer.md"],
  "hooks": "./config/extra-hooks.json",
  "mcpServers": {
    "deploy-api": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"]
    }
  },
  "lspServers": "./.lsp.json",
  "outputStyles": "./styles/",
  "experimental": {
    "themes": "./themes/",
    "monitors": "./config/monitors.json"
  },
  "userConfig": {
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "Token for the deployment API",
      "sensitive": true
    }
  }
}
```

<h3 id="unrecognized-fields">
  Campos no reconocidos
</h3>

Una clave de nivel superior no reconocida se elimina, y una clave no reconocida dentro de una opción `userConfig`, entrada `channels`, configuración `lspServers` o entrada `monitors` se rechaza:

* **Campos de nivel superior**: el campo se elimina y el plugin se carga. `claude plugin validate` reporta cada campo de nivel superior no reconocido como una advertencia
* **Objetos estrictos**: las opciones `userConfig`, entradas `channels`, configuraciones `lspServers` y entradas `monitors` son estrictas. Una clave desconocida dentro de una es un error, y el plugin no se carga

<h3 id="validate-the-manifest">
  Validar el manifiesto
</h3>

`claude plugin validate` es la verificación autorizada para un manifiesto. Ejecútelo desde su shell contra el directorio del plugin:

```bash theme={null}
claude plugin validate ./my-plugin
```

El comando reporta uno de estos resultados:

* **`Validation passed`**: el manifiesto se carga
* **`Validation passed with warnings`**: el manifiesto se carga, pero el validador encontró algo que corregir, como un campo de nivel superior desconocido que Claude Code elimina, un `name` que no está en kebab-case, o un `version`, `description` o `author` faltante. Pase `--strict` para convertir advertencias en fallos en CI
* **`Validation failed`**: el manifiesto tiene una discrepancia de tipo, una ruta que falta o escapa de la raíz del plugin, o una clave desconocida dentro de una opción `userConfig`, entrada `channels`, configuración `lspServers` o entrada `monitors`. Claude Code reporta el mismo problema cuando carga el plugin

<h2 id="fields">
  Campos
</h2>

La tabla enumera las claves de nivel superior en `plugin.json`. `name` es la única clave obligatoria. Donde un nombre de campo es un enlace, la sección vinculada tiene sus reglas completas.

Para claves de componentes como `commands` y `hooks`, [Formas de ruta de componente](#component-path-forms) muestra cada forma aceptada con un ejemplo, y cada ruta sigue las [reglas de ruta](#path-rules) para el prefijo `./`, extensiones y contención.

| Campo                                | Tipo                             | Descripción                                                                                                                                                                                                                                                                                                                                             |
| :----------------------------------- | :------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `$schema`                            | String                           | URL de JSON Schema para autocompletado del editor. Claude Code lo ignora en tiempo de carga                                                                                                                                                                                                                                                             |
| [`name`](#name)                      | String                           | Identificador del plugin, obligatorio. Use kebab-case. Cada componente se espacía bajo él                                                                                                                                                                                                                                                               |
| [`displayName`](#displayname)        | String                           | Nombre mostrado en la UI en lugar de `name`                                                                                                                                                                                                                                                                                                             |
| [`version`](#version)                | String                           | Cadena de versión. Configurarla mantiene a los usuarios en esa versión hasta que la cambie                                                                                                                                                                                                                                                              |
| `description`                        | String                           | Explicación breve de lo que proporciona el plugin                                                                                                                                                                                                                                                                                                       |
| `author`                             | Object                           | `name`, que es obligatorio, más `email` y `url` opcionales                                                                                                                                                                                                                                                                                              |
| `homepage`                           | String                           | URL de documentación. Debe analizarse como una URL, o el plugin no se carga                                                                                                                                                                                                                                                                             |
| `repository`                         | String                           | URL del repositorio de fuentes. No se valida                                                                                                                                                                                                                                                                                                            |
| `license`                            | String                           | Identificador SPDX como `MIT` o `Apache-2.0`                                                                                                                                                                                                                                                                                                            |
| `keywords`                           | Array of strings                 | Etiquetas de descubrimiento                                                                                                                                                                                                                                                                                                                             |
| [`metadata`](#metadata)              | Object                           | Objeto de forma libre para sus propios datos. Claude Code no lo lee                                                                                                                                                                                                                                                                                     |
| [`defaultEnabled`](#defaultenabled)  | Boolean                          | Si el plugin comienza habilitado cuando el usuario no lo ha configurado. Por defecto es `true`                                                                                                                                                                                                                                                          |
| [`dependencies`](#dependencies)      | Array of strings or objects      | Plugins que deben estar habilitados para que este funcione                                                                                                                                                                                                                                                                                              |
| [`settings`](#settings)              | Object                           | Configuración que Claude Code aplica mientras el plugin está habilitado. Solo `agent` y `subagentStatusLine` tienen efecto                                                                                                                                                                                                                              |
| [`userConfig`](#user-configuration)  | Object                           | Valores que Claude Code solicita al usuario cuando el plugin está habilitado                                                                                                                                                                                                                                                                            |
| [`channels`](#channels)              | Array of objects                 | Canales de mensajes que proporciona el plugin, cada uno vinculado a uno de sus servidores MCP                                                                                                                                                                                                                                                           |
| `skills`                             | Path, or array of paths          | Directorios para escanear en busca de skills, cada uno un directorio de carpetas `<name>/SKILL.md` o una carpeta que contenga `SKILL.md` directamente. `"."` nombra la raíz del plugin. Se suma al escaneo predeterminado `skills/`                                                                                                                     |
| [`commands`](#commands)              | Path, array of paths, or object  | Archivos de comando `.md` planos, directorios de ellos, u un objeto mapa de nombre de comando a `source` o `content`. Reemplaza el escaneo predeterminado `commands/`                                                                                                                                                                                   |
| `agents`                             | Path, or array of paths          | Archivos de agente `.md`. Los directorios no se aceptan. Reemplaza el escaneo predeterminado `agents/`                                                                                                                                                                                                                                                  |
| [`hooks`](#hooks)                    | Path, object, or array of either | Archivos hook `.json` o configuración de hook en línea. Se cargan junto con `hooks/hooks.json`                                                                                                                                                                                                                                                          |
| [`mcpServers`](#mcpservers)          | Path, object, or array of either | Archivos de configuración MCP `.json`, bundles `.mcpb` o `.dxt`, o configuraciones de servidor en línea con clave de nombre. Se cargan junto con `.mcp.json`; un nombre de servidor declarado después reemplaza uno anterior                                                                                                                            |
| [`lspServers`](#lspservers)          | Path, object, or array of either | Archivos de configuración LSP `.json` o configuraciones de servidor en línea con clave de nombre. Se cargan junto con `.lsp.json`                                                                                                                                                                                                                       |
| `outputStyles`                       | Path, or array of paths          | Archivos de estilo de salida o directorios. Reemplaza el escaneo predeterminado `output-styles/`                                                                                                                                                                                                                                                        |
| `workflows`                          | Path, or array of paths          | Archivos [Workflow](/docs/es/workflows#distribute-a-workflow-in-a-plugin) `.js` o directorios. Reemplaza el escaneo predeterminado `workflows/`                                                                                                                                                                                                              |
| `experimental`                       | Object                           | Contenedor para `themes`, `monitors` y `evals`, cuya forma de manifiesto aún puede cambiar                                                                                                                                                                                                                                                              |
| `experimental.themes`                | Path, or array of paths          | Archivos de tema o directorios. Reemplaza el escaneo predeterminado `themes/`. Una clave `themes` de nivel superior aún se carga, con una advertencia `claude plugin validate`                                                                                                                                                                          |
| [`experimental.monitors`](#monitors) | Path, or inline array            | Un archivo `.json` que contiene el array de monitores, o el array en sí. Por defecto es `monitors/monitors.json`. Una clave `monitors` de nivel superior aún se carga, con una advertencia `claude plugin validate`. Los monitores se ejecutan solo en sesiones interactivas, y no en Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry |
| `experimental.evals`                 | Path, or array of paths          | Directorio que contiene los [casos de evaluación](/docs/es/plugin-evals#use-a-different-eval-directory) del plugin cuando no es el predeterminado `evals/`. `claude plugin eval --eval-dir` lo anula                                                                                                                                                         |

En la columna Tipo, una ruta es una cadena relativa a la raíz del plugin, como `"./custom/commands"`.

<h3 id="name">
  `name`
</h3>

El identificador del plugin. Debe ser no vacío, sin espacios, `@`, `:`, separadores de ruta, caracteres de control o caracteres de formato bidireccional; use kebab-case.

Claude Code espacía cada componente bajo él, por lo que un agente `reviewer` en el plugin `deploy-tools` aparece como `deploy-tools:reviewer`.

<h3 id="displayname">
  `displayName`
</h3>

El nombre mostrado en la UI en lugar de `name`. Puede contener espacios y cualquier mayúscula, y no se usa para espaciado o búsqueda.

Para un plugin instalado desde el mercado, un `displayName` en la [entrada del mercado](/docs/es/plugins/marketplace-reference#plugin-entries) tiene precedencia sobre este valor.

<h3 id="version">
  `version`
</h3>

Una cadena de versión, no se verifica contra semver. Configurarla fija el plugin a esa versión hasta que la cambie; consulte [Versiones y actualizaciones](/docs/es/plugins/loading#versions-and-updates). Un plugin con una [`command` source](/docs/es/plugins/marketplace-reference), un plugin de un [mercado alojado en claude.ai](/docs/es/plugins/install#add-from-claude-ai), y un plugin [cargado en su lugar](/docs/es/plugins/loading#find-plugins-on-disk) desde un mercado agregado como directorio local no se fijan por este campo.

<h3 id="metadata">
  `metadata`
</h3>

Un objeto de forma libre para sus propios datos, como campos de catálogo o derechos. Claude Code no lo lee. Requiere Claude Code v2.1.222 o posterior.

<h3 id="defaultenabled">
  `defaultEnabled`
</h3>

Si el plugin comienza habilitado cuando el usuario no lo ha configurado en [`enabledPlugins`](/docs/es/settings-reference#enabledplugins). Por defecto es `true`. Un plugin del que depende un plugin habilitado comienza habilitado independientemente. El mismo campo en la entrada del mercado anula este.

Una vez que se escribe la entrada `enabledPlugins` de un usuario, persiste en las actualizaciones del plugin, por lo que cambiar `defaultEnabled` en una versión posterior no cambia la configuración para un usuario existente.

<h3 id="dependencies">
  `dependencies`
</h3>

Plugins que deben estar habilitados para que este funcione. Cada entrada es `"name"`, `"name@marketplace"`, o `{ "name": "...", "marketplace": "...", "version": "..." }`. Los nombres simples se resuelven contra el propio mercado de este plugin. Consulte [restricciones de dependencia](/docs/es/plugins/dependencies).

<h3 id="settings">
  `settings`
</h3>

Configuración que Claude Code aplica mientras el plugin está habilitado. Solo `agent` y `subagentStatusLine` tienen efecto; otras claves se descartan en la carga. Un `settings.json` en la raíz del plugin tiene precedencia sobre esta clave. Consulte [Configuración predeterminada](/docs/es/plugins/components#default-settings).

<h2 id="component-path-forms">
  Formas de ruta de componente
</h2>

Cada clave de componente acepta una ruta relativa a la raíz del plugin. `hooks`, `mcpServers`, `lspServers` y `experimental.monitors` también aceptan configuración en línea, `commands` también acepta un objeto mapa, y `mcpServers` también acepta rutas de bundle MCP y URLs. Los ejemplos que siguen muestran cada forma aceptada una vez. Para qué hace cada componente en tiempo de ejecución, consulte [Componentes de plugins](/docs/es/plugins/components).

<h3 id="path-only-fields">
  Campos solo de ruta
</h3>

`agents`, `skills`, `outputStyles`, `workflows` y `experimental.themes` toman una ruta o un array de rutas. Las entradas `agents` deben ser archivos `.md`, y las entradas `skills` deben ser directorios. Los otros tres aceptan un directorio o un archivo.

```json theme={null}
{
  "agents": ["./custom-agents/reviewer.md", "./custom-agents/tester.md"],
  "skills": ["./extra-skills/", "."],
  "outputStyles": "./styles/"
}
```

<h3 id="commands">
  `commands`
</h3>

`commands` toma una ruta, un array de rutas, u un objeto mapa. Una ruta nombra un archivo de comando `.md` plano o un directorio. En el objeto mapa, cada clave se convierte en el nombre del comando después del prefijo del plugin. Por ejemplo, `"about"` en el plugin `deploy-tools` se ejecuta como `/deploy-tools:about`.

Cada valor establece exactamente uno de `source` o `content`, y una entrada que establece ambos o ninguno falla la validación. Los otros campos en esta tabla son opcionales:

| Campo          | Tipo             | Descripción                                                                    |
| :------------- | :--------------- | :----------------------------------------------------------------------------- |
| `source`       | string           | Ruta al archivo Markdown del comando, relativa a la raíz del plugin            |
| `content`      | string           | Markdown en línea para el cuerpo del comando, en lugar de `source`             |
| `description`  | string           | Descripción mostrada para el comando                                           |
| `argumentHint` | string           | Sugerencia de argumento mostrada después del nombre del comando, como `[file]` |
| `model`        | string           | Modelo predeterminado para el comando                                          |
| `allowedTools` | array of strings | Herramientas que el comando puede usar sin solicitar                           |

Este mapa declara un comando de un archivo y uno de contenido en línea:

```json theme={null}
{
  "commands": {
    "status": { "source": "./commands/status.md", "argumentHint": "[env]" },
    "about": { "content": "Explain what this plugin provides." }
  }
}
```

<h3 id="hooks">
  `hooks`
</h3>

`hooks` toma una ruta de archivo `.json`, un objeto hooks en línea en la misma forma que [`hooks` en `settings.json`](/docs/es/hooks#configuration), o un array que mezcla ambos. Para eventos de hook y campos de controlador, consulte la [referencia de hooks](/docs/es/hooks#hook-events).

Claude Code fusiona lo que declare con `hooks/hooks.json` cuando ese archivo existe.

```json theme={null}
{
  "hooks": [
    "./config/extra-hooks.json",
    {
      "PostToolUse": [
        {
          "matcher": "Write|Edit",
          "hooks": [
            { "type": "command", "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/format.sh" }
          ]
        }
      ]
    }
  ]
}
```

<h3 id="mcpservers">
  `mcpServers`
</h3>

`mcpServers` toma una ruta de archivo `.json`, una ruta de bundle MCP o URL, un mapa en línea, o un array que mezcla ellos. Para campos de configuración del servidor, consulte [servidores MCP proporcionados por plugin](/docs/es/mcp#plugin-provided-mcp-servers).

Claude Code carga `.mcp.json` en la raíz del plugin primero, luego cada forma declarada en orden. Un nombre de servidor declarado después reemplaza uno anterior.

Un valor `mcpServers` toma una de estas formas:

| Forma                   | Valor de ejemplo                                                                       | Qué hace Claude Code                                                                                           |
| :---------------------- | :------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------- |
| Ruta de archivo `.json` | `"./mcp/servers.json"`                                                                 | Lee el archivo como un mapa `mcpServers`                                                                       |
| Ruta de bundle MCP      | `"./bundle.mcpb"`                                                                      | Extrae el bundle `.mcpb` o `.dxt` en `.mcpb-cache/` bajo la raíz del plugin y lee su configuración de servidor |
| URL de bundle MCP       | `"https://example.com/server.mcpb"`                                                    | Descarga el bundle en `.mcpb-cache/`, luego lo lee                                                             |
| Mapa en línea           | `{ "deploy-api": { "command": "node", "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"] } }` | Usa el mapa como configuraciones de servidor con clave de nombre                                               |

Una ruta de bundle o URL debe terminar en `.mcpb` o `.dxt`. Cualquier otra extensión falla la validación.

<h3 id="lspservers">
  `lspServers`
</h3>

`lspServers` toma una ruta de archivo `.json`, un mapa en línea de nombre de servidor a configuración, o un array de cualquiera.

Claude Code carga `.lsp.json` en la raíz del plugin primero, luego cada configuración declarada en orden. Un nombre de servidor declarado después reemplaza uno anterior.

Cada configuración de servidor es un objeto estricto con estos campos. Una clave desconocida falla la validación.

| Campo                   | Obligatorio | Descripción                                                                                                                                                                                                       |
| :---------------------- | :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`               | Yes         | Binario del servidor de lenguaje. Sin espacios a menos que el valor comience con `/`; coloque argumentos en `args`                                                                                                |
| `extensionToLanguage`   | Yes         | Mapa de extensión de archivo a ID de lenguaje LSP, al menos una entrada. Las claves comienzan con un punto, como `".go"`                                                                                          |
| `args`                  | No          | Argumentos pasados al servidor                                                                                                                                                                                    |
| `transport`             | No          | Transporte de comunicación: `stdio` (predeterminado) o `socket`. Claude Code acepta `socket` pero ejecuta cada servidor sobre stdio, por lo que las reglas del protocolo stdout se aplican a todos los servidores |
| `env`                   | No          | Variables de entorno para el proceso del servidor                                                                                                                                                                 |
| `initializationOptions` | No          | Opciones enviadas en la solicitud de inicialización                                                                                                                                                               |
| `settings`              | No          | Configuración enviada por `workspace/didChangeConfiguration`                                                                                                                                                      |
| `workspaceFolder`       | No          | Ruta de carpeta de espacio de trabajo para el servidor                                                                                                                                                            |
| `startupTimeout`        | No          | Milisegundos para esperar el inicio, un entero positivo                                                                                                                                                           |
| `shutdownTimeout`       | No          | Milisegundos para esperar un apagado elegante, un entero positivo. Cuando se agota el tiempo de espera, Claude Code termina el proceso del servidor. Cuando no se establece, no se aplica tiempo de espera        |
| `restartOnCrash`        | No          | Si reiniciar el servidor después de que se bloquee. Por defecto es `true`. Establezca en `false` para dejar un servidor bloqueado detenido en lugar de reiniciarlo                                                |
| `maxRestarts`           | No          | Intentos de reinicio antes de rendirse, cero o más                                                                                                                                                                |
| `diagnostics`           | No          | Si insertar diagnósticos en contexto después de ediciones. Por defecto es `true`                                                                                                                                  |

Esta configuración en línea ejecuta `gopls` para archivos `.go`:

```json theme={null}
{
  "lspServers": {
    "go": {
      "command": "gopls",
      "args": ["serve"],
      "extensionToLanguage": { ".go": "go" }
    }
  }
}
```

Para los servidores de lenguaje que Anthropic publica como plugins y cómo se comportan los servidores en tiempo de ejecución, consulte [Inteligencia de código](/docs/es/plugins/code-intelligence).

<h3 id="monitors">
  `monitors`
</h3>

`experimental.monitors` toma una ruta de archivo `.json` o el array en línea. Cuando omite la clave, Claude Code carga `monitors/monitors.json` si existe.

Cada entrada es un objeto estricto con estos campos.

| Campo         | Obligatorio | Descripción                                                                                                                                                                                 |
| :------------ | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`        | Yes         | Identificador único dentro del plugin                                                                                                                                                       |
| `command`     | Yes         | Comando de shell que Claude Code ejecuta como un proceso de fondo persistente en el directorio de trabajo de la sesión                                                                      |
| `description` | Yes         | Resumen breve mostrado en el panel de tareas y resúmenes de notificaciones                                                                                                                  |
| `when`        | No          | Con `"always"`, el predeterminado, el monitor comienza al inicio de la sesión y en la recarga del plugin. Con `"on-skill-invoke:<skill>"`, comienza la primera vez que se ejecuta esa skill |

Este array en línea declara un monitor que comienza la primera vez que se ejecuta la skill `deploy`:

```json theme={null}
{
  "experimental": {
    "monitors": [
      {
        "name": "deploy-status",
        "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/poll-deploy.sh",
        "description": "Deployment status changes",
        "when": "on-skill-invoke:deploy"
      }
    ]
  }
}
```

Un `command` de monitor no puede referenciar `${user_config.*}`. Consulte [Campos que se ejecutan a través de un shell](#fields-that-run-through-a-shell).

<h2 id="path-rules">
  Reglas de ruta
</h2>

Cada ruta de componente en un manifiesto es relativa a la raíz del plugin y debe comenzar con `./`. Una ruta como `commands/foo.md` falla la validación. `skills` y `mcpServers` cada uno aceptan una forma fuera de esa regla:

* **`skills`**: también acepta `"."`. Tanto `"."` como `"./"` denotan la raíz del plugin. Antes de v2.1.221, `"."` fallaba la validación del manifiesto, así que use `"./"` cuando el plugin deba cargarse en versiones anteriores
* **`mcpServers`**: también acepta una URL de bundle `https://`

<h3 id="containment-and-existence">
  Contención y existencia
</h3>

Cada ruta de componente debe resolverse dentro de la raíz del plugin y debe existir. `claude plugin validate` no verifica las rutas `outputStyles`, `lspServers`, `monitors` o `themes`, por lo que una ruta incorrecta en esos campos falla solo cuando el plugin se carga:

* **Contención**: una ruta que se resuelve fuera de la raíz del plugin no se carga, y la pestaña **Errors** de `/plugin` muestra `<component> path escapes plugin directory: <path>`. Una ruta que contiene `..` es el caso usual, y `claude plugin validate` la reporta como `Path contains ".." which could be a path traversal attempt`
* **Existencia**: una ruta que no existe no se carga, y la pestaña **Errors** de `/plugin` muestra `<component> path not found: <path>`. `claude plugin validate` la reporta como `Path not found`

<h3 id="how-each-key-combines-with-its-default-location">
  Cómo cada clave se combina con su ubicación predeterminada
</h3>

Cada clave de componente reemplaza su ubicación predeterminada, se suma a ella, o se fusiona con ella:

* **Reemplaza el predeterminado**: `commands`, `agents`, `outputStyles`, `workflows`, `experimental.themes`, `experimental.monitors`. Cuando establece `commands`, el directorio predeterminado `commands/` no se escanea. Para mantener el predeterminado y agregar más, enumérelo explícitamente: `"commands": ["./commands/", "./extras/"]`
* **Se suma al predeterminado**: `skills`. El directorio `skills/` aún se escanea, y los directorios enumerados se cargan junto a él
* **Se fusiona**: `hooks`, `mcpServers`, `lspServers`. El archivo predeterminado se carga primero, y lo que declara el manifiesto se fusiona en él, como se describe en [Formas de ruta de componente](#component-path-forms)

Si un plugin tiene una carpeta predeterminada como `commands/` y también establece la clave de manifiesto que la reemplaza, Claude Code carga las rutas del manifiesto y no la carpeta. `claude plugin list` y la interfaz `/plugin` entonces muestran la advertencia `Default <folder>/ folder is ignored because the manifest sets "<key>"`.

Para evitar la advertencia, establezca la clave en una ruta dentro de esa carpeta: `"commands": ["./commands/deploy.md"]` nombra un archivo en la carpeta predeterminada y no produce advertencia.

<h2 id="user-configuration">
  Configuración del usuario
</h2>

`userConfig` declara valores que Claude Code solicita al usuario cuando el plugin está habilitado, por lo que los usuarios no editan `settings.json` ellos mismos.

Las claves son identificadores hechos de letras, dígitos y guiones bajos, y no pueden comenzar con un dígito.

Cada valor es un objeto estricto con estos campos. Una clave desconocida falla la validación.

| Campo         | Obligatorio | Descripción                                                                                                                                                                                                       |
| :------------ | :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`        | Yes         | Uno de `string`, `number`, `boolean`, `directory` o `file`                                                                                                                                                        |
| `title`       | Yes         | Etiqueta mostrada en el diálogo de configuración                                                                                                                                                                  |
| `description` | Yes         | Texto de ayuda mostrado debajo del campo                                                                                                                                                                          |
| `required`    | No          | Si `true`, el diálogo de configuración no acepta un valor vacío                                                                                                                                                   |
| `default`     | No          | Valor usado cuando el usuario no proporciona nada: una cadena, número, booleano, o array de cadenas                                                                                                               |
| `options`     | No          | Para `string`, los valores que el campo acepta, mostrados como un selector en `/config`. Consulte [Limitar un campo a opciones fijas](#limit-a-field-to-fixed-options). Requiere Claude Code v2.1.271 o posterior |
| `multiple`    | No          | Para `string`, permite un array de cadenas                                                                                                                                                                        |
| `sensitive`   | No          | Si `true`, enmascara la entrada y almacena el valor en almacenamiento seguro en lugar de `settings.json`                                                                                                          |
| `min` / `max` | No          | Límites para `number`                                                                                                                                                                                             |

Cada opción de cada plugin habilitado también aparece como una fila en el panel `/config`, excepto opciones `sensitive` y listas `multiple`. Las filas `/config` requieren Claude Code v2.1.269 o posterior.

Este `userConfig` declara un punto final y un token enmascarado:

```json theme={null}
{
  "userConfig": {
    "api_endpoint": {
      "type": "string",
      "title": "API endpoint",
      "description": "Your team's API endpoint"
    },
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "API authentication token",
      "sensitive": true
    }
  }
}
```

<h3 id="limit-a-field-to-fixed-options">
  Limitar un campo a opciones fijas
</h3>

Establezca `options` en un campo `userConfig` para que los usuarios elijan su valor de una lista fija.

Para limitar un campo `tone` a tres opciones, enumérelas en `options` y establezca `default` en una de ellas:

```json theme={null}
{
  "userConfig": {
    "tone": {
      "type": "string",
      "title": "Tone",
      "description": "Voice for generated replies",
      "options": ["neutral", "warm", "formal"],
      "default": "neutral"
    }
  }
}
```

Si declara `options` en cualquier campo, los usuarios en versiones de Claude Code anteriores a v2.1.271 no pueden cargar el plugin.

`options` se aplica a un campo `string` que no es `multiple` o `sensitive`. Establezca `default` en uno de los valores enumerados, o establezca `required: true` para que el usuario deba elegir uno. Cada opción es una etiqueta simple de 1 a 64 caracteres, y `claude plugin validate`, que ejecuta en su shell, reporta cualquier otra cosa que rechace. Un plugin cuyas `options` rompan estas reglas no se carga.

<h3 id="where-values-are-stored">
  Dónde se almacenan los valores
</h3>

Los valores no sensibles se guardan bajo [`pluginConfigs`](/docs/es/settings-reference#pluginconfigs) en el `settings.json` del usuario. Los valores sensibles van al almacén de credenciales seguro de la plataforma en su lugar. La [página de configuración](/docs/es/settings-reference#pluginconfigs) enumera qué archivos de configuración se leen desde `pluginConfigs`.

<h3 id="reference-a-saved-value">
  Referenciar un valor guardado
</h3>

Referencie un valor guardado donde el plugin lo necesite, en una de dos formas:

* **`${user_config.KEY}`**: sustituido en configuración de servidor MCP, configuración de servidor LSP, [forma exec](/docs/es/hooks#exec-form-and-shell-form) hook `args`, y contenido de skill y agente. En contenido de skill y agente, solo se sustituyen valores no sensibles, y un valor sensible allí se convierte en un marcador de posición
* **`CLAUDE_PLUGIN_OPTION_<KEY>`**: exportado a procesos de hook para cada opción, con `<KEY>` en mayúsculas. Un hook de forma shell lee `$CLAUDE_PLUGIN_OPTION_API_TOKEN` para `api_token`

<h3 id="fields-that-run-through-a-shell">
  Campos que se ejecutan a través de un shell
</h3>

Los comandos de hook de forma shell, comandos de monitor y MCP [`headersHelper`](/docs/es/mcp#use-dynamic-headers-for-custom-authentication) rechazan `${user_config.*}`. Un componente que lo referencia en uno de estos campos falla con un [error](/docs/es/errors#plugin-command-references-user-config) en lugar de ejecutarse, porque el valor del campo se pasa a un shell que volvería a analizar el valor sustituido.

La tabla muestra cómo el valor puede llegar a cada uno de estos campos en su lugar.

| Campo                           | Cómo el valor puede llegar a él                                                                                                                                                                                                                     |
| :------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Comandos de hook de forma shell | Use [forma exec](/docs/es/hooks#exec-form-and-shell-form) con `args`, o lea `CLAUDE_PLUGIN_OPTION_<KEY>` del entorno del hook                                                                                                                            |
| Comandos de monitor             | No a través de Claude Code. Los procesos de monitor no reciben `CLAUDE_PLUGIN_OPTION_<KEY>`, por lo que el script de monitor tiene que obtener el valor por su cuenta                                                                               |
| MCP `headersHelper`             | No a través de Claude Code. El entorno del ayudante lleva `CLAUDE_PLUGIN_ROOT`, `CLAUDE_CODE_MCP_SERVER_NAME` y `CLAUDE_CODE_MCP_SERVER_URL` pero sin valores de opción, por lo que el script del ayudante tiene que obtener el valor por su cuenta |

<h2 id="channels">
  Canales
</h2>

`channels` declara los canales de mensajes que proporciona un plugin, como un puente a una aplicación de chat. Cuando declara uno, Claude Code puede solicitar la configuración del canal cuando el plugin está habilitado. Para cómo el servidor inyecta mensajes, consulte la [referencia de canales](/docs/es/channels-reference#package-as-a-plugin).

Cada entrada es un objeto estricto vinculado a uno de los servidores MCP del plugin, con estos campos:

| Campo         | Obligatorio | Descripción                                                                                                                                                                                    |
| :------------ | :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `server`      | Yes         | Clave del servidor MCP en `mcpServers` de este plugin al que se vincula el canal                                                                                                               |
| `displayName` | No          | Nombre mostrado en el título del diálogo de configuración. Por defecto es el nombre del servidor                                                                                               |
| `userConfig`  | No          | Opciones para solicitar, en la misma forma que [top-level `userConfig`](#user-configuration). Los valores guardados se sustituyen en referencias `${user_config.KEY}` en el `env` del servidor |

Este manifiesto vincula un canal al servidor MCP `telegram` del plugin y solicita un token de bot que se sustituye en el `env` del servidor:

```json theme={null}
{
  "mcpServers": {
    "telegram": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"],
      "env": { "BOT_TOKEN": "${user_config.bot_token}" }
    }
  },
  "channels": [
    {
      "server": "telegram",
      "displayName": "Telegram",
      "userConfig": {
        "bot_token": {
          "type": "string",
          "title": "Bot token",
          "description": "Telegram bot token",
          "sensitive": true
        }
      }
    }
  ]
}
```

<h2 id="environment-variables">
  Variables de entorno
</h2>

Claude Code proporciona tres variables de ruta a componentes de plugin. Referenciarlas como `${NAME}` en los campos enumerados en [Dónde se resuelve cada variable](#where-each-variable-resolves), y léalas como variables de entorno en los procesos que las reciben.

| Variable                | Se resuelve a                                                                                                                                                                                                                 | Úsela para                                                            |
| :---------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------- |
| `${CLAUDE_PLUGIN_ROOT}` | Ruta absoluta de la versión instalada del plugin                                                                                                                                                                              | Scripts, binarios y archivos de configuración incluidos con el plugin |
| `${CLAUDE_PLUGIN_DATA}` | `~/.claude/plugins/data/<id>/`, creado en la primera referencia y mantenido en actualizaciones de plugin. `<id>` es el identificador del plugin con cada carácter que no sea una letra, dígito, `_` o `-` reemplazado por `-` | Dependencias instaladas como `node_modules`, código generado y cachés |
| `${CLAUDE_PROJECT_DIR}` | La raíz del proyecto                                                                                                                                                                                                          | Scripts y archivos de configuración locales del proyecto              |

`${CLAUDE_PLUGIN_ROOT}` cambia cuando el plugin se actualiza, así que no escriba estado allí. Para dónde se mueve la raíz y cuándo se limpia el directorio antiguo, consulte la [página de carga](/docs/es/plugins/loading).

Cuando desinstala el plugin del último lugar donde está instalado, el directorio `${CLAUDE_PLUGIN_DATA}` se elimina a menos que pase [`--keep-data`](/docs/es/plugins/cli-reference).

<h3 id="where-each-variable-resolves">
  Dónde se resuelve cada variable
</h3>

En cada componente de plugin, las referencias `${...}` se resuelven en línea en campos específicos, y algunos componentes también reciben las variables en su entorno de proceso:

| Componente de plugin                 | Campos donde `${...}` se resuelve           | Exportado al proceso                                                                            |
| :----------------------------------- | :------------------------------------------ | :---------------------------------------------------------------------------------------------- |
| Comandos de hook                     | En cualquier lugar en `command` y `args`    | `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`, `CLAUDE_PROJECT_DIR` y `CLAUDE_PLUGIN_OPTION_<KEY>` |
| Comandos de monitor                  | En cualquier lugar en `command`             | No exportado                                                                                    |
| Servidores MCP `stdio`               | `command`, `args`, `env`                    | `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`                                                      |
| Servidores MCP `http`, `sse`, `ws`   | `url`, `headers`, `headersHelper`           | No aplicable                                                                                    |
| Servidores LSP                       | `command`, `args`, `env`, `workspaceFolder` | `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`, `CLAUDE_PROJECT_DIR`                                |
| Contenido de skill, comando y agente | En cualquier lugar en el cuerpo Markdown    | No aplicable                                                                                    |

Las variables no están presentes en el entorno de comandos que Claude ejecuta a través de la herramienta Bash, en la sesión principal o en un subagente. En contenido de skill, comando y agente, escriba la referencia `${...}` en el cuerpo Markdown en su lugar, y Claude Code sustituye la ruta en línea cuando carga el contenido.

<h3 id="quoting-and-path-separators">
  Entrecomillado y separadores de ruta
</h3>

Mantenga cada ruta sustituida como un argumento único:

* **Comandos de hook**: use [forma exec](/docs/es/hooks#exec-form-and-shell-form) con `args` para que cada ruta sea un argumento sin entrecomillado
* **Hooks de forma shell y comandos de monitor**: envuelva la variable en comillas dobles para que una ruta con espacios permanezca como una palabra

Este hook de forma shell ejecuta un script incluido con el plugin:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/process.sh"
          }
        ]
      }
    ]
  }
}
```

En Windows, las rutas sustituidas usan barras diagonales para que un shell no lea barras invertidas como escapes.

<h2 id="standard-layout">
  Diseño estándar
</h2>

Cada tipo de componente tiene una ubicación predeterminada bajo la raíz del plugin, usada cuando el manifiesto no apunta a otro lugar.

| Componente        | Ubicación predeterminada     | Contenidos                                                                                                                                                                                                                                                                                                                                                                              |
| :---------------- | :--------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Manifiesto        | `.claude-plugin/plugin.json` | Metadatos y configuración del plugin. Opcional                                                                                                                                                                                                                                                                                                                                          |
| Skills            | `skills/`                    | Un `<name>/SKILL.md` por skill. Un plugin con `SKILL.md` en su raíz, sin `skills/`, y sin clave `skills` se carga como una skill única                                                                                                                                                                                                                                                  |
| Comandos          | `commands/`                  | Archivos de comando Markdown planos. Prefiera `skills/` para nuevos plugins                                                                                                                                                                                                                                                                                                             |
| Agentes           | `agents/`                    | Archivos Markdown de agente. Las subcarpetas son parte del [nombre del agente](/docs/es/plugins/components#agents)                                                                                                                                                                                                                                                                           |
| Hooks             | `hooks/hooks.json`           | Configuración de hook                                                                                                                                                                                                                                                                                                                                                                   |
| Servidores MCP    | `.mcp.json`                  | Definiciones de servidor MCP                                                                                                                                                                                                                                                                                                                                                            |
| Servidores LSP    | `.lsp.json`                  | Configuraciones de servidor LSP                                                                                                                                                                                                                                                                                                                                                         |
| Estilos de salida | `output-styles/`             | Archivos de estilo de salida Markdown                                                                                                                                                                                                                                                                                                                                                   |
| Workflows         | `workflows/`                 | Archivos Workflow `.js`                                                                                                                                                                                                                                                                                                                                                                 |
| Temas             | `themes/`                    | Archivos de tema JSON                                                                                                                                                                                                                                                                                                                                                                   |
| Monitores         | `monitors/monitors.json`     | El array de monitores                                                                                                                                                                                                                                                                                                                                                                   |
| Ejecutables       | `bin/`                       | Los archivos aquí están en el `PATH` de la herramienta Bash mientras el plugin está habilitado, por lo que Claude los ejecuta como comandos simples. claude.ai y Cowork no instalan un plugin que tenga este directorio, incluido uno que [distribuya a través de la configuración de la organización claude.ai](/docs/es/plugins/host-marketplace#distribute-through-organization-settings) |
| Configuración     | `settings.json`              | Valores predeterminados `agent` y `subagentStatusLine` aplicados mientras el plugin está habilitado                                                                                                                                                                                                                                                                                     |

Un plugin que usa cada ubicación predeterminada, más una carpeta `scripts/` que sus hooks llaman, se distribuye así:

```text theme={null}
deploy-tools/
├── .claude-plugin/
│   └── plugin.json
├── skills/
│   └── deploy/
│       └── SKILL.md
├── commands/
│   └── status.md
├── agents/
│   └── reviewer.md
├── hooks/
│   └── hooks.json
├── monitors/
│   └── monitors.json
├── output-styles/
│   └── terse.md
├── themes/
│   └── dracula.json
├── workflows/
│   └── release-audit.js
├── bin/
│   └── deploy-tool
├── scripts/
│   └── format.sh
├── settings.json
├── .mcp.json
└── .lsp.json
```

Para hacer clic en este diseño y leer qué hace cada archivo, abra el [explorador de plugins](/docs/es/plugins/components#explore-the-plugin-directory).

Un `CLAUDE.md` en la raíz del plugin no se carga como contexto, y `claude plugin validate` advierte cuando encuentra uno. Para incluir instrucciones que se carguen en el contexto de Claude, colóquelas en una skill.

<h2 id="marketplace-entries-and-the-manifest">
  Entradas del mercado y el manifiesto
</h2>

Una [entrada del mercado](/docs/es/plugins/marketplace-reference) acepta cada campo en esta página junto con [sus propios campos](/docs/es/plugins/marketplace-reference#plugin-entries), incluido `strict`.

El campo `strict` decide si la entrada puede agregar componentes a un plugin que tiene su propio `plugin.json`. Por defecto es `true`.

<h3 id="how-entry-fields-combine-with-plugin-json">
  Cómo se combinan los campos de entrada con `plugin.json`
</h3>

La entrada sirve como manifiesto, agrega componentes a él, o entra en conflicto con él:

* **Sin `plugin.json`**: la entrada es el manifiesto, independientemente de `strict`. Los `hooks` de entrada se cargan solo en la forma de objeto en línea. Para una ruta de archivo o array allí, la pestaña **Errors** de `/plugin` muestra un error `not yet supported in a marketplace entry`
* **`plugin.json` presente, `strict` sin establecer o `true`**: Claude Code carga el manifiesto y agrega los `commands`, `agents`, `skills`, `outputStyles` y `themes` de la entrada a él. Para `hooks`, los matchers de la entrada para un evento reemplazan los matchers del manifiesto para ese mismo evento, y los eventos que solo declara el manifiesto mantienen los suyos
* **`plugin.json` presente, `strict: false`**: una entrada que declara cualquiera de `commands`, `agents`, `skills`, `hooks`, `outputStyles` o `themes` es un conflicto, y el plugin no se carga con `Plugin <name> has conflicting manifests`

Cuando una [entrada del mercado cuya `source` es la raíz del mercado](/docs/es/plugins/marketplace-reference) enumera subdirectorios `skills` específicos, solo se cargan esos subdirectorios, y el directorio predeterminado `skills/` del plugin no se escanea. Una clave `skills` en el manifiesto en su lugar [se suma al predeterminado](#how-each-key-combines-with-its-default-location).

<h3 id="metadata-precedence">
  Precedencia de metadatos
</h3>

Algunos campos de metadatos tienen una precedencia fija independientemente de `strict`:

* **`defaultEnabled` y campos de visualización**: el `defaultEnabled` de la entrada y sus [campos de visualización](/docs/es/plugins/marketplace-reference#entry-and-plugin-json) como `displayName` anulan los del manifiesto
* **`version`**: el `version` del manifiesto anula el de la entrada
* **`name`**: cuando la entrada enumera el plugin bajo un `name` diferente al del manifiesto, `enabledPlugins` usa el nombre de la entrada, y los componentes se espacían bajo el nombre del manifiesto

Para la tabla de precedencia completa, consulte [Modo estricto](/docs/es/plugins/marketplace-reference).

<h2 id="next-steps">
  Próximos pasos
</h2>

* [Agregar componentes a un plugin](/docs/es/plugins/components): qué hace cada componente en tiempo de ejecución, con un ejemplo que valida
* [Referencia del mercado](/docs/es/plugins/marketplace-reference): los campos de entrada que un mercado puede establecer para su plugin
* [Referencia de comandos de plugin](/docs/es/plugins/cli-reference#plugin-validate): banderas de `claude plugin validate` y salida
* [Solucionar problemas de plugins](/docs/es/plugins/troubleshooting#claude-plugin-validate-reports-errors): cada mensaje de validación con su solución
