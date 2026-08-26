> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code Desktop in WSL

> Führen Sie Code-Sitzungen in einer WSL 2-Distribution unter Windows aus

Unter Windows kann die Registerkarte Code eine Sitzung in einer WSL 2-Distribution ausführen, anstatt auf Windows selbst. Der Claude Code-Prozess der Sitzung, ihre Tools und Git werden alle in der Distribution ausgeführt und verwenden ihre Linux-Toolchain und native Linux-Pfade – die gleiche Umgebung, auf die Ihr Projekt abzielt.

Verwenden Sie eine WSL-Sitzung, wenn sich Ihr Repository im Dateisystem der Distribution befindet. Das Arbeiten mit diesen Dateien von Windows aus erfolgt über ein Netzwerk-Dateisystem, das langsam ist und die Dateiüberwachung unterbricht. Das Ausführen der Sitzung in der Distribution vermeidet beides.

<h2 id="requirements">
  Anforderungen
</h2>

* Windows 10 oder 11 mit [WSL 2](https://learn.microsoft.com/windows/wsl/install). WSL 1 wird nicht unterstützt.
* Mindestens eine installierte Distribution (z. B. Ubuntu).
* `git` in der Distribution installiert.

<h2 id="start-a-wsl-session">
  Starten Sie eine WSL-Sitzung
</h2>

<Steps>
  <Step title="Wählen Sie eine Distribution">
    Starten Sie eine neue Sitzung in der Registerkarte Code und öffnen Sie die Umgebungsauswahl. Ihre installierten WSL 2-Distributionen werden in einem Abschnitt **WSL** angezeigt. Wählen Sie eine aus.
  </Step>

  <Step title="Wählen Sie einen Ordner">
    Die Sitzung startet im Home-Verzeichnis der Distribution. Verwenden Sie die Ordnerauswahl, um einen Projektordner auszuwählen. Das Durchsuchen erfolgt in der Distribution mit Linux-Pfaden wie `/home/you/project`.
  </Step>

  <Step title="Vertrauen Sie dem Ordner">
    Die erste Sitzung in einem Ordner zeigt den Dialog zur Workspace-Vertrauenswürdigkeit. Vertrauen wird pro Distribution und Ordner gewährt; das Vertrauen in einen Ordner in einer Distribution gilt nicht für eine andere Distribution oder denselben Pfad unter Windows.
  </Step>
</Steps>

Die erste Sitzung in einer Distribution dauert etwas länger, während Claude sich darin einrichtet. Sie können auch einen Ordner `\\wsl.localhost\...` aus der normalen Ordnerauswahl öffnen, und er wird in dieser Distribution erneut geöffnet.

Ordner, die Sie kürzlich verwendet haben, werden in der Auswahl pro Distribution angezeigt, sodass das erneute Verbinden mit einem Projekt nur einen Klick erfordert.

<h2 id="what-works-in-a-wsl-session">
  Was in einer WSL-Sitzung funktioniert
</h2>

Parallele Sitzungen, Seitenchats, visuelle Diff-Überprüfung, Branch- und Pull-Request-Status sowie Worktrees funktionieren alle, unterstützt durch Git und die Toolchain in der Distribution. „In Editor öffnen" öffnet VS Code, das über [Remote - WSL](https://code.visualstudio.com/docs/remote/wsl) mit der Distribution verbunden ist.

Einige Funktionen sind in WSL-Sitzungen noch nicht verfügbar: das integrierte Terminal, Connectors und Plugins, Sitzungs-Forking, der Datei-Browser-Bereich und Dateivorschläge, wenn Sie `@` im Composer eingeben.

<h2 id="managed-devices">
  Verwaltete Geräte
</h2>

Auf Geräten, die von einer Organisation verwaltet werden, sind WSL-Sitzungen möglicherweise nicht verfügbar. Wenn der Sitzungsstart mit einer Meldung fehlschlägt, dass das Gerät verwaltet wird, wird dies von Ihrem Administrator gesteuert. Administratoren: siehe [wie Einstellungen auf Geräte gelangen](/docs/de/admin-setup#decide-how-settings-reach-devices) im Bereitstellungsleitfaden.
