> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Selbstgehostete Umgebungen in der Produktion bereitstellen

> Führen Sie selbstgehostete Runner in der Produktion aus: Sicherheitshärtung, Netzwerk-Egress-Kontrolle, Git-Anmeldedaten, Kubernetes- und Compose-Rezepte und Fehlerbehebung.

<Note>
  Selbstgehostete Umgebungen befinden sich in der öffentlichen Beta für Team- und Enterprise-Pläne; [Verfügbarkeit und Einschränkungen](/docs/de/self-hosted-environments#availability-and-limitations) behandelt den Aktivierungspfad. Diese Seite behandelt die Ausführung der Flotte in der Produktion; siehe den [Schnellstart](/docs/de/self-hosted-environments-quickstart) für Ihren ersten Runner und Ihre erste Sitzung.
</Note>

Eine [selbstgehostete Umgebung](/docs/de/self-hosted-environments) führt Claude Code [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) auf Runnern aus, die Sie in Ihrem Netzwerk bereitstellen, und in der Produktion führen diese Sitzungen modellgesteuerten Code im Namen aller aus, die eine Sitzung in der Umgebung starten können. Diese Seite ist für den Operator, der eine funktionierende Umgebung in die Produktion nimmt. Sie durchläuft die Bereitstellung der Reihe nach: was vor dem Verbinden mit echten Systemen gesperrt werden muss, welcher Egress die Flotte benötigt, wie Sitzungen sich bei Ihrem Git-Host authentifizieren, die Bereitstellungsrezepte selbst und was zu überprüfen ist, wenn Sitzungen nicht ordnungsgemäß funktionieren.

<h2 id="harden-your-deployment">
  Härten Sie Ihre Bereitstellung
</h2>

Ein selbstgehosteter Runner führt beliebigen, modellgesteuerten Code auf Ihrer Infrastruktur im Namen aller aus, die eine Sitzung in seiner Umgebung starten können. Das ist jedes Mitglied Ihrer Anthropic-Organisation und jeder, der eine [Claude Tag](https://claude.com/docs/claude-tag/overview)-Kanalsitzung in einem Bereich starten kann, den ein Owner zur Umgebung weitergeleitet hat. Arbeiten Sie jedes Element durch, bevor Sie eine Umgebung mit Produktionssystemen verbinden:

* **Ephemere, sitzungsspezifische Container**: Führen Sie jeden Runner-Prozess in einem frischen Container oder einer VM aus, die zerstört wird, wenn der Prozess beendet wird, mit `--capacity 1` und dem Standard `--drain-grace-sec 0`, sodass jeder Container genau eine Sitzung bedient. Bei einer höheren Kapazität oder mit einer positiven Drain-Grace bedient ein Container mehrere Sitzungen vom selben [gesperrten Owner](/docs/de/self-hosted-environments#key-concepts); siehe [Runner-Lebenszyklus](/docs/de/self-hosted-environments#runner-lifecycle). Verwenden Sie kein Dateisystem zwischen Runner-Neustarts wieder, außer in der absichtlichen [vorgewärmten Checkout](#reuse-a-pre-warmed-checkout)-Einrichtung, und niemals über Owners hinweg.
* **Keine breiten Anmeldedaten im Image**: Fügen Sie keine langlebigen SSH-Schlüssel, Cloud-Provider-Anmeldedaten oder persönliche Zugriffstokens ein, die mehr gewähren als eine Sitzung benötigt. Erstellen Sie Anmeldedaten, die während einer Sitzung verwendet werden, wie Push- oder API-Tokens, pro Sitzung aus Ihrem [Wrapper-Skript](/docs/de/self-hosted-environments-configuration#wrapper-scripts). Für den anfänglichen Clone, der vor dem Ausführen des Wrappers stattfindet, verwenden Sie einen [`checkout`-Lebenszyklus-Hook](/docs/de/self-hosted-environments-configuration#checkout) oder [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy); siehe [Git konfigurieren](#configure-git).
* **Halten Sie das Umgebungsgeheimnis von sitzungsausführenden Hosts fern**: Das Umgebungsgeheimnis kann Runner registrieren und jede Sitzung abholen, die in der Umgebung in die Warteschlange eingereiht ist. In einer festen Flotte lebt es auf jedem Runner-Host, wo jeder Sitzungscode das Geheimnisdatei lesen kann. Bevorzugen Sie [On-Demand-Runner](/docs/de/self-hosted-environments-configuration#on-demand-runners), bei denen das Geheimnis auf dem Orchestrator-Host bleibt, der niemals Benutzercode ausführt, und jeder Runner einen einmaligen Arbeitsauftrag erhält, der genau einen Runner registriert. Behandeln Sie in einer festen Flotte die Umgebungsgeheimnisdatei als von jeder Sitzung lesbar und rotieren Sie das Geheimnis nach jedem vermuteten Sitzungskompromiss.
* **Standard-Deny-Netzwerk-Egress**: Beschränken Sie den ausgehenden Datenverkehr von Runner- und Sitzungs-Containern an Ihrer eigenen Netzwerkgrenze in jeder Umgebung; [Standard-Deny-Egress](#default-deny-egress) behandelt, was erlaubt ist und warum.
* **Least-Privilege-Host-IAM**: Die Compute-Identität, die an den Runner-Host angehängt ist, wie ein Instance-Profil oder ein Node-Service-Konto, sollte nur das gewähren, was der Runner selbst benötigt. Sitzungen sollten ihre eigenen Anmeldedaten über Ihr Wrapper-Skript erhalten, anstatt die des Hosts zu erben.
* **Blockieren Sie den Cloud-Metadaten-Endpunkt von Sitzungen**: Um Sitzungen von der Host-Identität fernzuhalten, müssen Sie ihren Zugriff auf den Cloud-Metadaten-Endpunkt blockieren, und Subnetz-Level-Egress-Richtlinien unterbrechen keinen Link-Local-Metadaten-Datenverkehr, daher blockieren Sie ihn im Container selbst:

  * IMDSv2 mit einem Hop-Limit von eins
  * GKE Workload Identity mit Metadaten-Verbergung
  * Ein explizites Deny für `169.254.169.254` im Netzwerk-Namespace des Sitzungs-Containers

  Der Block gilt auch für Ihr Wrapper-Skript und Lebenszyklus-Hooks, da sie den Container teilen. Authentifizieren Sie jeden Token-Austausch mit dem [Sitzungs-JWT](/docs/de/self-hosted-environments-identity) gegen Ihren eigenen Token-Service über zulassungslisten-Egress, oder verwenden Sie eine dateibasierte Web-Identität wie IAM Roles for Service Accounts (IRSA) auf Amazon EKS.
* **Pro-Runner-Dateisystem-Isolation**: Jeder Runner-Prozess erhält sein eigenes Arbeitsverzeichnis, das kein anderer Prozess auf dem Host lesen oder schreiben kann. Machen Sie `--hooks-dir`, das Wrapper-Skript und das `~/.claude/`-Verzeichnis des Hosts für die Sitzung schreibgeschützt, entweder in das Image eingebaut oder schreibgeschützt eingebunden.
* **Dispatch hat keine Pro-Umgebungs-Zugriffskontrolle**: Jedes Mitglied Ihrer Anthropic-Organisation kann eine Sitzung in jede ihrer Umgebungen starten. Wenn ein Owner [Claude Tag-Kanäle zur Umgebung leitet](/docs/de/cloud-environments#set-the-environment-a-claude-tag-channel-uses), kann jeder, den die [Claude Tag-Zugriffssetting](https://claude.com/docs/claude-tag/admins/restrict-access#restrict-who-can-use-claude) zulässt, Kanalsitzungen starten, die dort ausgeführt werden. Standardmäßig ist das jeder im verbundenen Slack-Workspace, mit oder ohne Claude-Konto. Behandeln Sie jeden Runner-Host als erreichbar für die Codeausführung durch jeden, der ihn starten kann, und platzieren Sie auf einem Runner-Host nur Daten und Anmeldedaten, die alle diese Personen lesen dürfen. [`--lock-to-account`](/docs/de/self-hosted-environments-reference#runner-cli-flags) begrenzt, welche Konten-Sitzungen ein bestimmter Host ausführt, aber es verengt nicht, wer in die Umgebung starten kann. Um selbstgehostete Umgebungen zur einzigen Picker-Option zu machen, kann ein [Owner](/docs/de/cloud-environments#organization-shared-environments) Anthropic-gehostete Umgebungen für die ganze Organisation auf der [**Cloud-Umgebungen**-Seite](https://claude.ai/admin-settings/cloud-environments) ausblenden.
* **Erzwingen Sie die Repo-Settings-Guard**: Wählen Sie den Guard-Modus mit [`--confine-repo-settings`](/docs/de/self-hosted-environments-reference#runner-cli-flags). Der Standard `warn` protokolliert eine Verletzung und startet die Sitzung trotzdem, `enforce` lehnt die Sitzung ab, und `off` deaktiviert den Scan. Der Runner scannt die festgeschriebenen Einstellungen jedes Repositorys auf:

  * Eine Berechtigung, die außerhalb des eigenen Workspace dieser Sitzung aufgelöst wird: ein `additionalDirectories`-Eintrag, eine `Edit`-, `Write`- oder `NotebookEdit`-Regel in `permissions.allow`, oder ein `sandbox.filesystem.allowWrite`- oder `allowRead`-Eintrag
  * Ein nicht leerer `env`-Block
  * Eine Operator-Haltungs-Überschreibung wie `sandbox.enabled: false`

  Die Guard läuft unabhängig von [`--trust-workspace`](/docs/de/self-hosted-environments-reference#runner-cli-flags) und deckt keine Repository-Hooks, `.mcp.json` oder Bash-Regeln ab; siehe [Berechtigungen und Tool-Genehmigung](/docs/de/self-hosted-environments-configuration#permissions-and-tool-approval) für den Ort dieser Berechtigungen.

<Note>
  Die IP-Zulassungsliste Ihrer Organisation deckt selbstgehosteten Runner-Datenverkehr standardmäßig nicht ab. Verlassen Sie sich nicht darauf als Netzwerkkontrolle für Runner- oder Sitzungs-Datenverkehr; wenden Sie stattdessen Standard-Deny-Egress an Ihrer eigenen Netzwerkgrenze an, und kontaktieren Sie Ihr Anthropic-Account-Team, wenn Sie IP-Zulassungslisten-Durchsetzung für Ihre Organisation wünschen.
</Note>

<h2 id="network-requirements">
  Netzwerkanforderungen
</h2>

Der Runner und die Sitzungs-Kinder, die er spawnt, stellen ausgehende Verbindungen zu den folgenden Hosts her. Beschränken Sie den Sitzungs-Container-Egress auf diese Hosts und die spezifischen internen Services, die Sitzungen erreichen müssen; [Standard-Deny-Egress](#default-deny-egress) behandelt wie und warum.

Diese Hosts sind immer erforderlich:

| Host                                                           | Port                                      | Verwendet für                                                                                                                                                                                                                                                                                                                                                                                 |
| :------------------------------------------------------------- | :---------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `api.anthropic.com`                                            | 443, HTTPS; WSS nur für den SCM-Connector | Runner-Kontrollebene und Sitzungs-Streaming, Modell-Inferenz, Feature-Flags, Produkt-Analytik, [JWKS](/docs/de/self-hosted-environments-identity)-Schlüssel-Abrufe, Commit-Signierung, der Git-Proxy wenn `--use-anthropic-git-proxy` gesetzt ist, und der Orchestrator-[SCM-Connector](/docs/de/self-hosted-environments-reference#scm-connector-flags)-Tunnel wenn `--scm-connector-host` gesetzt ist |
| Ihr Git-Host, wie `github.com` oder Ihr GitHub Enterprise-Host | 443 oder 22                               | Klonen und Pushen von Repositorys. Nicht erforderlich, wenn der Runner `--use-anthropic-git-proxy` verwendet, das Git-Datenverkehr durch `api.anthropic.com` leitet.                                                                                                                                                                                                                          |

Ob diese Hosts erforderlich sind, hängt von Ihrer Konfiguration ab:

| Host                                 | Port | Wenn erforderlich                                                                                                                                                                                                                                                                                                                      |
| :----------------------------------- | :--- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `downloads.claude.ai`                | 443  | Zur Installationszeit, wenn Sie Claude Code auf dem Host mit dem nativen Installer installieren oder aktualisieren; das `install.sh`-Skript selbst wird von `claude.ai` bereitgestellt. Zur Sitzungs-Laufzeit nur, wenn Sitzungen Plugins vom offiziellen Anthropic-Marketplace installieren.                                          |
| `storage.googleapis.com`             | 443  | Zur Sitzungs-Laufzeit für die Plugin-Installationszähler und Metadaten, die in `/plugin` angezeigt werden.                                                                                                                                                                                                                             |
| `code.claude.com` und `claude.com`   | 443  | Dokumentations-Lookups durch den integrierten Claude-Code-Guide-Agent und vorab genehmigte WebFetch-Anfragen während Sitzungen. Das Blockieren dieser Hosts betrifft nur Dokumentations-Lookups.                                                                                                                                       |
| `*.frame.claudeusercontent.com`      | 443  | Nur wenn das [Artifact-Tool](/docs/de/artifacts#availability) für Sitzungen in Ihrer Organisation verfügbar ist; die Standardwerte variieren je nach Plan, gemäß der Verfügbarkeitstabelle dort. Setzen Sie `CLAUDE_CODE_DISABLE_ARTIFACT=1` auf dem Runner, um das Tool unabhängig von der Organisationseinstellung deaktiviert zu halten. |
| `registry.npmjs.org`                 | 443  | Wenn eine Sitzung ein Plugin installiert, sowohl zum Abrufen von npm-Quell-Plugin-Paketen als auch zum Installieren der Node.js-Abhängigkeiten eines Plugins, oder wenn ein `npx`-gestarteter MCP-Server läuft                                                                                                                         |
| `http-intake.logs.us5.datadoghq.com` | 443  | Anthropic-Betriebsmetriken. Nur wenn `CLAUDE_CODE_BYOC_ENABLE_DATADOG=1` gesetzt ist; standardmäßig in selbstgehosteten Umgebungen deaktiviert.                                                                                                                                                                                        |
| `browser-intake-us5-datadoghq.com`   | 443  | Anthropic-Fehlerberichts-Uploads, nur gesendet wenn [Fehlerberichterstattung](/docs/de/data-usage#telemetry-services) für das Konto der Sitzung aktiviert ist. Unterdrückt durch `DISABLE_ERROR_REPORTING=1` oder `DISABLE_TELEMETRY=1`.                                                                                                    |

Der Runner erreicht `statsig.anthropic.com`, `*.sentry.io`, `claude.ai` oder `platform.claude.com` nicht. Diese Hosts erscheinen in einigen älteren Enterprise-Netzwerk-Checklisten, aber Sie müssen sie nicht für Runner- oder Sitzungs-Datenverkehr zulassungslisten: Feature-Flag-Abrufe gehen zu `api.anthropic.com`, und der Runner authentifiziert sich mit dem Umgebungsgeheimnis anstelle von interaktivem OAuth. Zwei Host-seitige Flows erreichen `claude.ai`, daher führen Sie sie von einem Host aus, dessen Egress es erlaubt, anstatt den Sitzungs-Container-Egress zu verbreitern: Das Einzeilen-Installer-Skript ruft `install.sh` von `claude.ai` zur Installationszeit ab, und interaktives `claude auth login`, das der [geführte Setup](/docs/de/self-hosted-environments-quickstart#set-up-an-environment-and-runner), `doctor`'s signierter Modus und [CI-Dispatch](/docs/de/self-hosted-environments-testing#authenticate-from-ci) verwenden, melden sich durch `claude.ai`, `claude.com` und `platform.claude.com` an. `mcp-proxy.anthropic.com` ist auch nicht erforderlich: Selbstgehostete Sitzungen verwenden es nicht, und die Lieferung Ihrer Organisations-claude.ai-Connectoren an Sitzungen, wenn für Ihre Organisation aktiviert, leitet durch `api.anthropic.com`. Siehe [MCP-Server](/docs/de/self-hosted-environments-configuration#mcp-servers).

<h3 id="default-deny-egress">
  Standard-Deny-Egress
</h3>

Stellen Sie Runner- und Sitzungs-Container in einem Netzwerk-Segment oder Namespace bereit, dessen ausgehender Datenverkehr auf die Hosts in der [Netzwerkanforderungs-Tabelle](#network-requirements), Ihren Git-Host und die spezifischen internen Services begrenzt ist, die Sitzungen erreichen müssen. Das Produkt kann dies nicht überprüfen oder erzwingen, daher wenden Sie es an Ihrer eigenen Netzwerkgrenze in jeder Umgebung an. Sitzungscode ist modellgesteuert und kann Verbindungen zu beliebigen Hosts versuchen; Standard-Deny-Egress auf der Netzwerkebene begrenzt, wo diese Versuche landen können. Dies gilt unabhängig vom Berechtigungsmodus: Der Standard-Vorab-Genehmigungstool-Satz enthält bereits `Bash`, daher läuft Shell-Egress ohne Eingabeaufforderung auch ohne [Auto-Modus](/docs/de/self-hosted-environments-configuration#permissions-and-tool-approval).

Für Details darüber, welche Telemetrie jede Sitzung aussendet und wie man sie ausschaltet, siehe [Telemetrie](/docs/de/self-hosted-environments-reference#telemetry).

<h3 id="authenticate-to-an-egress-proxy">
  Authentifizieren Sie sich bei einem Egress-Proxy
</h3>

Einige Corporate-Egress-Proxys erfordern einen `Proxy-Authorization`-Header bei jeder Verbindung. Das Token in diesem Header rotiert oft zu schnell, um es in die Proxy-URL zu schreiben, die Sie in `HTTPS_PROXY` setzen. Setzen Sie `HTTPS_PROXY` oder `HTTP_PROXY` wie gewohnt auf die URL Ihres Proxys, dann setzen Sie `--proxy-authorization-command` oder `--proxy-authorization-file`, um dem Runner zu sagen, wo er den Header-Wert lesen soll. Beide Flags erfordern Claude Code v2.1.238 oder später.

<h4 id="choose-where-the-proxy-authorization-value-comes-from">
  Wählen Sie, woher der `Proxy-Authorization`-Wert kommt
</h4>

Wählen Sie das Flag, das der Art entspricht, wie Sie das `Proxy-Authorization`-Token erzeugen:

* **[`--proxy-authorization-command <command>`](/docs/de/self-hosted-environments-reference#runner-cli-flags)**: Wählen Sie dies für ein Token, das Sie bei Bedarf generieren. Der Runner führt das Shell-Kommando aus und verwendet seine getrimmte Standardausgabe als Header-Wert, zum Beispiel `Bearer <token>`.
* **[`--proxy-authorization-file <path>`](/docs/de/self-hosted-environments-reference#runner-cli-flags)**: Wählen Sie dies für ein Token, das ein anderer Prozess an Ort und Stelle rotiert. Der Runner liest die Datei und verwendet ihren getrimmten Inhalt als Header-Wert.

<h4 id="configurations-the-runner-refuses-to-start-with">
  Konfigurationen, die der Runner ablehnt zu starten
</h4>

Jedes Flag hat auch eine Umgebungsvariablen-Form, die neben ihm in der [Runner-CLI-Flags-Referenz](/docs/de/self-hosted-environments-reference#runner-cli-flags) aufgelistet ist. Bevor der Runner Ihren Proxy oder die Kontrollebene kontaktiert, überprüft er die Flags und ihre Variablen und lehnt in drei Fällen ab zu starten:

* **Beide Flags gesetzt**: Ein Flag plus die Umgebungsvariable des anderen Flags zählt als Setzen beider.
* **Keine Proxy-URL**: Weder `HTTPS_PROXY` noch `HTTP_PROXY` enthält eine `http://`- oder `https://`-URL. Der Runner liest beide Variablen in Groß- oder Kleinbuchstaben und konsultiert nicht `ALL_PROXY`.
* **Eines der Flags an den Orchestrator-Subcommand übergeben**: `self-hosted-runner orchestrator` akzeptiert die Flags oder ihre Umgebungsvariablen nicht. Übergeben Sie das Flag stattdessen an jeden Runner, den der Orchestrator startet.

<h4 id="what-the-runner-changes-while-a-proxy-authorization-flag-is-set">
  Was der Runner ändert, während ein Proxy-Autorisierungs-Flag gesetzt ist
</h4>

Mit einem der Flags gesetzt, startet der Runner seinen eigenen Listener und sendet Proxy-Datenverkehr von sich selbst, seinen Lebenszyklus-Hooks und seinen Sitzungen durch diesen Listener. Der Listener fügt den `Proxy-Authorization`-Header auf dem Weg zu Ihrem Proxy hinzu.

* **Listener**: Der Listener ist ein Forward-Proxy auf `127.0.0.1`. Der Runner startet den Listener vor der Registrierung bei der Kontrollebene und beendet sich beim Start, wenn der Listener nicht starten kann.
* **Proxy-Variablen**: Der Runner schreibt whichever von `HTTPS_PROXY` und `HTTP_PROXY` um, die Sie setzen, damit es auf den Listener zeigt. Dieser umgeschriebene Wert erreicht den Runner selbst, seine Lebenszyklus-Hooks und jede Sitzung, die er ausführt.
* **Token-Rotation**: Ein rotiertes Token wird ohne Neustart wirksam. Für jede Verbindung, die der Listener zu Ihrem Proxy öffnet, führt der Runner Ihren Befehl aus oder liest Ihre Datei erneut und fügt das Ergebnis als Header hinzu.
* **Sitzungs-Umgebung**: Eine Sitzung erreicht Ihren Proxy nur durch den Listener. In der Umgebung jeder Sitzung entfernt der Runner `ALL_PROXY`, entfernt jede Schreibweise von `HTTPS_PROXY` oder `HTTP_PROXY`, die Sie nicht gesetzt haben, und pinnt `NO_PROXY` auf den Wert des Runners.
* **Logs**: Der Runner protokolliert niemals den Header-Wert.

<h2 id="configure-git">
  Git konfigurieren
</h2>

Der Runner verwaltet Repository-Checkouts, konfiguriert aber standardmäßig nicht die Git-Identität oder Anmeldedaten. Sie kontrollieren das Runner-Image und die Prozessumgebung, daher kontrollieren Sie die Git-Konfiguration. Wählen Sie einen von zwei Ansätzen:

* **Lassen Sie den Runner Git konfigurieren**: Starten Sie den Runner mit `--configure-git`, um die gleiche Identität und Commit-Signierungskonfiguration zu schreiben, die Anthropic-gehostete Sitzungen verwenden
* **Versenden Sie Git-Konfiguration in Ihrem Image**: Setzen Sie Identität und Push-Anmeldedaten selbst, zum Beispiel um unter Ihrer eigenen Bot-Identität zu committen

Git-Versionsuntergrenzen auf dem Runner-Host: [`--configure-git`](#let-the-runner-configure-git) SSH-Commit-Signierung erfordert Git 2.34 oder neuer, [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy) erfordert 2.32 oder neuer, und das Fortsetzen von Sitzungen von Branches, die von [`--push-outcome-on-release`](/docs/de/self-hosted-environments-reference#runner-cli-flags) gepusht werden, erfordert 2.29 oder neuer. Git 2.24 ist ausreichend, wenn Sie alle drei weglassen und die Git-Identität selbst verwalten.

<h3 id="let-the-runner-configure-git">
  Lassen Sie den Runner Git konfigurieren
</h3>

Starten Sie den Runner mit `--configure-git`, oder setzen Sie `SELF_HOSTED_RUNNER_CONFIGURE_GIT=1`, um die globale Git-Konfiguration beim Start zu schreiben:

* `user.name = Claude` und `user.email = noreply@anthropic.com`, passend zu Anthropic-gehosteten Sitzungen
* SSH-Format-Commit- und Tag-Signierung, geleitet durch einen Runner-verwalteten Shim, der jeden Commit über Anthropic's Signierungsservice mit den Anmeldedaten der Sitzung signiert. Signaturen sind auf GitHub gegen Anthropic's veröffentlichten SSH-Signierungsschlüssel überprüfbar.
* `push.negotiate = true`, damit Git Ihren Git-Host fragt, welche Commits er bereits hat, bevor er einen Push packt. Erfordert Claude Code v2.1.257 oder später.
* `core.hooksPath`, das auf ein Runner-verwaltetes Hooks-Verzeichnis zeigt. Seine `commit-msg`- und `prepare-commit-msg`-Hooks fügen jedem Commit einen `Co-authored-by:`-Trailer für den Ersteller der Sitzung hinzu, erstellt aus der E-Mail in [`CCR_SESSION_ACCOUNT_EMAIL`](/docs/de/self-hosted-environments-configuration#wrapper-scripts) und weggelassen, wenn diese Variable nicht gesetzt ist. Wenn Ihr Image bereits `core.hooksPath` setzt, lässt der Runner Ihre Einstellung in Kraft, überspringt die Installation dieser Hooks und gibt eine `[runner:git]`-Warnung aus.

Commit-Signierung erfordert Git 2.34 oder neuer; der Runner überprüft beim Start und beendet sich mit einem Fehler, wenn Ihr Git älter ist. Dieses Flag konfiguriert keine Push-Anmeldedaten, die Sie immer noch im Image bereitstellen.

<h3 id="ship-git-config-in-your-image">
  Versenden Sie Git-Konfiguration in Ihrem Image
</h3>

Git-Identität ist für jeden Commit erforderlich. Setzen Sie sie systemweit in Ihrem Dockerfile, damit die Konfiguration unabhängig davon gilt, welcher Benutzer den Runner-Prozess ausführt:

```dockerfile theme={null}
RUN git config --system user.name "Claude" && \
    git config --system user.email "noreply@anthropic.com"
```

Ohne eine Identität schlägt `git commit` mit `Please tell me who you are` fehl und Sitzungen können nicht voranschreiten. Sie können stattdessen Ihre eigene Bot-Identität verwenden; der Runner überschreibt diese Werte nicht.

Backen Sie keine langlebigen oder breit gefassten Push-Anmeldedaten in ein gemeinsames Runner-Image: Eine Anmeldedaten im Image ist für jede Sitzung verfügbar, die das Image ausführt, wer auch immer sie gestartet hat. Erstellen Sie stattdessen ein kurzlebiges, minimal gefasstes Token pro Sitzung aus Ihrem [Wrapper-Skript](/docs/de/self-hosted-environments-configuration#wrapper-scripts), unter Verwendung der Identität des Sitzungs-Erstellers, die aus dem Sitzungs-JWT dekodiert ist. Paaren Sie es mit einem ephemeren Pro-Sitzungs-Container, der `--capacity 1` erfordert, damit keine Anmeldedaten die Sitzung überlebt, die sie erstellt hat; siehe den [Härtungsabschnitt](#harden-your-deployment).

Wenn Sie Push-Anmeldedaten auf Image-Ebene konfigurieren müssen, zum Beispiel für einen schreibgeschützten Deploy-Schlüssel, begrenzen Sie sie so eng wie Ihr Git-Host erlaubt:

* Ein SSH-Deploy-Schlüssel, der auf ein Repository mit einer `url.<base>.insteadOf`-Umschreibung begrenzt ist
* Ein `credential.helper`, der ein minimal gefasstes Token zurückgibt
* `GIT_SSH_COMMAND`, das auf einen eng gefassten Schlüssel zeigt

Welcher Mechanismus Sie auch konfigurieren, muss ohne Eingabeaufforderung funktionieren, da der integrierte Clone und Fetch des Runners die Eingabeaufforderungen deaktivieren, die Git, SSH und Git Credential Manager sonst zeigen würden:

* Der Runner setzt `GIT_TERMINAL_PROMPT=0`, daher fragt Git nicht nach Benutzername oder Passwort.
* Der Runner führt SSH mit `BatchMode=yes` aus, angehängt an Ihren `GIT_SSH_COMMAND`, wenn Sie einen setzen, daher fragt SSH nicht nach einer Passphrase oder Host-Bestätigung.
* Der Runner setzt `GCM_INTERACTIVE=never`, daher öffnet Git Credential Manager keinen Anmeldedialog.
* Der Runner löscht `core.askPass`, daher setzen Sie es, wenn Sie einen Askpass-Helper verwenden, stattdessen durch die `GIT_ASKPASS`-Umgebungsvariable.

Wenn Ihr Git-Host die Anmeldedaten ablehnt, oder Sie haben keine konfiguriert, versucht der Runner ein paar Mal erneut und schlägt dann fehl bei der Repository-Vorbereitung, wenn das Repository das ist, in das die Sitzung Ergebnisse pusht. Für ein Repository, das die Sitzung nur liest, behandelt [Troubleshooting](#troubleshooting) den Fall, wenn der Runner es stattdessen überspringt. Der Runner übergibt diese Einstellungen nicht in die Umgebung der Sitzung.

Wenn Checkout-Verzeichnisse einem anderen uid als dem Runner-Prozess gehören, weigert sich Git, auf ihnen zu arbeiten; fügen Sie `safe.directory` hinzu:

```dockerfile theme={null}
RUN git config --system --add safe.directory '*'
```

<h3 id="use-the-anthropic-git-proxy">
  Verwenden Sie den Anthropic-Git-Proxy
</h3>

Starten Sie den Runner mit `--use-anthropic-git-proxy`, oder setzen Sie `CLAUDE_RUNNER_USE_GIT_PROXY=1`, um durch Anthropic's Git-Proxy zu klonen, authentifiziert mit dem kurzlebigen Token der Sitzung. Für gewöhnliche Benutzersitzungen verwendet der Proxy das GitHub- oder GitHub Enterprise-OAuth-Token, das für den Sitzungs-Ersteller gespeichert ist; für Bot- und Agent-Sitzungen verwendet er das GitHub App-Installations-Token Ihrer Organisation. Wie auch immer, das Runner-Image benötigt überhaupt keine Git-Anmeldedaten: keine SSH-Schlüssel, keinen Credential-Helper, keine `.netrc`. Dies ist der gleiche Auth-Pfad, den Anthropic-gehostete Umgebungen verwenden.

Der Proxy erfordert `--capacity 1`, weil die Proxy-URL pro Sitzung ist, und Git 2.32 oder neuer, weil älteres Git den Konfigurationsmechanismus ignoriert, den der Proxy verwendet, um Sitzungen voneinander zu isolieren. Der Runner weigert sich zu starten, wenn eine der Anforderungen nicht erfüllt ist. Weil der Proxy von Anthropic's Seite abruft, muss Ihr Git-Host von Anthropic-Infrastruktur erreichbar sein, die gleiche Anforderung, die Anthropic-gehostete Sitzungen haben; für einen Git-Host, der nur innerhalb Ihres Netzwerks routbar ist, verwenden Sie stattdessen einen [`checkout`-Lebenszyklus-Hook](/docs/de/self-hosted-environments-configuration#checkout). Jeder Runner-Prozess verarbeitet jeweils eine Sitzung, daher führen Sie mehr Replicas für Parallelität aus. Wenn der Proxy aktiviert ist, haben `--git-host-rewrite` und `--git-ssh-rewrite` keine Auswirkung: Die Proxy-URL zeigt auf `api.anthropic.com`, nicht auf Ihren Git-Host.

Der Runner meldet die Opt-in-Entscheidung auch Anthropic, wenn er sich registriert, und gibt beim Start `Registering as opted in to Anthropic-managed git (--use-anthropic-git-proxy)` aus. Die Meldung der Opt-in-Entscheidung erfordert Claude Code v2.1.267 oder später, und frühere Versionen akzeptieren das Flag, ohne es zu melden oder diese Zeile auszugeben. Jede Sitzung auf einem Opt-in-Runner verwendet dann entweder Anthropic-verwaltetes Git oder die Pro-Sitzungs-Proxy-URL. Wenn eine Sitzung die Pro-Sitzungs-Proxy-URL verwendet, protokolliert der Runner eine `[runner:warn]`-Zeile, die dies angibt.

<h3 id="rewrite-git-urls-for-private-networks">
  Schreiben Sie Git-URLs für private Netzwerke um
</h3>

Repository-URLs kommen von der Kontrollebene als HTTPS mit dem Hostnamen Ihres Git-Hosts; für GitHub Enterprise ist das der Hostname, den Sie für die [GitHub Enterprise-Integration](/docs/de/github-enterprise-server) in Claude Code-Admin-Einstellungen auf claude.ai konfiguriert haben. Zwei wiederholbare Flags schreiben diese URLs vor dem Clone um:

* `--git-host-rewrite <from>=<to>`: für Split-Horizon-DNS, wo Anthropic Ihren Git-Host über einen externen Hostnamen erreicht, aber Runner einen internen verwenden müssen
* `--git-ssh-rewrite <host>`: für Git-Hosts, die nur SSH akzeptieren, Umschreiben von `https://<host>/owner/repo` zu `git@<host>:owner/repo`

Host-Umschreibung läuft zuerst, daher listen Sie den internen Hostnamen in `--git-ssh-rewrite` auf, wenn Sie beide benötigen. Für vollständige Kontrolle über Checkout verwenden Sie einen [`checkout`-Lebenszyklus-Hook](/docs/de/self-hosted-environments-configuration#checkout).

<h2 id="build-the-runner-image">
  Erstellen Sie das Runner-Image
</h2>

Anthropic veröffentlicht kein vorgefertigtes Runner-Image. Erstellen Sie Ihr eigenes um die `claude`-Binärdatei, schichten Sie ein, was auch immer Ihre Repositorys benötigen: Sprach-Laufzeiten, Compiler, Paket-Manager und [MCP](/docs/de/mcp)-Sidecars.

Die Rezepte unten verwenden `--capacity 4`, daher bedient ein Container bis zu vier gleichzeitige Sitzungen vom selben gesperrten Owner. Das bietet nicht die Pro-Sitzungs-Container-Isolation im [Härtungsabschnitt](#harden-your-deployment): Bevor Sie eine Umgebung mit Produktionssystemen verbinden, führen Sie entweder die Rezepte bei `--capacity 1` mit einem Container pro Sitzung aus, oder verwenden Sie [On-Demand-Runner](/docs/de/self-hosted-environments-configuration#on-demand-runners), die auch das Umgebungsgeheimnis von sitzungsausführenden Hosts fernhalten.

Dieses Dockerfile ist ein minimaler Ausgangspunkt:

```dockerfile theme={null}
FROM debian:bookworm-slim
ARG CLAUDE_CODE_VERSION
RUN apt-get update && apt-get install -y --no-install-recommends git curl ca-certificates openssh-client \
 && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL "https://downloads.claude.ai/claude-code-releases/${CLAUDE_CODE_VERSION:?set with --build-arg CLAUDE_CODE_VERSION}/linux-x64/claude" \
      -o /usr/local/bin/claude && chmod +x /usr/local/bin/claude
RUN git config --system user.name "Claude" \
 && git config --system user.email "noreply@anthropic.com" \
 && git config --system --add safe.directory '*'
ENTRYPOINT ["claude"]
```

Tauschen Sie `linux-x64` gegen `linux-arm64` aus, wenn Ihre Knoten ARM sind, oder gegen `linux-x64-musl` oder `linux-arm64-musl` auf einem musl-basierten Image wie Alpine; siehe [Alpine Linux-Setup](/docs/de/setup#alpine-linux-and-musl-based-distributions) für die zusätzlichen Pakete, die musl-Images benötigen. Die URL ist der Standard-Claude-Code-Release-Ort, daher können Sie die heruntergeladene Binärdatei gegen das signierte Manifest des Release überprüfen, wie in [Binäre Integrität und Code-Signierung](/docs/de/setup#binary-integrity-and-code-signing) beschrieben. Erstellen Sie das Image mit Claude Code Version 2.1.224 oder später, pushen Sie es dann in Ihre Registry und referenzieren Sie es in den Rezepten unten:

```bash theme={null}
docker build --build-arg CLAUDE_CODE_VERSION=2.1.267 -t <your-registry>/claude-runner:latest .
```

<h2 id="size-cpu-and-memory-for-sessions">
  Dimensionieren Sie CPU und Speicher für Sitzungen
</h2>

Dimensionieren Sie einen Runner-Container oder Host für die Sitzungen, die er ausführt, anstatt für den Runner-Prozess. Der Runner selbst fragt nach Arbeit ab, bereitet den Checkout jeder Sitzung vor, führt Ihre [Lebenszyklus-Hooks](/docs/de/self-hosted-environments-configuration#lifecycle-hooks) aus und startet und überwacht die Sitzungs-Prozesse. Die Last kommt von den Sitzungen: Jede ist ein Claude Code-Prozess plus alles, was sie startet, wie Builds, Test-Suites, Paket-Installationen und [MCP-Server](/docs/de/mcp).

Für eine Sitzung beginnen Sie mit den folgenden Werten, angegeben als Kubernetes-Anfragen und Limits oder das Äquivalent Ihrer Plattform, und behandeln Sie sie als Ausgangspunkt anstatt als Anforderung:

* **Speicher**: eine Anfrage und ein Limit von jeweils 4 GiB, was das 4-GB-Minimum in Claude Code's [Systemanforderungen](/docs/de/setup#system-requirements) erfüllt. Halten Sie die beiden gleich, damit der Scheduler die volle Speicherkapazität des Containers berücksichtigt. Wenn der Container sein Speicherlimit erreicht, tötet der Kernel Prozesse darin, was eine Sitzung mitten in einer Aufgabe beenden kann.
* **CPU**: eine Anfrage von 2 CPUs und ein Limit von 4 CPUs, daher kann eine Sitzung während Builds über die Anfrage hinaus platzen. Der Kernel drosselt einen Container bei seinem CPU-Limit, anstatt Prozesse darin zu töten, daher laufen Sitzungen am Limit langsamer, aber laufen weiter.

In einer Kubernetes-Container-Spezifikation setzen Sie diese Startwerte mit dem folgenden `resources`-Block:

```yaml theme={null}
resources:
  requests:
    cpu: "2"
    memory: 4Gi
  limits:
    cpu: "4"
    memory: 4Gi
```

Builds und Tests sind normalerweise der größte und variabelste Teil der Last einer Sitzung, daher führen Sie einen repräsentativen Build Ihres Repositorys aus, messen Sie seinen Peak-CPU und Speicher, und erhöhen Sie jeden Startwert, der keinen Platz für den Claude Code-Prozess auf diesem Peak lässt.

Der Runner verwendet `--capacity`, um zu begrenzen, wie viele Sitzungen er gleichzeitig ausführt. Er teilt CPU oder Speicher nicht zwischen ihnen, daher teilen sich die Sitzungen auf einem Runner die CPU und den Speicher des Containers. Um die Freigabe einer Sitzung zu begrenzen, wenden Sie Limits aus Ihrem [Wrapper-Skript](/docs/de/self-hosted-environments-configuration#wrapper-scripts) an. Was Sie einem Container geben, hängt daher davon ab, wie viele Sitzungen er gleichzeitig bedient:

* **Eine Sitzung pro Runner**: Geben Sie jedem Container die Werte einer Sitzung. Verwenden Sie diese Dimensionierung bei `--capacity 1`, das der [Härtungsabschnitt](#harden-your-deployment) empfiehlt, und für [On-Demand-Runner](/docs/de/self-hosted-environments-configuration#on-demand-runners), wo Sie die Werte auf der Workload setzen, die Ihr [`spawn-runner`-Hook](/docs/de/self-hosted-environments-configuration#the-spawn-runner-hook) einreicht, wie ein Kubernetes Job's Pod-Template.
* **Mehrere Sitzungen pro Runner**: Bei einem `--capacity` über eins multiplizieren Sie die Werte einer Sitzung mit der Kapazität, weil bis zu so viele Sitzungen gleichzeitig im Container laufen können. Die [Kubernetes](#kubernetes)- und [Docker Compose](#docker-compose)-Rezepte führen `--capacity 4` ohne CPU- oder Speicherlimits aus, daher fügen Sie Limits hinzu, die für die Kapazität dimensioniert sind, die Sie ausführen.

<h2 id="kubernetes">
  Kubernetes
</h2>

Der Runner bedient `GET /healthz` auf Port 8080 standardmäßig, konfigurierbar mit `--health-port`, daher funktionieren Kubernetes-Probes ohne zusätzliches Setup. Der Endpunkt gibt `200` zurück, wann immer der Prozess lebt, daher erkennen die Probes unten einen toten Prozess, nicht einen steckengebliebenen; um einen Runner zu fangen, der aufgehört hat zu pollen, warnen Sie die `last_poll_age_seconds`-Serie von [`/metrics`](/docs/de/self-hosted-environments-reference#prometheus-metrics). Die Deployment unten bindet das Umgebungsgeheimnis von einem Kubernetes Secret, zeigt die Liveness- und Readiness-Probes auf `/healthz` und setzt eine 90-Sekunden-Terminierungs-Grace-Periode. Siehe [Shutdown-Timing](#shutdown-timing) für warum die Grace-Periode wichtig ist.

Das Manifest setzt keine CPU- oder Speicher-`resources` auf dem Runner-Container. Fügen Sie einen Block hinzu, der für die Kapazität dimensioniert ist, die Sie ausführen, wie [Dimensionieren Sie CPU und Speicher für Sitzungen](#size-cpu-and-memory-for-sessions) beschreibt.

```yaml theme={null}
apiVersion: apps/v1
kind: Deployment
metadata:
  name: claude-runner
  namespace: claude-runners
spec:
  replicas: 3
  selector:
    matchLabels:
      app: claude-runner
  template:
    metadata:
      labels:
        app: claude-runner
        app.kubernetes.io/part-of: claude-code-self-hosted-runner
    spec:
      terminationGracePeriodSeconds: 90
      containers:
        - name: runner
          image: <your-registry>/claude-runner:latest
          args:
            - self-hosted-runner
            - --environment-secret-file
            - /etc/claude/environment-secret
            - --capacity
            - "4"
          volumeMounts:
            - name: environment-secret
              mountPath: /etc/claude
              readOnly: true
          ports:
            - name: health
              containerPort: 8080
          readinessProbe:
            httpGet:
              path: /healthz
              port: 8080
            initialDelaySeconds: 5
            periodSeconds: 10
          livenessProbe:
            httpGet:
              path: /healthz
              port: 8080
            initialDelaySeconds: 30
            periodSeconds: 30
      volumes:
        - name: environment-secret
          secret:
            secretName: claude-runner-environment-secret
```

Die Deployment oben lebt in einem `claude-runners`-Namespace. Erstellen Sie den Namespace zuerst:

```bash theme={null}
kubectl create namespace claude-runners
```

Erstellen Sie das Backing-Secret aus einer lokalen Datei, die den Wert enthält, den Sie im Admin-UI's [**Copy environment key**-Schritt](/docs/de/self-hosted-environments-quickstart#set-up-an-environment-and-runner) kopiert haben, daher erscheint das Geheimnis niemals in Ihrer Shell-Historie. Führen Sie `(umask 077 && cat > ./environment-secret)` aus, fügen Sie das Geheimnis ein, drücken Sie Enter, dann Ctrl-D. Dann erstellen Sie das Secret und löschen die Datei:

```bash theme={null}
kubectl create secret generic claude-runner-environment-secret -n claude-runners --from-file=environment-secret=./environment-secret
```

<h2 id="docker-compose">
  Docker Compose
</h2>

Der Compose-Service unten startet den Runner neu, wann immer er beendet wird, was sowohl Crashes als auch den normalen Exit nach dem Draining abdeckt. Eine Docker-Restart-Richtlinie startet den gleichen Container mit seiner beschreibbaren Schicht intakt neu, daher kommt der Runner auf einem wiederverwendeten Dateisystem anstatt dem frischen, das die [Härtungs-Haltung](#harden-your-deployment) empfiehlt; verwenden Sie dieses Rezept zur Evaluierung, und für die Produktion entweder den Container pro Lauf neu erstellen oder einen Orchestrator verwenden, der das tut.

```yaml theme={null}
services:
  claude-runner:
    image: <your-registry>/claude-runner:latest
    command:
      - self-hosted-runner
      - --environment-secret-file
      - /run/secrets/environment-secret
      - --capacity
      - "4"
    secrets:
      - environment-secret
    restart: always
    stop_grace_period: 90s

secrets:
  environment-secret:
    file: ./environment-secret
```

<h2 id="shutdown-timing">
  Shutdown-Timing
</h2>

Bei `SIGTERM` stoppt der Runner, neue Arbeit zu nehmen und, es sei denn, Sie setzen [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal), wartet bis zu `--drain-wait-sec`, null standardmäßig, damit laufende Turns fertig werden, beendet jeden Sitzungs-Prozessbaum und führt den [`post-session`-Lebenszyklus-Hook](/docs/de/self-hosted-environments-configuration#post-session) aus. Dieser Prozessbaum enthält Befehle, die Claude noch in der Sitzung ausführte.

Der vollständige Drain-Pfad benötigt bis zu `--session-stop-grace-sec` + `--drain-wait-sec` + `--post-session-hook-timeout-sec`, plus 15 Sekunden fester Overhead für Prozess-Cleanup, plus 30 weitere Sekunden wenn [`--push-outcome-on-release`](/docs/de/self-hosted-environments-reference#runner-cli-flags) gesetzt ist. Das sind 80 Sekunden bei Defaults, und der Runner protokolliert das Gesamte beim Start. Sitzungen drainieren parallel unter diesem einen Budget, daher wächst das Gesamte nicht mit `--capacity`.

Bei dem Standard `--drain-wait-sec 0` unterbricht ein Rolling Restart laufende Turns; jede Sitzung setzt sich auf einem anderen Runner fort, verliert unpushed Arbeit wie unter [Bekannte Probleme](#additional-limitations) beschrieben. Setzen Sie `--drain-wait-sec` und erhöhen Sie die Grace-Periode, um zu passen, um Turns zuerst fertig zu lassen.

Während des ganzen Pfads heartbeatet der Runner zur Kontrollebene bei null Kapazität, daher läuft das Sitzungs-Lease nicht ab und wird zu einem anderen Runner requeued, während der `post-session`-Hook immer noch uncommitted Arbeit schreibt. Der Heartbeat stoppt kurz bevor der Runner sich abmeldet.

Geben Sie dem Runner mindestens das Gesamte, das er beim Start protokolliert, bevor der Host ihn stoppt. Wo Sie das setzen, hängt davon ab, wie Ihre Hosts stoppen:

* **Mit einer `SIGTERM`-Grace-Periode**: Setzen Sie `terminationGracePeriodSeconds` auf Kubernetes, `stop_grace_period` auf Docker Compose oder das Äquivalent Ihres Orchestrators auf mindestens dieses Gesamte. Der Kubernetes-Standard von 30 Sekunden ist kürzer als der Drain-Pfad des Runners, daher stoppt Kubernetes den Pod, bevor der Runner fertig drainiert.
* **Mit [`--retire-at`](/docs/de/self-hosted-environments-reference#runner-cli-flags)**: Dimensionieren Sie die Marge zwischen der Retire-Zeit und der Host-Stop-Zeit, um typische Turns zu decken, plus den Background-Task-Hold, den [Runner-Lebenszyklus](/docs/de/self-hosted-environments#runner-lifecycle) beschreibt, plus dieses gleiche Gesamte. Berechnen Sie die Retire-Zeit bei jedem Launch, zum Beispiel `date +%s` plus die beabsichtigte Lebensdauer des Runners.
* **Mit [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal)**: Fügen Sie zwei weitere Teile zum Drain-Pfad-Gesamte hinzu. Der erste ist die Minuten, die Sie konfigurieren. Der zweite ist die Post-Release-Grace, die [Defer the drain past the first signal](#defer-the-drain-past-the-first-signal) beschreibt, 75 Sekunden bei Defaults. Mit dem Flag gesetzt, druckt der Runner auch die kombinierte Zahl beim Start, nach dem Drain-Pfad-Gesamte.

<h3 id="defer-the-drain-past-the-first-signal">
  Verschieben Sie den Drain über das erste Signal hinaus
</h3>

Setzen Sie [`--defer-shutdown-max-min <n>`](/docs/de/self-hosted-environments-reference#runner-cli-flags), wenn Sie einen Runner, den Sie neu starten, für bis zu `n` Minuten weiterhin die Sitzungen bedienen möchten, die er hält, anstatt sie beim ersten Signal zu drainieren. Beim ersten `SIGTERM` oder `SIGINT` stoppt der Runner, neue Arbeit zu nehmen und bedient weiterhin die Sitzungen, die er hält. Er pollt weiterhin, daher requeued die Kontrollebene diese Sitzungen nicht. Erfordert Claude Code v2.1.238 oder später.

<h4 id="what-happens-to-the-sessions-the-runner-holds-after-the-first-signal">
  Was mit den Sitzungen passiert, die der Runner nach dem ersten Signal hält
</h4>

In den ersten zwei Phasen, die dem Signal folgen, gibt der Runner Sitzungen frei, und eine freigegebene Sitzung setzt sich auf einem frischen Runner fort, wenn sein Benutzer seine nächste Nachricht sendet. Zählung vom ersten Signal, bewegt sich der Runner durch drei Phasen:

* **Für die ersten `n` Minuten**: Der Runner bedient seine Sitzungen normal und erzwingt weiterhin `--startup-timeout-min` und `--kill-session-after-min`. Wenn Sie auch [`--release-idle-session-min`](/docs/de/self-hosted-environments-reference#runner-cli-flags) setzen, gibt der Runner jede Sitzung frei, deren Benutzer so lange untätig war; ohne es gibt der Runner keine Sitzung früh frei, außer einem Startup-Timeout.
* **Wenn die `n` Minuten ablaufen**: Der Runner gibt jede Sitzung frei, die er noch hält, untätig oder nicht. Der Runner wartet auf die Turn einer Mid-Turn-Sitzung, um zu enden, und bis zu 60 Sekunden mehr für die Background-Tasks einer Turn, bevor er diese Sitzung freigibt.
* **Wenn die Post-Release-Grace abläuft**: Der Runner drainiert alle Sitzungen, die er noch hält, und die Kontrollebene requeued jede drainierte Sitzung sofort zu einem anderen Runner. Die Post-Release-Grace startet, wenn die `n` Minuten ablaufen und ist 75 Sekunden bei Defaults. Wenn Sie `--drain-wait-sec` über 60 Sekunden setzen, ist die Post-Release-Grace `--drain-wait-sec` plus 15 Sekunden stattdessen.

In jeder Phase beendet sich der Runner 0, sobald er keine Sitzungen hält. Ein zweites Signal verkürzt die Phasen: Der Runner drainiert sofort, wie er beim ersten Signal ohne `--defer-shutdown-max-min` tut. Sobald ein Drain unterwegs ist, beendet das nächste Signal den Runner mit Gewalt. Das gilt, ob ein zweites Signal oder die Post-Release-Grace ablaufen den Drain startete.

<h4 id="size-the-stop-timeout">
  Dimensionieren Sie das Stop-Timeout
</h4>

Geben Sie Ihrem Host's Stop-Timeout mindestens die Summe von drei Teilen: die `n` Minuten, die Sie konfigurieren, die Post-Release-Grace und den vollständigen Drain-Pfad, den [Shutdown-Timing](#shutdown-timing) beschreibt. Mit Standard-Einstellungen ist die Post-Release-Grace 75 Sekunden und der Drain-Pfad 80 Sekunden, daher erlauben Sie `n` Minuten plus 155 Sekunden. Der Runner druckt diese Summe beim Start, wann immer `--defer-shutdown-max-min` gesetzt ist.

Wenn das Stop-Timeout abläuft, bevor der Runner fertig ist, tötet der Host den Runner. Die Sitzungen, die er noch hält, bekommen keinen `post-session`-Hook. Der Runner meldet sich nicht ab, und die Kontrollebene requeued die Sitzungen etwa eine Minute später. Wenn Sie das Stop-Timeout nicht diese Summe geben können, lassen Sie `--defer-shutdown-max-min` ungesetzt, daher drainiert der Runner beim ersten Signal stattdessen.

<h3 id="what-reaches-a-running-post-session-hook">
  Was einen laufenden Post-Session-Hook erreicht
</h3>

Der `post-session`-Hook und das Claude-Sitzungs-Kind laufen jeweils in ihrer eigenen POSIX-Prozessgruppe, getrennt vom Runner's, daher erreichen Stop-Mechanismen sie unterschiedlich:

* **Ein `SIGTERM`, während der Runner bereits drainiert**: Beendet den Runner sofort mit Gewalt, überspringt alles, was vom Drain-Pfad bleibt. Ohne [`--defer-shutdown-max-min`](#defer-the-drain-past-the-first-signal) ist das das zweite `SIGTERM`, das der Runner erhält. Nichts signalisiert einen laufenden `post-session`-Hook, daher auf einem bloßen Host, wo ein Init-Prozess Waisen adoptiert, beendet er sich selbst, aber unüberwacht: sein Timeout-Budget gilt nicht mehr, und ein Schreiben in die geschlossene Log-Pipe kann ihn mit `SIGPIPE` töten, daher sollte ein Hook, der einen erzwungenen Exit dort überleben muss, seine eigene Ausgabe zu einer Datei umleiten. In den Container-Rezepten auf dieser Seite ist der Runner der Container's PID 1 und sein Exit beendet den Container, und unter systemd's Standard `KillMode=control-group` erreicht die Cgroup-weite Kill den Hook auch, wie der **Cgroup-weite Kills**-Eintrag beschreibt; in beiden behandeln Sie einen erzwungenen Exit als tödlich für den Hook und verlassen sich stattdessen auf die Grace-Periode.
* **Prozessgruppen-weite Signale**, wie `kill -- -<pid>` in einem Wrapper-Skript, Shell-Job-Kontrolle oder ein Gruppen-weiter Watchdog: erreichen den Runner und einen Mid-`checkout`-Hook-Subprozess, der absichtlich Gruppen-angehängt bleibt, aber nicht einen laufenden `post-session`-Hook oder das Sitzungs-Kind.
* **Cgroup-weite Kills**, wie systemd's Standard `KillMode=control-group` oder die `SIGKILL`, die Kubernetes dem ganzen Container liefert, wenn `terminationGracePeriodSeconds` abläuft: erreichen alles, einschließlich des Hooks. Prozessgruppen-Isolation schützt nicht gegen diese, weshalb die Grace-Periode den vollständigen Drain-Pfad decken muss.
* **Das Timeout des Hooks selbst**: Wenn ein Hook `--post-session-hook-timeout-sec` überschreitet, sendet der Runner `SIGTERM` zur ganzen Prozessgruppe des Hooks, dann `SIGKILL` zwei Sekunden später, daher beendet sich ein Worker, den der Hook forked, wie tar, rsync oder git, mit der Wrapper-Shell anstatt als Waise zu überleben. Die Überwachung des Runners endet, sobald das Hook's stdio schließt: ein Worker, der seine eigene Ausgabe zu einer Datei umgeleitet hat und die `SIGTERM`-Phase überlebt, ist jenseits der Reichweite des Runners.

Wenn der Drain startet, und wieder bei einem erzwungenen Exit, protokolliert der Runner, wie viele `post-session`-Hooks noch laufen, daher können Sie einen stillen Drain von einem unterscheiden, der Mid-Snapshot ist.

<h2 id="keep-the-base-directory-and-capacity-identical-across-runners">
  Halten Sie das Basis-Verzeichnis und die Kapazität über Runner identisch
</h2>

Wenn ein Runner Mid-Sitzung stirbt, requeued der Server die Sitzung und ein anderer Runner in der Umgebung hebt sie auf. Dieser Runner leitet den Checkout-Pfad von seinem eigenen `--base-dir` und `--capacity` ab: `--capacity 1` checkt direkt unter `--base-dir` aus, und ein `--capacity` über `1` verwendet stattdessen Pro-Sitzungs-Worktrees. Wenn Runner in der gleichen Umgebung unterschiedliche Werte für eines der Flags verwenden, ändert sich das Arbeitsverzeichnis der fortgesetzten Sitzung, und absolute Pfade, die der Agent früher aufgezeichnet hat, in Edits, Tool-Aufrufen oder seinen eigenen Notizen, zeigen auf einen Ort, der nicht mehr existiert.

Verwenden Sie den gleichen `--base-dir` und `--capacity` auf jedem Runner in einer Umgebung, und verwenden Sie keinen Pro-Host-Wert wie eine Instance-ID oder einen Hostnamen.

Das Basis-Verzeichnis standardmäßig auf `/workspace`, mit der Ausnahme, die die [`--base-dir`-Referenz-Zeile](/docs/de/self-hosted-environments-reference#runner-cli-flags) aufzeichnet. Der Runner benötigt Schreibzugriff darauf. Beim Start, vor der Registrierung, erstellt der Runner das Verzeichnis und bestätigt, dass er darin schreiben kann, und beendet sich mit `cannot create or write to base directory`, wenn er nicht kann. Ein Runner, der als Root gestartet wird, erstellt das Standard `/workspace` selbst. Für einen Non-Root-Runner erstellen Sie das Verzeichnis und geben Sie dem Runner's Benutzer Eigentum, bevor Sie den Runner starten, oder zeigen Sie `--base-dir` auf ein Verzeichnis, das dieser Benutzer bereits besitzt.

<h2 id="reuse-a-pre-warmed-checkout">
  Wiederverwendung eines vorgewärmten Checkouts
</h2>

Bei großen Repositories kann das Klonen den Sitzungsstart dominieren. Bei `--capacity 1` ohne [`checkout` Hook](/docs/de/self-hosted-environments-configuration#checkout) behält der Runner einen kanonischen Klon pro Repository unter `<base-dir>/<repo-owner>/<repo>` und verwendet ihn über Sitzungen hinweg erneut: Er ruft die angeforderte Referenz ab, trennt `HEAD` ab und setzt sie hart zurück, was nahezu augenblicklich ist, wenn sich wenig geändert hat. Um das kalte Klonen zu überspringen, stellen Sie den Klon auf eine von zwei Arten bereit:

* **Klon im Image**: Erstellen Sie den Klon in Ihrem Runner-Image unter diesem Pfad. Jeder neue Container startet dann mit dem warmen Klon, ohne eine Festplatte wiederzuverwenden.
* **Klon auf einem persistenten Volume**: Bei Runnern, die Sie mit [`--lock-to-account`](/docs/de/self-hosted-environments-reference#runner-cli-flags) auf das Konto eines Benutzers sperren, verweisen Sie `--base-dir` auf ein persistentes Volume, sodass die Festplatte nur diesem Konto dient. Ein gesperrter Runner akzeptiert niemals Claude Tag Channel-Sitzungen, daher gilt diese Option nicht für Runner, die diese bedienen.

Was der Wiederverwendungspfad garantiert und nicht garantiert:

* **Jede Klonform funktioniert**: Ein vollständiger, flacher oder Single-Branch-Klon unter dem Pfad wird unverändert verwendet. Der Runner übergibt niemals `--depth` beim Abrufen in einen vorhandenen Klon, daher behält ein vollständiger Vorwärm seine vollständige Historie und ein flacher bleibt flach. `CLAUDE_RUNNER_FETCH_DEPTH` (`full`, `0` oder eine Zahl; Standard 50) steuert nur den kalten Klon, den der Runner erstellt, wenn noch kein Klon vorhanden ist.
* **Nachverfollgte Änderungen werden zurückgesetzt, nicht nachverfolgte Dateien bleiben erhalten**: Jede Sitzung beginnt mit einem harten Zurücksetzen, das die verfolgten Änderungen der vorherigen Sitzung löscht, aber der Runner führt niemals `git clean` aus, daher bleiben nicht nachverfolgte Dateien aus früheren Sitzungen des gesperrten Besitzers im Baum.
* **Per-Session-Verzeichnisse bleiben ebenfalls erhalten**: Neben dem Checkout erstellt der Runner für jede Sitzung, die er ausführt, Einträge pro Sitzung unter `<base-dir>/_sessions/`. Das Claude-Konfigurationsverzeichnis der Sitzung enthält eine lokale Kopie des Gesprächstranskripts. Daneben befinden sich die hochgeladenen Dateien der Sitzung, falls die Sitzung welche hat. Das Sitzungsverzeichnis befindet sich auch dort: Es enthält alle Pro-Session-Worktrees und `checkout` Hook-Checkouts während der Sitzung ausgeführt wird, und es behält alles andere, was Claude darin geschrieben hat.

  Standardmäßig lässt der Runner diese an Ort und Stelle, wenn die Sitzung endet, daher sammeln sie sich auf einer Festplatte an, die den Runner-Prozess überlebt. Jede Sitzung wird als eigener Benutzer des Runners ausgeführt, daher kann jede spätere Sitzung, die diese Festplatte bedient, sie lesen. Wenn Sie ein persistentes `--base-dir` beibehalten, dimensionieren Sie das Volume für dieses Wachstum. Das Gleiche gilt für jedes Setup, das den Runner auf demselben Dateisystem neu startet, einschließlich des [Docker Compose-Rezepts](#docker-compose).
* **Mit `--remove-session-state` bleiben Per-Session-Verzeichnisse nicht erhalten**: Starten Sie den Runner mit [`--remove-session-state`](/docs/de/self-hosted-environments-reference#runner-cli-flags), um ihn dazu zu bringen, die Per-Session-Verzeichnisse jeder Sitzung zu löschen, wenn die Sitzung endet. Das Löschen ist bestmöglich: Die Verzeichnisse bleiben erhalten, wenn der Runner beendet wird, bevor seine Bereinigung ausgeführt wird. Der kanonische Klon und Dateien, die eine Sitzung an anderer Stelle auf dem Host geschrieben hat, wie das temporäre Verzeichnis, bleiben unabhängig davon erhalten.
* **Mit dem Git-Proxy wird das Zurücksetzen zu einem Checkout**: Mit [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy) bereinigt der Runner das `.git/`-Verzeichnis des Klons vor jeder Sitzung, behält den Objektspeicher, Referenzen und den flachen Zustand, löscht aber den Index, sodass jede Sitzung einen vollständigen Working-Tree-Checkout anstelle eines nahezu augenblicklichen Zurücksetzen zahlt; es wird immer noch nie neu geklont. Submodul-Vorwärme wird unter dem Proxy nicht unterstützt.
* **Lange Klone benötigen keine Umgehung**: Der Runner begrenzt jede Git-Operation mit einem 120-Sekunden-Watchdog ohne Fortschritt und einer 30-Minuten-Obergrenze, nicht mit einem flachen Timeout, daher wird ein langsamer kalter Klon, der weiterhin Fortschritt meldet, abgeschlossen.

<h2 id="pin-the-version">
  Pinnen Sie die Version
</h2>

Jeder Sitzungs-Kind-Claude-Code-Prozess führt die Binärdatei des Runners selbst aus, und der Runner schaltet Auto-Update in den Sitzungen, die er spawnt, aus, daher führt jede Sitzung die Version aus, die Sie auf dem Host installiert oder in das Image eingebaut haben. Ein Host-Level-Update wird wirksam, das nächste Mal, wenn der Runner startet.

* **Um eine Flotte auf einer Version zu halten**: Erstellen Sie das Image mit einer gepinnten Version, oder installieren Sie auf einem bloßen Host eine spezifische Version und [deaktivieren Sie Auto-Updates](/docs/de/setup#disable-auto-updates)
* **Um zu upgraden**: Installieren Sie die neuere Version oder erstellen Sie das Image neu, dann starten Sie die Runner neu
* **Plugins**: Plugin-Marktplätze auto-updaten auch nicht; setzen Sie `FORCE_AUTOUPDATE_PLUGINS=1` in der Runner's Umgebung, um Plugins auto-updaten zu lassen, während die Binärdatei gepinnt bleibt

<h2 id="scale-the-fleet">
  Skalieren Sie die Flotte
</h2>

Ihr Orchestrator entscheidet, wann Runner hinzugefügt oder entfernt werden. Wegen der [One-Owner-Per-Runner-Lock](/docs/de/self-hosted-environments#runner-lifecycle) ist die minimale Replica-Anzahl die Anzahl der Benutzer und Claude Tag-Agenten, die Sie gleichzeitig aktiv erwarten; `--capacity` kontrolliert Parallelismus innerhalb einer Owner's Sitzungen, nicht über Owners.

Zwei Skalierungs-Ansätze sind verfügbar:

* **Feste Flotte**: Führen Sie einen statischen Satz von Runner-Replicas aus und skalieren Sie auf den [Prometheus-Metriken](/docs/de/self-hosted-environments-reference#prometheus-metrics), die jeder Runner bedient
* **On-Demand-Runner**: Führen Sie den `claude self-hosted-runner orchestrator`-Subcommand aus, der Anthropic auf Sitzungen abfragt, die mit keinem verfügbaren Runner in die Warteschlange eingereiht sind und Ihren `spawn-runner`-Hook aufruft, um einen pro Sitzung zu starten. Siehe [On-Demand-Runner](/docs/de/self-hosted-environments-configuration#on-demand-runners).

<h2 id="known-issues-and-limitations">
  Bekannte Probleme und Einschränkungen
</h2>

Die folgenden sind die Einschränkungen in dieser Version, mit Umgehungen, wo eine existiert.

<h3 id="connector-traffic-leaves-your-network">
  Connector-Datenverkehr verlässt Ihr Netzwerk
</h3>

Anthropic ruft Connector-Tools von seiner eigenen Infrastruktur anstatt von Ihrem Runner auf. Connector-Tools sind die claude.ai-Connectoren, wie GitHub, Slack und Linear. Wenn Claude einen Connector in einer selbstgehosteten Sitzung verwendet, geht dieser Datenverkehr durch `api.anthropic.com` anstatt von innerhalb Ihrer Netzwerkgrenze zu stammen.

Um einen Connector aus selbstgehosteten Sitzungen zu halten, filtern Sie ihn mit den [`allowedMcpServers`- und `deniedMcpServers`-Richtlinien-Einstellungen](/docs/de/managed-mcp#policy-based-control-with-allowlists-and-denylists). Claude Code wendet diese Einstellungen auf die Connectoren an, die Anthropic liefert, sowie auf die Server, die Sie vom Runner-Host seeden, und die Server, die Benutzer hinzufügen. Wenn Sie also eine Zulassungsliste für andere Server bereitstellen, blockiert Claude Code auch gelieferte Connectoren. Um Connectoren neben einer URL-basierten Zulassungsliste verfügbar zu halten, fügen Sie Einträge hinzu, die den Anthropic-Proxy-Pfaden für gelieferte Connectoren entsprechen:

* `https://api.anthropic.com/v2/ccr-sessions/*`
* `https://api.anthropic.com/v1/code/sessions/*`
* `https://api.anthropic.com/v1/code/mcp/*`

Wenn Tool-Datenverkehr in Ihrem Netzwerk bleiben muss, führen Sie die äquivalenten Tools als lokale MCP-Server auf dem Runner-Image stattdessen aus. Siehe [MCP-Server](/docs/de/self-hosted-environments-configuration#mcp-servers).

<h3 id="some-sessions-don’t-count-as-idle">
  Einige Sitzungen zählen nicht als untätig
</h3>

Eine Sitzung, die einen Hintergrund-Task hält, der niemals fertig wird, zählt nicht als untätig, daher wird `--release-idle-session-min` diese Sitzung nicht freigeben. Eine Sitzung, die auf eine Genehmigung wartet, die von innerhalb eines laufenden Tool-Aufrufs angefordert wird, zählt auch nicht als untätig. Setzen Sie immer `--kill-session-after-min` daneben als harten Backstop, damit keine Sitzung einen Slot unbegrenzt halten kann.

`--kill-session-after-min` ist ein Backstop für Runaway-Sitzungen. Auf einem Runner mit v2.1.260 oder später wird eine Sitzung, die das Limit erreicht, nicht sofort beendet. Der Runner gibt ihr ein Kulanzfenster, standardmäßig 15 Minuten, das Sie mit [`SELF_HOSTED_RUNNER_MAX_LIFETIME_GRACE_MS`](/docs/de/self-hosted-environments-reference#environment-variable-only-settings) ändern können:

* Wenn die Sitzung auf ihren Benutzer wartet, gibt der Runner sie frei. Wenn ihr Zug beendet ist und sie hält nur Hintergrund-Tasks, wartet der Runner bis zu 60 Sekunden, damit diese Tasks fertig werden, und gibt sie dann frei. Die Sitzung wird fortgesetzt, wenn ihr Benutzer ihre nächste Nachricht sendet.
* Wenn ein Zug noch läuft, wartet der Runner darauf, dass der Zug fertig wird, oder dass die Sitzung das nächste Mal auf ihren Benutzer wartet, und gibt sie dann frei.
* Wenn die Sitzung noch auf dem Runner ist, wenn das Kulanzfenster endet, beendet der Runner sie, und jede laufende Zug-Arbeit geht verloren. Ein Zug, der auf eine Genehmigung wartet, die von innerhalb eines laufenden Tool-Aufrufs angefordert wird, ist eine Möglichkeit, wie eine Sitzung das Fenster überlebt.

Eine freigegebene Sitzung wird von einem frischen Klon fortgesetzt, daher ist Arbeit, die sie nicht gepusht hat, sowieso weg; siehe [Fortgesetzte Sitzungen verlieren unpushed Arbeit](#additional-limitations). Vor v2.1.260 beendete der Runner jede Sitzung beim Limit, nachdem er höchstens das Kulanzfenster wartete, damit ein laufender Zug fertig wird.

Setzen Sie das Flag über Ihre längste erwartete Sitzung, wie `--kill-session-after-min 480` für 8 Stunden. Um Slots aus Gesprächen freizugeben, die untätig werden, verwenden Sie stattdessen `--release-idle-session-min`.

<h3 id="additional-limitations">
  Zusätzliche Einschränkungen
</h3>

* **Fortgesetzte Sitzungen verlieren unpushed Arbeit**: Wenn eine Sitzung freigegeben wird oder ihr Runner neu gestartet wird, und der Benutzer sendet eine weitere Nachricht, wird die Sitzung auf einem frischen Runner fortgesetzt, der das Repository erneut von seinem Starting-Branch klont, daher ist Arbeit, die die Sitzung nicht gepusht hat, weg. Setzen Sie [`--push-outcome-on-release`](/docs/de/self-hosted-environments-reference#runner-cli-flags), um den Runner einen Best-Effort-Push der Sitzungs-Outcome-Branches vor der Freigabe machen zu lassen, damit die fortgesetzte Sitzung von diesen Commits startet; dies bewahrt committed Arbeit, nicht einen Dirty Working Tree. Bevor Sie es aktivieren, beschränken Sie, wer zu `claude/*`-Refs auf dem Source-Remote pushen kann, zum Beispiel mit einem Branch-Ruleset: Bei Resume ruft der Runner den zuvor gepushten Branch ab, ohne zu überprüfen, wer ihn gepusht hat, daher kann jeder mit Push-Zugriff zu diesen Refs Inhalt in den fortgesetzten Workspace platzieren. Der Runner verwirft auch Pro-Sitzungs-Konfiguration bei Resume, was das Claude-Konfigurationsverzeichnis der Sitzung und jeden Shell-Zustand bedeutet, den die Sitzung schrieb; `--push-outcome-on-release` deckt diese nicht ab.
* **Private Repositorys können nicht Mid-Sitzung hinzugefügt werden**: Ein Repository, das zu einer Sitzung hinzugefügt wird, nachdem es gestartet hat, wird nicht mit Anmeldedaten auf einem selbstgehosteten Runner geklont, daher schlägt das Hinzufügen fehl. Wählen Sie jedes Repository, das die Sitzung benötigt, wenn Sie sie erstellen.
* **Einige Connectoren erscheinen nicht in selbstgehosteten Sitzungen**: Ein Connector, den Sie noch nicht in claude.ai Settings verbunden haben, wird nicht in einer selbstgehosteten Sitzung aufgelistet, und die Sitzung wird Sie nicht auffordern, ihn zu verbinden. Verbinden Sie ihn zuerst in Settings, dann starten Sie eine frische Sitzung. Das Hinzufügen eines Connectors zu einer bereits laufenden Sitzung macht seine Tools auch nicht für Claude verfügbar; starten Sie eine frische Sitzung, um einen neu hinzugefügten Connector aufzugreifen.

<h3 id="report-an-issue">
  Melden Sie ein Problem
</h3>

Für Probleme mit selbstgehosteten Umgebungen kontaktieren Sie Ihr Anthropic-Account-Team.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

Für geführte Diagnose führen Sie den Doctor-Subcommand auf dem Runner-Host aus. Der Doctor-Subcommand startet eine interaktive Claude Code-Sitzung mit den Logs und dem Zustand des Runners angehängt. Melden Sie sich zuerst mit `claude auth login` auf diesem Host an, daher kann die Sitzung Ihre Umgebung, ihre Runner und ihre in die Warteschlange eingereihten Sitzungen abfragen. Ohne diese Anmeldung, zum Beispiel wenn der Host mit einem API-Schlüssel authentifiziert, ist es auf den lokalen Health-Endpunkt, Metriken und das Runner-Log begrenzt, und es liest das Log nur, wenn Sie den Runner mit `--log-file` gestartet haben.

```bash theme={null}
claude self-hosted-runner doctor
```

Häufige Probleme:

* **Runner erscheint nicht in der Umgebung**: Bestätigen Sie, dass der Host `api.anthropic.com` über HTTPS erreichen kann, das Umgebungsgeheimnis aktuell ist und die Host-Uhr innerhalb von fünf Minuten der echten Zeit liegt; größere Abweichung verursacht, dass die Authentifizierung fehlschlägt. Der Runner protokolliert `[runner:fatal]` mit dem Ablehnungsgrund bei Auth-Fehler.
* **Runner beendet sich beim Start mit `cannot create or write to base directory`**: Der Runner kann `--base-dir` nicht erstellen oder schreiben, das standardmäßig auf `/workspace` ist. Beheben Sie das Verzeichnis's Eigentum oder zeigen Sie `--base-dir` auf einen beschreibbaren Pfad, wie in [Halten Sie das Basis-Verzeichnis und die Kapazität über Runner identisch](#keep-the-base-directory-and-capacity-identical-across-runners) beschrieben. Wenn der Runner stattdessen `[runner:fatal]` protokolliert, dass die Basis-Verzeichnis-Überprüfung abgelaufen ist, ist das Verzeichnis auf einem hängenden NFS- oder CSI-Mount. Überprüfen Sie die Mount-Gesundheit anstatt der Berechtigungen. Der Runner druckt beide dieser Startup-Fehler zu stderr, bevor er `--log-file` öffnet, daher suchen Sie nach ihnen im Terminal oder Ihren Plattform-Container-Logs anstatt der Log-Datei. Vor v2.1.225 überprüfte der Runner das Basis-Verzeichnis nicht beim Start, und diese Fehlkonfiguration schlug Sitzungen nach der Aufnahme fehl.
* **Sitzungen bleiben in der Warteschlange**: Jeder Online-Runner kann auf einen anderen Owner gesperrt sein. Überprüfen Sie die `claude_code_self_hosted_runner_locked_account`-[Metrik](/docs/de/self-hosted-environments-reference#prometheus-metrics) jedes Runners oder das `locked_account`-Feld seiner `[runner:health]`-Log-Zeile, um zu sehen, wer sie hält. Beide zeigen die Email des Owners nur, nachdem der Runner ein Sitzungs-Token mit einem `act.email`-Claim ausgestellt bekommen hat, das die Sitzungen eines Claude Tag-Agenten niemals tun. Ohne den Claim sendet der Runner keine `locked_account`-Serie aus und protokolliert `locked_account=yes`, was Ihnen sagt, dass der Runner gesperrt ist, aber nicht auf welchen Owner. Fügen Sie Replicas hinzu, oder warten Sie, bis ein bestehender Runner drainiert und neu startet. Wenn die Umgebung On-Demand-Runner verwendet, überprüfen Sie stattdessen den Orchestrator; siehe [On-Demand-Runner](/docs/de/self-hosted-environments-configuration#on-demand-runners).
* **Sitzungen schlagen sofort nach der Aufnahme fehl**: Öffnen Sie die Sitzung in claude.ai/code, um den Fehler zu sehen. Die häufigsten Ursachen sind fehlende [Git-Anmeldedaten](#configure-git) im Runner-Image und Build-Tools, die nicht installiert sind. Ein nicht beschreibbares Basis-Verzeichnis stoppt den Runner beim Start anstatt Sitzungen zu fehlschlagen. Siehe den **Runner beendet sich beim Start mit `cannot create or write to base directory`**-Eintrag in dieser Liste.
* **Sitzungen können das Netzwerk nicht durch einen authentifizierenden Egress-Proxy erreichen**: Wenn die Quelle, die Sie mit [`--proxy-authorization-command` oder `--proxy-authorization-file`](#authenticate-to-an-egress-proxy) setzen, fehlschlägt, nach 30 Sekunden abläuft oder einen leeren Wert ergibt, antwortet der Runner dieser Verbindung `502 Bad Gateway` und protokolliert warum. Der Runner redigiert das Kommando's stderr in diesem Log und protokolliert niemals den Header-Wert. Mit `--proxy-authorization-command` führen Sie das Kommando selbst auf dem Host aus, um zu bestätigen, dass es den ganzen Header-Wert auf stdout druckt. Wenn der Runner stattdessen beim Start mit `could not start the proxy-authorization listener` beendet wird, konnte er seinen Loopback-Listener nicht öffnen.
* **Runner protokolliert `Poll failed`-Zeilen, die `rejecting the malformed poll response` enthalten**: Der Runner erhielt eine Work-Poll-Antwort, deren Body nicht das erwartete JSON der Warteschlange ist, am häufigsten weil etwas zwischen dem Runner und `api.anthropic.com`, wie ein abfangender Proxy oder ein Captive Portal, seine eigene Seite antwortet. Der Runner lehnt die Antwort ab, zählt sie unter der `transport`-Art der `claude_code_self_hosted_runner_poll_errors_total`-[Metrik](/docs/de/self-hosted-environments-reference#prometheus-metrics), und versucht erneut auf dem fehlgeschlagenen Poll-Plan, der in [Sitzungs-Lebenszyklus](/docs/de/self-hosted-environments#session-lifecycle) beschrieben ist. Der Runner bedient weiterhin seine Live-Sitzungen. Konfigurieren Sie den Proxy, um Antworten von `api.anthropic.com` unverändert durchzulassen. Vor v2.1.246 las der Runner eine solche Antwort als eine leere Warteschlange, die seine Live-Sitzungen beenden oder ihn zum Exit bringen könnte.
* **Ein Sitzungs-Branch existiert nicht mehr auf dem Remote**: Für eine Git-Quelle, die die Sitzung nur liest, überspringt der Runner diese Quelle und setzt auf den verbleibenden fort. Für die Quelle, zu der die Sitzung Ergebnisse pusht, schlägt ein gelöschter Branch, typischerweise weil er gemergt und auto-gelöscht wurde, die Sitzung mit einem Fehler fehl, der das Repository und den Branch benennt und Sie auffordert, den Branch wiederherzustellen und erneut zu versuchen. Der Runner schlägt die Sitzung mit dem gleichen Fehler fehl, wenn das Überspringen sie mit keinem Repository überhaupt verlassen würde. Vor v2.1.228 startete eine solche Sitzung in einem leeren Verzeichnis.
* **Eine Sitzung startet ohne eines ihrer Repositories**: Auf einem Runner ohne [`checkout` Hook](/docs/de/self-hosted-environments-configuration#checkout) kann der Git-Host den Zugriffsprüfung des Runners für ein Repository, das die Sitzung nur liest, ablehnen. Der Runner überspringt dann dieses Repository, protokolliert eine `[runner:warn] could not access context source`-Zeile, die die Ablehnung benennt, und startet die Sitzung auf den verbleibenden.

  Der Runner überspringt nur eine klare Ablehnung: Der Host antwortet, dass das Repository nicht gefunden wurde, Git findet keine Anmeldedaten für den Host, oder die Authentifizierung schlägt fehl. Ein Netzwerkfehler, ein Timeout oder ein HTTP `403` schlägt immer noch den Sitzungsstart fehl, ebenso wie eine Ablehnung für ein Repository, zu dem die Sitzung Ergebnisse pusht. Der Runner schlägt immer noch eine Sitzung fehl, die das Überspringen ohne Repository überhaupt verlassen würde. Mit [`--use-anthropic-git-proxy`](#use-the-anthropic-git-proxy) überspringt der Runner nur ein Repository, das der Git-Proxy selbst ablehnt.

  Die Zugriffsprüfung läuft jedes Mal erneut, wenn die Sitzung auf einem Runner startet, daher sobald die Git-Identität des Runners Lesezugriff hat, klont der nächste Start das Repository. Vor v2.1.274 schlugen jede dieser Ablehnungen den Sitzungsstart fehl.
* **Sitzungen dauern Minuten zum Start**: Der anfängliche Clone dominiert normalerweise. Beobachten Sie die `claude_code_self_hosted_runner_session_init_duration_seconds`-[Metrik](/docs/de/self-hosted-environments-reference#prometheus-metrics), um zu bestätigen, und schneiden Sie den Clone mit einem [vorgewärmten Checkout](#reuse-a-pre-warmed-checkout) oder einem kleineren `CLAUDE_RUNNER_FETCH_DEPTH`.
* **Turns schlagen mit einem 401 fehl**: Jede Sitzung authentifiziert Modellaufrufe mit dem kurzlebigen [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/de/self-hosted-environments-configuration#wrapper-scripts), das der Runner von Anthropic abruft und über die Sitzungs-stdin rotiert. Wenn ein Turn mit einem 401 oder 403 von der Modell-API endet, ruft der Runner ein frisches Token ab und übergibt es der Sitzung. Der fehlgeschlagene Turn wird nicht erneut versucht.

  Wenn ein Abruf fehlschlägt, protokolliert der Runner eine `inference_token refresh failed`-Zeile, die sagt, wann er erneut versuchen wird, und er versucht es weiterhin erneut, solange die Sitzung läuft.

  Wenn jeder Aufruf etwa 30 Minuten in eine Sitzung hinein fehlschlägt, hat ein Wrapper-Skript wahrscheinlich die Sitzungs-stdin unterbrochen, daher können Token-Rotationen sie nicht erreichen; siehe [Halten Sie stdin und Dateideskriptor 3 angehängt](/docs/de/self-hosted-environments-configuration#keep-stdin-and-file-descriptor-3-attached).

  Vor v2.1.274 stoppte der Runner das erneute Versuchen eines fehlgeschlagenen Abrufs nach einigen Versuchen und wartete auf den nächsten geplanten. Ein fehlgeschlagener Turn löste keinen Abruf aus, daher schlugen alle Turns mit einem 401 fehl, bis zum nächsten geplanten Abruf.
* **Pod wird Mid-Drain getötet**: Erhöhen Sie `terminationGracePeriodSeconds` auf mindestens den Wert, den der Runner beim Start protokolliert. Siehe [Shutdown-Timing](#shutdown-timing).

Sobald das Logging initialisiert ist, schreibt der Runner sein Lebenszyklus-Log, einschließlich `[runner:fatal]`-Zeilen, zu stdout, und Debug-Ausgabe zu stderr, alles als Plain-Text-Zeilen anstatt JSON. Die Startup-Fehler, die in den Fehlerbehebungs-Einträgen oben beschrieben sind, drucken zu stderr vor diesem Punkt. Erfassen Sie beide Streams mit `--log-file`, was auch `self-hosted-runner doctor` ermöglicht, sie zu tailing, oder mit Ihrer Plattform's Log-Sammlung.

Jeder Sitzungs-Kind-Prozess schreibt ein separates Debug-Log. Bei Fehler bewahrt der Runner das Log's Tail neben der Sitzung in claude.ai/code. Sofern Sie den Runner nicht mit [`--remove-session-state`](/docs/de/self-hosted-environments-reference#runner-cli-flags) gestartet haben, behält er auch das Log einer fehlgeschlagenen Sitzung auf der Festplatte und druckt seinen Pfad im Runner-Log.

<h2 id="what’s-next">
  Was kommt als nächstes
</h2>

* [Passen Sie Sitzungen an](/docs/de/self-hosted-environments-configuration): Wrapper-Skripte, Lebenszyklus-Hooks, On-Demand-Runner, MCP-Server und Berechtigungen
* [Testen Sie End-to-End](/docs/de/self-hosted-environments-testing): Überprüfen Sie ein neues Runner-Image von CI, bevor Sie es fördern
* [Referenz](/docs/de/self-hosted-environments-reference): Jedes CLI-Flag, jede Umgebungsvariable und jede Metrik
