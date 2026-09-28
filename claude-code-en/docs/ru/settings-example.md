> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Примеры файлов settings

> Реалистичные файлы settings.json для разработчика, команды и организации: скопируйте один, оставьте нужные вам ключи и измените значения.

На этой странице находятся три примера файлов `settings.json`, по одному для каждого места, где вы сохраняете параметр:

* `~/.claude/settings.json` разработчика
* `.claude/settings.json` команды, зафиксированный в репозитории
* `managed-settings.json` организации

Каждый файл является правдоподобным для этого читателя, поэтому вы можете увидеть структуру и скопировать нужные вам части. Ни один из них не является рекомендуемой базовой конфигурацией. Каждое значение взято из записи ключа в [справочнике параметров](/docs/ru/settings-reference), который содержит его тип, значение по умолчанию и место, где его можно установить.

Каждый пример имеет две вкладки. **Copyable settings file** — это файл в том виде, в котором вы его сохраняете. **What each key does** — это тот же файл с комментарием над каждым ключом; Claude Code не принимает комментарии в файле параметров, поэтому копируйте с первой вкладки.

<h2 id="your-own-settings">
  Ваши собственные параметры
</h2>

Личные параметры одного разработчика. Он выбирает модель и уровень усилий, настраивает терминал и предварительно одобряет команду только для чтения и одно чтение файла. Все остальное сохраняет значения по умолчанию. Файл, подобный этому, находится в `~/.claude/settings.json`, где он применяется к каждому проекту, который вы открываете.

<Tabs>
  <Tab title="Copyable settings file">
    Сохраните это как `~/.claude/settings.json`. Это корректный JSON без комментариев, поэтому вы можете вставить его как есть и удалить ненужные ключи.

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

  <Tab title="What each key does">
    Тот же файл с комментарием над каждым ключом. Прочитайте его здесь; копируйте с другой вкладки, потому что Claude Code не принимает комментарии в файле параметров.

    ```jsonc ~/.claude/settings.json theme={null}
    {
      // Начинайте каждый сеанс на Sonnet 5
      "model": "claude-sonnet-5",
      // Запускайте Sonnet 5 выше его уровня по умолчанию; /effort сохраняет уровень для каждой модели, а --effort устанавливает уровень для одного сеанса
      "modelSettings": {
        "claude-sonnet-5": { "effortLevel": "xhigh" }
      },
      // Сочетания клавиш Vim в приглашении
      "editorMode": "vim",
      // Светлая тема, удобная для дальтоников
      "theme": "light-daltonized",
      // Строка состояния под приглашением: имя модели и использованный контекст
      "statusLine": {
        "type": "command",
        "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
        "padding": 2
      },
      // Скройте советы, которые вращаются под спиннером
      "spinnerTipsEnabled": false,
      // Звоните в колокольчик терминала для уведомлений, таких как завершенная задача или ожидающее приглашение разрешения
      "preferredNotifChannel": "terminal_bell",
      // Позвольте Claude Code запускать git diff и читать ваш .zshrc без запроса
      "permissions": {
        "allow": [
          "Bash(git diff *)",
          "Read(~/.zshrc)"
        ]
      },
      // Получайте обновления из стабильного канала
      "autoUpdatesChannel": "stable",
      // Удалите стенограммы сеансов и другие локальные данные сеансов старше 20 дней
      "cleanupPeriodDays": 20
    }
    ```
  </Tab>
</Tabs>

<h2 id="a-teams-shared-settings">
  Общие параметры команды
</h2>

Общие параметры одной команды, зафиксированные в репозитории, чтобы каждый, кто его клонирует, получал одинаковые разрешения, hooks и marketplace плагинов. Сохраните файл, подобный этому, в `.claude/settings.json` в верхней части репозитория. Что нужно знать перед его фиксацией:

* **Облачные сеансы также его читают.** [Облачный сеанс](/docs/ru/settings#settings-in-cloud-sessions) начинается с клона репозитория, поэтому зафиксированный файл применяется и там.
* **Телеметрия входит в управляемые или личные параметры.** Claude Code игнорирует [переменные экспортера OpenTelemetry](/docs/ru/settings-reference#variables-claude-code-ignores-in-env) в файлах параметров репозитория, за исключением некоторых значений, которые отключают телеметрию. Установите их в [управляемых параметрах](/docs/ru/monitoring-usage#administrator-configuration) для вашей организации или в файле `~/.claude/settings.json` каждого человека.
* **Правила Allow ждут доверия.** Правила Allow и записи `extraKnownMarketplaces` вступают в силу после того, как каждый человек [доверит этой папке](/docs/ru/permissions#project-allow-rules-and-workspace-trust), а не только родительской папке; правила deny и ask применяются в каждом сеансе, доверенном или нет.
* **Hook — это скрипт в репозитории.** Hook этого файла запускает `.claude/hooks/block-rm.sh`; [How a hook resolves](/docs/ru/hooks#how-a-hook-resolves) описывает, как его написать.
* **Правила соответствуют команде и пути в том виде, в котором они написаны.** `Bash(git push *)` не соответствует [`git -C . push`](/docs/ru/permissions#bash-rule-limits). `Read(./.env)` само по себе останавливает инструменты файлов и команды, которые называют файл, такие как `cat .env`, но не [`grep -r`, запущенный по каталогу](/docs/ru/permissions#read-and-edit); блок `sandbox` в этом файле закрывает этот пробел, потому что sandbox [добавляет ваши пути `Read` deny](/docs/ru/settings-reference#sandbox-filesystem-denyread) к тому, что не может читать каждая команда в sandbox.

<Tabs>
  <Tab title="Copyable settings file">
    Сохраните это как `.claude/settings.json` в верхней части репозитория и зафиксируйте. Это корректный JSON без комментариев, поэтому вы можете вставить его как есть и удалить ненужные ключи.

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

  <Tab title="What each key does">
    Тот же файл с комментарием над каждым ключом. Прочитайте его здесь; копируйте с другой вкладки, потому что Claude Code не принимает комментарии в файле параметров.

    ```jsonc .claude/settings.json theme={null}
    {
      "permissions": {
        // Запускайте скрипты npm без запроса
        "allow": [
          "Bash(npm run *)"
        ],
        // Подтвердите перед командами git push
        "ask": [
          "Bash(git push *)"
        ],
        // Запретите чтение файлов env и папки secrets инструментами файлов и командами чтения файлов
        "deny": [
          "Read(./.env)",
          "Read(./.env.*)",
          "Read(./secrets/**)"
        ]
      },
      // Перед каждой командой Bash запустите скрипт в репозитории, который может ее заблокировать
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
      // Зарегистрируйте marketplace плагинов команды при каждом клонировании
      "extraKnownMarketplaces": {
        "acme-tools": {
          "source": {
            "source": "github",
            "repo": "acme-corp/claude-plugins"
          }
        }
      },
      // Включите один плагин из этого marketplace; плагин из внешнего источника, такого как репозиторий GitHub, все еще требует, чтобы каждый человек установил его один раз
      "enabledPlugins": {
        "code-formatter@acme-tools": true
      },
      // Команды Sandbox: записываемая папка build; npm и example.com предварительно разрешены, другие хосты все еще запрашивают
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
      // Держите файлы плана внутри репозитория
      "plansDirectory": "./plans"
    }
    ```
  </Tab>
</Tabs>

<h2 id="an-organizations-managed-settings">
  Управляемые параметры организации
</h2>

Файл `managed-settings.json`, который показывает форму управляемых ключей с одним правдоподобным значением для каждого. Это не рекомендуемая политика: выберите ключи, которые соответствуют вашим требованиям, и установите свои собственные значения. Пример устанавливает эти ключи:

* `forceLoginMethod` и `forceLoginOrgUUID` закрепляют метод входа и организацию
* `availableModels` и `enforceAvailableModels` ограничивают, какие модели могут использовать сеансы
* `permissions.deny` запрещает два чтения файлов и команды `curl` [как их пишет Claude](/docs/ru/permissions#bash-rule-limits), а `disableBypassPermissionsMode` удаляет режим обхода разрешений
* [`allowManagedPermissionRulesOnly`](/docs/ru/settings-reference#allowmanagedpermissionrulesonly) и [`allowManagedMcpServersOnly`](/docs/ru/settings-reference#allowmanagedmcpserversonly) делают управляемые списки разрешений для разрешений и MCP единственными применяемыми
* `allowedMcpServers` закрепляет MCP сервер по URL
* `strictKnownMarketplaces` разрешает один marketplace плагинов
* `sandbox` помещает команды в sandbox с фиксированным списком разрешений сети и без повторной попытки без sandbox
* `requiredMinimumVersion` устанавливает минимальную версию Claude Code
* `cleanupPeriodDays` сокращает хранение стенограмм сеансов и других локальных данных до семи дней
* `companyAnnouncements` показывает сообщение при запуске

Администраторы развертывают файл, подобный этому, как `managed-settings.json`, или тот же JSON через MDM или [параметры, управляемые сервером](/docs/ru/server-managed-settings). Один развернутый файл применяется к каждой машине или учетной записи, которую он достигает. Чтобы дать группе разные значения, разверните другой файл или профиль для этой группы, так как [параметры, управляемые сервером, еще не поддерживают политику для каждой группы](/docs/ru/server-managed-settings#current-limitations).

<Tabs>
  <Tab title="Copyable settings file">
    Разверните это как `managed-settings.json`, или тот же JSON через MDM или консоль claude.ai. Это корректный JSON без комментариев; замените пример UUID организации, URL сервера и marketplace на свои и удалите ненужные ключи.

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

  <Tab title="What each key does">
    Тот же файл с комментарием над каждым ключом. Прочитайте его здесь; копируйте с другой вкладки, потому что Claude Code не принимает комментарии в файле параметров.

    ```jsonc managed-settings.json theme={null}
    {
      // Только входы claude.ai и только в этой организации
      "forceLoginMethod": "claudeai",
      "forceLoginOrgUUID": [
        "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
      ],
      // Только модели Opus и Sonnet; с enforceAvailableModels опция Default также соблюдает список
      "availableModels": [
        "opus",
        "sonnet"
      ],
      "enforceAvailableModels": true,
      "permissions": {
        // Запретите команды curl и чтение файла .env проекта и папки secrets на каждой машине
        "deny": [
          "Bash(curl *)",
          "Read(./.env)",
          "Read(./secrets/**)"
        ],
        // Удалите режим обхода разрешений из каждого сеанса
        "disableBypassPermissionsMode": "disable"
      },
      // Игнорируйте правила разрешений из параметров пользователя, проекта и локальных параметров
      "allowManagedPermissionRulesOnly": true,
      // Только MCP сервер GitHub, сопоставленный по URL, а не по имени, так как пользователь может
      // назвать любой сервер "github". Добавленные пользователем серверы, которые не совпадают, не загружаются, включая
      // каждый stdio сервер, когда список содержит только записи URL. Ключ allowManagedMcpServersOnly
      // ниже делает этот управляемый список единственным применяемым списком разрешений
      "allowedMcpServers": [
        {
          "serverUrl": "https://api.githubcopilot.com/*"
        }
      ],
      "allowManagedMcpServersOnly": true,
      // Плагины могут поступать только из этого marketplace
      "strictKnownMarketplaces": [
        {
          "source": "github",
          "repo": "acme-corp/approved-plugins"
        }
      ],
      // Поместите каждую команду Claude в sandbox, откажитесь запускаться, если sandbox не может быть
      // установлен, и никогда не позволяйте заблокированной команде повторить попытку вне sandbox; сеть
      // ограничена npm и GitHub, и пользователи не могут добавлять домены
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
      // Откажитесь запускаться на версиях старше 2.1.150
      "requiredMinimumVersion": "2.1.150",
      // Удалите стенограммы сеансов и другие локальные данные сеансов через 7 дней
      "cleanupPeriodDays": 7,
      // Сообщение, которое видит каждый пользователь при запуске
      "companyAnnouncements": [
        "Welcome to Acme Corp! Review our code guidelines at docs.example.com"
      ]
    }
    ```
  </Tab>
</Tabs>
