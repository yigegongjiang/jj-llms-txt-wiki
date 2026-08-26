> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Consiglia il tuo plugin dalla tua CLI

> Emetti un marcatore su una riga dalla tua CLI in modo che Claude Code chieda agli utenti di installare il tuo plugin ufficiale.

Se mantieni una CLI o SDK e hai un plugin nel marketplace ufficiale di Anthropic, il tuo strumento può chiedere agli utenti di Claude Code di installare quel plugin. La tua CLI scrive un marcatore su una riga in stderr quando rileva che è in esecuzione all'interno di Claude Code. Claude Code legge il marcatore, lo rimuove dall'output e mostra all'utente un prompt di installazione una sola volta.

Claude Code rimuove la riga di suggerimento dall'output del comando prima di inviarlo al modello, quindi il marcatore non appare mai nella conversazione e non viene conteggiato verso l'utilizzo dei token. Il protocollo non richiede comandi extra e non cambia ciò che la tua CLI stampa per gli utenti al di fuori di Claude Code.

Questa pagina è per i manutentori di CLI e SDK. Se stai cercando di installare plugin, vedi [Scopri e installa plugin](/docs/it/discover-plugins).

<h2 id="how-it-works">
  Come funziona
</h2>

Claude Code imposta la variabile di ambiente [`CLAUDECODE`](/docs/it/env-vars) a `1` per ogni comando che esegue attraverso gli strumenti Bash e PowerShell, e per i comandi [hook](/docs/it/hooks). A partire dalla versione 2.1.172 imposta anche [`CLAUDE_CODE_CHILD_SESSION`](/docs/it/env-vars) a `1` negli stessi sottoprocessi. Quando la tua CLI vede una di queste variabili, scrive un tag `<claude-code-hint />` auto-chiudente in stderr. Nei comandi hook il tag di suggerimento viene rimosso e ignorato. Solo l'output degli strumenti Bash e PowerShell attiva il prompt di installazione.

Quando Claude Code riceve l'output del comando, esegue le seguenti operazioni:

1. Scansiona le righe di suggerimento e le rimuove prima che l'output raggiunga il modello
2. Verifica che il suggerimento sia destinato a un plugin in un marketplace ufficiale di Anthropic
3. Verifica che il plugin non sia già installato e che non sia stato suggerito in precedenza
4. Mostra all'utente un prompt di installazione che nomina il comando che ha emesso il suggerimento

Claude Code non installa mai un plugin automaticamente. L'utente conferma sempre.

<h2 id="emit-the-hint">
  Emetti il suggerimento
</h2>

I suggerimenti hint vengono attivati solo per i plugin elencati nel marketplace ufficiale di Anthropic. Vedi [Inserisci il tuo plugin nel marketplace ufficiale](#get-your-plugin-into-the-official-marketplace) prima di distribuire l'integrazione.

Condiziona l'emissione su una variabile di ambiente in modo che il marcatore sia improbabile che appaia quando un utente esegue direttamente la tua CLI, quindi scrivi il tag su stderr sulla sua propria riga. Scegli quale variabile controllare:

* `CLAUDECODE`: impostata su ogni versione di Claude Code, quindi raggiunge il maggior numero di sessioni. È anche impostata nelle sessioni tmux e nei sottoprocessi del server MCP stdio che Claude Code avvia. Le estensioni IDE la impostano anche nei loro terminali integrati, dove un utente potrebbe eseguire direttamente la tua CLI.
* `CLAUDE_CODE_CHILD_SESSION`: impostata solo nei sottoprocessi che Claude Code stesso genera, come le chiamate di strumenti, i comandi hook e i comandi della [riga di stato](/docs/it/statusline), quindi il tag normalmente non raggiunge un terminale umano. Un processo di lunga durata che è stato avviato all'interno di una sessione, come un server tmux, acquisisce la variabile, quindi le shell avviate successivamente da quel processo mostrano comunque il tag grezzo. Richiede Claude Code v2.1.172 o successivo, quindi le sessioni su versioni precedenti non ricevono il suggerimento.

I seguenti esempi condizionano su `CLAUDECODE` per la massima copertura e emettono un suggerimento per un plugin denominato `example-cli` nel marketplace ufficiale:

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
  [ -n "$CLAUDECODE" ] &&
    printf '%s\n' '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />' >&2
  ```
</CodeGroup>

Sostituisci `example-cli` con il nome del tuo plugin nel marketplace ufficiale.

<h2 id="choose-where-to-emit">
  Scegli dove emettere
</h2>

Controlli quali percorsi di codice emettono il suggerimento. Claude Code deduplica per plugin, quindi emettere ad ogni invocazione non ha alcun aspetto negativo. I punti di contatto che funzionano bene includono:

| Posizionamento                         | Perché funziona                                                    |
| :------------------------------------- | :----------------------------------------------------------------- |
| Output di `--help`                     | Claude spesso esegue help quando esplora una CLI sconosciuta       |
| Errori di sottocomando sconosciuto     | Raggiunge il momento in cui Claude è confuso sulla tua interfaccia |
| Accesso o successo dell'autenticazione | L'utente è già in uno stato mentale di configurazione              |
| Messaggio di benvenuto al primo avvio  | Un momento di onboarding naturale                                  |

<h2 id="what-the-user-sees">
  Cosa vede l'utente
</h2>

Quando il suggerimento supera tutti i controlli, Claude Code mostra un prompt come il seguente:

```text theme={null}
─────────────────────────────────────────────────────────────
  Raccomandazione Plugin

    Il comando example-cli suggerisce di installare un plugin.

    Plugin: example-cli
    Marketplace: claude-plugins-official
    Integrazione ufficiale per distribuzioni example-cli

    Desideri installarlo?
    ❯ 1. Sì, installa example-cli
      2. No
      3. No, e non mostrare più suggerimenti di installazione plugin

─────────────────────────────────────────────────────────────
```

Il prompt nomina il comando che ha prodotto il suggerimento in modo che gli utenti possano individuare una mancata corrispondenza tra lo strumento e il plugin che consiglia. Se l'utente non risponde entro 30 secondi, il prompt viene chiuso come **No**.

La frequenza del prompt è limitata:

* **Una volta per plugin**: dopo che il prompt viene mostrato, Claude Code registra il plugin e non lo suggerisce mai più, indipendentemente dalla risposta dell'utente.
* **Una volta per sessione**: su tutte le CLI della macchina, al massimo un suggerimento di prompt appare per sessione di Claude Code.

Selezionando **Sì** installa il plugin nell'ambito dell'utente. Selezionando **No, e non mostrare più suggerimenti di installazione plugin** disabilita tutti i futuri suggerimenti di prompt per l'utente.

<h2 id="hint-format">
  Formato del suggerimento
</h2>

Il suggerimento è un tag auto-chiudente con tre attributi obbligatori.

```text theme={null}
<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />
```

| Attributo | Obbligatorio | Descrizione                                                |
| :-------- | :----------- | :--------------------------------------------------------- |
| `v`       | Sì           | Versione del protocollo. `1` è l'unico valore supportato   |
| `type`    | Sì           | Tipo di suggerimento. `plugin` è l'unico valore supportato |
| `value`   | Sì           | Identificatore del plugin nella forma `name@marketplace`   |

I valori degli attributi possono essere racchiusi tra virgolette doppie o lasciati senza virgolette. I valori senza virgolette non possono contenere spazi. Le sequenze di escape non sono supportate.

<h2 id="requirements">
  Requisiti
</h2>

Claude Code applica due condizioni prima di agire su un suggerimento. I suggerimenti che non superano nessuno dei due controlli vengono scartati:

* **Riga propria**: il tag deve occupare la sua propria riga. Un tag incorporato a metà riga, ad esempio all'interno di un'istruzione di log, viene ignorato. Gli spazi bianchi iniziali e finali sulla riga sono consentiti.
* **Marketplace ufficiale**: il `value` deve fare riferimento a un plugin in un marketplace controllato da Anthropic come `claude-plugins-official`. I suggerimenti che puntano ad altri marketplace vengono silenziosamente scartati.

La riga di suggerimento viene sempre rimossa dall'output prima che raggiunga il modello, anche quando la versione o il tipo non è riconosciuto, quindi il marcatore non viene mai conteggiato verso l'utilizzo dei token.

Le linee guida rimanenti sono consigliate ma non applicate. Claude Code non può osservare se la tua CLI le segue:

* **Scrivi in stderr**: stderr mantiene il tag fuori dalle pipeline di shell come `example-cli deploy | jq`. Claude Code scansiona entrambi i flussi, quindi anche stdout funziona.
* **Condiziona su una variabile di ambiente**: emetti solo quando `CLAUDECODE` o `CLAUDE_CODE_CHILD_SESSION` è impostato. Vedi [Emetti il suggerimento](#emit-the-hint) per come i due variabili differiscono.

<h2 id="get-your-plugin-into-the-official-marketplace">
  Inserisci il tuo plugin nel marketplace ufficiale
</h2>

Il protocollo di suggerimento ha effetto solo per i plugin elencati nel marketplace ufficiale di Anthropic, `claude-plugins-official`. Anthropic cura quel marketplace a sua discrezione, e i moduli di invio in-app aggiungono plugin al [marketplace della comunità](/docs/it/plugins#submit-your-plugin-to-the-community-marketplace) invece, che il protocollo di suggerimento non controlla. Se stai lavorando con un contatto partner di Anthropic, contattalo per coordinare un elenco nel marketplace ufficiale.

<h2 id="see-also">
  Vedi anche
</h2>

* [Crea plugin](/docs/it/plugins): costruisci il plugin che la tua CLI consiglia
* [Crea e distribuisci un marketplace di plugin](/docs/it/plugin-marketplaces): ospita plugin al di fuori del marketplace ufficiale
* [Variabili di ambiente](/docs/it/env-vars): riferimento completo per `CLAUDECODE` e variabili correlate
