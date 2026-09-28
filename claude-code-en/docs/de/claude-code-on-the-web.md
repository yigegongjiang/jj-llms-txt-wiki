> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code in der Cloud verwenden

> Führen Sie Claude Code-Sitzungen in der Cloud aus Ihrem Browser, Telefon, Desktop-App oder Terminal aus, verschieben Sie sie mit --cloud und --teleport, und beheben Sie Pull Requests automatisch.

<Note>
  Cloud-Sitzungen sind in Pro-, Max- und Team-Plänen sowie für Enterprise-Benutzer mit Premium-Sitzen oder Chat + Claude Code-Sitzen verfügbar.
</Note>

Eine Cloud-Sitzung ist eine Claude Code-Sitzung, die auf Cloud-Infrastruktur statt auf Ihrem Computer ausgeführt wird. Standardmäßig wird sie auf von Anthropic verwalteter Infrastruktur ausgeführt oder auf der [selbstgehosteten Umgebung](/docs/de/self-hosted-environments) Ihrer Organisation, wenn sie dorthin weitergeleitet wird. Die Sitzung läuft weiter, nachdem Sie Ihren Laptop schließen, und Sie können sie von jedem Gerät aus überprüfen oder steuern.

Sie können eine Cloud-Sitzung von einer dieser Oberflächen aus starten:

* **Browser**: [claude.ai/code](https://claude.ai/code), auch Claude Code im Web genannt
* **Mobil**: die Registerkarte **Code** in der [Claude-App](/docs/de/mobile)
* **Desktop-App**: wählen Sie **Cloud** statt **Lokal**, wenn Sie [eine Sitzung starten](/docs/de/desktop#run-long-running-tasks-in-the-cloud)
* **Terminal**: [`claude --cloud`](#from-terminal-to-cloud)
* **Routinen**: [geplante und ausgelöste Ausführungen](/docs/de/routines) werden jeweils als Cloud-Sitzung ausgeführt

Um Claude viele Cloud-Sitzungen für einen Arbeitskörper starten und verfolgen zu lassen, verwenden Sie ein [Projekt](/docs/de/claude-projects). Eine Sitzung in Ihrem Terminal, Ihrer IDE oder der Desktop-App mit **Lokal** ausgewählt wird stattdessen auf Ihrem eigenen Computer ausgeführt. Um eine dieser lokalen Sitzungen von Ihrem Telefon oder Browser aus zu steuern, verwenden Sie [Remote Control](/docs/de/remote-control).

<Tip>
  Neu bei Cloud-Sitzungen? Beginnen Sie mit [Erste Schritte](/docs/de/web-quickstart), um Ihr GitHub-Konto zu verbinden und Ihre erste Aufgabe einzureichen.
</Tip>

Diese Seite behandelt:

* [Cloud-Umgebungen](#cloud-environments): wo Sitzungen ausgeführt werden und wo Sie das konfigurieren
* [GitHub-Authentifizierungsoptionen](#github-authentication-options): zwei Möglichkeiten, GitHub zu verbinden
* [Aufgaben zwischen Terminal und Cloud verschieben](#move-tasks-between-terminal-and-cloud) mit `--cloud` und `--teleport`
* [Mit Sitzungen arbeiten](#work-with-sessions): Berechtigungsmodi, Überprüfung, Freigabe, Archivierung, Löschung
* [Auto-fix Pull Requests](#auto-fix-pull-requests): automatische Reaktion auf CI-Fehler und Review-Kommentare
* [Sicherheit und Isolation](#security-and-isolation): wie Sitzungen isoliert sind
* [Einschränkungen](#limitations): Ratenlimits und Plattformbeschränkungen

<h2 id="cloud-environments">
  Cloud-Umgebungen
</h2>

Jede Cloud-Sitzung wird in einer [Cloud-Umgebung](/docs/de/cloud-environments) ausgeführt, der gespeicherten Konfiguration, die Netzwerkzugriff, Umgebungsvariablen und Setup-Skripte steuert. Wenn Sie noch keine Umgebung haben, richtet das Onboarding eine **Standard**-Umgebung mit [**Vertrautem** Netzwerkzugriff](/docs/de/cloud-environments#access-levels) ein, entweder indem sie für Sie erstellt wird oder indem Sie aufgefordert werden, sie zu erstellen. Siehe [Die Standard-Umgebung](/docs/de/cloud-environments#the-default-environment), um zu erfahren, welches auf Ihrem Plan geschieht und wie Sitzungen eine Umgebung auswählen, wenn Sie mehr als eine haben.

Die gleichen Umgebungen gelten überall dort, wo Sie eine Cloud-Sitzung starten: im Web, im Terminal, [Claude Tag](https://claude.com/docs/claude-tag/overview), [Routines](/docs/de/routines) und den Mobile- und Desktop-Apps. Claude Tag-Kanal-Sitzungen verwenden nur Umgebungen auf Organisationsebene, entweder [gemeinsam genutzte Umgebungen](/docs/de/cloud-environments#organization-shared-environments) oder [selbstgehostete Umgebungen](/docs/de/self-hosted-environments).

Siehe [Cloud-Umgebungen konfigurieren](/docs/de/cloud-environments), um zu ändern, was eine Umgebung erlaubt, Variablen zu setzen oder ein Setup-Skript hinzuzufügen, und [Installierte Tools](/docs/de/cloud-environments#installed-tools) für das, was Sitzungen ohne Konfiguration enthalten.

<h2 id="github-authentication-options">
  GitHub-Authentifizierungsoptionen
</h2>

Cloud-Sitzungen benötigen Zugriff auf Ihre GitHub-Repositories, um Code zu klonen und Branches zu pushen. Sie können Zugriff auf zwei Arten gewähren:

| Methode          | Funktionsweise                                                                                             | Repositories, auf die Sitzungen zugreifen können                                                                  | Am besten für                                                              |
| :--------------- | :--------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------- |
| **GitHub App**   | Autorisieren Sie die Claude GitHub App während des [Web-Onboardings](/docs/de/web-quickstart)                   | Alle öffentlichen Repositories und private Repositories, auf denen die Claude GitHub App installiert ist          | Browser-Onboarding; Teams, die [Auto-fix](#auto-fix-pull-requests) möchten |
| **`/web-setup`** | Führen Sie `/web-setup` in Ihrem Terminal aus, um Ihr lokales `gh` CLI-Token an Ihr Claude-Konto zu senden | Alle Repositories, auf die Ihr `gh`-Token zugreifen kann, unabhängig davon, ob die App installiert ist oder nicht | Einzelne Entwickler, die bereits `gh` verwenden                            |

Die Installation der Claude GitHub App auf einem Repository ermöglicht auch [Auto-fix](#auto-fix-pull-requests) für Pull Requests darin.

Threads in einem [Projekt](/docs/de/claude-projects) benötigen die App auf jedem Repository, das sie klonen, unabhängig davon, welche Methode Sie zum Verbinden verwendet haben. Siehe [GitHub-Zugriff einrichten](/docs/de/claude-projects#set-up-github-access).

Informationen dazu, wie `/schedule` den Repository-Zugriff überprüft, bevor eine Routine erstellt wird, finden Sie unter [Repositories und Branch-Berechtigungen](/docs/de/routines#repositories-and-branch-permissions). Siehe [Vom Terminal verbinden](/docs/de/web-quickstart#connect-from-your-terminal) für die `/web-setup`-Anleitung, einschließlich dessen, was `/web-setup` speichert und wie Sie es entfernen.

Quick web setup ist eine Organisationseinstellung, die es Mitgliedern ermöglicht, GitHub mit `/web-setup` zu verbinden, überspringt die Claude GitHub App-Installationsaufforderung während des Browser-Onboardings und lässt das Browser-Onboarding die [**Standard**-Umgebung](/docs/de/cloud-environments#the-default-environment) für sie erstellen, anstatt das Umgebungsformular anzuzeigen. Bei Team- und Enterprise-Plänen ist es standardmäßig deaktiviert, was `/web-setup` verbirgt. Ein [Owner](/docs/de/server-managed-settings#access-control) aktiviert es mit dem **Quick web setup**-Umschalter unter [**Admin-Einstellungen > Claude Code**](https://claude.ai/admin-settings/claude-code).

<Note>
  Organisationen mit aktivierter [Zero Data Retention](/docs/de/zero-data-retention) können `/web-setup` oder andere Cloud-Sitzungsfunktionen nicht verwenden.
</Note>

<h2 id="move-tasks-between-terminal-and-cloud">
  Aufgaben zwischen Terminal und Cloud verschieben
</h2>

Diese Workflows erfordern die [Claude Code CLI](/docs/de/quickstart), die bei demselben claude.ai-Konto angemeldet ist. Sie können neue Cloud-Sitzungen von Ihrem Terminal aus starten oder Cloud-Sitzungen in Ihr Terminal ziehen, um lokal fortzufahren. Cloud-Sitzungen bleiben bestehen, auch wenn Sie Ihren Laptop schließen, und Sie können sie von überall aus überwachen, einschließlich der Claude Mobile-App.

<Note>
  Von der CLI ist die Sitzungsübergabe unidirektional: Sie können Cloud-Sitzungen mit `--teleport` in Ihr Terminal ziehen, aber Sie können keine vorhandene Terminal-Sitzung in die Cloud verschieben. Das Flag `--cloud` mit einer Aufgabenbeschreibung erstellt eine neue Cloud-Sitzung für Ihr aktuelles Repository; mit `-p` und einer Sitzungs-ID oder claude.ai/code-URL [reiht es stattdessen eine Nachricht in diese vorhandene Sitzung ein](/docs/de/claude-code-on-the-web#send-follow-ups-from-the-cli). Die [Desktop-App](/docs/de/desktop#continue-in-another-surface) bietet ein **Continue in**-Menü, das eine lokale Sitzung in die Cloud senden kann.
</Note>

<h3 id="from-terminal-to-cloud">
  Vom Terminal zur Cloud
</h3>

Starten Sie eine Cloud-Sitzung von der Befehlszeile mit dem Flag `--cloud`:

```bash theme={null}
claude --cloud "Fix the authentication bug in src/auth/login.ts"
```

Dies erstellt eine neue Cloud-Sitzung auf claude.ai. Die Cloud-VM klont das GitHub-Remote Ihres aktuellen Verzeichnisses bei Ihrem aktuellen Branch, nicht Ihren lokalen Checkout, daher pushen Sie zuerst, wenn Sie lokale Commits haben. Siehe [Senden Sie lokale Repositories ohne GitHub](#send-local-repositories-without-github) für die Fälle, in denen Claude Code Ihr lokales Repository hochlädt, anstatt es zu klonen.

`--cloud` funktioniert mit einem Repository auf einmal. Die Aufgabe wird in der Cloud ausgeführt, während Sie lokal weiterarbeiten. Die ältere Schreibweise `--remote` funktioniert immer noch als veralteter Alias für `--cloud`.

Während der Cloud-Container startet, zeigt die CLI eine Live-Checkliste von Setup-Schritten an, wie z. B. das Klonen des Repositories und das Ausführen Ihres [Setup-Skripts](/docs/de/cloud-environments#setup-scripts). Sie reiht Nachrichten ein, die Sie während der Bereitstellung eingeben, und sendet sie, sobald die Sitzung bereit ist.

<Note>
  `--cloud` erstellt Cloud-Sitzungen. `--remote-control` ist nicht verwandt: Es stellt eine lokale CLI-Sitzung zur Überwachung und Steuerung von claude.ai oder der Claude-App aus bereit. Siehe [Remote Control](/docs/de/remote-control).
</Note>

Öffnen Sie die Sitzung auf claude.ai oder der Claude Mobile-App, um den Fortschritt zu überprüfen oder direkt zu interagieren. Von dort aus können Sie Claude steuern, Feedback geben oder Fragen beantworten, genau wie in jedem anderen Gespräch.

Wenn Claude eine Frage stellt und die Sitzung untätig bleibt, können Sie immer noch antworten, wenn Sie zurückkommen, bis zur [Umgebungsablauf](#environment-expired), und die Sitzung wird von Ihrer Antwort aus fortgesetzt.

<h4 id="tips-for-cloud-tasks">
  Tipps für Cloud-Aufgaben
</h4>

**Planen Sie lokal, führen Sie in der Cloud aus**: Für komplexe Aufgaben starten Sie Claude im Plan Mode, um den Ansatz zu besprechen, und senden Sie dann die Arbeit in die Cloud:

```bash theme={null}
claude --permission-mode plan
```

Im Plan Mode liest Claude Dateien, führt Befehle aus, um zu erkunden, und schlägt einen Plan vor, ohne Quellcode zu bearbeiten. Sobald Sie mit dem Plan zufrieden sind, speichern Sie den Plan im Repo, committen und pushen Sie, damit die Cloud-VM ihn klonen kann. Dann starten Sie eine Cloud-Sitzung für autonome Ausführung:

```bash theme={null}
claude --cloud "Execute the migration plan in docs/migration-plan.md"
```

**Führen Sie Aufgaben parallel aus**: Jeder `--cloud`-Befehl erstellt seine eigene Cloud-Sitzung, die unabhängig ausgeführt wird. Sie können mehrere Aufgaben starten und sie werden alle gleichzeitig in separaten Sitzungen ausgeführt:

```bash theme={null}
claude --cloud "Fix the flaky test in auth.spec.ts"
claude --cloud "Update the API documentation"
claude --cloud "Refactor the logger to use structured output"
```

Wenn eine Sitzung abgeschlossen ist, können Sie einen PR aus claude.ai/code erstellen oder [die Sitzung teleportieren](#from-cloud-to-terminal), um lokal fortzufahren.

<h4 id="send-local-repositories-without-github">
  Senden Sie lokale Repositories ohne GitHub
</h4>

Wenn Sie `claude --cloud` aus einem Repository ausführen, das kein Git-Remote hat, oder aus einem github.com-Repository, auf dem die Claude GitHub App nicht installiert ist, bündelt Claude Code Ihr lokales Repository und lädt es direkt in die Cloud-Sitzung hoch. Dies gilt auch, wenn Sie GitHub mit `/web-setup` verbunden haben. Das Bündel enthält Ihre vollständige Repository-Historie über alle Branches hinweg, plus nicht committete Änderungen an verfolgten Dateien.

Auf macOS, Linux und WSL lässt Claude Code nicht committete Änderungen an Dateien, die wie Anmeldedaten oder Schlüssel benannt sind, aus dem Upload weg und benennt die Dateien, die es weggelassen hat. Dies umfasst `.env`-Dateien, Terraform `*.tfvars`-Dateien und Schlüsseldateien wie `id_rsa` und `*.pem`. Die Sitzung startet mit der committeten Version jeder Datei oder ohne die Datei, wenn keine committiert ist. In einem verknüpften Worktree, Submodul oder ähnlichem Layout lädt Claude Code diese Änderungen mit dem Rest hoch und benennt die Dateien, die es hochlädt.

Um ein Bündel hochzuladen, auch wenn Claude Code andernfalls vom Remote klonen würde, setzen Sie `CCR_FORCE_BUNDLE=1`:

```bash theme={null}
CCR_FORCE_BUNDLE=1 claude --cloud "Run the test suite and fix any failures"
```

Gebündelte Repositories müssen diese Limits erfüllen:

* Das Verzeichnis muss ein Git-Repository mit mindestens einem Commit sein
* Das gebündelte Repository muss unter 100 MB liegen. Größere Repositories fallen auf das Bündeln nur des aktuellen Branches zurück, dann auf einen einzelnen gequetschten Snapshot des Arbeitsbaums, und schlagen nur fehl, wenn der Snapshot immer noch zu groß ist
* Nicht verfolgte Dateien sind nicht enthalten; führen Sie `git add` auf Dateien aus, die die Cloud-Sitzung sehen soll
* Sitzungen, die aus einem Bündel erstellt wurden, können nur dann zurück zu einem GitHub-Remote pushen, wenn Ihre [GitHub-Verbindung](#github-authentication-options) Push-Zugriff auf dieses Repository hat

<h3 id="send-follow-ups-from-the-cli">
  Senden Sie Folgenachrichten von der CLI
</h3>

Sobald eine Cloud-Sitzung ausgeführt wird, überall wo sie ausgeführt wird, senden Sie ihr eine Folgenachricht von der `claude` CLI auf jeder Maschine, auf der Sie mit `claude auth login` angemeldet sind. Die CLI authentifiziert sich mit Ihren Anthropic-Kontoanmeldedaten und sendet keinen lokalen Sitzungsstatus, daher muss der Befehl nicht von der Maschine ausgeführt werden, die die Sitzung gestartet hat, und er ist in jeder Shell gleich, einschließlich PowerShell.

Der Befehl postet eine Nachricht und beendet sich:

```bash theme={null}
claude -p "your message" --cloud <session-id>
```

Die CLI reiht die Nachricht in die Sitzung ein und beendet sich, ohne auf eine Antwort zu warten. Verwenden Sie es, um eine lange laufende Sitzung zu steuern, den nächsten Schritt in die Warteschlange einzureihen, während der aktuelle noch läuft, oder senden Sie Folgenachrichten von einem [CI-Skript](/docs/de/self-hosted-environments-testing#run-the-test-loop). Sie können die Nachricht auch auf stdin pipen, anstatt sie als Argument zu übergeben: `echo "your message" | claude -p --cloud <session-id>`.

Für `<session-id>` übergeben Sie die bloße ID, wie `session_...` oder `cse_...`, oder die `claude.ai/code/<id>`-URL der Sitzung, mit oder ohne Schema oder Abfragezeichenfolge. Finden Sie die ID in Ihrer Sitzungsliste unter claude.ai/code.

<Note>
  `--cloud` erfordert ein Anthropic-Konto. Es ist nicht verfügbar, wenn Claude Code für Amazon Bedrock, Google Cloud's Agent Platform oder einen anderen Drittanbieter konfiguriert ist. Ein [LLM-Gateway](/docs/de/llm-gateway), das nur über `ANTHROPIC_BASE_URL` konfiguriert ist, zählt nicht als Drittanbieter für diese Überprüfung, aber Sie müssen sich immer noch mit `claude auth login` anmelden. Die `allow_remote_sessions`-Richtlinie Ihrer Organisation muss auch aktiviert sein. Ein Owner kann sie in den Claude Code-Admin-Einstellungen unter claude.ai/admin-settings/claude-code aktivieren.
</Note>

<h4 id="output-and-errors">
  Ausgabe und Fehler
</h4>

Bei Erfolg druckt der Befehl die Sitzungs-ID und einen Link zum Anzeigen der Sitzung:

```
Sent to cloud session.
Session ID: session_01DiUkqY2kzbUbDmW1w96rfi
View: https://claude.ai/code/session_01DiUkqY2kzbUbDmW1w96rfi?from=cli&m=0
```

Übergeben Sie `--output-format json` für ein maschinenlesbares Ergebnis: `{ok, session_id, url}` bei Erfolg oder `{ok: false, session_id, error}`, wenn der Send fehlschlägt, beispielsweise wenn die Sitzung fehlt oder archiviert ist. Konfigurationsfehler, wie ein nicht unterstützter Anbieter oder eine deaktivierte Organisationsrichtlinie, werden ohne JSON auf stderr gedruckt. `--output-format stream-json` wird nicht mit `--cloud <session-id>` unterstützt.

Die CLI stellt Fehlern das Präfix `Error: ` voran. Ein fehlgeschlagener Versand wird als `failed to send message to cloud session <id>: <reason>` umschlossen.

| Nachricht                                                                                                                   | Was es bedeutet                                                                                                                                                                                                                                                                                                                                                                |
| --------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Cloud sessions aren't available with <provider>. They run on Anthropic's infrastructure and require an Anthropic account.` | Claude Code ist für einen Drittanbieter konfiguriert. Die Nachricht benennt den Anbieter mit dem Label, das Ihre Konfiguration verwendet, wie `Amazon Bedrock` oder `Google Vertex AI`. Entfernen Sie die Konfiguration dieses Anbieters, beispielsweise durch Aufheben von `CLAUDE_CODE_USE_BEDROCK`, und melden Sie sich mit einem Anthropic-Konto an (`claude auth login`). |
| `Cloud sessions are disabled by your organization's policy. Contact your organization admin to enable them.`                | Die `allow_remote_sessions`-Organisationsrichtlinie ist deaktiviert.                                                                                                                                                                                                                                                                                                           |
| `Couldn't verify your organization's policy for cloud sessions. Check your network connection and try again.`               | Claude Code konnte Ihre Organisationsrichtlinie nicht abrufen, daher weigert es sich zu senden, anstatt anzunehmen, dass Cloud-Sitzungen erlaubt sind. Überprüfen Sie Ihre Netzwerkverbindung und versuchen Sie es erneut.                                                                                                                                                     |
| `Attaching to an existing cloud session is not enabled for your account.`                                                   | Sie haben `--cloud <session-id>` ohne `-p` ausgeführt. Senden Sie die Nachricht mit `claude -p "your message" --cloud <session-id>`.                                                                                                                                                                                                                                           |
| `Session not found: <id>`                                                                                                   | Die ID oder URL stimmt nicht mit einer Sitzung überein, auf die Sie zugreifen können. Überprüfen Sie sie anhand der claude.ai/code-URL der Sitzung.                                                                                                                                                                                                                            |
| `cloud session <id> is archived and cannot accept new messages`                                                             | Die Sitzung wurde archiviert. Starten Sie stattdessen eine neue Sitzung.                                                                                                                                                                                                                                                                                                       |

<h3 id="from-cloud-to-terminal">
  Vom Cloud zum Terminal
</h3>

Ziehen Sie eine Cloud-Sitzung in Ihr Terminal mit einer dieser Methoden:

* **Mit `--teleport`**: Führen Sie von der Befehlszeile `claude --teleport` für eine interaktive Sitzungsauswahl aus, oder `claude --teleport <session-id>`, um eine bestimmte Sitzung direkt fortzusetzen. Wenn Sie nicht committete Änderungen haben, werden Sie aufgefordert, diese zuerst zu stashen.
* **Mit `/teleport`**: Führen Sie innerhalb einer vorhandenen CLI-Sitzung `/teleport` oder `/tp` aus, um die gleiche Sitzungsauswahl zu öffnen, ohne Claude Code neu zu starten.
* **Von `/tasks`**: Führen Sie `/tasks` aus, um Ihre Hintergrund-Sitzungen zu sehen, drücken Sie dann `t`, um in eine zu teleportieren.
* **Von claude.ai/code**: Wählen Sie **Open in > Terminal** aus dem Sitzungsmenü, um einen Befehl zu kopieren, den Sie in Ihr Terminal einfügen können.
* **Von innerhalb der Cloud-Sitzung**: Geben Sie `/teleport` ein und Claude Code antwortet mit dem genauen `claude --teleport <session-id>`-Befehl für diese Sitzung, bereit zum Ausführen aus einem Checkout des Repositories. Erfordert Claude Code v2.1.223 oder später in der Umgebung der Sitzung.

Wenn Sie eine Sitzung teleportieren, überprüft Claude, dass Sie sich im richtigen Repository befinden, ruft den Branch aus der Cloud-Sitzung ab und checkt ihn aus, und lädt die vollständige Gesprächshistorie in Ihr Terminal. Das Terminal erhält seine eigene Kopie der Sitzung: neue Arbeit dort bleibt lokal und erscheint nicht in der Cloud-Sitzung auf claude.ai oder der Claude Mobile-App. Um vom Telefon aus weiter zu steuern, nachdem Sie teleportiert haben, starten Sie [`/remote-control`](/docs/de/remote-control) in der lokalen Sitzung.

`--teleport` unterscheidet sich von `--resume`. `--resume` öffnet ein Gespräch aus der lokalen Historie dieser Maschine und listet keine Cloud-Sitzungen auf; `--teleport` zieht eine Cloud-Sitzung und ihren Branch.

<h4 id="teleport-requirements">
  Teleport-Anforderungen
</h4>

Teleport überprüft diese Anforderungen, bevor eine Sitzung fortgesetzt wird. Wenn eine Anforderung nicht erfüllt ist, sehen Sie einen Fehler oder werden aufgefordert, das Problem zu beheben.

| Anforderung          | Details                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| -------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Sauberer Git-Status  | Ihr Arbeitsverzeichnis darf keine nicht committeten Änderungen haben. Teleport fordert Sie auf, Änderungen zu stashen, falls erforderlich.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Korrektes Repository | Sie müssen `--teleport` aus einem Checkout desselben Repositories ausführen, nicht aus einem Fork. Wenn Sie es aus einem Checkout eines anderen Repositories ausführen, zeigt Claude Code einen Fehler an, der sowohl das Repository der Sitzung als auch das Repository Ihres Checkouts benennt. Vor v2.1.219 nannte der Fehler das Repository Ihres Checkouts nicht. Wenn Claude Code Ihr Remote nicht in einen Hostnamen analysieren kann, beispielsweise einen SSH-Host-Alias wie `git@work:owner/repo.git`, fordert es Sie zur Bestätigung auf und akzeptiert den Checkout, wenn der Owner und der Repository-Name des Remote mit dem Repository der Sitzung übereinstimmen. |
| Branch verfügbar     | Der Branch aus der Cloud-Sitzung muss in das Remote gepusht worden sein. Teleport ruft ihn automatisch ab und checkt ihn aus.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| Gleiches Konto       | Sie müssen sich bei demselben claude.ai-Konto authentifizieren, das in der Cloud-Sitzung verwendet wurde.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |

<h4 id="teleport-is-unavailable">
  `--teleport` ist nicht verfügbar
</h4>

Teleport erfordert claude.ai-Abonnement-Authentifizierung. Wenn Sie sich über API-Schlüssel anmelden, führen Sie `/login` aus, um sich stattdessen mit Ihrem claude.ai-Konto anzumelden. Wenn der Fehler Ihren Anbieter benennt, sind Cloud-Sitzungen nicht über Drittanbieter verfügbar; siehe die [Fehlertabelle](#output-and-errors). Wenn Sie bereits über claude.ai angemeldet sind und `--teleport` immer noch nicht verfügbar ist, hat Ihre Organisation möglicherweise Cloud-Sitzungen deaktiviert.

<h2 id="work-with-sessions">
  Mit Sitzungen arbeiten
</h2>

Sitzungen werden in der Seitenleiste unter claude.ai/code angezeigt. Von dort aus können Sie Änderungen überprüfen, mit Teamkollegen teilen, abgeschlossene Arbeiten archivieren oder Sitzungen dauerhaft löschen.

<h3 id="take-back-a-queued-message">
  Eine eingereihte Nachricht zurücknehmen
</h3>

Wenn Sie eine Nachricht senden, während Claude arbeitet, wird die Nachricht eingereicht, bis Claude sie liest. Um eine eingereihte Nachricht zurückzunehmen, klicken Sie auf das ✕ darauf. Der Text kehrt in das Nachrichtenfeld zurück, damit Sie ihn bearbeiten oder etwas anderes senden können.

Wenn Claude die Nachricht bereits gelesen hat, bleibt sie in der Konversation.

<h3 id="manage-context">
  Kontext verwalten
</h3>

Cloud-Sitzungen unterstützen [integrierte Befehle](/docs/de/commands), die Textausgabe erzeugen. Befehle, die nur in der Terminal-Schnittstelle ausgeführt werden, wie `/plugin` oder `/resume`, sind nicht verfügbar. Befehle, die eine Auswahl oder ein Panel in der Terminal-Schnittstelle öffnen, verhalten sich in Cloud-Sitzungen unterschiedlich:

* **`/model`, `/effort`, `/color` und `/rename`**: Übergeben Sie den Wert als Argument, zum Beispiel `/model sonnet`, anstatt die Terminal-Auswahl oder den Schieberegler zu öffnen. Die Argumentformen erfordern Claude Code v2.1.205 oder später in der Umgebung der Sitzung und folgen den [Verfügbarkeitshinweisen](/docs/de/commands#all-commands) jedes Befehls.
* **`/fast`**: schaltet den [Fast-Modus](/docs/de/fast-mode#use-fast-mode-in-cloud-sessions) für die Sitzung um, wenn Fast-Modus [auf Ihrem Konto verfügbar ist](/docs/de/fast-mode#requirements). Erfordert Claude Code v2.1.271 oder später in der Umgebung der Sitzung.
* **`/config`**: In Ihrem Browser unter claude.ai/code öffnet dies den Claude Code-Bereich Ihrer Einstellungen, anstatt einen Wert zu setzen, und Text nach dem Befehl, einschließlich `key=value`, wird ignoriert. Um eine Einstellung für eine Cloud-Sitzung zu ändern, legen Sie eine [Umgebungsvariable](/docs/de/cloud-environments#set-environment-variables) in der Umgebung fest, oder committen Sie in einer Sitzung mit einem Repository den Schlüssel in die `.claude/settings.json` dieses Repositorys. [Einstellungen in Cloud-Sitzungen](/docs/de/settings#settings-in-cloud-sessions) listet auf, was jede Sitzung liest.

Für Kontextverwaltung speziell:

| Befehl     | Funktioniert in Cloud-Sitzungen | Notizen                                                                                                                         |
| :--------- | :------------------------------ | :------------------------------------------------------------------------------------------------------------------------------ |
| `/compact` | Ja                              | Fasst das Gespräch zusammen, um Kontext freizugeben. Akzeptiert optionale Fokus-Anweisungen wie `/compact keep the test output` |
| `/context` | Ja                              | Zeigt, was sich derzeit im Kontextfenster befindet                                                                              |
| `/clear`   | Nein                            | Starten Sie stattdessen eine neue Sitzung aus der Seitenleiste                                                                  |

Auto-Kompaktierung wird automatisch ausgeführt, wenn sich das Kontextfenster der Kapazität nähert. Cloud-Sitzungen setzen [`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`](/docs/de/env-vars) selbst, daher wird die Kompaktierung partway durch das [Auto-Compact-Fenster](/docs/de/model-config#set-the-auto-compact-window) ausgelöst, anstatt wenn das Fenster sich füllt. Dieser Wert überschreibt einen, den Sie in Ihren [Umgebungsvariablen](/docs/de/cloud-environments#set-environment-variables) hinzufügen, daher ändert das Hinzufügen der Variablen dort nicht, wann die Kompaktierung ausgelöst wird.

Um das Auto-Compact-Fenster stattdessen zu ändern, setzen Sie [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/de/env-vars) in Ihren Umgebungsvariablen, oder führen Sie [`/autocompact`](/docs/de/commands#all-commands) mit einer Token-Anzahl in einer Sitzung aus, in der die Variable nicht gesetzt ist.

[Subagents](/docs/de/sub-agents) funktionieren genauso wie lokal. Claude kann sie mit dem Agent-Tool spawnen, um Forschung oder parallele Arbeit in ein separates Kontextfenster auszulagern, um das Hauptgespräch leichter zu halten. Subagents, die in Ihrem Repo's `.claude/agents/` definiert sind, werden automatisch aufgegriffen.

[Agent-Teams](/docs/de/agent-teams) sind standardmäßig deaktiviert, können aber aktiviert werden, indem Sie `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` zu Ihren [Umgebungsvariablen](/docs/de/cloud-environments#set-environment-variables) hinzufügen.

<h3 id="permission-modes-in-cloud-sessions">
  Berechtigungsmodi in Cloud-Sitzungen
</h3>

Sie wählen den [Berechtigungsmodus](/docs/de/permission-modes) einer Cloud-Sitzung aus dem [Modus-Dropdown](/docs/de/permission-modes#switch-permission-modes), sowohl wenn Sie die Aufgabe erstellen als auch während die Sitzung läuft. Wenn Sie eine Sitzung erneut öffnen, deren von Anthropic gehostete [Umgebung abgelaufen ist](#environment-expired), oder eine Nachricht an eine Sitzung senden, die ein selbstgehosteter Runner [freigegeben hat, während sie untätig war](/docs/de/self-hosted-environments-reference#runner-cli-flags), setzt Claude Code die Sitzung im Berechtigungsmodus fort, in dem sie sich befand.

<h3 id="review-changes">
  Änderungen überprüfen
</h3>

Jede Sitzung zeigt einen Diff-Indikator mit hinzugefügten und entfernten Zeilen, wie `+42 -18`. Wählen Sie ihn, um die Diff-Ansicht zu öffnen, hinterlassen Sie Inline-Kommentare zu bestimmten Zeilen und senden Sie sie mit Ihrer nächsten Nachricht an Claude.

Die Diff-Ansicht vergleicht die Änderungen der Sitzung standardmäßig mit ihrem Basis-Branch. Um gegen einen anderen Branch im Repository zu vergleichen, wählen Sie **Compare against** und wählen Sie einen aus.

Claude Code berechnet diese Diffs, einschließlich der Pro-Datei-Diffs, die als Claude-Bearbeitungen angezeigt werden, aus rohem Git-Blob-Inhalt, daher gelten Diff-Treiber und `textconv`-Filter, die im Repository konfiguriert sind, nicht. Für eine Datei in einem Repository, das nicht einer der eigenen Checkouts der Sitzung ist, wie eine während der Sitzung im Workspace geklonte Datei, zeigt der Pro-Datei-Diff die Claude-Bearbeitung selbst anstelle eines Git-Vergleichs.

Siehe [Überprüfung und Iteration](/docs/de/web-quickstart#review-and-iterate) für die vollständige Anleitung, einschließlich PR-Erstellung. Um Claude den PR auf CI-Fehler und Review-Kommentare automatisch überwachen zu lassen, siehe [Auto-fix Pull Requests](#auto-fix-pull-requests).

<h3 id="share-sessions">
  Sitzungen teilen
</h3>

Um eine Sitzung zu teilen, schalten Sie ihre Sichtbarkeit gemäß den Kontotypen unten um. Danach teilen Sie den Sitzungslink wie gewohnt. Empfänger sehen den neuesten Status, wenn sie den Link öffnen, aber ihre Ansicht wird nicht in Echtzeit aktualisiert.

<h4 id="share-from-an-enterprise-or-team-account">
  Teilen von einem Enterprise- oder Team-Konto
</h4>

Für Enterprise- und Team-Konten sind die beiden Sichtbarkeitsoptionen **Private** und **Team**. Team-Sichtbarkeit macht die Sitzung für andere Mitglieder Ihrer claude.ai-Organisation sichtbar. [Claude in Slack](/docs/de/slack)-Sitzungen werden automatisch mit Team-Sichtbarkeit geteilt.

Die Überprüfung des Repository-Zugriffs ist standardmäßig aktiviert, basierend auf dem GitHub-Konto, das mit dem Konto des Empfängers verbunden ist. Der Anzeigename Ihres Kontos ist für alle Empfänger mit Zugriff sichtbar.

<h4 id="share-from-a-max-or-pro-account">
  Teilen von einem Max- oder Pro-Konto
</h4>

Für Max- und Pro-Konten sind die beiden Sichtbarkeitsoptionen **Private** und **Public**. Public-Sichtbarkeit macht die Sitzung für jeden Benutzer sichtbar, der bei claude.ai angemeldet ist.

Überprüfen Sie Ihre Sitzung auf sensible Inhalte, bevor Sie sie teilen. Sitzungen können Code und Anmeldedaten aus privaten GitHub-Repositories enthalten. Die Überprüfung des Repository-Zugriffs ist standardmäßig nicht aktiviert.

Um zu verlangen, dass Empfänger Repository-Zugriff haben, oder um Ihren Namen aus gemeinsamen Sitzungen auszublenden, gehen Sie zu [**Einstellungen > Claude Code > Freigabeeinstellungen**](https://claude.ai/settings/claude-code).

<h3 id="archive-sessions">
  Sitzungen archivieren
</h3>

Sie können Sitzungen archivieren, um Ihre Sitzungsliste organisiert zu halten. Archivierte Sitzungen sind in der Standard-Sitzungsliste ausgeblendet, können aber durch Filtern nach archivierten Sitzungen angezeigt werden.

Um eine Sitzung zu archivieren, bewegen Sie den Mauszeiger über die Sitzung in der Seitenleiste und wählen Sie das Archiv-Symbol.

<h3 id="delete-sessions">
  Sitzungen löschen
</h3>

Das Löschen einer Sitzung entfernt die Sitzung und ihre Daten dauerhaft. Diese Aktion kann nicht rückgängig gemacht werden. Sie können eine Sitzung auf zwei Arten löschen:

* **Von der Seitenleiste**: Filtern Sie nach archivierten Sitzungen, bewegen Sie dann den Mauszeiger über die Sitzung, die Sie löschen möchten, und wählen Sie das Lösch-Symbol
* **Vom Sitzungsmenü**: Öffnen Sie eine Sitzung, wählen Sie das Dropdown-Menü neben dem Sitzungstitel und wählen Sie **Löschen**

Sie werden aufgefordert, vor dem Löschen einer Sitzung zu bestätigen.

<h2 id="auto-fix-pull-requests">
  Auto-fix Pull Requests
</h2>

Claude kann einen Pull Request überwachen und automatisch auf CI-Fehler und Review-Kommentare reagieren. Claude abonniert GitHub-Aktivitäten auf dem PR, und wenn eine Überprüfung fehlschlägt oder ein Reviewer einen Kommentar hinterlässt, untersucht Claude das Problem und pusht eine Lösung, wenn eine klar ist.

<Note>
  Auto-fix erfordert, dass die Claude GitHub App auf Ihrem Repository installiert ist. Falls noch nicht geschehen, installieren Sie sie von der [GitHub App-Seite](https://github.com/apps/claude).
</Note>

Es gibt mehrere Möglichkeiten, Auto-fix zu aktivieren, je nachdem, woher der PR stammt und welches Gerät Sie verwenden:

* **PRs, die in einer Cloud-Sitzung erstellt wurden**: Öffnen Sie die Sitzung unter claude.ai/code, öffnen Sie die CI-Statusleiste und wählen Sie **Auto-fix**
* **Von Ihrem Terminal**: Führen Sie [`/autofix-pr`](/docs/de/commands) aus, während Sie auf dem PR's Branch sind. Claude Code erkennt den offenen PR mit `gh`, spawnt eine Cloud-Sitzung und aktiviert Auto-fix in einem Schritt
* **Von der Mobile-App**: Sagen Sie Claude, den PR zu auto-fixen, zum Beispiel „watch this PR and fix any CI failures or review comments"
* **Jeder vorhandene PR**: Fügen Sie die PR-URL in eine Sitzung ein und sagen Sie Claude, den PR zu auto-fixen

Auto-fix ist ein Pro-PR-Toggle. Um die Überwachung zu beenden, öffnen Sie die CI-Statusleiste in der Sitzung unter claude.ai/code und deaktivieren Sie den **Auto-fix**-Toggle, oder sagen Sie Claude, die Überwachung des PR zu beenden.

<h3 id="how-claude-responds-to-pr-activity">
  Wie Claude auf PR-Aktivität reagiert
</h3>

Wenn Auto-fix aktiv ist, empfängt Claude GitHub-Events für den PR, einschließlich neuer Review-Kommentare und CI-Check-Fehler. Für jedes Event untersucht Claude das Problem und entscheidet, wie vorgegangen wird:

* **Klare Fixes**: Wenn Claude sich einer Lösung sicher ist und sie nicht mit früheren Anweisungen in Konflikt steht, nimmt Claude die Änderung vor, pusht sie und erklärt, was getan wurde, in der Sitzung
* **Mehrdeutige Anfragen**: Wenn ein Reviewer-Kommentar auf mehrere Arten interpretiert werden könnte oder etwas architektonisch Bedeutsames betrifft, fragt Claude Sie, bevor er handelt
* **Doppelte oder keine Aktion erforderlich Events**: Wenn ein Event ein Duplikat ist oder keine Änderung erfordert, notiert Claude es in der Sitzung und fährt fort

GitHub gibt keinen Webhook aus, wenn der Basis-Branch voranschreitet und einen Merge-Konflikt erzeugt, daher kann Auto-fix nicht von selbst auf Konflikte reagieren. Um einen Konflikt zu beheben, öffnen Sie die Sitzung und bitten Sie Claude, einen Rebase durchzuführen.

Claude kann als Teil der Auflösung auf Review-Kommentar-Threads auf GitHub antworten. Diese Antworten werden mit Ihrem GitHub-Konto gepostet, sodass sie unter Ihrem Benutzernamen erscheinen, aber jede Antwort ist als von Claude Code stammend gekennzeichnet, damit Reviewer wissen, dass sie vom Agent geschrieben wurde und nicht direkt von Ihnen.

<Warning>
  Wenn Ihr Repository Kommentar-ausgelöste Automatisierung wie Atlantis, Terraform Cloud oder benutzerdefinierte GitHub Actions verwendet, die auf `issue_comment`-Events ausgeführt werden, beachten Sie, dass Claude auf Ihrem Behalf antworten kann, was diese Workflows auslösen kann. Überprüfen Sie die Automatisierung Ihres Repositories, bevor Sie Auto-fix aktivieren, und erwägen Sie, Auto-fix für Repositories zu deaktivieren, in denen ein PR-Kommentar Infrastruktur bereitstellen oder privilegierte Operationen ausführen kann.
</Warning>

<h2 id="security-and-isolation">
  Sicherheit und Isolation
</h2>

Jede Cloud-Sitzung ist von Ihrem Computer und von anderen Sitzungen durch mehrere Schichten getrennt:

* **Isolierte virtuelle Maschinen**: Jede Sitzung wird in einer isolierten, von Anthropic verwalteten VM ausgeführt. Sitzungen, die Ihre Organisation zu einer [selbstgehosteten Umgebung](/docs/de/self-hosted-environments) leitet, werden stattdessen auf Ihrer eigenen Infrastruktur ausgeführt, wo Isolation die Verantwortung Ihrer Bereitstellung ist
* <span id="default-allowed-domains" />**Netzwerkzugriffskontrolle**: In von Anthropic gehosteten Umgebungen ist der Netzwerkzugriff standardmäßig begrenzt und kann deaktiviert werden. Siehe [Netzwerkzugriff](/docs/de/cloud-environments#network-access) für die Zugriffsstufen, die [standardmäßig zulässigen Domänen](/docs/de/cloud-environments#default-allowed-domains) und den Datenverkehr, der nicht durch die Zulassungsliste läuft. In einer selbstgehosteten Umgebung beschränken Sie den Sitzungs-Egress an Ihrer eigenen Netzwerkgrenze. Wenn Claude Code mit deaktiviertem Netzwerkzugriff ausgeführt wird, kann Claude Code immer noch mit der Anthropic API kommunizieren, was möglicherweise ermöglicht, dass Daten die VM verlassen.
* **Schutz von Anmeldedaten**: In von Anthropic gehosteten Umgebungen befinden sich Git-Anmeldedaten und Signaturschlüssel außerhalb der Sandbox, und ein Proxy authentifiziert sich im Namen der Sitzung mit scoped Credentials. In einer selbstgehosteten Umgebung stellt Ihre Bereitstellung Git-Anmeldedaten bereit; siehe [Git konfigurieren](/docs/de/self-hosted-environments-deploy#configure-git)
* **API-Anmeldedaten**: In von Anthropic gehosteten Umgebungen auf Pro- und Max-Plänen bleiben Schlüssel, die Sie [zu einer Cloud-Umgebung hinzufügen](/docs/de/cloud-environments#add-api-credentials), auf die gleiche Weise außerhalb der Sandbox, angehängt an übereinstimmende Anfragen, nachdem sie die Sitzung verlassen. Eine selbstgehostete Umgebung hat keine API-Anmeldedaten, und Team- und Enterprise-Pläne haben sie noch nicht
* **Sichere Analyse**: Code wird in der isolierten Umgebung der Sitzung analysiert und geändert, bevor PRs erstellt werden

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

Für Runtime-API-Fehler, die im Gespräch angezeigt werden, wie `API Error: 500`, `529 Overloaded`, `429` oder `Prompt is too long`, siehe die [Fehlerreferenz](/docs/de/errors). Diese Fehler und ihre Lösungen werden mit der CLI und der Desktop-App geteilt. Die folgenden Abschnitte behandeln Probleme, die spezifisch für Cloud-Sitzungen sind.

<h3 id="session-creation-failed">
  Sitzungserstellung fehlgeschlagen
</h3>

Wenn eine neue Sitzung mit `Session creation failed` fehlschlägt oder bei der Bereitstellung steckenbleibt, konnte Claude Code eine VM für die Sitzung nicht zuordnen.

* Überprüfen Sie [status.claude.com](https://status.claude.com) auf Cloud-Sitzungs-Incidents
* Versuchen Sie es nach einer Minute erneut, da die Kapazität bei Bedarf bereitgestellt wird
* Bestätigen Sie, dass Ihre GitHub-Verbindung das Repository erreichen kann, indem Sie [Keine Repositories werden nach dem Verbinden von GitHub angezeigt](/docs/de/web-quickstart#no-repositories-appear-after-connecting-github) befolgen

<h3 id="unable-to-get-organization-uuid">
  Unable to get organization UUID
</h3>

`claude --cloud` und `claude --teleport` erfordern Anmeldung mit einem claude.ai-Konto. Wenn Sie sich mit einem API-Schlüssel authentifizieren oder Ihre gespeicherten Kontodaten veraltet sind, schlagen diese Befehle mit `Unable to get organization UUID` oder einer Nachricht fehl, dass API-Schlüssel-Authentifizierung nicht ausreichend ist. Mit API-Schlüssel-Authentifizierung oder veralteten Kontodaten zeigt das Ausführen von `claude --teleport` ohne eine Sitzungs-ID `Error loading Claude Code sessions` in der Sitzungsauswahl anstelle einer der beiden Nachrichten an, und die gleiche Lösung gilt.

Führen Sie `/login` aus, um sich mit Ihrem claude.ai-Konto anzumelden, und versuchen Sie dann den Befehl erneut. Wenn der Fehler Ihren Anbieter benennt, siehe die [Fehlertabelle](#output-and-errors): Cloud-Sitzungen sind nicht über Drittanbieter verfügbar.

<h3 id="remote-control-session-expired-or-access-denied">
  Remote Control-Sitzung abgelaufen oder Zugriff verweigert
</h3>

`--teleport` verbindet sich über die gleiche Remote Control-Sitzungsinfrastruktur, die Cloud-Sitzungen verwenden, daher werden Authentifizierungs- und Sitzungs-Ablauf-Fehler mit Remote Control-Wording angezeigt. Sie können `Remote Control session expired` oder `Access denied` sehen. Das Verbindungs-Token ist kurzlebig und auf Ihr Konto begrenzt.

* Führen Sie `/login` lokal aus, um Ihre Anmeldedaten zu aktualisieren, und verbinden Sie sich dann erneut
* Bestätigen Sie, dass Sie sich bei demselben Konto angemeldet haben, das die Sitzung besitzt
* Wenn Sie `Remote Control may not be available for this organization` sehen, hat ein Owner Cloud-Sitzungen für Ihre Organisation nicht aktiviert

<h3 id="environment-expired">
  Umgebung abgelaufen
</h3>

Cloud-Sitzungen werden nach einer Inaktivitätszeit beendet und die Sitzungs-VM wird freigegeben. Eine Sitzung gilt als inaktiv, während sie darauf wartet, dass Sie einen [MCP-Connector](/docs/de/cloud-environments#network-access)-Tool-Aufruf genehmigen oder sich bei einem MCP-Server anmelden, und sie kann während dieses Wartens ablaufen.

Öffnen Sie die Sitzung erneut von [claude.ai/code](https://claude.ai/code), um eine frische VM mit Ihrer wiederhergestellten Gesprächshistorie bereitzustellen. Hintergrundarbeit, die noch lief, als die VM freigegeben wurde, wie Subagents und Shell-Befehle, wird nicht wiederhergestellt.

<h2 id="limitations">
  Einschränkungen
</h2>

Bevor Sie Cloud-Sitzungen für einen Workflow verwenden, berücksichtigen Sie diese Einschränkungen:

* **Ratenlimits**: Cloud-Sitzungen teilen Ratenlimits mit allen anderen Claude- und Claude Code-Nutzungen in Ihrem Konto. Das Ausführen mehrerer Aufgaben parallel verbraucht proportional mehr Ratenlimits. Es gibt keine separate Compute-Gebühr für die Cloud-VM.
* **Repository-Authentifizierung**: Sie können eine Cloud-Sitzung nur in Ihr Terminal ziehen, wenn Sie sich bei demselben Konto authentifizieren
* **Plattformbeschränkungen**: Repository-Klonen und Pull Request-Erstellung erfordern GitHub. Selbstgehostete [GitHub Enterprise Server](/docs/de/github-enterprise-server)-Instanzen werden für Team- und Enterprise-Pläne unterstützt. Sie können GitLab, Bitbucket oder andere Nicht-GitHub-Repositories als [lokales Bündel](#send-local-repositories-without-github) zu einer Cloud-Sitzung senden, indem Sie `CCR_FORCE_BUNDLE=1` setzen, aber die Sitzung kann die Ergebnisse nicht zurück zum Remote pushen
* **Organisations-IP-Allowlist**: Cloud-Sitzungen rufen die Anthropic API von von Anthropic verwalteter Infrastruktur auf, nicht von Ihrem Netzwerk, während Sitzungen in einer [selbstgehosteten Umgebung](/docs/de/self-hosted-environments) sie von Ihrem eigenen Netzwerk aufrufen. Wenn Ihre Organisation [IP-Allowlisting](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting) aktiviert hat, schlägt jede von Anthropic gehostete Cloud-Sitzung mit einem Authentifizierungsfehler fehl. Das gleiche gilt für [Code Review](/docs/de/code-review) und [Routines](/docs/de/routines), die auf von Anthropic gehosteten Umgebungen ausgeführt werden; eine Routine, die zu einer selbstgehosteten Umgebung geleitet wird, ruft die API von Ihrem eigenen Netzwerk auf. Kontaktieren Sie [Anthropic Support](https://support.claude.com/), um von Anthropic gehostete Services von der IP-Allowlist Ihrer Organisation auszunehmen.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Cloud-Umgebungen](/docs/de/cloud-environments): Konfigurieren Sie Netzwerkzugriff, Umgebungsvariablen und Setup-Skripte für Cloud-Sitzungen
* [Projekte](/docs/de/claude-projects): eine Konversation, in der Claude parallele Cloud-Sitzungen in Ihren Repositories koordiniert und Bericht erstattet
* [Ultrareview](/docs/de/ultrareview): Führen Sie eine tiefe Multi-Agent-Code-Review in einer Cloud-Sandbox aus
* [Routines](/docs/de/routines): Automatisieren Sie Arbeiten nach einem Zeitplan, über API-Aufruf oder als Reaktion auf GitHub-Events
* [Hooks-Konfiguration](/docs/de/hooks): Führen Sie Skripte bei Sitzungs-Lifecycle-Events aus
* [Alle Einstellungen](/docs/de/settings-reference): Alle Konfigurationsoptionen
* [Sicherheit](/docs/de/security): Isolationsgarantien und Datenverarbeitung
* [Datennutzung](/docs/de/data-usage): Was Anthropic aus Cloud-Sitzungen behält
* [Claude Tag](https://claude.com/docs/claude-tag/overview): Ein von der Organisation verwaltetes @Claude in Slack, das auf der gleichen Cloud-Infrastruktur ausgeführt wird
