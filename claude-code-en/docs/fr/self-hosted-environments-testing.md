> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Tester les environnements auto-hébergés de bout en bout

> Vérifiez une image de runner auto-hébergée à partir de CI : envoyez une session avec la CLI, lisez les réponses de Claude via un hook Stop, et scriptez la boucle complète.

<Note>
  Les environnements auto-hébergés sont en bêta publique sur les plans Team et Enterprise ; [Disponibilité et limitations](/docs/fr/self-hosted-environments#availability-and-limitations) couvre le chemin d'activation. Cette page est la recette de test CI ; consultez le [guide de démarrage rapide](/docs/fr/self-hosted-environments-quickstart) pour la configuration et [Déployer en production](/docs/fr/self-hosted-environments-deploy) pour les recettes de flotte.
</Note>

Dans un [environnement auto-hébergé](/docs/fr/self-hosted-environments), les [sessions cloud](/docs/fr/claude-code-on-the-web) de Claude Code s'exécutent sur une image de runner que vous construisez et maintenez. Avant de déployer une nouvelle image dans votre environnement de production, exécutez une session complète contre un environnement de test à partir d'un script : créez une session, lisez la réponse de Claude, envoyez un suivi, et lisez cette réponse aussi. C'est la forme d'un test de fumée CI qui vérifie votre image de runner, l'accès git, et tous les outils personnalisés avant de promouvoir une modification.

Cette recette suppose que vous avez déjà [configuré un environnement et un runner](/docs/fr/self-hosted-environments-quickstart#set-up-an-environment-and-runner), et que votre travail CI démarre le processus du runner sur le même hôte que le script de test, la configuration naturelle pour tester une nouvelle image de runner. Un hook Stop que vous installez sur le runner écrit la réponse finale de chaque tour dans un fichier local, et le script la lit de là, donc les seuls appels à l'API Anthropic sont les deux envois eux-mêmes. Si vos runners de test sont sur une infrastructure séparée, consultez [Runners de test distants](#remote-test-runners).

<h2 id="install-the-capture-hook-on-your-test-runner">
  Installer le hook de capture sur votre runner de test
</h2>

La relecture fonctionne via un [hook Stop](/docs/fr/hooks#stop) de Claude Code : quand Claude termine un tour, le hook reçoit le message assistant final en tant que `last_assistant_message` dans son JSON stdin et l'ajoute à `$E2E_REPLY_DIR/<session_id>.txt`. Installez-le de la même manière que le [hook Stop commit-nudge](/docs/fr/self-hosted-environments-configuration#prompt-sessions-to-push-their-work), sur le `~/.claude/` de l'hôte du runner, que le runner amorce dans chaque session.

<h3 id="save-the-hook-files">
  Enregistrer les fichiers du hook
</h3>

Enregistrez les deux fichiers ci-dessous sur l'hôte du runner :

* Le bloc de paramètres : fusionnez dans `~/.claude/settings.json` sur l'hôte du runner
* Le script : enregistrez comme `~/.claude/hooks/e2e-stop-hook-capture.sh` sur l'hôte du runner et rendez-le exécutable

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
  Avant de démarrer le runner
</h3>

Le hook a ces exigences :

* Installez-le avant de démarrer le runner. Le runner prend un instantané de `~/.claude/` une fois au démarrage, donc un hook ajouté à un runner en cours d'exécution ne prend effet qu'après un redémarrage.
* Exportez `E2E_REPLY_DIR` au processus du runner. Le hook est un no-op quand la variable n'est pas définie ou que le répertoire n'existe pas, donc définissez-la partout où vous démarrez le runner, comme l'unité systemd, la spécification du pod, ou l'étape CI. Le script de test ci-dessous le nécessite aussi.

Installez ce hook uniquement sur les runners servant votre environnement de test. Il écrit la réponse finale de chaque session sur le disque chaque fois que `E2E_REPLY_DIR` existe, ce qui est inoffensif sur un runner CI jetable mais pas quelque chose à porter dans une image de runner d'environnement de production où la variable pourrait être définie accidentellement.

<h2 id="run-the-test-loop">
  Exécuter la boucle de test
</h2>

Les drapeaux de dispatch `--environment` et `--ref` nécessitent Claude Code v2.1.224 ou ultérieur sur la machine qui exécute le script, le même plancher que le runner lui-même. Avec le hook en place et un runner démarré sur cet hôte, le script de test :

1. Crée une session sur l'environnement de test avec `claude -p "<prompt>" --environment <environment-id> --output-format json`, exécuté à partir d'une extraction git pour que la CLI puisse détecter automatiquement le référentiel à partir de la télécommande `origin`. Le `--ref <branch>` optionnel base l'extraction de la session sur une ref nommée au lieu du HEAD local. La commande crée la session, imprime une ligne de JSON contenant `session_id`, et se termine sans attendre la réponse de Claude.
2. Attend que la réponse apparaisse dans `$E2E_REPLY_DIR/<session_id>.txt`, écrite par le hook Stop sur le runner une fois le tour terminé.
3. Envoie un suivi avec `claude -p "<message>" --cloud <session_id> --output-format json` (voir [Envoyer un message de suivi à une session en cours d'exécution](/docs/fr/claude-code-on-the-web#send-follow-ups-from-the-cli)), qui publie un événement utilisateur à la session existante et se termine.
4. Attend la réponse du suivi de la même manière qu'à l'étape 2.

<h3 id="environment-dispatch-behavior">
  Comportement du dispatch `--environment`
</h3>

Claude Code crée la session, imprime l'ID de session et un lien vers celle-ci, et se termine.

Le drapeau prend la priorité sur le paramètre [`remote.defaultEnvironmentId`](/docs/fr/settings-reference#remote-defaultenvironmentid). Il ne supporte pas `--output-format stream-json`, et ne peut pas être combiné avec des drapeaux qui reprennent, s'attachent à, ou préconfigent une session, comme `--resume`, `--continue`, `--teleport`, `--session-id`, ou `--init-only`. `--cloud` est rejeté avec un ID de session ou une URL, et dans les exécutions non-interactives quand il porte une description. Un `--cloud` nu est traité comme absent. À partir d'un terminal, vous pouvez passer la tâche comme description `--cloud` au lieu d'une invite positionnelle.

<h2 id="example-script">
  Exemple de script
</h2>

Le script ci-dessous exécute la boucle complète contre `$CLAUDE_TEST_ENVIRONMENT_ID`, l'ID `ccpool_...` de votre environnement de test, affiché dans la boîte de dialogue de détail de l'environnement sur la page d'administration ou retourné par l'[appel create-environment](#create-a-dedicated-test-environment), et affirme sur une phrase sentinelle dans chaque réponse. Exécutez-le à partir d'une extraction git du référentiel dans lequel vous voulez que la session fonctionne, après avoir démarré un runner sur cet hôte avec le hook de capture installé et `E2E_REPLY_DIR` exporté.

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

Remplacez les invites `TURN1`/`TURN2` et les sentinelles `EXPECT1`/`EXPECT2` par tout ce qui exerce votre configuration, comme demander à Claude d'exécuter l'un de vos outils MCP personnalisés et affirmer sur sa sortie.

<h2 id="remote-test-runners">
  Runners de test distants
</h2>

Si vos runners de test sont sur une infrastructure séparée, comme une flotte Kubernetes persistante avec laquelle votre travail CI ne peut pas partager un système de fichiers, remplacez l'écriture de fichier dans le hook Stop par un POST à un point de terminaison que votre driver écoute :

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

Du côté du driver, exécutez n'importe quoi qui accepte le POST et maintient la réponse jusqu'à ce que le test la demande, comme un petit écouteur HTTP à l'intérieur du travail CI ou un récepteur webhook que vous exécutez déjà. Le hook s'exécute sur votre infrastructure, donc le point de terminaison n'a besoin d'être accessible que depuis vos runners.

<h2 id="authenticate-from-ci">
  S'authentifier à partir de CI
</h2>

À la fois `claude -p ... --environment` et `claude -p ... --cloud` s'authentifient avec un jeton OAuth claude.ai ; les clés API, comme `sk-ant-xxxxx`, ne sont pas acceptées pour l'un ou l'autre appel. Deux approches rendent un jeton disponible dans CI.

<h3 id="long-lived-ci-host">
  Hôte CI de longue durée
</h3>

Exécutez `claude auth login` une fois de manière interactive sur la machine qui exécute le script, en utilisant un compte utilisateur dédié pour l'automatisation. Claude Code stocke le jeton dans le trousseau du système d'exploitation sur macOS, ou dans `~/.claude/.credentials.json` sur Linux et Windows. Sur un hôte macOS dont le Keychain ne peut pas être écrit, comme c'est typique dans une session SSH où le Keychain de connexion reste verrouillé, Claude Code stocke le jeton dans `~/.claude/.credentials.json` là aussi. Voir [Gestion des identifiants](/docs/fr/authentication#credential-management).

La CLI actualise automatiquement le jeton d'accès de courte durée à chaque invocation, mais la subvention de jeton d'actualisation sous-jacente est plafonnée à 30 jours à partir de la connexion initiale, donc réexécutez `claude auth login` de manière interactive sur cet hôte tous les 30 jours.

<h3 id="ephemeral-ci-runners">
  Runners CI éphémères
</h3>

Il n'y a pas de jeton CI de longue durée pour cela aujourd'hui. La portée qui accorde le contrôle de session distante, `user:sessions:claude_code`, est plafonnée côté serveur à 30 jours, donc `claude setup-token`, qui frappe un jeton d'inférence uniquement d'un an, ne le couvre pas. Le [secret d'environnement](/docs/fr/self-hosted-environments-quickstart#set-up-an-environment-and-runner) n'est pas accepté non plus, car il n'autorise qu'un runner à s'enregistrer auprès de l'environnement, pas à créer des sessions.

Pour provisionner une connexion stockée sur un runner éphémère, définissez [`CLAUDE_CODE_OAUTH_REFRESH_TOKEN` et `CLAUDE_CODE_OAUTH_SCOPES`](/docs/fr/env-vars#variables) pour que `claude auth login` échange le jeton sans navigateur ; le même plafond de 30 jours s'applique à la subvention d'actualisation. Contactez votre équipe de compte Anthropic si vous avez besoin d'un chemin d'identité machine qui n'est pas lié à un compte humain.

<h2 id="create-a-dedicated-test-environment">
  Créer un environnement de test dédié
</h2>

Créez et supprimez les environnements par programmation pour que chaque exécution CI en obtienne un propre ; le runner que votre travail CI démarre s'enregistre dans l'environnement frais. Les appels de création et de suppression ci-dessous sont les mêmes points de terminaison que la page d'administration **Cloud environments** sur claude.ai utilise, et ils nécessitent l'en-tête `anthropic-beta: ccr-byoc-2025-07-29`.

<h3 id="mint-the-admin-token">
  Frapper le jeton d'administrateur
</h3>

`$ADMIN_TOKEN` est un jeton d'accès OAuth claude.ai pour un compte qui détient un rôle Propriétaire, frappé de la même manière que [S'authentifier à partir de CI](#authenticate-from-ci) :

* **Le frapper** : exécutez `claude auth login` avec un compte qui détient un rôle Propriétaire, puis lisez le jeton d'accès actuel à partir de partout où [Hôte CI de longue durée](#long-lived-ci-host) dit que Claude Code l'a stocké.
* **Le lire frais à chaque exécution** : la CLI fait tourner le jeton d'accès, et le même plafond de subvention d'actualisation de 30 jours s'applique, donc ne stockez pas une copie.
* **Le passer via stdin** : comme l'exemple le fait, pour que le jeton ne se retrouve jamais dans la liste d'arguments de curl ou votre journal de construction.

<h3 id="create-the-environment">
  Créer l'environnement
</h3>

Capturez la réponse sans l'afficher : `pool_secret` est une identifiante de longue durée qui peut enregistrer des runners dans l'environnement, donc stockez-la comme un secret CI masqué et imprimez uniquement l'ID d'environnement. La forme `-H @-` qui garde le jeton hors de la liste de processus nécessite curl 7.55 ou ultérieur ; les anciennes versions de curl traitent `@-` comme un en-tête littéral et envoient la demande sans autorisation.

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

Jusqu'à ce qu'un [Propriétaire active **Allow self-hosted environments**](/docs/fr/self-hosted-environments#availability-and-limitations) pour l'organisation, l'appel échoue avec une `403` `permission_error` lisant `self-hosted runners are disabled by your organization's policy`.

Démarrez un runner sur cet hôte avec `SELF_HOSTED_RUNNER_ENVIRONMENT_SECRET=$ENVIRONMENT_SECRET`, plus le hook de capture et `E2E_REPLY_DIR` par [Installer le hook de capture](#install-the-capture-hook-on-your-test-runner), puis exécutez le script de test.

<h3 id="delete-the-environment">
  Supprimer l'environnement
</h3>

Supprimez l'environnement quand l'exécution se termine, pour que chaque exécution CI commence propre :

```bash theme={null}
curl -fsS -X DELETE -H @- \
  -H "anthropic-beta: ccr-byoc-2025-07-29" -H "anthropic-version: 2023-06-01" \
  "https://api.anthropic.com/v1/code/runners/self-hosted/pools/$ENVIRONMENT_ID" \
  <<<"Authorization: Bearer $ADMIN_TOKEN"
```
