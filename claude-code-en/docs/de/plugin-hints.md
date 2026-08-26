> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Empfehlen Sie Ihr Plugin von Ihrer CLI aus

> Geben Sie einen einzeiligen Marker von Ihrer CLI aus, damit Claude Code Benutzer auffordert, Ihr offizielles Plugin zu installieren.

Wenn Sie eine CLI oder ein SDK verwalten und ein Plugin im offiziellen Anthropic-Marketplace haben, kann Ihr Tool Claude Code-Benutzer auffordern, dieses Plugin zu installieren. Ihre CLI schreibt einen einzeiligen Marker auf stderr, wenn sie erkennt, dass sie in Claude Code ausgeführt wird. Claude Code liest den Marker, entfernt ihn aus der Ausgabe und zeigt dem Benutzer eine einmalige Installationsaufforderung an.

Claude Code entfernt die Hinweiszeile aus der Befehlsausgabe, bevor sie an das Modell gesendet wird, sodass der Marker nie in der Konversation erscheint und nicht zur Token-Nutzung zählt. Das Protokoll erfordert keine zusätzlichen Befehle und ändert nicht, was Ihre CLI für Benutzer außerhalb von Claude Code ausgibt.

Diese Seite ist für CLI- und SDK-Verwalter. Wenn Sie nach der Installation von Plugins suchen, siehe [Plugins entdecken und installieren](/docs/de/discover-plugins).

<h2 id="how-it-works">
  Funktionsweise
</h2>

Claude Code setzt die [`CLAUDECODE`](/docs/de/env-vars) Umgebungsvariable auf `1` für jeden Befehl, den es über die Bash- und PowerShell-Tools ausführt, und für [hook](/docs/de/hooks)-Befehle. Ab v2.1.172 setzt es auch [`CLAUDE_CODE_CHILD_SESSION`](/docs/de/env-vars) auf `1` in denselben Subprozessen. Wenn Ihre CLI eine dieser Variablen sieht, schreibt sie ein selbstschließendes `<claude-code-hint />`-Tag auf stderr. Bei hook-Befehlen wird das Hinweis-Tag entfernt und ignoriert. Nur die Ausgabe von Bash- und PowerShell-Tools löst die Installationsaufforderung aus.

Wenn Claude Code die Befehlsausgabe empfängt, führt es folgende Schritte aus:

1. Scannt nach Hinweiszeilen und entfernt sie, bevor die Ausgabe das Modell erreicht
2. Überprüft, dass der Hinweis auf ein Plugin in einem offiziellen Anthropic-Marketplace abzielt
3. Überprüft, dass das Plugin nicht bereits installiert ist und nicht zuvor aufgefordert wurde
4. Zeigt dem Benutzer eine Installationsaufforderung an, die den Befehl benennt, der den Hinweis ausgegeben hat

Claude Code installiert ein Plugin nie automatisch. Der Benutzer bestätigt immer.

<h2 id="emit-the-hint">
  Geben Sie den Hinweis aus
</h2>

Hinweis-Prompts werden nur für Plugins ausgelöst, die im offiziellen Anthropic-Marketplace aufgeführt sind. Siehe [Bringen Sie Ihr Plugin in den offiziellen Marketplace](#get-your-plugin-into-the-official-marketplace), bevor Sie die Integration bereitstellen.

Geben Sie die Ausgabe an eine Umgebungsvariable ab, damit der Marker wahrscheinlich nicht erscheint, wenn ein Mensch Ihre CLI direkt ausführt, und schreiben Sie dann das Tag auf stderr auf seiner eigenen Zeile. Wählen Sie, welche Variable überprüft werden soll:

* `CLAUDECODE`: wird auf jeder Claude Code-Version gesetzt, daher erreicht es die meisten Sitzungen. Sie wird auch in tmux-Sitzungen und stdio MCP Server-Subprozessen gesetzt, die Claude Code startet. IDE-Erweiterungen setzen sie auch in ihren integrierten Terminals, wo ein Mensch Ihre CLI direkt ausführen kann.
* `CLAUDE_CODE_CHILD_SESSION`: wird nur in Subprozessen gesetzt, die Claude Code selbst startet, wie z. B. Tool-Aufrufe, Hook-Befehle und [Statuszeilen](/docs/de/statusline)-Befehle, daher erreicht das Tag normalerweise kein menschliches Terminal. Ein langlebiger Prozess, der innerhalb einer Sitzung gestartet wurde, wie z. B. ein tmux-Server, erfasst die Variable, daher zeigen Shells, die später von diesem Prozess aus gestartet werden, immer noch das rohe Tag. Erfordert Claude Code v2.1.172 oder später, daher verpassen Sitzungen auf älteren Versionen den Hinweis.

Die folgenden Beispiele geben auf `CLAUDECODE` ab, um maximale Reichweite zu erreichen, und geben einen Hinweis für ein Plugin namens `example-cli` im offiziellen Marketplace aus:

<CodeGroup>
  ```javascript Node.js theme={null}
  if (process.env.CLAUDECODE) {
    process.stderr.write(
      '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />\n',
    )
  }
  ```

  ```python Python theme={null}
  import os, sys

  if os.environ.get("CLAUDECODE"):
      print(
          '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />',
          file=sys.stderr,
      )
  ```

  ```go Go theme={null}
  if os.Getenv("CLAUDECODE") != "" {
      fmt.Fprintln(os.Stderr,
          `<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />`)
  }
  ```

  ```shell Shell theme={null}
  [ -n "$CLAUDECODE" ] &&
    printf '%s\n' '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />' >&2
  ```
</CodeGroup>

Ersetzen Sie `example-cli` durch den Namen Ihres Plugins im offiziellen Marketplace.

<h2 id="choose-where-to-emit">
  Wählen Sie, wo Sie ausgeben
</h2>

Sie kontrollieren, welche Code-Pfade den Hinweis ausgeben. Claude Code dedupliziert nach Plugin, daher hat die Ausgabe bei jeder Aufrufen keinen Nachteil. Berührungspunkte, die gut funktionieren, sind:

| Platzierung                                | Warum es funktioniert                                                   |
| :----------------------------------------- | :---------------------------------------------------------------------- |
| `--help` Ausgabe                           | Claude führt häufig Hilfe aus, wenn eine unbekannte CLI erkundet wird   |
| Fehler bei unbekanntem Unterbefehl         | Erreicht den Moment, in dem Claude über Ihre Schnittstelle verwirrt ist |
| Anmeldungs- oder Authentifizierungserfolg  | Der Benutzer ist bereits in einer Einrichtungsmentalität                |
| Willkommensnachricht beim ersten Ausführen | Ein natürlicher Onboarding-Moment                                       |

<h2 id="what-the-user-sees">
  Was der Benutzer sieht
</h2>

Wenn der Hinweis alle Überprüfungen besteht, zeigt Claude Code eine Aufforderung wie die folgende an:

```text theme={null}
─────────────────────────────────────────────────────────────
  Plugin-Empfehlung

    Der Befehl example-cli schlägt vor, ein Plugin zu installieren.

    Plugin: example-cli
    Marketplace: claude-plugins-official
    Offizielle Integration für example-cli-Bereitstellungen

    Möchten Sie es installieren?
    ❯ 1. Ja, example-cli installieren
      2. Nein
      3. Nein, und zeige mir keine Plugin-Installationshinweise mehr

─────────────────────────────────────────────────────────────
```

Die Aufforderung benennt den Befehl, der den Hinweis erzeugt hat, damit Benutzer einen Mismatch zwischen dem Tool und dem Plugin, das es empfiehlt, erkennen können. Wenn der Benutzer nicht innerhalb von 30 Sekunden antwortet, wird die Aufforderung als **Nein** verworfen.

Die Häufigkeit der Aufforderung ist begrenzt:

* **Einmal pro Plugin**: Nachdem die Aufforderung angezeigt wurde, zeichnet Claude Code das Plugin auf und fordert es nie wieder auf, unabhängig von der Antwort des Benutzers.
* **Einmal pro Sitzung**: Auf allen CLIs auf dem Computer erscheint höchstens eine Hinweisaufforderung pro Claude Code-Sitzung.

Wenn Sie **Ja** auswählen, wird das Plugin im Benutzerbereich installiert. Wenn Sie **Nein, und zeige mir keine Plugin-Installationshinweise mehr** auswählen, werden alle zukünftigen Hinweisaufforderungen für den Benutzer deaktiviert.

<h2 id="hint-format">
  Hinweisformat
</h2>

Der Hinweis ist ein selbstschließendes Tag mit drei erforderlichen Attributen.

```text theme={null}
<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />
```

| Attribut | Erforderlich | Beschreibung                                            |
| :------- | :----------- | :------------------------------------------------------ |
| `v`      | Ja           | Protokollversion. `1` ist der einzige unterstützte Wert |
| `type`   | Ja           | Hinweistyp. `plugin` ist der einzige unterstützte Wert  |
| `value`  | Ja           | Plugin-Identifier in der Form `name@marketplace`        |

Attributwerte können mit doppelten Anführungszeichen zitiert oder unzitiert gelassen werden. Unzitierte Werte können keine Leerzeichen enthalten. Escape-Sequenzen werden nicht unterstützt.

<h2 id="requirements">
  Anforderungen
</h2>

Claude Code erzwingt zwei Bedingungen, bevor es auf einen Hinweis reagiert. Hinweise, die eine der beiden Überprüfungen nicht bestehen, werden verworfen:

* **Eigene Zeile**: Das Tag muss auf seiner eigenen Zeile stehen. Ein Tag, das in der Mitte einer Zeile eingebettet ist, z. B. in einer Log-Anweisung, wird ignoriert. Führende und nachfolgende Leerzeichen auf der Zeile sind zulässig.
* **Offizieller Marketplace**: Der `value` muss auf ein Plugin in einem von Anthropic kontrollierten Marketplace wie `claude-plugins-official` verweisen. Hinweise, die auf andere Marketplaces verweisen, werden stillschweigend verworfen.

Die Hinweiszeile wird immer aus der Ausgabe entfernt, bevor sie das Modell erreicht, auch wenn die Version oder der Typ nicht erkannt wird, sodass der Marker nie zur Token-Nutzung zählt.

Die verbleibende Anleitung wird empfohlen, aber nicht erzwungen. Claude Code kann nicht beobachten, ob Ihre CLI sie befolgt:

* **Schreiben Sie auf stderr**: stderr hält das Tag aus Shell-Pipelines wie `example-cli deploy | jq` heraus. Claude Code scannt beide Streams, daher funktioniert auch stdout.
* **Gate on an environment variable**: Geben Sie nur aus, wenn `CLAUDECODE` oder `CLAUDE_CODE_CHILD_SESSION` gesetzt ist. Siehe [Emit the hint](#emit-the-hint), um zu erfahren, wie sich die beiden Variablen unterscheiden.

<h2 id="get-your-plugin-into-the-official-marketplace">
  Bringen Sie Ihr Plugin in den offiziellen Marketplace
</h2>

Das Hinweisprotokoll wird nur für Plugins wirksam, die im offiziellen Anthropic-Marketplace aufgelistet sind, `claude-plugins-official`. Anthropic kuratiert diesen Marketplace nach eigenem Ermessen, und die In-App-Einreichungsformulare fügen Plugins stattdessen zum [Community-Marketplace](/docs/de/plugins#submit-your-plugin-to-the-community-marketplace) hinzu, den das Hinweisprotokoll nicht überprüft. Wenn Sie mit einem Anthropic-Partner-Kontakt zusammenarbeiten, wenden Sie sich an ihn, um die Auflistung im offiziellen Marketplace zu koordinieren.

<h2 id="see-also">
  Siehe auch
</h2>

* [Erstellen Sie Plugins](/docs/de/plugins): Erstellen Sie das Plugin, das Ihre CLI empfiehlt
* [Erstellen und verteilen Sie einen Plugin-Marketplace](/docs/de/plugin-marketplaces): Hosten Sie Plugins außerhalb des offiziellen Marketplace
* [Umgebungsvariablen](/docs/de/env-vars): Vollständige Referenz für `CLAUDECODE` und verwandte Variablen
