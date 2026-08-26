> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Disponibilità delle funzionalità

> Confronta quali funzionalità di Claude Code sono disponibili nei piani di abbonamento Anthropic, nella Console Anthropic, in Amazon Bedrock, su Claude Platform on AWS, in Google Cloud's Agent Platform e in Microsoft Foundry.

Claude Code CLI e tutto ciò che viene eseguito localmente funziona su ogni provider. Per le istruzioni di configurazione per ogni provider, consulta la [panoramica della distribuzione aziendale](/docs/it/third-party-integrations). Per passare direttamente a ciò che manca sul tuo provider, consulta le schede [riepilogo per provider](#summary-by-provider).

Nelle tabelle seguenti, ✓ significa disponibile, ✗ significa non disponibile, e "Vedi nota" rimanda a una nota a piè di pagina per il supporto parziale. Un qualificatore dopo ✓ limita la disponibilità a quel sottoinsieme, e "Admin-enabled" significa che la funzionalità è disattivata finché un amministratore dell'organizzazione non la attiva.

<h2 id="availability-by-model-provider">
  Disponibilità per provider di modelli
</h2>

Il modo in cui ti autentichi determina quali funzionalità Claude Code può raggiungere. Per un singolo elenco di ciò che manca sul tuo provider, consulta le schede [riepilogo per provider](#summary-by-provider). Per trovare la tua colonna nelle tabelle:

* **Abbonamento Claude**: accedi con un account claude.ai nel piano Pro, Max, Team o Enterprise
* **Console Anthropic**: ti autentichi con una chiave API Anthropic
* **Amazon Bedrock**: utilizzi modelli Claude dal catalogo dei modelli Bedrock e imposti `CLAUDE_CODE_USE_BEDROCK`. L'[endpoint Mantle](/docs/it/amazon-bedrock#use-the-mantle-endpoint) (`CLAUDE_CODE_USE_MANTLE`) è coperto da questa colonna
* **Claude Platform on AWS**: hai acquistato Claude tramite AWS Marketplace ma chiami l'API Anthropic, e imposti `CLAUDE_CODE_USE_ANTHROPIC_AWS`
* **Google Cloud's Agent Platform**: gestito da Google; imposti `CLAUDE_CODE_USE_VERTEX`
* **Microsoft Foundry**: gestito da Anthropic su Azure; imposti `CLAUDE_CODE_USE_FOUNDRY`

<h3 id="features-available-on-every-provider">
  Funzionalità disponibili su ogni provider
</h3>

Queste funzionano su ogni provider:

* [CLI](/docs/it/quickstart) e [Agent SDK](/docs/it/agent-sdk/overview)
* Estensioni [VS Code](/docs/it/vs-code) e [JetBrains](/docs/it/jetbrains)
* [Subagents](/docs/it/sub-agents), [hooks](/docs/it/hooks-guide), [commands](/docs/it/commands) e [skills](/docs/it/skills)
* Memoria [CLAUDE.md](/docs/it/memory), [plugins](/docs/it/plugins) e [server MCP](/docs/it/mcp)
* [Checkpoints](/docs/it/checkpointing), [sandboxing](/docs/it/sandboxing) e [Workflows](/docs/it/workflows)
* Metriche [OpenTelemetry](/docs/it/monitoring-usage) e il [file di impostazioni gestito](/docs/it/settings#settings-files)

Tre di questi hanno differenze specifiche del provider:

* **Server MCP**: i [connettori da claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai) si caricano solo quando il tuo abbonamento claude.ai è il metodo di autenticazione attivo, e la [ricerca degli strumenti](/docs/it/mcp#configure-tool-search) è disattivata per impostazione predefinita su Google Cloud's Agent Platform e quando `ANTHROPIC_BASE_URL` punta a un host non di prima parte
* **Subagents**: il [subagent Explore integrato](/docs/it/sub-agents#built-in-subagents) limita il suo modello ereditato a Opus sull'API Claude, e eredita il modello della conversazione principale direttamente su qualsiasi altro provider, incluso Claude Platform on AWS
* **[Commands](/docs/it/commands#all-commands)**: `/design-sync` e `/radio` non sono disponibili su Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry e Claude Platform on AWS, e `/voice` richiede un account claude.ai

<h3 id="features-that-require-a-claude-subscription">
  Funzionalità che richiedono un abbonamento Claude
</h3>

Queste richiedono l'accesso con un account claude.ai e non sono raggiungibili con una chiave API della Console Anthropic o da un provider di terze parti:

* [Claude Code sul web](/docs/it/claude-code-on-the-web), Claude Code su mobile e [Claude Code in Slack](/docs/it/slack)
* [Claude Code Desktop](/docs/it/desktop)
* [Routines](/docs/it/routines) (`/schedule`)
* [Ultraplan](/docs/it/ultraplan) e [Ultrareview](/docs/it/ultrareview)
* [Code Review](/docs/it/code-review): piani Team e Enterprise
* [Remote Control](/docs/it/remote-control)
* [Estensione Chrome](/docs/it/chrome)
* [Computer use](/docs/it/computer-use): piani Pro e Max
* [Artifacts](/docs/it/artifacts): piani Pro, Max, Team e Enterprise
* [Voice dictation](/docs/it/voice-dictation)

Desktop è l'eccezione parziale: il [routing del gateway può essere configurato nell'app o da un amministratore](/docs/it/llm-gateway-connect#desktop-app), le distribuzioni Enterprise possono instradare Desktop a Google Cloud's Agent Platform o a un provider gateway tramite [impostazioni gestite](https://claude.com/docs/third-party/claude-desktop/configuration), e [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview) esegue la scheda Code su Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry o un gateway LLM self-hosted. Per la disponibilità per piano di queste funzionalità, consulta [Disponibilità per piano di abbonamento](#availability-by-subscription-plan).

<h3 id="cli-capabilities-that-vary-by-provider">
  Funzionalità CLI che variano per provider
</h3>

Queste funzionalità funzionano nella CLI locale ma dipendono da una funzionalità lato server che non tutti i provider espongono.

<table>
  <thead>
    <tr>
      <th>Funzionalità</th>
      <th>Abbonamento Claude</th>
      <th>Console Anthropic</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform on AWS</th>
      <th>Google Cloud's Agent Platform</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>[Web search](/docs/it/tools-reference#websearch-tool-behavior)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✓</td>
      <td>Vedi nota <sup><a href="#fn1">1</a></sup></td>
      <td>✓</td>
    </tr>

    <tr>
      <td>[Fast mode](/docs/it/fast-mode)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Auto mode](/docs/it/auto-mode-config)</td>
      <td>✓</td>
      <td>✓</td>
      <td>Vedi nota <sup><a href="#fn2">2</a></sup></td>
      <td>✓</td>
      <td>Vedi nota <sup><a href="#fn2">2</a></sup></td>
      <td>Vedi nota <sup><a href="#fn2">2</a></sup></td>
    </tr>

    <tr>
      <td>[Advisor](/docs/it/advisor)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Channels](/docs/it/channels)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[`/loop` attività pianificate](/docs/it/scheduled-tasks)</td>
      <td>✓</td>
      <td>✓</td>
      <td>Vedi nota <sup><a href="#fn3">3</a></sup></td>
      <td>Vedi nota <sup><a href="#fn3">3</a></sup></td>
      <td>Vedi nota <sup><a href="#fn3">3</a></sup></td>
      <td>Vedi nota <sup><a href="#fn3">3</a></sup></td>
    </tr>

    <tr>
      <td>[GitHub Actions](/docs/it/github-actions) e [GitLab CI/CD](/docs/it/gitlab-ci-cd)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
    </tr>
  </tbody>
</table>

<h3 id="admin-and-analytics">
  Admin e analytics
</h3>

Controlli a livello organizzativo e visibilità dell'utilizzo.

<table>
  <thead>
    <tr>
      <th>Funzionalità</th>
      <th>Abbonamento Claude</th>
      <th>Console Anthropic</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform on AWS</th>
      <th>Google Cloud's Agent Platform</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>[Dashboard analytics e API](/docs/it/analytics)</td>
      <td>✓ (dashboard: Team e Enterprise; API: Enterprise)</td>
      <td>✓ <sup><a href="#fn5">5</a></sup></td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Impostazioni gestite dal server](/docs/it/server-managed-settings)</td>
      <td>✓ (Team e Enterprise)</td>
      <td>✓ (Team e Enterprise)</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Zero Data Retention](/docs/it/zero-data-retention)</td>
      <td>✓ (account Enterprise qualificati)</td>
      <td>✓ (account qualificati)</td>
      <td>Vedi nota <sup><a href="#fn4">4</a></sup></td>
      <td>✓ (account qualificati)</td>
      <td>Vedi nota <sup><a href="#fn4">4</a></sup></td>
      <td>Vedi nota <sup><a href="#fn4">4</a></sup></td>
    </tr>
  </tbody>
</table>

<span id="fn1" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>1</sup> Su Google Cloud's Agent Platform, la ricerca web è disponibile per i modelli Claude 4 e successivi.<br />
<span id="fn2" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>2</sup> Su questi provider, auto mode supporta solo Claude Sonnet 5, Opus 4.7 e Opus 4.8. Consulta [Configurazione Auto mode](/docs/it/auto-mode-config). Dalla v2.1.158 alla v2.1.206, auto mode su questi provider richiedeva anche l'impostazione di `CLAUDE_CODE_ENABLE_AUTO_MODE=1`; v2.1.207 ha rimosso il requisito.<br />
<span id="fn3" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>3</sup> Gli intervalli espliciti come `/loop every 2 hours` funzionano su ogni provider. Su Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform e Microsoft Foundry, `/loop` non può scegliere il proprio intervallo o fornire il prompt di manutenzione predefinito, quindi un prompt senza intervallo viene eseguito ogni 10 minuti, e `/loop` senza argomenti mostra il messaggio di utilizzo. Consulta [Attività pianificate](/docs/it/scheduled-tasks).<br />
<span id="fn4" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>4</sup> Soggetto al tuo accordo con il provider cloud.<br />
<span id="fn5" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>5</sup> Solo dashboard e API. [Metriche di contribuzione](/docs/it/analytics#enable-contribution-metrics) richiede un'organizzazione Team o Enterprise su claude.ai.

<Note>
  Se ti autentichi tramite un [gateway LLM](/docs/it/llm-gateway), la disponibilità delle funzionalità corrisponde al provider sottostante a cui il gateway inoltra. Alcune funzionalità solo Anthropic come l'[Advisor](/docs/it/advisor) funzionano solo se il gateway inoltra le richieste intatte all'API Anthropic.
</Note>

<h3 id="summary-by-provider">
  Riepilogo per provider
</h3>

Ogni scheda elenca ciò che non è disponibile o è parzialmente supportato su quel provider, con alternative dove ne esiste una. Tutto ciò che non è elencato funziona allo stesso modo di un abbonamento Claude, a parte le [differenze specifiche del provider](#features-available-on-every-provider) indicate sopra. Su Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry e Claude Platform on AWS, la segnalazione degli errori e la telemetria ad Anthropic sono disattivate per impostazione predefinita. Consulta [comportamenti predefiniti per provider API](/docs/it/data-usage#default-behaviors-by-api-provider) per il traffico che raggiunge ancora Anthropic e come rinunciare.

<Tabs>
  <Tab title="Amazon Bedrock">
    **Non disponibile:** tutte le [funzionalità che richiedono un abbonamento Claude](#features-that-require-a-claude-subscription), più [web search](/docs/it/tools-reference#websearch-tool-behavior), [fast mode](/docs/it/fast-mode), [Advisor](/docs/it/advisor), [Channels](/docs/it/channels), il [dashboard analytics](/docs/it/analytics), le [impostazioni gestite dal server](/docs/it/server-managed-settings) e i [comandi `/design-sync` e `/radio`](/docs/it/commands#all-commands).

    **Supporto parziale:**

    * [Desktop](/docs/it/desktop): solo tramite [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Auto mode](/docs/it/auto-mode-config): Sonnet 5, Opus 4.7 e Opus 4.8 soltanto
    * [`/loop`](/docs/it/scheduled-tasks): solo intervalli espliciti
    * [Zero Data Retention](/docs/it/zero-data-retention): soggetto al tuo accordo AWS

    **Alternative:** per la pianificazione, utilizza [`/loop`](/docs/it/scheduled-tasks) con un intervallo esplicito invece di `/schedule`. Per le sessioni cloud, utilizza [GitHub Actions](/docs/it/github-actions) o [GitLab CI/CD](/docs/it/gitlab-ci-cd). Per le ricerche web, utilizza lo [strumento WebFetch](/docs/it/tools-reference#webfetch-tool-behavior) con un URL specifico.
  </Tab>

  <Tab title="Claude Platform on AWS">
    **Non disponibile:** tutte le [funzionalità che richiedono un abbonamento Claude](#features-that-require-a-claude-subscription), più [fast mode](/docs/it/fast-mode), [Advisor](/docs/it/advisor), [Channels](/docs/it/channels), il [dashboard analytics](/docs/it/analytics), le [impostazioni gestite dal server](/docs/it/server-managed-settings) e i [comandi `/design-sync` e `/radio`](/docs/it/commands#all-commands).

    **Disponibile dove Amazon Bedrock non lo è:** [web search](/docs/it/tools-reference#websearch-tool-behavior).

    **Supporto parziale:**

    * [`/loop`](/docs/it/scheduled-tasks): solo intervalli espliciti

    **Alternative:** per la pianificazione, utilizza [`/loop`](/docs/it/scheduled-tasks) con un intervallo esplicito invece di `/schedule`. Per le sessioni cloud, utilizza [GitHub Actions](/docs/it/github-actions) o [GitLab CI/CD](/docs/it/gitlab-ci-cd).
  </Tab>

  <Tab title="Google Cloud's Agent Platform">
    **Non disponibile:** tutte le [funzionalità che richiedono un abbonamento Claude](#features-that-require-a-claude-subscription), più [fast mode](/docs/it/fast-mode), [Advisor](/docs/it/advisor), [Channels](/docs/it/channels), il [dashboard analytics](/docs/it/analytics), le [impostazioni gestite dal server](/docs/it/server-managed-settings) e i [comandi `/design-sync` e `/radio`](/docs/it/commands#all-commands).

    **Supporto parziale:**

    * [Desktop](/docs/it/desktop): tramite [impostazioni gestite](https://claude.com/docs/third-party/claude-desktop/configuration) o [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Web search](/docs/it/tools-reference#websearch-tool-behavior): modelli Claude 4 e successivi
    * [Auto mode](/docs/it/auto-mode-config): Sonnet 5, Opus 4.7 e Opus 4.8 soltanto
    * [`/loop`](/docs/it/scheduled-tasks): solo intervalli espliciti
    * [Zero Data Retention](/docs/it/zero-data-retention): soggetto al tuo accordo Google Cloud

    **Alternative:** per la pianificazione, utilizza [`/loop`](/docs/it/scheduled-tasks) con un intervallo esplicito invece di `/schedule`. Per le sessioni cloud, utilizza [GitHub Actions](/docs/it/github-actions) o [GitLab CI/CD](/docs/it/gitlab-ci-cd).
  </Tab>

  <Tab title="Microsoft Foundry">
    **Non disponibile:** tutte le [funzionalità che richiedono un abbonamento Claude](#features-that-require-a-claude-subscription), più [fast mode](/docs/it/fast-mode), [Advisor](/docs/it/advisor), [Channels](/docs/it/channels), [GitHub Actions](/docs/it/github-actions) e [GitLab CI/CD](/docs/it/gitlab-ci-cd), il [dashboard analytics](/docs/it/analytics), le [impostazioni gestite dal server](/docs/it/server-managed-settings) e i [comandi `/design-sync` e `/radio`](/docs/it/commands#all-commands).

    **Supporto parziale:**

    * [Desktop](/docs/it/desktop): solo tramite [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Auto mode](/docs/it/auto-mode-config): Sonnet 5, Opus 4.7 e Opus 4.8 soltanto
    * [`/loop`](/docs/it/scheduled-tasks): solo intervalli espliciti
    * [Zero Data Retention](/docs/it/zero-data-retention): soggetto al tuo accordo Azure

    **Alternative:** per la pianificazione, utilizza [`/loop`](/docs/it/scheduled-tasks) con un intervallo esplicito invece di `/schedule`.
  </Tab>

  <Tab title="Console Anthropic">
    **Non disponibile:** tutte le [funzionalità che richiedono un abbonamento Claude](#features-that-require-a-claude-subscription).

    Tutto in [Funzionalità CLI che variano per provider](#cli-capabilities-that-vary-by-provider) è disponibile, così come le [impostazioni gestite dal server](/docs/it/server-managed-settings) quando la chiave API appartiene a un'organizzazione Team o Enterprise.
  </Tab>
</Tabs>

<h2 id="availability-by-subscription-plan">
  Disponibilità per piano di abbonamento
</h2>

Se ti autentichi tramite Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry o una chiave API della Console Anthropic, questa sezione non si applica a te. Quando accedi con un account claude.ai, il tuo piano determina quali delle funzionalità seguenti sono disponibili.

| Funzionalità                                                                | Pro | Max | Team          | Enterprise                        |
| :-------------------------------------------------------------------------- | :-- | :-- | :------------ | :-------------------------------- |
| [Claude Code sul web](/docs/it/claude-code-on-the-web)                           | ✓   | ✓   | ✓             | ✓ <sup><a href="#fn6">6</a></sup> |
| [Routines](/docs/it/routines)                                                    | ✓   | ✓   | ✓             | ✓                                 |
| [Remote Control](/docs/it/remote-control)                                        | ✓   | ✓   | Admin-enabled | Admin-enabled                     |
| [Channels](/docs/it/channels)                                                    | ✓   | ✓   | Admin-enabled | Admin-enabled                     |
| [Computer use](/docs/it/computer-use)                                            | ✓   | ✓   | ✗             | ✗                                 |
| Dispatch ([Desktop](/docs/it/desktop#sessions-from-dispatch))                    | ✓   | ✓   | ✗             | ✗                                 |
| [Code Review](/docs/it/code-review)                                              | ✗   | ✗   | ✓             | ✓                                 |
| [Artifacts](/docs/it/artifacts)                                                  | ✓   | ✓   | ✓             | Admin-enabled                     |
| [Dashboard analytics e metriche di contribuzione](/docs/it/analytics)            | ✗   | ✗   | ✓             | ✓                                 |
| [Enterprise Analytics API](/docs/it/analytics#access-data-programmatically)      | ✗   | ✗   | ✗             | ✓                                 |
| [Impostazioni gestite dal server](/docs/it/server-managed-settings)              | ✗   | ✗   | ✓             | ✓                                 |
| [SSO](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) | ✗   | ✗   | ✓             | ✓                                 |
| SCIM                                                                        | ✗   | ✗   | ✗             | ✓                                 |
| [Compliance API](https://platform.claude.com/docs/en/api/compliance)        | ✗   | ✗   | ✗             | ✓                                 |
| [Zero Data Retention](/docs/it/zero-data-retention)                              | ✗   | ✗   | ✗             | ✓ <sup><a href="#fn7">7</a></sup> |

<span id="fn6" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>6</sup> Su Enterprise, richiede un premium seat o un Chat + Claude Code seat. Consulta [Claude Code sul web](/docs/it/claude-code-on-the-web).<br />
<span id="fn7" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>7</sup> Non incluso nel piano Enterprise standard. Richiede un'abilitazione separata da Anthropic per account qualificati. Consulta [Zero Data Retention](/docs/it/zero-data-retention).

Per i prezzi e il confronto completo dei piani, consulta [Piani Team](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) e [Piani Enterprise](https://support.claude.com/en/articles/9797531-what-is-the-enterprise-plan).

<h2 id="model-availability">
  Disponibilità dei modelli
</h2>

Per sapere quali modelli Claude e dimensioni della finestra di contesto sono disponibili per provider e regione, consulta [Configurazione dei modelli](/docs/it/model-config) e la [panoramica dei modelli](https://platform.claude.com/docs/en/about-claude/models/overview). Vision, input PDF e extended thinking sono funzionalità dei modelli piuttosto che funzionalità di Claude Code e funzionano su ogni provider che offre il modello. [Prompt caching](/docs/it/prompt-caching) funziona allo stesso modo sulla maggior parte dei provider; su Amazon Bedrock, il supporto varia in base al modello.

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Panoramica della distribuzione aziendale](/docs/it/third-party-integrations): confronta autenticazione, fatturazione e regioni tra i provider
* Guide di configurazione del provider: [Amazon Bedrock](/docs/it/amazon-bedrock), [Claude Platform on AWS](/docs/it/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai), [Microsoft Foundry](/docs/it/microsoft-foundry)
* [Piattaforme e integrazioni](/docs/it/platforms): dove Claude Code viene eseguito, inclusi CLI, Desktop, estensioni IDE, web, mobile e CI/CD
