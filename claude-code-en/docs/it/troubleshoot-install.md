> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Risolvi i problemi di installazione e accesso

> Correggi gli errori di comando non trovato, PATH, permessi, rete e autenticazione durante l'installazione o l'accesso a Claude Code.

Se l'installazione non riesce o non riesci ad accedere, trova il tuo errore di seguito. Per i problemi di runtime dopo che Claude Code è funzionante, vedi [Risoluzione dei problemi](/docs/it/troubleshooting). Per i problemi di configurazione come impostazioni non applicate o hook non attivati, vedi [Debug della tua configurazione](/docs/it/debug-your-config).

<h2 id="find-your-error">
  Trova il tuo errore
</h2>

Abbina il messaggio di errore o il sintomo che stai vedendo a una soluzione:

| Quello che vedi                                                                                                        | Soluzione                                                                                                                                    |
| :--------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------- |
| `command not found: claude` o `'claude' is not recognized`                                                             | [Correggi il tuo PATH](#command-not-found-claude-after-installation)                                                                         |
| `syntax error near unexpected token '<'`                                                                               | [Lo script di installazione restituisce HTML](#install-script-returns-html-instead-of-a-shell-script)                                        |
| `curl: (22) The requested URL returned error: 403`                                                                     | [Lo script di installazione ha restituito 403](#install-script-returns-html-instead-of-a-shell-script)                                       |
| `curl: (23)` o `curl: (56) Failure writing output to destination`                                                      | [Controlla la connettività o usa un programma di installazione alternativo](#curl-56-failure-writing-output-to-destination)                  |
| `Killed` durante l'installazione su Linux, o `Installation was killed before it could finish (exit code 137)`          | [Libera memoria o aggiungi spazio di swap](#install-killed-on-low-memory-linux-servers)                                                      |
| `TLS connect error` o `SSL/TLS secure channel`                                                                         | [Aggiorna i certificati CA](#tls-or-ssl-connection-errors)                                                                                   |
| `Failed to fetch version` o impossibile raggiungere il server di download                                              | [Controlla le impostazioni di rete e proxy](#check-network-connectivity)                                                                     |
| `irm is not recognized` o `&& is not valid`                                                                            | [Usa il comando giusto per la tua shell](#wrong-install-command-on-windows)                                                                  |
| `Cask 'claude-code' is unavailable: No Cask with this name exists`                                                     | [Aggiorna Homebrew](#homebrew-cask-unavailable-or-outdated)                                                                                  |
| `'bash' is not recognized as the name of a cmdlet`                                                                     | [Usa il comando del programma di installazione di Windows](#wrong-install-command-on-windows)                                                |
| `A parameter cannot be found that matches parameter name 'fsSL'`                                                       | [Usa il comando del programma di installazione di Windows](#wrong-install-command-on-windows)                                                |
| `Claude Code on Windows requires either Git for Windows (for bash) or PowerShell`                                      | [Installa una shell](#claude-code-on-windows-requires-either-git-for-windows-for-bash-or-powershell)                                         |
| `Claude Code does not support 32-bit Windows`                                                                          | [Apri Windows PowerShell, non la voce x86](#claude-code-does-not-support-32-bit-windows)                                                     |
| `The process cannot access the file ... because it is being used by another process`                                   | [Svuota la cartella dei download e riprova](#the-process-cannot-access-the-file-during-windows-install)                                      |
| `Error loading shared library`                                                                                         | [Variante binaria sbagliata per il tuo sistema](#linux-musl-or-glibc-binary-mismatch)                                                        |
| `Illegal instruction`                                                                                                  | [Mancata corrispondenza dell'architettura o del set di istruzioni della CPU](#illegal-instruction)                                           |
| `cannot execute binary file: Exec format error` in WSL                                                                 | [Regressione binaria nativa WSL1](#exec-format-error-on-wsl1)                                                                                |
| Il programma di installazione di PowerShell si completa ma `claude` non viene trovato o mostra una versione precedente | [Aggiungi la directory di installazione al tuo PATH](#verify-your-path), quindi apri un nuovo terminale                                      |
| `dyld: cannot load`, `dyld: Symbol not found`, o `Abort trap` su macOS                                                 | [Incompatibilità binaria](#dyld-cannot-load-on-macos)                                                                                        |
| `Invoke-Expression: Missing argument in parameter list`                                                                | [Lo script di installazione restituisce HTML](#install-script-returns-html-instead-of-a-shell-script)                                        |
| `App unavailable in region`                                                                                            | Claude Code non è disponibile nel tuo paese. Vedi [paesi supportati](https://www.anthropic.com/supported-countries).                         |
| `unable to get local issuer certificate`                                                                               | [Configura i certificati CA aziendali](#tls-or-ssl-connection-errors)                                                                        |
| `OAuth error` o `403 Forbidden`                                                                                        | [Correggi l'autenticazione](#login-and-authentication)                                                                                       |
| `Could not load the default credentials` o `Could not load credentials from any providers`                             | [Credenziali Amazon Bedrock, Google Cloud's Agent Platform, o Microsoft Foundry](#bedrock-agent-platform-or-foundry-credentials-not-loading) |
| `ChainedTokenCredential authentication failed` o `CredentialUnavailableError`                                          | [Credenziali Amazon Bedrock, Google Cloud's Agent Platform, o Microsoft Foundry](#bedrock-agent-platform-or-foundry-credentials-not-loading) |
| `API Error: 500`, `529 Overloaded`, `429`, o altri errori 4xx e 5xx non elencati sopra                                 | Vedi il [riferimento degli errori](/docs/it/errors)                                                                                               |

Se il tuo problema non è elencato, esegui i controlli diagnostici di seguito per restringere la causa.

<Tip>
  Se preferisci saltare completamente il terminale, l'[app Claude Code Desktop](/docs/it/desktop-quickstart) ti consente di installare e utilizzare Claude Code tramite un'interfaccia grafica. Scaricala per [macOS](https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code\&utm_medium=docs) o [Windows](https://claude.com/download?utm_source=claude_code\&utm_medium=docs) e inizia a codificare senza alcuna configurazione da riga di comando. Su Linux, installa l'app con apt seguendo le [istruzioni di installazione per Linux](/docs/it/desktop-linux).
</Tip>

<h2 id="run-diagnostic-checks">
  Esegui controlli diagnostici
</h2>

<h3 id="check-network-connectivity">
  Controlla la connettività di rete
</h3>

Il programma di installazione scarica da `downloads.claude.ai`. Verifica di poterlo raggiungere:

```bash theme={null}
curl -sI https://downloads.claude.ai/claude-code-releases/latest
```

In PowerShell, esegui `curl.exe -sI` invece. PowerShell crea un alias di `curl` a `Invoke-WebRequest`, che rifiuta i flag `-sI`.

Una riga `HTTP/2 200` significa che hai raggiunto il server. Se non vedi output, `Could not resolve host`, o un timeout di connessione, la tua rete sta bloccando la connessione. Le cause comuni sono:

* Firewall aziendali o proxy che bloccano `downloads.claude.ai`
* Restrizioni di rete regionali: prova una VPN o una rete alternativa
* Problemi TLS/SSL: aggiorna i certificati CA del tuo sistema, o controlla se `HTTPS_PROXY` è configurato

Se sei dietro un proxy aziendale, imposta `HTTPS_PROXY` e `HTTP_PROXY` all'indirizzo del tuo proxy prima di installare. Chiedi al tuo team IT l'URL del proxy se non lo conosci, o controlla le impostazioni del proxy del tuo browser.

Questo esempio imposta entrambe le variabili proxy, quindi esegue il programma di installazione attraverso il tuo proxy:

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    export HTTP_PROXY=http://proxy.example.com:8080
    export HTTPS_PROXY=http://proxy.example.com:8080
    curl -fsSL https://claude.ai/install.sh | bash
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    $env:HTTP_PROXY = 'http://proxy.example.com:8080'
    $env:HTTPS_PROXY = 'http://proxy.example.com:8080'
    irm https://claude.ai/install.ps1 | iex
    ```
  </Tab>
</Tabs>

<h3 id="verify-your-path">
  Verifica il tuo PATH
</h3>

Se l'installazione è riuscita ma ricevi un errore `command not found` o `not recognized` quando esegui `claude`, la directory di installazione non è nel tuo PATH. La tua shell cerca i programmi nelle directory elencate in PATH, e il programma di installazione posiziona `claude` in `~/.local/bin/claude` su macOS/Linux o `%USERPROFILE%\.local\bin\claude.exe` su Windows.

<Note>
  L'[estensione VS Code](/docs/it/vs-code) non posiziona `claude` in questa posizione. Raggruppa una copia privata della CLI all'interno della directory dell'estensione per il suo pannello di chat e non la aggiunge a PATH. Se hai installato solo l'estensione, `~/.local/bin/claude` non esisterà. Esegui l'[installazione standalone](/docs/it/setup) per utilizzare `claude` da un terminale, quindi continua di seguito.
</Note>

Controlla se la directory di installazione è nel tuo PATH elencando le tue voci PATH e filtrando per `local/bin`:

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    echo $PATH | tr ':' '\n' | grep -Fx "$HOME/.local/bin"
    ```

    Se questo stampa `/Users/you/.local/bin` o `/home/you/.local/bin`, la directory è nel tuo PATH e puoi saltare a [Controlla le installazioni in conflitto](#check-for-conflicting-installations). Se non c'è output, aggiungilo alla tua configurazione shell.

    Per Zsh, il default su macOS:

    ```bash theme={null}
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
    source ~/.zshrc
    ```

    Per Bash, il default sulla maggior parte delle distribuzioni Linux:

    ```bash theme={null}
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
    source ~/.bashrc
    ```

    In alternativa, chiudi e riapri il tuo terminale.

    Per altri shell come fish o Nushell, aggiungi `~/.local/bin` al tuo PATH usando la sintassi di configurazione del tuo shell, quindi riavvia il tuo terminale.

    Verifica che la correzione abbia funzionato:

    ```bash theme={null}
    claude --version
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    $env:PATH -split ';' | Select-String '\.local\\bin'
    ```

    Se non c'è output, aggiungi la directory di installazione al tuo User PATH:

    ```powershell theme={null}
    $currentPath = [Environment]::GetEnvironmentVariable('PATH', 'User')
    [Environment]::SetEnvironmentVariable('PATH', "$currentPath;$env:USERPROFILE\.local\bin", 'User')
    ```

    Riavvia il tuo terminale affinché la modifica abbia effetto.

    Verifica che la correzione abbia funzionato:

    ```powershell theme={null}
    claude --version
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    echo %PATH% | findstr /i "local\bin"
    ```

    Se non c'è output, apri Impostazioni di sistema, vai a Variabili di ambiente, e aggiungi `%USERPROFILE%\.local\bin` alla tua variabile User PATH. Riavvia il tuo terminale.

    Verifica che la correzione abbia funzionato:

    ```batch theme={null}
    claude --version
    ```
  </Tab>
</Tabs>

<h3 id="check-for-conflicting-installations">
  Controlla le installazioni in conflitto
</h3>

Più installazioni di Claude Code possono causare mancate corrispondenze di versione o comportamenti inaspettati. Controlla cosa è installato:

<Tabs>
  <Tab title="macOS/Linux">
    Elenca tutti i binari `claude` trovati nel tuo PATH:

    ```bash theme={null}
    which -a claude
    ```

    Se questo non stampa nulla, nessun `claude` è ancora nel tuo PATH. Torna a [Verifica il tuo PATH](#verify-your-path).

    Controlla le tre posizioni da cui un binario `claude` può provenire. `~/.local/bin/claude` è il programma di installazione nativo, `~/.claude/local/` è un'installazione npm locale legacy creata da versioni precedenti di Claude Code, e l'elenco npm globale mostra un'installazione `-g`:

    ```bash theme={null}
    ls -la ~/.local/bin/claude
    ```

    Un'installazione nativa mostra un collegamento simbolico in `~/.local/share/claude/versions/`. Uno script o un collegamento simbolico che hai creato tu stesso in questo percorso è un launcher personalizzato, che [l'aggiornamento automatico lascia in posizione](/docs/it/setup#auto-updates).

    Se uno dei comandi `ls` stampa `No such file or directory`, non è un errore. Significa che nulla è installato in quella posizione, quindi passa al controllo successivo.

    ```bash theme={null}
    ls -la ~/.claude/local/
    ```

    ```bash theme={null}
    npm -g ls @anthropic-ai/claude-code 2>/dev/null
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    Elenca tutti i binari `claude` trovati nel tuo PATH:

    ```powershell theme={null}
    where.exe claude
    ```

    Controlla se il programma di installazione nativo ha posizionato un binario:

    ```powershell theme={null}
    Test-Path "$env:USERPROFILE\.local\bin\claude.exe"
    ```
  </Tab>
</Tabs>

Se trovi più installazioni, mantieni solo una. L'installazione nativa in `~/.local/bin/claude` su macOS/Linux o `%USERPROFILE%\.local\bin\claude.exe` su Windows è consigliata. Rimuovi le altre:

Disinstalla un'installazione npm globale:

```bash theme={null}
npm uninstall -g @anthropic-ai/claude-code
```

Rimuovi l'installazione npm locale legacy:

```bash theme={null}
rm -rf ~/.claude/local
```

Su Windows, usa PowerShell:

```powershell theme={null}
Remove-Item -Recurse -Force "$env:USERPROFILE\.claude\local"
```

Rimuovi un'installazione Homebrew su macOS. Se hai installato il cask `claude-code@latest`, sostituisci quel nome:

```bash theme={null}
brew uninstall --cask claude-code
```

Rimuovi un'installazione WinGet su Windows:

```powershell theme={null}
winget uninstall Anthropic.ClaudeCode
```

<h3 id="check-directory-permissions">
  Controlla i permessi della directory
</h3>

Il programma di installazione ha bisogno di accesso in scrittura a `~/.local/bin/` e `~/.claude/` su macOS e Linux. Su Windows la posizione di installazione è sotto `%USERPROFILE%`, che è scrivibile dal tuo utente per impostazione predefinita, quindi questa sezione raramente si applica lì.

Controlla se le directory sono scrivibili:

```bash theme={null}
test -w ~/.local/bin && echo "writable" || echo "not writable"
test -w ~/.claude && echo "writable" || echo "not writable"
```

Se una directory non è scrivibile, crea la directory di installazione e imposta il tuo utente come proprietario:

```bash theme={null}
sudo mkdir -p ~/.local/bin
sudo chown -R $(whoami) ~/.local
```

<h3 id="verify-the-binary-works">
  Verifica che il binario funzioni
</h3>

Se `claude --version` stampa una versione ma `claude` si arresta in modo anomalo o si blocca all'avvio, esegui questi controlli per restringere la causa. Se `claude --version` dice comando non trovato, vai a [Verifica il tuo PATH](#verify-your-path) prima; i comandi di seguito presuppongono che `claude` sia nel tuo PATH.

Conferma che il binario esiste ed è eseguibile:

```bash theme={null}
ls -la "$(command -v claude)"
```

Su Windows, usa PowerShell:

```powershell theme={null}
Get-Command claude | Select-Object Source
```

Su Linux, controlla le librerie condivise mancanti. Se `ldd` mostra librerie mancanti, potrebbe essere necessario installare pacchetti di sistema. Su Alpine Linux e altre distribuzioni basate su musl, vedi [Configurazione di Alpine Linux](/docs/it/setup#alpine-linux-and-musl-based-distributions).

```bash theme={null}
ldd "$(command -v claude)" | grep "not found"
```

Conferma che il binario può essere eseguito:

```bash theme={null}
claude --version
```

<h2 id="common-installation-issues">
  Problemi di installazione comuni
</h2>

Questi sono i problemi di installazione più frequentemente riscontrati e le loro soluzioni.

<h3 id="install-script-returns-html-instead-of-a-shell-script">
  Lo script di installazione restituisce HTML invece di uno script shell
</h3>

Quando esegui il comando di installazione, potresti vedere uno di questi errori:

```text theme={null}
bash: line 1: syntax error near unexpected token `<'
bash: line 1: `<!DOCTYPE html>'
```

Su PowerShell, lo stesso problema appare come:

```text theme={null}
Invoke-Expression: Missing argument in parameter list.
```

A seconda di come la richiesta è stata instradata, potresti invece vedere un 403 senza corpo HTML:

```text theme={null}
curl: (22) The requested URL returned error: 403
```

Tutti questi significano che l'URL di installazione ha restituito una pagina HTML o uno stato di errore invece dello script di installazione. Se la pagina HTML dice "App unavailable in region," Claude Code non è disponibile nel tuo paese. Vedi [paesi supportati](https://www.anthropic.com/supported-countries).

Un 403 nudo senza corpo spesso ha la stessa causa, ma può anche provenire da un proxy aziendale o da un firewall che blocca il download. Se sei in un paese supportato e vedi ancora il 403, esamina [Controlla la connettività di rete](#check-network-connectivity) prima di provare i programmi di installazione alternativi di seguito, poiché quelli raggiungono gli stessi host.

Altrimenti, questo può accadere a causa di problemi di rete, routing regionale, o un'interruzione temporanea del servizio.

**Soluzioni:**

1. **Usa un metodo di installazione alternativo**:

   Su macOS, installa tramite Homebrew:

   ```bash theme={null}
   brew install --cask claude-code
   ```

   Su Windows, installa tramite WinGet:

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

2. **Riprova dopo alcuni minuti**: il problema è spesso temporaneo. Aspetta e prova di nuovo il comando originale.

<h3 id="command-not-found-claude-after-installation">
  `command not found: claude` dopo l'installazione
</h3>

L'installazione è terminata ma `claude` non funziona. L'errore esatto varia in base alla piattaforma:

| Piattaforma | Messaggio di errore                                                    |
| :---------- | :--------------------------------------------------------------------- |
| macOS       | `zsh: command not found: claude`                                       |
| Linux       | `bash: claude: command not found`                                      |
| Windows CMD | `'claude' is not recognized as an internal or external command`        |
| PowerShell  | `claude : The term 'claude' is not recognized as the name of a cmdlet` |

Questo significa che la directory di installazione non è nel percorso di ricerca della tua shell. Vedi [Verifica il tuo PATH](#verify-your-path) per la correzione su ogni piattaforma.

<h3 id="curl-56-failure-writing-output-to-destination">
  `curl: (56) Failure writing output to destination`
</h3>

Il comando `curl ... | bash` scarica lo script e lo invia a Bash per l'esecuzione. Questo errore, e l'errore correlato `curl: (23) Failure writing output to destination`, significa che Bash non ha ricevuto lo script completo. Il codice di uscita 56 indica che il download stesso è stato interrotto, e il codice di uscita 23 indica che curl non ha potuto scrivere quello che ha ricevuto al pipe, di solito perché Bash è uscito anticipatamente.

**Soluzioni:**

1. **Controlla la stabilità della rete**: i binari di Claude Code sono ospitati in `downloads.claude.ai`. Testa che puoi raggiungerlo:
   ```bash theme={null}
   curl -sI https://downloads.claude.ai/claude-code-releases/latest
   ```
   Una riga `HTTP/2 200` significa che hai raggiunto il server e il fallimento originale era probabilmente intermittente; riprova il comando di installazione. Se vedi `Could not resolve host` o un timeout di connessione, la tua rete sta bloccando il download.

2. **Prova un metodo di installazione alternativo**:

   Su macOS:

   ```bash theme={null}
   brew install --cask claude-code
   ```

   Su Windows:

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

<h3 id="homebrew-cask-unavailable-or-outdated">
  Cask Homebrew non disponibile o obsoleto
</h3>

Homebrew segnala `Error: Cask 'claude-code' is unavailable: No Cask with this name exists` quando la tua copia locale dell'indice cask di Homebrew è precedente alla pubblicazione del cask. Aggiorna l'indice e riprova:

```bash theme={null}
brew update
brew install --cask claude-code
```

Se Homebrew installa una versione di Claude Code più vecchia di quella che ti aspetti, di solito la stessa causa è l'indice obsoleto. Il cask `claude-code` traccia il canale stabile ed è in genere circa una settimana dietro l'ultima versione; per la versione più recente esegui `brew install --cask claude-code@latest` invece. Vedi [Configura il canale di rilascio](/docs/it/setup#configure-release-channel) per la differenza tra i due cask.

<h3 id="tls-or-ssl-connection-errors">
  Errori di connessione TLS o SSL
</h3>

Errori come `curl: (35) TLS connect error`, `schannel: next InitializeSecurityContext failed`, o il `Could not establish trust relationship for the SSL/TLS secure channel` di PowerShell indicano fallimenti dell'handshake TLS.

**Soluzioni:**

1. **Aggiorna i certificati CA del tuo sistema**:

   Su Ubuntu/Debian:

   ```bash theme={null}
   sudo apt-get update && sudo apt-get install ca-certificates
   ```

   Su macOS, il curl di sistema utilizza l'archivio di fiducia Keychain; l'aggiornamento di macOS stesso aggiorna i certificati root.

2. **Su Windows, abilita TLS 1.2** in PowerShell prima di eseguire il programma di installazione:
   ```powershell theme={null}
   [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
   irm https://claude.ai/install.ps1 | iex
   ```

3. **Controlla l'interferenza del proxy o del firewall**: i proxy aziendali che eseguono l'ispezione TLS possono causare questi errori, inclusi `unable to get local issuer certificate` e `SELF_SIGNED_CERT_IN_CHAIN`. Per il passaggio di installazione, punta curl al tuo bundle CA aziendale con `--cacert`:
   ```bash theme={null}
   curl --cacert /path/to/corporate-ca.pem -fsSL https://claude.ai/install.sh | bash
   ```
   Per Claude Code stesso una volta installato, imposta `NODE_EXTRA_CA_CERTS` in modo che le richieste API si fidino dello stesso bundle:
   ```bash theme={null}
   export NODE_EXTRA_CA_CERTS=/path/to/corporate-ca.pem
   ```
   Chiedi al tuo team IT il file di certificato se non lo hai. Puoi anche provare su una connessione diretta per confermare che il proxy è la causa.

4. **Su Windows, cambia programmi di installazione se la tua rete blocca i controlli di revoca**. Gli errori `CRYPT_E_NO_REVOCATION_CHECK (0x80092012)` e `CRYPT_E_REVOCATION_OFFLINE (0x80092013)` significano che curl ha raggiunto il server ma la tua rete blocca la ricerca di revoca del certificato, che è comune dietro firewall aziendali. Aggiungere il flag `--ssl-revoke-best-effort` di curl non risolve questo: il flag si applica solo al download di `install.cmd` stesso, e i download dello script stesso vengono eseguiti senza di esso, quindi l'installazione fallisce con lo stesso errore. Usa un metodo di installazione che tollera la ricerca bloccata invece. Apri PowerShell ed esegui il programma di installazione PowerShell, che scarica tramite .NET e non fallisce quando il server di revoca è irraggiungibile:
   ```powershell theme={null}
   irm https://claude.ai/install.ps1 | iex
   ```
   Puoi anche installare con `winget install Anthropic.ClaudeCode`, che evita curl completamente.

<h3 id="failed-to-fetch-version-from-downloads-claude-ai">
  `Failed to fetch version from downloads.claude.ai`
</h3>

Il programma di installazione non ha potuto raggiungere il server di download. Questo in genere significa che `downloads.claude.ai` è bloccato sulla tua rete.

**Soluzioni:**

1. **Testa la connettività direttamente**:
   ```bash theme={null}
   curl -sI https://downloads.claude.ai/claude-code-releases/latest
   ```

2. **Se dietro un proxy**, imposta `HTTPS_PROXY` in modo che il programma di installazione possa instradarlo attraverso. Vedi [configurazione del proxy](/docs/it/network-config#proxy-configuration) per i dettagli.
   ```bash theme={null}
   export HTTPS_PROXY=http://proxy.example.com:8080
   curl -fsSL https://claude.ai/install.sh | bash
   ```

3. **Se su una rete ristretta**, prova una rete diversa o una VPN, o usa un metodo di installazione alternativo:

   Su macOS:

   ```bash theme={null}
   brew install --cask claude-code
   ```

   Su Windows:

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

<h3 id="wrong-install-command-on-windows">
  Comando di installazione sbagliato su Windows
</h3>

Se vedi `'irm' is not recognized`, `The token '&&' is not valid`, `A parameter cannot be found that matches parameter name 'fsSL'`, o `'bash' is not recognized as the name of a cmdlet`, hai copiato il comando di installazione per una shell o un sistema operativo diverso.

* **`irm` non riconosciuto**: sei in CMD, non PowerShell. Hai due opzioni:

  Apri PowerShell cercando "PowerShell" nel menu Start, quindi esegui il comando di installazione originale:

  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

  Oppure rimani in CMD e usa il programma di installazione CMD invece:

  ```batch theme={null}
  curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
  ```

* **`&&` non valido**: sei in PowerShell ma hai eseguito il comando del programma di installazione CMD. Usa il programma di installazione PowerShell:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

* **`A parameter cannot be found that matches parameter name 'fsSL'`**: hai eseguito il programma di installazione macOS/Linux `curl -fsSL ... | bash` in Windows PowerShell, dove `curl` è un alias per `Invoke-WebRequest` e rifiuta i flag `-fsSL`. Usa il programma di installazione PowerShell invece:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

* **`bash` non riconosciuto**: hai eseguito il programma di installazione macOS/Linux su Windows. Usa il programma di installazione PowerShell invece:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

<h3 id="the-process-cannot-access-the-file-during-windows-install">
  `The process cannot access the file` durante l'installazione su Windows
</h3>

Se il programma di installazione PowerShell non riesce con `Failed to download binary: The process cannot access the file ... because it is being used by another process`, il programma di installazione non ha potuto scrivere in `%USERPROFILE%\.claude\downloads`. Questo di solito significa che un tentativo di installazione precedente è ancora in esecuzione, o il software antivirus sta scansionando un binario parzialmente scaricato in quella cartella.

Chiudi tutte le altre finestre di PowerShell che eseguono il programma di installazione e aspetta che le scansioni antivirus rilascino il file. Quindi elimina la cartella dei download e esegui di nuovo il programma di installazione:

```powershell theme={null}
Remove-Item -Recurse -Force "$env:USERPROFILE\.claude\downloads"
irm https://claude.ai/install.ps1 | iex
```

<h3 id="install-killed-on-low-memory-linux-servers">
  L'installazione viene interrotta su server Linux a bassa memoria
</h3>

Un messaggio `Killed` durante l'installazione di solito significa che il killer OOM (out-of-memory) di Linux ha terminato il passaggio `claude install` perché il sistema ha esaurito la memoria libera. Questo è comune su piccoli VPS e istanze cloud. Lo script di installazione segnala la causa e esce con codice 137:

```text theme={null}
Setting up Claude Code...
bash: line 142: 34803 Killed    "$binary_path" install ${TARGET:+"$TARGET"}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

Prima della v2.1.200, lo script è uscito con solo la riga `Killed` nuda della shell e nessuna spiegazione.

L'installazione richiede approssimativamente 512 MB di memoria libera, e l'esecuzione di Claude Code ne richiede di più. Vedi i [requisiti di sistema](/docs/it/setup#system-requirements).

**Soluzioni:**

1. **Aggiungi spazio di swap** se il tuo server ha RAM limitata. Lo swap utilizza lo spazio su disco come memoria di overflow, consentendo al programma di installazione di completarsi anche con RAM fisica bassa.

   Crea un file di swap da 2 GB e abilitalo:

   ```bash theme={null}
   sudo fallocate -l 2G /swapfile
   sudo chmod 600 /swapfile
   sudo mkswap /swapfile
   sudo swapon /swapfile
   ```

   Quindi riprova l'installazione:

   ```bash theme={null}
   curl -fsSL https://claude.ai/install.sh | bash
   ```

2. **Chiudi altri processi** per liberare memoria prima di installare.

3. **Usa un'istanza più grande** se possibile. Claude Code richiede almeno 4 GB di RAM.

<h3 id="install-hangs-in-docker">
  L'installazione si blocca in Docker
</h3>

Quando installi Claude Code in un contenitore Docker, l'installazione come root in `/` può causare blocchi.

**Soluzioni:**

1. **Imposta una directory di lavoro** prima di eseguire il programma di installazione. Quando eseguito da `/`, il programma di installazione scansiona l'intero filesystem, il che causa un utilizzo eccessivo della memoria. L'impostazione di `WORKDIR` limita la scansione a una piccola directory:
   ```dockerfile theme={null}
   WORKDIR /tmp
   RUN curl -fsSL https://claude.ai/install.sh | bash
   ```

2. **Aumenta i limiti di memoria di Docker** se usi Docker Desktop:
   ```bash theme={null}
   docker build --memory=4g .
   ```

<h3 id="claude-desktop-overrides-the-claude-command-on-windows">
  Claude Desktop sostituisce il comando `claude` su Windows
</h3>

Se hai installato una versione precedente di Claude Desktop, potrebbe registrare un `Claude.exe` nella directory `WindowsApps` che ha priorità nel PATH rispetto a Claude Code CLI. L'esecuzione di `claude` apre l'app Desktop invece della CLI.

Aggiorna Claude Desktop alla versione più recente per risolvere questo problema.

<h3 id="claude-code-on-windows-requires-either-git-for-windows-for-bash-or-powershell">
  Claude Code su Windows richiede Git for Windows (per bash) o PowerShell
</h3>

Git for Windows è facoltativo. Claude Code utilizza lo [strumento PowerShell](/docs/it/tools-reference#powershell-tool) quando Git Bash è assente, quindi questo errore significa che nessuna delle due shell è stata trovata.

**Se PowerShell manca dal tuo PATH**, la sua posizione predefinita è `C:\Windows\System32\WindowsPowerShell\v1.0\`. Aggiungi quella directory al tuo `PATH`, o installa [PowerShell 7](https://aka.ms/powershell), che fornisce `pwsh`.

**Per installare Git for Windows invece**, scaricalo da [git-scm.com/downloads/win](https://git-scm.com/downloads/win). Durante la configurazione, seleziona "Add to PATH." Riavvia il tuo terminale dopo l'installazione. L'installazione abilita lo strumento Bash, utile quando si lavora con script e strumenti basati su Bash.

**Se Git è già installato** ma Claude Code non riesce a trovarlo, imposta il percorso nel tuo [file settings.json](/docs/it/settings):

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_GIT_BASH_PATH": "C:\\Program Files\\Git\\bin\\bash.exe"
  }
}
```

Se il tuo Git è installato da qualche altra parte, trova il percorso eseguendo `where.exe git` in PowerShell e usa il percorso `bin\bash.exe` da quella directory.

**Se il percorso è corretto e il file esiste** ma Claude Code continua a segnalarlo come non trovato, il software di sicurezza degli endpoint come AppLocker, i criteri di restrizione software di Criteri di gruppo, o gli agenti EDR potrebbero interferire. Nelle versioni precedenti a v2.1.116, Claude Code generava un processo figlio (`cmd.exe`) per verificare il percorso, che questi criteri possono bloccare — un segnale comune è che `cmd.exe /c dir "C:\Program Files\Git\bin\bash.exe"` funziona quando lo esegui direttamente in PowerShell ma fallisce silenziosamente quando lanciato da `claude.exe`.

Claude Code v2.1.116 e versioni successive controllano il filesystem direttamente, quindi aggiorna prima. Se l'errore persiste su una versione attuale, chiedi al tuo team IT di aggiungere alla whitelist `claude.exe` e i processi che genera, inclusi `cmd.exe` e `bash.exe`, nella tua politica di protezione degli endpoint.

<h3 id="claude-code-does-not-support-32-bit-windows">
  Claude Code non supporta Windows a 32 bit
</h3>

Windows include due voci di PowerShell nel menu Start: `Windows PowerShell` e `Windows PowerShell (x86)`. La voce x86 viene eseguita come processo a 32 bit e attiva questo errore anche su una macchina a 64 bit. Per verificare quale caso sei, esegui questo nella stessa finestra che ha prodotto l'errore:

```powershell theme={null}
[Environment]::Is64BitOperatingSystem
```

Se questo stampa `True`, il tuo sistema operativo va bene. Chiudi la finestra, apri `Windows PowerShell` senza il suffisso x86, e esegui di nuovo il comando di installazione.

Se questo stampa `False`, sei su un'edizione Windows a 32 bit. Claude Code richiede un sistema operativo a 64 bit. Vedi i [requisiti di sistema](/docs/it/setup#system-requirements).

<h3 id="linux-musl-or-glibc-binary-mismatch">
  Mancata corrispondenza binaria musl o glibc di Linux
</h3>

Se vedi errori su librerie condivise mancanti come `libstdc++.so.6` o `libgcc_s.so.1` dopo l'installazione, il programma di installazione potrebbe aver scaricato la variante binaria sbagliata per il tuo sistema.

```text theme={null}
Error loading shared library libstdc++.so.6: No such file or directory
```

Questo può accadere su sistemi basati su glibc che hanno pacchetti di cross-compilazione musl installati, causando al programma di installazione di rilevare erroneamente il sistema come musl.

**Soluzioni:**

1. **Controlla quale libc usa il tuo sistema**:
   ```bash theme={null}
   ldd --version 2>&1 | head -1
   ```
   L'output che menziona `GNU libc` o `GLIBC` significa glibc. L'output che menziona `musl` significa musl.

2. **Se sei su glibc ma hai ottenuto il binario musl**, rimuovi l'installazione e reinstalla. Puoi anche scaricare manualmente il binario corretto usando il manifesto in `https://downloads.claude.ai/claude-code-releases/{VERSION}/manifest.json`. Apri un [problema GitHub](https://github.com/anthropics/claude-code/issues) con l'output di `ldd --version` e `ls /lib/libc.musl*`.

3. **Se sei effettivamente su musl**, come Alpine Linux, installa i pacchetti richiesti:
   ```bash theme={null}
   apk add libgcc libstdc++ ripgrep
   ```

<h3 id="illegal-instruction">
  `Illegal instruction`
</h3>

Se l'esecuzione di `claude` o del programma di installazione stampa `Illegal instruction`, il binario nativo utilizza istruzioni CPU che il tuo processore non supporta. Ci sono due cause distinte.

**Mancata corrispondenza dell'architettura.** Il programma di installazione ha scaricato il binario sbagliato, ad esempio x86 su un server ARM. Controlla con `uname -m` su macOS o Linux, o `$env:PROCESSOR_ARCHITECTURE` in PowerShell. Se il risultato non corrisponde al binario che hai ricevuto, [apri un problema GitHub](https://github.com/anthropics/claude-code/issues) con l'output.

**Set di istruzioni AVX mancante.** Se la tua architettura è corretta ma vedi ancora `Illegal instruction`, la tua CPU probabilmente manca di AVX o di un'altra istruzione che il binario richiede. Questo colpisce approssimativamente i processori Intel e AMD pre-2013, e le macchine virtuali dove l'hypervisor non passa AVX al guest.

Su un VPS o VM, esegui `grep -m1 -ow avx /proc/cpuinfo`; un risultato vuoto significa che AVX non è disponibile per il guest.

Non c'è una soluzione binaria nativa; traccia il [problema #50384](https://github.com/anthropics/claude-code/issues/50384) per lo stato, e includi il modello della tua CPU da `grep -m1 "model name" /proc/cpuinfo` su Linux o `sysctl -n machdep.cpu.brand_string` su macOS quando segnali.

I metodi di installazione alternativi scaricano lo stesso binario nativo e non risolveranno nessuna delle due cause.

<h3 id="dyld-cannot-load-on-macos">
  `dyld: cannot load` su macOS
</h3>

Se vedi `dyld: cannot load`, `dyld: Symbol not found`, o `Abort trap: 6` durante l'installazione, il binario è incompatibile con la tua versione di macOS o hardware.

```text theme={null}
dyld: cannot load 'claude-2.1.42-darwin-x64' (load command 0x80000034 is unknown)
Abort trap: 6
```

Un errore `Symbol not found` che fa riferimento a `libicucore` indica anche che la tua versione di macOS è più vecchia di quella supportata dal binario:

```text theme={null}
dyld: Symbol not found: _ubrk_clone
  Referenced from: claude-darwin-x64 (which was built for Mac OS X 13.0)
  Expected in: /usr/lib/libicucore.A.dylib
```

**Soluzioni:**

1. **Controlla la tua versione di macOS**: Claude Code richiede macOS 13.0 o successivo. Apri il menu Apple e seleziona About This Mac per controllare la tua versione.

2. **Aggiorna macOS** se sei su una versione precedente. Il binario utilizza comandi di caricamento e librerie di sistema che le versioni macOS precedenti non supportano. I metodi di installazione alternativi come Homebrew scaricano lo stesso binario e non risolveranno questo errore.

<h3 id="exec-format-error-on-wsl1">
  `Exec format error` su WSL1
</h3>

Se l'esecuzione di `claude` in WSL stampa `cannot execute binary file: Exec format error`, sei su WSL1 e stai colpendo una regressione binaria nativa nota tracciata nel [problema #38788](https://github.com/anthropics/claude-code/issues/38788). Le intestazioni del programma del binario sono cambiate in un modo che il caricatore di WSL1 non può gestire.

La correzione più pulita è convertire la tua distribuzione a WSL2 da PowerShell:

```powershell theme={null}
wsl --set-version <DistroName> 2
```

Se devi rimanere su WSL1, invoca il binario attraverso il linker dinamico. Aggiungi questa funzione a `~/.bashrc` all'interno di WSL, sostituendo il percorso se la tua directory home è diversa:

```bash theme={null}
claude() {
  /lib64/ld-linux-x86-64.so.2 "$(readlink -f "$HOME/.local/bin/claude")" "$@"
}
```

Quindi esegui `source ~/.bashrc` e riprova `claude`.

<h3 id="npm-install-errors-in-wsl">
  Errori di installazione npm in WSL
</h3>

Questi problemi si applicano se hai installato Claude Code con `npm install -g` all'interno di WSL. Se hai usato il [programma di installazione nativo](/docs/it/setup), salta questa sezione.

**Problemi di rilevamento del sistema operativo o della piattaforma.** Se npm segnala una mancata corrispondenza della piattaforma durante l'installazione, WSL probabilmente sta raccogliendo il `npm` di Windows. Esegui prima `npm config set os linux`, quindi installa con `npm install -g @anthropic-ai/claude-code --force`. Non usare `sudo`.

**`exec: node: not found` quando esegui `claude`.** Il tuo ambiente WSL probabilmente sta usando l'installazione di Node.js di Windows. Conferma con `which npm` e `which node`: i percorsi che iniziano con `/mnt/c/` sono binari Windows, mentre i percorsi Linux iniziano con `/usr/`. Per risolvere questo, installa Node tramite il gestore di pacchetti della tua distribuzione Linux o tramite [`nvm`](https://github.com/nvm-sh/nvm).

**Conflitti di versione nvm.** Se hai nvm installato sia in WSL che in Windows, il cambio delle versioni di Node in WSL potrebbe interrompersi perché WSL importa il PATH di Windows per impostazione predefinita e nvm di Windows ha priorità. La causa più comune è che nvm non è caricato nella tua shell. Aggiungi il caricatore nvm a `~/.bashrc` o `~/.zshrc`:

```bash theme={null}
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"
```

Oppure caricalo nella tua sessione corrente:

```bash theme={null}
source ~/.nvm/nvm.sh
```

Se nvm è caricato ma i percorsi di Windows hanno ancora priorità, anteponi esplicitamente il tuo percorso Node di Linux:

```bash theme={null}
export PATH="$HOME/.nvm/versions/node/$(node -v)/bin:$PATH"
```

<Warning>
  Evita di disabilitare l'importazione del PATH di Windows tramite `appendWindowsPath = false` poiché questo interrompe la capacità di chiamare eseguibili di Windows da WSL. Allo stesso modo, evita di disinstallare Node.js da Windows se lo usi per lo sviluppo di Windows.
</Warning>

<h3 id="permission-errors-during-installation">
  Errori di permessi durante l'installazione
</h3>

Se il programma di installazione nativo non riesce con errori di permessi, la directory di destinazione potrebbe non essere scrivibile. Vedi [Controlla i permessi della directory](#check-directory-permissions).

Se hai precedentemente installato con npm e stai riscontrando errori di permessi specifici di npm, passa al programma di installazione nativo:

```bash theme={null}
curl -fsSL https://claude.ai/install.sh | bash
```

<h3 id="native-binary-not-found-after-npm-install">
  Binario nativo non trovato dopo l'installazione npm
</h3>

Il pacchetto npm `@anthropic-ai/claude-code` estrae il binario nativo attraverso una dipendenza opzionale per piattaforma come `@anthropic-ai/claude-code-darwin-arm64`. Se l'esecuzione di `claude` dopo l'installazione stampa `Could not find native binary package "@anthropic-ai/claude-code-<platform>"`, controlla le seguenti cause:

* **Le dipendenze opzionali sono disabilitate.** Rimuovi `--omit=optional` dal tuo comando di installazione npm, `--no-optional` da pnpm, o `--ignore-optional` da yarn, e controlla che `.npmrc` non imposti `optional=false`. Quindi reinstalla. Il binario nativo viene consegnato solo come dipendenza opzionale, quindi non c'è fallback JavaScript se viene saltato.
* **Piattaforma non supportata.** I binari precompilati vengono pubblicati per `darwin-arm64`, `darwin-x64`, `linux-x64`, `linux-arm64`, `linux-x64-musl`, `linux-arm64-musl`, `win32-x64`, e `win32-arm64`. Claude Code non spedisce un binario per altre piattaforme; vedi i [requisiti di sistema](/docs/it/setup#system-requirements). Su FreeBSD, il programma di installazione segnala la piattaforma come non supportata. Prima della v2.1.205, ha trattato FreeBSD come Linux e ha scaricato un binario che non poteva essere eseguito.
* **Lo specchio npm aziendale manca dei pacchetti della piattaforma.** Assicurati che il tuo registro rispecchi tutti gli otto pacchetti `@anthropic-ai/claude-code-*` della piattaforma oltre al pacchetto meta.

L'installazione con `--ignore-scripts` non attiva questo errore. Il passaggio postinstall che collega il binario in posizione viene saltato, quindi Claude Code ricade su un wrapper che individua e genera il binario della piattaforma ad ogni avvio. Questo funziona ma si avvia più lentamente; reinstalla con gli script abilitati per l'esecuzione diretta.

<h2 id="login-and-authentication">
  Accesso e autenticazione
</h2>

Queste sezioni affrontano i fallimenti di accesso, gli errori OAuth e i problemi di token.

<h3 id="reset-your-login">
  Reimposta il tuo accesso
</h3>

Quando l'accesso non riesce e la causa non è ovvia, una re-autenticazione pulita risolve la maggior parte dei casi:

1. Esegui `/logout` per disconnetterti completamente
2. Chiudi Claude Code
3. Riavvia con `claude` e completa di nuovo il processo di autenticazione

Se il browser non si apre automaticamente durante l'accesso, premi `c` per copiare l'URL OAuth negli appunti, quindi incollalo in un browser manualmente. Questo funziona anche quando l'URL si avvolge su più righe in un terminale stretto o SSH e non può essere cliccato direttamente.

<h3 id="oauth-error-invalid-code">
  Errore OAuth: codice non valido
</h3>

Se vedi `OAuth error: Invalid code. Please make sure the full code was copied`, il codice di accesso è scaduto o è stato troncato durante il copia-incolla.

**Soluzioni:**

* Premi Invio per riprovare e completa l'accesso rapidamente dopo che il browser si apre
* Digita `c` per copiare l'URL completo se il browser non si apre automaticamente
* Se usi una sessione remota/SSH, il browser potrebbe aprirsi sulla macchina sbagliata. Copia l'URL visualizzato nel terminale e aprilo nel tuo browser locale invece.

<h3 id="403-forbidden-after-login">
  403 Forbidden dopo l'accesso
</h3>

Se vedi `API Error: 403 {"error":{"type":"forbidden","message":"Request not allowed"}}` dopo l'accesso:

* **Utenti Claude Pro/Max**: verifica che il tuo abbonamento sia attivo in [claude.ai/settings](https://claude.ai/settings)
* **Utenti della Console Anthropic**: conferma che il tuo account ha il ruolo "Claude Code" o "Developer". Gli amministratori assegnano questo nella Console Anthropic sotto Impostazioni → Membri.
* **Dietro un proxy**: i proxy aziendali possono interferire con le richieste API. Vedi [configurazione di rete](/docs/it/network-config) per la configurazione del proxy.

<h3 id="this-organization-has-been-disabled-with-an-active-subscription">
  Questa organizzazione è stata disabilitata con un abbonamento attivo
</h3>

Se vedi `API Error: 400 ... "This organization has been disabled"` nonostante tu abbia un abbonamento Claude attivo, una variabile di ambiente `ANTHROPIC_API_KEY` sta sostituendo il tuo abbonamento. Questo accade comunemente quando una vecchia chiave API da un precedente datore di lavoro o progetto è ancora impostata nel tuo profilo shell.

Quando `ANTHROPIC_API_KEY` è presente e l'hai approvato, Claude Code utilizza quella chiave invece delle credenziali OAuth del tuo abbonamento. In modalità non interattiva con il flag `-p`, la chiave viene sempre utilizzata quando presente. Vedi [precedenza di autenticazione](/docs/it/authentication#authentication-precedence) per l'ordine di risoluzione completo.

Per usare il tuo abbonamento invece, annulla l'impostazione della variabile di ambiente e rimuovila dal tuo profilo shell:

```bash theme={null}
unset ANTHROPIC_API_KEY
claude
```

Controlla `~/.zshrc`, `~/.bashrc`, o `~/.profile` per le righe `export ANTHROPIC_API_KEY=...` e rimuovile per rendere il cambiamento permanente. Su Windows, controlla il tuo profilo PowerShell in `$PROFILE` e le tue variabili di ambiente dell'utente per `ANTHROPIC_API_KEY`. Esegui `/status` all'interno di Claude Code per confermare quale metodo di autenticazione è attivo.

<h3 id="oauth-login-fails-in-wsl2-ssh-or-containers">
  L'accesso OAuth non riesce in WSL2, SSH o container
</h3>

Quando Claude Code viene eseguito in WSL2, su una macchina remota tramite SSH, o all'interno di un container, il browser di solito si apre su un host diverso e il suo reindirizzamento non può raggiungere il server di callback locale di Claude Code. Dopo che accedi, il browser mostra un codice di accesso invece di reindirizzare automaticamente. Incolla quel codice nel terminale al prompt `Paste code here if prompted` per completare l'accesso.

Se il browser non si apre affatto da WSL2, imposta la variabile di ambiente `BROWSER` al percorso del tuo browser Windows:

```bash theme={null}
export BROWSER="/mnt/c/Program Files/Google/Chrome/Application/chrome.exe"
claude
```

In alternativa, premi `c` al prompt di accesso interattivo per copiare l'URL OAuth, o copia l'URL che `claude auth login` stampa, e aprilo in un browser sulla tua macchina locale.

Se incollare il codice nel prompt interattivo non fa nulla, il binding di incolla del tuo terminale probabilmente non sta raggiungendo il campo di input. Prova il collegamento di incolla alternativo del tuo terminale, spesso clic destro o Maiusc+Inserisci in Windows Terminal, o usa `claude auth login` invece, che legge il codice incollato dall'input standard:

```bash theme={null}
claude auth login
```

Questo fallback si applica anche su Windows nativo o su qualsiasi terminale in cui l'incollamento nel prompt interattivo non riesce.

<h3 id="not-logged-in-or-token-expired">
  Non connesso o token scaduto
</h3>

Se Claude Code ti chiede di accedere di nuovo dopo una sessione, il tuo token OAuth potrebbe essere scaduto.

Esegui `/login` per re-autenticarti. Se questo accade frequentemente, controlla che l'orologio di sistema sia accurato, poiché la convalida del token dipende da timestamp corretti.

Su macOS, l'accesso può anche non riuscire quando il Keychain è bloccato o la sua password non è sincronizzata con la password del tuo account, il che impedisce a Claude Code di salvare le credenziali. Esegui `claude doctor` per controllare l'accesso al Keychain. Per sbloccare il Keychain manualmente, esegui `security unlock-keychain ~/Library/Keychains/login.keychain-db`. Se lo sblocco non aiuta, apri Accesso Portachiavi, seleziona il keychain `login`, e scegli Modifica > Cambia password per Portachiavi "login" per risincronizzarlo con la password del tuo account.

<h3 id="bedrock-agent-platform-or-foundry-credentials-not-loading">
  Credenziali Bedrock, Agent Platform o Foundry non caricate
</h3>

Se hai configurato Claude Code per usare un provider cloud e vedi `Could not load credentials from any providers` su Amazon Bedrock, `Could not load the default credentials` su Google Cloud's Agent Platform, o `ChainedTokenCredential authentication failed` su Microsoft Foundry, la tua CLI del provider cloud probabilmente non è autenticata nella shell corrente.

Per Amazon Bedrock, conferma che le tue credenziali AWS sono valide:

```bash theme={null}
aws sts get-caller-identity
```

Per Google Cloud's Agent Platform, conferma che `ANTHROPIC_VERTEX_PROJECT_ID` e `CLOUD_ML_REGION` sono impostati nella tua shell, quindi imposta le credenziali predefinite dell'applicazione:

```bash theme={null}
gcloud auth application-default login
```

Per Microsoft Foundry, conferma che `ANTHROPIC_FOUNDRY_API_KEY` è impostato, o accedi con l'interfaccia della riga di comando di Azure in modo che la catena di credenziali predefinita possa trovare il tuo account:

```bash theme={null}
az login
```

Se le credenziali funzionano nel tuo terminale ma non nell'estensione VS Code o JetBrains, il processo IDE probabilmente non ha ereditato il tuo ambiente shell. Imposta le variabili di ambiente del provider nelle impostazioni dell'IDE stesso, o avvia l'IDE da un terminale dove sono già esportate.

Vedi [Amazon Bedrock](/docs/it/amazon-bedrock), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai), o [Microsoft Foundry](/docs/it/microsoft-foundry) per la configurazione completa del provider.

<h2 id="still-stuck">
  Ancora bloccato
</h2>

Se nessuno dei precedenti risolve il tuo problema:

1. Controlla il [repository GitHub](https://github.com/anthropics/claude-code/issues) per i problemi noti, o apri uno nuovo con il tuo sistema operativo, il comando di installazione che hai eseguito, e l'output di errore completo
2. Se `claude --version` funziona ma qualcos'altro non va, esegui `claude doctor` per un rapporto diagnostico automatizzato
3. Se riesci ad avviare una sessione, usa `/feedback` all'interno di Claude Code per segnalare il problema
