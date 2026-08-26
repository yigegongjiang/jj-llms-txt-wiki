> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Spracherfassung

> Sprechen Sie Ihre Eingabeaufforderungen in der Claude Code CLI mit Halten-zum-Aufnehmen oder Tippen-zum-Aufnehmen Spracherfassung.

Sprechen Sie Ihre Eingabeaufforderungen, anstatt sie in der Claude Code CLI einzutippen. Ihre Sprache wird live in die Eingabeaufforderung transkribiert, sodass Sie Sprache und Tippen in derselben Nachricht mischen können. Aktivieren Sie die Erfassung mit `/voice`, halten Sie dann entweder eine Taste gedrückt, während Sie sprechen, oder tippen Sie einmal zum Starten und erneut zum Senden.

<Note>
  Tap-Modus erfordert Claude Code v2.1.116 oder später. Überprüfen Sie Ihre Version mit `claude --version`.
</Note>

Die Erfassung funktioniert auch in der [Agent-Ansicht](/docs/de/agent-view#peek-and-reply). Halten Sie Ihre Push-to-Talk-Taste gedrückt oder tippen Sie darauf, während die Dispatch-Eingabe oder eine Peek-Panel-Antwort fokussiert ist, um zu einer Hintergrund-Sitzung zu diktieren.

<h2 id="requirements">
  Anforderungen
</h2>

Die Spracherfassung streamt Ihre aufgenommene Audiodatei an Anthropic-Server zur Transkription. Audio wird nicht lokal verarbeitet. Dies erfordert alle folgenden Voraussetzungen:

* **Ein Claude.ai-Konto**: Der Sprache-zu-Text-Dienst ist nur verfügbar, wenn Sie sich mit einem authentifizieren, und ist nicht verfügbar, wenn Claude Code für die Verwendung eines Anthropic API-Schlüssels direkt, Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry konfiguriert ist.
* **Eine Organisation ohne aktivierte HIPAA-Compliance**: `/voice` zeigt `Voice mode is disabled by your organization's policy` an, wenn diese Einschränkung gilt.
* **Ein lokales Mikrofon**: Die Spracherfassung funktioniert nicht in Remote-Umgebungen wie [Claude Code im Web](/docs/de/claude-code-on-the-web) oder SSH-Sitzungen.
* **WSLg, wenn Sie Claude Code in WSL ausführen**: WSLg ist in WSL2 enthalten, wenn es unter Windows 10 oder 11 aus dem Microsoft Store installiert wird. Wenn WSLg nicht verfügbar ist, beispielsweise unter WSL1, führen Sie Claude Code stattdessen nativ unter Windows aus.

Die Transkription verbraucht keine Claude-Nachrichten oder Token und wird nicht auf die in `/usage` angezeigten Limits angerechnet. Siehe [Datennutzung](/docs/de/data-usage) für Informationen darüber, wie Anthropic Ihre Daten verarbeitet.

Die Audioaufnahme verwendet ein integriertes natives Modul unter macOS, Linux und Windows. Unter Linux wird Claude Code auf `arecord` aus ALSA-Dienstprogrammen oder `rec` aus SoX zurückgreifen, wenn das native Modul nicht geladen werden kann. Wenn keines verfügbar ist, gibt `/voice` einen Installationsbefehl für Ihren Paketmanager aus.

Die Claude Code [VS Code-Erweiterung](/docs/de/vs-code) unterstützt auch Spracherfassung mit derselben Claude.ai-Kontoanforderung. Sie ist nicht in VS Code Remote-Sitzungen verfügbar, einschließlich SSH, Dev Containers und Codespaces, da sich das Mikrofon auf Ihrem lokalen Computer befindet und die Erweiterung auf dem Remote-Host ausgeführt wird.

<h2 id="enable-voice-dictation">
  Spracherfassung aktivieren
</h2>

Führen Sie `/voice` aus, um die Erfassung zu aktivieren. Beim ersten Aktivieren führt Claude Code eine Mikrofonprüfung durch. Unter macOS wird dies die Systemmikrofonberechtigungsaufforderung für Ihr Terminal auslösen, falls diese noch nie gewährt wurde.

```
/voice
Voice mode enabled (hold). Hold space to record. Dictation language: en (/config to change).
```

`/voice` akzeptiert ein optionales Modusargument:

| Befehl        | Effekt                                                |
| :------------ | :---------------------------------------------------- |
| `/voice`      | Ein- oder ausschalten, aktuellen Modus beibehalten    |
| `/voice hold` | Im [Halten-Modus](#hold-to-record) aktivieren         |
| `/voice tap`  | Im [Tippen-Modus](#tap-to-record-and-send) aktivieren |
| `/voice off`  | Deaktivieren                                          |

Die Spracherfassung bleibt über Sitzungen hinweg erhalten. Legen Sie sie direkt in Ihrer [Benutzereinstellungsdatei](/docs/de/settings) fest, anstatt `/voice` auszuführen:

```json theme={null}
{
  "voice": {
    "enabled": true,
    "mode": "tap"
  }
}
```

Während die Spracherfassung aktiviert ist, zeigt die Eingabefußzeile einen `hold space to speak`-Hinweis an, wenn die Eingabeaufforderung leer ist. Der Hinweis spiegelt Ihre aktuelle `voice:pushToTalk`-Bindung wider und wird aktualisiert, wenn Sie [die Erfassungstaste neu zuordnen](#rebind-the-dictation-key). Der Hinweistext ist in beiden Modi gleich und wird nicht angezeigt, wenn Sie eine [benutzerdefinierte Statuszeile](/docs/de/statusline) konfiguriert haben.

Die Transkription ist in beiden Modi auf Codierungsvokabular abgestimmt. Häufige Entwicklungsbegriffe wie `regex`, `OAuth`, `JSON` und `localhost` werden korrekt erkannt, und Ihr aktueller Projektname und Git-Branch-Name werden automatisch als Erkennungshinweise hinzugefügt.

<h2 id="hold-to-record">
  Halten zum Aufnehmen
</h2>

Der Halten-Modus ist Push-to-Talk: Die Aufnahme läuft, während Sie die Taste halten, und stoppt, wenn Sie sie loslassen. Dies ist der Standardmodus.

Halten Sie `Space` gedrückt, um die Aufnahme zu starten. Claude Code erkennt eine gehaltene Taste, indem es schnelle Tastenwiederholungsereignisse von Ihrem Terminal überwacht, daher gibt es eine kurze Aufwärmphase, bevor die Aufnahme beginnt. Die Fußzeile zeigt `keep holding…` während der Aufwärmphase an und wechselt dann zu einer Live-Wellenform, sobald die Aufnahme aktiv ist.

Die ersten paar Tastenwiederholungszeichen werden während der Aufwärmphase in die Eingabe eingegeben und werden automatisch entfernt, wenn die Aufnahme aktiviert wird. Ein einzelnes `Space`-Tippen gibt immer noch ein Leerzeichen ein, da die Halten-Erkennung nur bei schneller Wiederholung ausgelöst wird.

<Tip>
  Um die Aufwärmphase zu überspringen, wechseln Sie mit `/voice tap` zum [Tippen-Modus](#tap-to-record-and-send), oder [binden Sie eine Modifikatorkombination](#rebind-the-dictation-key) wie `meta+k` neu. Modifikatorkombinationen starten die Aufnahme beim ersten Tastendruck.
</Tip>

Ihre Sprache erscheint in der Eingabeaufforderung, während Sie sprechen, abgeblendet, bis das Transkript finalisiert ist. Lassen Sie `Space` los, um die Aufnahme zu stoppen und den Text zu finalisieren. Das Transkript wird an Ihrer Cursorposition eingefügt und der Cursor bleibt am Ende des eingefügten Textes, sodass Sie Tippen und Erfassung in beliebiger Reihenfolge mischen können. Halten Sie `Space` erneut gedrückt, um eine weitere Aufnahme anzufügen, oder verschieben Sie den Cursor zuerst, um Sprache an anderer Stelle in der Eingabeaufforderung einzufügen:

```
> refactor the auth middleware to ▮
  # hold Space, speak "use the new token validation helper"
> refactor the auth middleware to use the new token validation helper▮
```

Standardmäßig fügt das Loslassen der Taste das Transkript ein und wartet darauf, dass Sie `Enter` drücken. Legen Sie `"autoSubmit": true` im `voice`-Einstellungsobjekt fest, um die Eingabeaufforderung automatisch zu senden, wenn Sie die Taste loslassen, solange das Transkript mindestens drei Wörter lang ist.

<h2 id="tap-to-record-and-send">
  Tippen zum Aufnehmen und Senden
</h2>

Der Tippen-Modus schaltet die Aufnahme mit einem einzelnen Tastendruck um: Tippen Sie einmal zum Starten, sprechen Sie, dann tippen Sie erneut zum Senden der Eingabeaufforderung. Es gibt keine Aufwärmphase und Sie müssen die Taste nicht gedrückt halten.

Aktivieren Sie den Tippen-Modus mit `/voice tap`. Wenn die Eingabeaufforderung leer ist, tippen Sie auf `Space`, um die Aufnahme zu starten. Die Fußzeile zeigt eine Live-Wellenform während der Aufnahme. Tippen Sie erneut auf `Space`, um zu stoppen.

Claude Code fügt das Transkript ein und sendet die Eingabeaufforderung automatisch, wenn das Transkript mindestens drei Wörter lang ist. Kürzere Transkripte werden eingefügt, aber nicht gesendet, daher sendet ein versehentliches Tippen kein einzelnes Wort.

Die Schwelle von drei Wörtern zählt Wörter für Sprachen, die ohne Leerzeichen geschrieben werden. Ab v2.1.195 zählen japanische, chinesische und thailändische Transkripte einzelne Wörter, sodass sie im Tippen-Modus und im Halte-Modus mit `autoSubmit` automatisch gesendet werden. Frühere Versionen zählten ein Transkript ohne Leerzeichen als ein Wort und sendeten es nie automatisch.

Das erste Tippen startet die Aufnahme nur, wenn die Eingabeaufforderung leer ist, sodass Sie immer noch normal Leerzeichen eingeben können, während Sie eine Nachricht verfassen. Das zweite Tippen stoppt die Aufnahme unabhängig vom Eingabeinhalt. Die Aufnahme stoppt auch automatisch nach 15 Sekunden Stille oder zwei Minuten insgesamt.

<h2 id="change-the-dictation-language">
  Ändern Sie die Erfassungssprache
</h2>

Die Spracherfassung verwendet die gleiche [`language`-Einstellung](/docs/de/settings), die die Antwortsprache von Claude steuert. Wenn diese Einstellung leer ist, wird die Erfassung standardmäßig auf Englisch eingestellt. In der VS Code-Erweiterung wird die Erfassung, wenn `language` leer ist, die `accessibility.voice.speechLanguage`-Einstellung von VS Code verwenden, bevor sie auf Englisch zurückfällt.

<Accordion title="Unterstützte Erfassungssprachen">
  | Sprache        | Code |
  | :------------- | :--- |
  | Tschechisch    | `cs` |
  | Dänisch        | `da` |
  | Niederländisch | `nl` |
  | Englisch       | `en` |
  | Französisch    | `fr` |
  | Deutsch        | `de` |
  | Griechisch     | `el` |
  | Hindi          | `hi` |
  | Indonesisch    | `id` |
  | Italienisch    | `it` |
  | Japanisch      | `ja` |
  | Koreanisch     | `ko` |
  | Norwegisch     | `no` |
  | Polnisch       | `pl` |
  | Portugiesisch  | `pt` |
  | Russisch       | `ru` |
  | Spanisch       | `es` |
  | Schwedisch     | `sv` |
  | Türkisch       | `tr` |
  | Ukrainisch     | `uk` |
</Accordion>

Legen Sie die Sprache in `/config` fest oder direkt in den Einstellungen. Sie können entweder den [BCP 47-Sprachcode](https://en.wikipedia.org/wiki/IETF_language_tag) oder den Sprachnamen verwenden:

```json theme={null}
{
  "language": "japanese"
}
```

Wenn Ihre `language`-Einstellung nicht in der unterstützten Liste enthalten ist, warnt Sie `/voice` beim Aktivieren und fällt für die Erfassung auf Englisch zurück. Clauds Textantworten sind von diesem Fallback nicht betroffen.

<h2 id="rebind-the-dictation-key">
  Binden Sie die Erfassungstaste neu
</h2>

Die Erfassungstaste ist an `voice:pushToTalk` im `Chat`-Kontext gebunden und wird standardmäßig auf `Space` eingestellt. Die gleiche Bindung steuert sowohl den Halten- als auch den Tippen-Modus. Binden Sie sie in [`~/.claude/keybindings.json`](/docs/de/keybindings) neu:

```json theme={null}
{
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "meta+k": "voice:pushToTalk",
        "space": null
      }
    }
  ]
}
```

Die `voice:pushToTalk`-Aktion verwendet jeweils eine Taste. Wenn Sie eine benutzerdefinierte Taste binden, ersetzt sie die Standard-`Space`-Bindung, anstatt einen zweiten Auslöser hinzuzufügen. Daher dient die Zeile `"space": null` in diesem Beispiel der Klarheit und kann weggelassen werden, ohne das Verhalten zu ändern.

Im Halten-Modus vermeiden Sie das Binden einer bloßen Buchstabentaste wie `v`, da die Halten-Erkennung auf Tastenwiederholung angewiesen ist und der Buchstabe während der Aufwärmphase in die Eingabeaufforderung eingegeben wird. Verwenden Sie `Space` oder eine Modifikatorkombination wie `meta+k`, um die Aufnahme beim ersten Tastendruck ohne Aufwärmphase zu starten. Der Tippen-Modus hat keine Aufwärmphase, daher funktioniert jede Taste.

Einige Tasten werden nicht an Terminalanwendungen übermittelt und können überhaupt nicht gebunden werden. Beispielsweise zeigt `Caps Lock` einen Fehler an, wenn Sie versuchen, es zu binden. Siehe [Tastaturkürzel anpassen](/docs/de/keybindings) für die vollständige Tastenbindungssyntax und die Liste der reservierten Tastenkombinationen.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

Häufige Probleme, wenn die Spracherfassung nicht aktiviert wird oder nicht aufnimmt:

* **`Voice mode requires a Claude.ai account`**: Sie sind mit einem API-Schlüssel oder einem Drittanbieter authentifiziert. Führen Sie `/login` aus, um sich mit einem Claude.ai-Konto anzumelden.
* **`Voice mode is disabled by your organization's policy`**: Die Spracherfassungsfunktion ist durch die Compliance-Konfiguration Ihrer Organisation deaktiviert, wie in [Anforderungen](#requirements) beschrieben. Kontaktieren Sie Ihren Organisationsadministrator, um zu bestätigen, ob die Spracherfassung für Ihre Organisation verfügbar ist.
* **`Microphone access is denied`**: Gewähren Sie Ihrem Terminal in den Systemeinstellungen Mikrofonberechtigung. Unter macOS gehen Sie zu Systemeinstellungen → Datenschutz & Sicherheit → Mikrofon und aktivieren Sie Ihre Terminal-App, führen Sie dann `/voice` erneut aus. Unter Windows gehen Sie zu Einstellungen → Datenschutz & Sicherheit → Mikrofon und aktivieren Sie den Mikrofonzugriff für Desktop-Apps, führen Sie dann `/voice` erneut aus. Wenn Ihr Terminal nicht in den macOS-Einstellungen aufgeführt ist, siehe [Terminal nicht in macOS-Mikrofoneinstellungen aufgeführt](#terminal-not-listed-in-macos-microphone-settings).
* **`No audio recording tool found` unter Linux**: Das native Audiomodul konnte nicht geladen werden und kein Fallback ist installiert. Installieren Sie SoX mit dem im Fehlermeldung angezeigten Befehl, z. B. `sudo apt-get install sox`.
* **`Voice mode requires a microphone, but SoX could not open an audio capture device`**: SoX ist installiert, aber der Host hat kein Audioaufnahmegerät, z. B. einen Server ohne Kopfhörer oder einen Container. Führen Sie Claude Code auf einem Computer mit einem Mikrofon aus. Ab v2.1.195 meldet Claude Code unter Linux diese Meldung in dieser Situation; frühere Versionen forderten Sie auf, SoX zu installieren, auch wenn es bereits installiert war.
* **`Voice mode could not find a working audio recorder in WSL`**: WSLg leitet Audio über PulseAudio statt über ein ALSA-Gerät weiter, daher muss das PulseAudio-Backend von SoX explizit installiert werden. Führen Sie `sudo apt install sox libsox-fmt-pulse` aus. Die Installation von `sox` allein zieht das ALSA-Backend mit sich, das unter WSL nicht aufnehmen kann, da es kein `/dev/snd`-Gerät gibt.
* **`Voice input is failing repeatedly and has been paused`**: Die Spracherfassung ist mehrmals hintereinander fehlgeschlagen und hat versucht, neue Sitzungen zu stoppen, bis eine erfolgreich ist. Ein Fehler zählt, ob das Mikrofon nicht startet oder der Rekorder startet und dann stoppt, ohne Audio zu produzieren. Dies bedeutet normalerweise, dass das Mikrofon oder der Audiostapel auf diesem Host keinen Audio erfassen kann, z. B. ein Server ohne Kopfhörer, eine Remote-Shell ohne Audio-Durchleitung oder eine verweigerte Mikrofonberechtigung. Bestätigen Sie ein funktionierendes Eingabegerät, beheben Sie die zugrunde liegende Ursache aus den obigen Einträgen und lösen Sie dann die Spracherfassung erneut aus. Vor v2.1.202 zählten nur Start-Fehler zur Pause.
* **Nichts passiert, wenn Sie `Space` im Halten-Modus halten**: Beobachten Sie die Eingabeaufforderung, während Sie halten. Wenn sich Leerzeichen weiter ansammeln, ist die Spracherfassung wahrscheinlich aus; führen Sie `/voice hold` aus, um sie zu aktivieren. Wenn nur ein oder zwei Leerzeichen erscheinen und dann nichts, ist die Spracherfassung an, aber die Halten-Erkennung wird nicht ausgelöst. Die Halten-Erkennung erfordert, dass Ihr Terminal Tastenwiederholungsereignisse sendet, daher kann es eine gehaltene Taste nicht erkennen, wenn die Tastenwiederholung auf Betriebssystemebene deaktiviert ist. Wechseln Sie mit `/voice tap` zum Tippen-Modus, um die Tastenwiederholungsanforderung zu vermeiden.
* **Das Tippen auf `Space` gibt ein Leerzeichen ein, anstatt im Tippen-Modus aufzunehmen**: Das erste Tippen startet die Aufnahme nur, wenn die Eingabeaufforderung leer ist. Löschen Sie zuerst die Eingabe, oder überprüfen Sie, dass Sie im Tippen-Modus sind, indem Sie `/voice tap` ausführen.
* **`No audio detected from microphone`**: Die Aufnahme wurde gestartet, aber es wurde Stille erfasst. Bestätigen Sie, dass das richtige Eingabegerät als Systemstandard eingestellt ist und dass sein Eingabepegel nicht stummgeschaltet oder nahe Null ist. Unter Windows öffnen Sie Einstellungen → System → Sound → Eingabe und wählen Sie Ihr Mikrofon aus. Unter macOS öffnen Sie Systemeinstellungen → Sound → Eingabe.
* **`Voice connection failed`**: Ihre Aufnahme erreichte den Transkriptionsdienst nicht, da die Verbindung fehlgeschlagen ist. Überprüfen Sie Ihr Netzwerk und versuchen Sie es erneut. Eine Aufnahme, die keinen Audio erfasst, meldet stattdessen `No audio detected from microphone`. Vor v2.1.200 konnte ein stilles Mikrofon einen Verbindungsfehler melden, was auf ein Netzwerkproblem hindeutete, wenn das eigentliche Problem das Eingabegerät war.
* **`No speech detected`**: Audio erreichte den Transkriptionsdienst, aber es wurden keine Wörter erkannt. Sprechen Sie näher zum Mikrofon, reduzieren Sie Hintergrundgeräusche und bestätigen Sie, dass Ihre [Erfassungssprache](#change-the-dictation-language) der Sprache entspricht, die Sie sprechen.
* **Transkription ist verzerrt oder in der falschen Sprache**: Die Erfassung wird standardmäßig auf Englisch eingestellt. Wenn Sie in einer anderen Sprache erfassen, legen Sie sie zuerst in `/config` fest. Siehe [Ändern Sie die Erfassungssprache](#change-the-dictation-language).

<h3 id="terminal-not-listed-in-macos-microphone-settings">
  Terminal nicht in macOS-Mikrofoneinstellungen aufgeführt
</h3>

Wenn Ihre Terminal-App nicht unter Systemeinstellungen → Datenschutz & Sicherheit → Mikrofon angezeigt wird, gibt es keinen Schalter, den Sie aktivieren können. Setzen Sie den Berechtigungsstatus für Ihr Terminal zurück, damit die nächste `/voice`-Ausführung eine neue macOS-Berechtigungsaufforderung auslöst.

<Steps>
  <Step title="Setzen Sie die Mikrofonberechtigung für Ihr Terminal zurück">
    Führen Sie `tccutil reset Microphone <bundle-id>` aus und ersetzen Sie `<bundle-id>` durch die Kennung Ihres Terminals: `com.apple.Terminal` für das integrierte Terminal oder `com.googlecode.iterm2` für iTerm2. Für andere Terminals suchen Sie die Kennung mit `osascript -e 'id of app "AppName"'`.

    <Warning>
      Sie können `tccutil reset Microphone` ohne eine Bundle-ID ausführen, aber dies widerruft den Mikrofonzugriff von jeder App auf Ihrem Mac, einschließlich Apps wie Zoom oder Slack. Jede App muss beim nächsten Gebrauch erneut Zugriff anfordern, führen Sie es also nicht während eines aktiven Anrufs aus.
    </Warning>
  </Step>

  <Step title="Beenden Sie Ihr Terminal und starten Sie es neu">
    macOS wird einen Prozess, der bereits ausgeführt wird, nicht erneut auffordern. Beenden Sie die Terminal-App mit Cmd+Q, nicht nur schließen Sie ihre Fenster, öffnen Sie sie dann erneut.
  </Step>

  <Step title="Lösen Sie eine neue Aufforderung aus">
    Starten Sie Claude Code und führen Sie `/voice` aus. macOS fordert Mikrofonzugriff an; erlauben Sie es.
  </Step>
</Steps>

<h2 id="see-also">
  Siehe auch
</h2>

* [Tastaturkürzel anpassen](/docs/de/keybindings): Binden Sie `voice:pushToTalk` und andere CLI-Tastaturaktionen neu
* [Einstellungen konfigurieren](/docs/de/settings): Vollständige Referenz für `voice`, `language` und andere Einstellungsschlüssel
* [Interaktiver Modus](/docs/de/interactive-mode): Tastaturkürzel, Eingabemodi und Sitzungssteuerungen
* [Befehle](/docs/de/commands): Referenz für `/voice`, `/config` und alle anderen Befehle
