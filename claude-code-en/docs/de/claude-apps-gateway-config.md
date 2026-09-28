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
* [`pricing`](#pricing): vertraglich vereinbarte Sätze und ein Rabattmultiplikator für das Ausgabenmessgerät und für die Kostenzahlen, die Entwickler sehen
* [`models`](#models) und `auto_include_builtin_models`: von Admin kuratierte Modellliste und Pro-Upstream-IDs
* [`managed`](#managed): verwaltete Einstellungsrichtlinien nach IdP-Gruppe
* [`telemetry`](#telemetry): OTLP-Weiterleitung an Ihren Observability-Stack
* [`access_control`, `limits`, `timeouts`, `rate_limits`](#http-tuning): IP-Zulassung/Ablehnung, Anfragegrößenbeschränkungen, Upstream-Zeit-bis-erstes-Byte und Pro-IP-Anmeldungslimits
* [`load_test_mode`](#load_test_mode): Lasttests des Gateways ohne Aufruf eines Modellanbieters

<h2 id="secret-expansion">
  Geheimniserweiterung
</h2>

Schreiben Sie Geheimnisse wie `client_secret`, `jwt_secret` oder `postgres_url` nicht direkt in `gateway.yaml`. Referenzieren Sie sie mit einem der folgenden Formulare, und das Gateway löst den Wert beim Start aus einer Umgebungsvariablen oder einer Datei auf:

| Formular        | Wird aufgelöst zu                                                                                                                                                                                                                                                         | Verwenden für                                                        |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| `${VAR}`        | Die Umgebungsvariable `VAR`. Der Start schlägt fehl, wenn nicht definiert.                                                                                                                                                                                                | Container-Umgebungsvariablen, AWS Secrets Manager über Env-Injektion |
| `${file:/path}` | Inhalt der Datei unter diesem absoluten Pfad, gekürzt. Die Referenz muss der gesamte Wert des Feldes sein: Im Gegensatz zu `${VAR}` wird sie nicht in einer längeren Zeichenkette erweitert. Setzen Sie daher `store.password` anstatt sie in `postgres_url` einzubetten. | Kubernetes Secret-Volume-Mounts, Vault Agent, SOPS                   |

<h2 id="required-sections">
  Erforderliche Abschnitte
</h2>

<h3 id="listen">
  `listen`
</h3>

Der `listen`-Block steuert, wo das Gateway bereitgestellt wird: die Bindungsadresse und der Port, der extern sichtbare Ursprung und optionale TLS-Beendigung.

| Feld                   | Erforderlich                     | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ---------------------- | -------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `host`                 | Nein                             | Bindungsadresse. Standard `0.0.0.0`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `port`                 | Nein                             | Bindungsport. Standard `8080`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `public_url`           | Sofern `host` nicht loopback ist | Der extern sichtbare `https://`-Ursprung, der zum Erstellen des IdP-`redirect_uri` und der Discovery-Metadaten verwendet wird. Erforderlich, wenn `host` keine Loopback-Adresse ist, unabhängig davon, ob TLS bei einem Proxy wie ALB, Ingress oder Cloud Run oder beim Gateway selbst über `tls` beendet wird, da das Gateway seinen eigenen Ursprung niemals aus `X-Forwarded-*`-Headern ableitet; diese können vom Client gefälscht werden. Der Start schlägt ohne diese fehl. `trusted_proxies` unten regelt nur die Client-IP-Auflösung. Auch erforderlich, um [Telemetrie](#telemetry) zu aktivieren, da das Gateway den OTLP-Endpunkt, den es an Clients überträgt, aus dieser URL erstellt. |
| `tls.cert` / `tls.key` | Nein                             | PEM-Pfade, wenn das Gateway TLS selbst beendet                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `trusted_proxies`      | Nein                             | CIDRs oder IPs von Load Balancern vor dem Gateway. Wenn gesetzt, vertraut das Gateway `X-Forwarded-For` nur von diesen Peers und zeichnet die echte Client-IP für Pro-IP-Ratenbegrenzung und Audit auf. Äquivalent zu nginx `set_real_ip_from`. `X-Forwarded-For`-Einträge, die als `ipv4:port` oder `[ipv6]:port` geschrieben sind, wie es einige Load Balancer tun, werden mit dem Port gelesen, der gelöscht wird. Eine IPv6-Adresse mit angehängtem Port und ohne Klammern kann als eine andere Adresse gelesen werden oder überhaupt nicht gelesen werden, daher deaktivieren Sie die Port-Option auf jedem Proxy, der diese Form schreibt.                                                    |

<h3 id="oidc">
  `oidc`
</h3>

Der `oidc`-Block verbindet das Gateway mit Ihrem Identitätsanbieter und entscheidet, wer sich anmelden kann. Er benennt den Aussteller und OAuth-Client, ordnet die Ansprüche zu, die E-Mail und Gruppen enthalten, und beschränkt die Anmeldung nach E-Mail-Domäne oder Gruppe.

OpenID Connect (OIDC) ist das SSO-Protokoll, das das Gateway mit Ihrem Identitätsanbieter verwendet; siehe [Identitätsanbieter-Setup](/docs/de/claude-apps-gateway-deploy#identity-provider-setup) für das, was auf der IdP-Seite registriert werden muss.

| Feld                            | Erforderlich | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| ------------------------------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `issuer`                        | Ja           | OIDC-Discovery-Basis. Muss Discovery unter `/.well-known/openid-configuration` bereitstellen. Verwenden Sie HTTPS in der Produktion; das Gateway akzeptiert einen `http://`-Aussteller. Ein Loopback-Aussteller wie `http://localhost:8081` wird vom [SSRF-Schutz](/docs/de/claude-apps-gateway-deploy#threat-model-summary) abgelehnt, sofern `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` in der Umgebung des Gateways nicht gesetzt ist.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `client_id` / `client_secret`   | Ja           | Aus Ihrer OAuth-Client-Registrierung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `allowed_email_domains`         | Nein         | Lehnen Sie id\_tokens ab, deren `email`-Anspruch nicht in einer dieser Domänen liegt, Groß-/Kleinschreibung wird ignoriert. Defense-in-Depth gegen Multi-Tenant-IdP-Fehlkonfiguration. Unabhängig von dieser Einstellung wird ein id\_token, dessen `email_verified`-Anspruch explizit `false` ist, immer abgelehnt.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `allowed_groups`                | Nein         | Beschränken Sie die Anmeldung auf Mitglieder dieser IdP-Gruppen, abgeglichen gegen `groups_claim`. Ein Benutzer in einer zulässigen E-Mail-Domäne, aber in keiner dieser Gruppen, wird abgelehnt. Erfordert, dass der IdP den Gruppenanspruch ausgibt. Der Abgleich ist ein exakter, Groß-/Kleinschreibung beachtender Zeichenfolgenvergleich gegen die Werte in diesem Anspruch, und das Gateway erweitert verschachtelte Gruppen nicht: Um Mitglieder einer Untergruppe zuzulassen, listen Sie die Untergruppe hier auf oder konfigurieren Sie den IdP so, dass er flache Mitgliedschaften ausgibt.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `groups_claim`                  | Nein         | Welcher id\_token-Anspruch trägt die Gruppenmitgliedschaft. Standard `groups`. Microsoft Entra gibt App-Rollen unter `roles` aus. Akzeptiert einen flachen Schlüssel oder einen RFC-6901-JSON-Pointer wie `/resource_access/gateway/roles` für verschachtelte Ansprüche.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `google_groups`                 | Nein         | Schlagen Sie die Gruppen des angemeldeten Benutzers über die Google Workspace Admin SDK Directory API nach, da Googles id\_token keinen Gruppenanspruch trägt. Setzen Sie `service_account_json_path` auf eine Service-Account-Schlüsseldatei mit Domain-weiter Delegierung im Bereich `https://www.googleapis.com/auth/admin.directory.group.readonly`, und `admin_email` auf einen Workspace-Administrator, den der Service Account annimmt; die Directory API erfordert ein echtes Admin-Subjekt. Die E-Mail-Adressen jeder Benutzergruppe werden zu ihrem Gruppenanspruch, daher stimmen `allowed_groups` und `managed.policies.match.groups` mit Gruppen-E-Mails überein.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `email_claim`                   | Nein         | Welcher id\_token-Anspruch trägt die E-Mail des Benutzers. Standard `email`. Einige IdPs wie ADFS und Entra B2C geben stattdessen `upn` oder `preferred_username` aus. Akzeptiert einen flachen Schlüssel, einen JSON-Pointer oder eine Liste von Fallback-Schlüsseln, wobei der erste vorhandene Schlüssel verwendet wird.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `scopes`                        | Nein         | Vollständige Überschreibung der OIDC-Bereiche, die das Gateway anfordert. Standard `[openid, profile, email, offline_access]`. Setzen Sie, wenn Ihr IdP Bereiche ablehnt, die er nicht erkennt, oder einen benutzerdefinierten Bereich erfordert, um Gruppen oder E-Mail auszugeben. Muss `openid` enthalten. Das Löschen von `offline_access` deaktiviert Aktualisierungstoken, daher führen Entwickler die Browser-Anmeldung alle `session.ttl_hours` erneut aus. Siehe [Identitätsanbieter-Setup](/docs/de/claude-apps-gateway-deploy#identity-provider-setup) für IdP-spezifische Bereichsrezepte wie Googles Aktualisierungstoken-Flow.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `scope_on_refresh`              | Nein         | Senden Sie auch `scope` mit der gleichen Liste wie die Anmeldeanfrage, wenn das Gateway ein Aktualisierungstoken austauscht. Standard `false`: die Aktualisierungsanfrage lässt `scope` weg. Die meisten IdPs geben bei jeder Aktualisierung ein id\_token zurück und benötigen dies nicht. Setzen Sie `true`, wenn Ihr IdP ein id\_token bei Aktualisierung nur zurückgibt, wenn `openid` erneut angefordert wird, was Okta für seinen Aktualisierungszuschuss dokumentiert. Ohne ein id\_token hängt jede Aktualisierung davon ab, dass der Userinfo-Endpunkt des IdP das aktualisierte Zugriffstoken akzeptiert. Wenn Sie die Anmeldung oder Richtlinienabgleiche auf Gruppen beschränken und das id\_token Ihres IdP zur Aktualisierungszeit diese auslässt, setzen Sie auch `userinfo_fallback: true`, damit das Gateway diese vom Userinfo-Endpunkt ausfüllt. Ein IdP, der weniger Bereiche als angefordert gewährte, kann die Aktualisierung mit `invalid_scope` ablehnen, auch für bestehende Sitzungen, wenn Sie Einträge zu `scopes` hinzufügen, während dies aktiviert ist. Heben Sie den Schlüssel auf, wenn Aktualisierungen nach dem Setzen am `token_endpoint` fehlschlagen. Erfordert Claude Code v2.1.260 oder später auf dem Gateway-Server. |
| `extra_auth_params`             | Nein         | Zusätzliche Abfrageparameter, die wörtlich an die IdP-Autorisierungsanfrage angehängt werden. Dies ist der Überschreibungsmechanismus für IdP-spezifisches Verhalten, wie `access_type: offline` für Google-Aktualisierungstoken, `domain_hint` für einige Entra-Mandanten oder `acr_values` für Step-up-Flows. Kann die vom Gateway verwalteten Protokollparameter nicht überschreiben: `state`, `nonce`, `redirect_uri`, PKCE, `scope`, `response_type`, `response_mode` und `client_id`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `userinfo_fallback`             | Nein         | Wenn das id\_token E-Mail oder Gruppen auslässt, rufen Sie diese von `/userinfo` ab. Erforderlich für Keycloak-Lightweight-Zugriffstokens, den Okta-Org-Server und ADFS-Minimal-Tokens. Das id\_token bleibt maßgeblich; userinfo füllt nur Lücken. Standard `false`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `use_pkce`                      | Nein         | Senden Sie eine PKCE-Herausforderung (S256) in der Autorisierungsanfrage. Standard `true`. Setzen Sie `false` nur, wenn Ihr IdP PKCE für diesen vertraulichen Client ablehnt.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `clock_skew_seconds`            | Nein         | Tolerieren Sie Uhrenabweichungen beim Validieren von id\_token-Zeitansprüchen. Standard `0`, was streng ist. Erhöhen Sie, wenn Sie unmittelbar nach der Anmeldung aufgrund von Host-/IdP-Uhrenabweichung Fehler „Token abgelaufen / noch nicht gültig" sehen.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `token_endpoint_auth_method`    | Nein         | Überschreiben Sie die Token-Endpunkt-Authentifizierungsmethode. Akzeptiert `client_secret_basic` oder `client_secret_post`. Standardmäßig automatisch ausgehandelt.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `id_token_signed_response_alg`  | Nein         | Erwarteter id\_token-Signaturalgorithmus. Standard `RS256`. Setzen Sie für IdPs, die mit ES256, PS256 oder EdDSA signieren.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `additional_authorized_parties` | Nein         | Zusätzliche `azp`-Werte, die neben `client_id` akzeptiert werden, für Keycloak-Broker und Token-Exchange-Flows                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `discovery_url`                 | Nein         | Rufen Sie das Discovery-Dokument von dieser URL ab, anstatt es von `issuer` abzuleiten, für IdPs hinter einem Proxy, der den Aussteller-Host umschreibt. Der Pfad muss `/.well-known/` enthalten.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `use_proxy`                     | Nein         | Senden Sie die eigenen IdP-Anfragen des Gateways durch den Forward-Proxy in `HTTPS_PROXY` oder `HTTP_PROXY`, wobei `NO_PROXY` beachtet wird. `false` hält diese Anfragen direkt. Erfordert v2.1.227 oder später; siehe [IdP-Anfragen durch einen Forward-Proxy](#idp-requests-through-a-forward-proxy) unten.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `form_action_origins`           | Nein         | Zusätzliche Ursprünge für die `Content-Security-Policy: form-action`-Direktive der `/device`-Seite. Das Gateway erlaubt bereits `'self'` und den erkannten `authorization_endpoint`-Ursprung, aber Chrome erzwingt `form-action` gegen die gesamte Umleitungskette. Wenn Ihr IdP durch einen zweiten Host umleitet, wie Azure AD, das zu ADFS verbunden ist, Hub-Spoke-Okta oder ein unternehmensweiter SSO-Interceptor, listen Sie jeden Ursprung auf, durch den die Autorisierungsanfrage umgeleitet werden kann.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `ca_cert_pem`                   | Nein         | Das PEM-codierte CA-Zertifikat selbst, nicht ein Pfad zu einer Datei. Es ersetzt den System-Trust-Store nur für IdP-Anfragen. Um eine bereitgestellte Datei zu laden, schreiben Sie `${file:/etc/gateway/idp-ca.pem}`. Verwenden Sie für Keycloak oder Dex hinter unternehmensweiter PKI.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |

<h4 id="idp-requests-through-a-forward-proxy">
  IdP-Anfragen durch einen Forward-Proxy
</h4>

Die Inference-Upstreams beachten `HTTPS_PROXY` und `HTTP_PROXY` in jeder Version. Die eigenen Anfragen des Gateways an den IdP, Discovery, JWKS, Token und Userinfo gehen direkt, sofern Sie nicht `oidc.use_proxy: true` setzen, was v2.1.227 oder später erfordert. Wenn eine Proxy-Variable gesetzt ist, `use_proxy` nicht gesetzt ist und der Aussteller nicht von `NO_PROXY` abgedeckt ist, hält das Gateway diese Anfragen direkt und protokolliert beim Start einen Hinweis, der Sie auffordert, eine Wahl zu treffen; `use_proxy: false` hält sie direkt und unterdrückt den Hinweis.

Mit `use_proxy: true` löst der Pod den Hostnamen jedes IdP-Endpunkts selbst auf und fordert den Proxy auf, sich mit der aufgelösten IP-Adresse zu `CONNECT`, daher muss der Proxy `CONNECT` zur IP-Adresse jedes Hosts akzeptieren, den das Discovery-Dokument benennt, nicht nur den Aussteller. Verwenden Sie eine `http://`-Proxy-URL. `ca_cert_pem` und der [SSRF-Schutz](/docs/de/claude-apps-gateway-deploy#threat-model-summary) gelten auch auf dem Proxy-Pfad.

[Proxy-only Egress](#proxy-only-egress) ändert beide: Während es aktiv ist, folgen IdP-Anfragen dem Proxy, sofern Sie nicht `use_proxy: false` setzen, und das Gateway übergibt dem Proxy jeden IdP-Hostnamen, ohne ihn zuerst aufzulösen.

<h4 id="proxy-only-egress">
  Proxy-only Egress
</h4>

Setzen Sie `CLAUDE_GATEWAY_PROXY_IS_EGRESS_BOUNDARY=1` in der Umgebung des Gateways, neben `HTTPS_PROXY`, wenn der Pod andere Hosts nur durch diesen Forward-Proxy erreicht und öffentliche DNS-Namen nicht selbst auflösen kann, oder wenn der Proxy `CONNECT` zu einer IP-Adresse ablehnt. Erfordert v2.1.277 oder später. Es ist eine Umgebungsvariable statt eines `gateway.yaml`-Schlüssels, daher kann nichts in der Konfigurationsdatei die Adressprüfung des Gateways lockern.

```bash theme={null}
export HTTPS_PROXY=http://proxy.corp.example.com:3128
export NO_PROXY=
export no_proxy=
export CLAUDE_GATEWAY_PROXY_IS_EGRESS_BOUNDARY=1
```

Das Gateway protokolliert eine `network:`-Zeile beim Start, während Proxy-only Egress aktiv ist.

Jede Zeile unten ist eine Klasse von ausgehenden Anfragen auf einem Gateway mit `HTTPS_PROXY` gesetzt, standardmäßig und während Proxy-only Egress aktiv ist.

| Ausgehende Anfrage                                                                                                            | Standard                                                                                                                                                                        | Proxy-only Egress aktiv                                                                              |
| ----------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| `provider: anthropic`-Upstreams, Workload Identity Federation-Token-Austausch, `telemetry.forward_to`-Exporte                 | Lokal aufgelöst und überprüft, dann `CONNECT` zur überprüften IP-Adresse durch den Proxy. Ein in `NO_PROXY` aufgelisteter Telemetrie-Collector wird stattdessen direkt erreicht | Hostname an den Proxy übergeben                                                                      |
| IdP-Discovery, JWKS, Token und Userinfo                                                                                       | Direkt, sofern nicht [`oidc.use_proxy: true`](#idp-requests-through-a-forward-proxy), dann `CONNECT` zur überprüften IP-Adresse                                                 | Hostname an den Proxy übergeben, sofern nicht `oidc.use_proxy: false` hält einen internen IdP direkt |
| Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform und Microsoft Foundry-Upstreams; Google-Gruppen-Lookups | Hostname an den Proxy übergeben                                                                                                                                                 | Unverändert                                                                                          |

Proxy-only Egress bleibt aus, sofern die Umgebung des Gateways nicht alle drei dieser Bedingungen erfüllt:

* `HTTPS_PROXY` oder `HTTP_PROXY` ist gesetzt.
* `NO_PROXY` und `no_proxy` sind leer. Wenn Ihre Plattform eines in Pods injiziert, setzen Sie beide auf einen leeren Wert auf dem Gateway-Container. Das Auflisten eines Telemetrie-Collectors in `NO_PROXY` hält Proxy-only Egress aus.
* `CLAUDE_GATEWAY_ALLOW_LOOPBACK` ist nicht aktiviert. Ein Collector oder IdP auf dem eigenen Loopback des Pods kann nicht mit Proxy-only Egress kombiniert werden, da eine an den Proxy übergebene Loopback-Adresse die des Proxy-Hosts selbst wäre, daher geben Sie diesen Services stattdessen eine Adresse, die der Proxy erreichen kann. Aus dem gleichen Grund weigert sich das Gateway, `localhost`-ähnliche Namen direkt zu akzeptieren, während Proxy-only Egress aktiv ist.

Wenn eine dieser Bedingungen nicht erfüllt ist, protokolliert das Gateway beim Start eine Warnung, die die Variable benennt, die es gestoppt hat, und behält das Standardverhalten.

Sobald Proxy-only Egress aktiv ist, erlauben Sie jeden Ziel im Proxy, einschließlich eines internen Collectors und jeden Host, der durch IP-Adresse konfiguriert ist. Sie können immer noch einen internen IdP direkt mit [`oidc.use_proxy: false`](#idp-requests-through-a-forward-proxy) halten.

<Warning>
  Aktivieren Sie dies nur, wenn die Allowlist des Proxys mindestens so streng ist wie die eigene Prüfung des Gateways. Der Proxy muss Cloud-Metadaten-Endpunkte wie `169.254.169.254` und `metadata.google.internal`, Link-Local-Adressen und das eigene Loopback des Proxy-Hosts ablehnen, und er muss sie nach der Adresse ablehnen, zu der ein Name aufgelöst wird, nicht nur nach Name, da das Gateway einen Hostnamen, der zu einem von ihnen aufgelöst wird, nicht mehr abfängt. Ein Proxy, der überall verbunden ist, wo er gefragt wird, entfernt den [SSRF-Schutz](/docs/de/claude-apps-gateway-deploy#threat-model-summary) des Gateways für diese Anfragen.
</Warning>

<h3 id="session">
  `session`
</h3>

Der `session`-Block formt die Bearer-Tokens, die das Gateway nach der Anmeldung ausgibt: das Geheimnis, das sie signiert, und wie lange sie leben.

| Feld         | Erforderlich | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ------------ | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `jwt_secret` | Ja           | Mindestens 32 Bytes Entropie, zum Beispiel von `openssl rand -base64 32`. Signiert die HS256-Bearer-Tokens des Gateways. Akzeptiert eine einzelne Zeichenkette oder ein Array zur Rotation: Index 0 signiert und alle Einträge verifizieren. Zum Rotieren fügen Sie ein neues Geheimnis vorne hinzu, warten `ttl_hours`, dann löschen Sie das alte.                                                                                                                                                                             |
| `ttl_hours`  | Nein         | Lebensdauer des Gateway-Bearer-Tokens. Standard `1`. Die CLI aktualisiert automatisch vor Ablauf, wenn der IdP Aktualisierungstoken ausgibt. Eine kürzere Lebensdauer hebt die Bereitstellung schneller auf; eine längere macht weniger IdP-Roundtrips. Wenn Ihr IdP keine Aktualisierungstoken ausstellen kann, weil `offline_access` nicht verfügbar ist, gibt es keine automatische Aktualisierung, daher erhöhen Sie dies auf `8` oder `12`, um zu vermeiden, dass Entwickler stündlich zur Browser-Anmeldung zurückkehren. |

<h3 id="store">
  `store`
</h3>

Der `store`-Block verweist das Gateway auf seine PostgreSQL-Datenbank, die Gerätezuschüsse und Ratenbegrenzungszähler enthält.

| Feld                      | Erforderlich | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ------------------------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `postgres_url`            | Ja           | `postgres://` oder `postgresql://` URL. Erforderlich: das Gerätezuschuss-Rendezvous, wo der Browser-Callback schreibt und die Polling-CLI liest, benötigt Zustand über Replikas hinweg. Das Gateway führt seine eigenen Schema-Migrationen beim Start und bei Upgrades aus, daher benötigt die Rolle Rechte zum Erstellen und Ändern von Tabellen im Zielschema. Siehe [Upgrades](/docs/de/claude-apps-gateway-deploy#upgrades) und [Postgres](/docs/de/claude-apps-gateway-deploy#postgres). |
| `username`                | Nein         | Überschreibt den Benutzer in `postgres_url`                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `password`                | Nein         | Datenbankberechtigungsnachweis. Setzen Sie ihn hier anstelle von `postgres_url`, damit der Berechtigungsnachweis aus der URL bleibt. Akzeptiert beliebige Zeichen und hat Vorrang vor URL-Berechtigungsnachweisen.                                                                                                                                                                                                                                                                  |
| `max_connections`         | Nein         | Postgres-Verbindungspool-Größe pro Replik. Standard `5`, was konservativ und freundlich zu gemeinsamen Datenbanken ist. Mit [Ausgabenlimits](#admin) aktiviert, führt der Hot-Path einige Operationen pro Inference-Anfrage durch, daher erhöhen Sie ihn für eine dedizierte Datenbank unter Last, und halten Sie Replikas × dies unter dem `max_connections` der Datenbank.                                                                                                        |
| `connect_timeout_seconds` | Nein         | Sekunden, die das Gateway wartet, wenn es eine Postgres-Verbindung öffnet. Eine ganze Zahl von `1` bis `60`, Standard `5`. Erhöhen Sie, wenn Verbindungsversuche zeitlich überschritten werden, wenn eine neue Gateway-Instanz startet. Erfordert Claude Code v2.1.274 oder später auf dem Gateway-Server. Frühere Versionen weigern sich zu starten, wenn der Schlüssel gesetzt ist.                                                                                               |

Für die lokale Entwicklung verweisen Sie `postgres_url` auf einen Wegwerf-Postgres-Container, zum Beispiel `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`.

<h3 id="upstreams">
  `upstreams`
</h3>

`upstreams` ist eine geordnete Liste. Das Gateway leitet Inference an den ersten Upstream weiter, der das angeforderte Modell auflöst.

Bei `5xx`, `429`, `401`, `403`, `404` oder Timeout schlägt das Gateway zum nächsten Upstream fehl über; andere `4xx` nicht, da diese Fehler dem Request statt dem Upstream zuzuordnen sind. Ein `401` oder `403` bedeutet, dass die eigene Berechtigung des Gateways gegen diesen Upstream fehlgeschlagen ist. Ein `404` bedeutet, dass dieser Upstream das angeforderte Modell nicht bereitstellt, daher kann ein späterer Upstream in der Liste es immer noch tun.

Wenn Sie `forward_user_identity: true` auf einem Upstream setzen, schlägt ein `429`, das dieser auf eine Anfrage zurückgibt, die die E-Mail des Entwicklers trug, nicht fehl über. Siehe [wie eine Pro-Benutzer-Limit-Ablehnung den Entwickler erreicht](#per-user-identity-headers-for-a-proxy-you-run).

Failover bei `404` erfordert Gateway v2.1.198 oder später. Frühere Versionen gaben den ersten `404` an den Client zurück, auch wenn ein späterer Upstream in der Liste das Modell bereitstellte.

Mehrere Upstreams desselben Anbieters müssen einen unterschiedlichen `name:` setzen.

Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform und Microsoft Foundry-Clients werden beim Start einmal erstellt, und ihre SDKs aktualisieren Berechtigungsnachweise intern, daher erfordert das Rotieren von Cloud-Berechtigungsnachweisen keinen Neustart. Statische Anthropic-API-Schlüssel und Bearer werden beim Start gelesen; siehe [Anthropic API](#anthropic-api).

<h4 id="upstream-error-messages">
  Upstream-Fehlermeldungen
</h4>

Das Gateway gibt die Fehlerantwort eines Upstreams oder sein eigenes `502` zurück, je nachdem, wie die Upstreams antworteten:

* **Ein Upstream gab einen Status zurück, bei dem das Gateway nicht [fehlschlägt über](#multiple-upstreams)**: diese Upstream-Antwort. Das Gateway versucht keine weiteren Upstreams.
* **Jeder Upstream, den das Gateway versuchte, schlug auf eine Weise fehl, bei der es [fehlschlägt über](#multiple-upstreams)**: das letzte `429`. Wenn keiner ein `429` zurückgab, bevorzugt das Gateway in der Reihenfolge das letzte `401` oder `403`, das letzte `404` und das letzte `501`. Wenn keiner von diesen zurückgab, das eigene `502` des Gateways, `all upstreams failed (N attempted)`, wobei N jeden Eintrag in [`upstreams`](#upstreams) zählt, einschließlich Einträge, die das Gateway übersprungen hat, weil sie das angeforderte Modell nicht bereitstellen.

Wenn das Gateway eine Upstream-Antwort zurückgibt, behält es den Statuscode des Upstreams. Ob es die Nachricht des Upstreams behält, hängt vom Anbieter ab. Eine Fehlerantwort eines Anthropic-API-Upstreams erreicht den Entwickler unverändert.

Die Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform und Microsoft Foundry-Upstreams können Ihre Konto-IDs, Rollen-ARNs und Projekt-IDs in ihrem Fehlertext benennen. Das Gateway zeichnet diesen vollständigen Text im [Betriebsprotokoll](/docs/de/claude-apps-gateway-deploy#logs) auf. Was der Entwickler von diesen Upstreams sieht, hängt von der Ablehnung ab:

* `400` oder `413` in Anthropics Standard-Fehler-Envelope: die eigene Nachricht des Upstreams, wie `prompt is too long`. Claude Platform on AWS, Agent Platform und Microsoft Foundry geben dieses Envelope für Modell-API-Ablehnungen zurück.
* `400` oder `413` in der eigenen Form des Anbieters: ein `capability_rejected:`-Token. Wenn das Gateway die Ablehnung nicht klassifizieren kann, `upstream rejected the request` bei einem `400` oder `request too large for this upstream` bei einem `413`.
* Jeder andere Status: generischer Pro-Status-Text, wie `upstream rate limit exceeded` bei einem `429`.

Zum Beispiel ersetzt das Gateway Amazon Bedrocks `Input is too long for requested model.` durch `capability_rejected: prompt_too_long`. Claude Code [komprimiert automatisch](/docs/de/errors#prompt-is-too-long) auf dieses Token, wie es auf `prompt is too long` tut.

Das Beibehalten einer Cloud-Upstream-Nachricht von `400` oder `413` oder das Ersetzen durch ein `capability_rejected:`-Token erfordert Gateway v2.1.233 oder später.

<h4 id="anthropic-api">
  Anthropic API
</h4>

Der minimale Anthropic-Upstream ist ein API-Schlüssel aus der [Claude Console](https://platform.claude.com):

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}
    # OR an OAuth bearer (e.g. a Workload-Identity-Federation-exchanged token):
    #   oauth_token: ${file:/var/run/secrets/anthropic-oauth-token}
    # base_url: https://api.anthropic.com   # default; override for a forward proxy
```

Die zwei Berechtigungsnachweis-Formen unterscheiden sich im Header, den sie senden:

* **`api_key`**: sendet `x-api-key`. Rotieren Sie ihn in der Claude Console und aktualisieren Sie die Umgebungsvariable.
* **`oauth_token`**: sendet `Authorization: Bearer`. Verwenden Sie die Bearer-Form, wenn Ihre Organisation kurzlebige Tokens statt langlebiger API-Schlüssel ausgibt. Der Bearer wird einmal beim Start gelesen, daher aktualisieren Sie durch Remounten des Geheimnisses und Neustart.

Anstelle eines statischen Schlüssels oder Bearers können Sie Workload Identity Federation verwenden. Erstellen Sie eine Verbindungsregel, indem Sie dem [Workload Identity Federation-Leitfaden](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation) folgen, dann mounten Sie das OIDC-JWT Ihrer Workload als Datei, wie ein Kubernetes-projiziertes Service-Account-Token oder ein ID-Token einer CI-Plattform. Das Gateway tauscht das JWT gegen einen kurzlebigen Bearer aus und aktualisiert ihn automatisch. Die Token-Datei wird bei jedem Austausch erneut gelesen, daher werden rotierte projizierte Tokens ohne Neustart aufgegriffen.

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      federation_rule_id: ${ANTHROPIC_FEDERATION_RULE_ID}
      organization_id: ${ANTHROPIC_ORGANIZATION_ID}
      identity_token_file: /var/run/secrets/anthropic/id-token
      # workspace_id: wrkspc_...       # required if the rule covers >1 workspace
      # service_account_id: svac_...   # optional expected-target check
```

<a id="per-user-identity-headers-for-a-proxy-you-run" />

<h5 id="per-user-identity-headers-for-a-proxy-you-run">
  Pro-Benutzer-Identitäts-Header für einen Proxy, den Sie betreiben
</h5>

Sie können die `base_url` eines `provider: anthropic`-Upstreams auf einen Proxy verweisen, den Sie betreiben, anstatt auf die Anthropic API. Um diesem Proxy mitzuteilen, welcher Entwickler jede Anfrage gesendet hat, setzen Sie `forward_user_identity: true` auf diesem Upstream. Der Proxy kann dann Ausgaben pro Entwickler zuordnen. Erfordert ein Gateway, das Claude Code v2.1.233 oder später ausführt.

Zum Beispiel für einen Proxy unter `upstream-gateway.internal.example.com`:

```yaml theme={null}
upstreams:
  - provider: anthropic
    base_url: https://upstream-gateway.internal.example.com
    auth:
      api_key: ${PROXY_KEY}
    forward_user_identity: true        # default false
```

Das Gateway fügt diese Header zu jeder Anfrage hinzu, die es an diesen Upstream weiterleitet.

| Header                        | Wert                                                                |
| ----------------------------- | ------------------------------------------------------------------- |
| `x-litellm-end-user-id`       | Die E-Mail des Entwicklers, wenn der IdP eine lieferte.             |
| `x-claude-gateway-user-id`    | Das IdP-Subjekt des Entwicklers, aus dem `sub`-Anspruch des Tokens. |
| `x-claude-gateway-user-email` | Die E-Mail des Entwicklers, wenn der IdP eine lieferte.             |

Wenn das IdP-Token keine E-Mail trägt, sendet das Gateway nur `x-claude-gateway-user-id` und lässt die zwei E-Mail-Header weg. Wenn Ihr IdP die E-Mail in einem anderen Anspruch ablegt, setzen Sie [`oidc.email_claim`](#oidc) auf diesen Anspruch.

Wenn Ihr Proxy `429` auf eine Anfrage antwortet, die die E-Mail des Entwicklers trug, gibt das Gateway diese Antwort unverändert an den Entwickler zurück, anstatt zum nächsten Upstream fehlzuschlagen, daher hält das Budget oder die Ratenbegrenzung pro Benutzer Ihres Proxys. Die anderen Antworten des Proxys folgen den gewöhnlichen [Failover-Regeln](#upstreams). Wenn das IdP-Token eines Entwicklers keine E-Mail trägt, leitet das Gateway seine Anfragen ohne die E-Mail-Header weiter, daher zählt ein `429` auf eine dieser Anfragen als Upstream-Kapazität und schlägt fehl über. Vor v2.1.267 auf dem Gateway-Server schlug jedes `429` fehl über.

Setzen Sie `forward_user_identity` nur auf einem Upstream, dessen `base_url` ein Proxy ist, den Sie betreiben. Das Gateway sendet Entwickler-E-Mails an jeden Server, den diese `base_url` benennt. Wenn die `base_url` die Anthropic API ist, die Standard ist, weigert sich das Gateway zu starten.

<h4 id="amazon-bedrock">
  Amazon Bedrock
</h4>

Für die Client-seitige Amazon Bedrock-Bereitstellung, die das Gateway ersetzt oder frontet, siehe [Claude Code on Amazon Bedrock](/docs/de/amazon-bedrock). Der Gateway-seitige Upstream:

```yaml theme={null}
upstreams:
  - provider: bedrock
    region: us-east-1
    auth: {}                           # preferred: AWS default credential chain
    # OR explicit credentials:
    # auth:
    #   aws_access_key_id: ${AWS_AKID}
    #   aws_secret_access_key: ${AWS_SK}
    #   aws_session_token: ${AWS_ST}
    # OR a Bedrock API bearer token:
    # auth:
    #   aws_bearer_token: ${AWS_BEARER_TOKEN}
    # Override the bedrock-runtime endpoint for FIPS or VPC-endpoint deployments:
    # base_url: https://bedrock-runtime-fips.us-east-1.amazonaws.com
```

Ein leerer `auth`-Block verwendet die Standard-Berechtigungskette des AWS SDK: Umgebungsvariablen, `~/.aws/credentials`, ECS-Task-Rolle, EC2-Instanz-Metadaten oder IRSA auf EKS. Geben Sie in der Produktion dem Gateway-Pod eine IAM-Rolle, anstatt statische Schlüssel in ein Container-Image einzubetten.

Explizite Berechtigungsnachweise müssen vollständig sein: Das Gateway schlägt beim Start fehl, wenn `aws_access_key_id` und `aws_secret_access_key` nicht zusammen gesetzt sind, oder wenn `aws_session_token` ohne sie gesetzt ist. Vor v2.1.207 bestand ein partieller `auth:`-Block die Validierung.

| Setup              | Wie                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| ------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| IAM-Berechtigungen | Gewähren Sie dem Principal des Gateways `bedrock:InvokeModel` und `bedrock:InvokeModelWithResponseStream` auf sowohl den Inference-Profil-ARNs als auch den zugrunde liegenden Foundation-Modell-ARNs. Für den integrierten Katalog in US-Regionen: `arn:aws:bedrock:<region>:<account>:inference-profile/us.anthropic.*` und `arn:aws:bedrock:*::foundation-model/anthropic.*`. Gewähren Sie auch `bedrock:CountTokens` auf den Foundation-Modell-ARNs. Das Gateway verwendet es, kostenlos, um die Eingabe-Tokens einer Anfrage zu zählen, die der Client abgebrochen hat, daher bleiben [Ausgabenlimits](#admin) genau. Ohne es fällt das Gateway auf eine Ein-Token-Bedrock-Anfrage für diese Zählung zurück. |
| Modellzugriff      | Amazon Bedrock aktiviert Modellzugriff standardmäßig in kommerziellen Regionen. Das verbleibende Konto-Level-Gate ist Anthropics einmaliges Anwendungsformular: Wenn niemand in Ihrem AWS-Konto es eingereicht hat, öffnen Sie die Amazon Bedrock-Konsole, wählen Sie ein Anthropic-Modell aus dem Modellkatalog und füllen Sie das Formular aus. Siehe [Anwendungsdetails einreichen](/docs/de/amazon-bedrock#1-submit-use-case-details) für das AWS Organizations-Formular und die Berechtigungen, die der Einreicher benötigt.                                                                                                                                                                                      |
| EKS (IRSA)         | Erstellen Sie eine IAM-Rolle mit der obigen Richtlinie und einer Vertrauensrichtlinie für den OIDC-Provider Ihres Clusters, der auf das Service-Account des Gateways beschränkt ist. Kommentieren Sie das Service-Account mit `eks.amazonaws.com/role-arn: arn:aws:iam::<acct>:role/claude-gateway`. `auth: {}` nimmt es auf.                                                                                                                                                                                                                                                                                                                                                                                     |
| ECS / EC2          | Fügen Sie die IAM-Rolle an die Task-Definition oder das Instance-Profil an. `auth: {}` nimmt es auf.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| Überall sonst      | Übergeben Sie Berechtigungsnachweise über die Umgebungsvariablen `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` und `AWS_SESSION_TOKEN`, oder setzen Sie sie explizit in `auth:` mit `${VAR}`-Erweiterung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Region             | `region:` ist die API-Endpunkt-Region. Cross-Region-Inference-Profile leiten über die Geo (US, EU, APAC) weiter, unabhängig davon, welche Sie wählen. Für Nicht-US-Regionen oder bereitgestellte Durchsatz-ARNs fügen Sie einen [`models:`](#models)-Block mit den richtigen Pro-Upstream-IDs hinzu.                                                                                                                                                                                                                                                                                                                                                                                                              |

<h4 id="claude-platform-on-aws">
  Claude Platform on AWS
</h4>

Claude Platform on AWS bedient die First-Party-Anthropic-API auf AWS-Infrastruktur unter `aws-external-anthropic.<region>.api.aws`. Sie verwendet First-Party-Modell-IDs, beachtet `anthropic-beta`-Header wie gesendet und bedient `count_tokens`, daher gilt keine der Bedrock-spezifischen Übersetzung. Der `anthropicAws`-Provider erfordert Claude Code v2.1.198 oder später; frühere Gateway-Versionen lehnen ihn beim Start ab.

Für die Client-seitige Bereitstellung derselben Plattform siehe [Claude Code on Claude Platform on AWS](/docs/de/claude-platform-on-aws). Der Gateway-seitige Upstream:

```yaml theme={null}
upstreams:
  - provider: anthropicAws
    region: us-east-1
    workspace_id: wrkspc_...
    auth:
      api_key: ${ANTHROPIC_AWS_API_KEY}   # sent as x-api-key
    # OR SigV4 via the AWS default credential chain:
    # auth: {}
    # OR explicit SigV4 credentials:
    # auth:
    #   aws_access_key_id: ${AWS_ACCESS_KEY_ID}
    #   aws_secret_access_key: ${AWS_SECRET_ACCESS_KEY}
    # Override the derived endpoint:
    # base_url: https://aws-external-anthropic.us-east-1.api.aws
```

Die Plattform läuft in einem separaten AWS-Konto von Amazon Bedrock und signiert SigV4-Anfragen für seinen eigenen Service-Namen, `aws-external-anthropic`, daher autorisiert eine Bedrock-scoped IAM-Rolle es nicht. Ein API-Schlüssel in `auth.api_key` hat Vorrang, wenn SigV4-Berechtigungsnachweise auch gesetzt sind. Ein leerer `auth`-Block verwendet die Standard-Berechtigungskette des AWS SDK, die gleiche Kette, die der [Amazon Bedrock](#amazon-bedrock)-Upstream verwendet.

| Feld                                                    | Erforderlich | Beschreibung                                                                                                                                            |
| ------------------------------------------------------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `region`                                                | Ja           | AWS-Region, Kleinbuchstaben, Ziffern und Bindestriche. Das Gateway leitet den Endpunkt davon ab als `https://aws-external-anthropic.<region>.api.aws`.  |
| `workspace_id`                                          | Ja           | Wird als Header bei jeder Anfrage gesendet; die Plattform erfordert es                                                                                  |
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
    auth: {}                           # preferred: Application Default Credentials
    # OR a service account key file:
    # auth: { service_account_json: /secrets/sa.json }
    # Override the aiplatform endpoint for Private Service Connect:
    # base_url: https://us-east5-aiplatform.p.googleapis.com
```

Ein leerer `auth`-Block verwendet Application Default Credentials: `GOOGLE_APPLICATION_CREDENTIALS`, GCE-Metadaten oder GKE Workload Identity. Service-Account-JSON-Schlüsseldateien werden unterstützt, aber nicht empfohlen; verwenden Sie Workload Identity oder fügen Sie ein Service-Account an die GCE- oder Cloud Run-Instanz an.

Setzen Sie `region: global`, um den [globalen Endpunkt für Google Cloud's Agent Platform](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations) anstelle eines regionalen zu verwenden. Google leitet dann jede Anfrage an eine verfügbare Region weiter, daher verfolgen Sie keine Pro-Region-Modellverfügbarkeit. Das Setzen einer bestimmten Region heftet jede Anfrage daran.

| Setup                   | Wie                                                                                                                                                                                                                                             |
| ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| IAM-Berechtigungen      | Gewähren Sie dem Service-Account des Gateways `roles/aiplatform.user` auf dem Projekt, oder eine benutzerdefinierte Rolle mit `aiplatform.endpoints.predict`. Aktivieren Sie Google Cloud's Agent Platform API (`aiplatform.googleapis.com`).   |
| Modellzugriff           | Aktivieren Sie in Model Garden die Claude-Modelle für Ihr Projekt. Sie veröffentlichen zu bestimmten Regionen; überprüfen Sie die Modellkarte auf unterstützte Regionen.                                                                        |
| GKE (Workload Identity) | Binden Sie ein GCP-Service-Account an das Kubernetes-Service-Account des Gateways und kommentieren Sie das KSA mit `iam.gke.io/gcp-service-account: claude-gateway@<proj>.iam.gserviceaccount.com`. `auth: {}` nimmt es auf.                    |
| Cloud Run / GCE         | Setzen Sie das Service-Account des Service auf eines mit `roles/aiplatform.user`. `auth: {}` nimmt es auf.                                                                                                                                      |
| Überall sonst           | `auth: { service_account_json: /secrets/sa.json }`, der Pfad zu einer JSON-Schlüsseldatei, die als Geheimnis bereitgestellt wird. Das Feld nimmt einen Dateipfad, nicht den Schlüsselinhalt, daher ist keine `${file:…}`-Erweiterung beteiligt. |

<h4 id="microsoft-foundry">
  Microsoft Foundry
</h4>

Für die Client-seitige Microsoft Foundry-Bereitstellung siehe [Claude Code on Microsoft Foundry](/docs/de/microsoft-foundry). Der Gateway-seitige Upstream:

```yaml theme={null}
upstreams:
  - provider: foundry
    resource: example-foundry              # https://example-foundry.services.ai.azure.com
    auth: { use_azure_ad: true }        # preferred: DefaultAzureCredential / Managed Identity
    # OR an API key:
    # auth:
    #   api_key: ${FOUNDRY_API_KEY}
```

`use_azure_ad: true` löst durch `DefaultAzureCredential` auf: Managed Identity auf AKS, ACI oder App Service; die Azure CLI; oder Umgebungsberechtigungsnachweise. API-Schlüssel funktionieren, sind aber projektumfassend und rotieren nicht automatisch. Der Endpunkt von Microsoft Foundry wird von `resource:` abgeleitet; setzen Sie das optionale `base_url`, um es für souveräne Clouds wie Azure Government zu überschreiben.

| Setup                   | Wie                                                                                                                                                                                                                       |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| RBAC                    | Gewähren Sie der Identität des Gateways `Azure AI User` oder `Cognitive Services User` auf der Microsoft Foundry-Ressource                                                                                                |
| Bereitstellungen        | Microsoft Foundry verwendet von Administratoren gewählte Bereitstellungsnamen, nicht kanonische Modell-IDs. Fügen Sie einen [`models:`](#models)-Block hinzu, der jede kanonische ID Ihrem Bereitstellungsnamen zuordnet. |
| AKS (Workload Identity) | Verbinden Sie eine User-Assigned Managed Identity mit dem OIDC-Aussteller des Clusters und binden Sie sie an das Service-Account des Gateways. `use_azure_ad: true` nimmt es über `WorkloadIdentityCredential` auf.       |
| ACI / App Service       | Aktivieren Sie system-zugewiesene oder user-zugewiesene Managed Identity auf der Ressource. `use_azure_ad: true` nimmt es auf.                                                                                            |
| Überall sonst           | `auth: { api_key: "${FOUNDRY_API_KEY}" }`. Zitieren Sie `${…}` innerhalb von `{ }`.                                                                                                                                       |

<h4 id="static-headers-on-upstream-requests">
  Statische Header auf Upstream-Anfragen
</h4>

Um feste Header zu den Anfragen hinzuzufügen, die das Gateway an einen Upstream sendet, setzen Sie `headers:` auf diesem Upstream. Verwenden Sie es, wenn ein Proxy, den Sie vor dem Anbieter betreiben, Traffic nach einem Header leitet oder zuordnet.

`headers:` erfordert Claude Code v2.1.277 oder später auf dem Gateway-Server. Ein früheres Gateway weigert sich zu starten, wenn es den Schlüssel findet. Aktualisieren Sie jedes Replikat, bevor Sie den Schlüssel hinzufügen, und entfernen Sie den Schlüssel, bevor Sie zu einer früheren Version zurückrollen.

Die Header gehen an den Server, den `base_url` benennt, oder an den eigenen Endpunkt des Anbieters, wenn `base_url` nicht gesetzt ist. Der Anbieter erhält sie auch, sofern Ihr Proxy sie nicht entfernt.

Dieses Beispiel erreicht einen `provider: vertex`-Upstream durch einen Proxy unter `upstream-proxy.internal.example.com`. Es setzt den `x-source`-Header, den der Proxy liest, und sendet ein Token aus der `PROXY_TOKEN`-Umgebungsvariable als `x-proxy-token`:

```yaml theme={null}
upstreams:
  - provider: vertex
    region: us-east5
    project_id: example-prod
    base_url: https://upstream-proxy.internal.example.com
    auth: {}
    headers:
      x-source: claude-apps-gateway
      x-proxy-token: ${PROXY_TOKEN}
```

Werte sind druckbarer ASCII-Text ohne Leerzeichen an beiden Enden. Zitieren Sie eine Zahl, `true` oder `false`, damit YAML sie als Text liest.

Um ein Geheimnis aus der Konfigurationsdatei zu halten, verwenden Sie [Geheimnis-Erweiterung](#secret-expansion), um den Wert aus einer Umgebungsvariable mit `${VAR}` oder aus einer Datei mit `${file:/path}` zu laden. Ein `${VAR}`, das zu einem leeren Wert aufgelöst wird, stoppt das Gateway vom Start.

`headers:` funktioniert auf jedem Anbieter, und jeder Upstream sendet nur seine eigenen.

Nicht jede Anfrage, die das Gateway an einen Upstream sendet, trägt sie:

| Anfrage, die das Gateway an diesen Upstream sendet                                    | Trägt `headers:`                    |
| ------------------------------------------------------------------------------------- | ----------------------------------- |
| `/v1/messages`, Streaming oder nicht, und `/v1/messages/count_tokens`                 | Ja                                  |
| Eine Anfrage, die von einem anderen Upstream fehlgeschlagen ist                       | Ja, nur `headers:` dieses Upstreams |
| Amazon Bedrocks `CountTokens`-Aufruf für eine Anfrage, die der Client abgebrochen hat | Nein                                |
| Der Workload Identity Federation-Token-Austausch                                      | Nein                                |

Auf einem Amazon Bedrock oder Claude Platform on AWS-Upstream, der Anfragen mit AWS SigV4 signiert, sind diese Header Teil der Signatur, daher muss Ihr Proxy sie unverändert durchlassen.

Wenn Sie einen Namen verwenden, den das Gateway reserviert, weigert es sich zu starten, und der Startup-Fehler benennt den Header. Reservierte Namen umfassen:

* `authorization` und `x-api-key`
* `host`, `content-type` und `user-agent`
* Jeder Name, der mit `anthropic-`, `x-goog-`, `x-amz-` oder `x-amzn-` beginnt

<h4 id="multiple-upstreams">
  Mehrere Upstreams
</h4>

Der gleiche Provider kann mehr als einmal mit einem unterschiedlichen `name:` erscheinen. Dies deckt verschiedene Regionen, verschiedene Konten über verschiedene Berechtigungsketten, bereitgestellter Durchsatz versus On-Demand und Cross-Provider-Fallback ab.

Das Gateway versucht Upstreams in Reihenfolge. `5xx`, `429`, `401`, `403`, `404`, Timeouts und fehlender Endpunkt (`501`) schlagen fehl über; andere `4xx` nicht.

`429` ist Pro-Upstream-Kapazität, daher schlägt bereitgestellter Durchsatz (PT)-Erschöpfung zu On-Demand fehl über. Wenn Sie [`forward_user_identity: true`](#per-user-identity-headers-for-a-proxy-you-run) auf einem Upstream setzen, ist ein `429` auf eine Anfrage, die die E-Mail des Entwicklers trug, eine Pro-Benutzer-Ablehnung statt und schlägt nicht fehl über.

`404` ist Pro-Upstream-Modellverfügbarkeit, daher blockiert ein Upstream, der ein Modell nicht aktiviert hat, keinen späteren Upstream, der es bedient. Ein Upstream, der das angeforderte Modell nicht auflösen kann, wird ohne Netzwerk-Roundtrip übersprungen.

Jede Anfrage startet beim ersten Upstream. Eine Anfrage erreicht einen späteren Upstream nur, wenn jeder Upstream vor ihm fehlgeschlagen ist oder das angeforderte Modell nicht bedient.

Das Gateway führt keine Aufzeichnung fehlgeschlagener Upstreams, daher versucht jede Anfrage, die ihn erreicht, ihn immer noch und wartet, bis er fehlschlägt, bevor es weitergeht, während ein Upstream ausfällt.

Für einen Anthropic-API-Upstream begrenzt [`timeouts.upstream_ttfb_ms`](#http-tuning) das Warten auf einen ausgefallenen Upstream. Diese Einstellung gilt nicht für die anderen Anbieter, wo das Gateway bis zu eine Stunde wartet, bis ein Upstream anfängt zu antworten.

`404` ist Pro-Upstream-Modellverfügbarkeit, daher blockiert ein Upstream, der ein Modell nicht aktiviert hat, keinen späteren Upstream, der es bedient. Ein Upstream, der das angeforderte Modell nicht auflösen kann, wird ohne Netzwerk-Roundtrip übersprungen.

Dieses Beispiel leitet eine bereitgestellte Durchsatz-Amazon Bedrock-Zuteilung zuerst weiter, überläuft zu On-Demand und einem zweiten Konto und fällt zuletzt auf die Anthropic API zurück:

```yaml theme={null}
upstreams:
  # Primary: provisioned throughput in your home region.
  - name: bedrock-pt
    provider: bedrock
    region: us-east-1
    auth: {}
  # Overflow: on-demand cross-region.
  - name: bedrock-od
    provider: bedrock
    region: us-west-2
    auth: {}
  # Different account: a separate Bedrock allotment via assumed-role creds.
  - name: bedrock-acct2
    provider: bedrock
    region: us-east-1
    auth:
      aws_access_key_id: ${ACCT2_AKID}
      aws_secret_access_key: ${ACCT2_SK}
  # Last resort: direct Anthropic API.
  - name: anthropic-fallback
    provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}

# Per-upstream model IDs are keyed on the upstream's `name:`.
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      bedrock-pt: arn:aws:bedrock:us-east-1:111111111111:provisioned-model/abcdef
      bedrock-od: us.anthropic.claude-opus-4-8
      bedrock-acct2: us.anthropic.claude-opus-4-8
      anthropic-fallback: claude-opus-4-8
```

| Hebel                      | Wie                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Verschiedene Regionen      | Ein Amazon Bedrock-Upstream pro Region, jeder mit seiner eigenen `region:`. Mit [`auto_include_builtin_models: true`](#models) leiten die Cross-Region-Inference-Profile automatisch weiter; für Region-geheftete Bereitstellungen verwenden Sie einen `models:`-Block.                                                                                                                                                                                                                                                                                 |
| Verschiedene Konten        | Ein Amazon Bedrock-Upstream pro Konto, jeder mit seinen eigenen Berechtigungsnachweisen in `auth:`. Die Standard-Kette (`auth: {}`) verwendet die Identität des Pods; für ein zweites Konto setzen Sie explizite Berechtigungsnachweise oder ein Bearer-Token.                                                                                                                                                                                                                                                                                          |
| Bereitgestellter Durchsatz | Ordnen Sie das Modell der bereitgestellten Durchsatz-ARN in `models:` für den Namen dieses Upstreams zu. Andere Upstreams behalten die On-Demand-ID, daher wird PT-Kapazität vor dem Failover erschöpft.                                                                                                                                                                                                                                                                                                                                                |
| VPC / FIPS-Endpunkte       | Setzen Sie `base_url:` auf dem Upstream auf Ihre VPC-Endpunkt- oder FIPS-Endpunkt-URL                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Modell-scoped Routing      | Nur eine benutzerdefinierte Modell-`id`, eine, die kein integriertes Claude-Modell ist, überspringt die Upstreams, die in ihrer `upstream_model:`-Karte fehlen. Das Gateway versucht integrierte Modelle auf jedem Upstream in Reihenfolge und verwendet die Standard-ID des Anbieters, wo die Karte keinen Eintrag hat, daher ändert die Karte für integrierte Modelle, welche ID ein Upstream erhält, statt ob er versucht wird; ein Upstream, der die ID ablehnt, folgt den gleichen [Failover-Regeln](#upstreams) wie jeder andere Upstream-Fehler. |

Das Failover zwischen Cloud-Anbietern oder zur direkten Anthropic API ändert, welche Vereinbarung, Geographie und andere Bedingungen die Anfrage regeln.

Die CLI wendet die gleiche Feature-Gating auf Gateways an, unabhängig davon, welcher Upstream eine gegebene Anfrage bedient, daher sendet Failover kein Body-Feld, das ein Upstream ablehnen würde.

<h2 id="optional-sections">
  Optionale Abschnitte
</h2>

<h3 id="admin">
  `admin`
</h3>

Optional. Aktiviert `/v1/organizations/spend_limits`, das Anthropics öffentliche Admin-API widerspiegelt, und erzwingt Ausgabenlimits pro Entwickler auf `/v1/messages`. Siehe [Ausgabenlimits](/docs/de/claude-apps-gateway-spend-limits) für die Festlegung und Durchsetzung von Limits; dieser Abschnitt behandelt die `gateway.yaml`-Schlüssel, die die Funktion aktivieren und optimieren.

```yaml theme={null}
admin:
  # Benannte statische API-Schlüssel für die Admin-Endpunkte, gesendet als x-api-key.
  # Die ID wird im Audit-Log als admin-key:<id> angezeigt, sodass jeder Schlüssel
  # zuordenbar ist. Array für Rotation: neuen Schlüssel hinzufügen, Clients aktualisieren,
  # alten entfernen.
  write_keys:
    - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
    - { id: ci,        key: "${GATEWAY_ADMIN_WRITE_KEY_CI}" }
  read_keys:
    - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
  # IdP-Gruppen mit vollständigem Admin-Zugriff über das normale Gateway-JWT (kein API-Schlüssel).
  admin_groups: [platform-finops]
  blocked_message: request an increase at https://go.example.com/claude-limits
```

| Feld                      | Erforderlich | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| ------------------------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `write_keys`              | Nein         | Array von `{id, key}`. Ein `x-api-key`, das einem dieser Schlüssel entspricht, kann Ausgabenlimits auflisten, festlegen und löschen. Schlüsselwerte müssen mindestens 32 Zeichen lang sein; `id`s müssen über `read_keys` und `write_keys` hinweg eindeutig sein.                                                                                                                                                                                           |
| `read_keys`               | Nein         | Array von `{id, key}`. Schreibgeschützt: alle `GET`-Endpunkte, einschließlich Auflisten von Limits, Abrufen nach ID und Lesen von [`/effective`](/docs/de/claude-apps-gateway-spend-limits#%2Feffective) und [`/audit`](/docs/de/claude-apps-gateway-spend-limits#%2Faudit).                                                                                                                                                                                          |
| `admin_groups`            | Nein         | IdP-Gruppennamen. Ein Gateway-JWT, dessen `groups`-Anspruch einen dieser Namen enthält, hat vollständigen Admin-Zugriff, Lesen und Schreiben, und wird als `oidc:<sub>` geprüft. Verwenden Sie dies für menschliche Administratoren; verwenden Sie API-Schlüssel für Maschinen. Ein leerer Eintrag in dieser Liste stoppt das Gateway beim Start. Siehe [Matcher-Werte, die das Gateway beim Start stoppen](#matcher-values-that-stop-the-gateway-at-boot). |
| `blocked_message`         | Nein         | Wird wörtlich an den `429 billing_error` angehängt, den ein blockierter Entwickler sieht. Schreiben Sie die vollständige Anweisung, z. B. eine URL oder einen Slack-Kanal. Wenn nicht gesetzt, sendet das Gateway nur die Standardmeldung. Siehe [Wie die Durchsetzung funktioniert](/docs/de/claude-apps-gateway-spend-limits#how-enforcement-works).                                                                                                           |
| `audit_retention_days`    | Nein         | Standard `365`. Ältere `admin_audit`-Zeilen werden gelöscht.                                                                                                                                                                                                                                                                                                                                                                                                |
| `spend_retention_months`  | Nein         | Standard `13`. `spend`-Zählerzeilen, die älter als dieser Wert sind, werden gelöscht. Der Standard behält ein volles Jahr plus den aktuellen Teilmonat für Jahresvergleichsberichte.                                                                                                                                                                                                                                                                        |
| `identity_retention_days` | Nein         | Standard `90`. Last-Seen-TTL für `principal_emails`-Zeilen, die die E-Mail, den Anzeigenamen und die Gruppen jedes Entwicklers enthalten (PII). Absichtlich kürzer als die Ausgabenaufbewahrung, sodass eine bereitgestellte Identität ausfällt, während ihre anonymen Ausgabenzähler erhalten bleiben.                                                                                                                                                     |
| `group_limit_mode`        | Nein         | `min` (Standard) oder `max`. Wenn sich ein Entwickler in mehreren Gruppen mit Limits befindet, erzwingt `min` das restriktivste und `max` das am wenigsten restriktive. Wird sowohl von der Durchsetzung als auch von `/effective` verwendet.                                                                                                                                                                                                               |

<h3 id="enforcement">
  `enforcement`
</h3>

Der `enforcement`-Block steuert das Verhalten von Ausgabenlimit-Prüfungen, wenn der Store nicht verfügbar ist.

| Feld                   | Erforderlich | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| ---------------------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `fail_closed_on_error` | Nein         | Standard `false`. Die Ausgabendurchsetzung schlägt bei einem Postgres-Ausfall offen fehl, sodass die Inferenz aktiv bleibt. Setzen Sie auf `true`, um geschlossen fehlzuschlagen: Entwickler über dem Limit werden blockiert, aber auch alle anderen, wenn der Store nicht erreichbar ist. Erfordert einen [`admin:`](#admin)-Block: Die Ausgabendurchsetzung wird nur ausgeführt, wenn `admin` konfiguriert ist, und das Gateway weigert sich zu starten, wenn Sie dies auf `true` setzen, ohne einen. |

<h3 id="pricing">
  `pricing`
</h3>

Der `pricing`-Block teilt dem Ausgabenzähler mit, was statt des USD-Listenpreises berechnet werden soll, sodass Limits und [`/effective`](/docs/de/claude-apps-gateway-spend-limits#%2Feffective) Ihre vertraglich vereinbarten Sätze widerspiegeln. Beträge bleiben in USD und sind eine Schätzung, keine Rechnung. Zwei Voraussetzungen:

* Claude Code v2.1.227 oder später auf dem Gateway-Server. Frühere Versionen lehnen den unbekannten Schlüssel beim Start ab.
* Ein [`admin:`](#admin)-Block oder in v2.1.268 oder später ein [`managed:`](#managed)-Block mit mindestens einer Richtlinie. Das Gateway weigert sich zu starten, wenn `pricing` gesetzt ist und keiner der Blöcke vorhanden ist, da nichts es lesen würde.

```yaml theme={null}
pricing:
  multiplier: 0.85
  overrides:
    - upstream: bedrock-eu
      model: claude-sonnet-4-6
      input: 3.30
      output: 16.50
      cache_read: 0.33
      cache_write: 4.125
```

| Feld         | Erforderlich | Beschreibung                                                                                                                                                                                                                                                |
| ------------ | ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `multiplier` | Nein         | Standard `1`. Der Zähler multipliziert jeden gezählten Betrag mit diesem, ob listenpreisig oder überschrieben, sodass `0.85` 85 % des Preises berechnet. Muss größer als 0 und höchstens 10 sein, und ein Wert über 1 ist ein [Aufschlag](#mark-prices-up). |
| `overrides`  | Nein         | Zeilen von `{upstream, model, input, output, cache_read, cache_write}` in USD pro Million Token. Alle vier Sätze sind erforderlich. Jeder muss größer als 0 und höchstens 10.000 sein.                                                                      |

Wie der Zähler eine Überschreibungszeile abgleicht:

* Eine Zeile ersetzt den Listenpreis für Anfragen, die `upstream`, ein [`upstreams[].name`](#upstreams), für `model` bedient. Das schließt den höheren [Schnellmodus](/docs/de/fast-mode#understand-the-cost-tradeoff)-Satz ein, sodass Schnell- und Standardanfragen mit denselben vier Sätzen gezählt werden.
* Eine integrierte ID wie `claude-sonnet-4-6`, abgeglichen wie [`models[].id`](#models), deckt jede datierte Form, regionale Amazon-Bedrock-Form oder Google-Cloud-Agent-Plattformform ab, die der Zähler als dieses Modell bewertet. Jede andere Zeichenkette, z. B. ein Alias oder ein Inferenzprofil-ARN, gleicht die ID ab, die der Client gesendet hat, oder die Zeichenkette, die upstream gesendet wurde, Groß-/Kleinschreibung ignoriert.
* Wenn sich Zeilen überlappen, wählt der Zähler die spezifischste Zeile statt der ersten Zeile: eine Zeile, deren `model` die genaue Modellzeichenkette ist, die upstream gesendet wurde, dann eine Zeile, die die genaue ID abgleicht, die der Client gesendet hat, dann eine Zeile, die das integrierte Modell benennt.
* Ein unbekannter Upstream-Name schlägt beim Start fehl, ebenso wie zwei Zeilen für einen Upstream, die dasselbe Modell benennen, einschließlich zwei Schreibweisen eines integrierten Modells. Das Gateway warnt beim Start vor einer Zeile, die kein anforderbares Modell verwenden kann.
* Web-Such-Anfragen bleiben beim \$0,01-Listenpreis; der Multiplikator wird immer noch auf sie angewendet.

Für regionale Sätze geben Sie jeder Region seinen eigenen benannten Upstream und eine Zeile pro Upstream.

<h4 id="mark-prices-up">
  Preise erhöhen
</h4>

Mit v2.1.271 oder später auf dem Gateway-Server können Sie `multiplier` über 1, bis zu 10, setzen, um mehr als der Anbieter berechnet zu zählen, z. B. einen internen Verrechnungssatz. Dieses Beispiel zählt jede Anfrage mit 120 % des Preises:

```yaml theme={null}
pricing:
  multiplier: 1.2
```

Mit einem [`admin:`](#admin)-Block gilt der Aufschlag auch für Ausgabenlimits. Der Zähler zählt 120 % des Preises, sodass Entwickler ihre Limits schneller erreichen. Das Gateway protokolliert eine Warnung beim Start, die dies besagt.

Der Multiplikator ändert nicht, was der Upstream-Anbieter für die Anfragen berechnet.

Wenn das Gateway auch [die Sätze an angemeldete Clients sendet](#send-the-rates-to-signed-in-clients), benötigen Entwickler Claude Code v2.1.271 oder später, um den Aufschlag zu sehen. Frühere Clients ignorieren einen `multiplier` über 1 und zeigen Kosten ohne ihn.

Ein Gateway-Server früher als v2.1.271 weigert sich zu starten, wenn Sie einen `multiplier` über 1 setzen.

<h4 id="send-the-rates-to-signed-in-clients">
  Sätze an angemeldete Clients senden
</h4>

Mit v2.1.268 oder später auf dem Gateway-Server fügt das Gateway die Sätze aus `pricing` auch in die [`managed`](#managed)-Richtlinien ein, die es bedient, als die [`modelPricing`](/docs/de/settings-reference#modelpricing)-verwaltete Einstellung. Entwickler, die von einer Richtlinie abgeglichen werden, sehen dann die `pricing`-Sätze für den ersten Upstream, der jede Modell-ID in `/usage`, der Statuszeile und OpenTelemetry bedient. Ein Entwickler, der keine Richtlinie abgleicht, erhält keine verwalteten Einstellungen, sodass seine Zahlen beim Listenpreis bleiben. Clients wenden die Einstellung in Claude Code v2.1.242 oder später an.

* Was das Gateway hinzufügt: Sofern der `cli`-Block einer Richtlinie nicht bereits `modelPricing` setzt, fügt das Gateway den `multiplier` und für jede Modell-ID, die ein Client anfordern kann, die Überschreibungszeile des ersten Upstream hinzu, der diese ID bedient. Ein Satz, den nur ein Failover-Upstream berechnet, bleibt beim Gateway.
* Eine Richtlinie ausschließen: Setzen Sie `modelPricing` auf `{}` im `cli`-Block dieser Richtlinie, und ihre Entwickler bleiben beim Listenpreis.
* Sätze einer Richtlinie behalten: Eine Richtlinie, deren `cli`-Block `modelPricing` mit ihrem eigenen `multiplier` oder `overrides` setzt, behält dieses `modelPricing` ganz, und das Gateway fügt keine eigenen Sätze hinzu.

<h3 id="models">
  `models`
</h3>

Der `models`-Block ist eine optionale von Administratoren kuratierte Modellliste, die unter `/v1/models` bedient und verwendet wird, um Modell-IDs pro Upstream zu übersetzen. Sie ist erforderlich für Nicht-US-Amazon-Bedrock-Regionen, Amazon-Bedrock-Provisioned-Throughput-ARNs und Microsoft-Foundry-Bereitstellungsnamen.

```yaml theme={null}
auto_include_builtin_models: true   # false: expose only the list below
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    # description: optional text shown in clients that surface it
    upstream_model:
      anthropic: claude-opus-4-8
      bedrock: us.anthropic.claude-opus-4-8   # or an inference-profile ARN
      foundry: your-opus-deployment-name
```

Jeder Schlüssel unter `upstream_model` muss dem `name` eines konfigurierten Upstream entsprechen, der standardmäßig auf den Anbieternamen gesetzt ist. Ein Schlüssel, der keinem Upstream entspricht, schlägt beim Start fehl, daher lassen Sie die Zeilen für Anbieter weg, die Sie nicht verwenden.

<h3 id="managed">
  `managed`
</h3>

Der `managed`-Block definiert rollenbasierte Zugriffrichtlinien, die nach IdP-Gruppen oder E-Mail-Domäne verschlüsselt sind. Richtlinien werden in Reihenfolge ausgewertet; die erste Übereinstimmung wird ausgewählt und dann mit der unten beschriebenen `match: {}`-Catch-All-Basis zusammengeführt. Sie werden pro Benutzer unter `GET /managed/settings` mit ETag/304-Caching bedient.

```yaml theme={null}
managed:
  policies:
    # Specific groups first.
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
        permissions: { deny: ["WebFetch", "WebSearch"] }
    # Default catch-all last: matches everyone who authenticated.
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
```

Eine `match: {}`-Catch-All, üblicherweise zuletzt aufgelistet, wird als Basisschicht behandelt. Jede andere Richtlinie erbt jeden Schlüssel, den sie nicht setzt, von der Catch-All, sodass Pro-Rollen-Einträge nur auflisten müssen, was sich vom Organisationsstandardwert unterscheidet. Die Zusammenführungsregeln hängen vom Schlüsseltyp ab:

* **Zulassungslisten**: `availableModels` und `permissions.allow`. Die Liste einer bestimmten Richtlinie ersetzt die Basis vollständig.
* **Ablehnungslisten und Hook-Arrays**: `permissions.deny`, `permissions.ask`, `disabledMcpjsonServers`, `deniedMcpServers`, `blockedMarketplaces` und jedes `hooks`-Event-Typ-Array. Diese nehmen die Vereinigung von Basis und Richtlinie, sodass ein organisationsweiter Ablehnungs- oder Audit-Hook nicht versehentlich durch eine Pro-Rollen-Überschreibung gelöscht werden kann.
* **Datensatz-typisierte Schlüssel**: `env`, `modelOverrides` und `skillOverrides`. Diese werden flach zusammengeführt, sodass ein Pro-Rollen-`env`-Block Schlüssel überschreibt, die er setzt, und den Rest von der Basis erbt.

`availableModels` wird auch serverseitig unter `/v1/messages` erzwungen, sodass ein abgelehntes Modell `400` zurückgibt, unabhängig davon, was der Client sendet.

Das Gateway validiert den `model`-Wert selbst, bevor es eine Anfrage weitergeleitet, sodass ein fehlerhafter Wert niemals einen Upstream erreicht. Es lehnt die Anfrage in zwei Fällen mit `400` ab:

* Wenn der Wert fehlt oder leer ist, lehnt das Gateway die Anfrage mit der Meldung `model is required` ab. Diese Prüfung erfordert ein Gateway, das Claude Code v2.1.228 oder später ausführt.
* Wenn der Wert vorhanden ist, aber keine Zeichenkette ist, lehnt das Gateway die Anfrage mit der Meldung `model must be a string` ab. Erfordert ein Gateway, das Claude Code v2.1.221 oder später ausführt.

| Matcher                                             | Verhalten                                                                                                                                                                           |
| --------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `match: {}`                                         | Gleicht jeden authentifizierten Benutzer ab. Beginnen Sie mit einem davon und fügen Sie später gruppenbezogene Richtlinien darüber hinzu.                                           |
| `match: { groups: [a, b] }`                         | Gleicht ab, wenn der `groups`-Anspruch des JWT eine der aufgelisteten Gruppen enthält. Groß-/Kleinschreibung beachtet: Gruppen müssen der genauen Schreibweise des IdP entsprechen. |
| `match: { email_domain: example.com }`              | Gleicht den Teil nach dem letzten `@` im `email`-Anspruch des JWT ab, Groß-/Kleinschreibung ignoriert. Akzeptiert eine Domäne pro Richtlinie.                                       |
| `match: { groups: [a], email_domain: example.com }` | Beide Bedingungen müssen übereinstimmen                                                                                                                                             |

Ein authentifizierter Benutzer, der keine Richtlinie abgleicht, erhält die Standardwerte des Gateways, was bedeutet, jedes Modell im Katalog und keine verwalteten Einstellungen. Fügen Sie eine `match: {}`-Catch-All zuletzt hinzu, wenn Sie eine garantierte Standardrichtlinie möchten.

<Note>
  Das Gateway führt kein eigenes Benutzerverzeichnis. Es autorisiert jede Anfrage vom IdP-Token des Benutzers, liest die Gruppenmitgliedschaft aus dem `groups`-Anspruch des Tokens und wertet Richtlinien dagegen aus. Es gibt kein Verzeichnis zum Aufzählen und keine Konten zum Vorab-Erstellen, und daher keinen SCIM-Endpunkt, da es nichts gibt, das SCIM synchronisieren könnte.

  Führen Sie Benutzer- und Gruppenzyklus-Management an der Quelle der Wahrheit durch, die das native SCIM-Provisioning Ihres IdP oder eine dedizierte Identitäts-Governance-Plattform ist. Mitgliedschaft und Deprovisioning, die dort gesteuert werden, fließen automatisch durch den Token in das Gateway. Wenn Sie SCIM-Provisioning von Claude-Konten selbst möchten, ist das eine [Claude for Enterprise](/docs/de/admin-setup)-Fähigkeit.

  Zwei Ausbreitungsuhren gelten:

  * **Richtlinieninhalt**: Das Bearbeiten einer Richtlinie und das erneute Bereitstellen erreichen verbundene Clients bei ihrer nächsten verwalteten Einstellungsabfrage, innerhalb einer Stunde, abgesehen von den [Änderungen, die nur beim nächsten Start gelten](/docs/de/server-managed-settings#fetch-and-caching-behavior)
  * **Gruppenmitgliedschaft**: Das Ändern der Gruppenmitgliedschaft eines Benutzers ändert, welche Richtlinie ihn abgleicht. Dies tritt beim nächsten Sitzungs-Neuausgabe in Kraft, was die nächste stille Aktualisierung bedeutet, begrenzt durch `session.ttl_hours`.
</Note>

<h4 id="matcher-values-that-stop-the-gateway-at-boot">
  Matcher-Werte, die das Gateway beim Start stoppen
</h4>

Beim Start prüft das Gateway den `match`-Block jeder Richtlinie und die [`admin_groups`](#admin)-Liste. Jeder dieser Werte stoppt das Gateway mit einem Fehler, der das Feld benennt:

* Eine leere `groups`-Liste
* Ein leerer Eintrag in `groups` oder in `admin_groups`
* Eine leere `email_domain`
* Eine `email_domain`, die `@`, Leerzeichen oder ein Komma enthält. Das Gateway trimmt den Wert und entfernt ein führendes `@`, bevor diese Prüfung durchgeführt wird. Schreiben Sie eine bloße Domäne, z. B. `example.com`.

Vor v2.1.232 startete das Gateway mit diesen Werten. Jeder Wert hatte diese Auswirkung:

* Eine leere `email_domain`: Das Gateway übersprung die Domänenprüfung, sodass eine Richtlinie mit einer leeren `email_domain` und keiner `groups`-Liste jeden authentifizierten Benutzer abglich
* Eine leere `groups`-Liste: Die Richtlinie gleichte niemanden ab
* Eine `email_domain`, die `@`, Leerzeichen oder ein Komma enthält: Die Richtlinie gleichte niemanden ab
* Ein leerer Eintrag in `groups` oder in `admin_groups`: Der Eintrag gleichte einen Benutzer nur ab, wenn der `groups`-Anspruch des IdP dieses Benutzers auch einen leeren Eintrag enthielt. In `admin_groups` gewährte diese Übereinstimmung Admin-Zugriff. Wenn Ihre `admin_groups`-Liste niemals einen leeren Eintrag enthielt, erhielt niemand auf diese Weise Admin-Zugriff.

<h4 id="what-goes-in-cli">
  Was in `cli` geht
</h4>

Jeder `cli`-Wert ist ein vollständiges Claude-Code-`managed-settings.json`-Dokument, das gleiche Schema, das Sie über MDM oder `/etc/claude-code/managed-settings.json` bereitstellen würden, hier als YAML ausgedrückt. Die CLI wendet das bereitgestellte Dokument auf der verwalteten Ebene an, über Benutzer- und Projekteinstellungen, anstelle von serverseitig verwalteten Einstellungen. Sie ignoriert daher die Einstellungen, die [auf OS-Ebenen-Richtlinienquellen beschränkt sind](/docs/de/server-managed-settings#current-limitations), wie `policyHelper` und `wslInheritsWindowsSettings`.

Das Gateway validiert jedes Dokument beim Start gegen das Einstellungsschema der CLI, sodass ein nicht erkannter Top-Level-Schlüssel beim Start mit einem Fehler fehlschlägt, der jeden fehlerhaften Schlüssel benennt. Absichtlich offene Teile des Schemas akzeptieren immer noch beliebige Werte, da neuere Clients Einträge erkennen können, die das Schema des Gateways nicht erkennt. Diese offenen Schlüssel umfassen `env`, `pluginConfigs` und Schlüssel, die unter `permissions` verschachtelt sind.

Da die Validierung das Schema verwendet, das mit der installierten Version des Gateways gebündelt ist, erfordert das Einfügen eines Top-Level-Einstellungsschlüssels, der von einer neueren Claude-Code-Version eingeführt wurde, in die verwaltete Konfiguration, das Gateway zuerst zu aktualisieren. Testen Sie eine neue Richtlinie auf einem Client, bevor Sie sie ausrollen.

Die vollständige Schlüsselreferenz befindet sich in [Claude-Code-Einstellungen](/docs/de/settings-reference#all-settings). Die Schlüssel, die Operatoren zuerst erreichen:

```yaml theme={null}
managed:
  policies:
    - match: {}
      cli:
        # Model access (also enforced server-side at /v1/messages)
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]

        # Permission policy
        permissions:
          deny:
            - "WebFetch"
            - "Read(./.env)"
            - "Read(./secrets/**)"
          disableBypassPermissionsMode: disable   # blocks --dangerously-skip-permissions
        allowManagedPermissionRulesOnly: true     # ignore user/project permission rules

        # Environment pushed into the CLI process. DISABLE_UPDATES blocks
        # background and manual updates; DISABLE_AUTOUPDATER stops only
        # background updates.
        env:
          DISABLE_UPDATES: "1"                    # pin versions via your own distribution

        # Org-wide hooks. Hook commands run on developer machines, not the
        # gateway, so the path must exist on every client OS in the policy.
        hooks:
          PostToolUse:
            - matcher: "Edit|Write"
              hooks:
                - { type: command, command: /usr/local/bin/audit-edit.sh }
```

| Schlüssel                                  | Erzwungen von | Auswirkung                                                                                                                                                                                                                                                                                                                                                                                                 |
| ------------------------------------------ | ------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `availableModels`                          | Gateway + CLI | Modell-Zulassungsliste. Auch unter `/v1/messages` geprüft, sodass ein gepatchter Client sie nicht umgehen kann.                                                                                                                                                                                                                                                                                            |
| `permissions.allow` / `.deny`              | CLI           | Tool- und Befehlsregeln. Siehe [Berechtigungen](/docs/de/permissions).                                                                                                                                                                                                                                                                                                                                          |
| `permissions.disableBypassPermissionsMode` | CLI           | Setzen Sie auf `disable`, um [`bypassPermissions`](/docs/de/permission-modes#skip-all-checks-with-bypasspermissions-mode), den Modus, der Berechtigungsaufforderungen überspringt, und das Flag `--dangerously-skip-permissions` zu blockieren                                                                                                                                                                  |
| `allowManagedPermissionRulesOnly`          | CLI           | Wenn `true`, werden verwaltete Einstellungen zur einzigen Einstellungsquelle von Berechtigungsregeln. Der Eintrag [`allowManagedPermissionRulesOnly`](/docs/de/settings-reference#allowmanagedpermissionrulesonly) listet jede Quelle auf, die Claude Code dann ignoriert.                                                                                                                                      |
| `env`                                      | CLI           | Umgebungsvariablen, die in den CLI-Prozess zusammengeführt werden. Verwenden Sie für Telemetrie, Auto-Update und Modellnamen-Überschreibungen.                                                                                                                                                                                                                                                             |
| `hooks`                                    | CLI           | Organisationsweite [Hooks](/docs/de/hooks)                                                                                                                                                                                                                                                                                                                                                                      |
| `managedMcpServers`                        | CLI           | Remote-MCP-Server, [die jedem abgleichenden Entwickler bereitgestellt werden](/docs/de/managed-mcp#provide-servers-through-managed-settings) neben den Servern, die sie selbst hinzufügen, nur `http` und `sse`. Siehe [MCP-Server in einer Richtlinie](#mcp-servers-in-a-policy). Erfordert Claude Code v2.1.259 oder später auf dem Gateway-Server und auf Clients. Frühere Clients ignorieren den Schlüssel. |

Da diese Einstellungen über das Netzwerk ankommen, zeigt die CLI jedem Entwickler einen Sicherheitsgenehmigungsdialog, bevor die unten aufgelisteten Einstellungen angewendet werden:

* `hooks`
* `env`-Variablen, die die Genehmigung des Entwicklers erfordern, wie Proxy- und Basis-URL-Variablen
* Shell-Ausführungseinstellungen wie `apiKeyHelper` und `statusLine`
* die Sandbox-Binäreinstellungen `sandbox.bwrapPath`, `sandbox.socatPath` und `sandbox.ripgrep`
* Sandbox-Einstellungen, die Datenverkehr abfangen, Anmeldedaten injizieren oder die Isolation schwächen, wie `sandbox.network.tlsTerminate` und die Proxy-Port-Einstellungen. [Sicherheitsgenehmigungsdialoge](/docs/de/server-managed-settings#security-approval-dialogs) listet sie alle auf.

[Genehmigungsspeicher](/docs/de/server-managed-settings#approval-memory) behandelt, wie lange eine Genehmigung dauert und wann der Dialog erneut angezeigt wird.

Claude Code wendet einige bereitgestellte `env`-Variablen an, ohne dem Entwickler den Genehmigungsdialog zu zeigen, wie Modellauswahleinstellungen und numerische Limits. Andere bereitgestellte Variablen können die Genehmigung des Entwicklers erfordern, bevor sie wirksam werden; ein nicht leerer Proxy-, Basis-URL- oder `OTEL_EXPORTER_OTLP_ENDPOINT`-Wert tut dies immer. Wenn eine bereitgestellte Variable Genehmigung benötigt, benennt der Dialog sie.

[Umgebungsvariablen und der Genehmigungsdialog](/docs/de/server-managed-settings#environment-variables-and-the-approval-dialog) hat die Details, einschließlich vier Datenschutz-Umschalter, deren bereitgestellter Wert entscheidet, ob sie Genehmigung benötigen. Vor v2.1.218 wendete Claude Code weniger Variablen an, ohne den Entwickler zu fragen, sodass mehr bereitgestellte Variablen den Dialog auslösten.

Die [Telemetrie](#telemetry)-Konfiguration des Gateways drückt `OTEL_EXPORTER_OTLP_ENDPOINT`, sodass das Setzen von `telemetry.forward_to` den Dialog auf jedem interaktiven Client auslöst. Der Dialog schützt die Maschine des Entwicklers vor einem kompromittierten oder feindseligem Gateway, nicht die Organisation vor dem Entwickler.

Ein nicht-interaktiver Lauf mit dem Flag `-p` kann den Dialog nicht anzeigen. Er wendet die gepushten Einstellungen nur für diesen Lauf an und speichert sie nicht als genehmigt, sodass die nächste interaktive Sitzung des Entwicklers immer noch den Dialog für sie anzeigt. Vor v2.1.207 speicherte ein nicht-interaktiver Lauf die Einstellungen als genehmigt und keine spätere interaktive Sitzung zeigte den Dialog dafür.

Wenn ein Entwickler ablehnt, beendet Claude Code diese Sitzung, anstatt die Richtlinie anzuwenden. Wenn Sie einen neuen Hook oder eine beliebige Env-Variable, die den Dialog auslöst, an eine breite Richtlinie pushen, zeigt Claude Code daher den Dialog jedem abgleichenden Entwickler. Es zeigt den Dialog in einer laufenden Sitzung bei der nächsten stündlichen Abfrage und ansonsten beim nächsten Start des Entwicklers.

Der `cli`-Schlüssel hieß in früheren Versionen `settings`. Diese Schreibweise wird immer noch als Alias akzeptiert, aber neue Bereitstellungen sollten `cli` verwenden.

<h4 id="mcp-servers-in-a-policy">
  MCP-Server in einer Richtlinie
</h4>

Um MCP-Server für die Claude-Code-Clients bereitzustellen, die eine Richtlinie abgleicht, setzen Sie [`managedMcpServers`](/docs/de/managed-mcp#provide-servers-through-managed-settings) im `cli`-Block dieser Richtlinie. Sie benötigen Claude Code v2.1.259 oder später auf dem Gateway-Server und auf Clients.

Das Gateway prüft jeden Eintrag beim Start mit [den gleichen Regeln, die Claude Code auf dem Client anwendet](/docs/de/managed-mcp#what-an-entry-can-contain), und wenn ein Eintrag eine Prüfung nicht besteht, weigert sich das Gateway zu starten und benennt den Eintrag.

Wenn Sie eine `${VAR}`-Referenz in `gateway.yaml` schreiben, löst das Gateway sie beim Start aus seiner Umgebung durch [Geheimnis-Erweiterung](#secret-expansion) auf, bevor es die Eintragsprüfungen ausführt, sodass jeder abgleichende Client den Literalwert erhält und ihn lesen kann. Die [Header-Anleitung für bereitgestellte Server](/docs/de/managed-mcp#provide-servers-through-managed-settings) gilt für den erweiterten Wert.

Das Gateway lehnt die `.mcp.json`-Schreibweise `mcpServers` in einem `cli`-Block ab, und sein Boot-Fehler benennt `managedMcpServers` als den zu verwendenden Schlüssel. Vor v2.1.259 lehnte das Gateway jede MCP-Server-Definition in einem `cli`-Block ab.

<h4 id="claude-desktop-overlay">
  Claude Desktop-Überlagerung
</h4>

Wenn Ihre Organisation auch [Claude Desktop](/docs/de/desktop) bereitstellt, bedient das gleiche Gateway beide Clients. Zeigen Sie `bootstrapUrl` in Claude Desktops [verwalteter Konfiguration](https://claude.com/docs/third-party/claude-desktop/configuration) auf `<listen.public_url>/user/bootstrap`. Claude Desktop leitet den OAuth-Aussteller von dieser URL ab, führt die gleiche Geräte-Code-Anmeldung gegen dieses Gateway durch und ruft seine Konfiguration aus der Antwort ab.

<Note>
  Erfordert Claude Code v2.1.203 oder später auf dem Gateway-Server und ein explizites Opt-In: `/user/bootstrap` gibt 404 zurück, es sei denn, die Richtlinie, die den Benutzer abgleicht, trägt einen `desktop`-Schlüssel. Ein leerer `desktop: {}` meldet eine Richtlinie an, und ein `desktop`-Schlüssel auf der `match: {}`-Basisschicht meldet jede Richtlinie an, die ihn erbt. Das Audit-Log zeichnet jede Anfrage als `desktop_bootstrap.serve` oder `desktop_bootstrap.denied` auf.
</Note>

Das Gateway leitet einen Großteil der Antwort von der abgleichenden Richtlinie des `cli`-Blocks und von der Top-Level-Gateway-Konfiguration ab:

* Die Modellliste aus `availableModels`
* Deaktivierte Tools aus Bare-Tool-Namen-`permissions.deny`-Einträgen. Wenn Sie `disabledBuiltinTools` im `desktop`-Block der Richtlinie setzen, bedient das Gateway die Vereinigung Ihres Wertes und der abgeleiteten Liste, sodass Sie auf diese Weise mehr Tools deaktivieren können, aber eines, das Sie durch `permissions.deny` deaktiviert haben, nicht erneut aktivieren können
* Die Egress-Zulassungsliste aus `sandbox.network.allowedDomains`. Wenn Sie `coworkEgressAllowedHosts` im `desktop`-Block der Richtlinie setzen, verwendet das Gateway diesen Wert statt der abgeleiteten Liste
* Ein OTLP-Endpunkt, der auf das Gateway selbst zeigt, und die Identitätsattribute des angemeldeten Benutzers. Das Gateway leitet die Exporte, die es an diesem Endpunkt erhält, an Ihre `forward_to`-Ziele weiter. Es enthält den Endpunkt und die Attribute, wenn Sie sowohl [`telemetry.forward_to`](#telemetry) als auch `listen.public_url` setzen.

  Claude Desktop exportiert jedes Signal mit einer Kodierung: `http/protobuf` oder `http/json`, wenn Sie `OTEL_EXPORTER_OTLP_PROTOCOL` oder eine seiner Pro-Signal-Varianten auf `http/json` im `env` der Richtlinie setzen. Vor Claude Code v2.1.261 auf dem Gateway-Server setzte die Antwort `http/json` unabhängig, sodass ein Collector, der nur Protobuf akzeptiert, Claude Desktops Exporte ablehnte

Um `disabledBuiltinTools`, `coworkEgressAllowedHosts` oder Claude Desktops eigene `managedMcpServers`-Einstellung im `desktop`-Block einer Richtlinie zu setzen, benötigen Sie Claude Code v2.1.232 oder später auf dem Gateway-Server. Claude Desktops `managedMcpServers` nimmt einen Array-Wert statt eines Objekts.

Das Gateway lässt Schlüssel ohne Claude-Desktop-Äquivalent weg, wie `hooks` und scoped-Berechtigungsregeln wie `Bash(npm *)`, aus der Bootstrap-Antwort.

Fügen Sie den optionalen `desktop`-Block neben `cli` hinzu, um Claude-Desktop-Einstellungen direkt zu setzen. Schreiben Sie Einstellungen aus Claude Desktops [verwalteter Konfigurationsreferenz](https://claude.com/docs/third-party/claude-desktop/configuration) als flache Schlüsselnamen. Lassen Sie Schlüssel weg, die Claude Desktop nur aus MDM oder lokalen Dateien liest, wie `bootstrapUrl`; das Gateway lehnt sie beim Start ab. Vor v2.1.232 akzeptierte das Gateway eine feste Liste von 11 Feature-Gate-Schlüsseln, wie `chatTabEnabled` und `disableAutoUpdates`, und lehnte jeden anderen Schlüssel beim Start ab. Vor v2.1.227 lehnte das Gateway auch `chatTabEnabled` und `chatAdvancedFileAnalysisEnabled` beim Start ab.

```yaml theme={null}
managed:
  policies:
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
      desktop:
        isLocalDevMcpEnabled: false
        disableAutoUpdates: true
        banner: { text: "Contractor build: internal use only" }
```

Jeder Schlüssel ist optional; Claude Desktop wendet seinen eigenen Standard für jeden Schlüssel an, den Sie weglassen. Das Gateway validiert jeden `desktop`-Block beim Start gegen das Konfigurationsschema, das Claude Desktop selbst verwendet, sodass ein Fehler beim Gateway-Start als Fehler auftaucht, der den Schlüssel benennt, statt jeden verbundenen Desktop zu erreichen. Das Gateway schlägt beim Start fehl, wenn ein Block Folgendes enthält:

* Ein unbekannter Schlüssel
* Ein erkannter Schlüssel, dessen Wert Claude Desktop ablehnen oder stillschweigend löschen würde, wie ein leerer Wert oder ein falsch geschriebener Unterschlüssel in einem verschachtelten Eintrag. Vor v2.1.260 ließ das Gateway ein falsch geschriebenes Feld in einem verschachtelten Objekt eines `managedMcpServers`- oder `orgPluginSettings`-Eintrags stillschweigend fallen, anstatt beim Start fehlzuschlagen.
* Ein Schlüssel, den das Gateway selbst berechnet: die Inferenzverbindung, die Modellliste und das OTLP-Relay. Konfigurieren Sie diese durch [`upstreams`](#upstreams), [`models`](#models) und den [`telemetry`](#telemetry)-Abschnitt `forward_to`.
* Ein Legacy-Alias eines aktuellen Schlüssels. Im Boot-Fehler benennt das Gateway den kanonischen Schlüssel zum Schreiben.

Wenn Sie einen veralteten Wert oder eine Eintragform verwenden, wie einen `managedMcpServers`-Eintrag ohne `transport`, startet das Gateway und protokolliert eine Warnung, die den Ersatz benennt.

Das Gateway validiert einen `desktop`-Block gegen das Schema, das mit seiner installierten Version gebündelt ist, wie es den `cli`-Block tut. Um eine Einstellung bereitzustellen, die von einer neueren Claude-Desktop-Version eingeführt wurde, aktualisieren Sie das Gateway zuerst. Zum Beispiel benötigen `userPluginMarketplacesEnabled` und `userPluginUploadsEnabled` Claude Code v2.1.260 oder später auf dem Gateway-Server und Claude Desktop 1.37937.0 oder später auf den Maschinen der Mitglieder.

Wenn Sie `orgPluginSettings` im `desktop`-Block einer Richtlinie setzen, bedient das Gateway es in der Array-Form, die Claude Desktop 1.15200.0 und später liest. Ältere Desktops ignorieren das Array und erzwingen keine Plugin-Tool-Richtlinie, daher aktualisieren Sie Mitglieder auf 1.15200.0 oder später, bevor Sie sich darauf verlassen.

Das Gateway füllt Schlüssel, die der `desktop`-Block einer Richtlinie nicht setzt, aus dem `match: {}`-Catch-All-`desktop`-Block, auf die gleiche Weise, wie es den `cli`-Block einer Richtlinie ausfüllt. Wenn Sie `disabledBuiltinTools` oder `builtinToolPolicy` sowohl in der Basis als auch in einer Rollen-Richtlinie setzen, behält das Gateway die Einschränkung der Basis:

* `disabledBuiltinTools`: Das Gateway verwendet die Vereinigung der Liste der Basis und der Richtlinie
* `builtinToolPolicy`: Wenn Sie ein Tool in der Basis auf einen anderen Wert als `allow` setzen, behält das Gateway diesen Wert, auch wenn Sie `allow` für das gleiche Tool in einer Rollen-Richtlinie setzen

Für jeden anderen Schlüssel, wenn Sie ihn in der Rollen-Richtlinie setzen, verwendet das Gateway den Wert der Rollen-Richtlinie. Das Gateway ersetzt ein Array oder ein verschachteltes Objekt wie `banner` ganz, sodass wenn Sie `banner.text` in einer Rollen-Richtlinie setzen, das Gateway die `banner.backgroundColor` der Basis löscht.

Wenn Sie Claude Desktop nicht bereitstellen, lassen Sie `desktop` vollständig aus Ihren Richtlinien weg; das Gateway gibt dann 404 von `/user/bootstrap` für jeden Benutzer zurück.

<h4 id="precedence-with-other-managed-sources">
  Vorrang mit anderen verwalteten Quellen
</h4>

Wenn ein Gerät auch eine MDM-bereitgestellte Richtlinie oder eine lokale `managed-settings.json` hat, rangieren Gateway-bereitgestellte Einstellungen zuerst. [Vorrang innerhalb der verwalteten Ebene](/docs/de/managed-settings#precedence-within-the-managed-tier) auf der Seite der verwalteten Einstellungen sagt, wann die lokalen Quellen gelten, und hat die [Schlüssel, die Claude Code aus jeder Admin-Quelle liest](/docs/de/managed-settings#keys-read-from-every-admin-source) unabhängig davon, welche Quelle es ausgewählt hat, wie die Sandbox-Lock-Schlüssel, `forceRemoteSettingsRefresh` und die Pro-Variable `env`-Zusammenführung. Ein [`policyHelper`](/docs/de/settings-reference#policyhelper), der in einem MDM-Profil oder der Datei der verwalteten Einstellungen konfiguriert ist, wird nur ausgeführt, wenn das Gateway keine Einstellungen bereitstellt; der Eintrag sagt, was seine Ausgabe ersetzt.

Einbettungs-Hosts wie [Claude Desktop](/docs/de/desktop) können Richtlinien durch die SDK-Option `managedSettings` bereitstellen. [Übergeordnete Einstellungen von Einbettungs-Hosts](/docs/de/managed-settings#parent-settings-from-embedding-hosts) sagt, wann Claude Code sie anwendet, und [Übergeordnete Einstellungen einschränken](/docs/de/claude-apps-gateway#restrict-parent-settings) listet auf, welche Zulassungs-Richtungs-Einstellungen immer noch ohne die `allowManaged*Only`-Sperren gelten.

Gateway-Richtlinien gelten für jeden Claude-Code-Aufruf auf der Maschine, einschließlich nicht-interaktiver `claude -p`-Läufe und Sitzungen, die vom Agent SDK erzeugt werden. Wenn das Gateway beim Start nicht erreichbar ist, beenden sich angemeldete Sitzungen mit einem Fehler, anstatt ohne ihre Richtlinie zu laufen.

<h3 id="telemetry">
  `telemetry`
</h3>

Die CLI sendet Metriken, Protokolle und, wenn aktiviert, Traces an das Gateway, das sie wörtlich an jedes konfigurierte Ziel weitergeleitet. Die Exporte verwenden OpenTelemetry Protocol (OTLP) über HTTP. Um das Relay zu überspringen und Sitzungen direkt an Ihren Collector exportieren zu lassen, [benennen Sie den Collector in einer Richtlinie](#export-directly-to-your-collector). Siehe [Überwachung der Nutzung](/docs/de/monitoring-usage) für die Metriken und Ereignisse, die die CLI ausgibt.

Die CLI stempelt jeden Export mit der Identität des authentifizierten Benutzers, gelesen aus dem vom Gateway ausgegebenen JWT: die Attribute `user.id`, `user.email` und `user.groups`. Die Kostenattribution pro Entwickler und Nutzungsattribution funktioniert daher ohne Konfiguration auf der Entwicklerseite.

[Claude Desktop](#claude-desktop-overlay) und Cowork-Sitzungen, die sich durch das Gateway anmelden, stempeln ihre Telemetrie mit `user.email` und `user.groups` neben `enduser.id`, sodass Sie Terminal-, Desktop- und Cowork-Nutzung mit einer Abfrage auf `user.email` oder `user.groups` abdecken können. `user.groups` ist die kommagetrennte IdP-Gruppenliste.

Desktop und Cowork-Telemetrie tragen auch `enduser.sub`, den `sub`-Anspruch, den Ihr Identitätsanbieter für den Benutzer ausstellt, der gleich bleibt, wenn sich die E-Mail eines Benutzers ändert. Terminal-Sitzungen stempeln den gleichen Wert unter `user.id`, sodass eine Abfrage, die `enduser.sub` gegen Terminal-`user.id` abgleicht, die Terminal-, Desktop- und Cowork-Nutzung eines Benutzers zusammen abdeckt. Bei Desktop- und Cowork-Exporten ist `user.id` ein anonymer Bezeichner, nicht der Betreff.

Wie alle OpenTelemetry-Daten von Claude Code gehen diese Attribute nur an Ziele, die Ihre Organisation konfiguriert, niemals an Anthropic.

Wenn die Gruppenliste eines Benutzers länger als 255 Zeichen ist, sobald sie prozentual kodiert ist, oder ein Gruppenname ein Komma oder Gleichheitszeichen enthält, lässt das Gateway `user.groups` aus der Desktop- und Cowork-Telemetrie dieses Benutzers weg, anstatt es zu kürzen. Die Terminal-Sitzungen dieses Benutzers tragen immer noch die vollständige Liste.

Das Gateway lässt `enduser.sub` weg, wenn der Betreff länger als 255 Zeichen ist, sobald er prozentual kodiert ist, oder ein Leerzeichen, ein Zeichen außerhalb des druckbaren ASCII oder eines von `,` `;` `=` `\` `"` `%` enthält. Die Desktop- und Cowork-Telemetrie dieses Benutzers behält seine anderen Attribute.

Sie benötigen Claude Code v2.1.265 oder später auf dem Gateway-Server für `user.email` und `user.groups` auf Desktop- und Cowork-Telemetrie, und Claude Desktop 1.24012 oder später auf der Maschine jedes Entwicklers für `user.groups`.

Sie benötigen Claude Code v2.1.274 oder später auf dem Gateway-Server für `enduser.sub`.

```yaml theme={null}
telemetry:
  forward_to:
    - url: https://otel-collector.internal.example.com
      headers:
        Authorization: ${OTLP_TOKEN}
      # Per-signal opt-in. Default: metrics only.
      metrics: true
      logs: false
      traces: false
    - url: https://api.datadoghq.com/api/v2/otlp
      headers:
        DD-API-KEY: ${DD_API_KEY}
```

<Warning>
  Jedes Ziel meldet sich unabhängig für `metrics`, `logs` und `traces` an, und der Standard ist nur Metriken. Die Signale unterscheiden sich in der Empfindlichkeit:

  * **Metriken**: Aggregatzähler wie Token-Zählungen, Anfragezählungen und Latenz
  * **Protokolle und Traces**: können vollständige Bash-Befehle, Tool-Eingaben und Dateipfade tragen, die alles abdecken, was Claude Code auf der Maschine eines Entwicklers tut

  Aktivieren Sie Protokolle und Traces nur auf Zielen mit den Zugriffskontrolle und Aufbewahrungsrichtlinie, die diese Daten rechtfertigen.
</Warning>

Jede `forward_to`-URL muss `https://` verwenden, mit einer Ausnahme für einen Collector auf der Loopback-Schnittstelle des Gateways selbst:

* `http://localhost:<port>` besteht die Konfigurationsvalidierung, aber der [SSRF-Guard](/docs/de/claude-apps-gateway-deploy#threat-model-summary) blockiert jeden Export mit `ECONNREFUSED_SSRF`, es sei denn, Sie setzen `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1` in der Umgebung des Gateways
* `http://127.0.0.1:<port>` oder `http://[::1]:<port>` schlägt beim Start fehl, es sei denn, diese Variable ist gesetzt

Für einen In-Cluster-Collector stellen Sie ihn über HTTPS unter seiner eigenen internen Adresse bereit, oder führen Sie ihn als Sidecar mit der gesetzten Variable aus.

Wenn `HTTPS_PROXY` gesetzt ist, sendet das Gateway Exporte durch diesen Proxy.

Um einen internen Collector direkt zu erreichen, fügen Sie ihn zu `NO_PROXY` nach Hostname oder nach einer Domäne mit einem führenden Punkt wie `.internal.example.com` hinzu, was Claude Code v2.1.277 oder später auf dem Gateway-Server erfordert. Stellen Sie sicher, dass das Gateway den Collector ohne den Proxy erreichen kann. Ein Eintrag ohne einen führenden Punkt gleicht nur diesen genauen Namen ab, nicht Namen darunter. CIDR-Bereiche gleichen nicht ab.

Mit [Proxy-Only-Egress](#proxy-only-egress) aktiviert, erlauben Sie den Collector stattdessen im Proxy, da jeder `NO_PROXY`-Eintrag Proxy-Only-Egress ausschaltet.

Telemetrie ist in der CLI standardmäßig deaktiviert. Wenn Sie sowohl `telemetry.forward_to` als auch `listen.public_url` setzen, schaltet das Gateway sie für verbundene Clients ein, indem es sechs Umgebungsvariablen durch `/managed/settings` drückt:

* `CLAUDE_CODE_ENABLE_TELEMETRY=1`
* `OTEL_METRICS_EXPORTER`, `OTEL_LOGS_EXPORTER` und `OTEL_TRACES_EXPORTER`, jeweils auf `otlp` gesetzt, wenn mindestens ein `forward_to`-Ziel dieses Signal aktiviert, und auf `none` andernfalls
* `OTEL_EXPORTER_OTLP_ENDPOINT=<public_url>`
* `OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf`

Vor Claude Code v2.1.265 auf dem Gateway-Server drückte das Gateway alle drei Exporter-Selektoren als `otlp`, einschließlich für Signale, die kein Ziel aktiviert hat.

Der gepushte Endpunkt wird aus der öffentlichen URL erstellt, sodass Metriken und Protokolle keine OTEL-Konfiguration von Entwicklern oder Richtlinien benötigen.

Entwickler, die sich durch `/login` anmelden, können Exporte nicht mit ihrer eigenen OTEL-Konfiguration umleiten:

* **Lokal gesetzte Variablen**: Claude Code wendet die gepushten Variablen auf der verwalteten Ebene an, sodass jede den Wert überschreibt, den ein Entwickler lokal dafür setzt.
* **Lokal konfigurierte Endpunkte**: Mit OTLP/HTTP-Export aktiviert ignoriert die CLI jeden lokal konfigurierten Endpunkt, unabhängig davon, ob das Gateway die Telemetrie-Variablen gepusht hat. Seine Exporte gehen an das Gateway, es sei denn, eine Richtlinie [benennt Ihren Collector als Endpunkt](#export-directly-to-your-collector).

Ohne ein `forward_to`-Ziel für ein Signal akzeptiert das Gateway es und verwirft es. Wenn Entwickler bereits Claude-Code-Telemetrie an einen Ihrer Collector exportieren, fügen Sie ihn als `forward_to`-Ziel hinzu, mit Protokollen oder Traces aktiviert, wenn sie diese exportieren, sodass er ihre Daten weiterhin erhält, nachdem sie sich anmelden. Um das Relay stattdessen zu überspringen, [benennen Sie den Collector in einer Richtlinie](#export-directly-to-your-collector).

[Traces](/docs/de/monitoring-usage#traces-beta) erfordern auch `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` auf jedem Client. Setzen Sie es im `env`-Block einer verwalteten Richtlinie, da das Gateway es nicht drückt. Entwickler genehmigen es im gleichen [Sicherheitsgenehmigungsdialog](#managed), den der gepushte Endpunkt bereits auslöst.

Setzen Sie es auf `1` nur in den Richtlinien, deren Gruppen Sie verfolgen möchten. Eine Richtlinie, die es nicht setzt, erbt den Wert von Ihrer `match: {}`-Catch-All-Richtlinie, wenn diese Richtlinie einen setzt, pro den [Zusammenführungsregeln](#managed). Um zu verhindern, dass die Clients einer Gruppe Traces senden, auch wenn ein Entwickler die Variable lokal setzt, setzen Sie sie auf `0` in der Richtlinie dieser Gruppe.

Sowohl Protobuf- als auch JSON-OTLP-Kodierungen werden weitergeleitet, und jedes OpenTelemetry-kompatible Backend funktioniert als Ziel.

<h4 id="export-directly-to-your-collector">
  Direkt an Ihren Collector exportieren
</h4>

Um Sitzungen, die sich durch `/login` anmelden, Telemetrie direkt an Ihren Collector senden zu lassen, anstatt durch das Relay, setzen Sie `OTEL_EXPORTER_OTLP_ENDPOINT` auf die `https://`-Basis-URL des Collectors im `env`-Block einer [verwalteten Richtlinie](#managed). Claude Code hängt `/v1/metrics`, `/v1/logs` oder `/v1/traces` an die URL an, die Sie setzen, wie `https://otel-collector.example.com:4318`, und exportiert jedes Signal dort über OTLP/HTTP. Erfordert Claude Code v2.1.265 oder später auf der Maschine jedes Entwicklers. Frühere Clients exportieren durch das Relay.

Um sich beim Collector zu authentifizieren, setzen Sie `OTEL_EXPORTER_OTLP_HEADERS` im gleichen `env`-Block. Sitzungen senden niemals das Gateway-Sitzungstoken des Entwicklers an einen Collector, der auf diese Weise benannt wird.

Wenn Sie diesen Endpunkt in einer Richtlinie hinzufügen oder ändern, fragt Claude Code jeden Entwickler, ihn im [Sicherheitsgenehmigungsdialog](#managed) zu genehmigen, bevor er ihn in einer interaktiven Sitzung anwendet.

Claude Code prüft den Endpunkt, bevor es ein Signal direkt exportiert, und behält dieses Signal auf dem Relay, wenn eine Prüfung fehlschlägt. Die Prüfungen umfassen:

* Der Endpunkt kommt vom Gateway selbst. Wenn Sie die gleiche Variable in einem MDM-Profil oder einer lokalen `managed-settings.json` setzen, bleiben Exporte auf dem Relay.
* Die URL verwendet `https://` oder `http://` zu einer Loopback-Adresse
* Die URL wird zu einem Pfad aufgelöst, der mit `/v1/<signal>` endet, ohne Abfrage oder Fragment. Claude Code erstellt diesen Pfad selbst aus der generischen Variable. Es verwendet eine Pro-Signal-Variable wie `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT` wie geschrieben, daher den vollständigen Pfad dort einschließen.
* Die URL ist nicht der eigene Host des Gateways. Ein Endpunkt, der auf das Gateway adressiert ist, behält den Relay-Pfad und sein Sitzungstoken.
* Weder Sie noch der Entwickler haben [`otelHeadersHelper`](/docs/de/settings-reference#otelheadershelper) in einer Einstellungsquelle konfiguriert. Mit einem konfigurierten Helper bleibt jedes Signal auf dem Relay.

Der Endpunkt, den Sie benennen, ändert nur, wohin Exporte gehen. Sie wählen immer noch, welche Signale überhaupt exportieren, mit den `OTEL_*_EXPORTER`-Selektoren.

Der Endpunkt allein schaltet Export nicht ein, daher setzen Sie auch die Variablen, die dies tun, es sei denn, das Gateway drückt sie bereits:

* Wenn das Gateway bereits [die Telemetrie-Variablen drückt](#telemetry), decken sie Aktivierung, Selektoren und Protokoll ab, und Ihr expliziter Endpunkt überschreibt den gepushten `<public_url>`-Wert. Setzen Sie einen `OTEL_*_EXPORTER`-Selektor auf `otlp` selbst nur für ein Signal, das kein `forward_to`-Ziel aktiviert.
* Wenn nicht, setzen Sie auch `CLAUDE_CODE_ENABLE_TELEMETRY=1`, die `OTEL_*_EXPORTER`-Selektoren und `OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf`.

Wenn sich der Entwickler abmeldet oder bei einem anderen Gateway anmeldet, stoppen Exporte an den Collector und Claude Code verwirft jeden verbleibenden Batch, anstatt ihn zu senden.

<h4 id="when-a-destination-fails">
  Wenn ein Ziel fehlschlägt
</h4>

Das Gateway puffert, wiederholt oder speichert Telemetrie nicht, daher verwirft es einen Export, der ein Ziel nicht erreicht, anstatt ihn verspätet zu liefern. Jedes Ziel erfolgreich oder schlägt fehl auf eigene Faust, und der exportierende Client erhält eine Erfolgsmeldung auf jeden Fall, sodass eine fehlgeschlagene Lieferung nur im Protokoll des Gateways angezeigt wird.

Nach fünf aufeinanderfolgenden fehlgeschlagenen Lieferungen an ein Ziel pausiert das Gateway die Weiterleitung dorthin in 30-Sekunden-Abständen, protokolliert jede Pause, bis eine Lieferung erfolgreich ist. Jede Fehlerantwort, Timeout oder Verbindungsfehler zählt als fehlgeschlagene Lieferung, außer `400`, `413`, `415`, `422` und `431`, die bedeuten, dass der Collector diese Export-Nutzlast als fehlerhaft oder zu groß ablehnt.

Eine abgelehnte Nutzlast weder voranschreitet noch setzt den Fehlerzähler zurück: Das Gateway leitet weiterhin an das Ziel weiter und protokolliert eine Warnung, die es und den Status benennt, bei der ersten Ablehnung des Ziels und alle hundert danach.

<h3 id="http-tuning">
  HTTP-Optimierung
</h3>

Vier optionale Top-Level-Blöcke, `access_control`, `limits`, `timeouts` und `rate_limits`, optimieren die HTTP-Oberfläche. Die Standards passen zu den meisten Bereitstellungen.

| Block            | Schlüssel                                      | Standard      | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ---------------- | ---------------------------------------------- | ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `access_control` | `allow_cidrs` / `deny_cidrs`                   | leer          | Eingehende IP-Zulassung/Ablehnung nach Client-Adresse, nach `trusted_proxies`-Auflösung. `deny_cidrs` wird zuerst geprüft; ein Client, den es abgleicht, wird abgelehnt, auch wenn `allow_cidrs` auch abgleicht. Wenn `allow_cidrs` nicht leer ist, ist das Gateway standardmäßig Ablehnung. `/healthz` und `/readyz` sind von `allow_cidrs` ausgenommen. Wenn ein vertrauenswürdiger Proxy einen `X-Forwarded-For`-Eintrag sendet, der keine IP-Adresse ist, ist der echte Client unbekannt und das Gateway protokolliert eine Warnung einmal, die benennt, was zu prüfen ist. Wo eine der Listen auf die Anfrage zutrifft, lehnt sie sie mit `403` und Audit-Grund `xff_unparseable` ab. Wo keine zutrifft, bedient es die Anfrage und verwendet die Adresse des Proxys selbst als Client-IP für Pro-IP-Ratenlimits und Audit. |
| `limits`         | `max_request_bytes`                            | 32 MiB        | Max eingehender Anfragekörper; übergroße Anfragen erhalten `413`, bevor der Körper gepuffert wird. Erhöhen Sie für große Datei- oder Bildanfragen.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `limits`         | `max_request_header_bytes`                     | nicht gesetzt | Wenn gesetzt, geben übergroße Header `431` zurück                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `limits`         | `max_url_length`                               | nicht gesetzt | Wenn gesetzt, gibt eine zu lange URL `414` zurück                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `timeouts`       | `upstream_ttfb_ms`                             | 120000        | Max Wartezeit auf die Antwortheader des Upstream (Zeit zum ersten Byte). Der Antwortkörper streamt dann ohne Wall-Clock-Obergrenze. Gilt für den direkten Anthropic-Upstream-Pfad; auf jedem anderen Anbieter wartet das Gateway bis zu eine Stunde auf den Antwortkopf.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `rate_limits`    | `device_authorization.max` / `.window_seconds` | 30 / 600      | Pro-IP-Ratenlimit auf dem nicht authentifizierten Geräteautorisierungs-Endpunkt. Erhöhen Sie für eine große Organisation hinter einer gemeinsamen Egress-IP oder NAT. [Große Rollouts](/docs/de/claude-apps-gateway-deploy#large-rollouts) zeigt, wie weit Sie es erhöhen sollten. Diese Limits gelten nur für den Geräte-Grant-Anmeldungsfluss, nicht für `/v1/messages`-Inferenz. Siehe [Benutzer-Code-Brute-Force-Widerstand](/docs/de/claude-apps-gateway-deploy#user-code-brute-force-resistance).                                                                                                                                                                                                                                                                                                                                    |
| `rate_limits`    | `device_verify.max` / `.window_seconds`        | 10 / 600      | Pro-IP-Ratenlimit bei `user_code`-Einreichungen unter `/device`. Es ist das, was jemanden daran hindert, den Code eines anderen Entwicklers zu erraten. [Große Rollouts](/docs/de/claude-apps-gateway-deploy#large-rollouts) zeigt, wie weit Sie es erhöhen sollten.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

Wenn Sie beide `access_control`-Listen leer lassen, was der Standard ist, bedient das Gateway jede Client-Adresse, sodass nur Ihr Netzwerk einschränkt, wer es erreichen kann. Das ist wichtig, da ein Gateway [verwaltete Einstellungen](#managed) pushen kann, die Befehle auf Entwicklermaschinen ausführen.

Während `allow_cidrs` leer ist, warnt das Gateway an zwei Stellen, ohne zu ändern, wie es auf eine Anfrage antwortet:

* **Beim Start**: eine Warnung im Betriebsprotokoll empfiehlt, nur die privaten Bereiche `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `100.64.0.0/10`, `127.0.0.0/8`, `::1/128` und `fc00::/7` zuzulassen, plus alle anderen internen Bereiche, von denen sich Ihre Entwickler verbinden. Wenn Sie das Gateway an eine Loopback-Adresse binden und weder `trusted_proxies` noch `public_url` setzen, wie in der lokalen Entwicklung, erscheint die Warnung nicht.
* **Zur Laufzeit**: Das erste Mal, wenn eine Anfrage von einer Adresse außerhalb dieser privaten Bereiche ankommt, protokolliert das Gateway eine Warnung und gibt ein [`access.public_client`-Audit-Ereignis](/docs/de/claude-apps-gateway-deploy#logs) aus, das die Client-IP trägt. Beide werden einmal pro Prozess ausgelöst. Link-lokale Adressen, `169.254.0.0/16` und `fe80::/10`, zählen nicht als öffentlich. Das Gateway antwortet auf `/healthz` und `/readyz`, bevor diese Prüfung ausgeführt wird, sodass Health-Probes aus öffentlichen Bereichen sie nicht auslösen.

Beide Signale verwenden die Client-Adresse, wie das Gateway sie auflöst. Wenn ein Load Balancer, Port-Forward oder Tunnel Datenverkehr weitergeleitet und nicht in `listen.trusted_proxies` aufgelistet ist, sieht das Gateway die Adresse des Relays, die normalerweise privat ist, sodass weder die Laufzeit-Warnung noch eine private Zulassungsliste Datenverkehr, der durch sie weitergeleitet wird, erfasst.

Hinter einem solchen Front-End setzen Sie zuerst [`listen.trusted_proxies`](#listen), damit das Gateway echte Client-Adressen sieht, und halten Sie das Gateway und alles davor unabhängig vom öffentlichen Internet unerreichbar.

<h3 id="load_test_mode">
  `load_test_mode`
</h3>

Der `load_test_mode`-Block ermöglicht es Ihnen, ein Gateway zu laden, ohne einen Modell-Anbieter aufzurufen. Während es aktiviert ist, erstellt das Gateway jede Anbieter-Anfrage wie gewohnt, verwirft sie statt sie zu senden, und streamt eine vorgefertigte Antwort durch seinen normalen Antwortpfad zurück. Die Antwort ist Fülltext, der mit einem Satz beginnt, der besagt, dass er vorgefertigt ist.

Erfordert v2.1.283 oder später. Frühere Versionen weigern sich zu starten, wenn der Schlüssel gesetzt ist, daher aktualisieren Sie jedes Replikat, bevor Sie den Block hinzufügen, und entfernen Sie ihn, bevor Sie zurückrollen.

Das Beispiel unten schaltet den Modus mit den Standardwerten ein, eine Antwort von 750 Ausgabe-Token, die über etwa 10 Sekunden gestreamt werden:

```yaml theme={null}
load_test_mode:
  enabled: true
  reply_tokens: 750     # roughly how many tokens of text each canned reply carries
  reply_seconds: 9.5    # how long a streamed reply takes
```

| Feld            | Erforderlich | Beschreibung                                                                                                                                                                                    |
| --------------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`       | Ja           | `true` schaltet den Modus ein. `false` behält Ihre Zahlen in der Datei mit dem Modus aus. Das Gateway weigert sich zu starten, wenn der Block vorhanden ist, ohne ihn.                          |
| `reply_tokens`  | Nein         | Standard `750`. Ungefähr wie viele Token Text jede vorgefertigte Antwort trägt, eine ganze Zahl von 1 bis 100.000.                                                                              |
| `reply_seconds` | Nein         | Standard `9.5`. Wie lange eine gestreamte Antwort dauert, von 0 bis 600. `0` sendet die ganze Antwort auf einmal. Eine Antwort auf eine nicht-gestreamte Anfrage kommt immer auf einmal zurück. |

Ein Lasttest in diesem Modus deckt das Gateway, Ihre Postgres und alles vor dem Gateway ab. Es deckt die Grenzen, Geschwindigkeit oder den Netzwerkpfad des Anbieters nicht ab.

Während der Modus aktiviert ist, kann eine Anfrage einen `x-load-test-user`-Header tragen, der eine ganze Zahl von bis zu sieben Ziffern hält, und das Gateway zählt jede Zahl als einen separaten Entwickler mit der E-Mail und den Gruppen des Entwicklers, dessen Token mit der Anfrage kam. Geben Sie der Load-Test-Bereitstellung ihre eigene leere Datenbank, da das Gateway sich weigert zu starten, wenn der Modus gegen eine Datenbank aktiviert ist, in der ein Entwickler bereits etwas ausgegeben hat.

<Warning>
  Schalten Sie dies niemals für ein Gateway ein, das Entwickler verwenden. Jede Anfrage erhält die vorgefertigte Antwort und kein Modell wird aufgerufen. Das Gateway protokolliert eine `load_test_mode is on`-Warnung beim Start und markiert jedes `inference`-[Audit-Ereignis](/docs/de/claude-apps-gateway-deploy#logs) mit `load_test: true`, während der Modus aktiviert ist.
</Warning>

<h2 id="complete-example">
  Vollständiges Beispiel
</h2>

Diese vollständige Referenzkonfiguration behandelt jeden Kernabschnitt; die [HTTP-Abstimmungsblöcke](#http-tuning) behalten ihre Standardwerte. Kopieren Sie sie, löschen Sie, was Sie nicht brauchen, und füllen Sie Ihre Werte aus. Die Konfiguration im [Schnellstart](/docs/de/claude-apps-gateway#quickstart) ist eine minimale Version davon.

```yaml gateway.yaml theme={null}
# Laufen mit:
#   claude gateway --config gateway.yaml
#
# Operatives Log-Verbosity wird durch die Umgebungsvariable CLAUDE_GATEWAY_LOG_LEVEL
# gesteuert (debug | info | warn | error; Standard info). debug
# protokolliert auch die Anspruchsnamen in jedem id_token zur Diagnose von groups_claim.
# Es beeinflusst nicht Audit-Ereignisse, die immer ausgegeben werden.

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
  # connect_timeout_seconds: 5

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

# Laden Sie diese Bereitstellung zu Testzwecken, ohne einen Modell-Provider aufzurufen. Niemals auf einem
# Gateway, das Entwickler verwenden: jede Anfrage erhält eine vorgefertigte Antwort.
# load_test_mode:
#   enabled: true
#   # reply_tokens: 750
#   # reply_seconds: 9.5

# Meter zu vertraglich vereinbarten Sätzen statt USD-Listenpreis. Erfordert admin: oder eine
# managed:-Richtlinie. Mit managed: gehen die gleichen Sätze auch an angemeldete Clients.
# Die folgenden Sätze sind Platzhalter, keine echten Vertragspreise.
# pricing:
#   multiplier: 0.85
#   overrides:
#     - { upstream: anthropic, model: claude-sonnet-4-6, input: 3.30, output: 16.50, cache_read: 0.33, cache_write: 4.125 }

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

Alles oben konfiguriert den Gateway-Server. Sie zeigen Entwicklermaschinen separat auf jedem Gerät auf das Gateway, durch Claude Code's [verwaltete Einstellungen](/docs/de/managed-settings). Das Gateway kann die Anmeldeschlüssel nicht selbst pushen, da sie dem Client sagen, wo sich das Gateway befindet.

Für die CLI setzen Sie diese Schlüssel in die Pro-Betriebssystem-Datei `managed-settings.json`. Die beiden Anmeldeschlüssel leiten die `/login` jedes Entwicklers zu Ihrem Gateway:

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com",
  "parentSettingsBehavior": "merge"
}
```

`parentSettingsBehavior: "merge"` behält Claude Desktop's Bereitstellung der Egress-Allowlist für seine eingebetteten Claude Code-Sitzungen bei; [Richtlinie für Claude Desktop-Sitzungen bereitstellen](/docs/de/claude-apps-gateway#deliver-policy-to-claude-desktop-sessions) erklärt den Mechanismus und wo sich die Opt-in befinden muss.

Stellen Sie die `managed-settings.json`-Datei auf jedem Gerät bereit, typischerweise über Ihre MDM-Plattform. Der Dateipfad unterscheidet sich je nach Plattform. Siehe [wo jeder Mechanismus die Richtlinie speichert](/docs/de/managed-settings#where-each-mechanism-stores-the-policy).

Standardmäßig ersetzt eine Registrierungsrichtlinie unter Windows oder ein verwaltetes Preferences-Plist unter macOS die `managed-settings.json`-Datei, anstatt sie damit zu zusammenzuführen, mit Ausnahme der [Ausnahmeschlüssel und quellenübergreifenden Überprüfungen oben](#precedence-with-other-managed-sources). Alle drei Schlüssel in diesem Snippet folgen der Regel mit der höchsten Prioritätsquelle, daher müssen Flotten, die Richtlinien über Gruppenrichtlinien oder Konfigurationsprofile bereitstellen, alle drei stattdessen in diesem Mechanismus platzieren.

Für Claude Desktop setzen Sie den `bootstrapUrl`-Schlüssel in Claude Desktop's eigene [verwaltete Konfiguration](https://claude.com/docs/third-party/claude-desktop/configuration) auf `<listen.public_url>/user/bootstrap`. Der Anmeldungsfluss und die Pro-Gruppen-Richtlinie entsprechen dann der CLI's, sobald eine Richtlinie sich serverseitig mit einem `desktop`-Schlüssel anmeldet; ohne die Anmeldung gibt `/user/bootstrap` 404 zurück. Siehe [Claude Desktop-Overlay](#claude-desktop-overlay) für die serverseitige Hälfte.

Claude Code ehrt [`forceLoginGatewayUrl`](/docs/de/settings-reference#forcelogingatewayurl), [`gatewayInternalNetworks`](/docs/de/settings-reference#gatewayinternalnetworks) und den `"gateway"`-Wert von [`forceLoginMethod`](/docs/de/settings-reference#forceloginmethod) nur von einer verwalteten Quelle auf dem Computer: `managed-settings.json`, das macOS-Plist oder die Windows HKLM-Registrierung, oder ein Richtlinien-Helper. Ein Entwickler, der sie in seiner eigenen `~/.claude/settings.json` setzt, hat keine Auswirkung, und das Setzen im Gateway-Payload auch nicht.

<h2 id="related">
  Verwandt
</h2>

* [Claude Apps Gateway-Übersicht](/docs/de/claude-apps-gateway): Schnellstart und Entwickler-Verbindung
* [Bereitstellungsleitfaden](/docs/de/claude-apps-gateway-deploy): IdP-Setup, Container-Image, Kubernetes und Cloud Run sowie Operationen
* [Ausgabenlimits](/docs/de/claude-apps-gateway-spend-limits): Pro-Entwickler-Caps und die Admin API
