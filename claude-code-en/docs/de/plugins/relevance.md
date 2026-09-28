> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins für Ihre Organisation empfehlen

> Fügen Sie einen Relevanzblock zu Marketplace-Plugin-Einträgen hinzu, damit Claude Code diese vorschlägt, wenn die Arbeit eines Benutzers übereinstimmt, und erlauben Sie den Marketplace in verwalteten Einstellungen.

Claude Code kann die Installation eines Plugins aus dem Marketplace Ihrer Organisation vorschlagen, wenn eine Benutzersitzung Signale erfüllt, die Sie für dieses Plugin definieren. Signale umfassen das Arbeitsverzeichnis, Dateien, die Claude gelesen hat, und Befehle, die Claude ausgeführt hat. Sie definieren diese, indem Sie einen `relevance`-Block zum Plugin-Eintrag in `marketplace.json` hinzufügen.

Ein Marketplace-Betreiber schreibt die `relevance`-Einträge. Ein Administrator erlaubt dann den Marketplace in verwalteten Einstellungen. Benutzer sehen keine Vorschläge aus einem Marketplace, bis dieser erlaubt ist.

<Note>
  Diese Fälle werden auf anderen Seiten behandelt:

  * **Sie möchten Plugins installieren**: siehe [Plugins installieren und verwalten](/docs/de/plugins/install)
  * **Sie möchten Vorschläge deaktivieren**: siehe [Verstehen Sie, wie Plugin-Relevanz funktioniert](#understand-how-plugin-relevance-works)
</Note>

Beginnen Sie mit den Abschnitten für Ihre Rolle:

* **Marketplace-Betreiber**: lesen Sie [wie Vorschläge funktionieren](#understand-how-plugin-relevance-works), dann [fügen Sie Relevanz zu einem Plugin-Eintrag hinzu](#add-relevance-to-a-plugin-entry) und [validieren Sie Ihren Marketplace](#validate-your-marketplace)
* **Administratoren**: [aktivieren Sie Vorschläge in verwalteten Einstellungen](#enable-suggestions-in-managed-settings)

<h2 id="understand-how-plugin-relevance-works">
  Verstehen Sie, wie Plugin-Relevanz funktioniert
</h2>

Jeder Plugin-Eintrag in `marketplace.json` kann ein `relevance`-Objekt enthalten. Das Objekt benennt ein Thema und ein oder mehrere Signale. Ein Signal ist ein Muster, das Claude Code gegen die aktuelle Sitzung testet, z. B. das Arbeitsverzeichnis oder Dateien, die Claude gelesen hat.

Die Signalabstimmung erfolgt lokal auf dem Computer des Benutzers und erzeugt keinen Netzwerkverkehr. Claude Code meldet nicht, welche Signale abgestimmt wurden oder deren Werte an Anthropic oder den Marketplace-Betreiber.

Wenn ein Signal abgestimmt wird und das Plugin nicht bereits installiert ist, schlägt Claude Code das Plugin an diesen Stellen vor:

* **Spinner-Tipp**: Eine Nachricht mit dem Befehl `/plugin install` wird unter dem Spinner angezeigt, während Claude antwortet.
* **Benachrichtigung beim Sitzungsstart**: Wenn ein `cwd`-Signal mit dem Arbeitsverzeichnis abgestimmt wird, wird eine einzeilige Benachrichtigung angezeigt, bevor der Benutzer eine erste Nachricht sendet.
* **`/plugin` Discover-Registerkarte**: Das Plugin wird oben in der Discover-Liste angeheftet.

[Vorschau, was der Benutzer sieht](#preview-what-the-user-sees) zeigt den genauen Text jedes und wie oft sie sich wiederholen.

Claude Code installiert das Plugin niemals automatisch. Der Benutzer bestätigt immer.

Der Spinner-Tipp und die Benachrichtigung beim Sitzungsstart werden beide nicht mehr angezeigt, wenn der Benutzer oder das Projekt [`spinnerTipsEnabled`](/docs/de/settings-reference#spinnertipsenabled) auf `false` setzt, oder wenn ein [`spinnerTipsOverride`](/docs/de/settings-reference#spinnertipsoverride) mit `excludeDefault` die integrierten Tipps ersetzt. Die Discover-Registerkarten-Anheftung wird durch keine der beiden Einstellungen beeinflusst.

<h2 id="add-relevance-to-a-plugin-entry">
  Fügen Sie Relevanz zu einem Plugin-Eintrag hinzu
</h2>

Fügen Sie ein `relevance`-Objekt zum Plugin-Eintrag in Ihrer `marketplace.json` hinzu. Das folgende Beispiel erklärt, dass das Plugin `terraform-helpers` relevant ist, wenn Claude eine `.tf`-Datei liest oder `terraform` ausführt:

```json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "plugins": [
    {
      "name": "terraform-helpers",
      "source": "./plugins/terraform-helpers",
      "description": "Your organization's Terraform conventions and helpers",
      "relevance": {
        "topic": "Terraform",
        "signals": {
          "cli": ["terraform"],
          "filesRead": ["**/*.tf"]
        }
      }
    }
  ]
}
```

Solange keines seiner Signale abgestimmt wird, behält das Plugin seine normale Position in der Discover-Liste und wird nicht als Spinner-Tipp angezeigt.

Um den Block vor der Veröffentlichung zu überprüfen, [validieren Sie Ihren Marketplace](#validate-your-marketplace).

<h2 id="field-reference">
  Feldverweis
</h2>

Das `relevance`-Objekt und sein verschachteltes `signals`-Objekt akzeptieren die Felder in den folgenden Tabellen.

Ältere Clients laden weiterhin einen Marketplace, der `relevance`-Felder verwendet, die sie nicht erkennen, da unbekannte Felder unter `relevance` und `relevance.signals` beim Laden ignoriert werden. Ein erkanntes Feld, dessen Wert sein Limit im [Feldverweis](#field-reference) überschreitet, macht den gesamten Plugin-Eintrag ungültig, und Benutzer können dieses Plugin nicht aus dem Marketplace installieren, bis Sie es beheben; `claude plugin validate` meldet die gleichen Limits.

<h3 id="relevance">
  `relevance`
</h3>

| Feld      | Typ    | Beschreibung                                                                                                                                                                           |
| :-------- | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `topic`   | string | Optional. Der Ausdruck, der „Arbeiten mit *topic*?" im Spinner-Tipp ausfüllt. Standardmäßig der Plugin-Name mit jedem Bindestrich-Segment kapitalisiert. Maximal 64 Zeichen.           |
| `signals` | object | Matcher, die bestimmen, wann das Plugin relevant ist. Claude Code schlägt das Plugin nur vor, wenn mindestens ein Signal gesetzt ist. Siehe [`relevance.signals`](#relevance-signals). |

Das `topic` ist oft der Produktname, z. B. `Terraform`. Verwenden Sie eine Domäne wie `design`, wenn der Plugin-Name nicht natürlich als Thema klingt.

<h3 id="relevance-signals">
  `relevance.signals`
</h3>

Das `signals`-Objekt akzeptiert die folgenden Felder.

| Feld           | Typ              | Beschreibung                                                                                                                                                                                                                                                                  | Limit                                                                                              |
| :------------- | :--------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------- |
| `cwd`          | array of strings | Glob-Muster, die gegen das Arbeitsverzeichnis der Sitzung abgestimmt werden. Siehe [Arbeitsverzeichnis-Abgleich](#working-directory-matching).                                                                                                                                | 10 Muster à 256 Zeichen                                                                            |
| `cli`          | array of strings | Befehlsnamen aus Shell-Befehlen, die Claude in dieser Sitzung ausgeführt hat, z. B. `["terraform"]`. Exakte Übereinstimmung. Siehe [Befehlsnamen-Abgleich](#command-name-matching).                                                                                           | 10 Einträge à 64 Zeichen                                                                           |
| `hosts`        | array of strings | Hostnamen in `http://`- oder `https://`-URLs in Bash-Befehlen in dieser Sitzung, z. B. `["registry.terraform.io"]`. Nur reiner Hostname in Kleinbuchstaben: kein Schema, Port oder Pfad. Exakte Abgleichung ohne Berücksichtigung der Groß-/Kleinschreibung.                  | 20 Einträge à 128 Zeichen                                                                          |
| `filesRead`    | array of strings | Glob-Muster, die gegen die Pfade von Dateien abgestimmt werden, die Claude in dieser Sitzung gelesen hat, z. B. `["**/*.tf"]`. Vorwärtsschrägstrich normalisiert und Groß-/Kleinschreibung ignoriert.                                                                         | 10 Muster à 256 Zeichen                                                                            |
| `manifestDeps` | array of objects | Abhängigkeiten, die in Paketmanifesten deklariert sind, die Claude in dieser Sitzung gelesen hat. Jeder Eintrag ist `{ "file": "...", "pattern": "..." }`, wobei beide Werte reguläre Ausdrücke sind. Siehe [Manifest-Abhängigkeits-Abgleich](#manifest-dependency-matching). | 10 Einträge, jeder Wert maximal 256 Zeichen. Manifestdateien größer als 512 KB werden übersprungen |

Die Signale `filesRead` und `manifestDeps` stimmen auch gegen Dateien ab, die Claude in dieser Sitzung geschrieben oder bearbeitet hat, und gegen die automatisch geladenen `CLAUDE.md`-Speicherdateien des Projekts.

<h4 id="working-directory-matching">
  Arbeitsverzeichnis-Abgleich
</h4>

`cwd` ist das einzige Signal, das beim Sitzungsstart abgestimmt werden kann, bevor der Benutzer eine erste Nachricht sendet.

Claude Code stimmt jedes `cwd`-Muster wie folgt ab:

* Das Muster wird gegen das Arbeitsverzeichnis als absoluter Pfad abgestimmt. Wenn sich die Sitzung in einem Git-Repository befindet, wird es auch gegen den Pfad des Arbeitsverzeichnisses relativ zum Repository-Root abgestimmt.
* Der Abgleich ist vorwärtsschrägstrich normalisiert und Groß-/Kleinschreibung ignoriert.
* Jedes Muster stimmt mit dem Verzeichnis selbst und allem darunter ab, daher verhalten sich `infra`, `infra/` und `infra/**` identisch.

<h4 id="command-name-matching">
  Befehlsnamen-Abgleich
</h4>

Claude Code zeichnet einen Befehlsnamen für jeden Shell-Befehl auf, den Claude ausführt: das erste Token nach allen führenden Umgebungsvariablenzuweisungen und `sudo`. Zusammengesetzte Befehle tragen nur ihren führenden Befehl bei, daher zeichnet `cd infra && terraform plan` `cd` auf, nicht `terraform`.

<h4 id="manifest-dependency-matching">
  Manifest-Abhängigkeits-Abgleich
</h4>

Jeder `manifestDeps`-Eintrag paart zwei JavaScript-`RegExp`-Quellzeichenfolgen:

* `file`: Abgleich ohne Berücksichtigung der Groß-/Kleinschreibung gegen den Pfad der Manifestdatei. Der Pfad ist normalerweise absolut, daher verankern Sie das Muster am Ende statt am Anfang. Pfade werden für dieses Signal nicht separator-normalisiert, daher verwenden Windows-Pfade Backslashes.
* `pattern`: Abgleich mit Berücksichtigung der Groß-/Kleinschreibung gegen den Inhalt dieser Datei.

Das folgende Beispiel verwendet `manifestDeps`, um Ihr Plugin vorzuschlagen, sobald Claude eine `package.json` gelesen hat, die von Ihrem SDK-npm-Paket abhängt, das hier `your-sdk` genannt wird.

```json theme={null}
{
  "name": "your-plugin",
  "source": "./plugins/your-plugin",
  "relevance": {
    "signals": {
      "manifestDeps": [
        {
          "file": "[/\\\\]package\\.json$",
          "pattern": "\"your-sdk\"\\s*:"
        }
      ]
    }
  }
}
```

In diesem Beispiel verwendet das `file`-Muster `[/\\\\]`, damit es sowohl Vorwärts- als auch Backslash-Pfadtrennzeichen abgleicht, und `\\.`, damit der Punkt literal ist. In JSON wird jeder Backslash im regulären Ausdruck zweimal geschrieben.

<h2 id="validate-your-marketplace">
  Validieren Sie Ihren Marketplace
</h2>

Führen Sie in Ihrer Shell `claude plugin validate` gegen Ihr Marketplace-Verzeichnis aus, um den `relevance`-Block vor der Veröffentlichung zu überprüfen:

```bash theme={null}
claude plugin validate ./my-marketplace
```

Der Validator meldet Fehler und Warnungen im `relevance`-Block, einschließlich dieser:

* Meldet unbekannte Schlüssel unter `relevance` und `relevance.signals` als Warnungen
* Kennzeichnet einen `relevance`-Wert, der kein Objekt ist
* Lehnt einen `signals.hosts`-Eintrag ab, der ein Schema, einen Port oder einen Pfad enthält

Jeder Fund wird mit dem Pfad des Feldes gedruckt, das er betrifft, und die Ausgabe endet mit `Validation passed`, `Validation passed with warnings` oder `Validation failed`.

<h2 id="enable-suggestions-in-managed-settings">
  Aktivieren Sie Vorschläge in verwalteten Einstellungen
</h2>

Benutzer sehen keine Vorschläge aus einem Marketplace, bis ein Administrator ihn in [verwalteten Einstellungen](/docs/de/plugins/org) erlaubt, auch wenn seine `marketplace.json` `relevance` deklariert.

Um einen Marketplace zu erlauben, bearbeiten Sie Ihre verwalteten Einstellungen wie folgt:

* Fügen Sie den Marketplace-Namen zu `pluginSuggestionMarketplaces` hinzu.
* Für jeden Marketplace außer dem offiziellen Anthropic-Marketplace deklarieren Sie auch die Marketplace-Quelle, entweder als Eintrag dieses Namens in [`extraKnownMarketplaces`](/docs/de/plugins/org#require-a-marketplace-and-its-plugins) oder als Eintrag in [`strictKnownMarketplaces`](/docs/de/plugins/org#allowlist-with-strictknownmarketplaces).

Auf einem Computer, auf dem der Marketplace nicht registriert ist, oder unter dem erlaubten Namen aus einer anderen Quelle registriert ist, werden keine Vorschläge von ihm angezeigt. Die Quellprüfung verhindert, dass eine unabhängige Quelle sich unter einem erlaubten Namen registriert, um ihre Plugins in Ihrer gesamten Organisation vorgeschlagen zu bekommen.

Die folgende `managed-settings.json` registriert einen Org-Marketplace aus einem GitHub-Repository und aktiviert seine Vorschläge:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": {
        "source": "github",
        "repo": "your-org/your-marketplace"
      }
    }
  },
  "pluginSuggestionMarketplaces": ["your-marketplace"]
}
```

Der Name des offiziellen Marketplace kann sich nur von der offiziellen Anthropic-Quelle registrieren, daher benötigt er keine Quelldeklaration. Für den offiziellen Marketplace erlauben Sie nur den Namen:

```json theme={null}
{
  "pluginSuggestionMarketplaces": ["claude-plugins-official"]
}
```

<h2 id="preview-what-the-user-sees">
  Vorschau, was der Benutzer sieht
</h2>

Wenn ein `relevance`-Signal des Plugins während einer Sitzung abgestimmt wird, lautet der Tipp unter dem Spinner:

```text theme={null}
Working with Terraform? Install the terraform-helpers plugin:
/plugin install terraform-helpers@your-marketplace
```

Wenn ein `cwd`-Signal beim Sitzungsstart abgestimmt wird, lautet die einzeilige Benachrichtigung:

```text theme={null}
plugin suggestion: terraform-helpers@your-marketplace · /plugin
```

In der `/plugin` Discover-Registerkarte wird das Plugin oben in den anderen Ergebnissen mit einer Anmerkung angeheftet, die das abgestimmte Signal benennt, z. B. `suggested for this directory` oder `suggested for terraform commands`.

Claude Code begrenzt, wie oft es ein bestimmtes Plugin vorschlägt:

* Der Vorschlag wird höchstens einmal alle drei Sitzungen über den Spinner-Tipp und die Benachrichtigung beim Sitzungsstart kombiniert angezeigt.
* Die Benachrichtigung beim Sitzungsstart wird nicht mehr angezeigt, sobald der Spinner-Tipp und die Benachrichtigung das Plugin insgesamt zweimal angezeigt haben.
* Weder der Spinner-Tipp noch die Benachrichtigung beim Sitzungsstart wiederholen sich, sobald das Plugin installiert ist.
* Die Discover-Registerkarte heftet das Plugin das erste Mal an, wenn der Benutzer die Registerkarte öffnet, während die Signale des Plugins abgestimmt werden. Claude Code zeichnet das in `~/.claude.json` auf, daher wird das Plugin jedes Mal, wenn der Benutzer später `/plugin` auf diesem Computer öffnet, in normaler Reihenfolge angezeigt.

<h2 id="see-also">
  Siehe auch
</h2>

* [Hosten Sie einen Marketplace](/docs/de/plugins/host-marketplace): Führen Sie den Marketplace aus, der Ihre Plugins hostet
* [Marketplace-Verweis](/docs/de/plugins/marketplace-reference#plugin-entries): Jedes Feld, das ein Plugin-Eintrag akzeptiert
* [Empfehlen Sie Ihr Plugin von Ihrer CLI](/docs/de/plugins/cli-hints): Fordern Sie Benutzer von Ihrer eigenen CLI auf, anstatt von Claude Codes Sitzungssignalen
* [Verwalten Sie Plugins für Ihre Organisation](/docs/de/plugins/org): `extraKnownMarketplaces`, `strictKnownMarketplaces` und der Rest der Plugin-Richtlinienschlüssel
