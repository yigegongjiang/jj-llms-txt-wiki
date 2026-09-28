> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ein Marketplace hosten und verwalten

> Veröffentlichen Sie einen Plugin-Marketplace, auf den Benutzer zugreifen können, gewähren Sie Zugriff auf einen privaten Marketplace, und geben Sie Updates und Umbenennungen frei, ohne Installationen zu unterbrechen.

Ein Marketplace zu hosten bedeutet, Ihre `marketplace.json`-Katalogdatei an einem Ort bereitzustellen, an dem andere Benutzer sie mit `/plugin marketplace add` hinzufügen können, ihre Plugins installieren und Ihre Änderungen nach dem Hochladen weiterhin erhalten.

Diese Seite ist für die Person gedacht, die einen Marketplace betreibt.

<Note>
  Diese Fälle werden auf anderen Seiten behandelt:

  * **Sie haben die Katalogdatei noch nicht geschrieben**: Beginnen Sie mit [Erstellen Sie einen Marketplace](/docs/de/plugins/create-marketplace)
  * **Sie sind ein Administrator, der Marketplaces auf den Computern Ihrer Organisation anfordert, einschränkt oder vorinstalliert**: Lesen Sie [Verwalten Sie Plugins für Ihre Organisation](/docs/de/plugins/org)
</Note>

Beginnen Sie mit [Hosten Sie Ihren Marketplace](#host-your-marketplace), um einen Host auszuwählen und den Befehl festzulegen, den Ihre Benutzer ausführen. Lesen Sie [Halten Sie Benutzer auf dem neuesten Stand](#keep-users-up-to-date), bevor Sie Ihre erste Veröffentlichung durchführen. Lesen Sie [Benennen Sie ein Plugin um oder entfernen Sie es](#rename-or-remove-a-plugin), bevor Sie den `name` eines Plugins ändern.

<h2 id="host-your-marketplace">
  Hosten Sie Ihren Marketplace
</h2>

Sie können den Marketplace auf GitHub, auf einem anderen Git-Host, als gehostete `marketplace.json`-URL oder in einem Verzeichnis auf einem gemeinsamen Dateisystem hosten. Senden Sie Ihren Benutzern den Befehl zum Hinzufügen für Ihren Host und teilen Sie ihnen mit, was sie auf ihrer Maschine benötigen:

| Host                                                                  | Benutzer führen aus, in einer Claude Code-Sitzung                      | Was Benutzer benötigen                                                                                                                                              |
| :-------------------------------------------------------------------- | :--------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| GitHub                                                                | `/plugin marketplace add your-org/your-marketplace`                    | `git` und für ein privates Repository den unter [Gewähren Sie Zugriff auf einen privaten Marketplace](#grant-access-to-a-private-marketplace) beschriebenen Zugriff |
| GitLab, Bitbucket, GitHub Enterprise Server oder ein anderer Git-Host | `/plugin marketplace add https://gitlab.example.com/team/plugins.git`  | `git` und Zugriff auf den Host von ihrer Maschine. Senden Sie die vollständige URL, da die Kurzform `owner/repo` immer github.com bedeutet                          |
| Eine gehostete `marketplace.json`-URL                                 | `/plugin marketplace add https://plugins.example.com/marketplace.json` | HTTPS-Zugriff auf die URL. Benutzer benötigen `git` nicht für den Katalog selbst                                                                                    |
| Ein Verzeichnis auf einem gemeinsamen Dateisystem                     | `/plugin marketplace add /Volumes/shared/claude-plugins`               | Lesezugriff auf den Pfad                                                                                                                                            |

Um einen Branch oder Tag eines GitHub- oder Git-URL-Marketplace zu fixieren, teilen Sie Benutzern mit, dass sie `#<ref>` anhängen sollen, wie in `your-org/your-marketplace#stable`. Die [Plugin-Befehls-Referenz](/docs/de/plugins/cli-reference#plugin-marketplace-add) listet jede Form auf, die der Befehl akzeptiert.

Ein erfolgreiches Hinzufügen gibt `Successfully added marketplace: your-marketplace` aus. Claude Code nimmt diesen Namen aus dem `name`-Feld in Ihrer `marketplace.json`, nicht aus dem Repository-Namen.

Benutzer installieren dann ein Plugin nach dem `name` des Eintrags und dem `name` des Marketplace, wie in `/plugin install code-formatter@your-marketplace`.

<h3 id="register-the-marketplace-for-everyone-in-a-repository">
  Registrieren Sie den Marketplace für alle in einem Repository
</h3>

Um den Marketplace mit allen zu teilen, die in einem Repository arbeiten, führen Sie `claude plugin marketplace add your-org/your-marketplace --scope project` dort einmal aus Ihrer Shell aus und committen Sie die `.claude/settings.json`, die sie schreibt. Claude Code registriert dann den Marketplace für jeden Teamkollegen, der den Ordner [vertraut](/docs/de/plugins/org#require-plugins-per-repository).

<h3 id="avoid-relative-path-entries-in-a-url-hosted-marketplace">
  Vermeiden Sie relative Pfad-Einträge in einem URL-gehosteten Marketplace
</h3>

Wenn Benutzer Ihren Marketplace als bloße `marketplace.json`-URL hinzufügen, lädt Claude Code nur diese Datei herunter. Ein Eintrag in Ihrem `plugins`-Array, dessen `source` ein relativer Pfad wie `./plugins/formatter` ist, schlägt dann bei der Installation mit [`its marketplace entry path does not stay inside the marketplace directory`](/docs/de/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces) fehl. Geben Sie jedem Eintrag eine Quelle, die eigenständig abgerufen werden kann, wie ein `github`-Repository oder eine `archive`-URL, oder hosten Sie den Marketplace in einem Git-Repository, damit Claude Code den gesamten Baum klont.

<h3 id="edit-plugins-in-place-on-a-shared-directory">
  Bearbeiten Sie Plugins direkt in einem gemeinsamen Verzeichnis
</h3>

Wenn Benutzer Ihren Marketplace aus einem gemeinsamen Verzeichnis hinzufügen, liest Claude Code Plugins mit relativen Pfad-Quellen direkt aus diesem Verzeichnis, anstatt sie zu kopieren. Benutzer sehen Ihre Änderungen, wenn sie das nächste Mal eine Sitzung starten oder `/reload-plugins` ausführen, ohne einen Update-Schritt oder eine Versionsbump.

<h3 id="keep-plugin-files-out-of-git-lfs">
  Halten Sie Plugin-Dateien aus Git LFS fern
</h3>

Halten Sie die Dateien, die Ihre Plugins benötigen, aus [Git LFS](https://git-lfs.com) fern. Wenn Benutzer einen Marketplace hinzufügen, der in einem Git-Repository gehostet ist, oder ein Git-basiertes Plugin installieren, das er auflistet, klont Claude Code diesen Marketplace oder das Plugin-Repository auf ihre Maschine. Der Klon lädt niemals LFS-Inhalte herunter, daher kommen LFS-verfolgte Dateien als Zeiger-Dateien an.

<h3 id="share-files-within-a-marketplace-with-symlinks">
  Teilen Sie Dateien innerhalb eines Marketplace mit Symlinks
</h3>

Um Dateien zwischen Ihrem Plugin und anderen Teilen desselben Marketplace zu teilen, erstellen Sie symbolische Links in Ihrem Plugin-Verzeichnis. Wenn Claude Code das Plugin in seinen Cache kopiert, behandelt es jeden Symlink danach, wo das Ziel aufgelöst wird:

* **Innerhalb des eigenen Verzeichnisses des Plugins**: Der Symlink wird als relativer Symlink im Cache beibehalten, sodass er zur Laufzeit weiterhin zum kopierten Ziel aufgelöst wird.
* **Anderswo innerhalb desselben Marketplace**: Der Symlink wird dereferenziert. Der Inhalt des Ziels wird an seiner Stelle in den Cache kopiert. Dies ermöglicht es dem `skills/`-Verzeichnis eines Meta-Plugins, auf Skills zu verlinken, die von anderen Plugins im Marketplace definiert sind.
* **Außerhalb des Marketplace**: Der Symlink wird aus Sicherheitsgründen übersprungen.

Für Plugins, die von einem lokalen Pfad installiert sind, oder von einer [`command`-Quelle](/docs/de/plugins/marketplace-reference#command-plugin-source), deren `mode` der Standard `copy` ist, behält Claude Code nur Symlinks bei, die sich innerhalb des eigenen Verzeichnisses des Plugins auflösen, und überspringt alle anderen.

Der folgende Befehl erstellt einen Link von innerhalb eines Marketplace-Plugins zu einem gemeinsamen Skill, der von einem Sibling-Plugin definiert ist. Verwenden Sie unter Windows `mklink /D` aus einer erhöhten Eingabeaufforderung oder aktivieren Sie den Entwicklermodus:

```bash theme={null}
ln -s ../../shared-plugin/skills/foo ./skills/foo
```

<h2 id="distribute-through-organization-settings">
  Verteilen Sie über Organisationseinstellungen
</h2>

In einem Team- oder Enterprise-Plan können Sie den Marketplace auch über [**Organisationseinstellungen > Plugins & Skills**](https://claude.ai/admin-settings/skills?tab=inventory) auf claude.ai verteilen, anstatt ihn an einem Ort zu hosten, an dem Benutzer ihn selbst hinzufügen. Die Organisationssynchronisierung liest das Repository über die GitHub- oder GitLab-Verbindung Ihrer Organisation auf claude.ai, sodass die Git-Anmeldedaten Ihrer Benutzer nicht beteiligt sind.

Die Organisationssynchronisierung ist strenger in Bezug auf das Repository als `/plugin marketplace add`:

* **Marketplace-Repository**: Auf github.com und gitlab.com muss es privat oder intern sein
* **Plugin-Quellen**: Jede Plugin-Quelle muss vom Typ `github`, `url` oder `git-subdir` sein, oder ein [relativer Pfad](/docs/de/plugins/marketplace-reference#relative-path-plugin-source), der mit `./` beginnt
* **Top-Level `bin/`-Verzeichnis**: claude.ai lehnt ein Plugin ab, das eines hat, und synchronisiert den Rest des Marketplace. Die Fehlermeldung beginnt mit `Plugin contains a top-level bin/ directory`. Halten Sie ausführbare Dateien in einem anderen Verzeichnis, wie `scripts/`, und verweisen Sie auf sie als `${CLAUDE_PLUGIN_ROOT}/scripts/<name>` aus Ihren Hooks oder MCP-Server-Konfigurationen

Siehe [Verwalten Sie Plugins für Ihre Organisation](https://support.claude.com/en/articles/13837433) für den Admin-Workflow.

<h2 id="grant-access-to-a-private-marketplace">
  Gewähren Sie Zugriff auf einen privaten Marketplace
</h2>

Wenn ein Benutzer Ihren Marketplace hinzufügt, von ihm installiert oder ihn aktualisiert, führt Claude Code `git` auf seiner Maschine mit deaktivierten interaktiven Eingabeaufforderungen aus und verlässt sich auf alle Anmeldedaten, die diese Maschine bereits hält. Claude Code hat kein eigenes Git-Token, und `marketplace.json` hat kein Feld dafür.

Sie wählen, ob der Klon über SSH oder HTTPS ausgeführt wird, anhand der Form des Befehls zum Hinzufügen, den Sie Benutzern senden:

* **GitHub `owner/repo`**: Claude Code prüft `ssh -T git@github.com` und klont über SSH, wenn die Prüfung erfolgreich ist. Wenn die Prüfung fehlschlägt oder der SSH-Klon selbst fehlschlägt, klont es über HTTPS. Benutzer auf Maschinen ohne GitHub SSH-Schlüssel können `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1` setzen, um die Prüfung zu überspringen und über HTTPS zu klonen.
* **`git@host:path.git`**: SSH.
* **`https://example.com/repo.git`**: HTTPS.

Teilen Sie Benutzern mit, was jedes Protokoll auf ihrer Maschine benötigt:

* **SSH**: Der Schlüssel muss ohne Passphrase-Eingabeaufforderung funktionieren, zum Beispiel weil er in `ssh-agent` geladen ist. Der Host muss bereits in `known_hosts` sein.
* **HTTPS**: Claude Code lässt den Git-Credential-Helper des Benutzers aktiviert, verbietet ihm aber zu fragen. Eine Anmeldedaten, die der Helper bereits speichert, funktioniert; eine, die er fragen müsste, schlägt fehl. Auf GitHub speichert `gh auth login` gefolgt von `gh auth setup-git` eine.

Für einen GitHub Enterprise Server-Host benötigen Benutzer Git-Zugriff auf diesen Host von ihrer Maschine. Siehe [Plugin-Marketplaces auf GHES](/docs/de/github-enterprise-server#plugin-marketplaces-on-ghes) für das, was jede Claude Code-Oberfläche benötigt, um einen GHES-gehosteten Marketplace zu erreichen.

Wenn Sie stattdessen über **Organisationseinstellungen > Plugins & Skills** auf claude.ai verteilen, sind die Git-Anmeldedaten Ihrer Benutzer nicht beteiligt. Siehe [Verteilen Sie über Organisationseinstellungen](#distribute-through-organization-settings) für welche Plugin-Quellen dort privat sein können.

<h3 id="serve-users-who-have-no-git-host-account">
  Bedienen Sie Benutzer, die kein Git-Host-Konto haben
</h3>

Benutzer ohne Git-Host-Konto können einen Marketplace, den Sie als `marketplace.json`-URL oder aus einem gemeinsamen Verzeichnis bereitstellen, hinzufügen, aber sie können nur die Plugins installieren, deren Eintrag-Quellen sie auch erreichen können. Ein Eintrag, der auf ein privates `github`-Repository verweist, schlägt immer noch bei der Installation für sie fehl, da Claude Code ihn mit demselben nicht-interaktiven `git` abruft, das es für einen Git-gehosteten Marketplace verwendet.

Diese Eintrag-Quellen benötigen kein Git-Konto:

* **`archive`**: eine ZIP-Datei, die über HTTPS heruntergeladen wird. Benutzer benötigen weder `git` noch ein Konto, nur Netzwerkzugriff auf die URL. Erfordert Claude Code v2.1.224 oder später. Fixieren Sie jedes Archiv mit `sha256`, damit Claude Code einen geänderten Download ablehnt. Um Anmeldedaten mit dem Download zu senden, siehe [Authentifizieren Sie Archive-Downloads](#authenticate-archive-downloads).
* **Ein öffentliches Git-Repository**: Claude Code klont eine öffentliche `url`- oder `git-subdir`-Quelle über HTTPS ohne Anmeldedaten, wenn der Eintrag eine `https://`-URL angibt. Für eine `github`-Quelle oder eine `git-subdir`-Quelle, die als `owner/repo` geschrieben ist, setzen Benutzer ohne GitHub SSH-Schlüssel `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`.

Für ein Team in einem Netzwerk funktioniert auch ein `directory`-Marketplace auf einem gemeinsamen Dateisystem ohne Git-Konten. Benutzer benötigen nur Lesezugriff auf den Pfad.

<h3 id="what-background-auto-update-does-with-credentials">
  Was die Hintergrund-Autoupdate mit Anmeldedaten macht
</h3>

Hintergrund-Autoupdate ist Claude Codes unbeaufsichtigte Aktualisierung von Marketplaces und installierten Plugins nach dem Start einer Sitzung. Es ist für Ihren Marketplace ausgeschaltet, bis ein Benutzer oder Administrator es einschaltet, wie unter [Halten Sie Benutzer auf dem neuesten Stand](#keep-users-up-to-date) behandelt.

Wenn es für einen privaten Marketplace eingeschaltet ist, verwendet die Hintergrund-Prüfung auf neue Commits die konfigurierten Git-Credential-Helper des Benutzers und fragt niemals. Jede Art von Remote und Helper gibt ein anderes Ergebnis:

* **SSH-Remotes**: Ein Schlüssel, der in `ssh-agent` geladen ist, authentifiziert die Prüfung.
* **HTTPS-Remotes mit gespeicherten Anmeldedaten**: Ein Helper, der gespeicherte Anmeldedaten ohne Aufforderung bereitstellen kann, authentifiziert die Prüfung. Git Credential Manager, der macOS Keychain-Helper und `git-credential-store` funktionieren auf diese Weise, sobald sie Anmeldedaten für den Host halten.
* **HTTPS-Remotes mit einem Helper, der fragen muss**: Der Helper kann nicht im Hintergrund antworten. Die Aktualisierung schlägt stillschweigend fehl und der vorhandene Checkout bleibt an Ort und Stelle, sodass die Plugins des Benutzers aus dem letzten synchronisierten Zustand weiterhin funktionieren.

Nach der Prüfung führt Claude Code eines der folgenden aus:

* **Der Checkout ist auf dem neuesten Stand**: Claude Code lässt ihn, wie er ist.
* **Die Prüfung findet neue Commits oder schlägt fehl, weil sie den Remote nicht erreichen oder sich nicht authentifizieren kann**: Claude Code klont den Marketplace erneut und ersetzt den vorhandenen Checkout durch den neuen Klon. Wenn dieser Klon fehlschlägt, bleibt der vorhandene Checkout an Ort und Stelle. Der erneute Klon kann [bei großen Repositories nach 120 Sekunden abbrechen](/docs/de/plugins/troubleshooting#git-clone-timed-out-after-120s).

Um einen privaten Marketplace aktuell zu halten, kann ein Benutzer eines der folgenden tun:

* **Speichern Sie Anmeldedaten**: Melden Sie sich zuerst beim Credential-Helper an, damit er Anmeldedaten für den Host hält. Für GitHub führen Sie `gh auth login` aus, dann `gh auth setup-git`.
* **Halten Sie den Checkout bei Fehler**: Wenn der Benutzer `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1` setzt, behält Claude Code den vorhandenen Checkout bei, ohne den erneuten Klon zu versuchen, wenn die Hintergrund-Prüfung den Remote nicht erreichen oder sich nicht authentifizieren kann. Plugins funktionieren weiterhin aus dem letzten synchronisierten Zustand.

Wenn ein Benutzer `GITHUB_TOKEN` oder ein anderes Provider-Token in der Umgebung setzt, authentifiziert das allein nicht die Hintergrund-Prüfung. Ein Token wird durch einen Credential-Helper wirksam, wie den Helper der `gh`-CLI, der `GH_TOKEN` und `GITHUB_TOKEN` liest.

<h2 id="roll-out-to-a-whole-company">
  Rollen Sie für ein ganzes Unternehmen aus
</h2>

Das Ausrollen eines Plugins für ein Unternehmen beinhaltet Sie als Marketplace-Besitzer, einen Administrator, der verwaltete Einstellungen kontrolliert, und jede Person, die Claude Code verwendet. Sie können das Ausrollen ohne den Administrator durchführen, in welchem Fall jede Person den Marketplace hinzufügt und das Plugin selbst installiert.

| Wer                           | Was sie tun                                                                                                                                                                                                       | Wo es behandelt wird                                                                                                                                                                      |
| :---------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Sie, der Marketplace-Besitzer | Halten Sie den Katalog in einem Repository, das nur das Unternehmen lesen kann, senden Sie den Befehl zum Hinzufügen für Ihren Host, und sagen Sie, was jede Person auf ihrer Maschine benötigt                   | [Hosten Sie Ihren Marketplace](#host-your-marketplace) und [Gewähren Sie Zugriff auf einen privaten Marketplace](#grant-access-to-a-private-marketplace)                                  |
| Ein Administrator             | Registriert den Marketplace und schaltet seine Plugins für alle mit `extraKnownMarketplaces` und `enabledPlugins` in verwalteten Einstellungen ein, und setzt `autoUpdate` dort                                   | [Fordern Sie einen Marketplace und seine Plugins an](/docs/de/plugins/org#require-a-marketplace-and-its-plugins) und [Legen Sie die Update-Richtlinie fest](/docs/de/plugins/org#set-update-policy) |
| Jede Person                   | Benötigt Lesezugriff auf ein privates Git-Repository, mit Anmeldedaten, die bereits auf ihrer Maschine gespeichert sind. Ohne einen Administrator führen sie auch die Befehle zum Hinzufügen und Installieren aus | [Fügen Sie einen privaten Marketplace hinzu](/docs/de/plugins/install#add-a-private-marketplace)                                                                                               |

Für Personen, die kein Git-Host-Konto haben, behandelt jeder dieser Abschnitte eine Möglichkeit, sie zu erreichen:

* **Eintrag-Quellen, die kein Git-Konto benötigen**: [Bedienen Sie Benutzer, die kein Git-Host-Konto haben](#serve-users-who-have-no-git-host-account)
* **Ein vorausgefülltes Plugins-Verzeichnis**: [Seed-Container und CI](/docs/de/plugins/org#seed-containers-and-ci), das auch Benutzer bedient, die kein Git-Host-Konto haben
* **claude.ai-Organisationseinstellungen**: [Verteilen Sie über Organisationseinstellungen](#distribute-through-organization-settings), wo die Git-Anmeldedaten Ihrer Benutzer nicht beteiligt sind

<h2 id="keep-users-up-to-date">
  Halten Sie Benutzer auf dem neuesten Stand
</h2>

Ihre Änderungen erreichen Benutzer durch Hintergrund-Autoupdate, sobald es für Ihren Marketplace eingeschaltet ist, oder wenn Benutzer das Plugin selbst aktualisieren. In beiden Fällen erhält ein Benutzer eine neue Kopie eines Plugins nur, wenn sich seine berechnete Version ändert, wie unter [Geben Sie eine neue Version frei](#release-a-new-version) beschrieben.

<h3 id="turn-on-auto-update">
  Schalten Sie Autoupdate ein
</h3>

Hintergrund-Autoupdate ist für Ihren Marketplace standardmäßig ausgeschaltet, und `marketplace.json` hat kein Feld, um es einzuschalten. Ein Benutzer oder ein Administrator schaltet es ein:

* **Teilen Sie Benutzern mit, dass sie es einschalten sollen**: Jeder Benutzer geht zu **Marketplaces** in `/plugin`, wählt Ihren Marketplace aus und wählt **Enable auto-update**.
* **Bitten Sie einen Administrator, es zu setzen**: Wenn ein Administrator `"autoUpdate": true` auf dem `extraKnownMarketplaces`-Eintrag Ihres Marketplace in verwalteten Einstellungen setzt, ist es für alle eingeschaltet, die diese Einstellungen erhalten. Siehe [Legen Sie die Update-Richtlinie fest](/docs/de/plugins/org#set-update-policy).

Ohne Autoupdate erhalten Benutzer Ihre Änderungen, wenn sie `/plugin marketplace update <name>` in einer Sitzung oder `claude plugin update <plugin>@<name>` in der Shell ausführen.

Für das, was Benutzer sehen, wenn eine Aktualisierung sie erreicht, siehe [Wenn Autoupdate ausgeführt wird](/docs/de/plugins/loading#when-auto-update-runs).

<h3 id="release-a-new-version">
  Geben Sie eine neue Version frei
</h3>

Um eine neue Version für Benutzer freizugeben, ändern Sie die `version` des Plugins. Benutzer erhalten eine neue Kopie nur, wenn sich die berechnete Version des Plugins von der unterscheidet, die sie haben. Diese Version kommt zuerst aus `plugin.json`, dann aus dem Marketplace-Eintrag, pro [Versionen und Updates](/docs/de/plugins/loading#versions-and-updates).

Ein Plugin, das Benutzer [an Ort und Stelle laden](/docs/de/plugins/loading#find-plugins-on-disk) von einem Marketplace, den sie als lokales Verzeichnis hinzugefügt haben, wird nicht durch `version` kontrolliert. Es lädt Ihre aktuellen Dateien bei jedem Sitzungsstart, unabhängig davon, was sein Versions-String sagt.

Für jede Installation außer einem In-Place-Load oder einem von einer `command`-Quelle erhöhen Sie entweder `version` bei jeder Veröffentlichung oder lassen Sie es weg:

* **Erhöhen Sie `version` bei jeder Veröffentlichung**: Benutzer bleiben auf ihrer zwischengespeicherten Kopie, bis sich der String ändert. Wenn Sie `"version": "1.0.0"` setzen und neue Commits hochladen, ohne es zu ändern, erhalten Benutzer sie nicht.
* **Lassen Sie `version` weg**: Benutzer verfolgen stattdessen Ihre Commits. Lassen Sie `version` sowohl aus `plugin.json` als auch aus dem Marketplace-Eintrag weg.

Setzen Sie `version` nicht in sowohl `plugin.json` als auch dem Marketplace-Eintrag. Wenn Sie das tun, verwendet Claude Code den `plugin.json`-Wert ohne Warnung, und `claude plugin validate` meldet die Nichtübereinstimmung als `Entry declares version "<a>" but <path>/plugin.json says "<b>"`.

<h3 id="hold-users-on-one-version">
  Halten Sie Benutzer auf einer Version
</h3>

Ein Marketplace bedient eine Version jedes Plugins auf einmal, daher halten Sie Benutzer auf einer Version, indem Sie wählen, worauf jeder Eintrag verweist:

* **`ref` und `sha` auf dem Plugin-Eintrag**: `ref` benennt einen Branch oder Tag und `sha` benennt einen Commit für eine `github`-, `url`- oder `git-subdir`-Quelle. Siehe [Plugin-Quellen](/docs/de/plugins/marketplace-reference#plugin-sources).
* **`#<ref>` auf dem Befehl zum Hinzufügen**: Benutzer, die `your-org/your-marketplace#stable` hinzufügen, erhalten diesen Branch oder Tag des Katalogs. Für zwei Release-Linien auf einmal siehe [Führen Sie Release-Kanäle aus](#run-release-channels).
* **`<plugin>--v<version>`-Tags**: Ein Versionsbereich einer Abhängigkeit wird gegen diese Tags aufgelöst. Siehe [Geben Sie ein Plugin frei, auf das andere angewiesen sind](/docs/de/plugins/dependencies#tag-plugin-releases-for-version-resolution).

[Geben Sie eine neue Version frei](#release-a-new-version) sagt, wann ein geänderter Eintrag Benutzer erreicht.

<h3 id="change-the-command-of-a-command-source">
  Ändern Sie den Befehl einer Befehl-Quelle
</h3>

Wenn Sie den `command` einer [`command`-Quelle](/docs/de/plugins/marketplace-reference#command-plugin-source) ändern oder seinen `mode` wechseln, muss jeder Benutzer den neuen Befehl akzeptieren, bevor Claude Code ihn ausführt. Claude Code führt nur den genauen Befehl aus, den ein Benutzer akzeptiert hat, als er das Plugin installiert oder zuletzt aktualisiert hat.

Nachdem die Kopie des Marketplace eines Benutzers die Änderung aufgegriffen hat, sieht dieser Benutzer eines der folgenden:

* **Keine weiteren Hintergrund-Läufe**: Der [einmalige Lauf pro Sitzung](/docs/de/plugins/loading#when-a-command-source-re-runs) des Befehls stoppt für diesen Benutzer, sodass die neue Ausgabe des Tools ihn nicht erreicht.
* **Ein Eintrag in der Registerkarte `/plugin` Errors**: Der Eintrag zeigt den neuen Befehl und den `claude plugin update`-Befehl zum Ausführen.

Teilen Sie Benutzern mit, dass sie den `claude plugin update`-Befehl ausführen sollen, den dieser Eintrag zeigt, in einem Terminal. Claude Code zeigt ihnen den neuen Befehl und bittet sie, ihn zu akzeptieren.

<h2 id="run-release-channels">
  Führen Sie Release-Kanäle aus
</h2>

Um stabile und Early-Access-Tracks anzubieten, hosten Sie zwei Marketplaces, deren Einträge auf verschiedene Refs desselben Plugins verweisen, und lassen Sie jeden Benutzer den hinzufügen, den er möchte. Claude Code hat kein Release-Kanal-Konzept, und ein Marketplace bedient eine Version jedes Plugins auf einmal.

Geben Sie den zwei `marketplace.json`-Dateien unterschiedliche `name`-Werte. Claude Code identifiziert einen Marketplace anhand seines `name`, daher kann ein Benutzer nicht zwei Marketplaces mit demselben Namen registriert haben.

Mit diesen zwei Katalogen installieren Benutzer, die `stable-tools` hinzufügen, `code-formatter` aus dem `stable`-Branch, und Benutzer, die `latest-tools` hinzufügen, installieren ihn aus `latest`:

```json theme={null}
{
  "name": "stable-tools",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": { "source": "github", "repo": "your-org/code-formatter", "ref": "stable" } }
  ]
}
```

```json theme={null}
{
  "name": "latest-tools",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": { "source": "github", "repo": "your-org/code-formatter", "ref": "latest" } }
  ]
}
```

Geben Sie den zwei Refs unterschiedliche `plugin.json`-Versionen, oder lassen Sie `version` weg, damit der Commit SHA sie unterscheidet. Updates werden durch Vergleich von Versionen erkannt, daher lässt ein Ref, das sich ohne Versionsänderung bewegt, Benutzer auf der zwischengespeicherten Kopie.

Um die Kanäle Benutzergruppen zuzuweisen, anstatt Benutzer wählen zu lassen, gibt ein Administrator jeder Gruppe den passenden `extraKnownMarketplaces`-Eintrag, wie unter [Legen Sie die Update-Richtlinie fest](/docs/de/plugins/org#set-update-policy) beschrieben.

<h2 id="rename-or-remove-a-plugin">
  Benennen Sie ein Plugin um oder entfernen Sie es
</h2>

Der `name` eines Plugins ist sein Identifier. Benutzer verweisen darauf in den Einstellungsschlüsseln `enabledPlugins` und `pluginConfigs` und in `/plugin install`, daher bricht das Ändern davon jede vorhandene Installation.

Um das Label zu ändern, das Benutzer in `/plugin` sehen, ohne etwas zu unterbrechen, setzen Sie `displayName` in `plugin.json` und halten Sie `name` unverändert.

<h3 id="migrate-users-with-a-renames-map">
  Migrieren Sie Benutzer mit einer Umbenennungs-Map
</h3>

Wenn Sie einen `name` ändern müssen, fügen Sie eine Top-Level-`renames`-Map zu `marketplace.json` hinzu, damit Claude Code vorhandene Benutzer migriert, anstatt [`Plugin "<name>" not found in marketplace`](/docs/de/plugins/troubleshooting#plugin-not-found-in-marketplace) zu melden. Tun Sie dasselbe, wenn Sie einen Eintrag aus `plugins` entfernen. Die automatische Migration erfordert Claude Code v2.1.193 oder später.

Ordnen Sie jeden früheren Namen seinem aktuellen Namen zu, oder zu `null`, wenn das Plugin weg ist. Dieser Marketplace benennt `formatter` in `code-formatter` um und verzeichnet, dass `legacy-linter` entfernt wurde:

```json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": "./plugins/code-formatter" }
  ],
  "renames": {
    "formatter": "code-formatter",
    "legacy-linter": null
  }
}
```

Nachdem Sie hochgeladen haben, sieht ein Benutzer, der den alten Namen noch aktiviert hat, eines dieser Ergebnisse:

* **Umbenannter Eintrag**: Das Plugin lädt unter seinem neuen Namen. `claude plugin list` und die Details des Plugins unter `/plugin` zeigen `Renamed to "code-formatter" in the "your-marketplace" marketplace` einmal, und Claude Code schreibt den alten Schlüssel in den neuen in `enabledPlugins` und `pluginConfigs` in den Benutzer-, Projekt- und lokalen Einstellungsbereichen um.
* **`null`-Eintrag**: Der alte Schlüssel wird aus diesen Bereichen gelöscht und der Benutzer sieht `Removed from the "your-marketplace" marketplace`.
* **In verwalteten Einstellungen aktiviert**: Das Plugin lädt immer noch unter seinem neuen Namen, aber Claude Code kann verwaltete Einstellungen nicht umschreiben, daher wiederholt sich die Benachrichtigung, bis ein Administrator `enabledPlugins` dort aktualisiert.

Für einen Marketplace, den Benutzer aus einem Git-Repository oder einer URL hinzugefügt haben, meldet ein umbenanntes Plugin [`Plugin "<name>" not cached at <path>`](/docs/de/plugins/troubleshooting#plugin-not-cached-at), bis der Benutzer `/plugin install code-formatter@your-marketplace` einmal in einer Sitzung ausführt.

Behandeln Sie `renames` als Nur-Anhängen-Verlauf. Halten Sie alte Einträge, nachdem alle migriert sind. Wenn Sie erneut umbenennen, fügen Sie einen zweiten Eintrag hinzu, anstatt den ersten zu bearbeiten, da Claude Code der Kette vom ältesten Namen folgt.

Führen Sie in Ihrer Shell `claude plugin validate .` aus, nachdem Sie die Map bearbeitet haben. Es lehnt eine Kette ab, die zirkuliert oder die irgendwo anders als `null` oder einem Namen in `plugins` endet, mit `renames.<name>: chain does not resolve`.

<h3 id="uninstall-removed-plugins-from-users’-machines">
  Deinstallieren Sie entfernte Plugins von den Maschinen der Benutzer
</h3>

Um ein entferntes Plugin von den Maschinen der Benutzer zu deinstallieren, anstatt eine Kopie zu hinterlassen, setzen Sie `"forceRemoveDeletedPlugins": true` auf der Top-Level von `marketplace.json`. Ohne das Feld bleibt ein entferntes Plugin installiert und meldet `Plugin "<name>" not found in marketplace`, wenn eine Sitzung es lädt. Mit ihm führt Claude Code bei jedem Sitzungsstart folgendes aus:

1. Vergleicht, was Benutzer von Ihrem Marketplace installiert haben, gegen die Einträge und die `renames`-Map, und behandelt jedes Plugin, das weder aufgelistet noch umbenannt ist, als entfernt.
2. Deinstalliert jedes entfernte Plugin aus den Benutzer-, Projekt- und lokalen Bereichen. Plugins, die nur verwaltete Einstellungen installiert haben, bleiben an Ort und Stelle.
3. Listet jedes entfernte Plugin unter einer **Flagged**-Überschrift in `/plugin` mit dem Status `Removed from marketplace` auf.

<h2 id="authenticate-archive-downloads">
  Authentifizieren Sie Archive-Downloads
</h2>

Um einen [`archive`](/docs/de/plugins/marketplace-reference#archive-plugin-source)-Download zu authentifizieren, wie einen Download aus einer privaten Registry, setzen Sie die HTTP-Header, die Claude Code damit sendet. Sie können `headers` an einem dieser Orte setzen:

* **Die `url`-Quelle des Marketplace**: Die `url`-Quelle, von der Sie den Marketplace registriert haben, wie ein [`extraKnownMarketplaces`](/docs/de/settings-reference#extraknownmarketplaces)-Eintrag.
* **Der Eintrag des Plugins**: Auf Claude Code v2.1.238 oder später können Sie es stattdessen auf dem `marketplace.json`-Eintrag des Plugins setzen, neben `source`.

Setzen Sie an beiden Orten einen `headersHelper`-Befehl anstelle von `headers`, wenn der Wert kurzlebig ist, wie ein Token, das Ihre Registry auf Anfrage generiert. Claude Code führt den Befehl aus und sendet das JSON-Objekt, das er druckt, als Header dieses Ortes. Erfordert Claude Code v2.1.238 oder später.

Die [Marketplace-Referenz](/docs/de/plugins/marketplace-reference#plugin-entries) listet die `headers`- und `headersHelper`-Eintrag-Felder auf.

Der Ort, den Sie wählen, entscheidet, welche Downloads die Header erhalten und wann Claude Code den Befehl ausführt:

| Ort                      | Downloads, die die Header erhalten                                                                  | Wann Claude Code einen dort gesetzten `headersHelper` ausführt                                                                                                                     |
| :----------------------- | :-------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Marketplace `url`-Quelle | Archive-Downloads auf dem Ursprung der Marketplace-URL, was dasselbe Schema, Host und Port bedeutet | Vor jedem Abruf der `marketplace.json` des Marketplace und vor jedem Archive-Download auf diesem Ursprung. Claude Code verwendet die Ausgabe eines Laufs bis zu 60 Sekunden wieder |
| Plugin-Eintrag           | Nur der Download dieses Eintrags                                                                    | Nur wenn ein Benutzer dieses eine Plugin selbst installiert oder aktualisiert und [den Befehl akzeptiert](#how-users-accept-a-headershelper-command)                               |

Wo beide Orte einen Header desselben Namens setzen, sendet Claude Code den Wert des Eintrags. Innerhalb eines Ortes überschreibt ein Header, den der Befehl druckt, einen Header desselben Namens, der in `headers` aufgelistet ist.

<h3 id="add-a-headershelper-to-a-plugin-entry">
  Fügen Sie einen headersHelper zu einem Plugin-Eintrag hinzu
</h3>

Dieser Eintrag setzt `headersHelper` neben `source`. Er setzt auch [`"strict": false`](/docs/de/plugins/marketplace-reference#strict-mode), das Claude Code von einem `marketplace.json`-Eintrag erfordert, der `headersHelper` setzt:

```json theme={null}
{
  "name": "my-plugin",
  "description": "Formatting commands for internal services",
  "strict": false,
  "source": {
    "source": "archive",
    "url": "https://registry.example.com/plugins/my-plugin-2.1.0.zip"
  },
  "headersHelper": "/opt/bin/mint-registry-token.sh"
}
```

Um den Eintrag zu überprüfen, führen Sie `claude plugin install my-plugin@your-marketplace` in Ihrer Shell aus. Claude Code zeigt Ihnen den Befehl und die Archive-URL und lädt die ZIP herunter, nachdem Sie akzeptieren.

<h3 id="write-the-headershelper-command">
  Schreiben Sie den headersHelper-Befehl
</h3>

Ob Sie `headersHelper` auf einer `url`-Quelle eines Marketplace oder auf einem Plugin-Eintrag setzen, schreiben Sie den Befehl, um diese Anforderungen zu erfüllen:

* **Befehlstext**: Höchstens 500 Zeichen druckbares ASCII, ohne einen Lauf von vier oder mehr Leerzeichen.
* **Ausgabe**: Drucken Sie ein JSON-Objekt von Header-Namen und String-Werten auf stdout, dann beenden Sie 0 innerhalb von 10 Sekunden.
* **Shell und Arbeitsverzeichnis**: Claude Code führt den Befehl durch `sh` oder durch `cmd.exe` unter Windows aus. Das Arbeitsverzeichnis ist das Konfigurationsverzeichnis, das `~/.claude` oder [`CLAUDE_CONFIG_DIR`](/docs/de/env-vars#variables) ist. Geben Sie einen absoluten Pfad oder einen Befehl auf `PATH` an, da ein relativer Pfad gegen dieses Verzeichnis aufgelöst wird, nicht gegen das Projekt des Benutzers.
* **Variablen, die Claude Code entfernt**: Wenn der Befehl in einem `marketplace.json`-Eintrag oder in einer Projekt-`.claude/settings.json` oder `.claude/settings.local.json` gesetzt ist, entfernt Claude Code aus der Umgebung jede Variable, deren Name wie eine Anmeldedaten aussieht, nach der [gleichen Regel, die es auf einen MCP `headersHelper` anwendet](/docs/de/mcp#which-variables-a-helper-can-read). `ANTHROPIC_API_KEY` und `MY_REGISTRY_TOKEN` werden beide entfernt, daher lesen Sie die Anmeldedaten aus einer Datei oder einem Credential-Store. Diese Entfernung gilt nicht für einen Befehl, der in Benutzereinstellungen, einer `--settings`-Datei oder verwalteten Einstellungen gesetzt ist.
* **Variablen, die Claude Code setzt**: `CLAUDE_CODE_MARKETPLACE_URL` und `CLAUDE_CODE_MARKETPLACE_NAME` für den Befehl einer `url`-Quelle, und `CLAUDE_CODE_PLUGIN_NAME` und `CLAUDE_CODE_PLUGIN_ARCHIVE_URL` für den Befehl eines Eintrags. `CLAUDE_CODE_MARKETPLACE_NAME` ist beim ersten Abruf nach dem Hinzufügen eines Marketplace durch URL nicht gesetzt, da dieser Abruf der ist, der den Namen liefert.

Ein Befehl, der ein Bearer-Token prägt, druckt ein Objekt wie dieses:

```json theme={null}
{"Authorization": "Bearer eyJhbGciOiJSUzI1NiJ9"}
```

<h3 id="when-claude-code-skips-a-headershelper-command-or-drops-its-output">
  Wenn Claude Code einen headersHelper-Befehl überspringt oder seine Ausgabe ablegt
</h3>

Ein `headersHelper`-Befehl wird nicht ausgeführt, oder Header aus `headers` oder aus der Ausgabe des Befehls werden abgelegt, wenn eines der folgenden zutrifft:

* **Befehl schlägt fehl**: Wenn der Befehl mit Nicht-Null beendet wird, über 10 Sekunden läuft oder etwas anderes als ein JSON-Objekt von String-Werten druckt, findet der Abruf oder Download, für den der Befehl ausgeführt wurde, nicht statt.
* **Marketplace-URL beginnt nicht mit `https://`**: Der Befehl dieser `url`-Quelle wird nicht ausgeführt, und Anfragen tragen nur die in ihrem `headers`-Feld aufgelisteten Header.
* **Umleitung verlässt den Ursprung**: Wenn ein Download von der Archive-URL-Ursprung umgeleitet wird, trägt die umgeleitete Anfrage keine `headers`-Werte oder Befehlsausgabe von der Marketplace `url`-Quelle oder dem Plugin-Eintrag.
* **Eintrag setzt einen Routing- oder Identity-Header**: Claude Code legt Request-Routing- und Client-Identity-Namen wie `Host`, `Cookie` und `X-Forwarded-*` aus einem Eintrag `headers` und Befehlsausgabe ab und behält Authentifizierungsnamen wie `Authorization`. Jeder `marketplace.json`-Eintrag wird auf diese Weise gefiltert. Für einen Inline-Plugin-Eintrag in Einstellungen siehe [`extraKnownMarketplaces`](/docs/de/settings-reference#extraknownmarketplaces).
* **Befehl in den Einstellungen eines `--add-dir`-Verzeichnisses gesetzt**: Der Befehl wird ignoriert, auf einer `url`-Quelle und auf einem [Inline-Plugin-Eintrag](/docs/de/settings-reference#extraknownmarketplaces) gleichermaßen, und nur die `headers` dieser Datei werden gesendet.
* **Verwaltete Einstellungen blockieren den Befehl**: Das Setzen von [`disableCommandPluginSources`](/docs/de/settings-reference#disablecommandpluginsources) auf `true` blockiert `headersHelper`-Befehle, und [`allowManagedHooksOnly`](/docs/de/settings-reference#allowmanagedhooksonly) blockiert sie auch, es sei denn, `disableCommandPluginSources` ist explizit `false`. Unter beiden Blöcken führt Claude Code den Befehl immer noch für einen Marketplace aus, den verwaltete Einstellungen selbst deklarieren.

<h3 id="how-users-accept-a-headershelper-command">
  Wie Benutzer einen headersHelper-Befehl akzeptieren
</h3>

Ein Benutzer akzeptiert den Befehl eines Plugin-Eintrags jedes Mal, wenn er dieses eine Plugin selbst installiert oder aktualisiert. Sie tun das aus der eigenen Ansicht des Plugins in `/plugin` oder mit `claude plugin install` oder `claude plugin update`. Claude Code zeigt den Befehl und die Archive-URL und führt den Befehl nur aus, nachdem der Benutzer akzeptiert.

In einer nicht-interaktiven Shell übergeben Sie [`--yes`](/docs/de/plugins/cli-reference#plugin-install), um den Befehl zu akzeptieren. Um nur den Befehl zu akzeptieren, den ein vorheriger `--json`-Lauf angezeigt hat, übergeben Sie [`--accept-command`](/docs/de/plugins/cli-reference#plugin-install) mit dem `sha256`, den der Lauf gemeldet hat.

Claude Code führt nur den Befehl aus, den es zeigte, für die Archive-URL, die es zeigte. Wenn sich der Befehl oder die Archive-URL des Eintrags dazwischen geändert haben, weigert sich Claude Code zu installieren oder zu aktualisieren. Eine Änderung nur in der Abfragezeichenfolge zählt nicht.

<h3 id="installs-and-updates-that-refuse-the-command-instead-of-asking">
  Installationen und Updates, die einen Befehl ablehnen, anstatt zu fragen
</h3>

Bei jeder Operation außer einer Einzelplugin-Installation oder -Aktualisierung führt Claude Code den Befehl eines Eintrags nicht aus und lädt sein Archiv nicht herunter. Das Plugin bleibt auf seiner installierten Version oder bleibt deinstalliert, und der Benutzer sieht eines dieser Ergebnisse:

* **Installation mehrerer Plugins auf einmal, von einer Plugin-Suggestion oder als Abhängigkeit eines anderen Plugins**: Claude Code weigert das Plugin, das den Befehl hat, und leitet den Benutzer zur eigenen Ansicht dieses Plugins in `/plugin`. Die anderen Plugins in einer Massen-Installation installieren immer noch. Ein Plugin, das vom abgelehnten Plugin abhängt, schlägt bei der Installation fehl, bis der Benutzer das abgelehnte Plugin selbst installiert.
* **Hintergrund-Autoupdate oder Sitzungsstart für ein Plugin, dessen Archiv nie heruntergeladen wurde**: Claude Code listet das Plugin in der Registerkarte `/plugin` Errors auf, damit der Benutzer weiß, dass er es selbst installieren oder aktualisieren muss.

<h3 id="when-a-marketplace-url-sources-command-runs">
  Wenn der Befehl einer Marketplace `url`-Quelle ausgeführt wird
</h3>

Sie deklarieren den `headersHelper` einer Marketplace `url`-Quelle in einer Einstellungsdatei, wie einem [`extraKnownMarketplaces`](/docs/de/settings-reference#extraknownmarketplaces)-Eintrag, anstatt im Katalog, den der Marketplace veröffentlicht. Claude Code fragt daher nicht den Benutzer, ihn bei jeder Installation oder Aktualisierung zu akzeptieren. Stattdessen entscheidet die Einstellungsdatei, die ihn deklariert, wann Claude Code ihn ausführt:

| Einstellungsdatei                                                                                      | Wann Claude Code den Befehl ausführt                                                                                                                                                                                                                                              |
| :----------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Benutzereinstellungen, eine `--settings`-Datei oder eine verwaltete Einstellungsdatei auf der Maschine | Ohne zu fragen, einschließlich während einer Hintergrund-Marketplace-Aktualisierung                                                                                                                                                                                               |
| Eine Projekt-`.claude/settings.json` oder `.claude/settings.local.json`                                | Nur nachdem der Benutzer den [Workspace-Trust-Dialog](/docs/de/permissions#what-runs-before-you-trust-a-folder) für diesen Ordner selbst akzeptiert. Eine `-p`- oder SDK-Sitzung zählt nicht als Akzeptanz, und auch nicht das Vertrauen, das einem übergeordneten Ordner gewährt wird |
| Server-verwaltete Einstellungen                                                                        | In einer interaktiven Sitzung, nur nachdem der Benutzer die bereitgestellten Einstellungen im [Sicherheitsgenehmigungsdialog](/docs/de/server-managed-settings#security-approval-dialogs) genehmigt                                                                                    |

Für einen [Inline-Plugin-Eintrag](/docs/de/settings-reference#extraknownmarketplaces) in einer dieser Dateien erfordert Claude Code das gleiche Ordner-Vertrauen oder die gleiche Einstellungsgenehmigung wie für einen Marketplace-Level-Befehl in dieser Datei, und der Benutzer akzeptiert auch den Befehl des Eintrags bei jeder Installation oder Aktualisierung.

<h2 id="depend-on-and-recommend-other-plugins">
  Hängen Sie von anderen Plugins ab und empfehlen Sie sie
</h2>

Ein Eintrag kann Abhängigkeiten von anderen Plugins deklarieren.

* **Versionsbereiche**: Eine Abhängigkeit kann einen Semver-Bereich tragen.
* **Cross-Marketplace-Abhängigkeiten**: Eine Abhängigkeit von einem anderen Marketplace installiert nur, wenn Ihr Marketplace diesen Marketplace in `allowCrossMarketplaceDependenciesOn` auflistet.

Für Versionsbereiche, die `<plugin>--v<version>`-Git-Tag-Konvention, gegen die sie aufgelöst werden, und Cross-Marketplace-Vertrauen, siehe [Plugin-Abhängigkeiten](/docs/de/plugins/dependencies).

Um Claude Code zu haben, ein Plugin zu suggerieren, wenn ein Projekt es passt, fügen Sie einen `relevance`-Block zum Eintrag mit den Signalen hinzu, die das Projekt identifizieren. Benutzer sehen Suggestionen aus Ihrem Marketplace nur, wenn ein Administrator ihn in `pluginSuggestionMarketplaces` auflistet. Für die Signale und den Aktivierungsschritt siehe [Plugin-Relevanz](/docs/de/plugins/relevance).

<h2 id="work-around-what-a-marketplace-can’t-do">
  Arbeiten Sie um, was ein Marketplace nicht kann
</h2>

Einige Dinge, die Besitzer anfordern, haben kein Feld in `marketplace.json`. Hier ist die nächste Option für jede:

* **Beschränken Sie, was sonst Benutzer installieren**: Die Marketplace-Zulassungsliste ist eine verwaltete Einstellung, `strictKnownMarketplaces`. Siehe [Beschränken Sie, was Benutzer installieren können](/docs/de/plugins/org#restrict-what-users-can-install).
* **Installieren oder aktivieren Sie ein Plugin, ohne dass der Benutzer fragt**: Kein Eintrag-Feld installiert ein Plugin. Verwaltete `enabledPlugins` tut das für eine Flotte; siehe [Vorinstallieren und erfordern Sie Plugins](/docs/de/plugins/org#pre-install-and-require-plugins).
* **Zeigen Sie verschiedene Einträge verschiedenen Benutzern**: Einträge tragen kein Audience-Feld, und jeder Benutzer, der den Marketplace hinzufügt, sieht den gesamten Katalog. Hosten Sie separate Marketplaces für separate Audiences.
* **Markieren Sie ein Plugin als veraltet**: Es gibt keinen Veraltungszustand. Die Option ist, den Eintrag zu entfernen, seinen Namen in `renames` auf `null` zu ordnen und optional `forceRemoveDeletedPlugins` zu setzen.
* **Schalten Sie Autoupdate für Ihre Benutzer ein**: Jeder Benutzer schaltet es unter **Marketplaces** in `/plugin` ein, oder ein Administrator setzt `autoUpdate` in verwalteten Einstellungen. Siehe [Schalten Sie Autoupdate ein](#turn-on-auto-update).
* **Tragen Sie Git-Anmeldedaten**: Kein Marketplace-Feld hält ein Git-Token. Der Zugriff auf einen Git-gehosteten Marketplace oder ein Plugin folgt dem Git-Setup des Benutzers, pro [Gewähren Sie Zugriff auf einen privaten Marketplace](#grant-access-to-a-private-marketplace). Für `archive`-Quellen kann ein Eintrag stattdessen [`headers` oder `headersHelper`](#authenticate-archive-downloads) setzen.

<h2 id="next-steps">
  Nächste Schritte
</h2>

* [Marketplace-Referenz](/docs/de/plugins/marketplace-reference): `marketplace.json`-Felder, Quelltypen und Validierungsmeldungen
* [Verwalten Sie Plugins für Ihre Organisation](/docs/de/plugins/org): Fordern Sie an, beschränken Sie oder seeden Sie Ihren Marketplace in den Maschinen Ihrer Organisation
* [Plugin-Abhängigkeiten](/docs/de/plugins/dependencies): Taggen Sie Releases, damit Plugins, die von Ihrem abhängen, Versionen auflösen können
* [Beheben Sie Probleme mit Plugins](/docs/de/plugins/troubleshooting): Die Fehler, die Ihre Benutzer beim Hinzufügen oder Aktualisieren von Ihrem Marketplace sehen
