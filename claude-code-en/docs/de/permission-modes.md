> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Wählen Sie einen Berechtigungsmodus

> Steuern Sie, ob Claude vor dem Bearbeiten von Dateien oder dem Ausführen von Befehlen fragt. Wechseln Sie Modi mit Shift+Tab in der CLI oder verwenden Sie den Moduswahlschalter in VS Code, Desktop und claude.ai.

Wenn Claude eine Datei bearbeiten, einen Shell-Befehl ausführen oder eine Netzwerkanfrage stellen möchte, hält es inne und bittet Sie, die Aktion zu genehmigen. Berechtigungsmodi steuern, wie oft diese Pause auftritt. Der Modus, den Sie wählen, prägt den Ablauf einer Sitzung: Der manuelle Modus erfordert, dass Sie jede Aktion überprüfen, während weniger restriktive Modi Claude ermöglichen, in längeren ununterbrochenen Abschnitten zu arbeiten und danach Bericht zu erstatten. Wählen Sie mehr Überwachung für sensible Arbeiten oder weniger Unterbrechungen, wenn Sie der Richtung vertrauen.

<h2 id="available-modes">
  Verfügbare Modi
</h2>

Jeder Modus stellt einen anderen Kompromiss zwischen Benutzerfreundlichkeit und Kontrolle dar. Die folgende Tabelle zeigt, was Claude in jedem Modus ohne Genehmigungsaufforderung tun kann.

| Modus                                                               | Was ohne Nachfrage ausgeführt wird                                                                  | Am besten geeignet für                                 |
| :------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------- | :----------------------------------------------------- |
| `default`                                                           | Nur Lesevorgänge                                                                                    | Erste Schritte, sensible Arbeiten                      |
| [`acceptEdits`](#auto-approve-file-edits-with-acceptedits-mode)     | Lesevorgänge, Dateibearbeitungen und häufige Dateisystembefehle (`mkdir`, `touch`, `mv`, `cp` usw.) | Iteration bei Code-Überprüfung                         |
| [`plan`](#analyze-before-you-edit-with-plan-mode)                   | Nur Lesevorgänge                                                                                    | Erkundung einer Codebasis vor Änderungen               |
| [`auto`](#eliminate-prompts-with-auto-mode)                         | Alles mit Sicherheitsprüfungen im Hintergrund                                                       | Lange Aufgaben, Reduzierung von Aufforderungsmüdigkeit |
| [`dontAsk`](#allow-only-pre-approved-tools-with-dontask-mode)       | Nur vorab genehmigte Tools                                                                          | Gesperrte CI und Skripte                               |
| [`bypassPermissions`](#skip-all-checks-with-bypasspermissions-mode) | Alles                                                                                               | Nur isolierte Container und VMs                        |

Der Modus, der jede Aktion überprüft, wird in der CLI, in `claude --help`, in den VS Code- und JetBrains-Erweiterungen und in der Desktop-App als **Manual** bezeichnet. Sein Konfigurationswert ist `default`, was Hooks und SDK-Integrationen verwenden. Die CLI akzeptiert `manual` als Alias überall dort, wo Sie den Wert eingeben, zum Beispiel `claude --permission-mode manual` oder `"defaultMode": "manual"`. Das Manual-Label und der `manual`-Alias erfordern Claude Code v2.1.200 oder später. Das Label der Desktop-App hängt nicht von Ihrer CLI-Version ab.

In jedem Modus außer `bypassPermissions` werden Schreibvorgänge in [geschützte Pfade](#protected-paths) niemals automatisch genehmigt, um den Repository-Status und die Claude-Konfiguration vor versehentlicher Beschädigung zu schützen.

Modi legen die Grundlage fest. Überlagern Sie [Berechtigungsregeln](/docs/de/permissions#manage-permissions) darauf, um bestimmte Tools vorab zu genehmigen oder zu blockieren. Ablehnungsregeln, explizite Aufforderungsregeln, die [Organisationseinstellung `ask` für Connector-Tools](/docs/de/mcp#organization-controls-on-connector-tools) und der [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool)-Marker gelten in jedem Modus, einschließlich `bypassPermissions`. Genehmigungsregeln haben keine Auswirkung in diesem Modus, da alles andere bereits genehmigt ist.

<h2 id="switch-permission-modes">
  Berechtigungsmodi wechseln
</h2>

Sie können Modi während einer Sitzung, beim Start oder als persistente Standardeinstellung wechseln. Der Modus wird über diese Steuerelemente festgelegt, nicht durch Anfragen an Claude im Chat. Wählen Sie Ihre Schnittstelle unten aus, um zu sehen, wie Sie ihn ändern.

<Tabs>
  <Tab title="CLI">
    **Während einer Sitzung**: Drücken Sie `Shift+Tab`, um zwischen `default` → `acceptEdits` → `plan` zu wechseln. Der aktuelle Modus wird in der Statusleiste angezeigt. Der manuelle Modus, `default` in diesem Zyklus, zeigt ein graues `⏸ manual mode on` Badge. Vor v2.1.203 zeigte die Statusleiste im manuellen Modus kein Badge an.

    Nicht jeder Modus ist im Standard-Zyklus enthalten:

    * `auto`: wird angezeigt, wenn Ihr Konto die [Anforderungen für den Auto-Modus](#eliminate-prompts-with-auto-mode) erfüllt; das Wechseln zu diesem Modus schaltet Modi ohne Bestätigungsaufforderung um
    * `bypassPermissions`: wird angezeigt, nachdem Sie mit `--permission-mode bypassPermissions`, `--dangerously-skip-permissions` oder `--allow-dangerously-skip-permissions` starten; die `--allow-` Variante fügt den Modus zum Zyklus hinzu, ohne ihn zu aktivieren
    * `dontAsk`: wird nie im Zyklus angezeigt; legen Sie ihn mit `--permission-mode dontAsk` fest

    Aktivierte optionale Modi werden nach `plan` eingefügt, mit `bypassPermissions` zuerst und `auto` zuletzt. Wenn Sie beide aktiviert haben, wechseln Sie durch `bypassPermissions` auf dem Weg zu `auto`.

    **Beim Start**: Übergeben Sie den Modus als Flag.

    ```bash theme={null}
    claude --permission-mode plan
    ```

    **Als Standard**: Legen Sie `defaultMode` in [Einstellungen](/docs/de/settings#settings-files) fest.

    ```json theme={null}
    {
      "permissions": {
        "defaultMode": "acceptEdits"
      }
    }
    ```

    Das gleiche `--permission-mode` Flag funktioniert mit `-p` für [nicht-interaktive Ausführungen](/docs/de/headless).
  </Tab>

  <Tab title="VS Code">
    **Während einer Sitzung**: Klicken Sie auf den Modusindikator am unteren Rand des Eingabefelds.

    **Als Standard**: Legen Sie `claudeCode.initialPermissionMode` in VS Code-Einstellungen fest, oder verwenden Sie das Einstellungsfenster der Claude Code-Erweiterung.

    Der Modusindikator zeigt diese Beschriftungen, die dem Modus zugeordnet sind, auf den sich jede bezieht:

    | UI-Beschriftung    | Modus               |
    | :----------------- | :------------------ |
    | Manual             | `default`           |
    | Edit automatically | `acceptEdits`       |
    | Plan               | `plan`              |
    | Auto               | `auto`              |
    | Bypass permissions | `bypassPermissions` |

    Vor v2.1.205 bezeichnete die Erweiterung `plan` als Plan mode und `auto` als Auto mode.

    Der Auto-Modus wird im Modusindikator angezeigt, wenn Ihr Konto alle Anforderungen erfüllt, die im [Auto-Modus-Abschnitt](#eliminate-prompts-with-auto-mode) aufgelistet sind. Die Einstellung `claudeCode.initialPermissionMode` akzeptiert nicht `auto`. Um standardmäßig im Auto-Modus zu starten, legen Sie stattdessen `defaultMode` in Ihren [Benutzereinstellungen](/docs/de/settings#settings-files) fest. Claude Code ignoriert `defaultMode: "auto"` in Projekt- und lokalen Einstellungen.

    Bypass permissions erfordert den Schalter **Allow dangerously skip permissions** in den Erweiterungseinstellungen, bevor er im Modusindikator angezeigt wird.

    Weitere Informationen finden Sie im [VS Code-Leitfaden](/docs/de/vs-code).
  </Tab>

  <Tab title="JetBrains">
    Das JetBrains-Plugin führt Claude Code im IDE-Terminal aus, daher funktioniert das Wechseln von Modi genauso wie in der CLI: Drücken Sie `Shift+Tab` zum Wechseln, oder übergeben Sie `--permission-mode` beim Start.
  </Tab>

  <Tab title="Desktop">
    **Während einer Sitzung**: Verwenden Sie den Moduswahlschalter neben der Schaltfläche zum Senden. Nicht jeder Modus wird im Wahlschalter angezeigt:

    * **Auto**: wird angezeigt, wenn Ihr Konto die [Anforderungen für den Auto-Modus](#eliminate-prompts-with-auto-mode) erfüllt
    * **Bypass permissions**: erfordert den Schalter **Allow bypass permissions mode** in den Desktop-Einstellungen für Pro- und Max-Pläne; bei Team- und Enterprise-Plänen wird dies stattdessen durch die Organisationsrichtlinie gesteuert

    Weitere Desktop-spezifische Details finden Sie unter [Berechtigungsmodus wählen](/docs/de/desktop#choose-a-permission-mode) im Desktop-Leitfaden.

    **Als Standard**: Legen Sie `defaultMode` in [Einstellungen](/docs/de/settings#settings-files) fest. Die Desktop-App liest die gleichen Einstellungsdateien wie die CLI und wendet den Modus auf neue lokale Sitzungen an.

    Ein Modus, den Sie im Moduswahlschalter auswählen, wird pro Ordner gespeichert und hat Vorrang vor `defaultMode` für diesen Ordner. Plan ist die Ausnahme: Das Auswählen gilt nur für die aktuelle Sitzung.

    Dieses Beispiel legt den Plan-Modus als Standard für neue lokale Sitzungen fest:

    ```json theme={null}
    {
      "permissions": {
        "defaultMode": "plan"
      }
    }
    ```
  </Tab>

  <Tab title="Web and mobile">
    Verwenden Sie das Modusmenü neben dem Eingabefeld auf [claude.ai/code](https://claude.ai/code) oder in der mobilen App. Berechtigungsaufforderungen werden in claude.ai zur Genehmigung angezeigt. Welche Modi angezeigt werden, hängt davon ab, wo die Sitzung ausgeführt wird:

    * **Cloud-Sitzungen** auf [Claude Code im Web](/docs/de/claude-code-on-the-web): Bearbeitungen akzeptieren, Plan und Auto. Bearbeitungen akzeptieren entspricht dem `default` Modus: Die Cloud-Umgebung genehmigt Dateibearbeitungen vorab, unabhängig vom Modus, daher zeigt das Menü Bearbeitungen akzeptieren statt Manuell an. Cloud-Sitzungen respektieren weiterhin `defaultMode: "acceptEdits"` aus den Einstellungen. Der Auto-Modus wird nur angezeigt, wenn Ihre Organisation ihn zulässt und das ausgewählte Modell ihn unterstützt. Bypass permissions ist nicht verfügbar.
    * **[Remote Control](/docs/de/remote-control) Sitzungen** auf Ihrem lokalen Computer: Manuell, Bearbeitungen akzeptieren und Plan. Sie können Auto oder Bypass permissions nicht aus der App auswählen. Das Menü zeigt den Modus an, in dem sich die lokale Sitzung befindet, einschließlich eines Modus, der vom Terminal aus festgelegt wurde, und wird aktualisiert, wenn sich der Modus in der App oder im Terminal ändert. Die einzige Ausnahme ist Bypass permissions: Die Sitzung meldet diesen Modus nie an claude.ai, daher ändert das Wechseln vom Terminal aus nicht, was das Menü anzeigt. Vor v2.1.202 meldeten Sitzungen, die mit `/remote-control` oder `claude --remote-control` verbunden waren, ihren Modus überhaupt nicht, daher konnten claude.ai und die mobile App einen Modus anzeigen, in dem sich die Sitzung nicht befand. Die Nichtübereinstimmung betraf nur die Beschriftung: Claude Code generierte Berechtigungsaufforderungen aus dem tatsächlichen Modus der Sitzung, und sie wurden weiterhin in der App zur Genehmigung angezeigt.

    Für Remote Control können Sie auch den Startmodus beim Starten des Hosts festlegen:

    ```bash theme={null}
    claude remote-control --permission-mode acceptEdits
    ```
  </Tab>
</Tabs>

<h2 id="auto-approve-file-edits-with-acceptedits-mode">
  Dateibearbeitungen mit acceptEdits-Modus automatisch genehmigen
</h2>

Der `acceptEdits`-Modus ermöglicht es Claude, Dateien in Ihrem Arbeitsverzeichnis zu erstellen und zu bearbeiten, ohne Sie zu fragen. Die Statusleiste zeigt `⏵⏵ accept edits on` an, während dieser Modus aktiv ist.

Zusätzlich zu Dateibearbeitungen genehmigt der `acceptEdits`-Modus automatisch häufige Bash-Befehle im Dateisystem: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp` und `sed`. Diese Befehle werden auch automatisch genehmigt, wenn sie mit sicheren Umgebungsvariablen wie `LANG=C` oder `NO_COLOR=1` oder Prozess-Wrappern wie `timeout`, `nice` oder `nohup` vorangestellt sind. Wie bei Dateibearbeitungen gilt die automatische Genehmigung nur für Pfade in Ihrem Arbeitsverzeichnis oder `additionalDirectories`. Pfade außerhalb dieses Bereichs, Schreibvorgänge auf [geschützte Pfade](#protected-paths) und alle anderen Bash-Befehle außer dem [integrierten schreibgeschützten Satz](/docs/de/permissions#read-only-commands) erfordern weiterhin eine Genehmigung.

Wenn das [PowerShell-Tool](/docs/de/tools-reference#powershell-tool) aktiviert ist, genehmigt der `acceptEdits`-Modus auch automatisch `Set-Content`, `Add-Content`, `Clear-Content` und `Remove-Item` auf Pfaden im Gültigkeitsbereich, zusammen mit ihren häufigen Aliasen. Die gleichen Bereichs- und Schutzpfad-Regeln gelten.

Verwenden Sie `acceptEdits`, wenn Sie Änderungen in Ihrem Editor oder über `git diff` im Nachhinein überprüfen möchten, anstatt jede Bearbeitung inline zu genehmigen.

Drücken Sie `Shift+Tab` einmal vom Manuellen Modus aus, um ihn zu aktivieren, oder starten Sie direkt damit:

```bash theme={null}
claude --permission-mode acceptEdits
```

<h2 id="analyze-before-you-edit-with-plan-mode">
  Analysieren Sie vor dem Bearbeiten mit dem Plan-Modus
</h2>

Der Plan-Modus weist Claude an, Änderungen zu recherchieren und vorzuschlagen, ohne sie vorzunehmen. Claude liest Dateien, führt Shell-Befehle aus, um zu erkunden, und schreibt einen Plan, bearbeitet aber nicht Ihre Quelle. Genehmigungsaufforderungen gelten wie im manuellen Modus, es sei denn, der [Auto-Modus](/docs/de/auto-mode-config) ist verfügbar und `useAutoModeDuringPlan` ist aktiviert, was die Standardeinstellung ist. Mit aktiviertem Auto-Modus genehmigt der Klassifizierer schreibgeschützte Befehle wie Suchen und Dateileser ohne Aufforderung. Bearbeitungen bleiben in jedem Fall blockiert, bis Sie den Plan genehmigen.

Geben Sie den Plan-Modus ein, indem Sie `Shift+Tab` drücken oder einem einzelnen Prompt `/plan` voranstellen. Sie können auch vom CLI aus im Plan-Modus starten:

```bash theme={null}
claude --permission-mode plan
```

Drücken Sie `Shift+Tab` erneut, um den Plan-Modus zu verlassen, ohne einen Plan zu genehmigen.

<h3 id="review-and-approve-a-plan">
  Überprüfen und genehmigen Sie einen Plan
</h3>

Wenn der Plan fertig ist, präsentiert Claude ihn und fragt, wie Sie vorgehen möchten. Von dieser Aufforderung aus können Sie:

* Genehmigen und im Auto-Modus starten
* Genehmigen und Bearbeitungen akzeptieren
* Genehmigen und jede Bearbeitung manuell überprüfen
* Mit Feedback weiter planen
* Mit [Ultraplan](/docs/de/ultraplan) für browsergestützte Überprüfung verfeinern

Das Genehmigen eines Plans beendet den Plan-Modus und wechselt die Sitzung zum Genehmigungsmodus, den jede Genehmigungsoption beschreibt, sodass Claude mit der Bearbeitung beginnt. Um erneut zu planen, wechseln Sie mit `Shift+Tab` zurück zum Plan-Modus oder stellen Sie Ihrem nächsten Prompt `/plan` voran.

Drücken Sie `Ctrl+G`, um den vorgeschlagenen Plan in Ihrem Standard-Texteditor zu öffnen und ihn direkt zu bearbeiten, bevor Claude fortfährt. Wenn [`showClearContextOnPlanAccept`](/docs/de/settings#available-settings) aktiviert ist, bietet jede Genehmigungsoption auch an, den Planungskontext zuerst zu löschen.

Das Akzeptieren eines Plans benennt die Sitzung auch automatisch aus dem Planinhalt, es sei denn, Sie haben bereits einen Namen mit `--name` oder `/rename` festgelegt.

<h3 id="set-plan-mode-as-the-default">
  Legen Sie den Plan-Modus als Standard fest
</h3>

Um den Plan-Modus als Standard für ein Projekt festzulegen, setzen Sie `defaultMode` in `.claude/settings.json`:

```json theme={null}
{
  "permissions": {
    "defaultMode": "plan"
  }
}
```

<h2 id="eliminate-prompts-with-auto-mode">
  Berechtigungsaufforderungen mit Auto-Modus eliminieren
</h2>

Der Auto-Modus ermöglicht es Claude, ohne routinemäßige Berechtigungsaufforderungen auszuführen. Ein separates Klassifizierungsmodell überprüft Aktionen vor ihrer Ausführung und blockiert alles, das über Ihre Anfrage hinausgeht, auf nicht erkannte Infrastruktur abzielt oder von feindseligem Inhalt angetrieben zu sein scheint, den Claude gelesen hat. Explizite [Ask-Regeln](/docs/de/permissions#manage-permissions) erzwingen weiterhin eine Aufforderung.

Löschungen, die das Dateisystem-Root oder das Home-Verzeichnis anvisieren, wie `rm -rf /` und `rm -rf ~`, fordern zur Genehmigung auf, statt zum Klassifizierer zu gehen. Diese Aufforderung wird auch ausgelöst, wenn der Befehl Befehlsersetzung mit `$(...)` oder Backticks oder Prozessersetzung mit `<(...)` enthält, unabhängig davon, ob die Löschung innerhalb der Ersetzung liegt, wie in `echo "$(rm -rf ~)"`, oder anderswo im gleichen Befehl. Vor v2.1.208 wurden Befehle, die diese Formen enthielten, zum Klassifizierer geleitet, statt eine Aufforderung auszulösen.

Der Auto-Modus ermutigt Claude auch, ohne Unterbrechung für Klärungsfragen weiterzuarbeiten, obwohl Claude immer noch fragt, wenn Ihre Eingabeaufforderung oder eine Fähigkeit dies explizit erfordert. Für stärkeres autonomes Verhalten bei Beibehaltung von Berechtigungsaufforderungen stellen Sie stattdessen den [Proaktiven Ausgabestil](/docs/de/output-styles) ein.

<Warning>
  Der Auto-Modus reduziert Berechtigungsaufforderungen, garantiert aber keine Sicherheit. Verwenden Sie ihn für Aufgaben, bei denen Sie der allgemeinen Richtung vertrauen, nicht als Ersatz für die Überprüfung bei sensiblen Operationen.
</Warning>

Der Auto-Modus ist nur verfügbar, wenn Ihr Konto alle diese Anforderungen erfüllt:

* **Plan**: Alle Pläne.
* **Besitzer**: Bei Team und Enterprise muss ein Besitzer ihn in den [Claude Code Admin-Einstellungen](https://claude.ai/admin-settings/claude-code) aktivieren, bevor Benutzer ihn einschalten können. Administratoren können den Auto-Modus auch ausschalten, indem sie `permissions.disableAutoMode` in den [verwalteten Einstellungen](/docs/de/permissions#managed-settings) auf `"disable"` setzen. Für die Registerkarte „Code" der Desktop-App ist `disableAutoMode` die Kontrolle auf Organisationsebene, und der Admin-Einstellungen-Schalter gilt nicht.
* **Modell**: In der Anthropic API Claude Opus 4.6 oder später oder Sonnet 4.6 oder später. Bei Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry und angemeldeten [Claude Apps Gateway](/docs/de/claude-apps-gateway)-Sitzungen nur Claude Sonnet 5, Opus 4.7 und Opus 4.8. Ältere Modelle, einschließlich Sonnet 4.5, Opus 4.5, Haiku und claude-3-Modelle, werden auf keinem Anbieter unterstützt.
* **Anbieter**: Standardmäßig verfügbar in der Anthropic API, Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry und angemeldeten Claude Apps Gateway-Sitzungen. In v2.1.158 bis v2.1.206 war der Auto-Modus auf allen diesen Anbietern außer der Anthropic API deaktiviert, bis Sie `CLAUDE_CODE_ENABLE_AUTO_MODE=1` setzten; v2.1.207 entfernte die Anforderung.

Wenn Claude Code den Auto-Modus als nicht verfügbar meldet, ist eine dieser Anforderungen nicht erfüllt; dies ist kein vorübergehender Ausfall. Eine separate Nachricht, die ein Modell benennt und sagt, dass der Auto-Modus die Sicherheit einer Aktion „nicht bestimmen kann", ist ein vorübergehender Klassifizierungsausfall; siehe die [Fehlerreferenz](/docs/de/errors#auto-mode-cannot-determine-the-safety-of-an-action).

Wenn Sie `defaultMode: "auto"` in den [Einstellungen](/docs/de/settings#available-settings) setzen und die Sitzung im `default`-Modus ohne Fehler startet, befindet sich die Einstellung wahrscheinlich in `.claude/settings.json` oder `.claude/settings.local.json`. Claude Code v2.1.142 und später ignorieren `auto` aus diesen Dateien, sodass ein Repository sich nicht selbst den Auto-Modus gewähren kann. Verschieben Sie es zu `~/.claude/settings.json`.

<h3 id="enable-auto-mode-on-bedrock-agent-platform-or-foundry">
  Auto-Modus auf Bedrock, Agent Platform oder Foundry
</h3>

Bei [Amazon Bedrock](/docs/de/amazon-bedrock), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai), [Microsoft Foundry](/docs/de/microsoft-foundry) und angemeldeten [Claude Apps Gateway](/docs/de/claude-apps-gateway)-Sitzungen wird der Auto-Modus standardmäßig im `Shift+Tab`-Zyklus angezeigt. Das Erscheinen im Zyklus ändert nicht den Modus, in dem eine Sitzung startet: Sitzungen starten immer noch in Ihrem [`defaultMode`](/docs/de/settings#available-settings), der Manual ist, es sei denn, Sie ändern ihn. Nur Claude Sonnet 5, Opus 4.7 und Opus 4.8 werden auf diesen Anbietern unterstützt.

Um den Auto-Modus zum Standard-Startmodus zu machen, setzen Sie `"permissions": {"defaultMode": "auto"}` in Benutzer- oder verwalteten Einstellungen.

Um Entwickler daran zu hindern, den Auto-Modus zu verwenden, setzen Sie `disableAutoMode` in den [verwalteten Einstellungen](/docs/de/permissions#managed-settings) auf `"disable"`. Dies entfernt `auto` aus dem `Shift+Tab`-Zyklus und lehnt `--permission-mode auto` beim Start ab.

In v2.1.158 bis v2.1.206 war der Auto-Modus auf diesen Anbietern deaktiviert, bis Sie `CLAUDE_CODE_ENABLE_AUTO_MODE=1` setzten, und Claude Code ignorierte `defaultMode: "auto"` auf diesen Anbietern, es sei denn, die Variable war auch gesetzt. Die Variable wird weiterhin aus Kompatibilitätsgründen akzeptiert und hat ab v2.1.207 keine Auswirkung.

<h3 id="what-the-classifier-blocks-by-default">
  Was der Klassifizierer standardmäßig blockiert
</h3>

Der Klassifizierer vertraut Ihrem Arbeitsverzeichnis und den Remotes, die dafür konfiguriert wurden, als die Sitzung begann. Ein Remote, das während der Sitzung mit `git remote add` oder `git remote set-url` hinzugefügt oder umgeleitet wird, wird nicht vertraut, und alles andere wird als extern behandelt, bis Sie [vertrauenswürdige Infrastruktur konfigurieren](/docs/de/auto-mode-config). Vor v2.1.200 wurden Remotes, die während der Sitzung hinzugefügt wurden, ebenfalls vertraut.

**Standardmäßig blockiert**:

* Herunterladen und Ausführen von Code, wie `curl | bash`
* Senden sensibler Daten an externe Endpunkte
* Produktionsbereitstellungen und Migrationen
* Massenlöschung im Cloud-Speicher
* Gewährung von IAM- oder Repository-Berechtigungen
* Änderung gemeinsamer Infrastruktur
* Irreversibles Löschen von Dateien, die vor der Sitzung vorhanden waren
* Force Push
* Pushing zum Standard-Branch des Repositorys, wenn der Push sensible Inhalte wie Geheimnisse oder persönliche oder anvertraute Daten enthält, Änderungen enthält, die verborgen oder falsch beschrieben sind im Vergleich zu dem, worum Sie gebeten haben, Inhalte enthält, die von außerhalb des Repositorys portiert oder zuerst gelesen wurden, oder um einen Pull Request, eine Überprüfung oder eine Prüfung herumleitet, um die Sie gebeten haben. Ein einfacher Push zum Standard-Branch wird nicht allein blockiert, und das Aufheben der Blockierung eines gekennzeichneten Push erfordert das Benennen des gekennzeichneten Inhalts oder der umgangenen Überprüfung, nicht nur des Push. Der Klassifizierer ist eine Schicht: [`permissions.deny`-Regeln](/docs/de/permissions#manage-permissions) gelten in jedem Modus und können Pushes zum Standard-Branch vollständig blockieren, und der Branch-Schutz des Remote gilt immer noch. Vor v2.1.203 wurde jeder direkte Push zum Standard-Branch blockiert
* `git reset --hard`, `git checkout -- .`, `git restore .`, `git clean -fd`, `git stash drop` oder `git stash clear`, von denen der Klassifizierer annimmt, dass sie nicht committete Änderungen verwerfen würden
* `git commit --amend`, wenn der Commit am HEAD nicht in dieser Sitzung erstellt wurde
* Ab v2.1.198 `git commit --amend`, wenn der Commit am HEAD bereits gepusht wurde. Eine reine Umformulierung der Nachricht wird nicht blockiert: `--amend -m` ohne neu bereitgestellte Inhalte, bei einem Commit, den Claude während dieser Sitzung erstellt hat
* `terraform destroy`, `pulumi destroy`, `cdk destroy` oder `terragrunt destroy`, und Anwendung eines Plans, der Ressourcen zerstört

Claude Code v2.1.195 und später blockieren standardmäßig mehr Kategorien. Mehrere hängen von [Umgebungs](/docs/de/auto-mode-config#define-trusted-infrastructure)-Einträgen ab, wie sensible Remote-Ziele und geschützte IaC-Bereiche, die Sie auf konkrete Namen eingrenzen können.

* Schreiben in einen Secret Manager oder Ändern von DNS-Einträgen oder TLS-Zertifikaten
* Zusammenführen eines Pull Requests, den kein Mensch genehmigt hat, Genehmigung von Claudes eigenem Pull Request oder Deaktivierung von CI-Prüfungen
* Posten eines Kommentars, der selbst ein Befehl für Automatisierung ist, wie `atlantis apply` oder das `/deploy` oder `/merge` eines Bots
* Umschalten, Ramping oder Löschen eines Production Feature Flags
* Anwendung von Infrastrukturänderungen auf einen geschützten IaC-Bereich oder Draining (Leeren) und Entfernen von Cluster-Knoten
* Schreibvorgänge in einen gemeinsamen Compute-Cluster, die über die benannte Ressource hinausgehen, wie ein Label-Selektor oder `--all`, der die Jobs anderer Benutzer erfasst
* Erstellen von Kubernetes-Ressourcen, die auf jedem Knoten ausgeführt werden oder Cluster-Traffic abfangen, wie DaemonSets und Admission Webhooks
* Interaktive Shells oder Port-Forwards in ein sensibles Remote-Ziel
* Öffnen eines Tunnels oder einer Reverse Shell, die einen lokalen Service vom öffentlichen Internet erreichbar macht
* Drucken eines Live-Credentials oder Tokens in das Transkript oder eine Datei
* Zugriff auf einen Ort, der in Ihrer [Umgebung](/docs/de/auto-mode-config#define-trusted-infrastructure) als sensible Datenlocation aufgelistet ist, oder Kopieren von Daten daraus. Ab v2.1.198 blockiert dies auch das Senden von Daten von einem zu einer Zielgruppe, die der Eintrag ausschließt
* Umleitung einer Paketinstallation um Ihre interne Paketregistrierung zu einer öffentlichen Registrierung. Ab v2.1.198 gilt dies auch, wenn Sie Claude in der Konversation mitgeteilt haben, dass eine interne Registrierung oder ein Mirror vorhanden ist, nicht nur wenn eine in Ihrer Umgebung aufgelistet ist
* Ausführung eines Befehls mit einem Flag, das einen Sicherheitsschutz deaktiviert, wie `--insecure`
* Starten einer autonomen Agent-Schleife, die ohne menschliche Genehmigung oder Sandbox ausgeführt wird, wie eine mit `--dangerously-skip-permissions` oder `--no-sandbox` gestartete. Ab v2.1.198 deckt dies auch das Ausführen eines Third-Party-Agenten oder Eval-Harness mit deaktivierter Isolation und Pro-Aktion-Genehmigung ab, wie ein mit `--yes-always` gestarteter Runner
* [Claude in Chrome](/docs/de/chrome) Browser-Aktionen, die Seiteninhalte, Cookies oder Credentials off-origin senden könnten

Claude Code v2.1.198 und später blockieren diese auch standardmäßig:

* Löschen von Dateien in `/tmp`, `$TMPDIR` oder einem anderen gemeinsamen Scratch- oder Cache-Verzeichnis nach Wildcard, Glob oder Altersfilter statt nach einem spezifischen benannten Pfad
* Einbeziehung sensibler Details in Inhalte, die gesendet, hochgeladen, veröffentlicht oder an andere Personen oder gemeinsame Systeme geschrieben werden, wenn Ihre eigene Nachricht diese Details nicht für diesen Empfänger autorisiert hat. {{/* min-version: 2.1.200 */}}PR- und Issue-Bodies, Commit-Nachrichten und Kommentare zählen als diese Art von ausgehendem Inhalt, wenn das Repository außerhalb der Vertrauensgrenze oder öffentlich ist, einschließlich der eigenen öffentlichen Repositorys Ihrer Organisation; interne Dateipfade, Code-Namen, Live-API-Antwortdaten wie E-Mails oder Kontobezeichner und Infrastruktur-Bezeichner zählen als sensible Details. Die PR-, Issue- und Commit-Nachrichten-Scoping erfordert Claude Code v2.1.200 oder später. {{/* min-version: 2.1.203 */}}Live-Personendaten aus einer API-Antwort in einem PR- oder Issue-Body, wie eine E-Mail-Adresse, ein Konto- oder Organisationsbezeichner oder eine Nutzungsmetrik, erfordert, dass Sie diese Details und den Empfänger benennen, unabhängig von der Sichtbarkeit oder Vertrauensgrenze des Repositorys. Diese Prüfung erfordert Claude Code v2.1.203 oder später
* Senden von Tastenanschlägen an Claudes eigenen tmux-Pane, um seine eigene Schnittstelle zu steuern, was der Klassifizierer als Claude behandelt, das seine eigenen Berechtigungen oder Aufsicht ändert

{{/* min-version: 2.1.200 */}}

Claude Code v2.1.200 und später blockieren diese auch standardmäßig:

* Auskommentieren, Löschen oder Force-Passing eines Tests oder einer Assertion, die Sicherheitsverhalten schützt, wie Authentifizierung, Zugriffskontrolle, Eingabevalidierung oder Sandboxing
* Löschen oder Abbau einer zustandsbehafteten Ressource, die Claude nicht in der Sitzung erstellt hat, wenn keine spezifischere Löschregel gilt und Sie diese Ressource nicht benannt haben
* Umleitung einer API-Basis-URL, eines Proxy-Endpunkts, eines Webhook-Empfängers oder eines Registry-Mirrors auf einen Third-Party-Host, der nicht zur Aufgabe passt, einschließlich in Beispieldateien wie `.env.example`
* Änderung des Ziels von Pushes mit `git remote set-url` oder `git remote add`, es sei denn, Sie haben den neuen Remote benannt
* Pushing von Geheimnissen oder persönlichen oder anvertrauten Daten in ein Repository, das bekanntermaßen öffentlich ist, oder Pushing von vertraulichem Material dorthin, das nicht Teil der eigenen Arbeit dieses Repositorys ist. {{/* min-version: 2.1.203 */}}Das eigene Thema eines Dotfiles-Repositorys ist die einzige Ausnahme für persönliche oder anvertraute Daten, und Inhalte aus einem privaten Repository, die eine öffentliche Oberfläche erreichen, werden auf die gleiche Weise blockiert; beide Verfeinerungen erfordern Claude Code v2.1.203 oder später. Vor v2.1.203 wurden persönliche Daten mit vertraulichem Material gruppiert und nur blockiert, wenn sie nicht Teil der eigenen Arbeit dieses Repositorys waren. Wenn die Sichtbarkeit eines Repositorys nicht etabliert ist, blockiert der Klassifizierer nicht allein darauf; er beurteilt den Inhalt stattdessen gegen die anderen Regeln
* Öffnen eines Pull Requests gegen ein anderes Repository oder eine andere Organisation, Forking mit `gh repo fork` oder Pushing in ein Third-Party-Repository, es sei denn, Sie haben dieses externe Ziel benannt

{{/* min-version: 2.1.203 */}}

Claude Code v2.1.203 und später blockieren diese auch standardmäßig:

* Inhalte aus einem sensiblen lokalen Speicher oder aus einer Datei, deren Name, Pfad oder Typ sie als sensibel kennzeichnet, die in einen Commit, einen Push, PR- oder Issue-Text, einen Gist oder Paste oder eine Paketveröffentlichung eingehen, es sei denn, Sie haben sowohl die Quelle als auch das Ziel benannt. Sitzungstranskripte und Konversationsprotokolle, Credential- und Konfigurations-Dot-Ordner wie SSH-Schlüssel, Cloud-Credentials, Browser-Profile und Shell-Verlauf sowie Benutzer-Daten-Exporte zählen alle, und dass das Repository privat ist, hebt die Blockierung nicht auf

{{/* min-version: 2.1.205 */}}

Claude Code v2.1.205 und später blockieren diese auch standardmäßig:

* Schreiben in Claude Code-Sitzungstranskripte, die `.jsonl`-Verlaufsdateien unter `~/.claude/projects/` oder Ihrem konfigurierten Konfigurationsverzeichnis, ob direkt oder durch einen Shell-Befehl. Die Regel deckt auch die Metadatenzeilen ab, die Claude Code für seine eigenen Prüfungen an jeden Transkripteintrag anhängt. Ein Transkript ist Sitzungszustand, den Claude Code schreibt, nicht eine Arbeitsdatei, und ein manipulierter Eintrag erreicht jede spätere Prüfung, sobald Sie die Sitzung fortsetzen, sodass der Auto-Modus diese Schreibvorgänge als Verteidigungstiefe blockiert. Das Lesen eines Transkripts wird nicht blockiert
* Ein rekursives erzwungenes Löschen wie `rm -rf "$VAR"` oder `Remove-Item -Recurse -Force $dir`, dessen Ziel eine Shell-Variable ist, oder ein Glob, der an einer verwurzelt ist, die nirgendwo in der Konversation zugewiesen ist, die der Klassifizierer sieht. Der Wert kam nur aus früherer Befehlsausgabe, die der Klassifizierer nie erhält, sodass der Klassifizierer das Löschziel nicht gegen die anderen Löschregeln überprüfen kann. Der Klassifizierer liest die Konversation statt der Befehlsausgabe absichtlich, sodass er den Aufruf blockiert, statt das Ziel zu erraten. Die Blockierung wird aufgehoben, wenn Sie den genauen Pfad benennen, der gelöscht wird, oder wenn Claude das Löschen mit dem aufgelösten literalen Pfad, der in den Befehl geschrieben ist, erneut ausführt. Löschvorgänge, deren Ziel der Klassifizierer auflösen kann, sind nicht betroffen

**Standardmäßig erlaubt**:

* Lokale Dateivorgänge in Ihrem Arbeitsverzeichnis
* Installation von Abhängigkeiten, die in Ihren Lock-Dateien oder Manifesten deklariert sind
* Lesen von `.env` und Senden von Credentials an ihre entsprechende API
* Read-Only HTTP-Anfragen
* Pushing zum Branch, auf dem Sie gestartet haben, oder zu einem, den Claude erstellt hat
* {{/* min-version: 2.1.203 */}}Routinemäßige Pushes zum Repository Standard-Branch. Vor v2.1.203 wurde jeder direkte Push zum Standard-Branch blockiert

{{/* min-version: 2.1.195 */}}

Claude Code v2.1.195 und später erlauben diese auch standardmäßig:

* Löschen der genauen Jobs, die Claude früher in derselben Sitzung erstellt hat
* Lesen, Überprüfung oder Schreiben von sicherheitsbezogenem Code, Configs und Bedrohungsmodellen als Teil Ihrer Aufgabe
* Nachrichten zwischen Agenten, die zusammen in derselben Multi-Agent-Sitzung arbeiten
* Senden von Daten an die vertrauenswürdigen Domains, Buckets und Services, die Sie in [`environment`](/docs/de/auto-mode-config#define-trusted-infrastructure) auflisten. Dies deckt nur Datenfluss ab, nicht destruktive oder Credential-Operationen auf derselben Infrastruktur
* [Claude in Chrome](/docs/de/chrome) Navigation zu einer vertrauenswürdigen internen Domain, localhost oder einer URL, die Sie benannt haben

Sandbox-Netzwerkzugriff-Anfragen werden durch den Klassifizierer geleitet, statt standardmäßig erlaubt zu werden. {{/* min-version: 2.1.198 */}}Ab v2.1.198 verwendet der Klassifizierer sein Urteil für einen Netzwerk-Host und Port wieder, statt bei jeder Verbindung erneut auszuführen:

* Ein Allow wird wiederverwendet, bis neuer Inhalt in die Konversation eintritt, an welchem Punkt dieser Host erneut überprüft wird
* In der interaktiven CLI wird ein Deny gelöscht, wenn die Runde endet
* Im [nicht-interaktiven Modus](/docs/de/headless) und Agent SDK-Sitzungen gibt es keine Rundenbegrenzung, sodass ein Deny für den Rest des Laufs wiederverwendet wird
* Das Ändern Ihres Berechtigungsmodus oder Ihrer Regeln löscht alle zwischengespeicherten Urteile

Führen Sie `claude auto-mode defaults` aus, um die vollständigen Regellisten zu sehen. Wenn routinemäßige Aktionen blockiert werden, kann ein Administrator vertrauenswürdige Repos, Buckets und Services über die `autoMode.environment`-Einstellung hinzufügen: siehe [Auto-Modus konfigurieren](/docs/de/auto-mode-config).

Pushing zum Arbeits-Branch, Durchführung eines routinemäßigen Push zum Repository Standard-Branch und Erstellen eines Pull Requests, der Ihrer Anfrage entspricht, werden alle ohne Aufforderung ausgeführt. Der Klassifizierer blockiert einen Push nur, wenn er Risiken birgt, wie ein Force Push oder Inhalte, die um eine Überprüfung herumleiten, die Sie eingerichtet haben. Um einen menschlichen Checkpoint vor diesen Aktionen zu erzwingen, während Sie im Auto-Modus bleiben, fügen Sie `permissions.ask`-Regeln hinzu: siehe [Häufige Grenzen](/docs/de/auto-mode-config#common-boundaries).

<h3 id="boundaries-you-state-in-conversation">
  Grenzen, die Sie in der Konversation angeben
</h3>

Der Klassifizierer behandelt Grenzen, die Sie in der Konversation angeben, als Blocksignal. Wenn Sie Claude sagen „nicht pushen" oder „warten Sie, bis ich überprüfe, bevor Sie bereitstellen", blockiert der Klassifizierer entsprechende Aktionen, auch wenn die Standardregeln sie erlauben würden. Eine Grenze bleibt in Kraft, bis Sie sie in einer späteren Nachricht aufheben. Claudes eigenes Urteil, dass eine Bedingung erfüllt wurde, hebt sie nicht auf.

Grenzen werden nicht als Regeln gespeichert. Der Klassifizierer liest sie bei jeder Prüfung aus dem Transkript erneut, sodass eine Grenze verloren gehen kann, wenn [Kontext-Komprimierung](/docs/de/costs#reduce-token-usage) die Nachricht entfernt, die sie angegeben hat. Für eine harte Garantie fügen Sie stattdessen eine [Deny-Regel](/docs/de/permissions#permission-rule-syntax) hinzu.

<h3 id="when-auto-mode-falls-back">
  Wenn der Auto-Modus zurückfällt
</h3>

Jede abgelehnte Aktion zeigt eine Benachrichtigung und erscheint in `/permissions` unter der Registerkarte „Kürzlich abgelehnt", wo Sie `r` drücken können, um sie mit manueller Genehmigung erneut zu versuchen.

Wenn der Klassifizierer eine Aktion 3-mal hintereinander oder insgesamt 20-mal blockiert, pausiert der Auto-Modus und Claude Code setzt das Prompting fort. Das Genehmigen der aufgeforderten Aktion setzt den Auto-Modus fort. Diese Schwellwerte sind nicht konfigurierbar. Jede erlaubte Aktion setzt den aufeinanderfolgenden Zähler zurück, während der Gesamtzähler für die Sitzung bestehen bleibt und nur zurückgesetzt wird, wenn sein eigenes Limit einen Fallback auslöst.

Im [nicht-interaktiven Modus](/docs/de/headless) mit dem `-p`-Flag wird die Sitzung abgebrochen, wenn wiederholte Blockierungen auftreten, da es keinen Benutzer zum Prompting gibt.

Wiederholte Blöcke bedeuten normalerweise, dass dem Klassifizierer Kontext über Ihre Infrastruktur fehlt. Verwenden Sie `/feedback`, um falsch positive Ergebnisse zu melden, oder lassen Sie einen Administrator [vertrauenswürdige Infrastruktur konfigurieren](/docs/de/auto-mode-config).

<AccordionGroup>
  <Accordion title="Wie der Klassifizierer Aktionen bewertet">
    Jede Aktion durchläuft eine feste Entscheidungsreihenfolge. Der erste übereinstimmende Schritt gewinnt:

    1. Aktionen, die Ihren [Allow-, Ask- oder Deny-Regeln](/docs/de/permissions#manage-permissions) entsprechen, werden sofort aufgelöst. Schreibvorgänge zu [geschützten Pfaden](#protected-paths) werden zum Klassifizierer geleitet, auch wenn eine Allow-Regel übereinstimmt. Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) und MCP-Tools, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, fordern Sie direkt auf, auch wenn eine Allow-Regel übereinstimmt. Inhalts-bezogene Ask-Regeln fallen auf eine Berechtigungsaufforderung zurück
    2. Read-Only-Aktionen und Dateibearbeitungen in Ihrem Arbeitsverzeichnis werden automatisch genehmigt, außer Schreibvorgänge zu [geschützten Pfaden](#protected-paths)
    3. Alles andere geht zum Klassifizierer. Ein Connector-Tool [das Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) überspringt den Klassifizierer und fordert Sie direkt auf, sodass eine organisatorisch erforderliche Genehmigung niemals automatisch genehmigt wird. {{/* min-version: 2.1.199 */}}Ab v2.1.199 überspringt auch ein MCP-Tool, das mit [`_meta["anthropic/requiresUserInteraction"]`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet ist, den Klassifizierer und fordert Sie direkt auf, sodass ein Zustimmungsschritt niemals im Namen des Tool-Autors automatisch genehmigt wird
    4. Wenn der Klassifizierer blockiert, erhält Claude den Grund und versucht eine Alternative

    Beim Eintritt in den Auto-Modus werden breite Allow-Regeln, die willkürliche Code-Ausführung gewähren, gelöscht:

    * Pauschal `Bash(*)` oder `PowerShell(*)`
    * Wildcard-Interpreter wie `Bash(python*)`
    * Package-Manager-Ausführungsbefehle
    * `Agent` Allow-Regeln

    Enge Regeln wie `Bash(npm test)` werden übernommen. Gelöschte Regeln werden wiederhergestellt, wenn Sie den Auto-Modus verlassen.

    Der Klassifizierer sieht Benutzernachrichten, Tool-Aufrufe und Ihren CLAUDE.md-Inhalt. Tool-Ergebnisse werden entfernt, sodass feindselige Inhalte in einer Datei oder Webseite ihn nicht direkt manipulieren können. Ein separater Server-seitiger Probe scannt eingehende Tool-Ergebnisse und kennzeichnet verdächtige Inhalte, bevor Claude sie liest. Weitere Informationen darüber, wie diese Schichten zusammenarbeiten, finden Sie in der [Auto-Modus-Ankündigung](https://claude.com/blog/auto-mode) und dem [Engineering Deep Dive](https://www.anthropic.com/engineering/claude-code-auto-mode).
  </Accordion>

  <Accordion title="Wie der Auto-Modus Subagenten handhabt">
    Der Klassifizierer überprüft [Subagenten](/docs/de/sub-agents)-Arbeit an drei Punkten:

    1. Bevor ein Subagent startet, wird die delegierte Aufgabenbeschreibung bewertet, sodass eine gefährlich aussehende Aufgabe beim Spawn blockiert wird.
    2. Während der Subagent läuft, durchläuft jede seiner Aktionen den Klassifizierer mit den gleichen Regeln wie die übergeordnete Sitzung, und jeder `permissionMode` in der Frontmatter des Subagenten wird ignoriert.
    3. Wenn der Subagent fertig ist, überprüft der Klassifizierer seine vollständige Aktionshistorie; wenn diese Rückgabeprüfung ein Problem kennzeichnet, wird eine Sicherheitswarnung den Ergebnissen des Subagenten vorangestellt.

    {{/* min-version: 2.1.178 */}}

    Schritt 1 erfordert Claude Code v2.1.178 oder später. Frühere Versionen wendeten den Klassifizierer in den Schritten 2 und 3 an, bewerteten aber die Aufgabenbeschreibung nicht, bevor der Subagent startete.
  </Accordion>

  <Accordion title="Kosten und Latenz">
    Der Klassifizierer läuft auf einem Server-konfigurierten Modell, das unabhängig von Ihrer `/model`-Auswahl ist, sodass das Wechseln von Modellen die Klassifizierer-Verfügbarkeit nicht ändert. Klassifizierer-Aufrufe zählen zu Ihrer Token-Nutzung. Jede Prüfung sendet einen Teil des Transkripts plus die ausstehende Aktion und fügt eine Hin- und Rückfahrt vor der Ausführung hinzu. Lesevorgänge und Arbeitsverzeichnis-Bearbeitungen außerhalb geschützter Pfade überspringen den Klassifizierer, sodass der Overhead hauptsächlich von Shell-Befehlen und Netzwerkoperationen kommt. {{/* min-version: 2.1.198 */}}Ab v2.1.198 wird ein Sandbox-Netzwerk-Urteil für einen Host und Port wiederverwendet, statt bei jeder Verbindung neu klassifiziert zu werden, sodass wiederholte Verbindungen zum gleichen Host nicht jeweils eine Prüfung hinzufügen. [Was der Klassifizierer standardmäßig blockiert](#what-the-classifier-blocks-by-default) beschreibt, wie lange ein Allow und ein Deny dauern.
  </Accordion>
</AccordionGroup>

<h2 id="allow-only-pre-approved-tools-with-dontask-mode">
  Nur vorab genehmigte Tools mit dontAsk-Modus zulassen
</h2>

Wenn Sie den `dontAsk`-Modus einstellen, lehnt Claude Code automatisch jeden Tool-Aufruf ab, der sonst zu einer Eingabeaufforderung führen würde. Claude führt nur Aktionen aus, die Ihren `permissions.allow`-Regeln, [schreibgeschützten Bash-Befehlen](/docs/de/permissions#read-only-commands) und Aufrufen entsprechen, die von einem [PreToolUse Hook](/docs/de/permissions#extend-permissions-with-hooks) genehmigt wurden. Verwenden Sie diesen Modus für CI-Pipelines oder eingeschränkte Umgebungen, in denen Sie genau vordefinieren, was Claude tun darf; die Sitzung wartet nie auf Eingaben. Die Statusleiste zeigt `⏵⏵ don't ask on`, während dieser Modus aktiv ist.

Claude Code lehnt Aufrufe ab, die Ihren expliziten [`ask`-Regeln](/docs/de/permissions#manage-permissions) entsprechen, anstatt zu einer Eingabeaufforderung zu führen. Es lehnt auch das integrierte `AskUserQuestion`-Tool und Connector-Tools ab, [die Ihre Organisation auf `ask` eingestellt hat](/docs/de/mcp#organization-controls-on-connector-tools), auch wenn Ihre Allow-Regeln damit übereinstimmen. Es lehnt MCP-Tools, die mit [`_meta["anthropic/requiresUserInteraction"]`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, auf die gleiche Weise ab, da ihre Genehmigungskarte eine Antwort benötigt, die dieser Modus nie erfasst; dies erfordert Claude Code v2.1.199 oder später.

Cloud-Sitzungen auf [Claude Code im Web](/docs/de/claude-code-on-the-web) ignorieren `defaultMode: "dontAsk"`; siehe [bypassPermissions](#skip-all-checks-with-bypasspermissions-mode) für Details.

Legen Sie es beim Start mit dem Flag fest:

```bash theme={null}
claude --permission-mode dontAsk
```

<h2 id="skip-all-checks-with-bypasspermissions-mode">
  Alle Überprüfungen mit dem Modus bypassPermissions überspringen
</h2>

Der Modus `bypassPermissions` deaktiviert Berechtigungsaufforderungen und Sicherheitsüberprüfungen, sodass Werkzeugaufrufe sofort ausgeführt werden, einschließlich Schreibvorgänge in [geschützte Pfade](#protected-paths). Vor v2.1.126 haben geschützte Pfade in diesem Modus noch Aufforderungen angezeigt.

Explizite [ask-Regeln](/docs/de/permissions#manage-permissions) und Connector-Werkzeuge, [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools), erzwingen in diesem Modus weiterhin eine Aufforderung. MCP-Werkzeuge, die mit [`_meta["anthropic/requiresUserInteraction"]`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, lösen ebenfalls weiterhin eine Aufforderung aus; dies erfordert Claude Code v2.1.199 oder später.

Löschvorgänge, die das Dateisystem-Stammverzeichnis oder das Home-Verzeichnis betreffen, wie `rm -rf /` und `rm -rf ~`, lösen weiterhin als Schutzmaßnahme gegen Modellfehler eine Aufforderung aus. Die Schutzmaßnahme wird auch ausgelöst, wenn der Befehl Befehlsersetzung mit `$(...)` oder Backticks oder Prozessersetzung mit `<(...)` enthält, unabhängig davon, ob sich der Löschvorgang in der Ersetzung befindet, wie in `echo "$(rm -rf ~)"`, oder an anderer Stelle im selben Befehl. Die einfache Form, eingegeben als eigenständiger Befehl, hat in diesem Modus seit der Einführung der Schutzmaßnahme eine Aufforderung ausgelöst; vor v2.1.208 haben Befehle, die diese Formen enthielten, keine Aufforderung ausgelöst.

<Warning>
  Verwenden Sie diesen Modus nur in isolierten Umgebungen wie Containern, VMs oder Dev-Containern ohne Internetzugang, in denen Claude Code Ihr Host-System nicht beschädigen kann.
</Warning>

Sie können `bypassPermissions` nicht aus einer Sitzung eingeben, die ohne eines der aktivierenden Flags gestartet wurde. Starten Sie mit einem Flag neu, um es zu aktivieren:

```bash theme={null}
claude --permission-mode bypassPermissions
```

Das Flag `--dangerously-skip-permissions` ist gleichwertig.

Unter Linux und macOS weigert sich Claude Code, in diesem Modus zu starten, wenn es als Root oder unter `sudo` ausgeführt wird:

```text theme={null}
--dangerously-skip-permissions cannot be used with root/sudo privileges for security reasons
```

Die Überprüfung wird automatisch in einer erkannten Sandbox übersprungen. Um autonom in einem Container zu laufen, verwenden Sie die [Dev-Container](/docs/de/devcontainer)-Konfiguration, die Claude Code als Nicht-Root-Benutzer ausführt.

[Claude Code im Web](/docs/de/claude-code-on-the-web) berücksichtigt `defaultMode: "bypassPermissions"` oder `"dontAsk"` aus Ihren Einstellungsdateien nicht, daher können die eingecheckten Einstellungen eines Repositorys keine Cloud-Sitzung im Bypass-Permissions-Modus starten. Die Einstellung wird stillschweigend ignoriert und die Sitzung startet im Modus, der in der Modus-Dropdown angezeigt wird. Siehe [Berechtigungsmodi wechseln](#switch-permission-modes), welche Modi Cloud-Sitzungen bieten.

<Warning>
  `bypassPermissions` bietet keinen Schutz vor Prompt-Injection oder unbeabsichtigten Aktionen. Verwenden Sie stattdessen den [Auto-Modus](#eliminate-prompts-with-auto-mode) für Hintergrund-Sicherheitsüberprüfungen mit deutlich weniger Berechtigungsaufforderungen. Administratoren können diesen Modus blockieren, indem sie `permissions.disableBypassPermissionsMode` in [verwalteten Einstellungen](/docs/de/permissions#managed-settings) auf `"disable"` setzen.
</Warning>

<h2 id="protected-paths">
  Geschützte Pfade
</h2>

Schreibvorgänge in eine kleine Anzahl von Pfaden werden niemals automatisch genehmigt, in jedem Modus außer `bypassPermissions`. Dies verhindert versehentliche Beschädigungen des Repository-Status und der eigenen Konfiguration von Claude.

| Modus                            | Schreibvorgänge in geschützten Pfaden |
| :------------------------------- | :------------------------------------ |
| `default`, `acceptEdits`, `plan` | Abgefragt                             |
| `auto`                           | An den Klassifizierer weitergeleitet  |
| `dontAsk`                        | Verweigert                            |
| `bypassPermissions`              | Erlaubt                               |

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

<h2 id="see-also">
  Siehe auch
</h2>

* [Berechtigungen](/docs/de/permissions): allow-, ask- und deny-Regeln; verwaltete Richtlinien
* [Auto-Modus konfigurieren](/docs/de/auto-mode-config): teilen Sie dem Klassifizierer mit, welche Infrastruktur Ihre Organisation vertraut
* [Hooks](/docs/de/hooks): benutzerdefinierte Berechtigungslogik über `PreToolUse` und `PermissionRequest` Hooks
* [Ultraplan](/docs/de/ultraplan): führen Sie Plan Mode in einer Claude Code-Websitzung mit browsergestützter Überprüfung aus
* [Sicherheit](/docs/de/security): Schutzmaßnahmen und Best Practices
* [Sandboxing](/docs/de/sandboxing): Dateisystem- und Netzwerkisolation für Bash-Befehle
* [Nicht-interaktiver Modus](/docs/de/headless): führen Sie Claude Code mit dem Flag `-p` aus
