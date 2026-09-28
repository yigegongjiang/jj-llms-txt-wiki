> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Scannen Sie Ihre Codebasis auf Sicherheitslücken

> Installieren Sie das Claude Security Plugin, um Ihre Codebasis in einer Claude Code-Sitzung auf Sicherheitslücken zu scannen und Erkenntnisse in Patches umzuwandeln, die Sie überprüfen und anwenden.

Das Claude Security Plugin führt einen Multi-Agent-Sicherheitslücken-Scan Ihrer Codebasis innerhalb einer Claude Code-Sitzung durch. Ein Team von Claude-Agenten kartiert Ihre Architektur, erstellt ein Bedrohungsmodell, sucht nach Sicherheitslücken und überprüft unabhängig jeden Fund, bevor der Bericht geschrieben wird. Verwenden Sie das Plugin, um ein ganzes Repository zu scannen oder [nur einen Satz von Änderungen](#scan-only-your-changes), wie das Diff eines Branches, das Diff eines Pull Requests oder einen einzelnen Commit, und wandeln Sie dann die Erkenntnisse Ihrer Wahl in Patches um, die Sie selbst überprüfen und anwenden.

Das Plugin wird lokal in Ihrer Sitzung ausgeführt, verwendet die Modelle, auf die Sie in Claude Code Zugriff haben, und jeder Scan wird auf die Nutzungslimits Ihres Plans angerechnet. Wenn Sie einen verwalteten Service möchten, der Ihre Repositories überwacht, oder Scans auf [Claude Mythos 5](https://platform.claude.com/docs/en/about-claude/models/introducing-claude-fable-5-and-claude-mythos-5) ausführen möchten, siehe das [Claude Security](https://claude.com/product/claude-security) Produkt, das im Enterprise-Plan verfügbar ist. Das Plugin erreicht Code, den das verwaltete Produkt nicht erreichen kann, wie Repositories, die auf GitLab oder Bitbucket gehostet werden, oder auf Netzwerken, die keine eingehenden Verbindungen zulassen.

Das Plugin unterscheidet sich auch von den Überprüfungswerkzeugen, die bereits in Claude Code vorhanden sind: Das [Security Guidance Plugin](/docs/de/security-guidance) überprüft Code, während Claude ihn schreibt, [`/security-review`](/docs/de/commands#all-commands) führt einen einzelnen Durchgang über Ihren Branch durch, und [Code Review](/docs/de/code-review) überprüft Pull Requests. Wie die Ebenen zusammenpassen, siehe [Wie das Plugin mit anderen Sicherheitswerkzeugen passt](#how-the-plugin-fits-with-other-security-tools).

<h2 id="prerequisites">
  Voraussetzungen
</h2>

Um das Plugin auszuführen, benötigen Sie:

* Einen bezahlten Plan für die [dynamischen Workflows](/docs/de/workflows), die der Scan verwendet, um seine Agenten zu orchestrieren. Aktivieren Sie sie auf Pro über die Zeile „Dynamic workflows" in `/config`.
* Python 3.9 oder später, verfügbar auf Ihrem `PATH` als `python3`. Überprüfen Sie mit `python3 --version`. Das Tooling des Plugins verwendet nur die Python-Standardbibliothek, daher wird nichts installiert.
* Linux, macOS oder Windows.
* Git, für Änderungsscans und zum Umwandeln von Erkenntnissen in Patches; diese Jobs unterstützen keine anderen Versionskontrollsysteme. Ein vollständiger Scan funktioniert in jedem Verzeichnis, mit oder ohne Versionskontrolle.

<h2 id="install-the-plugin">
  Installieren Sie das Plugin
</h2>

Installieren Sie in einer Claude Code-Sitzung aus dem [offiziellen Anthropic-Marketplace](/docs/de/plugins/anthropic-marketplaces):

```text theme={null}
/plugin install claude-security@claude-plugins-official
```

Der Befehl öffnet die Details des Plugins, wo Sie einen [Installationsbereich](/docs/de/plugins/install#install-a-plugin) wählen, um die Installation zu starten.

Wenn die Installation fehlschlägt, hängt die Behebung von der Meldung ab, die Claude Code meldet:

* Wenn es meldet `Marketplace "claude-plugins-official" not found`, fügen Sie den Marketplace mit `/plugin marketplace add anthropics/claude-plugins-official` hinzu und versuchen Sie dann die Installation erneut.
* Wenn es meldet, dass es [das Plugin im Marketplace nicht finden kann](/docs/de/plugins/install#install-a-plugin), überprüfen Sie den Plugin-Namen auf Tippfehler.

Überprüfen Sie die Installationszusammenfassung. Wenn sie meldet `Run /reload-plugins to activate.`, lesen Sie [Plugin-Änderungen ohne Neustart anwenden](/docs/de/plugins/cli-reference#reload-plugins), um das Plugin in Ihrer aktuellen Sitzung zu aktivieren.

Sobald das Plugin aktiv ist, sind Sie bereit zum [Scannen und Beheben Ihrer Codebasis](#scan-and-fix-your-codebase).

<h3 id="uninstall-the-plugin">
  Deinstallieren Sie das Plugin
</h3>

Um das Plugin zu entfernen, deinstallieren Sie es aus dem `/plugin`-Menü, oder führen Sie `claude plugin uninstall claude-security` in Ihrem Terminal aus.

<h2 id="scan-and-fix-your-codebase">
  Scannen und beheben Sie Ihre Codebasis
</h2>

Das Plugin fügt einen Befehl hinzu, `/claude-security`, der ein Menü mit seinen drei Jobs öffnet: Scannen der Codebasis, Scannen einer Reihe von Änderungen und Vorschlagen von Patches. Der glückliche Weg führt einen vollständigen Scan durch und wandelt dann seine Erkenntnisse in Patches um:

<Steps>
  <Step title="Öffnen Sie das Claude Security-Menü">
    Führen Sie `/claude-security` aus und wählen Sie **Scan codebase**.
  </Step>

  <Step title="Wählen Sie aus, was gescannt werden soll">
    Das Plugin liest zunächst Ihr Repository, bietet dann das gesamte Repository oder einen fokussierten Bereich an, wobei die Dateianzahl und die relativen Kosten jeder Option angegeben sind. Wählen Sie das gesamte Repository, oder antworten Sie „I don't know" und das Plugin wählt einen sinnvollen Standard für die Größe Ihres Repositories.
  </Step>

  <Step title="Bestätigen Sie den Durchlauf">
    Ein Scan kann eine Weile dauern, kann eine erhebliche Anzahl von Tokens verwenden und erfordert, dass Claude Code offen bleibt, während er abgeschlossen wird. Nichts wird ausgeführt, bis Sie bestätigen.
  </Step>

  <Step title="Lesen Sie den Bericht">
    Während der Scan läuft, meldet er jede Phase, wenn sie beginnt, mit den Details verfügbar unter [`/workflows`](/docs/de/workflows). Die Ergebnisse landen in einem Verzeichnis mit Zeitstempel in Ihrem Repository, beschrieben in [Lesen Sie die Scan-Ergebnisse](#read-the-scan-results).
  </Step>

  <Step title="Wandeln Sie Erkenntnisse in Patches um">
    Führen Sie `/claude-security` erneut aus und wählen Sie **Suggest patches**, wählen Sie dann, welche Erkenntnisse Sie adressieren möchten. Überprüfte Patches landen im `patches/`-Ordner des Berichts; [Beheben Sie Erkenntnisse](#fix-findings) behandelt, wie jeder Patch erstellt und überprüft wird.
  </Step>

  <Step title="Wenden Sie die Patches an, die Sie akzeptieren">
    Wenden Sie jeden Patch aus Ihrer Shell mit `git apply` an, in seinem eigenen Pull Request. Patches werden niemals automatisch angewendet.
  </Step>
</Steps>

Sie müssen nicht vom Menü aus starten: Fragen Sie direkt nach einem Job, als Argumente für den Befehl, wie `/claude-security scan my branch`, oder in einfacher Sprache, wie „scan commit abc1234". Das Plugin funktioniert am besten im [Auto-Modus](/docs/de/permission-modes), der es den Agenten des Scans ermöglicht, ohne eine Berechtigungsaufforderung bei jedem Schritt fortzufahren.

<h3 id="scan-only-your-changes">
  Scannen Sie nur Ihre Änderungen
</h3>

Wenn Ihr Branch Commits hat, die seine Basis nicht hat, bietet das `/claude-security`-Menü an, nur dieses Diff zu scannen, damit Sie einen Branch vor dem Zusammenführen überprüfen können. Sie können auch einen Ihrer offenen Pull Requests scannen oder einen einzelnen Commit scannen, indem Sie danach fragen, wie z. B. „scan commit abc1234". Nur committete Änderungen werden gescannt: Committen oder stashen Sie laufende Änderungen zuerst, oder führen Sie einen vollständigen Scan durch, der den Arbeitsbaum liest.

Änderungsscans benötigen ein Git-Repository; vollständige Scans eines unversionierten Verzeichnisses funktionieren immer noch. Das Finden Ihrer offenen Pull Requests ist der einzige Schritt, der das Netzwerk erreicht, und er wird nur angeboten, wenn Ihre Sitzung bereits die Berechtigung hat, die GitHub CLI auszuführen und `gh` angemeldet ist.

<h3 id="scope-large-repositories">
  Umfang großer Repositories
</h3>

Scannen Sie bei einem großen Repository jeweils einen Bereich statt des gesamten Baums. Wählen Sie einen der fokussierten Bereiche, die das Plugin anbietet, wie z. B. Ihre API-Schicht oder Ihren Authentifizierungscode, und die Ausführung passt sich an das an, was Sie wählen. Der Abschnitt „Coverage" des Berichts gibt an, was untersucht wurde und was nicht. Führen Sie jederzeit einen weiteren Scan in einem anderen Bereich durch.

<h3 id="read-the-scan-results">
  Lesen Sie die Scan-Ergebnisse
</h3>

Jeder Scan schreibt seine Ergebnisse in ein Verzeichnis `CLAUDE-SECURITY-<timestamp>/` mit Zeitstempel in Ihrem Repository:

* **`CLAUDE-SECURITY-RESULTS.md`**: der Bericht, mit der ID jedes Funds, wie z. B. `F1`, plus seine Auswirkung, Exploitierungsszenario, Schweregrad, Konfidenz und Empfehlung
* **`CLAUDE-SECURITY-RESULTS.jsonl`**: die gleichen Erkenntnisse in maschinenlesbarer Form, ein JSON-Objekt pro Zeile
* **`CLAUDE-SECURITY-RESULTS.sarif`**: die gleichen Erkenntnisse als [SARIF 2.1.0](https://docs.oasis-open.org/sarif/sarif/v2.1.0/sarif-v2.1.0.html) Log für GitHub Code Scanning und jedes andere Tool, das den Standard liest. Der Scan klassifiziert Erkenntnisse unter ihren [CWE](https://cwe.mitre.org/) Schwächekategorien
* **`CLAUDE-SECURITY-REVISION-<commit>.json`**: der Revisionsstempel, der aufzeichnet, welcher Commit gescannt wurde, mit welchem Aufwand, ob nicht committete Änderungen Teil des gescannten Baums waren, und wie gründlich der Durchlauf überprüft wurde, damit ein Bericht immer an den Code gebunden ist, den er beschreibt. Ein Scan außerhalb der Versionskontrolle stempelt `UNVERSIONED` anstelle des Commits

Dieses Verzeichnis ist die einzige Änderung, die ein Scan an Ihrem Checkout vornimmt, und es hat sein eigenes `.gitignore`, daher wird ein verirrtes `git add` niemals einen Bericht in einen Commit fegen. Um einen Bericht in der Historie für einen Audit-Trail zu behalten, löschen Sie diese eine `.gitignore`-Datei und committen Sie das Verzeichnis wie jedes andere.

Erkenntnisse erscheinen nur im Bericht, nachdem unabhängige Verifier-Agenten sie analysiert haben, was Berichte kurz und lesenswert hält. Scans sind nicht deterministisch: Zwei Scans des gleichen Codes können unterschiedliche Erkenntnisse aufdecken. Führen Sie Scans regelmäßig durch, und verwenden Sie die Revisionsstempel, um jeden Bericht dem genauen Code und den Einstellungen zuzuordnen, die er abdeckte.

<h2 id="fix-findings">
  Beheben Sie Erkenntnisse
</h2>

Starten Sie den Fix-Flow, indem Sie **Suggest patches** aus dem `/claude-security`-Menü wählen, oder fragen Sie in einfacher Sprache, wie z. B. „fix finding F3", wählen Sie dann, welche Erkenntnisse aus dem Bericht Sie adressieren möchten. Patches werden gegen committeten Code erstellt, und der Bericht muss immer noch den Code beschreiben, den Sie haben: Erkenntnisse, deren Code sich seitdem geändert hat, werden mit einer Notiz übersprungen, und das Plugin bietet einen frischen Scan statt des Patchens aus einem veralteten Bericht an. Jeder Patch wird in einer Arbeitskopie Ihres Repositories entworfen, daher bleiben Ihre Quelldateien unberührt, bis Sie einen Patch selbst anwenden.

Vor der Lieferung wird jeder Patch von einem Agenten überprüft, der unabhängig von dem ist, der ihn geschrieben hat, der Ihre Projekttests gegen die Änderung ausführt, wenn der Code sie hat, und das Diff auf seine eigenen Bedingungen liest, um alles Neue zu finden, das es möglicherweise einführt. Ein Patch wird nur geschrieben, wenn diese Überprüfung bestätigen kann, dass die Änderung den einen Fund adressiert, keine neue Sicherheitslücke einführt und das Verhalten ansonsten unverändert lässt. Wenn es nicht für alle drei garantieren kann, erhalten Sie stattdessen eine kurze Notiz, die erklärt, warum.

<h3 id="patches-are-never-applied-automatically">
  Patches werden niemals automatisch angewendet
</h3>

Das Anwenden eines Patches ist immer Ihre Entscheidung. Patches landen im `patches/`-Ordner des Berichts, ein `F<n>.patch` pro Fund mit einer Notiz daneben, die die Änderung erklärt. Wenden Sie einen aus Ihrer Shell an, oder bitten Sie Claude, ihn anzuwenden und einen Pull Request zu öffnen:

```bash theme={null}
git apply CLAUDE-SECURITY-<timestamp>/patches/F1.patch
```

Wenn der gepatchte Code keine Tests hat, sagt die Notiz des Patches dies, daher wissen Sie, dass seine Überprüfung ohne einen Test-Pass lief. Wenden Sie jeden Patch in seinem eigenen Pull Request an, damit er überprüft und getestet werden kann.

<h2 id="how-the-plugin-fits-with-other-security-tools">
  Wie das Plugin mit anderen Sicherheitswerkzeugen passt
</h2>

Das Claude Security Plugin ist die On-Demand-Deep-Scan-Schicht in einem Defense-in-Depth-Stack, neben dem [Security Guidance Plugin](/docs/de/security-guidance), [`/security-review`](/docs/de/commands#all-commands), [Code Review](/docs/de/code-review), dem verwalteten [Claude Security](https://claude.com/product/claude-security) Produkt und Ihren bestehenden Scannern:

| Phase                          | Werkzeug                                                                       | Was es abdeckt                                                                                       |
| :----------------------------- | :----------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------- |
| In Sitzung                     | [Security Guidance Plugin](/docs/de/security-guidance)                              | Häufige Sicherheitslücken in Code, den Claude schreibt, behoben in der gleichen Sitzung              |
| On Demand, einzelner Durchgang | [`/security-review`](/docs/de/commands#all-commands)                                | Einmaliger Sicherheitsdurchgang auf dem aktuellen Branch                                             |
| On Demand, Deep Scan           | Claude Security Plugin                                                         | Multi-Agent-Scan eines Repositories oder Diffs, mit unabhängig überprüften Erkenntnissen und Patches |
| Bei Pull Request               | [Code Review](/docs/de/code-review), Team- und Enterprise-Pläne                     | Multi-Agent-Korrektheit und Sicherheitsüberprüfung mit vollständigem Codebase-Kontext                |
| Verwaltet                      | [Claude Security](https://claude.com/product/claude-security), Enterprise-Plan | Gehostetes Scannen, das verbundene Repositories überwacht                                            |
| In CI                          | Ihre bestehenden statischen Analyse- und Abhängigkeitsscanner                  | Sprachspezifische Regeln, Supply-Chain-Checks und Richtliniendurchsetzung                            |

Das Plugin ersetzt Ihre bestehenden Source-Code-Sicherheitswerkzeuge nicht. Führen Sie es neben statischer Analyse, Abhängigkeitsscanning und Code-Review aus: Es argumentiert über Ihren Code so, wie es ein menschlicher Sicherheitsforscher tun würde, was die deterministischen Checks ergänzt, die diese Werkzeuge bieten.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

**Das `/claude-security`-Menü öffnet sich mit einer Python-Warnung.** Das Plugin benötigt `python3` 3.9 oder später auf Ihrem `PATH`. Wenn es `python3` überhaupt nicht finden kann, warnt das Menü, dass Claude Security nicht funktioniert, bis eines installiert ist; wenn das erste `python3` auf Ihrem `PATH` älter ist, benennt die Warnung die Version, die es gefunden hat. Installieren Sie Python 3, oder setzen Sie ein neueres `python3` zuerst auf Ihren `PATH`, dann starten Sie eine neue Sitzung.

**Sie können eine Meldung „safeguards flagged this message" sehen, wenn Sie auf einem Fable-Modell scannen.** Die Meldung benennt das Modell, zum Beispiel „Fable 5.1's safeguards flagged this message". Die Cybersecurity-Sicherheitsklassifizierer von Fable kennzeichnen bestimmte Anfragen, und Claude Code führt eine gekennzeichnete Anfrage auf einem Opus-Modell durch [automatisches Modell-Fallback](/docs/de/model-config#automatic-model-fallback) erneut aus. Dies ist zu erwarten, und der Scan sollte immer noch erfolgreich abgeschlossen werden.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

Um tiefer in die Teile einzusteigen, die diese Seite berührt:

* [Security Guidance Plugin](/docs/de/security-guidance): Fangen Sie Probleme in Code ab, während Claude ihn schreibt, in der gleichen Sitzung
* [Code Review](/docs/de/code-review): Richten Sie die Multi-Agent-Überprüfung zur PR-Zeit ein
* [Claude Security](https://claude.com/product/claude-security): Der verwaltete Service, der verbundene Repositories überwacht
* [Claude Code-Sicherheit](/docs/de/security): Wie Claude Code Vertrauen, Berechtigungen und Schutzmaßnahmen angeht
* [Plugins installieren und verwalten](/docs/de/plugins/install): Finden und installieren Sie andere Plugins aus dem offiziellen Marketplace
