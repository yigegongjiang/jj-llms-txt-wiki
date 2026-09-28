> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Archivos de configuración de ejemplo

> Archivos settings.json realistas para un desarrollador, un equipo y una organización: copie uno, mantenga las claves que desee y cambie los valores.

Esta página contiene tres archivos `settings.json` de ejemplo, uno para cada lugar donde guarda una configuración:

* Un `~/.claude/settings.json` de desarrollador
* Un `.claude/settings.json` de equipo, confirmado en el repositorio
* Un `managed-settings.json` de organización

Cada uno es un archivo plausible para ese lector, para que pueda ver la forma y copiar las partes que desee. Ninguno de ellos es una línea base recomendada. Cada valor proviene de la entrada de la clave en la [referencia de configuración](/docs/es/settings-reference), que tiene su tipo, valor predeterminado y dónde se puede establecer.

Cada ejemplo tiene dos pestañas. **Archivo de configuración copiable** es el archivo tal como lo guardaría. **Lo que hace cada clave** es el mismo archivo con un comentario encima de cada clave; Claude Code no acepta comentarios en un archivo de configuración, así que copie de la primera pestaña.

<h2 id="your-own-settings">
  Su propia configuración
</h2>

La configuración personal de un desarrollador. Elige un modelo y esfuerzo, ajusta la terminal y aprueba previamente un comando de solo lectura y una lectura de archivo. Todo lo que no aparece en la lista mantiene su valor predeterminado. Un archivo como este va en `~/.claude/settings.json`, donde se aplica a cada proyecto que abre.

<Tabs>
  <Tab title="Archivo de configuración copiable">
    Guarde esto como `~/.claude/settings.json`. Es JSON válido sin comentarios, así que puede pegarlo tal cual y eliminar las claves que no desee.

    ```json ~/.claude/settings.json theme={null}
    {
      "model": "claude-sonnet-5",
      "modelSettings": {
        "claude-sonnet-5": { "effortLevel": "xhigh" }
      },
      "editorMode": "vim",
      "theme": "light-daltonized",
      "statusLine": {
        "type": "command",
        "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
        "padding": 2
      },
      "spinnerTipsEnabled": false,
      "preferredNotifChannel": "terminal_bell",
      "permissions": {
        "allow": [
          "Bash(git diff *)",
          "Read(~/.zshrc)"
        ]
      },
      "autoUpdatesChannel": "stable",
      "cleanupPeriodDays": 20
    }
    ```
  </Tab>

  <Tab title="Lo que hace cada clave">
    El mismo archivo con un comentario encima de cada clave. Léalo aquí; copie de la otra pestaña, porque Claude Code no acepta comentarios en un archivo de configuración.

    ```jsonc ~/.claude/settings.json theme={null}
    {
      // Inicie cada sesión en Sonnet 5
      "model": "claude-sonnet-5",
      // Ejecute Sonnet 5 por encima de su nivel alto predeterminado; /effort guarda un nivel por modelo, y --effort establece uno para una sola sesión
      "modelSettings": {
        "claude-sonnet-5": { "effortLevel": "xhigh" }
      },
      // Atajos de teclado Vim en el símbolo del sistema
      "editorMode": "vim",
      // El tema claro amigable para daltónicos
      "theme": "light-daltonized",
      // Una línea de estado debajo del símbolo del sistema: nombre del modelo y contexto utilizado
      "statusLine": {
        "type": "command",
        "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
        "padding": 2
      },
      // Oculte los consejos que rotan bajo el spinner
      "spinnerTipsEnabled": false,
      // Suene la campana de la terminal para notificaciones, como una tarea finalizada o un aviso de permiso en espera
      "preferredNotifChannel": "terminal_bell",
      // Permita que Claude Code ejecute git diff y lea su .zshrc sin preguntar
      "permissions": {
        "allow": [
          "Bash(git diff *)",
          "Read(~/.zshrc)"
        ]
      },
      // Tome actualizaciones del canal estable
      "autoUpdatesChannel": "stable",
      // Elimine transcripciones de sesión y otros datos de sesión local más antiguos que 20 días
      "cleanupPeriodDays": 20
    }
    ```
  </Tab>
</Tabs>

<h2 id="a-teams-shared-settings">
  Configuración compartida de un equipo
</h2>

La configuración compartida de un equipo, confirmada en el repositorio para que todos los que lo clonan obtengan los mismos permisos, hooks y marketplace de plugins. Guarde un archivo como este en `.claude/settings.json` en la parte superior del repositorio. Lo que debe saber antes de confirmar uno:

* **Las sesiones en la nube también lo leen.** Una [sesión en la nube](/docs/es/settings#settings-in-cloud-sessions) comienza desde un clon del repositorio, por lo que el archivo confirmado también se aplica allí.
* **La telemetría va en configuración administrada o personal.** Claude Code ignora las [variables del exportador de OpenTelemetry](/docs/es/settings-reference#variables-claude-code-ignores-in-env) en los archivos de configuración de un repositorio, aparte de algunos valores que desactivan la telemetría. Configúrelas en [configuración administrada](/docs/es/monitoring-usage#administrator-configuration) para su organización, o en el archivo `~/.claude/settings.json` de cada persona.
* **Las reglas de permiso esperan confianza.** Las reglas de permiso y las entradas de `extraKnownMarketplaces` entran en vigor después de que cada persona [confía en esta carpeta misma](/docs/es/permissions#project-allow-rules-and-workspace-trust), no solo en una carpeta principal; las reglas de denegación y pregunta se aplican en cada sesión, confiable o no.
* **El hook es un script en el repositorio.** El hook de este archivo ejecuta `.claude/hooks/block-rm.sh`; [Cómo se resuelve un hook](/docs/es/hooks#how-a-hook-resolves) explica cómo escribirlo.
* **Las reglas coinciden con el comando y la ruta tal como se escriben.** `Bash(git push *)` no coincide con [`git -C . push`](/docs/es/permissions#bash-rule-limits). `Read(./.env)` por sí solo detiene las herramientas de archivo y los comandos que nombran el archivo, como `cat .env`, pero no [`grep -r` ejecutado sobre el directorio](/docs/es/permissions#read-and-edit); el bloque `sandbox` en este archivo cierra esa brecha, porque el sandbox [añade sus rutas de denegación de `Read`](/docs/es/settings-reference#sandbox-filesystem-denyread) a lo que ningún comando en sandbox puede leer.

<Tabs>
  <Tab title="Archivo de configuración copiable">
    Guarde esto como `.claude/settings.json` en la parte superior del repositorio y confírmelo. Es JSON válido sin comentarios, así que puede pegarlo tal cual y eliminar las claves que no desee.

    ```json .claude/settings.json theme={null}
    {
      "permissions": {
        "allow": [
          "Bash(npm run *)"
        ],
        "ask": [
          "Bash(git push *)"
        ],
        "deny": [
          "Read(./.env)",
          "Read(./.env.*)",
          "Read(./secrets/**)"
        ]
      },
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.sh"
              }
            ]
          }
        ]
      },
      "extraKnownMarketplaces": {
        "acme-tools": {
          "source": {
            "source": "github",
            "repo": "acme-corp/claude-plugins"
          }
        }
      },
      "enabledPlugins": {
        "code-formatter@acme-tools": true
      },
      "sandbox": {
        "enabled": true,
        "filesystem": {
          "allowWrite": [
            "/tmp/build"
          ]
        },
        "network": {
          "allowedDomains": [
            "registry.npmjs.org",
            "*.example.com"
          ]
        }
      },
      "plansDirectory": "./plans"
    }
    ```
  </Tab>

  <Tab title="Lo que hace cada clave">
    El mismo archivo con un comentario encima de cada clave. Léalo aquí; copie de la otra pestaña, porque Claude Code no acepta comentarios en un archivo de configuración.

    ```jsonc .claude/settings.json theme={null}
    {
      "permissions": {
        // Ejecute scripts npm sin preguntar
        "allow": [
          "Bash(npm run *)"
        ],
        // Confirme antes de comandos git push
        "ask": [
          "Bash(git push *)"
        ],
        // Deniegue lecturas de archivos env y la carpeta de secretos por las herramientas de archivo y comandos que leen archivos
        "deny": [
          "Read(./.env)",
          "Read(./.env.*)",
          "Read(./secrets/**)"
        ]
      },
      // Antes de cada comando Bash, ejecute un script en el repositorio que pueda bloquearlo
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.sh"
              }
            ]
          }
        ]
      },
      // Registre el marketplace de plugins del equipo en cada clon
      "extraKnownMarketplaces": {
        "acme-tools": {
          "source": {
            "source": "github",
            "repo": "acme-corp/claude-plugins"
          }
        }
      },
      // Habilite un plugin de ese marketplace; un plugin de una fuente externa como un repositorio de GitHub aún necesita que cada persona lo instale una vez
      "enabledPlugins": {
        "code-formatter@acme-tools": true
      },
      // Comandos de sandbox: directorio de compilación escribible; npm y example.com preaprobados, otros hosts aún solicitan
      "sandbox": {
        "enabled": true,
        "filesystem": {
          "allowWrite": [
            "/tmp/build"
          ]
        },
        "network": {
          "allowedDomains": [
            "registry.npmjs.org",
            "*.example.com"
          ]
        }
      },
      // Mantenga los archivos de plan dentro del repositorio
      "plansDirectory": "./plans"
    }
    ```
  </Tab>
</Tabs>

<h2 id="an-organizations-managed-settings">
  Configuración administrada de una organización
</h2>

Un archivo `managed-settings.json` que muestra la forma de las claves administradas, con un valor plausible para cada una. No es una política recomendada: elija las claves que coincidan con sus propios requisitos y establezca sus propios valores. El ejemplo establece estas claves:

* `forceLoginMethod` y `forceLoginOrgUUID` fijan el método de inicio de sesión y la organización
* `availableModels` y `enforceAvailableModels` restringen qué modelos pueden usar las sesiones
* `permissions.deny` bloquea dos lecturas de archivo y comandos `curl` [como Claude los escribe](/docs/es/permissions#bash-rule-limits), y `disableBypassPermissionsMode` elimina el modo de permiso de omisión
* [`allowManagedPermissionRulesOnly`](/docs/es/settings-reference#allowmanagedpermissionrulesonly) y [`allowManagedMcpServersOnly`](/docs/es/settings-reference#allowmanagedmcpserversonly) hacen que las listas de permisos administrados y MCP sean las únicas que se apliquen
* `allowedMcpServers` fija el servidor MCP por URL
* `strictKnownMarketplaces` permite un marketplace de plugins
* `sandbox` coloca los comandos en sandbox con una lista de permisos de red fija y sin reintento sin sandbox
* `requiredMinimumVersion` establece una versión mínima de Claude Code
* `cleanupPeriodDays` acorta la retención de transcripciones de sesión y otros datos locales a siete días
* `companyAnnouncements` muestra un mensaje al inicio

Los administradores implementan un archivo como este como `managed-settings.json`, o el mismo JSON a través de MDM o [configuración administrada por servidor](/docs/es/server-managed-settings). Un archivo implementado se aplica a cada máquina o cuenta que alcanza. Para dar a un grupo valores diferentes, implemente un archivo o perfil diferente en ese grupo, ya que [la configuración administrada por servidor aún no admite política por grupo](/docs/es/server-managed-settings#current-limitations).

<Tabs>
  <Tab title="Archivo de configuración copiable">
    Implemente esto como `managed-settings.json`, o el mismo JSON a través de MDM o la consola de claude.ai. Es JSON válido sin comentarios; reemplace el UUID de organización de ejemplo, la URL del servidor y el marketplace con los suyos propios y elimine las claves que no desee.

    ```json managed-settings.json theme={null}
    {
      "forceLoginMethod": "claudeai",
      "forceLoginOrgUUID": [
        "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
      ],
      "availableModels": [
        "opus",
        "sonnet"
      ],
      "enforceAvailableModels": true,
      "permissions": {
        "deny": [
          "Bash(curl *)",
          "Read(./.env)",
          "Read(./secrets/**)"
        ],
        "disableBypassPermissionsMode": "disable"
      },
      "allowManagedPermissionRulesOnly": true,
      "allowedMcpServers": [
        {
          "serverUrl": "https://api.githubcopilot.com/*"
        }
      ],
      "allowManagedMcpServersOnly": true,
      "strictKnownMarketplaces": [
        {
          "source": "github",
          "repo": "acme-corp/approved-plugins"
        }
      ],
      "sandbox": {
        "enabled": true,
        "failIfUnavailable": true,
        "allowUnsandboxedCommands": false,
        "network": {
          "allowedDomains": [
            "registry.npmjs.org",
            "github.com"
          ],
          "allowManagedDomainsOnly": true
        }
      },
      "requiredMinimumVersion": "2.1.150",
      "cleanupPeriodDays": 7,
      "companyAnnouncements": [
        "Welcome to Acme Corp! Review our code guidelines at docs.example.com"
      ]
    }
    ```
  </Tab>

  <Tab title="Lo que hace cada clave">
    El mismo archivo con un comentario encima de cada clave. Léalo aquí; copie de la otra pestaña, porque Claude Code no acepta comentarios en un archivo de configuración.

    ```jsonc managed-settings.json theme={null}
    {
      // Solo inicios de sesión de claude.ai, y solo en esta organización
      "forceLoginMethod": "claudeai",
      "forceLoginOrgUUID": [
        "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
      ],
      // Solo modelos Opus y Sonnet; con enforceAvailableModels, la opción Predeterminado también obedece la lista
      "availableModels": [
        "opus",
        "sonnet"
      ],
      "enforceAvailableModels": true,
      "permissions": {
        // Bloquee curl, el archivo .env del proyecto y su carpeta de secretos en cada máquina
        "deny": [
          "Bash(curl *)",
          "Read(./.env)",
          "Read(./secrets/**)"
        ],
        // Elimine el modo de omisión de permisos de cada sesión
        "disableBypassPermissionsMode": "disable"
      },
      // Ignore las reglas de permiso de la configuración de usuario, proyecto y local
      "allowManagedPermissionRulesOnly": true,
      // Solo el servidor MCP de GitHub, coincidido por URL en lugar de por nombre, ya que un usuario puede
      // nombrar cualquier servidor "github". Los servidores que no coinciden no se cargan, lo que incluye cada
      // servidor stdio cuando la lista tiene solo entradas de URL; el bloqueo a continuación hace que esta lista administrada
      // sea la única lista de permisos que cuenta
      "allowedMcpServers": [
        {
          "serverUrl": "https://api.githubcopilot.com/*"
        }
      ],
      "allowManagedMcpServersOnly": true,
      // Los plugins pueden provenir solo de este marketplace
      "strictKnownMarketplaces": [
        {
          "source": "github",
          "repo": "acme-corp/approved-plugins"
        }
      ],
      // Coloque cada comando que ejecuta Claude en sandbox, rechace iniciar si el sandbox no puede
      // configurarse, y nunca permita que un comando bloqueado se reintente fuera del sandbox; red
      // limitada a npm y GitHub, y los usuarios no pueden agregar dominios
      "sandbox": {
        "enabled": true,
        "failIfUnavailable": true,
        "allowUnsandboxedCommands": false,
        "network": {
          "allowedDomains": [
            "registry.npmjs.org",
            "github.com"
          ],
          "allowManagedDomainsOnly": true
        }
      },
      // Rechace iniciar en versiones anteriores a 2.1.150
      "requiredMinimumVersion": "2.1.150",
      // Elimine transcripciones de sesión y otros datos de sesión local después de 7 días
      "cleanupPeriodDays": 7,
      // Un mensaje que cada usuario ve al inicio
      "companyAnnouncements": [
        "Welcome to Acme Corp! Review our code guidelines at docs.example.com"
      ]
    }
    ```
  </Tab>
</Tabs>
