> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Desktop su Linux (beta)

> Installa e aggiorna l'app desktop di Claude su Ubuntu e Debian

<Note>
  Il supporto di Linux per l'app desktop di Claude è in beta.
</Note>

L'app desktop su Linux ti offre la stessa esperienza di Chat, Cowork e Claude Code di macOS e Windows: sessioni parallele, revisione visiva delle differenze, un terminale e un editor integrati e anteprima live dell'app. Consulta [Usa Claude Code Desktop](/docs/it/desktop) per il riferimento delle funzionalità.

<h2 id="requirements">
  Requisiti
</h2>

* Una distribuzione basata su Debian: Ubuntu 22.04 o versioni successive, oppure Debian 12 o versioni successive
* x86\_64 o arm64

Altre distribuzioni basate su Debian che soddisfano questi requisiti potrebbero funzionare ma non sono ufficialmente testate. Su distribuzioni che non sono basate su Debian, come Fedora o Arch, eseguire la [CLI](/docs/it/setup#system-requirements) invece. Se lavorate su Windows con WSL 2, installate l'app desktop di Windows ed eseguite le sessioni all'interno della vostra distribuzione; vedere [Claude Code Desktop in WSL](/docs/it/desktop-wsl).

<h3 id="cowork-requirements">
  Requisiti di Cowork
</h3>

Cowork è la scheda desktop per [Dispatch e lavoro agentico più lungo](https://claude.com/docs/cowork/overview). Su Linux, Cowork esegue queste attività in una macchina virtuale che l'app desktop ospita con QEMU e KVM. Per utilizzare Cowork, la vostra macchina ha bisogno di:

* **Virtualizzazione hardware**: attivata nelle impostazioni del firmware. Senza di essa, la scheda Cowork segnala "Cowork requires hardware virtualization (KVM)".
* **QEMU e firmware UEFI**: `qemu-system-x86`, `ovmf` e `virtiofsd` su x86\_64, oppure `qemu-system-arm`, `qemu-efi-aarch64` e `virtiofsd` su arm64. `apt install claude-desktop` li installa per impostazione predefinita come pacchetti consigliati. Se avete installato con `--no-install-recommends`, o il vostro sistema è un'immagine minima che salta i pacchetti consigliati, la scheda Cowork segnala "Cowork requires QEMU" e mostra il comando `apt install` da eseguire. Ubuntu 22.04 non ha alcun pacchetto `virtiofsd`; l'app utilizza una copia in bundle lì.
* **Accesso a `/dev/kvm`**: aggiungete il vostro utente al gruppo `kvm` con `sudo usermod -aG kvm $USER`, quindi disconnettetevi e riconnettetevi. Alcuni ambienti desktop concedono all'utente connesso l'accesso a `/dev/kvm` senza il gruppo, ma Cowork ha anche bisogno di `/dev/vhost-vsock`, che solo i membri del gruppo `kvm` possono aprire. Unitevi al gruppo anche se `/dev/kvm` funziona già per voi.

L'app controlla questi requisiti una volta all'avvio: riavviatela dopo aver installato i pacchetti, e disconnettetevi e riconnettetevi dopo aver aderito al gruppo. Se `/dev/vhost-vsock` manca e il vostro kernel in esecuzione non ha alcuna directory di moduli sotto `/lib/modules`, la scheda Cowork segnala che il kernel non include il supporto di virtualizzazione di cui Cowork ha bisogno e che non può essere aggiunto manualmente. Questa combinazione è comune su ChromeOS e negli ambienti Linux basati su container.

<h2 id="install">
  Installa
</h2>

Installa dal repository apt di Anthropic in modo che gli aggiornamenti arrivino attraverso gli aggiornamenti regolari dei pacchetti del tuo sistema. Apri un terminale ed esegui i comandi in ogni passaggio.

<Steps>
  <Step title="Aggiungi il repository apt di Anthropic">
    Questo passaggio scarica la chiave di firma con `curl` e la verifica con `gpg`, che le installazioni fresche di Debian e Ubuntu potrebbero non includere. Se uno dei due comandi segnala `command not found`, installa prima entrambi:

    ```bash theme={null}
    sudo apt install curl gnupg
    ```

    Scarica la chiave di firma di Anthropic:

    ```bash theme={null}
    sudo curl -fsSLo /usr/share/keyrings/claude-desktop-archive-keyring.asc https://downloads.claude.ai/claude-desktop/key.asc
    ```

    Il comando non stampa nulla quando ha successo e un errore `curl:` quando non riesce. Una chiave mancante o errata fa fallire `apt update` in seguito con `NO_PUBKEY BAA929FF1A7ECACE`, quindi conferma che la chiave sia stata scaricata e appartenga ad Anthropic prima di continuare:

    ```bash theme={null}
    gpg --show-keys /usr/share/keyrings/claude-desktop-archive-keyring.asc
    ```

    L'impronta digitale che gpg stampa dovrebbe essere `31DDDE24DDFAB679F42D7BD2BAA929FF1A7ECACE`. Se gpg segnala che il file non può essere aperto o non contiene dati OpenPGP validi, il download non è riuscito o ha restituito il contenuto sbagliato: conferma che la tua rete possa raggiungere `downloads.claude.ai`, quindi esegui di nuovo il comando di download.

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

<h3 id="install-from-a-downloaded-file">
  Installa da un file scaricato
</h3>

Se non riesci a installare tramite il repository apt, scarica il pacchetto `.deb` direttamente dal pool di pacchetti del repository. Questo comando cerca il pacchetto più recente per la tua architettura nell'indice del repository, quindi lo scarica nella directory corrente:

```bash theme={null}
curl -fLO "https://downloads.claude.ai/claude-desktop/apt/stable/$(curl -s "https://downloads.claude.ai/claude-desktop/apt/stable/dists/stable/main/binary-$(dpkg --print-architecture)/Packages" | grep '^Filename: pool/main/c/claude-desktop/claude-desktop_' | sort -V | tail -n 1 | cut -d' ' -f2)"
```

Se il comando non riesce con `Remote file name has no length`, la ricerca non ha restituito alcun percorso di pacchetto. Questo può significare che l'indice del repository non potrebbe essere recuperato, ad esempio quando la tua rete blocca `downloads.claude.ai`, oppure che non esiste alcun pacchetto per la tua architettura. Conferma che la tua rete può raggiungere `downloads.claude.ai` e che `dpkg --print-architecture` stampa `amd64` o `arm64`; il repository non pubblica pacchetti per altre architetture.

Per installare senza registrare il repository apt di Anthropic, crea prima `/etc/default/claude-desktop` con la riga `CLAUDE_DESKTOP_ADD_REPO="false"`. Senza il repository, apt non fornisce nuove versioni; per aggiornare, esegui di nuovo il comando di download e reinstalla, oppure [registra il repository](#install) in seguito.

Quindi apri il file scaricato con il tuo programma di installazione del software, come GNOME Software, oppure installalo con apt dalla directory che contiene il file scaricato:

```bash theme={null}
sudo apt install ./claude-desktop_*.deb
```

Se apt segnala `E: Unsupported file ./claude-desktop_*.deb given on commandline`, il pattern non ha corrisposto a un file `.deb` nella directory corrente. Conferma che il download sia completato, quindi esegui di nuovo il comando dalla directory che contiene il file.

L'installazione del `.deb` registra anche il repository apt di Anthropic in `/etc/apt/sources.list.d/claude-desktop.list`, quindi gli aggiornamenti futuri arrivano con gli [aggiornamenti regolari dei pacchetti](#update) del tuo sistema.

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

La disinstallazione del pacchetto rimuove anche la voce del repository e la chiave di firma che ha registrato. Se hai aggiunto la voce del repository tu stesso con il passaggio [Aggiungi il repository apt di Anthropic](#install), rimuovila anche:

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

* Eseguite `sudo apt update` dopo aver aggiunto il repository. `apt install` da solo non vede un repository che avete aggiunto dopo l'ultima volta che avete eseguito `apt update`.
* Confermate che la voce del repository sia stata scritta. `cat /etc/apt/sources.list.d/claude-desktop.list` dovrebbe mostrare la riga `deb` dal passaggio [Aggiungere il repository apt di Anthropic](#install). Se il file è vuoto o mancante, eseguite di nuovo quel passaggio.
* Confermate che la vostra architettura sia supportata. `dpkg --print-architecture` dovrebbe stampare `amd64` o `arm64`. Il repository non pubblica pacchetti per altre architetture.
* Eseguite di nuovo `sudo apt update` e controllate il suo output per errori relativi a `downloads.claude.ai`. Un errore di rete o di chiave lì significa che il repository è stato aggiunto ma non poteva essere raggiunto o verificato.

Se il repository è in posizione e raggiungibile e il pacchetto non viene ancora trovato, [installate da un file scaricato](#install-from-a-downloaded-file) invece.

<h3 id="unmet-dependencies">
  Dipendenze non soddisfatte
</h3>

Se `apt` si ferma con `The following packages have unmet dependencies` o `Unsatisfied dependencies`, leggete quale dipendenza nomina:

* `libc6 (>= 2.34)`: la vostra distribuzione è più vecchia di quella supportata dal pacchetto. Ubuntu 20.04 fornisce `libc6` 2.31. Aggiornate a Ubuntu 22.04 o successivo, oppure Debian 12 o successivo.
* Tutte le dipendenze mancanti mostrano `not installable` con un suffisso `:amd64` o `:arm64`: avete scaricato il `.deb` per un'architettura diversa da quella della vostra macchina. Eseguite `dpkg --print-architecture` e scaricate il `.deb` corrispondente, oppure [installate dal repository apt](#install), che seleziona il pacchetto per la vostra architettura.

<h3 id="running-as-root-without-no-sandbox-is-not-supported">
  L'esecuzione come root senza --no-sandbox non è supportata
</h3>

Se `claude-desktop` esce con questo messaggio, l'avete lanciato come root. Accedete come utente regolare e lanciatelo da lì.

<h3 id="cowork-isn’t-available">
  Cowork non è disponibile
</h3>

Se la scheda Cowork mostra uno di questi messaggi, correggete il requisito che nomina, quindi riavviate l'app:

* **Cowork richiede QEMU**: installate i [pacchetti QEMU e firmware UEFI](#cowork-requirements) che il messaggio elenca.
* **Cowork richiede virtualizzazione hardware (KVM)**: attivate la [virtualizzazione hardware](#cowork-requirements) nelle impostazioni del firmware.
* **Claude non ha il permesso di utilizzare la virtualizzazione (/dev/kvm)**: aggiungete il vostro utente al [gruppo `kvm`](#cowork-requirements), quindi disconnettetevi e riconnettetevi.
* **Cowork richiede il modulo kernel `vhost_vsock`**: eseguite `sudo modprobe vhost_vsock`, quindi riavviate l'app. Questo carica il modulo solo per l'avvio corrente. Per caricarlo ad ogni avvio, eseguite `echo vhost_vsock | sudo tee /etc/modules-load.d/vhost_vsock.conf`.

<h2 id="what’s-not-in-the-linux-beta-yet">
  Cosa non è ancora nella beta di Linux
</h2>

* **Computer Use**: il [controllo dell'app e dello schermo](/docs/it/desktop#let-claude-use-your-computer) non è disponibile su Linux.
* **Dictation**: l'input vocale non è disponibile nell'app desktop di Linux. Utilizza invece la [dettatura vocale](/docs/it/voice-dictation) nella CLI.
* **Quick Entry global hotkey**: funziona su X11. Su Wayland nativo richiede il portale GlobalShortcuts del tuo ambiente desktop.
* **Fedora e RHEL**: sono supportate solo le distribuzioni basate su Debian oggi. Il supporto per distribuzioni aggiuntive arriverà in futuro.

Per qualsiasi cosa non ancora disponibile nell'app desktop, la [CLI](/docs/it/quickstart) esegue lo stesso motore Claude Code e supporta una gamma più ampia di distribuzioni Linux; consulta i [requisiti di sistema](/docs/it/setup#system-requirements).
