> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Kosten und Nutzung von Plugins messen

> Messen Sie die Token-Kosten eines Claude Code-Plugins, finden Sie heraus, ob es noch verwendet wird, und wählen Sie die Telemetrie-Events für organisationsweite Plugin-Fragen aus.

Jede Sitzung, in der ein Plugin aktiviert ist, enthält die Namen und Beschreibungen seiner Skills, Agents und Commands im Kontext von Claude, und diese Token werden gegen die Nutzung des Benutzers angerechnet, unabhängig davon, ob das Plugin verwendet wird oder nicht. Diese Seite zeigt, wie Sie diese Zahl für ein Plugin sehen, wie Sie sie reduzieren, wenn Sie das Plugin verwalten, und wo die Nutzung angezeigt wird, damit Sie feststellen können, ob ein Plugin noch verwendet wird.

Diese Seite ist für Plugin-Autoren und Verwalter. Wenn Sie Claude Code für eine Organisation verwalten, behandelt [Messung über eine Flotte](#measure-across-a-fleet) die gleichen Fragen auf jedem Computer.

<Note>
  Diese Fälle werden auf anderen Seiten behandelt:

  * **Testen, wie zuverlässig das Plugin Claudes Verhalten ändert**: siehe [Plugins mit Evals testen](/docs/de/plugin-evals)
  * **Trimmen des Kontexts Ihrer eigenen Sitzung**: siehe [Installierte Plugins verwalten](/docs/de/plugins/install#manage-installed-plugins) und die Seite [Kontextfenster](/docs/de/context-window)
</Note>

Beginnen Sie mit [Messen Sie, was ein Plugin kostet](#measure-what-a-plugin-costs).

<h2 id="measure-what-a-plugin-costs">
  Messen Sie, was ein Plugin kostet
</h2>

Um zu sehen, was ein Plugin zu Claudes Kontext hinzufügt, führen Sie [`claude plugin details`](/docs/de/plugins/cli-reference#plugin-details) mit dem Namen des Plugins aus. Sie führen es in Ihrer Shell aus, nicht an der Eingabeaufforderung einer laufenden Claude Code-Sitzung. Das Plugin muss geladen sein: installiert, in einem Skills-Verzeichnis oder mit `--plugin-dir` im gleichen Befehl übergeben, wie in `claude --plugin-dir ./formatter plugin details formatter`.

Dieses Beispiel liest ein installiertes Plugin namens `formatter`, das zwei Skills, einen Command, einen Agent, einen Hook und einen MCP-Server hat:

```bash theme={null}
claude plugin details formatter
```

```text theme={null}
formatter 1.0.0
  Description: Formats and lints code on save
  Source: formatter@my-marketplace

Component inventory
  Skills (3)  format-all, format-code, lint-fix
  Agents (1)  style-reviewer
  Hooks (1)  PostToolUse  (harness-only — no model context cost)
  MCP servers (1)  formatter-tools  (tool schemas resolved at runtime; not counted)
  LSP servers (0)

Projected token cost
  Always-on:   ~146 tok   added to every session

Per-component (rounded)
  component       always-on  on-invoke
  format-code           ~40        ~30
  lint-fix              ~50        ~30
  style-reviewer        ~40        ~40
  format-all           < 20        ~30

  On-invoke cost is paid each time a skill or agent fires.
  Token counts are estimates and may differ from actual usage.
```

Jeder Teil der Ausgabe beantwortet eine andere Frage:

* **Component inventory**: was Claude Code im Plugin gefunden hat. Commands werden mit Skills gezählt, daher erscheint `format-all` unter `Skills`. Hooks und MCP-Server erhalten keine Kostenschätzung und keine Pro-Komponenten-Zeile; um zu sehen, was die MCP-Tools eines Plugins hinzufügen, führen Sie `/context` in einer Sitzung mit aktiviertem Plugin aus und lesen Sie die Kategorie `MCP tools`.
* **Always-on**: die Token, die die Namen und Beschreibungen der Skills, Agents und Commands des Plugins zu jeder Sitzung hinzufügen, in der das Plugin aktiviert ist, unabhängig davon, ob etwas ausgeführt wird. Dies ist die Zahl, die jeder Benutzer trägt, und die zu reduzierende.
* **Per-component**: jede Zeile teilt einen Skill, Agent oder Command in seinen Always-on-Anteil und seine On-invoke-Kosten auf, die nur geladen werden, wenn diese Komponente ausgeführt wird. Verwenden Sie die Always-on-Spalte, um zu finden, welche Komponente am meisten beiträgt.

<h3 id="lower-the-always-on-figure">
  Reduzieren Sie die Always-on-Zahl
</h3>

Wenn Sie das Plugin verwalten, reduzieren diese Änderungen, was es zu jeder Sitzung hinzufügt. Wenn Sie es nur verwenden, sind Ihre Optionen, es zu deaktivieren oder zu deinstallieren; siehe [Installierte Plugins verwalten](/docs/de/plugins/install#manage-installed-plugins).

Die Always-on-Zahl zählt den Namen jeder Komponente plus ihre `description` und `when_to_use` Frontmatter. Um sie zu reduzieren:

* Verkürzen Sie Skill- und Agent-Beschreibungen.
* Teilen Sie ein großes Plugin auf, damit Benutzer nur die Komponenten installieren, die sie benötigen.

Die Beschreibung eines Skills ist auch das, wogegen Claude eine Anfrage abgleicht, daher kann eine kürzere verhindern, dass der Skill ausgelöst wird. Nachdem Sie Beschreibungen gekürzt haben, überprüfen Sie das Auslösen mit einem [`tool_used: Skill` Grader](/docs/de/plugin-evals#create-your-first-eval-suite) in Ihrer Eval-Suite.

Für das, was jeder Komponententyp beiträgt, siehe [Plugin-Komponenten](/docs/de/plugins/components).

<h3 id="cost-shown-to-users-before-install">
  Kosten, die Benutzern vor der Installation angezeigt werden
</h3>

Plugins im offiziellen Marketplace zeigen ihre Kosten Benutzern vor der Installation an. In `/plugin`, wenn ein Benutzer die Plugin-Liste eines Marketplace durchsucht und ein Plugin auswählt, zeigt der Detailbereich einen Abschnitt **Context cost** mit einer Zeile `Every turn:` und einer Zeile `When invoked:`. Wenn die Always-on-Zahl 2.000 Token oder mehr beträgt, wird die Zeile `Every turn:` hervorgehoben.

Ein Plugin in Ihrem eigenen Marketplace hat keinen Abschnitt **Context cost**.

<h2 id="check-whether-a-plugin-is-used">
  Überprüfen Sie, ob ein Plugin verwendet wird
</h2>

Claude Code meldet die Nutzung eines Plugins nicht an seinen Autor zurück. Die Nutzung wird auf dem Computer jeder Person aufgezeichnet, die das Plugin installiert hat, daher hängt das, was Sie erfahren können, von Ihrer Beziehung zu diesen Personen ab:

* **Sie verwalten Claude Code für ihre Organisation**: die OpenTelemetry-Events und die Analytics API zählen Installationen und Skill-Aktivierungen auf jedem Computer. Siehe [Messung über eine Flotte](#measure-across-a-fleet).
* **Sie sind Teamkollegen, die Sie fragen können**: Claudes eigenes Claude Code zeigt jedem Benutzer, ob er das Plugin noch verwendet, an vier Stellen: das [`/plugin` Panel](#not-used-recently-in-/plugin), [`/skill-doctor`](#find-skills-that-never-run), [`/doctor`](#unused-plugins-in-/doctor) und [`/usage`](#usage-share-in-/usage). Alle vier sind Commands, die der Benutzer an der Claude Code-Eingabeaufforderung in einer Sitzung auf seinem eigenen Computer ausführt.
* **Keines von beiden**: Sie haben kein Nutzungssignal von Claude Code für dieses Plugin.

<h3 id="not-used-recently-in-/plugin">
  Nicht kürzlich verwendet in `/plugin`
</h3>

Auf der Registerkarte **Installed** von `/plugin` wird ein Plugin, das der Benutzer aus einem Marketplace installiert hat, unter einer Kopfzeile **Not used recently** verschoben, sobald es mindestens 14 Tage und 10 Sitzungen lang nicht verwendet wurde. Die Details des Plugins zeigen auch eine Zeile `Last used:`. Für das, was Benutzer mit dieser Kopfzeile und Zeile tun, siehe [Finden Sie Plugins, die Sie nicht mehr verwenden](/docs/de/plugins/install#find-plugins-you-no-longer-use).

Die Kopfzeile **Not used recently** wird niemals angezeigt für:

* Plugins, die mit `--plugin-dir` oder aus einem Skills-Verzeichnis geladen werden
* Plugins, die durch verwaltete Einstellungen aktiviert werden, oder aus einem [Seed-Verzeichnis](/docs/de/plugins/org#seed-containers-and-ci) bereitgestellt werden
* Plugins, die ein Theme, einen Output-Stil, einen Monitor oder einen Workflow enthalten, da diese ohne eine verfolgbare Invokation verwendet werden

Der [Language Server](/docs/de/plugins/components#lsp-servers) eines Plugins wird als verwendet gezählt, wenn er Diagnosen liefert oder eine Code-Navigationsanfrage beantwortet, daher wird ein LSP-Plugin, dessen Server in Ihren Sitzungen aktiv ist, nicht als ungenutzt aufgelistet.

Wenn die Organisation des Benutzers [`strictKnownMarketplaces`](/docs/de/plugins/org#restrict-what-users-can-install) setzt, werden weder die Kopfzeile noch die Zeile `Last used:` angezeigt.

<h3 id="find-skills-that-never-run">
  Finden Sie Skills, die nie ausgeführt werden
</h3>

Führen Sie `/skill-doctor` aus, um zu sehen, was jeder Ihrer Skills kostet und wie oft er verwendet wird. Es kennzeichnet Skills, die in Claudes Skill-Listing vorhanden sind, aber nie aufgerufen wurden, einschließlich Skills aus Plugins.

In einer interaktiven Sitzung wird der Bericht auf der Registerkarte **Stats** des `/plugin`-Managers geöffnet. Siehe [Finden Sie ungenutzte Skills](/docs/de/skills#find-unused-skills) für das, was der Bericht abdeckt und wo er verfügbar ist.

<h3 id="unused-plugins-in-/doctor">
  Ungenutzte Plugins in `/doctor`
</h3>

Die `/doctor`-Überprüfung listet jeden benutzerinstallierten Skill, MCP-Server und Plugin auf und empfiehlt, die nicht verwendeten zu deaktivieren. Siehe [`/doctor` in der Befehlsreferenz](/docs/de/commands#all-commands).

<h3 id="usage-share-in-/usage">
  Nutzungsanteil in `/usage`
</h3>

In einem Pro-, Max-, Team- oder Enterprise-Plan teilt die `/usage`-Aufschlüsselung die aktuelle Nutzung als Anteil des Gesamtbetrags auf Skills, Subagents, Plugins und MCP-Server auf. Siehe [Verwenden des `/usage`-Befehls](/docs/de/costs#using-the-/usage-command).

<h2 id="measure-across-a-fleet">
  Messung über eine Flotte
</h2>

Wenn Sie Claude Code für eine Organisation verwalten, können Sie Plugin-Kosten und -Nutzung auf jedem Computer aus einer dieser Quellen messen:

* **OpenTelemetry-Events**: Claude Code exportiert diese an Ihr eigenes Backend, sobald Sie [einen Exporter konfigurieren](/docs/de/monitoring-usage). Siehe [OpenTelemetry-Events für Plugin-Installationen und -Nutzung](#pick-the-opentelemetry-event-for-each-question).
* **Analytics API**: von Anthropics Aufzeichnungen bereitgestellt, ohne dass ein Exporter erforderlich ist. Siehe [Abfragen der Analytics API](#query-the-analytics-api).

<h3 id="pick-the-opentelemetry-event-for-each-question">
  OpenTelemetry-Events für Plugin-Installationen und -Nutzung
</h3>

Diese OpenTelemetry-Events und Attribute beantworten jede Plugin-Frage von Ihrem Backend:

| Frage                                                         | OpenTelemetry-Event oder Attribut                                                                                                                                   |
| :------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Welche Plugins werden installiert und von wo                  | [`claude_code.plugin_installed`](/docs/de/monitoring-usage#plugin-installed-event), eine pro Installation                                                                |
| Welche Plugins sind in wie vielen Sitzungen aktiv             | [`claude_code.plugin_loaded`](/docs/de/monitoring-usage#plugin-loaded-event), eine pro aktiviertem Plugin beim Sitzungsstart                                             |
| Welche Skills werden aktiviert und welches Plugin besitzt sie | [`claude_code.skill_activated`](/docs/de/monitoring-usage#skill-activated-event), mit `plugin.name` und `marketplace.name` für Plugin-Skills                             |
| Was die Hooks eines Plugins melden                            | [`claude_code.hook_plugin_metrics`](/docs/de/monitoring-usage#hook-plugin-metrics-event), nur für Hooks in offiziellen Marketplace-Plugins ausgegeben                    |
| Was ein Plugin in API-Ausgaben kostet                         | `plugin.name` und `marketplace.name` auf dem [Cost Counter](/docs/de/monitoring-usage#cost-counter), gesetzt, wenn der aktive Skill oder Subagent zu einem Plugin gehört |

<h3 id="redacted-plugin-names-in-your-backend">
  Redigierte Plugin-Namen in Ihrem Backend
</h3>

Plugins aus dem offiziellen Marketplace melden ihren Plugin-Namen und Marketplace-Namen an Ihr Backend wörtlich. Der Name jedes anderen Plugins wird standardmäßig redigiert oder weggelassen, einschließlich eines Plugins aus dem eigenen Marketplace Ihrer Organisation. Der [Trust Tier](/docs/de/plugins/security#find-plugins-in-telemetry) des Plugins entscheidet, welcher.

Um echte Namen bei einigen Events zu erhalten, setzen Sie die Umgebungsvariable [`OTEL_LOG_TOOL_DETAILS`](/docs/de/monitoring-usage#common-configuration-variables) auf `1` auf den Computern, die Telemetrie exportieren, zum Beispiel im `env`-Block der gleichen [verwalteten Einstellungen](/docs/de/monitoring-usage#administrator-configuration), die den Exporter konfigurieren:

| Event                                 | Standard                                                                                                | Mit `OTEL_LOG_TOOL_DETAILS=1`                                |
| :------------------------------------ | :------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------- |
| `plugin_loaded`                       | `plugin.name` und `marketplace.name` sind die Literalzeichenfolge `third-party`                         | Echte Namen                                                  |
| `plugin_installed`, `skill_activated` | `plugin.name` und `marketplace.name` weggelassen; bei `skill_activated` ist `skill.name` `custom_skill` | Echte Namen                                                  |
| Cost Counter                          | `plugin.name` ist `third-party`; `marketplace.name` abwesend                                            | Echter `plugin.name`; `marketplace.name` immer noch abwesend |

Bei `plugin_loaded` identifiziert `plugin_id_hash` standardmäßig immer noch jedes Plugin, daher können Sie unterschiedliche Drittanbieter-Plugins zählen.

<h3 id="query-the-analytics-api">
  Abfragen der Analytics API
</h3>

Im Enterprise-Plan beantwortet die Analytics API „welche Plugins installiert und ruft meine Organisation auf" aus Anthropics Aufzeichnungen auf, ohne dass ein Exporter erforderlich ist. [`GET /v1/organizations/analytics/plugins`](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) gibt pro Plugin, pro Tag Installations- und Invokationszählungen über Claude Code und Cowork zurück, die Sie nach Benutzer, RBAC-Gruppe oder Produkt gruppieren können.

Plugin-Aktivität, die Anthropic ohne Plugin-Namen erreicht, wird in einer aggregierten Zeile `third-party` angezeigt. [Finden Sie Plugins in der Telemetrie](/docs/de/plugins/security#find-plugins-in-telemetry) sagt, welche Plugins Claude Code nach Namen meldet.

Authentifizieren Sie die Anfrage mit einem API-Schlüssel, der den Bereich `read:analytics` hat, den ein Primary Owner wie unter [Zugriff auf Daten programmgesteuert](/docs/de/analytics#access-data-programmatically) beschrieben erstellt.

Siehe die [Endpoint-Referenz](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) für die Parameter und Antwortfelder.

<h2 id="next-steps">
  Nächste Schritte
</h2>

* [Plugins mit Evals testen](/docs/de/plugin-evals): Messen Sie, wie zuverlässig das Plugin Claude lenkt, nicht nur was es kostet
* [Reduzieren Sie die Always-on-Zahl](#lower-the-always-on-figure): Was Sie im Plugin ändern müssen, um seine Pro-Turn-Kosten zu reduzieren
* [Plugin-Sicherheit und Vertrauen](/docs/de/plugins/security#find-plugins-in-telemetry): Welche Telemetrie-Felder Plugin-Namen tragen und wann sie redigiert werden
* [Überwachung der Nutzung](/docs/de/monitoring-usage): Die vollständige OpenTelemetry-Event-Referenz
