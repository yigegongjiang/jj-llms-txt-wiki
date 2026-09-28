> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Testare app iOS nel simulatore

> Claude Code Desktop apre la tua app nel riquadro iOS Simulator quando Claude la compila, esegue o la verifica, con un simulatore separato per ogni sessione.

<Note>
  Il riquadro iOS Simulator è in beta pubblica in Claude Code Desktop su macOS. È disponibile nei piani Pro, Max, Team ed Enterprise, tranne nelle organizzazioni Enterprise che hanno una configurazione HIPAA abilitata.
</Note>

Il riquadro iOS Simulator mostra la tua app in esecuzione nell'iOS Simulator di Apple accanto alla tua conversazione in Claude Code Desktop. Quando Claude compila, installa, avvia o verifica la tua app in un simulatore, il riquadro si apre automaticamente e trasmette lo schermo del dispositivo in diretta. Usalo per osservare Claude mentre esegue e testa la tua app, oppure per navigare tu stesso nell'app mentre Claude continua a lavorare.

Il riquadro del simulatore controlla il simulatore direttamente, quindi non ha bisogno di [computer use](/docs/it/desktop#let-claude-use-your-computer) e non prende mai il controllo dello schermo o nasconde le altre finestre. Dalla CLI, Claude raggiunge l'iOS Simulator attraverso [computer use](/docs/it/computer-use#test-a-simulator-flow), che controlla il simulatore sullo schermo nello stesso modo in cui lo faresti con un mouse.

<h2 id="requirements">
  Requisiti
</h2>

Il riquadro del simulatore utilizza gli strumenti del simulatore di Apple, che l'app desktop non include. Prima di avviare una sessione, assicurati di avere:

* Claude Desktop v1.24012.0 o successivo
* Un Mac, poiché l'iOS Simulator di Apple funziona solo su macOS
* [Xcode](https://developer.apple.com/xcode/) con la piattaforma iOS installata, che fornisce i dispositivi simulatore. Se Xcode non elenca ancora simulatori, vedi [Il riquadro del simulatore dice che non sono stati trovati simulatori](#the-simulator-pane-says-no-simulators-were-found)
  * Usa Xcode 26.x. Il riquadro non funziona ancora con Xcode 27, che sostituisce l'app Simulator con Device Hub. Se `xcode-select` punta a Xcode 27 sul tuo Mac, vedi [Il riquadro del simulatore non funziona con Xcode 27](#the-simulator-pane-fails-with-xcode-27)

<Note>
  In questa pagina, "dispositivo" si riferisce a un iPhone o iPad simulato, uno degli stessi dispositivi simulatore che gestisci in Xcode in **Window → Devices and Simulators**, non a hardware fisico.
</Note>

Il riquadro del simulatore è disponibile solo nelle sessioni locali. Nelle sessioni [cloud](/docs/it/desktop#run-long-running-tasks-in-the-cloud) e [SSH](/docs/it/desktop#ssh-sessions), Claude viene eseguito su una macchina che non può raggiungere i simulatori sul tuo Mac.

<h2 id="run-your-app-in-the-simulator">
  Esegui la tua app nel simulatore
</h2>

Non hai bisogno di un comando o di un'impostazione per aprire il riquadro del simulatore. Claude lo apre quando esegue la tua app in un simulatore.

<Steps>
  <Step title="Apri il tuo progetto iOS">
    In Claude Code Desktop, apri la scheda **Code** e avvia una sessione con la cartella del progetto della tua app come [cartella del progetto](/docs/it/desktop#start-a-session). Qualsiasi progetto che compila un'app per l'iOS Simulator funziona.
  </Step>

  <Step title="Chiedi a Claude di eseguire o testare l'app">
    Esprimi l'attività intorno all'esecuzione o alla verifica dell'app. Ad esempio:

    ```text theme={null}
    Build the app and run it in the simulator to check the onboarding flow.
    ```
  </Step>

  <Step title="Guarda l'app nel riquadro del simulatore">
    Quando l'app viene avviata in un simulatore, il riquadro iOS Simulator si apre accanto alla conversazione. La prima volta che Claude utilizza un dispositivo, l'app desktop ti chiede di consentirlo; vedi [Concedi a Claude l'accesso a un dispositivo](#grant-claude-access-to-a-device). Claude installa l'app, la naviga, e legge lo schermo per verificare i propri cambiamenti mentre tu osservi.
  </Step>
</Steps>

Il riquadro del simulatore si apre ogni volta che Claude avvia l'app in un simulatore, in qualsiasi momento della sessione. Quando la tua richiesta riguarda la visualizzazione dell'app, ad esempio "il nuovo schermo sembra giusto?", Claude avvia un simulatore prima di iniziare il lavoro. Dopo che Claude corregge un bug o cambia uno schermo, chiedigli di verificare il cambiamento: riavviare l'app riaprirà il riquadro se non è aperto.

Il riquadro del simulatore mostra il dispositivo in cui l'app è effettivamente stata avviata. Per testare su un dispositivo specifico, nominalo nella tua richiesta, ad esempio "eseguilo sul simulatore iPhone SE", e Claude indirizza quel dispositivo quando compila e avvia.

Un dispositivo che Claude avvia appare anche nell'app Simulator di Apple, e Claude può installare l'app su un dispositivo che hai già avviato.

Puoi anche aprire il riquadro del simulatore tu stesso. Una volta che la sessione ha un simulatore collegato o ha modificato file Swift, il menu **Views** nella barra degli strumenti della sessione mostra una voce **iOS Simulator**. Se il riquadro non sta ancora mostrando un dispositivo, fai clic su **Attach simulator**, oppure scegli un dispositivo specifico dal menu dei dispositivi accanto ad esso; scegliere un dispositivo spento lo avvia. Se Xcode o i suoi simulatori mancano, il riquadro mostra invece i passaggi di configurazione e li spunta man mano che li completi.

<h2 id="control-the-simulator-yourself">
  Controlla il simulatore tu stesso
</h2>

Il riquadro del simulatore è interattivo, non solo un visualizzatore. Mentre Claude lavora, o tra i compiti, puoi:

* Toccare e scorrere facendo clic e trascinando sullo schermo del dispositivo
* Premere i pulsanti hardware con le stesse scorciatoie da tastiera dell'app Simulator di Apple: **Cmd+Shift+H** per Home, **Cmd+L** per bloccare, **Cmd+Up Arrow** e **Cmd+Down Arrow** per il volume
* Ruotare il dispositivo di un quarto di giro in senso orario con il pulsante di rotazione o **Cmd+Right Arrow**
* Cambiare quale dispositivo il riquadro mostra dal menu dei dispositivi, che elenca la versione del sistema operativo di ogni simulatore e se è avviato
* Salvare uno screenshot con **Cmd+S** o una registrazione dello schermo con **Cmd+R**, utilizzando i pulsanti di acquisizione del riquadro o le scorciatoie da tastiera; i file vengono salvati sul tuo Desktop
* Interrompere lo streaming di un dispositivo senza spegnerlo facendo clic su **Detach simulator**, che riporta il riquadro allo stato **Attach simulator**

La riga sotto il nome del dispositivo regola il flusso video dal simulatore. Abbassa **Frame rate** o **Resolution** se il riquadro affatica il tuo Mac, cambia **Encoding** tra H.264 e JPEG, oppure seleziona **FPS** per visualizzare la frequenza dei fotogrammi che il riquadro sta ricevendo. Queste impostazioni cambiano il modo in cui il riquadro visualizza il dispositivo, non il modo in cui l'app viene eseguita.

Tu e Claude controllate lo stesso dispositivo, quindi i tuoi tocchi cambiano lo stato dell'app che Claude vede. Per fare in modo che Claude verifichi uno schermo specifico, navigaci toccando, quindi chiedi. Mentre Claude controlla il dispositivo, il riquadro mostra un badge **Claude is using this device** sopra lo schermo; aspetta che il badge scompaia prima di toccare, in modo che il risultato rifletta l'app piuttosto che il tuo input.

<h2 id="how-sessions-manage-devices">
  Come le sessioni gestiscono i dispositivi
</h2>

Ogni dispositivo appartiene alla sessione che lo ha avviato, quindi le [sessioni parallele](/docs/it/desktop#work-in-parallel-with-sessions) non condividono un dispositivo: quello che vedi nel riquadro di una sessione riflette il lavoro di quella sessione, non di un'altra. Cambiare sessioni nella barra laterale cambia la visualizzazione del simulatore insieme alla conversazione, e tornare indietro riprende lo stesso dispositivo da dove era rimasto. Se Claude lavora con più di un dispositivo, ognuno apre il proprio riquadro, fino a 4 per sessione.

Claude Code Desktop spegne i simulatori che ha avviato una volta che non sono più in uso: quando esci dall'app, quando archivi la sessione, o 10 minuti dopo aver scollegato un dispositivo dal suo riquadro. I dispositivi che avvii tu stesso, sia dal riquadro che dall'app Simulator di Apple, non vengono mai spenti automaticamente. Per spegnere il dispositivo collegato subito, usa il pulsante di spegnimento nel riquadro.

<h2 id="grant-claude-access-to-a-device">
  Concedi a Claude l'accesso a un dispositivo
</h2>

Claude chiede il tuo consenso prima di controllare un dispositivo, mentre compilare l'app o aprire un URL su di esso segue la modalità di autorizzazione della tua sessione. Tu o la tua organizzazione potete anche disattivare completamente l'accesso di Claude.

<h3 id="allow-a-device-the-first-time">
  Consenti un dispositivo la prima volta
</h3>

La prima volta che Claude utilizza un simulatore, l'app desktop ti chiede di consentirlo. Il consenso copre il controllo di quel dispositivo e l'acquisizione di screenshot, e lo dai una volta per dispositivo piuttosto che una volta per sessione. Gli screenshot di Claude del dispositivo vengono inviati ad Anthropic e conservati secondo le tue normali impostazioni di conservazione della conversazione, quindi non accedere ad account reali su un dispositivo che Claude utilizza.

Dopo aver consentito un dispositivo, le azioni di Claude su di esso, come toccare, digitare, avviare l'app e acquisire screenshot, vengono eseguite senza ulteriori prompt. Hanno la stessa fiducia di quando fai clic nel riquadro, e toccano solo il dispositivo simulato, quindi il riquadro non ha bisogno delle autorizzazioni macOS Accessibility e Screen Recording che il computer use richiede.

Se rifiuti, il dispositivo viene comunque avviato e il riquadro funziona ancora per i tuoi tocchi; solo l'accesso di Claude rimane disattivato. Per cambiare idea in seguito, fai clic su **Let Claude use it** nel riquadro.

<h3 id="actions-that-follow-your-permission-mode">
  Azioni che seguono la tua modalità di autorizzazione
</h3>

Due azioni seguono la [modalità di autorizzazione](/docs/it/permissions#permission-modes) della tua sessione invece del consenso una tantum:

* Aprire un URL sul dispositivo, ad esempio per testare un deep link o caricare una pagina nel Safari del dispositivo, perché un URL può portare dati fuori dal dispositivo.
* Compilare l'app, perché `xcodebuild` esegue gli script di compilazione del tuo progetto sul tuo Mac. Controllare una compilazione già in corso non genera un prompt.

<h3 id="turn-off-simulator-access">
  Disattiva l'accesso al simulatore
</h3>

Puoi disattivare l'accesso al simulatore di Claude nelle impostazioni dell'app desktop. Le organizzazioni hanno due modi per disattivarlo per tutti:

* L'[impostazione gestita](/docs/it/desktop#managed-settings) `disableMobileSimulatorTools` blocca gli strumenti del simulatore di Claude. Il riquadro del simulatore rimane utilizzabile per i tuoi tocchi, e l'impostazione non può essere ignorata dall'interno dell'app.
* La chiave della politica `requireCoworkFullVmSandbox`, che esegue gli strumenti di Claude all'interno di una macchina virtuale isolata invece che sul tuo Mac, disabilita il riquadro del simulatore e gli strumenti del simulatore di Claude interamente, quindi il riquadro non può collegare un dispositivo mentre è impostato.

Claude ti avvisa quando uno di questi si applica.

<h2 id="limitations">
  Limitazioni
</h2>

Claude controlla solo dispositivi simulati e non può controllare un iPhone o iPad fisico. Per testare su uno, esegui l'app su di esso da Xcode tu stesso, quindi descrivi quello che vedi o allega uno screenshot alla conversazione affinché Claude possa lavorarci.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="the-simulator-pane-doesn’t-open-when-claude-runs-the-app">
  Il riquadro del simulatore non si apre quando Claude esegue l'app
</h3>

Claude potrebbe non aver riconosciuto che volevi eseguire o testare l'app, oppure gli strumenti del simulatore potrebbero mancare. Controlla quanto segue:

* Dichiara l'obiettivo esplicitamente, ad esempio "esegui l'app nell'iOS Simulator e naviga attraverso il flusso di iscrizione".
* Conferma che Xcode e i simulatori iOS siano installati e che la tua versione di Xcode soddisfi i [requisiti](#requirements).
* Se la tua organizzazione gestisce Claude Code, gli [strumenti del simulatore potrebbero essere disabilitati dalla politica](#turn-off-simulator-access).
* Se sei in un'organizzazione Enterprise che ha una configurazione HIPAA abilitata, il riquadro del simulatore non è disponibile per te.
* Il riquadro del simulatore richiede Claude Desktop v1.24012.0 o successivo. Apri **Claude → Check for Updates**, quindi riavvia l'app.

<h3 id="the-simulator-pane-says-no-simulators-were-found">
  Il riquadro del simulatore dice che non sono stati trovati simulatori
</h3>

Se `xcode-select` punta a Xcode 27, il riquadro può segnalare che non sono stati trovati simulatori anche se i dispositivi esistono; vedi [Il riquadro del simulatore non funziona con Xcode 27](#the-simulator-pane-fails-with-xcode-27). Altrimenti, Xcode è installato ma non ha simulatori iOS da elencare. Il riquadro del simulatore mostra i passaggi di configurazione da seguire e li spunta man mano che ognuno si completa. Per installare il pezzo mancante manualmente, scarica il runtime del simulatore iOS dalle impostazioni di Xcode, oppure esegui `xcodebuild -downloadPlatform iOS`.

<h3 id="the-simulator-pane-fails-with-xcode-27">
  Il riquadro del simulatore non funziona con Xcode 27
</h3>

Il riquadro non funziona ancora con Xcode 27, che sostituisce l'app Simulator con Device Hub. Con Xcode 27 selezionato, il collegamento di un dispositivo non riesce, oppure il riquadro segnala che non sono stati trovati simulatori anche se i dispositivi esistono.

Il riquadro utilizza qualunque Xcode `xcode-select` punti. Se Xcode 27 è la tua unica installazione, installa prima Xcode 26.x insieme ad esso. Quindi seleziona l'installazione 26.x dal suo percorso. Ad esempio, se è installato come `/Applications/Xcode-26.4.app`:

```bash theme={null}
sudo xcode-select -s /Applications/Xcode-26.4.app
```

Esegui `xcode-select -p` per controllare quale installazione è selezionata.

<h2 id="see-also">
  Vedi anche
</h2>

* [Computer use in Desktop](/docs/it/desktop#let-claude-use-your-computer): controllo dello schermo per app senza un riquadro dedicato
* [Computer use from the CLI](/docs/it/computer-use): come la CLI raggiunge l'iOS Simulator
* [Work in parallel with sessions](/docs/it/desktop#work-in-parallel-with-sessions): come le sessioni isolano i cambiamenti
* [Get started with Claude Code Desktop](/docs/it/desktop-quickstart)
