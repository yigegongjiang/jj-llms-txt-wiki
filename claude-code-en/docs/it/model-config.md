> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurazione del modello

> Scopri la configurazione del modello Claude Code, inclusi gli alias dei modelli come `opusplan`

<h2 id="available-models">
  Modelli disponibili
</h2>

Per l'impostazione `model` in Claude Code, è possibile configurare:

* Un **alias del modello**
* Un **nome del modello**
  * API Anthropic: un **[nome del modello](https://platform.claude.com/docs/it/about-claude/models/overview)** completo
  * Amazon Bedrock: un ARN del profilo di inferenza
  * Microsoft Foundry: un nome di distribuzione
  * Google Cloud's Agent Platform: un nome di versione

Per indicazioni su quale modello e livello di sforzo si adattano a diversi tipi di lavoro, vedere [Choosing a Claude model and effort level in Claude Code](https://claude.com/blog/claude-model-and-effort-level-in-claude-code) sul blog.

<Note>
  `ANTHROPIC_BASE_URL` cambia dove vengono inviate le richieste, non quale modello le risponde. Per instradare Claude attraverso un gateway LLM, vedere [gateway LLM](/docs/it/llm-gateway).
</Note>

<h3 id="model-aliases">
  Alias dei modelli
</h3>

Gli alias dei modelli forniscono un modo conveniente per selezionare le impostazioni del modello senza dover ricordare i numeri di versione esatti:

| Alias del modello | Comportamento                                                                                                                                                                                                                                                                                                                                                           |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`default`**     | Valore speciale che cancella qualsiasi override del modello e ripristina il modello consigliato per il tipo di account, o al [modello predefinito dell'organizzazione](#organization-default-model) quando un amministratore ne ha impostato uno. Non è di per sé un alias del modello                                                                                  |
| **`best`**        | Utilizza Fable 5 dove la vostra organizzazione ha accesso, altrimenti il modello Opus più recente                                                                                                                                                                                                                                                                       |
| **`fable`**       | Utilizza Claude Fable 5 per i vostri compiti più difficili e di lunga durata                                                                                                                                                                                                                                                                                            |
| **`sonnet`**      | Utilizza il modello Sonnet più recente per le attività di codifica quotidiane                                                                                                                                                                                                                                                                                           |
| **`opus`**        | Utilizza il modello Opus più recente per attività di ragionamento complesso                                                                                                                                                                                                                                                                                             |
| **`haiku`**       | Utilizza il modello Haiku veloce ed efficiente per attività semplici                                                                                                                                                                                                                                                                                                    |
| **`sonnet[1m]`**  | Utilizza Sonnet con una [finestra di contesto di 1 milione di token](https://platform.claude.com/docs/it/build-with-claude/context-windows#context-window-sizes-by-model) per sessioni lunghe. Nessun effetto quando `sonnet` si risolve già in Sonnet 5 con la sua finestra 1M nativa; dietro un [gateway LLM](/docs/it/llm-gateway), seleziona la finestra 1M per Sonnet 5 |
| **`opus[1m]`**    | Utilizza Opus con una [finestra di contesto di 1 milione di token](https://platform.claude.com/docs/it/build-with-claude/context-windows#context-window-sizes-by-model) per sessioni lunghe                                                                                                                                                                             |
| **`opusplan`**    | Modalità speciale che utilizza `opus` durante Plan Mode, quindi passa a `sonnet` per l'esecuzione                                                                                                                                                                                                                                                                       |

La versione in cui gli alias `opus` e `sonnet` si risolvono dipende dal provider:

| Provider                                             | `opus`   | `sonnet`   |
| :--------------------------------------------------- | :------- | :--------- |
| API Anthropic                                        | Opus 4.8 | Sonnet 5   |
| [Claude Platform on AWS](/docs/it/claude-platform-on-aws) | Opus 4.8 | Sonnet 4.6 |
| Amazon Bedrock, Google Cloud's Agent Platform        | Opus 4.8 | Sonnet 4.5 |
| Microsoft Foundry                                    | Opus 4.6 | Sonnet 4.5 |

Dove un alias si risolve in un modello più vecchio, i modelli più recenti sono disponibili selezionando esplicitamente il nome del modello completo o impostando `ANTHROPIC_DEFAULT_OPUS_MODEL` o `ANTHROPIC_DEFAULT_SONNET_MODEL`.

Prima della v2.1.207, `opus` si risolveva in Opus 4.7 su Claude Platform on AWS e in Opus 4.6 su Amazon Bedrock e Google Cloud's Agent Platform.

Gli alias puntano alla versione consigliata per il vostro provider e si aggiornano nel tempo. Per fissare una versione specifica, utilizzare il nome del modello completo, ad esempio `claude-opus-4-8`, o impostare la variabile di ambiente corrispondente come `ANTHROPIC_DEFAULT_OPUS_MODEL`.

<Note>
  Sonnet 5 richiede Claude Code v2.1.197 o successivo. Opus 4.8 richiede v2.1.154 o successivo. Eseguire `claude update` per aggiornare.
</Note>

<h3 id="work-with-fable-5">
  Lavorare con Fable 5
</h3>

[Claude Fable 5](https://platform.claude.com/docs/it/about-claude/models/introducing-claude-fable-5-and-claude-mythos-5) è il modello più capace in Claude Code, adatto a compiti più grandi di una singola sessione. Sostiene lunghe sessioni autonome, indaga prima di agire e verifica il suo lavoro più spesso rispetto ai modelli più piccoli.

Fable 5 non è il modello predefinito. Selezionatelo con `/model fable`. Le richieste che i suoi classificatori di sicurezza contrassegnano, il più delle volte nei domini della sicurezza informatica e della biologia, attivano il [fallback automatico del modello](#automatic-model-fallback).

Per ottenere il massimo da Fable 5:

* **Descrivete il risultato, non i passaggi**: dategli il risultato che desiderate e lasciate che pianifichi il percorso. Per mantenerlo al lavoro fino a quando quel risultato non si verifica, [impostate un obiettivo](/docs/it/goal).
* **Dategli problemi ambigui**: le indagini sulla causa principale, il debug dei disservizi e le decisioni architettoniche sono dove l'indagine e la verifica aggiuntive ripagano.
* **Saltate i promemoria di verifica**: verifica il suo lavoro con meno sollecitazioni, quindi i promemoria per testare o controllare sono solitamente non necessari.
* **Dimensionate compiti più grandi**: dategli lavoro che normalmente dividereste in pezzi. Sostiene lunghe sessioni senza perdere il filo.

<Note>
  Fable 5 richiede Claude Code v2.1.170 o successivo. Le versioni precedenti non mostrano Fable 5 nel selettore di modelli e non possono selezionarlo. Eseguire `claude update` per aggiornare. Fable 5 non è disponibile con [zero data retention](/docs/it/zero-data-retention), dove il selettore `/model` lo omette o lo mostra disabilitato.
</Note>

<h3 id="setting-your-model">
  Impostazione del modello
</h3>

È possibile configurare il modello in diversi modi, elencati in ordine di priorità:

1. **Durante la sessione**: utilizzare `/model <alias|name>` per cambiare immediatamente, oppure eseguire `/model` senza argomenti per aprire il selettore. Il selettore chiede conferma quando la conversazione ha output precedente, poiché la risposta successiva rilegge la cronologia completa senza contesto memorizzato nella cache
2. **All'avvio**: avviare con `claude --model <alias|name>`
3. **Variabile di ambiente**: impostare `ANTHROPIC_MODEL=<alias|name>`
4. **Impostazioni**: configurare in modo permanente nel file delle impostazioni utilizzando il campo `model`

A partire dalla v2.1.153, `/model` salva la scelta come predefinita per le nuove sessioni scrivendo il campo `model` nelle impostazioni utente. Nel selettore:

* `Enter`: cambia modello e salva come predefinito
* `s`: cambia modello solo per questa sessione

Digitare `/model <name>` direttamente si comporta come `Enter`. Un modello impostato con `/model` in [modalità non interattiva](/docs/it/headless), con il flag `-p`, si applica solo alla sessione corrente e non viene salvato come predefinito. Le impostazioni del progetto e gestite hanno ancora la precedenza e si riapplicano al prossimo avvio. Un [modello predefinito dell'organizzazione](#organization-default-model) che l'amministratore ha configurato per sovrascrivere la selezione dell'utente si riapplica anche al prossimo avvio.

Nella v2.1.144 fino alla v2.1.152, `/model` si applicava solo alla sessione corrente e `d` nel selettore salvava un predefinito.

Il flag `--model` e la variabile di ambiente `ANTHROPIC_MODEL` si applicano solo alla sessione che avviate con essi. Per eseguire modelli diversi in terminali diversi contemporaneamente, avviate ciascuno con il proprio flag `--model` piuttosto che passare con `/model`.

I prezzi nel selettore `/model` appaiono quando Claude Code comunica con l'API Anthropic, direttamente o attraverso un [gateway LLM](/docs/it/llm-gateway) che lo fa da proxy, e il prezzo su una riga è il prezzo del modello che quella riga seleziona. Su [provider di terze parti](/docs/it/third-party-integrations) come Amazon Bedrock e sul [gateway delle app Claude](/docs/it/claude-apps-gateway), il vostro provider o gateway determina quanto pagate, quindi le righe del selettore non mostrano alcun prezzo. Il prezzo è solo un'etichetta di visualizzazione; non influisce su quale modello una riga seleziona o su cosa il vostro provider fattura. Prima della v2.1.206, [Claude Platform on AWS](/docs/it/claude-platform-on-aws) e le sessioni gateway mostravano i prezzi di listino di Anthropic, e una riga poteva mostrare il prezzo di un modello diverso da quello che selezionava.

Le sessioni riprese avviate con `claude --resume`, `--continue`, o il selettore `/resume` mantengono il modello che stavano utilizzando quando la trascrizione è stata salvata, indipendentemente dall'impostazione `model` corrente. Se quel modello è stato ritirato o è escluso da [`availableModels`](#restrict-model-selection), la sessione ricade nell'ordine di precedenza normale. Questo impedisce che la scelta `/model` di un'altra sessione cambi il modello al ripristino.

Un modello che scegliete per il nuovo avvio con `--model` o `ANTHROPIC_MODEL` ha ancora la precedenza sul modello ripristinato. A partire dalla v2.1.195, così come una variabile della famiglia [`ANTHROPIC_DEFAULT_OPUS_MODEL`](#environment-variables).

Quando il modello attivo all'avvio proviene dalle impostazioni del progetto o gestite piuttosto che dalla propria selezione, l'intestazione di avvio mostra quale file di impostazioni lo ha impostato. Eseguire `/model` per eseguire l'override; l'impostazione del progetto o gestita si riapplica al prossimo avvio.

Quando un cambio di modello viene richiesto tramite il metodo `setModel()` dell'[Agent SDK](/docs/it/agent-sdk/overview) o da un'app come l'[app Desktop](/docs/it/desktop) che esegue Claude Code CLI per voi, Claude Code verifica che la stringa sia una che riconosce prima di salvarla. Questo controllo richiede Claude Code v2.1.200 o successivo. Su API Anthropic, Claude Code riconosce:

* un alias del modello
* una voce dal selettore `/model`
* qualsiasi nome che inizia con `claude-`
* un valore che avete configurato voi stessi come [opzione di modello personalizzato](#add-a-custom-model-option) o in [`modelOverrides`](#override-model-ids-per-version)

Claude Code rifiuta una stringa non riconosciuta con `Model "<name>" is not a recognized model id.` e la sessione mantiene il suo modello corrente, invece di salvare la stringa e fallire alla prossima richiesta. Vedere il [riferimento degli errori](/docs/it/errors#model-is-not-a-recognized-model-id) per i passaggi di recupero.

Il controllo viene eseguito solo su API Anthropic. Su Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, [Claude Platform on AWS](/docs/it/claude-platform-on-aws), e dietro un [gateway LLM](/docs/it/llm-gateway) o un `ANTHROPIC_BASE_URL` personalizzato, il vostro provider o gateway definisce i nomi dei modelli, quindi Claude Code passa qualsiasi stringa senza controllarla. Il controllo inoltre non copre il flag `--model`, la variabile di ambiente `ANTHROPIC_MODEL`, o l'impostazione `model`; un valore digitato male lì produce [There's an issue with the selected model](/docs/it/errors#theres-an-issue-with-the-selected-model) alla prima richiesta invece.

Quando il modello richiesto ha una data di ritiro programmata o viene automaticamente rimappato a una versione più recente, Claude Code mostra un avviso che nomina il modello richiesto. Le sessioni interattive lo mostrano come un avviso di avvio. A partire dalla v2.1.182, lo stesso avviso viene scritto su stderr in [modalità non interattiva](/docs/it/headless) quando si utilizza il formato di output di testo predefinito. Il controllo copre anche un `model` impostato nel [frontmatter del subagent](/docs/it/sub-agents). L'avviso su stderr è soppresso per `--output-format json` e `stream-json`; leggere il modello effettivo dal campo `modelUsage` del [messaggio di risultato](/docs/it/headless#get-structured-output) invece.

Esempio di utilizzo:

```bash theme={null}
# Avviare con Opus
claude --model opus

# Passare a Sonnet durante la sessione
/model sonnet
```

File delle impostazioni di esempio:

```json theme={null}
{
    "permissions": {
        ...
    },
    "model": "opus"
}
```

<h2 id="restrict-model-selection">
  Limitare la selezione del modello
</h2>

Gli amministratori aziendali possono utilizzare `availableModels` nelle [impostazioni gestite o di policy](/docs/it/settings#settings-files) per limitare quali modelli gli utenti possono selezionare. Le voci corrispondono a una famiglia di modelli come `sonnet`, un prefisso di versione come `claude-sonnet-4-5`, o un ID modello completo come `claude-sonnet-4-5-20250929`.

Quando `availableModels` è impostato, l'elenco di autorizzazione si applica ovunque un utente possa specificare un modello:

* **Modello della sessione principale**: `/model`, il flag `--model`, la variabile di ambiente `ANTHROPIC_MODEL`, l'impostazione `model` e il modello ripristinato quando [si riprende una sessione](#setting-your-model)
* **Risoluzione alias**: le variabili di ambiente `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, `ANTHROPIC_DEFAULT_HAIKU_MODEL` e `ANTHROPIC_DEFAULT_FABLE_MODEL` non possono reindirizzare un alias consentito a un modello al di fuori dell'elenco
* **Modalità veloce**: `/fast` rifiuta di attivare/disattivare quando comporterebbe un passaggio implicito a un modello Opus al di fuori dell'elenco, con il messaggio "is not in your organization's allowed models"
* **Modelli dei subagent**: il campo `model` nel [frontmatter del subagent](/docs/it/sub-agents#choose-a-model), il parametro `model` dello strumento Agent, `CLAUDE_CODE_SUBAGENT_MODEL`, e, su v2.1.197 e versioni precedenti, il selettore di modelli nella procedura guidata `/agents`&#x20;
* **Modelli di skill e comando**: il frontmatter `model` in [skill e comandi](/docs/it/skills)
* **Modello Advisor**: l'impostazione [`advisorModel`](/docs/it/advisor) configurata e il flag `--advisor`
* **Modello dell'agente di background**: il modello selezionato nel [selettore di dispatch](/docs/it/agent-view)

Su Anthropic API e [Claude Platform su AWS](/docs/it/claude-platform-on-aws), un alias di famiglia di modelli, `opus`, `sonnet`, `haiku` o `fable`, si risolve alla versione più recente della sua famiglia che l'elenco di autorizzazione consente. Quando l'elenco di autorizzazione fissa versioni specifiche, ad esempio `["sonnet", "claude-opus-4-6"]`, sia `/model opus` che `--model opus` selezionano Claude Opus 4.6, l'Opus più recente consentito, e mostrano un avviso che nomina sia i modelli richiesti che quelli sostituiti. Prima della v2.1.205, un alias la cui versione più recente rilasciata era al di fuori dell'elenco veniva rifiutato o sostituito come qualsiasi altra selezione bloccata, anche quando l'elenco consentiva una versione precedente.

Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry e [Mantle](/docs/it/amazon-bedrock#use-the-mantle-endpoint) utilizzano ID di distribuzione specifici del provider anziché ID di modello Anthropic, quindi un alias bloccato lì segue il comportamento di rifiuto e sostituzione di seguito.

Claude Code gestisce qualsiasi altra selezione bloccata in base a dove il modello è stato impostato:

* **`/model`**: il passaggio viene rifiutato con un errore
* **Flag `--model`, `ANTHROPIC_MODEL` o impostazione `model`**: il valore viene sostituito all'avvio con un avviso che nomina sia i modelli richiesti che quelli sostituiti, e la sessione inizia sul modello predefinito
* **Override di subagent, skill o comando**: l'override ritorna al modello ereditato o predefinito piuttosto che non riuscire nella richiesta
* **Impostazione `advisorModel`**: l'advisor viene disabilitato per la sessione
* **Flag `--advisor`**: Claude Code esce con un errore all'avvio

I modelli esclusi sono nascosti dal selettore `/model`. Un ID modello completo nell'elenco che non ha una riga del selettore incorporata, come una versione precedente che l'elenco fissa, appare nel selettore `/model` come una propria riga etichettata. Prima della v2.1.199, tale ID era selezionabile solo digitando `/model <id>`.

I cambiamenti del modello che Claude Code effettua per vostro conto vengono controllati allo stesso modo:

* **[Catene di modelli di fallback](#fallback-model-chains)**: gli elementi al di fuori dell'elenco di autorizzazione vengono eliminati
* **Aggiornamenti in modalità plan**: su Anthropic API e Claude Platform su AWS, un aggiornamento come [`opusplan`](#opusplan-model-setting) a un modello escluso utilizza la versione più recente consentita della famiglia di aggiornamento. Su provider con ID di modello specifici del provider, e quando nessuna versione è consentita, l'aggiornamento viene saltato e la pianificazione continua sul modello della sessione
* **[Fallback automatico del modello](#automatic-model-fallback)**: un fallback il cui target è escluso non viene eseguito, quindi la richiesta contrassegnata termina con un rifiuto
* **[Modalità veloce](/docs/it/fast-mode)**: l'abilitazione della modalità veloce viene rifiutata quando il modello su cui la sessione verrebbe eseguita in seguito è al di fuori dell'elenco di autorizzazione

```json theme={null}
{
  "availableModels": ["sonnet", "haiku"]
}
```

<h3 id="surface-coverage">
  Copertura della superficie
</h3>

Ogni superficie applica l'elenco di autorizzazione che riceve. Il meccanismo di consegna che raggiunge ogni superficie differisce:

| Meccanismo di consegna                                                                          | CLI e IDE | Sessioni locali desktop | Sessioni web, mobile e cloud | Agent SDK e non interattive | Cowork                     |
| :---------------------------------------------------------------------------------------------- | :-------- | :---------------------- | :--------------------------- | :-------------------------- | :------------------------- |
| [Impostazioni gestite dal server](/docs/it/server-managed-settings) dalla console di amministrazione | Applicate | Applicate               | Applicate                    | Applicate                   | Non consegnate             |
| [File di impostazioni gestite o MDM](/docs/it/settings#settings-files)                               | Applicate | Applicate               | Non consegnate               | Applicate                   | Applicate dove distribuite |

* Le sessioni cloud, su [Claude Code sul web](/docs/it/claude-code-on-the-web) o nell'app Desktop, vengono eseguite su VM gestite da Anthropic: le impostazioni distribuite al vostro dispositivo non le raggiungono, quindi consegnate l'elenco di autorizzazione tramite impostazioni gestite dal server. Un cambio di modello a metà sessione in una sessione cloud viene rifiutato quando il modello richiesto è escluso dall'elenco di autorizzazione. Il rifiuto lato server alla creazione della sessione si applica alle [restrizioni del modello dell'organizzazione](#organization-model-restrictions), non alla chiave delle impostazioni `availableModels`.
* Cowork, la scheda agentic-work nell'app Claude Desktop, non è una superficie Claude Code e non riceve impostazioni gestite dal server per progettazione. Un file di impostazioni gestite si applica alle sessioni Cowork quando è presente dove la sessione viene eseguita; le sessioni Cowork remote vengono eseguite su VM gestite da Anthropic, dove un file distribuito dal dispositivo non è presente.
* Le sessioni su [provider di terze parti](/docs/it/server-managed-settings#platform-availability) come Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry e [Claude Platform su AWS](/docs/it/claude-platform-on-aws) non ricevono impostazioni gestite dal server, quindi consegnate l'elenco di autorizzazione tramite MDM o file di impostazioni gestite lì.
* La consegna gestita dal server richiede anche che la sessione si autentichi con un accesso all'organizzazione o una chiave API configurata direttamente. Le flotte che generano chiavi solo tramite uno script [`apiKeyHelper`](/docs/it/settings#available-settings) dovrebbero consegnare l'elenco di autorizzazione tramite MDM o file di impostazioni gestite.
* La scheda Desktop Code ospita anche [sessioni SSH](/docs/it/desktop#ssh-sessions), che leggono il file di impostazioni gestite dall'host remoto su cui vengono eseguite. Vedere [Impostazioni gestite desktop](/docs/it/desktop#managed-settings).
* I selettori di modelli su claude.ai e nell'app Desktop nascondono o disabilitano i modelli esclusi dall'elenco di autorizzazione della vostra organizzazione. Lo stato del selettore è una comodità per gli utenti; l'applicazione avviene nella sessione.

<h3 id="default-model-behavior">
  Comportamento del modello predefinito
</h3>

L'opzione Predefinito nel selettore di modelli non è interessata da `availableModels` a meno che [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) non sia anche impostato. Da solo, `availableModels` lascia Predefinito disponibile, risolvendo al [valore predefinito di runtime](#default-model-setting) del sistema per l'account. Se il valore predefinito è un modello che intendete limitare, impostare anche `enforceAvailableModels`.

Un array `availableModels` vuoto non attiva mai l'applicazione del modello Predefinito: con `availableModels: []`, le selezioni di modelli denominati vengono bloccate ma il modello Predefinito per il tipo di account rimane utilizzabile indipendentemente da `enforceAvailableModels`.

<h3 id="enforce-the-allowlist-for-the-default-model">
  Applicare l'elenco di autorizzazione per il modello Predefinito
</h3>

Impostare `enforceAvailableModels: true` insieme a un `availableModels` non vuoto nelle impostazioni gestite per estendere l'elenco di autorizzazione all'opzione Predefinito. Questo richiede Claude Code v2.1.175 o successivo.

```json theme={null}
{
  "availableModels": ["sonnet", "haiku"],
  "enforceAvailableModels": true
}
```

L'opzione Predefinito si risolve al valore predefinito del tipo di account, o al [modello predefinito dell'organizzazione](#organization-default-model) quando un amministratore ne ha impostato uno. Quando quel modello non è nell'elenco di autorizzazione, l'opzione Predefinito si risolve invece nella prima voce `availableModels` che nomina un modello consentito e disponibile, e la riga Predefinito del selettore `/model` mostra quel modello. Questo si applica ovunque il valore predefinito viene raggiunto: avvio della sessione, selezione di Predefinito in `/model`, la parola chiave `"default"` nelle [catene di modelli di fallback](#fallback-model-chains) e il fallback utilizzato quando una selezione esclusa viene eliminata.

`enforceAvailableModels` non ha effetto quando `availableModels` non è impostato o è vuoto: con `availableModels: []`, il modello Predefinito per il tipo di account rimane utilizzabile, quindi l'impostazione non può bloccare gli utenti da ogni modello. Quando `availableModels` è non vuoto ma nessuna voce si risolve a un modello consentito e disponibile, l'applicazione si degrada e Predefinito ricade al valore predefinito del tipo di account, con un avviso visibile solo sotto `--debug`. Mantenere almeno una voce garantita disponibile nell'elenco per evitare questo.

Distribuire entrambe le chiavi nella [fonte gestita con precedenza più alta](/docs/it/settings#settings-precedence): le fonti gestite distribuite dall'amministratore non si uniscono, quindi una coppia posizionata in un file di impostazioni gestite viene ignorata quando la console di amministrazione consegna qualsiasi impostazione.

<h3 id="control-the-model-users-run-on">
  Controllare il modello su cui gli utenti eseguono
</h3>

L'impostazione `model` è una selezione iniziale, non un'applicazione. Imposta quale modello è attivo quando una sessione inizia, ma gli utenti possono comunque aprire `/model` e scegliere Predefinito, che si risolve al [valore predefinito di runtime](#default-model-setting) del sistema indipendentemente da ciò che `model` è impostato, a meno che [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) non lo reindizzi.

Per controllare completamente l'esperienza del modello, combinare queste impostazioni:

* **`availableModels`**: limita a quali modelli denominati gli utenti possono passare
* **`enforceAvailableModels`**: estende l'elenco di autorizzazione `availableModels` all'opzione Predefinito, quindi Predefinito non può risolversi a un modello al di fuori dell'elenco
* **`model`**: imposta la selezione del modello iniziale quando una sessione inizia
* **`ANTHROPIC_DEFAULT_SONNET_MODEL`** / **`ANTHROPIC_DEFAULT_OPUS_MODEL`** / **`ANTHROPIC_DEFAULT_HAIKU_MODEL`** / **`ANTHROPIC_DEFAULT_FABLE_MODEL`**: controllano a cosa si risolvono l'opzione Predefinito e gli alias `sonnet`, `opus`, `haiku` e `fable`

Questo esempio avvia gli utenti su Sonnet 4.5, limita il selettore a Sonnet e Haiku, e assicura che Predefinito si risolva a un modello nell'elenco di autorizzazione piuttosto che al valore predefinito del livello:

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

Senza `enforceAvailableModels` o il blocco `env`, un utente che seleziona Predefinito nel selettore otterrebbe la versione più recente per il suo livello, bypassando il pin di versione in `model` e `availableModels`. Le due impostazioni coprono ambiti diversi: `enforceAvailableModels` fa sì che Predefinito obbedisca all'elenco di autorizzazione, mentre il blocco `env` fissa quale versione un alias consentito come `sonnet` si risolve. Utilizzare `enforceAvailableModels` da solo quando limitare le famiglie di modelli è sufficiente; aggiungere il blocco `env` quando è necessario anche fissare una versione specifica.

<h3 id="merge-behavior">
  Comportamento di unione
</h3>

Quando la [fonte di impostazioni gestite con precedenza più alta](/docs/it/server-managed-settings#settings-precedence) definisce `availableModels`, solo quell'elenco si applica: le voci nelle impostazioni utente, progetto o locali non possono estenderlo, e le fonti gestite distribuite dall'amministratore non si uniscono tra loro, quindi un elenco distribuito in un file di impostazioni gestite viene ignorato quando le impostazioni gestite dal server consegnano qualsiasi chiave. Altrimenti, gli elenchi dalle impostazioni utente, progetto e locali vengono [concatenati e deduplicati](/docs/it/settings#settings-precedence) come altre impostazioni di array. A partire da Claude Code v2.1.175, l'elenco gestito sostituisce le voci di precedenza inferiore; le versioni precedenti le uniscono.

All'interno dell'elenco effettivo, una voce che nomina un modello specifico in una famiglia, sia un prefisso di versione che un ID modello completo, disabilita la voce wildcard della famiglia: `["sonnet", "claude-sonnet-4-5"]` consente solo le versioni Sonnet 4.5, non ogni modello Sonnet.

<h3 id="mantle-model-ids">
  ID di modello Mantle
</h3>

Quando l'[endpoint Bedrock Mantle](/docs/it/amazon-bedrock#use-the-mantle-endpoint) è abilitato, le voci in `availableModels` che iniziano con `anthropic.` vengono aggiunte al selettore `/model` come opzioni personalizzate e instradate all'endpoint Mantle. Questa è un'eccezione alla corrispondenza degli alias descritta in [Fissare i modelli per le distribuzioni di terze parti](#pin-models-for-third-party-deployments). L'impostazione limita comunque il selettore alle voci elencate, e un ID Mantle incorpora un nome di famiglia, quindi conta come una voce specifica e disabilita il wildcard della famiglia: insieme a qualsiasi ID Mantle, elencare i prefissi di versione o gli ID completi che desiderate mantenere selezionabili. Vedere [Comportamento di unione](#merge-behavior).

<h3 id="organization-model-restrictions">
  Restrizioni del modello dell'organizzazione
</h3>

Gli amministratori dell'organizzazione sui piani Claude Enterprise limitano quali modelli i membri possono eseguire disabilitando i singoli modelli nella console di amministrazione claude.ai. Questa restrizione viene consegnata con i diritti dell'account quando Claude Code si autentica, separata da qualsiasi elenco `availableModels` nelle impostazioni, e il server applica la stessa restrizione indipendentemente quando una sessione viene creata. Richiede Claude Code v2.1.187 o successivo.

La restrizione si applica quando un membro accede o utilizza la propria chiave API. Le credenziali con ambito organizzazione, come le chiavi di servizio dell'organizzazione, non sono legate a un utente, quindi la restrizione non si applica a loro.

La Claude Console non ha controllo di restrizione del modello. Le organizzazioni senza un piano Claude Enterprise, incluse quelle i cui membri si autenticano tramite l'API Anthropic, limitano i modelli con [`availableModels`](#restrict-model-selection) nelle [impostazioni gestite](/docs/it/settings#settings-files), aggiungendo [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) per coprire l'opzione Predefinito. Queste impostazioni vengono applicate da Claude Code stesso, non dal server.

Un modello limitato è nascosto dal selettore `/model`. Selezionarlo per nome con `--model`, la variabile di ambiente `ANTHROPIC_MODEL` o l'impostazione `model` mostra l'avviso `Model "<name>" is restricted by your organization's settings. Using <model> instead.` e la sessione inizia su un modello consentito. Digitare `/model <name>` per un modello limitato viene rifiutato con `Model '<name>' is restricted by your organization's settings. Run /model to choose a different model.` e la sessione mantiene il suo modello attuale.

Un [alias di famiglia di modelli](#restrict-model-selection) come `opus` si risolve alla versione più recente della sua famiglia che l'organizzazione consente, con lo stesso avviso di sostituzione. `/model <alias>` viene rifiutato solo quando ogni versione della sua famiglia è limitata; un alias impostato con `--model`, `ANTHROPIC_MODEL` o l'impostazione `model` viene comunque sostituito all'avvio in quel caso. Prima della v2.1.205, un alias di famiglia veniva sostituito o rifiutato in base alla sua versione più recente rilasciata da sola, anche quando una versione precedente era consentita.

Le restrizioni si applicano a livello di organizzazione o per ruolo:

* La disabilitazione di un modello a livello di organizzazione lo rimuove per ogni membro.
* L'accesso a livello di ruolo concede modelli diversi a ruoli personalizzati diversi, e un membro che detiene più ruoli può utilizzare qualsiasi modello che uno dei suoi ruoli concede.
* I modelli Haiku sono sempre disponibili e non possono essere disabilitati, quindi ogni membro mantiene almeno un modello utilizzabile.
* Un cambio di accesso ha effetto su nuove richieste entro circa un minuto; il selettore `/model` lo riflette la prossima volta che una sessione inizia.

Entrambe le restrizioni si applicano insieme: un modello è selezionabile solo quando è consentito da `availableModels` e non è limitato dall'organizzazione. Le restrizioni dell'organizzazione vengono consegnate alle sessioni su Anthropic API e distribuzioni [gateway LLM](/docs/it/llm-gateway). Le sessioni su Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry e Claude Platform su AWS non le ricevono, quindi utilizzate `availableModels` su quei provider invece.

<h2 id="organization-default-model">
  Modello predefinito dell'organizzazione
</h2>

Gli amministratori dell'organizzazione sui piani Claude Enterprise possono impostare un modello predefinito per i membri di Claude Code dalla console di amministrazione claude.ai, per l'intera organizzazione o per ruolo personalizzato. Quando uno è impostato, l'opzione Predefinito si risolve a quel modello invece del [valore predefinito del tipo di account](#default-model-setting). Richiede Claude Code v2.1.196 o successivo.

La riga Predefinito nel selettore `/model` mostra il nome del modello predefinito dell'organizzazione con l'etichetta Org default. L'etichetta legge Org default indipendentemente dal fatto che l'amministratore abbia impostato il valore predefinito per l'intera organizzazione o per il vostro ruolo. Un valore predefinito del ruolo copre i membri di quel ruolo personalizzato e ha la precedenza sul valore predefinito a livello di organizzazione; quando diversi vostri ruoli impostano valori predefiniti diversi, si applica il modello più capace.

Il modello predefinito dell'organizzazione è un punto di partenza, non una restrizione, e qualsiasi altra selezione di modello ha la precedenza su di esso:

* il flag `--model` e la variabile di ambiente `ANTHROPIC_MODEL`
* un valore `model` nelle [impostazioni gestite](/docs/it/settings#settings-files) o fornito tramite `--settings`
* un valore `model` nelle impostazioni utente, progetto o locali, incluso un modello salvato con `/model`

Gli amministratori possono anche configurare il modello predefinito dell'organizzazione per sovrascrivere la selezione dell'utente. Con l'override attivato, ha la precedenza sul valore `model` nelle impostazioni utente, progetto e locali, quindi un modello salvato con `/model` si applica per la sessione corrente e il modello predefinito dell'organizzazione ritorna al prossimo avvio. Quando la vostra selezione differisce, `/model` mostra `Your organization's default (<model>) applies on restart`. Il flag `--model`, `ANTHROPIC_MODEL`, le impostazioni gestite e `--settings` hanno ancora la precedenza anche con l'override attivato. L'override è disponibile per un set limitato di organizzazioni; chiedete al vostro team di account Anthropic riguardo alla disponibilità.

Per limitare quali modelli i membri possono selezionare, utilizzare [restrizioni del modello dell'organizzazione](#organization-model-restrictions) o [`availableModels`](#restrict-model-selection) invece.

Claude Code legge il modello predefinito dell'organizzazione una volta all'avvio, quindi un valore predefinito che l'amministratore cambia a metà sessione ha effetto al prossimo avvio.

Quando il modello predefinito dell'organizzazione non sovrascrive la selezione dell'utente, il primo avvio interattivo dopo che l'amministratore lo cambia cancella la chiave `model` dalle impostazioni utente una volta, quindi il nuovo valore predefinito si applica. Non cambia nient'altro nel file, e un modello salvato con `/model` dopo quel lancio viene mantenuto.

Il modello predefinito dell'organizzazione passa attraverso gli stessi controlli di restrizione di qualsiasi altro modello Predefinito prima di essere adottato:

* [`availableModels`](#restrict-model-selection) da solo non vincola mai l'opzione Predefinito, quindi un modello predefinito dell'organizzazione al di fuori dell'elenco di autorizzazione si applica comunque. Quando [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) è anche impostato, un modello predefinito dell'organizzazione al di fuori dell'elenco di autorizzazione viene rimappato alla prima voce dell'elenco di autorizzazione, come qualsiasi altro Predefinito
* un modello predefinito dell'organizzazione che [restrizioni del modello dell'organizzazione](#organization-model-restrictions) negano per il vostro account viene sostituito dal modello consentito più recente nella sua famiglia, o una famiglia a costo inferiore quando ogni versione di essa è limitata
* un modello predefinito dell'organizzazione che non è disponibile per il vostro account affatto, come Fable 5 con [zero data retention](/docs/it/zero-data-retention), viene saltato, e l'opzione Predefinito si risolve al valore predefinito del tipo di account

A partire dalla v2.1.199, quando il modello predefinito dell'organizzazione è una famiglia di modelli diversa dal valore predefinito usuale del tipo di account, il selettore `/model` mantiene una riga separata per quella famiglia usuale, quindi potete comunque passare ad essa per una sessione. Nella v2.1.196 fino alla v2.1.198 quella riga manca dal selettore.

Il modello predefinito dell'organizzazione viene consegnato alle sessioni autenticate con l'API Anthropic. Le sessioni su distribuzioni [gateway LLM](/docs/it/llm-gateway), Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry e Claude Platform su AWS non lo ricevono. Per impostare un valore predefinito su quelle distribuzioni, utilizzare la chiave `model` nelle [impostazioni gestite](/docs/it/settings#settings-files) invece.

<h2 id="organization-effort-limits">
  Limiti di sforzo dell'organizzazione
</h2>

Gli amministratori dell'organizzazione sui piani Claude Enterprise possono impostare un [livello di sforzo](#adjust-effort-level) massimo per modello per ogni ruolo personalizzato, insieme alle [restrizioni del modello dell'organizzazione](#organization-model-restrictions) a livello di ruolo. I livelli al di sopra del limite non vengono offerti nel selettore `/effort`, e nominare un livello più alto con `--effort` o `/effort` viene eseguito al limite invece. Nelle sessioni interattive e nelle esecuzioni di testo semplice `--print`, un avviso nomina i livelli richiesti e applicati; con output `json` o `stream-json` o negli agenti di background, il limite si applica silenziosamente. I limiti sono per modello, quindi il cambio di modelli può modificare quali livelli sono disponibili. Quando diversi vostri ruoli concedono lo stesso modello, si applica il limite meno restrittivo. Richiede Claude Code v2.1.195 o successivo.

I limiti di sforzo vengono consegnati insieme alle [restrizioni del modello dell'organizzazione](#organization-model-restrictions) e seguono la stessa disponibilità del provider: le sessioni su Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry e Claude Platform su AWS non le ricevono.

<h2 id="special-model-behavior">
  Comportamento speciale del modello
</h2>

<h3 id="default-model-setting">
  Impostazione del modello `default`
</h3>

Il comportamento di `default` dipende dal tipo di account:

* **Max, Team Premium, Enterprise pay-as-you-go e API Anthropic**: per impostazione predefinita Opus 4.8
* **Claude Platform su AWS, Amazon Bedrock e Google Cloud's Agent Platform**: per impostazione predefinita Opus 4.8
* **Pro, Team Standard e posti di abbonamento Enterprise**: per impostazione predefinita Sonnet 5
* **Microsoft Foundry**: per impostazione predefinita Sonnet 4.5

Enterprise pay-as-you-go significa un'organizzazione Enterprise fatturata per utilizzo piuttosto che per posto di abbonamento.

Prima di v2.1.207, `default` si risolveva in Opus 4.7 su Claude Platform su AWS e in Sonnet 4.5 su Amazon Bedrock e Google Cloud's Agent Platform.

Quando un amministratore ha impostato un [modello predefinito dell'organizzazione](#organization-default-model), `default` si risolve a quel modello invece del valore predefinito del tipo di account sopra. Richiede Claude Code v2.1.196 o successivo.

Quando le impostazioni gestite [applicano l'elenco consentito per il modello predefinito](#enforce-the-allowlist-for-the-default-model) e il valore predefinito del tipo di account non è in `availableModels`, `default` si risolve nel valore predefinito applicato invece del valore predefinito del tipo di account sopra. Quando entrambi si applicano, il modello predefinito dell'organizzazione sostituisce il valore predefinito del tipo di account per primo e l'applicazione si applica quindi ad esso: un modello predefinito dell'organizzazione nell'elenco di autorizzazione viene mantenuto, mentre uno al di fuori dell'elenco si risolve nel Predefinito applicato.

Fable 5 non è il modello predefinito su alcun tipo di account. Le sessioni utilizzano Fable 5 solo dopo che lo scegliete, con `/model fable`, un'impostazione `model`, o l'alias `best` dove Fable 5 è disponibile. Sceglierlo con `/model` lo salva come modello selezionato nelle impostazioni utente, quindi le sessioni successive iniziano su Fable 5 fino a quando non cambiate modello.

<h3 id="opusplan-model-setting">
  Impostazione del modello `opusplan`
</h3>

L'alias del modello `opusplan` fornisce un approccio ibrido automatizzato:

* **In Plan Mode**: utilizza `opus` per il ragionamento complesso e le decisioni architettoniche
* **In modalità esecuzione**: passa automaticamente a `sonnet` per la generazione di codice e l'implementazione

Questo abbina il ragionamento di Opus per la pianificazione con l'efficienza di Sonnet per l'esecuzione.

La fase Opus in Plan Mode utilizza la stessa finestra di contesto dell'impostazione del modello `opus`. Sui livelli di abbonamento in cui Opus viene [automaticamente aggiornato a contesto 1M](#extended-context), `opusplan` riceve l'aggiornamento anche in Plan Mode. Per forzare il contesto 1M per entrambe le fasi quando non siete su un livello di aggiornamento automatico, impostare il modello su `opusplan[1m]`.

Quando [`availableModels`](#restrict-model-selection) esclude il più recente Opus ma consente una versione precedente, ad esempio `["sonnet", "claude-opus-4-6"]`, `opusplan` utilizza il più recente Opus consentito per la pianificazione e rimane su Sonnet solo quando ogni Opus è escluso. Una sessione Haiku che normalmente si aggiornerebbe a Sonnet in Plan Mode utilizza allo stesso modo il più recente Sonnet consentito e rimane su Haiku solo quando ogni Sonnet è escluso. Prima di v2.1.205, Plan Mode rimase sulla sessione del modello ogni volta che la versione più recente della famiglia di aggiornamento era esclusa, anche quando l'elenco di autorizzazione consentiva una versione precedente.

La sostituzione di una versione precedente consentita si applica su API Anthropic e [Claude Platform su AWS](/docs/it/claude-platform-on-aws). Su Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry e Mantle, le cui distribuzioni utilizzano ID modello specifici del provider, Plan Mode rimane sulla sessione del modello ogni volta che il modello di aggiornamento è escluso.

Per un approccio ibrido in cui Claude decide a metà attività quando consultare un secondo modello piuttosto che passare al confine del piano, vedere lo [strumento advisor](/docs/it/advisor).

<h3 id="fallback-model-chains">
  Catene di modelli di fallback
</h3>

Quando il modello primario è sovraccarico, non disponibile o restituisce un altro errore del server non ripetibile, Claude Code può passare a un modello di fallback invece di non riuscire nella richiesta. Gli errori di autenticazione, fatturazione, limite di velocità, dimensione della richiesta e trasporto non attivano mai un passaggio; quelli seguono il loro normale retry e gestione degli errori.

Configurate uno o più modelli di fallback e Claude Code li prova in ordine, mostrando un avviso quando passa. Il passaggio dura solo il turno corrente, quindi il vostro prossimo messaggio prova prima il modello primario di nuovo. Le catene sono limitate a tre modelli dopo la rimozione dei duplicati e le voci extra vengono ignorate.

Impostare una catena per una sessione con il flag `--fallback-model`, che accetta un elenco separato da virgole:

```bash theme={null}
claude --fallback-model sonnet,haiku
```

Per mantenere una catena tra le sessioni, impostare `fallbackModel` in [impostazioni](/docs/it/settings) come array:

```json theme={null}
{
  "fallbackModel": ["claude-sonnet-5", "claude-haiku-4-5"]
}
```

Il flag `--fallback-model` ha la precedenza sull'impostazione `fallbackModel`. Ogni elemento accetta un nome di modello o un alias, e `"default"` si espande al modello predefinito.

Due casi causano il salto di un elemento:

* **Modello non disponibile**: un modello che non può essere raggiunto, come un modello ritirato bloccato nelle impostazioni, viene saltato e Claude Code continua all'elemento successivo.
* **Fuori dall'elenco consentito**: un elemento non consentito da [`availableModels`](#restrict-model-selection) viene eliminato quando la catena viene letta e non viene mai provato.

<h3 id="automatic-model-fallback">
  Fallback automatico del modello
</h3>

Questa sezione copre il fallback basato sul contenuto da Fable 5. Per il fallback basato sulla disponibilità quando un modello è sovraccarico o non disponibile, vedere [Catene di modelli di fallback](#fallback-model-chains).

Fable 5 viene eseguito con classificatori di sicurezza per il contenuto di sicurezza informatica e biologia. Quando un classificatore contrassegna una richiesta, Claude Code riesegue quella richiesta sul modello Opus predefinito del vostro provider e mostra un avviso nella trascrizione. Su API Anthropic, distribuzioni [gateway LLM](/docs/it/llm-gateway) e [Claude Platform su AWS](/docs/it/claude-platform-on-aws), quel modello è Opus 4.8. Su [gateway app Claude](/docs/it/claude-apps-gateway), è Opus 4.7 a meno che non puntiate l'alias [`opus`](#environment-variables) a un altro modello.

La sessione continua quindi su quel modello Opus. Per tornare a Fable 5, eseguire `/model fable`.

Il target di fallback viene controllato rispetto a [`availableModels`](#restrict-model-selection). Quando è bloccato, non si verifica alcun fallback. Il rifiuto emerge come un errore normale e il modello della sessione rimane invariato.

<h4 id="check-what-triggered-fallback">
  Verificare cosa ha attivato il fallback
</h4>

Il fallback può attivarsi sulla prima richiesta di una sessione, prima di inviare qualcosa di insolito, perché la prima richiesta contiene il contesto dell'area di lavoro come il contenuto di CLAUDE.md e lo stato di git. Un repository che contiene materiale di sicurezza o biologia può attivare il classificatore solo su quel contesto.

Per verificare se le personalizzazioni sono il trigger, avviare una sessione con `claude --safe-mode`, che disabilita le personalizzazioni come CLAUDE.md, skills, server MCP e hooks. Lo stato di git e i nomi delle directory non sono personalizzazioni e sono ancora inclusi.

<h4 id="ask-before-switching">
  Chiedere prima di passare
</h4>

Per decidere cosa accade ogni volta che una richiesta viene contrassegnata, piuttosto che passare automaticamente, eseguire `/config` e disattivare "switch models when a message is flagged". Una richiesta contrassegnata mette quindi in pausa la sessione con due opzioni: passare al modello Opus o modificare il prompt e riprovare su Fable 5.

Alcuni casi si comportano diversamente:

* Se entrambi i modelli contrassegnano la stessa richiesta, potete modificare il prompt e riprovare, o avviare una nuova sessione.
* Su sessioni mobili [Claude Code sul web](/docs/it/claude-code-on-the-web), la modifica e il nuovo tentativo non sono supportati. Passate i modelli o continuate la sessione da un browser desktop o dall'app desktop.
* In [modalità non interattiva](/docs/it/cli-reference#cli-flags) e integrazioni SDK che non possono mostrare il prompt, una richiesta contrassegnata termina il turno con un rifiuto.
* Quando il target di fallback è bloccato da [`availableModels`](#restrict-model-selection), il prompt non viene mostrato. La richiesta contrassegnata termina con il rifiuto, lo stesso del fallback automatico quando il target è bloccato.

<h4 id="enable-fallback-on-bedrock-agent-platform-and-foundry">
  Abilitare il fallback su Bedrock, Agent Platform e Foundry
</h4>

Su [Amazon Bedrock](/docs/it/amazon-bedrock), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai) e [Microsoft Foundry](/docs/it/microsoft-foundry), gli ID modello sono specifici del provider, quindi il fallback automatico funziona solo quando Claude Code può identificare entrambi i modelli coinvolti:

* Claude Code deve riconoscere il modello corrente come Fable 5: l'ID modello contiene `claude-fable-5`, corrisponde al valore di `ANTHROPIC_DEFAULT_FABLE_MODEL`, o è mappato con [`modelOverrides`](#override-model-ids-per-version).
* Il target di fallback deve risolvere a un modello Opus: il valore di `ANTHROPIC_DEFAULT_OPUS_MODEL` se impostato, altrimenti una voce Opus 4.8 nell'elenco dei modelli del provider.

Se uno dei modelli non può essere identificato, Claude Code non passa automaticamente. La richiesta contrassegnata termina con un messaggio di rifiuto e potete passare i modelli con [`/model`](#setting-your-model) e riprovare. Per abilitare il fallback automatico su questi provider, impostare `ANTHROPIC_DEFAULT_FABLE_MODEL` al vostro ID modello Fable 5 e `ANTHROPIC_DEFAULT_OPUS_MODEL` al vostro ID modello Opus 4.8.

<h4 id="security-research-and-biology-workloads">
  Carichi di lavoro di ricerca sulla sicurezza e biologia
</h4>

I carichi di lavoro in sicurezza offensiva o biologia, inclusi test di penetrazione, esercizi Capture the Flag (CTF) e basi di codice adiacenti alla biologia, attivano il fallback frequentemente, spesso sulla prima richiesta. Per il lavoro sostanziale di biologia, aspettatevi che quasi tutte le richieste vengano reindirizzate.

Questo è il routing previsto per questi domini, non un flag di account. Se la vostra organizzazione ha bisogno di capacità di classe Fable per questo lavoro, chiedete al vostro team di account Anthropic riguardo ai programmi di accesso affidabile.

<h3 id="adjust-effort-level">
  Regolare il livello di sforzo
</h3>

I [livelli di sforzo](https://platform.claude.com/docs/en/build-with-claude/effort) controllano il ragionamento adattivo, che consente al modello di decidere se e quanto pensare ad ogni passo in base alla complessità dell'attività. Lo sforzo inferiore è più veloce ed economico per attività semplici, mentre lo sforzo superiore fornisce un ragionamento più profondo per problemi complessi.

I livelli di sforzo disponibili dipendono dal modello. I modelli non elencati qui non supportano lo sforzo:

| Modello                       | Livelli                                 |
| :---------------------------- | :-------------------------------------- |
| Fable 5                       | `low`, `medium`, `high`, `xhigh`, `max` |
| Sonnet 5, Opus 4.8 e Opus 4.7 | `low`, `medium`, `high`, `xhigh`, `max` |
| Opus 4.6 e Sonnet 4.6         | `low`, `medium`, `high`, `max`          |

Se impostate un livello che il modello attivo non supporta, Claude Code ricade al livello supportato più alto pari o inferiore a quello impostato. Ad esempio, `xhigh` viene eseguito come `high` su Opus 4.6. La vostra organizzazione può anche limitare quali livelli sono disponibili per un modello; vedere [Limiti di sforzo dell'organizzazione](#organization-effort-limits).

Lo sforzo predefinito è `high` su Fable 5, Sonnet 5, Opus 4.8, Opus 4.6 e Sonnet 4.6, e `xhigh` su Opus 4.7.

Quando eseguite Fable 5, Opus 4.8 o Opus 4.7 per la prima volta, Claude Code applica lo sforzo predefinito di quel modello anche se avete precedentemente impostato un livello diverso per un altro modello: `high` su Fable 5 e Opus 4.8, e `xhigh` su Opus 4.7. Eseguite `/effort` di nuovo per scegliere un livello diverso dopo il passaggio. Questo valore predefinito viene mantenuto tra le sessioni fino a quando non effettuate una scelta di sforzo esplicita, come l'esecuzione di `/effort` in una sessione interattiva o l'avvio con `--effort`.

`low`, `medium`, `high` e `xhigh` persistono tra le sessioni quando li impostate in una sessione interattiva. Un livello impostato con `/effort` in [modalità non interattiva](/docs/it/headless), con il flag `-p`, si applica solo alla sessione corrente e non viene salvato come valore predefinito. Un `/effort` non interattivo inoltre non può rilasciare il blocco predefinito del modello sopra: su Fable 5, Opus 4.8 e Opus 4.7 segnala `Not applied` e la sessione rimane allo sforzo predefinito del modello, quindi passare `--effort` al lancio. `max` fornisce il ragionamento più profondo senza vincoli sulla spesa di token e si applica solo alla sessione corrente, tranne quando impostato tramite la variabile di ambiente `CLAUDE_CODE_EFFORT_LEVEL`.

Il menu `/effort` offre anche `ultracode`. Ultracode è un'impostazione di Claude Code piuttosto che un livello di sforzo del modello: invia `xhigh` al modello e inoltre ha Claude orchestrare [flussi di lavoro dinamici](/docs/it/workflows) per attività sostanziali. Si applica solo alla sessione corrente.

Potete attivare ultracode attraverso uno dei seguenti:

* **`/effort`**: eseguire `/effort ultracode`, o selezionarlo dal menu
* **Flag `--effort`**: avviare con `claude --effort ultracode`, che avvia la sessione con sforzo `xhigh` e ultracode attivato
* **`--settings` o una richiesta di controllo Agent SDK**: passare `"ultracode": true`. Una richiesta [`applyFlagSettings()`](/docs/it/agent-sdk/typescript#applyflagsettings) accetta anche `effortLevel: "ultracode"`

Passare `ultracode` al flag `--effort` o al valore `effortLevel` di Agent SDK richiede Claude Code v2.1.203 o successivo. Prima di v2.1.203, `--effort ultracode` stampava `Unknown --effort value 'ultracode'` e la sessione iniziava con lo sforzo predefinito.

L'impostazione `effortLevel` persistente e la variabile di ambiente `CLAUDE_CODE_EFFORT_LEVEL` non accettano `ultracode`.

Quando ultracode non è disponibile, ad esempio quando [i flussi di lavoro sono disattivati](/docs/it/workflows#turn-workflows-off), `--effort ultracode` imposta solo lo sforzo `xhigh`.

<h4 id="choose-an-effort-level">
  Scegliere un livello di sforzo
</h4>

Ogni livello scambia la spesa di token rispetto alla capacità. Il valore predefinito è adatto alla maggior parte delle attività di codifica; regolate quando desiderate un equilibrio diverso.

| Livello     | Quando utilizzarlo                                                                                                                                                             |
| :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `low`       | Riservare per attività brevi, limitate e sensibili alla latenza che non sono sensibili all'intelligenza                                                                        |
| `medium`    | Riduce l'utilizzo di token per il lavoro sensibile ai costi che può scambiare un po' di intelligenza                                                                           |
| `high`      | Bilancia l'utilizzo di token e l'intelligenza. Valore predefinito su Fable 5, Sonnet 5, Opus 4.8, Opus 4.6 e Sonnet 4.6                                                        |
| `xhigh`     | Ragionamento più profondo con spesa di token più elevata. Valore predefinito su Opus 4.7                                                                                       |
| `max`       | Può migliorare le prestazioni su attività impegnative ma potrebbe mostrare rendimenti decrescenti ed è soggetto a pensiero eccessivo. Testare prima di adottare ampiamente     |
| `ultracode` | Un'impostazione di Claude Code che pianifica un [flusso di lavoro dinamico](/docs/it/workflows) per ogni attività sostanziale con ragionamento `xhigh` per messaggio. Solo sessione |

La scala dello sforzo è calibrata per modello, quindi lo stesso nome di livello non rappresenta lo stesso valore sottostante tra i modelli.

<h4 id="use-ultrathink-for-one-off-deep-reasoning">
  Utilizzare ultrathink per il ragionamento profondo una tantum
</h4>

Includete `ultrathink` in qualsiasi punto del prompt per richiedere un ragionamento più profondo su quel turno senza modificare l'impostazione dello sforzo della sessione. Claude Code riconosce la parola chiave e aggiunge un'istruzione in contesto. Il livello di sforzo inviato all'API rimane invariato. Altre frasi come "think", "think hard" e "think more" vengono passate come testo ordinario del prompt e non vengono riconosciute come parole chiave.

<h4 id="set-the-effort-level">
  Impostare il livello di sforzo
</h4>

Potete modificare lo sforzo attraverso uno dei seguenti:

* **`/effort`**: eseguire `/effort` senza argomenti per aprire un cursore interattivo, `/effort` seguito da un nome di livello per impostarlo direttamente, o `/effort auto` per ripristinare il valore predefinito del modello
* **In `/model`**: utilizzare i tasti freccia sinistra/destra per regolare il cursore dello sforzo quando si seleziona un modello
* **Flag `--effort`**: passare un nome di livello per impostarlo per una singola sessione quando si avvia Claude Code
* **Variabile di ambiente**: impostare `CLAUDE_CODE_EFFORT_LEVEL` su un nome di livello o `auto`
* **Impostazioni**: impostare `effortLevel` su `low`, `medium`, `high` o `xhigh` nel file delle impostazioni. `max` e `ultracode` sono [solo per la sessione](#adjust-effort-level) e non sono accettati qui
* **Frontmatter di skill e subagent**: impostare `effort` in un file markdown di [skill](/docs/it/skills#frontmatter-reference) o [subagent](/docs/it/sub-agents#supported-frontmatter-fields) per sovrascrivere il livello di sforzo quando quella skill o subagent viene eseguita

La variabile di ambiente ha la precedenza su tutti gli altri metodi, quindi il livello configurato, quindi il valore predefinito del modello. Lo sforzo del frontmatter si applica quando quella skill o subagent è attiva, sovrascrivendo il livello della sessione ma non la variabile di ambiente.

Il cursore dello sforzo appare in `/model` quando è selezionato un modello supportato. Il livello di sforzo corrente viene visualizzato anche accanto al logo e al spinner, ad esempio "with low effort", in modo da poter confermare quale impostazione è attiva senza aprire `/model`.

<h4 id="adaptive-reasoning-and-fixed-thinking-budgets">
  Ragionamento adattivo e budget di pensiero fissi
</h4>

Il ragionamento adattivo rende il pensiero facoltativo ad ogni passo, quindi Claude può rispondere più velocemente ai prompt di routine e riservare un pensiero più profondo per i passi che ne traggono vantaggio. Se desiderate che Claude pensi più o meno spesso di quanto il livello corrente produce, potete dirlo direttamente nel prompt o in `CLAUDE.md`; il modello risponde a quella guida entro l'impostazione dello sforzo.

Fable 5, Sonnet 5 e Opus 4.7 e versioni successive utilizzano sempre il ragionamento adattivo. La modalità budget di pensiero fisso e `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING` non si applicano ad essi.

Su Opus 4.6 e Sonnet 4.6, potete impostare `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` per ripristinare il budget di pensiero fisso precedente controllato da `MAX_THINKING_TOKENS`. Vedere [variabili di ambiente](/docs/it/env-vars).

<h3 id="extended-thinking">
  Pensiero esteso
</h3>

Il pensiero esteso è il ragionamento che Claude emette prima di rispondere. Sui modelli che supportano il [ragionamento adattivo](#adjust-effort-level), il livello di sforzo è il controllo principale per la quantità di pensiero che si verifica; le impostazioni seguenti attivano o disattivano il pensiero e controllano come viene visualizzato.

| Controllo                                   | Come impostarlo                                                                                                                                                                                                                                                                                                                                                                                              |
| :------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Attiva/disattiva per la sessione corrente   | Premere `Option+T` su macOS o `Alt+T` su Windows e Linux                                                                                                                                                                                                                                                                                                                                                     |
| Impostare il valore predefinito globale     | Eseguire `/config` e attivare/disattivare la modalità di pensiero. Salvato come `alwaysThinkingEnabled` in `~/.claude/settings.json`                                                                                                                                                                                                                                                                         |
| Disabilitare indipendentemente dallo sforzo | Impostare [`MAX_THINKING_TOKENS=0`](/docs/it/env-vars), che disattiva il pensiero su API Anthropic tranne su Fable 5. Su [provider di terze parti](/docs/it/third-party-integrations) questo omette il parametro `thinking` invece, e i modelli di ragionamento adattivo potrebbero comunque pensare. Altri valori si applicano solo con un [budget di pensiero fisso](#adaptive-reasoning-and-fixed-thinking-budgets) |

Il pensiero non può essere disattivato su Fable 5. L'attivazione/disattivazione della sessione, `alwaysThinkingEnabled` e `MAX_THINKING_TOKENS=0` non hanno effetto lì, e Fable 5 decide per ogni passo quanto pensare in base al livello di sforzo.

L'output del pensiero è compresso per impostazione predefinita. Premere `Ctrl+O` per attivare/disattivare la modalità dettagliata e visualizzare il ragionamento come testo grigio in corsivo. Le sessioni interattive su API Anthropic ricevono blocchi di pensiero redatti per impostazione predefinita, quindi impostare `showThinkingSummaries: true` nelle [impostazioni](/docs/it/settings) se desiderate che i riepiloghi completi siano disponibili quando li espandete. Viene addebitato il costo di tutti i token di pensiero generati, anche quando compressi o redatti.

<h3 id="extended-context">
  Contesto esteso
</h3>

Fable 5, Sonnet 5, Opus 4.6 e versioni successive, e Sonnet 4.6 supportano una [finestra di contesto di 1 milione di token](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) per sessioni lunghe con basi di codice di grandi dimensioni.

La disponibilità varia in base al modello e al piano. Su API Anthropic, Fable 5, Sonnet 5, Opus 4.8 e Opus 4.7 vengono sempre eseguiti con la finestra 1M. Nei piani Max, Team ed Enterprise, Opus viene automaticamente aggiornato al contesto 1M senza configurazione aggiuntiva. Questo si applica sia ai posti Team Standard che Team Premium. Sonnet 4.6 con contesto 1M non fa parte dell'aggiornamento automatico e richiede [crediti di utilizzo](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) su ogni piano di abbonamento, incluso Max.

| Piano                              | Opus con contesto 1M                                                                                              | Sonnet 4.6 con contesto 1M                                                                                        |
| ---------------------------------- | ----------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| Max, Team ed Enterprise            | Incluso nell'abbonamento                                                                                          | Richiede [crediti di utilizzo](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) |
| Pro                                | Richiede [crediti di utilizzo](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) | Richiede [crediti di utilizzo](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) |
| API e pagamento in base al consumo | Accesso completo                                                                                                  | Accesso completo                                                                                                  |

Per disabilitare completamente il contesto 1M, impostare `CLAUDE_CODE_DISABLE_1M_CONTEXT=1`. Questo rimuove le varianti di modello 1M dal selettore di modelli. Vedere [variabili di ambiente](/docs/it/env-vars).

La finestra di contesto 1M utilizza i prezzi standard del modello senza premio per i token oltre 200K. Per i piani in cui il contesto esteso è incluso nell'abbonamento, l'utilizzo rimane coperto dall'abbonamento. Per i piani che accedono al contesto esteso tramite crediti di utilizzo, i token vengono fatturati ai crediti di utilizzo.

Se l'account supporta il contesto 1M, l'opzione appare nel selettore di modelli (`/model`) nelle versioni più recenti di Claude Code. Se non la vedete, provate a riavviare la sessione.

Potete anche utilizzare il suffisso `[1m]` con alias di modelli o nomi di modelli completi:

```bash theme={null}
# Utilizzare l'alias opus[1m] o sonnet[1m]
/model opus[1m]
/model sonnet[1m]

# O aggiungere [1m] a un nome di modello completo
/model claude-opus-4-8[1m]
```

<h4 id="sonnet-5-context-window">
  Finestra di contesto di Sonnet 5
</h4>

Su API Anthropic, Sonnet 5 viene sempre eseguito con la finestra di contesto 1M. Non esiste una variante 200K, nessun suffisso `[1m]` da selezionare e nessun credito di utilizzo richiesto su alcun piano. Le sessioni si auto-compattano prima che la finestra si riempia, a circa 967K token per impostazione predefinita; impostare [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/it/env-vars) per scegliere una soglia diversa.

Due configurazioni limitano invece la finestra a 200K e si auto-compattano a quel limite:

* **Gateway LLM**: quando `ANTHROPIC_BASE_URL` punta a un [gateway](/docs/it/llm-gateway), Claude Code non può verificare il supporto 1M. Per utilizzare la finestra completa, selezionare Sonnet 5 (1M context) nel selettore di modelli, che corrisponde a `sonnet[1m]`.
* **`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`**: tratta le sessioni Sonnet 5 come aventi una finestra 200K, per le distribuzioni che devono limitare il contesto.

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

Questo esempio imposta tutte e tre le variabili per rendere selezionabile una distribuzione Opus instradata tramite gateway:

```bash theme={null}
export ANTHROPIC_CUSTOM_MODEL_OPTION="my-gateway/claude-opus-4-8"
export ANTHROPIC_CUSTOM_MODEL_OPTION_NAME="Opus via Gateway"
export ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION="Custom deployment routed through the internal LLM gateway"
```

La voce personalizzato appare in fondo al selettore `/model`. `ANTHROPIC_CUSTOM_MODEL_OPTION_NAME` e `ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION` sono facoltativi. Se omessi, l'ID del modello viene utilizzato come nome e la descrizione per impostazione predefinita è `Custom model (<model-id>)`.

Claude Code salta la convalida per l'ID del modello impostato in `ANTHROPIC_CUSTOM_MODEL_OPTION`, quindi è possibile utilizzare qualsiasi stringa che l'endpoint API accetta. Quando [`availableModels`](#restrict-model-selection) è impostato, includere l'ID del modello personalizzato anche nell'elenco di autorizzazione: la voce personalizzata viene filtrata dal selettore e una selezione `--model` di essa viene rifiutata come qualsiasi altro modello escluso. Un ID personalizzato che incorpora un nome di famiglia, come `my-gateway/claude-opus-4-8`, conta come una voce specifica per quella famiglia e disabilita il suo wildcard, quindi elencare anche le versioni che intendete mantenere selezionabili. Vedere [Comportamento di unione](#merge-behavior).

<h2 id="environment-variables">
  Variabili di ambiente
</h2>

È possibile utilizzare le seguenti variabili di ambiente per controllare i nomi dei modelli a cui gli alias si mappano. Ogni valore deve essere un nome di modello completo, o l'identificatore equivalente per il provider API.

| Variabile di ambiente            | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `ANTHROPIC_DEFAULT_FABLE_MODEL`  | Il modello da utilizzare per `fable`, e l'ID del modello che Claude Code riconosce come Fable 5 per il [fallback automatico del modello](#automatic-model-fallback) su provider di terze parti                                                                                                                                                                                                                                             |
| `ANTHROPIC_DEFAULT_OPUS_MODEL`   | Il modello da utilizzare per `opus`, o per `opusplan` quando Plan Mode è attivo.                                                                                                                                                                                                                                                                                                                                                           |
| `ANTHROPIC_DEFAULT_SONNET_MODEL` | Il modello da utilizzare per `sonnet`, o per `opusplan` quando Plan Mode non è attivo.                                                                                                                                                                                                                                                                                                                                                     |
| `ANTHROPIC_DEFAULT_HAIKU_MODEL`  | Il modello da utilizzare per `haiku`, o [funzionalità in background](/docs/it/costs#background-token-usage)                                                                                                                                                                                                                                                                                                                                     |
| `CLAUDE_CODE_SUBAGENT_MODEL`     | Il modello da utilizzare per tutti i [subagents](/docs/it/sub-agents#choose-a-model), i [team di agenti](/docs/it/agent-teams), e gli agenti che un [workflow](/docs/it/workflows) esegue. Accetta un alias come `haiku` o un nome di modello completo, e sostituisce sia il parametro `model` per invocazione che il frontmatter `model` della definizione del subagent. Impostare su `inherit` per utilizzare la risoluzione del modello normale invece |

Nota: `ANTHROPIC_SMALL_FAST_MODEL` è deprecato a favore di `ANTHROPIC_DEFAULT_HAIKU_MODEL`.

<h3 id="pin-models-for-third-party-deployments">
  Fissare i modelli per distribuzioni di terze parti
</h3>

Quando si distribuisce Claude Code tramite [Amazon Bedrock](/docs/it/amazon-bedrock), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai), [Microsoft Foundry](/docs/it/microsoft-foundry), o [Claude Platform on AWS](/docs/it/claude-platform-on-aws), fissare le versioni dei modelli prima di distribuire agli utenti.

Senza fissaggio, Claude Code utilizza alias di modelli come `fable`, `opus`, `sonnet` e `haiku` che si risolvono in un ID di modello predefinito incorporato per ogni provider. Tale impostazione predefinita può rimanere indietro rispetto alla versione più recente di Anthropic, e il modello a cui punta potrebbe non essere ancora abilitato nell'account di un utente. Quando l'impostazione predefinita non è disponibile, gli utenti Amazon Bedrock e Google Cloud's Agent Platform vedono un avviso e la sessione ricade nella versione precedente del modello predefinito, o nel modello Sonnet predefinito quando l'impostazione predefinita è un modello Opus e nessuna versione di Opus è disponibile. Gli utenti Microsoft Foundry vedono errori invece, perché Microsoft Foundry non ha alcun controllo di avvio equivalente.

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

Per abilitare il [contesto esteso](#extended-context) per un modello fissato, aggiungere `[1m]` all'ID del modello in `ANTHROPIC_DEFAULT_OPUS_MODEL` o `ANTHROPIC_DEFAULT_SONNET_MODEL`:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8[1m]'
```

Il suffisso `[1m]` applica la finestra di contesto 1M a tutto l'utilizzo degli alias `opus` e `sonnet`, inclusa la fase Opus in modalità piano di [`opusplan`](#opusplan-model-setting).

* Claude Code rimuove il suffisso prima di inviare l'ID del modello al provider.
* Aggiungere `[1m]` solo quando il modello sottostante [supporta il contesto 1M](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model).
* Il suffisso viene letto per variabile, non per modello. Su Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry, un ID di modello senza `[1m]` in una variabile utilizza il contesto 200K anche se un'altra variabile imposta lo stesso modello con il suffisso. Sonnet 5 viene sempre eseguito con la finestra 1M su questi provider e non ha mai bisogno del suffisso.

<Note>
  Un elenco di autorizzazione `availableModels` fornito tramite [MDM o un file di impostazioni gestite](/docs/it/settings#settings-files) si applica comunque quando si utilizzano provider di terze parti; [le impostazioni gestite dal server non vengono fornite lì](/docs/it/server-managed-settings#platform-availability). Il filtraggio corrisponde a un alias di modello come `opus`, a un prefisso di versione come `claude-opus-4-8`, o all'ID di modello completo in forma di provider. I prefissi specifici del provider come `us.anthropic.` non vengono rimossi, quindi per consentire un modello specifico, elencare lo stesso ID in forma di provider che il selettore mostra, o mapparlo tramite [`modelOverrides`](#override-model-ids-per-version). Qualsiasi suffisso `[1m]` viene rimosso sia dalla voce dell'elenco di autorizzazione che dal modello richiesto prima della corrispondenza.
</Note>

<h3 id="customize-pinned-model-display-and-capabilities">
  Personalizzare la visualizzazione e le capacità del modello fissato
</h3>

Quando si fissa un modello su un provider di terze parti, l'ID specifico del provider appare così com'è nel selettore `/model` e Claude Code potrebbe non riconoscere quali funzionalità il modello supporta. È possibile sovrascrivere il nome di visualizzazione e dichiarare le capacità con variabili di ambiente complementari per ogni modello fissato.

Queste variabili hanno effetto su provider di terze parti come Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry. Le variabili `_NAME` e `_DESCRIPTION` hanno effetto anche quando `ANTHROPIC_BASE_URL` punta a un [gateway LLM](/docs/it/llm-gateway). Non hanno effetto quando si effettua la connessione direttamente a `api.anthropic.com`.

| Variabile di ambiente                                 | Descrizione                                                                                                                                              |
| ----------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_NAME`                   | Nome di visualizzazione per il modello Opus fissato nel selettore `/model`. Per impostazione predefinita l'ID del modello quando non impostato           |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_DESCRIPTION`            | Descrizione di visualizzazione per il modello Opus fissato nel selettore `/model`. Per impostazione predefinita `Custom Opus model` quando non impostato |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES` | Elenco separato da virgole delle capacità che il modello Opus fissato supporta                                                                           |

Gli stessi suffissi `_NAME`, `_DESCRIPTION` e `_SUPPORTED_CAPABILITIES` sono disponibili per `ANTHROPIC_DEFAULT_SONNET_MODEL`, `ANTHROPIC_DEFAULT_HAIKU_MODEL`, `ANTHROPIC_DEFAULT_FABLE_MODEL` e `ANTHROPIC_CUSTOM_MODEL_OPTION`.

Claude Code abilita funzionalità come [livelli di sforzo](#adjust-effort-level) e [extended thinking](#extended-thinking) abbinando l'ID del modello rispetto a modelli noti. Gli ID specifici del provider come ARN Bedrock o nomi di distribuzione personalizzati spesso non corrispondono a questi modelli, lasciando le funzionalità supportate disabilitate. Impostare `_SUPPORTED_CAPABILITIES` per dire a Claude Code quali funzionalità il modello effettivamente supporta:

| Valore di capacità     | Abilita                                                                                           |
| ---------------------- | ------------------------------------------------------------------------------------------------- |
| `effort`               | [Livelli di sforzo](#adjust-effort-level) e il comando `/effort`                                  |
| `xhigh_effort`         | Il livello di sforzo `xhigh`                                                                      |
| `max_effort`           | Il livello di sforzo `max`                                                                        |
| `thinking`             | [Extended thinking](#extended-thinking)                                                           |
| `adaptive_thinking`    | Ragionamento adattivo che alloca dinamicamente il pensiero in base alla complessità dell'attività |
| `interleaved_thinking` | Pensiero tra le chiamate di strumento                                                             |

Quando `_SUPPORTED_CAPABILITIES` è impostato, le capacità elencate sono abilitate e le capacità non elencate sono disabilitate per il modello fissato corrispondente. Quando la variabile non è impostata, Claude Code ricade sulla rilevazione incorporata basata sull'ID del modello.

Questo esempio fissa Opus a un ARN di modello personalizzato Bedrock, imposta un nome amichevole e dichiara le sue capacità:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='arn:aws:bedrock:us-east-1:123456789012:custom-model/abc'
export ANTHROPIC_DEFAULT_OPUS_MODEL_NAME='Opus via Bedrock'
export ANTHROPIC_DEFAULT_OPUS_MODEL_DESCRIPTION='Opus 4.7 routed through a Bedrock custom endpoint'
export ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES='effort,xhigh_effort,max_effort,thinking,adaptive_thinking,interleaved_thinking'
```

<h3 id="override-model-ids-per-version">
  Eseguire l'override degli ID di modello per versione
</h3>

Le variabili di ambiente a livello di famiglia sopra configurano un ID di modello per alias di famiglia. Se è necessario mappare diverse versioni all'interno della stessa famiglia a ID di provider distinti, utilizzare invece l'impostazione `modelOverrides`.

`modelOverrides` mappa i singoli ID di modello Anthropic alle stringhe specifiche del provider che Claude Code invia all'API del provider. Quando un utente seleziona un modello mappato nel selettore `/model`, Claude Code utilizza il valore configurato invece del valore predefinito incorporato.

Questo consente agli amministratori aziendali di instradare ogni versione del modello a un ARN di profilo di inferenza Bedrock specifico, a un nome di versione Google Cloud's Agent Platform o a un nome di distribuzione Microsoft Foundry per governance, allocazione dei costi o instradamento regionale.

Impostare `modelOverrides` nel [file delle impostazioni](/docs/it/settings#settings-files):

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

Gli override sostituiscono gli ID di modello incorporati che supportano ogni voce nel selettore `/model`. Su Amazon Bedrock, gli override hanno la precedenza su qualsiasi profilo di inferenza che Claude Code scopre automaticamente all'avvio. Claude Code passa i valori che sono già nativi del provider, come ARN di profilo di inferenza Amazon Bedrock o nomi di distribuzione Microsoft Foundry, al provider così come sono.

Gli override si applicano anche quando si passa un ID di modello Anthropic direttamente tramite `--model`, la variabile di ambiente `ANTHROPIC_MODEL`, o una variabile di ambiente `ANTHROPIC_DEFAULT_*_MODEL`. Su Amazon Bedrock, Google Cloud's Agent Platform e [Mantle](/docs/it/amazon-bedrock#use-the-mantle-endpoint), un ID di modello Anthropic senza voce `modelOverrides` si risolve nello stesso ID specifico del provider della riga del selettore `/model` per quella versione, quando il provider supporta quella versione. Mantle supporta un sottoinsieme di versioni. Per un ID di modello Anthropic al di fuori di quel sottoinsieme, Claude Code invia l'ID grezzo a Mantle senza mapparlo, a meno che una voce `modelOverrides` lo copra. Prima della v2.1.200, `--model` e i valori delle variabili di ambiente raggiungevano il provider così come erano senza passare attraverso la mappa di override.

`modelOverrides` funziona insieme a `availableModels`. L'elenco di autorizzazione viene valutato rispetto all'ID di modello Anthropic, non al valore di override, quindi una voce come `"opus"` in `availableModels` continua a corrispondere anche quando le versioni di Opus sono mappate a ARN. Quando `enforceAvailableModels` è impostato nelle impostazioni gestite, il Default applicato si risolve tramite `modelOverrides` dalla [fonte gestita con precedenza più alta](/docs/it/server-managed-settings#settings-precedence) solo. Il mapping di un amministratore, come una versione fissata a un ARN di profilo di inferenza, viene rispettato nel Default applicato. Gli override dalle impostazioni utente o progetto non lo influenzano.

Quando `availableModels` è impostato nelle [impostazioni gestite](/docs/it/settings#settings-files), solo `modelOverrides` da quella fonte gestita si applicano a un ID di modello Anthropic passato direttamente tramite `--model` o le variabili di ambiente sopra. Claude Code ignora gli override nelle impostazioni utente o progetto per quegli ID, e non risolve mai un ID che l'elenco gestito esclude tramite `modelOverrides` da alcuna fonte di impostazioni. Questa restrizione di fonte gestita richiede Claude Code v2.1.200 o successivo. Vedere [Limitare la selezione del modello](#restrict-model-selection) per come vengono gestiti gli ID bloccati.

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

Per modificare il TTL della cache o scoprire cosa attiva un cache miss, vedere [Come Claude Code utilizza la prompt caching](/docs/it/prompt-caching).
