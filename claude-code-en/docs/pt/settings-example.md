> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Arquivos de configuração de exemplo

> Arquivos settings.json realistas para um desenvolvedor, uma equipe e uma organização: copie um, mantenha as chaves que deseja e altere os valores.

Esta página contém três arquivos `settings.json` de exemplo, um para cada lugar onde você salva uma configuração:

* Um `~/.claude/settings.json` de desenvolvedor
* Um `.claude/settings.json` de equipe, confirmado no repositório
* Um `managed-settings.json` de organização

Cada um é um arquivo plausível para esse leitor, para que você possa ver a forma e copiar as partes que deseja. Nenhum deles é uma linha de base recomendada. Cada valor vem da entrada da chave na [referência de configurações](/docs/pt/settings-reference), que tem seu tipo, padrão e onde pode ser definido.

Cada exemplo tem duas abas. **Copyable settings file** é o arquivo como você o salvaria. **What each key does** é o mesmo arquivo com um comentário acima de cada chave; Claude Code não aceita comentários em um arquivo de configurações, então copie da primeira aba.

<h2 id="your-own-settings">
  Suas próprias configurações
</h2>

As configurações pessoais de um desenvolvedor. Ele escolhe um modelo e esforço, ajusta o terminal e pré-aprova um comando somente leitura e uma leitura de arquivo. Tudo não listado mantém seu padrão. Um arquivo como este vai em `~/.claude/settings.json`, onde se aplica a cada projeto que você abre.

<Tabs>
  <Tab title="Copyable settings file">
    Salve isto como `~/.claude/settings.json`. É JSON válido sem comentários, então você pode colar como está e deletar as chaves que não deseja.

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
    O mesmo arquivo com um comentário acima de cada chave. Leia aqui; copie da outra aba, porque Claude Code não aceita comentários em um arquivo de configurações.

    ```jsonc ~/.claude/settings.json theme={null}
    {
      // Inicie cada sessão no Sonnet 5
      "model": "claude-sonnet-5",
      // Execute Sonnet 5 acima de seu nível alto padrão; /effort salva um nível por modelo, e --effort define um para uma única sessão
      "modelSettings": {
        "claude-sonnet-5": { "effortLevel": "xhigh" }
      },
      // Atalhos de teclado Vim no prompt
      "editorMode": "vim",
      // O tema claro amigável para daltônicos
      "theme": "light-daltonized",
      // Uma linha de status abaixo do prompt: nome do modelo e contexto usado
      "statusLine": {
        "type": "command",
        "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
        "padding": 2
      },
      // Oculte as dicas que giram sob o spinner
      "spinnerTipsEnabled": false,
      // Toque o sino do terminal para notificações, como uma tarefa concluída ou um prompt de permissão aguardando
      "preferredNotifChannel": "terminal_bell",
      // Deixe Claude Code executar git diff e ler seu .zshrc sem perguntar
      "permissions": {
        "allow": [
          "Bash(git diff *)",
          "Read(~/.zshrc)"
        ]
      },
      // Pegue atualizações do canal estável
      "autoUpdatesChannel": "stable",
      // Exclua transcrições de sessão e outros dados de sessão local com mais de 20 dias
      "cleanupPeriodDays": 20
    }
    ```
  </Tab>
</Tabs>

<h2 id="a-teams-shared-settings">
  Configurações compartilhadas de uma equipe
</h2>

As configurações compartilhadas de uma equipe, confirmadas no repositório para que todos que o clonem obtenham as mesmas permissões, hooks e marketplace de plugins. Salve um arquivo como este em `.claude/settings.json` no topo do repositório. O que você precisa saber antes de confirmar um:

* **Sessões em nuvem também o leem.** Uma [sessão em nuvem](/docs/pt/settings#settings-in-cloud-sessions) começa a partir de um clone do repositório, então o arquivo confirmado se aplica lá também.
* **Telemetria vai em configurações gerenciadas ou pessoais.** Claude Code ignora as [variáveis do exportador OpenTelemetry](/docs/pt/settings-reference#variables-claude-code-ignores-in-env) nos arquivos de configurações de um repositório, exceto alguns valores que desativam a telemetria. Defina-as em [configurações gerenciadas](/docs/pt/monitoring-usage#administrator-configuration) para sua organização, ou no `~/.claude/settings.json` de cada pessoa.
* **Regras de permissão aguardam confiança.** Regras de permissão e entradas `extraKnownMarketplaces` entram em vigor depois que cada pessoa [confia nesta pasta em si](/docs/pt/permissions#project-allow-rules-and-workspace-trust), não apenas em uma pasta pai; regras de negação e pergunta se aplicam em cada sessão, confiável ou não.
* **O hook é um script no repositório.** O hook deste arquivo executa `.claude/hooks/block-rm.sh`; [How a hook resolves](/docs/pt/hooks#how-a-hook-resolves) percorre como escrevê-lo.
* **Regras correspondem ao comando e caminho conforme escrito.** `Bash(git push *)` não corresponde a [`git -C . push`](/docs/pt/permissions#bash-rule-limits). `Read(./.env)` por si só impede as ferramentas de arquivo e comandos que nomeiam o arquivo, como `cat .env`, mas não [`grep -r` executado sobre o diretório](/docs/pt/permissions#read-and-edit); o bloco `sandbox` neste arquivo fecha essa lacuna, porque o sandbox [adiciona seus caminhos de negação `Read`](/docs/pt/settings-reference#sandbox-filesystem-denyread) ao que todo comando em sandbox não pode ler.

<Tabs>
  <Tab title="Copyable settings file">
    Salve isto como `.claude/settings.json` no topo do repositório e confirme. É JSON válido sem comentários, então você pode colar como está e deletar as chaves que não deseja.

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
    O mesmo arquivo com um comentário acima de cada chave. Leia aqui; copie da outra aba, porque Claude Code não aceita comentários em um arquivo de configurações.

    ```jsonc .claude/settings.json theme={null}
    {
      "permissions": {
        // Execute scripts npm sem perguntar
        "allow": [
          "Bash(npm run *)"
        ],
        // Confirme antes de comandos git push
        "ask": [
          "Bash(git push *)"
        ],
        // Negue leituras de arquivos env e da pasta de segredos pelas ferramentas de arquivo e comandos que leem arquivos
        "deny": [
          "Read(./.env)",
          "Read(./.env.*)",
          "Read(./secrets/**)"
        ]
      },
      // Antes de cada comando Bash, execute um script no repositório que pode bloqueá-lo
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
      // Registre o marketplace de plugins da equipe em cada clone
      "extraKnownMarketplaces": {
        "acme-tools": {
          "source": {
            "source": "github",
            "repo": "acme-corp/claude-plugins"
          }
        }
      },
      // Ative um plugin desse marketplace; um plugin de uma fonte externa como um repositório GitHub ainda precisa que cada pessoa o instale uma vez
      "enabledPlugins": {
        "code-formatter@acme-tools": true
      },
      // Comandos de sandbox: diretório de compilação gravável; npm e example.com pré-permitidos, outros hosts ainda solicitam
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
      // Mantenha arquivos de plano dentro do repositório
      "plansDirectory": "./plans"
    }
    ```
  </Tab>
</Tabs>

<h2 id="an-organizations-managed-settings">
  Configurações gerenciadas de uma organização
</h2>

Um arquivo `managed-settings.json` que mostra a forma das chaves gerenciadas, com um valor plausível para cada uma. Não é uma política recomendada: escolha as chaves que correspondem aos seus próprios requisitos e defina seus próprios valores. O exemplo define estas chaves:

* `forceLoginMethod` e `forceLoginOrgUUID` fixam o método de login e a organização
* `availableModels` e `enforceAvailableModels` restringem quais modelos as sessões podem usar
* `permissions.deny` bloqueia duas leituras de arquivo e comandos `curl` [conforme Claude os escreve](/docs/pt/permissions#bash-rule-limits), e `disableBypassPermissionsMode` remove o modo de permissão de bypass
* [`allowManagedPermissionRulesOnly`](/docs/pt/settings-reference#allowmanagedpermissionrulesonly) e [`allowManagedMcpServersOnly`](/docs/pt/settings-reference#allowmanagedmcpserversonly) fazem as listas de permissão gerenciadas e MCP as únicas que se aplicam
* `allowedMcpServers` fixa o servidor MCP pela URL
* `strictKnownMarketplaces` permite um marketplace de plugins
* `sandbox` coloca comandos em sandbox com uma lista de permissão de rede fixa e sem retry não sandboxed
* `requiredMinimumVersion` define uma versão mínima de Claude Code
* `cleanupPeriodDays` encurta a retenção de transcrições de sessão e outros dados locais para sete dias
* `companyAnnouncements` mostra uma mensagem na inicialização

Os administradores implantam um arquivo como este como `managed-settings.json`, ou o mesmo JSON através de MDM ou [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings). Um arquivo implantado se aplica a cada máquina ou conta que alcança. Para dar a um grupo valores diferentes, implante um arquivo ou perfil diferente para esse grupo, já que [as configurações gerenciadas pelo servidor não suportam política por grupo ainda](/docs/pt/server-managed-settings#current-limitations).

<Tabs>
  <Tab title="Arquivo de configurações copiável">
    Implante isto como `managed-settings.json`, ou o mesmo JSON através de MDM ou do console claude.ai. É JSON válido sem comentários; substitua o UUID da organização de exemplo, URL do servidor e marketplace pelos seus próprios e delete as chaves que não deseja.

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

  <Tab title="O que cada chave faz">
    O mesmo arquivo com um comentário acima de cada chave. Leia aqui; copie da outra aba, porque Claude Code não aceita comentários em um arquivo de configurações.

    ```jsonc managed-settings.json theme={null}
    {
      // Apenas logins claude.ai, e apenas nesta organização
      "forceLoginMethod": "claudeai",
      "forceLoginOrgUUID": [
        "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
      ],
      // Apenas modelos Opus e Sonnet; com enforceAvailableModels, a opção Padrão obedece à lista também
      "availableModels": [
        "opus",
        "sonnet"
      ],
      "enforceAvailableModels": true,
      "permissions": {
        // Bloqueie curl, o arquivo .env do projeto e sua pasta de segredos em cada máquina
        "deny": [
          "Bash(curl *)",
          "Read(./.env)",
          "Read(./secrets/**)"
        ],
        // Remova o modo de bypass de permissões de cada sessão
        "disableBypassPermissionsMode": "disable"
      },
      // Ignore regras de permissão de configurações de usuário, projeto e local
      "allowManagedPermissionRulesOnly": true,
      // Apenas o servidor MCP do GitHub, correspondido pela URL em vez de por nome, já que um usuário pode
      // nomear qualquer servidor "github". Servidores adicionados pelo usuário que não correspondem não carregam, incluindo
      // cada servidor stdio quando a lista tem apenas entradas de URL. A chave allowManagedMcpServersOnly
      // abaixo faz desta lista gerenciada a única lista de permissão que se aplica
      "allowedMcpServers": [
        {
          "serverUrl": "https://api.githubcopilot.com/*"
        }
      ],
      "allowManagedMcpServersOnly": true,
      // Plugins podem vir apenas deste marketplace
      "strictKnownMarketplaces": [
        {
          "source": "github",
          "repo": "acme-corp/approved-plugins"
        }
      ],
      // Coloque cada comando que Claude executa em sandbox, recuse iniciar se o sandbox não puder ser
      // configurado, e nunca deixe um comando bloqueado tentar novamente fora do sandbox; rede
      // limitada a npm e GitHub, e usuários não podem adicionar domínios
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
      // Recuse iniciar em versões mais antigas que 2.1.150
      "requiredMinimumVersion": "2.1.150",
      // Exclua transcrições de sessão e outros dados de sessão local após 7 dias
      "cleanupPeriodDays": 7,
      // Uma mensagem que cada usuário vê na inicialização
      "companyAnnouncements": [
        "Welcome to Acme Corp! Review our code guidelines at docs.example.com"
      ]
    }
    ```
  </Tab>
</Tabs>
