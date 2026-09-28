> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins-Übersicht

> Verstehen Sie, was ein Claude Code-Plugin ist, wann Sie eines statt einer eigenständigen Skill oder eines MCP-Servers benötigen, und welche Seite Sie lesen müssen, um eines zu installieren oder zu erstellen.

Ein Claude Code-Plugin ist ein Verzeichnis von Skills, Agents, Hooks, MCP-Servern oder anderen Komponenten, die Claude Code als eine Einheit installiert und lädt. Die meisten Plugins stammen aus einem Marketplace, der ein Katalog ist, der Plugins auflistet und anzeigt, wo man sie abruft. Sie können auch ein Plugin aus einem Ordner laden, den Ihnen jemand gibt, oder [Ihr eigenes erstellen](/docs/de/plugins/create).

<Note>
  Wenn Sie claude.ai Chat oder Cowork verwenden und nicht Claude Code, siehe [Plugins auf claude.ai und in Cowork](https://claude.com/docs/plugins/overview).
</Note>

Um jetzt ein Plugin auszuprobieren, führen Sie `/plugin` in einer Claude Code-Terminalsitzung aus und installieren Sie eines aus der Registerkarte **Discover**, die die Plugins aus Anthropics offiziellem Marketplace und jedem Marketplace auflistet, den Sie hinzugefügt haben. Von dort aus:

* [Plugins installieren und verwalten](/docs/de/plugins/install): die vollständigen Installationsschritte, Bereiche und andere Oberflächen
* [Ein Plugin erstellen](/docs/de/plugins/create): Erstellen Sie Ihr eigenes
* [Entscheiden Sie, ob Sie ein Plugin benötigen](#decide-whether-you-need-a-plugin): ob ein Plugin das richtige Werkzeug für das ist, was Sie möchten

<h2 id="understand-what-a-plugin-is">
  Verstehen Sie, was ein Plugin ist
</h2>

Ein Plugin ist ein Verzeichnis von Komponenten, normalerweise mit einem Manifest. Das Manifest, eine JSON-Datei unter `.claude-plugin/plugin.json`, gibt dem Plugin seinen Namen und kann eine Version, eine Beschreibung und andere [Metadaten](/docs/de/plugins/manifest-reference) hinzufügen. Die Komponenten sind das, was das Plugin zu Claude Code hinzufügt, wie zum Beispiel:

* [**Skills**](/docs/de/plugins/components#skills): `SKILL.md`-Anweisungen, die Claude lädt, wenn relevant, und die Sie auch als Befehl ausführen können
* [**Agents**](/docs/de/plugins/components#agents): Subagent-Definitionen, an die Claude delegieren kann
* [**Hooks**](/docs/de/plugins/components#hooks): Befehle, die Claude Code an Punkten in seinem Lebenszyklus ausführt, z. B. nach jeder Bearbeitung
* [**MCP-Server**](/docs/de/plugins/components#mcp-servers): Tool-Server, mit denen sich Claude Code verbindet, während das Plugin aktiviert ist

Dieses Diagramm zeigt ein Plugin namens `my-plugin`, das eine Komponente von jedem dieser Typen enthält, und was Sie von jeder Datei erhalten, sobald das Plugin geladen wird.

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugin-directory.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=f623b64e82713b830e48174f0a922888" className="dark:hidden" alt="Diagramm in zwei Spalten, verbunden durch fünf gerade Pfeile. Links das Verzeichnis eines Plugins namens my-plugin mit einem Manifest unter .claude-plugin/plugin.json, skills/review/SKILL.md, agents/reviewer.md, hooks/hooks.json, .mcp.json und anderen Komponenten. Rechts das, was jede Datei in Ihrer Sitzung gibt: Das Manifest setzt den Plugin-Namen, my-plugin; die Skill läuft als /my-plugin:review; die Agent-Datei ist ein Subagent, an den Claude delegieren kann; die Hooks-Datei enthält Hooks, die bei Lebenszyklusereignissen ausgeführt werden; und .mcp.json fügt einen MCP-Server hinzu, der Claude Tools gibt." width="760" height="336" data-path="images/plugin-directory.svg" />

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugin-directory-dark.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=17ee2bd45b63154fcc148ae1d1f736d8" className="hidden dark:block" alt="Diagramm in zwei Spalten, verbunden durch fünf gerade Pfeile. Links das Verzeichnis eines Plugins namens my-plugin mit einem Manifest unter .claude-plugin/plugin.json, skills/review/SKILL.md, agents/reviewer.md, hooks/hooks.json, .mcp.json und anderen Komponenten. Rechts das, was jede Datei in Ihrer Sitzung gibt: Das Manifest setzt den Plugin-Namen, my-plugin; die Skill läuft als /my-plugin:review; die Agent-Datei ist ein Subagent, an den Claude delegieren kann; die Hooks-Datei enthält Hooks, die bei Lebenszyklusereignissen ausgeführt werden; und .mcp.json fügt einen MCP-Server hinzu, der Claude Tools gibt." width="760" height="336" data-path="images/plugin-directory-dark.svg" />

Für jeden Komponententyp, den ein Plugin enthalten kann, mit einem Beispiel für jeden, siehe [Plugin-Komponenten](/docs/de/plugins/components). Um zu sehen, wo sich jedes Teil im Verzeichnis eines Plugins befindet, verwenden Sie den [Plugin-Explorer](/docs/de/plugins/components#explore-the-plugin-directory) auf dieser Seite.

<h3 id="decide-whether-you-need-a-plugin">
  Entscheiden Sie, ob Sie ein Plugin benötigen
</h3>

Skills, Subagents, Hooks und MCP-Server funktionieren alle allein, ohne ein Plugin. Eine Skill, die Sie in `~/.claude/skills/` speichern, ist beispielsweise in jedem Projekt auf Ihrem Computer verfügbar. Um eine allein einzurichten, siehe [Skills](/docs/de/skills), [Subagents](/docs/de/sub-agents), [Hooks](/docs/de/hooks-guide) oder [MCP](/docs/de/mcp).

Verwenden Sie ein Plugin, wenn Sie mehrere Skills, Subagents, Hooks oder MCP-Server als eine Einheit verpackt möchten. Installieren Sie eines, um eine Einrichtung zu erhalten, die jemand anderes erstellt hat, mit einem Befehl und Updates aus seinem Marketplace. Erstellen Sie eines, um Ihre eigene Einrichtung an Teamkollegen zu geben, es in vielen Projekten zu installieren oder versionierte Releases zu veröffentlichen.

<h3 id="what-an-enabled-plugin-adds-to-your-sessions">
  Was ein aktiviertes Plugin zu Ihren Sitzungen hinzufügt
</h3>

Ein aktiviertes Plugin ist Teil jeder Sitzung, nicht nur der Sitzungen, in denen Sie es verwenden. Das hat ein paar Konsequenzen, die es wert sind, vor der Installation zu wissen:

* **Kontext und Nutzung**: Für jede Skill, jeden Agent und jeden Befehl, den [Claude von selbst aufrufen kann](/docs/de/skills#control-who-invokes-a-skill), sind der Name und die Beschreibung in Claudes Kontext bei jedem Turn, damit Claude weiß, dass es existiert. Diese Token zählen zu Ihrer Nutzung und lassen weniger Platz im [Kontextfenster](/docs/de/context-window), auch in Sitzungen, in denen nichts aus dem Plugin läuft. Der vollständige Text einer Skill oder eines Agents wird nur geladen, wenn er verwendet wird. Was die MCP-Server des Plugins pro Turn hinzufügen, folgt [MCP-Tool-Suche](/docs/de/mcp#scale-with-mcp-tool-search).
* **Prozesse**: MCP-Server, die das Plugin definiert, laufen neben jeder Sitzung, in der es aktiviert ist, und seine Hooks werden bei ihren Ereignissen ausgelöst.
* **Berechtigungen**: Was das Plugin ausführt, führt es als Sie aus. Siehe [Plugin-Sicherheit und Vertrauen](/docs/de/plugins/security) für das, was Sie zuerst überprüfen sollten.

Sie können den Fußabdruck eines Plugins in jeder Phase überprüfen:

* **Vor der Installation**: Öffnen Sie das Plugin aus der Registerkarte **Marketplaces** in `/plugin`. Plugins im offiziellen Marketplace von Anthropic zeigen dort eine **Context cost**-Schätzung.
* **Nach der Installation**: [Messen Sie, was ein Plugin kostet](/docs/de/plugins/measure#measure-what-a-plugin-costs) zeigt, wie Sie den Fußabdruck eines Plugins lesen, und die Gruppe **Not used recently** der Registerkarte **Installed** listet Plugins auf, die Sie ausschalten könnten.
* **Um es zu stoppen, ohne es zu deinstallieren**: Deaktivieren Sie das Plugin mit `/plugin` oder in Ihrer Shell mit `claude plugin disable`. Siehe [Installierte Plugins verwalten](/docs/de/plugins/install#manage-installed-plugins).

<h2 id="get-plugins-from-a-marketplace">
  Holen Sie sich Plugins aus einem Marketplace
</h2>

Ein Marketplace ist ein Repository oder Verzeichnis mit einer `.claude-plugin/marketplace.json`-Datei, die Plugins auflistet und anzeigt, wo man sie abruft. Es ist ein Katalog, kein gehosteter Store. Sie fügen einen Marketplace einmal hinzu und installieren dann Plugins daraus nach Name, wie `commit-commands@claude-plugins-official`.

<Note>
  Ein Plugin-Marketplace ist nicht [Claude Marketplace](https://claude.com/marketplace). Claude Marketplace ist die Website unter claude.com/marketplace, auf der Sie Plugins, Konnektoren, Partnerprodukte und Service-Partner durchsuchen. Es ist kein Marketplace, den Sie mit `/plugin marketplace add` hinzufügen.
</Note>

Claude Code fügt Anthropics offiziellen Marketplace das erste Mal hinzu, wenn Sie eine interaktive Terminalsitzung starten, es sei denn, eine [verwaltete Richtlinie](/docs/de/plugins/org#allow-the-official-marketplace-and-your-own) blockiert es. Claude Code fügt von selbst keinen anderen Marketplace hinzu, einschließlich Anthropics Community- und Demo-Marketplaces. Um die drei Anthropic-Marketplaces zu unterscheiden, lesen Sie [Anthropics Marketplaces](/docs/de/plugins/anthropic-marketplaces). Um zu sehen, was der offizielle auflistet, öffnen Sie die Registerkarte **Discover** von `/plugin` in einer Sitzung oder durchsuchen Sie [Claude Marketplace](https://claude.com/marketplace/plugins).

Dieses Diagramm zeigt den Weg von einem Marketplace zu Ihrer Sitzung. Ein Marketplace listet ein Plugin auf, Sie installieren dieses Plugin, und Claude Code lädt seine Komponenten.

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugins-model.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=4196344954b7c2e27fc0bd6a9a1113a1" className="dark:hidden" alt="Diagramm des Marketplace-Pfads in drei Kästchen, von links nach rechts. Ein Marketplace, ein Katalog von Plugins, listet ein Plugin auf. Das Plugin ist ein Verzeichnis, das als Einheit installiert ist und Skills, Agents, Hooks, MCP-Server und andere Komponenten enthält. Sie installieren das Plugin in Claude Code, das seine Komponenten lädt." width="760" height="252" data-path="images/plugins-model.svg" />

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugins-model-dark.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=f6cdefe1fc05daf3b253d26e9f3f70f6" className="hidden dark:block" alt="Diagramm des Marketplace-Pfads in drei Kästchen, von links nach rechts. Ein Marketplace, ein Katalog von Plugins, listet ein Plugin auf. Das Plugin ist ein Verzeichnis, das als Einheit installiert ist und Skills, Agents, Hooks, MCP-Server und andere Komponenten enthält. Sie installieren das Plugin in Claude Code, das seine Komponenten lädt." width="760" height="252" data-path="images/plugins-model-dark.svg" />

[Plugins installieren und verwalten](/docs/de/plugins/install#install-a-plugin) hat die Installationsschritte für jeden Ort, an dem Sie Claude Code ausführen. Während Sie ein Plugin entwickeln, benötigen Sie keinen Marketplace: Laden Sie es direkt aus seinem Ordner mit `--plugin-dir`, wie [Entwickeln ohne einen Marketplace](/docs/de/plugins/create#develop-without-a-marketplace) zeigt.

<h3 id="make-an-installed-plugin-available-in-your-session">
  Machen Sie ein installiertes Plugin in Ihrer Sitzung verfügbar
</h3>

Bevor ein Plugin, das Sie installiert haben, Ihnen eine Skill gibt, die Sie ausführen können, muss es in jeder dieser Schichten vorhanden sein:

* **Einstellungen**: Ihre Einstellungen listen die Marketplaces auf, die Sie hinzugefügt haben, und die Plugins, die aktiviert sind.
* **Festplatte**: `~/.claude/plugins/` enthält das, was Claude Code abgerufen und installiert hat.
* **Sitzung**: Plugins werden beim Start geladen oder wenn Sie [Plugins neu laden](/docs/de/plugins/loading#check-which-stage-a-plugin-reached).

Lesen Sie [Plugin-Lade-Referenz](/docs/de/plugins/loading) für die Regeln in jeder Schicht, einschließlich welche Einstellungsdatei Vorrang hat und wo sich die Dateien auf der Festplatte befinden.

<h2 id="tell-anthropic’s-marketplaces-from-third-party-ones">
  Unterscheiden Sie Anthropics Marketplaces von Drittanbieter-Marketplaces
</h2>

Der Name eines Marketplace ordnet ihn in eine von drei Ebenen ein. Claude Code akzeptiert die offiziellen und Community-Namen nur für Marketplaces aus `github.com/anthropics/`-Repositories:

* **Offiziell**: Marketplaces mit einem von Anthropics [offiziellen Marketplace-Namen](/docs/de/plugins/security#official-marketplace-names), einschließlich `claude-plugins-official` und dem Demo-Marketplace `claude-code-plugins`.
* **Community**: Marketplaces mit einem von Anthropics Community-Namen, wie `claude-community`. [Identifizieren Sie Anthropics Marketplaces nach Name](/docs/de/plugins/security#marketplace-tiers) listet sie auf.
* **Drittanbieter**: Jeder andere Marketplace. Ein Marketplace, den Ihr Mitarbeiter oder Ihre Organisation veröffentlicht, ist Drittanbieter.

Unabhängig von der Ebene kann ein Plugin, das Sie installieren, Code mit Ihren Benutzerrechten ausführen. Lesen Sie [Plugin-Sicherheit und Vertrauen](/docs/de/plugins/security) für die Überprüfung eines Plugins vor der Installation.

Durch [verwaltete Einstellungen](/docs/de/settings#settings-files) kann eine Organisation Marketplaces auf eine Whitelist setzen oder blockieren, Plugins erzwingen und Session-only-Laden ausschalten. Lesen Sie [Verwalten Sie Plugins für Ihre Organisation](/docs/de/plugins/org) für diese Kontrollen.

<h2 id="understand-install-scopes">
  Verstehen Sie Installationsbereiche
</h2>

Wenn Sie ein Plugin installieren, wählen Sie einen Bereich, und der Bereich entscheidet, für wen das Plugin aktiviert ist:

* **Benutzerbereich**: aktiviert für Sie in jedem Projekt auf diesem Computer
* **Projektbereich**: aktiviert für alle, die in diesem Repository arbeiten, durch die committete `.claude/settings.json`. Jeder Mitarbeiter muss es immer noch [auf seiner eigenen Maschine installieren](/docs/de/plugins/loading#enabled-in-project-settings-but-not-installed)
* **Lokaler Bereich**: aktiviert für Sie nur in diesem Repository

Ein Plugin, das Sie im Terminal, in den lokalen Sitzungen der Desktop-App oder in der VS Code-Erweiterung im Benutzerbereich installieren, ist in den anderen beiden auf diesem Computer verfügbar, da alle drei die gleichen Einstellungsdateien lesen. Siehe [Wählen Sie einen Installationsbereich](/docs/de/plugins/install#choose-an-install-scope) für die Auswahl eines.

Eine Cloud-Sitzung, einschließlich einer im Browser unter claude.ai/code, lädt nicht die Plugins in Ihren lokalen Einstellungen. Für Installationsschritte im Terminal, VS Code und der Desktop-App sowie für das, was eine Cloud-Sitzung lädt, siehe [Ein Plugin installieren](/docs/de/plugins/install#install-a-plugin).

<Note>
  Das gleiche Plugin-Format wird auch auf claude.ai und in Cowork installiert, wo ein anderer Satz von Komponenten geladen wird. Für diese Oberflächen siehe [Plugins auf claude.ai und in Cowork](https://claude.com/docs/plugins/overview) auf claude.com.
</Note>

<h2 id="next-steps">
  Nächste Schritte
</h2>

Die meisten Menschen beginnen damit, ein Plugin aus Anthropics offiziellem Marketplace zu installieren, das Claude Code das erste Mal hinzufügt, wenn Sie eine interaktive Terminalsitzung starten. Führen Sie `/plugin` in einer Terminalsitzung aus, um ihn zu durchsuchen, oder folgen Sie [Plugins installieren und verwalten](/docs/de/plugins/install), das auch die Desktop-App und VS Code abdeckt. Um zu sehen, was sich in diesem Marketplace befindet, bevor Sie Claude Code öffnen, durchsuchen Sie [Claude Marketplace](https://claude.com/marketplace/plugins) im Web.

Um Ihr eigenes zu erstellen, [Erstellen Sie ein Plugin](/docs/de/plugins/create) beginnt mit einem leeren Verzeichnis und endet mit einem funktionierenden Plugin.

Sobald Sie ein Plugin installiert oder erstellt haben, behandeln diese Seiten das, was als Nächstes kommt:

* **Teilen Sie das, was Sie erstellt haben**: [Veröffentlichen und verteilen Sie ein Plugin](/docs/de/plugins/publish)
* **Überprüfen Sie, ob es funktioniert und verwendet wird**: [Testen Sie Plugins mit Evals](/docs/de/plugin-evals) und [Messen Sie Plugin-Kosten und -Nutzung](/docs/de/plugins/measure)
* **Führen Sie einen Marketplace für Ihr Team aus**: [Erstellen Sie einen Marketplace](/docs/de/plugins/create-marketplace), dann [Hosten und verwalten Sie einen Marketplace](/docs/de/plugins/host-marketplace)
* **Legen Sie Plugin-Richtlinie für eine Organisation fest**: [Verwalten Sie Plugins für Ihre Organisation](/docs/de/plugins/org)
* **Beheben Sie ein Problem**: [Beheben Sie Probleme mit Plugins](/docs/de/plugins/troubleshooting)
