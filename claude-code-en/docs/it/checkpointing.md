> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Checkpointing

> Traccia, riavvolgi e riassumi le modifiche e la conversazione di Claude per gestire lo stato della sessione.

Claude Code traccia automaticamente le modifiche ai file di Claude mentre lavori, permettendoti di annullare rapidamente le modifiche e tornare a stati precedenti se qualcosa non va come previsto.

<h2 id="how-checkpoints-work">
  Come funziona il checkpointing
</h2>

Mentre lavori con Claude, il checkpointing cattura automaticamente lo stato del tuo codice prima di ogni prompt che invii e che avvia un turno.

<h3 id="automatic-tracking">
  Tracciamento automatico
</h3>

Claude Code traccia tutti i cambiamenti effettuati dai suoi strumenti di modifica dei file:

* Ogni prompt che invii e che avvia un turno crea un nuovo checkpoint
* Claude Code mantiene snapshot dei file per i 100 checkpoint più recenti in una sessione. L'eliminazione di un checkpoint più vecchio cancella i file snapshot che nessun checkpoint rimanente referenzia, ad eccezione del primo snapshot di ogni file, che l'estensione VS Code utilizza come baseline per i suoi diff di sessione.
* Claude Code salva i checkpoint con la conversazione, quindi puoi comunque eseguire `/rewind` dopo aver ripreso una sessione
* Claude Code elimina gli snapshot dei file di una sessione nella [retention sweep](/docs/it/claude-directory#cleaned-up-automatically), per impostazione predefinita circa 30 giorni dopo l'ultimo salvataggio della sessione. Il riavvolgimento a un checkpoint i cui snapshot sono scomparsi può fallire con [`No files were restored`](/docs/it/errors#no-files-were-restored). Per mantenere gli snapshot più a lungo, imposta [`cleanupPeriodDays`](/docs/it/settings-reference#cleanupperioddays).

<h3 id="rewind-and-summarize">
  Riavvolgi e riassumi
</h3>

Esegui `/rewind`, oppure premi `Esc` due volte quando il campo di input del prompt è vuoto, per aprire il menu di riavvolgimento.

<Note>
  Se il campo di input del prompt contiene testo, doppio `Esc` lo cancella invece di aprire il menu. Il testo cancellato viene salvato nella cronologia di input, quindi premi `Su` per richiamarlo dopo aver terminato nel menu di riavvolgimento.
</Note>

Il menu di riavvolgimento elenca ogni prompt che hai inviato durante la sessione, ad eccezione dei [messaggi che si sono uniti a un turno in corso](#messages-sent-mid-turn-not-checkpointed). Seleziona il punto su cui desideri agire, quindi scegli un'azione:

* **Ripristina codice e conversazione**: ripristina sia il codice che la conversazione a quel punto
* **Ripristina conversazione**: riavvolgi al messaggio mantenendo il codice attuale
* **Ripristina codice**: ripristina le modifiche ai file mantenendo la conversazione
* **Riassumi da qui**: comprimi la conversazione da questo punto in avanti in un riassunto, liberando spazio nella context window
* **Riassumi fino a qui**: comprimi la conversazione prima di questo punto in un riassunto, mantenendo i messaggi successivi intatti
* **Non importa**: torna all'elenco dei messaggi senza apportare modifiche

Le due opzioni di ripristino del codice appaiono solo quando il checkpoint selezionato ha tracciato modifiche ai file da ripristinare. Se nessuna modifica ai file è stata acquisita dopo quel punto, il menu offre solo **Ripristina conversazione**, le opzioni di riassunto e **Non importa**.

Dopo aver ripristinato la conversazione o aver scelto Riassumi da qui, il prompt originale dal messaggio selezionato viene ripristinato nel campo di input in modo che tu possa reinviarlo o modificarlo.

Scegliendo Riassumi fino a qui ti lascia alla fine della conversazione con l'input vuoto. Con entrambe le opzioni di riassunto, un marcatore **Summarized conversation** appare nella conversazione dove i messaggi compressi erano.

<h4 id="rewind-past-a-cleared-conversation">
  Riavvolgi oltre una conversazione cancellata
</h4>

Se hai eseguito `/clear` in precedenza nello stesso processo Claude Code, il menu di riavvolgimento mostra una voce aggiuntiva in cima all'elenco etichettata `/resume <session-id> (previous session)`. Selezionala per riprendere la conversazione che era attiva prima che `/clear` venisse eseguito. La voce è disponibile fino a quando non esci da Claude Code o riprendi una sessione diversa.

<h4 id="guide-a-summary">
  Guida un riassunto
</h4>

Il riassunto non modifica i file su disco, e i messaggi originali rimangono nella trascrizione della sessione, quindi Claude può comunque fare riferimento ai dettagli. Per guidare su cosa si concentra il riassunto, evidenzia un'opzione **Summarize** con i tasti freccia e digita le istruzioni dove la riga legge **add context (optional)**, quindi premi `Enter`. Selezionando l'opzione con il suo tasto numerico si riassume immediatamente senza istruzioni.

<Note>
  Summarize ti mantiene nella stessa sessione e comprime il contesto, come un `/compact` mirato. Per creare un ramo e provare un approccio diverso preservando la sessione originale intatta, usa [`/branch`](/docs/it/sessions#branch-a-session) o `claude --continue --fork-session` invece.
</Note>

<h2 id="common-use-cases">
  Casi d'uso comuni
</h2>

I checkpoint sono particolarmente utili quando:

* **Esplorare alternative**: prova diversi approcci di implementazione senza perdere il tuo punto di partenza
* **Recuperare da errori**: annulla rapidamente le modifiche che hanno introdotto bug o rotto la funzionalità
* **Iterare sulle funzionalità**: sperimenta variazioni sapendo che puoi tornare a stati funzionanti
* **Liberare spazio di contesto**: riassumi una sessione di debug dettagliata dal punto intermedio in avanti, mantenendo le tue istruzioni iniziali intatte

<h2 id="limitations">
  Limitazioni
</h2>

<h3 id="bash-command-changes-not-tracked">
  Le modifiche dei comandi Bash non vengono tracciate
</h3>

Il checkpointing non traccia i file modificati dai comandi Bash. Ad esempio, se Claude Code esegue:

```bash theme={null}
rm file.txt
mv old.txt new.txt
cp source.txt dest.txt
```

Queste modifiche ai file non possono essere annullate tramite rewind. Solo le modifiche dirette ai file effettuate attraverso gli strumenti di modifica dei file di Claude vengono tracciate.

<h3 id="subagent-edits-not-restored">
  Le modifiche dei subagent non vengono ripristinate
</h3>

Un [subagent](/docs/it/sub-agents) effettua modifiche con gli strumenti di modifica dei file di Claude, ma Claude Code di solito non acquisisce queste modifiche nei checkpoint della tua sessione. Se il rewind ripristina le modifiche dipende da come viene eseguito il subagent:

* **Skill forked in foreground**: una [skill con `context: fork`](/docs/it/skills#run-skills-in-a-subagent) che viene eseguita in foreground modifica il tuo working tree durante il tuo turno, quindi il rewind ripristina le sue modifiche come al solito. Imposta `background: false` per eseguire un fork in foreground; alcune situazioni, [elencate nella pagina delle skills](/docs/it/skills#run-skills-in-a-subagent), lo eseguono lì indipendentemente dall'impostazione.
* **Qualsiasi altro subagent**: il rewind non ripristina le modifiche. Utilizza git per ripristinarle. Questo include una skill forked che viene eseguita in background, l'impostazione predefinita, e un'esecuzione di [`/code-review --fix`](/docs/it/code-review) in background.

<h3 id="external-changes-not-tracked">
  Le modifiche esterne non vengono tracciate
</h3>

Il checkpointing traccia solo i file che sono stati modificati nella sessione corrente. Le modifiche manuali che effettui ai file al di fuori di Claude Code e le modifiche da altre sessioni concorrenti normalmente non vengono acquisite, a meno che non modifichino gli stessi file della sessione corrente.

<h3 id="messages-sent-mid-turn-not-checkpointed">
  I messaggi inviati a metà turno non vengono sottoposti a checkpoint
</h3>

Quando un messaggio che [metti in coda mentre Claude lavora](/docs/it/interactive-mode#queue-messages-while-claude-works) raggiunge Claude durante il turno in esecuzione, si unisce a quel turno invece di iniziarne uno nuovo. Il messaggio appare nella conversazione, ma Claude Code non crea un checkpoint per esso, e il menu di rewind non lo elenca. Un messaggio in coda che Claude Code invia come suo proprio turno riceve un checkpoint come al solito.

Per rimuovere tale messaggio, o annullare le modifiche che Claude ha apportato dopo di esso, riavvolgi al prompt che ha avviato il turno. Questo riavvolge l'intero turno, incluso il lavoro che Claude ha svolto prima dell'arrivo del tuo messaggio.

<h3 id="symlinked-and-hard-linked-paths-not-restored">
  I percorsi symlink e hard-link non vengono ripristinati
</h3>

Il checkpointing non riavvolge i file symlink o hard-link. Quando scegli **Restore code** o **Restore code and conversation** dal menu `/rewind`, Claude Code salta qualsiasi percorso tracciato che è un symlink o hard link e mostra un avviso `Restored the code, but skipped N files`. I file saltati mantengono i loro contenuti attuali. Per annullare le modifiche della sessione a uno di essi, chiedi a Claude di invertire la modifica o modifica il file tu stesso. I file di configurazione che un gestore dotfile symlink nel tuo progetto e i file che pnpm hard-link nel posto rientrano entrambi in questa categoria.

Per vedere quali percorsi un ripristino salta, attiva la registrazione di debug con `/debug` prima di ripristinare: il log di debug in `~/.claude/debug/<session-id>.txt` nomina ogni percorso saltato. Per ogni motivo di salto e i passaggi di recupero, vedi [la voce skipped-files nel riferimento degli errori](/docs/it/errors#restored-the-code-but-skipped-files).

<h3 id="not-a-replacement-for-version-control">
  Non è un sostituto del controllo della versione
</h3>

I checkpoint sono progettati per il recupero rapido a livello di sessione. Per la cronologia permanente della versione e la collaborazione, continua a utilizzare il controllo della versione, come Git, per commit, rami e cronologia a lungo termine.

<h2 id="see-also">
  Vedi anche
</h2>

* [Modalità interattiva](/docs/it/interactive-mode) - Scorciatoie da tastiera e controlli della sessione
* [Comandi](/docs/it/commands) - Accesso ai checkpoint usando `/rewind`
* [Riferimento CLI](/docs/it/cli-reference) - Opzioni della riga di comando
