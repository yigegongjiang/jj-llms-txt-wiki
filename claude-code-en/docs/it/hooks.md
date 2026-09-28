> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Riferimento dei hooks

> Riferimento per gli eventi dei hook di Claude Code, schema di configurazione, formati JSON di input/output, codici di uscita, hook asincroni, hook HTTP, hook di prompt e hook degli strumenti MCP.

<Tip>
  Per una guida di avvio rapido con esempi, consultare [Automatizzare i flussi di lavoro con i hook](/docs/it/hooks-guide).
</Tip>

Gli hook sono comandi shell definiti dall'utente, endpoint HTTP, chiamate agli strumenti MCP, prompt LLM o subagent che si eseguono automaticamente in punti specifici del ciclo di vita di Claude Code. Claude Code attiva gli stessi eventi di hook ovunque venga eseguito: sessioni nel terminale, estensioni IDE, l'[app Desktop](/docs/it/desktop-quickstart) e [Claude Code sul web](/docs/it/claude-code-on-the-web). Utilizzare questo riferimento per cercare schemi di eventi, opzioni di configurazione, formati JSON di input/output e funzionalità avanzate come hook asincroni, hook HTTP e hook degli strumenti MCP.

<h2 id="hook-lifecycle">
  Ciclo di vita dei hook
</h2>

Claude Code esegue i hook in punti specifici durante una sessione. Quando un evento si attiva e un matcher corrisponde, Claude Code passa il contesto JSON dell'evento al gestore del hook. Per i hook di comando, l'input arriva su stdin. Per i hook HTTP, arriva come corpo della richiesta POST. Il gestore può quindi ispezionare l'input, intraprendere un'azione e facoltativamente restituire una decisione.

Gli eventi si dividono in tre cadenze:

* per sessione: `SessionStart` e `SessionEnd`
* per turno: `UserPromptSubmit`, `Stop` e `StopFailure`
* ad ogni chiamata dello strumento all'interno del ciclo agentico: `PreToolUse` e `PostToolUse`, ad eccezione delle chiamate [`EndConversation`](/docs/it/tools-reference#endconversation-tool-behavior), che saltano entrambe

<div style={{maxWidth: "500px", margin: "0 auto"}}>
  <Frame>
    <img src="https://mintcdn.com/claude-code/x7pO8l4XcvAXCoVc/images/hooks-lifecycle.svg?fit=max&auto=format&n=x7pO8l4XcvAXCoVc&q=85&s=81b9256c1bbe8832553485f5d9e9c746" className="dark:hidden" alt="Diagramma del ciclo di vita dei hook che mostra Setup facoltativo che alimenta SessionStart, quindi un ciclo per turno contenente UserPromptSubmit, UserPromptExpansion per slash commands, il ciclo agentico annidato (PreToolUse, PermissionRequest, PostToolUse, PostToolUseFailure, PostToolBatch, SubagentStart/Stop, TaskCreated, TaskCompleted), e Stop o StopFailure, seguito da TeammateIdle, PreCompact, PostCompact e SessionEnd, con Elicitation e ElicitationResult annidati all'interno dell'esecuzione dello strumento MCP, PermissionDenied come ramo laterale di PermissionRequest per i rifiuti in modalità automatica, WorktreeCreate, WorktreeRemove, Notification, ConfigChange, InstructionsLoaded, CwdChanged, FileChanged e DirectoryAdded come eventi asincroni autonomi, PreModelSwitch come evento sequenziale autonomo che viene eseguito prima di un cambio di modello richiesto, PostModelSwitch come evento asincrono autonomo che viene eseguito dopo il cambio del modello della sessione, e MessageDisplay come evento di sola visualizzazione che viene eseguito mentre il testo del messaggio dell'assistente viene trasmesso in streaming" width="520" height="1336" data-path="images/hooks-lifecycle.svg" />

    <img src="https://mintcdn.com/claude-code/x7pO8l4XcvAXCoVc/images/hooks-lifecycle-dark.svg?fit=max&auto=format&n=x7pO8l4XcvAXCoVc&q=85&s=c9b3d88487335f58cce0b52e2f9e7531" className="hidden dark:block" alt="Diagramma del ciclo di vita dei hook che mostra Setup facoltativo che alimenta SessionStart, quindi un ciclo per turno contenente UserPromptSubmit, UserPromptExpansion per slash commands, il ciclo agentico annidato (PreToolUse, PermissionRequest, PostToolUse, PostToolUseFailure, PostToolBatch, SubagentStart/Stop, TaskCreated, TaskCompleted), e Stop o StopFailure, seguito da TeammateIdle, PreCompact, PostCompact e SessionEnd, con Elicitation e ElicitationResult annidati all'interno dell'esecuzione dello strumento MCP, PermissionDenied come ramo laterale di PermissionRequest per i rifiuti in modalità automatica, WorktreeCreate, WorktreeRemove, Notification, ConfigChange, InstructionsLoaded, CwdChanged, FileChanged e DirectoryAdded come eventi asincroni autonomi, PreModelSwitch come evento sequenziale autonomo che viene eseguito prima di un cambio di modello richiesto, PostModelSwitch come evento asincrono autonomo che viene eseguito dopo il cambio del modello della sessione, e MessageDisplay come evento di sola visualizzazione che viene eseguito mentre il testo del messaggio dell'assistente viene trasmesso in streaming" width="520" height="1336" data-path="images/hooks-lifecycle-dark.svg" />
  </Frame>
</div>

La tabella seguente riassume quando si attiva ogni evento. La sezione [Hook events](#hook-events) documenta lo schema di input completo e le opzioni di controllo della decisione per ognuno.

| Evento                | Quando si attiva                                                                                                                                                                                                                                                                                                                        |
| :-------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `SessionStart`        | Quando una sessione inizia o riprende                                                                                                                                                                                                                                                                                                   |
| `Setup`               | Quando avvii Claude Code con `--init-only`, o con `--init` o `--maintenance` in modalità `-p`. Per la preparazione una tantum in CI o script                                                                                                                                                                                            |
| `UserPromptSubmit`    | Quando invii un prompt, prima che Claude lo elabori                                                                                                                                                                                                                                                                                     |
| `UserPromptExpansion` | Quando un comando digitato dall'utente si espande in un prompt, prima che raggiunga Claude. Può bloccare l'espansione                                                                                                                                                                                                                   |
| `PreToolUse`          | Prima che una chiamata a uno strumento si esegua. Può bloccarla                                                                                                                                                                                                                                                                         |
| `PermissionRequest`   | Quando una chiamata a uno strumento necessita di una decisione di autorizzazione                                                                                                                                                                                                                                                        |
| `PermissionDenied`    | Quando la modalità automatica nega una chiamata a uno strumento, inclusi i rifiuti senza un verdetto del classificatore. Utilizza JSON `hookSpecificOutput.retry: true` per indicare al modello che può riprovare la chiamata allo strumento negata. Claude Code ignora `retry` quando il classificatore non ha prodotto alcun verdetto |
| `PostToolUse`         | Dopo che una chiamata a uno strumento ha successo                                                                                                                                                                                                                                                                                       |
| `PostToolUseFailure`  | Dopo che una chiamata a uno strumento fallisce                                                                                                                                                                                                                                                                                          |
| `PostToolBatch`       | Dopo che un intero batch di chiamate a strumenti paralleli si risolve, prima della prossima chiamata al modello                                                                                                                                                                                                                         |
| `Notification`        | Quando Claude Code invia una notifica                                                                                                                                                                                                                                                                                                   |
| `MessageDisplay`      | Mentre il testo del messaggio dell'assistente viene visualizzato                                                                                                                                                                                                                                                                        |
| `SubagentStart`       | Quando un subagente viene generato                                                                                                                                                                                                                                                                                                      |
| `SubagentStop`        | Quando un subagente termina                                                                                                                                                                                                                                                                                                             |
| `TaskCreated`         | Quando un'attività viene creata tramite `TaskCreate`                                                                                                                                                                                                                                                                                    |
| `TaskCompleted`       | Quando un'attività viene contrassegnata come completata                                                                                                                                                                                                                                                                                 |
| `Stop`                | Quando Claude finisce di rispondere                                                                                                                                                                                                                                                                                                     |
| `StopFailure`         | Quando il turno termina a causa di un errore API                                                                                                                                                                                                                                                                                        |
| `TeammateIdle`        | Quando un compagno di squadra di un [team di agenti](/docs/it/agent-teams) sta per diventare inattivo                                                                                                                                                                                                                                        |
| `InstructionsLoaded`  | Quando un file CLAUDE.md o `.claude/rules/*.md` viene caricato nel contesto. Si attiva all'inizio della sessione e quando i file vengono caricati in modo pigro durante una sessione                                                                                                                                                    |
| `ConfigChange`        | Quando un file di configurazione cambia durante una sessione                                                                                                                                                                                                                                                                            |
| `CwdChanged`          | Quando la directory di lavoro cambia, ad esempio quando Claude esegue un comando `cd`. Utile per la gestione reattiva dell'ambiente con strumenti come direnv                                                                                                                                                                           |
| `DirectoryAdded`      | Quando una directory di lavoro viene aggiunta a metà sessione tramite `/add-dir` o la richiesta di controllo SDK `register_repo_root`                                                                                                                                                                                                   |
| `FileChanged`         | Quando un file osservato cambia su disco. Il campo `matcher` specifica quali nomi di file osservare                                                                                                                                                                                                                                     |
| `WorktreeCreate`      | Quando un worktree viene creato tramite `--worktree`, `isolation: "worktree"`, o per una sessione in background. Sostituisce il comportamento git predefinito                                                                                                                                                                           |
| `WorktreeRemove`      | Quando un worktree viene rimosso all'uscita della sessione, quando un subagente termina, o quando elimini una sessione in background                                                                                                                                                                                                    |
| `PreCompact`          | Prima della compattazione del contesto                                                                                                                                                                                                                                                                                                  |
| `PostCompact`         | Dopo che la compattazione del contesto è completata                                                                                                                                                                                                                                                                                     |
| `PreModelSwitch`      | Prima che Claude Code applichi un cambio di modello che hai richiesto tu o un client. Può bloccare il cambio                                                                                                                                                                                                                            |
| `PostModelSwitch`     | Dopo che il modello della sessione cambia, inclusi i cambiamenti che Claude Code effettua autonomamente, come il ripristino del modello quando riprendi una sessione                                                                                                                                                                    |
| `Elicitation`         | Quando un server MCP richiede input dell'utente durante una chiamata a uno strumento                                                                                                                                                                                                                                                    |
| `ElicitationResult`   | Dopo che un utente risponde a un'elicitazione MCP, prima che la risposta venga inviata al server                                                                                                                                                                                                                                        |
| `SessionEnd`          | Quando una sessione termina                                                                                                                                                                                                                                                                                                             |

<h3 id="how-a-hook-resolves">
  Come si risolve un hook
</h3>

Per vedere come l'evento, il matcher e il gestore si combinano insieme, considerare questo hook `PreToolUse` che blocca i comandi shell distruttivi.

<Tabs>
  <Tab title="macOS/Linux">
    Il `matcher` si restringe alle chiamate dello strumento Bash e la condizione `if` si restringe ulteriormente ai sottocomandi Bash che corrispondono a `rm *`, quindi `block-rm.sh` viene eseguito solo quando entrambi i filtri corrispondono:

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "if": "Bash(rm *)",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.sh",
                "args": []
              }
            ]
          }
        ]
      }
    }
    ```

    Lo script legge l'input JSON da stdin, estrae il comando e restituisce una `permissionDecision` di `"deny"` se contiene `rm -rf`. Salvarlo in `.claude/hooks/block-rm.sh` nel progetto e renderlo eseguibile con `chmod +x .claude/hooks/block-rm.sh` in modo che Claude Code possa eseguirlo:

    ```bash theme={null}
    #!/bin/bash
    # .claude/hooks/block-rm.sh
    COMMAND=$(jq -r '.tool_input.command')

    if echo "$COMMAND" | grep -q 'rm -rf'; then
      jq -n '{
        hookSpecificOutput: {
          hookEventName: "PreToolUse",
          permissionDecision: "deny",
          permissionDecisionReason: "Destructive command blocked by hook"
        }
      }'
    else
      exit 0  # nessuna decisione; il flusso di autorizzazione normale si applica
    fi
    ```

    Questo script, come gli altri esempi Bash su questa pagina che analizzano l'input JSON, utilizza `jq`, quindi installare `jq` e assicurarsi che sia nel `PATH` prima di provarli.
  </Tab>

  <Tab title="Windows (PowerShell)">
    Il matcher `Bash|PowerShell` copre lo [strumento PowerShell](#powershell) così come Bash. Una singola regola `if` corrisponde solo alle chiamate di uno strumento, quindi ogni strumento ottiene il suo gestore: il primo si restringe ai sottocomandi Bash che corrispondono a `rm *`, il secondo ai comandi PowerShell che corrispondono a `Remove-Item *`. Entrambi eseguono lo stesso script tramite `powershell.exe`:

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash|PowerShell",
            "hooks": [
              {
                "type": "command",
                "if": "Bash(rm *)",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.ps1"
                ]
              },
              {
                "type": "command",
                "if": "PowerShell(Remove-Item *)",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.ps1"
                ]
              }
            ]
          }
        ]
      }
    }
    ```

    Il flag `-NoProfile` salta il caricamento del profilo PowerShell in modo che l'hook si avvii rapidamente, e `-ExecutionPolicy Bypass` consente a PowerShell di eseguire il file di script locale.

    Lo script legge l'input JSON da stdin, estrae il comando e restituisce una `permissionDecision` di `"deny"` se contiene `rm -rf` o `Remove-Item` seguito da `-Recurse`. Salvarlo in `.claude/hooks/block-rm.ps1` nel progetto:

    ```powershell theme={null}
    # .claude/hooks/block-rm.ps1
    $callInput = [Console]::In.ReadToEnd() | ConvertFrom-Json
    $command = $callInput.tool_input.command

    if ($command -match 'rm -rf|Remove-Item.*-Recurse') {
      @{
        hookSpecificOutput = @{
          hookEventName = "PreToolUse"
          permissionDecision = "deny"
          permissionDecisionReason = "Destructive command blocked by hook"
        }
      } | ConvertTo-Json
    } else {
      exit 0  # nessuna decisione; il flusso di autorizzazione normale si applica
    }
    ```
  </Tab>
</Tabs>

Supponiamo che Claude Code decida di eseguire `Bash "rm -rf /tmp/build"` rispetto alla configurazione macOS/Linux. Ecco cosa accade:

<Frame>
  <img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/hook-resolution.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=be0bf3053550c26de5f54cd64674c197" className="dark:hidden" alt="Diagramma della risoluzione del hook: PreToolUse si attiva, il matcher controlla la corrispondenza di Bash, quindi la condizione if controlla la corrispondenza di Bash(rm *). Se entrambi corrispondono, il comando del hook viene eseguito e restituisce permissionDecision deny, quindi la chiamata dello strumento viene bloccata e Claude Code continua. Se uno dei controlli non corrisponde, l'hook viene saltato e la chiamata dello strumento è autorizzata a procedere." width="930" height="270" data-path="images/hook-resolution.svg" />

  <img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/hook-resolution-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=e80af91f8507cee6bd51ac3c2dd92f63" className="hidden dark:block" alt="Diagramma della risoluzione del hook: PreToolUse si attiva, il matcher controlla la corrispondenza di Bash, quindi la condizione if controlla la corrispondenza di Bash(rm *). Se entrambi corrispondono, il comando del hook viene eseguito e restituisce permissionDecision deny, quindi la chiamata dello strumento viene bloccata e Claude Code continua. Se uno dei controlli non corrisponde, l'hook viene saltato e la chiamata dello strumento è autorizzata a procedere." width="930" height="270" data-path="images/hook-resolution-dark.svg" />
</Frame>

<Steps>
  <Step title="L'evento si attiva">
    L'evento `PreToolUse` si attiva. Claude Code invia l'input dello strumento come JSON su stdin al hook:

    ```json theme={null}
    { "tool_name": "Bash", "tool_input": { "command": "rm -rf /tmp/build" }, ... }
    ```
  </Step>

  <Step title="Il matcher controlla">
    Il matcher `"Bash"` corrisponde al nome dello strumento, quindi questo gruppo di hook si attiva. Se si omette il matcher o si utilizza `"*"`, il gruppo si attiva ad ogni occorrenza dell'evento.
  </Step>

  <Step title="La condizione if controlla">
    La condizione `if` `"Bash(rm *)"` corrisponde perché `rm -rf /tmp/build` è un sottocomando che corrisponde a `rm *`, quindi questo gestore viene eseguito. Se il comando fosse stato `npm test`, il controllo `if` avrebbe fallito e `block-rm.sh` non sarebbe mai stato eseguito, evitando il sovraccarico di spawn del processo. Il campo `if` è facoltativo; senza di esso, ogni gestore nel gruppo corrispondente viene eseguito.
  </Step>

  <Step title="Il gestore del hook viene eseguito">
    Lo script ispeziona il comando completo e trova `rm -rf`, quindi stampa una decisione su stdout:

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PreToolUse",
        "permissionDecision": "deny",
        "permissionDecisionReason": "Destructive command blocked by hook"
      }
    }
    ```

    Se il comando fosse stato una variante più sicura di `rm` come `rm file.txt`, lo script avrebbe raggiunto `exit 0` invece. Il codice di uscita 0 senza output significa che l'hook non ha alcuna decisione da segnalare, quindi la chiamata dello strumento continua attraverso il normale [flusso di autorizzazione](/docs/it/permissions). L'hook può negare la chiamata, ma rimanere in silenzio non la approva.
  </Step>

  <Step title="Claude Code agisce sul risultato">
    Claude Code legge la decisione JSON, blocca la chiamata dello strumento e mostra a Claude il motivo.
  </Step>
</Steps>

La sezione [Configuration](#configuration) seguente documenta lo schema completo, e ogni sezione [hook event](#hook-events) documenta quale input riceve il comando e quale output può restituire.

<h2 id="configuration">
  Configurazione
</h2>

Gli hook sono definiti in file di impostazioni JSON. La configurazione ha tre livelli di annidamento:

1. Scegli un [evento hook](#hook-events) a cui rispondere, come `PreToolUse` o `Stop`
2. Aggiungi un [gruppo matcher](#matcher-patterns) per filtrare quando si attiva, come "solo per lo strumento Bash"
3. Definisci uno o più [handler hook](#hook-handler-fields) da eseguire quando corrisponde

Vedi [Come si risolve un hook](#how-a-hook-resolves) sopra per una procedura dettagliata completa con un esempio annotato.

<Note>
  Questa pagina utilizza termini specifici per ogni livello: **hook event** per il punto del ciclo di vita, **matcher group** per il filtro, e **hook handler** per il comando shell, endpoint HTTP, strumento MCP, prompt, o agente che viene eseguito. "Hook" da solo si riferisce alla funzione generale.
</Note>

<h3 id="hook-locations">
  Posizioni degli hook
</h3>

Il luogo in cui definisci un hook determina il suo ambito:

| Posizione                                         | Ambito                                                                                                                | Condivisibile                                                   |
| :------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------- |
| `~/.claude/settings.json`                         | Tutti i tuoi progetti                                                                                                 | No, locale sulla tua macchina                                   |
| `.claude/settings.json`                           | Singolo progetto                                                                                                      | Sì, può essere committato nel repo                              |
| `.claude/settings.local.json`                     | Singolo progetto                                                                                                      | No, gitignored quando Claude Code salva un'impostazione in esso |
| Impostazioni di policy gestite                    | A livello di organizzazione                                                                                           | Sì, controllato dall'amministratore                             |
| [Plugin](/docs/it/plugins/overview) `hooks/hooks.json` | Quando il plugin è abilitato                                                                                          | Sì, incluso nel plugin                                          |
| [Skill](/docs/it/skills) frontmatter                   | Il resto della sessione una volta che lo skill è invocato. Vedi [Hook in skill e agenti](#hooks-in-skills-and-agents) | Sì, definito nel file dello skill                               |
| [Subagent](/docs/it/sub-agents) frontmatter            | Mentre quel subagent è in esecuzione                                                                                  | Sì, definito nel file del subagent                              |

[Le sessioni cloud](/docs/it/claude-code-on-the-web) non leggono il tuo `~/.claude/settings.json` locale. In un [ambiente self-hosted](/docs/it/self-hosted-environments-configuration#permissions-and-tool-approval), Claude Code esegue anche gli hook che l'operatore ha seminato da `~/.claude/` dell'host runner, e esegue gli hook nel file di impostazioni gestite dell'immagine runner quando quel file è tra le [fonti gestite che Claude Code applica](/docs/it/managed-settings#how-claude-code-combines-managed-sources), il che per impostazione predefinita significa solo quando né le impostazioni gestite dal server né una policy Claude Code consegnata da MDM forniscono il livello gestito. Vedi [cosa viene trasferito dalla tua configurazione](/docs/it/cloud-environments#what-carries-over-from-your-setup) per quali file di impostazioni e plugin, e quindi quali hook, raggiungono una sessione cloud.

Per i dettagli sulla risoluzione dei file di impostazioni, vedi [settings](/docs/it/settings).

Gli hook dai file di impostazioni, dalle impostazioni di policy gestite e dai plugin vengono eseguiti anche all'interno di [subagenti](/docs/it/sub-agents). Quando un subagent chiama uno strumento, gli eventi dello strumento come `PreToolUse` e `PostToolUse` attivano gli stessi hook configurati della conversazione principale, e l'input contiene i campi di input comuni `agent_id` e `agent_type` [](#common-input-fields) che identificano il subagent.

Gli amministratori aziendali possono utilizzare `allowManagedHooksOnly` per limitare quali hook vengono eseguiti:

* I tuoi hook utente, progetto, locale e plugin sono bloccati. Gli hook dai plugin forzatamente abilitati nelle impostazioni gestite `enabledPlugins` sono esenti
* Claude Code restringe anche le tue impostazioni [`statusLine`](/docs/it/statusline), [`fileSuggestion`](/docs/it/settings-reference#filesuggestion), e [`subagentStatusLine`](/docs/it/statusline#subagent-status-lines) alle impostazioni gestite
* Claude Code disabilita anche i plugin con una [`command` source](/docs/it/plugins/marketplace-reference#command-plugin-source), inclusi i plugin forzatamente abilitati nelle impostazioni gestite `enabledPlugins`, a meno che [`disableCommandPluginSources`](/docs/it/settings-reference#disablecommandpluginsources) non sia esplicitamente impostato su `false`. Le `command` sources richiedono Claude Code v2.1.229 o successivo
* Claude Code blocca anche i comandi [`headersHelper`](/docs/it/plugins/host-marketplace#authenticate-archive-downloads) del marketplace a meno che [`disableCommandPluginSources`](/docs/it/settings-reference#disablecommandpluginsources) non sia esplicitamente impostato su `false`, tranne per un marketplace che le impostazioni gestite stesse dichiarano

Vedi [cosa viene eseguito sotto `allowManagedHooksOnly`](/docs/it/settings-reference#what-runs-under-allowmanagedhooksonly).

Le voci degli hook si uniscono tra i livelli di impostazioni piuttosto che sostituirsi a vicenda: le impostazioni utente, progetto e locale aggiungono i loro hook senza rimuovere quelli gestiti, e l'impostazione [`disableAllHooks`](#disable-or-remove-hooks) non può disabilitare gli hook gestiti da fuori le impostazioni gestite.

Le [allowlist degli hook HTTP](/docs/it/settings-reference#hook-and-skill-settings) si applicano agli hook da ogni fonte, incluse le impostazioni di policy gestite:

* `allowedHttpHookUrls`: quando definito a qualsiasi livello di impostazioni, Claude Code esegue un handler hook HTTP solo se il suo URL corrisponde all'allowlist unito
* `httpHookAllowedEnvVars`: quando definito, Claude Code interpola solo le variabili di ambiente in quella lista negli header degli hook

<h3 id="matcher-patterns">
  Modelli matcher
</h3>

Il campo `matcher` filtra quando gli hook si attivano. Come viene valutato un matcher dipende dai caratteri che contiene:

| Valore matcher                                    | Valutato come                                                                                             | Esempio                                                                                                                                                                                       |
| :------------------------------------------------ | :-------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"*"`, `""`, o omesso                             | Corrisponde a tutto                                                                                       | si attiva ad ogni occorrenza dell'evento                                                                                                                                                      |
| Solo lettere, cifre, `_`, `-`, spazi, `,`, e `\|` | Stringa esatta, o lista di stringhe esatte separate da `\|` o `,` con spazi bianchi opzionali circostanti | `Bash` corrisponde solo allo strumento Bash; `Edit\|Write` e `Edit, Write` corrispondono ciascuno a uno dei due strumenti esattamente; `code-reviewer` corrisponde solo a quel tipo di agente |
| Contiene qualsiasi altro carattere                | Espressione regolare JavaScript, non ancorata                                                             | `^Notebook` corrisponde a qualsiasi strumento il cui nome inizia con `Notebook`; `mcp__memory__.*` corrisponde a ogni strumento dal server `memory`                                           |

Un matcher sul percorso dell'espressione regolare viene testato con `RegExp.prototype.test` di JavaScript, che ha successo su una corrispondenza in qualsiasi punto del valore. `Edit.*` corrisponde sia a `Edit` che a `NotebookEdit`; racchiudi il pattern in `^` e `$`, come in `^Edit$`, quando hai bisogno di una corrispondenza di intera stringa.

I trattini nel set di corrispondenza esatta richiedono Claude Code v2.1.195 o successivo. Nelle versioni precedenti un nome con trattino come `code-reviewer` viene valutato come un'espressione regolare non ancorata, quindi si attiva anche per `senior-code-reviewer`; ancoratelo come `^code-reviewer$` in quelle versioni per corrispondere solo a quel nome.

`FileChanged` e `StopFailure` utilizzano un set di corrispondenza esatta più ristretto di sole lettere, cifre, `_`, e `|`. Un trattino, spazio, o virgola in un matcher per questi due eventi lo mantiene sul percorso dell'espressione regolare, e solo `|` separa le alternative. Ogni altro evento con supporto matcher nella tabella che segue accetta `|` o `,`.

L'evento `FileChanged` non segue queste regole quando costruisce la sua lista di osservazione. Vedi [FileChanged](#filechanged).

Ogni tipo di evento corrisponde su un campo diverso:

| Evento                                                                                                                                            | Cosa filtra il matcher                                                                                    | Valori matcher di esempio                                                                                                                                                                                                                                                      |
| :------------------------------------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`                                                        | nome dello strumento                                                                                      | `Bash`, `Edit\|Write`, `mcp__.*`                                                                                                                                                                                                                                               |
| `SessionStart`                                                                                                                                    | come è iniziata la sessione                                                                               | `startup`, `resume`, `clear`, `compact`, `fork`                                                                                                                                                                                                                                |
| `Setup`                                                                                                                                           | quale flag CLI ha attivato il setup                                                                       | `init`, `maintenance`                                                                                                                                                                                                                                                          |
| `SessionEnd`                                                                                                                                      | perché è terminata la sessione                                                                            | `clear`, `resume`, `logout`, `prompt_input_exit`, `other`                                                                                                                                                                                                                      |
| `Notification`                                                                                                                                    | tipo di notifica                                                                                          | `permission_prompt`, `idle_prompt`, `auth_success`, `elicitation_dialog`, `elicitation_url_dialog`, `elicitation_complete`, `elicitation_response`, `agent_needs_input`, `agent_completed`, `quota_auto_resume_fired`, `quota_auto_resume_stale`, `quota_auto_resume_disabled` |
| `SubagentStart`                                                                                                                                   | tipo di agente                                                                                            | `general-purpose`, `Explore`, `Plan`, nomi di agenti personalizzati, o nomi con scope plugin come `^my-plugin:reviewer$`                                                                                                                                                       |
| `PreCompact`, `PostCompact`                                                                                                                       | cosa ha attivato la compattazione                                                                         | `manual`, `auto`                                                                                                                                                                                                                                                               |
| `PreModelSwitch`, `PostModelSwitch`                                                                                                               | nome canonico del modello a cui la sessione passa, come descritto sotto [PreModelSwitch](#premodelswitch) | `claude-opus-5`, `claude-opus-4-6\|claude-opus-5`, `.*opus.*`                                                                                                                                                                                                                  |
| `SubagentStop`                                                                                                                                    | tipo di agente                                                                                            | stessi valori di `SubagentStart`                                                                                                                                                                                                                                               |
| `ConfigChange`                                                                                                                                    | fonte di configurazione                                                                                   | `user_settings`, `project_settings`, `local_settings`, `policy_settings`, `skills`                                                                                                                                                                                             |
| `CwdChanged`                                                                                                                                      | nessun supporto matcher                                                                                   | si attiva sempre ad ogni occorrenza                                                                                                                                                                                                                                            |
| `DirectoryAdded`                                                                                                                                  | come è stata aggiunta la directory                                                                        | `slash_command`, `register_repo_root`                                                                                                                                                                                                                                          |
| `FileChanged`                                                                                                                                     | nomi di file letterali da osservare (vedi [FileChanged](#filechanged))                                    | `.envrc\|.env`                                                                                                                                                                                                                                                                 |
| `StopFailure`                                                                                                                                     | tipo di errore                                                                                            | `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `account_on_hold`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `cloud_credential_error`, `unknown`                                               |
| `InstructionsLoaded`                                                                                                                              | motivo del caricamento                                                                                    | `session_start`, `nested_traversal`, `path_glob_match`, `include`, `compact`                                                                                                                                                                                                   |
| `UserPromptExpansion`                                                                                                                             | nome del comando                                                                                          | i tuoi nomi di skill o comando                                                                                                                                                                                                                                                 |
| `Elicitation`                                                                                                                                     | nome del server MCP                                                                                       | i tuoi nomi di server MCP configurati                                                                                                                                                                                                                                          |
| `ElicitationResult`                                                                                                                               | nome del server MCP                                                                                       | stessi valori di `Elicitation`                                                                                                                                                                                                                                                 |
| `UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `MessageDisplay` | nessun supporto matcher                                                                                   | si attiva sempre ad ogni occorrenza                                                                                                                                                                                                                                            |

La corrispondenza di `StopFailure` su `cloud_credential_error` richiede Claude Code v2.1.267 o successivo, la prima versione che segnala i fallimenti di caricamento delle credenziali sotto quel valore piuttosto che `server_error` o `unknown`.

Per la maggior parte degli eventi, Claude Code valuta il matcher rispetto a un campo dall'[input JSON](#hook-input-and-output) che invia al tuo hook su stdin. Per gli eventi dello strumento, quel campo è `tool_name`. Per `PreModelSwitch` e `PostModelSwitch`, Claude Code valuta il matcher rispetto al nome canonico che deriva da `to_model`, come descritto sotto [PreModelSwitch](#premodelswitch). Ogni sezione [hook event](#hook-events) elenca l'insieme completo dei valori matcher e lo schema di input per quell'evento.

Questo esempio esegue uno script di linting solo quando Claude scrive o modifica un file:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/lint-check.sh"
          }
        ]
      }
    ]
  }
}
```

Se aggiungi un campo `matcher` a un evento senza supporto matcher, viene silenziosamente ignorato.

Per gli eventi dello strumento, puoi filtrare più strettamente impostando il campo [`if`](#common-fields) sui singoli handler hook. `if` utilizza la [sintassi delle regole di permesso](/docs/it/permissions) per corrispondere al nome dello strumento e agli argomenti insieme, quindi `"Bash(git *)"` viene eseguito quando qualsiasi sottocomando dell'input Bash corrisponde a `git *` e `"Edit(*.ts)"` viene eseguito solo per i file TypeScript.

<h4 id="match-mcp-tools">
  Corrispondere ai tool MCP
</h4>

I tool del server [MCP](/docs/it/mcp) appaiono come tool regolari negli eventi dello strumento (`PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`), quindi puoi farli corrispondere allo stesso modo di qualsiasi altro nome di strumento.

I tool MCP seguono il modello di denominazione `mcp__<server>__<tool>`, ad esempio:

* `mcp__memory__create_entities`: tool create entities del server Memory
* `mcp__filesystem__read_file`: tool read file del server Filesystem
* `mcp__github__search_repositories`: tool search del server GitHub

Per corrispondere a ogni tool da un server, aggiungi `.*` al prefisso del server. `.*` è obbligatorio: un matcher come `mcp__memory` o `mcp__brave-search` contiene solo caratteri di corrispondenza esatta, quindi viene confrontato come una stringa esatta e non corrisponde a nessun tool.

* `mcp__memory__.*` corrisponde a tutti i tool dal server `memory`
* `mcp__brave-search__.*` corrisponde a tutti i tool da un server il cui nome contiene un trattino
* `mcp__.*__write.*` corrisponde a qualsiasi tool il cui nome inizia con `write` da qualsiasi server

I trattini nel set di corrispondenza esatta richiedono Claude Code v2.1.195 o successivo. Nelle versioni precedenti un prefisso nudo con trattino come `mcp__brave-search` viene valutato come un'espressione regolare non ancorata e corrisponde a ogni tool da quel server. La forma `mcp__brave-search__.*` funziona su ogni versione.

I tool da un [server MCP fornito da plugin](/docs/it/mcp#plugin-provided-mcp-servers) utilizzano un segmento di server con scope che include il nome del plugin: `mcp__plugin_<plugin-name>_<server-name>__<tool>`. Un matcher scritto rispetto alla chiave del server nudo non si attiva mai per questi tool. Per un plugin denominato `my-plugin` che raggruppa un server sotto la chiave `db`, un tool `query` appare come `mcp__plugin_my-plugin_db__query`, quindi il matcher per ogni tool da quel server è `mcp__plugin_my-plugin_db__.*`. Utilizza lo stesso nome di tool con scope nel campo [`if`](#common-fields) di un handler. Vedi [Plugin-provided MCP servers](/docs/it/mcp#plugin-provided-mcp-servers) per come viene costruito il nome con scope.

Questo esempio registra tutte le operazioni del server memory e convalida le operazioni di scrittura da qualsiasi server MCP:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "mcp__memory__.*",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'Memory operation initiated' >> ~/mcp-operations.log"
          }
        ]
      },
      {
        "matcher": "mcp__.*__write.*",
        "hooks": [
          {
            "type": "command",
            "command": "/home/user/scripts/validate-mcp-write.py"
          }
        ]
      }
    ]
  }
}
```

<h3 id="hook-handler-fields">
  Campi handler hook
</h3>

Ogni oggetto nell'array `hooks` interno è un handler hook: il comando shell, endpoint HTTP, tool MCP, prompt LLM, o agente che viene eseguito quando il matcher corrisponde. Ci sono cinque tipi:

* **[Command hooks](#command-hook-fields)** (`type: "command"`): esegui un comando shell. Il tuo script riceve l'[input JSON](#hook-input-and-output) dell'evento su stdin e comunica i risultati indietro attraverso codici di uscita e stdout.
* **[HTTP hooks](#http-hook-fields)** (`type: "http"`): invia l'input JSON dell'evento come richiesta HTTP POST a un URL. L'endpoint comunica i risultati indietro attraverso il corpo della risposta utilizzando lo stesso [formato di output JSON](#json-output) degli hook di comando.
* **[MCP tool hooks](#mcp-tool-hook-fields)** (`type: "mcp_tool"`): chiama un tool su un server [MCP](/docs/it/mcp) già connesso. L'output di testo del tool viene trattato come stdout di hook di comando.
* **[Prompt hooks](#prompt-and-agent-hook-fields)** (`type: "prompt"`): invia un prompt a un modello Claude per la valutazione a turno singolo. Il modello restituisce la sua decisione come JSON. Vedi [Prompt-based hooks](#prompt-based-hooks).
* **[Agent hooks](#prompt-and-agent-hook-fields)** (`type: "agent"`): genera un subagent che può utilizzare tool come Read, Grep, e Glob per verificare le condizioni prima di restituire una decisione. Gli agent hook sono sperimentali e potrebbero cambiare. Vedi [Agent-based hooks](#agent-based-hooks).

Tutti gli hook corrispondenti vengono eseguiti in parallelo. Se definisci lo stesso handler in più di un file di impostazioni, viene eseguito una volta. Una copia dello stesso handler di un plugin o skill rimane separata.

Gli handler vengono eseguiti nella directory corrente con l'ambiente di Claude Code. Se la directory corrente non esiste più, ad esempio un worktree o una directory temporanea che un'altra shell ha eliminato a metà sessione, Claude Code esegue gli hook di comando dal primo di questi che esiste ancora: la directory in cui è iniziata la sessione, la radice del progetto, la tua home directory, o la directory temporanea del sistema. Claude Code registra un avviso che nomina la directory di fallback nel [debug log](#debug-hooks).

La variabile di ambiente `$CLAUDE_CODE_REMOTE` è `"true"` negli ambienti web remoti e non è impostata nella CLI locale. Claude Code v2.1.199 e successivo imposta [`$CLAUDE_CODE_BRIDGE_SESSION_ID`](/docs/it/env-vars) all'ID della sessione [Remote Control](/docs/it/remote-control) mentre la sessione locale ha una connessione Remote Control attiva.

<h4 id="common-fields">
  Campi comuni
</h4>

Questi campi si applicano a tutti i tipi di hook:

| Campo           | Obbligatorio | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :-------------- | :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`          | sì           | `"command"`, `"http"`, `"mcp_tool"`, `"prompt"`, o `"agent"`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `if`            | no           | Sintassi della regola di permesso per filtrare quando questo hook viene eseguito, come `"Bash(git *)"` o `"Edit(*.ts)"`. Il comando hook viene eseguito solo se la chiamata dello strumento corrisponde al pattern. Vedi la tabella [Bash matching](#bash-if-matching) sotto per come i pattern Bash vengono valutati rispetto ai sottocomandi, `$()`, e backtick. Valutato solo su eventi dello strumento: `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, e `PermissionDenied`. Su altri eventi, un hook con `if` impostato non viene mai eseguito. Utilizza la stessa sintassi delle [regole di permesso](/docs/it/permissions)                                                                          |
| `timeout`       | no           | Secondi prima di annullare. Claude Code non lo applica su un hook di comando che esegui con [`async: true`](#run-hooks-in-the-background). Impostazioni predefinite: 600 per `command`, `http`, e `mcp_tool`; 30 per `prompt`; 60 per `agent`. Claude Code abbassa l'impostazione predefinita di `command`, `http`, e `mcp_tool` a 30 su [`UserPromptSubmit`](#userpromptsubmit), [`PreModelSwitch`](#premodelswitch), e [`PostModelSwitch`](#postmodelswitch), e a 10 su [`MessageDisplay`](#messagedisplay). Gli hook [`SessionEnd`](#sessionend) condividono un budget di 1,5 secondi; se le tue impostazioni impostano un `timeout` per hook più lungo, Claude Code aumenta il budget per corrispondere, fino a 60 secondi |
| `statusMessage` | no           | Messaggio spinner personalizzato visualizzato mentre l'hook viene eseguito                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `once`          | no           | Se `true`, Claude Code rimuove l'hook dopo la sua prima esecuzione riuscita. Un'esecuzione che fallisce, blocca con codice di uscita 2, o scade lascia l'hook in posizione, quindi viene eseguito di nuovo al prossimo evento corrispondente. Onorato solo per gli hook dichiarati nel [frontmatter dello skill](#hooks-in-skills-and-agents); ignorato nei file di impostazioni e nel frontmatter dell'agente                                                                                                                                                                                                                                                                                                                 |

Il campo `if` contiene esattamente una regola di permesso. Non c'è sintassi `&&`, `||`, o lista per combinare le regole; per applicare più condizioni, definisci un handler hook separato per ciascuna.

In una condizione `if` per uno strumento di file, un pattern di directory a segmento singolo come `"Edit(src/**)"` corrisponde solo alla directory `src` nella directory di lavoro e ai file sotto di essa. Per corrispondere a una directory denominata `src` a qualsiasi profondità, scrivi `"Edit(**/src/**)"`. Prima di v2.1.214, `"Edit(src/**)"` corrispondeva a una directory denominata `src` a qualsiasi profondità sotto la directory di lavoro.

<span id="bash-if-matching" />Per i pattern Bash, se il tuo comando hook viene eseguito dipende dalla forma del pattern e dal comando Bash che Claude sta invocando. Gli assegnamenti `VAR=value` iniziali vengono rimossi prima della corrispondenza.

| Pattern `if`       | Comando Bash                | L'hook viene eseguito? | Perché                                                                                                                                                          |
| :----------------- | :-------------------------- | :--------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Bash(git *)`      | `FOO=bar git push`          | sì                     | gli assegnamenti iniziali vengono rimossi; `git push` corrisponde                                                                                               |
| `Bash(git *)`      | `npm test && git push`      | sì                     | ogni sottocomando viene controllato; `git push` corrisponde                                                                                                     |
| `Bash(rm *)`       | `echo $(rm -rf /)`          | sì                     | i comandi dentro `$()` e backtick vengono controllati; `rm -rf /` corrisponde                                                                                   |
| `Bash(rm *)`       | `echo $(date)`              | no                     | nessun sottocomando corrisponde a `rm *`                                                                                                                        |
| `Bash(cat *)`      | `echo before $(date) after` | no                     | una sostituzione può stare in qualsiasi posizione di argomento, quindi il comando completo e `date` vengono entrambi controllati; nessuno corrisponde a `cat *` |
| `Bash(git *)`      | `$TOOL git push`            | sì                     | Claude Code non può dire a cosa si espande il nome del comando, quindi esegue l'hook                                                                            |
| `Bash(git push *)` | `echo $(date)`              | sì                     | i pattern che specificano più del nome del comando eseguono comunque l'hook su `$()`, backtick, o `$VAR`                                                        |

Quando Claude Code non può determinare quali comandi esegue l'input Bash, esegue il tuo hook indipendentemente dal pattern. Poiché il filtro `if` è best-effort, utilizza il [sistema di permessi](/docs/it/permissions) piuttosto che un hook per applicare un allow o deny rigido.

<h4 id="command-hook-fields">
  Campi command hook
</h4>

Oltre ai [campi comuni](#common-fields), gli hook di comando accettano questi campi:

| Campo         | Obbligatorio | Descrizione                                                                                                                                                                                                                                                                                                                                                                                    |
| :------------ | :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`     | sì           | Comando shell da eseguire. Con `args`, l'eseguibile da generare direttamente. Vedi [Exec form e shell form](#exec-form-and-shell-form)                                                                                                                                                                                                                                                         |
| `args`        | no           | Lista di argomenti. Quando presente, `command` viene risolto come un eseguibile e generato direttamente con `args` come vettore di argomenti, senza shell coinvolto. Vedi [Exec form e shell form](#exec-form-and-shell-form)                                                                                                                                                                  |
| `async`       | no           | Se `true`, viene eseguito in background senza bloccare. Vedi [Run hooks in the background](#run-hooks-in-the-background)                                                                                                                                                                                                                                                                       |
| `asyncRewake` | no           | Se `true`, viene eseguito in background e riattiva Claude al codice di uscita 2. Lo stderr dell'hook, o stdout se stderr è vuoto, viene mostrato a Claude come un promemoria di sistema in modo che possa reagire a un fallimento di background di lunga durata                                                                                                                                |
| `shell`       | no           | Shell da utilizzare per questo hook. Accetta `"bash"` o `"powershell"`. Impostazione predefinita `"bash"`, o `"powershell"` su Windows quando Git Bash non è installato. L'impostazione di `"powershell"` esegue il comando tramite PowerShell su Windows. Non richiede `CLAUDE_CODE_USE_POWERSHELL_TOOL` poiché gli hook generano PowerShell direttamente. Ignorato quando `args` è impostato |

<a id="exec-form-and-shell-form" />

<h5 id="exec-form-and-shell-form">
  Exec form e shell form
</h5>

Un hook di comando viene eseguito come exec form quando `args` è impostato, e shell form quando `args` è omesso. Imposta `args` ogni volta che l'hook fa riferimento a un [placeholder di percorso](#reference-scripts-by-path), poiché ogni elemento viene passato come un argomento senza virgolette. Ometti `args` quando hai bisogno di funzioni shell come pipe o `&&`, o quando nessuno dei due problemi si applica.

**Exec form** viene eseguito quando `args` è presente. Claude Code risolve `command` come un eseguibile su `PATH` e lo genera direttamente con `args` come vettore di argomenti. Non c'è shell, quindi ogni elemento `args` è un argomento esattamente come scritto, e i placeholder di percorso come `${CLAUDE_PLUGIN_ROOT}` vengono sostituiti in `command` e in ogni elemento `args` come stringhe semplici. I caratteri speciali come apostrofi, `$`, e backtick passano attraverso verbatim perché non c'è shell per interpretarli. Non avviene tokenizzazione shell su nessuna piattaforma.

**Shell form** viene eseguito quando `args` è assente. La stringa `command` viene passata a una shell: `sh -c` su macOS e Linux, Git Bash su Windows, o PowerShell quando Git Bash non è installato. Imposta il campo `shell` per scegliere esplicitamente. La shell tokenizza la stringa, espande le variabili, e interpreta pipe, `&&`, reindirizzamenti, e glob.

<Note>
  Su Windows, exec form richiede che `command` si risolva in un vero eseguibile come `.exe`. Gli shim `.cmd` e `.bat` che npm, npx, eslint, e altri tool installano in `node_modules/.bin` non sono eseguibili e non possono essere generati senza una shell. Per eseguirli in exec form, invoca lo script sottostante con `node` direttamente, ad esempio `"command": "node", "args": ["${CLAUDE_PLUGIN_ROOT}/node_modules/eslint/bin/eslint.js"]`. Il pattern `node` più script-path funziona su ogni piattaforma perché `node.exe` è un vero binario. Per eseguire uno shim `.cmd` o `.bat` per nome, utilizza shell form.
</Note>

Questo esempio esegue uno script Node raggruppato con un plugin. Exec form passa il percorso dello script risolto come un argomento senza virgolette:

```json theme={null}
{
  "type": "command",
  "command": "node",
  "args": ["${CLAUDE_PLUGIN_ROOT}/scripts/format.js", "--fix"]
}
```

La shell form equivalente ha bisogno di virgolette per gestire i percorsi con spazi o caratteri speciali:

```json theme={null}
{
  "type": "command",
  "command": "node \"${CLAUDE_PLUGIN_ROOT}\"/scripts/format.js --fix"
}
```

Entrambe le forme supportano gli stessi [placeholder di percorso](#reference-scripts-by-path), ed entrambe li esportano come variabili di ambiente `CLAUDE_PROJECT_DIR`, `CLAUDE_PLUGIN_ROOT`, e `CLAUDE_PLUGIN_DATA` sul processo generato, quindi uno script può leggere `process.env.CLAUDE_PLUGIN_ROOT` indipendentemente da come è stato lanciato.

Gli hook plugin inoltre sostituiscono i valori [`${user_config.*}`](/docs/it/plugins/manifest-reference#user-configuration), solo in exec form: il valore viene sostituito in `command` e in ogni elemento `args` come una stringa semplice, quindi nessuna shell lo ri-analizza.

Un hook plugin in shell form il cui `command` fa riferimento a `${user_config.*}` fallisce con un [errore](/docs/it/errors#plugin-command-references-user-config) invece di essere eseguito. Per utilizzare un valore di opzione da un hook in shell form, leggi la variabile di ambiente `$CLAUDE_PLUGIN_OPTION_<KEY>`, come `$CLAUDE_PLUGIN_OPTION_WEBHOOK_URL` per un'opzione `webhook_url`, o imposta `args` per passare l'hook a exec form. Prima di v2.1.207, i comandi degli hook plugin in shell form sostituivano anche `${user_config.*}`.

<Note>
  In exec form, `command` è solo il nome o il percorso dell'eseguibile. Se `command` è un nome nudo senza separatore di percorso e contiene spazi insieme a `args`, Claude Code registra un avviso perché la generazione fallirà: non c'è un eseguibile denominato `node script.js`. Sposta i token extra in `args`. I percorsi assoluti con spazi, come `C:\Program Files\nodejs\node.exe`, sono un singolo eseguibile valido e non attivano l'avviso.
</Note>

<h4 id="http-hook-fields">
  Campi HTTP hook
</h4>

Oltre ai [campi comuni](#common-fields), gli hook HTTP accettano questi campi:

| Campo            | Obbligatorio | Descrizione                                                                                                                                                                                                                                                     |
| :--------------- | :----------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url`            | sì           | URL a cui inviare la richiesta POST                                                                                                                                                                                                                             |
| `headers`        | no           | Header HTTP aggiuntivi come coppie chiave-valore. I valori supportano l'interpolazione delle variabili di ambiente utilizzando la sintassi `$VAR_NAME` o `${VAR_NAME}`. Solo le variabili elencate in `allowedEnvVars` vengono risolte                          |
| `allowedEnvVars` | no           | Lista di nomi di variabili di ambiente che possono essere interpolate nei valori degli header. I riferimenti alle variabili non elencate vengono sostituiti con stringhe vuote. Obbligatorio affinché avvenga qualsiasi interpolazione di variabili di ambiente |

Claude Code invia l'[input JSON](#hook-input-and-output) dell'hook come corpo della richiesta POST con `Content-Type: application/json`. Il corpo della risposta utilizza lo stesso [formato di output JSON](#json-output) degli hook di comando.

La gestione degli errori differisce dagli hook di comando; vedi [HTTP response handling](#http-response-handling).

Questo esempio invia gli eventi `PreToolUse` a un servizio di convalida locale, autenticandosi con un token dalla variabile di ambiente `MY_TOKEN`:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "http",
            "url": "http://localhost:8080/hooks/pre-tool-use",
            "timeout": 30,
            "headers": {
              "Authorization": "Bearer $MY_TOKEN"
            },
            "allowedEnvVars": ["MY_TOKEN"]
          }
        ]
      }
    ]
  }
}
```

<h4 id="mcp-tool-hook-fields">
  Campi MCP tool hook
</h4>

Oltre ai [campi comuni](#common-fields), gli hook MCP tool accettano questi campi:

| Campo    | Obbligatorio | Descrizione                                                                                                                                                                                                                                                                                                                       |
| :------- | :----------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `server` | sì           | Nome di un server MCP configurato. Per un [server fornito da plugin](/docs/it/mcp#plugin-provided-mcp-servers), questo è il nome con scope `plugin:<plugin-name>:<server-name>`, come `plugin:my-plugin:db`, non la chiave del server nudo. Il server deve essere già connesso; l'hook non attiva mai un flusso OAuth o di connessione |
| `tool`   | sì           | Nome del tool da chiamare su quel server                                                                                                                                                                                                                                                                                          |
| `input`  | no           | Argomenti passati al tool. I valori stringa supportano la sostituzione `${path}` dall'[input JSON](#hook-input-and-output) dell'hook, come `"${tool_input.file_path}"`                                                                                                                                                            |

Claude Code legge il contenuto di testo del tool allo stesso modo in cui legge stdout di hook di comando, seguendo la [regola di analisi sotto il codice di uscita 0](#exit-code-0). Se il server denominato non è connesso, o il tool restituisce `isError: true`, l'hook produce un errore non bloccante e l'esecuzione continua.

Questo esempio chiama il tool `security_scan` sul server MCP `my_server` dopo ogni `Write` o `Edit`, passando il percorso del file modificato:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "mcp_tool",
            "server": "my_server",
            "tool": "security_scan",
            "input": { "file_path": "${tool_input.file_path}" }
          }
        ]
      }
    ]
  }
}
```

Un hook `mcp_tool` può essere eseguito solo dopo che Claude Code ha reso i server MCP della sessione disponibili agli hook. `SessionStart` e `Setup` possono attivarsi prima di quel punto:

* **Al lancio**: `SessionStart` si attiva prima che i server siano disponibili, incluso quando avvii con `--continue` o `--resume`. Claude Code salta gli hook `mcp_tool` dell'evento senza chiamare i loro tool, e il [debug log](#debug-hooks) registra `mcp_tool hooks are not available for the 'SessionStart' hook event (no MCP client context)`.
* **Più tardi in una sessione in esecuzione**: dopo `/clear` o una compattazione, `SessionStart` si attiva di nuovo con i server già disponibili, e i suoi hook `mcp_tool` vengono eseguiti.
* **Su `Setup`**: `Setup` si attiva sempre prima che i server siano disponibili, quindi Claude Code salta i suoi hook `mcp_tool` ogni volta e registra lo stesso messaggio che nomina `Setup`.

Ad esempio, questa configurazione chiama il tool `load_context` sul server MCP `my_server` da un hook `SessionStart` senza matcher, quindi si applica a ogni fonte `SessionStart`:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "mcp_tool",
            "server": "my_server",
            "tool": "load_context"
          }
        ]
      }
    ]
  }
}
```

Quando esegui `claude`, Claude Code salta questo hook, non chiama mai `load_context`, e scrive il messaggio `no MCP client context` nel debug log. Esegui `/clear` in quella stessa sessione e l'hook viene eseguito e chiama `load_context`. Un hook `type: "command"` su `SessionStart` viene eseguito al lancio, quindi usane uno per qualsiasi cosa la sessione abbia bisogno dal suo primo turno.

<h4 id="prompt-and-agent-hook-fields">
  Campi prompt e agent hook
</h4>

Oltre ai [campi comuni](#common-fields), gli hook prompt e agent accettano questi campi:

| Campo    | Obbligatorio | Descrizione                                                                                                                                                                                                        |
| :------- | :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt` | sì           | Testo del prompt da inviare al modello. Utilizza `$ARGUMENTS` come placeholder per l'input JSON dell'hook. Sfuggi con una barra rovesciata per includere testo letterale: `\$1.00` viene renderizzato come `$1.00` |
| `model`  | no           | Modello da utilizzare per la valutazione. Impostazione predefinita a un modello veloce                                                                                                                             |

<h3 id="reference-scripts-by-path">
  Riferisci gli script per percorso
</h3>

Utilizza questi placeholder per fare riferimento agli script degli hook relativi alla radice del progetto o del plugin, indipendentemente dalla directory di lavoro quando l'hook viene eseguito:

* `${CLAUDE_PROJECT_DIR}`: la radice del progetto dove è iniziata la sessione. Claude Code imposta anche questa variabile nell'ambiente dei [server MCP stdio](/docs/it/mcp#option-3-add-a-local-stdio-server) e dei server LSP dei plugin.
* `${CLAUDE_PLUGIN_ROOT}`: la directory di installazione del plugin, per gli script raggruppati con un [plugin](/docs/it/plugins/overview). Vedi [variabili di ambiente del plugin](/docs/it/plugins/manifest-reference#environment-variables) per come il percorso si comporta tra gli aggiornamenti.
* `${CLAUDE_PLUGIN_DATA}`: la [directory di dati persistenti](/docs/it/plugins/components#path-variables-and-persistent-data) del plugin, per le dipendenze e lo stato che dovrebbero sopravvivere agli aggiornamenti del plugin.

<Note>
  **I worktree sono diversi.** Se Claude entra in un [worktree](/docs/it/worktrees) durante la sessione, Claude Code mantiene `${CLAUDE_PROJECT_DIR}` dove era e passa il percorso del worktree ai tuoi hook in un modo diverso:

  * **`${CLAUDE_PROJECT_DIR}` rimane fermo**: punta ancora alla radice del progetto dove è iniziata la sessione, quindi un comando come `${CLAUDE_PROJECT_DIR}/.claude/hooks/check-style.sh` esegue ancora lo script nel checkout principale.
  * **`cwd` segue Claude**: il campo `cwd` nell'[input JSON](#common-input-fields) dell'hook è la radice del worktree dopo che Claude entra in un worktree, e la nuova directory dopo che Claude esegue `cd`. Leggilo quando un hook ha bisogno di sapere quale directory Claude sta utilizzando.
</Note>

Preferisci [exec form](#exec-form-and-shell-form) per qualsiasi hook che faccia riferimento a un placeholder di percorso. In shell form, racchiudi ogni placeholder tra virgolette doppie.

<Tabs>
  <Tab title="Project scripts">
    Questo esempio utilizza `${CLAUDE_PROJECT_DIR}` per eseguire un verificatore di stile dalla directory `.claude/hooks/` del progetto dopo qualsiasi chiamata dello strumento `Write` o `Edit`:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/check-style.sh",
                "args": []
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Plugin scripts">
    Definisci gli hook del plugin in `hooks/hooks.json` con un campo `description` opzionale di livello superiore. Quando un plugin è abilitato, i suoi hook si uniscono ai tuoi hook utente e progetto.

    Questo esempio esegue uno script di formattazione raggruppato con il plugin:

    ```json theme={null}
    {
      "description": "Automatic code formatting",
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PLUGIN_ROOT}/scripts/format.sh",
                "args": [],
                "timeout": 30
              }
            ]
          }
        ]
      }
    }
    ```

    Vedi il [riferimento dei componenti del plugin](/docs/it/plugins/components#hooks) per i dettagli sulla creazione degli hook del plugin.
  </Tab>
</Tabs>

<h3 id="hooks-in-skills-and-agents">
  Hook in skill e agenti
</h3>

Oltre ai file di impostazioni e ai plugin, gli hook possono essere definiti direttamente negli [skill](/docs/it/skills) e nei [subagenti](/docs/it/sub-agents) utilizzando il frontmatter, nello stesso formato di configurazione degli hook basati su impostazioni. Per quanto tempo Claude Code li mantiene registrati dipende dal componente:

* **Hook del subagent**: Claude Code li esegue solo mentre quel subagent è in esecuzione e li rimuove quando finisce. Claude Code converte un hook `Stop` qui in `SubagentStop`, l'evento che si attiva quando un subagent si completa.
* **Hook dello skill**: Claude Code li registra quando tu o Claude invocate lo skill e continua a eseguirli per il resto della sessione, su turni dopo il turno dello skill stesso. Per fare in modo che Claude Code rimuova un hook dopo la sua prima esecuzione riuscita, imposta [`once: true`](#common-fields) su di esso.

Questo skill definisce un hook `PreToolUse` che esegue uno script di convalida della sicurezza prima di ogni comando `Bash`:

```yaml theme={null}
---
name: secure-operations
description: Perform operations with security checks
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/security-check.sh"
---
```

I subagenti utilizzano lo stesso formato nel loro frontmatter YAML.

Gli hook del frontmatter in uno skill del progetto seguono la stessa [regola di fiducia dell'area di lavoro degli hook nei file di impostazioni](#workspace-trust). Claude Code li registra quando tu o Claude invocate lo skill, incluso in un'esecuzione `-p` in una cartella che non hai ancora fidata.

Gli hook del frontmatter in un subagent del progetto vengono eseguiti solo dopo che accetti la [finestra di dialogo di fiducia dell'area di lavoro](/docs/it/permissions#project-allow-rules-and-workspace-trust) per la cartella da cui proviene il file dell'agente. Una sessione `-p` non conta come accettazione. [Cosa viene eseguito prima di fidarti di una cartella](/docs/it/permissions#what-runs-before-you-trust-a-folder) confronta questo con la regola del file di impostazioni, e la pagina dei subagenti elenca [quali ambiti sono esenti](/docs/it/sub-agents#hooks-in-subagent-frontmatter). Prima di v2.1.218, questi hook potevano essere eseguiti da cartelle che non avevi fidata.

<h3 id="the-/hooks-menu">
  Il menu `/hooks`
</h3>

Digita `/hooks` in Claude Code per aprire un browser di sola lettura per i tuoi hook configurati. Il menu mostra ogni evento hook con un conteggio degli hook configurati, ti permette di approfondire i matcher, e mostra i dettagli completi di ogni handler hook. Usalo per verificare la configurazione, controllare da quale file di impostazioni proviene un hook, o ispezionare il comando, il prompt, o l'URL di un hook.

Il menu visualizza tutti e cinque i tipi di hook: `command`, `prompt`, `agent`, `http`, e `mcp_tool`. Ogni hook è etichettato con un prefisso `[type]` e una fonte che indica dove è stato definito:

* `User Settings`: da `~/.claude/settings.json`
* `Project Settings`: da `.claude/settings.json`
* `Local Settings`: da `.claude/settings.local.json`
* `Plugin Hooks`: da `hooks/hooks.json` di un plugin
* `Session Hooks`: registrato in memoria per la sessione corrente

Selezionando un hook si apre una vista dettagliata che mostra il suo evento, matcher, tipo, file di origine, e il comando, prompt, o URL completo. Il menu è di sola lettura: per aggiungere, modificare, o rimuovere gli hook, modifica il JSON di impostazioni direttamente o chiedi a Claude di fare il cambiamento.

<h3 id="disable-or-remove-hooks">
  Disabilita o rimuovi gli hook
</h3>

Per rimuovere un hook, elimina la sua voce dal file JSON di impostazioni.

Per disabilitare temporaneamente tutti gli hook senza rimuoverli, imposta `"disableAllHooks": true` nel tuo file di impostazioni. Claude Code legge il valore rimasto dopo che la [precedenza delle impostazioni](/docs/it/settings#settings-precedence) si applica, quindi un `"disableAllHooks": false` nel `.claude/settings.json` di un progetto sostituisce un `true` nelle tue impostazioni utente. Per disattivare gli hook per un'esecuzione qualunque siano le impostazioni del progetto, passa `--settings '{"disableAllHooks": true}'`, che ha la precedenza sulle impostazioni di progetto e locale. Non c'è modo di disabilitare un singolo hook mantenendolo nella configurazione.

L'impostazione `disableAllHooks` rispetta la gerarchia delle impostazioni gestite. Se un amministratore ha configurato gli hook attraverso le impostazioni di policy gestite, `disableAllHooks` impostato nelle impostazioni utente, progetto, o locale non può disabilitare quegli hook gestiti. Solo `disableAllHooks` impostato a livello di impostazioni gestite può disabilitare gli hook gestiti. Per la portata completa di ogni livello, vedi [`disableAllHooks`](/docs/it/settings-reference#disableallhooks).

Le modifiche dirette agli hook nei file di impostazioni vengono normalmente rilevate automaticamente dal file watcher.

<h2 id="hook-input-and-output">
  Input e output del hook
</h2>

I command hook ricevono dati JSON tramite stdin e comunicano i risultati attraverso codici di uscita, stdout e stderr. Gli HTTP hook ricevono lo stesso JSON come corpo della richiesta POST e comunicano i risultati attraverso il corpo della risposta HTTP. Questa sezione copre i campi e il comportamento comuni a tutti gli eventi. Ogni sezione dell'evento sotto [Hook events](#hook-events) include il suo schema di input specifico e le opzioni di controllo della decisione.

Su macOS e Linux, i command hook vengono eseguiti nella loro propria sessione senza un terminale di controllo. Il processo hook e qualsiasi processo figlio non possono aprire `/dev/tty` o inviare sequenze di escape direttamente all'interfaccia Claude Code. Windows non ha `/dev/tty`.

Per visualizzare un messaggio all'utente su qualsiasi piattaforma, restituire [`systemMessage`](#json-output) nell'output JSON. Alcuni eventi lo scartano o lo consegnano altrove, e ogni [sezione dell'evento](#hook-events) lo specifica. Per attivare una notifica desktop, impostare un titolo della finestra o suonare il campanello, restituire [`terminalSequence`](#emit-terminal-notifications) invece.

<h3 id="common-input-fields">
  Campi di input comuni
</h3>

Gli hook event ricevono questi campi come JSON, oltre ai campi specifici dell'evento documentati in ogni sezione [hook event](#hook-events). Per i command hook, questo JSON arriva tramite stdin. Per gli HTTP hook, arriva come corpo della richiesta POST.

| Campo             | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :---------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `session_id`      | Identificatore della sessione corrente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `prompt_id`       | UUID che identifica il prompt dell'utente attualmente in elaborazione. Corrisponde all'attributo [`prompt.id` sugli eventi OpenTelemetry](/docs/it/monitoring-usage#event-correlation-attributes), quindi è possibile correlare l'output del hook con la telemetria per un singolo prompt. Assente fino al primo input dell'utente. Richiede Claude Code v2.1.196 o successivo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `transcript_path` | Percorso al JSON della conversazione. Il file della trascrizione viene scritto in modo asincrono e potrebbe rimanere indietro rispetto alla conversazione in memoria, quindi potrebbe non includere ancora i messaggi più recenti del turno corrente quando un hook si attiva. Gli hook che necessitano del testo dell'assistente finale del turno corrente dovrebbero utilizzare `last_assistant_message` su [Stop](#stop) e [SubagentStop](#subagentstop) invece di leggere la trascrizione                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `cwd`             | Directory di lavoro corrente quando l'hook viene invocato                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `scratchpad_dir`  | Percorso alla directory scratchpad della sessione, dove Claude mantiene i file di lavoro temporanei. Assente quando la sessione non ha uno scratchpad o la directory temporanea non è disponibile. Richiede Claude Code v2.1.257 o successivo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `permission_mode` | [Modalità di autorizzazione](/docs/it/permissions#permission-modes) corrente: `"default"`, `"plan"`, `"acceptEdits"`, `"auto"`, `"dontAsk"` o `"bypassPermissions"`. La modalità etichettata **Manual** arriva come `"default"`, mai come `"manual"`, quindi gli script che corrispondono a `"default"` continuano a funzionare. Non tutti gli eventi ricevono questo campo. Controllare l'esempio JSON in ogni sezione [hook event](#hook-events)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `effort`          | Oggetto con un campo `level` che contiene il [livello di effort](/docs/it/model-config#adjust-effort-level) in vigore quando l'hook viene eseguito: `"low"`, `"medium"`, `"high"`, `"xhigh"` o `"max"`. Se si imposta un livello che il modello attivo non supporta, `level` segnala il livello che Claude Code ha effettivamente eseguito; [Adjust effort level](/docs/it/model-config#adjust-effort-level) dice come lo sceglie. Ultracode non è un livello distinto e viene segnalato come `"xhigh"`. L'oggetto corrisponde al campo `effort` della [riga di stato](/docs/it/statusline#available-data). Presente per gli eventi che si attivano all'interno di un contesto di utilizzo dello strumento, come `PreToolUse`, `PostToolUse`, `Stop` e `SubagentStop`, quando il modello corrente supporta il parametro effort. Il livello è disponibile anche ai comandi hook e allo strumento Bash come variabile di ambiente `$CLAUDE_EFFORT`. |
| `hook_event_name` | Nome dell'evento che si è attivato                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |

Quando si esegue con `--agent` o all'interno di un subagent, vengono inclusi due campi aggiuntivi:

| Campo        | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                            |
| :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `agent_id`   | Identificatore univoco per il subagent. Presente solo quando l'hook si attiva all'interno di una chiamata di subagent. Utilizzare questo per distinguere le chiamate del hook del subagent dalle chiamate del thread principale.                                                                                                                                                                                                       |
| `agent_type` | Nome dell'agente (ad esempio, `"Explore"` o `"security-reviewer"`). Presente quando la sessione utilizza `--agent` o l'hook si attiva all'interno di un subagent. Per i subagent, il tipo del subagent ha la precedenza sul valore `--agent` della sessione. Consultare [SubagentStart](#subagentstart) per i valori che i subagent personalizzati e plugin segnalano e come scrivere un matcher rispetto a un nome con ambito plugin. |

Solo gli hook [`SessionStart`](#sessionstart) possono ricevere un campo `model`, e Claude Code non lo include sempre. Gli hook [`PreModelSwitch`](#premodelswitch) e [`PostModelSwitch`](#postmodelswitch) ricevono `from_model` e `to_model` invece, quindi utilizzare un hook PostModelSwitch per seguire il modello mentre cambia durante una sessione.

Non esiste una variabile di ambiente `$CLAUDE_MODEL`. L'hook può leggere `$ANTHROPIC_MODEL` se lo imposti nella tua shell, ma quel valore non cambia quando cambi modelli con `/model` durante una sessione.

Un processo hook eredita l'ambiente padre, a parte le variabili dell'esportatore `OTEL_*` che Claude Code [rimuove da ogni sottoprocesso che genera](/docs/it/monitoring-usage#administrator-configuration) e, quando [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/it/env-vars#variables) è impostato su `1`, le variabili che rimuove.

Ad esempio, un hook `PreToolUse` per un comando Bash riceve questo su stdin:

```json theme={null}
{
  "session_id": "abc123",
  "prompt_id": "550e8400-e29b-41d4-a716-446655440000",
  "transcript_path": "/home/user/.claude/projects/.../transcript.jsonl",
  "cwd": "/home/user/my-project",
  "scratchpad_dir": "/tmp/claude-1000/-home-user-my-project/abc123/scratchpad",
  "permission_mode": "default",
  "hook_event_name": "PreToolUse",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test",
    "description": "Run test suite",
    "timeout": 120000,
    "run_in_background": false
  },
  "tool_use_id": "toolu_01ABC123..."
}
```

I campi `tool_name`, `tool_input` e `tool_use_id` sono specifici dell'evento. Ogni sezione [hook event](#hook-events) documenta i campi aggiuntivi per quell'evento.

<h3 id="exit-code-output">
  Output del codice di uscita
</h3>

Il codice di uscita dal comando del hook dice a Claude Code se l'azione deve procedere, essere bloccata o essere ignorata. Il codice di uscita non agisce da solo. Claude Code legge i [campi di output JSON](#json-output) da stdout su ogni codice di uscita, non solo 0, e per gli eventi che utilizzano il modello di decisione standard, un oggetto analizzato che passa la convalida dello schema ha effetto insieme al codice. Il blocco di Exit 2 è l'unico risultato che JSON non può sovrascrivere.

Due tabelle possiedono le eccezioni per evento: [Exit code 2 behavior per event](#exit-code-2-behavior-per-event) dice cosa fanno i codici di uscita per ogni evento, e [Decision control](#decision-control) dice quali campi di decisione ogni evento onora. I campi universali come `systemMessage` funzionano su la maggior parte degli eventi e sono elencati nella tabella [JSON output](#json-output).

<h4 id="exit-code-0">
  Exit code 0
</h4>

Exit 0 significa successo, ed è il codice di uscita previsto quando stampi JSON per il controllo strutturato.

Per la maggior parte degli eventi, Claude Code scrive stdout nel log di debug e non lo mostra nella trascrizione. Le eccezioni sono `UserPromptSubmit`, `UserPromptExpansion`, `SessionStart` e `PostModelSwitch`, dove Claude Code aggiunge stdout in testo semplice come contesto che Claude può vedere e su cui agire.

Se Claude Code legge il tuo stdout come [JSON output](#json-output) o come testo semplice dipende da come inizia e finisce, ignorando gli spazi bianchi circostanti:

* **Inizia con `{` e finisce con `}`**: Claude Code lo analizza come JSON. Quando l'output è due o più righe che si analizzano ciascuna come JSON da sole, e nessuna riga è un oggetto [JSON output](#json-output) che imposta un campo, Claude Code tratta l'intero output come testo semplice. Quando una di quelle righe imposta un campo, l'intero output è un errore di analisi, descritto di seguito.
* **Inizia con `{` ma non finisce con `}`**: Claude Code lo tratta come testo semplice.
* **Inizia con qualsiasi altra cosa**: Claude Code lo tratta come testo semplice, un array JSON o una stringa JSON tra virgolette inclusa.

Per gli eventi che utilizzano il modello di decisione standard, exit 0 con un oggetto analizzato che non supera la convalida dello schema è un errore non bloccante: l'azione procede, e la trascrizione mostra un avviso `<hook name> hook error` con il messaggio di convalida. Lo stesso accade su qualsiasi codice di uscita diverso da 2, mentre [exit 2 blocca ancora](#exit-code-2).

Per gli eventi che utilizzano il modello di decisione standard, quando Claude Code tenta di analizzare il tuo stdout come JSON e non può, segnala un errore non bloccante su ogni codice di uscita diverso da 2. La trascrizione mostra un avviso `<hook name> hook error` con il messaggio di analisi. Sugli eventi che aggiungono stdout in testo semplice come contesto, Claude Code non aggiunge il testo. Prima di v2.1.248, Claude Code trattava quello stdout come testo semplice.

Stderr da un hook che esce 0 va solo nel log di debug, mai nella trascrizione, e Claude non lo vede. Per leggerlo tu stesso, abilita [debug logging](#debug-hooks). Per visualizzare un avviso a Claude da un hook `PostToolUse` o `PostToolUseFailure`, esci 2 invece in modo che [Claude veda stderr](#exit-code-2-behavior-per-event) anche se lo strumento è già stato eseguito.

<h4 id="exit-code-2">
  Exit code 2
</h4>

Exit 2 significa un errore bloccante. Su [eventi che possono bloccare](#exit-code-2-behavior-per-event), exit 2 blocca indipendentemente dal fatto che stampi JSON: anche un `permissionDecision` JSON di `"allow"` non può sovrascriverlo. Claude Code legge comunque qualsiasi [JSON output](#json-output) valido su stdout. Su `Elicitation` e `ElicitationResult`, l'`hookSpecificOutput` di un hook exit-2 viene ignorato.

Il messaggio di blocco è il motivo dalla decisione di blocco del tuo JSON quando ne fa una, e il tuo testo stderr altrimenti. Cosa fa il blocco varia per evento: `PreToolUse` blocca la chiamata dello strumento, `UserPromptSubmit` rifiuta il prompt, e così via. [Exit code 2 behavior per event](#exit-code-2-behavior-per-event) elenca l'effetto per ogni evento, e ogni sezione dell'evento dice dove va il messaggio.

Un hook che esce 2 mentre stampa JSON che non supera la convalida dello schema [JSON output](#json-output) blocca comunque: Claude Code utilizza stderr come motivo di blocco e registra l'errore di convalida nel log di debug. Prima di v2.1.214, Claude Code trattava quella combinazione come un errore non bloccante e l'azione procedeva.

Questo script blocca i comandi `rm` uscendo 2 e lascia ogni altro comando al flusso di autorizzazione normale:

```bash theme={null}
#!/bin/bash
# Legge l'input JSON da stdin, controlla il comando
input=$(cat)
command=$(jq -r '.tool_input.command' <<<"$input")

if [[ "$command" == rm* ]]; then
  echo "Blocked: rm commands are not allowed" >&2
  exit 2  # Errore bloccante: la chiamata dello strumento viene impedita
fi

exit 0  # Nessuna decisione: il flusso di autorizzazione normale si applica
```

<h4 id="other-exit-codes">
  Altri codici di uscita
</h4>

Qualsiasi altro codice di uscita non blocca da solo per la maggior parte degli eventi hook. Cosa accade dipende dal tuo stdout:

* Con un oggetto analizzato che passa la convalida dello schema, per gli eventi che utilizzano il modello di decisione standard, Claude Code ignora il codice di uscita e solo JSON decide il risultato:
  * Ogni campo che l'evento supporta è onorato, inclusi `permissionDecision`, `additionalContext`, `updatedInput` e `systemMessage`, e l'hook non viene segnalato come errore.
  * [Decision control](#decision-control) elenca i campi di decisione per evento; i campi universali come `systemMessage` seguono la tabella [JSON output](#json-output).
* Con un oggetto analizzato che non supera la convalida dello schema, per gli eventi che utilizzano il modello di decisione standard, è lo stesso errore non bloccante di [su exit 0](#exit-code-0): l'azione procede, e l'avviso `<hook name> hook error` porta il messaggio di convalida.
* Con stdout che Claude Code [tenta di analizzare come JSON](#exit-code-0) e non può, Claude Code segnala lo stesso errore non bloccante di exit 0 per gli eventi che utilizzano il modello di decisione standard. L'azione procede, e l'avviso porta il messaggio di analisi.
* Con stdout che Claude Code [tratta come testo semplice](#exit-code-0), o con stdout vuoto, è un errore non bloccante per la maggior parte degli eventi hook: l'azione procede, e la trascrizione mostra un avviso `<hook name> hook error` seguito dalla prima riga di stderr, con il prefisso `Failed with non-blocking status code:`. Per acquisire lo stderr completo, abilita [debug logging](#debug-hooks).

Gli eventi al di fuori del modello di decisione standard mantengono le loro proprie righe nella [tabella per evento](#exit-code-2-behavior-per-event): `WorktreeCreate` non riesce nella creazione su qualsiasi uscita diversa da zero indipendentemente da ciò che dice il tuo JSON, e gli eventi che scartano completamente l'output del hook, come `StopFailure`, ignorano il tuo JSON su ogni codice di uscita, a parte i campi di effetto collaterale come `terminalSequence`, che ancora si attivano.

Un hook che non può avviarsi finisce nello stesso bucket non bloccante. Quando il percorso dello script non esiste o non è eseguibile, la shell esce con un codice come 127 e vedi lo stesso avviso con il messaggio dell'interprete, ad esempio `Failed with non-blocking status code: /bin/sh: /path/to/hook.sh: No such file or directory`. Per la maggior parte degli eventi hook, l'azione procede. Quando configuri un hook di policy, guarda questo avviso alla sua prima esecuzione: un percorso digitato male in `settings.json` lascia il gate silenziosamente disabilitato.

<Warning>
  Per la maggior parte degli eventi hook, il codice di uscita 2 è l'unico codice di uscita che blocca solo attraverso il codice. Senza JSON valido su stdout, Claude Code tratta il codice di uscita 1 come un errore non bloccante e procede con l'azione, anche se 1 è il codice di errore Unix convenzionale. Se il tuo hook è destinato a applicare una policy, usa `exit 2`. Gli eventi worktree differiscono: qualsiasi codice di uscita diverso da zero da `WorktreeCreate` interrompe la creazione del worktree, e qualsiasi codice di uscita diverso da zero da `WorktreeRemove` fa fallire la rimozione del worktree se la directory esiste ancora dopo.
</Warning>

<h4 id="timeouts">
  Timeout
</h4>

A parte un command hook che esegui con [`async: true`](#run-hooks-in-the-background), Claude Code annulla un hook `command`, `http` o `mcp_tool` che raggiunge il suo [`timeout`](#common-fields), scartando l'output del hook, quindi su la maggior parte degli eventi un hook scaduto non rende alcuna decisione.

Su [`PreModelSwitch`](#premodelswitch), un hook annullato al suo timeout blocca il cambio di modello. Su `PreToolUse`, le due famiglie di hook differiscono:

* Un hook `command`, `http` o `mcp_tool` scaduto non blocca la chiamata dello strumento. La chiamata continua attraverso il [flusso di autorizzazione](/docs/it/permissions) normale, quindi non contare su un hook bloccato per agire come gate.
* Un hook di callback [Agent SDK](/docs/it/agent-sdk/hooks) che supera il suo timeout [blocca la chiamata dello strumento](#pretooluse).

<h4 id="exit-code-2-behavior-per-event">
  Exit code 2 behavior per event
</h4>

Exit code 2 è il modo in cui un hook segnala "fermarsi, non farlo". L'effetto dipende dall'evento, perché alcuni eventi rappresentano azioni che possono essere bloccate (come una chiamata dello strumento che non è ancora accaduta) e altri rappresentano cose che sono già accadute o non possono essere prevenute.

| Hook event            | Può bloccare? | Cosa accade su exit 2                                                                                                                                                                                                                                          |
| :-------------------- | :------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`          | Sì            | Blocca la chiamata dello strumento                                                                                                                                                                                                                             |
| `PermissionRequest`   | No            | Il codice di uscita 2 non è onorato per questo evento e il flusso di autorizzazione procede invariato. Nega attraverso l'oggetto [`decision`](#permissionrequest-decision-control) invece                                                                      |
| `UserPromptSubmit`    | Sì            | Blocca l'elaborazione del prompt e cancella il prompt                                                                                                                                                                                                          |
| `UserPromptExpansion` | Sì            | Blocca l'espansione                                                                                                                                                                                                                                            |
| `Stop`                | Sì            | Impedisce a Claude di fermarsi, continua la conversazione                                                                                                                                                                                                      |
| `SubagentStop`        | Sì            | Impedisce al subagent di fermarsi                                                                                                                                                                                                                              |
| `TeammateIdle`        | Sì            | Impedisce al compagno di squadra di andare inattivo, quindi continua a lavorare                                                                                                                                                                                |
| `TaskCreated`         | Sì            | Annulla la creazione dell'attività                                                                                                                                                                                                                             |
| `TaskCompleted`       | Sì            | Impedisce che l'attività sia contrassegnata come completata                                                                                                                                                                                                    |
| `ConfigChange`        | Sì            | Blocca la modifica della configurazione dall'avere effetto (tranne `policy_settings`)                                                                                                                                                                          |
| `StopFailure`         | No            | L'output e il codice di uscita vengono ignorati, tranne `terminalSequence`                                                                                                                                                                                     |
| `PostToolUse`         | No            | Mostra stderr a Claude; lo strumento è già stato eseguito                                                                                                                                                                                                      |
| `PostToolUseFailure`  | No            | Mostra stderr a Claude; lo strumento è già fallito                                                                                                                                                                                                             |
| `PostToolBatch`       | Sì            | Interrompe il loop agentico prima della prossima chiamata del modello                                                                                                                                                                                          |
| `PermissionDenied`    | No            | Il codice di uscita e stderr vengono ignorati perché il rifiuto è già avvenuto. Usa JSON `hookSpecificOutput.retry: true` per dire al modello che può riprovare; Claude Code ignora `retry: true` per [no-verdict denials](#permissiondenied-decision-control) |
| `Notification`        | No            | Il codice di uscita e stderr vengono ignorati                                                                                                                                                                                                                  |
| `SubagentStart`       | No            | Mostra stderr solo all'utente                                                                                                                                                                                                                                  |
| `SessionStart`        | No            | Mostra stderr solo all'utente                                                                                                                                                                                                                                  |
| `Setup`               | No            | Il codice di uscita e stderr vengono ignorati                                                                                                                                                                                                                  |
| `SessionEnd`          | No            | Mostra stderr solo all'utente                                                                                                                                                                                                                                  |
| `CwdChanged`          | No            | Mostra stderr solo all'utente                                                                                                                                                                                                                                  |
| `DirectoryAdded`      | No            | Stderr va nel log di debug; la directory è già aggiunta                                                                                                                                                                                                        |
| `FileChanged`         | No            | Mostra stderr solo all'utente                                                                                                                                                                                                                                  |
| `PreCompact`          | Sì            | Blocca la compattazione                                                                                                                                                                                                                                        |
| `PostCompact`         | No            | Mostra stderr solo all'utente                                                                                                                                                                                                                                  |
| `PreModelSwitch`      | Sì            | Blocca il cambio di modello e mostra stderr all'utente                                                                                                                                                                                                         |
| `PostModelSwitch`     | No            | Mostra stderr solo all'utente; il modello è già cambiato                                                                                                                                                                                                       |
| `Elicitation`         | Sì            | Nega l'elicitazione                                                                                                                                                                                                                                            |
| `ElicitationResult`   | Sì            | Blocca la risposta (l'azione diventa decline)                                                                                                                                                                                                                  |
| `WorktreeCreate`      | Sì            | Qualsiasi codice di uscita diverso da zero causa il fallimento della creazione del worktree                                                                                                                                                                    |
| `WorktreeRemove`      | Sì            | Qualsiasi codice di uscita diverso da zero causa il fallimento della rimozione del worktree se la directory esiste ancora dopo. Consultare [WorktreeRemove](#worktreeremove) per cosa accade alla directory                                                    |
| `InstructionsLoaded`  | No            | Il codice di uscita viene ignorato                                                                                                                                                                                                                             |
| `MessageDisplay`      | No            | Il testo originale viene visualizzato                                                                                                                                                                                                                          |

Per `SessionStart`, `SubagentStart` e `PostModelSwitch`, Claude Code rende lo stderr del codice di uscita 2 nella trascrizione come un avviso `<hook name> hook error`, nello stesso modo in cui rende un [errore non bloccante](#exit-code-output). Claude non lo vede, e la sessione o il subagent procede. Per `SubagentStart`, l'avviso appare nella trascrizione del subagent stesso, non nella conversazione padre.

<h3 id="http-response-handling">
  Gestione della risposta HTTP
</h3>

Gli HTTP hook utilizzano i codici di stato HTTP e i corpi della risposta invece dei codici di uscita e stdout. I risultati di seguito si applicano a la maggior parte degli eventi; un evento con il suo proprio contratto di fallimento nella [tabella per evento](#exit-code-2-behavior-per-event), come `WorktreeCreate`, applica quel contratto a un hook HTTP fallito anche:

* **2xx con corpo vuoto**: successo, equivalente al codice di uscita 0 senza output
* **2xx con corpo di oggetto JSON**: analizzato utilizzando lo stesso schema [JSON output](#json-output) dei command hook. Un corpo che non supera la convalida dello schema è un errore non bloccante
* **2xx con qualsiasi altro corpo, come testo semplice**: errore non bloccante, gestito nello stesso modo di uno stato non-2xx. Claude Code non aggiunge il testo al contesto di Claude
* **Stato non-2xx**: errore non bloccante, l'esecuzione continua
* **Guasto di connessione**: errore non bloccante, l'esecuzione continua
* **Timeout**: l'hook viene annullato, come descritto sotto [Timeouts](#timeouts)

A differenza dei command hook, gli HTTP hook non possono segnalare un errore bloccante solo attraverso i codici di stato. Per bloccare una chiamata dello strumento o negare un'autorizzazione, restituire una risposta 2xx con un corpo JSON contenente i campi di decisione appropriati.

<h3 id="json-output">
  Output JSON
</h3>

I codici di uscita ti permettono solo di bloccare o stare in silenzio, ma l'output JSON ti dà un controllo più granulare. Invece di uscire con il codice 2 per bloccare, esci 0 e stampa un oggetto JSON su stdout. Claude Code legge campi specifici da quel JSON per controllare il comportamento, incluso [decision control](#decision-control) per bloccare, consentire o escalare all'utente.

<Note>
  Scegli un approccio per hook: usa i codici di uscita da soli per la segnalazione, o esci 0 e stampa JSON per il controllo strutturato. Se li mescoli, exit 2 mantiene il suo [effetto di blocco](#exit-code-2-behavior-per-event), e Claude Code legge comunque i campi JSON, con l'eccezione di elicitazione notata sotto [Exit code 2](#exit-code-2).
</Note>

Lo stdout del tuo hook deve contenere solo l'oggetto JSON. Se il tuo profilo shell stampa testo all'avvio, può interferire con l'analisi JSON. Consultare [Hook JSON has no effect](/docs/it/hooks-guide#hook-json-has-no-effect) nella guida alla risoluzione dei problemi.

Le stringhe di `additionalContext`, `systemMessage` e `initialUserMessage` del tuo hook, e il suo stdout semplice, sono limitate a 10.000 caratteri:

* **Ambito**: Claude Code misura ogni stringa da sola, anche quando più hook vengono eseguiti per lo stesso evento. Per l'output JSON, ogni campo viene misurato separatamente; lo stdout semplice viene misurato nel complesso.
* **Oltre il limite**: Claude Code salva l'output in un file nella directory della sessione e lo sostituisce con il percorso del file e un'anteprima di fino ai primi 2.000 caratteri. Un grande risultato Bash valido viene gestito nello stesso modo, descritto sotto [Output limits](/docs/it/tools-reference#output-limits). A differenza di quel limite Bash, questo limite non ha un'impostazione o una variabile di ambiente per aumentarlo.
* **Lettura del file**: Claude Code non chiede a Claude di leggere il file, quindi mantieni tutto ciò che Claude deve sempre vedere entro il limite.

L'oggetto JSON supporta tre tipi di campi:

* **Campi universali** come `continue` sono elencati nella tabella di seguito. Ogni evento li accetta, ma alcuni eventi li scartano o consegnano `systemMessage` da qualche parte diversa dalla trascrizione. Ogni sezione dell'evento lo specifica. `terminalSequence` funziona su quegli eventi anche, con le eccezioni elencate sotto [Emit terminal notifications](#emit-terminal-notifications).
* **`decision` e `reason` di livello superiore** sono utilizzati da alcuni eventi per bloccare o fornire feedback.
* **`hookSpecificOutput`** è un oggetto annidato per gli eventi che necessitano di un controllo più ricco. Richiede un campo `hookEventName` impostato sul nome dell'evento.

| Campo              | Predefinito | Descrizione                                                                                                                                                                                                                                                                                                                                                                                 |
| :----------------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `continue`         | `true`      | Se `false`, Claude interrompe completamente l'elaborazione dopo l'esecuzione del hook. Ha la precedenza su qualsiasi campo di decisione specifico dell'evento                                                                                                                                                                                                                               |
| `stopReason`       | nessuno     | Messaggio mostrato all'utente quando `continue` è `false`. Rimane nella conversazione, quindi Claude lo vede se la conversazione continua                                                                                                                                                                                                                                                   |
| `suppressOutput`   | `false`     | Non ha effetto: Claude Code accetta il campo ma non agisce su di esso. Lo stdout di un hook riuscito non viene mai mostrato nella trascrizione e viene registrato nel log di debug                                                                                                                                                                                                          |
| `systemMessage`    | nessuno     | Messaggio di avviso mostrato all'utente. In [Agent SDK](/docs/it/agent-sdk/overview) e output [`--output-format stream-json`](/docs/it/headless), può arrivare come [`SDKInformationalMessage`](/docs/it/agent-sdk/typescript#sdkinformationalmessage)                                                                                                                                                     |
| `terminalSequence` | nessuno     | Una sequenza di escape del terminale per Claude Code da emettere per conto vostro, come una notifica desktop, un titolo della finestra o un campanello. Limitato a OSC `0`/`1`/`2`/`9`/`99`/`777` e BEL. Se il valore contiene qualcosa al di fuori della lista di autorizzazione, il campo viene ignorato. Usa questo invece di scrivere su `/dev/tty`, che non è disponibile per gli hook |

Per fermare Claude completamente:

```json theme={null}
{ "continue": false, "stopReason": "Build failed, fix errors before continuing" }
```

Per gli hook `PreToolUse` e `PostToolUse`, l'arresto si applica anche quando la chiamata dello strumento fallisce o si completa mentre Claude sta ancora trasmettendo una risposta.

<h4 id="emit-terminal-notifications">
  Emettere notifiche del terminale
</h4>

Gli hook vengono eseguiti senza un terminale di controllo, quindi scrivere sequenze di escape direttamente su `/dev/tty` non riesce. Invece, restituire la sequenza di escape nel campo `terminalSequence` e Claude Code la emetterà per voi attraverso il suo percorso di scrittura del terminale. Questo è privo di race condition, funziona all'interno di tmux e GNU screen, e funziona su Windows dove non esiste `/dev/tty`.

Il campo accetta una stringa di una o più sequenze di escape nella lista di autorizzazione:

* OSC `0`, `1`, `2`: titoli della finestra e dell'icona
* OSC `9`: notifiche iTerm2, ConEmu, Windows Terminal e WezTerm, incluso `9;4` progresso della barra delle applicazioni
* OSC `99`: notifiche Kitty
* OSC `777`: notifiche urxvt, Ghostty e Warp
* BEL nudo

Le sequenze possono essere terminate con BEL o con ST. Qualsiasi cosa al di fuori della lista di autorizzazione, incluse le sequenze CSI del cursore e del colore, le sequenze della tavolozza OSC, i collegamenti ipertestuali OSC 8, le scritture degli appunti OSC 52 e OSC 1337, viene rifiutata e il campo viene ignorato.

Claude Code scrive la sequenza stessa quando elabora l'output del tuo hook, quindi il campo funziona su eventi che scartano `systemMessage` e `continue`, come `Notification` e `StopFailure`. Ha due limiti:

* Claude Code scrive la sequenza solo in una sessione interattiva, e solo mentre la sua interfaccia è sullo schermo. In modalità non interattiva con il flag `-p` e in Agent SDK, ignora il campo.
* Un hook `WorktreeCreate` command non può restituire JSON, perché Claude Code legge il suo stdout come il percorso del worktree. Un hook HTTP `WorktreeCreate` restituisce JSON e può includere il campo.

L'esempio di seguito attiva una notifica desktop da un hook `Notification`. La sequenza di escape viene costruita con escape ottali `printf` in modo che i byte di controllo non compaiano mai sulla riga di comando della shell, e `jq -n --arg` costruisce l'output JSON in modo che le virgolette, le barre rovesciate e le nuove righe nel messaggio di notifica siano correttamente sfuggite:

```bash theme={null}
#!/bin/bash
# Hook di notifica: avvisa il desktop quando Claude Code ha bisogno di attenzione.
input=$(cat)
title="Claude Code"
body=$(jq -r '.message // "Needs your attention"' <<<"$input")
seq=$(printf '\033]777;notify;%s;%s\007' "$title" "$body")
jq -nc --arg seq "$seq" '{terminalSequence: $seq}'
```

La forma `{ "terminalSequence": "..." }` è la stessa da qualsiasi shell o linguaggio.

<h4 id="add-context-for-claude">
  Aggiungere contesto per Claude
</h4>

Il campo `additionalContext` passa una stringa dal tuo hook nel contesto della finestra di Claude. Claude Code avvolge la stringa in un promemoria di sistema e la inserisce nella conversazione nel punto in cui l'hook si è attivato. Claude legge il promemoria nella prossima richiesta del modello, ma non appare come messaggio di chat nell'interfaccia.

Restituire `additionalContext` all'interno di `hookSpecificOutput` insieme al nome dell'evento:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "This file is generated. Edit src/schema.ts and run `bun generate` instead."
  }
}
```

Il punto in cui appare il promemoria dipende dall'evento:

* [SessionStart](#sessionstart) e [SubagentStart](#subagentstart): all'inizio della conversazione, prima del primo prompt
* [UserPromptSubmit](#userpromptsubmit) e [UserPromptExpansion](#userpromptexpansion): insieme al prompt inviato
* [PreToolUse](#pretooluse), [PostToolUse](#posttooluse), [PostToolUseFailure](#posttoolusefailure) e [PostToolBatch](#posttoolbatch): accanto al risultato dello strumento
* [Stop](#stop) e [SubagentStop](#subagentstop): alla fine del turno. La conversazione continua in modo che Claude possa agire sul feedback. Consultare [Stop decision control](#stop-decision-control)
* [PostModelSwitch](#postmodelswitch): con la prossima richiesta dopo il cambio. Consultare [PostModelSwitch decision control](#postmodelswitch-decision-control) per i tempi

Quando più hook restituiscono `additionalContext` per lo stesso evento, Claude riceve tutti i valori.

Se un valore supera 10.000 caratteri, Claude Code scrive il testo in un file nella directory della sessione e passa a Claude il percorso del file con un'anteprima di fino ai primi 2.000 caratteri invece. Claude può leggere il file, ma Claude Code non lo chiede.

Usa `additionalContext` per informazioni che Claude dovrebbe conoscere sullo stato corrente del tuo ambiente o sull'operazione appena eseguita:

* **Stato dell'ambiente**: il ramo corrente, la destinazione di distribuzione o i flag di funzionalità attivi
* **Regole di progetto condizionali**: quale comando di test si applica al file appena modificato, quali directory sono di sola lettura in questo worktree
* **Dati esterni**: problemi aperti assegnati a voi, risultati CI recenti, contenuto recuperato da un servizio interno

Per le istruzioni che non cambiano mai, preferire [CLAUDE.md](/docs/it/memory). Si carica senza eseguire uno script ed è il luogo standard per le convenzioni di progetto statiche.

Scrivi il testo come affermazioni fattuali piuttosto che istruzioni di sistema imperative. Frasi come "La destinazione di distribuzione è produzione" o "Questo repository utilizza `bun test`" si leggono come informazioni di progetto. Il testo inquadrato come comandi di sistema fuori banda può attivare le difese di iniezione di prompt di Claude, il che causa a Claude di far emergere il testo a voi invece di trattarlo come contesto.

Claude Code salva il testo iniettato nella trascrizione della sessione. Per gli eventi a metà sessione come `PostToolUse` o `UserPromptSubmit`, quando riprendi con `--continue` o `--resume`, Claude Code riproduce il testo salvato piuttosto che rieseguire l'hook per i turni passati, quindi i valori come timestamp o SHA di commit diventano obsoleti. Gli hook `SessionStart` vengono eseguiti di nuovo al ripristino con `source` impostato su `"resume"`, o `"fork"` se hai aggiunto `--fork-session`, quindi possono aggiornare il loro contesto.

<h4 id="decision-control">
  Controllo della decisione
</h4>

Non ogni evento supporta il blocco o il controllo del comportamento attraverso JSON. Gli eventi che lo fanno utilizzano ciascuno un insieme diverso di campi per esprimere quella decisione. Usa questa tabella come riferimento rapido prima di scrivere un hook:

| Eventi                                                                                                                              | Modello di decisione                                   | Campi chiave                                                                                                                                                                                                                                                                                                                         |
| :---------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| UserPromptSubmit, UserPromptExpansion, PostToolUse, PostToolUseFailure, PostToolBatch, Stop, SubagentStop, ConfigChange, PreCompact | `decision` di livello superiore                        | `decision: "block"`, `reason`. Stop e SubagentStop accettano anche `hookSpecificOutput.additionalContext` per [feedback non-errore che continua la conversazione](#stop-decision-control)                                                                                                                                            |
| TeammateIdle, TaskCompleted                                                                                                         | Codice di uscita o `continue: false`                   | Il codice di uscita 2 blocca l'azione con feedback stderr. JSON `{"continue": false, "stopReason": "..."}` interrompe anche completamente il compagno di squadra, corrispondendo al comportamento dell'hook `Stop`; [TaskCompleted lo ignora quando lo strumento `TaskUpdate` ha attivato l'evento](#taskcompleted-decision-control) |
| TaskCreated                                                                                                                         | Codice di uscita o `decision` di livello superiore     | Il codice di uscita 2 o `decision: "block"` [annulla l'attività](#taskcreated-decision-control) e restituisce il messaggio a Claude. `continue: false` viene ignorato                                                                                                                                                                |
| PreToolUse                                                                                                                          | `hookSpecificOutput`                                   | `permissionDecision` (allow/deny/ask/defer), `permissionDecisionReason`                                                                                                                                                                                                                                                              |
| PreModelSwitch                                                                                                                      | `hookSpecificOutput` o `decision` di livello superiore | `permissionDecision` (allow/deny/ask), `permissionDecisionReason`. `decision: "block"` [annulla anche il cambio](#premodelswitch-decision-control)                                                                                                                                                                                   |
| PermissionRequest                                                                                                                   | `hookSpecificOutput`                                   | `decision.behavior` (allow/deny)                                                                                                                                                                                                                                                                                                     |
| PermissionDenied                                                                                                                    | `hookSpecificOutput`                                   | `retry: true` dice al modello che può riprovare la chiamata dello strumento negata; Claude Code lo ignora per [no-verdict denials](#permissiondenied-decision-control)                                                                                                                                                               |
| WorktreeCreate                                                                                                                      | percorso return                                        | Il command hook stampa il percorso su stdout; l'HTTP hook restituisce `hookSpecificOutput.worktreePath`. Il fallimento del hook o il percorso mancante non riesce nella creazione                                                                                                                                                    |
| WorktreeRemove                                                                                                                      | Codice di uscita                                       | Qualsiasi codice di uscita diverso da zero fa fallire la rimozione se la directory esiste ancora dopo. L'output JSON viene scartato                                                                                                                                                                                                  |
| Elicitation                                                                                                                         | `hookSpecificOutput`                                   | `action` (accept/decline/cancel), `content` (valori dei campi del modulo per accept)                                                                                                                                                                                                                                                 |
| ElicitationResult                                                                                                                   | `hookSpecificOutput`                                   | `action` (accept/decline/cancel), `content` (valori dei campi del modulo override)                                                                                                                                                                                                                                                   |
| MessageDisplay                                                                                                                      | `hookSpecificOutput`                                   | `displayContent` sostituisce il testo visualizzato sullo schermo. Solo visualizzazione: la trascrizione e ciò che Claude vede mantengono l'originale                                                                                                                                                                                 |
| SessionStart, SubagentStart, PostModelSwitch                                                                                        | Solo contesto                                          | `hookSpecificOutput.additionalContext` aggiunge contesto per Claude. SessionStart accetta anche [`initialUserMessage`, `watchPaths`, `sessionTitle` e `reloadSkills`](#sessionstart-decision-control). Nessun blocco o controllo della decisione                                                                                     |
| Setup, Notification, SessionEnd, PostCompact, InstructionsLoaded, StopFailure, CwdChanged, DirectoryAdded, FileChanged              | Nessuno                                                | Nessun controllo della decisione. Utilizzato per effetti collaterali come la registrazione o la pulizia                                                                                                                                                                                                                              |

Alcuni eventi possono anche riscrivere il contenuto piuttosto che solo consentire o bloccare:

* `PreToolUse`: `updatedInput` direttamente sotto `hookSpecificOutput` sostituisce gli argomenti di uno strumento prima che venga eseguito. Consultare [PreToolUse decision control](#pretooluse-decision-control)
* `PermissionRequest`: `updatedInput` all'interno dell'oggetto `decision`. Consultare [PermissionRequest decision control](#permissionrequest-decision-control)
* `PostToolUse`: `updatedToolOutput` sostituisce il risultato dello strumento. Consultare [PostToolUse decision control](#posttooluse-decision-control)
* `UserPromptSubmit`: non può sostituire il prompt; solo inietta `additionalContext` insieme ad esso

Per i casi di uso di redazione o trasformazione, intercettare a `PreToolUse` per gli input dello strumento in uscita e `PostToolUse` per i risultati dello strumento in entrata.

Ecco esempi di ogni modello in azione:

<Tabs>
  <Tab title="Decisione di livello superiore">
    L'unico valore per `decision` è `"block"`. Per consentire all'azione di procedere, omettere `decision` dal JSON, o uscire 0 senza alcun JSON:

    ```json theme={null}
    {
      "decision": "block",
      "reason": "Test suite must pass before proceeding"
    }
    ```
  </Tab>

  <Tab title="PreToolUse">
    Utilizza `hookSpecificOutput` per un controllo più ricco: consentire, negare, chiedere o rinviare all'utente. Puoi anche modificare l'input dello strumento prima che venga eseguito o iniettare contesto aggiuntivo per Claude. Consultare [PreToolUse decision control](#pretooluse-decision-control) per l'insieme completo di opzioni.

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PreToolUse",
        "permissionDecision": "deny",
        "permissionDecisionReason": "Database writes are not allowed"
      }
    }
    ```
  </Tab>

  <Tab title="PermissionRequest">
    Utilizza `hookSpecificOutput` per consentire o negare una richiesta di autorizzazione per conto dell'utente. Quando consenti, puoi anche modificare l'input dello strumento o applicare regole di autorizzazione in modo che l'utente non venga richiesto di nuovo. Consultare [PermissionRequest decision control](#permissionrequest-decision-control) per l'insieme completo di opzioni.

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PermissionRequest",
        "decision": {
          "behavior": "allow",
          "updatedInput": {
            "command": "npm run lint"
          }
        }
      }
    }
    ```
  </Tab>
</Tabs>

Per esempi estesi inclusa la convalida dei comandi Bash, il filtraggio dei prompt e gli script di approvazione automatica, consultare [What you can automate](/docs/it/hooks-guide#what-you-can-automate) nella guida e l'[implementazione di riferimento del validatore di comandi Bash](https://github.com/anthropics/claude-code/blob/main/examples/hooks/bash_command_validator_example.py).

<h2 id="hook-events">
  Hook events
</h2>

Ogni evento corrisponde a un punto nel ciclo di vita di Claude Code dove gli hook possono essere eseguiti. Le sezioni seguenti sono ordinate per corrispondere al ciclo di vita: dalla configurazione della sessione attraverso il loop agentico fino alla fine della sessione. Ogni sezione descrive quando l'evento si attiva, quali matcher supporta, quale input JSON riceve e come controllare il comportamento attraverso l'output.

<h3 id="sessionstart">
  SessionStart
</h3>

Si esegue quando Claude Code avvia una nuova sessione o riprende una sessione esistente. Utile per caricare il contesto di sviluppo come problemi esistenti o modifiche recenti al tuo codebase, o per configurare variabili di ambiente. Per il contesto statico che non richiede uno script, usa [CLAUDE.md](/docs/it/memory) invece.

SessionStart si esegue su ogni sessione, quindi mantieni questi hook veloci. Solo gli hook `type: "command"` e `type: "mcp_tool"` sono supportati. Vedi [MCP tool hook fields](#mcp-tool-hook-fields) per quando gli hook `mcp_tool` si eseguono.

Il valore del matcher corrisponde a come la sessione è stata avviata:

| Matcher   | Quando si attiva                                                                                                                                 |
| :-------- | :----------------------------------------------------------------------------------------------------------------------------------------------- |
| `startup` | Nuova sessione                                                                                                                                   |
| `resume`  | `--resume`, `--continue`, o `/resume`                                                                                                            |
| `clear`   | `/clear`                                                                                                                                         |
| `compact` | Compattazione automatica o manuale                                                                                                               |
| `fork`    | Una nuova sessione creata da una sessione esistente: `--fork-session` con `--resume` o `--continue`, la copia di background `/fork`, o `/branch` |

Prima della v2.1.214, le sessioni create da fork segnalano la sorgente `"resume"`.

Quando esegui un'esecuzione interattiva, riprendi una conversazione al lancio con `--continue` o `--resume`, o esegui `/clear`, gli hook SessionStart si eseguono in background. Puoi digitare subito, e una conversazione che hai ripreso appare senza aspettare gli hook. La prima risposta di Claude attende comunque che gli hook finiscano, quindi il loro contesto raggiunge Claude.

Quando passi a conversazioni con `/resume` all'interno di una sessione, l'attesa del cambio per gli hook finiscano. Se esegui `/clear` o passi a un'altra conversazione mentre gli hook di background sono ancora in esecuzione, nulla di ciò che restituiscono si applica alla sessione.

Lo stesso attesa si applica al lancio, inclusa una sessione ripresa: un prompt che invii mentre gli hook SessionStart sono ancora in esecuzione non raggiunge Claude finché non finiscono.

Durante l'attesa, premi `Esc` per riprendere il prompt nell'input senza inviarlo. Gli hook continuano a essere eseguiti.

<h4 id="sessionstart-input">
  SessionStart input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook SessionStart ricevono `source` e opzionalmente `model`, `agent_type`, e `session_title`:

| Field           | Description                                                                                                                                                                                                                                      |
| :-------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `source`        | Come la sessione è stata avviata: `"startup"` per nuove sessioni, `"resume"` per sessioni riprese, `"clear"` dopo `/clear`, `"compact"` dopo la compattazione, o `"fork"` per una nuova sessione creata da una sessione esistente                |
| `model`         | L'identificatore del modello attivo. Può essere omesso, ad esempio dopo `/clear` o quando una sessione viene ripristinata tramite il recupero della conversazione, quindi controlla il campo prima di leggerlo                                   |
| `agent_type`    | Il nome dell'agente, presente quando avvii Claude Code con `claude --agent <name>`                                                                                                                                                               |
| `session_title` | Il titolo della sessione corrente se già impostato, ad esempio tramite `--name` o `/rename`. Un hook che emette `sessionTitle` può controllare `session_title` prima per evitare di sovrascrivere un titolo impostato esplicitamente dall'utente |

Quando `source` è `"resume"` o `"fork"` e la trascrizione contiene almeno una risposta da Claude, gli hook SessionStart ricevono anche i quattro campi seguenti. Il tuo hook può usarli per segnalare quale costo ha riprendere una conversazione obsoleta prima della prima richiesta, ad esempio in un [`systemMessage`](#json-output). Questi campi richiedono Claude Code v2.1.251 o successivo.

| Field                         | Description                                                                                                                                                                                                         |
| :---------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `seconds_since_last_response` | Secondi di tempo reale dal momento dell'ultima risposta nella trascrizione ripresa                                                                                                                                  |
| `context_tokens`              | Token che la prima richiesta della sessione ripresa invia di nuovo come suo prompt                                                                                                                                  |
| `prompt_cache_likely_expired` | `true` quando l'ultima risposta è più vecchia della [prompt cache lifetime](/docs/it/prompt-caching#cache-lifetime) della sessione o una compattazione successiva ha sostituito la conversazione memorizzata nella cache |
| `estimated_cache_write_usd`   | Costo stimato in dollari USA della scrittura di `context_tokens` nella prompt cache sul modello della sessione, escludendo la risposta                                                                              |

Questo esempio mostra l'input per una sessione ripresa 90 minuti dopo la sua ultima risposta:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionStart",
  "source": "resume",
  "model": "claude-opus-5",
  "seconds_since_last_response": 5400,
  "context_tokens": 182340,
  "prompt_cache_likely_expired": true,
  "estimated_cache_write_usd": 1.1396
}
```

<h4 id="sessionstart-decision-control">
  SessionStart decision control
</h4>

Claude Code aggiunge stdout che [tratta come testo semplice](#exit-code-0) al contesto di Claude. Oltre ai [JSON output fields](#json-output) disponibili per tutti gli hook, puoi restituire questi campi specifici dell'evento:

| Field                | Description                                                                                                                                                                                                                                                                                                                                                |
| :------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext`  | Stringa aggiunta al contesto di Claude all'inizio della conversazione, prima del primo prompt. Vedi [Add context for Claude](#add-context-for-claude) per come il testo viene consegnato e cosa metterci                                                                                                                                                   |
| `initialUserMessage` | Stringa usata come primo messaggio utente della sessione. Si applica in [non-interactive mode](/docs/it/headless) con il flag `-p`, dove diventa il primo turno anche se non viene fornito alcun prompt. Se viene fornito un prompt, segue come turno successivo. A differenza di `additionalContext`, che si allega a un turno esistente, questo crea il turno |
| `sessionTitle`       | Imposta il titolo della sessione, con lo stesso effetto di `/rename`. Usa per nominare le sessioni automaticamente dalla cartella di lancio, dal ramo git, o dal nome del worktree. Si applica quando `source` è `"startup"`, `"resume"`, o `"fork"`; ignorato su `"clear"` e `"compact"`                                                                  |
| `watchPaths`         | Array di percorsi assoluti da osservare per gli eventi [FileChanged](#filechanged) durante questa sessione                                                                                                                                                                                                                                                 |
| `reloadSkills`       | Booleano. Quando `true`, Claude Code esegue nuovamente la scansione delle directory [skill](/docs/it/skills) e command dopo che gli hook SessionStart completano, quindi le skill che l'hook ha installato sono disponibili nella stessa sessione, a partire dal primo prompt                                                                                   |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "SessionStart",
    "additionalContext": "Current branch: feat/auth-refactor\nUncommitted changes: src/auth.ts, src/login.tsx\nActive issue: #4211 Migrate to OAuth2",
    "sessionTitle": "auth-refactor"
  }
}
```

Poiché lo stdout semplice raggiunge già Claude per questo evento, un hook che carica solo il contesto può stampare su stdout direttamente senza costruire JSON. Usa il modulo JSON quando hai bisogno di combinare il contesto con altri campi come `sessionTitle`.

Usa `reloadSkills` quando un hook SessionStart installa o aggiorna skill. La scoperta delle skill normalmente si esegue prima che gli hook SessionStart finiscano, quindi i file che l'hook scrive in `~/.claude/skills/` o `.claude/skills/` altrimenti apparirebbero solo nella sessione successiva. Questo esempio sincronizza un repository di skill condiviso e richiede la nuova scansione:

```bash theme={null}
#!/bin/bash

git -C ~/.claude/skills/team-skills pull --quiet 2>/dev/null || \
  git clone --quiet https://git.example.com/your-org/team-skills.git ~/.claude/skills/team-skills

echo '{"hookSpecificOutput": {"hookEventName": "SessionStart", "reloadSkills": true}}'
```

L'URL del repository è un segnaposto; sostituiscilo con il tuo repository di skill. Con il segnaposto, il clone fallisce e stampa un messaggio `fatal:` su stderr. Stderr da un hook SessionStart che esce con 0 è solo informativo, quindi la richiesta `reloadSkills` si applica comunque.

<h4 id="persist-environment-variables">
  Persist environment variables
</h4>

Gli hook SessionStart hanno accesso alla variabile di ambiente `CLAUDE_ENV_FILE`, che fornisce un percorso di file dove puoi persistere le variabili di ambiente per i comandi Bash successivi.

Per impostare singole variabili di ambiente, scrivi istruzioni `export` in `CLAUDE_ENV_FILE`. Usa append (`>>`) per preservare le variabili impostate da altri hook:

```bash theme={null}
#!/bin/bash

if [ -n "$CLAUDE_ENV_FILE" ]; then
  echo 'export NODE_ENV=production' >> "$CLAUDE_ENV_FILE"
  echo 'export DEBUG_LOG=true' >> "$CLAUDE_ENV_FILE"
  echo 'export PATH="$PATH:./node_modules/.bin"' >> "$CLAUDE_ENV_FILE"
fi

exit 0
```

Per catturare tutti i cambiamenti di ambiente dai comandi di configurazione, confronta le variabili esportate prima e dopo:

```bash theme={null}
#!/bin/bash

ENV_BEFORE=$(export -p | sort)

# Run your setup commands that modify the environment
source ~/.nvm/nvm.sh
nvm use 20

if [ -n "$CLAUDE_ENV_FILE" ]; then
  ENV_AFTER=$(export -p | sort)
  comm -13 <(echo "$ENV_BEFORE") <(echo "$ENV_AFTER") >> "$CLAUDE_ENV_FILE"
fi

exit 0
```

<Note>
  `CLAUDE_ENV_FILE` è disponibile per gli hook SessionStart, [Setup](#setup), [CwdChanged](#cwdchanged), e [FileChanged](#filechanged). Gli altri tipi di hook non hanno accesso a questa variabile.
</Note>

<h3 id="setup">
  Setup
</h3>

Si attiva solo quando avvii Claude Code con `--init-only`, o con `--init` o `--maintenance` in [non-interactive mode](/docs/it/headless) con il flag `-p`. Non si attiva all'avvio normale. Usalo per l'installazione di dipendenze una tantum o la pulizia programmata che attivi esplicitamente da CI o script, separato dall'avvio della sessione normale. Per l'inizializzazione per sessione, usa [SessionStart](#sessionstart) invece.

Il valore del matcher corrisponde al flag CLI che ha attivato l'hook:

| Matcher       | Quando si attiva                          |
| :------------ | :---------------------------------------- |
| `init`        | `claude --init-only` o `claude -p --init` |
| `maintenance` | `claude -p --maintenance`                 |

Quando esegui `claude --init-only`, Claude Code esegue gli hook Setup e gli hook `SessionStart` con il matcher `startup`, quindi esce senza avviare una conversazione.

Quando avvii o continui una conversazione con `-p`, devi anche fornire un prompt, come argomento o tramite pipe su stdin. Puoi saltare il prompt quando un hook `SessionStart` fornisce [`initialUserMessage`](#sessionstart-decision-control) o quando riprendi una sessione con una [deferred tool call](#defer-a-tool-call-for-later).

Al successo, `--init-only` non stampa nulla nel terminale. Per confermare che gli hook si sono eseguiti, inizia con `claude --debug-file <path> --init-only`, sostituendo `<path>` con una posizione di file di log, e controlla il log per le voci degli hook Setup e SessionStart.

Poiché Setup non si attiva ad ogni lancio, un plugin che ha bisogno di una dipendenza installata non può fare affidamento solo su Setup. Il modello pratico è controllare la dipendenza al primo utilizzo e installare se assente, ad esempio un hook o una skill che testa per `${CLAUDE_PLUGIN_DATA}/node_modules` ed esegue `npm install` se assente. Vedi la [persistent data directory](/docs/it/plugins/components#path-variables-and-persistent-data) per dove archiviare le dipendenze installate. Se distribuisci il tuo plugin tramite un marketplace, potresti non aver bisogno di questo modello: Claude Code [installa automaticamente le dipendenze dei pacchetti Node.js idonei](/docs/it/plugins/loading#node-js-package-dependencies) quando memorizza il plugin nella cache.

<h4 id="setup-input">
  Setup input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook Setup ricevono un campo `trigger` impostato su `"init"` o `"maintenance"`:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Setup",
  "trigger": "init"
}
```

<h4 id="setup-decision-control">
  Setup decision control
</h4>

Gli hook Setup non possono bloccare; l'esecuzione continua su qualsiasi codice di uscita. Su ogni codice di uscita, Claude Code scarta i [JSON output fields](#json-output) di un hook Setup, come `systemMessage`, `continue`, e `hookSpecificOutput.additionalContext`. Con `-p`, stdout, stderr e il codice di uscita di un hook Setup appaiono nell'output dell'esecuzione solo come [`hook_response` events](/docs/it/headless#read-session-metadata) quando avvii con `--output-format stream-json --verbose`.

Gli hook Setup hanno accesso a `CLAUDE_ENV_FILE`. Le variabili scritte in quel file persistono nei comandi Bash successivi per la sessione, proprio come negli [hook SessionStart](#persist-environment-variables). Solo gli hook `type: "command"` si eseguono su `Setup`. Un hook `type: "mcp_tool"` su `Setup` viene sempre saltato, come descritto sotto [MCP tool hook fields](#mcp-tool-hook-fields).

<h3 id="instructionsloaded">
  InstructionsLoaded
</h3>

Si attiva quando un file `CLAUDE.md` o `.claude/rules/*.md` viene caricato nel contesto. Questo evento si attiva all'avvio della sessione per i file caricati con entusiasmo e di nuovo più tardi quando i file vengono caricati pigrizia, ad esempio quando Claude accede a una sottodirectory che contiene un `CLAUDE.md` annidato o quando le regole condizionali con frontmatter `paths:` corrispondono. L'hook non supporta il blocco o il controllo delle decisioni. Si esegue in modo asincrono per scopi di osservabilità.

Questo evento non si attiva quando Claude [legge `AGENTS.md` direttamente](/docs/it/memory#agents-md) attraverso l'impostazione **Project instructions**. Si attiva quando un `CLAUDE.md` importa il tuo `AGENTS.md`, con `load_reason` impostato su `include` come per qualsiasi altro file importato, e quando `CLAUDE.md` è un symlink ad esso, come un caricamento `CLAUDE.md` normale.

Il matcher viene eseguito su `load_reason`. Ad esempio, usa `"matcher": "session_start"` per attivarsi solo per i file caricati all'avvio della sessione, o `"matcher": "path_glob_match|nested_traversal"` per attivarsi solo per i caricamenti pigri.

<h4 id="instructionsloaded-input">
  InstructionsLoaded input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook InstructionsLoaded ricevono questi campi:

| Field               | Description                                                                                                                                                                                                                               |
| :------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `file_path`         | Percorso assoluto al file di istruzioni che è stato caricato                                                                                                                                                                              |
| `memory_type`       | Ambito del file: `"User"`, `"Project"`, `"Local"`, o `"Managed"`                                                                                                                                                                          |
| `load_reason`       | Perché il file è stato caricato: `"session_start"`, `"nested_traversal"`, `"path_glob_match"`, `"include"`, o `"compact"`. Il valore `"compact"` si attiva quando i file di istruzioni vengono ricaricati dopo un evento di compattazione |
| `globs`             | Modelli di glob di percorso dal frontmatter `paths:` del file, se presenti. Presente solo per i caricamenti `path_glob_match`                                                                                                             |
| `trigger_file_path` | Percorso al file il cui accesso ha attivato questo caricamento, per i caricamenti pigri                                                                                                                                                   |
| `parent_file_path`  | Percorso al file di istruzioni genitore che ha incluso questo, per i caricamenti `include`                                                                                                                                                |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "InstructionsLoaded",
  "file_path": "/Users/my-project/CLAUDE.md",
  "memory_type": "Project",
  "load_reason": "session_start"
}
```

<h4 id="instructionsloaded-decision-control">
  InstructionsLoaded decision control
</h4>

Gli hook InstructionsLoaded non hanno controllo delle decisioni. Non possono bloccare o modificare il caricamento delle istruzioni. Claude Code scarta i loro [JSON output fields](#json-output), come `systemMessage` e `continue`. Usa questo evento per il logging di audit, il tracciamento della conformità, o l'osservabilità.

<h3 id="userpromptsubmit">
  UserPromptSubmit
</h3>

Si esegue quando l'utente invia un prompt, prima che Claude lo elabori. Questo ti consente di aggiungere contesto aggiuntivo basato sul prompt/conversazione, convalidare i prompt, o bloccare determinati tipi di prompt.

Gli hook `UserPromptSubmit` hanno un timeout predefinito di 30 secondi per i tipi `command`, `http`, e `mcp_tool`, più breve del default di 600 secondi per quei tipi sulla maggior parte degli altri eventi. Poiché questo hook si esegue prima di ogni prompt e blocca l'elaborazione del modello finché non si completa, un hook bloccato blocca la sessione. Se il tuo hook ha bisogno di più tempo, imposta il campo `timeout` nella voce dell'hook.

A parte un hook di comando che esegui con [`async: true`](#run-hooks-in-the-background), un hook `UserPromptSubmit` di comando, HTTP, o MCP tool che raggiunge il suo timeout viene annullato e il suo output, incluso qualsiasi `additionalContext`, viene scartato. Il prompt raggiunge comunque Claude senza quel contesto. La trascrizione mostra un avviso che nomina l'hook, il timeout che si è attivato, e che l'output è stato scartato.

Un [Agent SDK callback hook](/docs/it/agent-sdk/hooks) su `UserPromptSubmit` che raggiunge il suo timeout blocca il prompt con un messaggio che nomina l'hook e il timeout, perché un callback lì può agire come un gate di policy che non deve fallire in modo aperto. La sessione continua. Prima della v2.1.208, un timeout di callback su quell'evento terminava il turno con un errore di esecuzione.

<h4 id="userpromptsubmit-input">
  UserPromptSubmit input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook UserPromptSubmit ricevono il campo `prompt` contenente il testo che l'utente ha inviato. Il contenuto incollato che è crollato in un segnaposto `[Pasted text #N]` arriva espanso al suo posto. Nelle sessioni in cui Claude Code [contrassegna il testo incollato per Claude](/docs/it/terminal-config#how-claude-treats-pasted-text), quel contenuto espanso si trova tra una riga `<pasted_content id="…">` e una riga `</pasted_content id="…">`, quindi tieni conto di quelle righe se il tuo hook analizza il prompt.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "UserPromptSubmit",
  "prompt": "Write a function to calculate the factorial of a number"
}
```

<h4 id="userpromptsubmit-decision-control">
  UserPromptSubmit decision control
</h4>

Gli hook `UserPromptSubmit` possono controllare se un prompt utente viene elaborato e aggiungere contesto. Tutti i [JSON output fields](#json-output) sono disponibili.

Ci sono due modi per aggiungere contesto alla conversazione al codice di uscita 0:

* **Plain text stdout**: Claude Code aggiunge stdout che [tratta come testo semplice](#exit-code-0) al contesto di Claude
* **JSON con `additionalContext`**: usa il formato JSON sottostante per più controllo. Il campo `additionalContext` viene aggiunto come contesto

Nessuno dei due canali produce una voce di trascrizione visibile. Lo stdout semplice e il valore `additionalContext` vengono ciascuno iniettati come un promemoria di sistema che inizia con il nome dell'hook; Claude legge entrambi. Per confermare la consegna, controlla il [debug log](#debug-hooks).

Per bloccare un prompt, restituisci un oggetto JSON con `decision` impostato su `"block"`:

| Field                    | Description                                                                                                               |
| :----------------------- | :------------------------------------------------------------------------------------------------------------------------ |
| `decision`               | `"block"` impedisce l'elaborazione del prompt e lo cancella dal contesto. Ometti per consentire al prompt di procedere    |
| `reason`                 | Mostrato all'utente quando `decision` è `"block"`. Non aggiunto al contesto                                               |
| `additionalContext`      | Stringa aggiunta al contesto di Claude insieme al prompt inviato. Vedi [Add context for Claude](#add-context-for-claude)  |
| `sessionTitle`           | Imposta il titolo della sessione. Usa per nominare le sessioni automaticamente in base al contenuto del prompt            |
| `suppressOriginalPrompt` | Se `true` quando `decision` è `"block"`, omette il testo del prompt originale dal messaggio di blocco mostrato all'utente |

Un hook che blocca uscendo con 2 si instrada nello stesso modo di `reason`: il messaggio di blocco mostra il testo stderr all'utente, e non viene aggiunto al contesto.

```json theme={null}
{
  "decision": "block",
  "reason": "Explanation for decision",
  "hookSpecificOutput": {
    "hookEventName": "UserPromptSubmit",
    "additionalContext": "My additional context here",
    "sessionTitle": "My session title"
  }
}
```

<h3 id="userpromptexpansion">
  UserPromptExpansion
</h3>

Si esegue quando un comando digitato dall'utente si espande in un prompt prima di raggiungere Claude. Usalo per bloccare comandi specifici dall'invocazione diretta, iniettare contesto per una skill particolare, o registrare quali comandi gli utenti invocano. Ad esempio, un hook che corrisponde a `deploy` può bloccare `/deploy` a meno che un file di approvazione sia presente, o un hook che corrisponde a una skill di revisione può aggiungere la lista di controllo di revisione del team come `additionalContext`.

Questo evento copre il percorso che `PreToolUse` non copre: un hook `PreToolUse` che corrisponde allo strumento `Skill` si attiva solo quando Claude chiama lo strumento, ma digitare `/skillname` direttamente bypassa `PreToolUse`. `UserPromptExpansion` si attiva su quel percorso diretto.

Corrisponde a `command_name`. Lascia il matcher vuoto per attivarsi su ogni comando di tipo prompt.

<h4 id="userpromptexpansion-input">
  UserPromptExpansion input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook UserPromptExpansion ricevono `expansion_type`, `command_name`, `command_args`, `command_source`, e la stringa `prompt` originale. Il campo `expansion_type` è `slash_command` per le skill e i comandi personalizzati, o `mcp_prompt` per i prompt del server MCP.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../00893aaf.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "UserPromptExpansion",
  "expansion_type": "slash_command",
  "command_name": "example-skill",
  "command_args": "arg1 arg2",
  "command_source": "plugin",
  "prompt": "/example-skill arg1 arg2"
}
```

<h4 id="userpromptexpansion-decision-control">
  UserPromptExpansion decision control
</h4>

Gli hook `UserPromptExpansion` possono bloccare l'espansione o aggiungere contesto. Tutti i [JSON output fields](#json-output) sono disponibili.

| Field               | Description                                                                                                              |
| :------------------ | :----------------------------------------------------------------------------------------------------------------------- |
| `decision`          | `"block"` impedisce l'espansione del comando. Ometti per consentire a esso di procedere                                  |
| `reason`            | Mostrato all'utente quando `decision` è `"block"`                                                                        |
| `additionalContext` | Stringa aggiunta al contesto di Claude insieme al prompt espanso. Vedi [Add context for Claude](#add-context-for-claude) |

Un hook che blocca uscendo con 2 si instrada nello stesso modo di `reason`: il messaggio di blocco mostra il testo stderr all'utente.

```json theme={null}
{
  "decision": "block",
  "reason": "This slash command is not available",
  "hookSpecificOutput": {
    "hookEventName": "UserPromptExpansion",
    "additionalContext": "Additional context for this expansion"
  }
}
```

<h3 id="messagedisplay">
  MessageDisplay
</h3>

Si esegue mentre un messaggio dell'assistente viene trasmesso sullo schermo. Claude Code visualizza il messaggio in incrementi: ogni volta che un batch di righe appena completate è pronto per il rendering, l'hook si esegue una volta con quelle righe e Claude Code esegue il rendering del testo di sostituzione dell'hook al loro posto. Un messaggio lungo produce diverse chiamate; un messaggio breve potrebbe produrne solo una.

Usa MessageDisplay per:

* rimuovere il markdown per una visualizzazione minima
* trasformare il testo che un'applicazione Agent SDK mostra ai suoi utenti
* redarre chiavi API o nomi host interni dalle risposte di Claude

Claude Code tiene ogni batch finché il tuo hook non ritorna, quindi mantieni l'hook veloce. Se l'hook fallisce o scade, Claude Code visualizza il testo originale. Il timeout predefinito per questo evento è 10 secondi; se il tuo hook ha bisogno di più tempo, imposta il campo `timeout` nella voce dell'hook.

MessageDisplay è solo per la visualizzazione: il testo di sostituzione cambia solo ciò che viene renderizzato sullo schermo. La trascrizione e ciò che Claude vede mantengono il testo originale, quindi Claude non vede mai la sostituzione, e la modalità verbose mostra l'originale. L'hook riceve solo il testo del messaggio dell'assistente, quindi i risultati degli strumenti e il testo che digiti vengono renderizzati invariati.

MessageDisplay non supporta i matcher e si attiva per ogni messaggio dell'assistente che trasmette testo; i messaggi senza testo, come le risposte solo con chiamate di strumenti, non lo attivano.

Nelle esecuzioni non interattive, incluse le query Agent SDK e `claude -p`, MessageDisplay si esegue una volta per messaggio dell'assistente invece che una volta per batch di righe. La singola chiamata arriva dopo che il messaggio si completa e porta il testo del messaggio completo: `index` è `0`, `final` è `true`, e `delta` contiene l'intero messaggio. Un hook che raccoglie il testo `delta` per ogni messaggio riceve lo stesso testo totale in entrambe le modalità.

<h4 id="messagedisplay-input">
  MessageDisplay input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook MessageDisplay ricevono identificatori per il turno e il messaggio, la posizione di questa chiamata all'interno del messaggio, e il nuovo testo in `delta`. I confini dei batch dipendono da come il testo viene trasmesso, quindi usa `index` e `final` per tracciare il progresso attraverso un messaggio piuttosto che aspettarsi che le righe siano raggruppate in un modo particolare.

| Field        | Description                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :----------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `turn_id`    | UUID del turno corrente                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `message_id` | UUID del messaggio dell'assistente visualizzato. Stabile su ogni batch dello stesso messaggio. Questo non è l'ID API `msg_…`, quindi non può essere correlato con gli ID dei messaggi della trascrizione                                                                                                                                                                                                                                      |
| `index`      | Indice a base zero di questo batch all'interno del messaggio                                                                                                                                                                                                                                                                                                                                                                                  |
| `final`      | `true` sul batch finale del messaggio. Ogni messaggio ha esattamente un batch finale                                                                                                                                                                                                                                                                                                                                                          |
| `delta`      | Le righe appena completate dall'ultimo batch, incluse le newline finali. Sempre righe intere, tranne il batch finale che potrebbe terminare a metà riga. Nelle esecuzioni interattive, il delta del batch finale è vuoto quando il messaggio termina su una newline, quindi tratta `final`, non un delta non vuoto, come il segnale di fine messaggio. Nelle esecuzioni Agent SDK e `claude -p`, la singola chiamata porta l'intero messaggio |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "MessageDisplay",
  "turn_id": "0c9e6a2f-7d41-4f4e-9a15-3f4f7c2b8d10",
  "message_id": "5b2a9c8e-1f63-4d8a-b7c4-9e0d2a6f1c3b",
  "index": 0,
  "final": false,
  "delta": "Here is the plan:\n"
}
```

<h4 id="messagedisplay-output">
  MessageDisplay output
</h4>

Oltre ai [JSON output fields](#json-output) disponibili per tutti gli hook, gli hook MessageDisplay possono restituire `displayContent` per sostituire il delta sullo schermo:

| Field            | Description                                                                  |
| :--------------- | :--------------------------------------------------------------------------- |
| `displayContent` | Testo visualizzato al posto del delta. Omettilo per visualizzare l'originale |

Gli hook MessageDisplay non hanno controllo delle decisioni. Non possono bloccare il messaggio o cambiare ciò che viene archiviato nella trascrizione o inviato a Claude. Claude Code agisce su `displayContent` dal loro output JSON e scarta `systemMessage` e `continue`.

Questo esempio rimuove la formattazione markdown dalle risposte di Claude per una visualizzazione in testo semplice. Lo script legge ogni batch da stdin, rimuove i marcatori di grassetto e i backtick di codice inline da `delta`, e restituisce il risultato come `displayContent`.

<Tabs>
  <Tab title="macOS/Linux">
    Registra un hook di comando per l'evento nel tuo file di impostazioni:

    ```json theme={null}
    {
      "hooks": {
        "MessageDisplay": [
          {
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/plain-display.sh",
                "args": []
              }
            ]
          }
        ]
      }
    }
    ```

    Salva questo script in `.claude/hooks/plain-display.sh` nel tuo progetto e rendilo eseguibile con `chmod +x`:

    ```bash theme={null}
    #!/bin/bash
    jq '{hookSpecificOutput: {hookEventName: "MessageDisplay", displayContent: (.delta | gsub("\\*\\*"; "") | gsub("`"; ""))}}'
    ```
  </Tab>

  <Tab title="Windows (PowerShell)">
    Registra un hook di comando che esegue lo script tramite PowerShell:

    ```json theme={null}
    {
      "hooks": {
        "MessageDisplay": [
          {
            "hooks": [
              {
                "type": "command",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/plain-display.ps1"
                ]
              }
            ]
          }
        ]
      }
    }
    ```

    Il flag `-NoProfile` salta il caricamento del tuo profilo PowerShell in modo che l'hook si avvii velocemente, e `-ExecutionPolicy Bypass` consente a PowerShell di eseguire il file di script locale.

    Salva questo script in `.claude/hooks/plain-display.ps1` nel tuo progetto:

    ```powershell theme={null}
    $batch = [Console]::In.ReadToEnd() | ConvertFrom-Json
    $text = $batch.delta -replace '\*\*', '' -replace '`', ''
    @{
      hookSpecificOutput = @{
        hookEventName = "MessageDisplay"
        displayContent = $text
      }
    } | ConvertTo-Json
    ```
  </Tab>
</Tabs>

I batch senza markdown passano invariati. Se lo script fallisce, ad esempio perché `jq` manca, Claude Code visualizza il testo originale e nota il fallimento solo nell'[output di debug](#debug-hooks), non nella sessione.

<h3 id="pretooluse">
  PreToolUse
</h3>

Si esegue dopo che Claude crea i parametri dello strumento e prima di elaborare la chiamata dello strumento. Corrisponde a qualsiasi nome di strumento tranne `EndConversation`: strumenti incorporati come `Bash`, `PowerShell`, `Edit`, `Write`, `Read`, `Glob`, `Grep`, `Agent`, `Workflow`, `WebFetch`, `WebSearch`, `AskUserQuestion`, e `ExitPlanMode`, e qualsiasi [MCP tool names](#match-mcp-tools).

Per eseguire un hook quando un file specifico cambia su disco, qualunque cosa l'abbia scritto, usa [FileChanged](#filechanged) invece di corrispondere ai file-editing tools per nome. A differenza di PreToolUse, Claude Code esegue gli hook FileChanged dopo il cambiamento, e non hanno controllo delle decisioni, quindi non possono bloccare la scrittura.

<Warning>
  PreToolUse si esegue solo quando Claude chiama uno strumento. I file che [riferisci con `@` nel tuo prompt](/docs/it/common-workflows#reference-files-and-directories) vengono aggiunti senza alcuna chiamata di strumento: Claude Code inserisce i loro contenuti mentre costruisce il prompt, quindi nessun hook PreToolUse si attiva per loro, inclusi gli hook che corrispondono a `Read`. Per bloccare percorsi specifici dai riferimenti `@`, usa una [regola di negazione `Read`](/docs/it/permissions#read-and-edit) invece.

  PreToolUse inoltre non si attiva per [`EndConversation`](/docs/it/tools-reference#endconversation-tool-behavior).
</Warning>

Usa [PreToolUse decision control](#pretooluse-decision-control) per consentire, negare, chiedere, o rinviare la chiamata dello strumento.

Un [Agent SDK callback hook](/docs/it/agent-sdk/hooks) su `PreToolUse` che supera il suo timeout blocca la chiamata dello strumento, e Claude riceve un risultato di errore che nomina il timeout. Un esplicito rifiuto restituito da un altro hook ha comunque la precedenza.

<h4 id="pretooluse-input">
  PreToolUse input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook PreToolUse ricevono `tool_name`, `tool_input`, e `tool_use_id`.

Per uno [strumento MCP](#match-mcp-tools), l'input porta anche `mcp_server`, un oggetto con il `name` del server e una `source` che dice da dove proviene la definizione del server. I valori `source` includono `plugin`, `sdk`, e ambiti di configurazione come `user` e `project`. [`McpServerProvenance`](/docs/it/agent-sdk/typescript#mcpserverprovenance) nel riferimento Agent SDK li elenca tutti e dice come trattarne uno che non riconosci. Basa le decisioni di fiducia su `source` piuttosto che su `name` o il prefisso del nome dello strumento `mcp__<server>__`. Il campo `mcp_server` richiede Claude Code v2.1.274 o successivo.

Per gli strumenti di file `Write`, `Edit`, e `Read`, `tool_input.file_path` è sempre assoluto:

* Claude Code espande `~` e i percorsi relativi prima che gli hook si eseguano, quindi un hook che corrisponde ai percorsi non può essere bypassato tramite `~` o un'ortografia relativa dello stesso percorso
* Su Windows, il percorso arriva con separatori backslash, anche quando il tuo hook si esegue sotto Git Bash dove `$PWD` sembra `/c/project`
* Un confronto scritto con barre in avanti, come un controllo `/src/`, non corrisponde mai a un percorso con backslash, e la chiamata dello strumento procede come se l'hook non avesse nulla da bloccare
* Normalizza i separatori prima di confrontare: `FILE_PATH="${FILE_PATH//\\//}"` in Bash, o `file_path.replace("\\", "/")` in Python, quindi corrisponde a un segmento di percorso come `/src/` piuttosto che ancorare con `^`, poiché il percorso è assoluto

Una chiamata `Write` su Windows consegna:

```json theme={null}
{
  "hook_event_name": "PreToolUse",
  "tool_name": "Write",
  "tool_input": {
    "file_path": "C:\\project\\src\\index.ts",
    "content": "..."
  },
  ...
}
```

I campi `tool_input` dipendono dallo strumento:

<a id="bash" />

<h5 id="bash">
  Bash
</h5>

Esegue comandi shell.

| Field               | Type    | Example            | Description                                                                                                                                                     |
| :------------------ | :------ | :----------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | string  | `"npm test"`       | Il comando shell da eseguire                                                                                                                                    |
| `description`       | string  | `"Run test suite"` | Descrizione facoltativa di cosa fa il comando                                                                                                                   |
| `timeout`           | number  | `120000`           | Timeout facoltativo in millisecondi. I valori superiori al [massimo](/docs/it/tools-reference#bash-tool-behavior) vengono ridotti al massimo piuttosto che rifiutati |
| `run_in_background` | boolean | `false`            | Se eseguire il comando in background                                                                                                                            |

Quando un comando Bash cambia file in un repository Git, Claude Code può registrare cosa è cambiato. Lo registra in ogni modo di permesso quando l'impostazione [`bashEditDiffEnabled`](/docs/it/settings-reference#basheditdiffenabled) attiva la registrazione; la voce di quell'impostazione dice quali file possono impostarla. Altrimenti la registra solo in modalità auto e modalità `bypassPermissions`, e solo quando Claude Code dirige Claude a modificare file tramite Bash. Imposta `bashEditDiffEnabled` su `false` per disattivare la registrazione. I comandi in background e i comandi di sola lettura non portano diff.

Il tuo hook [PostToolUse](#posttooluse) riceve quindi i file modificati in `tool_response.bashEditDiff`. L'elenco copre ciò che è cambiato sotto il repository mentre il comando era in esecuzione. I file che Git ignora e i file nei submoduli non sono elencati. Richiede Claude Code v2.1.269 o successivo.

<Note>
  L'elenco è best effort e in beta pubblica. Claude Code può perdere un cambiamento, includere un file che un altro processo ha cambiato contemporaneamente, o fermarsi ai suoi limiti di dimensione. La forma del campo potrebbe cambiare. Usa l'elenco per trovare cosa rivedere, non per applicare una politica.
</Note>

`changedFiles` e `files` elencano ciò che il comando ha cambiato; i campi rimanenti dicono quanto è completo e quanto è affidabile quell'elenco.

| Field          | Type    | Example                                                 | Description                                                                                                                                                                                                      |
| :------------- | :------ | :------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `changedFiles` | array   | `["/path/to/src/app.ts"]`                               | Percorsi assoluti dei file che il comando ha cambiato, al massimo 200. Presente ogni volta che `files` contiene un diff o `moreFiles` è superiore a zero                                                         |
| `files`        | array   | `[{"filePath": "/path/to/src/app.ts", "hunks": [...]}]` | Diff di fino a 5 file modificati, per la visualizzazione. `created` o `deleted` è `true` per un file che il comando ha aggiunto o rimosso                                                                        |
| `moreFiles`    | number  | `2`                                                     | Conteggio dei file modificati senza diff in `files`                                                                                                                                                              |
| `unavailable`  | boolean | `true`                                                  | Impostato quando il diff è incompleto o non potrebbe essere preso                                                                                                                                                |
| `skipped`      | boolean | `true`                                                  | Impostato per un comando Git che sposta l'albero di lavoro, come `git checkout` o `git stash`, quindi Claude Code non prende diff                                                                                |
| `shared`       | boolean | `true`                                                  | Impostato quando un'altra chiamata di strumento Bash, come quella di un subagent, si è eseguita nello stesso repository contemporaneamente, quindi alcuni cambiamenti elencati potrebbero essere di quel comando |

<a id="powershell" />

<h5 id="powershell">
  PowerShell
</h5>

Esegue comandi PowerShell. Vedi lo [strumento PowerShell](/docs/it/tools-reference#powershell-tool) per la disponibilità per piattaforma.

I campi corrispondono allo strumento Bash, con la stringa di comando in `command`:

| Field               | Type    | Example                    | Description                                   |
| :------------------ | :------ | :------------------------- | :-------------------------------------------- |
| `command`           | string  | `"Get-ChildItem -Recurse"` | Il comando PowerShell da eseguire             |
| `description`       | string  | `"List files recursively"` | Descrizione facoltativa di cosa fa il comando |
| `timeout`           | number  | `120000`                   | Timeout facoltativo in millisecondi           |
| `run_in_background` | boolean | `false`                    | Se eseguire il comando in background          |

Corrisponde a `Bash|PowerShell` negli hook che ispezionano i comandi shell, in modo che coprano entrambi gli strumenti:

* Su Windows, ovunque lo strumento PowerShell sia abilitato, Claude tratta PowerShell come la shell primaria e instrada i comandi shell attraverso di esso.
* Su Windows senza Git Bash, lo strumento è abilitato automaticamente e Claude Code non registra affatto lo strumento Bash.
* Un hook che corrisponde solo a `Bash` non si attiva mai lì.

<h5 id="write">
  Write
</h5>

Crea o sovrascrive un file.

| Field       | Type   | Example               | Description                           |
| :---------- | :----- | :-------------------- | :------------------------------------ |
| `file_path` | string | `"/path/to/file.txt"` | Percorso assoluto al file da scrivere |
| `content`   | string | `"file content"`      | Contenuto da scrivere nel file        |

<h5 id="edit">
  Edit
</h5>

Sostituisce una stringa in un file esistente.

| Field         | Type    | Example               | Description                             |
| :------------ | :------ | :-------------------- | :-------------------------------------- |
| `file_path`   | string  | `"/path/to/file.txt"` | Percorso assoluto al file da modificare |
| `old_string`  | string  | `"original text"`     | Testo da trovare e sostituire           |
| `new_string`  | string  | `"replacement text"`  | Testo di sostituzione                   |
| `replace_all` | boolean | `false`               | Se sostituire tutte le occorrenze       |

<h5 id="read">
  Read
</h5>

Legge i contenuti del file.

| Field       | Type   | Example               | Description                                           |
| :---------- | :----- | :-------------------- | :---------------------------------------------------- |
| `file_path` | string | `"/path/to/file.txt"` | Percorso assoluto al file da leggere                  |
| `offset`    | number | `10`                  | Numero di riga facoltativo da cui iniziare la lettura |
| `limit`     | number | `50`                  | Numero facoltativo di righe da leggere                |

<h5 id="glob">
  Glob
</h5>

Trova file che corrispondono a un modello glob.

| Field     | Type   | Example          | Description                                                                         |
| :-------- | :----- | :--------------- | :---------------------------------------------------------------------------------- |
| `pattern` | string | `"**/*.ts"`      | Modello glob per corrispondere ai file                                              |
| `path`    | string | `"/path/to/dir"` | Directory facoltativa in cui cercare. Predefinito alla directory di lavoro corrente |

<h5 id="grep">
  Grep
</h5>

Cerca i contenuti dei file con espressioni regolari.

| Field         | Type    | Example          | Description                                                                            |
| :------------ | :------ | :--------------- | :------------------------------------------------------------------------------------- |
| `pattern`     | string  | `"TODO.*fix"`    | Modello di espressione regolare da cercare                                             |
| `path`        | string  | `"/path/to/dir"` | File o directory facoltativo in cui cercare                                            |
| `glob`        | string  | `"*.ts"`         | Modello glob facoltativo per filtrare i file                                           |
| `output_mode` | string  | `"content"`      | `"content"`, `"files_with_matches"`, o `"count"`. Predefinito a `"files_with_matches"` |
| `-i`          | boolean | `true`           | Ricerca senza distinzione tra maiuscole e minuscole                                    |
| `multiline`   | boolean | `false`          | Abilita la corrispondenza multiriga                                                    |

<h5 id="webfetch">
  WebFetch
</h5>

Recupera ed elabora il contenuto web.

| Field    | Type   | Example                       | Description                                 |
| :------- | :----- | :---------------------------- | :------------------------------------------ |
| `url`    | string | `"https://example.com/api"`   | URL da cui recuperare il contenuto          |
| `prompt` | string | `"Extract the API endpoints"` | Prompt da eseguire sul contenuto recuperato |

<h5 id="websearch">
  WebSearch
</h5>

Cerca il web.

| Field             | Type   | Example                        | Description                                          |
| :---------------- | :----- | :----------------------------- | :--------------------------------------------------- |
| `query`           | string | `"react hooks best practices"` | Query di ricerca                                     |
| `allowed_domains` | array  | `["docs.example.com"]`         | Facoltativo: includi solo risultati da questi domini |
| `blocked_domains` | array  | `["spam.example.com"]`         | Facoltativo: escludi risultati da questi domini      |

<h5 id="agent">
  Agent
</h5>

Genera un [subagent](/docs/it/sub-agents).

| Field           | Type   | Example                    | Description                                                    |
| :-------------- | :----- | :------------------------- | :------------------------------------------------------------- |
| `prompt`        | string | `"Find all API endpoints"` | Il compito per l'agente da eseguire                            |
| `description`   | string | `"Find API endpoints"`     | Breve descrizione del compito                                  |
| `subagent_type` | string | `"Explore"`                | Tipo di agente specializzato da usare                          |
| `model`         | string | `"sonnet"`                 | Alias del modello facoltativo per sovrascrivere il predefinito |

Quando una chiamata Agent in primo piano si completa, il tuo hook [PostToolUse](#posttooluse) riceve il testo finale del subagent e la telemetria di esecuzione in `tool_response`. Leggi questi campi per ispezionare l'esecuzione; per i rollup di token e costi tra i subagent, usa i [token and cost counters](/docs/it/monitoring-usage#token-counter) filtrati a `query_source` `"subagent"`, poiché `totalTokens` e `usage` coprono solo la richiesta finale:

| Field               | Type   | Example                                               | Description                                                                                                                                                                                                                                                      |
| :------------------ | :----- | :---------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`            | string | `"completed"`                                         | `"completed"` per i subagent in primo piano, `"async_launched"` per i subagent in background. A partire dalla v2.1.198, i subagent si eseguono in background per impostazione predefinita, quindi un `run_in_background` omesso produce anche `"async_launched"` |
| `agentId`           | string | `"a4d2c8f1e0b3a297"`                                  | Identificatore per l'esecuzione del subagent                                                                                                                                                                                                                     |
| `content`           | array  | `[{"type": "text", "text": "Found 12 endpoints..."}]` | I blocchi di testo finali del subagent, o, per un subagent il cui rapporto passa attraverso `SubagentHandback`, una breve nota su quel hand-back al loro posto                                                                                                   |
| `resolvedModel`     | string | `"claude-sonnet-4-5"`                                 | Modello su cui il subagent ha iniziato, che potrebbe differire dal modello richiesto                                                                                                                                                                             |
| `modelsUsed`        | array  | `["claude-sonnet-4-5", "claude-haiku-4-5"]`           | Modelli usati in ordine, con ripetizioni consecutive compresse; impostato solo quando il modello è stato scambiato durante l'esecuzione. Richiede Claude Code v2.1.212 o successivo                                                                              |
| `totalTokens`       | number | `12450`                                               | Conteggio dei token dalla richiesta API finale del subagent: token di input, output, e cache combinati. Questo non è un totale su tutta l'esecuzione                                                                                                             |
| `totalDurationMs`   | number | `48211`                                               | Durata di tempo reale dell'esecuzione del subagent                                                                                                                                                                                                               |
| `totalToolUseCount` | number | `7`                                                   | Conteggio delle chiamate di strumenti che il subagent ha effettuato                                                                                                                                                                                              |
| `usage`             | object | `{"input_tokens": 8320, ...}`                         | Suddivisione dei token per tipo della richiesta API finale: `input_tokens`, `output_tokens`, `cache_creation_input_tokens`, `cache_read_input_tokens`                                                                                                            |

Su Claude Code v2.1.271 o successivo, un subagent che si esegue con lo strumento [`SubagentHandback`](/docs/it/tools-reference), che Claude Code fornisce in [auto mode](/docs/it/permission-modes#eliminate-prompts-with-auto-mode), consegna il suo rapporto attraverso quello strumento piuttosto che restituirlo come testo. Il campo `content` del suo risultato `completed` porta quindi una breve nota su quel hand-back piuttosto che il rapporto stesso. Per leggere il rapporto, abbina un hook `PreToolUse` o `PostToolUse` su `SubagentHandback` e leggi `tool_input.message`.

Per i subagent in background, lo strumento ritorna quando il compito si sposta in background, quindi `tool_response` non porta campi di utilizzo: un lancio in background ritorna immediatamente, e un compito in primo piano che Claude Code sposta in background durante l'esecuzione ritorna a quella transizione. Ha `status: "async_launched"`, `agentId`, `description`, `prompt`, `outputFile`, e `resolvedModel`.

Su una risposta `completed`, `resolvedModel` nomina il modello su cui il subagent ha iniziato, che può differire dal valore `model` in `tool_input`, come quando `availableModels` o un altro override si applica. Su una risposta `async_launched`, `resolvedModel` nomina il modello in uso quando l'agente si è spostato in background, quindi uno scambio che è accaduto prima dello spostamento in background si riflette lì. `modelsUsed` e il comportamento di `resolvedModel` al momento dello spostamento in background richiedono Claude Code v2.1.212 o successivo.

<a id="askuserquestion" />

<h5 id="askuserquestion">
  AskUserQuestion
</h5>

Chiede all'utente da una a quattro domande a scelta multipla.

| Field       | Type   | Example                                                                                                            | Description                                                                                                                                                                                                                                        |
| :---------- | :----- | :----------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `questions` | array  | `[{"question": "Which framework?", "header": "Framework", "options": [{"label": "React"}], "multiSelect": false}]` | Domande da presentare, ciascuna con una stringa `question`, un breve `header`, un array `options`, e un flag `multiSelect` facoltativo                                                                                                             |
| `answers`   | object | `{"Which framework?": "React"}`                                                                                    | Facoltativo. Mappa il testo della domanda all'etichetta dell'opzione selezionata. Le risposte multi-select uniscono le etichette con virgole. Claude non imposta questo campo; forniscilo tramite `updatedInput` per rispondere programmaticamente |

<h5 id="exitplanmode">
  ExitPlanMode
</h5>

Presenta un piano e chiede all'utente di approvarlo prima che Claude lasci la [plan mode](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode). Claude scrive il piano in un file su disco prima di chiamare lo strumento, quindi il `tool_input` letterale dal modello è tipicamente vuoto. Claude Code inietta il contenuto del piano e il percorso del file prima di passare l'input agli hook.

| Field            | Type   | Example                                     | Description                                                                                                                                                     |
| :--------------- | :----- | :------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plan`           | string | `"## Refactor auth\n1. Extract..."`         | Contenuto del piano in Markdown. Iniettato dal file del piano su disco                                                                                          |
| `planFilePath`   | string | `"/Users/.../plans/refactor-auth.md"`       | Percorso al file del piano. Iniettato                                                                                                                           |
| `allowedPrompts` | array  | `[{"tool": "Bash", "prompt": "run tests"}]` | Deprecato. Claude Code accetta il campo ma lo ignora. Prima della v2.1.205, portava permessi basati su prompt che Claude ha richiesto per implementare il piano |

In `PostToolUse`, `tool_response` è un oggetto con campi `plan` e `filePath` che contengono il piano approvato, più flag di stato interni. Leggi `tool_response.plan` per il contenuto del piano piuttosto che rileggere il file da disco.

<h4 id="pretooluse-decision-control">
  PreToolUse decision control
</h4>

Gli hook `PreToolUse` possono controllare se una chiamata di strumento procede. A differenza di altri hook che usano un campo `decision` di livello superiore, PreToolUse restituisce la sua decisione all'interno di un oggetto `hookSpecificOutput`. Questo gli dà un controllo più ricco: quattro risultati (consenti, nega, chiedi, o rinvia) più la capacità di modificare l'input dello strumento prima dell'esecuzione.

| Field                      | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissionDecision`       | `"allow"` salta il prompt di permesso, tranne per le [azioni che nessuna modalità auto-approva](/docs/it/permission-modes#actions-no-mode-auto-approves) e per `AskUserQuestion` e `ExitPlanMode`, che hanno bisogno di [`updatedInput` accoppiato con esso](#allow-with-updatedinput). `"deny"` impedisce la chiamata dello strumento. `"ask"` chiede all'utente di confermare. `"defer"` esce correttamente in modo che lo strumento possa essere ripreso più tardi. Le [regole di negazione e richiesta](/docs/it/permissions#manage-permissions) vengono comunque valutate indipendentemente da ciò che l'hook restituisce |
| `permissionDecisionReason` | Per `"allow"` e `"ask"`, mostrato all'utente ma non a Claude. Per `"deny"`, mostrato a Claude. Per `"defer"`, ignorato                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `updatedInput`             | Modifica i parametri di input dello strumento prima dell'esecuzione. Sostituisce l'intero oggetto di input, quindi includi i campi invariati insieme a quelli modificati. Claude Code valuta le regole di permesso e l'idoneità di [auto-background](/docs/it/tools-reference#background-commands) di un comando Bash rispetto all'input che il tuo hook restituisce, non l'input che Claude ha inviato. Combina con `"allow"` per auto-approvare, o `"ask"` per mostrare l'input modificato all'utente. Per `"defer"`, ignorato                                                                                          |
| `additionalContext`        | Stringa aggiunta al contesto di Claude insieme al risultato dello strumento. Ignorato quando `permissionDecision` è `"defer"`. Vedi [Add context for Claude](#add-context-for-claude)                                                                                                                                                                                                                                                                                                                                                                                                                                |

Quando più hook PreToolUse restituiscono decisioni diverse, la precedenza è `deny` > `defer` > `ask` > `allow`.

Un hook che blocca uscendo con 2 si instrada nello stesso modo di `"deny"`: Claude vede il messaggio stderr come il motivo della negazione.

Quando un hook restituisce `"ask"`, il prompt di permesso visualizzato all'utente include un'etichetta che identifica da dove proviene l'hook: `[settings]` per un hook da qualsiasi file di impostazioni o dal frontmatter dell'agente, `[plugin:<name>]` per l'hook di un plugin, o `[skill]` per un hook dal frontmatter della skill. Questo aiuta gli utenti a capire quale fonte di configurazione sta richiedendo la conferma.

Un `"ask"` di un hook forza anche un prompt di permesso in [auto mode](/docs/it/permission-modes#eliminate-prompts-with-auto-mode): il classificatore può comunque negare la chiamata dello strumento, ma non può approvare la chiamata silenziosamente. Prima della v2.1.211, il classificatore poteva approvare un comando Bash in esecuzione al di fuori della [sandbox](/docs/it/sandboxing) senza mostrare il prompt che l'hook ha richiesto; il classificatore ha comunque applicato le sue stesse regole di sicurezza a quel comando, e un `"deny"` dell'hook era sempre onorato.

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "allow",
    "permissionDecisionReason": "My reason here",
    "updatedInput": {
      "field_to_modify": "new value"
    },
    "additionalContext": "Current environment: production. Proceed with caution."
  }
}
```

<span id="allow-with-updatedinput" />

In [non-interactive mode](/docs/it/headless) con il flag `-p`, Claude Code offre `AskUserQuestion` e `ExitPlanMode` solo quando l'esecuzione ha un [permission host](/docs/it/headless#turn-off-permission-prompts-in-unattended-runs) per ricevere il prompt, come un callback `canUseTool` di Agent SDK. Questi strumenti richiedono l'interazione dell'utente. Restituire `permissionDecision: "allow"` insieme a `updatedInput` soddisfa quel requisito: l'hook legge l'input dello strumento da stdin, raccoglie la risposta attraverso la tua UI, e la restituisce in `updatedInput` in modo che lo strumento si esegua senza chiedere. Restituire `"allow"` da solo non è sufficiente per questi strumenti. Per `AskUserQuestion`, ripeti l'array `questions` originale e aggiungi un oggetto [`answers`](#askuserquestion) che mappa il testo di ogni domanda alla risposta scelta.

A partire dalla v2.1.199, uno strumento MCP il cui server lo contrassegna con [`_meta["anthropic/requiresUserInteraction"]`](/docs/it/mcp#require-approval-for-a-specific-tool) è più rigoroso: un hook non può saltare il suo prompt di approvazione con `"allow"`, con o senza `updatedInput`, perché Claude Code non può confermare che l'hook ha raccolto l'interazione di cui lo strumento ha bisogno.

<Note>
  PreToolUse ha precedentemente usato campi `decision` e `reason` di livello superiore, ma questi sono deprecati per questo evento. Usa `hookSpecificOutput.permissionDecision` e `hookSpecificOutput.permissionDecisionReason` invece. I valori deprecati `"approve"` e `"block"` si mappano a `"allow"` e `"deny"` rispettivamente. Altri eventi come PostToolUse e Stop continuano a usare `decision` e `reason` di livello superiore come loro formato attuale.
</Note>

<h4 id="defer-a-tool-call-for-later">
  Defer a tool call for later
</h4>

`"defer"` è per le integrazioni che eseguono `claude -p` come un sottoprocesso e leggono il suo output JSON, come un'app Agent SDK o un'UI personalizzata costruita su Claude Code. Ti consente a quel processo di chiamata di mettere in pausa Claude a una chiamata di strumento, raccogliere input attraverso la sua interfaccia, e riprendere da dove si era fermato. Claude Code onora questo valore solo in [non-interactive mode](/docs/it/headless) con il flag `-p`. Nelle sessioni interattive registra un avviso e ignora il risultato dell'hook.

Lo strumento `AskUserQuestion` è il caso tipico: Claude vuole chiedere qualcosa all'utente, ma non c'è un terminale per rispondere. Un'esecuzione `-p` offre `AskUserQuestion` solo quando ha un [permission host](/docs/it/headless#turn-off-permission-prompts-in-unattended-runs), come uno strumento MCP che passi con `--permission-prompt-tool`, quindi avvia l'esecuzione con uno. Il round trip funziona così:

1. Claude chiama `AskUserQuestion`. L'hook `PreToolUse` si attiva.
2. L'hook restituisce `permissionDecision: "defer"`. Lo strumento non si esegue. Il processo esce con `stop_reason: "tool_deferred"` e la chiamata dello strumento in sospeso preservata nella trascrizione.
3. Il processo di chiamata legge `deferred_tool_use` dal risultato dell'SDK, visualizza la domanda nella sua UI, e attende una risposta.
4. Il processo di chiamata esegue `claude -p --resume <session-id>` con lo stesso permission host. La stessa chiamata dello strumento attiva `PreToolUse` di nuovo.
5. L'hook restituisce `permissionDecision: "allow"` con la risposta in `updatedInput`. Lo strumento si esegue e Claude continua.

Il campo `deferred_tool_use` porta l'`id`, il `name`, e l'`input` dello strumento. L'`input` è i parametri che Claude ha generato per la chiamata dello strumento, catturati prima dell'esecuzione:

```json theme={null}
{
  "type": "result",
  "subtype": "success",
  "stop_reason": "tool_deferred",
  "session_id": "abc123",
  "deferred_tool_use": {
    "id": "toolu_01abc",
    "name": "AskUserQuestion",
    "input": { "questions": [{ "question": "Which framework?", "header": "Framework", "options": [{"label": "React"}, {"label": "Vue"}], "multiSelect": false }] }
  }
}
```

Non c'è timeout o limite di tentativi. La sessione rimane su disco finché non la riprendi, soggetta alla pulizia [`cleanupPeriodDays`](/docs/it/settings-reference#cleanupperioddays), che elimina i file di sessione dopo 30 giorni per impostazione predefinita, seguendo le [regole di pulizia della conservazione](/docs/it/claude-directory#cleaned-up-automatically). Se la risposta non è pronta quando riprendi, l'hook può restituire `"defer"` di nuovo e il processo esce nello stesso modo. Il processo di chiamata controlla quando interrompere il ciclo restituendo infine `"allow"` o `"deny"` dall'hook.

`"defer"` funziona solo quando Claude effettua una singola chiamata di strumento nel turno. Se Claude effettua diverse chiamate di strumenti contemporaneamente, `"defer"` viene ignorato con un avviso e lo strumento procede attraverso il flusso di permesso normale. Il vincolo esiste perché la ripresa può solo ri-eseguire uno strumento: non c'è modo di rinviare una chiamata da un batch senza lasciare le altre irrisolte.

Se lo strumento rinviato non è più disponibile quando riprendi, il processo esce con `stop_reason: "tool_deferred_unavailable"` e `is_error: true` prima che l'hook si attivi. Questo accade quando un server MCP che ha fornito lo strumento non è connesso per la sessione ripresa. Il payload `deferred_tool_use` è comunque incluso in modo che tu possa identificare quale strumento è scomparso.

<Note>
  Per riprendere una sessione rinviata in plan mode, passa [`--permission-prompt-tool`](/docs/it/cli-reference#cli-flags) insieme a `--resume` in modo che Claude Code possa presentare il piano per l'approvazione. Senza di esso, Claude Code non ripristina la plan mode. Richiede Claude Code v2.1.246 o successivo.

  Quando riprendi con `-p`, Claude Code non ripristina nessun altro modo di permesso archiviato. Avvia l'esecuzione nel modo di permesso che una nuova esecuzione `claude -p` avvierebbe, quindi passa `--permission-mode` o `--dangerously-skip-permissions` di nuovo se la sessione rinviata ne ha usato uno. Quando riprendi con `claude --resume <session-id>` senza `-p`, Claude Code ripristina il modo di permesso archiviato, con le eccezioni elencate in [permission mode on resume](/docs/it/sessions#permission-mode-on-resume).
</Note>

<h3 id="permissionrequest">
  PermissionRequest
</h3>

Si esegue quando Claude Code sta per chiederti il permesso di usare uno strumento. Nelle sessioni che non possono mostrare un prompt, come i subagent in background in [non-interactive mode](/docs/it/headless), Claude Code esegue comunque questi hook, e se nessun hook restituisce una decisione, nega la chiamata dello strumento.
Usa [PermissionRequest decision control](#permissionrequest-decision-control) per consentire o negare per conto dell'utente.

Usa questo evento quando hai bisogno di un segnale nel momento in cui Claude chiede il permesso di usare uno strumento. Claude Code esegue un hook [Notification](#notification) con il tipo `permission_prompt` solo dopo che il prompt ha atteso circa sei secondi.

Claude Code non esegue gli hook PermissionRequest per la [richiesta di rete](/docs/it/sandboxing#network-isolation) di un comando sandboxed. Per ottenere un segnale per quel prompt, usa il tipo di notifica `permission_prompt`.

Corrisponde al nome dello strumento, gli stessi valori di PreToolUse.

<h4 id="permissionrequest-input">
  PermissionRequest input
</h4>

Gli hook PermissionRequest ricevono i campi `tool_name` e `tool_input` come gli hook PreToolUse, ma senza `tool_use_id`. Per uno strumento MCP, ricevono anche l'oggetto [`mcp_server`](#pretooluse-input). Un array `permission_suggestions` facoltativo contiene gli [permission updates](#permission-update-entries) che Claude Code suggerisce per questa richiesta, come aggiungere una regola di consentimento o cambiare il modo di permesso.

L'array `permission_suggestions` non è un elenco esatto delle opzioni che vedi, perché ogni dialogo di permesso costruisce le sue stesse opzioni. Alcuni dialoghi, come quello per le modifiche ai file, non leggono affatto l'array e derivano le loro opzioni dalla richiesta stessa. Un dialogo che lo legge può comunque trattenere un'opzione il cui suggerimento rimane nell'array, ad esempio quando [`allowManagedPermissionRulesOnly`](/docs/it/settings-reference#allowmanagedpermissionrulesonly) nasconde le opzioni di salvataggio delle regole. Può anche offrire opzioni che non hanno una voce di suggerimento, come [**Yes, and switch to auto mode**](/docs/it/permission-modes#switch-permission-modes), che cambia il modo di permesso direttamente piuttosto che attraverso un aggiornamento di permesso.

Gli hook PreToolUse si eseguono prima di ogni chiamata di strumento, indipendentemente dal fatto che abbia bisogno di permesso. Gli hook PermissionRequest si eseguono solo quando Claude Code sta per chiederti il permesso, o quando altrimenti auto-negherebbe una chiamata che non può chiedere. Nessuno dei due eventi si attiva per [`EndConversation`](/docs/it/tools-reference#endconversation-tool-behavior).

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PermissionRequest",
  "tool_name": "Bash",
  "tool_input": {
    "command": "rm -rf node_modules",
    "description": "Remove node_modules directory"
  },
  "permission_suggestions": [
    {
      "type": "addRules",
      "rules": [{ "toolName": "Bash", "ruleContent": "rm -rf node_modules" }],
      "behavior": "allow",
      "destination": "localSettings"
    }
  ]
}
```

<h4 id="permissionrequest-decision-control">
  PermissionRequest decision control
</h4>

Gli hook `PermissionRequest` possono consentire o negare le richieste di permesso. Oltre ai [JSON output fields](#json-output) disponibili per tutti gli hook, il tuo script hook può restituire un oggetto `decision` con questi campi specifici dell'evento:

| Field                | Description                                                                                                                                                                                                                                                                     |
| :------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `behavior`           | `"allow"` concede il permesso, `"deny"` lo nega. Le [regole di negazione e richiesta](/docs/it/permissions#manage-permissions) vengono comunque valutate, quindi un hook che restituisce `"allow"` non sovrascrive una regola di negazione corrispondente                            |
| `updatedInput`       | Solo per `"allow"`: modifica i parametri di input dello strumento prima dell'esecuzione. Sostituisce l'intero oggetto di input, quindi includi i campi invariati insieme a quelli modificati. L'input modificato viene rivalutato rispetto alle regole di negazione e richiesta |
| `updatedPermissions` | Solo per `"allow"`: array di [permission update entries](#permission-update-entries) da applicare, come aggiungere una regola di consentimento o cambiare il modo di permesso della sessione                                                                                    |
| `message`            | Solo per `"deny"`: dice a Claude perché il permesso è stato negato                                                                                                                                                                                                              |
| `interrupt`          | Solo per `"deny"`: se `true`, ferma Claude                                                                                                                                                                                                                                      |

Un hook che esce con 2 senza un oggetto `decision` lascia il flusso di permesso invariato, e il suo stderr viene scartato. Solo l'oggetto `decision` può concedere o negare la richiesta.

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "allow",
      "updatedInput": {
        "command": "npm run lint"
      }
    }
  }
}
```

<h4 id="permission-update-entries">
  Permission update entries
</h4>

Il campo di output `updatedPermissions` e il campo di input [`permission_suggestions`](#permissionrequest-input) usano entrambi lo stesso array di oggetti di voce. Ogni voce ha un `type` che determina i suoi altri campi, e una `destination` che controlla dove viene scritta la modifica.

| `type`              | Fields                             | Effect                                                                                                                                                                                                                    |
| :------------------ | :--------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `addRules`          | `rules`, `behavior`, `destination` | Aggiunge regole di permesso. `rules` è un array di oggetti `{toolName, ruleContent?}`. Ometti `ruleContent` per corrispondere all'intero strumento. `behavior` è `"allow"`, `"deny"`, o `"ask"`                           |
| `replaceRules`      | `rules`, `behavior`, `destination` | Sostituisce tutte le regole del `behavior` dato alla `destination` con le `rules` fornite                                                                                                                                 |
| `removeRules`       | `rules`, `behavior`, `destination` | Rimuove le regole corrispondenti del `behavior` dato                                                                                                                                                                      |
| `setMode`           | `mode`, `destination`              | Cambia il modo di permesso. I modi validi sono `default`, `auto`, `acceptEdits`, `dontAsk`, `bypassPermissions`, `plan`, e `manual` come alias per `default`. L'alias `manual` richiede Claude Code v2.1.200 o successivo |
| `addDirectories`    | `directories`, `destination`       | Aggiunge directory di lavoro. `directories` è un array di stringhe di percorso                                                                                                                                            |
| `removeDirectories` | `directories`, `destination`       | Rimuove directory di lavoro                                                                                                                                                                                               |

<Note>
  `setMode` con `bypassPermissions` ha effetto solo se hai avviato la sessione con il modo bypass già disponibile: `--dangerously-skip-permissions`, `--permission-mode bypassPermissions`, `--allow-dangerously-skip-permissions`, o `permissions.defaultMode: "bypassPermissions"` nelle [impostazioni utente, `--settings`, o gestite](/docs/it/settings-reference#permissions-defaultmode). Altrimenti l'aggiornamento è un no-op. L'aggiornamento è anche un no-op quando [`permissions.disableBypassPermissionsMode`](/docs/it/permissions#managed-settings) disabilita il modo, o quando la sessione inizia in [restricted mode](/docs/it/cli-reference#cli-flags).

  `bypassPermissions` non viene mai persistito come `defaultMode` indipendentemente da `destination`.
</Note>

Il campo `destination` su ogni voce determina se la modifica rimane in memoria o persiste in un file di impostazioni.

| `destination`     | Writes to                                            |
| :---------------- | :--------------------------------------------------- |
| `session`         | solo in memoria, scartato quando la sessione termina |
| `localSettings`   | `.claude/settings.local.json`                        |
| `projectSettings` | `.claude/settings.json`                              |
| `userSettings`    | `~/.claude/settings.json`                            |

Un hook può ripetere uno dei `permission_suggestions` che ha ricevuto come suo proprio output `updatedPermissions`.

<h3 id="posttooluse">
  PostToolUse
</h3>

Si esegue immediatamente dopo che uno strumento si completa con successo.

Corrisponde al nome dello strumento, gli stessi valori di PreToolUse.

Corrisponde più ampiamente quando il nome dello strumento non è il filtro giusto:

* Per eseguire un hook dopo che qualsiasi strumento si completa con successo, ometti il `matcher` o impostalo su `"*"`. Il tuo hook può quindi scoprire cosa è cambiato da solo, ad esempio eseguendo `git status --porcelain`, che elenca anche i file non tracciati che `git diff` perde. Per le chiamate di strumenti che falliscono, aggiungi lo stesso hook sotto [PostToolUseFailure](#posttoolusefailure).
* Per eseguire un hook quando un file specifico cambia su disco, qualunque cosa l'abbia scritto, usa [FileChanged](#filechanged). Claude Code non esegue un hook `PostToolUse` che corrisponde a `Edit|Write` quando un comando `Bash` o un processo al di fuori di Claude Code riscrive lo stesso file.

<h4 id="posttooluse-input">
  PostToolUse input
</h4>

Gli hook `PostToolUse` si attivano dopo che uno strumento si è già eseguito con successo. L'input include sia `tool_input`, gli argomenti inviati allo strumento, che `tool_response`, il risultato che ha restituito. Lo schema esatto per entrambi dipende dallo strumento. I percorsi `tool_input` dello strumento di file arrivano nello stesso formato di [PreToolUse](#pretooluse-input): sempre assoluti, con i separatori nativi della piattaforma, quindi backslash su Windows. Per uno strumento MCP, l'input porta anche l'oggetto [`mcp_server`](#pretooluse-input).

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolUse",
  "tool_name": "Write",
  "tool_input": {
    "file_path": "/path/to/file.txt",
    "content": "file content"
  },
  "tool_response": {
    "filePath": "/path/to/file.txt",
    "type": "create"
  },
  "tool_use_id": "toolu_01ABC123...",
  "duration_ms": 12
}
```

| Field         | Description                                                                                                                                 |
| :------------ | :------------------------------------------------------------------------------------------------------------------------------------------ |
| `duration_ms` | Facoltativo. Tempo di esecuzione dello strumento in millisecondi. Esclude il tempo trascorso nei prompt di permesso e negli hook PreToolUse |

<h4 id="posttooluse-decision-control">
  PostToolUse decision control
</h4>

Gli hook `PostToolUse` possono fornire feedback a Claude dopo l'esecuzione dello strumento. Oltre ai [JSON output fields](#json-output) disponibili per tutti gli hook, il tuo script hook può restituire questi campi specifici dell'evento:

| Field                  | Description                                                                                                                                                                                                                                                                                                         |
| :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `decision`             | `"block"` aggiunge il `reason` accanto al risultato dello strumento. Claude vede comunque l'output originale; per sostituirlo, usa `updatedToolOutput`                                                                                                                                                              |
| `reason`               | Spiegazione mostrata a Claude quando `decision` è `"block"`                                                                                                                                                                                                                                                         |
| `additionalContext`    | Stringa aggiunta al contesto di Claude insieme al risultato dello strumento. Vedi [Add context for Claude](#add-context-for-claude)                                                                                                                                                                                 |
| `classifierContext`    | Breve nota su questo risultato della chiamata per il classificatore [auto mode](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) piuttosto che per Claude. Vedi [Annotate a result for the auto mode classifier](#annotate-a-result-for-the-auto-mode-classifier). Richiede Claude Code v2.1.236 o successivo |
| `updatedToolOutput`    | Sostituisce l'output dello strumento con il valore fornito prima che venga inviato a Claude. Il valore deve corrispondere alla forma di output dello strumento                                                                                                                                                      |
| `updatedMCPToolOutput` | Sostituisce l'output solo per gli [strumenti MCP](#match-mcp-tools). Preferisci `updatedToolOutput`, che funziona per tutti gli strumenti                                                                                                                                                                           |

L'esempio seguente sostituisce l'output di una chiamata `Bash`. Il valore di sostituzione corrisponde alla forma di output dello strumento `Bash`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "Additional information for Claude",
    "updatedToolOutput": {
      "stdout": "[redacted]",
      "stderr": "",
      "interrupted": false,
      "isImage": false
    }
  }
}
```

<Warning>
  `updatedToolOutput` cambia solo ciò che Claude vede. Lo strumento si è già eseguito nel momento in cui l'hook si attiva, quindi tutti i file scritti, i comandi eseguiti, o le richieste di rete inviate hanno già avuto effetto. La telemetria come gli span degli strumenti OpenTelemetry e gli eventi di analisi catturano anche l'output originale prima che l'hook si esegua. Per prevenire o modificare una chiamata di strumento prima che si esegua, usa un hook [PreToolUse](#pretooluse) invece.

  Il valore di sostituzione deve corrispondere alla forma di output dello strumento. Gli strumenti incorporati restituiscono oggetti strutturati piuttosto che stringhe semplici. Ad esempio, `Bash` restituisce un oggetto con campi `stdout`, `stderr`, `interrupted`, e `isImage`. Per gli strumenti incorporati, un valore che non corrisponde allo schema di output dello strumento viene ignorato e viene usato l'output originale. L'output dello strumento MCP viene passato senza convalida dello schema. Rimuovere i dettagli di errore di cui Claude ha bisogno può causare che proceda su un'ipotesi falsa.
</Warning>

<h4 id="annotate-a-result-for-the-auto-mode-classifier">
  Annotate a result for the auto mode classifier
</h4>

Restituisci `classifierContext` per inviare una breve nota sul risultato della chiamata dello strumento al classificatore [auto mode](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) piuttosto che a Claude. Il classificatore [non riceve mai i risultati degli strumenti stessi](/docs/it/permission-modes#how-the-classifier-evaluates-actions), quindi questo campo è il modo supportato per dirgli qualcosa su ciò che una chiamata ha restituito prima che riveda le azioni successive. Il campo richiede Claude Code v2.1.236 o successivo.

L'esempio seguente dice al classificatore da dove proviene l'output di una query:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "classifierContext": "This query ran against the staging database, not production."
  }
}
```

Quanto peso il classificatore dà alla nota dipende da dove hai configurato l'hook:

* **Hook configurati in Claude Code**: per gli hook da file di impostazioni, plugin, skill, e frontmatter dell'agente, il classificatore tratta la nota come contesto non verificato fornito dall'applicazione. La nota non stabilisce mai l'intento dell'utente, e se afferma che hai approvato o richiesto qualcosa, il classificatore controlla quella affermazione rispetto ai tuoi stessi messaggi nella conversazione
* **Callback Agent SDK in-process**: quando un'applicazione che incorpora Claude Code registra l'hook come un [callback TypeScript SDK](/docs/it/agent-sdk/hooks) e restituisce la nota durante la sessione dal vivo, il classificatore può pesare un'affermazione dell'utente inoltrata nella nota come intento dell'utente. Tale affermazione può soddisfare un requisito di consenso che il classificatore accetterebbe da un messaggio che invii, ma non solleva mai un blocco che il tuo stesso messaggio non potrebbe sollevare neanche. Dopo che una sessione riprende, Claude Code tratta le note ripristinate come contesto non verificato. Quando gli hook di entrambi i gruppi annotano la stessa chiamata, il classificatore tratta la nota combinata come non verificata

Claude Code applica questi limiti quando consegna la nota:

* **Lunghezza**: Claude Code limita le note per una chiamata di strumento a 2.000 caratteri e tronca il resto. Il limite è condiviso su ogni hook che risponde a quella chiamata
* **Solo risposte sincrone**: Claude Code ignora il campo nella risposta di un hook che [si esegue in background](#run-hooks-in-the-background), perché quella risposta arriva dopo che Claude Code registra il risultato dello strumento
* **Chiamate che il classificatore non registra**: la trascrizione del classificatore omette le ricerche di sola lettura come le letture di file e le ricerche. Claude Code scarta una nota allegata a una di quelle chiamate
* **Interazione con le riscritture**: quando la nota descrive l'output che stai sostituendo con `updatedToolOutput`, restituisci entrambi i campi nella stessa risposta dell'hook. Claude Code scarta la nota se quella riscrittura viene rifiutata o la riscrittura di un altro hook la sostituisce. Claude Code consegna una nota che restituisci senza una riscrittura anche quando un altro hook riscrive l'output

<Warning>
  Il classificatore legge il contenuto che metti in `classifierContext` come informazioni dall'applicazione che ospita la sessione, quindi non copiare l'output dello strumento non attendibile o il testo di terze parti in esso. Mantieni la nota a una breve affermazione su questa sola chiamata, come un fatto sulla sua origine o un'affermazione dell'utente su di essa; non usare il campo per consegnare messaggi non correlati o un flusso di eventi.
</Warning>

<h3 id="posttoolusefailure">
  PostToolUseFailure
</h3>

Si esegue quando uno strumento che ha iniziato a eseguirsi fallisce: lo strumento ha lanciato un errore, o uno strumento MCP ha restituito un risultato di errore. Usalo per registrare i fallimenti, inviare avvisi, o fornire feedback correttivo a Claude.

Corrisponde al nome dello strumento, gli stessi valori di PreToolUse.

<Note>
  Questo evento non si attiva per le chiamate di strumenti rifiutate prima dell'esecuzione: un nome di strumento sconosciuto, input che fallisce la convalida dello schema o specifica dello strumento, o una negazione di permesso. I rifiuti di convalida vengono restituiti come risultati `tool_use_error` e accadono prima che gli hook si eseguano, quindi non attivano né `PreToolUse` né `PostToolUseFailure`. Le negazioni di permesso attivano `PreToolUse` ma non questo evento; vedi [PermissionDenied](#permissiondenied).
</Note>

<h4 id="posttoolusefailure-input">
  PostToolUseFailure input
</h4>

Gli hook PostToolUseFailure ricevono gli stessi campi `tool_name` e `tool_input` di PostToolUse, insieme alle informazioni di errore come campi di livello superiore. Per uno strumento MCP, ricevono anche l'oggetto [`mcp_server`](#pretooluse-input). Ad esempio, un comando `npm test` fallito potrebbe consegnare:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolUseFailure",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test",
    "description": "Run test suite"
  },
  "tool_use_id": "toolu_01ABC123...",
  "error": "Exit code 1\nError: Cannot find module 'express'",
  "is_interrupt": false,
  "duration_ms": 4187
}
```

| Field          | Description                                                                                                                                                                                                                                                                                            |
| :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `error`        | Stringa che descrive cosa è andato storto. Il formato dipende dallo strumento che ha fallito                                                                                                                                                                                                           |
| `is_interrupt` | Booleano facoltativo. True quando il fallimento ha raggiunto Claude Code come un'interruzione piuttosto che come un errore che lo strumento ha segnalato. L'annullamento di uno strumento in esecuzione non attiva questo hook; il risultato dello strumento porta il messaggio di interruzione invece |
| `duration_ms`  | Facoltativo. Tempo di esecuzione dello strumento in millisecondi. Esclude il tempo trascorso nei prompt di permesso e negli hook PreToolUse                                                                                                                                                            |

La stringa `error` è generalmente lo stesso testo che Claude riceve come risultato dello strumento fallito. Il suo formato varia per strumento e fallimento. Chiavi il tuo hook su `tool_name`, `is_interrupt`, e la prima riga `Exit code N`; tratta il resto della stringa come testo di visualizzazione, non come un formato stabile.

* Per Bash e PowerShell, un comando che si è eseguito e ha uscito produce una prima riga `Exit code N`, quindi qualsiasi output che il comando ha prodotto come un blocco con stdout e stderr intercalati
* Un payload può anche portare un messaggio di fallimento nudo senza una riga di codice di uscita, quando Claude Code non potrebbe avviare il processo shell stesso
* Claude Code tronca nel mezzo le stringhe lunghe intorno a un marcatore `... [N characters truncated] ...`, e può inserire righe proprie, come `Command timed out after 2m 0s`

<h4 id="posttoolusefailure-decision-control">
  PostToolUseFailure decision control
</h4>

Gli hook `PostToolUseFailure` possono fornire contesto a Claude dopo un fallimento dello strumento. Oltre ai [JSON output fields](#json-output) disponibili per tutti gli hook, il tuo script hook può restituire questi campi specifici dell'evento:

| Field               | Description                                                                                                       |
| :------------------ | :---------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | Stringa aggiunta al contesto di Claude insieme all'errore. Vedi [Add context for Claude](#add-context-for-claude) |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUseFailure",
    "additionalContext": "Additional information about the failure for Claude"
  }
}
```

<h3 id="posttoolbatch">
  PostToolBatch
</h3>

Si esegue una volta dopo che ogni chiamata di strumento in un batch si è risolta, prima che Claude Code invii la richiesta successiva al modello. `PostToolUse` si attiva una volta per strumento, il che significa che si attiva contemporaneamente quando Claude effettua chiamate di strumenti parallele. `PostToolBatch` si attiva esattamente una volta con il batch completo, quindi è il posto giusto per iniettare contesto che dipende dall'insieme di strumenti che si sono eseguiti piuttosto che da qualsiasi singolo strumento. Non c'è matcher per questo evento.

<h4 id="posttoolbatch-input">
  PostToolBatch input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook PostToolBatch ricevono `tool_calls`, un array che descrive ogni chiamata di strumento nel batch:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolBatch",
  "tool_calls": [
    {
      "tool_name": "Read",
      "tool_input": {"file_path": "/.../ledger/accounts.py"},
      "tool_use_id": "toolu_01...",
      "tool_response": "     1\tfrom __future__ import annotations\n     2\t..."
    },
    {
      "tool_name": "Read",
      "tool_input": {"file_path": "/.../ledger/transactions.py"},
      "tool_use_id": "toolu_02...",
      "tool_response": "     1\tfrom __future__ import annotations\n     2\t..."
    }
  ]
}
```

`tool_response` contiene lo stesso contenuto che il modello riceve nel blocco `tool_result` corrispondente. Il valore è una stringa serializzata o un array di blocchi di contenuto, esattamente come lo strumento l'ha emesso. Per `Read`, ciò significa testo con prefisso numero di riga piuttosto che contenuti di file grezzi. Le risposte possono essere grandi, quindi analizza solo i campi di cui hai bisogno.

<Note>
  La forma `tool_response` differisce da quella di `PostToolUse`. `PostToolUse` passa l'oggetto `Output` strutturato dello strumento, come `{filePath: "...", type: "create"}` per `Write`; `PostToolBatch` passa il contenuto `tool_result` serializzato che il modello vede.
</Note>

<h4 id="posttoolbatch-decision-control">
  PostToolBatch decision control
</h4>

Gli hook `PostToolBatch` possono iniettare contesto per Claude. Oltre ai [JSON output fields](#json-output) disponibili per tutti gli hook, il tuo script hook può restituire questi campi specifici dell'evento:

| Field               | Description                                                                                                                                                                                                                                 |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `additionalContext` | Stringa di contesto iniettata una volta prima della prossima chiamata del modello. Vedi [Add context for Claude](#add-context-for-claude) per i dettagli di consegna, cosa metterci, e come le sessioni riprese gestiscono i valori passati |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolBatch",
    "additionalContext": "These files are part of the ledger module. Run pytest before marking the task complete."
  }
}
```

Restituire `decision: "block"` o `continue: false` ferma il loop agentico prima della prossima chiamata del modello. Il messaggio di blocco viene dal `reason` JSON o `stopReason`, o da stderr all'uscita 2. Lo vedi come un avviso nella trascrizione, e rimane nella conversazione, quindi Claude lo vede quando la conversazione continua.

<h3 id="permissiondenied">
  PermissionDenied
</h3>

Si esegue quando [auto mode](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) nega una chiamata di strumento, incluso quando nega senza un verdetto del classificatore perché [un controllo di sicurezza separato da auto mode ha rifiutato la richiesta del classificatore stesso](/docs/it/errors#auto-mode-cannot-determine-the-safety-of-an-action) o la sua risposta non ha analizzato. Questo hook si attiva solo in auto mode: non si esegue quando neghi manualmente un dialogo di permesso, quando un hook `PreToolUse` blocca una chiamata, o quando una regola `deny` corrisponde. Usalo per registrare le negazioni, regolare la configurazione, o dire al modello che potrebbe riprovare la chiamata dello strumento.

Corrisponde al nome dello strumento, gli stessi valori di PreToolUse.

<h4 id="permissiondenied-input">
  PermissionDenied input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook PermissionDenied ricevono `tool_name`, `tool_input`, `tool_use_id`, e `reason`. Per uno strumento MCP, ricevono anche l'oggetto [`mcp_server`](#pretooluse-input).

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "auto",
  "hook_event_name": "PermissionDenied",
  "tool_name": "Bash",
  "tool_input": {
    "command": "rm -rf /tmp/build",
    "description": "Clean build directory"
  },
  "tool_use_id": "toolu_01ABC123...",
  "reason": "[Irreversible Local Destruction]"
}
```

| Field    | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `reason` | Il motivo della negazione. Per un verdetto del classificatore, nella maggior parte delle sessioni nomina la regola corrispondente tra parentesi quadre, come `[Data Exfiltration]`; vedi [Review denials](/docs/it/auto-mode-config#review-denials) per le altre forme. Per una [negazione senza verdetto](#permissiondenied-decision-control), inizia con `Auto mode could not evaluate this action and is blocking it for safety`. Per una negazione perché il modello del classificatore non era disponibile, è il testo fisso `Classifier unavailable` |

<h4 id="permissiondenied-decision-control">
  PermissionDenied decision control
</h4>

Gli hook PermissionDenied possono dire al modello che potrebbe riprovare la chiamata dello strumento negato. Restituisci un oggetto JSON con `hookSpecificOutput.retry` impostato su `true`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionDenied",
    "retry": true
  }
}
```

Quando `retry` è `true`, Claude Code aggiunge un messaggio alla conversazione dicendo al modello che potrebbe riprovare la chiamata dello strumento. Claude Code non inverte la negazione stessa. Se il tuo hook non restituisce JSON, o restituisce `retry: false`, la negazione rimane e il modello riceve il messaggio di rifiuto originale.

Claude Code ignora `retry: true` quando il classificatore ha prodotto [nessun verdetto sull'azione](/docs/it/errors#auto-mode-cannot-determine-the-safety-of-an-action): la sua risposta non ha analizzato, o un controllo di sicurezza separato da auto mode ha rifiutato la richiesta del classificatore. Per quelle negazioni, Claude Code già dice al modello nel messaggio di rifiuto se riprovare più tardi o procedere.

<h3 id="notification">
  Notification
</h3>

Si esegue quando Claude Code invia notifiche. Corrisponde al tipo di notifica. Ometti il matcher per eseguire gli hook per tutti i tipi di notifica.

Ricevi questi eventi hook anche con le notifiche desktop disattivate: l'impostazione `preferredNotifChannel`, incluso `notifications_disabled`, cambia solo come sei avvisato, non se il tuo hook si esegue.

| Matcher                      | Quando si attiva                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :--------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permission_prompt`          | Claude ha bisogno della tua approvazione per usare uno strumento o la [richiesta di rete](/docs/it/sandboxing#network-isolation) di un comando sandboxed, e il prompt ha atteso circa sei secondi                                                                                                                                                                                                                                                                                                                                  |
| `idle_prompt`                | Claude ha finito di rispondere circa 60 secondi fa e non hai digitato da allora                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `auth_success`               | L'autenticazione si completa                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `elicitation_dialog`         | Un server MCP apre un modulo di elicitazione e non hai digitato per circa sei secondi                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `elicitation_url_dialog`     | Un server MCP ti chiede di aprire un URL del browser e non hai digitato per circa sei secondi                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `elicitation_complete`       | Un server MCP segnala che un'[elicitazione in modalità URL](#elicitation-input) è completa                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `elicitation_response`       | Una risposta di elicitazione MCP viene inviata di nuovo al server                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `agent_needs_input`          | Una sessione in background inizia ad aspettare il tuo input mentre la [agent view](/docs/it/agent-view) è aperta in un terminale, o la sessione corrente ti chiede una domanda di configurazione del terminale del [teammate del team dell'agente](/docs/it/agent-teams#choose-a-display-mode) e non hai digitato per circa sei secondi                                                                                                                                                                                                 |
| `agent_completed`            | Una sessione in background finisce o fallisce. Si attiva solo mentre la [agent view](/docs/it/agent-view) è aperta in un terminale                                                                                                                                                                                                                                                                                                                                                                                                 |
| `quota_auto_resume_fired`    | Claude Code continua il tuo compito dopo che un limite di utilizzo di claude.ai l'ha messo in pausa: al reset, o prima quando qualcosa che fai in Claude Code durante l'attesa, come aggiungere crediti di utilizzo, aggiornare il tuo piano, o cambiare modelli, rende l'utilizzo disponibile di nuovo, con l'[eccezione dell'impostazione del modello](/docs/it/interactive-mode#wait-for-a-usage-limit-to-reset)                                                                                                                |
| `quota_auto_resume_stale`    | Un limite di utilizzo di claude.ai si è ripristinato mentre il tuo computer dormiva per più di circa 30 minuti. Claude Code attende che tu prema `Enter` invece di continuare. Dopo un sonno più breve continua e attiva `quota_auto_resume_fired` invece                                                                                                                                                                                                                                                                     |
| `quota_auto_resume_disabled` | Claude Code termina la sua attesa per un limite di utilizzo di claude.ai senza continuare il tuo compito: [`autoContinueAtUsageLimit`](/docs/it/settings-reference#autocontinueatusagelimit) si è spento o il reset si è spostato più di 24 ore lontano durante un'attesa che Claude Code ha avviato da solo, il compito continuato ha continuato a colpire il limite, o la continuazione è stata bloccata prima di raggiungere il modello. Non si attiva quando premi `Esc` o `Ctrl+C`, o scegli **Don't continue automatically** |

I tipi `agent_needs_input` e `agent_completed` richiedono Claude Code v2.1.198 o successivo.

I tipi `quota_auto_resume_fired`, `quota_auto_resume_stale`, e `quota_auto_resume_disabled` richiedono Claude Code v2.1.234 o successivo.

Nelle sessioni di terminale, `permission_prompt` per la richiesta di rete di un comando sandboxed richiede Claude Code v2.1.246 o successivo.

`agent_needs_input` per una domanda di configurazione del terminale del teammate richiede Claude Code v2.1.248 o successivo.

<Note>
  I tipi `permission_prompt`, `idle_prompt`, `elicitation_dialog`, e `elicitation_url_dialog` condividono il loro timing con le notifiche desktop, quindi nelle sessioni di terminale li vedi solo quando sembri essere lontano dal terminale:

  * Aspettati `permission_prompt` una volta che non hai digitato per circa sei secondi. Il timer inizia quando il prompt di permesso appare, e ogni pressione di tasto lo rinvia. Per eseguire un hook immediatamente quando Claude chiede il permesso di usare uno strumento, usa [PermissionRequest](#permissionrequest) invece.
  * Aspettati `idle_prompt` circa 60 secondi dopo che Claude finisce di rispondere, e solo se non hai digitato da allora. Claude Code non invia `idle_prompt` mentre attende che un limite di utilizzo di claude.ai si ripristini. Quando l'attesa termina da sola, uno dei tipi `quota_auto_resume_*` si attiva invece.
  * Aspettati `elicitation_dialog` per un modulo di elicitazione, o `elicitation_url_dialog` per una richiesta di URL del browser, una volta che non hai digitato per circa sei secondi. Entrambi condividono lo stesso gate di sei secondi di `permission_prompt`: il timer inizia quando il dialogo appare, e ogni pressione di tasto lo rinvia.

  Una richiesta di permesso o elicitazione che arriva mentre un altro dialogo è sullo schermo mantiene lo stesso gate di sei secondi, cronometrato da quando la richiesta arriva. La sua notifica può raggiungerti mentre la richiesta attende ancora dietro il dialogo aperto.
</Note>

Claude Code cronometra `permission_prompt` diversamente nelle sessioni in cui invia richieste di permesso al callback [`canUseTool`](/docs/it/agent-sdk/user-input) di Agent SDK, che è come Claude Desktop e l'estensione VS Code ospitano Claude Code:

* Aspettati `permission_prompt` circa sei secondi dopo che Claude chiede il permesso. Claude Code non lo rinvia mentre digiti.
* Se tu o un hook [PermissionRequest](#permissionrequest) rispondete prima, Claude Code non esegue `permission_prompt`.
* Imposta [`CLAUDE_CODE_DISABLE_PERMISSION_PROMPT_NOTIFY_HOOKS`](/docs/it/env-vars) su `1` per disattivare `permission_prompt` in queste sessioni.

Prima della v2.1.233, `permission_prompt` non si attivava in queste sessioni.

Usa matcher separati per eseguire diversi gestori a seconda del tipo di notifica. Questa configurazione attiva uno script di avviso specifico del permesso quando Claude ha bisogno dell'approvazione del permesso e una notifica diversa quando Claude è stato inattivo:

```json theme={null}
{
  "hooks": {
    "Notification": [
      {
        "matcher": "permission_prompt",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/permission-alert.sh"
          }
        ]
      },
      {
        "matcher": "idle_prompt",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/idle-notification.sh"
          }
        ]
      }
    ]
  }
}
```

<h4 id="notification-input">
  Notification input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook Notification ricevono `message` con il testo della notifica, un `title` facoltativo, e `notification_type` che indica quale tipo si è attivato.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Notification",
  "message": "Claude needs your permission",
  "title": "Permission needed",
  "notification_type": "permission_prompt"
}
```

Gli hook Notification non possono bloccare o modificare le notifiche. Claude Code scarta i loro campi `systemMessage` e `continue` ma emette comunque [`terminalSequence`](#emit-terminal-notifications), su cui si basa l'esempio di notifica desktop. Gli hook Notification sono destinati agli effetti collaterali come l'inoltro della notifica a un servizio esterno.

<h3 id="subagentstart">
  SubagentStart
</h3>

Si esegue quando Claude genera un subagent con lo strumento Agent, quando Claude [riprende un subagent](/docs/it/sub-agents#resume-subagents), e ogni volta che un [team dell'agente](/docs/it/agent-teams) in-process teammate gestisce un nuovo messaggio. Supporta i matcher per filtrare per nome del tipo di agente. Per gli agenti incorporati, questo è il nome dell'agente come `general-purpose`, `Explore`, o `Plan`. Per i [subagent personalizzati](/docs/it/sub-agents), questo è il campo `name` dal frontmatter dell'agente, non il nome del file.

Per i subagent spediti da un [plugin](/docs/it/plugins), il tipo di agente è l'identificatore con ambito del plugin come `my-plugin:reviewer`, non il nome del frontmatter nudo. I due punti mettono un nome con ambito del plugin sul percorso dell'espressione regolare, quindi ancora il matcher con `^` e `$` per una corrispondenza esatta: `^my-plugin:reviewer$`.

<h4 id="subagentstart-input">
  SubagentStart input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook SubagentStart ricevono `agent_id` con l'identificatore univoco per il subagent e `agent_type` con il nome dell'agente che il matcher filtra.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SubagentStart",
  "agent_id": "agent-abc123",
  "agent_type": "Explore"
}
```

Gli hook SubagentStart non possono bloccare la creazione del subagent, ma possono iniettare contesto nel subagent. Oltre ai [JSON output fields](#json-output) disponibili per tutti gli hook, puoi restituire:

| Field               | Description                                                                                                                                                      |
| :------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | Stringa aggiunta al contesto del subagent all'inizio della sua conversazione, prima del suo primo prompt. Vedi [Add context for Claude](#add-context-for-claude) |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "SubagentStart",
    "additionalContext": "Follow security guidelines for this task"
  }
}
```

Quando l'hook si esegue di nuovo per lo stesso subagent, Claude Code inietta il contesto restituito solo quando il contesto del subagent non contiene già la copia da un'esecuzione precedente. La copia iniettata al lancio rimane in posizione, lasciando intatta la [prompt cache](/docs/it/prompt-caching#subagents-and-the-cache) del subagent. Dopo che la [auto-compaction](/docs/it/sub-agents#auto-compaction) scarta quella copia, Claude Code inietta il contesto dell'esecuzione successiva di nuovo.

<h3 id="subagentstop">
  SubagentStop
</h3>

Si esegue quando un subagent di Claude Code ha finito di rispondere. Corrisponde al tipo di agente, gli stessi valori di SubagentStart.

<h4 id="subagentstop-input">
  SubagentStop input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook SubagentStop ricevono `stop_hook_active`, `agent_id`, `agent_type`, `agent_transcript_path`, e `last_assistant_message`. Il campo `agent_type` è il valore usato per il filtraggio del matcher. Il `transcript_path` è la trascrizione della sessione principale, mentre `agent_transcript_path` è la trascrizione propria del subagent archiviata in una cartella `subagents/` annidato. Il campo `last_assistant_message` contiene il contenuto di testo della risposta finale del subagent, quindi gli hook possono accedervi senza analizzare il file della trascrizione.

Non ogni evento SubagentStop proviene da un subagent che Claude ha generato. Claude Code esegue anche agenti interni per alcune delle sue stesse funzioni, come i [suggerimenti di prompt](/docs/it/interactive-mode#prompt-suggestions) e le [domande laterali `/btw`](/docs/it/interactive-mode#side-questions-with-%2Fbtw), e SubagentStop si attiva quando uno di quelli finisce anche. Per questi eventi, `agent_type` è il nome dell'agente che la sessione stessa esegue, come uno impostato con [`--agent`](/docs/it/cli-reference#cli-flags) o l'impostazione [`agent`](/docs/it/settings-reference#agent), e una stringa vuota quando la sessione si esegue senza uno.

Un `matcher` che nomina i tipi di agente non corrisponde a un `agent_type` vuoto. Un hook il cui matcher è omesso, `""`, o `"*"`, o è un'espressione regolare che corrisponde a una stringa vuota, si esegue anche per gli eventi con un `agent_type` vuoto.

Su Claude Code v2.1.271 o successivo, un subagent che si esegue con lo strumento [`SubagentHandback`](/docs/it/tools-reference) consegna il suo rapporto attraverso quello strumento prima che si fermi. Il campo `last_assistant_message` porta quindi il testo di chiusura del subagent, se presente, che non è il rapporto consegnato. Il rapporto è l'input `message` di quella chiamata, che un hook `PreToolUse` o `PostToolUse` abbinato su `SubagentHandback` riceve come `tool_input.message`.

Gli hook SubagentStop ricevono anche gli array `background_tasks` e `session_crons` descritti sotto [Stop input](#stop-input). Entrambi gli array hanno ambito alla sessione genitore, non al subagent.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "~/.claude/projects/.../abc123.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "SubagentStop",
  "stop_hook_active": false,
  "agent_id": "def456",
  "agent_type": "Explore",
  "agent_transcript_path": "~/.claude/projects/.../abc123/subagents/agent-def456.jsonl",
  "last_assistant_message": "Analysis complete. Found 3 potential issues...",
  "background_tasks": [],
  "session_crons": []
}
```

Gli hook SubagentStop usano lo stesso formato di controllo delle decisioni degli hook [Stop](#stop-decision-control), incluso `hookSpecificOutput.additionalContext` con `hookEventName` impostato su `"SubagentStop"`, per il feedback non di errore che mantiene il subagent in esecuzione. Restituire `decision: "block"` con un `reason` mantiene il subagent in esecuzione e consegna `reason` al subagent come sua prossima istruzione. Un hook che blocca uscendo con 2 consegna il suo messaggio stderr nello stesso modo. Per iniettare contesto nella sessione genitore dopo che un subagent ritorna, usa un hook [`PostToolUse`](#posttooluse) sullo strumento `Agent` invece.

<h3 id="taskcreated">
  TaskCreated
</h3>

Si esegue quando un compito viene creato tramite lo strumento `TaskCreate`. Usalo per applicare convenzioni di denominazione, richiedere descrizioni di compiti, o prevenire la creazione di determinati compiti. In una [sessione senza gli strumenti Task](/docs/it/tools-reference#task-tool-availability), questo evento non si attiva.

Gli hook TaskCreated non supportano i matcher e si attivano su ogni occorrenza.

<h4 id="taskcreated-input">
  TaskCreated input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook TaskCreated ricevono `task_id`, `task_subject`, e opzionalmente `task_description`, `teammate_name`, e `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "TaskCreated",
  "task_id": "task-001",
  "task_subject": "Implement user authentication",
  "task_description": "Add login and signup endpoints",
  "teammate_name": "implementer",
  "team_name": "session-a1b2c3d4"
}
```

| Field              | Description                                                                            |
| :----------------- | :------------------------------------------------------------------------------------- |
| `task_id`          | Identificatore del compito che viene creato                                            |
| `task_subject`     | Titolo del compito                                                                     |
| `task_description` | Descrizione dettagliata del compito. Potrebbe essere assente                           |
| `teammate_name`    | Nome del teammate che crea il compito. Potrebbe essere assente                         |
| `team_name`        | Deprecato. Nome del team derivato dalla sessione; verrà rimosso in una versione futura |

<h4 id="taskcreated-decision-control">
  TaskCreated decision control
</h4>

Un hook TaskCreated può bloccare la creazione in due modi. In entrambi i casi, Claude Code elimina il compito e restituisce il tuo messaggio a Claude come errore dello strumento. Claude Code ignora `continue: false` da questo evento e Claude continua a lavorare.

* **Codice di uscita 2**: Claude Code restituisce il testo stderr come messaggio.
* **JSON `{"decision": "block", "reason": "..."}`**: Claude Code restituisce `reason` come messaggio.

Questo esempio blocca i compiti i cui soggetti non seguono il formato richiesto:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
TASK_SUBJECT=$(echo "$INPUT" | jq -r '.task_subject')

if [[ ! "$TASK_SUBJECT" =~ ^\[TICKET-[0-9]+\] ]]; then
  echo "Task subject must start with a ticket number, e.g. '[TICKET-123] Add feature'" >&2
  exit 2
fi

exit 0
```

<h3 id="taskcompleted">
  TaskCompleted
</h3>

Si esegue quando un compito viene contrassegnato come completato. Questo si attiva in due situazioni: quando qualsiasi agente contrassegna esplicitamente un compito come completato tramite lo strumento TaskUpdate, o quando un [team dell'agente](/docs/it/agent-teams) teammate finisce il suo turno con compiti in corso. Usalo per applicare criteri di completamento come il passaggio dei test o dei controlli di lint prima che un compito possa chiudersi.

Gli hook TaskCompleted non supportano i matcher e si attivano su ogni occorrenza.

<h4 id="taskcompleted-input">
  TaskCompleted input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook TaskCompleted ricevono `task_id`, `task_subject`, e opzionalmente `task_description`, `teammate_name`, e `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "TaskCompleted",
  "task_id": "task-001",
  "task_subject": "Implement user authentication",
  "task_description": "Add login and signup endpoints",
  "teammate_name": "implementer",
  "team_name": "session-a1b2c3d4"
}
```

| Field              | Description                                                                            |
| :----------------- | :------------------------------------------------------------------------------------- |
| `task_id`          | Identificatore del compito che viene completato                                        |
| `task_subject`     | Titolo del compito                                                                     |
| `task_description` | Descrizione dettagliata del compito. Potrebbe essere assente                           |
| `teammate_name`    | Nome del teammate che completa il compito. Potrebbe essere assente                     |
| `team_name`        | Deprecato. Nome del team derivato dalla sessione; verrà rimosso in una versione futura |

<h4 id="taskcompleted-decision-control">
  TaskCompleted decision control
</h4>

Gli hook TaskCompleted supportano due modi per controllare il completamento del compito:

* **Codice di uscita 2**: il compito non viene contrassegnato come completato e il messaggio stderr viene restituito al modello come feedback.
* **JSON `{"continue": false, "stopReason": "..."}`**: quando un teammate che finisce il suo turno ha attivato l'evento, ferma completamente il teammate, corrispondendo al comportamento dell'hook `Stop`. Il `stopReason` viene mostrato all'utente. Quando lo strumento `TaskUpdate` ha attivato l'evento, Claude Code ignora `continue: false`; il codice di uscita 2 blocca comunque il completamento.

Questo esempio esegue i test e blocca il completamento del compito se falliscono:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
TASK_SUBJECT=$(echo "$INPUT" | jq -r '.task_subject')

# Run the test suite
if ! npm test 2>&1; then
  echo "Tests not passing. Fix failing tests before completing: $TASK_SUBJECT" >&2
  exit 2
fi

exit 0
```

<h3 id="stop">
  Stop
</h3>

Si esegue quando l'agente Claude Code principale ha finito di rispondere. Non si esegue se l'arresto si è verificato a causa di un'interruzione dell'utente. Gli errori API attivano [StopFailure](#stopfailure) invece.

<Tip>
  Il comando [`/goal`](/docs/it/goal) è una scorciatoia incorporata per un hook Stop basato su prompt con ambito di sessione. Usalo quando vuoi che Claude continui a lavorare verso una condizione senza scrivere la configurazione dell'hook.
</Tip>

<h4 id="stop-input">
  Stop input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook Stop ricevono `stop_hook_active`, `last_assistant_message`, `background_tasks`, e `session_crons`. Il campo `stop_hook_active` è `true` quando Claude Code sta già continuando come risultato di un hook stop. Controlla questo valore o elabora la trascrizione per evitare di bloccarsi su una condizione che non si risolverà mai. Claude Code sovrascrive l'hook e termina il turno dopo 8 blocchi consecutivi.

Il campo `last_assistant_message` contiene il contenuto di testo della risposta finale di Claude, quindi gli hook possono accedervi senza analizzare il file della trascrizione. Per gli hook che agiscono sul turno appena completato, come gli hook di lettura ad alta voce o notifica, usa questo campo piuttosto che leggere `transcript_path`: il file della trascrizione non è garantito che includa il messaggio finale al momento di Stop su tutte le versioni.

Gli array `background_tasks` e `session_crons` consentono agli hook di distinguere "la sessione è finita" da "la sessione è in pausa in attesa che il lavoro in background la risvegli di nuovo". Entrambi gli array sono presenti quando il registro dei compiti è raggiungibile e sono vuoti quando nulla è in volo o programmato.

Ogni voce in `background_tasks` descrive un compito in volo e usa questi campi:

| Field         | Description                                                                                                                                                                                                                                                                    |
| :------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`          | Identificatore del compito                                                                                                                                                                                                                                                     |
| `type`        | Etichetta del tipo di compito amichevole come `shell`, `subagent`, `monitor`, `workflow`, `teammate`, `cloud session`, o `MCP task`. Ogni etichetta identifica quale funzione di Claude Code ha creato il compito. Ritorna al discriminante grezzo per i tipi non riconosciuti |
| `status`      | Stato del compito corrente                                                                                                                                                                                                                                                     |
| `description` | Descrizione in testo libero, limitata a 1000 caratteri con un marcatore `… [+N chars]` in-stringa quando ritagliato                                                                                                                                                            |
| `command`     | Riga di comando shell, limitata a 1000 caratteri. Presente solo per i compiti `shell`                                                                                                                                                                                          |
| `agent_type`  | Nome del tipo di subagent. Presente solo per i compiti `subagent`                                                                                                                                                                                                              |
| `server`      | Nome del server MCP. Presente solo per i compiti `monitor` e `MCP task`                                                                                                                                                                                                        |
| `tool`        | Nome dello strumento MCP. Presente solo per i compiti `monitor` e `MCP task`                                                                                                                                                                                                   |
| `name`        | Nome del workflow. Presente solo per i compiti `workflow`                                                                                                                                                                                                                      |

Ogni voce in `session_crons` descrive un risveglio programmato con ambito di sessione, proveniente da `CronCreate`, `ScheduleWakeup`, e `/loop`:

| Field       | Description                                                                                                                                                 |
| :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`        | Identificatore del compito cron                                                                                                                             |
| `schedule`  | Espressione cron, ad esempio `0 9 * * 1-5`                                                                                                                  |
| `recurring` | `false` per i risvegli una tantum il cui programma codifica un sing olo tempo di attivazione, `true` per i compiti che si riattivano su ogni corrispondenza |
| `prompt`    | Prompt inviato quando il cron si attiva, limitato a 1000 caratteri con lo stesso marcatore `… [+N chars]`                                                   |

Questo esempio mostra un input Stop con un compito shell in volo e un cron ricorrente:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "~/.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "Stop",
  "stop_hook_active": true,
  "last_assistant_message": "I've completed the refactoring. Here's a summary...",
  "background_tasks": [
    {
      "id": "task-001",
      "type": "shell",
      "status": "running",
      "description": "tail logs",
      "command": "tail -f /var/log/syslog"
    }
  ],
  "session_crons": [
    {
      "id": "cron-001",
      "schedule": "0 9 * * 1-5",
      "recurring": true,
      "prompt": "check the build"
    }
  ]
}
```

<h4 id="stop-decision-control">
  Stop decision control
</h4>

Gli hook `Stop` e `SubagentStop` possono controllare se Claude continua. Oltre ai [JSON output fields](#json-output) disponibili per tutti gli hook, il tuo script hook può restituire questi campi specifici dell'evento:

| Field                                  | Description                                                                                                                                                                                                                                  |
| :------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `decision`                             | `"block"` impedisce a Claude di fermarsi. Ometti per consentire a Claude di fermarsi                                                                                                                                                         |
| `reason`                               | Richiesto quando `decision` è `"block"`. Dice a Claude perché dovrebbe continuare                                                                                                                                                            |
| `hookSpecificOutput.additionalContext` | Feedback non di errore per Claude. La conversazione continua in modo che Claude possa agire su di esso, ma a differenza di `decision: "block"` viene mostrato nella trascrizione come feedback dell'hook piuttosto che come errore dell'hook |

Un hook che blocca uscendo con 2 si instrada nello stesso modo di `reason`: Claude riceve il messaggio stderr come spiegazione per cui dovrebbe continuare.

```json theme={null}
{
  "decision": "block",
  "reason": "Must be provided when Claude is blocked from stopping"
}
```

Usa `additionalContext` quando l'hook funziona come progettato e dà a Claude una guida, come "esegui la suite di test prima di finire". Mantiene la conversazione attraverso gli stessi loop protections di `decision: "block"`, vale a dire l'input `stop_hook_active` e il limite di continuazione di 8 consecutivi, ma la trascrizione lo etichetta come `Stop hook feedback` e nessuna notifica di errore dell'hook viene mostrata:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "Stop",
    "additionalContext": "Please run the test suite before finishing"
  }
}
```

<h3 id="stopfailure">
  StopFailure
</h3>

Si esegue invece di [Stop](#stop) quando il turno termina a causa di un errore API. Claude Code ignora l'output e il codice di uscita dell'hook, a parte [`terminalSequence`](#emit-terminal-notifications). Usalo per registrare i fallimenti, inviare avvisi, o intraprendere azioni di recupero quando Claude non può completare una risposta a causa di limiti di velocità, problemi di autenticazione, o altri errori API.

<h4 id="stopfailure-input">
  StopFailure input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook StopFailure ricevono `error`, `error_details` facoltativo, e `last_assistant_message` facoltativo. Il campo `error` identifica il tipo di errore ed è usato per il filtraggio del matcher.

| Field                    | Description                                                                                                                                                                                                                                                              |
| :----------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `error`                  | Tipo di errore: `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `account_on_hold`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `cloud_credential_error`, o `unknown`                       |
| `error_details`          | Dettagli aggiuntivi sull'errore, quando disponibili                                                                                                                                                                                                                      |
| `last_assistant_message` | Il testo di errore renderizzato mostrato nella conversazione. A differenza di `Stop` e `SubagentStop`, dove questo campo contiene l'output conversazionale di Claude, per `StopFailure` contiene la stringa di errore API stessa, come `"API Error: Rate limit reached"` |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "StopFailure",
  "error": "rate_limit",
  "error_details": "429 Too Many Requests",
  "last_assistant_message": "API Error: Rate limit reached"
}
```

Gli hook StopFailure non hanno controllo delle decisioni. Si eseguono solo per scopi di notifica e logging.

<h3 id="teammateidle">
  TeammateIdle
</h3>

Si esegue quando un [team dell'agente](/docs/it/agent-teams) teammate sta per andare inattivo dopo aver finito il suo turno. Usalo per applicare gate di qualità prima che un teammate smetta di lavorare, come richiedere il passaggio dei controlli di lint o verificare che i file di output esistano.

Gli hook TeammateIdle non supportano i matcher e si attivano su ogni occorrenza.

<h4 id="teammateidle-input">
  TeammateIdle input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook TeammateIdle ricevono `teammate_name` e `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "TeammateIdle",
  "teammate_name": "researcher",
  "team_name": "session-a1b2c3d4"
}
```

| Field           | Description                                                                            |
| :-------------- | :------------------------------------------------------------------------------------- |
| `teammate_name` | Nome del teammate che sta per andare inattivo                                          |
| `team_name`     | Deprecato. Nome del team derivato dalla sessione; verrà rimosso in una versione futura |

<h4 id="teammateidle-decision-control">
  TeammateIdle decision control
</h4>

Gli hook TeammateIdle supportano due modi per controllare il comportamento del teammate:

* **Codice di uscita 2**: il teammate riceve il messaggio stderr come feedback e continua a lavorare invece di andare inattivo.
* **JSON `{"continue": false, "stopReason": "..."}`**: ferma completamente il teammate, corrispondendo al comportamento dell'hook `Stop`. Il `stopReason` viene mostrato all'utente.

Questo esempio controlla che un artefatto di build esista prima di consentire a un teammate di andare inattivo:

```bash theme={null}
#!/bin/bash

if [ ! -f "./dist/output.js" ]; then
  echo "Build artifact missing. Run the build before stopping." >&2
  exit 2
fi

exit 0
```

<h3 id="configchange">
  ConfigChange
</h3>

Si esegue quando un file di configurazione cambia durante una sessione. Usalo per controllare i cambiamenti delle impostazioni, applicare politiche di sicurezza, o bloccare modifiche non autorizzate ai file di configurazione.

Claude Code esegue gli hook ConfigChange quando un file di impostazioni, un file di politica gestita, o un file di skill cambia. Per la politica gestita, li esegue solo quando `managed-settings.json` o un file in `managed-settings.d/` cambia. Applica le [impostazioni gestite dal server](/docs/it/server-managed-settings) e i cambiamenti alle preferenze gestite di macOS o alla politica del registro di Windows senza eseguirli. Su WSL con [`wslInheritsWindowsSettings`](/docs/it/settings-reference#wslinheritswindowssettings), applica anche un file di impostazioni gestite di Windows modificato sul suo sondaggio di politica senza eseguirli.

Il matcher filtra sulla fonte di configurazione:

| Matcher            | Quando si attiva                                                  |
| :----------------- | :---------------------------------------------------------------- |
| `user_settings`    | `~/.claude/settings.json` cambia                                  |
| `project_settings` | `.claude/settings.json` cambia                                    |
| `local_settings`   | `.claude/settings.local.json` cambia                              |
| `policy_settings`  | `managed-settings.json` o un file in `managed-settings.d/` cambia |
| `skills`           | Un file di skill in `.claude/skills/` cambia                      |

Questo esempio registra tutti i cambiamenti di configurazione per il controllo di sicurezza:

```json theme={null}
{
  "hooks": {
    "ConfigChange": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/audit-config-change.sh",
            "args": []
          }
        ]
      }
    ]
  }
}
```

<h4 id="configchange-input">
  ConfigChange input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook ConfigChange ricevono `source` e opzionalmente `file_path`. Il campo `source` indica quale tipo di configurazione è cambiato, e `file_path` fornisce il percorso al file specifico che è stato modificato.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "ConfigChange",
  "source": "project_settings",
  "file_path": "/Users/.../my-project/.claude/settings.json"
}
```

<h4 id="configchange-decision-control">
  ConfigChange decision control
</h4>

Gli hook ConfigChange possono bloccare i cambiamenti di configurazione dal prendere effetto. Usa il codice di uscita 2 o un JSON `decision` per prevenire il cambiamento. Quando bloccato, le nuove impostazioni non vengono applicate alla sessione in esecuzione.

| Field      | Description                                                                                                |
| :--------- | :--------------------------------------------------------------------------------------------------------- |
| `decision` | `"block"` impedisce l'applicazione del cambiamento di configurazione. Ometti per consentire il cambiamento |
| `reason`   | Accettato ma mai mostrato                                                                                  |

```json theme={null}
{
  "decision": "block",
  "reason": "Configuration changes to project settings require admin approval"
}
```

I cambiamenti `policy_settings` non possono essere bloccati. Gli hook si attivano comunque per le fonti `policy_settings` quando un file di impostazioni gestite sulla macchina cambia, in modo che tu possa registrare quelle modifiche, ma qualsiasi decisione di blocco viene ignorata. Questo assicura che le impostazioni gestite dall'azienda abbiano sempre effetto. Claude Code non esegue gli hook `ConfigChange` quando le [impostazioni gestite dal server](/docs/it/server-managed-settings) arrivano o si aggiornano.

Claude Code agisce sulla decisione di blocco dall'output JSON di un hook ConfigChange e scarta `systemMessage` e `continue`. Un cambiamento bloccato non mostra alcun messaggio a te o a Claude, indipendentemente dal fatto che tu blocchi con `reason` o con stderr all'uscita 2. Claude Code scrive solo una riga nel log di debug.

<h3 id="cwdchanged">
  CwdChanged
</h3>

Si esegue quando un comando shell nella conversazione principale cambia la directory di lavoro, ad esempio quando Claude esegue un comando `cd`. Usalo per reagire ai cambiamenti di directory: ricaricare le variabili di ambiente, attivare toolchain specifiche del progetto, o eseguire script di configurazione automaticamente. Si accoppia con [FileChanged](#filechanged) per strumenti come [direnv](https://direnv.net/) che gestiscono l'ambiente per directory.

Gli hook CwdChanged hanno accesso a [`CLAUDE_ENV_FILE`](#persist-environment-variables). Le variabili scritte in quel file persistono nei comandi Bash successivi fino al prossimo evento CwdChanged, quando Claude Code le cancella.

CwdChanged non supporta i matcher e si attiva su ogni occorrenza.

<h4 id="cwdchanged-input">
  CwdChanged input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook CwdChanged ricevono `old_cwd` e `new_cwd`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project/src",
  "hook_event_name": "CwdChanged",
  "old_cwd": "/Users/my-project",
  "new_cwd": "/Users/my-project/src"
}
```

<h4 id="cwdchanged-output">
  CwdChanged output
</h4>

Oltre ai [JSON output fields](#json-output) disponibili per tutti gli hook, gli hook CwdChanged possono restituire `watchPaths` per impostare dinamicamente quali percorsi di file [FileChanged](#filechanged) osserva:

| Field        | Description                                                                                                                                                                                                                                                           |
| :----------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `watchPaths` | Array di percorsi assoluti. Sostituisce l'elenco di osservazione dinamico corrente. I percorsi dalla tua configurazione `matcher` vengono sempre osservati. Restituire un array vuoto cancella l'elenco dinamico, che è tipico quando si entra in una nuova directory |

Gli hook CwdChanged non hanno controllo delle decisioni. Non possono bloccare il cambiamento di directory.

Claude Code legge `watchPaths` e `systemMessage` dal loro output JSON e scarta `continue`. Nelle sessioni interattive, mostra il `systemMessage` come una breve notifica di terminale. Il messaggio non raggiunge il flusso di messaggi dell'SDK.

<h3 id="directoryadded">
  DirectoryAdded
</h3>

Si esegue dopo che aggiungi una directory di lavoro a metà sessione con il comando `/add-dir`, o dopo che un client SDK ne aggiunge una con la richiesta di controllo `register_repo_root`. Usalo per preparare un repository appena aggiunto, ad esempio installando le sue dipendenze.

Claude Code non attiva questo evento quando:

* Passi una directory con il flag di avvio `--add-dir`; [SessionStart](#sessionstart) copre quelle directory
* Aggiungi una directory sulla scheda Workspace `/permissions`
* Aggiungi una directory che è già una directory di lavoro o dentro una

Claude Code attiva DirectoryAdded dopo aver aggiornato lo stato della sandbox e del permesso, quindi gli strumenti sandboxed vedono già la nuova directory quando il tuo hook si esegue. I comandi dell'hook stessi si eseguono non sandboxed.

Claude Code non attende l'hook: l'aggiunta si completa immediatamente, e l'hook si esegue in background con il timeout predefinito di 600 secondi.

Il matcher filtra su come la directory è stata aggiunta:

| Matcher              | Quando si attiva                                                                        |
| :------------------- | :-------------------------------------------------------------------------------------- |
| `slash_command`      | Aggiungi una directory con `/add-dir`                                                   |
| `register_repo_root` | Un client SDK aggiunge una directory con la richiesta di controllo `register_repo_root` |

<h4 id="directoryadded-input">
  DirectoryAdded input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook DirectoryAdded ricevono `directory` e `source`.

| Field       | Description                                                                                                                          |
| :---------- | :----------------------------------------------------------------------------------------------------------------------------------- |
| `directory` | Percorso assoluto della directory che è stata aggiunta                                                                               |
| `source`    | Come la directory è stata aggiunta, `"slash_command"` per `/add-dir` o `"register_repo_root"` per la richiesta di controllo dell'SDK |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "DirectoryAdded",
  "directory": "/Users/my-other-repo",
  "source": "slash_command"
}
```

Gli hook DirectoryAdded non hanno controllo delle decisioni. Non possono bloccare l'aggiunta, che si è già completata quando l'hook si esegue. Claude Code scarta il campo `continue` dal loro output JSON e visualizza il resto diversamente per fonte:

* `slash_command`: Claude Code consegna il `systemMessage` dell'hook a Claude come contesto sul prossimo turno della conversazione, piuttosto che mostrarlo a te. Un conteggio degli hook falliti appare nella trascrizione. L'output di fallimento completo va al log di debug
* `register_repo_root`: Claude Code scrive l'output `systemMessage` e l'output di fallimento solo nel log di debug

<h3 id="filechanged">
  FileChanged
</h3>

Si esegue quando un file osservato cambia su disco. Claude Code rileva i cambiamenti con un osservatore del file system, non ispezionando le chiamate di strumenti, quindi esegue l'hook indipendentemente da cosa ha cambiato il file: una chiamata di strumento `Edit` o `Write`, uno script che Claude esegue con `Bash`, o un processo al di fuori di Claude Code interamente. Un uso comune è ricaricare le variabili di ambiente quando i file di configurazione del progetto cambiano.

Il `matcher` per questo evento serve due ruoli:

* **Costruire l'elenco di osservazione**: il valore viene diviso su `|` e ogni segmento viene registrato come un nome di file letterale nella directory di lavoro, quindi `".envrc|.env"` osserva esattamente quei due file. I modelli regex non sono utili qui: un valore come `^\.env` osserverebbe un file letteralmente denominato `^\.env`.
* **Filtrare quali hook si eseguono**: quando un file osservato cambia, lo stesso valore filtra quali gruppi di hook si eseguono usando le [regole di matcher](#matcher-patterns) standard rispetto al nome di base del file modificato.

Questo esempio normalizza le terminazioni di riga in `data.csv` dopo qualsiasi cambiamento, incluso un comando `Bash` o uno script esterno che riscrive il file:

```json theme={null}
{
  "hooks": {
    "FileChanged": [
      {
        "matcher": "data.csv",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/normalize-line-endings.sh"
          }
        ]
      }
    ]
  }
}
```

L'hook legge il percorso assoluto del file modificato dal campo `file_path` dell'[input JSON](#filechanged-input) su stdin. La sua guardia `grep` testa la stessa cosa che `perl` rimuove, un CR alla fine di una riga, quindi l'esecuzione dopo una normalizzazione esce senza toccare il file. Una guardia più sciolta si cicla per sempre, perché `perl -i` riscrive il file anche quando non sostituisce nulla e Claude Code esegue l'hook di nuovo dopo ogni riscrittura. Salva questo script in `/path/to/normalize-line-endings.sh` e rendilo eseguibile:

```bash theme={null}
#!/bin/bash
FILE=$(jq -r .file_path)
if grep -q $'\r$' "$FILE"; then
  perl -pi -e 's/\r$//' "$FILE"
fi
```

Per confermare che l'hook funziona, chiedi a Claude di aggiungere una riga CRLF a `data.csv` con un comando `Bash`. Claude Code esegue l'hook e il file finisce con terminazioni LF.

Per osservare file che non puoi nominare in anticipo, restituisci [`watchPaths`](#filechanged-output) da un hook per aggiornare l'elenco di osservazione dinamicamente. Claude Code avvia l'osservatore solo quando qualcosa nomina un file da osservare, quindi semina l'elenco con un gruppo FileChanged il cui matcher nomina almeno un file, o con un hook [SessionStart](#sessionstart-decision-control) o [CwdChanged](#cwdchanged) che restituisce `watchPaths`. Il matcher filtra comunque quali gruppi di hook si eseguono quando un file osservato cambia, quindi dai al gruppo che gestisce i percorsi dinamici un matcher omesso, che corrisponde a ogni file osservato e non aggiunge nulla all'elenco di osservazione. Un matcher `"*"` corrisponde anche a ogni file, ma Claude Code lo registra nell'elenco di osservazione come un file letterale denominato `*`.

Gli hook FileChanged hanno accesso a [`CLAUDE_ENV_FILE`](#persist-environment-variables). Le variabili scritte in quel file persistono nei comandi Bash successivi fino al prossimo evento [CwdChanged](#cwdchanged), quando Claude Code le cancella.

<h4 id="filechanged-input">
  FileChanged input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook FileChanged ricevono `file_path` e `event`.

| Field       | Description                                                                                                        |
| :---------- | :----------------------------------------------------------------------------------------------------------------- |
| `file_path` | Percorso assoluto al file che è cambiato                                                                           |
| `event`     | Cosa è accaduto: `"change"` per un file modificato, `"add"` per un file creato, o `"unlink"` per un file eliminato |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "FileChanged",
  "file_path": "/Users/my-project/.envrc",
  "event": "change"
}
```

<h4 id="filechanged-output">
  FileChanged output
</h4>

Oltre ai [JSON output fields](#json-output) disponibili per tutti gli hook, gli hook FileChanged possono restituire `watchPaths` per aggiornare dinamicamente quali percorsi di file vengono osservati:

| Field        | Description                                                                                                                                                                                                                                                |
| :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `watchPaths` | Array di percorsi assoluti. Sostituisce l'elenco di osservazione dinamico corrente. I percorsi dalla tua configurazione `matcher` vengono sempre osservati. Usalo quando il tuo script hook scopre file aggiuntivi da osservare in base al file modificato |

Gli hook FileChanged non hanno controllo delle decisioni. Non possono bloccare il cambiamento del file dal verificarsi.

Claude Code legge `watchPaths` e `systemMessage` dal loro output JSON e scarta `continue`. Nelle sessioni interattive, mostra il `systemMessage` come una breve notifica di terminale. Il messaggio non raggiunge il flusso di messaggi dell'SDK.

<h3 id="worktreecreate">
  WorktreeCreate
</h3>

Si esegue quando un worktree viene creato, sia da `claude --worktree`, da un [subagent che usa `isolation: "worktree"`](/docs/it/sub-agents#choose-the-subagent-scope), o per una [sessione in background](/docs/it/agent-view#how-file-edits-are-isolated) che Claude Code isola nel suo proprio worktree. Per impostazione predefinita Claude Code crea la copia di lavoro isolata con `git worktree`. Configurare un hook WorktreeCreate sostituisce quel comportamento git predefinito, consentendoti di usare un sistema di controllo della versione diverso come SVN, Perforce, o Mercurial.

Poiché l'hook sostituisce completamente il comportamento predefinito, [`.worktreeinclude`](/docs/it/worktrees#copy-gitignored-files-into-worktrees) non viene elaborato. Se hai bisogno di copiare file di configurazione locale come `.env` nel nuovo worktree, fallo dentro il tuo script hook.

L'hook deve restituire il percorso alla directory del worktree creato. Claude Code usa questo percorso come directory di lavoro per la sessione isolata. Vedi [WorktreeCreate output](#worktreecreate-output) per come ogni tipo di hook restituisce il percorso.

Claude Code agisce sul successo dell'hook e sul percorso restituito, e scarta `systemMessage` e `continue`.

Questo esempio crea una copia di lavoro SVN e stampa il percorso per Claude Code da usare. Sostituisci l'URL del repository con il tuo:

```json theme={null}
{
  "hooks": {
    "WorktreeCreate": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'NAME=$(jq -r .name); DIR=\"$HOME/.claude/worktrees/$NAME\"; svn checkout https://svn.example.com/repo/trunk \"$DIR\" >&2 && echo \"$DIR\"'"
          }
        ]
      }
    ]
  }
}
```

L'hook legge il `name` del worktree dall'input JSON su stdin, controlla una copia fresca in una nuova directory, e stampa il percorso della directory. L'`echo` sull'ultima riga è ciò che Claude Code legge come percorso del worktree. Reindirizza qualsiasi altro output a stderr in modo che non interferisca con il percorso.

<h4 id="worktreecreate-input">
  WorktreeCreate input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook WorktreeCreate ricevono il campo `name`. Questo è un identificatore slug per il nuovo worktree, specificato dall'utente o auto-generato, ad esempio `bold-oak-a3f2`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "WorktreeCreate",
  "name": "feature-auth"
}
```

<h4 id="worktreecreate-output">
  WorktreeCreate output
</h4>

Gli hook WorktreeCreate non usano il modello di decisione di consentimento/blocco standard. Invece, il successo o il fallimento dell'hook determina il risultato. L'hook deve restituire il percorso alla directory del worktree creato:

* **Hook di comando** (`type: "command"`): stampa il percorso come l'ultima riga non vuota di stdout. Claude Code rimuove i codici di escape ANSI prima di leggere quella riga, quindi i banner di avvio della shell stampati prima del tuo `echo` vengono ignorati. Reindirizza qualsiasi altro output dell'hook a stderr.
* **Hook HTTP** (`type: "http"`): restituisci `{ "hookSpecificOutput": { "hookEventName": "WorktreeCreate", "worktreePath": "/absolute/path" } }` nel corpo della risposta.

Se l'hook fallisce o non produce alcun percorso, la creazione del worktree fallisce con un errore.

Claude Code risolve un percorso relativo rispetto alla directory in cui l'hook si è eseguito, collassando qualsiasi segmento `.` o `..` in esso. Se il percorso risultante non è una directory in cui Claude Code può entrare, la sessione stampa un errore che nomina il percorso ed esce con il codice 1.

Claude Code rifiuta un percorso assoluto che contiene segmenti `.` o `..`, e qualsiasi percorso che passa attraverso un symlink al di sotto della radice del repository, perché un symlink impegnato nel repository potrebbe reindirizzare il worktree al di fuori di esso. L'errore nomina il componente rifiutato. Restituisci un percorso normalizzato che non passa attraverso un symlink all'interno del repository. Prima della v2.1.216, la creazione del worktree seguiva il percorso dell'hook senza questo screening.

<h3 id="worktreeremove">
  WorktreeRemove
</h3>

Si esegue quando un worktree viene rimosso. Questo è l'equivalente di pulizia di [WorktreeCreate](#worktreecreate). L'evento si attiva quando:

* esci da una sessione `--worktree` e scegli di rimuoverla
* un subagent con `isolation: "worktree"` finisce
* elimini una [sessione in background](/docs/it/agent-view#what-deleting-a-session-removes) il cui worktree l'hook ha creato

Per i worktree basati su git, Claude Code gestisce la pulizia automaticamente con `git worktree remove`. Se hai configurato un hook WorktreeCreate per un sistema di controllo della versione non git, accoppialo con un hook WorktreeRemove per gestire la pulizia. Senza uno, la directory del worktree viene lasciata su disco.

Claude Code scarta i [JSON output fields](#json-output) di un hook WorktreeRemove, come `systemMessage` e `continue`.

Per un'eliminazione di sessione in background, Claude Code verifica il percorso del worktree archiviato prima di eseguire l'hook e rifiuta un percorso che è un symlink o passa attraverso uno al di sotto della radice del repository. L'hook si esegue per un worktree che contiene ancora file solo quando confermi l'eliminazione nella [agent view](/docs/it/agent-view#what-deleting-a-session-removes); per un tale worktree, [`claude rm`](/docs/it/agent-view#manage-sessions-from-the-shell) mantiene la sessione e il worktree invece. Prima della v2.1.216, l'hook si eseguiva sul percorso archiviato senza questi controlli.

Claude Code passa il percorso restituito da WorktreeCreate come `worktree_path` nell'input dell'hook. Questo esempio legge quel percorso e rimuove la directory:

```json theme={null}
{
  "hooks": {
    "WorktreeRemove": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'jq -r .worktree_path | xargs rm -rf'"
          }
        ]
      }
    ]
  }
}
```

<h4 id="worktreeremove-input">
  WorktreeRemove input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook WorktreeRemove ricevono il campo `worktree_path`, che è il percorso assoluto al worktree che viene rimosso.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "WorktreeRemove",
  "worktree_path": "/Users/.../my-project/.claude/worktrees/feature-auth"
}
```

Il codice di uscita di un hook WorktreeRemove decide il risultato. Quando un hook esce con non-zero e la directory in `worktree_path` esiste ancora dopo, la rimozione fallisce:

* Il worktree rimane su disco, e il comando dell'hook e stderr vanno al [debug log](#debug-hooks).
* Se stavi eliminando una sessione in background, la sessione rimane anche. Il messaggio di rifiuto nella [agent view](/docs/it/agent-view#what-deleting-a-session-removes) segnala come l'hook è terminato, come `exited 1`, cita l'inizio del suo stderr, e dice se eliminare la sessione di nuovo rimuove la directory comunque.

<h3 id="precompact">
  PreCompact
</h3>

Si esegue prima che Claude Code stia per eseguire un'operazione di compattazione.

Il valore del matcher indica se la compattazione è stata attivata manualmente o automaticamente:

| Matcher  | Quando si attiva                                                                                                           |
| :------- | :------------------------------------------------------------------------------------------------------------------------- |
| `manual` | `/compact`                                                                                                                 |
| `auto`   | Auto-compact quando la conversazione raggiunge la [finestra di auto-compact](/docs/it/model-config#set-the-auto-compact-window) |

Esci con il codice 2 per bloccare la compattazione. Per un `/compact` manuale, il messaggio stderr viene mostrato all'utente. Puoi anche bloccare restituendo JSON con `"decision": "block"`.

Bloccare la compattazione automatica ha effetti diversi a seconda di quando si attiva. Se la compattazione è stata attivata in modo proattivo prima del limite di contesto, Claude Code la salta e la conversazione continua non compattata. Se la compattazione è stata attivata per recuperare da un errore di limite di contesto già restituito dall'API, l'errore sottostante emerge e la richiesta corrente fallisce.

Claude Code scarta i campi `systemMessage` e `continue` di un hook PreCompact.

<h4 id="precompact-input">
  PreCompact input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook PreCompact ricevono `trigger` e `custom_instructions`. Per `manual`, `custom_instructions` contiene ciò che l'utente passa in `/compact` ed è `null` quando non passa nulla. Per `auto`, `custom_instructions` è `null`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PreCompact",
  "trigger": "manual",
  "custom_instructions": null
}
```

<h3 id="postcompact">
  PostCompact
</h3>

Si esegue dopo che Claude Code completa un'operazione di compattazione. Usalo per reagire al nuovo stato compattato, ad esempio per registrare il riepilogo generato o aggiornare lo stato esterno. Claude Code scarta i campi `systemMessage` e `continue` di un hook PostCompact.

Gli stessi valori di matcher si applicano come per `PreCompact`:

| Matcher  | Quando si attiva                                                                                                                |
| :------- | :------------------------------------------------------------------------------------------------------------------------------ |
| `manual` | Dopo `/compact`                                                                                                                 |
| `auto`   | Dopo auto-compact quando la conversazione raggiunge la [finestra di auto-compact](/docs/it/model-config#set-the-auto-compact-window) |

<h4 id="postcompact-input">
  PostCompact input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook PostCompact ricevono `trigger` e `compact_summary`. Il campo `compact_summary` contiene il riepilogo della conversazione generato dall'operazione di compattazione.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PostCompact",
  "trigger": "manual",
  "compact_summary": "Summary of the compacted conversation..."
}
```

Gli hook PostCompact non hanno controllo delle decisioni. Non possono influenzare il risultato della compattazione ma possono eseguire compiti di follow-up.

<h3 id="premodelswitch">
  PreModelSwitch
</h3>

Si esegue prima che Claude Code applichi un cambio di modello che hai richiesto tu o un client. Usalo per bloccare un cambio, richiedere conferma, o mostrare quale sarà il costo del cambio prima che accada.

PreModelSwitch richiede Claude Code v2.1.251 o successivo. Claude Code lo esegue per queste richieste:

* `/model <name>` e il picker `/model`
* Il picker del modello `Option+P` o `Alt+P`
* L'impostazione Model in `/config`
* Attivare la [fast mode](/docs/it/fast-mode) quando ciò cambia il modello della sessione
* Una richiesta `set_model`, o un cambiamento di modello in una richiesta `apply_flag_settings`, da un host [Agent SDK](/docs/it/agent-sdk/typescript#query-object) o [Remote Control](/docs/it/remote-control)

Claude Code non esegue gli hook PreModelSwitch per i cambi che fa da solo, come un [fallback automatico del modello](/docs/it/model-config#automatic-model-fallback) o il ripristino del modello quando riprendi una sessione. Quei cambi raggiungono [PostModelSwitch](#postmodelswitch) solo.

Claude Code confronta il matcher rispetto al nome canonico del modello a cui la sessione sta passando, ignorando qualsiasi suffisso `[1m]`. Un alias come `opus`, un ID di modello datato, e un ID specifico del provider come un ID di modello Amazon Bedrock corrispondono tutti al nome canonico unico a cui si risolvono, quindi `claude-opus-5` copre ogni ortografia di Opus 5.

Quando Claude Code non può determinare un nome canonico per il target, ad esempio un ID di modello personalizzato che solo il tuo [LLM gateway](/docs/it/llm-gateway) conosce, esegue ogni hook PreModelSwitch indipendentemente dal matcher. Un hook che blocca dovrebbe quindi controllare `to_model` dal suo input piuttosto che fare affidamento solo sul matcher.

Scrivi il matcher come un nome esatto, un elenco separato da `|` come `claude-opus-4-6|claude-opus-5`, o un'espressione regolare come `.*opus.*`. Questo esempio usa un matcher di nome esatto e controlla anche `to_model` dall'input dell'hook, quindi rifiuta un cambio a Opus 4.6 uscendo con il codice 2 e consente qualsiasi altro target:

<Tabs>
  <Tab title="macOS/Linux">
    Il comando controlla `to_model` con `jq`:

    ```json theme={null}
    {
      "hooks": {
        "PreModelSwitch": [
          {
            "matcher": "claude-opus-4-6",
            "hooks": [
              {
                "type": "command",
                "command": "jq -e '.to_model | test(\"opus-4-6\")' > /dev/null && { echo 'Opus 4.6 is retired for this project. Use a newer model.' >&2; exit 2; }; exit 0"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Windows (PowerShell)">
    Registra un hook di comando che esegue uno script tramite PowerShell:

    ```json theme={null}
    {
      "hooks": {
        "PreModelSwitch": [
          {
            "matcher": "claude-opus-4-6",
            "hooks": [
              {
                "type": "command",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-opus-46.ps1"
                ]
              }
            ]
          }
        ]
      }
    }
    ```

    Salva questo script in `.claude/hooks/block-opus-46.ps1` nel tuo progetto:

    ```powershell theme={null}
    $hookInput = [Console]::In.ReadToEnd() | ConvertFrom-Json
    if ($hookInput.to_model -match 'opus-4-6') {
      [Console]::Error.WriteLine('Opus 4.6 is retired for this project. Use a newer model.')
      exit 2
    }
    exit 0
    ```
  </Tab>
</Tabs>

Per confermare che l'hook funziona, esegui `/model claude-opus-4-6` da una sessione che esegue un modello diverso. Claude Code mantiene il modello corrente e segnala che un hook PreModelSwitch ha bloccato il cambio, con il tuo messaggio come motivo.

<h4 id="premodelswitch-input">
  PreModelSwitch input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook PreModelSwitch ricevono i campi in questa tabella. Gli ultimi cinque descrivono quale costo ha l'invio della conversazione al nuovo modello, quindi un hook può mostrare quella cifra prima che il cambio accada.

| Field                       | Type             | Description                                                                                                                                                                                                                                                                                                               |
| :-------------------------- | :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `from_model`                | string           | ID del modello da cui il cambio cambia                                                                                                                                                                                                                                                                                    |
| `to_model`                  | string           | ID del modello a cui il cambio cambia. Il matcher confronta rispetto al nome canonico di questo modello                                                                                                                                                                                                                   |
| `requested_model`           | string or `null` | Il modello che la richiesta ha nominato: un alias come `opus`, un ID di modello completo, o `null` quando la richiesta era per il modello predefinito                                                                                                                                                                     |
| `source`                    | string           | Da dove proviene la richiesta: `"command"` per `/model <name>`, l'impostazione Model in `/config`, o l'attivazione della fast mode; `"picker"` per un picker di modello; `"sdk"` per una richiesta `set_model`, o un cambiamento di modello in una richiesta `apply_flag_settings`, da un host Agent SDK o Remote Control |
| `context_tokens`            | number           | Token che la prossima richiesta invia di nuovo come suo prompt: i token di input, lettura della cache, creazione della cache, e output dell'ultima risposta nella conversazione principale, combinati. `0` prima della prima risposta                                                                                     |
| `prompt_cache_warm`         | boolean          | Se la prompt cache del modello corrente è probabilmente ancora calda, il che significa che il cambio la perde                                                                                                                                                                                                             |
| `cache_ttl`                 | string           | [Prompt cache lifetime](/docs/it/prompt-caching#cache-lifetime) che Claude Code richiede per questa sessione: `"5m"` o `"1h"`                                                                                                                                                                                                  |
| `estimated_cache_write_usd` | number           | Costo stimato in dollari USA della scrittura di `context_tokens` nella prompt cache su `to_model` al tasso `cache_ttl`, escludendo la prossima risposta. Il server potrebbe non aver bisogno di ri-memorizzare l'intero contesto, quindi trattalo come una stima                                                          |
| `pricing`                   | string           | Come Claude Code ha prezzato `estimated_cache_write_usd`: `"configured"` ai tassi della tua organizzazione quando li ha configurati, `"catalog"` al prezzo di listino, o `"default"` quando `to_model` non ha un prezzo noto e Claude Code ha assunto un tasso predefinito                                                |

Questo esempio mostra l'input per `/model opus` in una sessione che esegue Sonnet 5:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PreModelSwitch",
  "from_model": "claude-sonnet-5",
  "to_model": "claude-opus-5",
  "requested_model": "opus",
  "source": "command",
  "context_tokens": 182340,
  "prompt_cache_warm": true,
  "cache_ttl": "5m",
  "estimated_cache_write_usd": 1.1396,
  "pricing": "catalog"
}
```

<h4 id="premodelswitch-decision-control">
  PreModelSwitch decision control
</h4>

Gli hook `PreModelSwitch` possono annullare il cambio, chiedere all'utente di confermarlo, o consentirgli di procedere. Il codice di uscita 2 o un `decision: "block"` di livello superiore annulla il cambio.

Per un controllo più fine, restituisci `permissionDecision` e `permissionDecisionReason` in un oggetto `hookSpecificOutput`, come su [PreToolUse](#pretooluse-decision-control). `PreModelSwitch` accetta `"allow"`, `"deny"`, e `"ask"`. Non accetta `"defer"`, `updatedInput`, o `additionalContext`. La tabella seguente descrive entrambi i campi:

| Field                      | Description                                                                                                                                                                                                    |
| :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissionDecision`       | `"allow"` procede e salta la [conferma che Claude Code mostra mentre la prompt cache è calda](/docs/it/prompt-caching#switching-models). `"deny"` annulla il cambio. `"ask"` chiede all'utente di confermarlo       |
| `permissionDecisionReason` | Per `"deny"`, mostrato all'utente come motivo per cui il cambio è stato bloccato, o restituito come errore per una richiesta `set_model`. Per `"ask"`, mostrato nel prompt di conferma. Ignorato per `"allow"` |

Solo `/model` in una sessione interattiva può mostrare il prompt `"ask"`. Su ogni altra superficie, inclusa la modalità non interattiva con il flag `-p`, `/config`, e le richieste `set_model`, Claude Code tratta `"ask"` come un rifiuto.

Questo esempio chiede all'utente di confermare e cita il conteggio dei token da `context_tokens`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreModelSwitch",
    "permissionDecision": "ask",
    "permissionDecisionReason": "Switching now re-sends about 180k tokens to the new model. Continue?"
  }
}
```

Quando più hook PreModelSwitch restituiscono decisioni diverse, la precedenza è `deny` > `ask` > `allow`.

Claude Code mostra all'utente qualsiasi `systemMessage` che il tuo hook restituisce indipendentemente dalla decisione, quindi un hook di rapporto dei costi può restituire `{"systemMessage": "..."}` e uscire 0.

Un hook PreModelSwitch che non risponde prima del suo timeout blocca il cambio. Su [PreToolUse](#timeouts), al contrario, un hook di comando che scade consente alla chiamata dello strumento di continuare. Il timeout predefinito per questo evento è 30 secondi. `PreModelSwitch` esegue solo gli hook `command`, `http`, e `mcp_tool`, quindi i default `prompt` e `agent` non si applicano.

Un hook che esce con un codice diverso da 0 o 2 e non stampa alcuna decisione JSON non blocca: Claude Code mostra il suo stderr e applica il cambio, come descritto sotto [Other exit codes](#other-exit-codes).

<h3 id="postmodelswitch">
  PostModelSwitch
</h3>

Si esegue dopo che il modello della sessione cambia. Usalo per dare a Claude una guida specifica del modello senza modificare ogni CLAUDE.md, ad esempio un'istruzione a livello di organizzazione che si applica su determinati modelli.

PostModelSwitch richiede Claude Code v2.1.251 o successivo. Non può bloccare, perché il modello è già cambiato. Claude Code esegue gli hook PostModelSwitch dopo qualsiasi di questi cambi:

* Un cambio che hai richiesto tu o un client
* Un [fallback automatico del modello](/docs/it/model-config#automatic-model-fallback), che cambia il modello della sessione
* Un'impostazione come [`opusplan`](/docs/it/model-config#opusplan-model-setting) che entra o esce dalla plan mode
* Claude Code che ripristina il modello quando riprendi una sessione

Claude Code non esegue gli hook PostModelSwitch quando un modello da una [catena di fallback del modello](/docs/it/model-config#fallback-model-chains) serve un turno, perché quella sostituzione dura un turno e lascia il modello della sessione invariato.

Il matcher segue le stesse regole di [PreModelSwitch](#premodelswitch): Claude Code confronta il matcher rispetto al nome canonico del modello a cui la sessione è passata.

Questo esempio aggiunge una guida ogni volta che il modello della sessione cambia a qualsiasi modello Opus:

```json theme={null}
{
  "hooks": {
    "PostModelSwitch": [
      {
        "matcher": ".*opus.*",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'On Opus, delegate implementation work to subagents and keep this conversation for planning and review.'"
          }
        ]
      }
    ]
  }
}
```

Per confermare che l'hook funziona, passa a un modello Opus da una sessione che esegue un modello diverso, ad esempio esegui `/model opus` da una sessione Sonnet, quindi chiedi a Claude quale guida ha sul modello corrente.

<h4 id="postmodelswitch-input">
  PostModelSwitch input
</h4>

Gli hook PostModelSwitch ricevono gli stessi campi di [PreModelSwitch](#premodelswitch-input), con `hook_event_name` impostato su `"PostModelSwitch"` e due valori `source` in più: `"auto"` per un fallback automatico o un altro cambiamento che Claude Code ha fatto da solo, e `"resume"` per il modello ripristinato quando riprendi una sessione.

`requested_model` è `null` quando `source` è `"auto"`. Quando `source` è `"resume"`, è l'impostazione del modello salvato che Claude Code ha ripristinato.

<h4 id="postmodelswitch-decision-control">
  PostModelSwitch decision control
</h4>

Claude Code prende il tuo stdout in testo semplice dell'hook [plain-text stdout](#exit-code-0) all'uscita 0, o `additionalContext` dall'output JSON, e lo consegna a Claude con la richiesta successiva dopo il cambio. Oltre ai [JSON output fields](#json-output) disponibili per tutti gli hook, puoi restituire:

| Field               | Description                                                                                                                |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | Stringa aggiunta al contesto di Claude con la richiesta successiva. Vedi [Add context for Claude](#add-context-for-claude) |

Se l'hook non finisce entro cinque secondi dopo che invii la richiesta successiva, Claude Code invia quella richiesta senza l'output e lo allega alla richiesta seguente invece. Se il modello cambia diverse volte prima della richiesta successiva, Claude Code consegna solo l'output per il modello target dell'ultimo cambio.

<h3 id="sessionend">
  SessionEnd
</h3>

Si esegue quando una sessione di Claude Code termina. Utile per i compiti di pulizia, la registrazione delle statistiche della sessione, o il salvataggio dello stato della sessione. Supporta i matcher per filtrare per motivo di uscita.

Il campo `reason` nell'input dell'hook indica perché la sessione è terminata:

| Reason                        | Description                                                                             |
| :---------------------------- | :-------------------------------------------------------------------------------------- |
| `clear`                       | Sessione cancellata con il comando `/clear`                                             |
| `resume`                      | Sessione passata tramite `/resume` interattivo                                          |
| `logout`                      | Utente disconnesso                                                                      |
| `prompt_input_exit`           | Utente uscito mentre l'input del prompt era visibile                                    |
| `other`                       | Altri motivi di uscita                                                                  |
| `bypass_permissions_disabled` | Rimosso nella v2.1.234; Claude Code non lo invia. Elimina dai tuoi matcher `SessionEnd` |

<h4 id="sessionend-input">
  SessionEnd input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook SessionEnd ricevono un campo `reason` che indica perché la sessione è terminata. Vedi la [tabella dei motivi](#sessionend) sopra per tutti i valori.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionEnd",
  "reason": "other"
}
```

Gli hook SessionEnd non hanno controllo delle decisioni. Non possono bloccare la terminazione della sessione ma possono eseguire compiti di pulizia. Claude Code scarta i loro [JSON output fields](#json-output), come `systemMessage`.

Gli hook SessionEnd hanno un timeout predefinito di 1,5 secondi. Questo si applica all'uscita della sessione, a `/clear`, e al passaggio di sessioni tramite `/resume` interattivo. Se un hook ha bisogno di più tempo, imposta un `timeout` per hook nella configurazione dei file di impostazioni. Il budget complessivo viene automaticamente aumentato al timeout per hook più alto configurato nei file di impostazioni, fino a 60 secondi. I timeout impostati su hook forniti da plugin non aumentano il budget.

Questo esempio imposta il budget a 5 secondi:

```bash theme={null}
CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS=5000 claude
```

Prima della v2.1.268, `CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS` aumentava solo il budget complessivo, e un hook senza il suo `timeout` era comunque annullato dopo 1,5 secondi.

<h3 id="elicitation">
  Elicitation
</h3>

Si esegue quando un server MCP richiede input dell'utente a metà compito. Per impostazione predefinita, Claude Code mostra un dialogo interattivo per l'utente per rispondere. Gli hook possono intercettare questa richiesta e rispondere programmaticamente, saltando completamente il dialogo.

Il campo matcher corrisponde al nome del server MCP.

<h4 id="elicitation-input">
  Elicitation input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook Elicitation ricevono `mcp_server_name`, `message`, e campi facoltativi `mode`, `url`, `elicitation_id`, e `requested_schema`.

Per l'elicitazione in modalità modulo, il caso più comune:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Elicitation",
  "mcp_server_name": "my-mcp-server",
  "message": "Please provide your credentials",
  "mode": "form",
  "requested_schema": {
    "type": "object",
    "properties": {
      "username": { "type": "string", "title": "Username" }
    }
  }
}
```

Per l'elicitazione in modalità URL, usata per l'autenticazione basata su browser:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Elicitation",
  "mcp_server_name": "my-mcp-server",
  "message": "Please authenticate",
  "mode": "url",
  "url": "https://auth.example.com/login"
}
```

<h4 id="elicitation-output">
  Elicitation output
</h4>

Per rispondere programmaticamente senza mostrare il dialogo, restituisci un oggetto JSON con `hookSpecificOutput`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "Elicitation",
    "action": "accept",
    "content": {
      "username": "alice"
    }
  }
}
```

| Field     | Values                        | Description                                                                   |
| :-------- | :---------------------------- | :---------------------------------------------------------------------------- |
| `action`  | `accept`, `decline`, `cancel` | Se accettare, rifiutare, o annullare la richiesta                             |
| `content` | object                        | Valori dei campi del modulo da inviare. Usato solo quando `action` è `accept` |

L'uscita con il codice 2 nega l'elicitazione. Claude Code non mostra il tuo messaggio stderr da nessuna parte.

Claude Code agisce su `hookSpecificOutput` dall'output JSON di un hook Elicitation e scarta `systemMessage` e `continue`.

<h3 id="elicitationresult">
  ElicitationResult
</h3>

Si esegue dopo che un utente risponde a un'elicitazione MCP. Gli hook possono osservare, modificare, o bloccare la risposta prima che venga inviata di nuovo al server MCP.

Il campo matcher corrisponde al nome del server MCP.

<h4 id="elicitationresult-input">
  ElicitationResult input
</h4>

Oltre ai [common input fields](#common-input-fields), gli hook ElicitationResult ricevono `mcp_server_name`, `action`, e campi facoltativi `mode`, `elicitation_id`, e `content`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "ElicitationResult",
  "mcp_server_name": "my-mcp-server",
  "action": "accept",
  "content": { "username": "alice" },
  "mode": "form",
  "elicitation_id": "elicit-123"
}
```

<h4 id="elicitationresult-output">
  ElicitationResult output
</h4>

Per sovrascrivere la risposta dell'utente, restituisci un oggetto JSON con `hookSpecificOutput`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "ElicitationResult",
    "action": "decline",
    "content": {}
  }
}
```

| Field     | Values                        | Description                                                                              |
| :-------- | :---------------------------- | :--------------------------------------------------------------------------------------- |
| `action`  | `accept`, `decline`, `cancel` | Sovrascrive l'azione dell'utente                                                         |
| `content` | object                        | Sovrascrive i valori dei campi del modulo. Significativo solo quando `action` è `accept` |

L'uscita con il codice 2 blocca la risposta, cambiando l'azione effettiva a `decline`. Claude Code non mostra il tuo messaggio stderr da nessuna parte.

Claude Code agisce su `hookSpecificOutput` dall'output JSON di un hook ElicitationResult e scarta `systemMessage` e `continue`.

<h2 id="prompt-based-hooks">
  Hook basati su prompt
</h2>

Oltre agli hook di comando, HTTP e MCP tool, Claude Code supporta gli hook basati su prompt (`type: "prompt"`) che utilizzano un LLM per valutare se consentire o bloccare un'azione, e gli hook basati su agenti (`type: "agent"`) che generano un verificatore agentico con accesso agli strumenti. Non tutti gli eventi supportano ogni tipo di hook.

Gli eventi che supportano tutti e cinque i tipi di hook (`command`, `http`, `mcp_tool`, `prompt` e `agent`):

* `PermissionDenied`
* `PostToolBatch`
* `PostToolUse`
* `PostToolUseFailure`
* `PreToolUse`
* `Stop`
* `SubagentStop`
* `TaskCompleted`
* `TaskCreated`
* `TeammateIdle`
* `UserPromptExpansion`
* `UserPromptSubmit`

`PermissionRequest` supporta gli hook `command`, `http`, `mcp_tool` e `prompt` ma non gli hook `agent`. Se configuri un hook agent su questo evento, Claude Code lo salta e il flusso di autorizzazione procede invariato. Per consentire o negare da un hook, restituisci l'[oggetto decisione](#permissionrequest-decision-control) da un hook di comando o HTTP.

Gli eventi che supportano gli hook `command`, `http` e `mcp_tool` ma non `prompt` o `agent`:

* `ConfigChange`
* `CwdChanged`
* `DirectoryAdded`
* `Elicitation`
* `ElicitationResult`
* `FileChanged`
* `InstructionsLoaded`
* `MessageDisplay`
* `Notification`
* `PostCompact`
* `PostModelSwitch`
* `PreCompact`
* `PreModelSwitch`
* `SessionEnd`
* `StopFailure`
* `SubagentStart`
* `WorktreeCreate`
* `WorktreeRemove`

`SessionStart` e `Setup` supportano gli hook `command` e `mcp_tool`, e [i campi degli hook MCP tool](#mcp-tool-hook-fields) descrivono quando i loro hook `mcp_tool` vengono eseguiti. Non supportano gli hook `http`, `prompt` o `agent`.

<h3 id="how-prompt-based-hooks-work">
  Come funzionano gli hook basati su prompt
</h3>

Invece di eseguire un comando Bash, gli hook basati su prompt:

1. Inviano l'input del hook e il prompt a un modello Claude, Haiku per impostazione predefinita
2. L'LLM risponde con JSON strutturato contenente una decisione
3. Claude Code elabora automaticamente la decisione

<h3 id="prompt-hook-configuration">
  Configurazione del prompt hook
</h3>

Impostare `type` su `"prompt"` e fornire una stringa `prompt` invece di un `command`. Utilizzare il segnaposto `$ARGUMENTS` per iniettare i dati di input JSON del hook nel testo del prompt.

Questo hook `Stop` chiede all'LLM di valutare se tutti i compiti sono completi prima di consentire a Claude di terminare:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "Evaluate if Claude should stop: $ARGUMENTS. Check if all tasks are complete."
          }
        ]
      }
    ]
  }
}
```

| Campo             | Obbligatorio | Descrizione                                                                                                                                                                                                                            |
| :---------------- | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`            | sì           | Deve essere `"prompt"`                                                                                                                                                                                                                 |
| `prompt`          | sì           | Il testo del prompt da inviare all'LLM. Utilizzare `$ARGUMENTS` come segnaposto per l'input JSON del hook. Se `$ARGUMENTS` non è presente, l'input JSON viene aggiunto al prompt                                                       |
| `model`           | no           | Modello da utilizzare per la valutazione. Impostazione predefinita: un modello veloce                                                                                                                                                  |
| `timeout`         | no           | Timeout in secondi. Impostazione predefinita: 30                                                                                                                                                                                       |
| `continueOnBlock` | no           | Sugli eventi a cui si applica, `true` reinvia un motivo `ok: false` a Claude e continua invece di terminare il turno. Impostazione predefinita: `false`. Vedere [Schema di risposta](#response-schema) per il comportamento per evento |

<h3 id="response-schema">
  Schema di risposta
</h3>

L'LLM deve rispondere con JSON contenente:

```json theme={null}
{
  "ok": true | false,
  "reason": "Explanation for the decision",
  "impossible": true | false
}
```

| Campo        | Descrizione                                                                                                                                                                                                                                                                     |
| :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `ok`         | `true` per consentire. Per `false`, vedere il comportamento per evento di seguito                                                                                                                                                                                               |
| `reason`     | Obbligatorio quando `ok` è `false`                                                                                                                                                                                                                                              |
| `impossible` | Facoltativo. Il modello lo restituisce con `ok: false` quando giudica che la condizione non può mai essere soddisfatta. Su `Stop` e `SubagentStop`, Claude Code consente quindi al turno di terminare invece di reinviare il motivo. Gli hook agenti e altri eventi lo ignorano |

Ciò che accade con `ok: false` dipende dall'evento:

* `Stop` e `SubagentStop`: il motivo viene reinviato a Claude come sua prossima istruzione e il turno continua, a meno che la risposta non imposti anche `impossible: true`, nel qual caso Claude Code consente lo stop e il turno termina
* `PreToolUse`: la chiamata dello strumento viene negata; per impostazione predefinita il turno termina e il motivo della negazione appare nella chat come una riga di avviso. Impostare `continueOnBlock: true` per reinviare il motivo a Claude come errore dello strumento in modo che possa adattarsi e continuare, equivalente a un hook di comando con `permissionDecision: "deny"`. Prima della v2.1.210, il motivo della negazione veniva restituito a Claude come errore dello strumento e il turno continuava
* `PostToolUse`: per impostazione predefinita il turno termina e il motivo appare nella chat come una riga di avviso. Impostare `continueOnBlock: true` per reinviare il motivo a Claude e continuare il turno invece
* `PostToolBatch`, `UserPromptSubmit` e `UserPromptExpansion`: il turno termina e il motivo appare come una riga di avviso. Questi eventi terminano il turno su `decision: "block"` indipendentemente da `continue`
* `PostToolUseFailure` e `TaskCreated`: il motivo viene restituito a Claude come errore dello strumento e il turno continua, indipendentemente da `continueOnBlock`
* `TaskCompleted`: quando si attiva perché un'attività è contrassegnata come completata durante un turno, il motivo viene restituito a Claude come errore dello strumento e il turno continua, indipendentemente da `continueOnBlock`. Quando si attiva perché un compagno di squadra si ferma, si comporta come `TeammateIdle` e arresta il compagno di squadra per impostazione predefinita
* `TeammateIdle`: per impostazione predefinita il compagno di squadra si ferma e il motivo appare come una riga di avviso. Impostare `continueOnBlock: true` per reinviare il motivo al compagno di squadra e mantenerlo al lavoro invece
* `PermissionRequest`: `ok: false` non ha effetto. Per negare un'approvazione da un hook, utilizzare un [hook di comando](#command-hook-fields) che restituisce `hookSpecificOutput.decision.behavior: "deny"`
* `PermissionDenied`: `ok: false` non ha effetto perché il rifiuto è già avvenuto. L'unico output che questo evento legge è `hookSpecificOutput.retry`, che gli hook di prompt e agenti non possono impostare. Vengono eseguiti su questo evento, ma il loro output viene scartato. Utilizzare un [hook di comando](#command-hook-fields) per restituire `retry`

Se hai bisogno di un controllo più fine su qualsiasi evento, utilizza un [hook di comando](#command-hook-fields) con i campi per evento descritti in [Controllo delle decisioni](#decision-control).

<h3 id="check-multiple-conditions-before-stopping">
  Controllare più condizioni prima di fermarsi
</h3>

Questo hook `Stop` utilizza un prompt dettagliato per controllare tre condizioni prima di consentire a Claude di fermarsi. Gli hook `SubagentStop` utilizzano lo stesso formato per valutare se un [subagent](/docs/it/sub-agents) dovrebbe fermarsi. Se il modello restituisce `"ok": false` perché la condizione non è ancora soddisfatta, Claude continua a lavorare con il motivo fornito come sua prossima istruzione:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "You are evaluating whether Claude should stop working. Context: $ARGUMENTS\n\nAnalyze the conversation and determine if:\n1. All user-requested tasks are complete\n2. Any errors need to be addressed\n3. Follow-up work is needed\n\nRespond with JSON: {\"ok\": true} to allow stopping, or {\"ok\": false, \"reason\": \"your explanation\"} to continue working.",
            "timeout": 30
          }
        ]
      }
    ]
  }
}
```

<h2 id="agent-based-hooks">
  Hook basati su agenti
</h2>

<Warning>
  Gli hook agente sono sperimentali. Il comportamento e la configurazione potrebbero cambiare nelle versioni future. Per i flussi di lavoro in produzione, preferire gli [hook di comando](#command-hook-fields).
</Warning>

Gli hook basati su agenti (`type: "agent"`) sono come gli hook basati su prompt ma con accesso agli strumenti multi-turno. Invece di una singola chiamata LLM, un hook agente genera un subagent che può leggere file, cercare codice e ispezionare il codebase per verificare le condizioni. Gli hook agente supportano gli stessi eventi degli [hook basati su prompt](#prompt-based-hooks), ad eccezione di `PermissionRequest`.

<h3 id="how-agent-hooks-work">
  Come funzionano gli hook basati su agenti
</h3>

Quando un hook agente si attiva:

1. Claude Code genera un subagent con il prompt e l'input JSON del hook
2. Il subagent può utilizzare strumenti come Read, Grep e Glob per investigare
3. Dopo fino a 50 turni, il subagent restituisce una decisione strutturata `{ "ok": true/false }`
4. Claude Code consente l'azione se `ok` è `true`. Se `ok` è `false`, Claude Code gestisce il blocco nello stesso modo di un hook di prompt con `continueOnBlock: true` su quell'evento, come elencato sotto [Schema di risposta](#response-schema)

Gli hook agente sono utili quando la verifica richiede l'ispezione dei file effettivi o dell'output dei test, non solo la valutazione dei dati di input del hook da soli.

<h3 id="agent-hook-configuration">
  Configurazione dell'hook agente
</h3>

Impostare `type` su `"agent"` e fornire una stringa `prompt`, utilizzando `$ARGUMENTS` come segnaposto per l'input JSON del hook. I campi di configurazione sono gli stessi degli [hook di prompt](#prompt-hook-configuration), ad eccezione del fatto che gli hook agente hanno un timeout predefinito più lungo di 60 secondi e nessun campo `continueOnBlock`.

Lo schema di risposta è `{ "ok": true }` per consentire o `{ "ok": false, "reason": "..." }` per bloccare. Su `ok: false`, Claude Code gestisce un hook agente nello stesso modo in cui gestisce un [hook di prompt con `continueOnBlock: true`](#response-schema) sullo stesso evento; gli hook agente non hanno un campo `continueOnBlock` e non supportano il campo `impossible` dell'hook di prompt.

Questo hook `Stop` verifica che tutti i test unitari passino prima di consentire a Claude di finire:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "agent",
            "prompt": "Verify that all unit tests pass. Run the test suite and check the results. $ARGUMENTS",
            "timeout": 120
          }
        ]
      }
    ]
  }
}
```

<h2 id="run-hooks-in-the-background">
  Eseguire i hook in background
</h2>

Per impostazione predefinita, gli hook bloccano l'esecuzione di Claude fino al completamento. Per le attività a lunga esecuzione come distribuzioni, suite di test o chiamate API esterne, impostare `"async": true` per eseguire l'hook in background mentre Claude continua a lavorare. Gli hook asincroni non possono bloccare o controllare il comportamento di Claude: i campi di risposta come `decision`, `permissionDecision` e `continue` non hanno effetto, perché l'azione che avrebbero controllato è già stata completata.

<h3 id="configure-an-async-hook">
  Configurare un hook asincrono
</h3>

Aggiungere `"async": true` alla configurazione di un command hook per eseguirlo in background senza bloccare Claude. Questo campo è disponibile solo sui hook `type: "command"`.

Questo hook esegue uno script di test dopo ogni chiamata dello strumento `Write`. Claude continua a lavorare immediatamente mentre `run-tests.sh` viene eseguito. Quando lo script termina, l'output viene consegnato al turno di conversazione successivo:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/run-tests.sh",
            "async": true
          }
        ]
      }
    ]
  }
}
```

Una volta che un hook asincrono è in esecuzione in background, Claude Code non applica `timeout` su di esso. Claude Code continua ad applicare `timeout` su un hook che si esegue con `asyncRewake`.

Claude Code consegna i risultati di un hook asincrono solo mentre la sessione è in esecuzione:

* In [modalità non interattiva](/docs/it/headless) con il flag `-p`, Claude Code termina qualsiasi hook asincrono ancora in esecuzione al teardown e lo finalizza con esito `cancelled`
* Se il lavoro del vostro hook deve sopravvivere a una sessione `claude -p`, avviate un processo completamente staccato da esso

<h3 id="how-async-hooks-execute">
  Come vengono eseguiti gli hook asincroni
</h3>

Quando un hook asincrono si attiva, Claude Code avvia il processo del hook e continua immediatamente senza aspettare il completamento. L'hook riceve lo stesso input JSON tramite stdin di un hook sincrono.

Dopo che il processo in background esce, Claude Code consegna i campi `additionalContext` e `systemMessage` dalla risposta JSON dell'hook a Claude al turno di conversazione successivo. A differenza di `systemMessage` di un hook sincrono, nessuno dei due campi viene mostrato a voi.

Claude Code convalida quella risposta JSON rispetto allo stesso [schema di output](#json-output) degli hook sincroni e scarta qualsiasi campo il cui valore ha il tipo errato, come un `systemMessage` che non è una stringa, invece di consegnarlo. Eseguire con `--debug` per vedere un avviso che nomina ogni campo scartato. Prima della v2.1.202, l'output JSON malformato da un hook asincrono poteva causare l'arresto della sessione e l'arresto si ripeteva ogni volta che la sessione veniva ripresa.

Le notifiche di completamento degli hook asincroni sono soppresse per impostazione predefinita. Per vederle, abilitare la modalità verbose con `Ctrl+O` o avviare Claude Code con `--verbose`.

<h3 id="run-tests-after-file-changes">
  Eseguire i test dopo le modifiche ai file
</h3>

Questo hook avvia una suite di test in background ogni volta che Claude scrive un file, quindi segnala i risultati a Claude quando i test terminano. Salvare questo script in `.claude/hooks/run-tests-async.sh` nel progetto e renderlo eseguibile con `chmod +x`:

```bash theme={null}
#!/bin/bash
# run-tests-async.sh

# Leggere l'input del hook da stdin
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

# Eseguire i test solo per i file di origine
if [[ "$FILE_PATH" != *.ts && "$FILE_PATH" != *.js ]]; then
  exit 0
fi

# Eseguire i test e segnalare i risultati a Claude tramite additionalContext
RESULT=$(npm test 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
  MSG="Tests passed after editing $FILE_PATH"
else
  MSG="Tests failed after editing $FILE_PATH: $RESULT"
fi
jq -nc --arg msg "$MSG" '{hookSpecificOutput: {hookEventName: "PostToolUse", additionalContext: $msg}}'
```

Quindi aggiungere questa configurazione a `.claude/settings.json` nella radice del progetto. Il flag `async: true` consente a Claude di continuare a lavorare mentre i test vengono eseguiti:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/run-tests-async.sh",
            "args": [],
            "async": true
          }
        ]
      }
    ]
  }
}
```

<h3 id="limitations">
  Limitazioni
</h3>

Gli hook asincroni hanno vincoli aggiuntivi rispetto agli hook sincroni:

* L'output del hook viene consegnato al turno di conversazione successivo. Se la sessione è inattiva, la risposta attende fino alla prossima interazione dell'utente. Eccezione: un hook `asyncRewake` che esce con il codice 2 riattiva Claude immediatamente anche quando la sessione è inattiva.
* Ogni esecuzione crea un processo in background separato. Non c'è deduplicazione tra più attivazioni dello stesso hook asincrono.

<h2 id="security-considerations">
  Considerazioni sulla sicurezza
</h2>

<h3 id="disclaimer">
  Disclaimer
</h3>

<Warning>
  I command hook eseguono comandi shell con i permessi completi dell'utente. Possono modificare, eliminare o accedere a qualsiasi file a cui l'account utente può accedere. Rivedere e testare tutti i comandi del hook prima di aggiungerli alla configurazione.
</Warning>

<h3 id="workspace-trust">
  Fiducia nell'area di lavoro
</h3>

Claude Code verifica la fiducia nell'area di lavoro prima di eseguire qualsiasi hook da un file di impostazioni. Ciò che conta come attendibile dipende dal tipo di sessione:

* **Sessione interattiva**: Claude Code trattiene i hook da ogni file di impostazioni, incluso il vostro `~/.claude/settings.json`, fino a quando non accettate la [finestra di dialogo di fiducia nell'area di lavoro](/docs/it/permissions#project-allow-rules-and-workspace-trust) per la cartella, o per una directory padre la cui fiducia si estende ad essa
* **Sessione `-p` o SDK**: Claude Code non mostra mai la finestra di dialogo e tratta la cartella come attendibile, quindi i hook sottoposti a commit nel `.claude/settings.json` di un repository vengono eseguiti in una cartella che non avete mai considerato attendibile

Prima di eseguire lo script `claude -p` su un repository che non avete scritto, rivedete i file di impostazioni `.claude/`, iniziate con [`--bare`](/docs/it/headless#start-faster-with-bare-mode), o [disattivate i hook per quella esecuzione](#disable-or-remove-hooks) con `--settings '{"disableAllHooks": true}'`. I hook nel frontmatter in un subagent di progetto seguono una regola più ristretta rispetto ai hook dei file di impostazioni. [Ciò che viene eseguito prima di considerare attendibile una cartella](/docs/it/permissions#what-runs-before-you-trust-a-folder) elenca ogni tipo di contenuto del repository per tipo di sessione.

<h3 id="security-best-practices">
  Migliori pratiche di sicurezza
</h3>

Tenere presenti queste pratiche quando si scrivono i hook:

* **Convalidare e disinfettare gli input**: non fidarsi mai ciecamente dei dati di input
* **Citare sempre le variabili shell**: utilizzare `"$VAR"` non `$VAR`
* **Bloccare l'attraversamento del percorso**: controllare `..` nei percorsi dei file
* **Utilizzare percorsi assoluti**: specificare percorsi completi per gli script. Nel modulo exec, utilizzare `${CLAUDE_PROJECT_DIR}` e il percorso non necessita di virgolette. Nel modulo shell, racchiuderlo tra virgolette doppie
* **Saltare i file sensibili**: evitare `.env`, `.git/`, chiavi, ecc.

<h2 id="windows-powershell-tool">
  Strumento Windows PowerShell
</h2>

Su Windows, è possibile eseguire singoli hook in PowerShell impostando `"shell": "powershell"` su un command hook. Claude Code rileva automaticamente `pwsh.exe`, l'eseguibile di PowerShell 7 e versioni successive, e ricade su `powershell.exe` per Windows PowerShell 5.1.

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write",
        "hooks": [
          {
            "type": "command",
            "shell": "powershell",
            "command": "Write-Host 'File written'"
          }
        ]
      }
    ]
  }
}
```

Per fare riferimento alla directory radice del progetto da un comando in forma shell di PowerShell, scrivere `${CLAUDE_PROJECT_DIR}` o `$env:CLAUDE_PROJECT_DIR`. A partire dalla v2.1.198, Claude Code riscrive i segnaposti `${CLAUDE_PROJECT_DIR}`, `${CLAUDE_PLUGIN_ROOT}` e `${CLAUDE_PLUGIN_DATA}` in un comando in forma shell di PowerShell nella forma `${env:NAME}` di PowerShell, indipendentemente dal fatto che l'hook sia definito in `settings.json`, un plugin o una skill. PowerShell quindi risolve il valore dall'ambiente esportato dopo l'analisi, quindi il segnaposto funziona all'interno di stringhe tra virgolette doppie ma non all'interno di stringhe tra virgolette singole, dove PowerShell non espande mai le variabili.

Prima della v2.1.198, questa riscrittura si applicava solo agli hook dei plugin. Nelle versioni precedenti, un hook `settings.json` necessita della forma `$env:` o della [forma exec](#exec-form-and-shell-form), dove `${CLAUDE_PROJECT_DIR}` viene sostituito in ogni elemento `args` indipendentemente da dove l'hook è definito.

Non scrivere la forma nuda `$CLAUDE_PROJECT_DIR` in un hook di PowerShell. PowerShell la analizza come una variabile locale non definita e la risolve in `$null`, il che lascia il percorso dello script senza il prefisso della directory radice del progetto. Claude Code non riscrive quella forma; invece registra un avviso nel [log di debug](#debug-hooks).

L'esempio seguente mostra un hook `settings.json` che esegue uno script di progetto con la forma `$env:`, che funziona su ogni versione:

```json theme={null}
{
  "type": "command",
  "shell": "powershell",
  "command": "& \"$env:CLAUDE_PROJECT_DIR\\.claude\\hooks\\check.ps1\""
}
```

<h2 id="debug-hooks">
  Debug dei hook
</h2>

I dettagli dell'esecuzione dei hook vengono scritti nel file di log di debug. Avviare Claude Code con `claude --debug-file <path>` per scrivere il log in una posizione nota, oppure eseguire `claude --debug` e leggere il log in `~/.claude/debug/<session-id>.txt`. Il flag `--debug` non stampa nel terminale.

Ad esempio, un hook `PostToolUse` su `Write` il cui comando stampa `hook-ran` produce voci come:

```text theme={null}
2026-07-19T02:03:24.382Z [DEBUG] Hook output does not start with {, treating as plain text
2026-07-19T02:03:24.382Z [DEBUG] "Hook PostToolUse:Write (PostToolUse) success:\nhook-ran"
```

Per dettagli di corrispondenza dei hook più granulari, impostare `CLAUDE_CODE_DEBUG_LOG_LEVEL=verbose` per visualizzare righe di log aggiuntive come i conteggi dei matcher del hook e la corrispondenza delle query.

Per la risoluzione dei problemi comuni come i hook che non si attivano, i Stop hook che continuano a bloccare, o gli errori di configurazione, consultare [Limitations and troubleshooting](/docs/it/hooks-guide#limitations-and-troubleshooting) nella guida. Per una procedura diagnostica più ampia che copre `/context`, `/doctor` e la precedenza delle impostazioni, consultare [Debug your config](/docs/it/debug-your-config).
