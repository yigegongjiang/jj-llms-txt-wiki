> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code erweitern

> Verstehen Sie, wann Sie CLAUDE.md, Skills, Subagents, Hooks, MCP und Plugins verwenden.

Claude Code kombiniert ein Modell, das über Ihren Code nachdenkt, mit [integrierten Tools](/docs/de/how-claude-code-works#tools) für Dateivorgänge, Suche, Ausführung und Webzugriff. Die integrierten Tools decken die meisten Codierungsaufgaben ab. Dieses Handbuch behandelt die Erweiterungsebene: Funktionen, die Sie hinzufügen, um anzupassen, was Claude weiß, es mit externen Diensten zu verbinden und Workflows zu automatisieren.

<Note>
  Informationen zur Funktionsweise der Kern-Agentenschleife finden Sie unter [How Claude Code works](/docs/de/how-claude-code-works).
</Note>

**Neu bei Claude Code?** Beginnen Sie mit [CLAUDE.md](/docs/de/memory) für Projektkonventionen. Fügen Sie dann andere Erweiterungen [hinzu, wenn spezifische Trigger auftreten](#build-your-setup-over-time).

<h2 id="overview">
  Übersicht
</h2>

Erweiterungen verbinden sich mit verschiedenen Teilen der Agentenschleife:

* **[CLAUDE.md](/docs/de/memory)** fügt persistenten Kontext hinzu, den Claude in jeder Sitzung sieht
* **[Skills](/docs/de/skills)** fügen wiederverwendbares Wissen und aufrufbare Workflows hinzu
* **[Code-Intelligenz](/docs/de/tools-reference#lsp-tool-behavior)** verbindet Claude mit einem Language Server für Symbol-Navigation und Live-Typfehler
* **[MCP](/docs/de/mcp)** verbindet Claude mit externen Diensten und Tools
* **[Subagents](/docs/de/sub-agents)** führen ihre eigenen Schleifen in isoliertem Kontext aus und geben Zusammenfassungen zurück
* **[Agent teams](/docs/de/agent-teams)** koordinieren mehrere unabhängige Sitzungen mit gemeinsamen Aufgaben und Peer-to-Peer-Messaging
* **[Hooks](/docs/de/hooks-guide)** werden bei Lebenszyklusereignissen ausgelöst und können ein Skript, eine HTTP-Anfrage, einen Prompt oder einen Subagent ausführen
* **[Plugins](/docs/de/plugins)** und **[Marketplaces](/docs/de/plugin-marketplaces)** verpacken und verteilen diese Funktionen

[Skills](/docs/de/skills) sind die flexibelste Erweiterung. Ein Skill ist eine Markdown-Datei, die Wissen, Workflows oder Anweisungen enthält. Sie können Skills mit einem Befehl wie `/deploy` aufrufen, oder Claude kann sie automatisch laden, wenn sie relevant sind. Skills können in Ihrer aktuellen Konversation oder in einem isolierten Kontext über Subagents ausgeführt werden.

<h2 id="match-features-to-your-goal">
  Funktionen an Ihr Ziel anpassen
</h2>

Funktionen reichen von immer aktivem Kontext, den Claude in jeder Sitzung sieht, bis zu On-Demand-Funktionen, die Sie oder Claude aufrufen können, bis zu Hintergrundautomatisierung, die bei bestimmten Ereignissen ausgeführt wird. Die folgende Tabelle zeigt, was verfügbar ist und wann jede Funktion sinnvoll ist.

| Funktion                                                       | Was sie tut                                                               | Wann man sie verwendet                                                                   | Beispiel                                                                                 |
| -------------------------------------------------------------- | ------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| **CLAUDE.md**                                                  | Persistenter Kontext, der in jeder Konversation geladen wird              | Projektkonventionen, „immer X tun"-Regeln                                                | „Verwenden Sie pnpm, nicht npm. Führen Sie Tests vor dem Commit aus."                    |
| **Skill**                                                      | Anweisungen, Wissen und Workflows, die Claude verwenden kann              | Wiederverwendbarer Inhalt, Referenzdokumente, wiederholbare Aufgaben                     | `/deploy` führt Ihre Bereitstellungs-Checkliste aus; API-Docs-Skill mit Endpunkt-Mustern |
| **Subagent**                                                   | Isolierter Ausführungskontext, der zusammengefasste Ergebnisse zurückgibt | Kontextisolation, parallele Aufgaben, spezialisierte Worker                              | Recherche-Aufgabe, die viele Dateien liest, aber nur wichtige Erkenntnisse zurückgibt    |
| **[Agent teams](/docs/de/agent-teams)**                             | Koordinieren Sie mehrere unabhängige Claude Code-Sitzungen                | Parallele Recherche, neue Funktionsentwicklung, Debugging mit konkurrierenden Hypothesen | Spawnen Sie Reviewer, um Sicherheit, Leistung und Tests gleichzeitig zu überprüfen       |
| **[Code intelligence](/docs/de/tools-reference#lsp-tool-behavior)** | Language-Server-Navigation und Diagnostik                                 | Typisierte Sprachen, große Codebases, bei denen grep langsam oder ungenau ist            | Springen Sie zur Definition eines Symbols, anstatt die ganze Datei zu lesen              |
| **MCP**                                                        | Verbindung zu externen Diensten                                           | Externe Daten oder Aktionen                                                              | Abfrage Ihrer Datenbank, Posten auf Slack, Steuerung eines Browsers                      |
| **Hook**                                                       | Skript, HTTP-Anfrage, Prompt oder Subagent, ausgelöst durch Ereignisse    | Automatisierung, die bei jedem übereinstimmenden Ereignis ausgeführt werden muss         | Führen Sie ESLint nach jeder Dateibearbeitung aus                                        |
| **[Artifact](/docs/de/artifacts)**                                  | Veröffentlichen Sie die Sitzungsausgabe als private, interaktive Webseite | Ausgabe, die Sie visuell sehen oder teilen möchten, anstatt als Terminaltext             | Eine Incident-Timeline, die sich aktualisiert, während Claude untersucht                 |

**[Plugins](/docs/de/plugins)** sind die Verpackungsebene. Ein Plugin bündelt Skills, Hooks, Subagents und MCP-Server in eine einzelne installierbare Einheit. Plugin-Skills sind namensgebunden (wie `/my-plugin:review`), sodass mehrere Plugins nebeneinander existieren können. Verwenden Sie Plugins, wenn Sie dasselbe Setup über mehrere Repositories hinweg wiederverwenden möchten oder es über einen **[Marketplace](/docs/de/plugin-marketplaces)** an andere verteilen möchten.

<h3 id="build-your-setup-over-time">
  Bauen Sie Ihr Setup im Laufe der Zeit auf
</h3>

Sie müssen nicht alles im Voraus konfigurieren. Jede Funktion hat einen erkennbaren Trigger, und die meisten Teams fügen sie ungefähr in dieser Reihenfolge hinzu:

| Trigger                                                                                               | Hinzufügen                                                                                               |
| :---------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------- |
| Claude bekommt eine Konvention oder einen Befehl zweimal falsch                                       | Fügen Sie es zu [CLAUDE.md](/docs/de/memory) hinzu                                                            |
| Sie tippen immer wieder denselben Prompt, um eine Aufgabe zu starten                                  | Speichern Sie es als benutzer-aufrufen [Skill](/docs/de/skills)                                               |
| Sie fügen zum dritten Mal dasselbe Playbook oder mehrstufige Verfahren in den Chat ein                | Erfassen Sie es als [Skill](/docs/de/skills)                                                                  |
| Sie kopieren immer wieder Daten aus einer Browser-Registerkarte, die Claude nicht sehen kann          | Verbinden Sie dieses System als [MCP-Server](/docs/de/mcp)                                                    |
| Claude liest viele Dateien, um zu finden, wo ein Symbol definiert oder verwendet wird                 | Installieren Sie ein [Code-Intelligence-Plugin](/docs/de/discover-plugins#code-intelligence) für Ihre Sprache |
| Eine Nebenaufgabe überschwemmt Ihre Konversation mit Ausgabe, auf die Sie nicht mehr verweisen werden | Leiten Sie sie durch einen [Subagent](/docs/de/sub-agents)                                                    |
| Sie möchten, dass etwas jedes Mal passiert, ohne zu fragen                                            | Schreiben Sie einen [Hook](/docs/de/hooks-guide)                                                              |
| Ein zweites Repository benötigt dasselbe Setup                                                        | Verpacken Sie es als [Plugin](/docs/de/plugins)                                                               |

Die gleichen Trigger sagen Ihnen, wann Sie das aktualisieren, was Sie bereits haben. Ein wiederholter Fehler oder ein wiederkehrender Review-Kommentar ist eine CLAUDE.md-Bearbeitung, keine einmalige Korrektur im Chat. Ein Workflow, den Sie immer wieder von Hand anpassen, ist ein Skill, der eine weitere Überarbeitung benötigt.

<h3 id="compare-similar-features">
  Vergleichen Sie ähnliche Funktionen
</h3>

Einige Funktionen können ähnlich wirken. Für eine tiefere Anleitung zur Auswahl zwischen ihnen siehe [Steering Claude Code: when to use CLAUDE.md, skills, hooks, and subagents](https://claude.com/blog/steering-claude-code-skills-hooks-rules-subagents-and-more) im Blog. Hier erfahren Sie, wie Sie sie unterscheiden.

<Tabs>
  <Tab title="Skill vs Subagent">
    Skills und Subagents lösen unterschiedliche Probleme:

    * **Skills** sind wiederverwendbare Inhalte, die Sie in jeden Kontext laden können
    * **Subagents** sind isolierte Worker, die separat von Ihrer Hauptkonversation ausgeführt werden

    | Aspekt                                              | Skill                                                | Subagent                                                                                 |
    | --------------------------------------------------- | ---------------------------------------------------- | ---------------------------------------------------------------------------------------- |
    | **Was es ist**                                      | Wiederverwendbare Anweisungen, Wissen oder Workflows | Isolierter Worker mit eigenem Kontext                                                    |
    | **Hauptvorteil**                                    | Inhalte über Kontexte hinweg teilen                  | Kontextisolation. Die Arbeit erfolgt separat, nur die Zusammenfassung wird zurückgegeben |
    | **[Kontextfenster](/docs/de/context-window) Auswirkung** | Wird zu Ihrem Hauptfenster hinzugefügt               | Verwendet ein separates Fenster mit eigenen Input- und Output-Tokens                     |
    | **Am besten für**                                   | Referenzmaterial, aufrufbare Workflows               | Aufgaben, die viele Dateien lesen, parallele Arbeit, spezialisierte Worker               |

    **Skills können Referenz oder Aktion sein.** Referenz-Skills bieten Wissen, das Claude während Ihrer Sitzung nutzt (wie Ihr API-Stilhandbuch). Action-Skills sagen Claude, etwas Bestimmtes zu tun (wie `/deploy`, das Ihren Bereitstellungs-Workflow ausführt).

    **Verwenden Sie einen Subagent**, wenn Sie Kontextisolation benötigen oder wenn Ihr Kontextfenster voll wird. Der Subagent könnte Dutzende von Dateien lesen oder umfangreiche Suchen durchführen, aber Ihre Hauptkonversation erhält nur eine Zusammenfassung. Da die Arbeit des Subagent Ihren Hauptkontext nicht verbraucht, ist dies auch nützlich, wenn Sie nicht möchten, dass die Zwischenarbeit sichtbar bleibt. Benutzerdefinierte Subagents können ihre eigenen Anweisungen haben und können Skills vorladen.

    **Sie können sich kombinieren.** Ein Subagent kann spezifische Skills vorladen (`skills:`-Feld). Ein Skill kann in isoliertem Kontext mit `context: fork` ausgeführt werden. Weitere Informationen finden Sie unter [Skills](/docs/de/skills).
  </Tab>

  <Tab title="CLAUDE.md vs Skill">
    Beide speichern Anweisungen, aber sie werden unterschiedlich geladen und dienen unterschiedlichen Zwecken.

    | Aspekt                      | CLAUDE.md                 | Skill                                  |
    | --------------------------- | ------------------------- | -------------------------------------- |
    | **Lädt**                    | Jede Sitzung, automatisch | On Demand                              |
    | **Kann Dateien enthalten**  | Ja, mit `@path`-Importen  | Ja, mit `@path`-Importen               |
    | **Kann Workflows auslösen** | Nein                      | Ja, mit `/<name>`                      |
    | **Am besten für**           | „Immer X tun"-Regeln      | Referenzmaterial, aufrufbare Workflows |

    **Fügen Sie es in CLAUDE.md ein**, wenn Claude es immer wissen sollte: Codierungskonventionen, Build-Befehle, Projektstruktur, „niemals X tun"-Regeln.

    **Fügen Sie es in einen Skill ein**, wenn es Referenzmaterial ist, das Claude manchmal benötigt (API-Docs, Stilhandbücher) oder ein Workflow, den Sie mit `/<name>` auslösen (bereitstellen, überprüfen, freigeben).

    **Faustregel:** Halten Sie CLAUDE.md unter 200 Zeilen. Wenn es wächst, verschieben Sie Referenzinhalte zu Skills oder teilen Sie sie in [`.claude/rules/`](/docs/de/memory#organize-rules-with-claude%2Frules%2F)-Dateien auf.
  </Tab>

  <Tab title="CLAUDE.md vs Rules vs Skills">
    Alle drei speichern Anweisungen, aber sie werden unterschiedlich geladen:

    | Aspekt            | CLAUDE.md                          | `.claude/rules/`                                                | Skill                                     |
    | ----------------- | ---------------------------------- | --------------------------------------------------------------- | ----------------------------------------- |
    | **Lädt**          | Jede Sitzung                       | Jede Sitzung oder wenn übereinstimmende Dateien geöffnet werden | On Demand, wenn aufgerufen oder relevant  |
    | **Umfang**        | Ganzes Projekt                     | Kann auf Dateipfade begrenzt werden                             | Aufgabenspezifisch                        |
    | **Am besten für** | Kernkonventionen und Build-Befehle | Sprachspezifische oder verzeichnisspezifische Richtlinien       | Referenzmaterial, wiederholbare Workflows |

    **Verwenden Sie CLAUDE.md** für Anweisungen, die jede Sitzung benötigt: Build-Befehle, Test-Konventionen, Projektarchitektur.

    **Verwenden Sie Regeln**, um CLAUDE.md fokussiert zu halten. Regeln mit [`paths`-Frontmatter](/docs/de/memory#path-specific-rules) werden nur geladen, wenn Claude mit übereinstimmenden Dateien arbeitet, was Kontext spart.

    **Verwenden Sie Skills** für Inhalte, die Claude nur manchmal benötigt, wie API-Dokumentation oder eine Bereitstellungs-Checkliste, die Sie mit `/<name>` auslösen.
  </Tab>

  <Tab title="Subagent vs Agent team">
    Beide parallelisieren Arbeit, aber sie sind architektonisch unterschiedlich:

    * **Subagents** laufen in Ihrer Sitzung und berichten Ergebnisse an Ihren Hauptkontext zurück
    * **Agent teams** sind unabhängige Claude Code-Sitzungen, die miteinander kommunizieren

    | Aspekt            | Subagent                                                      | Agent team                                                   |
    | ----------------- | ------------------------------------------------------------- | ------------------------------------------------------------ |
    | **Kontext**       | Eigenes Kontextfenster; Ergebnisse kehren zum Aufrufer zurück | Eigenes Kontextfenster; vollständig unabhängig               |
    | **Kommunikation** | Berichtet Ergebnisse nur an den Hauptagent zurück             | Teammates senden sich gegenseitig direkt Nachrichten         |
    | **Koordination**  | Hauptagent verwaltet alle Arbeiten                            | Gemeinsame Aufgabenliste mit Selbstkoordination              |
    | **Am besten für** | Fokussierte Aufgaben, bei denen nur das Ergebnis zählt        | Komplexe Arbeit, die Diskussion und Zusammenarbeit erfordert |
    | **Token-Kosten**  | Niedriger: Ergebnisse werden zum Hauptkontext zusammengefasst | Höher: jeder Teammate ist eine separate Claude-Instanz       |

    **Verwenden Sie einen Subagent**, wenn Sie einen schnellen, fokussierten Worker benötigen: eine Frage recherchieren, eine Behauptung überprüfen, eine Datei überprüfen. Der Subagent erledigt die Arbeit und gibt eine Zusammenfassung zurück. Ihre Hauptkonversation bleibt sauber.

    **Verwenden Sie ein Agent Team**, wenn Teammates Erkenntnisse teilen, sich gegenseitig in Frage stellen und unabhängig koordinieren müssen. Agent Teams sind am besten für Recherche mit konkurrierenden Hypothesen, parallele Code-Überprüfung und neue Funktionsentwicklung, bei der jeder Teammate ein separates Stück besitzt.

    **Übergangspunkt:** Wenn Sie parallele Subagents ausführen, aber auf Kontextgrenzen stoßen, oder wenn Ihre Subagents miteinander kommunizieren müssen, sind Agent Teams der natürliche nächste Schritt.

    <Note>
      Agent Teams sind experimentell und standardmäßig deaktiviert. Weitere Informationen zu Setup und aktuellen Einschränkungen finden Sie unter [agent teams](/docs/de/agent-teams).
    </Note>
  </Tab>

  <Tab title="MCP vs Skill">
    MCP verbindet Claude mit externen Diensten. Skills erweitern das Wissen von Claude, einschließlich der effektiven Verwendung dieser Dienste.

    | Aspekt         | MCP                                                     | Skill                                                              |
    | -------------- | ------------------------------------------------------- | ------------------------------------------------------------------ |
    | **Was es ist** | Protokoll zur Verbindung mit externen Diensten          | Wissen, Workflows und Referenzmaterial                             |
    | **Bietet**     | Tools und Datenzugriff                                  | Wissen, Workflows, Referenzmaterial                                |
    | **Beispiele**  | Slack-Integration, Datenbankabfragen, Browser-Steuerung | Code-Review-Checkliste, Bereitstellungs-Workflow, API-Stilhandbuch |

    Diese lösen unterschiedliche Probleme und funktionieren gut zusammen:

    **MCP** gibt Claude zweckgebundene Tools für ein externes System, wobei die Verbindung und Authentifizierung vom Server verwaltet werden.

    **Skills** geben Claude Wissen darüber, wie diese Tools effektiv verwendet werden, plus Workflows, die Sie mit `/<name>` auslösen können. Ein Skill könnte Ihr Team-Datenbankschema und Abfragemuster enthalten, oder einen `/post-to-slack`-Workflow mit Ihren Team-Nachrichtenformatierungsregeln.

    Beispiel: Ein MCP-Server verbindet Claude mit Ihrer Datenbank. Ein Skill lehrt Claude Ihr Datenmodell, häufige Abfragemuster und welche Tabellen für verschiedene Aufgaben verwendet werden.
  </Tab>

  <Tab title="Hook vs Skill">
    Ein Hook wird bei einem Lebenszyklusereignis ausgelöst; ein Skill wird in den Kontext geladen, damit Claude ihn anwendet.

    | Aspekt              | Hook                                                                                    | Skill                                                                        |
    | ------------------- | --------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
    | **Läuft**           | Ein Shell-Befehl, eine HTTP-Anfrage, ein LLM-Prompt oder ein Subagent                   | Anweisungen, die Claude liest und befolgt                                    |
    | **Ausgelöst durch** | [Lebenszyklusereignisse](/docs/de/hooks#hook-events) wie `PostToolUse` oder `SessionStart`   | Sie tippen `/<name>`, oder Claude passt die Beschreibung zu Ihrer Aufgabe an |
    | **Determinismus**   | Wird immer bei seinem Ereignis ausgelöst; der Trigger ist garantiert                    | Claude interpretiert die Anweisungen; das Ergebnis kann variieren            |
    | **Kontextkosten**   | Null, es sei denn, der Hook gibt Ausgabe zurück                                         | Beschreibung lädt jede Sitzung; vollständiger Inhalt lädt bei Verwendung     |
    | **Am besten für**   | Linting nach Bearbeitungen, Blockierung unsicherer Befehle, Logging, Benachrichtigungen | Workflows, die Überlegung benötigen, Referenzmaterial, mehrstufige Aufgaben  |

    **Verwenden Sie einen Hook**, wenn die Aktion jedes Mal auf die gleiche Weise erfolgen muss und Claude nicht denken muss. Beispiel: Formatierung beim Speichern, Ablehnung von `rm -rf /`, Posten einer Slack-Nachricht, wenn eine Sitzung endet.

    **Verwenden Sie einen Skill**, wenn Claude entscheiden sollte, wie die Schritte angewendet werden, oder wenn der Inhalt eher Wissen als ein Skript ist. Beispiel: eine `/release`-Checkliste, Ihr API-Stilhandbuch, ein Debugging-Playbook.

    **Setzen Sie Schutzmaßnahmen in Hooks.** Eine Anweisung wie „niemals `.env` bearbeiten" in CLAUDE.md oder einem Skill ist eine Anfrage, keine Garantie. Ein `PreToolUse`-Hook, der die Bearbeitung blockiert, ist Durchsetzung. Wenn eine Regel jedes Mal gelten muss, machen Sie sie zu einem Hook statt zu einer Prompt-Anweisung.

    **Hook-Ausgabe landet im Kontext.** Ein `PostToolUse`-Hook, der Ihren Linter ausführt, gibt Ergebnisse als Text zurück, den Claude liest; ein `/fix-lint`-Skill sagt Claude, wie sie zu beheben sind.
  </Tab>
</Tabs>

<h3 id="understand-how-features-layer">
  Verstehen Sie, wie Funktionen sich schichten
</h3>

Funktionen können auf mehreren Ebenen definiert werden: benutzerübergreifend, pro Projekt, über Plugins oder durch verwaltete Richtlinien. Sie können auch CLAUDE.md-Dateien in Unterverzeichnissen verschachteln oder Skills in bestimmten Paketen eines Monorepos platzieren. Wenn dieselbe Funktion auf mehreren Ebenen vorhanden ist, so schichten sie sich:

* **CLAUDE.md-Dateien** sind additiv: alle Ebenen tragen gleichzeitig Inhalte zu Claudes Kontext bei. Dateien aus Ihrem Arbeitsverzeichnis und darüber werden beim Start geladen; Unterverzeichnisse werden geladen, wenn Sie darin arbeiten. Wenn Anweisungen in Konflikt geraten, nutzt Claude sein Urteilsvermögen, um sie zu reconciliieren, wobei spezifischere Anweisungen typischerweise Vorrang haben. Siehe [wie CLAUDE.md-Dateien geladen werden](/docs/de/memory#how-claude-md-files-load).
* **Skills und Subagents** überschreiben nach Name: wenn derselbe Name auf mehreren Ebenen vorhanden ist, gewinnt eine Definition basierend auf Priorität (verwaltet > Benutzer > Projekt für Skills; verwaltet > CLI-Flag > Projekt > Benutzer > Plugin für Subagents). Plugin-Skills sind [namensgebunden](/docs/de/plugins#add-skills-to-your-plugin), um Konflikte zu vermeiden. Siehe [Skill-Erkennung](/docs/de/skills#where-skills-live) und [Subagent-Umfang](/docs/de/sub-agents#choose-the-subagent-scope).
* **MCP-Server** überschreiben nach Name: lokal > Projekt > Benutzer. Siehe [MCP-Umfang](/docs/de/mcp#scope-hierarchy-and-precedence).
* **Hooks** zusammenführen: alle registrierten Hooks werden für ihre übereinstimmenden Ereignisse unabhängig von der Quelle ausgelöst. Siehe [Hooks](/docs/de/hooks).

<h3 id="combine-features">
  Kombinieren Sie Funktionen
</h3>

Jede Erweiterung löst ein anderes Problem: CLAUDE.md behandelt immer aktivem Kontext, Skills behandeln On-Demand-Wissen und Workflows, MCP behandelt externe Verbindungen, Subagents behandeln Isolation und Hooks behandeln Automatisierung. Echte Setups kombinieren sie basierend auf Ihrem Workflow.

Beispielsweise könnten Sie CLAUDE.md für Projektkonventionen, einen Skill für Ihren Bereitstellungs-Workflow, MCP zur Verbindung mit Ihrer Datenbank und einen Hook zum Ausführen von Linting nach jeder Bearbeitung verwenden. Jede Funktion behandelt das, wofür sie am besten geeignet ist.

| Muster                 | Wie es funktioniert                                                                             | Beispiel                                                                                                  |
| ---------------------- | ----------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| **Skill + MCP**        | MCP bietet die Verbindung; ein Skill lehrt Claude, sie gut zu nutzen                            | MCP verbindet sich mit Ihrer Datenbank, ein Skill dokumentiert Ihr Schema und Abfragemuster               |
| **Skill + Subagent**   | Ein Skill spawnt Subagents für parallele Arbeit                                                 | `/audit`-Skill startet Sicherheits-, Leistungs- und Style-Subagents, die in isoliertem Kontext arbeiten   |
| **CLAUDE.md + Skills** | CLAUDE.md hält immer aktivem Regeln; Skills halten Referenzmaterial, das On Demand geladen wird | CLAUDE.md sagt „folgen Sie unseren API-Konventionen", ein Skill enthält das vollständige API-Stilhandbuch |
| **Hook + MCP**         | Ein Hook löst externe Aktionen über MCP aus                                                     | Post-Edit-Hook sendet eine Slack-Benachrichtigung, wenn Claude kritische Dateien ändert                   |

<h2 id="understand-context-costs">
  Verstehen Sie Kontextkosten
</h2>

Jede Funktion, die Sie hinzufügen, verbraucht etwas von Claudes Kontext. Zu viel kann Ihr Kontextfenster füllen, aber es kann auch Rauschen hinzufügen, das Claude weniger effektiv macht; Skills werden möglicherweise nicht korrekt ausgelöst, oder Claude kann Ihre Konventionen aus den Augen verlieren. Das Verständnis dieser Kompromisse hilft Ihnen, ein effektives Setup zu erstellen. Für eine interaktive Ansicht, wie diese Funktionen in einer laufenden Sitzung kombiniert werden, siehe [Erkunden Sie das Kontextfenster](/docs/de/context-window).

<h3 id="context-cost-by-feature">
  Kontextkosten nach Funktion
</h3>

Jede Funktion hat eine andere Ladestrategie und Kontextkosten:

| Funktion             | Wann sie lädt                          | Was lädt                                                       | Kontextkosten                                            |
| -------------------- | -------------------------------------- | -------------------------------------------------------------- | -------------------------------------------------------- |
| **CLAUDE.md**        | Sitzungsstart                          | Vollständiger Inhalt                                           | Jede Anfrage                                             |
| **Skills**           | Sitzungsstart + wenn verwendet         | Beschreibungen beim Start, vollständiger Inhalt bei Verwendung | Niedrig (Beschreibungen jede Anfrage)\*                  |
| **MCP-Server**       | Sitzungsstart                          | Tool-Namen; vollständige Schemas bei Bedarf                    | Niedrig bis ein Tool verwendet wird                      |
| **Code-Intelligenz** | Nach Dateibearbeitungen und bei Bedarf | Diagnosen nach Bearbeitungen; Symbol-Positionen bei Suche      | Niedrig; reduziert Dateileser anderswo                   |
| **Subagents**        | Wenn gespawnt                          | Frischer Kontext mit angegebenen Skills                        | Isoliert von Hauptsitzung                                |
| **Hooks**            | Bei Auslösung                          | Nichts (läuft extern)                                          | Null, es sei denn, Hook gibt zusätzlichen Kontext zurück |

\*Standardmäßig werden Skill-Beschreibungen beim Sitzungsstart geladen, damit Claude entscheiden kann, wann sie verwendet werden. Setzen Sie `disable-model-invocation: true` in das Frontmatter eines Skills, um es vollständig vor Claude zu verbergen, bis Sie es manuell aufrufen. Dies reduziert die Kontextkosten auf Null für Skills, die Sie nur selbst auslösen. Für einen Skill, den Sie nicht geschrieben haben, setzen Sie [`skillOverrides`](/docs/de/skills#override-skill-visibility-from-settings) in den Einstellungen, um dasselbe zu tun, ohne die Datei zu bearbeiten.

<h3 id="understand-how-features-load">
  Verstehen Sie, wie Funktionen geladen werden
</h3>

Jede Funktion wird an verschiedenen Punkten in Ihrer Sitzung geladen. Die Registerkarten unten erklären, wann jede geladen wird und was in den Kontext geht.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/context-loading.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=aab139e750494a237ae2e0c8f9139b0a" alt="Kontextladung: CLAUDE.md lädt beim Sitzungsstart und bleibt in jeder Anfrage. MCP-Tool-Namen laden beim Start mit vollständigen Schemas, die bis zur Verwendung aufgeschoben werden. Skills laden Beschreibungen beim Start, vollständigen Inhalt bei Aufruf. Subagents erhalten isolierten Kontext. Hooks laufen extern." width="720" height="382" data-path="images/context-loading.svg" />

<Tabs>
  <Tab title="CLAUDE.md">
    **Wann:** Sitzungsstart

    **Was lädt:** Vollständiger Inhalt aller CLAUDE.md-Dateien (verwaltet, Benutzer und Projektebenen).

    **Vererbung:** Claude liest CLAUDE.md-Dateien aus Ihrem Arbeitsverzeichnis bis zur Wurzel und entdeckt verschachtelte in Unterverzeichnissen, wenn es auf diese Dateien zugreift. Weitere Informationen finden Sie unter [Wie CLAUDE.md-Dateien geladen werden](/docs/de/memory#how-claude-md-files-load).

    <Tip>Halten Sie CLAUDE.md unter 200 Zeilen. Verschieben Sie Referenzmaterial zu Skills, die On-Demand geladen werden.</Tip>
  </Tab>

  <Tab title="Skills">
    Skills sind zusätzliche Funktionen in Claudes Toolkit. Sie können Referenzmaterial sein (wie ein API-Stilhandbuch) oder aufrufbare Workflows, die Sie mit `/<name>` auslösen (wie `/deploy`). Claude Code wird mit [gebündelten Skills](/docs/de/commands) wie `/code-review`, `/batch` und `/debug` ausgeliefert, die sofort funktionieren. Sie können auch Ihre eigenen erstellen. Claude verwendet Skills, wenn angemessen, oder Sie können einen direkt aufrufen.

    **Wann:** Hängt von der Konfiguration des Skills ab. Standardmäßig werden Beschreibungen beim Sitzungsstart geladen und vollständiger Inhalt bei Verwendung. Für nur-Benutzer-Skills (`disable-model-invocation: true`) wird nichts geladen, bis Sie sie aufrufen.

    **Was lädt:** Für modell-aufrufbare Skills sieht Claude Namen und Beschreibungen in jeder Anfrage. Wenn Sie einen Skill mit `/<name>` aufrufen oder Claude ihn automatisch lädt, wird der vollständige Inhalt in Ihre Konversation geladen.

    **Wie Claude Skills wählt:** Claude gleicht Ihre Aufgabe gegen Skill-Beschreibungen ab, um zu entscheiden, welche relevant sind. Wenn Beschreibungen vage oder überlappend sind, kann Claude den falschen Skill laden oder einen verpassen, der helfen würde. Um Claude zu sagen, einen bestimmten Skill zu verwenden, rufen Sie ihn mit `/<name>` auf. Skills mit `disable-model-invocation: true` sind für Claude unsichtbar, bis Sie sie aufrufen.

    **Kontextkosten:** Niedrig bis verwendet. Nur-Benutzer-Skills haben Null-Kosten bis aufgerufen.

    **In Subagents:** Skills funktionieren in Subagents anders. Anstelle von On-Demand-Laden werden Skills, die im `skills:`-Feld des Subagenten aufgelistet sind, vollständig in seinen Kontext beim Start vorgeladen. Subagents können immer noch unlisted Project-, Benutzer- und Plugin-Skills durch das Skill-Tool entdecken und aufrufen.

    <Tip>Verwenden Sie `disable-model-invocation: true` für Skills mit Nebenwirkungen. Dies spart Kontext und stellt sicher, dass nur Sie sie auslösen.</Tip>
  </Tab>

  <Tab title="MCP-Server">
    **Wann:** Sitzungsstart.

    **Was lädt:** Tool-Namen von verbundenen Servern. Vollständige JSON-Schemas bleiben aufgeschoben, bis Claude ein bestimmtes Tool benötigt.

    **Kontextkosten:** [Tool-Suche](/docs/de/mcp#scale-with-mcp-tool-search) ist standardmäßig aktiviert, sodass untätige MCP-Tools minimalen Kontext verbrauchen.

    <Tip>Führen Sie `/mcp` aus, um Verbindungsstatus und Token-Kosten pro Server zu sehen. Claude Code [verbindet sich automatisch wieder mit Remote-Servern](/docs/de/mcp#automatic-reconnection), wenn diese ausfallen, und Sie können Server trennen, die Sie nicht aktiv verwenden.</Tip>
  </Tab>

  <Tab title="Code-Intelligenz">
    **Wann:** Nach Dateibearbeitungen und bei Bedarf, wenn Claude Code navigiert.

    **Was lädt:** Typfehler und Warnungen nach jeder Dateibearbeitung. Definitions-, Referenz- und Typinformationen, wenn Claude ein Symbol nachschlägt.

    **Kontextkosten:** Niedrig. Symbol-Suchen ersetzen oft umfangreiche Dateileser, sodass die Netto-Kontextnutzung sinken kann.

    <Tip>Das LSP-Tool ist inaktiv, bis Sie ein [Code-Intelligenz-Plugin](/docs/de/discover-plugins#code-intelligence) für Ihre Sprache installieren.</Tip>
  </Tab>

  <Tab title="Subagents">
    **Wann:** Bei Bedarf, wenn Sie oder Claude einen für eine Aufgabe spawnt.

    **Was lädt:** Frischer, isolierter Kontext, der Folgendes enthält:

    * Der Agent's eigener System-Prompt, nicht der vollständige Claude Code System-Prompt
    * Vollständiger Inhalt von Skills, die im `skills:`-Feld des Agenten aufgelistet sind
    * CLAUDE.md und Git-Status, außer die integrierten Explore- und Plan-Agenten [lassen beide weg](/docs/de/sub-agents#what-loads-at-startup)
    * Welcher Kontext auch immer der Lead-Agent im Prompt übergibt

    **Kontextkosten:** Isoliert von Hauptsitzung. Subagents erben Ihre Konversationshistorie oder aufgerufenen Skills nicht.

    <Tip>Verwenden Sie Subagents für Arbeit, die Ihren vollständigen Konversationskontext nicht benötigt. Ihre Isolation verhindert, dass Ihre Hauptsitzung aufgebläht wird.</Tip>
  </Tab>

  <Tab title="Hooks">
    **Wann:** Bei Auslösung. Hooks werden bei bestimmten Lebenszyklusereignissen ausgelöst, wie Tool-Ausführung, Sitzungsgrenzen, Prompt-Einreichung, Berechtigungsanfragen und Komprimierung. Siehe [Hooks](/docs/de/hooks) für die vollständige Liste.

    **Was lädt:** Standardmäßig nichts. Hooks laufen außerhalb der Hauptkonversation.

    **Kontextkosten:** Null, es sei denn, der Hook gibt Ausgabe zurück, die als Nachrichten zu Ihrer Konversation hinzugefügt wird.

    <Tip>Hooks sind ideal für Nebenwirkungen (Linting, Logging), die Claudes Kontext nicht beeinflussen müssen.</Tip>
  </Tab>
</Tabs>

<h2 id="learn-more">
  Weitere Informationen
</h2>

Jede Funktion hat ihr eigenes Handbuch mit Setup-Anweisungen, Beispielen und Konfigurationsoptionen.

<CardGroup cols={2}>
  <Card title="CLAUDE.md" icon="file-lines" href="/docs/de/memory">
    Speichern Sie Projektkontext, Konventionen und Anweisungen
  </Card>

  <Card title="Skills" icon="brain" href="/docs/de/skills">
    Geben Sie Claude Fachkompetenz und wiederverwendbare Workflows
  </Card>

  <Card title="Subagents" icon="users" href="/docs/de/sub-agents">
    Lagern Sie Arbeit in isoliertem Kontext aus
  </Card>

  <Card title="Agent teams" icon="network" href="/docs/de/agent-teams">
    Koordinieren Sie mehrere Sitzungen, die parallel arbeiten
  </Card>

  <Card title="MCP" icon="plug" href="/docs/de/mcp">
    Verbinden Sie Claude mit externen Diensten
  </Card>

  <Card title="Hooks" icon="bolt" href="/docs/de/hooks-guide">
    Automatisieren Sie Workflows mit Hooks
  </Card>

  <Card title="Plugins" icon="puzzle-piece" href="/docs/de/plugins">
    Bündeln und teilen Sie Feature-Sets
  </Card>

  <Card title="Marketplaces" icon="store" href="/docs/de/plugin-marketplaces">
    Hosten und verteilen Sie Plugin-Sammlungen
  </Card>
</CardGroup>
