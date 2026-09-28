> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Personalizzare le sessioni negli ambienti self-hosted

> Personalizzare le sessioni degli ambienti self-hosted con script wrapper per credenziali per sessione, hook del ciclo di vita e spawning di runner su richiesta.

<Note>
  Gli ambienti self-hosted sono in beta pubblica sui piani Team ed Enterprise; un [Owner](/docs/it/cloud-environments#organization-shared-environments) li abilita attivando **Allow self-hosted environments** nella [pagina di amministrazione **Cloud environments**](https://claude.ai/admin-settings/cloud-environments). Questa pagina presuppone un runner funzionante; consultare la [guida rapida](/docs/it/self-hosted-environments-quickstart) per la configurazione e [Deploy to production](/docs/it/self-hosted-environments-deploy) per le ricette della flotta.
</Note>

Un [ambiente self-hosted](/docs/it/self-hosted-environments) esegue le [sessioni cloud](/docs/it/claude-code-on-the-web) di Claude Code sulla propria infrastruttura, eseguite da un processo runner che si distribuisce. Senza configurazione, quel runner clona il repository della sessione, avvia Claude Code e pulisce. Questa pagina è per l'ingegnere della piattaforma che gestisce i runner: copre i punti di estensione per quando questi valori predefiniti non si adattano, dal provisioning delle credenziali per sessione alla sostituzione completa del checkout. I wrapper e gli hook vengono eseguiti come file eseguibili sull'host del runner, che è Linux o macOS, e gli esempi su questa pagina presuppongono una shell POSIX.

Alcune variabili di ambiente hook su questa pagina utilizzano ancora `pool`, come `CLAUDE_RUNNER_POOL_ID`; i nomi dei flag CLI e delle variabili di ambiente utilizzano `environment`, come `--environment-secret-file`.

<h2 id="wrapper-scripts">
  Script wrapper
</h2>

Utilizzare uno script wrapper quando ogni sessione ha bisogno di una configurazione che il runner non può fare da solo: provisioning di credenziali di breve durata limitate al creatore della sessione, esportazione di segreti specifici dell'ambiente, preparazione di toolchain di linguaggio o applicazione di limiti di risorse attorno al processo figlio. Il runner avvia il wrapper al posto del binario Claude Code, una volta per sessione. Terminare il wrapper con `exec` in `$CLAUDE_RUNNER_CLAUDE_BIN`, il binario del runner stesso, in modo che i segnali e i codici di uscita si propaghino correttamente.

Puntare `--exec-path`, o `SELF_HOSTED_RUNNER_EXEC_PATH`, al wrapper quando si avvia il runner:

```bash theme={null}
claude self-hosted-runner --environment-secret-file /etc/claude/environment-secret --exec-path /etc/claude/session-wrapper.sh
```

Il runner imposta quanto segue nell'ambiente del wrapper:

| Variabile                           | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :---------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN`  | Il JWT della sessione, con prefisso `sk-ant-cc-`. Il suo claim `act` identifica il creatore della sessione, con l'email del creatore e il soggetto del provider di identità upstream quando la superficie di creazione li ha registrati. Il valore è il token al momento dello spawn; gli aggiornamenti arrivano sullo stdin del figlio, quindi un wrapper vede solo il valore iniziale. Consultare [Verify session identity](/docs/it/self-hosted-environments-identity).                                                                                                                                                                                                                                                                   |
| `CCR_SESSION_ACCOUNT_EMAIL`         | L'email del creatore della sessione, pre-estratta dal runner dal claim `act.email` del token senza verifica della firma. Adatto per l'etichettatura, come i trailer dei commit. Quando l'email controlla il rilascio delle credenziali, verificare il token e leggere il claim da esso; consultare [Provision credentials scoped to the session creator](#provision-credentials-scoped-to-the-session-creator). Non impostato quando il token non contiene un'email del creatore. Trattare come informazioni personali identificabili.                                                                                                                                                                                                  |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`     | La superficie client che ha creato la sessione, come `web_claude_ai`, `desktop_app`, `ios`, `claude_code_cli` o `scheduled_trigger`. Anthropic registra il valore una volta alla creazione della sessione, quindi il wrapper e ogni hook del ciclo di vita vedono lo stesso valore. Utilizzarlo solo per l'analisi dell'adozione e l'etichettatura, non come segnale di autorizzazione. Non impostato quando la sessione non ha una superficie registrata o riconosciuta, quindi fare riferimento ad esso come `${CLAUDE_RUNNER_CLIENT_PLATFORM:-}` sotto `set -u`. Richiede Claude Code v2.1.229 o successivo.                                                                                                                         |
| `CLAUDE_RUNNER_CLAUDE_BIN`          | Percorso assoluto al binario Claude Code del runner stesso. Terminare il wrapper con `exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"` per passare al binario bloccato senza hardcodificare un percorso di installazione.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `CLAUDE_CODE_REMOTE_SESSION_ID`     | ID sessione nel modulo taggato `cse_...`. Questa è la stessa sessione che gli [hook del ciclo di vita](#lifecycle-hooks) vedono come `CLAUDE_RUNNER_SESSION_ID` nel modulo `session_...`; le variabili UUID corrispondono su entrambi, e sostituire il prefisso `cse_` con `session_` produce l'ID mostrato nell'URL della sessione.                                                                                                                                                                                                                                                                                                                                                                                                    |
| `CLAUDE_CODE_REMOTE_SESSION_UUID`   | Lo stesso ID sessione nel modulo UUID canonico, per i sistemi che si basano su UUID.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `CLAUDE_SESSION_INGRESS_TOKEN_FILE` | Percorso assoluto a un file per sessione che contiene il JWT della sessione corrente, mantenuto aggiornato negli aggiornamenti dei token. I sottoprocessi della shell lo leggono per il loro header `Authorization` quando scaricano gli allegati che l'utente ha aggiunto alla sessione. `exec` preserva la variabile automaticamente; un wrapper che ricostruisce l'ambiente del figlio deve portare la variabile, o i download degli allegati si interrompono silenziosamente.                                                                                                                                                                                                                                                       |
| `CLAUDE_CONFIG_DIR`                 | Directory di configurazione Claude per sessione, scritta all'inizio della sessione dallo snapshot della configurazione dell'host del runner che il runner acquisisce all'avvio; consultare [Permissions and tool approval](#permissions-and-tool-approval). Le scritture qui sono isolate a questa sessione. La directory rimane sotto `<base-dir>/_sessions/` dopo la fine della sessione a meno che non si avvii il runner con [`--remove-session-state`](/docs/it/self-hosted-environments-reference#runner-cli-flags); consultare [Reuse a pre-warmed checkout](/docs/it/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout).                                                                                                        |
| `ANTHROPIC_BASE_URL`                | L'URL di base dell'API che il figlio utilizzerà, fornito dal piano di controllo per sessione e normalmente `https://api.anthropic.com`. Non sovrascriverlo: la credenziale di inferenza della sessione è un token OAuth emesso da Anthropic che altri provider non accettano, quindi l'inferenza negli ambienti self-hosted non è instradabile altrove.                                                                                                                                                                                                                                                                                                                                                                                 |
| `CLAUDE_CODE_OAUTH_TOKEN`           | Il token di accesso OAuth di breve durata che il figlio utilizza per l'inferenza del modello, limitato solo all'inferenza del modello e al caricamento di file, con una durata di circa 30 minuti. Il runner lo ricrea prima della scadenza e fornisce la rotazione sullo stdin del figlio, quindi un wrapper che non [mantiene stdin allegato](#keep-stdin-and-file-descriptor-3-attached) vede solo il valore iniziale. Non fare affidamento sulla lista di indirizzi IP della propria organizzazione per limitare l'uso di questo token: trattarlo come una credenziale bearer che rimane utilizzabile per circa 30 minuti se trapela, e non registrarlo, scriverlo su disco o inoltarlo al di fuori del contenitore della sessione. |

Il wrapper eredita anche il resto dell'ambiente gestito del figlio, incluse tutte le variabili di ambiente fornite dal server. `exec` le propaga tutte automaticamente; se il wrapper avvia il figlio in un altro modo, inoltrare l'ambiente completo.

<h3 id="keep-stdin-and-file-descriptor-3-attached">
  Mantenere stdin e il descrittore di file 3 allegati
</h3>

Lo stdin del figlio è il canale di controllo del runner. Gli aggiornamenti dei token e i segnali di fine sessione arrivano su di esso. Il runner apre anche una pipe sul descrittore di file 3 e legge i segnali di attività del figlio da esso per guidare i timeout di inattività e avvio. Un semplice `exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"` preserva entrambi automaticamente.

Se il wrapper mette in background il figlio con un semplice `&`, interrompe lo stdin del figlio: la sessione sembra sana fino a quando la durata di circa 30 minuti del token OAuth iniziale non scade, quindi ogni chiamata API fallisce con `401 authentication_error`. Se il wrapper deve mettere in background il figlio, ad esempio per mantenere vivo un trap di teardown, salvare stdin sul descrittore di file 4 o superiore e ricollegarlo esplicitamente:

```bash theme={null}
exec 4<&0
"$CLAUDE_RUNNER_CLAUDE_BIN" "$@" <&4 4<&- &
CHILD=$!
trap 'teardown' EXIT
wait "$CHILD"
```

Non chiudere o riutilizzare il descrittore di file 3 nel wrapper. Il reindirizzamento dello stdout e dello stderr del figlio va bene.

<h3 id="provision-credentials-scoped-to-the-session-creator">
  Provisioning di credenziali limitate al creatore della sessione
</h3>

Utilizzare il subcomando `decode-token` per leggere i claim dal JWT della sessione. Legge il token da un argomento, da `CLAUDE_CODE_SESSION_ACCESS_TOKEN` o da stdin, in quell'ordine; consultare [Verify the token inside the session](/docs/it/self-hosted-environments-identity#verify-the-token-inside-the-session) per ciò che controlla. L'esempio seguente decodifica l'identità del creatore, la scambia con credenziali AWS di breve durata e fa exec in Claude Code:

```bash theme={null}
#!/bin/bash
# Basarsi sull'ID utente Anthropic stabile e richiedere un creatore umano.
CREATOR_SUB=$("$CLAUDE_RUNNER_CLAUDE_BIN" self-hosted-runner decode-token \
  | jq -re '.act.sub // "" | select(startswith("user:"))') \
  || { echo "decode-token: verification failed or no human creator" >&2; exit 1; }

creds=$(your-sts-helper assume-role --subject "$CREATOR_SUB") \
  || { echo "credential exchange failed" >&2; exit 1; }
eval "$creds"

exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"
```

Utilizzare `jq -re` piuttosto che `jq -r` quando il claim estratto controlla una decisione di autenticazione, in modo che un claim assente esca con codice diverso da zero invece di passare la stringa letterale `null` a valle. Le sessioni create da un'identità di servizio dell'organizzazione, come sessioni di bot e agenti, portano un soggetto `agent:` piuttosto che `user:`, quindi questo esempio le rifiuta; se l'ambiente serve quelle sessioni, decidere esplicitamente se il wrapper ricade a una credenziale predefinita per loro invece di uscire. Quando lo scambio di credenziali ha bisogno del soggetto SSO o dell'email, leggere `.act.attested_by.sub` o `.act.email` e gestire la loro assenza: il token li porta solo quando la superficie di creazione li ha registrati, e una [sessione inviata da CLI](/docs/it/self-hosted-environments-testing#run-the-test-loop) può mancare di entrambi. Per il riferimento completo dei claim e la verifica da servizi al di fuori del runner, consultare [Verify session identity](/docs/it/self-hosted-environments-identity).

<h2 id="lifecycle-hooks">
  Hook del ciclo di vita
</h2>

Gli hook del ciclo di vita sostituiscono le fasi della pipeline per sessione del runner con i propri script. Puntare il runner a una directory di hook con `--hooks-dir <path>`, o `SELF_HOSTED_RUNNER_HOOKS_DIR`. Il runner cerca file eseguibili con nomi ben noti; qualsiasi hook che non è presente ricade nel comportamento integrato, quindi si scrivono solo quelli di cui si ha bisogno. Gli hook vengono eseguiti con i privilegi del runner stesso, e i figli della sessione condividono quel UID, quindi montare la directory degli hook in sola lettura, o cuocerla nell'immagine, in modo che il codice della sessione non possa modificarla; consultare la [sezione di hardening](/docs/it/self-hosted-environments-deploy#harden-your-deployment).

Questi hook sono distinti dagli [hook di Claude Code](/docs/it/hooks), che vengono eseguiti all'interno della sessione; gli hook del ciclo di vita vengono eseguiti sul runner, attorno alla sessione.

<h3 id="checkout">
  checkout
</h3>

Viene eseguito una volta per repository, al posto del clone e del fetch integrati del runner. Utilizzare l'hook per clonare da uno specchio di lettura, seminare un albero di lavoro da un archivio o applicare l'autenticazione git per sessione. Il runner imposta:

| Variabile                          | Descrizione                                                                                                                                                                    |
| :--------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_RUNNER_REPO_URL`           | URL del repository da clonare, dopo che `--git-host-rewrite` e `--git-ssh-rewrite` sono stati applicati                                                                        |
| `CLAUDE_RUNNER_REPO_REF`           | Revisione da controllare: ramo, tag o commit SHA come la sessione lo ha richiesto. Vuoto significa il ramo predefinito del repository.                                         |
| `CLAUDE_RUNNER_CHECKOUT_PATH`      | Percorso assoluto dove l'albero di lavoro deve essere lasciato                                                                                                                 |
| `CLAUDE_RUNNER_SESSION_ID`         | ID sessione nel modulo taggato `session_...`, per la registrazione e la correlazione                                                                                           |
| `CLAUDE_RUNNER_SESSION_UUID`       | Lo stesso ID sessione nel modulo UUID canonico                                                                                                                                 |
| `CLAUDE_RUNNER_API_BASE_URL`       | URL di base dell'API Anthropic per le chiamate limitate alla sessione                                                                                                          |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`    | La superficie client che ha creato la sessione, come `web_claude_ai`, `desktop_app` o `ios`. Non impostato quando la sessione non ha una superficie registrata o riconosciuta. |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN` | Il token di accesso della sessione, per le chiamate API limitate alla sessione                                                                                                 |

Lo script deve lasciare un albero di lavoro in `CLAUDE_RUNNER_CHECKOUT_PATH` controllato alla revisione richiesta. HEAD staccato va bene; il runner crea il ramo di lavoro della sessione in cima. Il runner verifica che il percorso contenga un `.git` in seguito; se l'hook materializza una fonte non-git come Perforce o un tarball scompattato, impostare `CLAUDE_RUNNER_SKIP_GIT_VERIFY=1` nell'ambiente del runner per saltare quel controllo. I flussi basati su Git come la creazione del ramo di lavoro e il push dei risultati richiedono un checkout git, quindi esportare i risultati da alberi non-git con un hook [`post-session`](#post-session).

Il runner non passa una credenziale git all'hook. Invece, coniare una credenziale di clone per sessione dall'identità della sessione: verificare `CLAUDE_CODE_SESSION_ACCESS_TOKEN` con una libreria JWT standard rispetto all'endpoint JWKS sotto `CLAUDE_RUNNER_API_BASE_URL`, come descritto in [Verify the token from your service](/docs/it/self-hosted-environments-identity#verify-the-token-from-your-service), quindi fare in modo che il servizio di credenziale emetta una credenziale di clone di breve durata per l'identità nel claim `act` del token. `CLAUDE_RUNNER_CLAUDE_BIN` non è impostato nell'ambiente dell'hook di checkout, quindi il subcomando `decode-token` non è disponibile qui. Ricadere in qualsiasi autenticazione git che l'host ha già, come un agente SSH, un helper di credenziale o `.netrc`, è anche un'opzione.

Quando l'hook esce con codice diverso da zero, o esce 0 senza lasciare un checkout utilizzabile dietro, ciò che il runner fa dipende dal repository:

* **Un repository a cui la sessione spinge i risultati**: il runner fallisce la sessione e su un'uscita diversa da zero mostra la coda dello stderr dello script all'utente.
* **Un repository che la sessione legge solo**, come un repository aggiunto a una sessione in esecuzione: il runner registra una riga `[runner:warn]` con il dettaglio del fallimento, pubblica un passo `Skipped` alla sessione, rimuove ciò che l'hook ha lasciato al percorso di checkout e continua con i repository rimanenti. Quando il runner non può rimuovere il percorso immediatamente, ritenta la rimozione alla fine della sessione. Se saltare lascia la sessione senza alcun repository, il runner fallisce comunque la sessione.

Prima della v2.1.228, il runner falliva la sessione su un fallimento dell'hook per qualsiasi repository, quindi un repository di sola lettura che l'hook non poteva servire falliva di nuovo la sessione su ogni nuovo runner su cui la sessione riprendeva.

Il runner rimuove il percorso di checkout dopo la fine della sessione.

<h3 id="post-session">
  post-session
</h3>

Viene eseguito una volta per sessione, dopo che il figlio Claude Code è uscito e prima che il runner smantelli l'area di lavoro. Questo hook è la tua unica possibilità di salvare il lavoro non committato: a `--capacity` superiore a uno, il runner elimina i worktree per sessione subito dopo il ritorno dell'hook, e a `--capacity 1` il [clone canonico](/docs/it/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout) riutilizzato viene hard-reset quando la sessione successiva inizia, quindi i cambiamenti tracciati non committati non sopravvivono su nessuno dei due percorsi. Gli usi tipici sono il push di un ramo snapshot di cambiamenti non committati, l'archiviazione di log o l'emissione di un evento di fine sessione ai propri sistemi.

L'hook si attiva ad ogni fine sessione dove un processo figlio è stato generato, qualunque sia la causa; i valori `CLAUDE_RUNNER_EXIT_REASON` di seguito enumerano i casi. Non può attivarsi quando il runner termina bruscamente, come una preemption VM o una perdita di potenza; se hai bisogno di garanzie contro la terminazione brusca, fai uno snapshot periodicamente dall'interno della sessione con un hook Claude Code `PostToolUse` invece. Il runner imposta:

| Variabile                          | Descrizione                                                                                                                                                                                                                |
| :--------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_RUNNER_SESSION_ID`         | ID sessione nel modulo taggato `session_...`                                                                                                                                                                               |
| `CLAUDE_RUNNER_SESSION_UUID`       | Lo stesso ID sessione nel modulo UUID canonico                                                                                                                                                                             |
| `CLAUDE_RUNNER_EXIT_REASON`        | Come la sessione è terminata; consultare i valori sotto la tabella                                                                                                                                                         |
| `CLAUDE_RUNNER_WORKSPACE_PATHS`    | Percorsi assoluti separati da due punti degli alberi di lavoro della sessione. Vuoto per sessioni senza repository.                                                                                                        |
| `CLAUDE_RUNNER_DEBUG_LOG_PATH`     | Percorso al log di debug della sessione, ancora su disco mentre l'hook viene eseguito                                                                                                                                      |
| `CLAUDE_RUNNER_API_BASE_URL`       | URL di base dell'API Anthropic per le chiamate limitate alla sessione                                                                                                                                                      |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`    | La superficie client che ha creato la sessione, come `web_claude_ai`, `desktop_app` o `ios`. Non impostato quando la sessione non ha una superficie registrata o riconosciuta. Richiede Claude Code v2.1.229 o successivo. |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN` | Il token di accesso della sessione, per le chiamate API limitate alla sessione                                                                                                                                             |

`CLAUDE_RUNNER_EXIT_REASON` assume uno di quattro valori:

* `completed`: la sessione è terminata in modo pulito. Il processo Claude Code è uscito normalmente, oppure la sessione è stata archiviata o eliminata mentre era ancora in esecuzione.
* `failed`: il processo Claude Code è andato in crash, oppure la configurazione è fallita dopo l'avvio.
* `interrupted`: il runner ha interrotto la sessione. Ha rilasciato la sessione per liberare lo slot, la sessione è scaduta all'avvio, il server ha spostato la sessione da questo runner, il runner era in drenaggio, oppure la sessione ha superato il limite [`--kill-session-after-min`](/docs/it/self-hosted-environments-reference#runner-cli-flags).
* `abandoned`: riservato per una sessione che un altro runner ha rivendicato. L'hook attualmente non si attiva in quel caso.

I [contatori del ciclo di vita della sessione](/docs/it/self-hosted-environments-reference#session-lifecycle-counter-semantics) contano un rilascio, un timeout di avvio e uno spostamento del server come `completed` piuttosto che `interrupted`, perché il runner ha restituito lo slot in modo pulito. Aspettati quella differenza se confronti le ricevute dell'hook con i contatori.

Lo stato di uscita dell'hook non influisce mai sul risultato della sessione; un fallimento viene registrato e ignorato. Il runner attende fino a `--post-session-hook-timeout-sec`, 60 secondi per impostazione predefinita, ad ogni fine sessione incluso l'arresto del runner. Questo esempio salva il lavoro non committato in un ramo di salvataggio:

```bash theme={null}
#!/usr/bin/env bash
set -u
IFS=':'
# Configurazione pin che la sessione potrebbe aver piantato in .git/config del checkout:
# -c gli override battono le impostazioni locali del repository, bloccando fsmonitor scritto dalla sessione,
# hook-path e configurazione gpg-program dall'esecuzione di codice con i privilegi dell'hook.
# Credenziale locale del repository, core.sshCommand e pushurl si applicano ancora; se l'hook
# contiene credenziali che la sessione non aveva, pin anche l'URL di push e l'helper (consultare la nota sotto lo script).
g() { git -c core.fsmonitor=false -c core.hooksPath=/dev/null \
        -c commit.gpgsign=false "$@"; }
for ws in $CLAUDE_RUNNER_WORKSPACE_PATHS; do
  cd "$ws" 2>/dev/null || continue
  [ -z "$(g status --porcelain 2>/dev/null)" ] && continue
  g add -A
  g commit -q -m "runner snapshot: $CLAUDE_RUNNER_SESSION_ID ($CLAUDE_RUNNER_EXIT_REASON)" || continue
  g push -q origin "HEAD:refs/heads/rescue/$CLAUDE_RUNNER_SESSION_ID" || true
done
```

L'hook spinge con qualsiasi credenziale git disponibile nel suo stesso ambiente sull'host del runner. Sotto la [postura no-credentials-in-the-image](/docs/it/self-hosted-environments-deploy#configure-git), incluso quando il clone integrato passa attraverso il proxy git di Anthropic, non ce ne sono, quindi conia una credenziale di push di breve durata all'interno dell'hook prima di spingere: scambia il token della sessione che l'hook riceve in `CLAUDE_CODE_SESSION_ACCESS_TOKEN` con il servizio di token proprio, verificandolo come [Verify session identity](/docs/it/self-hosted-environments-identity) descrive. Quando l'hook contiene una credenziale che la sessione non aveva, pin anche dove spinge: sostituisci `origin` con un URL fornito dall'operatore e passa `-c credential.helper=` più il tuo helper, in modo che la configurazione locale che la sessione ha scritto non possa reindirizzare il push credenziale.

<h4 id="hook-timing-when-the-runner-releases-a-session">
  Hook timing quando il runner rilascia una sessione
</h4>

Una sessione rilasciata può riprendere su un altro runner. Su un runner su v2.1.236 o successivo, ciò che la sessione stava facendo al rilascio decide se può riprendere prima che questo hook finisca:

* **Inattivo dopo un turno, o timeout all'avvio**: il runner ferma il figlio ed esegue questo hook fino al completamento. Solo allora rilascia la sessione. Un messaggio utente inviato mentre l'hook viene eseguito non può riprendere la sessione su un altro runner prima che l'hook finisca.
* **In attesa che l'utente risponda a un prompt, come un prompt di autorizzazione**: il runner rilascia la sessione per primo, quindi esegue questo hook. Un messaggio utente inviato mentre l'hook viene eseguito può riprendere la sessione su un altro runner prima che l'hook finisca.

Questo si applica ogni volta che il runner rilascia una sessione: al timeout di inattività, al momento [`--retire-at`](/docs/it/self-hosted-environments-reference#runner-cli-flags), e, su un runner su v2.1.260 o successivo, al limite [`--kill-session-after-min`](/docs/it/self-hosted-environments-reference#runner-cli-flags) di una sessione. Una sessione il cui turno è terminato e che contiene solo attività in background conta come inattiva qui. Prima della v2.1.236, il runner rilasciava la sessione per primo e quindi eseguiva questo hook in entrambi i casi.

Durante un drenaggio `SIGTERM`, il runner mantiene il lease della sessione fino al completamento dell'hook; consultare [Shutdown timing](/docs/it/self-hosted-environments-deploy#shutdown-timing).

<h3 id="command">
  command
</h3>

Viene eseguito una volta per sessione dopo il checkout, al posto dello spawn del figlio integrato. L'hook riceve lo stesso ambiente di uno [script wrapper](#wrapper-scripts) e dovrebbe fare `exec` in `"$CLAUDE_RUNNER_CLAUDE_BIN"` allo stesso modo. Utilizzare l'hook `command` per mantenere tutta la personalizzazione in una directory di hook; utilizzare `--exec-path` quando il wrapper vive altrove. Se `--exec-path` è anche impostato, il flag ha la precedenza e l'hook `command` viene ignorato.

Sempre fare `exec` del binario del runner stesso piuttosto che di un `claude` risolto da PATH; altrimenti si sconfigge il [pinning della versione](/docs/it/self-hosted-environments-deploy#pin-the-version).

<h2 id="on-demand-runners">
  Runner su richiesta
</h2>

Invece di eseguire una flotta fissa, è possibile avviare un runner per sessione. L'orchestratore è un subcomando separato e senza stato che esegue il polling di Anthropic per le richieste di spawn, una per sessione in coda senza runner disponibile, ed esegue l'hook `spawn-runner` per ciascuna. L'hook invia un carico di lavoro alla propria piattaforma: un Kubernetes Job, un'istanza EC2, un dispatch Nomad.

I runner su richiesta migliorano l'igiene delle credenziali. Su una flotta fissa, il segreto dell'ambiente vive su ogni host del runner, che è lo stesso host che esegue le sessioni utente. Con l'orchestratore, il segreto dell'ambiente rimane solo sull'host dell'orchestratore, che non esegue mai il codice utente; ogni runner generato riceve un ordine di lavoro monouso che registra esattamente un runner e quindi scade.

Per avviare l'orchestratore, passare il segreto dell'ambiente e una directory di hook contenente uno script `spawn-runner` eseguibile:

```bash theme={null}
claude self-hosted-runner orchestrator \
  --environment-secret-file /etc/claude/environment-secret \
  --hooks-dir /etc/claude/hooks
```

L'orchestratore non mantiene alcuno stato tra i poll, quindi è possibile eseguire due o più repliche rispetto allo stesso ambiente per la disponibilità. Ogni richiesta di spawn viene rivendicata lato server da esattamente una replica. Tutte le repliche devono utilizzare lo stesso valore `--expected-spawn-seconds`; consultare il [contratto dell'hook](#the-spawn-runner-hook).

<h3 id="the-spawn-runner-hook">
  L'hook spawn-runner
</h3>

L'orchestratore esegue `${hooks-dir}/spawn-runner` una volta per richiesta di spawn. L'hook deve inviare il lavoro in modo asincrono, senza attendere l'avvio del runner, e tornare entro `--hook-timeout`, 60 secondi per impostazione predefinita. L'hook riceve:

| Variabile                             | Descrizione                                                                                                                                                                                                                                                                                                                                                           |
| :------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_RUNNER_WORK_ORDER_FILE`       | Percorso a un file temporaneo contenente il JWT dell'ordine di lavoro firmato con cui il nuovo runner si registra. Eliminato dopo l'uscita dell'hook. Non registrare il contenuto del file.                                                                                                                                                                           |
| `CLAUDE_RUNNER_ORDER_ID`              | Chiave di idempotenza opaca, unica per richiesta di spawn e sicura per i nomi delle risorse Kubernetes. Utilizzarla come chiave di dedup del provisioner.                                                                                                                                                                                                             |
| `CLAUDE_RUNNER_SESSION_ID`            | La sessione per cui è questa richiesta. Vuoto per le richieste di pre-warming, che avviano un runner standby prima di qualsiasi sessione specifica quando [`--min-idle`](/docs/it/self-hosted-environments-reference#orchestrator-cli-flags) è impostato, quindi non assumere che la variabile sia impostata.                                                              |
| `CLAUDE_RUNNER_SESSION_UUID`          | Lo stesso ID sessione nel modulo UUID canonico. Vuoto per le richieste di pre-warming.                                                                                                                                                                                                                                                                                |
| `CLAUDE_RUNNER_ATTEMPT`               | Quante richieste di spawn questa sessione ha avuto. `0` per le richieste di pre-warming.                                                                                                                                                                                                                                                                              |
| `CLAUDE_RUNNER_ORDER_SERVER_TIME`     | Ora del server dalla risposta del poll header HTTP `Date`. Quando l'hook verifica l'`exp` del JWT dell'ordine di lavoro, confrontare rispetto a questo valore invece dell'orologio locale per tollerare lo skew. Vuoto quando il gateway ha omesso l'header.                                                                                                          |
| `CLAUDE_RUNNER_POOL_ID`               | L'ID dell'ambiente a cui il nuovo runner dovrebbe unirsi, nel modulo `ccpool_...`                                                                                                                                                                                                                                                                                     |
| `CLAUDE_RUNNER_ACCOUNT_ID`            | ID taggato dell'account che ha accodato la sessione, per l'instradamento per account, la quota o il chargeback. Vuoto quando non disponibile, e sempre vuoto per le sessioni del canale Claude Tag, che nessun account accoda.                                                                                                                                        |
| `CLAUDE_RUNNER_ACCOUNT_EMAIL`         | Email dell'account che ha accodato la sessione. Vuoto quando non disponibile. Trattare l'email come informazioni personali identificabili e non registrarla.                                                                                                                                                                                                          |
| `CLAUDE_RUNNER_PRIMARY_REPO_URL`      | URL della prima fonte git della sessione, per l'instradamento a un runner con quel repository pre-riscaldato. Vuoto quando la sessione non ha fonti git.                                                                                                                                                                                                              |
| `CLAUDE_RUNNER_PRIMARY_REPO_REVISION` | Revisione della prima fonte git della sessione: ramo, SHA o tag. Vuoto quando non specificato.                                                                                                                                                                                                                                                                        |
| `CLAUDE_RUNNER_REPO_SOURCES`          | Array JSON di `{url, revision}` per tutte le fonti git della sessione, per gli hook che instradano su un repository secondario. Vuoto quando non ci sono fonti.                                                                                                                                                                                                       |
| `CLAUDE_RUNNER_CORRELATION_ID`        | L'ID di correlazione fornito alla creazione della sessione, ripetuto in modo che l'hook possa mappare questo ordine di lavoro alla richiesta che ha creato la sessione. Vuoto quando la sessione non ne ha uno.                                                                                                                                                       |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`       | La superficie client che ha creato la sessione, come `web_claude_ai`, `desktop_app`, `ios` o `scheduled_trigger`, per l'analisi dell'adozione. Non impostato quando la sessione non ha una superficie registrata o riconosciuta, e per le richieste di pre-warming; controllarlo con `[ -n "${CLAUDE_RUNNER_CLIENT_PLATFORM:-}" ]`, che rimane sicuro sotto `set -u`. |

Il runner generato si registra con l'ordine di lavoro al posto del segreto dell'ambiente:

* **Avviarlo con l'ordine di lavoro**: puntare [`--environment-secret-file`](/docs/it/self-hosted-environments-reference#runner-cli-flags) a un file contenente il JWT dell'ordine di lavoro, o impostare `SELF_HOSTED_RUNNER_ENVIRONMENT_SECRET` al valore JWT.
* **Copiare il JWT prima che l'hook esca**: l'orchestratore elimina il file dell'ordine di lavoro dopo l'uscita dell'hook, quindi copiare il JWT nel carico di lavoro che si invia, come un Kubernetes Secret sul Job generato, piuttosto che passare il percorso del file.
* **Utilizzare `--capacity 1` sui runner generati**: un ordine di lavoro legato alla sessione registra esattamente un runner legato a quella sessione, quindi una capacità più alta aggiunge slot che non ricevono mai lavoro, e il runner registra un avviso all'avvio.
* **Gli ordini di lavoro di pre-warming si registrano non legati**: il runner standby non è legato a una sessione e rivendica il lavoro in coda come un runner di flotta fissa.

Il contratto ha quattro regole agnostiche del provisioner:

1. **Essere idempotenti su `CLAUDE_RUNNER_ORDER_ID`.** La rielaborazione della stessa richiesta deve generare al massimo un runner. Derivare un nome di risorsa deterministico dall'ID e lasciare che la propria piattaforma rifiuti il duplicato.
2. **Non ritentare il carico di lavoro.** Un ID ordine significa al massimo un carico di lavoro creato. Se il runner non si registra mai, Anthropic richiede con un ID ordine fresco dopo `--expected-spawn-seconds`.
3. **Utilizzare il contratto del codice di uscita.** Uscita 0 significa inviato. Uscita 1 significa fallimento ritentabile; la sessione si ritira e viene riottenuta. Uscita 2 o superiore significa non ritentabile; la sessione è bloccata dallo spawn di nuovo fino a quando un [Owner](/docs/it/cloud-environments#organization-shared-environments) non seleziona **Retry** su di essa nella scheda **Activity** dell'ambiente. Su uscita diversa da zero, la coda dello stderr dell'hook appare lì come motivo del fallimento, quindi scrivere l'errore azionabile su stderr e mai segreti. Per una richiesta di pre-warming non c'è sessione da fallire: l'orchestratore registra un'uscita diversa da zero localmente solo, e il server richiede di nuovo lo spawn dopo il lease.
4. **Impostare `--expected-spawn-seconds` ad almeno il tempo di avvio p99.** Questo è il lease lato server. Tutte le repliche dell'orchestratore devono utilizzare lo stesso valore.

Tutto ciò che l'hook scrive su stdout o stderr appare nel log dell'orchestratore con le credenziali automaticamente redatte. Se le sessioni rimangono in coda, controllare il corpo `/healthz` dell'orchestratore per i conteggi della coda, quindi aprire la scheda **Activity** dell'ambiente sulla [pagina di amministrazione **Cloud environments**](https://claude.ai/admin-settings/cloud-environments): espandere una sessione fallita lì per il suo errore di spawn e selezionare **Retry** per richiederlo di nuovo.

<h2 id="mcp-servers">
  Server MCP
</h2>

Per rendere i [server MCP](/docs/it/mcp) disponibili in ogni sessione, aggiungerli al momento della compilazione dell'immagine con lo stesso comando `claude mcp add` utilizzato su un'installazione desktop. Se il runner è un processo nudo piuttosto che un contenitore, eseguire lo stesso comando come utente del runner sull'host, quindi riavviare il runner: legge la configurazione dell'host una volta all'avvio. Il flag `--scope user` è obbligatorio; l'ambito locale predefinito scrive sotto una chiave per directory che il runner non semina nelle sessioni. Ad esempio, nel Dockerfile:

```dockerfile theme={null}
RUN claude mcp add --scope user sidecar -- /usr/local/bin/mcp-sidecar
RUN claude mcp add --scope user --transport http internal http://mcp-gateway.svc.cluster.local:8080
```

Il runner fa uno snapshot della configurazione dell'host una volta all'avvio. Lo snapshot acquisisce la chiave `mcpServers` dal `.claude.json` dell'host, che vive accanto piuttosto che dentro `~/.claude/`, e il runner semina solo quella chiave nella configurazione isolata di ogni sessione; lo stato dell'account e la cronologia del progetto vengono eliminati. Per confermare che i server hanno raggiunto le sessioni, avviare una sessione sull'ambiente e chiedere a Claude di elencare i suoi strumenti MCP; il runner registra anche un avviso di avvio per qualsiasi voce acquisita il cui `type` non riconosce e elimina la voce, quindi è possibile vedere perché quel server manca dalle sessioni. Quando `SELF_HOSTED_RUNNER_HOST_CONFIG_DIR` è impostato, il runner legge `.claude.json` da quella directory invece, quindi puntare la variabile a una directory vuota disabilita anche la semina MCP.

Claude Code carica anche server MCP da altre fonti:

* Il [file MCP gestito a livello aziendale](/docs/it/managed-mcp) al suo percorso di sistema standard: `/etc/claude-code/managed-mcp.json` sugli host del runner Linux, `/Library/Application Support/ClaudeCode/managed-mcp.json` sugli host macOS. Utilizzarlo per flotte bloccate dove solo i server elencati dall'amministratore possono caricarsi. Consultare [exclusive control with managed-mcp.json](/docs/it/managed-mcp#exclusive-control-with-managed-mcp-json) per le regole di precedenza. Quando questo file è sull'host del runner, Claude Code salta i server MCP che il piano di controllo di Anthropic fornisce a una sessione, inclusi i connettori claude.ai, e li nomina in un avviso su stderr del figlio della sessione, che il runner registra al livello di log `debug`. Prima della v2.1.229, quelle sessioni uscivano all'avvio con `You cannot dynamically configure MCP servers when an enterprise MCP config is present`.
* La chiave [`managedMcpServers`](/docs/it/settings-reference#managedmcpservers) nelle [impostazioni gestite](/docs/it/managed-settings) sull'host del runner: fornisce server HTTP e SSE senza prendere il controllo esclusivo, quindi i server dalle altre fonti si caricano ancora. Richiede Claude Code v2.1.259 o successivo.
* `<repo>/.mcp.json`: ambito del progetto. Committare il file al repository; i suoi server sono pre-approvati nelle sessioni cloud.

Quando la consegna del connettore è abilitata per l'organizzazione, il piano di controllo di Anthropic fornisce i connettori configurati su claude.ai alle sessioni create in modo interattivo attraverso la configurazione MCP fornita dal server, instradata attraverso `api.anthropic.com`. Le sessioni create a livello di programmazione, come i [dispatch CLI](/docs/it/self-hosted-environments-testing#run-the-test-loop), non ricevono la consegna del connettore; fornire loro server MCP attraverso una qualsiasi delle altre fonti che questa sezione elenca. Il token OAuth del figlio non porta un ambito per recuperare i connettori direttamente, quindi il figlio non tenta quel recupero stesso; la consegna è guidata dal server.

`settings.json` non porta definizioni di server MCP, e non c'è un campo `mcpServers` di livello superiore nello schema delle impostazioni. Nelle impostazioni gestite, fornire server con la chiave [`managedMcpServers`](/docs/it/settings-reference#managedmcpservers) invece.

Le sessioni ereditano l'ambiente del runner, quindi impostare [`ENABLE_TOOL_SEARCH`](/docs/it/mcp#scale-with-mcp-tool-search) lì per controllare la ricerca dello strumento MCP per ogni sessione che un runner genera; la pagina MCP copre i valori.

<h2 id="prompt-sessions-to-push-their-work">
  Prompt delle sessioni per spingere il loro lavoro
</h2>

Le sessioni ospitate da Anthropic eseguono un hook [`Stop`](/docs/it/hooks#stop), l'hook Claude Code che viene eseguito quando Claude finisce di rispondere, che richiede a Claude di committare e spingere il suo lavoro. Il runner non ne installa uno. Senza di esso, una sessione che termina con cambiamenti non committati lascia quel lavoro solo sul disco del runner, e il pulsante **Create PR** in claude.ai/code rimane inattivo fino a quando il ramo non esiste sul remoto.

L'implementazione di riferimento di seguito ha due parti. Unire il blocco delle impostazioni in `~/.claude/settings.json` sull'host del runner, che il runner semina in ogni sessione, e salvare lo script come `~/.claude/hooks/stop-hook-nudge.sh` sull'host del runner e renderlo eseguibile:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "timeout": 10,
            "command": "\"$CLAUDE_CONFIG_DIR/hooks/stop-hook-nudge.sh\""
          }
        ]
      }
    ]
  }
}
```

```sh theme={null}
#!/bin/sh
# Implementazione di riferimento dello stop-hook per runner self-hosted.
#
# Spinge Claude una volta per turno se la directory del progetto ha cambiamenti
# non committati O commit non spinti, in modo che il lavoro non vada perso quando
# una sessione inattiva viene rilasciata e in modo che il pulsante "Create PR" su
# claude.ai/code si illumini.
#
# Livello runner (nessun cambio del repository): rilasciare questo file in ~/.claude/hooks/
# sull'host del runner e unire il blocco delle impostazioni dello stop-hook accompagnante
# in ~/.claude/settings.json — il runner semina entrambi in ogni sessione.
# Alternativa a livello di repository: committare a <repo>/.claude/hooks/ e cambiare il
# percorso del comando settings.json a $CLAUDE_PROJECT_DIR/.claude/hooks/.
#
# stdin: payload JSON dell'hook (consultare https://code.claude.com/docs/en/hooks)
# stdout: {"decision":"block","reason":"..."} per spingere, o nulla per consentire lo stop.

# Guardia di rientrata: l'imbracatura imposta stop_hook_active=true quando reinvoca
# lo stop hook dopo un blocco. Uscire in modo da spingere solo una volta per turno. L'
# imbracatura emette JSON compatto (nessuno spazio dopo i due punti), che questo
# pattern si basa; usare jq se hai bisogno di un controllo tollerante agli spazi bianchi.
in=$(cat)
case "$in" in *'"stop_hook_active":true'*) exit 0 ;; esac

d="$CLAUDE_PROJECT_DIR"

# Non un repository git → nulla da spingere.
git -C "$d" rev-parse --git-dir >/dev/null 2>&1 || exit 0

# Nessun remoto → "spingere al remoto" è insodisfacibile; uscire.
[ -z "$(git -C "$d" remote 2>/dev/null)" ] && exit 0

# Cambiamenti non committati (staged, unstaged o untracked). Escludere .claude/
# interamente — le impostazioni seminate dall'operatore e lo stato di runtime scritto da CLI
# (blocco dello scheduler, worktree, stato della routine) vivono lì e nessuno è
# "lavoro non committato" che il modello ha bisogno di spingere.
s=$(git -C "$d" status --porcelain -- . ':(exclude).claude/' 2>/dev/null)
if [ -n "$s" ]; then
  printf '{"decision":"block","reason":"There are uncommitted changes in the repository. Please commit and push these changes to the remote branch."}'
  exit 0
fi

# Commit non spinti. Contare i commit su HEAD non raggiungibili da alcun
# ref di tracciamento remoto o FETCH_HEAD. Questo funziona uniformemente per:
#   - checkout init+fetch (runner predefinito: solo FETCH_HEAD esiste)
#   - checkout basati su clone (origin/* esiste)
#   - il runner predefinito: il figlio inizia sul ramo di risultato della sessione,
#     che il runner crea dopo il checkout
#   - HEAD staccato, quando una configurazione personalizzata salta quella creazione di ramo
# Senza alcun punto di riferimento (mai recuperato), rimanere silenzioso piuttosto che
# falso positivo su un turno di sola lettura.
base=""
git -C "$d" rev-parse --verify -q FETCH_HEAD >/dev/null && base="FETCH_HEAD"
if [ -z "$base" ] && [ -z "$(git -C "$d" for-each-ref --count=1 refs/remotes/origin 2>/dev/null)" ]; then
  exit 0
fi
# shellcheck disable=SC2086  # $base è "" o "FETCH_HEAD", word-split intenzionale
unpushed=$(git -C "$d" rev-list HEAD --not $base --remotes=origin --count 2>/dev/null) || unpushed=0
if [ "$unpushed" -gt 0 ]; then
  branch=$(git -C "$d" symbolic-ref --short -q HEAD)
  if [ -n "$branch" ]; then
    # $branch è influenzato dall'attaccante — git-check-ref-format(1) consente `"`
    # nei nomi dei ref. `\` è vietato (regola 10) ma comunque sfuggito come difesa
    # economica in profondità.
    # Sfuggire ai metacaratteri JSON prima di interpolare nel payload costruito a mano
    # in modo che un ramo come x","continue":false non possa iniettare chiavi nel
    # JSON di output dell'hook che l'imbracatura analizza. $unpushed è sicuro — il
    # guard -gt sopra rifiuta qualsiasi cosa che non sia un semplice intero.
    branch_esc=$(printf '%s' "$branch" | sed 's/\\/\\\\/g; s/"/\\"/g')
    printf '{"decision":"block","reason":"There are %s unpushed commit(s) on branch '\''%s'\''. Please push these changes to the remote repository."}' "$unpushed" "$branch_esc"
  else
    printf '{"decision":"block","reason":"There are %s unpushed commit(s) on a detached HEAD. Please create a branch and push it to the remote repository."}' "$unpushed"
  fi
  exit 0
fi

exit 0
```

L'hook spinge Claude a committare e spingere prima della fine della sessione, e rimane silenzioso quando la directory non è un repository git o non ha un remoto.

<h2 id="permissions-and-tool-approval">
  Autorizzazioni e approvazione degli strumenti
</h2>

Una sessione self-hosted non ha un terminale allegato, quindi un prompt di autorizzazione senza risposta blocca il turno fino a quando l'utente non risponde nell'interfaccia utente. Il piano di controllo di Anthropic invia l'elenco degli strumenti di ogni sessione e le regole di autorizzazione con il payload di lavoro; la configurazione predefinita pre-approva le chiamate di routine, incluso `Bash`, e le sessioni cloud [pre-approvano le modifiche ai file indipendentemente dalla modalità](/docs/it/permission-modes#switch-permission-modes). Una chiamata che nulla pre-approva richiede attraverso l'interfaccia utente della sessione.

<Note>
  Pinare solo la modalità auto su un ambiente le cui sessioni contenitore vengono eseguite con [default-deny network egress](/docs/it/self-hosted-environments-deploy#default-deny-egress) e il resto della [sezione di hardening](/docs/it/self-hosted-environments-deploy#harden-your-deployment) in atto. Le chiamate di routine, incluse le richieste di rete `Bash`, vengono eseguite senza un umano nel ciclo sia sul set di strumenti pre-approvati predefinito che in modalità auto, quindi il confine di rete è ciò che limita dove quelle chiamate possono raggiungere.
</Note>

Per mantenere i prompt al minimo indipendentemente da ciò che il piano di controllo invia, pinare la [modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) dal script wrapper o dall'hook [`command`](#command). La modalità auto consente alle sessioni di funzionare senza prompt di autorizzazione di routine: un modello di classificatore separato esamina le azioni prima che vengono eseguite e blocca quelle che rifiuta, e le regole di richiesta esplicita forzano comunque un prompt; la pagina delle modalità di autorizzazione copre ciò che il classificatore controlla. Il runner aggiunge flag calcolati dal server prima di invocare il wrapper, e per flag a valore singolo come `--permission-mode` il parser onora l'ultima occorrenza, quindi un flag che si aggiunge dopo `"$@"` sovrascrive il valore inviato dal server:

```bash theme={null}
#!/bin/bash
exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@" --permission-mode auto
```

Per pre-approvare strumenti specifici, aggiungere `--allowed-tools` con le proprie regole, ad esempio `--allowed-tools "Bash(bazel *) Bash(yarn *) mcp__internal__*"`. I flag di elenco come `--allowed-tools` e `--disallowed-tools` si accumulano tra le occorrenze piuttosto che sovrascrivere, quindi le proprie regole si applicano in cima a qualsiasi regola che il piano di controllo invia. Per restringere, aggiungere `--disallowed-tools`, che nega gli strumenti anche se un'altra regola li consente.

<h3 id="how-each-session’s-config-is-assembled">
  Come la configurazione di ogni sessione viene assemblata
</h3>

Il runner fornisce a ogni sessione la propria directory di configurazione, seminata da uno snapshot in memoria di `~/.claude/` dell'host che il runner acquisisce una volta all'avvio: `settings.json`, `CLAUDE.md`, hook, agenti, comandi e skill nell'immagine del runner si applicano a ogni sessione come baseline a livello di utente. Se si modifica la configurazione su un host in esecuzione, la modifica ha effetto solo dopo il riavvio del runner.

Impostare `SELF_HOSTED_RUNNER_HOST_CONFIG_DIR` per seminare da un percorso diverso, o puntarlo a una directory vuota per disabilitare la semina.

Il `.claude/settings.json` committato nel repository si sovrappone come impostazioni del progetto. Le sessioni leggono anche [`managed-settings.json`](/docs/it/settings#where-settings-live) dal percorso di sistema standard nell'immagine del runner. Se le sue chiavi si applicano insieme alle [impostazioni gestite dal server](/docs/it/server-managed-settings) segue [come Claude Code combina le fonti gestite](/docs/it/managed-settings#how-claude-code-combines-managed-sources): per impostazione predefinita, quando l'organizzazione fornisce qualsiasi chiave gestita dal server, le sessioni ignorano il file dell'immagine del runner a parte le [chiavi che Claude Code legge da ogni fonte di amministrazione](/docs/it/managed-settings#keys-read-from-every-admin-source), come il blocco `env`, i blocchi sandbox, i percorsi binari sandbox e `forceRemoteSettingsRefresh`. Consultare [settings precedence](/docs/it/settings#settings-precedence).

Quando il piano di controllo di Anthropic fornisce una sessione con [hook Claude Code](/docs/it/hooks), il runner li installa insieme, non sopra, la propria configurazione. Richiede Claude Code v2.1.229 o successivo.

* **Dove atterrano**: il runner scrive ogni script di hook fornito in una sottodirectory riservata `hooks/.ccr-launcher/` della directory di configurazione della sessione e registra gli script in un file di impostazioni separato che passa alla sessione con `--settings`, lasciando il `settings.json` seminato e i propri script in `hooks/<name>` intatti. Il runner ricrea la sottodirectory riservata per ogni sessione e non semina il contenuto dell'host in `~/.claude/hooks/.ccr-launcher/` nelle sessioni.
* **Chi li crea**: il piano di controllo popola gli script da costanti fisse nella propria distribuzione, mai da input per sessione o di terze parti.
* **Cosa ancora li governa**: gli hook forniti attraverso `--settings` entrano nella configurazione ordinaria dell'hook unito, non nel livello gestito, quindi le impostazioni gestite si applicano ancora. `disableAllHooks` li disabilita, e non sono tra le categorie che [`allowManagedHooksOnly`](/docs/it/settings-reference#allowmanagedhooksonly) mantiene caricate.

<h3 id="repository-committed-permission-rules">
  Regole di autorizzazione committate nel repository
</h3>

Non mettere una voce `"Edit"`, `"Write"` o `"NotebookEdit"` nuda in un `permissions.allow` committato nel repository. Una regola di strumento file nuda corrisponde allo strumento indipendentemente dal percorso, concedendo scritture ovunque sull'host piuttosto che solo l'area di lavoro, quindi la guardia di confinamento dell'ambito di scrittura del runner contrassegna la sessione; con [`--confine-repo-settings enforce`](/docs/it/self-hosted-environments-reference#runner-cli-flags) rifiuta di generare la sessione invece di registrare e continuare. Consultare la [sezione di hardening](/docs/it/self-hosted-environments-deploy#harden-your-deployment).

Un repository non ha bisogno di alcuna regola di strumento file: le sessioni cloud [pre-approvano le modifiche ai file indipendentemente dalla modalità](/docs/it/permission-modes#switch-permission-modes). Se si committano una regola, limitarla all'area di lavoro, come `"Edit(/**)"`; una singola barra iniziale è relativa alla radice del progetto, che è l'area di lavoro della sessione. Le regole di strumento file nude vanno bene nel `settings.json` a livello di host dell'operatore, poiché quel file non è committato nel repository.

Un `defaultMode` di `auto` è onorabile solo dal file di impostazioni a livello di immagine o a livello di utente, quindi un repository estratto non può concedere a se stesso la modalità auto. Per quali modalità le sessioni cloud accettano e la sintassi completa della regola, consultare [permission modes](/docs/it/permission-modes).

<h2 id="what’s-next">
  Cosa c'è dopo
</h2>

* [Reference](/docs/it/self-hosted-environments-reference): ogni flag CLI, variabile di ambiente e metrica
* [Verify session identity](/docs/it/self-hosted-environments-identity): convalidare il token della sessione da servizi al di fuori del runner
