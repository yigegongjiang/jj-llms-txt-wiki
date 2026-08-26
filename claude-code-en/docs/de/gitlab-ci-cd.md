> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitLab CI/CD

> Erfahren Sie, wie Sie Claude Code in Ihren Entwicklungs-Workflow mit GitLab CI/CD integrieren

<Info>
  Claude Code für GitLab CI/CD befindet sich derzeit in der Beta-Phase. Funktionen und Funktionalität können sich weiterentwickeln, während wir die Erfahrung verfeinern.

  Diese Integration wird von GitLab gepflegt. Für Support siehe das folgende [GitLab-Problem](https://gitlab.com/gitlab-org/gitlab/-/issues/573776).
</Info>

<Note>
  Diese Integration basiert auf der [Claude Code CLI und Agent SDK](/docs/de/agent-sdk/overview) und ermöglicht die programmgesteuerte Nutzung von Claude in Ihren CI/CD-Jobs und benutzerdefinierten Automatisierungs-Workflows.
</Note>

<h2 id="why-use-claude-code-with-gitlab">
  Warum Claude Code mit GitLab verwenden?
</h2>

* **Sofortige MR-Erstellung**: Beschreiben Sie, was Sie benötigen, und Claude schlägt einen vollständigen MR mit Änderungen und Erklärung vor
* **Automatisierte Implementierung**: Verwandeln Sie Probleme mit einem einzigen Befehl oder einer Erwähnung in funktionierenden Code
* **Projektbewusst**: Claude folgt Ihren `CLAUDE.md`-Richtlinien und vorhandenen Code-Mustern
* **Einfaches Setup**: Fügen Sie einen Job zu `.gitlab-ci.yml` und eine maskierte CI/CD-Variable hinzu
* **Enterprise-ready**: Wählen Sie Claude API, Amazon Bedrock oder Google Cloud's Agent Platform, um Anforderungen an Datenresidenz und Beschaffung zu erfüllen
* **Standardmäßig sicher**: Läuft in Ihren GitLab-Runnern mit Ihrem Branch-Schutz und Genehmigungen

<h2 id="how-it-works">
  Wie es funktioniert
</h2>

Claude Code verwendet GitLab CI/CD, um KI-Aufgaben in isolierten Jobs auszuführen und Ergebnisse über MRs zurückzucommiten:

1. **Ereignisgesteuerte Orchestrierung**: GitLab lauscht auf Ihre gewählten Trigger (zum Beispiel ein Kommentar, der `@claude` in einem Problem, MR oder Review-Thread erwähnt). Der Job sammelt Kontext aus dem Thread und Repository, erstellt Prompts aus dieser Eingabe und führt Claude Code aus.

2. **Provider-Abstraktion**: Verwenden Sie den Provider, der zu Ihrer Umgebung passt:
   * Claude API (SaaS)
   * Amazon Bedrock (IAM-basierter Zugriff, regionsübergreifende Optionen)
   * Google Cloud's Agent Platform (GCP-nativ, Workload Identity Federation)

3. **Sandboxed-Ausführung**: Jede Interaktion läuft in einem Container mit strikten Netzwerk- und Dateisystem-Regeln. Claude Code erzwingt Workspace-bezogene Berechtigungen, um Schreibvorgänge einzuschränken. Jede Änderung fließt durch einen MR, damit Reviewer den Diff sehen und Genehmigungen weiterhin gelten.

Wählen Sie regionale Endpunkte, um die Latenz zu reduzieren und Anforderungen an die Datensouveränität zu erfüllen, während Sie vorhandene Cloud-Vereinbarungen nutzen.

<h2 id="what-can-claude-do">
  Was kann Claude tun?
</h2>

Claude Code ermöglicht leistungsstarke CI/CD-Workflows, die verändern, wie Sie mit Code arbeiten:

* Erstellen und aktualisieren Sie MRs aus Problembeschreibungen oder Kommentaren
* Analysieren Sie Performance-Regressionstests und schlagen Sie Optimierungen vor
* Implementieren Sie Features direkt in einem Branch und öffnen Sie dann einen MR
* Beheben Sie Bugs und Regressionstests, die durch Tests oder Kommentare identifiziert wurden
* Antworten Sie auf Folgekommentare, um auf angeforderte Änderungen zu iterieren

<h2 id="setup">
  Setup
</h2>

<h3 id="quick-setup">
  Schnelles Setup
</h3>

Der schnellste Weg zum Einstieg ist, einen minimalen Job zu Ihrer `.gitlab-ci.yml` hinzuzufügen und Ihren API-Schlüssel als maskierte Variable festzulegen.

1. **Fügen Sie eine maskierte CI/CD-Variable hinzu**
   * Gehen Sie zu **Einstellungen** → **CI/CD** → **Variablen**
   * Fügen Sie `ANTHROPIC_API_KEY` hinzu (maskiert, bei Bedarf geschützt)

2. **Fügen Sie einen Claude-Job zu `.gitlab-ci.yml` hinzu**

```yaml theme={null}
stages:
  - ai

claude:
  stage: ai
  image: node:24-alpine3.21
  # Passen Sie die Regeln an, um zu passen, wie Sie den Job auslösen möchten:
  # - manuelle Ausführungen
  # - Merge-Request-Ereignisse
  # - Web/API-Trigger, wenn ein Kommentar '@claude' enthält
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
  variables:
    GIT_STRATEGY: fetch
  before_script:
    - apk update
    - apk add --no-cache git curl bash
    - curl -fsSL https://claude.ai/install.sh | bash
  script:
    # Optional: Starten Sie einen GitLab MCP-Server, wenn Ihr Setup einen bereitstellt
    - /bin/gitlab-mcp-server || true
    # Verwenden Sie AI_FLOW_*-Variablen beim Aufrufen über Web/API-Trigger mit Kontext-Payloads
    - echo "$AI_FLOW_INPUT for $AI_FLOW_CONTEXT on $AI_FLOW_EVENT"
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Review this MR and implement the requested changes'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
```

Nach dem Hinzufügen des Jobs und Ihrer `ANTHROPIC_API_KEY`-Variable testen Sie, indem Sie den Job manuell von **CI/CD** → **Pipelines** ausführen, oder lösen Sie ihn von einem MR aus, um Claude zu ermöglichen, Updates in einem Branch vorzuschlagen und bei Bedarf einen MR zu öffnen.

<Note>
  Um stattdessen auf Amazon Bedrock oder Google Cloud's Agent Platform auszuführen, siehe den Abschnitt [Verwendung mit Amazon Bedrock und Google Cloud](#using-with-amazon-bedrock-and-google-cloud) unten für Authentifizierung und Umgebungssetup.
</Note>

<h3 id="manual-setup-recommended-for-production">
  Manuelles Setup (empfohlen für Produktion)
</h3>

Wenn Sie ein kontrolliertes Setup bevorzugen oder Enterprise-Provider benötigen:

1. **Konfigurieren Sie Provider-Zugriff**:
   * **Claude API**: Erstellen und speichern Sie `ANTHROPIC_API_KEY` als maskierte CI/CD-Variable
   * **Amazon Bedrock**: **Konfigurieren Sie GitLab** → **AWS OIDC** und erstellen Sie eine IAM-Rolle für Amazon Bedrock
   * **Google Cloud's Agent Platform**: **Konfigurieren Sie Workload Identity Federation für GitLab** → **GCP**

2. **Fügen Sie Projekt-Anmeldedaten für GitLab API-Operationen hinzu**:
   * Verwenden Sie `CI_JOB_TOKEN` standardmäßig, oder erstellen Sie ein Project Access Token mit `api`-Bereich
   * Speichern Sie als `GITLAB_ACCESS_TOKEN` (maskiert), wenn Sie ein PAT verwenden

3. **Fügen Sie den Claude-Job zu `.gitlab-ci.yml` hinzu** (siehe Beispiele unten)

4. **(Optional) Aktivieren Sie Mention-gesteuerte Trigger**:
   * Fügen Sie einen Projekt-Webhook für "Kommentare (Notizen)" zu Ihrem Event-Listener hinzu (falls Sie einen verwenden)
   * Lassen Sie den Listener die Pipeline-Trigger-API mit Variablen wie `AI_FLOW_INPUT` und `AI_FLOW_CONTEXT` aufrufen, wenn ein Kommentar `@claude` enthält

<h2 id="example-use-cases">
  Beispiel-Anwendungsfälle
</h2>

<h3 id="turn-issues-into-mrs">
  Verwandeln Sie Probleme in MRs
</h3>

In einem Problemkommentar:

```text theme={null}
@claude implement this feature based on the issue description
```

Claude analysiert das Problem und die Codebasis, schreibt Änderungen in einem Branch und öffnet einen MR zur Überprüfung.

<h3 id="get-implementation-help">
  Erhalten Sie Implementierungshilfe
</h3>

In einer MR-Diskussion:

```text theme={null}
@claude suggest a concrete approach to cache the results of this API call
```

Claude schlägt Änderungen vor, fügt Code mit angemessenem Caching hinzu und aktualisiert den MR.

<h3 id="fix-bugs-quickly">
  Beheben Sie Bugs schnell
</h3>

In einem Problem- oder MR-Kommentar:

```text theme={null}
@claude fix the TypeError in the user dashboard component
```

Claude lokalisiert den Bug, implementiert eine Korrektur und aktualisiert den Branch oder öffnet einen neuen MR.

<h2 id="using-with-amazon-bedrock-and-google-cloud">
  Verwendung mit Amazon Bedrock und Google Cloud
</h2>

Für Enterprise-Umgebungen können Sie Claude Code vollständig auf Ihrer Cloud-Infrastruktur mit der gleichen Entwicklererfahrung ausführen.

<Tabs>
  <Tab title="Amazon Bedrock">
    ### Voraussetzungen

    Bevor Sie Claude Code mit Amazon Bedrock einrichten, benötigen Sie:

    1. Ein AWS-Konto mit Amazon Bedrock-Zugriff auf die gewünschten Claude-Modelle
    2. GitLab als OIDC-Identitätsanbieter in AWS IAM konfiguriert
    3. Eine IAM-Rolle mit Amazon Bedrock-Berechtigungen und einer Vertrauensrichtlinie, die auf Ihr GitLab-Projekt/Refs beschränkt ist
    4. GitLab CI/CD-Variablen für Rollenübernahme:
       * `AWS_ROLE_TO_ASSUME` (Rollen-ARN)
       * `AWS_REGION` (Amazon Bedrock-Region)

    ### Setup-Anweisungen

    Konfigurieren Sie AWS, um GitLab CI-Jobs zu ermöglichen, eine IAM-Rolle über OIDC anzunehmen (keine statischen Schlüssel).

    **Erforderliches Setup:**

    1. Aktivieren Sie Amazon Bedrock und fordern Sie Zugriff auf Ihre Ziel-Claude-Modelle an
    2. Erstellen Sie einen IAM OIDC-Provider für GitLab, falls nicht bereits vorhanden
    3. Erstellen Sie eine IAM-Rolle, der der GitLab OIDC-Provider vertraut, beschränkt auf Ihr Projekt und geschützte Refs
    4. Fügen Sie Least-Privilege-Berechtigungen für Amazon Bedrock-Invoke-APIs an

    **Erforderliche Werte zum Speichern in CI/CD-Variablen:**

    * `AWS_ROLE_TO_ASSUME`
    * `AWS_REGION`

    Fügen Sie Variablen in Einstellungen → CI/CD → Variablen hinzu:

    ```yaml theme={null}
    # Für Amazon Bedrock:
    - AWS_ROLE_TO_ASSUME
    - AWS_REGION
    ```

    Verwenden Sie das Amazon Bedrock-Job-Beispiel oben, um das GitLab-Job-Token gegen temporäre AWS-Anmeldedaten zur Laufzeit auszutauschen.
  </Tab>

  <Tab title="Google Cloud's Agent Platform">
    ### Voraussetzungen

    Bevor Sie Claude Code mit Google Cloud's Agent Platform einrichten, benötigen Sie:

    1. Ein Google Cloud-Projekt mit:
       * Aktivierter Google Cloud's Agent Platform API
       * Workload Identity Federation konfiguriert, um GitLab OIDC zu vertrauen
    2. Ein dediziertes Service-Konto mit nur den erforderlichen Google Cloud's Agent Platform-Rollen
    3. GitLab CI/CD-Variablen für WIF:
       * `GCP_WORKLOAD_IDENTITY_PROVIDER` (vollständiger Ressourcenname)
       * `GCP_SERVICE_ACCOUNT` (Service-Konto-E-Mail)

    ### Setup-Anweisungen

    Konfigurieren Sie Google Cloud, um GitLab CI-Jobs zu ermöglichen, ein Service-Konto über Workload Identity Federation zu imitieren.

    **Erforderliches Setup:**

    1. Aktivieren Sie IAM Credentials API, STS API und Google Cloud's Agent Platform API
    2. Erstellen Sie einen Workload Identity Pool und Provider für GitLab OIDC
    3. Erstellen Sie ein dediziertes Service-Konto mit Google Cloud's Agent Platform-Rollen
    4. Gewähren Sie dem WIF-Principal die Berechtigung, das Service-Konto zu imitieren

    **Erforderliche Werte zum Speichern in CI/CD-Variablen:**

    * `GCP_WORKLOAD_IDENTITY_PROVIDER`
    * `GCP_SERVICE_ACCOUNT`

    Fügen Sie Variablen in Einstellungen → CI/CD → Variablen hinzu:

    ```yaml theme={null}
    # Für Google Cloud's Agent Platform:
    - GCP_WORKLOAD_IDENTITY_PROVIDER
    - GCP_SERVICE_ACCOUNT
    - CLOUD_ML_REGION (zum Beispiel us-east5)
    ```

    Verwenden Sie das Job-Beispiel oben für Google Cloud's Agent Platform, um sich ohne Speicherung von Schlüsseln zu authentifizieren.
  </Tab>
</Tabs>

<h2 id="configuration-examples">
  Konfigurationsbeispiele
</h2>

Nachfolgend finden Sie einsatzbereite Snippets, die Sie an Ihre Pipeline anpassen können.

<h3 id="basic-gitlab-ci-yml-claude-api">
  Basis .gitlab-ci.yml (Claude API)
</h3>

```yaml theme={null}
stages:
  - ai

claude:
  stage: ai
  image: node:24-alpine3.21
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
  variables:
    GIT_STRATEGY: fetch
  before_script:
    - apk update
    - apk add --no-cache git curl bash
    - curl -fsSL https://claude.ai/install.sh | bash
  script:
    - /bin/gitlab-mcp-server || true
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Summarize recent changes and suggest improvements'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
  # Claude Code wird ANTHROPIC_API_KEY aus CI/CD-Variablen verwenden
```

<h3 id="amazon-bedrock-job-example-oidc">
  Amazon Bedrock-Job-Beispiel (OIDC)
</h3>

**Voraussetzungen:**

* Amazon Bedrock aktiviert mit Zugriff auf Ihr gewähltes Claude-Modell(e)
* GitLab OIDC in AWS konfiguriert mit einer Rolle, die Ihr GitLab-Projekt und Refs vertraut
* IAM-Rolle mit Amazon Bedrock-Berechtigungen (Least Privilege empfohlen)

**Erforderliche CI/CD-Variablen:**

* `AWS_ROLE_TO_ASSUME`: ARN der IAM-Rolle für Amazon Bedrock-Zugriff
* `AWS_REGION`: Amazon Bedrock-Region (zum Beispiel `us-west-2`)

```yaml theme={null}
claude-bedrock:
  stage: ai
  image: node:24-alpine3.21
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  before_script:
    - apk add --no-cache bash curl jq git python3 py3-pip
    - pip install --no-cache-dir awscli
    - curl -fsSL https://claude.ai/install.sh | bash
    # Tauschen Sie GitLab OIDC-Token gegen AWS-Anmeldedaten aus
    - export AWS_WEB_IDENTITY_TOKEN_FILE="${CI_JOB_JWT_FILE:-/tmp/oidc_token}"
    - if [ -n "${CI_JOB_JWT_V2}" ]; then printf "%s" "$CI_JOB_JWT_V2" > "$AWS_WEB_IDENTITY_TOKEN_FILE"; fi
    - >
      aws sts assume-role-with-web-identity
      --role-arn "$AWS_ROLE_TO_ASSUME"
      --role-session-name "gitlab-claude-$(date +%s)"
      --web-identity-token "file://$AWS_WEB_IDENTITY_TOKEN_FILE"
      --duration-seconds 3600 > /tmp/aws_creds.json
    - export AWS_ACCESS_KEY_ID="$(jq -r .Credentials.AccessKeyId /tmp/aws_creds.json)"
    - export AWS_SECRET_ACCESS_KEY="$(jq -r .Credentials.SecretAccessKey /tmp/aws_creds.json)"
    - export AWS_SESSION_TOKEN="$(jq -r .Credentials.SessionToken /tmp/aws_creds.json)"
  script:
    - /bin/gitlab-mcp-server || true
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Implement the requested changes and open an MR'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
  variables:
    AWS_REGION: "us-west-2"
```

<Note>
  Modell-IDs für Amazon Bedrock enthalten regionsspezifische Präfixe (zum Beispiel `us.anthropic.claude-sonnet-4-6`). Übergeben Sie das gewünschte Modell über Ihre Job-Konfiguration oder den Prompt, wenn Ihr Workflow dies unterstützt.
</Note>

<h3 id="agent-platform-job-example-workload-identity-federation">
  Agent Platform-Job-Beispiel (Workload Identity Federation)
</h3>

**Voraussetzungen:**

* Google Cloud's Agent Platform API in Ihrem GCP-Projekt aktiviert
* Workload Identity Federation konfiguriert, um GitLab OIDC zu vertrauen
* Ein Service-Konto mit Google Cloud's Agent Platform-Berechtigungen

**Erforderliche CI/CD-Variablen:**

* `GCP_WORKLOAD_IDENTITY_PROVIDER`: Vollständiger Provider-Ressourcenname
* `GCP_SERVICE_ACCOUNT`: Service-Konto-E-Mail
* `CLOUD_ML_REGION`: Google Cloud's Agent Platform-Region (zum Beispiel `us-east5`)

```yaml theme={null}
claude-vertex:
  stage: ai
  image: gcr.io/google.com/cloudsdktool/google-cloud-cli:slim
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  before_script:
    - apt-get update && apt-get install -y git && apt-get clean
    - curl -fsSL https://claude.ai/install.sh | bash
    # Authentifizieren Sie sich bei Google Cloud über WIF (keine heruntergeladenen Schlüssel)
    - >
      gcloud auth login --cred-file=<(cat <<EOF
      {
        "type": "external_account",
        "audience": "${GCP_WORKLOAD_IDENTITY_PROVIDER}",
        "subject_token_type": "urn:ietf:params:oauth:token-type:jwt",
        "service_account_impersonation_url": "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/${GCP_SERVICE_ACCOUNT}:generateAccessToken",
        "token_url": "https://sts.googleapis.com/v1/token"
      }
      EOF
      )
    - gcloud config set project "$(gcloud projects list --format='value(projectId)' --filter="name:${CI_PROJECT_NAMESPACE}" | head -n1)" || true
  script:
    - /bin/gitlab-mcp-server || true
    - >
      CLOUD_ML_REGION="${CLOUD_ML_REGION:-us-east5}"
      claude
      -p "${AI_FLOW_INPUT:-'Review and update code as requested'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
  variables:
    CLOUD_ML_REGION: "us-east5"
```

<Note>
  Mit Workload Identity Federation müssen Sie keine Service-Konto-Schlüssel speichern. Verwenden Sie Repository-spezifische Vertrauensbedingungen und Least-Privilege-Service-Konten.
</Note>

<h2 id="best-practices">
  Best Practices
</h2>

<h3 id="claude-md-configuration">
  CLAUDE.md-Konfiguration
</h3>

Erstellen Sie eine `CLAUDE.md`-Datei im Repository-Root, um Coding-Standards, Review-Kriterien und projektspezifische Regeln zu definieren. Claude liest diese Datei während der Ausführung und folgt Ihren Konventionen bei der Vorschlag von Änderungen.

<h3 id="security-considerations">
  Sicherheitsüberlegungen
</h3>

**Commiten Sie niemals API-Schlüssel oder Cloud-Anmeldedaten in Ihr Repository**. Verwenden Sie immer GitLab CI/CD-Variablen:

* Fügen Sie `ANTHROPIC_API_KEY` als maskierte Variable hinzu (und schützen Sie sie bei Bedarf)
* Verwenden Sie Provider-spezifisches OIDC, wo möglich (keine langlebigen Schlüssel)
* Begrenzen Sie Job-Berechtigungen und Netzwerk-Egress
* Überprüfen Sie Claudes MRs wie jeden anderen Beitrag

<h3 id="optimizing-performance">
  Optimierung der Leistung
</h3>

* Halten Sie `CLAUDE.md` fokussiert und prägnant
* Geben Sie klare Problem-/MR-Beschreibungen an, um Iterationen zu reduzieren
* Konfigurieren Sie angemessene Job-Timeouts, um unkontrollierte Ausführungen zu vermeiden
* Cachen Sie npm und Paketinstallationen in Runnern, wo möglich

<h3 id="ci-costs">
  CI-Kosten
</h3>

Bei der Verwendung von Claude Code mit GitLab CI/CD sollten Sie sich der damit verbundenen Kosten bewusst sein:

* **GitLab Runner-Zeit**:
  * Claude läuft auf Ihren GitLab-Runnern und verbraucht Compute-Minuten
  * Siehe die Runner-Abrechnung Ihres GitLab-Plans für Details

* **API-Kosten**:
  * Jede Claude-Interaktion verbraucht Token basierend auf Prompt- und Antwortgröße
  * Die Token-Nutzung variiert je nach Aufgabenkomplexität und Codebasis-Größe
  * Siehe [Anthropic-Preisgestaltung](https://platform.claude.com/docs/de/about-claude/pricing) für Details

* **Tipps zur Kostenoptimierung**:
  * Verwenden Sie spezifische `@claude`-Befehle, um unnötige Durchläufe zu reduzieren
  * Legen Sie angemessene `max_turns`- und Job-Timeout-Werte fest
  * Begrenzen Sie die Parallelität, um parallele Ausführungen zu kontrollieren

<h2 id="security-and-governance">
  Sicherheit und Governance
</h2>

* Jeder Job läuft in einem isolierten Container mit eingeschränktem Netzwerkzugriff
* Claudes Änderungen fließen durch MRs, damit Reviewer jeden Diff sehen
* Branch-Schutz und Genehmigungsregeln gelten für KI-generierte Code
* Claude Code verwendet Workspace-bezogene Berechtigungen, um Schreibvorgänge einzuschränken
* Kosten bleiben unter Ihrer Kontrolle, da Sie Ihre eigenen Provider-Anmeldedaten mitbringen

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude antwortet nicht auf @claude-Befehle
</h3>

* Überprüfen Sie, ob Ihre Pipeline ausgelöst wird (manuell, MR-Ereignis oder über einen Note-Event-Listener/Webhook)
* Stellen Sie sicher, dass CI/CD-Variablen (`ANTHROPIC_API_KEY` oder Cloud-Provider-Einstellungen) vorhanden und unmaskiert sind
* Überprüfen Sie, dass der Kommentar `@claude` enthält (nicht `/claude`) und dass Ihr Mention-Trigger konfiguriert ist

<h3 id="job-can’t-write-comments-or-open-mrs">
  Job kann keine Kommentare schreiben oder MRs öffnen
</h3>

* Stellen Sie sicher, dass `CI_JOB_TOKEN` ausreichende Berechtigungen für das Projekt hat, oder verwenden Sie ein Project Access Token mit `api`-Bereich
* Überprüfen Sie, dass das `mcp__gitlab`-Tool in `--allowedTools` aktiviert ist
* Bestätigen Sie, dass der Job im Kontext des MR ausgeführt wird oder über `AI_FLOW_*`-Variablen genug Kontext hat

<h3 id="authentication-errors">
  Authentifizierungsfehler
</h3>

* **Für Claude API**: Bestätigen Sie, dass `ANTHROPIC_API_KEY` gültig und nicht abgelaufen ist
* **Für Amazon Bedrock oder Google Cloud's Agent Platform**: Überprüfen Sie OIDC/WIF-Konfiguration, Rollenimitierung und Geheimnisnamen; bestätigen Sie Region und Modellverfügbarkeit

<h2 id="advanced-configuration">
  Erweiterte Konfiguration
</h2>

<h3 id="common-parameters-and-variables">
  Häufig verwendete Parameter und Variablen
</h3>

Claude Code unterstützt diese häufig verwendeten Eingaben:

* `prompt` / `prompt_file`: Geben Sie Anweisungen inline (`-p`) oder über eine Datei an
* `max_turns`: Begrenzen Sie die Anzahl der Hin- und Herbewegungen
* `timeout_minutes`: Begrenzen Sie die Gesamtausführungszeit
* `ANTHROPIC_API_KEY`: Erforderlich für die Claude API (nicht für Amazon Bedrock oder Google Cloud's Agent Platform verwendet)
* Provider-spezifische Umgebung: `AWS_REGION`, Projekt-/Regionsvariablen für Google Cloud's Agent Platform

<Note>
  Genaue Flags und Parameter können je nach Version von `@anthropic-ai/claude-code` variieren. Führen Sie `claude --help` in Ihrem Job aus, um unterstützte Optionen zu sehen.
</Note>

<h3 id="customizing-claude’s-behavior">
  Anpassung von Claudes Verhalten
</h3>

Sie können Claude auf zwei primäre Arten lenken:

1. **CLAUDE.md**: Definieren Sie Coding-Standards, Sicherheitsanforderungen und Projektkonventionen. Claude liest dies während der Ausführung und folgt Ihren Regeln.
2. **Benutzerdefinierte Prompts**: Übergeben Sie aufgabenspezifische Anweisungen über `prompt`/`prompt_file` im Job. Verwenden Sie unterschiedliche Prompts für verschiedene Jobs (zum Beispiel Review, Implementierung, Refactoring).
