> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configura il tuo terminale per Claude Code

> Correggi Shift+Invio per le nuove righe, ricevi un segnale acustico del terminale quando Claude finisce, configura tmux, abbina il tema dei colori e abilita la modalità Vim nella CLI di Claude Code.

Claude Code funziona in qualsiasi terminale senza configurazione. Questa pagina è per quando qualcosa di specifico non si comporta come previsto. Trova il tuo sintomo di seguito. Se tutto ti sembra già corretto, non hai bisogno di questa pagina.

* [Shift+Invio invia invece di inserire una nuova riga](#enter-multiline-prompts)
* [Le scorciatoie del tasto Option non funzionano su macOS](#enable-option-key-shortcuts-on-macos)
* [Nessun suono o avviso quando Claude finisce](#get-a-terminal-bell-or-notification)
* [Esegui Claude Code dentro tmux](#configure-tmux)
* [Lo schermo sfarfalla o il scrollback salta](#switch-to-fullscreen-rendering)
* [Vuoi i tasti Vim nel prompt](#edit-prompts-with-vim-keybindings)

Questa pagina riguarda come far inviare al tuo terminale i segnali giusti a Claude Code. Per modificare i tasti a cui Claude Code stesso risponde, consulta invece [scorciatoie da tastiera](/docs/it/keybindings).

<h2 id="enter-multiline-prompts">
  Inserire prompt multilinea
</h2>

Premere Invio invia il tuo messaggio. Per aggiungere un'interruzione di riga senza inviare, premi Ctrl+J, oppure digita `\` e poi premi Invio. Entrambi funzionano in ogni terminale senza configurazione.

Nella maggior parte dei terminali puoi anche premere Shift+Invio, ma il supporto varia a seconda dell'emulatore di terminale:

| Terminale                                                               | Shift+Invio per nuova riga                  |
| :---------------------------------------------------------------------- | :------------------------------------------ |
| Ghostty, Kitty, iTerm2, WezTerm, Warp, Apple Terminal, Windows Terminal | Funziona senza configurazione               |
| VS Code, Cursor, Devin Desktop, Alacritty, Zed                          | Esegui `/terminal-setup` una volta          |
| gnome-terminal, JetBrains IDEs come PyCharm e Android Studio            | Non disponibile; usa Ctrl+J o `\` poi Invio |

Per VS Code, Cursor, Devin Desktop, Alacritty e Zed, `/terminal-setup` scrive Shift+Invio e altre scorciatoie da tastiera nel file di configurazione del terminale. I binding esistenti rimangono in posizione; se vedi un messaggio come `VSCode terminal Shift+Enter key binding already configured`, non è stata apportata alcuna modifica. Esegui `/terminal-setup` direttamente nel terminale host piuttosto che dentro tmux o screen, poiché ha bisogno di scrivere nella configurazione del terminale host.

In VS Code, Cursor e Devin Desktop, `/terminal-setup` aggiorna anche due impostazioni dell'editor: imposta `terminal.integrated.gpuAcceleration` su `"off"` per prevenire testo distorto nel terminale integrato, e imposta `terminal.integrated.mouseWheelScrollSensitivity` per uno scorrimento più fluido in [modalità a schermo intero](/docs/it/fullscreen). Per annullare la modifica dell'accelerazione GPU, impostala di nuovo su `"auto"` e ricarica la finestra dell'editor.

Se stai eseguendo dentro tmux, Shift+Invio richiede anche la [configurazione tmux di seguito](#configure-tmux) anche quando il terminale esterno la supporta.

Per associare la nuova riga a un tasto diverso, o per scambiare il comportamento in modo che Invio inserisca una nuova riga e Shift+Invio invii, mappa le azioni `chat:newline` e `chat:submit` nel tuo [file delle scorciatoie da tastiera](/docs/it/keybindings).

<h2 id="enable-option-key-shortcuts-on-macos">
  Abilita le scorciatoie da tastiera Option su macOS
</h2>

Alcuni scorciatoie di Claude Code utilizzano il tasto Option, come Option+Invio per una nuova riga o Option+P per cambiare modelli. Su macOS, la maggior parte dei terminali non invia Option come modificatore per impostazione predefinita, quindi questi scorciatoie non funzionano finché non lo abiliti. L'impostazione del terminale per questo è solitamente etichettata "Use Option as Meta Key"; Meta è il nome storico Unix per il tasto ora etichettato come Option o Alt.

<Tabs>
  <Tab title="Apple Terminal">
    Apri Impostazioni → Profili → Tastiera e seleziona "Use Option as Meta Key".

    Se hai accettato il prompt di primo avvio di Claude Code che offriva "Option+Invio per nuove righe e campanello visivo", questo è già fatto. Quel prompt esegue `/terminal-setup` per te, che abilita Option come Meta e cambia il campanello audio a un flash di schermo visivo nel tuo profilo Apple Terminal.
  </Tab>

  <Tab title="iTerm2">
    Apri Impostazioni → Profili → Tasti → Generale e imposta il tasto Option sinistro e il tasto Option destro su "Esc+".

    L'esecuzione di `/terminal-setup` in iTerm2 abilita "Applications in terminal may access clipboard" in Impostazioni → Generale → Selezione in modo che il comando `/copy` possa scrivere negli appunti di sistema. Il comando rileva iTerm2 anche quando eseguito da dentro tmux. Riavvia iTerm2 affinché la modifica abbia effetto.
  </Tab>

  <Tab title="VS Code">
    Aggiungi `"terminal.integrated.macOptionIsMeta": true` alle tue impostazioni di VS Code.
  </Tab>
</Tabs>

Per Ghostty, Kitty e altri terminali, cerca un'impostazione Option-as-Alt o Option-as-Meta nel file di configurazione del terminale.

<h2 id="get-a-terminal-bell-or-notification">
  Ottieni un campanello del terminale o una notifica
</h2>

Quando Claude finisce un'attività o si mette in pausa per un prompt di autorizzazione, attiva un evento di notifica. Visualizzare questo come un campanello del terminale o una notifica desktop ti consente di passare ad altro lavoro mentre un'attività lunga è in esecuzione.

Per impostazione predefinita Claude Code invia una notifica desktop solo in Ghostty, Kitty e iTerm2. In altri terminali, imposta [`preferredNotifChannel`](/docs/it/settings#available-settings) su `"terminal_bell"` per far suonare il campanello del terminale, oppure configura un [hook Notification](#play-a-sound-with-a-notification-hook) per un suono personalizzato o un comando.

La notifica desktop raggiunge la tua macchina locale tramite SSH, quindi una sessione remota può comunque avvertirti. Ghostty e Kitty la inoltrano al tuo centro notifiche del sistema operativo senza ulteriore configurazione. iTerm2 richiede che tu abiliti l'inoltro:

<Steps>
  <Step title="Apri le impostazioni di notifica di iTerm2">
    Vai a Impostazioni → Profili → Terminale.
  </Step>

  <Step title="Abilita gli avvisi">
    Seleziona "Notification Center Alerts", poi fai clic su "Filter Alerts" e abilita "Send escape sequence-generated alerts".
  </Step>
</Steps>

Se le notifiche ancora non appaiono, conferma che la tua applicazione di terminale ha il permesso di notifica nelle impostazioni del tuo sistema operativo, e se stai eseguendo dentro tmux, [abilita il passthrough](#configure-tmux).

<h3 id="play-a-sound-with-a-notification-hook">
  Play a sound with a Notification hook
</h3>

In qualsiasi terminale puoi configurare un [hook Notification](/docs/it/hooks-guide#get-notified-when-claude-needs-input) per riprodurre un suono o eseguire un comando personalizzato quando Claude ha bisogno della tua attenzione. Gli hook vengono eseguiti insieme alla notifica desktop piuttosto che sostituirla, quindi i terminali che non ricevono una notifica desktop, come Warp o il terminale integrato di VS Code, possono utilizzare un hook o impostare `preferredNotifChannel` su `"terminal_bell"` invece.

L'esempio di seguito riproduce un suono di sistema su macOS. La guida collegata ha comandi di notifica desktop per macOS, Linux e Windows.

```json ~/.claude/settings.json theme={null}
{
  "hooks": {
    "Notification": [
      {
        "hooks": [{ "type": "command", "command": "afplay /System/Library/Sounds/Glass.aiff" }]
      }
    ]
  }
}
```

<h2 id="configure-tmux">
  Configure tmux
</h2>

Quando Claude Code viene eseguito dentro tmux, due cose si rompono per impostazione predefinita: Shift+Invio invia invece di inserire una nuova riga, e le notifiche desktop e la [barra di avanzamento](/docs/it/settings#available-settings) non raggiungono mai il terminale esterno. Aggiungi queste righe a `~/.tmux.conf`, poi esegui `tmux source-file ~/.tmux.conf` per applicarle al server in esecuzione:

```bash ~/.tmux.conf theme={null}
set -g allow-passthrough on
set -s extended-keys on
set -as terminal-features 'xterm*:extkeys'
```

La riga `allow-passthrough` consente alle notifiche e agli aggiornamenti di avanzamento di raggiungere il terminale esterno invece di essere inghiottiti da tmux. Le righe `extended-keys` consentono a tmux di distinguere Shift+Invio da Invio semplice in modo che la scorciatoia della nuova riga funzioni.

<h2 id="match-the-color-theme">
  Abbina il tema dei colori
</h2>

Usa il comando `/theme`, o il selettore di tema in `/config`, per scegliere un tema di Claude Code che corrisponda al tuo terminale. Selezionando l'opzione auto rileva lo sfondo chiaro o scuro del tuo terminale, quindi il tema segue i cambiamenti di aspetto del sistema operativo ogni volta che il tuo terminale lo fa. Claude Code non controlla lo schema di colori del terminale stesso, che è impostato dall'applicazione del terminale.

Per personalizzare ciò che appare in fondo all'interfaccia, configura una [linea di stato personalizzata](/docs/it/statusline) che mostra il modello corrente, la directory di lavoro, il ramo git o altro contesto.

<h3 id="create-a-custom-theme">
  Crea un tema personalizzato
</h3>

<Note>
  I temi personalizzati richiedono Claude Code v2.1.118 o versioni successive.
</Note>

Oltre ai preset incorporati, `/theme` elenca tutti i temi personalizzati che hai definito e tutti i temi forniti dai [plugin](/docs/it/plugins-reference#themes) installati. Seleziona **Nuovo tema personalizzato…** alla fine dell'elenco per crearne uno in modo interattivo: dai un nome al tema, quindi scegli i singoli token di colore da sovrascrivere. Premi `Ctrl+E` mentre un tema personalizzato è evidenziato per modificarlo.

Ogni tema personalizzato è un file JSON in `~/.claude/themes/`. Il nome del file senza l'estensione `.json` è lo slug del tema, e selezionare il tema memorizza `custom:<slug>` come preferenza del tema. Il file ha tre campi facoltativi:

| Campo       | Tipo   | Descrizione                                                                                                                                         |
| :---------- | :----- | :-------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`      | string | Etichetta di visualizzazione mostrata in `/theme`. Predefinito al nome del file slug                                                                |
| `base`      | string | Preset incorporato da cui inizia il tema: `dark`, `light`, `dark-daltonized`, `light-daltonized`, `dark-ansi`, o `light-ansi`. Predefinito a `dark` |
| `overrides` | object | Mappa dei nomi dei token di colore ai valori di colore. I token non elencati qui passano al preset di base                                          |

I valori di colore accettano `#rrggbb`, `#rgb`, `rgb(r,g,b)`, `ansi256(n)`, o `ansi:<name>` dove `<name>` è uno dei 16 nomi di colore ANSI standard come `red` o `cyanBright`. I token sconosciuti e i valori di colore non validi vengono ignorati, quindi un errore di battitura non può interrompere il rendering.

L'esempio seguente definisce un tema che mantiene il preset scuro ma ricolora l'accento del prompt, il testo di errore e il testo di successo:

```json ~/.claude/themes/dracula.json theme={null}
{
  "name": "Dracula",
  "base": "dark",
  "overrides": {
    "claude": "#bd93f9",
    "error": "#ff5555",
    "success": "#50fa7b"
  }
}
```

Claude Code monitora `~/.claude/themes/` e ricarica quando un file cambia, quindi le modifiche apportate nel tuo editor si applicano a una sessione in esecuzione senza un riavvio.

Il riferimento di seguito copre i token che puoi impostare in `overrides`. L'editor interattivo in `/theme` mostra gli stessi token con un'anteprima dal vivo, più alcuni accenti monouso come i colori della schermata di onboarding che vengono omessi qui.

<Accordion title="Riferimento token di colore">
  L'esempio seguente combina token da diversi gruppi sottostanti: l'accento del marchio, il bordo della modalità piano, gli sfondi diff e lo sfondo del messaggio a schermo intero.

  ```json ~/.claude/themes/midnight.json theme={null}
  {
    "name": "Midnight",
    "base": "dark",
    "overrides": {
      "claude": "#a78bfa",
      "planMode": "#38bdf8",
      "diffAdded": "#14532d",
      "diffRemoved": "#7f1d1d",
      "userMessageBackground": "#1e1b4b"
    }
  }
  ```

  <h4 id="text-and-accent-colors">
    Colori di testo e accento
  </h4>

  Controlla l'accento del marchio principale e le sfumature di testo in primo piano utilizzate in tutta l'interfaccia.

  | Token         | Controlla                                                                               |
  | :------------ | :-------------------------------------------------------------------------------------- |
  | `claude`      | Accento del marchio principale, utilizzato per lo spinner e l'etichetta dell'assistente |
  | `text`        | Testo in primo piano predefinito                                                        |
  | `inverseText` | Testo disegnato sopra uno sfondo colorato, come i badge di stato                        |
  | `inactive`    | Testo secondario come suggerimenti, timestamp e elementi disabilitati                   |
  | `subtle`      | Bordi sfumati e testo secondario de-enfatizzato                                         |
  | `suggestion`  | Suggerimenti di completamento automatico e evidenziazione della selezione nei selettori |
  | `permission`  | Bordi della finestra di dialogo, inclusi i prompt di autorizzazione e i selettori       |
  | `remember`    | Indicatori di memoria e `CLAUDE.md`                                                     |

  <h4 id="status-colors">
    Colori di stato
  </h4>

  Segnala stati di successo, fallimento e avviso nei messaggi e negli indicatori.

  | Token     | Controlla                                                  |
  | :-------- | :--------------------------------------------------------- |
  | `success` | Messaggi di successo e controlli superati                  |
  | `error`   | Messaggi di errore e fallimenti                            |
  | `warning` | Avvisi, messaggi di cautela e il bordo della modalità auto |
  | `merged`  | Stato della richiesta pull unita                           |

  <h4 id="input-box-and-mode-indicators">
    Casella di input e indicatori di modalità
  </h4>

  Imposta il colore del bordo della casella di input e l'accento mostrato mentre una modalità di autorizzazione o un indicatore è attivo.

  | Token          | Controlla                                                                 |
  | :------------- | :------------------------------------------------------------------------ |
  | `promptBorder` | Bordo della casella di input nella modalità di autorizzazione predefinita |
  | `planMode`     | Accento e bordo della modalità piano                                      |
  | `autoAccept`   | Accento e bordo della modalità accetta-modifiche                          |
  | `bashBorder`   | Bordo della casella di input quando si immette un comando shell `!`       |
  | `ide`          | Indicatore di connessione IDE                                             |
  | `fastMode`     | Indicatore della modalità veloce                                          |

  <h4 id="diff-rendering">
    Rendering diff
  </h4>

  Colora il codice aggiunto e rimosso nelle modifiche e revisioni dei file.

  | Token               | Controlla                                                           |
  | :------------------ | :------------------------------------------------------------------ |
  | `diffAdded`         | Sfondo delle righe aggiunte                                         |
  | `diffRemoved`       | Sfondo delle righe rimosse                                          |
  | `diffAddedDimmed`   | Sfondo del contesto invariato vicino alle righe aggiunte            |
  | `diffRemovedDimmed` | Sfondo del contesto invariato vicino alle righe rimosse             |
  | `diffAddedWord`     | Evidenziazione a livello di parola all'interno di una riga aggiunta |
  | `diffRemovedWord`   | Evidenziazione a livello di parola all'interno di una riga rimossa  |

  <h4 id="fullscreen-mode">
    Modalità a schermo intero
  </h4>

  Applicare solo in [modalità di rendering a schermo intero](/docs/it/fullscreen), dove i messaggi hanno un riempimento di sfondo.

  | Token                        | Controlla                                                                    |
  | :--------------------------- | :--------------------------------------------------------------------------- |
  | `userMessageBackground`      | Sfondo dietro i tuoi messaggi nella trascrizione                             |
  | `userMessageBackgroundHover` | Sfondo dietro un messaggio mentre è al passaggio del mouse o espanso         |
  | `messageActionsBackground`   | Sfondo dietro il messaggio selezionato quando la barra delle azioni è aperta |
  | `bashMessageBackgroundColor` | Sfondo dietro le voci di comando shell `!` nella trascrizione                |
  | `memoryBackgroundColor`      | Sfondo dietro le voci di memoria `#` nella trascrizione                      |
  | `selectionBg`                | Sfondo del testo selezionato con il mouse                                    |

  <h4 id="usage-meter-and-speaker-labels">
    Misuratore di utilizzo e etichette degli altoparlanti
  </h4>

  Regola la barra mostrata nella vista `/usage` e le etichette che distinguono i tuoi messaggi da quelli di Claude.

  | Token              | Controlla                                                   |
  | :----------------- | :---------------------------------------------------------- |
  | `rate_limit_fill`  | Porzione riempita del misuratore di utilizzo                |
  | `rate_limit_empty` | Porzione non riempita del misuratore di utilizzo            |
  | `briefLabelYou`    | Colore dell'etichetta `You` sui tuoi messaggi               |
  | `briefLabelClaude` | Colore dell'etichetta `Claude` sui messaggi dell'assistente |

  <h4 id="shimmer-variants-and-subagent-colors">
    Varianti shimmer e colori dei subagent
  </h4>

  Diversi token hanno una variante shimmer accoppiata che fornisce il colore più chiaro utilizzato nel gradiente animato dello spinner. Sovrascrivi lo shimmer insieme al suo token di base se l'animazione appare non corrispondente.

  * `claude` e `claudeShimmer`
  * `warning` e `warningShimmer`
  * `permission` e `permissionShimmer`
  * `promptBorder` e `promptBorderShimmer`
  * `inactive` e `inactiveShimmer`
  * `fastMode` e `fastModeShimmer`

  Ogni [subagent](/docs/it/sub-agents) e attività parallela viene mostrata in uno degli otto colori denominati in modo che tu possa distinguerli nella trascrizione. I nomi dei token seguono il modello `<color>_FOR_SUBAGENTS_ONLY`, dove `<color>` è `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink`, o `cyan`. Sovrascrivi questi per cambiare l'aspetto di ogni colore denominato. Ad esempio, un subagent con `color: blue` nella sua definizione viene disegnato utilizzando il valore `blue_FOR_SUBAGENTS_ONLY`.

  Le parole chiave [`ultrathink`](/docs/it/model-config#use-ultrathink-for-one-off-deep-reasoning) e [`ultraplan`](/docs/it/ultraplan) nell'input del prompt vengono renderizzate con un gradiente arcobaleno a sette colori. I nomi dei token seguono il modello `rainbow_<color>` e `rainbow_<color>_shimmer`, dove `<color>` è `red`, `orange`, `yellow`, `green`, `blue`, `indigo`, o `violet`.
</Accordion>

<h2 id="switch-to-fullscreen-rendering">
  Passa al rendering a schermo intero
</h2>

Se lo schermo sfarfalla o la posizione di scorrimento salta mentre Claude sta lavorando, passa alla [modalità di rendering a schermo intero](/docs/it/fullscreen). Disegna su uno schermo separato che il terminale riserva per le app a schermo intero invece di aggiungere al tuo normale scrollback, il che mantiene l'utilizzo della memoria piatto e aggiunge il supporto del mouse per lo scorrimento e la selezione. In questa modalità scorri con il mouse o PageUp dentro Claude Code piuttosto che con lo scrollback nativo del tuo terminale; consulta la [pagina fullscreen](/docs/it/fullscreen#search-and-review-the-conversation) per come cercare e copiare.

Se lo sfarfallio è l'unico problema e il tuo terminale supporta l'output sincronizzato ma non viene rilevato automaticamente, come Emacs `eat`, imposta [`CLAUDE_CODE_FORCE_SYNC_OUTPUT=1`](/docs/it/env-vars) per interrompere lo sfarfallio senza cambiare renderer.

Esegui `/tui fullscreen` per passare e salvare la preferenza. La tua conversazione si riavvia intatta e le sessioni future iniziano a schermo intero. Puoi anche impostare la variabile di ambiente `CLAUDE_CODE_NO_FLICKER` prima di avviare Claude Code:

<CodeGroup>
  ```bash Bash and Zsh theme={null}
  CLAUDE_CODE_NO_FLICKER=1 claude
  ```

  ```powershell PowerShell theme={null}
  $env:CLAUDE_CODE_NO_FLICKER = "1"; claude
  ```

  ```json ~/.claude/settings.json theme={null}
  {
    "env": {
      "CLAUDE_CODE_NO_FLICKER": "1"
    }
  }
  ```
</CodeGroup>

<h2 id="paste-large-content">
  Incolla contenuti di grandi dimensioni
</h2>

Quando incolla più di 10.000 caratteri nel prompt, Claude Code comprime l'input a un placeholder `[Pasted text]` in modo che la casella di input rimanga utilizzabile. Il contenuto completo viene comunque inviato a Claude quando invii.

Il terminale integrato di VS Code può perdere caratteri da incollamenti molto grandi prima che raggiungano Claude Code, quindi preferisci flussi di lavoro basati su file lì. Per input molto grandi come interi file o lunghi log, scrivi il contenuto in un file e chiedi a Claude di leggerlo invece di incollare. Questo mantiene la trascrizione della conversazione leggibile e consente a Claude di fare riferimento al file per percorso nei turni successivi.

<h2 id="edit-prompts-with-vim-keybindings">
  Modifica i prompt con le scorciatoie da tastiera Vim
</h2>

Claude Code include una modalità di editing in stile Vim per l'input del prompt. Abilitala tramite `/config` → Editor mode, o impostando [`editorMode`](/docs/it/settings#available-settings) su `"vim"` in `~/.claude/settings.json`. Imposta Editor mode di nuovo su `normal` per disattivarla.

La modalità Vim supporta un sottoinsieme di motions in modalità NORMAL e VISUAL e operatori, come la navigazione `hjkl`, la selezione `v`/`V`, e `d`/`c`/`y` con oggetti di testo. Consulta il [riferimento della modalità editor Vim](/docs/it/interactive-mode#vim-editor-mode) per la tabella completa dei tasti.

I motions Vim non sono rimappabili tramite il file delle scorciatoie da tastiera. Per mappare una sequenza in modalità INSERT a due tasti come `jj` su Escape, imposta [`vimInsertModeRemaps`](/docs/it/interactive-mode#remap-insert-mode-key-sequences) nelle tue impostazioni utente.

Premere Invio invia comunque il tuo prompt in modalità INSERT, a differenza di Vim standard. Usa `o` o `O` in modalità NORMAL, o Ctrl+J, per inserire una nuova riga invece.

<h2 id="related-resources">
  Related resources
</h2>

* [Interactive mode](/docs/it/interactive-mode): riferimento completo delle scorciatoie da tastiera e la tabella dei tasti Vim
* [Keybindings](/docs/it/keybindings): rimappa qualsiasi scorciatoia di Claude Code, inclusi Invio e Shift+Invio
* [Fullscreen rendering](/docs/it/fullscreen): dettagli su scorrimento, ricerca e copia in modalità fullscreen
* [Hooks guide](/docs/it/hooks-guide): altri esempi di hook Notification per Linux e Windows
* [Troubleshooting](/docs/it/troubleshooting): correzioni per problemi al di fuori della configurazione del terminale
