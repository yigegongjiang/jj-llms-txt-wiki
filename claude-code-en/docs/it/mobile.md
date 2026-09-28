> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code su mobile

> Avvia, monitora e guida i task di Claude Code dal tuo telefono con l'app Claude per iOS e Android.

L'app Claude per [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) e [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) è un client per le sessioni di Claude Code piuttosto che un luogo dove il codice viene eseguito. Dal tuo telefono raggiungi [sessioni cloud](#start-and-monitor-cloud-sessions) e [progetti](/docs/it/claude-projects) nel cloud, una sessione in esecuzione sulla tua macchina tramite [Remote Control](#continue-a-local-session-with-remote-control), o l'app Desktop tramite [Dispatch](/docs/it/desktop#sessions-from-dispatch).

<Note>
  Claude Code non ha un'app mobile separata: le sessioni cloud e Remote Control vivono entrambe nella scheda **Code** nell'app Claude, e Dispatch è un task a cui invii messaggi nell'app.
</Note>

<h2 id="get-the-app">
  Scarica l'app
</h2>

<Steps>
  <Step title="Scarica l'app Claude">
    Installa l'app Claude per [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) o [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude). Su un iPad, installa la stessa app iOS.

    <Tip>
      Esegui `/mobile` in una sessione di Claude Code per visualizzare un codice QR per [claude.ai/mobile](https://claude.ai/mobile), che apre il giusto app store per il tuo telefono. `/ios` e `/android` fanno la stessa cosa.
    </Tip>
  </Step>

  <Step title="Accedi">
    Accedi con lo stesso account claude.ai e organizzazione che usi per Claude Code. Le sessioni cloud e Remote Control richiedono un account claude.ai, quindi non sono raggiungibili con una chiave API della Console Anthropic o da un provider di terze parti come Amazon Bedrock.
  </Step>

  <Step title="Apri la scheda Code">
    Tocca **Code** nella navigazione dell'app per raggiungere le tue sessioni, o apri [claude.ai/code/new](https://claude.ai/code/new) sul tuo telefono per avviare una nuova sessione Code nell'app. Se non vedi la scheda Code, il tuo piano o organizzazione potrebbe non includere queste funzionalità; vedi [disponibilità per piano di abbonamento](/docs/it/feature-availability#availability-by-subscription-plan).
  </Step>
</Steps>

<h2 id="work-from-your-phone">
  Lavora dal tuo telefono
</h2>

Dall'app puoi avviare sessioni cloud, aprire un progetto, guidare una sessione di Claude Code in esecuzione sul tuo computer, o inviare un task a Dispatch tramite messaggio. L'app è la stessa per tutti; differiscono nel luogo dove avviene il lavoro.

| Funzionalità                                   | A cosa ti connetti                                                                      | Quando usare                                                                                                                                                                     |
| :--------------------------------------------- | :-------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Cloud sessions](/docs/it/claude-code-on-the-web)   | Una sessione su infrastruttura cloud, gestita da Anthropic per impostazione predefinita | Il tuo repository è su GitHub e il task dovrebbe continuare a essere eseguito dopo aver messo il telefono via. Vedi la [guida rapida cloud](/docs/it/web-quickstart) per configurare. |
| [Projects](/docs/it/claude-projects)                | Una conversazione dove Claude coordina sessioni cloud parallele come thread             | Hai un flusso di lavoro correlato piuttosto che un singolo task e vuoi vedere quali thread sono terminati o hanno bisogno di te.                                                 |
| [Remote Control](/docs/it/remote-control)           | Una sessione di Claude Code in esecuzione sul tuo computer                              | Il lavoro ha bisogno del tuo filesystem locale, strumenti o server MCP.                                                                                                          |
| [Dispatch](/docs/it/desktop#sessions-from-dispatch) | L'app Desktop sul tuo computer                                                          | Vuoi inviare un task tramite messaggio e lasciare che Dispatch decida come eseguirlo. Richiede un piano Pro o Max.                                                               |

Se il tuo computer sarà spento, usa le sessioni cloud o un progetto, che vengono eseguiti nel cloud e continuano con il tuo laptop chiuso. Remote Control e Dispatch guidano la tua macchina, quindi deve rimanere accesa con Claude Code o l'app Desktop in esecuzione. Se la tua macchina va in sospensione durante una sessione Remote Control, Claude Code si riconnette quando la macchina torna online.

Per un confronto più completo, vedi [lavora quando sei lontano dal tuo terminale](/docs/it/platforms#work-when-you-are-away-from-your-terminal).

Le sessioni cloud e Remote Control vengono eseguite dalla scheda **Code**. Per Dispatch, che invii come task nell'app, vedi [sessioni da Dispatch](/docs/it/desktop#sessions-from-dispatch).

<h3 id="start-and-monitor-cloud-sessions">
  Avvia e monitora le sessioni cloud
</h3>

Le sessioni cloud eseguono task su infrastruttura cloud, gestita da Anthropic per impostazione predefinita, quindi una sessione continua dopo aver messo il telefono via. Dalla scheda Code, seleziona un repository e un branch, descrivi il task e invialo. Le sessioni persistono tra i dispositivi: un task che avvii sul tuo laptop è pronto per la revisione dal tuo telefono, e uno che avvii dal tuo telefono ti sta aspettando quando torni alla tua scrivania.

Apri una sessione nell'app per controllare i progressi, rispondere alle domande di Claude, o guidarla in una nuova direzione. Puoi anche dire a Claude di [osservare una pull request](/docs/it/claude-code-on-the-web#auto-fix-pull-requests) e correggere i fallimenti CI o i commenti di revisione man mano che arrivano. Per connettere GitHub e configurare il tuo ambiente, segui la [guida rapida cloud](/docs/it/web-quickstart), e vedi [Usa Claude Code nel cloud](/docs/it/claude-code-on-the-web) per tutto quello che le sessioni cloud possono fare.

<h3 id="continue-a-local-session-with-remote-control">
  Continua una sessione locale con Remote Control
</h3>

Remote Control connette l'app Claude a una sessione di Claude Code in esecuzione sulla tua macchina, quindi l'esecuzione del codice e l'accesso al filesystem rimangono locali mentre guidi la sessione dal tuo telefono. Avvia la sessione sul tuo computer con `claude remote-control`, o esegui `/remote-control` in una sessione già aperta. Quindi scansiona il codice QR che il terminale può visualizzare, o apri l'app Claude, tocca **Code**, e scegli la sessione dall'elenco. Vedi [connetti da un altro dispositivo](/docs/it/remote-control#connect-from-another-device) per ogni opzione.

Quando aggiungi un allegato nell'app Claude, raggiunge anche la sessione locale:

* **Foto**: Claude vede le foto allegate direttamente come parte del tuo messaggio. Claude Code salva anche ogni foto in `~/.claude/uploads/` e dice a Claude il percorso del file salvato, quindi Claude può copiare l'immagine nei file che crea.
* **Altri file**: Claude Code li scarica sulla tua macchina e li passa a Claude come riferimenti di file `@`.

Per i requisiti, le modalità di invocazione e la risoluzione dei problemi, vedi la [panoramica di Remote Control](/docs/it/remote-control).

<h3 id="get-push-notifications">
  Ricevi notifiche push
</h3>

Quando Remote Control è attivo, Claude può inviare notifiche push al tuo telefono, tipicamente quando un task a lunga esecuzione finisce o quando ha bisogno di una decisione da te. Puoi anche chiederne una nel tuo prompt, come `notificami quando i test finiscono`. Vedi [notifiche push mobile](/docs/it/remote-control#mobile-push-notifications) per i due toggle `/config` e la risoluzione dei problemi di consegna.

Dispatch invia la sua propria notifica quando una sessione Code che ha generato finisce o ha bisogno della tua approvazione, descritta in [sessioni da Dispatch](/docs/it/desktop#sessions-from-dispatch).

<h2 id="limitations">
  Limitazioni
</h2>

Il client mobile copre la maggior parte di quello che una sessione ha bisogno, con alcune limitazioni:

* **Comandi solo locali**: comandi che vengono eseguiti solo nell'interfaccia del terminale, come `/plugin` e `/resume`, non funzionano dall'app. Le [limitazioni di Remote Control](/docs/it/remote-control#limitations) elencano i comandi che funzionano da mobile e come il loro comportamento differisce.
* **Modalità di autorizzazione**: le sessioni cloud offrono Accept edits, Plan e Auto nel dropdown della modalità, e le sessioni Remote Control offrono Manual, Accept edits e Plan. Non puoi selezionare Bypass permissions dall'app in nessuno dei due casi, e non puoi selezionare Auto per una sessione Remote Control. Vedi [cambia modalità di autorizzazione](/docs/it/permission-modes#switch-permission-modes).
* **Piani Dispatch**: Dispatch richiede un piano Pro o Max e non è disponibile su Team o Enterprise.

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Piattaforme e integrazioni](/docs/it/platforms): confronta ogni superficie su cui Claude Code viene eseguito
* [Claude Code sul web](/docs/it/claude-code-on-the-web): come vengono eseguite le sessioni cloud e come spostare il lavoro da e verso il tuo terminale
* [Configura ambienti cloud](/docs/it/cloud-environments): livelli di accesso di rete, variabili di ambiente e script di configurazione per le sessioni cloud
* [Remote Control](/docs/it/remote-control): continua una sessione locale da qualsiasi dispositivo
* [Sessioni da Dispatch](/docs/it/desktop#sessions-from-dispatch): come i task Dispatch diventano sessioni Code nell'app Desktop
* [Channels](/docs/it/channels): chiedi a Claude qualcosa dal tuo telefono tramite Telegram, Discord o iMessage mentre il lavoro viene eseguito sulla tua macchina
* [Claude Code in Slack](/docs/it/slack): delega task di codifica dal tuo workspace Slack menzionando `@Claude`
