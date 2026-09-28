> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Escalate hard decisions with the advisor tool

> Abbina il tuo modello principale con un modello advisor più potente che Claude consulta nei momenti chiave durante un'attività.

<Note>
  Lo strumento advisor è sperimentale e richiede l'API Anthropic. Non è disponibile su Amazon Bedrock, Claude Platform su AWS, Google Cloud's Agent Platform o Microsoft Foundry. Il comportamento, i prezzi e la disponibilità potrebbero cambiare.
</Note>

Lo strumento advisor consente a Claude di consultare un secondo modello, tipicamente più potente, nei momenti chiave durante un'attività, ad esempio prima di impegnarsi in un approccio, quando bloccato su un errore ricorrente, o prima di dichiarare un'attività completata. L'advisor riceve l'intera conversazione, incluse tutte le chiamate agli strumenti e i risultati, e restituisce una guida che Claude applica prima di continuare.

L'advisor viene eseguito lato server sull'infrastruttura di Anthropic come [server tool](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool), disponibile sia per gli account con abbonamento che per quelli fatturati tramite API. Scegli quale modello agisce come advisor, e Claude decide quando chiamarlo.

Questa pagina spiega come abilitare l'advisor, quali accoppiamenti di modelli sono accettati, cosa Claude mostra durante una consultazione e come viene fatturato l'utilizzo dell'advisor.

<h2 id="when-to-use-the-advisor">
  Quando utilizzare l'advisor
</h2>

L'advisor è adatto per attività lunghe e multi-step dove la maggior parte dei turni sono di routine ma la qualità del piano determina il risultato. Gli esempi includono refactoring di grandi dimensioni, sessioni di debug dove un errore continua a ripetersi, e attività che desideri controllare in modo indipendente prima che Claude le dichiari completate.

Aggiunge meno valore su attività brevi dove c'è poco da pianificare, o su lavori dove ogni turno ha bisogno del modello più potente. Per questi, [cambia il modello principale](/docs/it/model-config#setting-your-model) invece, o vedi [come l'advisor si confronta con opusplan e subagents](#compare-with-related-features) per altri modi di ottenere un secondo parere.

<h2 id="enable-the-advisor">
  Abilita l'advisor
</h2>

Puoi impostare il modello advisor in tre modi:

* **Comando `/advisor`**: imposta o cambia l'advisor a metà sessione e salvalo come predefinito
* **Impostazione `advisorModel`**: configura un predefinito persistente nel tuo [file di impostazioni](/docs/it/settings)
* **Flag `--advisor`**: imposta l'advisor per una singola sessione al lancio

Ognuno di questi abilita l'advisor per le sessioni il cui modello principale lo [supporta](#choose-an-advisor-model). Dopo l'avvio della sessione, Claude Code mostra una notifica `Advisor Tool (experimental) is on and may use more tokens · /advisor`. Per smettere di usare l'advisor, vedi [Disattiva l'advisor](#turn-the-advisor-off).

Su alcuni piani, Fable come advisor ha anche bisogno del tuo [consenso una tantum per fatturare l'utilizzo di Fable ai crediti di utilizzo](/docs/it/model-config#fable-and-usage-credits). Per sapere cosa accade prima di aver dato quel consenso, vedi [Advisor Fable e crediti di utilizzo](#fable-advisor-and-usage-credits).

<h3 id="use-the-/advisor-command">
  Usa il comando `/advisor`
</h3>

Esegui `/advisor` senza argomenti per aprire un selettore che elenca i modelli advisor disponibili, o passa il modello direttamente:

```
/advisor opus
```

Il comando conferma con `Advisor set to` seguito dal nome del modello advisor. La tua selezione viene salvata in `advisorModel` nelle impostazioni utente e persiste tra le sessioni, eccetto nei casi che la voce [`advisorModel`](/docs/it/settings-reference#advisormodel) elenca come applicabili solo alla sessione corrente.

Il comando funziona anche dove non c'è un selettore di terminale: in [modalità non interattiva](/docs/it/headless) con `-p`, nell'Agent SDK, nell'app desktop e su [Remote Control](/docs/it/remote-control). Questo richiede Claude Code v2.1.260 o successivo. Su quelle superfici:

* Esegui `/advisor` senza argomento per stampare il modello advisor corrente e gli alias che accetta.
* Esegui `/advisor` con un modello, come `/advisor opus`, per impostarlo.
* Esegui `/advisor off` per disattivarlo.

Claude Code non invoca un advisor salvato che l'allowlist [`availableModels`](/docs/it/model-config#restrict-model-selection) della tua organizzazione esclude. Per usare l'advisor, scegli un modello consentito con `/advisor`. Claude Code salva comunque un advisor che il tuo modello principale attuale non supporta. Quell'advisor si attiva dopo che passi a un modello principale [compatibile](#choose-an-advisor-model) con [`/model`](/docs/it/model-config#setting-your-model). Se l'API ha già rifiutato l'advisor salvato nella conversazione corrente, rimane disattivato fino a `/clear` o `/compact`, anche dopo aver cambiato modelli.

Su alcuni piani, Fable come advisor ha anche bisogno del tuo [consenso una tantum per fatturare l'utilizzo di Fable ai crediti di utilizzo](/docs/it/model-config#fable-and-usage-credits). Per sapere cosa fa `/advisor fable` prima di aver dato quel consenso, vedi [Advisor Fable e crediti di utilizzo](#fable-advisor-and-usage-credits).

<h3 id="set-advisormodel-in-settings">
  Imposta `advisorModel` nelle impostazioni
</h3>

Per configurare l'advisor come predefinito senza aprire una sessione, impostalo nel tuo file di impostazioni:

```json theme={null}
{
  "advisorModel": "opus"
}
```

<h3 id="use-the-advisor-flag">
  Usa il flag `--advisor`
</h3>

Per impostare l'advisor per una singola sessione senza modificare l'impostazione salvata, avvia con il flag:

```bash theme={null}
claude --advisor opus
```

Claude Code usa il flag al posto dell'impostazione `advisorModel` per quella sessione. Non elenca `--advisor` in `claude --help`. Claude Code esce con un errore al lancio se:

* Il modello principale della sessione non supporta l'advisor
* Il modello richiesto, come Haiku, non può agire come advisor
* L'allowlist [`availableModels`](/docs/it/model-config#restrict-model-selection) della tua organizzazione esclude il modello richiesto
* Hai richiesto Fable e il tuo account richiede ancora il [consenso ai crediti di utilizzo](#fable-advisor-and-usage-credits)

Se avvii una [sessione in background](/docs/it/agent-view) con `--advisor` e uno di questi si applica, Claude Code avvia la sessione senza l'advisor invece di uscire.

<h2 id="choose-an-advisor-model">
  Scegli un modello advisor
</h2>

L'advisor deve essere almeno altrettanto capace del modello principale. Gli advisor accettati per ogni modello principale sono:

| Modello principale  | Advisor accettati                      | Note                                                                                        |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------- |
| Haiku 4.5           | Fable, Opus, Sonnet                    | Haiku può chiamare l'advisor ma non può agire come uno                                      |
| Sonnet 4.6          | Fable, Opus, Sonnet                    |                                                                                             |
| Sonnet 5            | Fable, Opus 4.7 o successivo, Sonnet 5 | Un advisor Sonnet 4.6 viene rifiutato e l'API rifiuta un advisor Opus 4.6                   |
| Opus 4.6            | Fable, Opus, Sonnet 5                  | Un advisor Sonnet 4.6 viene rifiutato                                                       |
| Opus 4.7 o Opus 4.8 | Fable, e Opus 4.7 o successivo         | Un advisor Opus 4.6 o Sonnet viene rifiutato                                                |
| Opus 5.5 o Opus 5   | Fable, e Opus 5 o successivo           | Un advisor Opus 4.6 o Sonnet viene rifiutato e l'API rifiuta un advisor Opus 4.7 o Opus 4.8 |
| Fable 5             | Fable 5.1 o Fable 5                    | Un advisor Opus o Sonnet viene rifiutato                                                    |
| Fable 5.1           | Fable 5.1                              | Un advisor Opus o Sonnet viene rifiutato e l'API rifiuta un advisor Fable 5                 |

Fable 5.1 richiede Claude Code v2.1.257 o successivo. Entrambi i modelli Fable richiedono [accesso a Fable](/docs/it/model-config#work-with-fable).

Imposta l'advisor come `fable`, `opus`, o `sonnet`. Questi alias si risolvono nella versione predefinita integrata di Claude Code per ogni famiglia di modelli, che avanza con le nuove versioni di Claude Code. Puoi anche passare un ID modello completo come `claude-opus-5-5`.

I subagent ereditano l'advisor configurato e applicano lo stesso controllo di accoppiamento rispetto al loro modello.

Claude Code convalida l'accoppiamento prima di inviare una richiesta e l'API lo convalida di nuovo:

* Per un advisor che la tabella elenca come rifiutato, Claude Code non lo allega alle richieste del modello principale. L'output del comando `/advisor` e una notifica mostrano questo. I subagent il cui modello soddisfa l'accoppiamento possono comunque utilizzare l'advisor.
* Per un advisor che la tabella elenca come rifiutato dall'API, Claude Code lo allega e l'API lo rifiuta. Claude Code quindi rinvia quella richiesta senza l'advisor e il resto della conversazione procede senza uno, quindi non vedi alcun errore e non ottieni alcuna chiamata advisor. Scegli un advisor accettato con `/advisor`; il cambiamento ha effetto dopo `/clear` o `/compact` e nelle nuove sessioni.
* Se il modello principale o l'advisor è un modello che Claude Code non riconosce, l'advisor non è allegato.

<h3 id="fable-advisor-and-usage-credits">
  Advisor Fable e crediti di utilizzo
</h3>

Su alcuni piani, l'utilizzo di Fable viene addebitato ai crediti di utilizzo, e Fable come advisor viene addebitato allo stesso modo. Se il tuo account richiede il [consenso una tantum per addebitare l'utilizzo di Fable ai crediti di utilizzo](/docs/it/model-config#fable-and-usage-credits), Claude Code lo chiede quando selezioni un modello Fable con `/model` e non applica Fable come advisor finché non hai accettato quel consenso.

Prima di averlo accettato, Claude Code non salva Fable come advisor quando digiti `/advisor fable` o scegli Fable nel selettore `/advisor`. Ti indirizza invece a `/model fable`. Con `claude --advisor fable`, Claude Code esce al lancio con un messaggio che punta a `/model fable`. In una [sessione in background](#use-the-advisor-flag), avvia la sessione senza l'advisor invece di uscire. Con Fable già salvato come tuo `advisorModel`, Claude Code invia richieste senza l'advisor. In una sessione interattiva il cui modello principale supporta l'advisor, mostra anche una notifica che punta a `/model fable`.

Per accettare il consenso, esegui `/model fable` e scegli di continuare su Fable. Claude Code registra il consenso e [salva Fable come modello selezionato](/docs/it/model-config#default-model-setting). Quindi seleziona Fable come advisor.

<h3 id="common-model-pairings">
  Accoppiamenti di modelli comuni
</h3>

Qualsiasi accoppiamento accettato funziona. Queste combinazioni bilanciano il costo rispetto alla capacità in modi diversi:

| Accoppiamento                      | Quando utilizzare                                                                                                                                                            |
| ---------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Sonnet principale + advisor Opus   | Sonnet gestisce il lavoro di routine e escalation della pianificazione, fallimenti ambigui e controlli di completamento a Opus                                               |
| Sonnet principale + advisor Fable  | Guida Fable nei punti decisionali senza eseguire Fable in tutto. Richiede accesso a Fable                                                                                    |
| Haiku principale + advisor Opus    | Modello principale a costo più basso con pianificazione forte. Aspettati un costo più alto di Haiku da solo ma inferiore al passaggio del modello principale a Sonnet o Opus |
| Opus principale + advisor Opus     | Un secondo Opus esamina il primo. Utile per attività ad alto rischio dove un controllo indipendente è più importante del costo                                               |
| Fable principale + advisor Fable   | Accoppiamento con capacità più alta quando Fable è disponibile. Claude Code non applica un advisor Opus o Sonnet a un modello principale Fable                               |
| Sonnet principale + advisor Sonnet | Un secondo parere a costo inferiore per catturare errori di routine                                                                                                          |

<h2 id="when-claude-consults-the-advisor">
  Quando Claude consulta l'advisor
</h2>

Claude decide quando chiamare l'advisor. Tende a consultare prima di impegnarsi in un approccio, quando un errore continua a ripetersi, e prima di dichiarare un'attività completata, ma i tempi sono guidati dal modello piuttosto che basati su regole.

Puoi chiedere una consultazione nel tuo prompt nello stesso modo in cui richiederesti qualsiasi strumento, ad esempio `consult the advisor before you continue`. Non c'è un'impostazione per limitare o forzare le chiamate dell'advisor; se desideri che Claude consulti più o meno spesso durante un'attività, dillo nelle tue istruzioni.

<h2 id="what-you-see-during-a-session">
  Cosa vedi durante una sessione
</h2>

Quando Claude chiama l'advisor, la trascrizione mostra una riga `Advising` con il nome del modello advisor mentre la chiamata è in corso. Quando il risultato ritorna, la riga riporta se l'advisor ha fornito una guida:

* **Reviewed**: la riga conferma che l'advisor ha esaminato la conversazione. Quando l'advisor ha restituito una guida leggibile, premi `Ctrl+O` per leggerla.
* **Declined**: la riga legge `Advisor declined to advise on this request`. Se l'advisor ha fornito un motivo, premi `Ctrl+O` per leggerlo.

Claude generalmente segue la guida dell'advisor, ma si adatta quando le sue stesse prove contraddicono un'affermazione specifica: se un passaggio consigliato fallisce quando provato, o il contenuto del file contraddice il consiglio, Claude evidenzia il conflitto piuttosto che seguire la guida incondizionatamente.

L'advisor riceve sempre la conversazione completa, e Claude controlla i tempi. Per un maggiore controllo o una configurazione diversa, vedi [come l'advisor si confronta con subagents e opusplan](#compare-with-related-features).

<h2 id="cost">
  Costo
</h2>

Quando Claude chiama l'advisor, il modello advisor legge la conversazione, quindi ogni chiamata consuma token alle tariffe del modello advisor in aggiunta all'utilizzo del tuo modello principale. Come questi token dell'advisor vengono fatturati dipende da come paghi:

* **Fatturazione API**: paghi le tariffe di input e output del modello advisor per i token dell'advisor
* **Piani di abbonamento**: l'utilizzo dell'advisor conta verso i limiti di utilizzo del tuo piano, ad eccezione del fatto che un advisor Fable viene fatturato ai [crediti di utilizzo](/docs/it/model-config#fable-and-usage-credits) sui piani in cui l'utilizzo di Fable lo fa

Se il tuo account richiede il consenso ai crediti di utilizzo, un advisor Fable non viene fatturato finché non lo dai, perché Claude Code [non applica la selezione](#fable-advisor-and-usage-credits) fino ad allora.

Claude chiama l'advisor nei punti decisionali piuttosto che ad ogni turno, quindi l'accoppiamento di un modello principale più veloce con un advisor più potente in genere costa meno che eseguire il modello più potente in tutto. L'utilizzo dell'advisor conta verso i totali della sessione mostrati da [`/usage`](/docs/it/costs#track-your-costs).

Per come i token dell'advisor vengono segnalati nelle risposte API, vedi [Usage and billing](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool#usage-and-billing) nella documentazione dell'API Claude.

<h2 id="impact-on-prompt-caching">
  Impatto sulla memorizzazione nella cache dei prompt
</h2>

L'abilitazione o la disabilitazione dell'advisor a metà sessione non invalida la [cache dei prompt](/docs/it/prompt-caching) del tuo modello principale. A differenza di [cambiare modello](/docs/it/prompt-caching#switching-models), l'attivazione/disattivazione di `/advisor` mantiene il prefisso memorizzato nella cache intatto, e la guida restituita dall'advisor viene memorizzata nella cache come parte della trascrizione nei turni successivi.

La lettura della conversazione da parte del modello advisor stesso non viene memorizzata nella cache. Ogni chiamata dell'advisor elabora la trascrizione completa da capo, senza riutilizzo tra le chiamate.

<h2 id="requirements">
  Requisiti
</h2>

Lo strumento advisor richiede tutti i seguenti:

* **Solo API Anthropic**: l'advisor è uno strumento eseguito dal server. Non è disponibile su Amazon Bedrock, Claude Platform su AWS, Google Cloud's Agent Platform, o Microsoft Foundry. Attraverso un [gateway LLM](/docs/it/llm-gateway) configurato con `ANTHROPIC_BASE_URL`, la disponibilità dipende dal fatto che il gateway inoltri la richiesta intatta all'API Anthropic. Se il gateway o il suo upstream non riconosce lo strumento advisor, vedere [Retry automatico e inoltro degli errori](/docs/it/llm-gateway-protocol#automatic-retry-and-error-forwarding) per sapere come Claude Code risponde.
* **Modello principale supportato**: Fable, Opus 4.6 o successivo, Sonnet 4.6 o successivo, o Haiku 4.5. Vedere [Scegliere un modello advisor](#choose-an-advisor-model) per sapere quali advisor accettano ciascuno.
* **Recupero del feature flag**: Claude Code attiva l'advisor tramite un feature flag che recupera da Anthropic. In una sessione in cui è impostata una variabile che disattiva il recupero dei flag, come `DISABLE_TELEMETRY`, l'advisor rimane disattivato. Vedere [Funzionalità che richiedono il recupero del feature flag](/docs/it/env-vars#features-that-need-feature-flag-fetching).

<h2 id="turn-the-advisor-off">
  Disattiva l'advisor
</h2>

Per smettere di usare l'advisor, esegui `/advisor off` o scegli **No advisor** nel selettore `/advisor`:

```
/advisor off
```

Per disabilitare completamente lo strumento advisor, imposta `CLAUDE_CODE_DISABLE_ADVISOR_TOOL=1`. Il comando `/advisor` diventa non disponibile e qualsiasi `advisorModel` configurato viene ignorato. Il flag `--advisor` è accettato ma non ha alcun effetto. Vedi [Environment variables](/docs/it/env-vars).

<h2 id="compare-with-related-features">
  Confronta con funzionalità correlate
</h2>

L'advisor è uno dei diversi modi per combinare i punti di forza dei modelli. Scegli in base a quando desideri che un secondo modello sia coinvolto.

| Approccio                                                        | Quando viene eseguito il modello più potente                                                                                                          | Come inizia                                 |
| ---------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------- |
| Strumento advisor                                                | Nei punti decisionali a metà attività                                                                                                                 | Claude lo chiama quando ha bisogno di guida |
| [`opusplan`](/docs/it/model-config#opusplan-model-setting)            | Durante la modalità piano quando [consentito da `availableModels`](/docs/it/model-config#restrict-model-selection), quindi passa a Sonnet per l'esecuzione | Entri in modalità piano                     |
| [Subagents](/docs/it/sub-agents#choose-a-model) con `model` impostato | Per l'intera sottoattività delegata                                                                                                                   | Claude delega, o invochi il subagent        |
| [`/model`](/docs/it/model-config#setting-your-model)                  | Per tutti i turni successivi                                                                                                                          | Cambi modello                               |

<h2 id="see-also">
  Vedi anche
</h2>

* [Model configuration](/docs/it/model-config): cambia modelli, imposta livelli di sforzo e usa `opusplan`
* [Manage costs effectively](/docs/it/costs): traccia l'utilizzo dei token tra i modelli
* [Advisor tool in the Claude API](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool): comprendi lo strumento server sottostante, o usalo direttamente dall'API Messages
* [The advisor strategy](https://claude.com/blog/the-advisor-strategy): perché l'accoppiamento di un modello principale veloce con un advisor più potente funziona
