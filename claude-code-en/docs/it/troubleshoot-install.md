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
| `Raw mode is not supported` durante l'installazione                                                                    | [Riesegui il programma di installazione](#raw-mode-is-not-supported-during-install)                                                          |
| `TLS connect error` o `SSL/TLS secure channel`                                                                         | [Aggiorna i certificati CA](#tls-or-ssl-connection-errors)                                                                                   |
| `Failed to fetch version` o impossibile raggiungere il server di download                                              | [Controlla le impostazioni di rete e proxy](#check-network-connectivity)                                                                     |
| `irm is not recognized` o `The token '&&' is not a valid statement separator`                                          | [Usa il comando giusto per la tua shell](#wrong-install-command-on-windows)                                                                  |
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
| `dyld: Symbol not found`, `dyld: cannot load`, o `Abort trap` su macOS                                                 | [Incompatibilità binaria](#dyld-cannot-load-on-macos)                                                                                        |
| `claude update` si blocca dopo `Checking for updates`, o `claude doctor` si blocca senza output                        | [Sposta la directory nel percorso di configurazione della shell](#claude-update-or-claude-doctor-hangs)                                      |
| `Invoke-Expression` o `iex` errori di analisi che citano tag HTML o CSS, o `ParserError` con `ParseException`          | [Lo script di installazione restituisce HTML](#install-script-returns-html-instead-of-a-shell-script)                                        |
| `running scripts is disabled on this system` o `PSSecurityException`                                                   | [Consenti l'esecuzione dei shim npm](#running-scripts-is-disabled-on-this-system)                                                            |
| `Error: claude native binary not installed`                                                                            | [Completa l'installazione npm](#native-binary-not-found-after-npm-install)                                                                   |
| `npm error code ENOTEMPTY` durante l'aggiornamento o la reinstallazione                                                | [Rimuovi la directory del pacchetto rimasta](#npm-enotempty-during-update-or-reinstall)                                                      |
| Su Windows, il comando di installazione stampa il testo dello script e nulla viene installato                          | [Esegui il comando di installazione completo](#wrong-install-command-on-windows)                                                             |
| `App unavailable in region`                                                                                            | Claude Code non è disponibile nel tuo paese. Vedi [paesi supportati](https://www.anthropic.com/supported-countries).                         |
| `unable to get local issuer certificate`                                                                               | [Configura i certificati CA aziendali](#tls-or-ssl-connection-errors)                                                                        |
| `OAuth error` o `403 Forbidden`                                                                                        | [Correggi l'autenticazione](#login-and-authentication)                                                                                       |
| `Unable to connect to Anthropic services` durante la configurazione                                                    | Vedi [Unable to connect to Anthropic services](/docs/it/errors#unable-to-connect-to-anthropic-services) nel riferimento degli errori              |
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

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    curl -sI https://downloads.claude.ai/claude-code-releases/latest
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    curl.exe -sI https://downloads.claude.ai/claude-code-releases/latest
    ```

    PowerShell crea un alias di `curl` a `Invoke-WebRequest`, che rifiuta i flag `-sI`, quindi chiama `curl.exe` esplicitamente.
  </Tab>
</Tabs>

Hai raggiunto il server se la prima riga mostra uno stato `200`. Vedrai `HTTP/2 200` su macOS e Linux, e `HTTP/1.1 200 OK` da `curl.exe` incluso con Windows. Altri risultati indicano la causa:

* `403`: solitamente un proxy o un filtro di rete che blocca l'host, o Claude Code non è [disponibile nella tua regione](https://www.anthropic.com/supported-countries)
* `5xx`: solitamente un problema temporaneo del servizio; attendi alcuni minuti e riprova

Se non vedi output, `Could not resolve host`, o un timeout di connessione, la tua rete sta bloccando la connessione. Le cause comuni sono:

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

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    rm -rf ~/.claude/local
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    Remove-Item -Recurse -Force "$env:USERPROFILE\.claude\local"
    ```
  </Tab>
</Tabs>

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

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    ls -la "$(command -v claude)"
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    Get-Command claude | Select-Object Source
    ```
  </Tab>
</Tabs>

Su Linux, controlla le librerie condivise mancanti. Se `ldd` mostra librerie mancanti, potrebbe essere necessario installare pacchetti di sistema. Su Alpine Linux e altre distribuzioni basate su musl, vedi [Configurazione di Alpine Linux](/docs/it/setup#alpine-linux-and-musl-based-distributions).

```bash theme={null}
ldd "$(command -v claude)" | grep "not found"
```

Conferma che il binario può essere eseguito:

```bash theme={null}
claude --version
```

<h2 id="common-installation-issues">
  Problemi comuni di installazione
</h2>

Questi sono i problemi di installazione più frequentemente riscontrati e le loro soluzioni.

<h3 id="install-script-returns-html-instead-of-a-shell-script">
  Lo script di installazione restituisce HTML invece di uno script shell
</h3>

Quando si esegue il comando di installazione, è possibile che si veda uno di questi errori:

```text theme={null}
bash: line 1: syntax error near unexpected token `<'
bash: line 1: `<!DOCTYPE html>'
```

Su PowerShell, lo stesso problema appare come errori di analisi che puntano alla pagina restituita, con `iex` che tenta di eseguire HTML e CSS come PowerShell:

```text theme={null}
iex : At line:1 char:2310
+ ... igin="anonymous"/><script type="text/javascript">!function(o,c){var n ...
Missing argument in parameter list.
...
```

La formulazione varia a seconda della versione di PowerShell e della lingua del sistema: è possibile che si veda `Missing expression after unary operator '--'` o un `ParserError` con `ParseException`. I tag HTML o CSS nel testo tra virgolette identificano questo errore. Se si scarica con `-OutFile install.ps1`, il file salvato è la stessa pagina web, quindi nemmeno questo aiuta.

A seconda di come è stata instradata la richiesta, è possibile che si veda invece un 403 senza corpo HTML:

```text theme={null}
curl: (22) The requested URL returned error: 403
```

Tutti questi significano che l'URL di installazione ha restituito una pagina HTML o uno stato di errore invece dello script di installazione. Se la pagina HTML dice "App unavailable in region", Claude Code non è disponibile nel vostro paese. Vedere [paesi supportati](https://www.anthropic.com/supported-countries).

Un 403 nudo senza corpo spesso ha la stessa causa, ma può anche provenire da un proxy aziendale o da un firewall che blocca il download. Se siete in un paese supportato e vedete ancora il 403, consultate [Verificare la connettività di rete](#check-network-connectivity) prima di provare i programmi di installazione alternativi di seguito, poiché raggiungono gli stessi host.

Altrimenti, questo può accadere a causa di problemi di rete, routing regionale o un'interruzione temporanea del servizio.

**Soluzioni:**

1. **Utilizzare un metodo di installazione alternativo**:

   Su macOS, installare tramite Homebrew:

   ```bash theme={null}
   brew install --cask claude-code
   ```

   Su Windows, installare tramite WinGet:

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

   Quindi eseguire `claude --version` per confermare: il comando stampa un numero di versione come `2.1.211 (Claude Code)`. Se la shell segnala che `claude` non è trovato, aprire una nuova finestra del terminale e riprovare: la sessione da cui avete installato mantiene il vecchio `PATH`.

2. **Riprovare dopo alcuni minuti**: il problema è spesso temporaneo. Attendere e riprovare il comando originale.

<h3 id="command-not-found-claude-after-installation">
  `command not found: claude` dopo l'installazione
</h3>

L'installazione è terminata ma `claude` non funziona. L'errore esatto varia a seconda della piattaforma:

| Piattaforma | Messaggio di errore                                                    |
| :---------- | :--------------------------------------------------------------------- |
| macOS       | `zsh: command not found: claude`                                       |
| Linux       | `bash: claude: command not found`                                      |
| Windows CMD | `'claude' is not recognized as an internal or external command`        |
| PowerShell  | `claude : The term 'claude' is not recognized as the name of a cmdlet` |

Questo significa che la directory di installazione non è nel percorso di ricerca della shell. Vedere [Verificare il vostro PATH](#verify-your-path) per la correzione su ogni piattaforma.

<h3 id="curl-56-failure-writing-output-to-destination">
  `curl: (56) Failure writing output to destination`
</h3>

Il comando `curl ... | bash` scarica lo script e lo invia a Bash per l'esecuzione. Questo errore, e il correlato `curl: (23) Failure writing output to destination`, significa che Bash non ha ricevuto lo script completo. Il codice di uscita 56 indica che il download stesso è stato interrotto, e il codice di uscita 23 indica che curl non poteva scrivere ciò che ha ricevuto alla pipe, solitamente perché Bash è uscito prematuramente.

Verificare che sia possibile raggiungere `downloads.claude.ai` con il controllo in [Verificare la connettività di rete](#check-network-connectivity). Se avete raggiunto il server, il fallimento originale era probabilmente intermittente; riprovare il comando di installazione. Potete anche [provare un metodo di installazione alternativo](/docs/it/setup#install-claude-code).

<h3 id="homebrew-cask-unavailable-or-outdated">
  Cask Homebrew non disponibile o obsoleto
</h3>

Homebrew segnala `Error: Cask 'claude-code' is unavailable: No Cask with this name exists` quando la copia locale dell'indice cask di Homebrew è precedente alla pubblicazione del cask. Aggiornare l'indice e riprovare:

```bash theme={null}
brew update
brew install --cask claude-code
```

Se Homebrew installa una versione di Claude Code più vecchia di quella prevista, lo stesso indice obsoleto è solitamente la causa. Il cask `claude-code` traccia il canale stabile ed è tipicamente circa una settimana dietro l'ultima versione; per la versione più recente eseguire `brew install --cask claude-code@latest`. Vedere [Configurare il canale di rilascio](/docs/it/setup#configure-release-channel) per la differenza tra i due cask.

<h3 id="tls-or-ssl-connection-errors">
  Errori di connessione TLS o SSL
</h3>

Errori come `curl: (35) TLS connect error`, `schannel: next InitializeSecurityContext failed`, o il `Could not establish trust relationship for the SSL/TLS secure channel` di PowerShell indicano fallimenti dell'handshake TLS.

**Soluzioni:**

1. **Aggiornare i certificati CA del sistema**:

   Su Ubuntu/Debian:

   ```bash theme={null}
   sudo apt-get update && sudo apt-get install ca-certificates
   ```

   Su macOS, il curl di sistema utilizza l'archivio di fiducia Keychain; l'aggiornamento di macOS stesso aggiorna i certificati root.

2. **Su Windows, abilitare TLS 1.2** in PowerShell prima di eseguire il programma di installazione:
   ```powershell theme={null}
   [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
   irm https://claude.ai/install.ps1 | iex
   ```

3. **Verificare l'interferenza di proxy o firewall**: i proxy aziendali che eseguono l'ispezione TLS possono causare questi errori, inclusi `unable to get local issuer certificate` e `SELF_SIGNED_CERT_IN_CHAIN`. Per il passaggio di installazione, fare in modo che il download di installazione si fidi del CA del proxy aziendale:

   <Tabs>
     <Tab title="macOS/Linux">
       ```bash theme={null}
       curl --cacert /path/to/corporate-ca.pem -fsSL https://claude.ai/install.sh | bash
       ```
     </Tab>

     <Tab title="Windows PowerShell">
       Il programma di installazione PowerShell scarica tramite .NET, che convalida TLS rispetto all'archivio certificati di Windows. Chiedere al team IT di aggiungere il certificato CA del proxy all'archivio di Windows se non è già presente, quindi eseguire il programma di installazione:

       ```powershell theme={null}
       irm https://claude.ai/install.ps1 | iex
       ```
     </Tab>
   </Tabs>

   Per Claude Code stesso una volta installato, impostare `NODE_EXTRA_CA_CERTS` in modo che le richieste API si fidino dello stesso bundle:

   <Tabs>
     <Tab title="macOS/Linux">
       ```bash theme={null}
       export NODE_EXTRA_CA_CERTS=/path/to/corporate-ca.pem
       ```
     </Tab>

     <Tab title="Windows PowerShell">
       ```powershell theme={null}
       $env:NODE_EXTRA_CA_CERTS = 'C:\path\to\corporate-ca.pem'
       ```
     </Tab>
   </Tabs>

   Chiedere al team IT il file del certificato se non lo avete. Potete anche provare su una connessione diretta per confermare che il proxy è la causa.

4. **Su Windows, aggirare i controlli di revoca bloccati**. Gli errori `CRYPT_E_NO_REVOCATION_CHECK (0x80092012)` e `CRYPT_E_REVOCATION_OFFLINE (0x80092013)` significano che curl ha raggiunto il server ma la vostra rete blocca la ricerca di revoca del certificato, il che è comune dietro i firewall aziendali. Se il comando che fallisce è il `curl` che scarica `install.cmd`, rieseguirlo da un Command Prompt con `--ssl-revoke-best-effort` aggiunto:
   ```batch theme={null}
   curl --ssl-revoke-best-effort -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
   ```
   Quando i download dello script stesso colpiscono gli stessi errori, li ritenta con il controllo di revoca best-effort automaticamente, quindi il flag è necessario solo sul comando che eseguite voi stessi. Il controllo best-effort tollera un server di revoca irraggiungibile ma rifiuta comunque un certificato che è noto essere revocato, corrispondendo a come i browser gestiscono la revoca. Potete anche evitare completamente il controllo di revoca di curl eseguendo il programma di installazione PowerShell da PowerShell, che scarica tramite .NET e non fallisce quando il server di revoca è irraggiungibile:
   ```powershell theme={null}
   irm https://claude.ai/install.ps1 | iex
   ```
   Potete anche installare con `winget install Anthropic.ClaudeCode`, che evita curl completamente.

<h3 id="failed-to-fetch-version-from-downloads-claude-ai">
  `Failed to fetch version from downloads.claude.ai`
</h3>

Il programma di installazione non poteva raggiungere il server di download. Questo tipicamente significa che `downloads.claude.ai` è bloccato sulla vostra rete. Vedere [Verificare la connettività di rete](#check-network-connectivity).

<h3 id="wrong-install-command-on-windows">
  Comando di installazione errato su Windows
</h3>

Se vedete `'irm' is not recognized`, `The token '&&' is not a valid statement separator`, `A parameter cannot be found that matches parameter name 'fsSL'`, o `'bash' is not recognized as the name of a cmdlet`, avete copiato il comando di installazione per una shell o un sistema operativo diverso. Se il comando stampa il testo dello script invece di installare qualcosa, avete eseguito solo una parte di esso.

* **`irm` non riconosciuto**: siete in CMD, non in PowerShell. Avete due opzioni:

  Aprire PowerShell cercando "PowerShell" nel menu Start, quindi eseguire il comando di installazione originale:

  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

  Oppure rimanere in CMD e utilizzare il programma di installazione CMD:

  ```batch theme={null}
  curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
  ```

* **`&&` non è un separatore di istruzioni valido**: siete in PowerShell ma avete eseguito il comando del programma di installazione CMD. Utilizzare il programma di installazione PowerShell:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

* **`A parameter cannot be found that matches parameter name 'fsSL'`**: avete eseguito il programma di installazione `curl -fsSL ... | bash` di macOS/Linux in Windows PowerShell, dove `curl` è un alias per `Invoke-WebRequest` e rifiuta i flag `-fsSL`. Utilizzare il programma di installazione PowerShell:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

* **`bash` non riconosciuto**: avete eseguito il programma di installazione di macOS/Linux su Windows. Utilizzare il programma di installazione PowerShell:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

* **Il comando stampa il testo dello script invece di installare**: avete eseguito la metà del download del comando senza la parte che lo esegue. `irm https://claude.ai/install.ps1` da solo stampa lo script scaricato al terminale. Inviarlo a `iex` per eseguirlo:

  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

  In CMD, `curl -fsSL https://claude.ai/install.cmd` senza `-o` stampa lo script batch invece di salvarlo. Eseguire il comando completo:

  ```batch theme={null}
  curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
  ```

Qualunque programma di installazione utilizziate, confermare che ha funzionato: aprire un nuovo terminale ed eseguire `claude --version`, che stampa un numero di versione come `2.1.211 (Claude Code)`.

<h3 id="running-scripts-is-disabled-on-this-system">
  `running scripts is disabled on this system`
</h3>

L'installazione o l'esecuzione di Claude Code tramite npm su Windows può fallire con un `SecurityError`:

```text theme={null}
npm : File C:\Program Files\nodejs\npm.ps1 cannot be loaded because running scripts is disabled on this system. For more information, see about_Execution_Policies at https:/go.microsoft.com/fwlink/?LinkID=135170.
...
    + CategoryInfo          : SecurityError: (:) [], PSSecurityException
```

Lo stesso errore nomina `claude.ps1` quando eseguite `claude` dopo un'installazione npm. La politica di esecuzione di PowerShell sta bloccando gli script launcher `.ps1` che npm crea per i suoi comandi. La politica si applica ai file di script, quindi non influisce sul programma di installazione PowerShell `irm https://claude.ai/install.ps1 | iex`, che esegue il testo scaricato direttamente.

**Soluzioni:**

1. **Consentire gli script creati localmente per il vostro utente**, quindi riprovare:
   ```powershell theme={null}
   Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
   ```
2. **Chiamare il launcher `.cmd`**: `npm.cmd` e `claude.cmd` fanno lo stesso lavoro, e la politica non li copre.
3. **Utilizzare il [programma di installazione PowerShell](/docs/it/setup#install-claude-code)** invece di npm. Installa un binario piuttosto che uno script `.ps1`.

<h3 id="the-process-cannot-access-the-file-during-windows-install">
  `The process cannot access the file` durante l'installazione su Windows
</h3>

Se il programma di installazione PowerShell fallisce con `Failed to download binary: The process cannot access the file ... because it is being used by another process`, il programma di installazione non poteva scrivere in `%USERPROFILE%\.claude\downloads`. Questo solitamente significa che un tentativo di installazione precedente è ancora in esecuzione, o il software antivirus sta scansionando un binario parzialmente scaricato in quella cartella.

Chiudere qualsiasi altra finestra PowerShell che esegue il programma di installazione e attendere che le scansioni antivirus rilascino il file. Quindi eliminare la cartella dei download ed eseguire di nuovo il programma di installazione:

```powershell theme={null}
Remove-Item -Recurse -Force "$env:USERPROFILE\.claude\downloads"
irm https://claude.ai/install.ps1 | iex
```

<h3 id="install-killed-on-low-memory-linux-servers">
  Installazione interrotta su server Linux con poca memoria
</h3>

Un messaggio `Killed` durante l'installazione solitamente significa che il killer out-of-memory (OOM) di Linux ha terminato il passaggio `claude install` perché il sistema ha esaurito la memoria libera. Questo è comune su piccoli VPS e istanze cloud. Lo script di installazione segnala la causa e esce con il codice 137. In questo esempio, il numero di riga e l'ID del processo variano a seconda della versione e dell'esecuzione:

```text theme={null}
Setting up Claude Code...
bash: line 183: 34803 Killed    "$binary_path" install ${TARGET:+"$TARGET"}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

L'installazione ha bisogno di circa 512 MB di memoria libera, e l'esecuzione di Claude Code ne ha bisogno di più. Vedere i [requisiti di sistema](/docs/it/setup#system-requirements).

**Soluzioni:**

1. **Aggiungere spazio di swap** se il vostro server ha RAM limitata. Lo swap utilizza lo spazio su disco come memoria di overflow, permettendo all'installazione di completarsi anche con poca RAM fisica.

   Creare un file di swap di 2 GB e abilitarlo:

   ```bash theme={null}
   sudo fallocate -l 2G /swapfile
   sudo chmod 600 /swapfile
   sudo mkswap /swapfile
   sudo swapon /swapfile
   ```

   Quindi riprovare l'installazione:

   ```bash theme={null}
   curl -fsSL https://claude.ai/install.sh | bash
   ```

2. **Chiudere altri processi** per liberare memoria prima di installare.

3. **Utilizzare un'istanza più grande** se possibile. Claude Code richiede almeno 4 GB di RAM.

<h3 id="install-hangs-in-docker">
  L'installazione si blocca in Docker
</h3>

Quando si installa Claude Code in un contenitore Docker, l'installazione come root in `/` può causare blocchi.

**Soluzioni:**

1. **Impostare una directory di lavoro** prima di eseguire il programma di installazione. Quando eseguito da `/`, il programma di installazione scansiona l'intero filesystem, il che causa un utilizzo eccessivo della memoria. L'impostazione di `WORKDIR` limita la scansione a una piccola directory:
   ```dockerfile theme={null}
   WORKDIR /tmp
   RUN curl -fsSL https://claude.ai/install.sh | bash
   ```

2. **Dare a Docker più memoria** se si utilizza Docker Desktop. I contenitori di build condividono la memoria allocata alla macchina virtuale Docker Desktop, quindi aprire **Settings > Resources** in Docker Desktop, aumentare il limite di memoria e rieseguire la build.

<h3 id="raw-mode-is-not-supported-during-install">
  `Raw mode is not supported` durante l'installazione
</h3>

Quando le [impostazioni gestite dal server](/docs/it/server-managed-settings) della vostra organizzazione includono modifiche che necessitano di [approvazione di sicurezza](/docs/it/server-managed-settings#security-approval-dialogs), le versioni di Claude Code precedenti a 2.1.246 tentano di mostrare la finestra di dialogo di approvazione durante `claude install`. La finestra di dialogo ha bisogno di un terminale su stdin. Quando il programma di installazione esegue `claude install` da una pipe, come fa `curl -fsSL https://claude.ai/install.sh | bash`, stdin è la pipe piuttosto che un terminale, quindi l'installazione fallisce con un errore contenente `Raw mode is not supported`.

Claude Code v2.1.246 e successivi non mostrano la finestra di dialogo durante `claude install` o `claude update`. Il comando viene eseguito con le impostazioni che avete approvato l'ultima volta, e Claude Code mostra la finestra di dialogo nella vostra prossima sessione interattiva. Se la configurazione di avvio della vostra organizzazione [attende il recupero delle impostazioni](/docs/it/server-managed-settings#enforce-fail-closed-startup), come quando imposta `forceRemoteSettingsRefresh`, la finestra di dialogo appare comunque durante questi comandi, e un'esecuzione di installazione da una pipe fallisce comunque.

In ogni altra configurazione, rieseguire il programma di installazione supera questo errore, perché lo script esegue il comando `install` della versione più recente anche quando chiedete di installare una versione precedente. Rieseguire il comando per la vostra piattaforma:

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    irm https://claude.ai/install.ps1 | iex
    ```
  </Tab>
</Tabs>

`claude --version` stampa la versione che la riesecuzione ha installato.

<h3 id="claude-update-or-claude-doctor-hangs">
  `claude update` o `claude doctor` si blocca
</h3>

`claude update` e `claude doctor` scansionano i file di configurazione della shell per un alias `claude` obsoleto: `~/.zshrc`, `~/.bashrc`, e `~/.config/fish/config.fish`, più su macOS il primo di `~/.bash_profile`, `~/.bash_login`, o `~/.profile` che esiste. Se impostate `ZDOTDIR`, il file Zsh è `$ZDOTDIR/.zshrc`. Quando uno di questi percorsi è una directory, Claude Code lo salta e entrambi i comandi si completano normalmente. Prima di v2.1.214, una directory in uno di questi percorsi faceva bloccare entrambi i comandi e lasciava la sezione System diagnostics di `/status` vuota. `claude doctor` si bloccava senza output; `claude update` si bloccava subito dopo aver stampato `Checking for updates`.

Se colpite il blocco su una versione precedente, trovate la directory. Nell'output di questo comando, una riga che inizia con `d` contrassegna quel percorso come una directory. Una riga `No such file or directory` significa che nulla esiste in quel percorso e non è la causa:

```bash theme={null}
ls -ld ~/.zshrc ~/.bashrc ~/.bash_profile ~/.bash_login ~/.profile ~/.config/fish/config.fish
```

Spostare la directory da parte, o aggiornare a v2.1.214 o successivo. Poiché `claude update` si blocca sulle versioni interessate, aggiornare rieseguendo lo [script di installazione](/docs/it/setup#install-claude-code).

<h3 id="claude-desktop-overrides-the-claude-command-on-windows">
  Claude Desktop sostituisce il comando `claude` su Windows
</h3>

Se avete installato una versione precedente di Claude Desktop, potrebbe registrare un `Claude.exe` nella directory `WindowsApps` che ha priorità su PATH rispetto a Claude Code CLI. L'esecuzione di `claude` apre l'app Desktop invece della CLI.

Aggiornare Claude Desktop all'ultima versione per risolvere questo problema.

<h3 id="claude-code-on-windows-requires-either-git-for-windows-for-bash-or-powershell">
  Claude Code su Windows richiede Git for Windows (per bash) o PowerShell
</h3>

Git for Windows è opzionale. Claude Code utilizza lo [strumento PowerShell](/docs/it/tools-reference#powershell-tool) quando Git Bash è assente, quindi questo errore significa che nessuna shell è stata trovata.

**Se PowerShell manca dal vostro PATH**, la sua posizione predefinita è `C:\Windows\System32\WindowsPowerShell\v1.0\`. Aggiungere quella directory al vostro `PATH`, o installare [PowerShell 7](https://aka.ms/powershell), che fornisce `pwsh`.

**Per installare Git for Windows**, scaricarlo da [git-scm.com/downloads/win](https://git-scm.com/downloads/win). Durante l'installazione, selezionare "Add to PATH". Riavviare il terminale dopo l'installazione. L'installazione abilita lo strumento Bash, utile quando si lavora con script e strumenti basati su Bash.

**Se Git è già installato** ma Claude Code non riesce a trovarlo, confrontare la sua posizione rispetto ai posti in cui Claude Code cerca. Quando `CLAUDE_CODE_GIT_BASH_PATH` non è impostato, Claude Code cerca `bash.exe` in questo ordine:

1. Le posizioni di installazione predefinite `C:\Program Files\Git` e `C:\Program Files (x86)\Git`.
2. Il `git` sul vostro `PATH`, utilizzando il `bin\bash.exe` da quella installazione di Git.

Nel passaggio 2, Claude Code salta un `git` che si trova nella cartella da cui avete lanciato Claude Code, o sotto di essa in un percorso che contiene `node_modules` o una cartella di ambiente virtuale come `.venv` o `env`, ad esempio `C:\dev\env\myproject\Git` quando avete lanciato da `C:\dev\env\myproject`. Questo impedisce a Claude Code di eseguire un eseguibile che un progetto ha messo lì. Se il vostro Git è in una posizione come quella, puntate `CLAUDE_CODE_GIT_BASH_PATH` ad esso.

**Per puntare Claude Code a un'installazione Git specifica**, trovarla eseguendo `where.exe git` in PowerShell, quindi impostare il percorso `bin\bash.exe` da quella installazione come `CLAUDE_CODE_GIT_BASH_PATH` nel vostro [file settings.json](/docs/it/settings):

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_GIT_BASH_PATH": "C:\\Program Files\\Git\\bin\\bash.exe"
  }
}
```

**Se `CLAUDE_CODE_GIT_BASH_PATH` è impostato al percorso corretto e il file esiste** ma Claude Code ancora non lo utilizza, controllare il nome del file per primo. Claude Code accetta solo un file denominato `bash.exe`, `sh.exe`, `bash`, o `sh`; con qualsiasi altro nome, come il launcher `git-bash.exe` di Git for Windows, ignora la variabile e auto-rileva Git Bash come se fosse non impostato, registrando un avviso visibile con `--debug`. Un percorso che non esiste ottiene lo stesso fallback e avviso. Prima di v2.1.219, Claude Code utilizzava qualsiasi file esistente come shell senza controllare il suo nome, e usciva all'avvio con `Claude Code was unable to find CLAUDE_CODE_GIT_BASH_PATH path` quando il percorso non esisteva.

Se il nome del file è corretto, il software di sicurezza degli endpoint come AppLocker, le politiche di restrizione software di Group Policy, o gli agenti EDR potrebbero interferire. Chiedere al vostro team IT di inserire nella whitelist `claude.exe` e i processi che genera, inclusi `cmd.exe` e `bash.exe`, nella vostra politica di protezione degli endpoint.

<h3 id="claude-code-does-not-support-32-bit-windows">
  Claude Code non supporta Windows a 32 bit
</h3>

Windows include due voci di PowerShell nel menu Start: `Windows PowerShell` e `Windows PowerShell (x86)`. La voce x86 viene eseguita come processo a 32 bit e attiva questo errore anche su una macchina a 64 bit. Per verificare quale caso siete, eseguire questo nella stessa finestra che ha prodotto l'errore:

```powershell theme={null}
[Environment]::Is64BitOperatingSystem
```

Se stampa `True`, il vostro sistema operativo va bene. Chiudere la finestra, aprire `Windows PowerShell` senza il suffisso x86, ed eseguire di nuovo il comando di installazione.

Se stampa `False`, siete su un'edizione Windows a 32 bit. Claude Code richiede un sistema operativo a 64 bit. Vedere i [requisiti di sistema](/docs/it/setup#system-requirements).

<h3 id="linux-musl-or-glibc-binary-mismatch">
  Mancata corrispondenza binaria musl o glibc su Linux
</h3>

Se vedete errori su librerie condivise mancanti come `libstdc++.so.6` o `libgcc_s.so.1` dopo l'installazione, il programma di installazione potrebbe aver scaricato la variante binaria sbagliata per il vostro sistema.

```text theme={null}
Error loading shared library libstdc++.so.6: No such file or directory
```

Questo può accadere su sistemi basati su glibc che hanno pacchetti di cross-compilazione musl installati, causando al programma di installazione di rilevare erroneamente il sistema come musl.

**Soluzioni:**

1. **Verificare quale libc utilizza il vostro sistema**:
   ```bash theme={null}
   ldd --version 2>&1 | head -1
   ```
   L'output che menziona `GNU libc` o `GLIBC` significa glibc. L'output che menziona `musl` significa musl.

2. **Se siete su glibc ma avete ottenuto il binario musl**, rimuovere l'installazione e reinstallare. Potete anche scaricare manualmente il binario corretto utilizzando il manifesto in `https://downloads.claude.ai/claude-code-releases/{VERSION}/manifest.json`. Presentare un [problema GitHub](https://github.com/anthropics/claude-code/issues) con l'output di `ldd --version` e `ls /lib/libc.musl*`.

3. **Se siete effettivamente su musl**, come Alpine Linux, installare i pacchetti richiesti:
   ```bash theme={null}
   apk add libgcc libstdc++ ripgrep
   ```
   Su Alpine, `ripgrep` è nel repository community. Se `apk` segnala che il pacchetto manca, vedere [Configurazione Alpine Linux](/docs/it/setup#alpine-linux-and-musl-based-distributions).

<h3 id="illegal-instruction">
  `Illegal instruction`
</h3>

Se l'esecuzione di `claude` o del programma di installazione stampa `Illegal instruction`, il binario nativo utilizza istruzioni CPU che il vostro processore non supporta. Ci sono due cause distinte.

**Mancata corrispondenza dell'architettura.** Il programma di installazione ha scaricato il binario sbagliato, ad esempio x86 su un server ARM. Controllare con `uname -m` su macOS o Linux, o `$env:PROCESSOR_ARCHITECTURE` in PowerShell. Se il risultato non corrisponde al binario che avete ricevuto, [presentare un problema GitHub](https://github.com/anthropics/claude-code/issues) con l'output.

**Set di istruzioni AVX mancante.** Se la vostra architettura è corretta ma vedete ancora `Illegal instruction`, il vostro CPU probabilmente manca di AVX o di un'altra istruzione che il binario richiede. Questo influisce approssimativamente sui processori Intel e AMD pre-2013, e sulle macchine virtuali dove l'hypervisor non passa AVX all'ospite.

Su un VPS o VM, eseguire `grep -m1 -ow avx /proc/cpuinfo`; un risultato vuoto significa che AVX non è disponibile per l'ospite.

Non c'è una soluzione binaria nativa; tracciare il [problema #50384](https://github.com/anthropics/claude-code/issues/50384) per lo stato, e includere il vostro modello di CPU da `grep -m1 "model name" /proc/cpuinfo` su Linux o `sysctl -n machdep.cpu.brand_string` su macOS quando segnalate.

I metodi di installazione alternativi scaricano lo stesso binario nativo e non risolveranno nessuna delle due cause.

<h3 id="dyld-cannot-load-on-macos">
  `dyld: cannot load` su macOS
</h3>

Se vedete `dyld: Symbol not found`, `dyld: cannot load`, o `Abort trap: 6` durante l'installazione, il binario è incompatibile con la vostra versione di macOS o hardware.

Un errore `Symbol not found` che fa riferimento a `libicucore` significa che la vostra versione di macOS è più vecchia di quella che il binario supporta:

```text theme={null}
dyld: Symbol not found: _ubrk_clone
  Referenced from: claude-darwin-x64 (which was built for Mac OS X 13.0)
  Expected in: /usr/lib/libicucore.A.dylib
```

Il loader può invece rifiutare i comandi di caricamento del binario, il che significa anche che la vostra versione di macOS è troppo vecchia:

```text theme={null}
dyld: cannot load 'claude-2.1.42-darwin-x64' (load command 0x80000034 is unknown)
Abort trap: 6
```

**Soluzioni:**

1. **Verificare la vostra versione di macOS**: Claude Code richiede macOS 13.0 o successivo. Aprire il menu Apple e selezionare About This Mac per verificare la vostra versione.

2. **Aggiornare macOS** se siete su una versione precedente. Il binario utilizza comandi di caricamento e librerie di sistema che le versioni precedenti di macOS non supportano. I metodi di installazione alternativi come Homebrew scaricano lo stesso binario e non risolveranno questo errore.

<h3 id="exec-format-error-on-wsl1">
  `Exec format error` su WSL1
</h3>

Se l'esecuzione di `claude` in WSL stampa `cannot execute binary file: Exec format error`, siete su WSL1 e colpite una regressione binaria nativa nota tracciata nel [problema #38788](https://github.com/anthropics/claude-code/issues/38788). Le intestazioni del programma del binario sono cambiate in un modo che il loader di WSL1 non può gestire.

La correzione più pulita è convertire la vostra distribuzione a WSL2 da PowerShell:

```powershell theme={null}
wsl --set-version <DistroName> 2
```

Se dovete rimanere su WSL1, invocare il binario attraverso il linker dinamico. Aggiungere questa funzione a `~/.bashrc` dentro WSL, sostituendo il percorso se la vostra directory home è diversa:

```bash theme={null}
claude() {
  /lib64/ld-linux-x86-64.so.2 "$(readlink -f "$HOME/.local/bin/claude")" "$@"
}
```

Quindi eseguire `source ~/.bashrc` e riprovare `claude`.

<h3 id="npm-install-errors-in-wsl">
  Errori di installazione npm in WSL
</h3>

Questi problemi si applicano se avete installato Claude Code con `npm install -g` dentro WSL. Se avete utilizzato il [programma di installazione nativo](/docs/it/setup), saltare questa sezione.

**Problemi di rilevamento del sistema operativo o della piattaforma.** Se npm segnala una mancata corrispondenza della piattaforma durante l'installazione, WSL probabilmente sta raccogliendo il `npm` di Windows. Eseguire `npm config set os linux` per primo, quindi installare con `npm install -g @anthropic-ai/claude-code --force`. Non utilizzare `sudo`.

**`exec: node: not found` quando si esegue `claude`.** Il vostro ambiente WSL probabilmente sta utilizzando l'installazione di Node.js di Windows. Confermare con `which npm` e `which node`: i percorsi che iniziano con `/mnt/c/` sono binari di Windows, mentre i percorsi Linux iniziano con `/usr/`. Per risolvere questo, installare Node tramite il gestore di pacchetti della vostra distribuzione Linux o tramite [`nvm`](https://github.com/nvm-sh/nvm).

**Conflitti di versione nvm.** Se avete nvm installato sia in WSL che in Windows, il cambio di versioni di Node in WSL potrebbe rompersi perché WSL importa il PATH di Windows per impostazione predefinita e il nvm di Windows ha priorità. La causa più comune è che nvm non è caricato nella vostra shell. Aggiungere il caricatore nvm a `~/.bashrc` o `~/.zshrc`:

```bash theme={null}
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"
```

O caricarlo nella vostra sessione corrente:

```bash theme={null}
source ~/.nvm/nvm.sh
```

Se nvm è caricato ma i percorsi di Windows hanno ancora priorità, anteporre esplicitamente il vostro percorso di Node Linux:

```bash theme={null}
export PATH="$HOME/.nvm/versions/node/$(node -v)/bin:$PATH"
```

<Warning>
  Evitare di disabilitare l'importazione del PATH di Windows tramite `appendWindowsPath = false` poiché questo rompe la capacità di chiamare eseguibili di Windows da WSL. Allo stesso modo, evitare di disinstallare Node.js da Windows se lo utilizzate per lo sviluppo di Windows.
</Warning>

<h3 id="permission-errors-during-installation">
  Errori di permesso durante l'installazione
</h3>

Se il programma di installazione nativo fallisce con errori di permesso, la directory di destinazione potrebbe non essere scrivibile. Vedere [Verificare i permessi della directory](#check-directory-permissions).

Se avete precedentemente installato con npm e state colpendo errori di permesso specifici di npm, passare al programma di installazione nativo:

```bash theme={null}
curl -fsSL https://claude.ai/install.sh | bash
```

<h3 id="native-binary-not-found-after-npm-install">
  Binario nativo non trovato dopo l'installazione npm
</h3>

Il pacchetto npm `@anthropic-ai/claude-code` scarica il binario nativo come dipendenza opzionale per piattaforma, come `@anthropic-ai/claude-code-darwin-arm64`. npm quindi esegue lo script postinstall del pacchetto, che copia quel binario in posizione come comando `claude`; fino a quando non viene eseguito, `claude` è uno script segnaposto. Se uno dei due passaggi di download o postinstall viene saltato, il segnaposto rimane in posizione, e l'esecuzione di `claude` su macOS e Linux stampa:

```text theme={null}
Error: claude native binary not installed.

Either postinstall did not run (--ignore-scripts, some pnpm configs)
or the platform-native optional dependency was not downloaded
(--omit=optional).

Run the postinstall manually (adjust path for local vs global install):
  node node_modules/@anthropic-ai/claude-code/install.cjs

Or reinstall without --ignore-scripts / --omit=optional.
```

Su Windows, `bin/claude.exe` è quello stesso segnaposto di script shell piuttosto che un eseguibile reale, quindi PowerShell e CMD segnalano che non possono eseguire il file invece di stampare questo messaggio.

Controllare le seguenti cause:

* **Le dipendenze opzionali sono disabilitate.** Rimuovere `--omit=optional` dal vostro comando di installazione npm, `--no-optional` da pnpm, o `--ignore-optional` da yarn, e verificare che `.npmrc` non imposti `optional=false`. Quindi reinstallare. Il binario nativo viene consegnato solo come dipendenza opzionale, quindi non c'è fallback JavaScript se viene saltato, e l'esecuzione di `install.cjs` di nuovo non può posizionare un binario che non è mai stato scaricato.
* **Gli script di installazione sono disabilitati.** `--ignore-scripts` e alcune configurazioni pnpm saltano il passaggio postinstall ma scaricano comunque il pacchetto della piattaforma. Eseguire `node node_modules/@anthropic-ai/claude-code/install.cjs` come il messaggio suggerisce, o reinstallare senza il flag. Se postinstall non può essere eseguito nel vostro ambiente affatto, `node node_modules/@anthropic-ai/claude-code/cli-wrapper.cjs` trova il pacchetto scaricato e lo avvia, al costo di un processo Node aggiuntivo su ogni avvio. Se il wrapper stampa `Could not find native binary package`, il pacchetto della piattaforma non è mai stato scaricato, quindi correggere prima la causa delle dipendenze opzionali sopra.
* **Piattaforma non supportata.** I binari precompilati sono pubblicati per `darwin-arm64`, `darwin-x64`, `linux-x64`, `linux-arm64`, `linux-x64-musl`, `linux-arm64-musl`, `win32-x64`, e `win32-arm64`. Claude Code non spedisce un binario per altre piattaforme; vedere i [requisiti di sistema](/docs/it/setup#system-requirements). Su FreeBSD, il programma di installazione segnala la piattaforma come non supportata. Prima di v2.1.205, la trattava come Linux e scaricava un binario che non poteva essere eseguito.
* **Lo specchio npm aziendale manca dei pacchetti della piattaforma.** Assicurarsi che il vostro registro specchi tutti gli otto pacchetti della piattaforma `@anthropic-ai/claude-code-*` oltre al pacchetto meta.

<h3 id="npm-enotempty-during-update-or-reinstall">
  Errore npm `ENOTEMPTY` durante l'aggiornamento o la reinstallazione
</h3>

Quando eseguite `npm install -g @anthropic-ai/claude-code` su un'installazione esistente, npm può fallire mentre sposta la vecchia directory del pacchetto da parte:

```text theme={null}
npm error code ENOTEMPTY
npm error syscall rename
npm error path /home/you/.nvm/versions/node/v22.13.1/lib/node_modules/@anthropic-ai/claude-code
npm error dest /home/you/.nvm/versions/node/v22.13.1/lib/node_modules/@anthropic-ai/.claude-code-tVWAnUUt
npm error errno -39
npm error ENOTEMPTY: directory not empty, rename '...'
```

La riga `npm error path` nomina la directory che npm non poteva spostare. Eliminare quella directory e qualsiasi directory `.claude-code-*` rimanente accanto ad essa, che le esecuzioni interrotte precedenti possono lasciare dietro. I comandi di seguito trovano la vostra directory di pacchetto globale con `npm root -g`; se la directory che la riga `npm error path` nomina non è sotto la directory che `npm root -g` stampa, ad esempio perché avete cambiato versioni di Node con nvm, eliminare le directory che l'errore nomina:

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    rm -rf "$(npm root -g)/@anthropic-ai/claude-code"
    ```

    Quindi rimuovere qualsiasi directory temp rimanente. Se Zsh stampa `no matches found`, non ce n'erano da rimuovere:

    ```bash theme={null}
    rm -rf "$(npm root -g)/@anthropic-ai/.claude-code-"*
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    Remove-Item -Recurse -Force "$(npm root -g)/@anthropic-ai/claude-code", "$(npm root -g)/@anthropic-ai/.claude-code-*"
    ```
  </Tab>
</Tabs>

Quindi reinstallare:

```bash theme={null}
npm install -g @anthropic-ai/claude-code
```

Confermare con `claude --version`, che stampa un numero di versione come `2.1.211 (Claude Code)`.

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

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    unset ANTHROPIC_API_KEY
    claude
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    Remove-Item Env:ANTHROPIC_API_KEY
    claude
    ```
  </Tab>
</Tabs>

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

Le sessioni parallele su una macchina condividono un accesso salvato e coordinano il suo rinnovo in modo che solo un processo aggiorni il token alla volta. Prima della v2.1.211, il risveglio della macchina dal sonno potrebbe causare a due sessioni di rinnovare con lo stesso token, il che revocava l'accesso salvato e richiedeva a ogni sessione aperta di accedere di nuovo contemporaneamente.

Su macOS, Claude Code salva le credenziali nel Keychain di accesso. Quando il Keychain rifiuta la scrittura, ad esempio quando è bloccato in una sessione SSH o la sua password non è sincronizzata con la password del tuo account, Claude Code salva il tuo accesso nel file di testo semplice `~/.claude/.credentials.json` invece. Un accesso alla Console che crea una chiave API non riesce finché il Keychain non è di nuovo scrivibile.

Per rendere il Keychain scrivibile di nuovo e spostare il tuo accesso nel Keychain crittografato:

<Steps>
  <Step title="Controlla l'accesso al Keychain">
    Esegui `claude doctor` per controllare l'accesso al Keychain. Quando il Keychain rifiuta le scritture, il rapporto elenca un avviso che inizia con `macOS Keychain is not writable`, seguito da una correzione suggerita. Quando il rapporto non elenca alcun avviso del Keychain, il Keychain è scrivibile e puoi saltare all'ultimo passaggio.
  </Step>

  <Step title="Sblocca il Keychain">
    ```bash theme={null}
    security unlock-keychain ~/Library/Keychains/login.keychain-db
    ```

    Inserisci la tua password del Keychain quando il comando la richiede, quindi esegui di nuovo `claude doctor`. Quando lo sblocco ha funzionato, il rapporto non elenca più l'avviso del Keychain.
  </Step>

  <Step title="Risincronizza la password del Keychain se lo sblocco non aiuta">
    Apri Accesso Portachiavi, seleziona il keychain `login`, e scegli **Modifica > Cambia password per Portachiavi "login"** per risincronizzarlo con la password del tuo account. Quindi esegui di nuovo `claude doctor`. Procedi al passaggio successivo una volta che il rapporto non elenca più l'avviso del Keychain.
  </Step>

  <Step title="Esci e accedi di nuovo">
    Una volta che il Keychain è di nuovo scrivibile, Claude Code sposta le credenziali indietro la prossima volta che scrive una credenziale. Per forzarlo ora, esegui `/logout` e poi `/login`. L'uscita rimuove tutte le credenziali archiviate, inclusi i contenuti del file di testo semplice, gli accessi ai server MCP salvati e i valori sensibili dei plugin, quindi aspettati di re-autorizzare i server MCP e re-inserire i segreti dei plugin in seguito. L'accesso di nuovo archivia il tuo accesso nel Keychain.
  </Step>
</Steps>

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
4. Se il problema riguarda il tuo account piuttosto che l'installazione, come un ciclo di accesso, un abbonamento non riconosciuto, o un'organizzazione disabilitata, contatta il supporto di Anthropic: accedi a [claude.ai](https://claude.ai) (utenti Console: [platform.claude.com](https://platform.claude.com)), fai clic sulle tue iniziali in basso a sinistra, e seleziona **Ottieni aiuto**. Vedi [Come ottenere supporto](https://support.claude.com/en/articles/9015913-how-to-get-support) per il flusso completo.
