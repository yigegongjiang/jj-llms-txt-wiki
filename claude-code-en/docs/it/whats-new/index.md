> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Novità

> Un digest settimanale delle notevoli funzionalità di Claude Code, con frammenti di codice, demo e contesto su perché sono importanti.

Il digest settimanale per sviluppatori evidenzia le funzionalità più probabili a cambiare il modo in cui lavorate. Ogni voce include codice eseguibile, una breve demo e un collegamento alla documentazione completa. Per ogni correzione di bug e miglioramento minore, consultate il [changelog](/docs/it/changelog).

<Update label="Week 28" description="July 6–10, 2026" tags={["v2.1.202–v2.1.206"]}>
  **Browser in-app su Desktop**: Claude Code su desktop ottiene un browser integrato, quindi Claude può aprire documenti, design o qualsiasi altro sito e interagire con le pagine nello stesso modo in cui lo fa con le vostre anteprime del server di sviluppo locale.

  Anche questa settimana: **`/doctor`** è un controllo di configurazione completo che diagnostica i problemi e può risolverli, con `/checkup` come suo alias; **auto mode** blocca la manomissione della trascrizione e chiede prima di `rm -rf` su variabili non risolte; e **agent view rows** mostrano una parola di stato colorata e un titolo scritto da un classificatore.

  [Leggi il digest della Week 28 →](/docs/it/whats-new/2026-w28)
</Update>

<Update label="Week 27" description="June 29 – July 3, 2026" tags={["v2.1.195–v2.1.201"]}>
  **Claude Sonnet 5**: il nuovo modello predefinito per i posti di abbonamento Pro, Team Standard ed Enterprise, con codifica di livello superiore e utilizzo di strumenti al prezzo di Sonnet, una finestra di contesto nativa di 1M token e thinking adattivo attivato per impostazione predefinita.

  Anche questa settimana: **Claude in Chrome** è generalmente disponibile su tutti i piani Anthropic diretti; **i subagent vengono eseguiti in background per impostazione predefinita** quindi Claude continua a lavorare mentre vengono eseguiti; **Claude Desktop su Linux** arriva in beta su Ubuntu e Debian; e **`/radio`** si sintonizza su Claude FM lo-fi radio.

  [Leggi il digest della Week 27 →](/docs/it/whats-new/2026-w27)
</Update>

<Update label="Week 26" description="June 22–26, 2026" tags={["v2.1.185–v2.1.193"]}>
  **`claude mcp login`**: autenticate un server MCP configurato dalla vostra shell invece del menu interattivo `/mcp`, e cancellate le sue credenziali archiviate in seguito con `claude mcp logout`.

  Anche questa settimana: **shell mode risponde all'output dei comandi** (`! npm test` riceve una spiegazione senza un secondo prompt); **`/rewind`** può riprendere una conversazione da prima che `/clear` fosse eseguito; e **background subagents** ora mostrano i prompt di autorizzazione nella sessione principale invece di negarli automaticamente.

  [Leggi il digest della Week 26 →](/docs/it/whats-new/2026-w26)
</Update>

<Update label="Week 25" description="June 15–19, 2026" tags={["v2.1.178–v2.1.183"]}>
  **Artifacts**: trasformate l'output di una sessione in una pagina live e condivisibile su claude.ai che si aggiorna sul posto mentre la sessione lavora, ora in beta su piani Team e Enterprise.

  Anche questa settimana: **regole di deny e ask corrispondono ai parametri dello strumento** con `Tool(param:value)`, ad esempio `Agent(model:opus)`; **`/config key=value`** imposta qualsiasi impostazione dal prompt, in modalità `-p`, e da Remote Control; e **auto mode blocca i comandi git distruttivi** quando non avete chiesto di scartare il lavoro locale.

  [Leggi il digest della Week 25 →](/docs/it/whats-new/2026-w25)
</Update>

<Update label="Week 24" description="June 8–12, 2026" tags={["v2.1.166–v2.1.176"]}>
  **`/cd`**: sposta la sessione corrente in una nuova directory di lavoro a metà conversazione senza ricostruire la cache del prompt.

  Anche questa settimana: **i sub-agent possono generare i propri sub-agent** (le catene di background sono limitate a cinque livelli di profondità); **`--safe-mode`** avvia Claude Code con tutte le personalizzazioni disabilitate per la risoluzione dei problemi; e **`fallbackModel`** configura fino a tre modelli di fallback provati in ordine.

  [Leggi il digest della Week 24 →](/docs/it/whats-new/2026-w24)
</Update>

<Update label="Week 23" description="June 1–5, 2026" tags={["v2.1.158–v2.1.165"]}>
  **Auto mode su Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry**: auto mode è ora disponibile su provider di terze parti per Opus 4.7 e Opus 4.8, sostituendo i prompt di autorizzazione con controlli di sicurezza in background.

  Anche questa settimana: **modifiche automatiche più sicure** richiedono conferma prima di scrivere file che possono eseguire codice in modalità `acceptEdits`; **`/plugin list`** stampa i vostri plugin installati inline; e **requisiti di versione** consentono alle distribuzioni gestite di richiedere un intervallo di versione di Claude Code approvato.

  [Leggi il digest della Week 23 →](/docs/it/whats-new/2026-w23)
</Update>

<Update label="Week 22" description="May 25–29, 2026" tags={["v2.1.150–v2.1.157"]}>
  **Claude Opus 4.8**: il nuovo modello predefinito per Max, Team Premium, Enterprise pay-as-you-go e account Anthropic API, con sforzo elevato per impostazione predefinita e `/effort xhigh` per i compiti più difficili.

  Anche questa settimana: **dynamic workflows** orchestrano da decine a centinaia di subagent da uno script che Claude scrive; il **security-guidance plugin** esamina i cambiamenti di Claude per le vulnerabilità mentre lavora; e **fast mode** viene eseguito su Opus 4.8 a \$10/\$50 per MTok.

  [Leggi il digest della Week 22 →](/docs/it/whats-new/2026-w22)
</Update>

<Update label="Week 21" description="May 18–22, 2026" tags={["v2.1.143–v2.1.149"]}>
  **Auto mode sul piano Pro**: auto mode ora viene eseguito su account Pro e supporta Sonnet 4.6 insieme a Opus, sostituendo i prompt di autorizzazione con controlli di sicurezza in background.

  Anche questa settimana: **`/usage`** suddivide cosa guida i vostri limiti di piano per skill, subagent, plugin e server MCP; il nuovo comando **`/code-review`** segnala bug di correttezza; e **background sessions** appaiono in `/resume` e rimangono attive quando fissate.

  [Leggi il digest della Week 21 →](/docs/it/whats-new/2026-w21)
</Update>

<Update label="Week 20" description="May 11–15, 2026" tags={["v2.1.139–v2.1.142"]}>
  **Visualizzazione agente**: `claude agents` apre una schermata per ogni sessione di Claude Code, mostrando cosa è in esecuzione, cosa è bloccato in attesa di voi e cosa è completato.

  Anche questa settimana: **`/goal`** mantiene Claude al lavoro tra i turni finché non si verifica una condizione di completamento; **fast mode** ora viene eseguito su Opus 4.7 per impostazione predefinita; e il **menu Rewind** può comprimere il contesto precedente con "Summarize up to here".

  [Leggi il digest della Week 20 →](/docs/it/whats-new/2026-w20)
</Update>

<Update label="Week 19" description="May 4–8, 2026" tags={["v2.1.128–v2.1.136"]}>
  **I plugin si caricano da archivi `.zip` e URL**: `--plugin-dir` ora accetta file `.zip`, e `--plugin-url` recupera un archivio plugin per la sessione corrente.

  Anche questa settimana: **`worktree.baseRef`** sceglie se i nuovi worktree si diramano dal default remoto o dal `HEAD` locale; **regole di hard deny in auto mode** bloccano le azioni incondizionatamente indipendentemente dalle eccezioni di autorizzazione; e **i hook vedono il livello di sforzo attivo** tramite `effort.level` e `$CLAUDE_EFFORT`.

  [Leggi il digest della Week 19 →](/docs/it/whats-new/2026-w19)
</Update>

<Update label="Week 18" description="April 27 – May 1, 2026" tags={["v2.1.120–v2.1.126"]}>
  **Windows senza Git Bash**: Git per Windows non è più richiesto, e Claude Code utilizza PowerShell come strumento shell quando Bash è assente.

  Anche questa settimana: **`claude ultrareview`** porta la revisione del codice cloud a CI e script; **`claude project purge`** pulisce lo stato locale per un progetto; e incollare un **URL PR in `/resume`** trova la sessione che l'ha creato.

  [Leggi il digest della Week 18 →](/docs/it/whats-new/2026-w18)
</Update>

<Update label="Week 17" description="April 20–24, 2026" tags={["v2.1.114–v2.1.119"]}>
  **`/ultrareview`** si apre come anteprima di ricerca pubblica: una flotta di agenti cacciatori di bug viene eseguita nel cloud e i risultati tornano automaticamente nel vostro CLI o Desktop.

  Anche questa settimana: **session recap** vi mostra cosa è successo mentre un terminale era non focalizzato; **custom themes** vi permettono di costruire e distribuire tavolozze di colori da `/theme` o da un plugin; e **Claude Code sul web** riceve una riprogettazione con una nuova barra laterale delle sessioni e un layout drag-and-drop.

  [Leggi il digest della Week 17 →](/docs/it/whats-new/2026-w17)
</Update>

<Update label="Week 16" description="April 13–17, 2026" tags={["v2.1.105–v2.1.113"]}>
  **Claude Opus 4.7** arriva come nuovo predefinito su Max e Team Premium, con un nuovo livello di sforzo `xhigh` che è l'impostazione consigliata per la maggior parte del lavoro di codifica e uno slider interattivo `/effort` per regolarlo.

  Anche questa settimana: **Routines** su Claude Code sul web attivano agenti cloud templati da una pianificazione, un evento GitHub o una chiamata API; **notifiche push mobile** vi avvisano sul telefono quando un'attività lunga termina o Claude ha bisogno di voi; `/usage` mostra cosa sta guidando i vostri limiti; e il CLI si sposta su binari nativi.

  [Leggi il digest della Week 16 →](/docs/it/whats-new/2026-w16)
</Update>

<Update label="Week 15" description="April 6–10, 2026" tags={["v2.1.92–v2.1.101"]}>
  **Ultraplan** entra in anteprima iniziale: bozza un piano nel cloud dal vostro CLI, esaminate e commentate in un editor web, quindi eseguitelo in remoto o riportatelo in locale. La prima esecuzione ora crea automaticamente un ambiente cloud per voi.

  Anche questa settimana: lo strumento **Monitor** trasmette gli eventi di background nella conversazione in modo che Claude possa seguire i log e reagire in tempo reale, `/loop` si auto-regola quando omettete l'intervallo, `/team-onboarding` pacchetto la vostra configurazione in una guida riproducibile, e `/autofix-pr` attiva l'auto-fix PR dal vostro terminale.

  [Leggi il digest della Week 15 →](/docs/it/whats-new/2026-w15)
</Update>

<Update label="Week 14" description="March 30 – April 3, 2026" tags={["v2.1.86–v2.1.91"]}>
  **Computer use** arriva al CLI in anteprima di ricerca: Claude può aprire app native, fare clic attraverso l'interfaccia utente e verificare le modifiche dal vostro terminale. Migliore per chiudere il ciclo su cose che solo una GUI può verificare.

  Anche questa settimana: lezioni interattive `/powerup`, rendering alt-screen senza sfarfallio, un override della dimensione del risultato MCP per strumento fino a 500K, e eseguibili plugin sul `PATH` dello strumento Bash.

  [Leggi il digest della Week 14 →](/docs/it/whats-new/2026-w14)
</Update>

<Update label="Week 13" description="March 23–27, 2026" tags={["v2.1.83–v2.1.85"]}>
  **Auto mode** arriva in anteprima di ricerca: un classificatore gestisce i vostri prompt di autorizzazione in modo che le azioni sicure vengono eseguite senza interruzione e quelle rischiose vengono bloccate. La via di mezzo tra approvare tutto e `--dangerously-skip-permissions`.

  Anche questa settimana: computer use nell'app Desktop, PR auto-fix su Web, ricerca di trascritti con `/`, uno strumento PowerShell nativo per Windows, e hook `if` condizionali.

  [Leggi il digest della Week 13 →](/docs/it/whats-new/2026-w13)
</Update>
