> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Checkpointing

> Traccia, riavvolgi e riassumi le modifiche e la conversazione di Claude per gestire lo stato della sessione.

Claude Code traccia automaticamente le modifiche ai file di Claude mentre lavori, permettendoti di annullare rapidamente le modifiche e tornare a stati precedenti se qualcosa non va come previsto.

<h2 id="how-checkpoints-work">
  Come funziona il checkpointing
</h2>

Mentre lavori con Claude, il checkpointing cattura automaticamente lo stato del tuo codice prima di ogni prompt dell'utente. Questa rete di sicurezza ti permette di affrontare compiti ambiziosi e su larga scala sapendo che puoi sempre tornare a uno stato di codice precedente.

<h3 id="automatic-tracking">
  Tracciamento automatico
</h3>

Claude Code traccia tutti i cambiamenti effettuati dai suoi strumenti di modifica dei file:

* Ogni prompt dell'utente crea un nuovo checkpoint
* Claude Code mantiene snapshot dei file per i 100 checkpoint più recenti in una sessione. L'eliminazione di un checkpoint più vecchio cancella i file snapshot che nessun checkpoint rimanente referenzia, ad eccezione del primo snapshot di ogni file, che l'estensione VS Code utilizza come baseline per i suoi diff di sessione. Prima della v2.1.208, questi file snapshot sostituiti rimanevano su disco fino a quando la sessione non veniva pulita.
* I checkpoint vengono salvati con la conversazione, quindi una sessione ripresa può comunque `/rewind` verso di essi
* Puliti automaticamente insieme alle sessioni dopo 30 giorni (configurabile)

<h3 id="rewind-and-summarize">
  Riavvolgi e riassumi
</h3>

Esegui `/rewind`, oppure premi `Esc` due volte quando il campo di input del prompt è vuoto, per aprire il menu di riavvolgimento.

<Note>
  Se il campo di input del prompt contiene testo, doppio `Esc` lo cancella invece di aprire il menu. Il testo cancellato viene salvato nella cronologia di input, quindi premi `Su` per richiamarlo dopo aver terminato nel menu di riavvolgimento.
</Note>

Il menu di riavvolgimento elenca ogni prompt che hai inviato durante la sessione. Seleziona il punto su cui desideri agire, quindi scegli un'azione:

* **Ripristina codice e conversazione**: ripristina sia il codice che la conversazione a quel punto
* **Ripristina conversazione**: riavvolgi al messaggio mantenendo il codice attuale
* **Ripristina codice**: ripristina le modifiche ai file mantenendo la conversazione
* **Riassumi da qui**: comprimi la conversazione da questo punto in avanti in un riassunto, liberando spazio nella context window
* **Riassumi fino a qui**: comprimi la conversazione prima di questo punto in un riassunto, mantenendo i messaggi successivi intatti
* **Non importa**: torna all'elenco dei messaggi senza apportare modifiche

Dopo aver ripristinato la conversazione o aver scelto Riassumi da qui, il prompt originale dal messaggio selezionato viene ripristinato nel campo di input in modo che tu possa reinviarlo o modificarlo.

Scegliendo Riassumi fino a qui ti lascia alla fine della conversazione con l'input vuoto.

<h4 id="rewind-past-a-cleared-conversation">
  Riavvolgi oltre una conversazione cancellata
</h4>

Se hai eseguito `/clear` in precedenza nello stesso processo Claude Code, il menu di riavvolgimento mostra una voce aggiuntiva in cima all'elenco etichettata `/resume <session-id> (previous session)`. Selezionala per riprendere la conversazione che era attiva prima che `/clear` venisse eseguito. La voce è disponibile fino a quando non esci da Claude Code o riprendi una sessione diversa, e richiede Claude Code v2.1.191 o versione successiva. Nelle versioni precedenti, esegui `/resume` e scegli la sessione precedente dall'elenco.

<h4 id="restore-vs-summarize">
  Ripristina vs. riassumi
</h4>

Le opzioni di ripristino ripristinano lo stato: annullano le modifiche al codice, la cronologia della conversazione o entrambi. Le opzioni di riassunto comprimono parte della conversazione in un riassunto generato dall'IA senza modificare i file su disco:

* **Riassumi da qui**: i messaggi prima del messaggio selezionato rimangono intatti. Il messaggio selezionato e tutto ciò che segue vengono sostituiti con un riassunto. Usa questo per scartare una discussione laterale mantenendo il contesto iniziale in dettaglio completo.
* **Riassumi fino a qui**: i messaggi prima del messaggio selezionato vengono sostituiti con un riassunto. Il messaggio selezionato e tutto ciò che segue rimangono intatti, e rimani alla fine della conversazione. Usa questo per comprimere la discussione di configurazione iniziale mantenendo il lavoro recente in dettaglio completo.

In entrambi i casi i messaggi originali vengono conservati nella trascrizione della sessione, quindi Claude può fare riferimento ai dettagli se necessario. Puoi digitare istruzioni facoltative per guidare su cosa si concentra il riassunto. Questo è simile a `/compact`, ma mirato: invece di riassumere l'intera conversazione, scegli quale lato del messaggio selezionato comprimere.

<Note>
  Riassumi ti mantiene nella stessa sessione e comprime il contesto. Se desideri creare un ramo e provare un approccio diverso preservando la sessione originale intatta, usa [fork](/docs/it/sessions#branch-a-session) invece (`claude --continue --fork-session`).
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

Il checkpointing non traccia i file modificati dai comandi bash. Ad esempio, se Claude Code esegue:

```bash theme={null}
rm file.txt
mv old.txt new.txt
cp source.txt dest.txt
```

Queste modifiche ai file non possono essere annullate tramite riavvolgimento. Solo le modifiche dirette ai file effettuate attraverso gli strumenti di modifica dei file di Claude vengono tracciate.

<h3 id="external-changes-not-tracked">
  Le modifiche esterne non vengono tracciate
</h3>

Il checkpointing traccia solo i file che sono stati modificati nella sessione corrente. Le modifiche manuali che effettui ai file al di fuori di Claude Code e le modifiche da altre sessioni concorrenti normalmente non vengono acquisite, a meno che non modifichino gli stessi file della sessione corrente.

<h3 id="not-a-replacement-for-version-control">
  Non è un sostituto del controllo della versione
</h3>

I checkpoint sono progettati per il recupero rapido a livello di sessione. Per la cronologia permanente della versione e la collaborazione:

* Continua a utilizzare il controllo della versione (ad es. Git) per commit, rami e cronologia a lungo termine
* I checkpoint completano ma non sostituiscono il controllo della versione appropriato
* Pensa ai checkpoint come "annulla locale" e Git come "cronologia permanente"

<h2 id="see-also">
  Vedi anche
</h2>

* [Modalità interattiva](/docs/it/interactive-mode) - Scorciatoie da tastiera e controlli della sessione
* [Comandi](/docs/it/commands) - Accesso ai checkpoint usando `/rewind`
* [Riferimento CLI](/docs/it/cli-reference) - Opzioni della riga di comando
