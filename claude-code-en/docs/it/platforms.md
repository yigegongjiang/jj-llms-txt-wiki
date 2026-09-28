> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Piattaforme e integrazioni

> Scegli dove eseguire Claude Code e cosa collegare. Confronta CLI, Desktop, VS Code, JetBrains, web, mobile e integrazioni come Chrome, Slack e CI/CD.

Claude Code esegue lo stesso motore sottostante ovunque, ma ogni superficie è ottimizzata per un modo diverso di lavorare. Questa pagina ti aiuta a scegliere la piattaforma giusta per il tuo flusso di lavoro e a collegare gli strumenti che già utilizzi.

<h2 id="where-to-run-claude-code">
  Dove eseguire Claude Code
</h2>

Scegli una piattaforma in base a come preferisci lavorare e dove si trova il tuo progetto.

| Piattaforma                       | Ideale per                                                                                                          | Cosa ottieni                                                                                                                                                                          |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [CLI](/docs/it/quickstart)             | Flussi di lavoro da terminale, scripting, server remoti                                                             | Set completo di funzionalità, [Agent SDK](/docs/it/headless), [computer use](/docs/it/computer-use) su macOS (Pro e Max), provider di terze parti                                               |
| [Desktop](/docs/it/desktop)            | Revisione visiva, sessioni parallele, configurazione gestita                                                        | Visualizzatore diff, anteprima app, [computer use](/docs/it/desktop#let-claude-use-your-computer) e [Dispatch](/docs/it/desktop#sessions-from-dispatch) su Pro e Max                            |
| [VS Code](/docs/it/vs-code)            | Lavorare all'interno di VS Code senza passare a un terminale                                                        | Diff inline, terminale integrato, contesto file                                                                                                                                       |
| [JetBrains](/docs/it/jetbrains)        | Lavorare all'interno di IntelliJ, PyCharm, WebStorm o altri IDE JetBrains                                           | Visualizzatore diff, condivisione selezione, sessione terminale                                                                                                                       |
| [Web](/docs/it/claude-code-on-the-web) | Attività a lunga esecuzione che non richiedono molto controllo, o lavoro che dovrebbe continuare quando sei offline | Cloud, gestito da Anthropic per impostazione predefinita; continua dopo la disconnessione                                                                                             |
| [Mobile](/docs/it/mobile)              | Avviare e monitorare attività mentre sei lontano dal tuo computer                                                   | Sessioni cloud dall'app Claude per iOS e Android, [Remote Control](/docs/it/remote-control) per sessioni locali, [Dispatch](/docs/it/desktop#sessions-from-dispatch) verso Desktop su Pro e Max |

La CLI è la superficie più completa per il lavoro nativo da terminale: scripting e Agent SDK sono solo CLI. I provider di terze parti funzionano anche in [VS Code](/docs/it/vs-code#use-third-party-providers) e in [JetBrains](/docs/it/feature-availability#features-available-on-every-provider), che esegue la CLI nel terminale del tuo IDE. Le distribuzioni [Desktop](/docs/it/desktop) aziendali supportano Google Cloud's Agent Platform, e Desktop supporta [provider gateway](/docs/it/llm-gateway-connect#desktop-app); per Amazon Bedrock o Microsoft Foundry, usa la CLI o un'estensione IDE, oppure [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview), che esegue la scheda Code su questi provider. Desktop e le estensioni IDE scambiano alcune funzionalità solo CLI per revisione visiva e integrazione editor più stretta. Il web viene eseguito nel cloud, quindi le attività continuano dopo la disconnessione. Mobile è un thin client nelle stesse sessioni cloud o in una sessione locale tramite Remote Control, e può inviare attività a Desktop con Dispatch.

Puoi mescolare superfici sullo stesso progetto. La configurazione, la memoria del progetto e i server MCP sono condivisi tra le superfici locali.

<h2 id="connect-your-tools">
  Collega i tuoi strumenti
</h2>

Le integrazioni consentono a Claude di lavorare con servizi al di fuori della tua base di codice.

| Integrazione                                     | Cosa fa                                                                                                       | Usala per                                                                                   |
| :----------------------------------------------- | :------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------ |
| [Chrome](/docs/it/chrome)                             | Controlla il tuo browser con le tue sessioni connesse                                                         | Test di app web, compilazione moduli, automazione siti senza API                            |
| [GitHub Actions](/docs/it/github-actions)             | Esegue Claude nella tua pipeline CI                                                                           | Revisioni PR automatizzate, triage problemi, manutenzione programmata                       |
| [GitLab CI/CD](/docs/it/gitlab-ci-cd)                 | Come GitHub Actions per GitLab                                                                                | Automazione guidata da CI su GitLab                                                         |
| [Code Review](/docs/it/code-review)                   | Rivede automaticamente ogni PR                                                                                | Catturare bug prima della revisione umana                                                   |
| [Slack](/docs/it/slack)                               | Risponde alle menzioni `@Claude` nei tuoi canali                                                              | Trasformare segnalazioni di bug in pull request dalla chat del team                         |
| [Claude Tag](https://claude.com/docs/claude-tag) | Esegue `@Claude` come identità condivisa della tua organizzazione con accesso configurato dall'amministratore | Accesso condiviso del team su piani Team ed Enterprise, invece di sessioni Slack per utente |

Per integrazioni non elencate qui, [server MCP](/docs/it/mcp) e [connettori](/docs/it/desktop#connect-external-tools) ti permettono di collegare quasi tutto: Linear, Notion, Google Drive o le tue API interne.

<h2 id="work-when-you-are-away-from-your-terminal">
  Lavora quando sei lontano dal tuo terminale
</h2>

Claude Code offre diversi modi di lavorare quando non sei al tuo terminale. Differiscono in ciò che attiva il lavoro, dove Claude viene eseguito e quanto setup è necessario.

|                                                          | Trigger                                                                                               | Claude viene eseguito su                                                                    | Setup                                                                                                                             | Migliore per                                                         |
| :------------------------------------------------------- | :---------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------- |
| [Dispatch](/docs/it/desktop#sessions-from-dispatch)           | Invia un'attività dall'app mobile Claude                                                              | La tua macchina (Desktop)                                                                   | [Associa l'app mobile a Desktop](https://support.claude.com/en/articles/13947068)                                                 | Delegare il lavoro mentre sei via, setup minimo                      |
| [Remote Control](/docs/it/remote-control)                     | Guida una sessione in esecuzione da [claude.ai/code](https://claude.ai/code) o dall'app mobile Claude | La tua macchina (CLI o VS Code)                                                             | Esegui `claude remote-control`                                                                                                    | Guidare il lavoro in corso da un altro dispositivo                   |
| [Channels](/docs/it/channels)                                 | Invia eventi da un'app di chat come Telegram o Discord, o dal tuo server                              | La tua macchina (CLI)                                                                       | [Installa un plugin channel](/docs/it/channels#quickstart) o [crea il tuo](/docs/it/channels-reference)                                     | Reagire a eventi esterni come errori CI o messaggi di chat           |
| [Slack](/docs/it/slack)                                       | Menziona `@Claude` in un canale del team                                                              | Cloud Anthropic                                                                             | [Installa l'app Slack](/docs/it/slack#setting-up-claude-code-in-slack) con [Claude Code sul web](/docs/it/claude-code-on-the-web) abilitato | PR e revisioni dalla chat del team                                   |
| [Self-hosted environments](/docs/it/self-hosted-environments) | Avvia una [sessione cloud](/docs/it/claude-code-on-the-web) e scegli l'ambiente della tua organizzazione   | L'infrastruttura della tua organizzazione                                                   | [Distribuisci runner](/docs/it/self-hosted-environments-quickstart), su piani Team e Enterprise                                        | Sessioni cloud che devono essere eseguite all'interno della tua rete |
| [Scheduled tasks](/docs/it/scheduled-tasks)                   | Imposta una pianificazione                                                                            | [CLI](/docs/it/scheduled-tasks), [Desktop](/docs/it/desktop-scheduled-tasks), o [cloud](/docs/it/routines) | Scegli una frequenza                                                                                                              | Automazione ricorrente come revisioni giornaliere                    |

Se non sei sicuro da dove iniziare, [installa la CLI](/docs/it/quickstart) ed eseguila in una directory di progetto. Se preferisci non usare un terminale, [Desktop](/docs/it/desktop-quickstart) ti offre lo stesso motore con un'interfaccia grafica.

<h2 id="related-resources">
  Risorse correlate
</h2>

<h3 id="platforms">
  Piattaforme
</h3>

* [Guida rapida CLI](/docs/it/quickstart): installa ed esegui il tuo primo comando nel terminale
* [Desktop](/docs/it/desktop): revisione diff visiva, sessioni parallele, computer use e Dispatch
* [VS Code](/docs/it/vs-code): l'estensione Claude Code all'interno del tuo editor
* [JetBrains](/docs/it/jetbrains): l'estensione per IntelliJ, PyCharm e altri IDE JetBrains
* [Web](/docs/it/claude-code-on-the-web): sessioni cloud da browser su claude.ai/code che continuano a funzionare quando ti disconnetti
* [Projects](/docs/it/claude-projects): una conversazione in cui Claude coordina molte sessioni cloud per un corpo di lavoro e riferisce i risultati
* [Mobile](/docs/it/mobile): l'app Claude per [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) e [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) per avviare e monitorare attività mentre sei lontano dal tuo computer

<h3 id="integrations">
  Integrazioni
</h3>

* [Chrome](/docs/it/chrome): automatizza attività del browser con le tue sessioni connesse
* [Computer use](/docs/it/computer-use): consenti a Claude di aprire app e controllare il tuo schermo su macOS
* [GitHub Actions](/docs/it/github-actions): esegui Claude nella tua pipeline CI
* [GitLab CI/CD](/docs/it/gitlab-ci-cd): lo stesso per GitLab
* [Code Review](/docs/it/code-review): revisione automatica su ogni pull request
* [Slack](/docs/it/slack): invia attività dalla chat del team, ricevi PR indietro
* [Claude Tag](https://claude.com/docs/claude-tag): esegui `@Claude` come identità condivisa della tua organizzazione su piani Team ed Enterprise

<h3 id="remote-access">
  Accesso remoto
</h3>

* [Dispatch](/docs/it/desktop#sessions-from-dispatch): invia un'attività dal tuo telefono e può generare una sessione Desktop
* [Remote Control](/docs/it/remote-control): guida una sessione in esecuzione dal tuo telefono o browser
* [Channels](/docs/it/channels): invia eventi da app di chat o dai tuoi server in una sessione
* [Attività programmate](/docs/it/scheduled-tasks): esegui prompt su base ricorrente
