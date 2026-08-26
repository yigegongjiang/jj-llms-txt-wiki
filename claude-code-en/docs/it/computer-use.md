> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Consenti a Claude di usare il tuo computer dalla CLI

> Abilita computer use in Claude Code CLI affinché Claude possa aprire app, fare clic, digitare e vedere il tuo schermo su macOS. Testa app native, esegui il debug di problemi visivi e automatizza strumenti solo GUI senza lasciare il tuo terminale.

<Note>
  Computer use è un'anteprima di ricerca su macOS che richiede un piano Pro o Max. Non è disponibile nei piani Team o Enterprise. Richiede una sessione interattiva, quindi non è disponibile in modalità non interattiva con il flag `-p`.
</Note>

Computer use consente a Claude di aprire app, controllare il tuo schermo e lavorare sulla tua macchina come faresti tu. Dalla CLI, Claude può compilare un'app Swift, avviarla, fare clic su ogni pulsante e acquisire uno screenshot del risultato, il tutto nella stessa conversazione in cui ha scritto il codice.

Questa pagina spiega come funziona computer use nella CLI. Per l'app Desktop su macOS o Windows, vedi [computer use in Desktop](/docs/it/desktop#let-claude-use-your-computer).

<h2 id="what-you-can-do-with-computer-use">
  Cosa puoi fare con computer use
</h2>

Computer use gestisce attività che richiedono una GUI: qualsiasi cosa che normalmente dovresti lasciare il terminale e fare manualmente.

* **Compila e convalida app native**: chiedi a Claude di compilare un'app menu bar di macOS. Claude scrive lo Swift, lo compila, avvia l'app e fa clic su ogni controllo per verificare che funzioni prima che tu la apra.
* **Test UI end-to-end**: punta Claude a un'app Electron locale e dì "testa il flusso di onboarding." Claude apre l'app, fa clic attraverso l'iscrizione e acquisisce uno screenshot di ogni passaggio. Nessuna configurazione Playwright, nessun test harness.
* **Esegui il debug di problemi visivi e di layout**: dì a Claude "il modale si taglia su finestre piccole." Claude ridimensiona la finestra, riproduce il bug, acquisisce uno screenshot, corregge il CSS e verifica la correzione. Claude vede quello che vedi tu.
* **Guida strumenti solo GUI**: interagisci con strumenti di progettazione, pannelli di controllo hardware, iOS Simulator o app proprietarie che non hanno CLI o API.

<h2 id="when-computer-use-applies">
  Quando si applica computer use
</h2>

Claude ha diversi modi per interagire con un'app o un servizio. Computer use è il più ampio e lento, quindi Claude prova prima lo strumento più preciso:

* Se hai un [server MCP](/docs/it/mcp) per il servizio, Claude lo usa.
* Se l'attività è un comando shell, Claude usa Bash.
* Se l'attività è lavoro nel browser e hai [Claude in Chrome](/docs/it/chrome) configurato, Claude lo usa.
* Se nessuno di questi si applica, Claude usa computer use.

Il controllo dello schermo è riservato a cose che nient'altro può raggiungere: app native, simulatori e strumenti senza API.

<h2 id="enable-computer-use">
  Abilita computer use
</h2>

Computer use è disponibile come server MCP integrato chiamato `computer-use`. È disabilitato per impostazione predefinita finché non lo abiliti.

<Steps>
  <Step title="Apri il menu MCP">
    In una sessione interattiva di Claude Code, esegui:

    ```text theme={null}
    /mcp
    ```

    Trova `computer-use` nell'elenco dei server. Viene mostrato come disabilitato.
  </Step>

  <Step title="Abilita il server">
    Seleziona `computer-use` e scegli **Enable**. L'impostazione persiste per progetto, quindi lo fai una sola volta per ogni progetto in cui desideri computer use.
  </Step>

  <Step title="Concedi le autorizzazioni di macOS">
    La prima volta che Claude tenta di usare il tuo computer, vedrai un prompt per concedere due autorizzazioni di macOS:

    * **Accessibility**: consente a Claude di fare clic, digitare e scorrere
    * **Screen Recording**: consente a Claude di vedere cosa c'è sullo schermo

    Il prompt include link per aprire il riquadro Impostazioni di sistema pertinente. Concedi entrambi, quindi seleziona **Try again** nel prompt. macOS potrebbe richiedere il riavvio di Claude Code dopo aver concesso Screen Recording.
  </Step>
</Steps>

Dopo la configurazione, chiedi a Claude di fare qualcosa che necessita della GUI:

```text theme={null}
Build the app target, launch it, and click through each tab to make
sure nothing crashes. Screenshot any error states you find.
```

<h2 id="approve-apps-per-session">
  Approva app per sessione
</h2>

L'abilitazione del server `computer-use` non concede a Claude l'accesso a ogni app sulla tua macchina. La prima volta che Claude ha bisogno di un'app specifica in una sessione, nel tuo terminale appare un prompt che mostra:

* Quali app Claude vuole controllare
* Eventuali autorizzazioni aggiuntive richieste, come l'accesso agli appunti
* Quante altre app saranno nascoste mentre Claude lavora

Scegli **Allow for this session** o **Deny**. Le approvazioni durano per la sessione corrente. Puoi approvare più app contemporaneamente quando Claude le richiede insieme.

Le app con ampia portata mostrano un avviso aggiuntivo nel prompt in modo che tu sappia cosa concedere:

| Avviso                                    | Si applica a                                           |
| :---------------------------------------- | :----------------------------------------------------- |
| Equivalente all'accesso shell             | Terminal, iTerm, VS Code, Warp e altri terminali e IDE |
| Può leggere o scrivere qualsiasi file     | Finder                                                 |
| Può modificare le impostazioni di sistema | System Settings                                        |

Queste app non sono bloccate. L'avviso ti consente di decidere se l'attività giustifica quel livello di accesso.

Il livello di controllo di Claude varia anche per categoria di app: i browser e le piattaforme di trading sono di sola visualizzazione, i terminali e gli IDE sono di sola clic e tutto il resto ottiene il controllo completo. Vedi [app permissions in Desktop](/docs/it/desktop#app-permissions) per la suddivisione completa dei livelli.

<h2 id="how-claude-works-on-your-screen">
  Come Claude lavora sul tuo schermo
</h2>

Comprendere il flusso ti aiuta ad anticipare cosa farà Claude e come intervenire.

<h3 id="one-session-at-a-time">
  Una sessione alla volta
</h3>

Computer use mantiene un blocco a livello di macchina dalla prima azione di computer use fino a quando la sessione che lo ha acquisito esce. A partire da v2.1.195, completare l'attività non rilascia il blocco; solo l'uscita dalla sessione lo fa. Se un'altra sessione di Claude Code sta già usando il tuo computer, i nuovi tentativi falliscono con un messaggio che ti dice quale sessione tiene il blocco. Esci da quella sessione per primo.

<h3 id="apps-are-hidden-while-claude-works">
  Le app sono nascoste mentre Claude lavora
</h3>

Quando Claude inizia a controllare il tuo schermo, altre app visibili vengono nascoste in modo che Claude interagisca solo con le app approvate. La finestra del tuo terminale rimane visibile ed è esclusa dagli screenshot, quindi puoi guardare la sessione e Claude non vede mai il suo stesso output.

Quando Claude termina il turno, le app nascoste vengono ripristinate automaticamente.

<h3 id="screenshots-are-downscaled-automatically">
  Gli screenshot vengono ridimensionati automaticamente
</h3>

Claude Code ridimensiona ogni screenshot prima di inviarlo al modello. Non è necessario abbassare la risoluzione del display o ridimensionare le finestre su Retina o altri display ad alta risoluzione. Un MacBook Pro da 16 pollici con risoluzione Retina nativa acquisisce a 3456×2234 e ridimensiona a circa 1372×887, preservando le proporzioni.

Non c'è alcuna impostazione per modificare la dimensione target. Se il testo o i controlli sullo schermo sono troppo piccoli per Claude per leggerli dopo il ridimensionamento, aumenta la loro dimensione nell'app piuttosto che modificare la risoluzione del display.

<h3 id="stop-at-any-time">
  Ferma in qualsiasi momento
</h3>

Quando Claude acquisisce il blocco, appare una notifica di macOS: "Claude is using your computer · press Esc to stop." Premi `Esc` ovunque per interrompere immediatamente l'azione corrente, oppure premi `Ctrl+C` nel terminale. In entrambi i casi, Claude si ferma, mostra di nuovo le tue app e ti restituisce il controllo. La sessione mantiene il [blocco di computer use](#one-session-at-a-time) fino a quando non esce.

Una seconda notifica appare quando Claude ha finito.

<h2 id="safety-and-the-trust-boundary">
  Sicurezza e il confine di fiducia
</h2>

<Warning>
  A differenza dello [strumento Bash in sandbox](/docs/it/sandboxing), computer use viene eseguito sul tuo desktop effettivo con accesso alle app che approvi. Claude controlla ogni azione e segnala potenziali iniezioni di prompt dal contenuto sullo schermo, ma il confine di fiducia è diverso. Vedi la [guida alla sicurezza di computer use](https://support.claude.com/en/articles/14128542) per le best practice.
</Warning>

I guardrail integrati riducono il rischio senza richiedere configurazione:

* **Approvazione per app**: Claude può controllare solo le app che hai approvato nella sessione corrente.
* **Avvisi sentinel**: le app che concedono accesso shell, filesystem o impostazioni di sistema sono contrassegnate prima che tu le approvi.
* **Terminale escluso dagli screenshot**: Claude non vede mai la finestra del tuo terminale, quindi i prompt sullo schermo nella tua sessione non possono retroalimentare il modello.
* **Escape globale**: il tasto `Esc` interrompe computer use da qualsiasi luogo e la pressione del tasto viene consumata in modo che l'iniezione di prompt non possa usarla per chiudere i dialoghi.
* **File di blocco**: solo una sessione può controllare la tua macchina alla volta.

<h2 id="example-workflows">
  Flussi di lavoro di esempio
</h2>

Questi esempi mostrano modi comuni per combinare computer use con attività di codifica.

<h3 id="validate-a-native-build">
  Convalida una build nativa
</h3>

Dopo aver apportato modifiche a un'app macOS o iOS, chiedi a Claude di compilare e verificare in un unico passaggio:

```text theme={null}
Build the MenuBarStats target, launch it, open the preferences window,
and verify the interval slider updates the label. Screenshot the
preferences window when you're done.
```

Claude esegue `xcodebuild`, avvia l'app, interagisce con l'interfaccia utente e segnala quello che trova.

<h3 id="reproduce-a-layout-bug">
  Riproduci un bug di layout
</h3>

Quando un bug visivo appare solo a determinate dimensioni di finestra, lascia che Claude lo trovi:

```text theme={null}
The settings modal clips its footer on narrow windows. Resize the app
window down until you can reproduce it, screenshot the clipped state,
then check the CSS for the modal container.
```

Claude ridimensiona la finestra, cattura lo stato rotto e legge i fogli di stile pertinenti.

<h3 id="test-a-simulator-flow">
  Testa un flusso di simulatore
</h3>

Guida iOS Simulator senza scrivere XCTest:

```text theme={null}
Open the iOS Simulator, launch the app, tap through the onboarding
screens, and tell me if any screen takes more than a second to load.
```

Claude controlla il simulatore nello stesso modo in cui lo faresti tu con un mouse.

<h2 id="differences-from-the-desktop-app">
  Differenze dall'app Desktop
</h2>

Le superfici CLI e Desktop condividono lo stesso motore di computer use, con alcune differenze:

| Funzionalità                 | Desktop                                                            | CLI                              |
| :--------------------------- | :----------------------------------------------------------------- | :------------------------------- |
| Piattaforme                  | macOS e Windows                                                    | Solo macOS                       |
| Abilita                      | Attiva/disattiva in **Settings > General** (sotto **Desktop app**) | Abilita `computer-use` in `/mcp` |
| Elenco app negate            | Configurabile in Impostazioni                                      | Non ancora disponibile           |
| Attiva/disattiva auto-unhide | Facoltativo                                                        | Sempre attivo                    |
| Integrazione Dispatch        | Le sessioni generate da Dispatch possono usare computer use        | Non applicabile                  |

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="computer-use-is-in-use-by-another-claude-session">
  "Computer use is in use by another Claude session"
</h3>

Un'altra sessione di Claude Code tiene il blocco, che mantiene fino a quando non esce. Esci da quella sessione. Se l'altra sessione si è bloccata, il blocco viene rilasciato automaticamente quando Claude rileva che il processo non è più in esecuzione.

<h3 id="macos-permissions-prompt-keeps-reappearing">
  macOS permissions prompt keeps reappearing
</h3>

macOS a volte richiede un riavvio del processo richiedente dopo aver concesso Screen Recording. Esci completamente da Claude Code e avvia una nuova sessione. Se il prompt persiste, apri **System Settings > Privacy & Security > Screen Recording** e conferma che la tua app terminale è elencata e abilitata.

<h3 id="computer-use-doesn’t-appear-in-/mcp">
  `computer-use` doesn't appear in `/mcp`
</h3>

Il server appare solo su configurazioni idonee. Verifica che:

* Sei su macOS. Computer use nella CLI non è disponibile su Linux o Windows. Su Windows, usa [computer use in Desktop](/docs/it/desktop#let-claude-use-your-computer) invece.
* Sei su un piano Pro o Max. Esegui `/status` per confermare il tuo abbonamento.
* Sei autenticato tramite claude.ai. Computer use non è disponibile con provider di terze parti come Amazon Bedrock, Google Cloud's Agent Platform, o Microsoft Foundry. Se accedi a Claude esclusivamente tramite un provider di terze parti, hai bisogno di un account claude.ai separato per usare questa funzionalità.
* Sei in una sessione interattiva. Computer use non è disponibile in modalità non interattiva con il flag `-p`.

<h2 id="see-also">
  Vedi anche
</h2>

* [Computer use in Desktop](/docs/it/desktop#let-claude-use-your-computer): la stessa funzionalità con una pagina di impostazioni grafica
* [Claude in Chrome](/docs/it/chrome): automazione del browser per attività basate sul web
* [MCP](/docs/it/mcp): connetti Claude a strumenti e API strutturati
* [Sandboxing](/docs/it/sandboxing): come lo strumento Bash di Claude isola l'accesso al filesystem e alla rete
* [Guida alla sicurezza di computer use](https://support.claude.com/en/articles/14128542): best practice per l'uso sicuro di computer use
