> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Consiglia il tuo plugin dalla tua CLI

> Invita gli utenti di Claude Code a installare il tuo plugin del marketplace ufficiale emettendo un tag claude-code-hint dalla tua CLI o SDK.

Se gestisci una CLI o un SDK, il tuo strumento può invitare gli utenti di Claude Code a installare il tuo plugin. Quando la tua CLI rileva che è in esecuzione all'interno di Claude Code, fai in modo che scriva un tag `<claude-code-hint />` su una singola riga verso stderr. Claude Code rimuove la riga dall'output dello strumento Bash e PowerShell prima che il modello veda l'output, quindi mostra all'utente un prompt di installazione una sola volta.

Questa pagina si applica solo se il tuo plugin è elencato in `claude-plugins-official` o in un altro marketplace con uno dei [nomi ufficiali del marketplace](/docs/it/plugins/security#official-marketplace-names) di Anthropic. Il marketplace della comunità, `claude-community`, non è uno di questi.

<Note>
  Per pubblicare un plugin, vedi [Pubblica e distribuisci un plugin](/docs/it/plugins/publish).
</Note>

<h2 id="emit-the-hint">
  Emetti l'hint
</h2>

Emetti il tag solo quando `CLAUDECODE` o `CLAUDE_CODE_CHILD_SESSION` è impostato, in modo che non appaia quando una persona esegue la tua CLI direttamente.

Claude Code imposta `CLAUDECODE=1` nei comandi che esegue attraverso gli strumenti Bash e PowerShell e nei comandi hook. Dalla versione 2.1.172 in poi imposta anche `CLAUDE_CODE_CHILD_SESSION=1` lì. Le variabili differiscono nei processi che le portano:

* **`CLAUDECODE`**: impostato da ogni versione di Claude Code. Le estensioni IDE lo impostano anche nei loro terminali integrati, quindi un gate su `CLAUDECODE` da solo emette il tag anche quando una persona esegue la tua CLI direttamente in uno di questi terminali
* **`CLAUDE_CODE_CHILD_SESSION`**: impostato solo nei sottoprocessi che Claude Code stesso avvia. Usalo quando puoi richiedere la versione 2.1.172 o successiva

Il [riferimento delle variabili di ambiente](/docs/it/env-vars) contiene i dettagli.

I seguenti esempi usano un gate su `CLAUDECODE` per la massima portata ed emettono un hint per un plugin denominato `example-cli` nel marketplace ufficiale:

<CodeGroup>
  ```javascript Node.js theme={null}
  if (process.env.CLAUDECODE) {
    process.stderr.write(
      '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />\n',
    )
  }
  ```

  ```python Python theme={null}
  import os, sys

  if os.environ.get("CLAUDECODE"):
      print(
          '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />',
          file=sys.stderr,
      )
  ```

  ```go Go theme={null}
  if os.Getenv("CLAUDECODE") != "" {
      fmt.Fprintln(os.Stderr,
          `<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />`)
  }
  ```

  ```shell Shell theme={null}
  if [ -n "$CLAUDECODE" ]; then
    printf '%s\n' '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />' >&2
  fi
  ```
</CodeGroup>

Sostituisci `example-cli` con il nome del tuo plugin nel marketplace ufficiale.

Puoi emettere l'hint ad ogni invocazione, perché Claude Code chiede per ogni plugin una sola volta.

Per verificare l'emettitore, esegui `CLAUDECODE=1 example-cli` in un terminale e conferma che la riga del tag appaia su stderr, quindi esegui `example-cli` senza la variabile e conferma che non stampi nulla di extra.

<h2 id="hint-format">
  Formato dell'hint
</h2>

Il tag deve occupare la sua propria riga; Claude Code ignora un tag incorporato a metà riga.

Il tag accetta tre attributi, tutti obbligatori:

| Attributo | Descrizione                                              |
| :-------- | :------------------------------------------------------- |
| `v`       | Versione del protocollo. `1` è l'unico valore supportato |
| `type`    | Tipo di hint. `plugin` è l'unico valore supportato       |
| `value`   | Identificatore del plugin nella forma `name@marketplace` |

I valori possono essere tra virgolette doppie o senza virgolette; un valore senza virgolette non può contenere spazi.

Claude Code rimuove la riga dall'output anche quando `v` o `type` non è riconosciuto.

<h2 id="check-when-the-prompt-appears">
  Verifica quando appare il prompt
</h2>

Il prompt appare solo nelle sessioni di terminale interattive. Nelle esecuzioni `claude -p`, nelle esecuzioni di subagent e nell'output dei comandi hook, il tag viene rimosso e nessun prompt viene mostrato. Tutti questi controlli devono anche passare:

* **Ufficiale e installabile**: `value` nomina un plugin che Claude Code trova nella sua copia locale di un marketplace ufficiale, che non è già installato e che nessuna policy blocca
* **Analytics attivato**: una sessione in cui gli analytics di Claude Code sono disattivati non chiede mai, ad esempio una con `DISABLE_TELEMETRY`, `DO_NOT_TRACK` o `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` impostato, o una su un provider di terze parti come Amazon Bedrock, dove si applica l'[opt-out automatico della telemetria](/docs/it/data-usage#default-behaviors-by-api-provider)
* **Limiti di frequenza**: un prompt per sessione, un prompt mai per plugin indipendentemente dalla risposta dell'utente, e nessuno una volta che 100 plugin sono stati richiesti su quella macchina
* **Non disattivato**: l'utente non ha scelto **No, e non mostrare più gli hint di installazione dei plugin**
* **Sessione locale e presidiata**: lo spazio di lavoro della sessione è locale piuttosto che su una macchina cloud o remota, e la sessione non è in esecuzione incustodita. Ad esempio, una sessione avviata con `--cloud`, una che serve Remote Control, o un compagno di squadra agent-team non chiede mai

<h2 id="preview-what-the-user-sees">
  Anteprima di ciò che vede l'utente
</h2>

Quando i controlli in [Verifica quando appare il prompt](#check-when-the-prompt-appears) passano, Claude Code mostra una finestra di dialogo **Plugin recommendation** come la seguente:

```text theme={null}
─────────────────────────────────────────────────────────────
  Plugin recommendation

    The example-cli command suggests installing a plugin.

    Plugin: example-cli
    Marketplace: claude-plugins-official
    Description: Official integration for example-cli deployments

    Would you like to install it?
    ❯ 1. Yes, install
      2. No
      3. No, and don't show plugin installation hints again

─────────────────────────────────────────────────────────────
```

La finestra di dialogo nomina la prima parola del comando shell che Claude ha eseguito, in modo che gli utenti possano individuare una mancata corrispondenza. Ogni risposta ha un effetto:

* **Yes, install**: installa il plugin a [user scope](/docs/it/plugins/install)
* **No, and don't show plugin installation hints again**: disattiva i futuri prompt di hint per quell'utente
* **Nessuna risposta per 30 secondi**: conta come **No**

<h2 id="next-steps">
  Passaggi successivi
</h2>

* [Pubblica e distribuisci un plugin](/docs/it/plugins/publish): i percorsi in ogni marketplace, incluso il marketplace ufficiale, che l'hint richiede
* [Riferimento dei comandi dei plugin](/docs/it/plugins/cli-reference#plugin-install): il comando shell che installa lo stesso plugin al di fuori di una sessione
