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

Se uno di questi imposta un modello advisor, l'advisor è abilitato per le sessioni il cui modello principale lo [supporta](#choose-an-advisor-model). Per smettere di usarlo, vedi [Disattiva l'advisor](#turn-the-advisor-off).

<Note>
  Per utilizzare Fable 5 come advisor, hai bisogno di Claude Code v2.1.170 o successivo e [accesso a Fable 5](/docs/it/model-config#work-with-fable-5) per la tua organizzazione.
</Note>

<h3 id="use-the-/advisor-command">
  Usa il comando `/advisor`
</h3>

Esegui `/advisor` senza argomenti per aprire un selettore che elenca i modelli advisor disponibili, o passa il modello direttamente:

```
/advisor opus
```

La tua selezione viene salvata in `advisorModel` nelle impostazioni utente e persiste tra le sessioni. Se l'allowlist [`availableModels`](/docs/it/model-config#restrict-model-selection) della tua organizzazione esclude il modello advisor salvato, l'advisor non viene invocato finché non scegli un modello consentito con `/advisor`. Se il tuo modello principale attuale non supporta l'advisor, la selezione viene comunque salvata e si attiva quando passi a un modello principale [compatibile](#choose-an-advisor-model) con [`/model`](/docs/it/model-config#setting-your-model).

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

Il flag ha la precedenza sull'impostazione `advisorModel` per quella sessione. Esce con un errore se il modello principale della sessione non supporta l'advisor, o se il modello advisor richiesto è escluso dall'allowlist [`availableModels`](/docs/it/model-config#restrict-model-selection) della tua organizzazione.

<h2 id="choose-an-advisor-model">
  Scegli un modello advisor
</h2>

L'advisor deve essere almeno altrettanto capace del modello principale. Gli advisor accettati per ogni modello principale sono:

| Modello principale    | Advisor accettati         | Note                                                                                                                                                                             |
| --------------------- | ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Haiku 4.5             | Fable, Opus, Sonnet       | Haiku può chiamare l'advisor ma non può agire come uno                                                                                                                           |
| Sonnet 4.6            | Fable, Opus, Sonnet       |                                                                                                                                                                                  |
| Sonnet 5              | Fable, Opus, Sonnet 5     | Un advisor Sonnet 4.6 viene rifiutato                                                                                                                                            |
| Opus 4.6              | Fable, Opus, Sonnet 5     | Sonnet 5 e Opus 4.6 sono classificati come ugualmente capaci, quindi un Opus 4.6 principale accetta un advisor Sonnet 5                                                          |
| Opus 4.7 o successivo | Fable, Opus 4.7, Opus 4.8 | Opus 4.7 e Opus 4.8 sono classificati come ugualmente capaci, quindi uno accetta l'altro come advisor. Un Opus 4.7 principale con un advisor Opus 4.6 o Sonnet 5 viene rifiutato |
| Fable 5 (v2.1.170+)   | Fable                     | Un advisor Opus o Sonnet viene rifiutato                                                                                                                                         |

Fable 5 richiede Claude Code v2.1.170 o successivo e accesso a Fable 5, sia che agisca come modello principale che come advisor.

Imposta l'advisor come `opus`, `sonnet`, o `fable`. Questi alias si risolvono nella versione più recente di ogni modello. Puoi anche passare un ID modello completo come `claude-opus-4-8`.

I subagent ereditano l'advisor configurato e applicano lo stesso controllo di accoppiamento rispetto al loro modello.

Claude Code convalida l'accoppiamento prima di inviare una richiesta:

* Se l'advisor è meno capace del modello principale, l'advisor non è allegato alle richieste del modello principale. L'output del comando `/advisor` e una notifica mostrano questo. I subagent il cui modello soddisfa l'accoppiamento possono comunque utilizzare l'advisor.
* Se il modello principale o l'advisor è un modello che Claude Code non riconosce, l'advisor non è allegato.

<h3 id="common-model-pairings">
  Accoppiamenti di modelli comuni
</h3>

Qualsiasi accoppiamento accettato funziona. Queste combinazioni bilanciano il costo rispetto alla capacità in modi diversi:

| Accoppiamento                      | Quando utilizzare                                                                                                                                                                              |
| ---------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Sonnet principale + advisor Opus   | Sonnet gestisce il lavoro di routine e escalation della pianificazione, fallimenti ambigui e controlli di completamento a Opus                                                                 |
| Sonnet principale + advisor Fable  | Guida Fable 5 nei punti decisionali senza eseguire Fable 5 in tutto. Richiede v2.1.170 o successivo e accesso a Fable 5                                                                        |
| Haiku principale + advisor Opus    | Modello principale a costo più basso con pianificazione forte. Aspettati un costo più alto di Haiku da solo ma inferiore al passaggio del modello principale a Sonnet o Opus                   |
| Opus principale + advisor Opus     | Un secondo Opus esamina il primo. Utile per attività ad alto rischio dove un controllo indipendente è più importante del costo                                                                 |
| Fable principale + advisor Fable   | Accoppiamento con capacità più alta quando Fable 5 è disponibile (v2.1.170+). Fable è un livello superiore a Opus e Sonnet, quindi è l'unico advisor accettato per un modello principale Fable |
| Sonnet principale + advisor Sonnet | Un secondo parere a costo inferiore per catturare errori di routine                                                                                                                            |

<h2 id="when-claude-consults-the-advisor">
  Quando Claude consulta l'advisor
</h2>

Claude decide quando chiamare l'advisor. Tende a consultare prima di impegnarsi in un approccio, quando un errore continua a ripetersi, e prima di dichiarare un'attività completata, ma i tempi sono guidati dal modello piuttosto che basati su regole.

Puoi chiedere una consultazione nel tuo prompt nello stesso modo in cui richiederesti qualsiasi strumento, ad esempio `consult the advisor before you continue`. Non c'è un'impostazione per limitare o forzare le chiamate dell'advisor; se desideri che Claude consulti più o meno spesso durante un'attività, dillo nelle tue istruzioni.

<h2 id="what-you-see-during-a-session">
  Cosa vedi durante una sessione
</h2>

Quando Claude chiama l'advisor, la trascrizione mostra una riga `Advising` con il nome del modello advisor mentre la chiamata è in corso. Quando il risultato ritorna, la riga conferma che l'advisor ha esaminato la conversazione. Premi `Ctrl+O` per espanderla e leggere la guida completa dell'advisor.

Claude generalmente segue la guida dell'advisor, ma si adatta quando le sue stesse prove contraddicono un'affermazione specifica: se un passaggio consigliato fallisce quando provato, o il contenuto del file contraddice il consiglio, Claude evidenzia il conflitto piuttosto che seguire la guida incondizionatamente.

L'advisor riceve sempre la conversazione completa, e Claude controlla i tempi. Per un maggiore controllo o una configurazione diversa, vedi [come l'advisor si confronta con subagents e opusplan](#compare-with-related-features).

<h2 id="cost">
  Costo
</h2>

Ogni chiamata dell'advisor invia la conversazione al modello advisor, quindi consuma token alle tariffe del modello advisor in aggiunta all'utilizzo del tuo modello principale. Con la fatturazione API, i token dell'advisor vengono addebitati alle tariffe di input e output del modello advisor. Sui piani di abbonamento, l'utilizzo dell'advisor conta verso i limiti di utilizzo del tuo piano.

Claude chiama l'advisor nei punti decisionali piuttosto che ad ogni turno, quindi l'accoppiamento di un modello principale più veloce con un advisor più potente in genere costa meno che eseguire il modello più potente in tutto. L'utilizzo dell'advisor conta verso i totali della sessione mostrati da [`/usage`](/docs/it/costs#track-your-costs).

Per come i token dell'advisor vengono segnalati nelle risposte API, vedi [Usage and billing](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool#usage-and-billing) nella documentazione dell'API Claude.

<h2 id="impact-on-prompt-caching">
  Impatto sulla memorizzazione nella cache dei prompt
</h2>

L'abilitazione o la disabilitazione dell'advisor a metà sessione non invalida la [cache dei prompt](/docs/it/prompt-caching) del tuo modello principale. A differenza di [cambiare modello o livello di sforzo](/docs/it/prompt-caching#actions-that-invalidate-the-cache), l'attivazione/disattivazione di `/advisor` mantiene il prefisso memorizzato nella cache intatto, e la guida restituita dall'advisor viene memorizzata nella cache come parte della trascrizione nei turni successivi.

La lettura della conversazione da parte del modello advisor stesso non viene memorizzata nella cache. Ogni chiamata dell'advisor elabora la trascrizione completa da capo, senza riutilizzo tra le chiamate.

<h2 id="requirements">
  Requisiti
</h2>

Lo strumento advisor richiede tutti i seguenti:

* **Solo API Anthropic**: l'advisor è uno strumento eseguito dal server. Non è disponibile su Amazon Bedrock, Claude Platform su AWS, Google Cloud's Agent Platform, o Microsoft Foundry. Attraverso un [gateway LLM](/docs/it/llm-gateway) configurato con `ANTHROPIC_BASE_URL`, la disponibilità dipende dal fatto che il gateway inoltri la richiesta intatta all'API Anthropic.
* **Modello principale supportato**: Opus 4.6 o successivo, Sonnet 4.6 o successivo, o Haiku 4.5. Fable 5 si qualifica anche su Claude Code v2.1.170 o successivo.

<h2 id="turn-the-advisor-off">
  Disattiva l'advisor
</h2>

Per smettere di usare l'advisor e cancellare il tuo `advisorModel` salvato, esegui `/advisor off` o scegli **No advisor** nel selettore `/advisor`:

```
/advisor off
```

Per disabilitare completamente lo strumento advisor, imposta `CLAUDE_CODE_DISABLE_ADVISOR_TOOL=1`. Il comando `/advisor` diventa non disponibile e qualsiasi `advisorModel` configurato viene ignorato. Il flag `--advisor` è accettato ma non ha alcun effetto; gli script esistenti che lo passano continuano a funzionare senza errori. Vedi [Environment variables](/docs/it/env-vars).

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
