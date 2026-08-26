> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Beschleunigen Sie Antworten mit dem Schnellmodus

> Erhalten Sie schnellere Opus-Antworten in Claude Code durch Aktivierung des Schnellmodus.

<Note>
  Der Schnellmodus befindet sich in [Forschungsvorschau](#research-preview). Die Funktion, Preisgestaltung und Verfügbarkeit können sich basierend auf Feedback ändern.
</Note>

Der Schnellmodus ist eine Hochgeschwindigkeitskonfiguration für Claude Opus, die das Modell bis zu 2,5x schneller macht, allerdings zu höheren Kosten pro Token. Aktivieren Sie ihn mit `/fast`, wenn Sie Geschwindigkeit für interaktive Arbeiten wie schnelle Iteration oder Live-Debugging benötigen, und deaktivieren Sie ihn, wenn Kosten wichtiger sind als Latenz.

Der Schnellmodus ist kein anderes Modell. Er verwendet Claude Opus mit einer anderen API-Konfiguration, die Geschwindigkeit über Kosteneffizienz priorisiert. Sie erhalten identische Qualität und Funktionen mit schnelleren Antworten. Der Schnellmodus wird auf Opus 4.8 und Opus 4.7 unterstützt. Er ist nicht auf Sonnet, Haiku oder anderen Modellen verfügbar.

<Warning>
  Der Schnellmodus für Opus 4.7 ist seit dem 25. Juni 2026 veraltet und wird am 24. Juli 2026 entfernt. Nach der Entfernung geben Anfragen im Schnellmodus auf Opus 4.7 einen Fehler zurück und fallen nicht auf Standard-Opus 4.7 zurück. Migrieren Sie zu Opus 4.8, um die Beschleunigung beizubehalten.
</Warning>

Was Sie wissen sollten:

* Verwenden Sie `/fast`, um den Schnellmodus in der Claude Code CLI ein- oder auszuschalten. Der Schnellmodus wird in der VS Code-Erweiterung nicht unterstützt.
* Die Preisgestaltung für den Schnellmodus beträgt \$10/\$50 pro MTok Ein-/Ausgabe auf Opus 4.8 und \$30/\$150 pro MTok auf Opus 4.7.
* Verfügbar für alle Claude Code-Benutzer mit Abonnementplänen (Pro/Max/Team/Enterprise) und Claude Console.
* Für Claude Code-Benutzer mit Abonnementplänen (Pro/Max/Team/Enterprise) ist der Schnellmodus nur über Nutzungsguthaben verfügbar und nicht in den Abonnement-Ratenlimits enthalten.

<h2 id="toggle-fast-mode">
  Schnellmodus aktivieren
</h2>

Aktivieren Sie den Schnellmodus auf eine dieser Weisen:

* Geben Sie `/fast` ein und drücken Sie Tab, um ihn ein- oder auszuschalten
* Setzen Sie `"fastMode": true` in Ihrer [Benutzereinstellungsdatei](/docs/de/settings)

Standardmäßig bleibt der Schnellmodus, den Sie in einer interaktiven Sitzung aktivieren, über Sitzungen hinweg erhalten. Im [nicht-interaktiven Modus](/docs/de/headless) mit dem `-p`-Flag funktioniert `/fast` nur in einer Sitzung, die mit dem Schnellmodus in ihrem [`--settings`](/docs/de/cli-reference#cli-flags)-Wert gestartet wurde, zum Beispiel `claude -p --settings '{"fastMode": true}'`; der Toggle gilt dann nur für diese Sitzung und wird nicht als Ihr Standard gespeichert, und in jeder anderen nicht-interaktiven Sitzung meldet der Befehl, dass der Schnellmodus nicht verfügbar ist. Sie können den Schnellmodus so konfigurieren, dass er sich bei jeder Sitzung zurückgesetzt wird. Weitere Informationen finden Sie unter [Opt-in pro Sitzung erforderlich](#require-per-session-opt-in).

Für die beste Kosteneffizienz aktivieren Sie den Schnellmodus am Anfang einer Sitzung, anstatt ihn mitten in einem Gespräch zu wechseln. Weitere Informationen finden Sie unter [Kostenabwägung verstehen](#understand-the-cost-tradeoff).

Wenn Sie den Schnellmodus aktivieren:

* Wenn Sie sich auf einem anderen Modell befinden, wechselt Claude Code automatisch zu Opus
* Sie sehen eine Bestätigungsmeldung: „Fast mode ON"
* Ein kleines `↯`-Symbol wird neben der Eingabeaufforderung angezeigt, während der Schnellmodus aktiv ist
* Führen Sie `/fast` jederzeit erneut aus, um zu überprüfen, ob der Schnellmodus aktiviert oder deaktiviert ist

Wenn Sie den Schnellmodus mit `/fast` erneut deaktivieren, bleiben Sie auf Opus. Das Modell wird nicht auf Ihr vorheriges Modell zurückgesetzt. Um zu einem anderen Modell zu wechseln, verwenden Sie `/model`.

Wenn Sie zu einem Modell wechseln, das den Schnellmodus nicht unterstützt, wird der Schnellmodus deaktiviert. Wenn Sie zu einem unterstützten Opus-Modell zurückwechseln, wird es wieder aktiviert, wenn Ihre gespeicherte Schnellmodus-Einstellung aktiviert ist, die gleiche Einstellung, mit der eine neue Sitzung standardmäßig beginnt. Mit [Opt-in pro Sitzung](#require-per-session-opt-in) konfiguriert, wird der Schnellmodus beim Zurückwechseln nicht wieder aktiviert; führen Sie `/fast` aus, um ihn erneut zu aktivieren. Der Schnellmodus wird niemals für eine Sitzung aktiviert, deren gespeicherte Einstellung deaktiviert ist, und das `↯`-Symbol und die Bestätigung „Fast mode ON" werden angezeigt, wenn er aktiviert wird. Vor v2.1.208 blieb der Schnellmodus deaktiviert, nachdem Sie zurückgewechselt hatten, bis Sie `/fast` erneut ausführten.

Opus 4.8 ist der Standard für den Schnellmodus in Claude Code v2.1.154 und später. In v2.1.142 bis v2.1.153 wird der Schnellmodus standardmäßig auf Opus 4.7 gesetzt.

<h2 id="understand-the-cost-tradeoff">
  Kostenabwägung verstehen
</h2>

Der Schnellmodus hat höhere Pro-Token-Preise als Standard-Opus, wobei der Multiplikator je nach Modell variiert:

| Modell   | Eingabe (MTok) | Ausgabe (MTok) |
| -------- | -------------- | -------------- |
| Opus 4.8 | \$10           | \$50           |
| Opus 4.7 | \$30           | \$150          |

Die Preisgestaltung für den Schnellmodus ist über das gesamte 1M-Token-Kontextfenster einheitlich. Für den Standard-Opus-Satz zum Vergleich siehe die [Claude-Preisreferenz](https://platform.claude.com/docs/de/about-claude/pricing).

Wenn Sie den Schnellmodus zum ersten Mal in einem Gespräch aktivieren, zahlen Sie den vollständigen Schnellmodus-Preis für nicht zwischengespeicherte Eingabe-Token für den gesamten Gesprächskontext. Je tiefer Sie sich in einem Gespräch befinden, desto mehr kostet dies, daher ist die Aktivierung des Schnellmodus von Anfang an günstiger. Die Kosten fallen einmal pro Gespräch an, daher führt das spätere Ausschalten und erneute Einschalten des Schnellmodus nicht zu einer Wiederholung. Für den Mechanismus siehe [wie der Schnellmodus mit dem Prompt-Cache interagiert](/docs/de/prompt-caching#turning-on-fast-mode).

<h2 id="decide-when-to-use-fast-mode">
  Entscheiden Sie, wann Sie den Schnellmodus verwenden
</h2>

Der Schnellmodus ist am besten für interaktive Arbeiten geeignet, bei denen die Antwortlatenz wichtiger ist als die Kosten:

* Schnelle Iteration bei Code-Änderungen
* Live-Debugging-Sitzungen
* Zeitkritische Arbeiten mit engen Fristen

Der Standardmodus ist besser für:

* Lange autonome Aufgaben, bei denen Geschwindigkeit weniger wichtig ist
* Batch-Verarbeitung oder CI/CD-Pipelines
* Kostenempfindliche Arbeitslasten

<h3 id="fast-mode-vs-effort-level">
  Schnellmodus vs. Anstrengungsstufe
</h3>

Der Schnellmodus und die Anstrengungsstufe beeinflussen beide die Antwortgeschwindigkeit, aber auf unterschiedliche Weise:

| Einstellung                      | Auswirkung                                                                                        |
| -------------------------------- | ------------------------------------------------------------------------------------------------- |
| **Schnellmodus**                 | Gleiche Modellqualität, niedrigere Latenz, höhere Kosten                                          |
| **Niedrigere Anstrengungsstufe** | Weniger Denkzeit, schnellere Antworten, möglicherweise niedrigere Qualität bei komplexen Aufgaben |

Sie können beide kombinieren: Verwenden Sie den Schnellmodus mit einer niedrigeren [Anstrengungsstufe](/docs/de/model-config#adjust-effort-level) für maximale Geschwindigkeit bei einfachen Aufgaben.

<h2 id="requirements">
  Anforderungen
</h2>

Der Schnellmodus erfordert alle folgenden Voraussetzungen:

* **Nur Anthropic API oder Abonnement**: Der Schnellmodus ist über die Anthropic Console API und für Claude-Abonnementpläne mit Nutzungsguthaben verfügbar. Er ist nicht auf Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry oder Claude Platform auf AWS verfügbar.
* **Nutzungsguthaben aktiviert**: Ihr Konto muss Nutzungsguthaben aktiviert haben, was eine Abrechnung über die in Ihrem Plan enthaltene Nutzung hinaus ermöglicht. Aktivieren Sie dies für einzelne Konten in Ihren [Console-Abrechnungseinstellungen](https://platform.claude.com/settings/billing). Für Teams und Enterprise muss ein Administrator die Nutzungsguthaben für die Organisation aktivieren.

<Note>
  Die Nutzung des Schnellmodus wird direkt von Nutzungsguthaben abgerechnet, auch wenn Sie noch Nutzung in Ihrem Plan haben. Dies bedeutet, dass Schnellmodus-Token nicht gegen die in Ihrem Plan enthaltene Nutzung angerechnet werden und vom ersten Token an zum Schnellmodus-Tarif berechnet werden.
</Note>

* **Owner-Aktivierung für Teams und Enterprise**: Der Schnellmodus ist standardmäßig für Teams- und Enterprise-Organisationen deaktiviert. Ein Owner muss den Schnellmodus explizit [aktivieren](#enable-fast-mode-for-your-organization), bevor Benutzer darauf zugreifen können.

<Note>
  Wenn der Schnellmodus für Ihre Organisation nicht aktiviert wurde, zeigt der Befehl `/fast` „Fast mode has been disabled by your organization." an. Wenn die [`availableModels`](/docs/de/model-config#restrict-model-selection)-Zulassungsliste Ihrer Organisation das Schnellmodus-Opus-Modell ausschließt, wird `/fast` mit „is not in your organization's allowed models" abgelehnt. Die Ausnahme ist eine Sitzung, die bereits auf einem zulässigen Opus-Modell ausgeführt wird, das den Schnellmodus unterstützt: `/fast` aktiviert den Schnellmodus auf Ihrem aktuellen Modell, anstatt Modelle zu wechseln.
</Note>

<h3 id="enable-fast-mode-for-your-organization">
  Schnellmodus für Ihre Organisation aktivieren
</h3>

Wo Sie den Schnellmodus aktivieren, hängt davon ab, welches Produkt Ihre Organisation nutzt:

* **Console** (API-Kunden): Ein Administrator aktiviert ihn in [Claude Code-Einstellungen](https://platform.claude.com/claude-code/preferences)
* **Claude AI** (Teams und Enterprise): Ein Owner aktiviert ihn unter [Admin-Einstellungen > Claude Code](https://claude.ai/admin-settings/claude-code)

Eine weitere Option zum vollständigen Deaktivieren des Schnellmodus ist das Setzen von `CLAUDE_CODE_DISABLE_FAST_MODE=1`. Siehe [Umgebungsvariablen](/docs/de/env-vars).

<h3 id="require-per-session-opt-in">
  Opt-in pro Sitzung erforderlich
</h3>

Standardmäßig bleibt der Schnellmodus, den ein Benutzer in einer interaktiven Sitzung aktiviert, über Sitzungen hinweg erhalten: Er bleibt in zukünftigen Sitzungen aktiviert. Um dies zu ändern, setzen Sie `fastModePerSessionOptIn` in einer beliebigen [Einstellungsdatei](/docs/de/settings#settings-files) auf `true`, was dazu führt, dass jede Sitzung mit deaktiviertem Schnellmodus beginnt und Benutzer ihn explizit mit `/fast` aktivieren müssen. Owners in [Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_teams#team-&-enterprise)- oder [Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_enterprise)-Plänen können dies organisationsweit über [servergesteuerte Einstellungen](/docs/de/server-managed-settings) bereitstellen.

```json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

Dies ist nützlich zur Kostenkontrolle in Organisationen, in denen Benutzer mehrere gleichzeitige Sitzungen ausführen. Benutzer können den Schnellmodus immer noch mit `/fast` aktivieren, wenn sie Geschwindigkeit benötigen, aber er wird zu Beginn jeder neuen Sitzung zurückgesetzt. Die Schnellmodus-Einstellung des Benutzers wird immer noch gespeichert, sodass das Entfernen dieser Einstellung das standardmäßige persistente Verhalten wiederherstellt.

<h2 id="handle-rate-limits">
  Ratenlimits handhaben
</h2>

Der Schnellmodus hat separate Ratenlimits vom Standard-Opus. Der Schnellmodus auf Opus 4.8 und Opus 4.7 teilen sich den gleichen Ratenlimit-Pool: Die Nutzung auf einem beliebigen dieser Modelle wird von den gleichen Limits abgezogen. Wenn Sie das Ratenlimit des Schnellmodus erreichen oder keine Nutzungsguthaben mehr haben:

1. Der Schnellmodus fällt automatisch auf Standard-Geschwindigkeit auf
2. Das `↯`-Symbol wird grau, um die Abkühlung anzuzeigen
3. Sie arbeiten weiterhin mit Standard-Geschwindigkeit und -Preisen
4. Wenn die Abkühlung abläuft, wird der Schnellmodus automatisch wieder aktiviert

Um den Schnellmodus manuell zu deaktivieren, anstatt auf die Abkühlung zu warten, führen Sie `/fast` erneut aus.

<h2 id="research-preview">
  Forschungsvorschau
</h2>

Der Schnellmodus ist eine Forschungsvorschau-Funktion. Dies bedeutet:

* Die Funktion kann sich basierend auf Feedback ändern
* Verfügbarkeit und Preisgestaltung können sich ändern
* Die zugrunde liegende API-Konfiguration kann sich weiterentwickeln

Melden Sie Probleme oder Feedback über Ihre üblichen Anthropic-Supportkanäle.

<h2 id="see-also">
  Siehe auch
</h2>

* [Modellkonfiguration](/docs/de/model-config): Wechseln Sie Modelle und passen Sie Anstrengungsstufen an
* [Kosten effektiv verwalten](/docs/de/costs): Verfolgen Sie die Token-Nutzung und reduzieren Sie Kosten
* [Statuszeilen-Konfiguration](/docs/de/statusline): Zeigen Sie Modell- und Kontextinformationen an
