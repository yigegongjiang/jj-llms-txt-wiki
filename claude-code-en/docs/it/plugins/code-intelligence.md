> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Code intelligence plugins

> Installa un plugin di language server in modo che Claude veda gli errori di tipo dopo le modifiche e navighi il codice per simbolo, e rispondi alla finestra di dialogo di raccomandazione del plugin LSP.

Un plugin di code intelligence fornisce a Claude la diagnostica in tempo reale e la funzione go-to-definition che ha il tuo editor, in modo che Claude catturi gli errori di tipo e le importazioni mancanti che le sue stesse modifiche introducono prima di eseguire la compilazione, e trovi definizioni e riferimenti per simbolo invece di cercarli per testo.

Ogni plugin connette Claude Code a un language server per un linguaggio attraverso il Language Server Protocol (LSP). Installi il plugin dal marketplace ufficiale di Anthropic e il binario del language server sulla tua macchina.

<Note>
  I plugin di code intelligence funzionano nelle sessioni di terminale. Nelle [sessioni cloud](/docs/it/claude-code-on-the-web), Claude Code non avvia i language server dei plugin, quindi Claude non ottiene diagnostica o navigazione del codice lì. Per scrivere il tuo plugin di language server, o per connettere un language server che non ha un plugin, vedi [LSP servers in plugin components](/docs/it/plugins/components#lsp-servers).
</Note>

Per iniziare, trova il tuo linguaggio nella tabella sotto [Install a code intelligence plugin](#install-a-code-intelligence-plugin). I plugin in quella tabella provengono dal [marketplace ufficiale dei plugin](/docs/it/plugins/anthropic-marketplaces) di Anthropic.

Se hai già visto una finestra di dialogo di **LSP plugin recommendation**, vedi [Accept or dismiss the recommendation dialog](#accept-or-dismiss-the-recommendation-dialog) per sapere cosa fa ogni scelta.

<h2 id="install-a-code-intelligence-plugin">
  Install a code intelligence plugin
</h2>

Un plugin di code intelligence dice a Claude Code quale comando avvia il language server e quali estensioni di file gestisce. Non include il language server. Installa prima il binario del language server, poi il plugin, poi conferma che il server si avvia.

<Steps>
  <Step title="Install the language server binary">
    Trova il tuo linguaggio nella tabella sottostante e installa il binario nella sua riga. Se il tuo linguaggio non è elencato, vedi [Add a language without an official plugin](#add-a-language-without-an-official-plugin).

    | Language                  | Plugin                                                                                                           | Binary                          |
    | :------------------------ | :--------------------------------------------------------------------------------------------------------------- | :------------------------------ |
    | C/C++                     | [`clangd-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/clangd-lsp)               | `clangd`                        |
    | C#                        | [`csharp-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/csharp-lsp)               | `csharp-ls`                     |
    | Go                        | [`gopls-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/gopls-lsp)                 | `gopls`                         |
    | Java                      | [`jdtls-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/jdtls-lsp)                 | `jdtls`                         |
    | Kotlin                    | [`kotlin-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/kotlin-lsp)               | `kotlin-lsp`                    |
    | Liquid                    | [`liquid-lsp`](https://github.com/Shopify/liquid-skills/tree/main/plugins/liquid-lsp)                            | `shopify`, from the Shopify CLI |
    | Lua                       | [`lua-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/lua-lsp)                     | `lua-language-server`           |
    | PHP                       | [`php-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/php-lsp)                     | `intelephense`                  |
    | Python                    | [`pyright-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/pyright-lsp)             | `pyright-langserver`            |
    | Ruby                      | [`ruby-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/ruby-lsp)                   | `ruby-lsp`                      |
    | Rust                      | [`rust-analyzer-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/rust-analyzer-lsp) | `rust-analyzer`                 |
    | Swift                     | [`swift-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/swift-lsp)                 | `sourcekit-lsp`                 |
    | TypeScript and JavaScript | [`typescript-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/typescript-lsp)       | `typescript-language-server`    |

    Anthropic mantiene ogni plugin nella tabella tranne `liquid-lsp`, che Shopify mantiene e il marketplace ufficiale elenca.

    Per trovare il comando che installa il binario, segui il link del plugin nella tabella al suo README. Per TypeScript, quel comando è `npm install -g typescript-language-server typescript`.

    Dopo aver installato il binario, conferma che sia nel `PATH` della shell da cui avvii `claude`, ad esempio con `which typescript-language-server`, o `Get-Command typescript-language-server` in PowerShell.
  </Step>

  <Step title="Install the plugin">
    Per installare il plugin elencato per il tuo linguaggio nella tabella del passaggio 1, esegui `/plugin install` in una sessione di Claude Code, sostituendo `typescript-lsp` con il nome di quel plugin:

    ```
    /plugin install typescript-lsp@claude-plugins-official
    ```

    Un messaggio di conferma dice se il plugin è attivo ora o ha bisogno di `/reload-plugins`. Se l'installazione fallisce con `Marketplace "claude-plugins-official" not found`, vedi la [voce di risoluzione dei problemi per quell'errore](/docs/it/plugins/troubleshooting#marketplace-claude-plugins-official-not-found). Per controllare dove il plugin è installato, o per eseguire l'installazione dalla tua shell invece che dentro Claude Code, vedi [Install plugins](/docs/it/plugins/install).
  </Step>

  <Step title="Confirm the server starts">
    Il language server si avvia la prima volta che Claude modifica un file con una delle estensioni del plugin. Per vederlo funzionare, chiedi a Claude di introdurre un errore di tipo in un file di quel linguaggio e poi correggerlo. Poi controlla la conversazione per una riga di diagnostica:

    * **Una riga di diagnostica appare**: `Found N new diagnostic issues in M files (ctrl+o to expand)` sotto la modifica che ha introdotto l'errore significa che il server si è avviato.
    * **Nessuna riga di diagnostica appare**: esegui `/plugin` e apri la scheda **Errors**. Una riga che legge `Executable not found in $PATH: "<binary>"` nomina il binario da installare. Se la scheda non ha tale riga, vedi [Troubleshoot code intelligence](#troubleshoot-code-intelligence).

    Dopo aver installato un binario mancante, Claude Code riprova la prossima volta che Claude modifica un file corrispondente. Se hai installato il binario in una directory che non è nel `PATH` della shell da cui hai avviato `claude`, avvia una nuova sessione da una shell dove lo è.
  </Step>
</Steps>

<h2 id="see-what-claude-gains">
  See what Claude gains
</h2>

Con un language server in esecuzione, Claude guadagna diagnostica e navigazione del codice:

* **Diagnostica dopo le modifiche**: ogni volta che Claude modifica o scrive un file che il server gestisce, Claude ottiene gli errori e gli avvisi che il server segnala. Vede un errore di tipo, un'importazione mancante, o un errore di sintassi che ha introdotto senza eseguire un compilatore.
* **Navigazione del codice**: Claude ottiene uno strumento `LSP` che cerca i simboli attraverso il server invece di cercarli nel testo. Lo strumento è di sola lettura. Per quello che Claude può cercare con lo strumento e come i permessi si applicano ad esso, vedi [LSP tool behavior](/docs/it/tools-reference#lsp-tool-behavior).

<h3 id="read-the-diagnostics-yourself">
  Read the diagnostics yourself
</h3>

Dopo che Claude modifica un file che il server gestisce, la conversazione mostra solo il riassunto `Found N new diagnostic issues`. Per leggere i problemi stessi, premi **Ctrl+O**.

<h2 id="accept-or-dismiss-the-recommendation-dialog">
  Accept or dismiss the recommendation dialog
</h2>

Se un binario di language server è già nel tuo `PATH` e il plugin che lo usa non è installato, Claude Code ti offre di installare il plugin per te in una finestra di dialogo intitolata **LSP plugin recommendation**.

<h3 id="when-the-recommendation-dialog-appears">
  When the recommendation dialog appears
</h3>

La finestra di dialogo **LSP plugin recommendation** può apparire dopo che Claude modifica un file. Queste condizioni decidono se appare e quale plugin offre:

* **Un plugin corrisponde al file**: uno dei marketplace che hai aggiunto, o il marketplace ufficiale che Claude Code ha registrato per te, elenca un plugin di code intelligence per l'estensione di quel file, e il binario del plugin è installato.
* **Ufficiale per primo**: quando più di un marketplace offre un plugin per l'estensione, la finestra di dialogo offre il plugin del marketplace ufficiale.
* **Una volta per sessione**: la finestra di dialogo appare al massimo una volta in una sessione, per il primo file corrispondente che Claude modifica.
* **Non per le sessioni cloud**: la finestra di dialogo non appare mai quando il tuo terminale è collegato a una sessione cloud, come una che hai avviato con [`claude --cloud`](/docs/it/claude-code-on-the-web#from-terminal-to-cloud).

<h3 id="respond-to-the-recommendation-dialog">
  Respond to the recommendation dialog
</h3>

La finestra di dialogo **LSP plugin recommendation** nomina il plugin e offre queste scelte:

* **Yes, install**: Claude Code installa il plugin per il tuo account utente e stampa `<plugin> installed · restart to apply`. Avvia una nuova sessione per caricare il server.
* **No, not now**: la finestra di dialogo si chiude, e una sessione successiva può offrire il plugin di nuovo. Premere **Esc** fa lo stesso.
* **Never for this plugin**: la finestra di dialogo smette di apparire per quel plugin e continua ad apparire per altri.
* **Disable all LSP recommendations**: la finestra di dialogo smette di apparire per ogni linguaggio.

Se non scegli un'opzione, Claude Code la chiude dopo 30 secondi e lo conta come ignorato. Il conteggio viene mantenuto tra le sessioni. Dopo cinque finestre di dialogo ignorate, Claude Code smette di raccomandare plugin, lo stesso che se avessi scelto **Disable all LSP recommendations**.

<h3 id="turn-recommendations-back-on">
  Turn recommendations back on
</h3>

La finestra di dialogo **LSP plugin recommendation** smette di apparire dopo che scegli **Disable all LSP recommendations** o la ignori cinque volte.

* **Disabilitato o ignorato cinque volte**: per riattivarlo in entrambi i casi, rimuovi le chiavi `lspRecommendationDisabled` e `lspRecommendationIgnoredCount` da `~/.claude.json`, il file di configurazione di Claude Code.
* **Never for this plugin**: se hai scelto **Never for this plugin** e vuoi che quel plugin sia offerto di nuovo, rimuovi il suo id `name@marketplace` dalla lista `lspRecommendationNeverPlugins` nello stesso file.

<h2 id="troubleshoot-code-intelligence">
  Troubleshoot code intelligence
</h2>

La pagina di risoluzione dei problemi dei plugin copre i sintomi specifici dei plugin di code intelligence sotto [Language server doesn't start, uses too much memory, or reports wrong diagnostics](/docs/it/plugins/troubleshooting#language-server-doesnt-start):

* **Il language server non si avvia**: vedi `Executable not found in $PATH` nella scheda **Errors** di `/plugin`, o Claude non segnala mai diagnostica per il linguaggio.
* **Uso elevato della memoria**: l'uso della memoria aumenta mentre il server indicizza il progetto.
* **Diagnostica falsa positiva in un monorepo**: la diagnostica segnala le importazioni come non risolte quando non lo sono.

<h2 id="add-a-language-without-an-official-plugin">
  Add a language without an official plugin
</h2>

Se il tuo linguaggio non è nella [tabella dei plugin ufficiali](#install-a-code-intelligence-plugin), puoi comunque connettere un language server.

1. Scrivi un plugin con un file `.lsp.json` che nomina il comando del server e le estensioni di file che gestisce.
2. Poi carica il plugin con [`--plugin-dir`](/docs/it/plugins/cli-reference#flags-that-load-a-plugin-for-one-session) o pubblicalo su un marketplace.

Per i campi del file e un esempio elaborato, vedi [LSP servers in plugin components](/docs/it/plugins/components#lsp-servers).

<h2 id="next-steps">
  Passaggi successivi
</h2>

* [Server LSP nei componenti plugin](/docs/it/plugins/components#lsp-servers): scrivere il `.lsp.json` per un server di linguaggio che non ha un plugin ufficiale
* [Installare e gestire i plugin](/docs/it/plugins/install): ambiti, aggiornamenti e disinstallazione
* [Risolvere i problemi dei plugin](/docs/it/plugins/troubleshooting): errori di caricamento oltre a quelli del server di linguaggio su questa pagina
* [Trovare i plugin nel marketplace ufficiale](/docs/it/plugins/anthropic-marketplaces#find-plugins-in-the-official-marketplace): dove sfogliare il resto del marketplace ufficiale
