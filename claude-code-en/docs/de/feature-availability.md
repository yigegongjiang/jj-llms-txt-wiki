> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Verfügbarkeit von Funktionen

> Vergleichen Sie, welche Claude Code-Funktionen in Anthropic-Abonnementplänen, der Anthropic Console, Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform und Microsoft Foundry verfügbar sind.

Die Claude Code CLI und alles, was lokal ausgeführt wird, funktioniert auf jedem Anbieter identisch. Anweisungen zur Einrichtung pro Anbieter finden Sie in der [Übersicht zur Enterprise-Bereitstellung](/docs/de/third-party-integrations). Um direkt zu sehen, was auf Ihrem Anbieter fehlt, siehe die Registerkarten [Zusammenfassung nach Anbieter](#summary-by-provider).

In den Tabellen unten bedeutet ✓ verfügbar, ✗ nicht verfügbar, und „Siehe Hinweis" verlinkt auf eine Fußnote für teilweise Unterstützung. Ein Qualifizierer nach ✓ grenzt die Verfügbarkeit auf diese Teilmenge ein, und „Admin-aktiviert" bedeutet, dass die Funktion deaktiviert ist, bis ein Organisations-Admin sie aktiviert.

<h2 id="availability-by-model-provider">
  Verfügbarkeit nach Modell-Anbieter
</h2>

Wie Sie sich authentifizieren, bestimmt, welche Funktionen Claude Code erreichen kann. Eine einzelne Liste dessen, was auf Ihrem Anbieter fehlt, finden Sie in den Registerkarten [Zusammenfassung nach Anbieter](#summary-by-provider). Um Ihre Spalte in den Tabellen zu finden:

* **Claude-Abonnement**: Sie melden sich mit einem claude.ai-Konto im Pro-, Max-, Team- oder Enterprise-Plan an
* **Anthropic Console**: Sie authentifizieren sich mit einem Anthropic API-Schlüssel
* **Amazon Bedrock**: Sie verwenden Claude-Modelle aus dem Amazon Bedrock-Modellkatalog und setzen `CLAUDE_CODE_USE_BEDROCK`. Der [Mantle-Endpunkt](/docs/de/amazon-bedrock#use-the-mantle-endpoint) (`CLAUDE_CODE_USE_MANTLE`) wird von dieser Spalte abgedeckt
* **Claude Platform on AWS**: Sie haben Claude über AWS Marketplace gekauft, rufen aber die Anthropic API auf und setzen `CLAUDE_CODE_USE_ANTHROPIC_AWS`
* **Google Cloud's Agent Platform**: Von Google betrieben; Sie setzen `CLAUDE_CODE_USE_VERTEX`
* **Microsoft Foundry**: Von Anthropic auf Azure betrieben; Sie setzen `CLAUDE_CODE_USE_FOUNDRY`

<h3 id="features-available-on-every-provider">
  Funktionen, die auf jedem Anbieter verfügbar sind
</h3>

Diese funktionieren auf jedem Anbieter:

* [CLI](/docs/de/quickstart) und [Agent SDK](/docs/de/agent-sdk/overview)
* [VS Code](/docs/de/vs-code) und [JetBrains](/docs/de/jetbrains) Erweiterungen
* [Subagents](/docs/de/sub-agents), [hooks](/docs/de/hooks-guide), [commands](/docs/de/commands) und [skills](/docs/de/skills)
* [CLAUDE.md memory](/docs/de/memory), [plugins](/docs/de/plugins) und [MCP servers](/docs/de/mcp)
* [Checkpoints](/docs/de/checkpointing), [sandboxing](/docs/de/sandboxing) und [Workflows](/docs/de/workflows)
* [OpenTelemetry metrics](/docs/de/monitoring-usage) und die [verwaltete Einstellungsdatei](/docs/de/settings#settings-files)

Drei davon haben anbieter-spezifische Unterschiede:

* **MCP servers**: [Konnektoren von claude.ai](/docs/de/mcp#use-mcp-servers-from-claude-ai) werden nur geladen, wenn Ihr claude.ai-Abonnement die aktive Authentifizierungsmethode ist, und [Tool-Suche](/docs/de/mcp#configure-tool-search) ist standardmäßig auf Google Cloud's Agent Platform und deaktiviert, wenn `ANTHROPIC_BASE_URL` auf einen Nicht-First-Party-Host verweist
* **Subagents**: der integrierte [Explore Subagent](/docs/de/sub-agents#built-in-subagents) begrenzt sein vererbtes Modell auf Opus auf der Claude API und erbt das Modell der Hauptkonversation direkt auf jedem anderen Anbieter, einschließlich Claude Platform on AWS
* **[Commands](/docs/de/commands#all-commands)**: `/design-sync` und `/radio` sind auf Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry und Claude Platform on AWS nicht verfügbar, und `/voice` erfordert ein claude.ai-Konto

<h3 id="features-that-require-a-claude-subscription">
  Funktionen, die ein Claude-Abonnement erfordern
</h3>

Diese erfordern die Anmeldung mit einem claude.ai-Konto und sind nicht mit einem Anthropic Console API-Schlüssel oder von einem Drittanbieter erreichbar:

* [Claude Code im Web](/docs/de/claude-code-on-the-web), Claude Code auf Mobilgeräten und [Claude Code in Slack](/docs/de/slack)
* [Claude Code Desktop](/docs/de/desktop)
* [Routines](/docs/de/routines) (`/schedule`)
* [Ultraplan](/docs/de/ultraplan) und [Ultrareview](/docs/de/ultrareview)
* [Code Review](/docs/de/code-review): Team- und Enterprise-Pläne
* [Remote Control](/docs/de/remote-control)
* [Chrome-Erweiterung](/docs/de/chrome)
* [Computer use](/docs/de/computer-use): Pro- und Max-Pläne
* [Artifacts](/docs/de/artifacts): Pro-, Max-, Team- und Enterprise-Pläne
* [Voice dictation](/docs/de/voice-dictation)

Desktop ist die teilweise Ausnahme: [Gateway-Routing kann in der App oder von einem Administrator konfiguriert werden](/docs/de/llm-gateway-connect#desktop-app), Enterprise-Bereitstellungen können Desktop über [verwaltete Einstellungen](https://claude.com/docs/third-party/claude-desktop/configuration) an Google Cloud's Agent Platform oder einen Gateway-Anbieter weiterleiten, und [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview) führt die Code-Registerkarte auf Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry oder einem selbstgehosteten LLM-Gateway aus. Für die Verfügbarkeit dieser Funktionen pro Plan siehe [Verfügbarkeit nach Abonnementplan](#availability-by-subscription-plan).

<h3 id="cli-capabilities-that-vary-by-provider">
  CLI-Funktionen, die je nach Anbieter unterschiedlich sind
</h3>

Diese Funktionen funktionieren in der lokalen CLI, hängen aber von einer serverseitigen Funktion ab, die nicht jeder Anbieter verfügbar macht.

<table>
  <thead>
    <tr>
      <th>Funktion</th>
      <th>Claude-Abonnement</th>
      <th>Anthropic Console</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform on AWS</th>
      <th>Google Cloud's Agent Platform</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>[Web search](/docs/de/tools-reference#websearch-tool-behavior)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✓</td>
      <td>Siehe Hinweis <sup><a href="#fn1">1</a></sup></td>
      <td>✓</td>
    </tr>

    <tr>
      <td>[Fast mode](/docs/de/fast-mode)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Auto mode](/docs/de/auto-mode-config)</td>
      <td>✓</td>
      <td>✓</td>
      <td>Siehe Hinweis <sup><a href="#fn2">2</a></sup></td>
      <td>✓</td>
      <td>Siehe Hinweis <sup><a href="#fn2">2</a></sup></td>
      <td>Siehe Hinweis <sup><a href="#fn2">2</a></sup></td>
    </tr>

    <tr>
      <td>[Advisor](/docs/de/advisor)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Channels](/docs/de/channels)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[`/loop` scheduled tasks](/docs/de/scheduled-tasks)</td>
      <td>✓</td>
      <td>✓</td>
      <td>Siehe Hinweis <sup><a href="#fn3">3</a></sup></td>
      <td>Siehe Hinweis <sup><a href="#fn3">3</a></sup></td>
      <td>Siehe Hinweis <sup><a href="#fn3">3</a></sup></td>
      <td>Siehe Hinweis <sup><a href="#fn3">3</a></sup></td>
    </tr>

    <tr>
      <td>[GitHub Actions](/docs/de/github-actions) und [GitLab CI/CD](/docs/de/gitlab-ci-cd)</td>
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
  Admin und Analytik
</h3>

Kontrollen auf Organisationsebene und Sichtbarkeit der Nutzung.

<table>
  <thead>
    <tr>
      <th>Funktion</th>
      <th>Claude-Abonnement</th>
      <th>Anthropic Console</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform on AWS</th>
      <th>Google Cloud's Agent Platform</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>[Analytics dashboard and API](/docs/de/analytics)</td>
      <td>✓ (Dashboard: Team und Enterprise; API: Enterprise)</td>
      <td>✓ <sup><a href="#fn5">5</a></sup></td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Server-managed settings](/docs/de/server-managed-settings)</td>
      <td>✓ (Team und Enterprise)</td>
      <td>✓ (Team und Enterprise)</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Zero Data Retention](/docs/de/zero-data-retention)</td>
      <td>✓ (qualified Enterprise accounts)</td>
      <td>✓ (qualified accounts)</td>
      <td>Siehe Hinweis <sup><a href="#fn4">4</a></sup></td>
      <td>✓ (qualified accounts)</td>
      <td>Siehe Hinweis <sup><a href="#fn4">4</a></sup></td>
      <td>Siehe Hinweis <sup><a href="#fn4">4</a></sup></td>
    </tr>
  </tbody>
</table>

<span id="fn1" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>1</sup> Auf Google Cloud's Agent Platform ist Web search für Claude 4-Modelle und später verfügbar.<br />
<span id="fn2" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>2</sup> Auf diesen Anbietern unterstützt Auto mode nur Claude Sonnet 5, Opus 4.7 und Opus 4.8. Siehe [Auto mode configuration](/docs/de/auto-mode-config). In v2.1.158 bis v2.1.206 erforderte Auto mode auf diesen Anbietern auch das Setzen von `CLAUDE_CODE_ENABLE_AUTO_MODE=1`; v2.1.207 entfernte die Anforderung.<br />
<span id="fn3" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>3</sup> Explizite Intervalle wie `/loop every 2 hours` funktionieren auf jedem Anbieter. Auf Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform und Microsoft Foundry kann `/loop` sein eigenes Intervall nicht auswählen oder die Standard-Wartungsaufforderung bereitstellen, daher wird eine Aufforderung ohne Intervall alle 10 Minuten ausgeführt, und `/loop` ohne Argumente zeigt die Nutzungsmeldung. Siehe [Scheduled tasks](/docs/de/scheduled-tasks).<br />
<span id="fn4" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>4</sup> Unterliegt Ihrer Vereinbarung mit dem Cloud-Anbieter.<br />
<span id="fn5" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>5</sup> Dashboard und API nur. [Contribution metrics](/docs/de/analytics#enable-contribution-metrics) erfordert eine claude.ai Team- oder Enterprise-Organisation.

<Note>
  Wenn Sie sich über ein [LLM gateway](/docs/de/llm-gateway) authentifizieren, entspricht die Verfügbarkeit von Funktionen dem zugrunde liegenden Anbieter, an den das Gateway weiterleitet. Einige Anthropic-exklusive Funktionen wie der [Advisor](/docs/de/advisor) funktionieren nur, wenn das Gateway Anfragen intakt an die Anthropic API weiterleitet.
</Note>

<h3 id="summary-by-provider">
  Zusammenfassung nach Anbieter
</h3>

Jede Registerkarte listet auf, was auf diesem Anbieter nicht verfügbar oder teilweise unterstützt wird, mit Alternativen, wo vorhanden. Alles, was nicht aufgelistet ist, funktioniert genauso wie bei einem Claude-Abonnement, abgesehen von den [anbieter-spezifischen Unterschieden](#features-available-on-every-provider), die oben erwähnt werden. Auf Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry und Claude Platform on AWS sind Fehlerberichterstattung und Telemetrie an Anthropic standardmäßig deaktiviert. Siehe [Standardverhalten nach API-Anbieter](/docs/de/data-usage#default-behaviors-by-api-provider) für welcher Datenverkehr noch Anthropic erreicht und wie Sie sich abmelden.

<Tabs>
  <Tab title="Amazon Bedrock">
    **Nicht verfügbar:** alle [Funktionen, die ein Claude-Abonnement erfordern](#features-that-require-a-claude-subscription), plus [Web search](/docs/de/tools-reference#websearch-tool-behavior), [fast mode](/docs/de/fast-mode), [Advisor](/docs/de/advisor), [Channels](/docs/de/channels), das [analytics dashboard](/docs/de/analytics), [server-managed settings](/docs/de/server-managed-settings) und die [`/design-sync` und `/radio` commands](/docs/de/commands#all-commands).

    **Teilweise Unterstützung:**

    * [Desktop](/docs/de/desktop): nur über [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Auto mode](/docs/de/auto-mode-config): Sonnet 5, Opus 4.7 und Opus 4.8 nur
    * [`/loop`](/docs/de/scheduled-tasks): nur explizite Intervalle
    * [Zero Data Retention](/docs/de/zero-data-retention): unterliegt Ihrer AWS-Vereinbarung

    **Alternativen:** Verwenden Sie für die Planung [`/loop`](/docs/de/scheduled-tasks) mit einem expliziten Intervall statt `/schedule`. Für Cloud-Sitzungen verwenden Sie [GitHub Actions](/docs/de/github-actions) oder [GitLab CI/CD](/docs/de/gitlab-ci-cd). Für Web-Lookups verwenden Sie das [WebFetch tool](/docs/de/tools-reference#webfetch-tool-behavior) mit einer bestimmten URL.
  </Tab>

  <Tab title="Claude Platform on AWS">
    **Nicht verfügbar:** alle [Funktionen, die ein Claude-Abonnement erfordern](#features-that-require-a-claude-subscription), plus [fast mode](/docs/de/fast-mode), [Advisor](/docs/de/advisor), [Channels](/docs/de/channels), das [analytics dashboard](/docs/de/analytics), [server-managed settings](/docs/de/server-managed-settings) und die [`/design-sync` und `/radio` commands](/docs/de/commands#all-commands).

    **Verfügbar wo Amazon Bedrock nicht ist:** [Web search](/docs/de/tools-reference#websearch-tool-behavior).

    **Teilweise Unterstützung:**

    * [`/loop`](/docs/de/scheduled-tasks): nur explizite Intervalle

    **Alternativen:** Verwenden Sie für die Planung [`/loop`](/docs/de/scheduled-tasks) mit einem expliziten Intervall statt `/schedule`. Für Cloud-Sitzungen verwenden Sie [GitHub Actions](/docs/de/github-actions) oder [GitLab CI/CD](/docs/de/gitlab-ci-cd).
  </Tab>

  <Tab title="Google Cloud's Agent Platform">
    **Nicht verfügbar:** alle [Funktionen, die ein Claude-Abonnement erfordern](#features-that-require-a-claude-subscription), plus [fast mode](/docs/de/fast-mode), [Advisor](/docs/de/advisor), [Channels](/docs/de/channels), das [analytics dashboard](/docs/de/analytics), [server-managed settings](/docs/de/server-managed-settings) und die [`/design-sync` und `/radio` commands](/docs/de/commands#all-commands).

    **Teilweise Unterstützung:**

    * [Desktop](/docs/de/desktop): über [verwaltete Einstellungen](https://claude.com/docs/third-party/claude-desktop/configuration) oder [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Web search](/docs/de/tools-reference#websearch-tool-behavior): Claude 4-Modelle und später
    * [Auto mode](/docs/de/auto-mode-config): Sonnet 5, Opus 4.7 und Opus 4.8 nur
    * [`/loop`](/docs/de/scheduled-tasks): nur explizite Intervalle
    * [Zero Data Retention](/docs/de/zero-data-retention): unterliegt Ihrer Google Cloud-Vereinbarung

    **Alternativen:** Verwenden Sie für die Planung [`/loop`](/docs/de/scheduled-tasks) mit einem expliziten Intervall statt `/schedule`. Für Cloud-Sitzungen verwenden Sie [GitHub Actions](/docs/de/github-actions) oder [GitLab CI/CD](/docs/de/gitlab-ci-cd).
  </Tab>

  <Tab title="Microsoft Foundry">
    **Nicht verfügbar:** alle [Funktionen, die ein Claude-Abonnement erfordern](#features-that-require-a-claude-subscription), plus [fast mode](/docs/de/fast-mode), [Advisor](/docs/de/advisor), [Channels](/docs/de/channels), [GitHub Actions](/docs/de/github-actions) und [GitLab CI/CD](/docs/de/gitlab-ci-cd), das [analytics dashboard](/docs/de/analytics), [server-managed settings](/docs/de/server-managed-settings) und die [`/design-sync` und `/radio` commands](/docs/de/commands#all-commands).

    **Teilweise Unterstützung:**

    * [Desktop](/docs/de/desktop): nur über [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)
    * [Auto mode](/docs/de/auto-mode-config): Sonnet 5, Opus 4.7 und Opus 4.8 nur
    * [`/loop`](/docs/de/scheduled-tasks): nur explizite Intervalle
    * [Zero Data Retention](/docs/de/zero-data-retention): unterliegt Ihrer Azure-Vereinbarung

    **Alternativen:** Verwenden Sie für die Planung [`/loop`](/docs/de/scheduled-tasks) mit einem expliziten Intervall statt `/schedule`.
  </Tab>

  <Tab title="Anthropic Console">
    **Nicht verfügbar:** alle [Funktionen, die ein Claude-Abonnement erfordern](#features-that-require-a-claude-subscription).

    Alles in [CLI capabilities that vary by provider](#cli-capabilities-that-vary-by-provider) ist verfügbar, ebenso wie [server-managed settings](/docs/de/server-managed-settings), wenn der API-Schlüssel einer Team- oder Enterprise-Organisation gehört.
  </Tab>
</Tabs>

<h2 id="availability-by-subscription-plan">
  Verfügbarkeit nach Abonnementplan
</h2>

Wenn Sie sich über Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry oder einen Anthropic Console API-Schlüssel authentifizieren, gilt dieser Abschnitt nicht für Sie. Wenn Sie sich mit einem claude.ai-Konto anmelden, bestimmt Ihr Plan, welche der folgenden Funktionen verfügbar sind.

| Funktion                                                                    | Pro | Max | Team            | Enterprise                        |
| :-------------------------------------------------------------------------- | :-- | :-- | :-------------- | :-------------------------------- |
| [Claude Code on the web](/docs/de/claude-code-on-the-web)                        | ✓   | ✓   | ✓               | ✓ <sup><a href="#fn6">6</a></sup> |
| [Routines](/docs/de/routines)                                                    | ✓   | ✓   | ✓               | ✓                                 |
| [Remote Control](/docs/de/remote-control)                                        | ✓   | ✓   | Admin-aktiviert | Admin-aktiviert                   |
| [Channels](/docs/de/channels)                                                    | ✓   | ✓   | Admin-aktiviert | Admin-aktiviert                   |
| [Computer use](/docs/de/computer-use)                                            | ✓   | ✓   | ✗               | ✗                                 |
| Dispatch ([Desktop](/docs/de/desktop#sessions-from-dispatch))                    | ✓   | ✓   | ✗               | ✗                                 |
| [Code Review](/docs/de/code-review)                                              | ✗   | ✗   | ✓               | ✓                                 |
| [Artifacts](/docs/de/artifacts)                                                  | ✓   | ✓   | ✓               | Admin-aktiviert                   |
| [Analytics-Dashboard und Beitragskennzahlen](/docs/de/analytics)                 | ✗   | ✗   | ✓               | ✓                                 |
| [Enterprise Analytics API](/docs/de/analytics#access-data-programmatically)      | ✗   | ✗   | ✗               | ✓                                 |
| [Server-managed settings](/docs/de/server-managed-settings)                      | ✗   | ✗   | ✓               | ✓                                 |
| [SSO](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) | ✗   | ✗   | ✓               | ✓                                 |
| SCIM                                                                        | ✗   | ✗   | ✗               | ✓                                 |
| [Compliance API](https://platform.claude.com/docs/en/api/compliance)        | ✗   | ✗   | ✗               | ✓                                 |
| [Zero Data Retention](/docs/de/zero-data-retention)                              | ✗   | ✗   | ✗               | ✓ <sup><a href="#fn7">7</a></sup> |

<span id="fn6" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>6</sup> Auf Enterprise erfordert einen Premium-Sitz oder einen Chat + Claude Code-Sitz. Siehe [Claude Code on the web](/docs/de/claude-code-on-the-web).<br />
<span id="fn7" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>7</sup> Nicht im Standard-Enterprise-Plan enthalten. Erfordert separate Aktivierung durch Anthropic für qualifizierte Konten. Siehe [Zero Data Retention](/docs/de/zero-data-retention).

Für Preise und den vollständigen Planvergleich siehe [Team plans](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) und [Enterprise plans](https://support.claude.com/en/articles/9797531-what-is-the-enterprise-plan).

<h2 id="model-availability">
  Modellverfügbarkeit
</h2>

Für welche Claude-Modelle und Kontextfenstergrößen pro Anbieter und Region verfügbar sind, siehe [Model configuration](/docs/de/model-config) und die [Models overview](https://platform.claude.com/docs/en/about-claude/models/overview). Vision, PDF-Eingabe und erweitertes Denken sind Modellfunktionen und keine Claude Code-Funktionen und funktionieren auf jedem Anbieter, der das Modell anbietet. [Prompt caching](/docs/de/prompt-caching) funktioniert auf den meisten Anbietern gleich; auf Amazon Bedrock variiert die Unterstützung je nach Modell.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Enterprise deployment overview](/docs/de/third-party-integrations): Vergleichen Sie Authentifizierung, Abrechnung und Regionen über Anbieter hinweg
* Provider-Einrichtungsleitfäden: [Amazon Bedrock](/docs/de/amazon-bedrock), [Claude Platform on AWS](/docs/de/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai), [Microsoft Foundry](/docs/de/microsoft-foundry)
* [Platforms and integrations](/docs/de/platforms): wo Claude Code ausgeführt wird, einschließlich CLI, Desktop, IDE-Erweiterungen, Web, Mobilgeräte und CI/CD
