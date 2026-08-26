> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Personalizzare le scorciatoie da tastiera

> Personalizzare le scorciatoie da tastiera in Claude Code con un file di configurazione keybindings.

Claude Code supporta scorciatoie da tastiera personalizzabili. Eseguire `/keybindings` per creare o aprire il file di configurazione in `~/.claude/keybindings.json`.

<h2 id="configuration-file">
  File di configurazione
</h2>

Il file di configurazione delle scorciatoie da tastiera è un oggetto con un array `bindings`. Ogni blocco specifica un contesto e una mappa di sequenze di tasti per le azioni.

<Note>Le modifiche al file keybindings vengono rilevate automaticamente e applicate senza riavviare Claude Code.</Note>

| Campo      | Descrizione                                                         |
| :--------- | :------------------------------------------------------------------ |
| `$schema`  | URL dello schema JSON opzionale per l'autocompletamento dell'editor |
| `$docs`    | URL della documentazione opzionale                                  |
| `bindings` | Array di blocchi di binding per contesto                            |

Questo esempio associa `Ctrl+E` per aprire un editor esterno nel contesto della chat e annulla l'associazione di `Ctrl+U`:

```json theme={null}
{
  "$schema": "https://www.schemastore.org/claude-code-keybindings.json",
  "$docs": "https://code.claude.com/docs/it/keybindings",
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "ctrl+e": "chat:externalEditor",
        "ctrl+u": null
      }
    }
  ]
}
```

<h2 id="contexts">
  Contesti
</h2>

Ogni blocco di binding specifica un **contesto** dove si applicano i binding:

| Contesto          | Descrizione                                                                        |
| :---------------- | :--------------------------------------------------------------------------------- |
| `Global`          | Si applica ovunque nell'app                                                        |
| `Chat`            | Area di input della chat principale                                                |
| `Autocomplete`    | Menu di autocompletamento è aperto                                                 |
| `Settings`        | Menu delle impostazioni                                                            |
| `Confirmation`    | Dialoghi di permesso e conferma                                                    |
| `Tabs`            | Componenti di navigazione delle schede                                             |
| `Help`            | Menu della guida è visibile                                                        |
| `Transcript`      | Visualizzatore di trascrizione                                                     |
| `HistorySearch`   | Modalità di ricerca nella cronologia (Ctrl+R)                                      |
| `Task`            | Attività in background in esecuzione                                               |
| `ThemePicker`     | Dialogo di selezione del tema                                                      |
| `Attachments`     | Navigazione degli allegati nei dialoghi di selezione                               |
| `Footer`          | Navigazione dell'indicatore di piè di pagina (attività, team, diff)                |
| `MessageSelector` | Selezione dei messaggi nella finestra di dialogo di riavvolgimento e riepilogo     |
| `DiffDialog`      | Navigazione del visualizzatore diff                                                |
| `ModelPicker`     | Livello di sforzo del selezionatore di modelli                                     |
| `Select`          | Componenti generici di selezione/elenco                                            |
| `Plugin`          | Dialogo dei plugin (sfoglia, scopri, gestisci)                                     |
| `Scroll`          | Scorrimento della conversazione e selezione del testo in modalità a schermo intero |

Prima della v2.1.205, un contesto `Doctor` e un'azione `doctor:fix` esistevano per la schermata diagnostica `/doctor`.

<h2 id="available-actions">
  Azioni disponibili
</h2>

Le azioni seguono un formato `namespace:action`, come `chat:submit` per inviare un messaggio o `app:toggleTodos` per mostrare l'elenco delle attività. Ogni contesto ha azioni specifiche disponibili.

<h3 id="app-actions">
  Azioni dell'app
</h3>

Azioni disponibili nel contesto `Global`:

| Azione                 | Predefinito     | Descrizione                                                                                                                        |
| :--------------------- | :-------------- | :--------------------------------------------------------------------------------------------------------------------------------- |
| `app:interrupt`        | Ctrl+C          | Annulla l'operazione corrente                                                                                                      |
| `app:exit`             | Ctrl+D          | Esci da Claude Code                                                                                                                |
| `app:redraw`           | (non associato) | Forza il ridisegno del terminale                                                                                                   |
| `app:toggleTodos`      | Ctrl+T          | Attiva/disattiva la visibilità dell'elenco delle attività di Claude. Questo non è il [`/tasks`](/docs/it/commands) background-task view |
| `app:toggleTranscript` | Ctrl+O          | Attiva/disattiva la trascrizione dettagliata                                                                                       |

<h3 id="history-actions">
  Azioni della cronologia
</h3>

Azioni per navigare nella cronologia dei comandi:

| Azione             | Predefinito | Descrizione                          |
| :----------------- | :---------- | :----------------------------------- |
| `history:search`   | Ctrl+R      | Apri ricerca nella cronologia        |
| `history:previous` | Su          | Elemento della cronologia precedente |
| `history:next`     | Giù         | Elemento della cronologia successivo |

<h3 id="chat-actions">
  Azioni della chat
</h3>

Azioni disponibili nel contesto `Chat`:

| Azione                | Predefinito                     | Descrizione                                                                                                                                                                                |
| :-------------------- | :------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `chat:cancel`         | Escape                          | Annulla l'input corrente                                                                                                                                                                   |
| `chat:clearInput`     | Ctrl+L                          | Forza un ridisegno a schermo intero, preservando l'input. Nel [rendering a schermo intero](/docs/it/fullscreen#clear-the-conversation), premi due volte entro due secondi per eseguire `/clear` |
| `chat:clearScreen`    | Cmd+K                           | Nel [rendering a schermo intero](/docs/it/fullscreen#clear-the-conversation), premi due volte entro due secondi per eseguire `/clear`                                                           |
| `chat:killAgents`     | Ctrl+X Ctrl+K                   | Termina tutti gli [agenti in background](/docs/it/sub-agents#run-subagents-in-foreground-or-background) in esecuzione in questa sessione                                                        |
| `chat:cycleMode`      | Shift+Tab\*                     | Cicla le modalità di permesso                                                                                                                                                              |
| `chat:modelPicker`    | Meta+P                          | Apri il selezionatore di modelli                                                                                                                                                           |
| `chat:fastMode`       | Meta+O                          | Attiva/disattiva la modalità veloce                                                                                                                                                        |
| `chat:thinkingToggle` | Meta+T                          | Attiva/disattiva il pensiero esteso                                                                                                                                                        |
| `chat:submit`         | Invio                           | Invia il messaggio                                                                                                                                                                         |
| `chat:newline`        | Ctrl+J                          | Inserisci una nuova riga senza inviare                                                                                                                                                     |
| `chat:undo`           | Ctrl+\_, Ctrl+Shift+-           | Annulla l'ultima azione                                                                                                                                                                    |
| `chat:externalEditor` | Ctrl+G, Ctrl+X Ctrl+E           | Apri nell'editor esterno                                                                                                                                                                   |
| `chat:stash`          | Ctrl+S                          | Nascondi il prompt corrente                                                                                                                                                                |
| `chat:imagePaste`     | Ctrl+V (Alt+V su Windows e WSL) | Incolla immagine dagli appunti. Su WSL, entrambe le scorciatoie sono associate per impostazione predefinita                                                                                |

\*Su Windows senza modalità VT (Node \<24.2.0/\<22.17.0, Bun \<1.2.23), il valore predefinito è Meta+M.

<h3 id="autocomplete-actions">
  Azioni di autocompletamento
</h3>

Azioni disponibili nel contesto `Autocomplete`:

| Azione                  | Predefinito | Descrizione             |
| :---------------------- | :---------- | :---------------------- |
| `autocomplete:accept`   | Tab         | Accetta il suggerimento |
| `autocomplete:dismiss`  | Escape      | Chiudi il menu          |
| `autocomplete:previous` | Su          | Suggerimento precedente |
| `autocomplete:next`     | Giù         | Suggerimento successivo |

<h3 id="confirmation-actions">
  Azioni di conferma
</h3>

Azioni disponibili nel contesto `Confirmation`:

| Azione                      | Predefinito     | Descrizione                                                                                                                                     |
| :-------------------------- | :-------------- | :---------------------------------------------------------------------------------------------------------------------------------------------- |
| `confirm:yes`               | Y, Invio        | Conferma l'azione                                                                                                                               |
| `confirm:no`                | N, Escape       | Rifiuta l'azione                                                                                                                                |
| `confirm:previous`          | Su              | Opzione precedente                                                                                                                              |
| `confirm:next`              | Giù             | Opzione successiva                                                                                                                              |
| `confirm:nextField`         | Tab             | Campo successivo                                                                                                                                |
| `confirm:previousField`     | (non associato) | Campo precedente                                                                                                                                |
| `confirm:toggle`            | Spazio          | Attiva/disattiva la selezione                                                                                                                   |
| `confirm:cycleMode`         | Shift+Tab       | Cicla le modalità di permesso                                                                                                                   |
| `confirm:toggleExplanation` | Ctrl+E          | Attiva/disattiva una spiegazione generata dal modello del [comando](/docs/it/permissions#permission-system) sui prompt di permesso Bash e PowerShell |

<h3 id="permission-actions">
  Azioni di permesso
</h3>

Azioni disponibili nel contesto `Confirmation` per i dialoghi di permesso:

| Azione                   | Predefinito     | Descrizione                                                                                                                                                  |
| :----------------------- | :-------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permission:toggleDebug` | (non associato) | Attiva/disattiva le informazioni di debug del permesso. Il valore predefinito precedente di Ctrl+D è stato rimosso nella v2.1.146 perché oscurava `app:exit` |

<h3 id="transcript-actions">
  Azioni di trascrizione
</h3>

Azioni disponibili nel contesto `Transcript`:

| Azione                     | Predefinito       | Descrizione                                               |
| :------------------------- | :---------------- | :-------------------------------------------------------- |
| `transcript:toggleShowAll` | Ctrl+E            | Attiva/disattiva la visualizzazione di tutto il contenuto |
| `transcript:exit`          | q, Ctrl+C, Escape | Esci dalla visualizzazione della trascrizione             |

<h3 id="history-search-actions">
  Azioni di ricerca nella cronologia
</h3>

Azioni disponibili nel contesto `HistorySearch`:

| Azione                     | Predefinito | Descrizione                                 |
| :------------------------- | :---------- | :------------------------------------------ |
| `historySearch:next`       | Ctrl+R      | Corrispondenza successiva                   |
| `historySearch:accept`     | Escape, Tab | Accetta la selezione                        |
| `historySearch:cancel`     | Ctrl+C      | Annulla la ricerca                          |
| `historySearch:execute`    | Invio       | Esegui il comando selezionato               |
| `historySearch:cycleScope` | Ctrl+S      | Cicla l'ambito: sessione, progetto, ovunque |

<h3 id="task-actions">
  Azioni delle attività
</h3>

Azioni disponibili nel contesto `Task`:

| Azione            | Predefinito           | Descrizione                                                                                                                    |
| :---------------- | :-------------------- | :----------------------------------------------------------------------------------------------------------------------------- |
| `task:background` | Ctrl+B, Ctrl+X Ctrl+B | Attività in background corrente. L'accordo Ctrl+X Ctrl+B richiede v2.1.169 o successiva e evita il conflitto del prefisso tmux |

<h3 id="theme-actions">
  Azioni del tema
</h3>

Azioni disponibili nel contesto `ThemePicker`:

| Azione                           | Predefinito | Descrizione                                      |
| :------------------------------- | :---------- | :----------------------------------------------- |
| `theme:toggleSyntaxHighlighting` | Ctrl+T      | Attiva/disattiva l'evidenziazione della sintassi |

<h3 id="help-actions">
  Azioni della guida
</h3>

Azioni disponibili nel contesto `Help`:

| Azione         | Predefinito | Descrizione                |
| :------------- | :---------- | :------------------------- |
| `help:dismiss` | Escape      | Chiudi il menu della guida |

<h3 id="tabs-actions">
  Azioni delle schede
</h3>

Azioni disponibili nel contesto `Tabs`:

| Azione          | Predefinito         | Descrizione       |
| :-------------- | :------------------ | :---------------- |
| `tabs:next`     | Tab, Destra         | Scheda successiva |
| `tabs:previous` | Shift+Tab, Sinistra | Scheda precedente |

<h3 id="attachments-actions">
  Azioni degli allegati
</h3>

Azioni disponibili nel contesto `Attachments`:

| Azione                 | Predefinito     | Descrizione                           |
| :--------------------- | :-------------- | :------------------------------------ |
| `attachments:next`     | Destra          | Allegato successivo                   |
| `attachments:previous` | Sinistra        | Allegato precedente                   |
| `attachments:remove`   | Backspace, Canc | Rimuovi l'allegato selezionato        |
| `attachments:exit`     | Giù, Escape     | Esci dalla navigazione degli allegati |

<h3 id="footer-actions">
  Azioni del piè di pagina
</h3>

Azioni disponibili nel contesto `Footer`:

| Azione                  | Predefinito | Descrizione                                                 |
| :---------------------- | :---------- | :---------------------------------------------------------- |
| `footer:next`           | Destra      | Elemento del piè di pagina successivo                       |
| `footer:previous`       | Sinistra    | Elemento del piè di pagina precedente                       |
| `footer:up`             | Su          | Naviga verso l'alto nel piè di pagina (deseleziona in alto) |
| `footer:down`           | Giù         | Naviga verso il basso nel piè di pagina                     |
| `footer:openSelected`   | Invio       | Apri l'elemento del piè di pagina selezionato               |
| `footer:clearSelection` | Escape      | Cancella la selezione del piè di pagina                     |

<h3 id="message-selector-actions">
  Azioni del selezionatore di messaggi
</h3>

Azioni disponibili nel contesto `MessageSelector`:

| Azione                   | Predefinito                            | Descrizione                       |
| :----------------------- | :------------------------------------- | :-------------------------------- |
| `messageSelector:up`     | Su, K, Ctrl+P                          | Sposta verso l'alto nell'elenco   |
| `messageSelector:down`   | Giù, J, Ctrl+N                         | Sposta verso il basso nell'elenco |
| `messageSelector:top`    | Ctrl+Su, Shift+Su, Meta+Su, Shift+K    | Salta all'inizio                  |
| `messageSelector:bottom` | Ctrl+Giù, Shift+Giù, Meta+Giù, Shift+J | Salta alla fine                   |
| `messageSelector:select` | Invio                                  | Seleziona il messaggio            |

<h3 id="diff-actions">
  Azioni diff
</h3>

Azioni disponibili nel contesto `DiffDialog`:

| Azione                | Predefinito     | Descrizione                                                                                                                                                                                                    |
| :-------------------- | :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `diff:dismiss`        | Escape          | Chiudi il visualizzatore diff; dalla visualizzazione dei dettagli, torna all'elenco dei file                                                                                                                   |
| `diff:previousSource` | Sinistra        | Sorgente diff precedente                                                                                                                                                                                       |
| `diff:nextSource`     | Destra          | Sorgente diff successiva                                                                                                                                                                                       |
| `diff:previousFile`   | Su, K           | File precedente nell'elenco dei file; scorri verso l'alto di una riga nella visualizzazione dei dettagli                                                                                                       |
| `diff:nextFile`       | Giù, J          | File successivo nell'elenco dei file; scorri verso il basso di una riga nella visualizzazione dei dettagli                                                                                                     |
| `diff:viewDetails`    | Invio           | Visualizza i dettagli del diff                                                                                                                                                                                 |
| `diff:back`           | (non associato) | Torna indietro nel visualizzatore diff. Escape esegue l'azione indietro tramite `diff:dismiss`. Il valore predefinito precedente di Sinistra nella visualizzazione dei dettagli è stato rimosso nella v2.1.203 |

La visualizzazione dei dettagli diff associa anche i tasti in stile pager alle [azioni di scorrimento](#scroll-actions) standard. Questi binding fanno parte del contesto `DiffDialog` e si applicano solo nella visualizzazione dei dettagli; i valori predefiniti del contesto `Scroll` elencati in [Azioni di scorrimento](#scroll-actions) rimangono invariati.

| Azione                | Predefinito     | Descrizione                                   |
| :-------------------- | :-------------- | :-------------------------------------------- |
| `scroll:pageUp`       | PagSu           | Scorri verso l'alto della metà del viewport   |
| `scroll:pageDown`     | PagGiù          | Scorri verso il basso della metà del viewport |
| `scroll:fullPageUp`   | Shift+Spazio, B | Scorri verso l'alto di un viewport completo   |
| `scroll:fullPageDown` | Spazio          | Scorri verso il basso di un viewport completo |
| `scroll:top`          | G, Home         | Salta all'inizio                              |
| `scroll:bottom`       | Shift+G, Fine   | Salta alla fine                               |

<h3 id="model-picker-actions">
  Azioni del selezionatore di modelli
</h3>

Azioni disponibili nel contesto `ModelPicker`:

| Azione                        | Predefinito | Descrizione                                           |
| :---------------------------- | :---------- | :---------------------------------------------------- |
| `modelPicker:decreaseEffort`  | Sinistra    | Diminuisci il livello di sforzo                       |
| `modelPicker:increaseEffort`  | Destra      | Aumenta il livello di sforzo                          |
| `modelPicker:thisSessionOnly` | s           | Applica il modello evidenziato solo a questa sessione |

<h3 id="select-actions">
  Azioni di selezione
</h3>

Azioni disponibili nel contesto `Select`:

| Azione            | Predefinito    | Descrizione          |
| :---------------- | :------------- | :------------------- |
| `select:next`     | Giù, J, Ctrl+N | Opzione successiva   |
| `select:previous` | Su, K, Ctrl+P  | Opzione precedente   |
| `select:accept`   | Invio          | Accetta la selezione |
| `select:cancel`   | Escape         | Annulla la selezione |

<h3 id="plugin-actions">
  Azioni dei plugin
</h3>

Azioni disponibili nel contesto `Plugin`:

| Azione            | Predefinito | Descrizione                                                                                                           |
| :---------------- | :---------- | :-------------------------------------------------------------------------------------------------------------------- |
| `plugin:toggle`   | Spazio      | Attiva/disattiva la selezione del plugin                                                                              |
| `plugin:install`  | I           | Installa i plugin selezionati                                                                                         |
| `plugin:favorite` | F           | Aggiungi ai preferiti il plugin selezionato in modo che si ordini vicino alla parte superiore della scheda Installati |

<h3 id="settings-actions">
  Azioni delle impostazioni
</h3>

Azioni disponibili nel contesto `Settings`. Le azioni `select:accept` e `confirm:no` vengono riutilizzate dai contesti [Select](#select-actions) e [Confirmation](#confirmation-actions) con comportamento specifico delle impostazioni: le modifiche si applicano a ogni impostazione non appena la modifichi, quindi Escape chiude il pannello con le modifiche salvate piuttosto che rifiutarle.

| Azione            | Predefinito   | Descrizione                                                 |
| :---------------- | :------------ | :---------------------------------------------------------- |
| `settings:search` | /             | Entra in modalità di ricerca                                |
| `settings:retry`  | R             | Riprova a caricare i dati di utilizzo in caso di errore     |
| `select:accept`   | Invio, Spazio | Modifica l'impostazione selezionata o apri il suo sottomenu |
| `confirm:no`      | Escape        | Chiudi il pannello. Le modifiche sono già salvate           |

<h3 id="voice-actions">
  Azioni vocali
</h3>

Azioni disponibili nel contesto `Chat` quando la [dettatura vocale](/docs/it/voice-dictation) è abilitata:

| Azione             | Predefinito | Descrizione                                             |
| :----------------- | :---------- | :------------------------------------------------------ |
| `voice:pushToTalk` | Spazio      | Tieni premuto o tocca a seconda della modalità `/voice` |

<h3 id="scroll-actions">
  Azioni di scorrimento
</h3>

Azioni disponibili nel contesto `Scroll` quando il [rendering a schermo intero](/docs/it/fullscreen) è abilitato:

| Azione                      | Predefinito          | Descrizione                                                                                                                             |
| :-------------------------- | :------------------- | :-------------------------------------------------------------------------------------------------------------------------------------- |
| `scroll:lineUp`             | (non associato)      | Scorri verso l'alto di una riga. Lo scorrimento con la rotella del mouse attiva questa azione                                           |
| `scroll:lineDown`           | (non associato)      | Scorri verso il basso di una riga. Lo scorrimento con la rotella del mouse attiva questa azione                                         |
| `scroll:pageUp`             | PagSu                | Scorri verso l'alto della metà dell'altezza del viewport                                                                                |
| `scroll:pageDown`           | PagGiù               | Scorri verso il basso della metà dell'altezza del viewport                                                                              |
| `scroll:top`                | Ctrl+Home            | Salta all'inizio della conversazione                                                                                                    |
| `scroll:bottom`             | Ctrl+Fine            | Salta al messaggio più recente e riabilita il follow automatico                                                                         |
| `scroll:halfPageUp`         | (non associato)      | Scorri verso l'alto della metà dell'altezza del viewport. Stesso comportamento di `scroll:pageUp`, fornito per i rebind in stile vi     |
| `scroll:halfPageDown`       | (non associato)      | Scorri verso il basso della metà dell'altezza del viewport. Stesso comportamento di `scroll:pageDown`, fornito per i rebind in stile vi |
| `scroll:fullPageUp`         | (non associato)      | Scorri verso l'alto dell'intera altezza del viewport                                                                                    |
| `scroll:fullPageDown`       | (non associato)      | Scorri verso il basso dell'intera altezza del viewport                                                                                  |
| `selection:copy`            | Ctrl+Shift+C / Cmd+C | Copia il testo selezionato negli appunti                                                                                                |
| `selection:clear`           | (non associato)      | Cancella la selezione di testo attiva                                                                                                   |
| `selection:extendLeft`      | Shift+Sinistra       | Estendi la selezione attiva di una colonna a sinistra                                                                                   |
| `selection:extendRight`     | Shift+Destra         | Estendi la selezione attiva di una colonna a destra                                                                                     |
| `selection:extendUp`        | Shift+Su             | Estendi la selezione attiva di una riga verso l'alto. Scorri il viewport quando la selezione raggiunge il bordo superiore               |
| `selection:extendDown`      | Shift+Giù            | Estendi la selezione attiva di una riga verso il basso. Scorri il viewport quando la selezione raggiunge il bordo inferiore             |
| `selection:extendLineStart` | Shift+Home           | Estendi la selezione attiva all'inizio della riga                                                                                       |
| `selection:extendLineEnd`   | Shift+Fine           | Estendi la selezione attiva alla fine della riga                                                                                        |

<h2 id="keystroke-syntax">
  Sintassi delle sequenze di tasti
</h2>

<h3 id="modifiers">
  Modificatori
</h3>

Utilizzare i tasti modificatori con il separatore `+`:

* `ctrl` o `control` - Tasto Control
* `shift` - Tasto Shift
* `alt`, `opt`, `option`, o `meta` - Tasto Alt su Windows e Linux, tasto Option su macOS
* `cmd`, `command`, `super`, o `win` - Tasto Command su macOS, tasto Windows su Windows, tasto Super su Linux

Il gruppo `cmd` viene rilevato solo nei terminali che segnalano il modificatore Super, come quelli che supportano il protocollo della tastiera Kitty o la modalità `modifyOtherKeys` di xterm. La maggior parte dei terminali non lo invia, quindi utilizzare `ctrl` o `meta` per i binding che si desidera funzionino ovunque.

Ad esempio:

```text theme={null}
ctrl+k          Ctrl + K
shift+tab       Shift + Tab
meta+p          Option + P su macOS, Alt + P altrove
ctrl+shift+c    Più modificatori
```

<h3 id="uppercase-letters">
  Lettere maiuscole
</h3>

Una lettera maiuscola autonoma implica Shift. Ad esempio, `K` è equivalente a `shift+k`. Questo è utile per i binding in stile vim dove i tasti maiuscoli e minuscoli hanno significati diversi.

Le lettere maiuscole con modificatori (ad es. `ctrl+K`) sono trattate come stilistiche e **non** implicano Shift: `ctrl+K` è lo stesso di `ctrl+k`.

<h3 id="chords">
  Accordi
</h3>

Gli accordi sono sequenze di tasti separate da spazi:

```text theme={null}
ctrl+k ctrl+s   Premi Ctrl+K, rilascia, quindi Ctrl+S
```

<h3 id="special-keys">
  Tasti speciali
</h3>

* `escape` o `esc` - Tasto Escape
* `enter` o `return` - Tasto Invio
* `tab` - Tasto Tab
* `space` - Barra spaziatrice
* `up`, `down`, `left`, `right` - Tasti freccia
* `backspace`, `delete` - Tasti Canc

<h2 id="unbind-default-shortcuts">
  Annulla l'associazione delle scorciatoie predefinite
</h2>

Impostare un'azione su `null` per annullare l'associazione di una scorciatoia predefinita:

```json theme={null}
{
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "ctrl+s": null
      }
    }
  ]
}
```

Questo funziona anche per i binding degli accordi. Annullare l'associazione di ogni accordo che condivide un prefisso libera quel prefisso per l'uso come binding a tasto singolo. Un accordo in qualsiasi contesto attivo mantiene il suo prefisso riservato, quindi è necessario annullare l'associazione di ogni accordo nel contesto che lo definisce.

La famiglia predefinita `Ctrl+X` si estende su due contesti: `ctrl+x ctrl+k` e `ctrl+x ctrl+e` in `Chat`, e `ctrl+x ctrl+b` in `Task`. Per reclamare `ctrl+x` stesso come binding a tasto singolo, annullare l'associazione di tutti loro:

```json theme={null}
{
  "bindings": [
    {
      "context": "Task",
      "bindings": {
        "ctrl+x ctrl+b": null
      }
    },
    {
      "context": "Chat",
      "bindings": {
        "ctrl+x ctrl+k": null,
        "ctrl+x ctrl+e": null,
        "ctrl+x": "chat:newline"
      }
    }
  ]
}
```

Se annulli l'associazione di alcuni ma non di tutti gli accordi su un prefisso, premere il prefisso entra comunque in modalità di attesa degli accordi per i binding rimanenti.

<h2 id="reserved-shortcuts">
  Scorciatoie riservate
</h2>

Queste scorciatoie non possono essere riassociate:

| Scorciatoia | Motivo                                               |
| :---------- | :--------------------------------------------------- |
| Ctrl+C      | Interrupt/annullamento hardcoded                     |
| Ctrl+D      | Uscita hardcoded                                     |
| Ctrl+M      | Identico a Invio nei terminali (entrambi inviano CR) |
| Caps Lock   | Non consegnato alle applicazioni terminali           |

<h2 id="terminal-conflicts">
  Conflitti del terminale
</h2>

Alcune scorciatoie potrebbero entrare in conflitto con i multiplexer di terminale:

| Scorciatoia | Conflitto                                     |
| :---------- | :-------------------------------------------- |
| Ctrl+B      | Prefisso tmux (premere due volte per inviare) |
| Ctrl+A      | Prefisso GNU screen                           |
| Ctrl+Z      | Sospensione del processo Unix (SIGTSTP)       |

<h2 id="vim-mode-interaction">
  Interazione con la modalità Vim
</h2>

Quando la modalità vim è abilitata tramite `/config` → Editor mode, le scorciatoie da tastiera e la modalità vim operano indipendentemente:

* **Modalità Vim** gestisce l'input a livello di input di testo (movimento del cursore, modalità, movimenti)
* **Scorciatoie da tastiera** gestiscono le azioni a livello di componente (attiva/disattiva attività, invia, ecc.)
* Il tasto Escape in modalità vim passa da INSERT a NORMAL; non attiva `chat:cancel`
* La maggior parte delle scorciatoie Ctrl+tasto passano attraverso la modalità vim al sistema di scorciatoie da tastiera
* I tasti Vim non sono rimappabili tramite il file delle scorciatoie da tastiera. Per mappare una sequenza in modalità INSERT a due tasti come `jj` su Escape, utilizzare l'impostazione [`vimInsertModeRemaps`](/docs/it/interactive-mode#remap-insert-mode-key-sequences)
* In modalità vim NORMAL, `?` mostra il menu della guida (comportamento vim)
* In modalità vim NORMAL, `/` apre la ricerca nella cronologia, lo stesso di Ctrl+R in modalità standard

<h2 id="validation">
  Convalida
</h2>

Claude Code convalida i tuoi keybindings e mostra avvisi per:

* Errori di analisi (JSON non valido o struttura non valida)
* Nomi di contesto non validi
* Conflitti di scorciatoie riservate
* Conflitti di multiplexer di terminale
* Binding duplicati nello stesso contesto

Claude Code segnala avvisi quando il file viene caricato e scrive ognuno nel log di debug. Avvia Claude Code con [`--debug`](/docs/it/cli-reference#cli-flags) per visualizzare i dettagli.
