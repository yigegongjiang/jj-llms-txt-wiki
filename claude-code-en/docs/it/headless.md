> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Eseguire Claude Code a livello programmatico

> Utilizza l'Agent SDK per eseguire Claude Code a livello programmatico dalla CLI, Python o TypeScript.

L'[Agent SDK](/docs/it/agent-sdk/overview) ti fornisce gli stessi strumenti, il ciclo dell'agente e la gestione del contesto che alimentano Claude Code. È disponibile come CLI per script e CI/CD, oppure come pacchetti [Python](/docs/it/agent-sdk/python) e [TypeScript](/docs/it/agent-sdk/typescript) per il controllo programmatico completo.

Per eseguire Claude Code in modalità non interattiva, passa `-p` con il tuo prompt e qualsiasi [opzione CLI](/docs/it/cli-reference):

```bash theme={null}
claude -p "Find and fix the bug in auth.py" --allowedTools "Read,Edit,Bash"
```

Questa pagina copre l'utilizzo dell'Agent SDK tramite la CLI (`claude -p`). Per i pacchetti SDK Python e TypeScript con output strutturati, callback di approvazione degli strumenti e oggetti messaggio nativi, consulta la [documentazione completa dell'Agent SDK](/docs/it/agent-sdk/overview).

<h2 id="basic-usage">
  Utilizzo di base
</h2>

Aggiungi il flag `-p` (o `--print`) a qualsiasi comando `claude` per eseguirlo in modo non interattivo. Tutte le [opzioni CLI](/docs/it/cli-reference) funzionano con `-p`, incluse:

* `--continue` per [continuare le conversazioni](#continue-conversations)
* `--allowedTools` per [approvare automaticamente gli strumenti](#auto-approve-tools)
* `--output-format` per [ottenere output strutturato](#get-structured-output)

Questo esempio chiede a Claude una domanda sulla tua base di codice e stampa la risposta:

```bash theme={null}
claude -p "What does the auth module do?"
```

<h3 id="start-faster-with-bare-mode">
  Inizia più velocemente con la modalità bare
</h3>

Aggiungi `--bare` per ridurre il tempo di avvio saltando l'auto-discovery di hooks, skills, plugins, server MCP, memoria automatica e CLAUDE.md. Senza di esso, `claude -p` carica lo stesso [contesto](/docs/it/how-claude-code-works#the-context-window) che una sessione interattiva avrebbe, incluso tutto ciò che è configurato nella directory di lavoro o in `~/.claude`.

La modalità bare è utile per CI e script dove hai bisogno dello stesso risultato su ogni macchina. Un hook nel `~/.claude` di un collega o un server MCP nel `.mcp.json` del progetto non verranno eseguiti, perché la modalità bare non li legge mai. Solo i flag che passi esplicitamente hanno effetto.

Questo esempio esegue un'attività di riepilogo una tantum in modalità bare e pre-approva lo strumento Read in modo che la chiamata si completi senza un prompt di autorizzazione:

```bash theme={null}
claude --bare -p "Summarize this file" --allowedTools "Read"
```

In modalità bare Claude ha accesso agli strumenti Bash, lettura file e modifica file. Passa qualsiasi contesto di cui hai bisogno con un flag:

| Per caricare                  | Utilizza                                                |
| ----------------------------- | ------------------------------------------------------- |
| Aggiunte al prompt di sistema | `--append-system-prompt`, `--append-system-prompt-file` |
| Impostazioni                  | `--settings <file-or-json>`                             |
| Server MCP                    | `--mcp-config <file-or-json>`                           |
| Agenti personalizzati         | `--agents <json>`                                       |
| Un plugin                     | `--plugin-dir <path>`, `--plugin-url <url>`             |

La modalità bare salta le letture OAuth e keychain. L'autenticazione Anthropic deve provenire da `ANTHROPIC_API_KEY` o da un `apiKeyHelper` nel JSON passato a `--settings`. Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry utilizzano le loro credenziali provider usuali.

<Note>
  `--bare` è la modalità consigliata per le chiamate con script e SDK, e diventerà l'impostazione predefinita per `-p` in una versione futura.
</Note>

<h3 id="background-tasks-at-exit">
  Attività in background all'uscita
</h3>

Se Claude avvia un'[attività Bash in background](/docs/it/tools-reference#bash-tool-behavior) durante un'esecuzione di `claude -p`, ad esempio un server di sviluppo o una build di watch, tale attività viene terminata circa cinque secondi dopo che Claude ha restituito il suo risultato finale e stdin è stato chiuso. Il periodo di grazia consente a un'attività che termina subito dopo il risultato di consegnare comunque il suo output. Prima della v2.1.163, un processo in background che non termina mai avrebbe mantenuto l'invocazione di `claude -p` aperta indefinitamente.

I [subagenti](/docs/it/sub-agents) in background e i flussi di lavoro sono esenti dal periodo di grazia di cinque secondi perché il loro risultato fa parte dell'output finale, quindi `claude -p` attende il loro completamento. A partire dalla v2.1.182, tale attesa è limitata a dieci minuti per impostazione predefinita in modo che un agente in background bloccato non possa mantenere il processo aperto indefinitamente. Regola il limite con [`CLAUDE_CODE_PRINT_BG_WAIT_CEILING_MS`](/docs/it/env-vars), oppure impostalo su `0` per attendere senza limite.

<h2 id="examples">
  Esempi
</h2>

Questi esempi evidenziano i modelli CLI comuni. Per CI e altre chiamate con script, aggiungi [`--bare`](#start-faster-with-bare-mode) in modo che non raccolgano qualsiasi cosa sia configurata localmente.

<h3 id="pipe-data-through-claude">
  Inviare dati attraverso Claude
</h3>

La modalità non interattiva legge stdin, quindi puoi inviare dati e reindirizzare la risposta come qualsiasi altro strumento da riga di comando.

Questo esempio invia un log di compilazione a Claude e scrive la spiegazione in un file:

```bash theme={null}
cat build-error.txt | claude -p 'concisely explain the root cause of this build error' > output.txt
```

Con `--output-format json`, il payload della risposta include `total_cost_usd` e una suddivisione dei costi per modello, quindi i chiamanti con script possono tracciare la spesa per invocazione senza consultare il [dashboard di utilizzo](/docs/it/costs).

<Note>
  A partire da Claude Code v2.1.128, stdin inviato tramite pipe è limitato a 10MB. Se superi il limite, Claude Code esce con un errore chiaro e uno stato diverso da zero. Per lavorare con input più grandi, scrivi il contenuto in un file e fai riferimento al percorso del file nel tuo prompt invece di inviarlo tramite pipe.
</Note>

<h3 id="add-claude-to-a-build-script">
  Aggiungere Claude a uno script di compilazione
</h3>

Puoi avvolgere una chiamata non interattiva in uno script per utilizzare Claude come linter o revisore specifico del progetto.

Questo script `package.json` invia il diff rispetto a `main` a Claude e gli chiede di segnalare i refusi. Inviare il diff tramite pipe significa che Claude non ha bisogno del permesso Bash per leggerlo, e le virgolette doppie sfuggite mantengono lo script portabile su Windows:

```json theme={null}
{
  "scripts": {
    "lint:claude": "git diff main | claude -p \"you are a typo linter. for each typo in this diff, report filename:line on one line and the issue on the next. return nothing else.\""
  }
}
```

<h3 id="get-structured-output">
  Ottenere output strutturato
</h3>

Utilizza `--output-format` per controllare come vengono restituite le risposte:

* `text` (predefinito): output di testo semplice
* `json`: JSON strutturato con risultato, ID sessione e metadati
* `stream-json`: JSON delimitato da newline per lo streaming in tempo reale

Questo esempio restituisce un riepilogo del progetto come JSON con metadati della sessione, con il risultato del testo nel campo `result`:

```bash theme={null}
claude -p "Summarize this project" --output-format json
```

Per ottenere output conforme a uno schema specifico, utilizza `--output-format json` con `--json-schema` e una definizione [JSON Schema](https://json-schema.org/). La risposta include metadati sulla richiesta (ID sessione, utilizzo, ecc.) con l'output strutturato nel campo `structured_output`.

Questo esempio estrae i nomi delle funzioni e li restituisce come array di stringhe:

```bash theme={null}
claude -p "Extract the main function names from auth.py" \
  --output-format json \
  --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}'
```

Se il valore non è un JSON Schema valido, `claude` esce con `Error: --json-schema is not a valid JSON Schema` seguito dalla diagnostica del validatore. Claude Code accetta schemi che utilizzano la parola chiave `format`, come `"format": "email"`, ma tratta `format` come un'annotazione e non la applica. Prima della v2.1.205, Claude Code ignorava silenziosamente uno schema non valido e restituiva testo non strutturato, e trattava qualsiasi schema contenente `format` come non valido.

<Tip>
  Utilizza uno strumento come [jq](https://jqlang.github.io/jq/) per analizzare la risposta ed estrarre campi specifici:

  ```bash theme={null}
  # Extract the text result
  claude -p "Summarize this project" --output-format json | jq -r '.result'

  # Extract structured output
  claude -p "Extract function names from auth.py" \
    --output-format json \
    --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}' \
    | jq '.structured_output'
  ```
</Tip>

<h3 id="stream-responses">
  Streaming delle risposte
</h3>

Utilizza `--output-format stream-json` con `--verbose` e `--include-partial-messages` per ricevere i token mentre vengono generati. Ogni riga è un oggetto JSON che rappresenta un evento:

```bash theme={null}
claude -p "Explain recursion" --output-format stream-json --verbose --include-partial-messages
```

L'ultima riga del flusso è un messaggio `result` con il testo della risposta finale, il costo e i metadati della sessione. Prima della v2.1.208, l'invio tramite pipe di una risposta di grandi dimensioni potrebbe troncare l'ultima riga e omettere il messaggio `result`.

L'esempio seguente utilizza [jq](https://jqlang.github.io/jq/) per filtrare i delta di testo e visualizzare solo il testo in streaming. Il flag `-r` restituisce stringhe non elaborate (senza virgolette) e `-j` si unisce senza newline in modo che i token fluiscano continuamente:

```bash theme={null}
claude -p "Write a poem" --output-format stream-json --verbose --include-partial-messages | \
  jq -rj 'select(.type == "stream_event" and .event.delta.type? == "text_delta") | .event.delta.text'
```

Quando una richiesta API non riesce con un errore ritentabile, Claude Code emette un evento `system/api_retry` prima di ritentare. Puoi utilizzarlo per visualizzare il progresso del tentativo o implementare una logica di backoff personalizzata.

| Campo            | Tipo           | Descrizione                                                                                                                                                                                                |
| ---------------- | -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`           | `"system"`     | tipo di messaggio                                                                                                                                                                                          |
| `subtype`        | `"api_retry"`  | identifica questo come un evento di tentativo                                                                                                                                                              |
| `attempt`        | integer        | numero del tentativo corrente, a partire da 1                                                                                                                                                              |
| `max_retries`    | integer        | tentativi totali consentiti                                                                                                                                                                                |
| `retry_delay_ms` | integer        | millisecondi fino al prossimo tentativo                                                                                                                                                                    |
| `error_status`   | integer o null | codice di stato HTTP, o `null` per errori di connessione senza risposta HTTP                                                                                                                               |
| `error`          | string         | categoria di errore: `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `rate_limit`, `overloaded`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, o `unknown` |
| `uuid`           | string         | identificatore evento univoco                                                                                                                                                                              |
| `session_id`     | string         | sessione a cui appartiene l'evento                                                                                                                                                                         |

L'evento `system/init` segnala i metadati della sessione inclusi il modello, gli strumenti, i server MCP e i plugin caricati. È il primo evento nel flusso a meno che gli eventi di avvio lo precedano:

* Eventi `plugin_install`, quando [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/it/env-vars) è impostato.
* [`hook_started`, `hook_progress` e `hook_response` eventi](/docs/it/agent-sdk/typescript#sdkhookstartedmessage), mentre un hook [`SessionStart`](/docs/it/hooks#sessionstart) o [`Setup`](/docs/it/hooks#setup) configurato viene eseguito. Questi vengono trasmessi mentre l'hook li produce. Claude Code v2.1.169 attraverso v2.1.203 li ha consegnati in un batch dopo il completamento dell'hook, ancora prima di `system/init`; v2.1.204 ha ripristinato la consegna in tempo reale.

L'evento contiene anche un array `capabilities` opzionale di stringhe che denominano i comportamenti del protocollo che questa versione di Claude Code implementa, come `interrupt_receipt_v1`. Controllalo per rilevare le funzionalità invece di confrontare le stringhe di versione, e ignora i valori che non riconosci. Il campo richiede Claude Code v2.1.205 o successivo ed è assente dalle versioni precedenti. Consulta [`SDKSystemMessage`](/docs/it/agent-sdk/typescript#sdksystemmessage) per l'elenco delle capacità.

Utilizza i campi plugin per far fallire CI quando un plugin non è stato caricato:

| Campo           | Tipo  | Descrizione                                                                                                                                                                                                                                                                                                                              |
| --------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plugins`       | array | plugin che sono stati caricati con successo, ognuno con `name` e `path`                                                                                                                                                                                                                                                                  |
| `plugin_errors` | array | errori di caricamento del plugin, ognuno con `plugin`, `type` e `message`. Include versioni di dipendenza non soddisfatte e errori di caricamento di `--plugin-dir` come un percorso mancante o un archivio non valido. I plugin interessati vengono declassati e assenti da `plugins`. La chiave viene omessa quando non ci sono errori |

Quando [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/it/env-vars) è impostato, Claude Code emette eventi `system/plugin_install` mentre i plugin del marketplace si installano prima del primo turno. Utilizza questi per visualizzare il progresso dell'installazione nella tua interfaccia utente.

| Campo        | Tipo                                                    | Descrizione                                                                                                             |
| ------------ | ------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| `type`       | `"system"`                                              | tipo di messaggio                                                                                                       |
| `subtype`    | `"plugin_install"`                                      | identifica questo come un evento di installazione plugin                                                                |
| `status`     | `"started"`, `"installed"`, `"failed"`, o `"completed"` | `started` e `completed` racchiudono l'installazione complessiva; `installed` e `failed` segnalano i singoli marketplace |
| `name`       | string, opzionale                                       | nome del marketplace, presente su `installed` e `failed`                                                                |
| `error`      | string, opzionale                                       | messaggio di errore, presente su `failed`                                                                               |
| `uuid`       | string                                                  | identificatore evento univoco                                                                                           |
| `session_id` | string                                                  | sessione a cui appartiene l'evento                                                                                      |

Per lo streaming programmatico con callback e oggetti messaggio, consulta [Stream responses in real-time](/docs/it/agent-sdk/streaming-output) nella documentazione dell'Agent SDK.

<h3 id="auto-approve-tools">
  Approvare automaticamente gli strumenti
</h3>

Utilizza `--allowedTools` per consentire a Claude di utilizzare determinati strumenti senza chiedere. Questo esempio esegue una suite di test e corregge i guasti, consentendo a Claude di eseguire comandi Bash e leggere/modificare file senza chiedere il permesso:

```bash theme={null}
claude -p "Run the test suite and fix any failures" \
  --allowedTools "Bash,Read,Edit"
```

Per impostare una linea di base per l'intera sessione invece di elencare i singoli strumenti, passa una [modalità di autorizzazione](/docs/it/permission-modes). `dontAsk` nega qualsiasi cosa non sia nelle tue regole `permissions.allow` o nel [set di comandi di sola lettura](/docs/it/permissions#read-only-commands), utile per esecuzioni CI bloccate. `AskUserQuestion`, strumenti connettore [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools), e strumenti MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool) vengono negati anche quando una regola di autorizzazione corrisponde.

`acceptEdits` consente a Claude di scrivere file senza chiedere e approva automaticamente anche i comandi del filesystem comuni come `mkdir`, `touch`, `mv` e `cp`. Gli altri comandi shell e le richieste di rete hanno ancora bisogno di una voce `--allowedTools` o di una regola `permissions.allow`, altrimenti l'esecuzione si interrompe quando uno viene tentato:

```bash theme={null}
claude -p "Apply the lint fixes" --permission-mode acceptEdits
```

<h3 id="create-a-commit">
  Creare un commit
</h3>

Questo esempio esamina le modifiche in staging e crea un commit con un messaggio appropriato:

```bash theme={null}
claude -p "Look at my staged changes and create an appropriate commit" \
  --allowedTools "Bash(git diff *),Bash(git log *),Bash(git status *),Bash(git commit *)"
```

Il flag `--allowedTools` utilizza la [sintassi delle regole di autorizzazione](/docs/it/settings#permission-rule-syntax). Lo spazio finale ` *` abilita la corrispondenza dei prefissi, quindi `Bash(git diff *)` consente qualsiasi comando che inizia con `git diff`. Lo spazio prima di `*` è importante: senza di esso, `Bash(git diff*)` corrisponderebbe anche a `git diff-index`.

<Note>
  Le skills richiamate dall'utente e i comandi personalizzati funzionano in modalità `-p`: includi `/skill-name` nella stringa del prompt e Claude Code lo espande prima di eseguire. I comandi incorporati che solo eseguono nell'interfaccia del terminale, come `/login`, non sono disponibili in modalità `-p`. `/model`, `/effort`, `/fast`, `/color` e `/rename` accettano il valore come argomento, ad esempio `/model sonnet`, e `/mcp` senza argomento stampa un riepilogo di testo dello stato del server; questi moduli richiedono Claude Code v2.1.205 o successivo e seguono le [note di disponibilità](/docs/it/commands#all-commands) di ogni comando. Per modificare un'impostazione da un'invocazione `-p`, passa `key=value` a `/config`, ad esempio `/config thinking=false`.
</Note>

<h3 id="customize-the-system-prompt">
  Personalizzare il prompt di sistema
</h3>

Utilizza `--append-system-prompt` per aggiungere istruzioni mantenendo il comportamento predefinito di Claude Code. Questo esempio invia un diff PR a Claude e gli istruisce di esaminarlo per vulnerabilità di sicurezza:

```bash theme={null}
gh pr diff "$1" | claude -p \
  --append-system-prompt "You are a security engineer. Review for vulnerabilities." \
  --output-format json
```

Consulta i [flag del prompt di sistema](/docs/it/cli-reference#system-prompt-flags) per ulteriori opzioni incluso `--system-prompt` per sostituire completamente il prompt predefinito.

<h3 id="continue-conversations">
  Continuare le conversazioni
</h3>

Utilizza `--continue` per continuare la conversazione più recente, oppure `--resume` con un ID sessione per continuare una conversazione specifica. Questo esempio esegue una revisione, quindi invia prompt di follow-up:

```bash theme={null}
# First request
claude -p "Review this codebase for performance issues"

# Continue the most recent conversation
claude -p "Now focus on the database queries" --continue
claude -p "Generate a summary of all issues found" --continue
```

Se stai eseguendo più conversazioni, acquisisci l'ID sessione per riprendere una specifica:

```bash theme={null}
session_id=$(claude -p "Start a review" --output-format json | jq -r '.session_id')
claude -p "Continue that review" --resume "$session_id"
```

Esegui entrambi i comandi dalla stessa directory: la ricerca dell'ID sessione è limitata alla directory del progetto corrente e ai suoi git worktrees. Consulta [Resume a session](/docs/it/sessions#resume-a-session) per le regole di ambito complete.

<h2 id="next-steps">
  Passaggi successivi
</h2>

* [Agent SDK quickstart](/docs/it/agent-sdk/quickstart): costruisci il tuo primo agente con Python o TypeScript
* [CLI reference](/docs/it/cli-reference): tutti i flag e le opzioni CLI
* [GitHub Actions](/docs/it/github-actions): utilizza l'Agent SDK nei flussi di lavoro GitHub
* [GitLab CI/CD](/docs/it/gitlab-ci-cd): utilizza l'Agent SDK nelle pipeline GitLab
