> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Eseguire Claude Code a livello programmatico

> Utilizza l'Agent SDK per eseguire Claude Code a livello programmatico dalla CLI, Python o TypeScript.

L'[Agent SDK](/docs/it/agent-sdk/overview) ti fornisce gli stessi strumenti, il ciclo dell'agente e la gestione del contesto che alimentano Claude Code. È disponibile come CLI per script e CI/CD, oppure come pacchetti [Python](/docs/it/agent-sdk/python) e [TypeScript](/docs/it/agent-sdk/typescript) per il controllo programmatico completo.

Per eseguire Claude Code in modalità non interattiva, passa `-p` con il tuo prompt e le [opzioni CLI](/docs/it/cli-reference) di cui hai bisogno:

```bash theme={null}
claude -p "Find and fix the bug in auth.py" --allowedTools "Read,Edit,Bash"
```

Questa pagina copre l'utilizzo dell'Agent SDK tramite la CLI (`claude -p`). Per i pacchetti SDK Python e TypeScript con output strutturati, callback di approvazione degli strumenti e oggetti messaggio nativi, consulta la [documentazione completa dell'Agent SDK](/docs/it/agent-sdk/overview).

<h2 id="basic-usage">
  Utilizzo di base
</h2>

Aggiungi il flag `-p` (o `--print`) a qualsiasi comando `claude` per eseguirlo in modo non interattivo. Non tutte le [opzioni CLI](/docs/it/cli-reference) si combinano con `-p`. Claude Code rifiuta `--bg` e rifiuta `--cloud` con una descrizione di attività, con un errore che nomina il conflitto; `--cloud` con un ID di sessione e `-p` invece [accoda un messaggio in quella sessione cloud](/docs/it/claude-code-on-the-web#send-follow-ups-from-the-cli) ed esce. Le opzioni che combinerai con `-p` spesso includono:

* `--continue` per [continuare le conversazioni](#continue-conversations)
* `--allowedTools` per [approvare automaticamente gli strumenti](#auto-approve-tools)
* `--output-format` per [ottenere output strutturato](#get-structured-output)

Questo esempio chiede a Claude una domanda sulla tua base di codice e stampa la risposta:

```bash theme={null}
claude -p "What does the auth module do?"
```

Claude Code esce con codice 0 in caso di successo e con un codice diverso da zero quando l'esecuzione fallisce, quindi i tuoi script possono ramificarsi in base allo stato di uscita. Se passi un flag non valido, Claude Code segnala l'errore a stderr prima dell'inizio dell'esecuzione. Quando un errore si verifica durante l'esecuzione, come l'autenticazione mancante, Claude Code stampa l'errore come risultato su stdout.

<h3 id="start-faster-with-bare-mode">
  Inizia più velocemente con la modalità bare
</h3>

Aggiungi `--bare` per ridurre il tempo di avvio saltando l'auto-discovery di hooks, skills, comandi personalizzati, [subagenti](/docs/it/sub-agents), plugin installati, server MCP, memoria automatica e CLAUDE.md. Senza di esso, `claude -p` carica lo stesso [contesto](/docs/it/how-claude-code-works#the-context-window) che una sessione interattiva avrebbe, incluso tutto ciò che è configurato nella directory di lavoro o in `~/.claude`.

La modalità bare è utile per CI e script dove hai bisogno dello stesso risultato su ogni macchina. Un hook nel `~/.claude` di un collega o un server MCP nel `.mcp.json` del progetto non verranno eseguiti, perché la modalità bare non li legge mai. Una directory che nomini con `--add-dir` è un'eccezione parziale: la modalità bare carica skills dalla sua cartella `.claude/skills/`, ma salta comunque le sue cartelle `.claude/commands/` e `.claude/agents/`. [Skills da directory aggiuntive](/docs/it/skills#skills-from-additional-directories) copre ciò che viene e non viene caricato.

Senza `--bare`, una sessione `-p` esegue gli hook nel `.claude/settings.json` di un progetto e connette i server nel suo `.mcp.json`, anche in una cartella che non hai mai considerato attendibile. Una sessione `-p` non mostra alcuna finestra di dialogo di fiducia dell'area di lavoro e nessun prompt di approvazione per server. [Ciò che viene eseguito prima di considerare attendibile una cartella](/docs/it/permissions#what-runs-before-you-trust-a-folder) copre ogni tipo di contenuto del repository in `-p` e come mantenerlo fuori.

Questo esempio esegue un'attività di riepilogo una tantum in modalità bare e pre-approva lo strumento Read in modo che la chiamata si completi senza un prompt di autorizzazione. Imposta `ANTHROPIC_API_KEY` prima di eseguirlo, perché la modalità bare non utilizza il tuo accesso in abbonamento:

```bash theme={null}
claude --bare -p "Summarize README.md" --allowedTools "Read"
```

In modalità bare, Claude Code non legge mai le credenziali OAuth o il keychain di sistema. Per l'API Anthropic, imposta `ANTHROPIC_API_KEY` nell'ambiente, con una chiave creata nella [Claude Console](https://platform.claude.com), oppure fornisci un `apiKeyHelper` nel JSON `--settings`. Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry continuano a leggere le loro credenziali provider come al solito.

In modalità bare Claude ha accesso agli strumenti Bash, lettura file e modifica file. Passa qualsiasi contesto di cui hai bisogno con un flag:

| Per caricare                  | Utilizza                                                |
| ----------------------------- | ------------------------------------------------------- |
| Aggiunte al prompt di sistema | `--append-system-prompt`, `--append-system-prompt-file` |
| Impostazioni                  | `--settings <file-or-json>`                             |
| Server MCP                    | `--mcp-config <file-or-json>`                           |
| Agenti personalizzati         | `--agents <json>`                                       |
| Un plugin                     | `--plugin-dir <path>`, `--plugin-url <url>`             |

<Note>
  `--bare` è la modalità consigliata per le chiamate con script e SDK, e diventerà l'impostazione predefinita per `-p` in una versione futura.
</Note>

<h3 id="background-tasks-at-exit">
  Attività in background all'uscita
</h3>

Se Claude avvia un'[attività Bash in background](/docs/it/tools-reference#bash-tool-behavior) durante un'esecuzione di `claude -p`, ad esempio un server di sviluppo o una build di watch, tale shell viene terminata circa cinque secondi dopo che Claude ha restituito il suo risultato finale e stdin è stato chiuso. Il periodo di grazia consente a un'attività che termina subito dopo il risultato di consegnare comunque il suo output.

Se Claude avvia un [subagente](/docs/it/sub-agents) in background o un flusso di lavoro, `claude -p` rimane invece aperto fino al completamento di quel lavoro, perché il suo risultato fa parte dell'output finale.

Per impostazione predefinita l'attesa termina dopo 10 minuti di attesa continua inattiva, quindi un subagente o un flusso di lavoro bloccato non può mantenere il processo aperto indefinitamente. A quel punto Claude Code interrompe tutto ciò che è ancora in esecuzione e scarta il suo risultato parziale. Per modificare il limite, imposta [`CLAUDE_CODE_PRINT_BG_WAIT_CEILING_MS`](/docs/it/env-vars), oppure impostalo su `0` per attendere senza uno.

Se Claude avvia un watch [Monitor](/docs/it/tools-reference#monitor-tool) durante un'esecuzione di `claude -p`, Claude Code attende il watch fino a quando non scade o il limite di dieci minuti termina l'attesa, a seconda di quale arriva prima. Mentre attende, Claude continua a rispondere a ciò che il watch segnala. Per impostazione predefinita, un watch scade cinque minuti dopo che Claude lo avvia.

<h3 id="stop-a-run-with-sigterm">
  Interrompi un'esecuzione con SIGTERM
</h3>

Se interrompi un'esecuzione di `claude -p` con SIGTERM, ad esempio con `kill` o da un supervisore di processo, Claude Code esce con codice 143. Claude Code lascia il turno che era in corso non completato e non registra alcun risultato per esso. Per terminare il turno invece, invia SIGINT, o chiama `interrupt()` dell'Agent SDK, prima di interrompere il processo.

Su SIGTERM, Claude Code termina l'albero dei processi di qualsiasi comando Bash ancora in esecuzione. Claude Code quindi esegue gli hook [`SessionEnd`](/docs/it/hooks#sessionend) ed esce. Durante l'uscita, Claude Code non avvia alcuna nuova chiamata di strumento, non invia alcuna nuova richiesta di modello e non esegue alcun hook diverso da `SessionEnd`. Se l'esecuzione era nel mezzo di un comando o in attesa di una risposta a un prompt di autorizzazione quando il segnale è arrivato, Claude Code gestisce quel passaggio come segue:

* **Esecuzione di un comando**: Claude Code registra il comando come terminato nella sessione.
* **In attesa di una risposta a un prompt di autorizzazione**: se invii SIGTERM al processo, Claude Code lascia il prompt senza risposta. Se il tuo programma chiude la sessione tramite l'Agent SDK, l'SDK termina l'input di Claude Code prima di inviare qualsiasi segnale, e Claude Code annulla il prompt non appena l'input termina.

Quando [riprendi la sessione](#continue-conversations), Claude Code continua il turno che SIGTERM ha lasciato non completato.

<h2 id="examples">
  Esempi
</h2>

Questi esempi evidenziano i modelli CLI comuni. Dove un comando nomina un file come `auth.py` o `build-error.txt`, sostituisci un file dal tuo progetto. In CI o altri ambienti con script, aggiungi [`--bare`](#start-faster-with-bare-mode) in modo che Claude Code si avvii senza caricare gli hook, i plugin, la memoria automatica o `CLAUDE.md` dell'host.

<h3 id="pipe-data-through-claude">
  Inviare dati attraverso Claude
</h3>

La modalità non interattiva legge stdin, quindi puoi inviare dati e reindirizzare la risposta come qualsiasi altro strumento da riga di comando.

Questo esempio invia un log di compilazione a Claude e scrive la spiegazione in un file:

```bash theme={null}
cat build-error.txt | claude -p 'concisely explain the root cause of this build error' > output.txt
```

Con `--output-format json`, il payload della risposta include `total_cost_usd` e una suddivisione dei costi per modello, quindi i chiamanti con script possono tracciare la spesa senza consultare il [dashboard di utilizzo](/docs/it/costs). Quando continui una conversazione precedente con `--continue` o `--resume`, l'esecuzione segnala il totale complessivo della conversazione, [la spesa delle esecuzioni precedenti inclusa](/docs/it/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). Entrambe le cifre sono [stime lato client](/docs/it/agent-sdk/cost-tracking) e possono differire dalla tua fattura effettiva.

<Note>
  Stdin inviato tramite pipe è limitato a 10MB. Se superi il limite, Claude Code esce con un errore chiaro e uno stato diverso da zero. Per lavorare con input più grandi, scrivi il contenuto in un file e fai riferimento al percorso del file nel tuo prompt invece di inviarlo tramite pipe.
</Note>

Se Claude Code non riesce a leggere stdin, ad esempio perché il processo che lo ha avviato ha disconnesso la sua estremità, Claude Code stampa un avviso su stderr e continua con il prompt dalla riga di comando. Prima della v2.1.211, uno stdin illeggibile su Windows causava l'arresto della sessione o l'uscita silenziosa senza output.

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

Eseguilo con `npm run lint:claude`.

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
  Utilizza uno strumento come [jq](https://jqlang.org/) per analizzare la risposta ed estrarre campi specifici:

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

L'ultima riga del flusso è un messaggio `result` con il testo della risposta finale, il costo e i metadati della sessione.

Se il tuo consumer legge il flusso lentamente, Claude Code attende che l'output in coda si svuoti prima di uscire, scalando l'attesa con quanto è ancora in coda, limitato a 30 secondi. Prima della v2.1.214 l'attesa di uscita era limitata a circa due secondi, il che potrebbe troncare la fine di una risposta di grandi dimensioni.

L'esempio seguente utilizza [jq](https://jqlang.org/) per filtrare i delta di testo e visualizzare solo il testo in streaming. Il flag `-r` restituisce stringhe non elaborate (senza virgolette) e `-j` si unisce senza newline in modo che i token fluiscano continuamente:

```bash theme={null}
claude -p "Write a poem" --output-format stream-json --verbose --include-partial-messages | \
  jq -rj 'select(.type == "stream_event" and .event.delta.type? == "text_delta") | .event.delta.text'
```

Per lo streaming programmatico con callback e oggetti messaggio, consulta [Stream responses in real-time](/docs/it/agent-sdk/streaming-output) nella documentazione dell'Agent SDK.

<h4 id="follow-subagent-messages">
  Seguire i messaggi dei subagent
</h4>

I messaggi dai [subagent](/docs/it/sub-agents) appaiono nel flusso come messaggi `assistant` e `user` il cui campo `parent_tool_use_id` è l'ID della chiamata dello strumento che ha generato il subagent. I messaggi dalla conversazione principale portano `null` in quel campo.

Il primo messaggio da un subagent in esecuzione in [primo piano](/docs/it/sub-agents#run-subagents-in-foreground-or-background) è un messaggio `user` che contiene il prompt che lo guida. Dopo quel primo messaggio, Claude Code emette:

* **Per impostazione predefinita**: i blocchi `tool_use` e `tool_result` del subagent.
* **Con [`--forward-subagent-text`](/docs/it/cli-reference#cli-flags) o [`CLAUDE_CODE_FORWARD_SUBAGENT_TEXT`](/docs/it/env-vars)**: anche i blocchi di testo e thinking del subagent, in modo da poter ricostruire la trascrizione di ogni subagent. Questo richiede Claude Code v2.1.211 o successivo.

Quando abiliti una delle due opzioni, Claude Code inoltra i messaggi dai [subagent a ogni profondità di annidamento](/docs/it/sub-agents#let-subagents-spawn-their-own-subagents), indipendentemente dal fatto che ogni subagent sia stato generato con lo strumento Agent o avviato come una [skill biforcata](/docs/it/skills#run-skills-in-a-subagent). I messaggi dei subagent che una skill biforcata genera, e delle skill biforcate avviate all'interno di un subagent o di un'altra skill biforcata, richiedono Claude Code v2.1.275 o successivo. In `parent_tool_use_id`, i messaggi del subagent annidato portano l'ID della chiamata dello strumento Agent o Skill che lo ha avviato, in modo da poter ricostruire l'albero di annidamento completo seguendo quegli ID. Prima della v2.1.219, i messaggi dai subagent annidati non apparivano nel flusso.

Le skills che [vengono eseguite in un subagent](/docs/it/skills#run-skills-in-a-subagent) appaiono nel flusso allo stesso modo: il primo messaggio della skill biforcata è un messaggio `user` che contiene il contenuto della skill che guida l'esecuzione. Se abiliti una delle due opzioni, il flusso contiene anche i blocchi di testo e thinking della skill biforcata. Prima della v2.1.265, solo i blocchi `tool_use` e `tool_result` della skill biforcata apparivano nel flusso.

<h4 id="handle-api-retries">
  Gestire i tentativi API
</h4>

Quando una richiesta API non riesce con un errore ritentabile, Claude Code emette un evento `system/api_retry` prima di ritentare. Sulla v2.1.246 o successiva, quando un `401` o `403` rifiuta una credenziale [`apiKeyHelper`](/docs/it/settings-reference#apikeyhelper), Claude Code effettua i primi due tentativi silenziosamente senza evento, quindi emette l'evento come al solito dal terzo tentativo consecutivo in poi. I tentativi silenziosi contano comunque verso `attempt`. Puoi utilizzare l'evento per mostrare il progresso del tentativo nella tua interfaccia.

| Campo            | Tipo              | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ---------------- | ----------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`           | `"system"`        | tipo di messaggio                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `subtype`        | `"api_retry"`     | identifica questo come un evento di tentativo                                                                                                                                                                                                                                                                                                                                                                                     |
| `attempt`        | integer           | numero del tentativo corrente, a partire da 1                                                                                                                                                                                                                                                                                                                                                                                     |
| `max_retries`    | integer           | tentativi totali consentiti per la causa di questo errore, che possono essere meno del budget a livello di sessione                                                                                                                                                                                                                                                                                                               |
| `retry_delay_ms` | integer           | millisecondi fino al prossimo tentativo                                                                                                                                                                                                                                                                                                                                                                                           |
| `error_status`   | integer o null    | codice di stato HTTP del tentativo non riuscito, o `null` quando il tentativo non ha ricevuto alcuna risposta HTTP dall'API                                                                                                                                                                                                                                                                                                       |
| `no_response`    | object, opzionale | presente solo quando il tentativo non riuscito ha [ricevuto nessuna intestazione di risposta in tempo](/docs/it/errors#no-response-from-api). `waited_ms` è quanto tempo quel tentativo ha atteso e `retry_wait_ms` è quanto tempo il tentativo attenderà. In questi eventi, `max_retries` riflette il tentativo che questa causa normalmente ottiene, non il budget a livello di sessione. Richiede Claude Code v2.1.261 o successivo |
| `error`          | string            | categoria di errore: `authentication_failed`, `oauth_org_not_allowed`, `account_on_hold`, `billing_error`, `rate_limit`, `overloaded`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `cloud_credential_error`, o `unknown`                                                                                                                                                                           |
| `uuid`           | string            | identificatore evento univoco                                                                                                                                                                                                                                                                                                                                                                                                     |
| `session_id`     | string            | sessione a cui appartiene l'evento                                                                                                                                                                                                                                                                                                                                                                                                |

<h4 id="read-session-metadata">
  Leggere i metadati della sessione
</h4>

L'evento `system/init` segnala i metadati della sessione inclusi il modello, gli strumenti, i server MCP e i plugin caricati. È il primo evento nel flusso a meno che gli eventi di avvio lo precedano:

* Eventi `plugin_install`, quando [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/it/env-vars) è impostato.
* [`hook_started`, `hook_progress` e `hook_response` eventi](/docs/it/agent-sdk/typescript#sdkhookstartedmessage), mentre un hook [`SessionStart`](/docs/it/hooks#sessionstart) o [`Setup`](/docs/it/hooks#setup) configurato viene eseguito. Questi vengono trasmessi mentre l'hook li produce. Claude Code v2.1.169 attraverso v2.1.203 li ha consegnati in un batch dopo il completamento dell'hook, ancora prima di `system/init`; v2.1.204 ha ripristinato la consegna in tempo reale.

L'evento contiene anche un array `capabilities` opzionale di stringhe che denominano i comportamenti del protocollo che questa versione di Claude Code implementa, come `interrupt_receipt_v1` o `interrupt_cancel_queued_v1`. Controllalo per rilevare le funzionalità invece di confrontare le stringhe di versione, e ignora i valori che non riconosci. Il campo richiede Claude Code v2.1.205 o successivo ed è assente dalle versioni precedenti. Consulta [`SDKSystemMessage`](/docs/it/agent-sdk/typescript#sdksystemmessage) per l'elenco delle capacità.

<h4 id="fail-ci-when-a-plugin-or-mcp-server-doesn’t-load">
  Far fallire CI quando un plugin o un server MCP non si carica
</h4>

Utilizza i campi plugin nell'evento `system/init` per rilevare un plugin che non è stato caricato:

| Campo           | Tipo  | Descrizione                                                                                                                                                                                                                                                                                                                              |
| --------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plugins`       | array | plugin che sono stati caricati con successo, ognuno con `name` e `path`                                                                                                                                                                                                                                                                  |
| `plugin_errors` | array | errori di caricamento del plugin, ognuno con `plugin`, `type` e `message`. Include versioni di dipendenza non soddisfatte e errori di caricamento di `--plugin-dir` come un percorso mancante o un archivio non valido. I plugin interessati vengono declassati e assenti da `plugins`. La chiave viene omessa quando non ci sono errori |

Utilizza i campi del server MCP allo stesso modo. Quando passi [`--mcp-config`](/docs/it/cli-reference#cli-flags) con `-p`, Claude Code attende i server ancora in sospeso prima di eseguire il primo turno, fino al timeout di avvio [`MCP_TIMEOUT`](/docs/it/env-vars), 30 secondi per impostazione predefinita. Un server remoto con un [elenco di strumenti memorizzato nella cache](/docs/it/agent-sdk/mcp#connection-timing) salta l'attesa, mostra `pending` in `system/init` e si connette alla sua prima chiamata dello strumento. L'attesa richiede Claude Code v2.1.221 o successivo.

Claude Code convalida ogni voce `--mcp-config` all'avvio e salta le voci che non superano la convalida, ad esempio una voce `url` senza `type`. L'esecuzione continua e esce correttamente, quindi controlla questi campi per rilevare un server che non è mai stato caricato:

| Campo               | Tipo  | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mcp_servers`       | array | server MCP nella sessione, ognuno con `name` e `status`                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `mcp_server_errors` | array | voci `--mcp-config` saltate dalla convalida della configurazione, ognuna con `name`, `type` e `message`. `type` è una categoria di salto come `unknown_type`, `url_missing_type`, `invalid_config` o `reserved_name`; tratta i valori che non riconosci come un salto generico. I server interessati sono assenti da `mcp_servers`. La chiave viene omessa quando non ci sono errori, quindi un gate CI può fallire su un array non vuoto. Richiede Claude Code v2.1.219 o successivo |

Quando esegui il comando a mano in un terminale, Claude Code stampa anche un avviso di avvio su stderr, come `Warning: 1 MCP server skipped due to invalid config:`, seguito dal motivo di ogni voce saltata. Quando reindizzi stderr, o quando un programma come un runner CI o un host SDK lo cattura, Claude Code non stampa alcun avviso e segnala le voci saltate solo nel campo `mcp_server_errors`. L'avviso richiede Claude Code v2.1.219 o successivo.

<h4 id="track-plugin-installs">
  Tracciare le installazioni dei plugin
</h4>

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

<h3 id="auto-approve-tools">
  Approvare automaticamente gli strumenti
</h3>

Utilizza `--allowedTools` per consentire a Claude di utilizzare determinati strumenti senza chiedere. Questo esempio esegue una suite di test e corregge i guasti, consentendo a Claude di eseguire comandi Bash e leggere/modificare file senza chiedere il permesso:

```bash theme={null}
claude -p "Run the test suite and fix any failures" \
  --allowedTools "Bash,Read,Edit"
```

Per impostare una linea di base per l'intera sessione invece di elencare i singoli strumenti, passa una [modalità di autorizzazione](/docs/it/permission-modes). Per `-p`, la [modalità di autorizzazione iniziale incorporata](/docs/it/permission-modes#which-mode-a-session-starts-in) è Manual su ogni piano, quindi passa la modalità di autorizzazione che desideri:

* **`auto`**: passa `--permission-mode auto` per avere un classificatore che esamini la maggior parte delle azioni invece di te
* **`dontAsk`**: Claude Code nega qualsiasi cosa che comporterebbe altrimenti un prompt, il che è utile per esecuzioni CI bloccate. Le azioni che non necessitano di approvazione in modalità Manual vengono comunque eseguite, come le letture di file nelle tue directory di lavoro e il [set di comandi di sola lettura](/docs/it/permissions#read-only-commands), così come le azioni coperte dalle tue voci `--allowedTools` o dalle regole `permissions.allow`. `AskUserQuestion`, strumenti connettore [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools), e strumenti MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool) vengono negati anche quando una regola di autorizzazione corrisponde
* **`acceptEdits`**: Claude scrive file senza chiedere, e Claude Code approva automaticamente i comandi del filesystem comuni come `mkdir`, `touch`, `mv` e `cp`. Le [azioni che nessuna modalità approva automaticamente](/docs/it/permission-modes#actions-no-mode-auto-approves) si applicano comunque. A parte il set di comandi di sola lettura, altri comandi shell e richieste di rete hanno ancora bisogno di una voce `--allowedTools` o di una regola `permissions.allow`. Consulta [cosa `acceptEdits` approva automaticamente](/docs/it/permission-modes#auto-approve-file-edits-with-acceptedits-mode) per l'elenco completo

Questo esempio applica le correzioni di lint con `acceptEdits` come linea di base:

```bash theme={null}
claude -p "Apply the lint fixes" --permission-mode acceptEdits
```

<h3 id="turn-off-permission-prompts-in-unattended-runs">
  Disattivare i prompt di autorizzazione nelle esecuzioni incustodite
</h3>

Passa `--permission-prompts none` quando nessuno è disponibile per rispondere ai prompt di autorizzazione, ad esempio in un lavoro programmato. Il flag è più importante quando la tua esecuzione ha un host di autorizzazione: un'app Agent SDK con un callback [`canUseTool`](/docs/it/agent-sdk/user-input), o uno strumento MCP che passi con [`--permission-prompt-tool`](/docs/it/cli-reference#cli-flags). Senza il flag, la tua esecuzione attende che quell'host risponda a ogni richiesta di autorizzazione.

Con il flag, la tua esecuzione non consulta l'host e non lo attende. Qualsiasi cosa che comporterebbe un prompt viene negata a meno che un hook `PermissionRequest` non lo consenta, a Claude viene detto che nessuno può approvare la richiesta e di non ritentarla, e l'esecuzione continua. In un'esecuzione `-p` senza host, queste richieste vengono negate comunque, e il flag dice anche a Claude di non ritentarle. Le regole di autorizzazione, gli hook [`PermissionRequest`](/docs/it/hooks#permissionrequest) e la modalità di autorizzazione che imposti decidono comunque ogni chiamata per prima; Claude Code nega solo le richieste che nient'altro risolve.

Questo esempio esegue un'attività incustodita in [modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode). Il classificatore esamina ogni azione come al solito, e Claude Code nega qualsiasi cosa che avrebbe altrimenti richiesto un prompt:

```bash theme={null}
claude -p "Update the dependency pins and run the tests" --permission-mode auto --permission-prompts none
```

Con `--permission-prompts none`, Claude Code rimuove gli strumenti che hanno bisogno di una risposta da una persona, come [`AskUserQuestion`](/docs/it/tools-reference#askuserquestion-tool-behavior), in modo che Claude non possa chiamarli. Qualsiasi [richiesta di elicitazione MCP](/docs/it/mcp#respond-to-mcp-elicitation-requests) che nessun hook [`Elicitation`](/docs/it/hooks#elicitation) risponde viene annullata.

Con `--output-format stream-json`, i rifiuti appaiono come messaggi di sistema `permission_denied`, e il messaggio di risultato finale li elenca in `permission_denials`.

<Note>
  Il flag `--permission-prompts` richiede Claude Code v2.1.259 o successivo. Le versioni precedenti lo rifiutano con un errore di opzione sconosciuta.
</Note>

<h3 id="create-a-commit">
  Creare un commit
</h3>

Questo esempio esamina le modifiche in staging e crea un commit con un messaggio appropriato:

```bash theme={null}
claude -p "Look at my staged changes and create an appropriate commit" \
  --allowedTools "Bash(git diff *),Bash(git log *),Bash(git status *),Bash(git commit *)"
```

Il flag `--allowedTools` utilizza la [sintassi delle regole di autorizzazione](/docs/it/settings-reference#permission-rule-syntax). Lo spazio finale ` *` abilita la corrispondenza dei prefissi, quindi `Bash(git diff *)` consente qualsiasi comando che inizia con `git diff`. Lo spazio prima di `*` è importante: senza di esso, `Bash(git diff*)` corrisponderebbe anche a `git diff-index`.

<Note>
  Il supporto dei comandi differisce in modalità `-p`:

  * Le [skills](/docs/it/skills) richiamate dall'utente e i comandi personalizzati funzionano. Includi `/skill-name` nella stringa del prompt e Claude Code lo espande prima di eseguire.
  * I comandi incorporati che solo eseguono nell'interfaccia del terminale, come `/login`, non sono disponibili.
  * `/model`, `/effort`, `/fast`, `/color` e `/rename` accettano il valore come argomento, ad esempio `/model sonnet`, e `/mcp` senza argomento stampa un riepilogo di testo dello stato del server. Questi moduli richiedono Claude Code v2.1.205 o successivo e seguono le [note di disponibilità](/docs/it/commands#all-commands) di ogni comando.
  * Per modificare un'impostazione, passa `key=value` a `/config`, ad esempio `/config thinking=false`.
  * `/output-style <style>` cambia gli [stili di output](/docs/it/output-styles) e `/output-style` da solo li elenca. Richiede Claude Code v2.1.269 o successivo.
</Note>

<h3 id="customize-the-system-prompt">
  Personalizzare il prompt di sistema
</h3>

Utilizza `--append-system-prompt` per aggiungere istruzioni mantenendo il comportamento predefinito di Claude Code. Questo esempio invia un diff PR a Claude e gli istruisce di esaminarlo per vulnerabilità di sicurezza. Salvalo come script shell, ad esempio `review.sh`:

```bash theme={null}
gh pr diff "$1" | claude -p \
  --append-system-prompt "You are a security engineer. Review for vulnerabilities." \
  --output-format json
```

Nello script, `"$1"` rappresenta il primo argomento che passi sulla riga di comando. Esegui `bash review.sh 123` e la shell sostituisce `"$1"` con `123`, in modo che lo script recuperi il diff per il PR 123. Claude Code stampa la revisione come JSON, con il testo nel campo `result`.

Consulta i [flag del prompt di sistema](/docs/it/cli-reference#system-prompt-flags) per ulteriori opzioni incluso `--system-prompt` per sostituire completamente il prompt predefinito.

<h3 id="continue-conversations">
  Continuare le conversazioni
</h3>

Utilizza `--continue` per continuare la conversazione più recente, oppure `--resume` con un ID sessione per continuare una conversazione specifica. Su Claude Code v2.1.257 o successivo, quando passi `--continue`, Claude Code apre una [sessione in background](/docs/it/sessions#resume-a-session) che è terminata, ma non una che è ancora in esecuzione. Questo esempio esegue una revisione, quindi invia prompt di follow-up:

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

Puoi eseguire i due comandi da directory diverse: Claude Code [trova la sessione dal suo ID](/docs/it/sessions#resume-a-session) in qualsiasi progetto su questa macchina. Prima della v2.1.223, Claude Code cercava l'ID solo nella directory del progetto corrente e nei suoi git worktrees, quindi dovevi eseguire entrambi i comandi dalla stessa directory.

Al posto dell'ID sessione, puoi passare a `--resume` il percorso assoluto al file di [trascrizione](/docs/it/sessions#where-transcripts-are-stored) `.jsonl` di una sessione, e Claude Code continua la conversazione memorizzata in quel file.

<h2 id="next-steps">
  Passaggi successivi
</h2>

* [Agent SDK quickstart](/docs/it/agent-sdk/quickstart): costruisci il tuo primo agente con Python o TypeScript
* [CLI reference](/docs/it/cli-reference): tutti i flag e le opzioni CLI
* [GitHub Actions](/docs/it/github-actions): utilizza l'Agent SDK nei flussi di lavoro GitHub
* [GitLab CI/CD](/docs/it/gitlab-ci-cd): utilizza l'Agent SDK nelle pipeline GitLab
