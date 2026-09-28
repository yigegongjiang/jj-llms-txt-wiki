> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Risolvere i problemi dell'Agent SDK

> Correggi gli errori dell'Agent SDK quando la CLI di Claude Code non si avvia, il processo CLI esce, o un risultato riuscito arriva senza output strutturato.

Questa pagina copre gli errori dell'Agent SDK nell'avvio della CLI, nell'uscita del processo CLI e negli output strutturati. Le voci in questa pagina sono associate all'errore che vedi. Ogni voce indica la causa e cosa fare.

I sintomi legati a una funzionalità, come un hook che non si attiva o una skill che non viene utilizzata, hanno una sezione di troubleshooting nella pagina di quella funzionalità. La tabella indica la sezione o la pagina che copre ogni sintomo:

| Sintomo                                                                                                                                                                                                                                                                                                                                                    | Vai a                                                                                                                                       |
| :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| Skills non trovate, una skill non viene utilizzata, errore `Invalid skill name`                                                                                                                                                                                                                                                                            | [Troubleshooting delle skills](/docs/it/agent-sdk/skills#troubleshooting)                                                                        |
| Il server MCP mostra lo stato `failed`, gli strumenti non vengono chiamati, timeout di connessione, output dello strumento che supera il numero massimo di token consentiti                                                                                                                                                                                | [Troubleshooting di MCP](/docs/it/agent-sdk/mcp#troubleshooting)                                                                                 |
| Plugin non caricato, le skill del plugin non appaiono                                                                                                                                                                                                                                                                                                      | [Troubleshooting dei plugin](/docs/it/agent-sdk/plugins#troubleshooting)                                                                         |
| Claude non delega ai subagent, gli agenti basati su filesystem non si caricano                                                                                                                                                                                                                                                                             | [Troubleshooting dei subagent](/docs/it/agent-sdk/subagents#troubleshooting)                                                                     |
| Le opzioni di checkpointing non sono riconosciute, i messaggi dell'utente senza UUID, `No file checkpoint found`, `File rewinding is not enabled`, `ProcessTransport is not ready for writing`                                                                                                                                                             | [Troubleshooting del file checkpointing](/docs/it/agent-sdk/file-checkpointing#troubleshooting)                                                  |
| Hook non si attiva, il matcher non filtra come previsto, timeout dell'hook, lo strumento viene bloccato inaspettatamente, l'input modificato non viene applicato, gli hook di sessione non disponibili in Python, i prompt di autorizzazione del subagent si moltiplicano, i loop di hook ricorsivi con i subagent, `systemMessage` non appare nell'output | [Correggere i problemi comuni](/docs/it/agent-sdk/hooks#fix-common-issues) nella pagina degli hook                                               |
| Un agente che funziona sulla tua macchina fallisce in un servizio distribuito o in un contenitore                                                                                                                                                                                                                                                          | [Risolvere i problemi di distribuzione](/docs/it/agent-sdk/hosting#troubleshoot-deployment-failures)                                             |
| `Not logged in`, `Invalid API key`, `API Error`, `429`, `There's an issue with the selected model`                                                                                                                                                                                                                                                         | [Riferimento degli errori](/docs/it/errors#find-your-error)                                                                                      |
| `CLINotFoundError`, `CLIConnectionError`, `ProcessError`, `Claude Code process exited with code N`, `Claude Code returned an error result`, `structured_output` è `None`                                                                                                                                                                                   | [Avvio della CLI](#cli-startup), [Uscita del processo CLI](#cli-process-exit), e [Output strutturati](#structured-outputs) in questa pagina |

<h2 id="cli-startup">
  Avvio CLI
</h2>

<h3 id="clinotfounderror-claude-code-not-found">
  CLINotFoundError: Claude Code not found
</h3>

L'SDK Python avvia il CLI di Claude Code come un sottoprocesso. Quando non riesce a trovare un eseguibile `claude`, la connessione non riesce con un `CLINotFoundError`:

```
Claude Code not found at: /your/configured/path
```

Il messaggio include il percorso configurato quando imposti `ClaudeAgentOptions(cli_path=...)` e punta a un file mancante. Senza `cli_path`, l'SDK cerca nel tuo `PATH` e nelle posizioni di installazione comuni, e il messaggio include le istruzioni di installazione per la tua piattaforma.

Per correggerlo:

* Installa Claude Code se non è installato. Vedi [Install Claude Code](/docs/it/setup#install-claude-code) per il comando sulla tua piattaforma.
* Se hai impostato `cli_path`, conferma che il file esiste ed è l'eseguibile `claude`.
* Se dipendi dalla risoluzione di `PATH`, conferma che `claude --version` funziona nello stesso ambiente in cui viene eseguita la tua applicazione. I processi che avvii al di fuori della tua shell, ad esempio da un IDE o da un gestore di servizi, spesso vengono eseguiti con un `PATH` diverso.

L'SDK TypeScript cerca il CLI nel suo pacchetto di piattaforma in bundle e nel percorso che imposti in `pathToClaudeCodeExecutable`. Abbina il messaggio che vedi:

* `Native CLI binary for <platform>-<arch> not found`: il pacchetto di piattaforma in bundle è mancante, il più delle volte perché l'installazione ha saltato le dipendenze opzionali. Reinstalla `@anthropic-ai/claude-agent-sdk` senza saltare le dipendenze opzionali, oppure punta `pathToClaudeCodeExecutable` a un'[installazione nativa](/docs/it/setup#install-claude-code). In un eseguibile a file singolo creato con `bun build --compile`, lo stesso messaggio ha una causa e una soluzione diverse. Vedi [Compile to a single executable](/docs/it/agent-sdk/typescript#compile-to-a-single-executable).
* `Claude Code native binary not found at <path>` o `Claude Code executable not found at <path>. Is options.pathToClaudeCodeExecutable set?`: il file nel percorso risolto è mancante, oppure il processo non può accedervi. Conferma che il file esiste in quel percorso e che il processo può accedervi.

<h3 id="cliconnectionerror-refusing-to-execute-batch-script">
  CLIConnectionError: Refusing to execute batch script
</h3>

Su Windows, la connessione non riesce con un `CLIConnectionError` quando il percorso CLI che l'SDK Python utilizza è uno script batch `.bat` o `.cmd`, incluso lo shim `claude.cmd` che un'installazione npm crea:

```
Refusing to execute batch script 'C:\\Users\\you\\AppData\\Roaming\\npm\\claude.cmd': Windows runs .bat/.cmd files via cmd.exe, which can execute commands injected through CLI arguments, and no reliable escaping for cmd.exe exists. Use a native claude executable instead: install Claude Code natively (irm https://claude.ai/install.ps1 | iex), point ClaudeAgentOptions(cli_path=...) at a claude.exe, or install the claude-agent-sdk wheel for a platform that bundles claude.exe (e.g. Windows x64).
```

Il rifiuto è un indurimento della sicurezza deliberato, non un'installazione interrotta. Windows esegue gli script batch riscrivendo lo spawn in un'invocazione `cmd.exe /c`, e `cmd.exe` ripete l'analisi dell'intera riga di comando al momento dell'esecuzione, quindi un valore di argomento può eseguire comandi iniettati.

La maggior parte delle installazioni Windows non raggiunge mai questo errore. La wheel x64 di Windows di `claude-agent-sdk` include un `claude.exe`, e l'SDK preferisce il CLI in bundle, quindi qualsiasi `claude.exe` nativo che riesce a scoprire, prima di ricorrere a uno shim batch. Vedi il rifiuto in due casi:

* Hai impostato `ClaudeAgentOptions(cli_path=...)` su un file `.bat` o `.cmd`, come lo shim `claude.cmd` di npm.
* La tua installazione non ha un `claude.exe` in bundle o nativo, ad esempio un'installazione da sorgente su ARM64 Windows dove l'unico `claude` nel tuo `PATH` è lo shim npm.

Per correggerlo, dai all'SDK un eseguibile nativo invece di uno script batch:

* Se hai impostato `ClaudeAgentOptions(cli_path=...)`, puntalo a un `claude.exe` o rimuovi l'opzione. L'SDK salta la scoperta mentre `cli_path` è impostato, quindi un'installazione nativa da sola non può avere effetto.
* Installa Claude Code nativamente in PowerShell: `irm https://claude.ai/install.ps1 | iex`
* Su Windows x64, installa la wheel `claude-agent-sdk`, che include `claude.exe`.

Prima di `claude-agent-sdk` 0.2.124, l'SDK Python generava script batch tramite `cmd.exe` senza questo controllo.

<h3 id="cliconnectionerror-failed-to-start-claude-code">
  CLIConnectionError: Failed to start Claude Code
</h3>

L'SDK ha trovato un file nel percorso risolto ma non ha potuto avviarlo. Python genera questi errori come un `CLIConnectionError`. TypeScript rifiuta l'iterazione del messaggio con un errore che non porta alcuna classe SDK. La tabella seguente mappa ogni messaggio a ciò che ti dice. Abbina il messaggio che vedi:

| Messaggio                                                         | SDK        | Cosa ti dice                                                                       |
| ----------------------------------------------------------------- | ---------- | ---------------------------------------------------------------------------------- |
| `Failed to start Claude Code: <detail>`                           | Python     | Il resto del messaggio è l'errore del sistema operativo stesso                     |
| `Claude Code executable at <path> exists but failed to launch`    | TypeScript | Lo script nel percorso configurato non può essere eseguito                         |
| `Claude Code native binary at <path> exists but failed to launch` | TypeScript | Il binario non può essere eseguito, con un suggerimento libc aggiunto al messaggio |
| `Failed to spawn Claude Code process: <detail>`                   | TypeScript | Qualsiasi altro errore di avvio                                                    |

In entrambi gli SDK, la causa più comune è un percorso risolto che punta a qualcosa che non può essere eseguito, come un file di testo, una directory o un file senza permesso di esecuzione. Leggi il suggerimento libc del messaggio del binario nativo come una possibile causa.

Per correggerlo in uno qualsiasi degli SDK:

* Conferma che il percorso configurato punta all'eseguibile `claude` stesso e che il file ha il permesso di esecuzione.
* Se non hai bisogno di un percorso personalizzato, rimuovi `cli_path` in Python o `pathToClaudeCodeExecutable` in TypeScript in modo che l'SDK trovi un CLI da solo, preferendo la sua copia in bundle.
* Quando il binario che non funziona è la copia in bundle dell'SDK in un'immagine contenitore, reinstalla l'SDK durante la compilazione dell'immagine in modo che il binario in bundle corrisponda alla piattaforma del contenitore, oppure ricompila l'immagine per l'architettura su cui viene eseguita. La causa più comune è un binario che non corrisponde all'architettura o alla libc del contenitore, oppure uno che ha perso il suo permesso di esecuzione nella compilazione dell'immagine.

<h3 id="cliconnectionerror-not-connected">
  CLIConnectionError: Not connected
</h3>

Chiamare un metodo `ClaudeSDKClient` in Python prima che il client si sia connesso, o dopo che si sia disconnesso, genera un `CLIConnectionError` con questo messaggio:

```
Not connected. Call connect() first.
```

Fai quello che dice il messaggio. Chiama `await client.connect()` prima di qualsiasi altro metodo client, oppure apri il client con `async with ClaudeSDKClient() as client:`, che si connette all'ingresso.

<h2 id="cli-process-exit">
  Uscita del processo CLI
</h2>

Le voci in questa sezione significano che il processo Claude Code è terminato mentre la tua applicazione lo stava utilizzando. Quale errore vedi dipende dal linguaggio SDK e dal fatto che il CLI abbia segnalato un risultato di errore prima di uscire.

<h3 id="processerror-command-failed-with-exit-code">
  ProcessError: Command failed with exit code
</h3>

L'SDK Python genera un `ProcessError` quando il processo Claude Code esce con un codice diverso da zero:

```
Command failed with exit code 1 (exit code: 1)
Error output: Check stderr output for details
```

Il messaggio indica il codice di uscita due volte, e la riga `Error output` è testo fisso piuttosto che l'output di errore del tuo processo. Lo stesso testo fisso riempie l'attributo `stderr` dell'eccezione. L'attributo `exit_code` dell'eccezione porta il codice. Per acquisire ciò che il CLI ha effettivamente scritto su stderr, passa un callback `stderr` in `ClaudeAgentOptions` e registra ciò che riceve.

Un `ProcessError` nudo significa che il CLI è uscito senza segnalare un risultato di errore. Quando il CLI ha segnalato uno, l'SDK genera [`ResultError`](/docs/it/agent-sdk/python#resulterror) invece, coperto in [Claude Code returned an error result](#claude-code-returned-an-error-result). `ResultError` è una sottoclasse di `ProcessError`, quindi `except ProcessError` cattura entrambi. Per gestirli diversamente, metti la clausola `except ResultError` per prima.

Prima di `claude-agent-sdk` 0.2.140, l'SDK Python generava uscite di risultato di errore come una semplice `Exception` piuttosto che un `ResultError`.

<h3 id="claude-code-process-exited-with-code-n">
  Claude Code process exited with code N
</h3>

I wrapper IDE stampano anche questo messaggio, e il [riferimento agli errori](/docs/it/errors#claude-code-process-exited-with-code-n) lo copre per VS Code e altri launcher. Questa voce copre ciò che il tuo codice SDK TypeScript riceve. L'SDK presenta un'uscita CLI con codice diverso da zero come un semplice `Error` che rifiuta il ciclo `for await` sui messaggi di `query()`. Non c'è alcuna classe di errore SDK da catturare, quindi avvolgi il ciclo in `try`/`catch` e abbina il messaggio:

```
Claude Code process exited with code 1. stderr: <tail of the CLI's stderr>
```

Quando il CLI ha scritto su stderr, il messaggio termina con la coda di esso. Per acquisire il flusso completo, passa un callback `stderr` nelle opzioni di query. Un processo ucciso da un segnale segnala `Claude Code process terminated by signal <name>` nella stessa forma.

<h3 id="claude-code-returned-an-error-result">
  Claude Code returned an error result
</h3>

Entrambi gli SDK sostituiscono l'errore di uscita del processo con questo messaggio quando il CLI ha segnalato un risultato di errore prima di uscire:

```
Claude Code returned an error result: <the CLI's own error report>
```

Il testo dopo i due punti è il rapporto del CLI su ciò che è andato storto, quindi inizia da lì piuttosto che dall'uscita stessa. Python genera questo come un [`ResultError`](/docs/it/agent-sdk/python#resulterror), il cui attributo `data` porta il risultato di errore completo. TypeScript rifiuta il ciclo dei messaggi con un semplice `Error` che porta la stessa forma di messaggio.

<h2 id="structured-outputs">
  Output strutturati
</h2>

<h3 id="structured_output-is-none-but-the-result-says-success">
  structured\_output is None but the result says success
</h3>

Un messaggio di risultato può terminare con `subtype: "success"` mentre `structured_output` è `None` in Python o `undefined` in TypeScript. L'esecuzione si completa, ma non esiste un output convalidato. Un modo per raggiungere questo è uno schema che nessun output può soddisfare, ad esempio vincoli di lunghezza in conflitto. L'esecuzione termina senza un errore di convalida, e l'unico segnale è il `structured_output` mancante.

Tratta questo risultato come un errore nel codice dell'applicazione. Controlla sia che `subtype` sia `success` che che `structured_output` sia presente prima di utilizzarlo. La sezione [Error handling](/docs/it/agent-sdk/structured-outputs#error-handling) mostra questo modello per entrambi gli SDK.

Se accade ripetutamente con uno schema che ritieni sia corretto, verifica che lo schema sia soddisfacibile, quindi semplificalo finché gli output non si convalidano, e reintroduci i vincoli uno alla volta.

<h2 id="report-a-new-issue">
  Segnala un nuovo problema
</h2>

Se il tuo errore non è coperto qui, controlla i problemi aperti o apri un nuovo problema nei repository SDK: [claude-agent-sdk-typescript](https://github.com/anthropics/claude-agent-sdk-typescript/issues) o [claude-agent-sdk-python](https://github.com/anthropics/claude-agent-sdk-python/issues). Includi il testo di errore completo e la tua versione SDK.
