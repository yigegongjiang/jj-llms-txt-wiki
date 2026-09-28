> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Schwierige Entscheidungen mit dem Advisor-Tool eskalieren

> Kombinieren Sie Ihr Hauptmodell mit einem stärkeren Advisor-Modell, das Claude an wichtigen Momenten während einer Aufgabe konsultiert.

<Note>
  Das Advisor-Tool ist experimentell und erfordert die Anthropic API. Es ist nicht auf Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform oder Microsoft Foundry verfügbar. Verhalten, Preisgestaltung und Verfügbarkeit können sich ändern.
</Note>

Das Advisor-Tool ermöglicht es Claude, ein zweites, typischerweise stärkeres Modell an wichtigen Momenten während einer Aufgabe zu konsultieren, z. B. bevor ein Ansatz festgelegt wird, wenn ein wiederkehrender Fehler auftritt, oder bevor eine Aufgabe als abgeschlossen erklärt wird. Der Advisor erhält das gesamte Gespräch, einschließlich aller Tool-Aufrufe und Ergebnisse, und gibt Anleitung zurück, die Claude vor dem Fortfahren anwendet.

Der Advisor läuft serverseitig auf der Infrastruktur von Anthropic als [Server-Tool](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool), das sowohl für Abonnement- als auch für API-abgerechnete Konten verfügbar ist. Sie wählen, welches Modell als Advisor fungiert, und Claude entscheidet, wann es aufgerufen wird.

Diese Seite behandelt, wie Sie den Advisor aktivieren, welche Modellkombinationen akzeptiert werden, was Claude während einer Konsultation anzeigt, und wie die Advisor-Nutzung abgerechnet wird.

<h2 id="when-to-use-the-advisor">
  Wann der Advisor verwendet werden sollte
</h2>

Der Advisor eignet sich für lange, mehrstufige Aufgaben, bei denen die meisten Schritte Routine sind, aber die Planqualität das Ergebnis bestimmt. Beispiele sind große Umstrukturierungen, Debugging-Sitzungen, bei denen ein Fehler immer wieder auftritt, und Aufgaben, die Sie unabhängig überprüft haben möchten, bevor Claude sie als abgeschlossen erklärt.

Er bietet weniger Wert bei kurzen Aufgaben, bei denen es wenig zu planen gibt, oder bei Arbeiten, bei denen jeder Schritt das stärkste Modell benötigt. Für diese Fälle [wechseln Sie das Hauptmodell](/docs/de/model-config#setting-your-model) oder siehe [wie der Advisor mit opusplan und Subagents verglichen wird](#compare-with-related-features) für andere Möglichkeiten, eine zweite Meinung zu erhalten.

<h2 id="enable-the-advisor">
  Aktivieren Sie den Advisor
</h2>

Sie können das Advisor-Modell auf drei Arten festlegen:

* **`/advisor` Befehl**: Legen Sie den Advisor während einer Sitzung fest oder ändern Sie ihn und speichern Sie ihn als Standard
* **`advisorModel` Einstellung**: Konfigurieren Sie einen persistenten Standard in Ihrer [Einstellungsdatei](/docs/de/settings)
* **`--advisor` Flag**: Legen Sie den Advisor für eine einzelne Sitzung beim Start fest

Jede dieser Optionen aktiviert den Advisor für Sitzungen, deren Hauptmodell [es unterstützt](#choose-an-advisor-model). Nach dem Start der Sitzung zeigt Claude Code eine `Advisor Tool (experimental) is on and may use more tokens · /advisor` Benachrichtigung an. Um die Verwendung des Advisors zu beenden, siehe [Schalten Sie den Advisor aus](#turn-the-advisor-off).

Bei einigen Plänen benötigt Fable als Advisor auch Ihre einmalige [Zustimmung zur Abrechnung der Fable-Nutzung auf Nutzungsguthaben](/docs/de/model-config#fable-and-usage-credits). Informationen dazu, was vor Ihrer Zustimmung geschieht, finden Sie unter [Fable Advisor und Nutzungsguthaben](#fable-advisor-and-usage-credits).

<h3 id="use-the-/advisor-command">
  Verwenden Sie den `/advisor` Befehl
</h3>

Führen Sie `/advisor` ohne Argumente aus, um eine Auswahl mit den verfügbaren Advisor-Modellen zu öffnen, oder übergeben Sie das Modell direkt:

```
/advisor opus
```

Der Befehl bestätigt mit `Advisor set to` gefolgt vom Namen des Advisor-Modells. Ihre Auswahl wird in `advisorModel` in Ihren Benutzereinstellungen gespeichert und bleibt über Sitzungen hinweg erhalten, außer in den Fällen, die der [`advisorModel` Eintrag](/docs/de/settings-reference#advisormodel) als nur für die aktuelle Sitzung geltend auflistet.

Der Befehl funktioniert auch dort, wo es keine Terminal-Auswahl gibt: im [nicht-interaktiven Modus](/docs/de/headless) mit `-p`, im Agent SDK, in der Desktop-App und über [Remote Control](/docs/de/remote-control). Dies erfordert Claude Code v2.1.260 oder später. Auf diesen Oberflächen:

* Führen Sie `/advisor` ohne Argument aus, um das aktuelle Advisor-Modell und die Aliase, die es akzeptiert, auszudrucken.
* Führen Sie `/advisor` mit einem Modell aus, wie `/advisor opus`, um es festzulegen.
* Führen Sie `/advisor off` aus, um es auszuschalten.

Claude Code ruft einen gespeicherten Advisor nicht auf, den die [`availableModels`](/docs/de/model-config#restrict-model-selection) Allowlist Ihrer Organisation ausschließt. Um den Advisor zu verwenden, wählen Sie ein zulässiges Modell mit `/advisor`. Claude Code speichert trotzdem einen Advisor, den Ihr aktuelles Hauptmodell nicht unterstützt. Dieser Advisor wird aktiviert, nachdem Sie zu einem [kompatiblen Hauptmodell](#choose-an-advisor-model) mit [`/model`](/docs/de/model-config#setting-your-model) wechseln. Wenn die API den gespeicherten Advisor bereits in der aktuellen Konversation abgelehnt hat, bleibt er ausgeschaltet, bis Sie `/clear` oder `/compact` ausführen, auch nachdem Sie die Modelle wechseln.

Bei einigen Plänen benötigt Fable als Advisor auch Ihre einmalige [Zustimmung zur Abrechnung der Fable-Nutzung auf Nutzungsguthaben](/docs/de/model-config#fable-and-usage-credits). Informationen dazu, was `/advisor fable` vor Ihrer Zustimmung tut, finden Sie unter [Fable Advisor und Nutzungsguthaben](#fable-advisor-and-usage-credits).

<h3 id="set-advisormodel-in-settings">
  Legen Sie `advisorModel` in den Einstellungen fest
</h3>

Um den Advisor als Standard zu konfigurieren, ohne eine Sitzung zu öffnen, legen Sie ihn in Ihrer Einstellungsdatei fest:

```json theme={null}
{
  "advisorModel": "opus"
}
```

<h3 id="use-the-advisor-flag">
  Verwenden Sie das `--advisor` Flag
</h3>

Um den Advisor für eine einzelne Sitzung festzulegen, ohne Ihre gespeicherte Einstellung zu ändern, starten Sie mit dem Flag:

```bash theme={null}
claude --advisor opus
```

Claude Code verwendet das Flag statt der `advisorModel` Einstellung für diese Sitzung. Es listet `--advisor` nicht in `claude --help` auf. Claude Code beendet sich mit einem Fehler beim Start, wenn:

* Das Hauptmodell der Sitzung den Advisor nicht unterstützt
* Das angeforderte Modell, wie Haiku, nicht als Advisor fungieren kann
* Die [`availableModels`](/docs/de/model-config#restrict-model-selection) Allowlist Ihrer Organisation das angeforderte Modell ausschließt
* Sie Fable angefordert haben und Ihr Konto erfordert noch die [Zustimmung zu Nutzungsguthaben](#fable-advisor-and-usage-credits)

Wenn Sie eine [Hintergrund-Sitzung](/docs/de/agent-view) mit `--advisor` starten und eine dieser Bedingungen zutrifft, startet Claude Code die Sitzung ohne den Advisor, anstatt zu beenden.

<h2 id="choose-an-advisor-model">
  Wählen Sie ein Advisor-Modell
</h2>

Der Advisor muss mindestens so leistungsfähig sein wie das Hauptmodell. Die akzeptierten Advisors für jedes Hauptmodell sind:

| Hauptmodell            | Akzeptierte Advisors                  | Hinweise                                                                                                   |
| ---------------------- | ------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| Haiku 4.5              | Fable, Opus, Sonnet                   | Haiku kann den Advisor aufrufen, kann aber nicht als einer fungieren                                       |
| Sonnet 4.6             | Fable, Opus, Sonnet                   |                                                                                                            |
| Sonnet 5               | Fable, Opus 4.7 oder später, Sonnet 5 | Ein Sonnet 4.6 Advisor wird abgelehnt, und die API lehnt einen Opus 4.6 Advisor ab                         |
| Opus 4.6               | Fable, Opus, Sonnet 5                 | Ein Sonnet 4.6 Advisor wird abgelehnt                                                                      |
| Opus 4.7 oder Opus 4.8 | Fable und Opus 4.7 oder später        | Ein Opus 4.6 oder Sonnet Advisor wird abgelehnt                                                            |
| Opus 5.5 oder Opus 5   | Fable und Opus 5 oder später          | Ein Opus 4.6 oder Sonnet Advisor wird abgelehnt, und die API lehnt einen Opus 4.7 oder Opus 4.8 Advisor ab |
| Fable 5                | Fable 5.1 oder Fable 5                | Ein Opus oder Sonnet Advisor wird abgelehnt                                                                |
| Fable 5.1              | Fable 5.1                             | Ein Opus oder Sonnet Advisor wird abgelehnt, und die API lehnt einen Fable 5 Advisor ab                    |

Fable 5.1 erfordert Claude Code v2.1.257 oder später. Beide Fable-Modelle erfordern [Fable-Zugriff](/docs/de/model-config#work-with-fable).

Legen Sie den Advisor als `fable`, `opus` oder `sonnet` fest. Diese Aliase werden in die in Claude Code integrierte Standardversion für jede Modellfamilie aufgelöst, die sich mit neuen Claude Code-Versionen weiterentwickelt. Sie können auch eine vollständige Modell-ID wie `claude-opus-5-5` übergeben.

Subagenten erben den konfigurierten Advisor und wenden die gleiche Kopplungsprüfung gegen ihr eigenes Modell an.

Claude Code validiert die Kopplung vor dem Senden einer Anfrage, und die API validiert sie erneut:

* Für einen Advisor, den die Tabelle als abgelehnt auflistet, hängt Claude Code ihn nicht an die Anfragen des Hauptmodells an. Die `/advisor` Befehlsausgabe und eine Benachrichtigung zeigen dies an. Subagenten, deren eigenes Modell die Kopplung erfüllt, können den Advisor möglicherweise trotzdem verwenden.
* Für einen Advisor, den die Tabelle als von der API abgelehnt auflistet, hängt Claude Code ihn an und die API lehnt ihn ab. Claude Code sendet diese Anfrage dann ohne den Advisor erneut, und der Rest der Konversation läuft ohne einen, sodass Sie keinen Fehler sehen und keine Advisor-Aufrufe erhalten. Wählen Sie einen akzeptierten Advisor mit `/advisor`; die Änderung wird nach `/clear` oder `/compact` und in neuen Sitzungen wirksam.
* Wenn das Hauptmodell oder der Advisor ein Modell ist, das Claude Code nicht erkennt, wird der Advisor nicht angehängt.

<h3 id="fable-advisor-and-usage-credits">
  Fable Advisor und Nutzungsguthaben
</h3>

Bei einigen Plänen wird die Fable-Nutzung auf Nutzungsguthaben abgerechnet, und Fable als Advisor wird auf die gleiche Weise abgerechnet. Wenn Ihr Konto die [einmalige Zustimmung zur Abrechnung der Fable-Nutzung auf Nutzungsguthaben](/docs/de/model-config#fable-and-usage-credits) erfordert, fragt Claude Code danach, wenn Sie ein Fable-Modell mit `/model` auswählen, und wendet Fable nicht als Advisor an, bis Sie dieser Zustimmung zugestimmt haben.

Bevor Sie zugestimmt haben, speichert Claude Code Fable nicht als Advisor, wenn Sie `/advisor fable` eingeben oder Fable in der `/advisor` Auswahl wählen. Stattdessen verweist es Sie auf `/model fable`. Mit `claude --advisor fable` beendet Claude Code den Start mit einer Nachricht, die auf `/model fable` verweist. In einer [Hintergrundsitzung](#use-the-advisor-flag) wird die Sitzung ohne den Advisor gestartet, anstatt zu beenden. Mit Fable, das bereits als Ihr `advisorModel` gespeichert ist, sendet Claude Code Anfragen ohne den Advisor. In einer interaktiven Sitzung, deren Hauptmodell den Advisor unterstützt, wird auch eine Benachrichtigung angezeigt, die auf `/model fable` verweist.

Um der Zustimmung zuzustimmen, führen Sie `/model fable` aus und wählen Sie, um mit Fable fortzufahren. Claude Code zeichnet die Zustimmung auf und [speichert Fable als Ihr ausgewähltes Modell](/docs/de/model-config#default-model-setting). Wählen Sie dann Fable als Advisor.

<h3 id="common-model-pairings">
  Häufige Modellkombinationen
</h3>

Jede akzeptierte Kopplung funktioniert. Diese Kombinationen gleichen Kosten gegen Leistung auf verschiedene Weise aus:

| Kopplung                            | Wann zu verwenden                                                                                                                                                    |
| ----------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Sonnet Hauptmodell + Opus Advisor   | Sonnet verarbeitet Routineaufgaben und eskaliert Planung, mehrdeutige Fehler und Abschlussüberprüfungen an Opus                                                      |
| Sonnet Hauptmodell + Fable Advisor  | Fable-Anleitung an Entscheidungspunkten ohne Fable durchgehend auszuführen. Erfordert Fable-Zugriff                                                                  |
| Haiku Hauptmodell + Opus Advisor    | Kostengünstigstes Hauptmodell mit starker Planung. Erwarten Sie höhere Kosten als Haiku allein, aber niedriger als das Wechseln des Hauptmodells zu Sonnet oder Opus |
| Opus Hauptmodell + Opus Advisor     | Ein zweiter Opus überprüft den ersten. Nützlich für hochriskante Aufgaben, bei denen eine unabhängige Überprüfung wichtiger ist als Kosten                           |
| Fable Hauptmodell + Fable Advisor   | Höchste Leistungskopplung, wenn Fable verfügbar ist. Claude Code wendet keinen Opus oder Sonnet Advisor auf ein Fable Hauptmodell an                                 |
| Sonnet Hauptmodell + Sonnet Advisor | Eine kostengünstigere zweite Meinung zum Erkennen von Routineversäumnissen                                                                                           |

<h2 id="when-claude-consults-the-advisor">
  Wann Claude den Advisor konsultiert
</h2>

Claude entscheidet, wann der Advisor aufgerufen wird. Es neigt dazu, vor dem Festlegen eines Ansatzes zu konsultieren, wenn ein Fehler immer wieder auftritt, und vor der Erklärung einer Aufgabe als abgeschlossen, aber der Zeitpunkt ist modellgesteuert und nicht regelbasiert.

Sie können in Ihrem Prompt eine Konsultation anfordern, genauso wie Sie jedes andere Tool anfordern würden, zum Beispiel `consult the advisor before you continue`. Es gibt keine Einstellung, um Advisor-Aufrufe zu begrenzen oder zu erzwingen; wenn Sie möchten, dass Claude während einer Aufgabe häufiger oder seltener konsultiert, sagen Sie dies in Ihren Anweisungen.

<h2 id="what-you-see-during-a-session">
  Was Sie während einer Sitzung sehen
</h2>

Wenn Claude den Advisor aufruft, zeigt das Transkript eine `Advising` Zeile mit dem Namen des Advisor-Modells, während der Aufruf läuft. Wenn das Ergebnis zurückkommt, bestätigt die Zeile, ob der Advisor eine Anleitung gegeben hat:

* **Reviewed**: Die Zeile bestätigt, dass der Advisor das Gespräch überprüft hat. Wenn der Advisor lesbare Anleitung gegeben hat, drücken Sie `Ctrl+O`, um sie zu lesen.
* **Declined**: Die Zeile lautet `Advisor declined to advise on this request`. Wenn der Advisor einen Grund angegeben hat, drücken Sie `Ctrl+O`, um ihn zu lesen.

Claude folgt im Allgemeinen der Anleitung des Advisors, passt sich aber an, wenn seine eigenen Erkenntnisse einer spezifischen Aussage widersprechen: Wenn ein empfohlener Schritt beim Versuch fehlschlägt oder der Dateiinhalt der Anleitung widerspricht, zeigt Claude den Konflikt auf, anstatt die Anleitung bedingungslos zu befolgen.

Der Advisor erhält immer das gesamte Gespräch, und Claude kontrolliert den Zeitpunkt. Für mehr Kontrolle oder eine andere Konfiguration siehe [wie der Advisor mit Subagents und opusplan verglichen wird](#compare-with-related-features).

<h2 id="cost">
  Kosten
</h2>

Wenn Claude den Advisor aufruft, liest das Advisor-Modell das Gespräch, daher verbraucht jeder Aufruf Token zu den Sätzen des Advisor-Modells zusätzlich zu Ihrer Hauptmodellnutzung. Wie diese Advisor-Token abgerechnet werden, hängt davon ab, wie Sie bezahlen:

* **API-Abrechnung**: Sie zahlen die Input- und Output-Sätze des Advisor-Modells für Advisor-Token
* **Abonnementpläne**: Die Advisor-Nutzung zählt zu den Nutzungsgrenzen Ihres Plans, mit Ausnahme, dass ein Fable-Advisor zu [Nutzungsguthaben](/docs/de/model-config#fable-and-usage-credits) bei Plänen abgerechnet wird, bei denen die Fable-Nutzung dies tut

Wenn Ihr Konto die Zustimmung zu Nutzungsguthaben erfordert, wird ein Fable-Advisor vor der Zustimmung nicht abgerechnet, da Claude Code [die Auswahl nicht anwendet](#fable-advisor-and-usage-credits), bis Sie dies tun.

Claude ruft den Advisor an Entscheidungspunkten auf, nicht bei jedem Schritt, daher kostet die Kopplung eines schnelleren Hauptmodells mit einem stärkeren Advisor typischerweise weniger als das durchgehende Ausführen des stärkeren Modells. Die Advisor-Nutzung zählt zu den Sitzungssummen, die von [`/usage`](/docs/de/costs#track-your-costs) angezeigt werden.

Für die Berichterstattung von Advisor-Token in API-Antworten siehe [Nutzung und Abrechnung](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool#usage-and-billing) in der Claude API-Dokumentation.

<h2 id="impact-on-prompt-caching">
  Auswirkung auf Prompt-Caching
</h2>

Das Aktivieren oder Deaktivieren des Advisors während einer Sitzung invalidiert nicht den [Prompt-Cache](/docs/de/prompt-caching) Ihres Hauptmodells. Im Gegensatz zum [Wechsel von Modellen](/docs/de/prompt-caching#switching-models) behält das Umschalten von `/advisor` das zwischengespeicherte Präfix bei, und die vom Advisor zurückgegebene Anleitung wird als Teil des Transkripts bei späteren Schritten zwischengespeichert.

Das eigene Lesen des Advisor-Modells des Gesprächs wird nicht zwischengespeichert. Jeder Advisor-Aufruf verarbeitet das gesamte Transkript neu, ohne Wiederverwendung zwischen Aufrufen.

<h2 id="requirements">
  Anforderungen
</h2>

Das Advisor-Tool erfordert alle folgenden Voraussetzungen:

* **Nur Anthropic API**: Der Advisor ist ein serverseitig ausgeführtes Tool. Er ist nicht auf Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform oder Microsoft Foundry verfügbar. Über ein [LLM-Gateway](/docs/de/llm-gateway), das mit `ANTHROPIC_BASE_URL` konfiguriert ist, hängt die Verfügbarkeit davon ab, ob das Gateway die Anfrage intakt an die Anthropic API weiterleitet. Wenn das Gateway oder sein Upstream das Advisor-Tool nicht erkennt, siehe [Automatische Wiederholung und Fehlerweiterleitung](/docs/de/llm-gateway-protocol#automatic-retry-and-error-forwarding), um zu erfahren, wie Claude Code reagiert.
* **Unterstütztes Hauptmodell**: Fable, Opus 4.6 oder später, Sonnet 4.6 oder später oder Haiku 4.5. Siehe [Wählen Sie ein Advisor-Modell](#choose-an-advisor-model), um zu erfahren, welche Advisors jedes akzeptiert.
* **Feature-Flag-Abruf**: Claude Code aktiviert den Advisor über ein Feature-Flag, das er von Anthropic abruft. In einer Sitzung, in der eine Variable, die den Flag-Abruf deaktiviert, gesetzt ist, wie z. B. `DISABLE_TELEMETRY`, bleibt der Advisor deaktiviert. Siehe [Features, die Feature-Flag-Abruf benötigen](/docs/de/env-vars#features-that-need-feature-flag-fetching).

<h2 id="turn-the-advisor-off">
  Schalten Sie den Advisor aus
</h2>

Um die Verwendung des Advisors zu beenden, führen Sie `/advisor off` aus oder wählen Sie **No advisor** in der `/advisor` Auswahl:

```
/advisor off
```

Um das Advisor-Tool vollständig zu deaktivieren, setzen Sie `CLAUDE_CODE_DISABLE_ADVISOR_TOOL=1`. Der `/advisor` Befehl wird nicht verfügbar und jedes konfigurierte `advisorModel` wird ignoriert. Das `--advisor` Flag wird akzeptiert, hat aber keine Auswirkung. Siehe [Umgebungsvariablen](/docs/de/env-vars).

<h2 id="compare-with-related-features">
  Vergleich mit verwandten Funktionen
</h2>

Der Advisor ist eine von mehreren Möglichkeiten, Modellstärken zu kombinieren. Wählen Sie basierend darauf, wann Sie ein zweites Modell beteiligt haben möchten.

| Ansatz                                                         | Wann das stärkere Modell läuft                                                                                                                        | Wie es startet                                   |
| -------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------ |
| Advisor-Tool                                                   | An Entscheidungspunkten während der Aufgabe                                                                                                           | Claude ruft es auf, wenn es Anleitung benötigt   |
| [`opusplan`](/docs/de/model-config#opusplan-model-setting)          | Während des Plan-Modus, wenn [erlaubt durch `availableModels`](/docs/de/model-config#restrict-model-selection), dann wechselt zu Sonnet für die Ausführung | Sie treten in den Plan-Modus ein                 |
| [Subagents](/docs/de/sub-agents#choose-a-model) mit `model` gesetzt | Für die gesamte delegierte Teilaufgabe                                                                                                                | Claude delegiert oder Sie rufen den Subagent auf |
| [`/model`](/docs/de/model-config#setting-your-model)                | Für alle nachfolgenden Schritte                                                                                                                       | Sie wechseln Modelle                             |

<h2 id="see-also">
  Siehe auch
</h2>

* [Modellkonfiguration](/docs/de/model-config): Wechseln Sie Modelle, legen Sie Aufwandsstufen fest und verwenden Sie `opusplan`
* [Verwalten Sie Kosten effektiv](/docs/de/costs): Verfolgen Sie die Token-Nutzung über Modelle hinweg
* [Advisor-Tool in der Claude API](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool): Verstehen Sie das zugrunde liegende Server-Tool oder verwenden Sie es direkt von der Messages API
* [Die Advisor-Strategie](https://claude.com/blog/the-advisor-strategy): Warum die Kopplung eines schnellen Hauptmodells mit einem stärkeren Advisor funktioniert
