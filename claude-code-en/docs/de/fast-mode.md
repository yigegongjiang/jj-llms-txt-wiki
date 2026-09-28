> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Beschleunigen Sie Antworten mit dem Schnellmodus

> Erhalten Sie schnellere Opus-Antworten in Claude Code durch Aktivierung des Schnellmodus.

<Note>
  Der Schnellmodus befindet sich in [Forschungsvorschau](#research-preview). Die Funktion, Preisgestaltung und Verfügbarkeit können sich basierend auf Feedback ändern.
</Note>

Der Schnellmodus ist eine Hochgeschwindigkeitskonfiguration für Claude Opus, die das Modell bis zu 2,5x schneller macht, allerdings zu höheren Kosten pro Token. Aktivieren Sie ihn mit `/fast`, wenn Sie Geschwindigkeit für interaktive Arbeiten wie schnelle Iteration oder Live-Debugging benötigen, und deaktivieren Sie ihn, wenn Kosten wichtiger sind als Latenz.

Der Schnellmodus ist kein anderes Modell. Er verwendet Claude Opus mit einer anderen API-Konfiguration, die Geschwindigkeit über Kosteneffizienz priorisiert. Sie erhalten identische Qualität und Funktionen mit schnelleren Antworten. Der Schnellmodus wird auf Opus 5.5, Opus 5 und Opus 4.8 unterstützt. Er ist nicht auf Sonnet, Haiku oder anderen Modellen verfügbar.

Opus 4.7 unterstützt keinen Schnellmodus, daher schaltet das Wechseln zu ihm den Schnellmodus aus. Der Schnellmodus für Opus 4.7 wurde am 25. Juni 2026 eingestellt und am 24. Juli 2026 entfernt.

Was Sie wissen sollten:

* Verwenden Sie `/fast`, um den Schnellmodus in der Claude Code CLI ein- oder auszuschalten. Die [VS Code-Erweiterung](/docs/de/vs-code) bietet einen **Schnellmodus umschalten**-Befehl, wenn das ausgewählte Modell den Schnellmodus unterstützt. Claude Code speichert diese Einstellung in Ihrer [`fastMode`-Einstellung](#toggle-fast-mode).
* Die Preisgestaltung für den Schnellmodus beträgt \$8/\$40 pro MTok Ein-/Ausgabe auf Opus 5.5 und \$10/\$50 auf Opus 5 und Opus 4.8.
* Verfügbar für Claude Code-Benutzer mit Abonnementplänen (Pro/Max/Team/Enterprise) und auf Claude Console. Team- und Enterprise-Organisationen benötigen zunächst einen Eigentümer, um dies zu aktivieren, und Console-Organisationen benötigen zunächst bereitgestellten Zugriff, beide beschrieben unter [Anforderungen](#requirements).
* Für Claude Code-Benutzer mit Abonnementplänen (Pro/Max/Team/Enterprise) ist der Schnellmodus nur über Nutzungsguthaben verfügbar und nicht in den Abonnement-Ratenlimits enthalten.

<h2 id="toggle-fast-mode">
  Schnellmodus aktivieren
</h2>

Im CLI können Sie den Schnellmodus auf eine dieser Weisen aktivieren:

* Geben Sie `/fast` ein, drücken Sie Leertaste, um ihn ein- oder auszuschalten, und drücken Sie dann Enter, um zu bestätigen
* Setzen Sie `"fastMode": true` in Ihrer [Benutzereinstellungsdatei](/docs/de/settings)

Standardmäßig bleibt der Schnellmodus, den Sie in einer interaktiven Sitzung aktivieren, über Sitzungen hinweg erhalten. Sie können den Schnellmodus so konfigurieren, dass er sich bei jeder Sitzung zurückgesetzt wird. Weitere Informationen finden Sie unter [Opt-in pro Sitzung erforderlich](#require-per-session-opt-in).

Außerhalb einer [Cloud-Sitzung](#use-fast-mode-in-cloud-sessions), im [nicht-interaktiven Modus](/docs/de/headless) mit dem `-p`-Flag funktioniert `/fast` nur in einer Sitzung, die mit dem Schnellmodus in ihrem [`--settings`](/docs/de/cli-reference#cli-flags)-Wert gestartet wurde, zum Beispiel `claude -p --settings '{"fastMode": true}'`; der Toggle gilt dann nur für diese Sitzung und wird nicht als Ihr Standard gespeichert. Das `-p`-Formular erfordert Claude Code v2.1.205 oder später. Anderswo im nicht-interaktiven Modus meldet der Befehl, dass der Schnellmodus nicht verfügbar ist.

Sie können `/fast` ausführen, während Claude arbeitet, und Claude Code schaltet den Schnellmodus um, ohne auf das Ende des Zugs zu warten. Claude Code beendet den laufenden Zug mit seiner ursprünglichen Geschwindigkeit, sodass die Geschwindigkeitsänderung ab Ihrem nächsten Zug wirksam wird. Wenn Ihr aktuelles Modell den Schnellmodus nicht unterstützt, wird durch das Aktivieren auch Ihr Modell gewechselt, und Claude Code verwendet das neue Modell ab seiner nächsten Anfrage in diesem Zug.

Für die beste Kosteneffizienz aktivieren Sie den Schnellmodus am Anfang einer Sitzung, anstatt ihn mitten in einem Gespräch zu wechseln. Weitere Informationen finden Sie unter [Kostenabwägung verstehen](#understand-the-cost-tradeoff).

Wenn Sie den Schnellmodus aktivieren:

* Wenn Ihr aktuelles Modell den Schnellmodus nicht unterstützt, wechselt Claude Code zu Opus
* Sie sehen eine Bestätigungsmeldung: „Fast mode ON"
* Ein kleines `↯`-Symbol wird neben der Eingabeaufforderung angezeigt, während der Schnellmodus aktiv ist
* Führen Sie `/fast` jederzeit erneut aus, um zu überprüfen, ob der Schnellmodus aktiviert oder deaktiviert ist

Opus 5.5 ist der Standard für den Schnellmodus in Claude Code v2.1.280 und später. Vor v2.1.280 war der Schnellmodus standardmäßig auf Opus 5 in v2.1.219 und später, auf Opus 4.8 in v2.1.154 bis v2.1.218 und auf Opus 4.7 in v2.1.142 bis v2.1.153 eingestellt.

Wenn Sie den Schnellmodus mit `/fast` erneut deaktivieren, bleiben Sie auf Opus. Um zu einem anderen Modell zu wechseln, verwenden Sie `/model`.

<h3 id="switch-models-while-fast-mode-is-on">
  Modelle wechseln, während der Schnellmodus aktiviert ist
</h3>

Der Schnellmodus folgt Ihren Modellwechseln in beide Richtungen:

* **Wechsel weg**: Wenn Sie zu einem Modell wechseln, das den Schnellmodus nicht unterstützt, schaltet Claude Code den Schnellmodus aus. Dies gilt auch für Opus 4.7; vor v2.1.221 blieb der Schnellmodus nach einem Wechsel zu Opus 4.7 aktiviert und die API lehnte die Anfragen ab.
* **Zurückwechsel**: Das Zurückwechseln zu einem unterstützten Opus-Modell aktiviert den Schnellmodus erneut, wenn Ihre gespeicherte Schnellmodus-Einstellung aktiviert ist, die gleiche Einstellung, mit der eine neue Sitzung standardmäßig beginnt. Ein Modellwechsel aktiviert den Schnellmodus niemals für eine Sitzung, deren gespeicherte Einstellung deaktiviert ist, und mit [Opt-in pro Sitzung](#require-per-session-opt-in) konfiguriert, wird er beim Zurückwechseln auch nicht aktiviert; führen Sie `/fast` aus, um ihn erneut zu aktivieren.

Immer wenn ein Modellwechsel den Schnellmodus ein- oder ausschaltet, zeigt Claude Code eine Bestätigung „Fast mode ON" oder „Fast mode OFF" an, und das `↯`-Symbol wird angezeigt, während der Schnellmodus aktiviert ist. Dies gilt, ob Sie mit `/model`, mit [`/config model=<model>`](/docs/de/settings) oder von einem Gerät aus wechseln, das über [Remote Control](/docs/de/remote-control) verbunden ist.

Claude Code sendet den Schnellmodus-Status der Sitzung nach einem Modellwechsel, einer Wiederverbindung oder einer fehlgeschlagenen [Verfügbarkeitsprüfung](#use-fast-mode-behind-proxies-and-llm-gateways) erneut an Geräte, die über Remote Control verbunden sind.

<h3 id="use-fast-mode-in-cloud-sessions">
  Schnellmodus in Cloud-Sitzungen verwenden
</h3>

Der Schnellmodus funktioniert in [Cloud-Sitzungen](/docs/de/claude-code-on-the-web), wenn er auf Ihrem Konto verfügbar ist, unabhängig davon, ob die Sitzung auf von Anthropic verwalteter Infrastruktur oder einem [selbstgehosteten Runner](/docs/de/self-hosted-environments) ausgeführt wird. Erfordert Claude Code v2.1.271 oder später in der Umgebung der Sitzung.

Geben Sie `/fast on` in der Sitzung ein, um den Schnellmodus zu aktivieren. Er bleibt nur für diese Sitzung aktiviert und wird nicht als Ihr Standard gespeichert. Die [Anforderungen](#requirements) gelten auch in Cloud-Sitzungen.

<h2 id="understand-the-cost-tradeoff">
  Kostenabwägung verstehen
</h2>

Der Schnellmodus hat höhere Pro-Token-Preise als Standard-Opus:

| Modell   | Eingabe (MTok) | Ausgabe (MTok) |
| -------- | -------------- | -------------- |
| Opus 5.5 | \$8            | \$40           |
| Opus 5   | \$10           | \$50           |
| Opus 4.8 | \$10           | \$50           |

Die Preisgestaltung für den Schnellmodus ist über das gesamte 1M-Token-Kontextfenster einheitlich. Für den Standard-Opus-Satz zum Vergleich siehe die [Claude-Preisreferenz](https://platform.claude.com/docs/de/about-claude/pricing).

Wenn Sie den Schnellmodus zum ersten Mal in einem Gespräch aktivieren, zahlen Sie den vollständigen Schnellmodus-Preis für nicht zwischengespeicherte Eingabe-Token für den gesamten Gesprächskontext. Je tiefer Sie sich in einem Gespräch befinden, desto mehr kostet dies, daher ist die Aktivierung des Schnellmodus von Anfang an günstiger. Die Kosten fallen einmal pro Gespräch an, daher führt das spätere Ausschalten und erneute Einschalten des Schnellmodus nicht zu einer Wiederholung. Für den Mechanismus siehe [wie der Schnellmodus mit dem Prompt-Cache interagiert](/docs/de/prompt-caching#turning-on-fast-mode).

<h3 id="see-where-fast-mode-spend-appears">
  Sehen Sie, wo die Ausgaben für den Schnellmodus angezeigt werden
</h3>

Sie sehen die Ausgaben für den Schnellmodus an verschiedenen Stellen, je nachdem, wie Sie sich angemeldet haben. Führen Sie daher zunächst [`/status`](/docs/de/commands) aus, um dies zu überprüfen. Wenn eine `Login method`-Zeile wie `Claude Max account` angezeigt wird, haben Sie sich mit einem Claude-Abonnement angemeldet. Wenn stattdessen eine `API key`-Zeile angezeigt wird, werden Ihre Anfragen einer Claude Console-Organisation in Rechnung gestellt.

* **Pro und Max**: Sie zahlen für den Schnellmodus aus Ihren Nutzungsguthaben. Gehen Sie zu [**Einstellungen > Nutzung**](https://claude.ai/settings/usage) auf claude.ai, wo der Abschnitt **Nutzungsguthaben** anzeigt, wie viel Sie diesen Monat in Nutzungsguthaben ausgegeben haben. Diese Zahl umfasst den Schnellmodus, schlüsselt ihn aber nicht separat auf.
* **Team und Enterprise**: Ihre Organisation zahlt für Ihre Schnellmodus-Nutzung aus ihren Nutzungsguthaben. Um Ihre eigenen Ausgaben für Nutzungsguthaben anzuzeigen, führen Sie [`/usage`](/docs/de/costs#check-your-usage-credits-spend) aus. Informationen dazu, wo Ihre Organisation diese Ausgaben sieht, finden Sie unter [Claude für Teams und Enterprise](/docs/de/costs#claude-for-teams-and-enterprise).
* **Claude Console**: Ihre Organisation zahlt für den Schnellmodus zusammen mit dem Rest ihrer API-Nutzung. Wählen Sie auf den Console-Seiten [Usage](https://platform.claude.com/usage) und [Cost](https://platform.claude.com/cost) im Menü **Group by** die Option **Speed (Research Preview)** aus, um den Schnellmodus von der Standard-Geschwindigkeit zu trennen. Diese Option wird nur angezeigt, wenn der ausgewählte Datumsbereich die Nutzung des Schnellmodus umfasst.

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

* **Nur Anthropic API oder Abonnement**: Der Schnellmodus ist über die Anthropic Console API und für Claude-Abonnementpläne mit Nutzungsguthaben verfügbar. Er ist nicht auf Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry oder Claude Platform auf AWS verfügbar. Console-Organisationen müssen auch [Schnellmodus-Zugriff bereitgestellt haben](#enable-fast-mode-for-your-organization).
* **Nutzungsguthaben für Abonnementpläne aktiviert**: Bei einem Pro-, Max-, Team- oder Enterprise-Plan muss Ihr Konto [Nutzungsguthaben](/docs/de/costs#add-usage-credits-to-your-subscription) aktiviert haben, was eine Abrechnung über die in Ihrem Plan enthaltene Nutzung hinaus ermöglicht. Bis diese aktiviert sind, zeigt `/fast` „Fast mode requires usage credits" an. Wie Sie diese aktivieren, hängt von Ihrem Plan ab:
  * Bei Pro und Max aktivieren Sie diese im Abschnitt **Nutzungsguthaben** unter [**Einstellungen > Nutzung**](https://claude.ai/settings/usage) auf claude.ai, oder führen Sie `/usage-credits` aus, um diese Seite zu öffnen.
  * Bei Team und Enterprise aktiviert ein Mitglied mit Abrechnungszugriff diese für die Organisation unter [**Admin-Einstellungen > Nutzung**](https://claude.ai/admin-settings/usage), und ein Mitglied ohne Zugriff führt `/usage-credits` aus, um die Administratoren der Organisation um eine Anfrage zu bitten.

<Note>
  Die Nutzung des Schnellmodus wird direkt von Nutzungsguthaben abgerechnet, auch wenn Sie noch Nutzung in Ihrem Plan haben.
</Note>

* **Bezahlte Console-Organisation**: Claude Console-Konten verwenden keine Nutzungsguthaben, und Ihre Organisation zahlt für den Schnellmodus pro Token zusammen mit dem Rest ihrer API-Nutzung. Im kostenlosen Evaluierungsplan der Console zeigt `/fast` „Fast mode unavailable during evaluation. Please purchase credits." an. Um dies zu beheben, kaufen Sie Guthaben in Ihren [Console-Abrechnungseinstellungen](https://platform.claude.com/settings/billing).
* **Owner-Aktivierung für Teams und Enterprise**: Der Schnellmodus ist standardmäßig für Teams- und Enterprise-Organisationen deaktiviert. Ein Owner muss den Schnellmodus explizit [aktivieren](#enable-fast-mode-for-your-organization), bevor Benutzer darauf zugreifen können.

<Note>
  Vier Organisationseinstellungen können das Aktivieren des Schnellmodus mit `/fast` blockieren:

  * **Schnellmodus nicht aktiviert**: Wenn der Schnellmodus für Ihre Organisation nicht aktiviert wurde, zeigt das Aktivieren des Schnellmodus mit `/fast` „Fast mode has been disabled by your organization." an.
  * **Schnellmodus durch verwaltete Einstellungen deaktiviert**: Wenn Ihre Organisation [verwaltete Einstellungen](/docs/de/managed-settings) bereitstellt, die [`fastMode: false`](/docs/de/settings-reference#fastmode) setzen, zeigt das Aktivieren des Schnellmodus mit `/fast` die gleiche Meldung „Fast mode has been disabled by your organization" an.
  * **Opt-in pro Sitzung erforderlich**: Verwaltete Einstellungen, die [`fastModePerSessionOptIn: true`](#require-per-session-opt-in) setzen, lehnen `/fast on` mit der gleichen Meldung überall ab, außer in einer interaktiven Terminalsitzung.
  * **Schnellmodus-Modell nicht zulässig**: Wenn die [`availableModels`](/docs/de/model-config#restrict-model-selection)-Zulassungsliste Ihrer Organisation das Schnellmodus-Opus-Modell ausschließt, wird das Aktivieren mit „is not in your organization's allowed models" abgelehnt. In einer Sitzung, die bereits auf einem zulässigen Opus-Modell ausgeführt wird, das den Schnellmodus unterstützt, aktiviert `/fast` stattdessen den Schnellmodus auf Ihrem aktuellen Modell, ohne Modelle zu wechseln.
</Note>

<h3 id="enable-fast-mode-for-your-organization">
  Schnellmodus für Ihre Organisation aktivieren
</h3>

Wo Sie den Schnellmodus aktivieren, hängt davon ab, welches Produkt Ihre Organisation nutzt:

* **Console** (API-Kunden): Ein Administrator aktiviert ihn in [Claude Code-Einstellungen](https://platform.claude.com/claude-code/preferences). Der Schnellmodus befindet sich in [Forschungsvorschau](#research-preview), daher muss Ihre Organisation auch Schnellmodus-Zugriff bereitgestellt haben, bevor Schnellmodus-Anfragen erfolgreich sind. Um Zugriff zu erhalten, kontaktieren Sie Ihren Account Manager oder treten Sie der Warteliste bei, wie in [Schnellmodus auf der Claude API](https://platform.claude.com/docs/en/build-with-claude/fast-mode) beschrieben.

  Ohne bereitgestellten Zugriff lehnt die API jede Schnellmodus-Anfrage mit einem 429 ab, und Claude Code behandelt jede Ablehnung als [Schnellmodus-Ratenlimit](#handle-rate-limits). Im Gegensatz zu einer Ratenlimit-Abkühlung werden die Ablehnungen fortgesetzt, bis der Zugriff bereitgestellt wird.
* **Claude AI** (Teams und Enterprise): Ein Owner aktiviert ihn unter [Admin-Einstellungen > Claude Code](https://claude.ai/admin-settings/claude-code)

Eine weitere Option zum vollständigen Deaktivieren des Schnellmodus ist das Setzen von `CLAUDE_CODE_DISABLE_FAST_MODE=1`. Siehe [Umgebungsvariablen](/docs/de/env-vars).

<h3 id="use-fast-mode-behind-proxies-and-llm-gateways">
  Schnellmodus hinter Proxys und LLM-Gateways verwenden
</h3>

Bevor Claude Code den Schnellmodus anbietet, prüft es die Verfügbarkeit des Schnellmodus Ihrer Organisation mit einer direkten Anfrage an `api.anthropic.com`. Die Prüfung folgt nicht [`ANTHROPIC_BASE_URL`](/docs/de/llm-gateway-connect#set-the-base-url-and-credential), daher schlägt die Prüfung in einem Netzwerk fehl, das Claude-Verkehr durch ein [LLM-Gateway](/docs/de/llm-gateway) leitet und direkten Ausgang zu `api.anthropic.com` blockiert, obwohl Inferenzanfragen funktionieren. Die Prüfung verwendet einen konfigurierten [HTTP-Proxy](/docs/de/network-config#proxy-configuration), daher schlägt eine Netzwerkblockade die Prüfung nur dort fehl, wo `api.anthropic.com` auch über den Proxy nicht erreichbar ist.

Wenn die Prüfung fehlschlägt, meldet `/fast` „Fast mode unavailable due to network connectivity issues", und Anfragen werden mit Standardgeschwindigkeit ausgeführt, auch wenn Ihre Organisation den Schnellmodus aktiviert hat. Eine Prüfung, die in der Vergangenheit erfolgreich war, funktioniert weiterhin aus ihrem zwischengespeicherten Ergebnis, daher betrifft eine blockierte Prüfung hauptsächlich neue Installationen.

Die gleiche Konnektivitätsmeldung wird in einem offenen Netzwerk angezeigt, wenn die Prüfung `api.anthropic.com` erreicht, aber eine Anmeldedaten präsentiert, die Anthropic ablehnt. Eine Sitzung, deren aufgelöster Schlüssel eine vom Gateway ausgegebene Anmeldedaten ist, die in [`ANTHROPIC_API_KEY`](/docs/de/llm-gateway-connect#set-the-base-url-and-credential) gespeichert oder von einem [`apiKeyHelper`](/docs/de/settings-reference#apikeyhelper) erzeugt wird, sendet die Prüfung mit diesem Schlüssel, und die abgelehnte Anfrage wird als Konnektivitätsfehler gemeldet.

Um den Schnellmodus wiederherzustellen, erlauben Sie direkten Ausgang zu `api.anthropic.com`, wenn eine Netzwerkblockade die Ursache ist, oder setzen Sie die Variable, die dem Fehler der Prüfung entspricht:

* `CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS=1` behandelt eine fehlgeschlagene Prüfung als verfügbar und berücksichtigt immer noch eine „disabled by your organization"-Antwort. Verwenden Sie dies, wenn Ihr Netzwerk die Verbindung verweigert, oder wenn Anthropic eine Gateway-Anmeldedaten ablehnt; Whitelisting hilft nicht im Fall der Anmeldedaten, da nichts blockiert wird.
* `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK=1` überspringt die Prüfung vollständig. Verwenden Sie dies, wenn Ihr Netzwerk die Anfrage abfängt, anstatt sie zu verweigern.

Zwei Gateway-Konfigurationen melden „Fast mode has been disabled by your organization" anstelle der Konnektivitätsmeldung, auch wenn Ihre Organisation den Schnellmodus aktiviert hat:

* Eine Sitzung, die sich nur mit [`ANTHROPIC_AUTH_TOKEN`](/docs/de/llm-gateway-connect#set-the-base-url-and-credential) authentifiziert, überspringt die Prüfung: Ohne eine claude.ai-Anmeldung oder einen Anthropic API-Schlüssel und ohne eine zwischengespeicherte erfolgreiche Prüfung behandelt Claude Code den Schnellmodus als von Ihrer Organisation deaktiviert, ohne die Anfrage zu senden.
* Ein Proxy, der die Prüfung abfängt und mit seiner eigenen Seite antwortet, beispielsweise ein TLS-inspizierender Proxy, der eine HTTP 200-Blockierungsseite zurückgibt, wird als Antwort gelesen, die besagt, dass Ihre Organisation den Schnellmodus deaktiviert hat.

Setzen Sie in beiden Fällen `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK=1`, um den Schnellmodus wiederherzustellen. `CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS` gilt nicht für beide Fälle, da es nur fehlgeschlagene Prüfungen umgeht und beide eine deaktivierte Antwort erzeugen. Whitelisting direkten Ausgangs hilft nicht im Fall des Bearer-Tokens, der die Anfrage nie sendet.

Die Variablen beeinflussen nur die clientseitige Prüfung. Wenn Ihre Organisation den Schnellmodus deaktiviert hat, lehnt die API Schnellmodus-Anfragen ab, unabhängig davon, ob sie gesetzt sind. Eine Ablehnung von der API bleibt bestehen, auch wenn eine Skip-Variable gesetzt ist. Claude Code versucht die abgelehnte Anfrage mit Standardgeschwindigkeit erneut, schaltet den Schnellmodus aus, und `/fast` meldet, dass Ihre Organisation den Schnellmodus deaktiviert hat.

Das Setzen von `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` unterdrückt auch die Verfügbarkeitsprüfung. Ohne eine zuvor zwischengespeicherte erfolgreiche Prüfung meldet `/fast` „Fast mode is currently unavailable"; beide Skip-Variablen stellen den Schnellmodus in dieser Konfiguration auch wieder her.

<h3 id="require-per-session-opt-in">
  Opt-in pro Sitzung erforderlich
</h3>

Standardmäßig bleibt der Schnellmodus, den ein Benutzer in einer interaktiven Sitzung aktiviert, über Sitzungen hinweg erhalten. Um dies zu ändern, setzen Sie `fastModePerSessionOptIn` in einer beliebigen [Einstellungsdatei](/docs/de/settings#where-settings-live) auf `true`, was dazu führt, dass jede Sitzung mit deaktiviertem Schnellmodus beginnt und Benutzer ihn explizit mit `/fast` aktivieren müssen. Owners in [Team](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_teams#team-&-enterprise)- oder [Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_enterprise)-Plänen können dies organisationsweit über [servergesteuerte Einstellungen](/docs/de/server-managed-settings) bereitstellen.

```json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

Dies ist nützlich zur Kostenkontrolle in Organisationen, in denen Benutzer mehrere gleichzeitige Sitzungen ausführen. Die Schnellmodus-Einstellung des Benutzers wird immer noch gespeichert, daher stellt das Entfernen dieser Einstellung das standardmäßige persistente Verhalten wieder her.

Wenn verwaltete Einstellungen den Schlüssel setzen, funktioniert `/fast on` nur in einer interaktiven Terminalsitzung. Überall sonst, einschließlich [nicht-interaktivem Modus](/docs/de/headless), der [VS Code-Erweiterung](/docs/de/vs-code) und [Cloud-Sitzungen](#use-fast-mode-in-cloud-sessions), wird es mit einer Meldung abgelehnt, dass Ihre Organisation den Schnellmodus deaktiviert hat.

<h2 id="handle-rate-limits">
  Ratenlimits handhaben
</h2>

Der Schnellmodus hat separate Ratenlimits vom Standard-Opus. Alle unterstützten Opus-Modelle teilen sich einen Schnellmodus-Ratenlimit-Pool: Die Nutzung auf einem beliebigen dieser Modelle wird von den gleichen Limits abgezogen. Wenn Sie das Ratenlimit des Schnellmodus erreichen:

1. Der Schnellmodus fällt automatisch auf Standard-Geschwindigkeit auf
2. Das `↯`-Symbol wird grau, um die Abkühlung anzuzeigen
3. Sie arbeiten weiterhin mit Standard-Geschwindigkeit und -Preisen
4. Wenn die Abkühlung abläuft, wird der Schnellmodus automatisch wieder aktiviert

Um den Schnellmodus manuell zu deaktivieren, anstatt auf die Abkühlung zu warten, führen Sie `/fast` erneut aus.

Wenn Sie während einer Sitzung keine Nutzungsguthaben mehr haben, versucht Claude Code jeden abgelehnten Schnellmodus-Request mit Standard-Geschwindigkeit und -Preisen erneut, sodass Sie weiterarbeiten können und es keine Abkühlung gibt. Wie Sie die Ablehnung sehen, hängt vom Sitzungstyp ab:

* In einer interaktiven Sitzung zeigt Claude Code eine Benachrichtigung „Schnellmodus deaktiviert · Nutzungsguthaben aufgebraucht" an und schaltet den Schnellmodus für den Rest der Sitzung aus. Ihre gespeicherte Schnellmodus-Einstellung ändert sich nicht; führen Sie `/fast` aus, um den Schnellmodus wieder einzuschalten.
* Im [Headless-Modus](/docs/de/headless) mit `--output-format stream-json` und über das Agent SDK sendet Claude Code denselben Text im Message-Stream als `system`-Nachricht mit dem Subtyp `notification` einmal pro Turn, während Sie keine Nutzungsguthaben haben. Der Schnellmodus bleibt aktiviert. Erfordert Claude Code v2.1.221 oder später.

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
