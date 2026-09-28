> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Scegli una modalità di autorizzazione

> Controlla se Claude chiede prima di agire. Cambia le modalità di autorizzazione con Shift+Tab nella CLI, l'indicatore di modalità in VS Code, o il selettore di modalità in Desktop.

Una modalità di autorizzazione imposta quali azioni Claude può intraprendere in una sessione senza chiederti prima. In modalità Manual, Claude Code si ferma e ti chiede prima della maggior parte delle azioni che modificano file, eseguono comandi shell, o raggiungono la rete. In [modalità auto](#eliminate-prompts-with-auto-mode), un secondo modello, il classificatore, esamina le azioni al tuo posto; [come il classificatore valuta le azioni](#how-the-classifier-evaluates-actions) elenca quali azioni esamina e quali le saltano.

Sui piani Pro, Max e Team, la modalità di autorizzazione iniziale integrata è la modalità auto. [Quale modalità inizia una sessione](#which-mode-a-session-starts-in) copre le superfici e le impostazioni che cambiano la modalità di autorizzazione iniziale. Puoi anche cambiare la modalità di autorizzazione di una sessione in esecuzione in qualsiasi momento.

<h2 id="available-modes">
  Modalità disponibili
</h2>

Ogni modalità fa un diverso compromesso tra comodità e controllo. La tabella seguente mostra cosa Claude può fare senza una richiesta di autorizzazione in ogni modalità. La modalità Manual appare sotto il suo valore di configurazione, `default`.

| Modalità                                                            | Cosa viene eseguito senza chiedere                                                                                           | Migliore per                                         |
| :------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------- |
| `default`                                                           | Solo letture                                                                                                                 | Revisionare ogni azione voi stessi, lavoro sensibile |
| [`acceptEdits`](#auto-approve-file-edits-with-acceptedits-mode)     | Letture, modifiche di file e comandi comuni del filesystem (`mkdir`, `touch`, `mv`, `cp`, ecc.)                              | Iterare sul codice che state revisionando            |
| [`plan`](#analyze-before-you-edit-with-plan-mode)                   | Letture, più comandi approvati dal classificatore quando [la modalità auto](#eliminate-prompts-with-auto-mode) è disponibile | Esplorare una codebase prima di modificarla          |
| [`auto`](#eliminate-prompts-with-auto-mode)                         | Tutto, con controlli di sicurezza in background                                                                              | Attività lunghe, ridurre l'affaticamento da prompt   |
| [`dontAsk`](#allow-only-pre-approved-tools-with-dontask-mode)       | Letture e strumenti pre-approvati; qualsiasi cosa che comporterebbe una richiesta viene negata                               | CI bloccato e script                                 |
| [`bypassPermissions`](#skip-all-checks-with-bypasspermissions-mode) | Tutto                                                                                                                        | Solo container e VM isolati                          |

La modalità che esamina ogni azione è denominata **Manual** nella CLI, in `claude --help`, nelle estensioni VS Code e JetBrains, e nell'app desktop. Il suo valore di configurazione è `default`, che è quello utilizzato da hooks e integrazioni SDK. La CLI accetta `manual` come alias ovunque digitate il valore, ad esempio `claude --permission-mode manual` o `"defaultMode": "manual"`. L'etichetta Manual e l'alias `manual` richiedono Claude Code v2.1.200 o successivo. L'etichetta dell'app desktop non dipende dalla vostra versione CLI.

Le scritture su [percorsi protetti](#protected-paths) non vengono mai auto-approvate tranne in modalità `bypassPermissions` e in sessioni in modalità plan dove le autorizzazioni di bypass sono disponibili, il che significa sessioni avviate in modo da [mettere `bypassPermissions` nel ciclo di modalità](#switch-permission-modes).

Le modalità impostano la linea di base. Sovrapponete [regole di autorizzazione](/docs/it/permissions#manage-permissions) su top per pre-approvare o bloccare strumenti specifici. Le regole di negazione bloccano in ogni modalità, inclusa `bypassPermissions`. Le regole di negazione e richiesta non si applicano a [`EndConversation`](/docs/it/tools-reference#endconversation-tool-behavior) finché Claude ha ancora almeno uno strumento che può chiamare. Le regole allow non hanno effetto in `bypassPermissions`.

<h3 id="actions-no-mode-auto-approves">
  Azioni che nessuna modalità auto-approva
</h3>

Claude Code non auto-approva quanto segue in nessuna modalità, inclusa `bypassPermissions`. Ogni punto collega alla sezione che dice cosa succede invece in ogni modalità:

* Strumenti corrispondenti a una [regola ask](/docs/it/permissions#manage-permissions) esplicita
* Strumenti connector che la vostra organizzazione [ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools), in sessioni dove quella impostazione raggiunge Claude Code
* Strumenti che richiedono interazione dell'utente: lo strumento integrato `AskUserQuestion` e strumenti MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool)
* Rimozioni `rm` e `rmdir` che prendono di mira un [percorso critico](#critical-paths), che nessuna regola allow o hook `PreToolUse` `"allow"` approva
* Le [protezioni di messaggistica cross-sessione](#skip-all-checks-with-bypasspermissions-mode)
* Letture al di fuori delle directory di lavoro mentre [`permissions.blockReadsOutsideWorkingDirectories`](/docs/it/settings-reference#permissions-blockreadsoutsideworkingdirectories) è attivo: comandi Bash riconosciuti che leggono file e qualsiasi [retry non sandboxato](/docs/it/sandboxing#the-unsandboxed-retry-escape-hatch) che necessita di approvazione per eseguire al di fuori del sandbox anche in modalità auto e modalità `bypassPermissions`. Richiede Claude Code v2.1.257 o successivo.

  Un comando che il parser della shell non riesce a tracciare, come uno che cambia directory più di una volta o esegue una subshell, richiede lo stesso modo anche quando non nomina alcun percorso esterno. Questo prompt non si applica quando il comando viene eseguito nella [sandbox](/docs/it/sandboxing) e la sandbox applica il blocco.

<h2 id="common-setups">
  Configurazioni comuni
</h2>

Le modalità di autorizzazione decidono se Claude chiede prima di un'azione, e la [sandbox Bash](/docs/it/sandboxing) e i [confini di isolamento](/docs/it/sandbox-environments) esterni decidono cosa un'azione può raggiungere una volta che viene eseguita. Ogni riga seguente abbina un obiettivo ai flag o alle impostazioni che ti portano lì e all'isolamento di cui ha bisogno, come punto di partenza. [Modalità disponibili](#available-modes) elenca cosa viene eseguito senza un prompt in ogni modalità.

| Vuoi                                                        | Inizia con                                                                                                                                                                      | Isolamento necessario                                                                                                                                                                          | Note                                                                                                                                                                                                                                                                                      |
| :---------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Revisionare ogni azione tu stesso                           | Modalità Manual: `claude --permission-mode default`                                                                                                                             | Nessuno                                                                                                                                                                                        | Lavoro sensibile, codice non familiare                                                                                                                                                                                                                                                    |
| Iterare localmente con meno prompt, senza un classificatore | Modalità Manual più la sandbox Bash in [modalità auto-allow](/docs/it/sandboxing#sandbox-modes): `claude --permission-mode default`, quindi esegui `/sandbox` e seleziona auto-allow | La sandbox Bash integrata, su macOS, Linux e WSL2                                                                                                                                              | Le regole di negazione si applicano comunque, e le regole di richiesta che nominano un comando, come `Bash(git push *)`, richiedono comunque un prompt. Per attivare la sandbox da un file di impostazioni, imposta [`sandbox.enabled`](/docs/it/settings-reference#sandbox-enabled) su `true` |
| Esplorare prima di modificare qualsiasi cosa                | `claude --permission-mode plan`                                                                                                                                                 | Nessuno                                                                                                                                                                                        | Claude Code blocca le modifiche finché non [approvi un piano](#review-and-approve-a-plan)                                                                                                                                                                                                 |
| Lavorare senza intervento in modalità auto                  | `claude --permission-mode auto`, la [modalità di autorizzazione iniziale integrata](#which-mode-a-session-starts-in) su Pro, Max e Team                                         | Nessuno; una sandbox o un container aggiunge difesa in profondità                                                                                                                              | Richiede un [modello supportato](#eliminate-prompts-with-auto-mode), e la tua organizzazione può [disattivare la modalità auto](#eliminate-prompts-with-auto-mode)                                                                                                                        |
| Eseguire in CI con una lista di autorizzazione esatta       | `claude -p "run the test suite" --permission-mode dontAsk --allowedTools "Bash(npm test)" "Read"`                                                                               | Nessuno oltre a quello che il tuo runner CI fornisce                                                                                                                                           | [Cloud sessions](/docs/it/claude-code-on-the-web) ignora `dontAsk` dai file di impostazioni                                                                                                                                                                                                    |
| Eseguire completamente senza intervento dentro un container | `claude -p "<prompt>" --dangerously-skip-permissions`                                                                                                                           | Richiesto: un container, VM, o il [runtime sandbox](/docs/it/sandbox-environments#sandbox-runtime); su Linux e macOS, eseguilo come [utente non root](#skip-all-checks-with-bypasspermissions-mode) | Cloud sessions ignora questa modalità dai file di impostazioni. In questa esecuzione `-p`, le [poche chiamate che richiederebbero comunque un prompt](#skip-all-checks-with-bypasspermissions-mode) vengono negate invece                                                                 |

La sandbox Bash e la modalità auto funzionano indipendentemente e si combinano, con le eccezioni elencate in [Modalità sandbox](/docs/it/sandboxing#sandbox-modes). Per l'interazione completa, vedi [Come il sandboxing si relaziona alle autorizzazioni e alle modalità di autorizzazione](/docs/it/sandboxing#how-sandboxing-relates-to-permissions-and-permission-modes) e [Come l'isolamento si relaziona alle modalità di autorizzazione](/docs/it/sandbox-environments#how-isolation-relates-to-permission-modes).

<h2 id="which-mode-a-session-starts-in">
  Quale modalità inizia una sessione
</h2>

Quando avvii una nuova sessione in un terminale, Claude Code prende la modalità di autorizzazione dal primo di questi che si applica:

1. Il flag `--permission-mode`, o `--dangerously-skip-permissions`

2. `permissions.defaultMode` in un [file di impostazioni](/docs/it/settings#where-settings-live)

   Se imposti `"auto"` in `.claude/settings.json` o `.claude/settings.local.json`, il valore non ha effetto, e Claude Code utilizza invece l'impostazione predefinita integrata piuttosto che un `defaultMode` da `~/.claude/settings.json`. Se imposti `"bypassPermissions"` in questi due file, non ha effetto nemmeno, e la sessione inizia in modalità Manual. Gli altri valori si applicano da qualsiasi file di impostazioni.

3. L'impostazione predefinita integrata

Le conversazioni che l'estensione VS Code avvia seguono il proprio elenco dell'estensione in [Cambia modalità di autorizzazione](#switch-permission-modes). Per la modalità di autorizzazione in cui Claude Code avvia una sessione ripresa, vedi [modalità di autorizzazione al ripristino](/docs/it/sessions#permission-mode-on-resume).

L'impostazione predefinita integrata `auto` richiede Claude Code v2.1.228 o successivo su macOS, Linux e WSL, e v2.1.233 o successivo su Windows nativo. Su versioni precedenti, l'impostazione predefinita integrata è Manual.

L'impostazione predefinita integrata dipende da come esegui Claude Code, dal tuo piano, e da se Claude Code potrebbe recuperare i suoi flag di funzionalità. La prima riga che corrisponde alla tua sessione si applica. La tabella copre le sessioni che avvii in un terminale o tramite l'estensione VS Code; per l'app desktop e claude.ai, vedi le schede Desktop e Web in [Cambia modalità di autorizzazione](#switch-permission-modes).

| Come esegui Claude Code                                                                                                                                                                                                                                                   | Modalità di autorizzazione iniziale integrata |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------- |
| Un file di impostazioni imposta `disableAutoMode` su `"disable"`                                                                                                                                                                                                          | `default`                                     |
| Il [recupero dei flag di funzionalità](/docs/it/env-vars#features-that-need-feature-flag-fetching) è disattivato                                                                                                                                                               | `default`                                     |
| La tua [prima sessione dopo aver installato Claude Code o aggiornato](/docs/it/env-vars#first-session-after-an-install-or-upgrade) a una versione che aggiunge questa impostazione predefinita, a meno che, dopo un'installazione pulita, Claude Code recuperi i flag in tempo | `default`                                     |
| `claude -p` o l'[Agent SDK](/docs/it/agent-sdk/permissions)                                                                                                                                                                                                                    | `default`                                     |
| Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, [Claude Platform su AWS](/docs/it/claude-platform-on-aws), o una sessione [gateway app Claude](/docs/it/claude-apps-gateway) con accesso                                                                          | `default`                                     |
| Un piano Pro, Max o Team, in un terminale o tramite l'[estensione VS Code](/docs/it/vs-code)                                                                                                                                                                                   | `auto`                                        |
| Un piano Enterprise o una chiave API Claude Console                                                                                                                                                                                                                       | `default`                                     |

Quando il recupero dei flag di funzionalità è disattivato, o in una [prima sessione dopo un'installazione o aggiornamento](/docs/it/env-vars#first-session-after-an-install-or-upgrade) dove i flag non sono ancora arrivati, l'estensione VS Code ignora ogni file di impostazioni quando sceglie la modalità di autorizzazione iniziale.

Quando il flag, un file di impostazioni, o l'impostazione predefinita integrata seleziona `auto` ma la modalità auto non è disponibile per la sessione, Claude Code avvia la sessione in Manual invece. La modalità auto non è disponibile quando la sessione non soddisfa i [requisiti di disponibilità](#eliminate-prompts-with-auto-mode), come un file di impostazioni che la disattiva o un modello che non la supporta, o quando Anthropic l'ha temporaneamente disattivata lato server.

La prima volta che l'impostazione predefinita integrata avvia una delle tue sessioni in modalità auto, Claude Code mostra un avviso che collega a questa pagina:

* In un terminale, una volta, in cima alla sessione
* Nell'estensione VS Code, come una scheda nella schermata di nuova conversazione che rimane finché non la chiudi

Sui piani Pro, Max e Team, se il tuo `~/.claude/settings.json` imposta un `defaultMode` diverso da `auto` e nessun altro file di impostazioni ne imposta uno, le tue sessioni continuano a iniziare in quella modalità. Claude Code chiede una volta, nel terminale o nell'estensione VS Code, se cambiare l'impostazione in modalità auto. Se rifiuti, la tua impostazione rimane come è.

<h3 id="start-in-a-different-mode">
  Inizia in una modalità di autorizzazione diversa
</h3>

Puoi impostare la modalità di autorizzazione iniziale per una sessione, o come impostazione predefinita per ogni sessione su una macchina, in un progetto, o in un'organizzazione. Quando più di un file di impostazioni imposta `permissions.defaultMode`, la [precedenza delle impostazioni](/docs/it/settings#settings-precedence) decide, quindi un valore di progetto o gestito supera `~/.claude/settings.json`. Per cambiare la modalità di autorizzazione di una sessione già in esecuzione, vedi [Cambia modalità di autorizzazione](#switch-permission-modes).

| Per impostare la modalità di autorizzazione iniziale per | Fai questo                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Una sessione che stai per avviare                        | Passa la modalità di autorizzazione come flag, ad esempio `claude --permission-mode default`                                                                                                                                                                                                                                                                                                                                                           |
| Ogni sessione di terminale che avvii su questa macchina  | Imposta `permissions.defaultMode` in `~/.claude/settings.json`. Per quello che l'estensione VS Code legge, vedi [Cambia modalità di autorizzazione](#switch-permission-modes)                                                                                                                                                                                                                                                                          |
| Ogni sessione di terminale che avvii in un progetto      | Imposta `permissions.defaultMode` nel `.claude/settings.json` del progetto. Le sessioni che avvii in un terminale rispettano ogni valore tranne `auto` e `bypassPermissions`; le sessioni che l'estensione VS Code avvia non leggono le impostazioni del progetto per la modalità di autorizzazione iniziale                                                                                                                                           |
| Ogni sessione di terminale nella tua organizzazione      | Imposta `permissions.defaultMode` nelle [impostazioni gestite](/docs/it/managed-settings). Le sessioni di terminale iniziano in quella modalità e le persone possono comunque passare alla modalità auto; per quello che l'estensione VS Code legge, vedi [Cambia modalità di autorizzazione](#switch-permission-modes). Per rimuovere la modalità auto in modo che nessuno possa selezionarla, imposta `permissions.disableAutoMode` su `"disable"` invece |

Questo esempio fa sì che ogni sessione di terminale sulla tua macchina inizi in modalità Manual, il cui valore di configurazione è `default`. Salvalo in `~/.claude/settings.json`:

```json theme={null}
{
  "permissions": {
    "defaultMode": "default"
  }
}
```

La prossima sessione che avvii mostra `⏸ manual mode on` nella barra di stato.

<h2 id="switch-permission-modes">
  Cambiare le modalità di autorizzazione
</h2>

Ogni interfaccia ha il proprio controllo per cambiare le modalità di autorizzazione durante una sessione e il proprio modo di scegliere la modalità di autorizzazione con cui iniziano le nuove sessioni. Seleziona la tua interfaccia per vedere i suoi controlli.

<Tabs>
  <Tab title="CLI">
    **Durante una sessione**: premi `Shift+Tab` per ciclo attraverso le modalità di autorizzazione. Da `auto`, il primo pressione passa a `default`, e il ciclo quindi esegue `default` → `acceptEdits` → `plan` → ritorno a `default`. Le modalità opzionali, descritte di seguito, si inseriscono dopo `plan`. La barra di stato mostra la modalità attiva come `⏸ manual mode on` grigio per `default`, oppure come `⏵⏵ accept edits on`, `⏸ plan mode on`, `⏵⏵ auto mode on`, `⏵⏵ don't ask on`, o `⏵⏵ bypass permissions on`.

    Non tutte le modalità sono nel ciclo predefinito:

    * `auto`: appare quando [auto mode è disponibile](#eliminate-prompts-with-auto-mode); il ciclo ad essa passa le modalità di autorizzazione senza un prompt di conferma
    * `bypassPermissions`: appare dopo che inizi con `--permission-mode bypassPermissions`, `--dangerously-skip-permissions`, `--allow-dangerously-skip-permissions`, o `permissions.defaultMode: "bypassPermissions"` in [impostazioni utente, `--settings`, o impostazioni gestite](/docs/it/settings-reference#permissions-defaultmode). La variante `--allow-` aggiunge la modalità di autorizzazione al ciclo senza attivarla
    * `dontAsk`: non appare mai nel ciclo; impostala con `--permission-mode dontAsk`

    Le modalità opzionali abilitate si inseriscono dopo `plan`, con `bypassPermissions` per primo e `auto` per ultimo. Se hai entrambe abilitate, ciclerai attraverso `bypassPermissions` sulla strada verso `auto`.

    **Da un prompt di autorizzazione Bash**: nelle modalità di autorizzazione Manual e `acceptEdits`, quando [auto mode](#eliminate-prompts-with-auto-mode) è disponibile, Claude Code aggiunge **Yes, and switch to auto mode** al prompt di autorizzazione di un comando Bash. Selezionalo per approvare il comando e passare la sessione alla modalità auto. I prompt dello [strumento PowerShell](/docs/it/tools-reference#powershell-tool) non offrono l'opzione. Richiede Claude Code v2.1.247 o successivo.

    Claude Code non aggiunge l'opzione ai prompt forzati da una delle tue [regole `ask`](/docs/it/permissions#manage-permissions) o da un [hook](/docs/it/hooks#pretooluse-decision-control), perché la modalità auto ti mostra comunque quei prompt, quindi il passaggio non li rimuoverebbe.

    **All'avvio**: passa la modalità di autorizzazione come flag.

    ```bash theme={null}
    claude --permission-mode plan
    ```

    **Come predefinito**: imposta `permissions.defaultMode` nell'ambito che desideri, come descritto in [Inizia in una modalità di autorizzazione diversa](#start-in-a-different-mode).

    Lo stesso flag `--permission-mode` funziona con `-p` per [esecuzioni non interattive](/docs/it/headless).
  </Tab>

  <Tab title="VS Code">
    **Durante una sessione**: fai clic sull'indicatore di modalità nella parte inferiore della casella del prompt. Utilizza queste etichette per le modalità in questa pagina:

    | Etichetta UI       | Modalità            |
    | :----------------- | :------------------ |
    | Manual             | `default`           |
    | Edit automatically | `acceptEdits`       |
    | Plan               | `plan`              |
    | Auto               | `auto`              |
    | Bypass permissions | `bypassPermissions` |

    **Come predefinito**: per fissare la modalità di autorizzazione con cui iniziano le conversazioni, imposta `claudeCode.initialPermissionMode` nelle impostazioni utente di VS Code su `default`, `manual`, `acceptEdits`, `plan`, o `bypassPermissions`. L'impostazione non accetta `auto`; per iniziare in Auto, lasciala non impostata e seleziona **Auto** dall'indicatore di modalità una volta, come descrive l'elemento 2 di seguito. L'estensione inizia ogni nuova conversazione nel primo di questi che si applica:

    1. `claudeCode.initialPermissionMode`
    2. La modalità che hai selezionato per ultimo dall'indicatore di modalità, se era Manual, Edit automatically, o Auto. Selezionare Plan o Bypass permissions si applica solo a quella conversazione
    3. `permissions.defaultMode` da [impostazioni gestite](/docs/it/managed-settings) o `~/.claude/settings.json`, su piani Pro, Max e Team con [feature-flag fetching](#which-mode-a-session-starts-in) disponibile
    4. Il [predefinito incorporato](#which-mode-a-session-starts-in) per il tuo piano, provider e impostazioni dell'organizzazione

    L'estensione non legge mai il `.claude/settings.json` o `.claude/settings.local.json` di un progetto per la modalità di autorizzazione iniziale, e nelle conversazioni che non soddisfano le condizioni dell'elemento 3 non legge alcun file di impostazioni. Quando `claudeCode.claudeProcessWrapper` è impostato, gli elementi 3 e 4 non si applicano nemmeno: quelle conversazioni iniziano in Manual a meno che l'elemento 1 o l'elemento 2 non imposti una modalità di autorizzazione.

    Auto appare nell'indicatore di modalità quando [auto mode è disponibile](#eliminate-prompts-with-auto-mode).

    Bypass permissions richiede l'interruttore **Allow dangerously skip permissions** nelle impostazioni dell'estensione. Senza di esso, la modalità di autorizzazione non appare nell'indicatore, e un valore `bypassPermissions` dall'elemento 1 o dall'elemento 3 inizia la conversazione in Manual invece. Auto da qualsiasi elemento allo stesso modo inizia la conversazione in Manual quando la modalità auto non è disponibile.

    Vedi la [guida VS Code](/docs/it/vs-code) per i dettagli specifici dell'estensione.
  </Tab>

  <Tab title="JetBrains">
    Il plugin JetBrains esegue Claude Code nel terminale dell'IDE, quindi il cambio delle modalità di autorizzazione funziona come nella CLI: premi `Shift+Tab` per ciclo, o passa `--permission-mode` al lancio.
  </Tab>

  <Tab title="Desktop">
    **Durante una sessione**: nella scheda Code, utilizza il selettore di modalità accanto al pulsante di invio. Non tutte le modalità appaiono nel selettore:

    * **Auto**: appare quando [auto mode è disponibile](#eliminate-prompts-with-auto-mode)
    * **Bypass permissions**: richiede l'interruttore **Allow bypass permissions mode** nelle impostazioni Desktop su piani Pro e Max; su piani Team e Enterprise, la politica dell'organizzazione la controlla invece

    La scheda Cowork non utilizza queste modalità. Cowork ha le sue proprie modalità di autorizzazione, abilitate separatamente, e la scheda Cowork non mostra alcun selettore di modalità fino a quando una modalità oltre il suo predefinito non è abilitata per il tuo account. Vedi la [documentazione Cowork](https://claude.com/docs/cowork/overview).

    Per i dettagli specifici del desktop, vedi [Scegli una modalità di autorizzazione](/docs/it/desktop#choose-a-permission-mode) nella guida Desktop.

    **Come predefinito**: imposta `defaultMode` in [impostazioni](/docs/it/settings#where-settings-live). L'app desktop legge gli stessi file di impostazioni della CLI e applica la modalità di autorizzazione alle nuove sessioni locali.

    Una modalità che scegli nel selettore di modalità viene ricordata per cartella e ha la precedenza su `defaultMode` per quella cartella. Plan è l'eccezione: selezionarla si applica solo alla sessione corrente.

    Per dove `defaultMode` va in un file di impostazioni, vedi l'esempio sotto [Inizia in una modalità di autorizzazione diversa](#start-in-a-different-mode).
  </Tab>

  <Tab title="Web and mobile">
    Utilizza il menu a discesa della modalità accanto alla casella del prompt su [claude.ai/code](https://claude.ai/code) o nell'app mobile. I prompt di autorizzazione appaiono in claude.ai per l'approvazione. Quali modalità appaiono dipende da dove viene eseguita la sessione:

    * **[Sessioni cloud](/docs/it/claude-code-on-the-web)**: Accept edits, Plan e Auto. Accept edits corrisponde alla modalità `default`: le sessioni cloud pre-approvano le modifiche ai file indipendentemente dalla modalità, quindi il menu a discesa mostra Accept edits invece di Manual. Le sessioni cloud rispettano comunque `defaultMode: "acceptEdits"` dalle impostazioni. La modalità Auto appare solo quando la tua organizzazione la consente e il modello selezionato la supporta. Bypass permissions non è disponibile.
    * **[Sessioni Remote Control](/docs/it/remote-control)** sulla tua macchina locale: Manual, Accept edits e Plan. Non puoi selezionare Auto o Bypass permissions dall'app.
      * Ad eccezione di Bypass permissions, il menu a discesa mostra la modalità di autorizzazione in cui si trova la sessione locale, inclusa una impostata dal terminale. Si aggiorna quando la modalità di autorizzazione cambia nell'app o nel terminale. La sessione non segnala mai Bypass permissions a claude.ai, quindi il passaggio ad essa dal terminale non cambia quello che il menu a discesa mostra.
      * Le sessioni ospitate dall'[app desktop](/docs/it/desktop) o dall'[estensione VS Code](/docs/it/vs-code) segnalano i cambiamenti della modalità di autorizzazione a claude.ai mentre accadono, come le sessioni ospitate in un terminale.
      * Prima di v2.1.202, le sessioni connesse con `/remote-control` o `claude --remote-control` non segnalarono affatto la loro modalità di autorizzazione, quindi claude.ai e l'app mobile potevano mostrare una modalità di autorizzazione in cui la sessione non era. La discrepanza ha interessato solo l'etichetta. Claude Code ha generato prompt di autorizzazione dalla modalità di autorizzazione effettiva della sessione, e hanno comunque apparso nell'app per l'approvazione.

    Per Remote Control, la macchina locale che esegue la sessione deve essere connessa con il tuo account claude.ai; le chiavi API non sono supportate. Puoi anche impostare la modalità di autorizzazione iniziale al lancio di quella sessione locale:

    ```bash theme={null}
    claude remote-control --permission-mode acceptEdits
    ```
  </Tab>
</Tabs>

<h2 id="auto-approve-file-edits-with-acceptedits-mode">
  Auto-approva le modifiche ai file con la modalità acceptEdits
</h2>

La modalità `acceptEdits` consente a Claude di creare e modificare file nella tua directory di lavoro senza richiedere conferma. La barra di stato mostra `⏵⏵ accept edits on` mentre questa modalità è attiva.

Oltre alle modifiche ai file, la modalità `acceptEdits` auto-approva i comuni comandi Bash del filesystem: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp` e `sed`. Questi comandi vengono anche auto-approvati quando preceduti da variabili di ambiente sicure come `LANG=C` o `NO_COLOR=1`, o da wrapper di processi come `timeout`, `nice` o `nohup`. Come per le modifiche ai file, l'auto-approvazione si applica solo ai percorsi all'interno della tua directory di lavoro o `additionalDirectories`. I percorsi al di fuori di questo ambito, le scritture su [percorsi protetti](#protected-paths), le rimozioni `rm` e `rmdir` che prendono di mira un [percorso critico](#critical-paths), e tutti gli altri comandi Bash tranne il [set di sola lettura integrato](/docs/it/permissions#read-only-commands) richiedono comunque conferma.

Quando lo [strumento PowerShell](/docs/it/tools-reference#powershell-tool) è abilitato, la modalità `acceptEdits` auto-approva anche `Set-Content`, `Add-Content`, `Clear-Content` e `Remove-Item` su percorsi nell'ambito, insieme ai loro alias comuni. Si applicano le stesse regole di ambito e percorsi protetti, e `Remove-Item` ottiene [il suo proprio controllo](#remove-item-in-powershell). Un argomento posizionale che contiene un carattere di virgolette, come l'apostrofo in `Set-Content .\notes.txt "It's done"`, richiede comunque conferma anche su percorsi nell'ambito, perché Claude Code non può convalidare staticamente un argomento le cui letture tra virgolette e non virgolette differiscono. Passa il contenuto attraverso un parametro denominato come `-Value` per evitare il prompt.

Utilizza `acceptEdits` quando desideri revisionare le modifiche nel tuo editor o tramite `git diff` successivamente, piuttosto che approvare ogni modifica inline.

Premi `Shift+Tab` una volta dalla modalità Manual per accedervi, o inizia direttamente con essa:

```bash theme={null}
claude --permission-mode acceptEdits
```

<h2 id="analyze-before-you-edit-with-plan-mode">
  Analizza prima di modificare con la modalità plan
</h2>

La modalità plan dice a Claude di ricercare e proporre modifiche senza apportarle. Claude legge i file, esegue comandi shell per esplorare e scrive un piano, ma non modifica il tuo codice sorgente. Ad eccezione delle sessioni con [autorizzazioni di bypass disponibili](#skip-all-checks-with-bypasspermissions-mode), le modifiche rimangono bloccate finché non approvi il piano.

Quando [la modalità auto](/docs/it/auto-mode-config) è disponibile e l'impostazione `useAutoModeDuringPlan` è attiva, che è l'impostazione predefinita, il classificatore esamina i comandi shell durante la pianificazione invece di richiedere conferma. I comandi approvati vengono eseguiti, e quelli rifiutati vengono bloccati. Altrimenti, i comandi al di fuori del [set di sola lettura integrato](/docs/it/permissions#read-only-commands) richiedono approvazione, incluso quando la modalità [auto-allow](/docs/it/sandboxing#sandbox-modes) della sandbox è abilitata. Nelle sessioni con autorizzazioni di bypass disponibili, né il classificatore né un prompt si applica ai comandi di pianificazione; [Ignora tutti i controlli con la modalità bypassPermissions](#skip-all-checks-with-bypasspermissions-mode) copre le poche cose che richiedono comunque prompt lì. Nella v2.1.212 attraverso v2.1.217, le sessioni senza autorizzazioni di bypass richiedevano conferma per ogni comando al di fuori del set di sola lettura, indipendentemente dal fatto che la modalità auto fosse disponibile.

Accedi alla modalità plan premendo `Shift+Tab` o anteponendo un singolo prompt con `/plan`. Puoi anche iniziare in modalità plan dalla CLI:

```bash theme={null}
claude --permission-mode plan
```

Premi `Shift+Tab` di nuovo per uscire dalla modalità plan senza approvare un piano.

<h3 id="review-and-approve-a-plan">
  Rivedi e approva un piano
</h3>

Quando il piano è pronto, Claude lo presenta e chiede come procedere. Da quel prompt puoi scegliere:

* **Sì, e utilizza la modalità auto**: approva e inizia in [modalità auto](#eliminate-prompts-with-auto-mode). Se la modalità auto non è [disponibile per la tua sessione](#eliminate-prompts-with-auto-mode), ad esempio perché la tua organizzazione l'ha disattivata, questa opzione legge **Sì, auto-accetta le modifiche**. Se hai avviato la sessione con le autorizzazioni di bypass abilitate, l'opzione legge **Sì, e passa a BYPASS PERMISSIONS (nessun ulteriore prompt) per questa sessione** invece.
* **Sì, approva manualmente le modifiche**: approva e rivedi ogni modifica individualmente.
* **No, continua a pianificare**: rimani in modalità plan e dì a Claude cosa cambiare.

Approvare un piano esce dalla modalità plan e passa la sessione alla modalità di autorizzazione che ogni opzione di approvazione descrive, quindi Claude inizia a modificare. Per pianificare di nuovo, cicla di nuovo alla modalità plan con `Shift+Tab`, o anteponi il tuo prossimo prompt con `/plan`.

Premi `Ctrl+G` per aprire il piano proposto nel tuo editor di testo predefinito e modificarlo direttamente prima che Claude proceda. Quando [`showClearContextOnPlanAccept`](/docs/it/settings-reference#showclearcontextonplanaccept) è abilitato, l'elenco guadagna una prima opzione che approva il piano e cancella il contesto di pianificazione.

Approvare un piano dà anche alla sessione un [titolo generato](/docs/it/sessions#name-your-sessions) basato sul piano, a meno che tu non abbia già nominato la sessione.

<h3 id="set-plan-mode-as-the-default">
  Imposta la modalità plan come impostazione predefinita
</h3>

Per rendere la modalità plan l'impostazione predefinita per le sessioni di terminale di un progetto, imposta `defaultMode` su `plan` in `.claude/settings.json`, posizionato come l'esempio sotto [Inizia in una modalità di autorizzazione diversa](#start-in-a-different-mode) mostra. Le conversazioni che l'[estensione VS Code](/docs/it/vs-code) avvia non leggono le impostazioni del progetto per la modalità di autorizzazione iniziale. Lì, imposta `claudeCode.initialPermissionMode` su `plan` nelle tue impostazioni utente di VS Code invece.

<h2 id="eliminate-prompts-with-auto-mode">
  Elimina i prompt di autorizzazione con la modalità auto
</h2>

La modalità auto consente a Claude di eseguire senza prompt di autorizzazione di routine. Un modello classificatore separato esamina le azioni prima che vengano eseguite, bloccando qualsiasi cosa che vada oltre la vostra richiesta, che prenda di mira un'infrastruttura non riconosciuta, o che sembri guidata da contenuti ostili che Claude ha letto. Le [regole di richiesta](/docs/it/permissions#manage-permissions) esplicite forzano comunque un prompt.

Nei piani Pro, Max e Team, la modalità auto è la [modalità di autorizzazione predefinita della sessione](#which-mode-a-session-starts-in).

Il classificatore esamina anche ogni messaggio che Claude invia a un altro agente con [`SendMessage`](/docs/it/tools-reference), sia testo semplice che un messaggio strutturato di [team di agenti](/docs/it/agent-teams), prima che Claude Code lo consegni, sia in modalità auto che in [modalità piano mentre il classificatore esamina i comandi](#analyze-before-you-edit-with-plan-mode); la revisione dell'invio richiede Claude Code v2.1.222 o successivo.

Il classificatore esamina e approva o blocca anche le rimozioni `rm` e `rmdir` che prendono di mira un [percorso critico](#critical-paths), come `rm -rf /` e `rm -rf ~`, incluso quando la rimozione si trova all'interno della sostituzione di comando o processo.

La modalità auto incoraggia anche Claude a continuare a lavorare senza fermarsi per domande di chiarimento, anche se Claude chiede comunque quando la vostra richiesta o un'abilità si basa esplicitamente su di essa. Per un comportamento più autonomo in una modalità che vi chiede comunque, impostate invece lo [stile di output proattivo](/docs/it/output-styles).

<Warning>
  La modalità auto riduce i prompt di autorizzazione ma non garantisce la sicurezza. Utilizzatela per attività in cui fidate della direzione generale, non come sostituto della revisione su operazioni sensibili.
</Warning>

La modalità auto è disponibile solo quando il vostro account soddisfa tutti questi requisiti:

* **Piano**: Tutti i piani.
* **Organizzazione**: su Team ed Enterprise, la modalità auto è disponibile per impostazione predefinita. Gli amministratori possono disattivarla per l'organizzazione impostando `permissions.disableAutoMode` su `"disable"` nelle [impostazioni gestite](/docs/it/managed-settings).
* **Modello**: sull'API Anthropic e su [Claude Platform su AWS](/docs/it/claude-platform-on-aws), Claude Opus 4.6 o successivo, Sonnet 4.6 o successivo, o un [modello Fable](/docs/it/model-config#work-with-fable). Su Amazon Bedrock, Agent Platform di Google Cloud, Microsoft Foundry e sessioni [gateway di app Claude](/docs/it/claude-apps-gateway) con accesso, solo Claude Sonnet 5, Opus 4.7 o successivo e i modelli Fable. I modelli più vecchi, inclusi Sonnet 4.5, Opus 4.5, Haiku e modelli claude-3, non sono supportati su nessun provider.
* **Provider**: disponibile per impostazione predefinita sull'API Anthropic, Claude Platform su AWS, Amazon Bedrock, Agent Platform di Google Cloud, Microsoft Foundry e sessioni gateway di app Claude con accesso.

Se Claude Code segnala che la modalità auto non è disponibile, controllate prima questi requisiti e se un file di impostazioni imposta [`disableAutoMode`](/docs/it/settings-reference#disableautomode). Anthropic potrebbe anche aver disattivato la modalità auto lato server, oppure il server potrebbe aver rifiutato la modalità auto per il vostro account. Una sessione che ha ricevuto una di queste risposte mantiene la modalità auto disattivata fino al termine della sessione, quindi avviate una nuova sessione in seguito.

Un messaggio separato che nomina un modello e dice che la modalità auto "non può determinare la sicurezza" di un'azione significa che una richiesta del classificatore non è riuscita. Questo errore è solitamente transitorio, ma su Amazon Bedrock può ripetersi fino a quando il vostro account non può invocare il modello denominato. Consultate il [riferimento degli errori](/docs/it/errors#auto-mode-cannot-determine-the-safety-of-an-action) per le cause e cosa fare.

Se impostate `defaultMode: "auto"` nelle [impostazioni](/docs/it/settings-reference#all-settings) e una sessione di terminale inizia in modalità Manuale senza errore, l'impostazione è probabilmente in `.claude/settings.json` o `.claude/settings.local.json`. `auto` non ha effetto da questi file. Spostatela in `~/.claude/settings.json`. Per una conversazione avviata dall'estensione VS Code, controllate invece l'elenco proprio dell'estensione in [Cambia modalità di autorizzazione](#switch-permission-modes).

<h3 id="enable-auto-mode-on-bedrock-agent-platform-or-foundry">
  Modalità auto su Bedrock, Agent Platform o Foundry
</h3>

Su [Amazon Bedrock](/docs/it/amazon-bedrock), [Agent Platform di Google Cloud](/docs/it/google-vertex-ai), [Microsoft Foundry](/docs/it/microsoft-foundry) e sessioni [gateway di app Claude](/docs/it/claude-apps-gateway) con accesso, la modalità auto appare nel ciclo `Shift+Tab` per impostazione predefinita. L'apparizione nel ciclo non cambia la modalità di autorizzazione in cui una sessione inizia: su questi provider, le sessioni di terminale iniziano nella vostra [`defaultMode`](/docs/it/settings-reference#permissions-defaultmode), che è Manuale a meno che non la cambiate, e le conversazioni nell'[estensione VS Code](/docs/it/vs-code) iniziano in Manuale a meno che `claudeCode.initialPermissionMode` o una modalità che avete scelto nell'estensione ne imposti una. Solo Claude Sonnet 5, Opus 4.7 o successivo e i modelli Fable sono supportati su questi provider.

Per rendere la modalità auto la modalità di autorizzazione predefinita all'avvio, impostate `"permissions": {"defaultMode": "auto"}` nelle impostazioni utente o gestite. Nelle sessioni avviate dall'estensione VS Code, selezionate invece **Auto** dall'indicatore di modalità. [Cambia modalità di autorizzazione](#switch-permission-modes) copre cosa ha la precedenza su quella scelta.

Il checkup [`/doctor`](/docs/it/commands#all-commands) propone questa impostazione predefinita delle impostazioni utente su questi provider nello stesso modo in cui lo fa sull'API Anthropic.

Per impedire agli sviluppatori di utilizzare la modalità auto, impostate `disableAutoMode` su `"disable"` nelle [impostazioni gestite](/docs/it/managed-settings). Questo rimuove `auto` dal ciclo `Shift+Tab` e una sessione avviata con `--permission-mode auto` inizia in Manuale. Una sessione già in esecuzione in modalità auto la abbandona quando l'impostazione la raggiunge da un'[origine distribuita da amministratore](/docs/it/managed-settings#which-managed-source-claude-code-uses) e mostra `auto mode disabled by settings`. Prima della v2.1.251, una sessione in esecuzione manteneva la modalità auto fino al termine.

Nella v2.1.158 fino alla v2.1.206, la modalità auto era disattivata su questi provider fino a quando non impostavate `CLAUDE_CODE_ENABLE_AUTO_MODE=1` e Claude Code ignorava `defaultMode: "auto"` su questi provider a meno che la variabile non fosse anche impostata. La variabile è ancora accettata per compatibilità e non ha effetto dalla v2.1.207 in poi.

<h3 id="server-side-classifier-review">
  Revisione del classificatore lato server
</h3>

In modalità auto, Claude Code può chiedere al server di controllare le azioni che l'[ordine decisionale](#how-the-classifier-evaluates-actions) invia per la revisione, come parte delle richieste del modello della sessione, al posto di inviare le proprie richieste del classificatore. Queste sessioni chiedono:

* **Una connessione diretta all'API Anthropic**: in una sessione di terminale interattiva, su ogni piano claude.ai e su account che utilizzano l'API Claude, mentre Anthropic lo implementa. Richiede Claude Code v2.1.271 o successivo su piani Pro, Max e Team, e v2.1.278 o successivo su piani Enterprise e account API Claude. Dalla v2.1.282, una sessione che [non recupera i flag di funzionalità](/docs/it/env-vars#features-that-need-feature-flag-fetching), ad esempio perché avete disattivato la telemetria, chiede il server per impostazione predefinita in qualsiasi tipo di sessione.
* **Un provider cloud, un gateway LLM o un proxy**: su [Claude Platform su AWS](/docs/it/claude-platform-on-aws), Amazon Bedrock, Agent Platform di Google Cloud e Microsoft Foundry, e ogni volta che puntate `ANTHROPIC_BASE_URL` a un [gateway LLM o proxy](/docs/it/llm-gateway), indipendentemente dal vostro piano. Chiedere per impostazione predefinita richiede Claude Code v2.1.278 o successivo.
* **Una sessione [gateway di app Claude](/docs/it/claude-apps-gateway) con accesso**: richiede Claude Code v2.1.280 o successivo

Dove il server esamina le azioni, i suoi verdetti le decidono. Due altri risultati sono possibili:

* **Il server non esamina la sessione**: una risposta si completa senza risultati di revisione, oppure il server risponde che non esamina questa sessione. Le cause più comuni sono un gateway LLM o un proxy che scarta la richiesta di revisione o i risultati, e una piattaforma, una regione o una credenziale che non ha ancora controlli lato server. Claude Code ricade alle proprie richieste del classificatore. Una volta che questo fallback vale per il resto della sessione, mostra un [avviso sugli addebiti delle richieste del classificatore](/docs/it/auto-mode-classifier-billing) su account in cui queste richieste vengono fatturate.
* **Il server non fornisce un verdetto per un'azione**: Claude Code nega l'azione piuttosto che eseguirla senza revisione. Su qualsiasi connessione, questo accade quando la risposta termina prima che i risultati della revisione arrivino o i risultati arrivano in una forma che Claude Code non può leggere. Un gateway LLM o un proxy che taglia le risposte corte o riscrive i risultati può causare entrambi. Su una connessione diretta all'API Anthropic, accade anche quando il controllo del server non riesce per l'azione, ad esempio per timeout. [Il server non ha restituito un verdetto di sicurezza](/docs/it/errors#the-server-returned-no-safety-verdict) copre il messaggio di negazione, cosa accade quando le negazioni si ripetono e cosa fare.

Per saltare la richiesta al server e utilizzare sempre le proprie richieste del classificatore di Claude Code, impostate [`CLAUDE_CODE_AUTO_MODE_SERVER=0`](/docs/it/env-vars). Su una connessione diretta all'API Anthropic, la variabile richiede Claude Code v2.1.281 o successivo. Impostarla su `1` lì attiva la revisione del server in una sessione che non ce l'ha ancora, come una sessione `-p` o Agent SDK, a meno che non abbiate anche impostato `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`. Se impostate `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` e lasciate `CLAUDE_CODE_AUTO_MODE_SERVER` non impostato, Claude Code smette anche di chiedere al server.

<h3 id="what-the-classifier-blocks-by-default">
  Cosa blocca il classificatore per impostazione predefinita
</h3>

Il classificatore si fida della vostra directory di lavoro e dei remoti che erano configurati per essa quando la sessione è iniziata. Un remote aggiunto o reindirizzato durante la sessione con `git remote add` o `git remote set-url` non è attendibile e tutto il resto è trattato come esterno fino a quando non [configurate l'infrastruttura attendibile](/docs/it/auto-mode-config). Prima della v2.1.200, i remoti aggiunti a metà sessione erano anche attendibili.

**Bloccato per impostazione predefinita**:

* Download ed esecuzione di codice, come `curl | bash`
* Invio di dati sensibili a endpoint esterni
* Deploy e migrazioni di produzione
* Eliminazione di massa su archiviazione cloud
* Concessione di autorizzazioni IAM o repo
* Modifica dell'infrastruttura condivisa
* Distruzione irreversibile di file che esistevano prima della sessione
* Force push
* Commit o push di una modifica che invierebbe segreti o dati sensibili al di fuori del repository quando viene eseguito, o amplierebbe ciò che un deploy espone. Questo copre un flusso di lavoro CI o una configurazione di deploy che passa un segreto a una destinazione che non lo riceve già, uno script o un passaggio di configurazione che legge un archivio di segreti e invia i dati fuori, e una modifica di configurazione che amplia ciò che un deploy pubblica, come un'impostazione di registro, visibilità, artefatto o sourcemap. Il controllo si applica su qualsiasi ramo, si applica anche quando il repository è pubblico e si attiva quando la modifica viene sottoposta a commit o push, indipendentemente dal fatto che quel commit o push attivi la pipeline; cancellarlo richiede di nominare l'effetto di esecuzione, non solo il commit o push. Prima della v2.1.211, questo controllo era limitato al ramo predefinito: un push lì veniva bloccato quando conteneva contenuto sensibile, modifiche nascoste o descritte in modo errato rispetto a quello che avete chiesto, contenuto portato da fuori il repository o instradato intorno a una revisione che avete chiesto
* `git reset --hard`, `git checkout -- .`, `git restore .`, `git clean -fd`, `git stash drop` o `git stash clear`, che il classificatore presume scarterebbero le modifiche non sottoposte a commit
* `git commit --amend` quando il commit in HEAD non è stato creato in questa sessione
* Dalla v2.1.198, `git commit --amend` quando il commit in HEAD è già stato sottoposto a push. Un reword solo messaggio non è bloccato: `--amend -m` senza nulla di nuovo in staging, su un commit che Claude ha creato durante questa sessione
* `terraform destroy`, `pulumi destroy`, `cdk destroy` o `terragrunt destroy`, e applicazione di un piano che distrugge risorse

Claude Code v2.1.195 e successivo bloccano più categorie per impostazione predefinita. Diversi dipendono da voci di [ambiente](/docs/it/auto-mode-config#define-trusted-infrastructure), come target remoti sensibili e ambiti IaC protetti, che potete restringere a nomi concreti.

* Scrittura in un gestore di segreti, o modifica di record DNS o certificati TLS
* Unione di una pull request che nessun umano ha approvato, approvazione della propria pull request di Claude o disabilitazione dei controlli CI
* Posting di un commento che è esso stesso un comando per l'automazione, come `atlantis apply` o il `/deploy` o `/merge` di un bot
* Attivazione, ramping o eliminazione di un flag di funzionalità di produzione
* Applicazione di modifiche all'infrastruttura a un ambito IaC protetto, o drenaggio e rimozione di nodi del cluster
* Scritture in un cluster di calcolo condiviso che vanno oltre la risorsa che avete nominato, come un selettore di etichetta o `--all` che cattura i lavori di altri utenti
* Creazione di risorse Kubernetes che vengono eseguite su ogni nodo o intercettano il traffico del cluster, come DaemonSets e webhook di ammissione
* Shell interattive o port-forward in un target remoto sensibile
* Apertura di un tunnel o shell inversa che rende un servizio locale raggiungibile da Internet pubblico
* Stampa di una credenziale o token live nella trascrizione o in un file
* Accesso a una posizione elencata come posizione di dati sensibili nel vostro [ambiente](/docs/it/auto-mode-config#define-trusted-infrastructure), o copia di dati da una. A partire dalla v2.1.198 questo blocca anche l'invio di dati da uno a un pubblico che la voce esclude
* Instradamento di un'installazione di pacchetto intorno al vostro registro di pacchetti interno a un registro pubblico. A partire dalla v2.1.198, questo si applica anche quando avete detto a Claude che un registro interno o uno specchio esiste nella conversazione, non solo quando uno è elencato nel vostro ambiente
* Esecuzione di un comando con un flag che disarma una guardia di sicurezza, come `--insecure`
* Avvio di un ciclo di agente autonomo che viene eseguito senza approvazione umana o sandbox, come uno avviato con `--dangerously-skip-permissions` o `--no-sandbox`. A partire dalla v2.1.198 questo copre anche l'esecuzione di un agente di terze parti o di un harness di valutazione con isolamento e approvazione per azione disabilitati, come un runner avviato con `--yes-always`
* Azioni del browser [Claude in Chrome](/docs/it/chrome) che potrebbero inviare contenuto della pagina, cookie o credenziali off-origin

Claude Code v2.1.198 e successivo bloccano anche questi per impostazione predefinita:

* Eliminazione di file in `/tmp`, `$TMPDIR` o un'altra directory di scratch o cache condivisa per wildcard, glob o filtro di età piuttosto che per un percorso nominato specifico
* Inclusione di dettagli sensibili nel contenuto inviato, caricato, pubblicato o scritto ad altre persone o sistemi condivisi, quando il vostro messaggio non ha autorizzato quei dettagli per quel destinatario. I corpi di PR e issue, i messaggi di commit e i commenti contano come questo tipo di contenuto in uscita quando il repository è al di fuori del confine di fiducia o pubblico, inclusi i vostri repository pubblici dell'organizzazione; i percorsi di file interni, i nomi in codice, i dati di risposta API live come email o identificatori di account e gli identificatori di infrastruttura contano come dettagli sensibili. Lo scoping di PR, issue e messaggio di commit richiede Claude Code v2.1.200 o successivo. I dati personali live da una risposta API in un corpo di PR o issue, come un indirizzo email, un identificatore di account o organizzazione o una metrica di utilizzo, richiedono che nominiate quei dettagli e il destinatario indipendentemente dalla visibilità del repository o dal confine di fiducia. Quel controllo richiede Claude Code v2.1.203 o successivo
* Invio di pressioni di tasti al proprio riquadro tmux di Claude Code per guidare la propria interfaccia, che il classificatore tratta come Claude che cambia le proprie autorizzazioni o supervisione

Claude Code v2.1.200 e successivo bloccano anche questi per impostazione predefinita:

* Commento, eliminazione o superamento forzato di un test o asserzione che protegge il comportamento di sicurezza, come autenticazione, controllo di accesso, convalida dell'input o sandboxing
* Eliminazione o smantellamento di una risorsa stateful che Claude non ha creato nella sessione, quando nessuna regola di eliminazione più specifica si applica e non avete nominato quella risorsa
* Reindirizzamento di un URL di base API, endpoint proxy, ricevitore webhook o specchio di registro a un host di terze parti che non si adatta al compito, incluso nei file di esempio come `.env.example`
* Modifica di dove vanno i push con `git remote set-url` o `git remote add`, a meno che non abbiate nominato il nuovo remote
* Push di segreti o dati personali o affidati a un repository noto per essere pubblico, o push di materiale confidenziale lì che non fa parte del lavoro proprio di quel repository. Il soggetto proprio di un repository di dotfiles è l'unica eccezione per dati personali o affidati, e il contenuto da un repository privato che raggiunge qualsiasi superficie pubblica è bloccato nello stesso modo; entrambi i perfezionamenti richiedono Claude Code v2.1.203 o successivo. Prima della v2.1.203, i dati personali erano raggruppati con materiale confidenziale e bloccati solo quando non facevano parte del lavoro proprio di quel repository. Quando la visibilità di un repository non è stabilita, il classificatore non blocca solo su quello; giudica il contenuto rispetto alle altre regole
* Apertura di una pull request contro un repository o organizzazione diversa, fork con `gh repo fork` o push a un repository di terze parti, a meno che non abbiate nominato quel target esterno

Claude Code v2.1.203 e successivo bloccano anche questi per impostazione predefinita:

* Contenuto da un archivio locale sensibile, o da un file il cui nome, percorso o tipo lo contrassegna come sensibile, che entra in un commit, un push, testo di PR o issue, un gist o paste, o una pubblicazione di pacchetto, a meno che non abbiate nominato sia la fonte che la destinazione. Le trascrizioni di sessione e i log di conversazione, le cartelle di punti di configurazione e credenziali come chiavi SSH, credenziali cloud, profili del browser e cronologia della shell, e gli export di dati utente contano tutti, e il fatto che il repository sia privato non lo cancella

Claude Code v2.1.205 e successivo bloccano anche questi per impostazione predefinita:

* Scrittura nelle trascrizioni di sessione di Claude Code, i file di cronologia `.jsonl` sotto `~/.claude/projects/` o la vostra directory di configurazione configurata, direttamente o tramite un comando di shell. La regola copre anche le righe di metadati che Claude Code aggiunge a ogni voce di trascrizione per i propri controlli. La lettura di una trascrizione non è bloccata
* Un'eliminazione forzata ricorsiva come `rm -rf "$VAR"` o `Remove-Item -Recurse -Force $dir` il cui target è una variabile di shell, o un glob radicato in una, che non è assegnato da nessuna parte nella conversazione che il classificatore vede. Il valore proveniva solo dall'output di comando precedente, che il classificatore non riceve mai, quindi il classificatore non può verificare il target di eliminazione rispetto alle altre regole di eliminazione. Il blocco si cancella quando nominate il percorso esatto che viene eliminato, o quando Claude riesegue l'eliminazione con il percorso letterale risolto scritto nel comando. Le eliminazioni il cui target il classificatore può risolvere non sono interessate. I target `Remove-Item` che sono un `*` nudo o terminano in `/*` o `\*` non raggiungono mai il classificatore: Claude Code [li nega direttamente](#remove-item-in-powershell)

Claude Code v2.1.257 e successivo bloccano anche questi per impostazione predefinita:

* Richiesta di credenziali dall'endpoint dei metadati dell'istanza cloud, come `169.254.169.254`, o autenticazione esplicita di una chiamata cloud, cluster o registro con l'identità dell'account di servizio o del nodo della macchina
* Raggiungimento di un host pubblico per una rotta diversa da una richiesta diretta, come un tunnel, una shell inversa, o una configurazione di resolver o proxy riscritta per puntare al di fuori
* Lettura di credenziali che appartengono all'host piuttosto che al vostro compito, come certificati di nodo o l'auth del registro di container del nodo
* Connessione a o scansione di container, pod o VM fratelli che Claude non ha avviato, o il nodo sotto il container

Se Claude Code viene eseguito da qualche parte che intende consentire uno di questi, descrivete quella configurazione in una voce [Host containment](/docs/it/auto-mode-config#define-trusted-infrastructure) in `autoMode.environment`.

Claude Code v2.1.261 e successivo bloccano anche questi per impostazione predefinita:

* Posting o scrittura di un link a un servizio pubblico di paste, diagramma o condivisione dati in un messaggio, testo di PR o issue, un documento, o ovunque il link verrà aperto o recuperato, quando l'URL stesso contiene il contenuto condiviso, a meno che non abbiate nominato quel servizio

**Consentito per impostazione predefinita**:

* Operazioni su file locali nella vostra directory di lavoro
* Installazione di dipendenze dichiarate nei vostri file di lock o manifest
* Lettura di `.env` e invio di credenziali al loro API corrispondente
* Richieste HTTP di sola lettura
* Push a qualsiasi ramo del repository in cui state lavorando, incluso il ramo predefinito. Un ramo non predefinito il cui nome lo contrassegna come target di deploy o pubblicazione, come `production` o `gh-pages`, non è coperto: il classificatore giudica un push lì secondo i suoi termini. Il contenuto del push è ancora controllato rispetto alle altre regole, le regole [`permissions.deny`](/docs/it/permissions#manage-permissions) possono ancora bloccare i comandi push [come scritti](/docs/it/permissions#bash-rule-limits) in ogni modalità, e la protezione del ramo del remote si applica ancora. Prima della v2.1.211, solo i push al ramo su cui avete iniziato, i rami che Claude ha creato e i push di routine al ramo predefinito erano consentiti per impostazione predefinita, e prima della v2.1.203 qualsiasi push diretto al ramo predefinito era bloccato

Claude Code v2.1.195 e successivo consentono anche questi per impostazione predefinita:

* Eliminazione dei lavori esatti che Claude ha creato in precedenza nella stessa sessione
* Lettura, revisione o scrittura di codice, configurazioni e modelli di minaccia relativi alla sicurezza come parte del vostro compito
* Messaggi tra agenti che lavorano insieme nella stessa sessione multi-agente
* Invio di dati ai domini attendibili, bucket e servizi che elencate in [`environment`](/docs/it/auto-mode-config#define-trusted-infrastructure). Questo copre il flusso di dati solo, non operazioni distruttive o di credenziale sulla stessa infrastruttura
* [Claude in Chrome](/docs/it/chrome) navigazione a un dominio interno attendibile, localhost o un URL che avete nominato

I comandi in sandbox non ottengono accesso di rete per impostazione predefinita. Claude nomina gli host che un comando necessita sul comando stesso, il classificatore li esamina con il comando, e un elenco approvato apre quegli host per quel solo comando. [Domini consentiti per comando](/docs/it/sandboxing#per-command-allowed-domains-in-auto-mode) copre cosa un elenco può e non può aprire e cosa accade quando un comando raggiunge per un host non elencato.

Eseguite `claude auto-mode defaults` per stampare gli elenchi di regole completi come JSON. Se le azioni di routine vengono bloccate, un amministratore può aggiungere repo, bucket e servizi attendibili tramite l'impostazione `autoMode.environment`: consultate [Configura modalità auto](/docs/it/auto-mode-config).

Il push a qualsiasi ramo del repository in cui state lavorando e la creazione di una pull request che corrisponde alla vostra richiesta vengono eseguiti senza un prompt, a meno che il push o la pull request non rientri nell'[elenco bloccato](#what-the-classifier-blocks-by-default), come segreti o dati sensibili che lasciano il repository, o una pull request che prende di mira un repository o organizzazione diversa. Per richiedere un checkpoint umano prima di questi comandi mentre rimanete in modalità auto, aggiungete regole `permissions.ask`, che corrispondono al comando [come scritto](/docs/it/permissions#bash-rule-limits): consultate [Confini comuni](/docs/it/auto-mode-config#common-boundaries).

<h3 id="first-read-outside-the-working-directories">
  La prima lettura al di fuori delle directory di lavoro
</h3>

Mentre [`permissions.blockReadsOutsideWorkingDirectories`](/docs/it/settings-reference#permissions-blockreadsoutsideworkingdirectories) è disattivato, le letture di file vengono eseguite senza un prompt in modalità auto, incluse le letture al di fuori delle [directory di lavoro](/docs/it/permissions#working-directories). La prima volta che Claude utilizza lo strumento Read, Grep o Glob su un percorso al di fuori di esse, Claude Code vi chiede se continuare a consentire quelle letture.

Il prompt non appare nelle esecuzioni `-p` non interattive o nelle sessioni in background; le letture lì vengono eseguite come prima.

Qualunque sia la vostra risposta, Claude continua a lavorare:

* **Continua a consentire**: la lettura viene eseguita, le letture successive al di fuori delle directory di lavoro vengono eseguite come prima, e Claude Code registra la vostra risposta in modo che il prompt non appaia di nuovo
* **Blocca da ora in poi**: la lettura viene rifiutata e Claude Code imposta [`permissions.blockReadsOutsideWorkingDirectories`](/docs/it/settings-reference#permissions-blockreadsoutsideworkingdirectories) su `true` nelle vostre impostazioni utente, il che fa sì che gli strumenti di file rifiutino tali letture in ogni sessione successiva e in ogni modalità di autorizzazione. Per consentire a Claude di leggere tale percorso in seguito, aggiungete la sua directory con `/add-dir` o rimuovete l'impostazione.
* **Chiedi di nuovo la prossima volta**: la lettura viene rifiutata e la prossima lettura al di fuori delle directory di lavoro chiede di nuovo

<h3 id="boundaries-you-state-in-conversation">
  Confini che dichiarate nella conversazione
</h3>

Il classificatore tratta i confini che dichiarate nella conversazione come un segnale di blocco. Se dite a Claude "non fare push" o "aspetta finché non rivedo prima di fare il deploy", il classificatore blocca le azioni corrispondenti anche quando le regole predefinite le consentirebbero. Un confine rimane in vigore fino a quando non lo sollevate in un messaggio successivo. Il giudizio proprio di Claude che una condizione è stata soddisfatta non lo solleva.

I confini non vengono archiviati come regole. Il classificatore li rilegge dalla trascrizione su ogni controllo, quindi un confine può andare perso se la [compattazione del contesto](/docs/it/costs#reduce-token-usage) rimuove il messaggio che lo ha dichiarato. Per una garanzia dura, aggiungete invece una [regola di negazione](/docs/it/permissions#permission-rule-syntax).

<h3 id="approvals-you-state-in-conversation">
  Approvazioni che dichiarate nella conversazione
</h3>

Se dite a Claude che un'azione bloccata è consentita, il classificatore legge questo come la vostra approvazione e può cancellare il blocco. Il modo in cui l'avete formulato decide se l'azione viene eseguita e quanto lontano arriva l'approvazione:

* **Nominate l'azione e i suoi dettagli**: il vostro messaggio deve nominare l'azione e la cosa specifica che la rende pericolosa, come il ramo di un force push. Nominare solo il verbo non cancella nulla, quindi "potete fare force-push" lascia il blocco in vigore.
* **Aspettatevi che copra un'azione**: un'approvazione copre l'azione distruttiva che avete nominato, quindi un'azione successiva viene bloccata di nuovo a meno che non abbiate concesso l'approvazione come permanente. Per smettere di approvare un modello di routine un'azione alla volta, aggiungetelo a [`autoMode.allow`](/docs/it/auto-mode-config#override-the-block-and-allow-rules).
* **Alcuni blocchi rimangono in vigore**: l'[ordine di precedenza del classificatore](/docs/it/auto-mode-config#override-the-block-and-allow-rules) stabilisce quali blocchi la vostra approvazione può raggiungere. Per eseguire un passaggio che non cancellerà, [lasciate la modalità auto](#switch-permission-modes) e rispondete al prompt di autorizzazione.

<h3 id="when-auto-mode-falls-back">
  Quando la modalità auto ricade
</h3>

Quando la modalità auto non può approvare le azioni della vostra sessione, cosa accade dipende dal caso:

* **Un'azione bloccata**: Claude Code mostra una notifica ed elenca l'azione in `/permissions` nella scheda **Recently denied**, dove potete premere `r` per ritentarla con un'approvazione manuale. Quando il classificatore produce [nessun verdetto sull'azione](/docs/it/errors#auto-mode-cannot-determine-the-safety-of-an-action), perché un controllo di sicurezza separato dalla modalità auto ha rifiutato la richiesta propria del classificatore o la sua risposta non è stata analizzata, Claude Code nega l'azione senza la notifica o la voce **Recently denied**.
* **Blocchi ripetuti**: se il classificatore blocca un'azione 3 volte di seguito o 20 volte in totale, la modalità auto si mette in pausa e Claude Code riprende a chiedere. Approvare l'azione richiesta riprende la modalità auto. Questi soglie non sono configurabili. Qualsiasi azione consentita ripristina il contatore consecutivo, mentre il contatore totale persiste per la sessione e si ripristina solo quando il suo limite attiva un fallback. Claude Code non conta una negazione verso nessuna soglia quando [un controllo di sicurezza separato dalla modalità auto rifiuta la richiesta del classificatore](/docs/it/errors#auto-mode-cannot-determine-the-safety-of-an-action); la voce collegata copre come Claude Code gestisce quelle negazioni.
* **Sessioni che non possono chiedere**: un'esecuzione `-p` [non interattiva](/docs/it/headless) senza un [`--permission-prompt-tool`](/docs/it/cli-reference#cli-flags) non ha un prompt a cui ricadere. Quando i blocchi ripetuti raggiungono una soglia, l'azione non viene eseguita e Claude continua a lavorare. Lo stesso si applica quando [un controllo di sicurezza separato dalla modalità auto rifiuta la richiesta del classificatore](/docs/it/errors#auto-mode-cannot-determine-the-safety-of-an-action). Claude Code non ferma l'esecuzione in nessuno dei due casi.
* **Nessun verdetto dal server**: sotto [revisione del classificatore lato server](#server-side-classifier-review), Claude Code nega un'azione per la quale il server non fornisce un verdetto e ferma il turno dopo dieci risposte di seguito senza verdetto. Consultate [Il server non ha restituito un verdetto di sicurezza](/docs/it/errors#the-server-returned-no-safety-verdict).
* **Un cambio di modalità durante un controllo**: se cambiate modalità di autorizzazione mentre un controllo del classificatore è in sospeso, Claude Code scarta un verdetto che la nuova modalità non avrebbe richiesto piuttosto che applicarlo: vi viene chiesto l'approvazione, o l'azione viene negata automaticamente in [modalità `dontAsk`](#allow-only-pre-approved-tools-with-dontask-mode).

I blocchi ripetuti di solito significano che il classificatore manca di contesto sulla vostra infrastruttura. Utilizzate `/feedback` per segnalare falsi positivi, o fate in modo che un amministratore [configuri l'infrastruttura attendibile](/docs/it/auto-mode-config).

<span id="how-the-classifier-evaluates-actions" />

<AccordionGroup>
  <Accordion title="Come il classificatore valuta le azioni">
    Ogni azione passa attraverso un ordine decisionale fisso. Il primo passaggio corrispondente vince:

    1. Le azioni che corrispondono alle vostre [regole di consentimento, richiesta o negazione](/docs/it/permissions#manage-permissions) si risolvono immediatamente, con queste eccezioni:
       * Le scritture su [percorsi protetti](#protected-paths) vengono instradate al classificatore anche quando una regola di consentimento corrisponde, e così anche le rimozioni `rm` e `rmdir` che prendono di mira un [percorso critico](#critical-paths) in Claude Code v2.1.218 e successivo
       * Gli strumenti MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool) vi chiedono direttamente anche quando una regola di consentimento corrisponde, e così anche gli strumenti connettore [che la vostra organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) nelle sessioni in cui quella impostazione raggiunge Claude Code
       * Un comando di shell che contiene [domini consentiti per comando](/docs/it/sandboxing#per-command-allowed-domains-in-auto-mode) viene anche instradato al classificatore anche quando una regola di consentimento corrisponde, perché una regola approva il comando, non i suoi host
       * Le regole di richiesta che corrispondono al contenuto di un comando, come `Bash(git push *)`, ricadono a un prompt di autorizzazione
    2. Le azioni di sola lettura e le modifiche di file nella vostra directory di lavoro vengono approvate automaticamente, eccetto le scritture su [percorsi protetti](#protected-paths) e [la prima lettura al di fuori delle directory di lavoro](#first-read-outside-the-working-directories), che vi chiede
       * In una sessione con [revisione del classificatore lato server](#server-side-classifier-review), le azioni di sola lettura e i comandi di shell [in sandbox](/docs/it/sandboxing#sandbox-modes) attendono quella revisione e vengono bloccati se la contrassegna
    3. Tutto il resto va al classificatore. Gli strumenti connettore e gli strumenti MCP `requiresUserInteraction` che vi chiedono direttamente nel passaggio 1 non raggiungono mai il classificatore, quindi né un'approvazione richiesta dall'organizzazione né un passaggio di consenso viene approvato automaticamente
    4. Se il classificatore blocca, Claude riceve il motivo e prova un'alternativa. Nella maggior parte delle sessioni il motivo nomina la regola che il classificatore ha abbinato, come `[Data Exfiltration]`, piuttosto che dare una spiegazione scritta; consultate [Rivedi negazioni](/docs/it/auto-mode-config#review-denials)

    Entrando in modalità auto, le regole di consentimento ampie che concedono l'esecuzione arbitraria di codice vengono eliminate:

    * Blanket `Bash(*)` o `PowerShell(*)`
    * Interpreti con wildcard come `Bash(python*)`
    * Comandi di esecuzione del gestore di pacchetti
    * Regole di consentimento `Agent`
    * Regole di consentimento [`Monitor`](/docs/it/tools-reference#monitor-tool), perché Claude Code esegue i comandi Monitor attraverso la shell

    Le regole strette come `Bash(npm test)` rimangono in vigore. Claude Code ripristina le regole eliminate quando lasciate la modalità auto. Prima della v2.1.236, Claude Code lasciava le regole di consentimento `Monitor` in vigore in modalità auto, quindi una regola che corrispondeva all'intero strumento approvava i comandi Monitor senza revisione del classificatore.

    Claude Code esegue anche `git status` stesso prima di un comando che scarterebbero il lavoro non sottoposto a commit, come `git reset --hard` o `rm -rf`, e mostra al classificatore se il lavoro in staging, modificato o non tracciato è presente. Claude Code segnala i file non tracciati in quel controllo anche quando la configurazione git del repository imposta `status.showUntrackedFiles=no`.

    Nelle richieste del classificatore inviate da Claude Code stesso, il classificatore vede i messaggi utente, le chiamate di strumento diverse dalle ricerche di sola lettura come letture di file e ricerche, e il vostro contenuto CLAUDE.md. I risultati degli strumenti vengono rimossi da quelle richieste, quindi il contenuto ostile in un file o pagina web non può manipolare il classificatore direttamente.

    Potete annotare il risultato di una chiamata con il campo `classifierContext` di un hook [PostToolUse](/docs/it/hooks#annotate-a-result-for-the-auto-mode-classifier), che il classificatore legge come contesto fornito dall'applicazione. Il campo richiede Claude Code v2.1.236 o successivo.

    Una sonda lato server separata scansiona i risultati degli strumenti in arrivo e contrassegna il contenuto sospetto prima che Claude lo legga. Per ulteriori informazioni su come questi livelli lavorano insieme, consultate l'[annuncio della modalità auto](https://claude.com/blog/auto-mode) e l'[approfondimento tecnico](https://www.anthropic.com/engineering/claude-code-auto-mode).
  </Accordion>

  <Accordion title="Come la modalità auto gestisce i subagenti">
    Il classificatore controlla il lavoro dei [subagenti](/docs/it/sub-agents) in tre punti:

    1. Prima che un subagente inizi, la descrizione del compito delegato viene valutata, quindi un compito che sembra pericoloso viene bloccato al momento dello spawn.
    2. Mentre il subagente viene eseguito, ognuna delle sue azioni passa attraverso il classificatore con le stesse regole della sessione padre, e qualsiasi `permissionMode` nel frontmatter del subagente viene ignorato.
    3. Quando il subagente finisce, il classificatore esamina il suo lavoro e il suo rapporto finale prima che il padre legga il rapporto. Quando il classificatore contrassegna il lavoro o il rapporto del subagente, o un controllo di sicurezza API separato rifiuta la revisione, il rapporto viene comunque consegnato, preceduto da un avviso di sicurezza. Quando il classificatore non è disponibile per la revisione, il rapporto arriva con una nota per verificare il lavoro del subagente prima di agire su di esso.
  </Accordion>

  <Accordion title="Costo e latenza">
    Il classificatore viene eseguito su Claude Sonnet 5 per impostazione predefinita piuttosto che sulla vostra selezione `/model`. Un modello classificatore che Anthropic configura lato server ha la precedenza su quel default. Quando il modello della vostra sessione è Claude Sonnet 4.6, o quando [`availableModels`](/docs/it/model-config#restrict-model-selection) esclude Sonnet 5, il classificatore viene eseguito sul modello della sessione, o su un modello Opus quando la sessione viene eseguita su un [modello Fable](/docs/it/model-config#work-with-fable); su provider diversi dall'API Anthropic, quel fallback Opus è il modello Opus predefinito del provider.

    La prima richiesta in modalità auto della sessione convalida il default Sonnet 5: se la richiesta ha successo, Sonnet 5 rimane il modello classificatore della sessione, e se fallisce perché il modello non è disponibile, la sessione utilizza il fallback. Dopo che quella convalida si stabilizza, il modello del classificatore non cambia per la sessione.

    Su piani Enterprise e su account che utilizzano l'API Claude, [Claude Platform su AWS](/docs/it/claude-platform-on-aws), Amazon Bedrock, Agent Platform di Google Cloud o Microsoft Foundry, le chiamate del classificatore contano verso il vostro utilizzo di token. Ogni controllo invia una porzione della trascrizione più l'azione in sospeso, aggiungendo un round-trip prima dell'esecuzione. Le letture e le modifiche della directory di lavoro al di fuori dei percorsi protetti saltano il classificatore, quindi l'overhead proviene principalmente dai comandi di shell e dalle operazioni di rete. Dove il server esamina le azioni come parte delle richieste del modello della sessione, non ci sono richieste del classificatore separate da contare; consultate [Revisione del classificatore lato server](#server-side-classifier-review).

    L'accesso di rete in sandbox non aggiunge richieste del classificatore per connessione. Il classificatore giudica [gli host che un comando nomina](/docs/it/sandboxing#per-command-allowed-domains-in-auto-mode) insieme al comando in una revisione, e Claude Code controlla ogni connessione rispetto all'elenco approvato senza chiamare il classificatore di nuovo.
  </Accordion>
</AccordionGroup>

<h2 id="allow-only-pre-approved-tools-with-dontask-mode">
  Consenti solo strumenti pre-approvati con la modalità dontAsk
</h2>

Se imposti la modalità `dontAsk`, Claude Code nega automaticamente ogni chiamata di strumento che altrimenti richiederebbe una richiesta. Claude esegue ancora azioni che non richiedono approvazione in modalità Manual, come le letture di file all'interno delle tue directory di lavoro e i [comandi Bash di sola lettura](/docs/it/permissions#read-only-commands), più le azioni che corrispondono alle tue regole `permissions.allow` e le chiamate approvate da un [hook PreToolUse](/docs/it/permissions#extend-permissions-with-hooks). Utilizza questa modalità per pipeline CI o ambienti limitati in cui pre-definisci cosa Claude può fare; la sessione non attende mai input. La barra di stato mostra `⏵⏵ don't ask on` mentre questa modalità è attiva.

Claude Code nega le chiamate che corrispondono alle tue regole [`ask`](/docs/it/permissions#manage-permissions) esplicite piuttosto che richiedere una conferma. Nega anche lo strumento integrato `AskUserQuestion` anche se le tue regole di autorizzazione lo corrispondono, e fa lo stesso agli strumenti connector [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) in sessioni dove quella impostazione raggiunge Claude Code. Nega gli strumenti MCP contrassegnati [`_meta["anthropic/requiresUserInteraction"]`](/docs/it/mcp#require-approval-for-a-specific-tool) allo stesso modo, perché la loro scheda di approvazione necessita di una risposta che questa modalità non raccoglie mai; questo richiede Claude Code v2.1.199 o successivo.

Le rimozioni `rm` e `rmdir` che prendono di mira un [percorso critico](#critical-paths), come `rm -rf /` e `rm -rf ~`, vengono negate anche quando una regola allow le corrisponde o un hook `PreToolUse` le approva.

Le sessioni cloud su [Claude Code sul web](/docs/it/claude-code-on-the-web) ignorano `defaultMode: "dontAsk"`; vedi [bypassPermissions](#skip-all-checks-with-bypasspermissions-mode) per i dettagli.

Impostalo all'avvio con il flag:

```bash theme={null}
claude --permission-mode dontAsk
```

<h2 id="skip-all-checks-with-bypasspermissions-mode">
  Ignora tutti i controlli con la modalità bypassPermissions
</h2>

La modalità `bypassPermissions` disabilita i prompt di autorizzazione e i controlli di sicurezza in modo che le chiamate agli strumenti vengono eseguite immediatamente, incluse le scritture su [percorsi protetti](#protected-paths).

Le [azioni che nessuna modalità auto-approva](#actions-no-mode-auto-approves) richiedono comunque un prompt in questa modalità.

Due [protezioni di messaggistica cross-sessione](/docs/it/cross-session-messaging) si applicano comunque in questa modalità, e in sessioni in modalità plan interattive dove le autorizzazioni di bypass sono disponibili:

* Il prompt di approvazione [`isolatePeerMachines`](/docs/it/settings-reference#isolatepeermachines) per i messaggi alle tue sessioni oltre questa macchina appare comunque.
* Quando nessun valore [`crossSessionInbound`](/docs/it/cross-session-messaging#control-inbound-messages) si applica, Claude Code tiene un messaggio in arrivo da un'altra delle tue sessioni per la tua approvazione, e consegna senza chiedere solo quando la sessione di invio si identifica come anche bypassando i prompt di autorizzazione. Se lasci la modalità di autorizzazione mentre i messaggi sono tenuti, Claude Code riapplica le regole in arrivo e consegna qualsiasi messaggio tenuto che ora accettano.

Nelle sessioni di terminale interattive con autorizzazioni di bypass disponibili, Claude Code inoltre non applica i [blocchi della modalità plan](#analyze-before-you-edit-with-plan-mode). Claude è ancora istruito a pianificare senza modificare, ma una modifica di file o un comando shell che tenta durante la pianificazione viene eseguito senza richiedere. Le [regole ask](/docs/it/permissions#manage-permissions) esplicite e le rimozioni `rm` e `rmdir` che prendono di mira un [percorso critico](#critical-paths) richiedono comunque un prompt.

La modalità plan mantiene i suoi blocchi ovunque Claude Code viene eseguito senza un terminale interattivo, incluse le [esecuzioni non interattive](/docs/it/headless) con `-p`, le sessioni [Agent SDK](/docs/it/agent-sdk/permissions#plan-mode-plan), e le conversazioni nel pannello chat dell'[estensione VS Code](/docs/it/vs-code). Lì, `--allow-dangerously-skip-permissions` rende `bypassPermissions` selezionabile in seguito.

<Warning>
  Utilizza questa modalità solo in ambienti isolati come container, VM o dev container senza accesso a Internet, dove Claude Code non può danneggiare il sistema host.
</Warning>

Non puoi accedere a `bypassPermissions` da una sessione che hai avviato senza di essa abilitata. Abilitala al lancio con [`permissions.defaultMode: "bypassPermissions"`](/docs/it/settings-reference#permissions-defaultmode) o con un flag di abilitazione:

```bash theme={null}
claude --permission-mode bypassPermissions
```

Il flag `--dangerously-skip-permissions` è equivalente.

Claude Code rifiuta `bypassPermissions` in una sessione che avvii con [`--restricted`](/docs/it/cli-reference#cli-flags). `--restricted` richiede Claude Code v2.1.248 o successivo.

La prima volta che avvii una sessione interattiva con questa modalità abilitata, Claude Code mostra una finestra di dialogo di avviso che ti chiede di accettare la responsabilità per le azioni intraprese senza controlli di autorizzazione. Claude Code salva la tua accettazione nelle impostazioni utente, quindi la finestra di dialogo appare solo una volta. Se rifiuti, Claude Code esce. In [modalità non interattiva](/docs/it/headless) nessuna finestra di dialogo viene mostrata, e una [sessione in background](/docs/it/agent-view) avviata con `--bg` viene rifiutata finché non hai accettato la finestra di dialogo in una sessione interattiva.

Su Linux e macOS, Claude Code rifiuta di avviarsi in questa modalità quando viene eseguito come root o sotto `sudo`:

```text theme={null}
--dangerously-skip-permissions cannot be used with root/sudo privileges for security reasons
```

Il controllo viene saltato automaticamente all'interno di una sandbox riconosciuta. Per eseguire in modo autonomo in un container, utilizza la configurazione [dev container](/docs/it/devcontainer), che esegue Claude Code come utente non root.

[Claude Code sul web](/docs/it/claude-code-on-the-web) non rispetta `defaultMode: "bypassPermissions"` o `"dontAsk"` dai file di impostazioni, quindi le impostazioni archiviate di un repository non possono avviare una sessione cloud in modalità bypass-permissions. L'impostazione viene ignorata silenziosamente e la sessione si avvia nella modalità mostrata nel menu a discesa della modalità. Vedi [Cambia modalità di autorizzazione](#switch-permission-modes) per le modalità offerte dalle sessioni cloud.

<Warning>
  `bypassPermissions` non offre protezione contro l'iniezione di prompt o azioni indesiderate. Per i controlli di sicurezza di background con molti meno prompt di autorizzazione, utilizza invece la [modalità auto](#eliminate-prompts-with-auto-mode). Gli amministratori possono bloccare questa modalità impostando `permissions.disableBypassPermissionsMode` su `"disable"` nelle [impostazioni gestite](/docs/it/managed-settings).
</Warning>

<h2 id="protected-paths">
  Percorsi protetti
</h2>

Le scritture in un piccolo insieme di percorsi non vengono mai auto-approvate, tranne in modalità `bypassPermissions` e in sessioni in modalità plan dove le [autorizzazioni di bypass](#skip-all-checks-with-bypasspermissions-mode) sono disponibili. Ciò previene la corruzione accidentale dello stato del repository e della configurazione di Claude.

| Modalità                 | Scritture su percorsi protetti                                                                                                                                                                                                                                                              |
| :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `default`, `acceptEdits` | Richiesta di conferma                                                                                                                                                                                                                                                                       |
| `plan`                   | Consentito in sessioni con [autorizzazioni di bypass](#skip-all-checks-with-bypasspermissions-mode) disponibili. Altrimenti, instradato al classificatore quando [la modalità auto](#eliminate-prompts-with-auto-mode) è disponibile durante la pianificazione, e richiesto quando non lo è |
| `auto`                   | Instradato al classificatore                                                                                                                                                                                                                                                                |
| `dontAsk`                | Negato                                                                                                                                                                                                                                                                                      |
| `bypassPermissions`      | Consentito                                                                                                                                                                                                                                                                                  |

In una sessione avviata con [`--restricted`](/docs/it/cli-reference#cli-flags), che richiede Claude Code v2.1.248 o successivo, il classificatore non può approvare le scritture su percorsi protetti.

Le regole [`permissions.allow`](/docs/it/permissions#manage-permissions) nei file di impostazioni non pre-approvano le scritture su percorsi protetti. Il controllo di sicurezza viene eseguito prima che Claude Code valuti le regole allow dalle impostazioni, quindi una voce come `Edit(.claude/**)` in `~/.claude/settings.json` o `.claude/settings.json` non modifica il risultato per modalità nella tabella precedente. Nelle modalità che richiedono conferma, il prompt per una scrittura `.claude/` offre **Sì, e consenti a Claude di modificare le proprie impostazioni per questa sessione**, che approva le successive scritture `.claude/` in quella sessione senza richiedere di nuovo la conferma.

Directory protette:

* `.git`
* `.config/git`
* `.vscode`
* `.idea`
* `.husky`
* `.cargo`
* `.devcontainer`
* `.yarn`
* `.mvn`
* `.claude`, ad eccezione di `.claude/worktrees` dove Claude memorizza i propri git worktrees

File protetti:

* `.gitconfig`, `.gitmodules`
* `.bashrc`, `.bash_profile`, `.bash_login`, `.bash_aliases`, `.bash_logout`, `.zshrc`, `.zprofile`, `.zshenv`, `.zlogin`, `.zlogout`, `.profile`, `.envrc`
* `.npmrc`, `.yarnrc`, `.yarnrc.yml`, `.pnp.cjs`, `.pnp.loader.mjs`, `.pnpmfile.cjs`, `bunfig.toml`, `.bunfig.toml`
* `.bazelrc`, `.bazelversion`, `.bazeliskrc`
* `.pre-commit-config.yaml`, `lefthook.yml`, `lefthook.yaml`, `.lefthook.yml`, `.lefthook.yaml`
* `gradle-wrapper.properties`, `maven-wrapper.properties`
* `.devcontainer.json`
* `.ripgreprc`, `pyrightconfig.json`
* `.mcp.json`, `.claude.json`

<h2 id="critical-paths">
  Percorsi critici
</h2>

Claude Code non consente mai a una regola [`permissions.allow`](/docs/it/permissions#manage-permissions) o a un hook [`PreToolUse`](/docs/it/permissions#extend-permissions-with-hooks) che restituisce `"allow"` di approvare un comando `rm` o `rmdir` che prende di mira un percorso critico, anche in modalità che saltano altri prompt. Questo interruttore di circuito protegge contro l'errore del modello. Una regola di negazione corrispondente blocca comunque il comando completamente.

Cosa succede invece dipende dalla tua modalità di autorizzazione:

| Modalità                 | Cosa Claude Code fa con una rimozione di percorso critico                                                                                                                                                        |
| :----------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`, `acceptEdits` | Ti chiede di approvarlo                                                                                                                                                                                          |
| `plan`                   | Ti chiede di approvarlo. Con [la modalità auto disponibile durante la pianificazione](#analyze-before-you-edit-with-plan-mode) e nessuna autorizzazione di bypass disponibile, lo invia al classificatore invece |
| `auto`                   | Lo invia al [classificatore](#eliminate-prompts-with-auto-mode)                                                                                                                                                  |
| `dontAsk`                | Lo nega                                                                                                                                                                                                          |
| `bypassPermissions`      | Ti chiede di approvarlo                                                                                                                                                                                          |

Se una [regola ask](/docs/it/permissions#manage-permissions) esplicita corrisponde al comando, Claude Code ti chiede anche in modalità `auto`. Nelle modalità che chiedono, un hook [`PermissionRequest`](/docs/it/hooks#permissionrequest) può rispondere al prompt come risponde a qualsiasi altro.

Claude Code tratta un target `rm` o `rmdir` come un percorso critico quando è uno dei seguenti:

* La radice del filesystem
* Directory di primo livello, il che significa qualsiasi figlio diretto della radice, come `/usr`, `/etc`, o `/data`
* La tua directory home
* Radici di unità Windows e le loro directory di primo livello, come `C:\` e `C:\Windows`
* La tua directory di lavoro e i suoi genitori
* Le tue directory di lavoro aggiuntive e i loro genitori, ma solo quando la rimozione è un glob sotto uno di essi, come `rm -rf <dir>/*`. `rm -rf <dir>` sulla directory stessa non attiva questo controllo

Claude Code tratta anche un glob o una barra finale direttamente sotto una variabile shell, come `rm -rf "$DIR"/*`, come una rimozione di percorso critico, perché il comando diventa una rimozione dalla radice del filesystem quando la variabile è vuota.

Il prompt per questo caso di variabile nomina il comando `rm` contrassegnato e dice come riscriverlo in modo che il controllo passi:

* Per una variabile come `$DIR`, proteggi ogni espansione in modo che la shell si fermi con un errore quando la variabile non è impostata o è vuota, come in `rm -rf "${DIR:?}"/*`, oppure usa un percorso letterale
* Per una variabile che è normalmente impostata, come `$HOME`, usa un percorso letterale

Una rimozione le cui espansioni sono tutte protette in questo modo non è una rimozione di percorso critico, quindi in modalità `bypassPermissions` viene eseguita senza un prompt.

Nascondere la rimozione dentro una subshell con `(...)`, un gruppo di parentesi graffe con `{ ...; }`, la sostituzione di comando con `$(...)` o backtick, o la sostituzione di processo con `<(...)`, non salta il controllo. Claude Code trova una rimozione di percorso critico indipendentemente dal fatto che si trovi dentro la forma annidata, come in `(rm -rf ~)` o `echo "$(rm -rf ~)"`, o altrove nello stesso comando.

<h3 id="remove-item-in-powershell">
  Remove-Item in PowerShell
</h3>

Quando abiliti lo [strumento PowerShell](/docs/it/tools-reference#powershell-tool), Claude Code dà a `Remove-Item` il suo proprio controllo, separato dall'elenco di percorsi critici `rm`. Il risultato dipende dal target, e il primo caso corrispondente si applica:

* **Percorsi di sistema**: la radice del filesystem e le sue directory di primo livello, le radici di unità e le loro directory di primo livello, e la tua directory home. Claude Code nega il comando in ogni modalità, senza chiederti.
* **Wildcard**: un `*` nudo, o qualsiasi target che termina in `/*` o `\*`, incluso un glob sotto una variabile shell come `$dir/*`. Claude Code nega il comando in ogni modalità, senza chiederti, prima che il [classificatore](#eliminate-prompts-with-auto-mode) lo veda.
* **La tua directory di lavoro o uno dei suoi genitori, con `-Recurse`**: Claude Code tratta il comando come qualsiasi altro che necessita di approvazione nella tua modalità di autorizzazione, quindi ti chiede nelle modalità che chiedono, lo invia al classificatore in modalità `auto`, e lo nega in modalità `dontAsk`. La modalità `bypassPermissions` salta questo controllo.

<h2 id="see-also">
  Vedi anche
</h2>

* [Permissions](/docs/it/permissions): regole allow, ask e deny; politiche gestite
* [Configure auto mode](/docs/it/auto-mode-config): comunica al classificatore quale infrastruttura la tua organizzazione ritiene affidabile
* [Hooks](/docs/it/hooks): logica di autorizzazione personalizzata tramite hook `PreToolUse` e `PermissionRequest`
* [Security](/docs/it/security): protezioni e best practice
* [Sandboxing](/docs/it/sandboxing): isolamento del filesystem e della rete per i comandi Bash
* [Non-interactive mode](/docs/it/headless): esegui Claude Code con il flag `-p`
