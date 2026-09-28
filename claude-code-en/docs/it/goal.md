> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Mantenere Claude al lavoro verso un obiettivo

> Imposta una condizione di completamento con /goal e Claude continua a lavorare finché non è soddisfatta, un modello la giudica impossibile, o un errore che Lei deve correggere cancella l'obiettivo.

Il comando `/goal` imposta una condizione di completamento e Claude continua a lavorare verso di essa senza che Lei debba richiedere ogni passaggio. Dopo ogni turno, un piccolo modello veloce verifica se la condizione è soddisfatta. Se il modello giudica che non sia ancora soddisfatta, Claude inizia un altro turno invece di restituire il controllo a Lei. L'obiettivo si cancella automaticamente una volta che la condizione è soddisfatta, se il modello giudica la condizione impossibile da soddisfare, o se un turno fallisce su [un errore che Lei deve correggere](#errors-you-have-to-fix-clear-the-goal).

Utilizzi un obiettivo per lavori sostanziali con uno stato finale verificabile:

* Migrazione di un modulo a una nuova API finché ogni sito di chiamata compila e i test passano
* Implementazione di un documento di progettazione finché tutti i criteri di accettazione sono soddisfatti
* Divisione di un file di grandi dimensioni in moduli focalizzati finché ciascuno è entro un budget di dimensioni
* Elaborazione di un backlog di problemi etichettati finché la coda non è vuota

<h2 id="compare-ways-to-keep-a-session-running">
  Confrontare i modi per mantenere una sessione in esecuzione
</h2>

Tre approcci mantengono la sessione corrente in esecuzione tra i prompt. Scegli in base a cosa dovrebbe avviare il turno successivo:

| Approccio                                                           | Il turno successivo inizia quando                                                                                                                                                                              | Si ferma quando                                                                                                                                                                                                                     |
| :------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/goal`                                                             | Il turno precedente finisce, oppure, in una sessione interattiva, un [controllo di inattività](#background-work-defers-evaluation) o un [tentativo automatico](#other-errors-retry-or-pause-the-goal) è dovuto | Un modello conferma che la condizione è soddisfatta o la giudica impossibile, oppure un turno fallisce su [un errore che dovete correggere](#errors-you-have-to-fix-clear-the-goal), oppure eseguite [`/goal clear`](#clear-a-goal) |
| [`/loop`](/docs/it/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop) | Un intervallo di tempo trascorre                                                                                                                                                                               | Lo interrompi, o Claude decide che il lavoro è fatto                                                                                                                                                                                |
| [Stop hook](/docs/it/hooks-guide#prompt-based-hooks)                     | Il turno precedente finisce                                                                                                                                                                                    | Il tuo script o prompt decide                                                                                                                                                                                                       |

`/goal` e uno Stop hook si attivano entrambi dopo ogni turno. `/goal` è un collegamento con ambito di sessione: digiti una condizione ed è attiva solo per la sessione corrente. Uno Stop hook risiede nel tuo file di impostazioni, si applica a ogni sessione nel suo ambito e può eseguire uno script per controlli deterministici o un prompt per quelli valutati dal modello.

[Auto mode](/docs/it/auto-mode-config) da solo approva le chiamate agli strumenti all'interno di un singolo turno ma non ne avvia uno nuovo. Claude si ferma quando giudica il lavoro completato. `/goal` aggiunge un valutatore separato che verifica la tua condizione dopo ogni turno, quindi il completamento è deciso da un modello nuovo piuttosto che da quello che sta facendo il lavoro. I due sono complementari: auto mode rimuove i prompt per strumento, e `/goal` rimuove i prompt per turno.

<Tip>
  Gli approcci sopra mantengono la sessione corrente in esecuzione. Puoi anche pianificare lavori che vengono eseguiti indipendentemente da qualsiasi sessione aperta, come test notturni o triage mattutino. Vedi [opzioni di pianificazione](/docs/it/scheduled-tasks#compare-scheduling-options) per routine cloud e attività pianificate desktop.
</Tip>

<h2 id="use-/goal">
  Usa `/goal`
</h2>

Un obiettivo può essere attivo per sessione. Lo stesso comando lo imposta, lo verifica e lo cancella a seconda dell'argomento.

<h3 id="set-a-goal">
  Imposta un obiettivo
</h3>

Esegui `/goal` seguito dalla condizione che desideri soddisfatta. Se un obiettivo è già attivo, il nuovo lo sostituisce.

```text theme={null}
/goal all tests in test/auth pass and the lint step is clean
```

L'impostazione di un obiettivo avvia immediatamente un turno, con la condizione stessa come direttiva. Non è necessario inviare un prompt separato. Mentre l'obiettivo è attivo, un indicatore `◎ /goal active` mostra da quanto tempo l'obiettivo è in esecuzione.

Un obiettivo non cambia la modalità di autorizzazione. Per consentire ai turni di obiettivo di funzionare senza supervisione, esegui `/goal` in [modalità automatica](/docs/it/auto-mode-config). In [modalità manuale](/docs/it/permission-modes), Claude chiede comunque prima delle chiamate di strumenti che le Vostre impostazioni non consentono già, come il comando di test sopra.

Mentre l'obiettivo è attivo, la trascrizione mostra ogni verdetto che il valutatore restituisce, e potete premere Ctrl+O per vedere il motivo dietro di esso. La vista dello stato mostra anche il motivo più recente, così potete vedere verso cosa Claude sta lavorando successivamente.

<h3 id="write-an-effective-condition">
  Scrivi una condizione efficace
</h3>

Il [valutatore](#how-evaluation-works) giudica la Sua condizione rispetto a ciò che Claude ha esposto nella conversazione. Non esegue comandi o legge file indipendentemente, quindi scrivi la condizione come qualcosa che l'output stesso di Claude può dimostrare. "Tutti i test in `test/auth` passano" funziona perché Claude esegue i test e il risultato finisce nella trascrizione affinché il valutatore lo legga.

Una condizione che regge attraverso molti turni di solito ha:

* **Uno stato finale misurabile**: un risultato di test, un codice di uscita della build, un conteggio di file, una coda vuota
* **Un controllo dichiarato**: come Claude dovrebbe provarlo, come "`npm test` esce 0" o "`git status` è pulito"
* **Vincoli che contano**: qualsiasi cosa che non deve cambiare nel percorso, come "nessun altro file di test viene modificato"

La condizione può essere fino a 4.000 caratteri.

Per limitare quanto a lungo un obiettivo viene eseguito, includi una clausola di turno o tempo nella condizione, come `or stop after 20 turns`. Claude segnala i progressi rispetto a quella clausola ogni turno e il valutatore la giudica dalla conversazione.

<h3 id="check-status">
  Controlla lo stato
</h3>

Esegui `/goal` senza argomenti per vedere lo stato corrente.

```text theme={null}
/goal
```

Se un obiettivo è attivo, lo stato mostra:

* La condizione
* Da quanto tempo è in esecuzione
* Quanti turni sono stati valutati
* La spesa di token corrente
* Il motivo più recente del valutatore

Il conteggio dei turni e il motivo più recente appaiono dopo che la prima valutazione è stata eseguita.

Se nessun obiettivo è attivo ma uno è stato raggiunto in precedenza nella sessione, lo stato mostra la condizione raggiunta insieme alla sua durata, conteggio dei turni e spesa di token.

<h3 id="clear-a-goal">
  Cancella un obiettivo
</h3>

Esegui `/goal clear` per rimuovere un obiettivo attivo prima che si risolva.

```text theme={null}
/goal clear
```

Claude stampa `Goal cleared:` seguito dalla condizione per confermare, o `No goal set` se nulla era attivo.

`stop`, `off`, `reset`, `none` e `cancel` sono accettati come alias per `clear`. L'esecuzione di `/clear` per avviare una nuova conversazione rimuove anche qualsiasi obiettivo attivo.

<h3 id="resume-with-an-active-goal">
  Riprendi con un obiettivo attivo
</h3>

Quando riprendi una sessione, Claude Code ripristina un obiettivo che era ancora attivo quando la sessione è terminata. Claude Code lo ripristina su ogni percorso di ripresa: `--continue`, `--resume` con un ID di sessione, un nome, o un [percorso di file di trascrizione](/docs/it/sessions#resume-a-session), e il [selettore di sessione](/docs/it/sessions#use-the-session-picker). Prima della v2.1.239, Claude Code ripristinava l'obiettivo su ogni percorso tranne il selettore `claude --resume`.

Claude Code trasferisce la condizione ma ripristina il conteggio dei turni, il timer e la linea di base della spesa di token. Non ripristina un obiettivo che era già raggiunto o cancellato.

<h3 id="run-non-interactively">
  Esegui in modo non interattivo
</h3>

`/goal` funziona in [modalità non interattiva](/docs/it/headless), nell'[app desktop](/docs/it/desktop) e tramite [Remote Control](/docs/it/remote-control). L'impostazione di un obiettivo con `-p` esegue il ciclo fino al completamento in una singola invocazione:

```bash theme={null}
claude -p "/goal CHANGELOG.md has an entry for every PR merged this week"
```

Con l'output di testo predefinito, nulla viene stampato finché l'esecuzione non termina, quindi un obiettivo che viene eseguito per molti turni può sembrare bloccato. Aggiungete `--output-format stream-json --verbose` per emettere ogni messaggio mentre il ciclo viene eseguito.

Interrompete il processo con Ctrl+C per fermare un obiettivo non interattivo prima che si risolva.

<h2 id="how-evaluation-works">
  Come funziona la valutazione
</h2>

`/goal` è un wrapper attorno a uno [Stop hook basato su prompt](/docs/it/hooks#prompt-based-hooks) con ambito di sessione. Ogni volta che Claude finisce un turno, Claude Code invia la condizione e la conversazione finora al tuo [piccolo modello veloce](/docs/it/model-config) configurato, che per impostazione predefinita è Haiku sull'API Claude; su un provider di terze parti, controlla la tua [pagina del provider](/docs/it/third-party-integrations) per il valore predefinito della piattaforma. Il modello restituisce uno di tre verdetti, ciascuno con una breve motivazione:

* **Non ancora soddisfatto**: Claude continua a lavorare e prende la motivazione come guida per il turno successivo.
* **Soddisfatto**: Claude Code cancella l'obiettivo e registra una voce raggiunta nella trascrizione.
* **Impossibile**: il valutatore ha giudicato che la condizione non può mai essere soddisfatta. Claude Code cancella l'obiettivo e registra una voce non riuscita nella trascrizione insieme alla motivazione. Non è necessario cancellarla tu stesso.

Se Claude continua a rispondere al valutatore senza fare progressi (nessun utilizzo di strumenti per diversi turni di seguito), Claude Code interrompe il ciclo, stampa un avviso e ti restituisce il controllo con l'obiettivo ancora impostato. La valutazione riprende dopo il tuo prossimo prompt. La [guida ai hooks](/docs/it/hooks-guide#stop-hook-hits-the-block-cap) spiega il meccanismo sottostante.

<h3 id="when-a-turn-fails">
  Quando un turno fallisce
</h3>

Quando un turno fallisce, Claude Code cancella l'obiettivo se l'errore è uno che devi correggere. Dopo qualsiasi altro errore l'obiettivo rimane impostato.

<h4 id="errors-you-have-to-fix-clear-the-goal">
  Gli errori che devi correggere cancellano l'obiettivo
</h4>

Se un turno fallisce con un errore che non si cancellerà finché non lo correggi, Claude Code cancella l'obiettivo e stampa un avviso che nomina la causa. L'avviso inizia con `Goal cleared after an unrecoverable error` e termina con `Run /goal again to continue`. Correggi la causa, quindi [imposta di nuovo l'obiettivo](#set-a-goal) con `/goal <condition>`. Quattro tipi di errore cancellano l'obiettivo:

* Un errore di autenticazione, quando Claude Code gestisce le proprie credenziali. Quando un host le gestisce per te, come l'app desktop, l'estensione VS Code o una [sessione cloud](/docs/it/claude-code-on-the-web), Claude Code lascia l'obiettivo attivo perché l'host ripristina l'accesso da solo.
* Un saldo di credito esaurito
* Un overflow di contesto che [auto-compact](/docs/it/model-config#set-the-auto-compact-window) non poteva cancellare
* Un modello che non è disponibile

<h4 id="other-errors-retry-or-pause-the-goal">
  Altri errori riprovano o mettono in pausa l'obiettivo
</h4>

Dopo qualsiasi altro errore l'obiettivo rimane impostato. In una sessione interattiva su Claude Code v2.1.269 o successivo, Claude Code stampa anche una riga che nomina la causa e riprova da solo o ti aspetta:

* **Riprova**: dopo un errore che tende a risolversi da solo, come un server sovraccarico o una connessione interrotta, un avviso che inizia con `Goal still active` mostra l'attesa prima del prossimo tentativo. Dopo tre tentativi automatici, l'obiettivo viene messo in pausa.
* **Pausa**: dopo un errore che un nuovo tentativo ripeterebbe solo, come un limite di velocità API, un [limite di utilizzo](/docs/it/errors#youve-hit-your-session-limit) di claude.ai o un hook che ha terminato il turno, un avviso che inizia con `Goal paused` nomina la causa. Se la sessione è [in attesa di continuare automaticamente quando un limite di utilizzo si ripristina](/docs/it/interactive-mode#wait-for-a-usage-limit-to-reset), Claude riprende il lavoro verso l'obiettivo allora.

Invia un messaggio in qualsiasi momento per iniziare il turno successivo immediatamente. Per disattivare i tentativi automatici, imposta [`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](/docs/it/env-vars) su `0`, che disattiva anche i [check-in](#background-work-defers-evaluation).

<h3 id="background-work-defers-evaluation">
  Il lavoro in background rinvia la valutazione
</h3>

Se un subagent o un comando shell in background è ancora in esecuzione quando un turno termina, Claude Code salta la valutazione per quel turno. Valuta alla fine del turno successivo che termina senza lavoro in background in esecuzione. Quando il lavoro in background termina, Claude Code consegna il risultato a Claude come un nuovo turno, quindi non devi fare un prompt.

Una volta che il lavoro in background ha mantenuto l'obiettivo in attesa per 30 minuti, è dovuto un check-in. Nel check-in, Claude Code elenca le attività in esecuzione e chiede a Claude di leggere il loro output, continuare ad aspettare se stanno progredendo e correggere o interrompere quelli bloccati. Dopo il primo check-in, Claude Code attende il doppio del tempo prima di ogni check-in successivo, fino a quattro volte l'intervallo iniziale: con il valore predefinito, 1 ora dopo il primo check-in, quindi ogni 2 ore. Claude Code consegna un check-in dovuto, il primo incluso, in uno di due modi:

* **Quando un turno termina**: Claude Code consegna il check-in alla fine del turno successivo che termina con il lavoro ancora in esecuzione. In una sessione non interattiva, come una avviata con `-p`, questo è l'unico modo in cui Claude Code consegna i check-in.
* **Mentre la sessione è inattiva**: in una sessione interattiva, Claude Code avvia anche un turno da solo per consegnare il check-in invece di aspettare il tuo prossimo prompt. Se il lavoro in background si è fermato senza segnalare un risultato, Claude Code chiede a Claude di continuare verso l'obiettivo. Claude Code avvia al massimo tre check-in inattivi per obiettivo tra i tuoi prompt. Nel terzo check-in inattivo, Claude Code dice che i check-in inattivi sono sospesi fino a quando non invii un altro prompt. Prima della v2.1.246, i check-in inattivi erano illimitati. I check-in inattivi richiedono Claude Code v2.1.236 o successivo.

Prima della v2.1.239, solo i check-in inattivi si ritiravano in questo modo; un check-in consegnato alla fine di un turno ricorreva al primo intervallo.

Per modificare il primo intervallo, imposta [`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](/docs/it/env-vars). Claude Code utilizza il tuo valore al posto dell'intervallo di 30 minuti e scala gli intervalli successivi con esso. Impostalo su `0` per disattivare i check-in e i [tentativi automatici](#other-errors-retry-or-pause-the-goal).

I check-in richiedono Claude Code v2.1.234 o successivo.

<h3 id="evaluation-model-and-cost">
  Modello di valutazione e costo
</h3>

Per valutare su un modello diverso, imposta [`ANTHROPIC_DEFAULT_HAIKU_MODEL`](/docs/it/model-config#environment-variables).

<Warning>
  Claude Code legge `ANTHROPIC_DEFAULT_HAIKU_MODEL` ovunque utilizzi il piccolo modello veloce, non solo per la valutazione `/goal`. Quando lo imposti, Claude Code risolve anche l'[alias `haiku`](/docs/it/model-config#model-aliases) a quel modello ed esegue la [funzionalità in background](/docs/it/costs#background-token-usage), come il riepilogo della conversazione, su di esso.
</Warning>

Il valutatore viene eseguito su qualsiasi provider la tua sessione sia configurata. Non chiama strumenti, quindi può solo giudicare ciò che Claude ha già esposto nella conversazione.

<Note>
  I token di valutazione vengono fatturati sul piccolo modello veloce configurato per il tuo provider e sono in genere trascurabili rispetto alla spesa del turno principale.
</Note>

<h2 id="requirements">
  Requisiti
</h2>

Claude Code rende `/goal` disponibile secondo la stessa [regola di fiducia dell'area di lavoro degli hook nei file di impostazioni](/docs/it/permissions#what-runs-before-you-trust-a-folder), perché l'evaluator fa parte del sistema di hook. `/goal` è anche non disponibile quando [`disableAllHooks`](/docs/it/hooks#disable-or-remove-hooks) è `true` dopo l'applicazione della precedenza delle impostazioni, o quando [`allowManagedHooksOnly`](/docs/it/settings-reference#allowmanagedhooksonly) è impostato nelle impostazioni gestite. In ogni caso, il comando ti dice perché invece di non fare nulla silenziosamente.

<h2 id="see-also">
  Vedi anche
</h2>

* [Esegui un prompt ripetutamente con `/loop`](/docs/it/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop): riesegui a intervalli di tempo invece che finché una condizione non regge
* [Hook basati su prompt](/docs/it/hooks-guide#prompt-based-hooks): scrivi il tuo Stop hook quando hai bisogno di logica di valutazione personalizzata
* [Auto mode](/docs/it/auto-mode-config): approva le chiamate agli strumenti automaticamente in modo che ogni turno di obiettivo venga eseguito senza supervisione
* [Confronto della pianificazione](/docs/it/scheduled-tasks#compare-scheduling-options): esegui il lavoro secondo una pianificazione indipendente da qualsiasi sessione aperta
