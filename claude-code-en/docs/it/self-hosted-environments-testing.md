> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Testare gli ambienti self-hosted end to end

> Verificare un'immagine di runner self-hosted da CI: inviare una sessione con la CLI, leggere le risposte di Claude attraverso un hook Stop e scrivere lo script del ciclo completo.

<Note>
  Gli ambienti self-hosted sono in beta pubblica sui piani Team ed Enterprise; [Disponibilità e limitazioni](/docs/it/self-hosted-environments#availability-and-limitations) copre il percorso di abilitazione. Questa pagina è la ricetta di test CI; vedere [quickstart](/docs/it/self-hosted-environments-quickstart) per la configurazione e [Distribuire in produzione](/docs/it/self-hosted-environments-deploy) per le ricette della flotta.
</Note>

In un [ambiente self-hosted](/docs/it/self-hosted-environments), le [sessioni cloud](/docs/it/claude-code-on-the-web) di Claude Code vengono eseguite su un'immagine di runner che costruite e mantenete. Prima di distribuire una nuova immagine al vostro ambiente di produzione, eseguite una sessione completa contro un ambiente di test da uno script: create una sessione, leggete la risposta di Claude, inviate un follow-up e leggete anche quella risposta. Questa è la forma di un test di smoke CI che verifica l'immagine del vostro runner, l'accesso a git e qualsiasi strumento personalizzato prima di promuovere una modifica.

Questa ricetta presuppone che abbiate già [configurato un ambiente e un runner](/docs/it/self-hosted-environments-quickstart#set-up-an-environment-and-runner), e che il vostro job CI avvii il processo del runner sullo stesso host dello script di test, la configurazione naturale per testare una nuova immagine di runner. Un hook Stop che installate sul runner scrive la risposta finale di ogni turno in un file locale, e lo script la legge da lì, quindi le uniche chiamate all'API Anthropic sono i due dispatch stessi. Se i vostri runner di test si trovano su infrastrutture separate, vedere [Runner di test remoti](#remote-test-runners).

<h2 id="install-the-capture-hook-on-your-test-runner">
  Installare l'hook di cattura sul vostro runner di test
</h2>

La lettura funziona attraverso un [hook Stop](/docs/it/hooks#stop) di Claude Code: quando Claude termina un turno, l'hook riceve il messaggio dell'assistente finale come `last_assistant_message` nel JSON stdin e lo aggiunge a `$E2E_REPLY_DIR/<session_id>.txt`. Installatelo nello stesso modo dell'[hook Stop commit-nudge](/docs/it/self-hosted-environments-configuration#prompt-sessions-to-push-their-work), su `~/.claude/` dell'host del runner, che il runner semina in ogni sessione.

<h3 id="save-the-hook-files">
  Salvare i file dell'hook
</h3>

Salvate i due file seguenti sull'host del runner:

* Il blocco delle impostazioni: unite in `~/.claude/settings.json` sull'host del runner
* Lo script: salvate come `~/.claude/hooks/e2e-stop-hook-capture.sh` sull'host del runner e rendetelo eseguibile

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "timeout": 10,
            "command": "\"$CLAUDE_CONFIG_DIR/hooks/e2e-stop-hook-capture.sh\""
          }
        ]
      }
    ]
  }
}
```

```sh theme={null}
#!/bin/sh
# Stop hook for testing a self-hosted environment end to end: writes each
# turn's final assistant reply to $E2E_REPLY_DIR/<session_id>.txt so a
# co-located test driver can read it without calling the Anthropic API.
# Install on the TEST runner only. Requires jq.

# No-op unless the driver is listening. Never fail the turn.
[ -n "${E2E_REPLY_DIR:-}" ] && [ -d "$E2E_REPLY_DIR" ] || exit 0

# CLAUDE_CODE_REMOTE_SESSION_ID is exported in cse_... form; the session
# id the dispatch CLI prints is in session_... form. Same id, different
# prefix.
sid=$(printf '%s' "${CLAUDE_CODE_REMOTE_SESSION_ID:-}" | sed 's/^cse_/session_/')
[ -n "$sid" ] || exit 0

# last_assistant_message is absent when the final assistant turn had no
# text, such as a tool-use-only turn. The `// empty` filter makes that a
# zero-byte write rather than the literal string "null".
jq -r '.last_assistant_message // empty' >> "$E2E_REPLY_DIR/$sid.txt" 2>/dev/null
exit 0
```

<h3 id="before-you-start-the-runner">
  Prima di avviare il runner
</h3>

L'hook ha questi requisiti:

* Installatelo prima di avviare il runner. Il runner crea uno snapshot di `~/.claude/` una sola volta all'avvio, quindi un hook aggiunto a un runner in esecuzione ha effetto solo dopo un riavvio.
* Esportate `E2E_REPLY_DIR` al processo del runner. L'hook è un no-op quando la variabile non è impostata o la directory non esiste, quindi impostatela ovunque avviate il runner, come l'unità systemd, la specifica del pod o il passo CI. Lo script di test di seguito lo richiede anche.

Installate questo hook solo sui runner che servono il vostro ambiente di test. Scrive la risposta finale di ogni sessione su disco ogni volta che `E2E_REPLY_DIR` esiste, il che è innocuo su un runner CI monouso ma non qualcosa da portare in un'immagine di runner dell'ambiente di produzione dove la variabile potrebbe essere impostata accidentalmente.

<h2 id="run-the-test-loop">
  Eseguire il ciclo di test
</h2>

I flag di dispatch `--environment` e `--ref` richiedono Claude Code v2.1.224 o successivo sulla macchina che esegue lo script, lo stesso limite minimo del runner stesso. Con l'hook in posizione e un runner avviato su questo host, lo script di test:

1. Crea una sessione sull'ambiente di test con `claude -p "<prompt>" --environment <environment-id> --output-format json`, eseguito da un checkout git in modo che la CLI possa rilevare automaticamente il repository dal remote `origin`. L'opzionale `--ref <branch>` basa il checkout della sessione su un ref denominato invece di HEAD locale. Il comando crea la sessione, stampa una riga di JSON contenente `session_id` e esce senza attendere la risposta di Claude.
2. Attende che la risposta appaia in `$E2E_REPLY_DIR/<session_id>.txt`, scritta dall'hook Stop sul runner una volta completato il turno.
3. Invia un follow-up con `claude -p "<message>" --cloud <session_id> --output-format json` (vedere [Inviare un messaggio di follow-up a una sessione in esecuzione](/docs/it/claude-code-on-the-web#send-follow-ups-from-the-cli)), che pubblica un evento utente nella sessione esistente e esce.
4. Attende la risposta del follow-up nello stesso modo del passo 2.

<h3 id="environment-dispatch-behavior">
  Comportamento del dispatch `--environment`
</h3>

Claude Code crea la sessione, stampa l'ID della sessione e un link ad essa, e esce.

Il flag ha la precedenza sull'impostazione [`remote.defaultEnvironmentId`](/docs/it/settings-reference#remote-defaultenvironmentid). Non supporta `--output-format stream-json` e non può essere combinato con flag che riprendono, si collegano o preconfigurano una sessione, come `--resume`, `--continue`, `--teleport`, `--session-id` o `--init-only`. `--cloud` viene rifiutato con un ID di sessione o URL, e nelle esecuzioni non interattive quando porta una descrizione. Un `--cloud` nudo viene trattato come assente. Da un terminale, potete passare l'attività come descrizione `--cloud` invece di un prompt posizionale.

<h2 id="example-script">
  Script di esempio
</h2>

Lo script seguente esegue il ciclo completo contro `$CLAUDE_TEST_ENVIRONMENT_ID`, l'ID `ccpool_...` del vostro ambiente di test, mostrato nella finestra di dialogo dei dettagli dell'ambiente nella pagina di amministrazione o restituito dalla [chiamata create-environment](#create-a-dedicated-test-environment), e asserisce su una frase sentinella in ogni risposta. Eseguitelo da un checkout git del repository su cui desiderate che la sessione funzioni, dopo aver avviato un runner su questo host con l'hook di cattura installato e `E2E_REPLY_DIR` esportato.

```bash theme={null}
#!/usr/bin/env bash
# End-to-end test against a self-hosted environment, using Stop-hook read-back.
# Prereqs: `claude auth login` has been run on this machine (see "Authenticate
# from CI" below); jq is installed; CLAUDE_TEST_ENVIRONMENT_ID names an
# environment whose runner is the one on this host, with the capture hook
# installed and E2E_REPLY_DIR in its environment.

set -euo pipefail

: "${CLAUDE_TEST_ENVIRONMENT_ID:=${CLAUDE_TEST_POOL_ID:-}}"  # CLAUDE_TEST_POOL_ID is the legacy spelling
: "${CLAUDE_TEST_ENVIRONMENT_ID:?set CLAUDE_TEST_ENVIRONMENT_ID to a ccpool_... id served by a runner on this host}"
: "${E2E_REPLY_DIR:?set E2E_REPLY_DIR to the directory the Stop hook on your test runner writes to, and export it to the runner process}"
: "${TEST_REPO_REF:=main}"

[ -d "$E2E_REPLY_DIR" ] || {
  echo "FAIL: E2E_REPLY_DIR ($E2E_REPLY_DIR) does not exist. The Stop hook on the runner needs it." >&2
  exit 1
}

# Waits until $E2E_REPLY_DIR/<session_id>.txt contains $2, or fails after
# 90 seconds. Tune the timeout to your environment's cold-start time. The
# file is written by the Stop hook on the runner.
await_reply() {
  local expect="$2" f="$E2E_REPLY_DIR/$1.txt"
  local deadline=$(($(date +%s) + 90))
  while :; do
    if [ -f "$f" ] && grep -qF -- "$expect" "$f"; then
      return
    fi
    [ "$(date +%s)" -lt "$deadline" ] || {
      echo "FAIL: '$expect' not in $f within 90s. The Stop hook on the runner did not write it." >&2
      echo "-- $E2E_REPLY_DIR contents --" >&2; ls -la "$E2E_REPLY_DIR" >&2
      [ -f "$f" ] && { echo "-- $f --" >&2; cat "$f" >&2; }
      exit 1
    }
    sleep 1
  done
}

# 1. Create the session on the test environment. Run from a git checkout
# so the CLI can auto-detect the repo. --ref pins the checkout to a named
# ref regardless of local HEAD.
TURN1="e2e-probe-$(date +%s)-$$: say exactly 'ok: custom tools are reachable' and nothing else"
EXPECT1="ok: custom tools are reachable"
create_json=$(claude -p "$TURN1" --environment "$CLAUDE_TEST_ENVIRONMENT_ID" \
  --ref "$TEST_REPO_REF" --output-format json)
echo "create: $create_json"
SESSION_ID=$(jq -er '.session_id' <<<"$create_json")

# 2. Wait for the turn-1 reply.
await_reply "$SESSION_ID" "$EXPECT1"
echo "turn-1 reply ok"

# 3. Post a follow-up via the CLI.
TURN2="e2e-probe-followup-$(date +%s): say exactly 'ok: follow-up delivered' and nothing else"
EXPECT2="ok: follow-up delivered"
followup_json=$(claude -p "$TURN2" --cloud "$SESSION_ID" --output-format json)
echo "followup: $followup_json"
jq -e '.ok == true' <<<"$followup_json" >/dev/null

# 4. Wait for the turn-2 reply.
await_reply "$SESSION_ID" "$EXPECT2"
echo "turn-2 reply ok"

echo "PASS: test-environment round-trip (session $SESSION_ID)"
```

Sostituite i prompt `TURN1`/`TURN2` e i sentinella `EXPECT1`/`EXPECT2` con qualsiasi cosa eserciti la vostra configurazione, come chiedere a Claude di eseguire uno dei vostri strumenti MCP personalizzati e asserire sul suo output.

<h2 id="remote-test-runners">
  Runner di test remoti
</h2>

Se i vostri runner di test si trovano su infrastrutture separate, come una flotta Kubernetes persistente con cui il vostro job CI non può condividere un filesystem, scambiate la scrittura del file nell'hook Stop con un POST a un endpoint su cui il vostro driver ascolta:

```sh theme={null}
#!/bin/sh
# Variant of the capture hook for runners on separate infrastructure.
# Set E2E_REPLY_URL on the runner to an endpoint the driver controls.
[ -n "${E2E_REPLY_URL:-}" ] || exit 0
sid=$(printf '%s' "${CLAUDE_CODE_REMOTE_SESSION_ID:-}" | sed 's/^cse_/session_/')
[ -n "$sid" ] || exit 0
jq -r '.last_assistant_message // empty' | \
  curl -fsS -X POST --data-binary @- "$E2E_REPLY_URL/$sid" >/dev/null 2>&1
exit 0
```

Dal lato del driver, eseguite qualsiasi cosa che accetti il POST e mantenga la risposta fino a quando il test non la richiede, come un piccolo listener HTTP all'interno del job CI o un ricevitore webhook che già eseguite. L'hook viene eseguito sulla vostra infrastruttura, quindi l'endpoint deve solo essere raggiungibile dai vostri runner.

<h2 id="authenticate-from-ci">
  Autenticarsi da CI
</h2>

Sia `claude -p ... --environment` che `claude -p ... --cloud` si autenticano con un token OAuth di claude.ai; le chiavi API, come `sk-ant-xxxxx`, non sono accettate per nessuna delle due chiamate. Due approcci rendono disponibile un token in CI.

<h3 id="long-lived-ci-host">
  Host CI di lunga durata
</h3>

Eseguite `claude auth login` una sola volta in modo interattivo sulla macchina che esegue lo script, utilizzando un account utente dedicato per l'automazione. Claude Code memorizza il token nel keychain del sistema operativo su macOS, o in `~/.claude/.credentials.json` su Linux e Windows. Su un host macOS il cui Keychain non può essere scritto, come è tipico in una sessione SSH dove il Keychain di login rimane bloccato, Claude Code memorizza il token in `~/.claude/.credentials.json` anche lì. Vedere [Gestione delle credenziali](/docs/it/authentication#credential-management).

La CLI aggiorna automaticamente il token di accesso di breve durata ad ogni invocazione, ma la concessione del token di aggiornamento sottostante è limitata a 30 giorni dall'accesso iniziale, quindi eseguite di nuovo `claude auth login` in modo interattivo su quell'host ogni 30 giorni.

<h3 id="ephemeral-ci-runners">
  Runner CI effimeri
</h3>

Non esiste un token CI di lunga durata per questo oggi. L'ambito che concede il controllo della sessione remota, `user:sessions:claude_code`, è limitato lato server a 30 giorni, quindi `claude setup-token`, che conia un token di sola inferenza di un anno, non lo copre. Il [segreto dell'ambiente](/docs/it/self-hosted-environments-quickstart#set-up-an-environment-and-runner) non è accettato neanche, poiché autorizza solo un runner a registrarsi con l'ambiente, non a creare sessioni.

Per fornire un accesso memorizzato su un runner effimero, impostate [`CLAUDE_CODE_OAUTH_REFRESH_TOKEN` e `CLAUDE_CODE_OAUTH_SCOPES`](/docs/it/env-vars#variables) in modo che `claude auth login` scambi il token senza un browser; lo stesso limite di 30 giorni si applica alla concessione di aggiornamento. Contattate il vostro team di account Anthropic se avete bisogno di un percorso di identità della macchina che non sia legato a un account umano.

<h2 id="create-a-dedicated-test-environment">
  Creare un ambiente di test dedicato
</h2>

Create e eliminate gli ambienti a livello di programmazione in modo che ogni esecuzione CI ottenga uno pulito; il runner che il vostro job CI avvia si registra nell'ambiente nuovo. Le chiamate di creazione e eliminazione di seguito sono gli stessi endpoint che la pagina di amministrazione **Cloud environments** su claude.ai utilizza, e richiedono l'intestazione `anthropic-beta: ccr-byoc-2025-07-29`.

<h3 id="mint-the-admin-token">
  Coniare il token di amministrazione
</h3>

`$ADMIN_TOKEN` è un token di accesso OAuth di claude.ai per un account che detiene un ruolo Owner, coniato nello stesso modo di [Autenticarsi da CI](#authenticate-from-ci):

* **Coniarlo**: eseguite `claude auth login` con un account che detiene un ruolo Owner, quindi leggete il token di accesso corrente da dove [Host CI di lunga durata](#long-lived-ci-host) dice che Claude Code lo ha memorizzato.
* **Leggerlo fresco ad ogni esecuzione**: la CLI ruota il token di accesso, e lo stesso limite di 30 giorni per la concessione di aggiornamento si applica, quindi non memorizzate una copia.
* **Passarlo via stdin**: come fa l'esempio, in modo che il token non finisca mai nell'elenco degli argomenti di curl o nel vostro log di build.

<h3 id="create-the-environment">
  Creare l'ambiente
</h3>

Catturate la risposta senza ecoarla: `pool_secret` è una credenziale di lunga durata che può registrare runner nell'ambiente, quindi memorizzatela come segreto CI mascherato e stampate solo l'ID dell'ambiente. La forma `-H @-` che mantiene il token fuori dall'elenco dei processi richiede curl 7.55 o successivo; curl più vecchio tratta `@-` come un'intestazione letterale e invia la richiesta senza autorizzazione.

```bash theme={null}
create=$(curl -fsS -X POST -H @- \
  -H "anthropic-beta: ccr-byoc-2025-07-29" -H "anthropic-version: 2023-06-01" \
  -H "content-type: application/json" \
  -d '{"name":"ci-test-environment"}' \
  https://api.anthropic.com/v1/code/runners/self-hosted/pools \
  <<<"Authorization: Bearer $ADMIN_TOKEN")
ENVIRONMENT_ID=$(jq -er .pool.pool_id <<<"$create")
ENVIRONMENT_SECRET=$(jq -er .pool_secret <<<"$create")
```

Fino a quando un [Owner non attiva **Allow self-hosted environments**](/docs/it/self-hosted-environments#availability-and-limitations) per l'organizzazione, la chiamata fallisce con un `403` `permission_error` che legge `self-hosted runners are disabled by your organization's policy`.

Avviate un runner su questo host con `SELF_HOSTED_RUNNER_ENVIRONMENT_SECRET=$ENVIRONMENT_SECRET`, più l'hook di cattura e `E2E_REPLY_DIR` per [Installare l'hook di cattura](#install-the-capture-hook-on-your-test-runner), quindi eseguite lo script di test.

<h3 id="delete-the-environment">
  Eliminare l'ambiente
</h3>

Eliminate l'ambiente quando l'esecuzione finisce, in modo che ogni esecuzione CI inizi pulita:

```bash theme={null}
curl -fsS -X DELETE -H @- \
  -H "anthropic-beta: ccr-byoc-2025-07-29" -H "anthropic-version: 2023-06-01" \
  "https://api.anthropic.com/v1/code/runners/self-hosted/pools/$ENVIRONMENT_ID" \
  <<<"Authorization: Bearer $ADMIN_TOKEN"
```
