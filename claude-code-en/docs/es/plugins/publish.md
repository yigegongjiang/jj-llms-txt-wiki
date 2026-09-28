> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Publicar y distribuir un plugin

> Publique un plugin de Claude Code a través de su propio marketplace o del marketplace comunitario de Anthropic, con una lista de verificación previa al lanzamiento y cómo los usuarios reciben actualizaciones.

Publicar un plugin de Claude Code significa listarlo en un marketplace, un catálogo JSON que enumera plugins y dónde obtener cada uno, para que otras personas puedan instalarlo por nombre y recibir sus actualizaciones. Puede ejecutar su propio marketplace o enviar su plugin al marketplace comunitario de Anthropic. Para compartir un plugin sin publicarlo, envíe a las personas el directorio del plugin o un `.zip` del mismo para que lo carguen ellos mismos.

Esta página es para el autor de un plugin funcional que está listo para compartirlo.

<Note>
  Estos casos se tratan en otras páginas:

  * **Su plugin aún no está terminado**: comience con [Crear un plugin](/docs/es/plugins/create)
  * **Mantiene una CLI o SDK con un plugin en un marketplace oficial**: consulte [Recomendar su plugin desde su CLI](/docs/es/plugins/cli-hints)
</Note>

Comience con [Elegir cómo distribuir](#choose-how-to-distribute) para comparar las opciones de distribución. Si ya conoce su ruta, vaya a [Preparar su plugin para el lanzamiento](#prepare-your-plugin-for-release), luego siga la sección de su ruta para saber qué decirles a sus usuarios y cómo reciben sus actualizaciones.

<h2 id="choose-how-to-distribute">
  Elegir cómo distribuir
</h2>

Elija una opción de distribución según quién necesite instalar el plugin:

| Ruta                                                                         | Quién puede instalar                                                                              | Lo que necesita                                                                               | ¿Los usuarios reciben sus actualizaciones automáticamente? |
| :--------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------- | :--------------------------------------------------------- |
| [Sin marketplace](#share-a-plugin-without-a-marketplace)                     | Las personas a las que envíe la carpeta del plugin o un `.zip` del mismo                          | La carpeta del plugin                                                                         | Ninguno. Cargan la copia que envió                         |
| [Su propio marketplace](#publish-through-your-own-marketplace)               | Cualquiera que pueda acceder al repositorio, que puede ser uno privado que su equipo pueda clonar | Un repositorio git u otro host con un `.claude-plugin/marketplace.json` que enumere su plugin | Desactivado                                                |
| [Marketplace comunitario de Anthropic](#submit-to-the-community-marketplace) | Cualquiera que agregue `anthropics/claude-plugins-community`                                      | Un envío a través del formulario de envío del directorio de plugins                           | Desactivado                                                |

La actualización automática es una configuración por marketplace en el lado del usuario que obtiene nuevas versiones en segundo plano.

<h2 id="prepare-your-plugin-for-release">
  Preparar su plugin para el lanzamiento
</h2>

El nombre, la versión, la validación y una instalación desde un marketplace deciden si un lanzamiento funciona para las personas que lo instalan. Verifíquelos antes del primer lanzamiento y nuevamente antes de cada uno posterior.

<Steps>
  <Step title="Elegir un nombre permanente">
    Los usuarios instalan, habilitan y configuran su plugin por `name@marketplace`, por lo que un plugin renombrado es un plugin diferente para cada instalación existente. Elija un nombre en kebab-case como `deploy-helper`, porque `claude plugin validate` advierte sobre otras formas, y trátelo como permanente. Establezca `displayName` en `plugin.json` para la etiqueta que ven los usuarios.
  </Step>

  <Step title="Decidir cómo versionará">
    Si establece `version` en `plugin.json` y luego envía commits sin cambiarlo, `claude plugin update` imprime `<name> is already at the latest version (1.0.0).` y los usuarios mantienen la copia anterior. Incremente `version` en cada lanzamiento, u omítalo en un marketplace alojado en git para que Claude Code use el SHA del commit en su lugar. Consulte [Versiones y actualizaciones](/docs/es/plugins/loading#versions-and-updates).
  </Step>

  <Step title="Validar">
    En su shell, ejecute `claude plugin validate --strict ./your-plugin`. Una ejecución limpia imprime `✔ Validation passed`.

    * **En CI**: mantenga `--strict`, que también falla la ejecución con código de salida 1 en advertencias como un campo de manifiesto desconocido o una `version` faltante. Elimine `--strict` si eligió omitir `version` en el paso anterior.
    * **Rutas**: la validación informa rutas de componentes que no comienzan con `./`. Dentro de comandos hook y configuraciones de servidor MCP, consulte archivos como `${CLAUDE_PLUGIN_ROOT}/...`. Consulte [reglas de ruta](/docs/es/plugins/manifest-reference#path-rules).
  </Step>

  <Step title="Instalarlo desde un marketplace local">
    En su shell, agregue un marketplace local que enumere el plugin con `claude plugin marketplace add ./path-to-marketplace`, instale el plugin desde él e inicie una sesión para confirmar que se carga.

    * Para el marketplace más pequeño que funciona, consulte [Crear un marketplace](/docs/es/plugins/create-marketplace).
    * Para saber si una instalación carga su directorio de origen o una copia en caché, consulte [Plugins in-place y copiados](/docs/es/plugins/loading#in-place-and-copied-plugins).
  </Step>

  <Step title="Completar los metadatos que ven los usuarios">
    Establezca `description`, `author`, `homepage` y `repository` en `plugin.json`, y agregue un `README.md` en la raíz del plugin. `homepage` debe analizarse como una URL. La [referencia de manifiesto](/docs/es/plugins/manifest-reference#fields) enumera todos los campos.
  </Step>

  <Step title="Ejecutar su suite de evaluación">
    Si tiene una suite de evaluación, ejecute `claude plugin eval` en su shell. Ejecuta los casos de prueba del plugin y califica los resultados, lo que detecta regresiones cuando cambia el plugin. Consulte [Probar plugins con evaluaciones](/docs/es/plugin-evals).
  </Step>
</Steps>

<h2 id="share-a-plugin-without-a-marketplace">
  Compartir un plugin sin un marketplace
</h2>

Si el plugin está en un repositorio git, las personas pueden clonarlo y cargar el checkout, o iniciar Claude Code desde su shell con `--plugin-url` apuntando a un `.zip` que adjunte a un lanzamiento. Para obtener su próxima versión, extraen o descargan nuevamente. Si no está en un repositorio, envíeles el directorio o un `.zip` del mismo. Lo cargan de una de dos formas:

* **Para una sesión**: inician Claude Code desde su shell con `claude --plugin-dir ./deploy-helper`, donde la ruta es el clon, la carpeta descomprimida o el `.zip` en sí. Consulte [Banderas que cargan un plugin para una sesión](/docs/es/plugins/cli-reference#flags-that-load-a-plugin-for-one-session).
* **Para cada sesión**: mueven el directorio del plugin, con su `.claude-plugin/plugin.json`, bajo `~/.claude/skills/` para que Claude Code lo [cargue en cada sesión](/docs/es/plugins/loading#find-where-a-plugin-came-from).

Agregar un `.claude-plugin/marketplace.json` a ese mismo repositorio es lo que permite a las personas instalar por nombre y actualizar con un comando; consulte [Publicar a través de su propio marketplace](#publish-through-your-own-marketplace).

<h3 id="ship-a-plugin-with-your-own-tool">
  Enviar un plugin con su propia herramienta
</h3>

Si mantiene una CLI o SDK, publique el plugin en un marketplace y haga que su instalador o mensaje posterior a la instalación ejecute o imprima los dos comandos que necesita un usuario: `claude plugin marketplace add <source>`, luego `claude plugin install <name>@<marketplace>`. Para descubrimiento en sesión cuando alguien usa su herramienta, consulte [Recomendar su plugin desde su CLI](/docs/es/plugins/cli-hints).

<h2 id="publish-through-your-own-marketplace">
  Publicar a través de su propio marketplace
</h2>

Su propio marketplace es un archivo `.claude-plugin/marketplace.json` que enumera su plugin, agregado a un repositorio git. Una vez que el archivo está en el repositorio, el plugin se publica, sin formulario de envío. Puede mantener el archivo en el repositorio del plugin o en uno separado.

<h3 id="add-the-marketplace-file-to-your-repository">
  Agregar el archivo de marketplace a su repositorio
</h3>

Para publicar desde el repositorio del plugin, guarde el archivo de marketplace junto a `plugin.json` en `.claude-plugin/`, con una entrada cuya `source` sea `"./"`, la raíz del repositorio. Dé a la entrada el mismo `name` que `plugin.json`, según [Mantener el nombre de la entrada y el nombre del manifiesto iguales](/docs/es/plugins/create-marketplace#keep-the-entry-name-and-the-manifest-name-the-same):

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Name" },
  "plugins": [
    { "name": "deploy-helper", "source": "./" }
  ]
}
```

En su shell, ejecute `claude plugin validate .` en el repositorio para verificar el archivo antes de hacer push.

[Crear un marketplace](/docs/es/plugins/create-marketplace) cubre el diseño con varios plugins en un repositorio.

<h3 id="control-who-can-install">
  Controlar quién puede instalar
</h3>

Cualquiera que pueda clonar el repositorio puede instalar desde él, por lo que si el repositorio es privado, el marketplace también es privado. Para hosts que no sean un repositorio git, consulte [Alojar un marketplace](/docs/es/plugins/host-marketplace). Para llegar a todos en una empresa, incluidas las personas que no usan git, consulte [Implementar en toda una empresa](/docs/es/plugins/host-marketplace#roll-out-to-a-whole-company).

<h3 id="tell-users-how-to-install">
  Decirles a los usuarios cómo instalar
</h3>

Dígales a sus usuarios que agreguen el marketplace e instalen el plugin desde su shell, reemplazando la fuente y los nombres con los suyos:

* Agregue el marketplace una vez: `claude plugin marketplace add your-org/your-marketplace`, donde el argumento es un atajo de GitHub `owner/repo`, una URL o una ruta
* Instale el plugin: `claude plugin install deploy-helper@your-marketplace`
* O haga ambos desde dentro de una sesión: `/plugin install deploy-helper --marketplace your-org/your-marketplace`. Requiere Claude Code v2.1.275 o posterior. Consulte [Agregar un marketplace e instalar en un comando](/docs/es/plugins/install#add-a-marketplace-and-install-in-one-command)

<h3 id="ship-updates-to-users">
  Enviar actualizaciones a los usuarios
</h3>

Los usuarios reciben un lanzamiento cuando lo solicitan o cuando la actualización automática está activada para su marketplace:

* **Bajo solicitud**: `claude plugin update deploy-helper@your-marketplace` en el shell del usuario actualiza el marketplace e instala la nueva copia cuando la versión de su plugin ha cambiado
* **Actualización automática**: desactivada de forma predeterminada para su marketplace. Consulte [Activar la actualización automática](/docs/es/plugins/host-marketplace#turn-on-auto-update). Una vez activada, hace lo mismo que `claude plugin update` con un retraso después de que comienza la sesión

[Instalar plugins](/docs/es/plugins/install) cubre los comandos del lado del usuario, y [cuándo se ejecuta la actualización automática](/docs/es/plugins/loading#when-auto-update-runs) cubre el tiempo.

<h2 id="submit-to-the-community-marketplace">
  Enviar al marketplace comunitario
</h2>

El marketplace comunitario de Anthropic, `claude-community`, es el marketplace público que enumera plugins enviados a través del formulario de envío del directorio de plugins.

Los usuarios agregan el marketplace comunitario en una sesión de Claude Code con `/plugin marketplace add anthropics/claude-plugins-community` e instalan desde él como `@claude-community`.

Para saber cómo el marketplace comunitario difiere del marketplace oficial, consulte [Marketplaces de Anthropic](/docs/es/plugins/anthropic-marketplaces).

Para enviar su plugin al marketplace comunitario, use uno de los formularios en la aplicación:

* **claude.ai**: [claude.ai/admin-settings/directory/submissions/plugins/new](https://claude.ai/admin-settings/directory/submissions/plugins/new)
* **Console**: [platform.claude.com/plugins/submit](https://platform.claude.com/plugins/submit)

El formulario de claude.ai requiere una organización Team o Enterprise y el permiso de Directory, que los Owners tienen de forma predeterminada. Los autores individuales que no forman parte de una organización Team o Enterprise pueden usar el formulario de Console en su lugar.

En su shell, ejecute `claude plugin validate ./your-plugin` localmente antes de enviar, reemplazando `./your-plugin` con la ruta a su directorio de plugin. Cuando la validación pasa, Claude Code imprime `✔ Validation passed`, o `✔ Validation passed with warnings` si hay advertencias. Las advertencias no fallan la validación; agregue `--strict` para tratarlas como errores.

Los plugins listados aparecen en el catálogo [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community), en casi todos los casos fijados a un SHA de commit específico.

Puede haber un retraso entre el envío y la aparición de su plugin en `marketplace.json`. Para verificar si su plugin ya es instalable, busque su nombre en el [catálogo comunitario](https://github.com/anthropics/claude-plugins-community/blob/main/.claude-plugin/marketplace.json).

El marketplace oficial, `claude-plugins-official`, no acepta envíos a través de estos formularios. Si trabaja con un contacto de socio de Anthropic, pregúnteles sobre un listado en el marketplace oficial.

<h2 id="ship-updates-renames-and-removals">
  Enviar actualizaciones, cambios de nombre y eliminaciones
</h2>

<h3 id="release-a-new-version">
  Lanzar una nueva versión
</h3>

Si publica a través de su propio marketplace y su `plugin.json` establece `version`, increméntelo y haga push. Los usuarios que ejecuten `claude plugin update` o tengan la actualización automática activada reciben la nueva versión, como se describe en [Enviar actualizaciones a los usuarios](#ship-updates-to-users).

<h3 id="tag-a-release">
  Etiquetar un lanzamiento
</h3>

Etiquete el lanzamiento en git cuando otros plugins declaren un rango de versión en el suyo, porque esos rangos se resuelven contra etiquetas. De lo contrario, no necesita una etiqueta.

Para etiquetar, ejecute `claude plugin tag` en su shell desde el directorio del plugin. Crea una etiqueta `{name}--v{version}`. Agregue `--push` para enviar la etiqueta a `origin`. La [referencia de `plugin tag`](/docs/es/plugins/cli-reference#plugin-tag) enumera sus banderas.

<h3 id="rename-or-remove-a-plugin">
  Renombrar o eliminar un plugin
</h3>

Nunca cambie el `name` de un plugin publicado. Después de un cambio de nombre, los usuarios que ya lo instalaron pierden el plugin, porque su instalación se registra bajo el nombre anterior. Una entrada `renames` en su archivo de marketplace los migra en su lugar. Cambie `displayName` cuando desee una etiqueta diferente.

Si un cambio de nombre es inevitable, use el mapa `renames` del archivo de marketplace para que las instalaciones existentes se migren en lugar de fallar con [`Plugin "<name>" not found in marketplace`](/docs/es/plugins/troubleshooting#plugin-not-found-in-marketplace). Para eliminar un plugin del marketplace, o para los detalles completos de `renames`, consulte [Renombrar o eliminar un plugin](/docs/es/plugins/host-marketplace#rename-or-remove-a-plugin) en la página de alojamiento. La [referencia de marketplace](/docs/es/plugins/marketplace-reference#top-level-fields) tiene el campo.

<h2 id="declare-dependencies">
  Declarar dependencias
</h2>

Si su plugin necesita que otro plugin del mismo marketplace esté habilitado, enumérelo en la matriz `dependencies` de `plugin.json`. Cada entrada es un nombre simple o un objeto con un rango de versión semver. Cuando un usuario instala su plugin, Claude Code también instala y habilita la dependencia.

[Dependencias de plugins](/docs/es/plugins/dependencies) cubre la sintaxis de rango, dependencias entre marketplaces y cómo los usuarios eliminan dependencias que ya no necesitan.

<h2 id="next-steps">
  Próximos pasos
</h2>

* [Alojar y mantener un marketplace](/docs/es/plugins/host-marketplace): lanzar nuevas versiones y mantener a los usuarios actualizados
* [Dependencias de plugins](/docs/es/plugins/dependencies): declarar y versionar los plugins en los que depende el suyo
* [Recomendar su plugin desde su CLI](/docs/es/plugins/cli-hints): solicitar a los usuarios de Claude Code de su CLI que instalen el plugin
* [Medir el costo y el uso del plugin](/docs/es/plugins/measure): ver cuánto cuesta su plugin en contexto y si las personas lo usan
