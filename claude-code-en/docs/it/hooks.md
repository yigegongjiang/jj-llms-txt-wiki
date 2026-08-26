> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Riferimento dei hooks

> Riferimento per gli eventi dei hook di Claude Code, schema di configurazione, formati JSON di input/output, codici di uscita, hook asincroni, hook HTTP, hook di prompt e hook degli strumenti MCP.

<Tip>
  Per una guida di avvio rapido con esempi, consultare [Automatizzare i flussi di lavoro con i hook](/docs/it/hooks-guide).
</Tip>

Gli hook sono comandi shell definiti dall'utente, endpoint HTTP o prompt LLM che si eseguono automaticamente in punti specifici del ciclo di vita di Claude Code. Utilizzare questo riferimento per cercare schemi di eventi, opzioni di configurazione, formati JSON di input/output e funzionalità avanzate come hook asincroni, hook HTTP e hook degli strumenti MCP. Se si stanno configurando i hook per la prima volta, iniziare con la [guida](/docs/it/hooks-guide).

<h2 id="hook-lifecycle">
  Ciclo di vita dei hook
</h2>

Gli hook si attivano in punti specifici durante una sessione di Claude Code. Quando un evento si attiva e un matcher corrisponde, Claude Code passa il contesto JSON dell'evento al gestore del hook. Per i hook di comando, l'input arriva su stdin. Per i hook HTTP, arriva come corpo della richiesta POST. Il gestore può quindi ispezionare l'input, intraprendere un'azione e facoltativamente restituire una decisione.

Gli eventi si dividono in tre cadenze:

* una volta per sessione: `SessionStart` e `SessionEnd`
* una volta per turno: `UserPromptSubmit`, `Stop` e `StopFailure`
* ad ogni chiamata dello strumento all'interno del ciclo agentico: `PreToolUse` e `PostToolUse`

<div style={{maxWidth: "500px", margin: "0 auto"}}>
  <Frame>
    <img src="https://mintcdn.com/claude-code/jhXrDR5TrSZ5hgXM/images/hooks-lifecycle.svg?fit=max&auto=format&n=jhXrDR5TrSZ5hgXM&q=85&s=3ca47113d5956460e6e4611b8dbc63b7" alt="Diagramma del ciclo di vita dei hook che mostra Setup facoltativo che alimenta SessionStart, quindi un ciclo per turno contenente UserPromptSubmit, UserPromptExpansion per slash commands, il ciclo agentico annidato (PreToolUse, PermissionRequest, PostToolUse, PostToolUseFailure, PostToolBatch, SubagentStart/Stop, TaskCreated, TaskCompleted), e Stop o StopFailure, seguito da TeammateIdle, PreCompact, PostCompact e SessionEnd, con Elicitation e ElicitationResult annidati all'interno dell'esecuzione dello strumento MCP, PermissionDenied come ramo laterale di PermissionRequest per i rifiuti in modalità automatica, WorktreeCreate, WorktreeRemove, Notification, ConfigChange, InstructionsLoaded, CwdChanged e FileChanged come eventi asincroni autonomi, e MessageDisplay come evento di sola visualizzazione che viene eseguito mentre il testo del messaggio dell'assistente viene trasmesso in streaming" width="520" height="1228" data-path="images/hooks-lifecycle.svg" />
  </Frame>
</div>

La tabella seguente riassume quando si attiva ogni evento. La sezione [Hook events](#hook-events) documenta lo schema di input completo e le opzioni di controllo della decisione per ognuno.

| Event                 | When it fires                                                                                                                                                                                                                                         |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `SessionStart`        | When a session begins or resumes                                                                                                                                                                                                                      |
| `Setup`               | When you start Claude Code with `--init-only`, or with `--init` or `--maintenance` in `-p` mode. For one-time preparation in CI or scripts                                                                                                            |
| `UserPromptSubmit`    | When you submit a prompt, before Claude processes it                                                                                                                                                                                                  |
| `UserPromptExpansion` | When a user-typed command expands into a prompt, before it reaches Claude. Can block the expansion                                                                                                                                                    |
| `PreToolUse`          | Before a tool call executes. Can block it                                                                                                                                                                                                             |
| `PermissionRequest`   | When a tool call needs a permission decision                                                                                                                                                                                                          |
| `PermissionDenied`    | When auto mode denies a tool call, including denials without a classifier verdict. Use JSON `hookSpecificOutput.retry: true` to tell the model it may retry the denied tool call. Claude Code ignores `retry` when the classifier produced no verdict |
| `PostToolUse`         | After a tool call succeeds                                                                                                                                                                                                                            |
| `PostToolUseFailure`  | After a tool call fails                                                                                                                                                                                                                               |
| `PostToolBatch`       | After a full batch of parallel tool calls resolves, before the next model call                                                                                                                                                                        |
| `Notification`        | When Claude Code sends a notification                                                                                                                                                                                                                 |
| `MessageDisplay`      | While assistant message text is displayed                                                                                                                                                                                                             |
| `SubagentStart`       | When a subagent is spawned                                                                                                                                                                                                                            |
| `SubagentStop`        | When a subagent finishes                                                                                                                                                                                                                              |
| `TaskCreated`         | When a task is being created via `TaskCreate`                                                                                                                                                                                                         |
| `TaskCompleted`       | When a task is being marked as completed                                                                                                                                                                                                              |
| `Stop`                | When Claude finishes responding                                                                                                                                                                                                                       |
| `StopFailure`         | When the turn ends due to an API error                                                                                                                                                                                                                |
| `TeammateIdle`        | When an [agent team](/docs/en/agent-teams) teammate is about to go idle                                                                                                                                                                                    |
| `InstructionsLoaded`  | When a CLAUDE.md or `.claude/rules/*.md` file is loaded into context. Fires at session start and when files are lazily loaded during a session                                                                                                        |
| `ConfigChange`        | When a configuration file changes during a session                                                                                                                                                                                                    |
| `CwdChanged`          | When the working directory changes, for example when Claude executes a `cd` command. Useful for reactive environment management with tools like direnv                                                                                                |
| `DirectoryAdded`      | When a working directory is added mid-session via `/add-dir` or the SDK `register_repo_root` control request                                                                                                                                          |
| `FileChanged`         | When a watched file changes on disk. The `matcher` field specifies which filenames to watch                                                                                                                                                           |
| `WorktreeCreate`      | When a worktree is being created via `--worktree`, `isolation: "worktree"`, or for a background session. Replaces default git behavior                                                                                                                |
| `WorktreeRemove`      | When a worktree is being removed at session exit, when a subagent finishes, or when you delete a background session                                                                                                                                   |
| `PreCompact`          | Before context compaction                                                                                                                                                                                                                             |
| `PostCompact`         | After context compaction completes                                                                                                                                                                                                                    |
| `Elicitation`         | When an MCP server requests user input during a tool call                                                                                                                                                                                             |
| `ElicitationResult`   | After a user responds to an MCP elicitation, before the response is sent back to the server                                                                                                                                                           |
| `SessionEnd`          | When a session terminates                                                                                                                                                                                                                             |

<h3 id="how-a-hook-resolves">
  Come si risolve un hook
</h3>

Per vedere come questi elementi si combinano, considerare questo hook `PreToolUse` che blocca i comandi shell distruttivi. Il `matcher` si restringe alle chiamate dello strumento Bash e la condizione `if` si restringe ulteriormente ai comandi Bash che corrispondono a `rm *`, quindi `block-rm.sh` viene eseguito solo quando entrambi i filtri corrispondono:

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

Lo script legge l'input JSON da stdin, estrae il comando e restituisce una `permissionDecision` di `"deny"` se contiene `rm -rf`:

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

Supponiamo che Claude Code decida di eseguire `Bash "rm -rf /tmp/build"`. Ecco cosa accade:

<Frame>
  <img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/hook-resolution.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=be0bf3053550c26de5f54cd64674c197" alt="Diagramma della risoluzione del hook: PreToolUse si attiva, il matcher controlla la corrispondenza di Bash, la condizione if controlla la corrispondenza di Bash(rm *). Se entrambi corrispondono, il comando del hook viene eseguito e restituisce permissionDecision deny, quindi la chiamata dello strumento viene bloccata e Claude Code continua. Se uno dei controlli non corrisponde, l'hook viene saltato e la chiamata dello strumento è autorizzata a procedere." width="930" height="270" data-path="images/hook-resolution.svg" />
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

1. Scegliere un [hook event](#hook-events) a cui rispondere, come `PreToolUse` o `Stop`
2. Aggiungere un [matcher group](#matcher-patterns) per filtrare quando si attiva, come "solo per lo strumento Bash"
3. Definire uno o più [hook handlers](#hook-handler-fields) da eseguire quando corrisponde

Consultare [Come si risolve un hook](#how-a-hook-resolves) sopra per una procedura dettagliata completa con un esempio annotato.

<Note>
  Questa pagina utilizza termini specifici per ogni livello: **hook event** per il punto del ciclo di vita, **matcher group** per il filtro e **hook handler** per il comando shell, endpoint HTTP, strumento MCP, prompt o agente che viene eseguito. "Hook" da solo si riferisce alla funzionalità generale.
</Note>

<h3 id="hook-locations">
  Posizioni dei hook
</h3>

Il luogo in cui si definisce un hook determina il suo ambito:

| Posizione                                                 | Ambito                        | Condivisibile                             |
| :-------------------------------------------------------- | :---------------------------- | :---------------------------------------- |
| `~/.claude/settings.json`                                 | Tutti i progetti              | No, locale al computer                    |
| `.claude/settings.json`                                   | Singolo progetto              | Sì, può essere committato nel repository  |
| `.claude/settings.local.json`                             | Singolo progetto              | No, gitignored quando Claude Code lo crea |
| Impostazioni della politica gestita                       | Organizzazione intera         | Sì, controllato dall'amministratore       |
| [Plugin](/docs/it/plugins) `hooks/hooks.json`                  | Quando il plugin è abilitato  | Sì, fornito con il plugin                 |
| [Skill](/docs/it/skills) o [agent](/docs/it/sub-agents) frontmatter | Mentre il componente è attivo | Sì, definito nel file del componente      |

Per i dettagli sulla risoluzione del file di impostazioni, consultare [settings](/docs/it/settings). Gli amministratori aziendali possono utilizzare `allowManagedHooksOnly` per bloccare i hook dell'utente, del progetto e del plugin. Gli hook dai plugin forzatamente abilitati nelle impostazioni gestite `enabledPlugins` sono esenti, quindi gli amministratori possono distribuire hook verificati attraverso un marketplace dell'organizzazione. Consultare [Hook configuration](/docs/it/settings#hook-configuration).

<h3 id="matcher-patterns">
  Modelli di matcher
</h3>

Il campo `matcher` filtra quando gli hook si attivano. Come viene valutato un matcher dipende dai caratteri che contiene:

| Valore del matcher                               | Valutato come                                                                                              | Esempio                                                                                                                                                                                        |
| :----------------------------------------------- | :--------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"*"`, `""` o omesso                             | Corrisponde a tutti                                                                                        | si attiva ad ogni occorrenza dell'evento                                                                                                                                                       |
| Solo lettere, cifre, `_`, `-`, spazi, `,` e `\|` | Stringa esatta, o elenco di stringhe esatte separate da `\|` o `,` con spazi bianchi opzionali circostanti | `Bash` corrisponde solo allo strumento Bash; `Edit\|Write` e `Edit, Write` corrispondono ciascuno a entrambi gli strumenti esattamente; `code-reviewer` corrisponde solo a quel tipo di agente |
| Contiene qualsiasi altro carattere               | Espressione regolare JavaScript, non ancorata                                                              | `^Notebook` corrisponde a qualsiasi strumento il cui nome inizia con Notebook; `mcp__memory__.*` corrisponde a ogni strumento dal server `memory`                                              |

Un matcher sul percorso dell'espressione regolare viene testato con `RegExp.prototype.test` di JavaScript, che ha successo su una corrispondenza in qualsiasi punto del valore. `Edit.*` corrisponde sia a `Edit` che a `NotebookEdit`; racchiudere il modello in `^` e `$`, come in `^Edit$`, quando è necessaria una corrispondenza di intera stringa.

I separatori di virgola e la tolleranza dello spazio bianco circostante richiedono Claude Code v2.1.191 o successivo.

I trattini nel set di corrispondenza esatta richiedono Claude Code v2.1.195 o successivo. Nelle versioni precedenti un nome con trattino come `code-reviewer` viene valutato come un'espressione regolare non ancorata, quindi si attiva anche per `senior-code-reviewer`; ancorarlo come `^code-reviewer$` su quelle versioni per corrispondere solo a quel nome.

`FileChanged` e `StopFailure` utilizzano un set di corrispondenza esatta più ristretto di sole lettere, cifre, `_` e `|`. Un trattino, uno spazio o una virgola in un matcher per questi due eventi lo mantiene sul percorso dell'espressione regolare, e solo `|` separa le alternative. Ogni altro evento con supporto matcher nella tabella che segue accetta `|` o `,`.

L'evento `FileChanged` non segue queste regole quando costruisce il suo elenco di osservazione. Consultare [FileChanged](#filechanged).

Ogni tipo di evento corrisponde a un campo diverso:

| Evento                                                                                                                                            | Su cosa filtra il matcher                                                    | Valori matcher di esempio                                                                                                                                                           |
| :------------------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`                                                        | nome dello strumento                                                         | `Bash`, `Edit\|Write`, `mcp__.*`                                                                                                                                                    |
| `SessionStart`                                                                                                                                    | come è iniziata la sessione                                                  | `startup`, `resume`, `clear`, `compact`                                                                                                                                             |
| `Setup`                                                                                                                                           | quale flag CLI ha attivato la configurazione                                 | `init`, `maintenance`                                                                                                                                                               |
| `SessionEnd`                                                                                                                                      | perché è terminata la sessione                                               | `clear`, `resume`, `logout`, `prompt_input_exit`, `bypass_permissions_disabled`, `other`                                                                                            |
| `Notification`                                                                                                                                    | tipo di notifica                                                             | `permission_prompt`, `idle_prompt`, `auth_success`, `elicitation_dialog`, `elicitation_complete`, `elicitation_response`, `agent_needs_input`, `agent_completed`                    |
| `SubagentStart`                                                                                                                                   | tipo di agente                                                               | `general-purpose`, `Explore`, `Plan`, nomi di agenti personalizzati, o nomi con ambito plugin come `^my-plugin:reviewer$`                                                           |
| `PreCompact`, `PostCompact`                                                                                                                       | cosa ha attivato la compattazione                                            | `manual`, `auto`                                                                                                                                                                    |
| `SubagentStop`                                                                                                                                    | tipo di agente                                                               | stessi valori di `SubagentStart`                                                                                                                                                    |
| `ConfigChange`                                                                                                                                    | fonte di configurazione                                                      | `user_settings`, `project_settings`, `local_settings`, `policy_settings`, `skills`                                                                                                  |
| `CwdChanged`                                                                                                                                      | nessun supporto matcher                                                      | si attiva sempre ad ogni cambio di directory                                                                                                                                        |
| `FileChanged`                                                                                                                                     | nomi di file letterali da osservare (consultare [FileChanged](#filechanged)) | `.envrc\|.env`                                                                                                                                                                      |
| `StopFailure`                                                                                                                                     | tipo di errore                                                               | `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `unknown` |
| `InstructionsLoaded`                                                                                                                              | motivo del caricamento                                                       | `session_start`, `nested_traversal`, `path_glob_match`, `include`, `compact`                                                                                                        |
| `UserPromptExpansion`                                                                                                                             | nome del comando                                                             | i nomi della skill o del comando                                                                                                                                                    |
| `Elicitation`                                                                                                                                     | nome del server MCP                                                          | i nomi dei server MCP configurati                                                                                                                                                   |
| `ElicitationResult`                                                                                                                               | nome del server MCP                                                          | stessi valori di `Elicitation`                                                                                                                                                      |
| `UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `MessageDisplay` | nessun supporto matcher                                                      | si attiva sempre ad ogni occorrenza                                                                                                                                                 |

Il matcher viene eseguito su un campo dall'[input JSON](#hook-input-and-output) che Claude Code invia al hook su stdin. Per gli eventi degli strumenti, quel campo è `tool_name`. Ogni sezione [hook event](#hook-events) elenca l'insieme completo di valori matcher e lo schema di input per quell'evento.

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

`UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `MessageDisplay` e `CwdChanged` non supportano i matcher e si attivano sempre ad ogni occorrenza. Se si aggiunge un campo `matcher` a questi eventi, viene silenziosamente ignorato.

Per gli eventi degli strumenti, è possibile filtrare più strettamente impostando il campo [`if`](#common-fields) sui singoli gestori del hook. `if` utilizza la [sintassi delle regole di autorizzazione](/docs/it/permissions) per corrispondere al nome dello strumento e agli argomenti insieme, quindi `"Bash(git *)"` viene eseguito quando qualsiasi sottocomando dell'input Bash corrisponde a `git *` e `"Edit(*.ts)"` viene eseguito solo per i file TypeScript.

<h4 id="match-mcp-tools">
  Corrispondere ai strumenti MCP
</h4>

Gli strumenti del server [MCP](/docs/it/mcp) appaiono come strumenti regolari negli eventi degli strumenti (`PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`), quindi è possibile farvi corrispondere lo stesso modo in cui si fa corrispondere qualsiasi altro nome di strumento.

Gli strumenti MCP seguono il modello di denominazione `mcp__<server>__<tool>`, ad esempio:

* `mcp__memory__create_entities`: strumento create entities del server Memory
* `mcp__filesystem__read_file`: strumento read file del server Filesystem
* `mcp__github__search_repositories`: strumento search del server GitHub

Per corrispondere a ogni strumento da un server, aggiungere `.*` al prefisso del server. `.*` è obbligatorio: un matcher come `mcp__memory` o `mcp__brave-search` contiene solo caratteri di corrispondenza esatta, quindi viene confrontato come stringa esatta e non corrisponde a nessuno strumento.

* `mcp__memory__.*` corrisponde a tutti gli strumenti dal server `memory`
* `mcp__brave-search__.*` corrisponde a tutti gli strumenti da un server il cui nome contiene un trattino
* `mcp__.*__write.*` corrisponde a qualsiasi strumento il cui nome inizia con `write` da qualsiasi server

I trattini nel set di corrispondenza esatta richiedono Claude Code v2.1.195 o successivo. Nelle versioni precedenti un prefisso semplice con trattino come `mcp__brave-search` viene valutato come un'espressione regolare non ancorata e corrisponde a ogni strumento da quel server. La forma `mcp__brave-search__.*` funziona su ogni versione.

Gli strumenti da un [server MCP fornito da plugin](/docs/it/mcp#plugin-provided-mcp-servers) utilizzano un segmento server con ambito che include il nome del plugin: `mcp__plugin_<plugin-name>_<server-name>__<tool>`. Un matcher scritto rispetto alla chiave del server semplice non si attiva mai per questi strumenti. Per un plugin denominato `my-plugin` che fornisce un server con la chiave `db`, uno strumento `query` appare come `mcp__plugin_my-plugin_db__query`, quindi il matcher per ogni strumento da quel server è `mcp__plugin_my-plugin_db__.*`. Utilizzare lo stesso nome dello strumento con ambito nel campo [`if`](#common-fields) di un gestore. Consultare [Plugin-provided MCP servers](/docs/it/mcp#plugin-provided-mcp-servers) per come viene costruito il nome con ambito.

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
  Campi del gestore del hook
</h3>

Ogni oggetto nell'array `hooks` interno è un gestore del hook: il comando shell, endpoint HTTP, strumento MCP, prompt LLM o agente che viene eseguito quando il matcher corrisponde. Ci sono cinque tipi:

* **[Command hooks](#command-hook-fields)** (`type: "command"`): eseguono un comando shell. Lo script riceve l'[input JSON](#hook-input-and-output) dell'evento su stdin e comunica i risultati attraverso codici di uscita e stdout.
* **[HTTP hooks](#http-hook-fields)** (`type: "http"`): inviano l'input JSON dell'evento come richiesta HTTP POST a un URL. L'endpoint comunica i risultati attraverso il corpo della risposta utilizzando lo stesso [formato JSON di output](#json-output) dei command hook.
* **[MCP tool hooks](#mcp-tool-hook-fields)** (`type: "mcp_tool"`): chiamano uno strumento su un [server MCP](/docs/it/mcp) già connesso. L'output di testo dello strumento viene trattato come stdout del command hook.
* **[Prompt hooks](#prompt-and-agent-hook-fields)** (`type: "prompt"`): inviano un prompt a un modello Claude per la valutazione a turno singolo. Il modello restituisce una decisione sì/no come JSON. Consultare [Prompt-based hooks](#prompt-based-hooks).
* **[Agent hooks](#prompt-and-agent-hook-fields)** (`type: "agent"`): generano un subagent che può utilizzare strumenti come Read, Grep e Glob per verificare le condizioni prima di restituire una decisione. Gli agent hook sono sperimentali e potrebbero cambiare. Consultare [Agent-based hooks](#agent-based-hooks).

Tutti gli hook corrispondenti vengono eseguiti in parallelo e i gestori identici vengono automaticamente deduplicati. I command hook vengono deduplicati per stringa di comando e `args`, e gli HTTP hook vengono deduplicati per URL.

I gestori vengono eseguiti nella directory corrente con l'ambiente di Claude Code. La variabile di ambiente `$CLAUDE_CODE_REMOTE` è impostata su `"true"` negli ambienti web remoti e non è impostata nella CLI locale. A partire da v2.1.199, [`$CLAUDE_CODE_BRIDGE_SESSION_ID`](/docs/it/env-vars) è impostato sull'ID della sessione [Remote Control](/docs/it/remote-control) mentre la sessione locale ha una connessione Remote Control attiva.

<h4 id="common-fields">
  Campi comuni
</h4>

Questi campi si applicano a tutti i tipi di hook:

| Campo           | Obbligatorio | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :-------------- | :----------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`          | sì           | `"command"`, `"http"`, `"mcp_tool"`, `"prompt"` o `"agent"`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `if`            | no           | Sintassi della regola di autorizzazione per filtrare quando questo hook viene eseguito, come `"Bash(git *)"` o `"Edit(*.ts)"`. L'hook viene eseguito solo se la chiamata dello strumento corrisponde al modello. Consultare la [tabella di corrispondenza Bash](#bash-if-matching) di seguito per come i modelli Bash si valutano rispetto ai sottocomandi, `$()` e backtick. Valutato solo su eventi degli strumenti: `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` e `PermissionDenied`. Su altri eventi, un hook con `if` impostato non viene mai eseguito. Utilizza la stessa sintassi delle [regole di autorizzazione](/docs/it/permissions) |
| `timeout`       | no           | Secondi prima dell'annullamento. Impostazioni predefinite: 600 per `command`, `http` e `mcp_tool`; 30 per `prompt`; 60 per `agent`. [`UserPromptSubmit`](#userpromptsubmit) abbassa l'impostazione predefinita di `command`, `http` e `mcp_tool` a 30, e [`MessageDisplay`](#messagedisplay) la abbassa a 10                                                                                                                                                                                                                                                                                                                                                          |
| `statusMessage` | no           | Messaggio spinner personalizzato visualizzato mentre l'hook viene eseguito                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `once`          | no           | Se `true`, viene eseguito una sola volta per sessione e poi rimosso. Solo onorato per gli hook dichiarati nel [frontmatter della skill](#hooks-in-skills-and-agents); ignorato nei file di impostazioni e nel frontmatter dell'agente                                                                                                                                                                                                                                                                                                                                                                                                                                 |

Il campo `if` contiene esattamente una regola di autorizzazione. Non esiste sintassi `&&`, `||` o di elenco per combinare le regole; per applicare più condizioni, definire un gestore del hook separato per ciascuna.

<span id="bash-if-matching" />Per i modelli Bash, se il comando hook viene eseguito dipende dalla forma del modello e dal comando Bash che Claude sta invocando. Gli assegnamenti `VAR=value` iniziali vengono rimossi prima della corrispondenza.

| Modello `if`       | Comando Bash           | Hook viene eseguito? | Perché                                                                                                  |
| :----------------- | :--------------------- | :------------------- | :------------------------------------------------------------------------------------------------------ |
| `Bash(git *)`      | `FOO=bar git push`     | sì                   | gli assegnamenti iniziali vengono rimossi; `git push` corrisponde                                       |
| `Bash(git *)`      | `npm test && git push` | sì                   | ogni sottocomando viene controllato; `git push` corrisponde                                             |
| `Bash(rm *)`       | `echo $(rm -rf /)`     | sì                   | i comandi dentro `$()` e backtick vengono controllati; `rm -rf /` corrisponde                           |
| `Bash(rm *)`       | `echo $(date)`         | no                   | nessun sottocomando corrisponde a `rm *`                                                                |
| `Bash(git push *)` | `echo $(date)`         | sì                   | i modelli che specificano più del nome del comando eseguono l'hook comunque su `$()`, backtick o `$VAR` |

Il filtro fallisce anche in modo aperto, eseguendo l'hook indipendentemente dal modello, quando il comando Bash non può essere analizzato. Poiché il filtro `if` è best-effort, utilizzare il [sistema di autorizzazione](/docs/it/permissions) piuttosto che un hook per applicare un'autorizzazione o un diniego rigido.

<h4 id="command-hook-fields">
  Campi del command hook
</h4>

Oltre ai [campi comuni](#common-fields), i command hook accettano questi campi:

| Campo         | Obbligatorio | Descrizione                                                                                                                                                                                                                                                                                                                                                                                  |
| :------------ | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`     | sì           | Comando shell da eseguire. Con `args`, l'eseguibile da generare direttamente. Consultare [Exec form e shell form](#exec-form-and-shell-form)                                                                                                                                                                                                                                                 |
| `args`        | no           | Elenco di argomenti. Quando presente, `command` viene risolto come eseguibile e generato direttamente con `args` come vettore di argomenti, senza shell. Consultare [Exec form e shell form](#exec-form-and-shell-form)                                                                                                                                                                      |
| `async`       | no           | Se `true`, viene eseguito in background senza bloccare. Consultare [Run hooks in the background](#run-hooks-in-the-background)                                                                                                                                                                                                                                                               |
| `asyncRewake` | no           | Se `true`, viene eseguito in background e riattiva Claude su codice di uscita 2. Implica `async`. Lo stderr del hook, o stdout se stderr è vuoto, viene mostrato a Claude come promemoria di sistema in modo che possa reagire a un guasto in background a lunga esecuzione                                                                                                                  |
| `shell`       | no           | Shell da utilizzare per questo hook. Accetta `"bash"` o `"powershell"`. Impostazione predefinita: `"bash"`, o `"powershell"` su Windows quando Git Bash non è installato. L'impostazione `"powershell"` esegue il comando tramite PowerShell su Windows. Non richiede `CLAUDE_CODE_USE_POWERSHELL_TOOL` poiché gli hook generano PowerShell direttamente. Ignorato quando `args` è impostato |

<a id="exec-form-and-shell-form" />

<h5 id="exec-form-and-shell-form">
  Exec form e shell form
</h5>

Un command hook viene eseguito come exec form quando `args` è impostato, e come shell form quando `args` è omesso. Impostare `args` ogni volta che l'hook fa riferimento a un [segnaposto di percorso](#reference-scripts-by-path), poiché ogni elemento viene passato come un argomento senza virgolette. Omettere `args` quando è necessario utilizzare funzionalità shell come pipe o `&&`, o quando nessuno dei due problemi si applica.

**Exec form** viene eseguito quando `args` è presente. Claude Code risolve `command` come eseguibile su `PATH` e lo genera direttamente con `args` come vettore di argomenti. Non c'è shell, quindi ogni elemento `args` è un argomento esattamente come scritto, e i segnaposti di percorso come `${CLAUDE_PLUGIN_ROOT}` vengono sostituiti in `command` e in ogni elemento `args` come stringhe semplici. I caratteri speciali come apostrofi, `$` e backtick passano attraverso verbatim perché non c'è shell per interpretarli. Non avviene alcuna tokenizzazione shell su nessuna piattaforma.

**Shell form** viene eseguito quando `args` è assente. La stringa `command` viene passata a una shell: `sh -c` su macOS e Linux, Git Bash su Windows, o PowerShell quando Git Bash non è installato. Impostare il campo `shell` per scegliere esplicitamente. La shell tokenizza la stringa, espande le variabili e interpreta pipe, `&&`, reindirizzamenti e glob.

<Note>
  Su Windows, exec form richiede che `command` si risolva in un vero eseguibile come `.exe`. Gli shim `.cmd` e `.bat` che npm, npx, eslint e altri strumenti installano in `node_modules/.bin` non sono eseguibili e non possono essere generati senza una shell. Per eseguirli in exec form, invocare lo script sottostante con `node` direttamente, ad esempio `"command": "node", "args": ["${CLAUDE_PLUGIN_ROOT}/node_modules/eslint/bin/eslint.js"]`. Il modello `node` più percorso-script funziona su ogni piattaforma perché `node.exe` è un vero binario. Per eseguire uno shim `.cmd` o `.bat` per nome, utilizzare shell form.
</Note>

Questo esempio esegue uno script Node fornito con un plugin. Exec form passa il percorso dello script risolto come un argomento senza virgolette:

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

Entrambe le forme supportano gli stessi [segnaposti di percorso](#reference-scripts-by-path), ed entrambi li esportano come variabili di ambiente `CLAUDE_PROJECT_DIR`, `CLAUDE_PLUGIN_ROOT` e `CLAUDE_PLUGIN_DATA` sul processo generato, quindi uno script può leggere `process.env.CLAUDE_PLUGIN_ROOT` indipendentemente da come è stato lanciato. Gli hook del plugin inoltre sostituiscono i valori [`${user_config.*}`](/docs/it/plugins-reference#user-configuration), in exec form solo: il valore viene sostituito in `command` e in ogni elemento `args` come stringa semplice, quindi nessuna shell lo rianalizza.

Un hook del plugin in shell form il cui `command` fa riferimento a `${user_config.*}` fallisce con un [errore](/docs/it/errors#plugin-command-references-user-config) invece di eseguirsi. Per utilizzare un valore di opzione da un hook in shell form, leggere la variabile di ambiente `$CLAUDE_PLUGIN_OPTION_<KEY>`, come `$CLAUDE_PLUGIN_OPTION_WEBHOOK_URL` per un'opzione `webhook_url`, oppure impostare `args` per passare l'hook a exec form. Prima di v2.1.207, i comandi degli hook del plugin in shell form sostituivano anche `${user_config.*}`.

<Note>
  In exec form, `command` è solo il nome o il percorso dell'eseguibile. Se `command` è un nome semplice senza separatore di percorso e contiene spazi bianchi insieme a `args`, Claude Code registra un avviso perché la generazione avrà esito negativo: non esiste un eseguibile denominato `node script.js`. Spostare i token extra in `args`. I percorsi assoluti con spazi, come `C:\Program Files\nodejs\node.exe`, sono un singolo eseguibile valido e non attivano l'avviso.
</Note>

<h4 id="http-hook-fields">
  Campi del HTTP hook
</h4>

Oltre ai [campi comuni](#common-fields), gli HTTP hook accettano questi campi:

| Campo            | Obbligatorio | Descrizione                                                                                                                                                                                                                                                  |
| :--------------- | :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url`            | sì           | URL a cui inviare la richiesta POST                                                                                                                                                                                                                          |
| `headers`        | no           | Intestazioni HTTP aggiuntive come coppie chiave-valore. I valori supportano l'interpolazione delle variabili di ambiente utilizzando la sintassi `$VAR_NAME` o `${VAR_NAME}`. Solo le variabili elencate in `allowedEnvVars` vengono risolte                 |
| `allowedEnvVars` | no           | Elenco dei nomi delle variabili di ambiente che possono essere interpolate nei valori dell'intestazione. I riferimenti alle variabili non elencate vengono sostituiti con stringhe vuote. Obbligatorio per qualsiasi interpolazione di variabili di ambiente |

Claude Code invia l'[input JSON](#hook-input-and-output) del hook come corpo della richiesta POST con `Content-Type: application/json`. Il corpo della risposta utilizza lo stesso [formato JSON di output](#json-output) dei command hook.

La gestione degli errori differisce dai command hook: le risposte non-2xx, i guasti di connessione e i timeout producono tutti errori non bloccanti che consentono l'esecuzione di continuare. Per bloccare una chiamata dello strumento o negare un'autorizzazione, restituire una risposta 2xx con un corpo JSON contenente `decision: "block"` o un `hookSpecificOutput` con `permissionDecision: "deny"`.

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
  Campi del MCP tool hook
</h4>

Oltre ai [campi comuni](#common-fields), gli MCP tool hook accettano questi campi:

| Campo    | Obbligatorio | Descrizione                                                                                                                                                                                                                                                                                                                            |
| :------- | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `server` | sì           | Nome di un server MCP configurato. Per un [server fornito da plugin](/docs/it/mcp#plugin-provided-mcp-servers), questo è il nome con ambito `plugin:<plugin-name>:<server-name>`, come `plugin:my-plugin:db`, non la chiave del server semplice. Il server deve essere già connesso; l'hook non attiva mai un flusso OAuth o di connessione |
| `tool`   | sì           | Nome dello strumento da chiamare su quel server                                                                                                                                                                                                                                                                                        |
| `input`  | no           | Argomenti passati allo strumento. I valori stringa supportano la sostituzione `${path}` dall'[input JSON](#hook-input-and-output) del hook, come `"${tool_input.file_path}"`                                                                                                                                                           |

L'output di testo dello strumento viene trattato come stdout del command hook: se analizzato come [output JSON](#json-output) valido viene elaborato come una decisione, altrimenti viene mostrato come testo semplice. Se il server denominato non è connesso, o lo strumento restituisce `isError: true`, l'hook produce un errore non bloccante e l'esecuzione continua.

Gli MCP tool hook sono disponibili su ogni hook event una volta che Claude Code si è connesso ai server MCP. `SessionStart` e `Setup` in genere si attivano prima che i server finiscano di connettersi, quindi gli hook su questi eventi dovrebbero aspettarsi l'errore "not connected" alla prima esecuzione.

Questo esempio chiama lo strumento `security_scan` sul server MCP `my_server` dopo ogni `Write` o `Edit`, passando il percorso del file modificato:

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

<h4 id="prompt-and-agent-hook-fields">
  Campi del prompt hook e agent hook
</h4>

Oltre ai [campi comuni](#common-fields), i prompt hook e agent hook accettano questi campi:

| Campo    | Obbligatorio | Descrizione                                                                                                                                                                                                          |
| :------- | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt` | sì           | Testo del prompt da inviare al modello. Utilizzare `$ARGUMENTS` come segnaposto per l'input JSON del hook. Sfuggire con una barra rovesciata per includere testo letterale: `\$1.00` viene visualizzato come `$1.00` |
| `model`  | no           | Modello da utilizzare per la valutazione. Impostazione predefinita: un modello veloce                                                                                                                                |

<h3 id="reference-scripts-by-path">
  Fare riferimento agli script per percorso
</h3>

Utilizzare questi segnaposti per fare riferimento agli script del hook relativi alla radice del progetto o del plugin, indipendentemente dalla directory di lavoro quando l'hook viene eseguito:

* `${CLAUDE_PROJECT_DIR}`: la radice del progetto. Claude Code inoltre imposta questa variabile nell'ambiente dei [server MCP stdio](/docs/it/mcp#option-3-add-a-local-stdio-server) e dei server LSP del plugin.
* `${CLAUDE_PLUGIN_ROOT}`: la directory di installazione del plugin, per gli script forniti con un [plugin](/docs/it/plugins). Cambia ad ogni aggiornamento del plugin.
* `${CLAUDE_PLUGIN_DATA}`: la [directory di dati persistenti](/docs/it/plugins-reference#persistent-data-directory) del plugin, per le dipendenze e lo stato che dovrebbero sopravvivere agli aggiornamenti del plugin.

Preferire [exec form](#exec-form-and-shell-form) per qualsiasi hook che faccia riferimento a un segnaposto di percorso. Exec form passa ogni elemento `args` come un argomento senza tokenizzazione shell, quindi i percorsi con spazi o caratteri speciali non hanno bisogno di virgolette. In shell form, racchiudere ogni segnaposto tra virgolette doppie.

<Tabs>
  <Tab title="Script del progetto">
    Questo esempio utilizza `${CLAUDE_PROJECT_DIR}` per eseguire un controllo dello stile dalla directory `.claude/hooks/` del progetto dopo qualsiasi chiamata dello strumento `Write` o `Edit`:

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

  <Tab title="Script del plugin">
    Definire i hook del plugin in `hooks/hooks.json` con un campo `description` facoltativo di livello superiore. Quando un plugin è abilitato, i suoi hook si uniscono ai hook dell'utente e del progetto.

    Questo esempio esegue uno script di formattazione fornito con il plugin:

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

    Consultare il [plugin components reference](/docs/it/plugins-reference#hooks) per i dettagli sulla creazione dei hook del plugin.
  </Tab>
</Tabs>

<h3 id="hooks-in-skills-and-agents">
  Hook in skills e agents
</h3>

Oltre ai file di impostazioni e ai plugin, gli hook possono essere definiti direttamente in [skills](/docs/it/skills) e [subagents](/docs/it/sub-agents) utilizzando il frontmatter. Questi hook sono limitati al ciclo di vita del componente e vengono eseguiti solo quando quel componente è attivo.

Tutti gli hook event sono supportati. Per i subagent, gli hook `Stop` vengono automaticamente convertiti in `SubagentStop` poiché questo è l'evento che si attiva quando un subagent termina.

Gli hook utilizzano lo stesso formato di configurazione dei hook basati su impostazioni ma sono limitati alla durata del componente e vengono puliti quando termina.

Questa skill definisce un hook `PreToolUse` che esegue uno script di convalida della sicurezza prima di ogni comando `Bash`:

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

Gli agenti utilizzano lo stesso formato nel loro frontmatter YAML.

<h3 id="the-/hooks-menu">
  Il menu `/hooks`
</h3>

Digitare `/hooks` in Claude Code per aprire un browser di sola lettura per i hook configurati. Il menu mostra ogni hook event con un conteggio dei hook configurati, consente di approfondire i matcher e mostra i dettagli completi di ogni gestore del hook. Utilizzarlo per verificare la configurazione, controllare da quale file di impostazioni proviene un hook o ispezionare il comando, il prompt o l'URL di un hook.

Il menu visualizza tutti e cinque i tipi di hook: `command`, `prompt`, `agent`, `http` e `mcp_tool`. Ogni hook è etichettato con un prefisso `[type]` e una fonte che indica dove è stato definito:

* `User`: da `~/.claude/settings.json`
* `Project`: da `.claude/settings.json`
* `Local`: da `.claude/settings.local.json`
* `Plugin`: da `hooks/hooks.json` di un plugin
* `Session`: registrato in memoria per la sessione corrente
* `Built-in`: registrato internamente da Claude Code

Selezionando un hook si apre una vista dettagliata che mostra il suo evento, matcher, tipo, file di origine e il comando, il prompt o l'URL completo. Il menu è di sola lettura: per aggiungere, modificare o rimuovere i hook, modificare il JSON delle impostazioni direttamente o chiedere a Claude di fare la modifica.

<h3 id="disable-or-remove-hooks">
  Disabilitare o rimuovere i hook
</h3>

Per rimuovere un hook, eliminare la sua voce dal file di impostazioni JSON.

Per disabilitare temporaneamente tutti gli hook senza rimuoverli, impostare `"disableAllHooks": true` nel file di impostazioni. Non c'è modo di disabilitare un singolo hook mantenendolo nella configurazione.

L'impostazione `disableAllHooks` rispetta la gerarchia delle impostazioni gestite. Se un amministratore ha configurato i hook attraverso le impostazioni della politica gestita, `disableAllHooks` impostato nelle impostazioni dell'utente, del progetto o locali non può disabilitare quei hook gestiti. Solo `disableAllHooks` impostato a livello di impostazioni gestite può disabilitare i hook gestiti.

Le modifiche dirette ai hook nei file di impostazioni vengono normalmente acquisite automaticamente dal file watcher.

<h2 id="hook-input-and-output">
  Input e output del hook
</h2>

I command hook ricevono dati JSON tramite stdin e comunicano i risultati attraverso codici di uscita, stdout e stderr. Gli HTTP hook ricevono lo stesso JSON come corpo della richiesta POST e comunicano i risultati attraverso il corpo della risposta HTTP. Questa sezione copre i campi e il comportamento comuni a tutti gli eventi. Ogni sezione dell'evento sotto [Hook events](#hook-events) include il suo schema di input specifico e le opzioni di controllo della decisione.

Su macOS e Linux, i command hook vengono eseguiti nella loro propria sessione senza un terminale di controllo a partire da v2.1.139. Il processo hook e qualsiasi processo figlio non possono aprire `/dev/tty` o inviare sequenze di escape direttamente all'interfaccia Claude Code. Windows non ha `/dev/tty`. Per visualizzare un messaggio all'utente su qualsiasi piattaforma, restituire [`systemMessage`](#json-output) nell'output JSON. Per attivare una notifica desktop, impostare un titolo della finestra o suonare il campanello, restituire [`terminalSequence`](#emit-terminal-notifications) invece.

<h3 id="common-input-fields">
  Campi di input comuni
</h3>

Gli hook event ricevono questi campi come JSON, oltre ai campi specifici dell'evento documentati in ogni sezione [hook event](#hook-events). Per i command hook, questo JSON arriva tramite stdin. Per gli HTTP hook, arriva come corpo della richiesta POST.

| Campo             | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| :---------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `session_id`      | Identificatore della sessione corrente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `prompt_id`       | UUID che identifica il prompt dell'utente attualmente in elaborazione. Corrisponde all'attributo [`prompt.id` sugli eventi OpenTelemetry](/docs/it/monitoring-usage#event-correlation-attributes), quindi è possibile correlare l'output del hook con la telemetria per un singolo prompt. Assente fino al primo input dell'utente. Richiede Claude Code v2.1.196 o successivo                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `transcript_path` | Percorso al JSON della conversazione. Il file della trascrizione viene scritto in modo asincrono e potrebbe rimanere indietro rispetto alla conversazione in memoria, quindi potrebbe non includere ancora i messaggi più recenti del turno corrente quando un hook si attiva. Gli hook che necessitano del testo dell'assistente finale del turno corrente dovrebbero utilizzare `last_assistant_message` su [Stop](#stop) e [SubagentStop](#subagentstop) invece di leggere la trascrizione                                                                                                                                                                                                                                                                                                                                                       |
| `cwd`             | Directory di lavoro corrente quando l'hook viene invocato                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `permission_mode` | [Modalità di autorizzazione](/docs/it/permissions#permission-modes) corrente: `"default"`, `"plan"`, `"acceptEdits"`, `"auto"`, `"dontAsk"` o `"bypassPermissions"`. La modalità etichettata **Manual** arriva come `"default"`, mai come `"manual"`, quindi gli script che corrispondono a `"default"` continuano a funzionare. Non tutti gli eventi ricevono questo campo. Controllare l'esempio JSON in ogni sezione [hook event](#hook-events)                                                                                                                                                                                                                                                                                                                                                                                                       |
| `effort`          | Oggetto con un campo `level` che contiene il [livello di effort](/docs/it/model-config#adjust-effort-level) attivo per il turno: `"low"`, `"medium"`, `"high"`, `"xhigh"` o `"max"`. Se l'effort richiesto del modello supera quello supportato dal modello corrente, questo è il livello ridotto che il modello ha effettivamente utilizzato. Ultracode non è un livello distinto e viene segnalato come `"xhigh"`. L'oggetto corrisponde al campo `effort` della [riga di stato](/docs/it/statusline#available-data). Presente per gli eventi che si attivano all'interno di un contesto di utilizzo dello strumento, come `PreToolUse`, `PostToolUse`, `Stop` e `SubagentStop`, quando il modello corrente supporta il parametro effort. Il livello è disponibile anche ai comandi hook e allo strumento Bash come variabile di ambiente `$CLAUDE_EFFORT`. |
| `hook_event_name` | Nome dell'evento che si è attivato                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

Quando si esegue con `--agent` o all'interno di un subagent, vengono inclusi due campi aggiuntivi:

| Campo        | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `agent_id`   | Identificatore univoco per il subagent. Presente solo quando l'hook si attiva all'interno di una chiamata di subagent. Utilizzare questo per distinguere le chiamate del hook del subagent dalle chiamate del thread principale.                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `agent_type` | Nome dell'agente (ad esempio, `"Explore"` o `"security-reviewer"`). Presente quando la sessione utilizza `--agent` o l'hook si attiva all'interno di un subagent. Per i subagent, il tipo del subagent ha la precedenza sul valore `--agent` della sessione. Per i [subagent personalizzati](/docs/it/sub-agents), questo è il campo `name` dal frontmatter dell'agente, non il nome del file. Per i subagent forniti da un [plugin](/docs/it/plugins), questo è l'identificatore con ambito plugin come `my-plugin:reviewer`, non il nome del frontmatter semplice. Consultare [SubagentStart](#subagentstart) per come scrivere un matcher rispetto a un nome con ambito plugin. |

Solo gli hook [`SessionStart`](#sessionstart) possono ricevere un campo `model`, e non è garantito che sia presente. Non esiste una variabile di ambiente `$CLAUDE_MODEL`. Un processo hook eredita l'ambiente padre, quindi può leggere `$ANTHROPIC_MODEL` se lo imposti nella tua shell, ma quel valore non cambia quando cambi modelli con `/model` durante una sessione. Un insieme di variabili non viene ereditato: Claude Code [rimuove le variabili dell'esportatore `OTEL_*` da ogni sottoprocesso che genera](/docs/it/monitoring-usage#administrator-configuration), inclusi gli hook.

Ad esempio, un hook `PreToolUse` per un comando Bash riceve questo su stdin:

```json theme={null}
{
  "session_id": "abc123",
  "prompt_id": "550e8400-e29b-41d4-a716-446655440000",
  "transcript_path": "/home/user/.claude/projects/.../transcript.jsonl",
  "cwd": "/home/user/my-project",
  "permission_mode": "default",
  "hook_event_name": "PreToolUse",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test"
  }
}
```

I campi `tool_name` e `tool_input` sono specifici dell'evento. Ogni sezione [hook event](#hook-events) documenta i campi aggiuntivi per quell'evento.

<h3 id="exit-code-output">
  Output del codice di uscita
</h3>

Il codice di uscita dal comando del hook dice a Claude Code se l'azione deve procedere, essere bloccata o essere ignorata.

**Exit 0** significa successo. Claude Code analizza stdout per i [campi di output JSON](#json-output). L'output JSON viene elaborato solo su exit 0. Per la maggior parte degli eventi, stdout viene scritto nel log di debug ma non mostrato nella trascrizione. Le eccezioni sono `UserPromptSubmit`, `UserPromptExpansion` e `SessionStart`, dove stdout viene aggiunto come contesto che Claude può vedere e su cui agire.

**Exit 2** significa un errore bloccante. Claude Code ignora stdout e qualsiasi JSON in esso. Invece, il testo stderr viene restituito a Claude come messaggio di errore. L'effetto dipende dall'evento: `PreToolUse` blocca la chiamata dello strumento, `UserPromptSubmit` rifiuta il prompt e così via. Consultare [exit code 2 behavior](#exit-code-2-behavior-per-event) per l'elenco completo.

**Qualsiasi altro codice di uscita** è un errore non bloccante per la maggior parte degli eventi hook. La trascrizione mostra un avviso `<hook name> hook error` seguito dalla prima riga di stderr, in modo da poter identificare la causa senza `--debug`. L'esecuzione continua e lo stderr completo viene scritto nel log di debug.

Ad esempio, uno script di comando hook che blocca i comandi Bash pericolosi:

```bash theme={null}
#!/bin/bash
# Legge l'input JSON da stdin, controlla il comando
command=$(jq -r '.tool_input.command' < /dev/stdin)

if [[ "$command" == rm* ]]; then
  echo "Blocked: rm commands are not allowed" >&2
  exit 2  # Errore bloccante: la chiamata dello strumento viene impedita
fi

exit 0  # Nessuna decisione: il flusso di autorizzazione normale si applica
```

<Warning>
  Per la maggior parte degli eventi hook, solo il codice di uscita 2 blocca l'azione. Claude Code tratta il codice di uscita 1 come un errore non bloccante e procede con l'azione, anche se 1 è il codice di errore Unix convenzionale. Se l'hook è destinato a applicare una politica, utilizzare `exit 2`. L'eccezione è `WorktreeCreate`, dove qualsiasi codice di uscita diverso da zero interrompe la creazione del worktree.
</Warning>

<h4 id="exit-code-2-behavior-per-event">
  Comportamento del codice di uscita 2 per evento
</h4>

Il codice di uscita 2 è il modo in cui un hook segnala "fermarsi, non farlo". L'effetto dipende dall'evento, perché alcuni eventi rappresentano azioni che possono essere bloccate (come una chiamata dello strumento che non è ancora accaduta) e altri rappresentano cose che sono già accadute o non possono essere prevenute.

| Hook event            | Può bloccare? | Cosa accade su exit 2                                                                                                                                                  |
| :-------------------- | :------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`          | Sì            | Blocca la chiamata dello strumento                                                                                                                                     |
| `PermissionRequest`   | Sì            | Nega l'autorizzazione                                                                                                                                                  |
| `UserPromptSubmit`    | Sì            | Blocca l'elaborazione del prompt e cancella il prompt                                                                                                                  |
| `UserPromptExpansion` | Sì            | Blocca l'espansione                                                                                                                                                    |
| `Stop`                | Sì            | Impedisce a Claude di fermarsi, continua la conversazione                                                                                                              |
| `SubagentStop`        | Sì            | Impedisce al subagent di fermarsi                                                                                                                                      |
| `TeammateIdle`        | Sì            | Impedisce al compagno di squadra di andare inattivo, quindi continua a lavorare                                                                                        |
| `TaskCreated`         | Sì            | Annulla la creazione dell'attività                                                                                                                                     |
| `TaskCompleted`       | Sì            | Impedisce che l'attività sia contrassegnata come completata                                                                                                            |
| `ConfigChange`        | Sì            | Blocca la modifica della configurazione dall'avere effetto (tranne `policy_settings`)                                                                                  |
| `StopFailure`         | No            | L'output e il codice di uscita vengono ignorati                                                                                                                        |
| `PostToolUse`         | No            | Mostra stderr a Claude; lo strumento è già stato eseguito                                                                                                              |
| `PostToolUseFailure`  | No            | Mostra stderr a Claude; lo strumento è già fallito                                                                                                                     |
| `PostToolBatch`       | Sì            | Interrompe il loop agentico prima della prossima chiamata del modello                                                                                                  |
| `PermissionDenied`    | No            | Il codice di uscita e stderr vengono ignorati perché il rifiuto è già avvenuto. Utilizzare JSON `hookSpecificOutput.retry: true` per dire al modello che può riprovare |
| `Notification`        | No            | Mostra stderr solo all'utente                                                                                                                                          |
| `SubagentStart`       | No            | Mostra stderr solo all'utente                                                                                                                                          |
| `SessionStart`        | No            | Mostra stderr solo all'utente                                                                                                                                          |
| `Setup`               | No            | Mostra stderr solo all'utente                                                                                                                                          |
| `SessionEnd`          | No            | Mostra stderr solo all'utente                                                                                                                                          |
| `CwdChanged`          | No            | Mostra stderr solo all'utente                                                                                                                                          |
| `FileChanged`         | No            | Mostra stderr solo all'utente                                                                                                                                          |
| `PreCompact`          | Sì            | Blocca la compattazione                                                                                                                                                |
| `PostCompact`         | No            | Mostra stderr solo all'utente                                                                                                                                          |
| `Elicitation`         | Sì            | Nega l'elicitazione                                                                                                                                                    |
| `ElicitationResult`   | Sì            | Blocca la risposta (l'azione diventa decline)                                                                                                                          |
| `WorktreeCreate`      | Sì            | Qualsiasi codice di uscita diverso da zero causa il fallimento della creazione del worktree                                                                            |
| `WorktreeRemove`      | No            | I guasti vengono registrati solo in modalità debug                                                                                                                     |
| `InstructionsLoaded`  | No            | Il codice di uscita viene ignorato                                                                                                                                     |
| `MessageDisplay`      | No            | Il testo originale viene visualizzato                                                                                                                                  |

Per `SessionStart`, `Setup` e `SubagentStart`, lo stderr del codice di uscita 2 viene visualizzato nella trascrizione come un avviso `<hook name> hook error`, nello stesso modo di un [errore non bloccante](#exit-code-output). Claude non lo vede e la sessione o il subagent procede. Per `SubagentStart`, l'avviso appare nella trascrizione del subagent stesso, non nella conversazione padre.

A partire da Claude Code v2.1.199, `SessionStart`, `Setup` e `SubagentStart` mostrano lo stderr del codice di uscita 2 nella trascrizione. Le versioni precedenti lo scrivevano solo nel log di debug.

<h3 id="http-response-handling">
  Gestione della risposta HTTP
</h3>

Gli HTTP hook utilizzano i codici di stato HTTP e i corpi della risposta invece dei codici di uscita e stdout:

* **2xx con corpo vuoto**: successo, equivalente al codice di uscita 0 senza output
* **2xx con corpo di testo semplice**: successo, il testo viene aggiunto come contesto
* **2xx con corpo JSON**: successo, analizzato utilizzando lo stesso schema [JSON output](#json-output) dei command hook
* **Stato non-2xx**: errore non bloccante, l'esecuzione continua
* **Guasto di connessione o timeout**: errore non bloccante, l'esecuzione continua

A differenza dei command hook, gli HTTP hook non possono segnalare un errore bloccante solo attraverso i codici di stato. Per bloccare una chiamata dello strumento o negare un'autorizzazione, restituire una risposta 2xx con un corpo JSON contenente i campi di decisione appropriati.

<h3 id="json-output">
  Output JSON
</h3>

I codici di uscita consentono di bloccare o stare in silenzio, ma l'output JSON offre un controllo più granulare. Invece di uscire con il codice 2 per bloccare, uscire 0 e stampare un oggetto JSON su stdout. Claude Code legge campi specifici da quel JSON per controllare il comportamento, incluso il [decision control](#decision-control) per bloccare, consentire o escalare all'utente.

<Note>
  È necessario scegliere un approccio per hook, non entrambi: utilizzare i codici di uscita da soli per la segnalazione oppure uscire 0 e stampare JSON per il controllo strutturato. Claude Code elabora JSON solo su exit 0. Se si esce con 2, qualsiasi JSON viene ignorato.
</Note>

Lo stdout del hook deve contenere solo l'oggetto JSON. Se il profilo shell stampa testo all'avvio, può interferire con l'analisi JSON. Consultare [JSON validation failed](/docs/it/hooks-guide#json-validation-failed) nella guida alla risoluzione dei problemi.

Le stringhe di output del hook, incluse `additionalContext`, `systemMessage` e stdout semplice, sono limitate a 10.000 caratteri. L'output che supera questo limite viene salvato in un file e sostituito con un'anteprima e un percorso di file, nello stesso modo in cui vengono gestiti i risultati degli strumenti di grandi dimensioni.

L'oggetto JSON supporta tre tipi di campi:

* **Campi universali** come `continue` funzionano su tutti gli eventi. Questi sono elencati nella tabella seguente.
* **`decision` e `reason` di livello superiore** vengono utilizzati da alcuni eventi per bloccare o fornire feedback.
* **`hookSpecificOutput`** è un oggetto annidato per gli eventi che necessitano di un controllo più ricco. Richiede un campo `hookEventName` impostato sul nome dell'evento.

| Campo              | Impostazione predefinita | Descrizione                                                                                                                                                                                                                                                                                                                                                                                        |
| :----------------- | :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `continue`         | `true`                   | Se `false`, Claude interrompe completamente l'elaborazione dopo l'esecuzione del hook. Ha la precedenza su qualsiasi campo di decisione specifico dell'evento                                                                                                                                                                                                                                      |
| `stopReason`       | nessuno                  | Messaggio mostrato all'utente quando `continue` è `false`. Non mostrato a Claude                                                                                                                                                                                                                                                                                                                   |
| `suppressOutput`   | `false`                  | Se `true`, omette stdout dalla trascrizione. Stdout appare ancora nel log di debug                                                                                                                                                                                                                                                                                                                 |
| `systemMessage`    | nessuno                  | Messaggio di avviso mostrato all'utente                                                                                                                                                                                                                                                                                                                                                            |
| `terminalSequence` | nessuno                  | Una sequenza di escape del terminale per Claude Code da emettere per conto vostro, come una notifica desktop, un titolo della finestra o un campanello. Limitato a OSC `0`/`1`/`2`/`9`/`99`/`777` e BEL. Se il valore contiene qualcosa al di fuori della lista di autorizzazione, il campo viene ignorato. Utilizzare questo invece di scrivere su `/dev/tty`, che non è disponibile per gli hook |

Per fermare Claude completamente indipendentemente dal tipo di evento:

```json theme={null}
{ "continue": false, "stopReason": "Build failed, fix errors before continuing" }
```

<h4 id="emit-terminal-notifications">
  Emettere notifiche del terminale
</h4>

Il campo `terminalSequence` richiede Claude Code v2.1.141 o successivo.

Gli hook vengono eseguiti senza un terminale di controllo, quindi la scrittura di sequenze di escape direttamente su `/dev/tty` non riesce. Invece, restituire la sequenza di escape nel campo `terminalSequence` e Claude Code la emetterà per voi attraverso il suo percorso di scrittura del terminale. Questo è privo di race condition, funziona all'interno di tmux e GNU screen, e funziona su Windows dove non esiste `/dev/tty`.

Il campo accetta una stringa di una o più sequenze di escape nella lista di autorizzazione:

* OSC `0`, `1`, `2`: titoli della finestra e dell'icona
* OSC `9`: notifiche iTerm2, ConEmu, Windows Terminal e WezTerm, incluso `9;4` progresso della barra delle applicazioni
* OSC `99`: notifiche Kitty
* OSC `777`: notifiche urxvt, Ghostty e Warp
* BEL nudo

Le sequenze possono essere terminate con BEL o con ST. Qualsiasi cosa al di fuori della lista di autorizzazione, incluse le sequenze CSI del cursore e del colore, le sequenze della tavolozza OSC, i collegamenti ipertestuali OSC 8, le scritture degli appunti OSC 52 e OSC 1337, viene rifiutata e il campo viene ignorato.

L'esempio seguente attiva una notifica desktop da un hook `Notification`. La sequenza di escape viene costruita con `printf` escape ottali in modo che i byte di controllo non compaiano mai sulla riga di comando della shell, e `jq -n --arg` costruisce l'output JSON in modo che le virgolette, le barre rovesciate e le nuove righe nel messaggio di notifica siano correttamente sfuggite:

```bash theme={null}
#!/bin/bash
# Hook di notifica: avvisa il desktop quando Claude Code ha bisogno di attenzione.
input=$(cat)
title="Claude Code'
body=$(jq -r '.message // 'Needs your attention"' <<<"$input")
seq=$(printf '\033]777;notify;%s;%s\007' "$title" "$body")
jq -nc --arg seq "$seq" '{terminalSequence: $seq}'
```

La forma `{ "terminalSequence": "..." }` è la stessa da qualsiasi shell o linguaggio. Su Windows, costruire la stringa di escape in PowerShell o uno script e emettere lo stesso oggetto JSON.

<Note>
  `terminalSequence` è la sostituzione supportata per gli hook che in precedenza scrivevano sequenze di escape direttamente su `/dev/tty`. La lista di autorizzazione è limitata alle sequenze che non possono spostare il cursore o alterare i colori, quindi un hook non può mai corrompere un prompt sullo schermo.
</Note>

<h4 id="add-context-for-claude">
  Aggiungere contesto per Claude
</h4>

Il campo `additionalContext` passa una stringa dal hook nel contesto della finestra di Claude. Claude Code avvolge la stringa in un promemoria di sistema e la inserisce nella conversazione nel punto in cui l'hook si è attivato. Claude legge il promemoria nella prossima richiesta del modello, ma non appare come messaggio di chat nell'interfaccia.

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

* [SessionStart](#sessionstart), [Setup](#setup) e [SubagentStart](#subagentstart): all'inizio della conversazione, prima del primo prompt
* [UserPromptSubmit](#userpromptsubmit) e [UserPromptExpansion](#userpromptexpansion): insieme al prompt inviato
* [PreToolUse](#pretooluse), [PostToolUse](#posttooluse), [PostToolUseFailure](#posttoolusefailure) e [PostToolBatch](#posttoolbatch): accanto al risultato dello strumento
* [Stop](#stop) e [SubagentStop](#subagentstop): alla fine del turno. La conversazione continua in modo che Claude possa agire sul feedback. Consultare [Stop decision control](#stop-decision-control)

Quando più hook restituiscono `additionalContext` per lo stesso evento, Claude riceve tutti i valori. Se un valore supera 10.000 caratteri, Claude Code scrive il testo completo in un file nella directory della sessione e passa a Claude il percorso del file con un'anteprima breve.

Utilizzare `additionalContext` per informazioni che Claude dovrebbe conoscere sullo stato corrente dell'ambiente o sull'operazione appena eseguita:

* **Stato dell'ambiente**: il ramo corrente, la destinazione di distribuzione o i flag di funzionalità attivi
* **Regole di progetto condizionali**: quale comando di test si applica al file appena modificato, quali directory sono di sola lettura in questo worktree
* **Dati esterni**: problemi aperti assegnati a voi, risultati CI recenti, contenuto recuperato da un servizio interno

Per le istruzioni che non cambiano mai, preferire [CLAUDE.md](/docs/it/memory). Si carica senza eseguire uno script ed è il luogo standard per le convenzioni di progetto statiche.

Scrivere il testo come affermazioni fattuali piuttosto che istruzioni di sistema imperative. Frasi come "La destinazione di distribuzione è produzione" o "Questo repository utilizza `bun test`" si leggono come informazioni di progetto. Il testo inquadrato come comandi di sistema fuori banda può attivare le difese di iniezione di prompt di Claude, il che causa a Claude di far emergere il testo a voi invece di trattarlo come contesto.

Una volta iniettato, il testo viene salvato nella trascrizione della sessione. Per gli eventi a metà sessione come `PostToolUse` o `UserPromptSubmit`, la ripresa con `--continue` o `--resume` riproduce il testo salvato piuttosto che rieseguire l'hook per i turni passati, quindi i valori come timestamp o SHA di commit diventano obsoleti al ripristino. Gli hook `SessionStart` vengono eseguiti di nuovo al ripristino con `source` impostato su `"resume"`, quindi possono aggiornare il loro contesto.

<h4 id="decision-control">
  Controllo della decisione
</h4>

Non ogni evento supporta il blocco o il controllo del comportamento attraverso JSON. Gli eventi che lo fanno utilizzano ciascuno un insieme diverso di campi per esprimere quella decisione. Utilizzare questa tabella come riferimento rapido prima di scrivere un hook:

| Eventi                                                                                                                              | Modello di decisione                 | Campi chiave                                                                                                                                                                                                                                     |
| :---------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| UserPromptSubmit, UserPromptExpansion, PostToolUse, PostToolUseFailure, PostToolBatch, Stop, SubagentStop, ConfigChange, PreCompact | `decision` di livello superiore      | `decision: "block"`, `reason`. Stop e SubagentStop accettano anche `hookSpecificOutput.additionalContext` per [feedback non-errore che continua la conversazione](#stop-decision-control)                                                        |
| TeammateIdle, TaskCreated, TaskCompleted                                                                                            | Codice di uscita o `continue: false` | Il codice di uscita 2 blocca l'azione con feedback stderr. JSON `{"continue": false, "stopReason": "..."}` interrompe anche completamente il compagno di squadra, corrispondendo al comportamento dell'hook `Stop`                               |
| PreToolUse                                                                                                                          | `hookSpecificOutput`                 | `permissionDecision` (allow/deny/ask/defer), `permissionDecisionReason`                                                                                                                                                                          |
| PermissionRequest                                                                                                                   | `hookSpecificOutput`                 | `decision.behavior` (allow/deny)                                                                                                                                                                                                                 |
| PermissionDenied                                                                                                                    | `hookSpecificOutput`                 | `retry: true` dice al modello che può riprovare la chiamata dello strumento negata                                                                                                                                                               |
| WorktreeCreate                                                                                                                      | percorso return                      | Il command hook stampa il percorso su stdout; l'HTTP hook restituisce `hookSpecificOutput.worktreePath`. Il fallimento del hook o il percorso mancante non riesce nella creazione                                                                |
| Elicitation                                                                                                                         | `hookSpecificOutput`                 | `action` (accept/decline/cancel), `content` (valori dei campi del modulo per accept)                                                                                                                                                             |
| ElicitationResult                                                                                                                   | `hookSpecificOutput`                 | `action` (accept/decline/cancel), `content` (valori dei campi del modulo override)                                                                                                                                                               |
| MessageDisplay                                                                                                                      | `hookSpecificOutput`                 | `displayContent` sostituisce il testo visualizzato sullo schermo. Solo visualizzazione: la trascrizione e ciò che Claude vede mantengono l'originale                                                                                             |
| SessionStart, Setup, SubagentStart                                                                                                  | Solo contesto                        | `hookSpecificOutput.additionalContext` aggiunge contesto per Claude. SessionStart accetta anche [`initialUserMessage`, `watchPaths`, `sessionTitle` e `reloadSkills`](#sessionstart-decision-control). Nessun blocco o controllo della decisione |
| WorktreeRemove, Notification, SessionEnd, PostCompact, InstructionsLoaded, StopFailure, CwdChanged, FileChanged                     | Nessuno                              | Nessun controllo della decisione. Utilizzato per effetti collaterali come la registrazione o la pulizia                                                                                                                                          |

Alcuni eventi possono anche riscrivere il contenuto piuttosto che solo consentire o bloccare:

* `PreToolUse`: `updatedInput` direttamente sotto `hookSpecificOutput` sostituisce gli argomenti di uno strumento prima che venga eseguito. Consultare [PreToolUse decision control](#pretooluse-decision-control) per l'insieme completo di opzioni.
* `PermissionRequest`: `updatedInput` all'interno dell'oggetto `decision`. Consultare [PermissionRequest decision control](#permissionrequest-decision-control) per l'insieme completo di opzioni.
* `PostToolUse`: `updatedToolOutput` sostituisce il risultato dello strumento. Consultare [PostToolUse decision control](#posttooluse-decision-control) per l'insieme completo di opzioni.
* `UserPromptSubmit`: non può sostituire il prompt; solo inietta `additionalContext` insieme ad esso

Per i casi di uso di redazione o trasformazione, intercettare a `PreToolUse` per gli input dello strumento in uscita e `PostToolUse` per i risultati dello strumento in entrata.

Ecco esempi di ogni modello in azione:

<Tabs>
  <Tab title="Decisione di livello superiore">
    Utilizzato da `UserPromptSubmit`, `UserPromptExpansion`, `PostToolUse`, `PostToolUseFailure`, `PostToolBatch`, `Stop`, `SubagentStop`, `ConfigChange` e `PreCompact`. L'unico valore è `"block"`. Per consentire all'azione di procedere, omettere `decision` dal JSON o uscire 0 senza alcun JSON:

    ```json theme={null}
    {
      "decision": "block",
      "reason": "Test suite must pass before proceeding"
    }
    ```
  </Tab>

  <Tab title="PreToolUse">
    Utilizza `hookSpecificOutput` per un controllo più ricco: consentire, negare, chiedere o rinviare. È anche possibile modificare l'input dello strumento prima che venga eseguito o iniettare contesto aggiuntivo per Claude. Consultare [PreToolUse decision control](#pretooluse-decision-control) per l'insieme completo di opzioni.

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
    Utilizza `hookSpecificOutput` per consentire o negare una richiesta di autorizzazione per conto dell'utente. Quando si consente, è anche possibile modificare l'input dello strumento o applicare regole di autorizzazione in modo che l'utente non venga richiesto di nuovo. Consultare [PermissionRequest decision control](#permissionrequest-decision-control) per l'insieme completo di opzioni.

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

Ogni evento corrisponde a un punto nel ciclo di vita di Claude Code in cui gli hook possono essere eseguiti. Le sezioni seguenti sono ordinate per corrispondere al ciclo di vita: dalla configurazione della sessione attraverso il ciclo agentico alla fine della sessione. Ogni sezione descrive quando l'evento si attiva, quali matcher supporta, l'input JSON che riceve e come controllare il comportamento attraverso l'output.

<h3 id="sessionstart">
  SessionStart
</h3>

Viene eseguito quando Claude Code avvia una nuova sessione o riprende una sessione esistente. Utile per caricare il contesto di sviluppo come problemi esistenti o modifiche recenti al codebase, o per configurare le variabili di ambiente. Per il contesto statico che non richiede uno script, utilizzare [CLAUDE.md](/docs/it/memory) invece.

SessionStart viene eseguito ad ogni sessione, quindi mantenere questi hook veloci. Solo gli hook `type: "command"` e `type: "mcp_tool"` sono supportati.

Il valore del matcher corrisponde a come è stata avviata la sessione:

| Matcher   | Quando si attiva                     |
| :-------- | :----------------------------------- |
| `startup` | Nuova sessione                       |
| `resume`  | `--resume`, `--continue` o `/resume` |
| `clear`   | `/clear`                             |
| `compact` | Compattazione automatica o manuale   |

<h4 id="sessionstart-input">
  Input di SessionStart
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook SessionStart ricevono `source` e facoltativamente `model`, `agent_type` e `session_title`:

| Campo           | Descrizione                                                                                                                                                                                                                                            |
| :-------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `source`        | Come è iniziata la sessione: `"startup"` per le nuove sessioni, `"resume"` per le sessioni riprese, `"clear"` dopo `/clear` o `"compact"` dopo la compattazione                                                                                        |
| `model`         | L'identificatore del modello attivo. Può essere omesso, ad esempio dopo `/clear` o quando una sessione viene ripristinata attraverso il recupero della conversazione, quindi controllare il campo prima di leggerlo                                    |
| `agent_type`    | Il nome dell'agente, presente quando si avvia Claude Code con `claude --agent <name>`                                                                                                                                                                  |
| `session_title` | Il titolo della sessione corrente se uno è già impostato, ad esempio tramite `--name` o `/rename`. Un hook che emette `sessionTitle` può controllare prima `session_title` per evitare di sovrascrivere un titolo impostato esplicitamente dall'utente |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionStart",
  "source": "startup",
  "model": "claude-sonnet-5"
}
```

<h4 id="sessionstart-decision-control">
  Controllo della decisione di SessionStart
</h4>

Qualsiasi testo che lo script del hook stampa su stdout viene aggiunto come contesto per Claude. Oltre ai [campi di output JSON](#json-output) disponibili per tutti gli hook, è possibile restituire questi campi specifici dell'evento:

| Campo                | Descrizione                                                                                                                                                                                                                                                                                                                                                              |
| :------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext`  | Stringa aggiunta al contesto di Claude all'inizio della conversazione, prima del primo prompt. Consultare [Aggiungere contesto per Claude](#add-context-for-claude) per come il testo viene consegnato e cosa inserirvi                                                                                                                                                  |
| `initialUserMessage` | Stringa utilizzata come primo messaggio dell'utente della sessione. Si applica in [modalità non interattiva](/docs/it/headless) con il flag `-p`, dove diventa il primo turno anche se non viene fornito alcun prompt. Se viene fornito un prompt, segue come turno successivo. A differenza di `additionalContext`, che si allega a un turno esistente, questo crea il turno |
| `sessionTitle`       | Imposta il titolo della sessione, con lo stesso effetto di `/rename`. Utilizzare per denominare automaticamente le sessioni dalla cartella di avvio, dal ramo git o dal nome del worktree. Si applica solo quando `source` è `"startup"` o `"resume"`; ignorato su `"clear"` e `"compact"`                                                                               |
| `watchPaths`         | Array di percorsi assoluti da monitorare per gli eventi [FileChanged](#filechanged) durante questa sessione                                                                                                                                                                                                                                                              |
| `reloadSkills`       | Booleano. Quando `true`, Claude Code esegue nuovamente la scansione delle directory [skill](/docs/it/skills) e dei comandi dopo il completamento degli hook SessionStart, in modo che le skill installate dall'hook siano disponibili nella stessa sessione, a partire dal primo prompt                                                                                       |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "SessionStart",
    "additionalContext": "Current branch: feat/auth-refactor\nUncommitted changes: src/auth.ts, src/login.tsx\nActive issue: #4211 Migrate to OAuth2",
    "sessionTitle": "auth-refactor"
  }
}
```

Poiché lo stdout semplice raggiunge già Claude per questo evento, un hook che carica solo contesto può stampare su stdout direttamente senza costruire JSON. Utilizzare il formato JSON quando è necessario combinare il contesto con altri campi come `suppressOutput` o `sessionTitle`.

Utilizzare `reloadSkills` quando un hook SessionStart installa o aggiorna skill. La scoperta delle skill normalmente viene eseguita prima del completamento degli hook SessionStart, quindi i file che l'hook scrive in `~/.claude/skills/` o `.claude/skills/` altrimenti apparirebbero solo nella sessione successiva. Questo esempio sincronizza un repository di skill condiviso e richiede la nuova scansione:

```bash theme={null}
#!/bin/bash

git -C ~/.claude/skills/team-skills pull --quiet 2>/dev/null || \
  git clone --quiet https://git.example.com/your-org/team-skills.git ~/.claude/skills/team-skills

echo '{"hookSpecificOutput": {"hookEventName": "SessionStart", "reloadSkills": true}}'
```

<h4 id="persist-environment-variables">
  Persistere le variabili di ambiente
</h4>

Gli hook SessionStart hanno accesso alla variabile di ambiente `CLAUDE_ENV_FILE`, che fornisce un percorso di file in cui è possibile persistere le variabili di ambiente per i comandi Bash successivi.

Per impostare le singole variabili di ambiente, scrivere le istruzioni `export` in `CLAUDE_ENV_FILE`. Utilizzare l'aggiunta (`>>`) per preservare le variabili impostate da altri hook:

```bash theme={null}
#!/bin/bash

if [ -n "$CLAUDE_ENV_FILE" ]; then
  echo 'export NODE_ENV=production' >> "$CLAUDE_ENV_FILE"
  echo 'export DEBUG_LOG=true' >> "$CLAUDE_ENV_FILE"
  echo 'export PATH="$PATH:./node_modules/.bin"' >> "$CLAUDE_ENV_FILE"
fi

exit 0
```

Per acquisire tutte le modifiche dell'ambiente dai comandi di configurazione, confrontare le variabili esportate prima e dopo:

```bash theme={null}
#!/bin/bash

ENV_BEFORE=$(export -p | sort)

# Eseguire i comandi di configurazione che modificano l'ambiente
source ~/.nvm/nvm.sh
nvm use 20

if [ -n "$CLAUDE_ENV_FILE" ]; then
  ENV_AFTER=$(export -p | sort)
  comm -13 <(echo "$ENV_BEFORE") <(echo "$ENV_AFTER") >> "$CLAUDE_ENV_FILE"
fi

exit 0
```

Qualsiasi variabile scritta in questo file sarà disponibile in tutti i comandi Bash successivi che Claude Code esegue durante la sessione.

<Note>
  `CLAUDE_ENV_FILE` è disponibile per gli hook SessionStart, [Setup](#setup), [CwdChanged](#cwdchanged) e [FileChanged](#filechanged). Gli altri tipi di hook non hanno accesso a questa variabile.
</Note>

<h3 id="setup">
  Setup
</h3>

Si attiva solo quando si avvia Claude Code con `--init-only`, o con `--init` o `--maintenance` in [modalità non interattiva](/docs/it/headless) con il flag `-p`. Non si attiva all'avvio normale. Utilizzarlo per l'installazione di dipendenze una tantum o la pulizia pianificata che si attiva esplicitamente da CI o script, separato dall'avvio della sessione normale. Per l'inizializzazione per sessione, utilizzare [SessionStart](#sessionstart) invece.

Il valore del matcher corrisponde al flag CLI che ha attivato l'hook:

| Matcher       | Quando si attiva                          |
| :------------ | :---------------------------------------- |
| `init`        | `claude --init-only` o `claude -p --init` |
| `maintenance` | `claude -p --maintenance`                 |

`--init-only` esegue gli hook Setup e gli hook SessionStart con il matcher `startup`, quindi esce senza avviare una conversazione. `--init` e `--maintenance` attivano gli hook Setup solo quando combinati con `-p`; in una sessione interattiva questi due flag attualmente non attivano gli hook Setup.

Poiché Setup non si attiva ad ogni avvio, un plugin che ha bisogno di una dipendenza installata non può fare affidamento solo su Setup. Il modello pratico è controllare la dipendenza al primo utilizzo e installare se assente, ad esempio un hook o una skill che testa per `${CLAUDE_PLUGIN_DATA}/node_modules` ed esegue `npm install` se assente. Consultare la [directory dei dati persistenti](/docs/it/plugins-reference#persistent-data-directory) per dove archiviare le dipendenze installate.

<h4 id="setup-input">
  Input di Setup
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook Setup ricevono un campo `trigger` impostato su `"init"` o `"maintenance"`:

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
  Controllo della decisione di Setup
</h4>

Gli hook Setup non possono bloccare. Qualsiasi exit code non zero, incluso 2, fa emergere stderr all'utente come avviso `<hook name> hook error`, e l'esecuzione continua. In [modalità non interattiva](/docs/it/headless), l'output dell'hook appare solo quando si avvia con `--verbose`.

Per passare informazioni nel contesto di Claude, restituire `additionalContext` nell'output JSON; lo stdout semplice viene scritto nel log di debug solo. Oltre ai [campi di output JSON](#json-output) disponibili per tutti gli hook, è possibile restituire questi campi specifici dell'evento:

| Campo               | Descrizione                                                                      |
| :------------------ | :------------------------------------------------------------------------------- |
| `additionalContext` | Stringa aggiunta al contesto di Claude. I valori di più hook vengono concatenati |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "Setup",
    "additionalContext": "Dependencies installed: node_modules, .venv"
  }
}
```

Gli hook Setup hanno accesso a `CLAUDE_ENV_FILE`. Le variabili scritte in quel file persistono nei comandi Bash successivi per la sessione, proprio come negli [hook SessionStart](#persist-environment-variables). Solo gli hook `type: "command"` e `type: "mcp_tool"` sono supportati.

<h3 id="instructionsloaded">
  InstructionsLoaded
</h3>

Si attiva quando un file `CLAUDE.md` o `.claude/rules/*.md` viene caricato nel contesto. Questo evento si attiva all'avvio della sessione per i file caricati con entusiasmo e di nuovo in seguito quando i file vengono caricati in modo pigro, ad esempio quando Claude accede a una sottodirectory che contiene un `CLAUDE.md` annidato o quando le regole condizionali con frontmatter `paths:` corrispondono. L'hook non supporta il blocco o il controllo della decisione. Viene eseguito in modo asincrono per scopi di osservabilità.

Il matcher viene eseguito su `load_reason`. Ad esempio, utilizzare `"matcher": "session_start"` per attivarsi solo per i file caricati all'avvio della sessione, o `"matcher": "path_glob_match|nested_traversal"` per attivarsi solo per i caricamenti pigri.

<h4 id="instructionsloaded-input">
  Input di InstructionsLoaded
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook InstructionsLoaded ricevono questi campi:

| Campo               | Descrizione                                                                                                                                                                                                                              |
| :------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `file_path`         | Percorso assoluto al file di istruzioni che è stato caricato                                                                                                                                                                             |
| `memory_type`       | Ambito del file: `"User"`, `"Project"`, `"Local"` o `"Managed"`                                                                                                                                                                          |
| `load_reason`       | Perché il file è stato caricato: `"session_start"`, `"nested_traversal"`, `"path_glob_match"`, `"include"` o `"compact"`. Il valore `"compact"` si attiva quando i file di istruzioni vengono ricaricati dopo un evento di compattazione |
| `globs`             | Modelli glob del percorso dal frontmatter `paths:` del file, se presenti. Presente solo per i caricamenti `path_glob_match`                                                                                                              |
| `trigger_file_path` | Percorso al file il cui accesso ha attivato questo caricamento, per i caricamenti pigri                                                                                                                                                  |
| `parent_file_path`  | Percorso al file di istruzioni padre che ha incluso questo, per i caricamenti `include`                                                                                                                                                  |

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
  Controllo della decisione di InstructionsLoaded
</h4>

Gli hook InstructionsLoaded non hanno controllo della decisione. Non possono bloccare o modificare il caricamento delle istruzioni. Utilizzare questo evento per la registrazione di audit, il tracciamento della conformità o l'osservabilità.

<h3 id="userpromptsubmit">
  UserPromptSubmit
</h3>

Viene eseguito quando l'utente invia un prompt, prima che Claude lo elabori. Ciò consente di aggiungere contesto aggiuntivo in base al prompt/conversazione, convalidare i prompt o bloccare determinati tipi di prompt.

Gli hook `UserPromptSubmit` hanno un timeout predefinito di 30 secondi per i tipi `command`, `http` e `mcp_tool`, più breve del default di 600 secondi per questi tipi su altri eventi. Poiché questo hook viene eseguito prima di ogni prompt e blocca l'elaborazione del modello fino al completamento, un hook bloccato blocca la sessione. Se l'hook ha bisogno di più tempo, impostare il campo `timeout` nella voce dell'hook.

Un hook `UserPromptSubmit` che raggiunge il suo timeout viene annullato e il suo output, incluso qualsiasi `additionalContext`, viene scartato. Il prompt raggiunge comunque Claude senza quel contesto. A partire da v2.1.196, la trascrizione mostra un avviso che nomina l'hook, il timeout che si è attivato e che l'output è stato scartato. Le versioni precedenti annullano l'hook senza avviso.

Un hook di callback [Agent SDK](/docs/it/agent-sdk/hooks) su `UserPromptSubmit` che raggiunge il suo timeout blocca il prompt con un messaggio che nomina l'hook e il timeout, perché un callback lì può agire come un gate di politica che non deve fallire in modo aperto. La sessione continua. Prima di v2.1.208, un timeout di callback su quell'evento terminava il turno con un errore di esecuzione.

<h4 id="userpromptsubmit-input">
  Input di UserPromptSubmit
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook UserPromptSubmit ricevono il campo `prompt` contenente il testo che l'utente ha inviato.

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
  Controllo della decisione di UserPromptSubmit
</h4>

Gli hook `UserPromptSubmit` possono controllare se un prompt dell'utente viene elaborato e aggiungere contesto. Tutti i [campi di output JSON](#json-output) sono disponibili.

Ci sono due modi per aggiungere contesto alla conversazione su exit code 0:

* **Stdout di testo semplice**: qualsiasi testo non-JSON scritto su stdout viene aggiunto come contesto
* **JSON con `additionalContext`**: utilizzare il formato JSON seguente per un controllo maggiore. Il campo `additionalContext` viene aggiunto come contesto

Lo stdout semplice viene mostrato come output del hook nella trascrizione. Il valore `additionalContext` viene iniettato come un promemoria di sistema che Claude legge senza una voce di trascrizione visibile.

Per bloccare un prompt, restituire un oggetto JSON con `decision` impostato su `"block"`:

| Campo                    | Descrizione                                                                                                                            |
| :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------- |
| `decision`               | `"block"` impedisce l'elaborazione del prompt e lo cancella dal contesto. Omettere per consentire al prompt di procedere               |
| `reason`                 | Mostrato all'utente quando `decision` è `"block"`. Non aggiunto al contesto                                                            |
| `additionalContext`      | Stringa aggiunta al contesto di Claude insieme al prompt inviato. Consultare [Aggiungere contesto per Claude](#add-context-for-claude) |
| `sessionTitle`           | Imposta il titolo della sessione. Utilizzare per denominare automaticamente le sessioni in base al contenuto del prompt                |
| `suppressOriginalPrompt` | Se `true` quando `decision` è `"block"`, omette il testo del prompt originale dal messaggio di blocco mostrato all'utente              |

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

Viene eseguito quando un comando slash digitato dall'utente si espande in un prompt prima di raggiungere Claude. Utilizzare questo per bloccare comandi specifici dall'invocazione diretta, iniettare contesto per una skill particolare o registrare quali comandi gli utenti invocano. Ad esempio, un hook che corrisponde a `deploy` può bloccare `/deploy` a meno che non sia presente un file di approvazione, oppure un hook che corrisponde a una skill di revisione può aggiungere la checklist di revisione del team come `additionalContext`.

Questo evento copre il percorso che `PreToolUse` non copre: un hook `PreToolUse` che corrisponde allo strumento `Skill` si attiva solo quando Claude chiama lo strumento, ma digitare `/skillname` direttamente bypassa `PreToolUse`. `UserPromptExpansion` si attiva su quel percorso diretto.

Corrisponde a `command_name`. Lasciare il matcher vuoto per attivarsi su ogni slash command di tipo prompt.

<h4 id="userpromptexpansion-input">
  Input di UserPromptExpansion
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook UserPromptExpansion ricevono `expansion_type`, `command_name`, `command_args`, `command_source` e la stringa `prompt` originale. Il campo `expansion_type` è `slash_command` per skill e comandi personalizzati, o `mcp_prompt` per i prompt del server MCP.

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
  Controllo della decisione di UserPromptExpansion
</h4>

Gli hook `UserPromptExpansion` possono bloccare l'espansione o aggiungere contesto. Tutti i [campi di output JSON](#json-output) sono disponibili.

| Campo               | Descrizione                                                                                                                            |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------- |
| `decision`          | `"block"` impedisce l'espansione dello slash command. Omettere per consentirgli di procedere                                           |
| `reason`            | Mostrato all'utente quando `decision` è `"block"`                                                                                      |
| `additionalContext` | Stringa aggiunta al contesto di Claude insieme al prompt espanso. Consultare [Aggiungere contesto per Claude](#add-context-for-claude) |

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

Viene eseguito mentre un messaggio dell'assistente viene trasmesso sullo schermo. Claude Code visualizza il messaggio in incrementi: ogni volta che un batch di righe appena completate è pronto per il rendering, l'hook viene eseguito una volta con quelle righe e Claude Code esegue il rendering del testo di sostituzione dell'hook al loro posto. Un messaggio lungo produce più chiamate; un messaggio breve può produrne solo una.

Utilizzare MessageDisplay per:

* rimuovere il markdown per una visualizzazione minima
* trasformare il testo che un'applicazione Agent SDK mostra ai suoi utenti
* oscurare le chiavi API o i nomi host interni dalle risposte di Claude

Claude Code tiene ogni batch in sospeso fino al ritorno dell'hook, quindi mantenere l'hook veloce. Se l'hook non riesce o scade, Claude Code visualizza il testo originale. Il timeout predefinito per questo evento è 10 secondi; se l'hook ha bisogno di più tempo, impostare il campo `timeout` nella voce dell'hook.

MessageDisplay è solo per la visualizzazione: il testo di sostituzione cambia solo ciò che viene renderizzato sullo schermo. La trascrizione e ciò che Claude vede mantengono il testo originale, quindi Claude non vede mai la sostituzione e la modalità verbose mostra l'originale. L'hook riceve solo il testo del messaggio dell'assistente, quindi i risultati degli strumenti e il testo che digitate vengono renderizzati invariati.

MessageDisplay non supporta i matcher e si attiva per ogni messaggio dell'assistente che trasmette testo; i messaggi senza testo, come le risposte solo con chiamate di strumenti, non lo attivano.

Nelle esecuzioni non interattive, incluse le query Agent SDK e `claude -p`, MessageDisplay viene eseguito una volta per messaggio dell'assistente invece che una volta per batch di righe. La singola chiamata arriva dopo il completamento del messaggio e contiene il testo completo del messaggio: `index` è `0`, `final` è `true` e `delta` contiene l'intero messaggio. Un hook che raccoglie il testo `delta` per ogni messaggio riceve lo stesso testo totale in entrambe le modalità.

<h4 id="messagedisplay-input">
  Input di MessageDisplay
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook MessageDisplay ricevono identificatori per il turno e il messaggio, la posizione di questa chiamata all'interno del messaggio e il nuovo testo in `delta`. I confini dei batch dipendono da come il testo viene trasmesso, quindi utilizzare `index` e `final` per tracciare l'avanzamento attraverso un messaggio piuttosto che aspettarsi che le righe siano raggruppate in un modo particolare.

| Campo        | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `turn_id`    | UUID del turno corrente                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `message_id` | UUID del messaggio dell'assistente in corso di visualizzazione. Stabile in ogni batch dello stesso messaggio. Questo non è l'id API `msg_…`, quindi non può essere correlato con gli id dei messaggi della trascrizione                                                                                                                                                                                                                        |
| `index`      | Indice a base zero di questo batch all'interno del messaggio                                                                                                                                                                                                                                                                                                                                                                                   |
| `final`      | `true` nell'ultimo batch del messaggio. Ogni messaggio ha esattamente un batch finale                                                                                                                                                                                                                                                                                                                                                          |
| `delta`      | Le righe appena completate dall'ultimo batch, incluse le newline finali. Sempre righe intere, tranne l'ultimo batch che può terminare a metà riga. Nelle esecuzioni interattive, il delta dell'ultimo batch è vuoto quando il messaggio termina con una newline, quindi trattare `final`, non un delta non vuoto, come il segnale di fine messaggio. Nelle esecuzioni Agent SDK e `claude -p`, la singola chiamata contiene l'intero messaggio |

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
  Output di MessageDisplay
</h4>

Oltre ai [campi di output JSON](#json-output) disponibili per tutti gli hook, gli hook MessageDisplay possono restituire `displayContent` per sostituire il delta sullo schermo:

| Campo            | Descrizione                                                                  |
| :--------------- | :--------------------------------------------------------------------------- |
| `displayContent` | Testo visualizzato al posto del delta. Omettere per visualizzare l'originale |

Gli hook MessageDisplay non hanno controllo della decisione. Non possono bloccare il messaggio o modificare ciò che viene archiviato nella trascrizione o inviato a Claude.

Questo esempio rimuove la formattazione markdown dalle risposte di Claude per una visualizzazione in testo semplice. Lo script legge ogni batch da stdin, rimuove i marcatori di grassetto e i backtick del codice inline da `delta` e restituisce il risultato come `displayContent`.

<Tabs>
  <Tab title="macOS/Linux">
    Registrare un hook di comando per l'evento nel file di impostazioni:

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

    Salvare questo script in `.claude/hooks/plain-display.sh` nel progetto e renderlo eseguibile con `chmod +x`:

    ```bash theme={null}
    #!/bin/bash
    jq '{hookSpecificOutput: {hookEventName: "MessageDisplay", displayContent: (.delta | gsub("\\*\\*"; "") | gsub("`"; ""))}}'
    ```

    Lo script ha bisogno di `jq` nel vostro `PATH`.
  </Tab>

  <Tab title="Windows (PowerShell)">
    Registrare un hook di comando che esegue lo script tramite PowerShell:

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

    Il flag `-NoProfile` salta il caricamento del profilo PowerShell in modo che l'hook si avvii velocemente, e `-ExecutionPolicy Bypass` consente a PowerShell di eseguire il file di script locale.

    Salvare questo script in `.claude/hooks/plain-display.ps1` nel progetto:

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

I batch senza markdown passano invariati. Se lo script non riesce, ad esempio perché `jq` è mancante, Claude Code visualizza il testo originale e nota il guasto solo nell'[output di debug](#debug-hooks), non nella sessione.

<h3 id="pretooluse">
  PreToolUse
</h3>

Viene eseguito dopo che Claude crea i parametri dello strumento e prima dell'elaborazione della chiamata dello strumento. Corrisponde al nome dello strumento: `Bash`, `Edit`, `Write`, `Read`, `Glob`, `Grep`, `Agent`, `WebFetch`, `WebSearch`, `AskUserQuestion`, `ExitPlanMode` e qualsiasi [nome di strumento MCP](#match-mcp-tools).

<Warning>
  PreToolUse viene eseguito solo quando Claude chiama uno strumento. I file che [fate riferimento con `@` nel vostro prompt](/docs/it/common-workflows#reference-files-and-directories) vengono aggiunti senza alcuna chiamata dello strumento: Claude Code inserisce i loro contenuti mentre costruisce il prompt, quindi nessun hook PreToolUse si attiva per loro, inclusi gli hook che corrispondono a `Read`. Per bloccare percorsi specifici dai riferimenti `@`, utilizzare una [regola di negazione `Read`](/docs/it/permissions#read-and-edit) invece.
</Warning>

Utilizzare il [PreToolUse decision control](#pretooluse-decision-control) per consentire, negare, chiedere o rinviare il permesso di utilizzare lo strumento.

<h4 id="pretooluse-input">
  Input di PreToolUse
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook PreToolUse ricevono `tool_name`, `tool_input` e `tool_use_id`. I campi `tool_input` dipendono dallo strumento:

<h5 id="bash">
  Bash
</h5>

Esegue comandi shell.

| Campo               | Tipo    | Esempio            | Descrizione                                                                                                                                                     |
| :------------------ | :------ | :----------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | string  | `"npm test"`       | Il comando shell da eseguire                                                                                                                                    |
| `description`       | string  | `"Run test suite"` | Descrizione facoltativa di cosa fa il comando                                                                                                                   |
| `timeout`           | number  | `120000`           | Timeout facoltativo in millisecondi. I valori superiori al [massimo](/docs/it/tools-reference#bash-tool-behavior) vengono ridotti al massimo piuttosto che rifiutati |
| `run_in_background` | boolean | `false`            | Se eseguire il comando in background                                                                                                                            |

<h5 id="write">
  Write
</h5>

Crea o sovrascrive un file.

| Campo       | Tipo   | Esempio               | Descrizione                           |
| :---------- | :----- | :-------------------- | :------------------------------------ |
| `file_path` | string | `"/path/to/file.txt"` | Percorso assoluto al file da scrivere |
| `content`   | string | `"file content"`      | Contenuto da scrivere nel file        |

<h5 id="edit">
  Edit
</h5>

Sostituisce una stringa in un file esistente.

| Campo         | Tipo    | Esempio               | Descrizione                             |
| :------------ | :------ | :-------------------- | :-------------------------------------- |
| `file_path`   | string  | `"/path/to/file.txt"` | Percorso assoluto al file da modificare |
| `old_string`  | string  | `"original text"`     | Testo da trovare e sostituire           |
| `new_string`  | string  | `"replacement text"`  | Testo di sostituzione                   |
| `replace_all` | boolean | `false`               | Se sostituire tutte le occorrenze       |

<h5 id="read">
  Read
</h5>

Legge il contenuto del file.

| Campo       | Tipo   | Esempio               | Descrizione                                           |
| :---------- | :----- | :-------------------- | :---------------------------------------------------- |
| `file_path` | string | `"/path/to/file.txt"` | Percorso assoluto al file da leggere                  |
| `offset`    | number | `10`                  | Numero di riga facoltativo da cui iniziare la lettura |
| `limit`     | number | `50`                  | Numero facoltativo di righe da leggere                |

<h5 id="glob">
  Glob
</h5>

Trova i file che corrispondono a un modello glob.

| Campo     | Tipo   | Esempio          | Descrizione                                                                                  |
| :-------- | :----- | :--------------- | :------------------------------------------------------------------------------------------- |
| `pattern` | string | `"**/*.ts"`      | Modello glob per abbinare i file                                                             |
| `path`    | string | `"/path/to/dir"` | Directory facoltativa in cui cercare. Impostazione predefinita: directory di lavoro corrente |

<h5 id="grep">
  Grep
</h5>

Cerca il contenuto dei file con espressioni regolari.

| Campo         | Tipo    | Esempio          | Descrizione                                                                                       |
| :------------ | :------ | :--------------- | :------------------------------------------------------------------------------------------------ |
| `pattern`     | string  | `"TODO.*fix"`    | Modello di espressione regolare da cercare                                                        |
| `path`        | string  | `"/path/to/dir"` | File o directory facoltativa in cui cercare                                                       |
| `glob`        | string  | `"*.ts"`         | Modello glob facoltativo per filtrare i file                                                      |
| `output_mode` | string  | `"content"`      | `"content"`, `"files_with_matches"` o `"count"`. Impostazione predefinita: `"files_with_matches"` |
| `-i`          | boolean | `true`           | Ricerca senza distinzione tra maiuscole e minuscole                                               |
| `multiline`   | boolean | `false`          | Abilita l'abbinamento multilinea                                                                  |

<h5 id="webfetch">
  WebFetch
</h5>

Recupera ed elabora il contenuto web.

| Campo    | Tipo   | Esempio                       | Descrizione                                 |
| :------- | :----- | :---------------------------- | :------------------------------------------ |
| `url`    | string | `"https://example.com/api"`   | URL da cui recuperare il contenuto          |
| `prompt` | string | `"Extract the API endpoints"` | Prompt da eseguire sul contenuto recuperato |

<h5 id="websearch">
  WebSearch
</h5>

Cerca il web.

| Campo             | Tipo   | Esempio                        | Descrizione                                              |
| :---------------- | :----- | :----------------------------- | :------------------------------------------------------- |
| `query`           | string | `"react hooks best practices"` | Query di ricerca                                         |
| `allowed_domains` | array  | `["docs.example.com"]`         | Facoltativo: includere solo i risultati da questi domini |
| `blocked_domains` | array  | `["spam.example.com"]`         | Facoltativo: escludere i risultati da questi domini      |

<h5 id="agent">
  Agent
</h5>

Genera un [subagent](/docs/it/sub-agents).

| Campo           | Tipo   | Esempio                    | Descrizione                                                                |
| :-------------- | :----- | :------------------------- | :------------------------------------------------------------------------- |
| `prompt`        | string | `"Find all API endpoints"` | L'attività per l'agente da eseguire                                        |
| `description`   | string | `"Find API endpoints"`     | Breve descrizione dell'attività                                            |
| `subagent_type` | string | `"Explore"`                | Tipo di agente specializzato da utilizzare                                 |
| `model`         | string | `"sonnet"`                 | Alias del modello facoltativo per sovrascrivere l'impostazione predefinita |

In `PostToolUse`, `tool_response` per una chiamata Agent completata contiene il testo finale del subagent insieme alla telemetria di utilizzo. Leggere questi campi per registrare il costo per subagent da un hook:

| Campo               | Tipo   | Esempio                                               | Descrizione                                                                                                                                                                                                                                                        |
| :------------------ | :----- | :---------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`            | string | `"completed"`                                         | `"completed"` per i subagent in primo piano, `"async_launched"` per i subagent in background. A partire da v2.1.198, i subagent vengono eseguiti in background per impostazione predefinita, quindi un `run_in_background` omesso produce anche `"async_launched"` |
| `agentId`           | string | `"a4d2c8f1e0b3a297"`                                  | Identificatore per l'esecuzione del subagent                                                                                                                                                                                                                       |
| `content`           | array  | `[{"type": "text", "text": "Found 12 endpoints..."}]` | I blocchi di testo finali del subagent                                                                                                                                                                                                                             |
| `resolvedModel`     | string | `"claude-sonnet-4-5"`                                 | Modello su cui è stato eseguito il subagent, che può differire dal modello richiesto. Richiede Claude Code v2.1.174 o successivo                                                                                                                                   |
| `totalTokens`       | number | `12450`                                               | Token totali fatturati nei turni del subagent                                                                                                                                                                                                                      |
| `totalDurationMs`   | number | `48211`                                               | Durata wall-clock dell'esecuzione del subagent                                                                                                                                                                                                                     |
| `totalToolUseCount` | number | `7`                                                   | Conteggio delle chiamate dello strumento effettuate dal subagent                                                                                                                                                                                                   |
| `usage`             | object | `{"input_tokens": 8320, ...}`                         | Suddivisione dei token per tipo: `input_tokens`, `output_tokens`, `cache_creation_input_tokens`, `cache_read_input_tokens`                                                                                                                                         |

Per i subagent in background, lo strumento ritorna immediatamente dopo il lancio, quindi `tool_response` non contiene campi di utilizzo. Ha `status: "async_launched"`, `agentId`, `description`, `prompt`, `outputFile` e `resolvedModel` invece.

Il campo `resolvedModel` nomina il modello su cui il subagent effettivamente viene eseguito, che può differire dal valore `model` in `tool_input`, ad esempio quando `availableModels` o un altro override si applica. Richiede Claude Code v2.1.174 o successivo.

<a id="askuserquestion" />

<h5 id="askuserquestion">
  AskUserQuestion
</h5>

Chiede all'utente da una a quattro domande a scelta multipla.

| Campo       | Tipo   | Esempio                                                                                                            | Descrizione                                                                                                                                                                                                                                               |
| :---------- | :----- | :----------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `questions` | array  | `[{"question": "Which framework?", "header": "Framework", "options": [{"label": "React"}], "multiSelect": false}]` | Domande da presentare, ciascuna con una stringa `question`, un `header` breve, un array `options` e un flag `multiSelect` facoltativo                                                                                                                     |
| `answers`   | object | `{"Which framework?": "React"}`                                                                                    | Facoltativo. Mappa il testo della domanda all'etichetta dell'opzione selezionata. Le risposte multi-select uniscono le etichette con virgole. Claude non imposta questo campo; fornirlo tramite `updatedInput` per rispondere a livello di programmazione |

<h5 id="exitplanmode">
  ExitPlanMode
</h5>

Presenta un piano e chiede all'utente di approvarlo prima che Claude lasci la [modalità piano](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode). Claude scrive il piano in un file su disco prima di chiamare lo strumento, quindi l'`tool_input` letterale dal modello è tipicamente vuoto. Claude Code inietta il contenuto del piano e il percorso del file prima di passare l'input agli hook.

| Campo            | Tipo   | Esempio                                     | Descrizione                                                                                                                                                               |
| :--------------- | :----- | :------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `plan`           | string | `"## Refactor auth\n1. Extract..."`         | Contenuto del piano in Markdown. Iniettato dal file del piano su disco                                                                                                    |
| `planFilePath`   | string | `"/Users/.../plans/refactor-auth.md"`       | Percorso al file del piano. Iniettato                                                                                                                                     |
| `allowedPrompts` | array  | `[{"tool": "Bash", "prompt": "run tests"}]` | Deprecato. Claude Code accetta il campo ma lo ignora. Prima di v2.1.205, conteneva autorizzazioni basate su prompt che Claude stava richiedendo per implementare il piano |

In `PostToolUse`, `tool_response` è un oggetto con i campi `plan` e `filePath` che contengono il piano approvato, più flag di stato interni. Leggere `tool_response.plan` per il contenuto del piano piuttosto che rileggere il file da disco.

<h4 id="pretooluse-decision-control">
  Controllo della decisione di PreToolUse
</h4>

Gli hook `PreToolUse` possono controllare se una chiamata dello strumento procede. A differenza di altri hook che utilizzano un campo `decision` di livello superiore, PreToolUse restituisce la sua decisione all'interno di un oggetto `hookSpecificOutput`. Ciò gli dà un controllo più ricco: quattro risultati (consentire, negare, chiedere o rinviare) più la capacità di modificare l'input dello strumento prima dell'esecuzione.

| Campo                      | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissionDecision`       | `"allow"` bypassa il prompt di autorizzazione, tranne per [strumenti che richiedono l'interazione dell'utente](#pretooluse-decision-control) e strumenti connettore [che l'organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools). `"deny"` impedisce la chiamata dello strumento. `"ask"` richiede all'utente di confermare. `"defer"` esce correttamente in modo che lo strumento possa essere ripreso in seguito. Le regole [Deny and ask](/docs/it/permissions#manage-permissions) si applicano ancora indipendentemente da quello che l'hook restituisce |
| `permissionDecisionReason` | Per `"allow"` e `"ask"`, mostrato all'utente ma non a Claude. Per `"deny"`, mostrato a Claude. Per `"defer"`, ignorato                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `updatedInput`             | Modifica i parametri di input dello strumento prima dell'esecuzione. Sostituisce l'intero oggetto di input, quindi includere i campi invariati insieme a quelli modificati. Combinare con `"allow"` per l'approvazione automatica o `"ask"` per mostrare l'input modificato all'utente. Per `"defer"`, ignorato                                                                                                                                                                                                                                                                           |
| `additionalContext`        | Stringa aggiunta al contesto di Claude insieme al risultato dello strumento. Ignorato quando `permissionDecision` è `"defer"`. Consultare [Aggiungere contesto per Claude](#add-context-for-claude)                                                                                                                                                                                                                                                                                                                                                                                       |

Quando più hook PreToolUse restituiscono decisioni diverse, la precedenza è `deny` > `defer` > `ask` > `allow`.

Quando un hook restituisce `"ask"`, il prompt di autorizzazione visualizzato all'utente include un'etichetta che identifica da dove proviene l'hook: ad esempio, `[User]`, `[Project]`, `[Plugin]` o `[Local]`. Ciò aiuta gli utenti a capire quale fonte di configurazione sta richiedendo la conferma.

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

`AskUserQuestion` e `ExitPlanMode` richiedono l'interazione dell'utente e normalmente bloccano in [modalità non interattiva](/docs/it/headless) con il flag `-p`. Restituire `permissionDecision: "allow"` insieme a `updatedInput` soddisfa quel requisito: l'hook legge l'input dello strumento da stdin, raccoglie la risposta attraverso la propria interfaccia utente e la restituisce in `updatedInput` in modo che lo strumento venga eseguito senza richiedere. Restituire `"allow"` da solo non è sufficiente per questi strumenti. Per `AskUserQuestion`, ripetere l'array `questions` originale e aggiungere un oggetto [`answers`](#askuserquestion) che mappa il testo di ogni domanda alla risposta scelta. Le risposte multi-select uniscono le etichette con virgole. Claude non imposta questo campo; fornirlo tramite `updatedInput` per rispondere a livello di programmazione.

Strumenti connettore [che l'organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) richiedono il prompt anche quando un hook restituisce `"allow"`.

A partire da v2.1.199, uno strumento MCP il cui server lo contrassegna con [`_meta["anthropic/requiresUserInteraction"]`](/docs/it/mcp#require-approval-for-a-specific-tool) è più rigoroso: un hook non può saltare il suo prompt di approvazione con `"allow"`, con o senza `updatedInput`, perché Claude Code non può confermare che l'hook ha raccolto l'interazione di cui lo strumento ha bisogno.

<Note>
  PreToolUse in precedenza utilizzava i campi `decision` e `reason` di livello superiore, ma questi sono deprecati per questo evento. Utilizzare invece `hookSpecificOutput.permissionDecision` e `hookSpecificOutput.permissionDecisionReason`. I valori deprecati `"approve"` e `"block"` si mappano a `"allow"` e `"deny"` rispettivamente. Gli altri eventi come PostToolUse e Stop continuano a utilizzare `decision` e `reason` di livello superiore come formato corrente.
</Note>

<h4 id="defer-a-tool-call-for-later">
  Rinviare una chiamata dello strumento per dopo
</h4>

`"defer"` è per le integrazioni che eseguono `claude -p` come subprocess e leggono il suo output JSON, come un'app Agent SDK o un'interfaccia utente personalizzata costruita su Claude Code. Consente a quel processo chiamante di mettere in pausa Claude in una chiamata dello strumento, raccogliere input attraverso la sua interfaccia e riprendere da dove era rimasto. Claude Code onora questo valore solo in [modalità non interattiva](/docs/it/headless) con il flag `-p`. Nelle sessioni interattive registra un avviso e ignora il risultato del hook.

Lo strumento `AskUserQuestion` è il caso tipico: Claude vuole chiedere qualcosa all'utente, ma non c'è un terminale per rispondere. Il round trip funziona così:

1. Claude chiama `AskUserQuestion`. L'hook `PreToolUse` si attiva.
2. L'hook restituisce `permissionDecision: "defer"`. Lo strumento non viene eseguito. Il processo esce con `stop_reason: "tool_deferred"` e la chiamata dello strumento in sospeso preservata nella trascrizione.
3. Il processo chiamante legge `deferred_tool_use` dal risultato SDK, visualizza la domanda nella sua interfaccia utente e attende una risposta.
4. Il processo chiamante esegue `claude -p --resume <session-id>`. La stessa chiamata dello strumento attiva `PreToolUse` di nuovo.
5. L'hook restituisce `permissionDecision: "allow"` con la risposta in `updatedInput`. Lo strumento viene eseguito e Claude continua.

Il campo `deferred_tool_use` contiene l'`id`, il `name` e l'`input` dello strumento. L'`input` è i parametri che Claude ha generato per la chiamata dello strumento, acquisiti prima dell'esecuzione:

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

Non c'è timeout o limite di tentativi. La sessione rimane su disco fino a quando non la riprendi, soggetta alla [pulizia di conservazione](/docs/it/settings#available-settings) `cleanupPeriodDays` che elimina i file di sessione dopo 30 giorni per impostazione predefinita. Se la risposta non è pronta quando riprendi, l'hook può restituire `"defer"` di nuovo e il processo esce nello stesso modo. Il processo chiamante controlla quando interrompere il ciclo restituendo infine `"allow"` o `"deny"` dall'hook.

`"defer"` funziona solo quando Claude effettua una singola chiamata dello strumento nel turno. Se Claude effettua più chiamate dello strumento contemporaneamente, `"defer"` viene ignorato con un avviso e lo strumento procede attraverso il flusso di autorizzazione normale. Il vincolo esiste perché resume può solo rieseguire uno strumento: non c'è modo di rinviare una chiamata da un batch senza lasciare le altre irrisolte.

Se lo strumento rinviato non è più disponibile quando riprendi, il processo esce con `stop_reason: "tool_deferred_unavailable"` e `is_error: true` prima che l'hook si attivi. Questo accade quando un server MCP che ha fornito lo strumento non è connesso per la sessione ripresa. Il payload `deferred_tool_use` è ancora incluso in modo da poter identificare quale strumento è scomparso.

<Note>
  `--resume` ripristina la modalità di autorizzazione che era attiva quando lo strumento è stato rinviato, quindi non è necessario passare di nuovo `--permission-mode`. Le eccezioni sono `plan` e `bypassPermissions`, che non vengono mai trasportati. Passare `--permission-mode` esplicitamente su resume sovrascrive il valore ripristinato.
</Note>

<h3 id="permissionrequest">
  PermissionRequest
</h3>

Viene eseguito quando all'utente viene mostrata una finestra di dialogo di autorizzazione.
Utilizzare il [PermissionRequest decision control](#permissionrequest-decision-control) per consentire o negare per conto dell'utente.

Corrisponde al nome dello strumento, stessi valori di PreToolUse.

<h4 id="permissionrequest-input">
  Input di PermissionRequest
</h4>

Gli hook PermissionRequest ricevono i campi `tool_name` e `tool_input` come gli hook PreToolUse, ma senza `tool_use_id`. Un array `permission_suggestions` facoltativo contiene le opzioni "consenti sempre" che l'utente normalmente vedrebbe nella finestra di dialogo di autorizzazione. La differenza è quando l'hook si attiva: gli hook PermissionRequest vengono eseguiti quando una finestra di dialogo di autorizzazione sta per essere mostrata all'utente, mentre gli hook PreToolUse vengono eseguiti prima dell'esecuzione dello strumento indipendentemente dallo stato di autorizzazione.

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
  Controllo della decisione di PermissionRequest
</h4>

Gli hook `PermissionRequest` possono consentire o negare le richieste di autorizzazione. Oltre ai [campi di output JSON](#json-output) disponibili per tutti gli hook, lo script del hook può restituire un oggetto `decision` con questi campi specifici dell'evento:

| Campo                | Descrizione                                                                                                                                                                                                                                                                       |
| :------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `behavior`           | `"allow"` concede l'autorizzazione, `"deny"` la nega. Le regole [Deny and ask](/docs/it/permissions#manage-permissions) si applicano ancora, quindi un hook che restituisce `"allow"` non sovrascrive una regola di negazione corrispondente                                           |
| `updatedInput`       | Solo per `"allow"`: modifica i parametri di input dello strumento prima dell'esecuzione. Sostituisce l'intero oggetto di input, quindi includere i campi invariati insieme a quelli modificati. L'input modificato viene rivalutato rispetto alle regole di negazione e richiesta |
| `updatedPermissions` | Solo per `"allow"`: array di [permission update entries](#permission-update-entries) da applicare, come l'aggiunta di una regola di consentimento o la modifica della modalità di autorizzazione della sessione                                                                   |
| `message`            | Solo per `"deny"`: dice a Claude perché l'autorizzazione è stata negata                                                                                                                                                                                                           |
| `interrupt`          | Solo per `"deny"`: se `true`, interrompe Claude                                                                                                                                                                                                                                   |

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

Il campo di output `updatedPermissions` e il campo di input [`permission_suggestions`](#permissionrequest-input) utilizzano entrambi lo stesso array di oggetti di voce. Ogni voce ha un `type` che determina i suoi altri campi e una `destination` che controlla dove viene scritta la modifica.

| `type`              | Campi                              | Effetto                                                                                                                                                                                                                                   |
| :------------------ | :--------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `addRules`          | `rules`, `behavior`, `destination` | Aggiunge regole di autorizzazione. `rules` è un array di oggetti `{toolName, ruleContent?}`. Omettere `ruleContent` per abbinare l'intero strumento. `behavior` è `"allow"`, `"deny"` o `"ask"`                                           |
| `replaceRules`      | `rules`, `behavior`, `destination` | Sostituisce tutte le regole del `behavior` dato alla `destination` con le `rules` fornite                                                                                                                                                 |
| `removeRules`       | `rules`, `behavior`, `destination` | Rimuove le regole corrispondenti del `behavior` dato                                                                                                                                                                                      |
| `setMode`           | `mode`, `destination`              | Modifica la modalità di autorizzazione. Le modalità valide sono `default`, `auto`, `acceptEdits`, `dontAsk`, `bypassPermissions`, `plan` e `manual` come alias per `default`. L'alias `manual` richiede Claude Code v2.1.200 o successivo |
| `addDirectories`    | `directories`, `destination`       | Aggiunge directory di lavoro. `directories` è un array di stringhe di percorso                                                                                                                                                            |
| `removeDirectories` | `directories`, `destination`       | Rimuove directory di lavoro                                                                                                                                                                                                               |

<Note>
  `setMode` con `bypassPermissions` ha effetto solo se la sessione è stata avviata con la modalità bypass già disponibile: `--dangerously-skip-permissions`, `--permission-mode bypassPermissions`, `--allow-dangerously-skip-permissions` o `permissions.defaultMode: "bypassPermissions"` nelle impostazioni, e la modalità non è disabilitata da [`permissions.disableBypassPermissionsMode`](/docs/it/permissions#managed-settings). Altrimenti l'aggiornamento è un no-op. `bypassPermissions` non viene mai persistito come `defaultMode` indipendentemente da `destination`.
</Note>

Il campo `destination` su ogni voce determina se la modifica rimane in memoria o persiste in un file di impostazioni.

| `destination`     | Scrive in                                            |
| :---------------- | :--------------------------------------------------- |
| `session`         | solo in memoria, scartato quando la sessione termina |
| `localSettings`   | `.claude/settings.local.json`                        |
| `projectSettings` | `.claude/settings.json`                              |
| `userSettings`    | `~/.claude/settings.json`                            |

Un hook può ripetere uno dei `permission_suggestions` che ha ricevuto come suo proprio output `updatedPermissions`, che è equivalente all'utente che seleziona quell'opzione "consenti sempre" nella finestra di dialogo.

<h3 id="posttooluse">
  PostToolUse
</h3>

Viene eseguito immediatamente dopo il completamento riuscito di uno strumento.

Corrisponde al nome dello strumento, stessi valori di PreToolUse.

<h4 id="posttooluse-input">
  Input di PostToolUse
</h4>

Gli hook `PostToolUse` si attivano dopo che uno strumento è già stato eseguito con successo. L'input include sia `tool_input`, gli argomenti inviati allo strumento, che `tool_response`, il risultato che ha restituito. Lo schema esatto per entrambi dipende dallo strumento.

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
    "success": true
  },
  "tool_use_id": "toolu_01ABC123...",
  "duration_ms": 12
}
```

| Campo         | Descrizione                                                                                                                                       |
| :------------ | :------------------------------------------------------------------------------------------------------------------------------------------------ |
| `duration_ms` | Facoltativo. Tempo di esecuzione dello strumento in millisecondi. Esclude il tempo trascorso nei prompt di autorizzazione e negli hook PreToolUse |

<h4 id="posttooluse-decision-control">
  Controllo della decisione di PostToolUse
</h4>

Gli hook `PostToolUse` possono fornire feedback a Claude dopo l'esecuzione dello strumento. Oltre ai [campi di output JSON](#json-output) disponibili per tutti gli hook, lo script del hook può restituire questi campi specifici dell'evento:

| Campo                  | Descrizione                                                                                                                                                    |
| :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `decision`             | `"block"` aggiunge il `reason` accanto al risultato dello strumento. Claude vede ancora l'output originale; per sostituirlo, utilizzare `updatedToolOutput`    |
| `reason`               | Spiegazione mostrata a Claude quando `decision` è `"block"`                                                                                                    |
| `additionalContext`    | Stringa aggiunta al contesto di Claude insieme al risultato dello strumento. Consultare [Aggiungere contesto per Claude](#add-context-for-claude)              |
| `updatedToolOutput`    | Sostituisce l'output dello strumento con il valore fornito prima che venga inviato a Claude. Il valore deve corrispondere alla forma di output dello strumento |
| `updatedMCPToolOutput` | Sostituisce l'output solo per [strumenti MCP](#match-mcp-tools). Preferire `updatedToolOutput`, che funziona per tutti gli strumenti                           |

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
  `updatedToolOutput` cambia solo ciò che Claude vede. Lo strumento è già stato eseguito al momento dell'attivazione dell'hook, quindi tutti i file scritti, i comandi eseguiti o le richieste di rete inviate hanno già avuto effetto. La telemetria come gli span degli strumenti OpenTelemetry e gli eventi di analisi acquisiscono anche l'output originale prima dell'esecuzione dell'hook. Per impedire o modificare una chiamata dello strumento prima che venga eseguita, utilizzare un hook [PreToolUse](#pretooluse) invece.

  Il valore di sostituzione deve corrispondere alla forma di output dello strumento. Gli strumenti incorporati restituiscono oggetti strutturati piuttosto che stringhe semplici. Ad esempio, `Bash` restituisce un oggetto con i campi `stdout`, `stderr`, `interrupted` e `isImage`. Per gli strumenti incorporati, un valore che non corrisponde allo schema di output dello strumento viene ignorato e viene utilizzato l'output originale. L'output dello strumento MCP viene passato senza convalida dello schema. Rimuovere i dettagli di errore di cui Claude ha bisogno può causare il proseguimento su un presupposto falso.
</Warning>

<h3 id="posttoolusefailure">
  PostToolUseFailure
</h3>

Viene eseguito quando l'esecuzione di uno strumento non riesce: lo strumento ha generato un errore o uno strumento MCP ha restituito un risultato di errore. Utilizzare questo per registrare i guasti, inviare avvisi o fornire feedback correttivo a Claude.

Corrisponde al nome dello strumento, stessi valori di PreToolUse.

<Note>
  Questo evento non si attiva per le chiamate dello strumento rifiutate prima dell'esecuzione: un nome di strumento sconosciuto, input che non supera la convalida dello schema o dello strumento specifico, o un rifiuto di autorizzazione. I rifiuti di convalida vengono restituiti come risultati `tool_use_error` e si verificano prima che gli hook vengano eseguiti, quindi non attivano né `PreToolUse` né questo evento. I rifiuti di autorizzazione attivano `PreToolUse` ma non questo evento; consultare [PermissionDenied](#permissiondenied).
</Note>

<h4 id="posttoolusefailure-input">
  Input di PostToolUseFailure
</h4>

Gli hook PostToolUseFailure ricevono gli stessi campi `tool_name` e `tool_input` di PostToolUse, insieme alle informazioni di errore come campi di livello superiore:

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
  "error": "Command exited with non-zero status code 1",
  "is_interrupt": false,
  "duration_ms": 4187
}
```

| Campo          | Descrizione                                                                                                                                       |
| :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------ |
| `error`        | Stringa che descrive cosa è andato storto                                                                                                         |
| `is_interrupt` | Booleano facoltativo che indica se il guasto è stato causato dall'interruzione dell'utente                                                        |
| `duration_ms`  | Facoltativo. Tempo di esecuzione dello strumento in millisecondi. Esclude il tempo trascorso nei prompt di autorizzazione e negli hook PreToolUse |

<h4 id="posttoolusefailure-decision-control">
  Controllo della decisione di PostToolUseFailure
</h4>

Gli hook `PostToolUseFailure` possono fornire contesto a Claude dopo un guasto dello strumento. Oltre ai [campi di output JSON](#json-output) disponibili per tutti gli hook, lo script del hook può restituire questi campi specifici dell'evento:

| Campo               | Descrizione                                                                                                                     |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------ |
| `additionalContext` | Stringa aggiunta al contesto di Claude insieme all'errore. Consultare [Aggiungere contesto per Claude](#add-context-for-claude) |

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

Viene eseguito una volta dopo che ogni chiamata dello strumento in un batch è stata risolta, prima che Claude Code invii la richiesta successiva al modello. `PostToolUse` si attiva una volta per strumento, il che significa che si attiva contemporaneamente quando Claude effettua chiamate dello strumento parallele. `PostToolBatch` si attiva esattamente una volta con l'intero batch, quindi è il posto giusto per iniettare contesto che dipende dall'insieme di strumenti che hanno eseguito piuttosto che da qualsiasi singolo strumento. Non c'è matcher per questo evento.

<h4 id="posttoolbatch-input">
  Input di PostToolBatch
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook PostToolBatch ricevono `tool_calls`, un array che descrive ogni chiamata dello strumento nel batch:

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

`tool_response` contiene lo stesso contenuto che il modello riceve nel blocco `tool_result` corrispondente. Il valore è una stringa serializzata o un array di blocchi di contenuto, esattamente come lo strumento lo ha emesso. Per `Read`, ciò significa testo con prefisso numero di riga piuttosto che contenuti di file grezzi. Le risposte possono essere grandi, quindi analizzare solo i campi di cui hai bisogno.

<Note>
  La forma `tool_response` differisce da quella di `PostToolUse`. `PostToolUse` passa l'oggetto `Output` strutturato dello strumento, come `{filePath: "...", success: true}` per `Write`; `PostToolBatch` passa il contenuto `tool_result` serializzato che il modello vede.
</Note>

<h4 id="posttoolbatch-decision-control">
  Controllo della decisione di PostToolBatch
</h4>

Gli hook `PostToolBatch` possono iniettare contesto per Claude. Oltre ai [campi di output JSON](#json-output) disponibili per tutti gli hook, lo script del hook può restituire questi campi specifici dell'evento:

| Campo               | Descrizione                                                                                                                                                                                                                                               |
| :------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | Stringa di contesto iniettata una volta prima della prossima chiamata del modello. Consultare [Aggiungere contesto per Claude](#add-context-for-claude) per i dettagli di consegna, cosa inserirvi e come le sessioni riprese gestiscono i valori passati |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolBatch",
    "additionalContext": "These files are part of the ledger module. Run pytest before marking the task complete."
  }
}
```

Restituire `decision: "block"` o `continue: false` interrompe il ciclo agentico prima della prossima chiamata del modello.

<h3 id="permissiondenied">
  PermissionDenied
</h3>

Viene eseguito quando il classificatore della [modalità automatica](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) nega una chiamata dello strumento. Questo hook si attiva solo in modalità automatica: non viene eseguito quando si nega manualmente una finestra di dialogo di autorizzazione, quando un hook `PreToolUse` blocca una chiamata o quando una regola `deny` corrisponde. Utilizzare questo per registrare i rifiuti del classificatore, regolare la configurazione o dire al modello che può riprovare la chiamata dello strumento.

Corrisponde al nome dello strumento, stessi valori di PreToolUse.

<h4 id="permissiondenied-input">
  Input di PermissionDenied
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook PermissionDenied ricevono `tool_name`, `tool_input`, `tool_use_id` e `reason`.

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
  "reason": "Auto mode denied: command targets a path outside the project"
}
```

| Campo    | Descrizione                                                                                        |
| :------- | :------------------------------------------------------------------------------------------------- |
| `reason` | La spiegazione del classificatore per il motivo per cui la chiamata dello strumento è stata negata |

<h4 id="permissiondenied-decision-control">
  Controllo della decisione di PermissionDenied
</h4>

Gli hook PermissionDenied possono dire al modello che può riprovare la chiamata dello strumento negata. Restituire un oggetto JSON con `hookSpecificOutput.retry` impostato su `true`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionDenied",
    "retry": true
  }
}
```

Quando `retry` è `true`, Claude Code aggiunge un messaggio alla conversazione dicendo al modello che può riprovare la chiamata dello strumento. Il rifiuto stesso non viene invertito. Se l'hook non restituisce JSON o restituisce `retry: false`, il rifiuto rimane e il modello riceve il messaggio di rifiuto originale.

<h3 id="notification">
  Notification
</h3>

Viene eseguito quando Claude Code invia notifiche. Corrisponde al tipo di notifica. Omettere il matcher per eseguire gli hook per tutti i tipi di notifica.

| Matcher                | Quando si attiva                                                                                                                         |
| :--------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| `permission_prompt`    | Claude ha bisogno dell'approvazione di uno strumento                                                                                     |
| `idle_prompt`          | Claude è terminato e in attesa del prompt successivo                                                                                     |
| `auth_success`         | L'autenticazione è completata                                                                                                            |
| `elicitation_dialog`   | Un server MCP apre un modulo di elicitazione                                                                                             |
| `elicitation_complete` | Un modulo di elicitazione MCP viene inviato o chiuso                                                                                     |
| `elicitation_response` | Una risposta di elicitazione MCP viene inviata al server                                                                                 |
| `agent_needs_input`    | Una sessione in background inizia ad attendere l'input. Si attiva solo mentre la [vista agente](/docs/it/agent-view) è aperta in un terminale |
| `agent_completed`      | Una sessione in background termina o non riesce. Si attiva solo mentre la [vista agente](/docs/it/agent-view) è aperta in un terminale        |

I tipi `agent_needs_input` e `agent_completed` richiedono Claude Code v2.1.198 o successivo.

Utilizzare matcher separati per eseguire gestori diversi a seconda del tipo di notifica. Questa configurazione attiva uno script di avviso specifico per l'autorizzazione quando Claude ha bisogno dell'approvazione dell'autorizzazione e una notifica diversa quando Claude è stato inattivo:

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
  Input di Notification
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook Notification ricevono `message` con il testo della notifica, un `title` facoltativo e `notification_type` che indica quale tipo si è attivato.

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

Gli hook Notification non possono bloccare o modificare le notifiche. Sono destinati agli effetti collaterali come l'inoltro della notifica a un servizio esterno. I [campi di output JSON](#json-output) comuni come `systemMessage` si applicano.

<h3 id="subagentstart">
  SubagentStart
</h3>

Viene eseguito quando un subagent di Claude Code viene generato tramite lo strumento Agent. Supporta i matcher per filtrare per nome del tipo di agente. Per gli agenti incorporati, questo è il nome dell'agente come `general-purpose`, `Explore` o `Plan`. Per i [subagent personalizzati](/docs/it/sub-agents), questo è il campo `name` dal frontmatter dell'agente, non il nome del file.

Per i subagent forniti da un [plugin](/docs/it/plugins), il tipo di agente è l'identificatore con ambito plugin come `my-plugin:reviewer`, non il nome del frontmatter nudo. I due punti posizionano un nome con ambito plugin sul percorso dell'espressione regolare, quindi ancorare il matcher con `^` e `$` per una corrispondenza esatta: `^my-plugin:reviewer$`.

<h4 id="subagentstart-input">
  Input di SubagentStart
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook SubagentStart ricevono `agent_id` con l'identificatore univoco per il subagent e `agent_type` con il nome dell'agente che il matcher filtra.

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

Gli hook SubagentStart non possono bloccare la creazione del subagent, ma possono iniettare contesto nel subagent. Oltre ai [campi di output JSON](#json-output) disponibili per tutti gli hook, è possibile restituire:

| Campo               | Descrizione                                                                                                                                                                    |
| :------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | Stringa aggiunta al contesto del subagent all'inizio della sua conversazione, prima del suo primo prompt. Consultare [Aggiungere contesto per Claude](#add-context-for-claude) |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "SubagentStart",
    "additionalContext": "Follow security guidelines for this task"
  }
}
```

<h3 id="subagentstop">
  SubagentStop
</h3>

Viene eseguito quando un subagent di Claude Code ha finito di rispondere. Corrisponde al tipo di agente, stessi valori di SubagentStart.

<h4 id="subagentstop-input">
  Input di SubagentStop
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook SubagentStop ricevono `stop_hook_active`, `agent_id`, `agent_type`, `agent_transcript_path` e `last_assistant_message`. Il campo `agent_type` è il valore utilizzato per il filtraggio del matcher. Il `transcript_path` è la trascrizione della sessione principale, mentre `agent_transcript_path` è la trascrizione propria del subagent archiviata in una cartella `subagents/` annidato. Il campo `last_assistant_message` contiene il contenuto del testo della risposta finale del subagent, quindi gli hook possono accedervi senza analizzare il file della trascrizione.

Gli hook SubagentStop ricevono anche gli array `background_tasks` e `session_crons` descritti in [Stop input](#stop-input), disponibili in Claude Code v2.1.145 o successivo. Entrambi gli array sono limitati alla sessione padre, non al subagent.

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

Gli hook SubagentStop utilizzano lo stesso formato di controllo della decisione degli [hook Stop](#stop-decision-control), incluso `hookSpecificOutput.additionalContext` con `hookEventName` impostato su `"SubagentStop"`, per il feedback non-errore che mantiene il subagent in esecuzione. Restituire `decision: "block"` con un `reason` mantiene il subagent in esecuzione e consegna `reason` al subagent come sua prossima istruzione. Per iniettare contesto nella sessione padre dopo il ritorno di un subagent, utilizzare un hook [`PostToolUse`](#posttooluse) sullo strumento `Agent` invece.

<h3 id="taskcreated">
  TaskCreated
</h3>

Viene eseguito quando un'attività sta per essere creata tramite lo strumento `TaskCreate`. Utilizzare questo per applicare le convenzioni di denominazione, richiedere descrizioni delle attività o impedire la creazione di determinate attività.

Quando un hook `TaskCreated` esce con il codice 2, l'attività non viene creata e il messaggio stderr viene restituito al modello come feedback. Per interrompere completamente il compagno di squadra invece di rieseguirlo, restituire JSON con `{"continue": false, "stopReason": "..."}`. Gli hook TaskCreated non supportano i matcher e si attivano ad ogni occorrenza.

<h4 id="taskcreated-input">
  Input di TaskCreated
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook TaskCreated ricevono `task_id`, `task_subject` e facoltativamente `task_description`, `teammate_name` e `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "TaskCreated",
  "task_id": "task-001",
  "task_subject": "Implement user authentication",
  "task_description": "Add login and signup endpoints",
  "teammate_name": "implementer",
  "team_name": "session-a1b2c3d4"
}
```

| Campo              | Descrizione                                                                 |
| :----------------- | :-------------------------------------------------------------------------- |
| `task_id`          | Identificatore dell'attività in corso di creazione                          |
| `task_subject`     | Titolo dell'attività                                                        |
| `task_description` | Descrizione dettagliata dell'attività. Può essere assente                   |
| `teammate_name`    | Nome del compagno di squadra che crea l'attività. Può essere assente        |
| `team_name`        | Nome del team derivato dalla sessione; verrà rimosso in una versione futura |

<h4 id="taskcreated-decision-control">
  Controllo della decisione di TaskCreated
</h4>

Gli hook TaskCreated supportano due modi per controllare la creazione dell'attività:

* **Codice di uscita 2**: l'attività non viene creata e il messaggio stderr viene restituito al modello come feedback.
* **JSON `{"continue": false, "stopReason": "..."}`**: interrompe completamente il compagno di squadra, corrispondendo al comportamento dell'hook `Stop`. Il `stopReason` viene mostrato all'utente.

Questo esempio blocca le attività i cui soggetti non seguono il formato richiesto:

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

Viene eseguito quando un'attività sta per essere contrassegnata come completata. Questo si attiva in due situazioni: quando qualsiasi agente contrassegna esplicitamente un'attività come completata attraverso lo strumento TaskUpdate, o quando un compagno di squadra di un [agent team](/docs/it/agent-teams) finisce il suo turno con attività in corso. Utilizzare questo per applicare i criteri di completamento come il passaggio dei test o dei controlli di linting prima che un'attività possa chiudersi.

Quando un hook `TaskCompleted` esce con il codice 2, l'attività non viene contrassegnata come completata e il messaggio stderr viene restituito al modello come feedback. Per interrompere completamente il compagno di squadra invece di rieseguirlo, restituire JSON con `{"continue": false, "stopReason": "..."}`. Gli hook TaskCompleted non supportano i matcher e si attivano ad ogni occorrenza.

<h4 id="taskcompleted-input">
  Input di TaskCompleted
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook TaskCompleted ricevono `task_id`, `task_subject` e facoltativamente `task_description`, `teammate_name` e `team_name`.

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

| Campo              | Descrizione                                                                 |
| :----------------- | :-------------------------------------------------------------------------- |
| `task_id`          | Identificatore dell'attività in corso di completamento                      |
| `task_subject`     | Titolo dell'attività                                                        |
| `task_description` | Descrizione dettagliata dell'attività. Può essere assente                   |
| `teammate_name`    | Nome del compagno di squadra che completa l'attività. Può essere assente    |
| `team_name`        | Nome del team derivato dalla sessione; verrà rimosso in una versione futura |

<h4 id="taskcompleted-decision-control">
  Controllo della decisione di TaskCompleted
</h4>

Gli hook TaskCompleted supportano due modi per controllare il completamento dell'attività:

* **Codice di uscita 2**: l'attività non viene contrassegnata come completata e il messaggio stderr viene restituito al modello come feedback.
* **JSON `{"continue": false, "stopReason": "..."}`**: interrompe completamente il compagno di squadra, corrispondendo al comportamento dell'hook `Stop`. Il `stopReason` viene mostrato all'utente.

Questo esempio esegue i test e blocca il completamento dell'attività se non riescono:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
TASK_SUBJECT=$(echo "$INPUT" | jq -r '.task_subject')

# Eseguire la suite di test
if ! npm test 2>&1; then
  echo "Tests not passing. Fix failing tests before completing: $TASK_SUBJECT" >&2
  exit 2
fi

exit 0
```

<h3 id="stop">
  Stop
</h3>

Viene eseguito quando l'agente Claude Code principale ha finito di rispondere. Non viene eseguito se l'arresto si è verificato a causa di un'interruzione dell'utente. Gli errori API attivano [StopFailure](#stopfailure) invece.

<Tip>
  Il comando [`/goal`](/docs/it/goal) è una scorciatoia incorporata per un hook Stop basato su prompt con ambito di sessione. Utilizzarlo quando si desidera che Claude continui a lavorare fino a quando una condizione non si verifica senza scrivere la configurazione dell'hook.
</Tip>

<h4 id="stop-input">
  Input di Stop
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook Stop ricevono `stop_hook_active`, `last_assistant_message`, `background_tasks` e `session_crons`. Il campo `stop_hook_active` è `true` quando Claude Code sta già continuando a causa di un hook di arresto. Controllare questo valore o elaborare la trascrizione per impedire a Claude Code di eseguire indefinitamente. Il campo `last_assistant_message` contiene il contenuto del testo della risposta finale di Claude, quindi gli hook possono accedervi senza analizzare il file della trascrizione.

Gli array `background_tasks` e `session_crons`, disponibili in Claude Code v2.1.145 o successivo, consentono agli hook di distinguere "la sessione è terminata" da "la sessione è in pausa in attesa che il lavoro in background la risvegli". Entrambi gli array sono presenti quando il registro delle attività è raggiungibile e sono vuoti quando non c'è nulla in volo o programmato.

Ogni voce in `background_tasks` descrive un'attività in volo e utilizza questi campi:

| Campo         | Descrizione                                                                                                                                                                                                                                                                    |
| :------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`          | Identificatore dell'attività                                                                                                                                                                                                                                                   |
| `type`        | Etichetta del tipo di attività amichevole come `shell`, `subagent`, `monitor`, `workflow`, `teammate`, `cloud session` o `MCP task`. Ogni etichetta identifica quale funzione di Claude Code ha creato l'attività. Ritorna al discriminante grezzo per i tipi non riconosciuti |
| `status`      | Stato attuale dell'attività                                                                                                                                                                                                                                                    |
| `description` | Descrizione in testo libero, limitata a 1000 caratteri con un marcatore `… [+N chars]` in-stringa quando ritagliato                                                                                                                                                            |
| `command`     | Riga di comando shell, limitata a 1000 caratteri. Presente solo per le attività `shell`                                                                                                                                                                                        |
| `agent_type`  | Nome del tipo di subagent. Presente solo per le attività `subagent`                                                                                                                                                                                                            |
| `server`      | Nome del server MCP. Presente solo per le attività `monitor` e `MCP task`                                                                                                                                                                                                      |
| `tool`        | Nome dello strumento MCP. Presente solo per le attività `monitor` e `MCP task`                                                                                                                                                                                                 |
| `name`        | Nome del flusso di lavoro. Presente solo per le attività `workflow`                                                                                                                                                                                                            |

Ogni voce in `session_crons` descrive un risveglio programmato con ambito di sessione, proveniente da `CronCreate`, `ScheduleWakeup` e `/loop`:

| Campo       | Descrizione                                                                                                                                                  |
| :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`        | Identificatore dell'attività cron                                                                                                                            |
| `schedule`  | Espressione cron, ad esempio `0 9 * * 1-5`                                                                                                                   |
| `recurring` | `false` per i risvegli una tantum il cui programma codifica un singolo tempo di attivazione, `true` per le attività che si riattivano ad ogni corrispondenza |
| `prompt`    | Prompt inviato quando il cron si attiva, limitato a 1000 caratteri con lo stesso marcatore `… [+N chars]`                                                    |

Questo esempio mostra un input Stop con un'attività shell in volo e un cron ricorrente:

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
  Controllo della decisione di Stop
</h4>

Gli hook `Stop` e `SubagentStop` possono controllare se Claude continua. Oltre ai [campi di output JSON](#json-output) disponibili per tutti gli hook, lo script del hook può restituire questi campi specifici dell'evento:

| Campo                                  | Descrizione                                                                                                                                                                                                                               |
| :------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `decision`                             | `"block"` impedisce a Claude di fermarsi. Omettere per consentire a Claude di fermarsi                                                                                                                                                    |
| `reason`                               | Obbligatorio quando `decision` è `"block"`. Dice a Claude perché dovrebbe continuare                                                                                                                                                      |
| `hookSpecificOutput.additionalContext` | Feedback non-errore per Claude. La conversazione continua in modo che Claude possa agire su di esso, ma a differenza di `decision: "block"` viene mostrato nella trascrizione come feedback dell'hook piuttosto che come errore dell'hook |

```json theme={null}
{
  "decision": "block",
  "reason": "Must be provided when Claude is blocked from stopping"
}
```

Utilizzare `additionalContext` quando l'hook funziona come previsto e fornisce a Claude una guida, come "eseguire la suite di test prima di terminare". Mantiene la conversazione attraverso gli stessi loop protections di `decision: "block"`, vale a dire l'input `stop_hook_active` e il limite di 8 continuazioni consecutive, ma la trascrizione lo etichetta come `Stop hook feedback` e nessuna notifica di errore dell'hook viene mostrata:

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

Viene eseguito invece di [Stop](#stop) quando il turno termina a causa di un errore API. L'output e il codice di uscita vengono ignorati. Utilizzare questo per registrare i guasti, inviare avvisi o intraprendere azioni di recupero quando Claude non può completare una risposta a causa di limiti di velocità, problemi di autenticazione o altri errori API.

<h4 id="stopfailure-input">
  Input di StopFailure
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook StopFailure ricevono `error`, `error_details` facoltativo e `last_assistant_message` facoltativo. Il campo `error` identifica il tipo di errore ed è utilizzato per il filtraggio del matcher.

| Campo                    | Descrizione                                                                                                                                                                                                                                                              |
| :----------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `error`                  | Tipo di errore: `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens` o `unknown`                                                                     |
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

Gli hook StopFailure non hanno controllo della decisione. Vengono eseguiti solo per scopi di notifica e registrazione.

<h3 id="teammateidle">
  TeammateIdle
</h3>

Viene eseguito quando un compagno di squadra di un [agent team](/docs/it/agent-teams) sta per andare inattivo dopo aver finito il suo turno. Utilizzare questo per applicare gate di qualità prima che un compagno di squadra smetta di lavorare, come richiedere il passaggio dei controlli di linting o verificare che i file di output esistano.

Quando un hook `TeammateIdle` esce con il codice 2, il compagno di squadra riceve il messaggio stderr come feedback e continua a lavorare invece di andare inattivo. Per interrompere completamente il compagno di squadra invece di rieseguirlo, restituire JSON con `{"continue": false, "stopReason": "..."}`. Gli hook TeammateIdle non supportano i matcher e si attivano ad ogni occorrenza.

<h4 id="teammateidle-input">
  Input di TeammateIdle
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook TeammateIdle ricevono `teammate_name` e `team_name`.

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

| Campo           | Descrizione                                                                 |
| :-------------- | :-------------------------------------------------------------------------- |
| `teammate_name` | Nome del compagno di squadra che sta per andare inattivo                    |
| `team_name`     | Nome del team derivato dalla sessione; verrà rimosso in una versione futura |

<h4 id="teammateidle-decision-control">
  Controllo della decisione di TeammateIdle
</h4>

Gli hook TeammateIdle supportano due modi per controllare il comportamento del compagno di squadra:

* **Codice di uscita 2**: il compagno di squadra riceve il messaggio stderr come feedback e continua a lavorare invece di andare inattivo.
* **JSON `{"continue": false, "stopReason": "..."}`**: interrompe completamente il compagno di squadra, corrispondendo al comportamento dell'hook `Stop`. Il `stopReason` viene mostrato all'utente.

Questo esempio controlla che un artefatto di build esista prima di consentire a un compagno di squadra di andare inattivo:

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

Viene eseguito quando un file di configurazione cambia durante una sessione. Utilizzare questo per controllare le modifiche alle impostazioni, applicare le politiche di sicurezza o bloccare le modifiche non autorizzate ai file di configurazione.

Gli hook ConfigChange si attivano per le modifiche ai file di impostazioni, alle impostazioni della politica gestita e ai file di skill. Il campo `source` nell'input dice quale tipo di configurazione è cambiato e il campo `file_path` facoltativo fornisce il percorso al file modificato.

Il matcher filtra sulla fonte di configurazione:

| Matcher            | Quando si attiva                                |
| :----------------- | :---------------------------------------------- |
| `user_settings`    | `~/.claude/settings.json` cambia                |
| `project_settings` | `.claude/settings.json` cambia                  |
| `local_settings`   | `.claude/settings.local.json` cambia            |
| `policy_settings`  | Le impostazioni della politica gestita cambiano |
| `skills`           | Un file di skill in `.claude/skills/` cambia    |

Questo esempio registra tutte le modifiche di configurazione per il controllo della sicurezza:

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
  Input di ConfigChange
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook ConfigChange ricevono `source` e facoltativamente `file_path`. Il campo `source` indica quale tipo di configurazione è cambiato e `file_path` fornisce il percorso al file specifico che è stato modificato.

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
  Controllo della decisione di ConfigChange
</h4>

Gli hook ConfigChange possono bloccare le modifiche di configurazione dall'avere effetto. Utilizzare il codice di uscita 2 o un JSON `decision` per impedire la modifica. Quando bloccato, le nuove impostazioni non vengono applicate alla sessione in esecuzione.

| Campo      | Descrizione                                                                                              |
| :--------- | :------------------------------------------------------------------------------------------------------- |
| `decision` | `"block"` impedisce l'applicazione della modifica di configurazione. Omettere per consentire la modifica |
| `reason`   | Spiegazione mostrata all'utente quando `decision` è `"block"`                                            |

```json theme={null}
{
  "decision": "block",
  "reason": "Configuration changes to project settings require admin approval"
}
```

Le modifiche a `policy_settings` non possono essere bloccate. Gli hook si attivano ancora per le fonti `policy_settings`, quindi è possibile utilizzarli per la registrazione di audit, ma qualsiasi decisione di blocco viene ignorata. Ciò garantisce che le impostazioni gestite dall'azienda abbiano sempre effetto.

<h3 id="cwdchanged">
  CwdChanged
</h3>

Viene eseguito quando la directory di lavoro cambia durante una sessione, ad esempio quando Claude esegue un comando `cd`. Utilizzare questo per reagire ai cambi di directory: ricaricare le variabili di ambiente, attivare toolchain specifiche del progetto o eseguire script di configurazione automaticamente. Si accoppia con [FileChanged](#filechanged) per strumenti come [direnv](https://direnv.net/) che gestiscono l'ambiente per directory.

Gli hook CwdChanged hanno accesso a `CLAUDE_ENV_FILE`. Le variabili scritte in quel file persistono nei comandi Bash successivi per la sessione, proprio come negli [hook SessionStart](#persist-environment-variables).

CwdChanged non supporta i matcher e si attiva ad ogni cambio di directory.

<h4 id="cwdchanged-input">
  Input di CwdChanged
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook CwdChanged ricevono `old_cwd` e `new_cwd`.

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
  Output di CwdChanged
</h4>

Oltre ai [campi di output JSON](#json-output) disponibili per tutti gli hook, gli hook CwdChanged possono restituire `watchPaths` per impostare dinamicamente quali percorsi di file [FileChanged](#filechanged) monitora:

| Campo        | Descrizione                                                                                                                                                                                                                                                            |
| :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `watchPaths` | Array di percorsi assoluti. Sostituisce l'elenco di monitoraggio dinamico corrente. I percorsi dalla configurazione del `matcher` vengono sempre monitorati. Restituire un array vuoto cancella l'elenco dinamico, che è tipico quando si entra in una nuova directory |

Gli hook CwdChanged non hanno controllo della decisione. Non possono bloccare il cambio di directory.

<h3 id="filechanged">
  FileChanged
</h3>

Viene eseguito quando un file monitorato cambia su disco. Utile per ricaricare le variabili di ambiente quando i file di configurazione del progetto vengono modificati.

Il `matcher` per questo evento serve due ruoli:

* **Costruire l'elenco di osservazione**: il valore viene diviso su `|` e ogni segmento viene registrato come nome di file letterale nella directory di lavoro, quindi `".envrc|.env"` monitora esattamente quei due file. I modelli regex non sono utili qui: un valore come `^\.env` monitorerebbe un file letteralmente denominato `^\.env`.
* **Filtrare quali hook vengono eseguiti**: quando un file monitorato cambia, lo stesso valore filtra quali gruppi di hook vengono eseguiti utilizzando le [regole di matcher](#matcher-patterns) standard rispetto al basename del file modificato.

Gli hook FileChanged hanno accesso a `CLAUDE_ENV_FILE`. Le variabili scritte in quel file persistono nei comandi Bash successivi per la sessione, proprio come negli [hook SessionStart](#persist-environment-variables).

<h4 id="filechanged-input">
  Input di FileChanged
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook FileChanged ricevono `file_path` e `event`.

| Campo       | Descrizione                                                                                                       |
| :---------- | :---------------------------------------------------------------------------------------------------------------- |
| `file_path` | Percorso assoluto al file che è cambiato                                                                          |
| `event`     | Cosa è accaduto: `"change"` per un file modificato, `"add"` per un file creato o `"unlink"` per un file eliminato |

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
  Output di FileChanged
</h4>

Oltre ai [campi di output JSON](#json-output) disponibili per tutti gli hook, gli hook FileChanged possono restituire `watchPaths` per aggiornare dinamicamente quali percorsi di file vengono monitorati:

| Campo        | Descrizione                                                                                                                                                                                                                                                              |
| :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `watchPaths` | Array di percorsi assoluti. Sostituisce l'elenco di monitoraggio dinamico corrente. I percorsi dalla configurazione del `matcher` vengono sempre monitorati. Utilizzare questo quando lo script del hook scopre file aggiuntivi da monitorare in base al file modificato |

Gli hook FileChanged non hanno controllo della decisione. Non possono bloccare il cambio di file dall'occorrenza.

<h3 id="worktreecreate">
  WorktreeCreate
</h3>

Viene eseguito quando un worktree sta per essere creato, sia da `claude --worktree` che da un [subagent che utilizza `isolation: "worktree"`](/docs/it/sub-agents#choose-the-subagent-scope). Per impostazione predefinita Claude Code crea la copia di lavoro isolata con `git worktree`. Configurando un hook WorktreeCreate si sostituisce quel comportamento git predefinito, consentendo di utilizzare un sistema di controllo della versione diverso come SVN, Perforce o Mercurial.

Poiché l'hook sostituisce completamente il comportamento predefinito, [`.worktreeinclude`](/docs/it/worktrees#copy-gitignored-files-into-worktrees) non viene elaborato. Se è necessario copiare i file di configurazione locali come `.env` nel nuovo worktree, farlo all'interno dello script del hook.

L'hook deve restituire il percorso assoluto della directory del worktree creato. Claude Code utilizza questo percorso come directory di lavoro per la sessione isolata. Consultare [WorktreeCreate output](#worktreecreate-output) per come ogni tipo di hook restituisce il percorso.

Questo esempio crea una copia di lavoro SVN e stampa il percorso per Claude Code da utilizzare. Sostituire l'URL del repository con il proprio:

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

L'hook legge il `name` del worktree dall'input JSON su stdin, controlla una copia fresca in una nuova directory e stampa il percorso della directory. L'`echo` sull'ultima riga è quello che Claude Code legge come percorso del worktree. Reindirizzare qualsiasi altro output a stderr in modo che non interferisca con il percorso.

<h4 id="worktreecreate-input">
  Input di WorktreeCreate
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook WorktreeCreate ricevono il campo `name`. Questo è un identificatore slug per il nuovo worktree, specificato dall'utente o generato automaticamente, ad esempio `bold-oak-a3f2`.

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
  Output di WorktreeCreate
</h4>

Gli hook WorktreeCreate non utilizzano il modello di decisione di blocco/consentimento standard. Invece, il successo o il fallimento dell'hook determina il risultato. L'hook deve restituire il percorso assoluto della directory del worktree creato:

* **Command hooks** (`type: "command"`): stampano il percorso come ultima riga non vuota di stdout. Claude Code rimuove i codici di escape ANSI prima di leggere quella riga, quindi i banner di avvio della shell stampati prima del vostro `echo` vengono ignorati. Reindirizzare qualsiasi altro output dell'hook a stderr.
* **HTTP hooks** (`type: "http"`): restituiscono `{ "hookSpecificOutput": { "hookEventName": "WorktreeCreate", "worktreePath": "/absolute/path" } }` nel corpo della risposta.

Se l'hook non riesce o non produce un percorso, la creazione del worktree non riesce con un errore.

Claude Code risolve un percorso relativo rispetto alla directory in cui l'hook è stato eseguito. Se il percorso risultante non è una directory in cui Claude Code può entrare, la sessione stampa un errore che nomina il percorso e esce con il codice 1. Prima di v2.1.205, un percorso relativo o un percorso che non esisteva su disco causava un crash della sessione all'avvio e con `-p` si bloccava per circa 30 secondi prima di uscire con il codice 0.

<h3 id="worktreeremove">
  WorktreeRemove
</h3>

Viene eseguito quando un worktree sta per essere rimosso, sia quando si esce da una sessione `--worktree` e si sceglie di rimuoverla, sia quando un subagent con `isolation: "worktree"` termina. Questa è la controparte di pulizia di [WorktreeCreate](#worktreecreate).

Per i worktree basati su git, Claude Code gestisce la pulizia automaticamente con `git worktree remove`. Se si è configurato un hook WorktreeCreate per un sistema di controllo della versione non-git, accoppiarlo con un hook WorktreeRemove per gestire la pulizia. Senza uno, la directory del worktree viene lasciata su disco.

Claude Code passa il percorso restituito da WorktreeCreate come `worktree_path` nell'input del hook. Questo esempio legge quel percorso e rimuove la directory:

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
  Input di WorktreeRemove
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook WorktreeRemove ricevono il campo `worktree_path`, che è il percorso assoluto al worktree in corso di rimozione.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "WorktreeRemove",
  "worktree_path": "/Users/.../my-project/.claude/worktrees/feature-auth"
}
```

Gli hook WorktreeRemove non hanno controllo della decisione. Non possono bloccare la rimozione del worktree ma possono eseguire attività di pulizia come la rimozione dello stato del controllo della versione o l'archiviazione delle modifiche. I guasti degli hook vengono registrati solo in modalità debug.

<h3 id="precompact">
  PreCompact
</h3>

Viene eseguito prima che Claude Code stia per eseguire un'operazione di compattazione.

Il valore del matcher indica se la compattazione è stata attivata manualmente o automaticamente:

| Matcher  | Quando si attiva                                                |
| :------- | :-------------------------------------------------------------- |
| `manual` | `/compact`                                                      |
| `auto`   | Compattazione automatica quando la finestra di contesto è piena |

Uscire con il codice 2 per bloccare la compattazione. Per un `/compact` manuale, il messaggio stderr viene mostrato all'utente. È anche possibile bloccare restituendo JSON con `"decision": "block"`.

Bloccare la compattazione automatica ha effetti diversi a seconda di quando si attiva. Se la compattazione è stata attivata in modo proattivo prima del limite di contesto, Claude Code la salta e la conversazione continua non compattata. Se la compattazione è stata attivata per recuperare da un errore di limite di contesto già restituito dall'API, l'errore sottostante emerge e la richiesta corrente non riesce.

<h4 id="precompact-input">
  Input di PreCompact
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook PreCompact ricevono `trigger` e `custom_instructions`. Per `manual`, `custom_instructions` contiene quello che l'utente passa in `/compact`. Per `auto`, `custom_instructions` è vuoto.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PreCompact",
  "trigger": "manual",
  "custom_instructions": ""
}
```

<h3 id="postcompact">
  PostCompact
</h3>

Viene eseguito dopo che Claude Code completa un'operazione di compattazione. Utilizzare questo evento per reagire al nuovo stato compattato, ad esempio per registrare il riepilogo generato o aggiornare lo stato esterno.

Gli stessi valori di matcher si applicano come per `PreCompact`:

| Matcher  | Quando si attiva                                                        |
| :------- | :---------------------------------------------------------------------- |
| `manual` | Dopo `/compact`                                                         |
| `auto`   | Dopo la compattazione automatica quando la finestra di contesto è piena |

<h4 id="postcompact-input">
  Input di PostCompact
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook PostCompact ricevono `trigger` e `compact_summary`. Il campo `compact_summary` contiene il riepilogo della conversazione generato dall'operazione di compattazione.

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

Gli hook PostCompact non hanno controllo della decisione. Non possono influenzare il risultato della compattazione ma possono eseguire attività di follow-up.

<h3 id="sessionend">
  SessionEnd
</h3>

Viene eseguito quando una sessione di Claude Code termina. Utile per le attività di pulizia, la registrazione delle statistiche della sessione o il salvataggio dello stato della sessione. Supporta i matcher per filtrare per motivo di uscita.

Il campo `reason` nell'input del hook indica perché la sessione è terminata:

| Motivo                        | Descrizione                                                     |
| :---------------------------- | :-------------------------------------------------------------- |
| `clear`                       | Sessione cancellata con il comando `/clear`                     |
| `resume`                      | Sessione commutata tramite `/resume` interattivo                |
| `logout`                      | L'utente ha effettuato il logout                                |
| `prompt_input_exit`           | L'utente è uscito mentre l'input del prompt era visibile        |
| `bypass_permissions_disabled` | La modalità di bypass delle autorizzazioni è stata disabilitata |
| `other`                       | Altri motivi di uscita                                          |

<h4 id="sessionend-input">
  Input di SessionEnd
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook SessionEnd ricevono un campo `reason` che indica perché la sessione è terminata. Consultare la tabella dei motivi sopra per tutti i valori.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionEnd",
  "reason": "other"
}
```

Gli hook SessionEnd non hanno controllo della decisione. Non possono bloccare la terminazione della sessione ma possono eseguire attività di pulizia.

Gli hook SessionEnd hanno un timeout predefinito di 1,5 secondi. Questo si applica all'uscita della sessione, `/clear` e al cambio di sessioni tramite `/resume` interattivo. Se un hook ha bisogno di più tempo, impostare un `timeout` per hook nella configurazione del hook. Il budget complessivo viene automaticamente aumentato al timeout per hook più alto configurato nei file di impostazioni, fino a 60 secondi. I timeout impostati sui hook forniti dal plugin non aumentano il budget. Per sovrascrivere il budget in modo esplicito, impostare la variabile di ambiente `CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS` in millisecondi.

```bash theme={null}
CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS=5000 claude
```

<h3 id="elicitation">
  Elicitation
</h3>

Viene eseguito quando un server MCP richiede l'input dell'utente a metà attività. Per impostazione predefinita, Claude Code mostra una finestra di dialogo interattiva per l'utente per rispondere. Gli hook possono intercettare questa richiesta e rispondere a livello di programmazione, saltando completamente la finestra di dialogo.

Il campo matcher corrisponde al nome del server MCP.

<h4 id="elicitation-input">
  Input di Elicitation
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook Elicitation ricevono `mcp_server_name`, `message` e campi facoltativi `mode`, `url`, `elicitation_id` e `requested_schema`.

Per l'elicitazione in modalità modulo (il caso più comune):

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
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

Per l'elicitazione in modalità URL (autenticazione basata su browser):

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "Elicitation",
  "mcp_server_name": "my-mcp-server",
  "message": "Please authenticate",
  "mode": "url",
  "url": "https://auth.example.com/login"
}
```

<h4 id="elicitation-output">
  Output di Elicitation
</h4>

Per rispondere a livello di programmazione senza mostrare la finestra di dialogo, restituire un oggetto JSON con `hookSpecificOutput`:

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

| Campo     | Valori                        | Descrizione                                                                        |
| :-------- | :---------------------------- | :--------------------------------------------------------------------------------- |
| `action`  | `accept`, `decline`, `cancel` | Se accettare, rifiutare o annullare la richiesta                                   |
| `content` | object                        | Valori dei campi del modulo da inviare. Utilizzato solo quando `action` è `accept` |

Il codice di uscita 2 nega l'elicitazione e mostra stderr all'utente.

<h3 id="elicitationresult">
  ElicitationResult
</h3>

Viene eseguito dopo che un utente risponde a un'elicitazione MCP. Gli hook possono osservare, modificare o bloccare la risposta prima che venga inviata al server MCP.

Il campo matcher corrisponde al nome del server MCP.

<h4 id="elicitationresult-input">
  Input di ElicitationResult
</h4>

Oltre ai [campi di input comuni](#common-input-fields), gli hook ElicitationResult ricevono `mcp_server_name`, `action` e campi facoltativi `mode`, `elicitation_id` e `content`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "ElicitationResult",
  "mcp_server_name": "my-mcp-server",
  "action": "accept",
  "content": { "username": "alice" },
  "mode": "form",
  "elicitation_id": "elicit-123"
}
```

<h4 id="elicitationresult-output">
  Output di ElicitationResult
</h4>

Per sovrascrivere la risposta dell'utente, restituire un oggetto JSON con `hookSpecificOutput`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "ElicitationResult",
    "action": "decline",
    "content": {}
  }
}
```

| Campo     | Valori                        | Descrizione                                                                              |
| :-------- | :---------------------------- | :--------------------------------------------------------------------------------------- |
| `action`  | `accept`, `decline`, `cancel` | Sovrascrive l'azione dell'utente                                                         |
| `content` | object                        | Sovrascrive i valori dei campi del modulo. Significativo solo quando `action` è `accept` |

Il codice di uscita 2 blocca la risposta, cambiando l'azione effettiva in `decline`.

<h2 id="prompt-based-hooks">
  Hook basati su prompt
</h2>

Oltre agli hook di comando, HTTP e MCP tool, Claude Code supporta gli hook basati su prompt (`type: "prompt"`) che utilizzano un LLM per valutare se consentire o bloccare un'azione, e gli hook basati su agenti (`type: "agent"`) che generano un verificatore agentico con accesso agli strumenti. Non tutti gli eventi supportano ogni tipo di hook.

Gli eventi che supportano tutti e cinque i tipi di hook (`command`, `http`, `mcp_tool`, `prompt` e `agent`):

* `PermissionDenied`
* `PermissionRequest`
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

Gli eventi che supportano gli hook `command`, `http` e `mcp_tool` ma non `prompt` o `agent`:

* `ConfigChange`
* `CwdChanged`
* `Elicitation`
* `ElicitationResult`
* `FileChanged`
* `InstructionsLoaded`
* `Notification`
* `PostCompact`
* `PreCompact`
* `SessionEnd`
* `StopFailure`
* `SubagentStart`
* `WorktreeCreate`
* `WorktreeRemove`

`SessionStart` e `Setup` supportano gli hook `command` e `mcp_tool`. Non supportano gli hook `http`, `prompt` o `agent`.

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

Impostare `type` su `"prompt"` e fornire una stringa `prompt` invece di un `command`. Utilizzare il segnaposto `$ARGUMENTS` per iniettare i dati di input JSON del hook nel testo del prompt. Claude Code invia il prompt combinato e l'input a un modello Claude veloce, che restituisce una decisione JSON.

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

| Campo             | Obbligatorio | Descrizione                                                                                                                                                                                                                                                                                            |
| :---------------- | :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`            | sì           | Deve essere `"prompt"`                                                                                                                                                                                                                                                                                 |
| `prompt`          | sì           | Il testo del prompt da inviare all'LLM. Utilizzare `$ARGUMENTS` come segnaposto per l'input JSON del hook. Se `$ARGUMENTS` non è presente, l'input JSON viene aggiunto al prompt                                                                                                                       |
| `model`           | no           | Modello da utilizzare per la valutazione. Impostazione predefinita: un modello veloce                                                                                                                                                                                                                  |
| `timeout`         | no           | Timeout in secondi. Impostazione predefinita: 30                                                                                                                                                                                                                                                       |
| `continueOnBlock` | no           | Quando il prompt restituisce `ok: false`, reinvia il motivo a Claude e continua il turno invece di fermarsi. Impostazione predefinita: `false`. Implementato come `continue: true` sulla `decision: "block"` risultante. Vedere [Schema di risposta](#response-schema) per il comportamento per evento |

<h3 id="response-schema">
  Schema di risposta
</h3>

L'LLM deve rispondere con JSON contenente:

```json theme={null}
{
  "ok": true | false,
  "reason": "Explanation for the decision"
}
```

| Campo    | Descrizione                                                                                                   |
| :------- | :------------------------------------------------------------------------------------------------------------ |
| `ok`     | `true` per consentire. `false` produce una `decision: "block"`. Vedere il comportamento per evento di seguito |
| `reason` | Obbligatorio quando `ok` è `false`. Utilizzato come motivo del blocco                                         |

Ciò che accade con `ok: false` dipende dall'evento:

* `Stop` e `SubagentStop`: il motivo viene reinviato a Claude come sua prossima istruzione e il turno continua
* `PreToolUse`: la chiamata dello strumento viene negata e il motivo viene restituito a Claude come errore dello strumento, equivalente a un hook di comando con `permissionDecision: "deny"`
* `PostToolUse`: per impostazione predefinita il turno termina e il motivo appare nella chat come una riga di avviso. Impostare `continueOnBlock: true` per reinviare il motivo a Claude e continuare il turno invece
* `PostToolBatch`, `UserPromptSubmit` e `UserPromptExpansion`: il turno termina e il motivo appare come una riga di avviso. Questi eventi terminano il turno su `decision: "block"` indipendentemente da `continue`
* `PostToolUseFailure`, `TaskCreated` e `TaskCompleted`: il motivo viene restituito a Claude come errore dello strumento, simile a `PreToolUse`
* `TeammateIdle`: per impostazione predefinita il compagno di squadra si ferma e il motivo appare come una riga di avviso. Impostare `continueOnBlock: true` per reinviare il motivo al compagno di squadra e mantenerlo al lavoro invece
* `PermissionRequest`: `ok: false` non ha effetto. Per negare un'approvazione da un hook, utilizzare un [hook di comando](#command-hook-fields) che restituisce `hookSpecificOutput.decision.behavior: "deny"`
* `PermissionDenied`: `ok: false` non ha effetto perché il rifiuto è già avvenuto. L'unico output che questo evento legge è `hookSpecificOutput.retry`, che gli hook di prompt e agenti non possono impostare. Vengono eseguiti su questo evento, ma il loro output viene scartato. Utilizzare un [hook di comando](#command-hook-fields) per restituire `retry`

Se hai bisogno di un controllo più fine su qualsiasi evento, utilizza un [hook di comando](#command-hook-fields) con i campi per evento descritti in [Controllo delle decisioni](#decision-control).

<h3 id="check-multiple-conditions-before-stopping">
  Controllare più condizioni prima di fermarsi
</h3>

Questo hook `Stop` utilizza un prompt dettagliato per controllare tre condizioni prima di consentire a Claude di fermarsi. Gli hook `SubagentStop` utilizzano lo stesso formato per valutare se un [subagent](/docs/it/sub-agents) dovrebbe fermarsi. Se `"ok"` è `false`, Claude continua a lavorare con il motivo fornito come sua prossima istruzione:

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

Gli hook basati su agenti (`type: "agent"`) sono come gli hook basati su prompt ma con accesso agli strumenti multi-turno. Invece di una singola chiamata LLM, un hook agente genera un subagent che può leggere file, cercare codice e ispezionare il codebase per verificare le condizioni. Gli hook agente supportano gli stessi eventi degli hook basati su prompt.

<h3 id="how-agent-hooks-work">
  Come funzionano gli hook basati su agenti
</h3>

Quando un hook agente si attiva:

1. Claude Code genera un subagent con il prompt e l'input JSON del hook
2. Il subagent può utilizzare strumenti come Read, Grep e Glob per investigare
3. Dopo fino a 50 turni, il subagent restituisce una decisione strutturata `{ "ok": true/false }`
4. Claude Code elabora la decisione nello stesso modo di un hook di prompt

Gli hook agente sono utili quando la verifica richiede l'ispezione dei file effettivi o dell'output dei test, non solo la valutazione dei dati di input del hook da soli.

<h3 id="agent-hook-configuration">
  Configurazione dell'hook agente
</h3>

Impostare `type` su `"agent"` e fornire una stringa `prompt`. I campi di configurazione sono gli stessi degli [hook di prompt](#prompt-hook-configuration), con un timeout predefinito più lungo:

| Campo     | Obbligatorio | Descrizione                                                                                            |
| :-------- | :----------- | :----------------------------------------------------------------------------------------------------- |
| `type`    | sì           | Deve essere `"agent"`                                                                                  |
| `prompt`  | sì           | Prompt che descrive cosa verificare. Utilizzare `$ARGUMENTS` come segnaposto per l'input JSON del hook |
| `model`   | no           | Modello da utilizzare. Impostazione predefinita: un modello veloce                                     |
| `timeout` | no           | Timeout in secondi. Impostazione predefinita: 60                                                       |

Lo schema di risposta è lo stesso degli hook di prompt: `{ "ok": true }` per consentire o `{ "ok": false, "reason": "..." }` per bloccare.

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

Questo hook esegue uno script di test dopo ogni chiamata dello strumento `Write`. Claude continua a lavorare immediatamente mentre `run-tests.sh` viene eseguito per un massimo di 120 secondi. Quando lo script termina, l'output viene consegnato al turno di conversazione successivo:

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
            "async": true,
            "timeout": 120
          }
        ]
      }
    ]
  }
}
```

Il campo `timeout` imposta il tempo massimo in secondi per il processo in background. Se non specificato, gli hook asincroni utilizzano lo stesso timeout predefinito di 10 minuti degli hook sincroni.

<h3 id="how-async-hooks-execute">
  Come vengono eseguiti gli hook asincroni
</h3>

Quando un hook asincrono si attiva, Claude Code avvia il processo del hook e continua immediatamente senza aspettare il completamento. L'hook riceve lo stesso input JSON tramite stdin di un hook sincrono.

Dopo che il processo in background esce, se l'hook ha prodotto una risposta JSON con un campo `additionalContext`, quel contenuto viene consegnato a Claude come contesto al turno di conversazione successivo. Un campo `systemMessage` viene mostrato a voi, non a Claude.

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
            "async": true,
            "timeout": 300
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

Gli hook asincroni hanno diversi vincoli rispetto agli hook sincroni:

* Solo gli hook `type: "command"` supportano `async`. Gli hook basati su prompt non possono essere eseguiti in modo asincrono.
* Gli hook asincroni non possono bloccare le chiamate dello strumento o restituire decisioni. Nel momento in cui l'hook si completa, l'azione che lo ha attivato è già stata eseguita.
* L'output del hook viene consegnato al turno di conversazione successivo. Se la sessione è inattiva, la risposta attende fino alla prossima interazione dell'utente. Eccezione: un hook `asyncRewake` che esce con il codice 2 riattiva Claude immediatamente anche quando la sessione è inattiva.
* Ogni esecuzione crea un processo in background separato. Non c'è deduplicazione tra più attivazioni dello stesso hook asincrono.

<h2 id="security-considerations">
  Considerazioni sulla sicurezza
</h2>

<h3 id="disclaimer">
  Disclaimer
</h3>

I command hook vengono eseguiti con i permessi completi dell'utente del sistema.

<Warning>
  I command hook eseguono comandi shell con i permessi completi dell'utente. Possono modificare, eliminare o accedere a qualsiasi file a cui l'account utente può accedere. Rivedere e testare tutti i comandi del hook prima di aggiungerli alla configurazione.
</Warning>

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

Su Windows, è possibile eseguire singoli hook in PowerShell impostando `"shell": "powershell"` su un command hook. Gli hook generano PowerShell direttamente, quindi questo funziona indipendentemente dal fatto che `CLAUDE_CODE_USE_POWERSHELL_TOOL` sia impostato. Claude Code rileva automaticamente `pwsh.exe`, l'eseguibile di PowerShell 7 e versioni successive, e ricade su `powershell.exe` per Windows PowerShell 5.1.

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

I dettagli dell'esecuzione dei hook, inclusi quali hook corrispondono, i loro codici di uscita e l'output completo di stdout e stderr, vengono scritti nel file di log di debug. Avviare Claude Code con `claude --debug-file <path>` per scrivere il log in una posizione nota, oppure eseguire `claude --debug` e leggere il log in `~/.claude/debug/<session-id>.txt`. Il flag `--debug` non stampa nel terminale.

```text theme={null}
[DEBUG] Executing hooks for PostToolUse:Write
[DEBUG] Found 1 hook commands to execute
[DEBUG] Executing hook command: <Your command> with timeout 600000ms
[DEBUG] Hook command completed with status 0: <Your stdout>
```

Per dettagli di corrispondenza dei hook più granulari, impostare `CLAUDE_CODE_DEBUG_LOG_LEVEL=verbose` per visualizzare righe di log aggiuntive come i conteggi dei matcher del hook e la corrispondenza delle query.

Per la risoluzione dei problemi comuni come i hook che non si attivano, i cicli infiniti di Stop hook o gli errori di configurazione, consultare [Limitations and troubleshooting](/docs/it/hooks-guide#limitations-and-troubleshooting) nella guida. Per una procedura diagnostica più ampia che copre `/context`, `/doctor` e la precedenza delle impostazioni, consultare [Debug your config](/docs/it/debug-your-config).
