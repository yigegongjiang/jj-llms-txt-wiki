> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Modellkonfiguration

> Erfahren Sie mehr über die Claude Code-Modellkonfiguration, einschließlich Modellaliase wie `opusplan`

<h2 id="available-models">
  Verfügbare Modelle
</h2>

Für die `model`-Einstellung in Claude Code können Sie konfigurieren:

* Einen **Modellalias**
* Einen **Modellnamen**
  * Anthropic API: Einen vollständigen **[Modellnamen](https://platform.claude.com/docs/de/about-claude/models/overview)**
  * Amazon Bedrock: ein Inference-Profil-ARN
  * Microsoft Foundry: einen Bereitstellungsnamen
  * Google Cloud's Agent Platform: einen Versionsnamen

Hinweise zur Auswahl des Modells und der Aufwandsstufe, die für verschiedene Arten von Arbeit geeignet sind, finden Sie unter [Choosing a Claude model and effort level in Claude Code](https://claude.com/blog/claude-model-and-effort-level-in-claude-code) im Blog.

<Note>
  `ANTHROPIC_BASE_URL` ändert, wohin Anfragen gesendet werden, nicht welches Modell sie beantwortet. Um Claude durch ein LLM-Gateway zu leiten, siehe [LLM-Gateways](/docs/de/llm-gateway).
</Note>

<h3 id="model-aliases">
  Modellaliase
</h3>

Modellaliase bieten eine bequeme Möglichkeit, Modelleinstellungen auszuwählen, ohne sich genaue Versionsnummern merken zu müssen:

| Modellalias      | Verhalten                                                                                                                                                                                                                                                                                                                                                                        |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`default`**    | Spezieller Wert, der jede Modellüberschreibung löscht und auf das empfohlene Modell für Ihren Kontotyp zurückgesetzt wird, oder auf das [Organisationsstandardmodell](#organization-default-model), wenn ein Administrator eines festgelegt hat. Ist selbst kein Modellalias                                                                                                     |
| **`best`**       | Verwendet Fable 5, wo Ihre Organisation Zugriff darauf hat, andernfalls das neueste Opus-Modell                                                                                                                                                                                                                                                                                  |
| **`fable`**      | Verwendet Claude Fable 5 für Ihre schwierigsten und längsten Aufgaben                                                                                                                                                                                                                                                                                                            |
| **`sonnet`**     | Verwendet das neueste Sonnet-Modell für tägliche Codierungsaufgaben                                                                                                                                                                                                                                                                                                              |
| **`opus`**       | Verwendet das neueste Opus-Modell für komplexe Reasoning-Aufgaben                                                                                                                                                                                                                                                                                                                |
| **`haiku`**      | Verwendet das schnelle und effiziente Haiku-Modell für einfache Aufgaben                                                                                                                                                                                                                                                                                                         |
| **`sonnet[1m]`** | Verwendet Sonnet mit einem [1-Million-Token-Kontextfenster](https://platform.claude.com/docs/de/build-with-claude/context-windows#context-window-sizes-by-model) für lange Sitzungen. Hat keine Auswirkung, wenn `sonnet` bereits zu Sonnet 5 mit seinem nativen 1M-Fenster aufgelöst wird; hinter einem [LLM-Gateway](/docs/de/llm-gateway) wählt es das 1M-Fenster für Sonnet 5 aus |
| **`opus[1m]`**   | Verwendet Opus mit einem [1-Million-Token-Kontextfenster](https://platform.claude.com/docs/de/build-with-claude/context-windows#context-window-sizes-by-model) für lange Sitzungen                                                                                                                                                                                               |
| **`opusplan`**   | Spezieller Modus, der `opus` während des Plan Mode verwendet und dann zu `sonnet` für die Ausführung wechselt                                                                                                                                                                                                                                                                    |

Die Version, zu der die `opus`- und `sonnet`-Aliase aufgelöst werden, hängt vom Anbieter ab:

| Anbieter                                             | `opus`   | `sonnet`   |
| :--------------------------------------------------- | :------- | :--------- |
| Anthropic API                                        | Opus 4.8 | Sonnet 5   |
| [Claude Platform on AWS](/docs/de/claude-platform-on-aws) | Opus 4.8 | Sonnet 4.6 |
| Amazon Bedrock, Google Cloud's Agent Platform        | Opus 4.8 | Sonnet 4.5 |
| Microsoft Foundry                                    | Opus 4.6 | Sonnet 4.5 |

Wenn ein Alias zu einem älteren Modell aufgelöst wird, sind neuere Modelle verfügbar, indem Sie den vollständigen Modellnamen explizit auswählen oder `ANTHROPIC_DEFAULT_OPUS_MODEL` oder `ANTHROPIC_DEFAULT_SONNET_MODEL` setzen.

Vor v2.1.207 wurde `opus` zu Opus 4.7 auf Claude Platform on AWS und zu Opus 4.6 auf Amazon Bedrock und Google Cloud's Agent Platform aufgelöst.

Aliase verweisen auf die empfohlene Version für Ihren Anbieter und werden im Laufe der Zeit aktualisiert. Um eine bestimmte Version zu fixieren, verwenden Sie den vollständigen Modellnamen, z. B. `claude-opus-4-8`, oder setzen Sie die entsprechende Umgebungsvariable wie `ANTHROPIC_DEFAULT_OPUS_MODEL`.

<Note>
  Sonnet 5 erfordert Claude Code v2.1.197 oder später. Opus 4.8 erfordert v2.1.154 oder später. Führen Sie `claude update` aus, um zu aktualisieren.
</Note>

<h3 id="work-with-fable-5">
  Mit Fable 5 arbeiten
</h3>

[Claude Fable 5](https://platform.claude.com/docs/de/about-claude/models/introducing-claude-fable-5-and-claude-mythos-5) ist das leistungsfähigste Modell in Claude Code, geeignet für Aufgaben, die größer als eine einzelne Sitzung sind. Es unterstützt lange autonome Sitzungen, untersucht vor dem Handeln und überprüft seine Arbeit häufiger als kleinere Modelle.

Fable 5 ist nicht das Standardmodell. Wählen Sie es mit `/model fable` aus. Anfragen, die seine Sicherheitsklassifizierer kennzeichnen, meist in Cybersicherheits- und Biologie-Bereichen, lösen [automatisches Modell-Fallback](#automatic-model-fallback) aus.

Um das Beste aus Fable 5 herauszuholen:

* **Beschreiben Sie das Ergebnis, nicht die Schritte**: Geben Sie ihm das Ergebnis, das Sie möchten, und lassen Sie es den Weg planen. Um es arbeiten zu lassen, bis dieses Ergebnis eintritt, [setzen Sie ein Ziel](/docs/de/goal).
* **Geben Sie ihm mehrdeutige Probleme**: Root-Cause-Untersuchungen, Ausfalldebugging und Architekturentscheidungen sind Bereiche, in denen die zusätzliche Untersuchung und Überprüfung sich auszahlt.
* **Überspringen Sie die Überprüfungserinnerungen**: Es überprüft seine eigene Arbeit mit weniger Aufforderungen, daher sind Erinnerungen zum Testen oder Überprüfen normalerweise unnötig.
* **Größere Aufgaben einschätzen**: Geben Sie ihm Arbeit, die Sie normalerweise in Teile aufteilen würden. Es hält lange Sitzungen, ohne den Faden zu verlieren.

<Note>
  Fable 5 erfordert Claude Code v2.1.170 oder später. Ältere Versionen zeigen Fable 5 nicht in der Modellauswahl an und können es nicht auswählen. Führen Sie `claude update` aus, um zu aktualisieren. Fable 5 ist nicht unter [Zero Data Retention](/docs/de/zero-data-retention) verfügbar, wo die `/model`-Auswahl es entweder auslässt oder deaktiviert anzeigt.
</Note>

<h3 id="setting-your-model">
  Einstellung Ihres Modells
</h3>

Sie können Ihr Modell auf mehrere Arten konfigurieren, aufgelistet nach Priorität:

1. **Während der Sitzung**: Verwenden Sie `/model <alias|name>`, um sofort zu wechseln, oder führen Sie `/model` ohne Argument aus, um die Auswahl zu öffnen. Die Auswahl fordert zur Bestätigung auf, wenn das Gespräch vorherige Ausgaben hat, da die nächste Antwort den vollständigen Verlauf ohne zwischengespeicherten Kontext erneut liest
2. **Beim Start**: Starten Sie mit `claude --model <alias|name>`
3. **Umgebungsvariable**: Setzen Sie `ANTHROPIC_MODEL=<alias|name>`
4. **Einstellungen**: Konfigurieren Sie dauerhaft in Ihrer Einstellungsdatei mit dem `model`-Feld

Ab v2.1.153 speichert `/model` Ihre Auswahl als Standard für neue Sitzungen, indem das `model`-Feld in Ihren Benutzereinstellungen geschrieben wird. In der Auswahl:

* `Enter`: Modell wechseln und als Standard speichern
* `s`: Modell nur für diese Sitzung wechseln

Die direkte Eingabe von `/model <name>` verhält sich wie `Enter`. Ein Modell, das mit `/model` im [nicht-interaktiven Modus](/docs/de/headless) mit dem `-p`-Flag gesetzt wird, gilt nur für die aktuelle Sitzung und wird nicht als Standard gespeichert. Projekt- und verwaltete Einstellungen haben weiterhin Vorrang und werden beim nächsten Start erneut angewendet. Ein [Organisationsstandardmodell](#organization-default-model), das Ihr Administrator konfiguriert hat, um die Benutzerauswahl zu überschreiben, wird auch beim nächsten Start erneut angewendet.

In v2.1.144 bis v2.1.152 galt `/model` nur für die aktuelle Sitzung und `d` in der Auswahl speicherte einen Standard.

Das `--model`-Flag und die `ANTHROPIC_MODEL`-Umgebungsvariable gelten nur für die Sitzung, mit der Sie sie starten. Um verschiedene Modelle in verschiedenen Terminals gleichzeitig auszuführen, starten Sie jedes mit seinem eigenen `--model`-Flag, anstatt mit `/model` zu wechseln.

Preise in der `/model`-Auswahl werden angezeigt, wenn Claude Code mit der Anthropic API spricht, direkt oder über ein [LLM-Gateway](/docs/de/llm-gateway), das es vermittelt, und der Preis in einer Zeile ist der Preis des Modells, das diese Zeile auswählt. Bei [Drittanbieter-Anbietern](/docs/de/third-party-integrations) wie Amazon Bedrock und beim [Claude-Apps-Gateway](/docs/de/claude-apps-gateway) bestimmt Ihr Anbieter oder Gateway, was Sie zahlen, daher zeigen Auswahlzeilen keinen Preis. Der Preis ist nur ein Anzeigelabel; er beeinflusst nicht, welches Modell eine Zeile auswählt oder was Ihr Anbieter abrechnet. Vor v2.1.206 zeigten [Claude Platform on AWS](/docs/de/claude-platform-on-aws) und Gateway-Sitzungen Anthropic-Listenpreise an, und eine Zeile konnte den Preis eines anderen Modells anzeigen als das, das sie auswählte.

Fortgesetzte Sitzungen, die mit `claude --resume`, `--continue` oder der `/resume`-Auswahl gestartet werden, behalten das Modell bei, das sie verwendeten, als das Transkript gespeichert wurde, unabhängig von der aktuellen `model`-Einstellung. Wenn das angeforderte Modell eingestellt wurde oder durch [`availableModels`](#restrict-model-selection) ausgeschlossen ist, fällt die Sitzung auf die normale Prioritätsreihenfolge zurück. Dies verhindert, dass die `/model`-Auswahl einer anderen Sitzung das Modell beim Fortsetzen ändert.

Ein Modell, das Sie für den neuen Start mit `--model` oder `ANTHROPIC_MODEL` auswählen, hat weiterhin Vorrang vor dem wiederhergestellten Modell. Ab v2.1.195 gilt dies auch für eine [`ANTHROPIC_DEFAULT_OPUS_MODEL`](#environment-variables)-Familienvariable.

Wenn das aktive Modell beim Start aus Projekt- oder verwalteten Einstellungen stammt und nicht aus Ihrer eigenen Auswahl, zeigt der Startheader an, welche Einstellungsdatei es festgelegt hat. Führen Sie `/model` aus, um es zu überschreiben; die Projekt- oder verwaltete Einstellung wird beim nächsten Start erneut angewendet.

Wenn ein Modellwechsel durch die [Agent SDK](/docs/de/agent-sdk/overview) `setModel()`-Methode oder durch eine App wie die [Desktop-App](/docs/de/desktop), die die Claude Code CLI für Sie ausführt, angefordert wird, überprüft Claude Code, dass die Zeichenkette eine ist, die es erkennt, bevor es sie speichert. Diese Überprüfung erfordert Claude Code v2.1.200 oder später. Bei der Anthropic API erkennt Claude Code:

* einen Modellalias
* einen Eintrag aus der `/model`-Auswahl
* jeden Namen, der mit `claude-` beginnt
* einen Wert, den Sie selbst als [benutzerdefinierte Modelloption](#add-a-custom-model-option) oder in [`modelOverrides`](#override-model-ids-per-version) konfiguriert haben

Claude Code lehnt eine unbekannte Zeichenkette mit `Model "<name>" is not a recognized model id.` ab und die Sitzung behält ihr aktuelles Modell bei, anstatt die Zeichenkette zu speichern und beim nächsten Request zu fehlschlagen. Siehe [die Fehlerreferenz](/docs/de/errors#model-is-not-a-recognized-model-id) für Wiederherstellungsschritte.

Die Überprüfung wird nur bei der Anthropic API durchgeführt. Bei Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, [Claude Platform on AWS](/docs/de/claude-platform-on-aws) und hinter einem [LLM-Gateway](/docs/de/llm-gateway) oder einem benutzerdefinierten `ANTHROPIC_BASE_URL` definiert Ihr Anbieter oder Gateway die Modellnamen, daher leitet Claude Code jede Zeichenkette ohne Überprüfung durch. Die Überprüfung deckt auch nicht das `--model`-Flag, die `ANTHROPIC_MODEL`-Umgebungsvariable oder die `model`-Einstellung ab; ein Tippfehler dort erzeugt [There's an issue with the selected model](/docs/de/errors#theres-an-issue-with-the-selected-model) beim ersten Request statt.

Wenn das angeforderte Modell ein geplantes Ruhestandsdatum hat oder automatisch auf eine neuere Version neu zugeordnet wird, zeigt Claude Code eine Warnung an, die das angeforderte Modell benennt. Interaktive Sitzungen zeigen es als Startnachricht an. Ab v2.1.182 wird die gleiche Warnung in [nicht-interaktivem Modus](/docs/de/headless) auf stderr geschrieben, wenn das Standard-Textausgabeformat verwendet wird. Die Überprüfung deckt auch ein `model` ab, das in [Subagent-Frontmatter](/docs/de/sub-agents) festgelegt ist. Die stderr-Warnung wird für `--output-format json` und `stream-json` unterdrückt; lesen Sie das tatsächliche Modell stattdessen aus dem `modelUsage`-Feld der [Ergebnisnachricht](/docs/de/headless#get-structured-output).

Beispielverwendung:

```bash theme={null}
# Mit Opus starten
claude --model opus

# Während der Sitzung zu Sonnet wechseln
/model sonnet
```

Beispiel-Einstellungsdatei:

```json theme={null}
{
    "permissions": {
        ...
    },
    "model": "opus"
}
```

<h2 id="restrict-model-selection">
  Modellauswahl einschränken
</h2>

Enterprise-Administratoren können `availableModels` in [verwalteten oder Richtlinieneinstellungen](/docs/de/settings#settings-files) verwenden, um einzuschränken, welche Modelle Benutzer auswählen können. Einträge entsprechen einer Modellfamilie wie `sonnet`, einem Versionspräfix wie `claude-sonnet-4-5` oder einer vollständigen Modell-ID wie `claude-sonnet-4-5-20250929`.

Wenn `availableModels` gesetzt ist, gilt die Zulassungsliste überall dort, wo ein Benutzer ein Modell angeben kann:

* **Hauptsitzungsmodell**: `/model`, das `--model`-Flag, die `ANTHROPIC_MODEL`-Umgebungsvariable, die `model`-Einstellung und das Modell, das beim [Fortsetzen einer Sitzung](#setting-your-model) wiederhergestellt wird
* **Alias-Auflösung**: die Umgebungsvariablen `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, `ANTHROPIC_DEFAULT_HAIKU_MODEL` und `ANTHROPIC_DEFAULT_FABLE_MODEL` können einen zulässigen Alias nicht zu einem Modell außerhalb der Liste umleiten
* **Schnellmodus**: `/fast` weigert sich umzuschalten, wenn dies implizit zu einem Opus-Modell außerhalb der Liste führen würde, mit der Meldung „is not in your organization's allowed models"
* **Subagent-Modelle**: das `model`-Feld in [Subagent](/docs/de/sub-agents#choose-a-model)-Frontmatter, der `model`-Parameter des Agent-Tools, `CLAUDE_CODE_SUBAGENT_MODEL` und, auf v2.1.197 und früher, die Modellauswahl im `/agents`-Assistent&#x20;
* **Skill- und Befehlsmodelle**: das `model`-Frontmatter in [Skills und Befehlen](/docs/de/skills)
* **Advisor-Modell**: die konfigurierte [`advisorModel`](/docs/de/advisor)-Einstellung und das `--advisor`-Flag
* **Hintergrund-Agent-Modell**: das Modell, das in der [Dispatch-Auswahl](/docs/de/agent-view) ausgewählt ist

Auf der Anthropic API und [Claude Platform auf AWS](/docs/de/claude-platform-on-aws) wird ein Modellfamilien-Alias, `opus`, `sonnet`, `haiku` oder `fable`, zur neuesten Version seiner Familie aufgelöst, die die Zulassungsliste zulässt. Wenn die Zulassungsliste bestimmte Versionen fixiert, beispielsweise `["sonnet", "claude-opus-4-6"]`, wählen sowohl `/model opus` als auch `--model opus` Claude Opus 4.6, die neueste zulässige Opus-Version, und zeigen eine Mitteilung an, die sowohl das angeforderte als auch das ersetzte Modell benennt. Vor v2.1.205 wurde ein Alias, dessen neueste veröffentlichte Version außerhalb der Liste lag, wie jede andere blockierte Auswahl abgelehnt oder ersetzt, auch wenn die Liste eine ältere Version zulässig machte.

Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry und [Mantle](/docs/de/amazon-bedrock#use-the-mantle-endpoint) verwenden anbieterspezifische Bereitstellungs-IDs anstelle von Anthropic-Modell-IDs, daher folgt ein blockierter Alias dort dem Ablehnungs- und Ersetzungsverhalten unten.

Claude Code behandelt jede andere blockierte Auswahl je nachdem, wo das Modell gesetzt wurde:

* **`/model`**: der Wechsel wird mit einem Fehler abgelehnt
* **`--model`-Flag, `ANTHROPIC_MODEL` oder die `model`-Einstellung**: der Wert wird beim Start mit einer Warnung ersetzt, die sowohl das angeforderte als auch das ersetzte Modell benennt, und die Sitzung startet auf dem Standardmodell
* **Subagent-, Skill- oder Befehl-Override**: der Override fällt auf das geerbte oder Standardmodell zurück, anstatt die Anfrage fehlschlagen zu lassen
* **`advisorModel`-Einstellung**: der Advisor wird für die Sitzung deaktiviert
* **`--advisor`-Flag**: Claude Code beendet sich beim Start mit einem Fehler

Ausgeschlossene Modelle sind in der `/model`-Auswahl verborgen. Eine vollständige Modell-ID in der Liste, die keine integrierte Picker-Zeile hat, wie eine ältere Version, die die Liste fixiert, erscheint in der `/model`-Auswahl als eigene beschriftete Zeile. Vor v2.1.199 war eine solche ID nur durch Eingabe von `/model <id>` auswählbar.

Modellwechsel, die Claude Code in Ihrem Namen vornimmt, werden auf die gleiche Weise überprüft:

* **[Fallback-Modellketten](#fallback-model-chains)**: Elemente außerhalb der Zulassungsliste werden gelöscht
* **Plan-Modus-Upgrades**: auf der Anthropic API und Claude Platform auf AWS verwendet ein Upgrade wie [`opusplan`](#opusplan-model-setting) zu einem ausgeschlossenen Modell die neueste zulässige Version der Upgrade-Familie. Bei Anbietern mit anbieterspezifischen Modell-IDs und wenn keine Version zulässig ist, wird das Upgrade übersprungen und die Planung wird auf dem Sitzungsmodell fortgesetzt
* **[Automatisches Modell-Fallback](#automatic-model-fallback)**: ein Fallback, dessen Ziel ausgeschlossen ist, wird nicht ausgeführt, daher endet die gekennzeichnete Anfrage mit einer Ablehnung statt
* **[Schnellmodus](/docs/de/fast-mode)**: das Aktivieren des Schnellmodus wird abgelehnt, wenn das Modell, auf dem die Sitzung danach ausgeführt würde, außerhalb der Zulassungsliste liegt

```json theme={null}
{
  "availableModels": ["sonnet", "haiku"]
}
```

<h3 id="surface-coverage">
  Oberflächenabdeckung
</h3>

Jede Oberfläche erzwingt die Zulassungsliste, die sie erhält. Welcher Bereitstellungsmechanismus jede Oberfläche erreicht, unterscheidet sich:

| Bereitstellungsmechanismus                                                          | CLI und IDE | Desktop-Lokalsitzungen | Web-, Mobil- und Cloud-Sitzungen | Agent SDK und nicht-interaktiv | Cowork                       |
| :---------------------------------------------------------------------------------- | :---------- | :--------------------- | :------------------------------- | :----------------------------- | :--------------------------- |
| [Serververwaltete Einstellungen](/docs/de/server-managed-settings) aus der Admin-Konsole | Erzwungen   | Erzwungen              | Erzwungen                        | Erzwungen                      | Nicht bereitgestellt         |
| [MDM oder verwaltete Einstellungsdateien](/docs/de/settings#settings-files)              | Erzwungen   | Erzwungen              | Nicht bereitgestellt             | Erzwungen                      | Erzwungen, wo bereitgestellt |

* Cloud-Sitzungen auf [Claude Code im Web](/docs/de/claude-code-on-the-web) oder in der Desktop-App werden auf von Anthropic verwalteten VMs ausgeführt: Einstellungen, die auf Ihrem Gerät bereitgestellt werden, erreichen sie nicht, daher stellen Sie die Zulassungsliste über serververwaltete Einstellungen bereit. Ein Modellwechsel während einer Sitzung in einer Cloud-Sitzung wird abgelehnt, wenn das angeforderte Modell durch die Zulassungsliste ausgeschlossen ist. Die serverseitige Ablehnung bei der Sitzungserstellung gilt für [Organisationsmodellbeschränkungen](#organization-model-restrictions), nicht für den `availableModels`-Einstellungsschlüssel.
* Cowork, die agentengesteuerte Arbeitsregisterkarte in der Claude Desktop-App, ist keine Claude Code-Oberfläche und erhält serververwaltete Einstellungen nicht absichtlich. Eine verwaltete Einstellungsdatei gilt für Cowork-Sitzungen, wenn sie dort vorhanden ist, wo die Sitzung ausgeführt wird; Remote-Cowork-Sitzungen werden auf von Anthropic verwalteten VMs ausgeführt, wo eine gerätebereits bereitgestellte Datei nicht vorhanden ist.
* Sitzungen auf [Drittanbieter-Anbietern](/docs/de/server-managed-settings#platform-availability) wie Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry und [Claude Platform auf AWS](/docs/de/claude-platform-on-aws) erhalten keine serververwalteten Einstellungen, daher stellen Sie die Zulassungsliste dort über MDM oder verwaltete Einstellungsdateien bereit.
* Die serververwaltete Bereitstellung erfordert auch, dass sich die Sitzung mit einem Organisationslogin oder einem direkt konfigurierten API-Schlüssel authentifiziert. Flotten, die Schlüssel nur über ein [`apiKeyHelper`](/docs/de/settings#available-settings)-Skript generieren, sollten die Zulassungsliste über MDM oder verwaltete Einstellungsdateien bereitstellen.
* Die Desktop Code-Registerkarte hostet auch [SSH-Sitzungen](/docs/de/desktop#ssh-sessions), die die verwaltete Einstellungsdatei vom Remote-Host lesen, auf dem sie ausgeführt werden. Siehe [Desktop-verwaltete Einstellungen](/docs/de/desktop#managed-settings).
* Die Modellauswahl auf claude.ai und in der Desktop-App verbergen oder graut Modelle aus, die durch die Zulassungsliste Ihrer Organisation ausgeschlossen sind. Der Auswahlzustand ist eine Annehmlichkeit für Benutzer; die Erzwingung erfolgt in der Sitzung.

<h3 id="default-model-behavior">
  Standardmodell-Verhalten
</h3>

Die Option „Standard" in der Modellauswahl wird nicht von `availableModels` beeinflusst, es sei denn, [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) ist auch gesetzt. Allein lässt `availableModels` „Standard" verfügbar, das sich zum [Laufzeit-Standard](#default-model-setting) des Systems auflöst. Wenn dieser Standard ein Modell ist, das Sie einschränken möchten, setzen Sie auch `enforceAvailableModels`.

Ein leeres `availableModels`-Array aktiviert niemals die Erzwingung des Standardmodells: Mit `availableModels: []` werden benannte Modellauswahlen blockiert, aber das Standardmodell für den Kontotyp bleibt unabhängig von `enforceAvailableModels` nutzbar.

<h3 id="enforce-the-allowlist-for-the-default-model">
  Erzwingen Sie die Zulassungsliste für das Standardmodell
</h3>

Setzen Sie `enforceAvailableModels: true` zusammen mit einer nicht leeren `availableModels` in verwalteten Einstellungen, um die Zulassungsliste auf die Option „Standard" auszudehnen. Dies erfordert Claude Code v2.1.175 oder später.

```json theme={null}
{
  "availableModels": ["sonnet", "haiku"],
  "enforceAvailableModels": true
}
```

Das Standardmodell wird zum Kontotyp-Standard oder zum [Organisationsstandardmodell](#organization-default-model) aufgelöst, wenn ein Administrator eines festgelegt hat. Wenn dieses Modell nicht in der Zulassungsliste enthalten ist, wird die Option „Standard" stattdessen auf den ersten `availableModels`-Eintrag aufgelöst, der ein zulässiges, verfügbares Modell benennt, und die Zeile „Standard" der `/model`-Auswahl zeigt dieses Modell. Dies gilt überall dort, wo der Standard erreicht wird: Sitzungsstart, Auswahl von „Standard" in `/model`, das Schlüsselwort `"default"` in [Fallback-Modellketten](#fallback-model-chains) und das Fallback, das verwendet wird, wenn eine ausgeschlossene Auswahl gelöscht wird.

`enforceAvailableModels` hat keine Auswirkung, wenn `availableModels` nicht gesetzt oder leer ist: Mit `availableModels: []` bleibt das Standardmodell für den Kontotyp nutzbar, sodass die Einstellung Benutzer nicht aus jedem Modell sperren kann. Wenn `availableModels` nicht leer ist, aber kein Eintrag zu einem zulässigen und verfügbaren Modell aufgelöst wird, wird die Erzwingung herabgestuft und „Standard" fällt auf den Kontotyp-Standard zurück, mit einer Warnung, die nur unter `--debug` sichtbar ist. Behalten Sie mindestens einen garantiert verfügbaren Eintrag in der Liste, um dies zu vermeiden.

Stellen Sie beide Schlüssel in der [höchsten Prioritäts-verwalteten Quelle](/docs/de/settings#settings-precedence) bereit: Von Administratoren bereitgestellte verwaltete Quellen werden nicht zusammengeführt, daher wird ein Paar, das in einer verwalteten Einstellungsdatei platziert ist, ignoriert, wenn die Admin-Konsole Einstellungen bereitstellt.

<h3 id="control-the-model-users-run-on">
  Kontrollieren Sie das Modell, auf dem Benutzer ausgeführt werden
</h3>

Die `model`-Einstellung ist eine anfängliche Auswahl, keine Erzwingung. Sie legt fest, welches Modell aktiv ist, wenn eine Sitzung startet, aber Benutzer können weiterhin `/model` öffnen und „Standard" auswählen, das sich zum [Laufzeit-Standard](#default-model-setting) des Systems auflöst, unabhängig davon, was `model` gesetzt ist, es sei denn, [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) leitet es um.

Um die Modellerfahrung vollständig zu kontrollieren, kombinieren Sie diese Einstellungen:

* **`availableModels`**: schränkt ein, welche benannten Modelle Benutzer wechseln können
* **`enforceAvailableModels`**: erweitert die `availableModels`-Zulassungsliste auf die Option „Standard", sodass „Standard" nicht auf ein Modell außerhalb der Liste aufgelöst werden kann
* **`model`**: legt die anfängliche Modellauswahl fest, wenn eine Sitzung startet
* **`ANTHROPIC_DEFAULT_SONNET_MODEL`** / **`ANTHROPIC_DEFAULT_OPUS_MODEL`** / **`ANTHROPIC_DEFAULT_HAIKU_MODEL`** / **`ANTHROPIC_DEFAULT_FABLE_MODEL`**: steuern, worauf sich die Option „Standard" und die Aliase `sonnet`, `opus`, `haiku` und `fable` auflösen

Dieses Beispiel startet Benutzer auf Sonnet 4.5, begrenzt die Auswahl auf Sonnet und Haiku und stellt sicher, dass „Standard" auf ein Modell in der Zulassungsliste aufgelöst wird, anstatt auf den Tier-Standard:

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

Ohne `enforceAvailableModels` oder den `env`-Block würde ein Benutzer, der „Standard" in der Auswahl auswählt, die neueste Version für seinen Tier erhalten und damit die Versionsfixierung in `model` und `availableModels` umgehen. Die beiden Einstellungen decken unterschiedliche Bereiche ab: `enforceAvailableModels` macht „Standard" der Zulassungsliste unterworfen, während der `env`-Block festlegt, auf welche Version ein zulässiger Alias wie `sonnet` aufgelöst wird. Verwenden Sie `enforceAvailableModels` allein, wenn das Einschränken von Modellfamilien ausreicht; fügen Sie den `env`-Block hinzu, wenn Sie auch eine bestimmte Version fixieren müssen.

<h3 id="merge-behavior">
  Merge-Verhalten
</h3>

Wenn die [höchste Prioritäts-verwaltete Einstellungsquelle](/docs/de/server-managed-settings#settings-precedence) `availableModels` definiert, gilt diese Liste allein: Einträge in Benutzer-, Projekt- oder lokalen Einstellungen können sie nicht erweitern, und von Administratoren bereitgestellte verwaltete Quellen werden nicht miteinander zusammengeführt, daher wird eine Liste, die in einer verwalteten Einstellungsdatei bereitgestellt wird, ignoriert, wenn serververwaltete Einstellungen Schlüssel bereitstellen. Andernfalls werden Listen aus Benutzer-, Projekt- und lokalen Einstellungen wie andere Array-Einstellungen [verkettet und dedupliziert](/docs/de/settings#settings-precedence). Ab Claude Code v2.1.175 ersetzt die verwaltete Liste Einträge mit niedrigerer Priorität; frühere Versionen führen sie zusammen.

Innerhalb der effektiven Liste deaktiviert ein Eintrag, der ein bestimmtes Modell in einer Familie benennt, ob ein Versionspräfix oder eine vollständige Modell-ID, den Wildcard-Eintrag dieser Familie: `["sonnet", "claude-sonnet-4-5"]` erlaubt nur Sonnet 4.5-Versionen, nicht jedes Sonnet-Modell.

<h3 id="mantle-model-ids">
  Mantle-Modell-IDs
</h3>

Wenn der [Amazon Bedrock Mantle-Endpunkt](/docs/de/amazon-bedrock#use-the-mantle-endpoint) aktiviert ist, werden Einträge in `availableModels`, die mit `anthropic.` beginnen, als benutzerdefinierte Optionen zur `/model`-Auswahl hinzugefügt und zum Mantle-Endpunkt weitergeleitet. Dies ist eine Ausnahme zu der in [Modelle für Drittanbieter-Bereitstellungen fixieren](#pin-models-for-third-party-deployments) beschriebenen Alias-Zuordnung. Die Einstellung schränkt die Auswahl weiterhin auf aufgelistete Einträge ein, und eine Mantle-ID enthält einen Familiennamen, daher zählt sie als spezifischer Eintrag und deaktiviert den Wildcard dieser Familie: Führen Sie zusammen mit allen Mantle-IDs die Versionspräfixe oder vollständigen IDs auf, die Sie selektierbar halten möchten. Siehe [Merge-Verhalten](#merge-behavior).

<h3 id="organization-model-restrictions">
  Organisationsmodellbeschränkungen
</h3>

Organisationsadministratoren auf Claude Enterprise-Plänen schränken ein, welche Modelle Mitglieder ausführen können, indem sie einzelne Modelle in der claude.ai Admin-Konsole deaktivieren. Diese Beschränkung wird mit den Berechtigungen des Kontos bereitgestellt, wenn Claude Code sich authentifiziert, getrennt von jeder `availableModels`-Liste in Einstellungen, und der Server erzwingt die gleiche Beschränkung unabhängig, wenn eine Sitzung erstellt wird. Erfordert Claude Code v2.1.187 oder später.

Die Beschränkung gilt, wenn sich ein Mitglied anmeldet oder seinen eigenen API-Schlüssel verwendet. Organisationsbezogene Anmeldedaten, wie Organisationsservice-Schlüssel, sind nicht an einen Benutzer gebunden, daher gilt die Beschränkung nicht für sie.

Die Claude Console hat keine Modellbeschränkungskontrolle. Organisationen ohne Claude Enterprise-Plan, einschließlich derjenigen, deren Mitglieder sich über die Anthropic API authentifizieren, schränken Modelle mit [`availableModels`](#restrict-model-selection) in [verwalteten Einstellungen](/docs/de/settings#settings-files) ein, wobei [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) hinzugefügt wird, um die Option „Standard" abzudecken. Diese Einstellungen werden von Claude Code selbst erzwungen, nicht vom Server.

Ein eingeschränktes Modell ist in der `/model`-Auswahl verborgen. Die Auswahl nach Name mit `--model`, der `ANTHROPIC_MODEL`-Umgebungsvariable oder der `model`-Einstellung zeigt die Mitteilung `Model "<name>" is restricted by your organization's settings. Using <model> instead.` und die Sitzung startet auf einem zulässigen Modell. Die Eingabe von `/model <name>` für ein eingeschränktes Modell wird mit `Model '<name>' is restricted by your organization's settings. Run /model to choose a different model.` abgelehnt und die Sitzung behält ihr aktuelles Modell.

Ein [Modellfamilien-Alias](#restrict-model-selection) wie `opus` wird zur neuesten Version seiner Familie aufgelöst, die die Organisation zulässt, mit der gleichen Ersetzungsmitteilung. `/model <alias>` wird nur abgelehnt, wenn jede Version seiner Familie eingeschränkt ist; ein Alias, der mit `--model`, `ANTHROPIC_MODEL` oder der `model`-Einstellung gesetzt wird, wird beim Start in diesem Fall immer noch ersetzt. Vor v2.1.205 wurde ein Familienalias basierend auf seiner neuesten veröffentlichten Version allein ersetzt oder abgelehnt, auch wenn eine ältere Version zulässig war.

Beschränkungen gelten organisationsweit oder pro Rolle:

* Das Deaktivieren eines Modells auf Organisationsebene entfernt es für jedes Mitglied.
* Rollenbezogener Zugriff gewährt verschiedenen benutzerdefinierten Rollen unterschiedliche Modelle, und ein Mitglied, das mehrere Rollen innehat, kann jedes Modell verwenden, das eine seiner Rollen gewährt.
* Haiku-Modelle sind immer verfügbar und können nicht deaktiviert werden, daher behält jedes Mitglied mindestens ein nutzbares Modell.
* Eine Zugriffänderung wird innerhalb von etwa einer Minute bei neuen Anfragen wirksam; die `/model`-Auswahl spiegelt dies wider, wenn eine Sitzung das nächste Mal startet.

Beide Beschränkungen gelten zusammen: Ein Modell ist nur selektierbar, wenn es durch `availableModels` zulässig ist und nicht durch die Organisation eingeschränkt ist. Organisationsbeschränkungen werden an Sitzungen auf der Anthropic API und [LLM Gateway](/docs/de/llm-gateway)-Bereitstellungen bereitgestellt. Sitzungen auf Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry und Claude Platform auf AWS erhalten sie nicht, daher verwenden Sie stattdessen `availableModels` auf diesen Anbietern.

<h2 id="organization-default-model">
  Organisationsstandardmodell
</h2>

Organisationsadministratoren auf Claude Enterprise-Plänen können ein Standardmodell für Claude Code-Mitglieder aus der claude.ai Admin-Konsole festlegen, für die gesamte Organisation oder pro benutzerdefinierte Rolle. Wenn eines festgelegt ist, wird die Option „Standard" zu diesem Modell aufgelöst, anstatt zum [Kontotyp-Standard](#default-model-setting). Erfordert Claude Code v2.1.196 oder später.

Die Zeile „Standard" in der `/model`-Auswahl zeigt den Namen des Organisationsstandardmodells mit der Bezeichnung „Org default". Die Bezeichnung lautet „Org default", ob der Administrator den Standard für die gesamte Organisation oder für Ihre Rolle festgelegt hat. Ein Rollen-Standard deckt Mitglieder dieser benutzerdefinierten Rolle ab und hat Vorrang vor dem organisationsweiten Standard; wenn mehrere Ihrer Rollen unterschiedliche Standards festlegen, gilt das leistungsfähigste Modell.

Das Organisationsstandardmodell ist ein Ausgangspunkt, keine Beschränkung, und jede andere Modellauswahl hat Vorrang vor ihm:

* das `--model`-Flag und die `ANTHROPIC_MODEL`-Umgebungsvariable
* ein `model`-Wert in [verwalteten Einstellungen](/docs/de/settings#settings-files) oder bereitgestellt über `--settings`
* ein `model`-Wert in Ihren Benutzer-, Projekt- oder lokalen Einstellungen, einschließlich eines Modells, das Sie mit `/model` speichern

Administratoren können auch das Organisationsstandardmodell konfigurieren, um die Benutzerauswahl zu überschreiben. Mit Überschreibung aktiviert hat es Vorrang vor dem `model`-Wert in Benutzer-, Projekt- und lokalen Einstellungen, daher gilt ein Modell, das Sie mit `/model` speichern, für die aktuelle Sitzung und das Organisationsstandardmodell kehrt beim nächsten Start zurück. Wenn sich Ihre Auswahl unterscheidet, zeigt `/model` `Your organization's default (<model>) applies on restart`. Das `--model`-Flag, `ANTHROPIC_MODEL`, verwaltete Einstellungen und `--settings` haben weiterhin Vorrang, auch wenn die Überschreibung aktiviert ist. Die Überschreibung ist für eine begrenzte Anzahl von Organisationen verfügbar; fragen Sie Ihr Anthropic-Kontoteam nach der Verfügbarkeit.

Um einzuschränken, welche Modelle Mitglieder auswählen können, verwenden Sie stattdessen [Organisationsmodellbeschränkungen](#organization-model-restrictions) oder [`availableModels`](#restrict-model-selection).

Claude Code liest das Organisationsstandardmodell einmal beim Start, daher wird ein Standard, den der Administrator während einer Sitzung ändert, beim nächsten Start wirksam.

Wenn das Organisationsstandardmodell die Benutzerauswahl nicht überschreibt, löscht der erste interaktive Start nach der Änderung des Administrators den `model`-Schlüssel aus Ihren Benutzereinstellungen einmal, damit der neue Standard gilt. Es ändert nichts anderes in der Datei, und ein Modell, das Sie nach diesem Start mit `/model` speichern, wird beibehalten.

Das Organisationsstandardmodell durchläuft die gleichen Beschränkungsprüfungen wie jedes andere Standardmodell, bevor es angenommen wird:

* [`availableModels`](#restrict-model-selection) allein schränkt die Option „Standard" niemals ein, daher gilt ein Organisationsstandardmodell außerhalb der Zulassungsliste weiterhin. Wenn [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) auch gesetzt ist, wird ein Organisationsstandardmodell außerhalb der Zulassungsliste zum ersten Zulassungslisten-Eintrag neu zugeordnet, wie jedes andere Standard
* ein Organisationsstandardmodell, das [Organisationsmodellbeschränkungen](#organization-model-restrictions) für Ihr Konto verweigern, wird durch das neueste zulässige Modell in seiner Familie oder eine kostengünstigere Familie ersetzt, wenn jede Version davon eingeschränkt ist
* ein Organisationsstandardmodell, das Ihrem Konto überhaupt nicht verfügbar ist, wie Fable 5 unter [Zero Data Retention](/docs/de/zero-data-retention), wird übersprungen, und die Option „Standard" wird zum Kontotyp-Standard aufgelöst

Ab v2.1.199 behält die `/model`-Auswahl eine separate Zeile für die übliche Familie Ihres Kontotyps, wenn das Organisationsstandardmodell eine andere Modellfamilie ist, damit Sie für eine Sitzung noch zu ihr wechseln können. In v2.1.196 bis v2.1.198 fehlt diese Zeile in der Auswahl.

Das Organisationsstandardmodell wird an Sitzungen bereitgestellt, die sich mit der Anthropic API authentifizieren. Sitzungen auf [LLM Gateway](/docs/de/llm-gateway)-Bereitstellungen, Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry und Claude Platform auf AWS erhalten es nicht. Um einen Standard auf diesen Bereitstellungen zu setzen, verwenden Sie stattdessen den `model`-Schlüssel in [verwalteten Einstellungen](/docs/de/settings#settings-files).

<h2 id="organization-effort-limits">
  Organisationsaufwandsgrenzen
</h2>

Organisationsadministratoren auf Claude Enterprise-Plänen können ein maximales [Aufwandsniveau](#adjust-effort-level) pro Modell für jede benutzerdefinierte Rolle neben [Organisationsmodellbeschränkungen](#organization-model-restrictions) auf Rollenebene festlegen. Ebenen über der Obergrenze werden nicht in der `/effort`-Auswahl angeboten, und die Benennung einer höheren Ebene mit `--effort` oder `/effort` wird stattdessen mit der Obergrenze ausgeführt. In interaktiven Sitzungen und einfachen Text-`--print`-Läufen wird eine Warnung angezeigt, die die angeforderten und angewendeten Ebenen benennt; bei `json`- oder `stream-json`-Ausgabe oder in Hintergrund-Agenten wird die Begrenzung stillschweigend angewendet. Obergrenzen sind pro Modell, daher kann der Wechsel von Modellen ändern, welche Ebenen verfügbar sind. Wenn mehrere Ihrer Rollen das gleiche Modell gewähren, gilt die am wenigsten restriktive Obergrenze. Erfordert Claude Code v2.1.195 oder später.

Aufwandsgrenzen werden zusammen mit [Organisationsmodellbeschränkungen](#organization-model-restrictions) bereitgestellt und folgen der gleichen Anbieter-Verfügbarkeit: Sitzungen auf Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry und Claude Platform auf AWS erhalten sie nicht.

<h2 id="special-model-behavior">
  Spezielles Modellverhalten
</h2>

<h3 id="default-model-setting">
  `default`-Modelleinstellung
</h3>

Das Verhalten von `default` hängt von Ihrem Kontotyp ab:

* **Max, Team Premium, Enterprise Pay-as-you-go und Anthropic API**: Standard ist Opus 4.8
* **Claude Platform auf AWS, Amazon Bedrock und Google Cloud's Agent Platform**: Standard ist Opus 4.8
* **Pro, Team Standard und Enterprise-Abonnementplätze**: Standard ist Sonnet 5
* **Microsoft Foundry**: Standard ist Sonnet 4.5

Enterprise Pay-as-you-go bedeutet eine Enterprise-Organisation, die nach Nutzung statt nach Abonnementplatz abgerechnet wird.

Vor v2.1.207 wurde `default` auf Claude Platform auf AWS zu Opus 4.7 aufgelöst und auf Amazon Bedrock und Google Cloud's Agent Platform zu Sonnet 4.5.

Wenn ein Administrator ein [Organisationsstandardmodell](#organization-default-model) festgelegt hat, wird `default` zu diesem Modell aufgelöst, anstatt zum Kontotyp-Standard oben. Erfordert Claude Code v2.1.196 oder später.

Wenn verwaltete Einstellungen [die Zulassungsliste für das Standardmodell erzwingen](#enforce-the-allowlist-for-the-default-model) und der Kontotyp-Standard nicht in `availableModels` enthalten ist, wird `default` zum erzwungenen Standard aufgelöst, anstatt zum Kontotyp-Standard oben. Wenn beide gelten, ersetzt das Organisationsstandardmodell zuerst den Kontotyp-Standard und die Erzwingung wird dann darauf angewendet: Ein zulässiges Organisationsstandardmodell wird beibehalten, während eines außerhalb der Liste zum erzwungenen Standard aufgelöst wird.

Fable 5 ist auf keinem Kontotyp das Standardmodell. Sitzungen verwenden Fable 5 nur, nachdem Sie es auswählen, mit `/model fable`, einer `model`-Einstellung oder dem `best`-Alias, wo Fable 5 verfügbar ist. Wenn Sie es mit `/model` auswählen, wird es als das ausgewählte Modell in Ihren Benutzereinstellungen gespeichert, sodass spätere Sitzungen auf Fable 5 starten, bis Sie die Modelle ändern.

<h3 id="opusplan-model-setting">
  `opusplan`-Modelleinstellung
</h3>

Der `opusplan`-Modellalias bietet einen automatisierten Hybrid-Ansatz:

* **Im Plan-Modus**: verwendet `opus` für komplexes Reasoning und Architekturentscheidungen
* **Im Ausführungsmodus**: wechselt automatisch zu `sonnet` für Code-Generierung und Implementierung

Dies kombiniert Opus's Reasoning für die Planung mit Sonnets Effizienz für die Ausführung.

Das Kontextfenster der Plan-Modus-Opus-Phase ist das gleiche wie bei der `opus`-Modelleinstellung. Bei Abonnement-Tiers, bei denen Opus [automatisch auf 1M-Kontext aktualisiert wird](#extended-context), erhält `opusplan` die Aktualisierung auch im Plan-Modus. Um 1M-Kontext für beide Phasen zu erzwingen, wenn Sie sich nicht auf einem Auto-Upgrade-Tier befinden, setzen Sie das Modell auf `opusplan[1m]`.

Wenn [`availableModels`](#restrict-model-selection) das neueste Opus ausschließt, aber eine ältere Version erlaubt, z. B. `["sonnet", "claude-opus-4-6"]`, verwendet `opusplan` das neueste erlaubte Opus für die Planung und bleibt nur auf Sonnet, wenn jedes Opus ausgeschlossen ist. Eine Haiku-Sitzung, die normalerweise im Plan-Modus zu Sonnet aktualisiert würde, verwendet ebenfalls das neueste erlaubte Sonnet und bleibt nur auf Haiku, wenn jedes Sonnet ausgeschlossen ist. Vor v2.1.205 blieb der Plan-Modus auf dem Modell der Sitzung, wenn die neueste Version der Upgrade-Familie ausgeschlossen war, auch wenn die Zulassungsliste eine ältere erlaubte.

Die Substitution einer älteren erlaubten Version gilt auf der Anthropic API und [Claude Platform auf AWS](/docs/de/claude-platform-on-aws). Auf Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry und Mantle, deren Bereitstellungen anbieter-spezifische Modell-IDs verwenden, bleibt der Plan-Modus auf dem Modell der Sitzung, wenn das Upgrade-Modell ausgeschlossen ist.

Für einen Hybrid-Ansatz, bei dem Claude während einer Aufgabe entscheidet, wann ein zweites Modell konsultiert werden soll, anstatt an der Plan-Grenze zu wechseln, siehe das [Advisor-Tool](/docs/de/advisor).

<h3 id="fallback-model-chains">
  Fallback-Modellketten
</h3>

Wenn das primäre Modell überlastet ist, nicht verfügbar ist oder einen anderen nicht wiederholbaren Serverfehler zurückgibt, kann Claude Code zu einem Fallback-Modell wechseln, anstatt die Anfrage fehlschlagen zu lassen. Authentifizierungs-, Abrechnungs-, Rate-Limit-, Anfragegröße- und Transportfehler lösen niemals einen Wechsel aus; diese folgen ihrem normalen Wiederholungs- und Fehlerbehandlung.

Konfigurieren Sie ein oder mehrere Fallback-Modelle und Claude Code versucht sie der Reihe nach, wobei eine Benachrichtigung angezeigt wird, wenn es wechselt. Der Wechsel gilt nur für den aktuellen Durchgang, sodass Ihre nächste Nachricht zuerst wieder das primäre Modell versucht. Ketten sind auf drei Modelle nach Duplikatentfernung begrenzt, und zusätzliche Einträge werden ignoriert.

Setzen Sie eine Kette für eine Sitzung mit dem `--fallback-model`-Flag, das eine kommagetrennte Liste akzeptiert:

```bash theme={null}
claude --fallback-model sonnet,haiku
```

Um eine Kette über Sitzungen hinweg beizubehalten, setzen Sie `fallbackModel` in [Einstellungen](/docs/de/settings) als Array:

```json theme={null}
{
  "fallbackModel": ["claude-sonnet-5", "claude-haiku-4-5"]
}
```

Das `--fallback-model`-Flag hat Vorrang vor der `fallbackModel`-Einstellung. Jedes Element akzeptiert einen Modellnamen oder Alias, und `"default"` wird auf das Standardmodell erweitert.

Zwei Fälle führen dazu, dass ein Element übersprungen wird:

* **Nicht verfügbares Modell**: Ein Modell, das nicht erreichbar ist, z. B. ein in Einstellungen angeheftetes veraltetes Modell, wird übersprungen und Claude Code fährt mit dem nächsten Element fort.
* **Außerhalb der Zulassungsliste**: Ein Element, das nicht von [`availableModels`](#restrict-model-selection) erlaubt ist, wird gelöscht, wenn die Kette gelesen wird, und wird nie versucht.

<h3 id="automatic-model-fallback">
  Automatisches Modell-Fallback
</h3>

Dieser Abschnitt behandelt inhaltsbasiertes Fallback von Fable 5. Für verfügbarkeitsbasiertes Fallback, wenn ein Modell überlastet oder nicht verfügbar ist, siehe [Fallback-Modellketten](#fallback-model-chains).

Fable 5 wird mit Sicherheitsklassifizierern für Cybersicherheits- und Biologie-Inhalte ausgeführt. Wenn ein Klassifizierer eine Anfrage kennzeichnet, führt Claude Code diese Anfrage auf dem Standard-Opus-Modell Ihres Anbieters erneut aus und zeigt eine Benachrichtigung im Transkript an. Auf der Anthropic API, [LLM-Gateway](/docs/de/llm-gateway)-Bereitstellungen und [Claude Platform auf AWS](/docs/de/claude-platform-on-aws) ist dieses Modell Opus 4.8. Auf dem [Claude-Apps-Gateway](/docs/de/claude-apps-gateway) ist es Opus 4.7, es sei denn, Sie verweisen den [`opus`-Alias](#environment-variables) auf ein anderes Modell.

Die Sitzung wird dann auf diesem Opus-Modell fortgesetzt. Um zu Fable 5 zurückzukehren, führen Sie `/model fable` aus.

Das Fallback-Ziel wird gegen [`availableModels`](#restrict-model-selection) überprüft. Wenn es blockiert ist, findet kein Fallback statt. Die Ablehnung erscheint als normaler Fehler und das Modell der Sitzung bleibt unverändert.

<h4 id="check-what-triggered-fallback">
  Überprüfen Sie, was das Fallback ausgelöst hat
</h4>

Fallback kann bei der ersten Anfrage einer Sitzung ausgelöst werden, bevor Sie etwas Ungewöhnliches senden, da die erste Anfrage Workspace-Kontext wie Ihren CLAUDE.md-Inhalt und Git-Status trägt. Ein Repository, das Sicherheits- oder Biologie-Material enthält, kann den Klassifizierer allein auf diesem Kontext auslösen.

Um zu überprüfen, ob Anpassungen der Auslöser sind, starten Sie eine Sitzung mit `claude --safe-mode`, das Anpassungen wie CLAUDE.md, Skills, MCP-Server und Hooks deaktiviert. Git-Status und Verzeichnisnamen sind keine Anpassungen und sind immer noch enthalten.

<h4 id="ask-before-switching">
  Vor dem Wechsel fragen
</h4>

Um zu entscheiden, was jedes Mal passiert, wenn eine Anfrage gekennzeichnet wird, anstatt automatisch zu wechseln, führen Sie `/config` aus und schalten Sie „Modelle wechseln, wenn eine Nachricht gekennzeichnet wird" aus. Eine gekennzeichnete Anfrage pausiert dann die Sitzung mit zwei Optionen: zum Opus-Modell wechseln oder den Prompt bearbeiten und auf Fable 5 erneut versuchen.

Einige Fälle verhalten sich unterschiedlich:

* Wenn beide Modelle die gleiche Anfrage kennzeichnen, können Sie den Prompt bearbeiten und erneut versuchen oder eine neue Sitzung starten.
* Bei mobilen [Claude Code im Web](/docs/de/claude-code-on-the-web)-Sitzungen wird das Bearbeiten und erneute Versuchen nicht unterstützt. Wechseln Sie Modelle oder setzen Sie die Sitzung in einem Desktop-Browser oder der Desktop-App fort.
* Im [nicht-interaktiven Modus](/docs/de/cli-reference#cli-flags) und SDK-Integrationen, die den Prompt nicht anzeigen können, endet eine gekennzeichnete Anfrage den Durchgang mit einer Ablehnung.
* Wenn das Fallback-Ziel durch [`availableModels`](#restrict-model-selection) blockiert ist, wird der Prompt nicht angezeigt. Die gekennzeichnete Anfrage endet mit der Ablehnung, genauso wie automatisches Fallback, wenn das Ziel blockiert ist.

<h4 id="enable-fallback-on-bedrock-agent-platform-and-foundry">
  Aktivieren Sie Fallback auf Bedrock, Agent Platform und Foundry
</h4>

Auf [Amazon Bedrock](/docs/de/amazon-bedrock), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai) und [Microsoft Foundry](/docs/de/microsoft-foundry) sind Modell-IDs anbieter-spezifisch, daher funktioniert automatisches Fallback nur, wenn Claude Code beide beteiligten Modelle identifizieren kann:

* Claude Code muss das aktuelle Modell als Fable 5 erkennen: Die Modell-ID enthält `claude-fable-5`, stimmt mit dem Wert von `ANTHROPIC_DEFAULT_FABLE_MODEL` überein oder ist mit [`modelOverrides`](#override-model-ids-per-version) zugeordnet.
* Das Fallback-Ziel muss zu einem Opus-Modell aufgelöst werden: Der Wert von `ANTHROPIC_DEFAULT_OPUS_MODEL`, falls gesetzt, andernfalls ein Opus 4.8-Eintrag in der Modellliste des Anbieters.

Wenn eines der Modelle nicht identifiziert werden kann, wechselt Claude Code nicht automatisch. Die gekennzeichnete Anfrage endet mit einer Ablehnungsmeldung, und Sie können Modelle mit [`/model`](#setting-your-model) wechseln und erneut versuchen. Um automatisches Fallback auf diesen Anbietern zu aktivieren, setzen Sie `ANTHROPIC_DEFAULT_FABLE_MODEL` auf Ihre Fable 5-Modell-ID und `ANTHROPIC_DEFAULT_OPUS_MODEL` auf Ihre Opus 4.8-Modell-ID.

<h4 id="security-research-and-biology-workloads">
  Sicherheitsforschungs- und Biologie-Workloads
</h4>

Workloads in offensiver Sicherheit oder Biologie, einschließlich Penetrationstests, Capture the Flag (CTF)-Übungen und biologie-nahe Codebases, lösen häufig Fallback aus, oft bei der ersten Anfrage. Für substantielle Biologie-Arbeit erwarten Sie, dass fast alle Anfragen umgeleitet werden.

Dies ist erwartetes Routing für diese Domänen, kein Konto-Flag. Wenn Ihre Organisation Fable-Klasse-Fähigkeit für diese Arbeit benötigt, fragen Sie Ihr Anthropic-Kontoteam nach vertrauenswürdigen Zugriffsprogrammen.

<h3 id="adjust-effort-level">
  Anpassung des Aufwandsniveaus
</h3>

[Aufwandsniveaus](https://platform.claude.com/docs/en/build-with-claude/effort) steuern adaptives Reasoning, das das Modell entscheiden lässt, ob und wie viel es bei jedem Schritt basierend auf der Aufgabenkomplexität denken soll. Niedrigerer Aufwand ist schneller und günstiger für unkomplizierte Aufgaben, während höherer Aufwand tieferes Reasoning für komplexe Probleme bietet.

Die verfügbaren Aufwandsniveaus hängen vom Modell ab. Modelle, die hier nicht aufgeführt sind, unterstützen keinen Aufwand:

| Modell                          | Ebenen                                  |
| :------------------------------ | :-------------------------------------- |
| Fable 5                         | `low`, `medium`, `high`, `xhigh`, `max` |
| Sonnet 5, Opus 4.8 und Opus 4.7 | `low`, `medium`, `high`, `xhigh`, `max` |
| Opus 4.6 und Sonnet 4.6         | `low`, `medium`, `high`, `max`          |

Wenn Sie eine Ebene setzen, die das aktive Modell nicht unterstützt, greift Claude Code auf die höchste unterstützte Ebene bei oder unter der von Ihnen gesetzten zurück. Zum Beispiel wird `xhigh` auf Opus 4.6 als `high` ausgeführt. Ihre Organisation kann auch begrenzen, welche Ebenen für ein Modell verfügbar sind; siehe [Organisationsaufwandsgrenzen](#organization-effort-limits).

Der Standard-Aufwand ist `high` auf Fable 5, Sonnet 5, Opus 4.8, Opus 4.6 und Sonnet 4.6 und `xhigh` auf Opus 4.7.

Wenn Sie Fable 5, Opus 4.8 oder Opus 4.7 zum ersten Mal ausführen, wendet Claude Code das Standard-Aufwandsniveau dieses Modells an, auch wenn Sie zuvor ein anderes Niveau für ein anderes Modell gesetzt haben: `high` auf Fable 5 und Opus 4.8 und `xhigh` auf Opus 4.7. Führen Sie `/effort` erneut aus, um nach dem Wechsel ein anderes Niveau zu wählen. Dieser Standard wird über Sitzungen hinweg beibehalten, bis Sie eine explizite Aufwandsauswahl treffen, z. B. durch Ausführen von `/effort` in einer interaktiven Sitzung oder Starten mit `--effort`.

`low`, `medium`, `high` und `xhigh` bleiben über Sitzungen hinweg erhalten, wenn Sie sie in einer interaktiven Sitzung setzen. Ein Niveau, das mit `/effort` im [nicht-interaktiven Modus](/docs/de/headless) mit dem `-p`-Flag gesetzt wird, gilt nur für die aktuelle Sitzung und wird nicht als Ihr Standard gespeichert. Ein nicht-interaktives `/effort` kann auch die Modell-Standard-Sperre oben nicht freigeben: Bei Fable 5, Opus 4.8 und Opus 4.7 meldet es `Not applied` und die Sitzung bleibt beim Standard-Aufwand des Modells, daher übergeben Sie stattdessen `--effort` beim Start. `max` bietet das tiefste Reasoning ohne Einschränkung bei der Token-Ausgabe und gilt nur für die aktuelle Sitzung, außer wenn es über die `CLAUDE_CODE_EFFORT_LEVEL`-Umgebungsvariable gesetzt wird.

Das `/effort`-Menü bietet auch `ultracode`. Ultracode ist eine Claude Code-Einstellung und keine Modell-Aufwandsebene: Es sendet `xhigh` an das Modell und lässt Claude zusätzlich [dynamische Workflows](/docs/de/workflows) für substantielle Aufgaben orchestrieren. Es gilt nur für die aktuelle Sitzung.

Sie können Ultracode durch eine der folgenden Methoden aktivieren:

* **`/effort`**: Führen Sie `/effort ultracode` aus oder wählen Sie es aus dem Menü
* **`--effort`-Flag**: Starten Sie mit `claude --effort ultracode`, das die Sitzung mit `xhigh`-Aufwand und aktiviertem Ultracode startet
* **`--settings` oder eine Agent SDK-Steueranforderung**: Übergeben Sie `"ultracode": true`. Eine [`applyFlagSettings()`](/docs/de/agent-sdk/typescript#applyflagsettings)-Anforderung akzeptiert auch `effortLevel: "ultracode"`

Das Übergeben von `ultracode` an das `--effort`-Flag oder den Agent SDK `effortLevel`-Wert erfordert Claude Code v2.1.203 oder später. Vor v2.1.203 gab `--effort ultracode` `Unknown --effort value 'ultracode'` aus und die Sitzung startete mit dem Standard-Aufwand.

Die persistierte `effortLevel`-Einstellung und die `CLAUDE_CODE_EFFORT_LEVEL`-Umgebungsvariable akzeptieren nicht `ultracode`.

Wenn Ultracode nicht verfügbar ist, z. B. wenn [Workflows ausgeschaltet sind](/docs/de/workflows#turn-workflows-off), setzt `--effort ultracode` nur `xhigh`-Aufwand.

<h4 id="choose-an-effort-level">
  Wählen Sie ein Aufwandsniveau
</h4>

Jede Ebene tauscht Token-Ausgabe gegen Fähigkeit. Der Standard eignet sich für die meisten Codierungsaufgaben; passen Sie an, wenn Sie ein anderes Gleichgewicht wünschen.

| Ebene       | Wann man es verwendet                                                                                                                                                     |
| :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `low`       | Reservieren Sie für kurze, begrenzte, latenzempfindliche Aufgaben, die nicht intelligenzempfindlich sind                                                                  |
| `medium`    | Reduziert Token-Nutzung für kostensensitive Arbeiten, die etwas Intelligenz opfern können                                                                                 |
| `high`      | Balanciert Token-Nutzung und Intelligenz. Standard auf Fable 5, Sonnet 5, Opus 4.8, Opus 4.6 und Sonnet 4.6                                                               |
| `xhigh`     | Tieferes Reasoning bei höherer Token-Ausgabe. Standard auf Opus 4.7                                                                                                       |
| `max`       | Kann die Leistung bei anspruchsvollen Aufgaben verbessern, kann aber abnehmende Erträge zeigen und ist anfällig für Überdenken. Testen Sie vor breiter Übernahme          |
| `ultracode` | Eine Claude Code-Einstellung, die einen [dynamischen Workflow](/docs/de/workflows) für jede substantielle Aufgabe mit `xhigh`-Pro-Nachricht-Reasoning plant. Nur für Sitzungen |

Die Aufwandsskala ist pro Modell kalibriert, daher stellt der gleiche Ebenenname nicht den gleichen zugrunde liegenden Wert über Modelle hinweg dar.

<h4 id="use-ultrathink-for-one-off-deep-reasoning">
  Verwenden Sie ultrathink für einmaliges tiefes Reasoning
</h4>

Fügen Sie `ultrathink` überall in Ihrem Prompt ein, um tieferes Reasoning bei diesem Durchgang anzufordern, ohne Ihre Sitzungsaufwandseinstellung zu ändern. Claude Code erkennt das Schlüsselwort und fügt eine In-Context-Anweisung hinzu. Das an die API gesendete Aufwandsniveau bleibt unverändert. Andere Phrasen wie „think", „think hard" und „think more" werden als gewöhnlicher Prompt-Text weitergeleitet und werden nicht als Schlüsselwörter erkannt.

<h4 id="set-the-effort-level">
  Setzen Sie das Aufwandsniveau
</h4>

Sie können Aufwand durch eine der folgenden Methoden ändern:

* **`/effort`**: Führen Sie `/effort` ohne Argumente aus, um einen interaktiven Schieberegler zu öffnen, `/effort` gefolgt von einem Ebenennamen, um ihn direkt zu setzen, oder `/effort auto`, um auf den Modellstandard zurückzusetzen
* **In `/model`**: Verwenden Sie die Pfeiltasten nach links/rechts, um den Aufwand-Schieberegler anzupassen, wenn Sie ein Modell auswählen
* **`--effort`-Flag**: Übergeben Sie einen Ebenennamen, um ihn für eine einzelne Sitzung beim Starten von Claude Code zu setzen
* **Umgebungsvariable**: Setzen Sie `CLAUDE_CODE_EFFORT_LEVEL` auf einen Ebenennamen oder `auto`
* **Einstellungen**: Setzen Sie `effortLevel` auf `low`, `medium`, `high` oder `xhigh` in Ihrer Einstellungsdatei. `max` und `ultracode` sind [nur für Sitzungen](#adjust-effort-level) und werden hier nicht akzeptiert
* **Skill- und Subagent-Frontmatter**: Setzen Sie `effort` in einer [Skill](/docs/de/skills#frontmatter-reference)- oder [Subagent](/docs/de/sub-agents#supported-frontmatter-fields)-Markdown-Datei, um das Aufwandsniveau zu überschreiben, wenn dieser Skill oder Subagent ausgeführt wird

Die Umgebungsvariable hat Vorrang vor allen anderen Methoden, dann Ihre konfigurierte Ebene, dann der Modellstandard. Frontmatter-Aufwand gilt, wenn dieser Skill oder Subagent aktiv ist, und überschreibt die Sitzungsebene, aber nicht die Umgebungsvariable.

Der Aufwand-Schieberegler erscheint in `/model`, wenn ein unterstütztes Modell ausgewählt ist. Das aktuelle Aufwandsniveau wird auch neben dem Logo und dem Spinner angezeigt, z. B. „with low effort", damit Sie bestätigen können, welche Einstellung aktiv ist, ohne `/model` zu öffnen.

<h4 id="adaptive-reasoning-and-fixed-thinking-budgets">
  Adaptives Reasoning und feste Thinking-Budgets
</h4>

Adaptives Reasoning macht Thinking bei jedem Schritt optional, daher kann Claude schneller auf Routine-Prompts reagieren und tieferes Denken für Schritte reservieren, die davon profitieren. Wenn Sie möchten, dass Claude häufiger oder seltener denkt als das aktuelle Niveau erzeugt, können Sie dies direkt in Ihrem Prompt oder in `CLAUDE.md` sagen; das Modell reagiert auf diese Anleitung innerhalb seiner Aufwandseinstellung.

Fable 5, Sonnet 5 sowie Opus 4.7 und später verwenden immer adaptives Reasoning. Der Modus mit festem Thinking-Budget und `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING` gelten nicht dafür.

Bei Opus 4.6 und Sonnet 4.6 können Sie `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` setzen, um zum vorherigen festen Thinking-Budget zurückzukehren, das von `MAX_THINKING_TOKENS` gesteuert wird. Siehe [Umgebungsvariablen](/docs/de/env-vars).

<h3 id="extended-thinking">
  Erweitertes Thinking
</h3>

Erweitertes Thinking ist das Reasoning, das Claude vor der Antwort ausgibt. Bei Modellen, die [adaptives Reasoning](#adjust-effort-level) unterstützen, ist das Aufwandsniveau die primäre Kontrolle dafür, wie viel Thinking stattfindet; die folgenden Einstellungen schalten Thinking ein oder aus und steuern, wie es angezeigt wird.

| Kontrolle                               | Wie man es setzt                                                                                                                                                                                                                                                                                                                                                                                             |
| :-------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Umschalter für die aktuelle Sitzung     | Drücken Sie `Option+T` auf macOS oder `Alt+T` auf Windows und Linux                                                                                                                                                                                                                                                                                                                                          |
| Setzen Sie den globalen Standard        | Führen Sie `/config` aus und schalten Sie den Thinking-Modus um. Gespeichert als `alwaysThinkingEnabled` in `~/.claude/settings.json`                                                                                                                                                                                                                                                                        |
| Deaktivieren Sie unabhängig vom Aufwand | Setzen Sie [`MAX_THINKING_TOKENS=0`](/docs/de/env-vars), das Thinking auf der Anthropic API außer auf Fable 5 ausschaltet. Auf [Drittanbieter-Anbietern](/docs/de/third-party-integrations) wird stattdessen der `thinking`-Parameter weggelassen, und adaptive-Reasoning-Modelle können immer noch denken. Andere Werte gelten nur mit einem [festen Thinking-Budget](#adaptive-reasoning-and-fixed-thinking-budgets) |

Thinking kann auf Fable 5 nicht ausgeschaltet werden. Der Sitzungsumschalter, `alwaysThinkingEnabled` und `MAX_THINKING_TOKENS=0` haben dort keine Auswirkung, und Fable 5 entscheidet pro Schritt, wie viel zu denken ist, basierend auf dem Aufwandsniveau.

Die Thinking-Ausgabe ist standardmäßig zusammengeklappt. Drücken Sie `Ctrl+O`, um den ausführlichen Modus umzuschalten und das Reasoning als grauer kursiver Text zu sehen. Interaktive Sitzungen auf der Anthropic API erhalten standardmäßig redigierte Thinking-Blöcke, daher setzen Sie `showThinkingSummaries: true` in [Einstellungen](/docs/de/settings), wenn Sie die vollständigen Zusammenfassungen verfügbar haben möchten, wenn Sie erweitern. Ihnen werden alle generierten Thinking-Token berechnet, auch wenn sie zusammengeklappt oder redigiert sind.

<h3 id="extended-context">
  Erweiterter Kontext
</h3>

Fable 5, Sonnet 5, Opus 4.6 und später sowie Sonnet 4.6 unterstützen ein [1-Million-Token-Kontextfenster](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) für lange Sitzungen mit großen Codebases.

Die Verfügbarkeit variiert je nach Modell und Plan. Auf der Anthropic API werden Fable 5, Sonnet 5, Opus 4.8 und Opus 4.7 immer mit dem 1M-Fenster ausgeführt. Bei Max-, Team- und Enterprise-Plänen wird Opus automatisch auf 1M-Kontext ohne zusätzliche Konfiguration aktualisiert. Dies gilt für beide Team Standard- und Team Premium-Plätze. Sonnet 4.6 mit 1M-Kontext ist nicht Teil der automatischen Aktualisierung und erfordert [Nutzungsguthaben](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) bei jedem Abonnementplan, einschließlich Max.

| Plan                     | Opus mit 1M-Kontext                                                                                             | Sonnet 4.6 mit 1M-Kontext                                                                                       |
| ------------------------ | --------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| Max, Team und Enterprise | Im Abonnement enthalten                                                                                         | Erfordert [Nutzungsguthaben](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) |
| Pro                      | Erfordert [Nutzungsguthaben](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) | Erfordert [Nutzungsguthaben](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) |
| API und Pay-as-you-go    | Vollständiger Zugriff                                                                                           | Vollständiger Zugriff                                                                                           |

Um 1M-Kontext vollständig zu deaktivieren, setzen Sie `CLAUDE_CODE_DISABLE_1M_CONTEXT=1`. Dies entfernt 1M-Modellvarianten aus der Modellauswahl. Siehe [Umgebungsvariablen](/docs/de/env-vars).

Das 1M-Kontextfenster verwendet Standard-Modellpreise ohne Aufschlag für Token über 200K. Für Pläne, bei denen erweiterter Kontext in Ihrem Abonnement enthalten ist, bleibt die Nutzung von Ihrem Abonnement abgedeckt. Für Pläne, die über Nutzungsguthaben auf erweiterten Kontext zugreifen, werden Token zu Nutzungsguthaben abgerechnet.

Wenn Ihr Konto 1M-Kontext unterstützt, erscheint die Option in der Modellauswahl (`/model`) in den neuesten Versionen von Claude Code. Wenn Sie sie nicht sehen, versuchen Sie, Ihre Sitzung neu zu starten.

Sie können auch das `[1m]`-Suffix mit Modellaliasen oder vollständigen Modellnamen verwenden:

```bash theme={null}
# Verwenden Sie den opus[1m]- oder sonnet[1m]-Alias
/model opus[1m]
/model sonnet[1m]

# Oder fügen Sie [1m] an einen vollständigen Modellnamen an
/model claude-opus-4-8[1m]
```

<h4 id="sonnet-5-context-window">
  Sonnet 5-Kontextfenster
</h4>

Auf der Anthropic API wird Sonnet 5 immer mit dem 1M-Kontextfenster ausgeführt. Es gibt keine 200K-Variante, kein `[1m]`-Suffix zur Auswahl und keine erforderlichen Nutzungsguthaben bei irgendeinem Plan. Sitzungen werden automatisch komprimiert, bevor das Fenster voll ist, standardmäßig bei etwa 967K Token; setzen Sie [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/de/env-vars), um einen anderen Schwellenwert zu wählen.

Zwei Konfigurationen budgetieren das Fenster stattdessen auf 200K und komprimieren automatisch an dieser Grenze:

* **LLM-Gateway**: Wenn `ANTHROPIC_BASE_URL` auf ein [Gateway](/docs/de/llm-gateway) verweist, kann Claude Code die 1M-Unterstützung nicht überprüfen. Um das vollständige Fenster zu verwenden, wählen Sie Sonnet 5 (1M context) in der Modellauswahl aus, was `sonnet[1m]` zugeordnet wird.
* **`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`**: Behandelt Sonnet 5-Sitzungen so, als hätten sie ein 200K-Fenster, für Bereitstellungen, die den Kontext begrenzen müssen.

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

Dieses Beispiel setzt alle drei Variablen, um eine Gateway-gesteuerte Opus-Bereitstellung auswählbar zu machen:

```bash theme={null}
export ANTHROPIC_CUSTOM_MODEL_OPTION="my-gateway/claude-opus-4-8"
export ANTHROPIC_CUSTOM_MODEL_OPTION_NAME="Opus via Gateway"
export ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION="Custom deployment routed through the internal LLM gateway"
```

Der benutzerdefinierte Eintrag erscheint am unteren Ende der `/model`-Auswahl. `ANTHROPIC_CUSTOM_MODEL_OPTION_NAME` und `ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION` sind optional. Wenn weggelassen, wird die Modell-ID als Name verwendet und die Beschreibung wird standardmäßig auf `Custom model (<model-id>)` gesetzt.

Claude Code überspringt die Validierung für die Modell-ID, die in `ANTHROPIC_CUSTOM_MODEL_OPTION` gesetzt ist, daher können Sie jeden String verwenden, den Ihr API-Endpunkt akzeptiert. Wenn [`availableModels`](#restrict-model-selection) gesetzt ist, fügen Sie die benutzerdefinierte Modell-ID auch in die Zulassungsliste ein: Der benutzerdefinierte Eintrag wird aus der Auswahl gefiltert und eine `--model`-Auswahl wird wie jedes andere ausgeschlossene Modell abgelehnt. Eine benutzerdefinierte ID, die einen Familiennamen einbettet, wie z. B. `my-gateway/claude-opus-4-8`, zählt als spezifischer Eintrag für diese Familie und deaktiviert deren Wildcard. Daher müssen Sie auch die Versionen auflisten, die Sie auswählbar halten möchten. Siehe [Zusammenführungsverhalten](#merge-behavior).

<h2 id="environment-variables">
  Umgebungsvariablen
</h2>

Sie können die folgenden Umgebungsvariablen verwenden, um die Modellnamen zu steuern, auf die die Aliase verweisen. Jeder Wert muss ein vollständiger Modellname oder das entsprechende Äquivalent für Ihren API-Anbieter sein.

| Umgebungsvariable                | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_DEFAULT_FABLE_MODEL`  | Das Modell, das für `fable` verwendet werden soll, und die Modell-ID, die Claude Code als Fable 5 für [automatisches Modell-Fallback](#automatic-model-fallback) bei Drittanbieter-Anbietern erkennt                                                                                                                                                                                                                                                    |
| `ANTHROPIC_DEFAULT_OPUS_MODEL`   | Das Modell, das für `opus` verwendet werden soll, oder für `opusplan`, wenn Plan Mode aktiv ist.                                                                                                                                                                                                                                                                                                                                                        |
| `ANTHROPIC_DEFAULT_SONNET_MODEL` | Das Modell, das für `sonnet` verwendet werden soll, oder für `opusplan`, wenn Plan Mode nicht aktiv ist.                                                                                                                                                                                                                                                                                                                                                |
| `ANTHROPIC_DEFAULT_HAIKU_MODEL`  | Das Modell, das für `haiku` verwendet werden soll, oder [Hintergrundfunktionalität](/docs/de/costs#background-token-usage)                                                                                                                                                                                                                                                                                                                                   |
| `CLAUDE_CODE_SUBAGENT_MODEL`     | Das Modell, das für alle [Subagents](/docs/de/sub-agents#choose-a-model), [Agent-Teams](/docs/de/agent-teams) und die Agenten verwendet werden soll, die ein [Workflow](/docs/de/workflows) ausführt. Akzeptiert einen Alias wie `haiku` oder einen vollständigen Modellnamen und überschreibt den `model`-Parameter pro Aufruf und die `model`-Frontmatter der Subagent-Definition. Setzen Sie auf `inherit`, um stattdessen die normale Modellauflösung zu verwenden |

Hinweis: `ANTHROPIC_SMALL_FAST_MODEL` ist veraltet zugunsten von `ANTHROPIC_DEFAULT_HAIKU_MODEL`.

<h3 id="pin-models-for-third-party-deployments">
  Modelle für Drittanbieter-Bereitstellungen fixieren
</h3>

Beim Bereitstellen von Claude Code über [Amazon Bedrock](/docs/de/amazon-bedrock), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai), [Microsoft Foundry](/docs/de/microsoft-foundry) oder [Claude Platform on AWS](/docs/de/claude-platform-on-aws) sollten Sie Modellversionen vor dem Rollout für Benutzer fixieren.

Ohne Fixierung verwendet Claude Code Modellaliase wie `fable`, `opus`, `sonnet` und `haiku`, die zur integrierten Standard-Modell-ID für jeden Anbieter aufgelöst werden. Dieser Standard kann hinter der neuesten Anthropic-Veröffentlichung zurückbleiben, und das Modell, auf das er verweist, ist möglicherweise noch nicht in einem Benutzerkonto aktiviert. Wenn der Standard nicht verfügbar ist, sehen Amazon Bedrock- und Google Cloud's Agent Platform-Benutzer einen Hinweis und die Sitzung greift auf eine frühere Version des Standard-Modells zurück, oder auf das Standard-Sonnet-Modell, wenn der Standard ein Opus-Modell ist und keine Opus-Version verfügbar ist. Microsoft Foundry-Benutzer sehen stattdessen Fehler, da Microsoft Foundry keine entsprechende Startprüfung hat.

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

Um [erweiterten Kontext](#extended-context) für ein fixiertes Modell zu aktivieren, fügen Sie `[1m]` an die Modell-ID in `ANTHROPIC_DEFAULT_OPUS_MODEL` oder `ANTHROPIC_DEFAULT_SONNET_MODEL` an:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8[1m]'
```

Das `[1m]`-Suffix wendet das 1M-Kontextfenster auf alle Verwendungen der Aliase `opus` und `sonnet` an, einschließlich der Plan-Mode-Opus-Phase von [`opusplan`](#opusplan-model-setting).

* Claude Code entfernt das Suffix, bevor die Modell-ID an Ihren Anbieter gesendet wird.
* Fügen Sie `[1m]` nur an, wenn das zugrunde liegende Modell [1M-Kontext unterstützt](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model).
* Das Suffix wird pro Variable gelesen, nicht pro Modell. Bei Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry verwendet eine Modell-ID ohne `[1m]` in einer Variable 200K-Kontext, auch wenn eine andere Variable das gleiche Modell mit dem Suffix setzt. Sonnet 5 wird auf diesen Anbietern immer mit dem 1M-Fenster ausgeführt und benötigt niemals das Suffix.

<Note>
  Eine `availableModels`-Zulassungsliste, die über [MDM oder eine verwaltete Einstellungsdatei](/docs/de/settings#settings-files) bereitgestellt wird, gilt weiterhin bei Verwendung von Drittanbieter-Anbietern; [server-verwaltete Einstellungen werden dort nicht bereitgestellt](/docs/de/server-managed-settings#platform-availability). Die Filterung stimmt mit einem Modellalias wie `opus`, einem Versionspräfix wie `claude-opus-4-8` oder der vollständigen Modell-ID in Anbieterform überein. Anbieterspezifische Präfixe wie `us.anthropic.` werden nicht entfernt, daher müssen Sie zum Zulassen eines bestimmten Modells die gleiche Anbieterform-ID auflisten, die die Auswahl anzeigt, oder sie durch [`modelOverrides`](#override-model-ids-per-version) zuordnen. Alle `[1m]`-Suffixe werden sowohl aus dem Zulassungslisten-Eintrag als auch aus dem angeforderten Modell vor dem Abgleich entfernt.
</Note>

<h3 id="customize-pinned-model-display-and-capabilities">
  Anzeige und Funktionen des fixierten Modells anpassen
</h3>

Wenn Sie ein Modell bei einem Drittanbieter fixieren, erscheint die anbieterspezifische ID in der `/model`-Auswahl und Claude Code erkennt möglicherweise nicht, welche Funktionen das Modell unterstützt. Sie können den Anzeigenamen und die deklarierten Funktionen mit Begleit-Umgebungsvariablen für jedes fixierte Modell überschreiben.

Diese Variablen wirken sich auf Drittanbieter wie Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry aus. Die Variablen `_NAME` und `_DESCRIPTION` wirken sich auch aus, wenn `ANTHROPIC_BASE_URL` auf ein [LLM-Gateway](/docs/de/llm-gateway) verweist. Sie haben keine Auswirkung bei direkter Verbindung zu `api.anthropic.com`.

| Umgebungsvariable                                     | Beschreibung                                                                                                                     |
| ----------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_NAME`                   | Anzeigename für das fixierte Opus-Modell in der `/model`-Auswahl. Standardmäßig die Modell-ID, wenn nicht gesetzt                |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_DESCRIPTION`            | Anzeige-Beschreibung für das fixierte Opus-Modell in der `/model`-Auswahl. Standardmäßig `Custom Opus model`, wenn nicht gesetzt |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES` | Komma-getrennte Liste der Funktionen, die das fixierte Opus-Modell unterstützt                                                   |

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

Die oben genannten Umgebungsvariablen auf Familienebene konfigurieren eine Modell-ID pro Familienalias. Wenn Sie mehrere Versionen innerhalb der gleichen Familie auf unterschiedliche Anbieter-IDs abbilden müssen, verwenden Sie stattdessen die `modelOverrides`-Einstellung.

`modelOverrides` ordnet einzelne Anthropic-Modell-IDs den anbieterspezifischen Strings zu, die Claude Code an die API Ihres Anbieters sendet. Wenn ein Benutzer ein zugeordnetes Modell in der `/model`-Auswahl auswählt, verwendet Claude Code Ihren konfigurierten Wert anstelle des integrierten Standards.

Dies ermöglicht es Enterprise-Administratoren, jede Modellversion zu einem bestimmten Amazon Bedrock-Inference-Profil-ARN, Google Cloud's Agent Platform-Versionsnamen oder Microsoft Foundry-Bereitstellungsnamen für Governance, Kostenzuteilung oder regionales Routing zu leiten.

Setzen Sie `modelOverrides` in Ihrer [Einstellungsdatei](/docs/de/settings#settings-files):

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

Überschreibungen ersetzen die integrierten Modell-IDs, die jeden Eintrag in der `/model`-Auswahl unterstützen. Bei Amazon Bedrock haben Überschreibungen Vorrang vor allen Inference-Profilen, die Claude Code beim Start automatisch erkennt. Claude Code übergibt Werte, die bereits anbieterspezifisch sind, wie Amazon Bedrock-Inference-Profil-ARNs oder Microsoft Foundry-Bereitstellungsnamen, unverändert an den Anbieter.

Überschreibungen gelten auch, wenn Sie eine Anthropic-Modell-ID direkt über `--model`, die `ANTHROPIC_MODEL`-Umgebungsvariable oder eine `ANTHROPIC_DEFAULT_*_MODEL`-Umgebungsvariable übergeben. Bei Amazon Bedrock, Google Cloud's Agent Platform und [Mantle](/docs/de/amazon-bedrock#use-the-mantle-endpoint) wird eine Anthropic-Modell-ID ohne `modelOverrides`-Eintrag zur gleichen anbieterspezifischen ID aufgelöst wie die `/model`-Auswahl-Zeile für diese Version, wenn der Anbieter diese Version unterstützt. Mantle unterstützt eine Teilmenge von Versionen. Für eine Anthropic-Modell-ID außerhalb dieser Teilmenge sendet Claude Code die rohe ID an Mantle ohne Zuordnung, es sei denn, ein `modelOverrides`-Eintrag deckt sie ab. Vor v2.1.200 erreichten `--model` und die Umgebungsvariablenwerte den Anbieter unverändert, ohne die Überschreibungskarte zu durchlaufen.

`modelOverrides` funktioniert zusammen mit `availableModels`. Die Zulassungsliste wird gegen die Anthropic-Modell-ID ausgewertet, nicht gegen den Überschreibungswert, daher ein Eintrag wie `"opus"` in `availableModels` stimmt weiterhin überein, auch wenn Opus-Versionen ARNs zugeordnet sind. Wenn `enforceAvailableModels` in verwalteten Einstellungen gesetzt ist, wird der erzwungene Standard durch `modelOverrides` aus der [höchsten Prioritätsquelle für verwaltete Einstellungen](/docs/de/server-managed-settings#settings-precedence) aufgelöst. Die Zuordnung eines Administrators, wie eine Version, die an ein Inference-Profil-ARN fixiert ist, wird im erzwungenen Standard berücksichtigt. Überschreibungen aus Benutzer- oder Projekteinstellungen beeinflussen ihn nicht.

Wenn `availableModels` in [verwalteten Einstellungen](/docs/de/settings#settings-files) gesetzt ist, gelten nur `modelOverrides` aus dieser verwalteten Quelle für eine Anthropic-Modell-ID, die direkt über `--model` oder die oben genannten Umgebungsvariablen übergeben wird. Claude Code ignoriert Überschreibungen in Benutzer- oder Projekteinstellungen für diese IDs und löst niemals eine ID auf, die die verwaltete Liste ausschließt, durch `modelOverrides` aus einer beliebigen Einstellungsquelle. Diese Einschränkung der verwalteten Quelle erfordert Claude Code v2.1.200 oder später. Siehe [Modellauswahl einschränken](#restrict-model-selection), um zu erfahren, wie blockierte IDs behandelt werden.

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

Um die Cache-TTL zu ändern oder zu erfahren, was einen Cache-Miss auslöst, siehe [Wie Claude Code Prompt-Caching verwendet](/docs/de/prompt-caching).
