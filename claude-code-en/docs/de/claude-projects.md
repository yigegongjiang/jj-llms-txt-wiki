> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Lassen Sie Claude laufende Arbeiten mit Projekten koordinieren

> Geben Sie Claude einen Bestand zusammenhängender Arbeiten in einem Gespräch und lassen Sie ihn parallele Cloud-Sitzungen koordinieren, die Repositorys, Anweisungen und Speicher gemeinsam nutzen.

<Note>
  Projekte befinden sich in der öffentlichen Beta auf Pro- und Max-Plänen und werden schrittweise eingeführt, beginnend mit Konten, die [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) verwendet haben und keine vorhandenen Projekte in claude.ai-Chat oder Cowork haben. Sie sind noch nicht auf Team- oder Enterprise-Plänen verfügbar. Wenn **Projekte** nicht in der Seitenleiste unter [claude.ai/code](https://claude.ai/code) oder auf der Registerkarte „Code" der [Desktop-App](/docs/de/desktop) angezeigt wird, hat die Einführung Ihr Konto noch nicht erreicht, und Sie können sich [auf die Warteliste eintragen](https://claude.com/form/projects). [Agenten parallel ausführen](/docs/de/agents) listet auf, was Sie in der Zwischenzeit verwenden können.
</Note>

Ein Projekt ist ein laufendes Gespräch, in dem Claude einen Strom zusammenhängender Arbeiten für Sie koordiniert. Sie teilen ihm mit, was getan werden muss, und er startet einen Thread für jede Aufgabe.

Jeder Thread ist normalerweise eine [Cloud-Sitzung](/docs/de/claude-code-on-the-web): Claude Code, das in der Cloud statt auf Ihrem Computer ausgeführt wird. Wenn eine Aufgabe etwas benötigt, das nur Ihr Computer hat, können Sie Claude bitten, diesen Thread stattdessen auf Ihrem Computer durch [Remote Control](/docs/de/remote-control) auszuführen. Threads laufen parallel und Sie können sie von Ihrem Telefon aus überprüfen und steuern. Cloud-Threads laufen weiter, nachdem Sie den Laptop schließen.

Ohne ein Projekt bedeutet das Ausführen mehrerer Sitzungen, dass Sie die Koordination selbst übernehmen: Sie entscheiden, woran jede arbeitet, wiederholen denselben Hintergrund am Anfang jeder Sitzung und überprüfen, welche abgeschlossen ist oder eine Antwort benötigt. Mit einem Projekt tun Sie stattdessen:

* **Arbeiten an einen Ort senden**: Fügen Sie einen Fehlerbericht, eine Stack-Trace oder eine Liste von Aufgaben in das Gespräch ein, wann immer eine auftaucht. Claude startet einen Thread für jede Arbeit oder leitet sie an den Thread weiter, der bereits in diesem Bereich arbeitet, und beantwortet schnelle Fragen direkt.
* **Kontext einmal festlegen**: Jeder neue Thread beginnt mit den Anweisungen des Projekts, sodass eine Regel, die Sie einmal angeben, wie z. B. welcher Branch angestrebt werden soll, alle erreicht.
* **Weggehen und zur abgeschlossenen Arbeit zurückkehren**: Wenn Sie eine Stunde später oder am nächsten Morgen zurückkommen, zeigt der Bereich **Übersicht** an, welche Threads abgeschlossen sind, welche Pull Requests zur Überprüfung bereit sind und welcher Thread auf Ihre Antwort wartet.

Wenn Sie bereits wissen, welche Arbeiten ein Projekt ausführen soll, gehen Sie direkt zu [Projekt erstellen](#create-a-project).

<h2 id="when-to-use-a-project">
  Wann ein Projekt verwendet werden sollte
</h2>

Ein Projekt lohnt sich, wenn die Arbeit ein Ziel hat, das eine Sitzung übersteigt und weiterhin Aufgaben erzeugt. Diese Arten von Arbeiten eignen sich gut für ein Projekt:

* **Ein Ziel über viele Repositories**: „Bringen Sie jeden Service auf die neue Lint-Konfiguration." Claude kann einen Thread pro Repository ausführen, jeder mit seinem eigenen Pull Request, und der [**Übersichts**-Bereich](#see-what-needs-you-in-overview) zeigt, welche zur Überprüfung bereit sind.
* **Ein Bereich, den Sie ständig füttern**: Die Fehler, Stack-Traces und Überprüfungsanfragen für einen Service, die in das Gespräch eingefügt werden, wenn sie Sie erreichen. Eine Fallstricke, die Sie Claude nach einer Reparatur merken lassen, ist im [Projektgedächtnis](#give-a-project-standing-context) für die nächste.
* **Ein Build oder eine Migration größer als eine Sitzung**: „Bauen Sie, was `docs/spec.md` beschreibt" oder „Verschieben Sie die App weg vom veralteten ORM." Die Arbeit teilt sich in Threads auf, die jeweils einen Teil übernehmen, Entscheidungen, die Sie Claude früh merken lassen, erreichen die späteren Threads, und die Spec-Änderungen und Fehler, die Sie während des Builds finden, gehen in das gleiche Gespräch.
* **Arbeit, die kein Code ist**: Ein Ordner mit Verträgen oder ein Support-Ticket-Export, zu dem Sie ständig mit neuen Fragen zurückkehren, wie z. B. „Finden Sie die zehn häufigsten Integrationsfehler in diesen Tickets." Laden Sie die Dokumente hoch, anstatt ein Repository hinzuzufügen, und Threads liefern jede Zusammenfassung als Datei auf der Registerkarte [**Bibliothek**](#see-what-needs-you-in-overview) des Projekts.

In jedem dieser Fälle können Sie einen Batch von Aufgaben senden, Claude anweisen, ohne Sie zur Bestätigung aufzufordern zu starten, weggehen und die Threads finden, die Sie unter [**Warten auf Sie**](#see-what-needs-you-in-overview) benötigen, wenn Sie zurück sind, oder Claude bitten, einen Teil der Arbeit als [Routine](/docs/de/routines) zu planen. Wenn einer dieser Fälle Ihre Situation ist, [erstellen Sie ein Projekt](#create-a-project).

<h3 id="when-something-else-fits-better">
  Wann etwas anderes besser passt
</h3>

Cloud-Threads funktionieren auf GitHub-Repositories und auf Dateien, Ordnern und Google Drive-Ordnern, die Sie in das Projekt hochladen, nicht auf Dateien oder Tools, die nur auf Ihrem Computer vorhanden sind. Wenn eine Aufgabe Ihren Computer benötigt, bitten Sie Claude, seinen Thread dort über [Remote Control](/docs/de/remote-control) auszuführen. [Einschränkungen](#limitations) listet auf, was dafür erforderlich ist. Etwas anderes passt besser in diesen Fällen:

* **Eine Aufgabe, die in eine Sitzung passt**: „Reparieren Sie den instabilen Login-Test." Starten Sie selbst eine [Cloud-Sitzung](/docs/de/claude-code-on-the-web).
* **Arbeit, bei der jede Aufgabe Ihren Computer benötigt**: eine lokale Datenbank, einen Geräteemulator oder eine API hinter Ihrem VPN. Verwenden Sie eine lokale Sitzung oder [Agent-Ansicht](/docs/de/agent-view), um mehrere gleichzeitig auszuführen. Wenn die Arbeit nur lokale Dateien benötigt, laden Sie sie stattdessen in das Projekt hoch.
* **Eine Aufgabe, die sich nach einem Zeitplan ohne Gespräch wiederholt**: „Veröffentlichen Sie jeden Montag einen Abhängigkeitsbericht." Erstellen Sie eine [Routine](/docs/de/routines) für sich allein.
* **Mehrere Personen geben Claude Arbeit und lenken sie zusammen in einem Slack-Kanal**: Siehe [Claude Tag](https://claude.com/docs/claude-tag/overview).

Ein Projekt nutzt die gleichen Planlimits wie Ihre anderen Claude Code-Sitzungen und nutzt sie schneller. [Nutzung und Kosten](#usage-and-cost) behandelt, was Ihren Plan nutzt und wie Sie es niedrig halten.

<h2 id="how-a-project-is-organized">
  Wie ein Projekt organisiert ist
</h2>

Ein Projekt ist eine koordinierende Konversation mit Claude plus die Threads, die es startet, um die Arbeit zu erledigen. Dies sind seine Teile:

* **Die Projektkonversation**: eine lange laufende Sitzung, in der Claude als Koordinator fungiert. Sie nimmt das auf, was Sie senden, entscheidet, was zu einem Thread wird, und verfolgt jeden Thread, den sie gestartet hat. Sie sieht, was Threads zurückberichten, nicht jeden Schritt, den sie unternehmen.
* **Threads**: die Worker. Jeder ist eine separate Sitzung mit seinem eigenen Kontextfenster, die ein Stück Arbeit erledigt und der Konversation Bericht erstattet, wenn sie fertig ist. Ein Cloud-Thread arbeitet in seinem eigenen Branch und öffnet einen Pull Request, wenn die Arbeit dies erfordert.
* **Was jeder Cloud-Thread startet mit**:
  * Die Repositories und Dateien des Projekts, plus seine [Anweisungen und Memory](#give-a-project-standing-context)
  * Die `CLAUDE.md` und Skills in [jedem der Repositories des Projekts](#what-threads-pick-up-from-your-repositories), und in einem Projekt mit einem Repository auch die Berechtigungsregeln und Hooks dieses Repositories
  * Die [Connectors](#get-skills-plugins-connectors-and-tools-into-threads) auf Ihrem claude.ai-Konto
  * Eine [Cloud-Umgebung](#choose-an-environment-for-threads), die seinen Netzwerkzugriff, Umgebungsvariablen, API-Anmeldedaten und installierte Tools festlegt
* **Der Übersichtsbereich**: wo Sie [alle Threads auf einmal sehen](#see-what-needs-you-in-overview) und welche von ihnen Sie benötigen. Seine anderen Registerkarten sind **Library** für die Dateien, die Sie hinzugefügt haben, und die Dateien, die Threads produziert haben, **Pull requests** für die, die Threads geöffnet haben, und **Routines** für geplante Arbeit im Projekt.

Cloud-Threads übernehmen nichts aus dem Claude Code-Setup auf Ihrem eigenen Computer. [Get skills, plugins, connectors, and tools into threads](#get-skills-plugins-connectors-and-tools-into-threads) behandelt, wie Sie ihnen das geben, was ihnen sonst fehlen würde.

Hier ist, wie diese Teile verbunden sind, von Ihnen durch die Konversation zu den Threads, die die Arbeit erledigen, mit **Overview**, das ihren Zustand verfolgt:

<Frame>
  <img src="https://mintcdn.com/claude-code/e8CLbxM17eD7cAiv/images/claude-projects-overview.svg?fit=max&auto=format&n=e8CLbxM17eD7cAiv&q=85&s=dbf446f69f0bbdb9961d21af207cb93b" className="dark:hidden" alt="Diagramm eines Projekts. Sie schreiben in der Projektkonversation, wo Claude antwortet oder einen Thread startet. Jeder Cloud-Thread arbeitet in seinem eigenen Branch und Pull Request. Der Übersichtsbereich listet Threads nach Zustand auf, z. B. bereit zur Überprüfung, wartet auf Sie und arbeitet." width="600" height="250" data-path="images/claude-projects-overview.svg" />

  <img src="https://mintcdn.com/claude-code/e8CLbxM17eD7cAiv/images/claude-projects-overview-dark.svg?fit=max&auto=format&n=e8CLbxM17eD7cAiv&q=85&s=549a5ba9fea8433729babc37a1f6e9c8" className="hidden dark:block" alt="Diagramm eines Projekts. Sie schreiben in der Projektkonversation, wo Claude antwortet oder einen Thread startet. Jeder Cloud-Thread arbeitet in seinem eigenen Branch und Pull Request. Der Übersichtsbereich listet Threads nach Zustand auf, z. B. bereit zur Überprüfung, wartet auf Sie und arbeitet." width="600" height="250" data-path="images/claude-projects-overview-dark.svg" />
</Frame>

<h2 id="create-a-project">
  Erstellen Sie ein Projekt
</h2>

Sie erstellen und verwenden Projekte unter [claude.ai/code](https://claude.ai/code), auf der Registerkarte „Code" der Desktop-App oder in der Claude-Mobile-App für [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) und [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude). Im Browser und in der Desktop-App gibt es zwei Möglichkeiten, ein Projekt zu starten:

* **Von Grund auf**, wenn Sie den Strom von Arbeiten kennen, die Claude ausführen soll: Öffnen Sie den Dialog **Neues Projekt** und benennen Sie ihn. [Starten Sie ein neues Projekt von Grund auf](#start-a-new-project-from-scratch) führt Sie durch den Dialog.
* **Aus einer Cloud-Sitzung, die bereits die Arbeit erledigt**: Wählen Sie **Als Projekt fortfahren** aus dem Menü dieser Sitzung, und Claude schlägt die Projekteinrichtung aus dem vor, was die Sitzung tat. Siehe [Von einer vorhandenen Cloud-Sitzung starten](#start-from-an-existing-cloud-session).

In jedem Fall [überprüfen Sie zuerst die Voraussetzungen](#check-the-prerequisites).

<h3 id="check-the-prerequisites">
  Überprüfen Sie die Voraussetzungen
</h3>

Bevor Sie ein Projekt erstellen, überprüfen Sie Ihren Plan, Ihre GitHub-Einrichtung und was die Arbeit erreichen muss:

* **Plan**: Sie sind auf Pro oder Max und **Projekte** werden in Ihrer Seitenleiste angezeigt.
* **GitHub, wenn das Projekt an Code arbeitet**: Ihr Code ist auf github.com statt auf GitHub Enterprise Server, GitLab oder Bitbucket, Ihr verbundenes GitHub-Konto hat Push-Zugriff darauf, und die Claude GitHub App ist darauf installiert. Wenn Sie GitHub mit [`/web-setup`](/docs/de/web-quickstart#connect-from-your-terminal) verbunden haben, ermöglicht dieses Token Ihren anderen Cloud-Sitzungen, ein Repository zu erreichen, reicht aber nicht für Projekt-Cloud-Threads aus, die die Claude GitHub App benötigen. [Richten Sie GitHub-Zugriff ein](#set-up-github-access) hat die Schritte.
* **Netzwerk, Anmeldedaten und Tools**: Für Cloud-Threads stammen diese aus der [Cloud-Umgebung](#choose-an-environment-for-threads) des Projekts. Die Standardumgebung erreicht bereits [häufige Paket-Registries](/docs/de/cloud-environments#default-allowed-domains), überprüfen Sie dies also nur, wenn die Arbeit andere Domains, ein Geheimnis oder ein Tool benötigt, das nicht vorinstalliert ist. Wenn die Arbeit einen MCP-Server benötigt, überprüfen Sie, dass er als verbunden in Ihren [claude.ai-Connectors](https://claude.ai/customize/connectors) angezeigt wird.

<h3 id="start-a-new-project-from-scratch">
  Starten Sie ein neues Projekt von Grund auf
</h3>

Das Starten eines Projekts von Grund auf bedeutet, den Dialog **Neues Projekt** zu öffnen, den Strom von Arbeiten zu benennen und optional ein Ziel und die Repositories und Dateien anzugeben, an denen es arbeitet. Nur der Name ist erforderlich, daher können Sie das Projekt zuerst erstellen und den Rest ausfüllen, wenn die Arbeit Gestalt annimmt.

<Steps>
  <Step title="Öffnen Sie Projekte">
    Wählen Sie unter [claude.ai/code](https://claude.ai/code) oder auf der Registerkarte „Code" der Desktop-App **Projekte** in der linken Seitenleiste und dann **Neues Projekt**. In einem Browser können Sie auch direkt zu [claude.ai/code/projects/browse](https://claude.ai/code/projects/browse) gehen.
  </Step>

  <Step title="Füllen Sie den Dialog „Neues Projekt&#x22; aus">
    Beschränken Sie das Projekt auf einen Strom von Arbeiten, den Sie ständig hinzufügen, wie z. B. alles, was erforderlich ist, um eine API unter ihrem Latenz-Ziel zu halten. [Wann ein Projekt verwendet werden sollte](#when-to-use-a-project) hat mehr Beispiele. Füllen Sie dann die Felder des Dialogs aus:

    * **Name**: Wie das Projekt in der Liste **Projekte** angezeigt wird.
    * **Ziel** (optional): Eine Zeile, was Sie versuchen zu erreichen, wie z. B. „Halten Sie die p95-API-Latenz unter 200 ms". Claude im Gespräch arbeitet darauf hin. Ohne ein Ziel arbeitet Claude aus den Aufgaben, die Sie senden, und Sie können später ein Ziel in **Projekteinstellungen > Allgemein** hinzufügen.
    * **Kontext** (optional): Die GitHub-Repositories, an denen dieses Projekt arbeitet, plus alle Dateien, Ordner oder Google Drive-Ordner, die Threads lesen sollten. Klicken Sie für jeden auf **Hinzufügen**. Fügen Sie die Repositories hinzu, die die meisten Aufgaben benötigen, anstatt jeden, den die Arbeit möglicherweise berührt; [Entscheiden Sie, welche Repositories hinzugefügt werden sollen](#decide-which-repositories-to-add) behandelt die Wahl, und Sie können später mehr in **Projekteinstellungen > Umgebung** hinzufügen.

    Stehende Regeln für die Funktionsweise von Threads gehen in [Projektanweisungen](#give-a-project-standing-context), die Sie nach der Projekterstellung festlegen.
  </Step>

  <Step title="Erstellen Sie das Projekt">
    Klicken Sie auf **Projekt erstellen**. Das Projektgespräch öffnet sich mit einem Nachrichtenfeld am unteren Rand, in dem Sie Arbeit für Claude beschreiben.

    Bei Ihrem ersten Projekt macht Claude einen eigenen Zug, sobald das Projekt erstellt wird, es sei denn, Sie senden zuerst eine Nachricht. Dieser Zug nutzt Ihren Plan. Darin kann Claude:

    * Einen Thread starten, der das Repository erkundet, ohne etwas zu ändern und nächste Schritte vorzuschlagen, wenn das Projekt ein Repository hat, das es lesen kann.
    * **Setup-Empfehlungen** veröffentlichen, die aus Ihren letzten Cloud-Sitzungen stammen: Repositories zum Hinzufügen, Routinen zum Erstellen und Threads, die es starten könnte. Jedes empfohlene Repository und jede Routine ist standardmäßig aktiviert. Schalten Sie die aus, die Sie nicht möchten, klicken Sie dann auf **Setup aktualisieren**, um den Rest hinzuzufügen, oder ignorieren Sie die Empfehlungen und beschreiben Sie die Arbeit selbst.
  </Step>
</Steps>

Das Projekt wird nun unter **Projekte** in der Seitenleiste aufgelistet, und sein Gespräch ist offen. [Ihr erster Batch](#your-first-batch) behandelt, was Sie einrichten müssen, bevor Sie ihm Arbeit senden.

<h3 id="start-from-an-existing-cloud-session">
  Von einer vorhandenen Cloud-Sitzung starten
</h3>

Wenn Sie bereits eine Cloud-Sitzung haben, die Arbeit erledigt, die in ein Projekt gehört, öffnen Sie das Menü der Sitzung in der Seitenleiste und wählen Sie **Als Projekt fortfahren** oder **In Projekt verschieben**:

* **Als Projekt fortfahren** erstellt ein neues Projekt mit dem Namen der Sitzung und öffnet es. Claude liest die Sitzung und veröffentlicht **Setup-Empfehlungen** im Gespräch zur Bestätigung. Die ursprüngliche Sitzung bleibt in Ihrer Sitzungsliste, und wenn sie sich in der Mitte eines Zuges befand, läuft sie weiter, daher stoppen Sie sie selbst, wenn Sie nicht beide gleichzeitig arbeiten möchten. Wenn Sie stattdessen das Banner **Projekt einrichten** verwenden, das über dem Nachrichtenfeld einer Cloud-Sitzung angezeigt werden kann, ist das Ergebnis das gleiche, außer dass der laufende Zug der Sitzung stoppt, sobald das Projekt öffnet.
* **In Projekt verschieben** bringt die Arbeit der Sitzung in ein vorhandenes Projekt. Es veröffentlicht eine Nachricht in diesem Projektgespräch und bittet Claude, die Sitzung zu lesen und dort fortzufahren, wo sie aufgehört hat, und neue Arbeiten werden in den eigenen Threads des Projekts fortgesetzt. Die ursprüngliche Sitzung bleibt in Ihrer Sitzungsliste, unverändert.

<h3 id="set-up-github-access">
  Richten Sie GitHub-Zugriff ein
</h3>

Die meisten GitHub-Einrichtungen erfolgen einmal, nicht pro Projekt. Sie verbinden Ihr GitHub-Konto einmal mit Claude, und die Claude GitHub App wird einmal pro Repository oder einmal für eine ganze GitHub-Organisation installiert, wenn Sie ihr alle Repositories geben. Sie kehren zu diesen Schritten zurück, wenn Sie ein Repository hinzufügen, das die Claude GitHub App noch nicht abdeckt, oder eines in einer GitHub-Organisation, die SSO erzwingt.

<Steps>
  <Step title="Verbinden Sie Ihr GitHub-Konto">
    Wenn Sie claude.ai/code noch nicht verwendet haben, führt Ihr erster Besuch Sie durch die Verbindung mit GitHub; siehe [GitHub verbinden](/docs/de/web-quickstart#connect-github). Verwenden Sie andernfalls eine der [GitHub-Authentifizierungsoptionen](/docs/de/claude-code-on-the-web#github-authentication-options).
  </Step>

  <Step title="Installieren Sie die Claude GitHub App auf den Repositories des Projekts">
    Installieren Sie die [Claude GitHub App](https://github.com/apps/claude) und gewähren Sie ihr die Repositories, die das Projekt verwenden wird. Auf einem Repository, das einer GitHub-Organisation gehört, kann nur ein Organisationsbesitzer die Installation abschließen; wenn Sie nicht einer sind, sendet GitHub dem Besitzer eine Installationsanfrage und das Projekt kann das Repository nicht verwenden, bis er sie genehmigt.
  </Step>

  <Step title="Autorisieren Sie SSO für Organisationen, die es erzwingen">
    Wenn eine GitHub-Organisation SAML SSO erzwingt, verbinden Sie GitHub erneut und autorisieren Sie die Claude-App für diese Organisation. Bis Sie dies tun, werden die privaten Repositories dieser Organisation nicht im Dialog **Neues Projekt** oder **Projekteinstellungen > Umgebung** angezeigt.
  </Step>
</Steps>

Wenn einer dieser Schritte unvollständig ist, benennen Sie der Dialog **Neues Projekt** und die Projektseite den fehlenden Schritt und verlinken Sie, wo Sie ihn abschließen. Schließen Sie den Schritt dort ab, klicken Sie dann auf **Erneut überprüfen**, wenn der Dialog es anbietet. Wenn ein Repository danach immer noch in der Liste fehlt, öffnen Sie die Installation der Claude GitHub App auf GitHub unter [github.com/settings/installations](https://github.com/settings/installations) für ein persönliches Konto und bestätigen Sie, dass das Repository unter **Repository-Zugriff** aufgelistet ist. Für die Fehlermeldungen, die ein Thread oder das Projekt meldet, wenn der Zugriff immer noch falsch ist, siehe [Repository-Zugriffsfehler](#repository-access-errors).

<h2 id="work-in-a-project">
  Arbeiten Sie in einem Projekt
</h2>

Geben Sie Claude Arbeit durch das Projektgespräch: Aufgaben einzeln oder mehrere auf einmal, plus Updates und lose Gedanken, wenn sie auftauchen. Claude leitet jede Nachricht weiter, und Threads erledigen die Arbeit und berichten zurück.

<h3 id="your-first-batch">
  Ihr erster Batch
</h3>

Bevor Sie einem neuen Projekt einen Batch von Arbeiten senden, richten Sie es so ein, dass die ersten Threads auf die gewünschte Weise zurückkommen:

1. [Schreiben Sie Projektanweisungen](#write-project-instructions): Das Briefing, mit dem jeder Thread startet, wie z. B. welcher Branch angestrebt werden soll, wie ein Thread seine Arbeit überprüft und was Ihre Genehmigung benötigt.
2. Senden Sie ein kleines Stück der echten Arbeit, oder starten Sie einen der Threads, die Claude vorgeschlagen hat, wenn er welche angeboten hat, und öffnen Sie den Thread, wenn er fertig ist, um zu sehen, wie er zurückberichtet und was er auf seinem Branch tat. Wenn es etwas Falsches annahm oder nicht erreichen konnte, was es brauchte, [Threads haben geraten oder stecken fest, anstatt zu fragen](#threads-guessed-or-stalled-instead-of-asking) behandelt, wo das zu beheben ist.
3. Überprüfen Sie **Thread-Modell** und **Thread-Aufwand** in **Projekteinstellungen > Allgemein**. Ein neues Projekt führt jeden Thread auf Opus mit hohem Aufwand aus, was Ihren Plan am schnellsten nutzt; [Wählen Sie Modelle und lassen Sie Claude den Kontext verwalten](#choose-models-and-let-claude-manage-context) behandelt die Alternativen.
4. Bitten Sie Claude, [Threads vor dem Start vorzuschlagen und nur wenige gleichzeitig auszuführen](#tune-how-claude-runs-a-project), und lassen Sie diese Limits fallen, sobald ein paar Threads auf die gewünschte Weise zurückkommen.

<h3 id="send-work-and-read-results">
  Senden Sie Arbeit und lesen Sie Ergebnisse
</h3>

Claude entscheidet, wohin jede Nachricht, die Sie im Gespräch senden, geht:

* Eine schnelle Frage bekommt normalerweise eine Antwort im Gespräch.
* Neue Arbeit geht zu einem neuen Thread oder zu einem Thread, der bereits in diesem Bereich arbeitet, und Claude teilt Ihnen mit, welcher. Jeder neue Thread wird unter Ihrer Nachricht als Karte angezeigt: ein Feld mit dem Titel und Status des Threads, das Sie klicken, um den Thread zu öffnen.
* Mehrere unabhängige Aufgaben in einer Nachricht werden zu separaten Threads.

Wenn Claude etwas anders leitet, als Sie wollten, sagen Sie es. [Passen Sie an, wie Claude ein Projekt ausführt](#tune-how-claude-runs-a-project) listet Dinge auf, die Sie ihm sagen können, wie z. B. einen vorhandenen Thread für Nachverfolgungen wiederzuverwenden oder stattdessen an Ort und Stelle zu antworten.

Die vollständigen Ergebnisse eines Threads bleiben im Thread, und Sie öffnen seine Karte im Gespräch, um sie zu lesen. Dateien, die ein Thread erzeugt hat, sind auch auf der Registerkarte **Bibliothek** in **Übersicht**.

Manchmal schlägt Claude Threads vor, anstatt sie zu starten, in einer Liste **Vorgeschlagene Threads**. Klicken Sie auf den Pfeil bei einem Vorschlag, um diesen Thread zu starten. Wenn mehrere aufgelistet sind, startet eine Schaltfläche unter der Liste alle.

<h3 id="review-a-thread’s-pull-request">
  Überprüfen Sie den Pull Request eines Threads
</h3>

Wenn ein Thread Code ändert, tut er dies, es sei denn, Sie sagen ihm etwas anderes:

* **Branch**: Arbeitet an einem neuen Branch, der vom Standard-Branch des Repositories gestartet wird.
* **Pull Request**: Öffnet einen, wenn Sie fragen, und kann einen für eine Fehlerbehebung oder eine andere konkrete Änderung selbst öffnen.
* **Nachdem er öffnet**: Überwacht den Pull Request mit [Auto-Fix](/docs/de/claude-code-on-the-web#auto-fix-pull-requests) aktiviert, unabhängig davon, ob Auto-Fix für Ihre anderen Cloud-Sitzungen aktiviert ist. Es pusht Fixes, wenn CI fehlschlägt, adressiert Review-Kommentare und antwortet im Thread, wenn Checks bestanden werden und der Pull Request zur Überprüfung bereit ist.

Wenn ein Thread einen Branch gepusht oder einen Pull Request geöffnet hat, kann seine Karte im Gespräch eine Schaltfläche für den nächsten Schritt anzeigen:

* **Konflikte auflösen**, **CI reparieren**, **Kommentare adressieren** und **Merge it** senden diese Anweisung als Nachricht von Ihnen an den Thread, daher können Sie den Thread selbst auffordern, anstatt zu warten, dass er auf den Pull Request reagiert.
* **PR überprüfen** öffnet den Pull Request auf GitHub.
* **PR erstellen** wird angezeigt, wenn ein untätiger Thread einen Branch gepusht hat, aber keinen Pull Request geöffnet hat. Wenn Sie darauf klicken, wird der Pull Request direkt aus diesem Branch erstellt, anstatt den Thread anzuweisen, einen zu öffnen.

Um zu ändern, wann Threads Pull Requests öffnen, z. B. nur wenn Sie fragen, oder von welchem Branch sie starten, sagen Sie dies in der Aufgabe oder in [Projektanweisungen](#write-project-instructions).

<h3 id="see-what-needs-you-in-overview">
  Sehen Sie, was Sie in der Übersicht benötigen
</h3>

Der **Übersichts**-Bereich neben dem Gespräch verfolgt die Threads des Projekts. Er ist bereits offen, wenn Sie zum ersten Mal ein neues Projekt öffnen. Die Schaltfläche **Übersicht** in der Projektkopfzeile schließt und öffnet ihn erneut und zeigt einen Punkt, wenn ein Thread auf Sie wartet.

In der Desktop-App erhalten Sie auch eine Desktop-Benachrichtigung, wenn Claude im Gespräch veröffentlicht, ein Thread auf einen Fehler trifft oder ein Thread Ihre Eingabe benötigt, daher müssen Sie das Projekt nicht offen halten, um es herauszufinden. Um auch eine zu erhalten, jedes Mal wenn ein Thread einen Zug beendet, oder um sie für ein Projekt auszuschalten, wählen Sie **Benachrichtigungen** im Menü der Seitenleiste des Projekts. Diese Benachrichtigungen sind nur für den Desktop: Überprüfen Sie in einem Browser den Punkt auf der Schaltfläche **Übersicht**.

Die Registerkarte **Threads** des Bereichs gruppiert Threads nach Zustand:

| Gruppe                     | Was ist darin                                                                                                                                                                                                                                                                    |
| :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Zur Überprüfung bereit** | Threads, deren Pull Request offen und zur Überprüfung bereit ist                                                                                                                                                                                                                 |
| **Warten auf Sie**         | Threads, die Ihre Antwort oder Genehmigung benötigen, oder die fehlgeschlagen sind                                                                                                                                                                                               |
| **Arbeiten**               | Threads, die noch laufen                                                                                                                                                                                                                                                         |
| **Landen**                 | Threads, deren Pull Request genehmigt oder zum Merge in die Warteschlange eingereiht ist                                                                                                                                                                                         |
| **Untätig**                | Threads, die fertig sind und nicht auf etwas warten                                                                                                                                                                                                                              |
| **Gelöst**                 | Threads als erledigt markiert: von Ihnen aus dem Menü des Threads, von Claude, sobald Sie den letzten Schritt unternommen haben, wie z. B. seinen Pull Request zu mergen, oder automatisch nach einer Woche ohne Aktivität. Sie können einen aus dem gleichen Menü erneut öffnen |

Die anderen Registerkarten des Bereichs sind **Bibliothek** für die Dateien und Ordner, die Sie hinzugefügt haben, und die Dateien, die Threads erzeugt haben, **Pull Requests**, sobald Threads welche geöffnet haben, und **Routinen** für die [Routinen](/docs/de/routines), die Claude aus diesem Projekt eingerichtet hat.

<h3 id="open-a-thread-when-you-need-control">
  Öffnen Sie einen Thread, wenn Sie Kontrolle benötigen
</h3>

Klicken Sie auf die Karte eines Threads im Gespräch oder seine Zeile in **Übersicht**, um sein Transkript im Übersichts-Bereich zu öffnen. Von dort aus können Sie:

* Lesen Sie, was Claude tat, Schritt für Schritt.
* Lenken Sie die Aufgabe, indem Sie im eigenen Nachrichtenfeld des Threads schreiben. Eine Nachricht dort geht direkt zu diesem Thread, während eine Nachverfolgung im Projektgespräch ihn nur erreicht, wenn Claude die Nachverfolgung diesem Thread zuordnet.
* Beantworten Sie eine Berechtigungsaufforderung, auf die der Thread wartet.
* Unterbrechen Sie den Thread mit **Stopp**, das die Sendeschaltfläche ersetzt, während der Thread arbeitet, oder durch Drücken von Esc.

<h3 id="choose-models-and-let-claude-manage-context">
  Wählen Sie Modelle und lassen Sie Claude den Kontext verwalten
</h3>

Legen Sie Modelle und Aufwand in **Projekteinstellungen > Allgemein** fest. Ein neues Projekt führt Opus überall aus, mit hohem [Aufwand](/docs/de/model-config#adjust-effort-level) für Threads und niedrigem Aufwand für das Gespräch:

* **Thread-Modell** und **Thread-Aufwand** gelten für Threads. Um ein anderes Modell für eine Aufgabe zu verwenden, fragen Sie danach in der Aufgabe; für einen bereits laufenden Thread verwenden Sie den Modell-Picker dieses Threads.
* **Koordinator-Modell** und **Koordinator-Aufwand** gelten für Claude im Projektgespräch.

Sie verwalten Kontextfenster nicht in einem Projekt. Threads komprimieren automatisch, und das Gespräch funktioniert aus letzten Nachrichten, letzten Threads und Projektgedächtnis statt seiner vollständigen Geschichte, daher läuft es so lange, wie das Projekt läuft. Legen Sie alles, das niemals gelöscht werden darf, in [Projektgedächtnis](#give-a-project-standing-context). Wenn ein Thread seinen Kontext übersteigt, zeigt es [Claude ist auf diesem Zug aus dem Kontext gelaufen](#context-limit).

<h3 id="tune-how-claude-runs-a-project">
  Passen Sie an, wie Claude ein Projekt ausführt
</h3>

Sagen Sie Claude im Gespräch, wie viele Threads gleichzeitig ausgeführt werden sollen, wann Updates veröffentlicht werden sollen und wann Pull Requests geöffnet werden sollen. Wenn Claude auf eine Weise koordiniert, die Sie nicht möchten, sagen Sie es. Zum Beispiel können Sie sagen:

* „Schlagen Sie Threads vor und warten Sie auf meine Genehmigung, bevor Sie sie starten" oder „Starten Sie diese jetzt, ohne mich zur Bestätigung aufzufordern"
* „Führen Sie höchstens zwei Threads gleichzeitig aus" oder „Verwenden Sie einen vorhandenen Thread für Nachverfolgungen im gleichen Bereich erneut"
* „Veröffentlichen Sie kürzere Updates" oder „Veröffentlichen Sie nur, wenn etwas fertig ist oder blockiert"
* „Geben Sie mir ein Status-Update für jeden Thread"
* „Führen Sie diese Aufgabe mit einem kleineren Modell aus"
* „Öffnen Sie keinen Pull Request, bis ich den Plan gesehen habe"
* „Sagen Sie mir, was in diesen Repositories falsch ist, und reparieren Sie nichts", wenn Sie die Erkenntnisse durchgehen möchten, bevor einer von ihnen ein Thread wird
* „Beantworten Sie das hier stattdessen, anstatt einen Thread zu starten", wenn Claude einen Thread für etwas startet, das Sie als schnelle Frage gemeint haben

Claude speichert Voreinstellungen wie diese auf [Projektgedächtnis](#give-a-project-standing-context) selbst und folgt ihnen in späteren Threads. Sie sind Anweisungen, die Claude befolgt, nicht erzwungene Einstellungen, daher ist ein Thread-Limit, das Sie auf diese Weise geben, keine harte Obergrenze. Fügen Sie eines zu Projektanweisungen hinzu, wenn Sie es genau formuliert und auf jeden Thread von Anfang an angewendet haben möchten.

<h3 id="unblock-a-thread-waiting-on-approval">
  Entsperren Sie einen Thread, der auf Genehmigung wartet
</h3>

Threads laufen im [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode), wenn das Modell des Threads ihn unterstützt, daher laufen die meisten Tool-Aufrufe ohne Sie zu fragen. Wenn ein Thread Ihre Genehmigung benötigt, ist die Aufforderung in diesem Thread und der Thread wartet, bis Sie dort antworten. Claude im Projektgespräch zu sagen, dass er fortfahren soll, erreicht ihn nicht.

Jede Genehmigung deckt diese Aufforderung oder den Rest dieses Threads ab, wenn Sie die breitere Option wählen. Um jeden Thread bestimmte Befehle ohne Fragen ausführen zu lassen, oder um einige zu blockieren, fügen Sie [Berechtigungsregeln](/docs/de/permissions) zur `.claude/settings.json` des Repositories hinzu. Threads wenden sie nur in einem Projekt mit einem Repository an; siehe [Was Threads aus Ihren Repositories aufgreifen](#what-threads-pick-up-from-your-repositories).

<h2 id="give-a-project-standing-context">
  Einem Projekt Kontext geben
</h2>

Projektgedächtnis, Projektanweisungen und die Repositories, Dateien und Umgebung des Projekts tragen Kontext über Threads hinweg. Sie legen jedes einmal fest.

| Kontext                            | Was es trägt                                                                                                                                                                                                                                | Wie Sie es festlegen                                                                                                                                                                                                                                                      |
| :--------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Projektgedächtnis                  | Notizen, die Claude über das Projekt führt, wie Anforderungen, Entscheidungen und Fallstricke, gespeichert als Dateien. Jeder Cloud-Thread liest die Indexdatei `MEMORY.md` beim Start und öffnet die anderen Dateien, wenn er sie benötigt | Bitten Sie Claude im Projektgespräch oder in einem beliebigen Cloud-Thread, sich an eine Anforderung, eine Entscheidung oder einen Fallstrick zu erinnern oder einen zu vergessen. Lesen, bearbeiten und löschen Sie die Dateien in **Projekteinstellungen > Gedächtnis** |
| Projektanweisungen                 | Text, der an jeden neuen Thread und an Claude im Projektgespräch gesendet wird, bis zu 16.000 Zeichen. [Projektanweisungen schreiben](#write-project-instructions) behandelt, was Sie darin einfügen sollten                                | **Projekteinstellungen > Gedächtnis > Projektanweisungen**, oder bitten Sie Claude, die Anweisungen zu ändern                                                                                                                                                             |
| Repositories, Dateien und Umgebung | Die Repositories, die jeder Cloud-Thread klont, die Ordner und Dateien, die er unter `/mnt/project-files` lesen kann, und die Cloud-Umgebung, in der er ausgeführt wird                                                                     | Repositories und Umgebung in **Projekteinstellungen > Umgebung**, oder bitten Sie Claude im Gespräch, ein Repository zum Projekt hinzuzufügen. Dateien und Ordner von **Hinzufügen** auf der Registerkarte **Bibliothek** in **Übersicht**                                |

**Projekteinstellungen > Gedächtnis** listet diese Dateien unter **Automatisches Gedächtnis** auf, da Claude sie selbst schreibt, während es im Projekt arbeitet. Sie sind getrennt vom [automatischen Gedächtnis](/docs/de/memory), das Claude Code auf Ihrem Computer führt, obwohl beide einen `MEMORY.md`-Index verwenden. Projektgedächtnis ist auch getrennt von den `CLAUDE.md`-Dateien in den Repositories des Projekts. Jeder Cloud-Thread liest diese `CLAUDE.md`-Dateien immer noch aus seinem Klon beim Start, also fügen Sie Anweisungen zu einem Repository in seine `CLAUDE.md` ein und Notizen zum Projekt in das Projektgedächtnis.

<h3 id="write-project-instructions">
  Projektanweisungen schreiben
</h3>

Projektanweisungen sind das Briefing, mit dem jeder neue Thread beginnt. Klicken Sie auf das Zahnradsymbol in der Projektkopfzeile, um **Projekteinstellungen** zu öffnen, und gehen Sie dann zu **Gedächtnis > Projektanweisungen**. Ein nützliches Briefing behandelt:

* Wofür das Projekt ist
* Wo die Arbeit stattfindet: welche Repositories, von welchem Branch aus zu starten, wie Pull Requests zu benennen sind
* Wie ein Thread seine eigene Arbeit überprüft, bevor er sie für erledigt erklärt
* Was zu tun ist, wenn etwas, das er benötigt, fehlt
* Was zuerst Ihre Zustimmung benötigt

Zum Beispiel:

```text theme={null}
Dieses Projekt hält die p95-Latenz für die Payments-API unter 200 ms: Profiling, Abfrage- und Caching-Fixes und die Abhängigkeits-Upgrades, die damit einhergehen, im payments-api-Repository.

- Branch von main und öffnen Sie einen Draft-Pull-Request pro Thread.
- Bevor Sie die Arbeit für erledigt erklären, führen Sie `make test` und `make lint` aus und fügen Sie die Zusammenfassungszeilen in Ihre abschließende Nachricht ein.
- Wenn Sie etwas nicht erreichen können, das Sie benötigen, wie ein Repository, ein Secret, eine API oder einen Connector, sagen Sie genau, was in Ihrer ersten Nachricht fehlt und stoppen Sie. Ersetzen Sie nicht, mocken Sie nicht und raten Sie nicht.
- Führen Sie keine Merges durch, erzwingen Sie keine Pushes und ändern Sie die CI-Konfiguration nicht, ohne mich im Thread zu fragen.
```

Regeln zu einem Repository, wie seine Build-Befehle, gehören in die `CLAUDE.md` dieses Repositories, die jeder Cloud-Thread liest, wenn das Repository Teil des Projekts ist. Sobald die Arbeit im Gange ist, wenn Sie einen Thread korrigieren, sagen Sie Claude auch, dass er die Korrektur merken soll: Sie geht in das [Projektgedächtnis](#give-a-project-standing-context) und später Threads beginnen damit.

<h3 id="decide-which-repositories-to-add">
  Entscheiden Sie, welche Repositories hinzugefügt werden sollen
</h3>

Die Repositories, die Sie zu einem Projekt hinzufügen, enthalten alles darin, ihren Code, `CLAUDE.md` und Skills, in jedem Cloud-Thread. Repositories, die Sie nicht hinzufügen, sind immer noch erreichbar: Ein Cloud-Thread kann eines zu sich selbst hinzufügen, wenn seine Aufgabe es benötigt. Die meisten Projekte verwenden beide:

* **Fügen Sie es zum Projekt hinzu**, im Dialog **Neues Projekt**, in **Projekteinstellungen > Umgebung**, oder bitten Sie Claude im Gespräch, es zum Projekt hinzuzufügen. Jeder Cloud-Thread von da an klont es und startet mit seiner `CLAUDE.md` und seinen Skills geladen, unabhängig davon, ob die Aufgabe es berührt. Der Wechsel von einem Repository zu mehreren ändert auch, was Threads aus der `.claude/settings.json` jedes Repositories aufgreifen; siehe [Was Threads aus Ihren Repositories aufgreifen](#what-threads-pick-up-from-your-repositories).
* **Lassen Sie es weg und lassen Sie Threads es hinzufügen, wenn nötig.** Ein Cloud-Thread, dessen Aufgabe ein Repository benötigt, das das Projekt nicht hat, kann es zu sich selbst hinzufügen, und eine Notiz im Thread sagt, dass es nur zu diesem Thread hinzugefügt wurde. Der Klon findet teilweise durch die Aufgabe statt, also waren die `CLAUDE.md` und Skills dieses Repositories nicht vorhanden, als der Thread startete. Der nächste Thread startet wieder ohne ihn. Ein Repository, das ein Thread hinzufügt, benötigt die gleichen [Voraussetzungen](#check-the-prerequisites) wie ein Projekt-Repository: die Claude GitHub App ist darauf installiert und Sie haben Push-Zugriff von Ihrem GitHub-Konto.

Ein Projekt benötigt überhaupt kein Repository. Seine Cloud-Threads können immer noch recherchieren, Dokumente schreiben und Code in ihrer eigenen Sandbox schreiben und ausführen, und sie liefern Dateien auf der Registerkarte **Bibliothek** ab. Ein Cloud-Thread dort kann auch ein Repository zu sich selbst hinzufügen, wenn eine Aufgabe es erfordert.

Sobald das Projekt Repositories hat, kann Claude nur Repositories von einem GitHub-Besitzer hinzufügen, den das Projekt bereits verwendet, unabhängig davon, ob es eines zum Projekt hinzufügt oder ein Thread eines zu sich selbst hinzufügt. Um ein Repository von einem anderen Besitzer einzubringen, fügen Sie es selbst in **Projekteinstellungen > Umgebung** zum Projekt hinzu.

Für ein Projekt, das viele Repositories umfasst, wie ein Feature mit Server-, Web-, Mobile- und Desktop-Code, fügen Sie das eine oder zwei Repositories hinzu, die fast jede Aufgabe berührt, und nennen Sie die anderen in [Projektanweisungen](#write-project-instructions), damit Claude weiß, wo der Rest des Codes lebt. Cloud-Threads starten dann klein und ziehen die anderen Repositories nur für die Aufgaben ein, die sie benötigen.

<h3 id="what-threads-pick-up-from-your-repositories">
  Was Threads aus Ihren Repositories aufgreifen
</h3>

Jeder Cloud-Thread klont jedes Repository im Projekt und lädt `CLAUDE.md` und Skills aus allen. Berechtigungsregeln, Hooks und `env` kommen nur aus der `.claude/settings.json` in dem Verzeichnis, in dem der Thread startet: innerhalb des Repositories, wenn das Projekt eines hat, und über den Klonen, wenn es mehrere hat, wo keine Datei eines Repositories für sie gelesen wird.

| In jedem Repository                                                                 | Ein Repository                                                                                                                                        | Mehrere Repositories                                                                        |
| :---------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------ |
| `CLAUDE.md`                                                                         | Geladen, wenn der Thread startet                                                                                                                      | Geladen aus jedem Repository, wenn der Thread startet                                       |
| Skills, Agenten und Befehle unter `.claude/`                                        | Geladen                                                                                                                                               | Geladen aus jedem Repository                                                                |
| Plugins, die in `.claude/settings.json` aktiviert sind                              | Nicht geladen. Fügen Sie das Plugin stattdessen in **Projekteinstellungen > Plugins** hinzu                                                           | Nicht geladen. Fügen Sie das Plugin stattdessen in **Projekteinstellungen > Plugins** hinzu |
| Berechtigungsregeln, Hooks und `env`, die in `.claude/settings.json` definiert sind | Gelten für den Thread, außer den `env`-Schlüsseln, die [keine Cloud-Sitzung berücksichtigt](/docs/de/cloud-environments#what-carries-over-from-your-setup) | Gelten nicht                                                                                |

In einem Projekt mit mehreren Repositories ist jeder Klon an den Thread als [zusätzliches Verzeichnis](/docs/de/memory#load-from-additional-directories) mit aktiviertem `CLAUDE.md`-Laden angehängt, weshalb die `CLAUDE.md` und Skills jedes Repositories beim Start geladen werden, obwohl der Thread über ihnen startet. Legen Sie in einem solchen Projekt stehende Regeln in Projektanweisungen fest und geben Sie Threads Umgebungsvariablen durch die [Cloud-Umgebung](#choose-an-environment-for-threads).

<h3 id="choose-an-environment-for-threads">
  Wählen Sie eine Umgebung für Threads
</h3>

Jeder neue Cloud-Thread startet in der [Cloud-Umgebung](/docs/de/cloud-environments) des Projekts. Die Umgebung legt fest, welche Domains Threads erreichen können, welche Umgebungsvariablen sie haben, welche API-Anmeldedaten zu ihren Anfragen hinzugefügt werden, und was das Setup-Skript installiert, bevor Claude startet. Cloud-Threads verwenden eine Standard-Anthropic-gehostete Umgebung, bis Sie eine in **Projekteinstellungen > Umgebung** auswählen.

Wenn Cloud-Threads eine interne API oder eine private Package-Registry erreichen müssen oder ein Token benötigen, das Ihr Computer normalerweise hält, ändern Sie die Umgebung statt des Projekts: siehe [Netzwerkzugriff](/docs/de/cloud-environments#network-access), [API-Anmeldedaten hinzufügen](/docs/de/cloud-environments#add-api-credentials) und [Setup-Skripte](/docs/de/cloud-environments#setup-scripts).

<h3 id="get-skills-plugins-connectors-and-tools-into-threads">
  Bringen Sie Skills, Plugins, Connectors und Tools in Threads
</h3>

Cloud-Threads haben nicht die Skills, MCP-Server, Plugins und Tools, die nur auf Ihrem Computer installiert sind. Ein Thread, den Claude auf Ihrem Computer durch [Remote Control](/docs/de/remote-control) ausführt, verwendet das, was dort installiert ist. Um jedes dieser Elemente für Cloud-Threads verfügbar zu machen:

* Skills, Subagenten und Befehle: Committen Sie sie zu einem Repository, das Sie zum Projekt hinzugefügt haben, zum Beispiel ein Skill unter `.claude/skills/<skill-name>/SKILL.md`. Jeder Cloud-Thread klont jedes Repository im Projekt und lädt `.claude/skills/`, `.claude/agents/` und `.claude/commands/` aus jedem, also ist ein Skill, der zu einem Repository committed wird, in jedem Cloud-Thread verfügbar. Cloud-Threads laden auch die Skills, die Sie für Ihr claude.ai-Konto aktivieren.
* Plugins: Fügen Sie sie in **Projekteinstellungen > Plugins** hinzu; sie werden in jeden neuen Cloud-Thread geladen. Plugins, die ein Repository in seiner `.claude/settings.json` deklariert, [werden nicht in Cloud-Threads geladen](/docs/de/cloud-environments#what-carries-over-from-your-setup).
* MCP-Server: Cloud-Threads erhalten ihre MCP-Tools von den Connectors auf Ihrem claude.ai-Konto, die MCP-Server sind, die Sie einmal unter [claude.ai/customize/connectors](https://claude.ai/customize/connectors) verbinden oder über den Link **Connectors verwalten** in **Projekteinstellungen > Umgebung**. Jeder Cloud-Thread kann alle mit keinem projektspezifischen Setup verwenden. Das Projektgespräch selbst hat keine Connectors, also senden Sie Arbeit, die einen benötigt, als Aufgabe für einen Cloud-Thread. In einem Projekt mit einem Repository laden Cloud-Threads auch MCP-Server aus der [`.mcp.json`](/docs/de/cloud-environments#what-carries-over-from-your-setup) dieses Repositories. [Wie Connectors Claude Code erreichen](/docs/de/mcp#how-connectors-reach-claude-code) listet die Regeln für Cloud-Sitzungen und die Einstellungen auf, die Connectors ausschalten.
* Befehlszeilenwerkzeuge und Pakete: Installieren Sie sie im [Setup-Skript](/docs/de/cloud-environments#setup-scripts) der Umgebung.

Um zu sehen, welche Connectors ein laufender Cloud-Thread unter claude.ai/code hat, öffnen Sie den Thread und wählen Sie **Connectors** aus dem Menü **+** neben seinem Nachrichtenfeld. Das Ausschalten eines Connectors dort entfernt ihn aus diesem Thread und speichert das als Ihr Kontostandard, also starten neue Threads und claude.ai-Chats ohne ihn, bis Sie ihn wieder einschalten. Ein Cloud-Thread nimmt einen Connector auf, den Sie hinzufügen oder erneut verbinden, nach der nächsten Nachricht, die Sie ihm senden.

<h2 id="project-settings-reference">
  Referenz für Projekteinstellungen
</h2>

Sie ändern Projekteinstellungen unter claude.ai/code oder in der Desktop-App, nicht in `settings.json`. Öffnen Sie **Projekteinstellungen** über **Einstellungen** im Seitenmenü des Projekts oder über das Zahnradsymbol in der Projektkopfzeile.

Einstellungen werden gespeichert, während Sie sie ändern; ein Textfeld, das Sie bearbeiten, z. B. das Ziel oder die Anweisungen, zeigt **Änderungen speichern** und **Verwerfen** an, bis Sie es verlassen. Änderungen an Anweisungen, Repositories, Plugins und Umgebung in **Projekteinstellungen** erreichen neue Threads, nicht bereits laufende Threads.

| Einstellung                     | Abschnitt | Was es steuert                                                                                                                      |
| :------------------------------ | :-------- | :---------------------------------------------------------------------------------------------------------------------------------- |
| Name, Symbol und Ziel           | Allgemein | Der Name und das Symbol des Projekts in der Seitenleiste sowie sein einzeiliges Ziel                                                |
| Koordinator-Modell und Aufwand  | Allgemein | Das Modell und die [Aufwandsebene](/docs/de/model-config#adjust-effort-level) für Claude in der Projektkonversation                      |
| Thread-Modell und Aufwand       | Allgemein | Das Modell und die Aufwandsebene für Threads                                                                                        |
| Projektanweisungen              | Memory    | [Stehende Regeln](#give-a-project-standing-context), die jeder neue Thread erhält                                                   |
| Projekt-Repositories            | Umgebung  | Die Repositories, die neue Cloud-Threads klonen                                                                                     |
| Cloud-Umgebung                  | Umgebung  | Die [Cloud-Umgebung](#choose-an-environment-for-threads), in der neue Cloud-Threads ausgeführt werden                               |
| Connectors                      | Umgebung  | Ein Link zum Verwalten der claude.ai-Connectors, die Cloud-Threads erhalten                                                         |
| Plugins                         | Plugins   | Die Plugins, die in jeden neuen Cloud-Thread geladen werden                                                                         |
| Nutzung                         | Nutzung   | [Token-Nutzung](#usage-and-cost) nach Thread und nach Modell                                                                        |
| Memory                          | Memory    | Die [Memory-Dateien](#give-a-project-standing-context) des Projekts                                                                 |
| Claude neu starten              | Allgemein | Startet die Projektkonversation neu, wenn [Claude dort nicht mehr antwortet](#claude-hasnt-responded)                               |
| Pausieren, Archivieren, Löschen | Allgemein | Stoppt, verbirgt oder entfernt das Projekt; siehe [Projekt pausieren, archivieren oder löschen](#pause-archive-or-delete-a-project) |

<h3 id="pause-archive-or-delete-a-project">
  Projekt pausieren, archivieren oder löschen
</h3>

Alle drei Steuerelemente befinden sich am unteren Ende von **Projekteinstellungen > Allgemein**:

* **Pausieren**: stoppt alles auf einmal. Jeder laufende Thread und die Konversation werden unterbrochen, keine neuen Threads starten, Routinen werden nicht ausgeführt, und das Projekt akzeptiert keine Nachrichten, bis Sie es fortsetzen. Klicken Sie auf **Fortsetzen** an derselben Stelle oder auf dem Banner über dem Nachrichtenfeld des Projekts; ein pausierter Thread wird fortgesetzt, wenn Sie ihm danach eine Nachricht senden.
* **Archivieren**: verbirgt das Projekt aus der Seitenleiste und archiviert seine Threads, was jeden Thread stoppt, der lief oder einen Pull Request überwachte. Routinen im Projekt werden nicht ausgeführt, während es archiviert ist. Um das Projekt zurückzubringen, öffnen Sie es von der Seite „Projekte" und klicken Sie auf **Archivierung aufheben**. Seine Threads bleiben archiviert, bis Sie sie einzeln aus der Sitzungsliste archivieren aufheben.
* **Löschen**: entfernt das Projekt dauerhaft zusammen mit seinen Threads, seinem Memory und seinen Dateien und deaktiviert die Routinen des Projekts. Dies kann nicht rückgängig gemacht werden. Branches und Pull Requests, die die Threads zu GitHub gepusht haben, sind nicht betroffen.

<h2 id="usage-and-cost">
  Nutzung und Kosten
</h2>

Die Projektnutzung zählt gegen die gleichen [Planlimits](/docs/de/errors#youve-hit-your-session-limit) wie Ihre anderen Claude Code-Sitzungen, und ein Projekt kann diese Limits nicht selbst überschreiten.

Ein Thread, der das Limit Ihres Plans erreicht, wartet und wird fortgesetzt, wenn das Limit zurückgesetzt wird, daher beginnt die Arbeit, die Sie laufen gelassen haben, Ihr nächstes Nutzungsfenster ohne eine Nachricht von Ihnen zu verwenden. [Ein Thread hat das Nutzungslimit erreicht](#usage-limit-reached) behandelt, was Sie sehen, wie Sie es stoppen, und den einen Fall, der nicht wartet.

Arbeit geht über Ihre Planlimits hinaus, nur wenn Sie [Nutzungsguthaben](/docs/de/costs#add-usage-credits-to-your-subscription) für Ihr Konto aktiviert haben. Ein Thread kann sie nicht für Sie aktivieren.

<h3 id="what-draws-on-your-plan">
  Was nutzt Ihren Plan
</h3>

Ein Projekt nutzt Ihre Limits schneller als eine einzelne Sitzung, und auf einem Pro-Plan sollten Sie besonders erwarten, Ihr Limit früher an Tagen zu erreichen, an denen Sie einen ausführen. Dies sind die Teile eines Projekts, die Ihren Plan nutzen:

* **Laufende Threads**: Jeder ist eine vollständige Sitzung, und mehrere können gleichzeitig laufen. Es gibt keine feste Anzahl; Claude startet so viele, wie die Arbeit erfordert, und ein Limit, das Sie [anfordern](#tune-how-claude-runs-a-project), ist eher eine Vorliebe als eine Obergrenze. Das erzwungene Limit sind 200 neue Threads pro Tag über Ihre Projekte.
* **Das Gespräch**: Claude nutzt Token, um zu lesen, was Threads berichten, und zu entscheiden, was als nächstes zu tun ist.
* **Threads, die einen Pull Request überwachen**: Ein untätiger Thread wacht auf und nutzt Ihren Plan erneut, wenn CI fehlschlägt oder ein Review-Kommentar auf seinem Pull Request ankommt. Um das zu stoppen, bitten Sie im Thread, den Pull Request nicht mehr zu überwachen.

Ein Projekt ohne laufende Threads, keine überwachten Pull Requests und keine neuen Nachrichten nutzt Ihren Plan nicht, während es untätig sitzt, und auch kein archiviertes Projekt.

<h3 id="see-and-reduce-a-project’s-usage">
  Sehen und reduzieren Sie die Nutzung eines Projekts
</h3>

Öffnen Sie **Nutzung** in **Projekteinstellungen**, um die Token-Nutzung nach Thread und nach Modell zu sehen und wie viel zum Projektgespräch ging. Um es zu reduzieren:

* Eine Nachverfolgung, die zu einem Thread geleitet wird, der länger als die [Cache-Lebensdauer](/docs/de/prompt-caching#cache-lifetime) untätig war, eine Stunde auf Pro und Max innerhalb Ihrer Planlimits, liest die gesamte Konversation dieses Threads erneut, bevor etwas getan wird. Für neue Arbeit kann das Bitten von Claude, einen frischen Thread zu starten, weniger nutzen als einen großen alten zu beleben.
* Für Arbeit, die nicht das größte Modell benötigt, [wählen Sie ein kleineres Modell oder eine niedrigere Aufwand-Stufe](#choose-models-and-let-claude-manage-context) für Threads, das Gespräch oder beide.
* Bitten Sie Claude im Projektgespräch, weniger Threads gleichzeitig auszuführen, oder kleine Fragen selbst zu beantworten, anstatt einen Thread zu starten.

<h2 id="how-projects-relate-to-other-claude-code-features">
  Wie Projekte sich auf andere Claude Code-Funktionen beziehen
</h2>

Mehrere Claude Code-Funktionen ermöglichen es, dass mehr als eine Sitzung gleichzeitig arbeitet, daher ist das parallele Ausführen von Arbeit nicht an sich, wofür ein Projekt ist. In einem Projekt startet und verfolgt Claude die Sitzungen statt Ihnen, und jede startet aus den gleichen Anweisungen. So verbindet sich jede benachbarte Funktion mit einem Projekt:

* **Claude Tag**: [Claude Tag](https://claude.com/docs/claude-tag/overview) ist Claude in den Slack-Kanälen Ihres Teams, auf Team- und Enterprise-Plänen. Jeder in einem Kanal kann ihm Arbeit geben, jeder im Kanal sieht und lenkt es, und es nutzt Verbindungen, die ein Admin für diesen Kanal eingerichtet hat. Ein Projekt ist nur Ihres: Sie sind der Einzige, der ihm Arbeit gibt oder seine Threads sieht, es nutzt Ihren eigenen GitHub-Zugriff und Connectors, und es ist auf Pro und Max. [Wie sich Claude Tag von Cowork und Claude Code unterscheidet](https://claude.com/docs/claude-tag/concepts/how-it-works#how-claude-tag-differs-from-cowork-and-claude-code) hat die Seite-an-Seite.
* **Cloud-Sitzungen**: Jeder Thread ist eine [Cloud-Sitzung](/docs/de/claude-code-on-the-web), es sei denn, Sie bitten Claude, ihn auf Ihrem Computer auszuführen. In beiden Fällen startet und verfolgt Claude ihn statt Ihnen. Eine Cloud-Sitzung, die Sie selbst gestartet haben, kann ein Projekt werden oder einen durch [**Als Projekt fortfahren** oder **In Projekt verschieben**](#start-from-an-existing-cloud-session) füttern.
* **Routinen**: Wenn Sie geplante Arbeit in einem Projekt anfordern, erstellt Claude eine [Routine](/docs/de/routines), die als Threads in diesem Projekt läuft und auf seiner Registerkarte **Routinen** angezeigt wird. Routinen, die Sie außerhalb eines Projekts erstellen, laufen weiter auf ihre eigene Weise.
* **Lokale Sitzungen und Agent-Ansicht**: Eine Sitzung, die Sie selbst in Ihrem Terminal, IDE oder der lokalen Umgebung der Desktop-App starten, kann nicht zu einem Projekt hinzugefügt werden. Ein Projekt erreicht Ihren Computer nur, indem es einen Thread dort durch [Remote Control](/docs/de/remote-control) ausführt. [Agent-Ansicht](/docs/de/agent-view) ist ein Bildschirm zum Verfolgen mehrerer lokaler Sitzungen, die Sie selbst gestartet haben; es hat keinen Koordinator.
* **Worktrees**: Ein [Worktree](/docs/de/worktrees) gibt jeder lokalen Sitzung ihre eigene Arbeitskopie eines Repositories, daher überschreiben sich parallele Sitzungen auf Ihrem Computer nicht gegenseitig. Cloud-Threads benötigen sie nicht: Jeder klont seine Repositories in seine eigene Cloud-Sandbox und arbeitet auf seinem eigenen Branch.
* **Agent-Teams**: Ein [Agent-Team](/docs/de/agent-teams) ist eine Sitzung, die Teammate-Sitzungen für eine einzelne Aufgabe startet, auf Ihrem Computer oder in einer Cloud-Sitzung, und endet mit dieser Aufgabe.
* **Projekte in claude.ai-Chat und Cowork**: Die [frühere Projekte-Erfahrung](https://support.claude.com/en/articles/9517075-what-are-projects), die Gespräche und Referenzdateien ohne Threads oder einen Koordinator gruppiert. Diese Projekte funktionieren weiterhin wie heute, bis die neu gestaltete Erfahrung sie erreicht.

[Agenten parallel ausführen](/docs/de/agents) vergleicht diese Optionen Seite an Seite.

<h2 id="limitations">
  Einschränkungen
</h2>

* Projekte sind unter claude.ai/code, in der Desktop-App und in der Claude Mobile-App verfügbar, nicht im Terminal CLI oder über Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry. Der Befehl [`claude project`](/docs/de/cli-reference) der CLI, der den lokalen Claude Code-Status für ein Verzeichnis verwaltet, ist nicht verwandt.
* Projekt-Threads sind [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) oder Sitzungen auf Ihrem eigenen Computer über [Remote Control](/docs/de/remote-control), wobei Anthropic in beiden Fällen der Modell-Provider ist. [Sicherheit](/docs/de/security) und [Datennutzung](/docs/de/data-usage) erläutern, wie Cloud-Sitzungen isoliert sind und was beibehalten wird, und [Verbindung und Sicherheit](/docs/de/remote-control#connection-and-security) erläutert, wie ein Thread auf Ihrem Computer verbunden ist und was gespeichert wird.
* Sie können eine Sitzung, die Sie selbst auf Ihrem Computer gestartet haben, nicht zu einem Projekt hinzufügen. Um einem Projekt zu ermöglichen, einen Thread auf Ihrem Computer auszuführen, verbinden Sie den Ordner, in dem es funktionieren soll, über [Remote Control](/docs/de/remote-control#requirements): Aktivieren Sie Remote Control unter **Einstellungen > Claude Code** in der Claude Desktop-App, oder führen Sie `claude remote-control` im Ordner aus und lassen Sie es laufen. Dieser Computer benötigt Claude Code v2.1.280 oder später. Ein Projekt kann auch keinen Thread auf Ihrem Computer ausführen, während **Vertrauenswürdige Geräte erforderlich** in Ihren claude.ai-Einstellungen aktiviert ist.
* Ein Cloud-Thread-Sandbox wird zwischen Turns unterbrochen und wird fortgesetzt, wenn der Thread weitergeht. Wenn die Sandbox nicht fortgesetzt werden kann, wird der Thread von einem frischen Klon aus fortgesetzt, sodass nicht committete Änderungen verloren gehen können. Bei langen Aufgaben bitten Sie Claude, Work in Progress zu committen und zu pushen.
* Ein Projekt gehört einem Benutzer. Sie können ein Projekt oder seine Threads nicht mit einem anderen Benutzer teilen, und Thread-Transkripte haben nicht die Share-Option, die andere Cloud-Sitzungen haben. Es gibt keine Kontrollen auf Organisationsebene für Projekte während der Beta.
* Ein Thread gehört dem einen Projekt, das ihn gestartet hat. Sie können einen Thread nicht zu einem anderen Projekt verschieben oder kopieren, oder ihn als eigenständig verschieben. [**In Projekt verschieben**](#start-from-an-existing-cloud-session) funktioniert nur in die andere Richtung: Es bringt die Arbeit einer Cloud-Sitzung in ein Projekt.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

Für die GitHub-Setup-Aufforderungen im Dialog **Neues Projekt** siehe [GitHub-Zugriff einrichten](#set-up-github-access).

<h3 id="a-thread-looks-stuck">
  Ein Thread sieht fest aus
</h3>

Claude veröffentlicht nicht jeden Schritt, den ein Thread unternimmt, daher arbeitet ein Thread, der als laufend mit keinen neuen Nachrichten im Projektgespräch angezeigt wird, normalerweise immer noch. Ein neuer Cloud-Thread stellt auch seine [Cloud-Umgebung](/docs/de/cloud-environments) bereit, bevor Claude beginnt, daher dauert sein erstes Update einen Moment. Öffnen Sie den Thread, um sein Transkript zu lesen. Wenn der Thread auf einer Berechtigungsaufforderung wartet, beantworten Sie sie dort.

<h3 id="threads-guessed-or-stalled-instead-of-asking">
  Threads haben geraten oder stecken fest, anstatt zu fragen
</h3>

Wenn mehrere Threads zurückkommen und etwas Falsches angenommen haben, fehlenden Zugriff umgangen haben oder mit „blockiert" gestoppt haben, ist die Ursache normalerweise die gleiche Lücke in der Einrichtung des Projekts statt eines Problems mit jeder Aufgabe. Sortieren Sie, welche Threads solide sind, bevor Sie etwas reparieren:

1. Bitten Sie Claude im Gespräch: „Für jeden offenen Thread, listen Sie auf, was Sie ihn zu tun aufgefordert haben, was er angenommen oder nicht erreichen konnte, und worauf es wartet." Claude liest jeden Thread und antwortet im Gespräch.
2. Für Threads, die von einer falschen Annahme gestartet wurden, öffnen Sie den Thread von **Übersicht** und markieren Sie ihn als gelöst aus seinem Menü, oder sagen Sie ihm, was stattdessen zu tun ist, in seinem Nachrichtenfeld. Sein Branch und jeder Pull Request bleiben auf GitHub, bis Sie sie löschen.
3. Reparieren Sie die Lücke einmal, in [Projektanweisungen](#give-a-project-standing-context) oder der [Umgebung](#choose-an-environment-for-threads), dann senden Sie einen Thread, bevor Sie den Rest der Arbeit erneut als neue Threads senden.

<h3 id="claude-hasnt-responded">
  Claude hat nicht geantwortet
</h3>

Das Projektgespräch zeigt ein Banner „Claude hat nicht geantwortet", wenn Claude läuft, aber seine Antworten das Projekt nicht erreichen. Klicken Sie auf **Claude neu starten** auf dem Banner, oder gehen Sie zu **Projekteinstellungen > Allgemein** und klicken Sie auf **Neu starten** in der Zeile **Claude neu starten**. Claude verbindet sich erneut mit dem Gespräch; jede Antwort, die es gerade schrieb, geht verloren, und Threads sind nicht betroffen.

<h3 id="repository-access-errors">
  Repository-Zugriffsfehler
</h3>

Drei Nachrichten bedeuten, dass ein Thread oder das Projekt eines seiner Repositories nicht erreichen kann. Ein Projekt-Cloud-Thread benötigt die [GitHub-Voraussetzungen](#check-the-prerequisites) auch wenn Ihre anderen Cloud-Sitzungen das gleiche Repository ohne Probleme klonen.

* **„Konnte die Sitzung nicht starten — Claude hat keinen GitHub-Zugriff auf das Repository dieses Projekts"**, berichtet, bevor der Thread startet, wenn die Claude GitHub App nicht auf diesem Repository installiert ist, ausgesetzt ist oder nicht mit dem GitHub-Konto verknüpft ist, das Sie verbunden haben.
* **„Kann Ihr Repository nicht erreichen"**, berichtet von einem Thread, wenn sein Klon fehlschlägt: GitHub lehnte den Klon ab, das Repository wurde nicht unter dem Namen gefunden, den das Projekt hat, oder der Branch, von dem der Thread starten sollte, existiert nicht.
* **„Claude kann nicht auf" ein Repository zugreifen**, angezeigt, wenn Sie Repositories im Dialog **Neues Projekt** oder **Projekteinstellungen** speichern. Die Nachricht wird mit einem Install-Link und einem Reconnect-Link fortgesetzt. Verwenden Sie den Install-Link, wenn die Claude GitHub App nicht auf diesem Repository ist, und den Reconnect-Link, wenn sie es ist, da die GitHub App auf GitHub installiert sein kann, ohne mit dem Konto verknüpft zu sein, das Sie mit Claude verbunden haben. Wenn die Nachricht sagt, dass die GitHub App ausgesetzt ist oder dieses Repository nicht enthält, folgen Sie ihrem Link zu GitHub, um das zu beheben.

Um eines von ihnen zu beheben, klicken Sie auf die Schaltfläche, die die Nachricht anbietet, wie z. B. **GitHub App installieren** oder **Repositories auf GitHub auswählen**, dann **Erneut überprüfen**. Wenn der Block auf der GitHub-Organisations-Seite ist, wie z. B. ein Besitzer, der die App nicht genehmigt hat, oder eine IP-Zulassungsliste, die Claude ausschließt, zeigt die Nachricht einen Link **Siehe wie zu beheben** statt. Wenn es keine Schaltfläche gibt, folgen Sie [GitHub-Zugriff einrichten](#set-up-github-access), dann senden Sie eine weitere Nachricht zum Wiederholen.

<h3 id="usage-limit-reached">
  Ein Thread hat das Nutzungslimit erreicht
</h3>

Wenn ein Thread oder das Projektgespräch Ihr Plan-Limit von fünf Stunden oder wöchentlich erreicht, versucht es weiterhin selbst und wird fortgesetzt, wenn das Limit zurückgesetzt wird. Während es wartet, zeigt der Thread **Service ist beschäftigt** mit „Claude versucht immer noch und wird automatisch fortgesetzt." Sie müssen nichts tun, damit die Arbeit fortgesetzt wird. Wenn Sie lieber möchten, dass es Ihr nächstes Nutzungsfenster nicht nutzt, klicken Sie auf **Stopp** im Thread, oder [pausieren Sie das Projekt](#pause-archive-or-delete-a-project), um jeden Thread zu halten. Ein Thread, den eine Routine gestartet hat, wartet nicht: Sein Zug stoppt mit einem Limit-Fehler, und Sie senden ihm nach dem Limit-Reset eine Nachricht.

[Nutzungslimit-Fehler](/docs/de/errors#youve-hit-your-session-limit) erklären die Limits und wann sie zurückgesetzt werden.

<h3 id="additional-usage-credits-are-required">
  Zusätzliche Nutzungsguthaben sind erforderlich
</h3>

Ein Thread oder das Projektgespräch machte eine Anfrage, die Ihr Plan nur mit Nutzungsguthaben abdeckt, wie z. B. eine zu einem Modell oder einer Kontextgröße, die Ihr Plan nicht enthält, und Nutzungsguthaben sind nicht für Ihr Konto aktiviert. [Fügen Sie Nutzungsguthaben zu Ihrem Abonnement hinzu](/docs/de/costs#add-usage-credits-to-your-subscription) behandelt, wer sie auf jedem Plan aktivieren oder kaufen kann. Sobald Guthaben verfügbar sind, senden Sie eine weitere Nachricht zum Wiederholen.

<h3 id="context-limit">
  Andere Nachrichten
</h3>

Diese Nachrichten benennen ihre eigene Ursache. Die Tabelle gibt den nächsten Schritt für jeden.

| Nachricht                                                                                                       | Was zu tun ist                                                                                                                                                                                                                                                               |
| :-------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| „Kann nicht mit Repository verbinden" mit „Claude konnte nicht zu GitHub gehen, um Ihr Repository zu holen"     | Warten Sie einen Moment, dann senden Sie eine weitere Nachricht zum Wiederholen                                                                                                                                                                                              |
| „Kann nicht mit Repository verbinden" mit „Claude konnte nicht auf Ihr Repository oder Ihre Umgebung zugreifen" | Ihr GitHub-Konto benötigt Push-Zugriff auf das Repository, und die Umgebung muss immer noch existieren. Überprüfen Sie beide in **Projekteinstellungen > Umgebung**, dann wiederholen Sie                                                                                    |
| „Konnte die Setup-Vorschlag nicht anzeigen"                                                                     | Die App, die Sie offen haben, ist älter als die **Setup-Empfehlungen**, die Claude gesendet hat. Aktualisieren Sie die Seite oder starten Sie die Desktop-App neu, oder bitten Sie Claude, den Setup erneut vorzuschlagen                                                    |
| „Die Umgebung des Projekts wurde entfernt"                                                                      | Wählen Sie eine andere Umgebung in **Projekteinstellungen > Umgebung**; die Änderung gilt für neue Threads                                                                                                                                                                   |
| „Setup-Skript fehlgeschlagen"                                                                                   | Klicken Sie auf **Setup-Skript bearbeiten** auf dem Fehler, reparieren Sie das Skript in der Umgebung, dann senden Sie eine weitere Nachricht. [Setup-Skript fehlgeschlagen](/docs/de/web-quickstart#setup-script-failed) listet häufige Ursachen auf                             |
| „Claude ist auf diesem Zug aus dem Kontext gelaufen"                                                            | Der Thread hat sein Kontextfenster gefüllt. Wenn die Nachricht sagt, dass der Thread in einer frischen Sitzung fortgesetzt wird, wird er von selbst fortgesetzt; andernfalls bitten Sie Claude im Projektgespräch, einen neuen Thread für die verbleibende Arbeit zu starten |
| „Erreichte das Zug-Limit"                                                                                       | Der Thread erreichte die Obergrenze für agentic Turns, die [`CLAUDE_CODE_MAX_TURNS`](/docs/de/env-vars) setzt. Senden Sie eine weitere Nachricht zum Fortfahren, oder erhöhen oder entfernen Sie diese Variable, wo sie gesetzt ist                                               |

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Verwenden Sie Claude Code in der Cloud](/docs/de/claude-code-on-the-web): Wie die Cloud-Sitzungen hinter jedem Thread funktionieren, einschließlich GitHub-Zugriffsoptionen und Auto-Fix auf Pull Requests
* [Konfigurieren Sie Cloud-Umgebungen](/docs/de/cloud-environments): Ändern Sie, was Threads im Netzwerk erreichen können, geben Sie ihnen Umgebungsvariablen und API-Anmeldedaten, und installieren Sie Tools mit einem Setup-Skript
* [Automatisieren Sie Arbeit mit Routinen](/docs/de/routines): Zeitpläne, Trigger und Management für Routinen, einschließlich der von Claude aus einem Projekt erstellten
* [Verwalten Sie mehrere Agenten mit Agent-Ansicht](/docs/de/agent-view): Führen Sie mehrere Sitzungen auf Ihrem eigenen Computer aus und verfolgen Sie sie, wenn die Arbeit Tools oder Services benötigt, die nur Ihr Computer erreichen kann
* [Projekte neu gestaltet: von Ordner zu Gespräch](https://claude.com/blog/projects-redesigned): Die Launch-Ankündigung mit dem Denken hinter der Umwandlung eines Projekts in ein Gespräch mit Claude
