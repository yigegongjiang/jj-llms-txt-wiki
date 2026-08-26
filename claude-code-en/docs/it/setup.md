> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurazione avanzata

> Requisiti di sistema, installazione specifica per piattaforma, gestione delle versioni e disinstallazione per Claude Code.

Questa pagina copre i requisiti di sistema, i dettagli di installazione specifici per piattaforma, gli aggiornamenti e la disinstallazione. Per una procedura guidata della vostra prima sessione, consultate la [guida rapida](/docs/it/quickstart). Se non avete mai utilizzato un terminale prima, consultate la [guida del terminale](/docs/it/terminal-guide).

<h2 id="system-requirements">
  Requisiti di sistema
</h2>

Claude Code funziona sulle seguenti piattaforme e configurazioni:

* **Sistema operativo**:
  * macOS 13.0+
  * Windows 10 1809+ o Windows Server 2019+
  * Ubuntu 20.04+
  * Debian 10+
  * Alpine Linux 3.19+
* **Hardware**: 4 GB+ di RAM, processore x64 o ARM64
* **Rete**: connessione a Internet richiesta. Consultate la [configurazione di rete](/docs/it/network-config#network-access-requirements).
* **Shell**: Bash, Zsh, PowerShell o CMD.
* **Posizione**: [paesi supportati da Anthropic](https://www.anthropic.com/supported-countries)

<h3 id="additional-dependencies">
  Dipendenze aggiuntive
</h3>

* **ripgrep**: solitamente incluso con Claude Code. Se la ricerca non funziona, consultate la [risoluzione dei problemi di ricerca](/docs/it/troubleshooting#search-and-discovery-issues).

<h2 id="install-claude-code">
  Installare Claude Code
</h2>

<Tip>
  Preferite un'interfaccia grafica? L'[app Desktop](/docs/it/desktop-quickstart) vi consente di utilizzare Claude Code senza il terminale. Scaricatela per [macOS](https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code\&utm_medium=docs), [Windows](https://claude.com/download?utm_source=claude_code\&utm_medium=docs) o [Linux](/docs/it/desktop-linux).

  Siete nuovi al terminale? Consultate la [guida del terminale](/docs/it/terminal-guide) per istruzioni passo dopo passo.
</Tip>

To install Claude Code, use one of the following methods:

<Tabs>
  <Tab title="Native Install (Recommended)">
    **macOS, Linux, WSL:**

    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash
    ```

    **Windows PowerShell:**

    ```powershell theme={null}
    irm https://claude.ai/install.ps1 | iex
    ```

    **Windows CMD:**

    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
    ```

    If you see `The token '&&' is not a valid statement separator`, you're in PowerShell, not CMD. If you see `'irm' is not recognized as an internal or external command`, you're in CMD, not PowerShell. Your prompt shows `PS C:\` when you're in PowerShell and `C:\` without the `PS` when you're in CMD.

    If the install command fails with `syntax error near unexpected token '<'`, a `403`, or another curl error, see [Troubleshoot installation](/docs/en/troubleshoot-install#find-your-error) to match the error to a fix and for alternative install methods.

    [Git for Windows](https://git-scm.com/downloads/win) is recommended on native Windows so Claude Code can use the Bash tool. If Git for Windows is not installed, Claude Code uses PowerShell as the shell tool instead. WSL setups do not need Git for Windows.

    <Info>
      Native installations automatically update in the background to keep you on the latest version.
    </Info>
  </Tab>

  <Tab title="Homebrew">
    ```bash theme={null}
    brew install --cask claude-code
    ```

    Homebrew offers two casks. `claude-code` tracks the stable release channel, which is typically about a week behind and skips releases with major regressions. `claude-code@latest` tracks the latest channel and receives new versions as soon as they ship.

    <Info>
      Homebrew installations do not auto-update. Run `brew upgrade claude-code` or `brew upgrade claude-code@latest`, depending on which cask you installed, to get the latest features and security fixes.
    </Info>
  </Tab>

  <Tab title="WinGet">
    ```powershell theme={null}
    winget install Anthropic.ClaudeCode
    ```

    <Info>
      WinGet installations do not auto-update. Run `winget upgrade Anthropic.ClaudeCode` periodically to get the latest features and security fixes.
    </Info>
  </Tab>
</Tabs>

You can also install with [apt, dnf, or apk](/docs/en/setup#install-with-linux-package-managers) on Debian, Fedora, RHEL, and Alpine.

Dopo il completamento dell'installazione, aprite un terminale nel progetto su cui desiderate lavorare e avviate Claude Code:

```bash theme={null}
claude
```

Se riscontrate problemi durante l'installazione, consultate [Risoluzione dei problemi di installazione e accesso](/docs/it/troubleshoot-install).

<h3 id="set-up-on-windows">
  Configurazione su Windows
</h3>

Potete eseguire Claude Code nativamente su Windows o all'interno di WSL. Scegliete in base a dove si trovano i vostri progetti e quali funzionalità vi servono:

| Opzione        | Richiede                                                                    | [Sandboxing](/docs/it/sandboxing) | Quando utilizzare                                  |
| -------------- | --------------------------------------------------------------------------- | ---------------------------- | -------------------------------------------------- |
| Windows nativo | Nessuno; [Git for Windows](https://git-scm.com/downloads/win) è facoltativo | Non supportato               | Progetti e strumenti nativi di Windows             |
| WSL 2          | WSL 2 abilitato                                                             | Supportato                   | Toolchain Linux o esecuzione di comandi in sandbox |
| WSL 1          | WSL 1 abilitato                                                             | Non supportato               | Se WSL 2 non è disponibile                         |

**Opzione 1: Windows nativo**

Eseguite il comando di installazione da PowerShell o CMD. Non è necessario eseguire come Amministratore. L'installazione di [Git for Windows](https://git-scm.com/downloads/win) è facoltativa. Abilita lo [strumento Bash](/docs/it/tools-reference#bash-tool-behavior) fornendo Git Bash.

Se eseguite l'installazione da PowerShell o CMD influisce solo su quale comando di installazione eseguite. Il vostro prompt mostra `PS C:\Users\YourName>` in PowerShell e `C:\Users\YourName>` senza il `PS` in CMD. Se siete nuovi al terminale, la [guida del terminale](/docs/it/terminal-guide#windows) vi guida attraverso ogni passaggio.

Dopo l'installazione, avviate `claude` da qualsiasi terminale.

* **Senza Git for Windows**, Claude Code esegue i comandi della shell tramite lo [strumento PowerShell](/docs/it/tools-reference#powershell-tool).
* **Con Git for Windows**, Claude Code utilizza Git Bash per lo [strumento Bash](/docs/it/tools-reference#bash-tool-behavior). Se Claude Code non riesce a trovare Git Bash, impostate il percorso nel vostro [file settings.json](/docs/it/settings):

  ```json theme={null}
  {
    "env": {
      "CLAUDE_CODE_GIT_BASH_PATH": "C:\\Program Files\\Git\\bin\\bash.exe"
    }
  }
  ```

Quando Git for Windows è installato, lo strumento PowerShell è in fase di rollout progressivo come opzione aggiuntiva insieme a Bash. Impostate `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` per attivare o `0` per disattivare. Consultate [PowerShell tool](/docs/it/tools-reference#powershell-tool) per la configurazione e le limitazioni.

**Opzione 2: WSL**

Aprite la vostra distribuzione WSL ed eseguite l'installer Linux dalle [istruzioni di installazione](#install-claude-code) sopra. Installate e avviate `claude` all'interno del terminale WSL, non da PowerShell o CMD.

<h3 id="alpine-linux-and-musl-based-distributions">
  Alpine Linux e distribuzioni basate su musl
</h3>

L'installer nativo su Alpine e altre distribuzioni basate su musl/uClibc richiede `libgcc`, `libstdc++` e `ripgrep`. Installate questi utilizzando il gestore di pacchetti della vostra distribuzione, quindi impostate `USE_BUILTIN_RIPGREP=0`.

Questo esempio installa i pacchetti richiesti su Alpine:

```bash theme={null}
apk add libgcc libstdc++ ripgrep
```

Quindi impostate `USE_BUILTIN_RIPGREP` a `0` nel vostro file [`settings.json`](/docs/it/settings#available-settings):

```json theme={null}
{
  "env": {
    "USE_BUILTIN_RIPGREP": "0"
  }
}
```

<h2 id="verify-your-installation">
  Verificare l'installazione
</h2>

Dopo l'installazione, confermate che Claude Code funziona:

```bash theme={null}
claude --version
```

Se questo fallisce con `command not found` o un altro errore, consultate [Risoluzione dei problemi di installazione e accesso](/docs/it/troubleshoot-install).

Per un controllo più dettagliato dell'installazione e della configurazione, eseguite [`claude doctor`](/docs/it/troubleshooting#get-more-help):

```bash theme={null}
claude doctor
```

<h2 id="authenticate">
  Autenticazione
</h2>

Claude Code richiede un account Pro, Max, Team, Enterprise o Console. Il piano gratuito di Claude.ai non include l'accesso a Claude Code. Potete anche utilizzare Claude Code con un provider API di terze parti come [Amazon Bedrock](/docs/it/amazon-bedrock), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai) o [Microsoft Foundry](/docs/it/microsoft-foundry).

Dopo l'installazione, accedete eseguendo `claude` e seguendo i prompt del browser. Consultate [Autenticazione](/docs/it/authentication) per tutti i tipi di account e le opzioni di configurazione del team.

<h2 id="update-claude-code">
  Aggiornare Claude Code
</h2>

Le installazioni native si aggiornano automaticamente in background. Potete [configurare il canale di rilascio](#configure-release-channel) per controllare se ricevere gli aggiornamenti immediatamente o secondo una pianificazione stabile ritardata, oppure [disabilitare gli aggiornamenti automatici](#disable-auto-updates) completamente. Le installazioni Homebrew, WinGet e [gestori di pacchetti Linux](#install-with-linux-package-managers) richiedono aggiornamenti manuali per impostazione predefinita.

<h3 id="auto-updates">
  Aggiornamenti automatici
</h3>

Claude Code verifica la disponibilità di aggiornamenti all'avvio e periodicamente durante l'esecuzione. Gli aggiornamenti si scaricano e si installano in background, quindi hanno effetto la prossima volta che avviate Claude Code.

Eseguite `claude doctor` per visualizzare il risultato del tentativo di aggiornamento più recente.

Su macOS e Linux, il programma di installazione nativo gestisce il launcher in `~/.local/bin/claude` come un symlink in `~/.local/share/claude/versions/`. Se sostituite quel launcher con il vostro script o symlink, l'aggiornamento automatico e `claude update` lo lasciano in posizione: le nuove versioni si installano comunque nella directory `versions/`, e il vostro launcher decide quale versione viene eseguita. Prima della v2.1.207, l'aggiornatore automatico sostituiva un launcher personalizzato in quel percorso con il suo symlink ad ogni aggiornamento.

Con un launcher personalizzato, Claude Code mantiene anche ogni versione installata su disco perché non può determinare quale versione il launcher necessita. `claude doctor` segnala un launcher che il programma di installazione nativo non ha creato.

Per consentire a Claude Code di gestire di nuovo il launcher, rimuovete `~/.local/bin/claude` ed eseguite `claude update`.

Se un'installazione globale npm non può aggiornarsi automaticamente perché la directory globale npm non è scrivibile, Claude Code mostra un avviso una sola volta all'avvio, e `claude doctor` elenca le correzioni disponibili. Consultate [errori di autorizzazione durante l'installazione](/docs/it/troubleshoot-install#permission-errors-during-installation) per i dettagli.

<Note>
  Le installazioni Homebrew, WinGet, apt, dnf e apk non si aggiornano automaticamente per impostazione predefinita; consultate di seguito per attivare l'opzione per Homebrew e WinGet. Per aggiornare Homebrew manualmente, eseguite `brew upgrade claude-code` o `brew upgrade claude-code@latest`, a seconda di quale cask avete installato. Per WinGet, eseguite `winget upgrade Anthropic.ClaudeCode`. Per i gestori di pacchetti Linux, consultate i comandi di aggiornamento in [Installare con gestori di pacchetti Linux](#install-with-linux-package-managers).

  Per fare in modo che Claude Code esegua il comando di aggiornamento per voi su Homebrew o WinGet, impostate [`CLAUDE_CODE_PACKAGE_MANAGER_AUTO_UPDATE`](/docs/it/env-vars) a `1`. Claude Code esegue quindi l'aggiornamento in background quando una nuova versione è disponibile e mostra un prompt di riavvio al completamento. L'aggiornamento riguarda solo il pacchetto Claude Code e non influisce su altro software che avete installato.

  Su WinGet l'aggiornamento potrebbe non riuscire mentre Claude Code è in esecuzione perché Windows blocca l'eseguibile. In questo caso Claude Code mostra il comando manuale. apt, dnf e apk continuano a richiedere un aggiornamento manuale perché questi comandi necessitano di privilegi elevati.

  **Problema noto:** Claude Code potrebbe notificarvi gli aggiornamenti prima che la nuova versione sia disponibile in questi gestori di pacchetti. Se un aggiornamento non riesce, attendete e riprovate più tardi.

  Homebrew mantiene le versioni precedenti su disco dopo gli aggiornamenti. Eseguite `brew cleanup` periodicamente per recuperare spazio su disco.
</Note>

<h3 id="configure-release-channel">
  Configurare il canale di rilascio
</h3>

Controllate quale canale di rilascio Claude Code segue per gli aggiornamenti automatici e `claude update` con l'impostazione `autoUpdatesChannel`:

* `"latest"`, l'impostazione predefinita: ricevete le nuove funzionalità non appena vengono rilasciate
* `"stable"`: utilizzate una versione che è tipicamente circa una settimana più vecchia, saltando i rilasci con regressioni importanti

Configurate questo tramite `/config` → **Auto-update channel**, oppure aggiungetelo al vostro [file settings.json](/docs/it/settings):

```json theme={null}
{
  "autoUpdatesChannel": "stable"
}
```

Per le distribuzioni aziendali, potete applicare un canale di rilascio coerente in tutta l'organizzazione utilizzando [impostazioni gestite](/docs/it/permissions#managed-settings).

Le installazioni Homebrew scelgono un canale in base al nome del cask invece di questa impostazione: `claude-code` traccia stable e `claude-code@latest` traccia latest.

<h3 id="pin-a-minimum-version">
  Fissare una versione minima
</h3>

L'impostazione `minimumVersion` stabilisce un limite inferiore. Gli aggiornamenti automatici in background e `claude update` rifiutano di installare qualsiasi versione al di sotto di questo valore, quindi il passaggio al canale `"stable"` non vi fa regredire se siete già su una build `"latest"` più recente.

Il passaggio da `"latest"` a `"stable"` tramite `/config` vi chiede di rimanere sulla versione corrente o di consentire il downgrade. Se scegliete di rimanere, viene impostato `minimumVersion` a quella versione. Il passaggio di nuovo a `"latest"` lo cancella.

Aggiungetelo al vostro [file settings.json](/docs/it/settings) per fissare un limite inferiore esplicitamente:

```json theme={null}
{
  "autoUpdatesChannel": "stable",
  "minimumVersion": "2.1.100"
}
```

Nelle [impostazioni gestite](/docs/it/permissions#managed-settings), questo applica un minimo a livello di organizzazione che le impostazioni utente e di progetto non possono ignorare.

Il pin `minimumVersion` vincola solo gli aggiornamenti. Per fare in modo che Claude Code rifiuti di avviarsi al di fuori di un intervallo di versioni, utilizzate invece le impostazioni gestite `requiredMinimumVersion` e `requiredMaximumVersion`. Gli aggiornamenti rispettano anche il limite `requiredMaximumVersion`. Consultate [impostazioni disponibili](/docs/it/settings#available-settings).

<h3 id="disable-auto-updates">
  Disabilitare gli aggiornamenti automatici
</h3>

Impostate `DISABLE_AUTOUPDATER` a `"1"` nella chiave `env` del vostro file [`settings.json`](/docs/it/settings#available-settings):

```json theme={null}
{
  "env": {
    "DISABLE_AUTOUPDATER": "1"
  }
}
```

`DISABLE_AUTOUPDATER` arresta solo il controllo in background; `claude update` e `claude install` continuano a funzionare. Per bloccare tutti i percorsi di aggiornamento, inclusi gli aggiornamenti manuali, impostate invece [`DISABLE_UPDATES`](/docs/it/env-vars). Utilizzate questo quando distribuite Claude Code attraverso i vostri canali e avete bisogno che gli utenti rimangano sulla versione che fornite.

<h3 id="update-manually">
  Aggiornare manualmente
</h3>

Per applicare un aggiornamento immediatamente senza attendere il prossimo controllo in background, eseguite:

```bash theme={null}
claude update
```

<h2 id="advanced-installation-options">
  Opzioni di installazione avanzate
</h2>

Queste opzioni sono per il pinning delle versioni, i gestori di pacchetti Linux, npm e la verifica dell'integrità dei binari.

<h3 id="install-a-specific-version">
  Installare una versione specifica
</h3>

L'installer nativo accetta un numero di versione specifico o un canale di rilascio (`latest` o `stable`). Il canale che scegliete al momento dell'installazione diventa il vostro predefinito per gli aggiornamenti automatici. Consultate [configurare il canale di rilascio](#configure-release-channel) per ulteriori informazioni.

Per installare la versione più recente (predefinita):

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    irm https://claude.ai/install.ps1 | iex
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
    ```
  </Tab>
</Tabs>

Per installare la versione stabile:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash -s stable
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    & ([scriptblock]::Create((irm https://claude.ai/install.ps1))) stable
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd stable && del install.cmd
    ```
  </Tab>
</Tabs>

Per installare un numero di versione specifico:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash -s 2.1.89
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    & ([scriptblock]::Create((irm https://claude.ai/install.ps1))) 2.1.89
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd 2.1.89 && del install.cmd
    ```
  </Tab>
</Tabs>

<h3 id="install-with-linux-package-managers">
  Installare con i gestori di pacchetti Linux
</h3>

Claude Code pubblica repository apt, dnf e apk firmati. Ogni repository offre due canali: `stable` fornisce una versione che è tipicamente circa una settimana vecchia, saltando i rilasci con regressioni importanti, e `latest` fornisce ogni rilascio non appena viene spedito. I comandi seguenti configurano il canale `stable`, che si adatta alla maggior parte degli utenti; ogni scheda mostra anche l'URL del repository `latest`. Le installazioni tramite gestore di pacchetti non si aggiornano automaticamente tramite Claude Code; gli aggiornamenti arrivano attraverso il vostro normale flusso di lavoro di aggiornamento del sistema.

Tutti i repository sono firmati con la [chiave di firma del rilascio di Claude Code](#binary-integrity-and-code-signing). Prima di fidarvi della chiave, verificatela come descritto in ogni scheda.

<Tabs>
  <Tab title="apt">
    Per Debian e Ubuntu. I comandi di installazione seguenti scaricano la chiave di firma con `curl`, che le installazioni fresche di Debian e Ubuntu potrebbero non includere. Se il download fallisce con `sudo: curl: command not found`, installate prima curl:

    ```bash theme={null}
    sudo apt install curl
    ```

    I comandi seguenti configurano il canale `stable`:

    ```bash theme={null}
    sudo install -d -m 0755 /etc/apt/keyrings
    sudo curl -fsSL https://downloads.claude.ai/keys/claude-code.asc \
      -o /etc/apt/keyrings/claude-code.asc
    echo "deb [signed-by=/etc/apt/keyrings/claude-code.asc] https://downloads.claude.ai/claude-code/apt/stable stable main" \
      | sudo tee /etc/apt/sources.list.d/claude-code.list
    sudo apt update
    sudo apt install claude-code
    ```

    Per utilizzare il canale `latest` invece, sia il percorso dell'URL che il nome della suite cambiano. Utilizzate questa riga `deb`:

    ```bash theme={null}
    echo "deb [signed-by=/etc/apt/keyrings/claude-code.asc] https://downloads.claude.ai/claude-code/apt/latest latest main" \
      | sudo tee /etc/apt/sources.list.d/claude-code.list
    ```

    Verificate l'impronta digitale della chiave GPG prima di fidarvi: `gpg --show-keys /etc/apt/keyrings/claude-code.asc` dovrebbe riportare `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE`.

    Per aggiornare in seguito, eseguite `sudo apt update && sudo apt upgrade claude-code`.
  </Tab>

  <Tab title="dnf">
    Per Fedora e RHEL. I comandi seguenti configurano il canale `stable`:

    ```bash theme={null}
    sudo tee /etc/yum.repos.d/claude-code.repo <<'EOF'
    [claude-code]
    name=Claude Code
    baseurl=https://downloads.claude.ai/claude-code/rpm/stable
    enabled=1
    gpgcheck=1
    gpgkey=https://downloads.claude.ai/keys/claude-code.asc
    EOF
    sudo dnf install claude-code
    ```

    Per utilizzare il canale `latest` invece, impostate `baseurl` al repository `latest`:

    ```ini theme={null}
    baseurl=https://downloads.claude.ai/claude-code/rpm/latest
    ```

    dnf scarica la chiave al primo install e vi chiede di confermare l'impronta digitale. Verificate che corrisponda a `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE` prima di accettare.

    Per aggiornare in seguito, eseguite `sudo dnf upgrade claude-code`.
  </Tab>

  <Tab title="apk">
    Per Alpine Linux. I comandi seguenti configurano il canale `stable`:

    ```sh theme={null}
    wget -O /etc/apk/keys/claude-code.rsa.pub \
      https://downloads.claude.ai/keys/claude-code.rsa.pub
    echo "https://downloads.claude.ai/claude-code/apk/stable" >> /etc/apk/repositories
    apk add claude-code
    ```

    Per passare al canale `latest`, rimuovete la riga del repository `stable` e aggiungete il repository `latest`:

    ```sh theme={null}
    sed -i '\|downloads.claude.ai/claude-code/apk/stable|d' /etc/apk/repositories
    echo "https://downloads.claude.ai/claude-code/apk/latest" >> /etc/apk/repositories
    ```

    Verificate la chiave scaricata con `sha256sum /etc/apk/keys/claude-code.rsa.pub`, che dovrebbe riportare `395759c1f7449ef4cdef305a42e820f3c766d6090d142634ebdb049f113168b6`.

    Per aggiornare in seguito, eseguite `apk update && apk upgrade claude-code`.
  </Tab>
</Tabs>

<h3 id="install-with-npm">
  Installare con npm
</h3>

Potete anche installare Claude Code come pacchetto npm globale. A partire dalla v2.1.198, il pacchetto npm richiede [Node.js 22 o successivo](https://nodejs.org/en/download). Su una versione più vecchia di Node.js, npm stampa un avviso `EBADENGINE` durante l'installazione piuttosto che fallire; l'installazione si completa e `claude` continua a funzionare, poiché il pacchetto scarica un binario nativo che non utilizza il vostro Node.js al runtime.

```bash theme={null}
npm install -g @anthropic-ai/claude-code
```

Il pacchetto npm installa lo stesso binario nativo dell'installer standalone. npm estrae il binario attraverso una dipendenza opzionale per piattaforma come `@anthropic-ai/claude-code-darwin-arm64`, e un passaggio postinstall lo collega in posizione. Il binario `claude` installato non invoca Node stesso.

Le piattaforme di installazione npm supportate sono `darwin-arm64`, `darwin-x64`, `linux-x64`, `linux-arm64`, `linux-x64-musl`, `linux-arm64-musl`, `win32-x64` e `win32-arm64`. Il vostro gestore di pacchetti deve consentire dipendenze opzionali. Consultate la [risoluzione dei problemi](/docs/it/troubleshoot-install#native-binary-not-found-after-npm-install) se il binario manca dopo l'installazione.

Per aggiornare un'installazione npm, eseguite `npm install -g @anthropic-ai/claude-code@latest`. Evitate `npm update -g`, che rispetta l'intervallo semver dall'installazione originale e potrebbe non portarvi al rilascio più recente.

<Warning>
  NON utilizzate `sudo npm install -g` poiché ciò può portare a problemi di permessi e rischi di sicurezza. Se riscontrate errori di permessi, consultate la [risoluzione dei problemi di permessi](/docs/it/troubleshoot-install#permission-errors-during-installation).
</Warning>

<h3 id="binary-integrity-and-code-signing">
  Integrità dei binari e firma del codice
</h3>

Ogni rilascio pubblica un `manifest.json` contenente checksum SHA256 per ogni binario di piattaforma. Il manifest è firmato con una chiave GPG di Anthropic, quindi la verifica della firma sul manifest verifica transitivamente ogni binario che elenca.

<h4 id="verify-the-manifest-signature">
  Verificare la firma del manifest
</h4>

I passaggi 1-3 richiedono una shell POSIX con `gpg` e `curl`. Su Windows, eseguiteli in Git Bash o WSL. Il passaggio 4 include un'opzione PowerShell.

<Steps>
  <Step title="Scaricare e importare la chiave pubblica">
    La chiave di firma del rilascio è pubblicata a un URL fisso.

    ```bash theme={null}
    curl -fsSL https://downloads.claude.ai/keys/claude-code.asc | gpg --import
    ```

    Visualizzate l'impronta digitale della chiave importata.

    ```bash theme={null}
    gpg --fingerprint security@anthropic.com
    ```

    Confermate che l'output includa questa impronta digitale:

    ```text theme={null}
    31DD DE24 DDFA B679 F42D  7BD2 BAA9 29FF 1A7E CACE
    ```
  </Step>

  <Step title="Scaricare il manifest e la firma">
    Impostate `VERSION` al rilascio che desiderate verificare.

    ```bash theme={null}
    REPO=https://downloads.claude.ai/claude-code-releases
    VERSION=2.1.89
    curl -fsSLO "$REPO/$VERSION/manifest.json"
    curl -fsSLO "$REPO/$VERSION/manifest.json.sig"
    ```
  </Step>

  <Step title="Verificare la firma">
    Verificate la firma staccata rispetto al manifest.

    ```bash theme={null}
    gpg --verify manifest.json.sig manifest.json
    ```

    Un risultato valido riporta `Good signature from "Anthropic Claude Code Release Signing <security@anthropic.com>"`.

    `gpg` stampa anche `WARNING: This key is not certified with a trusted signature!` per qualsiasi chiave appena importata. Questo è previsto. La riga `Good signature` conferma che il controllo crittografico è passato. Il confronto dell'impronta digitale nel Passaggio 1 conferma che la chiave stessa è autentica.
  </Step>

  <Step title="Controllare il binario rispetto al manifest">
    Confrontate il checksum SHA256 del binario con il valore elencato sotto `platforms.<platform>.checksum` in `manifest.json`. I comandi seguenti presuppongono un binario `claude` nella directory corrente. Per verificare un binario nativo installato invece, eseguite il comando rispetto a `~/.local/share/claude/versions/VERSION`, sostituendo VERSION con il rilascio che avete impostato nel Passaggio 2.

    <Tabs>
      <Tab title="Linux">
        ```bash theme={null}
        sha256sum claude
        ```
      </Tab>

      <Tab title="macOS">
        ```bash theme={null}
        shasum -a 256 claude
        ```
      </Tab>

      <Tab title="Windows PowerShell">
        ```powershell theme={null}
        (Get-FileHash claude.exe -Algorithm SHA256).Hash.ToLower()
        ```
      </Tab>
    </Tabs>
  </Step>
</Steps>

<Note>
  Le firme del manifest sono disponibili per i rilasci da `2.1.89` in poi. I rilasci precedenti pubblicano checksum in `manifest.json` senza una firma staccata.
</Note>

<h4 id="platform-code-signatures">
  Firme del codice della piattaforma
</h4>

Oltre al manifest firmato, i singoli binari portano firme del codice native della piattaforma dove supportate.

* **macOS**: firmato da "Anthropic PBC" e notarizzato da Apple. Verificate con `codesign --verify --verbose ./claude`.
* **Windows**: firmato da "Anthropic, PBC". Verificate con `Get-AuthenticodeSignature .\claude.exe`.
* **Linux**: i binari non sono individualmente firmati dal codice. Se scaricate direttamente dal bucket `claude-code-releases` o utilizzate l'installer nativo, verificate l'integrità con la firma del manifest sopra. Se installate con [apt, dnf o apk](#install-with-linux-package-managers), il vostro gestore di pacchetti verifica automaticamente le firme utilizzando la chiave di firma del repository.

<h2 id="uninstall-claude-code">
  Disinstallare Claude Code
</h2>

Per rimuovere Claude Code, seguite le istruzioni per il vostro metodo di installazione. Se `claude` continua a funzionare in seguito, probabilmente avete una seconda installazione o un alias shell residuo da un programma di installazione più vecchio. Consultate [Verificare le installazioni in conflitto](/docs/it/troubleshoot-install#check-for-conflicting-installations) per trovarla e rimuoverla.

<h3 id="native-installation">
  Installazione nativa
</h3>

Rimuovete il binario di Claude Code e i file di versione:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    rm -f ~/.local/bin/claude
    rm -rf ~/.local/share/claude
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    Remove-Item -Path "$env:USERPROFILE\.local\bin\claude.exe" -Force
    Remove-Item -Path "$env:USERPROFILE\.local\share\claude" -Recurse -Force
    ```
  </Tab>
</Tabs>

<h3 id="homebrew-installation">
  Installazione Homebrew
</h3>

Rimuovete il cask Homebrew che avete installato. Se avete installato il cask stabile:

```bash theme={null}
brew uninstall --cask claude-code
```

Se avete installato il cask latest:

```bash theme={null}
brew uninstall --cask claude-code@latest
```

<h3 id="winget-installation">
  Installazione WinGet
</h3>

Rimuovete il pacchetto WinGet:

```powershell theme={null}
winget uninstall Anthropic.ClaudeCode
```

<h3 id="apt-/-dnf-/-apk">
  apt / dnf / apk
</h3>

Rimuovete il pacchetto e la configurazione del repository:

<Tabs>
  <Tab title="apt">
    ```bash theme={null}
    sudo apt remove claude-code
    sudo rm /etc/apt/sources.list.d/claude-code.list /etc/apt/keyrings/claude-code.asc
    ```
  </Tab>

  <Tab title="dnf">
    ```bash theme={null}
    sudo dnf remove claude-code
    sudo rm /etc/yum.repos.d/claude-code.repo
    ```
  </Tab>

  <Tab title="apk">
    ```sh theme={null}
    apk del claude-code
    sed -i '\|downloads.claude.ai/claude-code/apk|d' /etc/apk/repositories
    rm /etc/apk/keys/claude-code.rsa.pub
    ```
  </Tab>
</Tabs>

<h3 id="npm">
  npm
</h3>

Rimuovete il pacchetto npm globale:

```bash theme={null}
npm uninstall -g @anthropic-ai/claude-code
```

<h3 id="remove-configuration-files">
  Rimuovere i file di configurazione
</h3>

<Warning>
  La rimozione dei file di configurazione eliminerà tutte le vostre impostazioni, gli strumenti consentiti, le configurazioni del server MCP e la cronologia delle sessioni.
</Warning>

L'estensione VS Code, il plugin JetBrains e l'app Desktop scrivono anche in `~/.claude/`. Se uno di essi è ancora installato, la directory viene ricreata la prossima volta che viene eseguito. Per rimuovere Claude Code completamente, disinstallate l'[estensione VS Code](/docs/it/vs-code#uninstall-the-extension), il plugin JetBrains e l'app Desktop prima di eliminare questi file.

Per rimuovere le impostazioni di Claude Code e i dati memorizzati nella cache:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    # Rimuovere le impostazioni utente e lo stato
    rm -rf ~/.claude
    rm ~/.claude.json

    # Rimuovere le impostazioni specifiche del progetto (eseguire dalla directory del progetto)
    rm -rf .claude
    rm -f .mcp.json
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    # Rimuovere le impostazioni utente e lo stato
    Remove-Item -Path "$env:USERPROFILE\.claude" -Recurse -Force
    Remove-Item -Path "$env:USERPROFILE\.claude.json" -Force

    # Rimuovere le impostazioni specifiche del progetto (eseguire dalla directory del progetto)
    Remove-Item -Path ".claude" -Recurse -Force
    Remove-Item -Path ".mcp.json" -Force
    ```
  </Tab>
</Tabs>
