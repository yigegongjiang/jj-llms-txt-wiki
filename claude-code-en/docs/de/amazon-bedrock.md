> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code auf Amazon Bedrock

> Erfahren Sie, wie Sie Claude Code über Amazon Bedrock konfigurieren, einschließlich Setup, IAM-Konfiguration und Fehlerbehebung.

export const ContactSalesCard = ({surface}) => {
  const utm = content => `utm_source=claude_code&utm_medium=docs&utm_content=${surface}_${content}`;
  const iconArrowRight = (size = 13) => <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
      <line x1="5" y1="12" x2="19" y2="12" />
      <polyline points="12 5 19 12 12 19" />
    </svg>;
  const STYLES = `
.cc-cs {
  --cs-slate: #141413;
  --cs-clay: #d97757;
  --cs-clay-deep: #c6613f;
  --cs-gray-000: #ffffff;
  --cs-gray-700: #3d3d3a;
  --cs-border-default: rgba(31, 30, 29, 0.15);
  font-family: inherit;
}
.dark .cc-cs {
  --cs-slate: #f0eee6;
  --cs-gray-000: #262624;
  --cs-gray-700: #bfbdb4;
  --cs-border-default: rgba(240, 238, 230, 0.14);
}
.cc-cs-card {
  display: flex; align-items: center; justify-content: space-between;
  gap: 16px; padding: 14px 16px; margin: 0;
  background: var(--cs-gray-000); border: 0.5px solid var(--cs-border-default);
  border-radius: 8px; flex-wrap: wrap;
}
.cc-cs-text { font-size: 13px; color: var(--cs-gray-700); line-height: 1.5; flex: 1; min-width: 240px; }
.cc-cs-text strong { font-weight: 550; color: var(--cs-slate); }
.cc-cs-actions { display: flex; align-items: center; gap: 8px; flex-shrink: 0; }
.cc-cs-btn-clay {
  display: inline-flex; align-items: center; gap: 8px;
  background: var(--cs-clay-deep); color: #fff; border: none;
  border-radius: 8px; padding: 8px 14px;
  font-size: 13px; font-weight: 500;
  transition: background-color 0.15s; white-space: nowrap;
}
.cc-cs-btn-clay:hover { background: var(--cs-clay); }
.cc-cs-btn-ghost {
  display: inline-flex; align-items: center; gap: 8px;
  background: transparent; color: var(--cs-gray-700);
  border: 0.5px solid var(--cs-border-default);
  border-radius: 8px; padding: 8px 14px;
  font-size: 13px; font-weight: 500;
}
.cc-cs-btn-ghost:hover { background: rgba(0, 0, 0, 0.04); }
.dark .cc-cs-btn-ghost:hover { background: rgba(255, 255, 255, 0.04); }
@media (max-width: 720px) {
  .cc-cs-actions { width: 100%; }
}
`;
  return <div className="cc-cs not-prose">
      <style>{STYLES}</style>
      <div className="cc-cs-card">
        <div className="cc-cs-text">
          <strong>Deploying Claude Code across your organization?</strong> Talk to sales about enterprise plans, SSO, and centralized billing.
        </div>
        <div className="cc-cs-actions">
          <a href={`https://claude.com/pricing?${utm('view_plans')}#plans-business`} className="cc-cs-btn-ghost">
            View plans
          </a>
          <a href={`https://claude.com/contact-sales?${utm('contact_sales')}`} className="cc-cs-btn-clay">
            Contact sales {iconArrowRight()}
          </a>
        </div>
      </div>
    </div>;
};

<ContactSalesCard surface="bedrock" />

<h2 id="prerequisites">
  Voraussetzungen
</h2>

Bevor Sie Claude Code mit Amazon Bedrock konfigurieren, stellen Sie sicher, dass Sie über Folgendes verfügen:

* Ein AWS-Konto mit aktiviertem Amazon Bedrock-Zugriff
* Zugriff auf gewünschte Claude-Modelle (z. B. Claude Sonnet 4.6) in Amazon Bedrock
* AWS CLI installiert und konfiguriert (optional – nur erforderlich, wenn Sie keinen anderen Mechanismus zur Beschaffung von Anmeldedaten haben)
* Angemessene IAM-Berechtigungen

Um sich mit Ihren eigenen Amazon Bedrock-Anmeldedaten anzumelden, folgen Sie [Mit Amazon Bedrock anmelden](#sign-in-with-bedrock) unten. Um Claude Code in einem Team bereitzustellen, verwenden Sie die Schritte zum [manuellen Setup](#set-up-manually) und [fixieren Sie Ihre Modellversionen](#4-pin-model-versions), bevor Sie ausrollen.

<h2 id="sign-in-with-bedrock">
  Mit Bedrock anmelden
</h2>

Wenn Sie AWS-Anmeldedaten haben und Claude Code über Amazon Bedrock nutzen möchten, führt Sie der Anmeldungs-Assistent durch den Prozess. Sie führen die Voraussetzungen auf AWS-Seite einmal pro Konto durch; der Assistent kümmert sich um die Claude Code-Seite.

<Steps>
  <Step title="Aktivieren Sie Anthropic-Modelle in Ihrem AWS-Konto">
    Öffnen Sie in der [Amazon Bedrock-Konsole](https://console.aws.amazon.com/bedrock/) den Modellkatalog, wählen Sie ein Anthropic-Modell aus und reichen Sie das Anwendungsfallformular ein. Der Zugriff wird unmittelbar nach der Einreichung gewährt. Siehe [Anwendungsfalldetails einreichen](#1-submit-use-case-details) für AWS Organizations und [IAM-Konfiguration](#iam-configuration) für die Berechtigungen, die Ihre Rolle benötigt.
  </Step>

  <Step title="Starten Sie Claude Code und wählen Sie Amazon Bedrock">
    Führen Sie `claude` aus. Wählen Sie bei der Anmeldungsaufforderung **3rd-party platform** und dann **Amazon Bedrock**. Wenn Sie bereits angemeldet sind und stattdessen die Chat-Aufforderung sehen, führen Sie `/setup-bedrock` aus, um den Assistenten zu öffnen. Bis `CLAUDE_CODE_USE_BEDROCK=1` gesetzt ist, blendet Claude Code [den Befehl aus dem Befehlsmenü aus](/docs/de/commands#how-the-command-menu-matches-what-you-type); geben Sie ihn vollständig ein.
  </Step>

  <Step title="Folgen Sie den Assistenten-Aufforderungen">
    Wählen Sie, wie Sie sich bei AWS authentifizieren: ein aus Ihrem `~/.aws`-Verzeichnis erkanntes AWS-Profil, einen Amazon Bedrock API-Schlüssel, einen Zugriffscode und ein Geheimnis oder Anmeldedaten, die bereits in Ihrer Umgebung vorhanden sind. Der Assistent fragt nach Ihrer Region, überprüft, welche Claude-Modelle Ihr Konto aufrufen kann, und ermöglicht es Ihnen, diese anzuheften. Das Ergebnis wird im `env`-Block Ihrer [Benutzereinstellungsdatei](/docs/de/settings) gespeichert, sodass Sie Umgebungsvariablen nicht selbst exportieren müssen.
  </Step>
</Steps>

Nach der Anmeldung führen Sie jederzeit `/setup-bedrock` aus, um den Assistenten erneut zu öffnen und Ihre Anmeldedaten, Region oder Modellanheftungen zu ändern. Der Schritt zum Anheften von Modellen beginnt mit Ihren derzeit angehefteten Modellen. Der Assistent schreibt in `~/.claude/settings.json` oder in `$CLAUDE_CONFIG_DIR/settings.json`, wenn [`CLAUDE_CONFIG_DIR`](/docs/de/env-vars#variables) gesetzt ist.

<h2 id="set-up-manually">
  Manuell einrichten
</h2>

Um Amazon Bedrock über Umgebungsvariablen statt über den Assistenten zu konfigurieren, beispielsweise in CI oder einem skriptgesteuerten Enterprise-Rollout, führen Sie die folgenden Schritte aus.

<h3 id="1-submit-use-case-details">
  1. Anwendungsfalldetails einreichen
</h3>

Bevor Sie ein Anthropic-Modell zum ersten Mal aufrufen, reichen Sie Anwendungsfalldetails ein. Sie tun dies einmal pro AWS-Konto.

1. Stellen Sie sicher, dass Sie die unten beschriebenen IAM-Berechtigungen haben
2. Navigieren Sie zur [Amazon Bedrock-Konsole](https://console.aws.amazon.com/bedrock/)
3. Wählen Sie ein Anthropic-Modell aus dem **Modellkatalog** aus
4. Füllen Sie das Anwendungsfallformular aus. Der Zugriff wird unmittelbar nach der Einreichung gewährt.

Wenn Sie AWS Organizations verwenden, können Sie das Formular einmal vom Management-Konto aus mit der [`PutUseCaseForModelAccess`-API](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_PutUseCaseForModelAccess.html) einreichen. Dieser Aufruf erfordert die `bedrock:PutUseCaseForModelAccess`-IAM-Berechtigung. Die Genehmigung erstreckt sich automatisch auf untergeordnete Konten.

<h3 id="2-configure-aws-credentials">
  2. AWS-Anmeldedaten konfigurieren
</h3>

Claude Code verwendet die Standard-AWS-SDK-Anmeldedatenkette. Richten Sie Ihre Anmeldedaten mit einer dieser Methoden ein:

**Option A: AWS CLI-Konfiguration**

```bash theme={null}
aws configure
```

**Option B: Umgebungsvariablen (Zugriffsschlüssel)**

```bash theme={null}
export AWS_ACCESS_KEY_ID=your-access-key-id
export AWS_SECRET_ACCESS_KEY=your-secret-access-key
export AWS_SESSION_TOKEN=your-session-token
```

**Option C: Umgebungsvariablen (SSO-Profil)**

Ersetzen Sie `your-profile-name` durch den Namen Ihres AWS-Profils, bevor Sie diese Befehle ausführen.

```bash theme={null}
aws sso login --profile=your-profile-name

export AWS_PROFILE=your-profile-name
```

Claude Code fordert Rollenzugriffsdaten aus der IAM Identity Center-Region an, die vom `sso_region` des Profils benannt wird, was nicht mit der Region übereinstimmen muss, in der Sie Amazon Bedrock ausführen. In v2.1.207 hat die Amazon Bedrock-Region `sso_region` überschrieben, sodass ein Profil, dessen IAM Identity Center-Instanz sich in einer anderen Region befindet, die Authentifizierung mit einem `Session token not found or invalid`-Fehler nicht durchführen konnte.

**Option D: AWS Management Console-Anmeldedaten**

```bash theme={null}
aws login
```

[Weitere Informationen](https://docs.aws.amazon.com/signin/latest/userguide/command-line-sign-in.html) zu `aws login`.

**Option E: Amazon Bedrock API-Schlüssel**

```bash theme={null}
export AWS_BEARER_TOKEN_BEDROCK=your-bedrock-api-key
```

Amazon Bedrock API-Schlüssel bieten eine einfachere Authentifizierungsmethode ohne vollständige AWS-Anmeldedaten. [Weitere Informationen zu Amazon Bedrock API-Schlüsseln](https://aws.amazon.com/blogs/machine-learning/accelerate-ai-development-with-amazon-bedrock-api-keys/).

<h4 id="credential-caching-and-resolution-timeout">
  Anmeldedaten-Caching und Auflösungs-Timeout
</h4>

Claude Code löst die AWS-Standard-Anmeldedatenkette einmal auf und behält die aufgelösten Anmeldedaten im Speicher. Es verwendet sie erneut, bis fünf Minuten vor ihrem Ablauf, oder für eine Stunde, wenn sie kein Ablaufdatum haben, sodass ein SSO-gestütztes Profil etwa einmal pro Anmeldedaten-Lebensdauer Anmeldedaten von IAM Identity Center anfordert. Ein Anmeldedatenfehler von der API löscht den Cache, und der Wiederholungsversuch löst neue Anmeldedaten auf. Erfordert Claude Code v2.1.207 oder später.

Der Cache deckt alle oben genannten Anmeldedatenoptionen ab, außer einem Amazon Bedrock API-Schlüssel, der die Anbieterkette nicht verwendet. Um die Kette bei jeder Anfrage aufzulösen, setzen Sie stattdessen [`CLAUDE_CODE_SKIP_AWS_CRED_CACHE=1`](/docs/de/env-vars).

Jede Auflösung der Kette läuft nach 60 Sekunden ab. Wenn ein Schritt in der Kette steckenbleibt, beispielsweise ein `credential_process`-Helfer, der auf eine Eingabe wartet, die er nicht erhalten kann, schlägt die Anfrage mit [`AWS default-chain credential resolve timed out`](/docs/de/errors#aws-default-chain-credential-resolve-timed-out) fehl. Wenn Ihre Kette eine interaktive Anmeldung ausführt, die legitim länger dauert, z. B. browsergestützte SSO mit MFA über einen Wrapper wie `aws-vault`, erhöhen Sie das Limit in Millisekunden mit [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/de/env-vars). Vor v2.1.207 ließ eine steckengebliebene Anmeldedatenauflösung die Anfrage auf unbestimmte Zeit warten.

Außer wenn Sie sich mit einem Amazon Bedrock API-Schlüssel authentifizieren, wendet der [Setup-Assistent](#sign-in-with-bedrock) das gleiche Limit auf jeden AWS-Aufruf an, den er bei der Überprüfung Ihrer Anmeldedaten durchführt, und auf die Anmeldedaten-Suche vor jeder Modellprüfung. Während der Anmeldedaten-Überprüfung schlägt eine Prüfung, die das Limit überschreitet, mit [`Timed out after 60s waiting for AWS`](/docs/de/errors#bedrock-setup-verification-timed-out-waiting-for-aws) fehl.

<h4 id="advanced-credential-configuration">
  Erweiterte Anmeldedatenkonfiguration
</h4>

Claude Code unterstützt die automatische Anmeldedaten-Aktualisierung für AWS SSO und Unternehmensidentitätsanbieter. Fügen Sie diese Einstellungen zu Ihrer Claude Code-Einstellungsdatei hinzu (siehe [Einstellungen](/docs/de/settings) für Dateispeicherorte).

Diese beiden Einstellungen haben unterschiedliche Auslösebedingungen:

* **`awsAuthRefresh`**: wird nur ausgeführt, wenn Claude Code erkennt, dass Ihre AWS-Anmeldedaten abgelaufen sind, entweder lokal basierend auf ihrem Zeitstempel oder wenn die API einen Anmeldedatenfehler zurückgibt, und versucht dann erneut, die Anfrage mit aktualisierten Anmeldedaten zu stellen.
* **`awsCredentialExport`**: wird beim Sitzungsstart und bei jeder Anmeldedaten-Neuladeung ausgeführt, auch wenn die Anmeldedaten in Ihrer AWS-Standard-Anmeldedatenkette noch gültig sind. Verwenden Sie dies, wenn Ihr Amazon Bedrock-Konto kontoübergreifende Anmeldedaten erfordert, die sich von denen unterscheiden, die die Standard-Anbieterkette auflösen würde.

Bevor Claude Code den `awsAuthRefresh`-Befehl ausführt, führt es einen STS-`GetCallerIdentity`-Aufruf durch, um zu bestätigen, dass Ihre Anmeldedaten tatsächlich abgelaufen sind, und überspringt den Befehl, wenn sie noch funktionieren. Claude Code sendet diese Überprüfung durch Ihre [Proxy-Konfiguration](/docs/de/network-config#proxy-configuration) und berücksichtigt `HTTPS_PROXY` und `NO_PROXY`. Vor v2.1.239 sendete Claude Code diese Überprüfung direkt und hängte beim Startup in Netzwerken, die nur Ausgang durch einen Proxy zulassen.

<h5 id="example-configuration">
  Beispielkonfiguration
</h5>

```json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile myprofile",
  "env": {
    "AWS_PROFILE": "myprofile"
  }
}
```

<h5 id="configuration-settings-explained">
  Konfigurationseinstellungen erklärt
</h5>

**`awsAuthRefresh`**: Verwenden Sie dies für Befehle, die das `.aws`-Verzeichnis ändern, z. B. zum Aktualisieren von Anmeldedaten, SSO-Cache oder Konfigurationsdateien. Die Ausgabe des Befehls wird dem Benutzer angezeigt, aber interaktive Eingabe wird nicht unterstützt. Dies funktioniert gut für browsergestützte SSO-Flows, bei denen die CLI eine URL oder einen Code anzeigt und Sie die Authentifizierung im Browser abschließen.

**`awsCredentialExport`**: Verwenden Sie dies nur, wenn Sie `.aws` nicht ändern können und Anmeldedaten direkt zurückgeben müssen. Die Ausgabe wird stillschweigend erfasst und dem Benutzer nicht angezeigt. Der Befehl muss JSON in diesem Format ausgeben:

```json theme={null}
{
  "Credentials": {
    "AccessKeyId": "value",
    "SecretAccessKey": "value",
    "SessionToken": "value",
    "Expiration": "2026-01-01T00:00:00Z"
  }
}
```

Die flache Ausgabe von `aws configure export-credentials --format process` wird ebenfalls akzeptiert, mit denselben Schlüsseln auf der obersten Ebene statt verschachtelt unter `Credentials`.

`Expiration` ist optional. Wenn der Befehl ein gültiges ISO 8601-`Expiration` zurückgibt, speichert Claude Code die Anmeldedaten im Cache, bis fünf Minuten vor dieser Zeit. Ohne es werden Anmeldedaten für eine Stunde zwischengespeichert.

Wenn Sie `awsCredentialExport` ohne `awsAuthRefresh` konfigurieren, verwendet Claude Code die exportierten Anmeldedaten direkt und löst die AWS-Standard-Anmeldedatenkette beim Startup nicht erneut auf. Erfordert Claude Code v2.1.206 oder später.

<h3 id="3-configure-claude-code">
  3. Claude Code konfigurieren
</h3>

Legen Sie die folgenden Umgebungsvariablen fest, um Amazon Bedrock zu aktivieren:

```bash theme={null}
# Enable Bedrock integration
export CLAUDE_CODE_USE_BEDROCK=1
export AWS_REGION=us-east-1  # optional if your AWS profile already sets a region

# Optional: Override the AWS region for the small/fast model (Bedrock and Mantle).
# On Bedrock, has no effect without ANTHROPIC_DEFAULT_HAIKU_MODEL
# or the deprecated ANTHROPIC_SMALL_FAST_MODEL set.
export ANTHROPIC_SMALL_FAST_MODEL_AWS_REGION=us-west-2

# Optional: Override the Bedrock endpoint URL for custom endpoints or gateways
# export ANTHROPIC_BEDROCK_BASE_URL=https://bedrock-runtime.us-east-1.amazonaws.com
```

Beachten Sie beim Aktivieren von Amazon Bedrock für Claude Code Folgendes:

* Sie müssen nur `AWS_REGION` setzen, um die Region Ihres AWS-Profils zu überschreiben oder wenn Ihr Profil keine Region hat. Claude Code löst die Region in dieser Reihenfolge auf:

  * `AWS_REGION`
  * `AWS_DEFAULT_REGION`
  * die `region`, die auf Ihrem aktiven AWS-Profil gesetzt ist, gelesen aus der AWS-Anmeldedatendatei zuerst und dann aus der gemeinsamen Konfigurationsdatei, wobei die AWS SDK-Priorität übereinstimmt
  * `us-east-1`

  Wenn ein Wert aus einer dieser Quellen nicht wie ein Regionsname aussieht, behandelt Claude Code ihn als nicht gesetzt und setzt die Reihenfolge fort. Beispielsweise behandelt Claude Code einen Wert, der einen Schrägstrich, Punkt oder Leerzeichen enthält, als nicht gesetzt.

  Das aktive Profil ist `AWS_PROFILE`, falls gesetzt, andernfalls `default`. Setzen Sie `AWS_SHARED_CREDENTIALS_FILE` oder `AWS_CONFIG_FILE`, um auf nicht standardmäßige Dateipfade zu verweisen.

  Führen Sie `/status` aus, um die aufgelöste Region anzuzeigen. Wenn die Region aus Ihren AWS-Konfigurationsdateien oder dem Standard-Fallback stammt, notiert Claude Code auch die Quelle in der `/status`-Ausgabe.
* Bei Verwendung von Amazon Bedrock ist der `/logout`-Befehl nicht verfügbar, da die Authentifizierung über AWS-Anmeldedaten erfolgt.
* Das WebSearch-Tool ist auf Amazon Bedrock nicht verfügbar. Siehe [WebSearch-Tool-Verhalten](/docs/de/tools-reference#websearch-tool-behavior).
* Sie können Einstellungsdateien für Umgebungsvariablen wie `AWS_PROFILE` verwenden, die Sie nicht an andere Prozesse weitergeben möchten. Siehe [Einstellungen](/docs/de/settings) für weitere Informationen.

<h3 id="4-pin-model-versions">
  4. Modellversionen anheften
</h3>

<Warning>
  Heften Sie spezifische Modellversionen an, wenn Sie für mehrere Benutzer bereitstellen. Ohne Anheften werden Modellaliase wie `sonnet` und `opus` zu Claude Codes integriertem Standard für Amazon Bedrock aufgelöst, der hinter der neuesten Version zurückbleiben kann und möglicherweise noch nicht in Ihrem Konto verfügbar ist. Claude Code [fällt zurück](#startup-model-checks) beim Startup auf ein früheres oder niedrigeres Modell zurück, wenn der Standard nicht verfügbar ist, aber das Anheften ermöglicht es Ihnen, zu kontrollieren, wann Ihre Benutzer zu einem neuen Modell wechseln.
</Warning>

Legen Sie diese Umgebungsvariablen auf spezifische Amazon Bedrock-Modell-IDs fest.

Ohne `ANTHROPIC_DEFAULT_OPUS_MODEL` wird der `opus`-Alias auf Amazon Bedrock zu Opus 5.5 aufgelöst, und ohne `ANTHROPIC_DEFAULT_SONNET_MODEL` wird der `sonnet`-Alias zu Sonnet 4.5 aufgelöst. Dieses Beispiel heftet jeden Alias an eine spezifische Version an:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='us.anthropic.claude-sonnet-4-6'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='us.anthropic.claude-haiku-4-5-20251001-v1:0'
```

Diese IDs verwenden das Präfix des `us.`-Regions-Inferenzprofils. Wenn Sie ein anderes Regionspräfix oder Anwendungs-Inferenzprofile verwenden, passen Sie entsprechend an. In AWS GovCloud-Regionen verwenden Sie das `us-gov.`-Präfix.

Um die integrierten Standard-Modelle beizubehalten und nur ihr bevorzugtes Präfix zu ändern, setzen Sie stattdessen [`ANTHROPIC_BEDROCK_REGION_PREFIX`](#cross-region-inference-profile-prefixes). Der Unterschied zeigt sich darin, worauf der `opus`-Alias aufgelöst wird:

| Sie setzen                                                    | Der `opus`-Alias wird aufgelöst zu                                                    |
| :------------------------------------------------------------ | :------------------------------------------------------------------------------------ |
| `ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'` | `us.anthropic.claude-opus-4-8`, die genaue ID, die Sie angeheftet haben               |
| `ANTHROPIC_BEDROCK_REGION_PREFIX=eu`                          | `eu.anthropic.claude-opus-5-5`, der integrierte Standard mit Ihrem bevorzugten Präfix |

Aktuelle und ältere Modell-IDs finden Sie unter [Modellübersicht](https://platform.claude.com/docs/en/about-claude/models/overview). Die vollständige Liste der Anheft-Umgebungsvariablen finden Sie unter [Modellkonfiguration](/docs/de/model-config#pin-models-for-third-party-deployments).

Claude Code verwendet diese Standard-Modelle, wenn keine Anheft-Variablen gesetzt sind:

| Modelltyp                | Standard-Modell                                                                                  |
| :----------------------- | :----------------------------------------------------------------------------------------------- |
| Primäres Modell          | Opus 5.5, beispielsweise `us.anthropic.claude-opus-5-5` in einer `us-*`-Region                   |
| Kleines/schnelles Modell | Sonnet 4.5, beispielsweise `us.anthropic.claude-sonnet-4-5-20250929-v1:0` in einer `us-*`-Region |

Hintergrundaufgaben wie die Generierung von Sitzungstiteln verwenden das kleine/schnelle Modell, normalerweise ein Haiku-Klasse-Modell. Auf Amazon Bedrock verwendet Claude Code das Standard-Sonnet-Modell für Hintergrundaufgaben, da Haiku möglicherweise nicht in jedem Konto oder jeder Region aktiviert ist. Zwei Auswahlen ändern, welches Modell sie trägt:

* Wenn Sie ein primäres Modell mit `--model`, `ANTHROPIC_MODEL` oder der `model`-Einstellung auswählen, verwenden Hintergrundaufgaben dieses Modell. Wenn Claude Code die Sitzung auf dem Modell startet, das Sie mit [`ANTHROPIC_DEFAULT_MODEL`](/docs/de/model-config#set-a-default-model-for-new-sessions) gesetzt haben, verwenden Hintergrundaufgaben auch dieses Modell. Das Setzen von `ANTHROPIC_DEFAULT_OPUS_MODEL` ohne `ANTHROPIC_DEFAULT_SONNET_MODEL` zählt auch als Auswahl, da das integrierte Sonnet-Modell möglicherweise nicht in einem Konto aktiviert ist, das sein eigenes Opus steuert.
* Um Haiku für Hintergrundaufgaben zu verwenden, setzen Sie `ANTHROPIC_DEFAULT_HAIKU_MODEL` auf eine Modell-ID, die in Ihrem Konto verfügbar ist.

<Warning>
  Opus-Modelle haben einen höheren Pro-Token-Preis als Sonnet-Modelle, daher wird eine Bereitstellung, die kein primäres Modell anheftet, ab v2.1.207 oder später zum Opus-Satz abgerechnet. Um Sonnet 4.5 als primäres Modell beizubehalten, setzen Sie `ANTHROPIC_MODEL` auf seine vollständige Modell-ID. Eine Bereitstellung, die den Standard mit `ANTHROPIC_DEFAULT_SONNET_MODEL` steuert und `ANTHROPIC_DEFAULT_OPUS_MODEL` nicht setzt, behält ihr gesteuertes Sonnet-Modell als Standard.
</Warning>

Vor v2.1.280 war das primäre Modell auf Amazon Bedrock standardmäßig Opus 5 und der `opus`-Alias wurde zu Opus 5 ab v2.1.219 aufgelöst. In v2.1.207 bis v2.1.218 war das primäre Modell auf Amazon Bedrock standardmäßig Opus 4.8 und der `opus`-Alias wurde zu Opus 4.8 aufgelöst. Vor v2.1.207 war das primäre Modell standardmäßig Sonnet 4.5, der `opus`-Alias wurde zu Opus 4.6 aufgelöst, und Hintergrundaufgaben verwendeten immer das primäre Modell.

Um Modelle weiter anzupassen, verwenden Sie eine dieser Methoden:

```bash theme={null}
# Using inference profile ID
export ANTHROPIC_MODEL='us.anthropic.claude-sonnet-4-6'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='us.anthropic.claude-haiku-4-5-20251001-v1:0'

# Using application inference profile ARN
export ANTHROPIC_MODEL='arn:aws:bedrock:us-east-2:your-account-id:application-inference-profile/your-model-id'

# Optional: Disable prompt caching if needed
# export DISABLE_PROMPT_CACHING=1

# Optional: Request 1-hour prompt cache TTL instead of the 5-minute default
# export ENABLE_PROMPT_CACHING_1H=1
```

Die 1-Stunden-Cache-TTL wird zu einem höheren Satz als der 5-Minuten-Standard abgerechnet. Siehe [Cache-Lebensdauer](/docs/de/prompt-caching#cache-lifetime). Um unterschiedliche TTLs für Ihre Hauptkonversation und für die Anfragen festzulegen, die Claude Code außerhalb davon stellt, [wählen Sie die TTL selbst](/docs/de/prompt-caching#choose-the-ttl-yourself).

<Note>Prompt Caching ist möglicherweise nicht in allen Amazon Bedrock-Regionen verfügbar. Wenn die Cache-Token-Zählungen bei Null bleiben, überprüfen Sie [unterstützte Modelle, Regionen und Limits](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models) in der Amazon Bedrock-Dokumentation.</Note>

<h4 id="map-each-model-version-to-an-inference-profile">
  Jede Modellversion einem Inferenzprofil zuordnen
</h4>

Die `ANTHROPIC_DEFAULT_*_MODEL`-Umgebungsvariablen konfigurieren ein Inferenzprofil pro Modellfamilie. Wenn Ihre Organisation mehrere Versionen derselben Familie in der `/model`-Auswahl verfügbar machen muss, die jeweils zu ihrer eigenen Anwendungs-Inferenzprofil-ARN weitergeleitet werden, verwenden Sie stattdessen die `modelOverrides`-Einstellung in Ihrer [Einstellungsdatei](/docs/de/settings#where-settings-live).

Dieses Beispiel ordnet vier Opus-Versionen unterschiedlichen ARNs zu, damit Benutzer zwischen ihnen wechseln können, ohne die Inferenzprofile Ihrer Organisation zu umgehen:

```json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-7": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-47-prod",
    "claude-opus-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-46-prod",
    "claude-opus-4-5-20251101": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-45-prod",
    "claude-opus-4-1-20250805": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-41-prod"
  }
}
```

Wenn ein Benutzer eine dieser Versionen in `/model` auswählt, ruft Claude Code Amazon Bedrock mit der zugeordneten ARN auf. Die gleiche Zuordnung gilt, wenn Sie die Anthropic-Modell-ID direkt über `--model` oder `ANTHROPIC_MODEL` übergeben. Versionen ohne Überschreibung fallen auf die integrierte Amazon Bedrock-Modell-ID oder ein passendes Inferenzprofil zurück, das beim Startup erkannt wird. Vor v2.1.200 erreichten `--model`- und `ANTHROPIC_MODEL`-Werte Amazon Bedrock unverändert, ohne die Überschreibungskarte zu durchlaufen. Siehe [Modell-IDs pro Version überschreiben](/docs/de/model-config#override-model-ids-per-version) für Details, wie Überschreibungen mit `availableModels` und anderen Modelleinstellungen interagieren.

<h2 id="startup-model-checks">
  Startmodellprüfungen
</h2>

Wenn Claude Code mit Amazon Bedrock konfiguriert startet, überprüft es, ob die Modelle, die es verwenden möchte, in Ihrem Konto verfügbar sind.

Wenn Sie eine ältere Modellversion angeheftet haben als die aktuelle Claude Code-Standardversion, und Ihr Konto die neuere Version aufrufen kann, fordert Claude Code Sie auf, die Anheftung zu aktualisieren. Wenn Sie akzeptieren, wird die neue Modell-ID in Ihre [Benutzereinstellungsdatei](/docs/de/settings) geschrieben und Claude Code wird neu gestartet. Wenn Sie ablehnen, wird dies bis zur nächsten Standardversionänderung beibehalten. Anheftungen, die auf ein [Anwendungs-Inferenzprofil-ARN](#map-each-model-version-to-an-inference-profile) verweisen, werden übersprungen, da diese von Ihrem Administrator verwaltet werden.

Wenn Sie kein Modell angeheftet haben und der aktuelle Standard in Ihrem Konto nicht verfügbar ist, greift Claude Code für die aktuelle Sitzung zurück und zeigt einen Hinweis an. Es versucht zuerst frühere Versionen des Standardmodells und greift, wenn der Standard ein Opus-Modell ist und keine Opus-Version verfügbar ist, auf das Standard-Sonnet-Modell zurück. Der Fallback wird nicht beibehalten. Aktivieren Sie das neuere Modell in Ihrem Amazon Bedrock-Konto oder [heften Sie eine Version an](#4-pin-model-versions), um die Auswahl dauerhaft zu machen.

Wenn Sie die Sitzung auf einer bestimmten Sonnet- oder Opus-Version starten, beispielsweise mit `--model`, `ANTHROPIC_MODEL` oder der [`model`-Einstellung](/docs/de/settings-reference#model), fungiert diese Version als Standard mit Anheftung der Sitzung für den entsprechenden `sonnet`- oder `opus`-Alias. Claude Code überspringt die Verfügbarkeitsprüfung für den integrierten Standard, den Ihr Modell ersetzt, und startet auf dem von Ihnen konfigurierten Modell, ohne Fallback-Hinweis.

Modellaliase wie `opus` fungieren nicht als Anheftungen, und auch nicht eine Modell-ID, die Claude Code nicht erkennt, wie beispielsweise ein Anwendungs-Inferenzprofil-ARN.

<h2 id="cross-region-inference-profile-prefixes">
  Präfixe für regionsübergreifende Inferenzprofile
</h2>

In der Amazon Bedrock [Invoke API](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_InvokeModelWithResponseStream.html) löst Claude Code seine integrierten Standardmodelle in [regionsübergreifende Inferenzprofil](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html)-IDs auf; um Modellversionen stattdessen über Ihre eigenen Inferenzprofile zu leiten, siehe [Jede Modellversion einem Inferenzprofil zuordnen](#map-each-model-version-to-an-inference-profile). Diese Tabelle zeigt das Präfix, das Claude Code für jede aufgelöste AWS-Region bevorzugt:

| AWS-Region                | Präfix    |
| :------------------------ | :-------- |
| `us-gov-*` (AWS GovCloud) | `us-gov.` |
| `us-*`                    | `us.`     |
| `eu-*`                    | `eu.`     |
| `ap-*`                    | `apac.`   |
| Alle anderen Regionen     | `global.` |

Setzen Sie `ANTHROPIC_BEDROCK_REGION_PREFIX`, um das Präfix auszuwählen, das Claude Code zuerst versucht; wenn Claude Code die Verfügbarkeit des Profils überprüfen kann und kein passendes Profil für ein Modell findet, wird es wie in der unten beschriebenen Auflösungsreihenfolge zurückgestuft. Gültige Werte sind `us`, `eu`, `apac`, `jp`, `au` und `global`. Setzen Sie es beispielsweise auf `global`, wenn Ihr Konto `global.`-Profile aktiviert hat, Claude Code aber ein geografiespezifisches Profil von Ihrer AWS-Region ableiten würde. Erfordert Claude Code v2.1.224 oder später.

Dieses Beispiel leitet die Standardmodelle über `global.`-Profile:

```bash theme={null}
export ANTHROPIC_BEDROCK_REGION_PREFIX=global
# In einer us-* Region wird das primäre Modell jetzt zu
# global.anthropic.claude-opus-5-5 statt us.anthropic.claude-opus-5-5 aufgelöst
```

Das bevorzugte Präfix ist eine Präferenz, keine Garantie, unabhängig davon, ob es von Ihrer Region oder von der Variablen stammt. Wie Claude Code es anwendet, hängt davon ab, ob es die Verfügbarkeit des Profils in Ihrem Konto überprüfen kann:

* Wenn Claude Code die [Inferenzprofile in Ihrem Konto auflisten](#iam-configuration) kann, löst es jedes Modell in dieser Reihenfolge auf:
  1. Das Profil mit Ihrem bevorzugten Präfix.
  2. Jedes passende Profil für ein Modell, das kein Profil mit diesem Präfix hat.
  3. Die integrierte Modell-ID mit Ihrem bevorzugten Präfix für ein Modell, das überhaupt kein passendes Profil hat. Claude Code wendet diese ID ohne Verfügbarkeitsprüfung in diesem Schritt an; die [Startmodell-Überprüfungen](#startup-model-checks) decken immer noch die Standardmodelle der Sitzung ab.
* Wenn die Profilermittlung nicht verfügbar ist, wendet Claude Code das Präfix ohne Verfügbarkeitsprüfung an. Wenn Ihr Konto keine Inferenzprofile mit diesem Präfix aktiviert hat, schlagen Anfragen mit einem 400-Fehler fehl.

Claude Code schreibt Amazon Bedrock-Inferenzprofil-IDs oder ARNs, die Sie selbst konfigurieren, oder [`modelOverrides`](#map-each-model-version-to-an-inference-profile)-Werte nicht um; Anthropic-Format-Modell-IDs werden durch [die gleiche Zuordnung wie die `/model`-Auswahl](#map-each-model-version-to-an-inference-profile) aufgelöst. Claude Code ignoriert die Variable auch in zwei Fällen:

* In AWS GovCloud-Regionen verwendet Claude Code immer `us-gov.`, das einzige Präfix, das innerhalb der GovCloud-Partition leitet.
* Wenn Sie einen Wert setzen, der nicht einer der gültigen Werte ist, wird Claude Code auf das von der Region abgeleitete bevorzugte Präfix zurückgestuft.

<h2 id="iam-configuration">
  IAM-Konfiguration
</h2>

Erstellen Sie eine IAM-Richtlinie mit den erforderlichen Berechtigungen für Claude Code:

```json theme={null}
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "AllowModelAndInferenceProfileAccess",
      "Effect": "Allow",
      "Action": [
        "bedrock:InvokeModel",
        "bedrock:InvokeModelWithResponseStream",
        "bedrock:ListInferenceProfiles",
        "bedrock:GetInferenceProfile"
      ],
      "Resource": [
        "arn:aws:bedrock:*:*:inference-profile/*",
        "arn:aws:bedrock:*:*:application-inference-profile/*",
        "arn:aws:bedrock:*:*:foundation-model/*"
      ]
    },
    {
      "Sid": "AllowMarketplaceSubscription",
      "Effect": "Allow",
      "Action": [
        "aws-marketplace:ViewSubscriptions",
        "aws-marketplace:Subscribe"
      ],
      "Resource": "*",
      "Condition": {
        "StringEquals": {
          "aws:CalledViaLast": "bedrock.amazonaws.com"
        }
      }
    }
  ]
}
```

Für restriktivere Berechtigungen können Sie die Ressource auf spezifische Inferenzprofil-ARNs beschränken.

`bedrock:GetInferenceProfile` ermöglicht es Claude Code, eine [Anwendungs-Inferenzprofil-ARN](#map-each-model-version-to-an-inference-profile) in ihr zugrunde liegendes Foundation-Modell aufzulösen, das verwendet wird, um die richtige Anforderungsform für dieses Modell auszuwählen.

Wenn dem Token diese Berechtigung fehlt, wird Claude Code automatisch wiederhergestellt, indem es einmal mit der alternativen Form erneut versucht wird, sodass Anfragen weiterhin erfolgreich sind, aber jedes neue Modell einen zusätzlichen Roundtrip hinzufügt. Die Gewährung der Berechtigung vermeidet den Wiederholungsversuch. Dies gilt am häufigsten für `AWS_BEARER_TOKEN_BEDROCK`-Bereitstellungen, bei denen die Richtlinie des Tokens typischerweise enger ist als eine vollständige IAM-Rolle.

Weitere Details finden Sie in der [Bedrock IAM-Dokumentation](https://docs.aws.amazon.com/bedrock/latest/userguide/security-iam.html).

<Note>
  Erstellen Sie ein dediziertes AWS-Konto für Claude Code, um die Kostenverfolgung und Zugriffskontrolle zu vereinfachen.
</Note>

<h2 id="1m-token-context-window">
  1M Token-Kontextfenster
</h2>

Claude Sonnet 5, Opus 4.6 und später sowie Sonnet 4.6 unterstützen das [1M Token-Kontextfenster](https://platform.claude.com/docs/de/build-with-claude/context-windows#context-window-sizes-by-model) auf Amazon Bedrock. Sonnet 5 wird immer mit dem 1M-Fenster sowohl über die Invoke API als auch über den [Mantle-Endpunkt](#use-the-mantle-endpoint) ausgeführt, ohne dass eine `[1m]`-Variante ausgewählt werden kann. Bei den anderen Modellen auf der Invoke API aktiviert Claude Code automatisch das erweiterte Kontextfenster, wenn Sie eine 1M-Modellvariante auswählen.

Der [Setup-Assistent](#sign-in-with-bedrock) bietet eine 1M-Kontextoption, wenn er Modelle fixiert. Um es stattdessen für ein manuell fixiertes Modell zu aktivieren, hängen Sie `[1m]` an die Modell-ID an. Siehe [Modelle für Drittanbieter-Bereitstellungen fixieren](/docs/de/model-config#pin-models-for-third-party-deployments) für Details.

<h2 id="service-tiers">
  Service-Tiers
</h2>

[Amazon Bedrock Service-Tiers](https://docs.aws.amazon.com/bedrock/latest/userguide/service-tiers-inference.html) ermöglichen es Ihnen, Kosten gegen Latenz abzuwägen. Legen Sie `ANTHROPIC_BEDROCK_SERVICE_TIER` auf `default`, `flex` oder `priority` fest:

```bash theme={null}
export ANTHROPIC_BEDROCK_SERVICE_TIER=priority
```

Claude Code sendet dies als `X-Amzn-Bedrock-Service-Tier`-Header bei jeder Anfrage. Die Tier-Verfügbarkeit variiert je nach Modell und Region. Reservierte Kapazität verwendet einen [bereitgestellten Durchsatz](https://docs.aws.amazon.com/bedrock/latest/userguide/prov-throughput.html)-ARN als Modell-ID statt dieser Einstellung.

<h2 id="aws-guardrails">
  AWS Guardrails
</h2>

[Amazon Bedrock Guardrails](https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails.html) ermöglichen es Ihnen, Inhaltsfilterung für Claude Code zu implementieren. Erstellen Sie einen Guardrail in der [Amazon Bedrock-Konsole](https://console.aws.amazon.com/bedrock/), veröffentlichen Sie eine Version, und fügen Sie dann die Guardrail-Header zu Ihrer [Einstellungsdatei](/docs/de/settings) hinzu. Aktivieren Sie Cross-Region-Inferenz auf Ihrem Guardrail, wenn Sie Cross-Region-Inferenzprofile verwenden.

Beispielkonfiguration:

```json theme={null}
{
  "env": {
    "ANTHROPIC_CUSTOM_HEADERS": "X-Amzn-Bedrock-GuardrailIdentifier: your-guardrail-id\nX-Amzn-Bedrock-GuardrailVersion: 1"
  }
}
```

Wenn Ihre Organisation die Guardrail-Header stattdessen über eine [Claude-Apps-Gateway](/docs/de/claude-apps-gateway)-Richtlinie bereitstellt, zählen sie als [Einstellungen, die Genehmigung benötigen](/docs/de/server-managed-settings#environment-variables-and-the-approval-dialog).

<h2 id="use-the-mantle-endpoint">
  Verwenden Sie den Mantle-Endpunkt
</h2>

Mantle ist ein Amazon Bedrock-Endpunkt, der Claude-Modelle über die native Anthropic API-Form statt über die Amazon Bedrock Invoke API bereitstellt. Er verwendet die gleichen [AWS-Anmeldedaten](#2-configure-aws-credentials) und [`awsAuthRefresh`-Konfiguration](#advanced-credential-configuration).

Mantle hat seine eigenen IAM-Aktionen unter dem `bedrock-mantle:`-Präfix, daher decken die `bedrock:`-Aktionen in der [IAM-Konfiguration](#iam-configuration) es nicht ab. Gewähren Sie Ihrer IAM-Identität `bedrock-mantle:CreateInference` für Inferenz und `bedrock-mantle:CountTokens` für Token-Zählung. Siehe [Making inference requests](https://docs.aws.amazon.com/bedrock/latest/userguide/inference.html) und [Counting tokens](https://docs.aws.amazon.com/bedrock/latest/userguide/count-tokens.html) in der AWS-Dokumentation und die [service authorization reference](https://docs.aws.amazon.com/service-authorization/latest/reference/list_amazonbedrockpoweredbyawsmantle.html) für jede Mantle-Aktion.

<h3 id="enable-mantle">
  Aktivieren Sie Mantle
</h3>

Mit bereits konfigurierten AWS-Anmeldedaten legen Sie `CLAUDE_CODE_USE_MANTLE` fest, um Anfragen zum Mantle-Endpunkt weiterzuleiten:

```bash theme={null}
export CLAUDE_CODE_USE_MANTLE=1
export AWS_REGION=us-east-1
```

Claude Code erstellt die Endpunkt-URL aus der AWS-Region, aufgelöst mit der gleichen Priorität wie [Amazon Bedrock oben](#3-configure-claude-code). Um die URL für einen benutzerdefinierten Endpunkt oder ein Gateway zu überschreiben, legen Sie `ANTHROPIC_BEDROCK_MANTLE_BASE_URL` fest.

Führen Sie `/status` in Claude Code aus, um zu bestätigen. Die Provider-Zeile zeigt `Amazon Bedrock (Mantle)`, wenn Mantle aktiv ist.

<h3 id="select-a-mantle-model">
  Wählen Sie ein Mantle-Modell
</h3>

Mantle verwendet Modell-IDs mit dem Präfix `anthropic.` und ohne Versionssuffix, z. B. `anthropic.claude-sonnet-5` oder `anthropic.claude-haiku-4-5`. Die Modelle, die Ihrem Konto zur Verfügung stehen, hängen davon ab, was Ihre Organisation erhalten hat; zusätzliche Modell-IDs sind in Ihren Onboarding-Materialien von AWS aufgeführt. Wenden Sie sich an Ihr AWS-Kontoteam, um Zugriff auf zulässige Modelle anzufordern.

Legen Sie das Modell mit dem `--model`-Flag oder mit `/model` in Claude Code fest:

```bash theme={null}
claude --model anthropic.claude-haiku-4-5
```

<h3 id="run-mantle-alongside-the-invoke-api">
  Führen Sie Mantle neben der Invoke API aus
</h3>

Die Modelle, die Ihnen auf Mantle zur Verfügung stehen, enthalten möglicherweise nicht alle Modelle, die Sie heute verwenden. Das Setzen von `CLAUDE_CODE_USE_BEDROCK` und `CLAUDE_CODE_USE_MANTLE` ermöglicht es Claude Code, beide Endpunkte aus derselben Sitzung aufzurufen. Modell-IDs, die dem Mantle-Format entsprechen, werden zu Mantle weitergeleitet, und alle anderen Modell-IDs gehen zur Amazon Bedrock Invoke API.

```bash theme={null}
export CLAUDE_CODE_USE_BEDROCK=1
export CLAUDE_CODE_USE_MANTLE=1
```

Um ein Mantle-Modell in der `/model`-Auswahl anzuzeigen, listen Sie seine ID in `availableModels` in Ihrer [Einstellungsdatei](/docs/de/settings) auf. Diese Einstellung beschränkt die Auswahl auch auf die aufgelisteten Einträge. Das Auflisten von `anthropic.claude-haiku-4-5` entfernt den bloßen `haiku`-Alias aus der Auswahl, daher sollten Sie auch Versionspräfixe oder vollständige IDs für die Versionen auflisten, die Sie auswählbar halten möchten. Die Mantle-ID und der `haiku`-Alias werden zum gleichen Modell-Familie aufgelöst, daher behält die Zusammenführung nur den spezifischeren Eintrag. Siehe [Merge-Verhalten](/docs/de/model-config#merge-behavior):

```json theme={null}
{
  "availableModels": ["opus", "sonnet", "claude-haiku-4-5", "anthropic.claude-haiku-4-5"]
}
```

Einträge mit dem `anthropic.`-Präfix werden als benutzerdefinierte Auswahl-Optionen hinzugefügt und zu Mantle weitergeleitet. Ersetzen Sie `anthropic.claude-haiku-4-5` durch die Modell-ID, die Ihr Konto erhalten hat. Siehe [Modellauswahl einschränken](/docs/de/model-config#restrict-model-selection) für Details, wie `availableModels` mit anderen Modelleinstellungen interagiert.

Wenn beide Provider aktiv sind, zeigt `/status` `Amazon Bedrock + Amazon Bedrock (Mantle)` an.

<h3 id="route-mantle-through-a-gateway">
  Leiten Sie Mantle durch ein Gateway weiter
</h3>

Wenn Ihre Organisation Modellverkehr durch ein zentralisiertes [LLM-Gateway](/docs/de/llm-gateway) leitet, das AWS-Anmeldedaten serverseitig injiziert, deaktivieren Sie die clientseitige Authentifizierung, damit Claude Code Anfragen ohne SigV4-Signaturen oder `x-api-key`-Header sendet:

```bash theme={null}
export CLAUDE_CODE_USE_MANTLE=1
export CLAUDE_CODE_SKIP_MANTLE_AUTH=1
export ANTHROPIC_BEDROCK_MANTLE_BASE_URL=https://your-gateway.example.com
```

<h3 id="mantle-environment-variables">
  Mantle-Umgebungsvariablen
</h3>

Diese Variablen sind spezifisch für den Mantle-Endpunkt. Siehe [Umgebungsvariablen](/docs/de/env-vars) für die vollständige Liste.

| Variable                                | Zweck                                                                                       |
| :-------------------------------------- | :------------------------------------------------------------------------------------------ |
| `CLAUDE_CODE_USE_MANTLE`                | Aktivieren Sie den Mantle-Endpunkt. Legen Sie auf `1` oder `true` fest.                     |
| `ANTHROPIC_BEDROCK_MANTLE_BASE_URL`     | Überschreiben Sie die Standard-Mantle-Endpunkt-URL                                          |
| `CLAUDE_CODE_SKIP_MANTLE_AUTH`          | Überspringen Sie die clientseitige Authentifizierung für Proxy-Setups                       |
| `ANTHROPIC_SMALL_FAST_MODEL_AWS_REGION` | Überschreiben Sie die AWS-Region für das Haiku-Klasse-Modell (gemeinsam mit Amazon Bedrock) |

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="authentication-loop-with-sso-and-corporate-proxies">
  Authentifizierungsschleife mit SSO und Unternehmens-Proxys
</h3>

Wenn Browser-Registerkarten wiederholt geöffnet werden, wenn Sie AWS SSO verwenden, entfernen Sie die `awsAuthRefresh`-Einstellung aus Ihrer [Einstellungsdatei](/docs/de/settings). Dies kann auftreten, wenn Unternehmens-VPNs oder TLS-Inspektions-Proxys den SSO-Browser-Flow unterbrechen. Claude Code behandelt die unterbrochene Verbindung als Authentifizierungsfehler, führt `awsAuthRefresh` erneut aus und schleift sich endlos.

Wenn Ihre Netzwerkumgebung automatische browserbasierte SSO-Flows beeinträchtigt, verwenden Sie `aws sso login` manuell, bevor Sie Claude Code starten, anstatt sich auf `awsAuthRefresh` zu verlassen.

<h3 id="certificate-errors-behind-a-tls-inspecting-proxy">
  Zertifikatsfehler hinter einem TLS-inspizierenden Proxy
</h3>

Claude Code wendet Ihre [CA-Zertifikatsspeicher](/docs/de/network-config#ca-certificate-store)-Konfiguration auf seine Anfragen an AWS an, einschließlich:

* Modellermittlung
* Token-Zählung
* Die STS- und SSO-Rollenberechtigungsaufrufe, die Ihre AWS-Anmeldedaten auflösen
* Die [Setup-Assistent](#sign-in-with-bedrock)-Berechtigungsüberprüfung und Modellprüfungen

Für diese Anfragen benötigt ein Unternehmens-Stammzertifikat in Ihrem Betriebssystem-Vertrauensspeicher oder `NODE_EXTRA_CA_CERTS`-Bundle keine Amazon Bedrock-spezifische Einrichtung.

Vor v2.1.260 wendete Claude Code Ihre CA-Konfiguration auf diese Anfragen nur an, wenn sie durch einen konfigurierten Proxy gingen, und bei einer direkten Verbindung vertrauten sie nur auf den Standard-Zertifikatsspeicher der Laufzeit.

Vor v2.1.261 vertraute die Berechtigungssuche hinter den Modellprüfungen des Setup-Assistenten mit der Option **Anmeldedaten verwenden, die bereits in meiner Umgebung vorhanden sind** immer noch nur auf den Standard-Zertifikatsspeicher der Laufzeit. Hinter einem TLS-inspizierenden Proxy, dessen Stammzertifikat nur im Betriebssystem-Speicher vorhanden ist, schlugen die betroffenen Anfragen mit `unable to get local issuer certificate` fehl, oder der Assistent zeigte Modelle als `unreachable` an, während Inferenzanfragen erfolgreich waren. Aktualisieren Sie auf v2.1.261 oder später.

<h3 id="region-issues">
  Regionsprobleme
</h3>

Wenn Sie auf Regionsprobleme stoßen:

* Modellverfügbarkeit prüfen: `aws bedrock list-inference-profiles --region your-region`
* Zu einer unterstützten Region wechseln: `export AWS_REGION=us-east-1`
* Erwägen Sie die Verwendung von Inferenzprofilen für Cross-Region-Zugriff

Wenn Sie einen Fehler „on-demand throughput isn't supported" erhalten:

* Geben Sie das Modell als [Inferenzprofil](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html)-ID an

Claude Code verwendet die Amazon Bedrock [Invoke API](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_InvokeModelWithResponseStream.html) und unterstützt die Converse API nicht.

<h3 id="streaming-errors-behind-a-gateway-or-proxy">
  Streaming-Fehler hinter einem Gateway oder Proxy
</h3>

Amazon Bedrock streamt `InvokeModelWithResponseStream`-Antworten in einem binären Event-Stream-Format mit dem Header `Content-Type: application/vnd.amazon.eventstream`. Ein Gateway oder Proxy zwischen Claude Code und Amazon Bedrock muss den Antwortkörper und seine Header, einschließlich `Content-Type`, so durchleiten, wie Amazon Bedrock sie gesendet hat.

Wenn das Gateway `Content-Type` in einen anderen Wert umschreibt, lehnt Claude Code die Antwort mit einem Fehler ab, der mit `Bedrock streaming response has content-type` beginnt und den empfangenen Wert nennt. Die häufige Umschreibung ist `text/event-stream`, von einer Integration, die den Stream als Server-Sent Events erneut aussendet.

Wenn das Gateway den Header stattdessen löscht oder leer lässt, geht Claude Code davon aus, dass der Body Amazons Bedrock-Event-Stream ist, und dekodiert ihn, sodass ein Body, den das Gateway unverändert durchgeleitet hat, weiterhin streamt.

Wenn ein Gateway, das den Header löscht, auch den Stream als Server-Sent Events erneut aussendet, kann Claude Code den Body nicht dekodieren und fällt bei jedem Turn auf einen langsameren Non-Streaming-Pfad zurück: Jede Antwort wird erst angezeigt, wenn sie vollständig ist, anstatt zu streamen. Setzen Sie in diesem Fall [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_DEFAULT=1`](/docs/de/env-vars), damit Claude Code den Body stattdessen als Server-Sent Events liest.

Um den Fehler oder den Fallback zu beheben, konfigurieren Sie das Gateway so, dass es den `InvokeModelWithResponseStream`-Antwortkörper und seinen `Content-Type`-Header unverändert durchleitet.

Ein Gateway, das den Stream in Server-Sent Events konvertiert, bedient nicht mehr die Amazon Bedrock API. Wenn es auch Anthropic Messages API-Anfragen akzeptiert, verbinden Sie sich damit als [LLM-Gateway](/docs/de/llm-gateway-connect) mit `ANTHROPIC_BASE_URL` anstelle von `CLAUDE_CODE_USE_BEDROCK`.

<h3 id="zero-token-counts-in-/context">
  Null-Token-Zählungen in /context
</h3>

Der `/context`-Befehl zählt Token für jede Tool-Gruppe, indem die Tool-Schemas an die Amazon Bedrock-API zum Zählen von Tokens gesendet werden. Bei Claude Code-Versionen vor v2.1.196 lehnte Amazon Bedrock diese Anfrage ab, da die Schemas Felder enthielten, die die API zum Zählen von Tokens nicht akzeptiert, sodass jede Tool-Gruppe 0 Tokens anzeigte. Andere Zeilen in der Aufschlüsselung, wie Nachrichten und Speicherdateien, sind nicht betroffen.

Aktualisieren Sie auf v2.1.196 oder später.

<h3 id="mantle-endpoint-errors">
  Mantle-Endpunkt-Fehler
</h3>

Wenn `/status` nach dem Setzen von `CLAUDE_CODE_USE_MANTLE` nicht `Amazon Bedrock (Mantle)` anzeigt, erreicht die Variable den Prozess nicht. Bestätigen Sie, dass sie in der Shell exportiert wird, in der Sie `claude` gestartet haben, oder legen Sie sie im `env`-Block Ihrer [Einstellungsdatei](/docs/de/settings) fest.

Was ein `403` vom Mantle-Endpunkt bedeutet, hängt davon ab, ob der Fehler eine IAM-Aktion benennt:

* Wenn der Fehler eine `bedrock-mantle:`-Aktion benennt, gewähren Sie Ihrer IAM-Identität diese Aktion.
* Wenn der Fehler keine Aktion benennt und Ihre Anmeldedaten gültig sind, wurde Ihrem AWS-Konto kein Zugriff auf das angeforderte Modell gewährt. Wenden Sie sich an Ihr AWS-Kontoteam, um Zugriff anzufordern.

Ein `400`, das die Modell-ID nennt, bedeutet, dass dieses Modell nicht auf Mantle bereitgestellt wird. Mantle hat sein eigenes Modell-Lineup, das vom Standard-Amazon Bedrock-Katalog getrennt ist, daher funktionieren Inferenzprofil-IDs wie `us.anthropic.claude-sonnet-4-6` nicht. Verwenden Sie eine Mantle-Format-ID, oder aktivieren Sie [beide Endpunkte](#run-mantle-alongside-the-invoke-api), damit Claude Code jede Anfrage zum Endpunkt weiterleitet, wo das Modell verfügbar ist.

<h2 id="additional-resources">
  Zusätzliche Ressourcen
</h2>

* [Amazon Bedrock-Dokumentation](https://docs.aws.amazon.com/bedrock/)
* [Amazon Bedrock-Preisgestaltung](https://aws.amazon.com/bedrock/pricing/)
* [Amazon Bedrock-Inferenzprofile](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html)
* [Amazon Bedrock-Token-Burndown und Kontingente](https://docs.aws.amazon.com/bedrock/latest/userguide/quotas-token-burndown.html)
* [Claude Code auf Amazon Bedrock: Schnellstartanleitung](https://builder.aws.com/content/2tXkZKrZzlrlu0KfH8gST5Dkppq/claude-code-on-amazon-bedrock-quick-setup-guide)
* [Claude Code Monitoring Implementation (Amazon Bedrock)](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md)
