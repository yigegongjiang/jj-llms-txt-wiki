> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Cloud-Umgebungen konfigurieren

> Konfigurieren Sie Cloud-Umgebungen für Claude Code Cloud-Sitzungen: Netzwerkzugriffsstufen, Umgebungsvariablen, Setup-Skripte und Umgebungs-Caching.

<Note>
  Cloud-Umgebungen gelten für [Cloud-Sitzungen](/docs/de/claude-code-on-the-web), die auf Pro-, Max- und Team-Plänen verfügbar sind, sowie für Enterprise-Benutzer mit [Premium-Sitzen oder Chat + Claude Code-Sitzen](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan).
</Note>

Jede [Cloud-Sitzung](/docs/de/claude-code-on-the-web) wird in einer Cloud-Umgebung ausgeführt. Sie können eine Umgebung so konfigurieren, dass sie [Netzwerkzugriff](#access-levels) zulässt oder verweigert, [Umgebungsvariablen](#set-environment-variables) für die Sitzung festlegt, auf Pro- und Max-Plänen [API-Anmeldedaten](#add-api-credentials) speichert, die Sitzungen verwenden, ohne sie zu sehen, und ein [Setup-Skript](#setup-scripts) ausführt, bevor Claude mit der Arbeit beginnt.

Die gleichen Umgebungen gelten überall dort, wo Sie eine Cloud-Sitzung starten: die [Desktop-App](/docs/de/desktop), die [Claude Mobile-App](/docs/de/mobile), Ihr Browser unter [claude.ai/code](https://claude.ai/code), das Terminal mit [`claude --cloud`](/docs/de/claude-code-on-the-web#from-terminal-to-cloud), [Routinen](/docs/de/routines) und [Claude Tag](https://claude.com/docs/claude-tag/overview). Jede dieser Oberflächen kann auch zu einer [selbstgehosteten Umgebung](/docs/de/self-hosted-environments) weiterleiten. [Verfügbarkeit und Einschränkungen](/docs/de/self-hosted-environments#availability-and-limitations) behandelt, was Claude noch nicht verwenden kann, wenn eine Claude Tag-Sitzung in einer ausgeführt wird.

<Info>
  [Remote Control](/docs/de/remote-control)-Sitzungen verbinden die Web- und Mobile-Schnittstellen mit einer Sitzung auf Ihrem eigenen Computer, die das Netzwerk und die Dateien Ihres Computers nutzt, nicht eine Cloud-Umgebung. Claude Tag-Kanalsitzungen verwenden nur Umgebungen auf Organisationsebene, entweder [gemeinsame Umgebungen](#organization-shared-environments) oder [selbstgehostete Umgebungen](/docs/de/self-hosted-environments).
</Info>

<h2 id="the-default-environment">
  Die Standard-Umgebung
</h2>

Wenn Sie noch keine Umgebung haben, richtet das Onboarding die **Standard**-Umgebung für Sie ein. Wie hängt davon ab, wo Sie das Onboarding durchführen:

* **CLI-Flüsse wie `/web-setup`**: erstellen **Standard** für Sie
* **Web-Onboarding auf Pro und Max**: erstellt **Standard** für Sie
* **Web-Onboarding auf Team und Enterprise**: zeigt ein Formular **Erstellen Sie Ihre erste Cloud-Umgebung**, es sei denn, ein Eigentümer hat [Schnelles Web-Setup](/docs/de/claude-code-on-the-web#github-authentication-options) aktiviert; behalten Sie die Standardwerte des Formulars bei und klicken Sie auf **Erstellen und fertig**, um die gleiche **Standard**-Umgebung zu erhalten

**Standard** hat keine eigene Konfiguration:

* [**Vertrauenswürdiger** Netzwerkzugriff](#access-levels): Sitzungen erreichen Paketregistrierungen und andere [auf die Whitelist gesetzte Domänen](#default-allowed-domains) und sonst nichts über das Netzwerk der Sitzung.
* Keine andere Konfiguration: **Standard** definiert keine Umgebungsvariablen oder Setup-Skripte, daher starten Sitzungen nur mit den [vorinstallierten Tools](#installed-tools).

Wenn nur **Standard** verfügbar ist, wird jede Sitzung darin ausgeführt. Wenn Sie mehr als eine Umgebung haben, wählen Sitzungen eine pro Oberfläche:

* In der Desktop-App, der Mobile-App und unter claude.ai/code verwenden Sitzungen die im [Selector](#configure-your-environment) angezeigte Umgebung. Ein [Organisations-Standard](#organization-shared-environments), der von einem Eigentümer festgelegt wurde, füllt die Auswahl, wenn Sie noch keine ausgewählt haben. Threads in einem [Projekt](/docs/de/claude-projects#project-settings-reference) verwenden stattdessen die in den Projekteinstellungen festgelegte Umgebung.
* Aus der CLI verwendet Claude Code Ihre [`/remote-env`-Auswahl](#select-an-environment-from-the-cli) oder fällt auf die von Anthropic gehostete Umgebung zurück, wenn Ihre Liste eine hat, und andernfalls auf die erste Umgebung in Ihrer Liste, die keine Bridge-Umgebung ist, ein Eintrag [Remote Control](/docs/de/remote-control) registriert, um Ihren eigenen Computer darzustellen, anstatt eine Cloud-Umgebung. Für eine [selbstgehostete Umgebung](/docs/de/self-hosted-environments) überschreibt das Übergeben von `--environment <environment-id>` mit ihrer `ccpool_`-ID [wenn Sie eine Sitzung versenden](/docs/de/self-hosted-environments-testing#run-the-test-loop) die `/remote-env`-Auswahl und den Fallback für diese Invocation. Claude Code lehnt von Anthropic gehostete `env_`-IDs ab, die an das Flag übergeben werden, daher verwenden Sie `/remote-env`, um diese anzusteuern. Das Flag erfordert Claude Code v2.1.224 oder später.

Konfigurieren Sie eine Umgebung, wenn der Standard nicht ausreicht: wenn Claude Domänen außerhalb der [Standard-Whitelist](#default-allowed-domains) erreichen muss, Umgebungsvariablen für seine Sitzungen benötigt oder Abhängigkeiten installiert werden müssen, bevor es mit der Arbeit beginnt.

<h2 id="configure-your-environment">
  Konfigurieren Sie Ihre Umgebung
</h2>

Erstellen, bearbeiten und archivieren Sie Umgebungen über den Umgebungswähler, den Sie unter [claude.ai/code](https://claude.ai/code) nach dem [Web-Onboarding](/docs/de/web-quickstart) oder über das Eingabefeld in der [Desktop-App](/docs/de/desktop#cloud-sessions) erreichen. Umgebungen, die Sie erstellen, sind persönlich für Ihr Konto; [gemeinsame Umgebungen](#organization-shared-environments), die von einem Owner erstellt wurden, erscheinen im selben Wähler. Siehe [Installierte Tools](#installed-tools) für das, was ohne Konfiguration verfügbar ist.

<Steps>
  <Step title="Öffnen Sie den Umgebungswähler">
    Wählen Sie auf [claude.ai/code](https://claude.ai/code) das Cloud-Symbol aus, das den Namen der aktuellen Umgebung anzeigt, in der Zeile über dem Nachrichtenfeld. Es gibt keine Einstellungsseite oder direkte URL für den Wähler.

    <Frame>
      <img src="https://mintcdn.com/claude-code/ZFId6l95856c5LSw/images/cloud-environment-selector.png?fit=max&auto=format&n=ZFId6l95856c5LSw&q=85&s=cc2813a5664519eaf5a89d793ce5af26" alt="Der Umgebungswähler ist über dem Nachrichtenfeld auf claude.ai/code geöffnet. Die Cloud-Schaltfläche mit dem Umgebungsnamen Default befindet sich in der Zeile über dem Nachrichtenfeld. Das offene Menü zeigt eine Local-Zeile mit den Bezeichnungen Download und Desktop only, einen Cloud-Bereich, in dem die Default-Umgebung mit einem Häkchen ausgewählt ist und beim Hovern ein Einstellungszahnrad anzeigt, eine Option zum Hinzufügen einer Cloud-Umgebung und einen Remote Control-Bereich mit Setupanweisungen." width="1672" height="682" data-path="images/cloud-environment-selector.png" />
    </Frame>
  </Step>

  <Step title="Fügen Sie eine Umgebung hinzu oder bearbeiten Sie sie">
    Wählen Sie **Cloud-Umgebung hinzufügen** oder bewegen Sie den Mauszeiger über eine vorhandene Umgebung und wählen Sie das Einstellungssymbol aus, das auf der rechten Seite angezeigt wird. Der Dialog enthält den Namen, die Netzwerkzugriffsstufe, Umgebungsvariablen und ein Setup-Skript. Wenn Sie eine vorhandene Cloud-Umgebung in einem Pro- oder Max-Plan bearbeiten, enthält der Dialog auch [API-Anmeldedaten](#add-api-credentials).

    <Frame>
      <img src="https://mintcdn.com/claude-code/ZFId6l95856c5LSw/images/cloud-environment-dialog.png?fit=max&auto=format&n=ZFId6l95856c5LSw&q=85&s=30d4478b31d1f879f7ee287ddab32505" alt="Der Dialog Neue Cloud-Umgebung. Ein Namensfeld mit dem Platzhalter Default, ein Netzwerkzugriff-Wähler auf Trusted eingestellt mit Links zur Netzwerkrichtlinie und Zugriffsstufen, ein Umgebungsvariablen-Feld mit .env-Format-Platzhaltertext und einem Hinweis, dass Werte für jeden sichtbar sind, der die Umgebung nutzt, ein Setup-Skript-Feld, das als Bash-Skript beschrieben wird, das ausgeführt wird, wenn eine neue Sitzung startet, bevor Claude Code startet, und Schaltflächen zum Abbrechen und Erstellen der Umgebung." width="874" height="1372" data-path="images/cloud-environment-dialog.png" />
    </Frame>
  </Step>
</Steps>

<h3 id="set-environment-variables">
  Legen Sie Umgebungsvariablen fest
</h3>

Umgebungsvariablen verwenden das `.env`-Format, ein `KEY=value`-Paar pro Zeile. Einfache Werte benötigen keine Anführungszeichen, und wenn Sie einen Wert mit einem passenden Paar in Anführungszeichen setzen, werden die Anführungszeichen nicht Teil des Wertes. Setzen Sie einen Wert in Anführungszeichen, der sich über mehrere Zeilen erstreckt oder ein `#` enthält: in einem Wert ohne Anführungszeichen startet `#` einen Kommentar und der Rest der Zeile wird verworfen.

Das folgende Beispiel definiert drei Variablen.

```text theme={null}
NODE_ENV=development
LOG_LEVEL=debug
DATABASE_URL=postgres://localhost:5432/myapp
```

Jede Sitzung kopiert die Werte der Umgebung einmal beim Start in gewöhnliche Umgebungsvariablen, die jeder Befehl, den Claude ausführt, lesen kann. Da laufende Sitzungen die Konfiguration nicht erneut lesen, wirken sich Änderungen oder das Hinzufügen von Variablen auf Sitzungen aus, die Sie danach starten; Sitzungen, die bereits laufen, behalten die Werte, mit denen sie gestartet wurden.

Eine Cloud-Sitzung setzt auch einige Variablen selbst, wenn sie startet. Für [`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`](/docs/de/claude-code-on-the-web#manage-context) überschreibt der Wert, den die Sitzung setzt, einen, den Sie hier hinzufügen, daher hat das Hinzufügen dieses Schlüssels hier keine Auswirkung.

Jeder, der die Umgebung nutzt, kann die Werte lesen. Verwenden Sie in Pro- und Max-Plänen stattdessen eine [API-Anmeldedaten](#add-api-credentials) für einen Schlüssel, den der Agent-Proxy an eine Anfrage anhängen kann. Die [Anfragen, die niemals eine Anmeldedaten erhalten](#requests-that-never-get-the-credential), sind dort aufgelistet.

<h3 id="add-api-credentials">
  Fügen Sie API-Anmeldedaten hinzu
</h3>

Eine API-Anmeldedaten ist ein API-Schlüssel oder Token, den Sie in einer Cloud-Umgebung speichern, damit Claude diese API aus jeder Sitzung in der Umgebung aufrufen kann, ohne den Schlüssel zu sehen. Der Agent-Proxy von Anthropic fügt den Schlüssel zu Anfragen für die Hosts hinzu, die Sie auflisten, nachdem jede Anfrage die VM der Sitzung verlässt. Der Schlüssel erreicht niemals Claude, die Befehle, die er ausführt, oder die Umgebungsvariablen der Sitzung.

API-Anmeldedaten sind in Pro- und Max-Plänen verfügbar. Sie sind in Team- oder Enterprise-Plänen noch nicht verfügbar, daher erscheint der Bereich **API-Anmeldedaten** im Umgebungsdialog in diesen Plänen nicht.

<h4 id="requirements">
  Anforderungen
</h4>

Zwei davon entscheiden, ob Sie eine Anmeldedaten hinzufügen können, und zwei entscheiden, ob der Agent-Proxy sie nach dem Hinzufügen verwenden kann:

* **Rolle**: eine Organisationsadministrator-Rolle in Ihrer claude.ai-Organisation
  * In Team und Enterprise halten sie Owner und Admins nicht
  * In Pro und Max halten Sie sie in Ihrer eigenen Organisation
  * Ohne sie sehen Sie einen Hinweis statt der Anmeldedatenliste, auch in Ihren eigenen Umgebungen. Bitten Sie einen Owner, die Anmeldedaten zu einer gemeinsamen Umgebung hinzuzufügen und Ihre Sitzungen dort auszuführen
* **Umgebungstyp**: eine von Anthropic gehostete Cloud-Umgebung, die bereits existiert. Eine [selbst gehostete Umgebung](/docs/de/self-hosted-environments) hat keine API-Anmeldedaten
* **API-Erreichbarkeit**: die API akzeptiert Verbindungen aus dem Internet, da Anfragen aus dem Netzwerk von Anthropic ausgehen
* **Verschlüsselungsschlüssel**: wenn Ihre Organisation kundenverwaltete Verschlüsselungsschlüssel verwendet, können Sie keine Anmeldedaten speichern

<h4 id="add-a-credential">
  Fügen Sie eine Anmeldedaten hinzu
</h4>

Sie fügen Anmeldedaten einzeln aus dem Editor einer Umgebung hinzu, die bereits existiert. Der Dialog für eine neue Umgebung bietet sie nicht an. Es gibt auch keine Bearbeitung. Um die Hosts oder den Wert einer Anmeldedaten zu ändern, löschen Sie sie und fügen Sie sie erneut hinzu.

<Steps>
  <Step title="Öffnen Sie die API-Anmeldedaten der Umgebung">
    [Öffnen Sie die Umgebung zur Bearbeitung](#configure-your-environment) unter [claude.ai/code](https://claude.ai/code). Im Dialog **Cloud-Umgebung aktualisieren** finden Sie **API-Anmeldedaten** unter **Umgebungsvariablen**. Sie sehen die Anmeldedaten, die bereits in der Umgebung vorhanden sind, jeweils mit den Hosts, auf die sie sich beziehen.
  </Step>

  <Step title="Fügen Sie die Anmeldedaten hinzu">
    Wählen Sie **Anmeldedaten hinzufügen** und füllen Sie das Formular aus. Behalten Sie den Standard-**Anmeldedatentyp**, **Bearer**, für einen API-Schlüssel, der in einem Request-Header übertragen wird, und füllen Sie diese Felder aus:

    * **Name**: eine Bezeichnung für die Anmeldedaten, wie `Internal billing API`
    * **Zulässige Websites**: die Hosts der API, wie `api.example.com`. Ein führendes `*.` passt zu jeder Subdomain
    * **Benutzerdefinierte Header**: eine Zeile für den Header, der den Schlüssel trägt. Die Zeile beginnt mit `Authorization` als **Name** des Headers und `Bearer` als **Präfix**; fügen Sie den Schlüssel selbst als **Wert** ein. Für einen Header wie `X-Api-Key`, der den bloßen Wert nimmt, ändern Sie den Namen und löschen Sie das Präfix

    Für eine API, die sich anders authentifiziert, wählen Sie einen anderen **Anmeldedatentyp**. Die Liste ist dieselbe, die [Claude Tag](https://claude.com/docs/claude-tag/overview), die Slack-Integration für Team- und Enterprise-Pläne, für [Verbindungen](https://claude.com/docs/claude-tag/admins/add-connections) anbietet.
  </Step>

  <Step title="Speichern Sie die Anmeldedaten">
    Wählen Sie **Verbinden**. Die Anmeldedaten erscheinen in der Liste mit ihren Hosts, gespeichert ohne die Schaltfläche **Änderungen speichern** des Dialogs. Sie können den Wert nach dem Speichern nicht erneut anzeigen.
  </Step>
</Steps>

Um zu bestätigen, dass die Anmeldedaten funktionieren, starten Sie eine Sitzung in der Umgebung und bitten Sie Claude, die API aufzurufen, zum Beispiel mit `curl`. Die API antwortet, als wäre der Schlüssel in der Anfrage, und der Schlüssel erscheint nicht in den Umgebungsvariablen der Sitzung oder in einer Datei. Wenn die Liste eine Anmeldedaten stattdessen als **Nicht gesendet** markiert, sagt der Hinweis darunter, warum und was zu tun ist. Zwei Anmeldedaten, deren Hosts sich überlappen, ohne genau übereinzustimmen, erhalten keine Markierung, und der Agent-Proxy sendet nur eine davon.

<h4 id="which-requests-get-the-credential">
  Welche Anfragen erhalten die Anmeldedaten
</h4>

Der Agent-Proxy fügt eine Anmeldedaten an eine Anfrage an, wenn der Host der Anfrage mit einem übereinstimmt, den Sie auf dieser Anmeldedaten aufgelistet haben. Sitzungen können diese Hosts erreichen, auch wenn die [Netzwerkzugriffsstufe](#access-levels) der Umgebung dies sonst nicht zulassen würde, außer den [Hosts, die niemals die Anmeldedaten erhalten](#requests-that-never-get-the-credential). Die Anmeldedaten gelten in jeder Sitzung, die in der Umgebung ausgeführt wird, wer sie auch gestartet hat, bis Sie sie löschen.

<h4 id="requests-that-never-get-the-credential">
  Anfragen, die niemals die Anmeldedaten erhalten
</h4>

Der Agent-Proxy fügt niemals eine Anmeldedaten, die Sie hinzufügen, zu diesen Anfragen hinzu:

* **GitHub**: der [GitHub-Proxy](#github-proxy) authentifiziert stattdessen Anfragen an GitHub, daher benötigen Sie keine API-Anmeldedaten dafür
* **Die Anthropic API und öffentliche Paketregistrierungen**: `api.anthropic.com`, `registry.npmjs.org`, `jsr.io`, `npm.jsr.io`, `pypi.org`, `files.pythonhosted.org`, `index.crates.io` und `proxy.golang.org`
* **Setup-Skript-Anfragen**: Claude Code verbindet sich mit dem Agent-Proxy, wenn er startet, nachdem das [Setup-Skript](#setup-scripts) ausgeführt wurde

<h3 id="select-an-environment-from-the-cli">
  Wählen Sie eine Umgebung aus der CLI
</h3>

Führen Sie `/remote-env` in Ihrem Terminal aus, um die Standardumgebung für Cloud-Sitzungen auszuwählen, die Sie aus der CLI erstellen, wie [`claude --cloud`](/docs/de/claude-code-on-the-web#from-terminal-to-cloud). Der Befehl öffnet einen Picker Ihrer vorhandenen Umgebungen und speichert Ihre Auswahl im Schlüssel `remote.defaultEnvironmentId` in Ihren [Benutzereinstellungen](/docs/de/settings#where-settings-live), damit er in jedem Projekt auf Ihrem Computer gilt, bis Sie ihn ändern, es sei denn, derselbe Schlüssel ist in einer höheren Prioritäts-[Einstellungsebene](/docs/de/settings#settings-precedence) gesetzt, wie den Projekteinstellungen eines Repos.

Eine [selbst gehostete Umgebung](/docs/de/self-hosted-environments) ID, die die Form `ccpool_...` hat, folgt einer strengeren Quellregel. Siehe [`remote.defaultEnvironmentId`](/docs/de/settings-reference#remote-defaultenvironmentid) für die Einstellungsebenen, die Claude Code dafür berücksichtigt.

`/remote-env` setzt nur den Standard: es startet keine Sitzung und kann keine Umgebungen hinzufügen oder bearbeiten. Verwalten Sie sie aus dem [Umgebungswähler](#configure-your-environment).

<h3 id="archive-an-environment">
  Archivieren Sie eine Umgebung
</h3>

Um eine Ihrer eigenen Umgebungen zu archivieren, öffnen Sie sie zur Bearbeitung und wählen Sie **Archivieren**. Ein Owner archiviert eine [gemeinsame Umgebung](#organization-shared-environments) von der Seite **Cloud-Umgebungen** in den Admin-Einstellungen. Sie können eine Umgebung nicht löschen, nur archivieren.

Das Archivieren wirkt sich auf neue Sitzungen aus, nicht auf laufende:

* Sitzungen, die bereits in der Umgebung laufen, funktionieren weiterhin.
* Die Umgebung verschwindet aus dem Wähler und aus `/remote-env`, daher können Sie sie nicht für neue Sitzungen auswählen.
* API-Anmeldedaten in der Umgebung bleiben in ihren laufenden Sitzungen angehängt. Löschen Sie alle, die Sie nicht mehr benötigen, bevor Sie archivieren.
* Keine neue Sitzung kann in einer archivierten Umgebung starten, auf keiner Oberfläche. Wenn die Umgebung Ihr gespeicherter [CLI-Standard](#select-an-environment-from-the-cli) war, startet Claude Code CLI-Cloud-Sitzungen in der von Anthropic gehosteten Umgebung, wenn Ihre Liste eine hat, und ansonsten in der ersten Umgebung in Ihrer Liste, die keine [Remote Control-Bridge-Umgebung](#the-default-environment) ist. Alles, das explizit mit der Umgebung konfiguriert ist, wie eine [Routine](/docs/de/routines#environments-and-network-access), kann keine neuen Sitzungen darin starten. Zeigen Sie es auf eine andere Umgebung.

<h3 id="organization-shared-environments">
  Gemeinsame Organisationsumgebungen
</h3>

In Team- und Enterprise-Plänen kann ein Owner Cloud-Umgebungen erstellen, die mit jedem Mitglied der Organisation geteilt werden. Dieselbe Rolle verwaltet alles andere auf der Seite **Cloud-Umgebungen** in den Admin-Einstellungen, einschließlich [selbst gehosteter Umgebungen](/docs/de/self-hosted-environments); die Admin-Rolle kann die Seite nicht öffnen. Die vollständige Liste der Rollen, die die Seite öffnen können, ist die für [Verwaltung von Server-verwalteten Einstellungen](/docs/de/server-managed-settings#access-control).

Gemeinsame Umgebungen erscheinen im [Umgebungswähler](#organization-shared-environments) jedes Mitglieds unter einer **Organisation**-Überschrift, nach den eigenen Umgebungen des Mitglieds unter **Persönlich**, damit ein Team sich auf eine Konfiguration standardisieren kann, statt dass jedes Mitglied sie neu erstellt. Das Auswählen des Einstellungssymbols einer gemeinsamen Umgebung dort öffnet eine schreibgeschützte Zusammenfassung ihrer Konfiguration für jedes Mitglied, einschließlich Owner.

Ein Owner macht eine Umgebung der Organisation auf eine von zwei Arten verfügbar:

* **Erstellen Sie eine gemeinsame Umgebung**: verwenden Sie die Seite **Cloud-Umgebungen** in den [Admin-Einstellungen](https://claude.ai/admin-settings), wo Owner auch gemeinsame Umgebungen bearbeiten und archivieren. Jede hat einen Namen, eine [Netzwerkzugriffsstufe](#access-levels), [Umgebungsvariablen](#set-environment-variables) im `.env`-Format und ein [Setup-Skript](#setup-scripts).
* **Teilen Sie eine persönliche Umgebung**: öffnen Sie eine Ihrer eigenen Umgebungen zur Bearbeitung im Umgebungswähler, dann teilen Sie sie aus der Zeile **Wer kann sie verwenden**. Die Umgebung behält ihre ID, daher sind Sitzungen und Routinen, die sie bereits verwenden, nicht betroffen, und jedes Mitglied kann sie dann sehen und Sitzungen darin starten.

Owner wählen die [Standardumgebung](#the-default-environment) der Organisation separat unter [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code).

Die Sitzungen jedes Mitglieds in einer gemeinsamen Umgebung lesen ihre Variablen, daher fügen Sie keine Geheimnisse darin ein. [API-Anmeldedaten](#add-api-credentials), die Sitzungen einen Schlüssel geben, den sie nicht lesen können, sind in Team- oder Enterprise-Plänen noch nicht verfügbar.

<h3 id="set-the-environment-a-claude-tag-channel-uses">
  Legen Sie die Umgebung fest, die ein Claude Tag-Kanal verwendet
</h3>

In [Claude Tag](https://claude.com/docs/claude-tag/overview)-Kanälen arbeitet Claude als die gemeinsame Identität Ihrer Organisation, nicht als ein Mitglied, daher verwenden Kanalsitzungen nur Umgebungen auf Organisationsebene, entweder gemeinsame Umgebungen oder [selbst gehostete Umgebungen](/docs/de/self-hosted-environments). Um einem Kanal eine Toolchain zu geben, die nicht [vorinstalliert](#installed-tools) ist, wie .NET, kann ein Owner eine [gemeinsame Umgebung](#organization-shared-environments) von der Seite **Cloud-Umgebungen** in den Admin-Einstellungen mit einem [Setup-Skript](#setup-scripts) erstellen, das sie installiert. Zeigen Sie den Kanal auf eine Umgebung auf eine von zwei Arten:

* Legen Sie eine gemeinsame oder selbst gehostete Umgebung als die [Standardumgebung](#the-default-environment) der Organisation unter [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) fest.
* [Heften Sie eine an einen Kanal](https://claude.com/docs/claude-tag/admins/troubleshooting#channel-sessions-use-the-wrong-environment-or-can%E2%80%99t-find-one) in den Claude Tag-Admin-Einstellungen.

<h2 id="network-access">
  Netzwerkzugriff
</h2>

Jede Umgebung setzt eine Netzwerkzugriffsstufe, die die ausgehenden Verbindungen steuert, die ihre Sitzungen herstellen können. Die Standard-Stufe, **Trusted**, erlaubt Paketregistrierungen und andere [auf die Whitelist gesetzte Domänen](#default-allowed-domains); **Custom** nimmt Ihre eigene Domänenliste.

Um die Netzwerkzugriffsstufe einer Umgebung zu ändern, [öffnen Sie sie zur Bearbeitung](#configure-your-environment) und verwenden Sie den **Network access**-Selector im Dialog. Eine [gemeinsame Umgebung](#organization-shared-environments) öffnet sich dort schreibgeschützt, daher ändert ein Owner seinen Netzwerkzugriff stattdessen von der Seite **Cloud environments** in den [Admin-Einstellungen](https://claude.ai/admin-settings). Das Cloud-Symbol, das den Selector öffnet, erscheint auf den App-Oberflächen, die unter [Die Standard-Umgebung](#the-default-environment) aufgelistet sind, und im [Routine-Editor](/docs/de/routines#environments-and-network-access); persönliche Umgebungen haben keine separate Seite in Ihren claude.ai-Kontoeinstellungen.

<Note>
  MCP-Konnektoren, die Sie auf einer Sitzung oder Routine aktivieren, funktionieren, ohne ihre Hosts zu **Allowed domains** hinzuzufügen, da der Konnektoren-Verkehr über Anthropic-Server verläuft, anstatt über das Netzwerk der Sitzung. Dies beruht auf dem gleichen Anthropic-gebundenen Kanal, der unter [Security and isolation](/docs/de/claude-code-on-the-web#security-and-isolation) erwähnt wird. Schalten Sie alle Konnektoren aus, die Sie nicht benötigen, um zu begrenzen, welche Tools Claude erreichen kann.
</Note>

<h3 id="access-levels">
  Zugriffsstufen
</h3>

Das Feld **Network access** im [Umgebungs-Dialog](#configure-your-environment) nimmt eine von vier Stufen:

| Stufe       | Ausgehende Verbindungen                                                                                      |
| :---------- | :----------------------------------------------------------------------------------------------------------- |
| **None**    | Kein ausgehender Netzwerkzugriff über das Netzwerk der Sitzung                                               |
| **Trusted** | Nur [auf die Whitelist gesetzte Domänen](#default-allowed-domains): Paketregistrierungen, GitHub, Cloud-SDKs |
| **Full**    | Jede Domäne                                                                                                  |
| **Custom**  | Ihre eigene Whitelist, optional einschließlich der Standards                                                 |

Welche Stufe Sie auch wählen, Sitzungen können diese immer noch erreichen, da jede einen Pfad nimmt, der nicht durch die Netzwerk-Whitelist der Sitzung geht:

* GitHub, durch seinen [separaten Proxy](#github-proxy)
* [MCP-Konnektoren](#network-access), die Sie aktivieren, deren Verkehr über Anthropic-Server verläuft
* Die Hosts, die Sie auf den [API-Anmeldedaten](#add-api-credentials) der Umgebung aufgelistet haben, außer den [Hosts, die der Agent-Proxy überspringt](#requests-that-never-get-the-credential)
* Die Anthropic-API, für Claude Code-eigene Anfragen, auch bei **None**, wie unter [Security and isolation](/docs/de/claude-code-on-the-web#security-and-isolation) erwähnt

<h3 id="allow-specific-domains">
  Erlauben Sie bestimmte Domänen
</h3>

Um Domänen zu erlauben, die nicht in der Trusted-Liste sind, wählen Sie **Custom** in den Netzwerkzugriff-Einstellungen der Umgebung, dann listen Sie eine Domäne pro Zeile im Feld **Allowed domains** auf. Dieses Beispiel erlaubt drei Hosts, die ein internes Projekt benötigen könnte.

```text theme={null}
api.example.com
*.internal.example.com
registry.example.com
```

Sitzungen in dieser Umgebung können jetzt `api.example.com`, jede Subdomain von `internal.example.com` und `registry.example.com` erreichen, und keine anderen Domänen über das Netzwerk der Sitzung. [GitHub-Verkehr](#github-proxy), [MCP-Konnektoren-Verkehr](#network-access) und Anfragen an die Hosts der [API-Anmeldedaten](#add-api-credentials) der Umgebung, außer den [Hosts, die der Agent-Proxy überspringt](#requests-that-never-get-the-credential), gehen nicht durch diese Whitelist. Ein führendes `*.` passt zu jeder Subdomain. Um die [Trusted-Domänen](#default-allowed-domains) auch zu behalten, aktivieren Sie **Also include default list of common package managers**; lassen Sie es deaktiviert, um nur das zu erlauben, was Sie auflisten.

Wenn Ihre Organisation [Artifacts](/docs/de/artifacts#availability) verwendet, benötigen Sie `*.frame.claudeusercontent.com` nicht in der Liste, damit Sitzungen sie lesen können. Wenn die Liste diesen Host auslässt, liest Claude Code Artifact-Inhalte stattdessen über die Verbindung der Sitzung zu Anthropic. Behalten Sie den Host in einer Whitelist in zwei Situationen:

* **Sitzungen in dieser Umgebung öffnen öffentliche Artifacts einer anderen Organisation**: Claude Code ruft diese direkt vom Host ab, daher fügen Sie ihn zu dieser Liste hinzu.
* **Sie konfigurieren die lokale CLI oder einen selbstgehosteten Runner**: behalten Sie den Host in dieser Whitelist. Siehe [Netzwerkzugriff-Anforderungen](/docs/de/network-config#network-access-requirements) und die selbstgehosteten [Netzwerk-Anforderungen](/docs/de/self-hosted-environments-deploy#network-requirements).

Jede Umgebung hat ihre eigene Whitelist für zulässige Domänen; es gibt keine Organisations-Whitelist, die Administratoren an die Umgebungen jedes Mitglieds pushen können. [Server-verwaltete Einstellungen](/docs/de/server-managed-settings) gelten immer noch in Cloud-Sitzungen, aber keine von ihnen fügt Domänen zur Netzwerk-Whitelist der Umgebung hinzu. Um einem Team eine Standard-Liste zu geben, kann ein Owner eine [organisationsweite gemeinsame Umgebung](#organization-shared-environments) mit **Custom**-Netzwerkzugriff und dieser Liste erstellen.

<h3 id="github-proxy">
  GitHub-Proxy
</h3>

In von Anthropic gehosteten Umgebungen gehen alle GitHub-Operationen durch einen dedizierten Proxy, der Ihre echten GitHub-Anmeldedaten außerhalb der Sitzungs-VM hält, unabhängig von der [Zugriffsstufe](#access-levels) der Umgebung. Sitzungen in einer selbstgehosteten Umgebung authentifizieren Git-Operationen mit Anmeldedaten, die Ihre Bereitstellung bereitstellt; [Configure git](/docs/de/self-hosted-environments-deploy#configure-git) behandelt die Optionen, einschließlich pro-Sitzung geprägter Anmeldedaten und eines Opt-in zu diesem gleichen Proxy. Der Proxy bietet:

* **Git-Anmeldedaten**: Der Git-Client in der VM verwendet eine begrenzte Anmeldedaten, die der Proxy überprüft und gegen Ihren echten GitHub-Token austauscht.
* **API-Anfragen**: Anfragen von den integrierten GitHub-Tools und von `gh` unter dem [`proxy-injected`-Platzhalter](#work-with-github-issues-and-pull-requests) gehen mit Ihren echten Anmeldedaten aus.
* **Push-Schutz**: `git push` funktioniert nur gegen den aktuellen Arbeitszweig der Sitzung; Klonen, Abrufen und PR-Operationen funktionieren normal.
* **Repository-Bereich**: GitHub-API und Release-Asset-Anfragen erreichen nur Repositories, die an die Sitzung angehängt sind, daher erhält ein Setup-Skript, das Release-Assets aus einem nicht angehängten Repository herunterlädt, einen 403.
* **GraphQL-Einschränkungen**: der Proxy bedient nur einen angehefteten Satz von GraphQL-Operationen für Pull-Request-Workflows. Der Proxy lehnt alles andere auf dem GraphQL-Endpunkt mit einem 403 ab, der sagt `This GraphQL query is not enabled for this session` und nennt den REST-Fallback, `gh api repos/{owner}/{repo}/...`. Die Einschränkung gilt für jede Anfrage durch den Proxy, unabhängig von den Anmeldedaten, die Sie bereitstellen, daher erhält ein `GH_TOKEN`, den Sie setzen, den gleichen 403. Claude kann GitHub-APIs, die nur in GraphQL existieren, wie Projects v2, nicht durch den Proxy erreichen.

Committed-Dateien aus öffentlichen Repositories kommen über `raw.githubusercontent.com` an, das der [Sicherheits-Proxy](#security-proxy) stattdessen handhabt. Diese Domäne ist in der Standard-[Trusted-Liste](#default-allowed-domains), daher bleiben diese Dateien erreichbar, es sei denn, die [Zugriffsstufe](#access-levels) der Umgebung schließt sie aus.

<h3 id="security-proxy">
  Sicherheits-Proxy
</h3>

Cloud-Sitzungen in von Anthropic gehosteten Umgebungen laufen hinter einem HTTP/HTTPS-Netzwerk-Proxy für Sicherheits- und Missbrauchspräventionszwecke; in einer [selbstgehosteten Umgebung](/docs/de/self-hosted-environments-deploy#default-deny-egress) verläuft ausgehender Verkehr stattdessen durch Ihre eigene Netzwerk-Grenze. Der gesamte ausgehende Internet-Verkehr aus einer von Anthropic gehosteten Sitzung verläuft durch diesen Proxy, der Folgendes bietet:

* Schutz vor böswilligen Anfragen
* Ratenbegrenzung und Missbrauchsprävention
* Inhaltsfilterung für erhöhte Sicherheit
* Ein DNS-Audit-Trail der angeforderten Hostnamen

<h2 id="what’s-available-in-cloud-sessions">
  Was ist in Cloud-Sitzungen verfügbar
</h2>

In von Anthropic gehosteten Umgebungen erhält jede Sitzung eine frische virtuelle Maschine (VM) mit Ubuntu 24.04 auf x86\_64, unabhängig von Ihrem eigenen Betriebssystem und CPU-Architektur, mit Ihrem geklonten Repository und vorinstallierten gängigen Toolchains. Wenn eine Abhängigkeit vorkompilierte Binärdateien bereitstellt, wie Ruby-Gems mit nativen Erweiterungen oder vorgefertigte Python-Wheels, verwenden Sie seinen x86\_64 Linux-Build, um die VM zu entsprechen. Dieser Abschnitt behandelt die von Anthropic gehosteten Standardeinstellungen, die integrierten GitHub-Tools, wie man [Tests und Services ausführt](#run-tests-start-services-and-add-packages) und die [Ressourcenlimits](#resource-limits), die jede VM erhält.

<Note>
  Sitzungen, die Ihre Organisation zu einer [selbstgehosteten Umgebung](/docs/de/self-hosted-environments) leitet, laufen stattdessen auf Ihren eigenen Runnern mit den Tools, die Ihr Runner-Image bereitstellt.
</Note>

<h3 id="what-carries-over-from-your-setup">
  Was wird von Ihrem Setup übernommen
</h3>

Cloud-Sitzungen starten aus einem frischen Klon Ihres Repositories. Alles, das Sie in das Repo committen, ist verfügbar. Alles, das Sie nur auf Ihrem eigenen Computer installiert oder konfiguriert haben, ist nicht in der Sitzung verfügbar. Die Richtlinie Ihrer Organisation kommt separat über [Server-verwaltete Einstellungen](/docs/de/server-managed-settings) an.

|                                                                                                                                                                                          | Verfügbar in Cloud-Sitzungen                                          | Warum                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Die `CLAUDE.md` Ihres Repos                                                                                                                                                              | Ja                                                                    | Teil des Klons                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Die `.claude/settings.json`-Hooks und Berechtigungsregeln Ihres Repos                                                                                                                    | Ja, in einer Sitzung mit einem Repository                             | Teil des Klons. Eine Sitzung mit mehreren Repositories, einschließlich eines [Projekt](/docs/de/claude-projects#what-threads-pick-up-from-your-repositories)-Threads, startet über den Klonen und liest sie nicht                                                                                                                                                                                                                                                                                                                                                                                                                         |
| Die `.mcp.json` MCP-Server Ihres Repos                                                                                                                                                   | Ja, in einer Sitzung mit einem Repository                             | Teil des Klons, gefunden aus dem Arbeitsverzeichnis der Sitzung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| Das `.claude/rules/` Ihres Repos                                                                                                                                                         | Ja                                                                    | Teil des Klons                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Die `.claude/skills/`, `.claude/agents/`, `.claude/commands/` Ihres Repos                                                                                                                | Ja                                                                    | Teil des Klons                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Plugins und Marketplaces, die in der `.claude/settings.json` Ihres Repos deklariert sind                                                                                                 | Nein                                                                  | Eine Cloud-Sitzung installiert nicht die Plugins, die ein Repository unter [`enabledPlugins`](/docs/de/settings-reference#enabledplugins) aktiviert, einschließlich solcher aus den Marketplaces, die es unter [`extraKnownMarketplaces`](/docs/de/settings-reference#extraknownmarketplaces) auflistet                                                                                                                                                                                                                                                                                                                                        |
| Die [Server-verwalteten Einstellungen](/docs/de/server-managed-settings) Ihrer Organisation                                                                                                   | Ja                                                                    | Abgerufen von Anthropic-Servern, wenn die Sitzung startet. Siehe [Surface coverage](/docs/de/model-config#surface-coverage) für die Durchsetzung von `availableModels` in Cloud-Sitzungen. Einstellungen, die auf Ihrem Gerät über MDM oder verwaltete Einstellungsdateien bereitgestellt werden, gelten nicht, da die Sitzung auf einer von Anthropic verwalteten VM läuft; in einer [selbstgehosteten Umgebung](/docs/de/self-hosted-environments) lesen Sitzungen auch die verwaltete Einstellungsdatei im Runner-Image, pro [wie Claude Code verwaltete Quellen kombiniert](/docs/de/managed-settings#how-claude-code-combines-managed-sources) |
| Ihre Benutzer `~/.claude/CLAUDE.md`                                                                                                                                                      | Nein                                                                  | Lebt auf Ihrem Computer, nicht im Repo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| Ihre Benutzer `~/.claude/skills/`, `~/.claude/agents/`, `~/.claude/commands/`                                                                                                            | Nein                                                                  | Leben auf Ihrem Computer, nicht im Repo. Committen Sie sie stattdessen in das Verzeichnis `.claude/` des Repos. Cloud-Sitzungen laden automatisch Skills, die Sie auf claude.ai aktivieren                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Plugins, die nur in Ihren Benutzereinstellungen aktiviert sind                                                                                                                           | Nein                                                                  | Benutzer-scoped `enabledPlugins` lebt in `~/.claude/settings.json` auf Ihrem Computer                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| MCP-Server, die Sie mit `claude mcp add` im Standard-lokalen Bereich oder Benutzerbereich hinzugefügt haben                                                                              | Nein                                                                  | Diese schreiben in `~/.claude.json` auf Ihrem Computer, nicht im Repo. Fügen Sie den Server mit `claude mcp add --scope project` hinzu, das die [`.mcp.json`](/docs/de/mcp#project-scope) des Repos schreibt, und committen Sie diese Datei. Eine Sitzung mit einem Repository lädt sie                                                                                                                                                                                                                                                                                                                                                   |
| Transport-Variablen in der `.claude/settings.json` `env`-Block Ihres Repos, wie `NODE_EXTRA_CA_CERTS` und die [mTLS-Client-Zertifikat-Variablen](/docs/de/network-config#mtls-authentication) | Nein                                                                  | Die Hosting-Umgebung verwaltet die API-Verbindung der Sitzung, daher ignoriert Claude Code diese Schlüssel und notiert jeden ignorierten Schlüssel im Debug-Log der Sitzung                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| API-Schlüssel und Tokens für Services, die Claude aufruft                                                                                                                                | Auf Pro- und Max-Plänen, als [API-Anmeldedaten](#add-api-credentials) | Sie fügen den Schlüssel einmal auf der Umgebung hinzu und der Agent-Proxy fügt ihn an Anfragen für die Hosts an, die Sie auflisten. Ein Schlüssel, den der Agent-Proxy [nicht anhängen kann](#requests-that-never-get-the-credential), oder ein beliebiger Schlüssel auf einem Team- oder Enterprise-Plan, bleibt in einer Umgebungsvariable                                                                                                                                                                                                                                                                                         |
| Interaktive Authentifizierung wie AWS SSO                                                                                                                                                | Nein                                                                  | Nicht unterstützt. SSO erfordert browserbasierte Anmeldung, die nicht in einer Cloud-Sitzung ausgeführt werden kann                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

Um Ihre eigene Konfiguration in Cloud-Sitzungen verfügbar zu machen, committen Sie sie in das Repo.

Jeder, der die Umgebung nutzt, kann ihre Umgebungsvariablen und Setup-Skripte lesen. Der Hinweis des Dialogs unter **Umgebungsvariablen** sagt dies und warnt vor dem Hinzufügen von Geheimnissen. Auf Pro- und Max-Plänen speichern Sie einen Schlüssel, den der Agent-Proxy anhängen kann, als [API-Anmeldedaten](#add-api-credentials) stattdessen.

<h3 id="installed-tools">
  Installierte Tools
</h3>

Cloud-Sitzungen werden mit gängigen Sprach-Runtimes, Build-Tools und Datenbanken vorinstalliert geliefert. Die folgende Tabelle fasst zusammen, was nach Kategorie enthalten ist.

| Kategorie       | Enthalten                                                                |
| :-------------- | :----------------------------------------------------------------------- |
| **Python**      | Python 3.x mit pip, poetry, uv, black, mypy, pytest, ruff                |
| **Node.js**     | 20, 21 und 22, mit npm, yarn, pnpm, bun¹, eslint, prettier, chromedriver |
| **Ruby**        | 3.1, 3.2, 3.3 mit gem, bundler, rbenv                                    |
| **PHP**         | 8.3 mit Composer                                                         |
| **Java**        | OpenJDK 21 mit Maven und Gradle                                          |
| **Go**          | Go mit Modul-Unterstützung                                               |
| **Rust**        | rustc und cargo                                                          |
| **C/C++**       | GCC, Clang, cmake, ninja, conan                                          |
| **Docker**      | docker, dockerd, docker compose                                          |
| **Datenbanken** | PostgreSQL 16, Redis 7.0                                                 |
| **Utilities**   | git, gh, jq, yq, ripgrep, tmux, vim, nano                                |

¹ Bun ist installiert, hat aber bekannte [Proxy-Kompatibilitätsprobleme](#install-dependencies-with-a-sessionstart-hook) beim Paket-Abrufen.

Um die Versionen der meisten Tools in dieser Tabelle zu erhalten, bitten Sie Claude, `check-tools` in einer Cloud-Sitzung auszuführen. Es ist ein Shell-Befehl, der auf der Sitzungs-VM installiert ist, kein Befehl, den Sie mit `/` eingeben; Sie bitten Claude, weil [Claude alle VM-Befehle für Sie ausführt](#run-tests-start-services-and-add-packages). Für ein Tool, das es nicht meldet, wie Ruby, PHP, bun, PostgreSQL oder Redis, bitten Sie Claude, den Versions-Befehl des Tools selbst auszuführen, zum Beispiel `psql --version`.

Node.js-Versionen sind unter `/opt/node20`, `/opt/node21` und `/opt/node22` installiert, mit 22 auf `PATH` standardmäßig. Um mit einer anderen Version zu arbeiten, bitten Sie Claude, das `bin`-Verzeichnis dieser Version, wie `/opt/node20/bin`, zu `PATH` voranstellen.

Toolchains außerhalb dieser Liste, wie das .NET SDK, sind nicht vorinstalliert, auch wenn ihre Paketregistrierungen auf der [Standard-Whitelist](#default-allowed-domains) sind. Installieren Sie sie mit einem [Setup-Skript](#setup-scripts).

<h3 id="work-with-github-issues-and-pull-requests">
  Arbeiten Sie mit GitHub-Issues und Pull Requests
</h3>

Cloud-Sitzungen enthalten integrierte GitHub-Tools, die Claude Issues lesen, Pull Requests auflisten, Diffs abrufen und Kommentare posten lassen, ohne Setup. Diese Tools authentifizieren sich über den [GitHub-Proxy](#github-proxy) mit der Methode, die Sie unter [GitHub-Authentifizierungsoptionen](/docs/de/claude-code-on-the-web#github-authentication-options) konfiguriert haben, daher betritt Ihr Token niemals den Container.

Sie können `GH_TOKEN` oder `GITHUB_TOKEN` selbst in [Umgebungseinstellungen](#set-environment-variables) setzen, oder beide ungesetzt lassen und den [GitHub-Proxy](#github-proxy) für Sie authentifizieren lassen:

* Wenn Sie einen Token setzen, wird er unverändert an den Container übergeben, daher verwenden Ihre Skripte und GitHub's [`gh` CLI](https://cli.github.com) ihn direkt.
* Wenn Sie keinen setzen und der [GitHub-Proxy](#github-proxy) die Authentifizierung für Ihre Sitzung handhabt, lesen beide Variablen als die Platzhalter-Zeichenkette `proxy-injected` in den Befehlen, die Claude ausführt, und der Proxy ersetzt Ihre echten Anmeldedaten bei ausgehenden GitHub-Anfragen. `gh` funktioniert ohne einen Token von Ihnen, aber ein Skript, das `GITHUB_TOKEN` direkt liest, erhält den Platzhalter, nicht einen verwendbaren Token.

Ein Token, den Sie setzen, ist eine gewöhnliche Umgebungsvariable, daher kann jeder, der die Umgebung nutzt, ihn lesen; der Proxy-Pfad hält die Anmeldedaten aus der Umgebungskonfiguration und der Sitzungs-VM.

Um zu überprüfen, welcher Fall auf Ihre Sitzung zutrifft, bitten Sie Claude, `echo $GH_TOKEN` auszuführen.

GitHub's [`gh` CLI](https://cli.github.com) ist vorinstalliert. Wenn Sie einen `gh`-Befehl benötigen, den die integrierten Tools nicht abdecken, wie `gh release` oder `gh workflow run`, bitten Sie Claude, ihn auszuführen. `gh` liest `GH_TOKEN` automatisch, daher müssen Sie `gh auth login` nicht ausführen.

<h3 id="link-output-back-to-the-session">
  Verknüpfen Sie die Ausgabe zurück zur Sitzung
</h3>

Jede Cloud-Sitzung hat eine Transkript-URL auf claude.ai, und die Sitzung kann ihre eigene ID aus der Umgebungsvariable `CLAUDE_CODE_REMOTE_SESSION_ID` lesen. Verwenden Sie dies, um einen nachverfolgbaren Link in PR-Bodies, Commit-Nachrichten, Slack-Posts oder generierten Berichten zu platzieren, damit ein Reviewer den Lauf öffnen kann, der sie produziert hat.

Commits, die Claude in einer Cloud-Sitzung erstellt, enthalten einen `Claude-Session: <url>` Git-Trailer, und PR-Bodies enthalten die Sitzungs-URL auf ihrer eigenen Zeile. Um den Trailer und den PR-Body-Link zu weglassen, setzen Sie [`attribution.sessionUrl`](/docs/de/settings-reference#attribution-sessionurl) auf `false`.

Um den Sitzungs-Link in etwas anderem als einem Commit oder PR einzuschließen, wie eine Slack-Nachricht, die Claude postet, oder eine Berichtsdatei, die sie schreibt, lassen Sie Claude den folgenden Befehl ausführen und verwenden Sie seine Ausgabe. Der Befehl konvertiert das `cse_`-Präfix im Wert der Umgebungsvariable in das `session_`-Präfix, das die Transkript-URL erwartet:

```bash theme={null}
echo "https://claude.ai/code/${CLAUDE_CODE_REMOTE_SESSION_ID/#cse_/session_}"
```

<h3 id="run-tests-start-services-and-add-packages">
  Führen Sie Tests aus, starten Sie Services und fügen Sie Pakete hinzu
</h3>

Sie erhalten keine Shell in die Sitzungs-VM. Claude führt jeden Befehl für Sie aus, daher formulieren Sie die Aufgaben in diesem Abschnitt als Anfragen in Ihrem Prompt.

<h4 id="run-tests">
  Führen Sie Tests aus
</h4>

Claude führt Tests als Teil der Arbeit an einer Aufgabe aus. Bitten Sie darum in Ihrem Prompt, wie „Beheben Sie die fehlgeschlagenen Tests in `tests/`" oder „Führen Sie pytest nach jeder Änderung aus." Test-Runner, die mit den [vorinstallierten Toolchains](#installed-tools) kommen, wie pytest und cargo test, funktionieren ohne zusätzliches Setup. Ein Runner, den Ihr Projekt als Abhängigkeit deklariert, wie jest, installiert sich mit Ihren Abhängigkeiten.

<h4 id="start-services">
  Starten Sie Services
</h4>

PostgreSQL und Redis sind vorinstalliert, aber nicht standardmäßig laufen. Bitten Sie Claude, diejenigen zu starten, die Sie benötigen; die Befehle, die es ausführt, sind:

```bash theme={null}
service postgresql start
```

```bash theme={null}
service redis-server start
```

Docker ist für die Ausführung von containerisierten Services verfügbar. Bitten Sie Claude, `docker compose up` auszuführen, um die Services Ihres Projekts zu starten. Der Netzwerkzugriff zum Abrufen von Images folgt der [Zugriffsstufe](#access-levels) Ihrer Umgebung, und die [Trusted-Standardeinstellungen](#default-allowed-domains) enthalten Docker Hub und andere gängige Registrierungen.

Wenn Ihre Images groß oder langsam zum Abrufen sind, fügen Sie `docker compose pull` oder `docker compose build` zu Ihrem [Setup-Skript](#setup-scripts) hinzu. Der [Umgebungs-Cache](#environment-caching) behält die abgerufenen Images, daher hat jede neue Sitzung sie auf der Festplatte. Der Cache speichert nur Dateien, keine laufenden Prozesse, daher startet Claude die Container immer noch jede Sitzung.

<h4 id="add-packages">
  Fügen Sie Pakete hinzu
</h4>

Um Pakete hinzuzufügen, die nicht vorinstalliert sind, verwenden Sie ein [Setup-Skript](#setup-scripts). Der [Umgebungs-Cache](#environment-caching) behält das, was das Skript installiert, daher sind Pakete, die Sie dort installieren, am Anfang jeder Sitzung verfügbar, ohne jedes Mal neu zu installieren. Sie können Claude auch bitten, Pakete mid-Sitzung zu installieren, aber diese Installationen werden nicht auf andere Sitzungen übertragen.

<h3 id="resource-limits">
  Ressourcenlimits
</h3>

Cloud-Sitzungen in von Anthropic gehosteten Umgebungen laufen mit ungefähren Ressourcen-Obergrenzen, die sich im Laufe der Zeit ändern können:

* 4 vCPUs
* 16 GB RAM
* 30 GB Festplatte

Die VM kann Aufgaben stoppen, die erheblich mehr Speicher benötigen, wie große Build-Jobs oder speicherintensive Tests. Für Workloads jenseits dieser Limits verwenden Sie [Remote Control](/docs/de/remote-control), um Claude Code auf Ihrer eigenen Hardware auszuführen, oder führen Sie Cloud-Sitzungen in einer [selbstgehosteten Umgebung](/docs/de/self-hosted-environments) auf Compute aus, die Ihre Organisation betreibt.

<h2 id="setup-scripts">
  Setup-Skripte
</h2>

Ein Setup-Skript ist ein Bash-Skript, das ausgeführt wird, wenn eine neue Cloud-Sitzung startet, bevor Claude Code gestartet wird. Verwenden Sie Setup-Skripte, um Abhängigkeiten zu installieren, Tools zu konfigurieren oder alles zu beschaffen, das die Sitzung benötigt und nicht vorinstalliert ist.

Skripte werden als Root auf Ubuntu 24.04 ausgeführt, daher funktionieren `apt install` und die meisten Sprachpaket-Manager.

Um ein Setup-Skript hinzuzufügen, öffnen Sie den Dialog für Umgebungseinstellungen und geben Sie Ihr Skript in das Feld **Setup-Skript** ein.

Dieses Beispiel installiert [ShellCheck](https://www.shellcheck.net/), das nicht vorinstalliert ist.

```bash theme={null}
#!/bin/bash
apt update && apt install -y shellcheck
```

<h3 id="script-requirements">
  Anforderungen an Skripte
</h3>

Ein Setup-Skript hat drei Einschränkungen, die Sie beachten müssen:

* **Mit Null beenden**: Wenn das Skript mit einem Nicht-Null-Wert beendet wird, schlägt der Sitzungsstart fehl. Fügen Sie `|| true` an nicht kritische Befehle an, damit ein gelegentlicher Installationsfehler die Sitzung nicht blockiert.
* **Innerhalb von fünf Minuten fertig**: Halten Sie die Gesamtlaufzeit des Skripts unter etwa fünf Minuten, damit der [Umgebungs-Cache](#environment-caching) erstellt werden kann. Führen Sie unabhängige Installationen parallel mit `&` und `wait` aus, und verschieben Sie jeden einzelnen Download, der nicht passt, in einen [SessionStart-Hook](#setup-scripts-vs-sessionstart-hooks), der ihn im Hintergrund startet.
* **Netzwerkzugriff für Installationen**: Paketinstallationen müssen Registries erreichen. Die Standard-Stufe **Vertraut** deckt [häufige Paket-Registries](#default-allowed-domains) ab, einschließlich npm, PyPI, RubyGems und crates.io; mit **Keine** Netzwerkzugriff schlagen Installationen fehl.

<h3 id="environment-caching">
  Umgebungs-Caching
</h3>

Das Setup-Skript wird beim ersten Mal ausgeführt, wenn Sie eine Sitzung in einer Umgebung starten. Nach Abschluss erstellt Anthropic einen Snapshot des Dateisystems und verwendet diesen Snapshot als Ausgangspunkt für spätere Sitzungen. Neue Sitzungen starten mit Ihren Abhängigkeiten, Tools und Docker-Images bereits auf der Festplatte und überspringen den Setup-Skript-Schritt. Dies hält den Start schnell, auch wenn das Skript große Toolchains installiert oder Container-Images abruft.

Der Cache ist ein Dateisystem-Snapshot, daher behält er, was das Setup-Skript auf die Festplatte schreibt, und verliert alles, das nur ausgeführt wurde. Pakete, die Sie installieren, Docker-Images, die Sie abrufen, und Dateien, die Sie schreiben, werden alle übertragen. Eine Datenbank, die das Skript gestartet hat, ein `docker compose up`-Stack oder ein anderer Hintergrundprozess nicht; starten Sie diese pro Sitzung, indem Sie Claude fragen oder einen [SessionStart-Hook](#setup-scripts-vs-sessionstart-hooks) verwenden.

Das Setup-Skript wird erneut ausgeführt, um den Cache neu zu erstellen, wenn Sie das Setup-Skript oder die zulässigen Netzwerk-Hosts der Umgebung ändern, und wenn der Cache nach etwa sieben Tagen abläuft. Das Fortsetzen einer bestehenden Sitzung führt das Setup-Skript nie erneut aus.

Sie müssen Caching nicht aktivieren oder Snapshots selbst verwalten.

<h3 id="setup-scripts-vs-sessionstart-hooks">
  Setup-Skripte vs. SessionStart-Hooks
</h3>

Verwenden Sie ein Setup-Skript, um die VM selbst bereitzustellen: Toolchains und CLI-Tools, die nicht [vorinstalliert](#installed-tools) sind. Verwenden Sie einen [SessionStart-Hook](/docs/de/hooks#sessionstart) für die Projekteinrichtung, die überall ausgeführt werden sollte, lokal und in der Cloud, wie `npm install`.

Setup-Skripte und SessionStart-Hooks werden in einer festen Reihenfolge ausgeführt, wenn eine Cloud-Sitzung startet. Die Tabelle vergleicht, wo Sie sie konfigurieren, wann sie ausgeführt werden und wo sie ausgeführt werden.

|                                | Setup-Skripte                                                                                                                                                          | SessionStart-Hooks                                                                                                                                                                                                                              |
| ------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Wo Sie sie konfigurieren**   | Der Umgebungsdialog unter [claude.ai/code](https://claude.ai/code), plus die Seite **Cloud-Umgebungen** für [gemeinsame Umgebungen](#organization-shared-environments) | Eine [Einstellungsdatei](/docs/de/settings#where-settings-live) wie die `.claude/settings.json` Ihres Repositorys; siehe [Was wird aus Ihrem Setup übernommen](#what-carries-over-from-your-setup) für die Dateien, die eine Cloud-Sitzung erreichen |
| **Wann sie ausgeführt werden** | Vor dem Start von Claude Code, übersprungen, wenn eine [zwischengespeicherte Umgebung](#environment-caching) vorhanden ist                                             | Nach dem Start von Claude Code, bei jeder Sitzung einschließlich fortgesetzter                                                                                                                                                                  |
| **Wo sie ausgeführt werden**   | Nur Cloud-Sitzungen                                                                                                                                                    | Lokale und Cloud-Sitzungen                                                                                                                                                                                                                      |

Wenn Sie SessionStart-Hooks in Ihrer Benutzer-Ebene `~/.claude/settings.json` haben, erwarten Sie diese nicht in der Cloud. Benutzer-Ebene-Einstellungen bleiben auf Ihrem Computer. Welche anderen Hooks ausgeführt werden, hängt davon ab, wo die Sitzung ausgeführt wird:

* **Von Anthropic gehostete Umgebung**: Claude Code führt Hooks aus dem Repository und aus den [servergesteuerten Einstellungen](/docs/de/server-managed-settings) Ihrer Organisation aus.
* **[Selbst gehostete Umgebung](/docs/de/self-hosted-environments-configuration#permissions-and-tool-approval)**: Claude Code führt auch die Hooks aus, die der Operator aus dem `~/.claude/` des Runner-Hosts gesät hat, und die Hooks in der verwalteten Einstellungsdatei des Runner-Images, wenn diese Datei eine der [verwalteten Quellen ist, die Claude Code anwendet](/docs/de/managed-settings#how-claude-code-combines-managed-sources).

<h3 id="install-dependencies-with-a-sessionstart-hook">
  Abhängigkeiten mit einem SessionStart-Hook installieren
</h3>

Um Abhängigkeiten nur in Cloud-Sitzungen zu installieren, kombinieren Sie einen SessionStart-Hook mit einem Skript, das überprüft, wo es ausgeführt wird.

Fügen Sie zunächst einen SessionStart-Hook zu Ihrer `.claude/settings.json` des Repositorys hinzu. Diese Konfiguration weist Claude Code an, `scripts/install_pkgs.sh` aus Ihrem Repository auszuführen, wenn eine Sitzung startet oder fortgesetzt wird:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "startup|resume",
        "hooks": [
          {
            "type": "command",
            "command": "bash \"$CLAUDE_PROJECT_DIR\"/scripts/install_pkgs.sh"
          }
        ]
      }
    ]
  }
}
```

Der `matcher` begrenzt den Hook auf die `startup`- und `resume`-Ereignisse, und `$CLAUDE_PROJECT_DIR` wird zur Repository-Root aufgelöst, daher findet der Hook das Skript unabhängig vom Arbeitsverzeichnis der Sitzung.

Erstellen Sie anschließend das Skript unter `scripts/install_pkgs.sh`. Es wird sofort außerhalb der Cloud beendet und installiert dann Ihre Abhängigkeiten:

```bash theme={null}
#!/bin/bash

if [ "$CLAUDE_CODE_REMOTE" != "true" ]; then
  exit 0
fi

npm install
pip install -r requirements.txt
exit 0
```

Die `CLAUDE_CODE_REMOTE`-Überprüfung ist das, was die Installation auf Cloud-Sitzungen beschränkt: Die Umgebung der Sitzungs-VM trägt diese Variable als `true`, sie ist lokal nie `true`, daher beendet sich das Skript auf Ihrem Laptop, bevor es etwas installiert.

Zusammen geben die beiden Dateien jeder Cloud-Sitzung beim Start ein frisches `npm install` und `pip install`, während lokale Sitzungen unberührt bleiben.

<h4 id="limitations-in-cloud-sessions">
  Einschränkungen in Cloud-Sitzungen
</h4>

SessionStart-Hooks verhalten sich in der Cloud genauso wie lokal, mit diesen Vorbehalten:

* **Ein Repository pro Sitzung**: Eine Sitzung mit mehreren Repositorys lädt keine Hooks aus der `.claude/settings.json` eines Repositorys, daher wird ein SessionStart-Hook, den Sie dort definieren, nicht ausgeführt. Installieren Sie Abhängigkeiten für diese Sitzungen stattdessen mit einem [Setup-Skript](#setup-scripts).
* **Keine Cloud-Only-Scoping**: Hooks werden in lokalen und Cloud-Sitzungen ausgeführt. Um die lokale Ausführung zu überspringen, beenden Sie früh, es sei denn, die Umgebungsvariable `CLAUDE_CODE_REMOTE` ist `true`, wie das [Abhängigkeitsinstallations-Skript](#install-dependencies-with-a-sessionstart-hook) es tut.
* **Erfordert Netzwerkzugriff**: Installationsbefehle müssen Paket-Registries erreichen. Wenn Ihre Umgebung **Keine** Netzwerkzugriff verwendet, schlagen diese Hooks fehl. Die [Standard-Zulassungsliste](#default-allowed-domains) unter **Vertraut** deckt npm, PyPI, RubyGems und crates.io ab.
* **Proxy-Kompatibilität**: In von Anthropic gehosteten Umgebungen wird der gesamte ausgehende Datenverkehr durch einen [Sicherheits-Proxy](#security-proxy) geleitet, und einige Paket-Manager funktionieren nicht korrekt damit; Bun ist ein bekanntes Beispiel. In einer [selbst gehosteten Umgebung](/docs/de/self-hosted-environments-deploy#default-deny-egress) wird der ausgehende Datenverkehr stattdessen durch Ihre eigene Netzwerk-Grenze geleitet.
* **Fügt Startup-Latenz hinzu**: Hooks werden jedes Mal ausgeführt, wenn eine Sitzung startet oder fortgesetzt wird, im Gegensatz zu Setup-Skripten, die vom [Umgebungs-Caching](#environment-caching) profitieren. Halten Sie Installationsskripte schnell, indem Sie überprüfen, ob Abhängigkeiten bereits vorhanden sind, bevor Sie sie neu installieren.

Um das Basis-Image anzupassen, verwenden Sie ein Setup-Skript, um das zu installieren, was Sie auf dem [bereitgestellten Image](#installed-tools) benötigen, oder führen Sie Ihr eigenes Image als Container neben Claude mit `docker compose` aus. Das vollständige Ersetzen des Basis-Images wird noch nicht unterstützt.

<h2 id="default-allowed-domains">
  Standard-zulässige Domänen
</h2>

Mit **Trusted**-Netzwerkzugriff können Sitzungen standardmäßig die folgenden Domänen erreichen. Domänen, die mit `*` gekennzeichnet sind, zeigen Wildcard-Subdomain-Matching an, daher erlaubt `*.gcr.io` jede Subdomain von `gcr.io`.

<AccordionGroup>
  <Accordion title="Anthropic-Services">
    * api.anthropic.com
    * docs.claude.com
    * platform.claude.com
    * code.claude.com
    * claude.ai
  </Accordion>

  <Accordion title="Versionskontrolle">
    * github.com
    * [www.github.com](http://www.github.com)
    * api.github.com
    * npm.pkg.github.com
    * raw\.githubusercontent.com
    * pkg-npm.githubusercontent.com
    * objects.githubusercontent.com
    * release-assets.githubusercontent.com
    * codeload.github.com
    * avatars.githubusercontent.com
    * camo.githubusercontent.com
    * gist.github.com
    * gitlab.com
    * [www.gitlab.com](http://www.gitlab.com)
    * registry.gitlab.com
    * bitbucket.org
    * [www.bitbucket.org](http://www.bitbucket.org)
    * api.bitbucket.org
  </Accordion>

  <Accordion title="Container-Registrierungen">
    * registry-1.docker.io
    * auth.docker.io
    * index.docker.io
    * hub.docker.com
    * [www.docker.com](http://www.docker.com)
    * production.cloudflare.docker.com
    * download.docker.com
    * gcr.io
    * \*.gcr.io
    * ghcr.io
    * mcr.microsoft.com
    * \*.data.mcr.microsoft.com
    * public.ecr.aws
  </Accordion>

  <Accordion title="Cloud-Plattformen">
    * cloud.google.com
    * accounts.google.com
    * gcloud.google.com
    * \*.googleapis.com
    * storage.googleapis.com
    * compute.googleapis.com
    * container.googleapis.com
    * azure.com
    * portal.azure.com
    * microsoft.com
    * [www.microsoft.com](http://www.microsoft.com)
    * \*.microsoftonline.com
    * packages.microsoft.com
    * dotnet.microsoft.com
    * dot.net
    * visualstudio.com
    * dev.azure.com
    * \*.amazonaws.com
    * \*.api.aws
    * oracle.com
    * [www.oracle.com](http://www.oracle.com)
    * java.com
    * [www.java.com](http://www.java.com)
    * java.net
    * [www.java.net](http://www.java.net)
    * download.oracle.com
    * yum.oracle.com
    * \*.r2.cloudflarestorage.com
  </Accordion>

  <Accordion title="JavaScript und Node Paketmanager">
    * registry.npmjs.org
    * [www.npmjs.com](http://www.npmjs.com)
    * [www.npmjs.org](http://www.npmjs.org)
    * npmjs.com
    * npmjs.org
    * yarnpkg.com
    * registry.yarnpkg.com
    * jsr.io
    * npm.jsr.io
  </Accordion>

  <Accordion title="Python Paketmanager">
    * pypi.org
    * [www.pypi.org](http://www.pypi.org)
    * files.pythonhosted.org
    * pythonhosted.org
    * test.pypi.org
    * pypi.python.org
    * pypa.io
    * [www.pypa.io](http://www.pypa.io)
  </Accordion>

  <Accordion title="Ruby Paketmanager">
    * rubygems.org
    * [www.rubygems.org](http://www.rubygems.org)
    * api.rubygems.org
    * index.rubygems.org
    * ruby-lang.org
    * [www.ruby-lang.org](http://www.ruby-lang.org)
    * rubyforge.org
    * [www.rubyforge.org](http://www.rubyforge.org)
    * rubyonrails.org
    * [www.rubyonrails.org](http://www.rubyonrails.org)
    * rvm.io
    * get.rvm.io
  </Accordion>

  <Accordion title="Rust Paketmanager">
    * crates.io
    * [www.crates.io](http://www.crates.io)
    * index.crates.io
    * static.crates.io
    * rustup.rs
    * static.rust-lang.org
    * [www.rust-lang.org](http://www.rust-lang.org)
  </Accordion>

  <Accordion title="Go Paketmanager">
    * proxy.golang.org
    * sum.golang.org
    * index.golang.org
    * golang.org
    * [www.golang.org](http://www.golang.org)
    * goproxy.io
    * pkg.go.dev
  </Accordion>

  <Accordion title="JVM Paketmanager">
    * maven.org
    * repo.maven.org
    * central.maven.org
    * repo1.maven.org
    * repo.maven.apache.org
    * maven.google.com
    * jcenter.bintray.com
    * gradle.org
    * [www.gradle.org](http://www.gradle.org)
    * services.gradle.org
    * plugins.gradle.org
    * plugins-artifacts.gradle.org
    * kotlinlang.org
    * [www.kotlinlang.org](http://www.kotlinlang.org)
    * spring.io
    * repo.spring.io
  </Accordion>

  <Accordion title="Andere Paketmanager">
    * packagist.org (PHP Composer)
    * [www.packagist.org](http://www.packagist.org)
    * repo.packagist.org
    * nuget.org (.NET NuGet)
    * [www.nuget.org](http://www.nuget.org)
    * api.nuget.org
    * pub.dev (Dart/Flutter)
    * api.pub.dev
    * hex.pm (Elixir/Erlang)
    * [www.hex.pm](http://www.hex.pm)
    * cpan.org (Perl CPAN)
    * [www.cpan.org](http://www.cpan.org)
    * metacpan.org
    * [www.metacpan.org](http://www.metacpan.org)
    * api.metacpan.org
    * cocoapods.org (iOS/macOS)
    * [www.cocoapods.org](http://www.cocoapods.org)
    * cdn.cocoapods.org
    * haskell.org
    * [www.haskell.org](http://www.haskell.org)
    * hackage.haskell.org
    * swift.org
    * [www.swift.org](http://www.swift.org)
  </Accordion>

  <Accordion title="Linux-Distributionen">
    * archive.ubuntu.com
    * security.ubuntu.com
    * ubuntu.com
    * [www.ubuntu.com](http://www.ubuntu.com)
    * \*.ubuntu.com
    * ppa.launchpad.net
    * launchpad.net
    * [www.launchpad.net](http://www.launchpad.net)
    * \*.nixos.org
  </Accordion>

  <Accordion title="Entwicklungstools und Plattformen">
    * dl.k8s.io (Kubernetes)
    * pkgs.k8s.io
    * k8s.io
    * [www.k8s.io](http://www.k8s.io)
    * releases.hashicorp.com (HashiCorp)
    * apt.releases.hashicorp.com
    * rpm.releases.hashicorp.com
    * archive.releases.hashicorp.com
    * hashicorp.com
    * [www.hashicorp.com](http://www.hashicorp.com)
    * repo.anaconda.com (Anaconda/Conda)
    * conda.anaconda.org
    * anaconda.org
    * [www.anaconda.com](http://www.anaconda.com)
    * anaconda.com
    * continuum.io
    * apache.org (Apache)
    * [www.apache.org](http://www.apache.org)
    * archive.apache.org
    * downloads.apache.org
    * eclipse.org (Eclipse)
    * [www.eclipse.org](http://www.eclipse.org)
    * download.eclipse.org
    * nodejs.org (Node.js)
    * [www.nodejs.org](http://www.nodejs.org)
    * developer.apple.com
    * developer.android.com
    * pkg.stainless.com
    * binaries.prisma.sh
  </Accordion>

  <Accordion title="Cloud-Services und Monitoring">
    * http-intake.logs.datadoghq.com
    * \*.datadoghq.com
    * \*.datadoghq.eu
    * api.honeycomb.io
  </Accordion>

  <Accordion title="Content Delivery und Mirrors">
    * sourceforge.net
    * \*.sourceforge.net
    * packagecloud.io
    * \*.packagecloud.io
    * fonts.googleapis.com
    * fonts.gstatic.com
  </Accordion>

  <Accordion title="Schema und Konfiguration">
    * json-schema.org
    * [www.json-schema.org](http://www.json-schema.org)
    * json.schemastore.org
    * [www.schemastore.org](http://www.schemastore.org)
  </Accordion>

  <Accordion title="Model Context Protocol">
    * \*.modelcontextprotocol.io
  </Accordion>
</AccordionGroup>

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Cloud-Sitzungen-Referenz](/docs/de/claude-code-on-the-web): Starten, verwalten und teilen Sie Cloud-Sitzungen
* [Cloud-Sitzungen-Schnellstart](/docs/de/web-quickstart): Verbinden Sie GitHub und starten Sie Ihre erste Cloud-Sitzung
* [Claude Tag](https://claude.com/docs/claude-tag/overview): Sitzungen, die Claude aus Slack startet, werden in den gleichen Umgebungen ausgeführt
* [Routinen](/docs/de/routines): Geplante Läufe verwenden die gleichen Umgebungen und Netzwerkzugriffsstufen
* [Remote Control](/docs/de/remote-control): Führen Sie Sitzungen auf dem Netzwerk und den Dateien Ihres eigenen Computers aus
* [Selbstgehostete Umgebungen](/docs/de/self-hosted-environments): Führen Sie Cloud-Sitzungen auf der eigenen Infrastruktur Ihrer Organisation aus
* [SessionStart Hooks](/docs/de/hooks#sessionstart): Repo-committed Setup, das in lokalen und Cloud-Sitzungen ausgeführt wird
* [Server-verwaltete Einstellungen](/docs/de/server-managed-settings): Organisations-Richtlinie, die Cloud-Sitzungen erreicht
