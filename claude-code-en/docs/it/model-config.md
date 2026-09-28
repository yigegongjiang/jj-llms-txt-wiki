> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurazione del modello

> Configurare quale modello Claude Code utilizza, livelli di impegno, contesto esteso e la finestra di auto-compattazione

<h2 id="available-models">
  Modelli disponibili
</h2>

Per l'impostazione `model` in Claude Code, puoi configurare:

* Un **alias del modello**
* Un **nome del modello**
  * Anthropic API: un **[nome del modello](https://platform.claude.com/docs/en/about-claude/models/overview)** completo
  * Amazon Bedrock: un ARN del profilo di inferenza
  * Microsoft Foundry: un nome di distribuzione
  * Google Cloud's Agent Platform: un nome di versione

Per indicazioni su quale modello e livello di impegno si adattano a diversi tipi di lavoro, consulta [Choosing a Claude model and effort level in Claude Code](https://claude.com/blog/claude-model-and-effort-level-in-claude-code) sul blog.

<Note>
  `ANTHROPIC_BASE_URL` cambia dove vengono inviate le richieste, non quale modello le risponde. Per instradare Claude attraverso un gateway LLM, consulta [LLM gateways](/docs/it/llm-gateway).
</Note>

<h3 id="model-aliases">
  Alias del modello
</h3>

Usa un alias del modello per selezionare le impostazioni del modello senza ricordare i numeri di versione esatti:

| Alias del modello | Comportamento                                                                                                                                                                                                                                                                                                                                                                 |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`default`**     | Valore speciale che cancella qualsiasi override del modello e ripristina il [valore predefinito di runtime per il tuo account](#default-model-setting). Non è di per sé un alias del modello                                                                                                                                                                                  |
| **`best`**        | Utilizza il modello a cui si risolve l'alias [`fable`](#fable-alias-resolution) dove Fable è disponibile per te, altrimenti lo stesso modello di `opus`                                                                                                                                                                                                                       |
| **`fable`**       | Utilizza il [modello Fable per il tuo provider](#fable-alias-resolution) per i tuoi compiti più difficili e lunghi                                                                                                                                                                                                                                                            |
| **`sonnet`**      | Utilizza il modello Sonnet più recente per i compiti di codifica quotidiani                                                                                                                                                                                                                                                                                                   |
| **`opus`**        | Utilizza il modello Opus più recente per i compiti di ragionamento complesso                                                                                                                                                                                                                                                                                                  |
| **`haiku`**       | Utilizza il modello Haiku veloce ed efficiente per compiti semplici                                                                                                                                                                                                                                                                                                           |
| **`sonnet[1m]`**  | Utilizza Sonnet con una [finestra di contesto di 1 milione di token](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) per sessioni lunghe. Nessun effetto quando `sonnet` si risolve già in Sonnet 5 con la sua finestra nativa di 1M; dietro un [gateway LLM](/docs/it/llm-gateway), seleziona la finestra di 1M per Sonnet 5 |
| **`opus[1m]`**    | Utilizza Opus con una [finestra di contesto di 1 milione di token](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) per sessioni lunghe                                                                                                                                                                                   |
| **`opusplan`**    | Modalità speciale che utilizza `opus` durante la modalità piano, quindi passa a `sonnet` per l'esecuzione                                                                                                                                                                                                                                                                     |

La versione a cui si risolvono gli alias `opus` e `sonnet` dipende dal provider:

| Provider                                             | `opus`   | `sonnet`   |
| :--------------------------------------------------- | :------- | :--------- |
| Anthropic API                                        | Opus 5.5 | Sonnet 5   |
| [Claude Platform on AWS](/docs/it/claude-platform-on-aws) | Opus 5.5 | Sonnet 4.6 |
| Amazon Bedrock, Google Cloud's Agent Platform        | Opus 5.5 | Sonnet 4.5 |
| Microsoft Foundry                                    | Opus 4.6 | Sonnet 4.5 |

<span id="fable-alias-resolution" />

A meno che non imposti `ANTHROPIC_DEFAULT_FABLE_MODEL`, l'alias `fable` si risolve in Fable 5.1, tranne nelle sessioni [Claude apps gateway](/docs/it/claude-apps-gateway) dove `fable` e `best` si risolvono in Fable 5. Prima della v2.1.257, `fable` si risolveva in Fable 5 su ogni provider.

Un gateway non configurato per servire `claude-fable-5-1` rifiuta le richieste per quel modello. Per utilizzare Fable 5.1 attraverso un gateway che lo serve, selezionalo con `/model claude-fable-5-1`.

Dove un alias si risolve in un modello più vecchio, i modelli più recenti sono disponibili selezionando il nome del modello completo esplicitamente o impostando `ANTHROPIC_DEFAULT_OPUS_MODEL` o `ANTHROPIC_DEFAULT_SONNET_MODEL`.

Prima della v2.1.280, `opus` si risolveva in Opus 5 su Anthropic API, Claude Platform on AWS, Amazon Bedrock e Google Cloud's Agent Platform dalla v2.1.219. Prima della v2.1.219, `opus` si risolveva in Opus 4.8 su Anthropic API dalla v2.1.154, e su Claude Platform on AWS, Amazon Bedrock e Google Cloud's Agent Platform dalla v2.1.207. Prima della v2.1.207, `opus` si risolveva in Opus 4.7 su Claude Platform on AWS e in Opus 4.6 su Amazon Bedrock e Google Cloud's Agent Platform.

Gli alias puntano alla versione consigliata per il tuo provider e si aggiornano nel tempo. Per fissare una versione specifica, utilizza il nome del modello completo, ad esempio `claude-opus-5-5`, o imposta la variabile di ambiente corrispondente come `ANTHROPIC_DEFAULT_OPUS_MODEL`.

<Note>
  Opus 5.5 richiede Claude Code v2.1.280 o successiva. Opus 5 richiede v2.1.219 o successiva. Sonnet 5 richiede v2.1.197 o successiva. Esegui `claude update` per aggiornare.
</Note>

<h3 id="work-with-fable">
  Lavora con Fable
</h3>

[Claude Fable 5.1](https://platform.claude.com/docs/en/about-claude/models/overview) e Claude Fable 5 sono i modelli più capaci in Claude Code, adatti a compiti più grandi di una singola sessione. Sostengono lunghe sessioni autonome, investigano prima di agire e verificano il loro lavoro più spesso rispetto ai modelli più piccoli. Fable 5.1 è la versione più recente.

Nessuno dei due modelli Fable è il valore predefinito del tipo di account su alcun piano o provider. Selezionane uno esplicitamente:

* **Fable 5.1**: esegui `/model fable`, o avvia con `claude --model fable`. Nelle sessioni [Claude apps gateway](/docs/it/claude-apps-gateway) dove l'alias si risolve in Fable 5, esegui `/model claude-fable-5-1` invece.
* **Fable 5**: selezionalo per ID del modello. Su Anthropic API, esegui `/model claude-fable-5` o avvia con `claude --model claude-fable-5`. Su altri provider, utilizza l'ID del modello Fable 5 del tuo provider o [fissalo](#pin-models-for-third-party-deployments) con `ANTHROPIC_DEFAULT_FABLE_MODEL`.

Se ti connetti direttamente a Anthropic API e le tue impostazioni utente contengono `claude-fable-5` o `claude-fable-5[1m]` come modello, ad esempio perché hai selezionato Fable nel selettore `/model` prima della v2.1.257, Claude Code cambia quel valore salvato nell'alias `fable` o `fable[1m]` la prima volta che esegui v2.1.257 o successiva. La riga del modello di avvio mostra `(auto-updated)` una volta. Un valore `claude-fable-5` nelle impostazioni di progetto, locali o gestite rimane come è.

Le richieste che i classificatori di sicurezza di un modello Fable contrassegnano, il più delle volte nei domini della sicurezza informatica e della biologia, attivano il [fallback automatico del modello](#automatic-model-fallback).

Per ottenere il massimo da Fable:

* **Descrivi il risultato, non i passaggi**: dagli il risultato che desideri e lascia che pianifichi il percorso. Per mantenerlo orientato verso quel risultato, [imposta un obiettivo](/docs/it/goal).
* **Dagli problemi ambigui**: le indagini sulla causa principale, il debug delle interruzioni e le decisioni architettoniche sono dove l'indagine e la verifica extra ripagano.
* **Salta i promemoria di verifica**: verifica il suo lavoro con meno sollecitazioni, quindi i promemoria per testare o controllare sono solitamente non necessari.
* **Dimensiona compiti più grandi**: dagli lavoro che normalmente divideresti in pezzi. Sostiene lunghe sessioni senza perdere il filo.

<Note>
  Fable 5.1 richiede Claude Code v2.1.257 o successiva. Se una richiesta per essa da una versione precedente fallisce, consulta [Claude Code does not support this model](/docs/it/errors#claude-code-does-not-support-this-model). Esegui `claude update` per aggiornare. Per la disponibilità con zero data retention, consulta [Model availability under ZDR](/docs/it/zero-data-retention#model-availability-under-zdr).
</Note>

Su Anthropic API, un modello Fable appare nel selettore `/model` a meno che [`availableModels`](#restrict-model-selection) o [restrizioni del modello dell'organizzazione](#organization-model-restrictions) lo escludano. Quando la tua organizzazione non può utilizzare Fable affatto, ad esempio con [zero data retention](/docs/it/zero-data-retention#model-availability-under-zdr), la riga rimane nel selettore disattivata, con una nota sul motivo.

<h4 id="fable-and-usage-credits">
  Fable e crediti di utilizzo
</h4>

A seconda del tuo piano e del livello di posto, l'utilizzo di Fable può fatturare ai [crediti di utilizzo](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) invece di attingere ai limiti inclusi nel tuo piano. Quando lo fa, il selettore `/model` mostra "Requires usage credits" sulla riga Fable. Per gestire i crediti di utilizzo, consulta [Add usage credits to your subscription](/docs/it/costs#add-usage-credits-to-your-subscription).

Nelle sessioni interattive, Claude Code mostra un prompt di consenso prima che una richiesta Fable fatturi crediti di utilizzo. I membri dei piani Enterprise con fatturazione organizzativa non vedono il prompt. Puoi continuare su Fable utilizzando crediti di utilizzo o passare al tuo modello predefinito. Puoi anche chiudere il prompt:

* Nel selettore `/model`, mantieni il tuo modello attuale.
* A metà sessione, Claude Code continua il turno sul tuo modello predefinito.

Dopo aver scelto di continuare su Fable utilizzando crediti di utilizzo, Claude Code non mostra più il prompt.

In una sessione con [Remote Control](/docs/it/remote-control) connesso, una [sessione in background](/docs/it/agent-view), o una sessione del compagno di un [team di agenti](/docs/it/agent-teams), nessuno potrebbe essere al terminale, quindi Claude Code tiene il prompt di consenso a metà sessione per la scadenza [`dialogExpiry`](/docs/it/settings-reference#dialogexpiry), cinque minuti per impostazione predefinita. Se nessuno ha risposto entro la scadenza, Claude Code termina il turno senza inviare la richiesta e aggiunge un avviso alla trascrizione, che il client Remote Control mostra anche. La tua selezione del modello rimane invariata e Claude Code chiede di nuovo il consenso al tuo prossimo messaggio.

Quello che puoi fare mentre il prompt è in attesa dipende dalla sessione:

* Con Remote Control connesso o nella sessione di un compagno, premi un tasto qualsiasi al terminale per annullare la scadenza, e Claude Code attende la tua risposta.
* In una sessione in background, rispondi prima della scadenza.
* Se invii un nuovo messaggio dal client remoto prima che qualcuno abbia digitato al terminale, Claude Code termina il turno allo stesso modo e il tuo nuovo messaggio inizia il turno successivo. Dopo che qualcuno digita al terminale, Claude Code continua ad aspettare la risposta e mette in coda il tuo nuovo messaggio dietro di essa.

In [modalità non interattiva](/docs/it/headless) con il flag `-p` e attraverso Agent SDK, Claude Code non mostra mai il prompt di consenso. Quando una richiesta Fable lì fatturasse crediti di utilizzo, Claude Code la fattura senza chiedere.

<h3 id="setting-your-model">
  Impostazione del tuo modello
</h3>

Puoi configurare il tuo modello in diversi modi, elencati in ordine di priorità:

1. **Durante la sessione**: usa `/model <alias|name>` per passare immediatamente, o esegui `/model` senza argomenti per aprire il selettore. Consulta [quando Claude Code ti chiede di confermare il passaggio](/docs/it/prompt-caching#switching-models)
2. **All'avvio**: avvia con `claude --model <alias|name>`
3. **Variabile di ambiente**: imposta `ANTHROPIC_MODEL=<alias|name>`
4. **Impostazioni**: configura permanentemente nel tuo file di impostazioni usando il campo `model`
5. **[Predefinito per le nuove sessioni](#set-a-default-model-for-new-sessions)**: imposta `ANTHROPIC_DEFAULT_MODEL=<alias|name>`

`/model` salva la tua scelta come predefinita per le nuove sessioni scrivendo il campo `model` nelle tue impostazioni utente. Nel selettore:

* `Enter`: cambia modello e salva come predefinito
* `s`: cambia modello solo per questa sessione e lascia il tuo predefinito invariato. Per utilizzare un tasto diverso, riassegna [`modelPicker:thisSessionOnly`](/docs/it/keybindings#model-picker-actions)

Digitare `/model <name>` direttamente si comporta come `Enter`. Per cambiare solo per questa sessione, apri il selettore con `/model` e premi `s` sulla riga del modello.

Se cambi modelli con `/model`, il cambio raggiunge anche i [subagenti che ereditano il modello della conversazione principale](/docs/it/sub-agents#choose-a-model), perché Claude Code risolve il loro modello da quello che la tua sessione sta utilizzando quando Claude li avvia. Passa a Opus prima che Claude deleghi ricerche o esecuzioni di test a uno di loro, e quel lavoro viene eseguito su Opus anche. Per mantenere un subagente personalizzato su un modello più piccolo, imposta `model` nella sua definizione.

Se imposti un modello con `/model` in [modalità non interattiva](/docs/it/headless), con il flag `-p`, la tua scelta si applica solo alla sessione corrente e non viene salvata come predefinita; `/model` in quella modalità richiede Claude Code v2.1.205 o successiva. Le impostazioni di progetto e gestite hanno ancora la precedenza e si riapplicano al prossimo avvio. Un [modello predefinito dell'organizzazione](#organization-default-model) che il tuo amministratore ha configurato per ignorare la selezione dell'utente si riapplica anche al prossimo avvio.

Nella v2.1.144 attraverso v2.1.152, `/model` si applicava solo alla sessione corrente e `d` nel selettore salvava un predefinito.

Il flag `--model` e la variabile di ambiente `ANTHROPIC_MODEL` si applicano solo alla sessione che avvii con loro. Per eseguire modelli diversi in terminali diversi contemporaneamente, avvia ognuno con il suo flag `--model` piuttosto che cambiare con `/model`.

I prezzi nel selettore `/model` appaiono quando Claude Code parla con Anthropic API, direttamente o attraverso un [gateway LLM](/docs/it/llm-gateway) che lo proxies, e il prezzo su una riga è il prezzo del modello che quella riga seleziona. Su [provider di terze parti](/docs/it/third-party-integrations) come Amazon Bedrock e sul [gateway delle app Claude](/docs/it/claude-apps-gateway), il tuo provider o gateway determina quello che paghi, quindi le righe del selettore non mostrano alcun prezzo. Il prezzo è solo un'etichetta di visualizzazione; non influisce su quale modello una riga seleziona o su cosa il tuo provider fattura. Prima della v2.1.206, [Claude Platform on AWS](/docs/it/claude-platform-on-aws) e le sessioni del gateway mostravano i prezzi di listino di Anthropic, e una riga poteva mostrare il prezzo di un modello diverso da quello che selezionava.

Le sessioni riprese avviate con `claude --resume`, `--continue`, o il selettore `/resume` mantengono il modello che stavano utilizzando quando la trascrizione è stata salvata, indipendentemente dall'impostazione `model` corrente. Se il modello ripristinato è stato ritirato o è escluso da [`availableModels`](#restrict-model-selection), la sessione ricade nell'ordine di precedenza normale. Questo impedisce che la scelta `/model` di un'altra sessione cambi il modello al ripristino. Su provider che utilizzano ID di distribuzione specifici del provider piuttosto che ID di modello Anthropic, come Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry, il modello della trascrizione non viene affatto ripristinato e la sessione risolve il suo modello attraverso l'ordine di precedenza normale.

Un modello che scegli per il nuovo avvio con `--model` o `ANTHROPIC_MODEL` ha ancora la precedenza sul modello ripristinato. A partire dalla v2.1.195, così fa una variabile della famiglia [`ANTHROPIC_DEFAULT_OPUS_MODEL`](#environment-variables). [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions) può farlo anche, secondo le condizioni elencate nella sua sezione.

Quando il modello attivo all'avvio proviene dalle impostazioni di progetto o gestite piuttosto che dalla tua selezione, l'intestazione di avvio mostra quale file di impostazioni lo ha impostato. Esegui `/model` per ignorare; l'impostazione di progetto o gestita si riapplica al prossimo avvio. Su piattaforme che incorporano Claude Code e impostano [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/it/env-vars), la configurazione del modello dell'host ha la precedenza sulle impostazioni del modello gestite, mentre un elenco di autorizzazione `availableModels` gestito rimane in vigore a meno che l'host non fornisca il suo; [Exceptions to managed settings precedence](/docs/it/settings#exceptions-to-managed-settings-precedence) dice quali chiavi e variabili l'host ignora.

Se tu o la tua organizzazione configurate [hook PreModelSwitch](/docs/it/hooks#premodelswitch), vengono eseguiti prima che un cambio richiesto si applichi e possono bloccarlo o chiederti di confermare.

Quando Claude Code non riesce a dire quali hook PreModelSwitch i [plugin gestiti](/docs/it/settings-reference#enabledplugins) della tua organizzazione forniscono, ad esempio perché un plugin gestito non è riuscito a caricarsi, rifiuta il cambio piuttosto che applicarlo senza controllo, e controlla di nuovo ad ogni nuovo tentativo. Consulta [Model switch was blocked by a PreModelSwitch hook](/docs/it/errors#model-switch-was-blocked-by-a-premodelswitch-hook) per il messaggio e il recupero.

Quando cambi modelli attraverso il metodo `setModel()` dell'[Agent SDK](/docs/it/agent-sdk/overview) o da un dispositivo connesso attraverso [Remote Control](/docs/it/remote-control), o un'app come l'[app Desktop](/docs/it/desktop) che esegue il CLI di Claude Code cambia per te, Claude Code controlla che la stringa sia una che riconosce prima di salvarla. Questo controllo richiede Claude Code v2.1.200 o successiva. Controllare una scelta Remote Control richiede Claude Code v2.1.260 o successiva sulla tua macchina. Su Anthropic API, Claude Code riconosce:

* un alias del modello
* una voce dal selettore `/model`
* qualsiasi nome che inizia con `claude-`
* un valore che hai configurato tu stesso come [opzione di modello personalizzato](#add-a-custom-model-option) o in [`modelOverrides`](#override-model-ids-per-version)

Claude Code rifiuta una stringa non riconosciuta con `Model "<name>" is not a recognized model id.` e la sessione mantiene il suo modello attuale, invece di salvare la stringa e fallire alla prossima richiesta. Consulta [il riferimento dell'errore](/docs/it/errors#model-is-not-a-recognized-model-id) per i passaggi di recupero.

Il controllo viene eseguito solo su Anthropic API. Su Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, [Claude Platform on AWS](/docs/it/claude-platform-on-aws), e dietro un [gateway LLM](/docs/it/llm-gateway) o un `ANTHROPIC_BASE_URL` personalizzato, il tuo provider o gateway definisce i nomi dei modelli, quindi Claude Code passa qualsiasi stringa senza controllarla. Il controllo non copre nemmeno il flag `--model`, la variabile di ambiente `ANTHROPIC_MODEL`, o l'impostazione `model`; un valore digitato male lì produce [There's an issue with the selected model](/docs/it/errors#theres-an-issue-with-the-selected-model) alla prima richiesta invece. Claude Code può comunque scrivere la [riga diagnostica del modello non riconosciuto](/docs/it/errors#unrecognized-model-id-on-a-request) al momento della richiesta, su ogni provider.

Quando il modello richiesto ha una data di ritiro programmata o viene automaticamente rimappato a una versione più recente, Claude Code mostra un avviso che nomina il modello richiesto. Le sessioni interattive lo mostrano come un avviso di avvio. Dalla v2.1.182, lo stesso avviso viene scritto su stderr in [modalità non interattiva](/docs/it/headless) quando si utilizza il formato di output di testo predefinito. Il controllo copre anche un `model` impostato nel [frontmatter del subagente](/docs/it/sub-agents). L'avviso stderr è soppresso per `--output-format json` e `stream-json`; leggi il modello effettivo dal campo `modelUsage` del [messaggio di risultato](/docs/it/headless#get-structured-output) invece.

Ad esempio, avvia una sessione su Opus:

```bash theme={null}
claude --model opus
```

Quindi cambia modelli dalla sessione:

```text theme={null}
/model sonnet
```

File di impostazioni di esempio:

```json theme={null}
{
    "permissions": {
        "allow": ["Bash(npm run lint)"]
    },
    "model": "opus"
}
```

<h4 id="set-a-default-model-for-new-sessions">
  Imposta un modello predefinito per le nuove sessioni
</h4>

Imposta `ANTHROPIC_DEFAULT_MODEL=<alias|name>` per scegliere il modello su cui le tue sessioni si avviano per impostazione predefinita. Richiede Claude Code v2.1.236 o successiva.

Claude Code avvia una nuova sessione sul modello della variabile solo quando nessuno di questi seleziona un modello:

* Il flag `--model`
* `ANTHROPIC_MODEL`
* Un valore `model` in qualsiasi file di impostazioni, inclusa la scelta che salvi con `/model`
* Un [modello predefinito dell'organizzazione](#organization-default-model)

Una scelta che salvi con `/model` ha la precedenza sulla variabile anche nei lanci successivi. Con `ANTHROPIC_MODEL` impostato invece, Claude Code ritorna al modello della variabile al prossimo lancio, qualunque cosa tu abbia salvato con `/model`.

Claude Code risolve anche l'opzione Predefinito al modello della variabile, a meno che non si applichi un modello predefinito dell'organizzazione. Quando l'opzione Predefinito si risolve nel modello della variabile, la riga Predefinito nel selettore `/model` mostra l'etichetta Set by ANTHROPIC\_DEFAULT\_MODEL.

Claude Code ignora la variabile in questi casi e l'opzione Predefinito si risolve come se non l'avessi impostata:

* L'hai impostata su `default`, `inherit`, `opusplan`, o `haiku`
* [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) è attivo
* [`availableModels`](#restrict-model-selection) o [restrizioni del modello dell'organizzazione](#organization-model-restrictions) escludono il modello
* Il modello non è disponibile per il tuo account

Quando una nuova sessione si avvierebbe sul modello della variabile, una sessione che riprendi con `claude --resume`, `--continue`, o il selettore `/resume` si avvia su di esso anche. Claude Code non ripristina il modello salvato nella trascrizione di quella sessione. Altrimenti Claude Code non utilizza la variabile quando [riprendi una sessione](#setting-your-model).

<h4 id="a-new-session-starts-on-a-different-model-than-you-picked">
  Una nuova sessione si avvia su un modello diverso da quello che hai scelto
</h4>

Quando scegli un modello con `/model` e la tua prossima sessione si avvia su qualcos'altro, queste sono le cause solite:

* **L'hai scelto per una sessione.** Premere `s` nel selettore, avviare con `--model`, e eseguire `/model` in modalità non interattiva si applicano tutti alla sessione corrente e lasciano il tuo predefinito salvato da solo.
* **Qualcosa con priorità più alta imposta il modello.** Un valore `model` nelle impostazioni di progetto o gestite, `ANTHROPIC_MODEL` nella tua shell, o un [modello predefinito dell'organizzazione](#organization-default-model) che il tuo amministratore ha impostato per ignorare le scelte dell'utente si applica di nuovo ad ogni lancio. La tua scelta `/model` è ancora salvata; è superata. Quando le impostazioni di progetto o gestite impostano il modello, l'intestazione di avvio nomina il file.
* **Claude Code non ha potuto salvare la tua scelta.** `/model` scrive `model` in `~/.claude/settings.json`. Se non puoi scrivere in quel file, ad esempio perché un altro strumento lo genera o lo collega a una copia di sola lettura, il modello che hai scelto dura per la sessione e il prossimo lancio legge il valore vecchio. Imposta `model` nello strumento che genera il file, o rendi il file scrivibile. Consulta [A change you made in Claude Code is lost in new sessions](/docs/it/settings#a-change-you-made-in-claude-code-is-lost-in-new-sessions).
* **Hai ripreso una sessione.** Una sessione che riprendi con `claude --resume` o `--continue` di solito [mantiene il modello che stava utilizzando](#setting-your-model) piuttosto che il tuo predefinito attuale.

<h2 id="restrict-model-selection">
  Limitare la selezione del modello
</h2>

Gli amministratori aziendali possono utilizzare `availableModels` nelle [impostazioni gestite o di policy](/docs/it/managed-settings) per limitare quali modelli gli utenti possono selezionare. Le voci corrispondono a una famiglia di modelli come `sonnet`, un prefisso di versione come `claude-sonnet-4-5`, o un ID modello completo come `claude-sonnet-4-5-20250929`. Un prefisso di versione corrisponde anche a ID modello successivi che lo estendono con un altro segmento, quindi `claude-fable-5` consente sia Fable 5 che Fable 5.1, mentre `claude-fable-5-1` consente solo Fable 5.1.

Sulle piattaforme che incorporano Claude Code e impostano [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/it/env-vars), la configurazione del modello dell'host ha la precedenza rispetto alle impostazioni del modello gestito, mentre un allowlist `availableModels` gestito rimane in vigore a meno che l'host non fornisca il proprio; [Eccezioni alla precedenza delle impostazioni gestite](/docs/it/settings#exceptions-to-managed-settings-precedence) indica quali chiavi e variabili l'host sostituisce.

Quando `availableModels` è impostato, l'allowlist si applica ovunque un utente possa specificare un modello:

* **Modello della sessione principale**: `/model`, il flag `--model`, la variabile di ambiente `ANTHROPIC_MODEL`, l'impostazione `model`, [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions), e il modello ripristinato quando [si riprende una sessione](#setting-your-model)
* **Risoluzione alias**: le variabili di ambiente `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, `ANTHROPIC_DEFAULT_HAIKU_MODEL`, e `ANTHROPIC_DEFAULT_FABLE_MODEL` non possono reindirizzare un alias consentito a un modello al di fuori dell'elenco
* **Modalità veloce**: `/fast` rifiuta di attivare/disattivare quando comporterebbe un passaggio implicito a un modello Opus al di fuori dell'elenco, con il messaggio "is not in your organization's allowed models"
* **Modelli di subagent e teammate**: il campo `model` nel frontmatter di [subagent](/docs/it/sub-agents#choose-a-model), il parametro `model` dello strumento Agent, i modelli teammate del [team di agent](/docs/it/agent-teams#specify-teammates-and-models), `CLAUDE_CODE_SUBAGENT_MODEL`, e, nella versione 2.1.197 e precedenti, il selezionatore di modelli nella procedura guidata `/agents`&#x20;
* **Modelli di skill e comando**: il frontmatter `model` in [skills e commands](/docs/it/skills)
* **Modello Advisor**: l'impostazione [`advisorModel`](/docs/it/advisor) configurata e il flag `--advisor`
* **Modello di background agent**: il modello selezionato nel [dispatch picker](/docs/it/agent-view)

Sull'API Anthropic e su [Claude Platform on AWS](/docs/it/claude-platform-on-aws), un alias di famiglia di modelli, `opus`, `sonnet`, `haiku`, o `fable`, si risolve nel suo modello usuale quando l'allowlist consente quel modello. Quando l'allowlist blocca quel modello, Claude Code sostituisce la versione più recente della famiglia che l'allowlist consente e mostra un avviso che nomina sia i modelli richiesti che quelli sostituiti. Con `["sonnet", "claude-opus-4-6"]`, ad esempio, sia `/model opus` che `--model opus` selezionano Claude Opus 4.6, l'Opus più recente consentito. Prima della versione 2.1.205, un alias la cui versione più recente rilasciata era al di fuori dell'elenco veniva rifiutato o sostituito come qualsiasi altra selezione bloccata, anche quando l'elenco consentiva una versione precedente.

La sostituzione ha bisogno di una versione consentita su cui atterrare: quando l'allowlist non consente alcuna versione della famiglia dell'alias, l'alias segue il comportamento di rifiuto e sostituzione di seguito come qualsiasi altro valore bloccato.

Claude Code gestisce qualsiasi altra selezione bloccata in base a dove il modello è stato impostato:

* **`/model`**: Claude Code rifiuta il passaggio con un errore
* **Flag `--model`, `ANTHROPIC_MODEL`, o l'impostazione `model`**: Claude Code sostituisce il valore all'avvio con un avviso che nomina sia i modelli richiesti che quelli sostituiti, e la sessione inizia sul modello predefinito
* **[`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions)**: Claude Code ignora la variabile
* **Override di subagent o teammate**: Claude Code esegue il subagent o il teammate su un modello di fallback piuttosto che far fallire la richiesta. Vedere [Scegliere un modello](/docs/it/sub-agents#choose-a-model) per il fallback del subagent e [Specificare teammate e modelli](/docs/it/agent-teams#specify-teammates-and-models) per il fallback del teammate.

  Nelle sessioni interattive, Claude Code ti avverte quando sostituisce il modello di un subagent, da questo fallback o dalla sostituzione della versione più recente consentita di cui sopra, nominando i modelli richiesti e sostituiti; non segnala il fallback di un teammate.

  Dove opera la sostituzione della versione più recente consentita di cui sopra, un alias di famiglia bloccato la segue invece. Prima della versione 2.1.222, un alias ricadeva come qualsiasi altro valore bloccato su ogni provider
* **Override di skill o comando**: Claude Code ignora l'override, incluso un alias di famiglia bloccato, e lo skill o il comando viene eseguito sul modello della sessione. Uno skill o un comando che [viene eseguito in un subagent](/docs/it/skills#run-skills-in-a-subagent) segue il comportamento del subagent di cui sopra
* **Impostazione `advisorModel`**: l'advisor è disabilitato per la sessione
* **Flag `--advisor`**: Claude Code esce con un errore all'avvio. In una [sessione in background](/docs/it/agent-view), avvia la sessione senza l'advisor invece di uscire

Claude Code nasconde i modelli esclusi dal selezionatore `/model`. Un ID modello completo nell'elenco che non ha una riga del selezionatore integrata, come una versione precedente che l'elenco fissa, appare nel selezionatore `/model` come sua propria riga etichettata, a meno che Claude Code non sostituisca le opzioni integrate con una lineup [`modelPicker`](/docs/it/settings-reference#modelpicker). Prima della versione 2.1.199, tale ID era selezionabile solo digitando `/model <id>`.

I cambiamenti di modello che Claude Code effettua per tuo conto vengono controllati allo stesso modo:

* **[Catene di modelli di fallback](#fallback-model-chains)**: le voci al di fuori dell'allowlist vengono eliminate
* **Aggiornamenti in Plan Mode**: sull'API Anthropic e su Claude Platform on AWS, un aggiornamento come [`opusplan`](#opusplan-model-setting) a un modello escluso utilizza la versione più recente consentita della famiglia di aggiornamento. Su provider con ID modello specifici del provider, e quando nessuna versione è consentita, l'aggiornamento viene saltato e la pianificazione continua sul modello della sessione
* **[Fallback automatico del modello](#automatic-model-fallback)**: un fallback il cui target è escluso non viene eseguito, quindi la richiesta contrassegnata termina con un rifiuto
* **[Classificatore Auto Mode](/docs/it/permission-modes#eliminate-prompts-with-auto-mode)**: il valore predefinito Claude Sonnet 5 del classificatore si applica solo quando l'allowlist consente Sonnet 5. Quando è escluso, il classificatore viene eseguito sul modello della sessione, che l'allowlist già governa, o su un modello Opus quando la sessione viene eseguita su un [modello Fable](#work-with-fable). Su provider diversi dall'API Anthropic, quel fallback Opus viene eseguito sul modello Opus predefinito del provider senza consultare l'allowlist. Richiede Claude Code v2.1.210 o successivo
* **[Modalità veloce](/docs/it/fast-mode)**: l'abilitazione della modalità veloce viene rifiutata quando il modello su cui la sessione verrebbe eseguita in seguito è al di fuori dell'allowlist

```json theme={null}
{
  "availableModels": ["sonnet", "haiku"]
}
```

<h3 id="surface-coverage">
  Copertura della superficie
</h3>

Ogni superficie applica l'allowlist che riceve. Quale meccanismo di consegna raggiunge ogni superficie differisce:

| Meccanismo di consegna                                                                          | CLI e IDE | Sessioni locali desktop | Sessioni web, mobile e cloud                                                                                                                                                                                                                                           | Agent SDK e non-interattive | Cowork                     |
| :---------------------------------------------------------------------------------------------- | :-------- | :---------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------- | :------------------------- |
| [Impostazioni gestite dal server](/docs/it/server-managed-settings) dalla console di amministrazione | Applicate | Applicate               | Applicate                                                                                                                                                                                                                                                              | Applicate                   | Non consegnate             |
| [File MDM o impostazioni gestite](/docs/it/managed-settings#delivery-mechanisms)                     | Applicate | Applicate               | Non consegnate negli ambienti ospitati da Anthropic; negli [ambienti self-hosted](/docs/it/self-hosted-environments), applicate dall'immagine del runner secondo [come Claude Code combina le fonti gestite](/docs/it/managed-settings#how-claude-code-combines-managed-sources) | Applicate                   | Applicate dove distribuite |

* Le sessioni cloud su [Claude Code sul web](/docs/it/claude-code-on-the-web), incluse quelle che avvii dall'app Desktop, vengono eseguite su VM gestite da Anthropic per impostazione predefinita: le impostazioni distribuite al tuo dispositivo non le raggiungono, quindi consegna l'allowlist tramite impostazioni gestite dal server. Le sessioni che la tua organizzazione instrada a un [ambiente self-hosted](/docs/it/self-hosted-environments) vengono eseguite sul tuo calcolo e leggono anche il file di impostazioni gestite nell'immagine del runner. [Come Claude Code combina le fonti gestite](/docs/it/managed-settings#how-claude-code-combines-managed-sources) indica quando quel file si applica. Un cambio di modello a metà sessione in una sessione cloud viene rifiutato quando il modello richiesto è escluso dall'allowlist. Quando l'elenco `availableModels` nelle tue impostazioni gestite dal server è non vuoto, il server rifiuta la richiesta di un utente di avviare una sessione cloud su un modello che l'elenco esclude.
* Cowork, la scheda agentic-work nell'app Claude Desktop, esegue le sue sessioni su Claude Code ma, per progettazione, non riceve impostazioni gestite dal server dalla console di amministrazione claude.ai. Un file di impostazioni gestite si applica alle sessioni Cowork quando è presente dove la sessione viene eseguita; le sessioni Cowork remote vengono eseguite su VM gestite da Anthropic, dove un file distribuito dal dispositivo non è presente.
* Le sessioni su [provider di terze parti](/docs/it/server-managed-settings#platform-availability) come Amazon Bedrock, Agent Platform di Google Cloud, Microsoft Foundry, e [Claude Platform on AWS](/docs/it/claude-platform-on-aws) non ricevono impostazioni gestite dal server, quindi consegna l'allowlist tramite file MDM o impostazioni gestite lì.
* La consegna gestita dal server richiede anche che la sessione si autentichi con un [login o chiave idonei](/docs/it/server-managed-settings#platform-availability). Le flotte che generano chiavi solo tramite uno script [`apiKeyHelper`](/docs/it/settings-reference#apikeyhelper) dovrebbero consegnare l'allowlist tramite file MDM o impostazioni gestite.
* La scheda Desktop Code ospita anche [sessioni SSH](/docs/it/desktop#ssh-sessions), che leggono il file di impostazioni gestite dall'host remoto su cui vengono eseguite. Vedere [Impostazioni gestite desktop](/docs/it/desktop#managed-settings).
* I selezionatori di modelli su claude.ai e nell'app Desktop nascondono o disattivano i modelli esclusi dall'allowlist della tua organizzazione. Lo stato del selezionatore è una comodità per gli utenti; non applica l'allowlist.

<h3 id="default-model-behavior">
  Comportamento del modello predefinito
</h3>

Da solo, `availableModels` lascia l'opzione Predefinito sul [valore predefinito di runtime](#default-model-setting) del sistema per l'account fino a quando non imposti anche [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model). Se quel valore predefinito è un modello che intendi limitare, imposta anche `enforceAvailableModels`.

Un array `availableModels` vuoto non attiva mai l'applicazione del modello Predefinito: con `availableModels: []`, le selezioni di modelli denominati vengono bloccate ma il modello Predefinito per il tipo di account rimane utilizzabile indipendentemente da `enforceAvailableModels`.

<h3 id="enforce-the-allowlist-for-the-default-model">
  Applicare l'allowlist per il modello Predefinito
</h3>

Imposta `enforceAvailableModels: true` insieme a un `availableModels` non vuoto nelle impostazioni gestite per estendere l'allowlist all'opzione Predefinito. Ciò richiede Claude Code v2.1.175 o successivo.

```json theme={null}
{
  "availableModels": ["sonnet", "haiku"],
  "enforceAvailableModels": true
}
```

L'opzione Predefinito si risolve nel valore predefinito del tipo di account, o nel [modello predefinito dell'organizzazione](#organization-default-model) quando un amministratore ne ha impostato uno. Quando quel modello non è nell'allowlist, l'opzione Predefinito si risolve invece nella prima voce `availableModels` che nomina un modello consentito e disponibile, e la riga Predefinito del selezionatore `/model` mostra quel modello. Questo si applica ovunque il valore predefinito sia raggiunto: avvio della sessione, selezione di Predefinito in `/model`, la parola chiave `"default"` nelle [catene di modelli di fallback](#fallback-model-chains), e il fallback utilizzato quando una selezione esclusa viene eliminata.

`enforceAvailableModels` rimappa l'opzione Predefinito solo quando `availableModels` è non vuoto. Con `availableModels: []`, il modello Predefinito per il tipo di account rimane utilizzabile, quindi l'impostazione non può bloccare gli utenti da ogni modello. Quando `availableModels` è non vuoto ma nessuna voce si risolve in un modello consentito e disponibile, l'applicazione viene saltata e Predefinito si risolve nel valore predefinito del tipo di account, con un avviso visibile solo sotto `--debug`. Mantieni almeno una voce garantita disponibile nell'elenco per evitare questo.

Distribuisci entrambe le chiavi insieme nella fonte gestita con il ranking più alto che consegni. Per impostazione predefinita Claude Code legge solo quella fonte, quindi una coppia posizionata in un file di impostazioni gestite viene ignorata quando la console di amministrazione consegna qualsiasi impostazione; secondo il merge opt-in in [come Claude Code combina le fonti gestite](/docs/it/managed-settings#how-claude-code-combines-managed-sources), Claude Code ignora comunque una mappa `modelOverrides` da una fonte classificata al di sotto di quella che imposta `availableModels`.

<h3 id="control-the-model-users-run-on">
  Controllare il modello su cui gli utenti vengono eseguiti
</h3>

L'impostazione `model` è una selezione iniziale, non un'applicazione. Imposta quale modello è attivo quando una sessione inizia, ma gli utenti possono comunque aprire `/model` e scegliere Predefinito, che si risolve nel [valore predefinito di runtime](#default-model-setting) del sistema indipendentemente da cosa sia impostato `model`, a meno che [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) non lo reindizzi.

Per controllare completamente l'esperienza del modello, combina queste impostazioni:

* **`availableModels`**: limita quali modelli denominati gli utenti possono passare a
* **`enforceAvailableModels`**: estende l'allowlist `availableModels` all'opzione Predefinito, quindi Predefinito non può risolversi in un modello al di fuori dell'elenco
* **`model`**: imposta la selezione del modello iniziale quando una sessione inizia
* **`ANTHROPIC_DEFAULT_SONNET_MODEL`** / **`ANTHROPIC_DEFAULT_OPUS_MODEL`** / **`ANTHROPIC_DEFAULT_HAIKU_MODEL`** / **`ANTHROPIC_DEFAULT_FABLE_MODEL`**: controllano a cosa si risolvono gli alias `sonnet`, `opus`, `haiku`, e `fable`, e quale versione utilizza il [valore predefinito del tipo di account](#default-model-setting)

Questo esempio avvia gli utenti su Sonnet 4.5, limita il selezionatore a Sonnet e Haiku, e assicura che Predefinito si risolva in un modello nell'allowlist piuttosto che nel valore predefinito del tier:

```json theme={null}
{
  "model": "claude-sonnet-4-5",
  "availableModels": ["claude-sonnet-4-5", "haiku"],
  "enforceAvailableModels": true,
  "env": {
    "ANTHROPIC_DEFAULT_SONNET_MODEL": "claude-sonnet-4-5"
  }
}
```

Senza `enforceAvailableModels` o il blocco `env`, un utente che seleziona Predefinito nel selezionatore ottiene il [valore predefinito di runtime](#default-model-setting) piuttosto che la versione fissata in `model`. Le due impostazioni coprono ambiti diversi: `enforceAvailableModels` fa sì che Predefinito obbedisca all'allowlist, mentre il blocco `env` fissa quale versione un alias consentito come `sonnet` si risolve. Usa `enforceAvailableModels` da solo quando limitare le famiglie di modelli è sufficiente; aggiungi il blocco `env` quando hai anche bisogno di fissare una versione specifica.

<h3 id="merge-behavior">
  Comportamento di merge
</h3>

Quando le impostazioni gestite che Claude Code applica definiscono `availableModels`, solo quell'elenco si applica, a parte una [piattaforma host che fornisce il proprio](/docs/it/settings#exceptions-to-managed-settings-precedence): le voci nelle impostazioni utente, progetto o locale non possono estenderlo, e Claude Code non unisce mai `availableModels` tra fonti gestite nemmeno; [come Claude Code combina le fonti gestite](/docs/it/managed-settings#how-claude-code-combines-managed-sources) indica quale elenco della fonte si applica. Altrimenti, gli elenchi dalle impostazioni utente, progetto e locale vengono [concatenati e deduplicati](/docs/it/settings#settings-precedence) come altre impostazioni di array. Prima di Claude Code v2.1.175, le voci da ambiti di precedenza inferiore si univano nell'elenco gestito invece di essere sostituite da esso.

All'interno dell'elenco effettivo, una voce che nomina un modello specifico in una famiglia, sia un prefisso di versione che un ID modello completo, disabilita la voce wildcard della famiglia: `["sonnet", "claude-sonnet-4-5"]` consente solo le versioni di Sonnet 4.5, non ogni modello Sonnet.

<h3 id="mantle-model-ids">
  ID modello Mantle
</h3>

Quando l'[endpoint Amazon Bedrock Mantle](/docs/it/amazon-bedrock#use-the-mantle-endpoint) è abilitato, le voci in `availableModels` che iniziano con `anthropic.` vengono aggiunte al selezionatore `/model` come opzioni personalizzate e instradate all'endpoint Mantle. Questa è un'eccezione alla corrispondenza dell'alias descritta in [Fissare modelli per distribuzioni di terze parti](#pin-models-for-third-party-deployments). L'impostazione limita comunque il selezionatore alle voci elencate, e un ID Mantle incorpora un nome di famiglia, quindi conta come una voce specifica e disabilita il wildcard della famiglia: insieme a qualsiasi ID Mantle, elenca i prefissi di versione o gli ID completi che desideri mantenere selezionabili. Vedere [Comportamento di merge](#merge-behavior).

<h3 id="organization-model-restrictions">
  Restrizioni del modello dell'organizzazione
</h3>

Gli amministratori dell'organizzazione sui piani Claude Enterprise limitano quali modelli i membri possono eseguire disabilitando i singoli modelli nella console di amministrazione claude.ai. Questa restrizione viene consegnata con i diritti dell'account quando Claude Code si autentica, separata da qualsiasi elenco `availableModels` nelle impostazioni, e il server applica la stessa restrizione indipendentemente quando una sessione viene creata. Richiede Claude Code v2.1.187 o successivo.

La restrizione si applica quando un membro accede o utilizza la propria chiave API. Le credenziali con ambito organizzativo, come le chiavi di servizio dell'organizzazione, non sono legate a un utente, quindi la restrizione non si applica a loro.

La Claude Console non ha controllo di restrizione del modello. Le organizzazioni senza un piano Claude Enterprise, incluse quelle i cui membri si autenticano tramite l'API Anthropic, limitano i modelli con [`availableModels`](#restrict-model-selection) nelle [impostazioni gestite](/docs/it/managed-settings), aggiungendo [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) per coprire l'opzione Predefinito. [Copertura della superficie](#surface-coverage) indica come ogni superficie riceve e applica queste impostazioni.

Un modello limitato è nascosto dal selezionatore `/model`. Selezionarlo per nome con `--model`, la variabile di ambiente `ANTHROPIC_MODEL`, o l'impostazione `model` mostra l'avviso `Model "<name>" is restricted by your organization's settings. Using <model> instead.` e la sessione inizia su un modello consentito. Digitare `/model <name>` per un modello limitato viene rifiutato con `Model '<name>' is restricted by your organization's settings. Run /model to choose a different model.` e la sessione mantiene il suo modello attuale.

Un [alias di famiglia di modelli](#restrict-model-selection) come `opus` si risolve nel suo modello usuale quando l'organizzazione lo consente. Quando l'organizzazione limita quel modello, Claude Code sostituisce la versione più recente della famiglia che l'organizzazione consente, con lo stesso avviso di sostituzione. `/model <alias>` viene rifiutato solo quando ogni versione della sua famiglia è limitata; un alias impostato con `--model`, `ANTHROPIC_MODEL`, o l'impostazione `model` viene comunque sostituito all'avvio in quel caso. Prima della versione 2.1.205, un alias di famiglia veniva sostituito o rifiutato in base alla sua versione più recente rilasciata da sola, anche quando una versione precedente era consentita.

Le restrizioni si applicano a livello di organizzazione o per ruolo:

* Disabilitare un modello a livello di organizzazione lo rimuove per ogni membro.
* L'accesso a livello di ruolo concede modelli diversi a diversi ruoli personalizzati, e un membro che detiene diversi ruoli può utilizzare qualsiasi modello che uno dei suoi ruoli concede.
* I modelli Haiku sono sempre disponibili e non possono essere disabilitati, quindi ogni membro mantiene almeno un modello utilizzabile.
* Un cambio di accesso ha effetto su nuove richieste entro circa un minuto; il selezionatore `/model` lo riflette la prossima volta che una sessione inizia.

Entrambe le restrizioni si applicano insieme: un modello è selezionabile solo quando è consentito da `availableModels` e non limitato dall'organizzazione. Le restrizioni dell'organizzazione raggiungono le sessioni sull'API Anthropic e sui distribuzioni [LLM gateway](/docs/it/llm-gateway) solo; su qualsiasi altro provider, utilizza `availableModels` invece.

<h2 id="organization-default-model">
  Modello predefinito dell'organizzazione
</h2>

Gli amministratori dell'organizzazione nei piani Claude Enterprise possono impostare un modello predefinito per i membri di Claude Code dall'admin console di claude.ai, per l'intera organizzazione o per ruolo personalizzato. Quando ne viene impostato uno, l'opzione Predefinito si risolve in quel modello. Richiede Claude Code v2.1.196 o versione successiva.

La riga Predefinito nel selettore `/model` mostra il nome del modello predefinito dell'organizzazione con l'etichetta Org default. L'etichetta recita Org default indipendentemente dal fatto che l'amministratore abbia impostato il modello predefinito per l'intera organizzazione o per il vostro ruolo. Un modello predefinito del ruolo copre i membri di quel ruolo personalizzato e ha la precedenza sul modello predefinito a livello di organizzazione; quando diversi vostri ruoli impostano modelli predefiniti diversi, si applica il modello più capace.

Il modello predefinito dell'organizzazione è un punto di partenza, non una restrizione. Queste selezioni hanno la precedenza su di esso:

* il flag `--model` e la variabile di ambiente `ANTHROPIC_MODEL`
* un valore `model` nelle [impostazioni gestite](/docs/it/managed-settings) o fornito tramite `--settings`
* un valore `model` nelle vostre impostazioni utente, progetto o locali, incluso un modello che salvate con `/model`

Gli amministratori possono anche configurare il modello predefinito dell'organizzazione per ignorare la selezione dell'utente. Con l'override attivato, ha la precedenza sul valore `model` nelle impostazioni utente, progetto e locali, quindi un modello che salvate con `/model` si applica per la sessione corrente e il modello predefinito dell'organizzazione ritorna al prossimo avvio. Quando la vostra selezione differisce, `/model` mostra `Your organization's default (<model>) applies on restart`. Il flag `--model`, `ANTHROPIC_MODEL`, le impostazioni gestite e `--settings` hanno ancora la precedenza anche con l'override attivato. L'override è disponibile per un set limitato di organizzazioni; chiedete al vostro team di account Anthropic sulla disponibilità.

Per limitare quali modelli i membri possono selezionare, utilizzate [restrizioni del modello dell'organizzazione](#organization-model-restrictions) o [`availableModels`](#restrict-model-selection) invece.

Claude Code legge il modello predefinito dell'organizzazione una sola volta all'avvio, quindi un modello predefinito che l'amministratore cambia durante la sessione ha effetto al prossimo avvio.

Quando il modello predefinito dell'organizzazione non ignora la selezione dell'utente, il primo avvio interattivo dopo che l'amministratore lo cambia cancella la chiave `model` dalle vostre impostazioni utente una sola volta, in modo che il nuovo modello predefinito si applichi. Non cambia nient'altro nel file, e un modello che salvate con `/model` dopo quel lancio viene mantenuto.

Il modello predefinito dell'organizzazione passa attraverso questi controlli di restrizione prima di essere adottato:

* [`availableModels`](#restrict-model-selection) da solo non si applica al modello predefinito dell'organizzazione, quindi un modello predefinito dell'organizzazione al di fuori della lista di autorizzazione si applica comunque. Quando [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) è anche impostato, un modello predefinito dell'organizzazione al di fuori della lista di autorizzazione viene rimappato alla prima voce della lista di autorizzazione, come qualsiasi altro Predefinito
* un modello predefinito dell'organizzazione che [restrizioni del modello dell'organizzazione](#organization-model-restrictions) negano per il vostro account viene sostituito dal modello più recente consentito nella sua famiglia, o da una famiglia a costo inferiore quando ogni versione di essa è limitata
* un modello predefinito dell'organizzazione che non è disponibile per il vostro account affatto viene saltato, e l'opzione Predefinito si risolve come farebbe [senza un modello predefinito dell'organizzazione](#default-model-setting)

A partire dalla v2.1.199, quando il modello predefinito dell'organizzazione è una famiglia di modelli diversa dal modello predefinito usuale del tipo di account vostro, il selettore `/model` mantiene una riga separata per quella famiglia usuale, in modo che possiate comunque passare ad essa per una sessione. Dalla v2.1.196 alla v2.1.198 quella riga manca dal selettore.

Il modello predefinito dell'organizzazione raggiunge solo le sessioni autenticate con l'API Anthropic. Per impostare un modello predefinito in qualsiasi altro luogo, incluse le distribuzioni [LLM gateway](/docs/it/llm-gateway), utilizzate la chiave `model` nelle [impostazioni gestite](/docs/it/managed-settings) invece.

<h2 id="organization-effort-limits">
  Limiti di sforzo dell'organizzazione
</h2>

La vostra organizzazione può limitare il [livello di sforzo](#adjust-effort-level) in due modi. Su un piano Claude Enterprise, gli amministratori dell'organizzazione impostano limiti di sforzo per ruolo, descritti di seguito. Su qualsiasi piano e qualsiasi provider, inclusi Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry, l'impostazione gestita [`maxEffortLevel`](/docs/it/settings-reference#maxeffortlevel) limita lo sforzo sul client. Quando entrambi si applicano a un modello, si applica il limite inferiore.

Gli amministratori dell'organizzazione nei piani Claude Enterprise possono impostare un [livello di sforzo](#adjust-effort-level) massimo per modello per ogni ruolo personalizzato, insieme alle [restrizioni del modello a livello di organizzazione](#organization-model-restrictions). I livelli superiori al limite non vengono offerti nel selettore `/effort`, e denominare un livello superiore con `--effort` o `/effort` viene eseguito al limite. Nelle sessioni interattive e nelle esecuzioni in testo semplice `--print`, un avviso nomina i livelli richiesti e applicati; con output `json` o `stream-json` o negli agenti in background, il limite si applica silenziosamente. I limiti sono per modello, quindi il cambio di modelli può modificare quali livelli sono disponibili. Quando diversi vostri ruoli concedono lo stesso modello, si applica il limite meno restrittivo. Richiede Claude Code v2.1.195 o successivo.

I limiti di sforzo vengono forniti insieme alle [restrizioni del modello dell'organizzazione](#organization-model-restrictions) e raggiungono le stesse sessioni.

<h2 id="special-model-behavior">
  Comportamento speciale del modello
</h2>

<h3 id="default-model-setting">
  Impostazione del modello `default`
</h3>

Il comportamento di `default` dipende dal tipo di account:

* **Pro, Max, Team, Enterprise e Anthropic API**: predefinito su Opus 5.5
* **Claude Platform su AWS, Amazon Bedrock e Google Cloud's Agent Platform**: predefinito su Opus 5.5
* **Microsoft Foundry**: predefinito su Sonnet 4.5

Prima della v2.1.280, `default` si risolveva in Sonnet 5 su Pro e Team Standard, e in Opus 5 su Max, Team Premium, Enterprise, Anthropic API, Claude Platform su AWS, Amazon Bedrock e Google Cloud's Agent Platform da v2.1.219. Prima della v2.1.219, `default` si risolveva in Opus 4.8 su Anthropic API, Max, Team Premium e Enterprise con pagamento a consumo da v2.1.154, e su Claude Platform su AWS, Amazon Bedrock e Google Cloud's Agent Platform da v2.1.207. Prima della v2.1.207, `default` si risolveva in Opus 4.7 su Claude Platform su AWS e in Sonnet 4.5 su Amazon Bedrock e Google Cloud's Agent Platform.

Quando un amministratore ha impostato un [modello predefinito dell'organizzazione](#organization-default-model), `default` si risolve in quel modello invece del valore predefinito del tipo di account sopra indicato. Richiede Claude Code v2.1.196 o successiva. `default` può anche risolversi nel modello impostato con [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions), secondo le condizioni elencate nella relativa sezione.

Quando le impostazioni gestite [applicano l'elenco consentito per il modello predefinito](#enforce-the-allowlist-for-the-default-model) e il valore predefinito del tipo di account non è in `availableModels`, `default` si risolve nel valore predefinito applicato invece del valore predefinito del tipo di account sopra indicato. Quando entrambi si applicano, il valore predefinito dell'organizzazione sostituisce prima il valore predefinito del tipo di account e l'applicazione si applica quindi ad esso: un valore predefinito dell'organizzazione nell'elenco consentito viene mantenuto, mentre uno al di fuori dell'elenco si risolve nel valore predefinito applicato.

I modelli Fable non sono il valore predefinito del tipo di account su nessun piano o provider. Sceglierne uno con `/model` lo salva come modello selezionato nelle impostazioni utente, in modo che le sessioni successive inizino su di esso. Per il cambio una tantum che Claude Code apporta a una selezione Fable 5 salvata nella v2.1.257, vedere [Lavorare con Fable](#work-with-fable).

<h3 id="opusplan-model-setting">
  Impostazione del modello `opusplan`
</h3>

L'alias del modello `opusplan` fornisce un approccio ibrido automatizzato:

* **In modalità piano**: utilizza `opus` per il ragionamento complesso e le decisioni architettoniche
* **In modalità esecuzione**: passa automaticamente a `sonnet` per la generazione del codice e l'implementazione

Questo abbina il ragionamento di Opus per la pianificazione con l'efficienza di Sonnet per l'esecuzione.

La fase Opus in modalità piano utilizza la stessa finestra di contesto dell'impostazione del modello `opus`, e la fase di esecuzione utilizza la stessa finestra di `sonnet`. Quando `opus` e `sonnet` si risolvono in modelli che vengono eseguiti con la [finestra di contesto 1M](#extended-context) per impostazione predefinita, come i modelli attuali su Anthropic API, entrambe le fasi vengono eseguite con essa. Per richiedere il contesto 1M per entrambe le fasi dove non lo fanno, [imposta il modello](#setting-your-model) su `opusplan[1m]`, ad esempio con `/model opusplan[1m]`. Impostarlo con `/model` richiede Claude Code v2.1.265 o successiva; nelle versioni precedenti, usa il flag `--model` o l'impostazione `model`.

Quando [`availableModels`](#restrict-model-selection) esclude l'Opus più recente ma consente una versione precedente, ad esempio `["sonnet", "claude-opus-4-6"]`, `opusplan` utilizza l'Opus più recente consentito per la pianificazione e rimane su Sonnet solo quando ogni Opus è escluso. Una sessione Haiku che normalmente si aggiornerebbe a Sonnet in modalità piano utilizza allo stesso modo il Sonnet più recente consentito e rimane su Haiku solo quando ogni Sonnet è escluso. Prima della v2.1.205, la modalità piano rimase sul modello della sessione ogni volta che la versione più recente della famiglia di aggiornamento era esclusa, anche quando l'elenco consentito ne consentiva una precedente.

La sostituzione di una versione precedente consentita si applica su Anthropic API e [Claude Platform su AWS](/docs/it/claude-platform-on-aws). Su Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry e Mantle, i cui deployment utilizzano ID modello specifici del provider, la modalità piano rimane sul modello della sessione ogni volta che il modello di aggiornamento è escluso.

Per un approccio ibrido in cui Claude decide a metà attività quando consultare un secondo modello piuttosto che passare al confine del piano, vedere lo [strumento advisor](/docs/it/advisor).

<h3 id="fallback-model-chains">
  Catene di modelli di fallback
</h3>

Quando il modello primario è sovraccarico, non disponibile o restituisce un altro errore del server non ripetibile, Claude Code può passare a un modello di fallback invece di non riuscire nella richiesta. Gli errori di autenticazione, fatturazione, limite di velocità, dimensione della richiesta e trasporto, e un [rifiuto dalla verifica della politica della tua organizzazione](/docs/it/errors#automatic-retries), non attivano mai un passaggio; questi seguono il loro normale retry e gestione degli errori.

Configura uno o più modelli di fallback e Claude Code li prova in ordine, mostrando un avviso quando passa. Il passaggio dura solo per il turno corrente, quindi il tuo prossimo messaggio prova prima il modello primario di nuovo. Claude Code limita le catene a tre modelli dopo la rimozione dei duplicati e ignora le voci extra.

Imposta una catena per una sessione con il flag `--fallback-model`, che accetta un elenco separato da virgole:

```bash theme={null}
claude --fallback-model sonnet,haiku
```

Per mantenere una catena tra le sessioni, imposta `fallbackModel` in [settings](/docs/it/settings) come array:

```json theme={null}
{
  "fallbackModel": ["claude-sonnet-5", "claude-haiku-4-5"]
}
```

Il flag `--fallback-model` ha la precedenza sull'impostazione `fallbackModel`. Ogni voce accetta un nome di modello o un alias, e `"default"` si espande al modello predefinito.

Claude Code non conferma la catena all'avvio e `/status` non la visualizza. L'avviso mostrato quando si verifica un passaggio è il primo segno visibile che un fallback è configurato.

Quando una richiesta fallisce, Claude Code prova ogni voce in ordine finché una non l'accetta. Una voce che non può essere raggiunta nemmeno, come un modello ritirato bloccato nelle impostazioni, fallisce alla successiva nello stesso modo. Claude Code rimuove due tipi di voce prima di quel percorso:

* **Al di fuori dell'elenco consentito**: Claude Code elimina qualsiasi voce non consentita da [`availableModels`](#restrict-model-selection) quando legge la catena.
* **Finestra di contesto più piccola durante la compattazione**: la catena copre anche la [compattazione](/docs/it/context-window#what-survives-compaction), ma Claude Code non farà fallback a un modello con una finestra di contesto più piccola di quella del primario, poiché il riassunto lì taglierebbe prima parte della conversazione. Se ogni fallback è più piccolo, la compattazione mostra l'errore originale e puoi riprovare.

Claude Code applica anche la catena ai [subagenti](/docs/it/sub-agents). Quando la richiesta di un subagente fallisce, Claude Code prova i tuoi modelli di fallback configurati in ordine, e il subagente continua sul modello che accetta la richiesta. Il modello della tua sessione rimane invariato. Prima della v2.1.247, un fallimento che la catena copriva terminava il subagente.

<h3 id="automatic-model-fallback">
  Fallback automatico del modello
</h3>

Questa sezione copre il fallback basato sul contenuto dai modelli Fable, Opus 5.5 e Opus 5. Per il fallback basato sulla disponibilità quando un modello è sovraccarico o non disponibile, vedere [Catene di modelli di fallback](#fallback-model-chains).

I modelli Fable, Opus 5.5 e Opus 5 vengono eseguiti con classificatori di sicurezza, che il più delle volte contrassegnano il contenuto di sicurezza informatica e biologia. Quando un classificatore contrassegna una richiesta e la categoria contrassegnata ha un modello di fallback, Claude Code riesegue la richiesta su quel modello e mostra un avviso nella trascrizione. Per queste due categorie, il modello di fallback dipende da quale modello ha rifiutato:

* **Fable 5.1, Fable 5 e Opus 5.5**: le richieste contrassegnate per biologia vengono rieseguite su Opus 5, e le richieste contrassegnate per sicurezza informatica vengono rieseguite su Opus 4.8.
* **Opus 5**: le richieste contrassegnate per sicurezza informatica vengono rieseguite su Opus 4.8. Le richieste contrassegnate per biologia terminano con un rifiuto, perché Opus 5 esegue i propri classificatori di biologia senza modello di fallback.

Su Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry, Claude Code risolve questi target attraverso il tuo deployment, e se imposti `ANTHROPIC_DEFAULT_OPUS_MODEL`, le categorie che hanno un fallback vengono rieseguite sul modello bloccato; vedere [Abilitare il fallback su Bedrock, Agent Platform e Foundry](#enable-fallback-on-bedrock-agent-platform-and-foundry).

Dopo un fallback, la sessione continua sul modello di fallback. Per tornare al tuo modello originale, esegui [`/model`](#setting-your-model).

Il fallback basato sulla categoria richiede Claude Code v2.1.219 o successiva. Prima della v2.1.219, ogni richiesta Fable 5 contrassegnata veniva rieseguita sul modello Opus predefinito del tuo provider, e Opus 5 non era una fonte di fallback.

Il modello di fallback viene controllato rispetto a [`availableModels`](#restrict-model-selection). Quando è bloccato, non si verifica alcun fallback. Il rifiuto viene mostrato come un errore normale e il modello della sessione rimane invariato.

<h4 id="check-what-triggered-fallback">
  Verificare cosa ha attivato il fallback
</h4>

Il fallback può attivarsi alla prima richiesta di una sessione, prima di inviare qualcosa di insolito, perché la prima richiesta contiene il contesto dell'area di lavoro come il contenuto di CLAUDE.md e lo stato di git. Un repository che contiene materiale di sicurezza o biologia può attivare il classificatore solo su quel contesto.

Per verificare se le personalizzazioni sono il trigger, avvia una sessione con `claude --safe-mode`, che disabilita le personalizzazioni come CLAUDE.md, skills, server MCP e hooks. Lo stato di git e i nomi delle directory non sono personalizzazioni e sono ancora inclusi.

<h4 id="ask-before-switching">
  Chiedere prima di passare
</h4>

Per decidere cosa accade ogni volta che una richiesta viene contrassegnata, piuttosto che passare automaticamente, esegui `/config` e disattiva **Cambia modelli quando un messaggio viene contrassegnato**, oppure imposta [`switchModelsOnFlag`](/docs/it/settings-reference#switchmodelsonflag) su `false` nel tuo file di impostazioni. Una richiesta contrassegnata mette quindi in pausa la sessione con due opzioni: passare al modello di fallback o modificare il prompt e riprovare sul modello corrente.

Alcuni casi si comportano diversamente:

* Quando la categoria contrassegnata non ha un modello di fallback, come un flag di biologia su Opus 5, Claude Code non mostra il prompt e la richiesta termina con il rifiuto.
* Se entrambi i modelli contrassegnano la stessa richiesta, puoi modificare il prompt e riprovare, o avviare una nuova sessione.
* Su sessioni mobili [Claude Code sul web](/docs/it/claude-code-on-the-web), la modifica e il retry non sono supportati. Cambia modelli o continua la sessione da un browser desktop o dall'app desktop.
* In [modalità non interattiva](/docs/it/cli-reference#cli-flags) e integrazioni SDK che non possono mostrare il prompt, una richiesta contrassegnata termina il turno con un rifiuto.
* Quando il target di fallback è bloccato da [`availableModels`](#restrict-model-selection), Claude Code non mostra il prompt. La richiesta contrassegnata termina con il rifiuto, come il fallback automatico quando il target è bloccato.

<h4 id="enable-fallback-on-bedrock-agent-platform-and-foundry">
  Abilitare il fallback su Bedrock, Agent Platform e Foundry
</h4>

Su [Amazon Bedrock](/docs/it/amazon-bedrock), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai) e [Microsoft Foundry](/docs/it/microsoft-foundry), gli ID modello sono specifici del provider, quindi il fallback automatico funziona solo quando Claude Code può identificare entrambi i modelli coinvolti:

* Claude Code deve riconoscere il modello corrente come fonte di fallback. Fable 5.1 e Fable 5 vengono riconosciuti quando l'ID modello contiene `claude-fable-5`, corrisponde al valore di `ANTHROPIC_DEFAULT_FABLE_MODEL` o è mappato con [`modelOverrides`](#override-model-ids-per-version). Opus 5.5 e Opus 5 vengono riconosciuti dal loro ID modello del provider o da un mapping [`modelOverrides`](#override-model-ids-per-version).
* Il modello di fallback deve risolversi nel tuo deployment. Se imposti `ANTHROPIC_DEFAULT_OPUS_MODEL`, le richieste contrassegnate vengono rieseguite su quel modello per ogni categoria che ha un fallback; un flag di biologia su Opus 5 termina comunque con un rifiuto. Se non lo imposti, le richieste contrassegnate per sicurezza informatica vengono rieseguite su una voce Opus 4.8 nell'elenco dei modelli del provider, e le richieste contrassegnate per biologia da un modello Fable o Opus 5.5 su una voce Opus 5.

Se uno dei modelli non può essere identificato, Claude Code non passa automaticamente. La richiesta contrassegnata termina con un messaggio di rifiuto e puoi cambiare modelli con [`/model`](#setting-your-model) e riprovare. Impostare `ANTHROPIC_DEFAULT_FABLE_MODEL` sul tuo ID modello Fable abilita il riconoscimento di Fable. Impostare `ANTHROPIC_DEFAULT_OPUS_MODEL` su un ID modello Opus fornisce alle categorie contrassegnate un target di fallback, a meno che il pin non nomini un modello al di fuori della famiglia Opus o il modello che ha rifiutato; quindi Claude Code non passa e il rifiuto rimane.

<h4 id="security-research-and-biology-workloads">
  Ricerca sulla sicurezza e carichi di lavoro biologici
</h4>

I carichi di lavoro in sicurezza offensiva o biologia, inclusi test di penetrazione, esercizi Capture the Flag (CTF) e basi di codice adiacenti alla biologia, attivano il fallback frequentemente, spesso alla prima richiesta. Per un lavoro biologico sostanziale su Fable 5.1, Fable 5 o Opus 5.5, Claude Code sposta la sessione a Opus 5 alla prima richiesta contrassegnata, e le successive richieste contrassegnate per biologia terminano in rifiuti lì, perché Opus 5 non ha fallback biologico. Su Opus 5, ricevi quei rifiuti dalla prima richiesta contrassegnata.

Questo è il routing previsto per questi domini, non un flag dell'account. Se la tua organizzazione ha bisogno di capacità di classe Fable per questo lavoro, chiedi al tuo team di account Anthropic informazioni sui programmi di accesso affidabile.

<h3 id="adjust-effort-level">
  Regola il livello di sforzo
</h3>

I [livelli di sforzo](https://platform.claude.com/docs/en/build-with-claude/effort) controllano il ragionamento adattivo, che consente al modello di decidere se e quanto pensare ad ogni passo in base alla complessità dell'attività. Lo sforzo inferiore è più veloce e più economico per attività semplici e circoscritte, mentre lo sforzo superiore fornisce un ragionamento più profondo per problemi complessi.

I livelli di sforzo disponibili dipendono dal modello. I modelli non elencati qui non supportano lo sforzo:

| Modello                                         | Livelli                                 |
| :---------------------------------------------- | :-------------------------------------- |
| Fable 5.1 e Fable 5                             | `low`, `medium`, `high`, `xhigh`, `max` |
| Opus 5.5, Opus 5, Sonnet 5, Opus 4.8 e Opus 4.7 | `low`, `medium`, `high`, `xhigh`, `max` |
| Opus 4.6 e Sonnet 4.6                           | `low`, `medium`, `high`, `max`          |

Se imposti un livello che il modello attivo non supporta, Claude Code fallback al livello supportato più alto pari o inferiore a quello impostato. Ad esempio, `xhigh` viene eseguito come `high` su Opus 4.6. La tua organizzazione o le tue stesse impostazioni possono anche limitare i livelli che un modello offre; vedere [Limiti di sforzo dell'organizzazione](#organization-effort-limits).

Con l'impostazione [`ultracode`](/docs/it/settings-reference#ultracode) disattivata, Claude Code risolve il livello di sforzo della sessione in questo ordine, prendendo il primo che si applica:

1. Una scelta esplicita: la variabile di ambiente [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/it/env-vars#variables), l'avvio con `--effort`, o `/effort` nella sessione ([un `/effort` non interattivo ha effetto più ristretto](#non-interactive-effort))
2. Le tue impostazioni: il livello che hai salvato per il modello o una chiave [`effortLevel`](/docs/it/settings-reference#effortlevel), con la precedenza tra loro e tra i file di impostazioni indicata in [`modelSettings`](/docs/it/settings-reference#modelsettings)
3. Lo sforzo predefinito del modello: `high` su ogni modello che supporta lo sforzo, tranne che Opus 5.5 predefinito a `medium`, Opus 4.7 predefinito a `xhigh` e, quando la tua organizzazione imposta un livello di sforzo predefinito per il suo [modello predefinito dell'organizzazione](#organization-default-model), quel livello è il predefinito quando esegui quel modello

Opus 5.5 inizia a `medium` a meno che una delle fonti sopra non imposti un livello per esso, e un `effortLevel` di livello superiore nel tuo file di impostazioni utente non conta per Opus 5.5. Quella chiave è la forma più vecchia che `/effort` ha scritto prima che Claude Code salvasse i livelli per modello: continua ad applicarsi dove si applicava prima, su Opus 5, Fable 5.1 e modelli precedenti, mentre Opus 5.5 e i modelli rilasciati dopo di esso iniziano al loro predefinito fino a quando non scegli un livello per loro con `/effort` o il selettore `/model`. Un `effortLevel` di livello superiore nelle impostazioni di progetto, locale o gestite, o uno passato con `--settings`, si applica a ogni modello.

Quando imposti `low`, `medium`, `high` o `xhigh` in una sessione interattiva sulla tua macchina, scegli quanto dura selezionando come lo confermi:

* `Enter` nel cursore `/effort` o nel selettore `/model`, o un livello digitato dopo `/effort`: salva il livello come predefinito e applicalo nelle sessioni successive
* `s` nel cursore `/effort` o nel selettore `/model`: applica il livello solo a questa sessione. Richiede Claude Code v2.1.257 o successiva

Claude Code salva il livello per modello, sotto la chiave [`modelSettings`](/docs/it/settings-reference#modelsettings) nelle tue impostazioni utente, quindi ogni modello mantiene il suo livello salvato.

`max` è il livello di ragionamento più profondo. A meno che non lo imposti tramite la variabile di ambiente `CLAUDE_CODE_EFFORT_LEVEL`, Claude Code applica `max` solo alla sessione corrente.

<Note>
  Un livello che scegli dal controllo dello sforzo su un telefono o browser connesso tramite [Remote Control](/docs/it/remote-control#what-connected-devices-see) si applica solo a quella sessione.
</Note>

<span id="non-interactive-effort" />

Quando imposti un livello con `/effort` in un'esecuzione [`-p`](/docs/it/headless), Claude Code lo applica solo a quella sessione e non lo salva come predefinito.

Il menu `/effort` offre anche `ultracode`. Ultracode è un'impostazione di Claude Code piuttosto che un livello di sforzo del modello: invia `xhigh` al modello e inoltre ha Claude orchestrare [flussi di lavoro dinamici](/docs/it/workflows) per attività sostanziali. Per dove può essere impostato in modo persistente, vedere l'impostazione [`ultracode`](/docs/it/settings-reference#ultracode).

Puoi attivare ultracode attraverso uno dei seguenti:

* **`/effort`**: esegui `/effort ultracode`, o selezionalo dal menu
* **Flag `--effort`**: avvia con `claude --effort ultracode`, che avvia la sessione a sforzo `xhigh` con ultracode attivato
* **Impostazione `ultracode`**: imposta [`"ultracode": true`](/docs/it/settings-reference#ultracode) in un file di impostazioni, con `--settings`, o in una richiesta di controllo Agent SDK. Una richiesta [`applyFlagSettings()`](/docs/it/agent-sdk/typescript#applyflagsettings) accetta anche `effortLevel: "ultracode"`
* **Selettore `/model`**: sposta il cursore dello sforzo su `ultracode` con i tasti freccia mentre scegli un modello. Claude Code lo attiva per la sessione corrente, anche quando salvi quel modello come predefinito

Passare `ultracode` al flag `--effort` o al valore Agent SDK `effortLevel` richiede Claude Code v2.1.203 o successiva. Prima della v2.1.203, `--effort ultracode` stampava `Unknown --effort value 'ultracode'` e la sessione iniziava allo sforzo predefinito.

L'impostazione `effortLevel` persistente e la variabile di ambiente `CLAUDE_CODE_EFFORT_LEVEL` non accettano `ultracode`. Quando `CLAUDE_CODE_EFFORT_LEVEL` è impostato su un livello diverso da `xhigh`, le richieste vengono eseguite a quel livello e l'orchestrazione del flusso di lavoro di ultracode rimane inattiva. Selezionare ultracode mostra quindi un avviso che la variabile di ambiente sostituisce lo sforzo per la sessione.

<span id="when-ultracode-is-available" />

Ultracode non è disponibile quando:

* [I flussi di lavoro sono disattivati](/docs/it/workflows#turn-workflows-off)
* Il modello non supporta lo sforzo `xhigh`
* Un [limite di sforzo](#organization-effort-limits) inferiore a `xhigh` si applica al modello

In questi casi `--effort ultracode` avvia la sessione con ultracode disattivato, al livello di sforzo più alto che il modello e qualsiasi limite consentono, fino a `xhigh`.

<h4 id="choose-an-effort-level">
  Scegli un livello di sforzo
</h4>

Ogni livello scambia la spesa di token rispetto alla capacità. Il predefinito si adatta alla maggior parte dei compiti di codifica; regola quando desideri un equilibrio diverso.

| Livello     | Quando usarlo                                                                                                                                                           |
| :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `low`       | Riservato per attività brevi, circoscritte e sensibili alla latenza che non sono sensibili all'intelligenza                                                             |
| `medium`    | Riduce l'utilizzo di token per il lavoro sensibile ai costi che può scambiare un po' di intelligenza. Il predefinito su Opus 5.5                                        |
| `high`      | Bilancia l'utilizzo di token e l'intelligenza. Il predefinito su ogni modello tranne Opus 5.5 e Opus 4.7                                                                |
| `xhigh`     | Ragionamento più profondo a spesa di token più elevata. Il predefinito su Opus 4.7                                                                                      |
| `max`       | Può migliorare le prestazioni su attività impegnative ma può mostrare rendimenti decrescenti ed è soggetto a eccesso di riflessione. Prova prima di adottare ampiamente |
| `ultracode` | Un'impostazione di Claude Code che pianifica un [flusso di lavoro dinamico](/docs/it/workflows) per ogni attività sostanziale con ragionamento `xhigh` per messaggio         |

La scala dello sforzo è calibrata per modello, quindi lo stesso nome di livello non rappresenta lo stesso valore sottostante tra i modelli.

<h4 id="use-ultrathink-for-one-off-deep-reasoning">
  Usa ultrathink per il ragionamento profondo una tantum
</h4>

Includi `ultrathink` in qualsiasi punto del tuo prompt per richiedere un ragionamento più profondo su quel turno senza modificare l'impostazione dello sforzo della sessione. Claude Code riconosce la parola chiave e aggiunge un'istruzione nel contesto. Il livello di sforzo inviato all'API rimane invariato. Claude Code passa altre frasi come "think", "think hard" e "think more" come testo di prompt ordinario e non le riconosce come parole chiave.

<h4 id="set-the-effort-level">
  Imposta il livello di sforzo
</h4>

Puoi modificare lo sforzo attraverso uno dei seguenti:

* **`/effort`**: esegui `/effort` senza argomenti per aprire un cursore interattivo, `/effort` seguito da un nome di livello per impostarlo direttamente, o `/effort auto` per cancellare il tuo livello salvato per il modello attivo. Puoi eseguirlo mentre Claude sta lavorando, e una volta confermato l'[avviso della cache](/docs/it/prompt-caching#changing-effort-level), se Claude Code ne mostra uno, Claude Code applica il nuovo livello alla richiesta successiva nel turno
* **In `/model`**: usa i tasti freccia sinistra/destra per regolare il cursore dello sforzo quando selezioni un modello
* **Flag `--effort`**: passa un nome di livello per impostarlo per una singola sessione quando avvii Claude Code
* **Variabile di ambiente**: imposta `CLAUDE_CODE_EFFORT_LEVEL` su un nome di livello o `auto`
* **Impostazioni**: imposta un livello per modello in [`modelSettings`](/docs/it/settings-reference#modelsettings), o imposta [`effortLevel`](/docs/it/settings-reference#effortlevel) su `low`, `medium`, `high` o `xhigh` come predefinito per i modelli senza uno. `max` non è accettato in nessuna delle due chiavi, e `ultracode` ha la sua propria chiave [`ultracode`](/docs/it/settings-reference#ultracode)
* **Da un dispositivo connesso**: in una sessione [Remote Control](/docs/it/remote-control#what-connected-devices-see), scegli un livello dal controllo dello sforzo sul tuo telefono o nel tuo browser. Il livello si applica solo alla sessione corrente. Richiede Claude Code v2.1.234 o successiva
* **Frontmatter di skill e subagente**: imposta `effort` in un file markdown [skill](/docs/it/skills#frontmatter-reference) o [subagente](/docs/it/sub-agents#supported-frontmatter-fields) per sostituire il livello di sforzo quando quella skill o subagente viene eseguito

Lo sforzo del frontmatter si applica quando quella skill o subagente è attivo, sostituendo il livello della sessione ma non la variabile di ambiente. Un [`maxEffortLevel`](/docs/it/settings-reference#maxeffortlevel) o [limite di sforzo dell'organizzazione](#organization-effort-limits) limita comunque il livello a cui la skill o il subagente viene eseguito.

Se imposti `effortLevel` nelle [impostazioni gestite](/docs/it/managed-settings), Claude Code lo applica al passo delle impostazioni dell'[ordine di risoluzione dello sforzo](#adjust-effort-level), e gli utenti possono comunque modificare il livello con `/effort` o `--effort`. Per mantenere gli utenti a o sotto un livello, imposta [`maxEffortLevel`](/docs/it/settings-reference#maxeffortlevel).

Il cursore dello sforzo appare in `/model` quando è selezionato un modello supportato. Il livello di sforzo corrente è anche mostrato nell'intestazione della sessione accanto al nome del modello, ad esempio "con sforzo basso", in modo che tu possa confermare quale impostazione è attiva senza aprire `/model`. Il piè di pagina mostra anche brevemente il livello di sforzo all'avvio e quando cambia.

<h4 id="adaptive-reasoning-and-fixed-thinking-budgets">
  Ragionamento adattivo e budget di pensiero fissi
</h4>

Il ragionamento adattivo rende il pensiero opzionale ad ogni passo, quindi Claude può rispondere più velocemente ai prompt di routine e riservare il pensiero più profondo ai passi che ne traggono beneficio. Se desideri che Claude pensi più o meno spesso di quanto il livello corrente produce, puoi dirlo direttamente nel tuo prompt o in `CLAUDE.md`; il modello risponde a quella guida entro la sua impostazione di sforzo.

I modelli Fable, Sonnet 5 e Opus 4.7 e successivi utilizzano sempre il ragionamento adattivo. La modalità di budget di pensiero fisso e `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING` non si applicano a loro.

Su Opus 4.6 e Sonnet 4.6, puoi impostare `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` per tornare al precedente budget di pensiero fisso controllato da `MAX_THINKING_TOKENS`. Vedere [variabili di ambiente](/docs/it/env-vars).

<h3 id="extended-thinking">
  Pensiero esteso
</h3>

Il pensiero esteso è il ragionamento che Claude emette prima di rispondere. Sui modelli che supportano il [ragionamento adattivo](#adjust-effort-level), il livello di sforzo è il controllo primario per quanto pensiero accade; le impostazioni di seguito attivano o disattivano il pensiero e controllano come viene visualizzato. Con il pensiero disattivato su Anthropic API, Claude Code invia sforzo `high` invece di un livello superiore ai modelli che sa [non accettano quella combinazione](/docs/it/errors#effort-isnt-available-with-thinking-turned-off), come Opus 5.

| Controllo                                    | Come impostarlo                                                                                                                                                                                                                                                                                                                                                                                                             |
| :------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Attiva/disattiva per la sessione corrente    | Premi `Option+T` su macOS o `Alt+T` su Windows e Linux                                                                                                                                                                                                                                                                                                                                                                      |
| Imposta il predefinito globale               | Esegui `/config` e attiva/disattiva la modalità di pensiero. Salvato come `alwaysThinkingEnabled` in `~/.claude/settings.json`                                                                                                                                                                                                                                                                                              |
| Disabilita tramite una variabile di ambiente | Imposta [`MAX_THINKING_TOKENS=0`](/docs/it/env-vars), che disattiva il pensiero su Anthropic API tranne sui modelli Opus 5.5 e Fable. Su [provider di terze parti](/docs/it/third-party-integrations), Claude Code omette il parametro `thinking`, e i modelli di ragionamento adattivo potrebbero comunque pensare. Altri valori si applicano solo con un [budget di pensiero fisso](#adaptive-reasoning-and-fixed-thinking-budgets) |

Non puoi disattivare il pensiero sui modelli Opus 5.5 o Fable. L'attivazione/disattivazione della sessione, `alwaysThinkingEnabled` e `MAX_THINKING_TOKENS=0` non hanno effetto lì, e il modello decide per passo quanto pensare in base al livello di sforzo.

Claude Code comprime l'output di pensiero per impostazione predefinita. Premi `Ctrl+O` per attivare/disattivare la modalità dettagliata e vedi il ragionamento come testo grigio in corsivo. Le sessioni interattive su Anthropic API ricevono blocchi di pensiero redatti per impostazione predefinita, quindi imposta `showThinkingSummaries: true` nelle [impostazioni](/docs/it/settings) se desideri i riassunti completi disponibili quando espandi. Ti viene addebitato per tutti i token di pensiero generati, anche quando compressi o redatti.

<h3 id="extended-context">
  Contesto esteso
</h3>

Fable 5.1, Fable 5, Sonnet 5, Opus 4.6 e successivi e Sonnet 4.6 supportano una [finestra di contesto di 1 milione di token](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) per sessioni lunghe con basi di codice di grandi dimensioni.

Su Anthropic API, Fable 5.1, Fable 5, Sonnet 5 e Opus 4.7 e successivi vengono eseguiti con la finestra 1M su ogni piano, incluso Pro. Non selezioni una variante `[1m]` o attivi crediti di utilizzo per la finestra 1M su questi modelli. L'utilizzo di Fable stesso può fatturare ai crediti di utilizzo su alcuni piani; vedere [Fable e crediti di utilizzo](#fable-and-usage-credits).

Opus 4.6 e Sonnet 4.6 raggiungono 1M solo attraverso la loro variante `[1m]`, e l'accesso a quella variante dipende dal tuo piano. Su piani Max, Team ed Enterprise, inclusi sia i posti Team Standard che Team Premium, Opus 4.6 con contesto 1M è incluso nel tuo abbonamento. Sonnet 4.6 con contesto 1M richiede [crediti di utilizzo](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) su ogni piano di abbonamento, incluso Max.

| Piano                     | Opus 4.6 con contesto 1M                                                                                          | Sonnet 4.6 con contesto 1M                                                                                        |
| ------------------------- | ----------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| Max, Team e Enterprise    | Incluso nell'abbonamento                                                                                          | Richiede [crediti di utilizzo](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) |
| Pro                       | Richiede [crediti di utilizzo](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) | Richiede [crediti di utilizzo](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) |
| API e pagamento a consumo | Accesso completo                                                                                                  | Accesso completo                                                                                                  |

Claude Code controlla questi requisiti del piano solo quando si connette direttamente a Anthropic API. Se punti `ANTHROPIC_BASE_URL` a un [gateway LLM](/docs/it/llm-gateway#subscriptions-and-gateways) e il tuo accesso salvato a claude.ai rimane la credenziale attiva, Claude Code non controlla i crediti di utilizzo del tuo piano. Le opzioni `[1m]` rimangono disponibili in `/model`, e il gateway decide se la richiesta ha successo. Prima della v2.1.229, Claude Code rifiutava `/model sonnet[1m]` in quella configurazione quando non poteva confermare i crediti di utilizzo sull'account.

Per disattivare il contesto 1M, imposta `CLAUDE_CODE_DISABLE_1M_CONTEXT=1`. Claude Code rimuove le varianti di modello 1M dal selettore di modelli. Su modelli con una finestra 1M nativa, come Sonnet 5 e i modelli Fable, tratta anche il modello come avente una finestra di contesto di 200K:

* Con la compattazione automatica attivata, le sessioni si compattano al confine 200K attraverso la [compattazione automatica](#set-the-auto-compact-window). Impostare la finestra di compattazione automatica sopra 200K non solleva il mantenimento, perché Claude Code limita quella finestra alla finestra di contesto del modello.
* Con la compattazione automatica disattivata, le sessioni si fermano al confine 200K con l'[errore di limite di contesto](/docs/it/errors#prompt-is-too-long) invece di compattarsi.

Prima della v2.1.223, Claude Code manteneva solo le sessioni Sonnet 5, Opus 4.8 e Opus 5 a 200K. Vedere [variabili di ambiente](/docs/it/env-vars).

La finestra di contesto 1M utilizza i prezzi standard del modello senza premio per i token oltre 200K. Per i piani in cui il contesto esteso è incluso nel tuo abbonamento, l'utilizzo rimane coperto dal tuo abbonamento. Per i piani che accedono al contesto esteso tramite crediti di utilizzo, i token vengono fatturati ai crediti di utilizzo.

Se il tuo account supporta il contesto 1M, l'opzione appare nel selettore `/model` nelle versioni più recenti di Claude Code. Se non la vedi, prova a riavviare la tua sessione.

Puoi anche usare il suffisso `[1m]` con alias di modello o nomi di modello completi:

```text theme={null}
# Usa l'alias opus[1m] o sonnet[1m]
/model opus[1m]
/model sonnet[1m]

# O aggiungi [1m] a un nome di modello completo
/model claude-opus-4-8[1m]
```

<h4 id="sonnet-5-context-window">
  Finestra di contesto di Sonnet 5
</h4>

Su Anthropic API, Sonnet 5 viene sempre eseguito con la finestra di contesto 1M. Non c'è variante 200K, nessun suffisso `[1m]` da selezionare e nessun credito di utilizzo richiesto su nessun piano. Le sessioni si compattano automaticamente prima che la finestra si riempia, a circa 967K token per impostazione predefinita; imposta [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/it/env-vars) per scegliere una soglia diversa.

Due configurazioni limitano la finestra a 200K:

* **Gateway LLM**: quando `ANTHROPIC_BASE_URL` punta a un [gateway](/docs/it/llm-gateway), Claude Code non può verificare il supporto 1M. Per usare la finestra completa, seleziona Sonnet 5 (1M context) nel selettore di modelli, che mappa a `sonnet[1m]`.
* **`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`**: mantiene le sessioni su ogni modello con una finestra 1M nativa a una finestra 200K; vedere [Contesto esteso](#extended-context) per come il mantenimento viene applicato. Utile per i deployment che devono limitare il contesto.

<h2 id="context-window-and-auto-compaction">
  Finestra di contesto e auto-compattazione
</h2>

La finestra auto-compact è il livello di riempimento della finestra di contesto prima che Claude Code compatti la conversazione. Per informazioni su cosa la compattazione mantiene e scarta per ogni meccanismo, vedere [What survives compaction](/docs/it/context-window#what-survives-compaction).

<h3 id="set-the-auto-compact-window">
  Impostare la finestra auto-compact
</h3>

È possibile impostare la finestra auto-compact in tre posizioni:

* **Per questa sessione e quelle successive**: eseguire `/autocompact` con un valore, come `/autocompact 500k`. Claude Code lo salva nelle impostazioni utente come [`autoCompactWindow`](/docs/it/settings-reference#autocompactwindow) e lo applica alla sessione corrente; se un [ambito di impostazioni](/docs/it/settings#settings-precedence) con priorità più alta, come le impostazioni gestite, imposta la chiave, il comando salva il valore ma la sessione mantiene la finestra di tale ambito, e il comando lo comunica. Eseguire `/autocompact auto` per tornare alla finestra ottimizzata per il modello.
* **Per un singolo avvio**: passare [`--autocompact`](/docs/it/cli-reference#cli-flags) all'avvio di Claude Code. Il flag sostituisce l'impostazione salvata per tale avvio senza modificarla, e `claude --autocompact auto` esegue la sessione alla finestra ottimizzata anche se l'impostazione salvata ha un valore. A differenza di `/autocompact`, il flag non è prevenuto da un ambito di impostazioni con priorità più alta, come le impostazioni gestite.
* **In script e ambienti cloud**: impostare [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/it/env-vars). Mentre è impostato, ha precedenza sul comando, sul flag e sull'impostazione, e `/autocompact` segnala l'override invece di modificare la finestra.

Il comando e il flag accettano una dimensione della finestra da 100K a 1M token, in una qualsiasi di queste forme:

* Un conteggio di token semplice, come `200000`
* Un suffisso `k` o `M`, come `500k` o `1M`
* Un numero semplice da 100 a 1000, che significa migliaia, quindi `200` imposta 200.000

La variabile di ambiente accetta solo il conteggio di token semplice. Claude Code limita la finestra alla finestra di contesto del modello.

<h3 id="default-auto-compact-thresholds">
  Soglie auto-compact predefinite
</h3>

Se non si imposta una finestra auto-compact, Claude Code compatta quando la conversazione raggiunge il limite di contesto del modello, tranne in queste sessioni:

* Le [sessioni cloud](/docs/it/claude-code-on-the-web) si compattano mentre la conversazione si avvicina al limite del modello
* Sonnet 4.6 e Opus 4.6 senza [contesto esteso](#extended-context) si compattano al limite di 200K, così come Opus 4.8 e versioni successive quando vengono eseguiti con una finestra di contesto di 200K, come su Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry
* Quando si imposta [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/it/env-vars), i modelli con una finestra nativa di 1M, come Sonnet 5 e i modelli Fable, si compattano al limite di 200K
* I modelli in esecuzione con una finestra nativa di 1M, come Sonnet 5, i modelli Fable e Opus 4.7 e versioni successive su Anthropic API, si compattano prima che la finestra si riempia, a circa 967K token per impostazione predefinita. Su Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry, [Pin models for third-party deployments](#pin-models-for-third-party-deployments) indica quali modelli vengono eseguiti con quella finestra; per le configurazioni che assegnano a Sonnet 5 200K, vedere [Sonnet 5 context window](#sonnet-5-context-window)
* Le sessioni su un ID modello che Claude Code non riconosce, come un alias [LLM gateway](/docs/it/llm-gateway), si compattano alla finestra di contesto che Claude Code assume per l'ID; vedere [Correct the window for a gateway or custom model ID](#correct-the-window-for-a-gateway-or-custom-model-id)

<h3 id="correct-the-window-for-a-gateway-or-custom-model-id">
  Correggere la finestra per un gateway o un ID modello personalizzato
</h3>

Su un [LLM gateway](/docs/it/llm-gateway) o un'altra distribuzione personalizzata, Claude Code può assumere una finestra di contesto per l'ID modello che differisce dalla finestra reale del modello, indipendentemente dal fatto che risolva l'ID a un modello Claude o meno. Impostare [`CLAUDE_CODE_MAX_CONTEXT_TOKENS`](/docs/it/env-vars) alla finestra che Claude Code dovrebbe assumere invece.

Il modo in cui la variabile si applica dipende dall'ID. Claude Code tratta un ID come un provider o un'ortografia personalizzata quando non inizia con `claude-`, in qualsiasi maiuscola/minuscola, o quando porta un suffisso che Claude Code rimuove durante la lettura dell'ID, come la data `@YYYYMMDD` utilizzata su Google Cloud's Agent Platform. Prima della v2.1.259, Claude Code non contava un suffisso rimosso, quindi un ID `claude-` non riconosciuto con un suffisso di data era trattato come un nome `claude-` semplice.

Un provider non riconosciuto o un'ortografia personalizzata, la stessa ortografia con `[1m]` e ogni altro ID sono tre casi separati:

* Se Claude Code non può risolvere un provider o un'ortografia personalizzata a un modello che riconosce e l'ID non contiene `[1m]`, la variabile si applica direttamente e la compattazione proattiva continua alla finestra dichiarata.
* Se Claude Code non può risolvere un provider o un'ortografia personalizzata a un modello che riconosce e l'ID contiene `[1m]`, in qualsiasi maiuscola/minuscola, Claude Code assume una finestra di 1M per esso e la variabile non si applica da sola. Per correggere la finestra mantenendo la compattazione proattiva, impostare anche [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/it/env-vars). Con quella variabile impostata, Claude Code dimensiona l'ID come la stessa ortografia senza `[1m]`, quindi `CLAUDE_CODE_MAX_CONTEXT_TOKENS` si applica quando si applicherebbe a tale ortografia senza tag.

  Con una finestra dichiarata superiore a 200K, Claude Code mostra quindi un [avviso di avvio](/docs/it/errors#the-200k-limit-isnt-enforced) che il limite di 200K non è applicato. L'avviso è previsto in questa configurazione.
* Se l'ID si risolve a un modello che Claude Code riconosce, o l'ID è un nome `claude-` semplice senza suffisso per Claude Code da rimuovere, in qualsiasi maiuscola/minuscola, la variabile ha effetto solo quando si imposta anche [`DISABLE_COMPACT`](/docs/it/env-vars), che disabilita tutta la compattazione.

  Ad esempio, un ID che contiene un nome di modello Claude che Claude Code conosce, come `anthropic/claude-opus-4-8`, `us.anthropic.claude-…-v1:0`, o il datato `claude-sonnet-4-5@20250929`, si risolve a quel modello. Questo include gli ID che contengono anche `[1m]`: Claude Code risolve `claude-opus-4-8[1m]` a Opus 4.8 anche con `CLAUDE_CODE_DISABLE_1M_CONTEXT` impostato.

Per un ID modello che Claude Code non riconosce, impostare [`CLAUDE_CODE_DISABLE_UNKNOWN_MODEL_WINDOW_ENFORCEMENT=1`](/docs/it/env-vars) per fare in modo che Claude Code si compatti solo dopo che l'API rifiuta la conversazione con un [errore di lunghezza eccessiva che Claude Code riconosce](/docs/it/errors#prompt-is-too-long). Claude Code non esegue quel recupero quando un gateway [riscrive l'errore](/docs/it/llm-gateway-connect#troubleshoot-gateway-errors) con una formulazione che Claude Code non riconosce.

<h2 id="checking-your-current-model">
  Verifica del modello corrente
</h2>

È possibile vedere quale modello stai utilizzando attualmente in due posizioni:

* Nella [riga di stato](/docs/it/statusline), se ne hai una configurata
* In `/status`, che visualizza anche le informazioni del tuo account

<h2 id="add-a-custom-model-option">
  Aggiungere un'opzione di modello personalizzato
</h2>

Utilizzare `ANTHROPIC_CUSTOM_MODEL_OPTION` per aggiungere una singola voce personalizzata al selettore `/model` senza sostituire gli alias incorporati. Questo è utile per testare ID di modello che Claude Code non elenca per impostazione predefinita. Per le distribuzioni di gateway LLM, Claude Code può popolare il selettore dall'endpoint `/v1/models` del gateway quando `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1` è impostato, quindi questa variabile è necessaria solo quando la scoperta è disabilitata o non restituisce il modello desiderato. Vedere [gateway model discovery](/docs/it/llm-gateway-protocol#model-discovery).

Per elencare invece diversi modelli, nel vostro ordine e con etichette che scegliete, impostare [`modelPicker`](/docs/it/settings-reference#modelpicker). La sua voce specifica quali righe il selettore mantiene quando questo lineup sostituisce quello incorporato.

Questo esempio imposta tutte e tre le variabili per rendere selezionabile una distribuzione Opus instradata tramite gateway. Claude Code legge le variabili di ambiente all'avvio, quindi eseguire gli export prima di lanciare `claude`, o riavviare una sessione esistente per applicarle:

```bash theme={null}
export ANTHROPIC_CUSTOM_MODEL_OPTION="my-gateway/claude-opus-5-5"
export ANTHROPIC_CUSTOM_MODEL_OPTION_NAME="Opus via Gateway"
export ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION="Custom deployment routed through the internal LLM gateway"
```

`ANTHROPIC_CUSTOM_MODEL_OPTION_NAME` e `ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION` sono facoltativi:

* Se omettete il nome, la voce mostra il nome del modello quando Claude Code [riconosce l'ID](#customize-pinned-model-display-and-capabilities), e l'ID del modello altrimenti.
* Se omettete la descrizione, Claude Code utilizza `Custom model (<model-id>)`.

Claude Code elenca la voce personalizzata dopo le voci incorporate, e qualsiasi riga [`modelPicker`](/docs/it/settings-reference#modelpicker) che aggiungete viene dopo di essa.

Claude Code salta la convalida per l'ID del modello impostato in `ANTHROPIC_CUSTOM_MODEL_OPTION`, quindi è possibile utilizzare qualsiasi stringa che l'endpoint API accetta.

Quando [`availableModels`](#restrict-model-selection) è impostato, includere l'ID del modello personalizzato anche nell'elenco di autorizzazione. Altrimenti Claude Code filtra la voce personalizzata dal selettore e rifiuta una selezione `--model` di essa come qualsiasi altro modello escluso.

Un ID personalizzato che incorpora un nome di famiglia, come `my-gateway/claude-opus-5-5`, conta come una voce specifica per quella famiglia e disabilita il suo wildcard, quindi elencare anche le versioni che intendete mantenere selezionabili. Vedere [Comportamento di unione](#merge-behavior).

<h2 id="environment-variables">
  Variabili di ambiente
</h2>

Utilizzare le seguenti variabili di ambiente per controllare i nomi dei modelli a cui gli alias si mappano. Ogni valore deve essere un nome di modello completo, o l'identificatore equivalente per il provider API. Per scegliere il modello su cui iniziano le sessioni, impostare [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions), che questa tabella omette.

| Variabile di ambiente            | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| -------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_DEFAULT_FABLE_MODEL`  | Il modello da utilizzare per `fable`, e l'ID del modello che Claude Code riconosce come modello Fable per il [fallback automatico del modello](#automatic-model-fallback) su provider di terze parti                                                                                                                                                                                                                                                                                                                                          |
| `ANTHROPIC_DEFAULT_OPUS_MODEL`   | Il modello da utilizzare per `opus`, o per `opusplan` quando Plan Mode è attivo.                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `ANTHROPIC_DEFAULT_SONNET_MODEL` | Il modello da utilizzare per `sonnet`, o per `opusplan` quando Plan Mode non è attivo.                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `ANTHROPIC_DEFAULT_HAIKU_MODEL`  | Il modello da utilizzare per `haiku`, o [funzionalità in background](/docs/it/costs#background-token-usage)                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `CLAUDE_CODE_SUBAGENT_MODEL`     | Il modello predefinito per [subagents](/docs/it/sub-agents#choose-a-model), [team di agenti](/docs/it/agent-teams#specify-teammates-and-models) compagni di squadra, e agenti [workflow](/docs/it/workflows) che non sono assegnati a un modello in un altro modo. Accetta un alias come `haiku` o un nome di modello completo. Un modello per invocazione o il campo `model` di una definizione, incluso `inherit`, ha la precedenza. Per modificare questo, impostare [`CLAUDE_CODE_SUBAGENT_MODEL_FORCE`](/docs/it/sub-agents#run-every-subagent-on-one-model) |

Nota: `ANTHROPIC_SMALL_FAST_MODEL` è deprecato a favore di `ANTHROPIC_DEFAULT_HAIKU_MODEL`.

<h3 id="pin-models-for-third-party-deployments">
  Fissare i modelli per distribuzioni di terze parti
</h3>

Quando si distribuisce Claude Code tramite [Amazon Bedrock](/docs/it/amazon-bedrock), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai), [Microsoft Foundry](/docs/it/microsoft-foundry), o [Claude Platform on AWS](/docs/it/claude-platform-on-aws), fissare le versioni dei modelli prima di distribuire agli utenti.

Senza fissaggio, Claude Code utilizza alias di modelli come `fable`, `opus`, `sonnet` e `haiku` che si risolvono in un ID di modello predefinito incorporato per ogni provider. Tale impostazione predefinita può rimanere indietro rispetto alla versione più recente di Anthropic, e il modello a cui punta potrebbe non essere ancora abilitato nell'account di un utente. Quando l'impostazione predefinita non è disponibile, gli utenti Amazon Bedrock e Google Cloud's Agent Platform vedono un avviso e la sessione ricade nella versione precedente del modello predefinito, o nel modello Sonnet predefinito quando l'impostazione predefinita è un modello Opus e nessuna versione di Opus è disponibile. Gli utenti Microsoft Foundry vedono errori invece, perché Microsoft Foundry non ha alcun controllo di avvio equivalente.

Su Amazon Bedrock e Google Cloud's Agent Platform, un utente che avvia la sessione su una versione specifica di Sonnet oppure Opus, ad esempio con `--model`, `ANTHROPIC_MODEL`, o l'impostazione `model`, fissa quella versione come impostazione predefinita della sessione per l'alias corrispondente: il controllo di avvio salta l'impostazione predefinita incorporata che sostituisce e non mostra alcun avviso di fallback. Prima della v2.1.211, il controllo veniva eseguito e poteva mostrare un avviso anche quando un modello di sessione era configurato esplicitamente.

<Warning>
  Impostare le variabili di ambiente del modello su ID di versione specifici come parte della configurazione iniziale. Il fissaggio consente di controllare quando i vostri utenti passano a un nuovo modello.
</Warning>

Utilizzare le seguenti variabili di ambiente con ID di modello specifici della versione per il provider:

| Provider                      | Esempio                                                              |
| :---------------------------- | :------------------------------------------------------------------- |
| Amazon Bedrock                | `export ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'` |
| Google Cloud's Agent Platform | `export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'`              |
| Microsoft Foundry             | `export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'`              |

Applicare lo stesso modello per `ANTHROPIC_DEFAULT_FABLE_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL` e `ANTHROPIC_DEFAULT_HAIKU_MODEL`. Per gli ID di modello attuali e legacy su tutti i provider, vedere [Panoramica dei modelli](https://platform.claude.com/docs/en/about-claude/models/overview). Per aggiornare gli utenti a una nuova versione del modello, aggiornare queste variabili di ambiente e ridistribuire.

Per abilitare il [contesto esteso](#extended-context) per un modello fissato, aggiungere `[1m]` all'ID del modello in `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, o `ANTHROPIC_DEFAULT_FABLE_MODEL`:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8[1m]'
```

Con il suffisso `[1m]`, la finestra di contesto 1M si applica a tutto l'utilizzo dell'alias fissato, inclusa la fase Opus in modalità piano di [`opusplan`](#opusplan-model-setting) e [subagents](/docs/it/sub-agents#choose-a-model) il cui frontmatter `model` nomina l'alias.

* Claude Code rimuove il suffisso prima di inviare l'ID del modello al provider.
* Aggiungere `[1m]` solo quando il modello sottostante [supporta il contesto 1M](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model).
* Il suffisso viene letto per variabile, non per modello. Su Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry, un ID di modello senza `[1m]` in una variabile utilizza il contesto 200K anche se un'altra variabile imposta lo stesso modello con il suffisso. Sonnet 5 viene sempre eseguito con la finestra 1M su questi provider e non ha mai bisogno del suffisso.

<Note>
  Un elenco di autorizzazione `availableModels` fornito tramite [MDM o un file di impostazioni gestite](/docs/it/managed-settings#delivery-mechanisms) si applica comunque quando si utilizzano provider di terze parti; [le impostazioni gestite dal server non vengono fornite lì](/docs/it/server-managed-settings#platform-availability).

  Il filtraggio corrisponde a un alias di modello come `opus`, a un prefisso di versione come `claude-opus-4-8`, o all'ID di modello completo in forma di provider. I prefissi specifici del provider come `us.anthropic.` non vengono rimossi, quindi per consentire un modello specifico, elencare il suo ID completo in forma di provider, o mapparlo tramite [`modelOverrides`](#override-model-ids-per-version). Qualsiasi suffisso `[1m]` viene rimosso sia dalla voce dell'elenco di autorizzazione che dal modello richiesto prima della corrispondenza.
</Note>

<h3 id="customize-pinned-model-display-and-capabilities">
  Personalizzare la visualizzazione e le capacità del modello fissato
</h3>

Quando si fissa un modello su un provider di terze parti, la sua riga nel selettore `/model` mostra il nome del modello per impostazione predefinita se Claude Code riconosce l'ID fissato, e l'ID grezzo altrimenti:

* **Riconosciuto**: l'ID esatto di un modello che Claude Code conosce, come il suo ID API Anthropic o la forma del provider o del gateway, con o senza il suffisso `[1m]`. Fissare `us.anthropic.claude-sonnet-4-5-20250929-v1:0` e la riga legge `Sonnet 4.5`.
* **Non riconosciuto**: qualsiasi altro ID, come un ARN di profilo di inferenza dell'applicazione o una versione del modello che Claude Code non conosce, a meno che una voce [`modelOverrides`](#override-model-ids-per-version) non mappi un modello a quella stringa esatta. Su Microsoft Foundry, i nomi di distribuzione sono definiti dall'utente, quindi Claude Code non riconosce mai un ID fissato lì, mappato o meno, e la riga mostra il nome di distribuzione per impostazione predefinita.

Quando una riga mostra il nome del modello, la sua descrizione predefinita include l'ID fissato in modo da poter comunque vedere quale ID è fissato.

Claude Code potrebbe anche non riconoscere quali funzionalità supporta un modello fissato. È possibile impostare il nome di visualizzazione e la descrizione da soli e dichiarare le capacità con variabili di ambiente complementari per ogni modello fissato.

Queste variabili hanno effetto su provider di terze parti come Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry. Le variabili `_NAME` e `_DESCRIPTION` hanno effetto anche quando `ANTHROPIC_BASE_URL` punta a un [gateway LLM](/docs/it/llm-gateway). Non hanno effetto quando si effettua la connessione direttamente a `api.anthropic.com`.

| Variabile di ambiente                                 | Descrizione                                                                                                                                                                                           |
| ----------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_NAME`                   | Nome di visualizzazione per il modello Opus fissato nel selettore `/model`. Quando non impostato, la riga mostra il nome del modello se Claude Code riconosce l'ID fissato, e l'ID fissato altrimenti |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_DESCRIPTION`            | Descrizione di visualizzazione per il modello Opus fissato nel selettore `/model`. Quando non impostato, la riga mostra una descrizione predefinita che inizia con `Custom Opus model`                |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES` | Elenco separato da virgole delle capacità che il modello Opus fissato supporta                                                                                                                        |

Gli stessi suffissi `_NAME`, `_DESCRIPTION` e `_SUPPORTED_CAPABILITIES` sono disponibili per `ANTHROPIC_DEFAULT_SONNET_MODEL`, `ANTHROPIC_DEFAULT_HAIKU_MODEL`, `ANTHROPIC_DEFAULT_FABLE_MODEL` e `ANTHROPIC_CUSTOM_MODEL_OPTION`.

Claude Code abilita funzionalità come [livelli di sforzo](#adjust-effort-level) e [extended thinking](#extended-thinking) abbinando l'ID del modello rispetto a modelli noti. Gli ID specifici del provider come ARN Amazon Bedrock o nomi di distribuzione personalizzati spesso non corrispondono a questi modelli, lasciando le funzionalità supportate disabilitate. Impostare `_SUPPORTED_CAPABILITIES` per dire a Claude Code quali funzionalità il modello effettivamente supporta:

| Valore di capacità     | Abilita                                                                                           |
| ---------------------- | ------------------------------------------------------------------------------------------------- |
| `effort`               | [Livelli di sforzo](#adjust-effort-level) e il comando `/effort`                                  |
| `xhigh_effort`         | Il livello di sforzo `xhigh`                                                                      |
| `max_effort`           | Il livello di sforzo `max`                                                                        |
| `thinking`             | [Extended thinking](#extended-thinking)                                                           |
| `adaptive_thinking`    | Ragionamento adattivo che alloca dinamicamente il pensiero in base alla complessità dell'attività |
| `interleaved_thinking` | Pensiero tra le chiamate di strumento                                                             |

Quando `_SUPPORTED_CAPABILITIES` è impostato, Claude Code abilita le capacità elencate e disabilita quelle non elencate per il modello fissato corrispondente. Quando la variabile non è impostata, Claude Code ricade sulla rilevazione incorporata basata sull'ID del modello.

Questo esempio fissa Opus a un ARN di modello personalizzato Amazon Bedrock, imposta un nome amichevole e dichiara le sue capacità:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='arn:aws:bedrock:us-east-1:123456789012:custom-model/abc'
export ANTHROPIC_DEFAULT_OPUS_MODEL_NAME='Opus via Bedrock'
export ANTHROPIC_DEFAULT_OPUS_MODEL_DESCRIPTION='Opus 4.7 routed through a Bedrock custom endpoint'
export ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES='effort,xhigh_effort,max_effort,thinking,adaptive_thinking,interleaved_thinking'
```

<h3 id="override-model-ids-per-version">
  Eseguire l'override degli ID di modello per versione
</h3>

Su piattaforme che incorporano Claude Code e impostano [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/it/env-vars), la configurazione del modello dell'host ha la precedenza sulle impostazioni del modello gestite, mentre un elenco di autorizzazione `availableModels` gestito rimane in vigore a meno che l'host non fornisca il proprio; [Eccezioni alla precedenza delle impostazioni gestite](/docs/it/settings#exceptions-to-managed-settings-precedence) dice quali chiavi e variabili l'host sostituisce.

Le variabili di ambiente a livello di famiglia sopra configurano un ID di modello per alias di famiglia. Se è necessario mappare diverse versioni all'interno della stessa famiglia a ID di provider distinti, utilizzare invece l'impostazione `modelOverrides`.

`modelOverrides` mappa i singoli ID di modello Anthropic alle stringhe specifiche del provider che Claude Code invia all'API del provider. Quando un utente seleziona un modello mappato nel selettore `/model`, Claude Code utilizza il valore configurato invece del valore predefinito incorporato.

Questo consente agli amministratori aziendali di instradare ogni versione del modello a un ARN di profilo di inferenza Amazon Bedrock specifico, a un nome di versione Google Cloud's Agent Platform o a un nome di distribuzione Microsoft Foundry per governance, allocazione dei costi o instradamento regionale.

Impostare `modelOverrides` nel [file delle impostazioni](/docs/it/settings#where-settings-live):

```json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-7": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-prod",
    "claude-opus-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-46-prod",
    "claude-sonnet-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/sonnet-prod"
  }
}
```

Le chiavi devono essere ID di modello Anthropic come elencati nella [Panoramica dei modelli](https://platform.claude.com/docs/en/about-claude/models/overview). Per gli ID di modello datati, includere il suffisso della data esattamente come appare lì. Le chiavi sconosciute vengono ignorate.

Per interrompere la [riga diagnostica](/docs/it/errors#unrecognized-model-id-on-a-request) `[claude-code:unrecognized_model]` per un ID come un alias di gateway, aggiungere una voce con quell'ID come suo valore.

Gli override sostituiscono gli ID di modello incorporati che supportano ogni voce nel selettore `/model`. Su Amazon Bedrock, le voci `modelOverrides` hanno la precedenza su qualsiasi profilo di inferenza che Claude Code scopre automaticamente all'avvio. Claude Code passa i valori che sono già nativi del provider, come ARN di profilo di inferenza Amazon Bedrock o nomi di distribuzione Microsoft Foundry, al provider così come sono.

Gli override si applicano anche quando si passa un ID di modello Anthropic direttamente tramite `--model`, la variabile di ambiente `ANTHROPIC_MODEL`, o una variabile di ambiente `ANTHROPIC_DEFAULT_*_MODEL`. Su Amazon Bedrock, Google Cloud's Agent Platform e [Mantle](/docs/it/amazon-bedrock#use-the-mantle-endpoint), un ID di modello Anthropic senza voce `modelOverrides` si risolve nello stesso ID specifico del provider della riga del selettore `/model` per quella versione, quando il provider supporta quella versione. Mantle supporta un sottoinsieme di versioni. Per un ID di modello Anthropic al di fuori di quel sottoinsieme, Claude Code invia l'ID grezzo a Mantle senza mapparlo, a meno che una voce `modelOverrides` lo copra. Prima della v2.1.200, `--model` e i valori delle variabili di ambiente raggiungevano il provider così come erano senza passare attraverso la mappa di override.

`modelOverrides` funziona insieme a `availableModels`. L'elenco di autorizzazione viene valutato rispetto all'ID di modello Anthropic, non al valore di override, quindi una voce come `"opus"` in `availableModels` continua a corrispondere anche quando le versioni di Opus sono mappate a ARN. Quando `enforceAvailableModels` è impostato nelle impostazioni gestite, il Default applicato si risolve tramite `modelOverrides` dalle [impostazioni gestite](/docs/it/managed-settings#how-claude-code-combines-managed-sources) solo. Il mapping di un amministratore, come una versione fissata a un ARN di profilo di inferenza, viene rispettato nel Default applicato. Gli override dalle impostazioni utente o progetto non lo influenzano.

Quando `availableModels` è impostato nelle [impostazioni gestite](/docs/it/managed-settings), solo `modelOverrides` dalle impostazioni gestite si applicano a un ID di modello Anthropic passato direttamente tramite `--model` o le variabili di ambiente sopra. Claude Code ignora gli override nelle impostazioni utente o progetto per quegli ID, e non risolve mai un ID che l'elenco gestito esclude tramite `modelOverrides` da alcuna fonte di impostazioni. Questa restrizione di fonte gestita richiede Claude Code v2.1.200 o successivo. Vedere [Limitare la selezione del modello](#restrict-model-selection) per come vengono gestiti gli ID bloccati.

<h3 id="prompt-caching-configuration">
  Configurazione della prompt caching
</h3>

Claude Code utilizza automaticamente la [prompt caching](/docs/it/prompt-caching) per ottimizzare le prestazioni e ridurre i costi. È possibile disabilitare la prompt caching globalmente o per livelli di modello specifici:

| Variabile di ambiente           | Descrizione                                                                                                              |
| ------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `DISABLE_PROMPT_CACHING`        | Impostare su `1` per disabilitare la prompt caching per tutti i modelli. Ha la precedenza sulle impostazioni per modello |
| `DISABLE_PROMPT_CACHING_HAIKU`  | Impostare su `1` per disabilitare la prompt caching solo per i modelli Haiku                                             |
| `DISABLE_PROMPT_CACHING_SONNET` | Impostare su `1` per disabilitare la prompt caching solo per i modelli Sonnet                                            |
| `DISABLE_PROMPT_CACHING_OPUS`   | Impostare su `1` per disabilitare la prompt caching solo per i modelli Opus                                              |
| `DISABLE_PROMPT_CACHING_FABLE`  | Impostare su `1` per disabilitare la prompt caching solo per i modelli Fable                                             |

Per scegliere il TTL della cache per la conversazione principale e per i subagents separatamente, vedere [scegliere il TTL da soli](/docs/it/prompt-caching#choose-the-ttl-yourself). Per cosa attiva un cache miss, vedere [Come Claude Code utilizza la prompt caching](/docs/it/prompt-caching).
