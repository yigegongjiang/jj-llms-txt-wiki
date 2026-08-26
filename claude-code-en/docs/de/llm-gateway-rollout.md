> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Stellen Sie ein LLM-Gateway für Ihre Organisation bereit

> Stellen Sie ein Gateway-Produkt für Claude Code bereit: Konfigurieren Sie es so, dass es das weiterleitet, was Claude Code sendet, geben Sie Entwickleranmeldedaten aus, verteilen Sie die Konfiguration über verwaltete Einstellungen, und überprüfen Sie den Rollout.

Diese Seite führt einen Administrator durch die Bereitstellung eines LLM-Gateways für Claude Code. Sie setzt voraus, dass Sie ein Gateway-Produkt bereitgestellt haben, das die [Gateway-Anforderungen](#gateway-requirements) erfüllt. Die Bereitstellung oder der Betrieb eines bestimmten Produkts wird hier nicht behandelt; stellen Sie Ihres gemäß der Dokumentation des Herstellers bereit.

<Note>
  * Um Claude Code auf Ihrem eigenen Computer mit einem vorhandenen Gateway zu verbinden, siehe [Claude Code mit einem LLM-Gateway verbinden](/docs/de/llm-gateway-connect)
  * Für das, was Claude Code an ein Gateway sendet und was weitergeleitet werden soll, siehe die [Gateway-Protokollreferenz](/docs/de/llm-gateway-protocol)
</Note>

<h2 id="prerequisites">
  Voraussetzungen
</h2>

Um den Rollout abzuschließen, benötigen Sie:

* Ein Gateway, das auf Ihrer Infrastruktur bereitgestellt ist, HTTPS auf der genauen Adresse bereitstellt, die Sie an Entwickler verteilen werden, nicht auf einer Adresse, die zu ihr umleitet, und so konfiguriert ist, dass Claude-Modellnamen zu Ihrem Anbieter weitergeleitet werden
* Eine Anbieteranmeldedaten für das Gateway zum Weiterleiten mit:
  * Für die Anthropic API: einen API-Schlüssel aus der [Claude-Konsole](https://platform.claude.com/settings/keys)
  * Für einen Cloud-Anbieter: Cloud-Anmeldedaten mit Modellzugriff. Siehe die Voraussetzungen auf der Seite [Amazon Bedrock](/docs/de/amazon-bedrock#prerequisites), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai#prerequisites) oder [Microsoft Foundry](/docs/de/microsoft-foundry#prerequisites)
* Eine Möglichkeit, Einstellungsdateien auf Entwicklermaschinen bereitzustellen, z. B. MDM oder Konfigurationsverwaltung
  * Wenn Sie noch keine haben, vergleicht [wie Einstellungen Geräte erreichen](/docs/de/admin-setup#decide-how-settings-reach-devices) die Optionen

<h3 id="gateway-requirements">
  Gateway-Anforderungen
</h3>

Welches Produkt auch immer das Gateway bereitstellt, es muss:

* **Ein unterstütztes API-Format akzeptieren**: eines der Formate in der [API-Formattabelle](/docs/de/llm-gateway-protocol#api-formats). Die Rollout-Schritte unten setzen die Anthropic Messages API unter `POST /v1/messages` voraus, die die meisten Gateways bereitstellen
* **Antworten streamen**: Server-Sent-Events durchleiten, während sie ankommen, anstatt die gesamte Antwort zu puffern
* **Claude-Modellnamen weiterleiten**: jeden Namen, den Entwickler verwenden, einem Upstream-Modell zuordnen. Claude Code sendet einen Modellnamen wie `claude-sonnet-4-6` in jeder Anfrage; in den meisten Gateway-Produkten ist die Zuordnung eine Modellliste oder Routing-Tabelle in der eigenen Konfiguration des Gateways
* **Header und Body unverändert weiterleiten**: `anthropic-beta`, `anthropic-version` und den Request-Body in beide Richtungen durchleiten; die [Feature-Pass-Through-Tabelle](/docs/de/llm-gateway-protocol#feature-pass-through) ordnet jede dem Feature zu, das ohne sie bricht
* **Upstream-Fehler unverändert zurückgeben**: Claude Codes automatische Wiederherstellung basiert auf Fehlerformulierungen, daher bricht das Einwickeln von Fehlern in die eigene Hülle des Gateways es
* **Den Pfad von der WAF-Inspektion des Request-Body ausnehmen**: Claude Code-Prompts enthalten Quellcode und XML-ähnliche Tags, die Cross-Site-Scripting-Body-Regeln entsprechen; eine WAF vor dem Gateway gibt `403` bei echten Sitzungen zurück, während kurze Test-Anfragen durchgehen

Optional können Sie `GET /v1/models` bereitstellen, damit Claude Code die Modellauswahl von Ihrem Gateway mit [Modellermittlung](/docs/de/llm-gateway-protocol#model-discovery) füllen kann.&#x20;

<h2 id="rollout-steps">
  Rollout-Schritte
</h2>

Der Rollout umfasst fünf Schritte, jeder mit einem Checkpoint:

1. [Bestätigen Sie, dass das Gateway Ihre Modelle weiterleitet](#confirm-the-gateway-routes-your-models)
2. [Geben Sie jedem Entwickler eine Anmeldedaten aus](#issue-developer-credentials)
3. [Testen Sie Claude Code gegen das Gateway](#test-claude-code-against-the-gateway)
4. [Verteilen Sie die Basis-URL und Anmeldedaten](#distribute-the-configuration)
5. [Überprüfen Sie von einer Entwicklermaschine](#verify-the-rollout)

Die Schritte beinhalten drei verschiedene Anmeldedaten, und die Checkpoints benennen sie nach Platzhaltern, damit Sie sagen können, welche fehlerhaft ist, wenn etwas schiefgeht:

| Anmeldedaten                      | Wer hält sie                                                                                                   | Platzhalter in Checkpoints                                     |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------- |
| Anbieteranmeldedaten              | Das Gateway, das sie an den Upstream-Anbieter weiterleitet                                                     | Auf dem Gateway konfiguriert; erscheint nie in Client-Befehlen |
| Gateway-Administratoranmeldedaten | Sie, wenn Ihr Gateway-Produkt eine für seine Admin- oder Test-Schnittstelle ausgibt                            | `<gateway-key>`                                                |
| Entwicklerschlüssel               | Jeder Entwickler, ausgestellt vom Gateway in [Entwickleranmeldedaten ausstellen](#issue-developer-credentials) | `<developer-key>`                                              |

<h3 id="confirm-the-gateway-routes-your-models">
  Bestätigen Sie, dass das Gateway Ihre Modelle weiterleitet
</h3>

Ihr Gateway sollte bereits mit Ihren Anbieteranmeldedaten konfiguriert sein, auf seiner Basis-URL lauschen und Anfragen an die API Ihres Anbieters weiterleiten. Testen Sie, dass der Pfad end-to-end mit einer minimalen Anfrage funktioniert, indem Sie zwei Werte aus Ihrer Bereitstellung ersetzen:

* `<gateway-key>` ist alles, was Ihnen derzeit erlaubt, das Gateway aufzurufen: ein Administratorschlüssel, ein Test-Schlüssel oder Ihr eigener Entwicklerschlüssel, wenn Sie bereits einen ausgestellt haben. Nicht jedes Gateway-Produkt hat eine separate Admin-Anmeldedaten; wenn Ihres nicht, geben Sie sich selbst einen Entwicklerschlüssel in [Entwickleranmeldedaten ausstellen](#issue-developer-credentials) aus
* `model` ist ein Claude-Modellname, den Ihr Gateway weiterleiten soll. Das Beispiel verwendet `claude-sonnet-4-6`; ersetzen Sie einen Namen, den Sie konfiguriert haben

<Tabs>
  <Tab title="Bash oder Zsh">
    ```bash theme={null}
    curl -X POST "https://llm-gateway.example.com/v1/messages" \
      -H "Authorization: Bearer <gateway-key>" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    Invoke-RestMethod -Method Post -Uri "https://llm-gateway.example.com/v1/messages" `
      -Headers @{ "Authorization" = "Bearer <gateway-key>"; "anthropic-version" = "2023-06-01" } `
      -ContentType "application/json" `
      -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>
</Tabs>

**Checkpoint**: ein `200` mit einem `content`-Feld bedeutet, dass das Gateway den Anbieter mit diesem Modellnamen erreicht hat. Ein `404` bedeutet, dass dieser Name am Gateway nicht weitergeleitet wird; ein `401` vom Anbieter bedeutet, dass die Anbieteranmeldedaten des Gateways falsch sind.

Wiederholen Sie die Anfrage einmal pro Claude-Modellname in der Routing-Konfiguration Ihres Gateways. Ein Name, den das Gateway nicht weiterleitet, gibt `404` an jeden Entwickler zurück, der ihn auswählt, daher testen Sie jeden Namen vor dem Rollout.

<Note>
  Vermeiden Sie, das Gateway hinter einer Umleitung bereitzustellen. Eine Umleitung kann den Request-Body ablegen oder den Anmeldedaten-Header bei Inferenzanfragen entfernen, und [Modellermittlung](/docs/de/llm-gateway-protocol#model-discovery) behandelt jede Umleitung als Fehler, daher können die Anmeldedaten nicht zu einem Umleitungsziel durchsickern.
</Note>

<h3 id="issue-developer-credentials">
  Geben Sie Entwickleranmeldedaten aus
</h3>

Jeder Entwickler benötigt seinen eigenen Gateway-Schlüssel zur Authentifizierung. Erstellen Sie eine Anmeldedaten pro Entwickler am Gateway, gemäß der Dokumentation zur Anmeldedatenverwaltung Ihres Produkts.

Bestätigen Sie, dass ein neu ausgestellter Schlüssel gegen das Gateway mit der gleichen Anfrage wie [Bestätigen Sie, dass das Gateway Ihre Modelle weiterleitet](#confirm-the-gateway-routes-your-models) funktioniert, indem Sie `<gateway-key>` durch den neuen `<developer-key>` ersetzen:

<Tabs>
  <Tab title="Bash oder Zsh">
    ```bash theme={null}
    curl -X POST "https://llm-gateway.example.com/v1/messages" \
      -H "Authorization: Bearer <developer-key>" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    Invoke-RestMethod -Method Post -Uri "https://llm-gateway.example.com/v1/messages" `
      -Headers @{ "Authorization" = "Bearer <developer-key>"; "anthropic-version" = "2023-06-01" } `
      -ContentType "application/json" `
      -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>
</Tabs>

**Checkpoint**: ein `200` mit einem `content`-Feld bedeutet, dass der Entwicklerschlüssel das Gateway erreicht und das Gateway ihn weiterleitet. Ein `401` hier, wenn [der vorherige Schritt](#confirm-the-gateway-routes-your-models) erfolgreich war, bedeutet, dass der Entwicklerschlüssel falsch ist oder noch nicht am Gateway wirksam geworden ist.

Das Ausstellen eines Schlüssels pro Entwickler anstelle eines gemeinsamen Schlüssels ist das, was die Zuordnung der Nutzung pro Entwickler und das individuelle Offboarding ermöglicht. Die Umgebungsvariable, die den Schlüssel hält, hängt davon ab, welcher Header das Gateway liest. Für ein Gateway, das Anmeldedaten im `Authorization: Bearer`-Header überprüft, setzen Entwickler ihren Schlüssel in `ANTHROPIC_AUTH_TOKEN`. Für ein Gateway, das Schlüssel aus dem `x-api-key`-Header liest, setzen Entwickler stattdessen `ANTHROPIC_API_KEY`; die [Anmeldedatentabelle](/docs/de/llm-gateway-connect#set-the-credential-variable) behandelt die Zuordnung.

<h3 id="test-claude-code-against-the-gateway">
  Testen Sie Claude Code gegen das Gateway
</h3>

Führen Sie Claude Code selbst durch das Gateway aus, bevor Sie etwas verteilen, indem Sie die gleiche Konfiguration verwenden, die der Rollout unternehmensweite bereitstellen wird. Geben Sie diese direkt in einem Terminal ein, nicht in einer `.env`- oder Einstellungsdatei; sie gelten nur für diese Terminal-Sitzung, daher kehrt Ihre Maschine zu ihrer normalen Konfiguration zurück, wenn Sie sie schließen. Verwenden Sie `ANTHROPIC_API_KEY` anstelle von `ANTHROPIC_AUTH_TOKEN`, wenn Ihr Gateway den `x-api-key`-Header liest:

<Tabs>
  <Tab title="Bash oder Zsh">
    ```bash theme={null}
    export ANTHROPIC_BASE_URL=https://llm-gateway.example.com
    export ANTHROPIC_AUTH_TOKEN="<developer-key>"
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_BASE_URL = "https://llm-gateway.example.com"
    $env:ANTHROPIC_AUTH_TOKEN = "<developer-key>"
    ```
  </Tab>
</Tabs>

Senden Sie dann einen einmaligen Prompt durch das Gateway:

```bash theme={null}
claude -p "Reply with one word: connected"
```

**Checkpoint**: der Prompt gibt eine Antwort zurück, und die Anfrage erscheint im Gateway-Log als `POST` zum `/v1/messages`-Pfad mit Status `200`. Claude Code hängt eine Query-Zeichenkette wie `?beta=true` an, daher stimmen Sie mit dem Pfad überein, nicht mit der vollständigen URL. Zwei Fehlermeldungen weisen in verschiedene Richtungen:

* `Not logged in`: überprüfen Sie das Gateway-Log, um die beiden Ursachen auseinanderzuhalten. Wenn es leer ist, hat keine Anmeldedaten die Sitzung erreicht und keine Anfrage hat die Maschine verlassen; führen Sie die Exporte in der Shell aus, die Sie testen. Wenn es eine abgelehnte Anfrage mit `x-api-key` im `401`-Body zeigt, erwartet das Gateway Schlüssel in diesem Header stattdessen; wechseln Sie zu `ANTHROPIC_API_KEY`
* `Failed to authenticate. API Error: 401` bedeutet, dass eine Anmeldedaten gesendet und abgelehnt wurde, und das Gateway-Log sagt, wo: ein `401`, das `api.anthropic.com` oder Ihren Anbieter-Endpunkt benennt, bedeutet, dass das Gateway den Upstream erreicht hat, aber seine Anbieteranmeldedaten, die es hält, abgelehnt wurden, daher funktionierte der Entwicklerschlüssel und die Anbieteranmeldedaten, die das Gateway hält, sind falsch oder ein Platzhalter

Eine falsche oder unerreichbare Basis-URL erzeugt ein anderes Symptom: Claude Code [versucht die Verbindung mit Backoff erneut](/docs/de/errors#automatic-retries) und kann mehrere Minuten ohne Ausgabe sitzen, bevor es einen Fehler meldet. Wenn der Befehl zu hängen scheint, überprüfen Sie stattdessen das Gateway-Log, anstatt zu warten; keine ankommende Anfrage bedeutet, dass `ANTHROPIC_BASE_URL` nicht auf das Gateway zeigt.

<h3 id="distribute-the-configuration">
  Verteilen Sie die Konfiguration
</h3>

Jede Entwicklermaschine benötigt die Gateway-Adresse und eine Anmeldedaten. Sie können sie zentral über [verwaltete Einstellungen](/docs/de/settings#settings-files) verteilen, damit Entwickler nichts konfigurieren, oder Entwickler die Werte selbst setzen lassen.

<h4 id="what-to-distribute">
  Was zu verteilen ist
</h4>

Der gleiche Satz von Variablen gilt, welchen Weg Sie auch wählen. Die meisten Rollouts benötigen nur `ANTHROPIC_BASE_URL` und eine Anmeldedaten; fügen Sie die bedingten Zeilen ein, wenn Ihr Gateway-Setup sie erfordert.

| Variable oder Einstellung                                                                                                                                                                                                           | Was sie tut                                                                                                                                                                                                              | Einschließen wenn                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_BASE_URL`                                                                                                                                                                                                                | Sendet Claude Codes API-Anfragen an das Gateway anstelle von `api.anthropic.com`                                                                                                                                         | Immer                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `apiKeyHelper`, oder eine Anmeldedaten in `ANTHROPIC_AUTH_TOKEN` oder `ANTHROPIC_API_KEY`                                                                                                                                           | Authentifiziert jede Anfrage an das Gateway. Der Helper führt einen Befehl aus, um den Schlüssel zu holen; die Variablen halten einen statischen Schlüssel, gesendet als `Authorization: Bearer` und `x-api-key` jeweils | Immer; eine der drei                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `ANTHROPIC_CUSTOM_HEADERS`                                                                                                                                                                                                          | Fügt zusätzliche HTTP-Header zu jeder API-Anfrage hinzu                                                                                                                                                                  | Ihr Gateway erfordert einen Mandanten- oder Routing-Header bei jeder Anfrage                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY`                                                                                                                                                                                        | Fragt das `/v1/models` des Gateways beim Start ab und fügt die zurückgegebenen Namen zur `/model`-Auswahl hinzu                                                                                                          | Ihr Gateway stellt `/v1/models` bereit und Sie möchten, dass die Auswahl der Entwickler von ihm gefüllt wird                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`                                                                                                                                                                                            | Stoppt Claude Code beim Senden von Pre-Release-Capability-Headern und Body-Feldern                                                                                                                                       | Ihr Gateway leitet zu einem Amazon Bedrock- oder Google Cloud Agent Platform-Upstream weiter, der Beta-Felder ablehnt; siehe [Gateway-Anforderungen](#gateway-requirements)                                                                                                                                                                                                                                                                                                                                                            |
| `ANTHROPIC_MODEL` oder [`ANTHROPIC_DEFAULT_HAIKU_MODEL`](/docs/de/model-config)                                                                                                                                                          | Legen Sie fest, welchen Modellnamen Claude Code für die Hauptsitzung und für Hintergrund-Traffic anfordert                                                                                                               | Ihr Gateway leitet Modellnamen weiter, die nicht Claude Codes Standardwerte entsprechen, oder Sie leiten [Hintergrund-Funktionalität](/docs/de/costs#background-token-usage) an ein anderes Modell weiter. Leiten Sie sowohl die Override-Namen als auch die integrierten Modell-IDs weiter, die Claude Code anfordert, wenn kein Override gesetzt ist, da einige Unter-Aufrufe unabhängig vom Override eine integrierte ID anfordern; [Modellkonfiguration](/docs/de/model-config) behandelt, welches Modell jeder Teil einer Sitzung verwendet |
| `ANTHROPIC_BEDROCK_BASE_URL`, `ANTHROPIC_VERTEX_BASE_URL`, `ANTHROPIC_FOUNDRY_BASE_URL`, oder `ANTHROPIC_AWS_BASE_URL` mit den [Variablen für diesen Anbieter](/docs/de/llm-gateway-connect#route-to-a-cloud-provider-through-a-gateway) | Zeigen Sie Claude Code auf das Gateway durch eine anbieter-spezifische Basis-URL. Amazon Bedrock und Google Cloud Agent Platform wechseln auch zu den nativen Request-Formaten dieser Anbieter                           | Ihr Gateway frontet Amazon Bedrock, Google Cloud Agent Platform, Microsoft Foundry oder die Claude Platform auf AWS; siehe [API-Formate](/docs/de/llm-gateway-protocol#api-formats)                                                                                                                                                                                                                                                                                                                                                         |

<h4 id="distribute-through-managed-settings">
  Verteilen Sie über verwaltete Einstellungen
</h4>

Liefern Sie die Variablen über den `env`-Block einer [verwalteten Einstellungsdatei](/docs/de/settings#settings-files), die von MDM, Registry-Richtlinie oder Konfigurationsverwaltung gepusht wird:

```json theme={null}
{
  "env": {
    "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com"
  },
  "apiKeyHelper": "/usr/local/bin/get-gateway-key"
}
```

Fügen Sie die bedingten Variablen aus der Tabelle zum gleichen `env`-Block hinzu. Ein verwalteter `ANTHROPIC_BASE_URL` wird erzwungen und kann nicht durch einen Shell-Export eines Entwicklers überschrieben werden, da Claude Code ihn über die Prozessumgebung und niedrigere Prioritätseinstellungen anwendet.

Fügen Sie `forceLoginMethod` oder `forceLoginOrgUUID` nicht in verwalteten Einstellungen neben einer Gateway-Anmeldedaten ein. Auf Claude Code v2.1.146 und später blockiert jeder Schlüssel `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` und `apiKeyHelper` beim Start, daher sehen Entwickler `This machine's managed settings require a first-party login` und können nicht fortfahren.&#x20;

[Server-verwaltete Einstellungen](/docs/de/server-managed-settings#platform-availability) Lieferung erfordert eine direkte Verbindung zu `api.anthropic.com`, daher erreicht sie keine Gateway-gerouteten Sitzungen. Gateway-Bereitstellungen verwenden diesen dateigestützten verwalteten Einstellungspfad, der die gleichen Schlüssel erzwingt.

Für die Anmeldedaten verteilen Sie einen [`apiKeyHelper`](/docs/de/llm-gateway-connect#rotate-credentials-with-apikeyhelper)-Befehl in der verwalteten Einstellungsdatei wie oben gezeigt; der Befehl authentifiziert sich bei Ihrem Secrets-Store als lokaler Entwickler, daher erhält jede Maschine ihren eigenen Schlüssel. Alternativ liefern Sie jedem Entwickler seinen Schlüssel über Ihren bestehenden Secrets-Prozess und lassen ihn `ANTHROPIC_AUTH_TOKEN` selbst setzen.

Einige Umgebungen benötigen separate Lieferung:

* Die Desktop-App liest Gateway-Routing nur aus ihrer MDM-bereitgestellten Drittanbieter-Inferenzkonfiguration; stellen Sie diese Datei neben verwalteten Einstellungen bereit, damit Desktop-Sitzungen auch durch das Gateway geleitet werden. Siehe die [Desktop-Drittanbieter-Konfigurationsdocs](https://claude.com/docs/third-party/claude-desktop/configuration) und die [Desktop-Gateway-Docs](https://claude.com/docs/third-party/claude-desktop/gateway)
* CI-Runner benötigen `ANTHROPIC_BASE_URL` und die Anmeldedaten in der [Umgebung des Runners](/docs/de/llm-gateway-connect#configure-each-surface) gesetzt
* WSL auf verwalteten Windows-Maschinen liest die Windows-verwalteten Einstellungen nur, wenn [`wslInheritsWindowsSettings`](/docs/de/settings#available-settings) `true` ist

<h4 id="hand-developers-the-values-to-set-themselves">
  Geben Sie Entwicklern die Werte, um sie selbst zu setzen
</h4>

Wenn Sie keine verwaltete Einstellungsverteilung eingerichtet haben, senden Sie jedem Entwickler, was er benötigt, um die [Verbindungsseite](/docs/de/llm-gateway-connect#configure-claude-code-yourself) zu befolgen:

* Die Gateway-URL
* Ihre persönliche Anmeldedaten
* **Welche Variable die Anmeldedaten einzufügen ist**: `ANTHROPIC_AUTH_TOKEN` für ein Bearer-Token-Gateway, oder `ANTHROPIC_API_KEY` für ein `x-api-key`-Gateway. Entwicklern zu sagen, welche spart ihnen das Trial-and-Error, das auf der [Verbindungsseite](/docs/de/llm-gateway-connect#set-the-credential-variable) beschrieben ist
* Alle bedingten Variablen aus der [Was zu verteilen ist-Tabelle](#what-to-distribute), mit ihren Werten

Die [Verbindungsseite](/docs/de/llm-gateway-connect#configure-claude-code-yourself) führt Entwickler durch das Setzen jeder einzelnen.

**Checkpoint**: auf einer Entwicklermaschine startet `claude` eine Sitzung ohne den Login-Bildschirm anzuzeigen, da die verteilte Anmeldedaten die Authentifizierung erfüllt. Führen Sie dann `/status` aus und öffnen Sie die **Status**-Registerkarte: die `Anthropic base URL`-Zeile zeigt die Gateway-Adresse, und für verwaltete Verteilung enthält die `Setting sources`-Zeile verwaltete Einstellungen. Ein Login-Bildschirm oder eine fehlende `Anthropic base URL`-Zeile bedeutet, dass die Konfiguration die Maschine nicht erreicht hat.

<h3 id="verify-the-rollout">
  Überprüfen Sie den Rollout
</h3>

Bestätigen Sie, dass alles von einer Entwicklermaschine aus funktioniert, nicht vom Gateway-Host, damit der Test den Netzwerkpfad abdeckt, den Entwickler verwenden. Senden Sie eine Streaming-Anfrage, die den Endpunkt, Streaming-Pass-Through und Modell-Routing auf einmal überprüft:

<Tabs>
  <Tab title="Bash oder Zsh">
    ```bash theme={null}
    curl -N -X POST "https://llm-gateway.example.com/v1/messages" \
      -H "Authorization: Bearer <developer-key>" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 16, "stream": true, "messages": [{"role": "user", "content": "count to 3"}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $body = '{"model": "claude-sonnet-4-6", "max_tokens": 16, "stream": true, "messages": [{"role": "user", "content": "count to 3"}]}'
    $body | curl.exe -N -X POST "https://llm-gateway.example.com/v1/messages" `
      -H "Authorization: Bearer <developer-key>" `
      -H "anthropic-version: 2023-06-01" `
      -H "content-type: application/json" `
      --data-binary '@-'
    ```
  </Tab>
</Tabs>

Sie sollten `data:`-Zeilen inkrementell ankommen sehen. Die gesamte Antwort, die auf einmal nach einer Pause ankommt, bedeutet, dass das Gateway puffert, was Claude Code staut; ein `404` bedeutet, dass der Modellname nicht weitergeleitet wird. Wiederholen Sie pro Modellname.

Starten Sie dann `claude` und senden Sie eine Nachricht. Jedes Symptom in diesem Schritt hat eine Ursache:

* Eine Login-Aufforderung bedeutet eine Anmeldedaten-Lücke. Führen Sie `/status` aus und öffnen Sie die **Status**-Registerkarte: wenn die `Setting sources`-Zeile verwaltete Einstellungen nicht enthält, hat die Verteilung die Maschine nicht erreicht; wenn sie es tut, wurde die Entwickleranmeldedaten nicht bereitgestellt, daher setzen Sie `ANTHROPIC_AUTH_TOKEN` oder den `apiKeyHelper`
* `Failed to authenticate`-Fehler bedeuten, dass das Gateway Anfragen ablehnt; sein Log sagt, welche Anmeldedaten fehlgeschlagen ist. Eine Ablehnung, die das Gateway selbst protokolliert, benennt den Entwicklerschlüssel, während ein `401` von `api.anthropic.com` oder Ihrem Anbieter-Endpunkt bedeutet, dass die Anbieteranmeldedaten, die das Gateway hält, abgelehnt wurden
* Eine einmalige Genehmigungsaufforderung für den Schlüssel ist beim ersten Gebrauch erwartet, wenn das Gateway Schlüssel im `x-api-key`-Header erwartet, gesetzt als `ANTHROPIC_API_KEY`. Mit `ANTHROPIC_AUTH_TOKEN` erscheint keine Aufforderung und die Variable übernimmt stillschweigend; ein zuvor gespeicherter claude.ai-Login ist für diese Sitzung inaktiv

Überprüfen Sie abschließend die Logs des Gateways auf die Nachricht, die Sie gesendet haben: die Anmeldedaten identifiziert den Entwickler, und der [`x-claude-code-session-id`-Header](/docs/de/llm-gateway-protocol#request-headers) gruppiert Anfragen nach Sitzung. Wenn Features mit den [Fehlerbehebungssymptomen](/docs/de/llm-gateway-connect#troubleshoot-gateway-errors) fehlschlagen, entfernt das Gateway Header oder schreibt Fehler um; siehe die [Gateway-Anforderungen](#gateway-requirements) oben.

<h2 id="maintain-the-gateway">
  Verwalten Sie das Gateway
</h2>

Nach dem Rollout erreichen drei Arten von Änderungen das Gateway im Laufe der Zeit. Jede hat ein Symptom, auf das man achten sollte, und eine Aktion, die man ergreifen sollte.

| Änderung                                                                              | Symptom, wenn das Gateway nicht mitgehalten hat                                                                                                                                     | Aktion                                                                                                                                                                                                                                                                                          |
| :------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Neue Claude Code-Versionen fügen `anthropic-beta`-Werte und Request-Body-Felder hinzu | Entwickler berichten `400`-Fehler, die ein neues Feld nach der Aktualisierung von Claude Code benennen; siehe [Feature-Pass-Through](/docs/de/llm-gateway-protocol#feature-pass-through) | Leiten Sie `anthropic-*`-Header und Request-Bodies wörtlich weiter, anstatt eine Allowlist zu verwenden; testen Sie neue Claude Code-Versionen gegen das Gateway, bevor sie Entwickler erreichen                                                                                                |
| Neue Claude-Modelle werden verfügbar                                                  | Entwickler, die einen neuen Modellnamen auswählen, erhalten `404`; die `/model`-Auswahl listet ihn nicht auf                                                                        | Fügen Sie den Modellnamen zur Routing-Konfiguration des Gateways hinzu, führen Sie dann die [Routing-Überprüfung](#confirm-the-gateway-routes-your-models) erneut aus. Wenn Sie `ANTHROPIC_MODEL` oder die Standard-Modell-Variablen verteilen, aktualisieren Sie die verwalteten Einstellungen |
| Anmeldedaten verfallen oder müssen rotiert werden                                     | Alle Entwickleranfragen beginnen mit `401` vom Upstream zu fehlschlagen                                                                                                             | Rotieren Sie die Anbieteranmeldedaten des Gateways nach eigenem Zeitplan; Entwicklerschlüssel rotieren am Gateway, und ein [`apiKeyHelper`](/docs/de/llm-gateway-connect#rotate-credentials-with-apikeyhelper) handhabt die Rotation pro Entwickler, ohne Einstellungen neu zu verteilen             |

Berücksichtigen Sie bei der Dimensionierung von Pro-Schlüssel-Ratenlimits den Client [Wiederholung von transienten Fehlern](/docs/de/errors#automatic-retries), einschließlich `429`-Antworten, bis zu 10 Mal mit Backoff, unter Beachtung von `Retry-After`. Behalten Sie die [Protokollreferenz](/docs/de/llm-gateway-protocol) als Vertrag für das, was jede Claude Code-Version sendet.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Claude Code mit einem LLM-Gateway verbinden](/docs/de/llm-gateway-connect): die Einrichtungsschritte für Entwickler, mit Pro-Surface-Konfiguration und einer Fehlerbehebungstabelle, die Sie Entwicklern geben können
* [Gateway-Protokollreferenz](/docs/de/llm-gateway-protocol): der Wire-Vertrag für Gateway-Betreiber, der Endpunkte, weiterzuleitende Header und die Feature-Pass-Through-Tabelle abdeckt
* [Einstellungsdateien und Priorität](/docs/de/settings#settings-files): wie verwaltete, Projekt- und Benutzereinstellungen kombiniert werden, und wo die verwaltete Datei auf jeder Plattform geht
* [Richten Sie Claude Code für Ihre Organisation ein](/docs/de/admin-setup): der breitere Rollout, von dem dieses Gateway ein Teil ist, einschließlich Richtliniendurchsetzung, Nutzungssichtbarkeit und Datenbehandlung
