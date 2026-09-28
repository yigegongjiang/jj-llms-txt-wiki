> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Code-Intelligence-Plugins

> Installieren Sie ein Language-Server-Plugin, damit Claude nach Änderungen Typfehler sieht und Code nach Symbol navigiert, und beantworten Sie den LSP-Plugin-Empfehlungsdialog.

Ein Code-Intelligence-Plugin gibt Claude die Live-Diagnose und Go-to-Definition, die Ihr Editor hat, damit Claude Typfehler und fehlende Importe erkennt, die seine eigenen Änderungen einführen, bevor Sie Ihren Build ausführen, und findet Definitionen und Referenzen nach Symbol statt nach Textsuche.

Jedes Plugin verbindet Claude Code mit einem Language Server für eine Sprache über das Language Server Protocol (LSP). Sie installieren das Plugin aus Anthropics offiziellem Marketplace und die Language-Server-Binärdatei auf Ihrem Computer.

<Note>
  Code-Intelligence-Plugins funktionieren in Terminal-Sitzungen. In [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) startet Claude Code keine Plugin-Language-Server, daher erhält Claude dort keine Diagnose oder Code-Navigation. Um Ihren eigenen Language-Server-Plugin zu schreiben oder einen Language Server ohne Plugin zu verbinden, siehe [LSP-Server in Plugin-Komponenten](/docs/de/plugins/components#lsp-servers).
</Note>

Um zu beginnen, finden Sie Ihre Sprache in der Tabelle unter [Code-Intelligence-Plugin installieren](#install-a-code-intelligence-plugin). Die Plugins in dieser Tabelle stammen aus Anthropics [offiziellem Plugin-Marketplace](/docs/de/plugins/anthropic-marketplaces).

Wenn Sie bereits einen **LSP-Plugin-Empfehlungsdialog** gesehen haben, siehe [Empfehlungsdialog akzeptieren oder verwerfen](#accept-or-dismiss-the-recommendation-dialog), um zu sehen, was jede Wahl bewirkt.

<h2 id="install-a-code-intelligence-plugin">
  Code-Intelligence-Plugin installieren
</h2>

Ein Code-Intelligence-Plugin teilt Claude Code mit, welcher Befehl den Language Server startet und welche Dateierweiterungen er verarbeitet. Es enthält nicht den Language Server. Installieren Sie zuerst die Language-Server-Binärdatei, dann das Plugin, dann bestätigen Sie, dass der Server startet.

<Steps>
  <Step title="Language-Server-Binärdatei installieren">
    Finden Sie Ihre Sprache in der Tabelle unten und installieren Sie die Binärdatei in ihrer Zeile. Wenn Ihre Sprache nicht aufgelistet ist, siehe [Sprache ohne offizielles Plugin hinzufügen](#add-a-language-without-an-official-plugin).

    | Sprache                   | Plugin                                                                                                           | Binärdatei                     |
    | :------------------------ | :--------------------------------------------------------------------------------------------------------------- | :----------------------------- |
    | C/C++                     | [`clangd-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/clangd-lsp)               | `clangd`                       |
    | C#                        | [`csharp-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/csharp-lsp)               | `csharp-ls`                    |
    | Go                        | [`gopls-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/gopls-lsp)                 | `gopls`                        |
    | Java                      | [`jdtls-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/jdtls-lsp)                 | `jdtls`                        |
    | Kotlin                    | [`kotlin-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/kotlin-lsp)               | `kotlin-lsp`                   |
    | Liquid                    | [`liquid-lsp`](https://github.com/Shopify/liquid-skills/tree/main/plugins/liquid-lsp)                            | `shopify`, aus der Shopify CLI |
    | Lua                       | [`lua-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/lua-lsp)                     | `lua-language-server`          |
    | PHP                       | [`php-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/php-lsp)                     | `intelephense`                 |
    | Python                    | [`pyright-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/pyright-lsp)             | `pyright-langserver`           |
    | Ruby                      | [`ruby-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/ruby-lsp)                   | `ruby-lsp`                     |
    | Rust                      | [`rust-analyzer-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/rust-analyzer-lsp) | `rust-analyzer`                |
    | Swift                     | [`swift-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/swift-lsp)                 | `sourcekit-lsp`                |
    | TypeScript und JavaScript | [`typescript-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/typescript-lsp)       | `typescript-language-server`   |

    Anthropic verwaltet jedes Plugin in der Tabelle außer `liquid-lsp`, das Shopify verwaltet und der offizielle Marketplace auflistet.

    Um den Befehl zu finden, der die Binärdatei installiert, folgen Sie dem Plugin-Link in der Tabelle zu seiner README. Für TypeScript ist dieser Befehl `npm install -g typescript-language-server typescript`.

    Nachdem Sie die Binärdatei installiert haben, bestätigen Sie, dass sie sich auf dem `PATH` der Shell befindet, von der aus Sie `claude` starten, zum Beispiel mit `which typescript-language-server` oder `Get-Command typescript-language-server` in PowerShell.
  </Step>

  <Step title="Plugin installieren">
    Um das Plugin zu installieren, das für Ihre Sprache in der Tabelle von Schritt 1 aufgelistet ist, führen Sie `/plugin install` in einer Claude-Code-Sitzung aus und ersetzen Sie `typescript-lsp` durch den Namen dieses Plugins:

    ```
    /plugin install typescript-lsp@claude-plugins-official
    ```

    Eine Bestätigungsmeldung gibt an, ob das Plugin jetzt aktiv ist oder `/reload-plugins` benötigt. Wenn die Installation mit `Marketplace "claude-plugins-official" not found` fehlschlägt, siehe den [Troubleshooting-Eintrag für diesen Fehler](/docs/de/plugins/troubleshooting#marketplace-claude-plugins-official-not-found). Um zu kontrollieren, wo das Plugin installiert wird, oder um die Installation von Ihrer Shell aus statt in Claude Code auszuführen, siehe [Plugins installieren](/docs/de/plugins/install).
  </Step>

  <Step title="Bestätigen Sie, dass der Server startet">
    Der Language Server startet das erste Mal, wenn Claude eine Datei mit einer der Plugin-Erweiterungen bearbeitet. Um es in Aktion zu sehen, bitten Sie Claude, einen Typfehler in einer Datei dieser Sprache einzuführen und ihn dann zu beheben. Überprüfen Sie dann das Gespräch auf eine Diagnosezeile:

    * **Eine Diagnosezeile erscheint**: `Found N new diagnostic issues in M files (ctrl+o to expand)` unter der Änderung, die den Fehler eingeführt hat, bedeutet, dass der Server gestartet ist.
    * **Keine Diagnosezeile erscheint**: Führen Sie `/plugin` aus und öffnen Sie die Registerkarte **Errors**. Eine Zeile, die `Executable not found in $PATH: "<binary>"` liest, nennt die zu installierende Binärdatei. Wenn die Registerkarte keine solche Zeile hat, siehe [Code-Intelligence troubleshooten](#troubleshoot-code-intelligence).

    Nachdem Sie eine fehlende Binärdatei installiert haben, versucht Claude Code es erneut, wenn Claude das nächste Mal eine entsprechende Datei bearbeitet. Wenn Sie die Binärdatei in ein Verzeichnis installiert haben, das sich nicht auf dem `PATH` der Shell befindet, von der aus Sie `claude` gestartet haben, starten Sie eine neue Sitzung von einer Shell aus, in der es sich befindet.
  </Step>
</Steps>

<h2 id="see-what-claude-gains">
  Sehen Sie, was Claude gewinnt
</h2>

Mit einem laufenden Language Server erhält Claude Diagnose und Code-Navigation:

* **Diagnose nach Änderungen**: Jedes Mal, wenn Claude eine Datei bearbeitet oder schreibt, die der Server verarbeitet, erhält Claude die Fehler und Warnungen, die der Server meldet. Es sieht einen Typfehler, fehlenden Import oder Syntaxfehler, den es eingeführt hat, ohne einen Compiler auszuführen.
* **Code-Navigation**: Claude erhält ein `LSP`-Tool, das Symbole über den Server nachschlägt, anstatt sie durch Textsuche zu finden. Das Tool ist schreibgeschützt. Für das, was Claude mit dem Tool nachschlagen kann und wie Berechtigungen darauf angewendet werden, siehe [LSP-Tool-Verhalten](/docs/de/tools-reference#lsp-tool-behavior).

<h3 id="read-the-diagnostics-yourself">
  Lesen Sie die Diagnose selbst
</h3>

Nachdem Claude eine Datei bearbeitet hat, die der Server verarbeitet, zeigt das Gespräch nur die `Found N new diagnostic issues`-Zusammenfassung. Um die Probleme selbst zu lesen, drücken Sie **Ctrl+O**.

<h2 id="accept-or-dismiss-the-recommendation-dialog">
  Empfehlungsdialog akzeptieren oder ablehnen
</h2>

Wenn eine Language-Server-Binärdatei bereits auf Ihrem `PATH` ist und das Plugin, das sie verwendet, nicht installiert ist, bietet Claude Code an, das Plugin für Sie in einem Dialog mit dem Titel **LSP plugin recommendation** zu installieren.

<h3 id="when-the-recommendation-dialog-appears">
  Wenn der Empfehlungsdialog erscheint
</h3>

Der Dialog **LSP plugin recommendation** kann nach einer Änderung durch Claude erscheinen. Diese Bedingungen entscheiden, ob er erscheint und welches Plugin er anbietet:

* **Ein Plugin passt zur Datei**: Einer der Marketplaces, die Sie hinzugefügt haben, oder der offizielle Marketplace, den Claude Code für Sie registriert hat, listet ein Code-Intelligence-Plugin für die Erweiterung dieser Datei auf, und die Binärdatei des Plugins ist installiert.
* **Offiziell zuerst**: Wenn mehr als ein Marketplace ein Plugin für die Erweiterung anbietet, bietet der Dialog das Plugin des offiziellen Marketplace an.
* **Einmal pro Sitzung**: Der Dialog erscheint höchstens einmal in einer Sitzung, für die erste entsprechende Datei, die Claude bearbeitet.
* **Nicht für Cloud-Sitzungen**: Der Dialog erscheint nie, wenn Ihr Terminal an eine Cloud-Sitzung angehängt ist, z. B. eine, die Sie mit [`claude --cloud`](/docs/de/claude-code-on-the-web#from-terminal-to-cloud) gestartet haben.

<h3 id="respond-to-the-recommendation-dialog">
  Antworten Sie auf den Empfehlungsdialog
</h3>

Der Dialog **LSP plugin recommendation** nennt das Plugin und bietet diese Optionen:

* **Yes, install**: Claude Code installiert das Plugin für Ihr Benutzerkonto und gibt `<plugin> installed · restart to apply` aus. Starten Sie eine neue Sitzung, um den Server zu laden.
* **No, not now**: Der Dialog schließt sich, und eine spätere Sitzung kann das Plugin erneut anbieten. Das Drücken von **Esc** hat die gleiche Wirkung.
* **Never for this plugin**: Der Dialog hört auf, für dieses Plugin zu erscheinen, und erscheint weiterhin für andere.
* **Disable all LSP recommendations**: Der Dialog hört auf, für jede Sprache zu erscheinen.

Wenn Sie keine Option wählen, schließt Claude Code sie nach 30 Sekunden und zählt das als ignoriert. Die Anzahl wird über Sitzungen hinweg beibehalten. Nach fünf ignorierten Dialogen hört Claude Code auf, Plugins zu empfehlen, genauso wie wenn Sie **Disable all LSP recommendations** gewählt hätten.

<h3 id="turn-recommendations-back-on">
  Empfehlungen wieder aktivieren
</h3>

Der Dialog **LSP plugin recommendation** hört auf zu erscheinen, nachdem Sie **Disable all LSP recommendations** wählen oder ihn fünfmal ignorieren.

* **Deaktiviert oder fünfmal ignoriert**: Um ihn in beiden Fällen wieder zu aktivieren, entfernen Sie die Schlüssel `lspRecommendationDisabled` und `lspRecommendationIgnoredCount` aus `~/.claude.json`, Claude Codes eigener Konfigurationsdatei.
* **Never for this plugin**: Wenn Sie **Never for this plugin** gewählt haben und möchten, dass dieses Plugin erneut angeboten wird, entfernen Sie seine `name@marketplace`-ID aus der Liste `lspRecommendationNeverPlugins` in derselben Datei.

<h2 id="troubleshoot-code-intelligence">
  Code-Intelligence troubleshooten
</h2>

Die Seite zur Fehlerbehebung von Plugins behandelt die Symptome, die spezifisch für Code-Intelligence-Plugins sind, unter [Language Server startet nicht, verbraucht zu viel Speicher oder meldet falsche Diagnose](/docs/de/plugins/troubleshooting#language-server-doesnt-start):

* **Der Language Server startet nicht**: Sie sehen `Executable not found in $PATH` in der Registerkarte **Errors** von `/plugin`, oder Claude meldet nie Diagnose für die Sprache.
* **Hohe Speichernutzung**: Die Speichernutzung nimmt zu, während der Server das Projekt indiziert.
* **Falsch positive Diagnose in einem Monorepo**: Diagnose meldet Importe als ungelöst, wenn sie es nicht sind.

<h2 id="add-a-language-without-an-official-plugin">
  Sprache ohne offizielles Plugin hinzufügen
</h2>

Wenn Ihre Sprache nicht in der [Tabelle der offiziellen Plugins](#install-a-code-intelligence-plugin) aufgelistet ist, können Sie dennoch einen Language Server verbinden.

1. Schreiben Sie ein Plugin mit einer `.lsp.json`-Datei, die den Server-Befehl und die Dateierweiterungen nennt, die er verarbeitet.
2. Laden Sie dann das Plugin mit [`--plugin-dir`](/docs/de/plugins/cli-reference#flags-that-load-a-plugin-for-one-session) oder veröffentlichen Sie es in einem Marketplace.

Für die Felder der Datei und ein durchgearbeitetes Beispiel, siehe [LSP-Server in Plugin-Komponenten](/docs/de/plugins/components#lsp-servers).

<h2 id="next-steps">
  Nächste Schritte
</h2>

* [LSP-Server in Plugin-Komponenten](/docs/de/plugins/components#lsp-servers): Schreiben Sie die `.lsp.json` für einen Language Server, der kein offizielles Plugin hat
* [Plugins installieren und verwalten](/docs/de/plugins/install): Bereiche, Updates und Deinstallation
* [Plugins troubleshooten](/docs/de/plugins/troubleshooting): Ladefehler über die Language-Server-Fehler auf dieser Seite hinaus
* [Plugins im offiziellen Marketplace finden](/docs/de/plugins/anthropic-marketplaces#find-plugins-in-the-official-marketplace): Wo Sie den Rest des offiziellen Marketplace durchsuchen können
