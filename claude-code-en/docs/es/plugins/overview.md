> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Descripción general de plugins

> Comprenda qué es un plugin de Claude Code, cuándo necesita uno en lugar de una skill independiente o un servidor MCP, y qué página leer para instalar o crear uno.

Un plugin de Claude Code es un directorio de skills, agentes, hooks, servidores MCP u otros componentes que Claude Code instala y carga como una unidad. La mayoría de los plugins provienen de un marketplace, que es un catálogo que enumera plugins y dónde obtener cada uno. También puede cargar un plugin desde una carpeta que alguien le proporcione, o [crear el suyo propio](/docs/es/plugins/create).

<Note>
  Si utiliza el chat de claude.ai o Cowork y no Claude Code, consulte [Plugins en claude.ai y en Cowork](https://claude.com/docs/plugins/overview).
</Note>

Para probar un plugin ahora, ejecute `/plugin` en una sesión de terminal de Claude Code e instale uno desde la pestaña **Discover**, que enumera los plugins del marketplace oficial de Anthropic y cualquier marketplace que haya agregado. Desde allí:

* [Instalar y administrar plugins](/docs/es/plugins/install): los pasos de instalación completos, alcances y otras superficies
* [Crear un plugin](/docs/es/plugins/create): cree el suyo propio
* [Decidir si necesita un plugin](#decide-whether-you-need-a-plugin): si un plugin es la herramienta adecuada para lo que desea

<h2 id="understand-what-a-plugin-is">
  Comprenda qué es un plugin
</h2>

Un plugin es un directorio de componentes, generalmente con un manifiesto. El manifiesto, un archivo JSON en `.claude-plugin/plugin.json`, le da al plugin su nombre y puede agregar una versión, una descripción y otros [metadatos](/docs/es/plugins/manifest-reference). Los componentes son lo que el plugin agrega a Claude Code, como:

* [**Skills**](/docs/es/plugins/components#skills): instrucciones `SKILL.md` que Claude carga cuando es relevante, y que también puede ejecutar como un comando
* [**Agentes**](/docs/es/plugins/components#agents): definiciones de subagentes a las que Claude puede delegar
* [**Hooks**](/docs/es/plugins/components#hooks): comandos que Claude Code ejecuta en puntos de su ciclo de vida, como después de cada edición
* [**Servidores MCP**](/docs/es/plugins/components#mcp-servers): servidores de herramientas a los que Claude Code se conecta mientras el plugin está habilitado

Este diagrama muestra un plugin llamado `my-plugin` que contiene uno de cada uno de esos componentes, y lo que obtiene de cada archivo una vez que se carga el plugin.

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugin-directory.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=f623b64e82713b830e48174f0a922888" className="dark:hidden" alt="Diagram in two columns joined by five straight arrows. Left, the directory of a plugin named my-plugin, holding a manifest at .claude-plugin/plugin.json, skills/review/SKILL.md, agents/reviewer.md, hooks/hooks.json, .mcp.json, and other components. Right, what each file gives you in your session: the manifest sets the plugin name, my-plugin; the skill runs as /my-plugin:review; the agent file is a subagent Claude can delegate to; the hooks file holds hooks that run on lifecycle events; and .mcp.json adds an MCP server that gives Claude tools." width="760" height="336" data-path="images/plugin-directory.svg" />

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugin-directory-dark.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=17ee2bd45b63154fcc148ae1d1f736d8" className="hidden dark:block" alt="Diagram in two columns joined by five straight arrows. Left, the directory of a plugin named my-plugin, holding a manifest at .claude-plugin/plugin.json, skills/review/SKILL.md, agents/reviewer.md, hooks/hooks.json, .mcp.json, and other components. Right, what each file gives you in your session: the manifest sets the plugin name, my-plugin; the skill runs as /my-plugin:review; the agent file is a subagent Claude can delegate to; the hooks file holds hooks that run on lifecycle events; and .mcp.json adds an MCP server that gives Claude tools." width="760" height="336" data-path="images/plugin-directory-dark.svg" />

Para cada tipo de componente que un plugin puede contener, con un ejemplo de cada uno, consulte [Componentes de plugins](/docs/es/plugins/components). Para ver dónde se encuentra cada pieza en el directorio de un plugin, utilice el [explorador de plugins](/docs/es/plugins/components#explore-the-plugin-directory) en esa página.

<h3 id="decide-whether-you-need-a-plugin">
  Decidir si necesita un plugin
</h3>

Las skills, subagentes, hooks y servidores MCP funcionan por sí solos, sin un plugin. Una skill que guarde en `~/.claude/skills/`, por ejemplo, está disponible en cada proyecto en su máquina. Para configurar uno por su cuenta, consulte [Skills](/docs/es/skills), [Subagentes](/docs/es/sub-agents), [Hooks](/docs/es/hooks-guide) o [MCP](/docs/es/mcp).

Utilice un plugin cuando desee varias skills, subagentes, hooks o servidores MCP empaquetados como una unidad. Instale uno para obtener una configuración que alguien más construyó, con un comando y actualizaciones de su marketplace. Cree uno para dar su propia configuración a compañeros de equipo, instálelo en muchos proyectos o publique versiones con control de versiones.

<h3 id="what-an-enabled-plugin-adds-to-your-sessions">
  Lo que un plugin habilitado agrega a sus sesiones
</h3>

Un plugin habilitado es parte de cada sesión, no solo de las sesiones donde lo usa. Eso tiene algunas consecuencias que vale la pena conocer antes de instalar uno:

* **Contexto y uso**: para cada skill, agente y comando que [Claude puede invocar por su cuenta](/docs/es/skills#control-who-invokes-a-skill), el nombre y la descripción están en el contexto de Claude en cada turno para que Claude sepa que existe. Esos tokens cuentan hacia su uso y dejan menos espacio en la [ventana de contexto](/docs/es/context-window) incluso en sesiones donde nada del plugin se ejecuta. El texto completo de una skill o agente se carga solo cuando se usa. Lo que los servidores MCP del plugin agregan por turno sigue [búsqueda de herramientas MCP](/docs/es/mcp#scale-with-mcp-tool-search).
* **Procesos**: los servidores MCP que define el plugin se ejecutan junto con cada sesión donde está habilitado, y sus hooks se activan en sus eventos.
* **Permisos**: lo que ejecuta el plugin, lo ejecuta como usted. Consulte [Seguridad y confianza de plugins](/docs/es/plugins/security) para ver qué revisar primero.

Puede verificar la huella de un plugin en cada etapa:

* **Antes de instalar**: abra el plugin desde la pestaña **Marketplaces** en `/plugin`. Los plugins en el marketplace oficial de Anthropic muestran una estimación de **Context cost** allí.
* **Después de instalar**: [Medir lo que cuesta un plugin](/docs/es/plugins/measure#measure-what-a-plugin-costs) muestra cómo leer la huella de un plugin, y el grupo **Not used recently** de la pestaña **Installed** enumera los plugins que podría desactivar.
* **Para detenerlo sin desinstalarlo**: deshabilite el plugin con `/plugin` o, en su shell, `claude plugin disable`. Consulte [Administrar plugins instalados](/docs/es/plugins/install#manage-installed-plugins).

<h2 id="get-plugins-from-a-marketplace">
  Obtener plugins de un marketplace
</h2>

Un marketplace es un repositorio o directorio con un archivo `.claude-plugin/marketplace.json` que enumera plugins e indica dónde obtener cada uno. Es un catálogo, no una tienda alojada. Usted agrega un marketplace una sola vez y luego instala plugins desde él por nombre, como `commit-commands@claude-plugins-official`.

<Note>
  Un marketplace de plugins no es [Claude Marketplace](https://claude.com/marketplace). Claude Marketplace es el sitio web en claude.com/marketplace donde usted explora plugins, conectores, productos de socios y socios de servicios. No es un marketplace que usted agregue con `/plugin marketplace add`.
</Note>

Claude Code agrega el marketplace oficial de Anthropic la primera vez que usted inicia una sesión de terminal interactiva, a menos que una [política administrada](/docs/es/plugins/org#allow-the-official-marketplace-and-your-own) lo bloquee. Claude Code no agrega ningún otro marketplace por su cuenta, incluidos los marketplaces comunitarios y de demostración de Anthropic. Para distinguir los tres marketplaces de Anthropic, lea [Marketplaces de Anthropic](/docs/es/plugins/anthropic-marketplaces). Para ver lo que enumera el oficial, abra la pestaña **Discover** de `/plugin` en una sesión o explore [Claude Marketplace](https://claude.com/marketplace/plugins).

Este diagrama muestra la ruta desde un marketplace hasta su sesión. Un marketplace enumera un plugin, usted instala ese plugin, y Claude Code carga sus componentes.

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugins-model.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=4196344954b7c2e27fc0bd6a9a1113a1" className="dark:hidden" alt="Diagrama de la ruta del marketplace en tres cuadros, de izquierda a derecha. Un marketplace, un catálogo de plugins, enumera un plugin. El plugin es un directorio instalado como una unidad, que contiene skills, agentes, hooks, servidores MCP y otros componentes. Usted instala el plugin en Claude Code, que carga sus componentes." width="760" height="252" data-path="images/plugins-model.svg" />

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugins-model-dark.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=f6cdefe1fc05daf3b253d26e9f3f70f6" className="hidden dark:block" alt="Diagrama de la ruta del marketplace en tres cuadros, de izquierda a derecha. Un marketplace, un catálogo de plugins, enumera un plugin. El plugin es un directorio instalado como una unidad, que contiene skills, agentes, hooks, servidores MCP y otros componentes. Usted instala el plugin en Claude Code, que carga sus componentes." width="760" height="252" data-path="images/plugins-model-dark.svg" />

[Instalar y administrar plugins](/docs/es/plugins/install#install-a-plugin) tiene los pasos de instalación para cada lugar donde usted ejecuta Claude Code. Mientras está desarrollando un plugin, no necesita un marketplace: cárguelo directamente desde su carpeta con `--plugin-dir`, como muestra [Desarrollar sin un marketplace](/docs/es/plugins/create#develop-without-a-marketplace).

<h3 id="make-an-installed-plugin-available-in-your-session">
  Hacer que un plugin instalado esté disponible en su sesión
</h3>

Antes de que un plugin que usted instaló le proporcione una skill que pueda ejecutar, debe estar presente en cada una de estas capas:

* **Settings**: su configuración enumera los marketplaces que usted ha agregado y los plugins que están habilitados.
* **Disk**: `~/.claude/plugins/` contiene lo que Claude Code ha obtenido e instalado.
* **Session**: los plugins se cargan al inicio, o cuando usted [recarga plugins](/docs/es/plugins/loading#check-which-stage-a-plugin-reached).

Lea [Plugin loading reference](/docs/es/plugins/loading) para las reglas en cada capa, incluido qué archivo de configuración tiene prioridad y dónde están los archivos en el disco.

<h2 id="tell-anthropic’s-marketplaces-from-third-party-ones">
  Distinga los marketplaces de Anthropic de los de terceros
</h2>

El nombre de un marketplace lo coloca en uno de tres niveles. Claude Code acepta los nombres oficiales y comunitarios solo para marketplaces originarios de repositorios `github.com/anthropics/`:

* **Oficial**: marketplaces con uno de los [nombres de marketplace oficial](/docs/es/plugins/security#official-marketplace-names) de Anthropic, incluidos `claude-plugins-official` y el marketplace de demostración `claude-code-plugins`.
* **Comunitario**: marketplaces con uno de los nombres comunitarios de Anthropic, como `claude-community`. [Identificar los marketplaces de Anthropic por nombre](/docs/es/plugins/security#marketplace-tiers) los enumera.
* **Terceros**: todos los demás marketplaces. Un marketplace que publica su compañero de trabajo u organización es de terceros.

Sea cual sea el nivel, un plugin que instale puede ejecutar código con sus privilegios de usuario. Lea [Seguridad y confianza de plugins](/docs/es/plugins/security) para saber cómo revisar un plugin antes de instalarlo.

A través de [configuración administrada](/docs/es/settings#settings-files), una organización puede permitir o bloquear marketplaces, forzar la instalación de plugins y desactivar la carga solo de sesión. Lea [Administrar plugins para su organización](/docs/es/plugins/org) para esos controles.

<h2 id="understand-install-scopes">
  Comprenda los alcances de instalación
</h2>

Cuando instala un plugin, elige un alcance, y el alcance decide para quién está habilitado el plugin:

* **Alcance de usuario**: habilitado para usted en cada proyecto en esta computadora
* **Alcance de proyecto**: habilitado para todos los que trabajan en este repositorio, a través del `.claude/settings.json` confirmado. Cada colaborador aún [lo instala en su propia máquina](/docs/es/plugins/loading#enabled-in-project-settings-but-not-installed)
* **Alcance local**: habilitado para usted solo en este repositorio

Un plugin que instale en alcance de usuario en la terminal, las sesiones locales de la aplicación de escritorio o la extensión de VS Code está disponible en los otros dos en esa computadora, porque los tres leen los mismos archivos de configuración. Consulte [Elegir un alcance de instalación](/docs/es/plugins/install#choose-an-install-scope) para saber cómo elegir uno.

Una sesión en la nube, incluida una en el navegador en claude.ai/code, no carga los plugins en su configuración local. Para pasos de instalación en la terminal, VS Code y la aplicación de escritorio, y para lo que carga una sesión en la nube, consulte [Instalar un plugin](/docs/es/plugins/install#install-a-plugin).

<Note>
  El mismo formato de plugin también se instala en claude.ai y en Cowork, donde se carga un conjunto diferente de componentes. Para esas superficies, consulte [Plugins en claude.ai y en Cowork](https://claude.com/docs/plugins/overview) en claude.com.
</Note>

<h2 id="next-steps">
  Próximos pasos
</h2>

La mayoría de las personas comienzan instalando un plugin del marketplace oficial de Anthropic, que Claude Code agrega la primera vez que inicia una sesión de terminal interactiva. Ejecute `/plugin` en una sesión de terminal para explorarlo, o siga [Instalar y administrar plugins](/docs/es/plugins/install), que también cubre la aplicación de escritorio y VS Code. Para ver qué hay en ese marketplace antes de abrir Claude Code, explore [Claude Marketplace](https://claude.com/marketplace/plugins) en la web.

Para crear el suyo propio, [Crear un plugin](/docs/es/plugins/create) comienza con un directorio vacío y termina con un plugin funcional.

Una vez que haya instalado o creado un plugin, estas páginas cubren lo que viene después:

* **Compartir lo que construyó**: [Publicar y distribuir un plugin](/docs/es/plugins/publish)
* **Verificar si funciona y se usa**: [Probar plugins con evals](/docs/es/plugin-evals) y [Medir el costo y uso de plugins](/docs/es/plugins/measure)
* **Ejecutar un marketplace para su equipo**: [Crear un marketplace](/docs/es/plugins/create-marketplace), luego [Alojar y mantener un marketplace](/docs/es/plugins/host-marketplace)
* **Establecer política de plugins para una organización**: [Administrar plugins para su organización](/docs/es/plugins/org)
* **Solucionar un problema**: [Solucionar problemas de plugins](/docs/es/plugins/troubleshooting)
