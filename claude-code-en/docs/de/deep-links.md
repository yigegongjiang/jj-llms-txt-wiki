> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Sitzungen über Links starten

> Öffnen Sie eine Claude Code-Terminalsitzung über eine URL. Betten Sie `claude-cli://`-Links in Runbooks, Warnungen und Dashboards ein, damit ein Klick Claude Code im richtigen Repository mit der richtigen Eingabeaufforderung öffnet.

Ein Deep Link ist eine `claude-cli://`-URL, die Claude Code in einem neuen Terminalfenster öffnet. Die URL kann ein Arbeitsverzeichnis und eine Eingabeaufforderung zum Vorausfüllen enthalten.

Dies ermöglicht es Ihnen, einen One-Click-Startpunkt für eine Aufgabe freizugeben: Jeder, der Claude Code installiert hat und auf den Link klickt, sieht eine Sitzung mit der bereits eingegebenen Eingabeaufforderung. Die Eingabeaufforderung wird ausgefüllt, aber nicht gesendet, bis Sie die Eingabetaste drücken.

Da ein Deep Link eine URL ist, können Sie ihn überall dort platzieren, wo ein Link möglich ist:

* Ein Schritt in einem Incident-Runbook, der das betroffene Service-Repository mit einer Diagnose-Eingabeaufforderung öffnet
* Eine Überwachungsmeldung oder ein Dashboard, das auf eine Untersuchungs-Eingabeaufforderung für eine bestimmte Metrik verlinkt
* Eine README- oder Wiki-Seite, die das Projekt mit einer Onboarding-Eingabeaufforderung öffnet
* Eine CI-Fehlerbenachrichtigung, die den Namen des fehlgeschlagenen Jobs vorausfüllt

Diese Seite behandelt, wie Sie [einen Link erstellen](#build-a-link), [ihn in ein Runbook einbetten oder von der Shell aus auslösen](#examples) und [die Handler-Registrierung auf jeder Plattform verwalten oder deaktivieren](#registration-and-supported-platforms).

<h2 id="how-it-works">
  Funktionsweise
</h2>

Das `claude-cli://`-Präfix ist ein benutzerdefiniertes URL-Schema, das Claude Code bei Ihrem Betriebssystem registriert, ähnlich wie `mailto:`-Links Ihren E-Mail-Client öffnen. Der Link kann auf einer Webseite, in einem Wiki, in einer Slack-Nachricht oder in jeder App vorhanden sein, die Links rendert. Wenn Sie auf einen klicken:

1. Der Browser oder die App übergibt die URL an Ihr Betriebssystem.
2. Das Betriebssystem erkennt das `claude-cli://`-Präfix und startet Claude Code auf Ihrem Computer.
3. Ein neues Terminalfenster öffnet sich mit Claude Code, das im vom Link angegebenen Verzeichnis ausgeführt wird, und der Eingabeaufforderungstext des Links befindet sich bereits im Eingabefeld.
4. Sie lesen die Eingabeaufforderung, bearbeiten sie bei Bedarf und drücken die Eingabetaste, um sie zu senden.

Der Link selbst kann überall gehostet werden, aber die Sitzung öffnet sich immer lokal auf dem Computer, auf dem Sie geklickt haben. Siehe [Registrierung und unterstützte Plattformen](#registration-and-supported-platforms), um zu erfahren, welcher Terminal-Emulator auf jedem Betriebssystem geöffnet wird.

<Note>
  Die Plattform, die den Link anzeigt, muss benutzerdefinierte URL-Schemas zulassen. Von GitHub gerendertes Markdown erlaubt `http` und `https`, entfernt aber Schemas wie `claude-cli://` in READMEs, Issues, Pull Requests und Wikis. Nur der Link-Text wird angezeigt, ohne Link dahinter und die URL ist verborgen. Siehe [Fehlerbehebung](#the-link-renders-as-plain-text-instead-of-being-clickable) für eine Problemumgehung.
</Note>

<h3 id="what-a-launched-session-shows">
  Was eine gestartete Sitzung anzeigt
</h3>

Ein Deep Link führt niemals etwas von selbst aus. Der Link wählt nur ein Verzeichnis und füllt das Eingabefeld. Wenn Sie auf einen Link von einer Seite klicken, der Sie nicht vertrauen, ist die Eingabeaufforderung immer noch inert: Nichts erreicht das Modell, bis Sie lesen, was ausgefüllt wurde, und die Eingabetaste drücken.

Wenn die Sitzung öffnet, zeigt eine Warnzeile unter dem Eingabefeld `Prompt from an external link` an und bleibt sichtbar, bis Sie die Eingabeaufforderung senden oder löschen. Bei Eingabeaufforderungen über 1.000 Zeichen enthält die Warnung die Zeichenanzahl und teilt Ihnen mit, dass Sie scrollen und den vollständigen Text überprüfen sollten, bevor Sie die Eingabetaste drücken, da lange Eingabeaufforderungen Anweisungen vom Bildschirm schieben können. Berechtigungsregeln, `CLAUDE.md` und Vertrauensaufforderungen für das ausgewählte Verzeichnis gelten genauso wie für jede andere Sitzung.

<h2 id="build-a-link">
  Einen Link erstellen
</h2>

Jeder Deep Link beginnt mit `claude-cli://open`, das ist der einzige Pfad, den der Handler akzeptiert, gefolgt von optionalen Abfrageparametern. Die minimale Form öffnet Claude Code in Ihrem Home-Verzeichnis mit einer leeren Eingabeaufforderung:

```text theme={null}
claude-cli://open
```

Fügen Sie Parameter hinzu, um zu steuern, wo die Sitzung startet und was das Eingabefeld enthält:

| Parameter | Beschreibung                                                                                                                                                                                                                                                                 |
| --------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `q`       | Text zum Vorausfüllen im Eingabefeld. [URL-kodieren](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent) Sie den Wert. Verwenden Sie `%0A` für Zeilenumbrüche in mehrzeiligen Eingabeaufforderungen. Maximal 5.000 Zeichen. |
| `cwd`     | Absoluter Pfad, der als Arbeitsverzeichnis verwendet werden soll. Netzwerk- und UNC-Pfade werden abgelehnt, ebenso wie Pfade, die unsichtbare oder bidirektionale Steuerzeichen enthalten.                                                                                   |
| `repo`    | Ein GitHub `owner/name`-Slug. Claude Code löst ihn zu einem lokalen Klon auf, den es zuvor gesehen hat, und startet dort. Wenn Sie keinen passenden Klon haben, öffnet sich die Sitzung stattdessen in Ihrem Home-Verzeichnis.                                               |

`cwd` und `repo` sind [zwei Möglichkeiten, um das Arbeitsverzeichnis festzulegen](#choose-between-cwd-and-repo). Wenn Sie beide übergeben, hat `cwd` Vorrang und `repo` wird ignoriert, auch wenn der `cwd`-Pfad nicht existiert.

Der folgende Link verweist auf ein Repository namens `acme/payments` mit einer zweilinigen Diagnose-Eingabeaufforderung. Ersetzen Sie `acme/payments` durch den `owner/name`-Slug Ihres Repositorys, wenn Sie Ihren eigenen erstellen:

```text theme={null}
claude-cli://open?repo=acme/payments&q=Investigate%20the%20failed%20deploy%20of%20payments-api.%0ACheck%20recent%20commits%20to%20main%20and%20the%20last%20successful%20build.
```

Wenn Sie darauf klicken, öffnet sich ein neues Terminalfenster, startet Claude Code in Ihrem lokalen Klon von `acme/payments` und füllt das Eingabefeld mit dem dekodierten Text:

```text theme={null}
Investigate the failed deploy of payments-api.
Check recent commits to main and the last successful build.
```

Sie können die Eingabeaufforderung bearbeiten, bevor Sie die Eingabetaste drücken, um sie zu senden. Wenn Sie keinen lokalen Klon des Repositorys haben, öffnet sich die Sitzung stattdessen in Ihrem Home-Verzeichnis. Siehe [Wählen Sie zwischen `cwd` und `repo`](#choose-between-cwd-and-repo), um zu erfahren, wie der lokale Pfad ausgewählt wird, wenn Sie mehrere Klone oder Worktrees haben.

<h3 id="choose-between-cwd-and-repo">
  Wählen Sie zwischen `cwd` und `repo`
</h3>

Verwenden Sie `cwd`, wenn jeder, der auf den Link klickt, das Projekt unter demselben absoluten Pfad hat, z. B. in einem standardisierten DevContainer oder VM-Image.

Verwenden Sie `repo`, wenn der Link freigegeben wird und jede Person zu einem anderen Ort klont. Claude Code löst den Slug wie folgt zu einem lokalen Pfad auf:

* Jedes Mal, wenn Sie `claude` in einem Git-Repository ausführen, wird der Dateisystempfad dieses Verzeichnisses gegen den `owner/name`-Slug des Repositorys auf GitHub aufgezeichnet.
* Wenn ein Deep Link ankommt, öffnet `repo` den zuletzt verwendeten passenden Pfad. Mehrere Klone und Worktrees werden separat nachverfolgt, daher wird derjenige ausgewählt, in dem Sie zuletzt gearbeitet haben.
* Die Suche findet nur Pfade, in denen Sie Claude Code mindestens einmal ausgeführt haben.
* Der Link ändert nicht, welcher Branch ausgecheckt ist. Die Sitzung öffnet sich in dem Zustand, in dem sich dieses Verzeichnis derzeit befindet.

Der Willkommens-Header zeigt, welchen Pfad er ausgewählt hat, damit Sie bestätigen können, dass der richtige Klon geöffnet wurde.

<h2 id="examples">
  Beispiele
</h2>

Die folgenden Abschnitte zeigen zwei häufige Möglichkeiten, einen Deep Link zu verwenden: als Markdown-Link in einem Dokument und als Befehl in einem Skript oder Shell-Alias.

<h3 id="embed-a-link-in-a-runbook">
  Betten Sie einen Link in ein Runbook ein
</h3>

Ein Deep Link in einem Runbook gibt demjenigen, der einen Incident triagiert, eine One-Click-Möglichkeit, im richtigen Repository mit einer vorbereiteten Eingabeaufforderung zu untersuchen. Die Plattform, die das Runbook rendert, muss benutzerdefinierte URL-Schemas zulassen. Von GitHub gerendertes Markdown erlaubt `claude-cli://` nicht, daher zeigt ein Deep Link in einer GitHub README, einem Issue oder einem Wiki nur sein Label ohne klickbaren Link. Siehe [die Fehlerbehebungsnotiz](#the-link-renders-as-plain-text-instead-of-being-clickable) für eine Problemumgehung.

Die Eingabeaufforderung ist Teil der URL und muss URL-kodiert sein. Um den kodierten Wert zu erzeugen, übergeben Sie Ihren Eingabeaufforderungstext durch `encodeURIComponent` in einer Browser-Konsole oder einem beliebigen URL-Encoder.

Das folgende Beispiel fügt einen Untersuchungs-Einstiegspunkt zu einem Incident-Runbook für einen Service namens `web-gateway` hinzu:

```markdown theme={null}
## High 5xx rate on web-gateway

1. Acknowledge the page in PagerDuty.
2. [Open Claude Code in the gateway repo](claude-cli://open?repo=acme/web-gateway&q=5xx%20rate%20is%20elevated%20on%20web-gateway.%20Check%20recent%20deploys%2C%20error%20logs%20from%20the%20last%2030%20minutes%2C%20and%20open%20incidents%20in%20Linear.)
3. Post initial findings in #incident.
```

Um dies in Ihrem eigenen Runbook zu verwenden, ersetzen Sie `acme/web-gateway` durch den Repository-Slug Ihres Service. Dies ermöglicht es Ingenieuren mit installiertem Claude Code und einem lokalen Klon dieses Repositorys, auf Schritt 2 zu klicken und mit der Eingabeaufforderung zum Senden zu beginnen.

<h3 id="open-a-link-from-the-shell">
  Öffnen Sie einen Link von der Shell aus
</h3>

Sie können einen Deep Link auch von einem Shell-Skript, Alias oder einer Automatisierung aus öffnen, anstatt ihn anzuklicken. Rufen Sie den URL-Öffnungsbefehl Ihres Betriebssystems mit dem Link als Argument auf.

<Tabs>
  <Tab title="macOS">
    Der integrierte `open`-Befehl übergibt die URL an den registrierten `claude-cli://`-Handler:

    ```bash theme={null}
    open "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>

  <Tab title="Linux">
    Die meisten Desktop-Umgebungen bieten `xdg-open`, das die URL an den registrierten Handler übergibt:

    ```bash theme={null}
    xdg-open "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>

  <Tab title="Windows">
    In PowerShell übergibt `Start-Process` die URL an den registrierten Handler:

    ```powershell theme={null}
    Start-Process "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```

    In `cmd.exe` behandelt `start` sein erstes zitiertes Argument als Fenstertitel, daher übergeben Sie einen leeren Titel vor der URL:

    ```cmd theme={null}
    start "" "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>
</Tabs>

<h2 id="registration-and-supported-platforms">
  Registrierung und unterstützte Plattformen
</h2>

Claude Code registriert den `claude-cli://`-Handler bei Ihrem Betriebssystem, wenn Sie zum ersten Mal eine interaktive Sitzung auf macOS, Linux und Windows starten. Sie führen keinen separaten Installationsbefehl aus. Die Registrierung schreibt nur in Benutzer-Level-Speicherorte:

| Plattform | Handler-Speicherort                                                                                               |
| --------- | ----------------------------------------------------------------------------------------------------------------- |
| macOS     | `~/Applications/Claude Code URL Handler.app`                                                                      |
| Linux     | `claude-code-url-handler.desktop` unter `$XDG_DATA_HOME/applications`, Standard ist `~/.local/share/applications` |
| Windows   | `HKEY_CURRENT_USER\Software\Classes\claude-cli`                                                                   |

Der Handler startet Claude Code in einem erkannten Terminal-Emulator. Auf macOS merkt sich Claude Code den Terminal aus Ihrer letzten interaktiven Sitzung und verwendet ihn erneut, unterstützt iTerm2, Ghostty, kitty, Alacritty, WezTerm und Terminal.app. Auf Linux respektiert es die `$TERMINAL`-Umgebungsvariable, dann `x-terminal-emulator`, dann eine Liste häufiger Emulatoren. Auf Windows bevorzugt es Windows Terminal, dann PowerShell, dann `cmd.exe`.

Um die Registrierung vollständig zu verhindern, setzen Sie [`disableDeepLinkRegistration`](/docs/de/settings) auf `"disable"` in `settings.json`. Um dies organisationsweit durchzusetzen, damit Benutzer es nicht erneut aktivieren können, setzen Sie es stattdessen in [verwalteten Einstellungen](/docs/de/server-managed-settings).

<h2 id="open-a-vs-code-tab-instead-of-a-terminal">
  Öffnen Sie einen VS Code-Tab statt eines Terminals
</h2>

Die VS Code-Erweiterung registriert ihren eigenen Handler unter `vscode://anthropic.claude-code/open`, der einen Claude Code-Editor-Tab statt eines Terminalfensters öffnet. Siehe [Starten Sie einen VS Code-Tab von anderen Tools](/docs/de/vs-code#launch-a-vs-code-tab-from-other-tools) für die Parameter dieser URL.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="clicking-the-link-does-nothing">
  Das Klicken auf den Link bewirkt nichts
</h3>

Der Handler ist wahrscheinlich noch nicht registriert. Starten Sie einmal eine interaktive `claude`-Sitzung auf diesem Computer, beenden Sie sie und versuchen Sie den Link erneut. Wenn Sie unter Linux ohne Desktop-Umgebung sind, hat `xdg-open` möglicherweise nichts zum Verteilen.

<h3 id="the-link-renders-as-plain-text-instead-of-being-clickable">
  Der Link wird als einfacher Text angezeigt, anstatt klickbar zu sein
</h3>

Einige Markdown-Renderer erlauben nur `http`- und `https`-Links und entfernen andere URL-Schemas. GitHub tut dies in READMEs, Issues, Pull Requests und Wikis: `[label](claude-cli://...)` wird nur als `label` gerendert, ohne Link und die URL wird entfernt. Auf diesen Plattformen platzieren Sie den Deep Link in einem Code-Block, damit Leser die URL sehen und in die Adressleiste ihres Browsers einfügen können.

<h3 id="the-session-opens-in-my-home-directory-instead-of-the-repo">
  Die Sitzung öffnet sich in meinem Home-Verzeichnis statt im Repo
</h3>

Der `repo`-Parameter löst nur zu Klonen auf, die Claude Code bereits gesehen hat. Führen Sie `claude` einmal im Klon aus, damit sein Pfad aufgezeichnet wird, oder wechseln Sie den Link zu `cwd` mit einem absoluten Pfad.

<h3 id="the-link-opens-the-wrong-terminal">
  Der Link öffnet das falsche Terminal
</h3>

Auf macOS starten Sie `claude` einmal in Ihrem bevorzugten Terminal und der nächste Deep Link wird es verwenden. Auf Linux setzen Sie die `$TERMINAL`-Umgebungsvariable auf den Befehlsnamen Ihres bevorzugten Emulators. Auf Windows ist die Reihenfolge festgelegt: Installieren Sie Windows Terminal, wenn Sie möchten, dass Links dort statt in einem PowerShell- oder `cmd.exe`-Fenster geöffnet werden.

<h2 id="learn-more">
  Weitere Informationen
</h2>

Diese Seiten behandeln verwandte Möglichkeiten, Claude Code-Sitzungen zu starten oder zu erweitern:

* [Skills](/docs/de/skills): Speichern Sie eine lange Runbook-Eingabeaufforderung als `/skill` im Repo, damit der `q`-Parameter des Deep Links nur seinen Namen angeben muss
* [Non-interactive mode](/docs/de/headless): Führen Sie Claude von einem Skript aus und erfassen Sie die Ausgabe, ohne ein Terminal zu öffnen
