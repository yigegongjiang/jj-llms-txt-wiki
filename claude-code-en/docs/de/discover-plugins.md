> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Entdecken und installieren Sie vorgefertigte Plugins über Marktplätze

> Finden und installieren Sie Plugins aus Marktplätzen, um Claude Code mit neuen Befähigungen, Agenten und Funktionen zu erweitern.

Plugins erweitern Claude Code mit Befähigungen, Agenten, Hooks und MCP-Servern. Plugin-Marktplätze sind Kataloge, die Ihnen helfen, diese Erweiterungen zu entdecken und zu installieren, ohne sie selbst zu erstellen.

Möchten Sie Ihren eigenen Marktplatz erstellen und verteilen? Siehe [Erstellen und verteilen Sie einen Plugin-Marktplatz](/docs/de/plugin-marketplaces).

<h2 id="how-marketplaces-work">
  Wie Marktplätze funktionieren
</h2>

Ein Marktplatz ist ein Katalog von Plugins, die jemand anderes erstellt und geteilt hat. Die Verwendung eines Marktplatzes ist ein zweistufiger Prozess:

<Steps>
  <Step title="Fügen Sie den Marktplatz hinzu">
    Dies registriert den Katalog bei Claude Code, damit Sie durchsuchen können, was verfügbar ist. Es werden noch keine Plugins installiert.
  </Step>

  <Step title="Installieren Sie einzelne Plugins">
    Durchsuchen Sie den Katalog und installieren Sie die Plugins, die Sie möchten.
  </Step>
</Steps>

Stellen Sie sich das vor wie das Hinzufügen eines App-Stores: Das Hinzufügen des Stores gibt Ihnen Zugriff zum Durchsuchen seiner Sammlung, aber Sie wählen immer noch aus, welche Apps Sie einzeln herunterladen möchten.

<h2 id="official-anthropic-marketplace">
  Offizieller Anthropic-Marktplatz
</h2>

Der offizielle Anthropic-Marktplatz (`claude-plugins-official`) ist automatisch verfügbar, wenn Sie Claude Code starten. Führen Sie `/plugin` aus und gehen Sie zur Registerkarte **Discover**, um zu sehen, was verfügbar ist, oder sehen Sie sich den Katalog unter [claude.com/plugins](https://claude.com/plugins) an.

Um ein Plugin aus dem offiziellen Marktplatz zu installieren, verwenden Sie `/plugin install <name>@claude-plugins-official`. Um beispielsweise die GitHub-Integration zu installieren:

```shell theme={null}
/plugin install github@claude-plugins-official
```

Wenn Claude Code meldet, dass das Plugin in keinem Marktplatz gefunden wird, fehlt Ihnen entweder der Marktplatz oder er ist veraltet. Führen Sie `/plugin marketplace update claude-plugins-official` aus, um ihn zu aktualisieren, oder `/plugin marketplace add anthropics/claude-plugins-official`, wenn Sie ihn noch nicht hinzugefügt haben. Versuchen Sie dann die Installation erneut.

<Note>
  Der offizielle Marktplatz wird von Anthropic kuratiert, und die Aufnahme liegt im Ermessen von Anthropic. Die In-App-Einreichungsformulare fügen Plugins zum [Community-Marktplatz](#community-marketplace) hinzu, nicht zum offiziellen. Um Plugins unabhängig zu verteilen, [erstellen Sie Ihren eigenen Marktplatz](/docs/de/plugin-marketplaces) und teilen Sie ihn mit Benutzern.
</Note>

Der offizielle Marktplatz umfasst mehrere Kategorien von Plugins:

<h3 id="code-intelligence">
  Code-Intelligenz
</h3>

Code-Intelligenz-Plugins aktivieren das integrierte LSP-Tool von Claude Code und geben Claude die Möglichkeit, zu Definitionen zu springen, Referenzen zu finden und Typfehler unmittelbar nach Änderungen zu sehen. Diese Plugins konfigurieren [Language Server Protocol](https://microsoft.github.io/language-server-protocol/)-Verbindungen, die gleiche Technologie, die die Code-Intelligenz von VS Code antreibt.

Diese Plugins erfordern, dass die Language-Server-Binärdatei auf Ihrem System installiert ist. Wenn Sie bereits einen Language Server installiert haben, kann Claude Sie möglicherweise auffordern, das entsprechende Plugin zu installieren, wenn Sie ein Projekt öffnen.

| Sprache    | Plugin              | Erforderliche Binärdatei     |
| :--------- | :------------------ | :--------------------------- |
| C/C++      | `clangd-lsp`        | `clangd`                     |
| C#         | `csharp-lsp`        | `csharp-ls`                  |
| Go         | `gopls-lsp`         | `gopls`                      |
| Java       | `jdtls-lsp`         | `jdtls`                      |
| Kotlin     | `kotlin-lsp`        | `kotlin-language-server`     |
| Lua        | `lua-lsp`           | `lua-language-server`        |
| PHP        | `php-lsp`           | `intelephense`               |
| Python     | `pyright-lsp`       | `pyright-langserver`         |
| Rust       | `rust-analyzer-lsp` | `rust-analyzer`              |
| Swift      | `swift-lsp`         | `sourcekit-lsp`              |
| TypeScript | `typescript-lsp`    | `typescript-language-server` |

Sie können auch [Ihr eigenes LSP-Plugin erstellen](/docs/de/plugins-reference#lsp-servers) für andere Sprachen.

<Note>
  Wenn Sie nach der Installation eines Plugins `Executable not found in $PATH` in der Registerkarte `/plugin` Errors sehen, installieren Sie die erforderliche Binärdatei aus der obigen Tabelle.
</Note>

<h4 id="what-claude-gains-from-code-intelligence-plugins">
  Was Claude von Code-Intelligenz-Plugins gewinnt
</h4>

Sobald ein Code-Intelligenz-Plugin installiert ist und seine Language-Server-Binärdatei verfügbar ist, gewinnt Claude zwei Funktionen:

* **Automatische Diagnose**: Nach jeder Dateiänderung, die Claude vornimmt, analysiert der Language Server die Änderungen und meldet Fehler und Warnungen automatisch zurück. Claude sieht Typfehler, fehlende Importe und Syntaxprobleme, ohne einen Compiler oder Linter ausführen zu müssen. Wenn Claude einen Fehler einführt, bemerkt es das Problem und behebt es in derselben Runde. Dies erfordert keine Konfiguration über die Installation des Plugins hinaus. Sie können Diagnosen inline anzeigen, indem Sie **Strg+O** drücken, wenn der Indikator „Diagnosen gefunden" angezeigt wird.
* **Code-Navigation**: Claude kann den Language Server verwenden, um zu Definitionen zu springen, Referenzen zu finden, Typinformationen beim Hover zu erhalten, Symbole aufzulisten, Implementierungen zu finden und Call-Hierarchien zu verfolgen. Diese Operationen geben Claude eine präzisere Navigation als grep-basierte Suche, obwohl die Verfügbarkeit je nach Sprache und Umgebung variieren kann.

Wenn Sie auf Probleme stoßen, siehe [Code-Intelligenz-Fehlerbehebung](#code-intelligence-issues).

<h3 id="external-integrations">
  Externe Integrationen
</h3>

Diese Plugins bündeln vorkonfigurierte [MCP-Server](/docs/de/mcp), damit Sie Claude mit externen Diensten verbinden können, ohne manuelle Einrichtung:

* **Quellcodeverwaltung**: `github`, `gitlab`
* **Projektmanagement**: `atlassian` (Jira/Confluence), `asana`, `linear`, `notion`
* **Design**: `figma`
* **Infrastruktur**: `vercel`, `firebase`, `supabase`
* **Kommunikation**: `slack`
* **Überwachung**: `sentry`

<h3 id="automatic-security-review">
  Automatische Sicherheitsüberprüfung
</h3>

Das Plugin `security-guidance` überprüft jede Änderung, die Claude vornimmt, auf häufige Sicherheitslücken und weist Claude an, das Gefundene in derselben Sitzung zu beheben. Siehe [Sicherheitsprobleme erfassen, während Claude Code schreibt](/docs/de/security-guidance), um zu sehen, was es überprüft und wie Sie projektspezifische Regeln hinzufügen.

<h3 id="development-workflows">
  Entwicklungs-Workflows
</h3>

Plugins, die Befähigungen und Agenten für häufige Entwicklungsaufgaben hinzufügen:

* **commit-commands**: Git-Commit-Workflows einschließlich Commit, Push und PR-Erstellung
* **pr-review-toolkit**: Spezialisierte Agenten für die Überprüfung von Pull Requests
* **agent-sdk-dev**: Tools zum Erstellen mit dem Claude Agent SDK
* **plugin-dev**: Toolkit zum Erstellen Ihrer eigenen Plugins

<h3 id="output-styles">
  Ausgabestile
</h3>

Passen Sie an, wie Claude antwortet:

* **explanatory-output-style**: Pädagogische Einblicke in Implementierungsentscheidungen
* **learning-output-style**: Interaktiver Lernmodus zum Aufbau von Befähigungen

<h2 id="community-marketplace">
  Community-Marktplatz
</h2>

Der Community-Marktplatz unter [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community) hostet Plugins von Drittanbietern, die Anthropics automatisierte Validierung und Sicherheitsprüfung bestanden haben. Jedes Plugin ist an einen bestimmten Commit-SHA im Katalog gebunden. Im Gegensatz zum offiziellen Marktplatz fügen Sie ihn manuell hinzu:

```shell theme={null}
/plugin marketplace add anthropics/claude-plugins-community
```

Installieren Sie dann Plugins von ihm mit dem Marktplatznamen `claude-community`:

```shell theme={null}
/plugin install <plugin-name>@claude-community
```

Um Ihr eigenes Plugin zum Community-Marktplatz einzureichen, siehe [Reichen Sie Ihr Plugin beim Community-Marktplatz ein](/docs/de/plugins#submit-your-plugin-to-the-community-marketplace) im Leitfaden zum Erstellen von Plugins.

<h2 id="try-it-add-the-demo-marketplace">
  Probieren Sie es aus: Fügen Sie den Demo-Marktplatz hinzu
</h2>

Anthropic verwaltet auch einen [Demo-Plugins-Marktplatz](https://github.com/anthropics/claude-code/tree/main/plugins) (`claude-code-plugins`) mit Beispiel-Plugins, die zeigen, was mit dem Plugin-System möglich ist. Im Gegensatz zum offiziellen Marktplatz müssen Sie diesen manuell hinzufügen.

<Steps>
  <Step title="Fügen Sie den Marktplatz hinzu">
    Führen Sie in Claude Code den Befehl `plugin marketplace add` für den Marktplatz `anthropics/claude-code` aus:

    ```shell theme={null}
    /plugin marketplace add anthropics/claude-code
    ```

    Dies lädt den Marktplatz-Katalog herunter und macht seine Plugins für Sie verfügbar.
  </Step>

  <Step title="Durchsuchen Sie verfügbare Plugins">
    Führen Sie `/plugin` aus, um den Plugin-Manager zu öffnen. Dies öffnet eine Schnittstelle mit Registerkarten mit vier Registerkarten, die Sie mit **Tab** durchlaufen können (oder **Shift+Tab**, um rückwärts zu gehen):

    * **Discover**: Durchsuchen Sie verfügbare Plugins aus allen Ihren Marktplätzen
    * **Installed**: Zeigen Sie Ihre installierten Plugins an und verwalten Sie sie
    * **Marketplaces**: Fügen Sie Marktplätze hinzu, entfernen Sie sie oder aktualisieren Sie sie
    * **Errors**: Zeigen Sie alle Plugin-Ladefehler an

    Gehen Sie zur Registerkarte **Discover**, um Plugins aus dem Marktplatz zu sehen, den Sie gerade hinzugefügt haben. Wenn Ihr Administrator den Marktplatz über die verwaltete Einstellung [`pluginSuggestionMarketplaces`](/docs/de/settings#available-settings) auf die Whitelist gesetzt hat, werden Plugins, die für Ihr aktuelles Arbeitsverzeichnis relevant sind, oben mit einer Bezeichnung **suggested for this directory** angeheftet.
  </Step>

  <Step title="Installieren Sie ein Plugin">
    Wählen Sie ein Plugin aus, um seine Details anzuzeigen. Der Detailbereich zeigt, was das Plugin enthält und was es kostet:

    * Eine **Context cost**-Schätzung, damit Sie sehen können, wie viele Token das Plugin zu Ihrem [Kontextfenster](/docs/de/features-overview#understand-context-costs) bei jedem Durchgang hinzufügt (Claude Code v2.1.143 und später)
    * Das **Last updated**-Datum des Plugins (v2.1.144 und später)
    * Ein **Will install**-Bereich, der die Befehle, Agenten, Skills, Hooks und MCP- und LSP-Server des Plugins auflistet, damit Sie genau überprüfen können, was es hinzufügt, bevor Sie es installieren (v2.1.145 und später)

    Wählen Sie einen Installationsbereich:

    * **User scope**: Installieren Sie für sich selbst in allen Projekten
    * **Project scope**: Installieren Sie für alle Mitarbeiter in diesem Repository
    * **Local scope**: Installieren Sie für sich selbst nur in diesem Repository

    Wählen Sie beispielsweise **commit-commands** (ein Plugin, das Git-Workflow-Skills hinzufügt) und installieren Sie es in Ihrem Benutzerbereich.

    Sie können auch direkt über die Befehlszeile installieren:

    ```shell theme={null}
    /plugin install commit-commands@claude-code-plugins
    ```

    Siehe [Konfigurationsbereiche](/docs/de/settings#configuration-scopes), um mehr über Bereiche zu erfahren.
  </Step>

  <Step title="Verwenden Sie Ihr neues Plugin">
    Nach der Installation führen Sie `/reload-plugins` aus, um das Plugin zu aktivieren. Plugin-Skills werden nach dem Plugin-Namen benannt, daher bietet **commit-commands** Skills wie `/commit-commands:commit`.

    Probieren Sie es aus, indem Sie eine Änderung an einer Datei vornehmen und ausführen:

    ```shell theme={null}
    /commit-commands:commit
    ```

    Dies stellt Ihre Änderungen bereit, generiert eine Commit-Nachricht und erstellt den Commit.

    Jedes Plugin funktioniert anders. Überprüfen Sie die Details des Plugins in der Registerkarte **Discover**, um zu sehen, welche Befehle und Skills es bietet, oder besuchen Sie seine Homepage für Anleitung zur Verwendung.
  </Step>
</Steps>

Der Rest dieses Leitfadens behandelt alle Möglichkeiten, wie Sie Marktplätze hinzufügen, Plugins installieren und Ihre Konfiguration verwalten können.

<h2 id="add-marketplaces">
  Marktplätze hinzufügen
</h2>

Verwenden Sie den Befehl `/plugin marketplace add`, um Marktplätze aus verschiedenen Quellen hinzuzufügen.

<Tip>
  **Verknüpfungen**: Sie können `/plugin market` anstelle von `/plugin marketplace` verwenden und `rm` anstelle von `remove`.
</Tip>

* **GitHub-Repositories**: Format `owner/repo` (z. B. `anthropics/claude-code`)
* **Git-URLs**: Beliebige Git-Repository-URL (GitLab, Bitbucket, selbstgehostet)
* **Lokale Pfade**: Verzeichnisse oder direkte Pfade zu `marketplace.json`-Dateien
* **Remote-URLs**: Direkte URLs zu gehosteten `marketplace.json`-Dateien

<h3 id="add-from-github">
  Hinzufügen von GitHub
</h3>

Fügen Sie ein GitHub-Repository hinzu, das eine `.claude-plugin/marketplace.json`-Datei enthält, indem Sie das Format `owner/repo` verwenden – wobei `owner` der GitHub-Benutzername oder die Organisation und `repo` der Repository-Name ist.

Beispielsweise bezieht sich `anthropics/claude-code` auf das Repository `claude-code`, das sich im Besitz von `anthropics` befindet:

```shell theme={null}
/plugin marketplace add anthropics/claude-code
```

<h3 id="add-from-other-git-hosts">
  Hinzufügen von anderen Git-Hosts
</h3>

Fügen Sie ein beliebiges Git-Repository hinzu, indem Sie die vollständige URL angeben. Dies funktioniert mit jedem Git-Host, einschließlich GitLab, Bitbucket und selbstgehosteten Servern. Fügen Sie das Suffix `.git` ein, damit Claude Code das Repository klont, anstatt die URL als direkten Link zu einer gehosteten `marketplace.json`-Datei zu behandeln.

Fügen Sie auch das Präfix `https://` ein. Claude Code v2.1.196 und später lehnen einen Host ab, der ohne Präfix eingegeben wird, z. B. `gitlab.com/company/plugins.git`, als ungültige GitHub-`owner/repo`-Kurzform ab, und die Fehlermeldung weist Sie an, das Präfix hinzuzufügen. Frühere Versionen interpretieren es fälschlicherweise als GitHub-Repository-Pfad und schlagen beim Klonen fehl.

Mit HTTPS:

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git
```

Mit SSH:

```shell theme={null}
/plugin marketplace add git@gitlab.com:company/plugins.git
```

Um einen bestimmten Branch oder Tag hinzuzufügen, hängen Sie `#` gefolgt von der Referenz an:

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git#v1.0.0
```

<h3 id="add-from-local-paths">
  Hinzufügen von lokalen Pfaden
</h3>

Fügen Sie ein lokales Verzeichnis hinzu, das eine `.claude-plugin/marketplace.json`-Datei enthält:

```shell theme={null}
/plugin marketplace add ./my-marketplace
```

Sie können auch einen direkten Pfad zu einer `marketplace.json`-Datei hinzufügen:

```shell theme={null}
/plugin marketplace add ./path/to/marketplace.json
```

<h3 id="add-from-remote-urls">
  Hinzufügen von Remote-URLs
</h3>

Fügen Sie eine Remote-`marketplace.json`-Datei über URL hinzu:

```shell theme={null}
/plugin marketplace add https://example.com/marketplace.json
```

<Note>
  URL-basierte Marktplätze haben einige Einschränkungen im Vergleich zu Git-basierten Marktplätzen. Wenn beim Installieren von Plugins Fehler „Pfad nicht gefunden" auftreten, siehe [Fehlerbehebung](/docs/de/plugin-marketplaces#plugins-with-relative-paths-fail-in-url-based-marketplaces).
</Note>

<h2 id="install-plugins">
  Installieren Sie Plugins
</h2>

Nachdem Sie Marktplätze hinzugefügt haben, können Sie Plugins direkt installieren:

```shell theme={null}
/plugin install plugin-name@marketplace-name
```

Der Befehl öffnet die Details dieses Plugins, wo Sie einen [Installationsbereich](/docs/de/settings#configuration-scopes) wählen. Sie sehen die gleichen Optionen, wenn Sie `/plugin` ausführen, zur Registerkarte **Discover** gehen und **Enter** auf einem Plugin drücken:

* **User scope** (Standard): Installieren Sie für sich selbst in allen Projekten
* **Project scope**: Installieren Sie für alle Mitarbeiter in diesem Repository, was das Plugin zu `.claude/settings.json` hinzufügt
* **Local scope**: Installieren Sie für sich selbst nur in diesem Repository, nicht mit Mitarbeitern geteilt

Um ohne einen interaktiven Schritt zu installieren, verwenden Sie den [`claude plugin install`](/docs/de/plugins-reference#plugin-install) Shell-Befehl, der im Benutzerbereich installiert, es sei denn, Sie übergeben `--scope`.

Sie können auch Plugins mit **managed**-Bereich sehen. Diese werden von Administratoren über [verwaltete Einstellungen](/docs/de/settings#settings-files) installiert und können nicht geändert werden.

<Warning>
  Stellen Sie sicher, dass Sie einem Plugin vertrauen, bevor Sie es installieren. Anthropic kontrolliert nicht, welche MCP-Server, Dateien oder andere Software in Plugins enthalten sind, und kann nicht überprüfen, dass sie wie beabsichtigt funktionieren. Überprüfen Sie die Homepage jedes Plugins für weitere Informationen.
</Warning>

<h2 id="manage-installed-plugins">
  Verwalten Sie installierte Plugins
</h2>

Führen Sie `/plugin` aus und gehen Sie zur Registerkarte **Installed**, um Ihre Plugins anzuzeigen, zu aktivieren, zu deaktivieren oder zu deinstallieren. Die Liste ist nach Bereich gruppiert und sortiert, sodass Sie zuerst Probleme sehen: Plugins mit Ladefehlern oder ungelösten Abhängigkeiten erscheinen oben, gefolgt von Ihren Favoriten, mit deaktivierten Plugins, die hinter einem eingeklappten Header am unteren Rand verborgen sind.

Aus der Liste können Sie:

* `f` drücken, um das ausgewählte Plugin zu favorisieren oder zu entfavorisieren
* eingeben, um nach Plugin-Name oder Beschreibung zu filtern
* Enter drücken, um die Detailansicht eines Plugins zu öffnen und es zu aktivieren, zu deaktivieren oder zu deinstallieren

Das Deinstallieren eines Plugins, das die `.claude/settings.json` eines Projekts aktiviert, fragt, welchen Bereich Sie meinen: Deaktivieren Sie es nur für sich selbst, was einen Override in Ihre `.claude/settings.local.json` schreibt und das Plugin für das Projekt installiert lässt, oder deinstallieren Sie es für alle, was es aus der gemeinsamen `.claude/settings.json` entfernt. Erfordert Claude Code v2.1.203 oder später. Vor v2.1.203 bot der Dialog nur die lokale Deaktivierung an.

Die Detailansicht zeigt die Komponenten, die das Plugin bereitstellt: Befehle, Skills, Agenten, Hooks, MCP-Server und LSP-Server. Das gleiche Inventar ist über die Befehlszeile mit `claude plugin details` verfügbar.

Die Registerkarte **Installed** sammelt auch Marketplace-Plugins, die Sie selbst installiert haben, aber in mindestens zwei Wochen über einen Zeitraum von mindestens 10 Sitzungen nicht verwendet haben, unter einem Header **Not used recently**. Die Detailansicht zeigt eine Zeile **Last used** für jedes Plugin. Verwenden Sie diese, um Plugins zu finden, die immer noch Startup- und Kontextkosten verursachen, obwohl Sie sie nicht mehr verwenden, und deaktivieren oder deinstallieren Sie diese dann. Erfordert Claude Code v2.1.187 oder später.

Zwei Arten von Plugins werden niemals als ungenutzt aufgelistet:

* Plugins, die Ihre Organisation verwaltet oder die Sie mit `--plugin-dir` laden
* Plugins, die ein Theme, einen Output-Stil, einen Monitor oder einen Workflow bereitstellen, da diese Wert ohne eine zu verfolgbare Invocation liefern

Der Header **Not used recently** und die Zeile **Last used** sind beide verborgen, wenn Ihre Organisation Marketplaces mit [`strictKnownMarketplaces`](/docs/de/settings#strictknownmarketplaces) einschränkt.

Ein [Language Server](/docs/de/plugins#add-lsp-servers-to-your-plugin) eines Plugins zählt als verwendet, wenn er Diagnostiken liefert oder eine Code-Navigationsanfrage beantwortet, sodass ein LSP-Plugin, dessen Server in Ihren Sitzungen aktiv ist, nicht als ungenutzt aufgelistet wird. Vor v2.1.203 konnte die Language-Server-Aktivität nicht als Verwendung gezählt werden, sodass Plugins, die einen LSP-Server bereitstellen, vollständig von der Gruppe ausgenommen waren, genauso wie Theme- und Output-Style-Plugins es immer noch sind.

Die erste Sitzung in einer Version, die Language-Server-Aktivität zählt, setzt auch den Verwendungsdatensatz jedes LSP-Plugins zurück, das noch keine Verwendung aufgezeichnet hatte, sodass Claude Code ein Plugin, das Sie früher installiert haben, nicht als ungenutzt beurteilt, basierend auf Daten, die aufgezeichnet wurden, bevor seine Server-Aktivität verfolgt wurde. Vor v2.1.206 konnte diese erste Sitzung ein aktiv genutztes LSP-Plugin unter **Not used recently** auflisten und vorschlagen, es zu überprüfen.

Wenn Sie ein Plugin installieren, das Abhängigkeiten deklariert, listet die Installationsausgabe auf, welche Abhängigkeiten zusammen mit ihm automatisch installiert wurden.

Sie können Plugins auch mit direkten Befehlen verwalten.

Installierte Plugins auflisten, ohne das Menü zu öffnen:

```shell theme={null}
/plugin list
```

Übergeben Sie `--enabled` oder `--disabled`, um nur Plugins in diesem Zustand anzuzeigen.

Deaktivieren Sie ein Plugin, ohne es zu deinstallieren:

```shell theme={null}
/plugin disable plugin-name@marketplace-name
```

Aktivieren Sie ein deaktiviertes Plugin erneut:

```shell theme={null}
/plugin enable plugin-name@marketplace-name
```

In diesen Bezeichnern ist `plugin-name` der `name` des Plugins im [Marketplace-Eintrag](/docs/de/plugin-marketplaces#plugin-entries), der sich vom `name` in der eigenen `plugin.json` des Plugins unterscheiden kann.

Ab Claude Code v2.1.195 funktionieren **Enable** und **Disable** in der `/plugin`-Schnittstelle für Plugins, deren zwei Namen unterschiedlich sind, und `/plugin enable` und `/plugin disable` akzeptieren beide Namen. Wenn Sie ein solches Plugin in einer früheren Version deaktivieren, meldet Claude Code `already disabled` und lässt es aktiviert.

Entfernen Sie ein Plugin vollständig:

```shell theme={null}
/plugin uninstall plugin-name@marketplace-name
```

Die Option `--scope` ermöglicht es Ihnen, einen bestimmten Bereich mit CLI-Befehlen anzusteuern:

```shell theme={null}
claude plugin install formatter@your-org --scope project
claude plugin uninstall formatter@your-org --scope project
```

<h3 id="apply-plugin-changes-without-restarting">
  Wenden Sie Plugin-Änderungen an, ohne neu zu starten
</h3>

Wenn Sie während einer Sitzung Plugins installieren, aktivieren oder deaktivieren, führen Sie `/reload-plugins` aus, um alle Änderungen ohne Neustart zu aktivieren:

```shell theme={null}
/reload-plugins
```

Claude Code lädt alle aktiven Plugins neu und zeigt Zählungen für Plugins, Skills, Agenten, Hooks, Plugin-MCP-Server und Plugin-LSP-Server an.

Das Neuladen hat Tokenkosten bei der nächsten Anfrage: Neu geladene Komponenten kündigen sich in Inhalten an, die zum Gespräch hinzugefügt werden, während der vorhandene Verlauf weiterhin aus dem Prompt-Cache gelesen wird. Ein Plugin, das MCP-Server bereitstellt, kostet mehr, wenn seine Tools nicht durch [Tool-Suche](/docs/de/mcp#scale-with-mcp-tool-search) aufgeschoben werden: Die Änderung invalidiert den Cache und die nächste Anfrage liest das gesamte Gespräch erneut. In diesem Fall zeigt `/reload-plugins` eine Warnung an und wendet das Neuladen nicht an; übergeben Sie `--force`, um es trotzdem anzuwenden. Weitere Informationen finden Sie unter [Aktivieren oder Deaktivieren eines Plugins](/docs/de/prompt-caching#enabling-or-disabling-a-plugin).

<h2 id="manage-marketplaces">
  Marktplätze verwalten
</h2>

Sie können Marktplätze über die interaktive `/plugin`-Schnittstelle oder mit CLI-Befehlen verwalten.

<h3 id="use-the-interactive-interface">
  Verwenden Sie die interaktive Schnittstelle
</h3>

Führen Sie `/plugin` aus und gehen Sie zur Registerkarte **Marketplaces**, um:

* Alle Ihre hinzugefügten Marktplätze mit ihren Quellen und Status anzuzeigen
* Neue Marktplätze hinzuzufügen
* Marktplatz-Auflistungen aktualisieren, um die neuesten Plugins abzurufen
* Marktplätze zu entfernen, die Sie nicht mehr benötigen

<h3 id="use-cli-commands">
  Verwenden Sie CLI-Befehle
</h3>

Sie können Marktplätze auch mit direkten Befehlen verwalten.

Listet alle konfigurierten Marktplätze auf:

```shell theme={null}
/plugin marketplace list
```

Aktualisieren Sie Plugin-Auflistungen von einem Marktplatz:

```shell theme={null}
/plugin marketplace update marketplace-name
```

Entfernen Sie einen Marktplatz:

```shell theme={null}
/plugin marketplace remove marketplace-name
```

<Warning>
  Das Entfernen eines Marktplatzes deinstalliert alle Plugins, die Sie von ihm installiert haben.
</Warning>

<h3 id="configure-auto-updates">
  Konfigurieren Sie automatische Updates
</h3>

Claude Code kann Marktplätze und ihre installierten Plugins im Hintergrund nach dem Start automatisch aktualisieren. Wenn die automatische Aktualisierung für einen Marktplatz aktiviert ist, aktualisiert Claude Code die Marktplatzdaten und aktualisiert installierte Plugins auf ihre neuesten Versionen auf der Festplatte.

Claude Code prüft nach dem Start Ihrer Sitzung auf Marktplatz- und Plugin-Updates mit einer zufälligen Verzögerung von bis zu zehn Minuten, sodass die laufende Sitzung weiterhin die Versionen verwendet, die beim Start geladen wurden. Wenn Plugins aktualisiert wurden, sehen Sie eine Benachrichtigung, die Sie auffordert, `/reload-plugins` auszuführen, oder die neuen Versionen werden beim nächsten Start geladen.

Schalten Sie die automatische Aktualisierung für einzelne Marktplätze über die Benutzeroberfläche um:

1. Führen Sie `/plugin` aus, um den Plugin-Manager zu öffnen
2. Wählen Sie **Marketplaces**
3. Wählen Sie einen Marktplatz aus der Liste
4. Wählen Sie **Enable auto-update** oder **Disable auto-update**

Offizielle Anthropic-Marktplätze haben die automatische Aktualisierung standardmäßig aktiviert. Marktplätze von Drittanbietern und lokale Entwicklungsmarktplätze haben die automatische Aktualisierung standardmäßig deaktiviert.

Administratoren können auch `"autoUpdate": true` für jeden [`extraKnownMarketplaces`](/docs/de/settings#extraknownmarketplaces)-Eintrag in verwalteten Einstellungen festlegen, um die automatische Aktualisierung für einen Organisationsmarktplatz zu aktivieren, ohne dass jeder Benutzer diese umschalten muss.

Um alle automatischen Updates vollständig für Claude Code und alle Plugins zu deaktivieren, setzen Sie die Umgebungsvariable `DISABLE_AUTOUPDATER`. Siehe [Auto-Updates](/docs/de/setup#auto-updates) für Details.

Um Plugin-Auto-Updates aktiviert zu halten und gleichzeitig Claude Code-Auto-Updates zu deaktivieren, setzen Sie `FORCE_AUTOUPDATE_PLUGINS=1` zusammen mit `DISABLE_AUTOUPDATER`:

```bash theme={null}
export DISABLE_AUTOUPDATER=1
export FORCE_AUTOUPDATE_PLUGINS=1
```

Dies ist nützlich, wenn Sie Claude Code-Updates manuell verwalten möchten, aber immer noch automatische Plugin-Updates erhalten möchten.

<h2 id="configure-team-marketplaces">
  Konfigurieren Sie Team-Marktplätze
</h2>

Team-Administratoren können die automatische Marktplatz-Installation für Projekte einrichten, indem sie Marktplatz-Konfiguration zu `.claude/settings.json` hinzufügen. Wenn Team-Mitglieder dem Repository-Ordner vertrauen, fordert Claude Code sie auf, diese Marktplätze und Plugins zu installieren.

Ab Claude Code v2.1.195 gilt dieser Installationsschritt auf jedem Pfad, der Plugins lädt. Ein Plugin, das nur die `.claude/settings.json` des Projekts aktiviert und das aus einer externen Quelle wie einem GitHub-Repository oder npm-Paket stammt, wird nicht geladen, bis das Team-Mitglied es installiert. Bis dahin meldet Claude Code das Plugin als nicht installiert und zeigt den Befehl `claude plugin install` an, der ausgeführt werden soll.

Fügen Sie `extraKnownMarketplaces` zu Ihrer Projekt-`.claude/settings.json` hinzu:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "my-team-tools": {
      "source": {
        "source": "github",
        "repo": "your-org/claude-plugins"
      }
    }
  }
}
```

Für vollständige Konfigurationsoptionen einschließlich `extraKnownMarketplaces` und `enabledPlugins` siehe [Plugin-Einstellungen](/docs/de/settings#plugin-settings).

<h2 id="security">
  Sicherheit
</h2>

Plugins und Marktplätze sind hochgradig vertrauenswürdige Komponenten, die beliebigen Code auf Ihrem Computer mit Ihren Benutzerrechten ausführen können. Installieren Sie nur Plugins und fügen Sie Marktplätze aus Quellen hinzu, denen Sie vertrauen. Organisationen können einschränken, welche Marktplätze Benutzer hinzufügen dürfen, indem sie [verwaltete Marktplatz-Einschränkungen](/docs/de/plugin-marketplaces#managed-marketplace-restrictions) verwenden.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="/plugin-command-not-recognized">
  /plugin-Befehl nicht erkannt
</h3>

Wenn Sie „unknown command" sehen oder der `/plugin`-Befehl nicht angezeigt wird:

1. **Überprüfen Sie Ihre Version**: Führen Sie `claude --version` aus, um zu sehen, was installiert ist.
2. **Aktualisieren Sie Claude Code**:
   * **Homebrew**: `brew upgrade claude-code` oder `brew upgrade claude-code@latest`, wenn Sie diesen Cask installiert haben
   * **npm**: `npm install -g @anthropic-ai/claude-code@latest`
   * **Native Installer**: Führen Sie den Installationsbefehl von [Setup](/docs/de/setup) erneut aus
3. **Starten Sie Claude Code neu**: Starten Sie nach dem Update Ihr Terminal neu und führen Sie `claude` erneut aus.

<h3 id="common-issues">
  Häufige Probleme
</h3>

* **Marktplatz wird nicht geladen**: Überprüfen Sie, dass die URL zugänglich ist und dass `.claude-plugin/marketplace.json` unter dem Pfad vorhanden ist
* **Plugin-Installationsfehler**: Überprüfen Sie, dass Plugin-Quell-URLs zugänglich sind und Repositories öffentlich sind oder dass Sie Zugriff auf sie haben
* **Dateien nach der Installation nicht gefunden**: Plugins werden in einen Cache kopiert, daher funktionieren Pfade, die auf Dateien außerhalb des Plugin-Verzeichnisses verweisen, nicht
* **Plugin-Befähigungen werden nicht angezeigt**: Löschen Sie den Cache mit `rm -rf ~/.claude/plugins/cache`, starten Sie Claude Code neu und installieren Sie das Plugin erneut.

Für detaillierte Fehlerbehebung mit Lösungen siehe [Fehlerbehebung](/docs/de/plugin-marketplaces#troubleshooting) im Marktplatz-Leitfaden. Für Debugging-Tools siehe [Debugging- und Entwicklungstools](/docs/de/plugins-reference#debugging-and-development-tools).

<h3 id="code-intelligence-issues">
  Code-Intelligenz-Probleme
</h3>

* **Language Server startet nicht**: Überprüfen Sie, dass die Binärdatei installiert ist und in Ihrem `$PATH` verfügbar ist. Überprüfen Sie die Registerkarte `/plugin` Errors für Details.
* **Hohe Speichernutzung**: Language Server wie `rust-analyzer` und `pyright` können bei großen Projekten erhebliche Speichermengen verbrauchen. Wenn Sie Speicherprobleme haben, deaktivieren Sie das Plugin mit `/plugin disable <plugin-name>` und verlassen Sie sich stattdessen auf Claudes integrierte Suchtools.
* **Falsch positive Diagnosen in Monorepos**: Language Server können ungelöste Importfehler für interne Pakete melden, wenn der Arbeitsbereich nicht richtig konfiguriert ist. Diese beeinflussen nicht Claudes Fähigkeit, Code zu bearbeiten.

<h2 id="next-steps">
  Nächste Schritte
</h2>

* **Erstellen Sie Ihre eigenen Plugins**: Siehe [Plugins](/docs/de/plugins), um Befähigungen, Agenten und Hooks zu erstellen
* **Erstellen Sie einen Marktplatz**: Siehe [Erstellen Sie einen Plugin-Marktplatz](/docs/de/plugin-marketplaces), um Plugins an Ihr Team oder Ihre Community zu verteilen
* **Technische Referenz**: Siehe [Plugins-Referenz](/docs/de/plugins-reference) für vollständige Spezifikationen
