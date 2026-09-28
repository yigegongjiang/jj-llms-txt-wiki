> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Crear un plugin de Claude Code

> Construya su primer plugin de Claude Code desde un directorio vacío, pruébelo sin un marketplace y convierta una configuración .claude/ existente.

Un plugin es un directorio de skills, agentes, hooks y servidores MCP, más un archivo `plugin.json`, llamado el manifiesto, que nombra el plugin. Claude Code carga el directorio como una unidad, por lo que puede compartirlo con compañeros de equipo, instalarlo en varios proyectos o publicarlo en un marketplace.

Esta página es para personas que escriben sus propios plugins.

<Note>
  Estos casos se cubren en otras páginas:

  * **Instalar el plugin de otra persona**: consulte [Instalar plugins](/docs/es/plugins/install)
  * **No está seguro de si necesita un plugin**: consulte [Decidir si necesita un plugin](/docs/es/plugins/overview#decide-whether-you-need-a-plugin) en la descripción general
  * **Los usuarios de su plugin están en claude.ai o en Cowork**: la misma carpeta se instala allí con un subconjunto diferente de componentes. Consulte [Plugins en claude.ai y en Cowork](https://claude.com/docs/plugins/overview)
</Note>

Comience desde la sección que coincida con lo que ya tiene:

* **Nada aún**: siga [Crear su primer plugin](#create-your-first-plugin), luego [Desarrollar sin un marketplace](#develop-without-a-marketplace) y [Probar y depurar](#test-and-debug).
* **Archivos bajo `.claude/` ya**: haga el recorrido del primer plugin una vez para aprender el diseño, luego siga [Convertir una configuración `.claude/` existente](#convert-an-existing-claude-setup).

<h2 id="decide-when-to-use-a-plugin">
  Decidir cuándo usar un plugin
</h2>

Skills, agentes, hooks y servidores MCP funcionan todos de forma independiente en su proyecto o directorio de inicio. Mantenga esa configuración independiente mientras sirva a un proyecto o solo a usted. Cree un plugin cuando desee compartir la configuración con compañeros de equipo, instalarlo en varios proyectos o publicar versiones lanzadas.

Cuando mueve skills, agentes, hooks y configuración MCP independientes a un plugin, su ubicación y nombres cambian:

* **Dónde van los archivos**: bajo el directorio propio del plugin, llamado la raíz del plugin, como `skills/`, `agents/`, `hooks/hooks.json` y `.mcp.json`.
* **Cómo se nombran**: los skills y agentes del plugin obtienen el nombre del plugin como prefijo, como `/my-plugin:hello`, por lo que dos plugins pueden proporcionar cada uno un skill `hello` sin colisionar.

Para mover una configuración existente a un plugin, consulte [Convertir una configuración `.claude/` existente](#convert-an-existing-claude-setup).

<h2 id="create-your-first-plugin">
  Crear su primer plugin
</h2>

En este recorrido, crea un plugin cuyo único componente es un skill, un saludo, y lo ejecuta con `--plugin-dir`, que carga un plugin para una sesión sin instalarlo. Un plugin puede contener cualquier mezcla de [componentes](/docs/es/plugins/components), como skills, agentes, hooks y servidores MCP, y ninguno es requerido; un skill es el ejemplo más pequeño que muestra el diseño.

Necesita Claude Code [instalado e iniciado sesión](/docs/es/quickstart#step-1-install-claude-code).

Abra una terminal en el directorio donde desea mantener el plugin, como `~/projects`, y ejecute los comandos en estos pasos desde él. Puede mantener un plugin en cualquier lugar, porque pasa su ruta a Claude Code cuando inicia una sesión.

<Steps>
  <Step title="Crear el directorio del plugin">
    Cree el directorio del plugin, con una carpeta `.claude-plugin/` dentro para mantener el manifiesto:

    ```bash theme={null}
    mkdir -p my-first-plugin/.claude-plugin
    ```
  </Step>

  <Step title="Escribir el manifiesto">
    El [manifiesto](/docs/es/plugins/manifest-reference) es un archivo JSON llamado `plugin.json` que le dice a Claude Code el nombre del plugin y lo describe. Guarde este como `my-first-plugin/.claude-plugin/plugin.json`:

    ```json my-first-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-first-plugin",
      "description": "A greeting plugin to learn the basics",
      "version": "1.0.0",
      "author": {
        "name": "Your Name"
      }
    }
    ```

    Los cuatro campos hacen esto:

    * **`name`**: requerido. Identifica el plugin y se convierte en el prefijo en cada skill y agente que proporciona el plugin. No ponga espacios en él.
    * **`description`**: el texto que los usuarios ven para el plugin en `/plugin`.
    * **`version`**: opcional. Configurarlo mantiene a los usuarios en esa versión hasta que la cambie; [Lanzar una nueva versión](/docs/es/plugins/host-marketplace#release-a-new-version) dice cuándo configurarla u omitirla.
    * **`author`**: a quién acreditar. `name` es requerido dentro de él; `email` y `url` son opcionales.

    Todos los demás campos están en la [referencia del manifiesto](/docs/es/plugins/manifest-reference#fields).

    Solo `plugin.json` va dentro de `.claude-plugin/`. El skill que agrega a continuación va directamente bajo `my-first-plugin/`, junto a esa carpeta.
  </Step>

  <Step title="Agregar un skill">
    El único componente de este plugin es un skill. Cada skill es un directorio bajo `skills/` que contiene un archivo `SKILL.md`. Cree el directorio del skill:

    ```bash theme={null}
    mkdir -p my-first-plugin/skills/hello
    ```

    Luego cree `my-first-plugin/skills/hello/SKILL.md` con este contenido:

    ```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
    ---
    name: hello
    description: Greet the user with a friendly message
    disable-model-invocation: true
    ---

    Greet the user warmly and ask how you can help them today.
    ```

    La línea `disable-model-invocation: true` significa que Claude no ejecuta el skill por su cuenta, por lo que solo usted lo activa. Elimine esa línea de un skill que desee que Claude ejecute por su cuenta. El comando del skill combina el nombre del plugin y el nombre del skill, por lo que ejecuta este como `/my-first-plugin:hello`. Para los otros campos del frontmatter, consulte la [referencia del frontmatter del skill](/docs/es/skills#frontmatter-reference).
  </Step>

  <Step title="Validar el plugin">
    Verifique el manifiesto y el frontmatter del skill antes de ejecutar nada:

    ```bash theme={null}
    claude plugin validate ./my-first-plugin
    ```

    El comando imprime la ruta del manifiesto que verificó y `✔ Validation passed`. Si imprime `✘ Validation failed` en su lugar, cada línea anterior a esa línea de resultado nombra el campo a corregir. Busque cada mensaje bajo [`claude plugin validate` reporta errores](/docs/es/plugins/troubleshooting#claude-plugin-validate-reports-errors).
  </Step>

  <Step title="Ejecutar Claude Code con el plugin">
    Inicie una sesión con el plugin cargado:

    ```bash theme={null}
    claude --plugin-dir ./my-first-plugin
    ```

    Una vez que Claude Code se inicia, ejecute el skill:

    ```text theme={null}
    /my-first-plugin:hello
    ```

    Claude responde con un saludo.
  </Step>
</Steps>

El plugin se carga solo en sesiones que inicia con `--plugin-dir`. Para continuar trabajando en él sin la bandera, o para probar una compilación `.zip`, consulte [Desarrollar sin un marketplace](#develop-without-a-marketplace).

<h3 id="share-the-plugin">
  Compartir su plugin
</h3>

Un plugin que construyó con [Crear su primer plugin](#create-your-first-plugin) existe solo en su máquina. Cuando está listo para otras personas, hay tres formas de llevarlo a ellas:

* **Enviarlo a algunas personas directamente**: déles el directorio del plugin o un `.zip` del mismo, y nada necesita ser publicado. Consulte [Compartir un plugin sin un marketplace](/docs/es/plugins/publish#share-a-plugin-without-a-marketplace).
* **Listarlo en su propio marketplace**: los compañeros de equipo agregan su marketplace una vez e instalan el plugin por nombre, y reciben sus actualizaciones. Consulte [Publicar a través de su propio marketplace](/docs/es/plugins/publish#publish-through-your-own-marketplace).
* **Enviarlo al marketplace comunitario de Anthropic**: una vez que se lista, cualquiera que agregue ese marketplace puede instalarlo. Consulte [Enviar al marketplace comunitario](/docs/es/plugins/publish#submit-to-the-community-marketplace).

<h3 id="plugin-layout">
  Diseño del plugin
</h3>

Cada tipo de [componente](/docs/es/plugins/components), como skills, agentes, hooks y servidores MCP, va en un directorio fijo bajo la raíz del plugin, que es el directorio que pasa a `--plugin-dir`. Agregue solo los directorios que use. Para hacer clic a través de un directorio de plugin completo y leer qué hace cada archivo, abra el [explorador de plugins](/docs/es/plugins/components#explore-the-plugin-directory).

La tabla enumera los directorios con los que la mayoría de los plugins comienzan, y el [diseño completo](/docs/es/plugins/manifest-reference#standard-layout) enumera el resto.

| Ubicación                    | Contenidos                                                                                                                               |
| :--------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| `.claude-plugin/plugin.json` | El manifiesto. Cuando carga un plugin con `--plugin-dir` y no tiene manifiesto, Claude Code nombra el plugin después de su directorio    |
| `skills/`                    | Un directorio `<name>/SKILL.md` por skill                                                                                                |
| `commands/`                  | Archivos Markdown planos, la forma anterior de skills. Use `skills/` para nuevos plugins                                                 |
| `agents/`                    | Un archivo Markdown por subagente                                                                                                        |
| `hooks/hooks.json`           | Configuración de hooks: una clave `"hooks"` de nivel superior cuyo valor tiene la misma forma que `hooks` en un archivo de configuración |
| `.mcp.json`                  | Definiciones de servidores MCP                                                                                                           |

<Warning>
  Solo `plugin.json` va dentro de `.claude-plugin/`. Los componentes guardados allí no se cargan.

  La raíz del plugin es el directorio propio del plugin, no `~/.claude/` en sí. Un `.mcp.json` guardado en `~/.claude/.mcp.json` no se carga.
</Warning>

<h2 id="develop-without-a-marketplace">
  Desarrollar sin un marketplace
</h2>

No necesita un [marketplace](/docs/es/plugins/overview#get-plugins-from-a-marketplace) para ejecutar un plugin que está escribiendo. Cárguelo directamente desde el disco o una URL en su lugar:

* [`--plugin-dir`](#load-a-directory-or-archive-for-one-session): carga un directorio o archivo `.zip` para una sesión.
* [`--plugin-url`](#fetch-an-archive-from-a-url-for-one-session): obtiene un archivo `.zip` de una URL para una sesión.
* [`claude plugin init`](#scaffold-a-plugin-that-loads-every-session): estructura un plugin bajo `~/.claude/skills/` que se carga cada sesión.

Si dos plugins cargados de diferentes formas comparten un nombre, consulte [Conflictos de nombres](/docs/es/plugins/loading#name-conflicts) para ver cuál mantiene Claude Code.

<h3 id="load-a-directory-or-archive-for-one-session">
  Cargar un plugin para una sesión
</h3>

Puede cargar un plugin para una sola sesión de tres formas: desde un directorio o archivo `.zip` en el disco con `--plugin-dir`, desde una URL con `--plugin-url`, o desde una variable de entorno cuando no puede agregar una bandera. Cada plugin se carga solo para esa sesión, y nada se escribe en su configuración para él. Cuando edita los archivos del plugin durante la sesión, ejecute `/reload-plugins` para cargar los cambios.

<h4 id="from-a-directory-or-zip">
  Desde un directorio o `.zip`
</h4>

Cuando inicia `claude` desde su shell, pase `--plugin-dir` con el directorio raíz del plugin o un archivo `.zip` del mismo. Repita la bandera para cargar varios plugins:

```bash theme={null}
claude --plugin-dir ./my-first-plugin --plugin-dir ./other-plugin.zip
```

<h4 id="load-a-folder-of-plugins">
  Desde una carpeta de plugins
</h4>

Para cargar varios plugins desde un lugar, pase una carpeta que los contenga, como `--plugin-dir ./plugins`. Cargar una carpeta de plugins requiere Claude Code v2.1.265 o posterior.

Si la carpeta no tiene un directorio `.claude-plugin/` y no tiene componentes de plugin en su nivel superior, Claude Code la trata como una carpeta de plugins. Cada subcarpeta inmediata que tenga un manifiesto `.claude-plugin/plugin.json` se carga como un plugin separado. Todo lo demás en la carpeta se omite sin un error, incluida una subcarpeta que no tiene manifiesto. Si un plugin en la carpeta no se carga, verifique que su subcarpeta tenga un `.claude-plugin/plugin.json`.

En una sesión interactiva, también puede agregar y eliminar plugins en la carpeta después del inicio:

* Una subcarpeta que agrega se carga como un nuevo plugin una vez que existe su manifiesto.
* Cuando elimina una subcarpeta, su plugin se descarga.

Un mensaje aparece en la sesión para cada uno de estos cambios. Si cargar o descargar un plugin a mitad de la conversación [invalidaría el caché de prompt](/docs/es/prompt-caching#enabling-or-disabling-a-plugin), el cambio se retiene en su lugar, y el mensaje le dice que ejecute `/reload-plugins` para aplicarlo.

<h4 id="fetch-an-archive-from-a-url-for-one-session">
  Desde una URL
</h4>

Cuando inicia `claude` desde su shell, pase `--plugin-url` con la dirección de un archivo `.zip`, como un artefacto de compilación que su CI publica:

```bash theme={null}
claude --plugin-url https://example.com/my-first-plugin.zip
```

Claude Code descarga el archivo al inicio. Para cargar varios, repita la bandera o pase las URLs separadas por espacios en un argumento entrecomillado.

Apunte la bandera solo a archivos que controle o en los que confíe.

Si Claude Code no puede obtener el archivo, o el archivo no es válido, se inicia sin el plugin y registra un error de carga de plugin que puede revisar en la pestaña **Errors** del administrador `/plugin`.

<h4 id="from-an-environment-variable">
  Desde una variable de entorno
</h4>

Para cargar plugins en una sesión donde no puede agregar la bandera `--plugin-dir`, enumere sus rutas absolutas en la variable de entorno [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/es/env-vars#variables) en su lugar. Claude Code carga cada ruta como carga una ruta `--plugin-dir`. Estos plugins se cargan además de cualquiera que pase con `--plugin-dir`. [La configuración del proyecto y local no puede establecer esta variable](/docs/es/settings-reference#variables-claude-code-ignores-in-env). `CLAUDE_CODE_PLUGIN_DIRS` requiere Claude Code v2.1.280 o posterior.

La configuración administrada puede desactivar `--plugin-dir` y `CLAUDE_CODE_PLUGIN_DIRS`. Consulte [Banderas que cargan un plugin para una sesión](/docs/es/plugins/cli-reference#flags-that-load-a-plugin-for-one-session). Para probar un plugin junto con un plugin del que depende, consulte [Probar un plugin y su dependencia localmente](/docs/es/plugins/dependencies#test-a-plugin-and-its-dependency-locally).

<h3 id="scaffold-a-plugin-that-loads-every-session">
  Hacer que un plugin se cargue en cada sesión
</h3>

Su directorio de skills personal es `~/.claude/skills/`. Claude Code carga cualquier carpeta allí que contenga un `.claude-plugin/plugin.json` como un plugin en cada sesión, sin bandera y sin paso de instalación. `claude plugin init` estructura uno de estos plugins para usted.

<h4 id="scaffold-the-plugin-with-claude-plugin-init">
  Estructurar el plugin con `claude plugin init`
</h4>

`claude plugin init` escribe un plugin de inicio bajo `~/.claude/skills/`. Requiere Claude Code v2.1.157 o posterior. Estructure uno desde su shell:

```bash theme={null}
claude plugin init my-tool
```

El comando crea `~/.claude/skills/my-tool/` con un `.claude-plugin/plugin.json` y un `SKILL.md` raíz. Imprime `✔ Created plugin "my-tool" at ~/.claude/skills/my-tool` seguido de `It will auto-load next session as my-tool@skills-dir. Run /reload-plugins to load it now.`

Pase `--with skills` para que `claude plugin init` estructura un skill bajo `skills/` para usted. Los otros valores `--with` están en la [referencia de comandos de plugin](/docs/es/plugins/cli-reference#plugin-init).

<h4 id="skill-names-in-a-scaffolded-plugin">
  Nombrar los skills del plugin
</h4>

El skill raíz en `~/.claude/skills/my-tool/SKILL.md` también es un skill personal, por lo que lo invoca como `/my-tool`, no `/my-tool:my-tool`. Los skills que agrega bajo `skills/` dentro del plugin obtienen el prefijo del nombre del plugin, como `/my-tool:example`.

<h4 id="stop-loading-the-plugin">
  Dejar de cargar el plugin
</h4>

Para dejar de cargar un plugin estructurado, elimine su directorio, o ejecute `claude plugin disable my-tool@skills-dir` en su shell con el nombre `my-tool@skills-dir` que `claude plugin init` imprimió. En el ID `my-tool@skills-dir`, `skills-dir` se coloca donde iría un nombre de marketplace, porque el plugin se carga desde su directorio de skills en lugar de desde un marketplace.

<h4 id="load-a-plugin-for-everyone-in-one-repository">
  Compartir el plugin a través de un repositorio
</h4>

`claude plugin init` escribe el plugin en su directorio de skills personal en `~/.claude/skills/`, por lo que se carga para usted en cada proyecto. Para hacer que un plugin se cargue para todos en un repositorio, cree el mismo diseño usted mismo en `<project>/.claude/skills/<name>/`, incluido su `.claude-plugin/plugin.json`. Consulte [Plugins compartidos a través de un repositorio](/docs/es/plugins/loading#plugins-shared-through-a-repository) para las condiciones bajo las cuales Claude Code lo carga.

<h2 id="test-and-debug">
  Probar y depurar
</h2>

Cuando un cambio en su plugin no aparece, trabaje a través de estas verificaciones en orden. Cada una le dice qué hizo Claude Code con el plugin:

1. En su shell, ejecute `claude plugin validate <path>`. Verifica el manifiesto y el frontmatter de cada archivo de skill, agente y comando, y sale con `0` en `Validation passed`. Agregue `--strict` para fallar también en advertencias. Los códigos de salida y el manejo de directorios están en la [referencia de comandos de plugin](/docs/es/plugins/cli-reference#plugin-validate).
2. En la sesión en ejecución, ejecute `/reload-plugins` para aplicar ediciones que realizó en el disco. Imprime una línea `Reloaded:` con conteos. Luego confirme que un skill se cargó escribiendo su comando `/plugin-name:skill`, o encontrando el plugin en la pestaña **Installed** de `/plugin`.
3. En la misma sesión, ejecute `/plugin`. La pestaña **Installed** enumera su plugin y, en los detalles del plugin, los componentes que Claude Code encontró. La pestaña **Errors** enumera lo que no se cargó y por qué, como una ruta en su manifiesto que no existe.
4. De vuelta en su shell, ejecute `claude plugin list`. Imprime plugins de sesión única y del directorio de skills en sus propias secciones con `Status: ✔ loaded` o el error de carga. Para incluir el plugin que está desarrollando, pase `--plugin-dir` con su ruta antes de `plugin list`.

Para verificar un servidor MCP, ejecute `/mcp` en la sesión para ver el estado del servidor. Cuando el servidor es saludable, `/mcp` lo enumera como conectado. Si no es así, consulte [Servidores MCP que no se inician](/docs/es/plugins/troubleshooting#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start).

Para verificar un hook, active el evento que coincide. Por ejemplo, pida a Claude que edite un archivo para activar un hook `PostToolUse`. Luego lea el [registro de depuración](/docs/es/hooks#debug-hooks), que muestra qué hooks coincidieron, sus códigos de salida y su salida.

Las siguientes secciones cubren los fallos que es más probable que encuentre mientras desarrolla, y la [página de solución de problemas](/docs/es/plugins/troubleshooting#build-a-plugin) tiene la entrada completa para cada uno.

<h3 id="a-component-path-isn’t-found">
  Una ruta de componente no se encuentra
</h3>

La pestaña **Errors** de `/plugin` muestra `<component> path not found: <path>`, por ejemplo `commands path not found`. Una ruta de componente en su manifiesto, como `commands`, `skills`, `agents` o `hooks`, apunta a nada. Corrija la ruta o cree el directorio, luego ejecute `/reload-plugins` en la sesión. Consulte [`commands path not found`](/docs/es/plugins/troubleshooting#commands-path-not-found).

<h3 id="plugin-dir-at-a-marketplace-root-doesn’t-load-the-plugins-under-plugins/">
  `--plugin-dir` en una raíz de marketplace no carga los plugins bajo `plugins/`
</h3>

`--plugin-dir` toma el directorio raíz del plugin, el que contiene `.claude-plugin/plugin.json` y los directorios de componentes como `skills/`. Si lo apunta a una raíz de marketplace en su lugar, Claude Code no lee `marketplace.json`, por lo que un plugin bajo `plugins/` no se carga, y no ve ningún error. Apunte la bandera a la carpeta de un plugin, o agregue el marketplace. Consulte [la entrada de solución de problemas](/docs/es/plugins/troubleshooting#plugin-dir-loads-a-plugin-with-no-components).

<h3 id="the-plugin-loads-but-its-skills-are-missing">
  El plugin se carga pero sus skills faltan
</h3>

El directorio `skills/` está dentro de `.claude-plugin/`, o una entrada `skills` en el manifiesto apunta a un archivo. Mueva `skills/` a la raíz del plugin, apunte cada entrada `skills` a un directorio que contenga `SKILL.md`, y ejecute `/reload-plugins` en la sesión. Consulte [El plugin se carga pero sus skills faltan](/docs/es/plugins/troubleshooting#plugin-loads-but-its-skills-are-missing).

<h3 id="the-userconfig-dialog-never-appears">
  El diálogo `userConfig` nunca aparece
</h3>

El diálogo para las opciones [`userConfig`](/docs/es/plugins/components#user-configuration) de su plugin es parte de la instalación a través de `/plugin` en una sesión. Cargar con `--plugin-dir` no lo muestra, ni tampoco `claude plugin install` en el shell. Con el plugin cargado, ejecute `/plugin configure <plugin-name>` en la sesión para abrirlo. Consulte [El diálogo `userConfig` nunca aparece](/docs/es/plugins/troubleshooting#the-userconfig-dialog-never-appears).

<h3 id="check-that-the-plugin-changes-claude’s-behavior">
  Verificar que el plugin cambia el comportamiento de Claude
</h3>

Un plugin que se carga sin errores aún puede no dirigir a Claude de la manera que pretende. `claude plugin eval`, que ejecuta en su shell, ejecuta sus casos de prueba con y sin el plugin y califica la diferencia. Consulte [Probar plugins con evals](/docs/es/plugin-evals), comenzando con [Crear su primer conjunto de eval](/docs/es/plugin-evals#create-your-first-eval-suite).

<h2 id="convert-an-existing-claude-setup">
  Convertir una configuración `.claude/` existente
</h2>

Si ya tiene skills, agentes o hooks bajo el directorio `.claude/` de un proyecto, puede moverlos a un plugin sin reescribirlos.

Ejecute los comandos en estos pasos desde la raíz del proyecto, que es el directorio que contiene `.claude/`, porque las rutas `cp` son relativas a él.

<Steps>
  <Step title="Crear la estructura del plugin">
    Cree el directorio del plugin y su carpeta `.claude-plugin/` junto a `.claude/`. Puede mover el plugin a cualquier lugar después.

    ```bash theme={null}
    mkdir -p my-plugin/.claude-plugin
    ```

    Cree `my-plugin/.claude-plugin/plugin.json`:

    ```json my-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-plugin",
      "description": "Migrated from standalone configuration",
      "version": "1.0.0"
    }
    ```
  </Step>

  <Step title="Copiar sus archivos existentes">
    Copie cada directorio de configuración que tenga a la raíz del plugin, y omita el comando para cualquier directorio que no tenga.

    ```bash theme={null}
    cp -r .claude/commands my-plugin/
    ```

    ```bash theme={null}
    cp -r .claude/agents my-plugin/
    ```

    ```bash theme={null}
    cp -r .claude/skills my-plugin/
    ```

    Ejecute `ls -a my-plugin` para confirmar que cada directorio que copió aparece junto a `.claude-plugin`.
  </Step>

  <Step title="Mover sus hooks">
    Si tiene hooks en `.claude/settings.json` o `.claude/settings.local.json`, cree un directorio de hooks:

    ```bash theme={null}
    mkdir -p my-plugin/hooks
    ```

    Cree `my-plugin/hooks/hooks.json` y copie el objeto `hooks` de su archivo de configuración en él. El formato es el mismo.

    Este ejemplo muestra la forma con un hook que ejecuta un linter en cada archivo que Claude escribe o edita. Reemplace el ejemplo con su propio objeto `hooks`.

    ```json my-plugin/hooks/hooks.json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npm run lint:fix" }]
          }
        ]
      }
    }
    ```
  </Step>

  <Step title="Probar el plugin migrado">
    Cargue el plugin para una sesión:

    ```bash theme={null}
    claude --plugin-dir ./my-plugin
    ```

    Verifique cada componente bajo su nuevo nombre:

    * **Skills**: ejecute `/my-plugin:deploy` para un skill que era `/deploy`.
    * **Subagentes**: pida a Claude que use el agente `my-plugin:reviewer` para un agente que era `reviewer`.
    * **Hooks**: active el evento que cada hook coincide.

    Si algo falta, trabaje a través de [Probar y depurar](#test-and-debug).
  </Step>
</Steps>

Mientras los originales aún estén bajo `.claude/`, permanecen cargados junto con las copias del plugin:

* **Skills y agentes**: los dos conjuntos no colisionan, porque los skills y agentes del plugin llevan el prefijo `my-plugin:`. `/deploy` y `/my-plugin:deploy` funcionan ambos, y Claude ve `reviewer` y `my-plugin:reviewer` como dos subagentes.
* **Hooks**: los hooks no tienen prefijo, por lo que un hook que está tanto en su archivo de configuración como en `hooks/hooks.json` se ejecuta dos veces cada vez que se activa su evento.

Después de confirmar que el plugin funciona, elimine los originales de `.claude/` y elimine el objeto `hooks` de su archivo de configuración.

<h2 id="next-steps">
  Próximos pasos
</h2>

* [Componentes de plugin](/docs/es/plugins/components): agregue agentes, hooks, servidores MCP, servidores LSP y configuración de usuario a su plugin
* [Probar plugins con evals](/docs/es/plugin-evals): escriba casos de eval y ejecútelos con `claude plugin eval` para verificar qué tan confiablemente el plugin guía el comportamiento de Claude
* [Publicar un plugin](/docs/es/plugins/publish): versione, colóquelo en un marketplace y envíelo al marketplace comunitario
* [Plugins en claude.ai y en Cowork](https://claude.com/docs/plugins/overview): la misma carpeta de plugin se instala en claude.ai y en Cowork. Algunos componentes son solo de Claude Code
* [Referencia del manifiesto del plugin](/docs/es/plugins/manifest-reference): cada campo `plugin.json`, regla de ruta y directorio
* [Skills](/docs/es/skills): escriba los skills que proporciona su plugin
* [Plugins de Anthropic en el repositorio claude-code](https://github.com/anthropics/claude-code/tree/main/plugins): ejemplos completos trabajados del diseño en esta página, como `feature-dev` y `code-review`
