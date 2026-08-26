> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins für Ihre Organisation empfehlen

> Fügen Sie einen Relevanzblock zu Marketplace-Plugin-Einträgen hinzu, damit Claude Code diese vorschlägt, wenn die Arbeit eines Benutzers passt.

Wenn Sie einen Plugin-Marketplace für Ihre Organisation betreiben, können Sie Claude Code so konfigurieren, dass bestimmte Plugins Benutzern basierend auf ihrer aktuellen Arbeit vorgeschlagen werden. Fügen Sie einen `relevance`-Block zum Plugin-Eintrag in `marketplace.json` hinzu und genehmigen Sie den Marketplace in verwalteten Einstellungen. Wenn eine Benutzersitzung einem der deklarierten Signale entspricht, zeigt Claude Code einen Installationsvorschlag für dieses Plugin an.

Marketplace-deklarierte Vorschläge sind pro Marketplace durch [verwaltete Einstellungen](/docs/de/settings#settings-files) optional. Kein `relevance`-Deklaration eines Marketplace erzeugt Vorschläge, bis ein Administrator ihn zur Genehmigungsliste hinzufügt, einschließlich des offiziellen Anthropic-Marketplace. Claude Code enthält auch einen integrierten Vorschlag, der unabhängig von dieser Genehmigungsliste ist; dieser Tipp und alle Marketplace-deklarierten Tipps sind deaktiviert, wenn [`spinnerTipsEnabled`](/docs/de/settings#available-settings) auf `false` gesetzt ist.

Diese Funktion erfordert Claude Code v2.1.152 oder später. Ältere Clients ignorieren das `relevance`-Feld.

Diese Seite ist für Marketplace-Betreiber und Enterprise-Administratoren. Wenn Sie Plugins installieren möchten, siehe [Plugins entdecken und installieren](/docs/de/discover-plugins).

<h2 id="how-it-works">
  Funktionsweise
</h2>

Jeder Plugin-Eintrag in `marketplace.json` kann ein `relevance`-Objekt enthalten. Das Objekt benennt ein Thema und ein oder mehrere Signale. Ein Signal ist ein Muster, das Claude Code gegen die aktuelle Sitzung testet, z. B. das Arbeitsverzeichnis oder Dateien, die Claude gelesen hat.

Signal-Matching erfolgt lokal auf dem Computer des Benutzers. Das Matching erzeugt keinen Netzwerkverkehr und meldet nicht, welche Signale übereinstimmten oder deren Werte an Anthropic oder den Marketplace-Betreiber.

Wenn ein Signal übereinstimmt und das Plugin nicht bereits installiert ist, zeigt Claude Code das Plugin an drei Stellen an:

* **Spinner-Tipp**: Eine Meldung „Arbeiten mit *Thema*? Installieren Sie das *Plugin*-Plugin" mit dem `/plugin install`-Befehl wird unter dem Spinner angezeigt, während Claude antwortet.
* **Sitzungsstart-Vorschlag**: Wenn das `cwd`-Signal dem Arbeitsverzeichnis entspricht, wird eine einzeilige `plugin suggestion: <name>@<marketplace> · /plugin`-Benachrichtigung vor dem ersten Turn angezeigt. Diese Oberfläche erfordert Claude Code v2.1.153 oder später.
* **`/plugin` Discover-Registerkarte**: Das Plugin wird oben in der Discover-Liste mit einer Anmerkung wie „für dieses Verzeichnis empfohlen" oder „für Stripe-Befehle empfohlen" angeheftet. Diese Oberfläche erfordert Claude Code v2.1.154 oder später.

Der Spinner-Tipp und die Sitzungsstart-Benachrichtigung sind Teil des Spinner-Tipps-Systems. Beide sind deaktiviert, wenn der Benutzer oder das Projekt `spinnerTipsEnabled` auf `false` setzt oder wenn ein benutzerdefinierter `spinnerTipsOverride` mit `excludeDefault` konfiguriert ist. Das Anheften auf der Discover-Registerkarte ist unabhängig von Tipp-Einstellungen.

Claude Code installiert ein Plugin niemals automatisch. Der Benutzer bestätigt immer.

<h2 id="add-relevance-to-a-plugin-entry">
  Relevanz zu einem Plugin-Eintrag hinzufügen
</h2>

Fügen Sie ein `relevance`-Objekt zum Plugin-Eintrag in Ihrer `marketplace.json` hinzu. Das folgende Beispiel deklariert, dass das `terraform-helpers`-Plugin relevant ist, wenn Claude eine `.tf`-Datei liest oder wenn Claude `terraform` ausführt:

```json theme={null}
{
  "name": "acme-corp-plugins",
  "owner": { "name": "Acme Platform Team" },
  "plugins": [
    {
      "name": "terraform-helpers",
      "source": "./plugins/terraform-helpers",
      "description": "Acme conventions and helpers for Terraform",
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

Ein Plugin mit einem `relevance`-Block aber ohne übereinstimmendes Signal verhält sich wie jeder andere Marketplace-Eintrag. Es wird in der Discover-Liste an seiner normalen Position angezeigt und wird niemals als Spinner-Tipp angezeigt.

<h2 id="field-reference">
  Feldverweis
</h2>

<h3 id="relevance">
  `relevance`
</h3>

| Feld      | Typ    | Beschreibung                                                                                                                                                                                                                                                                                                                                                                           |
| :-------- | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `topic`   | string | Optional. Der Ausdruck, der „Arbeiten mit *Thema*?" im Spinner-Tipp ausfüllt. Oft der Produktname, z. B. `Stripe`. Verwenden Sie eine Domäne wie `design`, wenn der Plugin-Name nicht natürlich als Thema gelesen wird. Standardmäßig der Plugin-Name mit jedem Bindestrich-Segment kapitalisiert. Die Sitzungsstart-Benachrichtigung verwendet diesen Wert nicht. Maximal 64 Zeichen. |
| `signals` | object | Matcher, die bestimmen, wann das Plugin relevant ist. Mindestens ein Signal ist erforderlich, damit das Plugin vorschlagbar ist. Siehe die Tabelle unten.                                                                                                                                                                                                                              |

<h3 id="relevance-signals">
  `relevance.signals`
</h3>

| Feld           | Typ              | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :------------- | :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `cwd`          | array of strings | Glob-Muster, die gegen das Arbeitsverzeichnis der Sitzung abgeglichen werden. Abgeglichen als absoluter Pfad und, wenn sich in einem Git-Repository befindet, als Pfad relativ zum Repository-Root. Schrägstrich-normalisiert und Groß-/Kleinschreibung-insensitiv. Jedes Muster passt zum Verzeichnis selbst und allem darunter, daher verhalten sich `infra`, `infra/` und `infra/**` identisch. Dies ist das einzige Signal, das beim Sitzungsstart vor dem ersten Turn übereinstimmen kann. Maximal 10 Muster mit je 256 Zeichen.                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `cli`          | array of strings | Befehlsnamen aus Shell-Befehlen, die Claude in dieser Sitzung ausgeführt hat, z. B. `["stripe"]`. Gilt auf jeder Plattform: Befehle, die unter Windows über PowerShell oder Git Bash ausgeführt werden, werden auf die gleiche Weise aufgezeichnet. Claude Code zeichnet einen Befehlsnamen pro Shell-Tool-Aufruf auf: das erste Token nach allen führenden Umgebungsvariablenzuweisungen und `sudo`. Zusammengesetzte Befehle tragen nur ihren führenden Befehl bei, daher zeichnet `cd infra && terraform plan` `cd` auf, nicht `terraform`. Exakte Übereinstimmung. Maximal 10 Einträge mit je 64 Zeichen.                                                                                                                                                                                                                                                                                                                                                       |
| `hosts`        | array of strings | Hostnamen in `http://`- oder `https://`-URLs in Bash-Befehlen dieser Sitzung, z. B. `["api.stripe.com"]`. Nur reiner Hostname in Kleinbuchstaben: kein Schema, Port oder Pfad. Exakte Groß-/Kleinschreibung-insensitive Übereinstimmung. Maximal 20 Einträge mit je 128 Zeichen.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `filesRead`    | array of strings | Glob-Muster, die gegen die Pfade von Dateien abgeglichen werden, die Claude in dieser Sitzung gelesen hat, z. B. `["**/*.tf"]`. Schrägstrich-normalisiert und Groß-/Kleinschreibung-insensitiv. Maximal 10 Muster mit je 256 Zeichen.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `manifestDeps` | array of objects | Abhängigkeiten, die in Paketmanifesten deklariert sind, die Claude in dieser Sitzung gelesen hat. Jeder Eintrag ist `{ "file": "...", "pattern": "..." }`, wobei `file` ein regulärer Ausdruck ist, der gegen den Pfad der Manifestdatei abgeglichen wird, wie er im Sitzungszustand aufgezeichnet ist, typischerweise ein absoluter Pfad, und `pattern` ein regulärer Ausdruck ist, der gegen den Inhalt dieser Datei abgeglichen wird. Verankern Sie `file` am Ende, z. B. `[/\\\\]package\\.json$` in JSON-escaped-Form, da ein am Anfang verankertes Muster niemals einen absoluten Pfad passt. Pfade sind nicht Separator-normalisiert für dieses Signal, daher verwenden Windows-Pfade Backslashes. Manifestdateien größer als 512 KB werden übersprungen. Beide Werte sind JavaScript-`RegExp`-Quellzeichenfolgen mit maximal 256 Zeichen. `file` passt Groß-/Kleinschreibung-insensitiv. `pattern` ist Groß-/Kleinschreibung-sensitiv. Maximal 10 Einträge. |

Die Signale `cli`, `hosts`, `filesRead` und `manifestDeps` benötigen Sitzungsverlauf, daher können sie nur auf dem Spinner-Tipp und der Discover-Registerkarte übereinstimmen. Nur `cwd` kann beim Sitzungsstart übereinstimmen. Die Signale `filesRead` und `manifestDeps` testen den aufgezeichneten Dateizustand der Sitzung, der auch Dateien enthält, die Claude geschrieben oder bearbeitet hat, und automatisch geladene `CLAUDE.md`-Speicherdateien.

Das folgende Beispiel verwendet `manifestDeps`, um ein Stripe-Plugin vorzuschlagen, sobald Claude eine `package.json` gelesen hat, die von `stripe` abhängt. Das `file`-Muster verwendet `[/\\\\]`, damit es sowohl Schrägstrich- als auch Backslash-Pfad-Trennzeichen passt, und `\\.`, damit der Punkt literal ist. In JSON wird jeder Backslash im regulären Ausdruck zweimal geschrieben.

```json theme={null}
{
  "name": "stripe-helpers",
  "source": "./plugins/stripe-helpers",
  "relevance": {
    "topic": "Stripe",
    "signals": {
      "manifestDeps": [
        {
          "file": "[/\\\\]package\\.json$",
          "pattern": "\"stripe\"\\s*:"
        }
      ]
    }
  }
}
```

<Note>
  Unbekannte Felder unter `relevance` und `relevance.signals` werden beim Laden ignoriert, damit ältere Claude Code-Clients Ihren Marketplace weiterhin laden. Führen Sie `claude plugin validate` aus, um sie als Warnungen anzuzeigen.
</Note>

<h2 id="enable-suggestions-in-managed-settings">
  Vorschläge in verwalteten Einstellungen aktivieren
</h2>

Das Deklarieren von `relevance` in `marketplace.json` ist allein nicht ausreichend. Ein Administrator muss den Marketplace in [verwalteten Einstellungen](/docs/de/settings#settings-files) genehmigen, bevor seine Vorschläge Benutzern angezeigt werden.

Fügen Sie den Marketplace-Namen zu `pluginSuggestionMarketplaces` hinzu. Für jeden Marketplace außer dem offiziellen Anthropic-Marketplace deklarieren Sie auch die Marketplace-Quelle in denselben verwalteten Einstellungen, entweder als Eintrag dieses Namens in `extraKnownMarketplaces` oder als Eintrag in `strictKnownMarketplaces`. Der genehmigte Name wird ignoriert, wenn der auf dem Computer registrierte Marketplace aus einer anderen Quelle stammt. Dies verhindert, dass eine unabhängige Quelle sich unter einem genehmigten Namen registriert, um ihre Plugins in Ihrer Organisation vorgeschlagen zu bekommen.

Die folgende `managed-settings.json` registriert einen Organisations-Marketplace aus einem GitHub-Repository und aktiviert seine Vorschläge:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "acme-corp-plugins": {
      "source": {
        "source": "github",
        "repo": "acme-corp/claude-plugins"
      }
    }
  },
  "pluginSuggestionMarketplaces": ["acme-corp-plugins"]
}
```

Der offizielle Marketplace ist von der Quellendeklarationsanforderung befreit, da sein Name nur von der offiziellen Anthropic-Quelle registriert werden kann. Das Genehmigen des Namens allein ist ausreichend:

```json theme={null}
{
  "pluginSuggestionMarketplaces": ["claude-plugins-official"]
}
```

Siehe die [Einstellungsreferenz](/docs/de/settings) für `pluginSuggestionMarketplaces` und [`extraKnownMarketplaces`](/docs/de/settings#extraknownmarketplaces) für vollständige Konfigurationsdetails.

<h2 id="what-the-user-sees">
  Was der Benutzer sieht
</h2>

Wenn ein Signal während einer Sitzung übereinstimmt, lautet der Spinner-Tipp:

```text theme={null}
Working with Terraform? Install the terraform-helpers plugin:
/plugin install terraform-helpers@acme-corp-plugins
```

Beim Sitzungsstart zeigt ein übereinstimmendes `cwd`-Signal die einzeilige Benachrichtigung:

```text theme={null}
plugin suggestion: terraform-helpers@acme-corp-plugins · /plugin
```

Der Vorschlag eines bestimmten Plugins wird höchstens einmal alle drei Sitzungen über den Spinner-Tipp und die Sitzungsstart-Benachrichtigung kombiniert angezeigt, und keiner wiederholt sich, sobald das Plugin installiert ist. Die Sitzungsstart-Benachrichtigung stoppt zusätzlich, nachdem der Vorschlag zweimal angezeigt wurde.

In der `/plugin` Discover-Registerkarte wird das Plugin oben in den anderen Ergebnissen mit einer Anmerkung angeheftet, die das übereinstimmende Signal benennt, z. B. `suggested for this directory` oder `suggested for terraform commands`. Die Discover-Registerkarte heftet ein bestimmtes Plugin einmal an; spätere Besuche listen es in normaler Reihenfolge auf. Das Anheften auf der Discover-Registerkarte erfordert Claude Code v2.1.154 oder später. In v2.1.152 wird nur der Spinner-Tipp angezeigt; die Sitzungsstart-Benachrichtigung wird in v2.1.153 hinzugefügt.

<h2 id="validate-your-marketplace">
  Validieren Sie Ihren Marketplace
</h2>

Führen Sie `claude plugin validate` gegen Ihr Marketplace-Verzeichnis aus, um den `relevance`-Block vor der Veröffentlichung zu überprüfen:

```
claude plugin validate ./my-marketplace
```

Der Validator meldet unbekannte Schlüssel unter `relevance` und `relevance.signals` als Warnungen, kennzeichnet einen `relevance`-Wert, der kein Objekt ist, und lehnt einen `signals.hosts`-Eintrag ab, der ein Schema, einen Port oder einen Pfad enthält.

<h2 id="see-also">
  Siehe auch
</h2>

* [Erstellen und verteilen Sie einen Plugin-Marketplace](/docs/de/plugin-marketplaces): Erstellen Sie den Marketplace, der Ihre Plugins hostet
* [Empfehlen Sie Ihr Plugin von Ihrer CLI](/docs/de/plugin-hints): Fordern Sie Benutzer von Ihrer eigenen CLI auf, anstatt von Claude Code's Sitzungssignalen
* [Einstellungen](/docs/de/settings): Vollständige Referenz für `pluginSuggestionMarketplaces` und `extraKnownMarketplaces`
