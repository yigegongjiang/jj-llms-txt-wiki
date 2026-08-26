> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Champion-Kit

> Ein Leitfaden für Ingenieure, die Claude Code intern fördern: was man teilen sollte, wie man Fragen beantwortet und wie man die Akzeptanz im Team erhöht.

Diese Seite ist für einzelne Ingenieure, die Claude Code bereits nutzen und ihr Team bei der Einführung unterstützen möchten. Sie behandelt, was man teilen sollte, wie man die Fragen beantwortet, die man erhält, einen 30-Tage-Leitfaden und Antworten auf häufige Bedenken.

Die Einführung eines Entwickler-Tools geschieht selten aufgrund einer Rollout-Ankündigung. Sie geschieht, weil jemand im Team das Tool gut nutzt, offen darüber spricht und es anderen leicht macht, es zu übernehmen. Die Arbeit, die Sie als Champion leisten, hat eine überproportionale Wirkung: Jedes Beispiel, das Sie teilen, verkürzt die Lernkurve für die Ingenieure, die nach Ihnen kommen, und jede Frage, die Sie öffentlich beantworten, verwandelt die Erfahrung einer Person in etwas, das das ganze Team nutzen kann. Sie fungieren als Multiplikator für Ihr Team, nicht als Helpdesk, und dieser Leitfaden ist so strukturiert, dass die Rolle unter diesen Bedingungen nachhaltig bleibt.

<h2 id="the-champion-role">
  Die Champion-Rolle
</h2>

Die Rolle besteht aus drei Verhaltensweisen, die sich gegenseitig verstärken.

| Verhalten                           | Wie es in der Praxis aussieht                                                                                                                                                                                           | Warum es wichtig ist                                                                                                                                                                    |
| ----------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Teilen Sie, was Sie entdecken       | Posten Sie die Prompts, Screenshots und kleinen Erfolge aus Ihrer eigenen Arbeit an den Orten, die Ihr Team bereits liest, z. B. in einem Engineering-Kanal, einem Standup-Thread oder einer Pull-Request-Beschreibung. | Beispiele aus Ihrer eigenen Codebasis sind überzeugender als jede externe Dokumentation, da Kollegen genau sehen können, wie das Tool auf die Probleme angewendet wird, die Sie teilen. |
| Seien Sie die Person, die man fragt | Wenn ein Kollege fragt, wie Sie etwas erreicht haben, antworten Sie mit dem tatsächlichen Prompt, den Sie verwendet haben, damit er ihn direkt auf seine eigene Aufgabe anwenden kann.                                  | Ein konkretes, ausführbares Beispiel schließt die Lücke zwischen Neugier und einer ersten erfolgreichen Nutzung, wo die meisten Einführungsbemühungen steckenbleiben.                   |
| Erweitern Sie den Kreis             | Etablieren Sie eine kleine Anzahl leichter, wiederkehrender Gewohnheiten, z. B. einen dedizierten Kanal oder einen wöchentlichen Thread, damit der Schwung auch dann anhält, wenn Ihre Aufmerksamkeit woanders liegt.   | Eine Einführung, die von einer einzelnen Person abhängt, ist fragil. Eine Einführung, die durch gemeinsame Gewohnheiten getragen wird, setzt sich von selbst fort.                      |

Das meiste davon passt natürlich in die Arbeit, die Sie bereits leisten. Der Unterschied liegt in einer kleinen zusätzlichen Absicht darüber, wo Ihre Entdeckungen gepostet werden und wie Ihre Antworten verbreitet werden.

<h3 id="what-this-should-cost-you">
  Was dies Sie kosten sollte
</h3>

Setzen Sie Erwartungen mit sich selbst und mit Ihrem Vorgesetzten. Die folgenden Aktivitäten sollen in eine normale Arbeitswoche passen, und die Rolle sollte ein Multiplikator für Ihre bestehende Arbeit bleiben, anstatt eine zusätzliche Support-Verantwortung zu sein.

| Aktivität                                     | Zeit pro Woche   | Anleitung                                                                                                                                          |
| --------------------------------------------- | ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| Erfolge und Prompts posten                    | Etwa 15 Minuten  | Erfassen Sie diese im Moment mit einem Screenshot und ein oder zwei Sätzen; vermeiden Sie es, sie in formale Schriftstücke umzuwandeln.            |
| Fragen in einem gemeinsamen Kanal beantworten | Etwa 20 Minuten  | Beantworten Sie die Frage einmal öffentlich, dann verlinken Sie auf diese Antwort, wenn die Frage erneut auftritt.                                 |
| Wöchentlichen Show-and-Tell-Thread hosten     | Etwa 5 Minuten   | Sie posten den Eröffnungs-Prompt; das Team liefert den Inhalt.                                                                                     |
| Optionales Pairing oder Walkthroughs          | 0 bis 30 Minuten | Reservieren Sie dies für Kollegen, die wirklich blockiert sind, und bieten Sie den [Quickstart](/docs/de/quickstart)-Link an, bevor Sie Zeit einplanen. |

<h2 id="share-what-you-discover">
  Teilen Sie, was Sie entdecken
</h2>

Ihre eigene Erfahrung ist das überzeugendste Material, das Ihre Kollegen antreffen werden, da es spezifisch für die Codebasis, Workflows und Probleme ist, die Sie alle teilen. Dokumentation sagt den Menschen, was möglich ist; Ihre Posts zeigen ihnen, was in Ihrer Umgebung tatsächlich funktioniert.

<h3 id="what-is-worth-sharing">
  Was es wert ist, geteilt zu werden
</h3>

Die nützlichsten Posts beschreiben eine Technik, die ein Kollege morgen wiederverwenden kann, anstatt eines Ergebnisses, das bereits abgeschlossen ist. Techniken verstärken sich, wenn sie sich durch ein Team verbreiten; Statusaktualisierungen nicht.

Beispiele für wiederverwendbare Techniken:

* „Ich habe gelernt, dass das @-Erwähnen eines Verzeichnisses funktioniert. Wenn ich es auf `@src/components/` zeige und frage, welche Tests fehlen, werden zwei angezeigt, die ich übersehen hatte."
* „Plan Mode (`Shift+Tab`) zeigt genau, welche Dateien berührt werden, bevor eine Bearbeitung vorgenommen wird, weshalb ich mich wohlfühle, ihn auf gemeinsamen Code anzuwenden."
* „Ich habe einen Stop-Hook konfiguriert, damit ich eine Desktop-Benachrichtigung erhalte, wenn eine lange Aufgabe abgeschlossen ist. Die Konfiguration ist im Thread."
* „Das Ausführen von `/init` generiert eine `CLAUDE.md` aus dem Repository, damit der Assistent nicht mehr nach unseren Konventionen fragt."

<h3 id="where-to-share-it">
  Wo man es teilt
</h3>

Posten Sie überall dort, wo Ihr Team bereits liest. Das Ziel ist es, Beispiele in den Weg der normalen Arbeit zu platzieren, anstatt ein Ziel zu schaffen.

| Ort                                                    | Am besten geeignet für                                                 | Empfohlenes Format                                                                                              |
| ------------------------------------------------------ | ---------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| Ein `#claude-code`- oder allgemeiner Engineering-Kanal | Entdeckungen, Prompts und „heute habe ich gelernt"-Momente             | Ein Screenshot mit ein oder zwei Sätzen Kontext                                                                 |
| Pull-Request-Beschreibungen                            | Demonstration des Ansatzes auf echtem Code, den Reviewer bereits lesen | Ein einzelner Satz wie „Claude und ich haben dieses Refactoring durchgeführt; ich stelle gerne den Ansatz vor." |
| Standups oder wöchentliche schriftliche Updates        | Normalisierung der Nutzung mit Vorgesetzten und Skip-Level-Managern    | Ein Satz, der ein konkretes Ergebnis beschreibt                                                                 |
| Team-Wiki oder interne Dokumentation                   | Dauerhafte Muster, benutzerdefinierte Skills und `CLAUDE.md`-Beispiele | Eine kurze Seite, verlinkt vom Kanal-Thema, damit sie auffindbar bleibt                                         |

<h3 id="the-format-that-works">
  Das Format, das funktioniert
</h3>

Ein Screenshot mit einem einzelnen Satz Kontext oder eine kurze Vorher-Nachher-Beschreibung ist im Allgemeinen das richtige Detaillierungsniveau. Halten Sie jeden Post kurz genug, dass jemand, der vorbeiscrollt, den Punkt trotzdem versteht. Ein langer Schriftsatz wird tendenziell für später gespeichert und vergessen, während ein kurzer Post mit einem Screenshot tendenziell kopiert und ausprobiert wird.

Die folgenden Beispiel-Posts veranschaulichen Ton und Länge; passen Sie sie an, anstatt sie wörtlich zu kopieren.

```text theme={null}
Heute gelernt, dass das @-Erwähnen eines Verzeichnisses funktioniert. Ich habe es auf
@src/components/ gezeigt und gefragt, welche Komponenten Tests fehlen, und es
wurden zwei angezeigt, die ich vergessen hatte.
```

```text theme={null}
Ich habe einen Stop-Hook konfiguriert, damit ich eine Desktop-Benachrichtigung erhalte, wenn eine lange
Aufgabe abgeschlossen ist. Ich habe ein Refactoring gestartet, bin weggegangen und wurde benachrichtigt, als
es fertig war. Die Konfiguration ist im Thread.
```

```text theme={null}
Plan Mode ist der Grund, warum ich mich wohlfühle, dies auf Code zu verwenden, der wichtig ist.
Drücken Sie Shift+Tab, bis Sie „plan" sehen; es zeigt genau, welche Dateien es
berühren möchte, bevor etwas geändert wird.
```

<h2 id="be-the-person-people-ask">
  Seien Sie die Person, die man fragt
</h2>

Sobald Sie ein paar Beispiele geteilt haben, werden Fragen folgen. Dies ist der Punkt, an dem die Champion-Rolle die größte Hebelwirkung hat, da eine gute Antwort auf eine Person häufig mehrere andere freischaltet, die denselben Kanal beobachten.

<h3 id="answer-with-a-prompt-rather-than-an-explanation">
  Antworten Sie mit einem Prompt statt mit einer Erklärung
</h3>

Wenn ein Kollege fragt, wie Sie etwas erreicht haben, ist die nützlichste Antwort der Prompt, den Sie tatsächlich verwendet haben. Sie werden mehr lernen, wenn sie diesen Prompt gegen ihr eigenes Problem ausführen, als aus jeder Beschreibung, die Sie schreiben könnten, und es gibt ihnen etwas, das sie sofort umsetzen können.

```text theme={null}
Kollege: Wie hast du es geschafft, diese Race Condition zu finden?

Champion: Ich habe gefragt: „Der Test in @tests/scheduler.test.ts ist instabil, finde heraus, warum", und es hat zwei nicht verbundene Promises im Scheduler verfolgt. Versuchen Sie die gleiche Formulierung bei Ihrem Test.
```

<h3 id="point-at-the-feature-rather-than-the-documentation">
  Zeigen Sie auf die Funktion statt auf die Dokumentation
</h3>

Eine Antwort wie „Versuchen Sie Plan Mode, drücken Sie `Shift+Tab`, bis Sie ihn sehen" ist im Moment nützlicher als ein Link zur Dokumentation. Wenn die Person später mehr Tiefe braucht, wird sie sie selbst finden; jetzt brauchen sie das eine, das sie freischaltet.

<h3 id="questions-you-are-likely-to-hear">
  Fragen, die Sie wahrscheinlich hören werden
</h3>

| Frage                                             | Vorgeschlagene Antwort                                                                                                                                                                                                                                                               | Nachschlageressource                                        |
| ------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------- |
| „Womit sollte ich es zuerst versuchen?"           | Empfehlen Sie eine echte, aber begrenzte Aufgabe, idealerweise einen Bug oder eine Aufgabe, die die Person aufgeschoben hat, weil sie mühsam ist, nicht schwierig.                                                                                                                   | [Häufige Workflows](/docs/de/common-workflows)                   |
| „Wie kann ich meinem Code vertrauen?"             | Stellen Sie Plan Mode vor: Drücken Sie `Shift+Tab`, um ihn zu aktivieren, Claude schlägt genau vor, was es ändern möchte, und nichts wird geändert, bis der Benutzer zustimmt.                                                                                                       | [Berechtigungen](/docs/de/permissions)                           |
| „Lohnt sich der Einrichtungsaufwand?"             | Die Installation dauert etwa zwei Minuten, läuft im Terminal und erfordert keine IDE-Erweiterung. Das einmalige Ausführen von `/init` reicht aus, um zu beginnen.                                                                                                                    | [Quickstart](/docs/de/quickstart)                                |
| „Es hat ein falsches Ergebnis produziert."        | Ermutigen Sie sie, den Fehler an Claude zurückzugeben. Das Einfügen der Fehlermeldung oder des fehlgeschlagenen Tests ist viel effektiver als die Umformulierung der ursprünglichen Anfrage.                                                                                         | [Häufige Workflows](/docs/de/common-workflows)                   |
| „Es versteht unsere Codebase-Konventionen nicht." | Schlagen Sie vor, `/init` auszuführen, um eine `CLAUDE.md`-Datei zu generieren, und fügen Sie dann die Konventionen des Teams, Testbefehle und alle Verzeichnisse hinzu, die vermieden werden sollten.                                                                               | [Memory](/docs/de/memory)                                        |
| „Ist das nur Autovervollständigung?"              | Bieten Sie eine kurze Demonstration an, in der Claude eine unbekannte Datei erklärt, einen Bug über Services hinweg verfolgt oder einen Migrationsplan entwirft. Diese Aufgaben erfordern Überlegungen über das Repository hinweg, nicht das Vervollständigen einer einzelnen Zeile. | Eine zweiminütige Live-Demonstration                        |
| „Was ist mit Sicherheit und Datenbehandlung?"     | Verweisen Sie diese Frage an Ihren Administrator. Die Bereitstellungs- und Datenbehandlungsrichtlinie Ihrer Organisation ist bereits konfiguriert, und Champions sollten diese Antwort nicht improvisieren.                                                                          | [Sicherheit](/docs/de/security) · [Datennutzung](/docs/de/data-usage) |

<h2 id="grow-the-circle">
  Erweitern Sie den Kreis
</h2>

Das Ziel ist nicht, ein Programm zu erstellen oder einen Rollout zu besitzen. Es ist, eine kleine Anzahl leichter Gewohnheiten zu etablieren, die es dem Schwung ermöglichen, nach Ihnen fortzufahren. Wenn Fragen im Kanal von anderen Personen als Ihnen beantwortet werden, hat die Rolle ihre Aufgabe erfüllt.

<h3 id="patterns-that-tend-to-work">
  Muster, die tendenziell funktionieren
</h3>

| Muster                                                         | Wie man es ausführt                                                                                                                                                                                                                                                                                             | Erforderlicher Aufwand                         |
| -------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------- |
| Ein dedizierter Kanal                                          | Erstellen Sie einen `#claude-code`-Kanal (oder einen wiederkehrenden Thread in einem bestehenden), heften Sie den [Quickstart](/docs/de/quickstart)-Link und ein starkes Beispiel an, und beantworten Sie Fragen öffentlich, damit jede Antwort jedem, der zuschaut, zugute kommt.                                   | Etwa fünf Minuten zum Einrichten, dann ambient |
| Ein wöchentlicher Show-and-Tell-Thread                         | Jeden Freitag posten Sie „Womit hat Claude dir diese Woche geholfen?" Keine Vorbereitung, Folien oder Meetings erforderlich; Screenshots und kurze Beschreibungen reichen aus.                                                                                                                                  | Etwa zwei Minuten pro Woche                    |
| Teilen Sie einen benutzerdefinierten Skill                     | Posten Sie Ihre nützlichste `.claude/skills/<name>/SKILL.md`-Datei, z. B. einen `/ship`-Skill, der Tests und Lint vor dem Commit ausführt, mit einer einzeiligen Beschreibung. Da Skills einfaches Markdown sind, können Kollegen sie sofort übernehmen.                                                        | Etwa fünf Minuten pro Skill                    |
| Generieren Sie einen Setup-Leitfaden aus Ihrer eigenen Nutzung | Führen Sie `/team-onboarding` in einem Projekt aus, in dem Sie echte Zeit verbracht haben. Claude scannt Ihre letzten Sessions, Befehle und MCP-Server und erstellt dann einen Leitfaden, den ein neuer Teamkollege als erste Nachricht einfügen kann, um Ihr Setup zu wiederholen. Heften Sie ihn im Kanal an. | Etwa zwei Minuten                              |
| Pairing bei einer ersten Aufgabe                               | Bieten Sie eine einzelne fünfzehnminütige Pairing-Sitzung für jeden an, der anfängt. Ein erfolgreicher Ergebnis auf ihrem eigenen Code ist überzeugender als jede Präsentation.                                                                                                                                 | Etwa fünfzehn Minuten pro Person               |
| Identifizieren Sie den nächsten Champion                       | Der Kollege, der Ihnen die meisten Fragen stellt, ist normalerweise bereit, diese Rolle zu übernehmen. Leiten Sie diese Seite an ihn weiter und teilen Sie die Kanal-Verantwortung zwischen Ihnen auf.                                                                                                          | Vernachlässigbar                               |

<h3 id="thirty-day-playbook">
  30-Tage-Leitfaden
</h3>

Wenn ein lockerer Plan hilfreich ist, spiegelt die folgende Abfolge wider, was über die meisten Teams hinweg tendenziell funktioniert. Passen Sie frei an Ihren Kontext an.

<Steps>
  <Step title="Woche 1: Säen Sie den Kanal">
    Erstellen Sie den Kanal, heften Sie den [Quickstart](/docs/de/quickstart) an, und posten Sie zwei oder drei Ihrer eigenen Beispiele mit den Prompts.

    **Signal, dass es funktioniert:** Ein paar Kollegen reagieren oder antworten, und mindestens eine Frage wird im Kanal gestellt.
  </Step>

  <Step title="Woche 2: Starten Sie den Rhythmus">
    Starten Sie den wöchentlichen Show-and-Tell-Thread, beantworten Sie jede Frage öffentlich, und teilen Sie einen benutzerdefinierten Skill oder `CLAUDE.md`-Snippet.

    **Signal, dass es funktioniert:** Jemand anderes als Sie postet ein Beispiel aus seiner eigenen Erfahrung.
  </Step>

  <Step title="Woche 3: Pairing und Konsolidierung">
    Bieten Sie zwei oder drei kurze Pairing-Sitzungen an und konsolidieren Sie die häufigsten Fragen und Antworten in einer angehefteten FAQ-Nachricht.

    **Signal, dass es funktioniert:** Sie sehen wiederholte Nutzung, wobei die gleichen Kollegen zurückkehren, anstatt einmal zu versuchen und zu stoppen.
  </Step>

  <Step title="Woche 4: Übergabe">
    Identifizieren Sie einen zweiten Champion und teilen Sie eine kurze Zusammenfassung dessen, was funktioniert und was nicht, mit Ihrem Vorgesetzten oder Administrator.

    **Signal, dass es funktioniert:** Fragen im Kanal werden von anderen Personen als Ihnen beantwortet.
  </Step>
</Steps>

<h3 id="when-someone-wants-to-go-deeper">
  Wenn jemand tiefer gehen möchte
</h3>

Sie sind die warme Einführung, nicht das Onboarding-Programm. Wenn ein Kollege von „sollte ich das versuchen" zu „wie werde ich damit effektiv" übergeht, verweisen Sie ihn auf die Seiten [Quickstart](/docs/de/quickstart) und [Häufige Workflows](/docs/de/common-workflows). Sie enthalten kurze Abschnitte, die die Funktionen abdecken, die wirklich nützlich sind, aber schwer zu entdecken sind.

<h2 id="respond-to-common-concerns">
  Reagieren Sie auf häufige Bedenken
</h2>

Gesunde Skepsis ist zu erwarten; Ingenieure sollten vorsichtig mit Tools sein, die ihren Code berühren. Die effektivste Antwort ist selten, den allgemeinen Fall zu argumentieren. Stattdessen erkennen Sie das Bedenken an, bieten eine kurze Umformulierung an und schlagen eine konkrete Demonstration auf dem eigenen Code der Person vor. Die meisten Bedenken werden durch eine einzige erfolgreiche Erfahrung gelöst.

| Bedenken                                               | Vorgeschlagene Antwort                                                                                                                                                                                                                                   | Zu bietender Beweis                                                                |
| ------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| „Ich bin schneller ohne es."                           | Das ist wahrscheinlich wahr für Code, den die Person routinemäßig schreibt. Schlagen Sie vor, es auf die Arbeit zu versuchen, die sie tendenziell vermeiden: Legacy-Dateien, unbekannte Services oder Test-Gerüste, wo die Hebelwirkung am höchsten ist. | Zeitlich eine mühsame Aufgabe auf beide Arten und vergleichen Sie.                 |
| „Ich vertraue KI nicht, Production-Code zu berühren."  | Stimmen Sie zu, dass keine Änderung ungelesen landen sollte. Plan Mode kombiniert mit normalem Diff-Review bedeutet, dass nichts angewendet wird, das der Ingenieur nicht überprüft hat, der gleiche Standard wie jeder Pull Request.                    | Demonstrieren Sie Plan Mode auf einer echten Datei.                                |
| „Es wird Junior-Ingenieure schwächer machen."          | Wenn es gut verwendet wird, ist es ein effektiver Erklärer. Ermutigen Sie Junior-Ingenieure, Claude zu bitten, eine Datei und ihre Aufruforte zu erklären, bevor sie ihn bitten, etwas zu ändern.                                                        | Führen Sie „Erklären Sie @file und wo es aufgerufen wird" zusammen aus.            |
| „Ich habe es einmal versucht und es hat halluziniert." | Dies ist normalerweise ein Kontextproblem, kein Modellproblem. Das @-Erwähnen der relevanten Dateien, das Ausführen von `/init` und das Bereitstellen der tatsächlichen Fehlerausgabe lösen es normalerweise.                                            | Führen Sie ihren ursprünglichen Prompt mit ordnungsgemäßem `@`-Kontext erneut aus. |
| „Wir haben keine Zeit, ein anderes Tool zu lernen."    | Claude Code ist ein Terminal-Befehl, kein Plattform. Wenn es in der ersten Sitzung keinen Wert zurückgibt, ist es angemessen, es beiseite zu legen.                                                                                                      | Eine zweiminütige Installation gefolgt von einem echten Bug.                       |

<h2 id="quick-reference-sheet">
  Schnellreferenz-Blatt
</h2>

Die folgenden Techniken sind diejenigen, die am zuverlässigsten jemanden von einem ersten Versuch zur täglichen Nutzung bewegen. Heften Sie diese Tabelle in einem Kanal an oder teilen Sie sie allein.

| Technik                                        | Wie man sie anwendet                                                                                                                                                                                            |
| ---------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Geben Sie den richtigen Kontext an             | Verwenden Sie `@file`- oder `@directory/`-Referenzen, oder fügen Sie die Fehler- oder Log-Ausgabe direkt ein. Die Bereitstellung relevanten Kontexts ist effektiver als aufwendiges Prompting.                  |
| Überprüfen Sie den Plan vor der Bearbeitung    | Drücken Sie `Shift+Tab`, um in den Plan Mode zu gelangen. Claude wird die beabsichtigten Änderungen zur Genehmigung beschreiben, bevor sie ausgeführt werden.                                                   |
| Lehren Sie es Ihr Repository                   | Führen Sie `/init` aus, um eine `CLAUDE.md`-Datei zu generieren, und fügen Sie dann Ihre Konventionen, Testbefehle und alle Verzeichnisse hinzu, die nicht geändert werden sollten. Siehe [Memory](/docs/de/memory). |
| Verwenden Sie einen Workflow erneut            | Speichern Sie eine `SKILL.md`-Datei in `.claude/skills/<name>/`, um einen `/name`-Skill zu erstellen, den das gesamte Team verwenden kann. Siehe [Skills](/docs/de/skills).                                          |
| Bleiben Sie während langer Aufgaben informiert | Konfigurieren Sie einen Stop-Hook, um eine Desktop-Benachrichtigung zu erhalten, wenn eine lange Aufgabe abgeschlossen ist. Siehe [Hooks](/docs/de/hooks-guide).                                                     |
| Erholen Sie sich von einem falschen Ergebnis   | Anstatt die Anfrage umzuformulieren, fügen Sie den fehlgeschlagenen Test oder Stack Trace an Claude zurück und bitten Sie ihn, diesen spezifischen Fehler zu beheben.                                           |
| Halten Sie Bearbeitungen chirurgisch           | Fragen Sie nach einem Diff, oder geben Sie an „ändere nur X". Claude respektiert den Umfang, wenn der Umfang angegeben ist.                                                                                     |

<Tip>
  Claude Code wird häufig aktualisiert. Überprüfen Sie versionsspezifische Details gegen die [Dokumentationsstartseite](/docs/de/overview), bevor Sie dieses Material intern verteilen.
</Tip>
