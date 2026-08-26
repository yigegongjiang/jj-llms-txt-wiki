> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Rileva problemi di sicurezza mentre Claude scrive il codice

> Installa il plugin security-guidance per far sì che Claude riveda le proprie modifiche al codice per individuare vulnerabilità e correggerle nella stessa sessione.

Il plugin di guida sulla sicurezza fa sì che Claude riveda le proprie modifiche al codice per individuare vulnerabilità comuni mentre lavora e corregga ciò che trova nella stessa sessione. Il plugin rileva problemi come injection, deserializzazione non sicura e API DOM non sicure prima che il codice raggiunga una pull request, riducendo quanto la revisione della sicurezza ricade sui revisori umani a valle.

Una volta installato, il plugin viene eseguito automaticamente. Non c'è nulla da invocare e nessun comando separato da ricordare.

Il plugin è il compagno in-session di [Code Review](/docs/it/code-review), che viene eseguito sulle pull request. Questo plugin riduce ciò che raggiunge la PR. Code Review cattura ciò che fa. Per come il plugin si integra con la revisione su richiesta e la scansione CI, vedi [Come questo si adatta ad altri strumenti di sicurezza](#how-this-fits-with-other-security-tools).

<h2 id="prerequisites">
  Prerequisiti
</h2>

* Claude Code CLI versione 2.1.144 o successiva
* Python 3.8 o successiva nel tuo `PATH`. Il plugin prova `python3`, `python` e `py -3` in questo ordine
* Un repository git per la directory in cui lavori. Le revisioni di fine turno e commit fanno il diff rispetto allo stato git e saltano silenziosamente al di fuori di un repository. Il controllo del pattern per modifica funziona ovunque

Al primo avvio il plugin crea un ambiente virtuale sotto `~/.claude/security/` e installa Claude Agent SDK al suo interno, il che richiede `pip` e accesso alla rete. Se quell'installazione fallisce, la revisione del commit ricade a una revisione singola invece di quella agentica. Su Windows il passaggio dell'ambiente virtuale viene saltato, quindi la revisione del commit agentica viene eseguita solo se `claude-agent-sdk` è già importabile e altrimenti ricade allo stesso modo.

<h2 id="install-the-plugin">
  Installa il plugin
</h2>

In una sessione Claude Code, installa dal [marketplace ufficiale Anthropic](/docs/it/discover-plugins#official-anthropic-marketplace):

```text theme={null}
/plugin install security-guidance@claude-plugins-official
```

L'installazione richiede un ambito. Scegli l'ambito utente per scrivere il plugin nelle tue impostazioni utente, in modo che si carichi in ogni nuova sessione locale che avvii su questa macchina. Se Claude Code segnala che il marketplace non è stato trovato, esegui prima `/plugin marketplace add anthropics/claude-plugins-official`, quindi riprova l'installazione.

Quindi attivalo nella sessione corrente con `/reload-plugins`, che applica le modifiche ai plugin in sospeso senza un riavvio:

```text theme={null}
/reload-plugins
```

<h3 id="enable-in-cloud-sessions-and-shared-repositories">
  Abilita nelle sessioni cloud e nei repository condivisi
</h3>

I plugin con ambito utente non vengono trasferiti a [Claude Code sul web](/docs/it/claude-code-on-the-web), perché quelle sessioni vengono eseguite sull'infrastruttura Anthropic piuttosto che sulla tua macchina. Per abilitare il plugin lì, o per attivarlo per tutti coloro che clonano un repository, dichiaralo nelle impostazioni del progetto archiviate:

```json .claude/settings.json theme={null}
{
  "enabledPlugins": {
    "security-guidance@claude-plugins-official": true
  }
}
```

Gli amministratori possono abilitare il plugin a livello organizzativo impostando [`enabledPlugins`](/docs/it/settings#plugin-settings) nelle [impostazioni gestite](/docs/it/admin-setup).

<h2 id="what-the-plugin-checks">
  Cosa controlla il plugin
</h2>

Il plugin rivede il lavoro di Claude in tre punti, ognuno a una profondità diversa:

* [Su ogni modifica di file](#on-each-file-edit): una corrispondenza di pattern veloce per chiamate rischiose, senza chiamata al modello
* [Alla fine di ogni turno](#at-the-end-of-each-turn): una revisione del modello in background di tutto ciò che quel turno ha modificato
* [Su ogni commit o push che Claude fa](#on-each-commit-or-push-claude-makes): una revisione agentica più profonda che legge il codice circostante

Puoi estendere ogni livello [aggiungendo le tue regole](#add-your-own-rules). I controlli incorporati non possono essere rimossi individualmente, ma puoi [disabilitare ogni livello](#disable-or-uninstall) indipendentemente.

<h3 id="on-each-file-edit">
  Su ogni modifica di file
</h3>

Quando Claude scrive in un file, il plugin scansiona il nuovo contenuto per pattern noti rischiosi. Questa è una corrispondenza di pattern senza chiamata al modello, quindi non aggiunge alcun costo di utilizzo.

Categorie di pattern di esempio:

* Esecuzione di codice dinamico: `eval(`, `new Function`, `os.system`, `child_process.exec`
* Deserializzazione non sicura: `pickle`
* Injection DOM: `dangerouslySetInnerHTML`, `.innerHTML =`, `document.write`
* File di workflow: modifiche sotto `.github/workflows/`, che possono concedere autorizzazioni a livello di repository

Il controllo viene eseguito dopo che la modifica è stata applicata e aggiunge l'avviso al contesto di Claude per il passaggio successivo. Ogni avviso si attiva una volta per pattern per file per sessione, quindi le corrispondenze ripetute nello stesso file non inondano la conversazione.

Puoi [aggiungere i tuoi pattern](#add-custom-per-edit-patterns) a questo livello con un file `security-patterns.yaml`.

<h3 id="at-the-end-of-each-turn">
  Alla fine di ogni turno
</h3>

Un turno è un round di risposta di Claude: invii un messaggio, Claude lavora e risponde, e il turno termina. Dopo ogni turno, il plugin calcola un git diff di tutto ciò che è cambiato nell'albero di lavoro durante il turno, incluse le modifiche dagli strumenti di modifica di Claude, dai comandi Bash e dai subagent, e lo invia a una revisione Claude separata focalizzata sulla sicurezza. La revisione viene eseguita in background, quindi la risposta di Claude non viene ritardata. Se la revisione trova problemi, Claude viene ripromptato con i risultati e li affronta come follow-up.

Questo cattura problemi che una corrispondenza di stringa non può, come:

* Bypass dell'autorizzazione
* Riferimenti diretti a oggetti non sicuri
* Injection
* Server-side request forgery
* Crittografia debole

Vedi sia il risultato che la risoluzione di Claude direttamente nella tua sessione. La revisione copre fino a 30 file modificati per turno e si attiva al massimo tre volte di seguito prima di cedere il controllo a te.

<h3 id="on-each-commit-or-push-claude-makes">
  Su ogni commit o push che Claude fa
</h3>

Quando Claude esegue `git commit` o `git push` attraverso il suo strumento Bash, il plugin esegue una revisione agentica più profonda della modifica in background. Questa revisione legge il codice circostante, inclusi i chiamanti, i sanitizer e i file correlati, per decidere se un risultato è reale prima di segnalarlo. Il contesto aggiuntivo mantiene bassi i falsi positivi su pattern che sembrano pericolosi in isolamento ma sono sicuri nel tuo codebase.

Questo livello si attiva solo su commit e push che Claude fa attraverso il suo strumento Bash. I commit che esegui dalla tua shell, incluso l'escape shell `!` all'interno di una sessione, non vengono revisionati. Le revisioni di commit e push sono limitate a 20 per ora mobile. Se i risultati della revisione del commit duplicano ciò che la revisione di fine turno ha già segnalato, Claude non viene ripromptato, quindi un commit pulito non produce output visibile da questo livello.

<h3 id="review-independence-and-limits">
  Indipendenza e limiti della revisione
</h3>

Il plugin non chiede alla stessa istanza di Claude che ha scritto il codice di valutare se stesso. Il controllo per modifica è una corrispondenza di stringa deterministica senza modello coinvolto. Le revisioni di fine turno e commit vengono eseguite come una chiamata Claude separata con un contesto fresco e un prompt focalizzato sulla sicurezza: il revisore inizia dal diff, non ha investimento nell'approccio originale ed è istruito solo a trovare problemi.

Nessuno dei livelli blocca le scritture o i commit. I risultati raggiungono Claude che scrive come istruzioni, Claude li affronta nella conversazione, e il modello di revisione può perdere problemi. Tratta il plugin come uno strato di difesa in profondità, non una soluzione di sicurezza completa. Vedi [Come questo si adatta ad altri strumenti di sicurezza](#how-this-fits-with-other-security-tools).

<h2 id="add-your-own-rules">
  Aggiungi le tue regole
</h2>

Il plugin ha due punti di estensione: un file di guida Markdown per le revisioni supportate dal modello e un file di pattern YAML o JSON per la corrispondenza di stringa per modifica. Entrambi sono additivi. Puoi aggiungere controlli ma non puoi disabilitare quelli incorporati da questi file.

<h3 id="add-guidance-for-the-model-backed-reviews">
  Aggiungi guida per le revisioni supportate dal modello
</h3>

Crea `.claude/claude-security-guidance.md` nel tuo progetto e descrivi il tuo modello di minaccia e la tua lista di controllo di revisione in linguaggio naturale. Le revisioni supportate dal modello lo caricano come contesto aggiuntivo insieme alla lista di controllo delle vulnerabilità incorporata.

Il seguente esempio è per un servizio web con route admin controllate da ruoli e una politica di logging dei dati dei clienti:

```markdown .claude/claude-security-guidance.md theme={null}
# Security guidance for this repo

- Do not log `customer_id` or `account_number` at INFO level or above.
- All routes under `/admin` must call `require_role("admin")` before any database read.
- Use `crypto.timingSafeEqual` for token comparison instead of `===`.
```

Queste regole sono guida per il revisore, non guardrail deterministici. Il plugin presenta le violazioni come risultati per Claude da correggere, ma non blocca le scritture o garantisce che ogni violazione sia catturata. La guida è solo additiva: una regola che dice di ignorare una classe di vulnerabilità non sopprime quei risultati. Per l'applicazione rigida, abbina il plugin con un [hook che blocca la modifica](/docs/it/hooks-guide#block-edits-to-protected-files) o un controllo CI.

<h3 id="add-custom-per-edit-patterns">
  Aggiungi pattern personalizzati per modifica
</h3>

Crea `.claude/security-patterns.yaml` per aggiungere regole regex o substring al [controllo di pattern per modifica](#on-each-file-edit). Questi vengono eseguiti come corrispondenze di stringa deterministiche insieme ai pattern incorporati:

```yaml .claude/security-patterns.yaml theme={null}
patterns:
  - rule_name: internal_api_key
    substrings: ["sk_live_", "AKIA"]
    reminder: "Hardcoded API key prefix. Load credentials from the secret manager."
  - rule_name: tenant_unfiltered_query
    regex: "\\.objects\\.all\\(\\)"
    paths: ["**/src/tenants/**"]
    reminder: "Multi-tenant code must filter by org_id."
```

| Campo           | Tipo   | Descrizione                                                                                                                                                                        |
| :-------------- | :----- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `rule_name`     | string | Identificatore mostrato nell'avviso                                                                                                                                                |
| `reminder`      | string | Testo di avviso aggiunto al contesto di Claude, limitato a 1 KB                                                                                                                    |
| `regex`         | string | Regex Python abbinato al contenuto modificato                                                                                                                                      |
| `substrings`    | list   | Substring letterali; fornisci questo o `regex`                                                                                                                                     |
| `paths`         | list   | Pattern glob opzionali; la regola si applica solo ai file corrispondenti. I glob corrispondono al percorso file completo, quindi prefissa i pattern relativi al progetto con `**/` |
| `exclude_paths` | list   | Pattern glob opzionali da saltare; lo stesso matching di `paths`                                                                                                                   |

Il plugin legge anche `.claude/security-patterns.yml` e `.claude/security-patterns.json` con lo stesso schema. JSON funziona su qualsiasi installazione Python. Le forme YAML richiedono che PyYAML sia importabile, che il plugin non installa per te. Il plugin carica fino a 50 regole personalizzate e salta le regex che sembrano inclini al backtracking catastrofico.

<h3 id="rule-file-lookup-locations">
  Posizioni di ricerca del file di regole
</h3>

Il plugin cerca `claude-security-guidance.md` e `security-patterns.yaml` negli stessi percorsi, indipendentemente da come il plugin è stato abilitato:

| Ambito          | Percorso                                    | Note                                          |
| :-------------- | :------------------------------------------ | :-------------------------------------------- |
| Utente          | `~/.claude/claude-security-guidance.md`     | Si applica a ogni progetto sulla tua macchina |
| Progetto        | `.claude/claude-security-guidance.md`       | Archiviato con il repository                  |
| Progetto locale | `.claude/claude-security-guidance.local.md` | Gitignored, per override personali            |

Il plugin carica tutti i percorsi che esistono e li concatena, con un limite combinato di 8 KB per il file di guida. Gli amministratori possono distribuire regole a livello organizzativo spingendo il file con ambito utente a `~/.claude/` attraverso la gestione dei dispositivi. Gli stessi percorsi si applicano a `security-patterns.yaml`.

<h2 id="usage-cost">
  Costo di utilizzo
</h2>

Il [controllo di pattern per modifica](#on-each-file-edit) non effettua alcuna chiamata al modello e non aggiunge alcun costo. Le revisioni di [fine turno](#at-the-end-of-each-turn) e [commit](#on-each-commit-or-push-claude-makes) spendono ciascuna un utilizzo del modello aggiuntivo che conta verso il tuo [utilizzo](/docs/it/costs) come qualsiasi altra richiesta Claude. La revisione del commit è agentica e può richiedere diversi turni di modello per commit, limitata a 20 revisioni per ora mobile. Aspettati approssimativamente una chiamata di revisione per turno che modifica i file e una revisione più profonda per commit, entrambe soggette ai limiti di cui sopra.

Entrambe le revisioni supportate dal modello utilizzano Claude Opus 4.7 per impostazione predefinita. Imposta `SECURITY_REVIEW_MODEL` per scegliere un modello diverso per la revisione di fine turno e `SG_AGENTIC_MODEL` per la revisione del commit.

Il plugin è disponibile su tutti i piani.

<h2 id="disable-or-uninstall">
  Disabilita o disinstalla
</h2>

Per disattivare i singoli livelli mantenendo il resto, imposta la variabile di ambiente corrispondente:

| Variabile                       | Effetto                                                                          |
| :------------------------------ | :------------------------------------------------------------------------------- |
| `ENABLE_PATTERN_RULES=0`        | Disabilita il [controllo di pattern per modifica](#on-each-file-edit)            |
| `ENABLE_STOP_REVIEW=0`          | Disabilita la [revisione diff di fine turno](#at-the-end-of-each-turn)           |
| `ENABLE_COMMIT_REVIEW=0`        | Disabilita la [revisione di commit e push](#on-each-commit-or-push-claude-makes) |
| `ENABLE_CODE_SECURITY_REVIEW=0` | Disabilita tutte le revisioni supportate dal modello contemporaneamente          |
| `SECURITY_GUIDANCE_DISABLE=1`   | Disabilita il plugin completamente senza disinstallarlo                          |

Per mettere in pausa il plugin nel tuo ambito utente:

```text theme={null}
/plugin disable security-guidance@claude-plugins-official
```

Per rimuoverlo dal tuo ambito utente:

```text theme={null}
/plugin uninstall security-guidance@claude-plugins-official
```

Se il plugin è stato abilitato attraverso il `.claude/settings.json` di un progetto, disabilitarlo da `/plugin` scrive un override al tuo `.claude/settings.local.json` piuttosto che modificare il file archiviato, quindi il plugin rimane disattivato per te mentre i tuoi compagni di squadra non sono interessati. La stessa finestra di dialogo offre anche di disinstallare il plugin per tutti rimuovendolo dal `.claude/settings.json` condiviso; questa opzione richiede Claude Code v2.1.203 o versione successiva. Se è stato abilitato attraverso [impostazioni gestite](/docs/it/admin-setup), solo un amministratore può disabilitarlo.

<h2 id="how-the-plugin-integrates-with-claude-code">
  Come il plugin si integra con Claude Code
</h2>

Il plugin è costruito interamente su [hooks](/docs/it/hooks), il meccanismo per eseguire il tuo codice in punti specifici del loop di Claude. Registra:

| Evento Hook                                                   | Scopo                                                                                  |
| :------------------------------------------------------------ | :------------------------------------------------------------------------------------- |
| `SessionStart`                                                | Bootstrap dell'ambiente Python del plugin                                              |
| `UserPromptSubmit`                                            | Cattura la baseline dell'albero di lavoro su cui la revisione di fine turno fa il diff |
| `PostToolUse` su `Edit`, `Write` e `NotebookEdit`             | Corrispondenza di pattern per modifica                                                 |
| `Stop`                                                        | Revisione diff di fine turno, eseguita in background                                   |
| `PostToolUse` su `Bash`, filtrato a `git commit` e `git push` | Revisione di commit e push, eseguita in background                                     |

Se costruisci i tuoi hook, il [codice sorgente del plugin](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/security-guidance) è un esempio funzionante di esecuzione di una chiamata di modello separata da un hook e alimentazione del risultato di nuovo alla sessione.

<h2 id="how-this-fits-with-other-security-tools">
  Come questo si adatta ad altri strumenti di sicurezza
</h2>

Il plugin è uno strato in un approccio di difesa in profondità. Cattura i problemi il più presto possibile, mentre il codice è ancora nell'editor, ma non è una garanzia e non sostituisce i controlli successivi. Uno stack tipico:

| Fase            | Strumento                                                | Cosa copre                                                                                                                            |
| :-------------- | :------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------ |
| In sessione     | Plugin di guida sulla sicurezza                          | Vulnerabilità comuni nel codice che Claude scrive, corrette nella stessa sessione                                                     |
| Su richiesta    | [`/security-review`](/docs/it/commands#all-commands)          | Passaggio di sicurezza una tantum sul ramo corrente, eseguito quando lo chiedi                                                        |
| Su pull request | [Code Review](/docs/it/code-review), piani Team e Enterprise  | Revisione multi-agent di correttezza e sicurezza con contesto completo del codebase                                                   |
| In CI           | I tuoi scanner di analisi statica e dipendenze esistenti | Regole specifiche del linguaggio, controlli della catena di approvvigionamento e applicazione delle politiche che il plugin non tenta |

Ogni fase successiva cattura ciò che quelle precedenti perdono. Il valore del plugin è ridurre il volume che le raggiunge, non eliminare la necessità di esse.

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

Il plugin scrive diagnostica di runtime in `~/.claude/security/log.txt`. Controlla lì per primo se le revisioni non vengono visualizzate.

Motivi comuni per cui un livello di revisione salta senza un messaggio nella conversazione:

* La directory non è un repository git: le revisioni di fine turno e commit richiedono lo stato git e saltano al di fuori di un repository
* La sessione non ha autenticazione Anthropic: le revisioni supportate dal modello saltano e viene eseguito solo il controllo di pattern per modifica
* Un file `security-patterns.yaml` è presente ma PyYAML non è importabile: il file viene ignorato. Usa `security-patterns.json` invece

<h2 id="related-resources">
  Risorse correlate
</h2>

Per approfondire i pezzi che questa pagina tocca:

* [Code Review](/docs/it/code-review): configura la revisione multi-agent al momento della PR
* [Automatizza i workflow con gli hook](/docs/it/hooks-guide): costruisci i tuoi controlli negli stessi punti del ciclo di vita
* [Scopri e installa plugin](/docs/it/discover-plugins#official-anthropic-marketplace): sfoglia altri plugin ufficiali
