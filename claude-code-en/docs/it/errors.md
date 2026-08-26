> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Riferimento degli errori

> Consulta i messaggi di errore di runtime di Claude Code con il significato di ciascuno e come risolverli.

Questa pagina elenca gli errori di runtime che Claude Code visualizza e come recuperare da ciascuno, oltre a cosa controllare quando le risposte sembrano non corrette senza un errore. Per gli errori di installazione come `command not found` o errori TLS durante la configurazione, vedi [Troubleshooting installation and login](/docs/it/troubleshoot-install).

Questi errori e i comandi di recupero si applicano su CLI, l'[app Desktop](/docs/it/desktop) e [Claude Code sul web](/docs/it/claude-code-on-the-web), poiché tutti e tre avvolgono lo stesso Claude Code CLI. Per problemi specifici della superficie, vedi la sezione troubleshooting nella pagina di quella superficie.

<Note>
  Claude Code chiama l'API Claude per le risposte del modello, quindi la maggior parte degli errori di runtime si mappano a un codice di errore API sottostante. Questa pagina copre cosa significa ogni errore all'interno di Claude Code e come recuperare. Per le definizioni del codice di stato HTTP grezzo, vedi il [riferimento degli errori della piattaforma Claude](https://platform.claude.com/docs/en/api/errors).
</Note>

<h2 id="find-your-error">
  Trova il tuo errore
</h2>

Abbina il messaggio che vedi nel tuo terminale a una sezione sottostante.

| Messaggio                                                                                          | Sezione                                                                                                                           |
| :------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------- |
| `API Error: 500 Internal server error`                                                             | [Errori del server](#api-error-500-internal-server-error)                                                                         |
| `API Error: Repeated 529 Overloaded errors`                                                        | [Errori del server](#api-error-repeated-529-overloaded-errors)                                                                    |
| `Request timed out`                                                                                | [Errori del server](#request-timed-out), o [Rete](#unable-to-connect-to-api) se il messaggio menziona la tua connessione internet |
| `Server error mid-response. The response above may be incomplete.`                                 | [Errori del server](#the-response-above-may-be-incomplete)                                                                        |
| `Connection closed mid-response` / `Response stalled mid-stream`                                   | [Errori del server](#the-response-above-may-be-incomplete)                                                                        |
| `<model> is temporarily unavailable, so auto mode cannot determine the safety of...`               | [Errori del server](#auto-mode-cannot-determine-the-safety-of-an-action)                                                          |
| `Auto mode could not evaluate this action and is blocking it for safety`                           | [Errori del server](#auto-mode-cannot-determine-the-safety-of-an-action)                                                          |
| `Auto mode classifier transcript exceeded context window`                                          | [Errori del server](#auto-mode-cannot-determine-the-safety-of-an-action)                                                          |
| `Agent terminated early due to an API error`                                                       | [Errori del server](#agent-terminated-early-due-to-an-api-error)                                                                  |
| `You've hit your session limit` / `You've hit your weekly limit`                                   | [Limiti di utilizzo](#youve-hit-your-session-limit)                                                                               |
| `Usage credits required for 1M context`                                                            | [Limiti di utilizzo](#usage-credits-required-for-1m-context)                                                                      |
| `Server is temporarily limiting requests`                                                          | [Limiti di utilizzo](#server-is-temporarily-limiting-requests)                                                                    |
| `Request rejected (429)`                                                                           | [Limiti di utilizzo](#request-rejected-429)                                                                                       |
| `Credit balance is too low`                                                                        | [Limiti di utilizzo](#credit-balance-is-too-low)                                                                                  |
| `Not logged in · Please run /login`                                                                | [Autenticazione](#not-logged-in)                                                                                                  |
| `Could not resolve authentication method`                                                          | [Autenticazione](#could-not-resolve-authentication-method)                                                                        |
| `Invalid API key`                                                                                  | [Autenticazione](#invalid-api-key)                                                                                                |
| `Your apiKeyHelper script is failing`                                                              | [Autenticazione](#your-apikeyhelper-script-is-failing)                                                                            |
| `This organization has been disabled`                                                              | [Autenticazione](#this-organization-has-been-disabled)                                                                            |
| `Your organization has disabled API key authentication`                                            | [Autenticazione](#your-organization-has-disabled-api-key-authentication)                                                          |
| `Your organization has disabled Claude subscription access`                                        | [Autenticazione](#your-organization-has-disabled-claude-subscription-access)                                                      |
| `Routines are disabled by your organization's policy`                                              | [Autenticazione](#routines-are-disabled-by-your-organizations-policy)                                                             |
| `Remote Control is only available when using Claude via api.anthropic.com`                         | [Autenticazione](#remote-control-requires-the-anthropic-api)                                                                      |
| `OAuth token revoked` / `OAuth token has expired`                                                  | [Autenticazione](#oauth-token-revoked-or-expired)                                                                                 |
| `Login expired · Please run /login`                                                                | [Autenticazione](#login-expired)                                                                                                  |
| `Failed to authenticate: OAuth session expired and could not be refreshed`                         | [Autenticazione](#login-expired)                                                                                                  |
| `does not meet scope requirement user:profile`                                                     | [Autenticazione](#oauth-scope-requirement)                                                                                        |
| `AWS credentials expired or invalid`                                                               | [Autenticazione](#aws-credentials-expired-or-invalid)                                                                             |
| `AWS authentication failed`                                                                        | [Autenticazione](#aws-authentication-failed)                                                                                      |
| `AWS default-chain credential resolve timed out`                                                   | [Autenticazione](#aws-default-chain-credential-resolve-timed-out)                                                                 |
| `Unable to connect to API`                                                                         | [Rete](#unable-to-connect-to-api)                                                                                                 |
| `Waiting for API response · will retry in`                                                         | [Tentativi automatici](#automatic-retries), o [Rete](#unable-to-connect-to-api) se persiste                                       |
| `Bedrock streaming response has content-type "..."; expected "application/vnd.amazon.eventstream"` | [Rete](#bedrock-streaming-response-has-an-unexpected-content-type)                                                                |
| `SSL certificate verification failed`                                                              | [Rete](#ssl-certificate-errors)                                                                                                   |
| `SSL certificate error (...)` during login or startup                                              | [Rete](#ssl-certificate-errors)                                                                                                   |
| `403` with `x-deny-reason: host_not_allowed` in a cloud or routine session                         | [Rete](#host-not-allowed-in-a-cloud-session)                                                                                      |
| `Couldn't reconnect to your Remote Control session`                                                | [Rete](#couldnt-reconnect-to-your-remote-control-session)                                                                         |
| `Prompt is too long`                                                                               | [Errori di richiesta](#prompt-is-too-long)                                                                                        |
| `Error during compaction: Conversation too long`                                                   | [Errori di richiesta](#error-during-compaction-conversation-too-long)                                                             |
| `Request too large`                                                                                | [Errori di richiesta](#request-too-large)                                                                                         |
| `Image was too large`                                                                              | [Errori di richiesta](#image-was-too-large)                                                                                       |
| `Unable to resize image`                                                                           | [Errori di richiesta](#unable-to-resize-image)                                                                                    |
| `PDF too large` / `PDF is password protected`                                                      | [Errori di richiesta](#pdf-errors)                                                                                                |
| `Extra inputs are not permitted`                                                                   | [Errori di richiesta](#extra-inputs-are-not-permitted)                                                                            |
| `There's an issue with the selected model`                                                         | [Errori di richiesta](#theres-an-issue-with-the-selected-model)                                                                   |
| `Model ... is not a recognized model id`                                                           | [Errori di richiesta](#model-is-not-a-recognized-model-id)                                                                        |
| `Claude Opus is not available with the Claude Pro plan`                                            | [Errori di richiesta](#claude-opus-is-not-available-with-the-claude-pro-plan)                                                     |
| `Model ... is restricted by your organization's settings`                                          | [Errori di richiesta](#model-is-restricted-by-your-organizations-settings)                                                        |
| `thinking.type.enabled is not supported for this model`                                            | [Errori di richiesta](#thinking-type-enabled-is-not-supported-for-this-model)                                                     |
| `max_tokens must be greater than thinking.budget_tokens`                                           | [Errori di richiesta](#thinking-budget-exceeds-output-limit)                                                                      |
| `API Error: 400 due to tool use concurrency issues`                                                | [Errori di richiesta](#tool-use-or-thinking-block-mismatch)                                                                       |
| `Claude Code is unable to respond to this request, which appears to violate our Usage Policy`      | [Errori di richiesta](#usage-policy-refusal)                                                                                      |
| `<model> has safety measures that flagged this message for a cybersecurity topic`                  | [Errori di richiesta](#safety-measures-flagged-a-cybersecurity-topic)                                                             |
| `Installation was killed before it could finish (exit code 137)`                                   | [Errori di installazione](#installation-was-killed-before-it-could-finish)                                                        |
| `The connection dropped while downloading the update`                                              | [Errori di installazione](#the-connection-dropped-while-downloading-the-update)                                                   |
| `Download timed out: exceeded the total deadline`                                                  | [Errori di installazione](#the-connection-dropped-while-downloading-the-update)                                                   |
| `--bg and --print conflict`                                                                        | [Errori della riga di comando](#command-line-errors)                                                                              |
| `Error: --json-schema is not a valid JSON Schema`                                                  | [Errori della riga di comando](#command-line-errors)                                                                              |
| `Could not import <server>: <reason>`                                                              | [Errori della riga di comando](#could-not-import-a-server-from-claude-desktop)                                                    |
| `Error: MCP tool <name> (passed via --permission-prompt-tool) not found`                           | [Errori della riga di comando](#mcp-permission-prompt-tool-not-found)                                                             |
| `Marketplace "<name>" is registered from an untrusted source`                                      | [Errori dei plugin](#marketplace-is-registered-from-an-untrusted-source)                                                          |
| `references ${user_config.*} in a shell-form command`                                              | [Errori dei plugin](#plugin-command-references-user-config)                                                                       |
| `Monitor "<name>" from plugin <plugin> references ${user_config.*} in its command`                 | [Errori dei plugin](#plugin-command-references-user-config)                                                                       |
| `headersHelper for MCP server '<name>' references ${user_config.*}`                                | [Errori dei plugin](#plugin-command-references-user-config)                                                                       |
| `would be spawned with zero tools — refusing`                                                      | [Errori degli strumenti](#agent-would-be-spawned-with-zero-tools)                                                                 |
| `File is covered by a Read deny rule in your permission settings`                                  | [Errori degli strumenti](#file-is-covered-by-a-read-deny-rule)                                                                    |
| `Can't open MCP settings in a background session`                                                  | [Errori della sessione in background](#commands-refused-in-a-background-session)                                                  |
| `CLAUDE_CODE_PROCESS_WRAPPER: launcher ...`                                                        | [Errori della sessione in background](#claude_code_process_wrapper-launcher-errors)                                               |
| `Ignoring N permissions.allow entries from ... this workspace has not been trusted`                | [Avvisi di configurazione](#workspace-has-not-been-trusted)                                                                       |
| Responses seem lower quality than usual                                                            | [Qualità della risposta](#responses-seem-lower-quality-than-usual)                                                                |

<h2 id="automatic-retries">
  Tentativi automatici
</h2>

Claude Code ritenta i guasti transitori prima di mostrarti un errore. Gli errori del server, le risposte sovraccariche, i timeout delle richieste, i throttle 429 temporanei e le connessioni interrotte vengono tutti ritentati fino a 10 volte con backoff esponenziale. A partire da v2.1.198, questo copre le connessioni che si interrompono nel mezzo di una risposta prima che qualsiasi output visibile sia stato trasmesso: Claude Code ri-emette la richiesta con lo stesso backoff e il turno continua invece di fermarsi con un errore di connessione. A partire da v2.1.199, i throttle 429 temporanei che non portano le intestazioni di quota del tuo piano vengono anche ritentati quando sei connesso con un abbonamento claude.ai; le versioni precedenti li ritentavano solo per le chiavi API e gli accessi Enterprise.

Alcune classi di guasti non vengono ritentate, perché un tentativo non può avere successo:

* A partire da v2.1.199, un guasto di convalida del certificato TLS, come un proxy che ispeziona TLS, un bundle `NODE_EXTRA_CA_CERTS` mancante, o un certificato scaduto, fallisce al primo tentativo in modo che la correzione appaia immediatamente invece che dopo il budget di tentativo completo. Vedi [Errori del certificato SSL](#ssl-certificate-errors). Le condizioni TLS transitorie come un timeout di handshake continuano a ritentare.
* A partire da v2.1.199, un errore del server che arriva dopo che Claude ha già trasmesso output visibile mantiene la risposta parziale e aggiunge un [avviso di risposta incompleta](#the-response-above-may-be-incomplete) invece di ritentare, poiché ri-eseguire la richiesta potrebbe eseguire gli stessi strumenti due volte. Le versioni precedenti scartavano l'output parziale e segnalano il turno come un errore.
* Una [risposta di streaming Amazon Bedrock con un content-type inaspettato](#bedrock-streaming-response-has-an-unexpected-content-type) fallisce al primo tentativo, perché il gateway o il proxy che riscrive la risposta riscriverebbero il tentativo nello stesso modo. Richiede Claude Code v2.1.208 o successivo.

Durante il tentativo, lo spinner mostra un countdown `Retrying in Ns · attempt x/y` dopo un'etichetta di errore. L'etichetta nomina il motivo specifico dal primo tentativo per i guasti su cui puoi agire subito: la rete è inattiva, un handshake TLS non è riuscito, o hai raggiunto un limite di velocità. Per altri errori legge `API error` all'inizio. A partire da v2.1.198 passa al motivo specifico dal terzo tentativo, o al tentativo finale quando `CLAUDE_CODE_MAX_RETRIES` consente meno di tre; le versioni precedenti passano solo al tentativo finale.

A partire da v2.1.198, il suggerimento dello spinner usuale è soppresso durante i tentativi. Una volta rivelato il motivo dell'errore, se il guasto è un sovraccarico 529 la riga sotto il countdown nomina anche dove controllare lo stato del servizio: `status.claude.com` sull'API Anthropic, o l'host del provider o gateway indicato nel messaggio su altre configurazioni.

Se nessun dato arriva sul flusso di risposta per 20 secondi mentre una richiesta è ancora in sospeso, lo spinner mostra `Waiting for API response · will retry in … · check your network` prima che qualsiasi tentativo sia iniziato. La richiesta non è ancora fallita: il countdown viene eseguito fino al punto in cui Claude Code interrompe la connessione bloccata e ritenta, quindi il banner si cancella da solo una volta che i dati riprendono o il tentativo ha successo. A partire da v2.1.185 la soglia è di 20 secondi; le versioni precedenti mostrano il banner dopo 10 secondi con una formulazione diversa. Se riappare ad ogni tentativo, trattalo come un [problema di rete](#unable-to-connect-to-api).

Quando vedi uno degli errori in questa pagina, quei tentativi sono già stati esauriti, a meno che non appartenga a una classe che non viene ritentata, come un guasto di convalida del certificato. Puoi regolare il comportamento con queste variabili di ambiente:

| Variabile                                    | Predefinito   | Effetto                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| :------------------------------------------- | :------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [`CLAUDE_CODE_MAX_RETRIES`](/docs/it/env-vars)    | 10            | Numero di tentativi di ripetizione. Limitato a 15 a partire da v2.1.186; a partire da v2.1.199 `CLAUDE_CODE_RETRY_WATCHDOG` aumenta il valore predefinito e rimuove il limite. Abbassalo per far emergere i guasti più velocemente negli script.                                                                                                                                                                                                                                                              |
| [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/it/env-vars) | non impostato | Impostalo su `1` in sessioni incustodite come i job CI per ritentare gli errori di capacità `429` e `529` indefinitamente invece di fallire dopo i tentativi di `CLAUDE_CODE_MAX_RETRIES`. A partire da v2.1.199 aumenta anche il conteggio di tentativi predefinito per altri errori transitori, come errori del server, timeout e connessioni interrotte, a 300, approssimativamente tre ore di backoff, e rimuove il limite di 15 su `CLAUDE_CODE_MAX_RETRIES` se imposti esplicitamente quella variabile. |
| [`API_TIMEOUT_MS`](/docs/it/env-vars)             | 600000        | Timeout per richiesta in millisecondi. Aumentalo per reti lente o proxy.                                                                                                                                                                                                                                                                                                                                                                                                                                      |

<h2 id="server-errors">
  Errori del server
</h2>

Questi errori provengono dal provider di inferenza piuttosto che dal vostro account o dalla vostra richiesta. Sull'API Anthropic significa l'infrastruttura Anthropic. Su Amazon Bedrock, Agent Platform di Google Cloud, Microsoft Foundry o un gateway personalizzato significa l'infrastruttura di quel provider.

<h3 id="api-error-500-internal-server-error">
  API Error: 500 Internal server error
</h3>

Claude Code mostra il codice di stato e il messaggio di errore dell'API per qualsiasi risposta 5xx. L'esempio seguente mostra una risposta 500 sull'API Anthropic:

```text theme={null}
API Error: 500 Internal server error. This is a server-side issue, usually temporary — try again in a moment. If it persists, check https://status.claude.com.
```

La frase finale indica dove controllare lo stato del servizio e varia in base al provider. Le configurazioni di Amazon Bedrock, Agent Platform di Google Cloud e Microsoft Foundry indicano lo stato del servizio di quel provider. Un `ANTHROPIC_BASE_URL` personalizzato indica l'host del gateway.

Questo indica un errore imprevisto all'interno dell'API. Non è causato dal vostro prompt, dalle impostazioni o dall'account.

**Cosa fare:**

* Controllate [status.claude.com](https://status.claude.com), o la pagina di stato del provider indicata nel messaggio, per gli incidenti attivi
* Aspettate un minuto, quindi inviate di nuovo il vostro messaggio. Il vostro messaggio originale è ancora nella conversazione, quindi per un prompt lungo potete digitare `try again` invece di incollare l'intera cosa.
* Se l'errore persiste senza alcun incidente segnalato, eseguite `/feedback` in modo che Anthropic possa investigare con i dettagli della vostra richiesta. Consultate [Report an error](#report-an-error) se `/feedback` non è disponibile nel vostro ambiente.

<h3 id="api-error-repeated-529-overloaded-errors">
  API Error: Repeated 529 Overloaded errors
</h3>

L'API è temporaneamente al massimo della capacità per tutti gli utenti. Claude Code ha già ritentato più volte prima di mostrare questo messaggio:

```text theme={null}
API Error: Repeated 529 Overloaded errors. The API is at capacity — this is usually temporary. Try again in a moment. If it persists, check https://status.claude.com.
```

La frase finale varia in base al provider nello stesso modo dell'errore 500 sopra.

Un 529 non è il vostro limite di utilizzo e non conta rispetto alla vostra quota.

**Cosa fare:**

* Controllate [status.claude.com](https://status.claude.com), o la pagina di stato del provider indicata nel messaggio, per gli avvisi di capacità
* Ritentate tra pochi minuti
* Eseguite `/model` e passate a un modello diverso per continuare a lavorare, poiché la capacità è tracciata per modello. Claude Code vi chiede di farlo quando un modello è sotto un carico particolarmente elevato, ad esempio `Opus is experiencing high load, please use /model to switch to Sonnet`.

<h3 id="request-timed-out">
  Request timed out
</h3>

L'API non ha risposto prima della scadenza della connessione.

```text theme={null}
Request timed out
```

Questo può accadere durante periodi di carico elevato o quando il modello sta generando una risposta molto grande. Il timeout di richiesta predefinito è di 10 minuti.

**Cosa fare:**

* Ritentate la richiesta
* Per attività di lunga durata, suddividete il lavoro in prompt più piccoli
* Se la causa è una rete lenta o un proxy, aumentate `API_TIMEOUT_MS` come descritto in [Automatic retries](#automatic-retries)
* Se i timeout sono frequenti e la vostra rete è altrimenti sana, consultate [Network and connection errors](#network-and-connection-errors) di seguito

<h3 id="the-response-above-may-be-incomplete">
  The response above may be incomplete
</h3>

Una risposta in streaming non è riuscita dopo che Claude aveva già prodotto un output visibile. L'invio nuovamente della richiesta potrebbe eseguire le stesse chiamate di strumento due volte, quindi Claude Code mantiene ciò che è già stato trasmesso e aggiunge questo avviso invece di scartare il turno. La variante che vedete indica la causa:

```text theme={null}
API Error: Server error mid-response. The response above may be incomplete.
API Error: Connection closed mid-response. The response above may be incomplete.
API Error: Response stalled mid-stream. The response above may be incomplete.
```

* `Server error mid-response`: un errore di server sovraccarico o 5xx a metà flusso. Questa variante richiede Claude Code v2.1.199 o successivo; prima di allora quel caso scartava l'output parziale e segnalava l'intero turno come errore.
* `Connection closed mid-response`: la connessione è stata interrotta.
* `Response stalled mid-stream`: il flusso ha smesso di inviare dati.

**Cosa fare:**

* Leggete la risposta che è stata trasmessa. Nulla è stato perso, ma le frasi finali o le chiamate di strumento potrebbero mancare.
* Rispondete con `continue` per fare in modo che Claude riprenda da dove si era fermato
* Se lo stesso errore appare prima di qualsiasi output visibile, Claude Code ritenta la richiesta invece di finalizzarla. Consultate [Automatic retries](#automatic-retries).

<h3 id="auto-mode-cannot-determine-the-safety-of-an-action">
  Auto mode cannot determine the safety of an action
</h3>

Il modello che [auto mode](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) utilizza per classificare le azioni non ha potuto produrre una decisione, quindi auto mode non ha approvato l'azione automaticamente. Il messaggio che vedete dipende dal motivo per cui il classificatore non è riuscito.

Le letture, le ricerche e le modifiche all'interno della vostra directory di lavoro saltano il classificatore, quindi continuano a funzionare in tutti questi casi.

Quando il modello classificatore è sovraccarico:

```text theme={null}
<model> is temporarily unavailable, so auto mode cannot determine the safety of <tool> right now. Wait briefly and then try this action again.
```

**Cosa fare:**

* Ritentate dopo pochi secondi; Claude vede lo stesso messaggio e di solito ritenta da solo
* Se i tentativi continuano a fallire, continuate con attività di sola lettura e tornate all'azione bloccata in seguito
* Questo è transitorio e non correlato all'[idoneità di auto mode](/docs/it/permission-modes#eliminate-prompts-with-auto-mode); non è necessario modificare le impostazioni

Quando il classificatore ha restituito una risposta non analizzabile:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — run with --debug for details
```

**Cosa fare:**

* Ritentate l'azione; di solito ha successo al tentativo successivo
* Eseguite `claude --debug` e ripetete l'azione per vedere la risposta del classificatore sottostante nel log di debug

Quando un controllo di sicurezza API separato ha bloccato la richiesta del classificatore a causa del contenuto della conversazione precedente:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — a safety check separate from auto mode blocked this request because of earlier conversation content — it isn't about the action itself — run with --debug for details
```

**Cosa fare:**

* Questa non è una decisione sulla vostra azione. Il contenuto già nella vostra conversazione ha attivato un filtro di sicurezza sull'API quando auto mode ha inviato la conversazione al classificatore
* Ritentare non aiuterà; lo stesso contenuto della conversazione attiverà di nuovo il filtro
* Passate a una [modalità di autorizzazione](/docs/it/permission-modes) diversa in modo da poter approvare l'azione quando richiesto, o iniziate una conversazione nuova senza il contenuto che attiva il filtro

Quando la conversazione è cresciuta oltre la finestra di contesto del classificatore:

```text theme={null}
Auto mode classifier transcript exceeded context window — falling back to manual approval (try /compact to reduce conversation size)
```

In una sessione interattiva, auto mode ritorna a un normale prompt di autorizzazione per quell'azione in modo da poter approvare o negare manualmente. In [modalità non interattiva](/docs/it/headless) l'esecuzione si interrompe perché la trascrizione cresce solo e ritentare non può avere successo.

**Cosa fare:**

* Approvate o negate l'azione nel prompt che appare
* Eseguite `/compact` per ridurre la dimensione della conversazione in modo che le azioni successive si adattino di nuovo alla finestra del classificatore

<h3 id="agent-terminated-early-due-to-an-api-error">
  Agent terminated early due to an API error
</h3>

La richiesta API di un [subagent](/docs/it/sub-agents) non è riuscita in modo terminale, ad esempio perché è stato raggiunto un limite di utilizzo o i tentativi per un errore del server si sono esauriti, quindi il subagent si è fermato prima di completare il suo compito. Questo messaggio richiede Claude Code v2.1.199 o successivo; prima di allora il testo di errore dell'API è stato restituito a Claude come se fosse il risultato del subagent.

```text theme={null}
Agent terminated early due to an API error: <error detail>
```

**Cosa fare:**

* Abbinate il dettaglio dell'errore dopo i due punti alla sua sezione su questa pagina, come [Usage limits](#usage-limits) o [Server errors](#server-errors), e seguite i passaggi di quella sezione
* Una volta che l'errore sottostante si risolve, chiedete a Claude di ritentare il compito o [riprendere il subagent](/docs/it/sub-agents#resume-subagents)

Quando un limite di velocità, un sovraccarico o un errore del server interrompe un subagent in primo piano che ha già prodotto un output di testo, Claude riceve quell'output parziale contrassegnato come incompleto invece di questo errore. Un subagent il cui unico output era chiamate di strumento riceve anche questo errore; nella v2.1.199 quella forma ha restituito un risultato parziale vuoto. Consultate [API errors in subagents](/docs/it/sub-agents#api-errors-in-subagents).

<h2 id="usage-limits">
  Limiti di utilizzo
</h2>

Questi errori significano che è stata raggiunta una quota associata al vostro account o piano. Sono distinti dagli [errori del server](#server-errors), che interessano tutti.

<h3 id="youve-hit-your-session-limit">
  Avete raggiunto il vostro limite di sessione
</h3>

I piani di abbonamento includono un'indennità di utilizzo mobile. Quando si esaurisce, vedete uno di questi messaggi:

```text theme={null}
You've hit your session limit · resets 3:45pm
You've hit your weekly limit · resets Mon 12:00am
You've hit your Opus limit · resets 3:45pm
```

Claude Code blocca ulteriori richieste fino all'ora di ripristino mostrata nel messaggio. I limiti di sessione e settimanali sono condivisi tra tutti i modelli, quindi il cambio di modello non ripristina l'accesso. Il limite di Opus si applica solo alle richieste di Opus, quindi il passaggio a un altro modello con `/model` vi permette di continuare a lavorare.

L'utilizzo conta contemporaneamente sulle indennità di sessione e settimanali. Un singolo picco di attività intensa, come un ampio fanout di flusso di lavoro, può esaurire l'indennità settimanale prima che la finestra di sessione si ripristini.

**Cosa fare:**

* Attendete l'ora di ripristino mostrata nell'errore
* Per il limite di Opus, eseguite `/model` e passate a un altro modello per continuare a lavorare
* Eseguite `/usage` per vedere i vostri limiti di piano e quando si ripristinano
* Eseguite `/usage-credits` per acquistare utilizzo aggiuntivo su Pro e Max, o per richiederlo al vostro amministratore su Team ed Enterprise. Consultate [usage credits per piani a pagamento](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) per informazioni su come viene fatturato.
* Per aggiornare il vostro piano per limiti di base più elevati, consultate [claude.com/pricing](https://claude.com/pricing)

Per monitorare l'indennità rimanente prima di raggiungere il limite, aggiungete i campi `rate_limits` a una [riga di stato personalizzata](/docs/it/statusline#rate-limit-usage), oppure nell'app Desktop fate clic sull'[anello di utilizzo](/docs/it/desktop#check-usage) accanto al selettore di modello.

<h3 id="usage-credits-required-for-1m-context">
  Crediti di utilizzo richiesti per il contesto 1M
</h3>

Il modello selezionato utilizza la finestra di contesto estesa da 1M token, e il vostro piano lo include solo tramite crediti di utilizzo.

```text theme={null}
API Error: Usage credits required for 1M context · run /usage-credits to turn them on, or /model to switch to standard context
```

Questo è un controllo di diritto, non un esaurimento della quota. Si attiva anche quando le vostre indennità di sessione e settimanali hanno capacità rimanente. Consultate [Extended context](/docs/it/model-config#extended-context) per sapere quali piani includono il contesto 1M direttamente e quali richiedono crediti di utilizzo.

Quando questo errore appare a metà conversazione perché il contesto è cresciuto oltre i 200K token, Claude Code compatta automaticamente la conversazione al di sotto del limite di contesto standard e mantiene la sessione a quel limite in seguito, quindi non è necessaria alcuna azione. Nelle versioni precedenti a v2.1.172, l'errore si ripeteva su ogni richiesta successiva incluso `/compact`; eseguite `/clear` su quelle versioni per recuperare. I passaggi seguenti si applicano quando avete esplicitamente selezionato un modello `[1m]`.

**Cosa fare:**

* Eseguite `/model` e selezionate la variante senza il suffisso `[1m]` per tornare alla finestra di contesto standard
* Eseguite `/usage-credits` per attivare la fatturazione a consumo per la variante 1M su Pro e Max, o per richiederla al vostro amministratore su Team ed Enterprise
* Se l'errore persiste dopo `/model`, un ID modello 1M potrebbe essere impostato altrove. Consultate [There's an issue with the selected model](#theres-an-issue-with-the-selected-model) per i percorsi di configurazione da controllare in ordine di priorità.
* Per rimuovere completamente le varianti 1M dal selettore di modello, impostate [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/it/env-vars)

<h3 id="server-is-temporarily-limiting-requests">
  Il server sta limitando temporaneamente le richieste
</h3>

L'API ha applicato un throttle di breve durata non correlato alla vostra quota di piano.

```text theme={null}
API Error: Server is temporarily limiting requests (not your usage limit)
```

Claude Code distingue questi dai vostri limiti di piano dall'assenza delle intestazioni di quota unificate che una vera risposta di limite contiene. A partire da v2.1.199 questo viene [ritentato automaticamente](#automatic-retries) con backoff prima di essere mostrato, indipendentemente da come vi autenticate. Nelle versioni precedenti, una sessione acceduta con un abbonamento claude.ai falliva il turno alla prima occorrenza; solo le autenticazioni con chiave API ed Enterprise lo ritentavano.

**Cosa fare:**

* Attendete brevemente e riprovate
* Controllate [status.claude.com](https://status.claude.com) se persiste

<h3 id="request-rejected-429">
  Richiesta rifiutata (429)
</h3>

Avete raggiunto il limite di velocità configurato per la vostra chiave API, il progetto Amazon Bedrock o il progetto Google Cloud.

```text theme={null}
API Error: Request rejected (429) · this may be a temporary capacity issue. If it persists, check https://status.claude.com.
```

La frase finale indica dove controllare l'integrità del servizio e varia in base al provider. Le configurazioni di Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry indicano lo stato del servizio di quel provider invece della pagina di stato di Anthropic. Un `ANTHROPIC_BASE_URL` personalizzato indica l'host del gateway.

**Cosa fare:**

* Eseguite `/status` e confermate che le credenziali attive siano quelle che vi aspettate. Un `ANTHROPIC_API_KEY` casuale nel vostro ambiente può instradare le richieste attraverso una chiave di livello inferiore invece del vostro abbonamento.
* Controllate la console del vostro provider per i limiti attivi e richiedete un livello superiore se necessario
* Per le chiavi API di Anthropic, consultate il [riferimento dei limiti di velocità](https://platform.claude.com/docs/en/api/rate-limits) per sapere come funzionano i livelli e come impostare i limiti di spesa per workspace
* Riducete la concorrenza: abbassate [`CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY`](/docs/it/env-vars), evitate di eseguire molti subagent paralleli, o passate a un modello più piccolo con `/model` per esecuzioni scriptate ad alto volume

<h3 id="credit-balance-is-too-low">
  Il saldo dei crediti è troppo basso
</h3>

La vostra organizzazione Console ha esaurito i crediti prepagati.

```text theme={null}
Credit balance is too low
```

**Cosa fare:**

* Aggiungete crediti su [platform.claude.com/settings/billing](https://platform.claude.com/settings/billing), e considerate di abilitare l'auto-reload lì in modo che il saldo si ricarichi prima di raggiungere lo zero
* Passate all'autenticazione con abbonamento con `/login` se avete un piano Pro, Max, Team o Enterprise
* Impostate i limiti di spesa per workspace nella Console per evitare che un singolo progetto esaurisca il saldo dell'organizzazione. Consultate [Manage costs effectively](/docs/it/costs).

<h2 id="authentication-errors">
  Errori di autenticazione
</h2>

Questi errori significano che Claude Code non può provare la vostra identità all'API. Eseguite `/status` in qualsiasi momento per vedere quale credenziale è attualmente attiva.

<h3 id="not-logged-in">
  Non connesso
</h3>

Nessuna credenziale valida è disponibile per questa sessione.

```text theme={null}
Not logged in · Please run /login
```

**Cosa fare:**

* Eseguite `/login` per autenticarvi con il vostro abbonamento Claude o account Console
* Se vi aspettavate che una variabile d'ambiente vi autenticasse, confermate che `ANTHROPIC_API_KEY` sia impostata ed esportata nella shell dove avete lanciato `claude`
* Per CI o automazione dove il login interattivo non è possibile, configurate uno script [`apiKeyHelper`](/docs/it/settings#available-settings) che recuperi una chiave all'avvio
* Consultate [Precedenza dell'autenticazione](/docs/it/authentication#authentication-precedence) per comprendere quale credenziale Claude Code utilizza quando sono presenti più credenziali

Se vi viene richiesto di accedere ripetutamente, consultate [Non connesso o token scaduto](/docs/it/troubleshoot-install#not-logged-in-or-token-expired) per le correzioni dell'orologio di sistema e del Portachiavi di macOS.

<h3 id="could-not-resolve-authentication-method">
  Impossibile risolvere il metodo di autenticazione
</h3>

La sessione ha raggiunto il client API senza alcuna credenziale. Questo appare nelle [sessioni in background](/docs/it/agent-view), sessioni cloud e contesti Agent SDK dove il controllo del login interattivo non viene eseguito prima della prima richiesta.

```text theme={null}
Could not resolve authentication method. Expected one of apiKey, authToken, credentials, config, or profile to be set. Or for one of the "X-Api-Key" or "Authorization" headers to be explicitly omitted
```

Prima della v2.1.174, una sessione in background o cloud assegnata a un worker pre-inizializzato inattivo poteva fallire in questo modo anche quando erano configurate credenziali valide. Eseguite l'aggiornamento per recuperare. Nelle versioni attuali l'errore significa che nessuna credenziale era disponibile per il processo worker.

**Cosa fare:**

* Eseguite l'aggiornamento alla v2.1.174 o successiva se questo appare in una sessione in background o cloud e le vostre credenziali sono già configurate
* Confermate che `ANTHROPIC_API_KEY`, `CLAUDE_CODE_OAUTH_TOKEN` o le credenziali del vostro provider cloud siano impostate nell'ambiente che avvia il worker, non solo nella vostra shell interattiva
* Per Agent SDK, consultate [configurazione dell'autenticazione](/docs/it/agent-sdk/overview#get-started)
* Eseguite `/status` in una sessione interattiva nello stesso ambiente per confermare quale fonte di credenziale si risolve

<h3 id="invalid-api-key">
  Chiave API non valida
</h3>

La variabile d'ambiente `ANTHROPIC_API_KEY` o lo script `apiKeyHelper` ha restituito una chiave che l'API ha rifiutato.

```text theme={null}
Invalid API key · Fix external API key
```

**Cosa fare:**

* Controllate gli errori di digitazione e confermate che la chiave non sia stata revocata nella [Console](https://platform.claude.com/settings/keys)
* Eseguite `env | grep ANTHROPIC` nella stessa shell. Strumenti come direnv, plugin shell dotenv e terminali IDE possono caricare una chiave obsoleta da un file `.env` nel vostro progetto senza che la impostiate esplicitamente.
* Annullate l'impostazione di `ANTHROPIC_API_KEY` ed eseguite `/login` per utilizzare invece l'autenticazione tramite abbonamento
* Se la chiave proviene da uno script [`apiKeyHelper`](/docs/it/settings#available-settings), eseguite lo script direttamente per confermare che stampi una chiave valida su stdout
* Eseguite `/status` per confermare quale fonte di credenziale Claude Code sta effettivamente utilizzando

<h3 id="your-apikeyhelper-script-is-failing">
  Lo script apiKeyHelper sta fallendo
</h3>

Il comando configurato nell'impostazione [`apiKeyHelper`](/docs/it/settings#available-settings) è uscito con un errore, ha raggiunto il timeout o non ha stampato nulla su stdout. Senza una chiave dallo script, la richiesta raggiunge l'API con una credenziale segnaposto, e l'API la rifiuta con `401`.

```text theme={null}
Your apiKeyHelper script is failing · This usually means you need to re-authenticate with your provider · Run /status to see the script's error output
```

Claude Code esegue nuovamente lo script e ritenta la richiesta fino a due volte in più prima di mostrare questo messaggio, quindi il fallimento emerge entro tre tentativi. Prima della v2.1.208, Claude Code spendeva l'intero [budget di retry](#automatic-retries) reinviando la richiesta con la credenziale segnaposto e poi segnalava un errore di autenticazione generico `401` invece del fallimento dello script.

Eseguire `/login` non aiuta qui: l'output dell'helper [ha la precedenza](/docs/it/authentication#authentication-precedence) su un login salvato finché l'impostazione è presente.

**Cosa fare:**

* Eseguite il comando configurato in `apiKeyHelper` direttamente nella vostra shell per riprodurre il fallimento
* Se il comando segnala una sessione scaduta, ri-autenticatevi con il vostro provider di credenziali, ad esempio accedendo di nuovo al vostro SSO o vault di segreti
* Correggete il comando in modo che stampi la chiave su stdout e esca con codice 0. Consultate [ruotare le credenziali con apiKeyHelper](/docs/it/llm-gateway-connect#rotate-credentials-with-apikeyhelper) per una configurazione funzionante.
* Eseguite `/status` per confermare che `apiKeyHelper` è la fonte di credenziale attiva. Ogni volta che il comando fallisce, il suo codice di uscita e l'output di errore appaiono in un pannello `Cloud authentication` nel terminale.

<h3 id="this-organization-has-been-disabled">
  Questa organizzazione è stata disabilitata
</h3>

Una `ANTHROPIC_API_KEY` obsoleta da un'organizzazione Console disabilitata sta sovrascrivendo il vostro login tramite abbonamento.

```text theme={null}
Your ANTHROPIC_API_KEY belongs to a disabled organization · Unset the environment variable to use your other credentials
API Error: 400 ... This organization has been disabled.
```

Le variabili d'ambiente hanno la precedenza su `/login`, quindi una chiave esportata nel vostro profilo shell o caricata da un file `.env` viene utilizzata anche quando avete un abbonamento Pro o Max funzionante. In modalità non interattiva (`-p`), la chiave viene sempre utilizzata quando presente.

**Cosa fare:**

* Annullate l'impostazione di `ANTHROPIC_API_KEY` nella shell corrente e rimuovetela dal vostro profilo shell, quindi riavviate `claude`
* Eseguite `/status` in seguito per confermare che la credenziale attiva sia il vostro abbonamento
* Se nessuna variabile d'ambiente è impostata e l'errore persiste, l'organizzazione disabilitata è quella collegata al vostro `/login`. Contattate il supporto o accedete con un account diverso.

<h3 id="your-organization-has-disabled-api-key-authentication">
  La vostra organizzazione ha disabilitato l'autenticazione tramite chiave API
</h3>

Questo messaggio richiede Claude Code v2.1.169 o successiva. L'amministratore dell'organizzazione Console ha disattivato l'autenticazione tramite chiave API, quindi l'API rifiuta la chiave che Claude Code sta inviando. L'hint di recupero dopo il `·` varia a seconda di dove proviene la chiave:

```text theme={null}
Your organization has disabled API key authentication · Run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY to use your claude.ai account instead
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY and run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset the apiKeyHelper setting and run /login to sign in with your claude.ai account
```

Le variabili d'ambiente e `apiKeyHelper` hanno la precedenza su `/login`, quindi eseguire solo `/login` non aiuta mentre uno di essi sta ancora fornendo una chiave. Consultate [Precedenza dell'autenticazione](/docs/it/authentication#authentication-precedence).

**Cosa fare:**

* Se il messaggio nomina `ANTHROPIC_API_KEY`, annullate l'impostazione nella shell corrente e rimuovetela dal vostro profilo shell o file `.env`, quindi riavviate `claude`
* Se il messaggio nomina `apiKeyHelper`, rimuovete l'impostazione [`apiKeyHelper`](/docs/it/settings#available-settings) dal vostro `settings.json`
* Eseguite `/login` per accedere con il vostro account claude.ai
* Eseguite `/status` in seguito per confermare che la credenziale attiva sia il vostro abbonamento piuttosto che una chiave API
* Se avete bisogno dell'autenticazione tramite chiave API per l'automazione, chiedete all'amministratore della vostra organizzazione di riattivarla nella Console

<h3 id="your-organization-has-disabled-claude-subscription-access">
  La vostra organizzazione ha disabilitato l'accesso tramite abbonamento Claude
</h3>

La vostra organizzazione Claude non consente l'accesso a Claude Code con un login tramite abbonamento. Eseguire `/login` di nuovo con lo stesso account restituisce lo stesso errore.

```text theme={null}
Your organization has disabled Claude subscription access for Claude Code · Use an Anthropic API key instead, or ask your admin to enable access
```

Questa è un'impostazione dell'organizzazione lato server, quindi non può essere sovrascritta dalle impostazioni locali, variabili d'ambiente o flag CLI.

Agent SDK e modalità non interattiva `-p` presentano questo come il codice di errore `oauth_org_not_allowed`.

**Cosa fare:**

* Chiedete al vostro amministratore di abilitare l'accesso a Claude Code per la vostra organizzazione
* Autenticatevi con una chiave API Console invece del vostro abbonamento. Consultate [Autenticazione Claude Console](/docs/it/authentication#claude-console-authentication) per la configurazione.
* Se siete l'amministratore e non vedete un'opzione per abilitare l'accesso, contattate il [supporto Anthropic](https://support.claude.com)

<h3 id="routines-are-disabled-by-your-organizations-policy">
  Le routine sono disabilitate dalla politica della vostra organizzazione
</h3>

Un Owner nella vostra organizzazione Team o Enterprise ha disattivato le routine a livello di organizzazione. L'errore appare quando tentate di creare o eseguire una routine, incluso da `/schedule` e dall'interfaccia utente [Routines](/docs/it/routines) su claude.ai/code.

```text theme={null}
Routines are disabled by your organization's policy.
```

Questa è un'impostazione lato server, quindi non può essere sovrascritta dalle impostazioni locali, variabili d'ambiente o flag CLI.

**Cosa fare:**

* Chiedete a un Owner nella vostra organizzazione di abilitare l'interruttore **Routines** su [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)
* Per lavori programmati una tantum che non richiedono routine a livello di organizzazione, consultate [attività programmate](/docs/it/scheduled-tasks)

<h3 id="remote-control-requires-the-anthropic-api">
  Remote Control richiede l'API Anthropic
</h3>

La sessione non sta comunicando direttamente con l'API Anthropic, quindi non c'è un backend claude.ai per [Remote Control](/docs/it/remote-control) con cui associarsi.

```text theme={null}
Remote Control is only available when using Claude via api.anthropic.com.
```

Questo appare su Amazon Bedrock, Agent Platform di Google Cloud e Microsoft Foundry. A partire dalla v2.1.196 appare anche quando [`ANTHROPIC_BASE_URL`](/docs/it/env-vars) punta a un host diverso da `api.anthropic.com`, come un [gateway LLM](/docs/it/llm-gateway) o proxy, anche quando vi accedete con claude.ai.

**Cosa fare:**

* Annullate l'impostazione di `ANTHROPIC_BASE_URL` e riavviate la sessione, oppure avviate Remote Control da una sessione che comunica direttamente con l'API Anthropic
* Per questo e gli altri messaggi di avvio di Remote Control, consultate [Risoluzione dei problemi di Remote Control](/docs/it/remote-control#troubleshooting)

<h3 id="oauth-token-revoked-or-expired">
  Token OAuth revocato o scaduto
</h3>

Il vostro login salvato non è più valido. Un token revocato significa che vi siete disconnessi ovunque o un amministratore ha rimosso l'accesso; un token scaduto significa che l'aggiornamento automatico ha fallito durante la sessione.

Entrambi i messaggi segnalano un rifiuto che l'API ha restituito per una richiesta che Claude Code ha inviato. Quando il login salvato è già stato cancellato dopo un aggiornamento fallito, vedete invece [Login scaduto](#login-expired).

```text theme={null}
OAuth token revoked · Please run /login
OAuth token has expired · Please run /login
API Error: 401 ... authentication_error
```

**Cosa fare:**

* Eseguite `/login` per accedere di nuovo
* Se l'errore ritorna nella stessa sessione dopo la ri-autenticazione, eseguite prima `/logout` per cancellare completamente il token memorizzato, quindi `/login`
* Per prompt ripetuti di accesso tra i lanci, consultate i controlli dell'orologio di sistema e del Portachiavi di macOS in [Risoluzione dei problemi](/docs/it/troubleshoot-install#not-logged-in-or-token-expired)
* Per altri errori inclusi `403 Forbidden` e problemi del browser OAuth, consultate [Login e autenticazione](/docs/it/troubleshoot-install#login-and-authentication)

<h3 id="login-expired">
  Login scaduto
</h3>

Claude Code ha tentato di rinnovare il vostro login salvato claude.ai o Claude Console e il servizio OAuth ha rifiutato il token di aggiornamento memorizzato, quindi Claude Code ha cancellato le credenziali salvate. Dopo di che, ogni richiesta si ferma localmente prima di raggiungere l'API, perché solo `/login` può creare nuove credenziali. Prima della v2.1.206, Claude Code inviava comunque la richiesta con qualsiasi credenziale rimanesse nell'ambiente, e ogni modello falliva poi con [C'è un problema con il modello selezionato](#theres-an-issue-with-the-selected-model) o un 401 invece di un prompt per accedere.

```text theme={null}
Login expired · Please run /login
```

In [modalità non interattiva](/docs/it/headless) (`-p`) e [Agent SDK](/docs/it/agent-sdk/overview), il messaggio legge come segue, e il codice di errore strutturato è `authentication_failed`:

```text theme={null}
Failed to authenticate: OAuth session expired and could not be refreshed
```

Questo non è lo stesso stato di [Token OAuth revocato o scaduto](#oauth-token-revoked-or-expired). Quei messaggi segnalano un 401 che l'API ha restituito. Claude Code stesso produce `Login expired` per un login che ha già fallito a rinnovare, quindi non invia alcuna richiesta.

Le sessioni autenticate con una chiave API, [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/it/env-vars) o un provider di terze parti non utilizzano il login salvato e non vedono mai questo messaggio.

**Cosa fare:**

* Eseguite `/login` per accedere di nuovo. Riprovare senza accedere mostra lo stesso messaggio su ogni richiesta.
* In modalità non interattiva, eseguite `claude` nello stesso ambiente, completate `/login`, quindi rieseguite il vostro comando. Per l'automazione che non può accedere in modo interattivo, autenticatevi con `ANTHROPIC_API_KEY` o [generate un token di lunga durata con `claude setup-token`](/docs/it/authentication#generate-a-long-lived-token).
* Se l'accesso continua a fallire, consultate [Login e autenticazione](/docs/it/troubleshoot-install#login-and-authentication)

<h3 id="oauth-scope-requirement">
  Requisito di ambito OAuth
</h3>

Il token memorizzato precede un ambito di autorizzazione che una funzione più recente necessita. Lo vedete più spesso da `/usage` e dall'indicatore di utilizzo della riga di stato:

```text theme={null}
OAuth token does not meet scope requirement: user:profile
```

**Cosa fare:**

* Eseguite `/login` per ottenere un nuovo token con gli ambiti attuali. Non è necessario disconnettervi prima.

<h3 id="aws-credentials-expired-or-invalid">
  Credenziali AWS scadute o non valide
</h3>

Questo messaggio richiede Claude Code v2.1.198 o successiva e appare solo quando [`awsAuthRefresh`](/docs/it/amazon-bedrock#advanced-credential-configuration) è impostato nel vostro file di impostazioni. Il vostro token di sessione AWS è scaduto o è stato rifiutato, e l'aggiornamento automatico che Claude Code ha già eseguito non ha prodotto una credenziale che l'API accetta. Appare su un 401 da [Claude Platform on AWS](/docs/it/claude-platform-on-aws) o dall'[endpoint Mantle](/docs/it/amazon-bedrock#use-the-mantle-endpoint), che è come questi provider segnalano un token di sicurezza scaduto.

L'hint di azione nel mezzo nomina il comando `awsAuthRefresh` dalle vostre impostazioni, quindi varia. La parte stabile è il `AWS credentials expired or invalid` iniziale:

```text theme={null}
AWS credentials expired or invalid · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · API Error: 401 ...
```

Senza `awsAuthRefresh` configurato, lo stesso 401 mostra il messaggio generico `Please run /login` invece, che non può aggiornare le credenziali AWS.

**Cosa fare:**

* Eseguite il comando `awsAuthRefresh` nominato nel messaggio, come `aws sso login --profile myprofile`, in un altro terminale e completate l'accesso al browser, quindi riprovate
* In una sessione interattiva, eseguite `/login`, scegliete **3rd-party platform**, quindi selezionate **Claude Platform on AWS · refresh credentials** sotto **Using 3rd-party platforms** per eseguire lo stesso comando senza riavviare Claude Code. Consultate [Configurare le credenziali AWS](/docs/it/claude-platform-on-aws#1-configure-aws-credentials)
* Se l'errore si ripete dopo che il comando di aggiornamento ha avuto successo, confermate che l'identità sia valida al di fuori di Claude Code con `aws sts get-caller-identity` nella stessa shell e profilo

<h3 id="aws-authentication-failed">
  Autenticazione AWS non riuscita
</h3>

Questo messaggio richiede Claude Code v2.1.198 o successiva e appare solo quando [`awsAuthRefresh`](/docs/it/amazon-bedrock#advanced-credential-configuration) è impostato nel vostro file di impostazioni. Il vostro provider AWS ha restituito un 403, oppure [Amazon Bedrock](/docs/it/amazon-bedrock) ha restituito un 401.

Claude Code non può dire quale causa avete riscontrato. Amazon Bedrock segnala un token di sicurezza scaduto come un 403, ma un 403 è anche come segnala un diniego di autorizzazione, come un `AccessDeniedException` da un'autorizzazione IAM mancante o un modello che non è abilitato per il vostro account.

Un 401 da Amazon Bedrock finisce anche qui piuttosto che sotto [Credenziali AWS scadute o non valide](#aws-credentials-expired-or-invalid), perché Amazon Bedrock non segnala un token scaduto come un 401. Un 401 da quell'endpoint in genere proviene da qualcos'altro nel percorso della richiesta, come un proxy aziendale.

Un aggiornamento delle credenziali corregge un token scaduto e non può correggere le altre cause, quindi il messaggio offre entrambe:

```text theme={null}
AWS authentication failed · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · if credentials are current, check AWS permissions and model access · API Error: 403 ...
```

L'hint di azione nel mezzo nomina il comando `awsAuthRefresh` dalle vostre impostazioni, quindi varia. La parte stabile è il `AWS authentication failed` iniziale.

**Cosa fare:**

* Eseguite il comando `awsAuthRefresh` nominato nel messaggio, o `aws sso login`, nel caso in cui una credenziale scaduta sia la causa
* Se le vostre credenziali sono attuali, confermate che le autorizzazioni IAM in [Configurazione IAM](/docs/it/amazon-bedrock#iam-configuration) siano allegate all'identità che state utilizzando e che il modello selezionato sia abilitato per il vostro account e regione
* Eseguite `aws sts get-caller-identity` per confermare quale identità utilizzano le vostre richieste; un `AWS_PROFILE` obsoleto o profilo predefinito è una causa comune di una mancata corrispondenza di autorizzazioni

<h3 id="aws-default-chain-credential-resolve-timed-out">
  Risoluzione della credenziale della catena predefinita AWS scaduta
</h3>

Il provider di credenziali della catena predefinita AWS non ha prodotto credenziali entro 60 secondi, quindi Claude Code ha fermato la risoluzione e ha fallito la richiesta. Il fallimento è la risoluzione locale delle credenziali: la richiesta non ha mai raggiunto [Amazon Bedrock](/docs/it/amazon-bedrock), [Claude Platform on AWS](/docs/it/claude-platform-on-aws) o l'[endpoint Mantle](/docs/it/amazon-bedrock#use-the-mantle-endpoint). Claude Code cancella la [cache delle credenziali](/docs/it/amazon-bedrock#credential-caching-and-resolution-timeout) e ritenta prima che questo errore emerga, quindi nel momento in cui lo vedete la catena si è bloccata su tentativi ripetuti.

```text theme={null}
API Error: AWS default-chain credential resolve timed out
```

Le cause comuni sono un comando `credential_process` nel vostro profilo AWS che attende un input che non può ricevere, e un container o VM il cui servizio di metadati dell'istanza (IMDS) non risponde mai al probe della catena. Prima della v2.1.207, una catena bloccata lasciava la richiesta in attesa indefinitamente invece di fallire con questo messaggio.

**Cosa fare:**

* Eseguite `aws sts get-caller-identity` nella stessa shell con lo stesso `AWS_PROFILE`. Se si blocca anche, correggete il profilo; un comando `credential_process` che richiede in modo interattivo è una causa comune.
* Completate il passaggio di accesso prima di avviare Claude Code, ad esempio `aws sso login --profile myprofile`, in modo che la catena si risolva dalla cache SSO locale invece di attendere un flusso del browser
* Se la vostra catena esegue un accesso interattivo che legittimamente necessita di più di 60 secondi, come SSO con MFA attraverso un wrapper come `aws-vault`, aumentate il limite in millisecondi con [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/it/env-vars)

<h2 id="network-and-connection-errors">
  Errori di rete e connessione
</h2>

Questi errori significano che una richiesta di rete da Claude Code non ha raggiunto la sua destinazione, o qualcosa tra Claude Code e l'API ha alterato la risposta durante il percorso di ritorno. Di solito originano dalla vostra rete locale, proxy o firewall, oppure dalla politica di rete dell'ambiente cloud.

<h3 id="unable-to-connect-to-api">
  Impossibile connettersi all'API
</h3>

La connessione TCP all'API non è riuscita o non si è mai completata.

```text theme={null}
Unable to connect to API. Check your internet connection
Unable to connect to API (ECONNREFUSED)
Unable to connect to API (ECONNRESET)
Unable to connect to API (ETIMEDOUT)
fetch failed
Request timed out. Check your internet connection and proxy settings
```

Le cause comuni includono l'assenza di accesso a Internet, una VPN che blocca `api.anthropic.com`, o un proxy aziendale richiesto che non è configurato.

**Cosa fare:**

* Confermate di poter raggiungere l'host dell'API dalla stessa shell eseguendo `curl -I https://api.anthropic.com`. Su Windows PowerShell utilizzate `curl.exe -I https://api.anthropic.com` in modo che l'alias `Invoke-WebRequest` integrato non venga utilizzato.
* Se siete dietro un proxy aziendale, impostate `HTTPS_PROXY` prima di avviare Claude Code e consultate [Configurazione di rete](/docs/it/network-config)
* Se instradiate il traffico attraverso un gateway LLM o un relay, impostate [`ANTHROPIC_BASE_URL`](/docs/it/env-vars) al suo indirizzo. Consultate [Connettere Claude Code a un gateway LLM](/docs/it/llm-gateway-connect) per la configurazione.
* Assicuratevi che il vostro firewall consenta gli host elencati in [Requisiti di accesso di rete](/docs/it/network-config#network-access-requirements)
* I guasti intermittenti vengono [ritentati automaticamente](#automatic-retries); i guasti persistenti indicano un problema di rete locale

Se `curl` ha successo ma Claude Code continua a fallire, la causa è solitamente qualcosa tra il runtime e la rete piuttosto che la rete stessa:

* Su Linux e WSL, controllate `/etc/resolv.conf` per un nameserver non raggiungibile. WSL in particolare può ereditare un resolver rotto dall'host.
* Su macOS, un client VPN che è stato disconnesso o disinstallato può lasciare dietro un'interfaccia tunnel o una regola di routing. Controllate `ifconfig` per interfacce `utun` obsolete e rimuovete l'estensione di rete della VPN in Impostazioni di Sistema.
* Docker Desktop e runtime di container simili possono intercettare il traffico in uscita. Chiudeteli e ritentate per escludere questa possibilità.

<h3 id="bedrock-streaming-response-has-an-unexpected-content-type">
  La risposta di streaming di Bedrock ha un content-type inaspettato
</h3>

Un gateway o un proxy tra Claude Code e [Amazon Bedrock](/docs/it/amazon-bedrock) sta trasformando il corpo della risposta di streaming o la sua intestazione `Content-Type`. Amazon Bedrock trasmette le risposte come `application/vnd.amazon.eventstream`, e Claude Code rifiuta una risposta di streaming riuscita che segnala un content-type diverso invece di decodificare un corpo che non può leggere. La richiesta non viene ritentata.

```text theme={null}
Bedrock streaming response has content-type "text/event-stream"; expected "application/vnd.amazon.eventstream". A gateway or proxy between Claude Code and Bedrock is likely transforming the response body — Bedrock's binary event-stream format must be passed through unmodified. Set CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1 to suppress this check while the gateway is being fixed.
```

Prima della v2.1.208, la stessa configurazione errata si presentava come `API Error: Truncated event message received` dopo che l'intera risposta era stata memorizzata nel buffer.

**Cosa fare:**

* Configurate il gateway per passare il corpo della risposta `InvokeModelWithResponseStream` e la sua intestazione `Content-Type` senza modifiche. Un intermediario che riemette il flusso come server-sent events è una causa comune.
* Se il gateway riscrive solo l'intestazione e passa il corpo binario intatto, impostate [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1`](/docs/it/env-vars) per saltare il controllo fino a quando il gateway non viene corretto. Consultate [Errori di streaming dietro un gateway o un proxy](/docs/it/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy).

<h3 id="ssl-certificate-errors">
  Errori di certificato SSL
</h3>

Un proxy o un'appliance di sicurezza sulla vostra rete sta intercettando il traffico TLS con il proprio certificato, e Claude Code non lo considera attendibile.

```text theme={null}
Unable to connect to API: SSL certificate verification failed. Check your proxy or corporate SSL certificates
Unable to connect to API: Self-signed certificate detected
```

A partire dalla v2.1.199, un guasto di convalida del certificato non viene ritentato, quindi questo errore appare al primo tentativo invece che dopo il completo [budget di retry](#automatic-retries). Le versioni precedenti hanno speso alcuni minuti ritentando prima di mostrarlo. Le condizioni TLS transitorie, come un timeout di handshake, continuano a essere ritentate.

Durante `/login` e il controllo di connettività all'avvio, lo stesso guasto viene segnalato con il codice OpenSSL e la correzione inline:

```text theme={null}
SSL certificate error (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). If you are behind a corporate proxy or TLS-intercepting firewall, set NODE_EXTRA_CA_CERTS to your CA bundle path, or ask IT to allowlist *.anthropic.com. Run `claude doctor` for details.
```

**Cosa fare:**

* Esportate il bundle CA della vostra organizzazione e puntate Claude Code ad esso con `NODE_EXTRA_CA_CERTS=/path/to/ca-bundle.pem`
* Consultate [Configurazione di rete](/docs/it/network-config#custom-ca-certificates) per le istruzioni di configurazione complete
* Non impostate `NODE_TLS_REJECT_UNAUTHORIZED=0`, che disabilita completamente la convalida del certificato

<h3 id="host-not-allowed-in-a-cloud-session">
  Host non consentito in una sessione cloud
</h3>

Una richiesta HTTP in uscita da una sessione cloud o routine è stata bloccata dalla politica di rete dell'ambiente.

```text theme={null}
HTTP 403
x-deny-reason: host_not_allowed
```

Potete anche vedere un certificato TLS che non corrisponde al certificato reale della destinazione. L'ambiente cloud instrada il traffico in uscita attraverso un proxy che applica la politica di rete, quindi un certificato non corrispondente significa che il proxy ha terminato la connessione, non la destinazione.

Questo non è un problema di rete lato client. Le sessioni cloud e le [routine](/docs/it/routines) vengono eseguite all'interno di un ambiente sandbox il cui traffico in uscita è filtrato in base all'allowlist dell'ambiente. L'ambiente **Default** utilizza l'accesso **Trusted**, che consente l'[allowlist predefinito](/docs/it/claude-code-on-the-web#default-allowed-domains) dei registri di pacchetti, API dei provider cloud, registri di container e domini di sviluppo comuni, ma blocca tutto il resto.

**Cosa fare:**

* Aprite la routine per la modifica, o avviate una sessione cloud. Selezionate l'icona cloud che mostra il nome del vostro ambiente, come **Default**, per aprire il selettore. Passate il mouse sopra il vostro ambiente e fate clic sull'icona delle impostazioni.
* Nella finestra di dialogo **Update cloud environment**, cambiate **Network access** da **Trusted** a **Custom**, quindi aggiungete il dominio bloccato a **Allowed domains**. Inserite un dominio per riga. Selezionate **Also include default list of common package managers** per mantenere l'[allowlist predefinito](/docs/it/claude-code-on-the-web#default-allowed-domains) insieme ai vostri domini personalizzati. Selezionate **Full** invece se desiderate un accesso senza restrizioni.
* Fate clic su **Save changes**. L'esecuzione successiva utilizza l'allowlist aggiornato.

Consultate [Accesso di rete](/docs/it/claude-code-on-the-web#network-access) per i livelli di accesso e l'allowlist predefinito. Le sessioni CLI locali non sono interessate da questa politica.

<h3 id="couldnt-reconnect-to-your-remote-control-session">
  Impossibile riconnettersi alla sessione Remote Control
</h3>

```text theme={null}
Couldn't reconnect to your Remote Control session. Retry, or start a fresh session without --resume.
```

La ripresa con `claude --resume` o `claude --continue` si ricollega alla sessione [Remote Control](/docs/it/remote-control) registrata in quella conversazione. Questo messaggio significa che la riconnessione non è riuscita per un motivo che potrebbe essere temporaneo, come un'interruzione di rete o un errore del server, quindi Claude Code non può confermare se la sessione remota esiste ancora. La vostra sessione locale continua a funzionare senza Remote Control.

**Cosa fare:**

* Eseguite `/remote-control` per ritentare la connessione
* Avviate Claude Code senza `--resume` per creare una nuova sessione Remote Control
* Per altri messaggi di avvio di Remote Control, consultate [Risoluzione dei problemi di Remote Control](/docs/it/remote-control#troubleshooting)

Non vedrete questo messaggio quando il server conferma che la sessione precedente non esiste più; Claude Code ne crea una nuova in quel caso. Prima della v2.1.200, qualsiasi guasto di riconnessione creava una nuova sessione Remote Control, il che lasciava sessioni extra nell'elenco delle sessioni su claude.ai/code.

<h2 id="request-errors">
  Errori di richiesta
</h2>

Questi errori riguardano il contenuto della tua richiesta. La maggior parte proviene dall'API dopo che ha rifiutato la richiesta; alcuni sono prodotti localmente da Claude Code prima che venga inviata qualsiasi richiesta.

<h3 id="prompt-is-too-long">
  Il prompt è troppo lungo
</h3>

La conversazione più i file allegati superano la finestra di contesto del modello.

```text theme={null}
Prompt is too long
```

**Cosa fare:**

* Esegui `/compact` per riassumere i turni precedenti e liberare spazio, oppure `/clear` per ricominciare da capo
* Esegui `/context` per visualizzare una suddivisione di ciò che consuma la finestra: prompt di sistema, strumenti, file di memoria e messaggi
* Disabilita i server MCP che non stai utilizzando con `/mcp disable <name>` per rimuovere le loro definizioni di strumenti dal contesto
* Riduci i file di memoria `CLAUDE.md` di grandi dimensioni, oppure sposta le istruzioni in [regole con ambito di percorso](/docs/it/memory#path-specific-rules) che si caricano solo quando rilevanti
* I subagent ereditano ogni definizione di strumento MCP dalla sessione padre, il che può riempire la loro finestra di contesto prima del primo turno. Disabilita i server MCP che non stai utilizzando prima di generare subagent.
* L'auto-compact è abilitato per impostazione predefinita e normalmente previene questo errore. Se hai impostato [`DISABLE_AUTO_COMPACT`](/docs/it/env-vars), riabilitalo o esegui `/compact` manualmente prima che la finestra si riempia.

Vedi [Esplora la finestra di contesto](/docs/it/context-window) per una visualizzazione interattiva di come il contesto si riempie.

<h3 id="error-during-compaction-conversation-too-long">
  Errore durante la compattazione: Conversazione troppo lunga
</h3>

`/compact` stesso ha avuto esito negativo perché non c'è abbastanza contesto libero per contenere il riassunto che produce.

```text theme={null}
Error during compaction: Conversation too long. Press esc twice to go up a few messages and try again.
```

Questo può accadere quando la finestra è già piena nel momento in cui auto-compact si attiva, oppure quando esegui `/compact` dopo aver visto `Prompt is too long`.

**Cosa fare:**

* Premi Esc due volte per aprire l'elenco dei messaggi e tornare indietro di diversi turni. Questo elimina i messaggi più recenti dal contesto. Quindi esegui `/compact` di nuovo.
* Se tornare indietro non libera abbastanza spazio, esegui `/clear` per avviare una sessione nuova. La tua conversazione precedente viene preservata e può essere riaperta con `/resume`.

<h3 id="request-too-large">
  Richiesta troppo grande
</h3>

Il corpo della richiesta grezza ha superato il limite di byte dell'API prima della tokenizzazione, di solito a causa di un file incollato di grandi dimensioni o di un allegato.

```text theme={null}
Request too large (max 30 MB). Double press esc to go back and remove or shrink the attached content.
```

Questo è un limite di dimensione sulla richiesta HTTP, separato dal [limite della finestra di contesto](#prompt-is-too-long).

**Cosa fare:**

* Premi Esc due volte e torna indietro oltre il turno che ha aggiunto il contenuto di dimensioni eccessive
* Fai riferimento ai file di grandi dimensioni per percorso invece di incollarne i contenuti, in modo che Claude possa leggerli in blocchi
* Per le immagini, vedi [L'immagine era troppo grande](#image-was-too-large) di seguito

<h3 id="image-was-too-large">
  L'immagine era troppo grande
</h3>

Un'immagine incollata o allegata supera i limiti di dimensione o dimensione dell'API.

```text theme={null}
Image was too large. Double press esc to go back and try again with a smaller image.
API Error: 400 ... image dimensions exceed max allowed size
```

Claude Code sostituisce l'immagine non elaborabile con un segnaposto di testo e riprova, quindi i messaggi successivi hanno esito positivo. Nelle versioni precedenti a 2.1.142, un'immagine incollata potrebbe rimanere nella conversazione e ripetere lo stesso errore su ogni messaggio successivo. Per recuperare su quelle versioni, premi Esc due volte e torna indietro oltre il turno in cui è stata aggiunta l'immagine.

**Cosa fare:**

* Ridimensiona l'immagine prima di incollarla. L'API accetta immagini fino a 8000 pixel sul bordo più lungo per una singola immagine, o 2000 pixel quando molte immagini sono nel contesto.
* Fai uno screenshot più stretto della regione rilevante invece dello schermo intero

<h3 id="unable-to-resize-image">
  Impossibile ridimensionare l'immagine
</h3>

Claude Code non ha potuto ridimensionare un'immagine allegata prima di inviarla all'API.

```text theme={null}
Unable to resize image — image processing is unavailable and dimensions could not be read from the file header. Please convert the image to PNG, JPEG, GIF, or WebP.
Unable to resize image — dimensions exceed the 2000x2000px limit and image processing failed. Please resize the image to reduce its pixel dimensions.
Unable to resize image (… raw, … base64). The image exceeds the … API limit and compression failed. Please resize the image manually or use a smaller image.
Unable to resize image — could not verify image dimensions are within the 2000x2000px API limit.
```

Claude Code normalmente ridimensiona automaticamente le immagini di grandi dimensioni. Questi errori significano che il processore di immagini nativo non ha potuto caricarsi o ha restituito un errore, quindi l'immagine non potrebbe essere ridimensionata per rientrare nei limiti dell'API.

**Cosa fare:**

* Se il messaggio ti chiede di convertire l'immagine, convertila in PNG, JPEG, GIF o WebP e allegala di nuovo. Claude Code può verificare le dimensioni per questi formati senza il processore di immagini.
* Se il messaggio segnala un limite di dimensione o dimensione, ridimensiona o ricomprimi l'immagine al di sotto di quel limite prima di allegare.

<h3 id="pdf-errors">
  Errori PDF
</h3>

Il PDF che hai allegato non potrebbe essere elaborato.

```text theme={null}
PDF too large (max 100 pages, 32 MB). Try splitting it or extracting text first.
PDF is password protected. Try removing protection or extracting text first.
The PDF file was not valid. Try converting to a different format first.
```

**Cosa fare:**

* Per i PDF di grandi dimensioni, chiedi a Claude di leggere un intervallo di pagine con lo strumento Read invece di allegare l'intero file, oppure estrai il testo con uno strumento come `pdftotext` e fai riferimento al file di output per percorso
* Per i PDF protetti o non validi, rimuovi la password o riesporta il file dall'applicazione sorgente, quindi riprova

<h3 id="extra-inputs-are-not-permitted">
  Gli input aggiuntivi non sono consentiti
</h3>

Un proxy o un gateway LLM tra Claude Code e l'API ha rimosso l'intestazione della richiesta `anthropic-beta`, quindi l'API ha rifiutato i campi che dipendono da essa.

```text theme={null}
API Error: 400 ... Extra inputs are not permitted ... context_management
API Error: 400 ... Extra inputs are not permitted ... tools.0.custom.input_examples
API Error: 400 ... Unexpected value(s) for the `anthropic-beta` header
```

Claude Code invia campi solo beta come `context_management`, `effort` e `input_examples` dello strumento insieme a un'intestazione `anthropic-beta` che li abilita. Quando un gateway inoltra il corpo ma elimina l'intestazione, l'API vede campi che non riconosce.

**Cosa fare:**

* Configura il tuo gateway per inoltrare l'intestazione `anthropic-beta`. Vedi [feature pass-through](/docs/it/llm-gateway-protocol#feature-pass-through) per ciò che i gateway devono inoltrare.
* Come fallback, imposta [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/it/env-vars) prima di avviare. Questo disabilita le funzioni che richiedono l'intestazione beta in modo che le richieste abbiano esito positivo attraverso un gateway che non può inoltrarla.

<h3 id="theres-an-issue-with-the-selected-model">
  C'è un problema con il modello selezionato
</h3>

Il nome del modello configurato non è stato riconosciuto o il tuo account non ha accesso ad esso. A partire da v2.1.160 il suggerimento finale, mostrato qui nella sua forma interattiva, varia in base alla superficie.

```text theme={null}
There's an issue with the selected model (claude-...). It may not exist or you may not have access to it. Run /model to pick a different model.
```

**Cosa fare:**

* **CLI interattiva**: esegui `/model` per scegliere dai modelli disponibili per il tuo account.
* **Modalità non interattiva (`-p`)**: passa `--model` con un alias o ID valido, oppure imposta [`ANTHROPIC_MODEL`](/docs/it/env-vars). Il testo di errore mostra `Run --model` su questa superficie.
* **Agent SDK**: il testo di errore omette il suggerimento perché il modello è impostato a livello di programmazione. Imposta [`model` su `Options`](/docs/it/agent-sdk/typescript#options) in TypeScript o [`ClaudeAgentOptions(model=...)`](/docs/it/agent-sdk/python#claudeagentoptions) in Python, e gestisci l'errore strutturato `model_not_found` per visualizzare il tuo ritentativo o selettore di modello.
* Usa un alias come `sonnet` o `opus` invece di un ID completamente versionato. Gli alias si risolvono in un valore predefinito mantenuto in modo che non diventino obsoleti. Vedi [Configurazione del modello](/docs/it/model-config).
* Se il modello sbagliato continua a tornare nella CLI, un ID obsoleto è impostato da qualche parte. Controlla in [ordine di priorità](/docs/it/model-config#setting-your-model): il flag `--model`, la variabile di ambiente `ANTHROPIC_MODEL`, quindi il campo `model` in `.claude/settings.local.json`, il `.claude/settings.json` del tuo progetto e `~/.claude/settings.json`. Rimuovi il valore obsoleto e Claude Code torna al valore predefinito del tuo account.
* Claude Code segnala un accesso a claude.ai scaduto come [Accesso scaduto](#login-expired), non come questo errore. Prima di v2.1.206, un accesso scaduto che non poteva più essere aggiornato ha avuto esito negativo con ogni modello con questo errore; esegui `/login` se vedi questo su una versione precedente.
* Per le distribuzioni di Google Cloud's Agent Platform, vedi [Risoluzione dei problemi di Google Cloud's Agent Platform](/docs/it/google-vertex-ai#troubleshooting).

<h3 id="model-is-not-a-recognized-model-id">
  Il modello non è un ID modello riconosciuto
</h3>

La stringa del modello che hai passato a un cambio di modello non è un alias di modello, un ID di modello che questa versione di Claude Code conosce, o un ID che inizia con `claude-`. Le cause comuni sono un errore di battitura nell'ID, un nome visualizzato come `Sonnet 5` dove è previsto l'ID `claude-sonnet-5`, o un alias che solo le versioni più recenti di Claude Code riconoscono. Claude Code rifiuta il cambio immediatamente. Prima di v2.1.200, Claude Code salvava la stringa e aveva esito negativo sulla richiesta successiva con [C'è un problema con il modello selezionato](#theres-an-issue-with-the-selected-model).

```text theme={null}
Model "claud-sonnet-5" is not a recognized model id. Did you mean 'claude-sonnet-5'?
```

Il suggerimento finale nomina l'alias o l'ID del modello più vicino. Quando nulla è abbastanza vicino, legge `Run /model to see available models.` invece.

Claude Code produce questo errore localmente nel momento in cui il cambio è richiesto, prima che venga effettuata qualsiasi richiesta API. Si applica quando un modello è impostato tramite il metodo [Agent SDK](/docs/it/agent-sdk/typescript) `setModel()` o da un'app come l'[app Desktop](/docs/it/desktop) che esegue la CLI di Claude Code per te.

**Cosa fare:**

* Esegui `/model` senza argomenti per aprire il selettore e scegli dai modelli disponibili per il tuo account, quindi passa l'alias o l'ID mostrato lì
* Se hai utilizzato un alias che una versione più recente di Claude Code supporta, esegui `claude update`. Un ID completo che inizia con `claude-` passa questo controllo anche quando il modello è più recente della tua versione di Claude Code, quindi l'aggiornamento non è necessario per quelli.
* Un modello salvato prima di v2.1.200 non viene riparato da questo controllo. Se un valore obsoleto continua a tornare, rimuovilo dalle posizioni elencate in [C'è un problema con il modello selezionato](#theres-an-issue-with-the-selected-model).
* Il controllo viene eseguito solo sull'API Anthropic. Su Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, [Claude Platform on AWS](/docs/it/claude-platform-on-aws) e dietro un [gateway LLM](/docs/it/llm-gateway) o un `ANTHROPIC_BASE_URL` personalizzato, il tuo provider o gateway definisce i nomi dei modelli, quindi Claude Code accetta qualsiasi stringa e la passa attraverso.

<h3 id="claude-opus-is-not-available-with-the-claude-pro-plan">
  Claude Opus non è disponibile con il piano Claude Pro
</h3>

Il tuo piano di abbonamento attivo non include il modello che hai selezionato.

```text theme={null}
Claude Opus is not available with the Claude Pro plan · Select a different model in /model
```

**Cosa fare:**

* Esegui `/model` e seleziona un modello che il tuo piano include
* Se hai aggiornato il tuo piano di recente e vedi ancora questo, esegui `/logout` quindi `/login`. Il token memorizzato riflette il tuo piano al momento dell'accesso, quindi l'aggiornamento sul web non ha effetto in una sessione esistente fino a quando non ti autentica di nuovo.
* Vedi [claude.com/pricing](https://claude.com/pricing) per quali modelli ogni piano include

<h3 id="model-is-restricted-by-your-organizations-settings">
  Il modello è limitato dalle impostazioni della tua organizzazione
</h3>

L'amministratore della tua organizzazione ha disabilitato questo modello nella console di amministrazione di claude.ai, oppure è escluso da un elenco di autorizzazioni [`availableModels`](/docs/it/model-config#restrict-model-selection) nelle impostazioni gestite. Quando il modello limitato è stato impostato con `--model`, `ANTHROPIC_MODEL` o l'impostazione `model`, Claude Code sostituisce un modello consentito e continua. Digitare `/model <name>` per un modello limitato viene rifiutato con `Run /model to choose a different model.` e la sessione mantiene il suo modello attuale.

```text theme={null}
Model "claude-opus-4-8" is restricted by your organization's settings. Using claude-sonnet-4-6 instead.
```

Claude Code tratta un alias della famiglia di modelli, uno di `opus`, `sonnet`, `haiku` o `fable`, come una richiesta per quella famiglia piuttosto che per la sua versione più recente. Sull'API Anthropic e su [Claude Platform on AWS](/docs/it/claude-platform-on-aws), un alias della famiglia limitata si risolve nella versione più recente della famiglia che la tua organizzazione e l'elenco di autorizzazioni `availableModels` consentono, e l'avviso di sostituzione nomina quella versione. Claude Code rifiuta `/model <alias>` solo quando ogni versione della famiglia è limitata. Prima di v2.1.205, un alias della famiglia veniva sostituito o rifiutato in base alla sua versione più recente sola, anche quando una versione precedente della stessa famiglia era consentita.

**Cosa fare:**

* Esegui `/model` per scegliere dai modelli che la tua organizzazione consente. I modelli limitati sono nascosti dal selettore.
* Se il modello limitato è stato impostato in `--model`, `ANTHROPIC_MODEL` o il campo `model` di un file di impostazioni, rimuovi o aggiorna quel valore in modo che l'avviso non si ripeta ad ogni avvio
* Se hai bisogno di accesso al modello limitato, chiedi all'amministratore della tua organizzazione di abilitarlo. Vedi [Restrizioni del modello dell'organizzazione](/docs/it/model-config#organization-model-restrictions).

<h3 id="thinking-type-enabled-is-not-supported-for-this-model">
  thinking.type.enabled non è supportato per questo modello
</h3>

La tua versione di Claude Code è più vecchia del minimo per Sonnet 5, Opus 4.8 o Opus 4.7. La CLI ha inviato una configurazione di thinking che il modello non accetta più.

```text theme={null}
API Error: 400 ... "thinking.type.enabled" is not supported for this model. Use "thinking.type.adaptive" and "output_config.effort" to control thinking behavior.
```

**Cosa fare:**

* Esegui `claude update` e riavvia Claude Code. Opus 4.7 ha bisogno di v2.1.111 o successivo. Opus 4.8 ha bisogno di v2.1.154 o successivo. Sonnet 5 ha bisogno di v2.1.197 o successivo
* Se non puoi aggiornare, esegui `/model` e seleziona Opus 4.6 o Sonnet 4.6 invece
* Se lo riscontri nell'[Agent SDK](/docs/it/agent-sdk/overview), aggiorna il pacchetto SDK invece. Opus 4.8 ha bisogno di TypeScript SDK v0.3.154 o successivo e Python SDK v0.2.88 o successivo. Sonnet 5 ha bisogno di TypeScript SDK v0.3.197 o successivo

<h3 id="thinking-budget-exceeds-output-limit">
  Il budget di thinking supera il limite di output
</h3>

Il budget di thinking esteso configurato supera la lunghezza massima della risposta, quindi non c'è spazio rimasto per la risposta effettiva.

```text theme={null}
API Error: 400 ... max_tokens must be greater than thinking.budget_tokens
```

Claude Code regola questi valori automaticamente sull'API Anthropic. Normalmente vedi questo errore su Amazon Bedrock o Google Cloud's Agent Platform quando [`MAX_THINKING_TOKENS`](/docs/it/env-vars) è impostato più alto del limite di output del provider, o quando la modalità piano aumenta il budget di thinking.

**Cosa fare:**

* Abbassa `MAX_THINKING_TOKENS`, oppure aumenta [`CLAUDE_CODE_MAX_OUTPUT_TOKENS`](/docs/it/env-vars) al di sopra del budget di thinking
* Vedi [Extended thinking](/docs/it/model-config#extended-thinking) per come il budget interagisce con la lunghezza dell'output

<h3 id="tool-use-or-thinking-block-mismatch">
  Mancata corrispondenza tra l'uso dello strumento o il blocco di thinking
</h3>

La cronologia della conversazione ha raggiunto l'API in uno stato incoerente, di solito dopo che una chiamata di strumento è stata interrotta o un turno è stato modificato a metà flusso.

```text theme={null}
API Error: 400 due to tool use concurrency issues. Run /rewind to recover the conversation.
API Error: 400 ... unexpected `tool_use_id` found in `tool_result` blocks
API Error: 400 ... thinking blocks ... cannot be modified
```

Tutte e tre le varianti significano la stessa cosa: la sequenza di blocchi `tool_use`, `tool_result` e `thinking` nella cronologia non corrisponde più a ciò che l'API si aspetta.

**Cosa fare:**

* Se stai utilizzando Opus 4.7 o Opus 4.8, esegui prima `claude update`. Le versioni precedenti a v2.1.156 possono attivare questo errore durante l'uso normale dello strumento, e `/rewind` non lo cancella.
* Esegui `/rewind`, o premi Esc due volte, per tornare indietro a un checkpoint prima del turno corrotto e continua da lì. Vedi [Checkpointing](/docs/it/checkpointing) per come i checkpoint vengono creati e ripristinati.

<h3 id="usage-policy-refusal">
  Rifiuto della politica di utilizzo
</h3>

L'API ha rifiutato di rispondere perché il contenuto nella conversazione ha attivato un controllo della [Politica di utilizzo](https://www.anthropic.com/legal/aup). Il messaggio include un ID di richiesta che puoi citare al supporto se ritieni che il rifiuto sia errato.

```text theme={null}
API Error: Claude Code is unable to respond to this request, which appears to violate our Usage Policy (https://www.anthropic.com/legal/aup). Please double press esc to edit your last message or start a new session for Claude Code to assist with a different task.
```

Il controllo valuta la conversazione completa, non solo il tuo prompt più recente, quindi inviare un nuovo messaggio nella stessa sessione di solito riattiva lo stesso rifiuto. Lo stesso vale dopo l'uscita e la riapertura della sessione con `--continue` o `--resume`, poiché la trascrizione su disco contiene ancora il contenuto che attiva il controllo. Su [Amazon Bedrock](/docs/it/amazon-bedrock), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai) e [Microsoft Foundry](/docs/it/microsoft-foundry), questo messaggio copre anche le richieste che le misure di sicurezza del modello hanno contrassegnato come un argomento di cibersicurezza. Vedi [Le misure di sicurezza hanno contrassegnato un argomento di cibersicurezza](#safety-measures-flagged-a-cybersecurity-topic).

**Cosa fare:**

* Premi Esc due volte o esegui `/rewind` per tornare indietro a un checkpoint prima del turno che ha attivato il rifiuto, quindi riformula o prendi un approccio diverso. Vedi [Checkpointing](/docs/it/checkpointing).
* Se non riesci a identificare quale turno l'ha causato, esegui `/clear` per avviare una conversazione nuova nello stesso progetto. La tua conversazione precedente viene preservata su disco e rimane disponibile in `/resume`.
* In [modalità non interattiva](/docs/it/headless) (`-p`), dove il rewind non è disponibile, riprova con un prompt riformulato in una sessione nuova senza `--continue`. I controlli delle politiche variano in base al modello, quindi passare a un modello diverso con `--model` può anche risolvere il rifiuto in alcuni casi.

<h3 id="safety-measures-flagged-a-cybersecurity-topic">
  Le misure di sicurezza hanno contrassegnato un argomento di cibersicurezza
</h3>

Le misure di sicurezza del modello hanno contrassegnato il contenuto nella conversazione come un argomento di cibersicurezza. Il messaggio nomina il modello che ha contrassegnato la richiesta:

```text theme={null}
API Error: Opus 4.8 has safety measures that flagged this message for a cybersecurity topic. To learn about the Cyber Verification Program and apply for access, visit our help center: https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude.

If you were not engaging in a cybersecurity topic, please send feedback via /feedback.
```

Il messaggio si collega al [Cyber Verification Program](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude), che concede l'accesso per il lavoro di cibersicurezza legittimo. La salvaguardia stessa è lato server e precede v2.1.203; questa versione ha cambiato solo la formulazione del messaggio e la pagina a cui si collega.

Quello che vedi dipende dal tuo provider e dalla modalità:

* Su [Amazon Bedrock](/docs/it/amazon-bedrock), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai) e [Microsoft Foundry](/docs/it/microsoft-foundry), un flag di cibersicurezza produce il messaggio di [rifiuto della politica di utilizzo](#usage-policy-refusal) invece.
* [La modalità non interattiva](/docs/it/headless) omette la frase `/feedback`.

Prima di v2.1.203, il messaggio leggeva `<model>'s safeguards flagged this message for a cybersecurity topic. If your work requires this access, you can apply for an exemption:` seguito da un link del modulo di esenzione.

**Cosa fare:**

* Se il tuo lavoro richiede questo contenuto, richiedi l'accesso tramite il [Cyber Verification Program](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude)
* Se la tua richiesta non riguardava un argomento di cibersicurezza, esegui `/feedback` per segnalare il falso positivo
* Per continuare a lavorare nella stessa sessione, premi Esc due volte o esegui `/rewind` per tornare indietro a un checkpoint prima del turno che ha attivato il flag, quindi prendi un approccio diverso. Vedi [Checkpointing](/docs/it/checkpointing).

<h2 id="installation-errors">
  Errori di installazione
</h2>

Questi errori compaiono durante l'installazione o l'aggiornamento di Claude Code, dallo [script di installazione](/docs/it/setup#install-claude-code), `claude install`, o `claude update`. Per i problemi di `command not found`, PATH, permessi e TLS durante la configurazione, vedere [Risoluzione dei problemi di installazione e accesso](/docs/it/troubleshoot-install).

<h3 id="installation-was-killed-before-it-could-finish">
  L'installazione è stata interrotta prima di poter terminare
</h3>

Lo script di installazione segnala quando il passaggio `claude install` viene terminato da un segnale. Su Linux, il codice di uscita 137 significa che il processo ha ricevuto SIGKILL, e su un host con poca memoria è solitamente il killer out-of-memory (OOM) del kernel. Lo script stampa questa spiegazione ed esce con il codice 137:

```text theme={null}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

Per qualsiasi altro segnale fatale, e per il codice di uscita 137 su macOS, lo script stampa `Installation was killed before it could finish (exit code <N>)` con il codice di uscita effettivo e omette la spiegazione dell'esaurimento della memoria. Il messaggio proviene dallo script di installazione che macOS e Linux utilizzano, che copre anche le installazioni all'interno di WSL; gli script di installazione nativi di Windows non lo stampano mai. Prima della v2.1.200, lo script usciva con solo la riga `Killed` della shell.

**Cosa fare:**

* Interrompere altri processi per liberare memoria, quindi eseguire nuovamente il programma di installazione
* Aggiungere spazio di swap o passare a un'istanza più grande. Vedere [Installazione interrotta su server Linux con poca memoria](/docs/it/troubleshoot-install#install-killed-on-low-memory-linux-servers) per i comandi del file di swap.

<h3 id="the-connection-dropped-while-downloading-the-update">
  La connessione è stata interrotta durante il download dell'aggiornamento
</h3>

La connessione al server di download si è chiusa mentre `claude install`, `claude update`, o l'[aggiornatore automatico](/docs/it/setup#auto-updates) stava scaricando il binario di Claude Code, e i tentativi di ripetizione non hanno recuperato. Claude Code ritenta il download quando la connessione si interrompe, il trasferimento si blocca, o il file scaricato non supera il checksum, fino a tre tentativi in totale. Un errore HTTP completato, come un 404, non viene ritentato perché il server ha già risposto. Prima della v2.1.202, una singola connessione interrotta faceva fallire il download immediatamente con il semplice errore `aborted` invece di ritentare.

```text theme={null}
The connection dropped while downloading the update (attempt 3/3: aborted). Check your network — proxies sometimes cut off large downloads.
```

Il testo tra parentesi nomina quale tentativo ha fallito e l'errore di rete sottostante. `claude update` precede il messaggio con `Error: Failed to install native update` su stderr.

Un download che rimane connesso ma non termina entro 10 minuti fallisce con `Download timed out: exceeded the total deadline` invece. Claude Code non ritenta un download scaduto, perché una connessione troppo lenta per terminare entro la scadenza non terminerà nemmeno su un tentativo immediato di ripetizione. I passaggi seguenti si applicano a entrambi i messaggi. Prima della v2.1.205, la stessa scadenza di 10 minuti era segnalata come il generico `timeout of 600000ms exceeded` del client HTTP.

La causa più comune è un proxy o un gateway che chiude un trasferimento lungo prima che termini. Il binario di Claude Code è un download di grandi dimensioni, quindi un limite di connessione proxy che non influisce mai sul traffico API normale può comunque interromperlo.

**Cosa fare:**

* Eseguire `claude update` di nuovo. Su una rete altrimenti sana, il download di solito ha successo alla prossima esecuzione. Per il messaggio di timeout, eseguirlo di nuovo da una rete più veloce o meno limitata.
* Se la rete richiede un proxy, impostare `HTTPS_PROXY` prima di eseguire il programma di installazione o `claude update`. Vedere [Verificare la connettività di rete](/docs/it/troubleshoot-install#check-network-connectivity).
* Se un proxy aziendale continua a chiudere il trasferimento, chiedere al team di rete di consentire il download completo da `downloads.claude.ai`. Vedere [Requisiti di accesso alla rete](/docs/it/network-config#network-access-requirements).
* Eseguire `claude doctor` dalla shell per la diagnostica dell'installazione

<h2 id="command-line-errors">
  Errori da riga di comando
</h2>

Questi errori provengono dal comando `claude` da riga di comando e dai suoi sottocomandi. Claude Code li stampa prima di eseguire il vostro prompt o di inviare qualsiasi richiesta API.

<h3 id="conflict-between-bg-and-print">
  Conflitto tra --bg e --print
</h3>

Questo messaggio richiede Claude Code v2.1.198 o successivo. Avete combinato `--bg` con `-p` o `--print` nella stessa invocazione di `claude`. `--bg` avvia una [sessione in background](/docs/it/agent-view#from-your-shell) a cui vi collegherete successivamente con `claude agents`, mentre `--print` esegue [in modo non interattivo](/docs/it/headless) e non avvia mai la sessione interattiva a cui `claude agents` si collega. Prima della v2.1.198 questa combinazione creava silenziosamente un job in background che non poteva mai essere collegato.

```text theme={null}
--bg and --print conflict: --print never starts the interactive session that `claude agents` attaches to, so the job would be unattachable. The prompt is the positional — drop --print: `claude --bg '<task>'`.
```

**Cosa fare:**

* Eliminate `-p` o `--print`. `--bg` accetta il prompt come argomento posizionale, quindi `claude --bg "<task>"` è il comando completo. Vedere [Dispatch new agents from your shell](/docs/it/agent-view#from-your-shell).
* Per eseguire il prompt in modo non interattivo e stampare il risultato invece di creare una sessione in background, eliminate `--bg` ed eseguite `claude -p "<task>"`

<h3 id="the-json-schema-value-is-not-a-valid-json-schema">
  Il valore di --json-schema non è uno schema JSON valido
</h3>

Lo schema che avete passato a [`--json-schema`](/docs/it/cli-reference#cli-flags) in [modalità non interattiva](/docs/it/headless#get-structured-output) non ha superato la compilazione dello schema JSON, quindi `claude` esce con codice 1 invece di eseguire il prompt. Prima della v2.1.205, uno schema non valido produceva output non strutturato senza errore, e qualsiasi schema che utilizzava la parola chiave `format` era trattato come non valido.

```text theme={null}
Error: --json-schema is not a valid JSON Schema: data/type must be equal to one of the allowed values
```

Il testo dopo il secondo due punti è la diagnostica del validatore e nomina la parola chiave o la posizione che non ha superato il controllo. Gli schemi che utilizzano la parola chiave `format`, come `"format": "email"`, sono validi: Claude Code accetta `format` come annotazione e non la applica.

Claude Code esegue due controlli prima della compilazione dello schema: rifiuta un valore che non è JSON analizzabile con `Error: --json-schema is not valid JSON`, e JSON valido che non è un oggetto con `Error: --json-schema must be a JSON object`.

**Cosa fare:**

* Correggete la parte dello schema che la diagnostica nomina, quindi rieseguite il comando
* Se la diagnostica è `schema too large`, riducete l'annidamento dello schema e il riutilizzo di `$ref`
* Vedere [Get structured output](/docs/it/headless#get-structured-output) per uno schema e un comando funzionanti

<h3 id="could-not-import-a-server-from-claude-desktop">
  Impossibile importare un server da Claude Desktop
</h3>

Claude Code non ha potuto aggiungere uno dei server che avete selezionato in `claude mcp add-from-claude-desktop`. Il comando importa comunque gli altri server selezionati e stampa una riga per ogni server che non ha potuto aggiungere. Prima della v2.1.205, il primo server che non riusciva fermava l'importazione e nessuno dei server selezionati veniva aggiunto.

```text theme={null}
Could not import my server: Invalid name my server. Names can only contain letters, numbers, hyphens, and underscores.
```

Il testo dopo il nome del server è il motivo. Il più comune è il controllo del nome: Claude Desktop consente caratteri nei nomi dei server, come spazi e punti, che `claude mcp` limita a lettere, numeri, trattini e sottolineature. Altri motivi includono una configurazione del server che non supera la convalida e un server bloccato dalla [politica MCP](/docs/it/managed-mcp) della vostra organizzazione.

**Cosa fare:**

* Rinominate il server in `claude_desktop_config.json` per utilizzare solo lettere, numeri, trattini e sottolineature, quindi eseguite di nuovo `claude mcp add-from-claude-desktop`
* Aggiungete quel server direttamente con `claude mcp add` o `claude mcp add-json` con un nome valido. Vedere [Import MCP servers from Claude Desktop](/docs/it/mcp#import-mcp-servers-from-claude-desktop).

<h3 id="mcp-permission-prompt-tool-not-found">
  Strumento di prompt di autorizzazione MCP non trovato
</h3>

Lo strumento che avete passato a [`--permission-prompt-tool`](/docs/it/cli-reference#cli-flags) non era tra gli strumenti MCP connessi quando l'esecuzione ha avuto bisogno per la prima volta di una decisione di autorizzazione, sia perché il suo server non si è mai connesso sia perché nessun server connesso espone uno strumento con quel nome. Claude Code invia comunque il vostro prompt: l'esecuzione [non interattiva](/docs/it/headless) esce con questo errore, e codice di uscita 1, alla prima chiamata di strumento che necessita di approvazione, quindi non produce alcuna risposta anche se la richiesta è stata effettuata. Prima del primo prompt, Claude Code attende fino al timeout di connessione per server di 30 secondi impostato da [`MCP_TIMEOUT`](/docs/it/env-vars) affinché quel server si connetta. Prima della v2.1.206, l'avvio non attendeva che il server finisse di connettersi, quindi un server che si avvia lentamente ma sano produceva questo errore anche.

```text theme={null}
Error: MCP tool mcp__permissions__approve (passed via --permission-prompt-tool) not found. Available MCP tools: none
```

L'elenco dopo `Available MCP tools:` nomina gli strumenti MCP che erano connessi quando l'attesa è terminata.

**Cosa fare:**

* Verificate che il server si avvii e rimanga connesso: eseguite `claude mcp list` nella stessa directory e confermate che il server è elencato come connesso
* Confermate che il nome dello strumento corrisponda al nome `mcp__<server>__<tool>` che il server espone
* Se il server ha bisogno di più di 30 secondi per avviarsi, aumentate [`MCP_TIMEOUT`](/docs/it/env-vars)

<h2 id="plugin-errors">
  Errori dei plugin
</h2>

Questi errori provengono dalla configurazione dei [plugin](/docs/it/plugins) e del [marketplace](/docs/it/plugin-marketplaces). Per i problemi dei plugin che non producono uno dei messaggi in questa pagina, come un URL del marketplace che non si carica o un plugin che si installa ma non appare, vedere [Risoluzione dei problemi dei plugin](/docs/it/discover-plugins#troubleshooting).

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  Il marketplace è registrato da una fonte non attendibile
</h3>

Il marketplace è registrato con un nome che è [riservato per i marketplace ufficiali di Anthropic](/docs/it/plugin-marketplaces#marketplace-schema), ma la sua fonte registrata non è un repository GitHub di `anthropics`. Claude Code ricontrolla i nomi riservati ogni volta che carica o aggiorna un marketplace, quindi il marketplace e i plugin installati da esso smettono di caricarsi. Prima della v2.1.205, il nome veniva controllato solo quando il marketplace veniva aggiunto, quindi una voce registrata prima che il suo nome diventasse riservato continuava a caricarsi.

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

**Cosa fare:**

* Eseguire `claude plugin marketplace remove <name>`, quindi aggiungere di nuovo il marketplace dal repository ufficiale `github.com/anthropics`
* Se pubblicate un marketplace di terze parti che ha utilizzato il nome prima che diventasse riservato, rinominatelo e chiedete agli utenti di aggiungerlo di nuovo dalla vostra fonte
* Vedere l'elenco dei nomi riservati in [Schema del marketplace](/docs/it/plugin-marketplaces#marketplace-schema)

<h3 id="plugin-command-references-user-config">
  Il comando del plugin fa riferimento a user\_config in un comando shell
</h3>

Un hook del plugin, [monitor](/docs/it/plugins-reference#monitors), o un comando MCP [`headersHelper`](/docs/it/mcp#use-dynamic-headers-for-custom-authentication) fa riferimento a un'[opzione del plugin](/docs/it/plugins-reference#user-configuration) `${user_config.KEY}`, e la stringa sostituita verrebbe passata a una shell. Un valore configurato contenente `$(...)`, backtick o `;` verrebbe eseguito come codice lì, quindi Claude Code rifiuta di avviare il componente invece di sostituire il valore. Il controllo viene eseguito sul modello di comando, quindi l'errore appare anche quando nessun valore è ancora configurato. Prima della v2.1.207, il valore veniva sostituito nel comando shell.

La formulazione dipende da quale superficie ha fatto riferimento all'opzione. Un hook in forma shell segnala:

```text theme={null}
Hook from plugin formatter@acme-tools references ${user_config.*} in a shell-form command. The substituted value would be re-parsed by the shell. Use exec form instead — {"command": "<executable>", "args": ["${user_config.KEY}", ...]} — or read $CLAUDE_PLUGIN_OPTION_<KEY> from the hook's environment. Command: ./scripts/notify.sh ${user_config.webhook_url}
```

Un monitor segnala:

```text theme={null}
Monitor "deploy-status" from plugin deploy-tools references ${user_config.*} in its command. The substituted value would be passed to a shell. Monitor commands cannot safely reference ${user_config.*}; have the monitor script read the value from a config file or prompt instead.
```

Un MCP `headersHelper` segnala:

```text theme={null}
headersHelper for MCP server 'internal-api' references ${user_config.*}. The substituted value would be passed to a shell; read the value inside the helper script instead (e.g. from an env var set in the server's "env" block).
```

**Cosa fare:**

* Per un hook, aggiungere un array `args` in modo che venga eseguito in [forma exec](/docs/it/hooks#exec-form-and-shell-form), dove ogni `${user_config.KEY}` diventa un argomento senza shell in mezzo. Oppure eliminare il riferimento e leggere la variabile di ambiente `$CLAUDE_PLUGIN_OPTION_<KEY>` all'interno dello script
* Per un monitor, eliminare il riferimento e fare in modo che lo script del monitor legga il valore da un file di configurazione
* Per un `headersHelper`, spostare `${user_config.KEY}` nel campo `headers` del server, che non viene analizzato dalla shell, oppure leggere il valore all'interno dello script helper

<h2 id="tool-errors">
  Errori degli strumenti
</h2>

Questi errori provengono dai strumenti integrati di Claude che rifiutano un input. Claude corregge la maggior parte degli errori degli strumenti da solo; i due seguenti richiedono una modifica da parte vostra, perché provengono da una definizione di subagent o da una regola di autorizzazione che controllate.

<h3 id="agent-would-be-spawned-with-zero-tools">
  L'agente verrebbe generato con zero strumenti
</h3>

Nulla nell'[elenco `tools` di un subagent](/docs/it/sub-agents#supported-frontmatter-fields) si è risolto in uno strumento, quindi Claude Code rifiuta di avviare il subagent piuttosto che avviarne uno che non può agire. Il messaggio raggruppa le voci in base al motivo per cui non si sono risolte: non è uno strumento riconosciuto, uno strumento che non è disponibile per i subagent, o riconosciuto ma che non corrisponde a nessuno strumento nella sessione corrente. L'omissione del campo `tools` non attiva mai questo rifiuto. Un pattern di server MCP come `mcp__github__*` non è esente: quando nessuno strumento connesso proviene da quel server, l'avvio viene rifiutato con il pattern nel gruppo matched-nothing. Prima della v2.1.208, il subagent veniva avviato senza strumenti e restituiva un risultato vuoto o confuso.

```text theme={null}
Agent 'code-reviewer' would be spawned with zero tools — refusing. Its tools list resolved to nothing: unrecognized [Grpe]. Fix the agent's tools frontmatter or pass a different subagent_type.
```

**Cosa fare:**

* Correggete ogni voce che l'errore nomina rispetto agli [strumenti disponibili per i subagent](/docs/it/sub-agents#available-tools)
* Rimuovete le voci per gli strumenti che la sessione non ha, come gli strumenti MCP da un server che non è connesso
* Per dare al subagent ogni strumento che ha il genitore, eliminate il campo `tools` invece di elencare gli strumenti

<h3 id="file-is-covered-by-a-read-deny-rule">
  Il file è coperto da una regola di negazione Read
</h3>

Lo strumento Edit è stato chiamato su un percorso corrispondente a una [regola di negazione `Read`](/docs/it/permissions#read-and-edit), inclusa la creazione di un nuovo file in quel percorso. La modifica riscrive il contenuto che Claude deve essere in grado di leggere di nuovo, quindi la chiamata viene rifiutata prima di qualsiasi accesso ai file. La regola blocca solo lo strumento Edit: Write e NotebookEdit non sono coperti da regole di negazione `Read`. Prima della v2.1.208, solo una regola di negazione `Edit` bloccava le modifiche, e una regola di negazione `Read` da sola non lo faceva.

```text theme={null}
File is covered by a Read deny rule in your permission settings and cannot be edited.
```

**Cosa fare:**

* Se Claude dovrebbe essere in grado di modificare il file, rimuovete o restringete la regola di negazione `Read` in `/permissions` o nelle [impostazioni](/docs/it/settings#permission-settings)
* Se il file deve rimanere intatto, mantenete la regola e aggiungete una regola di negazione `Edit` per lo stesso percorso in modo che anche gli strumenti Write e NotebookEdit vengano bloccati

<h2 id="background-session-errors">
  Errori di sessione in background
</h2>

Le [sessioni in background](/docs/it/agent-view) vengono eseguite senza un terminale interattivo proprio, quindi i comandi che ne richiedono uno si comportano diversamente lì. Questi messaggi appaiono nella trascrizione di una sessione in background, nella vista agente o dopo il collegamento.

<h3 id="commands-refused-in-a-background-session">
  Comandi rifiutati in una sessione in background
</h3>

I comandi che aprono una finestra di dialogo interattiva vengono rifiutati in una sessione in background con un messaggio che nomina un modulo che funziona lì o che ti dice di eseguire il comando da un terminale regolare. `/install-github-app`, l'elenco delle impostazioni `/mcp` e le azioni di autenticazione nel menu del server MCP vengono tutti rifiutati in questo modo. Prima della versione 2.1.208, aprivano la loro finestra di dialogo all'interno della sessione in background.
Nella versione 2.1.208 solamente, il selettore `/model` è stato rifiutato anche in una sessione in background e `/upgrade` ha stampato l'URL di aggiornamento invece di aprire un browser.

La formulazione nomina il comando che è stato rifiutato. L'elenco delle impostazioni `/mcp` riporta:

```text theme={null}
Can't open MCP settings in a background session — use `/mcp enable|disable|reconnect <server>` to steer, or run /mcp from an interactive terminal to authenticate.
```

**Cosa fare:**

* Usa il modulo che il messaggio nomina, come `/mcp reconnect <server>`, `/mcp enable` o `/mcp disable`
* Per i flussi di accesso e autorizzazione, esegui il comando da una sessione `claude` regolare in un terminale

<h3 id="claude_code_process_wrapper-launcher-errors">
  Errori del launcher CLAUDE\_CODE\_PROCESS\_WRAPPER
</h3>

[`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/it/corporate-launcher) è impostato e il suo valore non può essere utilizzato, quindi Claude Code rifiuta di avviare il processo interessato piuttosto che eseguirlo senza il launcher. I problemi di configurazione vengono segnalati con un messaggio che inizia con il nome della variabile e dichiara il motivo, ad esempio:

```text theme={null}
CLAUDE_CODE_PROCESS_WRAPPER: launcher `/opt/corp/launcher` is not an executable regular file
```

Un launcher che si avvia ma esce senza sostituirsi con Claude Code non riesce nella sessione che stava avviando, e la riga della sessione nella vista agente riporta che il launcher `must exec, not daemonize`, seguito da qualsiasi cosa il launcher abbia stampato. Una sessione che non può avviarsi o raggiungere il servizio in background a causa del launcher riporta il problema del launcher come motivo all'interno di `Couldn't reach the background service (...)`.

**Cosa fare:**

* Imposta la variabile al percorso assoluto di un eseguibile che termina chiamando `exec "$@"`. Vedi [il contratto del launcher](/docs/it/corporate-launcher#the-launcher-contract) per il contratto completo
* Controlla `/status`, che mostra il comando di avvio risolto nella sua voce Self-exec e avverte quando il servizio in background in esecuzione non corrisponde, oppure esegui `claude daemon status` da una shell
* Dopo aver corretto il valore nel blocco `env` delle [impostazioni](/docs/it/corporate-launcher#set-up-the-launcher), riavvia il servizio in background con `claude daemon stop --any` in modo che il prossimo invio avvii uno avvolto

<h2 id="configuration-warnings">
  Avvisi di configurazione
</h2>

Claude Code scrive questi messaggi su stderr all'avvio anziché mostrare un errore nella conversazione. Segnalano la configurazione che ha letto ma non ha applicato.

<h3 id="workspace-has-not-been-trusted">
  Lo spazio di lavoro non è stato considerato attendibile
</h3>

Claude Code ha trovato regole `permissions.allow` o voci `permissions.additionalDirectories` nel file `.claude/settings.json` o `.claude/settings.local.json` del progetto e non le ha applicate, perché [le regole di autorizzazione dal progetto richiedono l'attendibilità dello spazio di lavoro](/docs/it/permissions#project-allow-rules-and-workspace-trust). Il conteggio, il nome dell'impostazione e il file denominato nel messaggio variano in base alla configurazione. Le regole `deny` e `ask` non sono interessate.

```text theme={null}
Ignoring 2 permissions.allow entries from .claude/settings.local.json: this workspace has not been trusted. Run Claude Code interactively here once and accept the trust dialog, or set projects["/Users/you/project"].hasTrustDialogAccepted: true in /Users/you/.claude.json.
```

**Cosa fare:**

* Eseguire `claude` nella directory e accettare la finestra di dialogo di attendibilità. La finestra di dialogo viene visualizzata anche quando una directory padre è già considerata attendibile, elenca le regole che vengono trattenute e consente di rifiutare e continuare a lavorare senza di esse. Prima della v2.1.200, nessuna finestra di dialogo veniva visualizzata in quella situazione, quindi questo passaggio non poteva essere completato lì.
* In [modalità non interattiva](/docs/it/headless) con `-p` nessuna finestra di dialogo viene mostrata. Impostare la voce `hasTrustDialogAccepted` in `~/.claude.json` utilizzando la chiave `projects` esatta che il messaggio stampa.
* Se il messaggio nomina `.claude/settings.local.json` e hai avviato Claude Code al di fuori di un repository git o nella directory home, aggiorna alla v2.1.200 o successiva. Le versioni da 2.1.196 a 2.1.199 hanno trattato il tuo `.claude/settings.local.json` come fornito dal repository in quegli spazi di lavoro. Sulla v2.1.207 e successive, l'aggiornamento non è sufficiente al di fuori di un repository git se non hai considerato attendibile la cartella: determinare che una cartella non si trova all'interno di un repository esegue git, e Claude Code esegue quel controllo solo dopo che accetti la finestra di dialogo di attendibilità, quindi utilizza il primo passaggio. La directory home e qualsiasi altra [home di configurazione](/docs/it/permissions#project-allow-rules-and-workspace-trust) sono esenti e non attendono la finestra di dialogo. Vedi [Regole di autorizzazione del progetto e attendibilità dello spazio di lavoro](/docs/it/permissions#project-allow-rules-and-workspace-trust).

<h2 id="responses-seem-lower-quality-than-usual">
  Le risposte sembrano di qualità inferiore al solito
</h2>

Se le risposte di Claude sembrano meno capaci di quanto ti aspetti ma non viene mostrato alcun errore, la causa è solitamente lo stato della conversazione piuttosto che il modello stesso. Claude Code non cambia silenziosamente le versioni del modello. Può passare a un modello di fallback in tre casi specifici:

* Un [`--fallback-model`](/docs/it/cli-reference#cli-flags) configurato subentra dopo un errore di disponibilità, solo per quel turno, con un avviso nella trascrizione
* Un controllo di avvio di Amazon Bedrock o della piattaforma Agent di Google Cloud trova il tuo modello predefinito non disponibile
* Il [fallback automatico del modello](/docs/it/model-config#automatic-model-fallback) su Fable 5 sposta la sessione al modello Opus predefinito e mostra un avviso nella trascrizione

Il controllo della selezione del modello di seguito cattura il secondo e il terzo caso; il primo appare come un avviso nella trascrizione piuttosto che come un cambio `/model`. La [configurazione del modello](/docs/it/model-config) spiega quando si applica ogni fallback.

Controlla prima questi elementi:

* **Selezione del modello**: esegui `/model` per confermare che sei sul modello che ti aspetti. Una scelta `/model` precedente o una variabile di ambiente `ANTHROPIC_MODEL` potrebbe averti messo su un modello più piccolo di quello che intendevi.
* **Livello di sforzo**: esegui `/effort` per controllare il livello di ragionamento attuale e aumentarlo per il debug difficile o il lavoro di progettazione. I valori predefiniti variano in base al modello, quindi controlla prima di assumere che sei al di sotto del massimo. Vedi [Regola il livello di sforzo](/docs/it/model-config#adjust-effort-level) per i valori predefiniti per modello e il collegamento `ultrathink`.
* **Pressione del contesto**: esegui `/context` per vedere quanto è pieno il window. Se è vicino alla capacità, esegui `/compact` a un punto naturale o `/clear` per ricominciare da capo. Vedi [Esplora la finestra di contesto](/docs/it/context-window) per come auto-compact influisce sui turni precedenti.
* **Istruzioni obsolete**: file `CLAUDE.md` grandi o obsoleti e definizioni di strumenti MCP consumano contesto e possono indirizzare le risposte. Il controllo `/doctor` contrassegna i file di memoria sovradimensionati e le estensioni inutilizzate, e `/context` mostra l'utilizzo dei token degli strumenti MCP. Prima della v2.1.205, `/doctor` apriva una schermata di diagnostica che contrassegnava i file di memoria sovradimensionati e le definizioni dei subagent.

Quando una risposta va male, il rewind di solito funziona meglio che rispondere con correzioni. Premi Esc due volte o esegui `/rewind` per tornare indietro prima del turno sbagliato, quindi riformula il prompt con più specifiche. Correggere nel thread mantiene il tentativo sbagliato nel contesto, il che può ancorare le risposte successive ad esso. Vedi [Checkpointing](/docs/it/checkpointing).

Se la qualità sembra ancora non corretta dopo aver controllato quanto sopra, esegui `/feedback` e descrivi cosa ti aspettavi rispetto a quello che hai ottenuto. Il feedback inviato in questo modo include la trascrizione della conversazione, che è il modo più veloce per Anthropic per diagnosticare una vera regressione. Vedi [Segnala un errore](#report-an-error) se `/feedback` non è disponibile nel tuo ambiente.

Se Claude ti avverte di un sospetto prompt injection, o rifiuta una richiesta a causa di un sospetto injection, e il testo che l'avviso nomina è contesto che Claude Code aggiunge automaticamente alla conversazione piuttosto che contenuto di file o web, esegui `claude update` e riprova. Se l'avviso si ripete dopo l'aggiornamento, [segnalalo](#report-an-error) piuttosto che incollare il contenuto contrassegnato di nuovo nel prompt. Prima della v2.1.201, Sonnet 5 rifiutava alcune richieste allo stesso modo.

<h2 id="report-an-error">
  Segnalare un errore
</h2>

Per gli errori dei componenti non trattati in questa pagina, consultare la guida pertinente:

* Il server MCP non è riuscito a connettersi o autenticarsi: [MCP](/docs/it/mcp)
* Lo script hook non è riuscito o ha bloccato uno strumento: [Debug hooks](/docs/it/hooks#debug-hooks)
* Permesso negato o errori del filesystem durante l'installazione: [Risoluzione dei problemi di installazione e accesso](/docs/it/troubleshoot-install)

Se un errore non è elencato qui o la correzione suggerita non aiuta:

* Eseguire `/feedback` all'interno di Claude Code per inviare la trascrizione e una descrizione ad Anthropic. Il comando offre anche di aprire un problema GitHub precompilato. L'invio ad Anthropic richiede l'[autenticazione](/docs/it/authentication). Su Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry e altri provider di terze parti, o quando non sono configurate credenziali Anthropic, `/feedback` salva un archivio locale che è possibile inviare al rappresentante dell'account Anthropic.
* Eseguire `claude doctor` dalla shell per una diagnostica di sola lettura dell'installazione, oppure eseguire il checkup `/doctor` all'interno di Claude Code per trovare e risolvere i problemi di configurazione
* Controllare [status.claude.com](https://status.claude.com) per gli incidenti attivi
* Cercare i [problemi esistenti](https://github.com/anthropics/claude-code/issues) su GitHub
