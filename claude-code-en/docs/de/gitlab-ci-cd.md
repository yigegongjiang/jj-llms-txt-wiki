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

In einer GitLab-Pipeline kann Claude Code:

* MRs aus Issue-Beschreibungen oder Kommentaren erstellen und aktualisieren
* Leistungsregressionen analysieren und Optimierungen vorschlagen
* Funktionen direkt in einem Branch implementieren und dann eine MR öffnen
* Fehler und Regressionen beheben, die durch Tests oder Kommentare identifiziert wurden
* Auf Folgekommen antworten, um angeforderte Änderungen zu iterieren

<h2 id="setup">
  Einrichtung
</h2>

<h3 id="quick-setup">
  Schnelle Einrichtung
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
  # Passen Sie die Regeln an, um festzulegen, wie Sie den Job auslösen möchten:
  # - manuelle Ausführungen
  # - Merge-Request-Ereignisse
  # - Web-/API-Trigger, wenn ein Kommentar „@claude" enthält
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
  variables:
    GIT_STRATEGY: fetch
  before_script:
    - apk update
    - apk add --no-cache git curl bash
    - curl -fsSL https://claude.ai/install.sh | bash
    # Das Installationsprogramm platziert Claude in ~/.local/bin, das in diesem Image nicht auf PATH ist
    - export PATH="$HOME/.local/bin:$PATH"
  script:
    # Optional: Starten Sie einen GitLab-MCP-Server, wenn Ihr Setup einen bereitstellt
    - /bin/gitlab-mcp-server || true
    # Verwenden Sie AI_FLOW_*-Variablen beim Aufrufen über Web-/API-Trigger mit Kontext-Payloads
    - echo "$AI_FLOW_INPUT for $AI_FLOW_CONTEXT on $AI_FLOW_EVENT"
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Review this MR and implement the requested changes'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
```

Nachdem Sie den Job und Ihre `ANTHROPIC_API_KEY`-Variable hinzugefügt haben, testen Sie, indem Sie den Job manuell von **CI/CD** → **Pipelines** ausführen, oder lösen Sie ihn von einem MR aus, um Claude Aktualisierungen in einem Branch vorzuschlagen und bei Bedarf einen MR zu öffnen.

<Note>
  Um auf Amazon Bedrock oder Google Cloud's Agent Platform statt der Claude API auszuführen, siehe den Abschnitt [Verwendung mit Amazon Bedrock und Google Cloud](#using-with-amazon-bedrock-and-google-cloud) unten für Authentifizierung und Umgebungseinrichtung.
</Note>

<h3 id="manual-setup-recommended-for-production">
  Manuelle Einrichtung (empfohlen für Produktion)
</h3>

Wenn Sie eine kontrollierte Einrichtung bevorzugen oder Enterprise-Provider benötigen:

1. **Konfigurieren Sie den Provider-Zugriff**:
   * **Claude API**: Erstellen und speichern Sie `ANTHROPIC_API_KEY` als maskierte CI/CD-Variable
   * **Amazon Bedrock**: **Konfigurieren Sie GitLab** → **AWS OIDC** und erstellen Sie eine IAM-Rolle für Amazon Bedrock
   * **Google Cloud's Agent Platform**: **Konfigurieren Sie Workload Identity Federation für GitLab** → **GCP**

2. **Fügen Sie Projektanmeldedaten für GitLab-API-Operationen hinzu**:
   * Verwenden Sie `CI_JOB_TOKEN` standardmäßig, oder erstellen Sie ein Project Access Token mit `api`-Bereich
   * Speichern Sie als `GITLAB_ACCESS_TOKEN` (maskiert), wenn Sie ein PAT verwenden

3. **Fügen Sie den Claude-Job zu `.gitlab-ci.yml` hinzu**: Verwenden Sie den [Schnelle Einrichtung](#quick-setup)-Job für die Claude API, oder einen Provider-Job aus [Konfigurationsbeispiele](#configuration-examples)

4. **(Optional) Aktivieren Sie Mention-gesteuerte Trigger**:
   * Fügen Sie einen Projekt-Webhook für „Kommentare (Notizen)" zu Ihrem Event-Listener hinzu (falls Sie einen verwenden)
   * Lassen Sie den Listener die Pipeline-Trigger-API mit Variablen wie `AI_FLOW_INPUT` und `AI_FLOW_CONTEXT` aufrufen, wenn ein Kommentar `@claude` enthält

<h2 id="example-use-cases">
  Beispiele für Anwendungsfälle
</h2>

<h3 id="turn-issues-into-mrs">
  Probleme in Merge Requests umwandeln
</h3>

In einem Issue-Kommentar:

```text wrap theme={null}
@claude implement this feature based on the issue description
```

Claude analysiert das Issue und die Codebasis, schreibt Änderungen in einem Branch und öffnet einen MR zur Überprüfung.

<h3 id="get-implementation-help">
  Implementierungshilfe erhalten
</h3>

In einer MR-Diskussion:

```text wrap theme={null}
@claude suggest a concrete approach to cache the results of this API call
```

Claude schlägt Änderungen vor, fügt Code mit angemessenem Caching hinzu und aktualisiert den MR.

<h3 id="fix-bugs-quickly">
  Fehler schnell beheben
</h3>

In einem Issue- oder MR-Kommentar:

```text wrap theme={null}
@claude fix the TypeError in the user dashboard component
```

Claude lokalisiert den Fehler, implementiert eine Korrektur und aktualisiert den Branch oder öffnet einen neuen MR.

<h2 id="using-with-amazon-bedrock-and-google-cloud">
  Verwendung mit Amazon Bedrock und Google Cloud
</h2>

Für Unternehmensumgebungen können Sie Claude Code vollständig auf Ihrer Cloud-Infrastruktur mit der gleichen Entwicklererfahrung ausführen.

<Tabs>
  <Tab title="Amazon Bedrock">
    ### Voraussetzungen

    Bevor Sie Claude Code mit Amazon Bedrock einrichten, benötigen Sie:

    1. Ein AWS-Konto mit Amazon Bedrock-Zugriff auf die gewünschten Claude-Modelle
    2. GitLab, das als OIDC-Identitätsanbieter in AWS IAM konfiguriert ist
    3. Eine IAM-Rolle mit Amazon Bedrock-Berechtigungen und eine Vertrauensrichtlinie, die auf Ihr GitLab-Projekt/Ihre Refs beschränkt ist
    4. GitLab CI/CD-Variablen für die Rollenübernahme:
       * `AWS_ROLE_TO_ASSUME` (Rollen-ARN)
       * `AWS_REGION` (Amazon Bedrock-Region)

    ### Einrichtungsanweisungen

    Konfigurieren Sie AWS, um GitLab CI-Jobs zu ermöglichen, eine IAM-Rolle über OIDC anzunehmen (keine statischen Schlüssel).

    **Erforderliche Einrichtung:**

    1. Aktivieren Sie Amazon Bedrock und fordern Sie Zugriff auf Ihre Ziel-Claude-Modelle an
    2. Erstellen Sie einen IAM OIDC-Anbieter für GitLab, falls nicht bereits vorhanden
    3. Erstellen Sie eine IAM-Rolle, der der GitLab OIDC-Anbieter vertraut, beschränkt auf Ihr Projekt und geschützte Refs
    4. Fügen Sie Berechtigungen mit minimalen Rechten für Amazon Bedrock Invoke APIs an

    Verwenden Sie das [Amazon Bedrock-Job-Beispiel](#configuration-examples), um das OIDC-Token des Jobs zur Laufzeit gegen temporäre AWS-Anmeldedaten auszutauschen.
  </Tab>

  <Tab title="Google Cloud's Agent Platform">
    ### Voraussetzungen

    Bevor Sie Claude Code mit Google Cloud's Agent Platform einrichten, benötigen Sie:

    1. Ein Google Cloud-Projekt mit:
       * Aktivierter Google Cloud's Agent Platform API
       * Workload Identity Federation, das GitLab OIDC vertraut
    2. Ein dediziertes Dienstkonto mit nur den erforderlichen Google Cloud's Agent Platform-Rollen
    3. GitLab CI/CD-Variablen:
       * `GCP_WORKLOAD_IDENTITY_PROVIDER` (Anbieter-Ressourcenname ohne das Präfix `//iam.googleapis.com/`, z. B. `projects/123456789/locations/global/workloadIdentityPools/my-pool/providers/my-provider`)
       * `GCP_SERVICE_ACCOUNT` (E-Mail-Adresse des Dienstkontos)
       * `GCP_PROJECT_ID` (Google Cloud-Projekt-ID)

    ### Einrichtungsanweisungen

    Konfigurieren Sie Google Cloud, um GitLab CI-Jobs zu ermöglichen, ein Dienstkonto über Workload Identity Federation zu imitieren.

    **Erforderliche Einrichtung:**

    1. Aktivieren Sie IAM Credentials API, STS API und Google Cloud's Agent Platform API
    2. Erstellen Sie einen Workload Identity Pool und einen Anbieter für GitLab OIDC
    3. Erstellen Sie ein dediziertes Dienstkonto mit Google Cloud's Agent Platform-Rollen
    4. Gewähren Sie dem WIF-Principal die Berechtigung, das Dienstkonto zu imitieren

    Verwenden Sie das [Agent Platform-Job-Beispiel](#configuration-examples), um sich zu authentifizieren, ohne Schlüssel zu speichern.
  </Tab>
</Tabs>

<h2 id="configuration-examples">
  Konfigurationsbeispiele
</h2>

Nachfolgend finden Sie einsatzbereite Snippets, die Sie an Ihre Pipeline anpassen können.

<h3 id="amazon-bedrock-job-example-oidc">
  Amazon Bedrock-Auftragsbeispiel (OIDC)
</h3>

**Voraussetzungen:**

* Amazon Bedrock aktiviert mit Zugriff auf Ihr(e) gewählte(s) Claude-Modell(e)
* GitLab OIDC in AWS konfiguriert mit einer Rolle, die Ihr GitLab-Projekt und Refs vertraut
* IAM-Rolle mit Amazon Bedrock-Berechtigungen (Least-Privilege empfohlen)

**Erforderliche CI/CD-Variablen:**

* `AWS_ROLE_TO_ASSUME`: ARN der IAM-Rolle für Amazon Bedrock-Zugriff
* `AWS_REGION`: Amazon Bedrock-Region (zum Beispiel `us-west-2`)

GitLab erstellt das OIDC-Token des Auftrags aus dem `id_tokens:`-Block und stellt es als `GITLAB_OIDC_TOKEN` bereit. Setzen Sie `aud` auf den Zielgruppenwert, den Sie auf dem IAM OIDC-Identitätsanbieter in AWS konfiguriert haben, zum Beispiel Ihre GitLab-Instanz-URL.

```yaml theme={null}
stages:
  - ai

claude-bedrock:
  stage: ai
  image: node:24-alpine3.21
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  id_tokens:
    GITLAB_OIDC_TOKEN:
      aud: https://gitlab.example.com
  before_script:
    - apk add --no-cache bash curl jq git aws-cli
    - curl -fsSL https://claude.ai/install.sh | bash
    # The installer places claude in ~/.local/bin, which isn't on PATH in this image
    - export PATH="$HOME/.local/bin:$PATH"
    # Exchange the job's OIDC token for AWS credentials
    - export AWS_WEB_IDENTITY_TOKEN_FILE="/tmp/oidc_token"
    - printf "%s" "$GITLAB_OIDC_TOKEN" > "$AWS_WEB_IDENTITY_TOKEN_FILE"
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
    CLAUDE_CODE_USE_BEDROCK: "1"
```

<Note>
  Modell-IDs für Amazon Bedrock enthalten regionsspezifische Präfixe (zum Beispiel `us.anthropic.claude-sonnet-4-6`). Übergeben Sie das gewünschte Modell über Ihre Auftragskonfiguration oder Eingabeaufforderung, wenn Ihr Workflow dies unterstützt.
</Note>

<h3 id="agent-platform-job-example-workload-identity-federation">
  Agent Platform-Auftragsbeispiel (Workload Identity Federation)
</h3>

**Voraussetzungen:**

* Google Cloud's Agent Platform API in Ihrem GCP-Projekt aktiviert
* Workload Identity Federation konfiguriert, um GitLab OIDC zu vertrauen
* Ein Dienstkonto mit Google Cloud's Agent Platform-Berechtigungen

**Erforderliche CI/CD-Variablen:**

* `GCP_WORKLOAD_IDENTITY_PROVIDER`: Anbieter-Ressourcenname ohne das `//iam.googleapis.com/`-Präfix, wie `projects/123456789/locations/global/workloadIdentityPools/my-pool/providers/my-provider`
* `GCP_SERVICE_ACCOUNT`: E-Mail des Dienstkontos
* `GCP_PROJECT_ID`: Google Cloud-Projekt-ID
* `CLOUD_ML_REGION`: Google Cloud's Agent Platform-Region (zum Beispiel `us-east5`)

GitLab erstellt das OIDC-Token des Auftrags aus dem `id_tokens:`-Block und stellt es als `GITLAB_OIDC_TOKEN` bereit. Setzen Sie `aud` auf den Zielgruppenwert, den Sie auf dem Workload Identity Pool-Anbieter konfiguriert haben, zum Beispiel Ihre GitLab-Instanz-URL. Der Auftrag schreibt das Token in eine Datei, und der `credential_source`-Eintrag der Anmeldeinformationskonfiguration teilt Googles Auth-Bibliotheken mit, es von dort zu lesen. Das Setzen von `GOOGLE_APPLICATION_CREDENTIALS` auf die Anmeldeinformationskonfigurationsdatei macht sie für Claude Code über [Application Default Credentials](/docs/de/google-vertex-ai#3-configure-gcp-credentials) verfügbar.

```yaml theme={null}
stages:
  - ai

claude-vertex:
  stage: ai
  image: gcr.io/google.com/cloudsdktool/google-cloud-cli:slim
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  id_tokens:
    GITLAB_OIDC_TOKEN:
      aud: https://gitlab.example.com
  before_script:
    - apt-get update && apt-get install -y git && apt-get clean
    - curl -fsSL https://claude.ai/install.sh | bash
    # The installer places claude in ~/.local/bin, which isn't on PATH in this image
    - export PATH="$HOME/.local/bin:$PATH"
    # Write the job's OIDC token where credential_source expects it
    - printf "%s" "$GITLAB_OIDC_TOKEN" > /tmp/oidc_token
    # Write the WIF credential configuration to a file (no downloaded keys)
    - |
      cat > /tmp/cred.json <<EOF
      {
        "type": "external_account",
        "audience": "//iam.googleapis.com/${GCP_WORKLOAD_IDENTITY_PROVIDER}",
        "subject_token_type": "urn:ietf:params:oauth:token-type:jwt",
        "token_url": "https://sts.googleapis.com/v1/token",
        "credential_source": {
          "file": "/tmp/oidc_token"
        },
        "service_account_impersonation_url": "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/${GCP_SERVICE_ACCOUNT}:generateAccessToken"
      }
      EOF
    # Expose the credentials to Claude Code via Application Default Credentials
    - export GOOGLE_APPLICATION_CREDENTIALS=/tmp/cred.json
    # Authenticate the gcloud CLI with the same credential configuration
    - gcloud auth login --cred-file=/tmp/cred.json
    - gcloud config set project "$GCP_PROJECT_ID"
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
    CLAUDE_CODE_USE_VERTEX: "1"
    ANTHROPIC_VERTEX_PROJECT_ID: "$GCP_PROJECT_ID"
```

<Note>
  Mit Workload Identity Federation müssen Sie keine Dienstkontoschlüssel speichern. Verwenden Sie Repository-spezifische Vertrauensbedingungen und Dienstkonten mit Least-Privilege.
</Note>

<h2 id="best-practices">
  Best Practices
</h2>

<h3 id="claude-md-configuration">
  CLAUDE.md-Konfiguration
</h3>

Erstellen Sie eine `CLAUDE.md`-Datei im Repository-Root, um Coding-Standards, Review-Kriterien und projektspezifische Regeln zu definieren. Claude liest diese Datei während der Ausführung und befolgt Ihre Konventionen bei der Vorschlagung von Änderungen.

<h3 id="security-considerations">
  Sicherheitsaspekte
</h3>

**Committen Sie niemals API-Schlüssel oder Cloud-Anmeldedaten in Ihr Repository**. Verwenden Sie immer GitLab CI/CD-Variablen:

* Fügen Sie `ANTHROPIC_API_KEY` als maskierte Variable hinzu (und schützen Sie sie bei Bedarf)
* Verwenden Sie wo möglich anbieterspezifisches OIDC (keine langlebigen Schlüssel)
* Begrenzen Sie Job-Berechtigungen und Netzwerk-Egress
* Überprüfen Sie Claudes MRs wie jeden anderen Beitrag

<h3 id="optimizing-performance">
  Leistungsoptimierung
</h3>

* Halten Sie `CLAUDE.md` fokussiert und prägnant
* Geben Sie klare Issue-/MR-Beschreibungen an, um Iterationen zu reduzieren
* Cachen Sie npm und Paketinstallationen in Runnern, wo möglich

<h3 id="ci-costs">
  CI-Kosten
</h3>

Bei der Verwendung von Claude Code mit GitLab CI/CD sollten Sie sich der damit verbundenen Kosten bewusst sein:

* **GitLab Runner-Zeit**:
  * Claude läuft auf Ihren GitLab-Runnern und verbraucht Compute-Minuten
  * Weitere Informationen finden Sie in der Runner-Abrechnung Ihres GitLab-Plans

* **API-Kosten**:
  * Jede Claude-Interaktion verbraucht Token basierend auf der Größe von Prompt und Antwort
  * Die Token-Nutzung variiert je nach Aufgabenkomplexität und Codebase-Größe
  * Weitere Informationen finden Sie unter [Anthropic-Preisgestaltung](https://platform.claude.com/docs/en/about-claude/pricing)

* **Tipps zur Kostenoptimierung**:
  * Verwenden Sie spezifische `@claude`-Befehle, um unnötige Durchläufe zu reduzieren
  * Legen Sie angemessene `--max-turns`- und Job-`timeout`-Werte fest
  * Begrenzen Sie die Parallelität, um parallele Ausführungen zu kontrollieren

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude antwortet nicht auf @claude-Befehle
</h3>

* Überprüfen Sie, dass Ihre Pipeline ausgelöst wird (manuell, MR-Ereignis oder über einen Note-Ereignis-Listener/Webhook)
* Stellen Sie sicher, dass Ihre `ANTHROPIC_API_KEY` oder Cloud-Provider-Variablen vorhanden sind
* Überprüfen Sie, dass der Kommentar `@claude` enthält (nicht `/claude`) und dass Ihr Mention-Trigger konfiguriert ist

<h3 id="job-can’t-write-comments-or-open-mrs">
  Job kann keine Kommentare schreiben oder MRs öffnen
</h3>

* Stellen Sie sicher, dass `CI_JOB_TOKEN` ausreichende Berechtigungen für das Projekt hat, oder verwenden Sie ein Project Access Token mit `api`-Bereich
* Überprüfen Sie, dass das `mcp__gitlab`-Tool in `--allowedTools` aktiviert ist
* Bestätigen Sie, dass der Job im Kontext des MR ausgeführt wird oder über `AI_FLOW_*`-Variablen genügend Kontext hat

<h3 id="authentication-errors">
  Authentifizierungsfehler
</h3>

* **Für Claude API**: Bestätigen Sie, dass `ANTHROPIC_API_KEY` gültig und nicht abgelaufen ist
* **Für Amazon Bedrock oder Google Cloud's Agent Platform**: Überprüfen Sie die OIDC/WIF-Konfiguration, Rollenidentitätswechsel und Geheimnisnamen; bestätigen Sie Regionen- und Modellverfügbarkeit

<h2 id="advanced-configuration">
  Erweiterte Konfiguration
</h2>

<h3 id="common-parameters-and-variables">
  Häufige Parameter und Variablen
</h3>

Steuern Sie Claude Code-Ausführungen in Ihren Jobs mit diesen CLI-Flags, GitLab-Schlüsselwörtern und Variablen:

* `-p`: Anweisungen inline bereitstellen, zum Beispiel `claude -p "Review this MR"`
* `--max-turns`: Begrenzen Sie die Anzahl der Hin- und Herbewegungen
* `timeout`: Begrenzen Sie die gesamte Job-Ausführungszeit mit GitLabs Job-Level-Schlüsselwort `timeout`, zum Beispiel `timeout: 30m`
* `ANTHROPIC_API_KEY`: erforderlich für die Claude API (nicht verwendet für Amazon Bedrock oder Google Cloud's Agent Platform)
* Anbieter-spezifische Umgebung: `AWS_REGION`, Projekt-/Regionsvariablen für Google Cloud's Agent Platform

<Note>
  Genaue Flags und Parameter können je nach Version von `@anthropic-ai/claude-code` variieren. Führen Sie `claude --help` in Ihrem Job aus, um unterstützte Optionen anzuzeigen.
</Note>

<h3 id="customizing-claude’s-behavior">
  Anpassung des Verhaltens von Claude
</h3>

Sie können Claude auf zwei primäre Arten lenken:

1. **CLAUDE.md**: Definieren Sie Codierungsstandards, Sicherheitsanforderungen und Projektkonventionen. Claude liest dies während der Ausführungen und befolgt Ihre Regeln.
2. **Benutzerdefinierte Prompts**: Übergeben Sie aufgabenspezifische Anweisungen über `-p` im Job. Verwenden Sie unterschiedliche Prompts für verschiedene Jobs (zum Beispiel Review, Implementierung, Umgestaltung).
