> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# File di impostazioni di esempio

> File settings.json realistici per uno sviluppatore, un team e un'organizzazione: copia uno, mantieni le chiavi che desideri e modifica i valori.

Questa pagina contiene tre file `settings.json` di esempio, uno per ogni luogo in cui salvi un'impostazione:

* Un file `~/.claude/settings.json` di uno sviluppatore
* Un file `.claude/settings.json` di un team, sottoposto a commit nel repository
* Un file `managed-settings.json` di un'organizzazione

Ognuno è un file plausibile per quel lettore, quindi puoi vedere la struttura e copiare le parti che desideri. Nessuno di essi è una baseline consigliata. Ogni valore proviene dalla voce della chiave nel [riferimento delle impostazioni](/docs/it/settings-reference), che contiene il suo tipo, il valore predefinito e dove può essere impostato.

Ogni esempio ha due schede. **Copyable settings file** è il file come lo salveresti. **What each key does** è lo stesso file con un commento sopra ogni chiave; Claude Code non accetta commenti in un file di impostazioni, quindi copia dalla prima scheda.

<h2 id="your-own-settings">
  Le tue impostazioni personali
</h2>

Le impostazioni personali di uno sviluppatore. Sceglie un modello e uno sforzo, regola il terminale e pre-approva un comando di sola lettura e una lettura di file. Tutto ciò che non è elencato mantiene il suo valore predefinito. Un file come questo va in `~/.claude/settings.json`, dove si applica a ogni progetto che apri.

<Tabs>
  <Tab title="Copyable settings file">
    Salva questo come `~/.claude/settings.json`. È JSON valido senza commenti, quindi puoi incollarlo così com'è ed eliminare le chiavi che non desideri.

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
    Lo stesso file con un commento sopra ogni chiave. Leggilo qui; copia dall'altra scheda, perché Claude Code non accetta commenti in un file di impostazioni.

    ```jsonc ~/.claude/settings.json theme={null}
    {
      // Inizia ogni sessione su Sonnet 5
      "model": "claude-sonnet-5",
      // Esegui Sonnet 5 al di sopra del suo livello alto predefinito; /effort salva un livello per modello, e --effort ne imposta uno per una singola sessione
      "modelSettings": {
        "claude-sonnet-5": { "effortLevel": "xhigh" }
      },
      // Scorciatoie da tastiera Vim nel prompt
      "editorMode": "vim",
      // Il tema chiaro adatto ai daltonici
      "theme": "light-daltonized",
      // Una riga di stato sotto il prompt: nome del modello e contesto utilizzato
      "statusLine": {
        "type": "command",
        "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
        "padding": 2
      },
      // Nascondi i suggerimenti che ruotano sotto lo spinner
      "spinnerTipsEnabled": false,
      // Suona il campanello del terminale per le notifiche, come un'attività completata o un prompt di autorizzazione in attesa
      "preferredNotifChannel": "terminal_bell",
      // Consenti a Claude Code di eseguire git diff e leggere il tuo .zshrc senza chiedere
      "permissions": {
        "allow": [
          "Bash(git diff *)",
          "Read(~/.zshrc)"
        ]
      },
      // Accetta gli aggiornamenti dal canale stabile
      "autoUpdatesChannel": "stable",
      // Elimina i trascritti delle sessioni e altri dati locali delle sessioni più vecchi di 20 giorni
      "cleanupPeriodDays": 20
    }
    ```
  </Tab>
</Tabs>

<h2 id="a-teams-shared-settings">
  Le impostazioni condivise di un team
</h2>

Le impostazioni condivise di un team, sottoposte a commit nel repository in modo che tutti coloro che lo clonano ottengano le stesse autorizzazioni, hook e marketplace di plugin. Salva un file come questo in `.claude/settings.json` nella parte superiore del repository. Cose da sapere prima di sottoporre a commit uno:

* **Le sessioni cloud lo leggono anche.** Una [sessione cloud](/docs/it/settings#settings-in-cloud-sessions) inizia da un clone del repository, quindi il file sottoposto a commit si applica anche lì.
* **La telemetria va nelle impostazioni gestite o personali.** Claude Code ignora le [variabili dell'esportatore OpenTelemetry](/docs/it/settings-reference#variables-claude-code-ignores-in-env) nei file di impostazioni di un repository, a parte alcuni valori che disattivano la telemetria. Impostale nelle [impostazioni gestite](/docs/it/monitoring-usage#administrator-configuration) per la tua organizzazione, o nel file `~/.claude/settings.json` di ogni persona.
* **Le regole di autorizzazione attendono la fiducia.** Le regole di autorizzazione e le voci `extraKnownMarketplaces` hanno effetto dopo che ogni persona [si fida di questa cartella stessa](/docs/it/permissions#project-allow-rules-and-workspace-trust), non solo di una cartella padre; le regole di negazione e richiesta si applicano in ogni sessione, fidata o meno.
* **L'hook è uno script nel repo.** L'hook di questo file esegue `.claude/hooks/block-rm.sh`; [How a hook resolves](/docs/it/hooks#how-a-hook-resolves) illustra come scriverlo.
* **Le regole corrispondono al comando e al percorso come scritti.** `Bash(git push *)` non corrisponde a [`git -C . push`](/docs/it/permissions#bash-rule-limits). `Read(./.env)` da solo interrompe i file tool e i comandi che nominano il file, come `cat .env`, ma non [`grep -r` eseguito sulla directory](/docs/it/permissions#read-and-edit); il blocco `sandbox` in questo file colma questa lacuna, perché la sandbox [aggiunge i tuoi percorsi di negazione `Read`](/docs/it/settings-reference#sandbox-filesystem-denyread) a ciò che ogni comando in sandbox non può leggere.

<Tabs>
  <Tab title="Copyable settings file">
    Salva questo come `.claude/settings.json` nella parte superiore del repository e sottoponi a commit. È JSON valido senza commenti, quindi puoi incollarlo così com'è ed eliminare le chiavi che non desideri.

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
    Lo stesso file con un commento sopra ogni chiave. Leggilo qui; copia dall'altra scheda, perché Claude Code non accetta commenti in un file di impostazioni.

    ```jsonc .claude/settings.json theme={null}
    {
      "permissions": {
        // Esegui gli script npm senza chiedere
        "allow": [
          "Bash(npm run *)"
        ],
        // Conferma prima dei comandi git push
        "ask": [
          "Bash(git push *)"
        ],
        // Nega le letture dei file env e della cartella dei segreti dai file tool e dai comandi che leggono i file
        "deny": [
          "Read(./.env)",
          "Read(./.env.*)",
          "Read(./secrets/**)"
        ]
      },
      // Prima di ogni comando Bash, esegui uno script nel repo che può bloccarlo
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
      // Registra il marketplace di plugin del team su ogni clone
      "extraKnownMarketplaces": {
        "acme-tools": {
          "source": {
            "source": "github",
            "repo": "acme-corp/claude-plugins"
          }
        }
      },
      // Abilita un plugin da quel marketplace; un plugin da una fonte esterna come un repository GitHub richiede comunque che ogni persona lo installi una volta
      "enabledPlugins": {
        "code-formatter@acme-tools": true
      },
      // Comandi sandbox: directory di build scrivibile; npm e example.com pre-autorizzati, altri host ancora richiedono conferma
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
      // Mantieni i file di piano all'interno del repo
      "plansDirectory": "./plans"
    }
    ```
  </Tab>
</Tabs>

<h2 id="an-organizations-managed-settings">
  Le impostazioni gestite di un'organizzazione
</h2>

Un file `managed-settings.json` che mostra la forma delle chiavi gestite, con un valore plausibile per ognuna. Non è una politica consigliata: scegli le chiavi che corrispondono ai tuoi requisiti e imposta i tuoi valori. L'esempio imposta queste chiavi:

* `forceLoginMethod` e `forceLoginOrgUUID` fissano il metodo di accesso e l'organizzazione
* `availableModels` e `enforceAvailableModels` limitano quali modelli possono utilizzare le sessioni
* `permissions.deny` blocca due letture di file e comandi `curl` [come Claude li scrive](/docs/it/permissions#bash-rule-limits), e `disableBypassPermissionsMode` rimuove la modalità di autorizzazione di bypass
* [`allowManagedPermissionRulesOnly`](/docs/it/settings-reference#allowmanagedpermissionrulesonly) e [`allowManagedMcpServersOnly`](/docs/it/settings-reference#allowmanagedmcpserversonly) rendono le liste di autorizzazione di autorizzazione e MCP gestite le uniche che si applicano
* `allowedMcpServers` fissa il server MCP per URL
* `strictKnownMarketplaces` consente un marketplace di plugin
* `sandbox` esegue il sandboxing dei comandi con una lista di autorizzazione di rete fissa e nessun retry non sandboxato
* `requiredMinimumVersion` imposta una versione minima di Claude Code
* `cleanupPeriodDays` accorcia la conservazione dei trascritti delle sessioni e altri dati locali a sette giorni
* `companyAnnouncements` mostra un messaggio all'avvio

Gli amministratori distribuiscono un file come questo come `managed-settings.json`, o lo stesso JSON tramite MDM o [impostazioni gestite dal server](/docs/it/server-managed-settings). Un file distribuito si applica a ogni macchina o account che raggiunge. Per dare a un gruppo valori diversi, distribuisci un file o profilo diverso a quel gruppo, poiché [le impostazioni gestite dal server non supportano ancora la politica per gruppo](/docs/it/server-managed-settings#current-limitations).

<Tabs>
  <Tab title="Copyable settings file">
    Distribuisci questo come `managed-settings.json`, o lo stesso JSON tramite MDM o la console claude.ai. È JSON valido senza commenti; sostituisci l'UUID dell'organizzazione di esempio, l'URL del server e il marketplace con i tuoi e elimina le chiavi che non desideri.

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
    Lo stesso file con un commento sopra ogni chiave. Leggilo qui; copia dall'altra scheda, perché Claude Code non accetta commenti in un file di impostazioni.

    ```jsonc managed-settings.json theme={null}
    {
      // Solo accessi claude.ai, e solo in questa organizzazione
      "forceLoginMethod": "claudeai",
      "forceLoginOrgUUID": [
        "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
      ],
      // Solo modelli Opus e Sonnet; con enforceAvailableModels, l'opzione Predefinito obbedisce anche alla lista
      "availableModels": [
        "opus",
        "sonnet"
      ],
      "enforceAvailableModels": true,
      "permissions": {
        // Blocca curl, il file .env del progetto e la sua cartella dei segreti su ogni macchina
        "deny": [
          "Bash(curl *)",
          "Read(./.env)",
          "Read(./secrets/**)"
        ],
        // Rimuovi la modalità di bypass delle autorizzazioni da ogni sessione
        "disableBypassPermissionsMode": "disable"
      },
      // Ignora le regole di autorizzazione dalle impostazioni utente, progetto e locali
      "allowManagedPermissionRulesOnly": true,
      // Solo il server MCP GitHub, abbinato per URL piuttosto che per nome, poiché un utente può
      // nominare qualsiasi server "github". I server aggiunti dall'utente che non corrispondono non si caricano, incluso
      // ogni server stdio quando la lista ha solo voci URL. La chiave allowManagedMcpServersOnly
      // sotto rende questa lista gestita l'unica lista di autorizzazione che si applica
      "allowedMcpServers": [
        {
          "serverUrl": "https://api.githubcopilot.com/*"
        }
      ],
      "allowManagedMcpServersOnly": true,
      // I plugin possono provenire solo da questo marketplace
      "strictKnownMarketplaces": [
        {
          "source": "github",
          "repo": "acme-corp/approved-plugins"
        }
      ],
      // Esegui il sandboxing di ogni comando che Claude esegue, rifiuta di avviare se il sandbox non può essere
      // configurato, e non consentire mai a un comando bloccato di riprovare al di fuori del sandbox; la rete
      // limitata a npm e GitHub, e gli utenti non possono aggiungere domini
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
      // Rifiuta di avviare su versioni più vecchie di 2.1.150
      "requiredMinimumVersion": "2.1.150",
      // Elimina i trascritti delle sessioni e altri dati locali delle sessioni dopo 7 giorni
      "cleanupPeriodDays": 7,
      // Un messaggio che ogni utente vede all'avvio
      "companyAnnouncements": [
        "Welcome to Acme Corp! Review our code guidelines at docs.example.com"
      ]
    }
    ```
  </Tab>
</Tabs>
