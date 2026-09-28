> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Wählen Sie einen Berechtigungsmodus

> Steuern Sie, ob Claude vor dem Bearbeiten von Dateien oder dem Ausführen von Befehlen fragt. Wechseln Sie Modi mit Shift+Tab in der CLI, dem Modusindikator in VS Code oder dem Moduswahlschalter in Desktop.

Ein Berechtigungsmodus legt fest, welche Aktionen Claude in einer Sitzung ausführen kann, ohne Sie vorher zu fragen. Im Manual-Modus hält Claude Code inne und fragt Sie, bevor die meisten Aktionen ausgeführt werden, die Dateien bearbeiten, Shell-Befehle ausführen oder das Netzwerk erreichen. Im [Auto-Modus](#eliminate-prompts-with-auto-mode) überprüft ein zweites Modell, der Klassifizierer, Aktionen statt Ihnen; [wie der Klassifizierer Aktionen bewertet](#how-the-classifier-evaluates-actions) listet auf, welche Aktionen er überprüft und welche er überspringt.

Bei Pro-, Max- und Team-Plänen ist der integrierte Standard-Berechtigungsmodus Auto-Modus. [Welcher Modus eine Sitzung startet](#which-mode-a-session-starts-in) behandelt die Oberflächen und Einstellungen, die den Standard-Berechtigungsmodus ändern. Sie können auch den Berechtigungsmodus einer laufenden Sitzung jederzeit ändern.

<h2 id="available-modes">
  Verfügbare Modi
</h2>

Jeder Modus stellt einen anderen Kompromiss zwischen Benutzerfreundlichkeit und Überwachung dar. Die folgende Tabelle zeigt, was Claude in jedem Modus ohne Berechtigungsaufforderung tun kann. Der Manual-Modus wird unter seinem Konfigurationswert `default` angezeigt.

| Modus                                                               | Was ohne Nachfrage ausgeführt wird                                                                                           | Am besten geeignet für                                 |
| :------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------- |
| `default`                                                           | Nur Lesevorgänge                                                                                                             | Überprüfung jeder Aktion selbst, sensible Arbeiten     |
| [`acceptEdits`](#auto-approve-file-edits-with-acceptedits-mode)     | Lesevorgänge, Dateibearbeitungen und häufige Dateisystembefehle (`mkdir`, `touch`, `mv`, `cp` usw.)                          | Iteration bei Code-Überprüfung                         |
| [`plan`](#analyze-before-you-edit-with-plan-mode)                   | Lesevorgänge, plus vom Klassifizierer genehmigte Befehle, wenn [Auto-Modus](#eliminate-prompts-with-auto-mode) verfügbar ist | Erkundung einer Codebasis vor Änderungen               |
| [`auto`](#eliminate-prompts-with-auto-mode)                         | Alles, mit Sicherheitsprüfungen im Hintergrund                                                                               | Lange Aufgaben, Reduzierung von Aufforderungsmüdigkeit |
| [`dontAsk`](#allow-only-pre-approved-tools-with-dontask-mode)       | Lesevorgänge und vorab genehmigte Tools; alles, das eine Aufforderung auslösen würde, wird abgelehnt                         | Gesperrte CI und Skripte                               |
| [`bypassPermissions`](#skip-all-checks-with-bypasspermissions-mode) | Alles                                                                                                                        | Nur isolierte Container und VMs                        |

Der Modus, der jede Aktion überprüft, wird in der CLI, in `claude --help`, in den VS Code- und JetBrains-Erweiterungen und in der Desktop-App als **Manual** bezeichnet. Sein Konfigurationswert ist `default`, was Hooks und SDK-Integrationen verwenden. Die CLI akzeptiert `manual` als Alias überall dort, wo Sie den Wert eingeben, zum Beispiel `claude --permission-mode manual` oder `"defaultMode": "manual"`. Das Manual-Label und der `manual`-Alias erfordern Claude Code v2.1.200 oder später. Das Label der Desktop-App hängt nicht von Ihrer CLI-Version ab.

Schreibvorgänge in [geschützte Pfade](#protected-paths) werden niemals automatisch genehmigt, außer im `bypassPermissions`-Modus und in Plan-Mode-Sitzungen, in denen Bypass-Berechtigungen verfügbar sind, was bedeutet, dass Sitzungen auf eine Weise gestartet wurden, die [`bypassPermissions` in den Moduszyklus](#switch-permission-modes) aufnimmt.

Modi legen die Grundlage fest. Überlagern Sie [Berechtigungsregeln](/docs/de/permissions#manage-permissions) darauf, um bestimmte Tools vorab zu genehmigen oder zu blockieren. Deny-Regeln blockieren in jedem Modus, einschließlich `bypassPermissions`. Deny- und Ask-Regeln gelten nicht für [`EndConversation`](/docs/de/tools-reference#endconversation-tool-behavior), solange Claude noch mindestens ein anderes Tool aufrufen kann. Allow-Regeln haben keine Auswirkung in `bypassPermissions`.

<h3 id="actions-no-mode-auto-approves">
  Aktionen, die kein Modus automatisch genehmigt
</h3>

Claude Code genehmigt die folgenden in keinem Modus automatisch, einschließlich `bypassPermissions`. Jeder Punkt verlinkt auf den Abschnitt, der sagt, was stattdessen in jedem Modus passiert:

* Tools, die einer expliziten [Ask-Regel](/docs/de/permissions#manage-permissions) entsprechen
* Connector-Tools, die Ihre Organisation [auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools), in Sitzungen, in denen diese Einstellung Claude Code erreicht
* Tools, die Benutzerinteraktion erfordern: das integrierte `AskUserQuestion`-Tool und MCP-Tools, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind
* `rm` und `rmdir` Löschungen, die auf einen [kritischen Pfad](#critical-paths) abzielen, den keine Allow-Regel oder `PreToolUse` Hook `"allow"` genehmigt
* Die [Cross-Session-Messaging-Schutzmaßnahmen](#skip-all-checks-with-bypasspermissions-mode)
* Lesevorgänge außerhalb der Arbeitsverzeichnisse, während [`permissions.blockReadsOutsideWorkingDirectories`](/docs/de/settings-reference#permissions-blockreadsoutsideworkingdirectories) aktiviert ist: erkannte Datei-lesende Bash-Befehle und jeder [unsandboxed Retry](/docs/de/sandboxing#the-unsandboxed-retry-escape-hatch), der Genehmigung benötigt, um außerhalb der Sandbox zu laufen, fordern auch im Auto-Modus und `bypassPermissions`-Modus auf. Erfordert Claude Code v2.1.257 oder später.

  Ein Befehl, den der Shell-Parser nicht verfolgen kann, wie einer, der das Verzeichnis mehr als einmal wechselt oder eine Subshell ausführt, fordert auf die gleiche Weise auf, auch wenn er keinen außerhalb liegenden Pfad benennt. Diese Aufforderung gilt nicht, wenn der Befehl in der [Sandbox](/docs/de/sandboxing) ausgeführt wird und die Sandbox die Blockierung erzwingt.

<h2 id="common-setups">
  Häufige Setups
</h2>

Berechtigungsmodi entscheiden, ob Claude vor einer Aktion fragt, und die [Bash-Sandbox](/docs/de/sandboxing) und äußere [Isolationsgrenzen](/docs/de/sandbox-environments) entscheiden, was eine Aktion erreichen kann, sobald sie läuft. Jede Zeile unten paart ein Ziel mit den Flags oder Einstellungen, die Sie dorthin bringen, und der Isolation, die es benötigt, als Ausgangspunkt. [Verfügbare Modi](#available-modes) listet auf, was in jedem Modus ohne Aufforderung läuft.

| Sie möchten                                                           | Starten Sie mit                                                                                                                                                                | Isolation erforderlich                                                                                                                                                                                                | Notizen                                                                                                                                                                                                                                                                         |
| :-------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Jede Aktion selbst überprüfen                                         | Manual-Modus: `claude --permission-mode default`                                                                                                                               | Keine                                                                                                                                                                                                                 | Sensible Arbeiten, unbekannter Code                                                                                                                                                                                                                                             |
| Lokal mit weniger Aufforderungen iterieren, ohne einen Klassifizierer | Manual-Modus plus die Bash-Sandbox im [Auto-Allow-Modus](/docs/de/sandboxing#sandbox-modes): `claude --permission-mode default`, dann `/sandbox` ausführen und Auto-Allow auswählen | Die integrierte Bash-Sandbox, auf macOS, Linux und WSL2                                                                                                                                                               | Deny-Regeln gelten weiterhin, und Ask-Regeln, die einen Befehl benennen, wie `Bash(git push *)`, fordern weiterhin auf. Um die Sandbox stattdessen aus einer Einstellungsdatei zu aktivieren, setzen Sie [`sandbox.enabled`](/docs/de/settings-reference#sandbox-enabled) auf `true` |
| Erkunden Sie, bevor Sie etwas ändern                                  | `claude --permission-mode plan`                                                                                                                                                | Keine                                                                                                                                                                                                                 | Claude Code blockiert Bearbeitungen, bis Sie einen Plan [genehmigen](#review-and-approve-a-plan)                                                                                                                                                                                |
| Arbeiten Sie freihändig im Auto-Modus                                 | `claude --permission-mode auto`, der [integrierte Standard-Berechtigungsmodus](#which-mode-a-session-starts-in) bei Pro, Max und Team                                          | Keine; eine Sandbox oder ein Container fügt Verteidigungstiefe hinzu                                                                                                                                                  | Erfordert ein [unterstütztes Modell](#eliminate-prompts-with-auto-mode), und Ihre Organisation kann [Auto-Modus ausschalten](#eliminate-prompts-with-auto-mode)                                                                                                                 |
| Führen Sie in CI mit einer genauen Allowlist aus                      | `claude -p "run the test suite" --permission-mode dontAsk --allowedTools "Bash(npm test)" "Read"`                                                                              | Keine über das hinaus, was Ihr CI-Runner bietet                                                                                                                                                                       | [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) ignorieren `dontAsk` aus Einstellungsdateien                                                                                                                                                                                      |
| Führen Sie vollständig unbeaufsichtigt in einem Container aus         | `claude -p "<prompt>" --dangerously-skip-permissions`                                                                                                                          | Erforderlich: ein Container, VM oder die [Sandbox-Laufzeit](/docs/de/sandbox-environments#sandbox-runtime); auf Linux und macOS, führen Sie es als [Nicht-Root-Benutzer](#skip-all-checks-with-bypasspermissions-mode) aus | Cloud-Sitzungen ignorieren diesen Modus aus Einstellungsdateien. In diesem `-p` Lauf werden die [wenigen Aufrufe, die weiterhin auffordern würden](#skip-all-checks-with-bypasspermissions-mode) stattdessen verweigert                                                         |

Die Bash-Sandbox und der Auto-Modus funktionieren unabhängig und kombinieren sich, mit den Ausnahmen, die unter [Sandbox-Modi](/docs/de/sandboxing#sandbox-modes) aufgelistet sind. Für die vollständige Interaktion siehe [Wie Sandboxing sich auf Berechtigungen und Berechtigungsmodi bezieht](/docs/de/sandboxing#how-sandboxing-relates-to-permissions-and-permission-modes) und [Wie Isolation sich auf Berechtigungsmodi bezieht](/docs/de/sandbox-environments#how-isolation-relates-to-permission-modes).

<h2 id="which-mode-a-session-starts-in">
  Welcher Modus eine Sitzung startet
</h2>

Wenn Sie eine neue Sitzung in einem Terminal starten, nimmt Claude Code den Berechtigungsmodus aus dem ersten dieser Punkte, der zutrifft:

1. Das `--permission-mode` Flag oder `--dangerously-skip-permissions`

2. `permissions.defaultMode` in einer [Einstellungsdatei](/docs/de/settings#where-settings-live)

   Wenn Sie `"auto"` in `.claude/settings.json` oder `.claude/settings.local.json` setzen, wird der Wert nicht wirksam, und Claude Code verwendet dann stattdessen den integrierten Standard, anstatt einen `defaultMode` aus `~/.claude/settings.json` zu verwenden. Wenn Sie `"bypassPermissions"` in diesen beiden Dateien setzen, wird es auch nicht wirksam, und die Sitzung startet im Manual-Modus. Die anderen Werte gelten aus jeder Einstellungsdatei.

3. Der integrierte Standard

Konversationen, die die VS Code-Erweiterung startet, folgen der eigenen Liste der Erweiterung in [Berechtigungsmodi wechseln](#switch-permission-modes). Für den Berechtigungsmodus, in dem Claude Code eine fortgesetzte Sitzung startet, siehe [Berechtigungsmodus beim Fortsetzen](/docs/de/sessions#permission-mode-on-resume).

Der integrierte `auto` Standard erfordert Claude Code v2.1.228 oder später auf macOS, Linux und WSL, und v2.1.233 oder später auf nativem Windows. Bei früheren Versionen ist der integrierte Standard Manual.

Der integrierte Standard hängt davon ab, wie Sie Claude Code ausführen, von Ihrem Plan und davon, ob Claude Code seine Feature-Flags abrufen konnte. Die erste Zeile, die Ihrer Sitzung entspricht, gilt. Die Tabelle behandelt Sitzungen, die Sie in einem Terminal oder über die VS Code-Erweiterung starten; für die Desktop-App und claude.ai siehe die Desktop- und Web-Registerkarten in [Berechtigungsmodi wechseln](#switch-permission-modes).

| Wie Sie Claude Code ausführen                                                                                                                                                                                                                                          | Integrierter Standard-Berechtigungsmodus |
| :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------- |
| Eine Einstellungsdatei setzt `disableAutoMode` auf `"disable"`                                                                                                                                                                                                         | `default`                                |
| [Feature-Flag-Abruf](/docs/de/env-vars#features-that-need-feature-flag-fetching) ist aus                                                                                                                                                                                    | `default`                                |
| Ihre [erste Sitzung nach der Installation von Claude Code oder dem Upgrade](/docs/de/env-vars#first-session-after-an-install-or-upgrade) auf eine Version, die diesen Standard hinzufügt, es sei denn, nach einer Neuinstallation ruft Claude Code die Flags rechtzeitig ab | `default`                                |
| `claude -p` oder das [Agent SDK](/docs/de/agent-sdk/permissions)                                                                                                                                                                                                            | `default`                                |
| Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, [Claude Platform on AWS](/docs/de/claude-platform-on-aws) oder eine angemeldete [Claude Apps Gateway](/docs/de/claude-apps-gateway) Sitzung                                                                    | `default`                                |
| Ein Pro-, Max- oder Team-Plan, in einem Terminal oder über die [VS Code-Erweiterung](/docs/de/vs-code)                                                                                                                                                                      | `auto`                                   |
| Ein Enterprise-Plan oder ein Claude Console API-Schlüssel                                                                                                                                                                                                              | `default`                                |

Wenn Feature-Flag-Abruf aus ist oder in einer [ersten Sitzung nach einer Installation oder einem Upgrade](/docs/de/env-vars#first-session-after-an-install-or-upgrade), in der die Flags noch nicht angekommen sind, ignoriert die VS Code-Erweiterung jede Einstellungsdatei bei der Wahl des Standard-Berechtigungsmodus.

Wenn das Flag, eine Einstellungsdatei oder der integrierte Standard `auto` auswählt, aber Auto-Modus nicht für die Sitzung verfügbar ist, startet Claude Code die Sitzung stattdessen im Manual-Modus. Auto-Modus ist nicht verfügbar, wenn die Sitzung die [Verfügbarkeitsanforderungen](#eliminate-prompts-with-auto-mode) nicht erfüllt, wie eine Einstellungsdatei, die ihn ausschaltet, oder ein Modell, das ihn nicht unterstützt, oder wenn Anthropic ihn vorübergehend serverseitig ausgeschaltet hat.

Das erste Mal, wenn der integrierte Standard eine Ihrer Sitzungen im Auto-Modus startet, zeigt Claude Code einen Hinweis, der auf diese Seite verlinkt:

* In einem Terminal, einmal, oben in der Sitzung
* In der VS Code-Erweiterung, als Karte auf dem Bildschirm für neue Konversationen, die bleibt, bis Sie sie schließen

Bei Pro-, Max- und Team-Plänen, wenn Ihre `~/.claude/settings.json` einen `defaultMode` setzt, der nicht `auto` ist, und keine andere Einstellungsdatei setzt einen, starten Ihre Sitzungen weiterhin in diesem Modus. Claude Code fragt einmal, im Terminal oder in der VS Code-Erweiterung, ob die Einstellung auf Auto-Modus geändert werden soll. Wenn Sie ablehnen, bleibt Ihre Einstellung wie sie ist.

<h3 id="start-in-a-different-mode">
  Starten Sie in einem anderen Berechtigungsmodus
</h3>

Sie können den Standard-Berechtigungsmodus für eine Sitzung oder als Standard für jede Sitzung auf einem Computer, in einem Projekt oder in einer Organisation festlegen. Wenn mehr als eine Einstellungsdatei `permissions.defaultMode` setzt, entscheidet [Einstellungspriorität](/docs/de/settings#settings-precedence), sodass ein Projekt- oder verwalteter Wert `~/.claude/settings.json` übertrumpft. Um den Berechtigungsmodus einer bereits laufenden Sitzung zu ändern, siehe [Berechtigungsmodi wechseln](#switch-permission-modes).

| Um den Standard-Berechtigungsmodus festzulegen für         | Tun Sie dies                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :--------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Eine Sitzung, die Sie gerade starten                       | Übergeben Sie den Berechtigungsmodus als Flag, zum Beispiel `claude --permission-mode default`                                                                                                                                                                                                                                                                                                                                            |
| Jede Terminal-Sitzung, die Sie auf diesem Computer starten | Setzen Sie `permissions.defaultMode` in `~/.claude/settings.json`. Für das, was die VS Code-Erweiterung liest, siehe [Berechtigungsmodi wechseln](#switch-permission-modes)                                                                                                                                                                                                                                                               |
| Jede Terminal-Sitzung, die Sie in einem Projekt starten    | Setzen Sie `permissions.defaultMode` in `.claude/settings.json` des Projekts. Sitzungen, die Sie in einem Terminal starten, respektieren jeden Wert außer `auto` und `bypassPermissions`; Sitzungen, die die VS Code-Erweiterung startet, lesen keine Projekteinstellungen für den Standard-Berechtigungsmodus                                                                                                                            |
| Jede Terminal-Sitzung in Ihrer Organisation                | Setzen Sie `permissions.defaultMode` in [verwalteten Einstellungen](/docs/de/managed-settings). Terminal-Sitzungen starten in diesem Modus und Personen können weiterhin zum Auto-Modus wechseln; für das, was die VS Code-Erweiterung liest, siehe [Berechtigungsmodi wechseln](#switch-permission-modes). Um Auto-Modus zu entfernen, damit niemand ihn auswählen kann, setzen Sie stattdessen `permissions.disableAutoMode` auf `"disable"` |

Dieses Beispiel macht jede Terminal-Sitzung auf Ihrem Computer im Manual-Modus starten, dessen Konfigurationswert `default` ist. Speichern Sie es in `~/.claude/settings.json`:

```json theme={null}
{
  "permissions": {
    "defaultMode": "default"
  }
}
```

Die nächste Sitzung, die Sie starten, zeigt `⏸ manual mode on` in der Statusleiste.

<h2 id="switch-permission-modes">
  Berechtigungsmodi wechseln
</h2>

Jede Schnittstelle hat ihre eigene Kontrolle zum Wechseln von Berechtigungsmodi während einer Sitzung und ihre eigene Weise, den Berechtigungsmodus zu wählen, den neue Sitzungen starten. Wählen Sie Ihre Schnittstelle aus, um ihre Kontrollen zu sehen.

<Tabs>
  <Tab title="CLI">
    **Während einer Sitzung**: Drücken Sie `Shift+Tab`, um Berechtigungsmodi zu durchlaufen. Von `auto` wechselt der erste Druck zu `default`, und der Zyklus läuft dann `default` → `acceptEdits` → `plan` → zurück zu `default`. Optionale Modi, die unten beschrieben werden, werden nach `plan` eingefügt. Die Statusleiste zeigt den aktiven Modus als graues `⏸ manual mode on` für `default` oder als `⏵⏵ accept edits on`, `⏸ plan mode on`, `⏵⏵ auto mode on`, `⏵⏵ don't ask on` oder `⏵⏵ bypass permissions on`.

    Nicht jeder Modus ist im Standard-Zyklus enthalten:

    * `auto`: wird angezeigt, wenn [Auto-Modus verfügbar ist](#eliminate-prompts-with-auto-mode); das Wechseln zu ihm schaltet Berechtigungsmodi ohne Bestätigungsaufforderung um
    * `bypassPermissions`: wird angezeigt, nachdem Sie mit `--permission-mode bypassPermissions`, `--dangerously-skip-permissions`, `--allow-dangerously-skip-permissions` oder `permissions.defaultMode: "bypassPermissions"` in [Benutzer-, `--settings`- oder verwalteten Einstellungen](/docs/de/settings-reference#permissions-defaultmode) starten. Die `--allow-` Variante fügt den Berechtigungsmodus zum Zyklus hinzu, ohne ihn zu aktivieren
    * `dontAsk`: wird nie im Zyklus angezeigt; setzen Sie ihn mit `--permission-mode dontAsk`

    Aktivierte optionale Modi werden nach `plan` eingefügt, mit `bypassPermissions` zuerst und `auto` zuletzt. Wenn Sie beide aktiviert haben, wechseln Sie durch `bypassPermissions` auf dem Weg zu `auto`.

    **Von einer Bash-Berechtigungsaufforderung**: im Manual- und `acceptEdits`-Berechtigungsmodus, wenn [Auto-Modus](#eliminate-prompts-with-auto-mode) verfügbar ist, fügt Claude Code **Ja, und zum Auto-Modus wechseln** zu einer Bash-Befehlsberechtigungsaufforderung hinzu. Wählen Sie es aus, um den Befehl zu genehmigen und die Sitzung zum Auto-Modus zu wechseln. [PowerShell-Tool](/docs/de/tools-reference#powershell-tool) Aufforderungen bieten die Option nicht an. Erfordert Claude Code v2.1.247 oder später.

    Claude Code fügt die Option nicht zu Aufforderungen hinzu, die von einer Ihrer [`ask` Regeln](/docs/de/permissions#manage-permissions) oder von einem [Hook](/docs/de/hooks#pretooluse-decision-control) erzwungen werden, da Auto-Modus diese Aufforderungen immer noch zeigt, sodass das Wechseln sie nicht entfernen würde.

    **Beim Start**: Übergeben Sie den Berechtigungsmodus als Flag.

    ```bash theme={null}
    claude --permission-mode plan
    ```

    **Als Standard**: Setzen Sie `permissions.defaultMode` im Bereich, den Sie möchten, wie in [Starten Sie in einem anderen Berechtigungsmodus](#start-in-a-different-mode) beschrieben.

    Das gleiche `--permission-mode` Flag funktioniert mit `-p` für [nicht-interaktive Ausführungen](/docs/de/headless).
  </Tab>

  <Tab title="VS Code">
    **Während einer Sitzung**: Klicken Sie auf den Modusindikator am unteren Rand des Eingabefelds. Er verwendet diese Beschriftungen für die Modi auf dieser Seite:

    | UI-Beschriftung    | Modus               |
    | :----------------- | :------------------ |
    | Manual             | `default`           |
    | Edit automatically | `acceptEdits`       |
    | Plan               | `plan`              |
    | Auto               | `auto`              |
    | Bypass permissions | `bypassPermissions` |

    **Als Standard**: Um den Berechtigungsmodus zu fixieren, in dem Konversationen starten, setzen Sie `claudeCode.initialPermissionMode` in Ihren VS Code-Benutzereinstellungen auf `default`, `manual`, `acceptEdits`, `plan` oder `bypassPermissions`. Die Einstellung akzeptiert nicht `auto`; um im Auto-Modus zu starten, lassen Sie sie ungesetzt und wählen Sie **Auto** aus dem Modusindikator einmal aus, wie Punkt 2 unten beschreibt. Die Erweiterung startet jede neue Konversation in der ersten dieser Punkte, die zutrifft:

    1. `claudeCode.initialPermissionMode`
    2. Der Modus, den Sie zuletzt aus dem Modusindikator gewählt haben, wenn er Manual, Edit automatically oder Auto war. Das Auswählen von Plan oder Bypass permissions gilt nur für diese Konversation
    3. `permissions.defaultMode` aus [verwalteten Einstellungen](/docs/de/managed-settings) oder `~/.claude/settings.json`, bei Pro-, Max- und Team-Plänen mit [Feature-Flag-Abruf](#which-mode-a-session-starts-in) verfügbar
    4. Der [integrierte Standard](#which-mode-a-session-starts-in) für Ihren Plan, Anbieter und Organisationseinstellungen

    Die Erweiterung liest nie `.claude/settings.json` oder `.claude/settings.local.json` eines Projekts für den Standard-Berechtigungsmodus, und in Konversationen, die Punkt 3 nicht erfüllen, liest sie überhaupt keine Einstellungsdatei. Wenn `claudeCode.claudeProcessWrapper` gesetzt ist, gelten Punkte 3 und 4 auch nicht: Diese Konversationen starten im Manual-Modus, es sei denn, Punkt 1 oder Punkt 2 setzt einen Berechtigungsmodus.

    Auto wird im Modusindikator angezeigt, wenn [Auto-Modus verfügbar ist](#eliminate-prompts-with-auto-mode).

    Bypass permissions erfordert den Schalter **Allow dangerously skip permissions** in den Erweiterungseinstellungen. Ohne ihn wird der Berechtigungsmodus nicht im Indikator angezeigt, und ein `bypassPermissions` Wert aus Punkt 1 oder Punkt 3 startet die Konversation stattdessen im Manual-Modus. Auto aus jedem Punkt startet die Konversation ebenfalls im Manual-Modus, wenn Auto-Modus nicht verfügbar ist.

    Siehe den [VS Code-Leitfaden](/docs/de/vs-code) für erweiterungsspezifische Details.
  </Tab>

  <Tab title="JetBrains">
    Das JetBrains-Plugin führt Claude Code im IDE-Terminal aus, daher funktioniert das Wechseln von Berechtigungsmodi genauso wie in der CLI: Drücken Sie `Shift+Tab` zum Durchlaufen, oder übergeben Sie `--permission-mode` beim Start.
  </Tab>

  <Tab title="Desktop">
    **Während einer Sitzung**: Verwenden Sie auf der Registerkarte Code den Moduswahlschalter neben der Schaltfläche zum Senden. Nicht jeder Modus wird im Wahlschalter angezeigt:

    * **Auto**: wird angezeigt, wenn [Auto-Modus verfügbar ist](#eliminate-prompts-with-auto-mode)
    * **Bypass permissions**: erfordert den Schalter **Allow bypass permissions mode** in den Desktop-Einstellungen bei Pro- und Max-Plänen; bei Team- und Enterprise-Plänen wird dies stattdessen durch die Organisationsrichtlinie gesteuert

    Die Registerkarte Cowork verwendet diese Modi nicht. Cowork hat seine eigenen Berechtigungsmodi, die separat aktiviert werden, und die Registerkarte Cowork zeigt überhaupt keinen Moduswahlschalter, bis ein Modus über seinen Standard für Ihr Konto aktiviert wird. Siehe die [Cowork-Dokumentation](https://claude.com/docs/cowork/overview).

    Für Desktop-spezifische Details siehe [Wählen Sie einen Berechtigungsmodus](/docs/de/desktop#choose-a-permission-mode) im Desktop-Leitfaden.

    **Als Standard**: Setzen Sie `defaultMode` in [Einstellungen](/docs/de/settings#where-settings-live). Die Desktop-App liest die gleichen Einstellungsdateien wie die CLI und wendet den Berechtigungsmodus auf neue lokale Sitzungen an.

    Ein Modus, den Sie im Moduswahlschalter auswählen, wird pro Ordner gespeichert und hat Vorrang vor `defaultMode` für diesen Ordner. Plan ist die Ausnahme: Das Auswählen gilt nur für die aktuelle Sitzung.

    Für wo `defaultMode` in einer Einstellungsdatei geht, siehe das Beispiel unter [Starten Sie in einem anderen Berechtigungsmodus](#start-in-a-different-mode).
  </Tab>

  <Tab title="Web and mobile">
    Verwenden Sie das Modusmenü neben dem Eingabefeld auf [claude.ai/code](https://claude.ai/code) oder in der mobilen App. Berechtigungsaufforderungen werden in claude.ai zur Genehmigung angezeigt. Welche Modi angezeigt werden, hängt davon ab, wo die Sitzung läuft:

    * **Cloud-Sitzungen** auf [Claude Code im Web](/docs/de/claude-code-on-the-web): Bearbeitungen akzeptieren, Plan und Auto. Bearbeitungen akzeptieren entspricht dem `default` Modus: Cloud-Sitzungen genehmigen Dateibearbeitungen vorab, unabhängig vom Modus, daher zeigt das Menü Bearbeitungen akzeptieren statt Manual an. Cloud-Sitzungen respektieren weiterhin `defaultMode: "acceptEdits"` aus den Einstellungen. Der Auto-Modus wird nur angezeigt, wenn Ihre Organisation ihn zulässt und das ausgewählte Modell ihn unterstützt. Bypass permissions ist nicht verfügbar.
    * **[Remote Control](/docs/de/remote-control) Sitzungen** auf Ihrem lokalen Computer: Manual, Bearbeitungen akzeptieren und Plan. Sie können Auto oder Bypass permissions nicht aus der App auswählen.
      * Außer für Bypass permissions zeigt das Menü den Berechtigungsmodus an, in dem sich die lokale Sitzung befindet, einschließlich eines vom Terminal aus festgelegten Modus. Es wird aktualisiert, wenn sich der Berechtigungsmodus in der App oder im Terminal ändert. Die Sitzung meldet Bypass permissions nie an claude.ai, daher ändert das Wechseln vom Terminal aus nicht, was das Menü anzeigt.
      * Sitzungen, die von der [Desktop-App](/docs/de/desktop) oder der [VS Code-Erweiterung](/docs/de/vs-code) gehostet werden, melden Berechtigungsmodus-Änderungen an claude.ai, während sie passieren, genauso wie Sitzungen, die in einem Terminal gehostet werden.
      * Vor v2.1.202 meldeten Sitzungen, die mit `/remote-control` oder `claude --remote-control` verbunden waren, ihren Berechtigungsmodus überhaupt nicht, daher konnten claude.ai und die mobile App einen Berechtigungsmodus anzeigen, in dem sich die Sitzung nicht befand. Die Nichtübereinstimmung betraf nur die Beschriftung. Claude Code generierte Berechtigungsaufforderungen aus dem tatsächlichen Berechtigungsmodus der Sitzung, und sie wurden weiterhin in der App zur Genehmigung angezeigt.

    Für Remote Control muss der lokale Computer, auf dem die Sitzung läuft, mit Ihrem claude.ai-Konto angemeldet sein; API-Schlüssel werden nicht unterstützt. Sie können auch den Standard-Berechtigungsmodus beim Start dieser lokalen Sitzung festlegen:

    ```bash theme={null}
    claude remote-control --permission-mode acceptEdits
    ```
  </Tab>
</Tabs>

<h2 id="auto-approve-file-edits-with-acceptedits-mode">
  Dateibearbeitungen mit acceptEdits-Modus automatisch genehmigen
</h2>

Der `acceptEdits`-Modus ermöglicht es Claude, Dateien in Ihrem Arbeitsverzeichnis zu erstellen und zu bearbeiten, ohne Sie zu fragen. Die Statusleiste zeigt `⏵⏵ accept edits on` an, während dieser Modus aktiv ist.

Zusätzlich zu Dateibearbeitungen genehmigt der `acceptEdits`-Modus automatisch häufige Bash-Befehle im Dateisystem: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp` und `sed`. Diese Befehle werden auch automatisch genehmigt, wenn sie mit sicheren Umgebungsvariablen wie `LANG=C` oder `NO_COLOR=1` oder Prozess-Wrappern wie `timeout`, `nice` oder `nohup` vorangestellt sind. Wie bei Dateibearbeitungen gilt die automatische Genehmigung nur für Pfade in Ihrem Arbeitsverzeichnis oder `additionalDirectories`. Pfade außerhalb dieses Bereichs, Schreibvorgänge in [geschützte Pfade](#protected-paths), `rm` und `rmdir` Löschungen, die auf einen [kritischen Pfad](#critical-paths) abzielen, und alle anderen Bash-Befehle außer dem [integrierten schreibgeschützten Satz](/docs/de/permissions#read-only-commands) erfordern weiterhin eine Genehmigung.

Wenn das [PowerShell-Tool](/docs/de/tools-reference#powershell-tool) aktiviert ist, genehmigt der `acceptEdits`-Modus auch automatisch `Set-Content`, `Add-Content`, `Clear-Content` und `Remove-Item` auf Pfaden im Gültigkeitsbereich, zusammen mit ihren häufigen Aliasen. Die gleichen Bereichs- und Schutzpfad-Regeln gelten, und `Remove-Item` erhält [seine eigene Prüfung](#remove-item-in-powershell). Ein Positionsargument, das ein Anführungszeichen enthält, wie das Apostroph in `Set-Content .\notes.txt "It's done"`, fordert weiterhin auf, auch auf Pfaden im Gültigkeitsbereich, da Claude Code ein Argument, dessen zitierte und unzitierte Lesarten unterschiedlich sind, nicht statisch validieren kann. Übergeben Sie den Inhalt durch einen benannten Parameter wie `-Value`, um die Aufforderung zu vermeiden.

Verwenden Sie `acceptEdits`, wenn Sie Änderungen in Ihrem Editor oder über `git diff` im Nachhinein überprüfen möchten, anstatt jede Bearbeitung inline zu genehmigen.

Drücken Sie `Shift+Tab` einmal vom Manual-Modus aus, um ihn zu aktivieren, oder starten Sie direkt damit:

```bash theme={null}
claude --permission-mode acceptEdits
```

<h2 id="analyze-before-you-edit-with-plan-mode">
  Analysieren Sie vor dem Bearbeiten mit dem Plan-Modus
</h2>

Der Plan-Modus weist Claude an, Änderungen zu recherchieren und vorzuschlagen, ohne sie vorzunehmen. Claude liest Dateien, führt Shell-Befehle aus, um zu erkunden, und schreibt einen Plan, bearbeitet aber nicht Ihre Quelle. Außer in interaktiven Terminal-Sitzungen mit [verfügbaren Bypass-Berechtigungen](#skip-all-checks-with-bypasspermissions-mode) bleiben Bearbeitungen blockiert, bis Sie den Plan genehmigen.

Wenn [Auto-Modus](/docs/de/auto-mode-config) verfügbar ist und die `useAutoModeDuringPlan` Einstellung aktiviert ist, was die Standardeinstellung ist, überprüft der Klassifizierer Shell-Befehle während der Planung statt Sie zu fragen. Genehmigte Befehle laufen, und abgelehnte werden blockiert. Andernfalls fordern Befehle außerhalb des [integrierten schreibgeschützten Satzes](/docs/de/permissions#read-only-commands) zur Genehmigung auf, auch wenn der Sandbox-[Auto-Allow-Modus](/docs/de/sandboxing#sandbox-modes) aktiviert ist. In interaktiven Terminal-Sitzungen mit verfügbaren Bypass-Berechtigungen gelten weder der Klassifizierer noch eine Aufforderung für Planungsbefehle; [Alle Überprüfungen mit bypassPermissions-Modus überspringen](#skip-all-checks-with-bypasspermissions-mode) behandelt die wenigen Dinge, die dort weiterhin auffordern. In v2.1.212 bis v2.1.217 forderten Sitzungen ohne verfügbare Bypass-Berechtigungen für jeden Befehl außerhalb des schreibgeschützten Satzes auf, unabhängig davon, ob Auto-Modus verfügbar war.

Geben Sie den Plan-Modus ein, indem Sie `Shift+Tab` drücken oder einem einzelnen Prompt `/plan` voranstellen. Sie können auch vom CLI aus im Plan-Modus starten:

```bash theme={null}
claude --permission-mode plan
```

Drücken Sie `Shift+Tab` erneut, um den Plan-Modus zu verlassen, ohne einen Plan zu genehmigen.

<h3 id="review-and-approve-a-plan">
  Überprüfen und genehmigen Sie einen Plan
</h3>

Wenn der Plan fertig ist, präsentiert Claude ihn und fragt, wie Sie vorgehen möchten. Von dieser Aufforderung aus können Sie wählen:

* **Ja, und Auto-Modus verwenden**: Genehmigen und im [Auto-Modus](#eliminate-prompts-with-auto-mode) starten. Wenn Auto-Modus nicht [für Ihre Sitzung verfügbar](#eliminate-prompts-with-auto-mode) ist, beispielsweise weil Ihre Organisation ihn deaktiviert hat, liest diese Option **Ja, Auto-Bearbeitungen akzeptieren**. Wenn Sie die Sitzung mit aktivierten Bypass-Berechtigungen gestartet haben, liest die Option **Ja, und zum BYPASS PERMISSIONS (keine weiteren Aufforderungen) für diese Sitzung wechseln** statt.
* **Ja, Bearbeitungen manuell genehmigen**: Genehmigen und jede Bearbeitung einzeln überprüfen.
* **Nein, weiter planen**: Bleiben Sie im Plan-Modus und sagen Sie Claude, was zu ändern ist.

Das Genehmigen eines Plans beendet den Plan-Modus und wechselt die Sitzung zum Berechtigungsmodus, den jede Genehmigungsoption beschreibt, sodass Claude mit der Bearbeitung beginnt. Um erneut zu planen, wechseln Sie mit `Shift+Tab` zurück zum Plan-Modus, oder stellen Sie Ihrem nächsten Prompt `/plan` voran.

Drücken Sie `Ctrl+G`, um den vorgeschlagenen Plan in Ihrem Standard-Texteditor zu öffnen und ihn direkt zu bearbeiten, bevor Claude fortfährt. Wenn [`showClearContextOnPlanAccept`](/docs/de/settings-reference#showclearcontextonplanaccept) aktiviert ist, erhält die Liste eine erste Option, die den Plan genehmigt und den Planungskontext löscht.

Das Akzeptieren eines Plans gibt der Sitzung auch einen [generierten Titel](/docs/de/sessions#name-your-sessions) basierend auf dem Plan, es sei denn, Sie haben die Sitzung bereits benannt.

<h3 id="set-plan-mode-as-the-default">
  Legen Sie den Plan-Modus als Standard fest
</h3>

Um den Plan-Modus als Standard für die Terminal-Sitzungen eines Projekts festzulegen, setzen Sie `defaultMode` auf `plan` in `.claude/settings.json`, wie das Beispiel unter [Starten Sie in einem anderen Berechtigungsmodus](#start-in-a-different-mode) zeigt. Konversationen, die die [VS Code-Erweiterung](/docs/de/vs-code) startet, lesen keine Projekteinstellungen für den Standard-Berechtigungsmodus. Dort setzen Sie stattdessen `claudeCode.initialPermissionMode` auf `plan` in Ihren VS Code-Benutzereinstellungen.

<h2 id="eliminate-prompts-with-auto-mode">
  Eliminieren Sie Genehmigungsaufforderungen mit dem Auto-Modus
</h2>

Der Auto-Modus ermöglicht Claude, ohne routinemäßige Genehmigungsaufforderungen auszuführen. Ein separates Klassifizierungsmodell überprüft Aktionen vor ihrer Ausführung und blockiert alles, das über Ihre Anfrage hinausgeht, auf nicht erkannte Infrastruktur abzielt oder von feindseligem Inhalt angetrieben zu sein scheint, den Claude gelesen hat. Explizite [Ask-Regeln](/docs/de/permissions#manage-permissions) erzwingen weiterhin eine Aufforderung.

In Pro-, Max- und Team-Plänen ist der Auto-Modus der [integrierte Standard-Genehmigungsmodus](#which-mode-a-session-starts-in).

Der Klassifizierer überprüft auch jede Nachricht, die Claude mit [`SendMessage`](/docs/de/tools-reference) an einen anderen Agent sendet, ob als Klartext oder als strukturierte [Agent-Team](/docs/de/agent-teams)-Nachricht, bevor Claude Code sie liefert, sowohl im Auto-Modus als auch im [Plan-Modus, während der Klassifizierer Befehle überprüft](#analyze-before-you-edit-with-plan-mode); die Sendungsüberprüfung erfordert Claude Code v2.1.222 oder später.

Der Klassifizierer überprüft und genehmigt oder blockiert auch `rm`- und `rmdir`-Löschungen, die auf einen [kritischen Pfad](#critical-paths) abzielen, wie `rm -rf /` und `rm -rf ~`, auch wenn die Löschung in einer Befehl- oder Prozessersetzung stattfindet.

Der Auto-Modus ermutigt Claude auch, weiter zu arbeiten, ohne bei Klärungsfragen zu stoppen, obwohl Claude immer noch fragt, wenn Ihre Eingabeaufforderung oder eine Fähigkeit explizit darauf angewiesen ist. Für stärkeres autonomes Verhalten in einem Modus, der Sie immer noch auffordert, stellen Sie stattdessen den [Proaktiven Ausgabestil](/docs/de/output-styles) ein.

<Warning>
  Der Auto-Modus reduziert Genehmigungsaufforderungen, garantiert aber keine Sicherheit. Verwenden Sie ihn für Aufgaben, bei denen Sie der allgemeinen Richtung vertrauen, nicht als Ersatz für die Überprüfung bei sensiblen Operationen.
</Warning>

Der Auto-Modus ist nur verfügbar, wenn Ihr Konto alle diese Anforderungen erfüllt:

* **Plan**: Alle Pläne.
* **Organisation**: Bei Team und Enterprise ist der Auto-Modus standardmäßig verfügbar. Administratoren können ihn für die Organisation deaktivieren, indem sie `permissions.disableAutoMode` in [verwalteten Einstellungen](/docs/de/managed-settings) auf `"disable"` setzen.
* **Modell**: In der Anthropic API und [Claude Platform on AWS](/docs/de/claude-platform-on-aws), Claude Opus 4.6 oder später, Sonnet 4.6 oder später, oder ein [Fable-Modell](/docs/de/model-config#work-with-fable). Bei Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry und angemeldeten [Claude-Apps-Gateway](/docs/de/claude-apps-gateway)-Sitzungen nur Claude Sonnet 5, Opus 4.7 oder später und die Fable-Modelle. Ältere Modelle, einschließlich Sonnet 4.5, Opus 4.5, Haiku und claude-3-Modelle, werden auf keinem Anbieter unterstützt.
* **Anbieter**: Standardmäßig verfügbar in der Anthropic API, Claude Platform on AWS, Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry und angemeldeten Claude-Apps-Gateway-Sitzungen.

Wenn Claude Code meldet, dass der Auto-Modus nicht verfügbar ist, überprüfen Sie zunächst diese Anforderungen und ob eine Einstellungsdatei [`disableAutoMode`](/docs/de/settings-reference#disableautomode) setzt. Anthropic kann den Auto-Modus auch serverseitig deaktiviert haben, oder der Server kann den Auto-Modus für Ihr Konto abgelehnt haben. Eine Sitzung, die eine dieser Antworten erhalten hat, behält den Auto-Modus aus, bis die Sitzung endet. Starten Sie später eine neue Sitzung.

Eine separate Nachricht, die ein Modell benennt und sagt, dass der Auto-Modus die Sicherheit einer Aktion „nicht bestimmen kann", bedeutet, dass eine Klassifizierer-Anfrage fehlgeschlagen ist. Dieser Fehler ist normalerweise vorübergehend, kann aber bei Amazon Bedrock wiederholt auftreten, bis Ihr Konto das benannte Modell aufrufen kann. Siehe die [Fehlerreferenz](/docs/de/errors#auto-mode-cannot-determine-the-safety-of-an-action) für die Ursachen und was zu tun ist.

Wenn Sie `defaultMode: "auto"` in [Einstellungen](/docs/de/settings-reference#all-settings) setzen und eine Terminal-Sitzung im manuellen Modus ohne Fehler startet, befindet sich die Einstellung wahrscheinlich in `.claude/settings.json` oder `.claude/settings.local.json`. `auto` wird aus diesen Dateien nicht wirksam. Verschieben Sie es zu `~/.claude/settings.json`. Für ein Gespräch, das die VS Code-Erweiterung gestartet hat, überprüfen Sie stattdessen die eigene Liste der Erweiterung in [Genehmigungsmodi wechseln](#switch-permission-modes).

<h3 id="enable-auto-mode-on-bedrock-agent-platform-or-foundry">
  Auto-Modus auf Bedrock, Agent Platform oder Foundry
</h3>

Bei [Amazon Bedrock](/docs/de/amazon-bedrock), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai), [Microsoft Foundry](/docs/de/microsoft-foundry) und angemeldeten [Claude-Apps-Gateway](/docs/de/claude-apps-gateway)-Sitzungen wird der Auto-Modus standardmäßig im `Shift+Tab`-Zyklus angezeigt. Das Erscheinen im Zyklus ändert nicht den Genehmigungsmodus, in dem eine Sitzung startet: Bei diesen Anbietern starten Terminal-Sitzungen in Ihrem [`defaultMode`](/docs/de/settings-reference#permissions-defaultmode), der manuell ist, es sei denn, Sie ändern ihn, und Gespräche in der [VS Code-Erweiterung](/docs/de/vs-code) starten im manuellen Modus, es sei denn, `claudeCode.initialPermissionMode` oder ein Modus, den Sie in der Erweiterung ausgewählt haben, setzt einen. Nur Claude Sonnet 5, Opus 4.7 oder später und die Fable-Modelle werden auf diesen Anbietern unterstützt.

Um den Auto-Modus zum Standard-Startgenehmigungsmodus zu machen, setzen Sie `"permissions": {"defaultMode": "auto"}` in Benutzer- oder verwalteten Einstellungen. Wählen Sie in Sitzungen, die die VS Code-Erweiterung startet, stattdessen **Auto** aus dem Modusindikator. [Genehmigungsmodi wechseln](#switch-permission-modes) behandelt, was diese Auswahl übertrumpft.

Die [`/doctor`](/docs/de/commands#all-commands)-Überprüfung schlägt diesen Benutzereinstellungs-Standard auf diesen Anbietern auf die gleiche Weise vor wie auf der Anthropic API.

Um Entwickler daran zu hindern, den Auto-Modus zu verwenden, setzen Sie `disableAutoMode` in [verwalteten Einstellungen](/docs/de/managed-settings) auf `"disable"`. Dies entfernt `auto` aus dem `Shift+Tab`-Zyklus, und eine Sitzung, die mit `--permission-mode auto` gestartet wurde, startet stattdessen im manuellen Modus. Eine bereits im Auto-Modus laufende Sitzung verlässt ihn, wenn die Einstellung diese Sitzung von einer [von Admin bereitgestellten Quelle](/docs/de/managed-settings#which-managed-source-claude-code-uses) erreicht, und zeigt `auto mode disabled by settings`. Vor v2.1.251 behielt eine laufende Sitzung den Auto-Modus bis zum Ende.

In v2.1.158 bis v2.1.206 war der Auto-Modus auf diesen Anbietern aus, bis Sie `CLAUDE_CODE_ENABLE_AUTO_MODE=1` setzten, und Claude Code ignorierte `defaultMode: "auto"` auf diesen Anbietern, es sei denn, die Variable war auch gesetzt. Die Variable wird weiterhin aus Kompatibilitätsgründen akzeptiert und hat ab v2.1.207 keine Auswirkung.

<h3 id="server-side-classifier-review">
  Serverseitige Klassifizierer-Überprüfung
</h3>

Im Auto-Modus kann Claude Code den Server bitten, die Aktionen zu überprüfen, die [die Entscheidungsreihenfolge](#how-the-classifier-evaluates-actions) zur Überprüfung sendet, als Teil der Modellabfragen der Sitzung, anstatt seine eigenen Klassifizierer-Anfragen zu senden. Diese Sitzungen fragen:

* **Eine direkte Verbindung zur Anthropic API**: in einer interaktiven Terminal-Sitzung, auf jedem claude.ai-Plan und auf Konten, die die Claude API verwenden, wenn Anthropic dies ausrollt. Erfordert Claude Code v2.1.271 oder später auf Pro-, Max- und Team-Plänen und v2.1.278 oder später auf Enterprise-Plänen und Claude API-Konten. Ab v2.1.282 fragt eine Sitzung, die [Feature-Flags nicht abruft](/docs/de/env-vars#features-that-need-feature-flag-fetching), zum Beispiel weil Sie Telemetrie deaktiviert haben, den Server standardmäßig in jeder Art von Sitzung.
* **Ein Cloud-Anbieter, oder ein LLM-Gateway oder Proxy**: auf [Claude Platform on AWS](/docs/de/claude-platform-on-aws), Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry, und wenn Sie `ANTHROPIC_BASE_URL` auf ein [LLM-Gateway oder einen Proxy](/docs/de/llm-gateway) verweisen, unabhängig von Ihrem Plan. Das standardmäßige Fragen erfordert Claude Code v2.1.278 oder später.
* **Eine angemeldete [Claude-Apps-Gateway](/docs/de/claude-apps-gateway)-Sitzung**: erfordert Claude Code v2.1.280 oder später

Wo der Server die Aktionen überprüft, entscheiden seine Urteile diese. Zwei andere Ergebnisse sind möglich:

* **Der Server überprüft die Sitzung nicht**: Eine Antwort wird ohne Überprüfungsergebnisse abgeschlossen, oder der Server antwortet, dass er diese Sitzung nicht überprüft. Die häufigsten Ursachen sind ein LLM-Gateway oder Proxy, der die Anfrage zur Überprüfung oder die Ergebnisse blockiert, und eine Plattform, Region oder Anmeldedaten, die noch keine serverseitigen Überprüfungen haben. Claude Code fällt auf seine eigenen Klassifizierer-Anfragen zurück. Sobald dieses Fallback für den Rest der Sitzung gilt, zeigt es eine [Mitteilung über Klassifizierer-Anfrage-Gebühren](/docs/de/auto-mode-classifier-billing) auf Konten, bei denen diese Anfragen abgerechnet werden.
* **Der Server gibt kein Urteil für eine Aktion**: Claude Code verweigert die Aktion, anstatt sie unüberprüft auszuführen. Bei jeder Verbindung geschieht dies, wenn die Antwort endet, bevor die Überprüfungsergebnisse ankommen, oder die Ergebnisse in einer Form ankommen, die Claude Code nicht lesen kann. Ein LLM-Gateway oder Proxy, der Antworten kürzt oder die Ergebnisse umschreibt, kann beides verursachen. Bei einer direkten Verbindung zur Anthropic API geschieht es auch, wenn die Überprüfung des Servers für die Aktion fehlschlägt, zum Beispiel durch Timeout. [Der Server hat kein Sicherheitsurteil zurückgegeben](/docs/de/errors#the-server-returned-no-safety-verdict) behandelt die Verweigerungsmitteilung, was passiert, wenn Verweigerungen wiederholt werden, und was zu tun ist.

Um das Fragen des Servers zu überspringen und immer Claude Codes eigene Klassifizierer-Anfragen zu verwenden, setzen Sie [`CLAUDE_CODE_AUTO_MODE_SERVER=0`](/docs/de/env-vars). Bei einer direkten Verbindung zur Anthropic API erfordert die Variable Claude Code v2.1.281 oder später. Das Setzen auf `1` dort schaltet die Serverseitige Überprüfung in einer Sitzung ein, die sie noch nicht hat, wie eine `-p`- oder Agent SDK-Sitzung, es sei denn, Sie haben auch `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` gesetzt. Wenn Sie `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` setzen und `CLAUDE_CODE_AUTO_MODE_SERVER` nicht gesetzt lassen, stoppt Claude Code auch das Fragen des Servers.

<h3 id="what-the-classifier-blocks-by-default">
  Was der Klassifizierer standardmäßig blockiert
</h3>

Der Klassifizierer vertraut Ihrem Arbeitsverzeichnis und den Remotes, die dafür konfiguriert wurden, als die Sitzung startete. Ein Remote, das während der Sitzung mit `git remote add` oder `git remote set-url` hinzugefügt oder umgeleitet wird, wird nicht vertraut, und alles andere wird als extern behandelt, bis Sie [vertrauenswürdige Infrastruktur konfigurieren](/docs/de/auto-mode-config). Vor v2.1.200 wurden auch während der Sitzung hinzugefügte Remotes vertraut.

**Standardmäßig blockiert**:

* Herunterladen und Ausführen von Code, wie `curl | bash`
* Senden sensibler Daten an externe Endpunkte
* Produktionsbereitstellungen und Migrationen
* Massenlöschung auf Cloud-Speicher
* Gewährung von IAM- oder Repo-Berechtigungen
* Änderung gemeinsamer Infrastruktur
* Irreversibles Zerstören von Dateien, die vor der Sitzung existierten
* Force Push
* Committen oder Pushen einer Änderung, die Geheimnisse oder sensible Daten außerhalb des Repositorys sendet, wenn es ausgeführt wird, oder erweitert, was eine Bereitstellung offenlegt. Dies umfasst einen CI-Workflow oder eine Bereitstellungskonfiguration, die ein Geheimnis an ein Ziel übergibt, das es nicht bereits erhält, ein Skript oder einen Einrichtungsschritt, der einen Geheimnisspeicher liest und die Daten sendet, und eine Konfigurationsänderung, die erweitert, was eine Bereitstellung veröffentlicht, wie eine Registrierung, Sichtbarkeit, ein Artefakt oder eine Sourcemap-Einstellung. Die Überprüfung gilt auf jedem Branch, gilt auch wenn das Repository öffentlich ist, und wird ausgelöst, wenn die Änderung committed oder gepusht wird, unabhängig davon, ob dieser Commit oder Push die Pipeline auslöst; das Löschen erfordert die Benennung des Ausführungseffekts, nicht nur des Commits oder Pushes. Vor v2.1.211 war diese Überprüfung auf den Standard-Branch beschränkt: Ein Push dort wurde blockiert, wenn er sensible Inhalte trug, Änderungen verborgen oder falsch beschrieben waren im Vergleich zu dem, was Sie fragten, Inhalte von außerhalb des Repositorys portiert wurden oder eine Überprüfung umgangen wurde, die Sie fragten
* `git reset --hard`, `git checkout -- .`, `git restore .`, `git clean -fd`, `git stash drop` oder `git stash clear`, von denen der Klassifizierer annimmt, dass sie nicht committete Änderungen verwerfen würden
* `git commit --amend`, wenn der Commit am HEAD nicht in dieser Sitzung erstellt wurde
* Ab v2.1.198, `git commit --amend`, wenn der Commit am HEAD bereits gepusht wurde. Eine nur-Nachricht-Umformulierung wird nicht blockiert: `--amend -m` ohne neu Gestaffeltes, auf einem Commit, den Claude während dieser Sitzung erstellt hat
* `terraform destroy`, `pulumi destroy`, `cdk destroy` oder `terragrunt destroy`, und Anwendung eines Plans, der Ressourcen zerstört

Claude Code v2.1.195 und später blockieren standardmäßig mehr Kategorien. Mehrere hängen von [Umgebungs](/docs/de/auto-mode-config#define-trusted-infrastructure)-Einträgen ab, wie sensible Remote-Ziele und geschützte IaC-Bereiche, die Sie auf konkrete Namen eingrenzen können.

* Schreiben in einen Geheimnisspeicher oder Ändern von DNS-Einträgen oder TLS-Zertifikaten
* Zusammenführen eines Pull Requests, den kein Mensch genehmigt hat, Genehmigung von Claudes eigenem Pull Request oder Deaktivierung von CI-Überprüfungen
* Posten eines Kommentars, der selbst ein Befehl für Automatisierung ist, wie `atlantis apply` oder ein Bot's `/deploy` oder `/merge`
* Umschalten, Ramping oder Löschen eines Produktions-Feature-Flags
* Anwendung von Infrastrukturänderungen auf einen geschützten IaC-Bereich oder Entwässerung und Entfernung von Cluster-Knoten
* Schreibvorgänge auf einen gemeinsamen Compute-Cluster, die über die benannte Ressource hinausgehen, wie ein Label-Selektor oder `--all`, der andere Benutzer's Jobs erfasst
* Erstellen von Kubernetes-Ressourcen, die auf jedem Knoten ausgeführt werden oder Cluster-Verkehr abfangen, wie DaemonSets und Admission Webhooks
* Interaktive Shells oder Port-Forwards in ein sensibles Remote-Ziel
* Öffnen eines Tunnels oder einer Reverse Shell, die einen lokalen Service vom öffentlichen Internet erreichbar macht
* Drucken einer Live-Anmeldedaten oder eines Tokens in das Transkript oder eine Datei
* Zugriff auf einen Ort, der in Ihrer [Umgebung](/docs/de/auto-mode-config#define-trusted-infrastructure) als sensible Datenlocation aufgelistet ist, oder Kopieren von Daten daraus. Ab v2.1.198 blockiert dies auch das Senden von Daten von einem zu einer Zielgruppe, die der Eintrag ausschließt
* Umleitung einer Paketinstallation um Ihre interne Paketregistrierung zu einer öffentlichen Registrierung. Ab v2.1.198 gilt dies auch, wenn Sie Claude in der Konversation mitgeteilt haben, dass eine interne Registrierung oder ein Mirror existiert, nicht nur wenn eine in Ihrer Umgebung aufgelistet ist
* Ausführung eines Befehls mit einem Flag, das einen Sicherheitsschutz deaktiviert, wie `--insecure`
* Starten einer autonomen Agent-Schleife, die ohne menschliche Genehmigung oder einen Sandbox läuft, wie eine mit `--dangerously-skip-permissions` oder `--no-sandbox` gestartete. Ab v2.1.198 umfasst dies auch das Ausführen eines Drittanbieter-Agenten oder Eval-Harness mit deaktivierter Isolation und Pro-Aktion-Genehmigung, wie ein Runner, der mit `--yes-always` gestartet wurde
* [Claude in Chrome](/docs/de/chrome) Browser-Aktionen, die Seiteninhalte, Cookies oder Anmeldedaten off-origin senden könnten

Claude Code v2.1.198 und später blockieren diese standardmäßig auch:

* Löschen von Dateien in `/tmp`, `$TMPDIR` oder einem anderen gemeinsamen Scratch- oder Cache-Verzeichnis nach Wildcard, Glob oder Altersfilter statt nach einem spezifischen benannten Pfad
* Einbeziehen sensibler Details in Inhalte, die gesendet, hochgeladen, veröffentlicht oder an andere Personen oder gemeinsame Systeme geschrieben werden, wenn Ihre eigene Nachricht diese Details nicht für diesen Empfänger autorisiert hat. PR- und Issue-Bodies, Commit-Nachrichten und Kommentare zählen als diese Art von ausgehendem Inhalt, wenn das Repository außerhalb der Vertrauensgrenze oder öffentlich ist, einschließlich der eigenen öffentlichen Repositorys Ihrer Organisation; interne Dateipfade, Code-Namen, Live-API-Antwortdaten wie E-Mails oder Kontobezeichner und Infrastruktur-Bezeichner zählen als sensible Details. Die PR-, Issue- und Commit-Nachricht-Scoping erfordert Claude Code v2.1.200 oder später. Live-Personendaten aus einer API-Antwort in einem PR- oder Issue-Body, wie eine E-Mail-Adresse, ein Konto- oder Organisationsbezeichner oder eine Nutzungsmetrik, erfordert, dass Sie diese Details und den Empfänger benennen, unabhängig von der Sichtbarkeit oder Vertrauensgrenze des Repositorys. Diese Überprüfung erfordert Claude Code v2.1.203 oder später
* Senden von Tastenanschlägen an Claude Codes eigenen tmux-Bereich, um seine eigene Schnittstelle zu steuern, was der Klassifizierer als Claude behandelt, das seine eigenen Berechtigungen oder Aufsicht ändert

Claude Code v2.1.200 und später blockieren diese standardmäßig auch:

* Auskommentieren, Löschen oder Force-Passing eines Tests oder einer Assertion, die Sicherheitsverhalten schützt, wie Authentifizierung, Zugriffskontrolle, Eingabevalidierung oder Sandboxing
* Löschen oder Abbau einer zustandsbehafteten Ressource, die Claude nicht in der Sitzung erstellt hat, wenn keine spezifischere Löschregel gilt und Sie diese Ressource nicht benannt haben
* Umleitung einer API-Basis-URL, eines Proxy-Endpunkts, eines Webhook-Empfängers oder eines Registrierungs-Mirrors auf einen Drittanbieter-Host, der nicht zur Aufgabe passt, auch in Beispieldateien wie `.env.example`
* Änderung, wohin Pushes mit `git remote set-url` oder `git remote add` gehen, es sei denn, Sie haben den neuen Remote benannt
* Pushen von Geheimnissen oder persönlichen oder anvertrauten Daten in ein Repository, das bekannt ist, öffentlich zu sein, oder Pushen von vertraulichem Material dorthin, das nicht Teil der eigenen Arbeit dieses Repositorys ist. Ein Dotfiles-Repository's eigenes Thema ist die einzige Ausnahme für persönliche oder anvertraute Daten, und Inhalte aus einem privaten Repository, die eine öffentliche Oberfläche erreichen, werden auf die gleiche Weise blockiert; beide Verfeinerungen erfordern Claude Code v2.1.203 oder später. Vor v2.1.203 wurden persönliche Daten mit vertraulichem Material gruppiert und nur blockiert, wenn sie nicht Teil der eigenen Arbeit dieses Repositorys waren. Wenn die Sichtbarkeit eines Repositorys nicht etabliert ist, blockiert der Klassifizierer nicht allein darauf; er beurteilt den Inhalt stattdessen gegen die anderen Regeln
* Öffnen eines Pull Requests gegen ein anderes Repository oder eine andere Organisation, Forking mit `gh repo fork` oder Pushen in ein Drittanbieter-Repository, es sei denn, Sie haben dieses externe Ziel benannt

Claude Code v2.1.203 und später blockieren diese standardmäßig auch:

* Inhalte aus einem sensiblen lokalen Speicher oder aus einer Datei, deren Name, Pfad oder Typ sie als sensibel markiert, die in einen Commit, einen Push, PR- oder Issue-Text, einen Gist oder Paste oder eine Paketveröffentlichung eingehen, es sei denn, Sie haben sowohl die Quelle als auch das Ziel benannt. Sitzungstranskripte und Gesprächsprotokolle, Anmeldedaten und Konfigurationspunkt-Ordner wie SSH-Schlüssel, Cloud-Anmeldedaten, Browser-Profile und Shell-Verlauf sowie Benutzer-Daten-Exporte zählen alle, und das Repository ist privat, löscht es nicht

Claude Code v2.1.205 und später blockieren diese standardmäßig auch:

* Schreiben in Claude Code-Sitzungstranskripte, die `.jsonl`-Verlaufsdateien unter `~/.claude/projects/` oder Ihrem konfigurierten Konfigurationsverzeichnis, ob direkt oder durch einen Shell-Befehl. Die Regel umfasst auch die Metadatenzeilen, die Claude Code an jeden Transkripteintrag für seine eigenen Überprüfungen anhängt. Das Lesen eines Transkripts wird nicht blockiert
* Eine rekursive erzwungene Löschung wie `rm -rf "$VAR"` oder `Remove-Item -Recurse -Force $dir`, deren Ziel eine Shell-Variable ist, oder ein Glob, der an einer verwurzelt ist, die nirgendwo in der Konversation zugewiesen ist, die der Klassifizierer sieht. Der Wert kam nur aus früherer Befehlsausgabe, die der Klassifizierer nie erhält, daher kann der Klassifizierer das Löschziel nicht gegen die anderen Löschregeln überprüfen. Der Block wird gelöscht, wenn Sie den genauen gelöschten Pfad benennen, oder wenn Claude die Löschung mit dem aufgelösten Literalpfad, der in den Befehl geschrieben ist, erneut ausführt. Löschungen, deren Ziel der Klassifizierer auflösen kann, sind nicht betroffen. `Remove-Item`-Ziele, die ein bloßes `*` sind oder auf `/*` oder `\*` enden, erreichen den Klassifizierer nie: Claude Code [lehnt sie direkt ab](#remove-item-in-powershell)

Claude Code v2.1.257 und später blockieren diese standardmäßig auch:

* Anfordern von Anmeldedaten vom Cloud-Instance-Metadaten-Endpunkt, wie `169.254.169.254`, oder explizites Authentifizieren eines Cloud-, Cluster- oder Registrierungsaufrufs mit der eigenen Service-Account oder Node-Identität der Maschine
* Erreichen eines öffentlichen Hosts durch eine Route, die nicht direkt ist, wie ein Tunnel, eine Reverse Shell oder eine Resolver- oder Proxy-Konfiguration, die umgeschrieben wurde, um außerhalb zu verweisen
* Lesen von Anmeldedaten, die dem Host gehören, nicht zu Ihrer Aufgabe, wie Node-Zertifikate oder die Container-Registry-Auth des Knotens
* Verbindung zu oder Scannen von Sibling-Containern, Pods oder VMs, die Claude nicht gestartet hat, oder dem Knoten unter dem Container

Wenn Claude Code irgendwo läuft, das eines davon erlauben soll, beschreiben Sie dieses Setup in einem [Host-Containment-Eintrag](/docs/de/auto-mode-config#define-trusted-infrastructure) in `autoMode.environment`.

Claude Code v2.1.261 und später blockieren diese standardmäßig auch:

* Posten oder Schreiben eines Links zu einem öffentlichen Paste-, Diagramm- oder Datenaustausch-Service in einer Nachricht, PR- oder Issue-Text, einem Dokument oder überall dort, wo der Link geöffnet oder abgerufen wird, wenn die URL selbst den geteilten Inhalt trägt, es sei denn, Sie haben diesen Service benannt

**Standardmäßig erlaubt**:

* Lokale Dateivorgänge in Ihrem Arbeitsverzeichnis
* Installation von Abhängigkeiten, die in Ihren Lock-Dateien oder Manifesten deklariert sind
* Lesen von `.env` und Senden von Anmeldedaten an ihre passende API
* Schreibgeschützte HTTP-Anfragen
* Pushen zu jedem Branch des Repositorys, an dem Sie arbeiten, einschließlich des Standard-Branchs. Ein nicht-Standard-Branch, dessen Name ihn als Deploy- oder Veröffentlichungsziel markiert, wie `production` oder `gh-pages`, ist nicht abgedeckt: Der Klassifizierer beurteilt einen Push dort auf seine eigenen Bedingungen. Der Inhalt des Pushes wird immer noch gegen die anderen Regeln überprüft, [`permissions.deny`-Regeln](/docs/de/permissions#manage-permissions) können Push-Befehle immer noch [wie geschrieben](/docs/de/permissions#bash-rule-limits) in jedem Modus blockieren, und der eigene Branch-Schutz des Remotes gilt immer noch. Vor v2.1.211 waren nur Pushes zum Branch, auf dem Sie starteten, Branches, die Claude erstellt hatte, und routinemäßige Pushes zum Standard-Branch standardmäßig erlaubt, und vor v2.1.203 war jeder direkte Push zum Standard-Branch blockiert

Claude Code v2.1.195 und später erlauben diese standardmäßig auch:

* Löschen der genauen Jobs, die Claude früher in der gleichen Sitzung erstellt hat
* Lesen, Überprüfen oder Schreiben von sicherheitsbezogenem Code, Konfigurationen und Bedrohungsmodellen als Teil Ihrer Aufgabe
* Nachrichten zwischen Agenten, die zusammen in der gleichen Multi-Agent-Sitzung arbeiten
* Senden von Daten an die vertrauenswürdigen Domänen, Buckets und Services, die Sie in [`environment`](/docs/de/auto-mode-config#define-trusted-infrastructure) auflisten. Dies umfasst nur Datenfluss, nicht destruktive oder Anmeldedaten-Operationen auf der gleichen Infrastruktur
* [Claude in Chrome](/docs/de/chrome) Navigation zu einer vertrauenswürdigen internen Domäne, localhost oder einer URL, die Sie benannt haben

Sandbox-Befehle erhalten standardmäßig keinen Netzwerkzugriff. Claude benennt die Hosts, die ein Befehl auf dem Befehl selbst benötigt, der Klassifizierer überprüft sie mit dem Befehl, und eine genehmigte Liste öffnet diese Hosts nur für diesen einen Befehl. [Pro-Befehl erlaubte Domänen](/docs/de/sandboxing#per-command-allowed-domains-in-auto-mode) behandelt, was eine Liste öffnen kann und nicht und was passiert, wenn ein Befehl nach einem nicht aufgelisteten Host greift.

Führen Sie `claude auto-mode defaults` aus, um die vollständigen Regellisten als JSON zu drucken. Wenn routinemäßige Aktionen blockiert werden, kann ein Administrator vertrauenswürdige Repos, Buckets und Services über die `autoMode.environment`-Einstellung hinzufügen: siehe [Auto-Modus konfigurieren](/docs/de/auto-mode-config).

Pushen zu jedem Branch des Repositorys, an dem Sie arbeiten, und Erstellen eines Pull Requests, der Ihrer Anfrage entspricht, laufen ohne Aufforderung, es sei denn, der Push oder Pull Request fällt unter die [blockierte Liste](#what-the-classifier-blocks-by-default), wie Geheimnisse oder sensible Daten, die das Repository verlassen, oder ein Pull Request, der auf ein anderes Repository oder eine andere Organisation abzielt. Um einen menschlichen Checkpoint vor diesen Befehlen zu erfordern, während Sie im Auto-Modus bleiben, fügen Sie `permissions.ask`-Regeln hinzu, die den Befehl [wie geschrieben](/docs/de/permissions#bash-rule-limits) abgleichen: siehe [Häufige Grenzen](/docs/de/auto-mode-config#common-boundaries).

<h3 id="first-read-outside-the-working-directories">
  Der erste Lesezugriff außerhalb der Arbeitsverzeichnisse
</h3>

Während [`permissions.blockReadsOutsideWorkingDirectories`](/docs/de/settings-reference#permissions-blockreadsoutsideworkingdirectories) aus ist, laufen Dateileseoperationen ohne Aufforderung im Auto-Modus, einschließlich Lesezugriffe außerhalb der [Arbeitsverzeichnisse](/docs/de/permissions#working-directories). Das erste Mal, wenn Claude das Read-, Grep- oder Glob-Tool auf einen Pfad außerhalb davon verwendet, fragt Claude Code Sie, ob Sie diese Lesezugriffe weiterhin erlauben möchten.

Die Aufforderung wird nicht in nicht-interaktiven `-p`-Läufen oder Hintergrund-Sitzungen angezeigt; Lesezugriffe dort laufen wie zuvor.

Was auch immer Sie antworten, Claude arbeitet weiter:

* **Weiterhin erlauben**: Der Lesezugriff läuft, spätere Lesezugriffe außerhalb der Arbeitsverzeichnisse laufen wie zuvor, und Claude Code zeichnet Ihre Antwort auf, damit die Aufforderung nicht wieder angezeigt wird
* **Von jetzt an blockieren**: Der Lesezugriff wird verweigert, und Claude Code setzt [`permissions.blockReadsOutsideWorkingDirectories`](/docs/de/settings-reference#permissions-blockreadsoutsideworkingdirectories) in Ihren Benutzereinstellungen auf `true`, was die Dateiwerkzeuge dazu bringt, solche Lesezugriffe in jeder späteren Sitzung und jedem Genehmigungsmodus zu verweigern. Um Claude später einen solchen Pfad lesen zu lassen, fügen Sie sein Verzeichnis mit `/add-dir` hinzu oder entfernen Sie die Einstellung.
* **Nächstes Mal erneut fragen**: Der Lesezugriff wird verweigert, und der nächste Lesezugriff außerhalb der Arbeitsverzeichnisse fordert erneut auf

<h3 id="boundaries-you-state-in-conversation">
  Grenzen, die Sie in der Konversation angeben
</h3>

Der Klassifizierer behandelt Grenzen, die Sie in der Konversation angeben, als Blocksignal. Wenn Sie Claude sagen „pushe nicht" oder „warte, bis ich überprüfe, bevor ich bereitstelle", blockiert der Klassifizierer passende Aktionen, auch wenn die Standardregeln sie erlauben würden. Eine Grenze bleibt in Kraft, bis Sie sie in einer späteren Nachricht aufheben. Claudes eigenes Urteil, dass eine Bedingung erfüllt wurde, hebt sie nicht auf.

Grenzen werden nicht als Regeln gespeichert. Der Klassifizierer liest sie bei jeder Überprüfung aus dem Transkript erneut, daher kann eine Grenze verloren gehen, wenn [Kontext-Komprimierung](/docs/de/costs#reduce-token-usage) die Nachricht entfernt, die sie angegeben hat. Für eine harte Garantie fügen Sie stattdessen eine [Deny-Regel](/docs/de/permissions#permission-rule-syntax) hinzu.

<h3 id="approvals-you-state-in-conversation">
  Genehmigungen, die Sie in der Konversation angeben
</h3>

Wenn Sie Claude sagen, dass eine blockierte Aktion erlaubt ist, liest der Klassifizierer das als Ihre Genehmigung und kann den Block löschen. Wie Sie es formuliert haben, entscheidet, ob die Aktion läuft und wie weit die Genehmigung reicht:

* **Benennen Sie die Aktion und ihre Besonderheiten**: Ihre Nachricht muss die Aktion und das Spezifische benennen, das sie gefährlich macht, wie der Branch eines Force Push. Das Benennen des Verbs allein löscht nichts, daher lässt „Sie können force-pushen" den Block in Kraft.
* **Erwarten Sie, dass es eine Aktion abdeckt**: Eine Genehmigung umfasst die destruktive Aktion, die Sie benannt haben, daher wird eine spätere Aktion wieder blockiert, es sei denn, Sie haben die Genehmigung als stehend gewährt. Um ein routinemäßiges Muster nicht mehr eine Aktion nach der anderen zu genehmigen, fügen Sie es zu [`autoMode.allow`](/docs/de/auto-mode-config#override-the-block-and-allow-rules) hinzu.
* **Einige Blöcke bleiben in Kraft**: [Die Vorrangordnung des Klassifizierers](/docs/de/auto-mode-config#override-the-block-and-allow-rules) legt fest, welche Blöcke Ihre Genehmigung erreichen kann. Um einen Schritt auszuführen, den sie nicht löschen wird, [verlassen Sie den Auto-Modus](#switch-permission-modes) und beantworten Sie die Genehmigungsaufforderung.

<h3 id="when-auto-mode-falls-back">
  Wenn Auto-Modus zurückfällt
</h3>

Wenn der Auto-Modus Ihre Sitzungsaktionen nicht genehmigen kann, hängt das, was passiert, vom Fall ab:

* **Eine blockierte Aktion**: Claude Code zeigt eine Benachrichtigung und listet die Aktion in `/permissions` unter der Registerkarte **Kürzlich verweigert** auf, wo Sie `r` drücken können, um sie mit einer manuellen Genehmigung erneut zu versuchen. Wenn der Klassifizierer [kein Urteil über die Aktion](/docs/de/errors#auto-mode-cannot-determine-the-safety-of-an-action) erzeugt, weil eine Sicherheitsüberprüfung, die vom Auto-Modus getrennt ist, die Anfrage des Klassifizierers selbst verweigerte oder seine Antwort nicht geparst wurde, verweigert Claude Code die Aktion ohne die Benachrichtigung oder den Eintrag **Kürzlich verweigert**.
* **Wiederholte Blöcke**: Wenn der Klassifizierer eine Aktion 3 Mal hintereinander oder 20 Mal insgesamt blockiert, pausiert der Auto-Modus und Claude Code setzt das Auffordern fort. Das Genehmigen der aufgeforderten Aktion setzt den Auto-Modus fort. Diese Schwellwerte sind nicht konfigurierbar. Jede erlaubte Aktion setzt den aufeinanderfolgenden Zähler zurück, während der Gesamtzähler für die Sitzung bestehen bleibt und nur zurückgesetzt wird, wenn sein eigenes Limit einen Fallback auslöst. Claude Code zählt eine Verweigerung nicht zu einem Schwellwert, wenn [eine Sicherheitsüberprüfung, die vom Auto-Modus getrennt ist, die Anfrage des Klassifizierers verweigert](/docs/de/errors#auto-mode-cannot-determine-the-safety-of-an-action); der verlinkte Eintrag behandelt, wie Claude Code diese Verweigerungen handhabt.
* **Sitzungen, die nicht auffordern können**: Ein [nicht-interaktiver](/docs/de/headless) `-p`-Lauf ohne ein [`--permission-prompt-tool`](/docs/de/cli-reference#cli-flags) hat keine Aufforderung, auf die zurückgegriffen werden kann. Wenn wiederholte Blöcke einen Schwellwert erreichen, läuft die Aktion nicht und Claude arbeitet weiter. Das gleiche gilt, wenn [eine Sicherheitsüberprüfung, die vom Auto-Modus getrennt ist, die Anfrage des Klassifizierers verweigert](/docs/de/errors#auto-mode-cannot-determine-the-safety-of-an-action). Claude Code stoppt den Lauf in keinem Fall.
* **Kein Urteil vom Server**: Unter [serverseitiger Klassifizierer-Überprüfung](#server-side-classifier-review) verweigert Claude Code eine Aktion, für die der Server kein Urteil gibt, und stoppt die Runde nach zehn Antworten hintereinander ohne Urteil. Siehe [Der Server hat kein Sicherheitsurteil zurückgegeben](/docs/de/errors#the-server-returned-no-safety-verdict).
* **Ein Modusschalter während einer Überprüfung**: Wenn Sie Genehmigungsmodi wechseln, während eine Klassifizierer-Überprüfung ausstehend ist, verwirft Claude Code ein Urteil, das der neue Modus nicht angefordert hätte, statt es anzuwenden: Sie werden stattdessen aufgefordert, oder die Aktion wird im [`dontAsk`-Modus](#allow-only-pre-approved-tools-with-dontask-mode) automatisch verweigert.

Wiederholte Blöcke bedeuten normalerweise, dass dem Klassifizierer Kontext über Ihre Infrastruktur fehlt. Verwenden Sie `/feedback`, um falsch positive zu melden, oder lassen Sie einen Administrator [vertrauenswürdige Infrastruktur konfigurieren](/docs/de/auto-mode-config).

<span id="how-the-classifier-evaluates-actions" />

<AccordionGroup>
  <Accordion title="Wie der Klassifizierer Aktionen bewertet">
    Jede Aktion durchläuft eine feste Entscheidungsreihenfolge. Der erste passende Schritt gewinnt:

    1. Aktionen, die Ihren [Allow-, Ask- oder Deny-Regeln](/docs/de/permissions#manage-permissions) entsprechen, werden sofort aufgelöst, mit diesen Ausnahmen:
       * Schreibvorgänge zu [geschützten Pfaden](#protected-paths) werden zum Klassifizierer weitergeleitet, auch wenn eine Allow-Regel passt, und ebenso `rm`- und `rmdir`-Löschungen, die auf einen [kritischen Pfad](#critical-paths) in Claude Code v2.1.218 und später abzielen
       * MCP-Tools, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) markiert sind, fordern Sie direkt auf, auch wenn eine Allow-Regel passt, und ebenso Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) in Sitzungen, in denen diese Einstellung Claude Code erreicht
       * Ein Shell-Befehl, der [pro-Befehl erlaubte Domänen](/docs/de/sandboxing#per-command-allowed-domains-in-auto-mode) trägt, wird auch zum Klassifizierer weitergeleitet, auch wenn eine Allow-Regel passt, weil eine Regel den Befehl genehmigt, nicht seine Hosts
       * Ask-Regeln, die auf Befehlsinhalte abgleichen, wie `Bash(git push *)`, fallen auf eine Genehmigungsaufforderung zurück
    2. Schreibgeschützte Aktionen und Dateieditierungen in Ihrem Arbeitsverzeichnis werden automatisch genehmigt, außer Schreibvorgänge zu [geschützten Pfaden](#protected-paths) und [dem ersten Lesezugriff außerhalb der Arbeitsverzeichnisse](#first-read-outside-the-working-directories), die Sie auffordern
       * In einer Sitzung mit [serverseitiger Klassifizierer-Überprüfung](#server-side-classifier-review) warten schreibgeschützte und [Sandbox](/docs/de/sandboxing#sandbox-modes)-Shell-Befehle auf diese Überprüfung und werden blockiert, wenn sie diese kennzeichnet
    3. Alles andere geht zum Klassifizierer. Die Connector-Tools und `requiresUserInteraction` MCP-Tools, die Sie direkt in Schritt 1 auffordern, erreichen den Klassifizierer nie, daher wird weder eine von der Organisation erforderliche Genehmigung noch ein Zustimmungsschritt automatisch genehmigt
    4. Wenn der Klassifizierer blockiert, erhält Claude den Grund und versucht eine Alternative. In den meisten Sitzungen benennt der Grund die Regel, die der Klassifizierer abgeglichen hat, wie `[Data Exfiltration]`, statt eine schriftliche Erklärung zu geben; siehe [Verweigerungen überprüfen](/docs/de/auto-mode-config#review-denials)

    Beim Eintritt in den Auto-Modus werden breite Allow-Regeln, die willkürliche Code-Ausführung gewähren, gelöscht:

    * Pauschal `Bash(*)` oder `PowerShell(*)`
    * Wildcard-Interpreter wie `Bash(python*)`
    * Paketmanager-Ausführungsbefehle
    * `Agent` Allow-Regeln
    * [`Monitor`](/docs/de/tools-reference#monitor-tool) Allow-Regeln, weil Claude Code Monitor-Befehle durch die Shell ausführt

    Enge Regeln wie `Bash(npm test)` bleiben in Kraft. Claude Code stellt die gelöschten Regeln wieder her, wenn Sie den Auto-Modus verlassen. Vor v2.1.236 ließ Claude Code `Monitor` Allow-Regeln im Auto-Modus in Kraft, daher genehmigte eine Regel, die das ganze Tool abglich, Monitor-Befehle ohne Klassifizierer-Überprüfung.

    Claude Code führt auch `git status` selbst vor einem Befehl aus, der nicht committete Arbeit verwerfen würde, wie `git reset --hard` oder `rm -rf`, und zeigt dem Klassifizierer, ob gestaffelte, geänderte oder nicht verfolgte Arbeit vorhanden ist. Claude Code meldet nicht verfolgte Dateien in dieser Überprüfung, auch wenn die Git-Konfiguration des Repositorys `status.showUntrackedFiles=no` setzt.

    In den Klassifizierer-Anfragen, die Claude Code selbst sendet, sieht der Klassifizierer Benutzernachrichten, Tool-Aufrufe außer schreibgeschützten Lookups wie Dateileseoperationen und Suchen, und Ihren CLAUDE.md-Inhalt. Tool-Ergebnisse werden aus diesen Anfragen entfernt, daher kann feindselige Inhalte in einer Datei oder Webseite den Klassifizierer nicht direkt manipulieren.

    Sie können einen Aufruf's Ergebnis mit einem [PostToolUse Hook's `classifierContext`-Feld](/docs/de/hooks#annotate-a-result-for-the-auto-mode-classifier) annotieren, das der Klassifizierer als von der Anwendung bereitgestellter Kontext liest. Das Feld erfordert Claude Code v2.1.236 oder später.

    Eine separate serverseitige Sonde scannt eingehende Tool-Ergebnisse und kennzeichnet verdächtige Inhalte, bevor Claude sie liest. Für mehr darüber, wie diese Schichten zusammenarbeiten, siehe die [Auto-Modus-Ankündigung](https://claude.com/blog/auto-mode) und den [Engineering Deep Dive](https://www.anthropic.com/engineering/claude-code-auto-mode).
  </Accordion>

  <Accordion title="Wie Auto-Modus Subagenten handhabt">
    Der Klassifizierer überprüft [Subagenten](/docs/de/sub-agents)-Arbeit an drei Punkten:

    1. Bevor ein Subagent startet, wird die delegierte Aufgabenbeschreibung bewertet, daher wird eine gefährlich aussehende Aufgabe beim Spawn blockiert.
    2. Während der Subagent läuft, geht jede seiner Aktionen durch den Klassifizierer mit den gleichen Regeln wie die übergeordnete Sitzung, und jeder `permissionMode` in der Frontmatter des Subagenten wird ignoriert.
    3. Wenn der Subagent fertig ist, überprüft der Klassifizierer seine Arbeit und seinen Abschlussbericht, bevor die übergeordnete Sitzung den Bericht liest. Wenn der Klassifizierer die Arbeit oder den Bericht des Subagenten kennzeichnet, oder eine separate API-Sicherheitsüberprüfung die Überprüfung verweigert, wird der Bericht immer noch geliefert, vorangestellt mit einer Sicherheitswarnung. Wenn der Klassifizierer für die Überprüfung nicht verfügbar ist, kommt der Bericht mit einer Notiz an, um die Arbeit des Subagenten zu überprüfen, bevor Sie darauf handeln.
  </Accordion>

  <Accordion title="Kosten und Latenz">
    Der Klassifizierer läuft standardmäßig auf Claude Sonnet 5 statt auf Ihrer `/model`-Auswahl. Ein Klassifizierer-Modell, das Anthropic serverseitig konfiguriert, hat Vorrang vor diesem Standard. Wenn das Modell Ihrer Sitzung Claude Sonnet 4.6 ist, oder wenn [`availableModels`](/docs/de/model-config#restrict-model-selection) Sonnet 5 ausschließt, läuft der Klassifizierer stattdessen auf dem Modell Ihrer Sitzung oder auf einem Opus-Modell, wenn die Sitzung auf einem [Fable-Modell](/docs/de/model-config#work-with-fable) läuft; bei Anbietern außer der Anthropic API ist dieses Opus-Fallback das Standard-Opus-Modell des Anbieters.

    Die erste Auto-Modus-Anfrage Ihrer Sitzung validiert den Sonnet 5-Standard: Wenn die Anfrage erfolgreich ist, bleibt Sonnet 5 das Klassifizierer-Modell Ihrer Sitzung, und wenn sie fehlschlägt, weil das Modell nicht verfügbar ist, verwendet die Sitzung stattdessen das Fallback. Nachdem diese Validierung sich beruhigt hat, ändert sich das Klassifizierer-Modell nicht für die Sitzung.

    Bei Enterprise-Plänen und Konten, die die Claude API verwenden, [Claude Platform on AWS](/docs/de/claude-platform-on-aws), Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry, zählen Klassifizierer-Aufrufe zu Ihrer Token-Nutzung. Jede Überprüfung sendet einen Teil des Transkripts plus die ausstehende Aktion, was eine Hin- und Rückfahrt vor der Ausführung hinzufügt. Lesezugriffe und Arbeitsverzeichnis-Editierungen außerhalb geschützter Pfade überspringen den Klassifizierer, daher kommt der Overhead hauptsächlich von Shell-Befehlen und Netzwerkoperationen. Wo der Server die Aktionen als Teil der Modellabfragen der Sitzung überprüft, gibt es keine separaten Klassifizierer-Aufrufe zu zählen; siehe [Serverseitige Klassifizierer-Überprüfung](#server-side-classifier-review).

    Sandbox-Netzwerkzugriff fügt keine Pro-Verbindungs-Klassifizierer-Anfragen hinzu. Der Klassifizierer beurteilt [die Hosts, die ein Befehl benennt](/docs/de/sandboxing#per-command-allowed-domains-in-auto-mode) zusammen mit dem Befehl in einer Überprüfung, und Claude Code überprüft jede Verbindung gegen die genehmigte Liste, ohne den Klassifizierer erneut aufzurufen.
  </Accordion>
</AccordionGroup>

<h2 id="allow-only-pre-approved-tools-with-dontask-mode">
  Nur vorab genehmigte Tools mit dontAsk-Modus zulassen
</h2>

Wenn Sie den `dontAsk`-Modus einstellen, lehnt Claude Code automatisch jeden Tool-Aufruf ab, der Sie sonst auffordern würde. Claude führt weiterhin Aktionen aus, die im Manual-Modus keine Genehmigung benötigen, wie z. B. Dateilesevorgänge in Ihren Arbeitsverzeichnissen und [schreibgeschützte Bash-Befehle](/docs/de/permissions#read-only-commands), sowie Aktionen, die Ihren `permissions.allow`-Regeln entsprechen, und Aufrufe, die von einem [PreToolUse-Hook](/docs/de/permissions#extend-permissions-with-hooks) genehmigt wurden. Verwenden Sie diesen Modus für CI-Pipelines oder eingeschränkte Umgebungen, in denen Sie vorab definieren, was Claude tun darf; die Sitzung wartet nie auf Eingaben. Die Statusleiste zeigt `⏵⏵ don't ask on`, während dieser Modus aktiv ist.

Claude Code lehnt Aufrufe ab, die Ihren expliziten [`ask`-Regeln](/docs/de/permissions#manage-permissions) entsprechen, anstatt Sie aufzufordern. Es lehnt auch das integrierte `AskUserQuestion`-Tool ab, selbst wenn Ihre Allow-Regeln damit übereinstimmen, und macht dasselbe mit Connector-Tools, [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools), in Sitzungen, in denen diese Einstellung Claude Code erreicht. Es lehnt MCP-Tools, die mit [`_meta["anthropic/requiresUserInteraction"]`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, auf die gleiche Weise ab, da ihre Genehmigungskarte eine Antwort benötigt, die dieser Modus nie erfasst; dies erfordert Claude Code v2.1.199 oder später.

`rm`- und `rmdir`-Löschungen, die auf einen [kritischen Pfad](#critical-paths) abzielen, wie z. B. `rm -rf /` und `rm -rf ~`, werden abgelehnt, selbst wenn eine Allow-Regel damit übereinstimmt oder ein `PreToolUse`-Hook sie zulässt.

Cloud-Sitzungen auf [Claude Code im Web](/docs/de/claude-code-on-the-web) ignorieren `defaultMode: "dontAsk"`; siehe [bypassPermissions](#skip-all-checks-with-bypasspermissions-mode) für Details.

Stellen Sie es beim Start mit dem Flag ein:

```bash theme={null}
claude --permission-mode dontAsk
```

<h2 id="skip-all-checks-with-bypasspermissions-mode">
  Alle Überprüfungen mit bypassPermissions-Modus überspringen
</h2>

Der `bypassPermissions`-Modus deaktiviert Berechtigungsaufforderungen und Sicherheitsüberprüfungen, sodass Tool-Aufrufe sofort ausgeführt werden, einschließlich Schreibvorgänge in [geschützte Pfade](#protected-paths).

Die [Aktionen, die kein Modus automatisch genehmigt](#actions-no-mode-auto-approves) fordern weiterhin in diesem Modus auf.

Zwei [Cross-Session-Messaging](/docs/de/cross-session-messaging) Schutzmaßnahmen gelten weiterhin in diesem Modus und in interaktiven Terminal-Plan-Mode-Sitzungen, in denen Bypass-Berechtigungen verfügbar sind:

* Die [`isolatePeerMachines`](/docs/de/settings-reference#isolatepeermachines) Genehmigungsaufforderung für Nachrichten an Ihre Sitzungen über diese Maschine hinaus wird weiterhin angezeigt.
* Wenn kein [`crossSessionInbound`](/docs/de/cross-session-messaging#control-inbound-messages) Wert zutrifft, hält Claude Code eine eingehende Nachricht von einer anderen Ihrer Sitzungen für Ihre Genehmigung und liefert ohne Fragen nur, wenn die sendende Sitzung sich selbst als auch Berechtigungsaufforderungen umgehend identifiziert. Wenn Sie den Berechtigungsmodus verlassen, während Nachrichten gehalten werden, wendet Claude Code die eingehenden Regeln erneut an und liefert jede gehaltene Nachricht, die sie jetzt akzeptieren.

In interaktiven Terminal-Sitzungen mit verfügbaren Bypass-Berechtigungen erzwingt Claude Code auch nicht [Plan-Mode's](#analyze-before-you-edit-with-plan-mode) Blockierungen. Claude wird immer noch angewiesen, ohne Bearbeitung zu planen, aber ein Dateibearbeitungs- oder Shell-Befehl, den es während der Planung versucht, läuft ohne Aufforderung. Explizite [Ask-Regeln](/docs/de/permissions#manage-permissions) und `rm` und `rmdir` Löschungen, die auf einen [kritischen Pfad](#critical-paths) abzielen, fordern weiterhin auf.

Der Plan-Mode behält seine Blockierungen überall dort bei, wo Claude Code ohne interaktives Terminal läuft, einschließlich [nicht-interaktiver Ausführungen](/docs/de/headless) mit `-p`, [Agent SDK](/docs/de/agent-sdk/permissions#plan-mode-plan) Sitzungen und Unterhaltungen im Chat-Panel der [VS Code-Erweiterung](/docs/de/vs-code). Dort macht `--allow-dangerously-skip-permissions` `bypassPermissions` später auswählbar.

<Warning>
  Verwenden Sie diesen Modus nur in isolierten Umgebungen wie Containern, VMs oder Dev-Containern ohne Internetzugang, in denen Claude Code Ihr Host-System nicht beschädigen kann.
</Warning>

Sie können `bypassPermissions` nicht aus einer Sitzung eingeben, die Sie ohne ihn gestartet haben. Aktivieren Sie es beim Start mit [`permissions.defaultMode: "bypassPermissions"`](/docs/de/settings-reference#permissions-defaultmode) oder mit einem aktivierenden Flag:

```bash theme={null}
claude --permission-mode bypassPermissions
```

Das Flag `--dangerously-skip-permissions` ist gleichwertig.

Claude Code verweigert `bypassPermissions` in einer Sitzung, die Sie mit [`--restricted`](/docs/de/cli-reference#cli-flags) starten. `--restricted` erfordert Claude Code v2.1.248 oder später.

Das erste Mal, wenn Sie eine interaktive Sitzung mit diesem Modus aktiviert starten, zeigt Claude Code einen Warnungsdialog, der Sie auffordert, Verantwortung für Aktionen ohne Berechtigungsprüfungen zu übernehmen. Claude Code speichert Ihre Akzeptanz in Benutzereinstellungen, sodass der Dialog nur einmal angezeigt wird. Wenn Sie ablehnen, beendet Claude Code. Im [nicht-interaktiven Modus](/docs/de/headless) wird kein Dialog angezeigt, und eine [Hintergrund-Sitzung](/docs/de/agent-view), die mit `--bg` gestartet wurde, wird verweigert, bis Sie den Dialog in einer interaktiven Sitzung akzeptiert haben.

Auf Linux und macOS weigert sich Claude Code, in diesem Modus zu starten, wenn es als Root oder unter `sudo` ausgeführt wird:

```text theme={null}
--dangerously-skip-permissions cannot be used with root/sudo privileges for security reasons
```

Die Überprüfung wird automatisch in einer erkannten Sandbox übersprungen. Um autonom in einem Container zu laufen, verwenden Sie die [Dev-Container](/docs/de/devcontainer) Konfiguration, die Claude Code als Nicht-Root-Benutzer ausführt.

[Claude Code im Web](/docs/de/claude-code-on-the-web) berücksichtigt `defaultMode: "bypassPermissions"` oder `"dontAsk"` aus Ihren Einstellungsdateien nicht, daher können die eingecheckten Einstellungen eines Repositorys keine Cloud-Sitzung im Bypass-Permissions-Modus starten. Die Einstellung wird stillschweigend ignoriert und die Sitzung startet im Berechtigungsmodus, der im Modusmenü angezeigt wird. Siehe [Berechtigungsmodi wechseln](#switch-permission-modes), welche Modi Cloud-Sitzungen bieten.

<Warning>
  `bypassPermissions` bietet keinen Schutz vor Prompt-Injection oder unbeabsichtigten Aktionen. Verwenden Sie stattdessen den [Auto-Modus](#eliminate-prompts-with-auto-mode) für Hintergrund-Sicherheitsüberprüfungen mit deutlich weniger Berechtigungsaufforderungen. Administratoren können diesen Modus blockieren, indem sie `permissions.disableBypassPermissionsMode` auf `"disable"` in [verwalteten Einstellungen](/docs/de/managed-settings) setzen.
</Warning>

<h2 id="protected-paths">
  Geschützte Pfade
</h2>

Schreibvorgänge in eine kleine Anzahl von Pfaden werden niemals automatisch genehmigt, außer im `bypassPermissions`-Modus und in interaktiven Terminal-Sitzungen im Plan-Modus mit [verfügbaren Bypass-Berechtigungen](#skip-all-checks-with-bypasspermissions-mode). Dies verhindert versehentliche Beschädigungen des Repository-Status und der eigenen Konfiguration von Claude.

| Modus                    | Schreibvorgänge in geschützten Pfaden                                                                                                                                                                                                                                                        |
| :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`, `acceptEdits` | Abgefragt                                                                                                                                                                                                                                                                                    |
| `plan`                   | Erlaubt in interaktiven Terminal-Sitzungen mit [verfügbaren Bypass-Berechtigungen](#skip-all-checks-with-bypasspermissions-mode). Andernfalls zum Klassifizierer geleitet, wenn [Auto-Modus](#eliminate-prompts-with-auto-mode) während der Planung verfügbar ist, und abgefragt, wenn nicht |
| `auto`                   | Zum Klassifizierer geleitet                                                                                                                                                                                                                                                                  |
| `dontAsk`                | Verweigert                                                                                                                                                                                                                                                                                   |
| `bypassPermissions`      | Erlaubt                                                                                                                                                                                                                                                                                      |

In einer Sitzung, die mit [`--restricted`](/docs/de/cli-reference#cli-flags) gestartet wurde, was Claude Code v2.1.248 oder später erfordert, kann der Klassifizierer Schreibvorgänge in geschützten Pfaden nicht genehmigen.

[`permissions.allow`](/docs/de/permissions#manage-permissions) Regeln in Einstellungsdateien genehmigen Schreibvorgänge in geschützten Pfaden nicht im Voraus. Die Sicherheitsprüfung wird ausgeführt, bevor Claude Code die Allow-Regeln aus den Einstellungen auswertet, daher ändert ein Eintrag wie `Edit(.claude/**)` in `~/.claude/settings.json` oder `.claude/settings.json` das Ergebnis pro Modus in der obigen Tabelle nicht. In Modi, die abfragen, bietet die Eingabeaufforderung für einen `.claude/` Schreibvorgang **Ja, und Claude darf seine eigenen Einstellungen für diese Sitzung bearbeiten**, was spätere `.claude/` Schreibvorgänge in dieser Sitzung genehmigt, ohne erneut abzufragen.

Geschützte Verzeichnisse:

* `.git`
* `.config/git`
* `.vscode`
* `.idea`
* `.husky`
* `.cargo`
* `.devcontainer`
* `.yarn`
* `.mvn`
* `.claude`, außer `.claude/worktrees`, wo Claude seine eigenen Git-Worktrees speichert

Geschützte Dateien:

* `.gitconfig`, `.gitmodules`
* `.bashrc`, `.bash_profile`, `.bash_login`, `.bash_aliases`, `.bash_logout`, `.zshrc`, `.zprofile`, `.zshenv`, `.zlogin`, `.zlogout`, `.profile`, `.envrc`
* `.npmrc`, `.yarnrc`, `.yarnrc.yml`, `.pnp.cjs`, `.pnp.loader.mjs`, `.pnpmfile.cjs`, `bunfig.toml`, `.bunfig.toml`
* `.bazelrc`, `.bazelversion`, `.bazeliskrc`
* `.pre-commit-config.yaml`, `lefthook.yml`, `lefthook.yaml`, `.lefthook.yml`, `.lefthook.yaml`
* `gradle-wrapper.properties`, `maven-wrapper.properties`
* `.devcontainer.json`
* `.ripgreprc`, `pyrightconfig.json`
* `.mcp.json`, `.claude.json`

<h2 id="critical-paths">
  Kritische Pfade
</h2>

Claude Code lässt niemals zu, dass eine [`permissions.allow`](/docs/de/permissions#manage-permissions)-Regel oder ein [`PreToolUse`-Hook](/docs/de/permissions#extend-permissions-with-hooks), der `"allow"` zurückgibt, einen `rm`- oder `rmdir`-Befehl genehmigt, der auf einen kritischen Pfad abzielt, auch nicht in Modi, die andere Eingabeaufforderungen überspringen. Dieser Schutzschalter schützt vor Modellfehlern. Eine entsprechende Deny-Regel blockiert den Befehl dennoch vollständig.

Was stattdessen geschieht, hängt von Ihrem Berechtigungsmodus ab:

| Modus                    | Was Claude Code mit einer Entfernung kritischer Pfade tut                                                                                                                                                               |
| :----------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`, `acceptEdits` | Fordert Sie auf, dies zu genehmigen                                                                                                                                                                                     |
| `plan`                   | Fordert Sie auf, dies zu genehmigen. Mit [Auto-Modus verfügbar während der Planung](#analyze-before-you-edit-with-plan-mode) und ohne verfügbare Bypass-Berechtigungen sendet es dies stattdessen an den Klassifizierer |
| `auto`                   | Sendet es an den [Klassifizierer](#eliminate-prompts-with-auto-mode)                                                                                                                                                    |
| `dontAsk`                | Lehnt es ab                                                                                                                                                                                                             |
| `bypassPermissions`      | Fordert Sie auf, dies zu genehmigen                                                                                                                                                                                     |

Wenn eine explizite [Ask-Regel](/docs/de/permissions#manage-permissions) dem Befehl entspricht, fordert Claude Code Sie auch im `auto`-Modus auf. In Modi, die fragen, kann ein [`PermissionRequest`-Hook](/docs/de/hooks#permissionrequest) die Eingabeaufforderung auf die gleiche Weise beantworten wie jede andere.

Claude Code behandelt ein `rm`- oder `rmdir`-Ziel als kritischen Pfad, wenn es eines der folgenden ist:

* Das Dateisystem-Root
* Verzeichnisse der obersten Ebene, d. h. alle direkten Unterordner des Root, wie `/usr`, `/etc` oder `/data`
* Ihr Home-Verzeichnis
* Windows-Laufwerk-Roots und ihre Verzeichnisse der obersten Ebene, wie `C:\` und `C:\Windows`
* Ihr Arbeitsverzeichnis und seine übergeordneten Verzeichnisse
* Ihre zusätzlichen Arbeitsverzeichnisse und ihre übergeordneten Verzeichnisse, aber nur wenn die Entfernung ein Glob unter einem von ihnen ist, wie `rm -rf <dir>/*`. `rm -rf <dir>` im Verzeichnis selbst löst diese Überprüfung nicht aus

Claude Code behandelt auch einen Glob oder einen nachgestellten Schrägstrich direkt unter einer Shell-Variable, wie `rm -rf "$DIR"/*`, als Entfernung eines kritischen Pfads, da der Befehl zu einer Entfernung vom Dateisystem-Root wird, wenn die Variable leer ist.

Die Eingabeaufforderung für diesen Variablenfall benennt den gekennzeichneten `rm` und sagt, wie Sie ihn umschreiben, damit die Überprüfung erfolgreich ist:

* Für eine Variable wie `$DIR` schützen Sie jede Erweiterung, damit die Shell mit einem Fehler stoppt, wenn die Variable nicht gesetzt oder leer ist, wie in `rm -rf "${DIR:?}"/*`, oder verwenden Sie einen literalen Pfad
* Für eine Variable, die normalerweise gesetzt ist, wie `$HOME`, verwenden Sie einen literalen Pfad

Eine Entfernung, deren Erweiterungen auf diese Weise geschützt sind, ist keine Entfernung eines kritischen Pfads, daher wird sie im `bypassPermissions`-Modus ohne Eingabeaufforderung ausgeführt.

Das Verstecken der Entfernung in einer Subshell mit `(...)`, einer Brace-Gruppe mit `{ ...; }`, einer Befehlsersetzung mit `$(...)` oder Backticks oder einer Prozessersetzung mit `<(...)` überspringt die Überprüfung nicht. Claude Code findet eine Entfernung eines kritischen Pfads, ob sie sich in der verschachtelten Form befindet, wie in `(rm -rf ~)` oder `echo "$(rm -rf ~)"`, oder anderswo im gleichen Befehl.

<h3 id="remove-item-in-powershell">
  Remove-Item in PowerShell
</h3>

Wenn Sie das [PowerShell-Tool](/docs/de/tools-reference#powershell-tool) aktivieren, gibt Claude Code `Remove-Item` eine eigene Überprüfung, getrennt von der `rm`-Liste kritischer Pfade. Das Ergebnis hängt vom Ziel ab, und der erste passende Fall gilt:

* **Systempfade**: das Dateisystem-Root und seine Verzeichnisse der obersten Ebene, Laufwerk-Roots und ihre Verzeichnisse der obersten Ebene sowie Ihr Home-Verzeichnis. Claude Code lehnt den Befehl in jedem Modus ab, ohne Sie zu fragen.
* **Platzhalter**: ein bloßes `*` oder ein beliebiges Ziel, das auf `/*` oder `\*` endet, einschließlich eines Globs unter einer Shell-Variable wie `$dir/*`. Claude Code lehnt den Befehl in jedem Modus ab, ohne Sie zu fragen, bevor der [Klassifizierer](#eliminate-prompts-with-auto-mode) ihn sieht.
* **Ihr Arbeitsverzeichnis oder eines seiner übergeordneten Verzeichnisse mit `-Recurse`**: Claude Code behandelt den Befehl wie jeden anderen, der in Ihrem Berechtigungsmodus genehmigt werden muss, daher fordert es Sie in Modi auf, die fragen, sendet es in `auto`-Modus an den Klassifizierer und lehnt es im `dontAsk`-Modus ab. Der `bypassPermissions`-Modus überspringt diese Überprüfung.

<h2 id="see-also">
  Siehe auch
</h2>

* [Berechtigungen](/docs/de/permissions): Allow-, Ask- und Deny-Regeln; verwaltete Richtlinien
* [Auto-Modus konfigurieren](/docs/de/auto-mode-config): Teilen Sie dem Klassifizierer mit, welche Infrastruktur Ihre Organisation vertraut
* [Hooks](/docs/de/hooks): Benutzerdefinierte Berechtigungslogik über `PreToolUse` und `PermissionRequest` Hooks
* [Sicherheit](/docs/de/security): Schutzmaßnahmen und Best Practices
* [Sandboxing](/docs/de/sandboxing): Dateisystem- und Netzwerkisolation für Bash-Befehle
* [Nicht-interaktiver Modus](/docs/de/headless): Führen Sie Claude Code mit dem Flag `-p` aus
