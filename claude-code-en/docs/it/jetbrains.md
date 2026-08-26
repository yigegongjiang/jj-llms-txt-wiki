> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# JetBrains IDEs

> Usa Claude Code con JetBrains IDEs inclusi IntelliJ, PyCharm, WebStorm e altri

Claude Code si integra con JetBrains IDEs attraverso un plugin dedicato, fornendo funzionalità come la visualizzazione interattiva dei diff, la condivisione del contesto della selezione e altro ancora.

<h2 id="supported-ides">
  IDE supportati
</h2>

Il plugin Claude Code funziona con la maggior parte dei JetBrains IDEs, inclusi:

* IntelliJ IDEA
* PyCharm
* Android Studio
* WebStorm
* PhpStorm
* GoLand

<h2 id="features">
  Funzionalità
</h2>

* **Avvio rapido**: Usa `Cmd+Esc` (Mac) o `Ctrl+Esc` (Windows/Linux) per aprire Claude Code direttamente dal tuo editor, oppure fai clic sul pulsante Claude Code nell'interfaccia utente
* **Visualizzazione dei diff**: Le modifiche al codice possono essere visualizzate direttamente nel visualizzatore diff dell'IDE invece del terminale
* **Contesto della selezione**: La selezione corrente o la scheda nell'IDE viene automaticamente condivisa con Claude Code. Le [regole di negazione `Read`](/docs/it/permissions#read-and-edit) bloccano questa condivisione per i file corrispondenti
* **Scorciatoie di riferimento file**: Usa `Cmd+Option+K` (Mac) o `Alt+Ctrl+K` (Linux/Windows) per inserire riferimenti ai file come `@src/auth.ts#L1-99`
* **Condivisione diagnostica**: Gli errori diagnostici dall'IDE, come errori di lint e sintassi, vengono automaticamente condivisi con Claude mentre lavori

<h2 id="installation">
  Installazione
</h2>

Il plugin esegue il comando `claude` nel terminale integrato del tuo IDE e si connette ad esso. Non include una propria copia della CLI, quindi installa entrambi i componenti:

<Steps>
  <Step title="Installa Claude Code CLI">
    Segui la [guida di avvio rapido](/docs/it/quickstart) per installare la CLI se non l'hai già fatto. Il plugin mostra una notifica "Cannot launch Claude Code" quando `claude` non è nel tuo PATH.
  </Step>

  <Step title="Installa il plugin JetBrains">
    Installa il [plugin Claude Code](https://plugins.jetbrains.com/plugin/27310-claude-code-beta-) dal Marketplace di JetBrains e riavvia il tuo IDE.
  </Step>
</Steps>

Se `claude` è installato in un percorso che il tuo IDE non riesce a trovare, imposta il percorso completo nell'impostazione [Claude command](#general-settings) del plugin.

Claude Code funziona con qualsiasi abbonamento Claude a pagamento (Pro, Max, Team o Enterprise) o un account Claude Console, e non è richiesta alcuna chiave API. Ti verrà chiesto di [accedere](/docs/it/authentication#log-in-to-claude-code) la prima volta che esegui `claude`.

<Note>
  Dopo aver installato il plugin, potrebbe essere necessario riavviare completamente il tuo IDE affinché abbia effetto.
</Note>

<h2 id="usage">
  Utilizzo
</h2>

<h3 id="from-your-ide">
  Dal tuo IDE
</h3>

Esegui `claude` dal terminale integrato del tuo IDE e tutte le funzionalità di integrazione saranno attive.

<h3 id="from-external-terminals">
  Da terminali esterni
</h3>

Usa il comando `/ide` in qualsiasi terminale esterno per connettere Claude Code al tuo JetBrains IDE e attivare tutte le funzionalità:

```bash theme={null}
claude
```

```text theme={null}
/ide
```

Se desideri che Claude abbia accesso agli stessi file del tuo IDE, avvia Claude Code dalla stessa directory della radice del progetto del tuo IDE.

<h2 id="configuration">
  Configurazione
</h2>

<h3 id="claude-code-settings">
  Impostazioni di Claude Code
</h3>

Configura l'integrazione dell'IDE attraverso le impostazioni di Claude Code:

1. Esegui `claude`
2. Inserisci il comando `/config`
3. Imposta lo strumento diff su `auto` per mostrare i diff nell'IDE, oppure `terminal` per mantenerli nel terminale

<h3 id="plugin-settings">
  Impostazioni del plugin
</h3>

Configura il plugin Claude Code andando a **Impostazioni → Strumenti → Claude Code \[Beta]**:

<h4 id="general-settings">
  Impostazioni generali
</h4>

* **Comando Claude**: Specifica un comando personalizzato per eseguire Claude, ad esempio `claude`, `/usr/local/bin/claude`, o `npx @anthropic-ai/claude-code`
* **Sopprimere la notifica per il comando Claude non trovato**: Salta le notifiche relative al mancato reperimento del comando Claude
* **Abilita l'uso di Option+Invio per prompt multi-riga**: Solo su macOS. Quando abilitato, Option+Invio inserisce nuove righe nei prompt di Claude Code. Disabilita se il tasto Option viene catturato inaspettatamente. Richiede il riavvio del terminale.
* **Abilita aggiornamenti automatici**: Controlla automaticamente e installa gli aggiornamenti del plugin, applicati al riavvio

<Tip>
  Per gli utenti WSL: Imposta `wsl -d Ubuntu -- bash -lic "claude"` come comando Claude (sostituisci `Ubuntu` con il nome della tua distribuzione WSL)
</Tip>

<h4 id="esc-key-configuration">
  Configurazione del tasto ESC
</h4>

Se il tasto ESC non interrompe le operazioni di Claude Code nei terminali JetBrains:

1. Vai a **Impostazioni → Strumenti → Terminale**
2. Oppure:
   * Deseleziona "Sposta il focus sull'editor con Escape", oppure
   * Fai clic su "Configura scorciatoie da tastiera del terminale" e elimina la scorciatoia "Sposta il focus sull'editor"
3. Applica le modifiche

Questo consente al tasto ESC di interrompere correttamente le operazioni di Claude Code.

<h2 id="special-configurations">
  Configurazioni speciali
</h2>

<h3 id="remote-development">
  Sviluppo remoto
</h3>

<Warning>
  Quando usi JetBrains Remote Development, devi installare il plugin nell'host remoto tramite **Impostazioni → Plugin (Host)**.
</Warning>

Il plugin deve essere installato sull'host remoto, non sulla tua macchina client locale.

<h3 id="wsl-configuration">
  Configurazione WSL
</h3>

Se stai usando Claude Code su WSL2 con un JetBrains IDE e vedi "No available IDEs detected", la causa è solitamente la rete NAT di WSL2 o il Windows Firewall che blocca la connessione tra WSL2 e l'IDE in esecuzione sull'host Windows. WSL1 utilizza direttamente la rete dell'host e non è interessato.

<h4 id="allow-wsl2-traffic-through-windows-firewall">
  Consenti il traffico WSL2 attraverso Windows Firewall
</h4>

Questa è la correzione consigliata perché mantiene la tua modalità di rete WSL2 esistente.

<Steps>
  <Step title="Trova il tuo indirizzo IP WSL2">
    Dalla tua shell WSL, esegui:

    ```bash theme={null}
    hostname -I
    ```

    Annota la subnet, ad esempio `172.21.123.45` è in `172.21.0.0/16`.
  </Step>

  <Step title="Crea una regola firewall">
    Apri PowerShell come Amministratore ed esegui quanto segue, regolando l'intervallo IP per corrispondere alla tua subnet:

    ```powershell theme={null}
    New-NetFirewallRule -DisplayName "Allow WSL2 Internal Traffic" -Direction Inbound -Protocol TCP -Action Allow -RemoteAddress 172.21.0.0/16 -LocalAddress 172.21.0.0/16
    ```
  </Step>

  <Step title="Riavvia il tuo IDE e Claude Code">
    Chiudi e riapri entrambi affinché la nuova regola abbia effetto.
  </Step>
</Steps>

<h4 id="switch-wsl2-to-mirrored-networking">
  Passa WSL2 alla rete con mirroring
</h4>

La rete con mirroring richiede Windows 11 22H2 o successivo. Se sei su Windows 10, usa la regola firewall sopra indicata.

Aggiungi questo a `.wslconfig` nella tua directory utente Windows:

```ini theme={null}
[wsl2]
networkingMode=mirrored
```

Quindi riavvia WSL con `wsl --shutdown` da PowerShell.

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

<h3 id="plugin-not-working">
  Plugin non funzionante
</h3>

Se il plugin è installato ma le funzionalità di Claude Code non appaiono nel tuo IDE:

* Assicurati di eseguire Claude Code dalla directory radice del progetto
* Verifica che il plugin JetBrains sia abilitato nelle impostazioni dell'IDE
* Riavvia completamente l'IDE (potrebbe essere necessario farlo più volte)
* Per Remote Development, assicurati che il plugin sia installato nell'host remoto

<h3 id="ide-not-detected">
  IDE non rilevato
</h3>

Se l'esecuzione di `claude` mostra "No available IDEs detected":

* Verifica che il plugin sia installato e abilitato
* Riavvia completamente l'IDE
* Verifica che tu stia eseguendo Claude Code dal terminale integrato
* Per gli utenti WSL, consulta la [configurazione WSL](#wsl-configuration) sopra

<h3 id="command-not-found">
  Comando non trovato
</h3>

Se facendo clic sull'icona Claude viene visualizzato "command not found":

1. Verifica che Claude Code sia installato eseguendo `claude --version` in un terminale
2. Configura il percorso del comando Claude nelle impostazioni del plugin
3. Per gli utenti WSL, usa il formato del comando WSL menzionato nella sezione di configurazione

<h2 id="security-considerations">
  Considerazioni sulla sicurezza
</h2>

Quando Claude Code viene eseguito in un JetBrains IDE in modalità di autorizzazione [`acceptEdits`](/docs/it/permission-modes#auto-approve-file-edits-with-acceptedits-mode), potrebbe essere in grado di modificare i file di configurazione dell'IDE che possono essere eseguiti automaticamente dal vostro IDE. Questo potrebbe aumentare il rischio di eseguire Claude Code in modalità `acceptEdits` e consentire di aggirare i prompt di autorizzazione di Claude Code per l'esecuzione bash.

Quando si esegue in JetBrains IDEs, considera:

* Utilizzare la modalità di approvazione manuale per le modifiche
* Prestare particolare attenzione per assicurarsi che Claude sia utilizzato solo con prompt affidabili
* Essere consapevole di quali file Claude Code ha accesso per modificare

Per problemi di installazione o accesso a Claude Code al di fuori dell'IDE, consulta [Risolvi i problemi di installazione e accesso](/docs/it/troubleshoot-install).

<h3 id="the-built-in-ide-mcp-server">
  Il server MCP IDE integrato
</h3>

Quando il plugin è attivo, esegue un server MCP locale a cui la CLI si connette automaticamente. È così che la CLI apre i diff nel visualizzatore diff nativo dell'IDE, legge la selezione corrente per le menzioni `@` e inserisce i diagnostici di ispezione nella conversazione.

Il server è denominato `ide` ed è nascosto da `/mcp` perché non c'è nulla da configurare. Se la vostra organizzazione utilizza un [hook `PreToolUse`](/docs/it/hooks#pretooluse) per creare un elenco di strumenti MCP consentiti, tuttavia, dovrete sapere che esiste.

**Contesto di selezione e file aperto.** Mentre è connesso, la CLI include la selezione dell'editor corrente e il percorso del file attivo come contesto su ogni prompt che inviate. La trascrizione mostra una riga `⧉ Selected N lines from <file>` quando ciò accade. Per escludere un file sensibile come `.env`, aggiungete una [regola di negazione `Read`](/docs/it/permissions#read-and-edit) per il suo percorso. Una regola di negazione corrispondente impedisce sia il testo selezionato che l'avviso di file aperto per quel file di raggiungere Claude.

**Trasporto e autenticazione.** Il server ascolta su una porta effimera assegnata dal sistema operativo e la porta non è configurabile. Il trasporto è `ws://` non crittografato; sul loopback, qualsiasi processo che potrebbe catturare il traffico può anche leggere il token dal file di blocco, quindi TLS non aggiungerebbe protezione contro un attaccante locale. Ogni avvio dell'IDE genera un token di autenticazione casuale fresco, lo scrive in un file di blocco in `~/.claude/ide/<port>.lock` e la CLI deve presentarlo come intestazione `X-Claude-Code-Ide-Authorization` per connettersi. Se `CLAUDE_CONFIG_DIR` è impostato, il file di blocco viene scritto in `$CLAUDE_CONFIG_DIR/ide/` invece.

**Strumenti esposti al modello.** Il server ospita diversi strumenti, ma solo uno è visibile al modello. Il resto è RPC interno che la CLI utilizza per la propria interfaccia utente, come l'apertura di diff e la lettura di selezioni, e viene filtrato prima che l'elenco degli strumenti raggiunga Claude.

| Nome dello strumento (come visto dagli hook) | Cosa fa                                                                                                                             | Sola lettura |
| -------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- | ------------ |
| `mcp__ide__getDiagnostics`                   | Restituisce i diagnostici di ispezione dell'IDE, gli errori e gli avvisi mostrati nell'editor. Facoltativamente limitato a un file. | Sì           |

Il plugin JetBrains non espone uno strumento di esecuzione del codice al modello.

**Interfaccia di ascolto.** Quale interfaccia di rete il server si associa è controllato da **Accept connections from all network interfaces** in **Settings → Tools → Claude Code \[Beta] → Networking (Advanced)**. Con l'impostazione disabilitata, il server ascolta solo su `127.0.0.1` e non è raggiungibile da altri host. Con essa abilitata, la porta è raggiungibile dalla vostra rete locale. L'impostazione esiste per i casi in cui la CLI non può raggiungere l'IDE sul loopback, come WSL2 con networking NAT predefinito o una configurazione IDE remota; consulta [Configurazione WSL](#wsl-configuration) per questo scenario.

<Warning>
  L'abilitazione di **Accept connections from all network interfaces** rende la porta MCP dell'IDE raggiungibile dalla vostra rete locale. Le connessioni richiedono comunque il token di autenticazione dal file di blocco, ma poiché il trasporto è `ws://` non crittografato, sia il traffico della sessione che quel token attraversano la rete in testo non crittografato quando l'impostazione è attiva. Attivatela solo quando il loopback genuinamente non può funzionare. Per WSL2, preferite il [networking con mirroring](#switch-wsl2-to-mirrored-networking) in modo che l'interfaccia loopback di Windows sia condivisa con la VM Linux e il socket possa rimanere sul loopback.
</Warning>
