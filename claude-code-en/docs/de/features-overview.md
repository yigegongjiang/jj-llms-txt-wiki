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
* **[Ausgabestile](/docs/de/output-styles)** legen Claudes Rolle, Ton und Antwortformat für jede Antwort in einer Sitzung fest
* **[Skills](/docs/de/skills)** fügen wiederverwendbares Wissen und aufrufbare Workflows hinzu
* **[Code-Intelligenz](/docs/de/tools-reference#lsp-tool-behavior)** verbindet Claude mit einem Language Server für Symbol-Navigation und Live-Typfehler
* **[MCP](/docs/de/mcp)** verbindet Claude mit externen Diensten und Tools
* **[Subagents](/docs/de/sub-agents)** führen ihre eigenen Schleifen in isoliertem Kontext aus und geben Zusammenfassungen zurück
* **[Dynamische Workflows](/docs/de/workflows)** führen viele Subagents aus einem Skript aus, das Claude schreibt, und geben ein Ergebnis zurück
* **[Sitzungsübergreifendes Messaging](/docs/de/cross-session-messaging)** ermöglicht es Claude, eine Nachricht von einer Ihrer Sitzungen an eine andere zu übergeben
* **[Hooks](/docs/de/hooks-guide)** führen Ihr Skript, eine HTTP-Anfrage, einen MCP-Tool-Aufruf, einen Prompt oder einen Subagent aus, wenn Claude Code ein Lebenszyklusereignis erreicht
* **[Plugins](/docs/de/plugins/overview)** und **[Marketplaces](/docs/de/plugins/overview)** verpacken und verteilen diese Funktionen

[Skills](/docs/de/skills) sind die flexibelste Erweiterung. Ein Skill ist eine Markdown-Datei, die Wissen, Workflows oder Anweisungen enthält. Sie können Skills mit einem Befehl wie `/deploy` aufrufen, oder Claude kann sie automatisch laden, wenn sie relevant sind. Skills können in Ihrer aktuellen Konversation oder in einem isolierten Kontext über Subagents ausgeführt werden.

<h2 id="match-features-to-your-goal">
  Funktionen an Ihr Ziel anpassen
</h2>

Funktionen reichen von immer aktivem Kontext, den Claude in jeder Sitzung sieht, über On-Demand-Funktionen, die Sie oder Claude aufrufen können, bis hin zu Hintergrundautomatisierung, die bei bestimmten Ereignissen ausgeführt wird. Die folgende Tabelle zeigt, was verfügbar ist und wann jede Funktion sinnvoll ist.

| Funktion                                                            | Was sie tut                                                                             | Wann man sie nutzt                                                                                                                 | Beispiel                                                                                                                               |
| ------------------------------------------------------------------- | --------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| **CLAUDE.md**                                                       | Persistenter Kontext, der in jedem Gespräch geladen wird                                | Projektkonventionen, „immer X tun"-Regeln                                                                                          | „Verwenden Sie pnpm, nicht npm. Führen Sie Tests vor dem Commit aus."                                                                  |
| **[Ausgabestil](/docs/de/output-styles)**                                | Anweisungen, die Claudes Rolle, Ton und Antwortformat für eine ganze Sitzung festlegen  | Eine Stimme, Länge oder Format, das Sie in jeder Antwort möchten, oder Claude arbeitet als etwas anderes als ein Softwareingenieur | Der integrierte Concise-Stil für kürzere Antworten; ein benutzerdefinierter Stil, der jede Frage zuerst mit einem Diagramm beantwortet |
| **Skill**                                                           | Anweisungen, Wissen und Workflows, die Claude nutzen kann                               | Wiederverwendbarer Inhalt, Referenzdokumente, wiederholbare Aufgaben                                                               | `/deploy` führt Ihre Bereitstellungs-Checkliste aus; API-Docs-Skill mit Endpunkt-Mustern                                               |
| **Subagent**                                                        | Isolierter Ausführungskontext, der zusammengefasste Ergebnisse zurückgibt               | Kontextisolation, parallele Aufgaben, spezialisierte Worker                                                                        | Recherche-Aufgabe, die viele Dateien liest, aber nur wichtige Erkenntnisse zurückgibt                                                  |
| **[Dynamischer Workflow](/docs/de/workflows)**                           | Skript, das Claude schreibt und das viele Subagents im Hintergrund ausführt             | Arbeit, die über eine Handvoll Subagents hinauswächst, oder Erkenntnisse, die Sie überprüft haben möchten                          | Audit eines ganzen Codebase, wobei ein zweiter Satz von Agenten jede Erkenntnis überprüft                                              |
| **[Sitzungsübergreifendes Messaging](/docs/de/cross-session-messaging)** | Claude liefert eine Nachricht von einer Ihrer Sitzungen zu einer anderen                | Sitzungen, die Sie selbst ausführen und die gegenseitig ihre Erkenntnisse während der Aufgabe benötigen                            | Eine Sitzung warnt eine andere, dass eine Änderung, die sie vorgenommen hat, das bricht, worauf die andere aufbaut                     |
| **[Code-Intelligenz](/docs/de/tools-reference#lsp-tool-behavior)**       | Language-Server-Navigation und Diagnose                                                 | Typisierte Sprachen, große Codebases, bei denen grep langsam oder ungenau ist                                                      | Springen Sie zur Definition eines Symbols, anstatt die ganze Datei zu lesen                                                            |
| **MCP**                                                             | Verbindung zu externen Diensten                                                         | Externe Daten oder Aktionen                                                                                                        | Abfrage Ihrer Datenbank, Posten auf Slack, Steuerung eines Browsers                                                                    |
| **Hook**                                                            | Skript, HTTP-Anfrage, MCP-Tool-Aufruf, Prompt oder Subagent, ausgelöst durch Ereignisse | Automatisierung, die bei jedem übereinstimmenden Ereignis ausgeführt werden muss                                                   | ESLint nach jeder Dateibearbeitung ausführen                                                                                           |
| **[Artefakt](/docs/de/artifacts)**                                       | Veröffentlichen Sie die Sitzungsausgabe als private, interaktive Webseite               | Ausgabe, die Sie visuell sehen oder teilen möchten, anstatt als Terminaltext                                                       | Eine Incident-Timeline, die sich aktualisiert, während Claude untersucht                                                               |

**[Plugins](/docs/de/plugins/overview)** sind die Verpackungsebene. Ein Plugin bündelt Skills, Hooks, Subagents und MCP-Server in eine einzelne installierbare Einheit. Plugin-Skills sind namespaced (wie `/my-plugin:review`), sodass mehrere Plugins nebeneinander existieren können. Verwenden Sie Plugins, wenn Sie dieselbe Einrichtung über mehrere Repositories hinweg wiederverwenden möchten oder über einen **[Marketplace](/docs/de/plugins/overview)** an andere verteilen möchten.

<h3 id="build-your-setup-over-time">
  Bauen Sie Ihre Einrichtung im Laufe der Zeit auf
</h3>

Sie müssen nicht alles im Voraus konfigurieren. Jede Funktion hat einen erkennbaren Auslöser, und die meisten Teams fügen sie ungefähr in dieser Reihenfolge hinzu:

| Auslöser                                                                                         | Hinzufügen                                                                                      |
| :----------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------- |
| Claude bekommt eine Konvention oder einen Befehl zweimal falsch                                  | Fügen Sie es zu [CLAUDE.md](/docs/de/memory) hinzu                                                   |
| Sie bitten Claude ständig, kürzer zu sein, mehr zu erklären oder im gleichen Format zu antworten | Legen Sie einen [Ausgabestil](/docs/de/output-styles) fest                                           |
| Sie tippen ständig denselben Prompt, um eine Aufgabe zu starten                                  | Speichern Sie ihn als benutzer-aufrufbaren [Skill](/docs/de/skills)                                  |
| Sie fügen zum dritten Mal dasselbe Playbook oder mehrstufige Verfahren in den Chat ein           | Erfassen Sie es als [Skill](/docs/de/skills)                                                         |
| Sie kopieren ständig Daten aus einer Browser-Registerkarte, die Claude nicht sehen kann          | Verbinden Sie dieses System als [MCP-Server](/docs/de/mcp)                                           |
| Claude liest viele Dateien, um zu finden, wo ein Symbol definiert oder verwendet wird            | Installieren Sie ein [Code-Intelligence-Plugin](/docs/de/plugins/code-intelligence) für Ihre Sprache |
| Eine Nebenaufgabe überschwemmt Ihr Gespräch mit Ausgabe, auf die Sie nicht mehr verweisen werden | Leiten Sie es durch einen [Subagent](/docs/de/sub-agents)                                            |
| Sie möchten, dass etwas jedes Mal passiert, ohne zu fragen                                       | Schreiben Sie einen [Hook](/docs/de/hooks-guide)                                                     |
| Ein zweites Repository benötigt dieselbe Einrichtung                                             | Verpacken Sie es als [Plugin](/docs/de/plugins/overview)                                             |

Die gleichen Auslöser sagen Ihnen, wann Sie das aktualisieren sollten, was Sie bereits haben. Ein wiederholter Fehler oder ein wiederkehrender Review-Kommentar ist eine CLAUDE.md-Bearbeitung, keine einmalige Korrektur im Chat. Ein Workflow, den Sie ständig von Hand anpassen, ist ein Skill, der eine weitere Überarbeitung benötigt.

<h3 id="compare-similar-features">
  Vergleichen Sie ähnliche Funktionen
</h3>

Einige Funktionen können ähnlich wirken. Für eine tiefere Anleitung zur Auswahl zwischen ihnen siehe [Steering Claude Code: when to use CLAUDE.md, skills, hooks, and subagents](https://claude.com/blog/steering-claude-code-skills-hooks-rules-subagents-and-more) auf dem Blog. So unterscheiden Sie sie.

<Tabs>
  <Tab title="Skill vs Subagent">
    Skills und Subagents lösen unterschiedliche Probleme:

    * **Skills** sind wiederverwendbarer Inhalt, den Sie in jeden Kontext laden können
    * **Subagents** sind isolierte Worker, die separat von Ihrem Hauptgespräch ausgeführt werden

    | Aspekt                                              | Skill                                                | Subagent                                                                               |
    | --------------------------------------------------- | ---------------------------------------------------- | -------------------------------------------------------------------------------------- |
    | **Was es ist**                                      | Wiederverwendbare Anweisungen, Wissen oder Workflows | Isolierter Worker mit eigenem Kontext                                                  |
    | **Hauptvorteil**                                    | Inhalte über Kontexte hinweg teilen                  | Kontextisolation. Die Arbeit läuft separat, nur die Zusammenfassung wird zurückgegeben |
    | **[Kontextfenster](/docs/de/context-window)-Auswirkung** | Fügt zu Ihrem Hauptfenster hinzu                     | Verwendet ein separates Fenster mit eigenen Input- und Output-Tokens                   |
    | **Am besten für**                                   | Referenzmaterial, aufrufbare Workflows               | Aufgaben, die viele Dateien lesen, parallele Arbeit, spezialisierte Worker             |

    **Skills können Referenz oder Aktion sein.** Referenz-Skills bieten Wissen, das Claude während Ihrer Sitzung nutzt (wie Ihr API-Stilhandbuch). Action-Skills sagen Claude, etwas Bestimmtes zu tun (wie `/deploy`, das Ihren Bereitstellungs-Workflow ausführt).

    **Verwenden Sie einen Subagent**, wenn Sie Kontextisolation benötigen oder wenn Ihr Kontextfenster voll wird. Der Subagent könnte Dutzende von Dateien lesen oder umfangreiche Suchen durchführen, aber Ihr Hauptgespräch erhält nur eine Zusammenfassung. Da die Subagent-Arbeit Ihren Hauptkontext nicht verbraucht, ist dies auch nützlich, wenn Sie nicht möchten, dass die Zwischenarbeit sichtbar bleibt. Benutzerdefinierte Subagents können ihre eigenen Anweisungen haben und Skills vorladen.

    **Sie können sich kombinieren.** Ein Subagent kann spezifische Skills vorladen (`skills:`-Feld). Ein Skill kann in isoliertem Kontext mit `context: fork` ausgeführt werden. Siehe [Skills](/docs/de/skills) für Details.
  </Tab>

  <Tab title="CLAUDE.md vs Skill">
    Beide speichern Anweisungen, aber sie laden unterschiedlich und dienen unterschiedlichen Zwecken.

    | Aspekt                        | CLAUDE.md                 | Skill                                  |
    | ----------------------------- | ------------------------- | -------------------------------------- |
    | **Lädt**                      | Jede Sitzung, automatisch | On Demand                              |
    | **Kann Dateien einschließen** | Ja, mit `@path`-Importen  | Ja, mit `@path`-Importen               |
    | **Kann Workflows auslösen**   | Nein                      | Ja, mit `/<name>`                      |
    | **Am besten für**             | „Immer X tun"-Regeln      | Referenzmaterial, aufrufbare Workflows |

    **Legen Sie es in CLAUDE.md** ab, wenn Claude es immer wissen sollte: Codierungskonventionen, Build-Befehle, Projektstruktur, „niemals X tun"-Regeln.

    **Legen Sie es in einen Skill**, wenn es Referenzmaterial ist, das Claude manchmal benötigt (API-Docs, Stilhandbücher) oder ein Workflow, den Sie mit `/<name>` auslösen (bereitstellen, überprüfen, freigeben).

    **Faustregel:** Halten Sie CLAUDE.md unter 200 Zeilen. Wenn es wächst, verschieben Sie Referenzinhalte zu Skills oder teilen Sie sie in [`.claude/rules/`](/docs/de/memory#organize-rules-with-claude/rules/)-Dateien auf.
  </Tab>

  <Tab title="CLAUDE.md vs Ausgabestil">
    Beide geben Claude stehende Anweisungen. CLAUDE.md trägt das, was Claude wissen sollte, und ein Ausgabestil legt fest, wie Claude antwortet.

    | Aspekt            | CLAUDE.md                                           | Ausgabestil                                                                                                     |
    | ----------------- | --------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
    | **Hält**          | Fakten und Regeln über Ihr Projekt                  | Eine Rolle, einen Ton und ein Antwortformat                                                                     |
    | **Wechsel**       | Immer geladen                                       | Einer aktiv auf einmal; [Wechseln Sie Stile](/docs/de/output-styles#change-your-output-style) wann immer Sie möchten |
    | **Am besten für** | Build-Befehle, Konventionen, „niemals X tun"-Regeln | Kürzere Antworten, Erklärungen neben Code, eine nicht-technische Rolle                                          |

    **Legen Sie es in CLAUDE.md** ab, wenn es unabhängig vom Stil wahr ist: Codierungskonventionen, Build-Befehle, Projektstruktur.

    **Verwenden Sie einen Ausgabestil**, wenn es um die Antwort selbst geht und Sie ihn möglicherweise wieder ausschalten möchten: Länge, Format, wie viel Claude erklärt, oder eine andere Rolle wie ein Schreibassistent. Claude Code enthält [integrierte Stile](/docs/de/output-styles#built-in-output-styles), und Sie können Ihre eigenen schreiben.

    **Sie kombinieren sich.** CLAUDE.md bleibt geladen, welchen Stil Sie auch wählen. Claude folgt beiden als Anweisungen, also wird keiner erzwungen. Für alles, das jedes Mal passieren muss, verwenden Sie einen [Hook](/docs/de/hooks-guide).
  </Tab>

  <Tab title="CLAUDE.md vs Regeln vs Skills">
    Alle drei speichern Anweisungen, aber sie laden unterschiedlich:

    | Aspekt            | CLAUDE.md                          | `.claude/rules/`                                                 | Skill                                     |
    | ----------------- | ---------------------------------- | ---------------------------------------------------------------- | ----------------------------------------- |
    | **Lädt**          | Jede Sitzung                       | Jede Sitzung, oder wenn übereinstimmende Dateien geöffnet werden | On Demand, wenn aufgerufen oder relevant  |
    | **Umfang**        | Ganzes Projekt                     | Kann auf Dateipfade begrenzt werden                              | Aufgabenspezifisch                        |
    | **Am besten für** | Kernkonventionen und Build-Befehle | Sprachspezifische oder verzeichnisspezifische Richtlinien        | Referenzmaterial, wiederholbare Workflows |

    **Verwenden Sie CLAUDE.md** für Anweisungen, die jede Sitzung benötigt: Build-Befehle, Test-Konventionen, Projektarchitektur.

    **Verwenden Sie Regeln**, um CLAUDE.md fokussiert zu halten. Regeln mit [`paths`-Frontmatter](/docs/de/memory#path-specific-rules) laden nur, wenn Claude mit übereinstimmenden Dateien arbeitet, was Kontext spart.

    **Verwenden Sie Skills** für Inhalte, die Claude nur manchmal benötigt, wie API-Dokumentation oder eine Bereitstellungs-Checkliste, die Sie mit `/<name>` auslösen.
  </Tab>

  <Tab title="Subagent vs Dynamischer Workflow">
    Beide führen Arbeit außerhalb Ihres Hauptgesprächs aus. Bei Subagents entscheidet Claude Zug um Zug, was als nächstes ausgeführt wird. In einem Workflow entscheidet das Skript:

    * **Subagents** sind Worker, die Claude spawnt, jeder gibt eine Zusammenfassung an das Gespräch zurück, das ihn spawnt
    * **[Dynamische Workflows](/docs/de/workflows)** sind Skripte, die Claude schreibt und die viele Subagents im Hintergrund ausführen und ein Ergebnis zurückgeben

    **Verwenden Sie einen Subagent**, wenn Sie einen schnellen, fokussierten Worker benötigen: eine Frage recherchieren, einen Anspruch überprüfen, eine Datei überprüfen. Der Subagent führt die Arbeit aus und gibt eine Zusammenfassung zurück, sodass Ihr Hauptgespräch sauber bleibt. Subagents, die Claude benannt hat, als es sie spawnte, können auch [sich gegenseitig Nachrichten senden](/docs/de/sub-agents#what-loads-at-startup).

    **Verwenden Sie einen dynamischen Workflow**, wenn ein Job [über eine Handvoll Subagents hinauswächst](/docs/de/workflows#when-to-use-a-workflow), oder wenn Sie möchten, dass die Erkenntnisse überprüft werden, bevor Sie sie sehen, wie ein codebase-weites Audit, eine große Migration oder ein Plan, der aus mehreren Blickwinkeln entworfen wurde. Um einen zu starten, [fragen Sie in Ihrem Prompt nach einem Workflow](/docs/de/workflows#ask-for-a-workflow-in-your-prompt).

    **Um eine Erkenntnis von einer Ihrer Sitzungen zu einer anderen zu übergeben**, bitten Sie Claudes erste Sitzung, sie zu senden. Claude liefert sie mit [sitzungsübergreifendem Messaging](/docs/de/cross-session-messaging). [Führen Sie Agenten parallel aus](/docs/de/agents) vergleicht die anderen Möglichkeiten, mehr als einen Claude auf einmal auszuführen, einschließlich Sitzungen, die Sie übergeben und später überprüfen.
  </Tab>

  <Tab title="MCP vs Skill">
    MCP verbindet Claude mit externen Diensten. Skills erweitern das, was Claude weiß, einschließlich wie man diese Dienste effektiv nutzt.

    | Aspekt         | MCP                                                     | Skill                                                              |
    | -------------- | ------------------------------------------------------- | ------------------------------------------------------------------ |
    | **Was es ist** | Protokoll zur Verbindung mit externen Diensten          | Wissen, Workflows und Referenzmaterial                             |
    | **Bietet**     | Tools und Datenzugriff                                  | Wissen, Workflows, Referenzmaterial                                |
    | **Beispiele**  | Slack-Integration, Datenbankabfragen, Browser-Steuerung | Code-Review-Checkliste, Bereitstellungs-Workflow, API-Stilhandbuch |

    Diese lösen unterschiedliche Probleme und funktionieren gut zusammen:

    **MCP** gibt Claude speziell entwickelte Tools für ein externes System, wobei die Verbindung und Authentifizierung vom Server behandelt werden.

    **Skills** geben Claude Wissen darüber, wie man diese Tools effektiv nutzt, plus Workflows, die Sie mit `/<name>` auslösen können. Ein Skill könnte Ihr Team-Datenbankschema und Abfragemuster enthalten, oder einen `/post-to-slack`-Workflow mit Ihren Team-Nachrichtenformatierungsregeln.
  </Tab>

  <Tab title="Hook vs Skill">
    Claude Code führt einen Hook bei einem Lebenszyklusereignis aus; es lädt einen Skill in den Kontext, damit Claude ihn anwendet.

    | Aspekt              | Hook                                                                                           | Skill                                                                        |
    | ------------------- | ---------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
    | **Führt aus**       | Ein Shell-Befehl, HTTP-Anfrage, MCP-Tool-Aufruf, LLM-Prompt oder Subagent                      | Anweisungen, die Claude liest und befolgt                                    |
    | **Ausgelöst durch** | [Lebenszyklusereignisse](/docs/de/hooks#hook-events) wie `PostToolUse` oder `SessionStart`          | Sie tippen `/<name>`, oder Claude passt die Beschreibung zu Ihrer Aufgabe an |
    | **Determinismus**   | Wird immer bei seinem Ereignis ausgelöst; der Auslöser ist garantiert                          | Claude interpretiert die Anweisungen; das Ergebnis kann variieren            |
    | **Kontextkosten**   | Null, es sei denn, der Hook gibt Ausgabe zurück                                                | Beschreibung lädt jede Sitzung; vollständiger Inhalt lädt bei Verwendung     |
    | **Am besten für**   | Linting nach Bearbeitungen, Blockieren unsicherer Befehle, Protokollierung, Benachrichtigungen | Workflows, die Überlegung benötigen, Referenzmaterial, mehrstufige Aufgaben  |

    **Verwenden Sie einen Hook**, wenn die Aktion jedes Mal auf die gleiche Weise passieren muss und Claude nicht denken muss. Zum Beispiel: Formatierung beim Speichern, Ablehnung von `rm -rf /`, Posten einer Slack-Nachricht, wenn eine Sitzung endet.

    **Verwenden Sie einen Skill**, wenn Claude entscheiden sollte, wie die Schritte angewendet werden, oder wenn der Inhalt Wissen statt ein Skript ist. Zum Beispiel: eine `/release`-Checkliste, Ihr API-Stilhandbuch, ein Debugging-Playbook.

    **Legen Sie Schutzmaßnahmen in Hooks.** Eine Anweisung wie „niemals `.env` bearbeiten" in CLAUDE.md oder einem Skill ist eine Anfrage, keine Garantie. Ein `PreToolUse`-Hook, der die Bearbeitung blockiert, ist Durchsetzung. Wenn eine Regel jedes Mal gelten muss, machen Sie sie zu einem Hook statt zu einer Prompt-Anweisung.

    **Hook-Ausgabe landet im Kontext.** Ein `PostToolUse`-Hook, der Ihren Linter ausführt, gibt Ergebnisse als Text zurück, den Claude liest; ein `/fix-lint`-Skill sagt Claude, wie man sie löst.
  </Tab>
</Tabs>

<h3 id="understand-how-features-layer">
  Verstehen Sie, wie Funktionen sich schichten
</h3>

Funktionen können auf mehreren Ebenen definiert werden: benutzerübergreifend, pro Projekt, über Plugins oder durch verwaltete Richtlinien. Sie können auch CLAUDE.md-Dateien in Unterverzeichnissen verschachteln oder Skills in bestimmten Paketen eines Monorepo platzieren. Wenn die gleiche Funktion auf mehreren Ebenen existiert, so schichten sie sich:

* **CLAUDE.md-Dateien** sind additiv: alle Ebenen tragen Inhalte gleichzeitig zu Claudes Kontext bei. Dateien aus Ihrem Arbeitsverzeichnis und darüber laden beim Start; Unterverzeichnisse laden, während Sie in ihnen arbeiten. Wenn Anweisungen in Konflikt geraten, nutzt Claude Urteilsvermögen, um sie zu versöhnen. Siehe [wie CLAUDE.md-Dateien laden](/docs/de/memory#how-claude-md-files-load).
* **Skills und Subagents** überschreiben nach Name: wenn der gleiche Name auf mehreren Ebenen existiert, gewinnt eine Definition basierend auf Priorität (verwaltet > Benutzer > Projekt für Skills; verwaltet > CLI-Flag > Projekt > Benutzer > Plugin für Subagents). Plugin-Skills sind [namespaced](/docs/de/plugins/components#skills) um Konflikte zu vermeiden. Siehe [Skill-Erkennung](/docs/de/skills#resolve-skills-that-share-a-name) und [Subagent-Umfang](/docs/de/sub-agents#choose-the-subagent-scope).
* **MCP-Server** überschreiben nach Name: lokal > Projekt > Benutzer. Siehe [MCP-Umfang](/docs/de/mcp#scope-hierarchy-and-precedence).
* **Hooks** zusammenführen: alle registrierten Hooks werden für ihre übereinstimmenden Ereignisse unabhängig von der Quelle ausgelöst. Siehe [Hooks](/docs/de/hooks).

<h3 id="combine-features">
  Kombinieren Sie Funktionen
</h3>

Jede Erweiterung löst ein anderes Problem: CLAUDE.md behandelt immer-aktiven Kontext, Skills behandeln On-Demand-Wissen und Workflows, MCP behandelt externe Verbindungen, Subagents behandeln Isolation, und Hooks behandeln Automatisierung. Echte Setups kombinieren sie basierend auf Ihrem Workflow.

Zum Beispiel könnten Sie CLAUDE.md für Projektkonventionen verwenden, einen Skill für Ihren Bereitstellungs-Workflow, MCP zur Verbindung mit Ihrer Datenbank und einen Hook zum Ausführen von Linting nach jeder Bearbeitung. Jede Funktion behandelt das, wofür sie am besten geeignet ist.

| Muster                 | Wie es funktioniert                                                                            | Beispiel                                                                                                  |
| ---------------------- | ---------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| **Skill + MCP**        | MCP bietet die Verbindung; ein Skill lehrt Claude, sie gut zu nutzen                           | MCP verbindet sich mit Ihrer Datenbank, ein Skill dokumentiert Ihr Schema und Abfragemuster               |
| **Skill + Subagent**   | Ein Skill spawnt Subagents für parallele Arbeit                                                | `/audit`-Skill startet Sicherheits-, Leistungs- und Style-Subagents, die in isoliertem Kontext arbeiten   |
| **CLAUDE.md + Skills** | CLAUDE.md hält immer-aktive Regeln; Skills halten Referenzmaterial, das On Demand geladen wird | CLAUDE.md sagt „folgen Sie unseren API-Konventionen", ein Skill enthält das vollständige API-Stilhandbuch |
| **Hook + MCP**         | Ein Hook löst externe Aktionen durch MCP aus                                                   | Post-Edit-Hook sendet eine Slack-Benachrichtigung, wenn Claude kritische Dateien ändert                   |

<h2 id="understand-context-costs">
  Verstehen Sie Kontextkosten
</h2>

Jede Funktion, die Sie hinzufügen, verbraucht etwas von Claudes Kontext. Zu viel kann Ihr Kontextfenster füllen, aber es kann auch Rauschen hinzufügen, das Claude weniger effektiv macht; Skills werden möglicherweise nicht korrekt ausgelöst, oder Claude kann Ihre Konventionen aus den Augen verlieren. Das Verständnis dieser Kompromisse hilft Ihnen, ein effektives Setup zu erstellen. Für eine interaktive Ansicht, wie diese Funktionen in einer laufenden Sitzung kombiniert werden, siehe [Erkunden Sie das Kontextfenster](/docs/de/context-window).

<h3 id="context-cost-by-feature">
  Kontextkosten nach Funktion
</h3>

Jede Funktion hat eine andere Ladestrategie und Kontextkosten:

| Funktion             | Wann sie lädt                                     | Was lädt                                                                                                                                   | Kontextkosten                                            |
| -------------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------- |
| **CLAUDE.md**        | Sitzungsstart                                     | Vollständiger Inhalt                                                                                                                       | Jede Anfrage                                             |
| **Output-Stile**     | Sitzungsstart und erneut, wenn Sie Stile wechseln | Die vollständigen Anweisungen des aktiven Stils; nichts für den Standardstil                                                               | Jede Anfrage                                             |
| **Skills**           | Sitzungsstart + wenn verwendet                    | Beschreibungen beim Start, vollständiger Inhalt bei Verwendung                                                                             | Niedrig (Beschreibungen jede Anfrage)\*                  |
| **MCP-Server**       | Sitzungsstart                                     | Tool-Namen; vollständige Schemas bei Bedarf                                                                                                | Niedrig bis ein Tool verwendet wird                      |
| **Code-Intelligenz** | Nach Dateibearbeitungen und bei Bedarf            | Diagnosen nach Bearbeitungen; Symbol-Positionen bei Suche                                                                                  | Niedrig; reduziert Dateileser anderswo                   |
| **Subagents**        | Wenn gespawnt                                     | Frischer Kontext mit angegebenen Skills oder die übergeordnete Konversation für einen [Fork](/docs/de/sub-agents#fork-the-current-conversation) | Isoliert von Hauptsitzung                                |
| **Hooks**            | Bei Auslösung                                     | Nichts (läuft extern)                                                                                                                      | Null, es sei denn, Hook gibt zusätzlichen Kontext zurück |

\*Standardmäßig werden Skill-Beschreibungen beim Sitzungsstart geladen, damit Claude entscheiden kann, wann sie verwendet werden. Setzen Sie `disable-model-invocation: true` in das Frontmatter eines Skills, um es vollständig vor Claude zu verbergen, bis Sie es manuell aufrufen. Für einen Skill, den Sie nicht geschrieben haben, setzen Sie [`skillOverrides`](/docs/de/skills#override-skill-visibility-from-settings) in den Einstellungen, um dasselbe zu tun, ohne die Datei zu bearbeiten.

<h3 id="understand-how-features-load">
  Verstehen Sie, wie Funktionen geladen werden
</h3>

Jede Funktion wird an verschiedenen Punkten in Ihrer Sitzung geladen. Die Registerkarten unten erklären, wann jede geladen wird und was in den Kontext geht.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/context-loading.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=aab139e750494a237ae2e0c8f9139b0a" className="dark:hidden" alt="Kontextladung: CLAUDE.md lädt beim Sitzungsstart und bleibt in jeder Anfrage. MCP-Tool-Namen laden beim Start mit vollständigen Schemas, die bis zur Verwendung aufgeschoben werden. Skills laden Beschreibungen beim Start, vollständigen Inhalt bei Aufruf. Subagents erhalten isolierten Kontext. Hooks laufen extern." width="720" height="382" data-path="images/context-loading.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/context-loading-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=b274089ef9612d9c760bca9838557626" className="hidden dark:block" alt="Kontextladung: CLAUDE.md lädt beim Sitzungsstart und bleibt in jeder Anfrage. MCP-Tool-Namen laden beim Start mit vollständigen Schemas, die bis zur Verwendung aufgeschoben werden. Skills laden Beschreibungen beim Start, vollständigen Inhalt bei Aufruf. Subagents erhalten isolierten Kontext. Hooks laufen extern." width="720" height="382" data-path="images/context-loading-dark.svg" />

<Tabs>
  <Tab title="CLAUDE.md">
    **Wann:** Sitzungsstart

    **Was lädt:** Vollständiger Inhalt aller CLAUDE.md-Dateien (verwaltet, Benutzer und Projektebenen).

    **Vererbung:** Claude liest CLAUDE.md-Dateien aus Ihrem Arbeitsverzeichnis bis zur Wurzel und entdeckt verschachtelte in Unterverzeichnissen, wenn es auf diese Dateien zugreift. Weitere Informationen finden Sie unter [Wie CLAUDE.md-Dateien geladen werden](/docs/de/memory#how-claude-md-files-load).

    <Tip>Halten Sie CLAUDE.md unter 200 Zeilen. Verschieben Sie Referenzmaterial zu Skills, die On-Demand geladen werden. Um [Kürzungsvorschläge für eine eingecheckte CLAUDE.md](/docs/de/memory#my-claude-md-is-too-large) zu erhalten, führen Sie `/doctor` aus.</Tip>
  </Tab>

  <Tab title="Skills">
    Skills sind zusätzliche Funktionen in Claudes Toolkit. Sie können Referenzmaterial sein (wie ein API-Stilhandbuch) oder aufrufbare Workflows, die Sie mit `/<name>` auslösen (wie `/deploy`). Claude Code wird mit [gebündelten Skills](/docs/de/commands) wie `/code-review`, `/batch` und `/debug` ausgeliefert, die sofort funktionieren. Sie können auch Ihre eigenen erstellen.

    **Wann:** Hängt von der Konfiguration des Skills ab. Standardmäßig werden Beschreibungen beim Sitzungsstart geladen und vollständiger Inhalt bei Verwendung. Für nur-Benutzer-Skills (`disable-model-invocation: true`) wird nichts geladen, bis Sie sie aufrufen.

    **Was lädt:** Für modell-aufrufbare Skills sieht Claude Namen und Beschreibungen in jeder Anfrage. Wenn Sie einen Skill mit `/<name>` aufrufen oder Claude ihn automatisch lädt, wird der vollständige Inhalt in Ihre Konversation geladen.

    **Wie Claude Skills wählt:** Claude gleicht Ihre Aufgabe gegen Skill-Beschreibungen ab, um zu entscheiden, welche relevant sind. Wenn Beschreibungen vage oder überlappend sind, kann Claude den falschen Skill laden oder einen verpassen, der helfen würde. Um Claude zu sagen, einen bestimmten Skill zu verwenden, rufen Sie ihn mit `/<name>` auf. Skills mit `disable-model-invocation: true` sind für Claude unsichtbar, bis Sie sie aufrufen.

    **Kontextkosten:** Niedrig bis verwendet. Nur-Benutzer-Skills haben Null-Kosten bis aufgerufen.

    **In Subagents:** Skills funktionieren in Subagents anders. Anstelle von On-Demand-Laden werden Skills, die im `skills:`-Feld des Subagenten aufgelistet sind, vollständig in seinen Kontext beim Start vorgeladen. Subagents können immer noch unlisted Project-, Benutzer- und Plugin-Skills durch das Skill-Tool entdecken und aufrufen.

    <Tip>Verwenden Sie `disable-model-invocation: true` für Skills mit Nebenwirkungen. Dies spart Kontext und stellt sicher, dass nur Sie sie auslösen.</Tip>
  </Tab>

  <Tab title="MCP-Server">
    **Wann:** Sitzungsstart.

    **Was lädt:** Tool-Namen und Server-Anweisungen von verbundenen Servern. Vollständige JSON-Schemas bleiben aufgeschoben, bis Claude ein bestimmtes Tool benötigt.

    **Kontextkosten:** [Tool-Suche](/docs/de/mcp#scale-with-mcp-tool-search) ist standardmäßig aktiviert, sodass untätige MCP-Tools minimalen Kontext verbrauchen.

    <Tip>Führen Sie `/mcp` aus, um den Verbindungsstatus jedes Servers zu sehen. Führen Sie `/context all` aus, um zu sehen, wie viele Token jedes geladene MCP-Tool verwendet. Claude Code [verbindet sich automatisch wieder mit Remote-Servern](/docs/de/mcp#automatic-reconnection), wenn diese ausfallen, und Sie können Server trennen, die Sie nicht aktiv verwenden.</Tip>
  </Tab>

  <Tab title="Code-Intelligenz">
    **Wann:** Nach Dateibearbeitungen und bei Bedarf, wenn Claude Code navigiert.

    **Was lädt:** Typfehler und Warnungen nach jeder Dateibearbeitung. Definitions-, Referenz- und Typinformationen, wenn Claude ein Symbol nachschlägt.

    **Kontextkosten:** Niedrig. Symbol-Suchen ersetzen oft umfangreiche Dateileser, sodass die Netto-Kontextnutzung sinken kann.

    <Tip>Das LSP-Tool ist inaktiv, bis Sie ein [Code-Intelligenz-Plugin](/docs/de/plugins/code-intelligence) für Ihre Sprache installieren.</Tip>
  </Tab>

  <Tab title="Subagents">
    **Wann:** Bei Bedarf, wenn Sie oder Claude einen für eine Aufgabe spawnt.

    **Was lädt:** Frischer, isolierter Kontext, der Folgendes enthält:

    * Der Agent's eigener System-Prompt, nicht der Claude Code System-Prompt
    * Vollständiger Inhalt von Skills, die im `skills:`-Feld des Agenten aufgelistet sind
    * CLAUDE.md und Git-Status, außer die integrierten Explore- und Plan-Agenten [lassen beide weg](/docs/de/sub-agents#what-loads-at-startup), und ein Agent, dessen Definition [`omitClaudeMd`](/docs/de/sub-agents#supported-frontmatter-fields) setzt, überspringt die Benutzer-, Projekt- und lokalen CLAUDE.md-Dateien
    * Welcher Kontext auch immer der Lead-Agent im Prompt übergibt

    Für einen [Fork](/docs/de/sub-agents#fork-the-current-conversation) lädt Claude Code die bisherige Konversation des übergeordneten Elements, den System-Prompt und die Tools statt dessen.

    **Kontextkosten:** Isoliert von Hauptsitzung.

    <Tip>Verwenden Sie Subagents für Arbeit, die Ihren vollständigen Konversationskontext nicht benötigt. Ihre Isolation verhindert, dass Ihre Hauptsitzung aufgebläht wird.</Tip>
  </Tab>

  <Tab title="Hooks">
    **Wann:** Bei Auslösung. Claude Code führt Hooks bei bestimmten Lebenszyklusereignissen aus, wie Tool-Ausführung, Sitzungsgrenzen, Prompt-Einreichung, Berechtigungsanfragen und Komprimierung. Siehe [Hooks](/docs/de/hooks) für die vollständige Liste.

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

  <Card title="Dynamic workflows" icon="network" href="/docs/de/workflows">
    Führen Sie viele Subagents aus einem Skript aus
  </Card>

  <Card title="Cross-session messaging" icon="terminal" href="/docs/de/cross-session-messaging">
    Lassen Sie Claude Ihre anderen Sitzungen benachrichtigen
  </Card>

  <Card title="MCP" icon="plug" href="/docs/de/mcp">
    Verbinden Sie Claude mit externen Diensten
  </Card>

  <Card title="Hooks" icon="bolt" href="/docs/de/hooks-guide">
    Automatisieren Sie Aktionen mit Hooks
  </Card>

  <Card title="Plugins" icon="puzzle-piece" href="/docs/de/plugins/overview">
    Bündeln und teilen Sie Feature-Sets
  </Card>

  <Card title="Marketplaces" icon="store" href="/docs/de/plugins/create-marketplace">
    Hosten und verteilen Sie Plugin-Sammlungen
  </Card>
</CardGroup>
