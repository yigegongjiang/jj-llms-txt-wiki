> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Dependencias de plugins

> Declare los plugins de los que depende su plugin, con rangos de versión como ^1.2, y vea cómo Claude Code instala, resuelve y elimina las dependencias.

Una dependencia de plugin es otro plugin del que su plugin depende, como uno cuyo servidor MCP o skill llama. Cada dependencia rastrea la versión más reciente que proporciona su marketplace a menos que declare una restricción de versión, un rango de versión semántica como `^2.0` o `~2.1.0` que haya probado.

Esta página es para autores de plugins que declaran dependencias en `plugin.json` y para mantenedores de marketplace que etiquetan versiones.

<Note>
  Estos casos se cubren en otras páginas:

  * **Instalar un plugin que tiene dependencias**: consulte [Administrar plugins instalados](/docs/es/plugins/install#manage-installed-plugins)
  * **Leer un error de dependencia**: consulte [Errores de dependencia](/docs/es/plugins/troubleshooting#dependency-errors)
  * **Declarar los paquetes npm y Bun que el código de su plugin necesita**: consulte [Dependencias de paquetes Node.js](/docs/es/plugins/loading#node-js-package-dependencies)
</Note>

Para agregar una restricción, comience en [Declare una dependencia con una restricción de versión](#declare-a-dependency-with-a-version-constraint). Si mantiene un plugin del que otros dependen, [etiquete sus versiones](#tag-plugin-releases-for-version-resolution) para que sus restricciones puedan resolverse.

<h2 id="declare-dependencies">
  Declare dependencias
</h2>

<span id="decide-whether-to-constrain-dependency-versions" />Sin una restricción de versión, una dependencia se mueve a cada nueva versión que su marketplace publica la próxima vez que los usuarios actualizan. Si esa versión cambia el nombre de una herramienta MCP que su plugin llama, su plugin se rompe para todos los que actualizan.

Con una restricción como `~2.1.0` en una dependencia de una fuente respaldada por git, los usuarios que tienen su plugin instalado siguen recibiendo parches `2.1.x` de la dependencia y nunca se mueven a `2.2`. Para actualizar en su propio cronograma, pruebe contra una versión más reciente y luego publique una nueva versión de su plugin con una restricción más amplia.

<h3 id="declare-a-dependency-with-a-version-constraint">
  Declare una dependencia con una restricción de versión
</h3>

Liste las dependencias en el array `dependencies` del archivo `.claude-plugin/plugin.json` de su plugin. El siguiente manifiesto declara una dependencia sin versión y una dependencia con restricción:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "deploy-kit",
  "version": "3.1.0",
  "dependencies": [
    "audit-logger",
    { "name": "secrets-vault", "version": "~2.1.0" }
  ]
}
```

Una entrada puede ser una cadena: solo el nombre del plugin, como `"audit-logger"` en este manifiesto, o `"name@marketplace"` para resolverlo en otro marketplace. Con una cadena simple, su plugin depende de cualquier versión que proporcione el marketplace de ese plugin.

Para establecer una restricción de versión, use un objeto con estos campos, cada uno una cadena:

| Campo         | Descripción                                                                                                                                                                                                                                                                                                                |
| :------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | El nombre del plugin de dependencia, tal como aparece en su entrada de marketplace. Claude Code lo busca en el mismo marketplace que el plugin declarante a menos que establezca `marketplace`. Requerido.                                                                                                                 |
| `version`     | Un [rango de versión semántica](https://github.com/npm/node-semver#ranges) como `~2.1.0`, `^2.0`, `>=1.4`, o `=2.1.0`. La dependencia se instala en la etiqueta git más alta que satisface este rango, por lo que el mantenedor de la dependencia debe [etiquetar versiones](#tag-plugin-releases-for-version-resolution). |
| `marketplace` | Un marketplace diferente para resolver `name` en. Una lista de permitidos controla las dependencias entre marketplaces, descrito en [Dependa de un plugin de otro marketplace](#depend-on-a-plugin-from-another-marketplace).                                                                                              |

Un rango no coincide con versiones previas al lanzamiento como `2.0.0-beta.1` a menos que opte por un sufijo previo al lanzamiento como `^2.0.0-0`.

<h3 id="bundle-plugins-for-a-team">
  Agrupe plugins para un equipo
</h3>

Para permitir que los ingenieros instalen un conjunto curado de plugins con un comando, publique un plugin cuyo manifiesto contenga un `name` y un array `dependencies`. Un manifiesto de plugin solo necesita `name`, por lo que este es un plugin válido, e instalarlo instala cada dependencia.

Por ejemplo, un equipo de plataforma puede publicar bundles específicos de roles en un marketplace interno para que los ingenieros ejecuten un `claude plugin install` en lugar de instalar cada plugin por separado:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "backend-standard",
  "version": "1.0.0",
  "description": "Conjunto de plugins estándar para ingenieros de backend",
  "dependencies": [
    "secrets-vault",
    "deploy-kit",
    { "name": "db-migrate", "version": "^3.0" },
    "oncall-runbook"
  ]
}
```

Para agregar un plugin al conjunto estándar más tarde, publique una nueva versión de `backend-standard` con la dependencia adicional. Cuando el marketplace no [se actualiza automáticamente de forma predeterminada](/docs/es/plugins/loading#which-marketplaces-and-plugins-auto-update), los ingenieros activan la actualización automática para el marketplace o actualizan manualmente:

* **Activar la actualización automática para el marketplace**: la próxima actualización automática mueve el bundle a la nueva versión e instala cualquier dependencia que agregue.
* **Actualizar manualmente**: ejecute `claude plugin update backend-standard` en un shell, luego `/reload-plugins` en una sesión abierta para instalar las dependencias recién agregadas.

Para los pasos del lado del ingeniero, consulte [Mantenga los plugins actualizados](/docs/es/plugins/install#keep-plugins-updated).

Para implementar un bundle para todos en una organización, un administrador lo agrega a `enabledPlugins` en la configuración administrada. Consulte [Pre-instalar y requerir plugins](/docs/es/plugins/org#pre-install-and-require-plugins).

<h3 id="depend-on-a-plugin-from-another-marketplace">
  Dependa de un plugin de otro marketplace
</h3>

De forma predeterminada, Claude Code no instala una dependencia de un marketplace diferente al del plugin declarante, a menos que el usuario ya tenga esa dependencia instalada y habilitada en el mismo alcance. Este valor predeterminado evita que un marketplace instale silenciosamente plugins de una fuente que el usuario no ha revisado.

Para permitir la instalación, agregue el nombre del marketplace de destino a `allowCrossMarketplaceDependenciesOn` en el `marketplace.json` del marketplace raíz. El marketplace raíz es el que aloja el plugin que el usuario está instalando. Solo se aplica la lista de permitidos del marketplace raíz.

El siguiente `marketplace.json` permite que `deploy-kit` dependa de un plugin de `your-shared-marketplace`:

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "allowCrossMarketplaceDependenciesOn": ["your-shared-marketplace"],
  "plugins": [
    {
      "name": "deploy-kit",
      "source": "./deploy-kit",
      "dependencies": [
        { "name": "audit-logger", "marketplace": "your-shared-marketplace" }
      ]
    }
  ]
}
```

Si `allowCrossMarketplaceDependenciesOn` falta o no incluye el marketplace de destino, Claude Code no instala la dependencia. Cuando la dependencia se declara en la entrada del marketplace, la instalación se rechaza con un mensaje que comienza con `Dependency "audit-logger@your-shared-marketplace" (required by deploy-kit@your-marketplace) is in marketplace "your-shared-marketplace", which is not in the allowlist` y nombra el campo a establecer. Cuando se declara en `plugin.json`, la instalación se completa sin la dependencia y su plugin luego falla al cargar.

La verificación de la lista de permitidos no se aplica a una dependencia que ya está habilitada. Si un usuario instala `audit-logger` de `your-shared-marketplace` primero, en el mismo alcance, `deploy-kit` luego se instala sin ningún cambio en la lista de permitidos.

<h3 id="test-a-plugin-and-its-dependency-locally">
  Pruebe un plugin y su dependencia localmente
</h3>

Si está desarrollando un plugin y el plugin del que depende al mismo tiempo, inicie Claude Code desde su shell y cargue ambos con [`--plugin-dir`](/docs/es/plugins/cli-reference#flags-that-load-a-plugin-for-one-session):

```bash theme={null}
claude --plugin-dir ./my-dependency --plugin-dir ./my-plugin
```

La copia local de la dependencia satisface la entrada de dependencia de su plugin, por lo que no necesita instalar la dependencia desde su marketplace.

* **Sin `version` necesaria**: el `plugin.json` local tampoco necesita una `version`, porque una [restricción de versión](#declare-a-dependency-with-a-version-constraint) no se verifica contra una copia local.
* **Entradas que nombran un marketplace**: una entrada que nombra un marketplace también coincide con la copia local en Claude Code v2.1.242 o posterior.

Hasta que instale la dependencia desde su marketplace, su plugin deja de cargar cada vez que la copia local está deshabilitada o ausente:

* **Deshabilitó la copia local**: su plugin está deshabilitado en la próxima carga de plugin, con un error que termina con `is disabled — enable it or remove the dependency`. Cuando el error nombra la dependencia como `<name>@inline`, ese identificador se refiere a la copia de `--plugin-dir`.
* **Inició una sesión sin la bandera `--plugin-dir` de la dependencia**: el error reporta que la dependencia no está instalada. Pase la bandera nuevamente, o instale la dependencia desde su marketplace.

Cuando ambos plugins están en una carpeta padre, puede pasar esa carpeta a `--plugin-dir` una vez. Si la carpeta no es en sí misma un plugin, Claude Code carga cada carpeta secundaria que tenga un `.claude-plugin/plugin.json`. Requiere Claude Code v2.1.265 o posterior.

<h2 id="tag-plugin-releases-for-version-resolution">
  Publique un plugin del que otros dependen
</h2>

Si mantiene un plugin del que otros plugins dependen con una restricción de versión, etiquete sus versiones para que esas restricciones puedan resolverse. Una restricción se resuelve contra etiquetas git en el repositorio que aloja el plugin. Etiquete el repositorio al que apunta la [fuente del plugin](/docs/es/plugins/marketplace-reference#plugin-sources) del plugin en `marketplace.json`:

* **Fuente `github`, `url`, o `git-subdir`**: el repositorio del plugin, por lo que el autor del plugin crea las etiquetas
* **Ruta relativa como `./plugins/secrets-vault`**: el repositorio del marketplace, por lo que el mantenedor del marketplace crea las etiquetas

<h3 id="create-a-release-tag">
  Cree una etiqueta de versión
</h3>

Etiquete cada versión como `<plugin-name>--v<version>`, donde `<version>` coincide con el campo `version` en el `plugin.json` de ese commit. El prefijo plugin-name permite que un repositorio de marketplace aloje varios plugins con historiales de versión independientes.

Cree la etiqueta desde el directorio del plugin, con un control remoto `origin` configurado para recibir la etiqueta enviada, usando [`claude plugin tag`](/docs/es/plugins/cli-reference#plugin-tag):

```bash theme={null}
claude plugin tag --push
```

El comando construye el nombre de la etiqueta desde el manifiesto del plugin. Antes de crear la etiqueta, ejecuta estas verificaciones:

* Valida el plugin
* Verifica que `plugin.json` y la entrada del marketplace estén de acuerdo sobre la versión, cuando el directorio del plugin está dentro de un checkout del marketplace
* Requiere un árbol de trabajo limpio bajo el directorio del plugin
* Se niega si la etiqueta ya existe

Una ejecución exitosa imprime `Created tag secrets-vault--v2.1.0`. Con `--push`, también imprime `Pushed to origin`. Sin `--push`, imprime el comando `git push` para ejecutar usted mismo.

Pase `--dry-run` para ver el plan sin crear nada.

La [referencia de `claude plugin tag`](/docs/es/plugins/cli-reference#plugin-tag) lista las banderas restantes.

También puede ejecutar `git tag secrets-vault--v2.1.0` directamente, siempre que mantenga la `version` en `plugin.json` y en la entrada del marketplace sincronizadas usted mismo.

<h3 id="constrain-a-dependency-that-has-a-non-git-source">
  Restrinja una dependencia que tiene una fuente que no es git
</h3>

La resolución basada en etiquetas se aplica solo a fuentes respaldadas por git. Para una dependencia con una [fuente de plugin](/docs/es/plugins/marketplace-reference#plugin-sources) `npm`, `archive`, o `command`, la restricción no controla qué versión se obtiene. Aún se verifica cuando el plugin carga, y el plugin dependiente se deshabilita si la versión instalada no la satisface.

Para fuentes `npm`, `archive`, y `command`, la versión verificada es la `version` en el `plugin.json` de la dependencia. Establezca una allí antes de restringir esa dependencia, porque un `plugin.json` que no establece versión no satisface ninguna restricción.

Claude Code nunca instala una dependencia con una fuente `command` en sí, por lo que los usuarios [la instalan primero](/docs/es/plugins/marketplace-reference#command-plugin-source). Tampoco ejecuta nunca el [`headersHelper`](/docs/es/plugins/host-marketplace#authenticate-archive-downloads) de una dependencia, por lo que los usuarios también instalan una dependencia cuya entrada de marketplace establece uno antes de instalar su plugin.

Además de `claude plugin install`, estas operaciones también instalan cualquier dependencia declarada faltante, y los límites de `command` y `headersHelper` se aplican a ellas también:

* `/reload-plugins`
* Auto-actualización del marketplace del plugin dependiente
* Re-ejecutar `claude plugin install` en el plugin dependiente
* `claude plugin marketplace add`

<h2 id="how-dependencies-behave-for-your-users">
  Cómo se comportan las dependencias para sus usuarios
</h2>

Estas secciones describen cómo Claude Code resuelve, verifica y combina las restricciones que declara una vez que su plugin está instalado junto con otros.

<h3 id="how-a-constraint-resolves-against-tags">
  Cómo una restricción se resuelve contra etiquetas
</h3>

Cuando un usuario instala un plugin que declara `{ "name": "secrets-vault", "version": "~2.1.0" }`, la dependencia se instala desde la etiqueta `secrets-vault--v` más alta que satisface `~2.1.0` en el repositorio que aloja `secrets-vault`. Cuando ninguna etiqueta satisface el rango, la instalación falla o usa la copia actual del marketplace:

* **Plugin con su propio repositorio**: la instalación falla con un mensaje que contiene `Dependency "secrets-vault@your-marketplace" has no git tag satisfying`.
* **Plugin referenciado por una ruta relativa**: la instalación usa la copia actual del marketplace en su lugar, y la restricción se verifica cuando el plugin carga. Si esa copia está fuera del rango, el plugin dependiente permanece deshabilitado y `claude plugin list` muestra `Requires "secrets-vault@your-marketplace" ~2.1.0, installed 3.0.0`.

Para un plugin que el marketplace referencia por una ruta relativa, un marketplace que agregó como una ruta de carpeta local también resuelve restricciones contra las etiquetas git de esa carpeta, cuando la carpeta es un repositorio git. Esto requiere Claude Code v2.1.196 o posterior. Una carpeta local que no es un repositorio git no tiene etiquetas, por lo que Claude Code instala la dependencia desde el contenido actual de la carpeta en su lugar.

<h3 id="confirm-the-resolved-version">
  Confirme la versión resuelta
</h3>

Para confirmar qué versión una restricción se resolvió, ejecute `claude plugin list` en su shell. Una dependencia resuelta por etiqueta muestra su versión con un sufijo de commit de 12 caracteres, como `2.1.0-8713c5b11005`.

Las verificaciones de restricción usan la versión de la etiqueta en lugar de la `version` en `plugin.json`, incluso si `plugin.json` en ese commit se queda atrás.

Si fuerza el movimiento de una etiqueta a un commit diferente, la próxima instalación obtiene el contenido de ese commit en lugar de reutilizar una copia en caché obsoleta. Consulte [Versiones y actualizaciones](/docs/es/plugins/loading#versions-and-updates) para ver cómo la versión de un plugin se convierte en su clave de caché.

<h3 id="combine-constraints-from-several-plugins">
  Combine restricciones de varios plugins
</h3>

Cuando varios plugins instalados restringen la misma dependencia, la dependencia se resuelve a la versión más alta que satisface todos sus rangos. Las combinaciones comunes se resuelven así:

| Plugin A requiere | Plugin B requiere | Resultado                                                                                                                                      |
| :---------------- | :---------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| `^2.0`            | `>=2.1`           | Una instalación en la etiqueta `2.x` más alta en o por encima de `2.1.0`. Ambos plugins cargan.                                                |
| `~2.1`            | `~3.0`            | La instalación del plugin B falla con un mensaje `has conflicting version requirements`. El plugin A y la dependencia permanecen como estaban. |
| `=2.1.0`          | ninguno           | La dependencia permanece en `2.1.0`. La auto-actualización omite versiones más nuevas mientras el plugin A está instalado.                     |

La auto-actualización obtiene una dependencia restringida en la etiqueta git más alta que satisface el rango de cada plugin instalado, en lugar de en la versión más reciente del marketplace. Si los rangos de los plugins instalados no se superponen, la auto-actualización deja esa dependencia en su versión actual, y la pestaña **Errors** de `/plugin` muestra una entrada que nombra el plugin que restringe. Si se superponen pero ninguna etiqueta cae en el rango, la auto-actualización obtiene la copia actual del marketplace y omite la actualización cuando la `version` de esa copia cae fuera del rango de cualquier plugin instalado.

Cuando un usuario desinstala el último plugin que restringe una dependencia, la dependencia ya no está restringida a un rango de versión y reanuda el seguimiento de su entrada de marketplace en la próxima actualización.

<h2 id="see-also">
  Véase también
</h2>

* [`claude plugin prune`](/docs/es/plugins/cli-reference#plugin-prune): elimine las dependencias instaladas automáticamente que ningún plugin necesita más
* [Aloje un marketplace](/docs/es/plugins/host-marketplace): canales de lanzamiento y recomendación de otros plugins
