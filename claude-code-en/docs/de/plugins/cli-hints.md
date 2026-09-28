> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Empfehlen Sie Ihr Plugin über Ihre CLI

> Fordern Sie Claude Code-Benutzer auf, Ihr offizielles Marketplace-Plugin zu installieren, indem Sie ein claude-code-hint-Tag von Ihrer CLI oder SDK ausgeben.

Wenn Sie eine CLI oder SDK verwalten, kann Ihr Tool Claude Code-Benutzer auffordern, Ihr Plugin zu installieren. Wenn Ihre CLI erkennt, dass sie in Claude Code ausgeführt wird, schreiben Sie ein einzeiliges `<claude-code-hint />`-Tag auf stderr. Claude Code entfernt die Zeile aus der Bash- und PowerShell-Tool-Ausgabe, bevor das Modell die Ausgabe sieht, und zeigt dem Benutzer dann eine einmalige Installationsaufforderung an.

Diese Seite gilt nur, wenn Ihr Plugin in `claude-plugins-official` oder einem anderen Marketplace mit einem der [offiziellen Marketplace-Namen](/docs/de/plugins/security#official-marketplace-names) von Anthropic aufgelistet ist. Der Community-Marketplace `claude-community` ist nicht einer davon.

<Note>
  Informationen zum Veröffentlichen eines Plugins finden Sie unter [Plugin veröffentlichen und verteilen](/docs/de/plugins/publish).
</Note>

<h2 id="emit-the-hint">
  Geben Sie den Hinweis aus
</h2>

Geben Sie das Tag nur aus, wenn `CLAUDECODE` oder `CLAUDE_CODE_CHILD_SESSION` gesetzt ist, damit es nicht angezeigt wird, wenn eine Person Ihre CLI direkt ausführt.

Claude Code setzt `CLAUDECODE=1` in den Befehlen, die es über die Bash- und PowerShell-Tools ausführt, und in Hook-Befehlen. Ab v2.1.172 setzt es dort auch `CLAUDE_CODE_CHILD_SESSION=1`. Die Variablen unterscheiden sich darin, welche Prozesse sie tragen:

* **`CLAUDECODE`**: wird von jeder Claude Code-Version gesetzt. IDE-Erweiterungen setzen es auch in ihren integrierten Terminals, sodass ein Gate nur auf `CLAUDECODE` das Tag auch ausgibt, wenn eine Person Ihre CLI selbst in einem dieser Terminals ausführt
* **`CLAUDE_CODE_CHILD_SESSION`**: wird nur in Subprozessen gesetzt, die Claude Code selbst startet. Verwenden Sie es, wenn Sie v2.1.172 oder später benötigen

Die [Referenz für Umgebungsvariablen](/docs/de/env-vars) enthält die Details.

Die folgenden Beispiele gaten auf `CLAUDECODE` für die größtmögliche Reichweite und geben einen Hinweis für ein Plugin namens `example-cli` im offiziellen Marketplace aus:

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
  if [ -n "$CLAUDECODE" ]; then
    printf '%s\n' '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />' >&2
  fi
  ```
</CodeGroup>

Ersetzen Sie `example-cli` durch den Namen Ihres Plugins im offiziellen Marketplace.

Sie können den Hinweis bei jeder Ausführung ausgeben, da Claude Code für jedes Plugin einmal auffordert.

Um den Emitter zu überprüfen, führen Sie `CLAUDECODE=1 example-cli` in einem Terminal aus und bestätigen Sie, dass die Tag-Zeile auf stderr angezeigt wird. Führen Sie dann `example-cli` ohne die Variable aus und bestätigen Sie, dass nichts Zusätzliches gedruckt wird.

<h2 id="hint-format">
  Hinweisformat
</h2>

Das Tag muss auf seiner eigenen Zeile stehen; Claude Code ignoriert ein Tag, das in der Mitte einer Zeile eingebettet ist.

Das Tag benötigt drei Attribute, alle erforderlich:

| Attribut | Beschreibung                                            |
| :------- | :------------------------------------------------------ |
| `v`      | Protokollversion. `1` ist der einzige unterstützte Wert |
| `type`   | Hinweistyp. `plugin` ist der einzige unterstützte Wert  |
| `value`  | Plugin-Identifier in der Form `name@marketplace`        |

Werte können in Anführungszeichen oder ohne Anführungszeichen stehen; ein Wert ohne Anführungszeichen kann keine Leerzeichen enthalten.

Claude Code entfernt die Zeile aus der Ausgabe, auch wenn `v` oder `type` nicht erkannt wird.

<h2 id="check-when-the-prompt-appears">
  Überprüfen Sie, wann die Aufforderung angezeigt wird
</h2>

Die Aufforderung wird nur in interaktiven Terminal-Sitzungen angezeigt. In `claude -p`-Ausführungen, in Subagent-Ausführungen und in Hook-Befehlsausgaben wird das Tag entfernt und es wird keine Aufforderung angezeigt. Alle diese Überprüfungen müssen auch bestanden werden:

* **Offiziell und installierbar**: `value` benennt ein Plugin, das Claude Code in seiner lokalen Kopie eines offiziellen Marketplace findet, das noch nicht installiert ist und das keine Richtlinie blockiert
* **Analytik aktiviert**: eine Sitzung, in der Claude Code-Analytik deaktiviert sind, wird nie aufgefordert, z. B. eine mit `DISABLE_TELEMETRY`, `DO_NOT_TRACK` oder `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` gesetzt, oder eine bei einem Drittanbieter wie Amazon Bedrock, wo die [automatische Telemetrie-Abmeldung](/docs/de/data-usage#default-behaviors-by-api-provider) gilt
* **Häufigkeitsgrenzen**: eine Aufforderung pro Sitzung, eine Aufforderung insgesamt pro Plugin unabhängig von der Antwort des Benutzers, und keine, sobald 100 Plugins auf diesem Computer aufgefordert wurden
* **Nicht deaktiviert**: der Benutzer hat nicht **Nein, und zeige mir keine Plugin-Installationshinweise mehr** gewählt
* **Lokale, betreute Sitzung**: der Arbeitsbereich der Sitzung ist lokal und nicht auf einem Cloud- oder Remote-Computer, und die Sitzung wird nicht unbeaufsichtigt ausgeführt. Beispielsweise wird eine Sitzung, die mit `--cloud` gestartet wurde, eine Sitzung, die Remote Control bereitstellt, oder ein Agent-Team-Teamkollege nie aufgefordert

<h2 id="preview-what-the-user-sees">
  Vorschau auf das, was der Benutzer sieht
</h2>

Wenn die Überprüfungen in [Überprüfen Sie, wann die Aufforderung angezeigt wird](#check-when-the-prompt-appears) bestanden werden, zeigt Claude Code einen **Plugin-Empfehlungs**-Dialog wie folgt an:

```text theme={null}
─────────────────────────────────────────────────────────────
  Plugin-Empfehlung

    Der Befehl example-cli schlägt vor, ein Plugin zu installieren.

    Plugin: example-cli
    Marketplace: claude-plugins-official
    Beschreibung: Offizielle Integration für example-cli-Bereitstellungen

    Möchten Sie es installieren?
    ❯ 1. Ja, installieren
      2. Nein
      3. Nein, und zeige mir keine Plugin-Installationshinweise mehr

─────────────────────────────────────────────────────────────
```

Der Dialog benennt das erste Wort des Shell-Befehls, den Claude ausgeführt hat, damit Benutzer eine Nichtübereinstimmung erkennen können. Jede Antwort hat eine Auswirkung:

* **Ja, installieren**: installiert das Plugin im [Benutzerbereich](/docs/de/plugins/install)
* **Nein, und zeige mir keine Plugin-Installationshinweise mehr**: deaktiviert zukünftige Hinweisaufforderungen für diesen Benutzer
* **Keine Antwort für 30 Sekunden**: zählt als **Nein**

<h2 id="next-steps">
  Nächste Schritte
</h2>

* [Plugin veröffentlichen und verteilen](/docs/de/plugins/publish): die Routen in jeden Marketplace, einschließlich des offiziellen Marketplace, den der Hinweis benötigt
* [Plugin-Befehle-Referenz](/docs/de/plugins/cli-reference#plugin-install): der Shell-Befehl, der das gleiche Plugin außerhalb einer Sitzung installiert
