> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Nachrichten an Ihre anderen Claude Code-Sitzungen

> Lassen Sie Claude Ihre anderen Claude Code-Sitzungen auf diesem Computer auflisten und anschreiben, und erreichen Sie Ihre Sitzungen auf anderen Computern oder im Web.

<Note>
  Sitzungsübergreifendes Messaging erfordert Claude Code v2.1.224 oder später auf macOS und Linux, einschließlich Linux in WSL 2. Unter nativem Windows ist Claude Code v2.1.234 oder später erforderlich. Wenn eine Sitzung die Anforderungen erfüllt, ist Messaging aktiviert und es ist nichts zu aktivieren. Siehe [Verfügbarkeit](#availability) für Anbieteranforderungen und wie Sie bestätigen, dass eine Sitzung dies hat.
</Note>

Sitzungsübergreifendes Messaging ermöglicht es Claude, eine Nachricht von einer Ihrer Claude Code-Sitzungen an eine andere zu übermitteln. Wenn eine Änderung in einer Sitzung das bricht, woran eine andere arbeitet, kann Claude diese Sitzung warnen, bevor Sie es bemerken. Wenn eine Sitzung eine Frage beantwortet, bei der eine andere blockiert ist, kann Claude die Antwort übermitteln.

Eine Nachricht ist ein Textabschnitt, den ein Claude an einen anderen schreibt, niemals die Gesprächsverlauf oder Dateien des Absenders. Um ein ganzes Gespräch oder seinen Kontext zu verschieben, [setzen Sie die Sitzung statt dessen fort](/docs/de/sessions#resume-a-session).

Claude verwendet zwei Tools für dies: `ListAgents` um zu entdecken, welche Agenten es erreichen kann, und `SendMessage` um eine Nachricht an einen von ihnen nach Name zu übermitteln. Mit demselben `SendMessage` Tool kann Claude auch [Subagenten](/docs/de/sub-agents#resume-subagents) und [Agent-Team](/docs/de/agent-teams) Teamkollegen innerhalb einer einzelnen Sitzung oder eines Teams anschreiben. Diese Seite behandelt Nachrichten zwischen Ihren unabhängigen Sitzungen.

<h2 id="when-to-use-cross-session-messaging">
  Wann Cross-Session-Messaging verwendet werden sollte
</h2>

Verwenden Sie Messaging, wenn eine Ihrer Sitzungen etwas hat, das eine andere Sitzung während einer Aufgabe benötigt. Claude kann eine Nachricht von selbst senden, wenn es den Bedarf sieht, zum Beispiel nach einer Änderung, die sich auf die Arbeit einer anderen Sitzung auswirkt, oder Sie können es auffordern, eine zu senden. Die häufigen Fälle:

* **Ein Ergebnis übergeben**: Wenn eine Sitzung eine Breaking Change entdeckt oder eine Entscheidung trifft, fasst Claude sie für die Sitzung zusammen, die an dem betroffenen Bereich arbeitet, anstatt dass Sie sie dort erneut erklären.
* **Parallele Worktrees koordinieren**: Wenn Sitzungen dasselbe Repository in separaten [Worktrees](/docs/de/worktrees) bearbeiten, kann Claude die anderen Sitzungen darüber informieren, was eingecheckt wurde.
* **Status von langfristiger Arbeit abrufen**: Lassen Sie eine Migration oder einen Test-Lauf an die Sitzung berichten, die Sie beobachten, oder fragen Sie selbst danach. Wenn diese Sitzung auf diesem Computer ist, kann Claude auch [sie fragen, um eine Benachrichtigung zu erhalten, wenn sie das nächste Mal untätig wird oder beendet wird](#get-a-notice-when-another-session-goes-idle).
* **Nachrichten über Computer hinweg**: Erreichen Sie eine Ihrer Sitzungen auf einem anderen Computer oder im Web.

Verwenden Sie Messaging zwischen unabhängigen Sitzungen, die Sie selbst starten und steuern. Claude Code hat eine dedizierte Funktion für jede der anderen Möglichkeiten, mehrere Sitzungen auszuführen oder zu erreichen, verwenden Sie also die für das, was Sie tun, gebaute:

* Um ein Gespräch in einem anderen Terminal fortzusetzen oder seinen Kontext mit einer neuen Sitzung zu teilen, [setzen Sie die Sitzung fort](/docs/de/sessions#resume-a-session)
* Für ein koordiniertes Team von Sitzungen, die Claude spawnt und beaufsichtigt, verwenden Sie [Agent-Teams](/docs/de/agent-teams)
* Um viele Sitzungen von einem Ort aus zu beobachten und zu steuern, verwenden Sie [Agent-Ansicht](/docs/de/agent-view)
* Um eine Sitzung selbst von Ihrem Telefon oder einem anderen Gerät aus zu steuern, anstatt dass Sitzungen sich gegenseitig benachrichtigen, verwenden Sie [Remote Control](/docs/de/remote-control)
* Um externe Ereignisse wie CI-Ergebnisse oder Chat-Nachrichten in eine Sitzung zu pushen, verwenden Sie [Kanäle](/docs/de/channels)

<h2 id="message-another-session">
  Eine andere Sitzung benachrichtigen
</h2>

Wenn eine Ihrer Sitzungen etwas lernt, das eine andere Sitzung benötigt, wie eine Erkenntnis, einen Status oder eine Entscheidung, leitet Claude es weiter, anstatt dass Sie zwischen Terminals kopieren und einfügen. Claude findet das Ziel mit `ListAgents` und sendet mit `SendMessage`, sodass Sie diese Tools nie selbst aufrufen. Claude kann entscheiden, eine Nachricht zu senden, ohne gefragt zu werden, und Sie können auch eine anfordern.

Um selbst eine anzufordern, teilen Sie Claude mit, was die andere Sitzung wissen oder tun soll. Dieses Beispiel ist eine Eingabeaufforderung, die Sie eingeben, nicht eine Nachricht, die Claude sendet:

```text wrap theme={null}
Fragen Sie die Sitzung, die in meinem anderen Terminal läuft, ob die Migration abgeschlossen ist
```

Claude schreibt die eigentliche Nachricht selbst, sodass Ihre Eingabeaufforderung den Inhalt Claude überlassen kann. Diese Eingabeaufforderung fordert eine Zusammenfassung an, ohne ihre Formulierung vorzuschreiben, und was Claude sendet, variiert:

```text wrap theme={null}
Erklären Sie der Sitzung, die an der Payments-API arbeitet, was wir gerade getan haben
```

Um das Ziel selbst zu benennen, erwähnen Sie die Sitzung in Ihrer Eingabeaufforderung: Geben Sie `@` gefolgt von den ersten Buchstaben des Sitzungsnamens ein und wählen Sie die Sitzung aus der Typeahead-Liste aus, genauso wie Sie [einen Subagenten @-erwähnen](/docs/de/sub-agents#invoke-subagents-explicitly). Erfordert Claude Code v2.1.232 oder später. Claude Code fügt die Erwähnung ein, z. B. `@api-worker`, und teilt Claude mit, welche Sitzung sie benennt, sodass Claude diese Sitzung benachrichtigen kann, ohne Ihre Sitzungen zuerst aufzulisten. Diese Eingabeaufforderung benennt das Ziel mit einer Erwähnung:

```text wrap theme={null}
Lassen Sie @api-worker wissen, dass die Schemamigration abgeschlossen ist
```

Die Typeahead-Liste zeigt Ihre anderen aktiven Sitzungen auf diesem Computer. Zwei Fälle erfordern mehr als die ersten Buchstaben eines Namens:

* **Eine Sitzung außerhalb dieses Computers**: Eine Cloud- oder Remote-Control-Sitzung wird in der Typeahead-Liste nur angezeigt, nachdem Claude Ihre Sitzungen außerhalb dieses Computers aufgelistet oder benachrichtigt hat. Bitten Sie Claude daher, diese zuerst aufzulisten.
* **Ein Name mit einem Leerzeichen oder anderen Zeichen außerhalb von Buchstaben, Ziffern, Bindestrichen und Unterstrichen**: Geben Sie ihn in doppelten Anführungszeichen ein, z. B. `@"release notes"`. Wenn Sie die Sitzung aus der Typeahead-Liste auswählen, fügt Claude Code die Anführungszeichen für Sie ein.

Sie können die Erwähnung auch ohne die Auswahl eingeben. Wenn mehr als eine aktive Sitzung auf den erwähnten Namen antwortet, fragt Claude Sie, welche Sie meinen, bevor die Nachricht gesendet wird.

Informationen darüber, wie die Nachricht aussieht, die Claude schreibt, wenn sie ankommt, einschließlich eines Beispiels, finden Sie unter [wie eine Nachricht aussieht](#what-a-message-looks-like).

<h3 id="message-delivery">
  Nachrichtenübermittlung
</h3>

Die empfangende Claude liest die Nachricht zwischen Werkzeugaufrufen während eines aktiven Zugs, sodass ein laufendes Werkzeug nie unterbrochen wird. Wenn die empfangende Sitzung untätig ist, startet Claude Code einen neuen Zug mit der Nachricht.

Eine Nachricht von einer anderen Sitzung kommt als Klartext an. Wenn sie eine Datei oder eine [MCP-Ressource](/docs/de/mcp#use-mcp-resources) mit `@` erwähnt, sieht Claude die Erwähnung wie geschrieben und Claude Code fügt nichts an, unabhängig davon, ob die Nachricht einen neuen Zug startet oder während eines ankommt. Claude kann einen erwähnten Pfad auf der empfangenden Maschine immer noch mit seinen eigenen Werkzeugen öffnen, vorbehaltlich der Berechtigungen dieser Sitzung. Vor v2.1.251 hat eine `@`-Erwähnung in einer Nachricht, die einen neuen Zug startete, die Datei oder MCP-Ressource auf der Empfängerseite angehängt.

Claude Code lehnt eine Nachricht in den folgenden Fällen ab:

* Die Nachricht überschreitet die [Größenbeschränkung](#limitations). Claude Code lehnt sie in der sendenden Sitzung ab, bevor sie versendet wird.
* Ein schneller Nachrichtenstoß zu einer Sitzung auf diesem Computer hat erreicht, was [diese Sitzung akzeptiert](#limitations). Claude Code lehnt weitere Nachrichten an diese Sitzung ab.
* Das Antwortziel auf diesem Computer besteht einen Sicherheitscheck nicht, z. B. ein symbolisch verknüpftes Ziel oder ein Endpunkt, der nicht der erwartete Prozess ist. [Ablehnung zum Senden einer sitzungsübergreifenden Nachricht](/docs/de/errors#refusing-to-send-a-cross-session-message) listet diese Checks auf.
* Claude adressiert die Nachricht an den Namen dieser Sitzung selbst, wie unter [Sehen Sie, welche Sitzungen Claude erreichen kann](#see-which-sessions-claude-can-reach) beschrieben.

Die empfangende Sitzung überprüft jede ankommende Nachricht gegen ihre eigenen [Eingangskontrollen](#control-inbound-messages), und die Überprüfung endet in einem von drei Ergebnissen:

* **Zugestellt**: Claude Code übergibt die Nachricht an die empfangende Claude.
* **Gehalten**: Claude Code legt die Nachricht unzugestellt beiseite. Eine gehaltene Nachricht erreicht Claude nur, wenn Sie sie genehmigen oder eine spätere Änderung des Modus oder der Einstellungen dies zulässt.
* **Abgelehnt**: Claude Code verwirft die Nachricht, ohne sie zuzustellen.

Nach der Zustellung zählt die Nachricht zur [Nutzung](/docs/de/costs) wie eine Eingabeaufforderung, die Sie eingeben, und die empfangende Claude kann auf die gleiche Weise antworten, außer im [einseitigen sitzungsübergreifenden Fall](#message-sessions-on-other-machines).

Berechtigungsgrenzen bleiben pro Sitzung. Claude wird angewiesen, eine andere Sitzung nie um eine Aktion zu bitten, die in ihrer eigenen Sitzung verweigert oder blockiert wurde, oder die ihre eigenen Berechtigungseinstellungen blockieren würden, und diese Arbeit stattdessen an Sie zurückzuleiten. Auf der Empfängerseite gelten die [Berechtigungsaufforderungen und Regeln der empfangenden Sitzung selbst](#how-a-session-treats-an-incoming-message) immer noch für alles, das die Nachricht anfordert.

<h3 id="get-a-notice-when-another-session-goes-idle">
  Erhalten Sie eine Benachrichtigung, wenn eine andere Sitzung untätig wird
</h3>

Claude kann eine Ihrer Sitzungen auf diesem Computer bitten, eine Benachrichtigung zu senden, wenn diese Sitzung das nächste Mal untätig wird oder beendet wird. Untätig bedeutet hier, dass die Sitzung einen Zug mit nichts in der Warteschlange beendet hat. Verwenden Sie dies, wenn Sie auf eine lange Aufgabe in einer anderen Sitzung warten und hören möchten, wenn sie erledigt ist, anstatt zu überprüfen. Erfordert Claude Code v2.1.236 oder später in beiden Sitzungen.

<h4 id="ask-for-a-notice">
  Fordern Sie eine Benachrichtigung an
</h4>

Teilen Sie Claude mit, worauf Sie warten. Diese Eingabeaufforderung fordert eine Benachrichtigung von der Migrationssitzung an:

```text wrap theme={null}
Sagen Sie mir, wenn die Migrationssitzung mit ihrer Arbeit fertig ist
```

Claude abonniert mit dem `SendMessage`-Werkzeug-Input `notify_when_idle`, entweder an eine Nachricht angehängt, die es ohnehin sendet, oder allein. Allein abonniert Claude Code, ohne einen Zug in der beobachteten Sitzung zu starten oder Token auszugeben, und sendet die Benachrichtigung sofort, wenn diese Sitzung bereits untätig ist. An eine Nachricht angehängt, liefert Claude Code die Nachricht zuerst und sendet die Benachrichtigung später.

<h4 id="what-each-session-shows">
  Was jede Sitzung anzeigt
</h4>

Die beobachtete Sitzung zeigt eine Zeile an, die besagt, dass ein anderer Prozess gebeten hat, benachrichtigt zu werden, wenn die Sitzung das nächste Mal untätig wird. Die anfragende Sitzung zeigt die Benachrichtigung als eine Zeile an, die die beobachtete Sitzung benennt. Die Zeile kann die Zeit enthalten, zu der der Zug dieser Sitzung beendet wurde, und einen einzeiligen Status aus diesem Zug. Wenn die anfragende Sitzung untätig ist, startet Claude Code einen neuen Zug mit der Benachrichtigung.

<h4 id="limits">
  Limits
</h4>

Die Benachrichtigung ist einmalig: Claude Code sendet sie einmal von der beobachteten Sitzung, und keine der beiden Sitzungen fragt die andere ab. Wenn innerhalb von 12 Stunden keine Benachrichtigung ankommt, verwirft Claude Code das Abonnement und teilt Claude dies mit, sodass es nicht weiter wartet.

Die [Eingangskontrollen](#control-inbound-messages) jeder Seite gelten für eine Benachrichtigung wie eine Nachricht:

* **`refuse` auf einer Seite**: nichts kommt an. Die beobachtete Sitzung verwirft die Anfrage, ohne sie aufzuzeichnen oder zu beantworten, sodass das Abonnement nach 12 Stunden unantwortlich abläuft, und eine anfragende Sitzung mit `refuse` abonniert nie.
* **`hold` auf einer Seite**: die Benachrichtigung kommt mit weniger an. Die beobachtete Sitzung lässt den einzeiligen Status weg, und die anfragende Sitzung zeigt die Benachrichtigung in Ihrem Transkript an, ohne sie an Claude zu liefern.

Nur die Claude in Ihrer Hauptkonversation kann abonnieren, und nur zu Ihren Sitzungen auf diesem Computer. Wenn ein Subagent oder ein Agent-Team-Kollege `notify_when_idle` setzt, macht Claude Code kein Abonnement und teilt ihm dies mit. Wenn Claude eine Benachrichtigung von einem anderen Agenten anfordert, z. B. einem Kollegen, einem Subagenten oder einer Sitzung außerhalb dieses Computers, lehnt Claude Code den gesamten Aufruf ab, einschließlich jeder daran angehängten Nachricht, und meldet die Ablehnung Claude, damit es die Nachricht ohne die Anfrage erneut senden kann.

<h3 id="see-which-sessions-claude-can-reach">
  Sehen Sie, welche Sitzungen Claude erreichen kann
</h3>

Claude findet das Ziel einer Nachricht selbst, sodass Sie nichts ausführen müssen, bevor Sie es auffordern zu senden. Um selbst zu sehen, welche Sitzungen Claude erreichen kann, führen Sie den Befehl `/list-agents` aus. Die erste Zeile ist, wenn vorhanden, der Name dieser Sitzung selbst, den Ihre anderen Sitzungen verwenden, um sie zu benachrichtigen. Die Zeilen darunter sind die Sitzungen, die Claude erreichen kann:

* **Subagenten**: Agenten, die in der aktuellen Sitzung ausgeführt werden.
* **Kollegen**: die [Agent-Team](/docs/de/agent-teams)-Kollegen dieser Sitzung. Vor v2.1.239 erschienen Kollegen nicht in der Auflistung, obwohl Claude sie bereits nach Name benachrichtigen konnte.
* **Ihre anderen lokalen Sitzungen**: Claude-Code-Sitzungen, die auf demselben Computer ausgeführt werden, einschließlich [Hintergrundsitzungen](/docs/de/agent-view). Eine Sitzung wird nur angezeigt, wenn sie einen [Inbox-Socket](#the-sessions-inbox-socket) bindet.
* **Ihre Cloud-Sitzungen**: Ihre [Claude Code im Web](/docs/de/claude-code-on-the-web)-Sitzungen, angezeigt, während diese Sitzung mit [Remote Control](/docs/de/remote-control) verbunden ist. Claude Code kennzeichnet sie in der Auflistung als `cloud`.
* **Ihre Remote-Control-Sitzungen auf anderen Computern**: angezeigt, während diese Sitzung mit [Remote Control](/docs/de/remote-control) verbunden ist, und gekennzeichnet als `Remote Control`. Claude Code zeigt `offline` als Status einer Sitzung an, deren Remote-Control-Verbindung unterbrochen wurde.

Diese Sitzung ist keine der Zeilen. Wenn Claude eine Nachricht an den Namen dieser Sitzung selbst adressiert, lehnt Claude Code sie ab und teilt Claude mit, dass das Ziel die aktuelle Sitzung ist. Vor v2.1.239 zeigte die Auflistung nicht den Namen dieser Sitzung an, und Claude Code meldete eine an sie gesendete Nachricht als einen Agenten, den es nicht finden konnte.

Während diese Sitzung mit [Remote Control](/docs/de/remote-control) verbunden ist, behält Claude Code einige Details Ihrer lokalen Sitzungen aus der `/list-agents`-Ausgabe zurück, ohne zu ändern, was Claude selbst sieht, wenn es nach einer Sitzung zum Benachrichtigen sucht:

* **Arbeitsverzeichnisse**: Es lässt das Arbeitsverzeichnis jeder lokalen Sitzung weg.
* **Sitzungsnamen**: Es lässt jeden Sitzungsnamen weg, den es nicht einer Person zuordnen kann, sodass eine Zeile ohne Namen `(unnamed session)` lautet.
* **Die erste Zeile**: Es lässt die Zeile mit dem Namen dieser Sitzung weg, es sei denn, Sie haben diesen Namen an diesem Terminal eingegeben, mit `--name` oder mit `/rename` und dem Namen, seit Sie die Sitzung gestartet oder zuletzt fortgesetzt haben.

Wenn die Ausgabe etwas auflistet, endet sie mit einer Notiz, die besagt, dass Details zurückgehalten wurden. Das Ausführen von `/rename` gefolgt von einem ungenutzten Namen an der eigenen Tastatur einer Sitzung gibt dieser Sitzung einen Namen, der in der Ausgabe angezeigt wird.

Claude Code liest Ihre Cloud- und Remote-Control-Sitzungslisten von neuesten zuerst und stoppt nach einer begrenzten Anzahl von Seiten für jede. Wenn Ihr Konto mehr dieser Sitzungen hat, als passen, listet Claude Code die älteren nicht auf, und Claude kann sie nicht nach Name benachrichtigen. Wenn dies geschieht, teilt Claude Code dies in der Auflistung mit, und Claude sieht die gleiche Notiz, wenn es eine Nachricht sendet.

Claude adressiert eine Sitzung außerhalb dieses Computers nach Name, genauso wie eine lokale Sitzung. Siehe [Benachrichtigung von Sitzungen auf anderen Computern](#message-sessions-on-other-machines) für die Reise dieser Nachrichten.

Eine Sitzung antwortet auf den Namen, den Sie mit dem Befehl [`/rename`](/docs/de/commands) oder dem Flag [`--name`](/docs/de/cli-reference#cli-flags) setzen. Wenn Sie keinen setzen, benennt Claude Code die Sitzung selbst. Für eine interaktive Sitzung ist dies der Name, der in [Auflistungen laufender Sitzungen](/docs/de/sessions#name-your-sessions) angezeigt wird.

Wenn Sie eine Sitzung umbenennen, aktualisiert Claude Code auch den gemeinsamen Datensatz, den Ihre anderen Sitzungen verwenden, um den Namen der Sitzung nachzuschlagen. Wenn es diesen Datensatz nicht aktualisieren kann, warnt es Sie in der `/rename`-Ausgabe, dass andere Sitzungen möglicherweise immer noch den alten Namen anzeigen. Führen Sie die Sitzung mit [`--debug`](/docs/de/cli-reference#cli-flags) aus, und Claude Code protokolliert die Ursache des fehlgeschlagenen Updates.

Wenn Sie eine Sitzung umbenennen oder eine interaktive mit einem Namen starten oder fortsetzen, den eine andere aktive Sitzung auf diesem Computer bereits verwendet, behält Claude Code den Namen bei der Sitzung, die ihn bereits hat, und [benennt Ihren in eine Variante um](/docs/de/sessions#name-your-sessions). Sitzungen können immer noch einen Namen teilen, z. B. wenn eine von ihnen eine frühere Version von Claude Code ausführt oder der gemeinsame Name einer ist, die Claude Code generiert hat. Es sei denn, diese Sitzung ist mit Remote Control verbunden. Claude Code zeigt das Arbeitsverzeichnis jeder lokalen Sitzung in der `/list-agents`-Ausgabe an, sodass Sie gleichnamige Sitzungen unterscheiden können, wenn sie in verschiedenen Verzeichnissen ausgeführt werden. Claude adressiert die Nachricht auf eine von zwei Arten, je nachdem, wie viele aktive Sitzungen auf den Namen antworten:

* **Eine Sitzung antwortet auf den Namen**: Claude Code liefert die Nachricht nur auf dem Namen.
* **Mehrere Sitzungen teilen den Namen, oder Claude Code konnte nicht überall überprüfen, wo Ihre Sitzungen ausgeführt werden**: Claude fügt jeder Zeile seiner Auflistung einen kurzen Bezeichner hinzu und verwendet den Bezeichner in der Adresse.

<h3 id="message-sessions-on-other-machines">
  Benachrichtigung von Sitzungen auf anderen Computern
</h3>

Wie eine Nachricht reist und ob sie Anthropic-Server durchläuft, hängt davon ab, wo die Zielsitzung ausgeführt wird:

| Wo die andere Sitzung ausgeführt wird                | Wie die Nachricht reist                                                                                                             |
| :--------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------- |
| Auf diesem Computer                                  | Über einen Pro-Sitzungs-Socket auf macOS und Linux oder ein Pro-Sitzungs-Named-Pipe auf nativem Windows, nie durch Anthropic-Server |
| Auf einem anderen Ihrer Computer                     | Durch Anthropic-Server, ankommend über die [Remote Control](/docs/de/remote-control)-Verbindung dieses Computers                         |
| Auf [Claude Code im Web](/docs/de/claude-code-on-the-web) | Durch Anthropic-Server, direkt zur Cloud-Sitzung                                                                                    |

Das Starten einer Konversation mit einer Sitzung auf einem anderen Ihrer Computer erfordert Claude Code v2.1.225 oder später und ein Ziel, das [in der Auflistung angezeigt wird](#see-which-sessions-claude-can-reach). Vor v2.1.225 konnte Claude nur auf eine Nachricht antworten, die von einer ankam.

Sie können eine Sitzung benachrichtigen, die als `offline` in [der Auflistung](#see-which-sessions-claude-can-reach) angezeigt wird, eine, deren Remote-Control-Verbindung unterbrochen wurde. Der Versand geht durch, aber die Nachricht kommt nur an, nachdem die Maschine dieser Sitzung sich erneut verbunden hat. Claude wird darüber informiert, wenn es sendet.

Same-Machine-Lieferung funktioniert überall dort, wo die Funktion aktiviert ist. Jede Sitzung registriert sich in Dateien auf der Festplatte. Wenn Claude Ihre lokalen Sitzungen auflistet oder benachrichtigt, liest Claude Code diese Dateien, um die Sitzungen zu finden, sodass zwei Sitzungen sich nur erreichen können, wenn sie die gleichen Dateien sehen können.

Ein Container hat sein eigenes Dateisystem, sodass eine Sitzung darin und eine Sitzung auf dem Host sich nicht erreichen können. Zwei Sitzungen im gleichen Container können sich immer noch gegenseitig benachrichtigen, einschließlich auf einem [selbstgehosteten Runner](/docs/de/self-hosted-environments). Eine Sitzung in WSL 2 und eine native Windows-Sitzung auf demselben Computer können sich auch nicht erreichen, da sie sich unter verschiedenen Home-Verzeichnissen registrieren und auf verschiedene Socket-Typen abhören.

Während diese Sitzung mit Remote Control verbunden ist, zeigt Claude Code die Nachricht in der Konversation dieser Sitzung unter dem Remote-Control-Namen dieser Sitzung an, wenn Sie eine Sitzung auf einem anderen Ihrer Computer benachrichtigen. Die Claude auf diesem Computer kann auf diesen Namen antworten. Wenn diese Sitzung beispielsweise mit Remote Control als `laptop-graceful-unicorn` verbunden ist und Sie Ihren Desktop benachrichtigen, sehen Sie die Nachricht in der Desktop-Sitzung unter `laptop-graceful-unicorn`.

Wenn diese Sitzung nicht mit Remote Control verbunden ist, wenn Claude an eine Sitzung außerhalb dieses Computers sendet, geht die Nachricht immer noch durch, aber ohne eine [Antwortwort](#what-a-message-looks-like), sodass die empfangende Claude nicht antworten kann. Claude wird darüber informiert, wenn es sendet.

Um Ihre Genehmigung zu verlangen, bevor eine Nachricht außerhalb dieses Computers geht, setzen Sie [`isolatePeerMachines`](#require-approval-for-cross-machine-messages).

<h2 id="how-a-session-treats-an-incoming-message">
  Wie eine Sitzung eine ankommende Nachricht behandelt
</h2>

Wenn Sitzung A Sitzung B benachrichtigt, teilt Claude Code B's Claude mit, dass die Nachricht von einer anderen Sitzung kam, nicht von Ihnen, und begrenzt, was die Nachricht tun kann:

* **Es kann nichts genehmigen**: Eine Nachricht von einer anderen Sitzung zählt niemals als Ihre Zustimmung, sodass sie nicht auf eine ausstehende Berechtigungsaufforderung in Ihrem Namen antworten kann.
* **Es kann die Konfiguration nicht ändern**: Claude Code weist den empfangenden Claude an, niemals Berechtigungseinstellungen, `CLAUDE.md` oder andere Konfiguration zu ändern, weil eine andere Sitzung es fragte.
* **Befehle werden nicht ausgeführt**: Ein Befehl im Text der Nachricht, wie `/compact`, kommt als Klartext an. Claude Code führt ihn niemals aus.
* **Berechtigungsaufforderungen werden immer noch ausgelöst**: Wenn das Handeln auf die Nachricht eine Berechtigung erfordert, die die empfangende Sitzung nicht hat, sehen Sie die gleiche Aufforderung, die Sie für jede andere Arbeit sehen würden.

<h3 id="what-a-message-looks-like">
  Wie eine Nachricht aussieht
</h3>

Wenn eine Nachricht ankommt, zeigt Claude Code sie im Gespräch als eine schwache einzeilige Vorschau an, und die Vorschauzeile bleibt danach im Gespräch. Die Vorschau trägt den Namen des Absenders und die erste Zeile der Nachricht, abgeschnitten mit `…`, wenn sie lang ist, wie `› Message from @api-worker: Schema migration finished (ctrl+o to expand)`. Vor v2.1.247 zeigte Claude Code die ankommende Nachricht vollständig anstelle einer Vorschau an.

Jede dieser zeigt Ihnen den vollständigen Text:

* Drücken Sie `Ctrl+O`, um den [Transkript-Viewer](/docs/de/interactive-mode#transcript-viewer) zu öffnen und den vollständigen Text unter dem Namen der Sitzung des Absenders zu lesen.
* In einer Sitzung, die mit [`--verbose`](/docs/de/cli-reference#cli-flags) gestartet wurde, zeigt Claude Code den vollständigen Text anstelle der Vorschau an.

Die Vorschau verkürzt nur das, was Sie sehen. Ob Sie es erweitern oder nicht, Claude liest die vollständige Nachricht.

Claude empfängt die Nachricht mit dem Namen des Absenders und einer Antwortwort, außer für eine [einseitige Cross-Computer-Nachricht](#message-sessions-on-other-machines), die keine Antwortwort trägt. Über den Namen und die Antwortwort hinaus erhält der empfangende Claude den Text der Nachricht, niemals die Gesprächshistorie oder Dateien des Absenders. [Nachrichtenübermittlung](#message-delivery) behandelt `@`-Erwähnungen im Text.

Eine Nachricht, die ein [Subagent](/docs/de/sub-agents) schrieb, kommt unter dem Namen der sendenden Sitzung an, mit dem Subagenten im Nachrichtentext identifiziert. Eine Antwort darauf erreicht das Hauptgespräch dieser Sitzung, nicht den Subagenten.

Dieses Beispiel ist eine Nachricht, die ein Claude an einen anderen schrieb, wie sein vollständiger Text liest, wenn Sie ihn erweitern:

```text wrap theme={null}
Schema migration finished
The new column is tenant_id, and rebasing on main is safe now.
```

<h3 id="control-inbound-messages">
  Inbound-Nachrichten kontrollieren
</h3>

Setzen Sie [`crossSessionInbound`](/docs/de/settings-reference#crosssessioninbound), um zu wählen, was eine Sitzung mit Nachrichten tut, die von Ihren anderen Sitzungen ankommen:

| Wert     | Verhalten                                                                                                                                                                                                                                       |
| :------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `accept` | Claude Code übermittelt jede Nachricht an Claude                                                                                                                                                                                                |
| `hold`   | Claude Code zeigt eine Benachrichtigung für jede Nachricht an und übermittelt sie nicht. Wenn ein `accept` später gilt, gemäß den [Vorrangregeln](/docs/de/settings-reference#crosssessioninbound), gibt Claude Code die gehaltenen Nachrichten frei |
| `refuse` | Claude Code verwirft jede Nachricht, ohne sie zu übermitteln                                                                                                                                                                                    |

Über das Bearbeiten einer Einstellungsdatei hinaus können Sie den Wert in der `/config`-Zeile **Nachrichten von Ihren anderen Sitzungen** auswählen. Claude Code schreibt den Wert, den Sie auswählen, in Ihre Benutzereinstellungen. Die Zeile erfordert Claude Code v2.1.232 oder später und erscheint nicht, während verwaltete Einstellungen oder das Flag `--settings` den Schlüssel setzt, da ein Benutzereinstellungswert dann nicht gelten würde. Claude Code lehnt die Kurzform `/config crossSessionInbound=value` für diesen Schlüssel ab.

Um zu sehen, welcher Wert gilt, folgen Sie den `crossSessionInbound`-Vorrangregeln in der [Einstellungsreferenz](/docs/de/settings-reference#crosssessioninbound).

Wenn kein Wert gilt, entscheidet Claude Code pro Nachricht aus den Berechtigungsmodi der beiden Sitzungen. Es gruppiert Sitzungen, die [Berechtigungsaufforderungen umgehen](/docs/de/permission-modes#skip-all-checks-with-bypasspermissions-mode), in eine Klasse und jede andere Sitzung in die andere. Plan Mode zählt als Umgehen in interaktiven Terminal-Sitzungen mit verfügbaren Bypass-Berechtigungen, und [auto](/docs/de/permission-modes#eliminate-prompts-with-auto-mode), `acceptEdits` und `dontAsk` zählen als Aufforderung:

* **Die empfangende Sitzung fordert Berechtigungen an**: Claude Code übermittelt jede Nachricht. Es hält eine nur für Ihre Genehmigung, wenn die sendende Sitzung sich selbst als Umgehen von Berechtigungsaufforderungen identifiziert.
* **Die empfangende Sitzung umgeht Berechtigungsaufforderungen**: Claude Code hält jede Nachricht für Ihre Genehmigung. Es übermittelt eine nur, wenn die sendende Sitzung sich selbst auch als Umgehen identifiziert.

Wenn der Standard eine Nachricht hält, öffnet Claude Code einen Genehmigungsdialog in der empfangenden Sitzung. Der Dialog zeigt den Absender und eine Vorschau:

* **Genehmigen** übermittelt diese eine Nachricht an Claude.
* **Ablehnen** oder das Schließen des Dialogs verwirft sie.
* Wenn der Dialog über die [`dialogExpiry`](/docs/de/settings-reference#dialogexpiry)-Frist hinaus unantwortlich bleibt, schließt Claude Code ihn und verwirft die Nachricht. Die Frist beträgt standardmäßig fünf Minuten.
* Während kein Terminal an eine [Hintergrund-Sitzung](/docs/de/agent-view) angehängt ist, lässt Claude Code den Dialog über die Frist hinaus offen. Nachdem Sie angehängt haben, schließt Claude Code den Dialog und verwirft die Nachricht nur, wenn sie für einen vollständigen Fristzeitraum unantwortlich bleibt.
* Wenn sich die Berechtigungsmodus-Klasse dieser Sitzung ändert, während Nachrichten gehalten werden, wendet Claude Code die Inbound-Regeln erneut an, übermittelt die Nachrichten, die sie jetzt akzeptieren, und zeigt eine Benachrichtigung an.
* Wenn eine Einstellungsänderung `refuse` anwendbar macht, während Nachrichten gehalten werden, verwirft Claude Code jede gehaltene Nachricht und meldet eine Ablehnung an jeden Absender, den es erreichen kann.

Wenn der Absender eine Sitzung auf dem gleichen Computer ist, sendet Claude Code eine Benachrichtigung zurück an ihn, wenn der Empfänger die Nachricht hält, und eine Nachverfolgung, wenn der Empfänger sie später übermittelt, ablehnt oder ablaufen lässt. Die Benachrichtigung erreicht den sendenden Claude, sodass er weiß, nicht weiter auf eine Nachricht zu warten, die die andere Sitzung nicht gelesen hat.

In einer interaktiven sendenden Sitzung erscheint die Benachrichtigung im Transkript. Ein [`claude -p`](/docs/de/headless)-Absender erhält sie in [gestreamter Ausgabe](/docs/de/headless#stream-responses) als eine [informative `system`-Nachricht](/docs/de/agent-sdk/typescript#sdkinformationalmessage). Benachrichtigungen an `claude -p`-Absender erfordern Claude Code v2.1.271 oder später.

Wenn der Empfänger die Nachricht ablehnt, sagt die Benachrichtigung des Absenders, dass der Empfänger keine Cross-Session-Nachrichten akzeptiert, und teilt dem Claude des Absenders mit, nicht zu warten oder erneut zu senden.

Claude Code hält höchstens 100 Nachrichten, getrennt von der Übermittlungswarteschlange, und verwirft danach die ältesten.

<h3 id="non-interactive-sessions">
  Nicht-interaktive Sitzungen
</h3>

Claude Code bindet einen Inbox-Socket für eine [`claude -p`](/docs/de/headless)-Sitzung wie eine interaktive, sodass ein langfristiger `-p`-Worker Nachrichten empfangen kann und in der Auflistung angezeigt wird. Wenn Sie eine Sitzung im [Bare Mode](/docs/de/headless#start-faster-with-bare-mode) starten, bindet Claude Code den Socket nicht, sodass diese Sitzung keine Nachrichten empfangen kann und nicht in der Agent-Liste angezeigt wird.

Eine `-p`-Sitzung kann den Genehmigungsdialog nicht anzeigen. Wenn der [Inbound-Standard](#control-inbound-messages) eine Nachricht dort hält, behält Claude Code sie für die gleiche [`dialogExpiry`](/docs/de/settings-reference#dialogexpiry)-Frist, die der Dialog verwendet, standardmäßig fünf Minuten:

* **Vor der Frist**: Wenn eine Einstellung oder Einstellungsänderung die Nachricht zulässt, übermittelt Claude Code sie.
* **Nach der Frist**: Claude Code verwirft die Nachricht und meldet sie als abgelaufen an einen Absender, den es erreichen kann.

Setzen Sie `dialogExpiry` auf `"never"`, um Standard-gehaltene Nachrichten bis zum Ende der Sitzung zu behalten. Eine Nachricht, die durch eine explizite `hold`-Einstellung gehalten wird, läuft nicht ab; Claude Code übermittelt sie nur, wenn ein `accept` später gilt.

Wenn die Sitzung mit noch gehaltenen Nachrichten endet, meldet Claude Code sie als abgelaufen an jeden Absender, den es erreichen kann. Vor v2.1.225 galt keine Frist in einer `-p`-Sitzung: Eine gehaltene Nachricht blieb gehalten, es sei denn, eine Berechtigungsmodus-Änderung während des Laufs übermittelte sie, und eine Sitzung, die mit gehaltenen Nachrichten endete, meldete nichts an ihre Absender.

Um einen `-p`-Worker unbeaufsichtigt Nachrichten zu nehmen, starten Sie ihn mit `crossSessionInbound` auf `accept` in seinem `--settings`-Wert. Ein `accept` in Ihren Benutzereinstellungen funktioniert auch, gilt aber für jede Sitzung, die Sie ausführen.

<h3 id="the-sessions-inbox-socket">
  Der Inbox-Socket der Sitzung
</h3>

Lesen Sie diesen Abschnitt, wenn eine Sitzung, die Sie erwarten, nicht in der Agent-Liste ist, wenn Sie möchten, dass ein Skript oder Hook in eine Sitzung postet, oder wenn ein sandboxierter Befehl den Socket nicht erreichen kann.

Claude Code bindet einen Inbox-Socket für jede Sitzung mit aktiviertem Cross-Session-Messaging, wo andere Sitzungen auf dem Computer Nachrichten übermitteln. Der Socket ist ein Unix-Domain-Socket auf macOS und Linux, einschließlich Linux in WSL 2, und ein Named Pipe auf nativem Windows. Für welche Sitzungstypen einen binden, siehe [Nicht-interaktive Sitzungen](#non-interactive-sessions).

Sie können den Pfad des Sockets an zwei Stellen finden:

* `/status` zeigt ihn in der Zeile `Peer address`. Der Pfad ist mit `uds:` vorangestellt.
* Claude Code exportiert ihn zu [Hooks](/docs/de/hooks) und Bash-Befehlen als die Umgebungsvariable [`CLAUDE_CODE_MESSAGING_SOCKET`](/docs/de/env-vars#variables):
  * In einer Sitzung, die mit aktiviertem Messaging startet, exportiert Claude Code die Variable, bevor ein Hook läuft, einschließlich `SessionStart`.
  * Jede Sitzung exportiert ihren eigenen Socket, niemals einen, der von einer übergeordneten Sitzung geerbt wird.

Auf macOS und Linux beschränkt Claude Code den Socket auf Ihren Betriebssystem-Benutzer. Auf nativem Windows erfordert es stattdessen, dass jede Verbindung sich zuerst mit einem Schlüssel authentifiziert, den nur Ihr Betriebssystem-Benutzer lesen kann. Auf jeden Fall kann auf einem gemeinsamen Computer die Sitzung eines anderen Benutzers nicht an ihn übermitteln.

Auf macOS und Linux lehnt Claude Code auch ab, den Socket in einem Verzeichnis zu erstellen, das es nicht akzeptieren kann, zum Beispiel eines, das ein anderer Benutzer besitzt, und verwendet stattdessen ein privates Pro-Benutzer-Verzeichnis, `/tmp/cc-socks-<uid>`. Wenn es kein Verzeichnis akzeptieren kann, läuft die Sitzung ohne einen Inbox: Claude Code zeigt eine Benachrichtigung, `/status` zeigt `unavailable` und den Grund in seiner Zeile `Peer address`, und das [`--debug`](/docs/de/cli-reference#cli-flags)-Protokoll erfasst die vollständige Ablehnung.

Neben dem Pfad des Sockets exportiert Claude Code ein Pro-Sitzungs-Token als [`CLAUDE_CODE_MESSAGING_TOKEN`](/docs/de/env-vars#variables). Ein Skript, das an seinen eigenen Socket der Sitzung postet, kann `{"type":"auth","token":"<token>"}` als erste Zeile seiner Verbindung senden, wobei `<token>` der Wert von `CLAUDE_CODE_MESSAGING_TOKEN` ist. Ob Claude Code die Zeile erfordert, hängt von der Plattform ab:

* **macOS und Linux, einschließlich WSL 2**: die Zeile ist optional. Claude Code akzeptiert eine Verbindung mit oder ohne sie.
* **Natives Windows**: die Zeile ist erforderlich. Claude Code schließt jede Verbindung, deren erste Zeile keine gültige Auth-Zeile ist, und übermittelt nichts von dieser Verbindung.

Öffnen Sie die Verbindung nur, wenn die Nachricht, die Sie posten, bereit ist. Claude Code schließt eine Verbindung, die innerhalb von 30 Sekunden keine vollständige Zeile gesendet hat, sodass erfassen Sie zuerst die Ausgabe eines langsamen Befehls und öffnen Sie dann die Verbindung, um sie zu senden.

Die [Eigene-Kind-Regeln](#own-child-messages) unten sagen, wann Claude Code das Token konsultiert und wie es eine Nachricht behandelt, die es nicht verifizieren kann.

<span id="own-child-messages" />Claude Code führt Nachrichten, die auf dem Socket ankommen, durch die gleichen [Inbound-Kontrollen](#control-inbound-messages) wie jede andere Peer-Nachricht, mit einer Ausnahme und einer Voraussetzung:

* **Eigene-Kind-Nachrichten**: Wenn kein `crossSessionInbound`-Wert gilt, übermittelt Claude Code eine Nachricht, die es verifiziert, kam von den eigenen Kind-Prozessen der Sitzung, wie ein Hook oder Bash-Befehl, der an seinen eigenen Socket der Sitzung zurückpostet.
  * Auf Linux, einschließlich in WSL 2, kann Claude Code durch Prozess-Beweis verifizieren, auch für ein Kind, das bereits beendet wurde. Auf macOS kann es das nur verifizieren, während der postende Prozess noch läuft, und in einem Container, wo Claude Code als Prozess-ID 1 läuft, hat es überhaupt keinen Prozess-Beweis. Auf nativem Windows hat es auch keinen.
  * Auf macOS, nachdem der postende Prozess beendet wurde, und in Containern, wo Claude Code als Prozess-ID 1 läuft, fehlt dieser Prozess-Beweis, und Claude Code verifiziert stattdessen ein Kind, das das exportierte [`CLAUDE_CODE_MESSAGING_TOKEN`](/docs/de/env-vars#variables) der Sitzung in der Auth-Zeile gesendet hat, die seine Verbindung öffnete. Auf nativem Windows ist dieses Token die einzige Möglichkeit, wie Claude Code eine Eigene-Kind-Nachricht verifiziert.
  * Wenn Claude Code auf keine Weise verifizieren kann, behandelt es die Nachricht wie jede andere, die keine Berechtigungsklasse behauptet, sodass eine Sitzung, die Berechtigungsaufforderungen umgeht, sie für Ihre Genehmigung hält.
* **Sandboxierte Sitzungen**: Kontrollieren Sie, ob ein Bash-Befehl den Socket von innen in der [Sandbox](/docs/de/sandboxing) mit den Unix-Socket-Einstellungen der Sandbox erreichen kann, [`sandbox.network.allowAllUnixSockets` und `sandbox.network.allowUnixSockets`](/docs/de/settings-reference#sandbox-settings).

<h2 id="restrict-cross-session-messaging">
  Cross-Session-Messaging einschränken
</h2>

Über die Pro-Nachricht-Standards hinaus können Sie Messaging auf zwei Wegen einschränken. Verlangen Sie Ihre Genehmigung, bevor eine Nachricht den Computer verlässt, oder schalten Sie Messaging für eine Sitzung oder eine Organisation aus.

<h3 id="require-approval-for-cross-machine-messages">
  Genehmigung für Cross-Computer-Nachrichten verlangen
</h3>

Setzen Sie [`isolatePeerMachines`](/docs/de/settings-reference#isolatepeermachines) auf `true`, um Ihre explizite Genehmigung zu verlangen, bevor ein `SendMessage` eine Sitzung jenseits dieses Computers erreicht:

```json theme={null}
{
  "isolatePeerMachines": true
}
```

Mit diesem Satz fragt Claude Code nach Ihrer Genehmigung, bevor Claude's Nachricht an eine Sitzung jenseits dieses Computers geht, auch im `bypassPermissions`-Modus, der gewöhnliche Berechtigungsaufforderungen überspringt. Ein `true` aus einem beliebigen Einstellungsbereich gilt, sodass eine eingecheckte Projektdatei die Anforderung einschalten, aber nicht ausschalten kann. Claude Code fordert nicht für Nachrichten zwischen Sitzungen auf dem gleichen Computer auf.

<h3 id="turn-off-cross-session-messaging">
  Cross-Session-Messaging ausschalten
</h3>

Empfangen und Senden sind separate Kontrollen, schalten Sie also aus, welche Richtung Sie benötigen, oder beide. Verwenden Sie `crossSessionInbound` für Nachrichten, die ankommen, und Berechtigungsregeln für das, was Claude hier senden oder auflisten kann:

* **Empfangen stoppen**: Setzen Sie `crossSessionInbound` auf `refuse`, und Claude Code verwirft eingehende Peer-Nachrichten, ohne sie zu übermitteln. Aus Projekt- oder lokalen Einstellungen gilt `refuse` über jede andere Quelle, und aus Ihren Benutzereinstellungen gilt es, es sei denn, verwaltete Einstellungen oder das Flag `--settings` setzen einen Wert.
* **Senden und Auflisten stoppen**: Fügen Sie [Berechtigungsregeln zum Ablehnen](/docs/de/permissions#tool-specific-permission-rules) hinzu, die `SendMessage` und `ListAgents` benennen. Beide nehmen den bloßen Tool-Namen ohne Spezifizierer.

Administratoren können beide Seiten für eine Organisation in [verwalteten Einstellungen](/docs/de/managed-settings) ausschalten, indem sie die Ablehnungsregeln mit dem `refuse` kombinieren:

```json theme={null}
{
  "permissions": {
    "deny": ["SendMessage", "ListAgents"]
  },
  "crossSessionInbound": "refuse"
}
```

Mit diesem an Ort und Stelle bindet Claude Code immer noch den Inbox-Socket jeder Sitzung, verwirft aber jede Nachricht, die darauf ankommt, ohne etwas an Claude zu übermitteln. Das Ablehnen von `SendMessage` entfernt auch Messaging an Subagenten und Agent-Team-Teamkollegen, da das gleiche Tool beiden dient. Eine ablehnende Sitzung zeigt keine sichtbare Änderung in ihrem eigenen `/status` oder in den Auflistungen anderer Sitzungen auf dem gleichen Computer, um es zu bestätigen, überprüfen Sie die Einstellungsdateien, die auf diese Sitzung gelten, anstatt ihren Status.

<h2 id="availability">
  Verfügbarkeit
</h2>

Cross-Session-Messaging erfordert Claude Code v2.1.224 oder später auf macOS, Linux und WSL 2, und v2.1.234 oder später auf nativem Windows. Verfügbarkeit und welche Sitzungen Claude benachrichtigen kann, hängen auch von Ihrem Betriebssystem, Anbieter und Konfiguration ab:

* **Betriebssystem**: verfügbar auf macOS, Windows und Linux, einschließlich Linux in WSL 2.

* **Sitzungen auf diesem Computer**: verfügbar auf jedem Anbieter, einschließlich Amazon Bedrock, Claude Platform auf AWS, Google Cloud's Agent Platform und Microsoft Foundry, und in Sitzungen, die mit [Feature-Flag-Abruf](/docs/de/env-vars#features-that-need-feature-flag-fetching) aus laufen. Auf diesen Anbietern und mit Flag-Abruf aus erfordert Same-Machine-Messaging Claude Code v2.1.248 oder später. Claude Code übermittelt diese Nachrichten über einen [Pro-Sitzungs-Socket auf Ihrem Computer](#the-sessions-inbox-socket), niemals durch Anthropic-Server.

  Um eine Sitzung davon abzuhalten, sie zu empfangen, setzen Sie [`crossSessionInbound`](#turn-off-cross-session-messaging) auf `refuse`.

* **Sitzungen jenseits dieses Computers**: Claude findet Ihre [Claude Code im Web](/docs/de/claude-code-on-the-web)-Sitzungen und Ihre Sitzungen auf anderen Computern von einer Sitzung, die mit Remote Control verbunden ist, was eine claude.ai-Anmeldung als aktive Authentifizierung dieser Sitzung und die anderen [Remote Control-Anforderungen](/docs/de/remote-control#requirements) benötigt. Claude kann diese Sitzungen nicht mit einem API-Schlüssel oder auf Amazon Bedrock, Claude Platform auf AWS, Google Cloud's Agent Platform und Microsoft Foundry finden.

Um eine Sitzung zu überprüfen, geben Sie `/list-agents` ein, auch verfügbar als `/peers`. Das Ergebnis trennt eine Sitzung, die die Funktion nicht hat, von einer Sitzung, wo etwas Engeres eine Nachricht blockierte, wie ein fehlendes `SendMessage`-Tool oder ein abgelehnter Send:

* **`/list-agents` wird nicht erkannt**: die Sitzung hat kein Cross-Session-Messaging. Arbeiten Sie durch die Anforderungen oben, beginnend mit `claude --version` für die Versionsanforderung.
* **`/list-agents` funktioniert, aber ein Send kam nicht an**: Messaging ist an, und etwas Engeres gilt:
  * **Ablehnungsregeln**: eine [Berechtigungsregel zum Ablehnen](#turn-off-cross-session-messaging) entfernt die Tools `SendMessage` und `ListAgents`.
  * **Inbound-Kontrollen**: die [Inbound-Kontrollen der empfangenden Sitzung](#control-inbound-messages) können das, was Sie senden, halten oder ablehnen.
  * **Cloud-Sitzung fehlt**: eine Cloud-Sitzung erscheint nur, während diese Sitzung mit [Remote Control](/docs/de/remote-control) verbunden ist.
  * **Sitzung auf anderem Computer fehlt**: eine Sitzung auf einem anderen Ihrer Computer erscheint nur, wenn sie mit [Remote Control](/docs/de/remote-control) läuft und diese Sitzung auch verbunden ist.
  * **Sitzung auf anderem Computer `offline`**: eine Nachricht an eine Sitzung, die als `offline` aufgelistet ist, wird durchgeleitet, kommt aber [erst an, nachdem sich der Computer dieser Sitzung wieder verbindet](#message-sessions-on-other-machines).
  * **Ältere Cloud- oder Sitzung auf anderem Computer fehlt**: Claude Code [liest diese Sitzungslisten neueste zuerst und stoppt nach einer begrenzten Anzahl von Seiten](#see-which-sessions-claude-can-reach), sodass Claude eine Sitzung, die über sie hinausfiel, nicht nach Name benachrichtigen kann.
  * **Ein Gespräch starten**: [Nachrichten an Sitzungen auf anderen Computern](#message-sessions-on-other-machines) behandelt das Starten eines Gesprächs mit einer Sitzung jenseits dieses Computers.

In einer Sitzung mit Messaging zeigt `/status` auch eine Zeile `Peer address` mit der eigenen Inbox-Adresse der Sitzung, oder `unavailable` und den Grund, wenn Claude Code [einen Inbox nicht einrichten konnte](#the-sessions-inbox-socket).

<h2 id="limitations">
  Einschränkungen
</h2>

Die Grenzen hier sind Eigenschaften des Messaging-Kanals selbst und gelten überall dort, wo die Funktion läuft. Für Plattform- und Anbieter-Lücken, siehe stattdessen [Verfügbarkeit](#availability).

* **Nur Klartext**: Claude sendet nur Klartext über Sitzungen. Strukturierte [Agent-Team](/docs/de/agent-teams)-Protokoll-Nachrichten bleiben in einem Team.
* **Same-Machine-Nachrichtengröße ist begrenzt**: Claude Code lehnt eine Nachricht an eine Sitzung auf diesem Computer ab, sobald ihre serialisierte Form etwa eine Million Zeichen überschreitet. Die Ablehnung [benennt die genauen Größen](/docs/de/errors#message-too-large-for-cross-session-delivery). Nichts erreicht die empfangende Sitzung.
* **Schnelle Bursts an eine Sitzung werden beim Absender abgelehnt**: Sobald ein schneller Burst von Nachrichten an eine Sitzung auf diesem Computer erreicht, was diese Sitzung akzeptiert, lehnt Claude Code weitere Sends in der sendenden Sitzung ab. Die [Ablehnung benennt den Burst](/docs/de/errors#too-many-messages-to-this-session-just-now) und teilt Claude mit, den Rest in eine Nachricht zu packen oder zu warten. Vor v2.1.236 meldete Claude Code diese Sends als gesendet, während die empfangende Sitzung sie verwarf.
* **Nachrichtenschleifen werden gedrosselt**: In der empfangenden Sitzung drosselt Claude Code wiederholte Nachrichten pro Absender, verwirft identische Wiederholungen, die in einem kurzen Fenster ankommen, und reiht höchstens 50 akzeptierte Nachrichten für Claude zum Lesen ein. Eine Nachrichtenschleife zwischen zwei Sitzungen stoppt daher von selbst. Wenn die Ratenbegrenzung, Wiederholungsprüfung oder Warteschlangen-Obergrenze eine Nachricht von einer interaktiven Sitzung auf diesem Computer verwirft, teilt Claude Code dieser Sitzung mit, welche verwirft wurde, und teilt ihrem Claude mit, nicht sofort erneut zu senden.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Subagenten](/docs/de/sub-agents#resume-subagents) und [Agent-Teams](/docs/de/agent-teams#messages-between-agents): Messaging innerhalb einer einzelnen Sitzung oder eines Teams
* [Hintergrund-Agenten](/docs/de/agent-view): Versenden und überwachen Sie die parallelen Sitzungen, die Sie möglicherweise benachrichtigen
* [Remote Control](/docs/de/remote-control): Verbinden Sie diese Sitzung, um Ihre Sitzungen auf anderen Computern zu erreichen
* [Einstellungen](/docs/de/settings-reference#all-settings): `crossSessionInbound`, `isolatePeerMachines` und `dialogExpiry`
* [Berechtigungsmodi](/docs/de/permission-modes): die Modi hinter den zwei Klassen des Inbound-Standards
* [Tools-Referenz](/docs/de/tools-reference): die Zeilen `ListAgents` und `SendMessage` in der Tools-Tabelle
* [Agenten parallel ausführen](/docs/de/agents): vergleichen Sie die Wege, wie Claude Code mehrere Agenten ausführt
