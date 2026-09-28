> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Fichiers de paramètres d'exemple

> Fichiers settings.json réalistes pour un développeur, une équipe et une organisation : copiez-en un, conservez les clés que vous voulez et modifiez les valeurs.

Cette page contient trois fichiers `settings.json` d'exemple, un pour chaque endroit où vous enregistrez un paramètre :

* Un `~/.claude/settings.json` de développeur
* Un `.claude/settings.json` d'équipe, validé dans le référentiel
* Un `managed-settings.json` d'organisation

Chacun est un fichier plausible pour ce lecteur, vous pouvez donc voir la forme et copier les parties que vous voulez. Aucun d'eux n'est une ligne de base recommandée. Chaque valeur provient de l'entrée de la clé sur la [référence des paramètres](/docs/fr/settings-reference), qui contient son type, sa valeur par défaut et l'endroit où elle peut être définie.

Chaque exemple a deux onglets. **Fichier de paramètres copiable** est le fichier tel que vous l'enregistreriez. **Ce que chaque clé fait** est le même fichier avec un commentaire au-dessus de chaque clé ; Claude Code n'accepte pas les commentaires dans un fichier de paramètres, donc copiez à partir du premier onglet.

<h2 id="your-own-settings">
  Vos propres paramètres
</h2>

Les paramètres personnels d'un développeur. Il choisit un modèle et un effort, ajuste le terminal et pré-approuve une commande en lecture seule et une lecture de fichier. Tout ce qui n'est pas listé conserve sa valeur par défaut. Un fichier comme celui-ci va dans `~/.claude/settings.json`, où il s'applique à chaque projet que vous ouvrez.

<Tabs>
  <Tab title="Fichier de paramètres copiable">
    Enregistrez ceci sous `~/.claude/settings.json`. C'est du JSON valide sans commentaires, vous pouvez donc le coller tel quel et supprimer les clés que vous ne voulez pas.

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

  <Tab title="Ce que chaque clé fait">
    Le même fichier avec un commentaire au-dessus de chaque clé. Lisez-le ici ; copiez à partir de l'autre onglet, car Claude Code n'accepte pas les commentaires dans un fichier de paramètres.

    ```jsonc ~/.claude/settings.json theme={null}
    {
      // Commencez chaque session sur Sonnet 5
      "model": "claude-sonnet-5",
      // Exécutez Sonnet 5 au-dessus de son niveau élevé par défaut ; /effort enregistre un niveau par modèle, et --effort en définit un pour une seule session
      "modelSettings": {
        "claude-sonnet-5": { "effortLevel": "xhigh" }
      },
      // Liaisons de touches Vim dans l'invite
      "editorMode": "vim",
      // Le thème clair adapté aux daltoniens
      "theme": "light-daltonized",
      // Une ligne d'état sous l'invite : nom du modèle et contexte utilisé
      "statusLine": {
        "type": "command",
        "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
        "padding": 2
      },
      // Masquez les conseils qui tournent sous le spinner
      "spinnerTipsEnabled": false,
      // Sonnez la cloche du terminal pour les notifications, comme une tâche terminée ou une invite de permission en attente
      "preferredNotifChannel": "terminal_bell",
      // Laissez Claude Code exécuter git diff et lire votre .zshrc sans demander
      "permissions": {
        "allow": [
          "Bash(git diff *)",
          "Read(~/.zshrc)"
        ]
      },
      // Prenez les mises à jour du canal stable
      "autoUpdatesChannel": "stable",
      // Supprimez les transcriptions de session et autres données de session locale plus anciennes que 20 jours
      "cleanupPeriodDays": 20
    }
    ```
  </Tab>
</Tabs>

<h2 id="a-teams-shared-settings">
  Paramètres partagés d'une équipe
</h2>

Les paramètres partagés d'une équipe, validés dans le référentiel afin que tous ceux qui le clonent obtiennent les mêmes permissions, hooks et marketplace de plugins. Enregistrez un fichier comme celui-ci à `.claude/settings.json` en haut du référentiel. Ce qu'il faut savoir avant de valider un :

* **Les sessions cloud le lisent aussi.** Une [session cloud](/docs/fr/settings#settings-in-cloud-sessions) démarre à partir d'un clone du référentiel, donc le fichier validé s'applique également là.
* **La télémétrie va dans les paramètres gérés ou personnels.** Claude Code ignore les [variables d'exportateur OpenTelemetry](/docs/fr/settings-reference#variables-claude-code-ignores-in-env) dans les fichiers de paramètres d'un référentiel, à l'exception de certaines valeurs qui désactivent la télémétrie. Définissez-les dans les [paramètres gérés](/docs/fr/monitoring-usage#administrator-configuration) de votre organisation, ou dans le fichier `~/.claude/settings.json` de chaque personne.
* **Les règles d'autorisation attendent la confiance.** Les règles d'autorisation et les entrées `extraKnownMarketplaces` prennent effet après que chaque personne [fasse confiance à ce dossier lui-même](/docs/fr/permissions#project-allow-rules-and-workspace-trust), pas seulement à un dossier parent ; les règles de refus et de demande s'appliquent dans chaque session, de confiance ou non.
* **Le hook est un script dans le référentiel.** Le hook de ce fichier exécute `.claude/hooks/block-rm.sh` ; [Comment un hook se résout](/docs/fr/hooks#how-a-hook-resolves) explique comment l'écrire.
* **Les règles correspondent à la commande et au chemin tels qu'écrits.** `Bash(git push *)` ne correspond pas à [`git -C . push`](/docs/fr/permissions#bash-rule-limits). `Read(./.env)` seul arrête les outils de fichier et les commandes qui nomment le fichier, comme `cat .env`, mais pas [`grep -r` exécuté sur le répertoire](/docs/fr/permissions#read-and-edit) ; le bloc `sandbox` dans ce fichier comble cette lacune, car le sandbox [ajoute vos chemins de refus `Read`](/docs/fr/settings-reference#sandbox-filesystem-denyread) à ce que chaque commande en sandbox ne peut pas lire.

<Tabs>
  <Tab title="Fichier de paramètres copiable">
    Enregistrez ceci sous `.claude/settings.json` en haut du référentiel et validez-le. C'est du JSON valide sans commentaires, vous pouvez donc le coller tel quel et supprimer les clés que vous ne voulez pas.

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

  <Tab title="Ce que chaque clé fait">
    Le même fichier avec un commentaire au-dessus de chaque clé. Lisez-le ici ; copiez à partir de l'autre onglet, car Claude Code n'accepte pas les commentaires dans un fichier de paramètres.

    ```jsonc .claude/settings.json theme={null}
    {
      "permissions": {
        // Exécutez les scripts npm sans demander
        "allow": [
          "Bash(npm run *)"
        ],
        // Confirmez avant les commandes git push
        "ask": [
          "Bash(git push *)"
        ],
        // Refusez les lectures des fichiers env et du dossier secrets par les outils de fichier et les commandes de lecture de fichier
        "deny": [
          "Read(./.env)",
          "Read(./.env.*)",
          "Read(./secrets/**)"
        ]
      },
      // Avant chaque commande Bash, exécutez un script dans le référentiel qui peut la bloquer
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
      // Enregistrez le marketplace de plugins de l'équipe à chaque clone
      "extraKnownMarketplaces": {
        "acme-tools": {
          "source": {
            "source": "github",
            "repo": "acme-corp/claude-plugins"
          }
        }
      },
      // Activez un plugin de ce marketplace ; un plugin d'une source externe comme un référentiel GitHub doit toujours être installé une fois par chaque personne
      "enabledPlugins": {
        "code-formatter@acme-tools": true
      },
      // Commandes sandbox : répertoire de construction inscriptible ; npm et example.com pré-autorisés, les autres hôtes demandent toujours
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
      // Gardez les fichiers de plan à l'intérieur du référentiel
      "plansDirectory": "./plans"
    }
    ```
  </Tab>
</Tabs>

<h2 id="an-organizations-managed-settings">
  Paramètres gérés d'une organisation
</h2>

Un fichier `managed-settings.json` qui montre la forme des clés gérées, avec une valeur plausible pour chacune. Ce n'est pas une politique recommandée : choisissez les clés qui correspondent à vos propres exigences et définissez vos propres valeurs. L'exemple définit ces clés :

* `forceLoginMethod` et `forceLoginOrgUUID` épinglent la méthode de connexion et l'organisation
* `availableModels` et `enforceAvailableModels` limitent les modèles que les sessions peuvent utiliser
* `permissions.deny` refuse deux lectures de fichiers et les commandes `curl` [comme Claude les écrit](/docs/fr/permissions#bash-rule-limits), et `disableBypassPermissionsMode` supprime le mode de permission de contournement
* [`allowManagedPermissionRulesOnly`](/docs/fr/settings-reference#allowmanagedpermissionrulesonly) et [`allowManagedMcpServersOnly`](/docs/fr/settings-reference#allowmanagedmcpserversonly) font des listes blanches de permission et MCP gérées les seules qui s'appliquent
* `allowedMcpServers` épingle le serveur MCP par URL
* `strictKnownMarketplaces` autorise un marketplace de plugins
* `sandbox` bac à sable les commandes avec une liste d'autorisation réseau fixe et pas de nouvelle tentative non bac à sable
* `requiredMinimumVersion` définit une version minimale de Claude Code
* `cleanupPeriodDays` raccourcit la rétention des transcriptions de session et autres données locales à sept jours
* `companyAnnouncements` affiche un message au démarrage

Les administrateurs déploient un fichier comme celui-ci en tant que `managed-settings.json`, ou le même JSON via MDM ou [paramètres gérés par serveur](/docs/fr/server-managed-settings). Un fichier déployé s'applique à chaque machine ou compte qu'il atteint. Pour donner à un groupe des valeurs différentes, déployez un fichier ou un profil différent à ce groupe, car [les paramètres gérés par serveur ne supportent pas encore la politique par groupe](/docs/fr/server-managed-settings#current-limitations).

<Tabs>
  <Tab title="Fichier de paramètres copiable">
    Déployez ceci en tant que `managed-settings.json`, ou le même JSON via MDM ou la console claude.ai. C'est du JSON valide sans commentaires ; remplacez l'UUID d'organisation d'exemple, l'URL du serveur et le marketplace par les vôtres et supprimez les clés que vous ne voulez pas.

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

  <Tab title="Ce que chaque clé fait">
    Le même fichier avec un commentaire au-dessus de chaque clé. Lisez-le ici ; copiez à partir de l'autre onglet, car Claude Code n'accepte pas les commentaires dans un fichier de paramètres.

    ```jsonc managed-settings.json theme={null}
    {
      // Uniquement les connexions claude.ai, et uniquement dans cette organisation
      "forceLoginMethod": "claudeai",
      "forceLoginOrgUUID": [
        "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
      ],
      // Uniquement les modèles Opus et Sonnet ; avec enforceAvailableModels, l'option Par défaut obéit également à la liste
      "availableModels": [
        "opus",
        "sonnet"
      ],
      "enforceAvailableModels": true,
      "permissions": {
        // Refusez les commandes curl et les lectures du fichier .env du projet et du dossier secrets sur chaque machine
        "deny": [
          "Bash(curl *)",
          "Read(./.env)",
          "Read(./secrets/**)"
        ],
        // Supprimez le mode de permission de contournement de chaque session
        "disableBypassPermissionsMode": "disable"
      },
      // Ignorez les règles de permission des paramètres utilisateur, projet et locaux
      "allowManagedPermissionRulesOnly": true,
      // Uniquement le serveur MCP GitHub, mis en correspondance par URL plutôt que par nom, car un utilisateur peut
      // nommer n'importe quel serveur « github ». Les serveurs ajoutés par l'utilisateur qui ne correspondent pas ne se chargent pas, y compris
      // chaque serveur stdio quand la liste n'a que des entrées URL. La clé allowManagedMcpServersOnly
      // ci-dessous rend cette liste gérée la seule liste blanche qui s'applique
      "allowedMcpServers": [
        {
          "serverUrl": "https://api.githubcopilot.com/*"
        }
      ],
      "allowManagedMcpServersOnly": true,
      // Les plugins ne peuvent provenir que de ce marketplace
      "strictKnownMarketplaces": [
        {
          "source": "github",
          "repo": "acme-corp/approved-plugins"
        }
      ],
      // Bac à sable chaque commande que Claude exécute, refusez de démarrer si le bac à sable ne peut pas être
      // configuré, et ne laissez jamais une commande bloquée réessayer en dehors du bac à sable ; réseau
      // limité à npm et GitHub, et les utilisateurs ne peuvent pas ajouter de domaines
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
      // Refusez de démarrer sur les versions antérieures à 2.1.150
      "requiredMinimumVersion": "2.1.150",
      // Supprimez les transcriptions de session et autres données de session locale après 7 jours
      "cleanupPeriodDays": 7,
      // Un message que chaque utilisateur voit au démarrage
      "companyAnnouncements": [
        "Welcome to Acme Corp! Review our code guidelines at docs.example.com"
      ]
    }
    ```
  </Tab>
</Tabs>
