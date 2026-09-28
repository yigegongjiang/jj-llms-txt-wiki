> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referencia de comandos de plugins

> Referencia completa de los comandos de shell de plugins de Claude, /plugin y /reload-plugins en una sesión, y las banderas que cargan un plugin para una sesión.

Ejecuta comandos de plugins ya sea como `claude plugin` desde tu shell o un script, o como `/plugin` y `/reload-plugins` dentro de una sesión de Claude Code. Esta referencia proporciona las banderas, valores predeterminados, salida y códigos de salida de cada comando, junto con las dos banderas que cargan un plugin para una sesión.

Ejecuta `claude plugin --help` en tu compilación para confirmar qué subcomandos tiene tu versión.

<Note>
  Estos casos se cubren en otras páginas:

  * **Instalar y gestionar pasos, y dónde se ejecuta `/plugin`**: consulta [Instalar y gestionar plugins](/docs/es/plugins/install)
  * **Qué cambia un comando en el disco y qué ámbito tiene precedencia**: consulta [Referencia de carga de plugins](/docs/es/plugins/loading)
  * **Qué significa un mensaje de error**: consulta [Solucionar problemas de plugins](/docs/es/plugins/troubleshooting)
</Note>

<h2 id="claude-plugin-commands">
  Comandos claude plugin
</h2>

Ejecuta `claude plugin <subcommand>` desde tu shell o un script, fuera de una sesión de Claude Code. Estos subcomandos instalan y gestionan plugins sin abrir el panel [`/plugin`](#plugin-in-a-session).

`claude plugins` es un alias para `claude plugin`.

Cada subcomando comparte estos códigos de salida, argumentos de plugins y valores de ámbito:

* **Códigos de salida**: `0` en caso de éxito y `1` en caso de fallo. `validate` añade salida `2` para un error inesperado, y `eval` añade los códigos listados en [su sección](#plugin-eval).
* **Argumentos de plugins**: un argumento `<plugin>` es un `name` de plugin o `name@marketplace`. Cuando dos marketplaces ofrecen el mismo nombre, usa la forma calificada.
* **Ámbitos**: `--scope` toma `user`, `project` o `local`, y nombra el archivo de configuración al que escribe el comando. `update` también toma `managed`.

<h3 id="plugin-init">
  plugin init
</h3>

Crea un nuevo plugin en `~/.claude/skills/<name>/`. Se carga en tu próxima sesión como `<name>@skills-dir` sin necesidad de un paso de instalación.

`new` es un alias para `init`.

Para el flujo de trabajo de crear, probar y editar que comienza con este comando, consulta [Crear un plugin](/docs/es/plugins/create).

```bash theme={null}
claude plugin init <name> [options]
```

`<name>` se convierte en el nombre del directorio bajo `~/.claude/skills/` y el `name` del plugin en su manifiesto.

El comando no tiene una bandera para otra ubicación. Para crear un scaffold dentro de un proyecto, consulta [Crear un plugin](/docs/es/plugins/create).

| Bandera                  | Descripción                                                                                                |
| :----------------------- | :--------------------------------------------------------------------------------------------------------- |
| `--description <text>`   | Descripción del manifiesto                                                                                 |
| `--author <name>`        | Nombre del autor. Por defecto es `git config user.name`                                                    |
| `--author-email <email>` | Correo electrónico del autor. Por defecto es `git config user.email`                                       |
| `--with <components...>` | También crea archivos de inicio para `skills`, `agents`, `hooks`, `mcp`, `lsp`, `output-style` o `channel` |
| `-f, --force`            | Sobrescribe un `.claude-plugin/` existente en el destino                                                   |

Crea un plugin con archivos de skill y hook de inicio:

```bash theme={null}
claude plugin init my-helper --with skills hooks
```

Claude Code valida lo que escribió e imprime `Created plugin "my-helper" at ~/.claude/skills/my-helper`, seguido del id con el que se carga y el comando `claude plugin disable` que lo desactiva.

Claude Code sale con `1` sin escribir cuando no puede crear un scaffold de forma segura, y el mensaje nombra la razón. Estas son razones comunes:

* Un valor desconocido de `--with`
* Un scaffold existente en el destino sin `--force`
* Una configuración gestionada que bloquea plugins de directorio de skills

<h3 id="plugin-install">
  plugin install
</h3>

Instala un plugin desde un marketplace que hayas añadido. `i` es un alias para `install`.

```bash theme={null}
claude plugin install <plugin> [options]
```

La mayoría de los plugins se instalan sin un aviso. Para un plugin cuya entrada de marketplace [ejecuta un comando para instalarlo](/docs/es/plugins/host-marketplace) o [establece un `headersHelper` para su descarga](/docs/es/plugins/host-marketplace#how-users-accept-a-headershelper-command), Claude Code primero imprime el comando y pregunta `Run this command now? [y/N]`.

| Bandera                     | Descripción                                                                                                                                                                                                                                                                                                                         |
| :-------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>`       | Ámbito de instalación: `user`, `project` o `local`. Por defecto es `user`                                                                                                                                                                                                                                                           |
| `--config <key=value>`      | Establece una opción [`userConfig`](/docs/es/plugins/manifest-reference) que declara el manifiesto del plugin. Repite la bandera para cada opción. Requiere Claude Code v2.1.147 o posterior                                                                                                                                             |
| `-y, --yes`                 | Acepta el comando de instalación mostrado sin el aviso `Run this command now?`. Se ignora cuando el comando se ejecuta dentro de una sesión de Claude Code, como desde la herramienta Bash o un hook. Requiere Claude Code v2.1.229 o posterior                                                                                     |
| `--accept-command <sha256>` | Acepta el comando de instalación mostrado cuyo `sha256` una ejecución anterior de [`--json`](#plugin-json-result) reportó en `shownCommand`, en lugar de `-y`. No se puede combinar con `-y`. Consulta [Aceptar un comando de instalación mostrado](#accept-a-displayed-install-command). Requiere Claude Code v2.1.271 o posterior |
| `--json`                    | Imprime el resultado como un objeto JSON en la última línea de stdout en lugar del mensaje legible por humanos, para usar en scripts. Consulta [Formato de resultado JSON](#plugin-json-result). Requiere Claude Code v2.1.268 o posterior                                                                                          |

Pasa `-y` desde tu propia terminal para aceptar el comando mostrado sin el aviso. Aquí está lo que sucede sin un TTY y cuando Claude ejecuta el comando:

* **stdin o stdout no es un TTY, y no pasas ni `-y` ni `--accept-command`**: la instalación se rechaza. La salida dice que el comando solo se mostró, y el código de salida es `1`
* **Claude ejecuta el comando a través de su herramienta Bash**: `-y` se ignora. Ejecuta el comando desde tu propia terminal en su lugar

Instala un plugin para todos los que clonan el proyecto:

```bash theme={null}
claude plugin install formatter@my-marketplace --scope project
```

Claude Code imprime `Successfully installed plugin: formatter@my-marketplace (scope: project)`. Cuando nada nuevo se instala, la salida dice por qué:

* **Ya instalado en ese ámbito**: la salida es `Plugin "formatter@my-marketplace" is already installed (scope: project)` y el código de salida es `0`
* **Rechazas un aviso de origen de comando**: la salida es `Aborted.` y el código de salida es `1`
* **Rechazas un aviso de `headersHelper`, o no se puede confirmar sin un TTY**: la salida es `Aborted — the command was not run.` y el código de salida es `1`

<h4 id="plugin-json-result">
  Formato de resultado JSON
</h4>

Cuando pasas `--json` a `plugin install`, la última línea de stdout es un objeto JSON. Analiza solo esa línea, porque Claude Code imprime cualquier comando que declare el marketplace antes de ella.

Tres campos siempre están presentes:

* `command`: el subcomando que se ejecutó, como `install`
* `outcome`: `ok` o `failed`
* `message`: una descripción legible por humanos del resultado

Otros campos, como `pluginId`, `scope` y `failureCode`, aparecen solo cuando aplican.

Un error de uso, como un `--scope` inválido, no imprime ninguna línea de resultado y sale con `1` con la razón en stderr.

<h4 id="accept-a-displayed-install-command">
  Aceptar un comando de instalación mostrado
</h4>

Cuando una ejecución de `--json` muestra un comando declarado por el marketplace y no lo ejecuta, el resultado `failed` también lleva un objeto `shownCommand`. Sus campos incluyen el comando tal como se mostró, el plugin al que pertenece, y el `sha256` del comando.

Para aceptar exactamente ese comando, vuelve a ejecutar con ese `sha256` como `--accept-command` desde tu propia terminal, porque la bandera no tiene efecto dentro de una sesión de Claude Code. Requiere Claude Code v2.1.271 o posterior.

El `sha256` cuenta como aceptación para exactamente ese comando, plugin y catálogo de marketplace. Si alguno de ellos cambió desde que se mostró el comando, Claude Code no acepta el `sha256` y muestra el comando nuevamente. Un cambio que la actualización del marketplace de la propia ejecución obtiene también cuenta como tal cambio.

Si `shownCommand.acceptCommandMatched` es `false`, el `sha256` que pasaste no coincide con el comando ahora mostrado. Revisa ese comando antes de volver a ejecutar con su `sha256`.

<h3 id="plugin-uninstall">
  plugin uninstall
</h3>

Elimina un plugin instalado de un ámbito. `remove` y `rm` son alias para `uninstall`.

```bash theme={null}
claude plugin uninstall <plugin> [options]
```

| Bandera               | Descripción                                                                                                                                                                                                                  |
| :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>` | Desinstala del ámbito: `user`, `project` o `local`. Por defecto es `user`                                                                                                                                                    |
| `--keep-data`         | Preserva el directorio de datos persistentes del plugin, `~/.claude/plugins/data/<id>/`                                                                                                                                      |
| `--prune`             | También elimina [dependencias](/docs/es/plugins/dependencies) auto-instaladas que ningún plugin restante necesita                                                                                                                 |
| `-y, --yes`           | Omite el aviso de confirmación de `--prune`. Requerido con `--prune` cuando stdin o stdout no es un TTY                                                                                                                      |
| `--json`              | Imprime el resultado como un objeto JSON en la última línea de stdout, en el [mismo formato que `plugin install --json`](#plugin-json-result). No se puede combinar con `--prune`. Requiere Claude Code v2.1.268 o posterior |

Desinstala un plugin del ámbito del proyecto:

```bash theme={null}
claude plugin uninstall formatter@my-marketplace --scope project
```

Claude Code imprime `Successfully uninstalled plugin: formatter (scope: project)`. Cuando el plugin no está instalado en ese ámbito, el comando imprime una línea que comienza con `Failed to uninstall plugin "formatter@my-marketplace":` y sale con `1`.

<h3 id="plugin-enable">
  plugin enable
</h3>

Habilita un plugin deshabilitado. Para un [plugin sincronizado desde claude.ai](/docs/es/plugins/loading#synced-plugins), pasa `<name>@synced` como el plugin.

```bash theme={null}
claude plugin enable <plugin> [options]
```

| Bandera               | Descripción                                                                                                                                                                              |
| :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>` | Ámbito para habilitar: `user`, `project` o `local`. Se auto-detecta cuando se omite                                                                                                      |
| `--json`              | Imprime el resultado como un objeto JSON en la última línea de stdout, en el [mismo formato que `plugin install --json`](#plugin-json-result). Requiere Claude Code v2.1.268 o posterior |

Sin `--scope`, el comando verifica tus archivos de configuración en el orden local, proyecto, usuario, y usa el primer ámbito que menciona el plugin.

Si pasas un `--scope` donde el plugin no está declarado, el comando escribe una anulación o falla:

* **Un ámbito que [tiene precedencia](/docs/es/plugins/loading) sobre el que lo declara**: Claude Code escribe una anulación en el ámbito que pasaste. Por ejemplo, `claude plugin disable formatter --scope local` desactiva un plugin habilitado en el proyecto solo para ti
* **Cualquier otro ámbito**: el comando falla con `Plugin "formatter" is installed at project scope, not user. Use --scope project or omit --scope to auto-detect.`

Si el plugin ya está habilitado en el ámbito resuelto, el comando imprime `Plugin "formatter" is already enabled` y sale con `1`. Con `--json`, el resultado tiene `"failureCode": "already_in_goal_state"` y `"alreadyInGoalState": true`, para que un script pueda tratar ese caso como éxito.

Cuando el plugin declara [dependencias](/docs/es/plugins/dependencies), Claude Code también las habilita. El comando falla en estos casos:

* **Una dependencia no está instalada**: enable falla e imprime el comando `claude plugin install` para cada dependencia faltante
* **Una dependencia está bloqueada por la política de plugins de tu organización**: enable falla y nombra la dependencia bloqueada
* **Una dependencia está establecida en `false` en un ámbito con mayor precedencia que el ámbito de destino**: enable falla. Habilita la dependencia en ese ámbito, o pasa `--scope` para escribir allí

Vuelve a habilitar un plugin dondequiera que esté declarado:

```bash theme={null}
claude plugin enable formatter
```

Claude Code imprime `Successfully enabled plugin: formatter (scope: project)`, nombrando el ámbito que detectó.

<h3 id="plugin-disable">
  plugin disable
</h3>

Deshabilita un plugin sin desinstalarlo. Para un [plugin sincronizado desde claude.ai](/docs/es/plugins/loading#synced-plugins), pasa `<name>@synced` como el plugin.

```bash theme={null}
claude plugin disable [plugin] [options]
```

| Bandera               | Descripción                                                                                                                                                                              |
| :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-a, --all`           | Deshabilita cada plugin habilitado. No se puede combinar con un nombre de plugin o `--scope`                                                                                             |
| `-s, --scope <scope>` | Ámbito para deshabilitar: `user`, `project` o `local`. Se auto-detecta cuando se omite                                                                                                   |
| `--json`              | Imprime el resultado como un objeto JSON en la última línea de stdout, en el [mismo formato que `plugin install --json`](#plugin-json-result). Requiere Claude Code v2.1.268 o posterior |

Sin `--scope`, el ámbito se auto-detecta en el mismo orden local, proyecto, usuario que [`plugin enable`](#plugin-enable).

Si no pasas ni un nombre de plugin ni `--all`, Claude Code imprime `Please specify a plugin name or use --all to disable all plugins` y sale con `1`. Deshabilitar un plugin que ya está deshabilitado imprime `Plugin "formatter" is already disabled` y sale con `1`, como [`plugin enable`](#plugin-enable) hace para un plugin ya habilitado.

El comando falla para un plugin que aún es requerido:

* **Otro plugin habilitado [depende de](/docs/es/plugins/dependencies) él**: el comando falla y nombra los dependientes a deshabilitar primero
* **Tu organización lo requiere como un plugin sincronizado**: el comando falla y no guarda nada

Deshabilita un plugin:

```bash theme={null}
claude plugin disable formatter
```

Claude Code imprime `Successfully disabled plugin: formatter (scope: project)`.

<h3 id="plugin-update">
  plugin update
</h3>

Actualiza un plugin a la última versión que su marketplace ofrece. La nueva versión se carga en tu próxima sesión, o después de ejecutar `/reload-plugins` en una en ejecución.

```bash theme={null}
claude plugin update <plugin> [options]
```

| Bandera                     | Descripción                                                                                                                                                                                                                                        |
| :-------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>`       | Ámbito a actualizar: `user`, `project`, `local` o `managed`. Por defecto es el ámbito en el que está instalado el plugin                                                                                                                           |
| `-y, --yes`                 | Acepta un comando de instalación cambiado de un plugin [command-source](/docs/es/plugins/host-marketplace), sin el aviso. Requerido cuando stdin o stdout no es un TTY, a menos que pases `--accept-command`. Requiere Claude Code v2.1.229 o posterior |
| `--accept-command <sha256>` | Acepta el comando declarado por el marketplace cuyo `sha256` una ejecución anterior de [`--json`](#plugin-json-result) reportó en `shownCommand`, en lugar de `-y`. No se puede combinar con `-y`. Requiere Claude Code v2.1.271 o posterior       |
| `--json`                    | Imprime el resultado como un objeto JSON en la última línea de stdout, en el [mismo formato que `plugin install --json`](#plugin-json-result). Requiere Claude Code v2.1.268 o posterior                                                           |

`managed` es el único ámbito que puedes actualizar pero no instalar. Para plugins instalados por administrador, consulta [Gestionar plugins para tu organización](/docs/es/plugins/org).

Actualiza un plugin:

```bash theme={null}
claude plugin update formatter@my-marketplace
```

Claude Code imprime `Checking for updates for plugin "formatter@my-marketplace"…`, luego el resultado. Cuando nada es más nuevo, imprime `formatter is already at the latest version (1.0.0).` y sale con `0`.

Puedes pasar un nombre de plugin simple, que el comando coincide contra tus plugins instalados. Cuando plugins instalados de diferentes marketplaces comparten el nombre, el comando rechaza la actualización y lista los comandos `plugin-name@marketplace-name` calificados a ejecutar en su lugar. Actualizar por nombre simple requiere Claude Code v2.1.246 o posterior.

<h3 id="plugin-list">
  plugin list
</h3>

Lista plugins instalados con su versión, ámbito y estado.

```bash theme={null}
claude plugin list [options]
```

| Bandera       | Descripción                                                                                           |
| :------------ | :---------------------------------------------------------------------------------------------------- |
| `--json`      | Imprime la lista como JSON                                                                            |
| `--available` | También lista plugins que tus marketplaces ofrecen que no has instalado. No tiene efecto sin `--json` |

Claude Code agrupa la salida legible por humanos por cómo se carga cada plugin:

* **`Installed plugins:`**: plugins que instalaste desde un marketplace
* **`Session-only plugins (--plugin-dir / --plugin-url):`**: plugins cargados por esas banderas en el mismo comando, como en `claude --plugin-dir ./my-plugin plugin list`
* **`Skills-directory plugins (.claude/skills/*):`**: plugins que Claude Code encontró en un directorio de skills
* **`Synced from claude.ai`**: [plugins sincronizados desde tu cuenta claude.ai](/docs/es/plugins/loading#synced-plugins)

Sin nada en ningún grupo, Claude Code imprime ``No plugins installed. Use `claude plugin install` to install a plugin.``

<h4 id="json-output">
  Salida JSON
</h4>

Con `--json`, Claude Code imprime un array con un objeto por instalación. Cada objeto lleva los campos a continuación. `id`, `version`, `scope`, `enabled` e `installPath` siempre están presentes, y los otros aparecen solo cuando aplican.

| Campo          | Tipo             | Descripción                                                                                                                                                                                                                                                                   |
| :------------- | :--------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`           | string           | `name@marketplace` para installs, `name@inline` para plugins de solo sesión, `name@skills-dir` para plugins de directorio de skills, `name@synced` para plugins sincronizados desde claude.ai                                                                                 |
| `version`      | string           | Para una instalación de marketplace, la [versión que Claude Code calculó](/docs/es/plugins/loading#versions-and-updates) en la instalación. Para un plugin de solo sesión, directorio de skills o sincronizado, el `version` del manifiesto, o `unknown` cuando no declara ninguno |
| `scope`        | string           | `user`, `project`, `local` o `managed` para installs; `user` o `project` para plugins de directorio de skills; `session` para plugins de solo sesión; `synced` para plugins sincronizados desde claude.ai                                                                     |
| `enabled`      | boolean          | Si el plugin está habilitado en tu configuración fusionada                                                                                                                                                                                                                    |
| `installPath`  | string           | Directorio desde el que se carga el plugin                                                                                                                                                                                                                                    |
| `installedAt`  | string           | Marca de tiempo ISO de la instalación. Solo installs de marketplace                                                                                                                                                                                                           |
| `lastUpdated`  | string           | Marca de tiempo ISO de la última actualización. Solo installs de marketplace                                                                                                                                                                                                  |
| `projectPath`  | string           | Proyecto al que pertenece la instalación. Solo ámbito `project` y `local`                                                                                                                                                                                                     |
| `mcpServers`   | object           | Las definiciones de servidor MCP del plugin, cuando un plugin instalado de marketplace tiene alguno                                                                                                                                                                           |
| `errors`       | array of strings | Errores de carga, cuando el plugin falló al cargar                                                                                                                                                                                                                            |
| `notes`        | array of strings | Advertencias de autoría para un plugin que cargó y funciona                                                                                                                                                                                                                   |
| `errorDetails` | array of objects | Un objeto por entrada de `errors`, dando su `type` de diagnóstico y los nombres a los que se refiere, como el plugin, marketplace, servidor o archivo. Requiere Claude Code v2.1.268 o posterior                                                                              |
| `noteDetails`  | array of objects | Los mismos objetos de detalle para cada entrada de `notes`. Requiere Claude Code v2.1.268 o posterior                                                                                                                                                                         |

Con `--json --available`, Claude Code imprime un objeto en lugar de un array. Su campo `installed` contiene el array de objetos de plugin instalado, y su campo `available` contiene un objeto por plugin de marketplace no instalado con los campos a continuación.

| Campo             | Tipo             | Descripción                                                                                                                              |
| :---------------- | :--------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| `pluginId`        | string           | `name@marketplace`                                                                                                                       |
| `name`            | string           | El nombre del plugin en el marketplace                                                                                                   |
| `marketplaceName` | string           | El marketplace que lo ofrece                                                                                                             |
| `source`          | string or object | La [source](/docs/es/plugins/marketplace-reference) de la entrada del marketplace: una string para una ruta relativa, un objeto de otra forma |
| `description`     | string           | La descripción de la entrada, cuando tiene una                                                                                           |
| `version`         | string           | La versión de la entrada, cuando declara una                                                                                             |
| `installCount`    | number           | Recuento de instalaciones, cuando Claude Code tiene uno para el plugin                                                                   |

<h3 id="plugin-details">
  plugin details
</h3>

Muestra el inventario de componentes de un plugin y su costo de token proyectado.

El plugin debe estar cargado: instalado, encontrado en un directorio de skills, o pasado con `--plugin-dir` o `--plugin-url` en el mismo comando. El `<name>` es un `name` de plugin o `name@marketplace`.

```bash theme={null}
claude plugin details <name>
```

El comando no toma banderas más allá de `--help`.

Muestra lo que contribuye un plugin instalado:

```bash theme={null}
claude plugin details formatter
```

Claude Code imprime el nombre, versión, descripción y fuente del plugin, luego estas secciones:

* **`Component inventory`**: los skills, agents, hooks, servidores MCP y servidores LSP del plugin
* **`Projected token cost`**: los tokens siempre activos que el plugin añade a cada sesión
* **`Per-component (rounded)`**: estimaciones siempre activas y bajo invocación para cada skill, agent y comando. Se omite cuando el plugin no tiene ninguno

Para lo que significan las dos figuras de costo, consulta [Medir costo y uso de plugins](/docs/es/plugins/measure).

Para un plugin que no está cargado, Claude Code imprime ``Plugin "formatter" not found. Run `claude plugin list` to see installed plugins, or pass --plugin-dir <path> to load one from disk.`` y sale con `1`.

<h3 id="plugin-prune">
  plugin prune
</h3>

Elimina [dependencias](/docs/es/plugins/dependencies) auto-instaladas que ningún plugin instalado necesita más. El comando nunca elimina un plugin que instalaste tú mismo. `autoremove` es un alias para `prune`.

```bash theme={null}
claude plugin prune [options]
```

| Bandera               | Descripción                                                                  |
| :-------------------- | :--------------------------------------------------------------------------- |
| `-s, --scope <scope>` | Prune en ámbito: `user`, `project` o `local`. Por defecto es `user`          |
| `--dry-run`           | Lista lo que se eliminaría sin eliminarlo                                    |
| `-y, --yes`           | Omite el aviso de confirmación. Requerido cuando stdin o stdout no es un TTY |

Previsualiza lo que un prune eliminaría:

```bash theme={null}
claude plugin prune --dry-run
```

Claude Code lista las dependencias huérfanas y termina con `(dry run — nothing removed)`. Sin nada a eliminar, imprime una línea que comienza con `Nothing to prune`.

Sin `--dry-run`, el comando elimina las dependencias huérfanas solo después de que confirmes en el aviso o pases `-y`.

El código de salida es `0` sin importar lo que respondas en el aviso.

Lo que `prune` hace depende de si una terminal está adjunta y si pasas `-y`:

| Terminal y banderas             | Qué sucede                                                                                    |
| :------------------------------ | :-------------------------------------------------------------------------------------------- |
| Terminal interactiva, sin `-y`  | Lista las dependencias huérfanas y pregunta `Remove? [y/N]`                                   |
| Cualquier terminal, `-y`        | Las elimina e imprime `Removed N auto-installed plugins: <names>`                             |
| stdin o stdout no TTY, sin `-y` | Imprime la lista y ``Not a TTY — run `claude plugin prune -y` to remove.``, sin eliminar nada |

<h3 id="plugin-eval">
  plugin eval
</h3>

Ejecuta los [casos eval](/docs/es/plugin-evals) de un plugin e informa resultados puntuados. Requiere Claude Code v2.1.269 o posterior.

Cada caso es un prompt más calificadores. Claude Code lo ejecuta varias veces en una sesión aislada con solo el plugin de destino cargado, y por defecto también sin el plugin para que el informe muestre la diferencia.

Consulta [Probar plugins con evals](/docs/es/plugin-evals) para el formato de caso, calificadores, resultados y uso de CI.

```bash theme={null}
claude plugin eval [target] [options]
```

El `target` opcional por defecto es el directorio actual y toma cualquiera de estas formas:

* Un directorio de plugin
* Un archivo único `prompt.md` o `case.yaml`
* Un plugin instalado como `name` o `name@marketplace`
* `name@skills-dir`

Pon el target antes de `--tag`, `--allow-tools` y `--json`. Cada una de estas opciones toma las palabras que la siguen como su valor, por lo que un target escrito después de una de ellas se lee como una etiqueta, un nombre de herramienta o la ruta de salida JSON en su lugar del target.

Esta tabla lista las opciones que la mayoría de ejecuciones usan. Ejecuta `claude plugin eval --help` para el conjunto completo, incluyendo `--case`, `--tag`, `--output-dir`, `--report`, `--allow-real-servers`, `--keep-temp` y `--verbose`.

| Opción                     | Descripción                                                                                                                                                                                   | Por defecto                                                                                                  |
| :------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------- |
| `--runs <n>`               | Ejecuciones por caso en cada [arm](/docs/es/plugin-evals#compare-against-a-no-plugin-baseline)                                                                                                     | El `runs` de cada caso, si no 3                                                                              |
| `-j, --concurrency <n>`    | Sesiones de agente a ejecutar a la vez, 1 a 8. Comparten tu límite de velocidad                                                                                                               | `1`                                                                                                          |
| `--model <model>`          | Modelo para el agente bajo prueba                                                                                                                                                             | El `model` de cada caso, si no `ANTHROPIC_MODEL` si está establecido, si no el predeterminado de Claude Code |
| `--judge-model <model>`    | Modelo para calificadores `llm` y `baseline`                                                                                                                                                  | Un modelo pequeño rápido                                                                                     |
| `--ablation <mode>`        | `none` o `with-without`. Consulta [Comparar contra una línea base sin plugin](/docs/es/plugin-evals#compare-against-a-no-plugin-baseline)                                                          | `with-without` cuando un plugin se resuelve, si no `none`                                                    |
| `--threshold <0..1>`       | Sale con 1 si algún caso puntúa por debajo de esto                                                                                                                                            | `1.0`                                                                                                        |
| `--max-cost-usd <usd>`     | Detente antes de la próxima ejecución una vez que el gasto alcance esto, sale con 2, e informa resultados parciales                                                                           | Sin límite                                                                                                   |
| `--allow-tools <tools...>` | Otorga herramientas más allá del conjunto de solo lectura, como `Bash`, `Write`, `Edit` o `"mcp__plugin_<plugin>_<server>__*"`. Consulta [Otorgar herramientas](/docs/es/plugin-evals#grant-tools) |                                                                                                              |
| `--scaffold`               | Ejecuta el [`scaffold_script`](/docs/es/plugin-evals#add-setup-or-history-with-case-yaml) de cada caso                                                                                             | Apagado                                                                                                      |
| `--trust-plugin`           | Omite el aviso de confianza de primera ejecución, para CI. Consulta [Qué puede acceder una ejecución](/docs/es/plugin-evals#security)                                                              | Apagado                                                                                                      |
| `--mocks <mode>`           | `record` u `off`. Consulta [Simular servidores MCP](/docs/es/plugin-evals#mock-mcp-servers)                                                                                                        | `record`                                                                                                     |
| `--eval-dir <dir>`         | Directorio debajo del plugin que contiene los casos                                                                                                                                           | El `experimental.evals` del manifiesto, si no `evals`                                                        |
| `--json [path]`            | Imprime el [documento de resultado](/docs/es/plugin-evals#json-result) a stdout, o escríbelo en una ruta `.json`                                                                                   |                                                                                                              |
| `--no-publish`             | Mantén el informe HTML local                                                                                                                                                                  |                                                                                                              |

El código de salida informa cómo terminó la ejecución. Para actuar sobre él en un pipeline, consulta [Ejecutar evals en CI](/docs/es/plugin-evals#run-evals-in-ci).

| Código de salida | Significado                                                                |
| :--------------- | :------------------------------------------------------------------------- |
| `0`              | Cada caso cumple el umbral                                                 |
| `1`              | Un caso fallido, un error de carga, o un directorio de plugin no confiable |
| `2`              | Una ejecución parcial                                                      |
| `130`            | Interrumpido                                                               |
| `143`            | Terminado                                                                  |

<h3 id="plugin-eval-init">
  plugin eval init
</h3>

Crea un conjunto eval para el plugin en el directorio actual. Requiere Claude Code v2.1.269 o posterior. Consulta [Crear tu primer conjunto eval](/docs/es/plugin-evals#create-your-first-eval-suite).

```bash theme={null}
claude plugin eval init [name] [options]
```

En una terminal, el comando abre una sesión interactiva de Claude Code para una entrevista de autoría. En la entrevista, Claude hace lo siguiente:

1. Lee el plugin
2. Te pregunta qué debería hacer bien
3. Propone casos y calificadores
4. Escribe los archivos de caso
5. Ejecuta los casos y revisa las calificaciones contigo para verificar que los calificadores puntúen de la forma que lo harías

Con `--bare`, o sin una terminal, el comando escribe una plantilla de caso único en blanco en su lugar. Cuando Claude ejecuta el comando desde dentro de una sesión de Claude Code, el comando imprime las instrucciones de entrevista para que esa sesión siga en su lugar de escribir una plantilla.

El `name` opcional es un nombre de caso. Es requerido con `--bare` o sin una terminal, porque el comando escribe la plantilla en blanco para ese caso. La entrevista no necesita uno.

El comando acepta estas opciones:

| Opción              | Descripción                                                                                                  | Por defecto                                           |
| :------------------ | :----------------------------------------------------------------------------------------------------------- | :---------------------------------------------------- |
| `--bare`            | Escribe un `prompt.md` en blanco y `graders/criteria.md` para `<name>` en su lugar de ejecutar la entrevista |                                                       |
| `-i, --interactive` | Requiere la entrevista. Falla sin una terminal en su lugar de escribir una plantilla                         |                                                       |
| `--eval-dir <dir>`  | Directorio debajo del directorio actual para escribir casos en                                               | El `experimental.evals` del manifiesto, si no `evals` |

<h3 id="plugin-tag">
  plugin tag
</h3>

Crea una etiqueta git anotada nombrada `<name>--v<version>` para una versión de plugin. Antes de etiquetar, el comando verifica que el `plugin.json` del plugin y cualquier entrada de marketplace que lo liste estén de acuerdo en la versión.

Para cuándo etiquetar una versión, consulta [Publicar un plugin](/docs/es/plugins/publish).

```bash theme={null}
claude plugin tag [path] [options]
```

El `[path]` es el directorio del plugin, por defecto el directorio actual. El comando encuentra la entrada de marketplace caminando hacia arriba desde ese directorio a un `.claude-plugin/marketplace.json` que lista el plugin.

| Bandera               | Descripción                                                                                     |
| :-------------------- | :---------------------------------------------------------------------------------------------- |
| `--push`              | Empuja la etiqueta a `--remote` después de crearla                                              |
| `--dry-run`           | Imprime lo que se etiquetaría sin crear la etiqueta                                             |
| `-f, --force`         | Omite los controles de árbol de trabajo sucio y etiqueta-ya-existe                              |
| `-m, --message <msg>` | Mensaje de anotación de etiqueta. `%s` representa la versión. Por defecto es `<name> <version>` |
| `--remote <name>`     | Remoto al que empujar con `--push`. Por defecto es `origin`                                     |

Previsualiza la etiqueta para un plugin en un checkout de marketplace:

```bash theme={null}
claude plugin tag plugins/formatter --dry-run
```

Claude Code imprime el plan:

* El nombre del plugin
* La versión y de qué archivo vino
* La entrada de marketplace coincidente, cuando hay una
* El nombre de la etiqueta
* Los comandos `git tag` y `git push` que ejecutaría

Sin `--dry-run`, Claude Code imprime `Created tag formatter--v1.0.0` y ya sea `Pushed to origin` o el comando push a ejecutar tú mismo. Si el push falla, la etiqueta aún se crea localmente y el comando sale con un error.

El comando sale con `1` e imprime la razón cuando no puede etiquetar de forma segura. Las razones comunes son:

* Sin `version` en `plugin.json` o la entrada de marketplace
* La etiqueta ya existe
* El árbol de trabajo está sucio

<h3 id="plugin-validate">
  plugin validate
</h3>

Valida un manifiesto de plugin, un manifiesto de marketplace, o los skills, agents y comandos en un directorio, y sale con un código en el que un trabajo de CI puede actuar. Para el flujo de trabajo de crear, probar y editar, consulta [Crear un plugin](/docs/es/plugins/create). Para lo que el validador verifica en cada manifiesto, consulta la [referencia de manifiesto de plugin](/docs/es/plugins/manifest-reference) y la [referencia de marketplace](/docs/es/plugins/marketplace-reference).

```bash theme={null}
claude plugin validate <path> [options]
```

| Bandera    | Descripción                                                                                                                                                                  |
| :--------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--strict` | Trata advertencias como errores, por lo que campos no reconocidos y metadatos faltantes que el runtime tolera fallan la ejecución. Requiere Claude Code v2.1.145 o posterior |
| `--json`   | Salida del informe de validación como un objeto JSON con los mismos códigos de salida. Requiere Claude Code v2.1.259 o posterior                                             |

Valida un plugin antes de confirmarlo:

```bash theme={null}
claude plugin validate ./my-plugin --strict
```

<h4 id="validate-a-directory">
  Validar un directorio
</h4>

El `<path>` es un archivo de manifiesto o un directorio. Dado un directorio, Claude Code elige qué validar por lo que encuentra allí:

* `.claude-plugin/marketplace.json`, cuando existe
* Si no `.claude-plugin/plugin.json`
* Si no los archivos de componentes, elegidos por el nombre del directorio. Validar archivos de componentes sin un manifiesto requiere Claude Code v2.1.233 o posterior:
  * Un directorio nombrado `skills`, `agents` o `commands`: los archivos dentro de él
  * Un directorio nombrado `.claude`: los directorios `skills`, `agents` y `commands` dentro de él
  * Cualquier otro directorio: esos tres directorios bajo su `.claude`

Claude Code no sigue enlaces simbólicos dentro del directorio que nombras. Lo que hace depende de dónde esté el enlace:

* **Un directorio `skills`, `agents` o `commands` enlazado bajo la raíz del plugin o `.claude`**: Claude Code advierte que nada en él fue leído.
* **Una entrada enlazada dentro de un directorio `skills`, `agents` o `commands`**: Claude Code la omite y advierte, por directorio, cuántas entradas omitió que una sesión cargaría.
* **El directorio `skills`, `agents` o `commands` que nombras es en sí mismo un enlace simbólico, o su directorio padre `.claude` es**: Claude Code reporta un error y no verifica nada en él. Nombra el directorio real en su lugar.

Algunos archivos no son leídos por una ejecución de validación:

* **Un `SKILL.md` en la raíz del plugin**: cuando ejecutas `claude plugin validate` contra un directorio de plugin, Claude Code no verifica un `SKILL.md` en la raíz del plugin
* **Un `CLAUDE.md` en la raíz del plugin**: en una ejecución de plugin, Claude Code también advierte sobre un `CLAUDE.md` en la raíz del plugin
* **Archivos de plugin en una ejecución de marketplace**: desde un directorio de marketplace, Claude Code no abre los archivos de skill, agent, comando o hook de los plugins. Para encontrar errores en esos archivos, valida cada directorio de plugin

<h4 id="output-and-exit-codes">
  Salida y códigos de salida
</h4>

Claude Code imprime el archivo que validó, cualquier error y advertencia con sus rutas, y una línea de veredicto. El código de salida sigue el veredicto:

| Código de salida | Línea de veredicto                                                             | Significado                                                   |
| :--------------- | :----------------------------------------------------------------------------- | :------------------------------------------------------------ |
| `0`              | `Validation passed` o `Validation passed with warnings`                        | El manifiesto carga. Con `--strict`, sin advertencias tampoco |
| `1`              | `Validation failed` o `Validation failed (--strict treats warnings as errors)` | Un error, o una advertencia bajo `--strict`                   |
| `2`              | `Unexpected error during validation: <reason>`                                 | El validador en sí falló, como en una ruta ilegible           |

Con `--json`, Claude Code escribe el informe a stdout como un objeto JSON con estos campos de nivel superior:

* `success`: el mismo veredicto que el código de salida da
* `strict`: si la ejecución trató advertencias como errores
* `target`: la ruta resuelta que Claude Code validó
* `manifest`: el resultado del propio manifiesto, o `null` para una ejecución sin manifiesto
* `contents`: resultados por archivo, cada uno nombrando su `file` y llevando arrays `errors`, `warnings` y `notes`

En salida `2`, el comando no escribe nada a stdout. El mensaje de error va a stderr.

<h2 id="claude-plugin-marketplace-commands">
  Comandos claude plugin marketplace
</h2>

Ejecuta `claude plugin marketplace <subcommand>` desde tu shell para añadir, listar, actualizar y eliminar los marketplaces desde los que instalas plugins.

* **Códigos de salida**: estos subcomandos siguen la [convención de código de salida](#claude-plugin-commands) de los comandos de plugin
* **Ámbitos**: su bandera `--scope` no tiene forma corta `-s`

Para qué es un marketplace y cómo Claude Code lo cachea, consulta [Referencia de carga de plugins](/docs/es/plugins/loading).

<h3 id="plugin-marketplace-add">
  plugin marketplace add
</h3>

Añade un marketplace desde un repositorio de GitHub, una URL de git, un `marketplace.json` alojado, o una ruta local, y decláralo en un archivo de configuración.

Después de añadirlo, Claude Code instala cualquier [dependencia](/docs/es/plugins/dependencies) que tus plugins instalados estaban perdiendo.

```bash theme={null}
claude plugin marketplace add <source> [options]
```

| Bandera               | Descripción                                                                                                                                                                           |
| :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `--scope <scope>`     | Archivo de configuración para declarar el marketplace en: `user`, `project` o `local`. Por defecto es `user`                                                                          |
| `--sparse <paths...>` | Limita el checkout de git a estos directorios, para monorepos. Solo fuentes `github` y `git`                                                                                          |
| `--claudeai`          | Lee el argumento como el nombre de un [marketplace alojado en claude.ai](/docs/es/plugins/install#add-from-claude-ai) en su lugar de una fuente. Requiere Claude Code v2.1.273 o posterior |

`<source>` toma cualquiera de las formas en la tabla a continuación, y su forma decide el tipo de fuente y cómo Claude Code obtiene el marketplace. Para el objeto de fuente resultante, consulta la [referencia de marketplace](/docs/es/plugins/marketplace-reference).

| Escribes                                                                                     | Tipo de fuente | Cómo Claude Code lo obtiene                                                                                                        |
| :------------------------------------------------------------------------------------------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------- |
| `owner/repo`, `owner/repo#ref` o `owner/repo@ref`                                            | `github`       | Clona el repositorio de GitHub, fijado a `ref` cuando se da. El propietario y el repo deben seguir las reglas de nombres de GitHub |
| `user@host:path[.git][#ref]`                                                                 | `git`          | Clona sobre SSH                                                                                                                    |
| `https://example.com/repo.git[#ref]`, o una URL que contiene `/_git/`                        | `git`          | Clona sobre HTTPS, incluyendo URLs de Azure DevOps                                                                                 |
| `https://github.com/owner/repo` o `https://gitlab.com/namespace/project`                     | `git`          | Clona sobre HTTPS después de añadir `.git`                                                                                         |
| Cualquier otra URL `http://` o `https://`, incluyendo un host de git auto-alojado sin `.git` | `url`          | Obtiene la URL como un `marketplace.json`. Para clonar un repositorio allí en su lugar, añade `.git`                               |
| `./path`, `../path`, `/path` o `~/path` a un directorio                                      | `directory`    | Lee el directorio en su lugar. En Windows, las formas `.\`, `..\` y `C:\` también funcionan                                        |
| Las mismas formas de ruta, a un archivo `.json`                                              | `file`         | Lee el archivo en su lugar                                                                                                         |

Para un host cuyas URLs de clonación no llevan el sufijo `.git`, como AWS CodeCommit, añade el marketplace como una entrada de git en [`extraKnownMarketplaces`](/docs/es/settings-reference#extraknownmarketplaces) en su lugar. Claude Code clona una entrada de git sin importar si su URL termina en `.git`.

Claude Code también clona una URL de `gitlab.com` con subgrupos anidados, como `https://gitlab.com/group/subgroup/project`.

Añade un marketplace y compártelo con el proyecto:

```bash theme={null}
claude plugin marketplace add your-org/your-marketplace --scope project
```

Claude Code imprime `Successfully added marketplace: your-marketplace (declared in project settings)`, usando el `name` del propio manifiesto del marketplace. Una adición repetida o una fuente inválida imprime uno de estos resultados en su lugar:

* **Marketplace ya en disco**: la salida es `Marketplace 'your-marketplace' already on disk — declared in project settings` y el código de salida es `0`
* **Fuente no reconocida**: la salida es `Invalid marketplace source format. Try: owner/repo, https://..., or ./path` y el código de salida es `1`
* **Host simple como `gitlab.example.com/team/plugins`**: la adición falla como un atajo `owner/repo` inválido, y el mensaje te dice que añadas `https://` o uses una ruta local

Añade un [marketplace alojado en claude.ai](/docs/es/plugins/install#add-from-claude-ai) por el nombre impreso en la sección `From claude.ai:` de `claude plugin marketplace list`:

```bash theme={null}
claude plugin marketplace add --claudeai claudeai-organization-library
```

Con `--claudeai`, el comando rechaza `--scope` y `--sparse`. El marketplace está alojado para tu cuenta, no declarado en un archivo de configuración, por lo que no puedes compartirlo a través del `.claude/settings.json` de un proyecto.

<h3 id="plugin-marketplace-list">
  plugin marketplace list
</h3>

Lista cada marketplace que has añadido, con su fuente.

```bash theme={null}
claude plugin marketplace list [options]
```

| Bandera  | Descripción                |
| :------- | :------------------------- |
| `--json` | Imprime la lista como JSON |

Claude Code imprime `Configured marketplaces:` y una línea `Source:` por marketplace, o `No marketplaces configured`.

Con `--json`, Claude Code imprime un array con un objeto por marketplace, llevando los campos a continuación. Cada campo es una string.

| Campo             | Descripción                                                                  |
| :---------------- | :--------------------------------------------------------------------------- |
| `name`            | El nombre del marketplace                                                    |
| `source`          | `github`, `git`, `url`, `directory`, `file` o `claudeai`                     |
| `repo`            | `owner/repo`. Solo fuentes `github`                                          |
| `url`             | La URL de clonación u obtención. Solo fuentes `git` y `url`                  |
| `path`            | La ruta local. Solo fuentes `directory` y `file`                             |
| `ref`             | La rama o etiqueta fijada. Fuentes `github` y `git`, solo cuando está fijada |
| `installLocation` | Dónde Claude Code cachó el marketplace                                       |

Un marketplace de [claude.ai](/docs/es/plugins/install#add-from-claude-ai) añadido no tiene un clon local, por lo que su entrada lleva sus identificadores de claude.ai, `marketplaceId` y `organizationUuid`, en su lugar de `installLocation`. También lleva `scope` cuando uno se registra, y `status`.

Si tus sesiones de terminal [sincronizan plugins desde tu cuenta claude.ai](/docs/es/plugins/loading#synced-plugins), el texto listado termina con una sección `From claude.ai:`. Esa sección nombra los marketplaces que claude.ai lista para tu cuenta que no has añadido, tanto basados en git como alojados. Requiere Claude Code v2.1.273 o posterior.

Para añadir un marketplace de esa sección, consulta [Añadir un marketplace desde claude.ai](/docs/es/plugins/install#add-from-claude-ai).

La salida de `--json` cubre solo marketplaces configurados y deja la sección fuera.

<h3 id="plugin-marketplace-remove">
  plugin marketplace remove
</h3>

Elimina la declaración de un marketplace de tu configuración. `rm` es un alias para `remove`.

<Warning>
  Cuando eliminas un marketplace del último ámbito que lo declara, Claude Code también elimina su caché y desinstala cada plugin que instalaste desde él. Sin `--scope`, el comando elimina la declaración de cada ámbito. Para actualizar un marketplace sin perder sus plugins, ejecuta `plugin marketplace update` en su lugar.
</Warning>

```bash theme={null}
claude plugin marketplace remove <name> [options]
```

El `<name>` es el nombre del marketplace que `plugin marketplace list` muestra, no la fuente que pasaste a `add`.

| Bandera           | Descripción                                                                                                                                  |
| :---------------- | :------------------------------------------------------------------------------------------------------------------------------------------- |
| `--scope <scope>` | Elimina la declaración de un ámbito de configuración: `user`, `project` o `local`. Sin él, Claude Code elimina la declaración de cada ámbito |

Elimina un marketplace de cada ámbito:

```bash theme={null}
claude plugin marketplace remove your-marketplace
```

Claude Code imprime `Successfully removed marketplace: your-marketplace`, añadiendo `(from project settings)` cuando lo limitaste. Si limitas a un archivo de configuración que no declara el marketplace, el comando falla con `Marketplace 'your-marketplace' is not declared in project settings. Omit --scope to remove it from all scopes.`

<h3 id="plugin-marketplace-update">
  plugin marketplace update
</h3>

Actualiza un marketplace, o cada marketplace, desde su fuente para obtener nuevos plugins y versiones. Un marketplace añadido con una rama o etiqueta `ref` se actualiza al último commit de esa ref, no la rama predeterminada del repositorio.

```bash theme={null}
claude plugin marketplace update [name]
```

El comando no toma banderas más allá de `--help`.

Actualiza un marketplace:

```bash theme={null}
claude plugin marketplace update your-marketplace
```

Claude Code imprime `Successfully updated marketplace: your-marketplace`. Cuando omites el nombre, imprime un recuento como `Successfully updated 2 marketplaces`. Sin marketplaces añadidos, imprime `No marketplaces configured` y sale con `0`.

<h2 id="plugin-in-a-session">
  /plugin en una sesión
</h2>

Dentro de una sesión interactiva, `/plugin` abre el panel de plugins. Cada subcomando abre el panel en una pestaña, ejecuta una acción allí, o imprime un resultado en línea. `/plugins` y `/marketplace` son alias para `/plugin`.

Solo puedes ejecutar estos comandos en una sesión de terminal interactiva. En una ejecución no interactiva como `claude -p`, Claude Code responde que `/plugin` no está disponible en este entorno.

Para qué superficies tienen `/plugin`, cómo instalar sin él, y qué muestra cada pestaña del panel, consulta [Instalar y gestionar plugins](/docs/es/plugins/install).

Un `<plugin>` es un `name` de plugin o `name@marketplace`.

La tabla a continuación lista cada forma de sesión. Los subcomandos de shell `init`, `update`, `details`, `prune`, `eval` y `eval init` no tienen forma de sesión.

| Comando                                             | Alias                                          | Qué hace                                                                                                                                                                                                                                                                                                      |
| :-------------------------------------------------- | :--------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `/plugin`                                           |                                                | Abre el panel en la pestaña **Discover**. Cualquier primera palabra no reconocida después de `/plugin` hace lo mismo                                                                                                                                                                                          |
| `/plugin help`                                      | `/plugin --help`, `/plugin -h`                 | Muestra la lista de uso de subcomandos de `/plugin`                                                                                                                                                                                                                                                           |
| `/plugin list [--enabled\|--disabled]`              | `ls`                                           | Imprime tus plugins instalados de marketplace en línea, con versión, ámbito y estado. Una bandera de filtro muestra solo ese estado. Un plugin cuyo estado de habilitación aún no se ha aplicado está marcado como `— run /reload-plugins to apply`. Requiere Claude Code v2.1.163 o posterior                |
| `/plugin install`                                   | `i`                                            | Abre la pestaña **Discover**                                                                                                                                                                                                                                                                                  |
| `/plugin install <plugin>`                          | `i`                                            | Abre los detalles del plugin en la pestaña **Discover**. Con `name@marketplace`, los abre en la lista de ese marketplace                                                                                                                                                                                      |
| `/plugin install <plugin> --marketplace <source>`   | `i`                                            | Añade el marketplace en `<source>` cuando aún no lo has añadido, pidiéndote que confirmes primero, luego abre los detalles del plugin. Consulta [Añadir un marketplace e instalar en un comando](/docs/es/plugins/install#add-a-marketplace-and-install-in-one-command). Requiere Claude Code v2.1.275 o posterior |
| `/plugin manage`                                    |                                                | Abre la pestaña **Installed**                                                                                                                                                                                                                                                                                 |
| `/plugin stats`                                     |                                                | Abre la pestaña **Stats**, en sesiones donde [`/skill-doctor`](/docs/es/skills#find-unused-skills) está disponible. En cualquier otro lugar abre el panel en la pestaña **Discover**                                                                                                                               |
| `/plugin enable <plugin>`                           |                                                | Abre la pestaña **Installed** en el plugin y lo habilita                                                                                                                                                                                                                                                      |
| `/plugin disable <plugin>`                          |                                                | Abre la pestaña **Installed** en el plugin y lo deshabilita                                                                                                                                                                                                                                                   |
| `/plugin uninstall <plugin>`                        |                                                | Abre la pestaña **Installed** en el plugin y lo desinstala                                                                                                                                                                                                                                                    |
| `/plugin configure <plugin>`                        | `config`                                       | Abre el diálogo [`userConfig`](/docs/es/plugins/manifest-reference) del plugin, o reporta que el plugin no declara ninguno. Requiere Claude Code v2.1.147 o posterior                                                                                                                                              |
| `/plugin validate <path>`                           |                                                | Imprime el mismo informe que `claude plugin validate`, en línea                                                                                                                                                                                                                                               |
| `/plugin tag [path] [--push] [--dry-run] [--force]` |                                                | Crea la etiqueta de versión como `claude plugin tag` hace. Acepta `--push`, `--dry-run` y `--force` o `-f`; con cualquier otra bandera o un argumento extra, Claude Code imprime el uso en su lugar                                                                                                           |
| `/plugin marketplace`                               | `market`                                       | No hace nada visible. Pasa `add`, `list`, `update` o `remove`                                                                                                                                                                                                                                                 |
| `/plugin marketplace add [source]`                  | `market add`                                   | Con una fuente, la añade e informa el resultado. Sin una, abre la entrada **Add marketplace**                                                                                                                                                                                                                 |
| `/plugin marketplace list`                          | `market list`                                  | Imprime tus nombres de marketplace en línea                                                                                                                                                                                                                                                                   |
| `/plugin marketplace update [name]`                 | `market update`                                | Abre la pestaña **Marketplaces**. Con un nombre, actualiza ese marketplace allí                                                                                                                                                                                                                               |
| `/plugin marketplace remove [name]`                 | `market remove`, `market rm`, `marketplace rm` | Abre la pestaña **Marketplaces**. Con un nombre, elimina ese marketplace allí                                                                                                                                                                                                                                 |

Si nombras un plugin que no está instalado en el proyecto actual en `/plugin enable`, `disable`, `uninstall` o `configure`, Claude Code imprime `Plugin "<plugin>" is not installed in this project` en su lugar de actuar.

<h2 id="reload-plugins">
  /reload-plugins
</h2>

Aplica cambios de plugins pendientes a la sesión en ejecución sin reiniciarla. Los cambios pendientes son plugins que instalaste, actualizaste, habilitaste, deshabilitaste o editaste en disco desde que comenzó la sesión.

Cuando cierras el panel `/plugin` con cambios pendientes que hiciste en él, Claude Code ejecuta `/reload-plugins` para ti. Ejecútalo tú mismo después de cambios de plugins que suceden fuera del panel, como un comando `claude plugin` que ejecutaste en otra terminal.

```text theme={null}
/reload-plugins [--force]
```

| Bandera   | Descripción                                                                                           |
| :-------- | :---------------------------------------------------------------------------------------------------- |
| `--force` | Aplica la recarga incluso cuando invalidaría el caché de prompt. `force` sin guiones también funciona |

<h3 id="reload-summary">
  Resumen de recarga
</h3>

Claude Code recarga cada plugin activo e imprime una línea de resumen, `Reloaded: N plugins · N skills · N agents · N hooks · N plugin MCP servers · N plugin LSP servers`, omitiendo el recuento de servidor MCP de plugin en una sesión sin una terminal interactiva. Cuando algún plugin falló, el resumen añade `N errors during load. Run /plugin for details.`

El recuento de skills cubre cada skill que proporciona un plugin, tanto sus entradas de `commands/` como sus skills de `SKILL.md`. El recuento de agents es el número de agents cargados en la sesión, incluyendo los que no vienen de plugins.

Cuando las [dependencias](/docs/es/plugins/dependencies) de un plugin recargado están faltando, Claude Code las instala, recarga de nuevo, y añade `(+ N dependencies: <names>) resolved` al resumen.

<h3 id="reloads-that-change-mcp-tools">
  Recargas que cambian herramientas MCP
</h3>

Cuando la recarga añadiría o eliminaría un servidor MCP de plugin o la herramienta `LSP`, y ese cambio invalidaría el [caché de prompt](/docs/es/prompt-caching#enabling-or-disabling-a-plugin), Claude Code no aplica la recarga. Imprime una línea como `This reload changes MCP tools (<server>) — your next message will re-read the whole conversation instead of using the cache. Run /reload-plugins --force to apply.` Pasa `--force` para aplicarla de todas formas.

<h3 id="sessions-without-an-interactive-terminal">
  Sesiones sin una terminal interactiva
</h3>

`/reload-plugins` también se ejecuta en sesiones sin una terminal interactiva, como la aplicación de escritorio, el Agent SDK, y [modo no interactivo](/docs/es/headless) con `-p`. Requiere Claude Code v2.1.260 o posterior.

En esas sesiones, el comando se ejecuta solo cuando lo escribes en la sesión tú mismo, como en el prompt de `-p` o la caja de prompt de la aplicación de escritorio. Cuando llega de otra forma, como a través de [Remote Control](/docs/es/remote-control) o un mensaje retransmitido desde Slack, el comando responde `/reload-plugins isn't available over a remote connection in this session.` y no recarga nada.

La recarga en esas sesiones no conecta o desconecta servidores MCP de plugins. Esos cambios toman efecto en tu próxima sesión.

<h2 id="flags-that-load-a-plugin-for-one-session">
  Banderas que cargan un plugin para una sesión
</h2>

Dos banderas de `claude` cargan un plugin para una sesión solamente, sin instalarlo. Ambas son repetibles.

Los autores de plugins las usan para probar un plugin antes de publicarlo. Para el flujo de trabajo de cargar-editar-recargar, consulta [Desarrollar sin un marketplace](/docs/es/plugins/create#develop-without-a-marketplace).

| Bandera               | Descripción                                                                                                                                                                             | Ejemplo                                                                     |
| :-------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------- |
| `--plugin-dir <path>` | Carga un plugin desde un directorio o un archivo `.zip` de uno. Una carpeta de plugins carga cada carpeta hijo que contiene un `.claude-plugin/plugin.json`. Cada bandera toma una ruta | `claude --plugin-dir ./my-plugin --plugin-dir ./other.zip`                  |
| `--plugin-url <url>`  | Obtiene un archivo `.zip` de plugin desde una URL. Repite la bandera, o pasa varias URLs separadas por espacios en un valor entrecomillado                                              | `claude --plugin-url "https://example.com/a.zip https://example.com/b.zip"` |

Un plugin que cualquiera de esas banderas carga es un plugin de solo sesión. `claude plugin list` lo muestra como `<name>@inline` con ámbito `session`, pero solo cuando la misma bandera precede al subcomando. Por ejemplo, ejecuta `claude --plugin-dir ./my-plugin plugin list`.

Cuando un plugin de solo sesión comparte un nombre con un plugin instalado, Claude Code carga la copia de solo sesión para esa sesión y omite la instalada. La copia instalada se carga en su lugar si deshabilitaste la copia de solo sesión con `claude plugin disable <name>@inline`, o si la configuración gestionada bloquea ese nombre de plugin. Para la precedencia, consulta [Referencia de carga de plugins](/docs/es/plugins/loading).

Un administrador puede rechazar ambas banderas, y carpetas nombradas en la variable [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/es/env-vars#variables), con la configuración gestionada [`disableSideloadFlags`](/docs/es/settings-reference#disablesideloadflags). Claude Code entonces imprime que la bandera está deshabilitada por la configuración gestionada de tu organización y sale con `1` sin iniciar.

Desde el Agent SDK, la opción [`plugins`](/docs/es/agent-sdk/plugins) es el equivalente de `--plugin-dir`.

<h2 id="next-steps">
  Próximos pasos
</h2>

* [Instalar y gestionar plugins](/docs/es/plugins/install): las mismas operaciones que pasos, con lo que ves en cada uno
* [Referencia de carga de plugins](/docs/es/plugins/loading): qué cambia cada comando en el disco y qué ámbito toma efecto
* [Solucionar problemas de plugins](/docs/es/plugins/troubleshooting): instalar, marketplace, cargar y mensajes de error de validación con sus soluciones
* [Referencia de manifiesto de plugin](/docs/es/plugins/manifest-reference): los campos que `claude plugin validate` verifica
