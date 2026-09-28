> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitHub Actions

> Führen Sie Claude Code in GitHub Actions-Workflows aus, um auf @claude-Erwähnungen zu reagieren, Aufgaben zu automatisieren und Issues in Pull Requests umzuwandeln

[Claude Code GitHub Actions](https://github.com/anthropics/claude-code-action) ist eine GitHub Action, die Claude Code in den Workflows Ihres Repositories ausführt. Erwähnen Sie `@claude` in einem Pull-Request- oder Issue-Kommentar, damit Claude Code analysiert, Änderungen implementiert und Commits pusht. Sie können der Claude Code GitHub Action auch einen Prompt geben, um automatisch bei jedem GitHub-Event ausgeführt zu werden. Verwenden Sie sie, um Issues in Pull Requests umzuwandeln, Bugs aus einem Kommentar zu beheben oder wiederkehrende Aufgaben zu automatisieren.

Mehrere Produkte teilen den Namen Claude Code. Diese Seite behandelt die `claude-code-action`-Workflow-Integration, die Sie mit Workflow-Dateien in Ihrem Repository konfigurieren. Für die verwandten Produkte siehe:

* [Code Review](/docs/de/code-review): automatische Überprüfung bei jedem Pull Request, ohne einen Workflow zu schreiben
* [Claude Code in der Cloud](/docs/de/claude-code-on-the-web): Claude Code-Sitzungen, die auf Cloud-Infrastruktur statt auf Ihrem Computer ausgeführt werden
* [Claude Agent SDK](/docs/de/agent-sdk/overview): benutzerdefinierte Automatisierung außerhalb von GitHub Actions. Die Claude Code GitHub Action basiert auf dem SDK
* [GitHub Enterprise Server](/docs/de/github-enterprise-server): Claude Code mit selbstgehostetem GitHub

<h2 id="setup">
  Setup
</h2>

Sie können die Claude Code GitHub Action auf eine von zwei Arten einrichten:

* **Schnelles Setup**: Führen Sie `/install-github-app` aus Claude Code aus. Claude Code installiert die GitHub App, fügt Ihr Authentifizierungsgeheimnis hinzu und bereitet den Workflow-Pull-Request für Sie vor
* **Manuelles Setup**: Installieren Sie die App, fügen Sie das Geheimnis hinzu und kopieren Sie die Workflow-Datei selbst in Ihr Repository. Verwenden Sie diesen Weg, wenn Sie Claude Code nicht lokal ausführen, wenn der Befehl fehlschlägt oder wenn Sie vollständige Kontrolle über die Workflow-Dateien möchten

Für beide Wege benötigen Sie Admin-Zugriff auf das Repository.

<h3 id="quick-setup">
  Schnelles Setup
</h3>

`/install-github-app` funktioniert nur mit github.com-Repositories. Wenn sich das Git-Remote Ihres Repositories auf gitlab.com oder bitbucket.org befindet, druckt der Befehl eine Benachrichtigung aus und beendet sich, anstatt das Setup zu starten. Um Claude Code aus GitLab-Pipelines auszuführen, siehe [Claude Code GitLab CI/CD](/docs/de/gitlab-ci-cd).

Bevor Sie beginnen, installieren Sie die [GitHub CLI](https://cli.github.com) und authentifizieren Sie sie mit `gh auth login`. Claude Code prüft darauf und warnt Sie, wenn sie fehlt.

Öffnen Sie `claude` im Repository, das Sie verbinden möchten, führen Sie `/install-github-app` aus und folgen Sie den Aufforderungen. Claude Code installiert die Claude GitHub App und richtet dann ein Authentifizierungsgeheimnis für die Workflows ein:

* Wenn Claude Code bereits einen API-Schlüssel hat, verwendet es diesen Schlüssel wieder und bietet an, das vorhandene `ANTHROPIC_API_KEY`-Geheimnis des Repositories zu behalten, falls bereits eines gesetzt ist
* Andernfalls wählen Sie zwischen dem Erstellen eines langlebigen Tokens mit Ihrem Claude-Abonnement und dem Einfügen eines API-Schlüssels

Claude Code speichert die Anmeldedaten als Repository-Geheimnis, benannt `ANTHROPIC_API_KEY` für einen API-Schlüssel oder `CLAUDE_CODE_OAUTH_TOKEN` für ein Abonnement-Token.

Claude Code pusht dann einen Branch mit den von Ihnen ausgewählten Workflow-Dateien, bereits so eingestellt, dass das Geheimnis verwendet wird, und öffnet GitHub in Ihrem Browser mit einem Pull Request, der bereit zum Erstellen ist. Erstellen und mergen Sie diesen Pull Request, und `@claude` funktioniert im Repository.

Wenn Sie den Review-Workflow auswählen, postet Claude jede Überprüfung auf dem Pull Request selbst, als Inline-Kommentar zu jedem gefundenen Problem oder als einen Zusammenfassungs-Kommentar, wenn keine gefunden werden. Claude überspringt einige Pull Requests, wie Entwürfe. Das [Review-Workflow-Beispiel](#run-a-skill) verwendet die gleiche Skill und listet sie auf. Vor v2.1.229 schrieb Claude seine Überprüfung nur in das Workflow-Run-Log.

Um einen Review-Workflow zu aktualisieren, den eine frühere Version generiert hat, führen Sie eines der folgenden Verfahren durch:

* Führen Sie `/install-github-app` erneut aus. Wenn das Repository bereits eine `claude.yml` hat, wählen Sie **Workflow-Datei mit neuester Version aktualisieren**. Claude Code pusht frische Kopien der Workflow-Dateien zu einem neuen Branch und öffnet den Pull Request, genauso wie eine erste Installation.
* Fügen Sie das `--comment`-Argument und die `claude_args`-Zeile aus dem [Review-Workflow-Beispiel](#run-a-skill) selbst zur eingecheckten Datei hinzu, was alle anderen Änderungen beibehält, die Sie daran vorgenommen haben.

Nach der Installation der GitHub App fragt Claude Code, ob Sie mit dem GitHub Actions-Setup fortfahren möchten. Wählen Sie **Jetzt überspringen**, um nur mit der installierten GitHub App zu stoppen. Führen Sie `/install-github-app` später erneut aus, um die Workflow- und Secret-Schritte abzuschließen.

<Note>
  * Wenn Sie die GitHub App installieren, gewähren Sie ihr mehrere Berechtigungen. Siehe [GitHub App-Berechtigungen](#github-app-permissions) für den vollständigen Satz
  * Schnelles Setup funktioniert mit der Claude API und Claude-Abonnements. Wenn Sie Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry verwenden, siehe [Claude Code GitHub Actions mit Cloud-Providern verwenden](/docs/de/github-actions-cloud-providers)
</Note>

<h3 id="manual-setup">
  Manuelles Setup
</h3>

Um die Claude Code GitHub Action ohne Ausführung von `/install-github-app` zu konfigurieren, installieren Sie die App, fügen Sie ein Geheimnis hinzu und kopieren Sie selbst eine Workflow-Datei:

<Steps>
  <Step title="Installieren Sie die Claude GitHub App">
    Installieren Sie die [Claude GitHub App](https://github.com/apps/claude) in Ihrem Repository. Die Claude Code GitHub Action basiert auf drei der App-Berechtigungen:

    * **Contents**: Lesen und Schreiben, damit Claude Repository-Dateien ändern kann
    * **Issues**: Lesen und Schreiben, damit Claude auf Issues antworten kann
    * **Pull requests**: Lesen und Schreiben, damit Claude PRs erstellen und Änderungen pushen kann

    Während der Installation gewähren Sie auch Berechtigungen, die andere Claude-Funktionen verwenden. Siehe [GitHub App-Berechtigungen](#github-app-permissions) für den vollständigen Satz.
  </Step>

  <Step title="Fügen Sie ein Authentifizierungsgeheimnis hinzu">
    Fügen Sie eines der folgenden Geheimnisse zu Ihrem Repository hinzu, je nachdem, wie Sie sich authentifizieren. Siehe Githubs Anleitung zum [Verwenden von Geheimnissen in GitHub Actions](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions).

    * `ANTHROPIC_API_KEY`: ein Claude API-Schlüssel aus der [Claude Console](https://platform.claude.com)
    * `CLAUDE_CODE_OAUTH_TOKEN`: ein OAuth-Token, das sich mit Ihrem Claude-Abonnement authentifiziert, verfügbar in Pro-, Max-, Team- und Enterprise-Plänen. Generieren Sie einen, indem Sie lokal `claude setup-token` ausführen. Siehe [Generieren Sie ein langlebiges Token](/docs/de/authentication#generate-a-long-lived-token)

    Übergeben Sie in Workflow-Dateien das Geheimnis an die entsprechende Eingabe: `anthropic_api_key` für einen API-Schlüssel oder `claude_code_oauth_token` für ein OAuth-Token.
  </Step>

  <Step title="Kopieren Sie die Workflow-Datei">
    Kopieren Sie [examples/claude.yml](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml) in das Verzeichnis `.github/workflows/` Ihres Repositories. Die Datei ist ein funktionierender Workflow, nicht nur ein Beispiel. Wie eingecheckt, antwortet Claude, wenn jemand `@claude` in einem Issue oder Pull Request erwähnt, authentifiziert mit dem `ANTHROPIC_API_KEY`-Geheimnis. Wenn Sie stattdessen `CLAUDE_CODE_OAUTH_TOKEN` hinzugefügt haben, ändern Sie die `anthropic_api_key`-Zeile des Workflows zu `claude_code_oauth_token: ${{ secrets.CLAUDE_CODE_OAUTH_TOKEN }}`.
  </Step>
</Steps>

<Tip>
  Nach dem Setup testen Sie die Claude Code GitHub Action, indem Sie `@claude` in einem Issue- oder PR-Kommentar markieren.
</Tip>

<h3 id="set-up-for-an-organization">
  Setup für eine Organisation
</h3>

Mit schnellem Setup oder manuellem Setup konfigurieren Sie jeweils ein Repository. Um die Claude Code GitHub Action in einer Organisation auszurollen:

* Installieren Sie die [Claude GitHub App](https://github.com/apps/claude) einmal auf Organisationsebene und wählen Sie alle Repositories oder eine ausgewählte Liste
* Speichern Sie das Authentifizierungsgeheimnis als Geheimnis auf Organisationsebene für Actions, damit jedes Repository nicht seine eigene Kopie benötigt
* Fügen Sie die Workflow-Datei zu jedem Repository hinzu, das die Claude Code GitHub Action ausführen soll, oder definieren Sie den Job einmal als [wiederverwendbaren Workflow](https://docs.github.com/en/actions/using-workflows/reusing-workflows), den jedes Repository aufruft

Für ein Geheimnis, das über Repositories hinweg geteilt wird, authentifizieren Sie sich mit einem API-Schlüssel aus der [Claude Console](https://platform.claude.com) statt mit einem OAuth-Token, da ein OAuth-Token an das Abonnement der Person gebunden ist, die `claude setup-token` ausgeführt hat.

Um ein langlebiges Geheimnis ganz zu vermeiden, authentifizieren Sie sich durch Workload Identity Federation, wobei die Claude Code GitHub Action das GitHub OpenID Connect (OIDC) Token des Workflows gegen Claude API-Zugriff durch ein Claude Console-Dienstkonto austauscht. Legen Sie diese Eingaben fest:

* `anthropic_federation_rule_id`: die Verbund-Regel-ID, `fdrl_...`
* `anthropic_organization_id`: Ihre Anthropic-Organisations-ID
* `anthropic_service_account_id`: die Dienstkonto-ID, `svac_...`. Optional, da die Verbund-Regel, die Sie in der Console erstellen, bereits ein Dienstkonto anvisiert
* `anthropic_workspace_id`: die Workspace-ID, `wrkspc_...`. Optional, wenn die Verbund-Regel ein einzelnes Workspace anvisiert

Gewähren Sie dem Workflow die Berechtigung `id-token: write`, die die Claude Code GitHub Action für den Verbund-Austausch benötigt, auch wenn Sie Ihr eigenes `github_token` übergeben. Siehe die [Claude Code GitHub Action-Setup-Anleitung](https://github.com/anthropics/claude-code-action/blob/main/docs/setup.md) für die Console-seitige Konfiguration.

Für Fragen zur Datenbehandlung und -aufbewahrung in einer Sicherheitsüberprüfung siehe [Datennutzung](/docs/de/data-usage) und [Sicherheit](/docs/de/security).

<h3 id="uninstall">
  Deinstallieren
</h3>

Um die Claude Code GitHub Action zu entfernen, machen Sie jeden Teil des Setups rückgängig, der auf Ihre Installation zutrifft:

* **Workflow-Dateien**: Löschen Sie die Workflows, die `anthropics/claude-code-action` aus `.github/workflows/` verwenden. Wenn Sie schnelles Setup verwendet haben, suchen Sie nach `claude.yml` und, wenn Sie den Review-Workflow ausgewählt haben, `claude-code-review.yml`. Mit den gelöschten Workflows wird die Claude Code GitHub Action nicht mehr ausgeführt
* **Geheimnisse**: Löschen Sie das `ANTHROPIC_API_KEY`- oder `CLAUDE_CODE_OAUTH_TOKEN`-Geheimnis aus dem Repository und aus Geheimnissen auf Organisationsebene für Actions, wenn Sie es [über Repositories hinweg geteilt haben](#set-up-for-an-organization). Wenn Sie ein Geheimnis löschen, bleibt die Anmeldedaten, die es hielt, gültig. Um einen API-Schlüssel vollständig zu deaktivieren, löschen Sie auch den Schlüssel in der [Claude Console](https://platform.claude.com)
* **GitHub App**: Deinstallieren Sie die Claude GitHub App in Ihren Repository- oder Organisationseinstellungen unter GitHub Apps, aber nur, wenn Sie sie nicht für eine andere Claude-Funktion verwenden, wie Code Review oder Web-Auto-Fix

Wenn Sie einen [Cloud-Provider](/docs/de/github-actions-cloud-providers) konfiguriert haben, löschen Sie auch die Provider-Geheimnisse, wie `AWS_ROLE_TO_ASSUME`, die `GCP_*`-Geheimnisse oder die `AZURE_*`-Geheimnisse, und deinstallieren Sie die benutzerdefinierte GitHub App zusammen mit ihren `APP_ID`- und `APP_PRIVATE_KEY`-Geheimnissen.

<h3 id="github-app-permissions">
  GitHub App-Berechtigungen
</h3>

Die [Claude GitHub App](https://github.com/apps/claude) wird von jeder Claude-Funktion geteilt, die sich mit GitHub integriert, einschließlich der Claude Code GitHub Action, [Code Review](/docs/de/code-review) und [Auto-Fix für Pull Requests](/docs/de/claude-code-on-the-web#auto-fix-pull-requests) in Cloud-Sitzungen. Eine GitHub App hat einen einzelnen Berechtigungssatz, der alle ihre Funktionen abdeckt, daher enthält der Satz einige Berechtigungen, die die Claude Code GitHub Action nicht verwendet.

Wenn Sie die App installieren, gewähren Sie die folgenden Berechtigungen:

| Berechtigung     | Zugriff             |
| ---------------- | ------------------- |
| Actions          | Lesen und Schreiben |
| Checks           | Lesen und Schreiben |
| Contents         | Lesen und Schreiben |
| Discussions      | Lesen und Schreiben |
| Issues           | Lesen und Schreiben |
| Members          | Lesen               |
| Metadata         | Lesen               |
| Pull requests    | Lesen und Schreiben |
| Repository hooks | Lesen und Schreiben |
| Statuses         | Lesen               |
| Workflows        | Lesen und Schreiben |

Der Berechtigungssatz kann sich auch vor den Funktionen ändern, die ihn verwenden. Wenn die App eine Berechtigung anfordert, die sie vorher nicht hatte, fordert GitHub den Kontoinhaber auf, sie zu genehmigen, einen Organisationsinhaber für eine Organisationsinstallation, und die Installation behält ihre alten Berechtigungen, bis sie dies tun. Wenn beispielsweise der Actions-Zugriff von Lesen zu Schreiben wechselt, kann die App Workflows erneut ausführen, anstatt nur Läufe und Logs anzuzeigen, daher fragt GitHub den Inhaber, die Änderung zu genehmigen.

Wenn Sie die App installieren, akzeptieren Sie ihren vollständigen Berechtigungssatz. GitHub lässt Sie nicht, eine Teilmenge zu akzeptieren. Wenn Ihre Organisation nur die Berechtigungen benötigt, die die Claude Code GitHub Action verwendet, erstellen Sie eine benutzerdefinierte GitHub App mit Contents, Issues und Pull Requests statt, indem Sie die [Claude Code GitHub Action-Setup-Anleitung](https://github.com/anthropics/claude-code-action/blob/main/docs/setup.md) befolgen. Eine benutzerdefinierte App deckt nur die Claude Code GitHub Action ab. Code Review und Web-Auto-Fix erfordern immer noch die offizielle App.

Für Details, wie die Claude Code GitHub Action einschränkt, was Claude mit diesen Berechtigungen tun kann, siehe die [Sicherheitsdokumentation](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).

<h2 id="interactive-and-automation-modes">
  Interaktive und Automatisierungsmodi
</h2>

Die Claude Code GitHub Action erkennt, wie sie aus Ihrer Workflow-Konfiguration ausgeführt werden soll:

* **Interaktiver Modus**: Wenn der Workflow keine `prompt`-Eingabe bereitstellt, wartet Claude auf die Trigger-Phrase, `@claude` standardmäßig, in einem Issue- oder Pull-Request-Kommentar, in einer Pull-Request-Überprüfung oder im Body oder Titel eines neu geöffneten Issues, und antwortet dann auf diese Anfrage. Fortschritt und Ergebnisse erscheinen als Kommentar auf dem auslösenden Issue oder PR.
* **Automatisierungsmodus**: Wenn der Workflow eine `prompt`-Eingabe bereitstellt, wird Claude ohne Warten auf eine Erwähnung ausgeführt, unterliegt nur den [Überprüfungen, wer Läufe auslösen kann](#who-can-trigger-runs). Standardmäßig erscheinen Ergebnisse im Workflow-Run-Log statt in einem Kommentar. Claude kann auf den Issue oder Pull Request posten, wenn der Prompt es anweist und er ein Tool hat, das posten kann, wie im [Code-Review-Beispiel](#run-a-skill).

<h3 id="who-can-trigger-runs">
  Wer kann Läufe auslösen
</h3>

In beiden Modi führt die Claude Code GitHub Action zwei Überprüfungen des auslösenden Akteurs durch, bevor Claude beginnt, und der Lauf schlägt fehl, wenn eine Überprüfung ihn ablehnt:

* **Schreibzugriff**: Bei Issue- und Pull-Request-Events muss der auslösende Benutzer Schreibzugriff auf das Repository haben. Um bestimmte Benutzer ohne Schreibzugriff zuzulassen, legen Sie `allowed_non_write_users` fest und übergeben Sie Ihre eigene `github_token`-Eingabe. Events, die kein Benutzer verfasst, wie ein `schedule`-Trigger, überspringen diese Überprüfung.
* **Menschlicher Akteur**: Bei jedem Event lehnt die Claude Code GitHub Action einen Bot-Akteur ab, es sei denn, Sie listen ihn in `allowed_bots` auf, was Bots daran hindert, Claude in einer Schleife auszulösen. Diese Überprüfung gilt auch für geplante Läufe, die GitHub einem Repository-Benutzer zuordnet, normalerweise dem, der zuletzt den `cron`-Zeitplan des Workflows geändert hat. Wenn dieser Benutzer ein Bot ist, listen Sie ihn in `allowed_bots` auf.

<h2 id="example-use-cases">
  Beispiel-Anwendungsfälle
</h2>

Das [Beispielverzeichnis](https://github.com/anthropics/claude-code-action/tree/main/examples) enthält einsatzbereite Workflows für verschiedene Szenarien.

Die Beispiele auf dieser Seite zeigen API-Schlüssel-Authentifizierung. Wenn Sie sich mit einem Claude-Abonnement authentifizieren, ersetzen Sie die `anthropic_api_key`-Zeile in jedem Beispiel durch `claude_code_oauth_token: ${{ secrets.CLAUDE_CODE_OAUTH_TOKEN }}`.

<h3 id="respond-to-claude-mentions">
  Reagieren Sie auf @claude-Erwähnungen
</h3>

Dieser Workflow führt die Claude Code GitHub Action im interaktiven Modus aus, daher antwortet Claude, wenn jemand `@claude` in einem Issue- oder PR-Kommentar erwähnt.

```yaml theme={null}
name: Claude Code
on:
  issue_comment:
    types: [created]
  pull_request_review_comment:
    types: [created]
jobs:
  claude:
    if: contains(github.event.comment.body, '@claude')
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: write
      issues: write
      id-token: write
      actions: read
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 1
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
```

Die Teile dieses Workflows, die nicht Boilerplate sind:

* `id-token: write`: erforderlich für die Standard-GitHub-App-Authentifizierung der Claude Code GitHub Action
* `actions: read`: lässt Claude CI-Ergebnisse auf PRs lesen
* `actions/checkout`: gibt Claude eine lokale Kopie des Repositories zum Arbeiten
* `if`: verhindert, dass Runner bei Kommentaren starten, die `@claude` nicht erwähnen. Die Claude Code GitHub Action prüft auch die Trigger-Phrase selbst, bevor sie antwortet

Sobald der Workflow vorhanden ist, erwähnen Sie `@claude` in jedem Issue- oder PR-Kommentar mit einer Anfrage:

```text wrap theme={null}
@claude implement this feature based on the issue description
@claude how should I implement user authentication for this endpoint?
@claude fix the TypeError in the user dashboard component
```

Claude antwortet in einem Kommentar auf dem gleichen Issue oder PR und aktualisiert ihn, während es arbeitet.

<h3 id="run-a-skill">
  Führen Sie eine Skill aus
</h3>

Die `prompt`-Eingabe akzeptiert eine [Skill](/docs/de/skills)-Invokation sowie einfachen Text:

* Für eine Skill in Ihrem Repository-Verzeichnis `.claude/skills/`, führen Sie `actions/checkout` vor dem `anthropics/claude-code-action`-Schritt aus, damit die Skill-Dateien auf dem Runner verfügbar sind, und übergeben Sie dann `/skill-name` als `prompt`.
* Für eine Skill, die in einem [Plugin](/docs/de/plugins/overview) verpackt ist, installieren Sie das Plugin mit den Eingaben `plugin_marketplaces` und `plugins`, und übergeben Sie dann den namespaced `/plugin-name:skill-name` als `prompt`. Die `plugins`-Eingabe nimmt `plugin-name@marketplace-name`, wobei der Marketplace-Name aus dem Manifest des Marketplace selbst kommt, nicht aus seiner Repository-URL.

Der folgende Workflow installiert das `code-review`-Plugin und führt seine Skill aus, wenn ein Pull Request geöffnet, aktualisiert, erneut geöffnet oder als bereit für Überprüfung markiert wird. Er führt das gleiche Plugin wie der Review-Workflow aus dem schnellen Setup aus. Verwenden Sie einen Workflow wie diesen, wenn Sie den Prompt, das Modell und die Trigger selbst kontrollieren möchten. Für automatische Überprüfungen ohne Wartung einer Workflow-Datei siehe [Code Review](/docs/de/code-review). Auf öffentlichen Repositories hält GitHub Geheimnisse von Läufen zurück, die durch Fork-Pull-Requests ausgelöst werden, daher wird die Überprüfung nur auf Pull Requests aus Branches im gleichen Repository ausgeführt.

```yaml theme={null}
name: Code Review
on:
  pull_request:
    types: [opened, synchronize, ready_for_review, reopened]
jobs:
  review:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: read
      issues: read
      id-token: write
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 1
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          plugin_marketplaces: "https://github.com/anthropics/claude-code.git"
          plugins: "code-review@claude-code-plugins"
          prompt: "/code-review:code-review --comment ${{ github.repository }}/pull/${{ github.event.pull_request.number }}"
          claude_args: '--allowedTools "mcp__github_inline_comment__create_inline_comment"'
```

Zwei Zeilen in diesem Workflow kontrollieren, wo die Überprüfung hingeht:

* **`--comment`**: Claude postet seine Überprüfung auf dem Pull Request, als Inline-Kommentar zu jedem gefundenen Problem oder als einen Zusammenfassungs-Kommentar, wenn keine gefunden werden. Ohne ihn postet Claude nichts, und Sie lesen die Erkenntnisse im Workflow-Run-Log.
* **`claude_args`**: Behalten Sie diese Zeile bei, obwohl die Frontmatter `allowed-tools` der Skill selbst das gleiche Tool benennt, weil die Claude Code GitHub Action den MCP-Server startet, der Inline-Kommentare postet, nur wenn `--allowedTools` in `claude_args` ihn benennt.

Claude überspringt Entwürfe und geschlossene Pull Requests, Pull Requests, die er nicht überprüft zu benötigen beurteilt, wie automatisierte oder triviale, und Pull Requests, die bereits einen Kommentar von Claude haben.

<h3 id="run-on-a-schedule">
  Führen Sie nach einem Zeitplan aus
</h3>

Mit einer `prompt`-Eingabe wird die Claude Code GitHub Action im Automatisierungsmodus bei jedem GitHub-Event ausgeführt, einschließlich eines Cron-Zeitplans. Für einen einfachen Text-Prompt hat Claude keinen Shell- oder GitHub API-Zugriff, bis Sie die Tools, die der Prompt benötigt, mit `--allowedTools` in `claude_args` oder einer [`permissions.allow`-Regel](/docs/de/permissions#permission-rule-syntax) in der `settings`-Eingabe gewähren. Wenn Sie stattdessen eine Skill aufrufen, kann Claude die Tools verwenden, die ihre [`allowed-tools`-Frontmatter](/docs/de/skills#pre-approve-tools-for-a-skill) gewährt. GitHub führt geplante Workflows nur vom Standard-Branch aus und deaktiviert den Zeitplan in öffentlichen Repositories nach 60 Tagen ohne Repository-Aktivität.

Dieser Workflow generiert einen Bericht im Workflow-Run-Log um 09:00 UTC jeden Tag. Seine `claude_args`-Zeile [übergibt CLI-Argumente](#pass-cli-arguments), die das Modell auswählen und zwei GitHub MCP-Tools zulassen. Claude liest Commits und Issues durch die GitHub API mit diesen Tools, daher können Sie den Checkout-Schritt weglassen:

```yaml theme={null}
name: Daily Report
on:
  schedule:
    - cron: "0 9 * * *"
jobs:
  report:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      issues: read
      id-token: write
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          prompt: "Generate a summary of yesterday's commits and open issues"
          claude_args: |
            --model claude-opus-5-5
            --allowedTools "mcp__github__list_commits,mcp__github__list_issues"
```

<h2 id="best-practices">
  Best Practices
</h2>

<h3 id="define-project-standards-in-claude-md">
  Definieren Sie Projektstandards in CLAUDE.md
</h3>

Erstellen Sie eine `CLAUDE.md`-Datei im Root-Verzeichnis Ihres Repositories, um Code-Style-Richtlinien, Review-Kriterien, projektspezifische Regeln und bevorzugte Muster zu definieren. Claude befolgt diese Richtlinien beim Erstellen von PRs und Antworten auf Anfragen. Siehe die [Memory-Dokumentation](/docs/de/memory) für Details.

<h3 id="protect-your-credentials">
  Schützen Sie Ihre Anmeldedaten
</h3>

<Warning>
  Committen Sie niemals API-Schlüssel oder OAuth-Tokens direkt in Ihr Repository. Speichern Sie sie immer als GitHub Secrets und referenzieren Sie sie in Workflows, zum Beispiel `anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}`.
</Warning>

Gewähren Sie dem Workflow nur die Berechtigungen, die er benötigt, und überprüfen Sie Claudes Änderungen vor dem Mergen.

Für umfassende Sicherheitsleitlinien einschließlich Berechtigungen und Authentifizierung siehe die [Claude Code Action-Sicherheitsdokumentation](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).

<h3 id="manage-costs">
  Verwalten Sie Kosten
</h3>

Jeder Lauf verbraucht zwei Arten von Ressourcen:

* **GitHub Actions-Minuten**: Die Claude Code GitHub Action wird auf GitHub-gehosteten Runnern ausgeführt, die Ihre GitHub Actions-Minuten verbrauchen. Siehe [Githubs Abrechnungsdokumentation](https://docs.github.com/en/billing/managing-billing-for-your-products/managing-billing-for-github-actions/about-billing-for-github-actions) für Preise und Minutenlimits.
* **API-Tokens**: Jede Interaktion verbraucht Tokens basierend auf der Länge von Prompts und Antworten, Aufgabenkomplexität und Codebase-Größe. Siehe [Claudes Preisseite](https://claude.com/platform/api) für aktuelle Token-Raten. Wenn Sie sich mit einem OAuth-Token authentifizieren, verwenden Läufe Ihr Claude-Abonnement statt API-Abrechnung.

Sie können beide Arten von Kosten senken, indem Sie Claude klareren Kontext geben und begrenzen, wie viel Arbeit jeder Lauf tun kann:

* Schreiben Sie spezifische `@claude`-Anfragen, damit Claude weniger Turns benötigt, um zu beenden
* Verwenden Sie Issue-Templates, um Kontext im Voraus bereitzustellen
* Halten Sie Ihre `CLAUDE.md` prägnant, da Claude sie bei jedem Lauf liest
* Legen Sie `--max-turns` in `claude_args` fest, um Iterationen zu begrenzen
* Legen Sie Workflow-Level-Timeouts fest, um unkontrollierte Jobs zu vermeiden
* Verwenden Sie Githubs Concurrency-Kontrollen, um parallele Läufe zu begrenzen

Für Nutzungsverfolgung über Ihre Organisation hinweg siehe das [Analytics-Dashboard](/docs/de/analytics) und [Monitoring](/docs/de/monitoring-usage). Für wie Nutzung gemessen und abgerechnet wird, siehe [Kosten](/docs/de/costs).

<h2 id="use-a-cloud-provider">
  Verwenden Sie einen Cloud-Provider
</h2>

Standardmäßig ruft die Claude Code GitHub Action die Claude API direkt mit Ihrem API-Schlüssel oder OAuth-Token auf. Um Inferenz stattdessen durch Ihr eigenes Cloud-Konto zu leiten, legen Sie die Eingabe für Ihren Provider fest und folgen Sie [Claude Code GitHub Actions mit Cloud-Providern verwenden](/docs/de/github-actions-cloud-providers):

* **Amazon Bedrock**: `use_bedrock: "true"`
* **Google Cloud's Agent Platform**: `use_vertex: "true"`
* **Microsoft Foundry**: `use_foundry: "true"`

Mit allen drei Providern authentifizieren Sie sich durch OIDC-Identitäts-Verbund statt mit einem Claude API-Schlüssel, daher speichern Sie keine statischen Cloud-Anmeldedaten in Ihrem Repository.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude antwortet nicht auf @claude-Befehle
</h3>

* Überprüfen Sie, dass die GitHub App auf dem Repository installiert ist
* Überprüfen Sie, dass Workflows für das Repository aktiviert sind
* Stellen Sie sicher, dass Ihr API-Schlüssel oder OAuth-Token in Repository-Geheimnissen gesetzt ist
* Bestätigen Sie, dass der Kommentar `@claude` als vollständiges Wort enthält, nicht `/claude` oder `@claude-bot`
* Bestätigen Sie, dass der kommentierende Benutzer Schreibzugriff auf das Repository hat. Siehe [Wer kann Läufe auslösen](#who-can-trigger-runs) für die Ausnahmen

<h3 id="ci-not-running-on-claude’s-commits">
  CI wird nicht auf Claudes Commits ausgeführt
</h3>

* GitHub löst keine Workflows auf Commits aus, die mit dem Standard-`GITHUB_TOKEN` gemacht werden. Wenn Sie `github_token: ${{ secrets.GITHUB_TOKEN }}` an die Claude Code GitHub Action übergeben, entfernen Sie es, damit es sich als Claude GitHub App authentifiziert, oder übergeben Sie stattdessen ein benutzerdefiniertes App-Token
* Überprüfen Sie, dass die Trigger Ihres CI-Workflows die Events enthalten, die Claudes Pushes produzieren, wie `push` oder `pull_request`

<h3 id="authentication-errors">
  Authentifizierungsfehler
</h3>

* Bestätigen Sie, dass der API-Schlüssel oder OAuth-Token gültig ist, indem Sie ihn lokal mit `claude` testen, bevor Sie den Workflow debuggen
* Für Bedrock, Agent Platform und Foundry siehe den [Fehlerbehebungsabschnitt](/docs/de/github-actions-cloud-providers#troubleshooting) der Cloud-Provider-Seite

Für weitere Lösungen siehe die [FAQ](https://github.com/anthropics/claude-code-action/blob/main/docs/faq.md) der Claude Code GitHub Action.

<h2 id="advanced-configuration">
  Erweiterte Konfiguration
</h2>

<h3 id="action-parameters">
  Action-Parameter
</h3>

Dies sind die am häufigsten verwendeten Eingaben. Jede ordnet sich einem `with:`-Schlüssel im `anthropics/claude-code-action`-Schritt zu.

| Parameter                 | Beschreibung                                                                                                                                                                                    | Erforderlich                                                                                                                                                                                        |
| ------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`                  | Anweisungen für Claude, als einfacher Text oder eine [Skill](/docs/de/skills)-Invokation. Wenn weggelassen, antwortet Claude stattdessen auf die [Trigger-Phrase](#interactive-and-automation-modes) | Nein                                                                                                                                                                                                |
| `claude_args`             | CLI-Argumente, die an Claude Code übergeben werden                                                                                                                                              | Nein                                                                                                                                                                                                |
| `anthropic_api_key`       | Claude API-Schlüssel                                                                                                                                                                            | Für die Claude API, es sei denn, Sie verwenden `claude_code_oauth_token` oder [Workload Identity Federation](#set-up-for-an-organization). Nicht verwendet für Bedrock, Agent Platform oder Foundry |
| `claude_code_oauth_token` | OAuth-Token für Authentifizierung mit einem Claude-Abonnement, generiert mit `claude setup-token`                                                                                               | Nein                                                                                                                                                                                                |
| `github_token`            | Token für GitHub-Operationen. Wenn weggelassen, authentifiziert sich die Claude Code GitHub Action als Claude GitHub App                                                                        | Nein                                                                                                                                                                                                |
| `plugin_marketplaces`     | Zeilenumbruch-getrennte Liste von Plugin-Marketplace-Git-URLs                                                                                                                                   | Nein                                                                                                                                                                                                |
| `plugins`                 | Zeilenumbruch-getrennte Liste von Plugin-Namen zur Installation vor der Ausführung                                                                                                              | Nein                                                                                                                                                                                                |
| `settings`                | Claude Code-Einstellungen, als JSON-String oder Pfad zu einer Settings-JSON-Datei                                                                                                               | Nein                                                                                                                                                                                                |
| `trigger_phrase`          | Trigger-Phrase, auf die Claude antwortet. Standard: `@claude`                                                                                                                                   | Nein                                                                                                                                                                                                |
| `use_bedrock`             | Verwenden Sie Amazon Bedrock statt der Claude API                                                                                                                                               | Nein                                                                                                                                                                                                |
| `use_vertex`              | Verwenden Sie Google Cloud's Agent Platform statt der Claude API                                                                                                                                | Nein                                                                                                                                                                                                |
| `use_foundry`             | Verwenden Sie Microsoft Foundry statt der Claude API                                                                                                                                            | Nein                                                                                                                                                                                                |

Für die vollständige Eingabeliste siehe die [Konfigurationsreferenz](https://github.com/anthropics/claude-code-action/blob/main/docs/usage.md#inputs) der Claude Code GitHub Action.

<h3 id="pass-cli-arguments">
  Übergeben Sie CLI-Argumente
</h3>

Der Parameter `claude_args` akzeptiert jedes [Claude Code CLI-Argument](/docs/de/cli-reference):

```yaml theme={null}
claude_args: "--max-turns 5 --model claude-sonnet-5 --mcp-config /path/to/config.json"
```

Häufige Argumente:

* `--max-turns`: Begrenzen Sie die Anzahl der Gesprächs-Turns
* `--model`: Zu verwendendes Modell, zum Beispiel `claude-sonnet-5`. Ohne dieses Argument verwendet die Claude Code GitHub Action das Claude Code [Standard-Modell](/docs/de/model-config)
* `--mcp-config`: Pfad zur [MCP-Konfiguration](/docs/de/mcp)
* `--allowedTools`: Komma-getrennte Liste zulässiger Tools. Der Alias `--allowed-tools` funktioniert auch
* `--debug`: Debug-Ausgabe aktivieren

<h2 id="upgrade-from-beta">
  Upgrade von Beta
</h2>

Wenn Ihre Workflows immer noch `anthropics/claude-code-action@beta` referenzieren, aktualisieren Sie sie auf v1:

1. Ändern Sie `@beta` zu `@v1` in der `uses`-Zeile
2. Entfernen Sie die `mode`-Eingabe, da die Claude Code GitHub Action jetzt [den Modus automatisch erkennt](#interactive-and-automation-modes)
3. Ersetzen Sie `direct_prompt` durch `prompt`
4. Verschieben Sie CLI-Optionen wie `max_turns` und `model` in `claude_args`. `custom_instructions` hat kein gleichnamiges Flag und wird zu `--append-system-prompt`

Für die vollständige Eingabe-Zuordnung und Vorher-und-Nachher-Beispiele siehe den [Migrationsleitfaden](https://github.com/anthropics/claude-code-action/blob/main/docs/migration-guide.md).

<h2 id="what’s-next">
  Was kommt als Nächstes
</h2>

* [Claude Code GitHub Actions mit Cloud-Providern verwenden](/docs/de/github-actions-cloud-providers): Leiten Sie Inferenz durch Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry
* [Konfigurationsreferenz](https://github.com/anthropics/claude-code-action/blob/main/docs/usage.md#inputs): die vollständige Liste der Action-Eingaben
* [Beispielverzeichnis](https://github.com/anthropics/claude-code-action/tree/main/examples): einsatzbereite Workflows für weitere Szenarien
* [Code Review](/docs/de/code-review): automatische Pull-Request-Überprüfung ohne Wartung einer Workflow-Datei
