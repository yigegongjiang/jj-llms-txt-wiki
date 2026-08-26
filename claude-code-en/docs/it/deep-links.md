> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Avviare sessioni dai link

> Apri una sessione di terminale Claude Code da un URL. Incorpora link `claude-cli://` in runbook, avvisi e dashboard in modo che un clic apra Claude Code nel repository corretto con il prompt corretto.

Un deep link è un URL `claude-cli://` che apre Claude Code in una nuova finestra di terminale. L'URL può contenere una directory di lavoro e un prompt da pre-compilare.

Questo ti consente di condividere un punto di partenza con un solo clic per un'attività: chiunque abbia Claude Code installato e faccia clic sul link vede una sessione aperta con il prompt già digitato. Il prompt è compilato ma non viene inviato finché non premi Invio.

Poiché un deep link è un URL, puoi inserirlo ovunque sia possibile inserire un link:

* Un passaggio del runbook di incidente che apre il repository del servizio interessato con un prompt diagnostico
* Un avviso di monitoraggio o una dashboard che si collega a un prompt di indagine per una metrica specifica
* Una pagina README o wiki che apre il progetto con un prompt di onboarding
* Una notifica di errore CI che pre-compila il nome del job in errore

Questa pagina spiega come [costruire un link](#build-a-link), [incorporarne uno in un runbook o attivarlo dalla shell](#examples), e [gestire o disabilitare la registrazione del gestore](#registration-and-supported-platforms) su ogni piattaforma.

<h2 id="how-it-works">
  Come funziona
</h2>

Il prefisso `claude-cli://` è uno schema URL personalizzato che Claude Code registra con il tuo sistema operativo, in modo simile a come i link `mailto:` aprono il tuo client di posta elettronica. Il link può trovarsi su una pagina web, in una wiki, in un messaggio Slack o in qualsiasi app che renderizza i link. Quando fai clic su uno:

1. Il browser o l'app passa l'URL al tuo sistema operativo.
2. Il sistema operativo riconosce il prefisso `claude-cli://` e avvia Claude Code sulla tua macchina.
3. Una nuova finestra di terminale si apre con Claude Code in esecuzione nella directory specificata dal link, e il testo del prompt del link è già nella casella di input.
4. Leggi il prompt, modificalo se vuoi, e premi Invio per inviarlo.

Il link stesso può essere ospitato ovunque, ma la sessione si apre sempre localmente sul computer in cui hai fatto clic. Vedi [Registrazione e piattaforme supportate](#registration-and-supported-platforms) per sapere quale emulatore di terminale si apre su ogni sistema operativo.

<Note>
  La piattaforma che visualizza il link deve consentire schemi URL personalizzati. Il Markdown renderizzato da GitHub consente `http` e `https` ma rimuove schemi come `claude-cli://` nei README, problemi, richieste pull e wiki. Viene visualizzato solo il testo del link, senza link dietro di esso e l'URL nascosto. Vedi [Risoluzione dei problemi](#the-link-renders-as-plain-text-instead-of-being-clickable) per una soluzione alternativa.
</Note>

<h3 id="what-a-launched-session-shows">
  Cosa mostra una sessione avviata
</h3>

Un deep link non esegue mai nulla da solo. Il link sceglie solo una directory e riempie la casella del prompt. Se fai clic su un link da una pagina di cui non ti fidi, il prompt è comunque inerte: nulla raggiunge il modello finché non leggi ciò che è stato compilato e premi Invio.

Quando la sessione si apre, una riga di avviso sotto la casella di input legge `Prompt da un link esterno` e rimane visibile finché non invii o cancelli il prompt. Per i prompt superiori a 1.000 caratteri, l'avviso include il conteggio dei caratteri e ti dice di scorrere e rivedere il testo completo prima di premere Invio, poiché i prompt lunghi possono spingere le istruzioni fuori dallo schermo. Le regole di autorizzazione, `CLAUDE.md` e i prompt di fiducia per la directory selezionata si applicano allo stesso modo di qualsiasi altra sessione.

<h2 id="build-a-link">
  Costruire un link
</h2>

Ogni deep link inizia con `claude-cli://open`, che è l'unico percorso che il gestore accetta, seguito da parametri di query facoltativi. La forma minima apre Claude Code nella tua home directory con un prompt vuoto:

```text theme={null}
claude-cli://open
```

Aggiungi parametri per controllare dove inizia la sessione e cosa contiene la casella del prompt:

| Parametro | Descrizione                                                                                                                                                                                                                                                           |
| --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `q`       | Testo da pre-compilare nella casella del prompt. [Codifica URL](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent) il valore. Usa `%0A` per le interruzioni di riga nei prompt multi-riga. Massimo 5.000 caratteri. |
| `cwd`     | Percorso assoluto da utilizzare come directory di lavoro. I percorsi di rete e UNC vengono rifiutati, così come i percorsi che contengono caratteri di controllo invisibili o bidirezionali.                                                                          |
| `repo`    | Uno slug `owner/name` di GitHub. Claude Code lo risolve in un clone locale che ha visto prima e inizia da lì. Se non hai un clone corrispondente, la sessione si apre nella tua home directory.                                                                       |

`cwd` e `repo` sono [due modi per impostare la directory di lavoro](#choose-between-cwd-and-repo). Se passi entrambi, `cwd` ha la precedenza e `repo` viene ignorato, anche se il percorso `cwd` non esiste.

Il seguente link punta a un repository chiamato `acme/payments` con un prompt diagnostico a due righe. Sostituisci `acme/payments` con lo slug `owner/name` del tuo repository quando costruisci il tuo:

```text theme={null}
claude-cli://open?repo=acme/payments&q=Investigate%20the%20failed%20deploy%20of%20payments-api.%0ACheck%20recent%20commits%20to%20main%20and%20the%20last%20successful%20build.
```

Facendo clic su di esso si apre una nuova finestra di terminale, si avvia Claude Code nel tuo clone locale di `acme/payments` e si riempie la casella del prompt con il testo decodificato:

```text theme={null}
Investigate the failed deploy of payments-api.
Check recent commits to main and the last successful build.
```

Puoi modificare il prompt prima di premere Invio per inviarlo. Se non hai un clone locale del repository, la sessione si apre nella tua home directory. Vedi [Scegliere tra `cwd` e `repo`](#choose-between-cwd-and-repo) per come viene selezionato il percorso locale quando hai più clone o worktrees.

<h3 id="choose-between-cwd-and-repo">
  Scegliere tra `cwd` e `repo`
</h3>

Usa `cwd` quando tutti coloro che fanno clic sul link hanno il progetto nello stesso percorso assoluto, ad esempio un devcontainer standardizzato o un'immagine VM.

Usa `repo` quando il link è condiviso e ogni persona clona in una posizione diversa. Claude Code risolve lo slug in un percorso locale come segue:

* Ogni volta che esegui `claude` in un repository Git, il percorso del filesystem di quella directory viene registrato rispetto allo slug `owner/name` di GitHub del repository.
* Quando arriva un deep link, `repo` apre il percorso corrispondente che hai utilizzato più di recente. I clone multipli e i worktrees vengono tracciati separatamente, quindi sceglie quello in cui hai lavorato per ultimo.
* La ricerca trova solo i percorsi in cui hai già eseguito Claude Code almeno una volta.
* Il link non cambia quale branch è estratto. La sessione si apre nello stato in cui quella directory si trova attualmente.

L'intestazione di benvenuto mostra quale percorso ha scelto in modo che tu possa confermare che il clone corretto si è aperto.

<h2 id="examples">
  Esempi
</h2>

Le sezioni seguenti mostrano due modi comuni di utilizzare un deep link: come link Markdown in un documento e come comando in uno script di shell o alias.

<h3 id="embed-a-link-in-a-runbook">
  Incorporare un link in un runbook
</h3>

Un deep link in un runbook offre a chiunque stia triaging un modo con un solo clic per iniziare a indagare nel repository corretto con un prompt preparato. La piattaforma che renderizza il runbook deve consentire schemi URL personalizzati. Il Markdown renderizzato da GitHub non consente `claude-cli://`, quindi un deep link in un README, problema o wiki di GitHub mostra solo la sua etichetta senza link cliccabile. Vedi [la nota sulla risoluzione dei problemi](#the-link-renders-as-plain-text-instead-of-being-clickable) per una soluzione alternativa.

Il prompt fa parte dell'URL e deve essere codificato in URL. Per produrre il valore codificato, passa il testo del tuo prompt attraverso `encodeURIComponent` in una console del browser o in qualsiasi codificatore URL.

L'esempio seguente aggiunge un punto di ingresso di indagine a un runbook di incidente per un servizio chiamato `web-gateway`:

```markdown theme={null}
## High 5xx rate on web-gateway

1. Acknowledge the page in PagerDuty.
2. [Open Claude Code in the gateway repo](claude-cli://open?repo=acme/web-gateway&q=5xx%20rate%20is%20elevated%20on%20web-gateway.%20Check%20recent%20deploys%2C%20error%20logs%20from%20the%20last%2030%20minutes%2C%20and%20open%20incidents%20in%20Linear.)
3. Post initial findings in #incident.
```

Per utilizzarlo nel tuo runbook, sostituisci `acme/web-gateway` con lo slug del repository del tuo servizio. Questo consente agli ingegneri che hanno Claude Code installato e un clone locale di quel repository di fare clic sul passaggio 2 e iniziare a indagare con il prompt pronto per l'invio.

<h3 id="open-a-link-from-the-shell">
  Aprire un link dalla shell
</h3>

Puoi anche aprire un deep link da uno script di shell, alias o automazione piuttosto che facendo clic su di esso. Chiama il comando di apertura URL del tuo sistema operativo con il link come argomento.

<Tabs>
  <Tab title="macOS">
    Il comando `open` integrato passa l'URL al gestore `claude-cli://` registrato:

    ```bash theme={null}
    open "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>

  <Tab title="Linux">
    La maggior parte degli ambienti desktop fornisce `xdg-open`, che passa l'URL al gestore registrato:

    ```bash theme={null}
    xdg-open "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>

  <Tab title="Windows">
    In PowerShell, `Start-Process` passa l'URL al gestore registrato:

    ```powershell theme={null}
    Start-Process "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```

    In `cmd.exe`, `start` tratta il suo primo argomento tra virgolette come titolo della finestra, quindi passa un titolo vuoto prima dell'URL:

    ```cmd theme={null}
    start "" "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>
</Tabs>

<h2 id="registration-and-supported-platforms">
  Registrazione e piattaforme supportate
</h2>

Claude Code registra il gestore `claude-cli://` con il tuo sistema operativo la prima volta che avvii una sessione interattiva su macOS, Linux e Windows. Non esegui un comando di installazione separato. La registrazione scrive solo in posizioni a livello di utente:

| Piattaforma | Posizione del gestore                                                                                                             |
| ----------- | --------------------------------------------------------------------------------------------------------------------------------- |
| macOS       | `~/Applications/Claude Code URL Handler.app`                                                                                      |
| Linux       | `claude-code-url-handler.desktop` sotto `$XDG_DATA_HOME/applications`, per impostazione predefinita `~/.local/share/applications` |
| Windows     | `HKEY_CURRENT_USER\Software\Classes\claude-cli`                                                                                   |

Il gestore avvia Claude Code in un emulatore di terminale rilevato. Su macOS, Claude Code ricorda il terminale dalla tua sessione interattiva più recente e lo riutilizza, supportando iTerm2, Ghostty, kitty, Alacritty, WezTerm e Terminal.app. Su Linux onora la variabile di ambiente `$TERMINAL`, quindi `x-terminal-emulator`, quindi un elenco di emulatori comuni. Su Windows preferisce Windows Terminal, quindi PowerShell, quindi `cmd.exe`.

Per impedire completamente la registrazione, imposta [`disableDeepLinkRegistration`](/docs/it/settings) su `"disable"` in `settings.json`. Per applicare questo in tutta un'organizzazione in modo che gli utenti non possano riabilitarlo, impostalo invece in [managed settings](/docs/it/server-managed-settings).

<h2 id="open-a-vs-code-tab-instead-of-a-terminal">
  Aprire una scheda VS Code invece di un terminale
</h2>

L'estensione VS Code registra il suo gestore a `vscode://anthropic.claude-code/open`, che apre una scheda dell'editor Claude Code piuttosto che una finestra di terminale. Vedi [Avviare una scheda VS Code da altri strumenti](/docs/it/vs-code#launch-a-vs-code-tab-from-other-tools) per i parametri di quell'URL.

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

<h3 id="clicking-the-link-does-nothing">
  Fare clic sul link non fa nulla
</h3>

Il gestore probabilmente non è ancora registrato. Avvia una sessione `claude` interattiva una volta su quella macchina, esci e riprova il link. Se sei su Linux senza un ambiente desktop, `xdg-open` potrebbe non avere nulla a cui inviare.

<h3 id="the-link-renders-as-plain-text-instead-of-being-clickable">
  Il link viene renderizzato come testo semplice invece di essere cliccabile
</h3>

Alcuni renderer Markdown consentono solo link `http` e `https` e rimuovono altri schemi URL. GitHub lo fa nei README, problemi, richieste pull e wiki: `[label](claude-cli://...)` viene renderizzato come solo `label`, senza link e con l'URL rimosso. Su queste piattaforme, metti il deep link in un blocco di codice in modo che i lettori possano vedere l'URL e incollarlo nella barra degli indirizzi del loro browser.

<h3 id="the-session-opens-in-my-home-directory-instead-of-the-repo">
  La sessione si apre nella mia home directory invece che nel repository
</h3>

Il parametro `repo` risolve solo i clone che Claude Code ha già visto. Esegui `claude` all'interno del clone una volta in modo che il suo percorso sia registrato, o cambia il link per utilizzare `cwd` con un percorso assoluto.

<h3 id="the-link-opens-the-wrong-terminal">
  Il link apre il terminale sbagliato
</h3>

Su macOS, avvia `claude` nel tuo terminale preferito una volta e il prossimo deep link lo userà. Su Linux, imposta la variabile di ambiente `$TERMINAL` sul nome del comando dell'emulatore preferito. Su Windows, l'ordine è fisso: installa Windows Terminal se vuoi che i link si aprano lì invece di una finestra PowerShell o `cmd.exe`.

<h2 id="learn-more">
  Ulteriori informazioni
</h2>

Queste pagine coprono modi correlati per avviare o estendere le sessioni di Claude Code:

* [Skills](/docs/it/skills): archivia un lungo prompt di runbook come `/skill` nel repository in modo che il parametro `q` del deep link debba solo nominarlo
* [Non-interactive mode](/docs/it/headless): esegui Claude da uno script e cattura l'output senza aprire un terminale
