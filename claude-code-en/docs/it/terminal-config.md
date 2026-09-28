> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configura il tuo terminale per Claude Code

> Correggi Shift+Invio per le nuove righe, ricevi un segnale acustico del terminale quando Claude termina, configura tmux, abbina il tema dei colori e abilita la modalità Vim nella CLI di Claude Code.

Claude Code funziona in qualsiasi terminale senza configurazione. Questa pagina è per quando qualcosa di specifico non si comporta come ti aspetti. Trova il tuo sintomo di seguito. Se tutto ti sembra già corretto, non hai bisogno di questa pagina.

* [Shift+Invio invia invece di inserire una nuova riga](#enter-multiline-prompts)
* [I tasti di scelta rapida Option non funzionano su macOS](#enable-option-key-shortcuts-on-macos)
* [Nessun suono o avviso quando Claude termina](#get-a-terminal-bell-or-notification)
* [Esegui Claude Code dentro tmux](#configure-tmux)
* [Backspace elimina un'intera parola su Windows](#fix-backspace-deleting-a-whole-word-on-windows)
* [La visualizzazione sfarfalla o lo scrollback salta](#switch-to-fullscreen-rendering)
* [Vuoi i tasti Vim nel prompt](#edit-prompts-with-vim-keybindings)

Questa pagina riguarda come far inviare al tuo terminale i segnali giusti a Claude Code. Per modificare i tasti a cui Claude Code stesso risponde, consulta invece [scorciatoie da tastiera](/docs/it/keybindings).

<h2 id="enter-multiline-prompts">
  Inserire prompt su più righe
</h2>

Premere Invio invia il messaggio. Per aggiungere un'interruzione di riga senza inviare, premere Ctrl+J, oppure digitare `\` e quindi premere Invio. Entrambi i metodi funzionano in ogni terminale senza alcuna configurazione.

Nella maggior parte dei terminali è anche possibile premere Maiusc+Invio, ma il supporto varia a seconda dell'emulatore di terminale:

| Terminale                                                                                                           | Maiusc+Invio per nuova riga                                                        |
| :------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------- |
| Ghostty, Kitty, iTerm2, WezTerm, Warp, Apple Terminal, Windows Terminal                                             | Funziona senza configurazione                                                      |
| Altri terminali che supportano il protocollo della tastiera kitty, come foot e Alacritty 0.16 o versioni successive | Funziona senza configurazione. Richiede Claude Code v2.1.269 o versioni successive |
| VS Code, Cursor, Devin Desktop, Alacritty prima della versione 0.16, Zed                                            | Eseguire `/terminal-setup` una volta                                               |
| gnome-terminal, JetBrains IDEs come PyCharm e Android Studio                                                        | Non disponibile; utilizzare Ctrl+J o `\` seguito da Invio                          |

Per VS Code, Cursor, Devin Desktop, Alacritty prima della versione 0.16 e Zed, `/terminal-setup` scrive una scorciatoia da tastiera Maiusc+Invio nel file di configurazione del terminale. Alla prima esecuzione viene visualizzato un messaggio di conferma come `Installed VSCode terminal Shift+Enter key binding`. Le associazioni esistenti rimangono in posizione; se viene visualizzato un messaggio come `VSCode terminal Shift+Enter key binding already configured`, non è stata apportata alcuna modifica. Eseguire `/terminal-setup` direttamente nel terminale host piuttosto che all'interno di tmux o screen, poiché deve scrivere nella configurazione del terminale host.

In VS Code, Cursor e Devin Desktop, `/terminal-setup` aggiorna anche due impostazioni dell'editor: imposta `terminal.integrated.gpuAcceleration` su `"off"` per evitare testo distorto nel terminale integrato, e imposta `terminal.integrated.mouseWheelScrollSensitivity` per uno scorrimento più fluido in [modalità a schermo intero](/docs/it/fullscreen). Per annullare la modifica dell'accelerazione GPU, impostarla di nuovo su `"auto"` e ricaricare la finestra dell'editor.

In Zed, `/terminal-setup` aggiorna il file `keymap.json` sul posto:

* Se la mappa dei tasti ha già associazioni e nessuna di esse è un Terminale `shift-enter`, Claude Code prima ne esegue il backup in una copia nella stessa directory, come `keymap.json.1a2b3c4d.bak`, quindi unisce l'associazione Maiusc+Invio nella mappa dei tasti, mantenendo le altre scorciatoie da tastiera e i commenti
* Se Claude Code non riesce a leggere o analizzare la mappa dei tasti, non riesce a eseguirne il backup, o non riesce a verificare il risultato unito, [lascia il file invariato e stampa il blocco di associazione da aggiungere manualmente](/docs/it/errors#terminal-setup-left-your-zed-keymap-unchanged)

Se si sta eseguendo all'interno di tmux, Maiusc+Invio richiede anche la [configurazione tmux di seguito](#configure-tmux) anche quando il terminale esterno la supporta.

Per associare l'interruzione di riga a un tasto diverso, o per scambiare il comportamento in modo che Invio inserisca un'interruzione di riga e Maiusc+Invio invii, mappare le azioni `chat:newline` e `chat:submit` nel file [scorciatoie da tastiera](/docs/it/keybindings).

<h2 id="enable-option-key-shortcuts-on-macos">
  Abilita le scorciatoie da tastiera Option su macOS
</h2>

Alcuni shortcut di Claude Code utilizzano il tasto Option, come Option+Invio per una nuova riga o Option+P per cambiare modelli. Su macOS, la maggior parte dei terminali non invia Option come modificatore per impostazione predefinita, quindi questi shortcut non funzionano finché non li abiliti. L'impostazione del terminale per questo è solitamente etichettata "Use Option as Meta Key"; Meta è il nome storico Unix per il tasto ora etichettato come Option o Alt.

<Tabs>
  <Tab title="Apple Terminal">
    Apri Impostazioni → Profili → Tastiera e seleziona "Use Option as Meta Key".

    Se hai accettato il prompt di configurazione del terminale al primo avvio di Claude Code, questo è già stato fatto. Quel prompt esegue `/terminal-setup` per te, che abilita Option come Meta e disattiva il campanello udibile nel tuo profilo Apple Terminal.

    In [modalità lettore schermo](/docs/it/accessibility), `/terminal-setup` lascia l'impostazione del campanello invariata in modo che il campanello del terminale rimanga udibile. Prima della v2.1.211, `/terminal-setup` disattivava il campanello anche in modalità lettore schermo. Se un'esecuzione precedente ha disattivato il campanello, riattivalo in Impostazioni → Profili → Avanzate → "Audible bell".
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
  Ricevere un campanello del terminale o una notifica
</h2>

Quando Claude termina un'attività o si mette in pausa per una richiesta di autorizzazione, e sembri essere lontano dal terminale, genera un evento di notifica. Consulta [quando ogni tipo di notifica viene attivato](/docs/it/hooks#notification) per i tempi esatti. Visualizzare questo come un campanello del terminale o una notifica desktop ti consente di passare ad altri lavori mentre un'attività lunga è in esecuzione.

Per impostazione predefinita, Claude Code invia una notifica desktop solo in Ghostty, Kitty e iTerm2. In altri terminali, imposta [`preferredNotifChannel`](/docs/it/settings-reference#preferrednotifchannel) su `"terminal_bell"` per far suonare il campanello del terminale, oppure configura un [hook Notification](#play-a-sound-with-a-notification-hook) per un suono personalizzato o un comando. La seguente voce di impostazioni attiva il campanello del terminale:

```json ~/.claude/settings.json theme={null}
{
  "preferredNotifChannel": "terminal_bell"
}
```

La notifica desktop raggiunge la tua macchina locale tramite SSH, quindi una sessione remota può comunque avvisarti. Ghostty e Kitty la inoltrano al centro notifiche del tuo sistema operativo senza ulteriore configurazione. iTerm2 richiede di abilitare l'inoltro:

<Steps>
  <Step title="Apri le impostazioni di notifica di iTerm2">
    Vai a Impostazioni → Profili → Terminale.
  </Step>

  <Step title="Abilita gli avvisi">
    Seleziona "Notification Center Alerts", quindi fai clic su "Filter Alerts" e abilita "Send escape sequence-generated alerts".
  </Step>
</Steps>

Se le notifiche non vengono ancora visualizzate, conferma che la tua applicazione terminale disponga dell'autorizzazione per le notifiche nelle impostazioni del sistema operativo, e se stai eseguendo tmux, [abilita il passthrough](#configure-tmux).

<h3 id="play-a-sound-with-a-notification-hook">
  Riproduci un suono con un hook Notification
</h3>

In qualsiasi terminale puoi configurare un [hook Notification](/docs/it/hooks-guide#get-notified-when-claude-needs-input) per riprodurre un suono o eseguire un comando personalizzato quando Claude ha bisogno della tua attenzione. Gli hook vengono eseguiti insieme alla notifica integrata anziché sostituirla, quindi i terminali che non ricevono una notifica desktop, come Warp o il terminale integrato di VS Code, possono utilizzare un hook o impostare `preferredNotifChannel` su `"terminal_bell"` invece.

L'esempio seguente riproduce un suono di sistema su macOS. La guida collegata contiene comandi di notifica desktop per macOS, Linux e Windows.

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
  Configurare tmux
</h2>

Quando Claude Code viene eseguito all'interno di tmux, per impostazione predefinita Shift+Invio invia il messaggio invece di inserire una nuova riga, e le notifiche desktop e la [barra di avanzamento](/docs/it/settings-reference#terminalprogressbarenabled) non raggiungono mai il terminale esterno. Aggiungere queste righe a `~/.tmux.conf`, quindi eseguire `tmux source-file ~/.tmux.conf` per applicarle al server in esecuzione:

```bash ~/.tmux.conf theme={null}
set -g allow-passthrough on
set -s extended-keys on
set -as terminal-features 'xterm*:extkeys'
```

La riga `allow-passthrough` consente alle notifiche e agli aggiornamenti di avanzamento di raggiungere il terminale esterno invece di essere inghiottiti da tmux. Le righe `extended-keys` consentono a tmux di distinguere Shift+Invio da Invio semplice in modo che la scorciatoia della nuova riga funzioni.

<h2 id="fix-backspace-deleting-a-whole-word-on-windows">
  Correggere l'eliminazione di una parola intera con Backspace su Windows
</h2>

Su Windows, Claude Code legge un Backspace che arriva come `^H` come Ctrl+Backspace, che [elimina la parola precedente](/docs/it/interactive-mode#text-editing), tranne quando `TERM_PROGRAM` è `mintty` o `TERM` è `cygwin`. Su macOS e Linux, Claude Code lo legge come semplice Backspace.

Se ogni pressione di Backspace elimina una parola intera, il vostro terminale invia `^H` per il semplice Backspace. Impostare [`CLAUDE_CODE_BS_AS_CTRL_BACKSPACE=0`](/docs/it/env-vars). Backspace e Ctrl+H quindi cancellano un carattere ciascuno. Se Ctrl+Backspace cancella solo un carattere su macOS o Linux perché il vostro terminale invia `^H` per esso, impostare la variabile a `1` invece.

<h2 id="match-the-color-theme">
  Abbina il tema dei colori
</h2>

Usa il comando `/theme`, oppure il selezionatore di tema in `/config`, per scegliere un tema Claude Code che corrisponda al tuo terminale. Selezionando l'opzione auto, il sistema rileva lo sfondo chiaro o scuro del tuo terminale, quindi il tema segue i cambiamenti dell'aspetto del sistema operativo ogni volta che il tuo terminale lo fa. Claude Code non controlla lo schema di colori del terminale stesso, che è impostato dall'applicazione del terminale.

Per personalizzare ciò che appare in fondo all'interfaccia, configura una [barra di stato personalizzata](/docs/it/statusline) che mostra il modello corrente, la directory di lavoro, il ramo git, o altri contesti.

<h3 id="create-a-custom-theme">
  Crea un tema personalizzato
</h3>

Oltre ai preset integrati, `/theme` elenca tutti i temi personalizzati che hai definito e tutti i temi forniti dai [plugin](/docs/it/plugins/components#themes-and-output-styles) installati. Seleziona **Nuovo tema personalizzato…** alla fine dell'elenco per crearne uno in modo interattivo: dai un nome al tema, quindi scegli i singoli token di colore da sovrascrivere. Premi `Ctrl+E` mentre un tema personalizzato è evidenziato per modificarlo.

Ogni tema personalizzato è un file JSON in `~/.claude/themes/`. Il nome del file senza l'estensione `.json` è lo slug del tema, e selezionare il tema memorizza `custom:<slug>` come preferenza del tema. Il file ha tre campi facoltativi:

| Campo       | Tipo   | Descrizione                                                                                                                                                        |
| :---------- | :----- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`      | string | Etichetta di visualizzazione mostrata in `/theme`. Per impostazione predefinita è lo slug del nome file                                                            |
| `base`      | string | Preset integrato da cui inizia il tema: `dark`, `light`, `dark-daltonized`, `light-daltonized`, `dark-ansi`, o `light-ansi`. Per impostazione predefinita è `dark` |
| `overrides` | object | Mappa dei nomi dei token di colore ai valori di colore. I token non elencati qui ricadono nel preset di base                                                       |

I valori di colore accettano `#rrggbb`, `#rgb`, `rgb(r,g,b)`, `ansi256(n)`, o `ansi:<name>` dove `<name>` è uno dei 16 nomi di colore ANSI standard come `red` o `cyanBright`. I token sconosciuti e i valori di colore non validi vengono ignorati, quindi un errore di battitura non può interrompere il rendering.

L'esempio seguente definisce un tema che mantiene il preset scuro ma ricolo il prompt accent, il testo di errore e il testo di successo:

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

Claude Code monitora `~/.claude/themes/` e ricarica quando un file viene aggiunto o modificato, quindi le modifiche apportate nel tuo editor si applicano a una sessione in esecuzione senza un riavvio. Se la cartella `~/.claude/themes/` stessa non esisteva quando Claude Code è stato avviato, riavvia una volta dopo aver creato il tuo primo file di tema. Dopo di che, le modifiche si applicano senza un riavvio.

Il riferimento di seguito copre i token che puoi impostare in `overrides`. L'editor interattivo in `/theme` mostra gli stessi token con un'anteprima dal vivo, più alcuni accent monouso come i colori della schermata di onboarding che sono omessi qui.

<Accordion title="Riferimento dei token di colore">
  L'esempio seguente combina token da diversi gruppi di seguito: l'accent del marchio, il bordo della modalità plan, gli sfondi diff e lo sfondo del messaggio.

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
    Colori di testo e accent
  </h4>

  Controlla l'accent del marchio primario e le sfumature di testo in primo piano utilizzate in tutta l'interfaccia.

  | Token         | Controlla                                                                            |
  | :------------ | :----------------------------------------------------------------------------------- |
  | `claude`      | Accent del marchio primario, utilizzato per lo spinner e l'etichetta dell'assistente |
  | `text`        | Testo in primo piano predefinito                                                     |
  | `inverseText` | Testo disegnato sopra uno sfondo colorato, come i badge di stato                     |
  | `inactive`    | Testo secondario come suggerimenti, timestamp e elementi disabilitati                |
  | `subtle`      | Bordi sfumati e testo secondario de-enfatizzato                                      |
  | `suggestion`  | Suggerimenti di completamento automatico e evidenziazione della selezione nei picker |
  | `permission`  | Bordi della finestra di dialogo, inclusi i prompt di autorizzazione e i picker       |
  | `remember`    | Indicatori di memoria e `CLAUDE.md`                                                  |

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

  Imposta il colore del bordo della casella di input e l'accent mostrato mentre una modalità di autorizzazione o un indicatore è attivo.

  | Token          | Controlla                                                                                                                                                                                               |
  | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
  | `promptBorder` | Bordo della casella di input                                                                                                                                                                            |
  | `planMode`     | Accent della modalità plan, messaggi della modalità plan e finestre di dialogo della modalità plan                                                                                                      |
  | `autoAccept`   | Accent della modalità accept-edits                                                                                                                                                                      |
  | `bashBorder`   | Bordo della casella di input quando si immette un comando shell `!`                                                                                                                                     |
  | `ide`          | Indicatore di connessione IDE                                                                                                                                                                           |
  | `fastMode`     | Indicatore della modalità fast                                                                                                                                                                          |
  | `effortUltra`  | Il tag `ultracode` sul bordo della casella di input mentre [ultracode](/docs/it/model-config#adjust-effort-level) è attivo. Il tuo override di questo colore ha effetto su Claude Code v2.1.239 o successivo |

  <h4 id="diff-rendering">
    Rendering diff
  </h4>

  Colora il codice aggiunto e rimosso nelle modifiche e revisioni dei file.

  | Token               | Controlla                                                                                |
  | :------------------ | :--------------------------------------------------------------------------------------- |
  | `diffAdded`         | Sfondo delle righe aggiunte                                                              |
  | `diffRemoved`       | Sfondo delle righe rimosse                                                               |
  | `diffAddedDimmed`   | Sfondo delle righe aggiunte nel diff attenuato mostrato dopo aver rifiutato una modifica |
  | `diffRemovedDimmed` | Sfondo delle righe rimosse nel diff attenuato mostrato dopo aver rifiutato una modifica  |
  | `diffAddedWord`     | Evidenziazione a livello di parola all'interno di una riga aggiunta                      |
  | `diffRemovedWord`   | Evidenziazione a livello di parola all'interno di una riga rimossa                       |

  <h4 id="fullscreen-mode">
    Modalità a schermo intero
  </h4>

  Claude Code dipinge `userMessageBackground`, `bashMessageBackgroundColor` e `memoryBackgroundColor` sia nei renderer predefiniti che a schermo intero. Utilizza `userMessageBackgroundHover` e `selectionBg` solo nella [modalità di rendering a schermo intero](/docs/it/fullscreen).

  | Token                        | Controlla                                                            |
  | :--------------------------- | :------------------------------------------------------------------- |
  | `userMessageBackground`      | Sfondo dietro i tuoi messaggi nella trascrizione                     |
  | `userMessageBackgroundHover` | Sfondo dietro un messaggio mentre è al passaggio del mouse o espanso |
  | `bashMessageBackgroundColor` | Sfondo dietro le voci di comando shell `!` nella trascrizione        |
  | `memoryBackgroundColor`      | Sfondo dietro le voci di memoria `#` nella trascrizione              |
  | `selectionBg`                | Sfondo del testo selezionato con il mouse                            |

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

  Diversi token hanno una variante shimmer accoppiata che fornisce il colore più chiaro utilizzato nel gradiente animato dello spinner. Sovrascrivi lo shimmer insieme al suo token di base se l'animazione sembra non corrispondere.

  * `claude` e `claudeShimmer`
  * `warning` e `warningShimmer`
  * `permission` e `permissionShimmer`
  * `promptBorder` e `promptBorderShimmer`
  * `inactive` e `inactiveShimmer`
  * `fastMode` e `fastModeShimmer`

  Ogni [subagent](/docs/it/sub-agents) e attività parallela è mostrato in uno degli otto colori denominati in modo che tu possa distinguerli nella trascrizione. I nomi dei token seguono il modello `<color>_FOR_SUBAGENTS_ONLY`, dove `<color>` è `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink`, o `cyan`. Sovrascrivi questi per cambiare l'aspetto di ogni colore denominato. Ad esempio, un subagent con `color: blue` nella sua definizione viene disegnato utilizzando il valore `blue_FOR_SUBAGENTS_ONLY`.

  Claude Code esegue il rendering della parola chiave [`ultrathink`](/docs/it/model-config#use-ultrathink-for-one-off-deep-reasoning) nell'input del prompt con un gradiente arcobaleno a sette colori. I nomi dei token seguono il modello `rainbow_<color>` e `rainbow_<color>_shimmer`, dove `<color>` è `red`, `orange`, `yellow`, `green`, `blue`, `indigo`, o `violet`.
</Accordion>

<h2 id="switch-to-fullscreen-rendering">
  Passa al rendering a schermo intero
</h2>

In [modalità lettore schermo](/docs/it/accessibility), questa sezione non si applica. Claude Code esegue sempre il rendering come testo scorrevole semplice tranne nelle [sessioni in background](/docs/it/agent-view) allegate, e se esegui `/tui fullscreen` in qualsiasi altra sessione, Claude Code stampa una spiegazione invece di passare.

Se il display sfarfalla o la posizione di scorrimento salta mentre Claude sta lavorando, passa a [modalità rendering a schermo intero](/docs/it/fullscreen). In questa modalità scorri con il mouse o PageUp all'interno di Claude Code piuttosto che con lo scrollback nativo del tuo terminale; consulta la [pagina fullscreen](/docs/it/fullscreen#search-and-review-the-conversation) per informazioni su come cercare e copiare.

Se lo sfarfallio è l'unico problema e il tuo terminale supporta l'output sincronizzato ma non viene rilevato automaticamente, come Emacs `eat`, imposta [`CLAUDE_CODE_FORCE_SYNC_OUTPUT=1`](/docs/it/env-vars) per interrompere lo sfarfallio senza cambiare renderer.

Esegui `/tui fullscreen` per passare e salvare la preferenza. La tua conversazione si riavvia intatta e le sessioni future iniziano a schermo intero a meno che un [avvio fullscreen non fallisca](/docs/it/fullscreen#fullscreen-renderer-didnt-finish-starting). Puoi anche impostare la variabile di ambiente `CLAUDE_CODE_NO_FLICKER` prima di avviare Claude Code:

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
  Incollare contenuti di grandi dimensioni
</h2>

Quando incollate più di 800 caratteri o più di tre righe nel prompt, Claude Code comprime l'input in un segnaposto come `[Pasted text #1 +120 lines]` in modo che la casella di input rimanga utilizzabile e invia comunque il contenuto completo quando inviate. Per input molto grandi come interi file o log lunghi, scrivete il contenuto in un file e chiedete a Claude di leggerlo invece di incollare. La trascrizione della conversazione rimane leggibile e Claude può fare riferimento al file per percorso nei turni successivi. Il terminale integrato di VS Code può anche perdere caratteri da incollamenti molto grandi prima che raggiungano Claude Code, quindi utilizzate un file lì.

Se l'incollamento contiene [caratteri Unicode invisibili](/docs/it/interactive-mode#invisible-characters-in-prompts), Claude Code li rimuove quando premete Invio e rimette il prompt pulito nella casella di input affinché lo inviate con un altro Invio.

<h3 id="how-claude-treats-pasted-text">
  Come Claude tratta il testo incollato
</h3>

Quando inviate, Claude vede il contenuto dietro ogni segnaposto `[Pasted text #N]` contrassegnato come testo che avete incollato da qualche parte piuttosto che digitato. Claude è informato che un incollamento può contenere istruzioni che non avete scritto e deve seguire le istruzioni al suo interno solo dove il messaggio che avete digitato lo chiede. Nelle sessioni che non [recuperano i flag delle funzionalità](/docs/it/env-vars#features-that-need-feature-flag-fetching), gli incollamenti non sono contrassegnati.

<h3 id="delete-and-restore-a-collapsed-paste">
  Eliminare e ripristinare un incollamento compresso
</h3>

Quando eliminate con una scorciatoia da tastiera di parola o riga come `Ctrl+W` o `Ctrl+K`, o con un'eliminazione vim attraverso un movimento `f`/`t` come `df]`, e l'intervallo eliminato raggiunge l'interno di un segnaposto `[Pasted text #N]`, Claude Code rimuove il segnaposto completamente. Per ripristinarlo, incollate l'eliminazione con [`Ctrl+Y`](/docs/it/interactive-mode#text-editing) dopo una scorciatoia da tastiera di parola o riga, o con [`p` in NORMAL mode](/docs/it/interactive-mode#editing-normal-mode) dopo un'eliminazione vim.

<h3 id="recall-a-prompt-that-had-pasted-text">
  Richiamare un prompt che aveva testo incollato
</h3>

Claude Code mantiene il contenuto dietro ogni segnaposto `[Pasted text #N]` in `~/.claude/paste-cache/`, quindi quando richiamate un prompt dalla [cronologia dei comandi](/docs/it/interactive-mode#command-history) e lo reinviate, il contenuto incollato completo viene inviato di nuovo, incluso in una sessione successiva.

I file della cache più vecchi di [`cleanupPeriodDays`](/docs/it/settings-reference#cleanupperioddays) vengono eliminati secondo le [regole di scansione di conservazione](/docs/it/claude-directory#cleaned-up-automatically), quindi un prompt richiamato può fare riferimento a testo incollato che non esiste più. Quando inviate un prompt di questo tipo, Claude Code non invia mai la stringa letterale `[Pasted text #N]` e mostra una notifica che nomina l'incollamento mancante:

* In un prompt semplice con testo rimanente, Claude Code rimuove il segnaposto e invia il testo rimanente.
* In un comando [shell mode](/docs/it/interactive-mode#shell-mode-with-prefix) o in un comando `/`, dove la rimozione cambierebbe ciò che viene eseguito, e in qualsiasi prompt la cui rimozione lascia vuoto, Claude Code annulla l'invio e mantiene il testo originale nell'input, con il segnaposto ancora in esso. Eliminate il segnaposto o modificate il comando, quindi reinviate.

<h2 id="edit-prompts-with-vim-keybindings">
  Modificare i prompt con le scorciatoie da tastiera di Vim
</h2>

Claude Code include una modalità di editing in stile Vim per l'input del prompt. Abilitatela tramite `/config` → Editor mode, oppure impostando [`editorMode`](/docs/it/settings-reference#editormode) su `"vim"` in `~/.claude/settings.json`. Impostate Editor mode di nuovo su `normal` per disabilitarla.

La modalità Vim supporta un sottoinsieme di movimenti e operatori in modalità NORMAL e VISUAL, come la navigazione `hjkl`, la selezione `v`/`V`, e `d`/`c`/`y` con text object. Consultate la [tabella completa delle scorciatoie da tastiera della modalità editor Vim](/docs/it/interactive-mode#vim-editor-mode) per la tabella completa dei tasti.

I movimenti di Vim non sono rimappabili tramite il file delle scorciatoie da tastiera. Per mappare una sequenza in modalità INSERT a due tasti come `jj` su Escape, impostate [`vimInsertModeRemaps`](/docs/it/interactive-mode#remap-insert-mode-key-sequences) nelle impostazioni utente.

Premere Invio continua a inviare il vostro prompt in modalità INSERT, a differenza del Vim standard. Utilizzate `o` oppure `O` in modalità NORMAL, o Ctrl+J, per inserire una nuova riga.

<h2 id="related-resources">
  Related resources
</h2>

* [Interactive mode](/docs/it/interactive-mode): riferimento completo delle scorciatoie da tastiera e la tabella dei tasti Vim
* [Keybindings](/docs/it/keybindings): rimappa qualsiasi scorciatoia di Claude Code, inclusi Invio e Shift+Invio
* [Fullscreen rendering](/docs/it/fullscreen): dettagli su scorrimento, ricerca e copia in modalità fullscreen
* [Hooks guide](/docs/it/hooks-guide): altri esempi di hook Notification per Linux e Windows
* [Troubleshooting](/docs/it/troubleshooting): correzioni per problemi al di fuori della configurazione del terminale
