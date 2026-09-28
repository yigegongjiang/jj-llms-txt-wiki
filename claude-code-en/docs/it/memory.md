> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Come Claude ricorda il tuo progetto

> Fornisci a Claude istruzioni persistenti con file CLAUDE.md o AGENTS.md e lascia che Claude accumuli apprendimenti automaticamente con la memoria automatica.

Ogni sessione di Claude Code inizia con una finestra di contesto nuova. Due meccanismi trasportano la conoscenza tra le sessioni:

* **File CLAUDE.md**: istruzioni che scrivi per dare a Claude un contesto persistente. Claude può anche leggere i file [`AGENTS.md`](#agents-md) di un repository, da soli o insieme a CLAUDE.md
* **Memoria automatica**: note che Claude scrive da solo in base alle tue correzioni e preferenze

Questa pagina spiega come:

* [Scrivere e organizzare file CLAUDE.md](#claude-md-files)
* [Utilizzare un file AGENTS.md esistente](#agents-md) come istruzioni del tuo progetto, da solo o insieme a CLAUDE.md
* [Limitare le regole a tipi di file specifici](#organize-rules-with-claude/rules/) con `.claude/rules/`
* [Configurare la memoria automatica](#auto-memory) in modo che Claude prenda note automaticamente
* [Risolvere i problemi](#troubleshoot-memory-issues) quando le istruzioni non vengono seguite

<h2 id="claude-md-vs-auto-memory">
  CLAUDE.md vs memoria automatica
</h2>

Claude Code ha due sistemi di memoria complementari. Entrambi vengono caricati all'inizio di ogni conversazione. Claude li tratta come contesto, non come configurazione forzata. Per bloccare un'azione indipendentemente da ciò che Claude decide, utilizza un [hook PreToolUse](/docs/it/hooks-guide) invece. Più specifiche e concise sono le tue istruzioni, più coerentemente Claude le segue.

|                   | File CLAUDE.md                                                    | Memoria automatica                                                                                                 |
| :---------------- | :---------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------- |
| **Chi lo scrive** | Tu                                                                | Claude                                                                                                             |
| **Cosa contiene** | Istruzioni e regole                                               | Apprendimenti e modelli                                                                                            |
| **Ambito**        | Progetto, utente o organizzazione                                 | Per repository, condiviso tra worktrees                                                                            |
| **Caricato in**   | Ogni sessione                                                     | Ogni sessione (prime 200 righe o 25KB)                                                                             |
| **Usare per**     | Standard di codifica, flussi di lavoro, architettura del progetto | Le tue preferenze, le correzioni che dai a Claude, il contesto del progetto che Claude non può derivare dal codice |

Usa file CLAUDE.md quando vuoi guidare il comportamento di Claude. La memoria automatica consente a Claude di imparare dalle tue correzioni senza sforzo manuale.

I subagents possono anche mantenere la propria memoria automatica. Vedi [configurazione subagent](/docs/it/sub-agents#enable-persistent-memory) per i dettagli.

<h2 id="claude-md-files">
  File CLAUDE.md
</h2>

I file CLAUDE.md sono file markdown che forniscono a Claude istruzioni persistenti per un progetto, il vostro flusso di lavoro personale o l'intera organizzazione. Scrivete questi file in testo semplice; Claude li legge all'inizio di ogni sessione. Se il vostro repository utilizza `AGENTS.md` invece, consultate [AGENTS.md](#agents-md).

<h3 id="when-to-add-to-claude-md">
  Quando aggiungere a CLAUDE.md
</h3>

Trattate CLAUDE.md come il luogo in cui scrivete ciò che altrimenti dovreste rispiegare. Aggiungete a esso quando:

* Claude commette lo stesso errore una seconda volta
* Una revisione del codice rileva qualcosa che Claude avrebbe dovuto sapere su questo codebase
* Digitate la stessa correzione o chiarimento nella chat che avete digitato nella sessione precedente
* Un nuovo membro del team avrebbe bisogno dello stesso contesto per essere produttivo

Mantenetelo limitato ai fatti che Claude dovrebbe ricordare in ogni sessione: comandi di build, convenzioni, layout del progetto, regole "fai sempre X". Se una voce è una procedura multi-step o riguarda solo una parte del codebase, spostatela in una [skill](/docs/it/skills) o in una [regola con ambito di percorso](#organize-rules-with-claude/rules/) invece. La [panoramica dell'estensione](/docs/it/features-overview#build-your-setup-over-time) copre quando utilizzare ogni meccanismo.

<h3 id="choose-where-to-put-claude-md-files">
  Scegliete dove posizionare i file CLAUDE.md
</h3>

I file CLAUDE.md possono trovarsi in diversi percorsi, ognuno con un ambito diverso. La tabella seguente li elenca in ordine di caricamento, dall'ambito più ampio al più specifico, quindi un'istruzione di progetto appare in contesto dopo un'istruzione utente.

| Ambito                     | Percorso                                                                                                                                                              | Scopo                                                                   | Esempi di casi d'uso                                                            | Condiviso con                                         |
| -------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------- | ------------------------------------------------------------------------------- | ----------------------------------------------------- |
| **Politica gestita**       | • macOS: `/Library/Application Support/ClaudeCode/CLAUDE.md`<br />• Linux e WSL: `/etc/claude-code/CLAUDE.md`<br />• Windows: `C:\Program Files\ClaudeCode\CLAUDE.md` | Istruzioni a livello organizzativo gestite da IT/DevOps                 | Standard di codifica aziendale, politiche di sicurezza, requisiti di conformità | Tutti gli utenti dell'organizzazione                  |
| **Istruzioni utente**      | `~/.claude/CLAUDE.md`                                                                                                                                                 | Preferenze personali per tutti i progetti                               | Preferenze di stile del codice, scorciatoie di strumenti personali              | Solo voi (tutti i progetti)                           |
| **Istruzioni di progetto** | `./CLAUDE.md` o `./.claude/CLAUDE.md`. Consultate [AGENTS.md](#agents-md) per quando `./AGENTS.md` si carica invece o insieme a essi                                  | Istruzioni condivise dal team per il progetto                           | Architettura del progetto, standard di codifica, flussi di lavoro comuni        | Membri del team tramite controllo del codice sorgente |
| **Istruzioni locali**      | `./CLAUDE.local.md`                                                                                                                                                   | Preferenze personali specifiche del progetto; aggiungete a `.gitignore` | I vostri URL sandbox, dati di test preferiti                                    | Solo voi (progetto corrente)                          |

I file CLAUDE.md e CLAUDE.local.md nella gerarchia di directory sopra la directory di lavoro vengono caricati all'avvio. I file nelle sottodirectory si caricano su richiesta quando Claude legge i file in quelle directory. Consultate [Come si caricano i file CLAUDE.md](#how-claude-md-files-load) per l'ordine di risoluzione completo.

Per i progetti di grandi dimensioni, potete suddividere le istruzioni in file specifici per argomento utilizzando [regole di progetto](#organize-rules-with-claude/rules/). Le regole vi permettono di limitare le istruzioni a tipi di file specifici o sottodirectory.

<h3 id="set-up-a-project-claude-md">
  Configurate un CLAUDE.md di progetto
</h3>

Un CLAUDE.md di progetto può essere archiviato in `./CLAUDE.md` o `./.claude/CLAUDE.md`. Create questo file e aggiungete istruzioni che si applicano a chiunque lavori sul progetto: comandi di build e test, standard di codifica, decisioni architettoniche, convenzioni di denominazione e flussi di lavoro comuni. Queste istruzioni sono condivise con il vostro team tramite controllo del codice sorgente, quindi concentratevi su standard a livello di progetto piuttosto che su preferenze personali. Per confermare che il file è stato caricato, eseguite `/context` in una sessione e controllate l'elenco sotto **Memory files**.

<Tip>
  Eseguite `/init` per generare automaticamente un CLAUDE.md iniziale. Claude analizza il vostro codebase e crea un file con comandi di build, istruzioni di test e convenzioni di progetto che scopre. Se un CLAUDE.md esiste già, `/init` suggerisce miglioramenti piuttosto che sovrascriverlo. Perfezionate da lì con istruzioni che Claude non scoprirebbe da solo.

  Per un flusso interattivo multi-fase invece, impostate la variabile di ambiente `CLAUDE_CODE_NEW_INIT` a `1` prima di eseguire `/init`. Impostatela nella vostra shell o nel blocco `env` di un file di impostazioni, come mostrato in [Impostare le variabili di ambiente](/docs/it/env-vars#set-environment-variables). Con essa impostata, `/init` chiede quali artefatti configurare: file CLAUDE.md, skills e hooks. Quindi esplora il vostro codebase con un subagent, colma le lacune tramite domande di follow-up e presenta una proposta revisionabile prima di scrivere qualsiasi file. La variabile cambia solo il modo in cui `/init` viene eseguito, quindi potete lasciarla impostata.
</Tip>

<h3 id="write-effective-instructions">
  Scrivete istruzioni efficaci
</h3>

I file CLAUDE.md vengono caricati nella finestra di contesto all'inizio di ogni sessione, consumando token insieme alla vostra conversazione. La [visualizzazione della finestra di contesto](/docs/it/context-window) mostra dove CLAUDE.md si carica rispetto al resto del contesto di avvio. Poiché sono contesto piuttosto che configurazione applicata, il modo in cui scrivete le istruzioni influisce su quanto affidabilmente Claude le segue. Le istruzioni specifiche, concise e ben strutturate funzionano meglio.

**Dimensione**: mirate a meno di 200 righe per file CLAUDE.md. I file più lunghi consumano più contesto e riducono l'aderenza. Se le vostre istruzioni stanno crescendo molto, utilizzate [regole con ambito di percorso](#path-specific-rules) in modo che le istruzioni si carichino solo quando Claude lavora con file corrispondenti. Potete anche dividere il contenuto in [importazioni](#import-additional-files) per l'organizzazione, anche se i file importati si caricano comunque e entrano nella finestra di contesto all'avvio.

**Struttura**: utilizzate intestazioni markdown e punti elenco per raggruppare le istruzioni correlate. Claude scansiona la struttura nello stesso modo in cui lo fanno i lettori: le sezioni organizzate sono più facili da seguire rispetto ai paragrafi densi.

**Specificità**: scrivete istruzioni abbastanza concrete da poter verificare. Ad esempio:

* "Utilizzate l'indentazione a 2 spazi" invece di "Formattate il codice correttamente"
* "Eseguite `npm test` prima di eseguire il commit" invece di "Testate i vostri cambiamenti"
* "I gestori API si trovano in `src/api/handlers/`" invece di "Mantenete i file organizzati"

**Coerenza**: se due regole si contraddicono a vicenda, Claude potrebbe sceglierne una arbitrariamente. Rivedete periodicamente i vostri file CLAUDE.md, i file CLAUDE.md annidati nelle sottodirectory e [`.claude/rules/`](#organize-rules-with-claude/rules/) per rimuovere istruzioni obsolete o conflittuali. Nei monorepo, utilizzate [`claudeMdExcludes`](#exclude-specific-claude-md-files) per saltare i file CLAUDE.md di altri team che non sono rilevanti per il vostro lavoro.

<h3 id="import-additional-files">
  Importate file aggiuntivi
</h3>

I file CLAUDE.md possono importare file aggiuntivi utilizzando la sintassi `@path/to/import`. I file importati vengono espansi e caricati in contesto all'avvio insieme al CLAUDE.md che li riferisce.

Sono consentiti sia i percorsi relativi che assoluti. I percorsi relativi si risolvono rispetto al file che contiene l'importazione, non alla directory di lavoro. I file importati possono importare ricorsivamente altri file, con una profondità massima di quattro hop.

L'analisi dell'importazione salta gli intervalli di codice Markdown e i blocchi di codice recintati. Per menzionare un percorso nel vostro CLAUDE.md senza importarlo, avvolgetelo in backtick: scrivere `` `@README` `` mantiene il testo letterale, mentre `@README` al di fuori dei backtick importa il file.

Per includere un README, package.json e una guida al flusso di lavoro, fate riferimento a essi con la sintassi `@` in qualsiasi punto del vostro CLAUDE.md:

```text theme={null}
Consultate @README per la panoramica del progetto e @package.json per i comandi npm disponibili per questo progetto.

# Istruzioni aggiuntive
- flusso di lavoro git @docs/git-instructions.md
```

Per le preferenze personali per progetto privato che non dovrebbero essere archiviate nel controllo del codice sorgente, create un `CLAUDE.local.md` nella radice del progetto. Si carica insieme a `CLAUDE.md` e viene trattato allo stesso modo. Aggiungete `CLAUDE.local.md` al vostro `.gitignore` in modo che non venga eseguito il commit. Con `CLAUDE_CODE_NEW_INIT=1` impostato, l'esecuzione di `/init` e la scelta dell'opzione personale lo fa per voi.

Se lavorate su più git worktrees dello stesso repository, un `CLAUDE.local.md` ignorato da git esiste solo nel worktree in cui lo avete creato. Per condividere istruzioni personali tra worktrees, importate invece un file dalla vostra home directory:

```text theme={null}
# Preferenze individuali
- @~/.claude/my-project-instructions.md
```

<Warning>
  Un'importazione in un file di memoria a livello di progetto è esterna quando il suo percorso si risolve al di fuori della vostra directory di lavoro, come l'importazione della home directory sopra. La prima volta che Claude Code incontra importazioni esterne in un progetto, mostra una finestra di dialogo di approvazione che elenca i file. Se rifiutate, le importazioni rimangono disabilitate e la finestra di dialogo non appare più.

  Claude Code mostra la finestra di dialogo per proteggervi dai file che altre persone eseguono il commit in un progetto condiviso. I file di memoria con ambito utente, come `~/.claude/CLAUDE.md` e `~/.claude/rules/`, sono file che avete scritto voi stessi. Tranne nelle sessioni [Cowork](https://claude.com/product/cowork) sul vostro desktop, Claude Code carica le loro importazioni senza la finestra di dialogo e le considera attendibili come il resto della vostra configurazione personale.

  Nelle sessioni Cowork sul vostro desktop, Claude Code salta qualsiasi importazione in un file con ambito utente che si risolve a un percorso al di fuori della directory di lavoro della sessione e carica il resto del file. In quelle sessioni salta anche un `~/.claude/CLAUDE.md` che è esso stesso un symlink o un hard link, e una directory `~/.claude/rules/` symlinked o un file di regola che punta al di fuori della directory di lavoro.
</Warning>

<h3 id="how-claude-md-files-load">
  Come si caricano i file CLAUDE.md
</h3>

Claude Code carica `CLAUDE.md` e `CLAUDE.local.md` dalla vostra directory di lavoro corrente e da ogni directory sopra di essa. Eseguite Claude Code in `foo/bar/` e carica le istruzioni da `foo/bar/CLAUDE.md`, `foo/CLAUDE.md` e da qualsiasi file `CLAUDE.local.md` accanto a essi.

Tutti i file scoperti vengono concatenati in contesto piuttosto che sovrascriversi a vicenda. Nell'albero delle directory, il contenuto è ordinato dalla radice del filesystem fino alla vostra directory di lavoro. Per l'esempio `foo/bar/`, `foo/CLAUDE.md` appare in contesto prima di `foo/bar/CLAUDE.md`, quindi le istruzioni più vicine a dove avete lanciato Claude vengono lette per ultime. All'interno di ogni directory, `CLAUDE.local.md` viene aggiunto dopo `CLAUDE.md`, quindi le vostre note personali sono l'ultima cosa che Claude legge a quel livello.

Claude scopre anche i file `CLAUDE.md` e `CLAUDE.local.md` nelle sottodirectory sotto la vostra directory di lavoro corrente. Invece di caricarli all'avvio, vengono inclusi quando Claude legge i file in quelle sottodirectory.

Se lavorate in un grande monorepo in cui i file CLAUDE.md di altri team vengono raccolti, utilizzate [`claudeMdExcludes`](#exclude-specific-claude-md-files) per saltarli. Per il layout completo dei file CLAUDE.md e delle regole a livello di radice e per directory, consultate [Monorepo e repository di grandi dimensioni](/docs/it/large-codebases).

I commenti HTML a livello di blocco (`<!-- maintainer notes -->`) nei file CLAUDE.md vengono rimossi prima che il contenuto venga iniettato nel contesto di Claude. Utilizzateli per lasciare note per i manutentori umani senza spendere token di contesto su di essi. I commenti all'interno dei blocchi di codice vengono preservati. Quando aprite un file CLAUDE.md direttamente con lo strumento Read, i commenti rimangono visibili.

<h4 id="load-from-additional-directories">
  Caricamento da directory aggiuntive
</h4>

Il flag `--add-dir` dà a Claude accesso a directory aggiuntive al di fuori della vostra directory di lavoro principale. Per impostazione predefinita, i file CLAUDE.md da queste directory non vengono caricati.

Per caricare anche i file di memoria da directory aggiuntive, impostate la variabile di ambiente `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD`:

```bash theme={null}
CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1 claude --add-dir ../shared-config
```

La forma inline imposta la variabile per quel singolo lancio in Bash o Zsh. Per mantenerla attiva per ogni sessione, aggiungetela al blocco `env` in `~/.claude/settings.json` come mostrato in [Impostare le variabili di ambiente](/docs/it/env-vars#set-environment-variables).

Questo carica `CLAUDE.md`, `.claude/CLAUDE.md`, `.claude/rules/*.md` e `CLAUDE.local.md` dalla directory aggiuntiva. `CLAUDE.local.md` viene saltato se escludete `local` da [`--setting-sources`](/docs/it/cli-reference).

<h3 id="organize-rules-with-claude/rules/">
  Organizzate le regole con `.claude/rules/`
</h3>

Per i progetti più grandi, potete organizzare le istruzioni in più file utilizzando la directory `.claude/rules/`. Questo mantiene le istruzioni modulari e più facili da mantenere per i team. Le regole possono anche essere [limitate a percorsi di file specifici](#path-specific-rules), quindi si caricano in contesto solo quando Claude lavora con file corrispondenti, riducendo il rumore e risparmiando spazio di contesto.

<Note>
  Le regole si caricano in contesto ogni sessione o quando vengono aperti i file corrispondenti. Per le istruzioni specifiche dell'attività che non devono essere in contesto tutto il tempo, utilizzate [skills](/docs/it/skills) invece, che si caricano solo quando le richiamate o quando Claude determina che sono rilevanti per il vostro prompt.
</Note>

<h4 id="set-up-rules">
  Configurate le regole
</h4>

Posizionate i file markdown nella directory `.claude/rules/` del vostro progetto. Ogni file dovrebbe coprire un argomento, con un nome di file descrittivo come `testing.md` o `api-design.md`. Tutti i file `.md` vengono scoperti ricorsivamente, quindi potete organizzare le regole in sottodirectory come `frontend/` o `backend/`:

```text theme={null}
your-project/
├── .claude/
│   ├── CLAUDE.md           # Istruzioni principali del progetto
│   └── rules/
│       ├── code-style.md   # Linee guida di stile del codice
│       ├── testing.md      # Convenzioni di test
│       └── security.md     # Requisiti di sicurezza
```

Le regole senza [frontmatter `paths`](#path-specific-rules) vengono caricate all'avvio con la stessa priorità di `.claude/CLAUDE.md`.

Le regole di progetto vengono saltate se escludete `project` da [`--setting-sources`](/docs/it/cli-reference). Prima della v2.1.211, le regole che si caricano su richiesta, incluse le regole con ambito di percorso e le regole nelle directory `.claude/rules/` annidate, si caricavano anche quando `project` era escluso.

<h4 id="path-specific-rules">
  Regole specifiche del percorso
</h4>

Le regole possono essere limitate a file specifici utilizzando il frontmatter YAML con il campo `paths`. Queste regole condizionali si applicano solo quando Claude lavora con file che corrispondono ai modelli specificati.

```markdown theme={null}
---
paths:
  - "src/api/**/*.ts"
---

# Regole di sviluppo API

- Tutti gli endpoint API devono includere la convalida dell'input
- Utilizzate il formato di risposta di errore standard
- Includete commenti di documentazione OpenAPI
```

Le regole senza un campo `paths` vengono caricate incondizionatamente e si applicano a tutti i file. Le regole con ambito di percorso si attivano quando Claude legge file che corrispondono al modello, non ad ogni utilizzo dello strumento. A partire dalla v2.1.198, la corrispondenza funziona anche quando Claude raggiunge un file attraverso un percorso symlinked alla directory del progetto, ad esempio in un checkout symlinked.

Utilizzate i modelli glob nel campo `paths` per far corrispondere i file per estensione, directory o qualsiasi combinazione:

| Modello                | Corrisponde a                                  |
| ---------------------- | ---------------------------------------------- |
| `**/*.ts`              | Tutti i file TypeScript in qualsiasi directory |
| `src/**/*`             | Tutti i file sotto la directory `src/`         |
| `*.md`                 | File Markdown nella radice del progetto        |
| `src/components/*.tsx` | Componenti React in una directory specifica    |

Potete specificare più modelli e utilizzare l'espansione tra parentesi graffe per far corrispondere più estensioni in un modello:

```markdown theme={null}
---
paths:
  - "src/**/*.{ts,tsx}"
  - "lib/**/*.ts"
  - "tests/**/*.test.ts"
---
```

Ogni gruppo tra parentesi graffe moltiplica il numero di modelli espansi: `src/*.{ts,tsx}` si espande a due modelli, e `{a,b}/{c,d}/*.{ts,tsx}` a otto. Per mantenere l'espansione limitata, l'intero elenco `paths` di una regola condivide un budget di 1.000 modelli espansi e 4 MiB, e i modelli senza parentesi graffe non contano contro di esso.

Claude Code utilizza qualsiasi modello che supererebbe il budget non espanso, e le sue parentesi graffe letterali non corrispondono a nessun file. Prima della v2.1.217, un valore `paths` con molti gruppi tra parentesi graffe bloccava o causava l'arresto anomalo della CLI all'avvio.

La sintassi Glob tratta `[` come l'inizio di un'espressione tra parentesi quadre come `[abc]`. Un modello con un `[` che non può essere letto come un'espressione tra parentesi quadre, come `photos [2024/**`, non è valido: non corrisponde a nulla, e gli altri modelli della regola continuano a funzionare. Per far corrispondere un `[` letterale in un nome di file, sfuggitelo come `photos \[2024/**`. Prima della v2.1.207, un modello non valido causava il fallimento dello strumento Read per ogni file su cui la regola veniva valutata, invece di non corrispondere a nulla.

<h4 id="rules-frontmatter-reference">
  Riferimento frontmatter delle regole
</h4>

Configurate una regola con il [frontmatter](/docs/it/glossary#frontmatter) YAML tra i marcatori `---` all'inizio del file. `paths` è l'unico campo che Claude Code legge da una regola; qualsiasi altro campo viene ignorato senza errore. Claude Code rimuove il frontmatter prima di caricare la regola in contesto.

| Campo   | Obbligatorio | Descrizione                                                                                                                                  |
| :------ | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------- |
| `paths` | No           | Modelli glob che [limitano la regola ai file corrispondenti](#path-specific-rules). Accetta un elenco YAML o una stringa separata da virgole |

Se lo YAML tra i marcatori non viene analizzato, Claude Code ignora il frontmatter e carica la regola come se non avesse `paths`. Eseguite `claude --debug` per vedere l'errore di analisi.

<h4 id="share-rules-across-projects-with-symlinks">
  Condividete le regole tra i progetti con symlink
</h4>

La directory `.claude/rules/` supporta symlink, quindi potete mantenere un set di regole condivise e collegarle in più progetti. I symlink circolari vengono rilevati e gestiti correttamente.

Claude Code tratta un symlink il cui target è al di fuori della vostra directory di lavoro come un'[importazione esterna](#import-additional-files). Le regole collegate non si caricano finché non approvate le importazioni esterne per il progetto, e dopo di che solo quelle senza un [campo `paths`](#path-specific-rules) si caricano. Claude Code chiede quell'approvazione solo quando un file di memoria di progetto importa un file al di fuori della directory di lavoro con `@path`, non per i symlink da soli. Per caricare le regole condivise senza quell'approvazione, mantenetele in [`~/.claude/rules/`](#user-level-rules), dove si applicano a ogni progetto sulla vostra macchina.

Questo esempio collega sia una directory condivisa che un file individuale:

```bash theme={null}
ln -s ~/shared-claude-rules .claude/rules/shared
ln -s ~/company-standards/security.md .claude/rules/security.md
```

<h4 id="user-level-rules">
  Regole a livello di utente
</h4>

Le regole personali in `~/.claude/rules/` si applicano a ogni progetto sulla vostra macchina. Utilizzatele per le preferenze che non sono specifiche del progetto:

```text theme={null}
~/.claude/rules/
├── preferences.md    # Le vostre preferenze di codifica personali
└── workflows.md      # I vostri flussi di lavoro preferiti
```

Claude Code carica le regole a livello di utente prima delle regole di progetto, quindi una regola di progetto appare più tardi nel contesto di Claude rispetto a una regola utente. Nessuno dei due set sostituisce l'altro: se una regola utente e una regola di progetto si contraddicono, Claude potrebbe seguire l'una o l'altra, quindi mantenete i due coerenti.

<h3 id="manage-claude-md-for-large-teams">
  Gestite CLAUDE.md per i team di grandi dimensioni
</h3>

Per le organizzazioni che distribuiscono Claude Code tra i team, potete centralizzare le istruzioni e controllare quali file CLAUDE.md vengono caricati.

<h4 id="deploy-organization-wide-claude-md">
  Distribuite CLAUDE.md a livello organizzativo
</h4>

Le organizzazioni possono distribuire un CLAUDE.md gestito centralmente che si applica a tutti gli utenti su una macchina. Questo file non può essere escluso dalle impostazioni individuali.

<Steps>
  <Step title="Create il file nel percorso della politica gestita">
    * macOS: `/Library/Application Support/ClaudeCode/CLAUDE.md`
    * Linux e WSL: `/etc/claude-code/CLAUDE.md`
    * Windows: `C:\Program Files\ClaudeCode\CLAUDE.md`
  </Step>

  <Step title="Distribuite con il vostro sistema di gestione della configurazione">
    Utilizzate MDM, Group Policy, Ansible o strumenti simili per distribuire il file tra le macchine degli sviluppatori. Consultate [impostazioni gestite](/docs/it/managed-settings) per altre opzioni di configurazione a livello organizzativo.
  </Step>
</Steps>

La chiave `claudeMd` vi permette di inserire il contenuto CLAUDE.md gestito direttamente all'interno di `managed-settings.json` invece di distribuire un file separato.

**Ambito**: ogni sessione di Claude Code sulla macchina, in ogni repository. Per la guida specifica del repository, eseguite il commit di un CLAUDE.md di progetto invece.

**Precedenza**: uguale a un file CLAUDE.md gestito. Si carica prima di CLAUDE.md utente e progetto.

**Dove viene rispettato**: solo impostazioni gestite e di politica. L'impostazione di `claudeMd` nelle impostazioni utente, progetto o locali non ha effetto.

L'esempio seguente aggiunge istruzioni comportamentali direttamente in un file di impostazioni gestite:

```json theme={null}
{
  "claudeMd": "Eseguite sempre `make lint` prima di eseguire il commit.\nNon eseguite mai il push direttamente su main."
}
```

Un CLAUDE.md gestito e [impostazioni gestite](/docs/it/managed-settings) servono a scopi diversi. Utilizzate le impostazioni per l'applicazione tecnica e CLAUDE.md per la guida comportamentale:

| Preoccupazione                                           | Configurate in                                                |
| :------------------------------------------------------- | :------------------------------------------------------------ |
| Bloccate strumenti, comandi o percorsi di file specifici | Impostazioni gestite: `permissions.deny`                      |
| Applicate l'isolamento sandbox                           | Impostazioni gestite: `sandbox.enabled`                       |
| Variabili di ambiente e routing del provider API         | Impostazioni gestite: `env`                                   |
| Metodo di accesso e restrizioni organizzative            | Impostazioni gestite: `forceLoginMethod`, `forceLoginOrgUUID` |
| Linee guida di stile del codice e qualità                | CLAUDE.md gestito                                             |
| Promemoria sulla gestione dei dati e conformità          | CLAUDE.md gestito                                             |
| Istruzioni comportamentali per Claude                    | CLAUDE.md gestito                                             |

Le regole delle impostazioni vengono applicate dal client indipendentemente da ciò che Claude decide di fare. Le istruzioni CLAUDE.md modellano il comportamento di Claude ma non sono un livello di applicazione rigido.

<h4 id="exclude-specific-claude-md-files">
  Escludete file CLAUDE.md specifici
</h4>

Nei grandi monorepo, i file CLAUDE.md antenati possono contenere istruzioni che non sono rilevanti per il vostro lavoro. L'impostazione `claudeMdExcludes` vi permette di saltare file specifici per percorso o modello glob.

Questo esempio esclude un CLAUDE.md di livello superiore e una directory di regole da una cartella padre. Aggiungitelo a `.claude/settings.local.json` in modo che l'esclusione rimanga locale alla vostra macchina:

```json theme={null}
{
  "claudeMdExcludes": [
    "**/monorepo/CLAUDE.md",
    "/home/user/monorepo/other-team/.claude/rules/**"
  ]
}
```

I modelli vengono confrontati con i percorsi di file assoluti utilizzando la sintassi glob. Potete configurare `claudeMdExcludes` a qualsiasi [livello di impostazioni](/docs/it/settings#where-settings-live): utente, progetto, locale o politica gestita. Gli array si uniscono tra i livelli.

Per escludere un file di regole che raggiungete attraverso un [symlink](#share-rules-across-projects-with-symlinks), sia che il file o la sua directory sia il link, scrivete il modello rispetto a uno dei due percorsi: il percorso del file sotto `.claude/rules/` o il suo target di link. Un modello che corrisponde a uno dei due percorsi esclude il file. Prima della v2.1.239, solo un modello che corrispondeva al target del link escludeva il file.

I file CLAUDE.md della politica gestita non possono essere esclusi. Questo garantisce che le istruzioni a livello organizzativo si applichino sempre indipendentemente dalle impostazioni individuali.

<h2 id="agents-md">
  AGENTS.md
</h2>

Claude Code può leggere [`AGENTS.md`](/docs/it/glossary#agents-md) come istruzioni del vostro progetto, quindi un repository già configurato per altri agenti di codifica funziona senza aggiungere un `CLAUDE.md`, un import o un'impostazione. Questa tabella mostra cosa Claude legge per impostazione predefinita per ogni combinazione di file di istruzioni nel vostro repository:

| Il vostro repository ha                                                                                   | Claude legge                                                       |
| :-------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------- |
| Un `AGENTS.md`, e nessun `CLAUDE.md` o `CLAUDE.local.md` nella vostra directory di lavoro o sopra di essa | Il vostro `AGENTS.md`                                              |
| Un `AGENTS.md` e un `CLAUDE.md` o `CLAUDE.local.md` nella vostra directory di lavoro o sopra di essa      | Solo i vostri file `CLAUDE.md`                                     |
| Un `CLAUDE.md` che già [importa `AGENTS.md`](#share-one-file-with-other-coding-tools)                     | Il vostro `CLAUDE.md`, con `AGENTS.md` incluso attraverso l'import |

Per cambiare il valore predefinito, ad esempio per fare in modo che Claude legga sempre entrambi i file, legga solo `CLAUDE.md`, o legga solo le istruzioni gestite della vostra organizzazione, [cambiate l'impostazione **Project instructions**](#choose-which-instruction-files-load).

<Note>
  La lettura diretta di `AGENTS.md` richiede Claude Code v2.1.277 o successivo. In alcune sessioni Claude [non può leggere `AGENTS.md`](#when-agents-md-support-is-unavailable), quindi [importatelo da un `CLAUDE.md`](#share-one-file-with-other-coding-tools) lì invece.
</Note>

<h3 id="when-claude-code-reads-agents-md">
  When Claude Code reads AGENTS.md
</h3>

Per impostazione predefinita, Claude legge `AGENTS.md` solo quando non avete nessun `CLAUDE.md` nella vostra directory di lavoro o sopra di essa. Ecco quali dei vostri file contano per quel controllo:

* **Contano, quindi Claude li legge invece di `AGENTS.md`**: un `CLAUDE.md`, `.claude/CLAUDE.md`, o `CLAUDE.local.md` nella vostra directory di lavoro o in qualsiasi directory sopra di essa
* **Non contano, e continuano a caricarsi insieme a `AGENTS.md`**: il vostro `~/.claude/CLAUDE.md`, il `CLAUDE.md` gestito della vostra organizzazione, e i vostri file `.claude/rules/`

Quando nessuno conta, ecco cosa Claude legge e come potete dirlo:

* **All'inizio della sessione**: ogni `AGENTS.md` e `.claude/AGENTS.md` nella vostra directory di lavoro e nelle directory sopra di essa. In una sessione interattiva vedete una riga come `no CLAUDE.md found; AGENTS.md loaded: /home/you/repo/AGENTS.md` nella conversazione
* **Mentre Claude lavora nelle sottodirectory**: un `AGENTS.md` di una sottodirectory, quando Claude apre un file lì con lo strumento Read e quella sottodirectory non ha nessuno dei tre file `CLAUDE.md` propri
* **All'interno di ogni `AGENTS.md`**: gli import [`@path`](#import-additional-files) sono espansi, i pattern [`claudeMdExcludes`](#exclude-specific-claude-md-files) si applicano, e i subagent che [saltano le istruzioni del progetto](/docs/it/sub-agents#what-loads-at-startup) saltano anche questi file
* **Non letto**: `AGENTS.local.md`, `AGENTS.override.md`, o qualsiasi cosa sotto una directory `.agents/`

<Note>
  Poiché `CLAUDE.local.md` conta, aggiungerne uno per mantenere le vostre istruzioni non committate in un progetto che si basa su `AGENTS.md` impedisce a Claude di leggere `AGENTS.md` per voi. Per mantenere il vostro `CLAUDE.local.md` e avere comunque Claude che legge `AGENTS.md`, impostate **Project instructions** su [`claude-md-and-agents-md`](#choose-which-instruction-files-load).
</Note>

<h3 id="choose-which-instruction-files-load">
  Choose which instruction files load
</h3>

Per cambiare quali file Claude legge, digitate `/config` in una sessione di Claude Code per aprire il pannello delle impostazioni, quindi impostate **Project instructions** su uno di questi valori:

| Value                     | What Claude reads                                                                                                                                                                                                                                                                                                                                                                                |
| :------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `claude-md-or-agents-md`  | I vostri file `CLAUDE.md`, o i vostri file `AGENTS.md` quando non avete nessun `CLAUDE.md` o `CLAUDE.local.md` nella vostra directory di lavoro o sopra di essa. Questo è il valore predefinito                                                                                                                                                                                                  |
| `claude-md-and-agents-md` | I vostri file `CLAUDE.md` e `AGENTS.md` insieme, ogni `CLAUDE.md` della directory per primo e il suo `AGENTS.md` dopo. Claude Code salta un `AGENTS.md` che ha già caricato, quindi uno che il vostro `CLAUDE.md` importa o crea un symlink non viene letto due volte                                                                                                                            |
| `claude-md`               | Solo i vostri file `CLAUDE.md`                                                                                                                                                                                                                                                                                                                                                                   |
| `managed-only`            | Solo il `CLAUDE.md` gestito della vostra organizzazione e [auto memory](#auto-memory) all'avvio. I vostri file `CLAUDE.md` del progetto, locale e utente, i vostri file `.claude/rules/`, e ogni `AGENTS.md` sono esclusi. Il `CLAUDE.md` e i file `.claude/rules/` di una sottodirectory, e le [path-scoped rules](#path-specific-rules), continuano a caricarsi quando Claude legge un file lì |

Potete anche impostare il valore in un file di impostazioni invece di `/config`. Aggiungetelo sotto l'ID del plugin `agents-md` integrato in [`pluginConfigs`](/docs/it/settings-reference#pluginconfigs), in `~/.claude/settings.json`, un file `--settings`, o [managed settings](/docs/it/managed-settings). Claude Code lo ignora nei file di impostazioni del progetto e locale. Questo esempio fa leggere a Claude entrambi i file:

```json settings.json theme={null}
{
  "pluginConfigs": {
    "agents-md@builtin": {
      "options": { "instructionFiles": "claude-md-and-agents-md" }
    }
  }
}
```

Il vostro cambiamento si applica dal prossimo messaggio che inviate e in ogni nuova sessione.

<h3 id="when-agents-md-support-is-unavailable">
  When AGENTS.md support is unavailable
</h3>

In queste sessioni Claude legge solo i file `CLAUDE.md`, e **Project instructions** non appare nel pannello delle impostazioni `/config`:

* Siete su una versione di Claude Code precedente a v2.1.277
* Voi o la vostra organizzazione avete disabilitato il plugin integrato `agents-md` in `/plugin`
* In alcuni casi, è la vostra [prima sessione dopo aver aggiornato](/docs/it/env-vars#first-session-after-an-install-or-upgrade) da v2.1.276 o precedente. Claude legge `AGENTS.md` dalla vostra prossima sessione

Prima di v2.1.281, alcune sessioni, come quelle su Amazon Bedrock o con telemetria disabilitata, leggevano solo i file `CLAUDE.md`. Su quelle versioni, aggiornate Claude Code. Per dare a Claude il vostro `AGENTS.md` in una qualsiasi di queste sessioni, [importatelo da un `CLAUDE.md`](#share-one-file-with-other-coding-tools).

<h3 id="where-agents-md-differs-from-claude-md">
  Where AGENTS.md differs from CLAUDE.md
</h3>

Un `AGENTS.md` che Claude legge attraverso l'impostazione **Project instructions** differisce da un `CLAUDE.md` in questi posti:

|                                                                                                                                                 | `CLAUDE.md`                                                                       | `AGENTS.md` letto attraverso l'impostazione                                                                 |
| :---------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------- |
| [Hook `InstructionsLoaded`](/docs/it/hooks#instructionsloaded)                                                                                       | Si attivano                                                                       | Non si attivano. Si attivano come al solito per un `AGENTS.md` che un `CLAUDE.md` importa o crea un symlink |
| Directory che aggiungete con `--add-dir` mentre [`CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD`](#load-from-additional-directories) è impostato | Il loro `CLAUDE.md` carica                                                        | Il loro `AGENTS.md` non carica                                                                              |
| Un import `@path` di un file al di fuori della vostra directory di lavoro                                                                       | Claude Code vi chiede di approvare gli [import esterni](#import-additional-files) | Carica solo se avete già approvato gli import esterni per questo progetto, senza prompt                     |

<h3 id="remove-an-earlier-agents-md-workaround">
  Remove an earlier AGENTS.md workaround
</h3>

Se avete configurato Claude Code per leggere `AGENTS.md` prima che lo facesse da solo, ecco cosa fare con ogni configurazione comune:

* **Un `CLAUDE.md` contenente `@AGENTS.md`**: potete lasciarlo. Mantenere l'import non fa mai leggere `AGENTS.md` due volte a Claude, qualunque valore di **Project instructions** usiate. Rimuovete il `CLAUDE.md` se non contiene nient'altro, o mantenetelo se alcune delle vostre sessioni [non possono caricare `AGENTS.md` direttamente](#when-agents-md-support-is-unavailable).
* **Un `CLAUDE.md` che dice a Claude a parole di leggere `AGENTS.md`**: Claude vede `AGENTS.md` solo se decide di aprire il file. Eliminate il `CLAUDE.md` in modo che Claude legga `AGENTS.md` direttamente, o sostituite la frase con un import `@AGENTS.md`.
* **Un `CLAUDE.md` con symlink a `AGENTS.md`**: nulla, o eliminate il symlink. In entrambi i casi Claude legge il contenuto una volta.
* **Un hook `SessionStart` che stampa `AGENTS.md`**: rimuovetelo. Una volta che Claude legge `AGENTS.md` direttamente, l'hook aggiunge una seconda copia al contesto.

<h3 id="share-one-file-with-other-coding-tools">
  Share one file with other coding tools
</h3>

Quando Claude non legge il vostro `AGENTS.md` direttamente, potete comunque mantenerlo come l'unico file che ogni strumento condivide mettendo un import `@AGENTS.md` in un `CLAUDE.md` accanto ad esso. Fatelo quando il vostro progetto ha anche un `CLAUDE.md`, quando avete impostato **Project instructions** su `claude-md`, o in sessioni che [non possono caricare `AGENTS.md`](#when-agents-md-support-is-unavailable). Aggiungete qualsiasi istruzione specifica di Claude sotto l'import, e Claude legge il file importato per primo, poi il resto:

```markdown CLAUDE.md theme={null}
@AGENTS.md

## Claude Code

Use plan mode for changes under `src/billing/`.
```

Se non avete bisogno di contenuto specifico di Claude, un symlink funziona anche:

```bash theme={null}
ln -s AGENTS.md CLAUDE.md
```

Il comando non stampa nulla in caso di successo. Prima di scegliere il symlink rispetto all'import, controllate questi vincoli:

* **Modifica**: Claude legge `CLAUDE.md` attraverso il link, ma gli strumenti Edit e Write [rifiutano di scrivere attraverso un symlink](/docs/it/errors#refusing-after-a-symlink-changed), e il rifiuto dirige Claude a modificare il target del link, `AGENTS.md`, invece
* **Windows**: se voi o chiunque cloni il repository lavorate su Windows, usate l'import `@AGENTS.md` invece. Creare un symlink lì richiede privilegi di amministratore o modalità sviluppatore, e Git controlla un symlink committato come un file di testo semplice a meno che `core.symlinks` non sia abilitato, il che lascia quel clone con un `CLAUDE.md` di una riga al posto delle vostre istruzioni

Con entrambi gli approcci, eseguite `/context` nella vostra prossima sessione e confermate che `CLAUDE.md` appare sotto **Memory files**.

<h3 id="migrate-instructions-from-other-tools">
  Migrate instructions from other tools
</h3>

L'esecuzione di [`/init`](/docs/it/commands) legge i file di istruzioni di altri strumenti e incorpora le parti rilevanti nel `CLAUDE.md` generato:

* Regole Cursor in `.cursor/rules/` o `.cursorrules`
* Regole Copilot in `.github/copilot-instructions.md`
* Con `CLAUDE_CODE_NEW_INIT=1` impostato: `AGENTS.md`, `.devin/rules/`, `.windsurf/rules/` o `.windsurfrules`, e `.clinerules`

Potete anche eseguire [`/import`](/docs/it/commands) per portare la configurazione di un agente di codifica supportato in Claude Code, che aggiunge una copia una tantum di file di istruzioni come `AGENTS.md` al `CLAUDE.md` corrispondente e trasporta i server MCP, i comandi, i subagent e le skills. Richiede Claude Code v2.1.213 o successivo.

<h2 id="auto-memory">
  Memoria automatica
</h2>

La memoria automatica consente a Claude di accumulare conoscenze tra le sessioni senza che tu scriva nulla. Mentre lavora, Claude salva quattro tipi di note per se stesso. Claude registra il tipo come campo `type` nel frontmatter del file di memoria:

* `user`: il tuo ruolo, competenze e preferenze di lavoro
* `feedback`: correzioni che dai a Claude e approcci che confermi
* `project`: lavoro in corso, scadenze e decisioni che Claude non può derivare dal codice o dalla cronologia git
* `reference`: dove trovare informazioni al di fuori del progetto, come un issue tracker o una dashboard

Claude salta qualsiasi cosa possa derivare dalla codebase, come architettura, percorsi di file o correzioni di debug. Salta anche qualsiasi cosa i tuoi file CLAUDE.md dicono già.

Claude non salva qualcosa ogni sessione. Decide cosa vale la pena ricordare in base al fatto che l'informazione sarebbe utile in una conversazione futura.

<h3 id="enable-or-disable-auto-memory">
  Abilita o disabilita la memoria automatica
</h3>

La memoria automatica è attivata per impostazione predefinita. Per attivarla/disattivarla, apri `/memory` in una sessione e usa l'interruttore di memoria automatica, che salva `autoMemoryEnabled` nelle impostazioni utente in `~/.claude/settings.json`. Per disattivarla per un singolo progetto, imposta `autoMemoryEnabled` nelle impostazioni di quel progetto:

```json theme={null}
{
  "autoMemoryEnabled": false
}
```

Per disabilitare la memoria automatica tramite variabile di ambiente, imposta `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1`.

<h3 id="storage-location">
  Posizione di archiviazione
</h3>

Ogni progetto ottiene la propria directory di memoria in `~/.claude/projects/<project>/memory/`. Il percorso `<project>` è derivato dal repository git, quindi tutti i worktrees e le sottodirectory all'interno dello stesso repo condividono una directory di memoria automatica. Al di fuori di un repository git, viene utilizzata la radice del progetto.

Se imposti [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/it/sessions#name-the-project-directory-yourself) accanto a `CLAUDE_CONFIG_DIR`, Claude Code utilizza quel nome come directory `<project>` sotto `<config dir>/projects/` indipendentemente da quale repository avvii, quindi i progetti avviati con quella directory di configurazione condividono una directory di memoria automatica. Richiede Claude Code v2.1.234 o successivo.

Per archiviare la memoria automatica in una posizione diversa, imposta `autoMemoryDirectory` nel tuo `settings.json`. Viene letto da qualsiasi [ambito di impostazioni](/docs/it/settings#settings-precedence): utente, progetto, locale, politica, o `--settings`.

```json theme={null}
{
  "autoMemoryDirectory": "~/my-custom-memory-dir"
}
```

Il valore deve essere un percorso assoluto o iniziare con `~/`.

Quando lo imposti nel `.claude/settings.json` o `.claude/settings.local.json` di un progetto, Claude Code lo rispetta secondo la stessa [regola di trust dell'area di lavoro dei hook nei file di impostazioni](/docs/it/permissions#what-runs-before-you-trust-a-folder). Mentre [`permissions.blockReadsOutsideWorkingDirectories`](/docs/it/settings-reference#permissions-blockreadsoutsideworkingdirectories) è attivo, Claude Code non carica alcuna memoria automatica da una directory che un [file di impostazioni fornito dal repository](/docs/it/permissions#when-your-local-settings-file-needs-trust) sceglie e non salva nulla in essa, ovunque si trovi quella directory.

La directory contiene un indice `MEMORY.md` e un file di argomento per memoria:

```text theme={null}
~/.claude/projects/<project>/memory/
├── MEMORY.md           # Indice, una riga per memoria, caricato in ogni sessione
├── user_role.md        # Una memoria
├── feedback_testing.md # Una memoria
└── ...                 # Qualsiasi altro file di argomento che Claude crea
```

`MEMORY.md` funge da indice della directory di memoria. Claude legge e scrive file in questa directory durante la tua sessione, usando `MEMORY.md` per tenere traccia di ciò che è archiviato dove.

La memoria automatica è locale alla macchina. Tutti i worktrees e le sottodirectory all'interno dello stesso repository git condividono una directory di memoria automatica. I file non vengono condivisi tra macchine o ambienti cloud.

Claude Code elimina i vecchi transcript di sessione dopo il periodo di conservazione [`cleanupPeriodDays`](/docs/it/settings-reference#cleanupperioddays), ma esclude i file di memoria nella directory di memoria da quella [scansione di conservazione](/docs/it/claude-directory#cleaned-up-automatically). `MEMORY.md` e i file di argomento rimangono fino a quando tu o Claude non li modificate o eliminate.

<h3 id="how-it-works">
  Come funziona
</h3>

Le prime 200 righe di `MEMORY.md`, o i primi 25KB, a seconda di quale viene raggiunto per primo, vengono caricate all'inizio di ogni conversazione. Il contenuto oltre quella soglia non viene caricato all'inizio della sessione. Claude mantiene `MEMORY.md` conciso spostando le note dettagliate in file di argomento separati.

Dopo che Claude scrive in `MEMORY.md`, Claude Code misura il file rispetto ai limiti di lettura di 200 righe e 25KB. Se il file è vicino a un limite, Claude Code ricorda a Claude di accorciarlo: mantieni una riga per voce, sposta i dettagli nei file di argomento e unisci o elimina le voci obsolete. Se il file supera un limite, la scrittura ha comunque successo, ma Claude Code restituisce un [errore che dice a Claude di riscrivere l'indice](/docs/it/errors#memory-index-is-over-its-read-limit), perché tutto ciò che supera il limite viene eliminato al caricamento successivo.

Questo limite si applica solo a `MEMORY.md`. Claude Code carica un file CLAUDE.md fino a 4 MiB completamente e salta un file più grande. I file più brevi producono una migliore aderenza.

Claude Code non carica i file di argomento come `user_role.md` o `feedback_testing.md` all'avvio. Claude li legge su richiesta usando i suoi strumenti di file standard quando ha bisogno delle informazioni.

La memoria automatica della conversazione principale non viene caricata nei [subagenti](/docs/it/sub-agents#what-loads-at-startup); l'eccezione è un [fork](/docs/it/sub-agents#fork-the-current-conversation), che eredita la conversazione padre e il prompt di sistema. La memoria automatica di un subagente, abilitata con il campo `memory` del subagente, è una directory separata.

Claude legge e scrive file di memoria durante la tua sessione. Quando vedi messaggi come "Saved 2 memories" o "Recalled 2 memories" nell'interfaccia di Claude Code, Claude sta attivamente aggiornando o leggendo da `~/.claude/projects/<project>/memory/`.

Quando Claude scrive un file di memoria che inizia con frontmatter YAML, Claude Code registra l'ora di scrittura in un campo frontmatter `modified` come timestamp ISO 8601. Il timestamp mostra quanto è attuale il fatto, sia per te che per Claude quando lo legge di nuovo. Qualsiasi file che ha frontmatter ottiene il campo la prossima volta che Claude lo scrive, inclusi i file creati in versioni precedenti; Claude Code non aggiunge mai frontmatter a un file che non ne ha. Il campo `modified` richiede Claude Code v2.1.214 o successivo.

<h3 id="audit-and-edit-your-memory">
  Controlla e modifica la tua memoria
</h3>

I file di memoria automatica sono markdown semplice che puoi modificare o eliminare in qualsiasi momento. Esegui [`/memory`](#view-and-edit-with-%2Fmemory) per sfogliare e aprire i file di memoria da una sessione.

<h2 id="view-and-edit-with-/memory">
  Visualizza e modifica con `/memory`
</h2>

Il comando `/memory` elenca i tuoi file CLAUDE.md, CLAUDE.local.md e altri file di memoria in tutti gli ambiti utente e progetto, incluse le voci CLAUDE.md utente e progetto per i file che non esistono ancora. Ti consente inoltre di attivare o disattivare la memoria automatica e fornisce un'opzione per aprire la cartella di memoria automatica. Seleziona qualsiasi file per aprirlo nel tuo editor; selezionando uno che non esiste ancora lo crea prima. Per verificare quali file `CLAUDE.md` e file di regole sono stati caricati nella sessione corrente, esegui `/context`.

Gli editor GUI come VS Code aprono il file in una finestra separata e puoi continuare a utilizzare la sessione mentre è aperta. Prima della v2.1.216, `/memory` attendeva che chiudessi il file prima di rispondere. Gli editor terminali come Vim prendono il controllo del terminale fino a quando non esci.

Quando chiedi a Claude di ricordare qualcosa, come "usa sempre pnpm, non npm" o "ricorda che i test API richiedono un'istanza Redis locale", Claude lo salva nella memoria automatica. Per aggiungere istruzioni a CLAUDE.md, chiedi direttamente a Claude, come "aggiungi questo a CLAUDE.md", oppure modifica il file tu stesso tramite `/memory`.

<h2 id="troubleshoot-memory-issues">
  Risolvi i problemi di memoria
</h2>

Questi sono i problemi più comuni con CLAUDE.md e la memoria automatica, insieme ai passaggi per risolverli.

<h3 id="claude-isn’t-following-my-claude-md">
  Claude non sta seguendo il mio CLAUDE.md
</h3>

Il contenuto di CLAUDE.md viene consegnato come messaggio utente dopo il prompt di sistema, non come parte del prompt di sistema stesso. Claude lo legge e cerca di seguirlo, ma non c'è garanzia di conformità rigorosa, specialmente per istruzioni vaghe o conflittuali.

Per eseguire il debug:

* Esegui `/context` e controlla l'elenco sotto **Memory files** per verificare che i tuoi file CLAUDE.md e CLAUDE.local.md siano stati caricati. Se un file `CLAUDE.md` non è presente lì, Claude non può vederlo. Usa `/memory` per aprire e modificare i file.
* Verifica che il CLAUDE.md rilevante si trovi in una posizione che viene caricata per la tua sessione (vedi [Scegli dove mettere i file CLAUDE.md](#choose-where-to-put-claude-md-files)).
* Rendi le istruzioni più specifiche. "Usa indentazione a 2 spazi" funziona meglio di "formatta il codice bene".
* Cerca istruzioni conflittuali tra i file CLAUDE.md. Se due file danno una guida diversa per lo stesso comportamento, Claude potrebbe sceglierne una arbitrariamente.

Se l'istruzione è qualcosa che deve essere eseguito in un punto specifico, come prima di ogni commit o dopo ogni modifica di file, scrivila come un [hook](/docs/it/hooks-guide). Gli hook vengono eseguiti come comandi shell in eventi del ciclo di vita fissi e si applicano indipendentemente da ciò che Claude decide di fare.

Per le istruzioni che vuoi a livello di prompt di sistema, usa [`--append-system-prompt`](/docs/it/cli-reference#system-prompt-flags). Questo deve essere passato al lancio, quindi è più adatto a script e automazione che all'uso interattivo. Per come si comporta quando riprendi una conversazione, vedi [System prompt flags in resumed conversations](/docs/it/cli-reference#system-prompt-flags-in-resumed-conversations).

<Tip>
  Usa l'hook [`InstructionsLoaded`](/docs/it/hooks#instructionsloaded) per registrare esattamente quali file `CLAUDE.md` e file di regole vengono caricati, quando vengono caricati e perché. Questo è utile per eseguire il debug di regole specifiche del percorso o file caricati pigriamente nelle sottodirectory.
</Tip>

<h3 id="my-agents-md-isn’t-loading">
  Il mio AGENTS.md non sta caricando
</h3>

Se il tuo repository ha un `AGENTS.md` e Claude non sembra sapere cosa dice, la causa solita è un `CLAUDE.md` da qualche parte nel percorso del progetto. Per impostazione predefinita Claude legge `AGENTS.md` solo quando non hai alcun `CLAUDE.md` o `CLAUDE.local.md` nella tua directory di lavoro o sopra di essa. Controlla questi in ordine:

1. Cerca un `CLAUDE.md`, `.claude/CLAUDE.md`, o `CLAUDE.local.md` nella tua directory di lavoro o in qualsiasi directory sopra di essa, diversa dal tuo `~/.claude/CLAUDE.md`. Se ne trovi uno, Claude lo legge invece di `AGENTS.md` a meno che tu non imposti **Project instructions** su `claude-md-and-agents-md`.
2. Esegui `claude --version` e conferma v2.1.277 o successivo. Prima della v2.1.281, alcune sessioni, come quelle su Amazon Bedrock o con telemetria disabilitata, [non potevano caricare `AGENTS.md`](#when-agents-md-support-is-unavailable), quindi su quelle versioni aggiorna a v2.1.281 o successivo.
3. Digita `/config` nella tua sessione per aprire il pannello delle impostazioni e conferma che **Project instructions** non è impostato su `claude-md` o `managed-only`. Se non vedi affatto l'impostazione lì, la tua sessione è una che [non può caricare `AGENTS.md`](#when-agents-md-support-is-unavailable).

Per verificare se Claude ha letto il tuo `AGENTS.md`, esegui `/memory` e cerca il suo percorso nell'elenco.

Prima della v2.1.280, `/memory` e `/context` non elencavano un `AGENTS.md` che Claude leggeva direttamente. Su quelle versioni, chiedi a Claude quali sono le sue istruzioni di progetto.

Se vuoi mantenere il `CLAUDE.md` che hai trovato, o la tua sessione non può caricare `AGENTS.md`, [aggiungi un `CLAUDE.md` accanto al tuo `AGENTS.md` che lo importa](#share-one-file-with-other-coding-tools).

<h3 id="i-don’t-know-what-auto-memory-saved">
  Non so cosa ha salvato la memoria automatica
</h3>

Esegui `/memory` e seleziona la cartella di memoria automatica per sfogliare ciò che Claude ha salvato. Tutto è markdown semplice che puoi leggere, modificare o eliminare.

<h3 id="my-claude-md-is-too-large">
  Il mio CLAUDE.md è troppo grande
</h3>

I file con più di 200 righe consumano più contesto e possono ridurre l'aderenza. Claude Code salta un file superiore a 4 MiB. Usa [regole con ambito di percorso](#path-specific-rules) per caricare istruzioni solo quando Claude lavora con file corrispondenti, oppure riduci il contenuto che non è necessario in ogni sessione. La divisione in [importazioni `@path`](#import-additional-files) aiuta l'organizzazione ma non riduce il contesto, poiché i file importati vengono caricati all'avvio.

Il controllo [`/doctor`](/docs/it/commands#all-commands) propone riduzioni per un CLAUDE.md archiviato: taglia il contenuto che Claude può derivare dalla base di codice, come layout di directory, elenchi di dipendenze e panoramiche dell'architettura, e mantiene i rischi, la logica e le convenzioni che differiscono dai valori predefiniti dello strumento. Il controllo di riduzione richiede Claude Code v2.1.206 o successivo.

<h3 id="instructions-seem-lost-after-/compact">
  Le istruzioni sembrano perse dopo `/compact`
</h3>

CLAUDE.md di progetto sopravvive alla compattazione: dopo `/compact`, Claude rilegge il tuo CLAUDE.md dal disco e lo reinetta nella sessione. I file CLAUDE.md annidati nelle sottodirectory e le regole con [frontmatter `paths:`](#path-specific-rules) vengono ricaricati quando Claude legge i file a cui si applicano.

Se un'istruzione è scomparsa dopo la compattazione, è stata data solo nella conversazione, si trova in un CLAUDE.md annidato che non è stato ancora ricaricato, oppure è una regola con ambito di percorso che non ha corrisposto a un file da allora. Aggiungi istruzioni solo per conversazione a CLAUDE.md per farle persistere. Vedi [Cosa sopravvive alla compattazione](/docs/it/context-window#what-survives-compaction) per il breakdown completo.

Vedi [Scrivi istruzioni efficaci](#write-effective-instructions) per una guida su dimensione, struttura e specificità.

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Esegui il debug della tua configurazione](/docs/it/debug-your-config): diagnostica perché CLAUDE.md o le impostazioni non hanno effetto
* [Skills](/docs/it/skills): pacchetto di flussi di lavoro ripetibili che si caricano su richiesta
* [Impostazioni](/docs/it/settings): configura il comportamento di Claude Code con file di impostazioni
* [Memoria subagent](/docs/it/sub-agents#enable-persistent-memory): consenti ai subagents di mantenere la propria memoria automatica
