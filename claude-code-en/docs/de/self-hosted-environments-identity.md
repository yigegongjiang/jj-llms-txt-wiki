> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Sitzungsidentität in selbstgehosteten Umgebungen überprüfen

> Überprüfen Sie das CLAUDE_CODE_SESSION_ACCESS_TOKEN JWT, damit Dienste in Ihrem Netzwerk Anfragen von Sitzungen in Ihrer selbstgehosteten Umgebung vertrauen können.

<Note>
  Selbstgehostete Umgebungen befinden sich in der öffentlichen Beta für Team- und Enterprise-Pläne; ein [Owner](/docs/de/cloud-environments#organization-shared-environments) aktiviert sie, indem er **Selbstgehostete Umgebungen zulassen** auf der [**Cloud-Umgebungen**-Administratorseite](https://claude.ai/admin-settings/cloud-environments) einschaltet. Diese Seite behandelt die Überprüfung der Sitzungsidentität; siehe den [Schnellstart](/docs/de/self-hosted-environments-quickstart) für die Einrichtung und [In Produktion bereitstellen](/docs/de/self-hosted-environments-deploy) für die Fleet-Rezepte.
</Note>

Eine [selbstgehostete Umgebung](/docs/de/self-hosted-environments) ermöglicht es [Claude Code im Web](/docs/de/claude-code-on-the-web)-Sitzungen, auf einer Infrastruktur zu laufen, die Sie betreiben, anstatt auf der von Anthropic. Da die Sitzung in Ihrem Netzwerk läuft, kann Claude Ihre internen Dienste direkt aufrufen. Diese Dienste benötigen eine Möglichkeit zu bestätigen, dass eine Anfrage von einer Claude Code-Sitzung in Ihrer Umgebung stammt, und um die Benutzer- oder Dienstidentität zu identifizieren, die diese Sitzung erstellt hat.

Jede Sitzung in einer selbstgehosteten Umgebung erhält ein signiertes JSON Web Token (JWT) in der Umgebungsvariablen `CLAUDE_CODE_SESSION_ACCESS_TOKEN`. Eine Sitzung präsentiert das Token wie jede Bearer-Anmeldeinformation; beispielsweise kann ein Skript, das Claude ausführt, Ihren Dienst mit `curl -H "Authorization: Bearer $CLAUDE_CODE_SESSION_ACCESS_TOKEN"` aufrufen. Anthropic signiert das Token und veröffentlicht die Überprüfungsschlüssel an einem öffentlichen JWKS-Endpunkt. Ihre Dienste rufen diese Schlüssel ab, überprüfen die Signatur und lesen die Claims, um zu entscheiden, welchen Zugriff sie gewähren.

<h2 id="the-session-token">
  Das Sitzungstoken
</h2>

Bevor Sie Überprüfungscode schreiben, wissen Sie, was das Token etabliert und welche Form Ihre JWT-Bibliothek sehen wird.

<h3 id="what-the-token-proves">
  Was das Token beweist
</h3>

Ein gültiges Token etabliert einige Fakten und bewusst nicht andere:

* **Beweist**: Anthropic hat das Token für eine bestimmte Sitzung in einer bestimmten Umgebung ausgestellt und wie die Sitzung erstellt wurde: von einem Benutzer in Ihrer Organisation oder von der Dienstidentität Ihrer Organisation, was der Weg ist, wie [Claude Tag-Kanalsitzungen](https://claude.com/docs/claude-tag/concepts/agent-identity) starten
* **Beweist nicht**: welcher Prozess auf dem Runner-Host es präsentiert. Das Token sitzt in einer Umgebungsvariablen innerhalb der Sitzung, daher kann jeder Code, den Claude ausführt, und jedes Tool oder MCP-Server, das die Sitzung startet, es lesen und präsentieren.

Zwei Konsequenzen für Ihre Dienste:

* Überprüfen Sie den `aud`-Claim gegen Ihre Umgebungs-ID, den `ccpool_...`-Wert, der mit Ihrer Umgebung auf der [**Cloud-Umgebungen**-Administratorseite](https://claude.ai/admin-settings/cloud-environments) angezeigt wird, um Tokens abzulehnen, die für die Umgebung einer anderen Organisation ausgestellt wurden.
* Beschränken Sie Anmeldeinformationen, die Sie vom Token ableiten, auf das, was eine einzelne Codierungssitzung tun sollte, nicht auf alles, was der Ersteller der Sitzung tun kann. Siehe [Abgeleitete Anmeldeinformationen beschränken](#scope-derived-credentials).

<h3 id="token-format">
  Token-Format
</h3>

Der Wert von `CLAUDE_CODE_SESSION_ACCESS_TOKEN` hat ein `sk-ant-cc-`-Präfix gefolgt von einem Standard-JWT mit drei Teilen:

```text theme={null}
sk-ant-cc-<base64url header>.<base64url payload>.<base64url signature>
```

Entfernen Sie das Präfix, bevor Sie den Wert an eine JWT-Bibliothek übergeben. Tokens, die für von Anthropic gehostete Cloud-Sitzungen ausgestellt werden, tragen stattdessen ein `sk-ant-si-`-Präfix und werden von einem anderen Schlüsselsatz signiert, daher lehnen Sie jeden Wert ab, der nicht mit `sk-ant-cc-` beginnt.

Der Signaturalgorithmus ist `ES256`, das ist ECDSA auf der P-256-Kurve mit SHA-256. Der Token-Header trägt einen `kid`, der identifiziert, welcher Schlüssel im JWKS ihn signiert hat.

<h2 id="verify-the-token">
  Token verifizieren
</h2>

Die Verifizierung läuft an einem von zwei Orten ab. Dienste in Ihrem Netzwerk verifizieren den Token kryptografisch gegen die veröffentlichten Schlüssel von Anthropic, und Wrapper-Skripte innerhalb der Sitzung können stattdessen den integrierten Decoder des Runner-Binärs verwenden.

<h3 id="verify-the-token-from-your-service">
  Token von Ihrem Dienst verifizieren
</h3>

Anthropic veröffentlicht die Verifizierungsschlüssel an einem öffentlichen, nicht authentifizierten Endpunkt:

```text theme={null}
https://api.anthropic.com/v1/code/.well-known/jwks.json
```

Die Antwort ist ein Standard-[JSON Web Key Set](https://www.rfc-editor.org/rfc/rfc7517). Anthropic rotiert die Signaturschlüssel regelmäßig, und Schlüssel von vor einer Rotation bleiben in der Menge lange genug erhalten, damit die von ihnen signierten Tokens weiterhin verifiziert werden. Heften Sie daher keinen einzelnen Schlüssel fest. Der Endpunkt setzt `Cache-Control: public, max-age=300`, daher ist es sicher, die Schlüsselmenge zu cachen und alle fünf Minuten erneut abzurufen.

Verifizieren Sie jeden eingehenden Token gegen diese Prüfungen:

<Steps>
  <Step title="Präfix prüfen">
    Lehnen Sie den Wert ab, wenn er nicht mit `sk-ant-cc-` beginnt, und entfernen Sie dann dieses Präfix. Der Rest ist ein Standard-Compact-JWT.
  </Step>

  <Step title="Signatur verifizieren">
    Rufen Sie die JWKS ab, wählen Sie den Schlüssel aus, dessen `kid` dem Token-Header entspricht, und verifizieren Sie die `ES256`-Signatur. Lehnen Sie Tokens ab, deren `alg`-Header nicht `ES256` ist. Wenn ein Token mit einer `kid` ankommt, die sich nicht in Ihrem gecachten Schlüsselsatz befindet, rufen Sie die JWKS einmal ab, bevor Sie ihn ablehnen: Nach einer Rotation werden neue Tokens mit einem Schlüssel signiert, den Ihr gecachter Satz noch nicht hat.
  </Step>

  <Step title="Aussteller verifizieren">
    Lehnen Sie den Token ab, wenn `iss` nicht genau `ccr` ist.
  </Step>

  <Step title="Zielgruppe gegen Ihre Umgebung verifizieren">
    Der `aud`-Anspruch ist ein Array. Lehnen Sie den Token ab, es sei denn, er enthält Ihre Umgebungs-ID, die die Form `ccpool_...` hat. Die Umgebungs-ID wird im Detaildialog Ihrer Umgebung auf der [**Cloud-Umgebungen**-Administratorseite](https://claude.ai/admin-settings/cloud-environments) angezeigt und erscheint als `ccr:pool_id`-Anspruch in jedem der Sitzungs-Tokens der Umgebung. Diese Prüfung ist das, was den Token auf Ihre Umgebung beschränkt und Tokens ablehnt, die für andere Organisationen ausgestellt wurden.
  </Step>

  <Step title="Rolle verifizieren">
    Lehnen Sie den Token ab, wenn `ccr:role` nicht genau `session_worker` ist. Andere Tokens, die für selbstgehostete Umgebungen ausgestellt werden, wie Umgebungsgeheimnisse, Runner-Tokens und Arbeitsaufträge, werden vom gleichen Schlüsselsatz signiert, tragen aber unterschiedliche Rollen.
  </Step>

  <Step title="Ablauf verifizieren">
    Lehnen Sie den Token ab, wenn `exp` in der Vergangenheit liegt. Anthropic stellt Sitzungs-Tokens standardmäßig mit einer Lebensdauer von vier Stunden und maximal acht Stunden aus. Der Runner aktualisiert den Token vor Ablauf und überträgt den neuen Wert an die Sitzung, daher erben Unterprozesse, die Claude nach einer Aktualisierung startet, ihn. Eine Sitzung kann daher über ihre Lebensdauer hinweg mehrere unterschiedliche gültige Tokens für Ihren Dienst präsentieren.
  </Step>

  <Step title="Identität lesen">
    Die Identität des erstellenden Benutzers befindet sich im `act`-Anspruch: `act.sub` ist seine Anthropic-Benutzer-ID in der präfixierten Form `user:<id>`, und `act.email`, wenn die erstellende Oberfläche eine aufgezeichnet hat, ist seine E-Mail-Adresse. Sitzungen, die die Service-Identität Ihrer Organisation erstellt, einschließlich Claude-Tag-Kanalsitzungen, tragen stattdessen einen `agent:`-Betreff, daher behandeln Sie eine Sitzung nur als vom Benutzer erstellt, wenn `act.sub` das `user:`-Präfix trägt, anstatt zu testen, ob Identitätsansprüche fehlen. Siehe die [Anspruchsreferenz](#claims-reference) für die vollständige Struktur und die flachen doppelten Ansprüche.
  </Step>
</Steps>

Die Prüfungen werden direkt auf Standard-JWT-Bibliotheken abgebildet. Die folgenden Beispiele implementieren die vollständige Sequenz in Node.js mit [`jose`](https://www.npmjs.com/package/jose), das JWKS-Abruf, Caching und `kid`-Auswahl handhabt, und in Python mit [`PyJWT`](https://pyjwt.readthedocs.io/) und seinem integrierten JWKS-Client.

<Tabs>
  <Tab title="Node.js (jose)">
    ```typescript theme={null}
    import { createRemoteJWKSet, jwtVerify } from "jose";

    const JWKS = createRemoteJWKSet(
      new URL("https://api.anthropic.com/v1/code/.well-known/jwks.json")
    );

    const PREFIX = "sk-ant-cc-";
    const EXPECTED_POOL_ID = "ccpool_...";

    export async function verifySessionToken(raw: string) {
      if (!raw.startsWith(PREFIX)) {
        throw new Error("not a self-hosted runner session token");
      }
      const jwt = raw.slice(PREFIX.length);

      const { payload } = await jwtVerify(jwt, JWKS, {
        issuer: "ccr",
        audience: EXPECTED_POOL_ID,
        algorithms: ["ES256"],
      });

      if (payload["ccr:role"] !== "session_worker") {
        throw new Error("token is not a session_worker token");
      }

      const act = payload.act as { email?: string; sub?: string };
      return {
        sessionId: payload["ccr:session_id"] as string,
        poolId: payload["ccr:pool_id"] as string,
        orgId: payload["ccr:org_id"] as string,
        creatorEmail: act?.email,
        creatorSub: act?.sub,
      };
    }
    ```
  </Tab>

  <Tab title="Python (PyJWT)">
    ```python theme={null}
    import jwt
    from jwt import PyJWKClient

    JWKS_URL = "https://api.anthropic.com/v1/code/.well-known/jwks.json"
    PREFIX = "sk-ant-cc-"
    EXPECTED_POOL_ID = "ccpool_..."

    jwks = PyJWKClient(JWKS_URL)


    def verify_session_token(raw: str) -> dict:
        if not raw.startswith(PREFIX):
            raise ValueError("not a self-hosted runner session token")
        token = raw.removeprefix(PREFIX)

        signing_key = jwks.get_signing_key_from_jwt(token)
        payload = jwt.decode(
            token,
            signing_key.key,
            algorithms=["ES256"],
            issuer="ccr",
            audience=EXPECTED_POOL_ID,
        )

        if payload.get("ccr:role") != "session_worker":
            raise ValueError("token is not a session_worker token")

        act = payload.get("act") or {}
        return {
            "session_id": payload["ccr:session_id"],
            "pool_id": payload["ccr:pool_id"],
            "org_id": payload["ccr:org_id"],
            "creator_email": act.get("email"),
            "creator_sub": act.get("sub"),
        }
    ```
  </Tab>
</Tabs>

<h3 id="verify-the-token-inside-the-session">
  Token innerhalb der Sitzung verifizieren
</h3>

[Wrapper-Skripte](/docs/de/self-hosted-environments-configuration#wrapper-scripts) laufen innerhalb der Sitzung, bevor Claude startet. Anstatt eine JWT-Bibliothek aufzurufen, können sie den Unterbefehl `self-hosted-runner decode-token` des Runner-Binärs ausführen. Der Unterbefehl liest den Token aus einem Positionsargument, aus `CLAUDE_CODE_SESSION_ACCESS_TOKEN` oder aus gepiptem stdin, in dieser Reihenfolge, entfernt dann das Präfix, verifiziert die Signatur gegen den JWKS-Endpunkt, prüft den Ablauf und gibt die Ansprüche als JSON aus. Der Unterbefehl führt nur die Signatur- und Ablaufprüfungen durch; er prüft nicht `iss`, `aud` oder `ccr:role`. Wenn die Authentifizierungsentscheidung Ihres Wrappers von diesen Ansprüchen abhängt, lesen Sie sie aus dem gedruckten JSON und vergleichen Sie sie explizit.

Dieser Befehl extrahiert die Ersteller-Identität und bevorzugt dabei den Betreff des SSO-Anbieters, dann die E-Mail-Adresse, dann den `act.sub`-Betreff des Erstellers, `user:<id>` oder `agent:<id>`:

```bash theme={null}
"$CLAUDE_RUNNER_CLAUDE_BIN" self-hosted-runner decode-token | jq -re '.act.attested_by.sub // .act.email // .act.sub'
```

Wrapper erhalten den absoluten Pfad zum Binär des Runners selbst in `CLAUDE_RUNNER_CLAUDE_BIN`; verwenden Sie diesen Pfad anstelle eines PATH-aufgelösten `claude`, damit die Dekodierung auf dem gleichen Binär läuft, das der Runner selbst verwendet.

Verwenden Sie `jq -re` anstelle von `jq -r`, damit ein fehlender Anspruch einen Exit-Code ungleich Null verursacht. Mit nur `-r` gibt ein fehlender Anspruch die Literalzeichenkette `null` aus und beendet mit Null, was einen schlechten Wert stillschweigend nachgelagert übergibt. Übergeben Sie `--no-verify` an `decode-token` nur zur Offline-Inspektion, wenn der JWKS-Endpunkt nicht erreichbar ist.

<h2 id="claims-reference">
  Claims-Referenz
</h2>

Die folgende Tabelle listet die Sitzungs-Token-Claims auf, die für die Überprüfung relevant sind. Lesen Sie die Identität aus dem `ccr:*`-Namespace und der `act`-Kette; die flachen `account_email`-, `organization_uuid`- und `account_uuid`-Claims sind Rückwärtskompatibilitätsduplikate, die möglicherweise entfernt werden. Sitzungen, die die Dienstidentität Ihrer Organisation erstellt, einschließlich Claude Tag-Kanalsitzungen, tragen einen `agent:`-Betreff in `act.sub` und lassen `act.email`, `ccr:account_id`, `account_email` und `account_uuid` weg. Die beiden E-Mail-Claims sind auch für vom Benutzer erstellte Sitzungen optional: Anthropic zeichnet sie bei der Sitzungserstellung nur auf, wenn die Anmeldeinformationen der erstellenden Anfrage eine E-Mail tragen, und eine Sitzung, die von der CLI versendet wird, kann beide fehlen, daher basieren Sie die Identität auf `act.sub` oder `ccr:account_id` anstelle von E-Mail. Tokens können auch zusätzliche Claims über diese Tabelle hinaus tragen; ignorieren Sie Claims, die Sie nicht erkennen.

| Claim               | Typ               | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :------------------ | :---------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `iss`               | string            | Immer `ccr`.                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `sub`               | string            | `ccr:session:<session_id>`.                                                                                                                                                                                                                                                                                                                                                                                                               |
| `aud`               | Array von Strings | Enthält immer `anthropic-api`. Für Sitzungen in selbstgehosteten Umgebungen enthält das Array auch Ihre Umgebungs-ID, wie `ccpool_...`. Überprüfen Sie die Umgebungs-ID, nicht `anthropic-api`.                                                                                                                                                                                                                                           |
| `exp`               | number            | Ablauf als Unix-Zeitstempel. Vier-Stunden-Standard-Lebensdauer, acht-Stunden-Maximum.                                                                                                                                                                                                                                                                                                                                                     |
| `iat`               | number            | Ausgestellt-bei als Unix-Zeitstempel.                                                                                                                                                                                                                                                                                                                                                                                                     |
| `jti`               | string            | Eindeutige Token-ID.                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `ccr:role`          | string            | Immer `session_worker` für Sitzungs-Tokens.                                                                                                                                                                                                                                                                                                                                                                                               |
| `ccr:session_id`    | string            | Die Sitzungs-ID. Gleicher Wert wie das Suffix von `sub`.                                                                                                                                                                                                                                                                                                                                                                                  |
| `ccr:pool_id`       | string            | Ihre Umgebungs-ID. Gleicher Wert, der in `aud` erscheint.                                                                                                                                                                                                                                                                                                                                                                                 |
| `ccr:org_id`        | string            | Ihre Anthropic-Organisations-ID.                                                                                                                                                                                                                                                                                                                                                                                                          |
| `ccr:account_id`    | string            | Die Anthropic-Konto-ID des erstellenden Benutzers: der Wert von `act.sub` ohne das `user:`-Präfix, eine getaggte `user_...`-ID. Der gleiche Wert, den die `CLAUDE_RUNNER_ACCOUNT_ID` des [spawn-runner-Hooks](/docs/de/self-hosted-environments-configuration#the-spawn-runner-hook) trägt und [`--lock-to-account`](/docs/de/self-hosted-environments-reference#runner-cli-flags) akzeptiert, daher vergleichen sich die drei als gleiche Strings. |
| `account_email`     | string            | Duplikat von `act.email`; fehlt, wenn `act.email` fehlt.                                                                                                                                                                                                                                                                                                                                                                                  |
| `organization_uuid` | string            | Ihre Anthropic-Organisations-UUID.                                                                                                                                                                                                                                                                                                                                                                                                        |
| `account_uuid`      | string            | Die Anthropic-Konto-UUID des erstellenden Benutzers.                                                                                                                                                                                                                                                                                                                                                                                      |
| `act`               | object            | [RFC 8693](https://www.rfc-editor.org/rfc/rfc8693) Delegationskette. Siehe [Die `act`-Kette](#the-act-chain).                                                                                                                                                                                                                                                                                                                             |

<h3 id="the-act-chain">
  Die `act`-Kette
</h3>

Der `act`-Claim zeichnet den vollständigen Delegationspfad von der Benutzer- oder Dienstidentität, die die Sitzung erstellt hat, bis zur [Umgebung](/docs/de/self-hosted-environments#key-concepts), deren Geheimnis den Runner zugelassen hat, und die Identität, die dieses Geheimnis erstellt hat, auf. Der Ersteller ist der äußerste Akteur, daher identifiziert `act.sub` ihn direkt.

| Pfad              | Beschreibung                                                                                                                                                                                                                                                                                            |
| :---------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `act.sub`         | Die Anthropic-Benutzer-ID des erstellenden Benutzers in der Form `user:<id>` oder `agent:<id>`, wenn die Dienstidentität Ihrer Organisation die Sitzung erstellt hat, wie sie es für Claude Tag-Kanalsitzungen tut.                                                                                     |
| `act.email`       | Die E-Mail-Adresse des erstellenden Benutzers, wenn eine bei der Sitzungserstellung aufgezeichnet wurde. Verlangen Sie sie nicht; basieren Sie auf `act.sub`.                                                                                                                                           |
| `act.attested_by` | Die Attestation des Upstream-Identitätsanbieters für den erstellenden Benutzer, wenn verfügbar. `act.attested_by.sub` ist der Betreff, den Ihr SSO-Anbieter, wie Google oder Okta, ausgestellt hat. Bevorzugen Sie dies gegenüber `act.email`, wenn Sie Identitäten in Ihren eigenen Systemen zuordnen. |
| `act.act`         | Der Runner, der die Sitzung gespawnt hat. `act.act.sub` ist `ccr:runner:<runner_id>`.                                                                                                                                                                                                                   |
| `act.act.act`     | Die Umgebung. `act.act.act.sub` ist `ccr:pool:<pool_id>`.                                                                                                                                                                                                                                               |
| `act.act.act.act` | Die Identität, die das Umgebungsgeheimnis erstellt hat, mit dem sich der Runner registriert hat. Die Kette endet hier.                                                                                                                                                                                  |

<h2 id="scope-derived-credentials">
  Abgeleitete Anmeldeinformationen beschränken
</h2>

Das Sitzungs-Token identifiziert die Benutzer- oder Dienstidentität, die die Sitzung erstellt hat, aber behandeln Sie es nicht als gleichwertig mit diesem Ersteller, der sich direkt anmeldet. Das Token sitzt in einer Umgebungsvariablen innerhalb der Sitzung, daher kann jeder Code, den Claude ausführt, und jedes Tool oder MCP-Server, das die Sitzung startet, es lesen und präsentieren.

Die Überprüfung ist auch offline: Ein Token, das gegen das JWKS überprüft wird, bleibt gültig bis zu seinem `exp`, was auch immer mit der Sitzung seitdem passiert ist, und Anthropic veröffentlicht keinen Widerrufsfeed für Sitzungs-Tokens. Binden Sie alles, das Sie vom Token ableiten, entsprechend.

Wenn Ihr Dienst das Token gegen interne Anmeldeinformationen austauscht, geben Sie Anmeldeinformationen aus, die auf das beschränkt sind, was eine Codierungssitzung erreichen sollte:

* **Fähigkeiten begrenzen**: Gewähren Sie Lese- und Schreibzugriff auf die Ressourcen, die die Sitzung für Codierungsaufgaben benötigt, nicht auf administrative Fähigkeiten, die der Ersteller anderswo hält.
* **Lebensdauer begrenzen**: Binden Sie abgeleitete Anmeldeinformationen an das `exp` des Tokens oder kürzer.
* **Audit als Sitzung**: Zeichnen Sie die `ccr:session_id` und `jti` neben der Ersteller-Identität auf, damit Sie Aktionen auf eine bestimmte Sitzung zurückverfolgen können.

<h2 id="related-environment-variables">
  Zugehörige Umgebungsvariablen
</h2>

Die Ersteller-Identität erscheint auch in einfachen Umgebungsvariablen auf zwei Oberflächen, die das Token niemals überprüfen:

* **Der [`spawn-runner`-Hook](/docs/de/self-hosted-environments-configuration#the-spawn-runner-hook), auf dem Orchestrator**: Der Hook läuft, bevor ein Runner für eine warteschlangige Sitzung existiert, und erhält die Ersteller-Identität in Variablen wie `CLAUDE_RUNNER_ACCOUNT_EMAIL` und `CLAUDE_RUNNER_ACCOUNT_ID`. Der Orchestrator liest sie aus dem Arbeitsauftrag, dem signierten Einmal-Token, das das Spawnen eines Runners autorisiert, ohne die Signatur des Arbeitsauftrags selbst zu überprüfen; die Claims werden vertraut, weil der Arbeitsauftrag über die Verbindung des Orchestrators zu Anthropic ankommt, die das Umgebungsgeheimnis authentifiziert.
* **[Wrapper-Skripte](/docs/de/self-hosted-environments-configuration#wrapper-scripts), innerhalb der Sitzung**: Wrapper erhalten `CCR_SESSION_ACCOUNT_EMAIL`, die E-Mail-Adresse des Erstellers, die ohne Signaturüberprüfung aus dem Token vorextrahiert wurde. Die Variable ist für Beschriftungen geeignet, wie Commit-Trailer, nicht für Auth-Entscheidungen.

Verwenden Sie die einfachen Variablen für Orchestrator-seitige Entscheidungen wie die Auswahl eines Maschinenabbilds. Verwenden Sie `CLAUDE_CODE_SESSION_ACCESS_TOKEN`, wenn ein nachgelagerter Dienst unabhängigen kryptographischen Beweis benötigt, anstatt dem Runner-Umfeld zu vertrauen.

<h2 id="what’s-next">
  Nächste Schritte
</h2>

* [Selbstgehostete Umgebungen](/docs/de/self-hosted-environments): die Umgebung, den Runner und das Sitzungsmodell; der [Schnellstart](/docs/de/self-hosted-environments-quickstart) und [In Produktion bereitstellen](/docs/de/self-hosted-environments-deploy) enthalten Einrichtung und Betrieb
* [Sitzungen anpassen](/docs/de/self-hosted-environments-configuration): Wrapper-Skripte, die das Token verbrauchen, und der `spawn-runner`-Hook
* [Referenz](/docs/de/self-hosted-environments-reference): CLI-Flags, Umgebungsvariablen und Metriken
