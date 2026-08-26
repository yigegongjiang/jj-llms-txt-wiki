> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Pianificare attività ricorrenti in Claude Code Desktop

> Configura attività pianificate in Claude Code Desktop per eseguire Claude automaticamente su base ricorrente per revisioni del codice giornaliere, audit delle dipendenze o briefing mattutini.

Le attività pianificate avviano una nuova sessione automaticamente a un'ora e una frequenza che scegli. Usale per lavori ricorrenti come revisioni del codice giornaliere, controlli degli aggiornamenti delle dipendenze o briefing mattutini che estraggono dati dal tuo calendario e dalla tua posta in arrivo.

La pagina **Routines** dell'app Desktop ti consente di creare sia attività pianificate locali che [routine](/docs/it/routines) remote. Un'attività locale viene eseguita sulla tua macchina con accesso diretto ai tuoi file e strumenti, ma si attiva solo mentre l'app è aperta e il tuo computer è sveglio. Una routine remota viene eseguita su infrastrutture cloud gestite da Anthropic anche quando il tuo computer è spento e può anche attivarsi su chiamate API o eventi GitHub. Questa pagina copre le attività pianificate locali; per le routine remote e le loro opzioni di attivazione, vedi [Routines](/docs/it/routines).

<h2 id="compare-scheduling-options">
  Confronta le opzioni di pianificazione
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

<Note>
  Per impostazione predefinita, le attività pianificate vengono eseguite rispetto a qualsiasi stato si trovi la tua directory di lavoro, incluse le modifiche non sottoposte a commit. Abilita l'interruttore worktree quando crei l'attività per dare a ogni esecuzione il suo proprio worktree Git isolato, nello stesso modo in cui funzionano le [sessioni parallele](/docs/it/desktop#work-in-parallel-with-sessions).
</Note>

<h2 id="create-a-scheduled-task">
  Crea un'attività pianificata
</h2>

Fai clic su **Routines** nella barra laterale, quindi fai clic su **New routine** e scegli **Local**. Configura questi campi:

| Campo        | Descrizione                                                                                                                                                                                                                                                                                                                                   |
| ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Name         | Identificatore per l'attività. Convertito in kebab-case minuscolo e utilizzato come nome della cartella su disco. Deve essere univoco tra le tue attività.                                                                                                                                                                                    |
| Description  | Breve riepilogo mostrato nell'elenco delle attività.                                                                                                                                                                                                                                                                                          |
| Instructions | Cosa Claude dovrebbe fare quando l'attività viene eseguita. Scrivi questo nello stesso modo in cui scriveresti qualsiasi messaggio nella casella del prompt. L'input delle istruzioni include selettori per la modalità di autorizzazione e il modello, e sotto di esso selezioni la cartella di lavoro e se eseguire in un worktree isolato. |
| Schedule     | Con quale frequenza viene eseguita l'attività. Vedi [opzioni di pianificazione](#schedule-options) di seguito.                                                                                                                                                                                                                                |

Una cartella è obbligatoria prima di poter salvare l'attività. Se non hai ancora considerato attendibile quella cartella, Desktop ti chiede di considerarla attendibile prima di salvarla.

Puoi anche creare un'attività descrivendo quello che vuoi in qualsiasi sessione. Ad esempio, "configura una revisione del codice giornaliera che viene eseguita ogni mattina alle 9" crea un'attività ricorrente, e "ricordami alle 15 domani di controllare il deploy" crea un'attività una tantum che si disabilita dopo che si attiva.

<h2 id="schedule-options">
  Opzioni di pianificazione
</h2>

Scegli un preset dal controllo Schedule:

* **Manual**: nessuna pianificazione, viene eseguita solo quando fai clic su **Run now**. Utile per salvare un prompt che attivi su richiesta
* **Hourly**: viene eseguita ogni ora
* **Daily**: mostra un selettore di ora, per impostazione predefinita alle 9:00 AM ora locale
* **Weekdays**: come Daily ma salta sabato e domenica
* **Weekly**: mostra un selettore di ora e un selettore di giorno

Per intervalli che il selettore non offre, come ogni 15 minuti, il primo di ogni mese o un'esecuzione singola a un'ora futura specifica, chiedi a Claude in qualsiasi sessione Desktop di impostare la pianificazione. Usa il linguaggio naturale; ad esempio, "pianifica un'attività per eseguire tutti i test ogni 6 ore."

<h2 id="how-scheduled-tasks-run">
  Come vengono eseguite le attività pianificate
</h2>

Le attività pianificate vengono eseguite sulla tua macchina. Desktop controlla la pianificazione ogni minuto mentre l'app è aperta e avvia una sessione nuova quando un'attività è dovuta, indipendentemente da qualsiasi sessione manuale che hai aperta. Ogni attività riceve un piccolo ritardo di pochi minuti dopo l'ora pianificata per scaglionare il traffico API. Il ritardo è deterministico: la stessa attività inizia sempre allo stesso offset.

Quando un'attività si attiva, ricevi una notifica desktop e una nuova sessione appare sotto una sezione **Scheduled** nella barra laterale. Aprila per vedere cosa ha fatto Claude, rivedere le modifiche o rispondere ai prompt di autorizzazione. La sessione funziona come qualsiasi altra: Claude può modificare file, eseguire comandi, creare commit e aprire pull request.

Le attività vengono eseguite solo mentre l'app desktop è in esecuzione e il tuo computer è sveglio. Se il tuo computer dorme durante un'ora pianificata, l'esecuzione viene saltata. Per impedire il sonno inattivo, abilita **Keep computer awake** in Impostazioni sotto **Desktop app → General**. Chiudere il coperchio del laptop lo mette comunque in modalità sospensione. Per attività che devono essere eseguite anche quando il tuo computer è spento, o che dovrebbero attivarsi su una chiamata API o un evento GitHub, crea invece una [routine](/docs/it/routines) remota.

<h2 id="missed-runs">
  Esecuzioni mancate
</h2>

Quando l'app si avvia o il tuo computer si riattiva, Desktop controlla se ogni attività ha perso esecuzioni negli ultimi sette giorni. Se lo ha fatto, Desktop avvia esattamente un'esecuzione di recupero per l'ora più recentemente mancata e scarta tutto ciò che è più vecchio. Un'attività giornaliera che ha perso sei giorni viene eseguita una volta al risveglio. Desktop mostra una notifica quando inizia un'esecuzione di recupero.

Tieni questo in mente quando scrivi i prompt. Un'attività pianificata per le 9 potrebbe essere eseguita alle 23 se il tuo computer è stato in sospensione tutto il giorno. Se il timing è importante, aggiungi protezioni al prompt stesso, ad esempio: "Rivedi solo i commit di oggi. Se è dopo le 17, salta la revisione e pubblica solo un riepilogo di ciò che è stato perso."

<h2 id="permissions-for-scheduled-tasks">
  Autorizzazioni per le attività pianificate
</h2>

Ogni attività ha la sua propria modalità di autorizzazione, che imposti quando crei o modifichi l'attività. Le regole di autorizzazione da `~/.claude/settings.json` si applicano anche alle sessioni di attività pianificate. Se un'attività viene eseguita in modalità Ask e ha bisogno di eseguire uno strumento per il quale non ha autorizzazione, l'esecuzione si blocca fino a quando non la approvi. La sessione rimane aperta nella barra laterale in modo da poter rispondere in seguito.

Per evitare blocchi, fai clic su **Run now** dopo aver creato un'attività, guarda i prompt di autorizzazione e seleziona "always allow" per ognuno. Le esecuzioni future di quell'attività approvano automaticamente gli stessi strumenti senza chiedere. Puoi rivedere e revocare queste approvazioni dalla pagina dei dettagli dell'attività.

Gli strumenti Connector [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) e gli strumenti MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool) richiedono un prompt ad ogni chiamata e non offrono un'opzione "always allow". Le esecuzioni che chiamano questi strumenti si bloccano ogni volta.

<h2 id="manage-scheduled-tasks">
  Gestisci le attività pianificate
</h2>

Fai clic su un'attività nell'elenco **Routines** per aprire la sua pagina di dettaglio. Da qui puoi:

* **Run now**: avvia l'attività immediatamente senza aspettare l'ora pianificata successiva
* **Status**: attiva/disattiva tra Active e Paused per mettere in pausa o riprendere le esecuzioni pianificate senza eliminare l'attività
* **Edit**: modifica le istruzioni, la pianificazione, la cartella o altre impostazioni
* **Review history**: vedi ogni esecuzione passata, incluse le esecuzioni saltate. Passa il mouse su una voce saltata per vedere il motivo: il tuo computer era in sospensione, l'esecuzione precedente era ancora in corso o altre attività pianificate erano già in esecuzione. Fai clic su **Show more** per caricare voci più vecchie.
* **Review allowed permissions**: vedi e revoca le approvazioni degli strumenti salvate per questa attività dal pannello **Always allowed**
* **Delete**: rimuovi l'attività e archivia tutte le sessioni che ha creato. Una casella di controllo **Also delete files on disk** appare nella finestra di dialogo di conferma; selezionala per rimuovere anche il file `SKILL.md` dell'attività e i dati associati da `~/.claude/scheduled-tasks/`.

Puoi anche elencare, creare, modificare e mettere in pausa le attività chiedendo a Claude in qualsiasi sessione Desktop. Ad esempio, "pausa la mia attività dependency-audit" o "mostrami le mie attività pianificate." Per eliminare un'attività, usa il pulsante **Delete** sulla sua pagina di dettaglio.

Un'attività pianificata può anche modificare la sua propria pianificazione o prompt da una sessione in esecuzione utilizzando lo strumento MCP `update_scheduled_task`. Questo consente a un'attività di ripianificarsi in base a ciò che trova, ad esempio, ripianificando una revisione del codice per essere eseguita prima quando rileva che è stato creato un ramo di rilascio.

Per modificare il prompt di un'attività su disco, apri `~/.claude/scheduled-tasks/<task-name>/SKILL.md` (o sotto [`CLAUDE_CONFIG_DIR`](/docs/it/env-vars) se impostato). Il file utilizza frontmatter YAML per `name` e `description`, con il prompt come corpo. Le modifiche hanno effetto alla prossima esecuzione. La pianificazione, la cartella, il modello e lo stato abilitato non sono in questo file: modificali tramite il modulo Edit o chiedi a Claude.

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Routines](/docs/it/routines): esegui attività su infrastrutture gestite da Anthropic su una pianificazione, tramite chiamata API o in risposta a eventi GitHub, anche quando il tuo computer è spento
* [Run prompts on a schedule](/docs/it/scheduled-tasks): pianificazione con ambito di sessione con `/loop` nella CLI
* [Claude Code GitHub Actions](/docs/it/github-actions): esegui Claude su una pianificazione in CI invece che sulla tua macchina
* [Use Claude Code Desktop](/docs/it/desktop): la guida completa dell'app Desktop
