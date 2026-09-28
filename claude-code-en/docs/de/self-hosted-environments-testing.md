> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Self-Hosted-Umgebungen end-to-end testen

> Überprüfen Sie ein selbstgehostetes Runner-Image aus CI: Starten Sie eine Sitzung mit der CLI, lesen Sie Claudes Antworten über einen Stop-Hook und schreiben Sie die vollständige Schleife.

<Note>
  Self-Hosted-Umgebungen befinden sich in der öffentlichen Beta auf Team- und Enterprise-Plänen; [Verfügbarkeit und Einschränkungen](/docs/de/self-hosted-environments#availability-and-limitations) behandelt den Aktivierungspfad. Diese Seite ist das CI-Test-Rezept; siehe [Schnellstart](/docs/de/self-hosted-environments-quickstart) für Setup und [In Produktion bereitstellen](/docs/de/self-hosted-environments-deploy) für die Fleet-Rezepte.
</Note>

In einer [Self-Hosted-Umgebung](/docs/de/self-hosted-environments) werden Claude Code [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) auf einem Runner-Image ausgeführt, das Sie erstellen und verwalten. Bevor Sie ein neues Image in Ihrer Produktionsumgebung bereitstellen, führen Sie eine vollständige Sitzung gegen eine Test-Umgebung aus einem Skript aus: erstellen Sie eine Sitzung, lesen Sie Claudes Antwort, senden Sie eine Nachfrage und lesen Sie auch diese Antwort. Dies ist die Form eines CI-Smoke-Tests, der Ihr Runner-Image, Git-Zugriff und alle benutzerdefinierten Tools überprüft, bevor Sie eine Änderung durchführen.

Dieses Rezept setzt voraus, dass Sie bereits [eine Umgebung und einen Runner eingerichtet haben](/docs/de/self-hosted-environments-quickstart#set-up-an-environment-and-runner), und dass Ihr CI-Job den Runner-Prozess auf demselben Host wie das Test-Skript startet, das natürliche Setup zum Testen eines neuen Runner-Images. Ein Stop-Hook, den Sie auf dem Runner installieren, schreibt die endgültige Antwort jeder Runde in eine lokale Datei, und das Skript liest sie von dort, sodass die einzigen Aufrufe der Anthropic API die beiden Dispatches selbst sind. Wenn sich Ihre Test-Runner auf separater Infrastruktur befinden, siehe [Remote-Test-Runner](#remote-test-runners).

<h2 id="install-the-capture-hook-on-your-test-runner">
  Installieren Sie den Capture-Hook auf Ihrem Test-Runner
</h2>

Das Zurücklesen funktioniert über einen Claude Code [Stop-Hook](/docs/de/hooks#stop): Wenn Claude eine Runde beendet, erhält der Hook die endgültige Assistenten-Nachricht als `last_assistant_message` in seinem stdin-JSON und hängt sie an `$E2E_REPLY_DIR/<session_id>.txt` an. Installieren Sie ihn auf die gleiche Weise wie den [commit-nudge Stop-Hook](/docs/de/self-hosted-environments-configuration#prompt-sessions-to-push-their-work), auf dem `~/.claude/` des Runner-Hosts, das der Runner in jede Sitzung einspeist.

<h3 id="save-the-hook-files">
  Speichern Sie die Hook-Dateien
</h3>

Speichern Sie die beiden folgenden Dateien auf dem Runner-Host:

* Der Einstellungsblock: Zusammenführen in `~/.claude/settings.json` auf dem Runner-Host
* Das Skript: Speichern als `~/.claude/hooks/e2e-stop-hook-capture.sh` auf dem Runner-Host und machen Sie es ausführbar

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "timeout": 10,
            "command": "\"$CLAUDE_CONFIG_DIR/hooks/e2e-stop-hook-capture.sh\""
          }
        ]
      }
    ]
  }
}
```

```sh theme={null}
#!/bin/sh
# Stop hook for testing a self-hosted environment end to end: writes each
# turn's final assistant reply to $E2E_REPLY_DIR/<session_id>.txt so a
# co-located test driver can read it without calling the Anthropic API.
# Install on the TEST runner only. Requires jq.

# No-op unless the driver is listening. Never fail the turn.
[ -n "${E2E_REPLY_DIR:-}" ] && [ -d "$E2E_REPLY_DIR" ] || exit 0

# CLAUDE_CODE_REMOTE_SESSION_ID is exported in cse_... form; the session
# id the dispatch CLI prints is in session_... form. Same id, different
# prefix.
sid=$(printf '%s' "${CLAUDE_CODE_REMOTE_SESSION_ID:-}" | sed 's/^cse_/session_/')
[ -n "$sid" ] || exit 0

# last_assistant_message is absent when the final assistant turn had no
# text, such as a tool-use-only turn. The `// empty` filter makes that a
# zero-byte write rather than the literal string "null".
jq -r '.last_assistant_message // empty' >> "$E2E_REPLY_DIR/$sid.txt" 2>/dev/null
exit 0
```

<h3 id="before-you-start-the-runner">
  Bevor Sie den Runner starten
</h3>

Der Hook hat diese Anforderungen:

* Installieren Sie ihn, bevor Sie den Runner starten. Der Runner erstellt eine Momentaufnahme von `~/.claude/` einmal beim Start, sodass ein Hook, der zu einem laufenden Runner hinzugefügt wird, erst nach einem Neustart wirksam wird.
* Exportieren Sie `E2E_REPLY_DIR` zum Runner-Prozess. Der Hook ist ein No-Op, wenn die Variable nicht gesetzt ist oder das Verzeichnis nicht existiert. Setzen Sie sie daher überall dort, wo Sie den Runner starten, z. B. in der systemd-Unit, Pod-Spezifikation oder CI-Schritt. Das Test-Skript unten benötigt es auch.

Installieren Sie diesen Hook nur auf Runnern, die Ihre Test-Umgebung bedienen. Er schreibt die endgültige Antwort jeder Sitzung auf die Festplatte, wenn `E2E_REPLY_DIR` existiert, was auf einem Wegwerf-CI-Runner harmlos ist, aber nicht in ein Produktionsumgebungs-Runner-Image gehört, in dem die Variable versehentlich gesetzt werden könnte.

<h2 id="run-the-test-loop">
  Führen Sie die Test-Schleife aus
</h2>

Die Dispatch-Flags `--environment` und `--ref` erfordern Claude Code v2.1.224 oder später auf dem Computer, auf dem das Skript ausgeführt wird, die gleiche Untergrenze wie der Runner selbst. Mit dem Hook an Ort und Stelle und einem auf diesem Host gestarteten Runner führt das Test-Skript folgende Schritte aus:

1. Erstellt eine Sitzung in der Test-Umgebung mit `claude -p "<prompt>" --environment <environment-id> --output-format json`, ausgeführt aus einem Git-Checkout, damit die CLI das Repository aus dem `origin`-Remote automatisch erkennen kann. Das optionale `--ref <branch>` basiert auf dem Checkout der Sitzung auf einer benannten Referenz statt auf lokalem HEAD. Der Befehl erstellt die Sitzung, gibt eine Zeile JSON mit `session_id` aus und beendet sich, ohne auf Claudes Antwort zu warten.
2. Wartet darauf, dass die Antwort in `$E2E_REPLY_DIR/<session_id>.txt` erscheint, geschrieben vom Stop-Hook auf dem Runner, sobald die Runde abgeschlossen ist.
3. Sendet eine Nachverfolgung mit `claude -p "<message>" --cloud <session_id> --output-format json` (siehe [Senden Sie eine Nachverfolgungsnachricht an eine laufende Sitzung](/docs/de/claude-code-on-the-web#send-follow-ups-from-the-cli)), die ein Benutzerereignis in der vorhandenen Sitzung bereitstellt und beendet sich.
4. Wartet auf die Antwort der Nachverfolgung auf die gleiche Weise wie Schritt 2.

<h3 id="environment-dispatch-behavior">
  `--environment` Dispatch-Verhalten
</h3>

Claude Code erstellt die Sitzung, gibt die Sitzungs-ID und einen Link dazu aus und beendet sich.

Das Flag hat Vorrang vor der [`remote.defaultEnvironmentId`](/docs/de/settings-reference#remote-defaultenvironmentid)-Einstellung. Es unterstützt nicht `--output-format stream-json` und kann nicht mit Flags kombiniert werden, die eine Sitzung fortsetzen, anhängen oder vorkonfigurieren, wie z. B. `--resume`, `--continue`, `--teleport`, `--session-id` oder `--init-only`. `--cloud` wird mit einer Sitzungs-ID oder URL abgelehnt und in nicht-interaktiven Läufen, wenn es eine Beschreibung trägt. Ein bloßes `--cloud` wird als abwesend behandelt. Von einem Terminal aus können Sie die Aufgabe statt eines positionalen Prompts als `--cloud`-Beschreibung übergeben.

<h2 id="example-script">
  Beispielskript
</h2>

Das folgende Skript führt die vollständige Schleife gegen `$CLAUDE_TEST_ENVIRONMENT_ID`, die `ccpool_...`-ID Ihrer Test-Umgebung, angezeigt im Detaildialog der Umgebung auf der Admin-Seite oder zurückgegeben durch den [create-environment-Aufruf](#create-a-dedicated-test-environment), aus und behauptet eine Sentinel-Phrase in jeder Antwort. Führen Sie es aus einem Git-Checkout des Repositorys aus, in dem die Sitzung arbeiten soll, nachdem Sie einen Runner auf diesem Host mit dem installierten Capture-Hook und exportiertem `E2E_REPLY_DIR` gestartet haben.

```bash theme={null}
#!/usr/bin/env bash
# End-to-end test against a self-hosted environment, using Stop-hook read-back.
# Prereqs: `claude auth login` has been run on this machine (see "Authenticate
# from CI" below); jq is installed; CLAUDE_TEST_ENVIRONMENT_ID names an
# environment whose runner is the one on this host, with the capture hook
# installed and E2E_REPLY_DIR in its environment.

set -euo pipefail

: "${CLAUDE_TEST_ENVIRONMENT_ID:=${CLAUDE_TEST_POOL_ID:-}}"  # CLAUDE_TEST_POOL_ID is the legacy spelling
: "${CLAUDE_TEST_ENVIRONMENT_ID:?set CLAUDE_TEST_ENVIRONMENT_ID to a ccpool_... id served by a runner on this host}"
: "${E2E_REPLY_DIR:?set E2E_REPLY_DIR to the directory the Stop hook on your test runner writes to, and export it to the runner process}"
: "${TEST_REPO_REF:=main}"

[ -d "$E2E_REPLY_DIR" ] || {
  echo "FAIL: E2E_REPLY_DIR ($E2E_REPLY_DIR) does not exist. The Stop hook on the runner needs it." >&2
  exit 1
}

# Waits until $E2E_REPLY_DIR/<session_id>.txt contains $2, or fails after
# 90 seconds. Tune the timeout to your environment's cold-start time. The
# file is written by the Stop hook on the runner.
await_reply() {
  local expect="$2" f="$E2E_REPLY_DIR/$1.txt"
  local deadline=$(($(date +%s) + 90))
  while :; do
    if [ -f "$f" ] && grep -qF -- "$expect" "$f"; then
      return
    fi
    [ "$(date +%s)" -lt "$deadline" ] || {
      echo "FAIL: '$expect' not in $f within 90s. The Stop hook on the runner did not write it." >&2
      echo "-- $E2E_REPLY_DIR contents --" >&2; ls -la "$E2E_REPLY_DIR" >&2
      [ -f "$f" ] && { echo "-- $f --" >&2; cat "$f" >&2; }
      exit 1
    }
    sleep 1
  done
}

# 1. Create the session on the test environment. Run from a git checkout
# so the CLI can auto-detect the repo. --ref pins the checkout to a named
# ref regardless of local HEAD.
TURN1="e2e-probe-$(date +%s)-$$: say exactly 'ok: custom tools are reachable' and nothing else"
EXPECT1="ok: custom tools are reachable"
create_json=$(claude -p "$TURN1" --environment "$CLAUDE_TEST_ENVIRONMENT_ID" \
  --ref "$TEST_REPO_REF" --output-format json)
echo "create: $create_json"
SESSION_ID=$(jq -er '.session_id' <<<"$create_json")

# 2. Wait for the turn-1 reply.
await_reply "$SESSION_ID" "$EXPECT1"
echo "turn-1 reply ok"

# 3. Post a follow-up via the CLI.
TURN2="e2e-probe-followup-$(date +%s): say exactly 'ok: follow-up delivered' and nothing else"
EXPECT2="ok: follow-up delivered"
followup_json=$(claude -p "$TURN2" --cloud "$SESSION_ID" --output-format json)
echo "followup: $followup_json"
jq -e '.ok == true' <<<"$followup_json" >/dev/null

# 4. Wait for the turn-2 reply.
await_reply "$SESSION_ID" "$EXPECT2"
echo "turn-2 reply ok"

echo "PASS: test-environment round-trip (session $SESSION_ID)"
```

Ersetzen Sie die `TURN1`/`TURN2`-Prompts und `EXPECT1`/`EXPECT2`-Sentinels durch alles, was Ihr Setup ausübt, z. B. Claude auffordern, eines Ihrer benutzerdefinierten MCP-Tools auszuführen und seine Ausgabe zu behaupten.

<h2 id="remote-test-runners">
  Remote-Test-Runner
</h2>

Wenn sich Ihre Test-Runner auf separater Infrastruktur befinden, z. B. eine persistente Kubernetes-Fleet, mit der Ihr CI-Job kein Dateisystem teilen kann, tauschen Sie den Dateischreibvorgang im Stop-Hook gegen einen POST an einen Endpunkt aus, auf den Ihr Driver lauscht:

```sh theme={null}
#!/bin/sh
# Variant of the capture hook for runners on separate infrastructure.
# Set E2E_REPLY_URL on the runner to an endpoint the driver controls.
[ -n "${E2E_REPLY_URL:-}" ] || exit 0
sid=$(printf '%s' "${CLAUDE_CODE_REMOTE_SESSION_ID:-}" | sed 's/^cse_/session_/')
[ -n "$sid" ] || exit 0
jq -r '.last_assistant_message // empty' | \
  curl -fsS -X POST --data-binary @- "$E2E_REPLY_URL/$sid" >/dev/null 2>&1
exit 0
```

Führen Sie auf der Driver-Seite alles aus, das den POST akzeptiert und die Antwort hält, bis der Test sie anfordert, z. B. einen kleinen HTTP-Listener innerhalb des CI-Jobs oder einen Webhook-Receiver, den Sie bereits ausführen. Der Hook wird auf Ihrer Infrastruktur ausgeführt, sodass der Endpunkt nur von Ihren Runnern erreichbar sein muss.

<h2 id="authenticate-from-ci">
  Authentifizieren Sie sich von CI
</h2>

Sowohl `claude -p ... --environment` als auch `claude -p ... --cloud` authentifizieren sich mit einem claude.ai OAuth-Token; API-Schlüssel wie `sk-ant-xxxxx` werden für keinen der beiden Aufrufe akzeptiert. Zwei Ansätze machen ein Token in CI verfügbar.

<h3 id="long-lived-ci-host">
  Langlebiger CI-Host
</h3>

Führen Sie `claude auth login` einmal interaktiv auf dem Computer aus, auf dem das Skript ausgeführt wird, mit einem dedizierten Benutzerkonto für Automatisierung. Claude Code speichert das Token im OS-Keychain auf macOS oder in `~/.claude/.credentials.json` auf Linux und Windows. Auf einem macOS-Host, dessen Keychain nicht geschrieben werden kann, wie es typisch in einer SSH-Sitzung der Fall ist, in der der Login-Keychain gesperrt bleibt, speichert Claude Code das Token auch dort in `~/.claude/.credentials.json`. Siehe [Credential-Verwaltung](/docs/de/authentication#credential-management).

Die CLI aktualisiert das kurzlebige Zugriffstoken automatisch bei jeder Invokation, aber die zugrunde liegende Refresh-Token-Gewährung ist auf 30 Tage ab der ersten Anmeldung begrenzt, daher führen Sie `claude auth login` interaktiv auf diesem Host alle 30 Tage erneut aus.

<h3 id="ephemeral-ci-runners">
  Ephemere CI-Runner
</h3>

Es gibt heute kein langlebiges CI-Token dafür. Der Umfang, der Cloud-Sitzungskontrolle gewährt, `user:sessions:claude_code`, ist serverseitig auf 30 Tage begrenzt, sodass `claude setup-token`, das ein einjähriges Inference-only-Token prägt, es nicht abdeckt. Das [Umgebungsgeheimnis](/docs/de/self-hosted-environments-quickstart#set-up-an-environment-and-runner) wird auch nicht akzeptiert, da es nur einen Runner autorisiert, sich bei der Umgebung zu registrieren, nicht um Sitzungen zu erstellen.

Um eine gespeicherte Anmeldung auf einem ephemeren Runner bereitzustellen, setzen Sie [`CLAUDE_CODE_OAUTH_REFRESH_TOKEN` und `CLAUDE_CODE_OAUTH_SCOPES`](/docs/de/env-vars#variables), damit `claude auth login` das Token ohne Browser austauscht; die gleiche 30-Tage-Obergrenze gilt für die Refresh-Gewährung. Kontaktieren Sie Ihr Anthropic-Kontoteam, wenn Sie einen Machine-Identity-Pfad benötigen, der nicht an ein menschliches Konto gebunden ist.

<h2 id="create-a-dedicated-test-environment">
  Erstellen Sie eine dedizierte Test-Umgebung
</h2>

Erstellen und löschen Sie Umgebungen programmgesteuert, damit jeder CI-Lauf eine saubere erhält; der Runner, den Ihr CI-Job startet, registriert sich in der neuen Umgebung. Die unten aufgeführten Aufrufe zum Erstellen und Löschen sind die gleichen Endpunkte, die die **Cloud-Umgebungen**-Admin-Seite auf claude.ai verwendet, und sie erfordern den Header `anthropic-beta: ccr-byoc-2025-07-29`.

<h3 id="mint-the-admin-token">
  Prägen Sie das Admin-Token
</h3>

`$ADMIN_TOKEN` ist ein claude.ai OAuth-Zugriffstoken für ein Konto, das eine Owner-Rolle hält, geprägt auf die gleiche Weise wie [Authentifizieren Sie sich von CI](#authenticate-from-ci):

* **Prägen Sie es**: Führen Sie `claude auth login` mit einem Konto aus, das eine Owner-Rolle hält, und lesen Sie dann das aktuelle Zugriffstoken von überall dort, wo [Langlebiger CI-Host](#long-lived-ci-host) sagt, dass Claude Code es gespeichert hat.
* **Lesen Sie es bei jedem Lauf neu**: Die CLI rotiert das Zugriffstoken, und die gleiche 30-Tage-Refresh-Grant-Obergrenze gilt, speichern Sie also keine Kopie.
* **Übergeben Sie es über stdin**: wie das Beispiel, damit das Token nie in Curls Argumentliste oder Ihrem Build-Log landet.

<h3 id="create-the-environment">
  Erstellen Sie die Umgebung
</h3>

Erfassen Sie die Antwort, ohne sie auszugeben: `pool_secret` ist eine langlebige Anmeldeinformation, die Runner in der Umgebung registrieren kann. Speichern Sie sie als maskiertes CI-Geheimnis und geben Sie nur die Umgebungs-ID aus. Das `-H @-`-Formular, das das Token aus der Prozessliste hält, erfordert curl 7.55 oder später; älteres curl behandelt `@-` als wörtlichen Header und sendet die Anfrage ohne Autorisierung.

```bash theme={null}
create=$(curl -fsS -X POST -H @- \
  -H "anthropic-beta: ccr-byoc-2025-07-29" -H "anthropic-version: 2023-06-01" \
  -H "content-type: application/json" \
  -d '{"name":"ci-test-environment"}' \
  https://api.anthropic.com/v1/code/runners/self-hosted/pools \
  <<<"Authorization: Bearer $ADMIN_TOKEN")
ENVIRONMENT_ID=$(jq -er .pool.pool_id <<<"$create")
ENVIRONMENT_SECRET=$(jq -er .pool_secret <<<"$create")
```

Bis ein [Owner **Allow self-hosted environments**](/docs/de/self-hosted-environments#availability-and-limitations) für die Organisation aktiviert hat, schlägt der Aufruf mit einem `403` `permission_error` fehl, der `self-hosted runners are disabled by your organization's policy` liest.

Starten Sie einen Runner auf diesem Host mit `SELF_HOSTED_RUNNER_ENVIRONMENT_SECRET=$ENVIRONMENT_SECRET`, plus den Capture-Hook und `E2E_REPLY_DIR` pro [Installieren Sie den Capture-Hook](#install-the-capture-hook-on-your-test-runner), dann führen Sie das Test-Skript aus.

<h3 id="delete-the-environment">
  Löschen Sie die Umgebung
</h3>

Löschen Sie die Umgebung, wenn der Lauf beendet ist, damit jeder CI-Lauf sauber startet:

```bash theme={null}
curl -fsS -X DELETE -H @- \
  -H "anthropic-beta: ccr-byoc-2025-07-29" -H "anthropic-version: 2023-06-01" \
  "https://api.anthropic.com/v1/code/runners/self-hosted/pools/$ENVIRONMENT_ID" \
  <<<"Authorization: Bearer $ADMIN_TOKEN"
```
