> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Testare i plugin con evals

> Scrivi casi di eval per il tuo plugin Claude Code, eseguili con claude plugin eval, valuta i risultati, confrontali con una baseline senza plugin e blocca la CI in base al punteggio.

Il comando shell `claude plugin eval` esegue il tuo [plugin](/docs/it/plugins/overview) rispetto a una suite di casi di test e valuta i risultati. Ogni caso è un prompt realistico più uno o più valutatori. Un valutatore è un controllo pass/fail su ciò che Claude ha prodotto, come un'espressione regolare sulla risposta, se uno strumento particolare è stato chiamato, o una rubrica che un secondo modello giudica rispetto alla risposta.

Non devi scrivere la suite manualmente. `claude plugin eval init` ti pone domande sul tuo plugin, propone i casi e i valutatori, li prova e scrive i file. Puoi anche chiedere a Claude di fare lo stesso da una sessione che hai già aperto.

Usa evals per:

* Misurare quanto affidabilmente il tuo plugin porta Claude a produrre il risultato corretto
* Rilevare regressioni quando modifichi il plugin o viene rilasciato un nuovo modello
* Vedere quale contributo apporta il plugin rispetto a nessun plugin

Questa pagina è per gli autori di plugin e skill che hanno un plugin funzionante e desiderano testarne il comportamento, e per i team che bloccano i cambiamenti dei plugin nella CI. Il suo formato di caso è separato dal file `evals/evals.json` che il [plugin skill-creator](/docs/it/skills#run-evals-with-skill-creator) utilizza. Per creare un plugin, vedi [Creare un plugin](/docs/it/plugins/create); per controllare i file di un plugin per errori di sintassi e schema piuttosto che il suo comportamento, usa [`claude plugin validate`](/docs/it/plugins/cli-reference#plugin-validate).

<Note>
  Ogni esecuzione di eval e ogni valutatore giudice è una vera chiamata al modello sul tuo account, conteggiata rispetto all'utilizzo del tuo piano o alla tua fattura API, quindi controlla prima i [requisiti](#requirements). Quindi [crea la tua prima suite di eval](#create-your-first-eval-suite), oppure vai a [Eseguire evals nella CI](#run-evals-in-ci) se ne hai già una.
</Note>

<h2 id="requirements">
  Requisiti
</h2>

Per eseguire plugin evals hai bisogno di:

* Claude Code v2.1.269 o successivo. Esegui `claude --version` per controllare e `claude update` per aggiornare.
* Una directory di plugin con un manifesto `plugin.json` o `.claude-plugin/plugin.json`, o un [plugin skills-directory](/docs/it/plugins/loading#plugins-shared-through-a-repository).
* La stessa autenticazione e provider di modello che le tue normali sessioni Claude Code utilizzano. Le esecuzioni di eval, i grader valutati da judge, e `claude plugin eval init` chiamano il modello con le tue credenziali, quindi contano rispetto ai tuoi limiti di utilizzo del piano o alla tua fattura API. Quando il comando riporta un costo, la cifra è una [stima del prezzo di listino](/docs/it/costs) di quelle chiamate.

<h2 id="how-an-eval-run-works">
  Come funziona un'esecuzione di eval
</h2>

Una suite di eval vive in una directory chiamata `evals/` dentro il tuo plugin, organizzata come mostra [Scrivere e perfezionare i casi](#write-and-refine-cases). Ogni caso è la sua sottodirectory con un [prompt](#set-run-limits-and-tools-in-prompt-md) e uno o più [grader](#grade-the-result). Il prompt è qualcosa che una persona che usa il tuo plugin potrebbe digitare, come una richiesta che una delle sue skill dovrebbe gestire.

<h3 id="what-happens-in-a-run">
  Cosa succede in un'esecuzione
</h3>

Per ogni esecuzione di un caso, Claude Code avvia una sessione fresca, [isolata](#how-runs-are-isolated) [non-interattiva](/docs/it/headless) con solo il tuo plugin caricato, invia il prompt, e lascia che Claude lavori finché non finisce o non raggiunge il limite di turni o tempo del caso. Ogni grader quindi controlla la risposta finale, la trascrizione, o un file che Claude ha creato, e passa o fallisce.

<h3 id="how-a-case-is-scored">
  Come viene valutato un caso
</h3>

Un'esecuzione di un agente non deterministico ti dice poco, quindi ogni caso viene eseguito tre volte per impostazione predefinita. Il punteggio di un'esecuzione è la frazione dei suoi grader che hanno passato, ponderata se imposti pesi, e il punteggio del caso è la media tra le sue esecuzioni. Un caso passa quando il suo punteggio soddisfa la [`--threshold`](#command-options), `1.0` per impostazione predefinita. Nelle chiamate di modello, una suite fa approssimativamente casi × esecuzioni esecuzioni di agenti con il plugin e altrettante per la [baseline senza plugin](#the-no-plugin-baseline), più tre brevi chiamate judge per grader `llm` o `baseline` per esecuzione.

<h3 id="the-no-plugin-baseline">
  La baseline senza plugin
</h3>

Un punteggio alto di per sé non ti dice se il plugin ha aiutato, perché Claude potrebbe fare altrettanto bene senza di esso. Per separare i due, le esecuzioni di ogni caso vengono ripetute senza plugin caricato per impostazione predefinita, e ottieni due punteggi, `WITH` e `W/OUT`. La loro differenza, `Δ`, è quello che il plugin ha contribuito. Se un caso ottiene 1.0 sia con che senza il plugin, il plugin non è quello che l'ha fatto passare. I due set di esecuzioni sono chiamati with-arm e without-arm; [Confrontare con una baseline senza plugin](#compare-against-a-no-plugin-baseline) copre come i grader vengono valutati tra loro e come disattivare la baseline.

<h2 id="create-your-first-eval-suite">
  Crea la tua prima suite di eval
</h2>

Questa procedura scrive un caso per il tuo plugin, lo esegue, e legge il risultato. Prima di iniziare, assicurati di avere:

* Claude Code v2.1.269 o successivo e gli altri [requisiti](#requirements)
* Un terminale aperto nella directory root del tuo plugin, quella che contiene `plugin.json` o `.claude-plugin/plugin.json`
* Una skill nel plugin che vuoi testare, e una richiesta che un utente digiterebbe che dovrebbe attivarla

<Steps>
  <Step title="Crea i casi">
    Dalla root del plugin, esegui:

    ```bash theme={null}
    claude plugin eval init
    ```

    Se Claude Code non ha già fiducia in questa directory, prima chiede `Trust this plugin directory?`; rispondi `y`. Una sessione Claude Code interattiva si apre quindi. Claude legge il tuo plugin e ti chiede quale sia un buon risultato, propone prompt che dovrebbero e non dovrebbero attivare il plugin, progetta grader per ognuno, li pilota una volta per controllare che si comportino, e scrive una directory di caso per prompt sotto `evals/`, ognuna denominata dal suo prompt. Quando Claude ti dice che la suite è pronta, esci da quella sessione con `/exit` o Ctrl+D per tornare alla tua shell.

    Se hai già una sessione Claude Code aperta nella root del plugin, puoi invece chiedere a Claude di eseguire `claude plugin eval init`. Claude esegue il comando e poi ti pone le stesse domande in quella conversazione.

    Se preferisci scrivere un caso tu stesso per vedere esattamente cosa contengono i file, segui [Scrivi un caso a mano](#write-a-case-manually) e torna qui per eseguirlo.
  </Step>

  <Step title="Esegui la suite">
    Di nuovo alla tua shell nella root del plugin, esegui ogni caso sotto `evals/`:

    ```bash theme={null}
    claude plugin eval .
    ```

    Hai già fiducia in questa directory durante il passaggio 1, quindi l'esecuzione inizia immediatamente. Se hai scritto il caso a mano invece, l'esecuzione prima chiede `Trust this plugin directory? [y/N]`; rispondi `y`. [Cosa un'esecuzione può accedere](#security) spiega a cosa stai acconsentendo.

    Ogni caso viene eseguito tre volte con il tuo plugin e tre volte senza, quindi un caso è sei esecuzioni. Una linea di progresso viene stampata mentre ogni esecuzione finisce, con il punteggio di quella esecuzione e il verdetto di ogni grader.
  </Step>

  <Step title="Leggi il riepilogo">
    Quando la suite finisce vedi una tabella di riepilogo, seguita da dove è andato il rapporto:

    ```text theme={null}
    CASE        WITH  W/OUT Δ      RUNS COST    NOTES
    first-case  1.00  0.33  +0.67  6    $0.41

    1 case(s) · mean Δ +0.67 · 74s · $0.41
    Report: /Users/you/my-plugin/evals/results/2026-09-10T17-02-11-482Z/report.html
    Published: https://claude.ai/... · keep local next time with --no-publish
    ```

    `WITH` è il punteggio del caso con il tuo plugin caricato, `W/OUT` è il punteggio senza di esso, e un `Δ` positivo significa che il plugin ha aumentato il punteggio. `COST` è una stima del prezzo di listino delle chiamate al modello, e `NOTES` mostra la spiegazione del grader con il peso più alto che fallisce, o l'errore dell'esecuzione, dal with-arm.
  </Step>

  <Step title="Apri il rapporto e itera">
    Apri l'URL `Published:`, o il percorso `Report:` quando non appare una linea `Published:`, per vedere il verdetto di ogni grader e la spiegazione per ogni esecuzione, e per i grader `llm` i voti del judge e l'estratto che ha valutato. La linea `Published:` appare solo quando il tuo account può [pubblicare rapporti](#html-report).

    Il risultato più comune della prima ricerca è un `Δ` vicino a zero con il grader `tool_used: Skill` del caso che fallisce, il che significa che Claude non sta scegliendo la tua skill sulla formulazione naturale. Regola la [`description`](/docs/it/skills#frontmatter-reference) della skill, esegui di nuovo `claude plugin eval .`, e confronta.

    Per iterare su un caso in modo economico, esegui un singolo arm una volta. Un'esecuzione singola è rumorosa, quindi conferma qualsiasi modifica alle tre esecuzioni predefinite prima di fidarti. Con un arm la tabella mostra colonne `SCORE` e `PASS%` invece di `WITH`, `W/OUT`, e `Δ`:

    ```bash theme={null}
    claude plugin eval . --case <case-name> --runs 1 --ablation none
    ```

    Sostituisci `<case-name>` con uno dei nomi di directory sotto `evals/`.
  </Step>
</Steps>

<h2 id="write-and-refine-cases">
  Scrivi e perfeziona i casi
</h2>

I casi che `claude plugin eval init` scrive sono file semplici che puoi aprire, modificare e aggiungere. Un caso è una directory sotto la directory eval del plugin che contiene un `prompt.md`, un `case.yaml`, o entrambi. Per raggruppare i casi, annidali sotto una directory che non è essa stessa un caso; qualsiasi cosa dentro una directory di caso, come `graders/` e file fixture, appartiene a quel caso.

Questo è il layout che `claude plugin eval init` scrive e quello da usare per le nuove suite. Il [riferimento della suite di eval](#eval-suite-reference) ha l'albero completo, inclusi mock e risultati:

```text theme={null}
my-plugin/
├── .claude-plugin/plugin.json
├── skills/...
└── evals/
    ├── first-case/
    │   ├── prompt.md          # frontmatter: case fields; body: the prompt
    │   ├── graders/
    │   │   ├── criteria.md    # frontmatter: type + options; body: rubric or pattern
    │   │   └── skill-fired.md
    │   └── case.yaml          # optional: only for context.* fields
    ├── ignores-unrelated-request/
    │   └── ...
    └── results/               # written by each run; add to .gitignore
```

<h3 id="write-a-case-manually">
  Scrivi un caso a mano
</h3>

Avere Claude che scrive i casi con `claude plugin eval init` è il percorso consigliato. Per scriverne uno tu stesso invece, inizia da un modello vuoto. Il seguente comando scrive un caso denominato `first-case` con un `prompt.md` segnaposto e un grader segnaposto, e non esegue nulla:

```bash theme={null}
claude plugin eval init --bare first-case
```

```text theme={null}
evals/first-case/
├── prompt.md            # the prompt sent to Claude, plus run limits
└── graders/
    └── criteria.md      # one grader: how to score the result
```

In `prompt.md` scrivi il messaggio che Claude riceve in ogni esecuzione, e imposta i limiti dell'esecuzione e gli strumenti che il caso può usare nel suo frontmatter. Apri `evals/first-case/prompt.md` e sostituisci il corpo segnaposto con una richiesta che una delle tue skill dovrebbe gestire, formulata nel modo in cui un utente la digiterebbe piuttosto che nominare la skill. Questo esempio è per una skill che redige messaggi di commit; usa la tua richiesta:

```markdown theme={null}
---
max_turns: 10
allowed_tools: [Read, Glob, Grep, Skill]
---

Write me a commit message for this change: I renamed getUser to fetchUser and updated the three call sites.
```

Ogni esecuzione inizia in una directory di lavoro vuota, quindi metti quello di cui il compito ha bisogno nel prompt stesso, o [configura lo spazio di lavoro](#add-setup-or-history-with-case-yaml) prima.

L'[elenco completo dei campi frontmatter](#prompt-md-fields) copre il modello, il timeout, i tag e le variabili di ambiente.

Ogni file sotto `graders/` è un controllo applicato dopo l'esecuzione. Apri `evals/first-case/graders/criteria.md` e sostituisci il segnaposto con una rubrica per il modello judge, scritta come condizioni PASS e FAIL concrete:

```markdown theme={null}
---
type: llm
---

PASS if <what a correct response contains>.
FAIL if <what a wrong or missing response looks like>.
```

Poi aggiungi un secondo grader che controlla se la tua skill è quella che ha prodotto la risposta. Crea `evals/first-case/graders/skill-fired.md`, sostituendo `your-skill-name` con il nome della directory della tua skill sotto `skills/`, che è il nome con cui Claude la invoca:

```markdown theme={null}
---
type: tool_used
tool: Skill
input_match: '"skill"\s*:\s*"(?:[\w-]+:)?your-skill-name"'
---
```

Questo passa quando Claude ha invocato quella skill almeno una volta durante l'esecuzione, incluso dalla sua forma `plugin-name:skill-name` con namespace.

[Tipi di grader](#grader-types) elenca gli altri controlli disponibili, come la corrispondenza di una regex o la conferma che un file è stato creato.

Con entrambi i file salvati, esegui il caso nel modo in cui la [guida rapida](#create-your-first-eval-suite) fa, con `claude plugin eval .` dalla root del plugin.

<h3 id="set-run-limits-and-tools-in-prompt-md">
  Imposta i limiti di esecuzione e gli strumenti in prompt.md
</h3>

Imposta `max_turns`, `timeout_seconds`, `model`, `tags` di un caso, e gli `allowed_tools` che può usare nel frontmatter di `prompt.md`; il riferimento [prompt.md frontmatter](#prompt-md-fields) elenca ogni campo e il suo valore predefinito.

Claude riceve il corpo esattamente come l'hai scritto. Le menzioni `@path` in esso non vengono espanse in allegati di file, quindi se Claude ha bisogno di leggere un file, concedi uno strumento per esso in `allowed_tools`.

<h3 id="grade-the-result">
  Scegli e pesa i grader
</h3>

Il frontmatter di un grader imposta il suo `type`, e opzionalmente un `weight` che lo fa contare per più del punteggio dell'esecuzione e un [`arm`](#compare-against-a-no-plugin-baseline) che controlla come viene valutato rispetto alla baseline. Dei sei tipi, `regex`, `tool_used`, `tool_order`, e `file_exists` vengono calcolati dalla trascrizione e dai file e non costano nulla, mentre `llm` e `baseline` chiamano un modello judge e si aggiungono al costo dell'esecuzione.

Non ci sono grader di codice personalizzato.

[Tipi di grader](#grader-types) elenca le opzioni di ogni tipo e la condizione di passaggio, e [cosa un grader può guardare](#what-a-grader-can-look-at) elenca i valori che `target` e `focus` accettano.

Il judge per i grader `llm` e `baseline` è un modello piccolo e veloce per impostazione predefinita. Passa `--judge-model sonnet` o un ID modello completo per usarne uno più forte per le rubriche sfumate.

<h4 id="choose-graders-that-give-a-stable-signal">
  Scegli grader che danno un segnale stabile
</h4>

Un grader `llm` chiede a un modello un verdetto, quindi la sua risposta può differire tra le esecuzioni, e differisce di più il testo più lungo che deve leggere. Queste abitudini mantengono i punteggi di una suite abbastanza stabili da fidarsi:

* Per output lungo come un file generato, valutalo con un grader `regex` sui contenuti del file, che controlla l'intero file allo stesso modo ogni volta. Mantieni i grader `llm` per output brevi, con rubriche scritte come condizioni PASS e FAIL concrete.
* Dai a ogni caso un grader sul risultato, come il messaggio finale o un file prodotto, e uno su come Claude ci è arrivato, come `tool_used` o `tool_order`. Insieme ti dicono sia se la risposta era corretta che se il tuo plugin l'ha prodotta.
* Se il grader `tool_used: Skill` di un caso passa ma `Δ` è negativo, sospetta il judge prima del plugin. Un modello judge piccolo può contrassegnare una risposta corretta come sbagliata perché è formattata diversamente da quello che la rubrica descrive. Riesegui con `--judge-model sonnet`, e stringi la rubrica in modo che la formattazione non decida il verdetto.
* Per controllare che una build o un test sia passato dentro l'esecuzione, chiedi al prompt a Claude di eseguirlo e scrivere il risultato in un file, valuta quel file, e asserire che il comando è stato eseguito con un grader `tool_used` il cui `input_match` nomina il comando.

<h3 id="compare-against-a-no-plugin-baseline">
  Valuta rispetto alla baseline senza plugin
</h3>

Quando un plugin è sotto test, ogni caso viene eseguito in due arm per impostazione predefinita. Il with-arm è le sue esecuzioni con il plugin caricato, e il without-arm è lo stesso numero di esecuzioni senza plugin. Il riepilogo e il rapporto mostrano entrambi i punteggi e `Δ`, il punteggio with-arm meno il punteggio without-arm.

Passa `--ablation none` per eseguire solo il with-arm, che dimezza il costo quando non hai bisogno del confronto, come durante l'iterazione sui grader.

In un'esecuzione a due arm, alcuni grader vengono riportati con `scored: false`. Un controllo come "la skill è stata invocata" non può mai passare senza il plugin, quindi contarlo spingerebbe il without-arm verso zero e gonfierebbe `Δ`. Per mantenere i due arm comparabili, Claude Code esclude tali grader dal punteggio in entrambi gli arm e li riporta nel with-arm come indicatori pass/fail solo. Questo include:

* Ogni grader `tool_used` il cui `tool` è `Skill`
* Ogni grader `regex` con `target: mock_calls` e ogni grader `llm` con `focus: mock_calls`, quando ogni [server mock](#mock-mcp-servers) nel caso è uno che il tuo plugin dichiara
* Qualsiasi grader che contrassegni `arm: with-only`

Tre impostazioni cambiano quella esclusione:

* **Ogni grader escluso**: se ogni grader in un caso è nell'insieme escluso, vengono valutati normalmente invece, poiché non ci sarebbe nulla di sinistra da valutare.
* **`arm: both`**: imposta `arm: both` su un grader per valutarlo in entrambi gli arm indipendentemente, che è quello che vuoi per un controllo "non deve invocare la skill" con `min: 0` e `max: 0`.
* **`--ablation none`**: sotto `--ablation none` nulla viene escluso, quindi la stessa suite può produrre un punteggio assoluto diverso nei due modi.

<h3 id="use-a-different-eval-directory">
  Usa una directory eval diversa
</h3>

Se `evals/` è già presa da un altro strumento, mantieni la suite in una directory diversa. Puoi registrare quella directory nel `plugin.json` del plugin in modo che ogni esecuzione e ogni collaboratore la usi, o passarla sulla riga di comando per un'esecuzione singola:

* **In `plugin.json`**: aggiungi `"experimental": { "evals": "quality/evals" }`.
* **Sulla riga di comando**: passa `--eval-dir quality/evals` sia a `claude plugin eval` che a `claude plugin eval init`.

Se imposti entrambi, viene usata la directory del flag. Dai un percorso relativo di nomi di directory semplici come `qa` o `quality/evals`; un percorso assoluto o uno contenente `..` viene rifiutato: come valore di flag è un errore, mentre un valore di manifest inutilizzabile stampa una riga `Warning:` e l'esecuzione usa `evals/` invece. I casi, i risultati, e l'output `init` si spostano tutti in quella directory.

<h2 id="set-up-fixtures-and-mocks">
  Configurare fixture e mock
</h2>

Un caso può richiedere più di un prompt: file o un repository git nell'area di lavoro, una conversazione precedente da continuare, o risposte dai server MCP con cui il vostro plugin comunica. Ognuno di questi viene configurato accanto al caso in modo che le esecuzioni rimangono ripetibili.

<h3 id="add-setup-or-history-with-case-yaml">
  Inizializzare l'area di lavoro o la conversazione
</h3>

Ogni esecuzione inizia in un'area di lavoro vuota. Quando un caso ha bisogno di più del prompt, aggiungete un `case.yaml` accanto a `prompt.md` con un blocco `context`:

* **File fixture o un repository git**: scrivete uno script Bash nella directory del caso e nominate lo script in `context.scaffold_script`. Lo script viene eseguito come voi, al di fuori della sandbox dell'agente, e solo quando passate `--scaffold`, quindi passate questo flag solo per suite che voi o la vostra organizzazione avete scritto.
* **Una conversazione precedente da continuare**: salvate la trascrizione come file `.jsonl` e nominate il file in `context.history_file`, e il prompt del caso diventa il turno utente successivo.
* **Directory fixture che Claude può leggere durante l'esecuzione**: elencatele in `context.add_dirs`.

Un `case.yaml` ha anche bisogno di `schema_version: "1.1"` e `name`; il riferimento [case.yaml fields](#case-yaml-fields) ha l'elenco completo.

Questo `case.yaml` inizializza un'area di lavoro da uno script e permette a Claude di leggere fixture da una directory `resources/`:

```yaml theme={null}
schema_version: "1.1"
name: changelog-from-diff
tags: [smoke]
context:
  scaffold_script: fixture.sh
  add_dirs: [resources]
```

<h3 id="mock-mcp-servers">
  Mock dei server MCP
</h3>

Potete valutare un plugin le cui skill chiamano tool MCP senza il servizio reale dietro di essi. Mettete un file Markdown per tool sotto `evals/mocks/<server>/<tool>.md` per l'intera suite, o sotto la directory `mocks/` del caso stesso per un singolo caso, dove `<server>` è il nome del server nella [configurazione MCP](/docs/it/plugins/components#mcp-servers) del vostro plugin.

Un'esecuzione non avvia mai i veri server MCP del vostro plugin a meno che non lo chiediate. Claude Code registra un sostituto sotto il nome di ogni server. I tool con un file mock rispondono da esso e sono consentiti senza una concessione `--allow-tools`, e un tool senza file mock non è disponibile per Claude. Un server senza alcun mock appare nella riga di progresso `mocked:` del caso come `plugin_<plugin>_<server>[not started: no mock]`.

Il corpo del file è quello che il tool restituisce a Claude. Questo mock sostituisce un tool `create_issue` su un server denominato `tracker`, controlla l'input che Claude invia, e ripete il titolo. Salvate lo come `evals/mocks/tracker/create_issue.md`:

```markdown theme={null}
---
expect:
  title: string
  priority: [low, medium, high]
---

Created issue #4821: {{input.title}}
```

Il corpo e il frontmatter di un file mock accettano queste opzioni:

* **Sostituzioni**: inserite i campi dalla chiamata dell'input con `{{input.<field>}}`, e il contenuto di un file fixture accanto al mock con `{{file:fixtures/{input.<field>}.json}}`.
* **`expect:`**: il blocco `expect:` protegge l'input. Se una chiamata lo viola, l'esecuzione si interrompe con punteggio 0 e registra il motivo, in modo che un caso possa asserire cosa il vostro plugin ha chiesto al server di fare.
* **`error: true`**: impostate `error: true` per restituire il corpo come errore di tool.
* **`type: agent`**: impostate `type: agent` per far rispondere un modello piccolo come il server da istruzioni nel corpo.

Il [mock file reference](#mock-files) elenca ogni chiave e i file `_server.md` e `_tools.json`.

Per valutare le chiamate stesse, puntate un grader a `target: mock_calls`.

Per eseguire contro i veri server MCP del plugin, passate uno di questi flag. In entrambi i casi questi processi vengono eseguiti come voi, al di fuori della sandbox dell'esecuzione, e i loro tool hanno bisogno di una concessione [`--allow-tools`](#grant-tools):

* **`--allow-real-servers`**: avvia il processo reale per ogni server che non avete mockato, e continua a rispondere ai tool mockati dai loro file
* **`--mocks off`**: ignora `mocks/` completamente e avvia ogni server che il plugin dichiara

<h4 id="replay-agent-mock-answers">
  Riprodurre risposte mock dell'agente
</h4>

Un mock `type: agent` risponde con una chiamata al [`--judge-model`](#command-options), quindi il suo output varia tra le esecuzioni e cambia se cambiate il giudice. Quando un'esecuzione si completa senza errore o interruzione, Claude Code salva ogni risposta che un mock dell'agente ha dato sotto la directory dei risultati in `mock-recordings/`.

Aprite `ADOPT.txt` lì per vedere ogni registrazione e la directory `.replay/<server>/` in cui copiarla, accanto al mock che l'ha prodotta. Dopo aver copiato una registrazione lì, le esecuzioni successive rispondono alla chiamata identica da essa senza alcuna chiamata al modello. Committete `mocks/.replay/` insieme al resto di `mocks/` in modo che le esecuzioni CI siano ripetibili.

<h2 id="run-evals">
  Esegui evals
</h2>

Una volta che una suite esiste, `claude plugin eval` la esegue. Scegli quale plugin e quali casi eseguire con l'argomento target, concedi qualsiasi strumento che i casi hanno bisogno oltre il set di sola lettura con `--allow-tools`, e controlla il conteggio delle esecuzioni, i modelli, il costo, e l'output con le altre opzioni.

<h3 id="choose-what-to-evaluate">
  Scegli cosa valutare
</h3>

La maggior parte delle volte esegui `claude plugin eval .` dalla root del plugin, che esegue ogni caso nella suite con il plugin in cui stai caricato. Per eseguire un singolo file di caso, o per valutare un plugin che hai installato piuttosto che uno che stai sviluppando, passa un target diverso:

| Target                                                     | Cosa viene eseguito                                                                                                                                                                                            |
| :--------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| La directory root di un plugin, come `.`                   | Ogni caso sotto la sua directory eval, con quel plugin caricato                                                                                                                                                |
| Un singolo file `prompt.md` o `case.yaml`                  | Quel caso, con il suo plugin contenitore caricato                                                                                                                                                              |
| Un plugin installato per nome, `name` o `name@marketplace` | I casi nella directory eval della copia installata, con la copia installata caricata. I risultati vengono scritti sotto `./evals/results/` nella tua directory corrente, o `./<dir>/results/` con `--eval-dir` |
| `name@skills-dir`                                          | Lo stesso, per un [plugin skills-directory](/docs/it/plugins/loading#plugins-shared-through-a-repository)                                                                                                           |
| Omesso                                                     | La directory corrente come percorso                                                                                                                                                                            |

Aggiungi `--case <glob>` per filtrare per nome di caso e `--tag <tag>` per mantenere i casi con uno qualsiasi dei tag dati. Metti il target prima di `--tag`, `--allow-tools`, e `--json`. I primi due prendono un elenco e `--json` prende un percorso opzionale, quindi ognuno di loro legge un target che segue come il suo valore proprio.

<h3 id="grant-tools">
  Concedi strumenti
</h3>

Le esecuzioni non si fermano mai per chiedere il permesso. Gli strumenti incorporati che hanno bisogno di una concessione che non hai dato, come `Bash`, `Write`, `Edit`, `WebFetch`, e `WebSearch`, vengono rimossi dalla sessione, quindi Claude non può chiamarli affatto.

Un'esecuzione consente solo gli strumenti di sola lettura che il caso elenca in `allowed_tools`, da `Read`, `Glob`, `Grep`, `NotebookRead`, `Skill`, `AskUserQuestion`, `Agent`, `TodoWrite`, e gli strumenti di task `TaskCreate`, `TaskGet`, `TaskList`, `TaskUpdate`, e `TaskStop`, più quello che concedi con `--allow-tools`. Quella concessione si applica a ogni caso nell'esecuzione. Per lasciare che i casi usino `Bash`, `Write`, `Edit`, `WebFetch`, o `WebSearch`, concedili tu stesso:

```bash theme={null}
claude plugin eval . --allow-tools Write Edit "Bash(npm test *)"
```

Quando un caso ha chiesto uno strumento che non hai concesso, l'output di progresso lo elenca come `not granted`. Gli strumenti su un server MCP [mockato](#mock-mcp-servers) non hanno bisogno di concessione. Gli strumenti su un vero server MCP del plugin hanno bisogno sia del server avviato, con `--allow-real-servers` o `--mocks off`, che di una concessione per nome, come `--allow-tools "mcp__plugin_my-plugin_github__*"`; gli strumenti MCP di un plugin sono denominati `mcp__plugin_<plugin>_<server>__<tool>`.

Quando concedi `Bash` in qualsiasi forma, ogni comando viene eseguito sotto la [sandbox a livello di OS](/docs/it/sandboxing) di Claude Code. Le scritture sono confinate allo spazio di lavoro dell'esecuzione, la tua directory home e la configurazione di Claude Code sono illeggibili, e l'accesso alla rete è limitato ai domini che concedi con `--allow-tools "WebFetch(domain:example.com)"`. Se concedi Bash o PowerShell su una macchina senza backend sandbox, Claude Code rifiuta ogni esecuzione piuttosto che eseguirla non confinata, e il caso mostra un errore di esecuzione e di solito ottiene un punteggio 0. Windows nativo non ha backend, quindi esegui le suite che concedono shell sotto WSL2; su Linux, installa prima `bubblewrap` e `socat`. Vedi i [prerequisiti di sandboxing](/docs/it/sandboxing).

<h3 id="command-options">
  Opzioni di comando
</h3>

Questa tabella copre le opzioni per il conteggio delle esecuzioni, i modelli, la valutazione, il costo, le concessioni di strumenti, i mock, e l'output. Esegui `claude plugin eval --help` per l'elenco completo, che include anche `--case`, `--tag`, `--eval-dir`, `--no-scaffold`, `--report`, e `--verbose`.

| Opzione                    | Predefinito                                                                                               | Effetto                                                                                                                                                                                                                                                                                                                                                                 |
| :------------------------- | :-------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--runs <n>`               | `runs` di ogni caso, altrimenti 3                                                                         | Esecuzioni per caso per arm                                                                                                                                                                                                                                                                                                                                             |
| `-j`, `--concurrency <n>`  | `1`                                                                                                       | Esegui fino a questo numero di esecuzioni di agenti contemporaneamente, da 1 a 8. Condividono il limite di velocità del tuo account, quindi questo accorcia il tempo di parete piuttosto che aumentare la velocità oltre quel limite. I risultati mantengono l'ordine dei casi                                                                                          |
| `--model <model>`          | `model` di ogni caso, altrimenti `ANTHROPIC_MODEL` se impostato, altrimenti il predefinito di Claude Code | Modello per l'agente sotto test. Fissalo in CI in modo che un rollout di modello non sia scambiato per una regressione del plugin                                                                                                                                                                                                                                       |
| `--judge-model <model>`    | Un modello piccolo e veloce                                                                               | Modello per i grader `llm` e `baseline`                                                                                                                                                                                                                                                                                                                                 |
| `--ablation <mode>`        | `with-without` quando un plugin si risolve, altrimenti `none`                                             | Se eseguire anche ogni caso senza il plugin per misurare cosa aggiunge. `none` esegue un arm; `with-without` aggiunge la baseline senza plugin                                                                                                                                                                                                                          |
| `--threshold <0..1>`       | `1.0`                                                                                                     | Un caso passa quando il suo punteggio with-arm è almeno questo. Qualsiasi caso sotto di esso fa uscire il comando 1                                                                                                                                                                                                                                                     |
| `--max-cost-usd <usd>`     | Nessun limite                                                                                             | Un limite sul costo stimato al prezzo di listino dell'esecuzione, non sull'utilizzo del piano. Controllato prima che ogni esecuzione inizi. Una volta speso, nulla di ulteriore inizia; le esecuzioni già in volo finiscono, quindi la spesa può superare il limite di quelle esecuzioni. Se un'esecuzione rimane non avviata, il comando esce 2 con risultati parziali |
| `--allow-tools <tools...>` | Nessuno                                                                                                   | Concedi strumenti oltre il set di sola lettura. Vedi [Concedi strumenti](#grant-tools)                                                                                                                                                                                                                                                                                  |
| `--scaffold`               | Spento                                                                                                    | Esegui lo [`scaffold_script`](#add-setup-or-history-with-case-yaml) di ogni caso                                                                                                                                                                                                                                                                                        |
| `--trust-plugin`           | Spento                                                                                                    | Salta il primo prompt di fiducia per un plugin il cui codice e suite eseguiresti tu stesso. Passalo in CI in modo che il lavoro non sia mai rifiutato da o rimasto in attesa al prompt. Vedi [Cosa un'esecuzione può accedere](#security)                                                                                                                               |
| `--mocks <mode>`           | `record`                                                                                                  | `record` risponde alle chiamate di strumenti MCP dai [mock](#mock-mcp-servers), non avvia i veri server del plugin, e salva le risposte dei mock dell'agente per la riproduzione. `off` ignora i mock e avvia i server MCP reali del plugin                                                                                                                             |
| `--allow-real-servers`     | Spento                                                                                                    | Con `--mocks record`, avvia anche i veri server MCP del plugin per i server che non hanno mock                                                                                                                                                                                                                                                                          |
| `--json [path]`            | Spento                                                                                                    | Stampa il [documento di risultato](#json-result) su stdout, o scrivilo in un percorso che termina in `.json`. L'esecuzione è silenziosa: nessuna linea di progresso o tabella di riepilogo                                                                                                                                                                              |
| `--output-dir <dir>`       | `<eval dir>/results/<timestamp>/`                                                                         | Dove vanno `aggregate-result.json` e `report.html`                                                                                                                                                                                                                                                                                                                      |
| `--no-publish`             |                                                                                                           | Mantieni il rapporto HTML locale. Vedi [Rapporto HTML](#html-report)                                                                                                                                                                                                                                                                                                    |
| `--publish-report`         |                                                                                                           | Pubblica il rapporto anche dove rimarrebbe locale per impostazione predefinita, come un'esecuzione che una sessione Claude Code ha avviato                                                                                                                                                                                                                              |
| `--keep-temp`              | Spento                                                                                                    | Mantieni la directory sandbox di ogni esecuzione e stampa il suo percorso, per il debug di quello che Claude ha prodotto                                                                                                                                                                                                                                                |

<h3 id="run-evals-in-ci">
  Esegui evals in CI
</h3>

Nel tuo lavoro CI, esegui la suite con `--json` per scrivere il risultato per l'archiviazione, e fallisci la build sul codice di uscita. Passa `--trust-plugin` in modo che il lavoro non aspetti mai al [primo prompt di fiducia](#security), fissa entrambi i modelli in modo che i punteggi siano comparabili nel tempo, mantieni il rapporto locale, e imposta un limite di costo come limite superiore:

```bash theme={null}
claude plugin eval . \
  --trust-plugin \
  --json results.json \
  --threshold 0.8 \
  --model claude-sonnet-5 \
  --judge-model claude-haiku-4-5 \
  --no-publish \
  --max-cost-usd 20
```

Il codice di uscita del lavoro ti dice cosa è successo:

| Codice di uscita | Significato                                                                                                                                                                                                                                                                       |
| :--------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 0                | Ogni caso ha ottenuto un punteggio pari o superiore a `--threshold` e ogni file di caso è stato caricato                                                                                                                                                                          |
| 1                | Un caso ha ottenuto un punteggio inferiore alla soglia, un file di caso non è stato caricato, non sono stati trovati casi, un'esecuzione non poteva essere avviata, la directory del plugin non è attendibile e `--trust-plugin` non è stato passato, o un'opzione non era valida |
| 2                | Esecuzione parziale: il limite `--max-cost-usd` è stato raggiunto, o la tua credenziale è stata rifiutata prima o alla prima esecuzione. `results.json` è ancora scritto con `partial: true` e il motivo                                                                          |
| 130              | Interrotto. I risultati parziali sono scritti                                                                                                                                                                                                                                     |
| 143              | Terminato, come da un timeout CI                                                                                                                                                                                                                                                  |

I problemi di scrittura o pubblicazione del rapporto HTML non cambiano mai il codice di uscita.

Per vedere perché un caso ha ottenuto un punteggio basso, eseguilo localmente senza `--json` in modo che le linee di progresso per caso e per grader vengano stampate.

Un runner CI ha anche bisogno di questi elementi:

* **Installazione e credenziali**: un runner CI ha bisogno di un'installazione di Claude Code e [credenziali nell'ambiente](/docs/it/authentication) come `ANTHROPIC_API_KEY`.
* **Fiducia**: senza `--trust-plugin`, un lavoro la cui directory di checkout Claude Code non ha già fiducia ha bisogno del [primo prompt di fiducia](#trust-the-plugin-directory), e un'esecuzione che non può chiedere è rifiutata con uscita 1.
* **`init` in CI**: `claude plugin eval init` ha bisogno di un terminale per farti le sue domande; in CI, esegui `claude plugin eval init --bare <name>` per ottenere il modello vuoto.

Per mantenere i costi prevedibili, dai alle suite di ogni cambio veloce solo grader che non chiamano un judge, usa `--ablation none` dove non hai bisogno di `Δ`, e lascia i documenti `partial: true` e le esecuzioni con `skippedPaidGraders` fuori da qualsiasi tendenza che grafici.

<h2 id="read-the-results">
  Leggi i risultati
</h2>

Ogni esecuzione con almeno un caso scrive una directory `results/<timestamp>/` dentro la directory eval, contenente `aggregate-result.json` e `report.html`. Per un target di percorso che è sotto il plugin; per un plugin che hai denominato, è sotto la tua directory corrente, come la [tabella target](#choose-what-to-evaluate) mostra. La tabella di riepilogo, il JSON, e il rapporto rendono tutti gli stessi dati di risultato.

<h3 id="html-report">
  Rapporto HTML
</h3>

`report.html` è un singolo file autonomo che non fa richieste esterne, quindi puoi allegarlo a un lavoro CI o aprirlo dal disco. Questo esempio è la parte superiore di un rapporto per un'esecuzione di suite a tre casi con `--threshold 0.8`; il costo mostrato è una stima al prezzo di listino e varia con il modello e il numero di casi:

<img src="https://mintcdn.com/claude-code/qq7LHDi_F0aeFHgk/images/plugin-eval-report.png?fit=max&auto=format&n=qq7LHDi_F0aeFHgk&q=85&s=106eb6e6a70a6565f891ea3a4564f87d" alt="Parte superiore di un rapporto eval: una linea di verdetto che legge &#x22;Plugin effect: +33.3 pts vs baseline, improved 2, flat 1, regressed 0 of 3 cases&#x22;, cinque riquadri di riepilogo per il punteggio della suite, il delta di ablazione, il punteggio di base, i casi che superano la soglia, e le esecuzioni perfette, quindi il primo caso con il suo delta, la barra del punteggio, e un'esecuzione i cui due grader mostrano entrambi il passaggio" width="1360" height="1032" data-path="images/plugin-eval-report.png" />

Leggilo dall'alto verso il basso:

* **La linea di verdetto e i riquadri** rispondono se il plugin ha aiutato in tutta la suite. Il punteggio della suite è la media dei punteggi con plugin per caso, Ablation Δ è quanto quella si trova sopra o sotto il punteggio di base, e Cases conta quanti hanno raggiunto la soglia. Perfect runs è la quota di esecuzioni con plugin dove ogni grader ha superato.
* **Ogni scheda di caso** mostra il `Δ` del caso e il punteggio con plugin, con un segno sulla barra alla soglia. Un caso il cui `Δ` è negativo ottiene un bordo sinistro rosso, quindi le regressioni risaltano quando scorri.
* **All'interno di un caso**, le esecuzioni con plugin vengono prima e le esecuzioni di base dopo. Ogni esecuzione elenca i suoi grader con un chip di passaggio o fallimento. Un grader fallito è già espanso con la sua spiegazione, e un grader `llm` mostra anche i voti del giudice e le prove che gli sono state mostrate, che è dove scopri perché un'esecuzione ha ottenuto un punteggio basso. I grader che non contano verso il punteggio, come `tool_used: Skill`, portano un badge `plugin-fired indicator`.
* **Prompt e Graders**, sotto le esecuzioni, mostrano il prompt del caso e la rubrica o il modello di ogni grader, quindi qualcuno che legge il rapporto senza la suite può vedere cosa è stato chiesto e cosa è contato come buono.

Se sei connesso con un abbonamento claude.ai e gli [artifact](/docs/it/artifacts) sono disponibili per il tuo account, Claude Code pubblica anche il rapporto come artifact privato e stampa `Published: <url>`. Passa `--no-publish` per mantenerlo locale. Se non appare una linea `Published:`, come con l'autenticazione con chiave API, il file locale è il rapporto.

Un'esecuzione che una sessione Claude Code ha avviato, come quando chiedi a Claude di eseguire la suite per te, rimane anche locale, e la sua linea `Report:` dice `kept local`. Aggiungi `--publish-report` a quel comando per pubblicarla.

<h3 id="json-result">
  Risultato JSON
</h3>

`aggregate-result.json`, e l'output `--json`, è un documento versionato con `schemaVersion: 1` per gli script CI da analizzare. I nomi dei campi sono camelCase e i nuovi campi vengono aggiunti senza rinominare quelli esistenti, quindi scrivi il tuo script per ignorare i campi che non riconosce.

Questi sono i campi che uno script di gating di solito legge. Il documento porta anche la configurazione della suite, ogni definizione di grader, e risultati di grader per esecuzione con spiegazioni e prove:

| Campo                                             | Significato                                                                                                                                                                                                                            |
| :------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `partial`, `partialReason`                        | `true` con `cost_ceiling`, `interrupted`, o `auth_failed` quando la suite non è finita. Lascia i risultati parziali fuori dai grafici di tendenza                                                                                      |
| `aggregates.overallScore`                         | Punteggio medio del caso in tutta la suite                                                                                                                                                                                             |
| `aggregates.casesPassed`, `aggregates.casesTotal` | Casi pari o superiori a `--threshold`, e il totale                                                                                                                                                                                     |
| `aggregates.meanDelta`                            | Media `Δ` tra i casi, in modalità a due arm                                                                                                                                                                                            |
| `cases[].name`                                    | Nome del caso                                                                                                                                                                                                                          |
| `cases[].aggregates.score`                        | Punteggio medio dell'esecuzione with-arm per il caso                                                                                                                                                                                   |
| `cases[].aggregates.delta`                        | Punteggio with-arm meno punteggio without-arm. Omesso quando gli arm non sono comparabili                                                                                                                                              |
| `cases[].arms.with[].error`                       | `null`, o perché un'esecuzione è terminata anormalmente, come `timed out after 300s`. Un'esecuzione che è iniziata ma è terminata male è ancora valutata su quello che ha prodotto, quindi un errore non nullo non implica punteggio 0 |
| `cases[].arms.with[].aborted`                     | Presente quando un [mock](#mock-mcp-servers) `expect:` o `abort_when` ha fermato l'esecuzione, con `server`, `tool`, e `reason`. L'esecuzione ottiene un punteggio 0 e `error` rimane `null`                                           |
| `cases[].arms.with[].skippedPaidGraders`          | `true` quando il limite di costo ha saltato i grader judge di questa esecuzione, quindi il suo punteggio non è comparabile                                                                                                             |
| `costUsd`, `durationSeconds`, `claudeVersion`     | Costo stimato al prezzo di listino incluse le chiamate judge, secondi di parete, e la versione di Claude Code che ha eseguito la suite                                                                                                 |

<h2 id="security">
  Cosa un'esecuzione può accedere
</h2>

`claude plugin eval` carica le skill, gli hook e gli agenti del plugin target ed esegue la sua suite di eval sulla tua macchina, come te. Puntare a un plugin è la stessa decisione di fiducia di `claude --plugin-dir`, quindi valuta solo i plugin di cui ti fidi.

L'isolamento descritto in questa sezione limita quello che l'agente sotto test può raggiungere; non è un confine contro il codice del plugin stesso, e una suite che passa non dice nulla su se il plugin è sicuro.

<h3 id="trust-the-plugin-directory">
  Fidati della directory del plugin
</h3>

La prima volta che esegui `claude plugin eval` rispetto a una directory, Claude Code chiede `Trust this plugin directory?` prima di caricare qualsiasi cosa da essa, a meno che non hai già accettato il prompt di fiducia lì in una sessione `claude` interattiva. All'interno di un repository git, rispondere sì fidati dell'intero repository, per le sessioni interattive anche. Quando stdin o stdout non è un terminale, sotto `--json`, o quando la variabile di ambiente `CI` è impostata a un valore vero come `true`, l'esecuzione non può chiedere ed è rifiutata con uscita 1; passa `--trust-plugin` per asserire la fiducia tu stesso, solo per un plugin che eseguiresti sulla tua macchina. Un target che nomini piuttosto che dai come percorso, significando un plugin installato o un plugin skills-directory, salta il prompt.

Alcune parti del plugin e della suite vengono eseguite solo quando passi il loro flag per quella esecuzione:

* Uno [`scaffold_script`](#add-setup-or-history-with-case-yaml) di un caso con `--scaffold`
* [Strumenti oltre il set di sola lettura](#grant-tools) con `--allow-tools`
* I [veri server MCP](#mock-mcp-servers) del plugin con `--allow-real-servers` o `--mocks off`

Un `allowed_tools` di un caso e un frontmatter `allowed-tools` proprio di una skill non possono ampliare nessuno di loro.

Quando il plugin include hook che non hai scritto, o inizi i suoi veri server MCP, tratta i suoi punteggi come consultivi a meno che non l'hai eseguito in un ambiente isolato come un contenitore o runner CI, poiché gli hook e i server vengono eseguiti fuori dalla sandbox dell'agente e potrebbero modificare i file che i grader leggono.

<h3 id="how-runs-are-isolated">
  Come le esecuzioni sono isolate
</h3>

Ogni esecuzione ottiene una directory home, directory di lavoro, e configurazione di Claude Code monouso, e l'agente sotto test viene eseguito lì come processo figlio `claude -p` con solo il tuo plugin caricato. Tieni a mente queste conseguenze quando scrivi i casi:

* **Nulla di personale o a livello di progetto carica.** Le tue impostazioni utente, gli hook, i file `CLAUDE.md`, i server MCP, gli altri plugin installati, la memoria, e le skill sono assenti, e nessun `.claude/` o `.mcp.json` con scope di progetto sopra la sandbox viene letto. La maggior parte del tuo ambiente shell è anche trattenuta; solo un [allowlist](#prompt-md-fields) e le variabili `EVAL_*` raggiungono l'esecuzione. Se il plugin ha bisogno di setup, spediscilo nel plugin, crealo in uno `scaffold_script`, o passa le variabili `EVAL_*`.
* **La politica gestita può ancora limitare un'esecuzione.** Le restrizioni nelle [impostazioni gestite](/docs/it/managed-settings) che un amministratore ha distribuito alla macchina si applicano dentro un'esecuzione, quindi i risultati su una macchina gestita possono differire da una non gestita da quella politica.
* **Lo strumento Artifact è spento.** Una skill che pubblica un [artifact](/docs/it/artifacts) può essere valutata solo su quello che produce prima di quel passaggio.
* **Le definizioni del caso sono nascoste all'agente.** Un'esecuzione non può leggere la directory eval, quindi Claude non può vedere il prompt del caso, i suoi grader, o i casi fratelli.
* **Nessuna sandbox di rete al di fuori dei comandi shell.** I comandi shell che concedi vengono eseguiti sotto le regole sandbox della rete. Una concessione `WebFetch(domain:…)` raggiunge quel dominio direttamente, e gli hook del plugin stesso e qualsiasi vero server MCP che inizi possono raggiungere qualsiasi host.

<h2 id="eval-suite-reference">
  Riferimento della suite di eval
</h2>

Tutto quello che una suite di eval può contenere vive sotto la directory eval del plugin, `evals/` a meno che non hai [configurato un'altra](#use-a-different-eval-directory). Questo albero mostra ogni file che `claude plugin eval` legge o scrive lì; solo `prompt.md` o `case.yaml` è richiesto per un caso per esistere:

```text theme={null}
evals/
├── <case>/                        # one directory per case; nest under a non-case directory to group
│   ├── prompt.md                  # frontmatter: case and run fields; body: the prompt
│   ├── case.yaml                  # optional: context.* fields, or the whole case in one file
│   ├── graders/
│   │   └── <name>.md              # one grader per file; frontmatter: type and options; body: rubric
│   ├── mocks/                     # optional: mocks for this case only, same layout as below
│   └── <fixtures, scripts, transcripts referenced by case.yaml>
├── mocks/                         # optional: suite-wide MCP mocks
│   ├── <server>/
│   │   ├── <tool>.md              # one mocked tool; body: the tool result
│   │   ├── _server.md             # optional: one agent that answers several tools
│   │   ├── _tools.json            # optional: saved tools/list response for real descriptions and schemas
│   │   └── fixtures/              # files inserted with {{file:fixtures/...}}
│   └── .replay/<server>/          # adopted agent-mock recordings, answered without a model call
└── results/<timestamp>/           # written by each run; add results/ to .gitignore
    ├── aggregate-result.json
    ├── report.html
    └── mock-recordings/           # agent-mock answers from clean runs, with ADOPT.txt
```

<h3 id="prompt-md-fields">
  prompt.md frontmatter
</h3>

Il frontmatter di `prompt.md` accetta questi campi. Una chiave sconosciuta è un errore:

| Campo                  | Predefinito                          | Scopo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| :--------------------- | :----------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `schema_version`       | `"1.1"`, impostato per te            | Versione del formato del caso. I casi scritti come `prompt.md` lo ottengono automaticamente, quindi raramente lo imposti                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `name`                 | Il nome della directory              | Nome del caso. I glob `--case` lo corrispondono e il rapporto lo chiave                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `description`          |                                      | Per gli umani. Non usato al momento dell'esecuzione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `tags`                 | `[]`                                 | Etichette per il filtraggio `--tag`. Un caso viene eseguito se uno qualsiasi dei suoi tag corrisponde                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `plugins`              | Il plugin contenitore più vicino     | Directory di plugin sotto test, relative alla directory del caso. Imposta `plugins: ["../.."]` quando il rilevamento automatico non trova il tuo plugin; vedi [il plugin non è stato caricato](#the-baseline-arm-shows-no-plugin-or-delta-is-zero)                                                                                                                                                                                                                                                                                                          |
| `runs`                 | `3`                                  | Esecuzioni per arm, da 1 a 50. `--runs` lo sostituisce                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `expected_outcome`     |                                      | Per gli umani. Non usato al momento dell'esecuzione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `model`                | Il predefinito della sessione figlio | Modello per l'agente sotto test. `--model` lo sostituisce                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `max_turns`            | `10`                                 | Limite di turni, fino a 200. Colpirlo viene registrato come errore di esecuzione e di solito abbassa il punteggio, quindi impostalo generosamente                                                                                                                                                                                                                                                                                                                                                                                                           |
| `timeout_seconds`      | `300`                                | Limite di parete per esecuzione, fino a 3600                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `allowed_tools`        | `[]`                                 | Strumenti che il caso vuole, come `[Read, Glob, Grep, Skill]`. Gli strumenti di sola lettura vengono concessi quando elencati qui; per qualsiasi altra cosa, vedi [Concedi strumenti](#grant-tools)                                                                                                                                                                                                                                                                                                                                                         |
| `append_system_prompt` |                                      | Testo aggiunto al prompt di sistema della sessione figlio                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `env`                  | `{}`                                 | Variabili di ambiente extra per la sessione figlio. Le chiavi devono corrispondere a `EVAL_[A-Z0-9_]*`; qualsiasi altra chiave fallisce l'esecuzione. L'esecuzione eredita solo un allowlist dalla tua shell: le basi come `PATH` e locale, le impostazioni proxy e certificato, le variabili che selezionano e autenticano il tuo provider di modello, la maggior parte di `ANTHROPIC_*` e `CLAUDE_CODE_*` configurazione, e `EVAL_*`. Per consegnare al plugin qualsiasi altra cosa, come un'impostazione di toolchain, esportala come variabile `EVAL_*` |

<h3 id="case-yaml-fields">
  case.yaml fields
</h3>

`case.yaml` è un'alternativa o un complemento a `prompt.md`: descrive un caso in YAML e aggiunge i campi che puntano ad altri file. Richiede `schema_version: "1.1"` e `name`. I campi `prompt.md` `description`, `tags`, `plugins`, `runs`, e `expected_outcome` vanno al livello superiore; `model`, `max_turns`, `timeout_seconds`, `allowed_tools`, `append_system_prompt`, e `env` vanno sotto `execution:`. Quando entrambi i file esistono, il frontmatter di `prompt.md` sostituisce i campi corrispondenti di `case.yaml`, il corpo di `prompt.md` è il prompt, e `graders/*.md` vengono aggiunti dopo qualsiasi grader elencato in `case.yaml`.

Questi campi esistono solo in `case.yaml`:

| Campo                     | Scopo                                                                                                                                                                                                                                               |
| :------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `context.scaffold_script` | Uno script Bash nella directory del caso che viene eseguito nello spazio di lavoro vuoto prima che Claude inizi, per creare file fixture o un repository git. Viene eseguito solo quando passi [`--scaffold`](#add-setup-or-history-with-case-yaml) |
| `context.history_file`    | Una trascrizione `.jsonl` nella directory del caso da riprendere. Il prompt del caso diventa il turno utente successivo                                                                                                                             |
| `context.add_dirs`        | Directory dentro la directory del caso che Claude può leggere durante l'esecuzione, concesse in sola lettura                                                                                                                                        |
| `execution.prompt`        | Il prompt, quando mantieni l'intero caso in `case.yaml` e ometti `prompt.md`                                                                                                                                                                        |
| `graders`                 | Un elenco di grader, ognuno con un `name` più le stesse chiavi che un file `graders/*.md` prende nel frontmatter. Per i grader `llm`, metti la rubrica in `criteria`                                                                                |

<h3 id="grader-frontmatter">
  Frontmatter del grader
</h3>

Ogni file di grader sotto `graders/` prende queste chiavi nel frontmatter, più le opzioni per il suo tipo. Il nome del grader è il nome del file senza `.md`:

| Chiave   | Predefinito   | Scopo                                                                                                                                                                                                                      |
| :------- | :------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`   | richiesto     | Uno dei [tipi di grader](#grader-types)                                                                                                                                                                                    |
| `weight` | `1`           | Peso relativo nel punteggio dell'esecuzione. Qualsiasi numero positivo                                                                                                                                                     |
| `arm`    | non impostato | `with-only` esclude il grader dalla valutazione in un'[esecuzione a due arm](#compare-against-a-no-plugin-baseline); `both` forza un grader che Claude Code altrimenti escluderebbe ad essere valutato in entrambi gli arm |

<h4 id="what-a-grader-can-look-at">
  Cosa un grader può guardare
</h4>

I grader `regex` prendono un `target` e i grader `llm` prendono un `focus`. Entrambi accettano gli stessi valori:

| Valore                           | Cosa vede il grader                                                                                                                                                                                                                                                                                                          |
| :------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `last_message`                   | Il testo della risposta finale di Claude. Questo è il predefinito                                                                                                                                                                                                                                                            |
| `trace`                          | L'intera sessione come JSON, un messaggio per riga. Un grader `regex` vede ogni messaggio; un judge `llm` vede i primi 12 e gli ultimi 12. Le virgolette e le newline dentro di esso sono JSON-escaped, quindi una regex corrisponde a `\"` piuttosto che `"`                                                                |
| `files`                          | L'elenco dei percorsi che Claude ha creato durante l'esecuzione, uno per riga. Non i loro contenuti, e non i file che uno scaffold ha creato o che Claude ha solo modificato                                                                                                                                                 |
| `{ source: file, path: <path> }` | I contenuti di un file nello spazio di lavoro dopo l'esecuzione. Usalo per valutare quello che il plugin ha prodotto. Un file PNG, JPEG, GIF, o WebP viene mostrato a un judge `llm` come immagine. Un judge `llm` rifiuta altri file binari come `.pptx` o PDF; rendili come immagine o scrivili come testo e valuta quello |
| `mock_calls`                     | Ogni chiamata che Claude ha fatto a uno [strumento MCP mockato](#mock-mcp-servers), con il suo input e la risposta del mock                                                                                                                                                                                                  |

<h4 id="grader-types">
  Tipi di grader
</h4>

Ogni tipo di grader sotto elenca le sue opzioni e quando passa:

| Tipo          | Opzioni                               | Passa quando                                                                                                                                                                                                                                                        |
| :------------ | :------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `regex`       | `pattern`, `flags`, `match`, `target` | La regex JavaScript `pattern` viene trovata nel target. Imposta `match: not_contains` per richiedere assenza o `match: "count:N"` per richiedere esattamente N corrispondenze. Metti l'insensibilità alle maiuscole in `flags: i`; l'inline `(?i)` non è supportato |
| `tool_used`   | `tool`, `input_match`, `min`, `max`   | Il numero di chiamate a `tool` il cui input JSON-encoded corrisponde alla regex opzionale `input_match` è tra `min`, predefinito 1, e `max`, predefinito illimitato. Per asserire che uno strumento non è mai stato chiamato, imposta sia `min: 0` che `max: 0`     |
| `tool_order`  | `before`, `after`                     | Entrambi gli strumenti sono stati chiamati e la prima chiamata corrispondente a `before` precede la prima chiamata corrispondente a `after`. Ognuno è un nome di strumento o `{ tool, input_match }`                                                                |
| `file_exists` | `path`, `exists`                      | Un file che Claude ha creato corrisponde al glob `path`, o nessuno con `exists: false`. Solo i file creati durante l'esecuzione contano                                                                                                                             |
| `llm`         | `criteria`, `focus`                   | Un modello judge vota PASS sulla rubrica in almeno due di tre voti. Nel layout `.md` il corpo del file è i criteri                                                                                                                                                  |
| `baseline`    | `baseline_file`, `criteria`           | Un judge trova che l'esecuzione soddisfa i criteri almeno altrettanto bene della trascrizione di riferimento a `baseline_file`, un `.jsonl` nella directory del caso                                                                                                |

<h3 id="mock-files">
  File mock
</h3>

Un file `<tool>.md` sotto `mocks/<server>/` risponde a uno strumento. Il suo corpo è il risultato dello strumento, con sostituzioni `{{input.<field>}}` e `{{file:fixtures/<name>}}`. Il suo frontmatter accetta queste chiavi:

| Chiave       | Predefinito   | Scopo                                                                                                                                                                                                                                                                                                                           |
| :----------- | :------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `type`       | `fixed`       | `fixed` restituisce il corpo come scritto. `agent` tratta il corpo come istruzioni per un modello piccolo che gioca il server per l'esecuzione e vede le chiamate precedenti come storia                                                                                                                                        |
| `expect`     | non impostato | Una mappa da percorsi di input punteggiati a un nome di tipo come `string`, `number`, `boolean`, `array`, o `object`, un `/regex/`, un letterale, o un elenco di letterali consentiti. Una chiamata che lo viola interrompe l'esecuzione con punteggio 0 ed è riportata come `aborted` con il server, lo strumento, e il motivo |
| `error`      | `false`       | `fixed` solo. Restituisci il corpo come errore dello strumento                                                                                                                                                                                                                                                                  |
| `abort_when` | non impostato | `agent` solo. Prosa che elenca le uniche condizioni sotto le quali l'agente può interrompere l'esecuzione                                                                                                                                                                                                                       |

Due file opzionali si trovano accanto ai file dello strumento nella directory di un server:

* **`_server.md`**: un singolo mock `type: agent` che risponde a diversi strumenti, elencati nella sua chiave frontmatter `tools:`. Un `<tool>.md` per lo stesso strumento ha precedenza. Metti una guardia `expect:` sul singolo `<tool>.md`, non qui
* **`_tools.json`**: una risposta `tools/list` salvata dal vero server, in modo che gli strumenti mockati portino le loro vere descrizioni e schemi di input invece di un placeholder permissivo

La directory `mocks/` propria di un caso usa lo stesso layout e sostituisce i file della suite file per file.

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

Questi sono i problemi che gli autori incontrano più spesso, chiavi su quello che vedi.

<h3 id="plugin-eval-is-currently-in-early-access">
  "plugin eval is currently in early access"
</h3>

La tua build precede la disponibilità generale del comando. Esegui `claude update`, poi esegui di nuovo il comando in una sessione fresca.

<h3 id="plugin-eval-is-currently-unavailable">
  "plugin eval is currently unavailable"
</h3>

Anthropic ha disattivato il comando lato server. Nulla sulla tua macchina lo riattiva; esegui `claude update` e riprova in una sessione fresca più tardi.

<h3 id="is-not-a-trusted-plugin-directory-and-this-run-cannot-stop-to-ask-you-about-it">
  "is not a trusted plugin directory, and this run cannot stop to ask you about it"
</h3>

Questa è la prima esecuzione rispetto a una directory di cui Claude Code non ha ancora fiducia, e non può chiedere perché stdin o stdout non è un terminale, hai passato `--json`, o la variabile di ambiente `CI` è impostata su un valore vero come `true`. Esegui `claude plugin eval <dir>` una volta in un terminale e rispondi al prompt, o passa `--trust-plugin` se ti fidi del codice e della suite del plugin. Vedi [Cosa un'esecuzione può accedere](#security).

<h3 id="no-eval-cases-found">
  "No eval cases found"
</h3>

Nessun `<case>/prompt.md` o `<case>/case.yaml` esiste sotto la directory eval in vigore, o i tuoi filtri `--case` e `--tag` non hanno corrisposto a nessun caso. Esegui dalla root del plugin, o esegui `claude plugin eval init` per creare una suite.

<h3 id="the-baseline-arm-shows-no-plugin-or-delta-is-zero">
  Il baseline arm mostra nessun plugin, o delta è zero
</h3>

Se il riepilogo non ha colonna `W/OUT`, o il caso fallisce con "ablation requested but no plugin resolved", nessun plugin è stato trovato per il caso. Aggiungi `plugins: ["../.."]` al caso, dando il percorso dalla directory del caso alla directory del plugin.

Se il plugin è stato caricato e `Δ` è ancora vicino a zero con il tuo grader `tool_used: Skill` che fallisce, questo è di solito un risultato reale, significando che la `description` della skill non attiva sulla formulazione del prompt. Regola la descrizione e riesegui la stessa suite.

<h3 id="agent-type-’-’-not-found-for-one-of-your-plugin’s-agents">
  "Agent type '...' not found" for one of your plugin's agents
</h3>

Per impostazione predefinita, ogni caso viene eseguito sia con il tuo plugin che senza di esso, e le esecuzioni senza di esso sono la [baseline senza plugin](#the-no-plugin-baseline). Quando Claude invia uno degli agenti del tuo plugin in un'esecuzione baseline, la chiamata dello strumento Agent fallisce con `Agent type '<plugin>:<agent-name>' not found. Available agents: ...`. L'elenco nomina solo gli agenti che esistono senza il plugin, come i [subagenti integrati](/docs/it/sub-agents#built-in-subagents).

L'errore è previsto, poiché `Δ` confronta le tue esecuzioni del plugin rispetto alla baseline. Nel risultato JSON, le esecuzioni baseline sono sotto `cases[].arms.without`.

Nelle esecuzioni con il tuo plugin caricato, un caso che elenca `Agent` in `allowed_tools` può inviare uno degli agenti del tuo plugin per il suo nome con namespace, come `my-plugin:code-reviewer` per l'agente `code-reviewer` in un plugin denominato `my-plugin`. Per saltare le esecuzioni baseline, passa `--ablation none`.

<h3 id="everything-scores-zero-although-the-right-files-were-produced">
  Tutto ottiene un punteggio zero sebbene i file corretti siano stati prodotti
</h3>

I tuoi grader puntano a `files`, l'elenco dei percorsi creati, quando intendevi i contenuti del file. Usa `{ source: file, path: <path> }` come `target` o `focus`.

Separatamente, `file_exists` conta solo i file creati durante l'esecuzione, quindi un file che lo scaffold ha creato o che Claude ha solo modificato è invisibile ad esso; valuta i suoi contenuti, o usa `tool_used` su `Edit`.

<h3 id="a-regex-over-the-trace-doesn’t-match-text-i-can-see">
  Una regex sulla traccia non corrisponde al testo che posso vedere
</h3>

* **Target sbagliato**: il `target` predefinito è `last_message`, non la traccia.
* **Escape JSON**: quando fai il target `trace`, è JSON per riga, quindi le virgolette appaiono come `\"`.
* **Sintassi regex**: le regex usano la sintassi JavaScript, quindi metti `i` in `flags` piuttosto che scrivere `(?i)`.

<h3 id="tools-are-denied-mcp-tools-are-missing-or-bash-won’t-run">
  Gli strumenti vengono negati, gli strumenti MCP mancano, o Bash non verrà eseguito
</h3>

Qualsiasi cosa oltre il set di sola lettura ha bisogno della tua concessione, come `--allow-tools Bash Write`. I tuoi server MCP personali non caricano mai in un'esecuzione. I server del plugin stesso non si avviano a meno che non [opti per esso](#mock-mcp-servers), e i loro strumenti hanno quindi bisogno anche di una concessione `--allow-tools "mcp__plugin_<plugin>_<server>__*"`; uno strumento mockato non ha bisogno di nessuno dei due.

<h3 id="the-run-exits-1-but-the-results-look-fine">
  L'esecuzione esce 1 ma i risultati sembrano bene
</h3>

Il `--threshold` predefinito è 1.0, quindi il comando esce 1 quando qualsiasi caso ottiene un punteggio inferiore a perfetto. Imposta una soglia che corrisponda al punteggio che richiedi. L'uscita 1 copre anche un file di caso che non è stato caricato, che viene riportato su stderr sopra la tabella.

<h3 id="json-output-path-must-end-in-json">
  "--json output path must end in .json"
</h3>

Hai messo il target dopo `--json`, quindi è stato letto come il percorso di output. Metti il target per primo, come in `claude plugin eval . --json`, o dai a `--json` un percorso `.json` esplicito.

<h3 id="a-grader-shows-passed-false-under-a-run-that-scored-1-0">
  Un grader mostra passed: false sotto un'esecuzione che ha ottenuto un punteggio 1.0
</h3>

Quel grader è escluso dal punteggio per design in un'esecuzione a due arm, e il suo campo `scored` è `false`. Vedi [Punteggio rispetto alla baseline senza plugin](#compare-against-a-no-plugin-baseline).

<h3 id="runs-fail-with-a-usage-limit-or-rate-limit-error-partway-through">
  Le esecuzioni falliscono con un errore di limite di utilizzo o limite di velocità a metà strada
</h3>

Se il tuo account raggiunge il limite di utilizzo del piano o un limite di velocità API mentre una suite è in esecuzione, ogni esecuzione successiva termina con quell'errore, viene valutata su quello che ha prodotto, e di solito ottiene un punteggio 0. La suite finisce comunque e non è contrassegnata `partial`, quindi il risultato può sembrare una regressione. Controlla la colonna `NOTES` o `cases[].arms.with[].error` nel JSON per il messaggio di limite prima di fidarti dei punteggi, poi riesegui dopo che il limite si ripristina, con `--runs 1` o un filtro `--case` se hai bisogno di rimanere sotto di esso.

<h3 id="runs-time-out-or-hit-the-turn-cap">
  Le esecuzioni scadono o colpiscono il limite di turni
</h3>

I predefiniti sono 10 turni e 300 secondi. Aumenta `max_turns` e `timeout_seconds` nel caso per compiti che hanno bisogno di più, e usa `--max-cost-usd` come limite di costo piuttosto che limiti stretti per esecuzione.

<h2 id="see-also">
  Vedi anche
</h2>

* [Creare un plugin](/docs/it/plugins/create): costruisci il plugin che stai testando, e caricalo con `--plugin-dir` durante lo sviluppo
* [Riferimento dei comandi plugin](/docs/it/plugins/cli-reference#plugin-eval): le voci di comando `plugin eval` e `plugin eval init`. La chiave [`experimental.evals`](/docs/it/plugins/manifest-reference#fields) del manifesto è nel riferimento del manifesto
* [Skills](/docs/it/skills): come la descrizione di una skill decide quando Claude la invoca, che è quello che un caso che controlla se la skill si attiva sta misurando
* [Sandboxing](/docs/it/sandboxing): la sandbox a livello di OS che si applica quando concedi Bash a un'esecuzione
* [Pubblicare un plugin](/docs/it/plugins/publish): pubblica il plugin una volta che la sua suite passa
* [Misurare il costo e l'utilizzo del plugin](/docs/it/plugins/measure): quello che il plugin aggiunge al contesto di ogni sessione e se le persone lo usano ancora
