> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Automatizzare le azioni con hooks

> Esegui comandi shell automaticamente quando Claude Code modifica file, completa attività o ha bisogno di input. Formatta il codice, invia notifiche, convalida comandi e applica le regole del progetto.

Gli hooks sono comandi shell definiti dall'utente. Claude Code li esegue in punti specifici del suo ciclo di vita, il che ti dà un controllo deterministico: determinate azioni avvengono sempre piuttosto che affidarsi al modello linguistico per scegliere di eseguirle. Utilizza gli hooks per applicare le regole del progetto, automatizzare attività ripetitive e integrare Claude Code con i tuoi strumenti esistenti.

Per decisioni che richiedono giudizio piuttosto che regole deterministiche, potete anche utilizzare [hooks basati su prompt](#prompt-based-hooks) o [hooks basati su agenti](#agent-based-hooks) che utilizzano un modello Claude per valutare le condizioni.

Per altri modi di estendere Claude Code, consultate [skills](/docs/it/skills) per fornire a Claude istruzioni aggiuntive e comandi eseguibili, [subagents](/docs/it/sub-agents) per eseguire attività in contesti isolati, e [plugins](/docs/it/plugins/overview) per pacchettizzare estensioni da condividere tra i progetti.

<Tip>
  Questa guida copre i casi d'uso comuni e come iniziare. Per schemi di eventi completi, formati di input/output JSON e funzionalità avanzate come hooks asincroni e hooks di strumenti MCP, consultate il [riferimento Hooks](/docs/it/hooks).
</Tip>

<h2 id="set-up-your-first-hook">
  Configurare il vostro primo hook
</h2>

Per creare un hook, aggiungete un blocco `hooks` a un [file di impostazioni](#configure-hook-location). Questa procedura crea un hook di notifica desktop, in modo da ricevere un avviso ogni volta che Claude sta aspettando il vostro input invece di guardare il terminale.

<Steps>
  <Step title="Aggiungere l'hook alle vostre impostazioni">
    Aprite `~/.claude/settings.json` e aggiungete un hook `Notification`. Se il file non esiste, createlo. L'esempio sottostante utilizza `osascript` per macOS; consultate [Ricevere una notifica quando Claude ha bisogno di input](#get-notified-when-claude-needs-input) per i comandi Linux e Windows.

    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'"
              }
            ]
          }
        ]
      }
    }
    ```

    Se il vostro file di impostazioni ha già una chiave `hooks`, aggiungete `Notification` come sibling delle chiavi di evento esistenti piuttosto che sostituire l'intero oggetto. Ogni nome di evento è una chiave all'interno del singolo oggetto `hooks`:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write" }]
          }
        ],
        "Notification": [
          {
            "matcher": "",
            "hooks": [{ "type": "command", "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'" }]
          }
        ]
      }
    }
    ```

    Potete anche chiedere a Claude di scrivere l'hook per voi descrivendo quello che volete nella CLI.
  </Step>

  <Step title="Verificare la configurazione">
    Digitate `/hooks` per aprire il browser degli hooks. Vedrete un elenco di tutti gli eventi hook disponibili, con un conteggio accanto a ogni evento che ha hook configurati. Selezionate `Notification` per confermare che il vostro nuovo hook appare nell'elenco. Selezionando l'hook vengono mostrati i suoi dettagli: l'evento, il matcher, il tipo, il file di origine e il comando.
  </Step>

  <Step title="Testare l'hook">
    Premete `Esc` per tornare alla CLI. Premete `Shift+Tab` finché la barra di stato non mostra `⏸ manual mode on`, chiedete a Claude di fare qualcosa che richieda autorizzazione, quindi passate a un'altra finestra dal terminale. Dovreste ricevere una notifica desktop.
  </Step>
</Steps>

<Tip>
  Il menu `/hooks` è di sola lettura. Per aggiungere, modificare o rimuovere hooks, modificate il vostro JSON di impostazioni direttamente o chiedete a Claude di fare il cambiamento.
</Tip>

<h2 id="what-you-can-automate">
  Cosa potete automatizzare
</h2>

Gli hooks vi permettono di eseguire codice in punti chiave del ciclo di vita di Claude Code: formattare file dopo le modifiche, bloccare comandi prima che si eseguano, inviare notifiche quando Claude ha bisogno di input, iniettare contesto all'inizio della sessione, e altro ancora. Per l'elenco completo degli eventi hook, consultate il [riferimento Hooks](/docs/it/hooks#hook-lifecycle).

Ogni esempio include un blocco di configurazione pronto all'uso che aggiungete a un [file di impostazioni](#configure-hook-location).

Per un esempio di produzione di hooks che eseguono una revisione di un modello separato e reinseriscono i risultati nella sessione, consultate [come il plugin `security-guidance` si integra con Claude Code](/docs/it/security-guidance#how-the-plugin-integrates-with-claude-code).

<h3 id="get-notified-when-claude-needs-input">
  Ricevere una notifica quando Claude ha bisogno di input
</h3>

Ricevete una notifica desktop ogni volta che Claude finisce di lavorare e ha bisogno del vostro input, in modo da poter passare ad altri compiti senza controllare il terminale.

Questo hook utilizza l'evento `Notification`, che Claude Code attiva quando Claude sta aspettando input o autorizzazione. Consultate [quando ogni tipo di notifica si attiva](/docs/it/hooks#notification) per il timing esatto. Ogni scheda sottostante utilizza il comando di notifica nativo della piattaforma. Aggiungete questo a `~/.claude/settings.json`:

<Tabs>
  <Tab title="macOS">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'"
              }
            ]
          }
        ]
      }
    }
    ```

    <Accordion title="Se nessuna notifica appare">
      `osascript` instrada le notifiche attraverso l'app Script Editor integrata. Se Script Editor non ha il permesso di notifica, il comando fallisce silenziosamente e macOS non vi chiederà di concederlo. Eseguite questo in Terminal una volta per far apparire Script Editor nelle vostre impostazioni di notifica:

      ```bash theme={null}
      osascript -e 'display notification "test"'
      ```

      Nulla apparirà ancora. Aprite **System Settings > Notifications**, trovate **Script Editor** nell'elenco e attivate **Allow Notifications**. Eseguite il comando di nuovo per confermare che la notifica di test appare.
    </Accordion>
  </Tab>

  <Tab title="Linux">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "notify-send 'Claude Code' 'Claude Code needs your attention'"
              }
            ]
          }
        ]
      }
    }
    ```

    <Accordion title="Se nessuna notifica appare">
      `notify-send` ha bisogno di un daemon di notifica desktop, che i server headless, le sessioni SSH e la maggior parte dei container non hanno. Testate il comando direttamente per primo:

      ```bash theme={null}
      notify-send 'Claude Code' 'test'
      ```

      Se il comando non viene trovato, installate il pacchetto `libnotify-bin` su Debian e Ubuntu, o l'equivalente della vostra distribuzione.
    </Accordion>
  </Tab>

  <Tab title="Windows (PowerShell)">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "powershell.exe -Command \"[System.Reflection.Assembly]::LoadWithPartialName('System.Windows.Forms'); [System.Windows.Forms.MessageBox]::Show('Claude Code needs your attention', 'Claude Code')\""
              }
            ]
          }
        ]
      }
    }
    ```

    <Accordion title="Se nessuna finestra di dialogo appare">
      Questo comando apre una finestra di dialogo piuttosto che una notifica nell'angolo dello schermo, quindi la finestra di dialogo può aprirsi dietro la vostra finestra del terminale. Testate il comando direttamente in PowerShell per primo. Se eseguite Claude Code all'interno di WSL, `powershell.exe` deve essere disponibile nel vostro `PATH` attraverso l'interoperabilità di Windows.
    </Accordion>
  </Tab>
</Tabs>

Il matcher vuoto si attiva su tutti i tipi di notifica. Per attivarsi solo su eventi specifici, impostatelo a uno di questi valori:

| Matcher                      | Si attiva quando                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :--------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permission_prompt`          | Claude ha bisogno che approviate un uso dello strumento o una [richiesta di rete](/docs/it/sandboxing#network-isolation) di un comando sandboxed, e il prompt ha aspettato circa sei secondi                                                                                                                                                                                                                                                                                                                                                        |
| `idle_prompt`                | Claude ha finito di rispondere circa 60 secondi fa e voi non avete digitato da allora                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `auth_success`               | L'autenticazione si completa                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `elicitation_dialog`         | Un server MCP apre un modulo di elicitazione e voi non avete digitato per circa sei secondi                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `elicitation_url_dialog`     | Un server MCP vi chiede di aprire un URL del browser e voi non avete digitato per circa sei secondi                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `elicitation_complete`       | Un server MCP segnala che un'[elicitazione in modalità URL](/docs/it/hooks#elicitation-input) è completa                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `elicitation_response`       | Una risposta di elicitazione MCP viene inviata al server                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `agent_needs_input`          | Una sessione in background inizia ad aspettare il vostro input mentre la [visualizzazione agente](/docs/it/agent-view) è aperta, o la sessione corrente vi pone una [domanda di configurazione del terminale di un compagno di squadra agente](/docs/it/agent-teams#choose-a-display-mode) e voi non avete digitato per circa sei secondi                                                                                                                                                                                                                |
| `agent_completed`            | Una sessione in background finisce o fallisce. Si attiva solo mentre la [visualizzazione agente](/docs/it/agent-view) è aperta                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `quota_auto_resume_fired`    | Claude Code continua il vostro compito dopo che un limite di utilizzo di claude.ai lo ha messo in pausa: al reset, o prima quando qualcosa che fate in Claude Code durante l'attesa, come aggiungere crediti di utilizzo, aggiornare il vostro piano o cambiare modelli, rende l'utilizzo disponibile di nuovo, con l'[eccezione di impostazione del modello](/docs/it/interactive-mode#wait-for-a-usage-limit-to-reset)                                                                                                                            |
| `quota_auto_resume_stale`    | Un limite di utilizzo di claude.ai si è resettato mentre il vostro computer dormiva per più di circa 30 minuti. Claude Code aspetta che premiate `Enter` invece di continuare. Dopo un sonno più breve continua e attiva `quota_auto_resume_fired` invece                                                                                                                                                                                                                                                                                      |
| `quota_auto_resume_disabled` | Claude Code termina l'attesa per un limite di utilizzo di claude.ai senza continuare il vostro compito: [`autoContinueAtUsageLimit`](/docs/it/settings-reference#autocontinueatusagelimit) è stato disattivato o il reset si è spostato a più di 24 ore di distanza durante un'attesa che Claude Code ha avviato da solo, il compito continuato ha continuato a colpire il limite, o la continuazione è stata bloccata prima di raggiungere il modello. Non si attiva quando premete `Esc` o `Ctrl+C`, o scegliete **Don't continue automatically** |

Claude Code cronometra `permission_prompt` diversamente in un terminale e in Claude Desktop, l'estensione VS Code e altri host che rispondono alle richieste di autorizzazione attraverso l'Agent SDK. Consultate [quando ogni tipo di notifica si attiva](/docs/it/hooks#notification) per entrambi i timing.

I matcher `agent_needs_input` e `agent_completed` richiedono Claude Code v2.1.198 o successivo.

I matcher `quota_auto_resume_fired`, `quota_auto_resume_stale` e `quota_auto_resume_disabled` richiedono Claude Code v2.1.234 o successivo.

Nelle sessioni di terminale, `permission_prompt` per una richiesta di rete di un comando sandboxed richiede Claude Code v2.1.246 o successivo.

`agent_needs_input` per una domanda di configurazione del terminale di un compagno di squadra richiede Claude Code v2.1.248 o successivo.

Digitate `/hooks` e selezionate `Notification` per confermare che l'hook è registrato. Per lo schema completo dell'evento, consultate il [riferimento Notification](/docs/it/hooks#notification).

<h3 id="auto-format-code-after-edits">
  Formattare automaticamente il codice dopo le modifiche
</h3>

Eseguite automaticamente [Prettier](https://prettier.io/) su ogni file che Claude modifica, in modo che la formattazione rimanga coerente senza intervento manuale.

Questo hook utilizza l'evento `PostToolUse` con un matcher `Edit|Write`, quindi si esegue solo dopo gli strumenti di modifica dei file. Il comando estrae il percorso del file modificato con [`jq`](https://jqlang.org/) e lo passa a Prettier. Aggiungete questo a `.claude/settings.json` nella radice del vostro progetto:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write"
          }
        ]
      }
    ]
  }
}
```

Per testare l'hook, chiedete a Claude di aggiungere una riga con stringhe tra virgolette singole a un file JavaScript, quindi aprite il file: con le impostazioni predefinite di Prettier, l'hook le riscrive in virgolette doppie.

Quando l'hook ha successo, Claude Code non mostra nulla nella conversazione. Per confermare che l'hook è stato eseguito, controllate che il file modificato sia riformattato, o consultate [Tecniche di debug](#debug-techniques).

Per riformattare un file specifico comunque cambi, incluso quando un comando `Bash` lo riscrive, utilizzate un hook [FileChanged](/docs/it/hooks#filechanged) invece.

<Note>
  Gli esempi Bash in questa pagina utilizzano `jq` per l'analisi JSON. Installatelo con `brew install jq` su macOS, `apt-get install jq` su Debian e Ubuntu, o consultate i [download di `jq`](https://jqlang.org/download/).
</Note>

<h3 id="block-edits-to-protected-files">
  Bloccare le modifiche ai file protetti
</h3>

Impedite a Claude di modificare file sensibili come `.env`, `package-lock.json`, o qualsiasi cosa in `.git/`. Claude riceve un feedback che spiega perché la modifica è stata bloccata, in modo da poter adattare il suo approccio.

Questo esempio utilizza un file di script separato che l'hook chiama. Lo script controlla il percorso del file di destinazione rispetto a un elenco di modelli protetti ed esce con il codice 2 per bloccare la modifica.

<Steps>
  <Step title="Creare lo script dell'hook">
    Salvate questo in `.claude/hooks/protect-files.sh`:

    ```bash theme={null}
    #!/bin/bash
    # protect-files.sh

    INPUT=$(cat)
    FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

    # Normalize Windows backslash separators so the patterns below match
    FILE_PATH="${FILE_PATH//\\//}"

    PROTECTED_PATTERNS=(".env" "package-lock.json" ".git/")

    for pattern in "${PROTECTED_PATTERNS[@]}"; do
      if [[ "$FILE_PATH" == *"$pattern"* ]]; then
        echo "Blocked: $FILE_PATH matches protected pattern '$pattern'" >&2
        exit 2
      fi
    done

    exit 0
    ```
  </Step>

  <Step title="Rendere lo script eseguibile su macOS e Linux">
    Gli script degli hook devono essere eseguibili affinché Claude Code li esegua:

    ```bash theme={null}
    chmod +x .claude/hooks/protect-files.sh
    ```
  </Step>

  <Step title="Registrare l'hook">
    Aggiungete un hook `PreToolUse` a `.claude/settings.json` che esegue lo script prima di qualsiasi chiamata dello strumento `Edit` o `Write`:

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [
              {
                "type": "command",
                "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/protect-files.sh"
              }
            ]
          }
        ]
      }
    }
    ```
  </Step>

  <Step title="Testare l'hook">
    Chiedete a Claude di aggiungere un commento al vostro file `.env`. Claude Code blocca la modifica prima che si esegua e passa il messaggio `Blocked:` dello script a Claude come feedback.
  </Step>
</Steps>

<h3 id="re-inject-context-after-compaction">
  Reiniettare il contesto dopo la compattazione
</h3>

Quando la finestra di contesto di Claude si riempie, la compattazione riassume la conversazione per liberare spazio. Questo può perdere dettagli importanti. Utilizzate un hook `SessionStart` con un matcher `compact` per reiniettare il contesto critico dopo ogni compattazione.

Claude Code aggiunge il testo semplice che il vostro comando scrive su stdout al contesto di Claude. Questo esempio ricorda a Claude le convenzioni del progetto e il lavoro recente. Aggiungete questo a `.claude/settings.json` nella radice del vostro progetto:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "compact",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'Reminder: use Bun, not npm. Run bun test before committing. Current sprint: auth refactor.'"
          }
        ]
      }
    ]
  }
}
```

Potete sostituire l'`echo` con qualsiasi comando che produce output dinamico, come `git log --oneline -5` per mostrare i commit recenti. Per iniettare contesto all'inizio di ogni sessione, considerate di utilizzare [CLAUDE.md](/docs/it/memory) invece. Per le variabili di ambiente, consultate [`CLAUDE_ENV_FILE`](/docs/it/hooks#persist-environment-variables) nel riferimento.

<h3 id="audit-configuration-changes">
  Controllare le modifiche di configurazione
</h3>

Tracciate quando i file di impostazioni o skills cambiano durante una sessione. L'evento `ConfigChange` si attiva quando un processo esterno o un editor modifica un file di configurazione, in modo da poter registrare le modifiche per la conformità o bloccare le modifiche non autorizzate.

Questo esempio aggiunge ogni modifica a un registro di controllo. Aggiungete questo a `~/.claude/settings.json`:

```json theme={null}
{
  "hooks": {
    "ConfigChange": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "jq -c '{timestamp: now | todate, source: .source, file: .file_path}' >> ~/claude-config-audit.log"
          }
        ]
      }
    ]
  }
}
```

Il matcher filtra per tipo di configurazione: `user_settings`, `project_settings`, `local_settings`, `policy_settings`, o `skills`. Per bloccare una modifica dall'avere effetto, uscite con il codice 2 o restituite `{"decision": "block"}`. Consultate il [riferimento ConfigChange](/docs/it/hooks#configchange) per lo schema di input completo.

Per confermare che l'hook registra le modifiche, modificate un file di impostazioni in un altro editor mentre una sessione è in esecuzione, quindi aprite `~/claude-config-audit.log`: l'hook aggiunge una riga JSON per modifica con il timestamp, la fonte e il percorso del file.

<h3 id="reload-environment-when-directory-or-files-change">
  Ricaricare l'ambiente quando la directory o i file cambiano
</h3>

Alcuni progetti impostano variabili di ambiente diverse a seconda di quale directory siete. Strumenti come [direnv](https://direnv.net/) lo fanno automaticamente nella vostra shell, ma lo strumento Bash di Claude non raccoglie quei cambiamenti da solo.

L'accoppiamento di un hook `SessionStart` con un hook `CwdChanged` risolve questo. `SessionStart` carica le variabili per la directory in cui avviate, e `CwdChanged` le ricarica ogni volta che Claude cambia directory. Entrambi scrivono su `CLAUDE_ENV_FILE`, che Claude Code esegue come preambolo di script prima di ogni comando Bash. Aggiungete questo a `~/.claude/settings.json`:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ],
    "CwdChanged": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ]
  }
}
```

Eseguite `direnv allow` una volta in ogni directory che ha un `.envrc` in modo che direnv sia autorizzato a caricarlo. Se utilizzate devbox o nix invece di direnv, lo stesso modello funziona con `devbox shellenv` o `devbox global shellenv` al posto di `direnv export bash`.

Per reagire a file specifici invece di ogni cambio di directory, utilizzate `FileChanged` con un `matcher` che elenca i nomi dei file da guardare, separati da `|`. Quando costruite l'elenco di osservazione, Claude Code divide questo valore in nomi di file letterali piuttosto che valutarlo come regex. Consultate [FileChanged](/docs/it/hooks#filechanged) per come lo stesso valore filtra anche quali gruppi di hook si eseguono quando un file cambia. Questo esempio guarda `.envrc` e `.env` nella directory di lavoro:

```json theme={null}
{
  "hooks": {
    "FileChanged": [
      {
        "matcher": ".envrc|.env",
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ]
  }
}
```

Consultate le voci di riferimento [CwdChanged](/docs/it/hooks#cwdchanged) e [FileChanged](/docs/it/hooks#filechanged) per gli schemi di input, l'output `watchPaths`, e i dettagli di `CLAUDE_ENV_FILE`.

<h3 id="auto-approve-specific-permission-prompts">
  Approvare automaticamente specifici prompt di autorizzazione
</h3>

Saltate la finestra di dialogo di approvazione per le chiamate di strumenti che consentite sempre. Questo esempio approva automaticamente `ExitPlanMode`, lo strumento che Claude chiama quando finisce di presentare un piano e chiede di procedere, in modo da non essere richiesto ogni volta che un piano è pronto.

A differenza degli esempi di codice di uscita sopra, l'approvazione automatica richiede che il vostro hook scriva una decisione JSON su stdout. Claude Code esegue gli hook `PermissionRequest` quando sta per chiedervi l'autorizzazione, e se il vostro hook restituisce `"behavior": "allow"`, Claude Code risponde alla richiesta per vostro conto.

Il matcher limita l'hook a `ExitPlanMode` solo, in modo che nessun altro prompt sia interessato. Aggiungete questo a `~/.claude/settings.json`:

```json theme={null}
{
  "hooks": {
    "PermissionRequest": [
      {
        "matcher": "ExitPlanMode",
        "hooks": [
          {
            "type": "command",
            "command": "echo '{\"hookSpecificOutput\": {\"hookEventName\": \"PermissionRequest\", \"decision\": {\"behavior\": \"allow\"}}}'"
          }
        ]
      }
    ]
  }
}
```

Quando l'hook approva, Claude Code esce dalla modalità piano e ripristina qualsiasi modalità di autorizzazione fosse attiva prima di entrare in modalità piano. La trascrizione mostra "Allowed by PermissionRequest hook" dove la finestra di dialogo sarebbe apparsa. Il percorso dell'hook mantiene sempre la conversazione corrente: non può cancellare il contesto e avviare una sessione di implementazione fresca come la finestra di dialogo può.

Per impostare una modalità di autorizzazione specifica invece, l'output del vostro hook può includere un array `updatedPermissions` con una voce `setMode`. Il valore `mode` è qualsiasi modalità di autorizzazione come `default`, `acceptEdits`, o `bypassPermissions`, e `destination: "session"` la applica solo per la sessione corrente.

<Note>
  `bypassPermissions` si applica solo se la sessione è stata avviata con modalità bypass già disponibile: `--dangerously-skip-permissions`, `--permission-mode bypassPermissions`, `--allow-dangerously-skip-permissions`, o `permissions.defaultMode: "bypassPermissions"` nelle [impostazioni utente, `--settings`, o impostazioni gestite](/docs/it/settings-reference#permissions-defaultmode). Non si applica se la modalità bypass è disabilitata da [`permissions.disableBypassPermissionsMode`](/docs/it/permissions#managed-settings), o se avete avviato la sessione in [modalità ristretta](/docs/it/cli-reference#cli-flags).

  Claude Code non la salva mai come `defaultMode`.
</Note>

Per passare la sessione a `acceptEdits`, il vostro hook scrive questo JSON su stdout:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "allow",
      "updatedPermissions": [
        { "type": "setMode", "mode": "acceptEdits", "destination": "session" }
      ]
    }
  }
}
```

Mantenete il matcher il più ristretto possibile. Corrispondere a `.*` o lasciare il matcher vuoto approverebbe automaticamente ogni prompt di autorizzazione, incluse le scritture di file e i comandi shell. Consultate il [riferimento PermissionRequest](/docs/it/hooks#permissionrequest-decision-control) per l'insieme completo di campi di decisione.

<h2 id="how-hooks-work">
  Come funzionano gli hooks
</h2>

Claude Code attiva eventi hook in punti specifici del suo ciclo di vita. Quando un evento si attiva, Claude Code esegue tutti gli hooks corrispondenti in parallelo; consultate [Campi del gestore hook](/docs/it/hooks#hook-handler-fields) per come vengono trattati i gestori duplicati. La tabella sottostante mostra ogni evento e quando si attiva:

| Evento                | Quando si attiva                                                                                                                                                                                                                                                                                                                        |
| :-------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `SessionStart`        | Quando una sessione inizia o riprende                                                                                                                                                                                                                                                                                                   |
| `Setup`               | Quando avvii Claude Code con `--init-only`, o con `--init` o `--maintenance` in modalità `-p`. Per la preparazione una tantum in CI o script                                                                                                                                                                                            |
| `UserPromptSubmit`    | Quando invii un prompt, prima che Claude lo elabori                                                                                                                                                                                                                                                                                     |
| `UserPromptExpansion` | Quando un comando digitato dall'utente si espande in un prompt, prima che raggiunga Claude. Può bloccare l'espansione                                                                                                                                                                                                                   |
| `PreToolUse`          | Prima che una chiamata a uno strumento si esegua. Può bloccarla                                                                                                                                                                                                                                                                         |
| `PermissionRequest`   | Quando una chiamata a uno strumento necessita di una decisione di autorizzazione                                                                                                                                                                                                                                                        |
| `PermissionDenied`    | Quando la modalità automatica nega una chiamata a uno strumento, inclusi i rifiuti senza un verdetto del classificatore. Utilizza JSON `hookSpecificOutput.retry: true` per indicare al modello che può riprovare la chiamata allo strumento negata. Claude Code ignora `retry` quando il classificatore non ha prodotto alcun verdetto |
| `PostToolUse`         | Dopo che una chiamata a uno strumento ha successo                                                                                                                                                                                                                                                                                       |
| `PostToolUseFailure`  | Dopo che una chiamata a uno strumento fallisce                                                                                                                                                                                                                                                                                          |
| `PostToolBatch`       | Dopo che un intero batch di chiamate a strumenti paralleli si risolve, prima della prossima chiamata al modello                                                                                                                                                                                                                         |
| `Notification`        | Quando Claude Code invia una notifica                                                                                                                                                                                                                                                                                                   |
| `MessageDisplay`      | Mentre il testo del messaggio dell'assistente viene visualizzato                                                                                                                                                                                                                                                                        |
| `SubagentStart`       | Quando un subagente viene generato                                                                                                                                                                                                                                                                                                      |
| `SubagentStop`        | Quando un subagente termina                                                                                                                                                                                                                                                                                                             |
| `TaskCreated`         | Quando un'attività viene creata tramite `TaskCreate`                                                                                                                                                                                                                                                                                    |
| `TaskCompleted`       | Quando un'attività viene contrassegnata come completata                                                                                                                                                                                                                                                                                 |
| `Stop`                | Quando Claude finisce di rispondere                                                                                                                                                                                                                                                                                                     |
| `StopFailure`         | Quando il turno termina a causa di un errore API                                                                                                                                                                                                                                                                                        |
| `TeammateIdle`        | Quando un compagno di squadra di un [team di agenti](/docs/it/agent-teams) sta per diventare inattivo                                                                                                                                                                                                                                        |
| `InstructionsLoaded`  | Quando un file CLAUDE.md o `.claude/rules/*.md` viene caricato nel contesto. Si attiva all'inizio della sessione e quando i file vengono caricati in modo pigro durante una sessione                                                                                                                                                    |
| `ConfigChange`        | Quando un file di configurazione cambia durante una sessione                                                                                                                                                                                                                                                                            |
| `CwdChanged`          | Quando la directory di lavoro cambia, ad esempio quando Claude esegue un comando `cd`. Utile per la gestione reattiva dell'ambiente con strumenti come direnv                                                                                                                                                                           |
| `DirectoryAdded`      | Quando una directory di lavoro viene aggiunta a metà sessione tramite `/add-dir` o la richiesta di controllo SDK `register_repo_root`                                                                                                                                                                                                   |
| `FileChanged`         | Quando un file osservato cambia su disco. Il campo `matcher` specifica quali nomi di file osservare                                                                                                                                                                                                                                     |
| `WorktreeCreate`      | Quando un worktree viene creato tramite `--worktree`, `isolation: "worktree"`, o per una sessione in background. Sostituisce il comportamento git predefinito                                                                                                                                                                           |
| `WorktreeRemove`      | Quando un worktree viene rimosso all'uscita della sessione, quando un subagente termina, o quando elimini una sessione in background                                                                                                                                                                                                    |
| `PreCompact`          | Prima della compattazione del contesto                                                                                                                                                                                                                                                                                                  |
| `PostCompact`         | Dopo che la compattazione del contesto è completata                                                                                                                                                                                                                                                                                     |
| `PreModelSwitch`      | Prima che Claude Code applichi un cambio di modello che hai richiesto tu o un client. Può bloccare il cambio                                                                                                                                                                                                                            |
| `PostModelSwitch`     | Dopo che il modello della sessione cambia, inclusi i cambiamenti che Claude Code effettua autonomamente, come il ripristino del modello quando riprendi una sessione                                                                                                                                                                    |
| `Elicitation`         | Quando un server MCP richiede input dell'utente durante una chiamata a uno strumento                                                                                                                                                                                                                                                    |
| `ElicitationResult`   | Dopo che un utente risponde a un'elicitazione MCP, prima che la risposta venga inviata al server                                                                                                                                                                                                                                        |
| `SessionEnd`          | Quando una sessione termina                                                                                                                                                                                                                                                                                                             |

Ogni hook ha un `type` che determina come si esegue. La maggior parte degli hooks utilizza `"type": "command"`, che esegue un comando shell. Sono disponibili altri quattro tipi:

* `"type": "http"`: POST dei dati dell'evento a un URL. Consultate [HTTP hooks](#http-hooks).
* `"type": "mcp_tool"`: chiama uno strumento su un server MCP già connesso. Consultate [MCP tool hooks](/docs/it/hooks#mcp-tool-hook-fields).
* `"type": "prompt"`: valutazione LLM a turno singolo. Consultate [Prompt-based hooks](#prompt-based-hooks).
* `"type": "agent"`: verifica multi-turno con accesso agli strumenti. Gli agent hooks sono sperimentali e potrebbero cambiare. Consultate [Agent-based hooks](#agent-based-hooks).

<h3 id="combine-results-from-multiple-hooks">
  Combinare i risultati da più hooks
</h3>

Quando più hooks corrispondono allo stesso evento, il comando di ogni hook si esegue fino al completamento prima che Claude Code unisca i risultati. Un hook che restituisce `deny` non impedisce ai sibling hooks di eseguirsi. Non affidatevi al `deny` di un hook per sopprimere gli effetti collaterali in un altro hook.

Dopo che tutti gli hooks corrispondenti terminano, Claude Code combina i loro output. Per le decisioni di autorizzazione `PreToolUse`, la risposta più restrittiva vince, nell'ordine `deny`, `defer`, `ask`, `allow`. Il testo da `additionalContext` viene mantenuto da ogni hook e passato a Claude insieme.

L'esempio sottostante registra due hooks `PreToolUse` su `Bash`. Il primo aggiunge ogni comando a un file di log e esce con 0. Il secondo esegue uno script che esce con 2 per negare quando il comando contiene `rm -rf`:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r .tool_input.command >> ~/.claude/bash.log"
          },
          {
            "type": "command",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/block-rm-rf.sh"
          }
        ]
      }
    ]
  }
}
```

Quando Claude tenta di eseguire `rm -rf /tmp/build`, entrambi gli hooks si eseguono in parallelo. L'hook di logging scrive il comando a `~/.claude/bash.log` e esce con 0, il che non riporta alcuna decisione. L'hook di guardrail esce con 2, il che nega la chiamata dello strumento. Il deny vince, quindi Claude Code blocca il comando e mostra a Claude lo stderr del guardrail. La voce di log viene comunque scritta perché l'hook di logging si è già eseguito.

<h3 id="read-input-and-return-output">
  Leggere l'input e restituire l'output
</h3>

Gli hooks comunicano con Claude Code attraverso stdin, stdout, stderr e codici di uscita. Quando un evento si attiva, Claude Code passa i dati specifici dell'evento come JSON allo stdin del vostro script. Il vostro script legge quei dati, fa il suo lavoro, e dice a Claude Code cosa fare dopo tramite il codice di uscita.

<h4 id="hook-input">
  Input dell'hook
</h4>

Ogni evento include campi comuni come `session_id`, un ID univoco per la sessione, e `cwd`, la directory di lavoro quando l'evento si è attivato, ma ogni tipo di evento aggiunge dati diversi. Quando Claude esegue un comando Bash, un hook `PreToolUse` riceve questi campi su stdin:

* `hook_event_name`: l'evento che ha attivato l'hook
* `tool_name`: lo strumento che Claude sta per utilizzare
* `tool_input`: gli argomenti che Claude ha passato allo strumento. Per Bash, il suo campo `command` contiene il comando shell.

Ad esempio, l'input dell'hook per un comando `npm test` assomiglia a questo:

```json theme={null}
{
  "session_id": "abc123",
  "cwd": "/Users/sarah/myproject",
  "hook_event_name": "PreToolUse",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test"
  }
}
```

Il vostro script può analizzare quel JSON e agire su qualsiasi di quei campi. Gli hooks `UserPromptSubmit` ricevono il testo `prompt` invece, gli hook `SessionStart` ricevono una `source` di `startup`, `resume`, `clear`, `compact`, o `fork`, e così via. Consultate [Campi di input comuni](/docs/it/hooks#common-input-fields) nel riferimento per i campi condivisi, e la sezione di ogni evento per gli schemi specifici dell'evento.

<h4 id="hook-output">
  Output dell'hook
</h4>

Il vostro script dice a Claude Code cosa fare dopo scrivendo su stdout o stderr e uscendo con un codice specifico. Il seguente hook `PreToolUse` blocca un comando:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command')

if echo "$COMMAND" | grep -q "drop table"; then
  echo "Blocked: dropping tables is not allowed" >&2  # stderr becomes Claude's feedback
  exit 2 # exit 2 = block the action
fi

exit 0  # exit 0 = no decision; the normal permission flow applies
```

Il codice di uscita determina cosa succede dopo:

* **Exit 0**: il vostro hook non riporta obiezioni attraverso il suo codice di uscita.
  * Per un hook `PreToolUse` questo non approva la chiamata dello strumento: il normale [flusso di autorizzazione](/docs/it/permissions) si applica ancora.
  * Per gli hook `UserPromptSubmit`, `UserPromptExpansion`, `SessionStart` e `PostModelSwitch`, Claude Code aggiunge stdout che [tratta come testo semplice](/docs/it/hooks#exit-code-0) al contesto di Claude.
* **Exit 2**: Claude Code blocca l'azione. Scrivete un motivo su stderr. Dove finisce dipende dall'evento: alcuni eventi lo alimentano a Claude come feedback in modo che possa adattarsi, altri lo mostrano all'utente, e alcuni, come `ConfigChange` e `Elicitation`, non mostrano alcun messaggio. Alcuni eventi non possono essere bloccati: per `SessionStart` e altri, exit 2 mostra stderr all'utente e l'esecuzione continua. Consultate [exit code 2 behavior per evento](/docs/it/hooks#exit-code-2-behavior-per-event) per l'elenco completo.
* **Qualsiasi altro codice di uscita**: per la maggior parte degli eventi, il risultato dipende da quello che il vostro hook ha stampato su stdout:
  * Un oggetto analizzato che passa la validazione dello schema: Claude Code ignora il codice di uscita, il JSON da solo decide il risultato, e l'hook non viene segnalato come errore. Le eccezioni per evento, come `WorktreeCreate` che fallisce su qualsiasi uscita diversa da zero, sono elencate nella sezione [Exit code output](/docs/it/hooks#exit-code-output) del riferimento.
  * Un oggetto analizzato che fallisce la validazione dello schema, o stdout che Claude Code [tenta di analizzare come JSON](/docs/it/hooks#exit-code-0) ma che non è JSON valido: un errore non bloccante; l'avviso contiene il messaggio di validazione o analisi.
  * Stdout che Claude Code [tratta come testo semplice](/docs/it/hooks#exit-code-0), o stdout vuoto: l'azione procede come errore non bloccante. La trascrizione mostra un avviso `<hook name> hook error`, quindi la prima riga di stderr con prefisso `Failed with non-blocking status code:`. Per catturare lo stderr completo, abilitate il [debug logging](/docs/it/hooks#debug-hooks) con `claude --debug` o eseguendo `/debug` durante la sessione.

<h4 id="structured-json-output">
  Output JSON strutturato
</h4>

I codici di uscita vi permettono solo di bloccare o stare in silenzio. Per un controllo maggiore, uscite con 0 e stampate un oggetto JSON su stdout invece.

<Note>
  Utilizzate exit 2 per bloccare con un messaggio stderr, o exit 0 con JSON per un controllo strutturato. Scegliete un approccio per hook. Per quello che succede quando li mescolate, consultate [Exit code output](/docs/it/hooks#exit-code-output).
</Note>

Ad esempio, un hook `PreToolUse` può negare una chiamata di strumento e dire a Claude perché, o escalarlo all'utente per l'approvazione:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "deny",
    "permissionDecisionReason": "Use rg instead of grep for better performance"
  }
}
```

Con `"deny"`, Claude Code annulla la chiamata dello strumento e alimenta `permissionDecisionReason` di nuovo a Claude.

Su `PreToolUse`, Claude Code gestisce ogni valore `permissionDecision` come segue:

* `"allow"`: salta il prompt di autorizzazione interattivo. Le regole di negazione e richiesta, incluse le liste di negazione gestite dall'azienda, si applicano ancora, così come i prompt per gli strumenti MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool) e per gli strumenti connettore [che la vostra organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) nelle sessioni dove quella impostazione raggiunge Claude Code
* `"deny"`: annulla la chiamata dello strumento e invia il motivo a Claude
* `"ask"`: mostra il prompt di autorizzazione all'utente come al solito

Un quarto valore, `"defer"`, è disponibile in [modalità non interattiva](/docs/it/headless) con il flag `-p`. Esce dal processo con la chiamata dello strumento preservata in modo che un wrapper SDK Agent possa raccogliere input e riprendere. Consultate [Rinviare una chiamata di strumento per dopo](/docs/it/hooks#defer-a-tool-call-for-later) nel riferimento.

Un hook `PreModelSwitch` restituisce lo stesso campo `permissionDecision`: `"allow"` lascia procedere un cambio di modello, e `"deny"` lo annulla. `"ask"` vi chiede di confermare il cambio quando eseguite `/model` in una sessione interattiva; in tutti gli altri casi, Claude Code tratta `"ask"` come un rifiuto. Consultate [PreModelSwitch decision control](/docs/it/hooks#premodelswitch-decision-control).

Altri eventi utilizzano modelli di decisione diversi. Ad esempio, gli hook `PostToolUse` e `Stop` utilizzano un campo `decision: "block"` di livello superiore, mentre `PermissionRequest` utilizza `hookSpecificOutput.decision.behavior`. Consultate la [tabella di riepilogo](/docs/it/hooks#decision-control) nel riferimento per una suddivisione completa per evento.

Per gli hook `UserPromptSubmit`, utilizzate `hookSpecificOutput.additionalContext` invece per iniettare testo nel contesto di Claude. Annidare `additionalContext` dentro `hookSpecificOutput`; se lo posizionate al livello superiore del JSON, Claude Code lo ignora silenziosamente. Ad esempio, questo output aggiunge lo stato del ramo corrente a ogni prompt:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "UserPromptSubmit",
    "additionalContext": "Current branch: release-42. Deploy freeze until Friday."
  }
}
```

Consultate [UserPromptSubmit decision control](/docs/it/hooks#userpromptsubmit-decision-control) per la forma di output completa, incluso il blocco dei prompt e l'impostazione del titolo della sessione.

Gli hooks con `type: "prompt"` gestiscono l'output diversamente: consultate [Prompt-based hooks](#prompt-based-hooks).

<h3 id="filter-hooks-with-matchers">
  Filtrare gli hooks con i matcher
</h3>

Senza un matcher, un hook si attiva su ogni occorrenza del suo evento. I matcher vi permettono di restringerlo. Ad esempio, se volete eseguire un formattatore solo dopo le modifiche ai file, non dopo ogni chiamata di strumento, aggiungete un matcher al vostro hook `PostToolUse`:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          { "type": "command", "command": "prettier --write ..." }
        ]
      }
    ]
  }
}
```

Il matcher `"Edit|Write"` si attiva solo quando Claude utilizza lo strumento `Edit` o `Write`, non quando utilizza `Bash`, `Read`, o qualsiasi altro strumento. Una virgola separa le alternative allo stesso modo, quindi `"Edit, Write"` è equivalente. Consultate [Matcher patterns](/docs/it/hooks#matcher-patterns) per come i nomi semplici e le espressioni regolari vengono valutati.

<Note>
  Claude può anche creare o modificare file eseguendo comandi shell. Se il vostro hook deve vedere ogni modifica ai file, come per la scansione di conformità o il logging di audit, aggiungete un hook [`Stop`](/docs/it/hooks#stop) che scansiona l'albero di lavoro una volta per turno. Per una copertura per-chiamata invece, corrispondere anche a `Bash|PowerShell` e avere il vostro script elencare i file modificati e non tracciati con `git status --porcelain`. La sezione [PowerShell hook input](/docs/it/hooks#powershell) spiega perché corrispondere solo a `Bash` non è sufficiente. Per eseguire un hook quando un file specifico cambia su disco, indipendentemente da chi lo ha scritto, utilizzate un hook [FileChanged](/docs/it/hooks#filechanged).
</Note>

Ogni tipo di evento corrisponde a un campo specifico:

| Evento                                                                                                                                                          | Cosa filtra il matcher                                                                                             | Valori matcher di esempio                                                                                                                                                                                                                                                      |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`                                                                      | nome dello strumento                                                                                               | `Bash`, `Edit\|Write`, `mcp__.*`                                                                                                                                                                                                                                               |
| `SessionStart`                                                                                                                                                  | come è iniziata la sessione                                                                                        | `startup`, `resume`, `clear`, `compact`, `fork`                                                                                                                                                                                                                                |
| `Setup`                                                                                                                                                         | quale flag CLI ha attivato il setup                                                                                | `init`, `maintenance`                                                                                                                                                                                                                                                          |
| `SessionEnd`                                                                                                                                                    | perché è terminata la sessione                                                                                     | `clear`, `resume`, `logout`, `prompt_input_exit`, `other`                                                                                                                                                                                                                      |
| `Notification`                                                                                                                                                  | tipo di notifica                                                                                                   | `permission_prompt`, `idle_prompt`, `auth_success`, `elicitation_dialog`, `elicitation_url_dialog`, `elicitation_complete`, `elicitation_response`, `agent_needs_input`, `agent_completed`, `quota_auto_resume_fired`, `quota_auto_resume_stale`, `quota_auto_resume_disabled` |
| `SubagentStart`                                                                                                                                                 | tipo di agente                                                                                                     | `general-purpose`, `Explore`, `Plan`, o nomi di agenti personalizzati                                                                                                                                                                                                          |
| `PreCompact`, `PostCompact`                                                                                                                                     | cosa ha attivato la compattazione                                                                                  | `manual`, `auto`                                                                                                                                                                                                                                                               |
| `PreModelSwitch`, `PostModelSwitch`                                                                                                                             | nome canonico del modello a cui la sessione passa, come descritto sotto [PreModelSwitch](/docs/it/hooks#premodelswitch) | `claude-opus-5`, `claude-opus-4-6\|claude-opus-5`, `.*opus.*`                                                                                                                                                                                                                  |
| `SubagentStop`                                                                                                                                                  | tipo di agente                                                                                                     | stessi valori di `SubagentStart`                                                                                                                                                                                                                                               |
| `ConfigChange`                                                                                                                                                  | fonte di configurazione                                                                                            | `user_settings`, `project_settings`, `local_settings`, `policy_settings`, `skills`                                                                                                                                                                                             |
| `DirectoryAdded`                                                                                                                                                | come è stata aggiunta la directory                                                                                 | `slash_command`, `register_repo_root`                                                                                                                                                                                                                                          |
| `StopFailure`                                                                                                                                                   | tipo di errore                                                                                                     | `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `account_on_hold`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `cloud_credential_error`, `unknown`                                               |
| `InstructionsLoaded`                                                                                                                                            | motivo del caricamento                                                                                             | `session_start`, `nested_traversal`, `path_glob_match`, `include`, `compact`                                                                                                                                                                                                   |
| `Elicitation`                                                                                                                                                   | nome del server MCP                                                                                                | i vostri nomi di server MCP configurati                                                                                                                                                                                                                                        |
| `ElicitationResult`                                                                                                                                             | nome del server MCP                                                                                                | stessi valori di `Elicitation`                                                                                                                                                                                                                                                 |
| `FileChanged`                                                                                                                                                   | nomi di file letterali da guardare (consultate [FileChanged](/docs/it/hooks#filechanged))                               | `.envrc\|.env`                                                                                                                                                                                                                                                                 |
| `UserPromptExpansion`                                                                                                                                           | nome del comando                                                                                                   | i vostri nomi di skill o comando                                                                                                                                                                                                                                               |
| `UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `CwdChanged`, `MessageDisplay` | nessun supporto matcher                                                                                            | si attiva sempre su ogni occorrenza                                                                                                                                                                                                                                            |

Le schede sottostanti mostrano alcuni altri matcher su diversi tipi di evento.

<Tabs>
  <Tab title="Registrare ogni comando Bash">
    Corrispondere solo alle chiamate dello strumento `Bash` e registrare ogni comando in un file. L'evento `PostToolUse` si attiva dopo che il comando è completato, quindi `tool_input.command` contiene quello che è stato eseguito. L'hook riceve i dati dell'evento come JSON su stdin, e `jq -r '.tool_input.command'` estrae solo la stringa del comando, che `>>` aggiunge al file di log:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "jq -r '.tool_input.command' >> ~/.claude/command-log.txt"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Corrispondere agli strumenti MCP">
    Gli strumenti MCP utilizzano una convenzione di denominazione diversa rispetto agli strumenti integrati: `mcp__<server>__<tool>`, dove `<server>` è il nome del server MCP e `<tool>` è lo strumento che fornisce. Ad esempio, `mcp__github__search_repositories` o `mcp__filesystem__read_file`. Gli strumenti da un [server MCP fornito da plugin](/docs/it/mcp#plugin-provided-mcp-servers) utilizzano un segmento di server con ambito invece, come `mcp__plugin_my-plugin_db__query`. Utilizzate un matcher regex per indirizzare tutti gli strumenti da un server specifico, o corrispondere tra i server con un modello come `mcp__.*__write.*`. Consultate [Match MCP tools](/docs/it/hooks#match-mcp-tools) nel riferimento per l'elenco completo degli esempi.

    Il comando sottostante estrae il nome dello strumento dall'input JSON dell'hook con `jq` e lo scrive su stderr. Scrivere su stderr mantiene stdout pulito per l'output JSON e invia il messaggio al [debug log](/docs/it/hooks#debug-hooks):

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "mcp__github__.*",
            "hooks": [
              {
                "type": "command",
                "command": "echo \"GitHub tool called: $(jq -r '.tool_name')\" >&2"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Pulire alla fine della sessione">
    L'evento `SessionEnd` supporta i matcher sul motivo per cui la sessione è terminata. Questo hook si attiva solo sul motivo `clear`, impostato quando eseguite `/clear`, non su uscite normali:

    ```json theme={null}
    {
      "hooks": {
        "SessionEnd": [
          {
            "matcher": "clear",
            "hooks": [
              {
                "type": "command",
                "command": "rm -f /tmp/claude-scratch-*.txt"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>
</Tabs>

<h4 id="filter-by-tool-name-and-arguments-with-the-if-field">
  Filtrare per nome dello strumento e argomenti con il campo `if`
</h4>

Il campo `if` utilizza la [permission rule syntax](/docs/it/permissions) per filtrare gli hooks per nome dello strumento e argomenti insieme, in modo che il processo dell'hook si generi solo quando la chiamata dello strumento corrisponde. Questo va oltre il `matcher`, che filtra a livello di gruppo per nome dello strumento solo.

Ad esempio, questa configurazione esegue un hook solo quando Claude utilizza comandi `git` piuttosto che tutti i comandi Bash:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "if": "Bash(git *)",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/check-git-policy.sh"
          }
        ]
      }
    ]
  }
}
```

Se il vostro hook comando si esegue dipende dalla forma del vostro modello `if` e dal comando Bash che Claude sta invocando:

| Modello `if`       | Comando Bash           | L'hook si esegue? | Perché                                                                                                   |
| :----------------- | :--------------------- | :---------------- | :------------------------------------------------------------------------------------------------------- |
| `Bash(git *)`      | `git push`             | sì                | il nome del comando corrisponde                                                                          |
| `Bash(git *)`      | `npm test && git push` | sì                | ogni sottocomando viene controllato; `git push` corrisponde                                              |
| `Bash(git *)`      | `echo $(git log)`      | sì                | i comandi dentro `$()` e backtick vengono controllati; `git log` corrisponde                             |
| `Bash(git *)`      | `echo $(date)`         | no                | nessun sottocomando corrisponde a `git *`                                                                |
| `Bash(git push *)` | `echo $(date)`         | sì                | i modelli che specificano più del nome del comando eseguono l'hook comunque su `$()`, backtick, o `$VAR` |

Quando Claude Code non può determinare quali comandi l'input Bash esegue, esegue il vostro hook indipendentemente dal modello. La [tabella di corrispondenza Bash](/docs/it/hooks#bash-if-matching) copre le forme di comando che Claude Code può e non può restringere per sottocomando. Poiché il filtro è best-effort, utilizzate il [sistema di autorizzazione](/docs/it/permissions) piuttosto che un hook per applicare un hard allow o deny.

Il campo `if` accetta gli stessi modelli delle regole di autorizzazione: `"Bash(git *)"`, `"Edit(*.ts)"`, e così via. Per corrispondere a più nomi di strumenti, utilizzate handler separati ognuno con il suo valore `if`, o corrispondere a livello di `matcher` dove l'alternazione con pipe è supportata.

`if` funziona solo su eventi di strumenti: `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` e `PermissionDenied`. Aggiungerlo a qualsiasi altro evento impedisce all'hook di eseguirsi.

<h3 id="configure-hook-location">
  Configurare la posizione dell'hook
</h3>

Dove aggiungete un hook determina il suo ambito:

| Posizione                                | Ambito                                                                                                                                   | Condivisibile                                                   |
| :--------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------- |
| `~/.claude/settings.json`                | Tutti i vostri progetti                                                                                                                  | No, locale alla vostra macchina                                 |
| `.claude/settings.json`                  | Singolo progetto                                                                                                                         | Sì, può essere committato nel repo                              |
| `.claude/settings.local.json`            | Singolo progetto                                                                                                                         | No, gitignored quando Claude Code salva un'impostazione in esso |
| Impostazioni di policy gestite           | Organizzazione intera                                                                                                                    | Sì, controllato dall'amministratore                             |
| [Plugin](/docs/it/plugins) `hooks/hooks.json` | Quando il plugin è abilitato                                                                                                             | Sì, raggruppato con il plugin                                   |
| [Skill](/docs/it/skills) frontmatter          | Il resto della sessione una volta che la skill è invocata. Consultate [Hooks in skills and agents](/docs/it/hooks#hooks-in-skills-and-agents) | Sì, definito nel file della skill                               |
| [Subagent](/docs/it/sub-agents) frontmatter   | Mentre quel subagent è in esecuzione                                                                                                     | Sì, definito nel file del subagent                              |

Eseguite [`/hooks`](/docs/it/hooks#the-%2Fhooks-menu) in Claude Code per sfogliare tutti gli hooks configurati raggruppati per evento.

Per disabilitare gli hooks, impostate `"disableAllHooks": true` nel vostro file di impostazioni. Claude Code legge il valore rimasto dopo che la [precedenza delle impostazioni](/docs/it/hooks#disable-or-remove-hooks) si applica, in modo che il file di impostazioni di un progetto possa sovrascrivere il vostro. Gli hooks configurati nelle impostazioni gestite si eseguono comunque a meno che `disableAllHooks` non sia impostato anche lì. Per la portata completa di ogni livello, consultate [`disableAllHooks`](/docs/it/settings-reference#disableallhooks).

Se modificate i file di impostazioni direttamente mentre Claude Code è in esecuzione, il file watcher normalmente raccoglie i cambiamenti degli hook automaticamente.

<h2 id="prompt-based-hooks">
  Hooks basati su prompt
</h2>

Per decisioni che richiedono giudizio piuttosto che regole deterministiche, utilizzate gli hook `type: "prompt"`. Invece di eseguire un comando shell, Claude Code invia il vostro prompt e i dati di input dell'hook a un modello Claude (Haiku per impostazione predefinita) per prendere la decisione. Potete specificare un modello diverso con il campo `model` se avete bisogno di più capacità.

L'unico lavoro del modello è restituire la sua decisione come JSON:

* `"ok": true`: l'azione procede
* `"ok": false`: ciò che accade dipende dall'evento:
  * `Stop` e `SubagentStop`: il `reason` viene alimentato di nuovo a Claude in modo che continui a lavorare, a meno che la risposta non imposti anche `"impossible": true` per contrassegnare la condizione come una che non può mai essere soddisfatta, nel qual caso Claude Code consente lo stop e il turno termina
  * `PreToolUse`: la chiamata dello strumento viene negata; per impostazione predefinita il turno termina e il `reason` di negazione appare nella chat come una riga di avviso. Impostate `continueOnBlock: true` sull'hook per restituire invece il `reason` a Claude come errore dello strumento, in modo che possa adattarsi e continuare. Prima della v2.1.210, il `reason` di negazione veniva restituito a Claude come errore dello strumento e il turno continuava
  * `PostToolUse`: per impostazione predefinita il turno termina e il `reason` appare nella chat come una riga di avviso. Impostate `continueOnBlock: true` per alimentare il `reason` di nuovo a Claude e continuare il turno invece
  * `PostToolBatch`, `UserPromptSubmit` e `UserPromptExpansion`: il turno termina e il `reason` appare nella chat come una riga di avviso

Questo esempio utilizza un hook `Stop` per chiedere al modello se tutti i compiti richiesti sono completi. Se il modello restituisce `"ok": false` perché la condizione non è ancora soddisfatta, Claude continua a lavorare e utilizza il `reason` come sua prossima istruzione:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "Check if all tasks are complete. If not, respond with {\"ok\": false, \"reason\": \"what remains to be done\"}."
          }
        ]
      }
    ]
  }
}
```

Per le opzioni di configurazione complete, consultate [Hooks basati su prompt](/docs/it/hooks#prompt-based-hooks) nel riferimento.

<h2 id="agent-based-hooks">
  Hooks basati su agenti
</h2>

<Warning>
  Gli agent hooks sono sperimentali. Il comportamento e la configurazione potrebbero cambiare nelle versioni future. Per i flussi di lavoro di produzione, preferite gli [hooks di comando](/docs/it/hooks#command-hook-fields).
</Warning>

Quando la verifica richiede l'ispezione di file o l'esecuzione di comandi, utilizzate gli hook `type: "agent"`. A differenza degli hook di prompt che effettuano una singola chiamata LLM, gli hook di agenti generano un subagent che può leggere file, cercare codice e utilizzare altri strumenti per verificare le condizioni prima di restituire una decisione.

Gli hook di agenti utilizzano il formato di risposta `"ok"` / `"reason"` con un timeout predefinito più lungo di 60 secondi e fino a 50 turni di utilizzo dello strumento. Non supportano il campo `impossible` degli hook di prompt. Su `ok: false`, Claude Code gestisce un agent hook nello stesso modo in cui gestisce un hook di prompt con `continueOnBlock: true` sullo stesso evento, quindi su `PreToolUse` e `PostToolUse` il turno continua; gli agent hook non hanno un campo `continueOnBlock`. Consultate la [configurazione degli agent hook](/docs/it/hooks#agent-hook-configuration) per i campi, incluso il placeholder `$ARGUMENTS` che Claude Code sostituisce con l'input JSON dell'hook.

Questo esempio verifica che i test passino prima di consentire a Claude di fermarsi:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "agent",
            "prompt": "Verify that all unit tests pass. Run the test suite and check the results. $ARGUMENTS",
            "timeout": 120
          }
        ]
      }
    ]
  }
}
```

Utilizzate gli hook di prompt quando i dati di input dell'hook da soli sono sufficienti per prendere una decisione. Utilizzate gli hook di agenti quando avete bisogno di verificare qualcosa rispetto allo stato effettivo della base di codice.

Per le opzioni di configurazione complete, consultate [Hooks basati su agenti](/docs/it/hooks#agent-based-hooks) nel riferimento.

<h2 id="http-hooks">
  HTTP hooks
</h2>

Utilizzate gli hook `type: "http"` per POST dei dati dell'evento a un endpoint HTTP invece di eseguire un comando shell. L'endpoint riceve lo stesso JSON che un hook di comando riceverebbe su stdin, e restituisce i risultati attraverso il corpo della risposta HTTP utilizzando lo stesso formato JSON.

Gli HTTP hooks sono utili quando volete che un server web, una funzione cloud o un servizio esterno gestisca la logica dell'hook: ad esempio, un servizio di controllo condiviso che registra gli eventi di utilizzo dello strumento in un team.

Questo esempio pubblica ogni utilizzo dello strumento a un servizio di registrazione locale:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "http",
            "url": "http://localhost:8080/hooks/tool-use",
            "headers": {
              "Authorization": "Bearer $MY_TOKEN"
            },
            "allowedEnvVars": ["MY_TOKEN"]
          }
        ]
      }
    ]
  }
}
```

L'endpoint dovrebbe restituire un corpo di risposta JSON utilizzando lo stesso [formato di output](/docs/it/hooks#json-output) degli hook di comando. Per bloccare una chiamata di strumento, restituite una risposta 2xx con i campi `hookSpecificOutput` appropriati. I codici di stato HTTP da soli non possono bloccare le azioni.

I valori dell'intestazione supportano l'interpolazione delle variabili di ambiente utilizzando la sintassi `$VAR_NAME` o `${VAR_NAME}`. Solo le variabili elencate nell'array `allowedEnvVars` vengono risolte; tutti gli altri riferimenti `$VAR` rimangono vuoti.

Per le opzioni di configurazione complete e la gestione delle risposte, consultate [HTTP hooks](/docs/it/hooks#http-hook-fields) nel riferimento.

<h2 id="limitations-and-troubleshooting">
  Limitazioni e risoluzione dei problemi
</h2>

<h3 id="limitations">
  Limitazioni
</h3>

Tenete presenti questi vincoli quando progettate gli hook:

* Gli hook di comando comunicano solo attraverso stdout, stderr e codici di uscita. Non possono attivare comandi `/` o chiamate di strumenti. Il testo restituito tramite `additionalContext` viene iniettato come un promemoria di sistema che Claude legge come testo semplice. Gli HTTP hooks comunicano attraverso il corpo della risposta invece.
* I timeout dell'hook variano in base al tipo. Sovrascrivete per hook con il campo `timeout` in secondi.
  * `command`, `http`, `mcp_tool`: 10 minuti. Claude Code riduce questo valore predefinito a 30 secondi per gli hook `UserPromptSubmit`, `PreModelSwitch` e `PostModelSwitch`, e a 10 secondi per `MessageDisplay`.
  * `prompt`: 30 secondi.
  * `agent`: 60 secondi.
  * Gli hook [`SessionEnd`](/docs/it/hooks#sessionend) di qualsiasi tipo condividono un budget di 1,5 secondi. Se le vostre impostazioni impostano un `timeout` per hook più lungo, Claude Code aumenta il budget per corrispondere, fino a 60 secondi.
* Gli hook `PostToolUse` non possono annullare le azioni poiché lo strumento è già stato eseguito.
* Gli hook `PermissionRequest` si attivano quando Claude Code sta per chiedervi il permesso.
  * In [modalità non interattiva](/docs/it/headless) con il flag `-p`, quel prompt esiste solo quando il callback [`canUseTool`](/docs/it/agent-sdk/permissions) dell'Agent SDK lo fornisce. Nelle esecuzioni `-p` semplici o con `--permission-prompt-tool`, utilizzate gli hook `PreToolUse` per le decisioni di autorizzazione automatizzate.
  * I subagent in background non possono mostrare un prompt in modalità non interattiva. Claude Code esegue comunque gli hook per le loro chiamate di strumenti, e se nessun hook restituisce una decisione, nega la chiamata. In una sessione interattiva, i prompt dei subagent in background vengono visualizzati nella vostra sessione principale e gli hook si attivano come al solito.
* Gli hook `Stop` si attivano ogni volta che Claude finisce di rispondere, non solo al completamento dell'attività. Non si attivano su interruzioni dell'utente. Gli errori API attivano [StopFailure](/docs/it/hooks#stopfailure) invece.
* Quando più hook `PreToolUse` restituiscono [`updatedInput`](/docs/it/hooks#pretooluse) per riscrivere gli argomenti di uno strumento, l'ultimo a terminare vince. Poiché gli hook si eseguono in parallelo, l'ordine è non deterministico. Evitate di avere più di un hook che modifica l'input dello stesso strumento.

<h3 id="hooks-and-permission-modes">
  Hooks e modalità di autorizzazione
</h3>

Gli hook `PreToolUse` si attivano prima di qualsiasi controllo della modalità di autorizzazione, in ogni [modalità di autorizzazione](/docs/it/permission-modes), inclusa `dontAsk`. Un hook che restituisce `permissionDecision: "deny"` blocca lo strumento anche in modalità `bypassPermissions` o con `--dangerously-skip-permissions`. Questo vi permette di applicare una policy che gli utenti non possono aggirare cambiando la loro modalità di autorizzazione.

Il contrario non è vero: un hook che restituisce `"allow"` non aggira le regole di negazione dalle impostazioni, e non può sopprimere il prompt per gli strumenti MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool) o per gli strumenti connettore [che la vostra organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) nelle sessioni in cui quella impostazione raggiunge Claude Code. Gli hook possono stringere le restrizioni ma non allentarle oltre quello che le regole di autorizzazione consentono.

<h3 id="hook-not-firing">
  Hook non si attiva
</h3>

L'hook è configurato ma non si esegue mai.

* Eseguite `/hooks` e confermate che l'hook appare sotto l'evento corretto
* Controllate che il modello del matcher corrisponda esattamente al nome dello strumento. I matcher sono sensibili alle maiuscole
* Verificate che state attivando il tipo di evento corretto: `PreToolUse` si attiva prima dell'esecuzione dello strumento, `PostToolUse` si attiva dopo. Un hook `PermissionRequest` si attiva quando Claude Code sta per chiedervi il permesso; vedete le [limitazioni](#limitations) per i casi non interattivi

<h3 id="hook-error-in-output">
  Errore dell'hook nell'output
</h3>

Vedete un messaggio come "PreToolUse hook error: ..." nella trascrizione.

* Il vostro script è uscito con un codice diverso da zero inaspettatamente. Testatelo manualmente inviando JSON di esempio:
  ```bash theme={null}
  echo '{"tool_name":"Bash","tool_input":{"command":"ls"}}' | ./my-hook.sh
  echo $?  # Check the exit code
  ```
* Se vedete "command not found", utilizzate percorsi assoluti o `${CLAUDE_PROJECT_DIR}` per fare riferimento agli script. Per evitare completamente le virgolette della shell, aggiungete `"args": []` per passare alla [forma exec](/docs/it/hooks#exec-form-and-shell-form), che genera lo script direttamente senza una shell
* Se vedete "jq: command not found", installate `jq` o utilizzate Python/Node.js per l'analisi JSON
* Se l'avviso mostra un messaggio di convalida JSON, l'stdout del vostro hook è stato analizzato come JSON ma ha fallito la convalida dello schema. Se mostra un messaggio di analisi JSON, l'stdout sembrava un oggetto JSON ma non era JSON valido. Entrambi accadono anche su exit 0.

  Per risolvere un errore di analisi, costruite il payload con un codificatore JSON come `jq` invece della concatenazione di stringhe, in modo che le virgolette e i backslash all'interno dei valori siano sfuggiti. La sezione [Exit code output](/docs/it/hooks#exit-code-output) del riferimento copre le combinazioni di codice di uscita e JSON
* Se lo script non si esegue affatto, rendetelo eseguibile: `chmod +x ./my-hook.sh`

<h3 id="/hooks-shows-no-hooks-configured">
  `/hooks` non mostra hook configurati
</h3>

Avete modificato un file di impostazioni ma gli hooks non appaiono nel menu.

* Le modifiche ai file vengono normalmente raccolte automaticamente. Se non sono apparse dopo alcuni secondi, il file watcher potrebbe aver perso il cambiamento: riavviate la vostra sessione per forzare un ricaricamento.
* Verificate che il vostro JSON sia valido: le virgole finali e i commenti non sono consentiti
* Confermate che il file di impostazioni è nella posizione corretta: `.claude/settings.json` per gli hook del progetto, `~/.claude/settings.json` per gli hook globali

<h3 id="stop-hook-hits-the-block-cap">
  L'hook Stop colpisce il limite di blocco
</h3>

Claude continua a lavorare invece di fermarsi, quindi termina il turno con un avviso che l'hook Stop ha bloccato troppe volte consecutive.

Claude Code ignora un hook Stop dopo che ha bloccato otto volte di fila senza progresso. Il vostro script di hook deve controllare se ha già attivato una continuazione. Analizzate il campo `stop_hook_active` dall'input JSON e uscite presto se è `true`:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
if [ "$(echo "$INPUT" | jq -r '.stop_hook_active')" = "true" ]; then
  exit 0  # Allow Claude to stop
fi
# ... rest of your hook logic
```

Se il vostro hook ha legittimamente bisogno di più di otto iterazioni per convergere, aumentate il limite con [`CLAUDE_CODE_STOP_HOOK_BLOCK_CAP`](/docs/it/env-vars).

<h3 id="hook-json-has-no-effect">
  Hook JSON non ha effetto
</h3>

Il vostro hook stampa JSON valido, ma la decisione non ha effetto e nessun errore appare nella trascrizione. Controllate quale causa si applica:

* **Output extra prima del JSON**: qualcos'altro scrive su stdout per primo, solitamente un `echo` incondizionato nel vostro profilo shell, quindi l'output non inizia più con `{` e Claude Code non lo analizza come JSON. La causa e la soluzione seguono questo elenco.
* **Un campo al livello sbagliato**: confrontate il posizionamento di ogni campo rispetto al formato [JSON output](/docs/it/hooks#json-output). Ad esempio, `permissionDecision` appartiene dentro `hookSpecificOutput`, non al livello superiore.

Quando Claude Code esegue un hook di comando in forma shell, uno senza `args`, genera `sh -c` su macOS e Linux, Git Bash su Windows, o PowerShell quando Git Bash non è installato per impostazione predefinita. Questa shell è non interattiva, ma Git Bash e alcune configurazioni, come `BASH_ENV` che punta a `~/.bashrc`, comunque forniscono il vostro profilo. Se quel profilo contiene istruzioni `echo` incondizionate, l'output viene anteposto al vostro JSON dell'hook:

```text theme={null}
Shell ready on arm64
{"decision": "block", "reason": "Not allowed"}
```

L'output combinato non inizia più con `{`, quindi Claude Code tratta tutto stdout come testo semplice e ignora il JSON. Su exit 0 nulla viene segnalato nella trascrizione; il tentativo di analisi viene registrato solo nel [debug log](/docs/it/hooks#debug-hooks). Per risolvere questo, avvolgete le istruzioni echo nel vostro profilo shell in modo che si eseguano solo in shell interattive:

```bash theme={null}
# In ~/.zshrc or ~/.bashrc
if [[ $- == *i* ]]; then
  echo "Shell ready"
fi
```

La variabile `$-` contiene i flag della shell, e `i` significa interattiva. Gli hooks si eseguono in shell non interattive, quindi l'echo viene saltato.

Quando il vostro hook restituisce `permissionDecision` o `additionalContext` al livello superiore invece che dentro `hookSpecificOutput`, il JSON comunque si analizza, e Claude Code ignora i campi posizionati male senza segnalare un errore. Per vedere quali campi ha ignorato, avviate Claude Code con `claude --debug` e cercate nel [debug log](/docs/it/hooks#debug-hooks) `Hook JSON output had unrecognized keys`.

<h3 id="debug-techniques">
  Tecniche di debug
</h3>

Premete `Ctrl+O` per aprire la vista della trascrizione per controllare il risultato di un'esecuzione dell'hook:

* **Esecuzione riuscita**: non vedete nulla, a meno che il JSON dell'hook non faccia emergere qualcosa, come `systemMessage` o feedback dell'hook Stop.
  * Per confermare che un hook è stato eseguito, controllate il suo effetto, come un file riformattato, o attivate la registrazione di debug come descritto di seguito e attivate di nuovo l'hook
* **Errore di blocco**: nella maggior parte degli eventi vedete il feedback dell'hook. Quando il JSON dell'hook ha preso una decisione di blocco, il feedback è il motivo da quella decisione; altrimenti è lo stderr dell'hook. Su alcuni eventi, come `ConfigChange` e `Elicitation`, un blocco non mostra alcun messaggio.
* **Errore non bloccante**: l'azione è proceduta, e vedete un avviso `<hook name> hook error` con una breve spiegazione, come la prima riga di stderr con il prefisso `Failed with non-blocking status code:`, o un messaggio di convalida JSON o di analisi.

Quali combinazioni di codice di uscita e JSON producono ogni risultato, incluse le eccezioni per evento, è definito nella sezione [Exit code output](/docs/it/hooks#exit-code-output) del riferimento.

Per i dettagli di esecuzione completi incluso quali hook hanno corrisposto, i loro codici di uscita, stdout e stderr, leggete il debug log. Avviate Claude Code con `claude --debug-file /tmp/claude.log` per scrivere in un percorso noto, quindi `tail -f /tmp/claude.log` in un altro terminale. Se avete avviato senza quel flag, eseguite `/debug` a metà sessione per abilitare la registrazione e trovare il percorso del log.

<h2 id="learn-more">
  Ulteriori informazioni
</h2>

* [Riferimento Hooks](/docs/it/hooks): schemi di eventi completi, formato di output JSON, hooks asincroni e hooks di strumenti MCP
* [Considerazioni sulla sicurezza](/docs/it/hooks#security-considerations): esaminate prima di distribuire gli hooks in ambienti condivisi o di produzione
* [Esempio di validatore di comandi Bash](https://github.com/anthropics/claude-code/blob/main/examples/hooks/bash_command_validator_example.py): implementazione di riferimento completa
