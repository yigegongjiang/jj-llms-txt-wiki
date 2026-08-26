> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Personalizza la tua barra di stato

> Configura una barra di stato personalizzata per monitorare l'utilizzo della finestra di contesto, i costi e lo stato git in Claude Code

La barra di stato è una barra personalizzabile nella parte inferiore di Claude Code che esegue qualsiasi script di shell che configuri. Riceve dati di sessione JSON su stdin e visualizza tutto ciò che il tuo script stampa, fornendoti una visualizzazione persistente e immediata dell'utilizzo del contesto, dei costi, dello stato git o di qualsiasi altra cosa tu voglia tracciare.

Le barre di stato sono utili quando:

* Vuoi monitorare l'utilizzo della finestra di contesto mentre lavori
* Hai bisogno di tracciare i costi della sessione
* Lavori su più sessioni e hai bisogno di distinguerle
* Vuoi che il ramo git e lo stato siano sempre visibili

La barra di stato viene renderizzata nella sua propria riga sopra i badge del footer integrati e non li sostituisce. Per aggiungere badge di collegamento cliccabili al footer quando un ID appare nella conversazione, senza scrivere uno script, configura invece [`footerLinksRegexes`](/docs/it/settings#footer-link-badges).

Ecco un esempio di una [barra di stato multi-riga](#display-multiple-lines) che visualizza le informazioni git sulla prima riga e una barra di contesto codificata a colori sulla seconda.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-multiline.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=60f11387658acc9ff75158ae85f2ac87" alt="Una barra di stato multi-riga che mostra il nome del modello, la directory, il ramo git sulla prima riga, e una barra di progresso dell'utilizzo del contesto con costo e durata sulla seconda riga" width="776" height="212" data-path="images/statusline-multiline.png" />
</Frame>

Questa pagina illustra come [configurare una barra di stato di base](#set-up-a-status-line), spiega [come fluiscono i dati](#how-status-lines-work) da Claude Code al tuo script, elenca [tutti i campi che puoi visualizzare](#available-data), e fornisce [esempi pronti all'uso](#examples) per modelli comuni come lo stato git, il tracciamento dei costi e le barre di progresso.

<h2 id="set-up-a-status-line">
  Configura una barra di stato
</h2>

Usa il [comando `/statusline`](#use-the-%2Fstatusline-command) per far generare uno script a Claude Code, oppure [crea manualmente uno script](#manually-configure-a-status-line) e aggiungilo alle tue impostazioni.

<h3 id="use-the-/statusline-command">
  Usa il comando /statusline
</h3>

Il comando `/statusline` accetta istruzioni in linguaggio naturale che descrivono cosa vuoi visualizzare. Claude Code genera un file di script in `~/.claude/` e aggiorna automaticamente le tue impostazioni:

```text theme={null}
/statusline show model name and context percentage with a progress bar
```

<h3 id="manually-configure-a-status-line">
  Configura manualmente una barra di stato
</h3>

Aggiungi un campo `statusLine` alle tue impostazioni utente (`~/.claude/settings.json`, dove `~` è la tua directory home) o alle [impostazioni del progetto](/docs/it/settings#settings-files). Imposta `type` su `"command"` e punta `command` a un percorso di script o a un comando di shell inline. Per una procedura dettagliata sulla creazione di uno script, vedi [Costruisci una barra di stato passo dopo passo](#build-a-status-line-step-by-step).

```json theme={null}
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/statusline.sh",
    "padding": 2
  }
}
```

Il campo `command` viene eseguito in una shell, quindi puoi anche usare comandi inline invece di un file di script. Questo esempio usa `jq` per analizzare l'input JSON e visualizzare il nome del modello e la percentuale di contesto:

```json theme={null}
{
  "statusLine": {
    "type": "command",
    "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'"
  }
}
```

Il campo opzionale `padding` aggiunge spazi orizzontali extra (in caratteri) al contenuto della barra di stato. Il valore predefinito è `0`. Questo padding è in aggiunta alla spaziatura integrata dell'interfaccia, quindi controlla l'indentazione relativa piuttosto che la distanza assoluta dal bordo del terminale.

Il campo opzionale `refreshInterval` esegue nuovamente il tuo comando ogni N secondi oltre agli [aggiornamenti guidati da eventi](#how-status-lines-work). Il minimo è `1`. Impostalo quando la tua barra di stato mostra dati basati sul tempo come un orologio, o quando i subagent in background cambiano lo stato git mentre la sessione principale è inattiva. Lascialo non impostato per eseguire solo su eventi.

Il campo opzionale `hideVimModeIndicator` sopprime il testo integrato `-- INSERT --` sotto il prompt. Impostalo su `true` quando il tuo script renderizza [`vim.mode`](#available-data) stesso, in modo che la modalità non venga visualizzata due volte.

<h3 id="disable-the-status-line">
  Disabilita la barra di stato
</h3>

Esegui `/statusline` e chiedigli di rimuovere o cancellare la tua barra di stato (ad esempio, `/statusline delete`, `/statusline clear`, `/statusline remove it`). Puoi anche eliminare manualmente il campo `statusLine` dal tuo settings.json.

<h2 id="build-a-status-line-step-by-step">
  Costruisci una barra di stato passo dopo passo
</h2>

Questa procedura mostra cosa sta accadendo dietro le quinte creando manualmente una barra di stato che visualizza il modello corrente, la directory di lavoro e la percentuale di utilizzo della finestra di contesto.

<Note>Eseguire [`/statusline`](#use-the-%2Fstatusline-command) con una descrizione di quello che vuoi configura tutto questo automaticamente per te.</Note>

Questi esempi usano script Bash, che funzionano su macOS e Linux. Su Windows, vedi [Configurazione Windows](#windows-configuration) per esempi PowerShell e Git Bash.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-quickstart.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=696445e59ca0059213250651ad23db6b" alt="Una barra di stato che mostra il nome del modello, la directory e la percentuale di contesto" width="726" height="164" data-path="images/statusline-quickstart.png" />
</Frame>

<Steps>
  <Step title="Crea uno script che legge JSON e stampa l'output">
    Claude Code invia dati JSON al tuo script tramite stdin. Questo script usa [`jq`](https://jqlang.github.io/jq/), un parser JSON da riga di comando che potrebbe essere necessario installare, per estrarre il nome del modello, la directory e la percentuale di contesto, quindi stampa una riga formattata.

    Salva questo in `~/.claude/statusline.sh` (dove `~` è la tua directory home, come `/Users/username` su macOS o `/home/username` su Linux):

    ```bash theme={null}
    #!/bin/bash
    # Read JSON data that Claude Code sends to stdin
    input=$(cat)

    # Extract fields using jq
    MODEL=$(echo "$input" | jq -r '.model.display_name')
    DIR=$(echo "$input" | jq -r '.workspace.current_dir')
    # The "// 0" provides a fallback if the field is null
    PCT=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)

    # Output the status line - ${DIR##*/} extracts just the folder name
    echo "[$MODEL] 📁 ${DIR##*/} | ${PCT}% context"
    ```
  </Step>

  <Step title="Rendilo eseguibile">
    Contrassegna lo script come eseguibile in modo che la tua shell possa eseguirlo:

    ```bash theme={null}
    chmod +x ~/.claude/statusline.sh
    ```
  </Step>

  <Step title="Aggiungi alle impostazioni">
    Dì a Claude Code di eseguire il tuo script come barra di stato. Aggiungi questa configurazione a `~/.claude/settings.json`, che imposta `type` su `"command"` (che significa "esegui questo comando di shell") e punta `command` al tuo script:

    ```json theme={null}
    {
      "statusLine": {
        "type": "command",
        "command": "~/.claude/statusline.sh"
      }
    }
    ```

    La tua barra di stato appare nella parte inferiore dell'interfaccia. Le impostazioni si ricaricano automaticamente, ma le modifiche non appariranno fino alla tua prossima interazione con Claude Code.
  </Step>
</Steps>

<h2 id="how-status-lines-work">
  Come funzionano le barre di stato
</h2>

Claude Code esegue il tuo script e invia i [dati di sessione JSON](#available-data) ad esso tramite stdin. Il tuo script legge il JSON, estrae quello di cui ha bisogno e stampa il testo su stdout. Claude Code visualizza tutto ciò che il tuo script stampa.

**Quando si aggiorna**

Il tuo script viene eseguito dopo ogni nuovo messaggio dell'assistente, dopo che `/compact` termina, quando cambia la modalità di autorizzazione o quando la modalità vim si attiva/disattiva. Gli aggiornamenti vengono debounced a 300ms, il che significa che i cambiamenti rapidi si raggruppano insieme e il tuo script viene eseguito una volta che le cose si stabilizzano. Se un nuovo aggiornamento si attiva mentre il tuo script è ancora in esecuzione, l'esecuzione in corso viene annullata. Se modifichi il tuo script, le modifiche non appariranno fino al prossimo aggiornamento di Claude Code.

Questi trigger possono diventare silenziosi quando la sessione principale è inattiva, ad esempio mentre un coordinatore attende i subagent in background. Per mantenere i segmenti basati sul tempo o provenienti da fonti esterne aggiornati durante i periodi di inattività, imposta [`refreshInterval`](#manually-configure-a-status-line) per eseguire nuovamente il comando anche su un timer fisso.

**Cosa può produrre il tuo script**

* **Più righe**: ogni istruzione `echo` o `print` viene visualizzata come una riga separata. Vedi l'[esempio multi-riga](#display-multiple-lines).
* **Colori**: usa [codici di escape ANSI](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors) come `\033[32m` per il verde (il terminale deve supportarli). Vedi l'[esempio di stato git](#git-status-with-colors).
* **Link**: usa [sequenze di escape OSC 8](https://en.wikipedia.org/wiki/ANSI_escape_code#OSC) per rendere il testo cliccabile (Cmd+clic su macOS, Ctrl+clic su Windows/Linux). Richiede un terminale che supporti i hyperlink come iTerm2, Kitty o WezTerm. Vedi l'[esempio di link cliccabili](#clickable-links).

**Ridimensionamento dell'output al terminale**

Claude Code acquisisce l'output del tuo script invece di collegarlo direttamente al terminale, quindi `tput cols` e il rilevamento della larghezza a livello di linguaggio non possono leggere la dimensione del terminale dall'interno dello script. Leggi invece le variabili di ambiente `COLUMNS` e `LINES`. Claude Code imposta queste variabili alle dimensioni attuali del terminale prima di eseguire il tuo script. Richiede Claude Code v2.1.153 o successivo.

<Note>La barra di stato viene eseguita localmente e non consuma token API. Si nasconde temporaneamente durante determinate interazioni dell'interfaccia utente, inclusi i suggerimenti di completamento automatico, il menu della guida e i prompt di autorizzazione.</Note>

<h2 id="available-data">
  Dati disponibili
</h2>

Claude Code invia i seguenti campi JSON al tuo script tramite stdin:

| Campo                                                                            | Descrizione                                                                                                                                                                                                                                                                                                                       |
| -------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `model.id`, `model.display_name`                                                 | Identificatore del modello corrente e nome visualizzato                                                                                                                                                                                                                                                                           |
| `cwd`, `workspace.current_dir`                                                   | Directory di lavoro corrente. Entrambi i campi contengono lo stesso valore; `workspace.current_dir` è preferito per coerenza con `workspace.project_dir`.                                                                                                                                                                         |
| `workspace.project_dir`                                                          | Directory in cui Claude Code è stato avviato, che potrebbe differire da `cwd` se la directory di lavoro cambia durante una sessione                                                                                                                                                                                               |
| `workspace.added_dirs`                                                           | Directory aggiuntive aggiunte tramite `/add-dir` o `--add-dir`. Array vuoto se nessuna è stata aggiunta                                                                                                                                                                                                                           |
| `workspace.git_worktree`                                                         | Nome del Git worktree quando la directory corrente si trova all'interno di un worktree collegato creato con `git worktree add`. Assente nel worktree principale. Popolato per qualsiasi git worktree, a differenza di `worktree.*` che si applica solo alle sessioni `--worktree`                                                 |
| `workspace.repo.host`, `workspace.repo.owner`, `workspace.repo.name`             | Identità del repository analizzata dal remote `origin`, ad esempio `"github.com"`, `"anthropics"`, `"claude-code"`. Assente al di fuori di un repository git o quando nessun remote `origin` è configurato                                                                                                                        |
| `cost.total_cost_usd`                                                            | Costo totale stimato della sessione in USD, calcolato lato client. Potrebbe differire dalla tua fattura effettiva                                                                                                                                                                                                                 |
| `cost.total_duration_ms`                                                         | Tempo totale trascorso dal momento dell'avvio della sessione, in millisecondi                                                                                                                                                                                                                                                     |
| `cost.total_api_duration_ms`                                                     | Tempo totale trascorso in attesa delle risposte API in millisecondi                                                                                                                                                                                                                                                               |
| `cost.total_lines_added`, `cost.total_lines_removed`                             | Righe di codice modificate                                                                                                                                                                                                                                                                                                        |
| `context_window.total_input_tokens`, `context_window.total_output_tokens`        | Conteggi dei token attualmente nella finestra di contesto, dalla risposta API più recente. L'input include letture e scritture della cache. Prima della v2.1.132 questi erano totali cumulativi della sessione                                                                                                                    |
| `context_window.context_window_size`                                             | Dimensione massima della finestra di contesto in token. 200000 per impostazione predefinita, o 1000000 per i modelli con contesto esteso.                                                                                                                                                                                         |
| `context_window.used_percentage`                                                 | Percentuale pre-calcolata della finestra di contesto utilizzata                                                                                                                                                                                                                                                                   |
| `context_window.remaining_percentage`                                            | Percentuale pre-calcolata della finestra di contesto rimanente                                                                                                                                                                                                                                                                    |
| `context_window.current_usage`                                                   | Conteggi dei token dall'ultima chiamata API, descritti in [campi della finestra di contesto](#context-window-fields)                                                                                                                                                                                                              |
| `exceeds_200k_tokens`                                                            | Se il conteggio totale dei token (token di input, cache e output combinati) dalla risposta API più recente supera 200k. Questo è un limite fisso indipendentemente dalla dimensione effettiva della finestra di contesto.                                                                                                         |
| `effort.level`                                                                   | Livello di sforzo di ragionamento corrente (`low`, `medium`, `high`, `xhigh`, o `max`). Riflette il valore della sessione attiva, inclusi i cambiamenti di `/effort` durante la sessione. Ultracode non è un livello distinto e viene segnalato come `xhigh`. Assente quando il modello corrente non supporta il parametro effort |
| `thinking.enabled`                                                               | Se il pensiero esteso è abilitato per la sessione                                                                                                                                                                                                                                                                                 |
| `rate_limits.five_hour.used_percentage`, `rate_limits.seven_day.used_percentage` | Percentuale del limite di velocità di 5 ore o 7 giorni consumato, da 0 a 100                                                                                                                                                                                                                                                      |
| `rate_limits.five_hour.resets_at`, `rate_limits.seven_day.resets_at`             | Secondi di epoca Unix quando la finestra del limite di velocità di 5 ore o 7 giorni si ripristina                                                                                                                                                                                                                                 |
| `session_id`                                                                     | Identificatore univoco della sessione                                                                                                                                                                                                                                                                                             |
| `session_name`                                                                   | Nome della sessione personalizzato impostato con il flag `--name` o `/rename`. Assente se nessun nome personalizzato è stato impostato                                                                                                                                                                                            |
| `prompt_id`                                                                      | UUID che identifica il prompt dell'utente attualmente in elaborazione. Corrisponde all'attributo [`prompt.id` sugli eventi OpenTelemetry](/docs/it/monitoring-usage#event-correlation-attributes). Assente fino al primo input dell'utente. Richiede Claude Code v2.1.196 o successivo                                                 |
| `transcript_path`                                                                | Percorso del file di trascrizione della conversazione                                                                                                                                                                                                                                                                             |
| `version`                                                                        | Versione di Claude Code                                                                                                                                                                                                                                                                                                           |
| `output_style.name`                                                              | Nome dello stile di output corrente                                                                                                                                                                                                                                                                                               |
| `vim.mode`                                                                       | Modalità vim corrente (`NORMAL`, `INSERT`, `VISUAL`, o `VISUAL LINE`) quando la [modalità vim](/docs/it/interactive-mode#vim-editor-mode) è abilitata                                                                                                                                                                                  |
| `agent.name`                                                                     | Nome dell'agente quando si esegue con il flag `--agent` o le impostazioni dell'agente configurate                                                                                                                                                                                                                                 |
| `pr.number`, `pr.url`                                                            | Pull request aperta per il ramo corrente. Rispecchia il badge PR nella barra di stato inferiore. Assente fino a quando non viene trovata una PR, quando non si è in un repository git, o una volta che la PR si unisce o si chiude                                                                                                |
| `pr.review_state`                                                                | Stato di revisione della PR aperta: `approved`, `pending`, `changes_requested`, o `draft`. Potrebbe essere indipendentemente assente anche quando `pr` è presente                                                                                                                                                                 |
| `worktree.name`                                                                  | Nome del worktree attivo. Presente solo durante le sessioni `--worktree`                                                                                                                                                                                                                                                          |
| `worktree.path`                                                                  | Percorso assoluto della directory del worktree                                                                                                                                                                                                                                                                                    |
| `worktree.branch`                                                                | Nome del ramo git per il worktree (ad esempio, `"worktree-my-feature"`). Assente per i worktree basati su hook                                                                                                                                                                                                                    |
| `worktree.original_cwd`                                                          | La directory in cui Claude si trovava prima di entrare nel worktree                                                                                                                                                                                                                                                               |
| `worktree.original_branch`                                                       | Ramo git estratto prima di entrare nel worktree. Assente per i worktree basati su hook                                                                                                                                                                                                                                            |

<Accordion title="Schema JSON completo">
  Il tuo comando della barra di stato riceve questa struttura JSON tramite stdin:

  ```json theme={null}
  {
    "cwd": "/current/working/directory",
    "session_id": "abc123...",
    "session_name": "my-session",
    "prompt_id": "550e8400-e29b-41d4-a716-446655440000",
    "transcript_path": "/path/to/transcript.jsonl",
    "model": {
      "id": "claude-opus-4-8",
      "display_name": "Opus"
    },
    "workspace": {
      "current_dir": "/current/working/directory",
      "project_dir": "/original/project/directory",
      "added_dirs": [],
      "git_worktree": "feature-xyz",
      "repo": {
        "host": "github.com",
        "owner": "anthropics",
        "name": "claude-code"
      }
    },
    "version": "2.1.90",
    "output_style": {
      "name": "default"
    },
    "cost": {
      "total_cost_usd": 0.01234,
      "total_duration_ms": 45000,
      "total_api_duration_ms": 2300,
      "total_lines_added": 156,
      "total_lines_removed": 23
    },
    "context_window": {
      "total_input_tokens": 15500,
      "total_output_tokens": 1200,
      "context_window_size": 200000,
      "used_percentage": 8,
      "remaining_percentage": 92,
      "current_usage": {
        "input_tokens": 8500,
        "output_tokens": 1200,
        "cache_creation_input_tokens": 5000,
        "cache_read_input_tokens": 2000
      }
    },
    "exceeds_200k_tokens": false,
    "effort": {
      "level": "high"
    },
    "thinking": {
      "enabled": true
    },
    "rate_limits": {
      "five_hour": {
        "used_percentage": 23.5,
        "resets_at": 1738425600
      },
      "seven_day": {
        "used_percentage": 41.2,
        "resets_at": 1738857600
      }
    },
    "vim": {
      "mode": "NORMAL"
    },
    "agent": {
      "name": "security-reviewer"
    },
    "pr": {
      "number": 1234,
      "url": "https://github.com/anthropics/claude-code/pull/1234",
      "review_state": "pending"
    },
    "worktree": {
      "name": "my-feature",
      "path": "/path/to/.claude/worktrees/my-feature",
      "branch": "worktree-my-feature",
      "original_cwd": "/path/to/project",
      "original_branch": "main"
    }
  }
  ```

  **Campi che potrebbero essere assenti** (non presenti in JSON):

  * `session_name`: appare solo quando un nome personalizzato è stato impostato con `--name` o `/rename`
  * `prompt_id`: appare solo dopo il primo input dell'utente
  * `workspace.git_worktree`: appare solo quando la directory corrente si trova all'interno di un git worktree collegato
  * `workspace.repo`: appare solo all'interno di un repository git con un remote `origin` configurato
  * `effort`: appare solo quando il modello corrente supporta il parametro di sforzo di ragionamento
  * `vim`: appare solo quando la modalità vim è abilitata
  * `agent`: appare solo quando si esegue con il flag `--agent` o le impostazioni dell'agente configurate
  * `pr`: appare solo mentre viene trovata una PR aperta per il ramo corrente, e viene rimossa una volta che la PR si unisce o si chiude. `pr.review_state` potrebbe essere indipendentemente assente
  * `worktree`: appare solo durante le sessioni `--worktree`. Quando presente, `branch` e `original_branch` potrebbero anche essere assenti per i worktree basati su hook
  * `rate_limits`: appare solo per gli abbonati Claude.ai (Pro/Max) dopo la prima risposta API nella sessione. Ogni finestra (`five_hour`, `seven_day`) potrebbe essere indipendentemente assente. Usa `jq -r '.rate_limits.five_hour.used_percentage // empty'` per gestire l'assenza con eleganza.

  **Campi che potrebbero essere `null`**:

  * `context_window.current_usage`: `null` prima della prima chiamata API in una sessione, e di nuovo dopo `/compact` fino a quando la prossima chiamata API non lo ripopola
  * `context_window.used_percentage`, `context_window.remaining_percentage`: potrebbero essere `null` all'inizio della sessione

  Gestisci i campi mancanti con accesso condizionale e i valori null con fallback predefiniti nei tuoi script.
</Accordion>

<h3 id="context-window-fields">
  Campi della finestra di contesto
</h3>

L'oggetto `context_window` descrive la finestra di contesto attiva dalla risposta API più recente. A partire dalla v2.1.132, `total_input_tokens` e `total_output_tokens` riflettono l'utilizzo del contesto corrente, non i totali cumulativi della sessione.

* **Totali combinati** (`total_input_tokens`, `total_output_tokens`): token attualmente nella finestra di contesto. `total_input_tokens` è la somma di `input_tokens`, `cache_creation_input_tokens` e `cache_read_input_tokens`; `total_output_tokens` sono i token di output dalla risposta più recente. Entrambi sono `0` prima della prima risposta API.
* **Utilizzo per componente** (`current_usage`): gli stessi conteggi dei token suddivisi per categoria. Usa questo quando hai bisogno di separare i cache hit dall'input fresco.

L'oggetto `current_usage` contiene:

* `input_tokens`: token di input nel contesto corrente
* `output_tokens`: token di output generati
* `cache_creation_input_tokens`: token scritti nella cache
* `cache_read_input_tokens`: token letti dalla cache

Per sapere cosa significano i campi della cache e come vengono fatturati, vedi [controlla le prestazioni della cache](/docs/it/prompt-caching#check-cache-performance).

Il campo `used_percentage` viene calcolato solo dai token di input: `input_tokens + cache_creation_input_tokens + cache_read_input_tokens`. Non include `output_tokens`.

Se calcoli manualmente la percentuale di contesto da `current_usage`, usa la stessa formula solo per l'input per corrispondere a `used_percentage`.

L'oggetto `current_usage` è `null` prima della prima chiamata API in una sessione, e di nuovo immediatamente dopo `/compact` fino a quando la prossima chiamata API non lo ripopola.

<h2 id="examples">
  Esempi
</h2>

Questi esempi mostrano modelli comuni della barra di stato. Per usare qualsiasi esempio:

1. Salva lo script in un file come `~/.claude/statusline.sh` (o `.py`/`.js`)
2. Rendilo eseguibile: `chmod +x ~/.claude/statusline.sh`
3. Aggiungi il percorso alle tue [impostazioni](#manually-configure-a-status-line)

Gli esempi Bash usano [`jq`](https://jqlang.github.io/jq/) per analizzare JSON. Python e Node.js hanno l'analisi JSON integrata.

<h3 id="context-window-usage">
  Utilizzo della finestra di contesto
</h3>

Visualizza il modello corrente e l'utilizzo della finestra di contesto con una barra di progresso visiva. Ogni script legge JSON da stdin, estrae il campo `used_percentage` e costruisce una barra di 10 caratteri dove i blocchi pieni (▓) rappresentano l'utilizzo:

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-context-window-usage.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=15b58ab3602f036939145dde3165c6f7" alt="Una barra di stato che mostra il nome del modello e una barra di progresso con percentuale" width="448" height="152" data-path="images/statusline-context-window-usage.png" />
</Frame>

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  # Read all of stdin into a variable
  input=$(cat)

  # Extract fields with jq, "// 0" provides fallback for null
  MODEL=$(echo "$input" | jq -r '.model.display_name')
  PCT=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)

  # Build progress bar: printf -v creates a run of spaces, then
  # ${var// /▓} replaces each space with a block character
  BAR_WIDTH=10
  FILLED=$((PCT * BAR_WIDTH / 100))
  EMPTY=$((BAR_WIDTH - FILLED))
  BAR=""
  [ "$FILLED" -gt 0 ] && printf -v FILL "%${FILLED}s" && BAR="${FILL// /▓}"
  [ "$EMPTY" -gt 0 ] && printf -v PAD "%${EMPTY}s" && BAR="${BAR}${PAD// /░}"

  echo "[$MODEL] $BAR $PCT%"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys

  # json.load reads and parses stdin in one step
  data = json.load(sys.stdin)
  model = data['model']['display_name']
  # "or 0" handles null values
  pct = int(data.get('context_window', {}).get('used_percentage', 0) or 0)

  # String multiplication builds the bar
  filled = pct * 10 // 100
  bar = '▓' * filled + '░' * (10 - filled)

  print(f"[{model}] {bar} {pct}%")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  // Node.js reads stdin asynchronously with events
  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      // Optional chaining (?.) safely handles null fields
      const pct = Math.floor(data.context_window?.used_percentage || 0);

      // String.repeat() builds the bar
      const filled = Math.floor(pct * 10 / 100);
      const bar = '▓'.repeat(filled) + '░'.repeat(10 - filled);

      console.log(`[${model}] ${bar} ${pct}%`);
  });
  ```
</CodeGroup>

<h3 id="git-status-with-colors">
  Stato git con colori
</h3>

Mostra il ramo git con indicatori codificati a colori per i file in staging e modificati. Questo script usa [codici di escape ANSI](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors) per i colori del terminale: `\033[32m` è verde, `\033[33m` è giallo e `\033[0m` ripristina il valore predefinito.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-git-context.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=e656f34f90d1d9a1d0e220988914345f" alt="Una barra di stato che mostra il modello, la directory, il ramo git e indicatori codificati a colori per i file in staging e modificati" width="742" height="178" data-path="images/statusline-git-context.png" />
</Frame>

Ogni script verifica se la directory corrente è un repository git, conta i file in staging e modificati e visualizza indicatori codificati a colori:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  DIR=$(echo "$input" | jq -r '.workspace.current_dir')

  GREEN='\033[32m'
  YELLOW='\033[33m'
  RESET='\033[0m'

  if git rev-parse --git-dir > /dev/null 2>&1; then
      BRANCH=$(git branch --show-current 2>/dev/null)
      STAGED=$(git diff --cached --numstat 2>/dev/null | wc -l | tr -d ' ')
      MODIFIED=$(git diff --numstat 2>/dev/null | wc -l | tr -d ' ')

      GIT_STATUS=""
      [ "$STAGED" -gt 0 ] && GIT_STATUS="${GREEN}+${STAGED}${RESET}"
      [ "$MODIFIED" -gt 0 ] && GIT_STATUS="${GIT_STATUS}${YELLOW}~${MODIFIED}${RESET}"

      echo -e "[$MODEL] 📁 ${DIR##*/} | 🌿 $BRANCH $GIT_STATUS"
  else
      echo "[$MODEL] 📁 ${DIR##*/}"
  fi
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, os

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  directory = os.path.basename(data['workspace']['current_dir'])

  GREEN, YELLOW, RESET = '\033[32m', '\033[33m', '\033[0m'

  try:
      subprocess.check_output(['git', 'rev-parse', '--git-dir'], stderr=subprocess.DEVNULL)
      branch = subprocess.check_output(['git', 'branch', '--show-current'], text=True).strip()
      staged_output = subprocess.check_output(['git', 'diff', '--cached', '--numstat'], text=True).strip()
      modified_output = subprocess.check_output(['git', 'diff', '--numstat'], text=True).strip()
      staged = len(staged_output.split('\n')) if staged_output else 0
      modified = len(modified_output.split('\n')) if modified_output else 0

      git_status = f"{GREEN}+{staged}{RESET}" if staged else ""
      git_status += f"{YELLOW}~{modified}{RESET}" if modified else ""

      print(f"[{model}] 📁 {directory} | 🌿 {branch} {git_status}")
  except:
      print(f"[{model}] 📁 {directory}")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const dir = path.basename(data.workspace.current_dir);

      const GREEN = '\x1b[32m', YELLOW = '\x1b[33m', RESET = '\x1b[0m';

      try {
          execSync('git rev-parse --git-dir', { stdio: 'ignore' });
          const branch = execSync('git branch --show-current', { encoding: 'utf8' }).trim();
          const staged = execSync('git diff --cached --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;
          const modified = execSync('git diff --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;

          let gitStatus = staged ? `${GREEN}+${staged}${RESET}` : '';
          gitStatus += modified ? `${YELLOW}~${modified}${RESET}` : '';

          console.log(`[${model}] 📁 ${dir} | 🌿 ${branch} ${gitStatus}`);
      } catch {
          console.log(`[${model}] 📁 ${dir}`);
      }
  });
  ```
</CodeGroup>

<h3 id="cost-and-duration-tracking">
  Tracciamento di costi e durata
</h3>

Traccia i costi API della tua sessione e il tempo trascorso. Il campo `cost.total_cost_usd` accumula il costo stimato di tutte le chiamate API nella sessione corrente. Il campo `cost.total_duration_ms` misura il tempo totale trascorso dall'inizio della sessione, mentre `cost.total_api_duration_ms` traccia solo il tempo trascorso in attesa delle risposte API.

Ogni script formatta il costo come valuta e converte i millisecondi in minuti e secondi:

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-cost-tracking.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=e3444a51fe6f3440c134bd5f1f08ad29" alt="Una barra di stato che mostra il nome del modello, il costo della sessione e la durata" width="588" height="180" data-path="images/statusline-cost-tracking.png" />
</Frame>

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  COST=$(echo "$input" | jq -r '.cost.total_cost_usd // 0')
  DURATION_MS=$(echo "$input" | jq -r '.cost.total_duration_ms // 0')

  COST_FMT=$(printf '$%.2f' "$COST")
  DURATION_SEC=$((DURATION_MS / 1000))
  MINS=$((DURATION_SEC / 60))
  SECS=$((DURATION_SEC % 60))

  echo "[$MODEL] 💰 $COST_FMT | ⏱️ ${MINS}m ${SECS}s"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  cost = data.get('cost', {}).get('total_cost_usd', 0) or 0
  duration_ms = data.get('cost', {}).get('total_duration_ms', 0) or 0

  duration_sec = duration_ms // 1000
  mins, secs = duration_sec // 60, duration_sec % 60

  print(f"[{model}] 💰 ${cost:.2f} | ⏱️ {mins}m {secs}s")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const cost = data.cost?.total_cost_usd || 0;
      const durationMs = data.cost?.total_duration_ms || 0;

      const durationSec = Math.floor(durationMs / 1000);
      const mins = Math.floor(durationSec / 60);
      const secs = durationSec % 60;

      console.log(`[${model}] 💰 $${cost.toFixed(2)} | ⏱️ ${mins}m ${secs}s`);
  });
  ```
</CodeGroup>

<h3 id="display-multiple-lines">
  Visualizza più righe
</h3>

Il tuo script può produrre più righe per creare una visualizzazione più ricca. Ogni istruzione `echo` produce una riga separata nell'area di stato.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-multiline.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=60f11387658acc9ff75158ae85f2ac87" alt="Una barra di stato multi-riga che mostra il nome del modello, la directory, il ramo git sulla prima riga, e una barra di progresso dell'utilizzo del contesto con costo e durata sulla seconda riga" width="776" height="212" data-path="images/statusline-multiline.png" />
</Frame>

Questo esempio combina diverse tecniche: colori basati su soglie (verde sotto il 70%, giallo 70-89%, rosso 90%+), una barra di progresso e informazioni sul ramo git. Ogni istruzione `print` o `echo` crea una riga separata:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  DIR=$(echo "$input" | jq -r '.workspace.current_dir')
  COST=$(echo "$input" | jq -r '.cost.total_cost_usd // 0')
  PCT=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)
  DURATION_MS=$(echo "$input" | jq -r '.cost.total_duration_ms // 0')

  CYAN='\033[36m'; GREEN='\033[32m'; YELLOW='\033[33m'; RED='\033[31m'; RESET='\033[0m'

  # Pick bar color based on context usage
  if [ "$PCT" -ge 90 ]; then BAR_COLOR="$RED"
  elif [ "$PCT" -ge 70 ]; then BAR_COLOR="$YELLOW"
  else BAR_COLOR="$GREEN"; fi

  FILLED=$((PCT / 10)); EMPTY=$((10 - FILLED))
  printf -v FILL "%${FILLED}s"; printf -v PAD "%${EMPTY}s"
  BAR="${FILL// /█}${PAD// /░}"

  MINS=$((DURATION_MS / 60000)); SECS=$(((DURATION_MS % 60000) / 1000))

  BRANCH=""
  git rev-parse --git-dir > /dev/null 2>&1 && BRANCH=" | 🌿 $(git branch --show-current 2>/dev/null)"

  echo -e "${CYAN}[$MODEL]${RESET} 📁 ${DIR##*/}$BRANCH"
  COST_FMT=$(printf '$%.2f' "$COST")
  echo -e "${BAR_COLOR}${BAR}${RESET} ${PCT}% | ${YELLOW}${COST_FMT}${RESET} | ⏱️ ${MINS}m ${SECS}s"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, os

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  directory = os.path.basename(data['workspace']['current_dir'])
  cost = data.get('cost', {}).get('total_cost_usd', 0) or 0
  pct = int(data.get('context_window', {}).get('used_percentage', 0) or 0)
  duration_ms = data.get('cost', {}).get('total_duration_ms', 0) or 0

  CYAN, GREEN, YELLOW, RED, RESET = '\033[36m', '\033[32m', '\033[33m', '\033[31m', '\033[0m'

  bar_color = RED if pct >= 90 else YELLOW if pct >= 70 else GREEN
  filled = pct // 10
  bar = '█' * filled + '░' * (10 - filled)

  mins, secs = duration_ms // 60000, (duration_ms % 60000) // 1000

  try:
      branch = subprocess.check_output(['git', 'branch', '--show-current'], text=True, stderr=subprocess.DEVNULL).strip()
      branch = f" | 🌿 {branch}" if branch else ""
  except:
      branch = ""

  print(f"{CYAN}[{model}]{RESET} 📁 {directory}{branch}")
  print(f"{bar_color}{bar}{RESET} {pct}% | {YELLOW}${cost:.2f}{RESET} | ⏱️ {mins}m {secs}s")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const dir = path.basename(data.workspace.current_dir);
      const cost = data.cost?.total_cost_usd || 0;
      const pct = Math.floor(data.context_window?.used_percentage || 0);
      const durationMs = data.cost?.total_duration_ms || 0;

      const CYAN = '\x1b[36m', GREEN = '\x1b[32m', YELLOW = '\x1b[33m', RED = '\x1b[31m', RESET = '\x1b[0m';

      const barColor = pct >= 90 ? RED : pct >= 70 ? YELLOW : GREEN;
      const filled = Math.floor(pct / 10);
      const bar = '█'.repeat(filled) + '░'.repeat(10 - filled);

      const mins = Math.floor(durationMs / 60000);
      const secs = Math.floor((durationMs % 60000) / 1000);

      let branch = '';
      try {
          branch = execSync('git branch --show-current', { encoding: 'utf8', stdio: ['pipe', 'pipe', 'ignore'] }).trim();
          branch = branch ? ` | 🌿 ${branch}` : '';
      } catch {}

      console.log(`${CYAN}[${model}]${RESET} 📁 ${dir}${branch}`);
      console.log(`${barColor}${bar}${RESET} ${pct}% | ${YELLOW}$${cost.toFixed(2)}${RESET} | ⏱️ ${mins}m ${secs}s`);
  });
  ```
</CodeGroup>

<h3 id="clickable-links">
  Link cliccabili
</h3>

Questo esempio crea un link cliccabile al tuo repository GitHub. Legge l'URL del remote git, converte il formato SSH in HTTPS con `sed` e avvolge il nome del repository nei codici di escape OSC 8. Tieni premuto Cmd (macOS) o Ctrl (Windows/Linux) e fai clic per aprire il link nel tuo browser.

<Frame>
  <img src="https://mintcdn.com/claude-code/nibzesLaJVh4ydOq/images/statusline-links.png?fit=max&auto=format&n=nibzesLaJVh4ydOq&q=85&s=4bcc6e7deb7cf52f41ab85a219b52661" alt="Una barra di stato che mostra un link cliccabile a un repository GitHub" width="726" height="198" data-path="images/statusline-links.png" />
</Frame>

Ogni script ottiene l'URL del remote git, converte il formato SSH in HTTPS e avvolge il nome del repository nei codici di escape OSC 8. La versione Bash usa `printf '%b'` che interpreta gli escape di backslash in modo più affidabile rispetto a `echo -e` su diverse shell:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')

  # Convert git SSH URL to HTTPS
  REMOTE=$(git remote get-url origin 2>/dev/null | sed 's/git@github.com:/https:\/\/github.com\//' | sed 's/\.git$//')

  if [ -n "$REMOTE" ]; then
      REPO_NAME=$(basename "$REMOTE")
      # OSC 8 format: \e]8;;URL\a then TEXT then \e]8;;\a
      # printf %b interprets escape sequences reliably across shells
      printf '%b' "[$MODEL] 🔗 \e]8;;${REMOTE}\a${REPO_NAME}\e]8;;\a\n"
  else
      echo "[$MODEL]"
  fi
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, re, os

  data = json.load(sys.stdin)
  model = data['model']['display_name']

  # Get git remote URL
  try:
      remote = subprocess.check_output(
          ['git', 'remote', 'get-url', 'origin'],
          stderr=subprocess.DEVNULL, text=True
      ).strip()
      # Convert SSH to HTTPS format
      remote = re.sub(r'^git@github\.com:', 'https://github.com/', remote)
      remote = re.sub(r'\.git$', '', remote)
      repo_name = os.path.basename(remote)
      # OSC 8 escape sequences
      link = f"\033]8;;{remote}\a{repo_name}\033]8;;\a"
      print(f"[{model}] 🔗 {link}")
  except:
      print(f"[{model}]")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;

      try {
          let remote = execSync('git remote get-url origin', { encoding: 'utf8', stdio: ['pipe', 'pipe', 'ignore'] }).trim();
          // Convert SSH to HTTPS format
          remote = remote.replace(/^git@github\.com:/, 'https://github.com/').replace(/\.git$/, '');
          const repoName = path.basename(remote);
          // OSC 8 escape sequences
          const link = `\x1b]8;;${remote}\x07${repoName}\x1b]8;;\x07`;
          console.log(`[${model}] 🔗 ${link}`);
      } catch {
          console.log(`[${model}]`);
      }
  });
  ```
</CodeGroup>

<h3 id="rate-limit-usage">
  Utilizzo del limite di velocità
</h3>

Visualizza l'utilizzo del limite di velocità dell'abbonamento Claude.ai nella barra di stato. L'oggetto `rate_limits` contiene `five_hour` (finestra mobile di 5 ore) e `seven_day` (finestra settimanale). Ogni finestra fornisce `used_percentage` (0-100) e `resets_at` (secondi di epoca Unix quando la finestra si ripristina).

Questo campo è presente solo per gli abbonati Claude.ai (Pro/Max) dopo la prima risposta API. Ogni script gestisce il campo assente con eleganza:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  # "// empty" produces no output when rate_limits is absent
  FIVE_H=$(echo "$input" | jq -r '.rate_limits.five_hour.used_percentage // empty')
  WEEK=$(echo "$input" | jq -r '.rate_limits.seven_day.used_percentage // empty')

  LIMITS=""
  [ -n "$FIVE_H" ] && LIMITS="5h: $(printf '%.0f' "$FIVE_H")%"
  [ -n "$WEEK" ] && LIMITS="${LIMITS:+$LIMITS }7d: $(printf '%.0f' "$WEEK")%"

  [ -n "$LIMITS" ] && echo "[$MODEL] | $LIMITS" || echo "[$MODEL]"
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys

  data = json.load(sys.stdin)
  model = data['model']['display_name']

  parts = []
  rate = data.get('rate_limits', {})
  five_h = rate.get('five_hour', {}).get('used_percentage')
  week = rate.get('seven_day', {}).get('used_percentage')

  if five_h is not None:
      parts.append(f"5h: {five_h:.0f}%")
  if week is not None:
      parts.append(f"7d: {week:.0f}%")

  if parts:
      print(f"[{model}] | {' '.join(parts)}")
  else:
      print(f"[{model}]")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;

      const parts = [];
      const fiveH = data.rate_limits?.five_hour?.used_percentage;
      const week = data.rate_limits?.seven_day?.used_percentage;

      if (fiveH != null) parts.push(`5h: ${Math.round(fiveH)}%`);
      if (week != null) parts.push(`7d: ${Math.round(week)}%`);

      console.log(parts.length ? `[${model}] | ${parts.join(' ')}` : `[${model}]`);
  });
  ```
</CodeGroup>

<h3 id="cache-expensive-operations">
  Memorizza nella cache le operazioni costose
</h3>

Il tuo script della barra di stato viene eseguito frequentemente durante le sessioni attive. Comandi come `git status` o `git diff` possono essere lenti, specialmente in repository di grandi dimensioni. Questo esempio memorizza nella cache le informazioni git in un file temporaneo e le aggiorna solo ogni 5 secondi.

Il nome del file di cache deve essere stabile tra le invocazioni della barra di stato all'interno di una sessione, ma univoco tra le sessioni in modo che le sessioni simultanee in repository diversi non leggano lo stato git memorizzato nella cache l'uno dell'altro. Gli identificatori basati su processi come `$$`, `os.getpid()` o `process.pid` cambiano ad ogni invocazione e annullano la cache. Usa invece `session_id` dall'input JSON: è stabile per la durata di una sessione ed è univoco per sessione.

Ogni script verifica se il file di cache è mancante o più vecchio di 5 secondi prima di eseguire i comandi git:

<CodeGroup>
  ```bash Bash theme={null}
  #!/bin/bash
  input=$(cat)

  MODEL=$(echo "$input" | jq -r '.model.display_name')
  DIR=$(echo "$input" | jq -r '.workspace.current_dir')
  SESSION_ID=$(echo "$input" | jq -r '.session_id')

  CACHE_FILE="/tmp/statusline-git-cache-$SESSION_ID"
  CACHE_MAX_AGE=5  # seconds

  cache_is_stale() {
      [ ! -f "$CACHE_FILE" ] || \
      # stat -f %m is macOS, stat -c %Y is Linux
      [ $(($(date +%s) - $(stat -f %m "$CACHE_FILE" 2>/dev/null || stat -c %Y "$CACHE_FILE" 2>/dev/null || echo 0))) -gt $CACHE_MAX_AGE ]
  }

  if cache_is_stale; then
      if git rev-parse --git-dir > /dev/null 2>&1; then
          BRANCH=$(git branch --show-current 2>/dev/null)
          STAGED=$(git diff --cached --numstat 2>/dev/null | wc -l | tr -d ' ')
          MODIFIED=$(git diff --numstat 2>/dev/null | wc -l | tr -d ' ')
          echo "$BRANCH|$STAGED|$MODIFIED" > "$CACHE_FILE"
      else
          echo "||" > "$CACHE_FILE"
      fi
  fi

  IFS='|' read -r BRANCH STAGED MODIFIED < "$CACHE_FILE"

  if [ -n "$BRANCH" ]; then
      echo "[$MODEL] 📁 ${DIR##*/} | 🌿 $BRANCH +$STAGED ~$MODIFIED"
  else
      echo "[$MODEL] 📁 ${DIR##*/}"
  fi
  ```

  ```python Python theme={null}
  #!/usr/bin/env python3
  import json, sys, subprocess, os, time

  data = json.load(sys.stdin)
  model = data['model']['display_name']
  directory = os.path.basename(data['workspace']['current_dir'])
  session_id = data['session_id']

  CACHE_FILE = f"/tmp/statusline-git-cache-{session_id}"
  CACHE_MAX_AGE = 5  # seconds

  def cache_is_stale():
      if not os.path.exists(CACHE_FILE):
          return True
      return time.time() - os.path.getmtime(CACHE_FILE) > CACHE_MAX_AGE

  if cache_is_stale():
      try:
          subprocess.check_output(['git', 'rev-parse', '--git-dir'], stderr=subprocess.DEVNULL)
          branch = subprocess.check_output(['git', 'branch', '--show-current'], text=True).strip()
          staged = subprocess.check_output(['git', 'diff', '--cached', '--numstat'], text=True).strip()
          modified = subprocess.check_output(['git', 'diff', '--numstat'], text=True).strip()
          staged_count = len(staged.split('\n')) if staged else 0
          modified_count = len(modified.split('\n')) if modified else 0
          with open(CACHE_FILE, 'w') as f:
              f.write(f"{branch}|{staged_count}|{modified_count}")
      except:
          with open(CACHE_FILE, 'w') as f:
              f.write("||")

  with open(CACHE_FILE) as f:
      branch, staged, modified = f.read().strip().split('|')

  if branch:
      print(f"[{model}] 📁 {directory} | 🌿 {branch} +{staged} ~{modified}")
  else:
      print(f"[{model}] 📁 {directory}")
  ```

  ```javascript Node.js theme={null}
  #!/usr/bin/env node
  const { execSync } = require('child_process');
  const fs = require('fs');
  const path = require('path');

  let input = '';
  process.stdin.on('data', chunk => input += chunk);
  process.stdin.on('end', () => {
      const data = JSON.parse(input);
      const model = data.model.display_name;
      const dir = path.basename(data.workspace.current_dir);
      const sessionId = data.session_id;

      const CACHE_FILE = `/tmp/statusline-git-cache-${sessionId}`;
      const CACHE_MAX_AGE = 5; // seconds

      const cacheIsStale = () => {
          if (!fs.existsSync(CACHE_FILE)) return true;
          return (Date.now() / 1000) - fs.statSync(CACHE_FILE).mtimeMs / 1000 > CACHE_MAX_AGE;
      };

      if (cacheIsStale()) {
          try {
              execSync('git rev-parse --git-dir', { stdio: 'ignore' });
              const branch = execSync('git branch --show-current', { encoding: 'utf8' }).trim();
              const staged = execSync('git diff --cached --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;
              const modified = execSync('git diff --numstat', { encoding: 'utf8' }).trim().split('\n').filter(Boolean).length;
              fs.writeFileSync(CACHE_FILE, `${branch}|${staged}|${modified}`);
          } catch {
              fs.writeFileSync(CACHE_FILE, '||');
          }
      }

      const [branch, staged, modified] = fs.readFileSync(CACHE_FILE, 'utf8').trim().split('|');

      if (branch) {
          console.log(`[${model}] 📁 ${dir} | 🌿 ${branch} +${staged} ~${modified}`);
      } else {
          console.log(`[${model}] 📁 ${dir}`);
      }
  });
  ```
</CodeGroup>

<h3 id="windows-configuration">
  Configurazione Windows
</h3>

Su Windows, Claude Code esegue i comandi della barra di stato tramite Git Bash quando Git Bash è installato, o tramite PowerShell quando Git Bash è assente.

Git Bash tratta i backslash non quotati come caratteri di escape, quindi un percorso in stile Windows come `C:\Users\username\script.mjs` raggiunge lo script runner con i suoi separatori rimossi e il comando fallisce senza un errore visibile. Scrivi i percorsi dei file nella stringa `command` con barre oblique, come mostrato negli esempi seguenti. La scorciatoia `~` funziona anche e si espande alla tua directory home di Windows.

Per eseguire uno script PowerShell come barra di stato, invocalo tramite `powershell`. Questo funziona indipendentemente dal fatto che Claude Code instrada il comando tramite Git Bash o PowerShell:

<CodeGroup>
  ```json settings.json theme={null}
  {
    "statusLine": {
      "type": "command",
      "command": "powershell -NoProfile -File C:/Users/username/.claude/statusline.ps1"
    }
  }
  ```

  ```powershell statusline.ps1 theme={null}
  $input_json = $input | Out-String | ConvertFrom-Json
  $cwd = $input_json.cwd
  $model = $input_json.model.display_name
  $used = $input_json.context_window.used_percentage
  $dirname = Split-Path $cwd -Leaf

  if ($used) {
      Write-Host "$dirname [$model] ctx: $used%"
  } else {
      Write-Host "$dirname [$model]"
  }
  ```
</CodeGroup>

Oppure, quando Git Bash è installato, esegui uno script Bash direttamente:

<CodeGroup>
  ```json settings.json theme={null}
  {
    "statusLine": {
      "type": "command",
      "command": "~/.claude/statusline.sh"
    }
  }
  ```

  ```bash statusline.sh theme={null}
  #!/usr/bin/env bash
  input=$(cat)
  cwd=$(echo "$input" | grep -o '"cwd":"[^"]*"' | cut -d'"' -f4)
  model=$(echo "$input" | grep -o '"display_name":"[^"]*"' | cut -d'"' -f4)
  dirname="${cwd##*[/\\]}"
  echo "$dirname [$model]"
  ```
</CodeGroup>

<h2 id="subagent-status-lines">
  Barre di stato dei subagent
</h2>

L'impostazione `subagentStatusLine` renderizza un corpo di riga personalizzato per ogni [subagent](/docs/it/sub-agents) mostrato nel pannello dell'agente sotto il prompt. Usalo per sostituire la riga predefinita `name · description · token count` con la tua formattazione.

```json theme={null}
{
  "subagentStatusLine": {
    "type": "command",
    "command": "~/.claude/subagent-statusline.sh"
  }
}
```

Il comando viene eseguito una volta per tick di aggiornamento con tutte le righe dei subagent visibili passate come un singolo oggetto JSON su stdin. L'input include i [campi hook comuni](/docs/it/hooks#common-input-fields), un campo `columns` con la larghezza di riga utilizzabile, e un array `tasks`. Ogni task ha `id`, `name`, `type`, `status`, `description`, `label`, `startTime`, `model`, `contextWindowSize`, `tokenCount`, `tokenSamples` e `cwd`.

Il campo `model` per-task è l'ID del modello risolto su cui viene eseguito il task. `contextWindowSize` è la finestra di contesto di quel modello in token, calcolata nello stesso modo della barra di stato principale `context_window.context_window_size`, quindi puoi renderizzare una percentuale per-riga da `tokenCount`. Entrambi i campi richiedono Claude Code v2.1.205 o successivo e vengono omessi per un task il cui modello non è ancora risolto.

Scrivi una riga JSON su stdout per ogni riga che vuoi sovrascrivere, nella forma `{"id": "<task id>", "content": "<row body>"}`. La stringa `content` viene renderizzata così com'è, inclusi i colori ANSI e i hyperlink OSC 8. Ometti l'`id` di un task per mantenere il rendering predefinito per quella riga; emetti una stringa `content` vuota per nasconderla.

Gli stessi gate di fiducia e `disableAllHooks` che si applicano a `statusLine` si applicano qui. I plugin possono fornire un `subagentStatusLine` predefinito nel loro [`settings.json`](/docs/it/plugins-reference#standard-plugin-layout).

<h2 id="tips">
  Suggerimenti
</h2>

* **Testa con input simulato**: `echo '{"model":{"display_name":"Opus"},"workspace":{"current_dir":"/home/user/project"},"context_window":{"used_percentage":25},"session_id":"test-session-abc"}' | ./statusline.sh`
* **Mantieni l'output breve**: la barra di stato ha una larghezza limitata, quindi l'output lungo potrebbe essere troncato o andare a capo in modo sgradevole
* **Memorizza nella cache le operazioni lente**: il tuo script viene eseguito frequentemente durante le sessioni attive, quindi comandi come `git status` possono causare lag. Vedi l'[esempio di caching](#cache-expensive-operations) per come gestire questo.

Progetti della comunità come [ccstatusline](https://github.com/sirmalloc/ccstatusline) e [starship-claude](https://github.com/martinemde/starship-claude) forniscono configurazioni pre-costruite con temi e funzionalità aggiuntive.

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

**La barra di stato non appare**

* Verifica che il tuo script sia eseguibile: `chmod +x ~/.claude/statusline.sh`
* Controlla che il tuo script stampi su stdout, non su stderr
* Esegui il tuo script manualmente per verificare che produca output
* Su Windows con Git Bash installato, i backslash nel percorso `command` vengono probabilmente consumati come caratteri di escape prima che lo script venga eseguito. Usa barre oblique nel percorso. Vedi [Configurazione Windows](#windows-configuration).
* Se `disableAllHooks` è impostato su `true` nelle tue impostazioni, anche la barra di stato è disabilitata. Rimuovi questa impostazione o impostala su `false` per riabilitarla.
* Esegui `claude --debug` per registrare il codice di uscita e stderr dalla prima invocazione della barra di stato in una sessione
* Chiedi a Claude di leggere il tuo file di impostazioni ed eseguire il comando `statusLine` direttamente per far emergere gli errori

**La barra di stato mostra `--` o valori vuoti**

* I campi potrebbero essere `null` prima che la prima risposta API si completi
* Gestisci i valori null nel tuo script con fallback come `// 0` in jq
* Riavvia Claude Code se i valori rimangono vuoti dopo più messaggi

**La percentuale di contesto mostra valori inaspettati**

* Usa `used_percentage` per lo stato di contesto più semplice e accurato
* La percentuale di contesto potrebbe differire dall'output `/context` a causa di quando ciascuno viene calcolato

**I link OSC 8 non sono cliccabili**

* Verifica che il tuo terminale supporti i hyperlink OSC 8 (iTerm2, Kitty, WezTerm)

* Terminal.app non supporta i link cliccabili

* Se il testo del link appare ma non è cliccabile, Claude Code potrebbe non aver rilevato il supporto dei hyperlink nel tuo terminale. Questo accade comunemente con Windows Terminal e altri emulatori non nell'elenco di rilevamento automatico. Imposta la variabile di ambiente `FORCE_HYPERLINK` per sovrascrivere il rilevamento prima di avviare Claude Code:

  ```bash theme={null}
  FORCE_HYPERLINK=1 claude
  ```

  In PowerShell, imposta la variabile nella sessione corrente prima:

  ```powershell theme={null}
  $env:FORCE_HYPERLINK = "1"; claude
  ```

* Le sessioni SSH e tmux potrebbero eliminare le sequenze OSC a seconda della configurazione

* Se le sequenze di escape appaiono come testo letterale come `\e]8;;`, usa `printf '%b'` invece di `echo -e` per una gestione più affidabile degli escape

**Glitch di visualizzazione con sequenze di escape**

* Le sequenze di escape complesse (colori ANSI, link OSC 8) possono occasionalmente causare output corrotto se si sovrappongono ad altri aggiornamenti dell'interfaccia utente
* Se vedi testo corrotto, prova a semplificare il tuo script in output di testo semplice
* Le barre di stato multi-riga con codici di escape sono più soggette a problemi di rendering rispetto al testo semplice su una sola riga

**Fiducia nell'area di lavoro richiesta**

* Il comando della barra di stato viene eseguito solo se hai accettato la finestra di dialogo di fiducia dell'area di lavoro per la directory corrente. Poiché `statusLine` esegue un comando di shell, richiede la stessa accettazione di fiducia di hook e altre impostazioni che eseguono shell.
* Se la fiducia non è accettata, vedrai la notifica `statusline skipped · restart to fix` invece dell'output della tua barra di stato. Riavvia Claude Code e accetta il prompt di fiducia per abilitarla.

**Errori di script o blocchi**

* Gli script che escono con codici diversi da zero o non producono output causano il vuoto della barra di stato
* Gli script lenti bloccano l'aggiornamento della barra di stato fino al completamento. Mantieni gli script veloci per evitare output obsoleto.
* Se un nuovo aggiornamento si attiva mentre uno script lento è in esecuzione, lo script in corso viene annullato
* Testa il tuo script indipendentemente con input simulato prima di configurarlo

**Le notifiche condividono la riga della barra di stato**

* Le notifiche di sistema come errori del server MCP e aggiornamenti automatici vengono visualizzate sul lato destro della stessa riga della tua barra di stato. Le notifiche transitorie come l'avviso di contesto basso si cicla anche attraverso questa area.
* L'abilitazione della modalità verbose aggiunge un contatore di token a questa area
* Su terminali stretti, queste notifiche potrebbero troncare l'output della tua barra di stato
