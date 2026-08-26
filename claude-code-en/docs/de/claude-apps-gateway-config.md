> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Apps Gateway-Konfiguration

> Referenz für jede gateway.yaml-Option: Listener und TLS, OIDC, Session, Postgres-Speicher, Amazon Bedrock, Claude Platform auf AWS, Google Cloud's Agent Platform und Microsoft Foundry-Upstreams, Modellrouting, verwaltete Richtlinien und Telemetrie.

Eine Claude Apps Gateway-Bereitstellung wird durch eine YAML-Datei konfiguriert, üblicherweise `gateway.yaml`. Die Datei definiert alles, was das Gateway tut: wo es lauscht, wie sich Entwickler anmelden, wohin Inferenz geht und welche Richtlinien und Telemetrie gelten. Diese Seite ist die Referenz für jede Option in dieser Datei.

Um Ihre erste zu schreiben, beginnen Sie mit dem [Schnellstart](/docs/de/claude-apps-gateway#quickstart), der eine minimale funktionierende Konfiguration erstellt und ausführt. Sobald Sie eine Konfiguration haben, mit der Sie zufrieden sind, behandelt der [Bereitstellungsleitfaden](/docs/de/claude-apps-gateway-deploy) die Containerisierung und das Hosting auf Kubernetes, Cloud Run oder Ihrer eigenen Plattform.

Das Gateway liest die Datei einmal beim Start mit `claude gateway --config /path/to/gateway.yaml`. Jede Option wird beim Start gegen ein Schema validiert, sodass eine fehlerhafte Konfiguration beim Start mit einem Fehler auf Feldebene fehlschlägt, anstatt bei der ersten Verwendung.

Das [vollständige Beispiel](#complete-example) am Ende dieser Seite behandelt jeden Abschnitt.

<h2 id="file-structure">
  Dateistruktur
</h2>

Fünf Abschnitte sind [erforderlich](#required-sections). Jeder andere Abschnitt ist [optional](#optional-sections), und ein fehlender Abschnitt nimmt seine Standardwerte an. Unbekannte Schlüssel führen zum Fehlschlag beim Start, sodass ein Tippfehler als benannter Fehler anstelle einer stillschweigend ignorierten Einstellung auftaucht.

**Erforderliche Abschnitte:**

* [`listen`](#listen): Bindungsadresse, öffentliche URL, TLS-Beendigung
* [`oidc`](#oidc): Ihr Identitätsanbieter (IdP), einschließlich Aussteller, Client, Anspruchszuordnung und wer sich anmelden darf
* [`session`](#session): die Bearer-Token, die das Gateway ausstellt, mit Geheimnis und Lebensdauer
* [`store`](#store): PostgreSQL, für Gerätezuschüsse und Rate-Limit-Zähler
* [`upstreams`](#upstreams): wohin Inferenz geht, ob Anthropic, Amazon Bedrock, Claude Platform auf AWS, Agent Platform von Google Cloud oder Microsoft Foundry

**Optionale Abschnitte:**

* [`admin`](#admin): Admin-API-Authentifizierung und Aufbewahrung für Ausgabenlimits
* [`enforcement`](#enforcement): Ausgabenlimit-Verhalten bei Fehler-offen oder Fehler-geschlossen
* [`models`](#models) und `auto_include_builtin_models`: von Admin kuratierte Modellliste und Pro-Upstream-IDs
* [`managed`](#managed): verwaltete Einstellungsrichtlinien nach IdP-Gruppe
* [`telemetry`](#telemetry): OTLP-Weiterleitung an Ihren Observability-Stack
* [`access_control`, `limits`, `timeouts`, `rate_limits`](#http-tuning): IP-Zulassung/Ablehnung, Anfragegrößenbeschränkungen, Upstream-Zeit-bis-erstes-Byte und Pro-IP-Anmeldungslimits

<h2 id="secret-expansion">
  Geheimniserweiterung
</h2>

Schreiben Sie Geheimnisse wie `client_secret`, `jwt_secret` oder `postgres_url` nicht direkt in `gateway.yaml`. Referenzieren Sie sie mit einem der folgenden Formulare, und das Gateway löst den Wert beim Start aus einer Umgebungsvariablen oder einer Datei auf:

| Formular        | Wird aufgelöst zu                                                          | Verwenden für                                                        |
| --------------- | -------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| `${VAR}`        | Die Umgebungsvariable `VAR`. Der Start schlägt fehl, wenn nicht definiert. | Container-Umgebungsvariablen, AWS Secrets Manager über Env-Injektion |
| `${file:/path}` | Dateiinhalt, gekürzt                                                       | Kubernetes Secret-Volume-Mounts, Vault Agent, SOPS                   |

<h2 id="required-sections">
  Erforderliche Abschnitte
</h2>

<h3 id="listen">
  `listen`
</h3>

Der `listen`-Block steuert, wo das Gateway bereitgestellt wird: die Bindungsadresse und der Port, der extern sichtbare Ursprung und optionale TLS-Beendigung.

| Feld                   | Erforderlich       | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| ---------------------- | ------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `host`                 | Nein               | Bindungsadresse. Standard `0.0.0.0`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `port`                 | Nein               | Bindungsport. Standard `8080`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `public_url`           | Hinter einem Proxy | Der extern sichtbare `https://`-Ursprung, der zum Erstellen des IdP-`redirect_uri` und der Erkennungsmetadaten verwendet wird. Erforderlich hinter jedem TLS-beendenden Proxy wie ALB, Ingress oder Cloud Run, da das Gateway `X-Forwarded-*`-Header nicht vertraut, wenn es seinen eigenen Ursprung konstruiert; diese sind Client-spoofbar. `trusted_proxies` unten regelt nur die Client-IP-Auflösung. Auch erforderlich, um [Telemetrie](#telemetry) zu aktivieren, da das Gateway den OTLP-Endpunkt, den es an Clients pusht, aus dieser URL erstellt. |
| `tls.cert` / `tls.key` | Nein               | PEM-Pfade, wenn das Gateway selbst TLS beendet                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `trusted_proxies`      | Nein               | CIDRs oder IPs von Load Balancern vor dem Gateway. Wenn gesetzt, vertraut das Gateway `X-Forwarded-For` nur von diesen Peers und zeichnet die echte Client-IP für Pro-IP-Rate-Limiting und Audit auf. Äquivalent zu nginx `set_real_ip_from`.                                                                                                                                                                                                                                                                                                               |

<h3 id="oidc">
  `oidc`
</h3>

Der `oidc`-Block verbindet das Gateway mit Ihrem Identitätsanbieter und entscheidet, wer sich anmelden kann. Er benennt den Aussteller und OAuth-Client, ordnet die Ansprüche zu, die E-Mail und Gruppen enthalten, und beschränkt die Anmeldung nach E-Mail-Domäne oder Gruppe.

OpenID Connect (OIDC) ist das SSO-Protokoll, das das Gateway mit Ihrem Identitätsanbieter verwendet; siehe [Identitätsanbieter-Setup](/docs/de/claude-apps-gateway-deploy#identity-provider-setup) für das, was Sie auf der IdP-Seite registrieren müssen.

| Feld                            | Erforderlich | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ------------------------------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `issuer`                        | Ja           | OIDC-Erkennungsbasis. Muss Erkennung unter `/.well-known/openid-configuration` bereitstellen. Verwenden Sie HTTPS in der Produktion; das Gateway akzeptiert einen `http://`-Aussteller. Ein Loopback-Aussteller wie `http://localhost:8081` wird vom [SSRF-Schutz](/docs/de/claude-apps-gateway-deploy#threat-model-summary) abgelehnt, es sei denn, `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` ist in der Gateway-Umgebung gesetzt.                                                                                                                                                                                                                                                           |
| `client_id` / `client_secret`   | Ja           | Aus Ihrer OAuth-Client-Registrierung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `allowed_email_domains`         | Nein         | Lehnen Sie id\_tokens ab, deren `email`-Anspruch nicht in einer dieser Domänen liegt, Groß-/Kleinschreibung ignoriert. Verteidigungstiefe gegen Multi-Tenant-IdP-Fehlkonfiguration. Unabhängig von dieser Einstellung wird ein id\_token, dessen `email_verified`-Anspruch explizit `false` ist, immer abgelehnt.                                                                                                                                                                                                                                                                                                                                                                |
| `allowed_groups`                | Nein         | Beschränken Sie die Anmeldung auf Mitglieder dieser IdP-Gruppen, abgeglichen gegen `groups_claim`. Ein Benutzer in einer zulässigen E-Mail-Domäne, aber in keiner dieser Gruppen, wird abgelehnt. Erfordert, dass der IdP den Gruppenanspruch ausgibt.                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `groups_claim`                  | Nein         | Welcher id\_token-Anspruch trägt die Gruppenmitgliedschaft. Standard `groups`. Microsoft Entra gibt App-Rollen unter `roles` aus. Akzeptiert einen flachen Schlüssel oder einen RFC 6901 JSON Pointer wie `/resource_access/gateway/roles` für verschachtelte Ansprüche.                                                                                                                                                                                                                                                                                                                                                                                                         |
| `google_groups`                 | Nein         | Schlagen Sie die Gruppen des angemeldeten Benutzers über die Google Workspace Admin SDK Directory API nach, da Googles id\_token keinen Gruppenanspruch trägt. Setzen Sie `service_account_json_path` auf eine Service-Account-Schlüsseldatei mit Domain-weiter Delegation im Bereich `https://www.googleapis.com/auth/admin.directory.group.readonly` und `admin_email` auf einen Workspace-Administrator, den der Service Account annimmt; die Directory API erfordert ein echtes Admin-Subjekt. Die E-Mail-Adressen jeder Benutzergruppe werden zu ihrem Gruppenanspruch, sodass `allowed_groups` und `managed.policies.match.groups` auf Gruppen-E-Mails abgeglichen werden. |
| `email_claim`                   | Nein         | Welcher id\_token-Anspruch trägt die E-Mail des Benutzers. Standard `email`. Einige IdPs wie ADFS und Entra B2C geben stattdessen `upn` oder `preferred_username` aus. Akzeptiert einen flachen Schlüssel, einen JSON Pointer oder eine Liste von Fallback-Schlüsseln, wobei der erste vorhandene Schlüssel verwendet wird.                                                                                                                                                                                                                                                                                                                                                      |
| `scopes`                        | Nein         | Vollständige Überschreibung der OIDC-Bereiche, die das Gateway anfordert. Standard `[openid, profile, email, offline_access]`. Setzen Sie, wenn Ihr IdP Bereiche ablehnt, die er nicht erkennt, oder einen benutzerdefinierten Bereich erfordert, um Gruppen oder E-Mail auszugeben. Muss `openid` enthalten. Das Löschen von `offline_access` deaktiviert Aktualisierungstoken, sodass Entwickler die Browser-Anmeldung alle `session.ttl_hours` erneut ausführen. Siehe [Identitätsanbieter-Setup](/docs/de/claude-apps-gateway-deploy#identity-provider-setup) für Pro-IdP-Bereich-Rezepte wie Googles Aktualisierungstoken-Fluss.                                                 |
| `extra_auth_params`             | Nein         | Zusätzliche Abfrageparameter, die wörtlich an die IdP-Autorisierungsanfrage angehängt werden. Dies ist der Überschreibungsmechanismus für IdP-spezifisches Verhalten, wie `access_type: offline` für Google-Aktualisierungstoken, `domain_hint` für einige Entra-Mandanten oder `acr_values` für Step-up-Flüsse. Kann die vom Gateway verwalteten Protokollparameter nicht überschreiben: `state`, `nonce`, `redirect_uri`, PKCE, `scope`, `response_type`, `response_mode` und `client_id`.                                                                                                                                                                                     |
| `userinfo_fallback`             | Nein         | Wenn der id\_token E-Mail oder Gruppen auslässt, rufen Sie sie von `/userinfo` ab. Erforderlich für Keycloak-Lightweight-Zugriffstokens, den Okta-Org-Server und ADFS-Minimal-Tokens. Der id\_token bleibt maßgeblich; userinfo füllt nur Lücken. Standard `false`.                                                                                                                                                                                                                                                                                                                                                                                                              |
| `use_pkce`                      | Nein         | Senden Sie eine PKCE (S256)-Herausforderung bei der Autorisierungsanfrage. Standard `true`. Setzen Sie `false` nur, wenn Ihr IdP PKCE für diesen vertraulichen Client ablehnt.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `clock_skew_seconds`            | Nein         | Tolerieren Sie Uhrenabweichungen beim Validieren von id\_token-Zeitansprüchen. Standard `0`, was streng ist. Erhöhen Sie, wenn Sie "Token abgelaufen / noch nicht gültig"-Fehler direkt nach der Anmeldung aufgrund von Host-/IdP-Uhrenabweichung sehen.                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `token_endpoint_auth_method`    | Nein         | Überschreiben Sie die Token-Endpunkt-Authentifizierungsmethode. Akzeptiert `client_secret_basic` oder `client_secret_post`. Standardmäßig automatisch ausgehandelt.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `id_token_signed_response_alg`  | Nein         | Erwarteter id\_token-Signaturalgorithmus. Standard `RS256`. Setzen Sie für IdPs, die mit ES256, PS256 oder EdDSA signieren.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `additional_authorized_parties` | Nein         | Zusätzliche `azp`-Werte, die neben `client_id` akzeptiert werden, für Keycloak-Broker und Token-Exchange-Flüsse                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `discovery_url`                 | Nein         | Rufen Sie das Erkennungsdokument von dieser URL ab, anstatt es vom `issuer` abzuleiten, für IdPs hinter einem Proxy, der den Aussteller-Host umschreibt. Der Pfad muss `/.well-known/` enthalten.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `form_action_origins`           | Nein         | Zusätzliche Ursprünge für die `Content-Security-Policy: form-action`-Direktive der `/device`-Seite. Das Gateway erlaubt bereits `'self'` und den erkannten `authorization_endpoint`-Ursprung, aber Chrome erzwingt `form-action` gegen die gesamte Umleitungskette. Wenn Ihr IdP durch einen zweiten Host umleitet, wie Azure AD, das zu ADFS verbunden ist, Hub-Spoke-Okta oder ein unternehmensweiter SSO-Interceptor, listen Sie jeden Ursprung auf, durch den die Autorisierungsanfrage umgeleitet werden kann.                                                                                                                                                              |
| `ca_cert_pem`                   | Nein         | PEM-CA-Zertifikat, das den System-Trust-Store nur für IdP-Anfragen ersetzt. Verwenden Sie für Keycloak oder Dex hinter unternehmensweiter PKI.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |

<h3 id="session">
  `session`
</h3>

Der `session`-Block formt die Bearer-Token, die das Gateway nach der Anmeldung ausstellt: das Geheimnis, das sie signiert, und wie lange sie leben.

| Feld         | Erforderlich | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| ------------ | ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `jwt_secret` | Ja           | Mindestens 32 Bytes Entropie, zum Beispiel von `openssl rand -base64 32`. Signiert die HS256-Bearer-Tokens des Gateways. Akzeptiert einen einzelnen String oder ein Array für Rotation: Index 0 signiert und alle Einträge verifizieren. Um zu rotieren, stellen Sie ein neues Geheimnis voran, warten Sie `ttl_hours`, dann löschen Sie das alte.                                                                                                                                                                                       |
| `ttl_hours`  | Nein         | Gateway-Bearer-Token-Lebensdauer. Standard `1`. Die CLI aktualisiert sich stillschweigend vor Ablauf, wenn der IdP Aktualisierungstoken ausgibt. Eine kürzere Lebensdauer hebt die Bereitstellung schneller auf; eine längere macht weniger IdP-Rundfahrten. Wenn Ihr IdP keine Aktualisierungstoken ausstellen kann, weil `offline_access` nicht verfügbar ist, gibt es keine stille Aktualisierung, also erhöhen Sie dies auf `8` oder `12`, um zu vermeiden, dass Entwickler alle Stunde zur Browser-Anmeldung zurückgesendet werden. |

<h3 id="store">
  `store`
</h3>

Der `store`-Block zeigt das Gateway auf seine PostgreSQL-Datenbank, die Gerätezuschüsse und Rate-Limit-Zähler enthält.

| Feld              | Erforderlich | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| ----------------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `postgres_url`    | Ja           | `postgres://` oder `postgresql://` URL. Erforderlich: das Gerätezuschuss-Rendezvous, wo der Browser-Callback schreibt und die Polling-CLI liest, benötigt Cross-Replica-Status. Das Gateway führt seine eigenen Schema-Migrationen beim Start aus, sodass die Rolle `CREATE TABLE` im Zielschema benötigt. Wenn Ihre Sicherheitsrichtlinie DDL von der Anwendungsrolle verbietet, führen Sie die Migrationen mit einer Admin-Rolle aus, anfangs und jedes Mal, wenn ein neues Release Migrationen ausliefert, und gewähren Sie der App-Rolle `SELECT, INSERT, UPDATE, DELETE` auf den Gateway-Tabellen. Siehe [Upgrades](/docs/de/claude-apps-gateway-deploy#upgrades) und [Postgres](/docs/de/claude-apps-gateway-deploy#postgres). |
| `username`        | Nein         | Überschreibt den Benutzer in `postgres_url`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `password`        | Nein         | Datenbankberechtigungsnachweis. Setzen Sie ihn hier anstelle von `postgres_url`, damit die Berechtigung aus der URL bleibt. Akzeptiert beliebige Zeichen und hat Vorrang vor URL-Berechtigungsnachweisen.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `max_connections` | Nein         | Postgres-Verbindungspool-Größe pro Replik. Standard `5`, was konservativ und freundlich zu gemeinsamen Datenbanken ist. Mit [Ausgabenlimits](#admin) aktiviert, macht der Hot Path ein paar Operationen pro Inferenzanfrage, also erhöhen Sie es für eine dedizierte Datenbank unter Last und halten Sie Replikas × dies unter der `max_connections` der Datenbank.                                                                                                                                                                                                                                                                                                                                                        |

Für die lokale Entwicklung zeigen Sie `postgres_url` auf einen Wegwerf-Postgres-Container, zum Beispiel `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`.

<h3 id="upstreams">
  `upstreams`
</h3>

`upstreams` ist eine geordnete Liste. Das Gateway leitet Inferenz an den ersten Upstream weiter, der das angeforderte Modell auflöst. Bei `5xx`, `429`, `401`, `403`, `404` oder Timeout schlägt es zum nächsten fehl; andere `4xx` nicht, da diese Fehler der Anfrage statt dem Upstream zuzuordnen sind. Ein `401` oder `403` bedeutet, dass die eigenen Berechtigungsnachweise des Gateways gegen diesen Upstream fehlgeschlagen sind, und ein `404` bedeutet, dass dieser Upstream das angeforderte Modell nicht bedient, sodass ein späterer Upstream in der Liste es immer noch kann.

Failover bei `404` erfordert Gateway v2.1.198 oder später. Frühere Releases gaben den ersten `404` an den Client zurück, auch wenn ein späterer Upstream in der Liste das Modell bediente.

Mehrere Upstreams desselben Anbieters müssen einen unterschiedlichen `name:` setzen.

Bedrock-, Claude Platform on AWS-, Agent Platform- und Foundry-Clients werden einmal beim Start erstellt, und ihre SDKs aktualisieren Berechtigungsnachweise intern, sodass das Rotieren von Cloud-Berechtigungsnachweisen keinen Neustart erfordert. Statische Anthropic-API-Schlüssel und Bearer werden beim Start gelesen; siehe [Anthropic API](#anthropic-api).

<h4 id="anthropic-api">
  Anthropic API
</h4>

Der minimale Anthropic-Upstream ist ein API-Schlüssel aus der [Claude Console](https://platform.claude.com):

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}
    # ODER ein OAuth-Bearer (z.B. ein Workload-Identity-Federation-ausgetauschter Token):
    #   oauth_token: ${file:/var/run/secrets/anthropic-oauth-token}
    # base_url: https://api.anthropic.com   # Standard; Überschreibung für einen Forward Proxy
```

Die zwei Berechtigungsnachweis-Formulare unterscheiden sich im Header, den sie senden:

* **`api_key`**: sendet `x-api-key`. Rotieren Sie ihn in der Claude Console und aktualisieren Sie die Env-Variable.
* **`oauth_token`**: sendet `Authorization: Bearer`. Verwenden Sie das Bearer-Formular, wenn Ihre Organisation kurzlebige Token statt langlebiger API-Schlüssel ausgibt. Der Bearer wird einmal beim Start gelesen, also aktualisieren Sie durch Remounten des Geheimnisses und Neustart.

Anstelle eines statischen Schlüssels oder Bearers können Sie Workload Identity Federation verwenden. Erstellen Sie eine Verbindungsregel, indem Sie dem [Workload Identity Federation-Leitfaden](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation) folgen, dann mounten Sie das OIDC-JWT Ihrer Workload als Datei, wie ein Kubernetes-projiziertes Service-Account-Token oder ein ID-Token einer CI-Plattform. Das Gateway tauscht das JWT gegen einen kurzlebigen Bearer aus und aktualisiert ihn automatisch. Die Token-Datei wird bei jedem Austausch erneut gelesen, sodass rotierte projizierte Tokens ohne Neustart aufgegriffen werden.

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      federation_rule_id: ${ANTHROPIC_FEDERATION_RULE_ID}
      organization_id: ${ANTHROPIC_ORGANIZATION_ID}
      identity_token_file: /var/run/secrets/anthropic/id-token
      # workspace_id: wrkspc_...       # erforderlich, wenn die Regel >1 Workspace abdeckt
      # service_account_id: svac_...   # optionale erwartete Zielprüfung
```

<h4 id="amazon-bedrock">
  Amazon Bedrock
</h4>

Für die Client-seitige Bedrock-Bereitstellung, die das Gateway ersetzt oder frontet, siehe [Claude Code on Amazon Bedrock](/docs/de/amazon-bedrock). Der Gateway-seitige Upstream:

```yaml theme={null}
upstreams:
  - provider: bedrock
    region: us-east-1
    auth: {}                           # bevorzugt: AWS-Standard-Berechtigungskette
    # ODER explizite Berechtigungsnachweise:
    # auth:
    #   aws_access_key_id: ${AWS_AKID}
    #   aws_secret_access_key: ${AWS_SK}
    #   aws_session_token: ${AWS_ST}
    # ODER ein Bedrock-API-Bearer-Token:
    # auth:
    #   aws_bearer_token: ${AWS_BEARER_TOKEN}
    # Überschreiben Sie den bedrock-runtime-Endpunkt für FIPS oder VPC-Endpunkt-Bereitstellungen:
    # base_url: https://bedrock-runtime-fips.us-east-1.amazonaws.com
```

Ein leerer `auth`-Block verwendet die Standard-Berechtigungskette des AWS SDK: Env-Variablen, `~/.aws/credentials`, ECS-Task-Rolle, EC2-Instanzmetadaten oder IRSA auf EKS. In der Produktion geben Sie dem Gateway-Pod eine IAM-Rolle statt statische Schlüssel in ein Container-Image einzubetten.

Explizite Berechtigungsnachweise müssen vollständig sein: Das Gateway schlägt beim Start fehl, wenn `aws_access_key_id` und `aws_secret_access_key` nicht zusammen gesetzt sind, oder wenn `aws_session_token` ohne sie gesetzt ist. Vor v2.1.207 bestand ein partieller `auth:`-Block die Validierung.

| Setup              | Wie                                                                                                                                                                                                                                                                                                                                                                           |
| ------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| IAM-Berechtigungen | Gewähren Sie dem Gateway-Principal `bedrock:InvokeModel` und `bedrock:InvokeModelWithResponseStream` sowohl auf den Inferenz-Profil-ARNs als auch auf den zugrunde liegenden Foundation-Model-ARNs. Für den integrierten Katalog in US-Regionen: `arn:aws:bedrock:<region>:<account>:inference-profile/us.anthropic.*` und `arn:aws:bedrock:*::foundation-model/anthropic.*`. |
| Modellzugriff      | In der Bedrock-Konsole pro Region fordern Sie Modellzugriff für die Claude-Modelle an, die Sie möchten, und aktivieren Sie ihn. Cross-Region-Inferenz-Profile (`us.anthropic.*`) erfordern Modellzugriff in jeder Region, die das Profil umfasst.                                                                                                                             |
| EKS (IRSA)         | Erstellen Sie eine IAM-Rolle mit der obigen Richtlinie und einer Vertrauensrichtlinie für den OIDC-Provider Ihres Clusters, der auf das Service-Account des Gateways beschränkt ist. Kommentieren Sie das Service-Account mit `eks.amazonaws.com/role-arn: arn:aws:iam::<acct>:role/claude-gateway`. `auth: {}` nimmt es auf.                                                 |
| ECS / EC2          | Hängen Sie die IAM-Rolle an die Task-Definition oder das Instance-Profil an. `auth: {}` nimmt es auf.                                                                                                                                                                                                                                                                         |
| Überall sonst      | Übergeben Sie Berechtigungsnachweise über die Env-Variablen `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` und `AWS_SESSION_TOKEN`, oder setzen Sie sie explizit in `auth:` mit `${VAR}`-Erweiterung                                                                                                                                                                            |
| Region             | `region:` ist die API-Endpunkt-Region. Cross-Region-Inferenz-Profile routen über die Geo (US, EU, APAC) unabhängig davon, welche Sie wählen. Für Nicht-US-Regionen oder bereitgestellte Durchsatz-ARNs fügen Sie einen [`models:`](#models)-Block mit den richtigen Pro-Upstream-IDs hinzu.                                                                                   |

<h4 id="claude-platform-on-aws">
  Claude Platform on AWS
</h4>

Claude Platform on AWS bedient die First-Party-Anthropic-API auf AWS-Infrastruktur unter `aws-external-anthropic.<region>.api.aws`. Sie verwendet First-Party-Modell-IDs, berücksichtigt `anthropic-beta`-Header wie gesendet und bedient `count_tokens`, sodass keine der Bedrock-spezifischen Übersetzung gilt. Der `anthropicAws`-Provider erfordert Claude Code v2.1.198 oder später; frühere Gateway-Releases lehnen ihn beim Start ab.

Für die Client-seitige Bereitstellung derselben Plattform siehe [Claude Code on Claude Platform on AWS](/docs/de/claude-platform-on-aws). Der Gateway-seitige Upstream:

```yaml theme={null}
upstreams:
  - provider: anthropicAws
    region: us-east-1
    workspace_id: wrkspc_...
    auth:
      api_key: ${ANTHROPIC_AWS_API_KEY}   # gesendet als x-api-key
    # ODER SigV4 über die AWS-Standard-Berechtigungskette:
    # auth: {}
    # ODER explizite SigV4-Berechtigungsnachweise:
    # auth:
    #   aws_access_key_id: ${AWS_ACCESS_KEY_ID}
    #   aws_secret_access_key: ${AWS_SECRET_ACCESS_KEY}
    # Überschreiben Sie den abgeleiteten Endpunkt:
    # base_url: https://aws-external-anthropic.us-east-1.api.aws
```

Die Plattform läuft in einem separaten AWS-Konto von Amazon Bedrock und signiert SigV4-Anfragen für seinen eigenen Service-Namen, `aws-external-anthropic`, sodass eine Bedrock-scoped IAM-Rolle es nicht autorisiert. Ein API-Schlüssel in `auth.api_key` hat Vorrang, wenn SigV4-Berechtigungsnachweise auch gesetzt sind. Ein leerer `auth`-Block verwendet die Standard-Berechtigungskette des AWS SDK, dieselbe Kette, die der [Amazon Bedrock](#amazon-bedrock)-Upstream verwendet.

| Feld                                                    | Erforderlich | Beschreibung                                                                                                                                            |
| ------------------------------------------------------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `region`                                                | Ja           | AWS-Region, Kleinbuchstaben, Ziffern und Bindestriche. Das Gateway leitet den Endpunkt davon ab als `https://aws-external-anthropic.<region>.api.aws`.  |
| `workspace_id`                                          | Ja           | Gesendet als Header bei jeder Anfrage; die Plattform erfordert es                                                                                       |
| `auth.api_key`                                          | Nein         | API-Schlüssel für die Plattform, gesendet als `x-api-key`. Kein Bearer-Token: die zwei Auth-Modi sind ein API-Schlüssel oder SigV4.                     |
| `auth.aws_access_key_id` / `auth.aws_secret_access_key` | Nein         | Explizite SigV4-Berechtigungsnachweise. Das Setzen eines ohne das andere schlägt beim Start fehl. `auth.aws_session_token` wird neben ihnen akzeptiert. |
| `base_url`                                              | Nein         | Überschreiben Sie den abgeleiteten Endpunkt                                                                                                             |

Da die Plattform First-Party-Modell-IDs auflöst, leitet der integrierte Katalog zu ihr ohne [`models:`](#models)-Block weiter. Wenn Sie eine `models:`-Liste kuratieren, schlüsseln Sie den Eintrag `anthropicAws:` mit der First-Party-ID.

<h4 id="google-cloud-agent-platform">
  Google Cloud Agent Platform
</h4>

Für das äquivalente Client-seitige Setup siehe [Claude Code on Google Cloud](/docs/de/google-vertex-ai). Der Gateway-seitige Upstream:

```yaml theme={null}
upstreams:
  - provider: vertex
    region: us-east5
    project_id: example-prod
    auth: {}                           # bevorzugt: Application Default Credentials
    # ODER eine Service-Account-Schlüsseldatei:
    # auth: { service_account_json: /secrets/sa.json }
    # Überschreiben Sie den aiplatform-Endpunkt für Private Service Connect:
    # base_url: https://us-east5-aiplatform.p.googleapis.com
```

Ein leerer `auth`-Block verwendet Application Default Credentials: `GOOGLE_APPLICATION_CREDENTIALS`, GCE-Metadaten oder GKE Workload Identity. Service-Account-JSON-Schlüsseldateien werden unterstützt, aber nicht empfohlen; verwenden Sie Workload Identity oder hängen Sie ein Service-Account an die GCE- oder Cloud Run-Instanz an.

Setzen Sie `region: global`, um [Agent Platforms globalen Endpunkt](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations) statt eines regionalen zu verwenden. Google leitet dann jede Anfrage an eine verfügbare Region weiter, sodass Sie die Pro-Region-Modellverfügbarkeit nicht verfolgen. Das Setzen einer bestimmten Region heftet jede Anfrage daran.

| Setup                   | Wie                                                                                                                                                                                                                                      |
| ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| IAM-Berechtigungen      | Gewähren Sie dem Gateway-Service-Account `roles/aiplatform.user` auf dem Projekt oder eine benutzerdefinierte Rolle mit `aiplatform.endpoints.predict`. Aktivieren Sie die Agent Platform API (`aiplatform.googleapis.com`).             |
| Modellzugriff           | Aktivieren Sie in Model Garden die Claude-Modelle für Ihr Projekt. Sie werden in bestimmten Regionen veröffentlicht; überprüfen Sie die Modellkarte auf unterstützte Regionen.                                                           |
| GKE (Workload Identity) | Binden Sie ein GCP-Service-Account an das Kubernetes-Service-Account des Gateways und kommentieren Sie das KSA mit `iam.gke.io/gcp-service-account: claude-gateway@<proj>.iam.gserviceaccount.com`. `auth: {}` nimmt es auf.             |
| Cloud Run / GCE         | Setzen Sie das Service-Account des Service auf eines mit `roles/aiplatform.user`. `auth: {}` nimmt es auf.                                                                                                                               |
| Überall sonst           | `auth: { service_account_json: /secrets/sa.json }`, der Pfad zu einer JSON-Schlüsseldatei, die als Geheimnis gemountet ist. Das Feld nimmt einen Dateipfad, nicht den Schlüsselinhalt, also ist keine `${file:…}`-Erweiterung beteiligt. |

<h4 id="microsoft-foundry">
  Microsoft Foundry
</h4>

Für die Client-seitige Foundry-Bereitstellung siehe [Claude Code on Microsoft Foundry](/docs/de/microsoft-foundry). Der Gateway-seitige Upstream:

```yaml theme={null}
upstreams:
  - provider: foundry
    resource: example-foundry              # https://example-foundry.services.ai.azure.com
    auth: { use_azure_ad: true }        # bevorzugt: DefaultAzureCredential / Managed Identity
    # ODER ein API-Schlüssel:
    # auth:
    #   api_key: ${FOUNDRY_API_KEY}
```

`use_azure_ad: true` wird durch `DefaultAzureCredential` aufgelöst: Managed Identity auf AKS, ACI oder App Service; die Azure CLI; oder Umgebungsberechtigungsnachweise. API-Schlüssel funktionieren, sind aber projektumfassend und rotieren nicht automatisch. Der Foundry-Endpunkt wird von `resource:` abgeleitet; setzen Sie das optionale `base_url`, um es für souveräne Clouds wie Azure Government zu überschreiben.

| Setup                    | Wie                                                                                                                                                                                                             |
| ------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| RBAC                     | Gewähren Sie dem Gateway-Identity `Azure AI User` oder `Cognitive Services User` auf der Foundry-Ressource                                                                                                      |
| Bereitstellungen         | Foundry verwendet von Admin gewählte Bereitstellungsnamen, nicht kanonische Modell-IDs. Fügen Sie einen [`models:`](#models)-Block hinzu, der jede kanonische ID Ihrem Bereitstellungsnamen zuordnet.           |
| AKS (Workload-Identität) | Verbinden Sie eine User-Assigned Managed Identity mit dem OIDC-Issuer des Clusters und binden Sie sie an das Service-Account des Gateways. `use_azure_ad: true` nimmt es über `WorkloadIdentityCredential` auf. |
| ACI / App Service        | Aktivieren Sie system-zugewiesene oder user-zugewiesene Managed Identity auf der Ressource. `use_azure_ad: true` nimmt es auf.                                                                                  |
| Überall sonst            | `auth: { api_key: "${FOUNDRY_API_KEY}" }`. Zitieren Sie `${…}` innerhalb von `{ }`.                                                                                                                             |

<h4 id="multiple-upstreams">
  Mehrere Upstreams
</h4>

Derselbe Anbieter kann mehr als einmal mit einem unterschiedlichen `name:` erscheinen. Dies deckt verschiedene Regionen, verschiedene Konten über verschiedene Berechtigungsketten, bereitgestellter Durchsatz versus On-Demand und Cross-Provider-Fallback ab.

Das Gateway versucht Upstreams in Reihenfolge. `5xx`, `429`, `401`, `403`, `404`, Timeouts und fehlender Endpunkt (`501`) schlagen fehl; andere `4xx` nicht.

`429` ist Pro-Upstream-Kapazität, sodass bereitgestellter Durchsatz (PT)-Erschöpfung zu On-Demand fehlschlägt. `404` ist Pro-Upstream-Modellverfügbarkeit, sodass ein Upstream, der ein Modell nicht aktiviert hat, einen späteren Upstream, der es bedient, nicht blockiert. Ein Upstream, der das angeforderte Modell nicht auflösen kann, wird ohne Netzwerk-Rundfahrt übersprungen.

Dieses Beispiel leitet eine bereitgestellte Durchsatz-Bedrock-Zuteilung zuerst weiter, überläuft zu On-Demand und einem zweiten Konto und fällt zuletzt auf die Anthropic API zurück:

```yaml theme={null}
upstreams:
  # Primär: bereitgestellter Durchsatz in Ihrer Heimatregion.
  - name: bedrock-pt
    provider: bedrock
    region: us-east-1
    auth: {}
  # Überlauf: On-Demand Cross-Region.
  - name: bedrock-od
    provider: bedrock
    region: us-west-2
    auth: {}
  # Anderes Konto: eine separate Bedrock-Zuteilung über angenommene Rollberechtigungsnachweise.
  - name: bedrock-acct2
    provider: bedrock
    region: us-east-1
    auth:
      aws_access_key_id: ${ACCT2_AKID}
      aws_secret_access_key: ${ACCT2_SK}
  # Letzter Ausweg: direkte Anthropic API.
  - name: anthropic-fallback
    provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}

# Pro-Upstream-Modell-IDs werden auf dem `name:` des Upstream geschlüsselt; ein Upstream
# ohne `name:` nimmt standardmäßig seinen Provider-String (z.B. `bedrock`). Jeder
# Upstream, der nicht für ein Modell aufgelistet ist, wird übersprungen, was ist, wie Sie ein Modell
# zu bereitgestelltem Durchsatz routen, während alles andere On-Demand bleibt.
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      bedrock-pt: arn:aws:bedrock:us-east-1:111111111111:provisioned-model/abcdef
      bedrock-od: us.anthropic.claude-opus-4-8
      bedrock-acct2: us.anthropic.claude-opus-4-8
      anthropic-fallback: claude-opus-4-8
```

| Hebel                      | Wie                                                                                                                                                                                                                                                     |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Verschiedene Regionen      | Ein Bedrock-Upstream pro Region, jeder mit seiner eigenen `region:`. Mit [`auto_include_builtin_models: true`](#models) routen die Cross-Region-Inferenz-Profile automatisch; für Region-gepinnte Bereitstellungen verwenden Sie einen `models:`-Block. |
| Verschiedene Konten        | Ein Bedrock-Upstream pro Konto, jeder mit seinen eigenen Berechtigungsnachweisen in `auth:`. Die Standard-Kette (`auth: {}`) verwendet die Pod-Identität; für ein zweites Konto setzen Sie explizite Berechtigungsnachweise oder ein Bearer-Token.      |
| Bereitgestellter Durchsatz | Ordnen Sie das Modell der bereitgestellten Durchsatz-ARN in `models:` für den Namen dieses Upstream zu. Andere Upstreams behalten die On-Demand-ID, sodass PT-Kapazität vor dem Failover erschöpft ist.                                                 |
| VPC / FIPS-Endpunkte       | Setzen Sie `base_url:` auf dem Upstream auf Ihren VPC-Endpunkt oder FIPS-Endpunkt-URL                                                                                                                                                                   |
| Modell-gesteuertes Routing | Lassen Sie einen Upstream aus der `upstream_model:`-Karte eines Modells weg und dieser Upstream wird für dieses Modell übersprungen. Zum Beispiel routen Sie Opus zu bereitgestelltem Durchsatz und Sonnet und Haiku zu On-Demand.                      |

Das Failover zwischen Cloud-Anbietern oder zur direkten Anthropic API ändert, welche Vereinbarung, Geographie und andere Bedingungen die Anfrage regeln.

Die CLI wendet dasselbe Feature-Gating auf Gateways an, unabhängig davon, welcher Upstream eine bestimmte Anfrage bedient, sodass Failover kein Body-Feld sendet, das ein Upstream ablehnen würde.

<h2 id="optional-sections">
  Optionale Abschnitte
</h2>

<h3 id="admin">
  `admin`
</h3>

Optional. Aktiviert `/v1/organizations/spend_limits`, das Anthropics öffentliche Admin API spiegelt, und Pro-Entwickler-Ausgabendurchsetzung auf `/v1/messages`. Siehe [Ausgabenlimits](/docs/de/claude-apps-gateway-spend-limits) für wie Caps gesetzt und durchgesetzt werden; dieser Abschnitt behandelt die `gateway.yaml`-Schlüssel, die die Funktion aktivieren und sie abstimmen.

```yaml theme={null}
admin:
  # Benannte statische API-Schlüssel für die Admin-Endpunkte, gesendet als x-api-key.
  # Die ID erscheint im Audit-Log als admin-key:<id>, sodass jeder Schlüssel
  # zurechenbar ist. Array für Rotation: fügen Sie den neuen Schlüssel hinzu, rollen Sie Clients,
  # entfernen Sie den alten.
  write_keys:
    - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
    - { id: ci,        key: "${GATEWAY_ADMIN_WRITE_KEY_CI}" }
  read_keys:
    - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
  # IdP-Gruppen, denen vollständiger Admin über das normale Gateway-JWT gewährt wird (kein API-Schlüssel).
  admin_groups: [platform-finops]
  blocked_message: request an increase at https://go.example.com/claude-limits
```

| Feld                      | Erforderlich | Beschreibung                                                                                                                                                                                                                                                                             |
| ------------------------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `write_keys`              | Nein         | Array von `{id, key}`. Ein `x-api-key`, das einem dieser entspricht, kann Ausgabenlimits auflisten, setzen und löschen. Schlüsselwerte müssen mindestens 32 Zeichen sein; `id`s müssen über `read_keys` und `write_keys` eindeutig sein.                                                 |
| `read_keys`               | Nein         | Array von `{id, key}`. Schreibgeschützt: jeder `GET`-Endpunkt, einschließlich Auflistung von Caps, Abrufen eines nach ID und Lesen von [`/effective`](/docs/de/claude-apps-gateway-spend-limits#%2Feffective) und [`/audit`](/docs/de/claude-apps-gateway-spend-limits#%2Faudit).                  |
| `admin_groups`            | Nein         | IdP-Gruppennamen. Ein Gateway-JWT, dessen `groups`-Anspruch einen dieser enthält, hat vollständigen Admin-Zugriff, Lesen und Schreiben, und Audits als `oidc:<sub>`. Verwenden Sie dies für menschliche Admins; verwenden Sie API-Schlüssel für Maschinen.                               |
| `blocked_message`         | Nein         | Wörtlich an die `429 billing_error` angehängt, die ein blockierter Entwickler sieht. Schreiben Sie die ganze Anweisung, wie eine URL oder einen Slack-Kanal. Nicht gesetzt, der Fehler ist `spend limit reached`.                                                                        |
| `audit_retention_days`    | Nein         | Standard `365`. Ältere `admin_audit`-Zeilen werden gefegt.                                                                                                                                                                                                                               |
| `spend_retention_months`  | Nein         | Standard `13`. `spend`-Zähler-Zeilen älter als dies werden gefegt. Der Standard behält ein volles Jahr plus den aktuellen Teilmonat für Jahr-über-Jahr-Berichterstattung.                                                                                                                |
| `identity_retention_days` | Nein         | Standard `90`. Last-Seen-TTL für `principal_emails`-Zeilen, die die E-Mail, den Anzeigenamen und die Gruppen jedes Entwicklers enthalten (PII). Absichtlich kürzer als Ausgabenaufbewahrung, sodass eine bereitgestellte Identität altert, während ihre anonymen Ausgabenzähler bleiben. |
| `group_limit_mode`        | Nein         | `min` (Standard) oder `max`. Wenn ein Entwickler in mehreren Gruppen mit Caps ist, erzwingt `min` die restriktivste und `max` die am wenigsten restriktive. Wird sowohl von Durchsetzung als auch von `/effective` verwendet.                                                            |

<h3 id="enforcement">
  `enforcement`
</h3>

Der `enforcement`-Block steuert, wie Ausgabenlimit-Prüfungen sich verhalten, wenn der Store nicht verfügbar ist.

| Feld                   | Erforderlich | Beschreibung                                                                                                                                                                                                                                                                                                                 |
| ---------------------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `fail_closed_on_error` | Nein         | Standard `false`. Ausgabendurchsetzung schlägt bei einem Postgres-Ausfall offen fehl, sodass Inferenz oben bleibt. Setzen Sie `true`, um geschlossen fehlzuschlagen: Über-Cap-Entwickler werden blockiert, aber so ist jeder, wenn der Store nicht erreichbar ist. Hat keine Auswirkung ohne einen [`admin:`](#admin)-Block. |

<h3 id="models">
  `models`
</h3>

Der `models`-Block ist eine optionale von Admin kuratierte Modellliste, die unter `/v1/models` bereitgestellt wird und zum Übersetzen von Modell-IDs pro Upstream verwendet wird. Es ist erforderlich für Nicht-US-Amazon Bedrock-Regionen, Amazon Bedrock-bereitgestellte Durchsatz-ARNs und Microsoft Foundry-Bereitstellungsnamen.

```yaml theme={null}
auto_include_builtin_models: true   # false: nur die Liste unten exponieren
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    # description: optionaler Text, der in Clients angezeigt wird, die ihn exponieren
    upstream_model:
      anthropic: claude-opus-4-8
      bedrock: us.anthropic.claude-opus-4-8   # oder eine Inferenz-Profil-ARN
      foundry: your-opus-deployment-name
```

<h3 id="managed">
  `managed`
</h3>

Der `managed`-Block definiert rollenbasierte Zugriffrichtlinien, die auf IdP-Gruppen oder E-Mail-Domäne geschlüsselt sind. Richtlinien werden in Reihenfolge ausgewertet; die erste Übereinstimmung wird ausgewählt, dann auf die `match: {}`-Catch-All-Basis zusammengeführt, die unten beschrieben wird. Sie werden pro Benutzer unter `GET /managed/settings` mit ETag/304-Caching bereitgestellt.

```yaml theme={null}
managed:
  policies:
    # Spezifische Gruppen zuerst.
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
        permissions: { deny: ["WebFetch", "WebSearch"] }
    # Standard-Catch-All zuletzt: passt zu jedem, der sich authentifiziert hat.
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
```

Ein `match: {}`-Catch-All, üblicherweise zuletzt aufgelistet, wird als Basisschicht behandelt. Jede andere Richtlinie erbt jeden Schlüssel, den sie nicht von der Catch-All setzt, sodass Pro-Rollen-Einträge nur auflisten müssen, was sich vom Org-Standard unterscheidet. Die Zusammenführungsregeln hängen vom Schlüsseltyp ab:

* **Zulassungslisten**: `availableModels` und `permissions.allow`. Die Liste einer spezifischen Richtlinie ersetzt die Basis vollständig.
* **Ablehnungslisten und Hook-Arrays**: `permissions.deny`, `permissions.ask`, `disabledMcpjsonServers`, `deniedMcpServers`, `blockedMarketplaces` und jedes `hooks`-Event-Typ-Array. Diese nehmen die Vereinigung von Basis und Richtlinie, sodass ein Org-weiter Ablehnungs- oder Audit-Hook nicht versehentlich durch eine Pro-Rollen-Überschreibung gelöscht werden kann.
* **Record-typisierte Schlüssel**: `env`, `modelOverrides` und `skillOverrides`. Diese flach-zusammenführen, sodass ein Pro-Rollen-`env`-Block Schlüssel überschreibt, die er setzt, und den Rest von der Basis erbt.

`availableModels` wird auch Server-seitig unter `/v1/messages` durchgesetzt, sodass ein abgelehntes Modell `400` zurückgibt, unabhängig davon, was der Client sendet.

| Matcher                                             | Verhalten                                                                                                                                                                         |
| --------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `match: {}`                                         | Passt zu jedem authentifizierten Benutzer. Beginnen Sie mit einem davon und fügen Sie später Gruppen-gesteuerter Richtlinien darüber hinzu.                                       |
| `match: { groups: [a, b] }`                         | Passt, wenn der JWT-`groups`-Anspruch eine der aufgelisteten Gruppen enthält. Groß-/Kleinschreibung beachtet: Gruppen müssen die genaue Groß-/Kleinschreibung des IdP abgleichen. |
| `match: { email_domain: example.com }`              | Passt den Teil nach dem letzten `@` im JWT-`email`-Anspruch, Groß-/Kleinschreibung ignoriert. Akzeptiert eine Domäne pro Richtlinie.                                              |
| `match: { groups: [a], email_domain: example.com }` | Beide Bedingungen müssen passen                                                                                                                                                   |

Ein authentifizierter Benutzer, der keine Richtlinie passt, erhält die Gateway-Standardwerte, was bedeutet, jedes Modell im Katalog und keine verwalteten Einstellungen. Fügen Sie einen `match: {}`-Catch-All zuletzt hinzu, wenn Sie eine garantierte Standard-Richtlinie möchten.

<Note>
  Das Gateway führt kein eigenes Benutzerverzeichnis. Es autorisiert jede Anfrage vom IdP-Token des Benutzers, liest die Gruppenmitgliedschaft vom `groups`-Anspruch des Tokens und wertet Richtlinien dagegen aus. Es gibt kein Roster zum Aufzählen und keine Konten zum Vorerstellen, und daher keinen SCIM-Endpunkt, da es nichts gibt, das SCIM hinein synchronisieren könnte.

  Führen Sie Benutzer- und Gruppen-Lebenszyklusverwaltung an der Quelle der Wahrheit durch, die der native SCIM-Bereitstellung Ihres IdP oder eine dedizierte Identitäts-Governance-Plattform ist. Mitgliedschaft und Bereitstellung, die dort regiert werden, fließen automatisch durch den Token in das Gateway. Wenn Sie SCIM-Bereitstellung von Claude-Konten selbst möchten, das ist eine [Claude for Enterprise](/docs/de/admin-setup)-Fähigkeit.

  Zwei Ausbreitungsuhren gelten:

  * **Richtlinieninhalt**: Das Bearbeiten einer Richtlinie und das erneute Bereitstellen erreichen verbundene Clients bei ihrer nächsten verwalteten Einstellungsabfrage, innerhalb einer Stunde
  * **Gruppenmitgliedschaft**: Das Ändern der Gruppenmitgliedschaft eines Benutzers ändert, welche Richtlinie ihn passt. Dies tritt bei der nächsten Session-Neuerstellung in Kraft, was die nächste stille Aktualisierung bedeutet, begrenzt durch `session.ttl_hours`.
</Note>

<h4 id="what-goes-in-cli">
  Was geht in `cli`
</h4>

Jeder `cli`-Wert ist ein vollständiges Claude Code `managed-settings.json`-Dokument, das gleiche Schema, das Sie über MDM oder `/etc/claude-code/managed-settings.json` bereitstellen würden, hier als YAML ausgedrückt. Die CLI wendet das bereitgestellte Dokument auf der verwalteten Ebene an, über Benutzer- und Projekteinstellungen.

Das Gateway validiert jedes Dokument beim Start gegen das Einstellungsschema der CLI, sodass ein nicht erkannter Top-Level-Schlüssel oder ein erkannter Schlüssel mit einem fehlgeformten Wert beim Start mit einem Fehler fehlschlägt, der jeden fehlerhaften Schlüssel benennt. Absichtlich offene Teile des Schemas akzeptieren immer noch beliebige Werte, da neuere Clients Einträge erkennen können, die das Gateway-Schema nicht erkennt. Diese offenen Schlüssel sind `env`, `pluginConfigs` und Schlüssel, die unter `permissions` verschachtelt sind.

Da die Validierung das Schema verwendet, das mit der installierten Version des Gateways gebündelt ist, erfordert das Einfügen eines Top-Level-Einstellungsschlüssels, der von einer neueren Claude Code-Version eingeführt wurde, in verwaltete Konfiguration, das Gateway zuerst zu aktualisieren. Rauchtesten Sie eine neue Richtlinie auf einem Client, bevor Sie sie ausrollen.

Die vollständige Schlüsselreferenz ist in [Claude Code-Einstellungen](/docs/de/settings#available-settings). Die Schlüssel, die Operatoren zuerst erreichen:

```yaml theme={null}
managed:
  policies:
    - match: {}
      cli:
        # Modellzugriff (auch Server-seitig unter /v1/messages durchgesetzt)
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]

        # Berechtigungsrichtlinie
        permissions:
          deny:
            - "WebFetch"
            - "Read(./.env)"
            - "Read(./secrets/**)"
          disableBypassPermissionsMode: disable   # blockiert --dangerously-skip-permissions
        allowManagedPermissionRulesOnly: true     # ignoriert Benutzer-/Projektberechtigungsregeln

        # Umgebung in den CLI-Prozess gepusht. DISABLE_UPDATES blockiert
        # Hintergrund- und manuelle Updates; DISABLE_AUTOUPDATER stoppt nur
        # Hintergrund-Updates.
        env:
          DISABLE_UPDATES: "1"                    # pin-Versionen über Ihre eigene Verteilung

        # Org-weite Hooks. Hook-Befehle laufen auf Entwicklermaschinen, nicht dem
        # Gateway, sodass der Pfad auf jedem Client-OS in der Richtlinie existieren muss.
        hooks:
          PostToolUse:
            - matcher: "Edit|Write"
              hooks:
                - { type: command, command: /usr/local/bin/audit-edit.sh }
```

| Schlüssel                                  | Durchgesetzt von | Effekt                                                                                                                                                                                                                             |
| ------------------------------------------ | ---------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `availableModels`                          | Gateway + CLI    | Modell-Zulassungsliste. Auch unter `/v1/messages` überprüft, sodass ein gepatchter Client ihn nicht umgehen kann.                                                                                                                  |
| `permissions.allow` / `.deny`              | CLI              | Tool- und Befehlsregeln. Siehe [Berechtigungen](/docs/de/permissions).                                                                                                                                                                  |
| `permissions.disableBypassPermissionsMode` | CLI              | Setzen Sie auf `disable`, um [`bypassPermissions`](/docs/de/permission-modes#skip-all-checks-with-bypasspermissions-mode), den Modus, der jeden Tool-Aufruf auto-genehmigt, und das `--dangerously-skip-permissions`-Flag zu blockieren |
| `allowManagedPermissionRulesOnly`          | CLI              | Wenn `true`, werden Benutzer- und Projektberechtigungsregeln ignoriert; nur Regeln aus diesem Dokument gelten                                                                                                                      |
| `env`                                      | CLI              | Umgebungsvariablen, die in den CLI-Prozess zusammengeführt werden. Verwenden Sie für Telemetrie, Auto-Update und Modellnamen-Überschreibungen.                                                                                     |
| `hooks`                                    | CLI              | Org-weite [Hooks](/docs/de/hooks)                                                                                                                                                                                                       |

Da diese Einstellungen über das Netzwerk ankommen, zeigt die CLI jedem Entwickler einen einmaligen Sicherheitsgenehmigungsdialog, bevor etwas angewendet wird, das einen Shell-Befehl ausführen oder ändern kann, wohin Traffic geht. Der Dialog behandelt:

* `hooks`
* `env`-Variablen, die nicht auf der sicheren Liste der CLI sind
* Shell-Ausführungseinstellungen wie `apiKeyHelper` und `statusLine`
* verwalteter CLAUDE.md-Inhalt

Die sichere Liste bestimmt, welche `env`-Variablen ohne Genehmigung gelten:

* **Auf der sicheren Liste**: Auto-Update und Modellnamen-Variablen
* **Nicht auf der sicheren Liste**: Proxy-Variablen, Base-URL-Variablen und `OTEL_EXPORTER_OTLP_ENDPOINT`

Die [Telemetrie](#telemetry)-Konfiguration des Gateways pusht `OTEL_EXPORTER_OTLP_ENDPOINT`, sodass das Setzen von `telemetry.forward_to` den Dialog bei jedem interaktiven Client auslöst. Ein nicht-interaktiver Lauf mit dem `-p`-Flag kann den Dialog nicht anzeigen. Er wendet die gepushten Einstellungen nur für diesen Lauf an und speichert sie nicht als genehmigt, sodass die nächste interaktive Session des Entwicklers immer noch den Dialog anzeigt. Vor v2.1.207 speicherte ein nicht-interaktiver Lauf die Einstellungen als genehmigt und keine spätere interaktive Session zeigte den Dialog dafür.

Wenn ein Entwickler ablehnt, beendet Claude Code statt die Richtlinie anzuwenden. Das Pushen eines neuen Hooks oder einer nicht-sicheren Env-Variable zu einer breiten Richtlinie bedeutet daher eine Genehmigungsaufforderung bei jedem passenden Entwickler beim nächsten Start.

Der `cli`-Schlüssel wurde in früheren Releases `settings` genannt. Diese Schreibweise wird immer noch als Alias akzeptiert, aber neue Bereitstellungen sollten `cli` verwenden.

<h4 id="precedence-with-other-managed-sources">
  Vorrang mit anderen verwalteten Quellen
</h4>

Wenn ein Gerät auch eine lokale `managed-settings.json` oder MDM-bereitgestellte Richtlinie hat, werden die verwalteten Quellen nicht zusammengeführt. Die höchste Prioritätsquelle stellt alle Richtlinieneinstellungen bereit, in dieser Reihenfolge mit höchster Priorität zuerst:

1. Der [Richtlinien-Helper](/docs/de/settings#compute-managed-settings-with-a-policy-helper)
2. Gateway-bereitgestellte Einstellungen
3. MDM, über die HKLM-Registrierung unter Windows oder eine plist unter macOS
4. Die `managed-settings.json`-Datei
5. Die HKCU-Registrierung, nur unter Windows

Einbettungs-Hosts können Richtlinie durch die SDK-`managedSettings`-Option bereitstellen. Sie wird standardmäßig ignoriert und gilt nur, wenn eine verwaltete Quelle sich mit [`parentSettingsBehavior: "merge"`](/docs/de/settings#available-settings) anmeldet, gefiltert, sodass sie Richtlinie straffen, aber nicht lockern kann.

Die einzige Ausnahme ist die folgende Menge von Schlüsseln, die geehrt werden, wenn eine Admin-Quelle über der Benutzer-schreibbaren HKCU-Ebene sie setzt, unabhängig davon, welche Quelle den Rest der Richtlinie bereitstellt:

* `sandbox.network.allowManagedDomainsOnly` und `sandbox.filesystem.allowManagedReadPathsOnly`: wenn gesperrt, werden die entsprechenden Zulassungslisten über Quellen vereinigt
* [`allowAllClaudeAiMcps`](/docs/de/settings#available-settings): Zulassung-nur-Überschreibung für die claude.ai MCP-Server-Zulassungsliste
* `sandbox.bwrapPath` und `sandbox.socatPath`: Dateisystempfade zu den [Sandbox](/docs/de/sandboxing)-Helper-Binärdateien
* [`forceRemoteSettingsRefresh`](/docs/de/server-managed-settings): blockiert den Start, bis Remote-verwaltete Einstellungen frisch abgerufen werden, sodass eine MDM- oder Datei-Richtlinie, die sie setzt, geehrt wird, auch wenn ein gecachter Remote-Payload, dem der Schlüssel fehlt, die höchste Prioritätsquelle ist

Jeder andere Schlüssel, einschließlich `allowManagedPermissionRulesOnly` und `disableBypassPermissionsMode`, kommt nur von der höchsten Prioritätsquelle. Siehe [Einstellungs-Vorrang](/docs/de/settings#settings-precedence) für die gleiche Regel auf der Einstellungsseite.

Gateway-Richtlinien gelten für jeden Claude Code-Aufruf auf der Maschine, einschließlich nicht-interaktiver `claude -p`-Läufe und Sessions, die vom Agent SDK erzeugt werden. Wenn das Gateway beim Start nicht erreichbar ist, beenden sich angemeldete Sessions mit einem Fehler, anstatt ohne ihre Richtlinie zu laufen.

<Warning>
  `mcpServers` innerhalb eines Richtlinien-`cli`-Blocks wird beim Gateway-Start abgelehnt. Pro-Gruppen-MCP-Verteilung ist nicht verfügbar; stellen Sie MCP-Server über die dateibasierte `managed-mcp.json` auf jedem Gerät bereit oder lassen Sie Entwickler sie lokal hinzufügen.
</Warning>

<h3 id="telemetry">
  `telemetry`
</h3>

Die CLI sendet OpenTelemetry Protocol (OTLP) über HTTP-Metriken, Logs und, wenn aktiviert, Traces an das Gateway, das sie wörtlich an jedes konfigurierte Ziel weiterleitet. Siehe [Überwachung der Nutzung](/docs/de/monitoring-usage) für die Metriken und Ereignisse, die die CLI ausgibt.

Die CLI stempelt jeden Export mit der Identität des authentifizierten Benutzers, gelesen aus dem Gateway-ausgegebenen JWT: die `user.id`-, `user.email`- und `user.groups`-Attribute. Pro-Entwickler-Kosten- und Nutzungszuordnung funktioniert daher ohne Entwickler-seitige Konfiguration.

```yaml theme={null}
telemetry:
  forward_to:
    - url: https://otel-collector.internal.example.com
      headers:
        Authorization: ${OTLP_TOKEN}
      # Pro-Signal-Opt-In. Standard: nur Metriken.
      metrics: true
      logs: false
      traces: false
    - url: https://api.datadoghq.com/api/v2/otlp
      headers:
        DD-API-KEY: ${DD_API_KEY}
```

<Warning>
  Jedes Ziel meldet sich unabhängig in `metrics`, `logs` und `traces` an, und der Standard ist nur Metriken. Die Signale unterscheiden sich in Empfindlichkeit:

  * **Metriken**: Aggregatzähler wie Token-Zähler, Anfragezähler und Latenz
  * **Logs und Traces**: können vollständige Bash-Befehle, Tool-Eingaben und Dateipfade tragen, die alles abdecken, was Claude Code auf einer Entwicklermaschine tut

  Aktivieren Sie Logs und Traces nur auf Zielen mit den Zugriffskontrolle und Aufbewahrungsrichtlinie, die Daten rechtfertigen.
</Warning>

Telemetrie ist in der CLI standardmäßig aus. Das Konfigurieren von `telemetry.forward_to` zusammen mit `listen.public_url` schaltet es ein. Das Gateway pusht fünf Env-Variablen an jeden verbundenen Client durch `/managed/settings`:

* `CLAUDE_CODE_ENABLE_TELEMETRY=1`
* `OTEL_METRICS_EXPORTER=otlp`
* `OTEL_LOGS_EXPORTER=otlp`
* `OTEL_TRACES_EXPORTER=otlp`
* `OTEL_EXPORTER_OTLP_ENDPOINT=<public_url>`

Der gepushte Endpunkt wird aus der öffentlichen URL erstellt, sodass Metriken und Logs keine OTEL-Konfiguration von Entwicklern oder Richtlinien benötigen. Die gepushte Konfiguration wird auf der verwalteten Ebene angewendet, überschreibt `OTEL_*`-Variablen, die ein Entwickler lokal setzt.

[Traces](/docs/de/monitoring-usage#traces-beta) erfordern zusätzlich `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` auf jedem Client. Das Gateway pusht diese Variable nicht, also setzen Sie sie durch einen verwalteten Richtlinien-`env`-Block. Sie ist nicht auf der sicheren Liste der CLI, sodass die Bereitstellung durch eine Richtlinie durch den gleichen [Sicherheitsgenehmigungsdialog](#managed) abgedeckt ist, den der gepushte OTLP-Endpunkt bereits auslöst.

Sowohl Protobuf- als auch JSON-OTLP-Codierungen werden weitergeleitet, und jedes OpenTelemetry-kompatible Backend funktioniert als Ziel.

<h3 id="http-tuning">
  HTTP-Abstimmung
</h3>

Vier optionale Top-Level-Blöcke, `access_control`, `limits`, `timeouts` und `rate_limits`, stimmen die HTTP-Oberfläche ab. Die Standardwerte passen zu den meisten Bereitstellungen.

| Block            | Schlüssel                                      | Standard      | Beschreibung                                                                                                                                                                                                                                                                                                                                                               |
| ---------------- | ---------------------------------------------- | ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `access_control` | `allow_cidrs` / `deny_cidrs`                   | leer          | Eingehende IP-Zulassung/Ablehnung nach Client-Adresse, nach `trusted_proxies`-Auflösung. `deny_cidrs` wird zuerst überprüft; ein Client, den es passt, wird abgelehnt, auch wenn `allow_cidrs` auch passt. Wenn `allow_cidrs` nicht leer ist, ist das Gateway Standard-Ablehnung. `/healthz` und `/readyz` sind von `allow_cidrs` ausgenommen.                             |
| `limits`         | `max_request_bytes`                            | 32 MiB        | Max eingehende Anfragebody; übergroße Anfragen erhalten `413`, bevor der Body gepuffert wird. Erhöhen Sie für große Datei- oder Bildanfragen.                                                                                                                                                                                                                              |
| `limits`         | `max_request_header_bytes`                     | nicht gesetzt | Wenn gesetzt, geben übergroße Header `431` zurück                                                                                                                                                                                                                                                                                                                          |
| `limits`         | `max_url_length`                               | nicht gesetzt | Wenn gesetzt, gibt eine zu lange URL `414` zurück                                                                                                                                                                                                                                                                                                                          |
| `timeouts`       | `upstream_ttfb_ms`                             | 120000        | Max Wartezeit für die Response-Header des Upstream (Zeit bis erstes Byte). Der Response-Body streamt dann ohne Wall-Clock-Cap. Gilt für den direkten Anthropic-Upstream-Pfad; jeder andere Provider ist durch die eigenen Timeouts des Provider-SDK begrenzt.                                                                                                              |
| `rate_limits`    | `device_authorization.max` / `.window_seconds` | 30 / 600      | Pro-IP-Rate-Limit auf dem nicht authentifizierten Gerätegenehmigungsendpunkt. Erhöhen Sie für eine große Org hinter einer gemeinsamen Egress-IP oder NAT. Diese Limits gelten nur für den Gerätezuschuss-Anmeldungsfluss, nicht für `/v1/messages`-Inferenz. Siehe [Benutzercode-Brute-Force-Widerstand](/docs/de/claude-apps-gateway-deploy#user-code-brute-force-resistance). |
| `rate_limits`    | `device_verify.max` / `.window_seconds`        | 10 / 600      | Pro-IP-Rate-Limit auf `user_code`-Einreichungen unter `/device`                                                                                                                                                                                                                                                                                                            |

<h2 id="complete-example">
  Vollständiges Beispiel
</h2>

Diese vollständige Referenzkonfiguration behandelt jeden Kernabschnitt; die [HTTP-Abstimmungsblöcke](#http-tuning) behalten ihre Standardwerte. Kopieren Sie sie, löschen Sie, was Sie nicht brauchen, und füllen Sie Ihre Werte aus. Die Konfiguration im [Schnellstart](/docs/de/claude-apps-gateway#quickstart) ist eine minimale Version davon.

```yaml gateway.yaml theme={null}
# Laufen mit:
#   claude gateway --config gateway.yaml
#
# Operatives Log-Verbosity wird durch die Umgebungsvariable CLAUDE_GATEWAY_LOG_LEVEL
# gesteuert (info | warn | error; Standard info). Sie beeinflusst nicht
# Audit-Ereignisse, die immer ausgegeben werden.

listen:
  host: 0.0.0.0
  port: 8080
  public_url: https://claude-gateway.internal.example.com
  # Lassen Sie den tls-Block weg, wenn Sie hinter einem TLS-beendenden Ingress laufen.
  # tls:
  #   cert: /certs/gateway.crt
  #   key: /certs/gateway.key
  # trusted_proxies:
  #   - 10.0.0.0/8

oidc:
  issuer: https://example.okta.com
  client_id: 0oa1example2
  client_secret: ${OIDC_CLIENT_SECRET}
  allowed_email_domains:
    - example.com
  # Erforderlich, wenn der Aussteller der Okta-Org-Server ist, dessen id_tokens
  # E-Mail und Gruppen auslassen können; das Gateway füllt sie von /userinfo.
  userinfo_fallback: true
  # allowed_groups: [claude-code-users]
  # Okta gibt Gruppen nur aus, wenn der `groups`-Bereich angefordert wird und die
  # App-Gruppenanspruchsfilter sie erlauben. Die Contractor-Richtlinie unten
  # passt auf Gruppen, also wird der Bereich hier angefordert.
  scopes: [openid, profile, email, offline_access, groups]
  # extra_auth_params: { access_type: offline, prompt: consent }  # Google
  # groups_claim: groups          # Entra-App-Rollen: verwenden Sie `roles`
  # email_claim: email

session:
  jwt_secret: ${GATEWAY_JWT_SECRET}   # openssl rand -base64 32
  # ttl_hours: 1

store:
  postgres_url: ${GATEWAY_POSTGRES_URL}
  # max_connections: 5

# Aktiviert /v1/organizations/spend_limits (spiegelt die Anthropic Admin API)
# und Pro-Entwickler-Ausgabendurchsetzung auf /v1/messages. Lassen Sie weg, um zu deaktivieren.
# Caps selbst werden über die Admin API gesetzt, nicht hier.
# admin:
#   write_keys:
#     - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
#   read_keys:
#     - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
#   admin_groups: [platform-finops]
#   blocked_message: request an increase at https://go.example.com/claude-limits
#   # audit_retention_days: 365
#   # spend_retention_months: 13
#   # identity_retention_days: 90
#   # group_limit_mode: min

# enforcement:
#   fail_closed_on_error: false

upstreams:
  - provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}

  # - provider: bedrock
  #   region: us-east-1
  #   auth: {}

  # - provider: anthropicAws
  #   region: us-east-1
  #   workspace_id: wrkspc_...
  #   auth:
  #     api_key: ${ANTHROPIC_AWS_API_KEY}

  # - provider: vertex
  #   region: us-east5
  #   project_id: example-prod
  #   auth: {}

  # - provider: foundry
  #   resource: example-foundry
  #   auth: { use_azure_ad: true }

auto_include_builtin_models: true
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      anthropic: claude-opus-4-8
      # bedrock: us.anthropic.claude-opus-4-8
      # anthropicAws: claude-opus-4-8
      # vertex: claude-opus-4-8
      # foundry: <your-opus-deployment-name>
  - id: claude-sonnet-4-6
    label: Claude Sonnet 4.6
    upstream_model:
      anthropic: claude-sonnet-4-6
  - id: claude-haiku-4-5
    label: Claude Haiku 4.5
    upstream_model:
      anthropic: claude-haiku-4-5

managed:
  policies:
    - match: { groups: [contractors] }
      cli:
        availableModels: [claude-haiku-4-5]
        # Beschränken Sie die Standard-Picker-Option auf availableModels statt
        # der Tier-Standard, sodass Contractors keinen 400 auf dem Standard erhalten.
        enforceAvailableModels: true
        # allow genehmigt diese Tools automatisch; es blockiert nicht den Rest.
        # Fügen Sie deny-Regeln hinzu, um Tools zu beschränken.
        permissions: { allow: [Read, Grep] }
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
        permissions:
          allow: [Read, Grep, Bash, Edit]
          deny: ["WebFetch"]
        env: { HTTP_PROXY: http://proxy.example.com:8080 }

telemetry:
  forward_to:
    - url: https://otel.internal.example.com:4318
      headers:
        Authorization: Bearer ${OTEL_TOKEN}
```

<h2 id="client-side-managed-settings">
  Client-seitige verwaltete Einstellungen
</h2>

Alles oben konfiguriert den Gateway-Server. Das Zeigen von Entwicklermaschinen darauf wird separat auf jedem Gerät durch Claude Code's [verwaltete Einstellungen](/docs/de/settings#settings-files) konfiguriert. Das Gateway kann diese Schlüssel nicht selbst pushen, da sie dem Client sagen, wo das Gateway ist.

Für die CLI setzen Sie beide Schlüssel in die Pro-OS `managed-settings.json`:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com"
}
```

Stellen Sie diese Datei auf jedem Gerät bereit, typischerweise über Ihre MDM-Plattform. Der Dateipfad unterscheidet sich je nach Plattform:

| Plattform     | Pfad                                                                                                                               |
| ------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| macOS         | `/Library/Application Support/ClaudeCode/managed-settings.json`, oder die `com.anthropic.claudecode` verwaltete Präferenzen-Domäne |
| Linux und WSL | `/etc/claude-code/managed-settings.json`                                                                                           |
| Windows       | `C:\Program Files\ClaudeCode\managed-settings.json`, oder Gruppenrichtlinie über die HKLM-Registrierung                            |

`forceLoginGatewayUrl` und der `"gateway"`-Wert von `forceLoginMethod` werden nur von der Admin-kontrollierten verwalteten Ebene geehrt. Ein Entwickler, der sie in seinen eigenen `~/.claude/settings.json` setzt, hat keine Auswirkung.

<h2 id="related">
  Verwandt
</h2>

* [Claude Apps Gateway-Übersicht](/docs/de/claude-apps-gateway): Schnellstart und Entwickler-Verbindung
* [Bereitstellungsleitfaden](/docs/de/claude-apps-gateway-deploy): IdP-Setup, Container-Image, Kubernetes und Cloud Run sowie Operationen
* [Ausgabenlimits](/docs/de/claude-apps-gateway-spend-limits): Pro-Entwickler-Caps und die Admin API
