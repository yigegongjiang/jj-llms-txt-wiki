> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Dettatura vocale

> Pronuncia i tuoi prompt nella CLI di Claude Code con dettatura vocale a pressione prolungata o a tocco.

Pronuncia i tuoi prompt invece di digitarli nella CLI di Claude Code. Il tuo discorso viene trascritto in tempo reale nell'input del prompt, quindi puoi mescolare voce e digitazione nello stesso messaggio. Abilita la dettatura con `/voice`, quindi tieni premuto un tasto mentre parli oppure tocca una volta per iniziare e di nuovo per inviare.

<Note>
  Tap mode richiede Claude Code v2.1.116 o successivo. Controlla la tua versione con `claude --version`.
</Note>

La dettatura funziona anche nella [visualizzazione agente](/docs/it/agent-view#peek-and-reply). Tieni premuto o tocca il tasto push-to-talk mentre l'input di dispatch o una risposta del pannello peek è focalizzata per dettare a una sessione in background.

<h2 id="requirements">
  Requisiti
</h2>

La dettatura vocale trasmette l'audio registrato ai server di Anthropic per la trascrizione. L'audio non viene elaborato localmente. È necessario disporre di tutti i seguenti elementi:

* **Un account Claude.ai**: il servizio di sintesi vocale è disponibile solo quando vi autenticate con uno, e non è disponibile quando Claude Code è configurato per utilizzare direttamente una chiave API di Anthropic, Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry.
* **Un'organizzazione senza conformità HIPAA abilitata**: `/voice` mostra `Voice mode is disabled by your organization's policy` quando questa restrizione si applica.
* **Un microfono locale**: la dettatura vocale non funziona in ambienti remoti come [Claude Code sul web](/docs/it/claude-code-on-the-web) o sessioni SSH.
* **WSLg, se eseguite Claude Code in WSL**: WSLg è incluso con WSL2 quando installato da Microsoft Store su Windows 10 o 11. Se WSLg non è disponibile, ad esempio su WSL1, eseguite Claude Code in Windows nativo.

La trascrizione non consuma messaggi Claude o token e non conta verso i limiti mostrati in `/usage`. Consultate [data usage](/docs/it/data-usage) per scoprire come Anthropic gestisce i vostri dati.

La registrazione audio utilizza un modulo nativo integrato su macOS, Linux e Windows. Su Linux, se il modulo nativo non riesce a caricarsi, Claude Code ricade su `arecord` da ALSA utils o `rec` da SoX. Se nessuno dei due è disponibile, `/voice` stampa un comando di installazione per il vostro gestore di pacchetti.

L'[estensione VS Code](/docs/it/vs-code) di Claude Code supporta anche la dettatura vocale con lo stesso requisito di account Claude.ai. Non è disponibile nelle sessioni VS Code Remote, incluse SSH, Dev Containers e Codespaces, perché il microfono si trova sulla vostra macchina locale e l'estensione viene eseguita sull'host remoto.

<h2 id="enable-voice-dictation">
  Abilita la dettatura vocale
</h2>

Esegui `/voice` per abilitare la dettatura. La prima volta che la abiliti, Claude Code esegue un controllo del microfono. Su macOS, questo attiva il prompt di autorizzazione del microfono di sistema per il tuo terminale se non è mai stato concesso.

```
/voice
Voice mode enabled (hold). Hold space to record. Dictation language: en (/config to change).
```

`/voice` accetta un argomento di modalità opzionale:

| Comando       | Effetto                                                     |
| :------------ | :---------------------------------------------------------- |
| `/voice`      | Attiva/disattiva, mantieni la modalità corrente             |
| `/voice hold` | Abilita in [modalità pressione prolungata](#hold-to-record) |
| `/voice tap`  | Abilita in [modalità tocco](#tap-to-record-and-send)        |
| `/voice off`  | Disabilita                                                  |

La dettatura vocale persiste tra le sessioni. Impostala direttamente nel tuo [file di impostazioni utente](/docs/it/settings) invece di eseguire `/voice`:

```json theme={null}
{
  "voice": {
    "enabled": true,
    "mode": "tap"
  }
}
```

Mentre la dettatura vocale è abilitata, il footer di input mostra un suggerimento `hold space to speak` quando il prompt è vuoto. Il suggerimento riflette il tuo binding `voice:pushToTalk` corrente e si aggiorna se [riassegni il tasto di dettatura](#rebind-the-dictation-key). Il testo del suggerimento è lo stesso in entrambe le modalità e non appare se hai un [status line personalizzato](/docs/it/statusline) configurato.

La trascrizione è ottimizzata per il vocabolario di codifica in entrambe le modalità. I termini di sviluppo comuni come `regex`, `OAuth`, `JSON` e `localhost` vengono riconosciuti correttamente e il nome del tuo progetto attuale e il nome del ramo git vengono aggiunti automaticamente come suggerimenti di riconoscimento.

<h2 id="hold-to-record">
  Pressione prolungata per registrare
</h2>

La modalità pressione prolungata è push-to-talk: la registrazione viene eseguita mentre tieni premuto il tasto e si interrompe quando lo rilasci. Questa è la modalità predefinita.

Tieni premuto `Space` per iniziare la registrazione. Claude Code rileva un tasto premuto osservando gli eventi di ripetizione rapida dei tasti dal tuo terminale, quindi c'è un breve riscaldamento prima che inizi la registrazione. Il footer mostra `keep holding…` durante il riscaldamento, quindi passa a una forma d'onda dal vivo una volta che la registrazione è attiva.

I primi caratteri di ripetizione dei tasti digitano nell'input durante il riscaldamento e vengono rimossi automaticamente quando la registrazione si attiva. Un singolo tocco di `Space` digita comunque uno spazio, poiché il rilevamento della pressione prolungata si attiva solo sulla ripetizione rapida.

<Tip>
  Per saltare il riscaldamento, passa a [modalità tocco](#tap-to-record-and-send) con `/voice tap`, oppure [riassegna a una combinazione di modificatori](#rebind-the-dictation-key) come `meta+k`. Le combinazioni di modificatori iniziano la registrazione alla prima pressione del tasto.
</Tip>

Il tuo discorso appare nel prompt mentre parli, attenuato fino a quando la trascrizione non viene finalizzata. Rilascia `Space` per interrompere la registrazione e finalizzare il testo. La trascrizione viene inserita nella posizione del cursore e il cursore rimane alla fine del testo inserito, quindi puoi mescolare digitazione e dettatura in qualsiasi ordine. Tieni premuto `Space` di nuovo per aggiungere un'altra registrazione, oppure sposta il cursore prima per inserire il discorso altrove nel prompt:

```
> refactor the auth middleware to ▮
  # hold Space, speak "use the new token validation helper"
> refactor the auth middleware to use the new token validation helper▮
```

Per impostazione predefinita, il rilascio del tasto inserisce la trascrizione e attende che tu prema `Enter`. Imposta `"autoSubmit": true` nell'oggetto impostazioni `voice` per inviare il prompt automaticamente quando rilasci il tasto, purché la trascrizione sia lunga almeno tre parole.

<h2 id="tap-to-record-and-send">
  Tocco per registrare e inviare
</h2>

La modalità tocco attiva/disattiva la registrazione con una singola pressione di tasto: tocca una volta per iniziare, parla, quindi tocca di nuovo per inviare il prompt. Non c'è riscaldamento e non è necessario mantenere il tasto premuto.

Abilita la modalità tocco con `/voice tap`. Con l'input del prompt vuoto, tocca `Space` per iniziare la registrazione. Il footer mostra una forma d'onda dal vivo durante la registrazione. Tocca `Space` di nuovo per interrompere.

Claude Code inserisce la trascrizione e invia il prompt automaticamente quando la trascrizione è lunga almeno tre parole. Le trascrizioni più brevi vengono inserite ma non inviate, quindi un tocco accidentale non invia una parola casuale.

La soglia di tre parole conta le parole per le lingue scritte senza spazi. A partire dalla v2.1.195, le trascrizioni in giapponese, cinese e tailandese contano le parole individuali, quindi si inviano automaticamente in modalità tocco e in modalità di mantenimento con `autoSubmit`. Le versioni precedenti contavano una trascrizione senza spazi come una parola e non l'inviavano mai automaticamente.

Il primo tocco avvia la registrazione solo quando l'input del prompt è vuoto, quindi puoi comunque digitare spazi normalmente mentre componi un messaggio. Il secondo tocco interrompe la registrazione indipendentemente dal contenuto dell'input. La registrazione si interrompe anche automaticamente dopo 15 secondi di silenzio o due minuti totali.

<h2 id="change-the-dictation-language">
  Cambia la lingua della dettatura
</h2>

La dettatura vocale utilizza la stessa [impostazione `language`](/docs/it/settings) che controlla la lingua di risposta di Claude. Se tale impostazione è vuota, la dettatura predefinita è l'inglese. Nell'estensione VS Code, se `language` è vuoto, la dettatura utilizza l'impostazione `accessibility.voice.speechLanguage` di VS Code prima di predefinire l'inglese.

<Accordion title="Lingue di dettatura supportate">
  | Lingua      | Codice |
  | :---------- | :----- |
  | Ceco        | `cs`   |
  | Danese      | `da`   |
  | Olandese    | `nl`   |
  | Inglese     | `en`   |
  | Francese    | `fr`   |
  | Tedesco     | `de`   |
  | Greco       | `el`   |
  | Hindi       | `hi`   |
  | Indonesiano | `id`   |
  | Italiano    | `it`   |
  | Giapponese  | `ja`   |
  | Coreano     | `ko`   |
  | Norvegese   | `no`   |
  | Polacco     | `pl`   |
  | Portoghese  | `pt`   |
  | Russo       | `ru`   |
  | Spagnolo    | `es`   |
  | Svedese     | `sv`   |
  | Turco       | `tr`   |
  | Ucraino     | `uk`   |
</Accordion>

Imposta la lingua in `/config` o direttamente nelle impostazioni. Puoi utilizzare il [codice lingua BCP 47](https://en.wikipedia.org/wiki/IETF_language_tag) o il nome della lingua:

```json theme={null}
{
  "language": "japanese"
}
```

Se la tua impostazione `language` non è nell'elenco supportato, `/voice` ti avverte all'abilitazione e ricade all'inglese per la dettatura. Le risposte di testo di Claude non sono influenzate da questo fallback.

<h2 id="rebind-the-dictation-key">
  Riassegna il tasto di dettatura
</h2>

Il tasto di dettatura è associato a `voice:pushToTalk` nel contesto `Chat` e predefinito su `Space`. Lo stesso binding controlla sia la modalità pressione prolungata che la modalità tocco. Riassegnalo in [`~/.claude/keybindings.json`](/docs/it/keybindings):

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

L'azione `voice:pushToTalk` utilizza un tasto alla volta. Quando assegni un tasto personalizzato, sostituisce il binding predefinito `Space` anziché aggiungere un secondo trigger, quindi la riga `"space": null` in questo esempio è per chiarezza e può essere omessa senza modificare il comportamento.

In modalità pressione prolungata, evita di associare un tasto lettera nudo come `v` poiché il rilevamento della pressione prolungata si basa sulla ripetizione dei tasti e la lettera digita nel prompt durante il riscaldamento. Usa `Space`, oppure usa una combinazione di modificatori come `meta+k` per iniziare la registrazione alla prima pressione del tasto senza riscaldamento. La modalità tocco non ha riscaldamento, quindi la maggior parte dei tasti funziona.

Alcuni tasti non vengono consegnati alle applicazioni terminali e non possono essere associati affatto. Ad esempio, `Caps Lock` mostra un errore se tenti di associarlo. Consulta [personalizza scorciatoie da tastiera](/docs/it/keybindings) per la sintassi completa del keybinding e l'elenco delle scorciatoie riservate.

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

Problemi comuni quando la dettatura vocale non si attiva o non registra:

* **`Voice mode requires a Claude.ai account`**: sei autenticato con una chiave API o un provider di terze parti. Esegui `/login` per accedere con un account Claude.ai.
* **`Voice mode is disabled by your organization's policy`**: la configurazione di conformità della tua organizzazione disabilita la dettatura vocale, come descritto in [Requisiti](#requirements). Contatta l'amministratore della tua organizzazione per confermare se la dettatura vocale è disponibile per la tua organizzazione.
* **`Microphone access is denied`**: concedi l'autorizzazione del microfono al tuo terminale nelle impostazioni di sistema. Su macOS, vai a Impostazioni di sistema → Privacy e sicurezza → Microfono e abilita la tua app terminale, quindi esegui `/voice` di nuovo. Su Windows, vai a Impostazioni → Privacy e sicurezza → Microfono e attiva l'accesso al microfono per le app desktop, quindi esegui `/voice` di nuovo. Se il tuo terminale non è elencato nelle impostazioni macOS, consulta [Terminale non elencato nelle impostazioni del microfono di macOS](#terminal-not-listed-in-macos-microphone-settings).
* **`No audio recording tool found` su Linux**: il modulo audio nativo non ha potuto caricarsi e nessun fallback è installato. Installa SoX con il comando mostrato nel messaggio di errore, ad esempio `sudo apt-get install sox`.
* **`Voice mode requires a microphone, but SoX could not open an audio capture device`**: SoX è installato, ma l'host non ha alcun dispositivo di cattura audio, ad esempio un server headless o un container. Esegui Claude Code su una macchina con un microfono. A partire dalla v2.1.195, Claude Code su Linux segnala questo messaggio in quella situazione; le versioni precedenti ti chiedevano di installare SoX anche quando era già installato.
* **`Voice mode could not find a working audio recorder in WSL`**: WSLg instrada l'audio attraverso PulseAudio piuttosto che un dispositivo ALSA, quindi SoX ha bisogno che il suo backend PulseAudio sia installato esplicitamente. Esegui `sudo apt install sox libsox-fmt-pulse`. L'installazione di `sox` da sola estrae il backend ALSA, che non può registrare su WSL perché non c'è alcun dispositivo `/dev/snd`.
* **`Voice input is failing repeatedly and has been paused`**: la dettatura vocale ha riscontrato diversi errori di cattura di seguito e ha smesso di tentare nuove sessioni fino a quando una non avrà successo. Un errore conta sia che il microfono non riesca ad avviarsi sia che il registratore si avvii e poi si fermi senza produrre alcun audio. Questo di solito significa che il microfono o lo stack audio su questo host non può catturare l'audio, ad esempio un server headless, una shell remota senza passthrough audio, o un'autorizzazione del microfono negata. Conferma un dispositivo di input funzionante, correggi la causa sottostante dalle voci precedenti, quindi attiva di nuovo la voce. Prima della v2.1.202, solo gli errori di avvio contavano verso la pausa.
* **Nulla accade quando tieni premuto `Space` in modalità pressione prolungata**: osserva l'input del prompt mentre tieni premuto. Se gli spazi continuano ad accumularsi, la dettatura vocale è probabilmente disattivata; esegui `/voice hold` per abilitarla. Se appare solo uno o due spazi e poi nulla, la dettatura vocale è attiva ma il rilevamento della pressione prolungata non si attiva. Il rilevamento della pressione prolungata richiede che il tuo terminale invii eventi di ripetizione dei tasti, quindi non può rilevare un tasto premuto se la ripetizione dei tasti è disabilitata a livello del sistema operativo. Passa a modalità tocco con `/voice tap` per evitare il requisito di ripetizione dei tasti.
* **Toccare `Space` digita uno spazio invece di registrare in modalità tocco**: il primo tocco avvia la registrazione solo quando l'input del prompt è vuoto. Cancella prima l'input, oppure verifica di essere in modalità tocco eseguendo `/voice tap`.
* **`No audio detected from microphone`**: la registrazione è iniziata ma ha catturato il silenzio. Conferma che il dispositivo di input corretto è impostato come predefinito di sistema e che il suo livello di input non è disattivato o vicino a zero. Su Windows, apri Impostazioni → Sistema → Suono → Input e seleziona il tuo microfono. Su macOS, apri Impostazioni di sistema → Suono → Input.
* **`Voice connection failed`**: la tua registrazione non ha mai raggiunto il servizio di trascrizione perché la connessione è fallita. Controlla la tua rete e riprova. Una registrazione che non cattura alcun audio segnala `No audio detected from microphone` invece di questo messaggio. Prima della v2.1.200, un microfono silenzioso potrebbe segnalare un errore di connessione, che suggeriva un problema di rete quando il problema effettivo era il dispositivo di input.
* **`No speech detected`**: l'audio ha raggiunto il servizio di trascrizione ma nessuna parola è stata riconosciuta. Parla più vicino al microfono, riduci il rumore di fondo e conferma che la tua [lingua di dettatura](#change-the-dictation-language) corrisponda alla lingua che stai parlando.
* **La trascrizione è distorta o in una lingua sbagliata**: la dettatura predefinita è l'inglese. Se stai dettando in un'altra lingua, impostala prima in `/config`. Consulta [Cambia la lingua della dettatura](#change-the-dictation-language).

<h3 id="terminal-not-listed-in-macos-microphone-settings">
  Terminale non elencato nelle impostazioni del microfono di macOS
</h3>

Se la tua app terminale non appare in Impostazioni di sistema → Privacy e sicurezza → Microfono, non c'è alcun interruttore che puoi abilitare. Reimposta lo stato di autorizzazione per il tuo terminale in modo che la prossima esecuzione di `/voice` attivi un nuovo prompt di autorizzazione macOS.

<Steps>
  <Step title="Reimposta l'autorizzazione del microfono per il tuo terminale">
    Esegui `tccutil reset Microphone <bundle-id>`, sostituendo `<bundle-id>` con l'identificatore del tuo terminale: `com.apple.Terminal` per il Terminale integrato, o `com.googlecode.iterm2` per iTerm2. Per altri terminali, cerca l'identificatore con `osascript -e 'id of app "AppName"'`.

    <Warning>
      Puoi eseguire `tccutil reset Microphone` senza un ID bundle, ma revoca l'accesso al microfono da ogni app sul tuo Mac, incluse app come Zoom o Slack. Ogni app dovrà richiedere nuovamente l'accesso al prossimo utilizzo, quindi non eseguirlo durante una chiamata attiva.
    </Warning>
  </Step>

  <Step title="Esci e riavvia il tuo terminale">
    macOS non riproporrà un processo che è già in esecuzione. Esci dall'app terminale con Cmd+Q, non solo chiudere le sue finestre, quindi aprila di nuovo.
  </Step>

  <Step title="Attiva un nuovo prompt">
    Avvia Claude Code ed esegui `/voice`. macOS richiede l'accesso al microfono; consentilo.
  </Step>
</Steps>

<h2 id="see-also">
  Vedi anche
</h2>

* [Personalizza scorciatoie da tastiera](/docs/it/keybindings): riassegna `voice:pushToTalk` e altre azioni da tastiera della CLI
* [Configura impostazioni](/docs/it/settings): riferimento completo per `voice`, `language` e altre chiavi di impostazioni
* [Modalità interattiva](/docs/it/interactive-mode): scorciatoie da tastiera, modalità di input e controlli di sessione
* [Comandi](/docs/it/commands): riferimento per `/voice`, `/config` e tutti gli altri comandi
