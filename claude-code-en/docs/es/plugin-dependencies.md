> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Restringir versiones de dependencias de plugins

> Declare restricciones de versión en las dependencias de plugins e incluya un conjunto de plugins curado detrás de una única instalación.

Un plugin puede depender de otros plugins listándolos en `plugin.json` o en su entrada de marketplace. De forma predeterminada, una dependencia rastrea la versión más reciente disponible, por lo que un lanzamiento ascendente puede cambiar la dependencia bajo su plugin sin previo aviso. Las restricciones de versión le permiten mantener una dependencia en un rango de versión probado hasta que elija cambiar.

Cuando instala un plugin que declara dependencias, Claude Code resuelve e instala automáticamente y lista qué dependencias se agregaron al final de la salida de instalación. Si una dependencia desaparece más tarde, `/reload-plugins` y la actualización automática de plugins en segundo plano la reinstalan, siempre que su marketplace ya esté en sus marketplaces configurados. Volver a ejecutar `claude plugin install` en el plugin dependiente, o agregar un marketplace con `claude plugin marketplace add`, también resuelve cualquier dependencia faltante pendiente. Las dependencias de un marketplace que no ha agregado se dejan sin resolver.

Esta guía es para autores de plugins que declaran dependencias en `plugin.json` y para mantenedores de marketplace que etiquetan lanzamientos. Para instalar plugins que tienen dependencias, consulte [Descubrir e instalar plugins](/docs/es/discover-plugins). Para el esquema de manifiesto completo, consulte la [referencia de Plugins](/docs/es/plugins-reference).

<h2 id="why-constrain-dependency-versions">
  Por qué restringir versiones de dependencias
</h2>

Considere un marketplace interno donde dos equipos publican plugins. El equipo de plataforma mantiene `secrets-vault`, un servidor MCP que envuelve un backend de secretos. El equipo de implementación mantiene `deploy-kit`, que llama a `secrets-vault` para obtener credenciales durante las implementaciones.

`deploy-kit` se prueba contra `secrets-vault` v2.1.0. Sin una restricción de versión, la próxima vez que el equipo de plataforma etiquete un lanzamiento que renombre una herramienta MCP, la actualización automática mueve `secrets-vault` de cada ingeniero a la nueva versión y `deploy-kit` se rompe.

Con una restricción de versión, `deploy-kit` declara que necesita `secrets-vault` en el rango `~2.1.0`. Los ingenieros con `deploy-kit` instalado permanecen en el parche `2.1.x` más alto que coincida. El equipo de implementación se actualiza en su propio cronograma publicando una nueva versión de `deploy-kit` con una restricción más amplia.

<h2 id="declare-a-dependency-with-a-version-constraint">
  Declarar una dependencia con una restricción de versión
</h2>

Liste las dependencias en el array `dependencies` del `plugin.json` de su plugin. Cada entrada es un nombre de plugin u objeto con una restricción de versión.

El siguiente manifiesto declara una dependencia sin versión y una dependencia restringida:

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

Una entrada puede ser una cadena simple con solo el nombre del plugin, como `"audit-logger"` en el ejemplo anterior, que depende de cualquier versión que proporcione el marketplace de ese plugin. Para más control, use un objeto con estos campos:

| Field         | Type   | Description                                                                                                                                                                                                                                                                                       |
| :------------ | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`        | string | Nombre del plugin. Se resuelve dentro del mismo marketplace que el plugin declarante. Requerido.                                                                                                                                                                                                  |
| `version`     | string | Un [rango semver](https://github.com/npm/node-semver#ranges) como `~2.1.0`, `^2.0`, `>=1.4`, o `=2.1.0`. La dependencia se obtiene en la versión etiquetada más alta que satisface este rango.                                                                                                    |
| `marketplace` | string | Un marketplace diferente para resolver `name` en. Las dependencias entre marketplaces están bloqueadas a menos que el marketplace de destino esté listado en [`allowCrossMarketplaceDependenciesOn`](#depend-on-a-plugin-from-another-marketplace) en el `marketplace.json` del marketplace raíz. |

El campo `version` acepta cualquier expresión soportada por el paquete `semver` de Node, incluyendo rangos de circunflejo, tilde, guión y comparador. Las versiones previas al lanzamiento como `2.0.0-beta.1` se excluyen a menos que su rango opte por un sufijo previo al lanzamiento como `^2.0.0-0`.

<h2 id="bundle-plugins-for-a-team">
  Agrupar plugins para un equipo
</h2>

Además del `name` requerido, un manifiesto de plugin puede consistir únicamente en un array `dependencies`. Instalarlo extrae todas las dependencias, lo que lo convierte en una forma de empaquetar un conjunto de plugins curado detrás de una única instalación.

Por ejemplo, un equipo de plataforma puede publicar paquetes específicos de roles en un marketplace interno para que los ingenieros ejecuten un único `claude plugin install` en lugar de instalar cada herramienta por separado:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "backend-standard",
  "version": "1.0.0",
  "description": "Standard plugin set for backend engineers",
  "dependencies": [
    "secrets-vault",
    "deploy-kit",
    { "name": "db-migrate", "version": "^3.0" },
    "oncall-runbook"
  ]
}
```

Instalar `backend-standard` resuelve e instala las cuatro dependencias.

Para agregar una herramienta al conjunto estándar más adelante, publique una nueva versión de `backend-standard` con la dependencia adicional. La actualización automática está desactivada de forma predeterminada para marketplaces que no son de Anthropic, por lo que los ingenieros adoptan la nueva versión de una de dos formas:

* Habilite la actualización automática para el marketplace en `/plugin`. La siguiente actualización automática mueve el paquete a la nueva versión e instala cualquier dependencia que agregue.
* Ejecute `claude plugin update backend-standard`, luego `/reload-plugins` para instalar las dependencias recién agregadas.

Para implementar paquetes en toda una organización, agregue el plugin de paquete a `enabledPlugins` en [configuración administrada](/docs/es/settings#enabledplugins).

<h2 id="depend-on-a-plugin-from-another-marketplace">
  Depender de un plugin de otro marketplace
</h2>

De forma predeterminada, Claude Code se niega a instalar automáticamente una dependencia que vive en un marketplace diferente al del plugin que la declara. Esto evita que un marketplace extraiga silenciosamente plugins de una fuente que no ha revisado.

Para permitirlo, el mantenedor del marketplace raíz agrega el nombre del marketplace de destino a `allowCrossMarketplaceDependenciesOn` en `marketplace.json`. El marketplace raíz es el que aloja el plugin que el usuario está instalando; solo se consulta su lista de permitidos, por lo que la confianza no se encadena a través de marketplaces intermedios.

El siguiente `marketplace.json` permite que `deploy-kit` dependa de un plugin de `acme-shared`:

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "acme-tools",
  "owner": { "name": "Acme" },
  "allowCrossMarketplaceDependenciesOn": ["acme-shared"],
  "plugins": [
    {
      "name": "deploy-kit",
      "source": "./deploy-kit",
      "dependencies": [
        { "name": "audit-logger", "marketplace": "acme-shared" }
      ]
    }
  ]
}
```

Si el campo falta o no incluye el marketplace de destino, la instalación falla con un error `cross-marketplace` que nombra el campo a establecer. Los usuarios aún pueden instalar la dependencia manualmente primero, lo que satisface la restricción sin cambiar la lista de permitidos.

<h2 id="tag-plugin-releases-for-version-resolution">
  Etiquetar lanzamientos de plugins para la resolución de versiones
</h2>

Las restricciones de versión se resuelven contra etiquetas de git en el repositorio del marketplace. Para que Claude Code encuentre las versiones disponibles de una dependencia, los lanzamientos del plugin ascendente deben etiquetarse usando una convención de nomenclatura específica.

Etiquete cada lanzamiento como `{plugin-name}--v{version}`, donde `{version}` coincide con el campo `version` en el `plugin.json` de ese commit. Desde el directorio del plugin, ejecute:

```bash theme={null}
claude plugin tag --push
```

El comando `claude plugin tag` deriva el nombre de la etiqueta del manifiesto del plugin y de la entrada del marketplace que lo contiene. Antes de crear la etiqueta, valida el contenido del plugin, verifica que `plugin.json` y la entrada del marketplace coincidan en la versión, requiere un árbol de trabajo limpio bajo el directorio del plugin, y rechaza si la etiqueta ya existe. Agregue `--dry-run` para ver qué se etiquetaría sin crearlo. Ejecutar `git tag secrets-vault--v2.1.0` directamente es equivalente si mantiene `plugin.json` y la entrada del marketplace sincronizados usted mismo.

El prefijo del nombre del plugin permite que un repositorio de marketplace aloje múltiples plugins con líneas de versión independientes. El separador `--v` se analiza como una coincidencia de prefijo en el nombre completo del plugin, por lo que los nombres de plugins que contienen guiones se manejan correctamente.

Cuando instala un plugin que declara `{ "name": "secrets-vault", "version": "~2.1.0" }`, Claude Code lista las etiquetas del marketplace, filtra las que comienzan con `secrets-vault--v`, y obtiene la versión más alta que satisface `~2.1.0`. Si no existe una etiqueta coincidente, el plugin dependiente se deshabilita con un error que lista las versiones disponibles.

Un marketplace agregado como una ruta de carpeta local resuelve etiquetas de la misma manera cuando la carpeta es un repositorio de git. Esto requiere Claude Code v2.1.196 o posterior. En dos casos Claude Code instala la dependencia desde el contenido actual de la carpeta en su lugar:

* Las versiones anteriores no leen etiquetas de un marketplace de carpeta local, por lo que una dependencia restringida se carga solo si esa copia satisface el rango.
* Una carpeta local que no es un repositorio de git no tiene etiquetas, independientemente de la versión.

La versión semver de la etiqueta resuelta se registra por separado del `version` de `plugin.json`, por lo que las verificaciones de restricción utilizan la etiqueta que se obtuvo realmente incluso si el `plugin.json` en ese commit tiene un valor obsoleto. El nombre del directorio de caché para una instalación resuelta por etiqueta incluye un sufijo SHA de commit de 12 caracteres, por lo que si un mantenedor mueve forzadamente una etiqueta a un commit diferente, la siguiente instalación obtiene un directorio de caché nuevo en lugar de reutilizar contenido obsoleto.

<Note>
  Para fuentes de marketplace `npm`, la restricción no controla qué versión se obtiene, ya que la resolución basada en etiquetas se aplica solo a fuentes respaldadas por git. La restricción aún se verifica en tiempo de carga, y el plugin dependiente se deshabilita con `dependency-version-unsatisfied` si la versión instalada no la satisface.
</Note>

<h2 id="how-constraints-interact">
  Cómo interactúan las restricciones
</h2>

Cuando varios plugins instalados restringen la misma dependencia, Claude Code intersecta sus rangos y resuelve la dependencia a la versión más alta que satisface todos ellos. La tabla a continuación muestra cómo se resuelven las combinaciones comunes.

| Plugin A requiere | Plugin B requiere | Resultado                                                                                                                        |
| :---------------- | :---------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| `^2.0`            | `>=2.1`           | Una instalación en la etiqueta `2.x` más alta en o por encima de `2.1.0`. Ambos plugins se cargan.                               |
| `~2.1`            | `~3.0`            | La instalación del plugin B falla con `range-conflict`. El plugin A y la dependencia permanecen como estaban.                    |
| `=2.1.0`          | ninguno           | La dependencia permanece en `2.1.0`. La actualización automática omite versiones más nuevas mientras el plugin A está instalado. |

La actualización automática obtiene una dependencia restringida en la etiqueta git más alta que satisface el rango de cada plugin instalado, en lugar de obtenerla en la versión más reciente del marketplace, por lo que la dependencia continúa recibiendo actualizaciones dentro de su rango permitido. Si ninguna etiqueta satisface todos los rangos, la actualización automática omite esa dependencia y enumera la omisión en la pestaña Errores de `/plugin`, nombrando el plugin que la restringe.

Cuando desinstala el último plugin que restringe una dependencia, la dependencia ya no se mantiene y reanuda el seguimiento de su entrada de marketplace en la próxima actualización.

<h2 id="enable-or-disable-a-plugin-with-dependencies">
  Habilitar o deshabilitar un plugin con dependencias
</h2>

Habilitar un plugin también habilita los plugins de los que depende, y deshabilitar un plugin se bloquea si otro plugin habilitado aún lo necesita. Ambos comportamientos requieren Claude Code v2.1.143 o posterior. Las versiones anteriores habilitan o deshabilitan solo el plugin nombrado y muestran un error `dependency-unsatisfied` en la próxima carga.

Cuando habilita un plugin, Claude Code también habilita sus dependencias en el mismo ámbito. Si una dependencia tiene sus propias dependencias, Claude Code también las habilita. El mensaje de éxito lista qué más se habilitó junto con el plugin que nombró. Si una dependencia no se puede habilitar, el comando se rechaza y le dice qué está bloqueando y cómo solucionarlo:

| Condición                                                                                           | Resultado                                                                                                                           |
| :-------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------- |
| Una dependencia no está instalada                                                                   | La habilitación falla e imprime el comando `claude plugin install` para cada dependencia faltante.                                  |
| Una dependencia está bloqueada por la política de plugins de su organización                        | La habilitación falla y nombra la dependencia bloqueada.                                                                            |
| Una dependencia se establece en `false` en un ámbito con mayor precedencia que el ámbito de destino | La habilitación falla. Habilite la dependencia en ese ámbito, o pase `--scope` para escribir allí.                                  |
| Todas las dependencias están instaladas y permitidas                                                | La habilitación tiene éxito y escribe `true` para el plugin y cada dependencia que no estaba ya habilitada en el ámbito de destino. |

Esto se mantiene incluso cuando una dependencia establece [`defaultEnabled: false`](/docs/es/plugins-reference#default-enablement) en su manifiesto, porque Claude Code escribe un `true` explícito para ella. Lo mismo se aplica en la instalación: una dependencia extraída para satisfacer un plugin activo se instala con `true` independientemente de su propio valor predeterminado.

Cuando deshabilita un plugin, Claude Code se rechaza si otro plugin habilitado aún depende de él. El error nombra los plugins que dependen de él y le da un comando encadenado que los deshabilita en el orden correcto, terminando con el que pidió.

Por ejemplo, si `deploy-kit` depende de `secrets-vault`, deshabilitar `secrets-vault` solo falla con una salida similar a la siguiente:

```text theme={null}
secrets-vault is still required by deploy-kit. Disable that plugin first, or
disable everything together: claude plugin disable deploy-kit@acme-tools && claude plugin disable secrets-vault@acme-tools
```

Copie el comando encadenado del error para deshabilitar el conjunto completo en un paso.

<h2 id="remove-orphaned-auto-installed-dependencies">
  Eliminar dependencias auto-instaladas huérfanas
</h2>

Las dependencias auto-instaladas permanecen en el disco después de que se desinstalan los plugins que las instalaron, en caso de que desee reinstalar un plugin dependiente o desee seguir usando la dependencia directamente. Para limpiarlas, ejecute `claude plugin prune` para listar las dependencias auto-instaladas que ya no tienen ningún plugin instalado que las requiera y eliminarlas después de un mensaje de confirmación. Esto requiere Claude Code v2.1.121 o posterior.

```bash theme={null}
claude plugin prune
```

De forma predeterminada, prune opera en el ámbito del usuario. Use `--scope project` o `--scope local` para dirigirse a un ámbito diferente. Pase `--dry-run` para listar qué se eliminaría sin cambiar nada. Pase `-y` para omitir el mensaje de confirmación. Cuando stdin o stdout no es una terminal, prune lista los huérfanos y sale sin eliminarlos a menos que se pase `-y`.

Para prune como parte de una desinstalación, pase `--prune` a `claude plugin uninstall`. Después de eliminar el plugin nombrado, Claude Code escanea y elimina cualquier dependencia auto-instalada que ahora esté huérfana. Los plugins que instaló usted mismo nunca se podan, solo los instalados automáticamente a través del array `dependencies` de otro plugin.

Por ejemplo, para desinstalar `deploy-kit` y limpiar las dependencias que deja atrás:

```bash theme={null}
claude plugin uninstall deploy-kit --prune
```

<h2 id="resolve-dependency-errors">
  Resolver errores de dependencia
</h2>

Los problemas de dependencia aparecen en `claude plugin list` y en la interfaz `/plugin`. Claude Code deshabilita el plugin afectado hasta que resuelva el error. La tabla a continuación enumera los errores más comunes y cómo resolverlos.

| Error                            | Significado                                                                                                                                                                                                                                                         | Cómo resolver                                                                                                                                                                                                                                                                                         |
| :------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `dependency-unsatisfied`         | Una dependencia declarada no está instalada, o está instalada pero deshabilitada.                                                                                                                                                                                   | Ejecute el comando `claude plugin install` que se muestra en el mensaje de error. Si el marketplace de la dependencia aún no está configurado, agréguelo con `claude plugin marketplace add` y Claude Code resuelve la dependencia automáticamente. Si la dependencia está deshabilitada, habilítela. |
| `range-conflict`                 | Los requisitos de versión para una dependencia no se pueden combinar. El mensaje de error nombra la causa: ninguna versión satisface todos los rangos, un rango no es una sintaxis semver válida, o los rangos combinados son demasiado complejos para intersectar. | Desinstale o actualice uno de los plugins en conflicto, corrija cualquier cadena `version` inválida, simplifique cadenas `\|\|` largas, o pida al autor ascendente que amplíe su restricción.                                                                                                         |
| `dependency-version-unsatisfied` | La versión de la dependencia instalada está fuera del rango declarado de este plugin.                                                                                                                                                                               | Ejecute `claude plugin install <dependency>@<marketplace>` para re-resolver la dependencia contra todas las restricciones actuales.                                                                                                                                                                   |
| `no-matching-tag`                | El repositorio de la dependencia no tiene una etiqueta `{name}--v*` que satisfaga el rango.                                                                                                                                                                         | Verifique que el ascendente haya etiquetado lanzamientos usando la convención anterior, o relaje su rango.                                                                                                                                                                                            |

Para verificar estos errores mediante programación, ejecute `claude plugin list --json` y lea el campo `errors` en cada plugin.

<h2 id="see-also">
  Ver también
</h2>

* [Crear plugins](/docs/es/plugins): construir plugins con skills, agentes y hooks
* [Crear y distribuir un marketplace de plugins](/docs/es/plugin-marketplaces): alojar plugins para su equipo
* [Referencia de Plugins](/docs/es/plugins-reference#plugin-manifest-schema): el esquema completo de `plugin.json`
* [Gestión de versiones](/docs/es/plugins-reference#version-management): cómo se resuelve la versión propia de un plugin y se utiliza como clave de caché
