> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Auto-Modus konfigurieren

> Teilen Sie dem Auto-Modus-Klassifizierer mit, welche Repos, Buckets und Domains Ihre Organisation vertraut. Legen Sie den Umgebungskontext fest, überschreiben Sie die Standard-Block- und Allow-Regeln, und überprüfen Sie Ihre effektive Konfiguration mit den Auto-Modus-CLI-Unterbefehlen.

[Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) ermöglicht es Claude Code, ohne routinemäßige Berechtigungsaufforderungen zu laufen, indem Werkzeugaufrufe durch einen Klassifizierer geleitet werden, der alles blockiert, das irreversibel, destruktiv oder außerhalb Ihrer Umgebung ausgerichtet ist. Deny- und explizite Ask-Regeln werden vor dem Klassifizierer ausgewertet und blockieren oder fordern weiterhin auf. Verwenden Sie den `autoMode`-Einstellungsblock, um diesem Klassifizierer mitzuteilen, welche Repos, Buckets und Domains Ihre Organisation vertraut, damit er routinemäßige interne Operationen nicht mehr blockiert.

<Note>
  Auto-Modus ist für alle Benutzer auf jedem Anbieter verfügbar, einschließlich der Anthropic API, Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry und angemeldeter [Claude Apps Gateway](/docs/de/claude-apps-gateway)-Sitzungen. Wenn Claude Code Auto-Modus als nicht verfügbar für Ihr Konto meldet, überprüfen Sie die [vollständigen Anforderungen](/docs/de/permission-modes#eliminate-prompts-with-auto-mode), die auch die unterstützten Modelle und die Owner-Aktivierung in Team- und Enterprise-Plänen abdecken. In v2.1.158 bis v2.1.206 erforderte Auto-Modus auf Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry und Claude Apps Gateway-Sitzungen das Setzen von `CLAUDE_CODE_ENABLE_AUTO_MODE=1`; v2.1.207 entfernte die Anforderung.
</Note>

Standardmäßig vertraut der Klassifizierer nur dem Arbeitsverzeichnis und den konfigurierten Remotes des aktuellen Repos. Aktionen wie das Pushen zu Ihrer Unternehmens-Quellcode-Org oder das Schreiben in einen Team-Cloud-Bucket werden blockiert, bis Sie sie zu `autoMode.environment` hinzufügen.

Informationen zum Aktivieren des Auto-Modus und was er standardmäßig blockiert, finden Sie unter [Berechtigungsmodi](/docs/de/permission-modes#eliminate-prompts-with-auto-mode). Diese Seite ist die Konfigurationsreferenz.

Diese Seite behandelt, wie Sie:

* [Einen menschlichen Checkpoint](#common-boundaries) für Pushes und Pull Requests mit `permissions.ask` hinzufügen
* [Wählen Sie, wo Sie Regeln festlegen](#where-the-classifier-reads-configuration) über CLAUDE.md, Benutzereinstellungen und verwaltete Einstellungen
* [Definieren Sie vertrauenswürdige Infrastruktur](#define-trusted-infrastructure) mit `autoMode.environment`
* [Überschreiben Sie die Block- und Allow-Regeln](#override-the-block-and-allow-rules), wenn die Standardwerte nicht zu Ihrer Pipeline passen
* [Leiten Sie alle Shell-Befehle durch den Klassifizierer](#route-all-shell-commands-through-the-classifier) mit `autoMode.classifyAllShell`
* [Überprüfen Sie Ihre effektive Konfiguration](#inspect-the-defaults-and-your-effective-config) mit den `claude auto-mode`-Unterbefehlen
* [Überprüfen Sie Ablehnungen](#review-denials), damit Sie wissen, was Sie als Nächstes hinzufügen müssen

<h2 id="common-boundaries">
  Häufige Grenzen
</h2>

Der Auto-Modus ermöglicht standardmäßig Pushes zu Ihrem Arbeitszweig, regelmäßige Pushes zum Standard-Repository-Zweig und die Erstellung von Pull Requests. Der Klassifizierer blockiert einen Push nur, wenn dieser ein Risiko birgt, z. B. ein Force Push oder Inhalte, die eine von Ihnen eingerichtete Überprüfung umgehen. Wenn Sie vor jedem Push oder Pull Request einen menschlichen Checkpoint wünschen, fügen Sie Berechtigungsregeln hinzu: Die folgenden Rezepte halten den Auto-Modus für alles andere aktiviert.

Der direkteste Mechanismus ist [`permissions.ask`](/docs/de/permissions#permission-rule-syntax). Inhaltsgebundene Ask-Regeln wie die folgenden werden vor dem Klassifizierer ausgewertet und erzwingen immer eine Berechtigungsaufforderung, auch im Auto-Modus, da eine explizite Ask-Regel Ihre ausdrückliche Absicht ist, für diese Aktion aufgefordert zu werden. Fügen Sie die Regeln in Ihren [Einstellungen](/docs/de/settings#settings-files) hinzu:

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

Wählen Sie den Mechanismus, der der Festigkeit der erforderlichen Grenze entspricht:

| Grenze                             | Mechanismus                                                               | Verhalten im Auto-Modus                                                                                                                                                                                                                                                          |
| :--------------------------------- | :------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Aufforderung vor der Aktion        | `permissions.ask`                                                         | Fordert immer für inhaltsgebundene Regeln wie das obige Rezept auf. Der Klassifizierer kann eine übereinstimmende Aktion nicht automatisch genehmigen.                                                                                                                           |
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
  Define trusted infrastructure
</h2>

Für die meisten Organisationen ist `autoMode.environment` das einzige Feld, das Sie festlegen müssen. Es teilt dem Klassifizierer mit, welche Repos, Buckets und Domains vertrauenswürdig sind: Der Klassifizierer verwendet es, um zu entscheiden, was „extern" bedeutet, daher ist jedes Ziel, das nicht aufgelistet ist, ein potenzielles Exfiltrationsziel.

Ab Claude Code v2.1.198 druckt `claude auto-mode defaults` drei Arten von Umgebungseinträgen. Versionen vor v2.1.195 drucken nur die ersten fünf Vertrauens-Slots.

* **Kontext-Slots**: beschreiben Ihre Organisation, Ihren Stack und Ihre Sicherheitslage, damit der Klassifizierer die anderen Regeln in Ihrem Kontext liest. Im Gegensatz zu den anderen beiden Arten haben Kontext-Slots keine eigenen Regeln, die auf sie abzielen. Jeder wird standardmäßig auf `Keine konfiguriert` oder auf die nächste aufgelistete konservative Annahme gesetzt:
  * **Organisation**
  * **Primäre Verwendung von Claude Code**: wird standardmäßig auf Softwareentwicklung gesetzt
  * **Cloud-Anbieter**
  * **Repository-Sichtbarkeit**: Ein Repository wird als privat angenommen, es sei denn, sein Remote-Host und Name deuten anderweitig darauf hin, oder eine Sichtbarkeitsprüfung früher in der Konversation, die der Klassifizierer liest, zeigt, dass es öffentlich ist. Der Klassifizierer liest Ihre Nachrichten und die Befehle, die Claude ausführt, nicht deren Ausgabe, daher muss der Beweis etwas sein, das er lesen kann, wie Ihre eigene Nachricht, die das Repository als öffentlich benennt; die Ausgabe eines `gh repo view` allein erreicht ihn nicht. Die Transkript-Evidenz-Überprüfung erfordert Claude Code v2.1.200 oder später
  * **Interne Freigabe / Snippet-Hosting**: öffentliche Paste- und Gist-Services werden als außerhalb der Vertrauensgrenze behandelt, bis Sie einen benennen
  * **Organisationsspezifische CLIs**
  * **Secrets-Verwaltung**
  * **Standard- / geschützte Branches**: `main` und `master` werden als geschützt behandelt, bis Sie andere benennen
  * **CI/CD-Bereitstellungsziele**
  * **Netzwerk-Lage**
  * **Geschützte Bereitstellungs-Namespaces / Umgebungen**: fällt auf die Heuristik für sensible Remote-Ziele zurück, bis Sie sie benennen
  * **Datenspeicherung / Klassifizierungsaufhebung**
* **Vertrauens-Slots**: benennen, was der Klassifizierer als innerhalb Ihrer Grenze behandelt. Die Slots sind Vertrauenswürdiges Repo, Quellcode-Verwaltung, Vertrauenswürdige interne Domains, Vertrauenswürdige Cloud-Buckets, Wichtige interne Services und Interne Paket-Registry. Die Repo- und Quellcode-Verwaltungseinträge werden standardmäßig auf das Arbeits-Repo und seine konfigurierten Remotes gesetzt. Jeder andere Vertrauens-Slot wird standardmäßig auf `Keine konfiguriert` gesetzt, daher wird nichts anderes vertraut, bis Sie es hinzufügen. Die Sichtbarkeit eines Repositorys bezieht sich nur auf vertrauliches Material: Ein privates Repository ist ein akzeptables Ziel für vertrauliches Material, aber das Privatmachen eines Repositorys löscht niemals Geheimnisse oder persönliche oder anvertraute Daten darin, und der Klassifizierer behandelt Inhalte, die von außerhalb des Arbeits-Repositorys portiert, umgeleitet oder zuerst gelesen werden, als nicht die eigene Arbeit dieses Repositorys. Diese Scoping-Anforderung erfordert Claude Code v2.1.203 oder später.
* **Sensitivitäts-Slots**: benennen, was die Schutzregeln als hochriskant behandeln. Die Slots sind Sensible Datenspeicherorte & Zielgruppen, Sensible Remote-Ziele und Geschützte IaC-Bereiche. Jeder wird standardmäßig auf eine breite Heuristik gesetzt, wie z. B. die Behandlung eines Hosts oder Namespace, dessen Name `prod` oder `production` trägt, als sensibles Remote-Ziel, daher sind die Schutzregeln aktiv, bevor Sie etwas konfigurieren. Das Benennen konkreter Ziele in einem Sensitivitäts-Slot lässt diese Regeln auf die benannten Ziele anstelle der Heuristik angewendet werden.

Um Ihre eigenen Einträge neben den Standardwerten hinzuzufügen, fügen Sie die Literalzeichenfolge `"$defaults"` in das Array ein. Die Standard-Einträge werden an dieser Position eingefügt, sodass Ihre benutzerdefinierten Einträge vor oder nach ihnen stehen können.

Das folgende Beispiel behält die Standard-Einträge bei und fügt die Repos, Buckets, Domains und Services einer Organisation hinzu.

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

Einträge sind Prosa, keine Regex oder Tool-Muster. Der Klassifizierer liest sie als natürlichsprachige Regeln. Schreiben Sie sie so, wie Sie Ihre Infrastruktur einem neuen Ingenieur beschreiben würden. Ein gründlicher Umgebungsabschnitt deckt Folgendes ab:

* **Organisation**: Ihr Unternehmensname und wofür Claude Code hauptsächlich verwendet wird, wie Softwareentwicklung, Infrastrukturautomatisierung oder Datentechnik
* **Quellcode-Verwaltung**: jede GitHub-, GitLab- oder Bitbucket-Organisation, zu der Ihre Entwickler pushen
* **Cloud-Anbieter und vertrauenswürdige Buckets**: Bucket-Namen oder Präfixe, aus denen und in die Claude lesen und schreiben können sollte
* **Vertrauenswürdige interne Domains**: Hostnamen für APIs, Dashboards und Services in Ihrem Netzwerk, wie `*.internal.example.com`
* **Wichtige interne Services**: CI, Artifact-Registries, interne Paketindizes, Incident-Tools
* **Interne Paket-Registry**: die private npm-, PyPI- oder andere Registry, durch die Installationen laufen sollten, damit Installationen, die sie für eine öffentliche Registry umgehen, blockiert werden
* **Sensible Datenspeicherorte & Zielgruppen**: die Buckets, Datenbanken oder Pfade, die persönliche Daten, vertrauliche Geschäftsdaten, Anmeldedaten, regulierte Daten oder ähnlich sensibles Material enthalten, und die Zielgruppen, mit denen Daten an jedem Speicherort geteilt werden können, damit der Klassifizierer diese Speicherorte schützt, anstatt aus dem Inhalt zu erraten. Claude Code v2.1.195 bis v2.1.197 nennt diesen Eintrag PII / Standorte mit regulierten Daten und deckt nur Speicherorte ab, die persönliche oder regulierte Daten enthalten, ohne die Zielgruppen-Dimension
* **Sensible Remote-Ziele**: die Namespaces, Hosts oder Container, die als Produktion zählen, damit Remote-Shells und Port-Forwards in sie Ihre explizite Genehmigung benötigen
* **Geschützte IaC-Bereiche**: die Infrastruktur-Ressourcen, deren Apply oder Destroy immer erfordern sollte, dass Sie die Änderung benennen
* **Zusätzlicher Kontext**: Einschränkungen in regulierten Branchen, Multi-Tenant-Infrastruktur oder Compliance-Anforderungen, die beeinflussen, was der Klassifizierer als riskant behandeln sollte

Die Einträge für Interne Paket-Registry, Sensible Datenspeicherorte & Zielgruppen, Sensible Remote-Ziele und Geschützte IaC-Bereiche erfordern Claude Code v2.1.195 oder später. Frühere Versionen lesen sie immer noch als einfachen Kontext, haben aber nicht die integrierten Regeln, die auf sie abzielen.

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

Je spezifischer der Kontext, den Sie geben, desto besser kann der Klassifizierer routinemäßige interne Operationen von Exfiltrationversuchen unterscheiden.

Sie müssen nicht alles auf einmal ausfüllen. Ein angemessener Rollout: Beginnen Sie mit den Standardwerten und fügen Sie Ihre Quellcode-Verwaltungsorganisation und wichtige interne Services hinzu, was die häufigsten falschen Positive wie das Pushen zu Ihren eigenen Repos behebt. Fügen Sie als Nächstes vertrauenswürdige Domains und Cloud-Buckets hinzu. Füllen Sie den Rest aus, wenn Blockierungen auftreten.

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

<h2 id="route-all-shell-commands-through-the-classifier">
  Route all shell commands through the classifier
</h2>

Standardmäßig werden enge Bash- und PowerShell-Allow-Regeln wie `Bash(npm test)` in den Auto-Modus übernommen und gelöst, bevor der Klassifizierer ausgeführt wird. Der Auto-Modus setzt nur die breiten Regeln aus, die beliebige Code-Ausführung gewähren, wie `Bash(*)` oder Wildcard-Interpreter. Dies bedeutet, dass eine enge Regel immer noch ein destruktives Argument durchlassen kann, ohne dass der Klassifizierer es sieht, z. B. einen Skriptpfad oder ein Flag, das das Präfix der Regel nicht antizipiert hat.

Setzen Sie `autoMode.classifyAllShell` auf `true`, um jede Bash- und PowerShell-Allow-Regel auszusetzen, während der Auto-Modus aktiv ist, damit der Klassifizierer jeden Shell-Befehl unabhängig von Ihrer Allow-Liste auswertet.

```json theme={null}
{
  "autoMode": {
    "classifyAllShell": true
  }
}
```

Dies tauscht Latenz gegen Abdeckung: Ein Befehl, den eine Allow-Regel sofort genehmigt hätte, wartet jetzt auf eine Klassifizierer-Entscheidung, und jeder Shell-Befehl zählt als ein Klassifizierer-Aufruf.

Die Einstellung gilt nur, während der Auto-Modus aktiv ist, und Ihre Allow-Regeln verhalten sich in anderen Berechtigungsmodi normal.

<Note>
  `autoMode.classifyAllShell` erfordert Claude Code v2.1.193 oder später. Frühere Versionen ignorieren den Schlüssel und setzen enge Shell-Allow-Regeln weiterhin in den Auto-Modus um.
</Note>

<h2 id="inspect-the-defaults-and-your-effective-config">
  Überprüfen Sie die Standardwerte und Ihre effektive Konfiguration
</h2>

Drei CLI-Unterbefehle helfen Ihnen beim Überprüfen und Validieren Ihrer Konfiguration.

Drucken Sie die integrierten `environment`-, `allow`-, `soft_deny`- und `hard_deny`-Regeln als JSON:

```bash theme={null}
claude auto-mode defaults
```

Um die vollständige Formulierung einer Regel ohne Piping durch `jq` zu lesen, übergeben Sie `--label` mit dem Anfang des Regelbezeichners, z. B. `claude auto-mode defaults --label 'Git Destructive'`. Der Abgleich ist ein Präfix ohne Berücksichtigung der Groß-/Kleinschreibung für jeden Regelbezeichner, und Abschnitte ohne Übereinstimmung werden als leere Listen gedruckt. Erfordert Claude Code v2.1.208 oder später.

Drucken Sie, was der Klassifizierer tatsächlich als JSON verwendet, mit Ihren Einstellungen angewendet, wo festgelegt, und Standardwerte andernfalls:

```bash theme={null}
claude auto-mode config
```

Erhalten Sie KI-Feedback zu Ihren benutzerdefinierten `allow`-, `soft_deny`- und `hard_deny`-Regeln:

```bash theme={null}
claude auto-mode critique
```

Führen Sie `claude auto-mode config` nach dem Speichern Ihrer Einstellungen aus, um zu bestätigen, dass die effektiven Regeln das sind, was Sie erwarten, mit `"$defaults"` an Ort und Stelle erweitert. Wenn Sie benutzerdefinierte Regeln geschrieben haben, überprüft `claude auto-mode critique` diese und kennzeichnet Einträge, die mehrdeutig, redundant oder wahrscheinlich zu falschen Positiven führen.

Wenn Sie eine integrierte Regel entfernen oder umschreiben müssen, anstatt sie hinzuzufügen, speichern Sie die Ausgabe von `claude auto-mode defaults` in einer Datei, bearbeiten Sie die Listen und fügen Sie das Ergebnis in Ihre Einstellungsdatei anstelle von `"$defaults"` ein.

<h2 id="review-denials">
  Review denials
</h2>

Wenn der Auto-Modus einen Tool-Aufruf ablehnt, wird die Ablehnung in `/permissions` unter der Registerkarte „Kürzlich abgelehnt" aufgezeichnet. Drücken Sie `r` auf einer abgelehnten Aktion, um sie zum Wiederholen zu markieren: Wenn Sie das Dialogfeld beenden, sendet Claude Code eine Nachricht, die dem Modell mitteilt, dass es diesen Tool-Aufruf wiederholen kann, und setzt das Gespräch fort.

In Claude Code v2.1.193 und später wird der Grund des Klassifizierers für jede Ablehnung neben dem blockierten Tool-Aufruf im Transkript, in der Ablehnungsbenachrichtigung und unter jedem Eintrag auf der Registerkarte „Kürzlich abgelehnt" angezeigt. Verwenden Sie den Grund, um zu entscheiden, ob die Behebung ein `environment`-Eintrag, eine `allow`-Ausnahme oder ein Wiederholen mit expliziter Absicht in Ihrer nächsten Nachricht ist.

Wiederholte Ablehnungen für dasselbe Ziel bedeuten normalerweise, dass dem Klassifizierer der Kontext fehlt. Fügen Sie dieses Ziel zu `autoMode.environment` hinzu, führen Sie dann `claude auto-mode config` aus, um zu bestätigen, dass es wirksam wurde.

Um programmatisch auf Ablehnungen zu reagieren, verwenden Sie den [`PermissionDenied`-Hook](/docs/de/hooks#permissiondenied).

<h2 id="see-also">
  See also
</h2>

* [Berechtigungsmodi](/docs/de/permission-modes#eliminate-prompts-with-auto-mode): Was Auto-Modus ist, was er standardmäßig blockiert, und wie man ihn aktiviert
* [Verwaltete Einstellungen](/docs/de/server-managed-settings): Stellen Sie die `autoMode`-Konfiguration in Ihrer Organisation bereit
* [Berechtigungen](/docs/de/permissions): Allow-, Ask- und Deny-Regeln, die vor dem Klassifizierer gelten
* [Einstellungen](/docs/de/settings): Die vollständige Einstellungsreferenz, einschließlich des `autoMode`-Schlüssels
