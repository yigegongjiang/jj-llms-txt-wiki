> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ausgabenlimits für Claude-Apps-Gateway

> Begrenzen Sie die Ausgaben jedes Entwicklers über das Claude-Apps-Gateway pro Tag, Woche oder Monat. Legen Sie Limits mit einer Admin-API fest und das Gateway erzwingt sie live bei jeder Anfrage.

Ausgabenlimits begrenzen, wie viel jeder Entwickler über Ihr [Claude-Apps-Gateway](/docs/de/claude-apps-gateway) an einem bestimmten Tag, einer Woche oder einem Monat ausgeben kann. Wenn ein Entwickler sein Limit überschreitet, gibt das Gateway `429` bei seiner nächsten Anfrage zurück und blockiert ihn, bis sich der Zeitraum zurückgesetzt hat oder ein Administrator das Limit erhöht. Verwenden Sie Ausgabenlimits, um jedem Entwickler, einer Gruppe oder der gesamten Organisation eine Obergrenze für eine gemeinsam genutzte Anmeldedaten zu setzen.

Ein Claude-Apps-Gateway leitet alle Inferenzen über eine gemeinsame Upstream-Anmeldedaten weiter, sodass die Rechnung Ihres Anbieters alles dieser Anmeldedaten zuordnet, nicht einzelnen Entwicklern. Ohne Pro-Entwickler-Limits kann eine unkontrollierte Agent-Flotte die gesamte Verpflichtung der Organisation ausgeben. Ausgabenlimits sind die Pro-Entwickler-Ansicht des Gateways und ein Schutzschalter auf dieser gemeinsamen Rechnung.

<h2 id="set-a-cap">
  Limit festlegen
</h2>

Mit dem konfigurierten [`admin:`](/docs/de/claude-apps-gateway-config#admin)-Block in `gateway.yaml` stellt das Gateway eine Admin-API unter `/v1/organizations/spend_limits` bereit und erzwingt Limits live bei jeder Inferenzanfrage. Limits selbst werden über diese API festgelegt, nicht in `gateway.yaml`; jede `POST /v1/organizations/spend_limits`-Anfrage erstellt oder ersetzt ein Limit aus `{scope, amount, period}`. Die API spiegelt die Drahtformate der öffentlichen [Admin-API](https://platform.claude.com/docs/en/manage-claude/admin-api) von Anthropic für Ausgabenlimits wider, sodass ein HTTP-Client, der gegen diesen Vertrag geschrieben wurde, das Gateway ansteuern kann, indem er seine Basis-URL ändert.

Diese Anfrage legt einen organisationsweiten Standard von 500 USD pro Monat für jeden Entwickler fest:

```bash theme={null}
curl -sS https://claude-gateway.internal.example.com/v1/organizations/spend_limits \
  -H "x-api-key: $GATEWAY_ADMIN_WRITE_KEY" \
  -H "Content-Type: application/json" \
  -d '{"scope": {"type": "organization"}, "amount": "50000", "period": "monthly"}'
```

Diese Anfrage legt ein strengeres Limit von 100 USD pro Tag für jedes Mitglied der Gruppe `contractors` fest:

```bash theme={null}
curl -sS https://claude-gateway.internal.example.com/v1/organizations/spend_limits \
  -H "x-api-key: $GATEWAY_ADMIN_WRITE_KEY" \
  -H "Content-Type: application/json" \
  -d '{"scope": {"type": "rbac_group", "rbac_group_id": "contractors"}, "amount": "10000", "period": "daily"}'
```

| Feld         | Werte                                    | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ------------ | ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `scope.type` | `user`, `rbac_group`, `organization`     | `user` zielt auf einen Entwickler nach seiner OpenID Connect (OIDC) `sub`, der stabilen Benutzer-ID, die Ihr Identitätsanbieter zuweist; übergeben Sie sie als `scope.user_id`. `rbac_group` zielt auf eine [IdP-Gruppe](/docs/de/claude-apps-gateway-config#managed) nach Name; übergeben Sie sie als `scope.rbac_group_id`. `organization` ist der organisationsweite Standard. Das Gateway akzeptiert alle drei; Anthropics öffentliches `POST` ist heute nur für Benutzer. |
| `amount`     | Ganzzahl-String von USD-Cent oder `null` | `null` ist unbegrenzt. `"0"` ist ein Nulllimit, das jede Anfrage blockiert.                                                                                                                                                                                                                                                                                                                                                                                               |
| `period`     | `daily`, `weekly`, `monthly`             | Ein Bereich kann ein Limit pro Zeitraum enthalten, und jedes wird unabhängig erzwungen: Ein Entwickler wird blockiert, wenn er eines überschreitet.                                                                                                                                                                                                                                                                                                                       |

Ein Gruppen- oder Organisationslimit ist ein Pro-Sitz-Standard, den jedes Mitglied erbt, nicht ein gemeinsamer Pool. Pro Zeitraum wird das effektive Limit eines Entwicklers in dieser Reihenfolge aufgelöst: ein Pro-Benutzer-Override, dann das restriktivste seiner Gruppenlimits, dann der Organisationsstandard, dann unbegrenzt. [`admin.group_limit_mode: max`](/docs/de/claude-apps-gateway-config#admin) dreht das Multi-Gruppen-Tie-Break zu am wenigsten restriktiv um.

<h3 id="authenticate-to-the-admin-api">
  Authentifizierung bei der Admin-API
</h3>

Senden Sie eines der folgenden:

* Ein `x-api-key`-Header, der einem Schlüssel in [`admin.write_keys`](/docs/de/claude-apps-gateway-config#admin) für vollständigen Zugriff oder `admin.read_keys` für `GET`-only-Zugriff entspricht. Jeder Schlüssel trägt eine `id`, die im Audit-Log als `admin-key:<id>` angezeigt wird, also geben Sie Terraform, CI und jeder Automatisierung seinen eigenen.
* Ein Gateway-Bearer-Token, dessen `groups`-Anspruch eines der [`admin.admin_groups`](/docs/de/claude-apps-gateway-config#admin) enthält. Dies ist vollständiger Zugriff und wird als `oidc:<sub>` überwacht, also bevorzugen Sie es für menschliche Administratoren.

<h2 id="how-enforcement-works">
  Wie die Durchsetzung funktioniert
</h2>

Bei jeder `/v1/messages`-Anfrage löst das Gateway die Limits des Entwicklers und die Ausgaben bis zum aktuellen Zeitraum in einer Postgres-Abfrage auf. Wenn sie ein Limit überschreiten, gibt die Anfrage `429` mit `error.type: billing_error` und dem Header `x-should-retry: false` zurück. Die Nachricht ist `spend limit reached`, gefolgt von Ihrer [`admin.blocked_message`](/docs/de/claude-apps-gateway-config#admin), falls gesetzt.

`/v1/messages/count_tokens` ist ausgenommen. Token-Zählung ist kostenlos, daher wird sie unabhängig vom Limit-Status ausgeführt.

Nach jeder Antwort liest ein Nutzungsmesser Token-Zählungen aus der Antwort, während sie zum Client streamt, bewertet sie zum USD-Listenpreis und erhöht Postgres-Zähler für alle drei Zeitraum-Buckets. Der Messer ist ein einzelner Leser im Stream, daher werden die Bytes des Clients nicht berührt und ein Messfehler bricht die Antwort nicht.

Ausgabenlimits schätzen Ausgaben aus Token-Zählungen zum USD-Listenpreis; sie sind ein Schutzschalter, keine Rechnung. Für verbindliche Abrechnung gleichen Sie gegen die eigene Nutzungsberichterstattung Ihres Anbieters ab, wie die Anthropic Usage & Cost Admin API, Aufruflogs auf Amazon Bedrock oder Cloud Monitoring auf Google Cloud.

Die Preisgestaltung verwendet die gleiche Tabelle, die die Claude Code CLI für ihre eigene Kostenanzahl verwendet, mit der gleichen Modell-ID-Kanonisierung über Anthropic, Amazon Bedrock (`us.anthropic.…-v1:0`), Google Cloud's Agent Platform (`claude-…@date`) und Microsoft Foundry ID-Formen. Eine Modell-ID, die die Tabelle nicht platzieren kann, wie ein Microsoft Foundry-Bereitstellungsname oder ein Inferenz-Profil-ARN, wird zum Standard-Tier für unbekannte Modelle von \$5/\$25 pro Million Input-/Output-Token bewertet, anstatt null, sodass eine nicht erkannte ID ein Limit nicht durch Nicht-Messung umgehen kann. Das Gateway warnt beim Start und einmal pro ID zur Laufzeit, wenn ein Modell durch den Fallback bewertet wird.

Client-Abbrüche werden auch abgerechnet. Der Upstream meldet Output-Token nur im Terminal-Frame des Streams, daher trägt ein abgebrochener Stream sie nicht. Der Messer behält eine konservative Untergrenze aus der gestreamten Inhaltsgröße bei, etwa vier Zeichen pro Token, und rechnet sie ab, wenn und nur wenn der Terminal-Nutzungs-Frame fehlt. Ein vollständiger Stream rechnet immer die vom Upstream gemeldete Anzahl ab. Ohne dies könnte ein begrenzter Entwickler Output streamen und jede Anfrage unmittelbar vor dem Ende abbrechen, ohne jemals gezählt zu werden.

<h3 id="postgres-availability">
  Postgres-Verfügbarkeit
</h3>

Die Vorabprüfung fragt Postgres mit einem Zwei-Sekunden-Timeout ab. Wenn der Speicher nicht erreichbar ist oder das Timeout überschreitet, schlägt die Durchsetzung standardmäßig offen fehl: Die Anfrage wird fortgesetzt und das Gateway protokolliert eine Warnung. Setzen Sie [`enforcement.fail_closed_on_error: true`](/docs/de/claude-apps-gateway-config#enforcement), um stattdessen geschlossen fehlzuschlagen, was den gleichen `429 billing_error` mit der Nachricht `spend limit unavailable` zurückgibt. Fail-Open verhindert, dass ein Speicherausfall zu einem Inferenzausfall wird; Fail-Closed garantiert keine nicht gemessenen Ausgaben.

<h2 id="admin-api-reference">
  Admin-API-Referenz
</h2>

Die folgenden Endpunkte werden unter `/v1/organizations/spend_limits` bereitgestellt.

| Methode und Pfad                               | Beschreibung                                                                          |
| ---------------------------------------------- | ------------------------------------------------------------------------------------- |
| `GET /v1/organizations/spend_limits`           | Konfigurierte Limits auflisten. Abfrage: `?limit=&after_id=&before_id=`.              |
| `POST /v1/organizations/spend_limits`          | Erstellen oder ersetzen Sie ein Limit für `{scope, period}`.                          |
| `GET /v1/organizations/spend_limits/{id}`      | Rufen Sie ein Limit nach seiner `spl_`-präfixierten ID ab.                            |
| `DELETE /v1/organizations/spend_limits/{id}`   | Löschen Sie ein Limit. Gibt `{type: "spend_limit_deleted", id}` zurück.               |
| `GET /v1/organizations/spend_limits/effective` | Aufgelöstes Limit und Ausgaben bis zum aktuellen Zeitraum pro Principal pro Zeitraum. |
| `GET /v1/organizations/spend_limits/audit`     | Admin-Mutationsverlauf, neueste zuerst. Abfrage: `?limit=`.                           |

Konventionen spiegeln Anthropics Admin-API wider:

* Ein `type` auf jedem Objekt
* `spl_`-präfixierte IDs
* Beträge als Ganzzahl-Strings von USD-Cent; `POST` lehnt jede andere `currency` mit `400` ab
* Die `{type: "error", error: {type, message}, request_id}`-Fehler-Umhüllung
* Ein `request-id`-Response-Header auf jeder Admin-Antwort, Erfolg oder Fehler, der der `request_id` des Body entspricht

Jede Mutation schreibt eine Vor-/Nach-Zeile in `admin_audit` in der gleichen Transaktion, zugeordnet zu `admin-key:<id>` oder `oidc:<sub>`.

Das Gateway stellt die Spend-Limits-Endpunkte nur bereit. Andere Admin-API-Oberflächen, wie die `spend_limit_increase_requests`-Warteschlange, sind nicht Teil der Admin-API des Gateways.

<h3 id="/effective">
  `/effective`
</h3>

`GET /v1/organizations/spend_limits/effective` gibt Anthropics `SpendSummary`-Schema zurück: jede Zeile ist ein Principal für einen Zeitraum, mit dem aufgelösten Limit, den Ausgaben bis zum aktuellen Zeitraum und einem `actor`-Objekt. Gateway-spezifische Unterschiede:

* `user_id` ist die OIDC `sub`.
* `actor.name` und `actor.email_address` sind `null`, bis die erste Inferenzanfrage des Principal durch das Gateway erfolgt. Das Gateway hat kein Benutzerverzeichnis; es zeichnet zuletzt gesehene Werte aus dem Session-JWT jedes Benutzers auf.
* Jede Zeile trägt auch ein `groups`-Array, die zuletzt gesehenen IdP-Gruppen des Principal. Dies ist eine Gateway-Erweiterung, damit eine Admin-UI jeden anwendbaren Limit-Tier anzeigen kann; Anthropic-förmige Clients ignorieren es.
* Ohne einen `user_ids[]`-Filter listet es Principal mit aufgezeichneten Ausgaben auf, da das Gateway nicht alle Organisationsmitglieder aufzählen kann.

Gruppen-basierte Limits werden gegen diese zuletzt gesehenen Gruppen mit dem gleichen `group_limit_mode`-Tie-Break aufgelöst, den die Durchsetzung verwendet, sodass der Viewer das tatsächlich anwendbare Limit anzeigt.

| Abfrageparameter | Beschreibung                                                                                                                         |
| ---------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `user_ids[]`     | Wiederholbar. Filtern Sie nach spezifischen Principal nach OIDC `sub`.                                                               |
| `period[]`       | Wiederholbar. Filtern Sie nach `daily`, `weekly` oder `monthly`-Zeilen.                                                              |
| `sort`           | `spend_desc` listet Top-Spender zuerst auf. Erfordert genau einen `period[]`.                                                        |
| `q`              | Groß-/Kleinschreibung-unabhängiger Substring-Filter über die OIDC `sub`, zuletzt gesehene E-Mail und zuletzt gesehenen Anzeigenamen. |
| `limit` / `page` | Seitengröße (1–1000, Standard 20) und der undurchsichtige Cursor aus dem `next_page` der vorherigen Antwort.                         |

<Warning>
  `q=` und `user_ids[]=` fahren GET-Abfrage-Strings, daher erfasst jeder Fronting-Proxy oder Load-Balancer sie in seinen Zugriffslogs. Wenn Ihre PII-Log-Richtlinie streng ist, bereinigen Sie diese Parameter dort.
</Warning>

<h3 id="/audit">
  `/audit`
</h3>

Gibt den Ausgabenlimit-Mutationsverlauf zurück: wer welches Limit geändert hat, Vor-/Nach-Snapshots und den optionalen Grund, neueste zuerst. `has_more` ist exakt. Dieser Endpunkt folgt den lokalen Admin-API-Konventionen statt einer First-Party-Drahtform.

<h3 id="pagination">
  Pagination
</h3>

Die rohe Liste paginiert nach `after_id` und `before_id`, die sich gegenseitig ausschließende `spl_…`-IDs sind; Ergebnisse werden nach Erstellung sortiert und `has_more` spiegelt die Traversierungsrichtung wider. `/effective` paginiert nach dem undurchsichtigen `next_page`-Token, der als `?page=` zurückgegeben wird, mit Principal in aufsteigender Reihenfolge sortiert, sodass Seiten stabil bleiben, während Ausgaben aufgezeichnet werden. `limit` ist 1–1000, Standard 20, auf beiden.

<h2 id="data-lifecycle">
  Datenzyklus
</h2>

Das Gateway enthält vier ausgabenbezogene Tabellen; ein stündlicher Sweep erzwingt die Aufbewahrungsfenster:

| Tabelle            | Inhalt                                                                             | Aufbewahrung                                                                                                |
| ------------------ | ---------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `spend`            | Pro-Principal-Zeitraum-bis-Datum-Zähler in Cent                                    | [`admin.spend_retention_months`](/docs/de/claude-apps-gateway-config#admin), Standard 13                         |
| `spend_limits`     | Die konfigurierten Limits                                                          | Bis gelöscht über die API                                                                                   |
| `admin_audit`      | Der Mutationsverlauf                                                               | [`admin.audit_retention_days`](/docs/de/claude-apps-gateway-config#admin), Standard 365                          |
| `principal_emails` | Zuletzt gesehene E-Mail, Anzeigename und IdP-Gruppen jedes Principal. Enthält PII. | [`admin.identity_retention_days`](/docs/de/claude-apps-gateway-config#admin) seit letzter Aktivität, Standard 90 |

`identity_retention_days` ist absichtlich kürzer als `spend_retention_months`: Eine bereitgestellte Identität wird nicht mehr aktualisiert und veraltet, während ihre anonymen Ausgabenzähler für Jahr-über-Jahr-Berichte erhalten bleiben.

Wenn ein Entwickler geht, löschen Sie alle Pro-Benutzer-Limits über `DELETE /v1/organizations/spend_limits/{id}`; ihre Ausgaben und Identitätszeilen veralten auf den oben genannten Aufbewahrungsfenstern. Um eine Person sofort zu löschen, für Offboarding oder eine Datenschutzanfrage (DSAR), führen Sie `DELETE FROM principal_emails WHERE principal = '<sub>'` direkt gegen die Gateway-Datenbank aus. Das entfernt die einzige Tabelle, die ihre E-Mail, ihren Namen und ihre Gruppen enthält. Die `spend`- und `admin_audit`-Zeilen verweisen nur auf die pseudonyme OIDC `sub` und veralten auf ihren eigenen Fenstern.

<h2 id="related">
  Verwandt
</h2>

* [`admin`- und `enforcement`-Konfiguration](/docs/de/claude-apps-gateway-config#admin): Aktivierung der Admin-API und Abstimmung der Aufbewahrung
* [Bereitstellungsleitfaden](/docs/de/claude-apps-gateway-deploy#postgres): Postgres-Schema und Backup-Anleitung
