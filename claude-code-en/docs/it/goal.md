> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Mantenere Claude al lavoro verso un obiettivo

> Imposta una condizione di completamento con /goal e Claude continua a lavorare tra i turni finché la condizione non è soddisfatta.

<Note>
  `/goal` richiede Claude Code v2.1.139 o successivo.
</Note>

Il comando `/goal` imposta una condizione di completamento e Claude continua a lavorare verso di essa senza che Lei debba richiedere ogni passaggio. Dopo ogni turno, un piccolo modello veloce verifica se la condizione è soddisfatta. Se non lo è, Claude inizia un altro turno invece di restituire il controllo a Lei. L'obiettivo si cancella automaticamente una volta che la condizione è soddisfatta.

Utilizzi un obiettivo per lavori sostanziali con uno stato finale verificabile:

* Migrazione di un modulo a una nuova API finché ogni sito di chiamata compila e i test passano
* Implementazione di un documento di progettazione finché tutti i criteri di accettazione sono soddisfatti
* Divisione di un file di grandi dimensioni in moduli focalizzati finché ciascuno è entro un budget di dimensioni
* Elaborazione di un backlog di problemi etichettati finché la coda non è vuota

<h2 id="compare-ways-to-keep-a-session-running">
  Confrontare i modi per mantenere una sessione in esecuzione
</h2>

Tre approcci mantengono la sessione corrente in esecuzione tra i prompt. Scegli in base a cosa dovrebbe avviare il turno successivo:

| Approccio                                                           | Il turno successivo inizia quando | Si ferma quando                                      |
| :------------------------------------------------------------------ | :-------------------------------- | :--------------------------------------------------- |
| `/goal`                                                             | Il turno precedente finisce       | Un modello conferma che la condizione è soddisfatta  |
| [`/loop`](/docs/it/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop) | Un intervallo di tempo trascorre  | Lo interrompi, o Claude decide che il lavoro è fatto |
| [Stop hook](/docs/it/hooks-guide#prompt-based-hooks)                     | Il turno precedente finisce       | Il tuo script o prompt decide                        |

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

Un obiettivo non cambia le autorizzazioni. Nella modalità di autorizzazione predefinita, Claude chiede comunque prima delle chiamate di strumenti che le Vostre impostazioni non consentono già, come il comando di test sopra. Per consentire ai turni di obiettivo di funzionare senza supervisione, abbinate `/goal` con [modalità automatica](/docs/it/auto-mode-config).

Dopo ogni turno, il valutatore restituisce una breve spiegazione del motivo per cui la condizione è o non è soddisfatta. Il motivo più recente appare nella vista dello stato e nella trascrizione in modo che Lei possa vedere verso cosa Claude sta lavorando successivamente.

<Note>
  Un obiettivo continua a funzionare finché la condizione non è soddisfatta o finché non esegui `/goal clear`. Esegui `/goal` senza argomenti per vedere i turni e i token spesi finora.
</Note>

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

Esegui `/goal clear` per rimuovere un obiettivo attivo prima che la sua condizione sia soddisfatta.

```text theme={null}
/goal clear
```

Claude stampa `Goal cleared:` seguito dalla condizione per confermare, o `No goal set` se nulla era attivo.

`stop`, `off`, `reset`, `none` e `cancel` sono accettati come alias per `clear`. L'esecuzione di `/clear` per avviare una nuova conversazione rimuove anche qualsiasi obiettivo attivo.

<h3 id="resume-with-an-active-goal">
  Riprendi con un obiettivo attivo
</h3>

Un obiettivo che era ancora attivo quando una sessione è terminata viene ripristinato quando riprendi quella sessione con `--resume` o `--continue`. La condizione viene trasferita, ma il conteggio dei turni, il timer e la linea di base della spesa di token si azzerano al ripristino. Un obiettivo che era già raggiunto o cancellato non viene ripristinato.

<h3 id="run-non-interactively">
  Esegui in modo non interattivo
</h3>

`/goal` funziona in [modalità non interattiva](/docs/it/headless), nell'[app desktop](/docs/it/desktop) e tramite [Remote Control](/docs/it/remote-control). L'impostazione di un obiettivo con `-p` esegue il ciclo fino al completamento in una singola invocazione:

```bash theme={null}
claude -p "/goal CHANGELOG.md has an entry for every PR merged this week"
```

Con l'output di testo predefinito, nulla viene stampato finché la condizione non è soddisfatta, quindi un obiettivo che viene eseguito per molti turni può sembrare bloccato. Aggiungete `--output-format stream-json --verbose` per emettere ogni messaggio mentre il ciclo viene eseguito.

Interrompi il processo con Ctrl+C per fermare un obiettivo non interattivo prima che la condizione sia soddisfatta.

<h2 id="how-evaluation-works">
  Come funziona la valutazione
</h2>

`/goal` è un wrapper attorno a uno [Stop hook basato su prompt](/docs/it/hooks#prompt-based-hooks) con ambito di sessione. Ogni volta che Claude finisce un turno, la condizione e la conversazione finora vengono inviate al tuo [piccolo modello veloce](/docs/it/model-config) configurato, che per impostazione predefinita è Haiku. Il modello restituisce una decisione sì o no e una breve spiegazione. Un "no" dice a Claude di continuare a lavorare e include il motivo come guida per il turno successivo. Un "sì" cancella l'obiettivo e registra una voce raggiunta nella trascrizione.

Il valutatore viene eseguito su qualsiasi provider la tua sessione sia configurata. Non chiama strumenti, quindi può solo giudicare ciò che Claude ha già esposto nella conversazione.

<Note>
  I token di valutazione vengono fatturati sul piccolo modello veloce configurato per il tuo provider e sono in genere trascurabili rispetto alla spesa del turno principale.
</Note>

<h2 id="requirements">
  Requisiti
</h2>

`/goal` viene eseguito solo negli spazi di lavoro in cui hai accettato la finestra di dialogo di fiducia, perché il valutatore fa parte del sistema di hook. `/goal` è anche non disponibile quando [`disableAllHooks`](/docs/it/hooks#disable-or-remove-hooks) è impostato a qualsiasi livello di impostazioni o quando [`allowManagedHooksOnly`](/docs/it/settings#hook-configuration) è impostato nelle impostazioni gestite. In ogni caso, il comando ti dice perché invece di non fare nulla silenziosamente.

<h2 id="see-also">
  Vedi anche
</h2>

* [Esegui un prompt ripetutamente con `/loop`](/docs/it/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop): riesegui a intervalli di tempo invece che finché una condizione non regge
* [Hook basati su prompt](/docs/it/hooks-guide#prompt-based-hooks): scrivi il tuo Stop hook quando hai bisogno di logica di valutazione personalizzata
* [Auto mode](/docs/it/auto-mode-config): approva le chiamate agli strumenti automaticamente in modo che ogni turno di obiettivo venga eseguito senza supervisione
* [Confronto della pianificazione](/docs/it/scheduled-tasks#compare-scheduling-options): esegui il lavoro secondo una pianificazione indipendente da qualsiasi sessione aperta
