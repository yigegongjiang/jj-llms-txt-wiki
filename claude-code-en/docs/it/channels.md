> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Invia eventi in una sessione in esecuzione con i canali

> Utilizza i canali per inviare messaggi, avvisi e webhook nella tua sessione Claude Code da un server MCP. Inoltra i risultati CI, i messaggi di chat e gli eventi di monitoraggio in modo che Claude possa reagire mentre sei assente.

<Note>
  I canali sono in [anteprima di ricerca](#research-preview). Richiedono l'autenticazione Anthropic tramite claude.ai o una chiave API Console, e non sono disponibili su Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry. Le organizzazioni Team ed Enterprise devono [abilitarli esplicitamente](#enterprise-controls).
</Note>

Un canale è un server MCP che invia eventi nella tua sessione Claude Code in esecuzione, in modo che Claude possa reagire alle cose che accadono mentre non sei al terminale. I canali possono essere bidirezionali: Claude legge l'evento e risponde attraverso lo stesso canale, come un ponte di chat. Gli eventi arrivano solo mentre la sessione è aperta, quindi per una configurazione sempre attiva esegui Claude in un processo in background o in un terminale persistente.

A differenza delle integrazioni che avviano una nuova sessione cloud o attendono di essere interrogate, l'evento arriva nella sessione che hai già aperto: vedi [come si confrontano i canali](#how-channels-compare).

Installi un canale come plugin e lo configuri con le tue credenziali. Telegram, Discord e iMessage sono inclusi nell'anteprima di ricerca.

Quando Claude risponde attraverso un canale, vedi il messaggio in arrivo nel tuo terminale ma non il testo della risposta. Il terminale mostra la chiamata dello strumento e una conferma (come "inviato"), e la risposta effettiva appare sull'altra piattaforma.

Se gestisci un'organizzazione Team, Enterprise o Console, vedi [Abilita i canali per la tua organizzazione](#enterprise-controls). Per creare il tuo canale, vedi il [riferimento Canali](/docs/it/channels-reference).

<h2 id="supported-channels">
  Canali supportati
</h2>

Ogni canale supportato è un plugin che richiede [Bun](https://bun.sh). Per una demo pratica del flusso del plugin prima di connettere una piattaforma reale, prova la [guida rapida fakechat](#quickstart).

<Tabs>
  <Tab title="Telegram">
    Visualizza il [codice sorgente del plugin Telegram](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram) completo.

    <Steps>
      <Step title="Crea un bot Telegram">
        Apri [BotFather](https://t.me/BotFather) in Telegram e invia `/newbot`. Dagli un nome visualizzato e un nome utente univoco che termina con `bot`. Copia il token che BotFather restituisce.
      </Step>

      <Step title="Installa il plugin">
        In Claude Code, esegui:

        ```
        /plugin install telegram@claude-plugins-official
        ```

        Se Claude Code segnala che il plugin non si trova in alcun marketplace, il tuo marketplace è mancante o obsoleto. Esegui `/plugin marketplace update claude-plugins-official` per aggiornarlo, oppure `/plugin marketplace add anthropics/claude-plugins-official` se non l'hai ancora aggiunto. Quindi riprova l'installazione.

        Dopo l'installazione, esegui `/reload-plugins` per attivare il comando di configurazione del plugin.
      </Step>

      <Step title="Configura il tuo token">
        Esegui il comando di configurazione con il token da BotFather:

        ```
        /telegram:configure <token>
        ```

        Questo lo salva in `~/.claude/channels/telegram/.env`. Puoi anche impostare `TELEGRAM_BOT_TOKEN` nel tuo ambiente shell prima di avviare Claude Code.
      </Step>

      <Step title="Riavvia con i canali abilitati">
        Esci da Claude Code e riavvia con il flag del canale. Questo avvia il plugin Telegram, che inizia a eseguire il polling dei messaggi dal tuo bot:

        ```bash theme={null}
        claude --channels plugin:telegram@claude-plugins-official
        ```
      </Step>

      <Step title="Accoppia il tuo account">
        Apri Telegram e invia qualsiasi messaggio al tuo bot. Il bot risponde con un codice di accoppiamento.

        <Note>Se il tuo bot non risponde, assicurati che Claude Code sia in esecuzione con `--channels` dal passaggio precedente. Il bot può rispondere solo mentre il canale è attivo.</Note>

        Torna in Claude Code ed esegui:

        ```
        /telegram:access pair <code>
        ```

        Quindi blocca l'accesso in modo che solo il tuo account possa inviare messaggi:

        ```
        /telegram:access policy allowlist
        ```
      </Step>
    </Steps>
  </Tab>

  <Tab title="Discord">
    Visualizza il [codice sorgente del plugin Discord](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord) completo.

    <Steps>
      <Step title="Crea un bot Discord">
        Vai al [Discord Developer Portal](https://discord.com/developers/applications), fai clic su **New Application** e assegnagli un nome. Nella sezione **Bot**, crea un nome utente, quindi fai clic su **Reset Token** e copia il token.
      </Step>

      <Step title="Abilita Message Content Intent">
        Nelle impostazioni del tuo bot, scorri fino a **Privileged Gateway Intents** e abilita **Message Content Intent**.
      </Step>

      <Step title="Invita il bot al tuo server">
        Vai a **OAuth2 > URL Generator**. Seleziona l'ambito `bot` e abilita questi permessi:

        * View Channels
        * Send Messages
        * Send Messages in Threads
        * Read Message History
        * Attach Files
        * Add Reactions

        Apri l'URL generato per aggiungere il bot al tuo server.
      </Step>

      <Step title="Installa il plugin">
        In Claude Code, esegui:

        ```
        /plugin install discord@claude-plugins-official
        ```

        Se Claude Code segnala che il plugin non si trova in alcun marketplace, il tuo marketplace è mancante o obsoleto. Esegui `/plugin marketplace update claude-plugins-official` per aggiornarlo, oppure `/plugin marketplace add anthropics/claude-plugins-official` se non l'hai ancora aggiunto. Quindi riprova l'installazione.

        Dopo l'installazione, esegui `/reload-plugins` per attivare il comando di configurazione del plugin.
      </Step>

      <Step title="Configura il tuo token">
        Esegui il comando di configurazione con il token del bot che hai copiato:

        ```
        /discord:configure <token>
        ```

        Questo lo salva in `~/.claude/channels/discord/.env`. Puoi anche impostare `DISCORD_BOT_TOKEN` nel tuo ambiente shell prima di avviare Claude Code.
      </Step>

      <Step title="Riavvia con i canali abilitati">
        Esci da Claude Code e riavvia con il flag del canale. Questo connette il plugin Discord in modo che il tuo bot possa ricevere e rispondere ai messaggi:

        ```bash theme={null}
        claude --channels plugin:discord@claude-plugins-official
        ```
      </Step>

      <Step title="Accoppia il tuo account">
        Invia un messaggio privato al tuo bot su Discord. Il bot risponde con un codice di accoppiamento.

        <Note>Se il tuo bot non risponde, assicurati che Claude Code sia in esecuzione con `--channels` dal passaggio precedente. Il bot può rispondere solo mentre il canale è attivo.</Note>

        Torna in Claude Code ed esegui:

        ```
        /discord:access pair <code>
        ```

        Quindi blocca l'accesso in modo che solo il tuo account possa inviare messaggi:

        ```
        /discord:access policy allowlist
        ```
      </Step>
    </Steps>
  </Tab>

  <Tab title="iMessage">
    Visualizza il [codice sorgente del plugin iMessage](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage) completo.

    Il canale iMessage legge il tuo database Messaggi direttamente e invia risposte tramite AppleScript. Richiede macOS e non ha bisogno di token bot o servizio esterno.

    <Steps>
      <Step title="Concedi accesso completo al disco">
        Il database Messaggi in `~/Library/Messages/chat.db` è protetto da macOS. La prima volta che il server lo legge, macOS richiede l'accesso: fai clic su **Allow**. Il prompt nomina qualsiasi app abbia avviato Bun, come Terminal, iTerm o il tuo IDE.

        Se il prompt non appare o hai fatto clic su Don't Allow, concedi l'accesso manualmente in **System Settings > Privacy & Security > Full Disk Access** e aggiungi il tuo terminale. Senza questo, il server esce immediatamente con `authorization denied`.
      </Step>

      <Step title="Installa il plugin">
        In Claude Code, esegui:

        ```
        /plugin install imessage@claude-plugins-official
        ```

        Se Claude Code segnala che il plugin non si trova in alcun marketplace, il tuo marketplace è mancante o obsoleto. Esegui `/plugin marketplace update claude-plugins-official` per aggiornarlo, oppure `/plugin marketplace add anthropics/claude-plugins-official` se non l'hai ancora aggiunto. Quindi riprova l'installazione.
      </Step>

      <Step title="Riavvia con i canali abilitati">
        Esci da Claude Code e riavvia con il flag del canale:

        ```bash theme={null}
        claude --channels plugin:imessage@claude-plugins-official
        ```
      </Step>

      <Step title="Invia un messaggio a te stesso">
        Apri Messaggi su qualsiasi dispositivo connesso al tuo Apple ID e invia un messaggio a te stesso. Raggiunge Claude immediatamente: l'auto-chat bypassa il controllo di accesso senza configurazione.

        <Note>La prima risposta che Claude invia attiva un prompt di automazione macOS che chiede se il tuo terminale può controllare Messaggi. Fai clic su **OK**.</Note>
      </Step>

      <Step title="Consenti altri mittenti">
        Per impostazione predefinita, solo i tuoi messaggi passano. Per consentire a un altro contatto di raggiungere Claude, aggiungi il suo handle:

        ```
        /imessage:access allow +15551234567
        ```

        Gli handle sono numeri di telefono nel formato `+country` o email Apple ID come `user@example.com`.
      </Step>
    </Steps>
  </Tab>
</Tabs>

Puoi anche [creare il tuo canale](/docs/it/channels-reference) per sistemi che non hanno ancora un plugin.

<h2 id="quickstart">
  Guida rapida
</h2>

Fakechat è un canale demo ufficialmente supportato che esegue un'interfaccia di chat su localhost, senza nulla da autenticare e nessun servizio esterno da configurare.

Una volta installato e abilitato fakechat, puoi digitare nel browser e il messaggio arriva nella tua sessione Claude Code. Claude risponde e la risposta appare di nuovo nel browser. Dopo aver testato l'interfaccia fakechat, prova [Telegram](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram), [Discord](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord) o [iMessage](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage).

Per provare la demo fakechat, avrai bisogno di:

* Claude Code [installato e autenticato](/docs/it/quickstart#step-1-install-claude-code) con un account claude.ai o una chiave API Console
* [Bun](https://bun.sh) installato. I plugin di canale pre-costruiti sono script Bun. Controlla con `bun --version`; se fallisce, [installa Bun](https://bun.sh/docs/installation).
* **Organizzazioni Team, Enterprise o Console gestite**: l'amministratore della tua organizzazione deve [abilitare i canali](#enterprise-controls) nelle impostazioni gestite

<Steps>
  <Step title="Installa il plugin del canale fakechat">
    Avvia una sessione Claude Code ed esegui il comando di installazione:

    ```text theme={null}
    /plugin install fakechat@claude-plugins-official
    ```

    Se Claude Code segnala che il plugin non si trova in alcun marketplace, il tuo marketplace è mancante o obsoleto. Esegui `/plugin marketplace update claude-plugins-official` per aggiornarlo, oppure `/plugin marketplace add anthropics/claude-plugins-official` se non l'hai ancora aggiunto. Quindi riprova l'installazione.
  </Step>

  <Step title="Riavvia con il canale abilitato">
    Esci da Claude Code, quindi riavvia con `--channels` e passa il plugin fakechat che hai installato:

    ```bash theme={null}
    claude --channels plugin:fakechat@claude-plugins-official
    ```

    Il server fakechat si avvia automaticamente.

    <Tip>
      Puoi passare più plugin a `--channels`, separati da spazi.
    </Tip>
  </Step>

  <Step title="Invia un messaggio">
    Apri l'interfaccia fakechat su [http://localhost:8787](http://localhost:8787) e digita un messaggio:

    ```text theme={null}
    hey, what's in my working directory?
    ```

    Il messaggio arriva nella tua sessione Claude Code come evento `<channel source="fakechat">`. Claude lo legge, fa il lavoro e chiama lo strumento `reply` di fakechat. La risposta appare nell'interfaccia di chat.
  </Step>
</Steps>

Se Claude incontra un prompt di permesso mentre sei lontano dal terminale, la sessione si mette in pausa fino a quando non rispondi. I server di canale che dichiarano la [capacità di inoltro dei permessi](/docs/it/channels-reference#relay-permission-prompts) possono inoltrarti questi prompt in modo che tu possa approvare o negare da remoto. Per l'uso incustodito, [`--dangerously-skip-permissions`](/docs/it/permission-modes#skip-all-checks-with-bypasspermissions-mode) bypassa la maggior parte dei prompt, ma usalo solo in ambienti di cui ti fidi. Le regole di richiesta esplicita, gli strumenti del connettore [che la tua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools) e gli strumenti MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool) continuano a richiedere l'approvazione.

Quando esegui i canali in modalità non interattiva con `-p`, gli strumenti che necessitano di input da terminale, come domande a scelta multipla e approvazione della modalità plan, sono disabilitati in modo che la sessione non si blocchi mai in attesa di input.

<h2 id="security">
  Sicurezza
</h2>

Ogni plugin di canale approvato mantiene un allowlist dei mittenti: solo gli ID che hai aggiunto possono inviare messaggi e tutti gli altri vengono silenziosamente scartati.

Telegram e Discord avviano l'elenco mediante accoppiamento:

1. Trova il tuo bot in Telegram o Discord e invigli qualsiasi messaggio
2. Il bot risponde con un codice di accoppiamento
3. Nella tua sessione Claude Code, approva il codice quando richiesto
4. Il tuo ID mittente viene aggiunto all'allowlist

iMessage funziona diversamente: inviarti messaggi bypassa automaticamente il gate, e aggiungi altri contatti per handle con `/imessage:access allow`.

Oltre a ciò, controlli quali server sono abilitati ogni sessione con `--channels`, e la tua organizzazione controlla la disponibilità con [`channelsEnabled`](#enterprise-controls) su piani Team ed Enterprise di claude.ai e su organizzazioni Console che distribuiscono impostazioni gestite.

Essere in `.mcp.json` non è sufficiente per inviare messaggi: un server deve anche essere nominato in `--channels`.

L'allowlist controlla anche l'[inoltro dei permessi](/docs/it/channels-reference#relay-permission-prompts) se il canale lo dichiara. Chiunque possa rispondere attraverso il canale può approvare o negare l'uso dello strumento nella tua sessione, quindi allowlist solo i mittenti di cui ti fidi con questa autorità.

<h2 id="enterprise-controls">
  Controlli Enterprise
</h2>

Gli amministratori controllano la disponibilità attraverso due [impostazioni gestite](/docs/it/settings) che gli utenti non possono ignorare. L'impostazione predefinita dipende da come ti autentichi:

* **claude.ai Team ed Enterprise**: i canali sono bloccati fino a quando un Owner non li abilita.
* **Anthropic Console con autenticazione tramite chiave API**: i canali sono consentiti per impostazione predefinita. Hai bisogno di questa impostazione solo se la tua organizzazione distribuisce impostazioni gestite.

In tutti i casi, nessun canale viene eseguito fino a quando un utente non lo abilita per la sessione con `--channels`.

| Impostazione            | Scopo                                                                                                                                                                                                                                                                                                                       | Quando non configurato                                                                                                                                                                                                             |
| :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `channelsEnabled`       | Interruttore principale. Deve essere `true` affinché qualsiasi canale consegni messaggi. Impostato tramite l'interruttore della [console Admin di claude.ai](https://claude.ai/admin-settings/claude-code) o direttamente nelle impostazioni gestite. Blocca tutti i canali incluso il flag di sviluppo quando disattivato. | claude.ai Team ed Enterprise: canali bloccati. Console: canali consentiti a meno che la tua organizzazione non distribuisca impostazioni gestite, nel qual caso i canali sono bloccati fino a quando questa chiave non è impostata |
| `allowedChannelPlugins` | Quali plugin possono registrarsi una volta abilitati i canali. Sostituisce l'elenco mantenuto da Anthropic quando impostato. Si applica solo quando `channelsEnabled` è `true`.                                                                                                                                             | Si applica l'elenco predefinito di Anthropic                                                                                                                                                                                       |

Gli utenti Pro e Max senza un'organizzazione saltano completamente questi controlli: i canali sono disponibili e gli utenti optano per sessione con `--channels`.

<h3 id="enable-channels-for-your-organization">
  Abilita i canali per la tua organizzazione
</h3>

Abilita i canali per la tua organizzazione da [**claude.ai → Admin settings → Claude Code → Channels**](https://claude.ai/admin-settings/claude-code), che richiede il ruolo Owner, o impostando `channelsEnabled` su `true` nelle impostazioni gestite.

Una volta abilitati, gli utenti della tua organizzazione possono usare `--channels` per optare i server di canale in sessioni individuali. Se l'impostazione è disabilitata o non impostata, il server MCP si connette comunque e i suoi strumenti funzionano, ma i messaggi del canale non arriveranno. Un avviso di avvio dice all'utente di far abilitare l'impostazione a un amministratore.

<h3 id="restrict-which-channel-plugins-can-run">
  Limita quali plugin di canale possono essere eseguiti
</h3>

Per impostazione predefinita, qualsiasi plugin nell'elenco di allowlist mantenuto da Anthropic può registrarsi come canale. Gli amministratori su piani Team ed Enterprise possono sostituire quell'allowlist con il loro impostando `allowedChannelPlugins` nelle impostazioni gestite. Usalo per limitare quali plugin ufficiali sono consentiti, approvare canali dal tuo marketplace interno, o entrambi. Ogni voce nomina un plugin e il marketplace da cui proviene:

```json theme={null}
{
  "channelsEnabled": true,
  "allowedChannelPlugins": [
    { "marketplace": "claude-plugins-official", "plugin": "telegram" },
    { "marketplace": "claude-plugins-official", "plugin": "discord" },
    { "marketplace": "acme-corp-plugins", "plugin": "internal-alerts" }
  ]
}
```

Quando `allowedChannelPlugins` è impostato, sostituisce completamente l'allowlist di Anthropic: solo i plugin elencati possono registrarsi. Lascialo non impostato per tornare all'allowlist predefinito di Anthropic. Se imposti un array vuoto, blocchi tutti i plugin di canale dall'allowlist, ma `--dangerously-load-development-channels` può comunque bypassarlo per i test locali. Per bloccare completamente i canali incluso il flag di sviluppo, lascia invece `channelsEnabled` non impostato.

Questa impostazione richiede `channelsEnabled: true`. Se un utente passa un plugin a `--channels` che non è nel tuo elenco, Claude Code si avvia normalmente ma il canale non si registra, e l'avviso di avvio spiega che il plugin non è nell'elenco approvato dell'organizzazione.

<h2 id="research-preview">
  Anteprima di ricerca
</h2>

I canali sono una funzione di anteprima di ricerca. La disponibilità viene implementata gradualmente e la sintassi del flag `--channels` e il contratto del protocollo possono cambiare in base al feedback.

Durante l'anteprima, `--channels` accetta solo plugin da un allowlist mantenuto da Anthropic, o dall'allowlist della tua organizzazione se un amministratore ha impostato [`allowedChannelPlugins`](#restrict-which-channel-plugins-can-run). I plugin di canale in [claude-plugins-official](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins) sono l'insieme approvato predefinito. Se passi qualcosa che non è nell'allowlist effettivo, Claude Code si avvia normalmente ma il canale non si registra, e l'avviso di avvio ti dice perché.

Per testare un canale che stai costruendo, usa `--dangerously-load-development-channels`. Vedi [Test durante l'anteprima di ricerca](/docs/it/channels-reference#test-during-the-research-preview) per informazioni sul test di canali personalizzati che costruisci.

Segnala problemi o feedback nel [repository GitHub di Claude Code](https://github.com/anthropics/claude-code/issues).

<h2 id="how-channels-compare">
  Come si confrontano i canali
</h2>

Diverse funzioni di Claude Code si connettono a sistemi al di fuori del terminale, ognuna adatta a un diverso tipo di lavoro:

| Funzione                                          | Cosa fa                                                                    | Buono per                                                               |
| ------------------------------------------------- | -------------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| [Claude Code sul web](/docs/it/claude-code-on-the-web) | Esegue attività in una nuova sandbox cloud, clonata da GitHub              | Delegare lavoro asincrono autonomo su cui torni in seguito              |
| [Claude in Slack](/docs/it/slack)                      | Avvia una sessione web da una menzione `@Claude` in un canale o thread     | Avviare attività direttamente dal contesto della conversazione del team |
| Server [MCP](/docs/it/mcp) standard                    | Claude lo interroga durante un'attività; nulla viene inviato alla sessione | Dare a Claude accesso on-demand per leggere o interrogare un sistema    |
| [Remote Control](/docs/it/remote-control)              | Guidi la tua sessione locale da claude.ai o dall'app mobile Claude         | Guidare una sessione in corso mentre sei lontano dalla tua scrivania    |

I canali colmano il divario in quell'elenco inviando eventi da fonti non-Claude nella tua sessione locale già in esecuzione.

* **Ponte di chat**: chiedi a Claude qualcosa dal tuo telefono tramite Telegram, Discord o iMessage, e la risposta torna nella stessa chat mentre il lavoro viene eseguito sulla tua macchina contro i tuoi file reali.
* **[Ricevitore webhook](/docs/it/channels-reference#example-build-a-webhook-receiver)**: un webhook da CI, il tuo error tracker, una pipeline di deploy o altro servizio esterno arriva dove Claude ha già i tuoi file aperti e ricorda cosa stavi debuggando.

<h2 id="next-steps">
  Passaggi successivi
</h2>

Una volta che hai un canale in esecuzione, esplora queste funzioni correlate:

* [Crea il tuo canale](/docs/it/channels-reference) per sistemi che non hanno ancora plugin
* [Remote Control](/docs/it/remote-control) per guidare una sessione locale dal tuo telefono invece di inoltrarvi eventi
* [Attività pianificate](/docs/it/scheduled-tasks) per eseguire il polling su un timer invece di reagire a eventi inviati
