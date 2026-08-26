> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Eseguire prompt in base a una pianificazione

> Utilizzare /loop e gli strumenti di pianificazione cron per eseguire prompt ripetutamente, eseguire il polling dello stato o impostare promemoria una tantum all'interno di una sessione Claude Code.

Le attività pianificate consentono a Claude di rieseguire automaticamente un prompt a intervalli regolari. Utilizzarle per eseguire il polling di una distribuzione, monitorare una PR, controllare una compilazione a lunga esecuzione o ricordarsi di fare qualcosa più tardi nella sessione. Per reagire agli eventi man mano che si verificano invece di eseguire il polling, vedere [Channels](/docs/it/channels): il vostro CI può inviare il fallimento direttamente nella sessione. Per mantenere la sessione in funzione turno dopo turno fino al raggiungimento di una condizione piuttosto che a intervalli, vedere [`/goal`](/docs/it/goal).

Le attività hanno ambito di sessione: vivono nella conversazione corrente e si interrompono quando si avvia una nuova. La ripresa con `--resume` o `--continue` ripristina qualsiasi attività che non sia [scaduta](#seven-day-expiry): un'attività ricorrente creata negli ultimi 7 giorni, oppure una singola la cui ora pianificata non è ancora passata. Per la pianificazione che sopravvive indipendentemente da qualsiasi sessione, utilizzare [Routines](/docs/it/routines) per creare una routine su infrastruttura gestita da Anthropic, configurare un'[Attività pianificata Desktop](/docs/it/desktop-scheduled-tasks) o utilizzare [GitHub Actions](/docs/it/github-actions).

<h2 id="compare-scheduling-options">
  Confrontare le opzioni di pianificazione
</h2>

Claude Code offers three ways to schedule recurring or one-off work:

|                            | [Cloud](/docs/en/routines)               | [Desktop](/docs/en/desktop-scheduled-tasks) | [`/loop`](/docs/en/scheduled-tasks)      |
| :------------------------- | :---------------------------------- | :------------------------------------- | :---------------------------------- |
| Runs on                    | Cloud, Anthropic-managed by default | Your machine                           | Your machine                        |
| Requires machine on        | No                                  | Yes                                    | Yes                                 |
| Requires open session      | No                                  | No                                     | Yes                                 |
| Persistent across restarts | Yes                                 | Yes                                    | Restored on `--resume` if unexpired |
| Access to local files      | No (fresh clone)                    | Yes                                    | Yes                                 |
| MCP servers                | Connectors configured per task      | [Config files](/docs/en/mcp) and connectors | Inherits from session               |
| Permission prompts         | No (runs autonomously)              | Configurable per task                  | Inherits from session               |
| Customizable schedule      | Via `/schedule` in the CLI          | Yes                                    | Yes                                 |
| Minimum interval           | 1 hour                              | 1 minute                               | 1 minute                            |

<Tip>
  Use **cloud tasks** for work that should run reliably without your machine. Use **Desktop tasks** when you need access to local files and tools. Use **`/loop`** for quick polling during a session.
</Tip>

<h2 id="run-a-prompt-repeatedly-with-/loop">
  Eseguire un prompt ripetutamente con /loop
</h2>

Lo [skill bundled](/docs/it/commands) `/loop` è il modo più rapido per eseguire un prompt ripetutamente mentre la sessione rimane aperta. Sia l'intervallo che il prompt sono facoltativi, e quello che fornite determina il comportamento del ciclo.

| Quello che fornite       | Esempio                     | Cosa accade                                                                                                                      |
| :----------------------- | :-------------------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| Intervallo e prompt      | `/loop 5m check the deploy` | Il vostro prompt viene eseguito su un [programma fisso](#run-on-a-fixed-interval)                                                |
| Solo prompt              | `/loop check the deploy`    | Il vostro prompt viene eseguito a un [intervallo scelto da Claude](#let-claude-choose-the-interval) ad ogni iterazione           |
| Solo intervallo, o nulla | `/loop`                     | Il [prompt di manutenzione integrato](#run-the-built-in-maintenance-prompt) viene eseguito, oppure il vostro `loop.md` se esiste |

Potete anche passare uno skill come prompt, ad esempio `/loop 20m /review-pr 1234`, per rieseguire quello skill ad ogni iterazione. A partire dalla v2.1.196, un'esecuzione pianificata esegue solo gli skill che Claude è [autorizzato a invocare autonomamente](/docs/it/skills#control-who-invokes-a-skill). I seguenti raggiungono Claude come testo semplice invece di essere eseguiti:

* comandi integrati come `/permissions`, `/model`, o `/clear`
* skill contrassegnati [`disable-model-invocation: true`](/docs/it/skills#frontmatter-reference)
* skill trattenuti da Claude da un'impostazione [`skillOverrides`](/docs/it/skills#override-skill-visibility-from-settings) o da una regola di [negazione](/docs/it/skills#restrict-claude’s-skill-access) `Skill`
* [prompt MCP](/docs/it/mcp#use-mcp-prompts-as-commands) come `/mcp__github__list_prs`; gli skill che un server MCP espone vengono comunque eseguiti

<h3 id="run-on-a-fixed-interval">
  Eseguire su un intervallo fisso
</h3>

Quando fornite un intervallo, Claude lo converte in un'espressione cron, pianifica il processo e conferma la cadenza e l'ID del processo.

```text theme={null}
/loop 5m check if the deployment finished and tell me what happened
```

L'intervallo può precedere il prompt come token nudo come `30m`, oppure seguirlo come clausola come `every 2 hours`. Le unità supportate sono `s` per secondi, `m` per minuti, `h` per ore e `d` per giorni.

I secondi vengono arrotondati al minuto più vicino poiché cron ha una granularità di un minuto. Gli intervalli che non si dividono uniformemente in un passo cron pulito, come `7m` o `90m`, vengono arrotondati all'intervallo più vicino che lo fa e Claude vi dice quale ha scelto.

<h3 id="let-claude-choose-the-interval">
  Lasciare che Claude scelga l'intervallo
</h3>

Quando omettete l'intervallo, Claude ne sceglie uno dinamicamente invece di eseguire su un programma cron fisso. Dopo ogni iterazione sceglie un ritardo tra un minuto e un'ora in base a quello che ha osservato: attese brevi mentre una compilazione sta terminando o una PR è attiva, attese più lunghe quando non c'è nulla in sospeso. Il ritardo scelto e il motivo sono stampati alla fine di ogni iterazione.

L'esempio seguente controlla CI e i commenti di revisione, con Claude che attende più a lungo tra le iterazioni una volta che la PR diventa silenziosa:

```text theme={null}
/loop check whether CI passed and address any review comments
```

Quando chiedete un programma `/loop` dinamico, Claude potrebbe utilizzare direttamente lo [strumento Monitor](/docs/it/tools-reference#monitor-tool). Monitor esegue uno script in background e trasmette ogni riga di output, il che evita completamente il polling ed è spesso più efficiente in termini di token e più reattivo rispetto alla riesecuzione di un prompt a intervalli.

Un ciclo pianificato dinamicamente appare nel vostro [elenco di attività pianificate](#manage-scheduled-tasks) come qualsiasi altra attività, quindi potete elencarla o annullarla nello stesso modo. Le [regole di jitter](#jitter) non si applicano ad esso, ma la [scadenza di sette giorni](#seven-day-expiry) sì: il ciclo termina automaticamente sette giorni dopo averlo avviato.

<Note>
  Su Amazon Bedrock, Claude Platform su AWS, Google Cloud's Agent Platform e Microsoft Foundry, un prompt senza intervallo viene eseguito su un programma fisso di 10 minuti.
</Note>

<h3 id="run-the-built-in-maintenance-prompt">
  Eseguire il prompt di manutenzione integrato
</h3>

Quando omettete il prompt, Claude utilizza un prompt di manutenzione integrato invece di uno che fornite. Ad ogni iterazione lavora attraverso quanto segue, in ordine:

* continuare qualsiasi lavoro non terminato dalla conversazione
* prendersi cura della pull request del ramo corrente: commenti di revisione, esecuzioni CI non riuscite, conflitti di merge
* eseguire passaggi di pulizia come cacce ai bug o semplificazione quando non c'è nulla di altro in sospeso

Claude non avvia nuove iniziative al di fuori di tale ambito, e le azioni irreversibili come il push o l'eliminazione procedono solo quando continuano qualcosa che il trascritto ha già autorizzato.

```text theme={null}
/loop
```

Un `/loop` nudo esegue questo prompt a un [intervallo scelto dinamicamente](#let-claude-choose-the-interval). Aggiungete un intervallo, ad esempio `/loop 15m`, per eseguirlo su un programma fisso invece. Per sostituire il prompt integrato con il vostro predefinito, vedere [Personalizzare il prompt predefinito con loop.md](#customize-the-default-prompt-with-loop-md).

<Note>
  Su Amazon Bedrock, Claude Platform su AWS, Google Cloud's Agent Platform e Microsoft Foundry, `/loop` senza prompt stampa il messaggio di utilizzo invece di eseguire il prompt di manutenzione.
</Note>

<h3 id="customize-the-default-prompt-with-loop-md">
  Personalizzare il prompt predefinito con loop.md
</h3>

Un file `loop.md` sostituisce il prompt di manutenzione integrato con le vostre istruzioni. Definisce un singolo prompt predefinito per un `/loop` nudo, non un elenco di attività pianificate separate, ed è ignorato ogni volta che fornite un prompt sulla riga di comando. Per pianificare prompt aggiuntivi insieme ad esso, utilizzate `/loop <prompt>` o [chiedete direttamente a Claude](#manage-scheduled-tasks).

Claude cerca il file in due posizioni e utilizza il primo che trova.

| Percorso            | Ambito                                                                              |
| :------------------ | :---------------------------------------------------------------------------------- |
| `.claude/loop.md`   | A livello di progetto. Ha la precedenza quando entrambi i file esistono.            |
| `~/.claude/loop.md` | A livello di utente. Si applica in qualsiasi progetto che non definisce il proprio. |

Il file è Markdown semplice senza struttura richiesta. Scrivete come se steste digitando il prompt `/loop` direttamente. L'esempio seguente mantiene un ramo di rilascio sano:

```markdown title=".claude/loop.md" theme={null}
Check the `release/next` PR. If CI is red, pull the failing job log,
diagnose, and push a minimal fix. If new review comments have arrived,
address each one and resolve the thread. If everything is green and
quiet, say so in one line.
```

Le modifiche a `loop.md` hanno effetto alla successiva iterazione, quindi potete perfezionare le istruzioni mentre un ciclo è in esecuzione. Quando nessun `loop.md` esiste in nessuna posizione, il ciclo ritorna al prompt di manutenzione integrato. Mantenete il file conciso: il contenuto oltre 25.000 byte viene troncato.

<Note>
  Su Amazon Bedrock, Claude Platform su AWS, Google Cloud's Agent Platform e Microsoft Foundry, `loop.md` non viene letto e `/loop` senza prompt stampa il messaggio di utilizzo invece.
</Note>

<h3 id="stop-a-loop">
  Interrompere un ciclo
</h3>

Per interrompere un `/loop` mentre è in attesa della successiva iterazione, premete `Esc`. Questo cancella il risveglio in sospeso in modo che il ciclo non si attivi di nuovo. Le attività pianificate [chiedendo direttamente a Claude](#manage-scheduled-tasks) non sono interessate da `Esc` e rimangono in posizione fino a quando non le eliminate.

In [modalità autonoma](#let-claude-choose-the-interval), Claude può anche terminare il ciclo da solo una volta che l'attività è completa. Claude chiama lo strumento [`ScheduleWakeup`](/docs/it/tools-reference) con `stop: true`, che annulla il risveglio in sospeso immediatamente. Se un'iterazione termina senza ripianificare o interrompere, Claude Code pianifica un risveglio di fallback circa 20 minuti dopo e termina il ciclo quando quell'iterazione non ripianifica nemmeno. Prima della v2.1.202, non ripianificare era l'unico modo in cui Claude poteva terminare un ciclo da solo.

I cicli su un intervallo fisso continuano a funzionare fino a quando non li interrompete o [trascorrono sette giorni](#seven-day-expiry).

<h2 id="set-a-one-time-reminder">
  Impostare un promemoria una tantum
</h2>

Per promemoria una tantum, descrivete quello che volete in linguaggio naturale invece di utilizzare `/loop`. Claude pianifica un'attività a fuoco singolo che si elimina dopo l'esecuzione.

```text theme={null}
remind me at 3pm to push the release branch
```

```text theme={null}
in 45 minutes, check whether the integration tests passed
```

Claude fissa l'ora di attivazione a un minuto e un'ora specifici utilizzando un'espressione cron e conferma quando si attiverà.

<h2 id="manage-scheduled-tasks">
  Gestire le attività pianificate
</h2>

Chiedete a Claude in linguaggio naturale di elencare o annullare le attività, oppure fate riferimento direttamente agli strumenti sottostanti.

```text theme={null}
what scheduled tasks do I have?
```

```text theme={null}
cancel the deploy check job
```

Dietro le quinte, Claude utilizza questi strumenti:

| Strumento    | Scopo                                                                                                                                 |
| :----------- | :------------------------------------------------------------------------------------------------------------------------------------ |
| `CronCreate` | Pianificare una nuova attività. Accetta un'espressione cron a 5 campi, il prompt da eseguire e se ricorre o si attiva una sola volta. |
| `CronList`   | Elencare tutte le attività pianificate con i loro ID, pianificazioni e prompt.                                                        |
| `CronDelete` | Annullare un'attività per ID.                                                                                                         |

Ogni attività pianificata ha un ID di 8 caratteri che potete passare a `CronDelete`. Una sessione può contenere fino a 50 attività pianificate contemporaneamente.

<h2 id="how-scheduled-tasks-run">
  Come vengono eseguite le attività pianificate
</h2>

Lo scheduler controlla ogni secondo le attività dovute e le accoda a bassa priorità. Un prompt pianificato si attiva tra i vostri turni, non mentre Claude sta rispondendo. Se Claude è occupato quando un'attività scade, il prompt attende fino al termine del turno corrente.

Tutti i tempi vengono interpretati nel vostro fuso orario locale. Un'espressione cron come `0 9 * * *` significa le 9 del mattino ovunque stiate eseguendo Claude Code, non UTC.

<h3 id="jitter">
  Jitter
</h3>

Per evitare che ogni sessione colpisca l'API nello stesso momento, lo scheduler aggiunge un offset deterministico ai tempi di attivazione:

* Le attività ricorrenti si attivano fino a 30 minuti dopo l'ora pianificata (o fino a metà dell'intervallo, per le attività che vengono eseguite più frequentemente di ogni ora). Un processo orario pianificato per `:00` potrebbe attivarsi in qualsiasi momento fino a `:30`.
* Le attività una tantum pianificate per l'inizio o la fine dell'ora si attivano fino a 90 secondi prima.

L'offset è derivato dall'ID dell'attività, quindi la stessa attività ottiene sempre lo stesso offset. Se il timing esatto è importante, scegliete un minuto che non sia `:00` o `:30`, ad esempio `3 9 * * *` invece di `0 9 * * *`, e il jitter una tantum non si applicherà.

<h3 id="seven-day-expiry">
  Scadenza di sette giorni
</h3>

Le attività ricorrenti scadono automaticamente 7 giorni dopo la creazione. L'attività si attiva un'ultima volta, quindi si elimina. Questo limita il tempo di esecuzione di un ciclo dimenticato. Se avete bisogno che un'attività ricorrente duri più a lungo, annullate e ricreate prima che scada, oppure utilizzate [Routines](/docs/it/routines) o [Attività pianificate Desktop](/docs/it/desktop-scheduled-tasks) per la pianificazione durevole.

<h2 id="cron-expression-reference">
  Riferimento dell'espressione cron
</h2>

`CronCreate` accetta espressioni cron standard a 5 campi: `minute hour day-of-month month day-of-week`. Tutti i campi supportano caratteri jolly (`*`), valori singoli (`5`), step (`*/15`), intervalli (`1-5`) e elenchi separati da virgole (`1,15,30`).

| Esempio        | Significato                              |
| :------------- | :--------------------------------------- |
| `*/5 * * * *`  | Ogni 5 minuti                            |
| `0 * * * *`    | Ogni ora in punto                        |
| `7 * * * *`    | Ogni ora alle 7 minuti passati           |
| `0 9 * * *`    | Ogni giorno alle 9 del mattino locale    |
| `0 9 * * 1-5`  | Giorni feriali alle 9 del mattino locale |
| `30 14 15 3 *` | 15 marzo alle 14:30 locale               |

Day-of-week utilizza `0` o `7` per domenica fino a `6` per sabato. La sintassi estesa come `L`, `W`, `?` e gli alias dei nomi come `MON` o `JAN` non sono supportati.

Quando sia day-of-month che day-of-week sono vincolati, una data corrisponde se uno dei campi corrisponde. Questo segue la semantica standard di vixie-cron.

<h2 id="disable-scheduled-tasks">
  Disabilitare le attività pianificate
</h2>

Impostare `CLAUDE_CODE_DISABLE_CRON=1` nel vostro ambiente per disabilitare completamente lo scheduler. Gli strumenti cron e `/loop` diventano non disponibili e tutte le attività già pianificate smettono di attivarsi. Vedere [Variabili di ambiente](/docs/it/env-vars) per l'elenco completo dei flag di disabilitazione.

<h2 id="limitations">
  Limitazioni
</h2>

La pianificazione con ambito di sessione ha vincoli intrinseci:

* Le attività si attivano solo mentre Claude Code è in esecuzione e inattivo. La chiusura del terminale o l'uscita dalla sessione le interrompe. [Mettere in background la sessione](/docs/it/agent-view#from-inside-a-session) trasporta le attività `/loop` a una sessione in background, che continua a funzionare senza un terminale.
* Nessun recupero per attivazioni perse. Se l'ora pianificata di un'attività passa mentre Claude è occupato in una richiesta a lunga esecuzione, si attiva una sola volta quando Claude diventa inattivo, non una volta per ogni intervallo perso.
* L'avvio di una nuova conversazione cancella tutte le attività con ambito di sessione. La ripresa con `claude --resume` o `claude --continue` ripristina le attività che non sono scadute: attività ricorrenti entro sette giorni dalla creazione, e attività una tantum la cui ora pianificata non è ancora passata. Le attività Bash in background e monitor non vengono mai ripristinate al riavvio.

Per l'automazione basata su cron che deve essere eseguita senza supervisione:

* [Routines](/docs/it/routines): eseguite su infrastruttura gestita da Anthropic su un programma, tramite chiamata API o su eventi GitHub
* [GitHub Actions](/docs/it/github-actions): utilizzare un trigger `schedule` in CI
* [Attività pianificate Desktop](/docs/it/desktop-scheduled-tasks): eseguite localmente sulla vostra macchina
