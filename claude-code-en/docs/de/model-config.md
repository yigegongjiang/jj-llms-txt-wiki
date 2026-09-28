> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Modellkonfiguration

> Konfigurieren Sie, welches Modell Claude Code verwendet, Aufwandsstufen, erweiterten Kontext und das Auto-Compact-Fenster

<h2 id="available-models">
  Verfügbare Modelle
</h2>

Für die `model`-Einstellung in Claude Code können Sie entweder konfigurieren:

* Einen **Modell-Alias**
* Einen **Modellnamen**
  * Anthropic API: einen vollständigen **[Modellnamen](https://platform.claude.com/docs/en/about-claude/models/overview)**
  * Amazon Bedrock: ein Inference-Profil-ARN
  * Microsoft Foundry: einen Bereitstellungsnamen
  * Google Cloud's Agent Platform: einen Versionsnamen

Hinweise dazu, welches Modell und welche Aufwandsstufe für verschiedene Arten von Arbeit geeignet sind, finden Sie unter [Choosing a Claude model and effort level in Claude Code](https://claude.com/blog/claude-model-and-effort-level-in-claude-code) im Blog.

<Note>
  `ANTHROPIC_BASE_URL` ändert, wohin Anfragen gesendet werden, nicht welches Modell sie beantwortet. Um Claude durch ein LLM-Gateway zu leiten, siehe [LLM gateways](/docs/de/llm-gateway).
</Note>

<h3 id="model-aliases">
  Modell-Aliase
</h3>

Verwenden Sie einen Modell-Alias, um Modelleinstellungen auszuwählen, ohne sich genaue Versionsnummern merken zu müssen:

| Modell-Alias     | Verhalten                                                                                                                                                                                                                                                                                                                                                                |
| ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **`default`**    | Spezialwert, der jeden Modell-Override löscht und auf den [Laufzeit-Standard für Ihr Konto](#default-model-setting) zurückgesetzt wird. Ist selbst kein Modell-Alias                                                                                                                                                                                                     |
| **`best`**       | Verwendet das Modell, zu dem der Alias [`fable`](#fable-alias-resolution) aufgelöst wird, wo Fable für Sie verfügbar ist, andernfalls das gleiche Modell wie `opus`                                                                                                                                                                                                      |
| **`fable`**      | Verwendet das [Fable-Modell für Ihren Anbieter](#fable-alias-resolution) für Ihre schwierigsten und längsten Aufgaben                                                                                                                                                                                                                                                    |
| **`sonnet`**     | Verwendet das neueste Sonnet-Modell für tägliche Codierungsaufgaben                                                                                                                                                                                                                                                                                                      |
| **`opus`**       | Verwendet das neueste Opus-Modell für komplexe Reasoning-Aufgaben                                                                                                                                                                                                                                                                                                        |
| **`haiku`**      | Verwendet das schnelle und effiziente Haiku-Modell für einfache Aufgaben                                                                                                                                                                                                                                                                                                 |
| **`sonnet[1m]`** | Verwendet Sonnet mit einem [1-Million-Token-Kontextfenster](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) für lange Sitzungen. Keine Auswirkung, wenn `sonnet` bereits zu Sonnet 5 mit seinem nativen 1M-Fenster aufgelöst wird; hinter einem [LLM-Gateway](/docs/de/llm-gateway) wählt es das 1M-Fenster für Sonnet 5 |
| **`opus[1m]`**   | Verwendet Opus mit einem [1-Million-Token-Kontextfenster](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) für lange Sitzungen                                                                                                                                                                                       |
| **`opusplan`**   | Spezialmodell, das `opus` während des Plan Mode verwendet und dann zu `sonnet` für die Ausführung wechselt                                                                                                                                                                                                                                                               |

Die Version, zu der die Aliase `opus` und `sonnet` aufgelöst werden, hängt vom Anbieter ab:

| Anbieter                                             | `opus`   | `sonnet`   |
| :--------------------------------------------------- | :------- | :--------- |
| Anthropic API                                        | Opus 5.5 | Sonnet 5   |
| [Claude Platform on AWS](/docs/de/claude-platform-on-aws) | Opus 5.5 | Sonnet 4.6 |
| Amazon Bedrock, Google Cloud's Agent Platform        | Opus 5.5 | Sonnet 4.5 |
| Microsoft Foundry                                    | Opus 4.6 | Sonnet 4.5 |

<span id="fable-alias-resolution" />

Sofern Sie `ANTHROPIC_DEFAULT_FABLE_MODEL` nicht setzen, wird der Alias `fable` zu Fable 5.1 aufgelöst, außer in [Claude apps gateway](/docs/de/claude-apps-gateway)-Sitzungen, wo `fable` und `best` zu Fable 5 aufgelöst werden. Vor v2.1.257 wurde `fable` auf jedem Anbieter zu Fable 5 aufgelöst.

Ein Gateway, das nicht konfiguriert ist, um `claude-fable-5-1` bereitzustellen, lehnt Anfragen für dieses Modell ab. Um Fable 5.1 über ein Gateway zu verwenden, das es bereitstellt, wählen Sie es mit `/model claude-fable-5-1` aus.

Wenn ein Alias zu einem älteren Modell aufgelöst wird, sind neuere Modelle verfügbar, indem Sie den vollständigen Modellnamen explizit auswählen oder `ANTHROPIC_DEFAULT_OPUS_MODEL` oder `ANTHROPIC_DEFAULT_SONNET_MODEL` setzen.

Vor v2.1.280 wurde `opus` zu Opus 5 auf der Anthropic API, Claude Platform on AWS, Amazon Bedrock und Google Cloud's Agent Platform ab v2.1.219 aufgelöst. Vor v2.1.219 wurde `opus` zu Opus 4.8 auf der Anthropic API ab v2.1.154 und auf Claude Platform on AWS, Amazon Bedrock und Google Cloud's Agent Platform ab v2.1.207 aufgelöst. Vor v2.1.207 wurde `opus` zu Opus 4.7 auf Claude Platform on AWS und zu Opus 4.6 auf Amazon Bedrock und Google Cloud's Agent Platform aufgelöst.

Aliase verweisen auf die empfohlene Version für Ihren Anbieter und werden im Laufe der Zeit aktualisiert. Um eine bestimmte Version zu fixieren, verwenden Sie den vollständigen Modellnamen, z. B. `claude-opus-5-5`, oder setzen Sie die entsprechende Umgebungsvariable wie `ANTHROPIC_DEFAULT_OPUS_MODEL`.

<Note>
  Opus 5.5 erfordert Claude Code v2.1.280 oder später. Opus 5 erfordert v2.1.219 oder später. Sonnet 5 erfordert v2.1.197 oder später. Führen Sie `claude update` aus, um zu aktualisieren.
</Note>

<h3 id="work-with-fable">
  Mit Fable arbeiten
</h3>

[Claude Fable 5.1](https://platform.claude.com/docs/en/about-claude/models/overview) und Claude Fable 5 sind die leistungsfähigsten Modelle in Claude Code, geeignet für Aufgaben, die größer als eine einzelne Sitzung sind. Sie halten lange autonome Sitzungen, untersuchen vor dem Handeln und überprüfen ihre Arbeit häufiger als kleinere Modelle. Fable 5.1 ist die neuere Version.

Keines der Fable-Modelle ist der Standard für den Kontotyp auf einem Plan oder Anbieter. Wählen Sie eines explizit aus:

* **Fable 5.1**: Führen Sie `/model fable` aus, oder starten Sie mit `claude --model fable`. In [Claude apps gateway](/docs/de/claude-apps-gateway)-Sitzungen, wo der Alias zu Fable 5 aufgelöst wird, führen Sie stattdessen `/model claude-fable-5-1` aus.
* **Fable 5**: Wählen Sie es nach Modell-ID aus. Auf der Anthropic API führen Sie `/model claude-fable-5` aus oder starten Sie mit `claude --model claude-fable-5`. Bei anderen Anbietern verwenden Sie die Fable-5-Modell-ID Ihres Anbieters oder [fixieren Sie sie](#pin-models-for-third-party-deployments) mit `ANTHROPIC_DEFAULT_FABLE_MODEL`.

Wenn Sie sich direkt mit der Anthropic API verbinden und Ihre Benutzereinstellungen `claude-fable-5` oder `claude-fable-5[1m]` als Modell enthalten, z. B. weil Sie Fable in der `/model`-Auswahl vor v2.1.257 ausgewählt haben, ändert Claude Code diesen gespeicherten Wert beim ersten Ausführen von v2.1.257 oder später zu dem Alias `fable` oder `fable[1m]`, und die Startmodellzeile zeigt `(auto-updated)` einmal an. Ein `claude-fable-5`-Wert in Projekt-, lokalen oder verwalteten Einstellungen wird unverändert gelassen.

Anfragen, die die Sicherheitsklassifizierer eines Fable-Modells kennzeichnen, meist in Cybersicherheits- und Biologie-Bereichen, lösen [automatisches Modell-Fallback](#automatic-model-fallback) aus.

Um das Beste aus Fable herauszuholen:

* **Beschreiben Sie das Ergebnis, nicht die Schritte**: Geben Sie ihm das Ergebnis, das Sie möchten, und lassen Sie es den Weg planen. Um es auf dieses Ergebnis hinzuarbeiten, [setzen Sie ein Ziel](/docs/de/goal).
* **Geben Sie ihm mehrdeutige Probleme**: Root-Cause-Untersuchungen, Ausfalldebugging und Architekturentscheidungen sind Bereiche, in denen die zusätzliche Untersuchung und Überprüfung sich auszahlt.
* **Überspringen Sie die Überprüfungserinnerungen**: Es überprüft seine eigene Arbeit mit weniger Aufforderungen, daher sind Erinnerungen zum Testen oder Überprüfen normalerweise unnötig.
* **Größere Aufgaben einschätzen**: Geben Sie ihm Arbeit, die Sie normalerweise in Teile aufteilen würden. Es hält lange Sitzungen, ohne den Faden zu verlieren.

<Note>
  Fable 5.1 erfordert Claude Code v2.1.257 oder später. Wenn eine Anfrage dafür von einer älteren Version fehlschlägt, siehe [Claude Code does not support this model](/docs/de/errors#claude-code-does-not-support-this-model). Führen Sie `claude update` aus, um zu aktualisieren. Zur Verfügbarkeit unter Zero Data Retention siehe [Model availability under ZDR](/docs/de/zero-data-retention#model-availability-under-zdr).
</Note>

Auf der Anthropic API wird ein Fable-Modell in der `/model`-Auswahl aufgelistet, es sei denn, [`availableModels`](#restrict-model-selection) oder [Organisations-Modellbeschränkungen](#organization-model-restrictions) schließen es aus. Wenn Ihre Organisation Fable überhaupt nicht verwenden kann, z. B. unter [Zero Data Retention](/docs/de/zero-data-retention#model-availability-under-zdr), bleibt die Zeile in der Auswahl ausgegraut, mit einer Notiz darüber, warum.

<h4 id="fable-and-usage-credits">
  Fable und Nutzungsguthaben
</h4>

Je nach Ihrem Plan und Seat-Tier kann die Fable-Nutzung zu [Nutzungsguthaben](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) abgerechnet werden, anstatt auf die in Ihrem Plan enthaltenen Limits zu ziehen. Wenn dies der Fall ist, zeigt die `/model`-Auswahl „Requires usage credits" in der Fable-Zeile an. Um Nutzungsguthaben zu verwalten, siehe [Add usage credits to your subscription](/docs/de/costs#add-usage-credits-to-your-subscription).

In interaktiven Sitzungen zeigt Claude Code eine Zustimmungsaufforderung an, bevor eine Fable-Anfrage Nutzungsguthaben abrechnet. Mitglieder von Enterprise-Plänen mit Organisationsabrechnung sehen die Aufforderung nicht. Sie können auf Fable mit Nutzungsguthaben fortfahren oder zu Ihrem Standardmodell wechseln. Sie können die Aufforderung auch ablehnen:

* In der `/model`-Auswahl behalten Sie Ihr aktuelles Modell.
* Während der Sitzung setzt Claude Code den Zug auf Ihrem Standardmodell fort.

Nachdem Sie sich entschieden haben, auf Fable mit Nutzungsguthaben fortzufahren, zeigt Claude Code die Aufforderung nicht erneut an.

In einer Sitzung mit verbundenem [Remote Control](/docs/de/remote-control), einer [Hintergrund-Sitzung](/docs/de/agent-view) oder einer Sitzung eines [Agent-Team](/docs/de/agent-teams)-Teamkollegen kann niemand am Terminal sein, daher hält Claude Code die Zustimmungsaufforderung während der Sitzung bis zur [`dialogExpiry`](/docs/de/settings-reference#dialogexpiry)-Frist, standardmäßig fünf Minuten. Wenn bis zur Frist niemand geantwortet hat, beendet Claude Code den Zug ohne Senden der Anfrage und fügt eine Notiz zum Transkript hinzu, die der Remote-Control-Client auch anzeigt. Ihre Modellauswahl ist unverändert, und Claude Code fragt beim nächsten Mal nach Zustimmung.

Was Sie tun können, während die Aufforderung wartet, hängt von der Sitzung ab:

* Mit verbundenem Remote Control oder in einer Teamkollegen-Sitzung drücken Sie eine beliebige Taste am Terminal, um die Frist zu stornieren, und Claude Code wartet auf Ihre Antwort.
* In einer Hintergrund-Sitzung antworten Sie vor der Frist.
* Wenn Sie eine neue Nachricht vom Remote-Client senden, bevor jemand am Terminal eingegeben hat, beendet Claude Code den Zug auf die gleiche Weise, und Ihre neue Nachricht startet den nächsten Zug. Nachdem jemand am Terminal eingegeben hat, wartet Claude Code weiter auf die Antwort und reiht Ihre neue Nachricht dahinter ein.

Im [nicht-interaktiven Modus](/docs/de/headless) mit dem `-p`-Flag und über das Agent SDK zeigt Claude Code die Zustimmungsaufforderung nie an. Wenn eine Fable-Anfrage dort zu Nutzungsguthaben abgerechnet würde, rechnet Claude Code sie ohne Nachfrage ab.

<h3 id="setting-your-model">
  Ihr Modell einstellen
</h3>

Sie können Ihr Modell auf mehrere Arten konfigurieren, aufgelistet nach Priorität:

1. **Während der Sitzung**: Verwenden Sie `/model <alias|name>`, um sofort zu wechseln, oder führen Sie `/model` ohne Argument aus, um die Auswahl zu öffnen. Siehe [wenn Claude Code Sie auffordert, den Wechsel zu bestätigen](/docs/de/prompt-caching#switching-models)
2. **Beim Start**: Starten Sie mit `claude --model <alias|name>`
3. **Umgebungsvariable**: Setzen Sie `ANTHROPIC_MODEL=<alias|name>`
4. **Einstellungen**: Konfigurieren Sie dauerhaft in Ihrer Einstellungsdatei mit dem Feld `model`
5. **[Standard für neue Sitzungen](#set-a-default-model-for-new-sessions)**: Setzen Sie `ANTHROPIC_DEFAULT_MODEL=<alias|name>`

`/model` speichert Ihre Auswahl als Standard für neue Sitzungen, indem das Feld `model` in Ihren Benutzereinstellungen geschrieben wird. In der Auswahl:

* `Enter`: Modell wechseln und als Standard speichern
* `s`: Modell nur für diese Sitzung wechseln und Ihren Standard unverändert lassen. Um eine andere Taste zu verwenden, binden Sie [`modelPicker:thisSessionOnly`](/docs/de/keybindings#model-picker-actions) neu.

Die direkte Eingabe von `/model <name>` verhält sich wie `Enter`. Um nur für diese Sitzung zu wechseln, öffnen Sie die Auswahl mit `/model` und drücken Sie `s` auf der Zeile des Modells.

Wenn Sie Modelle mit `/model` wechseln, erreicht der Wechsel auch [Subagenten, die das Modell der Hauptkonversation erben](/docs/de/sub-agents#choose-a-model), da Claude Code ihr Modell aus dem auflöst, das Ihre Sitzung verwendet, wenn Claude sie startet. Wechseln Sie zu Opus, bevor Claude Forschung oder Test-Läufe an einen von ihnen delegiert, und diese Arbeit läuft auch auf Opus. Um einen benutzerdefinierten Subagenten auf einem kleineren Modell zu halten, setzen Sie `model` in seiner Definition.

Wenn Sie ein Modell mit `/model` im [nicht-interaktiven Modus](/docs/de/headless) mit dem `-p`-Flag setzen, gilt Ihre Auswahl nur für die aktuelle Sitzung und wird nicht als Standard gespeichert; `/model` in diesem Modus erfordert Claude Code v2.1.205 oder später. Projekt- und verwaltete Einstellungen haben weiterhin Vorrang und werden beim nächsten Start erneut angewendet. Ein [Organisations-Standard-Modell](#organization-default-model), das Ihr Administrator konfiguriert hat, um die Benutzerauswahl zu überschreiben, wird auch beim nächsten Start erneut angewendet.

In v2.1.144 bis v2.1.152 galt `/model` nur für die aktuelle Sitzung und `d` in der Auswahl speicherte einen Standard.

Das Flag `--model` und die Umgebungsvariable `ANTHROPIC_MODEL` gelten nur für die Sitzung, mit der Sie sie starten. Um verschiedene Modelle in verschiedenen Terminals gleichzeitig auszuführen, starten Sie jedes mit seinem eigenen `--model`-Flag, anstatt mit `/model` zu wechseln.

Preise in der `/model`-Auswahl werden angezeigt, wenn Claude Code mit der Anthropic API spricht, direkt oder über ein [LLM-Gateway](/docs/de/llm-gateway), das es proxiert, und der Preis in einer Zeile ist der Preis des Modells, das diese Zeile auswählt. Bei [Drittanbieter-Anbietern](/docs/de/third-party-integrations) wie Amazon Bedrock und beim [Claude apps gateway](/docs/de/claude-apps-gateway) bestimmt Ihr Anbieter oder Gateway, was Sie zahlen, daher zeigen Auswahl-Zeilen keinen Preis. Der Preis ist nur ein Anzeigelabel; er beeinflusst nicht, welches Modell eine Zeile auswählt oder was Ihr Anbieter abrechnet. Vor v2.1.206 zeigten [Claude Platform on AWS](/docs/de/claude-platform-on-aws) und Gateway-Sitzungen Anthropic-Listenpreise, und eine Zeile könnte den Preis eines anderen Modells als dem, das sie auswählte, anzeigen.

Wiederaufgenommene Sitzungen, die mit `claude --resume`, `--continue` oder der `/resume`-Auswahl gestartet wurden, behalten das Modell, das sie beim Speichern des Transkripts verwendeten, unabhängig von der aktuellen `model`-Einstellung. Wenn das wiederhergestellte Modell eingestellt wurde oder von [`availableModels`](#restrict-model-selection) ausgeschlossen wird, fällt die Sitzung auf die normale Prioritätsreihenfolge zurück. Dies verhindert, dass die `/model`-Auswahl einer anderen Sitzung das Modell beim Fortsetzen ändert. Bei Anbietern, die anbieter-spezifische Bereitstellungs-IDs anstelle von Anthropic-Modell-IDs verwenden, wie Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry, wird das Transkript-Modell überhaupt nicht wiederhergestellt und die Sitzung löst ihr Modell durch die normale Prioritätsreihenfolge auf.

Ein Modell, das Sie für den neuen Start mit `--model` oder `ANTHROPIC_MODEL` auswählen, hat weiterhin Vorrang vor dem wiederhergestellten Modell. Ab v2.1.195 gilt dies auch für eine [`ANTHROPIC_DEFAULT_OPUS_MODEL`](#environment-variables)-Familienvariable. [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions) kann dies auch unter den in seinem Abschnitt aufgelisteten Bedingungen tun.

Wenn das aktive Modell beim Start aus Projekt- oder verwalteten Einstellungen statt aus Ihrer eigenen Auswahl stammt, zeigt der Start-Header, welche Einstellungsdatei es gesetzt hat. Führen Sie `/model` aus, um zu überschreiben; die Projekt- oder verwaltete Einstellung wird beim nächsten Start erneut angewendet. Auf Plattformen, die Claude Code einbetten und [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/de/env-vars) setzen, hat die Modellkonfiguration des Hosts Vorrang vor verwalteten Modelleinstellungen, während eine verwaltete `availableModels`-Zulassungsliste in Kraft bleibt, es sei denn, der Host liefert seine eigene; [Exceptions to managed settings precedence](/docs/de/settings#exceptions-to-managed-settings-precedence) sagt, welche Schlüssel und Variablen der Host überschreibt.

Wenn Sie oder Ihre Organisation [PreModelSwitch-Hooks](/docs/de/hooks#premodelswitch) konfigurieren, werden sie ausgeführt, bevor ein angefordeter Wechsel angewendet wird, und können ihn blockieren oder Sie auffordern, ihn zu bestätigen.

Wenn Claude Code nicht feststellen kann, welche PreModelSwitch-Hooks Ihre Organisation's [verwaltete Plugins](/docs/de/settings-reference#enabledplugins) bereitstellen, z. B. weil ein verwaltetes Plugin nicht geladen werden konnte, lehnt es den Wechsel ab, anstatt ihn unkontrolliert anzuwenden, und prüft ihn bei jedem neuen Versuch erneut. Siehe [Model switch was blocked by a PreModelSwitch hook](/docs/de/errors#model-switch-was-blocked-by-a-premodelswitch-hook) für die Nachricht und Wiederherstellung.

Wenn Sie Modelle über die [Agent SDK](/docs/de/agent-sdk/overview)-Methode `setModel()` wechseln oder von einem Gerät, das über [Remote Control](/docs/de/remote-control) verbunden ist, oder eine App wie die [Desktop-App](/docs/de/desktop), die die Claude Code CLI für Sie ausführt, Claude Code wechselt, prüft Claude Code, dass die Zeichenkette eine ist, die es erkennt, bevor es sie speichert. Diese Prüfung erfordert Claude Code v2.1.200 oder später. Das Überprüfen einer Remote-Control-Auswahl erfordert Claude Code v2.1.260 oder später auf Ihrem Computer. Auf der Anthropic API erkennt Claude Code:

* einen Modell-Alias
* einen Eintrag aus der `/model`-Auswahl
* jeden Namen, der mit `claude-` beginnt
* einen Wert, den Sie selbst als [benutzerdefinierte Modelloption](#add-a-custom-model-option) oder in [`modelOverrides`](#override-model-ids-per-version) konfiguriert haben

Claude Code lehnt eine unbekannte Zeichenkette mit `Model "<name>" is not a recognized model id.` ab und die Sitzung behält ihr aktuelles Modell, anstatt die Zeichenkette zu speichern und beim nächsten Request fehlzuschlagen. Siehe [die Fehlerreferenz](/docs/de/errors#model-is-not-a-recognized-model-id) für Wiederherstellungsschritte.

Die Prüfung läuft nur auf der Anthropic API. Bei Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, [Claude Platform on AWS](/docs/de/claude-platform-on-aws) und hinter einem [LLM-Gateway](/docs/de/llm-gateway) oder einem benutzerdefinierten `ANTHROPIC_BASE_URL` definiert Ihr Anbieter oder Gateway die Modellnamen, daher gibt Claude Code jede Zeichenkette ohne Prüfung durch. Die Prüfung deckt auch nicht das Flag `--model`, die Umgebungsvariable `ANTHROPIC_MODEL` oder die Einstellung `model` ab; ein Tippfehler dort erzeugt [There's an issue with the selected model](/docs/de/errors#theres-an-issue-with-the-selected-model) beim ersten Request statt. Claude Code kann immer noch die [unrecognized-model-Diagnosezeile](/docs/de/errors#unrecognized-model-id-on-a-request) zur Request-Zeit auf jedem Anbieter schreiben.

Wenn das angeforderte Modell ein geplantes Pensionierungsdatum hat oder automatisch zu einer neueren Version neu zugeordnet wird, zeigt Claude Code eine Warnung an, die das angeforderte Modell benennt. Interaktive Sitzungen zeigen es als Start-Notiz. Ab v2.1.182 wird die gleiche Warnung im [nicht-interaktiven Modus](/docs/de/headless) bei Verwendung des Standard-Textausgabeformats auf stderr geschrieben. Die Prüfung deckt auch ein in [Subagent-Frontmatter](/docs/de/sub-agents) gesetztes `model` ab. Die stderr-Warnung wird für `--output-format json` und `stream-json` unterdrückt; lesen Sie das tatsächliche Modell stattdessen aus dem Feld `modelUsage` der [Ergebnisnachricht](/docs/de/headless#get-structured-output).

Beispiel: Starten Sie eine Sitzung auf Opus:

```bash theme={null}
claude --model opus
```

Wechseln Sie dann Modelle innerhalb der Sitzung:

```text theme={null}
/model sonnet
```

Beispiel-Einstellungsdatei:

```json theme={null}
{
    "permissions": {
        "allow": ["Bash(npm run lint)"]
    },
    "model": "opus"
}
```

<h4 id="set-a-default-model-for-new-sessions">
  Einen Standard-Modell für neue Sitzungen setzen
</h4>

Setzen Sie `ANTHROPIC_DEFAULT_MODEL=<alias|name>`, um das Modell auszuwählen, auf dem Ihre Sitzungen standardmäßig starten. Erfordert Claude Code v2.1.236 oder später.

Claude Code startet eine neue Sitzung auf dem Modell der Variablen nur, wenn keines dieser Modelle auswählt:

* Das Flag `--model`
* `ANTHROPIC_MODEL`
* Ein `model`-Wert in einer beliebigen Einstellungsdatei, einschließlich der Auswahl, die Sie mit `/model` speichern
* Ein [Organisations-Standard-Modell](#organization-default-model)

Eine Auswahl, die Sie mit `/model` speichern, hat auch bei späteren Starts Vorrang vor der Variablen. Mit stattdessen gesetztem `ANTHROPIC_MODEL` kehrt Claude Code beim nächsten Start zu dem Modell der Variablen zurück, unabhängig davon, was Sie mit `/model` gespeichert haben.

Claude Code löst auch die Option „Standard" zu dem Modell der Variablen auf, es sei denn, es gilt ein Organisations-Standard-Modell. Wenn die Option „Standard" zu dem Modell der Variablen aufgelöst wird, zeigt die Zeile „Standard" in der `/model`-Auswahl das Label „Set by ANTHROPIC\_DEFAULT\_MODEL".

Claude Code ignoriert die Variable in diesen Fällen, und die Option „Standard" wird aufgelöst, als hätten Sie sie nicht gesetzt:

* Sie setzen sie auf `default`, `inherit`, `opusplan` oder `haiku`
* [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) ist aktiviert
* [`availableModels`](#restrict-model-selection) oder [Organisations-Modellbeschränkungen](#organization-model-restrictions) schließen das Modell aus
* Das Modell ist nicht für Ihr Konto verfügbar

Wenn eine neue Sitzung auf dem Modell der Variablen starten würde, startet auch eine Sitzung, die Sie mit `claude --resume`, `--continue` oder der `/resume`-Auswahl fortsetzen, auf ihm. Claude Code stellt das in dem Transkript dieser Sitzung gespeicherte Modell nicht wieder her. Andernfalls verwendet Claude Code die Variable nicht, wenn Sie [eine Sitzung fortsetzen](#setting-your-model).

<h4 id="a-new-session-starts-on-a-different-model-than-you-picked">
  Eine neue Sitzung startet auf einem anderen Modell als Sie ausgewählt haben
</h4>

Wenn Sie ein Modell mit `/model` auswählen und Ihre nächste Sitzung auf etwas anderem startet, sind dies die üblichen Ursachen:

* **Sie haben es für eine Sitzung ausgewählt.** Das Drücken von `s` in der Auswahl, das Starten mit `--model` und das Ausführen von `/model` im nicht-interaktiven Modus gelten alle für die aktuelle Sitzung und lassen Ihren gespeicherten Standard unverändert.
* **Etwas mit höherer Priorität setzt das Modell.** Ein `model`-Wert in Projekt- oder verwalteten Einstellungen, `ANTHROPIC_MODEL` in Ihrer Shell oder ein [Organisations-Standard](#organization-default-model), den Ihr Administrator gesetzt hat, um Benutzerauswahlen zu überschreiben, gilt wieder bei jedem Start. Ihre `/model`-Auswahl ist immer noch gespeichert; sie wird übertroffen. Wenn Projekt- oder verwaltete Einstellungen das Modell setzen, benennt der Start-Header die Datei.
* **Claude Code konnte Ihre Auswahl nicht speichern.** `/model` schreibt `model` zu `~/.claude/settings.json`. Wenn Sie nicht in diese Datei schreiben können, z. B. weil ein anderes Tool sie generiert oder mit einer schreibgeschützten Kopie verlinkt, dauert das Modell, das Sie ausgewählt haben, für die Sitzung an und der nächste Start liest den alten Wert. Setzen Sie `model` in dem Tool, das die Datei generiert, oder machen Sie die Datei beschreibbar. Siehe [A change you made in Claude Code is lost in new sessions](/docs/de/settings#a-change-you-made-in-claude-code-is-lost-in-new-sessions).
* **Sie haben eine Sitzung fortgesetzt.** Eine Sitzung, die Sie mit `claude --resume` oder `--continue` fortsetzen, [behält normalerweise das Modell, das sie verwendete](#setting-your-model), anstatt Ihren aktuellen Standard.

<h2 id="restrict-model-selection">
  Modellauswahl einschränken
</h2>

Unternehmensadministratoren können `availableModels` in [verwalteten oder Richtlinieneinstellungen](/docs/de/managed-settings) verwenden, um einzuschränken, welche Modelle Benutzer auswählen können. Einträge entsprechen einer Modellfamilie wie `sonnet`, einem Versionspräfix wie `claude-sonnet-4-5` oder einer vollständigen Modell-ID wie `claude-sonnet-4-5-20250929`. Ein Versionspräfix entspricht auch späteren Modell-IDs, die es um ein weiteres Segment erweitern, sodass `claude-fable-5` sowohl Fable 5 als auch Fable 5.1 zulässt, während `claude-fable-5-1` nur Fable 5.1 zulässt.

Auf Plattformen, die Claude Code einbetten und [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/de/env-vars) setzen, hat die Modellkonfiguration des Hosts Vorrang vor verwalteten Modelleinstellungen, während eine verwaltete `availableModels`-Allowlist in Kraft bleibt, es sei denn, der Host stellt seine eigene bereit; [Ausnahmen von der Vorrangigkeit verwalteter Einstellungen](/docs/de/settings#exceptions-to-managed-settings-precedence) gibt an, welche Schlüssel und Variablen der Host überschreibt.

Wenn `availableModels` gesetzt ist, gilt die Allowlist überall dort, wo ein Benutzer ein Modell angeben kann:

* **Hauptsitzungsmodell**: `/model`, das Flag `--model`, die Umgebungsvariable `ANTHROPIC_MODEL`, die Einstellung `model`, [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions) und das Modell, das beim [Fortsetzen einer Sitzung](#setting-your-model) wiederhergestellt wird
* **Alias-Auflösung**: die Umgebungsvariablen `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, `ANTHROPIC_DEFAULT_HAIKU_MODEL` und `ANTHROPIC_DEFAULT_FABLE_MODEL` können einen zulässigen Alias nicht zu einem Modell außerhalb der Liste umleiten
* **Schnellmodus**: `/fast` weigert sich umzuschalten, wenn dies implizit zu einem Opus-Modell außerhalb der Liste führen würde, mit der Meldung „is not in your organization's allowed models"
* **Subagent- und Teamkollegen-Modelle**: das Feld `model` in [Subagent](/docs/de/sub-agents#choose-a-model)-Frontmatter, der Parameter `model` des Agent-Tools, [Agent-Team](/docs/de/agent-teams#specify-teammates-and-models)-Teamkollegen-Modelle, `CLAUDE_CODE_SUBAGENT_MODEL` und in v2.1.197 und früher das Modellauswahlfeld im Assistenten `/agents`&#x20;
* **Skill- und Befehlsmodelle**: das Frontmatter `model` in [Skills und Befehlen](/docs/de/skills)
* **Advisor-Modell**: die konfigurierte Einstellung [`advisorModel`](/docs/de/advisor) und das Flag `--advisor`
* **Hintergrund-Agent-Modell**: das Modell, das in der [Dispatch-Auswahl](/docs/de/agent-view) ausgewählt ist

Auf der Anthropic API und [Claude Platform on AWS](/docs/de/claude-platform-on-aws) wird ein Modellfamilien-Alias, `opus`, `sonnet`, `haiku` oder `fable`, zu seinem üblichen Modell aufgelöst, wenn die Allowlist dieses Modell zulässt. Wenn die Allowlist dieses Modell blockiert, ersetzt Claude Code es durch die neueste Version der Familie, die die Allowlist zulässt, und zeigt einen Hinweis an, der sowohl das angeforderte als auch das ersetzte Modell benennt. Mit `["sonnet", "claude-opus-4-6"]` wählen beispielsweise sowohl `/model opus` als auch `--model opus` Claude Opus 4.6, das neueste zulässige Opus. Vor v2.1.205 wurde ein Alias, dessen neueste veröffentlichte Version außerhalb der Liste lag, wie jede andere blockierte Auswahl abgelehnt oder ersetzt, auch wenn die Liste eine ältere Version zulässt.

Die Ersetzung benötigt eine zulässige Version zum Landen: Wenn die Allowlist keine Version der Aliasfamilie zulässt, folgt der Alias dem Ablehnungs- und Ersetzungsverhalten unten wie jeder andere blockierte Wert.

Claude Code behandelt jede andere blockierte Auswahl je nachdem, wo das Modell gesetzt wurde:

* **`/model`**: Claude Code lehnt den Wechsel mit einem Fehler ab
* **Flag `--model`, `ANTHROPIC_MODEL` oder die Einstellung `model`**: Claude Code ersetzt den Wert beim Start durch eine Warnung, die sowohl das angeforderte als auch das ersetzte Modell benennt, und die Sitzung startet auf dem Standardmodell
* **[`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions)**: Claude Code ignoriert die Variable
* **Subagent- oder Teamkollegen-Überschreibung**: Claude Code führt den Subagent oder Teamkollegen auf einem Fallback-Modell aus, anstatt die Anfrage fehlschlagen zu lassen. Siehe [Modell auswählen](/docs/de/sub-agents#choose-a-model) für den Subagent-Fallback und [Teamkollegen und Modelle angeben](/docs/de/agent-teams#specify-teammates-and-models) für den Teamkollegen-Fallback.

  In interaktiven Sitzungen warnt Sie Claude Code, wenn es ein Subagent-Modell durch diesen Fallback oder durch die oben beschriebene Ersetzung der neuesten zulässigen Version ersetzt, und benennt die angeforderten und ersetzten Modelle; es meldet keinen Teamkollegen-Fallback.

  Wo die oben beschriebene Ersetzung der neuesten zulässigen Version gilt, folgt ein blockierter Familien-Alias stattdessen. Vor v2.1.222 fiel ein Alias auf jedem Provider wie jeder andere blockierte Wert zurück
* **Skill- oder Befehlsüberschreibung**: Claude Code ignoriert die Überschreibung, einschließlich eines blockierten Familien-Alias, und der Skill oder Befehl wird auf dem Sitzungsmodell ausgeführt. Ein Skill oder Befehl, der [in einem Subagent ausgeführt wird](/docs/de/skills#run-skills-in-a-subagent), folgt stattdessen dem Subagent-Verhalten oben
* **Einstellung `advisorModel`**: der Advisor ist für die Sitzung deaktiviert
* **Flag `--advisor`**: Claude Code beendet sich beim Start mit einem Fehler. In einer [Hintergrund-Sitzung](/docs/de/agent-view) startet es die Sitzung ohne den Advisor, anstatt sich zu beenden

Claude Code verbirgt ausgeschlossene Modelle vor der Auswahl `/model`. Eine vollständige Modell-ID in der Liste, die keine integrierte Auswahlzeile hat, wie eine ältere Version, die die Liste festlegt, erscheint in der Auswahl `/model` als eigene beschriftete Zeile, es sei denn, Claude Code ersetzt die integrierten Optionen durch eine [`modelPicker`](/docs/de/settings-reference#modelpicker)-Aufstellung. Vor v2.1.199 war eine solche ID nur durch Eingabe von `/model <id>` auswählbar.

Modelländerungen, die Claude Code in Ihrem Namen vornimmt, werden auf die gleiche Weise überprüft:

* **[Fallback-Modellketten](#fallback-model-chains)**: Einträge außerhalb der Allowlist werden gelöscht
* **Plan-Modus-Upgrades**: Auf der Anthropic API und Claude Platform on AWS verwendet ein Upgrade wie [`opusplan`](#opusplan-model-setting) zu einem ausgeschlossenen Modell die neueste zulässige Version der Upgrade-Familie. Bei Providern mit Provider-spezifischen Modell-IDs und wenn keine Version zulässig ist, wird das Upgrade übersprungen und die Planung wird auf dem Sitzungsmodell fortgesetzt
* **[Automatischer Modell-Fallback](#automatic-model-fallback)**: Ein Fallback, dessen Ziel ausgeschlossen ist, wird nicht ausgeführt, sodass die gekennzeichnete Anfrage mit einer Ablehnung endet
* **[Auto-Modus-Klassifizierer](/docs/de/permission-modes#eliminate-prompts-with-auto-mode)**: der Standard Claude Sonnet 5 des Klassifizierers gilt nur, wenn die Allowlist Sonnet 5 zulässt. Wenn es ausgeschlossen ist, wird der Klassifizierer auf dem Sitzungsmodell ausgeführt, das die Allowlist bereits regelt, oder auf einem Opus-Modell, wenn die Sitzung auf einem [Fable-Modell](#work-with-fable) läuft. Bei Providern außer der Anthropic API wird dieser Opus-Fallback auf dem Standard-Opus-Modell des Providers ausgeführt, ohne die Allowlist zu konsultieren. Erfordert Claude Code v2.1.210 oder später
* **[Schnellmodus](/docs/de/fast-mode)**: Das Aktivieren des Schnellmodus wird abgelehnt, wenn das Modell, auf dem die Sitzung danach läuft, außerhalb der Allowlist liegt

```json theme={null}
{
  "availableModels": ["sonnet", "haiku"]
}
```

<h3 id="surface-coverage">
  Oberflächenabdeckung
</h3>

Jede Oberfläche erzwingt die Allowlist, die sie erhält. Welcher Liefermechanismus jede Oberfläche erreicht, unterscheidet sich:

| Liefermechanismus                                                                    | CLI und IDE | Desktop-Lokalsitzungen | Web-, Mobil- und Cloud-Sitzungen                                                                                                                                                                                                                                                 | Agent SDK und nicht-interaktiv | Cowork                       |
| :----------------------------------------------------------------------------------- | :---------- | :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------- | :--------------------------- |
| [Server-verwaltete Einstellungen](/docs/de/server-managed-settings) aus der Admin-Konsole | Erzwungen   | Erzwungen              | Erzwungen                                                                                                                                                                                                                                                                        | Erzwungen                      | Nicht bereitgestellt         |
| [MDM oder verwaltete Einstellungsdateien](/docs/de/managed-settings#delivery-mechanisms)  | Erzwungen   | Erzwungen              | Nicht bereitgestellt in von Anthropic gehosteten Umgebungen; in [selbst gehosteten Umgebungen](/docs/de/self-hosted-environments) erzwungen aus dem Runner-Image gemäß [wie Claude Code verwaltete Quellen kombiniert](/docs/de/managed-settings#how-claude-code-combines-managed-sources) | Erzwungen                      | Erzwungen, wo bereitgestellt |

* Cloud-Sitzungen auf [Claude Code im Web](/docs/de/claude-code-on-the-web), einschließlich derjenigen, die Sie aus der Desktop-App starten, laufen standardmäßig auf von Anthropic verwalteten VMs: Einstellungen, die auf Ihrem Gerät bereitgestellt werden, erreichen sie nicht, daher stellen Sie die Allowlist über server-verwaltete Einstellungen bereit. Sitzungen, die Ihre Organisation zu einer [selbst gehosteten Umgebung](/docs/de/self-hosted-environments) leitet, laufen auf Ihrem eigenen Compute und lesen auch die verwaltete Einstellungsdatei im Runner-Image. [Wie Claude Code verwaltete Quellen kombiniert](/docs/de/managed-settings#how-claude-code-combines-managed-sources) gibt an, wann diese Datei gilt. Ein Modellwechsel in der Mitte einer Cloud-Sitzung wird abgelehnt, wenn das angeforderte Modell durch die Allowlist ausgeschlossen ist. Wenn die `availableModels`-Liste in Ihren server-verwalteten Einstellungen nicht leer ist, lehnt der Server eine Anfrage des Benutzers ab, eine Cloud-Sitzung auf einem Modell zu starten, das die Liste ausschließt.
* Cowork, die agentengesteuerte Arbeit-Registerkarte in der Claude Desktop-App, führt ihre Sitzungen auf Claude Code aus, erhält aber absichtlich keine server-verwalteten Einstellungen aus der Admin-Konsole von claude.ai. Eine verwaltete Einstellungsdatei gilt für Cowork-Sitzungen, wenn sie dort vorhanden ist, wo die Sitzung läuft; Remote-Cowork-Sitzungen laufen auf von Anthropic verwalteten VMs, wo eine auf dem Gerät bereitgestellte Datei nicht vorhanden ist.
* Sitzungen auf [Drittanbieter-Providern](/docs/de/server-managed-settings#platform-availability) wie Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry und [Claude Platform on AWS](/docs/de/claude-platform-on-aws) erhalten keine server-verwalteten Einstellungen, daher stellen Sie die Allowlist dort über MDM oder verwaltete Einstellungsdateien bereit.
* Server-verwaltete Bereitstellung erfordert auch, dass sich die Sitzung mit einem [zulässigen Login oder Schlüssel](/docs/de/server-managed-settings#platform-availability) authentifiziert. Flotten, die Schlüssel nur über ein [`apiKeyHelper`](/docs/de/settings-reference#apikeyhelper)-Skript generieren, sollten die Allowlist über MDM oder verwaltete Einstellungsdateien bereitstellen.
* Die Desktop-Code-Registerkarte hostet auch [SSH-Sitzungen](/docs/de/desktop#ssh-sessions), die die verwaltete Einstellungsdatei vom Remote-Host lesen, auf dem sie laufen. Siehe [Desktop-verwaltete Einstellungen](/docs/de/desktop#managed-settings).
* Die Modellauswahlfelder auf claude.ai und in der Desktop-App verbergen oder grau aus Modelle, die durch die Allowlist Ihrer Organisation ausgeschlossen sind. Der Auswahlzustand ist eine Bequemlichkeit für Benutzer; die Erzwingung erfolgt in der Sitzung.

<h3 id="default-model-behavior">
  Standardmodell-Verhalten
</h3>

Für sich allein lässt `availableModels` die Option Standard auf dem [Laufzeit-Standard](#default-model-setting) des Systems für das Konto, bis Sie auch [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) setzen. Wenn dieser Standard ein Modell ist, das Sie einschränken möchten, setzen Sie auch `enforceAvailableModels`.

Ein leeres `availableModels`-Array aktiviert niemals die Erzwingung des Standardmodells: Mit `availableModels: []` werden benannte Modellauswahlen blockiert, aber das Standardmodell für den Kontotyp bleibt unabhängig von `enforceAvailableModels` nutzbar.

<h3 id="enforce-the-allowlist-for-the-default-model">
  Allowlist für das Standardmodell erzwingen
</h3>

Setzen Sie `enforceAvailableModels: true` zusammen mit einem nicht leeren `availableModels` in verwalteten Einstellungen, um die Allowlist auf die Option Standard zu erweitern. Dies erfordert Claude Code v2.1.175 oder später.

```json theme={null}
{
  "availableModels": ["sonnet", "haiku"],
  "enforceAvailableModels": true
}
```

Die Option Standard wird zu dem Kontotyp-Standard aufgelöst, oder zu dem [Organisationsstandardmodell](#organization-default-model), wenn ein Administrator eines gesetzt hat. Wenn dieses Modell nicht in der Allowlist ist, wird die Option Standard stattdessen zu dem ersten `availableModels`-Eintrag aufgelöst, der ein zulässiges, verfügbares Modell benennt, und die Zeile Standard der Auswahl `/model` zeigt dieses Modell. Dies gilt überall dort, wo der Standard erreicht wird: Sitzungsstart, Auswahl von Standard in `/model`, das Schlüsselwort `"default"` in [Fallback-Modellketten](#fallback-model-chains) und der Fallback, der verwendet wird, wenn eine ausgeschlossene Auswahl gelöscht wird.

`enforceAvailableModels` ordnet die Option Standard nur neu zu, wenn `availableModels` nicht leer ist. Mit `availableModels: []` bleibt das Standardmodell für den Kontotyp nutzbar, sodass die Einstellung Benutzer nicht von jedem Modell ausschließen kann. Wenn `availableModels` nicht leer ist, aber kein Eintrag zu einem zulässigen und verfügbaren Modell aufgelöst wird, wird die Erzwingung übersprungen und Standard wird zu dem Kontotyp-Standard aufgelöst, mit einer Warnung, die nur unter `--debug` sichtbar ist. Behalten Sie mindestens einen garantiert verfügbaren Eintrag in der Liste, um dies zu vermeiden.

Stellen Sie beide Schlüssel zusammen in der höchstrangigen verwalteten Quelle bereit, die Sie bereitstellen. Standardmäßig liest Claude Code nur diese Quelle, daher wird ein Paar, das in einer verwalteten Einstellungsdatei platziert ist, ignoriert, wenn die Admin-Konsole Einstellungen bereitstellt; unter dem Opt-in-Merge in [wie Claude Code verwaltete Quellen kombiniert](/docs/de/managed-settings#how-claude-code-combines-managed-sources) ignoriert Claude Code immer noch eine `modelOverrides`-Map aus einer Quelle, die unter der Quelle rangiert, die `availableModels` setzt.

<h3 id="control-the-model-users-run-on">
  Kontrollieren Sie das Modell, auf dem Benutzer laufen
</h3>

Die Einstellung `model` ist eine anfängliche Auswahl, keine Erzwingung. Sie legt fest, welches Modell aktiv ist, wenn eine Sitzung startet, aber Benutzer können immer noch `/model` öffnen und Standard auswählen, das zu dem [Laufzeit-Standard](#default-model-setting) des Systems aufgelöst wird, unabhängig davon, was `model` gesetzt ist, es sei denn, [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) leitet es um.

Um die Modellerfahrung vollständig zu kontrollieren, kombinieren Sie diese Einstellungen:

* **`availableModels`**: schränkt ein, welche benannten Modelle Benutzer wechseln können
* **`enforceAvailableModels`**: erweitert die `availableModels`-Allowlist auf die Option Standard, sodass Standard nicht zu einem Modell außerhalb der Liste aufgelöst werden kann
* **`model`**: legt die anfängliche Modellauswahl fest, wenn eine Sitzung startet
* **`ANTHROPIC_DEFAULT_SONNET_MODEL`** / **`ANTHROPIC_DEFAULT_OPUS_MODEL`** / **`ANTHROPIC_DEFAULT_HAIKU_MODEL`** / **`ANTHROPIC_DEFAULT_FABLE_MODEL`**: kontrollieren, zu welchen Modellen die Aliase `sonnet`, `opus`, `haiku` und `fable` aufgelöst werden, und welche Version der [Kontotyp-Standard](#default-model-setting) verwendet

Dieses Beispiel startet Benutzer auf Sonnet 4.5, begrenzt die Auswahl auf Sonnet und Haiku und stellt sicher, dass Standard zu einem Modell in der Allowlist aufgelöst wird, anstatt zum Tier-Standard:

```json theme={null}
{
  "model": "claude-sonnet-4-5",
  "availableModels": ["claude-sonnet-4-5", "haiku"],
  "enforceAvailableModels": true,
  "env": {
    "ANTHROPIC_DEFAULT_SONNET_MODEL": "claude-sonnet-4-5"
  }
}
```

Ohne `enforceAvailableModels` oder den `env`-Block erhält ein Benutzer, der Standard in der Auswahl auswählt, den [Laufzeit-Standard](#default-model-setting) anstelle der in `model` festgelegten Version. Die beiden Einstellungen decken unterschiedliche Bereiche ab: `enforceAvailableModels` macht Standard der Allowlist gehorchen, während der `env`-Block festlegt, zu welcher Version ein zulässiger Alias wie `sonnet` aufgelöst wird. Verwenden Sie `enforceAvailableModels` allein, wenn das Einschränken von Modellfamilien ausreicht; fügen Sie den `env`-Block hinzu, wenn Sie auch eine bestimmte Version festlegen müssen.

<h3 id="merge-behavior">
  Merge-Verhalten
</h3>

Wenn die verwalteten Einstellungen, die Claude Code anwendet, `availableModels` definieren, gilt diese Liste allein, abgesehen von einer [Host-Plattform, die ihre eigene bereitstellt](/docs/de/settings#exceptions-to-managed-settings-precedence): Einträge in Benutzer-, Projekt- oder lokalen Einstellungen können sie nicht erweitern, und Claude Code führt `availableModels` auch nie über verwaltete Quellen hinweg zusammen; [wie Claude Code verwaltete Quellen kombiniert](/docs/de/managed-settings#how-claude-code-combines-managed-sources) gibt an, welche Quellenliste gilt. Andernfalls werden Listen aus Benutzer-, Projekt- und lokalen Einstellungen wie andere Array-Einstellungen [verkettet und dedupliziert](/docs/de/settings#settings-precedence). Vor Claude Code v2.1.175 wurden Einträge aus Bereichen mit niedrigerer Priorität in die verwaltete Liste zusammengeführt, anstatt sie zu ersetzen.

Innerhalb der effektiven Liste deaktiviert ein Eintrag, der ein bestimmtes Modell in einer Familie benennt, ob ein Versionspräfix oder eine vollständige Modell-ID, den Wildcard-Eintrag dieser Familie: `["sonnet", "claude-sonnet-4-5"]` erlaubt nur Sonnet 4.5-Versionen, nicht jedes Sonnet-Modell.

<h3 id="mantle-model-ids">
  Mantle-Modell-IDs
</h3>

Wenn der [Amazon Bedrock Mantle-Endpunkt](/docs/de/amazon-bedrock#use-the-mantle-endpoint) aktiviert ist, werden Einträge in `availableModels`, die mit `anthropic.` beginnen, als benutzerdefinierte Optionen zur Auswahl `/model` hinzugefügt und zum Mantle-Endpunkt weitergeleitet. Dies ist eine Ausnahme von der Alias-Zuordnung, die in [Modelle für Drittanbieter-Bereitstellungen festlegen](#pin-models-for-third-party-deployments) beschrieben ist. Die Einstellung schränkt die Auswahl immer noch auf aufgelistete Einträge ein, und eine Mantle-ID bettet einen Familiennamen ein, daher zählt sie als spezifischer Eintrag und deaktiviert den Wildcard dieser Familie: Neben allen Mantle-IDs listen Sie die Versionspräfixe oder vollständigen IDs auf, die Sie auswählbar behalten möchten. Siehe [Merge-Verhalten](#merge-behavior).

<h3 id="organization-model-restrictions">
  Organisationsmodellbeschränkungen
</h3>

Organisationsadministratoren in Claude Enterprise-Plänen schränken ein, welche Modelle Mitglieder ausführen können, indem sie einzelne Modelle in der Admin-Konsole von claude.ai deaktivieren. Diese Beschränkung wird mit den Berechtigungen des Kontos bereitgestellt, wenn Claude Code sich authentifiziert, getrennt von jeder `availableModels`-Liste in Einstellungen, und der Server erzwingt die gleiche Beschränkung unabhängig, wenn eine Sitzung erstellt wird. Erfordert Claude Code v2.1.187 oder später.

Die Beschränkung gilt, wenn sich ein Mitglied anmeldet oder seinen eigenen API-Schlüssel verwendet. Organisationsbezogene Anmeldedaten, wie Organisationsdienst-Schlüssel, sind nicht an einen Benutzer gebunden, daher gilt die Beschränkung nicht für sie.

Die Claude Console hat keine Modellbeschränkungskontrolle. Organisationen ohne Claude Enterprise-Plan, einschließlich derjenigen, deren Mitglieder sich über die Anthropic API authentifizieren, schränken Modelle stattdessen mit [`availableModels`](#restrict-model-selection) in [verwalteten Einstellungen](/docs/de/managed-settings) ein und fügen [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) hinzu, um die Option Standard abzudecken. [Oberflächenabdeckung](#surface-coverage) gibt an, wie jede Oberfläche diese Einstellungen empfängt und erzwingt.

Ein eingeschränktes Modell ist in der Auswahl `/model` verborgen. Wenn Sie es mit `--model`, der Umgebungsvariable `ANTHROPIC_MODEL` oder der Einstellung `model` nach Name auswählen, wird die Meldung `Model "<name>" is restricted by your organization's settings. Using <model> instead.` angezeigt und die Sitzung startet auf einem zulässigen Modell. Wenn Sie `/model <name>` für ein eingeschränktes Modell eingeben, wird es mit `Model '<name>' is restricted by your organization's settings. Run /model to choose a different model.` abgelehnt und die Sitzung behält ihr aktuelles Modell.

Ein [Modellfamilien-Alias](#restrict-model-selection) wie `opus` wird zu seinem üblichen Modell aufgelöst, wenn die Organisation es zulässt. Wenn die Organisation dieses Modell einschränkt, ersetzt Claude Code es durch die neueste Version der Familie, die die Organisation zulässt, mit dem gleichen Ersetzungshinweis. `/model <alias>` wird nur abgelehnt, wenn jede Version seiner Familie eingeschränkt ist; ein Alias, der mit `--model`, `ANTHROPIC_MODEL` oder der Einstellung `model` gesetzt ist, wird in diesem Fall immer noch beim Start ersetzt. Vor v2.1.205 wurde ein Familien-Alias basierend auf seiner neuesten veröffentlichten Version allein ersetzt oder abgelehnt, auch wenn eine ältere Version zulässig war.

Beschränkungen gelten organisationsweit oder pro Rolle:

* Das Deaktivieren eines Modells auf Organisationsebene entfernt es für jedes Mitglied.
* Rollenbezogener Zugriff gewährt verschiedenen benutzerdefinierten Rollen unterschiedliche Modelle, und ein Mitglied, das mehrere Rollen innehat, kann jedes Modell verwenden, das eine seiner Rollen gewährt.
* Haiku-Modelle sind immer verfügbar und können nicht deaktiviert werden, daher behält jedes Mitglied mindestens ein nutzbares Modell.
* Eine Zugangsänderung tritt bei neuen Anfragen innerhalb von etwa einer Minute in Kraft; die Auswahl `/model` spiegelt dies wider, wenn eine Sitzung das nächste Mal startet.

Beide Beschränkungen gelten zusammen: Ein Modell ist nur auswählbar, wenn es durch `availableModels` zulässig ist und nicht durch die Organisation eingeschränkt ist. Organisationsbeschränkungen erreichen Sitzungen nur auf der Anthropic API und [LLM-Gateway](/docs/de/llm-gateway)-Bereitstellungen; bei jedem anderen Provider verwenden Sie stattdessen `availableModels`.

<h2 id="organization-default-model">
  Organisationsstandardmodell
</h2>

Organisationsadministratoren in Claude Enterprise-Plänen können ein Standardmodell für Claude Code-Mitglieder über die claude.ai-Administrationskonsole festlegen – für die gesamte Organisation oder pro benutzerdefinierte Rolle. Wenn ein Standardmodell festgelegt ist, wird die Option „Standard" zu diesem Modell aufgelöst. Erfordert Claude Code v2.1.196 oder später.

Die Zeile „Standard" in der `/model`-Auswahl zeigt den Namen des Organisationsstandardmodells mit der Bezeichnung „Org-Standard". Die Bezeichnung lautet „Org-Standard", unabhängig davon, ob der Administrator den Standard für die gesamte Organisation oder für Ihre Rolle festgelegt hat. Ein Rollenstandard gilt für Mitglieder dieser benutzerdefinierten Rolle und hat Vorrang vor dem organisationsweiten Standard. Wenn mehrere Ihrer Rollen unterschiedliche Standards festlegen, wird das leistungsfähigste Modell angewendet.

Das Organisationsstandardmodell ist ein Ausgangspunkt, keine Einschränkung. Diese Auswahlmöglichkeiten haben Vorrang vor ihm:

* das Flag `--model` und die Umgebungsvariable `ANTHROPIC_MODEL`
* ein `model`-Wert in [verwalteten Einstellungen](/docs/de/managed-settings) oder bereitgestellt über `--settings`
* ein `model`-Wert in Ihren Benutzer-, Projekt- oder lokalen Einstellungen, einschließlich eines Modells, das Sie mit `/model` speichern

Administratoren können auch das Organisationsstandardmodell so konfigurieren, dass es die Benutzerauswahl überschreibt. Wenn die Überschreibung aktiviert ist, hat sie Vorrang vor dem `model`-Wert in Benutzer-, Projekt- und lokalen Einstellungen. Ein Modell, das Sie mit `/model` speichern, gilt also für die aktuelle Sitzung, und das Organisationsstandardmodell wird beim nächsten Start wieder angewendet. Wenn sich Ihre Auswahl unterscheidet, zeigt `/model` `Your organization's default (<model>) applies on restart` an. Das Flag `--model`, `ANTHROPIC_MODEL`, verwaltete Einstellungen und `--settings` haben auch bei aktivierter Überschreibung weiterhin Vorrang. Die Überschreibung ist für einen begrenzten Satz von Organisationen verfügbar. Fragen Sie Ihr Anthropic-Kontoteam nach der Verfügbarkeit.

Um einzuschränken, welche Modelle Mitglieder auswählen können, verwenden Sie stattdessen [Organisationsmodellbeschränkungen](#organization-model-restrictions) oder [`availableModels`](#restrict-model-selection).

Claude Code liest das Organisationsstandardmodell einmal beim Start ein. Ein Standard, den der Administrator während einer Sitzung ändert, wird also beim nächsten Start wirksam.

Wenn das Organisationsstandardmodell die Benutzerauswahl nicht überschreibt, löscht der erste interaktive Start nach der Änderung durch den Administrator den `model`-Schlüssel aus Ihren Benutzereinstellungen einmalig, damit der neue Standard angewendet wird. Es ändert nichts anderes in der Datei, und ein Modell, das Sie nach diesem Start mit `/model` speichern, wird beibehalten.

Das Organisationsstandardmodell durchläuft diese Beschränkungsprüfungen, bevor es übernommen wird:

* [`availableModels`](#restrict-model-selection) allein gilt nicht für das Organisationsstandardmodell, daher wird ein Organisationsstandardmodell außerhalb der Zulassungsliste trotzdem angewendet. Wenn auch [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) festgelegt ist, wird ein Organisationsstandardmodell außerhalb der Zulassungsliste wie jedes andere Standardmodell zum ersten Eintrag der Zulassungsliste neu zugeordnet
* ein Organisationsstandardmodell, das [Organisationsmodellbeschränkungen](#organization-model-restrictions) für Ihr Konto verweigern, wird durch das neueste zulässige Modell in seiner Familie ersetzt, oder durch eine kostengünstigere Familie, wenn jede Version davon eingeschränkt ist
* ein Organisationsstandardmodell, das für Ihr Konto überhaupt nicht verfügbar ist, wird übersprungen, und die Option „Standard" wird aufgelöst, wie es [ohne ein Organisationsstandardmodell](#default-model-setting) der Fall wäre

Ab v2.1.199 behält die `/model`-Auswahl eine separate Zeile für die übliche Familie bei, wenn das Organisationsstandardmodell eine andere Modellfamilie ist als die übliche Standard des Kontotyps, sodass Sie für eine Sitzung trotzdem zu ihr wechseln können. In v2.1.196 bis v2.1.198 fehlt diese Zeile in der Auswahl.

Das Organisationsstandardmodell gilt nur für Sitzungen, die mit der Anthropic API authentifiziert sind. Um einen Standard an anderer Stelle festzulegen, einschließlich [LLM-Gateway](/docs/de/llm-gateway)-Bereitstellungen, verwenden Sie stattdessen den `model`-Schlüssel in [verwalteten Einstellungen](/docs/de/managed-settings).

<h2 id="organization-effort-limits">
  Organisatorische Anstrengungsgrenzen
</h2>

Ihre Organisation kann die [Anstrengungsstufe](#adjust-effort-level) auf zwei Wege begrenzen. Bei einem Claude Enterprise-Plan legen Organisationsadministratoren Anstrengungsgrenzen pro Rolle fest, wie unten beschrieben. Bei jedem Plan und jedem Anbieter, einschließlich Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry, begrenzt die verwaltete Einstellung [`maxEffortLevel`](/docs/de/settings-reference#maxeffortlevel) die Anstrengung stattdessen auf dem Client. Wenn beide auf ein Modell zutreffen, gilt die niedrigere Obergrenze.

Organisationsadministratoren in Claude Enterprise-Plänen können eine maximale [Anstrengungsstufe](#adjust-effort-level) pro Modell für jede benutzerdefinierte Rolle festlegen, zusammen mit Rollenbeschränkungen auf [Organisationsmodellebene](#organization-model-restrictions). Stufen über der Obergrenze werden nicht in der `/effort`-Auswahl angeboten, und das Benennen einer höheren Stufe mit `--effort` oder `/effort` wird stattdessen mit der Obergrenze ausgeführt. In interaktiven Sitzungen und einfachen Text-`--print`-Läufen warnt eine Meldung vor den angeforderten und angewendeten Stufen; bei `json`- oder `stream-json`-Ausgabe oder in Hintergrund-Agenten wird die Begrenzung stillschweigend angewendet. Obergrenzen gelten pro Modell, daher kann der Wechsel von Modellen ändern, welche Stufen verfügbar sind. Wenn mehrere Ihrer Rollen dasselbe Modell gewähren, gilt die am wenigsten restriktive Obergrenze. Erfordert Claude Code v2.1.195 oder später.

Anstrengungsgrenzen werden zusammen mit [Organisationsmodellbeschränkungen](#organization-model-restrictions) bereitgestellt und erreichen dieselben Sitzungen.

<h2 id="special-model-behavior">
  Spezielles Modellverhalten
</h2>

<h3 id="default-model-setting">
  `default` Modelleinstellung
</h3>

Das Verhalten von `default` hängt von Ihrem Kontotyp ab:

* **Pro, Max, Team, Enterprise und Anthropic API**: Standard ist Opus 5.5
* **Claude Platform auf AWS, Amazon Bedrock und Google Cloud's Agent Platform**: Standard ist Opus 5.5
* **Microsoft Foundry**: Standard ist Sonnet 4.5

Vor v2.1.280 wurde `default` auf Pro und Team Standard zu Sonnet 5 aufgelöst und auf Max, Team Premium, Enterprise, der Anthropic API, Claude Platform auf AWS, Amazon Bedrock und Google Cloud's Agent Platform ab v2.1.219 zu Opus 5. Vor v2.1.219 wurde `default` auf der Anthropic API zu Opus 4.8 aufgelöst, auf Max, Team Premium und Enterprise Pay-as-you-go ab v2.1.154 und auf Claude Platform auf AWS, Amazon Bedrock und Google Cloud's Agent Platform ab v2.1.207. Vor v2.1.207 wurde `default` auf Claude Platform auf AWS zu Opus 4.7 aufgelöst und auf Amazon Bedrock und Google Cloud's Agent Platform zu Sonnet 4.5.

Wenn ein Administrator ein [Organisationsstandardmodell](#organization-default-model) festgelegt hat, wird `default` stattdessen zu diesem Modell aufgelöst, anstatt zum oben genannten Kontotyp-Standard. Erfordert Claude Code v2.1.196 oder später. `default` kann auch zu dem Modell aufgelöst werden, das Sie mit [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions) festlegen, unter den in dessen Abschnitt aufgelisteten Bedingungen.

Wenn verwaltete Einstellungen [die Zulassungsliste für das Standardmodell erzwingen](#enforce-the-allowlist-for-the-default-model) und der Kontotyp-Standard nicht in `availableModels` enthalten ist, wird `default` zum erzwungenen Standard aufgelöst, anstatt zum oben genannten Kontotyp-Standard. Wenn beide zutreffen, ersetzt der Organisationsstandard zuerst den Kontotyp-Standard und die Erzwingung wird dann darauf angewendet: Ein auf der Zulassungsliste befindlicher Organisationsstandard wird beibehalten, während einer außerhalb der Liste zum erzwungenen Standard aufgelöst wird.

Fable-Modelle sind auf keinem Plan oder Provider der Kontotyp-Standard. Wenn Sie eines mit `/model` auswählen, wird es als das ausgewählte Modell in Ihren Benutzereinstellungen gespeichert, sodass spätere Sitzungen damit beginnen. Für die einmalige Änderung, die Claude Code an einer gespeicherten Fable 5-Auswahl in v2.1.257 vornimmt, siehe [Mit Fable arbeiten](#work-with-fable).

<h3 id="opusplan-model-setting">
  `opusplan` Modelleinstellung
</h3>

Der `opusplan` Modellalias bietet einen automatisierten Hybrid-Ansatz:

* **Im Plan-Modus**: verwendet `opus` für komplexe Überlegungen und Architekturentscheidungen
* **Im Ausführungsmodus**: wechselt automatisch zu `sonnet` für Code-Generierung und Implementierung

Dies verbindet Opus's Überlegungen für die Planung mit Sonnets Effizienz für die Ausführung.

Die Plan-Modus-Opus-Phase verwendet das gleiche Kontextfenster wie die `opus` Modelleinstellung, und die Ausführungsphase verwendet das gleiche Fenster wie `sonnet`. Wenn `opus` und `sonnet` zu Modellen aufgelöst werden, die standardmäßig mit dem [1M-Kontextfenster](#extended-context) laufen, wie die aktuellen Modelle auf der Anthropic API, werden beide Phasen damit ausgeführt. Um 1M-Kontext für beide Phasen anzufordern, wo sie nicht vorhanden sind, [setzen Sie das Modell](#setting-your-model) auf `opusplan[1m]`, zum Beispiel mit `/model opusplan[1m]`. Das Setzen mit `/model` erfordert Claude Code v2.1.265 oder später; verwenden Sie auf früheren Versionen stattdessen das `--model` Flag oder die `model` Einstellung.

Wenn [`availableModels`](#restrict-model-selection) das neueste Opus ausschließt, aber eine ältere Version zulässt, zum Beispiel `["sonnet", "claude-opus-4-6"]`, verwendet `opusplan` das neueste zulässige Opus für die Planung und bleibt nur bei Sonnet, wenn jedes Opus ausgeschlossen ist. Eine Haiku-Sitzung, die normalerweise im Plan-Modus zu Sonnet aktualisiert würde, verwendet ebenfalls das neueste zulässige Sonnet und bleibt nur bei Haiku, wenn jedes Sonnet ausgeschlossen ist. Vor v2.1.205 blieb der Plan-Modus beim Modell der Sitzung, wenn die neueste Version der Upgrade-Familie ausgeschlossen war, auch wenn die Zulassungsliste eine ältere Version zuließ.

Die Substitution einer älteren zulässigen Version gilt auf der Anthropic API und [Claude Platform auf AWS](/docs/de/claude-platform-on-aws). Auf Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry und Mantle, deren Bereitstellungen Provider-spezifische Modell-IDs verwenden, bleibt der Plan-Modus beim Modell der Sitzung, wenn das Upgrade-Modell ausgeschlossen ist.

Für einen Hybrid-Ansatz, bei dem Claude während einer Aufgabe entscheidet, wann ein zweites Modell konsultiert werden soll, anstatt an der Plan-Grenze zu wechseln, siehe das [Advisor-Tool](/docs/de/advisor).

<h3 id="fallback-model-chains">
  Fallback-Modellketten
</h3>

Wenn das primäre Modell überlastet ist, nicht verfügbar ist oder einen anderen nicht wiederholbaren Serverfehler zurückgibt, kann Claude Code zu einem Fallback-Modell wechseln, anstatt die Anfrage fehlschlagen zu lassen. Authentifizierungs-, Abrechnungs-, Rate-Limit-, Anfragegröße- und Transportfehler sowie eine [Ablehnung durch die Richtlinienprüfung Ihrer Organisation](/docs/de/errors#automatic-retries) lösen niemals einen Wechsel aus; diese folgen ihrer normalen Wiederholung und Fehlerbehandlung.

Konfigurieren Sie ein oder mehrere Fallback-Modelle und Claude Code versucht sie der Reihe nach, wobei eine Benachrichtigung angezeigt wird, wenn es wechselt. Der Wechsel dauert nur für den aktuellen Zug, sodass Ihre nächste Nachricht zuerst wieder das primäre Modell versucht. Claude Code begrenzt Ketten auf drei Modelle nach Duplikatentfernung und ignoriert zusätzliche Einträge.

Legen Sie eine Kette für eine Sitzung mit dem `--fallback-model` Flag fest, das eine kommagetrennte Liste akzeptiert:

```bash theme={null}
claude --fallback-model sonnet,haiku
```

Um eine Kette über Sitzungen hinweg beizubehalten, setzen Sie `fallbackModel` in [Einstellungen](/docs/de/settings) als Array:

```json theme={null}
{
  "fallbackModel": ["claude-sonnet-5", "claude-haiku-4-5"]
}
```

Das `--fallback-model` Flag hat Vorrang vor der `fallbackModel` Einstellung. Jeder Eintrag akzeptiert einen Modellnamen oder Alias, und `"default"` wird zum Standardmodell erweitert.

Claude Code bestätigt die Kette beim Start nicht und `/status` zeigt sie nicht an. Die Benachrichtigung, die angezeigt wird, wenn ein Wechsel stattfindet, ist das erste sichtbare Zeichen, dass ein Fallback konfiguriert ist.

Wenn eine Anfrage fehlschlägt, versucht Claude Code jeden Eintrag der Reihe nach, bis einer ihn akzeptiert. Ein Eintrag, der auch nicht erreichbar ist, wie ein in Einstellungen angeheftetes pensioniertes Modell, schlägt auf die gleiche Weise zum nächsten fehl. Claude Code entfernt zwei Arten von Einträgen, bevor dieser Durchgang beginnt:

* **Außerhalb der Zulassungsliste**: Claude Code löscht jeden Eintrag, der nicht von [`availableModels`](#restrict-model-selection) zulässig ist, wenn es die Kette liest.
* **Kleineres Kontextfenster während der Komprimierung**: die Kette deckt auch [Komprimierung](/docs/de/context-window#what-survives-compaction) ab, aber Claude Code wird nicht zu einem Modell mit einem kleineren Kontextfenster als das des primären zurückfallen, da die Zusammenfassung dort zuerst einen Teil des Gesprächs abschneiden würde. Wenn jedes Fallback kleiner ist, zeigt die Komprimierung den ursprünglichen Fehler an und Sie können erneut versuchen.

Claude Code wendet die Kette auch auf [Subagenten](/docs/de/sub-agents) an. Wenn die Anfrage eines Subagenten fehlschlägt, versucht Claude Code Ihre konfigurierten Fallback-Modelle der Reihe nach, und der Subagent wird auf dem Modell fortgesetzt, das die Anfrage akzeptiert. Das Modell Ihrer Sitzung bleibt unverändert. Vor v2.1.247 endete ein Fehler, den die Kette abdeckte, den Subagenten.

<h3 id="automatic-model-fallback">
  Automatisches Modell-Fallback
</h3>

Dieser Abschnitt behandelt inhaltsbasiertes Fallback von Fable-Modellen, Opus 5.5 und Opus 5. Für verfügbarkeitsbasiertes Fallback, wenn ein Modell überlastet oder nicht verfügbar ist, siehe [Fallback-Modellketten](#fallback-model-chains).

Fable-Modelle, Opus 5.5 und Opus 5 werden mit Sicherheitsklassifizierern ausgeführt, die am häufigsten Cybersicherheits- und Biologie-Inhalte kennzeichnen. Wenn ein Klassifizierer eine Anfrage kennzeichnet und die gekennzeichnete Kategorie ein Fallback-Modell hat, führt Claude Code die Anfrage auf diesem Modell erneut aus und zeigt eine Benachrichtigung im Transkript an. Für diese beiden Kategorien hängt das Fallback-Modell davon ab, welches Modell abgelehnt hat:

* **Fable 5.1, Fable 5 und Opus 5.5**: Biologie-gekennzeichnete Anfragen werden auf Opus 5 erneut ausgeführt, und Cybersicherheits-gekennzeichnete Anfragen werden auf Opus 4.8 erneut ausgeführt.
* **Opus 5**: Cybersicherheits-gekennzeichnete Anfragen werden auf Opus 4.8 erneut ausgeführt. Biologie-gekennzeichnete Anfragen enden stattdessen mit einer Ablehnung, da Opus 5 seine eigenen Biologie-Klassifizierer mit keinem Fallback-Modell ausführt.

Auf Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry löst Claude Code diese Ziele stattdessen durch Ihre Bereitstellung auf, und wenn Sie `ANTHROPIC_DEFAULT_OPUS_MODEL` setzen, werden Kategorien, die ein Fallback haben, auf dem angehefteten Modell erneut ausgeführt; siehe [Fallback auf Bedrock, Agent Platform und Foundry aktivieren](#enable-fallback-on-bedrock-agent-platform-and-foundry).

Nach einem Fallback wird die Sitzung auf dem Fallback-Modell fortgesetzt. Um zu Ihrem ursprünglichen Modell zurückzukehren, führen Sie [`/model`](#setting-your-model) aus.

Kategoriebasiertes Fallback erfordert Claude Code v2.1.219 oder später. Vor v2.1.219 wurde jede gekennzeichnete Fable 5-Anfrage auf dem Standard-Opus-Modell Ihres Providers erneut ausgeführt, und Opus 5 war keine Fallback-Quelle.

Das Fallback-Modell wird gegen [`availableModels`](#restrict-model-selection) überprüft. Wenn es blockiert ist, findet kein Fallback statt. Die Ablehnung wird als normaler Fehler angezeigt und das Modell der Sitzung bleibt unverändert.

<h4 id="check-what-triggered-fallback">
  Überprüfen Sie, was das Fallback ausgelöst hat
</h4>

Das Fallback kann bei der ersten Anfrage einer Sitzung ausgelöst werden, bevor Sie etwas Ungewöhnliches senden, da die erste Anfrage Arbeitsbereich-Kontext wie Ihren CLAUDE.md-Inhalt und Git-Status trägt. Ein Repository, das Sicherheits- oder Biologie-Material enthält, kann den Klassifizierer allein auf diesem Kontext auslösen.

Um zu überprüfen, ob Anpassungen der Auslöser sind, starten Sie eine Sitzung mit `claude --safe-mode`, das Anpassungen wie CLAUDE.md, Skills, MCP-Server und Hooks deaktiviert. Git-Status und Verzeichnisnamen sind keine Anpassungen und sind immer noch enthalten.

<h4 id="ask-before-switching">
  Vor dem Wechsel fragen
</h4>

Um zu entscheiden, was jedes Mal passiert, wenn eine Anfrage gekennzeichnet wird, anstatt automatisch zu wechseln, führen Sie `/config` aus und schalten Sie **Modelle wechseln, wenn eine Nachricht gekennzeichnet wird** aus, oder setzen Sie [`switchModelsOnFlag`](/docs/de/settings-reference#switchmodelsonflag) auf `false` in Ihrer Einstellungsdatei. Eine gekennzeichnete Anfrage pausiert dann die Sitzung mit zwei Optionen: zum Fallback-Modell wechseln oder die Eingabeaufforderung bearbeiten und auf dem aktuellen Modell erneut versuchen.

Einige Fälle verhalten sich anders:

* Wenn die gekennzeichnete Kategorie kein Fallback-Modell hat, wie eine Biologie-Kennzeichnung auf Opus 5, zeigt Claude Code die Eingabeaufforderung nicht an und die Anfrage endet mit der Ablehnung.
* Wenn beide Modelle die gleiche Anfrage kennzeichnen, können Sie die Eingabeaufforderung bearbeiten und erneut versuchen oder eine neue Sitzung starten.
* Auf mobilen [Claude Code im Web](/docs/de/claude-code-on-the-web) Sitzungen wird das Bearbeiten und erneute Versuchen nicht unterstützt. Wechseln Sie Modelle oder setzen Sie die Sitzung in einem Desktop-Browser oder der Desktop-App fort.
* Im [nicht-interaktiven Modus](/docs/de/cli-reference#cli-flags) und SDK-Integrationen, die die Eingabeaufforderung nicht anzeigen können, endet eine gekennzeichnete Anfrage den Zug mit einer Ablehnung.
* Wenn das Fallback-Ziel von [`availableModels`](#restrict-model-selection) blockiert wird, zeigt Claude Code die Eingabeaufforderung nicht an. Die gekennzeichnete Anfrage endet mit der Ablehnung, genauso wie automatisches Fallback, wenn das Ziel blockiert ist.

<h4 id="enable-fallback-on-bedrock-agent-platform-and-foundry">
  Fallback auf Bedrock, Agent Platform und Foundry aktivieren
</h4>

Auf [Amazon Bedrock](/docs/de/amazon-bedrock), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai) und [Microsoft Foundry](/docs/de/microsoft-foundry) sind Modell-IDs Provider-spezifisch, daher funktioniert automatisches Fallback nur, wenn Claude Code beide beteiligten Modelle identifizieren kann:

* Claude Code muss das aktuelle Modell als Fallback-Quelle erkennen. Fable 5.1 und Fable 5 werden erkannt, wenn die Modell-ID `claude-fable-5` enthält, dem Wert von `ANTHROPIC_DEFAULT_FABLE_MODEL` entspricht oder mit [`modelOverrides`](#override-model-ids-per-version) zugeordnet ist. Opus 5.5 und Opus 5 werden durch ihre Provider-Modell-ID oder eine [`modelOverrides`](#override-model-ids-per-version) Zuordnung erkannt.
* Das Fallback-Modell muss in Ihrer Bereitstellung aufgelöst werden. Wenn Sie `ANTHROPIC_DEFAULT_OPUS_MODEL` setzen, werden gekennzeichnete Anfragen für jede Kategorie, die ein Fallback hat, auf diesem Modell erneut ausgeführt; eine Biologie-Kennzeichnung auf Opus 5 endet immer noch mit einer Ablehnung. Wenn Sie es nicht setzen, werden Cybersicherheits-gekennzeichnete Anfragen auf einem Opus 4.8-Eintrag in der Modelliste des Providers erneut ausgeführt, und Biologie-gekennzeichnete Anfragen von einem Fable-Modell oder Opus 5.5 auf einem Opus 5-Eintrag.

Wenn eines der Modelle nicht identifiziert werden kann, wechselt Claude Code nicht automatisch. Die gekennzeichnete Anfrage endet mit einer Ablehnungsmeldung, und Sie können Modelle mit [`/model`](#setting-your-model) wechseln und erneut versuchen. Das Setzen von `ANTHROPIC_DEFAULT_FABLE_MODEL` auf Ihre Fable-Modell-ID ermöglicht die Fable-Erkennung. Das Setzen von `ANTHROPIC_DEFAULT_OPUS_MODEL` auf eine Opus-Modell-ID gibt den gekennzeichneten Kategorien ein Fallback-Ziel, es sei denn, die Angeheftung benennt ein Modell außerhalb der Opus-Familie oder das Modell, das abgelehnt hat; dann wechselt Claude Code nicht und die Ablehnung bleibt bestehen.

<h4 id="security-research-and-biology-workloads">
  Sicherheitsforschung und Biologie-Workloads
</h4>

Workloads in offensiver Sicherheit oder Biologie, einschließlich Penetrationstests, Capture the Flag (CTF) Übungen und Biologie-nahe Codebases, lösen häufig Fallback aus, oft bei der ersten Anfrage. Für substantive Biologie-Arbeit auf Fable 5.1, Fable 5 oder Opus 5.5 verschiebt Claude Code die Sitzung bei der ersten gekennzeichneten Anfrage zu Opus 5, und später Biologie-gekennzeichnete Anfragen enden dort in Ablehnungen, da Opus 5 kein Biologie-Fallback hat. Auf Opus 5 erhalten Sie diese Ablehnungen von der ersten gekennzeichneten Anfrage an.

Dies ist erwartetes Routing für diese Domänen, keine Kontoflagge. Wenn Ihre Organisation Fable-Klasse-Fähigkeit für diese Arbeit benötigt, wenden Sie sich an Ihr Anthropic-Kontoteam bezüglich vertrauenswürdiger Zugriffsprogramme.

<h3 id="adjust-effort-level">
  Anstrengungsstufe anpassen
</h3>

[Anstrengungsstufen](https://platform.claude.com/docs/en/build-with-claude/effort) steuern adaptive Überlegungen, die es dem Modell ermöglichen, bei jedem Schritt basierend auf der Aufgabenkomplexität zu entscheiden, ob und wie viel es denken soll. Niedrigere Anstrengung ist schneller und billiger für einfache Aufgaben, während höhere Anstrengung tiefere Überlegungen für komplexe Probleme bietet.

Die verfügbaren Anstrengungsstufen hängen vom Modell ab. Modelle, die hier nicht aufgelistet sind, unterstützen keine Anstrengung:

| Modell                                            | Stufen                                  |
| :------------------------------------------------ | :-------------------------------------- |
| Fable 5.1 und Fable 5                             | `low`, `medium`, `high`, `xhigh`, `max` |
| Opus 5.5, Opus 5, Sonnet 5, Opus 4.8 und Opus 4.7 | `low`, `medium`, `high`, `xhigh`, `max` |
| Opus 4.6 und Sonnet 4.6                           | `low`, `medium`, `high`, `max`          |

Wenn Sie eine Stufe setzen, die das aktive Modell nicht unterstützt, fällt Claude Code auf die höchste unterstützte Stufe bei oder unter der von Ihnen gesetzten zurück. Zum Beispiel wird `xhigh` auf Opus 4.6 als `high` ausgeführt. Ihre Organisation oder Ihre eigenen Einstellungen können auch begrenzen, welche Stufen ein Modell anbietet; siehe [Organisationsanstrengungsgrenzen](#organization-effort-limits).

Mit der [`ultracode`](/docs/de/settings-reference#ultracode) Einstellung aus löst Claude Code die Anstrengungsstufe der Sitzung in dieser Reihenfolge auf und nimmt die erste, die zutrifft:

1. Eine explizite Wahl: die [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/de/env-vars#variables) Umgebungsvariable, Start mit `--effort` oder `/effort` in der Sitzung ([ein nicht-interaktives `/effort` hat engere Auswirkungen](#non-interactive-effort))
2. Ihre Einstellungen: die Stufe, die Sie für das Modell gespeichert haben, oder ein [`effortLevel`](/docs/de/settings-reference#effortlevel) Schlüssel, mit der Vorrangigkeit zwischen ihnen und über Einstellungsdateien hinweg, die unter [`modelSettings`](/docs/de/settings-reference#modelsettings) angegeben ist
3. Die Standard-Anstrengung des Modells: `high` auf jedem Modell, das Anstrengung unterstützt, außer dass Opus 5.5 auf `medium` standardmäßig ist, Opus 4.7 auf `xhigh` standardmäßig ist und, wenn Ihre Organisation eine Standard-Anstrengungsstufe für sein [Organisationsstandardmodell](#organization-default-model) setzt, diese Stufe der Standard ist, wenn Sie dieses Modell ausführen

Opus 5.5 startet bei `medium`, es sei denn, eine der oben genannten Quellen setzt eine Stufe dafür, und ein Top-Level `effortLevel` in Ihrer Benutzereinstellungsdatei zählt nicht für Opus 5.5. Dieser Schlüssel ist die ältere Form, die `/effort` vor Claude Code schrieb, das Stufen pro Modell speicherte: er wird weiterhin angewendet, wo er zuvor angewendet wurde, auf Opus 5, Fable 5.1 und frühere Modelle, während Opus 5.5 und Modelle, die nach ihm veröffentlicht wurden, bei ihrem eigenen Standard starten, bis Sie eine Stufe dafür mit `/effort` oder dem `/model` Picker wählen. Ein Top-Level `effortLevel` in Projekt-, Lokal- oder verwalteten Einstellungen oder einer, die mit `--settings` übergeben wird, gilt für jedes Modell.

Wenn Sie `low`, `medium`, `high` oder `xhigh` in einer interaktiven Sitzung auf Ihrem Computer setzen, wählen Sie, wie lange es dauert, indem Sie bestätigen:

* `Enter` im `/effort` Schieber oder dem `/model` Picker oder eine Stufe, die nach `/effort` eingegeben wird: speichern Sie die Stufe als Ihren Standard und wenden Sie sie in späteren Sitzungen an
* `s` im `/effort` Schieber oder dem `/model` Picker: wenden Sie die Stufe nur auf diese Sitzung an. Erfordert Claude Code v2.1.257 oder später

Claude Code speichert die Stufe pro Modell unter dem [`modelSettings`](/docs/de/settings-reference#modelsettings) Schlüssel in Ihren Benutzereinstellungen, sodass jedes Modell seine eigene gespeicherte Stufe behält.

`max` ist die tiefste Überlegungsstufe. Sofern Sie sie nicht über die `CLAUDE_CODE_EFFORT_LEVEL` Umgebungsvariable setzen, wendet Claude Code `max` nur auf die aktuelle Sitzung an.

<Note>
  Eine Stufe, die Sie aus der Anstrengungskontrolle auf einem Telefon oder Browser auswählen, das über [Remote Control](/docs/de/remote-control#what-connected-devices-see) verbunden ist, gilt nur für diese Sitzung.
</Note>

<span id="non-interactive-effort" />

Wenn Sie eine Stufe mit `/effort` in einer [`-p` Ausführung](/docs/de/headless) setzen, wendet Claude Code sie nur auf diese Sitzung an und speichert sie nicht als Ihren Standard.

Das `/effort` Menü bietet auch `ultracode`. Ultracode ist eine Claude Code-Einstellung statt einer Modell-Anstrengungsstufe: es sendet `xhigh` an das Modell und hat zusätzlich Claude, um [dynamische Workflows](/docs/de/workflows) für substantive Aufgaben zu orchestrieren. Für wo es persistent gesetzt werden kann, siehe die [`ultracode`](/docs/de/settings-reference#ultracode) Einstellung.

Sie können Ultracode durch eine der folgenden Methoden aktivieren:

* **`/effort`**: führen Sie `/effort ultracode` aus oder wählen Sie es aus dem Menü
* **`--effort` Flag**: starten Sie mit `claude --effort ultracode`, das die Sitzung mit `xhigh` Anstrengung und Ultracode an startet
* **`ultracode` Einstellung**: setzen Sie [`"ultracode": true`](/docs/de/settings-reference#ultracode) in einer Einstellungsdatei, mit `--settings` oder in einer Agent SDK-Steueranfrage. Eine [`applyFlagSettings()`](/docs/de/agent-sdk/typescript#applyflagsettings) Anfrage akzeptiert auch `effortLevel: "ultracode"`
* **`/model` Picker**: bewegen Sie den Anstrengungsschieber mit den Pfeiltasten zu `ultracode`, während Sie ein Modell auswählen. Claude Code schaltet es für die aktuelle Sitzung ein, auch wenn Sie dieses Modell als Ihren Standard speichern

Das Übergeben von `ultracode` an das `--effort` Flag oder den Agent SDK `effortLevel` Wert erfordert Claude Code v2.1.203 oder später. Vor v2.1.203 druckte `--effort ultracode` `Unknown --effort value 'ultracode'` und die Sitzung startete mit der Standard-Anstrengung.

Die persistierte `effortLevel` Einstellung und die `CLAUDE_CODE_EFFORT_LEVEL` Umgebungsvariable akzeptieren nicht `ultracode`. Wenn `CLAUDE_CODE_EFFORT_LEVEL` auf eine andere Stufe als `xhigh` gesetzt ist, werden Anfragen auf dieser Stufe ausgeführt und die Workflow-Orchestrierung von Ultracode bleibt inaktiv. Das Auswählen von Ultracode zeigt dann eine Warnung an, dass die Umgebungsvariable die Anstrengung für die Sitzung überschreibt.

<span id="when-ultracode-is-available" />

Ultracode ist nicht verfügbar, wenn:

* [Workflows sind ausgeschaltet](/docs/de/workflows#turn-workflows-off)
* Das Modell unterstützt `xhigh` Anstrengung nicht
* Eine [Anstrengungsgrenze](#organization-effort-limits) unter `xhigh` gilt für das Modell

In diesen Fällen startet `--effort ultracode` die Sitzung mit Ultracode aus, auf der höchsten Anstrengungsstufe, die das Modell und jede Grenze zulassen, bis zu `xhigh`.

<h4 id="choose-an-effort-level">
  Wählen Sie eine Anstrengungsstufe
</h4>

Jede Stufe handelt Token-Ausgaben gegen Fähigkeit. Der Standard passt zu den meisten Codierungsaufgaben; passen Sie an, wenn Sie ein anderes Gleichgewicht möchten.

| Stufe       | Wann man sie verwendet                                                                                                                                           |
| :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `low`       | Reservieren Sie für kurze, begrenzte, latenzempfindliche Aufgaben, die nicht intelligenzempfindlich sind                                                         |
| `medium`    | Reduziert die Token-Nutzung für kostensensitive Arbeit, die etwas Intelligenz opfern kann. Der Standard auf Opus 5.5                                             |
| `high`      | Balanciert Token-Nutzung und Intelligenz. Der Standard auf jedem Modell außer Opus 5.5 und Opus 4.7                                                              |
| `xhigh`     | Tiefere Überlegungen bei höherer Token-Ausgabe. Der Standard auf Opus 4.7                                                                                        |
| `max`       | Kann die Leistung bei anspruchsvollen Aufgaben verbessern, kann aber abnehmende Erträge zeigen und ist anfällig für Überdenken. Testen Sie vor breiter Übernahme |
| `ultracode` | Eine Claude Code-Einstellung, die für jede substantive Aufgabe einen [dynamischen Workflow](/docs/de/workflows) mit `xhigh` pro-Nachricht-Überlegungen plant          |

Die Anstrengungsskala ist pro Modell kalibriert, daher stellt der gleiche Stufenname nicht den gleichen zugrunde liegenden Wert über Modelle hinweg dar.

<h4 id="use-ultrathink-for-one-off-deep-reasoning">
  Verwenden Sie Ultrathink für einmalige tiefe Überlegungen
</h4>

Fügen Sie `ultrathink` überall in Ihrer Eingabeaufforderung ein, um tiefere Überlegungen bei diesem Zug anzufordern, ohne Ihre Sitzungs-Anstrengungseinstellung zu ändern. Claude Code erkennt das Schlüsselwort und fügt eine In-Context-Anweisung hinzu. Die an die API gesendete Anstrengungsstufe bleibt unverändert. Claude Code leitet andere Phrasen wie „think", „think hard" und „think more" als gewöhnlichen Eingabeaufforderungstext durch und erkennt sie nicht als Schlüsselwörter.

<h4 id="set-the-effort-level">
  Legen Sie die Anstrengungsstufe fest
</h4>

Sie können die Anstrengung durch eine der folgenden Methoden ändern:

* **`/effort`**: führen Sie `/effort` ohne Argumente aus, um einen interaktiven Schieber zu öffnen, `/effort` gefolgt von einem Stufennamen, um ihn direkt zu setzen, oder `/effort auto`, um Ihre gespeicherte Stufe für das aktive Modell zu löschen. Sie können es ausführen, während Claude arbeitet, und sobald Sie die [Cache-Warnung](/docs/de/prompt-caching#changing-effort-level) bestätigen, wenn Claude Code eine anzeigt, wendet Claude Code die neue Stufe auf die nächste Anfrage im Zug an
* **In `/model`**: verwenden Sie die Pfeiltasten nach links/rechts, um den Anstrengungsschieber anzupassen, wenn Sie ein Modell auswählen
* **`--effort` Flag**: übergeben Sie einen Stufennamen, um ihn für eine einzelne Sitzung beim Start von Claude Code zu setzen
* **Umgebungsvariable**: setzen Sie `CLAUDE_CODE_EFFORT_LEVEL` auf einen Stufennamen oder `auto`
* **Einstellungen**: setzen Sie eine pro-Modell-Stufe in [`modelSettings`](/docs/de/settings-reference#modelsettings), oder setzen Sie [`effortLevel`](/docs/de/settings-reference#effortlevel) auf `low`, `medium`, `high` oder `xhigh` als Standard für Modelle ohne eine. `max` wird in keinem Schlüssel akzeptiert, und `ultracode` hat seinen eigenen [`ultracode`](/docs/de/settings-reference#ultracode) Schlüssel
* **Von einem verbundenen Gerät**: in einer [Remote Control](/docs/de/remote-control#what-connected-devices-see) Sitzung wählen Sie eine Stufe aus der Anstrengungskontrolle auf Ihrem Telefon oder in Ihrem Browser. Die Stufe gilt nur für die aktuelle Sitzung. Erfordert Claude Code v2.1.234 oder später
* **Skill- und Subagenten-Frontmatter**: setzen Sie `effort` in einer [Skill](/docs/de/skills#frontmatter-reference) oder [Subagenten](/docs/de/sub-agents#supported-frontmatter-fields) Markdown-Datei, um die Anstrengungsstufe zu überschreiben, wenn dieser Skill oder Subagent ausgeführt wird

Frontmatter-Anstrengung gilt, wenn dieser Skill oder Subagent aktiv ist, überschreibt die Sitzungsstufe, aber nicht die Umgebungsvariable. Eine [`maxEffortLevel`](/docs/de/settings-reference#maxeffortlevel) oder [Organisationsanstrengungsgrenze](#organization-effort-limits) begrenzt immer noch die Stufe, auf der der Skill oder Subagent ausgeführt wird.

Wenn Sie `effortLevel` in [verwalteten Einstellungen](/docs/de/managed-settings) setzen, wendet Claude Code es beim Einstellungsschritt der [Anstrengungsauflösungsreihenfolge](#adjust-effort-level) an, und Benutzer können die Stufe immer noch mit `/effort` oder `--effort` ändern. Um Benutzer bei oder unter einer Stufe zu halten, setzen Sie [`maxEffortLevel`](/docs/de/settings-reference#maxeffortlevel).

Der Anstrengungsschieber erscheint in `/model`, wenn ein unterstütztes Modell ausgewählt ist. Die aktuelle Anstrengungsstufe wird auch in der Sitzungskopfzeile neben dem Modellnamen angezeigt, zum Beispiel „with low effort", sodass Sie bestätigen können, welche Einstellung aktiv ist, ohne `/model` zu öffnen. Die Fußzeile zeigt auch kurz die Anstrengungsstufe beim Start und wenn sie sich ändert.

<h4 id="adaptive-reasoning-and-fixed-thinking-budgets">
  Adaptive Überlegungen und feste Denk-Budgets
</h4>

Adaptive Überlegungen machen Denken bei jedem Schritt optional, sodass Claude schneller auf Routine-Eingabeaufforderungen reagieren und tiefere Überlegungen für Schritte reservieren kann, die davon profitieren. Wenn Sie möchten, dass Claude häufiger oder seltener denkt, als die aktuelle Stufe produziert, können Sie dies direkt in Ihrer Eingabeaufforderung oder in `CLAUDE.md` sagen; das Modell reagiert auf diese Anleitung innerhalb seiner Anstrengungseinstellung.

Fable-Modelle, Sonnet 5 und Opus 4.7 und später verwenden immer adaptive Überlegungen. Der Modus mit festem Denk-Budget und `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING` gelten nicht für sie.

Auf Opus 4.6 und Sonnet 4.6 können Sie `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` setzen, um zum vorherigen Modus mit festem Denk-Budget zurückzukehren, der von `MAX_THINKING_TOKENS` gesteuert wird. Siehe [Umgebungsvariablen](/docs/de/env-vars).

<h3 id="extended-thinking">
  Erweitertes Denken
</h3>

Erweitertes Denken ist die Überlegung, die Claude vor der Antwort ausgibt. Bei Modellen, die [adaptive Überlegungen](#adjust-effort-level) unterstützen, ist die Anstrengungsstufe die primäre Kontrolle dafür, wie viel Denken stattfindet; die folgenden Einstellungen schalten Denken ein oder aus und steuern, wie es angezeigt wird. Mit ausgeschaltetem Denken auf der Anthropic API sendet Claude Code Anstrengung `high` statt einer höheren Stufe an Modelle, von denen es weiß, dass sie [diese Kombination nicht akzeptieren](/docs/de/errors#effort-isnt-available-with-thinking-turned-off), wie Opus 5.

| Kontrolle                                    | Wie man sie setzt                                                                                                                                                                                                                                                                                                                                                                                                          |
| :------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Umschalter für die aktuelle Sitzung          | Drücken Sie `Option+T` auf macOS oder `Alt+T` auf Windows und Linux                                                                                                                                                                                                                                                                                                                                                        |
| Setzen Sie den globalen Standard             | Führen Sie `/config` aus und schalten Sie den Denkmodus um. Gespeichert als `alwaysThinkingEnabled` in `~/.claude/settings.json`                                                                                                                                                                                                                                                                                           |
| Deaktivieren Sie über eine Umgebungsvariable | Setzen Sie [`MAX_THINKING_TOKENS=0`](/docs/de/env-vars), das Denken auf der Anthropic API außer auf Opus 5.5 und Fable-Modellen ausschaltet. Auf [Drittanbieter-Providern](/docs/de/third-party-integrations) lässt dies den `thinking` Parameter stattdessen weg, und adaptive-Überlegungs-Modelle können immer noch denken. Andere Werte gelten nur mit einem [festem Denk-Budget](#adaptive-reasoning-and-fixed-thinking-budgets) |

Denken kann auf Opus 5.5 oder den Fable-Modellen nicht ausgeschaltet werden. Der Sitzungs-Umschalter, `alwaysThinkingEnabled` und `MAX_THINKING_TOKENS=0` haben dort keine Auswirkung, und das Modell entscheidet pro Schritt, wie viel es denken soll, basierend auf der Anstrengungsstufe.

Claude Code bricht Denk-Ausgabe standardmäßig zusammen. Drücken Sie `Ctrl+O`, um den ausführlichen Modus umzuschalten und die Überlegung als grauer kursiver Text zu sehen. Interaktive Sitzungen auf der Anthropic API erhalten standardmäßig redigierte Denk-Blöcke, daher setzen Sie `showThinkingSummaries: true` in [Einstellungen](/docs/de/settings), wenn Sie die vollständigen Zusammenfassungen verfügbar haben möchten, wenn Sie erweitern. Ihnen werden alle generierten Denk-Token berechnet, auch wenn sie zusammengeklappt oder redigiert sind.

<h3 id="extended-context">
  Erweiterter Kontext
</h3>

Fable 5.1, Fable 5, Sonnet 5, Opus 4.6 und später sowie Sonnet 4.6 unterstützen ein [1-Million-Token-Kontextfenster](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) für lange Sitzungen mit großen Codebases.

Auf der Anthropic API werden Fable 5.1, Fable 5, Sonnet 5 und Opus 4.7 und später standardmäßig mit dem 1M-Fenster ausgeführt. Sie wählen keine `[1m]` Variante aus und schalten keine Nutzungsguthaben für das 1M-Fenster auf diesen Modellen ein. Fable-Nutzung selbst kann auf einigen Plänen zu Nutzungsguthaben abgerechnet werden; siehe [Fable und Nutzungsguthaben](#fable-and-usage-credits).

Opus 4.6 und Sonnet 4.6 erreichen 1M nur durch ihre `[1m]` Variante, und der Zugriff auf diese Variante hängt von Ihrem Plan ab. Auf Max-, Team- und Enterprise-Plänen, einschließlich sowohl Team Standard- als auch Team Premium-Plätze, wird Opus 4.6 mit 1M-Kontext in Ihrem Abonnement enthalten. Sonnet 4.6 mit 1M-Kontext erfordert [Nutzungsguthaben](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) auf jedem Abonnement-Plan, einschließlich Max.

| Plan                     | Opus 4.6 mit 1M-Kontext                                                                                         | Sonnet 4.6 mit 1M-Kontext                                                                                       |
| ------------------------ | --------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| Max, Team und Enterprise | Im Abonnement enthalten                                                                                         | Erfordert [Nutzungsguthaben](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) |
| Pro                      | Erfordert [Nutzungsguthaben](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) | Erfordert [Nutzungsguthaben](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) |
| API und Pay-as-you-go    | Vollständiger Zugriff                                                                                           | Vollständiger Zugriff                                                                                           |

Claude Code überprüft diese Plan-Anforderungen nur, wenn es sich direkt mit der Anthropic API verbindet. Wenn Sie `ANTHROPIC_BASE_URL` auf ein [LLM-Gateway](/docs/de/llm-gateway#subscriptions-and-gateways) verweisen und Ihr gespeicherter claude.ai-Login bleibt die aktive Anmeldeinformation, überprüft Claude Code nicht Ihre Plan-Nutzungsguthaben. Die `[1m]` Optionen bleiben in `/model` verfügbar, und das Gateway entscheidet, ob die Anfrage erfolgreich ist. Vor v2.1.229 lehnte Claude Code `/model sonnet[1m]` in dieser Konfiguration ab, wenn es die Nutzungsguthaben auf dem Konto nicht bestätigen konnte.

Um 1M-Kontext auszuschalten, setzen Sie `CLAUDE_CODE_DISABLE_1M_CONTEXT=1`. Claude Code entfernt 1M-Modellvarianten aus dem Modell-Picker. Bei Modellen mit einem nativen 1M-Fenster, wie Sonnet 5 und die Fable-Modelle, behandelt es das Modell auch als ein 200K-Kontextfenster:

* Mit Auto-Komprimierung an komprimieren Sitzungen bei der 200K-Grenze durch [Auto-Komprimierung](#set-the-auto-compact-window). Das Setzen des Auto-Komprimierungs-Fensters über 200K hebt den Halt nicht auf, da Claude Code dieses Fenster auf das Kontextfenster des Modells begrenzt.
* Mit Auto-Komprimierung aus stoppen Sitzungen bei der 200K-Grenze mit dem [Kontext-Limit-Fehler](/docs/de/errors#prompt-is-too-long) statt zu komprimieren.

Vor v2.1.223 hielt Claude Code nur Sonnet 5, Opus 4.8 und Opus 5-Sitzungen bei 200K. Siehe [Umgebungsvariablen](/docs/de/env-vars).

Das 1M-Kontextfenster verwendet Standard-Modell-Preisgestaltung ohne Prämie für Token über 200K. Für Pläne, bei denen erweiterter Kontext in Ihrem Abonnement enthalten ist, bleibt die Nutzung von Ihrem Abonnement abgedeckt. Für Pläne, die erweiterten Kontext durch Nutzungsguthaben zugreifen, werden Token zu Nutzungsguthaben abgerechnet.

Wenn Ihr Konto 1M-Kontext unterstützt, erscheint die Option im `/model` Picker in den neuesten Versionen von Claude Code. Wenn Sie sie nicht sehen, versuchen Sie, Ihre Sitzung neu zu starten.

Sie können auch das `[1m]` Suffix mit Modellaliasen oder vollständigen Modellnamen verwenden:

```text theme={null}
# Verwenden Sie den opus[1m] oder sonnet[1m] Alias
/model opus[1m]
/model sonnet[1m]

# Oder hängen Sie [1m] an einen vollständigen Modellnamen an
/model claude-opus-4-8[1m]
```

<h4 id="sonnet-5-context-window">
  Sonnet 5 Kontextfenster
</h4>

Auf der Anthropic API wird Sonnet 5 immer mit dem 1M-Kontextfenster ausgeführt. Es gibt keine 200K-Variante, kein `[1m]` Suffix zum Auswählen und keine erforderlichen Nutzungsguthaben auf einem Plan. Sitzungen komprimieren automatisch, bevor das Fenster sich füllt, bei etwa 967K Token standardmäßig; setzen Sie [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/de/env-vars), um einen anderen Schwellenwert zu wählen.

Zwei Konfigurationen budgetieren das Fenster stattdessen bei 200K:

* **LLM-Gateway**: wenn `ANTHROPIC_BASE_URL` auf ein [Gateway](/docs/de/llm-gateway) verweist, kann Claude Code 1M-Unterstützung nicht überprüfen. Um das vollständige Fenster zu verwenden, wählen Sie Sonnet 5 (1M context) im Modell-Picker, das zu `sonnet[1m]` zugeordnet wird.
* **`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`**: hält Sitzungen auf jedem Modell mit einem nativen 1M-Fenster bei einem 200K-Fenster; siehe [Erweiterter Kontext](#extended-context) für wie der Halt durchgesetzt wird. Nützlich für Bereitstellungen, die den Kontext begrenzen müssen.

<h2 id="context-window-and-auto-compaction">
  Kontextfenster und automatische Komprimierung
</h2>

Das Fenster für automatische Komprimierung bestimmt, wie voll das Kontextfenster werden kann, bevor Claude Code die Konversation komprimiert. Informationen darüber, was die Komprimierung pro Mechanismus behält und verwirft, finden Sie unter [Was die Komprimierung übersteht](/docs/de/context-window#what-survives-compaction).

<h3 id="set-the-auto-compact-window">
  Fenster für automatische Komprimierung einstellen
</h3>

Sie können das Fenster für automatische Komprimierung an drei Stellen einstellen:

* **Für diese Sitzung und spätere**: Führen Sie `/autocompact` mit einem Wert aus, z. B. `/autocompact 500k`. Claude Code speichert es in Ihren Benutzereinstellungen als [`autoCompactWindow`](/docs/de/settings-reference#autocompactwindow) und wendet es auf die aktuelle Sitzung an. Wenn ein höher priorisierter [Einstellungsbereich](/docs/de/settings#settings-precedence) wie verwaltete Einstellungen den Schlüssel setzt, speichert der Befehl Ihren Wert, aber die Sitzung behält das Fenster dieses Bereichs bei, und der Befehl teilt dies mit. Führen Sie `/autocompact auto` aus, um zum für Ihr Modell optimierten Fenster zurückzukehren.
* **Für einen Start**: Übergeben Sie [`--autocompact`](/docs/de/cli-reference#cli-flags) beim Starten von Claude Code. Das Flag setzt Ihre gespeicherte Einstellung für diesen Start außer Kraft, ohne sie zu ändern, und `claude --autocompact auto` führt die Sitzung mit dem optimierten Fenster aus, auch wenn Ihre gespeicherte Einstellung einen Wert hat. Im Gegensatz zu `/autocompact` wird das Flag nicht durch einen höher priorisierten Einstellungsbereich wie verwaltete Einstellungen außer Kraft gesetzt.
* **In Skripten und Cloud-Umgebungen**: Setzen Sie [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/de/env-vars). Während es gesetzt ist, hat es Vorrang vor dem Befehl, dem Flag und der Einstellung, und `/autocompact` meldet die Außerkraftsetzung, anstatt das Fenster zu ändern.

Der Befehl und das Flag akzeptieren eine Fenstergröße von 100K bis 1M Token in einer dieser Formen:

* Eine einfache Token-Anzahl, z. B. `200000`
* Ein `k`- oder `M`-Suffix, z. B. `500k` oder `1M`
* Eine bloße Zahl von 100 bis 1000, was Tausende bedeutet, also setzt `200` 200.000

Die Umgebungsvariable akzeptiert nur die einfache Token-Anzahl. Claude Code begrenzt das Fenster auf das Kontextfenster des Modells.

<h3 id="default-auto-compact-thresholds">
  Standard-Schwellwerte für automatische Komprimierung
</h3>

Wenn Sie kein Fenster für automatische Komprimierung einstellen, komprimiert Claude Code, wenn die Konversation das Kontextlimit des Modells erreicht, außer in diesen Sitzungen:

* [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) komprimieren, wenn sich die Konversation dem Limit des Modells nähert
* Sonnet 4.6 und Opus 4.6 ohne [erweiterter Kontext](#extended-context) komprimieren bei der 200K-Grenze, ebenso wie Opus 4.8 und später, wenn sie mit einem 200K-Kontextfenster laufen, z. B. auf Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry
* Wenn Sie [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/de/env-vars) setzen, komprimieren Modelle mit einem nativen 1M-Fenster, wie Sonnet 5 und die Fable-Modelle, bei der 200K-Grenze
* Modelle, die mit einem nativen 1M-Fenster laufen, wie Sonnet 5, die Fable-Modelle und Opus 4.7 und später auf der Anthropic API, komprimieren, bevor das Fenster voll wird, standardmäßig bei etwa 967K Token. Auf Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry gibt [Modelle für Bereitstellungen von Drittanbietern anheften](#pin-models-for-third-party-deployments) an, welche Modelle mit diesem Fenster laufen; für die Konfigurationen, die Sonnet 5 stattdessen mit 200K budgetieren, siehe [Sonnet 5 Kontextfenster](#sonnet-5-context-window)
* Sitzungen auf einer Modell-ID, die Claude Code nicht erkennt, z. B. ein [LLM-Gateway](/docs/de/llm-gateway)-Alias, komprimieren bei dem Kontextfenster, das Claude Code für die ID annimmt; siehe [Fenster für ein Gateway oder eine benutzerdefinierte Modell-ID korrigieren](#correct-the-window-for-a-gateway-or-custom-model-id)

<h3 id="correct-the-window-for-a-gateway-or-custom-model-id">
  Fenster für ein Gateway oder eine benutzerdefinierte Modell-ID korrigieren
</h3>

Bei einem [LLM-Gateway](/docs/de/llm-gateway) oder einer anderen benutzerdefinierten Bereitstellung kann Claude Code ein Kontextfenster für die Modell-ID annehmen, das sich vom echten Fenster des Modells unterscheidet, unabhängig davon, ob es die ID zu einem Claude-Modell auflöst oder nicht. Setzen Sie [`CLAUDE_CODE_MAX_CONTEXT_TOKENS`](/docs/de/env-vars) auf das Fenster, das Claude Code stattdessen annehmen sollte.

Wie die Variable angewendet wird, hängt von der ID ab. Claude Code behandelt eine ID als Anbieter oder benutzerdefinierte Schreibweise, wenn sie nicht mit `claude-` beginnt, in beliebiger Schreibweise, oder wenn sie ein Suffix trägt, das Claude Code beim Lesen der ID entfernt, z. B. das `@YYYYMMDD`-Datum, das auf Google Cloud's Agent Platform verwendet wird. Vor v2.1.259 zählte Claude Code ein entferntes Suffix nicht, daher wurde eine nicht erkannte `claude-`-ID mit einem Datumssuffix als bloßer `claude-`-Name behandelt.

Eine nicht erkannte Anbieter- oder benutzerdefinierte Schreibweise, dieselbe Schreibweise mit `[1m]` und jede andere ID sind drei separate Fälle:

* Wenn Claude Code einen Anbieter oder eine benutzerdefinierte Schreibweise nicht zu einem Modell auflösen kann, das es erkennt, und die ID kein `[1m]` enthält, wird die Variable direkt angewendet und die proaktive Komprimierung wird bei dem deklarierten Fenster fortgesetzt.
* Wenn Claude Code einen Anbieter oder eine benutzerdefinierte Schreibweise nicht zu einem Modell auflösen kann, das es erkennt, und die ID enthält `[1m]`, in beliebiger Schreibweise, nimmt Claude Code ein 1M-Fenster dafür an und die Variable wird nicht von selbst angewendet. Um das Fenster zu korrigieren und gleichzeitig die proaktive Komprimierung beizubehalten, setzen Sie auch [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/de/env-vars). Mit dieser Variable gesetzt, dimensioniert Claude Code die ID wie dieselbe Schreibweise ohne `[1m]`, daher wird `CLAUDE_CODE_MAX_CONTEXT_TOKENS` angewendet, wenn es auf diese ungetaggte Schreibweise angewendet würde.

  Mit einem deklarierten Fenster über 200K zeigt Claude Code dann eine [Startwarnung](/docs/de/errors#the-200k-limit-isnt-enforced) an, dass das 200K-Limit nicht erzwungen wird. Die Warnung ist in dieser Konfiguration zu erwarten.
* Wenn die ID zu einem Modell aufgelöst wird, das Claude Code erkennt, oder die ID ein bloßer `claude-`-Name ohne Suffix ist, das Claude Code entfernen kann, in beliebiger Schreibweise, wird die Variable nur wirksam, wenn auch [`DISABLE_COMPACT`](/docs/de/env-vars) gesetzt ist, was alle Komprimierung deaktiviert.

  Beispielsweise löst eine ID, die einen Claude-Modellnamen enthält, den Claude Code kennt, wie `anthropic/claude-opus-4-8`, `us.anthropic.claude-…-v1:0`, oder das datierte `claude-sonnet-4-5@20250929`, zu diesem Modell auf. Dies umfasst IDs, die auch `[1m]` enthalten: Claude Code löst `claude-opus-4-8[1m]` zu Opus 4.8 auf, auch wenn `CLAUDE_CODE_DISABLE_1M_CONTEXT` gesetzt ist.

Für eine Modell-ID, die Claude Code nicht erkennt, setzen Sie [`CLAUDE_CODE_DISABLE_UNKNOWN_MODEL_WINDOW_ENFORCEMENT=1`](/docs/de/env-vars), damit Claude Code nur komprimiert, nachdem die API die Konversation mit einem [zu-langen Fehler, den Claude Code erkennt](/docs/de/errors#prompt-is-too-long), ablehnt. Claude Code führt diese Wiederherstellung nicht aus, wenn ein Gateway [den Fehler](/docs/de/llm-gateway-connect#troubleshoot-gateway-errors) zu einer Formulierung umschreibt, die Claude Code nicht erkennt.

<h2 id="checking-your-current-model">
  Überprüfung Ihres aktuellen Modells
</h2>

Sie können sehen, welches Modell Sie derzeit verwenden, an zwei Stellen:

* In der [Statuszeile](/docs/de/statusline), falls Sie eine konfiguriert haben
* In `/status`, das auch Ihre Kontoinformationen anzeigt

<h2 id="add-a-custom-model-option">
  Benutzerdefinierte Modelloption hinzufügen
</h2>

Verwenden Sie `ANTHROPIC_CUSTOM_MODEL_OPTION`, um einen einzelnen benutzerdefinierten Eintrag zur `/model`-Auswahl hinzuzufügen, ohne die integrierten Aliase zu ersetzen. Dies ist nützlich zum Testen von Modell-IDs, die Claude Code standardmäßig nicht auflistet. Für LLM-Gateway-Bereitstellungen kann Claude Code die Auswahl vom `/v1/models`-Endpunkt des Gateways auffüllen, wenn `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1` gesetzt ist. Daher ist diese Variable nur erforderlich, wenn die Erkennung deaktiviert ist oder das gewünschte Modell nicht zurückgibt. Siehe [Gateway-Modellauswahl](/docs/de/llm-gateway-protocol#model-discovery).

Um stattdessen mehrere Modelle aufzulisten, in Ihrer eigenen Reihenfolge und unter Bezeichnungen Ihrer Wahl, setzen Sie [`modelPicker`](/docs/de/settings-reference#modelpicker). Der Eintrag gibt an, welche Zeilen die Auswahl behält, wenn diese Aufstellung die integrierte ersetzt.

Dieses Beispiel setzt alle drei Variablen, um eine Gateway-gesteuerte Opus-Bereitstellung auswählbar zu machen. Claude Code liest Umgebungsvariablen beim Start, daher führen Sie die Exporte vor dem Starten von `claude` aus oder starten Sie eine vorhandene Sitzung neu, um sie zu übernehmen:

```bash theme={null}
export ANTHROPIC_CUSTOM_MODEL_OPTION="my-gateway/claude-opus-5-5"
export ANTHROPIC_CUSTOM_MODEL_OPTION_NAME="Opus via Gateway"
export ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION="Custom deployment routed through the internal LLM gateway"
```

`ANTHROPIC_CUSTOM_MODEL_OPTION_NAME` und `ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION` sind optional:

* Wenn Sie den Namen weglassen, zeigt der Eintrag den Namen des Modells an, wenn Claude Code die ID [erkennt](#customize-pinned-model-display-and-capabilities), und die Modell-ID andernfalls.
* Wenn Sie die Beschreibung weglassen, verwendet Claude Code `Custom model (<model-id>)`.

Claude Code listet den benutzerdefinierten Eintrag nach den integrierten Einträgen auf, und alle [`modelPicker`](/docs/de/settings-reference#modelpicker)-Zeilen, die Sie anhängen, kommen danach.

Claude Code überspringt die Validierung für die Modell-ID, die in `ANTHROPIC_CUSTOM_MODEL_OPTION` gesetzt ist, daher können Sie jeden String verwenden, den Ihr API-Endpunkt akzeptiert.

Wenn [`availableModels`](#restrict-model-selection) gesetzt ist, fügen Sie die benutzerdefinierte Modell-ID auch in die Zulassungsliste ein. Andernfalls filtert Claude Code den benutzerdefinierten Eintrag aus der Auswahl und lehnt eine `--model`-Auswahl davon wie jedes andere ausgeschlossene Modell ab.

Eine benutzerdefinierte ID, die einen Familiennamen einbettet, wie z. B. `my-gateway/claude-opus-5-5`, zählt als spezifischer Eintrag für diese Familie und deaktiviert deren Wildcard. Daher müssen Sie auch die Versionen auflisten, die Sie auswählbar halten möchten. Siehe [Zusammenführungsverhalten](#merge-behavior).

<h2 id="environment-variables">
  Umgebungsvariablen
</h2>

Verwenden Sie die folgenden Umgebungsvariablen, um die Modellnamen zu steuern, auf die die Aliase verweisen. Jeder Wert muss ein vollständiger Modellname oder das entsprechende Äquivalent für Ihren API-Anbieter sein. Um das Modell auszuwählen, mit dem Ihre Sitzungen starten, setzen Sie [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions), das diese Tabelle auslässt.

| Umgebungsvariable                | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| -------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_DEFAULT_FABLE_MODEL`  | Das Modell, das für `fable` verwendet werden soll, und die Modell-ID, die Claude Code als Fable-Modell für [automatisches Modell-Fallback](#automatic-model-fallback) bei Drittanbieter-Anbietern erkennt                                                                                                                                                                                                                                                                                                                         |
| `ANTHROPIC_DEFAULT_OPUS_MODEL`   | Das Modell, das für `opus` verwendet werden soll, oder für `opusplan`, wenn Plan Mode aktiv ist.                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `ANTHROPIC_DEFAULT_SONNET_MODEL` | Das Modell, das für `sonnet` verwendet werden soll, oder für `opusplan`, wenn Plan Mode nicht aktiv ist.                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `ANTHROPIC_DEFAULT_HAIKU_MODEL`  | Das Modell, das für `haiku` verwendet werden soll, oder [Hintergrundfunktionalität](/docs/de/costs#background-token-usage)                                                                                                                                                                                                                                                                                                                                                                                                             |
| `CLAUDE_CODE_SUBAGENT_MODEL`     | Das Standard-Modell für [Subagents](/docs/de/sub-agents#choose-a-model), [Agent-Team](/docs/de/agent-teams#specify-teammates-and-models)-Kollegen und [Workflow](/docs/de/workflows)-Agenten, denen auf andere Weise kein Modell zugewiesen ist. Akzeptiert einen Alias wie `haiku` oder einen vollständigen Modellnamen. Ein Modell pro Aufruf oder das `model`-Feld einer Definition, einschließlich `inherit`, hat Vorrang. Um das zu ändern, setzen Sie [`CLAUDE_CODE_SUBAGENT_MODEL_FORCE`](/docs/de/sub-agents#run-every-subagent-on-one-model) |

Hinweis: `ANTHROPIC_SMALL_FAST_MODEL` ist veraltet zugunsten von `ANTHROPIC_DEFAULT_HAIKU_MODEL`.

<h3 id="pin-models-for-third-party-deployments">
  Modelle für Drittanbieter-Bereitstellungen fixieren
</h3>

Beim Bereitstellen von Claude Code über [Amazon Bedrock](/docs/de/amazon-bedrock), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai), [Microsoft Foundry](/docs/de/microsoft-foundry) oder [Claude Platform on AWS](/docs/de/claude-platform-on-aws) sollten Sie Modellversionen vor dem Rollout für Benutzer fixieren.

Ohne Fixierung verwendet Claude Code Modellaliase wie `fable`, `opus`, `sonnet` und `haiku`, die zur integrierten Standard-Modell-ID für jeden Anbieter aufgelöst werden. Dieser Standard kann hinter der neuesten Anthropic-Veröffentlichung zurückbleiben, und das Modell, auf das er verweist, ist möglicherweise noch nicht in einem Benutzerkonto aktiviert. Wenn der Standard nicht verfügbar ist, sehen Amazon Bedrock- und Google Cloud's Agent Platform-Benutzer einen Hinweis und die Sitzung greift auf eine frühere Version des Standard-Modells zurück, oder auf das Standard-Sonnet-Modell, wenn der Standard ein Opus-Modell ist und keine Opus-Version verfügbar ist. Microsoft Foundry-Benutzer sehen stattdessen Fehler, da Microsoft Foundry keine entsprechende Startprüfung hat.

Bei Amazon Bedrock und Google Cloud's Agent Platform fixiert ein Benutzer, der die Sitzung auf einer bestimmten Sonnet- oder Opus-Version startet, beispielsweise mit `--model`, `ANTHROPIC_MODEL` oder der `model`-Einstellung, diese Version als Standard der Sitzung für den entsprechenden Alias: Die Startprüfung überspringt den integrierten Standard, den sie ersetzt, und zeigt keinen Fallback-Hinweis. Vor v2.1.211 wurde die Prüfung ausgeführt und konnte einen Hinweis anzeigen, auch wenn ein Sitzungsmodell explizit konfiguriert war.

<Warning>
  Setzen Sie die Modell-Umgebungsvariablen auf spezifische Versions-IDs als Teil Ihres anfänglichen Setups. Das Fixieren ermöglicht es Ihnen, zu kontrollieren, wann Ihre Benutzer zu einem neuen Modell wechseln.
</Warning>

Verwenden Sie die folgenden Umgebungsvariablen mit versionsspezifischen Modell-IDs für Ihren Anbieter:

| Anbieter                      | Beispiel                                                             |
| :---------------------------- | :------------------------------------------------------------------- |
| Amazon Bedrock                | `export ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'` |
| Google Cloud's Agent Platform | `export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'`              |
| Microsoft Foundry             | `export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'`              |

Wenden Sie das gleiche Muster auf `ANTHROPIC_DEFAULT_FABLE_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL` und `ANTHROPIC_DEFAULT_HAIKU_MODEL` an. Für aktuelle und ältere Modell-IDs über alle Anbieter hinweg siehe [Modellübersicht](https://platform.claude.com/docs/en/about-claude/models/overview). Um Benutzer auf eine neue Modellversion zu aktualisieren, aktualisieren Sie diese Umgebungsvariablen und stellen Sie erneut bereit.

Um [erweiterten Kontext](#extended-context) für ein fixiertes Modell zu aktivieren, fügen Sie `[1m]` an die Modell-ID in `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL` oder `ANTHROPIC_DEFAULT_FABLE_MODEL` an:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8[1m]'
```

Mit dem `[1m]`-Suffix wird das 1M-Kontextfenster auf alle Verwendungen des fixierten Alias angewendet, einschließlich der Plan-Mode-Opus-Phase von [`opusplan`](#opusplan-model-setting) und [Subagents](/docs/de/sub-agents#choose-a-model), deren `model`-Frontmatter den Alias benennt.

* Claude Code entfernt das Suffix, bevor die Modell-ID an Ihren Anbieter gesendet wird.
* Fügen Sie `[1m]` nur an, wenn das zugrunde liegende Modell [1M-Kontext unterstützt](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model).
* Das Suffix wird pro Variable gelesen, nicht pro Modell. Bei Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry verwendet eine Modell-ID ohne `[1m]` in einer Variable 200K-Kontext, auch wenn eine andere Variable das gleiche Modell mit dem Suffix setzt. Sonnet 5 wird auf diesen Anbietern immer mit dem 1M-Fenster ausgeführt und benötigt niemals das Suffix.

<Note>
  Eine `availableModels`-Zulassungsliste, die über [MDM oder eine verwaltete Einstellungsdatei](/docs/de/managed-settings#delivery-mechanisms) bereitgestellt wird, gilt weiterhin bei Verwendung von Drittanbieter-Anbietern; [server-verwaltete Einstellungen werden dort nicht bereitgestellt](/docs/de/server-managed-settings#platform-availability).

  Die Filterung stimmt mit einem Modellalias wie `opus`, einem Versionspräfix wie `claude-opus-4-8` oder der vollständigen Modell-ID in Anbieterform überein. Anbieterspezifische Präfixe wie `us.anthropic.` werden nicht entfernt, daher müssen Sie zum Zulassen eines bestimmten Modells die vollständige Anbieterform-ID auflisten oder sie durch [`modelOverrides`](#override-model-ids-per-version) zuordnen. Für ein fixiertes Modell ist diese ID der Wert, den Sie in seiner `ANTHROPIC_DEFAULT_*_MODEL`-Variable setzen. Alle `[1m]`-Suffixe werden sowohl aus dem Zulassungslisten-Eintrag als auch aus dem angeforderten Modell vor dem Abgleich entfernt.
</Note>

<h3 id="customize-pinned-model-display-and-capabilities">
  Anzeige und Funktionen des fixierten Modells anpassen
</h3>

Wenn Sie ein Modell bei einem Drittanbieter fixieren, zeigt seine Zeile in der `/model`-Auswahl standardmäßig den Namen des Modells an, wenn Claude Code die fixierte ID erkennt, und andernfalls die rohe ID:

* **Erkannt**: die genaue ID eines Modells, das Claude Code kennt, wie seine Anthropic-API-ID oder die Form Ihres Anbieters oder Gateways, mit oder ohne das `[1m]`-Suffix. Fixieren Sie `us.anthropic.claude-sonnet-4-5-20250929-v1:0` und die Zeile liest `Sonnet 4.5`.
* **Nicht erkannt**: jede andere ID, wie ein Anwendungs-Inferenzprofil-ARN oder eine Modellversion, die Claude Code nicht kennt, es sei denn, ein [`modelOverrides`](#override-model-ids-per-version)-Eintrag ordnet ein Modell dieser genauen Zeichenkette zu. Bei Microsoft Foundry sind Bereitstellungsnamen benutzerdefiniert, daher erkennt Claude Code eine fixierte ID dort niemals, zugeordnet oder nicht, und die Zeile zeigt standardmäßig den Bereitstellungsnamen.

Wenn eine Zeile den Namen des Modells anzeigt, enthält seine Standardbeschreibung die fixierte ID, damit Sie immer noch sehen können, welche ID fixiert ist.

Claude Code erkennt möglicherweise auch nicht, welche Funktionen ein fixiertes Modell unterstützt. Sie können den Anzeigenamen und die Beschreibung selbst festlegen und Funktionen mit Begleit-Umgebungsvariablen für jedes fixierte Modell deklarieren.

Diese Variablen wirken sich auf Drittanbieter wie Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry aus. Die Variablen `_NAME` und `_DESCRIPTION` wirken sich auch aus, wenn `ANTHROPIC_BASE_URL` auf ein [LLM-Gateway](/docs/de/llm-gateway) verweist. Sie haben keine Auswirkung bei direkter Verbindung zu `api.anthropic.com`.

| Umgebungsvariable                                     | Beschreibung                                                                                                                                                                                              |
| ----------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_NAME`                   | Anzeigename für das fixierte Opus-Modell in der `/model`-Auswahl. Wenn nicht gesetzt, zeigt die Zeile den Namen des Modells an, wenn Claude Code die fixierte ID erkennt, und die fixierte ID andernfalls |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_DESCRIPTION`            | Anzeige-Beschreibung für das fixierte Opus-Modell in der `/model`-Auswahl. Wenn nicht gesetzt, zeigt die Zeile eine Standardbeschreibung an, die mit `Custom Opus model` beginnt                          |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES` | Komma-getrennte Liste der Funktionen, die das fixierte Opus-Modell unterstützt                                                                                                                            |

Die gleichen `_NAME`-, `_DESCRIPTION`- und `_SUPPORTED_CAPABILITIES`-Suffixe sind für `ANTHROPIC_DEFAULT_SONNET_MODEL`, `ANTHROPIC_DEFAULT_HAIKU_MODEL`, `ANTHROPIC_DEFAULT_FABLE_MODEL` und `ANTHROPIC_CUSTOM_MODEL_OPTION` verfügbar.

Claude Code aktiviert Funktionen wie [Aufwandsniveaus](#adjust-effort-level) und [erweitertes Denken](#extended-thinking) durch Abgleich der Modell-ID mit bekannten Mustern. Anbieterspezifische IDs wie Amazon Bedrock-ARNs oder benutzerdefinierte Bereitstellungsnamen stimmen oft nicht mit diesen Mustern überein, wodurch unterstützte Funktionen deaktiviert bleiben. Setzen Sie `_SUPPORTED_CAPABILITIES`, um Claude Code mitzuteilen, welche Funktionen das Modell tatsächlich unterstützt:

| Funktionswert          | Aktiviert                                                                                    |
| ---------------------- | -------------------------------------------------------------------------------------------- |
| `effort`               | [Aufwandsniveaus](#adjust-effort-level) und der `/effort`-Befehl                             |
| `xhigh_effort`         | Das `xhigh`-Aufwandsniveau                                                                   |
| `max_effort`           | Das `max`-Aufwandsniveau                                                                     |
| `thinking`             | [Erweitertes Denken](#extended-thinking)                                                     |
| `adaptive_thinking`    | Adaptives Reasoning, das das Denken dynamisch basierend auf der Aufgabenkomplexität zuordnet |
| `interleaved_thinking` | Denken zwischen Tool-Aufrufen                                                                |

Wenn `_SUPPORTED_CAPABILITIES` gesetzt ist, werden aufgelistete Funktionen aktiviert und nicht aufgelistete Funktionen werden für das entsprechende fixierte Modell deaktiviert. Wenn die Variable nicht gesetzt ist, greift Claude Code auf die integrierte Erkennung basierend auf der Modell-ID zurück.

Dieses Beispiel fixiert Opus auf ein benutzerdefiniertes Amazon Bedrock-Modell-ARN, setzt einen benutzerfreundlichen Namen und deklariert seine Funktionen:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='arn:aws:bedrock:us-east-1:123456789012:custom-model/abc'
export ANTHROPIC_DEFAULT_OPUS_MODEL_NAME='Opus via Bedrock'
export ANTHROPIC_DEFAULT_OPUS_MODEL_DESCRIPTION='Opus 4.7 routed through a Bedrock custom endpoint'
export ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES='effort,xhigh_effort,max_effort,thinking,adaptive_thinking,interleaved_thinking'
```

<h3 id="override-model-ids-per-version">
  Modell-IDs pro Version überschreiben
</h3>

Auf Plattformen, die Claude Code einbetten und [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/de/env-vars) setzen, hat die Modellkonfiguration des Hosts Vorrang vor verwalteten Modelleinstellungen, während eine verwaltete `availableModels`-Zulassungsliste in Kraft bleibt, es sei denn, der Host stellt seine eigene bereit; [Ausnahmen zur Vorrangigkeit verwalteter Einstellungen](/docs/de/settings#exceptions-to-managed-settings-precedence) sagt, welche Schlüssel und Variablen der Host überschreibt.

Die oben genannten Umgebungsvariablen auf Familienebene konfigurieren eine Modell-ID pro Familienalias. Wenn Sie mehrere Versionen innerhalb der gleichen Familie auf unterschiedliche Anbieter-IDs abbilden müssen, verwenden Sie stattdessen die `modelOverrides`-Einstellung.

`modelOverrides` ordnet einzelne Anthropic-Modell-IDs den anbieterspezifischen Strings zu, die Claude Code an die API Ihres Anbieters sendet. Wenn ein Benutzer ein zugeordnetes Modell in der `/model`-Auswahl auswählt, verwendet Claude Code Ihren konfigurierten Wert anstelle des integrierten Standards.

Dies ermöglicht es Enterprise-Administratoren, jede Modellversion zu einem bestimmten Amazon Bedrock-Inference-Profil-ARN, Google Cloud's Agent Platform-Versionsnamen oder Microsoft Foundry-Bereitstellungsnamen für Governance, Kostenzuteilung oder regionales Routing zu leiten.

Setzen Sie `modelOverrides` in Ihrer [Einstellungsdatei](/docs/de/settings#where-settings-live):

```json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-7": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-prod",
    "claude-opus-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-46-prod",
    "claude-sonnet-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/sonnet-prod"
  }
}
```

Schlüssel müssen Anthropic-Modell-IDs sein, wie in der [Modellübersicht](https://platform.claude.com/docs/en/about-claude/models/overview) aufgelistet. Für datierte Modell-IDs fügen Sie das Datumssuffix genau so ein, wie es dort angezeigt wird. Unbekannte Schlüssel werden ignoriert.

Um die `[claude-code:unrecognized_model]`-[Diagnosezeile](/docs/de/errors#unrecognized-model-id-on-a-request) für eine ID wie einen Gateway-Alias zu stoppen, fügen Sie einen Eintrag mit dieser ID als Wert hinzu.

Überschreibungen ersetzen die integrierten Modell-IDs, die jeden Eintrag in der `/model`-Auswahl unterstützen. Bei Amazon Bedrock haben Überschreibungen Vorrang vor allen Inference-Profilen, die Claude Code beim Start automatisch erkennt. Claude Code übergibt Werte, die bereits anbieterspezifisch sind, wie Amazon Bedrock-Inference-Profil-ARNs oder Microsoft Foundry-Bereitstellungsnamen, unverändert an den Anbieter.

Überschreibungen gelten auch, wenn Sie eine Anthropic-Modell-ID direkt über `--model`, die `ANTHROPIC_MODEL`-Umgebungsvariable oder eine `ANTHROPIC_DEFAULT_*_MODEL`-Umgebungsvariable übergeben. Bei Amazon Bedrock, Google Cloud's Agent Platform und [Mantle](/docs/de/amazon-bedrock#use-the-mantle-endpoint) wird eine Anthropic-Modell-ID ohne `modelOverrides`-Eintrag zur gleichen anbieterspezifischen ID aufgelöst wie die `/model`-Auswahl-Zeile für diese Version, wenn der Anbieter diese Version unterstützt. Mantle unterstützt eine Teilmenge von Versionen. Für eine Anthropic-Modell-ID außerhalb dieser Teilmenge sendet Claude Code die rohe ID an Mantle ohne Zuordnung, es sei denn, ein `modelOverrides`-Eintrag deckt sie ab. Vor v2.1.200 erreichten `--model` und die Umgebungsvariablenwerte den Anbieter unverändert, ohne die Überschreibungskarte zu durchlaufen.

`modelOverrides` funktioniert zusammen mit `availableModels`. Die Zulassungsliste wird gegen die Anthropic-Modell-ID ausgewertet, nicht gegen den Überschreibungswert, daher ein Eintrag wie `"opus"` in `availableModels` stimmt weiterhin überein, auch wenn Opus-Versionen ARNs zugeordnet sind. Wenn `enforceAvailableModels` in verwalteten Einstellungen gesetzt ist, wird der erzwungene Standard durch `modelOverrides` aus [verwalteten Einstellungen](/docs/de/managed-settings#how-claude-code-combines-managed-sources) aufgelöst. Die Zuordnung eines Administrators, wie eine Version, die an ein Inference-Profil-ARN fixiert ist, wird im erzwungenen Standard berücksichtigt. Überschreibungen aus Benutzer- oder Projekteinstellungen beeinflussen ihn nicht.

Wenn `availableModels` in [verwalteten Einstellungen](/docs/de/managed-settings) gesetzt ist, gelten nur `modelOverrides` aus verwalteten Einstellungen für eine Anthropic-Modell-ID, die direkt über `--model` oder die oben genannten Umgebungsvariablen übergeben wird. Claude Code ignoriert Überschreibungen in Benutzer- oder Projekteinstellungen für diese IDs und löst niemals eine ID auf, die die verwaltete Liste ausschließt, durch `modelOverrides` aus einer beliebigen Einstellungsquelle. Diese Einschränkung der verwalteten Quelle erfordert Claude Code v2.1.200 oder später. Siehe [Modellauswahl einschränken](#restrict-model-selection), um zu erfahren, wie blockierte IDs behandelt werden.

<h3 id="prompt-caching-configuration">
  Prompt-Caching-Konfiguration
</h3>

Claude Code verwendet automatisch [Prompt-Caching](/docs/de/prompt-caching), um die Leistung zu optimieren und Kosten zu senken. Sie können Prompt-Caching global oder für bestimmte Modell-Tiers deaktivieren:

| Umgebungsvariable               | Beschreibung                                                                                                                 |
| ------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| `DISABLE_PROMPT_CACHING`        | Setzen Sie auf `1`, um Prompt-Caching für alle Modelle zu deaktivieren. Hat Vorrang vor den modellspezifischen Einstellungen |
| `DISABLE_PROMPT_CACHING_HAIKU`  | Setzen Sie auf `1`, um Prompt-Caching nur für Haiku-Modelle zu deaktivieren                                                  |
| `DISABLE_PROMPT_CACHING_SONNET` | Setzen Sie auf `1`, um Prompt-Caching nur für Sonnet-Modelle zu deaktivieren                                                 |
| `DISABLE_PROMPT_CACHING_OPUS`   | Setzen Sie auf `1`, um Prompt-Caching nur für Opus-Modelle zu deaktivieren                                                   |
| `DISABLE_PROMPT_CACHING_FABLE`  | Setzen Sie auf `1`, um Prompt-Caching nur für Fable-Modelle zu deaktivieren                                                  |

Um die Cache-TTL für das Hauptgespräch und für Subagents separat auszuwählen, siehe [wählen Sie die TTL selbst](/docs/de/prompt-caching#choose-the-ttl-yourself). Für das, was einen Cache-Miss auslöst, siehe [Wie Claude Code Prompt-Caching verwendet](/docs/de/prompt-caching).
