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

Wenn eine dieser Optionen ein Advisor-Modell festlegt, ist der Advisor für Sitzungen aktiviert, deren Hauptmodell [es unterstützt](#choose-an-advisor-model). Um die Verwendung zu beenden, siehe [Schalten Sie den Advisor aus](#turn-the-advisor-off).

<Note>
  Um Fable 5 als Advisor zu verwenden, benötigen Sie Claude Code v2.1.170 oder später und [Fable 5 Zugriff](/docs/de/model-config#work-with-fable-5) für Ihre Organisation.
</Note>

<h3 id="use-the-/advisor-command">
  Verwenden Sie den `/advisor` Befehl
</h3>

Führen Sie `/advisor` ohne Argumente aus, um eine Auswahl mit den verfügbaren Advisor-Modellen zu öffnen, oder übergeben Sie das Modell direkt:

```
/advisor opus
```

Ihre Auswahl wird in `advisorModel` in Ihren Benutzereinstellungen gespeichert und bleibt über Sitzungen hinweg erhalten. Wenn die [`availableModels`](/docs/de/model-config#restrict-model-selection) Allowlist Ihrer Organisation das gespeicherte Advisor-Modell ausschließt, wird der Advisor nicht aufgerufen, bis Sie ein zulässiges Modell mit `/advisor` auswählen. Wenn Ihr aktuelles Hauptmodell den Advisor nicht unterstützt, wird die Auswahl trotzdem gespeichert und aktiviert sich, wenn Sie zu einem [kompatiblen Hauptmodell](#choose-an-advisor-model) mit [`/model`](/docs/de/model-config#setting-your-model) wechseln.

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

Das Flag hat Vorrang vor der `advisorModel` Einstellung für diese Sitzung. Es beendet sich mit einem Fehler, wenn das Hauptmodell der Sitzung den Advisor nicht unterstützt, oder wenn das angeforderte Advisor-Modell durch die [`availableModels`](/docs/de/model-config#restrict-model-selection) Allowlist Ihrer Organisation ausgeschlossen ist.

<h2 id="choose-an-advisor-model">
  Wählen Sie ein Advisor-Modell
</h2>

Der Advisor muss mindestens so leistungsfähig sein wie das Hauptmodell. Die akzeptierten Advisors für jedes Hauptmodell sind:

| Hauptmodell          | Akzeptierte Advisors      | Hinweise                                                                                                                                                                                           |
| -------------------- | ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Haiku 4.5            | Fable, Opus, Sonnet       | Haiku kann den Advisor aufrufen, kann aber nicht als einer fungieren                                                                                                                               |
| Sonnet 4.6           | Fable, Opus, Sonnet       |                                                                                                                                                                                                    |
| Sonnet 5             | Fable, Opus, Sonnet 5     | Ein Sonnet 4.6 Advisor wird abgelehnt                                                                                                                                                              |
| Opus 4.6             | Fable, Opus, Sonnet 5     | Sonnet 5 und Opus 4.6 werden als gleich leistungsfähig eingestuft, daher akzeptiert ein Opus 4.6 Hauptmodell einen Sonnet 5 Advisor                                                                |
| Opus 4.7 oder später | Fable, Opus 4.7, Opus 4.8 | Opus 4.7 und Opus 4.8 werden als gleich leistungsfähig eingestuft, daher akzeptiert jedes das andere als Advisor. Ein Opus 4.7 Hauptmodell mit einem Opus 4.6 oder Sonnet 5 Advisor wird abgelehnt |
| Fable 5 (v2.1.170+)  | Fable                     | Ein Opus oder Sonnet Advisor wird abgelehnt                                                                                                                                                        |

Fable 5 erfordert Claude Code v2.1.170 oder später und Fable 5 Zugriff, unabhängig davon, ob es als Hauptmodell oder als Advisor fungiert.

Legen Sie den Advisor als `opus`, `sonnet` oder `fable` fest. Diese Aliase werden in die neueste Version jedes Modells aufgelöst. Sie können auch eine vollständige Modell-ID wie `claude-opus-4-8` übergeben.

Subagenten erben den konfigurierten Advisor und wenden die gleiche Kopplungsprüfung gegen ihr eigenes Modell an.

Claude Code validiert die Kopplung vor dem Senden einer Anfrage:

* Wenn der Advisor weniger leistungsfähig ist als das Hauptmodell, wird der Advisor nicht an die Anfragen des Hauptmodells angehängt. Die `/advisor` Befehlsausgabe und eine Benachrichtigung zeigen dies an. Subagenten, deren eigenes Modell die Kopplung erfüllt, können den Advisor möglicherweise trotzdem verwenden.
* Wenn das Hauptmodell oder der Advisor ein Modell ist, das Claude Code nicht erkennt, wird der Advisor nicht angehängt.

<h3 id="common-model-pairings">
  Häufige Modellkombinationen
</h3>

Jede akzeptierte Kopplung funktioniert. Diese Kombinationen gleichen Kosten gegen Leistung auf verschiedene Weise aus:

| Kopplung                            | Wann zu verwenden                                                                                                                                                                          |
| ----------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Sonnet Hauptmodell + Opus Advisor   | Sonnet verarbeitet Routineaufgaben und eskaliert Planung, mehrdeutige Fehler und Abschlussüberprüfungen an Opus                                                                            |
| Sonnet Hauptmodell + Fable Advisor  | Fable 5 Anleitung an Entscheidungspunkten ohne Fable 5 durchgehend auszuführen. Erfordert v2.1.170 oder später und Fable 5 Zugriff                                                         |
| Haiku Hauptmodell + Opus Advisor    | Kostengünstigstes Hauptmodell mit starker Planung. Erwarten Sie höhere Kosten als Haiku allein, aber niedriger als das Wechseln des Hauptmodells zu Sonnet oder Opus                       |
| Opus Hauptmodell + Opus Advisor     | Ein zweiter Opus überprüft den ersten. Nützlich für hochriskante Aufgaben, bei denen eine unabhängige Überprüfung wichtiger ist als Kosten                                                 |
| Fable Hauptmodell + Fable Advisor   | Höchste Leistungskopplung, wenn Fable 5 verfügbar ist (v2.1.170+). Fable ist eine höhere Stufe als Opus und Sonnet, daher ist es der einzige akzeptierte Advisor für ein Fable Hauptmodell |
| Sonnet Hauptmodell + Sonnet Advisor | Eine kostengünstigere zweite Meinung zum Erkennen von Routineversäumnissen                                                                                                                 |

<h2 id="when-claude-consults-the-advisor">
  Wann Claude den Advisor konsultiert
</h2>

Claude entscheidet, wann der Advisor aufgerufen wird. Es neigt dazu, vor dem Festlegen eines Ansatzes zu konsultieren, wenn ein Fehler immer wieder auftritt, und vor der Erklärung einer Aufgabe als abgeschlossen, aber der Zeitpunkt ist modellgesteuert und nicht regelbasiert.

Sie können in Ihrem Prompt eine Konsultation anfordern, genauso wie Sie jedes andere Tool anfordern würden, zum Beispiel `consult the advisor before you continue`. Es gibt keine Einstellung, um Advisor-Aufrufe zu begrenzen oder zu erzwingen; wenn Sie möchten, dass Claude während einer Aufgabe häufiger oder seltener konsultiert, sagen Sie dies in Ihren Anweisungen.

<h2 id="what-you-see-during-a-session">
  Was Sie während einer Sitzung sehen
</h2>

Wenn Claude den Advisor aufruft, zeigt das Transkript eine `Advising` Zeile mit dem Namen des Advisor-Modells, während der Aufruf läuft. Wenn das Ergebnis zurückkommt, bestätigt die Zeile, dass der Advisor das Gespräch überprüft hat. Drücken Sie `Ctrl+O`, um es zu erweitern und die vollständige Anleitung des Advisors zu lesen.

Claude folgt im Allgemeinen der Anleitung des Advisors, passt sich aber an, wenn seine eigenen Erkenntnisse einer spezifischen Aussage widersprechen: Wenn ein empfohlener Schritt beim Versuch fehlschlägt oder der Dateiinhalt der Anleitung widerspricht, zeigt Claude den Konflikt auf, anstatt die Anleitung bedingungslos zu befolgen.

Der Advisor erhält immer das gesamte Gespräch, und Claude kontrolliert den Zeitpunkt. Für mehr Kontrolle oder eine andere Konfiguration siehe [wie der Advisor mit Subagents und opusplan verglichen wird](#compare-with-related-features).

<h2 id="cost">
  Kosten
</h2>

Jeder Advisor-Aufruf sendet das Gespräch an das Advisor-Modell, daher verbraucht es Token zu den Sätzen des Advisor-Modells zusätzlich zu Ihrer Hauptmodellnutzung. Bei API-Abrechnung werden Advisor-Token zu den Input- und Output-Sätzen des Advisor-Modells berechnet. Bei Abonnementplänen zählt die Advisor-Nutzung zu den Nutzungsgrenzen Ihres Plans.

Claude ruft den Advisor an Entscheidungspunkten auf, nicht bei jedem Schritt, daher kostet die Kopplung eines schnelleren Hauptmodells mit einem stärkeren Advisor typischerweise weniger als das durchgehende Ausführen des stärkeren Modells. Die Advisor-Nutzung zählt zu den Sitzungssummen, die von [`/usage`](/docs/de/costs#track-your-costs) angezeigt werden.

Für die Berichterstattung von Advisor-Token in API-Antworten siehe [Nutzung und Abrechnung](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool#usage-and-billing) in der Claude API-Dokumentation.

<h2 id="impact-on-prompt-caching">
  Auswirkung auf Prompt-Caching
</h2>

Das Aktivieren oder Deaktivieren des Advisors während einer Sitzung invalidiert nicht den [Prompt-Cache](/docs/de/prompt-caching) Ihres Hauptmodells. Im Gegensatz zum [Ändern von Modell oder Aufwandsstufe](/docs/de/prompt-caching#actions-that-invalidate-the-cache) behält das Umschalten von `/advisor` das zwischengespeicherte Präfix bei, und die vom Advisor zurückgegebene Anleitung wird als Teil des Transkripts bei späteren Schritten zwischengespeichert.

Das eigene Lesen des Advisor-Modells des Gesprächs wird nicht zwischengespeichert. Jeder Advisor-Aufruf verarbeitet das gesamte Transkript neu, ohne Wiederverwendung zwischen Aufrufen.

<h2 id="requirements">
  Anforderungen
</h2>

Das Advisor-Tool erfordert alle folgenden Voraussetzungen:

* **Nur Anthropic API**: Der Advisor ist ein serverseitig ausgeführtes Tool. Er ist nicht auf Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform oder Microsoft Foundry verfügbar. Über ein [LLM-Gateway](/docs/de/llm-gateway), das mit `ANTHROPIC_BASE_URL` konfiguriert ist, hängt die Verfügbarkeit davon ab, ob das Gateway die Anfrage intakt an die Anthropic API weiterleitet.
* **Unterstütztes Hauptmodell**: Opus 4.6 oder später, Sonnet 4.6 oder später oder Haiku 4.5. Fable 5 qualifiziert sich auch auf Claude Code v2.1.170 oder später.

<h2 id="turn-the-advisor-off">
  Schalten Sie den Advisor aus
</h2>

Um die Verwendung des Advisors zu beenden und Ihren gespeicherten `advisorModel` zu löschen, führen Sie `/advisor off` aus oder wählen Sie **No advisor** in der `/advisor` Auswahl:

```
/advisor off
```

Um das Advisor-Tool vollständig zu deaktivieren, setzen Sie `CLAUDE_CODE_DISABLE_ADVISOR_TOOL=1`. Der `/advisor` Befehl wird nicht verfügbar und jedes konfigurierte `advisorModel` wird ignoriert. Das `--advisor` Flag wird akzeptiert, hat aber keine Auswirkung; vorhandene Skripte, die es übergeben, funktionieren weiterhin ohne Fehler. Siehe [Umgebungsvariablen](/docs/de/env-vars).

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
