> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Esegui il debug della tua configurazione

> Diagnostica perché CLAUDE.md, impostazioni, hooks, server MCP o skills non hanno effetto. Usa /context, /doctor, /hooks e /mcp per vedere cosa è stato effettivamente caricato.

Quando Claude ignora un'istruzione o una funzione che hai configurato non appare, la causa è solitamente che il file non è stato caricato, è stato caricato da una posizione diversa da quella prevista, o un altro file l'ha sovrascritto. Questa guida mostra come ispezionare cosa Claude Code ha effettivamente caricato in modo da poter restringere quale situazione si applica.

Per problemi di installazione, autenticazione e connettività, consulta invece [Troubleshooting installation and login](/docs/it/troubleshoot-install).

<h2 id="see-what-loaded-into-context">
  Vedi cosa è stato caricato nel contesto
</h2>

Il comando `/context` mostra tutto ciò che occupa la finestra di contesto per la sessione corrente, suddiviso per categoria: prompt di sistema, file di memoria, skills, subagenti personalizzati con la fonte da cui ciascuno è stato caricato, strumenti MCP e messaggi di conversazione. Eseguilo per primo per confermare se i tuoi `CLAUDE.md`, regole o descrizioni di skill sono presenti.

Per dettagli su una categoria specifica, segui con il comando dedicato:

| Comando          | Mostra                                                                                                                                                                                                                                                                                               |
| :--------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/memory`        | Quali file `CLAUDE.md` e rules sono stati caricati, più voci di memoria automatica                                                                                                                                                                                                                   |
| `/skills`        | Skills disponibili da fonti di progetto, utente e plugin                                                                                                                                                                                                                                             |
| `/hooks`         | Configurazioni di hook attive                                                                                                                                                                                                                                                                        |
| `/mcp`           | Server MCP connessi e il loro stato                                                                                                                                                                                                                                                                  |
| `/permissions`   | Regole di consentimento e negazione risolte attualmente in vigore                                                                                                                                                                                                                                    |
| `/doctor`        | Diagnostica della configurazione: salute dell'installazione, file di impostazioni non validi, estensioni inutilizzate, nomi di [subagent](/docs/it/sub-agents) duplicati nella stessa directory, e contenuto `CLAUDE.md` archiviato che Claude può derivare dalla base di codice, con correzioni proposte |
| `/debug [issue]` | Abilita la registrazione del debug per la sessione e richiede a Claude di diagnosticare utilizzando l'output del log e i percorsi delle impostazioni                                                                                                                                                 |
| `/status`        | Fonti di impostazioni attive, incluso se le impostazioni gestite sono in vigore                                                                                                                                                                                                                      |

Se un file di memoria manca da `/memory`, controlla la sua posizione rispetto a [come i file CLAUDE.md si caricano](/docs/it/memory#how-claude-md-files-load). I file `CLAUDE.md` della sottodirectory si caricano su richiesta quando Claude legge un file in quella directory con lo strumento Read, non all'inizio della sessione.

Se `/memory` conferma che il file è stato caricato ma Claude ancora non segue una particolare istruzione, il problema è probabilmente come l'istruzione è scritta piuttosto che se è stata caricata. CLAUDE.md funziona bene per il tipo di guida che daresti a un nuovo collega, come convenzioni di progetto, comandi di compilazione e dove appartengono i file.

L'aderenza diminuisce quando un'istruzione è abbastanza vaga da poter essere interpretata in più modi, quando due file danno indicazioni conflittuali, o quando il file è cresciuto abbastanza che le singole regole ricevono meno attenzione. [Scrivi istruzioni efficaci](/docs/it/memory#write-effective-instructions) copre i modelli di specificità, dimensione e struttura che mantengono l'aderenza alta.

<Note>
  CLAUDE.md e permissions risolvono problemi diversi. CLAUDE.md dice a Claude come funziona il tuo progetto in modo che prenda buone decisioni. [Permissions](/docs/it/permissions) e [hooks](/docs/it/hooks) applicano limiti indipendentemente da ciò che Claude decide. Usa CLAUDE.md per "lo facciamo così qui". Usa permissions o hooks per confini di sicurezza e qualsiasi cosa che non deve mai accadere, dove hai bisogno di una garanzia invece di una guida.
</Note>

<h2 id="check-resolved-settings">
  Controlla le impostazioni risolte
</h2>

Le impostazioni si uniscono tra gli ambiti gestiti, utente, progetto e locale. Le impostazioni gestite vincono sempre quando presenti. Tra il resto, l'ambito più vicino sostituisce quello più ampio nell'ordine locale, poi progetto, poi utente. Alcune impostazioni possono anche essere impostate da flag della riga di comando o [variabili di ambiente](/docs/it/env-vars), che agiscono come un altro livello di override. Quando un'impostazione non sembra applicarsi, il valore che hai impostato è solitamente sovrascritto da un altro ambito o da una variabile di ambiente.

Esegui `/doctor` per controllare la tua configurazione e installazione. Segnala quello che trova, inclusi file di impostazioni non validi, installazioni duplicate, estensioni inutilizzate e contenuto `CLAUDE.md` archiviato che Claude può derivare dalla base di codice, quindi propone correzioni che applica solo dopo che tu confermi. Il controllo di trim di `CLAUDE.md` richiede Claude Code v2.1.206 o successivo. Prima della v2.1.205, `/doctor` apriva una schermata diagnostica di sola lettura e premere `f` inviava il rapporto a Claude per correggerlo.

Dal terminale, `claude doctor` stampa diagnostica di installazione e impostazioni di sola lettura senza avviare una sessione.

Esegui `/status` per vedere quali fonti di impostazioni sono attive, incluso se le impostazioni gestite sono in vigore. Per capire quale ambito vince per una data chiave, vedi [Come gli ambiti interagiscono](/docs/it/settings#how-scopes-interact).

<h2 id="check-mcp-servers">
  Controlla i server MCP
</h2>

Esegui `/mcp` per vedere ogni server configurato, il suo stato di connessione e se l'hai approvato per il progetto corrente. Un server può essere definito correttamente ma comunque non fornire strumenti per alcuni motivi comuni:

* I server con ambito di progetto in `.mcp.json` richiedono un'approvazione una tantum. Se il prompt è stato chiuso, il server rimane disabilitato fino a quando non lo approvi da `/mcp`.
* Un server che non riesce ad avviarsi appare come non riuscito in `/mcp`. I percorsi di file relativi in `command` o `args` sono una causa frequente, poiché si risolvono rispetto alla directory da cui hai lanciato Claude Code piuttosto che alla posizione di `.mcp.json`.
* Un server che appare come connesso ma elenca zero strumenti si è avviato correttamente ma non sta restituendo un elenco di strumenti. Seleziona **Reconnect** da `/mcp`. Se il conteggio rimane a zero, esegui `claude --debug mcp` per vedere l'output stderr del server.

Per i percorsi di configurazione e le regole di ambito, vedi [MCP](/docs/it/mcp).

<h2 id="check-hooks">
  Controlla gli hooks
</h2>

Esegui `/hooks` per elencare ogni hook registrato per la sessione corrente, raggruppato per evento. Se un hook che hai definito non appare, non viene letto: gli hooks vanno sotto la chiave `"hooks"` in un file di impostazioni, non in un file autonomo.

Se l'hook appare ma non si attiva, il matcher è la causa usuale. Controllalo per questi errori:

* Il campo `matcher` è una singola stringa che usa `|` per corrispondere a più nomi di strumenti, ad esempio `"Edit|Write"`. Un separatore `,` è equivalente, quindi `"Edit,Write"` corrisponde agli stessi strumenti. Prima della v2.1.191, una virgola passava alla valutazione regex e il matcher non corrispondeva mai, quindi usa `|` se non sei ancora su v2.1.191.
* Un nome di strumento scritto male produce un matcher che non corrisponde a nulla, quindi l'hook fallisce silenziosamente.
* Un valore di array è un errore di schema: Claude Code mostra un avviso di errore di impostazioni e rifiuta l'intero file di impostazioni utente, progetto o locale, `claude doctor` segnala l'errore di convalida e nessun hook da quel file appare in `/hooks`. Nelle [impostazioni gestite](/docs/it/settings#settings-files), solo la voce non valida viene rimossa e gli altri hooks del file si applicano ancora.

Le modifiche a `settings.json` hanno effetto nella sessione in esecuzione dopo un breve ritardo di stabilità del file. Non è necessario riavviare. Se `/hooks` mostra ancora la definizione precedente alcuni secondi dopo il salvataggio, esegui `/hooks` di nuovo per aggiornare la visualizzazione.

Se `/hooks` mostra l'hook ma comunque non si attiva, il passo successivo è guardare la valutazione dell'hook dal vivo. Avvia una sessione con `claude --debug hooks` e attiva la chiamata dello strumento. Il log di debug registra ogni evento, quali matcher sono stati controllati e il codice di uscita e l'output dell'hook. Vedi [Debug hooks](/docs/it/hooks#debug-hooks) per il formato del log e [troubleshooting degli hooks](/docs/it/hooks-guide#limitations-and-troubleshooting) per i modelli di errore comuni.

<h2 id="test-against-a-clean-configuration">
  Prova con una configurazione pulita
</h2>

Inizia con [`claude --safe-mode`](/docs/it/cli-reference#cli-flags), che avvia una sessione con tutte le personalizzazioni disabilitate, inclusi `CLAUDE.md`, skills, plugins, hooks, server MCP e comandi e agenti personalizzati. L'autenticazione, la selezione del modello, gli strumenti integrati e le autorizzazioni funzionano normalmente. Se il problema scompare in modalità sicura, una di quelle superfici è la causa; usa i controlli mirati sopra per trovare quale. La modalità sicura applica comunque gli hooks gestiti e la policy delle impostazioni dalla vostra organizzazione. I plugin gestiti, gli skills, `CLAUDE.md` e i server MCP sono disattivati.

Se il problema persiste in modalità sicura, o le tue impostazioni stesse sono sospette, confronta con una sessione che non carica nulla dalla tua configurazione usuale. Punta [`CLAUDE_CONFIG_DIR`](/docs/it/env-vars) a una directory vuota per bypassare tutto sotto `~/.claude`, e avvia da una directory che non ha una cartella `.claude`, `.mcp.json` o `CLAUDE.md` in modo che la configurazione del progetto sia anche saltata.

```bash theme={null}
cd /tmp && CLAUDE_CONFIG_DIR=/tmp/claude-clean claude
```

La sessione pulita non ha impostazioni utente o progetto, hooks, server MCP, plugin o memoria.

* Le impostazioni gestite si applicano comunque se la tua organizzazione le distribuisce, poiché vivono in un percorso di sistema al di fuori di `~/.claude`
* Su Linux e Windows, ti verrà chiesto di accedere di nuovo perché le credenziali sono archiviate nella directory di configurazione
* Su macOS, le credenziali sono nel Keychain e si trasferiscono alla sessione pulita

Se il problema scompare qui, la causa è da qualche parte nei tuoi veri file `~/.claude` o progetto `.claude`. Reintroducili uno alla volta, copiando i file nella directory temporanea o avviando dal tuo progetto, per trovare quale. Se persiste nella sessione pulita, la causa è al di fuori della tua configurazione utente e progetto. Esegui `/status` per verificare se le impostazioni gestite sono in vigore, cerca [variabili di ambiente](/docs/it/env-vars) che influenzano Claude Code, quindi vedi [Risoluzione dei problemi](/docs/it/troubleshooting).

<h2 id="check-common-causes">
  Controlla le cause comuni
</h2>

La maggior parte delle sorprese di configurazione risale a un piccolo insieme di regole di posizione e sintassi. Controlla questi prima di assumere un bug:

| Sintomo                                                                    | Causa                                                                                                                                                        | Soluzione                                                                                                                                                                                                                                                                                  |
| :------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Hook non si attiva mai                                                     | `matcher` è un array JSON invece di una stringa                                                                                                              | Usa una singola stringa con `\|` per corrispondere a più strumenti, ad esempio `"Edit\|Write"`. Vedi [matcher patterns](/docs/it/hooks#matcher-patterns).                                                                                                                                       |
| Hook non si attiva mai                                                     | `matcher` utilizza `,` come separatore su una versione precedente a v2.1.191                                                                                 | Claude Code v2.1.191 o successivo tratta `,` come separatore di elenco come `\|`. Le versioni precedenti valutano una virgola come carattere letterale, quindi `"Edit,Write"` non corrisponde a nulla. Usa `\|` invece, o aggiorna Claude Code.                                            |
| Hook non si attiva mai                                                     | Il valore di `matcher` è minuscolo, ad esempio `"bash"`                                                                                                      | La corrispondenza è sensibile alle maiuscole. I nomi degli strumenti sono capitalizzati: `Bash`, `Edit`, `Write`, `Read`.                                                                                                                                                                  |
| Hook non si attiva mai                                                     | Gli hooks sono definiti in un file autonomo invece di `settings.json`                                                                                        | Non esiste un file di hooks autonomo per la configurazione del progetto o dell'utente. Definisci gli hooks sotto la chiave `"hooks"` in `settings.json`. Solo i [plugins](/docs/it/plugins-reference#hooks) caricano un file separato `hooks/hooks.json`. Vedi [hook configuration](/docs/it/hooks). |
| Permissions, hooks o env impostati globalmente vengono ignorati            | La configurazione è stata aggiunta a `~/.claude.json`                                                                                                        | `~/.claude.json` contiene lo stato dell'app e gli interruttori dell'interfaccia utente. `permissions`, `hooks` e `env` appartengono a `~/.claude/settings.json`. Questi sono due file diversi.                                                                                             |
| Un valore di `settings.json` sembra ignorato                               | La stessa chiave è impostata in `settings.local.json`                                                                                                        | `settings.local.json` sostituisce `settings.json`, e entrambi sostituiscono `~/.claude/settings.json`. Vedi [settings precedence](/docs/it/settings#how-scopes-interact).                                                                                                                       |
| Skill non appare in `/skills`                                              | Il file di skill è in `.claude/skills/name.md` invece che in una cartella                                                                                    | Usa una cartella con `SKILL.md` dentro: `.claude/skills/name/SKILL.md`.                                                                                                                                                                                                                    |
| Skill appare in `/skills` ma Claude non la invoca mai                      | Skill ha `disable-model-invocation: true` nel suo frontmatter, o la sua descrizione non corrisponde a come formuli la richiesta                              | Controlla il badge in `/skills`: un'etichetta "user-only" significa che Claude non la attiverà da sola. Vedi [skill invocation](/docs/it/skills).                                                                                                                                               |
| Le istruzioni di `CLAUDE.md` della sottodirectory sembrano ignorate        | I file della sottodirectory si caricano su richiesta, non all'inizio della sessione                                                                          | Si caricano quando Claude legge un file in quella directory con lo strumento Read, non al lancio e non quando scrive o crea file lì. Vedi [come i file CLAUDE.md si caricano](/docs/it/memory#how-claude-md-files-load).                                                                        |
| Subagent ignora le istruzioni di `CLAUDE.md`                               | Gli agenti Explore e Plan incorporati saltano `CLAUDE.md`. I subagenti personalizzati lo caricano nello stesso modo in cui la conversazione principale lo fa | Per Explore o Plan, ripeti l'istruzione nel tuo prompt di delega. Per un subagente personalizzato, metti le istruzioni critiche nel corpo del file dell'agente, che diventa il prompt di sistema dell'agente. Vedi [cosa si carica all'avvio](/docs/it/sub-agents#what-loads-at-startup).       |
| La logica di pulizia non viene mai eseguita alla fine della sessione       | Nessun hook `SessionEnd` configurato                                                                                                                         | Aggiungi un hook `SessionEnd` in `settings.json`. Vedi l'[elenco degli eventi di hook](/docs/it/hooks#hook-events).                                                                                                                                                                             |
| I server MCP in `.mcp.json` non si caricano mai                            | Il file è sotto `.claude/` o utilizza il formato di configurazione di Claude Desktop                                                                         | La configurazione MCP del progetto va alla radice del repository come `.mcp.json`, non dentro `.claude/`. Vedi [MCP configuration](/docs/it/mcp).                                                                                                                                               |
| Server MCP aggiunti sotto `mcpServers` in `settings.json` non appaiono mai | `settings.json` non legge una chiave `mcpServers`                                                                                                            | Definisci i server del progetto in `.mcp.json` alla radice del repository, o esegui `claude mcp add --scope user` per i server con ambito utente. Vedi [MCP configuration](/docs/it/mcp).                                                                                                       |
| Server MCP del progetto aggiunto ma non appare                             | Il prompt di approvazione una tantum è stato chiuso                                                                                                          | I server con ambito di progetto richiedono approvazione. Esegui `/mcp` per vedere lo stato e approvare.                                                                                                                                                                                    |
| Il server MCP non riesce ad avviarsi da alcune directory                   | `command` o `args` utilizza un percorso di file relativo                                                                                                     | Usa percorsi assoluti per gli script locali. Gli eseguibili sul tuo `PATH` come `npx` o `uvx` funzionano così come sono.                                                                                                                                                                   |
| Il server MCP si avvia senza le variabili di ambiente previste             | Le variabili sono in `settings.json` `env`, che non si propaga ai processi figlio MCP                                                                        | Imposta invece `env` per server in `.mcp.json`.                                                                                                                                                                                                                                            |
| La regola di negazione `Bash(rm *)` non blocca `/bin/rm` o `find -delete`  | Le regole di prefisso corrispondono alla stringa di comando letterale, non all'eseguibile sottostante                                                        | Aggiungi modelli espliciti per ogni variante, o usa un [PreToolUse hook](/docs/it/hooks-guide) o la [sandbox](/docs/it/sandboxing) per una garanzia difficile.                                                                                                                                       |

<h2 id="related-resources">
  Risorse correlate
</h2>

Per il riferimento completo su ogni superficie di configurazione, vedi la pagina dedicata:

* **[Riferimento della directory `.claude`](/docs/it/claude-directory)**: ogni posizione del file di configurazione e cosa lo legge
* **[Settings](/docs/it/settings)**: ordine di precedenza e l'elenco completo delle chiavi
* **[Riferimento degli hooks](/docs/it/hooks)**: nomi degli eventi, payload e formato di output `--debug hooks`
* **[MCP](/docs/it/mcp)**: configurazione del server, approvazione e output `/mcp`
* **[Troubleshooting installation and login](/docs/it/troubleshoot-install)**: `command not found`, PATH e problemi di autenticazione
* **[Risoluzione dei problemi](/docs/it/troubleshooting)**: prestazioni, blocchi e problemi di ricerca
