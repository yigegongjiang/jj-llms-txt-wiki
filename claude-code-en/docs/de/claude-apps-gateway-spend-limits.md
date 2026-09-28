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

Bei jeder `/v1/messages`-Anfrage sucht das Gateway die Limits des Entwicklers und die Ausgaben bis zum aktuellen Zeitraum in einer Postgres-Abfrage auf. Ein Entwickler, der ein Limit überschreitet, erhält eine `429` mit `error.type: billing_error` und dem Header `x-should-retry: false`.

Die Nachricht benennt den Zeitraum und die Rücksetzeit, z. B. `spend limit reached (daily; resets 2026-08-08 00:00 UTC)`, gefolgt von Ihrer [`admin.blocked_message`](/docs/de/claude-apps-gateway-config#admin), falls gesetzt. Wenn ein Entwickler mehrere Limits gleichzeitig überschreitet, benennt die Nachricht das Limit, das zuletzt zurückgesetzt wird. Die Antwort enthält auch einen `retry-after`-Header mit den Sekunden bis zu diesem Zurücksetzen. Vor v2.1.225 auf dem Gateway-Server war die Nachricht `spend limit reached` ohne Zeitraum, Rücksetzeit oder `retry-after`-Header.

Bei v2.1.227 oder später listet die Protokollreferenz unter `<public_url>/protocol` auch die genauen Response-Header für Nutzungslimits und den `429`-Body auf.

Limits werden an UTC-Kalendergrenzen zurückgesetzt: täglich um 00:00 UTC, wöchentlich am Montag und monatlich am ersten. Das Gateway blockiert niemals `/v1/messages/count_tokens`, da Token-Zählung kostenlos ist.

<h3 id="how-requests-are-priced">
  Wie Anfragen bepreist werden
</h3>

Nach jeder Antwort liest ein Nutzungsmesser die Token-Zählungen und addiert die Kosten zu den täglichen, wöchentlichen und monatlichen Zählern. Er berührt niemals die an den Client gesendeten Bytes, daher kann ein Messfehler eine Antwort nicht unterbrechen. Die Beträge sind USD-Schätzungen, ein Schutzschalter statt einer Rechnung; für die Abrechnung gleichen Sie gegen die Nutzungsberichterstattung Ihres Anbieters ab.

Der Messer wählt die Sätze jeder Anfrage in dieser Reihenfolge:

1. Eine passende [`pricing.overrides`](/docs/de/claude-apps-gateway-config#pricing)-Zeile für den Upstream, der die Anfrage bedient hat. Erfordert v2.1.227 oder später.
2. Listenpreis für die Upstream-Modell-ID, die Zeichenkette, die das Gateway an den Anbieter sendet, wenn die Claude Code-Kostenentabelle sie erkennt. Die Tabelle akzeptiert Anthropic-, Amazon Bedrock-, Google Cloud's Agent Platform- und Microsoft Foundry-ID-Formen.
3. Listenpreis für die [`models[].id`](/docs/de/claude-apps-gateway-config#models), die Sie dieser Upstream-ID zugeordnet haben, für Upstream-Zeichenketten, die keinen Modellnamen enthalten, wie ein Amazon Bedrock-Anwendungs-Inferenz-Profil-ARN oder ein Microsoft Foundry-Bereitstellungsname. Erfordert v2.1.218 oder später.
4. Die Tier für unbekannte Modelle von \$5/\$25 pro Million Input-/Output-Token, daher ist eine ID, die der Messer nicht einordnen kann, niemals kostenlos. Das Gateway warnt beim Start und einmal pro ID zur Laufzeit, wenn es diese Tier verwendet.

Welcher Satz auch immer gilt, multipliziert der Messer den Betrag dann mit [`pricing.multiplier`](/docs/de/claude-apps-gateway-config#pricing), Standard `1`.

Client-Abbrüche werden auch abgerechnet. Wenn ein Stream ohne den finalen Nutzungs-Frame des Upstream endet, rechnet der Messer eine Untergrenze von etwa vier Zeichen pro Output-Token für den bereits an den Client gesendeten Text ab, daher vermeiden Anfragen, die früh abgebrochen werden, nicht die Einhaltung eines Limits.

<h3 id="postgres-availability">
  Postgres-Verfügbarkeit
</h3>

Die Vorabprüfung fragt Postgres mit einem Zwei-Sekunden-Timeout ab. Wenn der Speicher nicht erreichbar ist oder das Timeout überschreitet, schlägt die Durchsetzung standardmäßig offen fehl: Die Anfrage wird fortgesetzt, das Gateway protokolliert eine Warnung, und die Antwort enthält keine `anthropic-ratelimit-unified-*`-Header. Setzen Sie [`enforcement.fail_closed_on_error: true`](/docs/de/claude-apps-gateway-config#enforcement), um stattdessen geschlossen fehlzuschlagen, was den gleichen `429 billing_error` mit der Nachricht `spend limit unavailable` und ohne Zeitraum, Rücksetzeit oder `retry-after`-Header zurückgibt. Fail-Open verhindert, dass ein Speicherausfall zu einem Inferenzausfall wird; Fail-Closed garantiert keine nicht gemessenen Ausgaben.

<h3 id="usage-warnings-in-claude-code">
  Nutzungswarnungen in Claude Code
</h3>

Claude Code warnt einen Entwickler, wenn er sich seinem Limit nähert: sobald die Auslastung 75% überschreitet, und erneut über 95% seines am meisten verbrauchten Limits. Wenn das Gateway eine Anfrage blockiert, zeigt Claude Code die `429`-Nachricht des Gateways unverändert an, einschließlich Ihrer `admin.blocked_message`.

Die Warnung funktioniert mit Response-Headern:

* Mit v2.1.225 oder später auf dem Gateway-Server trägt jede erfolgreiche `/v1/messages`-Antwort für einen Entwickler, der ein Limit hat, seine eigene Limit-Auslastung und Rücksetzeit in den `anthropic-ratelimit-unified-*`-Headern.
* Mit v2.1.225 oder später auch auf der Maschine des Entwicklers liest Claude Code die Header und zeigt die Warnung an.

Die Header beschreiben immer das eigene Limit des Entwicklers: Das Gateway entfernt die Rate-Limit-Header des Upstream-Anbieters, die Ihre gemeinsame Quote beschreiben, und leitet sie niemals weiter.

Mit v2.1.251 oder später auf der Maschine des Entwicklers liest Claude Code auch die gleichen Header, um einen **Spend limit**-Balken in `/usage` anzuzeigen, mit dem Prozentsatz ihres verwendeten Limits und wann es zurückgesetzt wird, und um ein `rate_limits.spend_limit`-Objekt zur [Statuszeile](/docs/de/statusline#rate-limit-usage)-Eingabe hinzuzufügen. Claude Code zeigt beide als Prozentsatz statt als Dollarbetrag an und benötigt nichts Neueres als v2.1.225 auf dem Gateway-Server.

<h2 id="admin-api-reference">
  Admin-API-Referenz
</h2>

Die folgenden Endpunkte werden unter `/v1/organizations/spend_limits` bereitgestellt.

| Methode und Pfad                               | Beschreibung                                                                                                                                                                 |
| ---------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `GET /v1/organizations/spend_limits`           | Konfigurierte Limits auflisten, optional gefiltert auf einen `scope_type` von `organization`, `rbac_group` oder `user`. Abfrage: `?limit=&after_id=&before_id=&scope_type=`. |
| `POST /v1/organizations/spend_limits`          | Erstellen oder ersetzen Sie ein Limit für `{scope, period}`.                                                                                                                 |
| `GET /v1/organizations/spend_limits/{id}`      | Rufen Sie ein Limit nach seiner `spl_`-präfixierten ID ab.                                                                                                                   |
| `DELETE /v1/organizations/spend_limits/{id}`   | Löschen Sie ein Limit. Gibt `{type: "spend_limit_deleted", id}` zurück.                                                                                                      |
| `GET /v1/organizations/spend_limits/effective` | Aufgelöstes Limit und Ausgaben bis zum aktuellen Zeitraum pro Principal pro Zeitraum.                                                                                        |
| `GET /v1/organizations/spend_limits/audit`     | Admin-Mutationsverlauf, neueste zuerst. Abfrage: `?limit=&after_id=`.                                                                                                        |

Konventionen spiegeln Anthropics Admin-API wider:

* Ein `type` auf jedem Objekt
* `spl_`-präfixierte IDs
* Beträge als Ganzzahl-Strings von USD-Cent; `POST` lehnt jede andere `currency` mit `400` ab
* Die `{type: "error", error: {type, message}, request_id}`-Fehler-Umhüllung
* Ein `request-id`-Response-Header auf jeder Admin-Antwort, Erfolg oder Fehler; Fehlertexte enthalten ihn auch als `request_id`

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

Gibt den Ausgabenlimit-Mutationsverlauf zurück: wer welches Limit geändert hat, mit Vor-/Nach-Snapshots, neueste zuerst. `has_more` ist exakt. Dieser Endpunkt folgt den lokalen Admin-API-Konventionen statt einer First-Party-Drahtform.

<h3 id="pagination">
  Pagination
</h3>

Die rohe Liste paginiert nach `after_id` und `before_id`, die sich gegenseitig ausschließende `spl_…`-IDs sind; Ergebnisse werden nach Erstellung sortiert und `has_more` spiegelt die Traversierungsrichtung wider. `/effective` paginiert nach dem undurchsichtigen `next_page`-Token, der als `?page=` zurückgegeben wird, mit Principal in aufsteigender Reihenfolge sortiert, sodass Seiten stabil bleiben, während Ausgaben aufgezeichnet werden. `limit` ist 1–1000, Standard 20, auf beiden. `/audit` paginiert nach `after_id`, der numerischen `id` des letzten Ereignisses auf der vorherigen Seite, und sein `limit` hat einen Standard von 100.

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

Wenn ein Entwickler geht, löschen Sie alle Pro-Benutzer-Limits über `DELETE /v1/organizations/spend_limits/{id}`; ihre Ausgaben und Identitätszeilen veralten auf den oben genannten Aufbewahrungsfenstern. Um eine Person sofort zu löschen, für Offboarding oder eine Datenschutzanfrage (DSAR), führen Sie `DELETE FROM principal_emails WHERE principal = '<sub>'` direkt gegen die Gateway-Datenbank aus. Das entfernt die einzige Tabelle, die ihre E-Mail, ihren Namen und ihre Gruppen enthält. Die `spend`- und `admin_audit`-Zeilen verweisen nur auf die pseudonyme OIDC `sub` und veralten auf ihren eigenen Fenstern.

<h2 id="related">
  Verwandt
</h2>

* [`admin`- und `enforcement`-Konfiguration](/docs/de/claude-apps-gateway-config#admin): Aktivierung der Admin-API und Abstimmung der Aufbewahrung
* [Bereitstellungsleitfaden](/docs/de/claude-apps-gateway-deploy#postgres): Postgres-Schema und Backup-Anleitung
