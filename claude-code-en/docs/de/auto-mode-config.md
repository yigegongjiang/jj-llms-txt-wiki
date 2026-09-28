> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Auto-Modus konfigurieren

> Teilen Sie dem Auto-Modus-Klassifizierer mit, welche Repos, Buckets und Domains Ihre Organisation vertraut. Legen Sie den Umgebungskontext fest, überschreiben Sie die Standard-Block- und Allow-Regeln, und überprüfen Sie Ihre effektive Konfiguration mit den Auto-Modus-CLI-Unterbefehlen.

[Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) ermöglicht es Claude Code, ohne routinemäßige Berechtigungsaufforderungen zu laufen, indem Werkzeugaufrufe durch einen Klassifizierer geleitet werden, der alles blockiert, das irreversibel, destruktiv oder außerhalb Ihrer Umgebung ausgerichtet ist. Deny- und explizite Ask-Regeln werden vor dem Klassifizierer ausgewertet und blockieren oder fordern weiterhin auf. Verwenden Sie den `autoMode`-Einstellungsblock, um diesem Klassifizierer mitzuteilen, welche Repos, Buckets und Domains Ihre Organisation vertraut, damit er routinemäßige interne Operationen nicht mehr blockiert.

<Note>
  Auto-Modus ist für alle Benutzer auf jedem Anbieter verfügbar, einschließlich der Anthropic API, [Claude Platform on AWS](/docs/de/claude-platform-on-aws), Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry und angemeldeter [Claude apps gateway](/docs/de/claude-apps-gateway)-Sitzungen. Wenn Claude Code Auto-Modus als nicht verfügbar für Ihr Konto meldet, überprüfen Sie die [vollständigen Anforderungen](/docs/de/permission-modes#eliminate-prompts-with-auto-mode), die auch die unterstützten Modelle und die Kontrolle auf Organisationsebene in Team- und Enterprise-Plänen abdecken. In v2.1.158 bis v2.1.206 erforderte Auto-Modus auf Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry und Claude apps gateway-Sitzungen das Setzen von `CLAUDE_CODE_ENABLE_AUTO_MODE=1`; v2.1.207 entfernte die Anforderung.
</Note>

Standardmäßig vertraut der Klassifizierer nur dem Arbeitsverzeichnis und den konfigurierten Remotes des aktuellen Repos. Aktionen wie das Pushen zu Ihrer Unternehmens-Quellcode-Org oder das Schreiben in einen Team-Cloud-Bucket werden blockiert, bis Sie sie zu `autoMode.environment` hinzufügen.

Informationen dazu, wie Sitzungen in Auto-Modus landen und was der Klassifizierer standardmäßig blockiert, finden Sie unter [Auto-Modus auf der Seite „Berechtigungsmodi"](/docs/de/permission-modes#eliminate-prompts-with-auto-mode). Diese Seite ist die Konfigurationsreferenz.

Diese Seite behandelt, wie Sie:

* [Einen menschlichen Checkpoint](#add-a-human-checkpoint) für Pushes und Pull Requests mit `permissions.ask` hinzufügen
* [Wählen Sie, wo Sie Regeln festlegen](#where-the-classifier-reads-configuration) über CLAUDE.md, Benutzereinstellungen und verwaltete Einstellungen
* [Definieren Sie vertrauenswürdige Infrastruktur](#define-trusted-infrastructure) mit `autoMode.environment`
* [Generieren Sie Umgebungseinträge](#generate-environment-entries) mit `/auto-mode-setup`
* [Überschreiben Sie die Block- und Allow-Regeln](#override-the-block-and-allow-rules), wenn die Standardwerte nicht zu Ihrer Pipeline passen
* [Bearbeiten Sie Regeln von `/permissions`](#edit-rules-from-permissions) ohne eine Einstellungsdatei zu öffnen
* [Leiten Sie alle Shell-Befehle durch den Klassifizierer](#route-all-shell-commands-through-the-classifier) mit `autoMode.classifyAllShell`
* [Überprüfen Sie Ihre effektive Konfiguration](#inspect-the-defaults-and-your-effective-config) mit den `claude auto-mode`-Unterbefehlen
* [Überprüfen Sie Ablehnungen](#review-denials), damit Sie wissen, was Sie als Nächstes hinzufügen müssen

<h2 id="common-boundaries">
  Häufige Grenzen
</h2>

Der Auto-Modus ermöglicht standardmäßig Pushes zu jedem Zweig des Repositorys, in dem Sie arbeiten, einschließlich des Standard-Zweigs, und die Erstellung von Pull Requests. Ein Nicht-Standard-Zweig, dessen Name ihn als Deploy- oder Veröffentlichungsziel kennzeichnet, wie `production`, `release` oder `gh-pages`, wird von diesem Standard nicht abgedeckt: Der Klassifizierer beurteilt einen Push dorthin nach seinen eigenen Bedingungen, einschließlich als Production-Deploy. Der Inhalt des Push wird ebenfalls noch überprüft, sodass ein Force Push, ein Geheimnis, das in den Commit eintritt, oder eine Änderung, die Geheimnisse außerhalb des Repositorys senden würde, wenn CI oder eine Deploy-Pipeline es ausführt, blockiert bleibt.

<Info>Vor v2.1.211 erlaubte der Klassifizierer Pushes nur zu Ihrem Arbeitszweig, von Claude erstellten Zweigen und regelmäßigen Pushes zum Standard-Zweig.</Info>

Wenn Sie vor jedem Push und Pull Request einen menschlichen Checkpoint wünschen, fügen Sie Berechtigungsregeln hinzu: Die [Rezepte unten](#add-a-human-checkpoint) halten den Auto-Modus für alles andere aktiviert.

<h3 id="add-a-human-checkpoint">
  Einen menschlichen Checkpoint hinzufügen
</h3>

Der direkteste Mechanismus ist [`permissions.ask`](/docs/de/permissions#permission-rule-syntax). Inhaltsgebundene Ask-Regeln wie die folgenden werden vor dem Klassifizierer ausgewertet und erzwingen immer eine Berechtigungsaufforderung, auch im Auto-Modus, da eine explizite Ask-Regel Ihre ausdrückliche Absicht ist, für diese Aktion aufgefordert zu werden. Fügen Sie die Regeln in Ihren [Einstellungen](/docs/de/settings#where-settings-live) hinzu:

```json theme={null}
{
  "permissions": {
    "ask": [
      "Bash(git push *)",
      "Bash(gh pr create *)"
    ]
  }
}
```

Diese Regeln entsprechen Befehlen, die mit `git push` oder `gh pr create` beginnen. Ein Push, den Claude auf andere Weise schreibt, wie `git -C <dir> push` oder `git -c <key>=<value> push`, [entspricht der Regel nicht](/docs/de/permissions#bash-rule-limits), daher wird er nicht kontrolliert. Für einen Checkpoint, der den vollständigen Befehlstext überprüft, fügen Sie einen [PreToolUse Hook](/docs/de/hooks#pretooluse) hinzu.

Wählen Sie den Mechanismus, der der Festigkeit der erforderlichen Grenze entspricht:

| Grenze                             | Mechanismus                                                               | Verhalten im Auto-Modus                                                                                                                                                                                                                                                          |
| :--------------------------------- | :------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Aufforderung vor der Aktion        | `permissions.ask`                                                         | Fordert immer für einen Befehl auf, der einer inhaltsgebundenen Regel wie dem obigen Rezept entspricht. Der Klassifizierer kann eine übereinstimmende Aktion nicht automatisch genehmigen.                                                                                       |
| Aktion niemals ausführen           | `permissions.deny`                                                        | Blockiert, bevor der Klassifizierer konsultiert wird. Weder der Klassifizierer noch die Benutzerabsicht können dies überschreiben.                                                                                                                                               |
| Einmalige Grenze für diese Sitzung | Geben Sie es im Gespräch an, z. B. „nicht pushen, bis ich überprüft habe" | Der Klassifizierer blockiert übereinstimmende Aktionen, aber die Grenze kann verloren gehen, wenn die [Kontext-Komprimierung](/docs/de/costs#reduce-token-usage) die Nachricht entfernt, die sie angegeben hat. Verwenden Sie eine Ask- oder Deny-Regel für eine dauerhafte Garantie. |

<h2 id="where-the-classifier-reads-configuration">
  Wo der Klassifizierer die Konfiguration liest
</h2>

Der Klassifizierer liest denselben [CLAUDE.md](/docs/de/memory)-Inhalt, den Claude selbst lädt, sodass eine Anweisung wie „niemals force push" in der CLAUDE.md deines Projekts sowohl Claude als auch den Klassifizierer gleichzeitig steuert. Beginne dort mit Projektkonventionen und Verhaltensregeln.

Für Regeln, die projektübergreifend gelten, wie vertrauenswürdige Infrastruktur oder organisationsweite Ablehnungsregeln, verwende den `autoMode`-Einstellungsblock. Der Klassifizierer liest `autoMode` aus den folgenden Bereichen:

| Bereich                          | Datei                                                   | Verwendung für                                                        |
| :------------------------------- | :------------------------------------------------------ | :-------------------------------------------------------------------- |
| Ein Entwickler                   | `~/.claude/settings.json`                               | Persönliche vertrauenswürdige Infrastruktur                           |
| Organisationsweit                | [Verwaltete Einstellungen](/docs/de/server-managed-settings) | Vertrauenswürdige Infrastruktur, die an alle Entwickler verteilt wird |
| `--settings`-Flag oder Agent SDK | Inline-JSON                                             | Pro-Aufruf-Überschreibungen für Automatisierung                       |

Der Klassifizierer liest `autoMode` nicht aus Projekteinstellungen in `.claude/settings.json` oder `.claude/settings.local.json`. Beide Dateien befinden sich im Repository-Verzeichnis, sodass ein eingechecktes Repository oder ein Build-Schritt sonst seine eigenen Allow-Regeln injizieren könnte. Vor v2.1.207 las der Klassifizierer auch `.claude/settings.local.json`; verschiebe alle `autoMode`-Blöcke in dieser Datei zu `~/.claude/settings.json`. Das Ausschließen von `.claude/settings.local.json` schließt auch den Fall, in dem ein Repository die Datei committed oder ein lokales Tool oder Build-Schritt sie schreibt.

Einträge aus jedem Bereich werden kombiniert. Ein Entwickler kann `environment`, `allow`, `soft_deny` und `hard_deny` mit persönlichen Einträgen erweitern, kann aber Einträge, die verwaltete Einstellungen bereitstellen, nicht entfernen. Da Allow-Regeln als Ausnahmen zu Soft-Block-Regeln innerhalb des Klassifizierers fungieren, kann ein von einem Entwickler hinzugefügter `allow`-Eintrag einen organisatorischen `soft_deny`-Eintrag überschreiben: Die Kombination ist additiv, keine harte Richtliniengrenze.

<Note>
  Der Klassifizierer ist ein zweites Gate, das nach dem [Berechtigungssystem](/docs/de/permissions) ausgeführt wird. Für Aktionen, die unabhängig von Benutzerabsicht oder Klassifizierer-Konfiguration niemals ausgeführt werden dürfen, verwende `permissions.deny` in verwalteten Einstellungen, das die Aktion blockiert, bevor der Klassifizierer konsultiert wird, und kann nicht überschrieben werden.
</Note>

<h2 id="define-trusted-infrastructure">
  Vertrauenswürdige Infrastruktur definieren
</h2>

Für die meisten Organisationen ist `autoMode.environment` das einzige Feld, das Sie festlegen müssen. Es teilt dem Klassifizierer mit, welche Repos, Buckets und Domains vertrauenswürdig sind: Der Klassifizierer nutzt dies, um zu entscheiden, was „extern" bedeutet, sodass jedes Ziel, das nicht aufgelistet ist, ein potenzielles Exfiltrationsziel darstellt.

Ab Claude Code v2.1.198 gibt `claude auto-mode defaults` drei Arten von Umgebungseinträgen aus. Versionen vor v2.1.195 geben nur die ersten fünf Vertrauensplätze aus.

* **Kontextplätze**: beschreiben Ihre Organisation, Ihren Stack und Ihre Sicherheitslage, damit der Klassifizierer die anderen Regeln in Ihrem Kontext liest. Jeder hat standardmäßig den Wert `None configured` oder die neben ihm genannte konservative Annahme:
  * **Organisation**
  * **Primäre Verwendung von Claude Code**: standardmäßig Softwareentwicklung
  * **Cloud-Anbieter**
  * **Repository-Sichtbarkeit**: Ein Repository wird als privat angenommen, es sei denn, sein Remote-Host und Name deuten darauf hin, dass es anders ist, oder der Klassifizierer liest eine Sichtbarkeitsprüfung früher im Gespräch, die zeigt, dass es öffentlich ist.

    In den Klassifizierer-Anfragen, die Claude Code selbst sendet, liest der Klassifizierer Ihre Nachrichten und die Befehle, die Claude ausführt, nicht deren Ausgabe. Der Beweis muss etwas sein, das der Klassifizierer lesen kann, wie Ihre eigene Nachricht, die das Repository als öffentlich benennt; die Ausgabe eines `gh repo view` allein erreicht ihn nicht. Die Transkript-Beweis-Prüfung erfordert Claude Code v2.1.200 oder später
  * **Interne Freigabe / Snippet-Hosting**: öffentliche Paste- und Gist-Services werden als außerhalb der Vertrauensgrenze behandelt, bis Sie einen benennen
  * **Organisations-spezifische CLIs**
  * **Secrets-Management**
  * **CI/CD-Bereitstellungsziele**
  * **Netzwerk-Postur**
  * **Host-Containment**: standardmäßig ein gewöhnlicher Entwickler-Rechner oder CI-Runner mit offenem Internet. Wenn Claude Code in einem Container, einer VM oder einem Pod mit einer Egress-Allow-Liste oder Nachbarn läuft, die es nicht berühren darf, benennen Sie die zulässigen Hosts, ob der Cloud-Metadaten-Endpunkt erreichbar sein sollte, und welches Cloud-Projekt, Cluster oder Registry die Aufgabe nutzt und unter welcher Identität. Bis dieser Eintrag diese Identität benennt, [blockiert](/docs/de/permission-modes#what-the-classifier-blocks-by-default) der Klassifizierer Anfragen für die eigenen Anmeldedaten des Hosts. Erfordert Claude Code v2.1.257 oder später
  * **Geschützte Bereitstellungs-Namespaces / Umgebungen**: fällt auf die Heuristik für sensible Remote-Ziele zurück, bis Sie diese benennen
  * **Datenspeicherung / Deklassifizierung**
* **Vertrauensplätze**: benennen, was der Klassifizierer als innerhalb Ihrer Grenze behandelt. Die Plätze sind Trusted repo, Source control, Trusted internal domains, Trusted cloud buckets, Key internal services und Internal package registry. Die Repo- und Source-Control-Einträge haben standardmäßig das funktionierende Repository und seine konfigurierten Remotes. Jeder andere Vertrauensplatz hat standardmäßig den Wert `None configured`, sodass nichts anderes vertrauenswürdig ist, bis Sie es hinzufügen. Die Sichtbarkeit eines Repositorys bezieht sich nur auf vertrauliches Material: Ein privates Repository ist ein akzeptables Ziel für vertrauliches Material, aber das Privatmachen eines Repositorys ermöglicht niemals das Verschieben von Secrets oder persönlichen oder anvertrauten Daten in dieses Repository, und der Klassifizierer behandelt Inhalte, die von außerhalb des funktionierenden Repositorys portiert, umgeleitet oder erstmals gelesen werden, als nicht das eigene Werk dieses Repositorys. Diese Scoping-Anforderung erfordert Claude Code v2.1.203 oder später.
* **Sensibilitätsplätze**: benennen, was die Schutzregeln als hochriskant behandeln. Die Plätze sind Sensitive data locations & audiences, Sensitive remote targets und Protected IaC scopes. Jeder hat standardmäßig eine breite Heuristik, wie das Behandeln eines beliebigen Hosts oder Namespace, dessen Name `prod` oder `production` trägt, als sensibles Remote-Ziel, sodass die Schutzregeln aktiv sind, bevor Sie etwas konfigurieren. Das Benennen konkreter Ziele in einem Sensibilitätsplatz lässt diese Regeln stattdessen auf die benannten Ziele angewendet werden.

<Info>Vor v2.1.211 enthielten die Kontextplätze auch einen Eintrag für Standard / geschützte Branches, der `main` und `master` als geschützt behandelte, bis Sie andere benannten. v2.1.211 entfernte ihn: [Pushes zu jedem Branch des Repositorys, an dem Sie arbeiten](#common-boundaries) sind standardmäßig zulässig, sodass es keinen geschützten-Branch-Standard zu konfigurieren gibt.</Info>

Um Ihre eigenen Einträge neben den Standardwerten hinzuzufügen, fügen Sie die wörtliche Zeichenkette `"$defaults"` in das Array ein. Die Standardeinträge werden an dieser Position eingefügt, sodass Ihre benutzerdefinierten Einträge vor oder nach ihnen stehen können.

Das folgende Beispiel behält die Standardeinträge bei und fügt die Repos, Buckets, Domains und Services einer Organisation hinzu.

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Source control: github.example.com/acme-corp and all repos under it",
      "Trusted cloud buckets: s3://acme-build-artifacts, gs://acme-ml-datasets",
      "Trusted internal domains: *.corp.example.com, api.internal.example.com",
      "Key internal services: Jenkins at ci.example.com, Artifactory at artifacts.example.com"
    ]
  }
}
```

Nachdem Sie Ihre Einstellungen gespeichert haben, führen Sie `claude auto-mode config` aus, um [zu bestätigen, dass die geltenden Regeln](#inspect-the-defaults-and-your-effective-config) Ihre Einträge enthalten.

Einträge sind Prosa, keine Regex oder Tool-Muster. Der Klassifizierer liest sie als natürlichsprachige Regeln. Schreiben Sie sie so, wie Sie Ihre Infrastruktur einem neuen Ingenieur beschreiben würden. Ein gründlicher Umgebungsabschnitt deckt ab:

* **Organisation**: Ihr Unternehmensname und wofür Claude Code hauptsächlich verwendet wird, wie Softwareentwicklung, Infrastrukturautomatisierung oder Datentechnik
* **Source Control**: jede GitHub-, GitLab- oder Bitbucket-Organisation, in die Ihre Entwickler pushen
* **Cloud-Anbieter und vertrauenswürdige Buckets**: Bucketnamen oder Präfixe, aus denen und in die Claude lesen und schreiben können sollte
* **Vertrauenswürdige interne Domains**: Hostnamen für APIs, Dashboards und Services in Ihrem Netzwerk, wie `*.internal.example.com`
* **Wichtige interne Services**: CI, Artifact-Registries, interne Paketindizes, Incident-Tools
* **Interne Paket-Registry**: die private npm-, PyPI- oder andere Registry, durch die Installationen laufen sollten, sodass Installationen, die sie für eine öffentliche Registry umgehen, blockiert werden
* **Sensible Datenspeicherorte & Zielgruppen**: die Buckets, Datenbanken oder Pfade, die persönliche Daten, vertrauliche Geschäftsdaten, Anmeldedaten, regulierte Daten oder ähnlich sensibles Material enthalten, und die Zielgruppen, mit denen Daten an jedem Speicherort geteilt werden dürfen, sodass der Klassifizierer diese Speicherorte schützt, anstatt vom Inhalt zu raten. Claude Code v2.1.195 bis v2.1.197 nennt diesen Eintrag PII / regulated-data locations und deckt nur Speicherorte ab, die persönliche oder regulierte Daten enthalten, ohne die Zielgruppen-Dimension
* **Sensible Remote-Ziele**: die Namespaces, Hosts oder Container, die als Produktion zählen, sodass Remote-Shells und Port-Forwards in diese Ihre explizite Genehmigung benötigen
* **Geschützte IaC-Bereiche**: die Infrastruktur-Ressourcen, deren Apply oder Destroy immer erfordern sollte, dass Sie die Änderung benennen
* **Zusätzlicher Kontext**: Constraints für regulierte Industrien, Multi-Tenant-Infrastruktur oder Compliance-Anforderungen, die beeinflussen, was der Klassifizierer als riskant behandeln sollte

Die Einträge Internal package registry, Sensitive data locations & audiences, Sensitive remote targets und Protected IaC scopes erfordern Claude Code v2.1.195 oder später. Frühere Versionen lesen sie immer noch als einfachen Kontext, haben aber nicht die integrierten Regeln, die auf sie abzielen.

Eine nützliche Startvorlage: Füllen Sie die eingeklammerten Felder aus und entfernen Sie alle Zeilen, die nicht zutreffen.

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Organization: {COMPANY_NAME}. Primary use: {PRIMARY_USE_CASE, e.g. software development, infrastructure automation}",
      "Source control: {SOURCE_CONTROL, e.g. GitHub org github.example.com/acme-corp}",
      "Cloud provider(s): {CLOUD_PROVIDERS, e.g. AWS, GCP, Azure}",
      "Trusted cloud buckets: {TRUSTED_BUCKETS, e.g. s3://acme-builds, gs://acme-datasets}",
      "Trusted internal domains: {TRUSTED_DOMAINS, e.g. *.internal.example.com, api.example.com}",
      "Key internal services: {SERVICES, e.g. Jenkins at ci.example.com, Artifactory at artifacts.example.com}",
      "Additional context: {EXTRA, e.g. regulated industry, multi-tenant infrastructure, compliance requirements}"
    ]
  }
}
```

Je spezifischer der Kontext ist, den Sie geben, desto besser kann der Klassifizierer Routine-Internaloperationen von Exfiltrationversuchen unterscheiden.

Sie müssen nicht alles auf einmal ausfüllen. Ein angemessener Rollout: Beginnen Sie mit den Standardwerten und fügen Sie Ihre Source-Control-Organisation und wichtige interne Services hinzu, was die häufigsten falschen Positive wie das Pushen in Ihre eigenen Repos behebt. Fügen Sie als nächstes vertrauenswürdige Domains und Cloud-Buckets hinzu. Füllen Sie den Rest aus, wenn Blockierungen auftreten.

<h2 id="generate-environment-entries">
  Umgebungseinträge mit `/auto-mode-setup` generieren
</h2>

Führen Sie `/auto-mode-setup` aus, damit Claude Code `autoMode.environment`-Einträge und manchmal auch [Regeleinträge](#override-the-block-and-allow-rules) aus Ihrem Projekt und Ihren letzten Sitzungen darin entwirft. Wenn Sie den Entwurf akzeptieren, schreibt Claude Code ihn in `~/.claude/settings.json`.

<Note>
  `/auto-mode-setup` erfordert einen Pro-, Max- oder Team-Plan und Claude Code v2.1.228 oder später. Unter nativem Windows ist v2.1.233 oder später erforderlich. Sie können es nicht in einer [Cloud-Sitzung](/docs/de/claude-code-on-the-web) ausführen. Es benötigt auch [Feature-Flag-Abruf](/docs/de/env-vars#features-that-need-feature-flag-fetching), daher können Sie es nicht in einer Sitzung ausführen, in der Sie den Flag-Abruf deaktiviert haben.
</Note>

<h3 id="what-auto-mode-setup-reads">
  Was `/auto-mode-setup` liest
</h3>

Wenn `~/.claude/settings.json` bereits `autoMode`-Einträge enthält, fragt Claude Code zunächst, ob Sie Ihre Umgebungsliste erweitern oder ersetzen möchten, und behält die von Ihnen geschriebenen Regeln in jedem Fall bei. Claude Code fragt dann, wie Sie dieses Projekt nutzen, und bietet zwei optionale Scans an, bevor es etwas scannt. Beim Scan liest Claude Code immer diese Quellen:

* Die `CLAUDE.md`, `README.md`, Konfigurationsdateien und Git-Remotes dieses Projekts
* Ihre `autoMode`- und `permissions.allow`-Einstellungen
* Die Hosts, Buckets und Befehlsnamen aus den Befehlen, die Claude in Ihren letzten Sitzungen in diesem Projekt ausgeführt hat, niemals Ihre Nachrichten

Die zwei optionalen Scans fügen jeweils eine Quelle hinzu:

* Das erste Wort jedes Befehls in Ihrem Shell-Verlauf
* Die Remote-Hosts und Namen der Repositories unter Ihrem Home-Verzeichnis

<h3 id="review-and-save-the-draft">
  Entwurf überprüfen und speichern
</h3>

Claude Code scannt im Hintergrund und zeigt Ihnen dann den Entwurf. Sie akzeptieren oder verwerfen ihn als Ganzes, daher bearbeiten Sie `~/.claude/settings.json` danach, um einzelne Einträge anzupassen. Wenn Sie akzeptieren, schreibt Claude Code den Entwurf und gleicht ihn mit den Einstellungen ab, die Sie bereits haben:

* Claude Code schreibt die `environment`-Liste ohne `"$defaults"`, da der Entwurf die integrierten Einträge aufzählt, die unverändert geblieben sind
* Claude Code fügt `"$defaults"` in jede der `allow`-, `soft_deny`- und `hard_deny`-Listen ein, zu denen der Entwurf Einträge hinzufügt, es sei denn, Sie haben bereits eine `allow`-Liste ohne sie geschrieben, damit die [integrierten Regeln](#override-the-block-and-allow-rules), die Sie nicht ersetzt haben, weiterhin gelten
* Nach dem Speichern bietet Claude Code an, `permissions.allow`-Regeln in `~/.claude/settings.json` zu entfernen, die der Auto-Modus ignoriert, wie `Bash(*)`, oder die destruktive Befehle automatisch genehmigen

Führen Sie dann `claude auto-mode config` aus, um [das effektive Ergebnis zu sehen](#inspect-the-defaults-and-your-effective-config).

<h3 id="turn-off-auto-mode-setup">
  `/auto-mode-setup` ausschalten
</h3>

Sobald der Auto-Modus mehrere Aktionen blockiert hat und Sie immer noch keine `autoMode.environment`-Einträge haben, zeigt Claude Code am Ende einer Runde ein Dialogfeld mit dem Titel „Auto-Modus über Ihre Umgebung unterrichten?" an und bietet an, `/auto-mode-setup` für Sie auszuführen. Um das Angebot zu stoppen, aber den Befehl zu behalten, wählen Sie **Nicht mehr anzeigen** in diesem Dialogfeld.

Um sowohl den Befehl als auch das Angebot auszuschalten, fügen Sie diesen [`skillOverrides`](/docs/de/skills#override-skill-visibility-from-settings)-Eintrag zu `~/.claude/settings.json` hinzu:

```json theme={null}
{
  "skillOverrides": {
    "auto-mode-setup": "off"
  }
}
```

`/auto-mode-setup` ist ein integrierter Befehl und keine [gebündelte Skill](/docs/de/skills#bundled-skills), daher gilt dieser `skillOverrides`-Eintrag immer noch dafür, aber [`disableBundledSkills`](/docs/de/settings-reference#disablebundledskills) schaltet ihn nicht aus.

<h2 id="override-the-block-and-allow-rules">
  Override the block and allow rules
</h2>

Drei zusätzliche Felder ermöglichen es Ihnen, die integrierten Regellisten des Klassifizierers zu ersetzen:

* `autoMode.hard_deny`: bedingungslose Sicherheitsgrenzen
* `autoMode.soft_deny`: destruktive Aktionen, die Benutzerabsicht aufheben kann
* `autoMode.allow`: Ausnahmen zu Soft-Block-Regeln

Jedes ist ein Array von Prosabeschreibungen, das als natürlichsprachige Regeln gelesen wird. Für werkzeugmuster-basierte harte Blöcke, die vor dem Klassifizierer ausgeführt werden, verwenden Sie [`permissions.deny`](/docs/de/permissions).

Innerhalb des Klassifizierers funktioniert die Vorrangigkeit in vier Ebenen:

* `hard_deny`-Regeln blockieren bedingungslos. Benutzerabsicht und `allow`-Ausnahmen gelten nicht.
* `soft_deny`-Regeln blockieren als nächstes. Benutzerabsicht und `allow`-Ausnahmen können diese überschreiben.
* `allow`-Regeln überschreiben dann übereinstimmende `soft_deny`-Regeln als Ausnahmen.
* Explizite Benutzerabsicht überschreibt die verbleibenden weichen Blöcke: Wenn die Nachricht des Benutzers direkt und spezifisch die genaue Aktion beschreibt, die Claude ausführen wird, erlaubt der Klassifizierer es, auch wenn eine `soft_deny`-Regel zutrifft.

Allgemeine Anfragen zählen nicht als explizite Absicht. Claude zu bitten, das Repo „aufzuräumen", autorisiert keinen Force-Push, aber Claude zu bitten, „diesen Branch zu force-pushen", tut es.

Um zu lockern, fügen Sie zu `allow` hinzu, wenn der Klassifizierer wiederholt ein routinemäßiges Muster kennzeichnet, das die Standard-Ausnahmen nicht abdecken. Um zu straffen, fügen Sie zu `soft_deny` für destruktive Risiken hinzu, die spezifisch für Ihre Umgebung sind und die Standardwerte übersehen, oder zu `hard_deny` für Sicherheitsgrenzen, die niemals überschritten werden dürfen.

Um die integrierten Regeln beizubehalten und gleichzeitig Ihre eigenen hinzuzufügen, fügen Sie die Literalzeichenkette `"$defaults"` in das Array ein. Die Standardregeln werden an dieser Position eingefügt, sodass Ihre benutzerdefinierten Regeln vor oder nach ihnen stehen können, und Sie erhalten weiterhin Updates, wenn sich die integrierte Liste über Versionen hinweg ändert.

Das folgende Beispiel behält die Standardwerte in allen vier Listen bei und fügt organisationsspezifische Regeln zu jedem hinzu.

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Source control: github.example.com/acme-corp and all repos under it"
    ],
    "allow": [
      "$defaults",
      "Deploying to the staging namespace is allowed: staging is isolated from production and resets nightly",
      "Writing to s3://acme-scratch/ is allowed: ephemeral bucket with a 7-day lifecycle policy"
    ],
    "soft_deny": [
      "$defaults",
      "Never run database migrations outside the migrations CLI, even against dev databases",
      "Never modify files under infra/terraform/prod/: production infrastructure changes go through the review workflow"
    ],
    "hard_deny": [
      "$defaults",
      "Never send repository contents to third-party code-review APIs"
    ]
  }
}
```

<Danger>
  Das Festlegen eines der Felder `environment`, `allow`, `soft_deny` oder `hard_deny` ohne `"$defaults"` ersetzt die gesamte Standardliste für diesen Abschnitt. Wenn Sie ein Array ohne `"$defaults"` festlegen, verwerfen Sie die integrierten Regeln für diesen Abschnitt:

  * `soft_deny`: jede integrierte Soft-Block-Regel, einschließlich Force-Push, `curl | bash`, Produktionsbereitstellungen und Auto-Mode-Bypass
  * `hard_deny`: die integrierte Datenexfiltrations-Regel
</Danger>

Jeder Abschnitt wird unabhängig ausgewertet, sodass das Festlegen von `environment` allein die Standard-`allow`-, `soft_deny`- und `hard_deny`-Listen intakt lässt.

Lassen Sie `"$defaults"` nur weg, wenn Sie die vollständige Kontrolle über die Liste übernehmen möchten. Um dies sicher zu tun, führen Sie `claude auto-mode defaults` aus, um die integrierten Regeln auszudrucken, kopieren Sie sie in Ihre Einstellungsdatei, und überprüfen Sie dann jede Regel gegen Ihre eigene Pipeline und Risikotoleranz.

<h2 id="edit-rules-from-permissions">
  Bearbeitungsregeln aus `/permissions`
</h2>

Um Klassifiziererregeln anzuzeigen und zu bearbeiten, ohne eine Einstellungsdatei zu öffnen, führen Sie [`/permissions`](/docs/de/permissions#manage-permissions) aus und wählen Sie die Registerkarte **Auto mode** aus. Die Registerkarte erfordert Claude Code v2.1.246 oder später und wird nur angezeigt, wenn [Auto mode für Ihre Sitzung verfügbar](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) ist.

Die Registerkarte listet die Einträge `allow`, `soft_deny`, `hard_deny` und `environment` aus jedem der [Bereiche, die der Klassifizierer liest](#where-the-classifier-reads-configuration), auf und zeigt an, ob die integrierten Regeln für jeden Abschnitt wirksam sind. Claude Code zeigt Einträge aus [verwalteten Einstellungen](/docs/de/server-managed-settings) oder dem Flag `--settings` als schreibgeschützt an und speichert jede Änderung, die Sie auf der Registerkarte vornehmen, in `~/.claude/settings.json`. Auf der Registerkarte können Sie:

* Regeln in den Abschnitten `allow`, `soft_deny` und `hard_deny` hinzufügen, bearbeiten oder löschen. Wenn Sie die erste Regel zu einem Abschnitt hinzufügen, fügt Claude Code auch `"$defaults"` ein, damit die [integrierten Regeln](#override-the-block-and-allow-rules) wirksam bleiben.
* Die integrierten Regeln für `allow`, `soft_deny` oder `hard_deny` aus- oder wieder einschalten. Claude Code speichert die Auswahl, indem es `"$defaults"` in Ihrer Liste für diesen Abschnitt hinzufügt oder entfernt. Ein Abschnitt benötigt daher mindestens eine eigene Regel, bevor Sie die integrierten Regeln ausschalten können.
* Die Einträge `environment` als ein Dokument in Ihrem Editor bearbeiten. Wenn Sie noch keine `environment`-Einträge konfiguriert haben, fragt Claude Code zunächst, ob die integrierte Umgebung ersetzt werden soll, und öffnet dann den Editor mit dem vollständigen integrierten Text. Wenn Sie speichern, ersetzt Claude Code Ihr `autoMode.environment`-Array durch das Dokument. Fügen Sie die Zeile `"$defaults"` ein, um [die integrierten Einträge beizubehalten](#define-trusted-infrastructure).

<h2 id="route-all-shell-commands-through-the-classifier">
  Alle Shell-Befehle durch den Klassifizierer leiten
</h2>

Standardmäßig ermöglichen enge Bash- und PowerShell-Regeln wie `Bash(npm test)`, dass sie im Auto-Modus wirksam bleiben, und Claude Code löst sie auf, bevor der Klassifizierer ausgeführt wird, es sei denn, der Befehl trägt [pro-Befehl zulässige Domänen](/docs/de/sandboxing#per-command-allowed-domains-in-auto-mode). Claude Code setzt nur die breiten Regeln aus, die beliebige Code-Ausführung ermöglichen, wie `Bash(*)` oder Wildcards-Interpreter, zusammen mit jeder Regel, die [`Monitor`](/docs/de/tools-reference#monitor-tool) benennt, da Monitor-Befehle durch die Shell laufen. Dies bedeutet, dass eine enge Regel immer noch ein destruktives Argument durchlassen kann, ohne dass der Klassifizierer es sieht, beispielsweise einen Skriptpfad oder ein Flag, das das Präfix der Regel nicht vorgesehen hat.

Setzen Sie `autoMode.classifyAllShell` auf `true`, um jede Bash- und PowerShell-Erlaubnisregel auszusetzen, während der Auto-Modus aktiv ist, damit der Klassifizierer jeden Shell-Befehl unabhängig von Ihrer Erlaubnisliste evaluiert.

```json theme={null}
{
  "autoMode": {
    "classifyAllShell": true
  }
}
```

Dies tauscht Latenz gegen Abdeckung: Ein Befehl, den eine Erlaubnisregel sofort genehmigt hätte, wartet jetzt auf eine Klassifizierer-Entscheidung, und jeder Shell-Befehl zählt als ein Klassifizierer-Aufruf.

Die Einstellung gilt nur, während der Auto-Modus aktiv ist, und Ihre Erlaubnisregeln verhalten sich in anderen Berechtigungsmodi normal.

<Note>
  `autoMode.classifyAllShell` erfordert Claude Code v2.1.193 oder später. Frühere Versionen ignorieren den Schlüssel und setzen weiterhin enge Shell-Erlaubnisregeln in den Auto-Modus um.
</Note>

<h2 id="inspect-the-defaults-and-your-effective-config">
  Überprüfen Sie die Standardwerte und Ihre effektive Konfiguration
</h2>

Die Unterbefehle `claude auto-mode` helfen Ihnen, Ihre Konfiguration zu überprüfen, zu validieren und zurückzusetzen.

Geben Sie die integrierten Regeln `environment`, `allow`, `soft_deny` und `hard_deny` als JSON aus:

```bash theme={null}
claude auto-mode defaults
```

Um die vollständige Formulierung einer Regel ohne Piping durch `jq` zu lesen, übergeben Sie `--label` mit dem Anfang des Regelbezeichners, z. B. `claude auto-mode defaults --label 'Git Destructive'`. Der Abgleich ist ein Präfix ohne Berücksichtigung der Groß-/Kleinschreibung für jeden Regelbezeichner, und Abschnitte ohne Übereinstimmung werden als leere Listen ausgegeben. Erfordert Claude Code v2.1.208 oder später.

Geben Sie aus, was der Klassifizierer tatsächlich als JSON verwendet, mit Ihren Einstellungen, wo festgelegt, und Standardwerten andernfalls:

```bash theme={null}
claude auto-mode config
```

Sowohl `defaults` als auch `config` geben die vier Regellisten als ein einzelnes JSON-Objekt aus, wobei jede Regel als Prosatext dargestellt wird. Dies ist ein verkürztes Beispiel:

```json theme={null}
{
  "allow": [
    ...
    "Test Artifacts: Hardcoded test API keys, placeholder credentials in examples, or hardcoding test cases. Placeholder means authored as a placeholder — a file or value copied from a real secret or sensitive path is never a test artifact (see Sensitive-Source Provenance).",
    ...
  ],
  "soft_deny": [
    "Git Destructive [named+specifics — **must name:** the destructive operation and its target]: Force pushing (`git push --force`), deleting remote branches, tags, or releases, or rewriting remote history. Also `git commit --amend` when the commit being rewritten is not the agent's own unpushed work: either no prior `git commit` is visible (HEAD pre-dates the session), or a `git push` of the current branch is visible after the most recent commit (it has been pushed). Clears when the user asked to amend/reword/fixup, or when it is a message-only reword (`--amend -m …`, nothing newly staged) of a commit the agent visibly created this session.",
    ...
  ],
  "hard_deny": [...],
  "environment": [
    ...
    "**Trusted repo**: The git repository the agent started in (its working directory) and its configured remote(s). When the repo's public/private visibility is given — by the Repository visibility entry or the user's own message — use it to scope what is OK to commit or push there: confidential material is fine in a private repo; in a public one, only that repo's own work is — and content ported, repointed, or first read from outside this session's repo is not its own work, whoever directed the port. Visibility scopes confidential material only: secrets and sensitive data (personal & entrusted) are never cleared into any repo by its visibility (see Definitions).",
    ...
  ]
}
```

Erhalten Sie KI-Feedback zu Ihren benutzerdefinierten Regeln `allow`, `soft_deny` und `hard_deny`:

```bash theme={null}
claude auto-mode critique
```

Führen Sie `claude auto-mode config` nach dem Speichern Ihrer Einstellungen aus, um zu bestätigen, dass die effektiven Regeln Ihren Erwartungen entsprechen, wobei `"$defaults"` an Ort und Stelle erweitert wird. Wenn Sie benutzerdefinierte Regeln geschrieben haben, überprüft `claude auto-mode critique` diese und kennzeichnet Einträge, die mehrdeutig, redundant oder wahrscheinlich zu falsch positiven Ergebnissen führen.

Um Ihre Anpassungen zu verwerfen und zu den integrierten Standardwerten zurückzukehren, führen Sie den Reset-Unterbefehl aus. Er erfordert Claude Code v2.1.212 oder später und entfernt den Abschnitt `autoMode` aus Ihrer Benutzereinstellungsdatei:

```bash theme={null}
claude auto-mode reset
```

Der Befehl fasst zusammen, was entfernt wird, und fragt `Reset auto mode configuration to defaults?` ab, bevor er schreibt. Übergeben Sie `--yes`, um die Bestätigung zu überspringen. Reset ändert nur `~/.claude/settings.json`: Regeln für `autoMode` aus [verwalteten Einstellungen](/docs/de/server-managed-settings) oder dem Flag `--settings` gelten weiterhin.

<h2 id="review-denials">
  Ablehnungen überprüfen
</h2>

Um Aktionen zu überprüfen und erneut zu versuchen, die der Auto-Modus-Klassifizierer abgelehnt hat, öffnen Sie `/permissions` und wählen Sie die Registerkarte **Kürzlich abgelehnt**, in der Claude Code jede Ablehnung aufzeichnet. Drücken Sie `r` auf einer abgelehnten Aktion, um sie zum Wiederholen zu markieren: Wenn Sie den Dialog beenden, sendet Claude Code eine Nachricht, die dem Modell mitteilt, dass es diesen Tool-Aufruf wiederholen darf, und setzt das Gespräch fort.

Wenn der Klassifizierer [keine Entscheidung zur Aktion trifft](/docs/de/errors#auto-mode-cannot-determine-the-safety-of-an-action), weil eine Sicherheitsprüfung, die vom Auto-Modus getrennt ist, die Anfrage des Klassifizierers selbst oder dessen Antwort nicht analysiert hat, lehnt Claude Code die Aktion ab, ohne sie unter **Kürzlich abgelehnt** aufzuzeichnen. Der verlinkte Fehlereintrag behandelt, was Claude mitgeteilt wird, und wie die Aktion ausgeführt wird, falls Sie sie benötigen.

<h3 id="fix-a-denial-with-an-allow-rule-an-environment-entry-or-a-retry">
  Eine Ablehnung mit einer Erlaubnisregel, einem Umgebungseintrag oder einem Wiederholungsversuch beheben
</h3>

Um zu sehen, was der Klassifizierer blockiert hat, suchen Sie den Tool-Aufruf im Gespräch. Wenn der Aufruf verkürzt oder in eine Zusammenfassungszeile wie `Ran 3 shell commands` eingeklappt angezeigt wird, drücken Sie `Ctrl+O`, um den [Transcript-Viewer](/docs/de/interactive-mode#transcript-viewer) zu öffnen, der ihn erweitert.

Zwei weitere Stellen auf dem Bildschirm, die Ablehnungen melden, lassen den Befehl oder die URL aus: Die Benachrichtigung neben dem Eingabefeld, z. B. `bash denied by auto mode · [Data Exfiltration] · /permissions`, gibt das Tool und den Grund an, und die Registerkarte **Kürzlich abgelehnt** listet einen Shell-Befehl nach der Beschreibung auf, die Claude dafür geschrieben hat. Um die genaue Eingabe dieser Ablehnungen programmgesteuert zu erfassen, fügen Sie einen [`PermissionDenied`-Hook](/docs/de/hooks#permissiondenied) hinzu, der sie als `tool_input` empfängt.

Der Text unter dem Aufruf teilt Ihnen mit, ob es etwas zu beheben gibt. Text, der ein Problem mit dem Klassifizierer selbst meldet, z. B. ein Modell, das `is temporarily unavailable` ist, oder ein Klassifizierer-Fehler, bedeutet, dass Claude Code den Aufruf ohne endgültige Entscheidung des Klassifizierers blockiert hat; siehe [Auto-Modus kann die Sicherheit einer Aktion nicht bestimmen](/docs/de/errors#auto-mode-cannot-determine-the-safety-of-an-action) für weitere Informationen. Andernfalls bedeutet eine Zeile, die `Denied by auto mode classifier` mit einem Grund wie `[Production Deploy]` oder `Blocked by classifier` liest, dass der Klassifizierer den Aufruf für unsicher befunden hat. Wählen Sie daher die Behebung aus dem aus, was der Aufruf erreichen oder tun wollte:

* Ein Ziel, das Claude während der gesamten Aufgabe benötigt, z. B. eine Paket-Registry, eine interne Domäne oder einen Repository-Host: Fügen Sie es zu `autoMode.environment` hinzu.
* Ein Befehl, den Sie von nun an ohne Überprüfung ausführen möchten: Fügen Sie eine `allow`-Regel hinzu.
* Eine einmalige Aktion, die Sie beabsichtigt haben: Geben Sie diese Absicht in Ihrer nächsten Nachricht an und lassen Sie Claude erneut versuchen.

Sie können den Umgebungseintrag oder die `allow`-Regel aus der Registerkarte [**Auto-Modus**](#edit-rules-from-permissions) des Dialogs `/permissions` hinzufügen.

In den meisten Sitzungen benennt der Grund die Regel, die der Klassifizierer abgeglichen hat, in eckigen Klammern, z. B. `[Data Exfiltration]` oder `[Production Deploy]`, und einige Sitzungen führen ein Klassifizierer-Modell aus, das eine kurze Erklärung hinzufügt. Claude Code wählt das Klassifizierer-Modell aus, daher ist es nicht etwas, das Sie konfigurieren können, welche Form Sie sehen.

<h3 id="fix-repeated-denials">
  Wiederholte Ablehnungen beheben
</h3>

Wiederholte Ablehnungen für dasselbe Ziel bedeuten normalerweise, dass dem Klassifizierer der Kontext fehlt. Fügen Sie dieses Ziel zu `autoMode.environment` hinzu, oder [führen Sie `/auto-mode-setup`](#generate-environment-entries) aus, damit Claude Code die Einträge entwirft, und führen Sie dann `claude auto-mode config` aus, um zu bestätigen, dass die Änderung wirksam wurde.

Um programmgesteuert auf Ablehnungen zu reagieren, verwenden Sie den [`PermissionDenied`-Hook](/docs/de/hooks#permissiondenied).

<h2 id="see-also">
  Siehe auch
</h2>

* [Berechtigungsmodi](/docs/de/permission-modes#eliminate-prompts-with-auto-mode): was Auto-Modus ist, was er standardmäßig blockiert, und welche Sitzungen darin starten
* [Verwaltete Einstellungen](/docs/de/server-managed-settings): stellen Sie `autoMode`-Konfiguration in Ihrer Organisation bereit
* [Berechtigungen](/docs/de/permissions): Zulassen-, Fragen- und Ablehnungsregeln, die vor dem Klassifizierer gelten
* [Alle Einstellungen](/docs/de/settings-reference#automode): jeden Einstellungsschlüssel, einschließlich `autoMode`
