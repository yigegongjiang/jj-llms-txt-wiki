> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Desktop su Linux (beta)

> Installa e aggiorna l'app desktop di Claude su Ubuntu e Debian

<Note>
  Il supporto di Linux per l'app desktop di Claude è in beta. Le schede Chat, Cowork e Code sono tutte disponibili.
</Note>

L'app desktop su Linux ti offre la stessa esperienza di Chat, Cowork e Claude Code di macOS e Windows: sessioni parallele, revisione visiva delle differenze, un terminale e un editor integrati e anteprima live dell'app. Consulta [Usa Claude Code Desktop](/docs/it/desktop) per il riferimento completo delle funzionalità.

<h2 id="requirements">
  Requisiti
</h2>

* Ubuntu 22.04 o versioni successive, oppure Debian 12 o versioni successive
* x86\_64 o arm64

Altre distribuzioni basate su Debian che soddisfano questi requisiti potrebbero funzionare ma non sono ufficialmente testate.

<h2 id="install">
  Installa
</h2>

Installa dal repository apt di Anthropic in modo che gli aggiornamenti arrivino attraverso gli aggiornamenti regolari dei pacchetti del tuo sistema. Apri un terminale ed esegui i comandi in ogni passaggio.

<Steps>
  <Step title="Aggiungi il repository apt di Anthropic">
    Questo passaggio scarica la chiave di firma con `curl`, che le installazioni fresche di Debian e Ubuntu potrebbero non includere. Se il comando di download non riesce con `sudo: curl: command not found`, installa prima curl:

    ```bash theme={null}
    sudo apt install curl
    ```

    Scarica la chiave di firma di Anthropic:

    ```bash theme={null}
    sudo curl -fsSLo /usr/share/keyrings/claude-desktop-archive-keyring.asc https://downloads.claude.ai/claude-desktop/key.asc
    ```

    Registra il repository:

    ```bash theme={null}
    echo "deb [arch=amd64,arm64 signed-by=/usr/share/keyrings/claude-desktop-archive-keyring.asc] https://downloads.claude.ai/claude-desktop/apt/stable stable main" | sudo tee /etc/apt/sources.list.d/claude-desktop.list
    ```
  </Step>

  <Step title="Installa il pacchetto">
    ```bash theme={null}
    sudo apt update && sudo apt install claude-desktop
    ```
  </Step>

  <Step title="Avvia e accedi">
    Avvia **Claude** dal tuo launcher di applicazioni, oppure esegui `claude-desktop` da un terminale, e accedi con il tuo account Anthropic.

    L'app Linux accede allo stesso modo di macOS e Windows: con un abbonamento a claude.ai, oppure tramite l'SSO della tua organizzazione. Desktop non accetta direttamente una chiave API di Claude Console; utilizza la [CLI](/docs/it/quickstart) per l'autenticazione tramite chiave API. Per le distribuzioni aziendali che instradano Desktop a Google Cloud's Agent Platform o a un gateway LLM, consulta [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview) e la [configurazione di rete](/docs/it/network-config).
  </Step>
</Steps>

<Accordion title="Verifica la chiave di firma">
  Puoi confermare che la chiave di firma scaricata appartiene ad Anthropic:

  ```bash theme={null}
  gpg --show-keys /usr/share/keyrings/claude-desktop-archive-keyring.asc
  ```

  L'impronta digitale dovrebbe essere `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE`.
</Accordion>

<h3 id="install-from-a-downloaded-file">
  Installa da un file scaricato
</h3>

Se non riesci a installare tramite il repository apt, scarica il pacchetto `.deb` direttamente dal pool di pacchetti del repository. Questo comando cerca il pacchetto più recente per la tua architettura nell'indice del repository, quindi lo scarica nella directory corrente:

```bash theme={null}
curl -fLO "https://downloads.claude.ai/claude-desktop/apt/stable/$(curl -s "https://downloads.claude.ai/claude-desktop/apt/stable/dists/stable/main/binary-$(dpkg --print-architecture)/Packages" | grep '^Filename: pool/main/c/claude-desktop/claude-desktop_' | sort -V | tail -n 1 | cut -d' ' -f2)"
```

Se il comando non riesce con `Remote file name has no length`, la ricerca non ha restituito alcun percorso di pacchetto. Questo può significare che l'indice del repository non potrebbe essere recuperato, ad esempio quando la tua rete blocca `downloads.claude.ai`, oppure che non esiste alcun pacchetto per la tua architettura. Conferma che la tua rete può raggiungere `downloads.claude.ai` e che `dpkg --print-architecture` stampa `amd64` o `arm64`; il repository non pubblica pacchetti per altre architetture.

Quindi apri il file scaricato con il tuo programma di installazione del software, come GNOME Software, oppure installalo con apt dalla directory che contiene il file scaricato:

```bash theme={null}
sudo apt install ./claude-desktop_*.deb
```

Se apt segnala `E: Unsupported file ./claude-desktop_*.deb given on commandline`, il pattern non ha corrisposto a un file `.deb` nella directory corrente. Conferma che il download sia completato, quindi esegui di nuovo il comando dalla directory che contiene il file.

Un `.deb` installato in questo modo non riceve aggiornamenti. Per ricevere gli aggiornamenti tramite apt, registra il repository dal passaggio [Aggiungi il repository apt di Anthropic](#install). Il pacchetto scrive anche una voce di repository commentata in `/etc/apt/sources.list.d/claude-desktop.list`; rimuovere il commento dalla sua riga `deb` è equivalente.

<h2 id="update">
  Aggiorna
</h2>

L'app desktop non si aggiorna da sola su Linux. Gli aggiornamenti arrivano con gli aggiornamenti regolari dei pacchetti del tuo sistema:

```bash theme={null}
sudo apt update && sudo apt upgrade
```

Lo strumento di aggiornamento software grafico della tua distribuzione raccoglierà anche le nuove versioni.

<h2 id="uninstall">
  Disinstalla
</h2>

```bash theme={null}
sudo apt remove claude-desktop
```

Questo rimuove la chiave di firma insieme all'app, quindi se hai aggiunto la voce del repository durante l'installazione, rimuovila anche:

```bash theme={null}
sudo rm /etc/apt/sources.list.d/claude-desktop.list
```

<h2 id="troubleshoot">
  Risoluzione dei problemi
</h2>

<h3 id="unable-to-locate-package-claude-desktop">
  Impossibile individuare il pacchetto claude-desktop
</h3>

Se `sudo apt install claude-desktop` non riesce con `E: Unable to locate package claude-desktop`, apt non ha trovato il repository che avete aggiunto. Verificate quanto segue:

* Confermate che la voce del repository sia stata scritta. `cat /etc/apt/sources.list.d/claude-desktop.list` dovrebbe mostrare la riga `deb` dal passaggio [Aggiungere il repository apt di Anthropic](#install). Se il file è vuoto o mancante, eseguite di nuovo quel passaggio.
* Confermate che la vostra architettura sia supportata. `dpkg --print-architecture` dovrebbe stampare `amd64` o `arm64`. Il repository non pubblica pacchetti per altre architetture.
* Eseguite di nuovo `sudo apt update` e controllate il suo output per errori relativi a `downloads.claude.ai`. Un errore di rete o di chiave lì significa che il repository è stato aggiunto ma non poteva essere raggiunto o verificato.

Se il repository è in posizione e raggiungibile e il pacchetto non viene ancora trovato, [installate da un file scaricato](#install-from-a-downloaded-file) invece.

<h2 id="what’s-not-in-the-linux-beta-yet">
  Cosa non è ancora nella beta di Linux
</h2>

* **Computer Use**: il [controllo dell'app e dello schermo](/docs/it/desktop#let-claude-use-your-computer) non è disponibile su Linux.
* **Dictation**: l'input vocale non è disponibile nell'app desktop di Linux. Utilizza invece la [dettatura vocale](/docs/it/voice-dictation) nella CLI.
* **Quick Entry global hotkey**: funziona su X11. Su Wayland nativo richiede il portale GlobalShortcuts del tuo ambiente desktop.
* **Fedora e RHEL**: sono supportate solo le distribuzioni basate su Debian oggi. Il supporto per distribuzioni aggiuntive arriverà in futuro.

Per qualsiasi cosa non ancora disponibile nell'app desktop, la [CLI](/docs/it/quickstart) esegue lo stesso motore Claude Code e supporta una gamma più ampia di distribuzioni Linux; consulta i [requisiti di sistema](/docs/it/setup#system-requirements).
