> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code in einem Monorepo oder großen Codebase einrichten

> Konfigurieren Sie Claude Code für Monorepos und große Single-Tree-Codebases mit verschachtelten CLAUDE.md-Dateien, Sparse Worktrees, Code Intelligence und Skills pro Paket, damit Claude sich auf den Code konzentriert, an dem Sie arbeiten.

Eine große Codebase kann ein Repository mit Millionen von Zeilen oder ein Monorepo mit vielen Paketen sein. Claude Code funktioniert in jeder Größe, aber mit wachsender Codebase können die für kleinere Projekte optimierten Standardeinstellungen das Kontextfenster mit Anweisungen und Dateileseoperationen füllen, die nicht mit der Aufgabe zusammenhängen, was Token kostet und Claudes Leistung verschlechtert.

Dieser Leitfaden zeigt einzelnen Entwicklern und Engineering-Teams, wie sie Claude auf den Teil der Codebase beschränken, den eine Aufgabe berührt. Jeder Abschnitt vermerkt, ob eine Einstellung persönlich auf Ihrem Computer ist oder im Repository committed ist.

<h2 id="what-this-guide-covers">
  Was dieser Leitfaden abdeckt
</h2>

Die [Tabelle unten](#settings-on-this-page) listet jede Einstellung und ihre Funktion auf. Der [Dateibaum danach](#the-example-monorepo) ist das Beispiel-Monorepo, auf das sich jedes Codebeispiel auf dieser Seite bezieht.

<h3 id="settings-on-this-page">
  Einstellungen auf dieser Seite
</h3>

Jede Einstellung unten ist unabhängig. Sie überlagern sich, anstatt sich gegenseitig zu ersetzen, daher wenden Sie diejenigen an, die zu Ihrem Repository passen. [Wählen Sie, wo Claude gestartet werden soll](#choose-where-to-start-claude) bestimmt, wo Ihre Einstellungsdateien leben, daher lesen Sie dies zuerst. [Alles zusammensetzen](#put-it-together) zeigt alle kombiniert.

| Ich möchte                                                                                                      | Verwenden                                                                                       |
| :-------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------- |
| Nur die Konventionen für den Code laden, den Sie berühren, anstatt eine Root-Datei, die jedes Subsystem abdeckt | Per-Verzeichnis [CLAUDE.md-Dateien](#layer-claude-md-files-by-directory)                        |
| CLAUDE.md-Dateien für Pakete ausschließen, in denen Sie nie arbeiten                                            | [`claudeMdExcludes`](#exclude-irrelevant-claude-md-files)                                       |
| Claude daran hindern, Build-Ausgabe, generierten Code und Vendor-Abhängigkeiten zu öffnen                       | [`Read` Deny-Regeln](#block-reads-of-generated-and-vendored-code) in `permissions.deny`         |
| Eine Symbol-Definition oder Aufrufer über den Language Server finden, anstatt Dateien zu scannen                | Ein [Code-Intelligence-Plugin](#reduce-file-reads-with-code-intelligence)                       |
| Nur die Verzeichnisse auschecken, die eine Aufgabe benötigt, wenn Claude einen Worktree erstellt                | [`worktree.sparsePaths`](#check-out-only-the-directories-you-need)                              |
| Ein Schwester-Paket oder ein anderes Repository aus derselben Sitzung lesen und bearbeiten                      | [`--add-dir`](#grant-access-across-packages-or-repositories) oder `additionalDirectories`       |
| Claude Verfahren geben, die spezifisch für einen Bereich sind und nur bei Bedarf geladen werden                 | Per-Verzeichnis [Skills](#add-per-directory-skills)                                             |
| Viele Per-Verzeichnis-CLAUDE.md-Dateien durch einen Satz von Konventionen ersetzen, den jeder installiert       | Ein [Plugin](#centralize-conventions-when-layering-stops-scaling) in einem internen Marketplace |

<Tip>
  Für Workflow-Techniken, die den Kontext in jedem Repository klein halten, wie z. B. [Exploration in einem Subagenten ausführen](/docs/de/best-practices#use-subagents-for-investigation), damit Dateileseoperationen aus der Hauptkonversation herausbleiben, siehe [Best Practices für Claude Code](/docs/de/best-practices). Um eine Baseline-Konfiguration für jeden Entwickler in Ihrer Organisation bereitzustellen, siehe [Claude Code für Ihre Organisation einrichten](/docs/de/admin-setup).
</Tip>

<h3 id="the-example-monorepo">
  Das Beispiel-Monorepo
</h3>

Die Beispiele auf dieser Seite beziehen sich auf ein Monorepo mit drei Paketen. Die gleichen Muster funktionieren in einer großen Single-Tree-Codebase: Wo ein Beispiel `packages/api/` verwendet, ersetzen Sie Ihr eigenes Subsystem-Verzeichnis wie `src/backend/` oder `lib/core/`.

```text theme={null}
monorepo/
  CLAUDE.md                     # Root-Anweisungen
  packages/
    api/
      CLAUDE.md                 # API-spezifische Anweisungen
      .claude/skills/
      src/
    web/
      CLAUDE.md                 # Frontend-spezifische Anweisungen
      .claude/skills/
      src/
    shared/
      CLAUDE.md                 # Anweisungen für gemeinsame Bibliothek
      src/
```

<h2 id="choose-where-to-start-claude">
  Wählen Sie, wo Claude gestartet werden soll
</h2>

Wo Sie `claude` starten, bestimmt, welche Dateien Claude ohne zusätzliche Genehmigung lesen und bearbeiten kann, welche CLAUDE.md-Dateien beim Start in den Kontext geladen werden, und welche Projekteinstellungen gelten.

| Start von            | Dateizugriff                               | CLAUDE.md beim Start geladen                                                               | Verwenden Sie, wenn                                          |
| :------------------- | :----------------------------------------- | :----------------------------------------------------------------------------------------- | :----------------------------------------------------------- |
| Repository-Root      | Jede Datei                                 | Nur Root; Dateien in Unterverzeichnissen werden bei Bedarf geladen, wenn Claude dort liest | Aufgaben erstrecken sich über mehrere Pakete oder Subsysteme |
| Ein Unterverzeichnis | Nur dieser Teilbaum, bis Sie mehr gewähren | Das Verzeichnis plus jedes übergeordnete Verzeichnis                                       | Die Arbeit ist auf ein Paket oder Subsystem beschränkt       |

Projekteinstellungen in `.claude/settings.json` werden nur aus Ihrem Startverzeichnis geladen und nicht von übergeordneten Verzeichnissen geerbt, wie es bei CLAUDE.md-Dateien der Fall ist: Eine `.claude/settings.json` im Repository-Root gilt nur, wenn Sie vom Root starten.

Jeder Abschnitt unten gibt an, ob seine Einstellungsdatei im Repository-Root oder im Unterverzeichnis, von dem Sie starten, gehört, und ob sie committed oder lokal behalten wird.

<h2 id="layer-claude-md-files-by-directory">
  Schichten Sie CLAUDE.md-Dateien nach Verzeichnis
</h2>

In einer großen Codebase neigt eine einzelne CLAUDE.md im Repository-Root dazu, entweder zu wachsen, um die Konventionen jedes Subsystems abzudecken, was Kontext für Anweisungen kostet, die nicht mit der aktuellen Aufgabe zusammenhängen, oder zu generisch zu bleiben, um nützlich zu sein. Das Aufteilen von Anweisungen auf Per-Verzeichnis-Dateien bedeutet, dass Claude Repository-weite Regeln plus nur die Konventionen für den Code lädt, an dem Sie arbeiten.

Claude Code lädt jede [CLAUDE.md](/docs/de/memory)-Datei aus Ihrem Arbeitsverzeichnis und jedem übergeordneten Verzeichnis beim Start, lädt dann die Datei jedes Unterverzeichnisses bei Bedarf, wenn es dort Dateien liest. Eine Root-Datei setzt Repository-weite Regeln und jedes Unterverzeichnis fügt seine eigenen hinzu.

Eine häufige Aufteilung ist zwei Ebenen:

* **Root `CLAUDE.md`**: Anweisungen, die überall gelten, wie Coding-Standards, Commit-Konventionen und Repository-Layout
* **Per-Unterverzeichnis `CLAUDE.md`**: Konventionen spezifisch für den Stack dieses Bereichs. In einem Monorepo ist das eine pro Paket. In einem großen Single-Tree ist es eine pro Subsystem wie `src/db/` oder `src/api/`

Committen Sie diese Dateien ins Repository, damit Teamkollegen sie erben. Der Eigentümer jedes Verzeichnisses verwaltet typischerweise seine Datei.

Die Root `CLAUDE.md` orientiert Claude an der Repository-Struktur:

```markdown CLAUDE.md theme={null}
Dies ist ein Monorepo mit drei Paketen unter packages/:

- packages/api: Node.js REST API mit Express, TypeScript und PostgreSQL
- packages/web: React-Frontend mit Vite, TypeScript und TailwindCSS
- packages/shared: Gemeinsame TypeScript-Utilities, die von api und web verwendet werden

Führen Sie Befehle aus dem Paketverzeichnis aus, nicht aus dem Monorepo-Root.
Jedes Paket hat seine eigene tsconfig.json, package.json und Test-Suite.
```

Die `CLAUDE.md` jedes Unterverzeichnisses, hier `packages/api/CLAUDE.md`, fügt Kontext hinzu, der spezifisch für den Stack dieses Bereichs ist:

```markdown packages/api/CLAUDE.md theme={null}
Dieses Paket ist der REST-API-Server.

- Tests ausführen: `npm test` (verwendet Vitest)
- Dev-Server ausführen: `npm run dev` (Port 3001)
- Datenbankmigration: `npm run migrate`
- Umgebungsvariablen: Kopieren Sie `.env.example` zu `.env`

API-Routen sind in src/routes/. Jede Route-Datei exportiert einen Express-Router.
Datenbankabfragen verwenden Knex in src/db/. Schreiben Sie niemals Raw-SQL-Strings in Route-Handler.
```

Wenn Sie Claude von `packages/api/` starten, lädt es sowohl `packages/api/CLAUDE.md` als auch die Root `CLAUDE.md`. Claude sieht die lokalen Anweisungen neben den Repository-weiten Regeln, ohne Anweisungen von `packages/web/` im Kontext. Das Gleiche gilt für jedes Unterverzeichnis in einem Non-Monorepo-Baum.

Ein paar Möglichkeiten, um die Dateien aktuell zu halten, während sich die Codebase und Modelle ändern:

* **In Pull Requests überprüfen**: Behandeln Sie CLAUDE.md-Änderungen wie jede andere Dokumentationsänderung, damit Konventionen den Code verfolgen
* **Nach großen Modellveröffentlichungen erneut besuchen**: Anweisungen, die eine Einschränkung eines älteren Modells umgingen, können Overhead werden, sobald ein neueres Modell den Fall selbst handhabt. Zum Beispiel kann eine Regel, die Single-File-Refactors erzwingt, gelöscht werden, sobald die Einschränkung weg ist
* **Fügen Sie einen Stop-Hook hinzu, der Updates vorschlägt**: Ein [`Stop`-Hook](/docs/de/hooks#stop) erhält den Pfad zum Sitzungstranskript, wenn Claude antwortet, daher kann ein Skript die Sitzung überprüfen und CLAUDE.md-Updates vorschlagen, während die Lücke, die es offengelegt hat, noch frisch ist

Weitere Informationen darüber, wie CLAUDE.md-Dateien geladen werden und interagieren, finden Sie unter [Memory und Projektanweisungen](/docs/de/memory).

<h3 id="choose-between-per-directory-claude-md-and-path-scoped-rules">
  Wählen Sie zwischen Per-Verzeichnis-CLAUDE.md und Pfad-bezogenen Regeln
</h3>

Per-Verzeichnis-`CLAUDE.md`-Dateien und [Pfad-bezogene Regeln](/docs/de/memory#path-specific-rules) unter `.claude/rules/` ermöglichen es Ihnen beide, Anweisungen auf einen Teil des Baums auszurichten. Sie unterscheiden sich darin, wo die Datei lebt und wann sie geladen wird.

| Ansatz                                  | Dateispeicherort                                | Wird geladen, wenn                                                                                    | Verwenden Sie, wenn                                                                                    |
| :-------------------------------------- | :---------------------------------------------- | :---------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------- |
| Per-Verzeichnis `CLAUDE.md`             | Innerhalb des Verzeichnisses, neben seinem Code | Beim Start, wenn von diesem Verzeichnis gestartet, oder bei Bedarf, wenn Claude eine Datei dort liest | Verzeichniseigentümer verwalten ihre eigenen Konventionen; Anweisungen werden mit dem Code versioniert |
| Pfad-bezogene Regel in `.claude/rules/` | Zentral `.claude/` im Repository-Root           | Wenn Claude mit einer Datei arbeitet, die dem `paths:`-Glob der Regel entspricht                      | Sie möchten alle Konventionen an einem Ort, oder die gleiche Regel gilt für viele verstreute Pfade     |

Für einen Vergleich, der auch Skills abdeckt, siehe [Vergleichen Sie ähnliche Funktionen](/docs/de/features-overview#compare-similar-features).

<h3 id="exclude-irrelevant-claude-md-files">
  Schließen Sie irrelevante CLAUDE.md-Dateien aus
</h3>

Wenn Sie Claude vom Repository-Root starten, lädt sich die CLAUDE.md jedes Unterverzeichnisses, sobald Claude eine Datei in diesem Verzeichnis liest. Die `claudeMdExcludes`-Einstellung überspringt spezifische Dateien nach Pfad oder Glob-Muster, damit sie nie geladen werden.

Verwenden Sie dies für Verzeichnisse, in denen Sie nie arbeiten, wie Pakete anderer Teams, Legacy-Code oder Vendor-Teilbäume. Die Ausschlussliste ist statisch, nicht ein Pro-Aufgaben-Schalter. Um sich heute auf ein Paket und morgen auf ein anderes zu konzentrieren, [starten Sie Claude von diesem Paketverzeichnis](#choose-where-to-start-claude) anstatt Ausschlüsse zu bearbeiten.

Wenn Sie diese Ausschlüsse nur für sich selbst möchten, setzen Sie die Einstellung in `.claude/settings.local.json`. Claude Code ignoriert diese Datei, wenn es sie erstellt; da Sie sie hier von Hand erstellen, fügen Sie sie zu Ihrer gitignore hinzu. Muster verwenden Glob-Syntax, die gegen absolute Dateipfade abgeglichen wird, daher starten Sie Muster im Stil relativ mit `**/`, um überall im Baum zu passen. Das Beispiel unten schließt Pakete aus, die von anderen Teams gehören:

```json .claude/settings.local.json theme={null}
{
  "claudeMdExcludes": [
    "**/packages/admin-dashboard/**",
    "**/packages/legacy-*/**"
  ]
}
```

Dies überspringt jede CLAUDE.md und Rules-Datei unter diesen Paketen. Die Root CLAUDE.md und die Pakete, in denen Sie arbeiten, werden weiterhin normal geladen.

Diese Muster decken andere häufige Fälle ab:

* `"**/packages/*/CLAUDE.md"`: schließt die CLAUDE.md jedes Pakets aus, während der Root erhalten bleibt
* `"**/packages/web/**"`: schließt alles unter dem Web-Paket aus, einschließlich Regeln
* `"/home/user/monorepo/legacy/CLAUDE.md"`: schließt eine spezifische Datei nach absolutem Pfad aus

Verwaltete Policy-CLAUDE.md-Dateien können nicht ausgeschlossen werden, daher gelten organisationsweite Anweisungen immer. Sie können `claudeMdExcludes` auf jedem [Einstellungsbereich](/docs/de/settings#configuration-scopes) setzen: Benutzer, Projekt, lokal oder verwaltet. Arrays werden über Bereiche hinweg zusammengeführt, daher kann ein Team Projekt-Level-Standardwerte setzen, während Einzelne lokale Overrides hinzufügen.

Für die vollständige Ausschlussdokumentation siehe [Schließen Sie spezifische CLAUDE.md-Dateien aus](/docs/de/memory#exclude-specific-claude-md-files).

<h2 id="reduce-what-claude-reads">
  Reduzieren Sie, was Claude liest
</h2>

Anweisungen sind nur ein Teil dessen, was in Claudes Kontext endet. Dateileseoperationen sind ein weiterer Kostenfaktor, der mit der Codebase wächst. Die Einstellungen unten blockieren Lesevorgänge irrelevanter Pfade und ersetzen erschöpfende Datei-Scans durch Language-Server-Lookups.

<h3 id="block-reads-of-generated-and-vendored-code">
  Blockieren Sie Lesevorgänge von generiertem und Vendor-Code
</h3>

Claudes Inhaltssuchen respektieren `.gitignore` standardmäßig, daher bleiben Pfade, die bereits dort aufgelistet sind, wie `node_modules/`, `dist/` und `build/`, ohne zusätzliche Konfiguration aus Suchergebnissen heraus.

Für Pfade, die eingecheckt sind, wie ein Vendor-SDK oder committed generierter Code, fügen Sie `Read`-Deny-Regeln in `permissions.deny` hinzu, um Claude daran zu hindern, diese Dateien zu öffnen, auch wenn eine Suche sie auflistet.

Um diese Ausschlüsse für alle, die im Repository arbeiten, anzuwenden, committen Sie sie zu `.claude/settings.json`. Um sie persönlich zu halten, verwenden Sie stattdessen `.claude/settings.local.json`. Wie andere Projekteinstellungen auf dieser Seite werden diese Dateien nur aus Ihrem Startverzeichnis geladen. Platzieren Sie sie im Repository-Root, wenn Sie Claude dort starten, oder in jedem Paket `.claude/`, wenn Sie von Unterverzeichnissen starten. Um die gleichen Deny-Regeln in jeder Sitzung unabhängig vom Startverzeichnis durchzusetzen, setzen Sie sie in [verwalteten Einstellungen](/docs/de/settings#settings-files), die Benutzer- und Projekteinstellungen nicht überschreiben können.

Das Beispiel unten blockiert Build-Artefakte und ein Vendor-SDK:

```json .claude/settings.json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./**/dist/**)",
      "Read(./**/build/**)",
      "Read(./**/*.generated.*)",
      "Read(./vendor/**)"
    ]
  }
}
```

Deny-Regeln decken Claudes integrierte Datei-Tools und erkannte Bash-Dateibefehle ab, einschließlich `cat`, `head`, `grep` und `find`, wenn ein verweigerte Pfad als Argument übergeben wird. Sie filtern verweigerte Pfade nicht aus der Ausgabe einer rekursiven Suche heraus, und sie decken keine willkürlichen Subprozesse ab, die Dateien selbst öffnen. Für die vollständige Mustersyntax siehe [Read und Edit Berechtigungsregeln](/docs/de/permissions#read-and-edit).

<h3 id="reduce-file-reads-with-code-intelligence">
  Reduzieren Sie Dateileseoperationen mit Code Intelligence
</h3>

In einer großen Codebase kann das Finden, wo ein Symbol definiert oder verwendet wird, viele Dateileseoperationen und Grep-Aufrufe kosten. [Code-Intelligence-Plugins](/docs/de/discover-plugins#code-intelligence) verbinden Claude mit einem Language Server, damit er zu Definitionen springen, Referenzen finden und Typfehler direkt anzeigen kann, anstatt den Baum zu scannen.

Der offizielle Marketplace hat Plugins für TypeScript, Python, Go, Rust und andere häufige Sprachen. Das Beispiel unten installiert das TypeScript-Plugin:

```shell theme={null}
/plugin install typescript-lsp@claude-plugins-official
```

Um ein Plugin für alle im Repository zu aktivieren, anstatt es selbst zu installieren, fügen Sie es zur [`enabledPlugins`-Projekteinstellung](/docs/de/settings#plugin-settings) hinzu.

Code-Intelligence-Plugins erfordern die Language-Server-Binärdatei der Sprache auf jedem Entwickler-Computer. Siehe [welche Binärdatei jede Sprache erfordert](/docs/de/discover-plugins#code-intelligence). Die Installation aus dem offiziellen Marketplace erfordert Netzwerkzugriff zu GitHub, wo der Marketplace gehostet wird. In einem eingeschränkten Netzwerk [fügen Sie den Marketplace stattdessen von einem internen Git-Host oder lokalen Pfad hinzu](/docs/de/discover-plugins#add-from-other-git-hosts).

Dies funktioniert gut mit `claudeMdExcludes` und den `Read`-Deny-Regeln oben. Diese halten irrelevante Inhalte aus dem Kontext heraus, und Code Intelligence hält Claude davon ab, durch das zu lesen, was bleibt, um eine Definition zu finden.

<h2 id="scope-worktrees-and-file-access">
  Bereich Worktrees und Dateizugriff
</h2>

Diese Einstellungen steuern, was auf der Festplatte in Worktrees ist und welche Verzeichnisse Claude über Ihren Startpunkt hinaus lesen und schreiben kann.

<h3 id="check-out-only-the-directories-you-need">
  Checken Sie nur die Verzeichnisse aus, die Sie benötigen
</h3>

Das `--worktree`-Flag startet eine Sitzung in einem neuen Git-Worktree, damit Änderungen von Ihrem Haupt-Checkout isoliert bleiben. Standardmäßig checkt es das gesamte Repository aus. In einem großen Repository verwendet die `worktree.sparsePaths`-Einstellung Git Sparse-Checkout, um nur die aufgelisteten Verzeichnisse plus Root-Level-Dateien auf die Festplatte zu schreiben, damit Worktrees schneller starten und weniger Platz verwenden.

Wenn jeder, der in diesem Verzeichnis arbeitet, die gleichen Pfade benötigt, committen Sie die Einstellung zu `.claude/settings.json`. Um Pfade für sich selbst hinzuzufügen, verwenden Sie `.claude/settings.local.json`: Die Listen werden über Bereiche hinweg zusammengeführt, daher kann eine lokale Datei Pfade zur committed Liste hinzufügen, aber nicht entfernen. Das Beispiel unten zeigt die committed Datei:

```json .claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ]
  }
}
```

Wenn Claude einen Worktree erstellt, checkt er nur `.claude/`, `packages/api/` und `packages/shared/` aus, anstatt des vollständigen Baums. Pfade in `sparsePaths` sind relativ zum Repository-Root, unabhängig davon, von welchem Unterverzeichnis Sie Claude starten. Alle Verzeichnispfade funktionieren hier, nicht nur Paket-Roots.

Dies ist besonders nützlich für [Subagenten-Worktree-Isolation](/docs/de/worktrees#isolate-subagents-with-worktrees). Subagenten sind parallele Claude-Instanzen, die für Unteraufgaben spawned werden, und jede, die in einem Worktree läuft, bekommt einen leichtgewichtigen Checkout anstelle des vollständigen Baums. Alle Worktrees in einer Sitzung teilen die gleichen `sparsePaths`, daher wenn ein Subagent `packages/api/` benötigt und ein anderer `packages/web/` benötigt, listen Sie beide auf.

Listen Sie Verzeichnisse in `sparsePaths` auf, nicht einzelne Dateien. Root-Level-Dateien wie `package.json`, `tsconfig.base.json` und Lock-Dateien werden immer neben den Verzeichnissen ausgecheckt, die Sie auflisten. Root-Level-Verzeichnisse sind nicht, daher schließen Sie `.claude` in die Liste ein, wenn Sie die `.claude/settings.json`, `.claude/rules/` oder `.claude/skills/` des Repository-Roots im Worktree verfügbar haben möchten.

Sparse Checkout erfordert, dass Git `extensions.worktreeConfig` in der gemeinsamen `.git/config` des Repositorys aktiviert, während ein Sparse-Worktree existiert. Claude Code entfernt diesen Eintrag nach dem Entfernen des letzten Worktrees, aber nur wenn Claude Code ihn hinzugefügt hat. Es entfernt niemals einen Wert, den Sie selbst gesetzt haben. Vor v2.1.207 blieb der Eintrag nach dem Entfernen des letzten Worktrees bestehen, und Go-Git-basierte Tools wie `tea` konnten das Repository nicht öffnen, bis Sie `git config --unset extensions.worktreeConfig` ausführten.

Um zu vermeiden, dass große Verzeichnisse wie `node_modules` über Worktrees dupliziert werden, kombinieren Sie `sparsePaths` mit `symlinkDirectories` in der gleichen `.claude/settings.json`:

```json .claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ],
    "symlinkDirectories": [
      "node_modules"
    ]
  }
}
```

Dies erstellt einen Symlink von jedem Worktree `node_modules/` zurück zur Kopie des Haupt-Repositorys, anstatt ihn auf der Festplatte zu duplizieren.

<Note>
  Die `sparsePaths`- und `symlinkDirectories`-Einstellungen werden aus Ihrem Startverzeichnis gelesen, bevor der Worktree erstellt wird. Nach der Erstellung ist das Arbeitsverzeichnis der Sitzung der Worktree-Root, nicht das Unterverzeichnis, von dem Sie gestartet haben. Projekteinstellungen im Worktree werden daher aus der `.claude/settings.json` des Worktree-Roots geladen, der ausgecheckten Kopie der Datei des Repository-Roots. Setzen Sie alle anderen Einstellungen, die Sie in Worktrees benötigen, wie Berechtigungsregeln oder Hooks, in die `.claude/settings.json` des Repository-Roots.
</Note>

Für die vollständige Worktree-Einstellungsreferenz siehe [Worktree-Einstellungen](/docs/de/settings#worktree-settings).

<h3 id="grant-access-across-packages-or-repositories">
  Gewähren Sie Zugriff über Pakete oder Repositories
</h3>

Dieser Abschnitt gilt, wenn Sie Claude von einem Unterverzeichnis starten, oder wenn eine Aufgabe mehrere Checkouts umfasst. Wenn Sie vom Repository-Root in einem großen Single-Tree starten, hat Claude bereits Zugriff auf jede Datei und Sie können dies überspringen.

Wenn Sie Claude von `packages/api/` starten, kann es Dateien in diesem Verzeichnis lesen und schreiben. Wenn eine Aufgabe Änderungen über Pakete hinweg erfordert, wie das Aktualisieren eines gemeinsamen Typs, den sowohl `api` als auch `web` importieren, müssen Sie Zugriff auf das Schwester-Verzeichnis gewähren. Der gleiche Mechanismus gewährt Zugriff auf ein separat ausgechecktes Repository.

Die `additionalDirectories`-Einstellung in `.claude/settings.json` gibt Claude Zugriff auf Verzeichnisse außerhalb des Arbeitsverzeichnisses. Das Beispiel unten gewährt Zugriff auf zwei Schwester-Pakete:

```json .claude/settings.json theme={null}
{
  "permissions": {
    "additionalDirectories": [
      "../shared",
      "../web"
    ]
  }
}
```

Relative Pfade werden gegen das Verzeichnis aufgelöst, von dem Sie Claude starten. Mit dieser Konfiguration kann Claude Dateien in `packages/shared/` und `packages/web/` lesen und bearbeiten, während er von `packages/api/` arbeitet.

Sie können auch zur Laufzeit Zugriff gewähren, ohne Einstellungen zu bearbeiten, indem Sie `--add-dir` übergeben, wenn Sie Claude starten:

```bash theme={null}
claude --add-dir ../shared
```

Wie auch immer Sie ein Verzeichnis hinzufügen, Claude kann Dateien darin lesen und bearbeiten. Ob die CLAUDE.md, `.claude/rules/`-Dateien und Skills des Verzeichnisses auch geladen werden, hängt davon ab, wie Sie es hinzugefügt haben:

| Hinzugefügt mit                         | Lädt CLAUDE.md und Regeln           | Lädt Skills |
| :-------------------------------------- | :---------------------------------- | :---------- |
| `additionalDirectories`-Einstellung     | Nie                                 | Nie         |
| `--add-dir`-Flag oder `/add-dir`-Befehl | Nur mit der Umgebungsvariable unten | Ja          |

Um CLAUDE.md und Rules-Dateien aus einem Verzeichnis zu laden, das mit `--add-dir` oder `/add-dir` hinzugefügt wurde, setzen Sie die `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD`-Umgebungsvariable:

```bash theme={null}
CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1 claude --add-dir ../shared
```

Die Umgebungsvariable hat keine Auswirkung auf Verzeichnisse, die in der `additionalDirectories`-Einstellung aufgelistet sind. Siehe [Laden aus zusätzlichen Verzeichnissen](/docs/de/memory#load-from-additional-directories) für Details.

Für Schwester-Verzeichnisse, die jeder in diesem Bereich benötigt, committen Sie `additionalDirectories` zu `.claude/settings.json`. Für eine persönliche Auswahl oder einmaligen Zugriff verwenden Sie `.claude/settings.local.json` oder übergeben Sie `--add-dir` beim Start.

<h2 id="add-per-directory-skills">
  Fügen Sie Per-Verzeichnis-Skills hinzu
</h2>

Jedes Unterverzeichnis kann [Skills](/docs/de/skills) definieren, die auf seinen eigenen Stack beschränkt sind. Ein Skill wird bei Bedarf geladen, wenn Claude bestimmt, dass er relevant ist, daher verbraucht API-spezifisches Tooling keinen Kontext während Frontend-Arbeit.

Skills leben unter `.claude/skills/` im Verzeichnis. Committen Sie sie neben dem Code dieses Bereichs, damit jeder, der das Repository klont, sie bekommt. In einem Monorepo kann dies ein Satz Skills pro Paket sein. In einer großen Single-Tree-Codebase ist es ein Satz pro Subsystem wie `src/db/.claude/skills/`.

Erstellen Sie ein Skill-Verzeichnis im Unterverzeichnis:

```bash theme={null}
mkdir -p packages/api/.claude/skills/api-testing
```

Schreiben Sie dann `SKILL.md` in diesem Verzeichnis, hier `packages/api/.claude/skills/api-testing/SKILL.md`. Dieses Beispiel lehrt Claude die Test-Muster des API-Pakets:

```markdown packages/api/.claude/skills/api-testing/SKILL.md theme={null}
---
name: api-testing
description: Test-Muster für das API-Paket. Verwenden Sie beim Schreiben oder Ändern von Tests in packages/api/.
---

## Test-Struktur

Tests sind in `src/__tests__/` und spiegeln die `src/`-Verzeichnisstruktur.
Jede Route-Datei hat eine entsprechende `.test.ts`-Datei.

## Tests ausführen

- Alle Tests: `npm test`
- Einzelne Datei: `npm test -- src/__tests__/routes/users.test.ts`
- Watch-Modus: `npm test -- --watch`

## Test-Utilities

- `src/__tests__/helpers/db.ts`: bietet `setupTestDb()` und `teardownTestDb()` für Datenbank-Tests
- `src/__tests__/helpers/auth.ts`: bietet `createTestUser()` und `getAuthToken()` für authentifizierte Endpoints

## Muster

- Verwenden Sie `supertest` für HTTP-Assertions, nicht Raw-Fetch
- Wickeln Sie Datenbank-Tests immer in eine Transaktion, die zurückrollt
- Mock externe Services in `src/__tests__/mocks/`
```

Ein anderes Unterverzeichnis hält auf die gleiche Weise andere Skills: `packages/web/.claude/skills/component-patterns/` beschreibt die Frontend-Komponentenkonventionen anstelle von Tests. Wenn Claude an einer Datei in `packages/api/` arbeitet, lädt es den api-testing-Skill. Wenn es in `packages/web/` arbeitet, lädt es stattdessen component-patterns. Die Skills keines Verzeichnisses werden während der Aufgaben des anderen geladen.

Sie können einen Skill auch nach Dateimuster anstelle von Platzierung scoped. Das [`paths`-Frontmatter-Feld](/docs/de/skills#frontmatter-reference) nimmt Glob-Muster, und Claude lädt den Skill automatisch nur, wenn er mit passenden Dateien arbeitet. Verwenden Sie dies für einen Skill, der im `.claude/skills/` des Repository-Roots lebt, aber nur auf bestimmte Dateien überall angewendet wird, wie einen Datenbankmigrations-Skill, der auf `**/migrations/**` beschränkt ist.

Weitere Informationen zum Erstellen und Organisieren von Skills finden Sie unter [Skills](/docs/de/skills).

<h3 id="keep-skills-discoverable">
  Halten Sie Skills auffindbar
</h3>

Mit Skills, die über viele Verzeichnisse verteilt sind, kann die Liste, aus der Claude wählt, groß werden. Claude wählt einen Skill, indem er den Namen und die Beschreibung jedes entdeckten Skills liest, und nur der Inhalt des gewählten Skills wird vollständig in den Kontext geladen. Dieser Abschnitt behandelt, wie Sie diese Liste klein halten und Beschreibungen schreiben, die Verkürzung überstehen.

Welche Skills im Bereich sind, hängt davon ab, wo Sie Claude starten:

* **Von einem Unterverzeichnis wie `packages/api/`**: Skills aus diesem Verzeichnis, jedem übergeordneten bis zum Repository-Root und den Benutzer- und Enterprise-Ebenen
* **Vom Repository-Root**: Skills aus jedem Unterverzeichnis, das Claude während der Sitzung berührt, was sich zu Hunderten ansammeln kann
* **Nach dem Hinzufügen eines Schwester mit [`--add-dir`](#grant-access-across-packages-or-repositories)**: Die Skills dieses Schwester werden auch geladen. Die `additionalDirectories`-Einstellung gewährt nur Dateizugriff und lädt keine Skills

Namen werden immer geladen, aber [Beschreibungen werden gekürzt, wenn es viele gibt](/docs/de/skills#skill-descriptions-are-cut-short), was die Schlüsselwörter entfernen kann, die Claude verwendet, um zu entscheiden, ob ein Skill angewendet wird. Halten Sie Beschreibungen kurz und führen Sie mit Wörtern an, die eine Anfrage enthalten würde, wie „Schreiben oder Ändern von Tests in `packages/api/`".

Für Skills, die viele Verzeichnisse teilen, wie PR-Konventionen oder eine Deploy-Checkliste, platzieren Sie sie in `.claude/skills/` des Repository-Roots, damit sie von jedem Startverzeichnis geladen werden. Wenn gemeinsame Skills ihre eigene Versionsgeschichte benötigen oder über Repositories hinweg funktionieren müssen, packen Sie sie stattdessen als [Plugin](/docs/de/plugins). Plugin-Skills verwenden einen `plugin-name:skill-name`-Namespace, daher kollidieren sie nie mit Per-Verzeichnis-Skills. Ein Platform-Team kann sie an einem Ort versionieren und aktualisieren.

Um zu finden, welche Skills ungenutzt sind, aktivieren Sie den OpenTelemetry [Logs Exporter](/docs/de/monitoring-usage) und setzen Sie `OTEL_LOG_TOOL_DETAILS=1`, damit Skill-Namen wörtlich aufgezeichnet werden, anstatt redigiert zu werden. Das [`skill_activated`-Event](/docs/de/monitoring-usage#skill-activated-event) zeichnet jede Invokation in seinem `skill.name`-Attribut auf, und `invocation_trigger` zeichnet auf, ob ein Befehl, Claude oder ein verschachtelter Skill es invokiert hat, was Ihnen sagt, was zu konsolidieren oder zu beenden ist.

<h2 id="centralize-conventions-when-layering-stops-scaling">
  Zentralisieren Sie Konventionen, wenn Schichtung nicht mehr skaliert
</h2>

Per-Verzeichnis-CLAUDE.md-Dateien können schwer zu regieren werden, wenn die Codebase wächst. Konventionen driften ab, Dateien werden veraltet, und niemand besitzt den Root. Das Lösen fällt typischerweise dem Team zu, das die Claude-Code-Einrichtung des Repositorys verwaltet, anstatt zu jedem Entwickler, der in seinem eigenen Bereich arbeitet.

Verschieben Sie Konventionen und Referenzmaterial aus immer geladenem CLAUDE.md in Mechanismen, die bei Bedarf geladen werden:

* [Skills](/docs/de/skills): Referenzmaterial, das Claude nur lädt, wenn es für die Aufgabe relevant ist
* [Plugins](/docs/de/plugins): Versionierte Bundles von Skills, Hooks und Befehlen, die ein Platform-Team zentral besitzt
* [MCP-Server](/docs/de/mcp): Wenn Ihre Organisation bereits eine Code-Suche oder RAG-Index über das Repository ausführt, stellen Sie es als MCP-Tool zur Verfügung, damit Claude es abfragt, anstatt Dateien direkt zu lesen

Siehe [Server-verwaltete oder Endpoint-verwaltete Einstellungen](/docs/de/server-managed-settings#choose-between-server-managed-and-endpoint-managed-settings) für wie Platform-Teams diese zentral durchsetzen können.

<h3 id="recommend-the-right-plugin-at-session-start">
  Empfehlen Sie das richtige Plugin beim Sitzungsstart
</h3>

Sobald Konventionen in Plugins leben, hat ein Teamkollege, der Claude in einem unbekannten Teil des Baums startet, kein Signal darüber, welches Plugin die Eigentümer dieses Bereichs verwalten. Ein [`SessionStart`-Hook](/docs/de/hooks#sessionstart) kann diese Lücke schließen, da alles, das der Hook zu stdout druckt, vor der ersten Eingabeaufforderung zu Claudes Kontext hinzugefügt wird.

Zum Beispiel können Sie ein Skript schreiben, das das Startverzeichnis aus der [Hook-Eingabe](/docs/de/hooks#common-input-fields) liest, es in einer im Repository committed Pfad-zu-Plugin-Map nachschlägt und die Empfehlung druckt, damit Claude sie in seiner ersten Antwort weitergeben kann. Siehe [Automatisieren Sie Aktionen mit Hooks](/docs/de/hooks-guide), um den Hook zu schreiben und zu registrieren.

<h2 id="put-it-together">
  Alles zusammensetzen
</h2>

Die kombinierte Konfiguration unten verwendet das Monorepo-Layout. Die gleichen Dateien funktionieren für jedes Unterverzeichnis in einem großen Single-Tree. Projekteinstellungen werden nur aus dem Verzeichnis geladen, von dem Sie Claude starten, daher muss jedes Unterverzeichnis `.claude/settings.json` selbstständig sein, anstatt auf einer Root-Datei geschichtet zu werden.

Das Beispiel committed `worktree`, `additionalDirectories` und die `Read`-Deny-Regeln in `.claude/settings.json`, daher bekommt jeder Entwickler in `packages/api/` den gleichen Schwester-Zugriff, Sparse-Pfade und Ausschlüsse. Die Datei unten ist die committed Per-Bereich-Einstellung für `packages/api/`:

```json packages/api/.claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ],
    "symlinkDirectories": [
      "node_modules"
    ]
  },
  "permissions": {
    "additionalDirectories": [
      "../shared"
    ],
    "deny": [
      "Read(./**/dist/**)",
      "Read(./**/build/**)"
    ]
  }
}
```

Da diese Sitzung von `packages/api/` startet, sind die CLAUDE.md-Dateien von Schwester-Paketen bereits außerhalb des Bereichs, daher ist `claudeMdExcludes` hier nicht erforderlich. Fügen Sie es stattdessen zu `.claude/settings.local.json` des Repository-Roots hinzu, wenn Sie auch Sitzungen vom Root starten.

Der `additionalDirectories`-Eintrag gilt, wenn Sie Claude direkt von `packages/api/` starten. Innerhalb eines Worktrees, der von dieser Sitzung erstellt wurde, ist das Arbeitsverzeichnis der Worktree-Root, daher wird diese Einstellungsdatei nicht geladen. Die Schwester-Pakete sind bereits im Worktree ohne sie erreichbar, aber die Deny-Regeln benötigen eine zweite Kopie in `.claude/settings.json` des Repository-Roots, damit Worktree-Sitzungen sie abholen, wie die [Worktree-Einstellungsnotiz](#check-out-only-the-directories-you-need) beschreibt:

```json .claude/settings.json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./**/dist/**)",
      "Read(./**/build/**)"
    ]
  }
}
```

Nach der Einrichtung hat das Repository dieses Layout:

```text theme={null}
monorepo/
  CLAUDE.md
  .claude/settings.json                           # Deny-Regeln für Worktree-Sitzungen
  packages/
    api/
      CLAUDE.md
      .claude/settings.json                       # worktree, additionalDirectories, Deny-Regeln
      .claude/skills/api-testing/SKILL.md
    web/
      CLAUDE.md
      .claude/skills/component-patterns/SKILL.md
    shared/
      CLAUDE.md
```

Mit dieser Einrichtung, wenn Sie Claude von `packages/api/` starten:

* Lädt die Root CLAUDE.md und `packages/api/CLAUDE.md`, überspringt `packages/web/CLAUDE.md`
* Kann Dateien in `packages/api/` und `packages/shared/` lesen und bearbeiten
* Überspringt Lesevorgänge von Build-Ausgabe unter `dist/` und `build/` in `packages/api/`
* Hat den api-testing-Skill bei Bedarf verfügbar
* Erstellt Worktrees, die `.claude/`, `packages/api/`, `packages/shared/` und Root-Level-Dateien enthalten, mit den Deny-Regeln, die aus der Root-Einstellungsdatei über den Worktree angewendet werden

<h2 id="scope-and-plan-changes-that-span-packages">
  Bereich und Plan Änderungen, die Pakete umfassen
</h2>

Die Konfiguration oben steuert, was Claude sieht. Wenn eine einzelne Änderung mehrere Pakete berührt, wie das Aktualisieren eines gemeinsamen Typs zusammen mit jedem Call-Site, der ihn verwendet, beeinflussen auch wie Sie die Aufgabe scoped und sequenzieren das Ergebnis.

Zwei Techniken helfen, eine Paket-übergreifende Änderung konsistent zu halten:

* **Geben Sie Claude die ganze Änderung in einer Sitzung**: Das Übergeben der gemeinsamen Bearbeitung und ihrer Call-Sites zusammen hält die Entscheidungen hinter jeder Bearbeitung konsistent, anstatt sie pro Paket neu abzuleiten
* **Speichern Sie den Plan in einer Datei, bevor Sie bearbeiten**: [Planen Sie zuerst](/docs/de/best-practices#explore-first-then-plan-then-code) und bitten Sie Claude, den Plan in eine Markdown-Datei im Repository zu schreiben. Eine lange Paket-übergreifende Sitzung [komprimiert ihren Kontext](/docs/de/context-window#what-survives-compaction) unterwegs, und der gespeicherte Plan überlebt, wo Konversationsverlauf möglicherweise nicht.

<h2 id="next-steps">
  Nächste Schritte
</h2>

Sobald diese Konfiguration vorhanden ist, können Sie sie verfeinern:

* Verwenden Sie [Hooks](/docs/de/hooks-guide), um Per-Verzeichnis-Linter oder Type-Checker nach Claude-Bearbeitungen auszuführen
* Überprüfen Sie [Verwalten Sie Kosten effektiv](/docs/de/costs), um zu verstehen, wie Codebase-Größe die Token-Nutzung beeinflusst und wie Sie Ausgabenlimits vor einem breiteren Rollout setzen
* Lesen Sie [Wie Claude Code in großen Codebases funktioniert](https://claude.com/blog/how-claude-code-works-in-large-codebases-best-practices-and-where-to-start) im Claude-Blog für Organisationsrollout-Muster und Eigentumsmodelle, die über der Per-Repository-Konfiguration auf dieser Seite liegen
