> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gestire le sessioni

> Assegnare nomi, riprendere, creare rami e passare tra conversazioni di Claude Code. Copre `--continue`, `--resume`, `--from-pr`, il selezionatore `/resume`, la denominazione delle sessioni, l'esportazione dei trascritti e dove vengono archiviati i trascritti.

Una sessione è una conversazione salvata legata a una directory di progetto. Claude Code la archivia localmente mentre lavori, così puoi riprendere da dove hai lasciato, creare un ramo per provare un approccio diverso, o passare tra attività.

L'[app desktop](/docs/it/desktop#work-in-parallel-with-sessions), [Claude Code sul web](/docs/it/claude-code-on-the-web) e l'[estensione VS Code](/docs/it/vs-code#resume-past-conversations) mantengono ciascuno la propria cronologia delle sessioni. Questa pagina copre la CLI.

<h2 id="resume-a-session">
  Riprendere una sessione
</h2>

Le sessioni vengono salvate continuamente in [file di trascritto locali](#export-and-locate-session-data) mentre lavori, quindi puoi tornare a una dopo aver chiuso o eseguito `/clear`. Usa questi punti di ingresso:

| Comando                     | Cosa fa                                                      |
| :-------------------------- | :----------------------------------------------------------- |
| `claude --continue`         | Riprende la sessione più recente nella directory corrente    |
| `claude --resume`           | Apre il [selezionatore di sessioni](#use-the-session-picker) |
| `claude --resume <name>`    | Riprende direttamente la sessione denominata                 |
| `claude --from-pr <number>` | Riprende la sessione collegata a quella pull request         |
| `/resume`                   | Passa a una conversazione diversa da una sessione attiva     |

Le sessioni create con [`claude -p`](/docs/it/headless) o l'[Agent SDK](/docs/it/agent-sdk/overview) non vengono visualizzate nel selezionatore di sessioni, ma puoi comunque riprenderne una passando il suo ID di sessione a `claude --resume <session-id>`. Esegui questo dalla directory in cui è stata avviata la sessione: la ricerca dell'ID di sessione è limitata alla directory del progetto corrente e ai suoi git worktrees, quindi una sessione creata altrove segnala `No conversation found with session ID: <session-id>`.

<h3 id="where-the-session-picker-looks">
  Dove il selezionatore di sessioni cerca
</h3>

Le sessioni vengono archiviate per directory di progetto. Per impostazione predefinita, il selezionatore di sessioni mostra le sessioni interattive dal worktree corrente, più le sessioni avviate altrove che hanno aggiunto la directory corrente con `/add-dir`. Usa `Ctrl+W` per ampliare a tutti i worktree del repository o `Ctrl+A` per ampliare a ogni progetto su questa macchina.

A partire dalla v2.1.169, lo spostamento di una sessione con [`/cd`](/docs/it/commands) la trasferisce nell'archivio del progetto della nuova directory, quindi appare nel selezionatore di quella directory in seguito. A partire dalla v2.1.196, una sessione spostata rimane fuori dal selezionatore della directory precedente anche dopo un arresto anomalo o un'uscita forzata. Nelle versioni precedenti, potrebbe anche riapparire nell'elenco della directory precedente dopo un'uscita non pulita quando il percorso precedente conteneva caratteri speciali come trattini bassi.

Selezionare una sessione da un altro worktree dello stesso repository la riprende sul posto. Selezionare una sessione da un progetto non correlato copia un comando `cd` e resume negli appunti.

La ripresa per nome si risolve nel repository corrente e nei suoi worktree. Entrambi i moduli cercano una corrispondenza esatta e la riprendono direttamente anche se si trova in un worktree diverso:

| Comando                  | Corrispondenza esatta | Nome ambiguo                                                                                |
| :----------------------- | :-------------------- | :------------------------------------------------------------------------------------------ |
| `claude --resume <name>` | Riprende direttamente | Apre il selezionatore di sessioni con il nome pre-compilato come termine di ricerca         |
| `/resume <name>`         | Riprende direttamente | Segnala un errore; esegui `/resume` senza argomenti per aprire il selezionatore di sessioni |

<h2 id="name-your-sessions">
  Assegnare nomi alle sessioni
</h2>

Dai alle sessioni nomi descrittivi in modo che siano trovabili nel selezionatore di sessioni e riprendibili per nome. Questo è particolarmente importante quando stai lavorando su più attività in parallelo.

| Quando                        | Come impostare il nome                                                                                                                                                                         |
| :---------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| All'avvio                     | `claude -n auth-refactor`                                                                                                                                                                      |
| Durante una sessione          | `/rename auth-refactor`. Il nome appare anche sulla barra del prompt                                                                                                                           |
| Dal selezionatore di sessioni | Evidenzia una sessione e premi `Ctrl+R`                                                                                                                                                        |
| Sull'accettazione del piano   | Accettare un piano in [plan mode](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode) assegna un nome alla sessione dal contenuto del piano a meno che non ne abbia già impostato uno |

Una volta che una sessione è denominata, torna a essa con `claude --resume <name>` o `/resume <name>`. Vedi [Riprendere una sessione](#resume-a-session) per come il comportamento della risoluzione dei nomi funziona nei worktree.

Le sessioni interattive che non nomini mai ricevono comunque un nome di visualizzazione predefinito quando si avviano. Richiede Claude Code v2.1.196 o versione successiva. Il valore predefinito combina il nome della directory di lavoro con un suffisso di due caratteri, ad esempio `my-app-3f`, e identifica la sessione negli elenchi delle sessioni in esecuzione, come la [agent view](/docs/it/agent-view) e l'output di `claude agents --json`.

Il valore predefinito non è un handle di ripresa: `claude --resume <name>`, `/resume <name>` e il selezionatore di sessioni corrispondono solo ai nomi che hai impostato. Denominare la sessione sostituisce il valore predefinito.

<h2 id="use-the-session-picker">
  Usare il selezionatore di sessioni
</h2>

Esegui `/resume` all'interno di una sessione, o `claude --resume` senza argomenti, per aprire il selezionatore di sessioni interattivo. Usa queste scorciatoie da tastiera per navigare, cercare e ampliare l'elenco:

| Scorciatoia                                             | Azione                                                                                                                                                                             |
| :------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `↑` / `↓`                                               | Naviga tra le sessioni                                                                                                                                                             |
| `→` / `←`                                               | Espandi o comprimi le sessioni raggruppate                                                                                                                                         |
| `Enter`                                                 | Riprendi la sessione evidenziata                                                                                                                                                   |
| `Space`                                                 | Anteprima del contenuto della sessione. `Ctrl+V` funziona anche su terminali che non lo catturano come incolla                                                                     |
| `Ctrl+R`                                                | Rinomina la sessione evidenziata                                                                                                                                                   |
| `/` o qualsiasi carattere stampabile diverso da `Space` | Entra in modalità di ricerca e filtra le sessioni. Incolla un URL di pull o merge request di GitHub, GitHub Enterprise, GitLab o Bitbucket per trovare la sessione che l'ha creato |
| `Ctrl+A`                                                | Mostra le sessioni da tutti i progetti su questa macchina. Premi di nuovo per tornare al repository corrente                                                                       |
| `Ctrl+W`                                                | Mostra le sessioni da tutti i worktree del repository corrente. Premi di nuovo per tornare al worktree corrente. Mostrato solo nei repository multi-worktree                       |
| `Ctrl+B`                                                | Filtra le sessioni dal ramo git corrente. Premi di nuovo per mostrare tutti i rami                                                                                                 |
| `Esc`                                                   | Esci dal selezionatore di sessioni o dalla modalità di ricerca                                                                                                                     |

Ogni riga mostra il nome della sessione se impostato, altrimenti il riassunto della conversazione o il primo prompt, insieme al tempo dall'ultima attività, al conteggio dei messaggi e al ramo git. Il percorso del progetto appare dopo aver ampliato a tutti i progetti con `Ctrl+A`.

Le sessioni con rami create con `/branch`, `/rewind` o `--fork-session` sono raggruppate sotto la loro sessione radice. Premi `→` per espandere un gruppo.

<h2 id="branch-a-session">
  Creare un ramo di una sessione
</h2>

La creazione di un ramo crea una copia della conversazione finora e ti passa in essa, lasciando l'originale intatto. Usalo per provare un approccio diverso senza perdere il percorso su cui eri.

Da dentro una sessione, esegui `/branch` con un nome opzionale:

```text theme={null}
/branch try-streaming-approach
```

Se ometti il nome, Claude Code assegna un nome al nuovo ramo in base al primo prompt nella conversazione. A partire dalla v2.1.198 questo si applica anche dopo la [compattazione](/docs/it/how-claude-code-works#when-context-fills-up); le versioni precedenti ricadevano nel nome letterale `Branched conversation` invece di guardare oltre il riepilogo della compattazione al prompt originale iniziale.

Dalla riga di comando, combina `--continue` o `--resume` con `--fork-session`:

```bash theme={null}
claude --continue --fork-session
```

La sessione originale rimane invariata e disponibile nel selezionatore di sessioni. La conferma di `/branch` stampa due ID di sessione: il nuovo ramo in cui sei ora e l'originale. Per tornare all'originale, passa il suo ID a `/resume`, usa il selezionatore di sessioni, o esegui `/resume <original-name>`. I permessi che hai approvato con "allow for this session" non vengono trasferiti al nuovo ramo. Se riprendi la stessa sessione in due terminali senza creare un fork, i messaggi di entrambi si intercalano in un unico trascritto.

Per il rewind basato su checkpoint all'interno di una singola sessione, vedi [Checkpointing](/docs/it/checkpointing).

<h2 id="manage-context-within-a-session">
  Gestire il contesto all'interno di una sessione
</h2>

Questi comandi controllano cosa c'è nella finestra di contesto senza lasciare la sessione:

* **`/clear`**: ricomincia da capo con un contesto vuoto. La conversazione precedente viene salvata e riprendibile con `/resume`, oppure, nello stesso processo Claude Code, dalla [voce della sessione precedente del menu rewind](/docs/it/checkpointing#rewind-past-a-cleared-conversation)
* **`/compact [instructions]`**: sostituisci la cronologia con un riassunto, facoltativamente focalizzato su ciò che specifichi
* **`/context`**: mostra cosa sta attualmente consumando contesto

Per come la compattazione interagisce con CLAUDE.md, skills e regole, vedi la [guida della finestra di contesto](/docs/it/context-window). Per strategie su quando cancellare rispetto a compattare, vedi [Best practices](/docs/it/best-practices#manage-your-session).

<h2 id="export-and-locate-session-data">
  Esportare e individuare i dati della sessione
</h2>

Esegui `/export` per aprire un menu che ti consente di copiare la conversazione corrente negli appunti o salvarla come file di testo semplice, con messaggi e output degli strumenti renderizzati come testo leggibile. Passa un nome file per saltare il menu e scrivere direttamente in quel file.

<h3 id="access-conversations-from-scripts">
  Accedere alle conversazioni da script
</h3>

`/export` produce una trascrizione renderizzata per una persona da leggere. Le interfacce di seguito producono dati strutturati per uno script da analizzare: un risultato JSON da un'esecuzione, il percorso al file di trascrizione di una sessione, o un flusso live di eventi. Scegli in base a ciò che attiva lo script:

* **Esegui Claude una volta e cattura il risultato**: invoca `claude -p` con [`--output-format json` o `stream-json`](/docs/it/headless#get-structured-output) per catturare il risultato, l'ID della sessione, l'utilizzo e il costo di un'esecuzione non interattiva come JSON strutturato.
* **Poni una domanda a una sessione esistente**: passa un ID di sessione a [`claude -p --resume`](/docs/it/headless#continue-conversations) per inviare un prompt di follow-up, come una richiesta di riepilogo, e catturare la risposta strutturata.
* **Reagisci agli eventi della sessione**: leggi il campo `transcript_path` che [hooks](/docs/it/hooks#common-input-fields) e [comandi della riga di stato](/docs/it/statusline#available-data) ricevono come input. Un hook `SessionEnd` può archiviare la trascrizione quando una sessione termina.
* **Incorpora Claude in un'app TypeScript o Python**: usa l'[Agent SDK](/docs/it/agent-sdk/overview) per ricevere ogni messaggio a livello di programmazione.

L'esempio di seguito utilizza la seconda interfaccia. Invia un prompt di follow-up a una sessione esistente e legge la risposta con `jq`:

```bash theme={null}
claude -p --resume <session-id> --output-format json "summarize what we changed" | jq -r '.result'
```

<h3 id="where-transcripts-are-stored">
  Dove vengono archiviati i trascritti
</h3>

Per impostazione predefinita, i trascritti vengono archiviati come JSONL in `~/.claude/projects/<project>/<session-id>.jsonl`, dove `<project>` è il percorso della directory di lavoro con caratteri non alfanumerici sostituiti da `-`. Ogni riga è un oggetto JSON per un messaggio, uso dello strumento o voce di metadati. Il formato della voce è interno a Claude Code e cambia tra le versioni, quindi gli script che analizzano direttamente questi file possono interrompersi in qualsiasi rilascio. Per costruire sui dati della sessione, utilizza `/export` o le [interfacce di script](#access-conversations-from-scripts) invece.

La posizione, la conservazione e il comportamento di scrittura sono configurabili:

| Per                                                        | Imposta                                                | Dove                     |
| ---------------------------------------------------------- | ------------------------------------------------------ | ------------------------ |
| Sposta l'archiviazione da `~/.claude`                      | [`CLAUDE_CONFIG_DIR`](/docs/it/env-vars)                    | Variabile di ambiente    |
| Cambia la conservazione di 30 giorni                       | [`cleanupPeriodDays`](/docs/it/settings#available-settings) | `settings.json`          |
| Sopprimere le scritture di trascritto in tutte le modalità | [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/it/env-vars)      | Variabile di ambiente    |
| Sopprimere le scritture per un'esecuzione non interattiva  | [`--no-session-persistence`](/docs/it/cli-reference)        | Flag CLI con `claude -p` |

<h2 id="see-also">
  Vedi anche
</h2>

Queste pagine coprono la meccanica correlata delle sessioni e del parallelismo:

* [Worktrees](/docs/it/worktrees): esegui sessioni parallele isolate su rami separati
* [Checkpointing](/docs/it/checkpointing): riavvolgi il codice e la conversazione a un punto precedente
* [Context window](/docs/it/context-window): cosa riempie il contesto e cosa sopravvive alla compattazione
* [Non-interactive mode](/docs/it/headless): comportamento della sessione in `claude -p`
