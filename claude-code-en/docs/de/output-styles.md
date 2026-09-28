> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ausgabestile

> Ändern Sie Claudes Rolle, Ton und Antwortformat mit einem integrierten Ausgabestil wie „Prägnant" oder „Erklärend", oder schreiben Sie einen benutzerdefinierten Stil.

Ein Ausgabestil ist ein Satz von Anweisungen, der Claudes Rolle, Ton und Antwortformat für jede Antwort in einer Sitzung festlegt. Claude Code enthält vier integrierte Stile neben seinem Standard, und Sie können Ihre eigenen schreiben.

Verwenden Sie einen Ausgabestil, um die Art und Weise zu ändern, wie Claude antwortet und mit Ihnen für eine ganze Sitzung zusammenarbeitet, damit Sie die Anfrage nicht in jedem Durchgang wiederholen müssen. Beispielsweise kann ein integrierter Stil Antworten kürzer machen, eine Erklärung jeder Änderung hinzufügen oder Claude veranlassen, die Arbeit zu beginnen, ohne Routinefragen zu stellen. Ein benutzerdefinierter Stil kann Claude auch in etwas anderes als einen Softwareentwickler umwandeln, z. B. in einen Schreib-Assistenten oder einen Datenanalysten.

* Um einen integrierten Stil zu verwenden, wählen Sie einen aus den [integrierten Ausgabestilen](#built-in-output-styles) und [wechseln Sie zu ihm](#change-your-output-style).
* Um Ihre eigenen Anweisungen zu schreiben, [erstellen Sie einen benutzerdefinierten Ausgabestil](#create-a-custom-output-style).

<Note>
  Ein Ausgabestil gibt Claude Anweisungen zum Befolgen. Es garantiert nicht, dass etwas immer passiert oder nie passiert. Einige Anforderungen passen zu einer anderen Funktion:

  * Für das, was Claude über Ihr Projekt wissen sollte, verwenden Sie [CLAUDE.md](/docs/de/memory).
  * Für etwas, das jedes Mal passieren muss, z. B. Formatierung nach jeder Bearbeitung oder Blockierung eines Befehls, verwenden Sie einen [Hook](/docs/de/hooks-guide).
  * Für Skills, Subagenten und die anderen Optionen siehe [Wählen Sie zwischen einem Ausgabestil und anderen Funktionen](#choose-between-an-output-style-and-other-features).
</Note>

<h2 id="built-in-output-styles">
  Integrierte Ausgabestile
</h2>

Claude Code startet im [**Standard**](#default)-Stil, seinen Standard-Anweisungen für die Durchführung von Softwareentwicklungsaufgaben. Jeder der vier anderen integrierten Stile behält diese Anweisungen bei und fügt seine eigenen hinzu.

Diese Tabelle zeigt, was jeder Stil in einer Sitzung ändert und wann er passt:

| Stil                      | Was ändert sich                                                                                                   | Verwenden Sie ihn, wenn                                                                                                             |
| :------------------------ | :---------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------- |
| [Proaktiv](#proactive)    | Claude beginnt sofort mit der Arbeit und trifft vernünftige Annahmen, anstatt bei Routineentscheidungen zu fragen | Sie möchten, dass Claude durch Routineentscheidungen weitermacht, und Sie werden den Kurs korrigieren, wenn eine Annahme falsch ist |
| [Prägnant](#concise)      | Antworten beginnen mit dem Ergebnis und lassen Präambel, Erzählung und Zusammenfassungen weg                      | Standard-Antworten sind länger als Sie möchten                                                                                      |
| [Erklärend](#explanatory) | Claude fügt kurze `Insight`-Blöcke hinzu, die die Entscheidungen hinter dem geschriebenen Code erklären           | Sie lernen eine Codebasis kennen oder möchten die Begründung zusammen mit der Änderung                                              |
| [Lernend](#learning)      | Claude erklärt seine Entscheidungen und lässt kleine Codestücke für Sie selbst schreiben                          | Sie möchten praktische Codierungserfahrung, während die Aufgabe noch erledigt wird                                                  |

<h3 id="default">
  Standard
</h3>

Standard bedeutet, dass kein Ausgabestil ausgewählt ist. Claude Code fügt keine Stil-Anweisungen hinzu, und Claude arbeitet aus Claude Codes Standard-Systemaufforderung, die für Softwareentwicklungsaufgaben geschrieben ist.

`default` erscheint in der `/output-style`-Liste zusammen mit den anderen Stilen, sodass Sie [ihn auf die gleiche Weise auswählen](#change-your-output-style).

<h3 id="proactive">
  Proaktiv
</h3>

Im Proaktiv-Stil beginnt Claude sofort mit der Implementierung, sobald Sie eine Aufgabe senden. Es trifft vernünftige Annahmen über Routineentscheidungen, anstatt zu fragen, und wechselt nicht in den Plan-Modus, es sei denn, Sie fragen nach einem Plan. Sie können es jederzeit umleiten.

Die Anweisungen des Stils sagen Claude auch, dass es mit Ihnen im Gespräch überprüfen soll, bevor eine Aktion durchgeführt wird, die Daten löscht oder ein gemeinsames oder Produktionssystem ändert. Diese Überprüfung ist eine Anweisung, die Claude befolgt, und ist getrennt von Berechtigungsaufforderungen.

Das Wechseln zum Proaktiv-Stil ändert Ihren [Berechtigungsmodus](/docs/de/permission-modes) nicht. Ihr Berechtigungsmodus entscheidet weiterhin, welche Tool-Aufrufe ohne Nachfrage ausgeführt werden, sodass Berechtigungsaufforderungen auf die gleiche Weise wie zuvor angezeigt werden.

<h3 id="concise">
  Prägnant
</h3>

Im Prägnant-Stil gibt der erste Satz einer Antwort an, was passiert ist oder was die Antwort ist. Claude lässt die Einleitung, die Schritt-für-Schritt-Erzählung und die abschließende Zusammenfassung weg und beantwortet eine einfache Frage in ein bis drei Sätzen. Es führt die Softwareentwicklungsarbeit genauso gründlich durch wie im Standard-Stil. Erfordert Claude Code v2.1.237 oder später.

Claude schreibt in diesen Fällen immer noch in voller Länge:

* **Alles, das Sie fragen**: Wenn Sie um eine Erklärung oder mehr Details bitten, antwortet Claude vollständig.
* **Alles, das Sie benötigen, um sicher zu handeln**: Fehlerberichte, fehlgeschlagene Testausgabe, Sicherheitswarnungen und Bestätigungen für destruktive Aktionen behalten ihren vollständigen Inhalt.

<h3 id="explanatory">
  Erklärend
</h3>

Im Erklärend-Stil führt Claude die Aufgabe auf die gleiche Weise durch wie im Standard-Stil und fügt kurze Erklärungen hinzu, warum es die Entscheidungen traf, die es traf. Jede Erklärung erscheint im Gespräch, vor oder nach dem Code, auf den sie sich bezieht, in einem Block mit der Bezeichnung `Insight`. Die Erklärungen werden nicht als Kommentare in Ihre Dateien geschrieben.

Ein `Insight`-Block enthält zwei oder drei Punkte über Ihre Codebasis oder den Code, den Claude geschrieben hat, wie dieser nach dem Hinzufügen eines API-Endpunkts:

```text theme={null}
★ Insight ─────────────────────────────────────
- Every route in this repo goes through the withAuth wrapper, so the new endpoint gets session checks without its own middleware.
- Rate limits are set per route in limits.ts, which is why this change adds an entry there rather than a global default.
─────────────────────────────────────────────────
```

<h3 id="learning">
  Lernend
</h3>

Im Lernend-Stil fügt Claude die gleichen `Insight`-Blöcke wie der [Erklärend-Stil](#explanatory) hinzu und fordert Sie auch auf, einen Teil des Codes zu schreiben. Claude kümmert sich selbst um die Routine-Implementierung. Wenn es ein Stück mit einer echten Designentscheidung erreicht, wie Fehlerbehandlung, eine Datenstruktur oder Geschäftslogik mit mehr als einem gültigen Ansatz, lässt es ein paar Zeilen für Sie.

Claude markiert die Stelle mit einem `TODO(human)`-Kommentar in der Datei und sendet dann eine Anfrage, die angibt, was bereits erstellt ist, was zu schreiben ist und was zu berücksichtigen ist:

```text theme={null}
● Learn by Doing

Context: The upload form is in place and calls validateFile() before accepting a file. Size and type checks work for images, but the switch statement has no handling for documents yet.

Your Task: In upload.js, implement the case "document" branch inside validateFile(). Look for TODO(human).

Guidance: Decide on a size limit for documents and whether the file extension has to match the MIME type. Return {valid: boolean, error?: string}.
```

Claude hält dann an und wartet. Schreiben Sie Ihren Code beim `TODO(human)`-Kommentar und teilen Sie Claude mit, wenn Sie fertig sind. Claude antwortet mit einem `Insight` über Ihren Code und setzt die Aufgabe fort.

<h2 id="change-your-output-style">
  Ändern Sie Ihren Ausgabestil
</h2>

Wählen Sie einen Stil mit dem Befehl, einem Menü oder einer Einstellungsdatei. Der Befehl und beide Menüs speichern Ihre Auswahl in `.claude/settings.local.json` auf der [lokalen Projektebene](/docs/de/settings).

* **`/output-style` Befehl**: Führen Sie `/output-style <style>` aus, um zu wechseln, zum Beispiel `/output-style concise`. Ohne Argument listet der Befehl die Stile auf, die Sie auswählen können, und markiert den aktuellen.

  Der Befehl funktioniert auch im [nicht-interaktiven Modus](/docs/de/headless) und Agent SDK-Sitzungen sowie über die mobile App oder das Web über [Remote Control](/docs/de/remote-control#limitations), wo Sie nur [integrierte Stile](#built-in-output-styles) auflisten und auswählen können. Erfordert Claude Code v2.1.269 oder später.
* **Terminal-Menü**: Führen Sie `/config` aus und wählen Sie **Output style**, um einen Stil aus einem Menü auszuwählen.
* **VS Code-Erweiterung**: Öffnen Sie das [Befehlsmenü](/docs/de/vs-code#use-the-prompt-box) mit `/` und wählen Sie **Output styles**, um einen Stil auszuwählen, einschließlich Ihrer benutzerdefinierten Stile. Erfordert Claude Code v2.1.257 oder später.
* **Desktop-App**: Legen Sie das Feld `outputStyle` in einer Einstellungsdatei fest, beispielsweise `.claude/settings.local.json`, die Datei, in die das Terminal-Menü schreibt. Wenn Sie dort `/config` ausführen, öffnet Claude Code [**Einstellungen > Claude Code**](/docs/de/desktop#what%E2%80%99s-not-available-in-desktop) statt eines Menüs.

Um einen Stil ohne Menü festzulegen, bearbeiten Sie das Feld `outputStyle` direkt in einer Einstellungsdatei:

```json theme={null}
{
  "outputStyle": "Explanatory"
}
```

Der Wert ist Groß-/Kleinschreibung-empfindlich, daher schreiben Sie die integrierten Namen als `Proactive`, `Concise`, `Explanatory` und `Learning`. Ein Wert, der nicht exakt mit einem Stilnamen übereinstimmt, wie `explanatory`, gibt Ihnen den Standard-Stil. Der `/output-style` Befehl ignoriert die Groß-/Kleinschreibung.

Um einen Stil über Projekte hinweg als Standard festzulegen, legen Sie `outputStyle` in `~/.claude/settings.json` fest. Die eigenen Einstellungsdateien eines Projekts [haben Vorrang](/docs/de/settings#settings-precedence) vor diesem Wert.

Wenn Sie Stile während einer Sitzung wechseln, verwendet Claude den neuen Stil ab Ihrer nächsten Nachricht. Informationen zu den Kosten dieser ersten Nachricht für Prompt Caching finden Sie unter [Ausgabestil ändern](/docs/de/prompt-caching#changing-output-style). Vor v2.1.251 wurde der neue Stil erst nach dem Ausführen von `/clear` oder dem Starten einer neuen Sitzung angewendet.

<h2 id="create-a-custom-output-style">
  Erstellen Sie einen benutzerdefinierten Ausgabestil
</h2>

Ein benutzerdefinierter Ausgabestil ist eine Markdown-Datei: Frontmatter für Metadaten, dann die Anweisungen für Claude.

In der VS Code-Erweiterung können Sie die Datei auch über das Menü [**Output styles**](/docs/de/vs-code#use-the-prompt-box) erstellen, anstatt sie von Hand zu schreiben. Dies erfordert Claude Code v2.1.261 oder später.

<Steps>
  <Step title="Erstellen Sie eine Markdown-Datei">
    Speichern Sie sie auf einer von drei Ebenen. Der Dateiname wird zum Stilnamen, es sei denn, Sie legen `name` im Frontmatter fest.

    * Benutzer: `~/.claude/output-styles`
    * Projekt: `.claude/output-styles`
    * Verwaltete Richtlinie: `.claude/output-styles` im [Verzeichnis für verwaltete Einstellungen](/docs/de/managed-settings#delivery-mechanisms)

    Projekt-Ausgabestile werden aus jedem `.claude/output-styles/` zwischen dem Arbeitsverzeichnis und dem Repository-Root geladen. Wenn mehr als eines dieser verschachtelten Verzeichnisse einen Stil mit demselben Namen definiert, verwendet Claude Code denjenigen, der dem Arbeitsverzeichnis am nächsten ist.
  </Step>

  <Step title="Fügen Sie Frontmatter und Anweisungen hinzu">
    Entscheiden Sie, ob Sie die Softwareentwicklungsanweisungen von Claude Code beibehalten möchten. Setzen Sie `keep-coding-instructions: true`, wenn Sie ändern, wie Claude kommuniziert, aber möchten, dass es auf die gleiche Weise codiert. Lassen Sie es weg, wenn Claude keine Softwareentwicklung durchführt.

    Dieses Beispiel leitet jede Erklärung mit einem Diagramm ein und behält Claudes Codierungsverhalten bei:

    ```markdown theme={null}
    ---
    name: Diagrams first
    description: Lead every explanation with a diagram
    keep-coding-instructions: true
    ---

    When explaining code, architecture, or data flow, start with a Mermaid diagram showing the structure, then explain in prose.

    ## Diagram conventions

    Use `flowchart TD` for control flow and `sequenceDiagram` for request paths. Keep diagrams under 15 nodes.
    ```
  </Step>

  <Step title="Wechseln Sie zu Ihrem Stil">
    Führen Sie `/output-style <style>` im Terminal aus, oder führen Sie `/config` aus und wählen Sie Ihren Stil unter **Output style**. Claude verwendet den neuen Stil ab Ihrer nächsten Nachricht. Im Terminal liest Claude Code Stildateien beim Start, daher müssen Sie Claude Code neu starten, wenn Sie eine Datei während einer laufenden Sitzung erstellen oder bearbeiten, um die Änderung zu übernehmen.
  </Step>
</Steps>

[Plugins](/docs/de/plugins/manifest-reference) können auch Ausgabestile in einem `output-styles/`-Verzeichnis bereitstellen.

<h3 id="frontmatter">
  Frontmatter-Referenz
</h3>

Konfigurieren Sie einen Ausgabestil mit YAML-[Frontmatter](/docs/de/glossary#frontmatter) zwischen `---`-Markierungen am Anfang der Datei. Alle Felder sind optional, und Feldnamen verwenden Kleinbuchstaben, die durch Bindestriche getrennt sind. Ein falsch geschriebenes Feld wird ohne Fehler ignoriert. Wenn das YAML nicht analysiert wird, wird der Stil trotzdem unter seinem Dateinamen mit keinen festgelegten Feldern geladen; führen Sie `claude --debug` aus, um den Analysefehler zu sehen.

| Feld                       | Erforderlich | Beschreibung                                                                                                                                                                                                                                                                                                                          |
| :------------------------- | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`                     | Nein         | Name des Ausgabestils, angezeigt in der `/config`-Auswahl. Standard: der Dateiname                                                                                                                                                                                                                                                    |
| `description`              | Nein         | Beschreibung des Ausgabestils, angezeigt in der `/config`-Auswahl                                                                                                                                                                                                                                                                     |
| `keep-coding-instructions` | Nein         | Setzen Sie auf `true`, um die integrierten Softwareentwicklungsanweisungen von Claude Code neben Ihrem Stil beizubehalten. Standard: `false`                                                                                                                                                                                          |
| `force-for-plugin`         | Nein         | Nur Plugin-Ausgabestile. Setzen Sie auf `true`, um diesen Stil automatisch anzuwenden, wenn das Plugin aktiviert ist, ohne dass Benutzer ihn auswählen müssen. Überschreibt die `outputStyle`-Einstellung des Benutzers. Wenn mehrere aktivierte Plugins dies festlegen, verwendet Claude Code das zuerst geladene. Standard: `false` |

<span id="comparisons-to-related-features" />

<h2 id="choose-between-an-output-style-and-other-features">
  Wählen Sie zwischen einem Ausgabestil und anderen Funktionen
</h2>

Ein Ausgabestil gilt für jede Antwort in einer Sitzung. Es ist eine Anweisung, die Claude befolgt, daher wird nichts erzwungen. Wenn das, was Sie möchten, enger ist als jede Antwort, oder ohne Fehler geschehen muss, passt eine andere Funktion besser.

Diese Tabelle ordnet das, was Sie möchten, der Funktion zu, die es tut:

| Sie möchten                                                                                                               | Verwenden Sie                                                     | Warum es passt                                                                                                                           |
| :------------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| Jede Antwort in einer bestimmten Stimme, Länge oder Format, oder Claude in einer anderen Rolle                            | Ein Ausgabestil                                                   | Er gilt für die gesamte Sitzung, und Sie wechseln Stile mit einem Befehl                                                                 |
| Claude sollte die Konventionen, Befehle und Struktur Ihres Projekts kennen                                                | [CLAUDE.md](/docs/de/memory)                                           | Es enthält, was Claude über die Codebasis wissen sollte, und es bleibt geladen, welchen Stil Sie auch wählen                             |
| Anweisungen für eine Art von Aufgabe, wie eine Veröffentlichungs-Checkliste oder ein Überprüfungsverfahren                | Ein [Skill](/docs/de/skills)                                           | Claude lädt es nur, wenn Sie es aufrufen oder die Aufgabe passt, daher formt es keine unabhängigen Antworten                             |
| Etwas, das jedes Mal ohne Ausnahme geschehen muss, wie Formatierung nach jeder Bearbeitung oder Blockierung eines Befehls | Ein [Hook](/docs/de/hooks-guide)                                       | Claude Code führt einen Hook selbst bei einem Lebenszyklusereignis aus, daher hängt es nicht davon ab, dass Claude einer Anweisung folgt |
| Ein Helfer mit seinen eigenen Anweisungen, Modell und Tools für eine fokussierte Aufgabe                                  | Ein [Subagent](/docs/de/sub-agents)                                    | Er läuft in einem separaten Kontext mit seinem eigenen Systemprompt und gibt eine Zusammenfassung an Ihr Gespräch zurück                 |
| Eine Ergänzung zu Claudes Anweisungen, die Sie beim Start von Claude Code übergeben                                       | [`--append-system-prompt`](/docs/de/cli-reference#system-prompt-flags) | Sie wird an den Systemprompt angehängt, ohne etwas zu entfernen                                                                          |

Diese Funktionen lassen sich kombinieren. Sie können beispielsweise CLAUDE.md für das verwenden, was Claude wissen sollte, einen Ausgabestil für die Art, wie es antwortet, und einen Hook für alles, das garantiert sein muss. [Erweitern Sie Claude Code](/docs/de/features-overview) vergleicht die restlichen Erweiterungsfunktionen.

<h2 id="how-output-styles-work">
  Wie Ausgabestile funktionieren
</h2>

Ein Ausgabestil ändert die Anweisungen, die Claude Code an Claude sendet.

* Claude Code sendet die Anweisungen des aktiven Stils mit jeder Anfrage.
* Benutzerdefinierte Ausgabestile lassen Claude Codes integrierte Softwareentwicklungsanweisungen aus, wie z. B. wie man Änderungen begrenzt, Kommentare schreibt und Arbeiten überprüft, es sei denn, `keep-coding-instructions` ist auf `true` gesetzt.

Ausgabestile gelten für die Hauptkonversation und für einen [Fork](/docs/de/sub-agents#fork-the-current-conversation), der die vollständige Konversation und Systemaufforderung des übergeordneten Elements erbt. Andere [Subagents führen ihre eigene Systemaufforderung aus](/docs/de/sub-agents#what-loads-at-startup), daher ändern Stile nicht, wie sie reagieren.

Die Tokennutzung hängt vom Stil ab. Die Anweisungen eines Stils fügen Eingabe-Token hinzu, obwohl Prompt Caching diese Kosten nach der ersten Anfrage in einer Sitzung reduziert.

Die integrierten Stile „Explanatory" und „Learning" erzeugen absichtlich längere Antworten als „Default", was die Ausgabe-Token erhöht. Der Stil „Concise" bewirkt das Gegenteil, indem Claude angewiesen wird, Antworten standardmäßig kurz zu halten. Bei benutzerdefinierten Stilen hängt die Ausgabe-Token-Nutzung davon ab, was Ihre Anweisungen Claude zu produzieren anweisen.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Settings](/docs/de/settings): wo das Feld `outputStyle` lebt und wie die Einstellungspriorität funktioniert
* [Permission modes](/docs/de/permission-modes): wie der Proactive-Stil den Auto-Modus vergleicht
* [Plugins](/docs/de/plugins/overview): Verpacken und verteilen Sie Ausgabestile zusammen mit Skills, Hooks und Agents
* [Debug your configuration](/docs/de/debug-your-config): Diagnostizieren Sie, warum ein Ausgabestil nicht wirksam wird
