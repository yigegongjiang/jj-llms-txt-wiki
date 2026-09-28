> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Sitzungen in selbstgehosteten Umgebungen anpassen

> Passen Sie selbstgehostete Umgebungssitzungen mit Wrapper-Skripten für Anmeldedaten pro Sitzung, Lifecycle-Hooks und On-Demand-Runner-Spawning an.

<Note>
  Selbstgehostete Umgebungen befinden sich in der öffentlichen Beta für Team- und Enterprise-Pläne; ein [Owner](/docs/de/cloud-environments#organization-shared-environments) aktiviert sie, indem er **Selbstgehostete Umgebungen zulassen** auf der [**Cloud-Umgebungen**-Administratorseite](https://claude.ai/admin-settings/cloud-environments) aktiviert. Diese Seite setzt einen funktionierenden Runner voraus; siehe die [Schnellstartanleitung](/docs/de/self-hosted-environments-quickstart) für die Einrichtung und [In die Produktion bereitstellen](/docs/de/self-hosted-environments-deploy) für die Fleet-Rezepte.
</Note>

Eine [selbstgehostete Umgebung](/docs/de/self-hosted-environments) führt Claude Code [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) auf Ihrer eigenen Infrastruktur aus, ausgeführt durch einen Runner-Prozess, den Sie bereitstellen. Ohne Konfiguration klont dieser Runner das Repository der Sitzung, startet Claude Code und räumt auf. Diese Seite ist für den Plattformingenieur, der die Runner betreibt: Sie behandelt die Erweiterungspunkte für den Fall, dass diese Standardeinstellungen nicht passen, von der Bereitstellung von Anmeldedaten pro Sitzung bis zum vollständigen Ersetzen des Checkouts. Wrapper und Hooks werden als ausführbare Dateien auf dem Runner-Host ausgeführt, bei dem es sich um Linux oder macOS handelt, und die Beispiele auf dieser Seite gehen von einer POSIX-Shell aus.

Einige Hook-Umgebungsvariablen auf dieser Seite verwenden noch `pool`, wie `CLAUDE_RUNNER_POOL_ID`; die CLI-Flag- und Umgebungsvariablennamen verwenden `environment`, wie `--environment-secret-file`.

<h2 id="wrapper-scripts">
  Wrapper-Skripte
</h2>

Verwenden Sie ein Wrapper-Skript, wenn jede Sitzung eine Einrichtung benötigt, die der Runner nicht selbst durchführen kann: Bereitstellung kurzlebiger Anmeldedaten mit Bereich auf den Sitzungsersteller, Export umgebungsspezifischer Geheimnisse, Vorbereitung von Sprach-Toolchains oder Anwendung von Ressourcenlimits um den untergeordneten Prozess. Der Runner startet Ihren Wrapper anstelle der Claude Code-Binärdatei, einmal pro Sitzung. Beenden Sie den Wrapper durch `exec` in `$CLAUDE_RUNNER_CLAUDE_BIN`, die eigene Binärdatei des Runners, damit Signale und Exit-Codes korrekt weitergegeben werden.

Zeigen Sie `--exec-path` oder `SELF_HOSTED_RUNNER_EXEC_PATH` auf den Wrapper, wenn Sie den Runner starten:

```bash theme={null}
claude self-hosted-runner --environment-secret-file /etc/claude/environment-secret --exec-path /etc/claude/session-wrapper.sh
```

Der Runner setzt Folgendes in der Umgebung des Wrappers:

| Variable                            | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| :---------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN`  | Das Sitzungs-JWT, mit dem Präfix `sk-ant-cc-`. Sein `act`-Anspruch identifiziert den Sitzungsersteller mit der E-Mail des Erstellers und dem Upstream-Identitätsanbieter-Betreff, wenn die erstellende Oberfläche diese aufgezeichnet hat. Der Wert ist das Token zum Zeitpunkt des Spawning; Aktualisierungen kommen über stdin des Kindes an, daher sieht ein Wrapper nur den Anfangswert. Siehe [Sitzungsidentität überprüfen](/docs/de/self-hosted-environments-identity).                                                                                                                                                                                                                                                                                            |
| `CCR_SESSION_ACCOUNT_EMAIL`         | Die E-Mail des Sitzungsererstellers, vom Runner aus dem `act.email`-Anspruch des Tokens ohne Signaturüberprüfung vorab extrahiert. Geeignet für Beschriftung, wie Commit-Trailer. Wenn die E-Mail die Ausstellung von Anmeldedaten steuert, überprüfen Sie das Token und lesen Sie den Anspruch stattdessen daraus; siehe [Anmeldedaten mit Bereich auf den Sitzungsersteller bereitstellen](#provision-credentials-scoped-to-the-session-creator). Nicht gesetzt, wenn das Token keine Ersteller-E-Mail enthält. Behandeln Sie als personenbezogene Informationen.                                                                                                                                                                                                  |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`     | Die Client-Oberfläche, die die Sitzung erstellt hat, wie `web_claude_ai`, `desktop_app`, `ios`, `claude_code_cli` oder `scheduled_trigger`. Anthropic zeichnet den Wert einmal bei der Sitzungserstellung auf, daher sehen der Wrapper und jeder Lifecycle-Hook denselben Wert. Verwenden Sie ihn nur für Adoptionsanalysen und Beschriftung, nicht als Autorisierungssignal. Nicht gesetzt, wenn die Sitzung keine aufgezeichnete oder erkannte Oberfläche hat, daher referenzieren Sie sie als `${CLAUDE_RUNNER_CLIENT_PLATFORM:-}` unter `set -u`. Erfordert Claude Code v2.1.229 oder später.                                                                                                                                                                    |
| `CLAUDE_RUNNER_CLAUDE_BIN`          | Absoluter Pfad zur eigenen Claude Code-Binärdatei des Runners. Beenden Sie Ihren Wrapper mit `exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"`, um an die angeheftete Binärdatei zu übergeben, ohne einen Installationspfad hartcodieren zu müssen.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `CLAUDE_CODE_REMOTE_SESSION_ID`     | Sitzungs-ID in der getaggten Form `cse_...`. Dies ist dieselbe Sitzung, die die [Lifecycle-Hooks](#lifecycle-hooks) als `CLAUDE_RUNNER_SESSION_ID` in der Form `session_...` sehen; die UUID-Variablen stimmen über beide überein, und das Ersetzen des Präfixes `cse_` durch `session_` ergibt die in der Sitzungs-URL angezeigte ID.                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `CLAUDE_CODE_REMOTE_SESSION_UUID`   | Dieselbe Sitzungs-ID in kanonischer UUID-Form, für Systeme, die auf UUIDs basieren.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `CLAUDE_SESSION_INGRESS_TOKEN_FILE` | Absoluter Pfad zu einer pro-Sitzungs-Datei, die das aktuelle Sitzungs-JWT enthält, das über Token-Aktualisierungen hinweg aktuell gehalten wird. Shell-Unterprozesse lesen es für ihren `Authorization`-Header beim Herunterladen von Anhängen, die der Benutzer zur Sitzung hinzugefügt hat. `exec` bewahrt die Variable automatisch; ein Wrapper, der die Umgebung des Kindes neu erstellt, muss die Variable übertragen, oder Anhang-Downloads funktionieren stillschweigend nicht mehr.                                                                                                                                                                                                                                                                          |
| `CLAUDE_CONFIG_DIR`                 | Pro-Sitzungs-Claude-Konfigurationsverzeichnis, geschrieben beim Sitzungsstart aus dem Snapshot der Konfiguration des Runner-Hosts, den der Runner beim Startup erfasst; siehe [Berechtigungen und Tool-Genehmigung](#permissions-and-tool-approval). Schreibvorgänge hier sind auf diese Sitzung isoliert. Das Verzeichnis bleibt unter `<base-dir>/_sessions/` nach dem Sitzungsende, es sei denn, Sie starten den Runner mit [`--remove-session-state`](/docs/de/self-hosted-environments-reference#runner-cli-flags); siehe [Einen vorgewärmten Checkout wiederverwenden](/docs/de/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout).                                                                                                                            |
| `ANTHROPIC_BASE_URL`                | Die API-Basis-URL, die das Kind verwendet, bereitgestellt von der Kontrolleben pro Sitzung und normalerweise `https://api.anthropic.com`. Überschreiben Sie sie nicht: Die Inferenz-Anmeldedaten der Sitzung sind ein von Anthropic ausgegebenes OAuth-Token, das andere Anbieter nicht akzeptieren, daher ist Inferenz in selbstgehosteten Umgebungen nicht an anderer Stelle routbar.                                                                                                                                                                                                                                                                                                                                                                              |
| `CLAUDE_CODE_OAUTH_TOKEN`           | Das kurzlebige OAuth-Zugangstoken, das das Kind für Modell-Inferenz verwendet, mit Bereich auf Modell-Inferenz und Datei-Upload nur, mit einer Lebensdauer von etwa 30 Minuten. Der Runner prägt es vor Ablauf neu und liefert die Rotation über stdin des Kindes, daher sieht ein Wrapper, der [stdin nicht angehängt hält](#keep-stdin-and-file-descriptor-3-attached), nur den Anfangswert. Verlassen Sie sich nicht auf die IP-Allowlist Ihrer Organisation, um die Verwendung dieses Tokens zu begrenzen: Behandeln Sie es als Bearer-Anmeldedaten, die etwa 30 Minuten lang verwendbar bleiben, wenn sie durchsickern, und protokollieren Sie es nicht, schreiben Sie es nicht auf die Festplatte oder leiten Sie es außerhalb des Sitzungs-Containers weiter. |

Der Wrapper erbt auch den Rest der verwalteten Umgebung des Kindes, einschließlich aller vom Server bereitgestellten Umgebungsvariablen. `exec` propagiert alles automatisch; wenn Ihr Wrapper das Kind auf andere Weise startet, leiten Sie die vollständige Umgebung weiter.

<h3 id="keep-stdin-and-file-descriptor-3-attached">
  Halten Sie stdin und Dateideskriptor 3 angehängt
</h3>

Stdin des Kindes ist der Steuerkanal des Runners. Token-Rotationen und Sitzungsend-Signale kommen darauf an. Der Runner öffnet auch eine Pipe auf Dateideskriptor 3 und liest die Aktivitätssignale des Kindes daraus, um Idle- und Startup-Timeouts zu steuern. Ein einfaches `exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"` bewahrt beide automatisch.

Wenn Ihr Wrapper das Kind mit einem bloßen `&` in den Hintergrund versetzt, trennt es stdin des Kindes: Die Sitzung sieht gesund aus, bis die Lebensdauer des anfänglichen OAuth-Tokens von etwa 30 Minuten abläuft, dann schlagen alle API-Aufrufe mit `401 authentication_error` fehl. Wenn Ihr Wrapper das Kind in den Hintergrund versetzten muss, zum Beispiel um eine Teardown-Falle am Leben zu erhalten, speichern Sie stdin auf Dateideskriptor 4 oder höher und hängen Sie ihn explizit wieder an:

```bash theme={null}
exec 4<&0
"$CLAUDE_RUNNER_CLAUDE_BIN" "$@" <&4 4<&- &
CHILD=$!
trap 'teardown' EXIT
wait "$CHILD"
```

Schließen oder verwenden Sie Dateideskriptor 3 im Wrapper nicht erneut. Das Umleiten von stdout und stderr des Kindes ist in Ordnung.

<h3 id="provision-credentials-scoped-to-the-session-creator">
  Anmeldedaten mit Bereich auf den Sitzungsersteller bereitstellen
</h3>

Verwenden Sie den Unterbefehl `decode-token`, um Ansprüche aus dem Sitzungs-JWT zu lesen. Er liest das Token aus einem Argument, aus `CLAUDE_CODE_SESSION_ACCESS_TOKEN` oder aus stdin, in dieser Reihenfolge; siehe [Token innerhalb der Sitzung überprüfen](/docs/de/self-hosted-environments-identity#verify-the-token-inside-the-session) für das, was es überprüft. Das folgende Beispiel dekodiert die Ersteller-Identität, tauscht sie gegen kurzlebige AWS-Anmeldedaten aus und führt in Claude Code aus:

```bash theme={null}
#!/bin/bash
# Basieren Sie auf der stabilen Anthropic-Benutzer-ID und erfordern Sie einen menschlichen Ersteller.
CREATOR_SUB=$("$CLAUDE_RUNNER_CLAUDE_BIN" self-hosted-runner decode-token \
  | jq -re '.act.sub // "" | select(startswith("user:"))') \
  || { echo "decode-token: verification failed or no human creator" >&2; exit 1; }

creds=$(your-sts-helper assume-role --subject "$CREATOR_SUB") \
  || { echo "credential exchange failed" >&2; exit 1; }
eval "$creds"

exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"
```

Verwenden Sie `jq -re` anstelle von `jq -r`, wenn der extrahierte Anspruch eine Autorisierungsentscheidung steuert, damit ein fehlender Anspruch ungleich Null endet, anstatt die Literalzeichenfolge `null` nachgelagert zu übergeben. Sitzungen, die von einer Organisationsservice-Identität erstellt wurden, wie Bot- und Agent-Sitzungen, tragen einen `agent:`-Betreff anstelle von `user:`, daher lehnt dieses Beispiel sie ab; wenn Ihre Umgebung diese Sitzungen bedient, entscheiden Sie explizit, ob der Wrapper stattdessen auf eine Standard-Anmeldedaten zurückfällt oder beendet wird. Wenn Ihr Anmeldedatenaustausch den SSO-Betreff oder die E-Mail benötigt, lesen Sie stattdessen `.act.attested_by.sub` oder `.act.email` und behandeln Sie deren Abwesenheit: Das Token trägt sie nur, wenn die erstellende Oberfläche sie aufgezeichnet hat, und eine [CLI-versandte Sitzung](/docs/de/self-hosted-environments-testing#run-the-test-loop) kann beide fehlen. Für die vollständige Anspruchsreferenz und Überprüfung von Diensten außerhalb des Runners siehe [Sitzungsidentität überprüfen](/docs/de/self-hosted-environments-identity).

<h2 id="lifecycle-hooks">
  Lifecycle-Hooks
</h2>

Lifecycle-Hooks ersetzen Phasen der Pro-Sitzungs-Pipeline des Runners durch Ihre eigenen Skripte. Zeigen Sie den Runner mit `--hooks-dir <path>` oder `SELF_HOSTED_RUNNER_HOOKS_DIR` auf ein Verzeichnis von Hooks. Der Runner sucht nach ausführbaren Dateien mit bekannten Namen; jeder Hook, der nicht vorhanden ist, fällt auf das integrierte Verhalten zurück, daher schreiben Sie nur die, die Sie benötigen. Hooks werden mit den eigenen Berechtigungen des Runners ausgeführt, und Sitzungskinder teilen diese UID, daher mounten Sie das Hooks-Verzeichnis schreibgeschützt oder backen Sie es in das Image, damit Sitzungscode es nicht ändern kann; siehe den [Härtungsabschnitt](/docs/de/self-hosted-environments-deploy#harden-your-deployment).

Diese Hooks unterscheiden sich von [Claude Code-Hooks](/docs/de/hooks), die innerhalb der Sitzung ausgeführt werden; Lifecycle-Hooks werden auf dem Runner um die Sitzung herum ausgeführt.

<h3 id="checkout">
  checkout
</h3>

Wird einmal pro Repository anstelle des integrierten Klons und Abrufs des Runners ausgeführt. Verwenden Sie den Hook, um von einem Read-Through-Mirror zu klonen, einen Arbeitsbaum aus einem Archiv zu seeden oder Pro-Sitzungs-Git-Authentifizierung anzuwenden. Der Runner setzt:

| Variable                           | Beschreibung                                                                                                                                                                           |
| :--------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_RUNNER_REPO_URL`           | Repository-URL zum Klonen, nachdem alle `--git-host-rewrite` und `--git-ssh-rewrite` angewendet wurden                                                                                 |
| `CLAUDE_RUNNER_REPO_REF`           | Revision zum Auschecken: Branch, Tag oder Commit-SHA, wie die Sitzung sie angefordert hat. Leer bedeutet den Standard-Branch des Repositorys.                                          |
| `CLAUDE_RUNNER_CHECKOUT_PATH`      | Absoluter Pfad, wo der Arbeitsbaum hinterlassen werden muss                                                                                                                            |
| `CLAUDE_RUNNER_SESSION_ID`         | Sitzungs-ID in der getaggten Form `session_...`, für Protokollierung und Korrelation                                                                                                   |
| `CLAUDE_RUNNER_SESSION_UUID`       | Dieselbe Sitzungs-ID in kanonischer UUID-Form                                                                                                                                          |
| `CLAUDE_RUNNER_API_BASE_URL`       | Anthropic-API-Basis-URL für Sitzungs-bezogene Aufrufe                                                                                                                                  |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`    | Die Client-Oberfläche, die die Sitzung erstellt hat, wie `web_claude_ai`, `desktop_app` oder `ios`. Nicht gesetzt, wenn die Sitzung keine aufgezeichnete oder erkannte Oberfläche hat. |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN` | Das Sitzungs-Zugangstoken für Sitzungs-bezogene API-Aufrufe                                                                                                                            |

Das Skript muss einen Arbeitsbaum bei `CLAUDE_RUNNER_CHECKOUT_PATH` hinterlassen, der bei der angeforderten Revision ausgecheckt ist. Detached HEAD ist in Ordnung; der Runner erstellt den Arbeitsbranch der Sitzung darauf. Der Runner überprüft danach, ob der Pfad eine `.git` enthält; wenn Ihr Hook eine Nicht-Git-Quelle wie Perforce oder ein entpacktes Tarball materialisiert, setzen Sie `CLAUDE_RUNNER_SKIP_GIT_VERIFY=1` in der Umgebung des Runners, um diese Überprüfung zu überspringen. Git-basierte Flows wie Arbeitsbranch-Erstellung und Pushing-Ergebnisse erfordern einen Git-Checkout, daher exportieren Sie Ergebnisse aus Nicht-Git-Bäumen mit einem [`post-session`-Hook](#post-session).

Der Runner übergibt keine Git-Anmeldedaten an den Hook. Stattdessen prägen Sie eine Pro-Sitzungs-Klone-Anmeldedaten aus der Identität der Sitzung: Überprüfen Sie `CLAUDE_CODE_SESSION_ACCESS_TOKEN` mit einer Standard-JWT-Bibliothek gegen den JWKS-Endpunkt unter `CLAUDE_RUNNER_API_BASE_URL`, wie in [Token von Ihrem Dienst überprüfen](/docs/de/self-hosted-environments-identity#verify-the-token-from-your-service) beschrieben, dann lassen Sie Ihren Anmeldedatendienst eine kurzlebige Klone-Anmeldedaten für die Identität im `act`-Anspruch des Tokens ausstellen. `CLAUDE_RUNNER_CLAUDE_BIN` ist nicht in der Checkout-Hook-Umgebung gesetzt, daher ist der Unterbefehl `decode-token` hier nicht verfügbar. Das Zurückfallen auf die Git-Authentifizierung, die der Host bereits hat, wie einen SSH-Agent, Anmeldedaten-Helper oder `.netrc`, ist auch eine Option.

Wenn der Hook mit ungleich Null endet oder mit 0 endet, ohne einen verwendbaren Checkout hinterlassen zu haben, hängt das, was der Runner tut, vom Repository ab:

* **Ein Repository, zu dem die Sitzung Ergebnisse pusht**: Der Runner schlägt die Sitzung fehl, und bei einem Nicht-Null-Exit zeigt er das Ende des Stderr des Skripts dem Benutzer an.
* **Ein Repository, das die Sitzung nur liest**, wie ein Repository, das zu einer laufenden Sitzung hinzugefügt wird: Der Runner protokolliert eine `[runner:warn]`-Zeile mit dem Fehlerdetail, postet einen `Skipped`-Schritt zur Sitzung, entfernt, was der Hook bei dem Checkout-Pfad hinterlassen hat, und fährt mit den verbleibenden Repositories fort. Wenn der Runner den Pfad nicht sofort entfernen kann, versucht er die Entfernung beim Sitzungsende erneut. Wenn das Überspringen die Sitzung ohne Repository verlässt, schlägt der Runner die Sitzung trotzdem fehl.

Vor v2.1.228 schlägt der Runner die Sitzung bei einem Hook-Fehler für jedes Repository fehl, daher schlägt ein Read-Only-Repository, das der Hook nicht bedienen konnte, die Sitzung erneut auf jedem frischen Runner fehl, auf dem die Sitzung fortgesetzt wurde.

Der Runner entfernt den Checkout-Pfad nach dem Sitzungsende.

<h3 id="post-session">
  post-session
</h3>

Wird einmal pro Sitzung ausgeführt, nachdem das Claude Code-Kind beendet wurde und bevor der Runner den Arbeitsbereich abbaut. Dieser Hook ist Ihre einzige Chance, ungespeicherte Arbeit zu speichern: Bei `--capacity` über eins löscht der Runner Pro-Sitzungs-Worktrees direkt nach der Hook-Rückgabe, und bei `--capacity 1` wird der wiederverwendete [kanonische Klon](/docs/de/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout) hart zurückgesetzt, wenn die nächste Sitzung startet, daher überleben ungespeicherte verfolgte Änderungen auf keinem Pfad. Typische Verwendungen sind das Pushen eines Snapshot-Branches von ungespeicherten Änderungen, das Archivieren von Protokollen oder das Ausgeben eines Sitzungs-beendeten Ereignisses an Ihre eigenen Systeme.

Der Hook wird bei jedem Sitzungsende ausgelöst, bei dem ein untergeordneter Prozess gespawnt wurde, unabhängig von der Ursache; die `CLAUDE_RUNNER_EXIT_REASON`-Werte unten zählen die Fälle auf. Er kann nicht ausgelöst werden, wenn der Runner abrupt beendet wird, wie eine VM-Preemption oder ein Stromausfall; wenn Sie Garantien gegen abrupte Beendigung benötigen, snapshotten Sie regelmäßig von innerhalb der Sitzung mit einem Claude Code `PostToolUse`-Hook stattdessen. Der Runner setzt:

| Variable                           | Beschreibung                                                                                                                                                                                                                       |
| :--------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_RUNNER_SESSION_ID`         | Sitzungs-ID in der getaggten Form `session_...`                                                                                                                                                                                    |
| `CLAUDE_RUNNER_SESSION_UUID`       | Dieselbe Sitzungs-ID in kanonischer UUID-Form                                                                                                                                                                                      |
| `CLAUDE_RUNNER_EXIT_REASON`        | Wie die Sitzung endete; siehe die Werte unter der Tabelle                                                                                                                                                                          |
| `CLAUDE_RUNNER_WORKSPACE_PATHS`    | Doppelpunkt-getrennte absolute Pfade der Arbeitsbäume der Sitzung. Leer für Null-Repo-Sitzungen.                                                                                                                                   |
| `CLAUDE_RUNNER_DEBUG_LOG_PATH`     | Pfad zum Debug-Protokoll der Sitzung, noch auf der Festplatte während der Hook-Ausführung                                                                                                                                          |
| `CLAUDE_RUNNER_API_BASE_URL`       | Anthropic-API-Basis-URL für Sitzungs-bezogene Aufrufe                                                                                                                                                                              |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`    | Die Client-Oberfläche, die die Sitzung erstellt hat, wie `web_claude_ai`, `desktop_app` oder `ios`. Nicht gesetzt, wenn die Sitzung keine aufgezeichnete oder erkannte Oberfläche hat. Erfordert Claude Code v2.1.229 oder später. |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN` | Das Sitzungs-Zugangstoken für Sitzungs-bezogene API-Aufrufe                                                                                                                                                                        |

`CLAUDE_RUNNER_EXIT_REASON` nimmt einen von vier Werten an:

* `completed`: die Sitzung endete sauber. Der Claude Code-Prozess wurde normal beendet, oder die Sitzung wurde archiviert oder gelöscht, während sie noch lief.
* `failed`: Der Claude Code-Prozess ist abgestürzt, oder das Setup ist nach dem Start fehlgeschlagen.
* `interrupted`: Der Runner hat die Sitzung gestoppt. Er gab die Sitzung frei, um den Slot freizugeben, die Sitzung ist beim Startup abgelaufen, der Server hat die Sitzung von diesem Runner verschoben, der Runner wurde geleert, oder die Sitzung hat sein [`--kill-session-after-min`](/docs/de/self-hosted-environments-reference#runner-cli-flags)-Limit überschritten.
* `abandoned`: reserviert für eine Sitzung, die ein anderer Runner beansprucht hat. Der Hook wird derzeit in diesem Fall nicht ausgelöst.

Die [Sitzungs-Lifecycle-Zähler](/docs/de/self-hosted-environments-reference#session-lifecycle-counter-semantics) zählen eine Freigabe, ein Startup-Timeout und einen Server-Umzug als `completed` statt `interrupted`, weil der Runner den Slot sauber zurückgegeben hat. Erwarten Sie diesen Unterschied, wenn Sie Hook-Quittungen mit den Zählern vergleichen.

Der Exit-Status des Hooks beeinflusst niemals das Sitzungsergebnis; ein Fehler wird protokolliert und ignoriert. Der Runner wartet bis zu `--post-session-hook-timeout-sec`, standardmäßig 60 Sekunden, bei jedem Sitzungsende einschließlich Runner-Shutdown. Dieses Beispiel speichert ungespeicherte Arbeit in einem Rettungs-Branch:

```bash theme={null}
#!/usr/bin/env bash
set -u
IFS=':'
# Pin-Konfiguration, die die Sitzung in der Checkout-Datei `.git/config` gepflanzt haben könnte:
# -c-Overrides schlagen Repo-lokale Einstellungen, blockieren Sitzungs-geschriebene fsmonitor,
# Hook-Pfad und gpg-program-Konfiguration von der Ausführung von Code mit den Hook-Berechtigungen.
# Repo-lokale credential.helper, core.sshCommand und pushurl
# gelten immer noch; wenn der Hook Anmeldedaten hält, die die Sitzung nicht hatte, pin die
# Push-URL und Helper auch (siehe die Notiz unter dem Skript).
g() { git -c core.fsmonitor=false -c core.hooksPath=/dev/null \
        -c commit.gpgsign=false "$@"; }
for ws in $CLAUDE_RUNNER_WORKSPACE_PATHS; do
  cd "$ws" 2>/dev/null || continue
  [ -z "$(g status --porcelain 2>/dev/null)" ] && continue
  g add -A
  g commit -q -m "runner snapshot: $CLAUDE_RUNNER_SESSION_ID ($CLAUDE_RUNNER_EXIT_REASON)" || continue
  g push -q origin "HEAD:refs/heads/rescue/$CLAUDE_RUNNER_SESSION_ID" || true
done
```

Der Hook pusht mit den Git-Anmeldedaten, die in seiner eigenen Umgebung auf dem Runner-Host verfügbar sind. Unter der [Keine-Anmeldedaten-im-Image-Haltung](/docs/de/self-hosted-environments-deploy#configure-git), einschließlich wenn der integrierte Klon durch den Anthropic-Git-Proxy geht, gibt es keine, daher prägen Sie eine kurzlebige Push-Anmeldedaten innerhalb des Hooks, bevor Sie pushen: Tauschen Sie das Sitzungs-Token, das der Hook in `CLAUDE_CODE_SESSION_ACCESS_TOKEN` erhält, mit Ihrem eigenen Token-Dienst aus, überprüfen Sie es wie [Sitzungsidentität überprüfen](/docs/de/self-hosted-environments-identity) beschreibt. Wenn der Hook eine Anmeldedaten hält, die die Sitzung nicht hatte, pinnen Sie auch, wohin er pusht: Ersetzen Sie `origin` durch eine vom Operator bereitgestellte URL und übergeben Sie `-c credential.helper=` plus Ihren eigenen Helper, damit Repo-lokale Konfiguration, die die Sitzung geschrieben hat, den berechtigten Push nicht umleiten kann.

<h4 id="hook-timing-when-the-runner-releases-a-session">
  Hook-Timing, wenn der Runner eine Sitzung freigibt
</h4>

Eine freigegebene Sitzung kann auf einem anderen Runner fortgesetzt werden. Auf einem Runner mit v2.1.236 oder später entscheidet, was die Sitzung bei der Freigabe tat, ob sie fortgesetzt werden kann, bevor dieser Hook endet:

* **Idle nach einer Runde oder Timeout beim Startup**: Der Runner stoppt das Kind und führt diesen Hook bis zum Ende aus. Erst dann gibt er die Sitzung frei. Eine Benutzernachricht, die gesendet wird, während der Hook ausgeführt wird, kann die Sitzung nicht auf einem anderen Runner fortsetzen, bevor der Hook endet.
* **Warten auf die Antwort des Benutzers auf eine Eingabeaufforderung, wie eine Berechtigungsaufforderung**: Der Runner gibt die Sitzung zuerst frei, dann führt er diesen Hook aus. Eine Benutzernachricht, die gesendet wird, während der Hook ausgeführt wird, kann die Sitzung auf einem anderen Runner fortsetzen, bevor der Hook endet.

Dies gilt, wenn der Runner eine Sitzung freigibt: beim Idle-Timeout, zur [`--retire-at`](/docs/de/self-hosted-environments-reference#runner-cli-flags)-Zeit, und, auf einem Runner mit v2.1.260 oder später, beim [`--kill-session-after-min`](/docs/de/self-hosted-environments-reference#runner-cli-flags)-Limit einer Sitzung. Eine Sitzung, deren Runde beendet ist und die nur Hintergrundaufgaben hält, zählt hier als Idle. Vor v2.1.236 gab der Runner die Sitzung zuerst frei und führte dann diesen Hook in beiden Fällen aus.

Während eines `SIGTERM`-Drains hält der Runner das Sitzungs-Lease, bis der Hook endet; siehe [Shutdown-Timing](/docs/de/self-hosted-environments-deploy#shutdown-timing).

<h3 id="command">
  command
</h3>

Wird einmal pro Sitzung nach dem Checkout anstelle des integrierten Kind-Spawns ausgeführt. Der Hook erhält dieselbe Umgebung wie ein [Wrapper-Skript](#wrapper-scripts) und sollte auf dieselbe Weise in `"$CLAUDE_RUNNER_CLAUDE_BIN"` `exec` ausführen. Verwenden Sie den `command`-Hook, um alle Anpassungen in einem Hooks-Verzeichnis zu halten; verwenden Sie `--exec-path`, wenn der Wrapper anderswo lebt. Wenn `--exec-path` auch gesetzt ist, hat das Flag Vorrang und der `command`-Hook wird ignoriert.

Führen Sie immer die eigene Binärdatei des Runners aus, anstatt ein PATH-aufgelöstes `claude`; andernfalls besiegen Sie [Versions-Pinning](/docs/de/self-hosted-environments-deploy#pin-the-version).

<h2 id="on-demand-runners">
  On-Demand-Runner
</h2>

Anstatt eine feste Fleet zu betreiben, können Sie einen Runner pro Sitzung starten. Der Orchestrator ist ein separater, zustandsloser Unterbefehl, der Anthropic nach Spawn-Anfragen abfragt, eine pro Sitzung, die in der Warteschlange mit keinem verfügbaren Runner steht, und führt Ihren `spawn-runner`-Hook für jeden aus. Ihr Hook sendet eine Workload an Ihre Plattform: einen Kubernetes Job, eine EC2-Instanz, einen Nomad-Dispatch.

On-Demand-Runner verbessern die Anmeldedaten-Hygiene. Bei einer festen Fleet lebt das Umgebungsgeheimnis auf jedem Runner-Host, das ist derselbe Host, der Benutzersitzungen ausführt. Mit dem Orchestrator bleibt das Umgebungsgeheimnis nur auf dem Orchestrator-Host, der niemals Benutzercode ausführt; jeder gespawnte Runner erhält eine einmalige Arbeitsorder, die genau einen Runner registriert und dann abläuft.

Um den Orchestrator zu starten, übergeben Sie das Umgebungsgeheimnis und ein Hooks-Verzeichnis, das ein ausführbares `spawn-runner`-Skript enthält:

```bash theme={null}
claude self-hosted-runner orchestrator \
  --environment-secret-file /etc/claude/environment-secret \
  --hooks-dir /etc/claude/hooks
```

Der Orchestrator behält keinen Zustand zwischen Abfragen, daher können Sie zwei oder mehr Replikas gegen dieselbe Umgebung für Verfügbarkeit ausführen. Jede Spawn-Anfrage wird serverseitig von genau einer Replik beansprucht. Alle Replikas müssen denselben `--expected-spawn-seconds`-Wert verwenden; siehe den [Hook-Vertrag](#the-spawn-runner-hook).

<h3 id="the-spawn-runner-hook">
  Der spawn-runner-Hook
</h3>

Der Orchestrator führt `${hooks-dir}/spawn-runner` einmal pro Spawn-Anfrage aus. Der Hook muss Arbeit asynchron einreichen, ohne auf den Runner-Boot zu warten, und innerhalb von `--hook-timeout`, standardmäßig 60 Sekunden, zurückkehren. Der Hook erhält:

| Variable                              | Beschreibung                                                                                                                                                                                                                                                                                                                                                             |
| :------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_RUNNER_WORK_ORDER_FILE`       | Pfad zu einer Temp-Datei, die das signierte Arbeitsorder-JWT enthält, das der neue Runner registriert. Gelöscht nach dem Hook-Exit. Protokollieren Sie nicht den Inhalt der Datei.                                                                                                                                                                                       |
| `CLAUDE_RUNNER_ORDER_ID`              | Undurchsichtiger Idempotenz-Schlüssel, eindeutig pro Spawn-Anfrage und sicher für Kubernetes-Ressourcennamen. Verwenden Sie ihn als Dedup-Schlüssel Ihres Provisioners.                                                                                                                                                                                                  |
| `CLAUDE_RUNNER_SESSION_ID`            | Die Sitzung, für die diese Anfrage bestimmt ist. Leer für Pre-Warming-Anfragen, die einen Standby-Runner im Voraus starten, bevor eine bestimmte Sitzung, wenn [`--min-idle`](/docs/de/self-hosted-environments-reference#orchestrator-cli-flags) gesetzt ist, daher nehmen Sie nicht an, dass die Variable gesetzt ist.                                                      |
| `CLAUDE_RUNNER_SESSION_UUID`          | Dieselbe Sitzungs-ID in kanonischer UUID-Form. Leer für Pre-Warming-Anfragen.                                                                                                                                                                                                                                                                                            |
| `CLAUDE_RUNNER_ATTEMPT`               | Wie viele Spawn-Anfragen diese Sitzung hatte. `0` für Pre-Warming-Anfragen.                                                                                                                                                                                                                                                                                              |
| `CLAUDE_RUNNER_ORDER_SERVER_TIME`     | Server-Zeit aus dem HTTP-`Date`-Header der Poll-Antwort. Wenn der Hook das Arbeitsorder-JWT `exp` überprüft, vergleichen Sie gegen diesen Wert anstelle der lokalen Uhr, um Skew zu tolerieren. Leer, wenn das Gateway den Header weggelassen hat.                                                                                                                       |
| `CLAUDE_RUNNER_POOL_ID`               | Die ID der Umgebung, der der neue Runner beitreten sollte, in der Form `ccpool_...`                                                                                                                                                                                                                                                                                      |
| `CLAUDE_RUNNER_ACCOUNT_ID`            | Getaggte ID des Kontos, das die Sitzung in die Warteschlange eingereiht hat, für Pro-Konto-Routing, Kontingent oder Chargeback. Leer, wenn nicht verfügbar, und immer leer für Claude Tag-Kanal-Sitzungen, die kein Konto einreiht.                                                                                                                                      |
| `CLAUDE_RUNNER_ACCOUNT_EMAIL`         | E-Mail des Kontos, das die Sitzung in die Warteschlange eingereiht hat. Leer, wenn nicht verfügbar. Behandeln Sie die E-Mail als personenbezogene Informationen und protokollieren Sie sie nicht.                                                                                                                                                                        |
| `CLAUDE_RUNNER_PRIMARY_REPO_URL`      | URL der ersten Git-Quelle der Sitzung, für Routing zu einem Runner mit diesem Repository pre-warmed. Leer, wenn die Sitzung keine Git-Quellen hat.                                                                                                                                                                                                                       |
| `CLAUDE_RUNNER_PRIMARY_REPO_REVISION` | Revision der ersten Git-Quelle der Sitzung: Branch, SHA oder Tag. Leer, wenn nicht angegeben.                                                                                                                                                                                                                                                                            |
| `CLAUDE_RUNNER_REPO_SOURCES`          | JSON-Array von `{url, revision}` für alle Git-Quellen der Sitzung, für Hooks, die auf einem sekundären Repository routen. Leer, wenn es keine Quellen gibt.                                                                                                                                                                                                              |
| `CLAUDE_RUNNER_CORRELATION_ID`        | Die Korrelations-ID, die bei der Sitzungserstellung bereitgestellt wurde, echoed zurück, damit der Hook diese Arbeitsorder der Anfrage zuordnen kann, die die Sitzung erstellt hat. Leer, wenn die Sitzung keine hat.                                                                                                                                                    |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`       | Die Client-Oberfläche, die die Sitzung erstellt hat, wie `web_claude_ai`, `desktop_app`, `ios` oder `scheduled_trigger`, für Adoptionsanalysen. Nicht gesetzt, wenn die Sitzung keine aufgezeichnete oder erkannte Oberfläche hat, und für Pre-Warming-Anfragen; überprüfen Sie sie mit `[ -n "${CLAUDE_RUNNER_CLIENT_PLATFORM:-}" ]`, was unter `set -u` sicher bleibt. |

Der gespawnte Runner registriert sich mit der Arbeitsorder anstelle des Umgebungsgeheimnisses:

* **Starten Sie ihn mit der Arbeitsorder**: Zeigen Sie [`--environment-secret-file`](/docs/de/self-hosted-environments-reference#runner-cli-flags) auf eine Datei, die das Arbeitsorder-JWT enthält, oder setzen Sie `SELF_HOSTED_RUNNER_ENVIRONMENT_SECRET` auf den JWT-Wert.
* **Kopieren Sie das JWT, bevor der Hook endet**: Der Orchestrator löscht die Arbeitsorder-Datei nach dem Hook-Exit, daher kopieren Sie das JWT in die Workload, die Sie einreichen, wie ein Kubernetes Secret auf dem gespawten Job, anstatt den Dateipfad durchzuleiten.
* **Verwenden Sie `--capacity 1` auf gespawten Runnern**: Eine Sitzungs-gebundene Arbeitsorder registriert genau einen Runner, der an diese Sitzung gebunden ist, daher fügt eine höhere Kapazität Slots hinzu, die niemals Arbeit erhalten, und der Runner protokolliert eine Warnung beim Startup.
* **Pre-Warming-Arbeitsorder registrieren ungebunden**: Der Standby-Runner ist nicht an eine Sitzung gebunden und beansprucht in der Warteschlange befindliche Arbeit wie ein Fixed-Fleet-Runner.

Der Vertrag hat vier Provisioner-agnostische Regeln:

1. **Seien Sie idempotent auf `CLAUDE_RUNNER_ORDER_ID`.** Neulieferung derselben Anfrage muss höchstens einen Runner spawnen. Leiten Sie einen deterministischen Ressourcennamen von der ID ab und lassen Sie Ihre Plattform das Duplikat ablehnen.
2. **Versuchen Sie nicht, die Workload erneut zu versuchen.** Eine Order-ID bedeutet höchstens eine erstellte Workload. Wenn sich der Runner nie registriert, fordert Anthropic nach `--expected-spawn-seconds` mit einer frischen Order-ID erneut an.
3. **Verwenden Sie den Exit-Code-Vertrag.** Exit 0 bedeutet eingereicht. Exit 1 bedeutet wiederholbarer Fehler; die Sitzung sichert sich ab und wird erneut angeboten. Exit 2 oder höher bedeutet nicht wiederholbar; die Sitzung wird blockiert, bis ein [Owner](/docs/de/cloud-environments#organization-shared-environments) auf der Registerkarte **Aktivität** der Umgebung **Erneut versuchen** auswählt. Bei Nicht-Null-Exit erscheint das Ende des Stderr des Hooks dort als Fehlergrund, daher schreiben Sie den umsetzbaren Fehler auf stderr und niemals Geheimnisse. Für eine Pre-Warming-Anfrage gibt es keine Sitzung zum Fehlschlag: Der Orchestrator protokolliert einen Nicht-Null-Exit lokal nur, und der Server fordert den Spawn nach dem Lease erneut an.
4. **Setzen Sie `--expected-spawn-seconds` auf mindestens Ihre p99-Boot-Zeit.** Dies ist das serverseitige Lease. Alle Orchestrator-Replikas müssen denselben Wert verwenden.

Alles, was der Hook auf stdout oder stderr schreibt, erscheint im Protokoll des Orchestrators mit automatisch redigierten Anmeldedaten. Wenn Sitzungen in der Warteschlange bleiben, überprüfen Sie den `/healthz`-Body des Orchestrators auf Warteschlangen-Zählungen, öffnen Sie dann die Registerkarte **Aktivität** Ihrer Umgebung auf der [**Cloud-Umgebungen**-Administratorseite](https://claude.ai/admin-settings/cloud-environments): Erweitern Sie eine fehlgeschlagene Sitzung dort für ihren Spawn-Fehler und wählen Sie **Erneut versuchen**, um sie erneut anzufordern.

<h2 id="mcp-servers">
  MCP-Server
</h2>

Um [MCP-Server](/docs/de/mcp) in jeder Sitzung verfügbar zu machen, fügen Sie sie zur Image-Build-Zeit mit demselben `claude mcp add`-Befehl hinzu, der bei einer Desktop-Installation verwendet wird. Wenn Ihr Runner ein bloßer Prozess anstelle eines Containers ist, führen Sie denselben Befehl als Runner-Benutzer auf dem Host aus, dann starten Sie den Runner neu: Er liest Host-Konfiguration einmal beim Startup. Das Flag `--scope user` ist erforderlich; der Standard-Local-Bereich schreibt unter einem Pro-Verzeichnis-Schlüssel, den der Runner nicht in Sitzungen seeded. Zum Beispiel in Ihrem Dockerfile:

```dockerfile theme={null}
RUN claude mcp add --scope user sidecar -- /usr/local/bin/mcp-sidecar
RUN claude mcp add --scope user --transport http internal http://mcp-gateway.svc.cluster.local:8080
```

Der Runner snapshott die Host-Konfiguration einmal beim Startup. Der Snapshot erfasst den `mcpServers`-Schlüssel aus der `.claude.json` des Hosts, die neben statt innerhalb von `~/.claude/` lebt, und der Runner seeded nur diesen Schlüssel in die isolierte Konfiguration jeder Sitzung; Kontostatus und Projekthistorie werden gelöscht. Um zu bestätigen, dass die Server Sitzungen erreicht haben, starten Sie eine Sitzung in der Umgebung und bitten Sie Claude, seine MCP-Tools aufzulisten; der Runner protokolliert auch eine Startup-Warnung für jeden erfassten Eintrag, dessen `type` er nicht erkennt, und löscht den Eintrag, daher können Sie sehen, warum dieser Server in Sitzungen fehlt. Wenn `SELF_HOSTED_RUNNER_HOST_CONFIG_DIR` gesetzt ist, liest der Runner `.claude.json` stattdessen aus diesem Verzeichnis, daher zeigt das Setzen der Variablen auf ein leeres Verzeichnis auch das MCP-Seeding deaktiviert.

Claude Code lädt auch MCP-Server aus anderen Quellen:

* Die Enterprise-Scope [verwaltete MCP-Datei](/docs/de/managed-mcp) unter ihrem Standard-Systempfad: `/etc/claude-code/managed-mcp.json` auf Linux-Runner-Hosts, `/Library/Application Support/ClaudeCode/managed-mcp.json` auf macOS-Hosts. Verwenden Sie sie für abgesperrte Fleets, bei denen nur Administrator-aufgelistete Server geladen werden dürfen. Siehe [exklusive Kontrolle mit managed-mcp.json](/docs/de/managed-mcp#exclusive-control-with-managed-mcp-json) für die Vorrangregeln. Wenn diese Datei auf dem Runner-Host ist, überspringt Claude Code die MCP-Server, die Anthropics Kontrolleben an eine Sitzung liefert, einschließlich claude.ai-Konnektoren, und nennt sie in einer Warnung auf stderr des Sitzungs-Kindes, die der Runner auf der `debug`-Protokollebene aufzeichnet. Vor v2.1.229 beendeten diese Sitzungen beim Startup mit `You cannot dynamically configure MCP servers when an enterprise MCP config is present`.
* Der [`managedMcpServers`](/docs/de/settings-reference#managedmcpservers)-Schlüssel in [verwalteten Einstellungen](/docs/de/managed-settings) auf dem Runner-Host: bietet HTTP- und SSE-Server, ohne exklusive Kontrolle zu übernehmen, daher werden Server aus den anderen Quellen weiterhin geladen. Erfordert Claude Code v2.1.259 oder später.
* `<repo>/.mcp.json`: Projekt-Scope. Committen Sie die Datei zum Repository; ihre Server werden in Cloud-Sitzungen automatisch genehmigt.

Wenn die Konnektoren-Lieferung für Ihre Organisation aktiviert ist, liefert Anthropics Kontrolleben die Konnektoren, die Sie auf claude.ai konfiguriert haben, an interaktiv erstellte Sitzungen durch serverseitig bereitgestellte MCP-Konfiguration, geroutet durch `api.anthropic.com`. Sitzungen, die programmgesteuert erstellt wurden, wie [CLI-Dispatches](/docs/de/self-hosted-environments-testing#run-the-test-loop), erhalten keine Konnektoren-Lieferung; geben Sie ihnen MCP-Server durch eine der anderen Quellen, die dieser Abschnitt auflistet, stattdessen. Das Token des Kindes trägt keinen Bereich zum direkten Abrufen von Konnektoren, daher versucht das Kind diesen Abruf nicht selbst; die Lieferung ist serverseitig gesteuert.

`settings.json` trägt keine MCP-Server-Definitionen, und es gibt kein Top-Level-Feld `mcpServers` im Settings-Schema. Geben Sie in verwalteten Einstellungen Server mit dem [`managedMcpServers`](/docs/de/settings-reference#managedmcpservers)-Schlüssel stattdessen an.

Sitzungen erben die Umgebung des Runners, daher setzen Sie [`ENABLE_TOOL_SEARCH`](/docs/de/mcp#scale-with-mcp-tool-search) dort, um die MCP-Tool-Suche für jede Sitzung zu steuern, die ein Runner spawnt; die MCP-Seite behandelt die Werte.

<h2 id="prompt-sessions-to-push-their-work">
  Fordern Sie Sitzungen auf, ihre Arbeit zu pushen
</h2>

Anthropic-gehostete Sitzungen führen einen [`Stop`-Hook](/docs/de/hooks#stop) aus, den Claude Code-Hook, der ausgeführt wird, wenn Claude fertig mit der Antwort ist, der Claude auffordert, seine Arbeit zu committen und zu pushen. Der Runner installiert keinen. Ohne ihn hinterlässt eine Sitzung, die mit ungespeicherten Änderungen endet, diese Arbeit nur auf der Festplatte des Runners, und die Schaltfläche **PR erstellen** in claude.ai/code bleibt inaktiv, bis der Branch auf dem Remote existiert.

Die Referenzimplementierung unten hat zwei Teile. Führen Sie den Settings-Block in `~/.claude/settings.json` auf dem Runner-Host zusammen, den der Runner in jede Sitzung seeded, und speichern Sie das Skript als `~/.claude/hooks/stop-hook-nudge.sh` auf dem Runner-Host und machen Sie es ausführbar:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "timeout": 10,
            "command": "\"$CLAUDE_CONFIG_DIR/hooks/stop-hook-nudge.sh\""
          }
        ]
      }
    ]
  }
}
```

```sh theme={null}
#!/bin/sh
# Stop-Hook-Referenzimplementierung für selbstgehostete Runner.
#
# Nudgt Claude einmal pro Runde, wenn das Projektverzeichnis ungespeicherte
# Änderungen ODER ungepushte Commits hat, damit Arbeit nicht verloren geht, wenn eine Idle-Sitzung
# freigegeben wird und damit die Schaltfläche "PR erstellen" auf claude.ai/code leuchtet.
#
# Runner-Ebene (keine Repo-Änderungen): Legen Sie diese Datei auf dem Runner-Host unter ~/.claude/hooks/ ab und
# führen Sie den begleitenden Stop-Hook-Settings-Block
# in ~/.claude/settings.json zusammen — der Runner seeded beide in jede Sitzung.
# Repo-Ebene-Alternative: Committen Sie zu <repo>/.claude/hooks/ und ändern Sie den
# settings.json-Befehlspfad zu $CLAUDE_PROJECT_DIR/.claude/hooks/.
#
# stdin: Hook-JSON-Payload (siehe https://code.claude.com/docs/en/hooks)
# stdout: {"decision":"block","reason":"..."} zum Nudgen oder nichts zum Zulassen des Stops.

# Re-Entry-Guard: Das Harness setzt stop_hook_active=true, wenn der Stop-Hook erneut aufgerufen wird
# nach einem Block. Bail, damit wir nur einmal pro Runde nudgen. Das
# Harness gibt kompaktes JSON aus (kein Leerzeichen nach dem Doppelpunkt), das dieses
# Muster nutzt; verwenden Sie jq, wenn Sie eine Whitespace-tolerante Überprüfung benötigen.
in=$(cat)
case "$in" in *'"stop_hook_active":true'*) exit 0 ;; esac

d="$CLAUDE_PROJECT_DIR"

# Kein Git-Repo → nichts zum Nudgen.
git -C "$d" rev-parse --git-dir >/dev/null 2>&1 || exit 0

# Kein Remote → "zum Remote pushen" ist nicht erfüllbar; bail.
[ -z "$(git -C "$d" remote 2>/dev/null)" ] && exit 0

# Ungespeicherte Änderungen (staged, unstaged oder untracked). Schließen Sie .claude/ aus
# vollständig — Operator-gekeimte Einstellungen und CLI-geschriebener Laufzeitzustand
# (Scheduler-Sperre, Worktrees, Routine-Zustand) leben dort und keiner ist
# "ungespeicherte Arbeit", die das Modell pushen muss.
s=$(git -C "$d" status --porcelain -- . ':(exclude).claude/' 2>/dev/null)
if [ -n "$s" ]; then
  printf '{"decision":"block","reason":"There are uncommitted changes in the repository. Please commit and push these changes to the remote branch."}'
  exit 0
fi

# Ungepushte Commits. Zählen Sie Commits auf HEAD, die von keinem
# Remote-Tracking-Ref oder FETCH_HEAD erreichbar sind. Dies funktioniert einheitlich für:
#   - init+fetch-Checkouts (Runner-Standard: nur FETCH_HEAD existiert)
#   - Clone-basierte Checkouts (origin/* existieren)
#   - der Runner-Standard: Das Kind startet auf dem Outcome-
#     Branch der Sitzung, den der Runner nach dem Checkout erstellt
#   - Detached HEAD, wenn ein benutzerdefiniertes Setup diese Branch-Erstellung überspringt
# Ohne Referenzpunkt überhaupt (nie abgerufen), bleiben Sie still, anstatt
# falsch-positiv auf einer Read-Only-Runde.
base=""
git -C "$d" rev-parse --verify -q FETCH_HEAD >/dev/null && base="FETCH_HEAD"
if [ -z "$base" ] && [ -z "$(git -C "$d" for-each-ref --count=1 refs/remotes/origin 2>/dev/null)" ]; then
  exit 0
fi
# shellcheck disable=SC2086  # $base ist entweder "" oder "FETCH_HEAD", beabsichtigter Word-Split
unpushed=$(git -C "$d" rev-list HEAD --not $base --remotes=origin --count 2>/dev/null) || unpushed=0
if [ "$unpushed" -gt 0 ]; then
  branch=$(git -C "$d" symbolic-ref --short -q HEAD)
  if [ -n "$branch" ]; then
    # $branch ist Angreifer-beeinflusst — git-check-ref-format(1) erlaubt `"`
    # in Ref-Namen. `\` ist verboten (Regel 10), aber trotzdem als billiger
    # Defense-in-Depth escaped.
    # Escape JSON-Metazeichen vor der Interpolation in die handgebaute
    # Payload, damit ein Branch wie x","continue":false keine Schlüssel in
    # das Hook-Output-JSON injizieren kann, das das Harness parst. $unpushed ist sicher — das
    # -gt-Guard oben lehnt alles ab, das keine einfache Ganzzahl ist.
    branch_esc=$(printf '%s' "$branch" | sed 's/\\/\\\\/g; s/"/\\"/g')
    printf '{"decision":"block","reason":"There are %s unpushed commit(s) on branch '\''%s'\''. Please push these changes to the remote repository."}' "$unpushed" "$branch_esc"
  else
    printf '{"decision":"block","reason":"There are %s unpushed commit(s) on a detached HEAD. Please create a branch and push it to the remote repository."}' "$unpushed"
  fi
  exit 0
fi

exit 0
```

Der Hook fordert Claude auf, vor dem Sitzungsende zu committen und zu pushen, und bleibt still, wenn das Verzeichnis kein Git-Repository ist oder keinen Remote hat.

<h2 id="permissions-and-tool-approval">
  Berechtigungen und Tool-Genehmigung
</h2>

Eine selbstgehostete Sitzung hat kein angehängtes Terminal, daher stellt eine unbeantwortete Berechtigungsaufforderung die Runde stall, bis der Benutzer in der UI antwortet. Anthropics Kontrolleben sendet die Tool-Liste und Berechtigungsregeln jeder Sitzung mit der Workload; die Standardkonfiguration genehmigt Routine-Tool-Aufrufe vor, einschließlich `Bash`, und Cloud-Sitzungen [genehmigen Datei-Edits unabhängig vom Modus vor](/docs/de/permission-modes#switch-permission-modes). Ein Aufruf, den nichts genehmigt, fordert durch die Sitzungs-UI auf.

<Note>
  Pinnen Sie nur Auto-Modus auf einer Umgebung, deren Sitzungs-Container mit [Standard-Deny-Netzwerk-Egress](/docs/de/self-hosted-environments-deploy#default-deny-egress) und dem Rest des [Härtungsabschnitts](/docs/de/self-hosted-environments-deploy#harden-your-deployment) laufen. Routine-Tool-Aufrufe, einschließlich `Bash`-Netzwerk-Anfragen, werden ohne einen Menschen in der Schleife sowohl auf dem Standard-Pre-Approved-Tool-Set als auch im Auto-Modus ausgeführt, daher ist die Netzwerk-Grenze das, was begrenzt, wo diese Aufrufe erreichen können.
</Note>

Um Aufforderungen unabhängig davon, was die Kontrolleben sendet, auf ein Minimum zu halten, pinnen Sie [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) von Ihrem Wrapper-Skript oder [`command`-Hook](#command). Auto-Modus lässt Sitzungen ohne Routine-Berechtigungsaufforderungen ausgeführt werden: Ein separates Klassifizierungsmodell überprüft Aktionen, bevor sie ausgeführt werden, und blockiert die, die es ablehnt, und explizite Ask-Regeln erzwingen immer noch eine Aufforderung; die Berechtigungsmodi-Seite behandelt, was der Klassifizierer überprüft. Der Runner hängt serverseitig berechnete Flags an, bevor er den Wrapper aufruft, und für Single-Value-Flags wie `--permission-mode` ehrt der Parser das letzte Vorkommen, daher überschreibt ein Flag, das Sie nach `"$@"` anhängen, den serverseitig gesendeten Wert:

```bash theme={null}
#!/bin/bash
exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@" --permission-mode auto
```

Um stattdessen spezifische Tools vorab zu genehmigen, hängen Sie `--allowed-tools` mit Ihren Regeln an, zum Beispiel `--allowed-tools "Bash(bazel *) Bash(yarn *) mcp__internal__*"`. Listen-Flags wie `--allowed-tools` und `--disallowed-tools` sammeln sich über Vorkommen an, anstatt zu überschreiben, daher gelten Ihre Regeln zusätzlich zu allen Regeln, die die Kontrolleben sendet. Um zu verengen, hängen Sie `--disallowed-tools` an, das Tools verweigert, auch wenn eine andere Regel sie zulässt.

<h3 id="how-each-session’s-config-is-assembled">
  Wie die Konfiguration jeder Sitzung zusammengestellt wird
</h3>

Der Runner gibt jeder Sitzung sein eigenes Konfigurationsverzeichnis, gekeimed aus einem Snapshot von `~/.claude/` des Hosts, den der Runner einmal beim Startup erfasst: `settings.json`, `CLAUDE.md`, Hooks, Agenten, Befehle und Skills in Ihrem Runner-Image gelten für jede Sitzung als Benutzer-Ebenen-Baseline. Wenn Sie die Konfiguration auf einem laufenden Host ändern, wird die Änderung erst nach einem Runner-Neustart wirksam. Setzen Sie `SELF_HOSTED_RUNNER_HOST_CONFIG_DIR`, um von einem anderen Pfad zu keimen, oder zeigen Sie auf ein leeres Verzeichnis, um das Keimen zu deaktivieren.

Repository-committete `.claude/settings.json` schichtet sich als Projekt-Einstellungen oben auf. Sitzungen lesen auch [`managed-settings.json`](/docs/de/settings#where-settings-live) vom Standard-Systempfad in Ihrem Runner-Image. Ob seine Schlüssel neben [serverseitig verwalteten Einstellungen](/docs/de/server-managed-settings) gelten, folgt [wie Claude Code verwaltete Quellen kombiniert](/docs/de/managed-settings#how-claude-code-combines-managed-sources): Standardmäßig ignorieren Sitzungen, wenn Ihre Organisation serverseitig verwaltete Schlüssel liefert, die Runner-Image-Datei, abgesehen von den [Schlüsseln, die Claude Code von jeder Admin-Quelle liest](/docs/de/managed-settings#keys-read-from-every-admin-source), wie der `env`-Block, die Sandbox-Sperren, die Sandbox-Binärpfade und `forceRemoteSettingsRefresh`. Siehe [Einstellungs-Vorrang](/docs/de/settings#settings-precedence).

Wenn Anthropics Kontrolleben eine Sitzung mit [Claude Code-Hooks](/docs/de/hooks) versorgt, installiert der Runner sie neben, nicht über, Ihrer eigenen Konfiguration. Erfordert Claude Code v2.1.229 oder später.

* **Wo sie landen**: Der Runner schreibt jedes bereitgestellte Hook-Skript in ein reserviertes `hooks/.ccr-launcher/`-Unterverzeichnis des Konfigurationsverzeichnisses der Sitzung und registriert die Skripte in einer separaten Einstellungsdatei, die er mit `--settings` an die Sitzung übergibt, wobei die gekeimte `settings.json` und Ihre eigenen Skripte bei `hooks/<name>` unberührt bleiben. Der Runner erstellt das reservierte Unterverzeichnis für jede Sitzung neu und keimed keinen Host-Inhalt bei `~/.claude/hooks/.ccr-launcher/` in Sitzungen.
* **Wer sie verfasst**: Die Kontrolleben füllt die Skripte aus festen Konstanten in ihrer eigenen Bereitstellung, niemals aus Pro-Sitzungs- oder Drittanbieter-Input.
* **Was sie immer noch regiert**: Hooks, die durch `--settings` bereitgestellt werden, treten in die gewöhnliche zusammengeführte Hook-Konfiguration ein, nicht in die verwaltete Ebene, daher gelten Ihre verwalteten Einstellungen immer noch. `disableAllHooks` deaktiviert sie, und sie gehören nicht zu den Kategorien, die [`allowManagedHooksOnly`](/docs/de/settings-reference#allowmanagedhooksonly) geladen hält.

<h3 id="repository-committed-permission-rules">
  Repository-committete Berechtigungsregeln
</h3>

Legen Sie keinen bloßen `"Edit"`-, `"Write"`- oder `"NotebookEdit"`-Eintrag in eine Repository-committete `permissions.allow`. Eine bloße Datei-Tool-Regel passt das Tool unabhängig vom Pfad, gewährt Schreibvorgänge überall auf dem Host anstelle nur des Arbeitsbereichs, daher flaggt der Schreib-Bereich-Confine-Guard des Runners die Sitzung; mit [`--confine-repo-settings enforce`](/docs/de/self-hosted-environments-reference#runner-cli-flags) weigert er sich, die Sitzung zu spawnen, anstatt zu protokollieren und fortzufahren. Siehe den [Härtungsabschnitt](/docs/de/self-hosted-environments-deploy#harden-your-deployment).

Ein Repository benötigt überhaupt keine Datei-Tool-Regel: Cloud-Sitzungen [genehmigen Datei-Edits unabhängig vom Modus vor](/docs/de/permission-modes#switch-permission-modes). Wenn Sie eine Regel committen, beschränken Sie sie auf den Arbeitsbereich, wie `"Edit(/**)"`; ein einzelner führender Schrägstrich ist relativ zum Projekt-Root, das der Arbeitsbereich der Sitzung ist. Bloße Datei-Tool-Regeln sind in der Operator-Host-Ebenen-`settings.json` in Ordnung, da diese Datei nicht Repository-committiert ist.

Ein `defaultMode` von `auto` wird nur aus der Image-weiten oder Benutzer-Ebenen-Einstellungsdatei geehrt, daher kann ein ausgechecktes Repository sich nicht selbst Auto-Modus gewähren. Für welche Modi Cloud-Sitzungen akzeptieren und die vollständige Regel-Syntax, siehe [Berechtigungsmodi](/docs/de/permission-modes).

<h2 id="what’s-next">
  Nächste Schritte
</h2>

* [Referenz](/docs/de/self-hosted-environments-reference): jedes CLI-Flag, jede Umgebungsvariable und jede Metrik
* [Sitzungsidentität überprüfen](/docs/de/self-hosted-environments-identity): Validieren Sie das Sitzungs-Token von Diensten außerhalb des Runners
