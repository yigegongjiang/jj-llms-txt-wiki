> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code mit einem LLM-Gateway verbinden

> Richten Sie Claude Code auf das LLM-Gateway Ihrer Organisation aus. Überprüfen Sie, ob Ihr Administrator es bereits konfiguriert hat, oder legen Sie die Basis-URL und die Anmeldedaten selbst fest, überprüfen Sie dann die Verbindung und beheben Sie Gateway-Fehler.

Ein [LLM-Gateway](/docs/de/llm-gateway) ist ein Proxy, den Ihre Organisation zwischen Claude Code und dem Modell-Anbieter betreibt. Wenn Ihre Organisation einen verwendet, authentifiziert sich Claude Code beim Gateway mit einer Anmeldedaten, die Ihre Organisation ausstellt, anstelle Ihres persönlichen claude.ai-Logins.

Diese Seite ist für Entwickler, die Claude Code über ein Gateway ausführen, das ihre Organisation betreibt. Sie behandelt zwei Pfade: [Überprüfung, ob Ihr Administrator es bereits für Sie konfiguriert hat](#check-for-an-existing-configuration), und [Konfiguration selbst](#configure-claude-code-yourself), wenn dies nicht der Fall ist.

<Note>
  * Um ein Gateway für Ihre Organisation bereitzustellen, siehe [Rollout eines LLM-Gateways](/docs/de/llm-gateway-rollout)
  * Für das, was Claude Code an ein Gateway sendet, siehe die [Gateway-Protokoll-Referenz](/docs/de/llm-gateway-protocol)
</Note>

<h2 id="check-for-an-existing-configuration">
  Überprüfung einer vorhandenen Konfiguration
</h2>

Administratoren können die Gateway-Adresse und die Anmeldedaten über [verwaltete Einstellungen](/docs/de/settings#settings-files), Geräteverwaltung oder einen [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper) verteilen, sodass Claude Code sie beim Start ohne weitere Konfiguration aufgreift. Um zu überprüfen, ob Ihre Organisation dies bereits getan hat:

<Steps>
  <Step title="Claude Code starten">
    Führen Sie `claude` aus. Wenn es sich stattdessen zum Anmeldebildschirm öffnet, wurde keine Gateway-Anmeldedaten verteilt; [konfigurieren Sie es selbst](#configure-claude-code-yourself) unten.
  </Step>

  <Step title="Überprüfen Sie die Registerkarte Status">
    Wenn Claude Code eine Sitzung ohne Anmeldebildschirm gestartet hat, führen Sie `/status` aus, öffnen Sie die Registerkarte **Status**, und überprüfen Sie zwei Zeilen:

    * `Anthropic base URL`: Diese Zeile wird nur angezeigt, wenn eine Gateway-Adresse festgelegt ist. Wenn sie nicht vorhanden ist, ist Claude Code nicht auf das Gateway ausgerichtet; [konfigurieren Sie es selbst](#configure-claude-code-yourself) unten.
    * `Auth token` oder `API key`: Eine Zeile mit `ANTHROPIC_AUTH_TOKEN`, `ANTHROPIC_API_KEY` oder einem `apiKeyHelper` bestätigt, dass eine Gateway-Anmeldedaten aktiv ist. Eine `Login method`-Zeile mit einem claude.ai-Konto bedeutet stattdessen, dass die Anmeldedaten nicht verteilt wurden; [legen Sie sie selbst fest](#set-the-credential-variable).
  </Step>

  <Step title="Senden Sie eine Testnachricht">
    Schließen Sie das `/status`-Menü und senden Sie eine beliebige Eingabeaufforderung in Claude Code. Eine normale Antwort von Claude ohne Fehler bestätigt, dass die Gateway-Verbindung funktioniert.
  </Step>
</Steps>

Wenn beide Zeilen im `/status`-Menü richtig aussehen, aber die Nachricht an Claude fehlschlägt, siehe die [Fehlerbehebungstabelle](#troubleshoot-gateway-errors).

<h2 id="configure-claude-code-yourself">
  Claude Code selbst konfigurieren
</h2>

Um Claude Code selbst für das Gateway zu konfigurieren, benötigen Sie von Ihrem Gateway-Team:

* Die Basis-URL des Gateways
* Eine Anmeldedaten: eine Schlüssel- oder Token-Zeichenkette oder ein Befehl, der eine abruft
  * Wenn Ihr Gateway-Team nicht angegeben hat, welche Art von Anmeldedaten es ist, behandelt der Abschnitt [Anmeldedaten-Variable](#set-the-credential-variable) unten, was zu versuchen ist

Die folgenden Abschnitte behandeln die Konfiguration in Reihenfolge:

* [Legen Sie die Anmeldedaten-Variable fest](#set-the-credential-variable) und [legen Sie die Basis-URL fest](#set-the-base-url-and-credential): die zwei Variablen, die jede Gateway-Verbindung benötigt
* [Überprüfen Sie die Verbindung](#verify-the-connection): bestätigen Sie, dass sie funktioniert, bevor Sie etwas speichern
* [Konfigurieren Sie jede Oberfläche](#configure-each-surface): Wenn Sie eine andere Oberfläche als die Claude Code CLI verwenden, z. B. VS Code, erfahren Sie, wie Sie sie mit Ihren Gateway-Anmeldedaten konfigurieren
* [Zusätzliche Konfiguration](#additional-configuration): Variablen, die einige Gateways über die Basis-URL und die Anmeldedaten hinaus benötigen, z. B. einen benutzerdefinierten Header, einen Anmeldedaten-Helper, Modellermittlung, eine Basis-URL im Anbieterformat oder das Ausschalten des Datenverkehrs außerhalb des Gateway-Pfads. Legen Sie diese nur fest, wenn Ihr Administrator sie benannt hat oder Ihr Netzwerk den ausgehenden Datenverkehr einschränkt

<h3 id="set-the-credential-variable">
  Legen Sie die Anmeldedaten-Variable fest
</h3>

Um Claude Code beim Gateway zu authentifizieren, legen Sie Ihre Anmeldedaten in einer Umgebungsvariablen fest. Welche Variable hängt davon ab, was Ihr Gateway-Team Ihnen mitgeteilt hat:

| Legen Sie die Anmeldedaten fest in                      | Verwenden Sie, wenn                                               |
| :------------------------------------------------------ | :---------------------------------------------------------------- |
| `ANTHROPIC_AUTH_TOKEN`                                  | Ihr Gateway-Team sagte 'Bearer-Token" oder „Authorization-Header" |
| `ANTHROPIC_API_KEY`                                     | Ihr Gateway-Team sagte „API-Schlüssel" oder „x-api-key"           |
| [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper) | Die Anmeldedaten rotieren oder stammen aus einem Tresor           |

Wenn Sie nicht angegeben wurden, welche Art, verwenden Sie `ANTHROPIC_AUTH_TOKEN`; die [Überprüfungsanfrage](#verify-the-connection) unten zeigt, wie Sie feststellen, ob Sie wechseln müssen.

<h3 id="set-the-base-url-and-credential">
  Legen Sie die Basis-URL und die Anmeldedaten fest
</h3>

Legen Sie die Basis-URL des Gateways und die Anmeldedaten-Variable, die Sie oben ausgewählt haben, als Umgebungsvariablen fest. Die Beispiele verwenden `ANTHROPIC_AUTH_TOKEN`; ersetzen Sie es durch `ANTHROPIC_API_KEY`, wenn das [die Variable ist, die Sie ausgewählt haben](#set-the-credential-variable). Sie können sie [in Ihrer Shell](#set-as-shell-environment-variables) festlegen, was für eine Terminal-Sitzung gilt, oder [in einer Claude Code-Einstellungsdatei](#set-in-a-settings-file), was überall dort bestehen bleibt, wo Claude Code ausgeführt wird.

Beginnen Sie für Ihre erste Verbindung mit Shell-Exporten und führen Sie die [Überprüfungsanfrage](#verify-the-connection) aus, bevor Sie die Werte in eine Einstellungsdatei verschieben.

<h4 id="set-as-shell-environment-variables">
  Legen Sie als Shell-Umgebungsvariablen fest
</h4>

Ersetzen Sie die Werte durch die, die Ihr Gateway-Team Ihnen gegeben hat:

<Tabs>
  <Tab title="Bash oder Zsh">
    ```bash theme={null}
    export ANTHROPIC_BASE_URL=https://llm-gateway.example.com
    export ANTHROPIC_AUTH_TOKEN=sk-gateway-key
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_BASE_URL = "https://llm-gateway.example.com"
    $env:ANTHROPIC_AUTH_TOKEN = "sk-gateway-key"
    ```
  </Tab>
</Tabs>

Shell-Exporte gelten nur für diese Terminal-Sitzung und Programme, die von ihr aus gestartet werden; ein Editor, der vom Dock oder Startmenü gestartet wird, sieht sie nicht. Um sie über neue Terminals hinweg bestehen zu lassen, fügen Sie die gleichen Zeilen zu Ihrem Shell-Profil hinzu, z. B. `~/.zshrc`, `~/.bashrc` oder Ihr PowerShell-`$PROFILE`, oder verwenden Sie stattdessen eine Einstellungsdatei.

<h4 id="set-in-a-settings-file">
  Legen Sie in einer Einstellungsdatei fest
</h4>

Um die Konfiguration überall dort anzuwenden, wo Claude Code ausgeführt wird, ohne von Ihrer Shell abhängig zu sein, legen Sie die Variablen im `env`-Block einer [Einstellungsdatei](/docs/de/settings) fest. Einstellungsdateien haben unterschiedliche Bereiche:

* `~/.claude/settings.json` gilt für alle Ihre Projekte. Unter Windows ist der Pfad `%USERPROFILE%\.claude\settings.json`
* `.claude/settings.local.json` gilt für ein Projekt. Claude Code fügt es zu Ihrem gitignore hinzu, wenn es die Datei erstellt; wenn Sie sie selbst erstellen, fügen Sie sie zuerst manuell zu Ihrem gitignore hinzu, damit Sie Ihre Anmeldedaten nicht versehentlich committen

<Warning>
  Legen Sie die Anmeldedaten nicht in die `.claude/settings.json` eines Projekts. Diese Datei wird committed und mit jedem geteilt, der das Repository klont.
</Warning>

Der `env`-Block sieht in beiden Dateien gleich aus:

```json theme={null}
{
  "env": {
    "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com",
    "ANTHROPIC_AUTH_TOKEN": "sk-gateway-key"
  }
}
```

Wenn sowohl ein Shell-Export als auch ein `env`-Block einer Einstellungsdatei die gleiche Variable festlegen, gilt der Wert der Einstellungsdatei. Führen Sie `/status` aus, um zu sehen, welche Basis-URL und Anmeldedaten-Quelle Claude Code verwendet.

<h3 id="verify-the-connection">
  Überprüfen Sie die Verbindung
</h3>

Mit den in Ihrer Shell exportierten Variablen senden Sie eine Anfrage mit einem Token direkt an das Gateway. Dies bestätigt, dass die URL und die Anmeldedaten funktionieren, bevor Sie Claude Code öffnen, sodass ein Fehler auf das Gateway statt auf Ihre Konfiguration hinweist. Die folgenden Befehle lesen die Shell-Variablen, daher benötigen sie die [Shell-Exporte](#set-as-shell-environment-variables), auch wenn Sie die Werte auch in eine Einstellungsdatei eingeben.

<Tabs>
  <Tab title="Bash oder Zsh">
    ```bash theme={null}
    curl -X POST "$ANTHROPIC_BASE_URL/v1/messages" \
      -H "Authorization: Bearer $ANTHROPIC_AUTH_TOKEN" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    Invoke-RestMethod -Method Post -Uri "$env:ANTHROPIC_BASE_URL/v1/messages" `
      -Headers @{ "Authorization" = "Bearer $env:ANTHROPIC_AUTH_TOKEN"; "anthropic-version" = "2023-06-01" } `
      -ContentType "application/json" `
      -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>
</Tabs>

Wenn Ihr Gateway Schlüssel im `x-api-key`-Header erwartet, ersetzen Sie den `Authorization`-Header durch `x-api-key: $ANTHROPIC_API_KEY` im Bash-Befehl oder den `"Authorization"`-Hashtable-Eintrag durch `"x-api-key" = "$env:ANTHROPIC_API_KEY"` im PowerShell-Befehl.

Eine JSON-Antwort, die mit `{"id":"msg_` beginnt und ein `"content":[...]`-Feld enthält, bedeutet, dass das Gateway erreichbar ist und die Anmeldedaten funktionieren. Ein Fehler, der ein unbekanntes Modell benennt, beweist immer noch, dass die URL und die Anmeldedaten funktionieren, da das Gateway die Anfrage authentifiziert hat, bevor es den Modellnamen ablehnt; Sie müssen kein Modell finden, das Ihr Gateway für diesen Test bereitstellt. Ein `401` bedeutet, dass die Anmeldedaten abgelehnt wurden: Wenn Sie die Variable erraten haben, wechseln Sie zur anderen und exportieren Sie erneut.

<h4 id="confirm-in-claude-code">
  Bestätigen Sie in Claude Code
</h4>

Starten Sie `claude` aus der gleichen Shell, damit es die Exporte erbt, senden Sie eine Nachricht, und führen Sie `/status` aus.

Auf der Registerkarte **Status** sollte die Zeile `Anthropic base URL` Ihre Gateway-Adresse anzeigen, was bestätigt, dass Anfragen dorthin weitergeleitet werden; wenn die Zeile nicht vorhanden ist, hat die Variable die Sitzung nicht erreicht. Eine `Auth token`- oder `API key`-Zeile, die die Variable benennt, die Sie festgelegt haben, bestätigt, dass die Gateway-Anmeldedaten aktiv sind, anstatt eines gespeicherten claude.ai-Logins.

Wenn die Nachricht fehlschlägt oder `/status` die Gateway-URL nicht anzeigt, siehe die [Fehlerbehebungstabelle](#troubleshoot-gateway-errors) unten.

<h3 id="how-the-credential-variable-maps-to-a-header">
  Wie die Anmeldedaten-Variable einem Header zugeordnet wird
</h3>

Jede Variable sendet die Anmeldedaten in einem anderen HTTP-Header: `ANTHROPIC_AUTH_TOKEN` in `Authorization: Bearer`, `ANTHROPIC_API_KEY` in `x-api-key` und `apiKeyHelper` in beiden. Eine Anmeldedaten in der falschen Variable erreicht das Gateway in einem Header, den es nicht liest, und die Anfrage schlägt mit `401` fehl. Wenn die Überprüfungsanfrage `401` zurückgegeben hat, wechseln Sie zur anderen Variable und versuchen Sie es erneut.

<h3 id="conflicts-with-an-existing-login">
  Konflikte mit einem vorhandenen Login
</h3>

Eine Gateway-Anmeldedaten-Variable hat Vorrang vor einem gespeicherten claude.ai-Login oder Console-Schlüssel. Ihr claude.ai-Login bleibt gespeichert und ungenutzt, während die Variable festgelegt ist; heben Sie die Festlegung der Variable auf und Claude Code kehrt zu ihr zurück. Mit `ANTHROPIC_AUTH_TOKEN` hat die Variable sofort Vorrang. Mit `ANTHROPIC_API_KEY` werden Sie einmal im interaktiven Modus aufgefordert, den Schlüssel zu genehmigen, bevor er übernimmt.

Führen Sie `/status` aus, um zu bestätigen, welche Anmeldedaten-Quelle aktiv ist. Wenn der Start eine Auth-Konflikt-Warnung anzeigt, die zwei Quellen benennt, siehe die erste Zeile der [Fehlerbehebungstabelle](#troubleshoot-gateway-errors), um zu sehen, welche zu löschen ist. Um einen gespeicherten Login zu löschen, damit nur die Gateway-Anmeldedaten verbleiben, führen Sie `/logout` aus.

<h2 id="configure-each-surface">
  Konfigurieren Sie jede Oberfläche
</h2>

Die CLI liest die Umgebungsvariablen und Einstellungsdateien oben. Die anderen Oberflächen sind die VS Code-Erweiterung, die Desktop-App, GitHub Actions, das Agent SDK und die Cloud-Oberflächen wie Slack und das Web; die folgenden Abschnitte behandeln, ob diese Einstellungen jede erreichen.

<h3 id="vs-code-extension">
  VS Code-Erweiterung
</h3>

Legen Sie die Gateway-Variablen für die [VS Code-Erweiterung](/docs/de/vs-code) in `claudeCode.environmentVariables` in VS Codes eigenen Benutzereinstellungen fest, die mit dem Befehl **Preferences: Open User Settings (JSON)** geöffnet werden. Die Erweiterung überprüft Anmeldedaten aus dieser Einstellung vor dem Start, daher ist es der zuverlässige Ort für die Gateway-Anmeldedaten; Werte in `~/.claude/settings.json` erreichen den erzeugten Prozess, aber nicht die Anmeldungsprüfung der Erweiterung selbst.

```json theme={null}
{
  "claudeCode.environmentVariables": [
    { "name": "ANTHROPIC_BASE_URL", "value": "https://llm-gateway.example.com" },
    { "name": "ANTHROPIC_AUTH_TOKEN", "value": "sk-gateway-key" }
  ]
}
```

<h3 id="desktop-app">
  Desktop-App
</h3>

Die Desktop-App liest Gateway-Routing aus ihrer [Konfiguration für Drittanbieter-Inferenz](https://claude.com/docs/third-party/claude-desktop/gateway), nicht aus `ANTHROPIC_BASE_URL` oder `settings.json`. Diese Konfiguration kann von Ihrer Organisation oder von einem Formular in der App selbst stammen:

* **Von einem Administrator verteilt**: Wenn Ihre Organisation die [Konfiguration bereitgestellt hat](/docs/de/llm-gateway-rollout#distribute-through-managed-settings), leitet die Desktop-App ohne Einrichtung auf Ihrer Seite durch das Gateway
* **Lokal konfiguriert**: Für Geräte ohne eine von einem Administrator verteilte Konfiguration öffnen Sie Help → Troubleshooting → Enable Developer Mode, wodurch die App mit einem Developer-Menü neu gestartet wird. Öffnen Sie dann Developer → Configure Third-Party Inference und geben Sie Ihre Gateway-Basis-URL ein. Eine von einem Administrator verteilte Konfiguration hat Vorrang und macht dieses Formular schreibgeschützt

Mit der aktiven Gateway-Konfiguration führt die Desktop-App Sitzungen nur auf Ihrem lokalen Computer aus: Der Umgebungswähler bietet keine SSH-Sitzungen oder von Anthropic gehostete Cloud-Umgebungen an, und [Remote Control](/docs/de/remote-control) ist nicht verfügbar. Um Claude Code auf einem Remote-Host über das Gateway zu verwenden, führen Sie die CLI auf diesem Host mit [`ANTHROPIC_BASE_URL` und der Gateway-Anmeldedaten](#set-the-base-url-and-credential) aus, die dort festgelegt sind.

Wenn die Desktop-App `Gateway was unreachable` anzeigt, konnte die App die konfigurierte Basis-URL beim Start nicht erreichen; überprüfen Sie die URL und den Netzwerkpfad mit dem [curl-Test oben](#verify-the-connection).

<h3 id="github-actions">
  GitHub Actions
</h3>

[Claude Code GitHub Actions](/docs/de/github-actions) liest `ANTHROPIC_BASE_URL` und `ANTHROPIC_CUSTOM_HEADERS` aus dem `env`-Block des Workflows. Übergeben Sie die Anmeldedaten als die `anthropic_api_key`-Eingabe der Aktion; die Aktion legt sie als `ANTHROPIC_API_KEY` fest, sodass sie das Gateway im `x-api-key`-Header erreichen.

Für ein `x-api-key`-Gateway legen Sie die Basis-URL in `env` fest und übergeben Sie den Gateway-Schlüssel als Eingabe:

```yaml theme={null}
env:
  ANTHROPIC_BASE_URL: https://llm-gateway.example.com

steps:
  - uses: anthropics/claude-code-action@v1
    with:
      anthropic_api_key: ${{ secrets.GATEWAY_API_KEY }}
```

Für ein Bearer-Token-Gateway übergeben Sie das gleiche Geheimnis sowohl als die `anthropic_api_key`-Eingabe als auch als `ANTHROPIC_AUTH_TOKEN` im Workflow-`env`-Block. Die Aktion erfordert `anthropic_api_key`, `CLAUDE_CODE_OAUTH_TOKEN` oder Workload-Identitäts-Verbund, bevor sie Claude Code startet, und sie liest `ANTHROPIC_AUTH_TOKEN` nicht, daher erfüllt die Eingabe diese Start-Prüfung. Die Env-Variable ist das, was den Schlüssel in den `Authorization`-Header legt, den das Gateway liest; die Kopie in `x-api-key` wird ignoriert:

```yaml theme={null}
env:
  ANTHROPIC_BASE_URL: https://llm-gateway.example.com
  ANTHROPIC_AUTH_TOKEN: ${{ secrets.GATEWAY_API_KEY }}

steps:
  - uses: anthropics/claude-code-action@v1
    with:
      anthropic_api_key: ${{ secrets.GATEWAY_API_KEY }}
```

Für die anderen Authentifizierungsoptionen der Aktion, einschließlich `CLAUDE_CODE_OAUTH_TOKEN` und Workload-Identitäts-Verbund, siehe [Claude Code GitHub Actions](/docs/de/github-actions) und die [README](https://github.com/anthropics/claude-code-action#readme) der Aktion.

<h3 id="agent-sdk">
  Agent SDK
</h3>

Das [Agent SDK](/docs/de/agent-sdk/overview) hat keine Gateway-spezifischen Optionen; es übergibt Umgebungsvariablen an den Claude Code-Prozess, den es erzeugt. Jedes SDK akzeptiert eine `env`-Option, die die Umgebung des erzeugten Prozesses festlegt, und die TypeScript- und Python-SDKs behandeln sie unterschiedlich:

* TypeScript: Der erzeugte Prozess erbt standardmäßig die übergeordnete Umgebung, aber das Festlegen von `options.env` ersetzt die Umgebung vollständig. Verteilen Sie `process.env` darin, um Ihre Gateway-Variablen zu behalten.
* Python: `ClaudeAgentOptions(env=...)` wird auf der geerbten Umgebung zusammengeführt, daher werden Gateway-Variablen, die im übergeordneten Prozess festgelegt sind, ohne Verteilung durchgeleitet.

<CodeGroup>
  ```ts TypeScript theme={null}
  const result = query({
    prompt: "...",
    options: {
      env: {
        ...process.env,
        ANTHROPIC_BASE_URL: "https://llm-gateway.example.com",
        ANTHROPIC_AUTH_TOKEN: process.env.GATEWAY_KEY,
      },
    },
  })
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      env={
          "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com",
          "ANTHROPIC_AUTH_TOKEN": os.environ["GATEWAY_KEY"],
      }
  )
  ```
</CodeGroup>

<h3 id="slack-web-and-remote-control">
  Slack, Web und Remote Control
</h3>

[Claude Code in Slack](/docs/de/slack) und [Claude Code im Web](/docs/de/claude-code-on-the-web) sind von Anthropic gehostete Produkte, die immer die Anthropic-API verwenden; sie sind nicht Teil einer Gateway-Bereitstellung. Gateway-Variablen, die in der Umgebungskonfiguration einer Cloud-Sitzung festgelegt sind, werden nicht angewendet. Wenn Ihr Datenverkehr auf dem Gateway bleiben muss, aktivieren Sie diese Oberflächen nicht für diese Benutzer.

[Remote Control](/docs/de/remote-control) und [Sprachdiktat](/docs/de/voice-dictation) verlassen sich beide auf eine claude.ai-Identität: Remote Control, um eine Live-Sitzung mit Ihrem Konto zu koppeln, und Sprachdiktat, um den claude.ai-Transkriptions-Endpunkt zu erreichen. Sie sind nicht verfügbar, während `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` oder ein `apiKeyHelper` aktiv ist. Ab v2.1.196 ist Remote Control auch deaktiviert, während `ANTHROPIC_BASE_URL` auf einen Nicht-Anthropic-Host verweist, daher ist die Anmeldung mit claude.ai allein nicht ausreichend.

Um eines dieser Features wiederherzustellen, melden Sie sich mit claude.ai an und heben Sie die Festlegung der Gateway-Variablen auf, die es überprüft. Der Remote Control-Abschnitt von `claude doctor` benennt die Anmeldedaten-Variable, die aufgehoben werden soll.

* Sprachdiktat: Heben Sie die Festlegung der Gateway-Anmeldedaten auf
* Remote Control: Heben Sie die Festlegung der Gateway-Anmeldedaten und `ANTHROPIC_BASE_URL` auf

<h2 id="additional-configuration">
  Zusätzliche Konfiguration
</h2>

Diese Einstellungen behandeln Fälle über die Basis-URL und die Anmeldedaten hinaus. Legen Sie sie nur fest, wenn die Anweisungen Ihres Administrators, die Netzwerk-Egress-Regeln oder die [Fehlerbehebungstabelle](#troubleshoot-gateway-errors) eine erfordern.

<h3 id="send-additional-headers">
  Senden Sie zusätzliche Header
</h3>

Einige Gateways leiten oder kennzeichnen Anfragen mit einem benutzerdefinierten Header zusätzlich zur Anmeldedaten, z. B. eine Mandanten-ID oder einen Routing-Schlüssel. Um einen zu senden, legen Sie [`ANTHROPIC_CUSTOM_HEADERS`](/docs/de/env-vars) mit einem `Name: Value`-Paar pro Zeile fest. Das Beispiel unten fügt einen Routing-Header namens `X-Org-Route` hinzu:

<Tabs>
  <Tab title="Bash oder Zsh">
    ```bash theme={null}
    export ANTHROPIC_CUSTOM_HEADERS="X-Org-Route: prod"
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_CUSTOM_HEADERS = "X-Org-Route: prod"
    ```
  </Tab>
</Tabs>

Sie können auch `ANTHROPIC_CUSTOM_HEADERS` im `env`-Block einer Einstellungsdatei festlegen. Verwenden Sie `\n` zwischen Paaren dort, da JSON-Zeichenketten nicht mehrere Zeilen umfassen können:

```json theme={null}
{
  "env": {
    "ANTHROPIC_CUSTOM_HEADERS": "X-Org-Route: prod\nX-Tenant: example"
  }
}
```

<h3 id="add-gateway-models-to-the-model-picker">
  Fügen Sie Gateway-Modelle zum Modellwähler hinzu
</h3>

Die Modellermittlung fragt das Gateway beim Start nach seiner Modellliste ab und fügt diese Namen zum `/model`-Wähler neben den integrierten Einträgen hinzu.

Aktivieren Sie es, wenn Ihr Gateway Modellnamen bereitstellt, die nicht in der integrierten Liste von Claude Code enthalten sind, und Sie diese aus dem Wähler auswählen möchten. Wenn die integrierten Modelle das sind, was Sie verwenden, benötigen Sie keine Ermittlung; Ihr Administrator hat sie möglicherweise auch bereits über verwaltete Einstellungen aktiviert.

Um sie zu aktivieren, legen Sie `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1` in Ihrer Shell oder im `env`-Block von `~/.claude/settings.json` fest. Die Ermittlung erfordert Claude Code v2.1.129 oder später.&#x20;

Ermittelte Modelle werden als zusätzliche `/model`-Einträge mit der Bezeichnung `From gateway` angezeigt. Um zu bestätigen, dass die Ermittlung ausgeführt wurde, starten Sie `claude --debug` und suchen Sie nach den `[gatewayDiscovery]`-Zeilen: Ein Erfolg protokolliert, wie viele Modelle zwischengespeichert wurden, und ein `404`, Timeout oder Umleitung wird dort ebenfalls aufgezeichnet. Für den Zeitpunkt der Ermittlung, was sie filtert, und das Antwortformat, das Gateways bereitstellen, siehe die [Modellermittlungs-Referenz](/docs/de/llm-gateway-protocol#model-discovery).

<h3 id="rotate-credentials-with-apikeyhelper">
  Rotieren Sie Anmeldedaten mit apiKeyHelper
</h3>

Ein `apiKeyHelper` ist ein Befehl, den Claude Code ausführt, um Ihre Gateway-Anmeldedaten abzurufen, anstatt sie aus einer statischen Umgebungsvariablen zu lesen.

Verwenden Sie einen Helper, wenn die Anmeldedaten nach einem Zeitplan ablaufen, aus einem Tresor oder SSO-Befehl stammen, oder Ihr Administrator Ihnen mitgeteilt hat, einen zu konfigurieren. Wenn Ihre Anmeldedaten eine feste Zeichenkette sind, die Sie einmal festlegen, ist die [Anmeldedaten-Variable](#set-the-credential-variable) alles, was Sie benötigen, und Sie können diesen Abschnitt überspringen.

Der Helper ist ein beliebiger Shell-Befehl, der die aktuelle Anmeldedaten auf stdout ausgibt. Claude Code führt ihn durch Ihre System-Shell aus, daher kann er unter Windows eine ausführbare Datei oder eine PowerShell-Invokation sein. Schreiben Sie das Skript, machen Sie es ausführbar, und verweisen Sie darauf von `apiKeyHelper` in Ihrer [Einstellungsdatei](/docs/de/settings):

<Tabs>
  <Tab title="Bash oder Zsh">
    Zum Beispiel ein Skript, das aus einem Tresor liest:

    ```bash theme={null}
    #!/bin/bash
    vault kv get -field=api_key secret/llm-gateway/claude-code
    ```

    Verweisen Sie auf seinen Pfad in `~/.claude/settings.json`:

    ```json theme={null}
    {
      "apiKeyHelper": "~/bin/get-gateway-key.sh"
    }
    ```
  </Tab>

  <Tab title="PowerShell">
    Zum Beispiel ein Skript, das aus einem Tresor liest:

    ```powershell theme={null}
    vault kv get -field=api_key secret/llm-gateway/claude-code
    ```

    Verweisen Sie auf die PowerShell-Invokation in `%USERPROFILE%\.claude\settings.json`, wobei Sie die Backslashes in der JSON-Zeichenkette escapen:

    ```json theme={null}
    {
      "apiKeyHelper": "powershell -NoProfile -File C:\\scripts\\get-gateway-key.ps1"
    }
    ```
  </Tab>
</Tabs>

Claude Code speichert die Ausgabe des Helpers standardmäßig fünf Minuten lang zwischen und führt ihn erneut aus, wenn eine Anfrage HTTP 401 zurückgibt. Um die Lebensdauer des Caches zu ändern, legen Sie `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` in Millisekunden fest, z. B. `CLAUDE_CODE_API_KEY_HELPER_TTL_MS=900000` für 15 Minuten.

Der Wert des Helpers wird in beiden `Authorization`- und `x-api-key`-Headern gesendet, daher funktioniert er, welcher Header Ihr Gateway auch liest.

<h3 id="turn-off-traffic-outside-the-gateway-path">
  Schalten Sie den Datenverkehr außerhalb des Gateway-Pfads aus
</h3>

Das Gateway trägt Modellanfragen, aber Claude Code sendet auch nicht wesentlichen Hintergrund-Datenverkehr außerhalb des Gateway-Pfads an Anthropic und an Drittanbieter-Services wie GitHub: Versionsüberprüfungen, Telemetrie, Fehlerberichte, Versionshinweise und ähnliche Anfragen. In einem Netzwerk, das nur Egress zum Gateway zulässt, schlagen diese Anfragen fehl und können als blockierte Verbindungen in Ihrer Egress-Überwachung angezeigt werden.

Um diesen Datenverkehr auszuschalten, legen Sie `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` neben den Gateway-Variablen fest, im gleichen Shell-Export oder im `env`-Block der Einstellungsdatei:

<Tabs>
  <Tab title="Bash oder Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC = "1"
    ```
  </Tab>
</Tabs>

Das Festlegen der Variablen hat diese Auswirkungen und Einschränkungen:

* Es deaktiviert automatische Updates, daher planen Sie einen anderen Update-Pfad, z. B. Ihren Paketmanager oder verwaltete Verteilung.
* Es unterdrückt die [Schnellmodus](/docs/de/fast-mode)-Verfügbarkeitsprüfung. Sofern eine vorherige Prüfung den Schnellmodus auf dem Computer nicht bereits aktiviert hat, meldet `/fast`, dass der Schnellmodus nicht verfügbar ist.
* Es schaltet die [Gateway-Modellermittlung](#add-gateway-models-to-the-model-picker) aus, obwohl die Ermittlung das Gateway selbst abfragt. Zuvor ermittelte Modelle bleiben aus dem lokalen Cache verfügbar, aber die Liste wird nicht aktualisiert.
* Die [Domain-Sicherheitsprüfung](/docs/de/data-usage#webfetch-domain-safety-check) des WebFetch-Tools ist nicht betroffen und ruft weiterhin `api.anthropic.com` auf. Schalten Sie sie separat mit `skipWebFetchPreflight: true` in [Einstellungen](/docs/de/settings) aus, wenn Ihr Netzwerk diesen Host blockiert.
* Für jeden Telemetrie-Stream und die Variable, die ihn steuert, siehe [Telemetrie-Services](/docs/de/data-usage#telemetry-services).

<h3 id="route-to-a-cloud-provider-through-a-gateway">
  Leiten Sie zu einem Cloud-Anbieter über ein Gateway weiter
</h3>

Diese Konfigurationen richten Claude Code auf ein Gateway über eine anbieter-spezifische Basis-URL-Variable anstelle von `ANTHROPIC_BASE_URL` aus. Amazon Bedrock und Google Cloud's Agent Platform-Gateways akzeptieren die nativen Anforderungsformate dieser Anbieter; Microsoft Foundry und Claude Platform on AWS-Gateways akzeptieren das Anthropic Messages-Format und unterscheiden sich nur in der Basis-URL-Variable, die sie erreicht.

Verwenden Sie eine nur, wenn Ihr Gateway-Team speziell Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry oder die Claude Platform on AWS benannt hat. Wenn die [Überprüfungsanfrage](#verify-the-connection) oben JSON zurückgegeben hat, können Sie diesen Abschnitt überspringen.

Legen Sie den Block für den Anbieter fest, den Ihr Gateway-Team benannt hat. Die Skip-Auth-Variablen teilen Claude Code mit, dass Anfragen nicht mit Anbieter-Anmeldedaten signiert werden sollen, da das Gateway diese hält. Wenn das Gateway sein eigenes Token benötigt, fügen Sie `ANTHROPIC_AUTH_TOKEN` nach dem Block hinzu, außer für Microsoft Foundry, das `ANTHROPIC_FOUNDRY_API_KEY` wie gezeigt verwendet. Ein Microsoft Foundry-Gateway, das einen Bearer-Token erwartet, kann stattdessen [`ANTHROPIC_FOUNDRY_AUTH_TOKEN`](/docs/de/env-vars) verwenden; es hat Vorrang vor `ANTHROPIC_FOUNDRY_API_KEY`, wenn beide gesetzt sind. `ANTHROPIC_FOUNDRY_AUTH_TOKEN` erfordert Claude Code v2.1.203 oder später.

<h4 id="amazon-bedrock">
  Amazon Bedrock
</h4>

<Tabs>
  <Tab title="Bash oder Zsh">
    ```bash theme={null}
    export ANTHROPIC_BEDROCK_BASE_URL=https://llm-gateway.example.com/bedrock
    export CLAUDE_CODE_SKIP_BEDROCK_AUTH=1
    export CLAUDE_CODE_USE_BEDROCK=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_BEDROCK_BASE_URL = "https://llm-gateway.example.com/bedrock"
    $env:CLAUDE_CODE_SKIP_BEDROCK_AUTH = "1"
    $env:CLAUDE_CODE_USE_BEDROCK = "1"
    ```
  </Tab>
</Tabs>

<h4 id="google-cloud’s-agent-platform">
  Google Cloud's Agent Platform
</h4>

<Tabs>
  <Tab title="Bash oder Zsh">
    ```bash theme={null}
    export ANTHROPIC_VERTEX_BASE_URL=https://llm-gateway.example.com/vertex
    export ANTHROPIC_VERTEX_PROJECT_ID=your-gcp-project-id
    export CLAUDE_CODE_SKIP_VERTEX_AUTH=1
    export CLAUDE_CODE_USE_VERTEX=1
    export CLOUD_ML_REGION=us-east5
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_VERTEX_BASE_URL = "https://llm-gateway.example.com/vertex"
    $env:ANTHROPIC_VERTEX_PROJECT_ID = "your-gcp-project-id"
    $env:CLAUDE_CODE_SKIP_VERTEX_AUTH = "1"
    $env:CLAUDE_CODE_USE_VERTEX = "1"
    $env:CLOUD_ML_REGION = "us-east5"
    ```
  </Tab>
</Tabs>

<h4 id="microsoft-foundry">
  Microsoft Foundry
</h4>

Legen Sie die Anmeldedaten des Gateways in `ANTHROPIC_FOUNDRY_API_KEY` fest; sie werden an das Gateway als `x-api-key`-Header gesendet. Ein Gateway, das einen Bearer-Token erwartet, kann stattdessen [`ANTHROPIC_FOUNDRY_AUTH_TOKEN`](/docs/de/env-vars) verwenden. Claude Code sendet diesen Wert als `Authorization: Bearer`-Header, und er hat Vorrang vor `ANTHROPIC_FOUNDRY_API_KEY`, wenn beide gesetzt sind. Erfordert Claude Code v2.1.203 oder später.

Für ein Gateway, das seinen eigenen `Authorization`-Header injiziert, legen Sie `CLAUDE_CODE_SKIP_FOUNDRY_AUTH=1` fest und lassen Sie beide Anmeldedaten-Variablen ungesetzt. Claude Code sendet dann Anfragen ohne eine Azure-Anmeldedaten und bewahrt den `Authorization`-Header, den Sie bereitstellen, z. B. durch `ANTHROPIC_CUSTOM_HEADERS`. Vor v2.1.203 ließ `CLAUDE_CODE_SKIP_FOUNDRY_AUTH` ohne einen API-Schlüssel den Microsoft Foundry-Client unfähig, Anfragen zu senden.

<Tabs>
  <Tab title="Bash oder Zsh">
    ```bash theme={null}
    export ANTHROPIC_FOUNDRY_BASE_URL=https://llm-gateway.example.com/foundry
    export ANTHROPIC_FOUNDRY_API_KEY=sk-gateway-key
    export CLAUDE_CODE_USE_FOUNDRY=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_FOUNDRY_BASE_URL = "https://llm-gateway.example.com/foundry"
    $env:ANTHROPIC_FOUNDRY_API_KEY = "sk-gateway-key"
    $env:CLAUDE_CODE_USE_FOUNDRY = "1"
    ```
  </Tab>
</Tabs>

<h4 id="claude-platform-on-aws">
  Claude Platform on AWS
</h4>

Siehe [Claude Platform on AWS](/docs/de/claude-platform-on-aws) für die Workspace-ID.

<Tabs>
  <Tab title="Bash oder Zsh">
    ```bash theme={null}
    export ANTHROPIC_AWS_BASE_URL=https://llm-gateway.example.com/anthropic-aws
    export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
    export CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1
    export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_AWS_BASE_URL = "https://llm-gateway.example.com/anthropic-aws"
    $env:ANTHROPIC_AWS_WORKSPACE_ID = "wrkspc_01ABCDEFGHIJKLMN"
    $env:CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH = "1"
    $env:CLAUDE_CODE_USE_ANTHROPIC_AWS = "1"
    ```
  </Tab>
</Tabs>

<h2 id="troubleshoot-gateway-errors">
  Fehlerbehebung bei Gateway-Fehlern
</h2>

Dies sind die häufigsten Fehler beim Ausführen von Claude Code über ein Gateway, mit der Gateway-seitigen Ursache und der Behebung:

| Fehler                                                                                                                                                                                                                  | Ursache                                                                                                                                                                                                                                                                                                                                      | Behebung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Eine Start-Warnung, die zwei Anmeldedaten-Quellen benennt und mit `auth may not work as expected` endet. Ältere Versionen zeigen stattdessen `Auth conflict: Both a token (SOURCE) and an API key (SOURCE) are set` an. | Eine Gateway-Anmeldedaten und ein gespeicherter Login sind beide aktiv; die Variable wird für Anfragen verwendet, aber der veraltete Login kann zu unerwartetem Auth-Verhalten führen                                                                                                                                                        | Heben Sie die Festlegung der Variable auf, um den gespeicherten Login zu verwenden, oder führen Sie `/logout` aus, um die Gateway-Anmeldedaten zu verwenden                                                                                                                                                                                                                                                                                                                               |
| `401`-Fehler, die ein ungültiges oder nicht erkanntes Token benennen                                                                                                                                                    | Die Anmeldedaten sind nicht eine, die das Gateway ausgestellt hat, oder sie sind in einem Header, den das Gateway nicht liest                                                                                                                                                                                                                | Bestätigen Sie, dass die Variable Ihrer Anmeldedaten-Art in der [Anmeldedaten-Tabelle](#set-the-credential-variable) entspricht, und generieren Sie den Schlüssel beim Gateway neu, wenn er widerrufen wurde                                                                                                                                                                                                                                                                              |
| `Your apiKeyHelper script is failing`                                                                                                                                                                                   | Der Befehl in der [`apiKeyHelper`](/docs/de/settings#available-settings)-Einstellung ist mit einem Fehler beendet worden, hat das Zeitlimit überschritten oder nichts ausgegeben, daher enthalten Anfragen einen Platzhalter-Schlüssel                                                                                                            | Führen Sie den Befehl direkt aus, um zu sehen, warum er fehlschlägt, und authentifizieren Sie sich erneut bei Ihrem Anmeldedaten-Anbieter, wenn dieser eine abgelaufene Sitzung meldet; siehe [die Fehlerreferenz](/docs/de/errors#your-apikeyhelper-script-is-failing)                                                                                                                                                                                                                        |
| `Unable to connect to API (ConnectionRefused)`, oder `(ECONNREFUSED)` von npm-Installationen, oft nach einer stillen Pause, während Claude Code [mit Backoff erneut versucht](/docs/de/errors#automatic-retries)             | Nichts antwortete unter der Basis-URL: Die Adresse ist falsch, oder ein VPN oder eine Firewall blockiert den Pfad zum Gateway                                                                                                                                                                                                                | Führen Sie den [curl-Test oben](#verify-the-connection) aus, der sofort mit der gleichen Ursache fehlschlägt, und bestätigen Sie die URL und den Netzwerkpfad mit Ihrem Gateway-Team                                                                                                                                                                                                                                                                                                      |
| `API returned an empty or malformed response (HTTP 200)`                                                                                                                                                                | Das Gateway oder ein zwischengelagerter Proxy gab eine Nicht-API-Antwort zurück, oft eine HTML-Fehler oder Anmeldeseite                                                                                                                                                                                                                      | Testen Sie mit der [curl-Anfrage oben](#verify-the-connection); beheben Sie die Gateway-Route, die Nicht-JSON zurückgibt                                                                                                                                                                                                                                                                                                                                                                  |
| `400`-Fehler, die `context_management`, `Extra inputs are not permitted` oder andere nicht erkannte Felder benennen                                                                                                     | Das Gateway leitet Anfragen an einen Upstream weiter, der Felder ablehnt, die Claude Code an Anthropic-Format-Endpunkte sendet                                                                                                                                                                                                               | Legen Sie `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` fest, das die meisten Pre-Release-Felder unterdrückt; siehe [Feature-Durchleitung](/docs/de/llm-gateway-protocol#feature-pass-through). Einige Betas werden nicht durch dieses Flag gated; für diese legen Sie die passende `CLAUDE_CODE_USE_*`-Anbieter-Variable fest, damit Claude Code nur das sendet, das dieser Anbieter akzeptiert                                                                                                  |
| `400`-Fehler, die `thinking` oder `adaptive` benennen, z. B. `Input tag 'adaptive' found`                                                                                                                               | Der Upstream-Modell-Build akzeptiert keine adaptive Überlegung, die Claude Code für Claude 4.6 und neuere Modelle anfordert                                                                                                                                                                                                                  | Aktualisieren Sie den Upstream des Gateways. Auf Opus 4.6 und Sonnet 4.6 funktioniert stattdessen `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1`. Die [Modellkonfiguration](/docs/de/model-config)-Fähigkeitsvariablen gelten nur für die Anbieter-Konfigurationen, z. B. `CLAUDE_CODE_USE_BEDROCK` und `CLAUDE_CODE_USE_VERTEX`, nicht hinter einem `ANTHROPIC_BASE_URL`-Gateway                                                                                                                   |
| `400`-Fehler, die einen Kontext- oder Token-Limit in den eigenen Worten des Gateways angeben, z. B. `ContextWindowExceededError` oder `prompt token count of N exceeds the limit of M`                                  | Das Gateway erzwingt ein kleineres Kontext als das native Fenster des Modells und schreibt den Upstream-Fehler um, daher wird die automatische Komprimierung und Wiederholung, die Anthropics `prompt is too long`-Wortlaut entspricht, nicht ausgelöst                                                                                      | Führen Sie `/compact` aus, um die Sitzung wiederherzustellen. Um dies zu verhindern, legen Sie `CLAUDE_CODE_AUTO_COMPACT_WINDOW` auf das Limit des Gateways fest; der Wert wird auf mindestens 100.000 Token und höchstens das Kontext-Fenster des Modells begrenzt, daher kann ein Gateway-Limit unter 100.000 nicht abgeglichen werden und `/compact` bleibt die Wiederherstellung dort. Legen Sie auch `CLAUDE_CODE_MAX_OUTPUT_TOKENS` unter das Output-Limit des Gateway-Modells fest |
| Modelle fehlen im `/model`-Wähler                                                                                                                                                                                       | Gateway-Modellnamen sind nicht in der integrierten Liste von Claude Code enthalten                                                                                                                                                                                                                                                           | Aktivieren Sie die [Gateway-Modellermittlung](#add-gateway-models-to-the-model-picker) oder fügen Sie Namen mit den [Modellkonfiguration](/docs/de/model-config)-Variablen hinzu                                                                                                                                                                                                                                                                                                               |
| Claude Code fordert Sie auf, sich anzumelden, obwohl der [curl-Test](#verify-the-connection) erfolgreich ist                                                                                                            | Die CLI hat keine eigene Anmeldedaten: Eine erreichbare Basis-URL ist keine, und ein `env`-Block in einer Projekt-`.claude/settings.json` oder `.claude/settings.local.json` gilt nur nach dem First-Run-Wizard und der Vertrauensaufforderung                                                                                               | Legen Sie `ANTHROPIC_AUTH_TOKEN` irgendwo fest, wo Claude Code vor dem First-Run-Setup liest: ein Shell-Export, der `env`-Block in `~/.claude/settings.json` oder verwaltete Einstellungen                                                                                                                                                                                                                                                                                                |
| `ANTHROPIC_API_KEY` ist festgelegt, wird aber ignoriert, ohne Aufforderung                                                                                                                                              | Der Schlüssel benötigt eine einmalige Genehmigung in interaktiven Sitzungen, und ein zuvor abgelehnter Schlüssel wird ohne erneute Frage ignoriert                                                                                                                                                                                           | Aktivieren Sie ihn unter `/config` mit der Option `Use custom API key`                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `This machine's managed settings require a first-party login`                                                                                                                                                           | Verwaltete Einstellungen enthalten `forceLoginMethod` oder `forceLoginOrgUUID`, die auf Claude Code v2.1.146 und später nicht mit `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` oder `apiKeyHelper` koexistieren können                                                                                                                        | Ihr Administrator muss `forceLoginMethod` und `forceLoginOrgUUID` aus verwalteten Einstellungen entfernen, um Gateway-Anmeldedaten zu verwenden, oder die Gateway-Anmeldedaten entfernen, um First-Party-Login zu verwenden. Die beiden können nicht kombiniert werden                                                                                                                                                                                                                    |
| `403` mit einem HTML-Body wie `403 Forbidden`, wenn die eigenen Logs des Gateways zeigen, dass keine Anfrage empfangen wurde                                                                                            | Eine Web-Anwendungs-Firewall oder ein Reverse-Proxy vor dem Gateway blockierte den Anfrage-Body, bevor er das Gateway erreichte. Claude Code-Eingabeaufforderungen enthalten XML-ähnliche Tags und Quellcode, die Cross-Site-Scripting-Body-Regeln entsprechen, daher besteht ein kurzer curl-Test, während eine echte Sitzung nicht besteht | Befreien Sie den `/v1/messages`-Pfad des Gateways von der Anfrage-Body-Inspektion. Auf AWS WAF ist dies die verwaltete Regel `CrossSiteScripting_Body`; auf nginx mit ModSecurity sind dies die äquivalenten OWASP CRS-Body-Regeln                                                                                                                                                                                                                                                        |
| Zertifikat- oder TLS-Fehler wie `SSL certificate verification failed` oder `Self-signed certificate detected`, wenn der [curl-Test](#verify-the-connection) erfolgreich ist                                             | Die Runtime von Claude Code vertraut nicht der gleichen Zertifizierungsstelle, die `curl` verwendet. Häufig hinter Unternehmens-TLS-Inspektions-Proxys                                                                                                                                                                                       | Legen Sie `NODE_EXTRA_CA_CERTS` auf den CA-Bundle-Pfad fest; siehe [CA-Zertifikat-Speicher](/docs/de/network-config#ca-certificate-store)                                                                                                                                                                                                                                                                                                                                                      |

Wenn Claude Code Sie nach dem Entfernen der Gateway-Konfiguration wiederholt auffordert, sich anzumelden, ist die Ursache normalerweise Anmeldedaten-Speicherung statt des Gateways; siehe [Authentifizierungsfehler](/docs/de/errors#authentication-errors).

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [LLM-Gateways-Übersicht](/docs/de/llm-gateway): Was ein Gateway ist und wie es mit claude.ai-Abonnements interagiert
* [Rollout eines LLM-Gateways für Ihre Organisation](/docs/de/llm-gateway-rollout): Die Admin-seitige Checkliste für die Bereitstellung und Verteilung der Gateway-Konfiguration
* [Gateway-Protokoll-Referenz](/docs/de/llm-gateway-protocol): Was Claude Code an ein Gateway sendet, einschließlich der Header und Felder, die das Gateway weiterleiten muss
* [Einstellungen](/docs/de/settings): Wo Einstellungsdateien leben und wie der `env`-Block gelesen wird
* [Authentifizierung](/docs/de/authentication): Wie Anmeldedaten-Variablen, `apiKeyHelper` und OAuth-Login interagieren
