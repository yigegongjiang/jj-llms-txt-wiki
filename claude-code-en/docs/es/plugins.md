> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Crear plugins

> Crea plugins personalizados para extender Claude Code con skills, agentes, hooks y servidores MCP.

Los plugins le permiten extender Claude Code con funcionalidad personalizada que se puede compartir entre proyectos y equipos. Esta guía cubre la creación de sus propios plugins con skills, agentes, hooks y servidores MCP.

¿Buscando instalar plugins existentes? Consulte [Descubrir e instalar plugins](/docs/es/discover-plugins). Para especificaciones técnicas completas, consulte [Referencia de plugins](/docs/es/plugins-reference).

<h2 id="when-to-use-plugins-vs-standalone-configuration">
  Cuándo usar plugins versus configuración independiente
</h2>

Claude Code admite dos formas de agregar skills, agentes y hooks personalizados:

| Enfoque                                                                                           | Nombres de skills    | Mejor para                                                                                                                   |
| :------------------------------------------------------------------------------------------------ | :------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| **Independiente** (directorio `.claude/`)                                                         | `/hello`             | Flujos de trabajo personales, personalizaciones específicas del proyecto, experimentos rápidos                               |
| **Plugins** (directorios con skills, agentes, hooks o un manifiesto `.claude-plugin/plugin.json`) | `/plugin-name:hello` | Compartir con compañeros de equipo, distribuir a la comunidad, lanzamientos versionados, reutilizable en múltiples proyectos |

**Use configuración independiente cuando**:

* Esté personalizando Claude Code para un único proyecto
* La configuración es personal y no necesita ser compartida
* Esté experimentando con skills o hooks antes de empaquetarlos
* Quiera nombres de skills cortos como `/hello` o `/deploy`

**Use plugins cuando**:

* Quiera compartir funcionalidad con su equipo o comunidad
* Necesite los mismos skills/agentes en múltiples proyectos
* Quiera control de versiones y actualizaciones fáciles para sus extensiones
* Esté distribuyendo a través de un marketplace
* Esté de acuerdo con skills con espacios de nombres como `/my-plugin:hello` (los espacios de nombres previenen conflictos entre plugins)

<Tip>
  Comience con configuración independiente en `.claude/` para iteración rápida, luego [convierta a un plugin](#convert-existing-configurations-to-plugins) cuando esté listo para compartir.
</Tip>

<h2 id="quickstart">
  Inicio rápido
</h2>

Este inicio rápido le guía a través de la creación de un plugin con un skill personalizado. Creará un manifiesto (el archivo de configuración que define su plugin), agregará un skill y lo probará localmente usando la bandera `--plugin-dir`.

<h3 id="prerequisites">
  Requisitos previos
</h3>

* Claude Code [instalado y autenticado](/docs/es/quickstart#step-1-install-claude-code)

<Note>
  Si no ve el comando `/plugin`, actualice Claude Code a la última versión. Consulte [Troubleshooting](/docs/es/troubleshooting) para obtener instrucciones de actualización.
</Note>

<h3 id="create-your-first-plugin">
  Cree su primer plugin
</h3>

<Steps>
  <Step title="Cree el directorio del plugin">
    Cada plugin vive en su propio directorio que contiene sus skills, agentes o hooks, opcionalmente junto con un manifiesto `.claude-plugin/plugin.json`. La ubicación no importa para este inicio rápido porque apuntará Claude Code al directorio con `--plugin-dir` en el paso de prueba. Créelo en cualquier lugar conveniente, como una carpeta temporal o un directorio de proyectos:

    ```bash theme={null}
    mkdir my-first-plugin
    ```

    Los pasos restantes se ejecutan desde el directorio padre y hacen referencia a rutas como `my-first-plugin/...` relativas a él.
  </Step>

  <Step title="Cree el manifiesto del plugin">
    El archivo de manifiesto en `.claude-plugin/plugin.json` define la identidad de su plugin: su nombre, descripción y versión. Claude Code usa estos metadatos para mostrar su plugin en el administrador de plugins.

    Cree el directorio `.claude-plugin` dentro de su carpeta de plugin:

    ```bash theme={null}
    mkdir my-first-plugin/.claude-plugin
    ```

    Luego cree `my-first-plugin/.claude-plugin/plugin.json` con este contenido:

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

    | Campo         | Propósito                                                                                                                                                                                                                                                                                                        |
    | :------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `name`        | Identificador único y espacio de nombres de skill. Los skills tienen este prefijo (por ejemplo, `/my-first-plugin:hello`).                                                                                                                                                                                       |
    | `description` | Se muestra en el administrador de plugins al examinar o instalar plugins.                                                                                                                                                                                                                                        |
    | `version`     | Opcional. Si se establece, los usuarios solo reciben actualizaciones cuando usted incrementa este campo. Si se omite y su plugin se distribuye a través de git, se usa el SHA del commit y cada commit cuenta como una nueva versión. Consulte [gestión de versiones](/docs/es/plugins-reference#version-management). |
    | `author`      | Opcional. Útil para atribución.                                                                                                                                                                                                                                                                                  |

    Para campos adicionales como `homepage`, `repository` y `license`, consulte el [esquema de manifiesto completo](/docs/es/plugins-reference#plugin-manifest-schema).
  </Step>

  <Step title="Agregue un skill">
    Los skills viven en el directorio `skills/`. Cada skill es una carpeta que contiene un archivo `SKILL.md`. El nombre de la carpeta se convierte en el nombre del skill, con el prefijo del espacio de nombres del plugin (`hello/` en un plugin llamado `my-first-plugin` crea `/my-first-plugin:hello`).

    Cree un directorio de skill en su carpeta de plugin:

    ```bash theme={null}
    mkdir -p my-first-plugin/skills/hello
    ```

    Luego cree `my-first-plugin/skills/hello/SKILL.md` con este contenido:

    ```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
    ---
    description: Greet the user with a friendly message
    disable-model-invocation: true
    ---

    Greet the user warmly and ask how you can help them today.
    ```
  </Step>

  <Step title="Pruebe su plugin">
    Ejecute Claude Code con la bandera `--plugin-dir` para cargar su plugin:

    ```bash theme={null}
    claude --plugin-dir ./my-first-plugin
    ```

    Una vez que Claude Code se inicie, pruebe su nuevo skill:

    ```shell theme={null}
    /my-first-plugin:hello
    ```

    Verá que Claude responde con un saludo. Ejecute `/help` para ver su skill listado bajo el espacio de nombres del plugin.

    <Note>
      **¿Por qué espacios de nombres?** Los skills de plugin siempre tienen espacios de nombres (como `/my-first-plugin:hello`) para prevenir conflictos cuando múltiples plugins tienen skills con el mismo nombre.

      Para cambiar el prefijo del espacio de nombres, actualice el campo `name` en `plugin.json`.
    </Note>
  </Step>

  <Step title="Agregue argumentos de skill">
    Haga su skill dinámico aceptando entrada del usuario. El marcador de posición `$ARGUMENTS` captura cualquier texto que el usuario proporcione después del nombre del skill.

    Actualice su archivo `SKILL.md`:

    ```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
    ---
    description: Greet the user with a personalized message
    ---

    # Hello Skill

    Greet the user named "$ARGUMENTS" warmly and ask how you can help them today. Make the greeting personal and encouraging.
    ```

    Ejecute `/reload-plugins` para recoger los cambios, luego pruebe el skill con su nombre:

    ```shell theme={null}
    /my-first-plugin:hello Alex
    ```

    Claude le saludará por su nombre. Para más información sobre pasar argumentos a skills, consulte [Skills](/docs/es/skills#pass-arguments-to-skills).
  </Step>
</Steps>

Ha creado y probado exitosamente un plugin con estos componentes clave:

* **Manifiesto del plugin** (`.claude-plugin/plugin.json`): describe los metadatos de su plugin
* **Directorio de skills** (`skills/`): contiene sus skills personalizados
* **Argumentos de skill** (`$ARGUMENTS`): captura entrada del usuario para comportamiento dinámico

<Tip>
  La bandera `--plugin-dir` es útil para desarrollo y pruebas. Cuando esté listo para compartir su plugin con otros, consulte [Crear y distribuir un marketplace de plugins](/docs/es/plugin-marketplaces).
</Tip>

<h2 id="develop-a-plugin-in-your-skills-directory">
  Desarrolle un plugin en su directorio de skills
</h2>

En lugar de pasar `--plugin-dir` en cada lanzamiento, puede mantener un plugin en su directorio de skills y hacer que Claude Code lo cargue automáticamente. `claude plugin init` lo estructura:

```bash theme={null}
claude plugin init my-tool
```

Esto crea `~/.claude/skills/my-tool/` con un manifiesto `.claude-plugin/plugin.json` y un `SKILL.md` inicial. En la siguiente sesión se carga como `my-tool@skills-dir` sin necesidad de marketplace o paso de instalación.

Para las reglas de carga automática, alcance personal versus de proyecto, el requisito de confianza del espacio de trabajo y cómo actualizar o eliminar uno, consulte [Plugins del directorio de skills](/docs/es/plugins-reference#skills-directory-plugins).

<h2 id="plugin-structure-overview">
  Descripción general de la estructura del plugin
</h2>

Ha creado un plugin con un skill, pero los plugins pueden incluir mucho más: agentes personalizados, hooks, servidores MCP, servidores LSP y monitores de fondo.

<Warning>
  **Error común**: No ponga `commands/`, `agents/`, `skills/` o `hooks/` dentro del directorio `.claude-plugin/`. Solo `plugin.json` va dentro de `.claude-plugin/`. Todos los otros directorios deben estar en el nivel raíz del plugin.

  La raíz del plugin es el directorio propio del plugin individual: el que contiene `.claude-plugin/plugin.json`. Nunca es `~/.claude/`. Por ejemplo, Claude Code no lee un `.mcp.json` colocado en `~/.claude/.mcp.json`.
</Warning>

| Directorio        | Ubicación       | Propósito                                                                                           |
| :---------------- | :-------------- | :-------------------------------------------------------------------------------------------------- |
| `.claude-plugin/` | Raíz del plugin | Contiene el manifiesto `plugin.json` (opcional si los componentes usan ubicaciones predeterminadas) |
| `skills/`         | Raíz del plugin | Skills como directorios `<name>/SKILL.md`                                                           |
| `commands/`       | Raíz del plugin | Skills como archivos Markdown planos. Use `skills/` para plugins nuevos                             |
| `agents/`         | Raíz del plugin | Definiciones de agentes personalizados                                                              |
| `hooks/`          | Raíz del plugin | Manejadores de eventos en `hooks.json`                                                              |
| `.mcp.json`       | Raíz del plugin | Configuraciones de servidor MCP                                                                     |
| `.lsp.json`       | Raíz del plugin | Configuraciones de servidor LSP para inteligencia de código                                         |
| `monitors/`       | Raíz del plugin | Configuraciones de monitor de fondo en `monitors.json`                                              |
| `bin/`            | Raíz del plugin | Ejecutables agregados a la `PATH` de la herramienta Bash mientras el plugin está habilitado         |
| `settings.json`   | Raíz del plugin | [Configuraciones](/docs/es/settings) predeterminadas aplicadas cuando el plugin está habilitado          |

Un plugin que incluye exactamente un skill puede colocar `SKILL.md` directamente en la raíz del plugin en lugar de crear un directorio `skills/`. Claude Code lo carga como un único skill y utiliza el campo `name` del frontmatter para el nombre de invocación. Use el diseño `skills/` para plugins que pueden crecer a más de un skill.

<Note>
  **Próximos pasos**: ¿Listo para agregar más características? Salte a [Desarrollar plugins más complejos](#develop-more-complex-plugins) para agregar agentes, hooks, servidores MCP y servidores LSP. Para especificaciones técnicas completas de todos los componentes del plugin, consulte [Referencia de plugins](/docs/es/plugins-reference).
</Note>

<h2 id="develop-more-complex-plugins">
  Desarrollar plugins más complejos
</h2>

Una vez que se sienta cómodo con plugins básicos, puede crear extensiones más sofisticadas.

<h3 id="add-skills-to-your-plugin">
  Agregue Skills a su plugin
</h3>

Los plugins pueden incluir [Agent Skills](/docs/es/skills) para extender las capacidades de Claude. Los skills son invocados por el modelo: Claude los usa automáticamente basándose en el contexto de la tarea.

Agregue un directorio `skills/` en la raíz de su plugin con carpetas de Skill que contengan archivos `SKILL.md`:

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json
└── skills/
    └── code-review/
        └── SKILL.md
```

Cada `SKILL.md` contiene frontmatter YAML e instrucciones. Incluya una `description` para que Claude sepa cuándo usar el skill:

```yaml theme={null}
---
description: Reviews code for best practices and potential issues. Use when reviewing code, checking PRs, or analyzing code quality.
---

When reviewing code, check for:
1. Code organization and structure
2. Error handling
3. Security concerns
4. Test coverage
```

Después de instalar el plugin, ejecute `/reload-plugins` para cargar los Skills. Para orientación completa sobre la autoría de Skills incluyendo divulgación progresiva y restricciones de herramientas, consulte [Agent Skills](/docs/es/skills).

<h3 id="add-lsp-servers-to-your-plugin">
  Agregue servidores LSP a su plugin
</h3>

<Tip>
  Para lenguajes comunes como TypeScript, Python y Rust, instale los plugins LSP precompilados desde el marketplace oficial. Cree plugins LSP personalizados solo cuando necesite soporte para lenguajes que aún no están cubiertos.
</Tip>

Los plugins LSP (Language Server Protocol) dan a Claude inteligencia de código en tiempo real. Si necesita soportar un lenguaje que no tiene un plugin LSP oficial, puede crear uno propio agregando un archivo `.lsp.json` a su plugin:

```json .lsp.json theme={null}
{
  "go": {
    "command": "gopls",
    "args": ["serve"],
    "extensionToLanguage": {
      ".go": "go"
    }
  }
}
```

Los usuarios que instalen su plugin deben tener el binario del servidor de lenguaje instalado en su máquina.

Para opciones de configuración LSP completas, consulte [Servidores LSP](/docs/es/plugins-reference#lsp-servers).

<h3 id="add-background-monitors-to-your-plugin">
  Agregue monitores de fondo a su plugin
</h3>

Los monitores de fondo permiten que su plugin observe registros, archivos o estado externo en el fondo y notifique a Claude cuando lleguen eventos. Claude Code inicia cada monitor automáticamente cuando el plugin está activo, por lo que no necesita instruir a Claude para que inicie la observación.

Agregue un archivo `monitors/monitors.json` en la raíz del plugin con una matriz de entradas de monitor:

```json monitors/monitors.json theme={null}
[
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "Application error log"
  }
]
```

Cada línea de stdout del `command` se entrega a Claude como una notificación durante la sesión. Para el esquema completo, incluyendo el disparador `when` y la sustitución de variables, consulte [Monitors](/docs/es/plugins-reference#monitors).

<h3 id="ship-default-settings-with-your-plugin">
  Envíe configuraciones predeterminadas con su plugin
</h3>

Los plugins pueden incluir un archivo `settings.json` en la raíz del plugin para aplicar configuración predeterminada cuando el plugin está habilitado. Actualmente, solo se admiten las claves `agent` y `subagentStatusLine`.

Establecer `agent` activa uno de los [agentes personalizados](/docs/es/sub-agents) del plugin como el hilo principal, aplicando su indicación del sistema, restricciones de herramientas y modelo. Esto permite que un plugin cambie cómo se comporta Claude Code por defecto cuando está habilitado.

```json settings.json theme={null}
{
  "agent": "security-reviewer"
}
```

Este ejemplo activa el agente `security-reviewer` definido en el directorio `agents/` del plugin. Las configuraciones de `settings.json` tienen prioridad sobre `settings` declarados en `plugin.json`. Las claves desconocidas se ignoran silenciosamente.

<h3 id="organize-complex-plugins">
  Organice plugins complejos
</h3>

Para plugins con muchos componentes, organice su estructura de directorios por funcionalidad. Para diseños de directorios completos y patrones de organización, consulte [Estructura de directorios del plugin](/docs/es/plugins-reference#plugin-directory-structure).

<h3 id="test-your-plugins-locally">
  Pruebe sus plugins localmente
</h3>

Use la bandera `--plugin-dir` para probar plugins durante el desarrollo. Esto carga su plugin directamente sin requerir instalación.

```bash theme={null}
claude --plugin-dir ./my-plugin
```

La bandera también acepta un archivo `.zip` del directorio del plugin, que requiere Claude Code v2.1.128 o posterior.

```bash theme={null}
claude --plugin-dir ./my-plugin.zip
```

Cuando un plugin `--plugin-dir` tiene el mismo nombre que un plugin de marketplace instalado, la copia local tiene prioridad para esa sesión. Esto le permite probar cambios en un plugin que ya tiene instalado sin desinstalarlo primero. La excepción son los plugins cuyas configuraciones administradas fuerzan la habilitación o deshabilitación: `--plugin-dir` no puede anular esos.

A medida que haga cambios en su plugin, ejecute `/reload-plugins` para recoger las actualizaciones sin reiniciar. Esto recarga plugins, skills, agentes, hooks, servidores MCP de plugin y servidores LSP de plugin. Pruebe los componentes de su plugin:

* Pruebe sus skills con `/plugin-name:skill-name`
* Verifique que los agentes aparezcan en `/context` bajo Agentes Personalizados, o mencione uno con @-mention por su nombre con alcance
* Verifique que los hooks funcionen como se espera

<Tip>
  Puede cargar múltiples plugins a la vez especificando la bandera varias veces:

  ```bash theme={null}
  claude --plugin-dir ./plugin-one --plugin-dir ./plugin-two
  ```
</Tip>

Para probar un plugin que ya está empaquetado como un archivo `.zip` y alojado en una URL, como un artefacto de compilación de CI, use `--plugin-url` en su lugar. Claude Code obtiene el archivo al inicio y lo carga solo para esa sesión. Si la obtención falla o el archivo no es válido, Claude Code reporta un error de carga de plugin e inicia sin él. Las mismas [consideraciones de confianza](/docs/es/discover-plugins#security) se aplican como para cualquier fuente de plugin: solo apunte esta bandera a archivos que controle o en los que confíe.

Para cargar múltiples plugins, repita la bandera para cada URL:

```bash theme={null}
claude --plugin-url https://example.com/my-plugin.zip --plugin-url https://example.com/other.zip
```

O pase URLs separadas por espacios como un argumento entrecomillado:

```bash theme={null}
claude --plugin-url "https://example.com/my-plugin.zip https://example.com/other.zip"
```

<h3 id="debug-plugin-issues">
  Depure problemas del plugin
</h3>

Si su plugin no funciona como se espera:

1. **Verifique la estructura**: Asegúrese de que sus directorios estén en la raíz del plugin, no dentro de `.claude-plugin/`
2. **Pruebe componentes individualmente**: Verifique cada skill, agente y hook por separado
3. **Use herramientas de validación y depuración**: Consulte [Herramientas de depuración y desarrollo](/docs/es/plugins-reference#debugging-and-development-tools) para comandos CLI y técnicas de solución de problemas

<h3 id="share-your-plugins">
  Comparta sus plugins
</h3>

Cuando su plugin esté listo para compartir:

1. **Agregue documentación**: Incluya un `README.md` con instrucciones de instalación y uso
2. **Elija una estrategia de versionado**: Decida si establecer una `version` explícita o confiar en el SHA del commit de git. Consulte [gestión de versiones](/docs/es/plugins-reference#version-management)
3. **Cree o use un marketplace**: Distribuya a través de [marketplaces de plugins](/docs/es/plugin-marketplaces) para instalación
4. **Pruebe con otros**: Haga que los miembros del equipo prueben el plugin antes de una distribución más amplia

Una vez que su plugin esté en un marketplace, otros pueden instalarlo usando las instrucciones en [Descubrir e instalar plugins](/docs/es/discover-plugins). Para mantener un plugin interno en su equipo, aloje el marketplace en un [repositorio privado](/docs/es/plugin-marketplaces#private-repositories).

<h3 id="submit-your-plugin-to-the-community-marketplace">
  Envíe su plugin al marketplace de la comunidad
</h3>

Anthropic mantiene dos marketplaces públicos para plugins de Claude Code:

* **`claude-plugins-official`**: un conjunto curado de plugins mantenidos por Anthropic. Registrado automáticamente la primera vez que inicia Claude Code de forma interactiva. Un script no interactivo que se ejecuta antes de ese primer lanzamiento debe agregarlo explícitamente con `claude plugin marketplace add anthropics/claude-plugins-official`.
* **`claude-community`**: el marketplace público de la comunidad donde los envíos de terceros llegan después de la revisión. Los usuarios lo agregan con `/plugin marketplace add anthropics/claude-plugins-community` e instalan desde él como `@claude-community`.

Para enviar su plugin para revisión del marketplace de la comunidad, use uno de los formularios en la aplicación:

* **claude.ai**: [claude.ai/admin-settings/directory/submissions/plugins/new](https://claude.ai/admin-settings/directory/submissions/plugins/new)
* **Console**: [platform.claude.com/plugins/submit](https://platform.claude.com/plugins/submit)

El formulario de claude.ai requiere una organización de Team o Enterprise y acceso a la gestión de directorios; los Propietarios de la organización tienen este acceso por defecto. Los autores individuales que no forman parte de una organización de Team o Enterprise pueden usar el formulario de Console en su lugar.

Ejecute `claude plugin validate` localmente antes de enviar. La canalización de revisión ejecuta la misma verificación en cada envío, junto con el análisis de seguridad automatizado.

Los plugins aprobados se fijan a un SHA de commit específico en el catálogo [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community), y CI actualiza el pin automáticamente a medida que envía nuevos commits a su repositorio. El catálogo público se sincroniza cada noche desde la canalización de revisión, por lo que puede haber un retraso entre la aprobación y que su plugin aparezca en `marketplace.json`. Para verificar si su plugin ya es instalable, busque su nombre en el [catálogo de la comunidad](https://github.com/anthropics/claude-plugins-community/blob/main/.claude-plugin/marketplace.json).

El marketplace oficial, `claude-plugins-official`, se cura por separado. Anthropic decide qué plugins incluir a su discreción. No hay un proceso de solicitud, y el formulario de envío no agrega plugins al marketplace oficial.

Si Anthropic lista su plugin en el marketplace oficial, su CLI puede solicitar a los usuarios de Claude Code que lo instalen. Consulte [Recomienda su plugin desde su CLI](/docs/es/plugin-hints).

<Note>
  Para especificaciones técnicas completas, técnicas de depuración y estrategias de distribución, consulte [Referencia de plugins](/docs/es/plugins-reference).
</Note>

<h2 id="convert-existing-configurations-to-plugins">
  Convierta configuraciones existentes en plugins
</h2>

Si ya tiene skills o hooks en su directorio `.claude/`, puede convertirlos en un plugin para compartir y distribución más fácil.

<h3 id="migration-steps">
  Pasos de migración
</h3>

<Steps>
  <Step title="Cree la estructura del plugin">
    Cree un nuevo directorio de plugin en la raíz de su proyecto, junto a la carpeta `.claude/` existente, para que las rutas relativas de `cp` en el siguiente paso se resuelvan:

    ```bash theme={null}
    mkdir -p my-plugin/.claude-plugin
    ```

    Cree el archivo de manifiesto en `my-plugin/.claude-plugin/plugin.json`:

    ```json my-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-plugin",
      "description": "Migrated from standalone configuration",
      "version": "1.0.0"
    }
    ```
  </Step>

  <Step title="Copie sus archivos existentes">
    Copie sus configuraciones existentes al directorio del plugin:

    ```bash theme={null}
    # Copy commands
    cp -r .claude/commands my-plugin/

    # Copy agents (if any)
    cp -r .claude/agents my-plugin/

    # Copy skills (if any)
    cp -r .claude/skills my-plugin/
    ```
  </Step>

  <Step title="Migre hooks">
    Si tiene hooks en su configuración, cree un directorio de hooks:

    ```bash theme={null}
    mkdir my-plugin/hooks
    ```

    Cree `my-plugin/hooks/hooks.json` con su configuración de hooks. Copie el objeto `hooks` de su `.claude/settings.json` o `settings.local.json`, ya que el formato es el mismo. El comando recibe entrada de hook como JSON en stdin, así que use `jq` para extraer la ruta del archivo:

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

  <Step title="Pruebe su plugin migrado">
    Cargue su plugin para verificar que todo funciona:

    ```bash theme={null}
    claude --plugin-dir ./my-plugin
    ```

    Pruebe cada componente: ejecute sus comandos, verifique que los agentes aparezcan en `/context`, y verifique que los hooks se activen correctamente.
  </Step>
</Steps>

<h3 id="what-changes-when-migrating">
  Qué cambia al migrar
</h3>

| Independiente (`.claude/`)             | Plugin                                      |
| :------------------------------------- | :------------------------------------------ |
| Solo disponible en un proyecto         | Se puede compartir a través de marketplaces |
| Archivos en `.claude/commands/`        | Archivos en `plugin-name/commands/`         |
| Hooks en `settings.json`               | Hooks en `hooks/hooks.json`                 |
| Debe copiar manualmente para compartir | Instalar con `/plugin install`              |

<Note>
  Después de migrar, elimine los archivos originales de `.claude/` para evitar duplicados. Las definiciones de `.claude/agents/` a nivel de proyecto y usuario anulan los agentes del plugin con el mismo nombre, por lo que la versión del plugin solo tiene efecto una vez que se eliminan los originales. Las skills del plugin se espacian de nombres como `/plugin-name:skill-name`, por lo que tanto el `/skill-name` original como la copia del plugin permanecen disponibles en lugar de que uno anule al otro.
</Note>

<h2 id="next-steps">
  Próximos pasos
</h2>

Ahora que entiende el sistema de plugins de Claude Code, aquí hay caminos sugeridos para diferentes objetivos:

<h3 id="for-plugin-users">
  Para usuarios de plugins
</h3>

* [Descubrir e instalar plugins](/docs/es/discover-plugins): examine marketplaces e instale plugins
* [Configure marketplaces de equipo](/docs/es/discover-plugins#configure-team-marketplaces): configure plugins a nivel de repositorio para su equipo

<h3 id="for-plugin-developers">
  Para desarrolladores de plugins
</h3>

* [Crear y distribuir un marketplace](/docs/es/plugin-marketplaces): empaquete y comparta sus plugins
* [Referencia de plugins](/docs/es/plugins-reference): especificaciones técnicas completas
* Profundice en componentes específicos del plugin:
  * [Skills](/docs/es/skills): detalles de desarrollo de skills
  * [Subagents](/docs/es/sub-agents): configuración y capacidades del agente
  * [Hooks](/docs/es/hooks): manejo de eventos y automatización
  * [MCP](/docs/es/mcp): integración de herramientas externas
