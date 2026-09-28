> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Crear un marketplace

> Cree un marketplace de plugins a partir de un archivo marketplace.json y pruébelo localmente antes de alojarlo.

Un marketplace de plugins es un directorio o repositorio con un archivo `.claude-plugin/marketplace.json` que enumera sus plugins e indica dónde obtener cada uno. Usted envía el directorio a un host de git, y cualquiera que tenga acceso lo registra en Claude Code con un comando e instala sus plugins desde él.

Cree su propio marketplace cuando desee que un grupo que usted elija, como su equipo u organización, instale sus plugins y continúe recibiendo sus actualizaciones desde un catálogo que usted controla. El repositorio puede ser privado, puede enumerar tantos plugins como desee, y un administrador puede [requerirlo en cada máquina](/docs/es/plugins/org).

<Note>
  Estos casos se tratan en otras páginas:

  * **Compartir un plugin con algunas personas**: envíeles el directorio del plugin o un `.zip` del mismo. Consulte [Compartir un plugin sin un marketplace](/docs/es/plugins/publish#share-a-plugin-without-a-marketplace).
  * **Ofrecer un plugin a todos**: envíelo al marketplace comunitario de Anthropic. Consulte [Enviar al marketplace comunitario](/docs/es/plugins/publish#submit-to-the-community-marketplace).
  * **Usar un plugin usted mismo**: cárguelo con `--plugin-dir` o guárdelo en su directorio de skills. Consulte [Desarrollar sin un marketplace](/docs/es/plugins/create#develop-without-a-marketplace).
</Note>

Comience con [Crear un marketplace](#create-a-marketplace) para crear uno en su propia máquina e instalar un plugin desde él, luego [agregue más entradas de plugins](#add-plugin-entries).

<h2 id="create-a-marketplace">
  Crear un marketplace
</h2>

Los siguientes pasos crean un marketplace en su máquina, agregan un plugin a él, lo registran en Claude Code e instalan el plugin desde él. Este es el ciclo completo, y es el mismo ciclo por el que pasan sus usuarios una vez que aloja el marketplace en algún lugar al que puedan acceder. Ejecute cada comando en su shell, desde el directorio donde desea que se cree `my-marketplace/`.

Necesita un plugin para enumerar. El ejemplo utiliza `my-first-plugin` de [Crear su primer plugin](/docs/es/plugins/create#create-your-first-plugin), un plugin con una skill que ejecuta como `/my-first-plugin:hello`; constrúyalo primero si aún no tiene un plugin. Para usar un plugin propio en su lugar, sustituya su directorio y su `name` dondequiera que los pasos digan `my-first-plugin`. Para saber qué puede contener un directorio de plugins, consulte el [explorador de directorios de plugins](/docs/es/plugins/components#explore-the-plugin-directory).

<Steps>
  <Step title="Configurar el directorio del marketplace">
    Un marketplace es un directorio con un archivo `.claude-plugin/marketplace.json`, más los plugins que enumera. Cree el directorio del marketplace y su carpeta `.claude-plugin/`, luego copie su plugin bajo `plugins/`:

    ```bash theme={null}
    mkdir -p my-marketplace/.claude-plugin my-marketplace/plugins
    cp -r my-first-plugin my-marketplace/plugins/
    ```

    Verifique que el plugin sea válido donde ahora se encuentra, para que cualquier error posterior sea sobre el marketplace y no sobre el plugin:

    ```bash theme={null}
    claude plugin validate ./my-marketplace/plugins/my-first-plugin
    ```

    La última línea de la salida dice `✔ Validation passed`.
  </Step>

  <Step title="Crear el archivo del marketplace">
    Guarde `marketplace.json` en `my-marketplace/.claude-plugin/marketplace.json`. El archivo requiere un `name`, un `owner` y un array `plugins`.

    Cada objeto en `plugins` es una entrada de plugin y necesita un `name` y una `source`. Escriba la `source` de la entrada como una ruta desde la raíz del marketplace. La raíz es `my-marketplace/`, el directorio que contiene `.claude-plugin/`.

    ```json my-marketplace/.claude-plugin/marketplace.json theme={null}
    {
      "name": "my-marketplace",
      "description": "Plugins for my team",
      "owner": {
        "name": "Your Name"
      },
      "plugins": [
        {
          "name": "my-first-plugin",
          "source": "./plugins/my-first-plugin",
          "description": "A greeting plugin to learn the basics"
        }
      ]
    }
    ```
  </Step>

  <Step title="Validar el marketplace">
    Ejecute `claude plugin validate` en el directorio del marketplace para verificar la sintaxis JSON, los campos requeridos y cada entrada de plugin en su `.claude-plugin/marketplace.json`.

    ```bash theme={null}
    claude plugin validate ./my-marketplace
    ```

    Para el archivo tal como se escribió en el paso 2, la última línea de la salida dice `✔ Validation passed`.
  </Step>

  <Step title="Agregar el marketplace e instalar el plugin">
    Registre el directorio como un marketplace.

    ```bash theme={null}
    claude plugin marketplace add ./my-marketplace
    ```

    El comando imprime `✔ Successfully added marketplace: my-marketplace (declared in user settings)`, lo que significa que el marketplace se registra en su archivo de configuración de usuario.

    Instale el plugin. El id de instalación es el `name` de la entrada, una `@` y el `name` del marketplace.

    ```bash theme={null}
    claude plugin install my-first-plugin@my-marketplace
    ```

    El comando imprime `✔ Successfully installed plugin: my-first-plugin@my-marketplace (scope: user)`.

    Dentro de una sesión, `/plugin marketplace add ./my-marketplace` registra el marketplace de la misma manera. `/plugin install my-first-plugin@my-marketplace` abre los detalles del plugin en el panel `/plugin`, donde lo instala. Para ese flujo, consulte [Instalar y administrar plugins](/docs/es/plugins/install).
  </Step>

  <Step title="Confirmar que el plugin se cargó">
    Enumere los plugins instalados.

    ```bash theme={null}
    claude plugin list
    ```

    La salida enumera `my-first-plugin@my-marketplace` con `Status: ✔ enabled`.

    Para ver qué cargó el plugin, muestre sus detalles.

    ```bash theme={null}
    claude plugin details my-first-plugin
    ```

    La sección `Component inventory` dice `Skills (1)  hello`.

    Para ejecutar la skill, inicie una sesión e ingrese `/my-first-plugin:hello`. Claude lo saluda. El comando tiene el nombre del plugin como prefijo, como lo hace el nombre de cada skill de plugin.
  </Step>
</Steps>

<h2 id="add-plugin-entries">
  Agregar entradas de plugins
</h2>

Cada plugin que distribuya es un objeto en el array `plugins` de `marketplace.json`. Para agregar un segundo plugin, agregue un segundo objeto. Estos campos cubren la mayoría de entradas:

* `name`: el identificador que las personas escriben antes de `@` cuando instalan. No puede contener espacios.
* `source`: dónde Claude Code obtiene el plugin. Escriba una cadena de ruta relativa para un plugin dentro del directorio del marketplace, como en [el tutorial](#create-a-marketplace), o un objeto de source para un plugin fuera de él. Consulte [Elegir una source de plugin](#choose-a-plugin-source).
* `description`: la línea que las personas ven junto al plugin cuando exploran su marketplace en `/plugin`.

Para la lista completa de campos, consulte [Entradas de plugins](/docs/es/plugins/marketplace-reference#plugin-entries).

Una entrada también puede establecer cualquier campo de [`plugin.json`](/docs/es/plugins/manifest-reference). Para saber cuándo se aplican los campos `plugin.json` de una entrada a un plugin que tiene su propio `plugin.json`, consulte [Entrada y plugin.json](/docs/es/plugins/marketplace-reference#entry-and-plugin-json).

<h2 id="rules-for-plugin-entries">
  Reglas para entradas de plugins
</h2>

La mayoría de las instalaciones fallidas desde un nuevo marketplace provienen de una ruta relativa escrita desde el directorio incorrecto, o de un nombre de entrada que difiere del `name` en el `plugin.json` del plugin.

<h3 id="write-relative-paths-from-the-marketplace-root">
  Escribir rutas relativas desde la raíz del marketplace
</h3>

La raíz del marketplace es el directorio que contiene `.claude-plugin/`. En [el tutorial](#create-a-marketplace), eso es `my-marketplace/`, por lo que la `source` de la entrada es `"./plugins/my-first-plugin"`. La ruta no comienza dentro de `.claude-plugin/`, así que no use `..` para salir de él.

Una ruta con `..` y una ruta a un directorio faltante fallan en comandos diferentes:

* **Una ruta con `..`**: `claude plugin validate` reporta la entrada como inválida. El mensaje comienza `Path contains "..": ./../plugins/my-first-plugin`.
* **Una ruta a un directorio que no existe**: `claude plugin validate` pasa. `claude plugin install` falla con `Source path does not exist: <path>`, y `<path>` es la ubicación absoluta que Claude Code verificó.

<h3 id="keep-the-entry-name-and-the-manifest-name-the-same">
  Mantener el nombre de la entrada y el nombre del manifest iguales
</h3>

Un plugin de marketplace tiene un `name` de entrada en `marketplace.json` y un `name` en su propio `plugin.json`, llamado el nombre del manifest. Cada nombre aparece en lugares diferentes:

* **Nombre de entrada**: el id de instalación, `<entry-name>@<marketplace>`. Es lo que las personas escriben para instalar, lo que `claude plugin list` muestra, y la clave que Claude Code escribe bajo [`enabledPlugins`](/docs/es/settings-reference#enabledplugins) en su archivo de configuración.
* **Nombre del manifest**: el prefijo en las skills del plugin, y el nombre que `claude plugin details` toma.

Cuando los dos nombres difieren y alguien instala por el nombre del manifest, Claude Code reporta `Plugin "<manifest-name>" not found in marketplace "<marketplace>"`. Mantenga los dos nombres iguales. Para más información sobre cómo Claude Code usa los dos nombres, consulte [Referencia de carga de plugins](/docs/es/plugins/loading#find-where-a-plugin-came-from).

<h2 id="choose-a-plugin-source">
  Elegir una source de plugin
</h2>

Cada entrada de plugin en `marketplace.json` tiene una `source` que le dice a Claude Code dónde obtener ese plugin. Elija la source según dónde se almacenan los archivos del plugin. La tabla enumera las sources que la mayoría de los propietarios de marketplace utilizan.

| Source        | Úsela cuando                                                              | Valor mínimo de `source`                                                                  |
| :------------ | :------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------- |
| Ruta relativa | Los archivos del plugin están dentro del directorio del marketplace mismo | `"./plugins/my-first-plugin"`                                                             |
| `github`      | El plugin es su propio repositorio de GitHub                              | `{ "source": "github", "repo": "your-org/my-first-plugin" }`                              |
| `git-subdir`  | El plugin es un subdirectorio de algún otro repositorio, como un monorepo | `{ "source": "git-subdir", "url": "your-org/monorepo", "path": "tools/my-first-plugin" }` |

En una source `git-subdir`, `url` toma una URL de git o una abreviatura de GitHub `owner/repo`.

Un plugin también puede provenir de uno de estos tipos de source:

* `url`: un repositorio de git por URL, en cualquier host
* `archive`: un archivo zip descargado sobre HTTPS
* `npm`: un paquete npm
* `command`: un directorio producido al ejecutar un comando en la máquina donde se instala el plugin

Para los campos de cada tipo de source, y para fijar una source basada en git a una `ref` o `sha`, consulte [Plugin sources](/docs/es/plugins/marketplace-reference#plugin-sources).

<h2 id="validate-and-test">
  Validar y probar
</h2>

A medida que agregue plugins, ejecute `claude plugin validate ./my-marketplace` en su shell después de cada edición, e instale desde el marketplace en su propia máquina antes de compartirlo. La validación y la instalación detectan problemas diferentes.

<h3 id="problems-that-validation-reports">
  Problemas que la validación reporta
</h3>

`claude plugin validate` lee solo archivos dentro del directorio del marketplace. Reporta:

* Errores de sintaxis JSON, como `json: Invalid JSON syntax: <reason>`
* Campos requeridos faltantes, como `owner: Invalid input`
* Un nombre de marketplace con espacios, caracteres no ASCII, o una forma que imita un marketplace oficial de Anthropic, como `claude-official`
* Una `source` relativa que contiene `..`
* Campos desconocidos en el nivel superior o en una entrada de plugin, como advertencias
* Problemas en el `plugin.json` de cada plugin de ruta relativa, como `plugins[N] plugin.json → <field>: <message>`

Para cada mensaje que `validate` puede imprimir, consulte [Mensajes de validación](/docs/es/plugins/marketplace-reference#validation-messages). Para sus banderas y códigos de salida, consulte [`plugin validate`](/docs/es/plugins/cli-reference#plugin-validate).

<h3 id="problems-that-surface-when-you-add-or-install">
  Problemas que aparecen cuando agrega o instala
</h3>

Los problemas que `claude plugin validate` no reporta aparecen cuando agrega el marketplace o instala desde él:

* **Cuando agrega el marketplace**: los [nombres de marketplace oficiales](/docs/es/plugins/marketplace-reference#reserved-names) exactos, como `claude-plugins-official`, pasan la validación. Cuando agrega un marketplace con uno de esos nombres, Claude Code lo rechaza con un mensaje que comienza `The name '<name>' is reserved for official Anthropic marketplaces`.
* **Cuando instala un plugin**:
  * Claude Code primero obtiene una source `github`, `git-subdir` u otra remota cuando instala el plugin, por lo que un `repo` o `path` incorrecto aparece entonces.
  * Una `source` relativa cuyo directorio no existe también falla en la instalación, con `Source path does not exist: <path>`.

<h3 id="test-an-edit-to-a-plugin">
  Probar una edición a un plugin
</h3>

En [el tutorial](#create-a-marketplace), agregó `my-marketplace` desde un directorio local con una `source` de ruta relativa. Con esa configuración, Claude Code lee los archivos del plugin directamente desde `my-marketplace/plugins/`. Sus ediciones surten efecto al iniciar la siguiente sesión o cuando ejecuta `/reload-plugins` en una sesión, sin cambio en la `version` del plugin.

Las personas que instalan desde su marketplace alojado obtienen una copia en el caché de plugins en su lugar. Para saber cómo reciben una nueva versión, consulte [Mantener a los usuarios actualizados](/docs/es/plugins/host-marketplace#keep-users-up-to-date).

<h3 id="remove-the-marketplace-to-start-over">
  Eliminar el marketplace para empezar de nuevo
</h3>

Para eliminar todo y empezar de nuevo, ejecute `claude plugin marketplace remove my-marketplace` en su shell. El comando elimina el marketplace y desinstala sus plugins.

<h2 id="host-your-marketplace">
  Alojar su marketplace
</h2>

Una vez que pueda instalar un plugin desde el marketplace en su propia máquina, como en [Crear un marketplace](#create-a-marketplace), envíe el directorio del marketplace a un host de git.

Sus compañeros de equipo ejecutan `claude plugin marketplace add <owner>/<repo>` en su shell para un repositorio de GitHub, o el mismo comando con la URL del repositorio. Luego instalan un plugin por nombre como en [el tutorial](#create-a-marketplace).

Para acceso a repositorio privado, actualizaciones, versionado, y cambio de nombre o eliminación de entradas, consulte [Alojar y mantener un marketplace](/docs/es/plugins/host-marketplace).

<h2 id="next-steps">
  Próximos pasos
</h2>

* [Alojar y mantener un marketplace](/docs/es/plugins/host-marketplace): elija un host, mantenga a los usuarios actualizados, y cambie el nombre o elimine plugins de forma segura
* [Referencia de marketplace](/docs/es/plugins/marketplace-reference): campos de `marketplace.json` y tipos de source
* [Administrar plugins para su organización](/docs/es/plugins/org): requiera su marketplace y sus plugins en cada máquina
* [Sugerir plugins por relevancia](/docs/es/plugins/relevance): haga que Claude Code sugiera un plugin de su marketplace cuando una sesión coincida
