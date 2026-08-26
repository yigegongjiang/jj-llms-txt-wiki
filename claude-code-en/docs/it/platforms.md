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
| [Web](/docs/it/claude-code-on-the-web) | Attività a lunga esecuzione che non richiedono molto controllo, o lavoro che dovrebbe continuare quando sei offline | Cloud gestito da Anthropic, continua dopo la disconnessione                                                                                                                           |
| Mobile                            | Avviare e monitorare attività mentre sei lontano dal tuo computer                                                   | Sessioni cloud dall'app Claude per iOS e Android, [Remote Control](/docs/it/remote-control) per sessioni locali, [Dispatch](/docs/it/desktop#sessions-from-dispatch) verso Desktop su Pro e Max |

La CLI è la superficie più completa per il lavoro nativo da terminale: scripting e Agent SDK sono solo CLI. I provider di terze parti funzionano anche in [VS Code](/docs/it/vs-code#use-third-party-providers). Le distribuzioni [Desktop](/docs/it/desktop) aziendali supportano Google Cloud's Agent Platform, e Desktop supporta [provider gateway](/docs/it/llm-gateway-connect#desktop-app); per Amazon Bedrock o Microsoft Foundry, usa la CLI o VS Code, oppure [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview), che esegue la scheda Code su questi provider. Desktop e le estensioni IDE scambiano alcune funzionalità solo CLI per revisione visiva e integrazione editor più stretta. Il web viene eseguito nel cloud di Anthropic, quindi le attività continuano dopo la disconnessione. Mobile è un thin client nelle stesse sessioni cloud o in una sessione locale tramite Remote Control, e può inviare attività a Desktop con Dispatch.

Puoi mescolare superfici sullo stesso progetto. La configurazione, la memoria del progetto e i server MCP sono condivisi tra le superfici locali.

<h2 id="connect-your-tools">
  Collega i tuoi strumenti
</h2>

Le integrazioni consentono a Claude di lavorare con servizi al di fuori della tua base di codice.

| Integrazione                         | Cosa fa                                               | Usala per                                                             |
| :----------------------------------- | :---------------------------------------------------- | :-------------------------------------------------------------------- |
| [Chrome](/docs/it/chrome)                 | Controlla il tuo browser con le tue sessioni connesse | Test di app web, compilazione moduli, automazione siti senza API      |
| [GitHub Actions](/docs/it/github-actions) | Esegue Claude nella tua pipeline CI                   | Revisioni PR automatizzate, triage problemi, manutenzione programmata |
| [GitLab CI/CD](/docs/it/gitlab-ci-cd)     | Come GitHub Actions per GitLab                        | Automazione guidata da CI su GitLab                                   |
| [Code Review](/docs/it/code-review)       | Rivede automaticamente ogni PR                        | Catturare bug prima della revisione umana                             |
| [Slack](/docs/it/slack)                   | Risponde alle menzioni `@Claude` nei tuoi canali      | Trasformare segnalazioni di bug in pull request dalla chat del team   |

Per integrazioni non elencate qui, [server MCP](/docs/it/mcp) e [connettori](/docs/it/desktop#connect-external-tools) ti permettono di collegare quasi tutto: Linear, Notion, Google Drive o le tue API interne.

<h2 id="work-when-you-are-away-from-your-terminal">
  Lavora quando sei lontano dal tuo terminale
</h2>

Claude Code offers several ways to work when you're not at your terminal. They differ in what triggers the work, where Claude runs, and how much you need to set up.

|                                                          | Trigger                                                                                        | Claude runs on                                                                               | Setup                                                                                                                                | Best for                                                      |
| :------------------------------------------------------- | :--------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------ |
| [Dispatch](/docs/en/desktop#sessions-from-dispatch)           | Message a task from the Claude mobile app                                                      | Your machine (Desktop)                                                                       | [Pair the mobile app with Desktop](https://support.claude.com/en/articles/13947068)                                                  | Delegating work while you're away, minimal setup              |
| [Remote Control](/docs/en/remote-control)                     | Drive a running session from [claude.ai/code](https://claude.ai/code) or the Claude mobile app | Your machine (CLI or VS Code)                                                                | Run `claude remote-control`                                                                                                          | Steering in-progress work from another device                 |
| [Channels](/docs/en/channels)                                 | Push events from a chat app like Telegram or Discord, or your own server                       | Your machine (CLI)                                                                           | [Install a channel plugin](/docs/en/channels#quickstart) or [build your own](/docs/en/channels-reference)                                      | Reacting to external events like CI failures or chat messages |
| [Slack](/docs/en/slack)                                       | Mention `@Claude` in a team channel                                                            | Anthropic cloud                                                                              | [Install the Slack app](/docs/en/slack#setting-up-claude-code-in-slack) with [Claude Code on the web](/docs/en/claude-code-on-the-web) enabled | PRs and reviews from team chat                                |
| [Self-hosted environments](/docs/en/self-hosted-environments) | Start a [cloud session](/docs/en/claude-code-on-the-web) and pick your organization's environment   | Your organization's infrastructure                                                           | [Deploy runners](/docs/en/self-hosted-environments-quickstart), on Team and Enterprise plans                                              | Cloud sessions that must run inside your network              |
| [Scheduled tasks](/docs/en/scheduled-tasks)                   | Set a schedule                                                                                 | [CLI](/docs/en/scheduled-tasks), [Desktop](/docs/en/desktop-scheduled-tasks), or [cloud](/docs/en/routines) | Pick a frequency                                                                                                                     | Recurring automation like daily reviews                       |

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
* [Claude Code sul web](/docs/it/claude-code-on-the-web): sessioni cloud che continuano a funzionare quando ti disconnetti
* Mobile: l'app Claude per [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) e [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) per avviare e monitorare attività mentre sei lontano dal tuo computer

<h3 id="integrations">
  Integrazioni
</h3>

* [Chrome](/docs/it/chrome): automatizza attività del browser con le tue sessioni connesse
* [Computer use](/docs/it/computer-use): consenti a Claude di aprire app e controllare il tuo schermo su macOS
* [GitHub Actions](/docs/it/github-actions): esegui Claude nella tua pipeline CI
* [GitLab CI/CD](/docs/it/gitlab-ci-cd): lo stesso per GitLab
* [Code Review](/docs/it/code-review): revisione automatica su ogni pull request
* [Slack](/docs/it/slack): invia attività dalla chat del team, ricevi PR indietro

<h3 id="remote-access">
  Accesso remoto
</h3>

* [Dispatch](/docs/it/desktop#sessions-from-dispatch): invia un'attività dal tuo telefono e può generare una sessione Desktop
* [Remote Control](/docs/it/remote-control): guida una sessione in esecuzione dal tuo telefono o browser
* [Channels](/docs/it/channels): invia eventi da app di chat o dai tuoi server in una sessione
* [Attività programmate](/docs/it/scheduled-tasks): esegui prompt su base ricorrente
