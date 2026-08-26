> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Prompts nach Zeitplan ausführen

> Verwenden Sie /loop und die Cron-Planungstools, um Prompts wiederholt auszuführen, den Status abzurufen oder einmalige Erinnerungen innerhalb einer Claude Code-Sitzung zu setzen.

Geplante Aufgaben ermöglichen es Claude, einen Prompt automatisch in regelmäßigen Abständen erneut auszuführen. Verwenden Sie sie, um eine Bereitstellung abzurufen, einen PR zu überwachen, einen langwierigen Build zu überprüfen oder sich später in der Sitzung an etwas zu erinnern. Um auf Ereignisse zu reagieren, während sie geschehen, anstatt abzurufen, siehe [Kanäle](/docs/de/channels): Ihr CI kann den Fehler direkt in die Sitzung übertragen. Um die Sitzung Zug um Zug weiterarbeiten zu lassen, bis eine Bedingung erfüllt ist, anstatt in einem Intervall, siehe [`/goal`](/docs/de/goal).

Aufgaben sind sitzungsbezogen: Sie existieren im aktuellen Gespräch und werden beendet, wenn Sie ein neues starten. Das Fortsetzen mit `--resume` oder `--continue` bringt alle Aufgaben zurück, die nicht [abgelaufen sind](#seven-day-expiry): eine wiederkehrende Aufgabe, die in den letzten 7 Tagen erstellt wurde, oder eine einmalige Aufgabe, deren geplante Zeit noch nicht vergangen ist. Für Planung, die unabhängig von einer Sitzung bestehen bleibt, verwenden Sie [Routinen](/docs/de/routines), um eine Routine auf von Anthropic verwalteter Infrastruktur zu erstellen, richten Sie eine [Desktop-geplante Aufgabe](/docs/de/desktop-scheduled-tasks) ein, oder verwenden Sie [GitHub Actions](/docs/de/github-actions).

<h2 id="compare-scheduling-options">
  Vergleichen Sie Planungsoptionen
</h2>

Claude Code offers three ways to schedule recurring or one-off work:

|                            | [Cloud](/docs/en/routines)               | [Desktop](/docs/en/desktop-scheduled-tasks) | [`/loop`](/docs/en/scheduled-tasks)      |
| :------------------------- | :---------------------------------- | :------------------------------------- | :---------------------------------- |
| Runs on                    | Cloud, Anthropic-managed by default | Your machine                           | Your machine                        |
| Requires machine on        | No                                  | Yes                                    | Yes                                 |
| Requires open session      | No                                  | No                                     | Yes                                 |
| Persistent across restarts | Yes                                 | Yes                                    | Restored on `--resume` if unexpired |
| Access to local files      | No (fresh clone)                    | Yes                                    | Yes                                 |
| MCP servers                | Connectors configured per task      | [Config files](/docs/en/mcp) and connectors | Inherits from session               |
| Permission prompts         | No (runs autonomously)              | Configurable per task                  | Inherits from session               |
| Customizable schedule      | Via `/schedule` in the CLI          | Yes                                    | Yes                                 |
| Minimum interval           | 1 hour                              | 1 minute                               | 1 minute                            |

<Tip>
  Use **cloud tasks** for work that should run reliably without your machine. Use **Desktop tasks** when you need access to local files and tools. Use **`/loop`** for quick polling during a session.
</Tip>

<h2 id="run-a-prompt-repeatedly-with-/loop">
  Führen Sie einen Prompt wiederholt mit /loop aus
</h2>

Die `/loop` [bundled skill](/docs/de/commands) ist der schnellste Weg, um einen Prompt wiederholt auszuführen, während die Sitzung offen bleibt. Sowohl das Intervall als auch der Prompt sind optional, und das, was Sie bereitstellen, bestimmt, wie sich die Schleife verhält.

| Was Sie bereitstellen     | Beispiel                    | Was passiert                                                                                                       |
| :------------------------ | :-------------------------- | :----------------------------------------------------------------------------------------------------------------- |
| Intervall und Prompt      | `/loop 5m check the deploy` | Ihr Prompt läuft nach einem [festen Zeitplan](#run-on-a-fixed-interval)                                            |
| Nur Prompt                | `/loop check the deploy`    | Ihr Prompt läuft in einem [Intervall, das Claude wählt](#let-claude-choose-the-interval) bei jeder Iteration       |
| Nur Intervall oder nichts | `/loop`                     | Der [integrierte Wartungs-Prompt](#run-the-built-in-maintenance-prompt) läuft, oder Ihr `loop.md`, falls vorhanden |

Sie können auch einen Skill als Prompt übergeben, zum Beispiel `/loop 20m /review-pr 1234`, um diesen Skill bei jeder Iteration erneut auszuführen. Ab v2.1.196 führt ein geplanter Auslöser nur Skills aus, die Claude [selbstständig aufrufen darf](/docs/de/skills#control-who-invokes-a-skill). Die folgenden erreichen Claude als einfacher Text statt auszuführen:

* integrierte Befehle wie `/permissions`, `/model` oder `/clear`
* Skills, die mit [`disable-model-invocation: true`](/docs/de/skills#frontmatter-reference) gekennzeichnet sind
* Skills, die Claude durch eine [`skillOverrides`](/docs/de/skills#override-skill-visibility-from-settings)-Einstellung oder eine `Skill` [deny rule](/docs/de/skills#restrict-claude’s-skill-access) vorenthalten werden
* [MCP prompts](/docs/de/mcp#use-mcp-prompts-as-commands) wie `/mcp__github__list_prs`; Skills, die ein MCP-Server bereitstellt, laufen weiterhin

<h3 id="run-on-a-fixed-interval">
  Führen Sie nach einem festen Intervall aus
</h3>

Wenn Sie ein Intervall angeben, konvertiert Claude es in einen Cron-Ausdruck, plant den Job und bestätigt die Häufigkeit und die Job-ID.

```text theme={null}
/loop 5m check if the deployment finished and tell me what happened
```

Das Intervall kann dem Prompt als einfaches Token wie `30m` vorangehen oder als Klausel wie `every 2 hours` folgen. Unterstützte Einheiten sind `s` für Sekunden, `m` für Minuten, `h` für Stunden und `d` für Tage.

Sekunden werden auf die nächste Minute aufgerundet, da Cron eine Granularität von einer Minute hat. Intervalle, die nicht gleichmäßig in einen sauberen Cron-Schritt abgebildet werden, wie `7m` oder `90m`, werden auf das nächste Intervall gerundet, das dies tut, und Claude teilt Ihnen mit, was es gewählt hat.

<h3 id="let-claude-choose-the-interval">
  Lassen Sie Claude das Intervall wählen
</h3>

Wenn Sie das Intervall weglassen, wählt Claude stattdessen dynamisch eines, anstatt nach einem festen Cron-Zeitplan zu laufen. Nach jeder Iteration wählt es eine Verzögerung zwischen einer Minute und einer Stunde basierend auf dem, was es beobachtet hat: kurze Wartezeiten, während ein Build fertig wird oder ein PR aktiv ist, längere Wartezeiten, wenn nichts ansteht. Die gewählte Verzögerung und der Grund dafür werden am Ende jeder Iteration gedruckt.

Das folgende Beispiel überprüft CI und Überprüfungskommentare, wobei Claude länger zwischen Iterationen wartet, sobald der PR ruhig wird:

```text theme={null}
/loop check whether CI passed and address any review comments
```

Wenn Sie einen dynamischen `/loop`-Zeitplan anfordern, kann Claude das [Monitor-Tool](/docs/de/tools-reference#monitor-tool) direkt verwenden. Monitor führt ein Hintergrundskript aus und streamt jede Ausgabezeile zurück, was das Abrufen ganz vermeidet und oft token-effizienter und reaktiver ist als das erneute Ausführen eines Prompts in einem Intervall.

Eine dynamisch geplante Schleife erscheint in Ihrer [geplanten Aufgabenliste](#manage-scheduled-tasks) wie jede andere Aufgabe, sodass Sie sie auf die gleiche Weise auflisten oder stornieren können. Die [Jitter-Regeln](#jitter) gelten nicht dafür, aber die [sieben-Tage-Ablauf](#seven-day-expiry) tut es: die Schleife endet automatisch sieben Tage nach dem Start.

<Note>
  Bei Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform und Microsoft Foundry läuft ein Prompt ohne Intervall stattdessen nach einem festen 10-Minuten-Zeitplan.
</Note>

<h3 id="run-the-built-in-maintenance-prompt">
  Führen Sie den integrierten Wartungs-Prompt aus
</h3>

Wenn Sie den Prompt weglassen, verwendet Claude stattdessen einen integrierten Wartungs-Prompt. Bei jeder Iteration arbeitet es folgende Punkte in dieser Reihenfolge durch:

* Fortsetzen unvollendeter Arbeiten aus dem Gespräch
* Kümmern Sie sich um den Pull Request des aktuellen Branches: Überprüfungskommentare, fehlgeschlagene CI-Läufe, Merge-Konflikte
* Führen Sie Bereinigungsdurchläufe durch, wie Fehlersuche oder Vereinfachung, wenn nichts anderes ansteht

Claude startet keine neuen Initiativen außerhalb dieses Umfangs, und irreversible Aktionen wie Pushing oder Löschen erfolgen nur, wenn sie etwas fortsetzen, das das Transkript bereits autorisiert hat.

```text theme={null}
/loop
```

Ein einfaches `/loop` führt diesen Prompt in einem [dynamisch gewählten Intervall](#let-claude-choose-the-interval) aus. Fügen Sie ein Intervall hinzu, zum Beispiel `/loop 15m`, um es stattdessen nach einem festen Zeitplan auszuführen. Um den integrierten Prompt durch Ihren eigenen Standard zu ersetzen, siehe [Passen Sie den Standard-Prompt mit loop.md an](#customize-the-default-prompt-with-loop-md).

<Note>
  Bei Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform und Microsoft Foundry wird `/loop` ohne Prompt die Nutzungsmeldung gedruckt, anstatt den Wartungs-Prompt auszuführen.
</Note>

<h3 id="customize-the-default-prompt-with-loop-md">
  Passen Sie den Standard-Prompt mit loop.md an
</h3>

Eine `loop.md`-Datei ersetzt den integrierten Wartungs-Prompt durch Ihre eigenen Anweisungen. Sie definiert einen einzelnen Standard-Prompt für einfaches `/loop`, nicht eine Liste separater geplanter Aufgaben, und wird ignoriert, wenn Sie einen Prompt in der Befehlszeile angeben. Um zusätzliche Prompts daneben zu planen, verwenden Sie `/loop <prompt>` oder [fragen Sie Claude direkt](#manage-scheduled-tasks).

Claude sucht die Datei an zwei Orten und verwendet die erste, die er findet.

| Pfad                | Umfang                                                                  |
| :------------------ | :---------------------------------------------------------------------- |
| `.claude/loop.md`   | Projektebene. Hat Vorrang, wenn beide Dateien vorhanden sind.           |
| `~/.claude/loop.md` | Benutzerebene. Gilt in jedem Projekt, das sein eigenes nicht definiert. |

Die Datei ist einfaches Markdown ohne erforderliche Struktur. Schreiben Sie sie so, als würden Sie den `/loop`-Prompt direkt eingeben. Das folgende Beispiel hält einen Release-Branch gesund:

```markdown title=".claude/loop.md" theme={null}
Check the `release/next` PR. If CI is red, pull the failing job log,
diagnose, and push a minimal fix. If new review comments have arrived,
address each one and resolve the thread. If everything is green and
quiet, say so in one line.
```

Änderungen an `loop.md` treten bei der nächsten Iteration in Kraft, sodass Sie die Anweisungen verfeinern können, während eine Schleife läuft. Wenn keine `loop.md` an einem der beiden Orte vorhanden ist, fällt die Schleife auf den integrierten Wartungs-Prompt zurück. Halten Sie die Datei prägnant: Inhalte über 25.000 Bytes werden gekürzt.

<Note>
  Bei Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform und Microsoft Foundry wird `loop.md` nicht gelesen und `/loop` ohne Prompt druckt die Nutzungsmeldung aus.
</Note>

<h3 id="stop-a-loop">
  Stoppen Sie eine Schleife
</h3>

Um eine `/loop` zu stoppen, während sie auf die nächste Iteration wartet, drücken Sie `Esc`. Dies löscht den ausstehenden Wakeup, sodass die Schleife nicht erneut läuft. Aufgaben, die Sie durch [direktes Fragen an Claude](#manage-scheduled-tasks) geplant haben, sind nicht von `Esc` betroffen und bleiben bestehen, bis Sie sie löschen.

Im [selbstgesteuertem Modus](#let-claude-choose-the-interval) kann Claude die Schleife auch selbst beenden, sobald die Aufgabe abgeschlossen ist. Claude ruft das [`ScheduleWakeup`-Tool](/docs/de/tools-reference) mit `stop: true` auf, was den ausstehenden Wakeup sofort storniert. Wenn eine Iteration endet, ohne entweder neu zu planen oder zu stoppen, plant Claude Code einen Fallback-Wakeup etwa 20 Minuten später und beendet die Schleife, wenn diese Iteration auch nicht neu plant. Vor v2.1.202 war das Nicht-Neuplanung die einzige Möglichkeit, wie Claude eine Schleife selbst beenden konnte.

Schleifen nach einem festen Intervall laufen weiter, bis Sie sie stoppen oder [sieben Tage vergehen](#seven-day-expiry).

<h2 id="set-a-one-time-reminder">
  Setzen Sie eine einmalige Erinnerung
</h2>

Für einmalige Erinnerungen beschreiben Sie, was Sie möchten, in natürlicher Sprache, anstatt `/loop` zu verwenden. Claude plant eine einmalige Aufgabe, die sich nach der Ausführung selbst löscht.

```text theme={null}
remind me at 3pm to push the release branch
```

```text theme={null}
in 45 minutes, check whether the integration tests passed
```

Claude heftet die Ausführungszeit an eine bestimmte Minute und Stunde mit einem Cron-Ausdruck an und bestätigt, wann sie läuft.

<h2 id="manage-scheduled-tasks">
  Verwalten Sie geplante Aufgaben
</h2>

Bitten Sie Claude in natürlicher Sprache, Aufgaben aufzulisten oder zu stornieren, oder verweisen Sie direkt auf die zugrunde liegenden Tools.

```text theme={null}
what scheduled tasks do I have?
```

```text theme={null}
cancel the deploy check job
```

Unter der Haube verwendet Claude diese Tools:

| Tool         | Zweck                                                                                                                                         |
| :----------- | :-------------------------------------------------------------------------------------------------------------------------------------------- |
| `CronCreate` | Planen Sie eine neue Aufgabe. Akzeptiert einen 5-Feld-Cron-Ausdruck, den auszuführenden Prompt und ob er wiederkehrend ist oder einmal läuft. |
| `CronList`   | Listet alle geplanten Aufgaben mit ihren IDs, Zeitplänen und Prompts auf.                                                                     |
| `CronDelete` | Stornieren Sie eine Aufgabe nach ID.                                                                                                          |

Jede geplante Aufgabe hat eine 8-stellige ID, die Sie an `CronDelete` übergeben können. Eine Sitzung kann gleichzeitig bis zu 50 geplante Aufgaben enthalten.

<h2 id="how-scheduled-tasks-run">
  Wie geplante Aufgaben ausgeführt werden
</h2>

Der Scheduler überprüft jede Sekunde auf fällige Aufgaben und reiht sie mit niedriger Priorität ein. Ein geplanter Prompt läuft zwischen Ihren Zügen, nicht während Claude mitten in einer Antwort ist. Wenn Claude beschäftigt ist, wenn eine Aufgabe fällig wird, wartet der Prompt, bis der aktuelle Zug endet.

Alle Zeiten werden in Ihrer lokalen Zeitzone interpretiert. Ein Cron-Ausdruck wie `0 9 * * *` bedeutet 9 Uhr, wo immer Sie Claude Code ausführen, nicht UTC.

<h3 id="jitter">
  Jitter
</h3>

Um zu vermeiden, dass jede Sitzung die API zum gleichen Wanduhrzeitpunkt trifft, fügt der Scheduler einen deterministischen Offset zu Ausführungszeiten hinzu:

* Wiederkehrende Aufgaben laufen bis zu 30 Minuten nach der geplanten Zeit (oder bis zu der Hälfte des Intervalls für Aufgaben, die häufiger als stündlich ausgeführt werden). Ein stündlicher Job, der für `:00` geplant ist, kann überall bis `:30` laufen.
* Einmalige Aufgaben, die für die Ober- oder Unterseite der Stunde geplant sind, laufen bis zu 90 Sekunden früh.

Der Offset wird von der Aufgaben-ID abgeleitet, daher erhält die gleiche Aufgabe immer den gleichen Offset. Wenn genaue Zeitangaben wichtig sind, wählen Sie eine Minute, die nicht `:00` oder `:30` ist, zum Beispiel `3 9 * * *` statt `0 9 * * *`, und der einmalige Jitter wird nicht angewendet.

<h3 id="seven-day-expiry">
  Ablauf nach sieben Tagen
</h3>

Wiederkehrende Aufgaben verfallen automatisch 7 Tage nach der Erstellung. Die Aufgabe läuft ein letztes Mal, dann löscht sie sich selbst. Dies begrenzt, wie lange eine vergessene Schleife laufen kann. Wenn Sie benötigen, dass eine wiederkehrende Aufgabe länger dauert, stornieren und erstellen Sie sie neu, bevor sie abläuft, oder verwenden Sie [Routinen](/docs/de/routines) oder [Desktop-geplante Aufgaben](/docs/de/desktop-scheduled-tasks) für dauerhafte Planung.

<h2 id="cron-expression-reference">
  Cron-Ausdrucksreferenz
</h2>

`CronCreate` akzeptiert Standard-5-Feld-Cron-Ausdrücke: `minute hour day-of-month month day-of-week`. Alle Felder unterstützen Wildcards (`*`), einzelne Werte (`5`), Schritte (`*/15`), Bereiche (`1-5`) und kommagetrennte Listen (`1,15,30`).

| Beispiel       | Bedeutung                     |
| :------------- | :---------------------------- |
| `*/5 * * * *`  | Alle 5 Minuten                |
| `0 * * * *`    | Jede Stunde zur vollen Stunde |
| `7 * * * *`    | Jede Stunde um 7 Minuten nach |
| `0 9 * * *`    | Jeden Tag um 9 Uhr lokal      |
| `0 9 * * 1-5`  | Wochentags um 9 Uhr lokal     |
| `30 14 15 3 *` | 15. März um 14:30 Uhr lokal   |

Der Wochentag verwendet `0` oder `7` für Sonntag bis `6` für Samstag. Erweiterte Syntax wie `L`, `W`, `?` und Namensaliase wie `MON` oder `JAN` werden nicht unterstützt.

Wenn sowohl der Tag des Monats als auch der Wochentag eingeschränkt sind, stimmt ein Datum überein, wenn eines der Felder übereinstimmt. Dies folgt der Standard-Vixie-Cron-Semantik.

<h2 id="disable-scheduled-tasks">
  Deaktivieren Sie geplante Aufgaben
</h2>

Setzen Sie `CLAUDE_CODE_DISABLE_CRON=1` in Ihrer Umgebung, um den Scheduler vollständig zu deaktivieren. Die Cron-Tools und `/loop` werden nicht verfügbar, und alle bereits geplanten Aufgaben stoppen das Laufen. Siehe [Umgebungsvariablen](/docs/de/env-vars) für die vollständige Liste der Deaktivierungsflags.

<h2 id="limitations">
  Einschränkungen
</h2>

Die sitzungsbezogene Planung hat inhärente Einschränkungen:

* Aufgaben laufen nur, während Claude Code läuft und untätig ist. Das Schließen des Terminals oder das Beenden der Sitzung stoppt sie. [Backgrounding der Sitzung](/docs/de/agent-view#from-inside-a-session) trägt `/loop`-Aufgaben zu einer Hintergrund-Sitzung über, die ohne Terminal weiterläuft.
* Kein Aufholen für verpasste Läufe. Wenn die geplante Zeit einer Aufgabe verstreicht, während Claude mit einer langwierigen Anfrage beschäftigt ist, läuft sie einmal, wenn Claude untätig wird, nicht einmal pro verpasstem Intervall.
* Neues Gespräch löscht alle sitzungsbezogenen Aufgaben. Das Fortsetzen mit `claude --resume` oder `claude --continue` stellt Aufgaben wieder her, die nicht abgelaufen sind: wiederkehrende Aufgaben innerhalb von sieben Tagen nach der Erstellung und einmalige Aufgaben, deren geplante Zeit noch nicht vergangen ist. Hintergrund-Bash- und Monitor-Aufgaben werden bei Fortsetzen nie wiederhergestellt.

Für Cron-gesteuerte Automatisierung, die unbeaufsichtigt laufen muss:

* [Routinen](/docs/de/routines): Laufen auf von Anthropic verwalteter Infrastruktur nach Zeitplan, über API-Aufruf oder bei GitHub-Ereignissen
* [GitHub Actions](/docs/de/github-actions): Verwenden Sie einen `schedule`-Trigger in CI
* [Desktop-geplante Aufgaben](/docs/de/desktop-scheduled-tasks): Laufen lokal auf Ihrem Computer
