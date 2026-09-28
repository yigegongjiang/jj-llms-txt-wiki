> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Personnaliser les sessions dans les environnements auto-hébergés

> Personnalisez les sessions d'environnement auto-hébergé avec des scripts wrapper pour les identifiants par session, les hooks de cycle de vie et le spawning de runners à la demande.

<Note>
  Les environnements auto-hébergés sont en bêta publique sur les plans Team et Enterprise ; un [Owner](/docs/fr/cloud-environments#organization-shared-environments) les active en activant **Allow self-hosted environments** sur la [page d'administration **Cloud environments**](https://claude.ai/admin-settings/cloud-environments). Cette page suppose un runner fonctionnel ; consultez le [guide de démarrage rapide](/docs/fr/self-hosted-environments-quickstart) pour la configuration et [Déployer en production](/docs/fr/self-hosted-environments-deploy) pour les recettes de flotte.
</Note>

Un [environnement auto-hébergé](/docs/fr/self-hosted-environments) exécute les [sessions cloud](/docs/fr/claude-code-on-the-web) de Claude Code sur votre propre infrastructure, exécutées par un processus runner que vous déployez. Sans configuration, ce runner clone le référentiel de la session, lance Claude Code et nettoie. Cette page s'adresse à l'ingénieur plateforme qui exploite les runners : elle couvre les points d'extension pour quand ces valeurs par défaut ne conviennent pas, de la fourniture d'identifiants par session au remplacement complet du checkout. Les wrappers et les hooks s'exécutent en tant que fichiers exécutables sur l'hôte runner, qui est Linux ou macOS, et les exemples de cette page supposent un shell POSIX.

Quelques variables d'environnement de hook sur cette page utilisent toujours `pool`, comme `CLAUDE_RUNNER_POOL_ID` ; les noms de flag CLI et de variable d'environnement utilisent `environment`, comme `--environment-secret-file`.

<h2 id="wrapper-scripts">
  Scripts wrapper
</h2>

Utilisez un script wrapper quand chaque session a besoin d'une configuration que le runner ne peut pas faire seul : provisionner des identifiants de courte durée limités au créateur de la session, exporter des secrets spécifiques à l'environnement, préparer des chaînes d'outils de langage ou appliquer des limites de ressources autour du processus enfant. Le runner démarre votre wrapper à la place du binaire Claude Code, une fois par session. Terminez le wrapper en `exec`-ant dans `$CLAUDE_RUNNER_CLAUDE_BIN`, le binaire du runner lui-même, afin que les signaux et les codes de sortie se propagent correctement.

Pointez `--exec-path`, ou `SELF_HOSTED_RUNNER_EXEC_PATH`, vers le wrapper quand vous démarrez le runner :

```bash theme={null}
claude self-hosted-runner --environment-secret-file /etc/claude/environment-secret --exec-path /etc/claude/session-wrapper.sh
```

Le runner définit les éléments suivants dans l'environnement du wrapper :

| Variable                            | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :---------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN`  | Le JWT de session, préfixé `sk-ant-cc-`. Sa revendication `act` identifie le créateur de la session, avec l'email du créateur et le sujet du fournisseur d'identité en amont quand la surface de création les a enregistrés. La valeur est le token au moment du spawn ; les actualisations arrivent sur stdin de l'enfant, donc un wrapper ne voit que la valeur initiale. Voir [Vérifier l'identité de la session](/docs/fr/self-hosted-environments-identity).                                                                                                                                                                                                                                                                                                            |
| `CCR_SESSION_ACCOUNT_EMAIL`         | L'email du créateur de la session, pré-extrait par le runner de la revendication `act.email` du token sans vérification de signature. Approprié pour l'étiquetage, comme les trailers de commit. Quand l'email contrôle l'émission d'identifiants, vérifiez le token et lisez la revendication à partir de celui-ci à la place ; voir [Provisionner les identifiants limités au créateur de la session](#provision-credentials-scoped-to-the-session-creator). Non défini quand le token ne porte pas d'email de créateur. Traiter comme des informations d'identification personnelle.                                                                                                                                                                                 |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`     | La surface client qui a créé la session, comme `web_claude_ai`, `desktop_app`, `ios`, `claude_code_cli` ou `scheduled_trigger`. Anthropic enregistre la valeur une fois à la création de la session, donc le wrapper et chaque hook de cycle de vie voient la même valeur. Utilisez-la pour l'analyse d'adoption et l'étiquetage uniquement, pas comme signal d'autorisation. Non défini quand la session n'a pas de surface enregistrée ou reconnue, donc référencez-la comme `${CLAUDE_RUNNER_CLIENT_PLATFORM:-}` sous `set -u`. Nécessite Claude Code v2.1.229 ou ultérieur.                                                                                                                                                                                         |
| `CLAUDE_RUNNER_CLAUDE_BIN`          | Chemin absolu vers le binaire Claude Code du runner lui-même. Terminez votre wrapper avec `exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"` pour passer au binaire épinglé sans coder en dur un chemin d'installation.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `CLAUDE_CODE_REMOTE_SESSION_ID`     | ID de session sous la forme balisée `cse_...`. C'est la même session que les [hooks de cycle de vie](#lifecycle-hooks) voient comme `CLAUDE_RUNNER_SESSION_ID` sous la forme `session_...` ; les variables UUID correspondent entre les deux, et remplacer le préfixe `cse_` par `session_` donne l'ID affiché dans l'URL de la session.                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `CLAUDE_CODE_REMOTE_SESSION_UUID`   | Le même ID de session sous la forme UUID canonique, pour les systèmes qui utilisent des UUIDs comme clé.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `CLAUDE_SESSION_INGRESS_TOKEN_FILE` | Chemin absolu vers un fichier par session contenant le JWT de session actuel, maintenu à jour lors des actualisations de token. Les sous-processus shell le lisent pour leur en-tête `Authorization` lors du téléchargement des pièces jointes que l'utilisateur a ajoutées à la session. `exec` préserve la variable automatiquement ; un wrapper qui reconstruit l'environnement de l'enfant doit transporter la variable, ou les téléchargements de pièces jointes s'arrêtent silencieusement.                                                                                                                                                                                                                                                                       |
| `CLAUDE_CONFIG_DIR`                 | Répertoire de configuration Claude par session, écrit au démarrage de la session à partir de l'instantané de la configuration de l'hôte runner que le runner capture au démarrage ; voir [Permissions et approbation d'outils](#permissions-and-tool-approval). Les écritures ici sont isolées à cette session. Le répertoire reste sous `<base-dir>/_sessions/` après la fin de la session sauf si vous démarrez le runner avec [`--remove-session-state`](/docs/fr/self-hosted-environments-reference#runner-cli-flags) ; voir [Réutiliser un checkout pré-chauffé](/docs/fr/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout).                                                                                                                                      |
| `ANTHROPIC_BASE_URL`                | L'URL de base de l'API que l'enfant utilisera, livrée par le plan de contrôle par session et normalement `https://api.anthropic.com`. Ne la remplacez pas : l'identifiant d'inférence de la session est un token OAuth émis par Anthropic que les autres fournisseurs n'acceptent pas, donc l'inférence dans les environnements auto-hébergés n'est pas routable ailleurs.                                                                                                                                                                                                                                                                                                                                                                                              |
| `CLAUDE_CODE_OAUTH_TOKEN`           | Le token d'accès OAuth de courte durée que l'enfant utilise pour l'inférence du modèle, limité à l'inférence du modèle et au téléchargement de fichiers uniquement, avec une durée de vie d'environ 30 minutes. Le runner le rémet avant l'expiration et livre la rotation sur stdin de l'enfant, donc un wrapper qui ne [garde pas stdin attaché](#keep-stdin-and-file-descriptor-3-attached) ne voit que la valeur initiale. Ne vous fiez pas à la liste d'adresses IP autorisées de votre organisation pour limiter l'utilisation de ce token : traitez-le comme un identifiant bearer qui reste utilisable pendant environ 30 minutes s'il fuit, et ne le consignez pas, ne l'écrivez pas sur le disque et ne le transmettez pas en dehors du conteneur de session. |

Le wrapper hérite également du reste de l'environnement géré de l'enfant, y compris toutes les variables d'environnement fournies par le serveur. `exec` propage tout automatiquement ; si votre wrapper lance l'enfant d'une autre manière, transmettez l'environnement complet.

<h3 id="keep-stdin-and-file-descriptor-3-attached">
  Garder stdin et le descripteur de fichier 3 attachés
</h3>

stdin de l'enfant est le canal de contrôle du runner. Les rotations de token et les signaux de fin de session arrivent dessus. Le runner ouvre également un tuyau sur le descripteur de fichier 3 et lit les signaux d'activité de l'enfant à partir de celui-ci pour piloter les délais d'inactivité et de démarrage. Un simple `exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"` préserve les deux automatiquement.

Si votre wrapper met l'enfant en arrière-plan avec un simple `&`, il coupe stdin de l'enfant : la session semble saine jusqu'à ce que la durée de vie du token OAuth initial d'environ 30 minutes expire, puis chaque appel API échoue avec `401 authentication_error`. Si votre wrapper doit mettre l'enfant en arrière-plan, par exemple pour garder un trap de démontage actif, enregistrez stdin sur le descripteur de fichier 4 ou supérieur et réattachez-le explicitement :

```bash theme={null}
exec 4<&0
"$CLAUDE_RUNNER_CLAUDE_BIN" "$@" <&4 4<&- &
CHILD=$!
trap 'teardown' EXIT
wait "$CHILD"
```

Ne fermez pas ou ne réutilisez pas le descripteur de fichier 3 dans le wrapper. Rediriger stdout et stderr de l'enfant est correct.

<h3 id="provision-credentials-scoped-to-the-session-creator">
  Provisionner les identifiants limités au créateur de la session
</h3>

Utilisez la sous-commande `decode-token` pour lire les revendications du JWT de session. Elle lit le token à partir d'un argument, de `CLAUDE_CODE_SESSION_ACCESS_TOKEN` ou de stdin, dans cet ordre ; voir [Vérifier le token à l'intérieur de la session](/docs/fr/self-hosted-environments-identity#verify-the-token-inside-the-session) pour ce qu'elle vérifie. L'exemple ci-dessous décode l'identité du créateur, l'échange contre des identifiants AWS de courte durée et exec dans Claude Code :

```bash theme={null}
#!/bin/bash
# Clé sur l'ID utilisateur Anthropic stable et exiger un créateur humain.
CREATOR_SUB=$("$CLAUDE_RUNNER_CLAUDE_BIN" self-hosted-runner decode-token \
  | jq -re '.act.sub // "" | select(startswith("user:"))') \
  || { echo "decode-token: verification failed or no human creator" >&2; exit 1; }

creds=$(your-sts-helper assume-role --subject "$CREATOR_SUB") \
  || { echo "credential exchange failed" >&2; exit 1; }
eval "$creds"

exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"
```

Utilisez `jq -re` plutôt que `jq -r` quand la revendication extraite contrôle une décision d'authentification, afin qu'une revendication absente se termine avec un code non-zéro au lieu de transmettre la chaîne littérale `null` en aval. Les sessions créées par une identité de service d'organisation, comme les sessions de bot et d'agent, portent un sujet `agent:` plutôt que `user:`, donc cet exemple les refuse ; si votre environnement sert ces sessions, décidez explicitement si le wrapper revient à un identifiant par défaut pour elles au lieu de se terminer. Quand votre échange d'identifiants a besoin du sujet SSO ou de l'email à la place, lisez `.act.attested_by.sub` ou `.act.email` et gérez leur absence : le token ne les porte que quand la surface de création les a enregistrés, et une [session envoyée par CLI](/docs/fr/self-hosted-environments-testing#run-the-test-loop) peut manquer les deux. Pour la référence complète des revendications et la vérification à partir de services en dehors du runner, voir [Vérifier l'identité de la session](/docs/fr/self-hosted-environments-identity).

<h2 id="lifecycle-hooks">
  Hooks de cycle de vie
</h2>

Les hooks de cycle de vie remplacent les étapes du pipeline par session du runner par vos propres scripts. Pointez le runner vers un répertoire de hooks avec `--hooks-dir <path>`, ou `SELF_HOSTED_RUNNER_HOOKS_DIR`. Le runner cherche des fichiers exécutables avec des noms bien connus ; tout hook qui n'est pas présent revient au comportement intégré, donc vous n'écrivez que ceux dont vous avez besoin. Les hooks s'exécutent avec les privilèges du runner lui-même, et les enfants de session partagent cet UID, donc montez le répertoire de hooks en lecture seule, ou intégrez-le à l'image, afin que le code de session ne puisse pas le modifier ; voir la [section de durcissement](/docs/fr/self-hosted-environments-deploy#harden-your-deployment).

Ces hooks sont distincts des [hooks Claude Code](/docs/fr/hooks), qui s'exécutent à l'intérieur de la session ; les hooks de cycle de vie s'exécutent sur le runner, autour de la session.

<h3 id="checkout">
  checkout
</h3>

S'exécute une fois par référentiel, à la place du clone et de la récupération intégrés du runner. Utilisez le hook pour cloner à partir d'un miroir de lecture directe, amorcer un arbre de travail à partir d'une archive ou appliquer une authentification git par session. Le runner définit :

| Variable                           | Description                                                                                                                                                     |
| :--------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_RUNNER_REPO_URL`           | URL du référentiel à cloner, après que tout `--git-host-rewrite` et `--git-ssh-rewrite` aient été appliqués                                                     |
| `CLAUDE_RUNNER_REPO_REF`           | Révision à vérifier : branche, tag ou SHA de commit comme la session l'a demandé. Vide signifie la branche par défaut du référentiel.                           |
| `CLAUDE_RUNNER_CHECKOUT_PATH`      | Chemin absolu où l'arbre de travail doit être laissé                                                                                                            |
| `CLAUDE_RUNNER_SESSION_ID`         | ID de session sous la forme balisée `session_...`, pour la journalisation et la corrélation                                                                     |
| `CLAUDE_RUNNER_SESSION_UUID`       | Le même ID de session sous la forme UUID canonique                                                                                                              |
| `CLAUDE_RUNNER_API_BASE_URL`       | URL de base de l'API Anthropic pour les appels limités à la session                                                                                             |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`    | La surface client qui a créé la session, comme `web_claude_ai`, `desktop_app` ou `ios`. Non défini quand la session n'a pas de surface enregistrée ou reconnue. |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN` | Le token d'accès de session, pour les appels API limités à la session                                                                                           |

Le script doit laisser un arbre de travail à `CLAUDE_RUNNER_CHECKOUT_PATH` vérifié à la révision demandée. HEAD détaché est correct ; le runner crée la branche de travail de la session par-dessus. Le runner vérifie que le chemin contient un `.git` après ; si votre hook matérialise une source non-git comme Perforce ou une archive dépaquetée, définissez `CLAUDE_RUNNER_SKIP_GIT_VERIFY=1` dans l'environnement du runner pour ignorer cette vérification. Les flux basés sur git comme la création de branche de travail et l'envoi de résultats nécessitent un checkout git, donc exportez les résultats à partir d'arbres non-git avec un hook [`post-session`](#post-session).

Le runner ne transmet pas un identifiant git au hook. À la place, frappez un identifiant de clone par session à partir de l'identité de la session : vérifiez `CLAUDE_CODE_SESSION_ACCESS_TOKEN` avec une bibliothèque JWT standard par rapport au point de terminaison JWKS sous `CLAUDE_RUNNER_API_BASE_URL`, comme décrit dans [Vérifier le token à partir de votre service](/docs/fr/self-hosted-environments-identity#verify-the-token-from-your-service), puis faites en sorte que votre service d'identifiants émette un identifiant de clone de courte durée pour l'identité dans la revendication `act` du token. `CLAUDE_RUNNER_CLAUDE_BIN` n'est pas défini dans l'environnement du hook de checkout, donc la sous-commande `decode-token` n'est pas disponible ici. Revenir à tout ce que l'authentification git de l'hôte a déjà, comme un agent SSH, un helper d'identifiants ou `.netrc`, est aussi une option.

Quand le hook se termine avec un code non-zéro, ou se termine avec 0 sans laisser un checkout utilisable derrière, ce que le runner fait dépend du référentiel :

* **Un référentiel vers lequel la session envoie les résultats** : le runner échoue la session, et sur une sortie non-zéro affiche la queue du stderr du script à l'utilisateur.
* **Un référentiel que la session lit uniquement**, comme un référentiel ajouté à une session en cours d'exécution : le runner enregistre une ligne `[runner:warn]` avec le détail de l'échec, affiche une étape `Skipped` à la session, supprime ce que le hook a laissé au chemin de checkout et continue avec les référentiels restants. Quand le runner ne peut pas supprimer le chemin immédiatement, il réessaie la suppression à la fin de la session. Si ignorer laisse la session sans aucun référentiel du tout, le runner échoue la session de toute façon.

Avant v2.1.228, le runner échouait la session sur un échec de hook pour tout référentiel, donc un référentiel en lecture seule que le hook ne pouvait pas servir échouait la session à nouveau sur chaque nouveau runner sur lequel la session reprenait.

Le runner supprime le chemin de checkout après la fin de la session.

<h3 id="post-session">
  post-session
</h3>

S'exécute une fois par session, après la sortie de l'enfant Claude Code et avant que le runner ne démonte l'espace de travail. Ce hook est votre seule chance de sauvegarder le travail non commis : à `--capacity` au-dessus d'un, le runner supprime les worktrees par session juste après le retour du hook, et à `--capacity 1` le [clone canonique](/docs/fr/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout) réutilisé est réinitialisé en dur quand la session suivante démarre, donc les changements suivis non commis ne survivent sur aucun chemin. Les utilisations typiques sont l'envoi d'une branche d'instantané de changements non commis, l'archivage de journaux ou l'émission d'un événement de fin de session à vos propres systèmes.

Le hook se déclenche à chaque fin de session où un processus enfant a été généré, quelle qu'en soit la cause ; les valeurs `CLAUDE_RUNNER_EXIT_REASON` ci-dessous énumèrent les cas. Il ne peut pas se déclencher quand le runner se termine brutalement, comme une préemption de VM ou une perte de courant ; si vous avez besoin de garanties contre une terminaison brutale, prenez des instantanés périodiquement à partir de l'intérieur de la session avec un hook Claude Code `PostToolUse` à la place. Le runner définit :

| Variable                           | Description                                                                                                                                                                                                  |
| :--------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_RUNNER_SESSION_ID`         | ID de session sous la forme balisée `session_...`                                                                                                                                                            |
| `CLAUDE_RUNNER_SESSION_UUID`       | Le même ID de session sous la forme UUID canonique                                                                                                                                                           |
| `CLAUDE_RUNNER_EXIT_REASON`        | Comment la session s'est terminée ; voir les valeurs ci-dessous du tableau                                                                                                                                   |
| `CLAUDE_RUNNER_WORKSPACE_PATHS`    | Chemins absolus séparés par des deux-points des arbres de travail de la session. Vide pour les sessions sans référentiel.                                                                                    |
| `CLAUDE_RUNNER_DEBUG_LOG_PATH`     | Chemin vers le journal de débogage de la session, toujours sur le disque pendant que le hook s'exécute                                                                                                       |
| `CLAUDE_RUNNER_API_BASE_URL`       | URL de base de l'API Anthropic pour les appels limités à la session                                                                                                                                          |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`    | La surface client qui a créé la session, comme `web_claude_ai`, `desktop_app` ou `ios`. Non défini quand la session n'a pas de surface enregistrée ou reconnue. Nécessite Claude Code v2.1.229 ou ultérieur. |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN` | Le token d'accès de session, pour les appels API limités à la session                                                                                                                                        |

`CLAUDE_RUNNER_EXIT_REASON` prend l'une des quatre valeurs :

* `completed` : la session s'est terminée proprement. Le processus Claude Code s'est terminé normalement, ou la session a été archivée ou supprimée pendant qu'elle était toujours en cours d'exécution.
* `failed` : le processus Claude Code s'est écrasé, ou la configuration a échoué après son démarrage.
* `interrupted` : le runner a arrêté la session. Il a libéré la session pour libérer l'emplacement, la session a expiré au démarrage, le serveur a déplacé la session hors de ce runner, le runner était en drainage, ou la session a dépassé sa limite [`--kill-session-after-min`](/docs/fr/self-hosted-environments-reference#runner-cli-flags).
* `abandoned` : réservé à une session qu'un autre runner a revendiquée. Le hook ne se déclenche actuellement pas dans ce cas.

Les [compteurs de cycle de vie de la session](/docs/fr/self-hosted-environments-reference#session-lifecycle-counter-semantics) comptent une libération, un délai d'expiration au démarrage et un déplacement du serveur comme `completed` plutôt que `interrupted`, car le runner a remis l'emplacement proprement. Attendez-vous à cette différence si vous comparez les reçus de hook avec les compteurs.

Le statut de sortie du hook n'affecte jamais le résultat de la session ; un échec est enregistré et ignoré. Le runner attend jusqu'à `--post-session-hook-timeout-sec`, 60 secondes par défaut, à chaque fin de session y compris l'arrêt du runner. Cet exemple sauvegarde le travail non commis dans une branche de secours :

```bash theme={null}
#!/usr/bin/env bash
set -u
IFS=':'
# Épinglez la configuration que la session aurait pu planter dans le .git/config du checkout :
# -c les remplacements battent les paramètres au niveau du référentiel, bloquant la session écrite fsmonitor,
# hook-path et gpg-program config de l'exécution de code avec les privilèges du hook.
# Repo-local credential.helper, core.sshCommand et pushurl
# s'appliquent toujours ; si le hook détient des identifiants que la session n'avait pas, épinglez le
# push URL et helper aussi (voir la note ci-dessous le script).
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

Le hook envoie avec tout ce que les identifiants git disponibles dans son propre environnement sur l'hôte runner. Sous la [posture sans identifiants dans l'image](/docs/fr/self-hosted-environments-deploy#configure-git), y compris quand le clone intégré passe par le proxy git Anthropic, il n'y en a pas, donc frappez un identifiant de push de courte durée à l'intérieur du hook avant d'envoyer : échangez le token de session que le hook reçoit dans `CLAUDE_CODE_SESSION_ACCESS_TOKEN` avec votre propre service de token, en le vérifiant comme [Vérifier l'identité de la session](/docs/fr/self-hosted-environments-identity) le décrit. Quand le hook détient un identifiant que la session n'avait pas, épinglez aussi où il envoie : remplacez `origin` par une URL fournie par l'opérateur et passez `-c credential.helper=` plus votre propre helper, afin que la configuration au niveau du référentiel que la session a écrite ne puisse pas rediriger l'envoi accrédité.

<h4 id="hook-timing-when-the-runner-releases-a-session">
  Timing du hook quand le runner libère une session
</h4>

Une session libérée peut reprendre sur un autre runner. Sur un runner en v2.1.236 ou ultérieur, ce que la session faisait à la libération décide si elle peut reprendre avant que ce hook se termine :

* **Inactif après un tour, ou expiré au démarrage** : le runner arrête l'enfant et exécute ce hook jusqu'à la fin. Ce n'est qu'alors qu'il libère la session. Un message utilisateur envoyé pendant que le hook s'exécute ne peut pas reprendre la session sur un autre runner avant que le hook se termine.
* **En attente que l'utilisateur réponde à une invite, comme une invite de permission** : le runner libère d'abord la session, puis exécute ce hook. Un message utilisateur envoyé pendant que le hook s'exécute peut reprendre la session sur un autre runner avant que le hook se termine.

Ceci s'applique chaque fois que le runner libère une session : au délai d'inactivité, à l'heure [`--retire-at`](/docs/fr/self-hosted-environments-reference#runner-cli-flags), et, sur un runner en v2.1.260 ou ultérieur, à la limite [`--kill-session-after-min`](/docs/fr/self-hosted-environments-reference#runner-cli-flags) d'une session. Une session dont le tour s'est terminé et qui ne contient que des tâches en arrière-plan compte comme inactif ici. Avant v2.1.236, le runner libérait d'abord la session puis exécutait ce hook dans les deux cas.

Pendant un drainage `SIGTERM`, le runner maintient le bail de session jusqu'à ce que le hook se termine ; voir [Timing d'arrêt](/docs/fr/self-hosted-environments-deploy#shutdown-timing).

<h3 id="command">
  command
</h3>

S'exécute une fois par session après le checkout, à la place du spawn enfant intégré. Le hook reçoit le même environnement qu'un [script wrapper](#wrapper-scripts) et doit `exec` dans `"$CLAUDE_RUNNER_CLAUDE_BIN"` de la même manière. Utilisez le hook `command` pour garder toute la personnalisation dans un répertoire de hooks ; utilisez `--exec-path` quand le wrapper vit ailleurs. Si `--exec-path` est aussi défini, le flag prend la priorité et le hook `command` est ignoré.

Toujours `exec` le binaire du runner plutôt qu'un `claude` résolu par PATH ; sinon vous annulez l'[épinglage de version](/docs/fr/self-hosted-environments-deploy#pin-the-version).

<h2 id="on-demand-runners">
  Runners à la demande
</h2>

Au lieu d'exécuter une flotte fixe, vous pouvez démarrer un runner par session. L'orchestrateur est une sous-commande séparée et sans état qui interroge Anthropic pour les demandes de spawn, une par session qui est en attente sans runner disponible, et exécute votre hook `spawn-runner` pour chacune. Votre hook soumet une charge de travail à votre plateforme : un Kubernetes Job, une instance EC2, un Nomad dispatch.

Les runners à la demande améliorent l'hygiène des identifiants. Sur une flotte fixe, le secret d'environnement vit sur chaque hôte runner, qui est le même hôte qui exécute les sessions utilisateur. Avec l'orchestrateur, le secret d'environnement reste uniquement sur l'hôte orchestrateur, qui n'exécute jamais le code utilisateur ; chaque runner généré reçoit un bon de travail à usage unique qui enregistre exactement un runner puis expire.

Pour démarrer l'orchestrateur, passez le secret d'environnement et un répertoire de hooks contenant un script `spawn-runner` exécutable :

```bash theme={null}
claude self-hosted-runner orchestrator \
  --environment-secret-file /etc/claude/environment-secret \
  --hooks-dir /etc/claude/hooks
```

L'orchestrateur ne conserve aucun état entre les sondages, donc vous pouvez exécuter deux ou plusieurs répliques contre le même environnement pour la disponibilité. Chaque demande de spawn est revendiquée côté serveur par exactement une réplique. Toutes les répliques doivent utiliser la même valeur `--expected-spawn-seconds` ; voir le [contrat du hook](#the-spawn-runner-hook).

<h3 id="the-spawn-runner-hook">
  Le hook spawn-runner
</h3>

L'orchestrateur exécute `${hooks-dir}/spawn-runner` une fois par demande de spawn. Le hook doit soumettre le travail de manière asynchrone, sans attendre le démarrage du runner, et revenir dans `--hook-timeout`, 60 secondes par défaut. Le hook reçoit :

| Variable                              | Description                                                                                                                                                                                                                                                                                                                                            |
| :------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_RUNNER_WORK_ORDER_FILE`       | Chemin vers un fichier temporaire contenant le JWT du bon de travail signé avec lequel le nouveau runner s'enregistre. Supprimé après la sortie du hook. Ne consignez pas le contenu du fichier.                                                                                                                                                       |
| `CLAUDE_RUNNER_ORDER_ID`              | Clé d'idempotence opaque, unique par demande de spawn et sûre pour les noms de ressources Kubernetes. Utilisez-la comme clé de déduplication de votre approvisionneur.                                                                                                                                                                                 |
| `CLAUDE_RUNNER_SESSION_ID`            | La session pour laquelle cette demande est. Vide pour les demandes de pré-réchauffage, qui démarrent un runner de secours avant toute session spécifique quand [`--min-idle`](/docs/fr/self-hosted-environments-reference#orchestrator-cli-flags) est défini, donc ne supposez pas que la variable est définie.                                             |
| `CLAUDE_RUNNER_SESSION_UUID`          | Le même ID de session sous la forme UUID canonique. Vide pour les demandes de pré-réchauffage.                                                                                                                                                                                                                                                         |
| `CLAUDE_RUNNER_ATTEMPT`               | Combien de demandes de spawn cette session a eues. `0` pour les demandes de pré-réchauffage.                                                                                                                                                                                                                                                           |
| `CLAUDE_RUNNER_ORDER_SERVER_TIME`     | Heure du serveur à partir de l'en-tête HTTP `Date` de la réponse du sondage. Quand le hook vérifie le `exp` du JWT du bon de travail, comparez par rapport à cette valeur au lieu de l'horloge locale pour tolérer l'asymétrie. Vide quand la passerelle a omis l'en-tête.                                                                             |
| `CLAUDE_RUNNER_POOL_ID`               | L'ID de l'environnement auquel le nouveau runner doit se joindre, sous la forme `ccpool_...`                                                                                                                                                                                                                                                           |
| `CLAUDE_RUNNER_ACCOUNT_ID`            | ID balisé du compte qui a mis en attente la session, pour l'acheminement par compte, le quota ou la rétrofacturation. Vide quand indisponible, et toujours vide pour les sessions du canal Claude Tag, qu'aucun compte ne met en attente.                                                                                                              |
| `CLAUDE_RUNNER_ACCOUNT_EMAIL`         | Email du compte qui a mis en attente la session. Vide quand indisponible. Traitez l'email comme des informations d'identification personnelle et ne le consignez pas.                                                                                                                                                                                  |
| `CLAUDE_RUNNER_PRIMARY_REPO_URL`      | URL de la première source git de la session, pour l'acheminement vers un runner avec ce référentiel pré-réchauffé. Vide quand la session n'a pas de sources git.                                                                                                                                                                                       |
| `CLAUDE_RUNNER_PRIMARY_REPO_REVISION` | Révision de la première source git de la session : branche, SHA ou tag. Vide quand non spécifié.                                                                                                                                                                                                                                                       |
| `CLAUDE_RUNNER_REPO_SOURCES`          | Tableau JSON de `{url, revision}` pour toutes les sources git de la session, pour les hooks qui acheminent sur un référentiel secondaire. Vide quand il n'y a pas de sources.                                                                                                                                                                          |
| `CLAUDE_RUNNER_CORRELATION_ID`        | L'ID de corrélation fourni à la création de la session, renvoyé afin que le hook puisse mapper ce bon de travail à la demande qui a créé la session. Vide quand la session n'en a pas.                                                                                                                                                                 |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`       | La surface client qui a créé la session, comme `web_claude_ai`, `desktop_app`, `ios` ou `scheduled_trigger`, pour l'analyse d'adoption. Non défini quand la session n'a pas de surface enregistrée ou reconnue, et pour les demandes de pré-réchauffage ; vérifiez-le avec `[ -n "${CLAUDE_RUNNER_CLIENT_PLATFORM:-}" ]`, qui reste sûr sous `set -u`. |

Le runner généré s'enregistre avec le bon de travail à la place du secret d'environnement :

* **Démarrez-le avec le bon de travail** : pointez [`--environment-secret-file`](/docs/fr/self-hosted-environments-reference#runner-cli-flags) vers un fichier contenant le JWT du bon de travail, ou définissez `SELF_HOSTED_RUNNER_ENVIRONMENT_SECRET` à la valeur JWT.
* **Copiez le JWT avant la sortie du hook** : l'orchestrateur supprime le fichier du bon de travail après la sortie du hook, donc copiez le JWT dans la charge de travail que vous soumettez, comme un Secret Kubernetes sur le Job généré, plutôt que de transmettre le chemin du fichier.
* **Utilisez `--capacity 1` sur les runners générés** : un bon de travail lié à une session enregistre exactement un runner lié à cette session, donc une capacité plus élevée ajoute des emplacements qui ne reçoivent jamais de travail, et le runner enregistre un avertissement au démarrage.
* **Les bons de travail de pré-réchauffage s'enregistrent sans liaison** : le runner de secours n'est pas lié à une session et revendique le travail en attente comme un runner de flotte fixe.

Le contrat a quatre règles agnostiques de l'approvisionneur :

1. **Soyez idempotent sur `CLAUDE_RUNNER_ORDER_ID`.** La redélivraison de la même demande doit générer au maximum un runner. Dérivez un nom de ressource déterministe à partir de l'ID et laissez votre plateforme rejeter le doublon.
2. **Ne réessayez pas la charge de travail.** Un ID de commande signifie au maximum une charge de travail créée. Si le runner ne s'enregistre jamais, Anthropic re-demande avec un ID de commande frais après `--expected-spawn-seconds`.
3. **Utilisez le contrat du code de sortie.** Sortie 0 signifie soumis. Sortie 1 signifie échec réessayable ; la session recule et est re-proposée. Sortie 2 ou supérieure signifie non-réessayable ; la session est bloquée du spawning à nouveau jusqu'à ce qu'un [Owner](/docs/fr/cloud-environments#organization-shared-environments) sélectionne **Retry** sur elle dans l'onglet **Activity** de l'environnement. Sur une sortie non-zéro, la queue du stderr du hook apparaît là comme la raison de l'échec, donc écrivez l'erreur exploitable sur stderr et jamais les secrets. Pour une demande de pré-réchauffage il n'y a pas de session à échouer : l'orchestrateur enregistre une sortie non-zéro localement uniquement, et le serveur re-demande le spawn après le bail.
4. **Définissez `--expected-spawn-seconds` à au moins votre temps de démarrage p99.** C'est le bail côté serveur. Toutes les répliques d'orchestrateur doivent utiliser la même valeur.

Tout ce que le hook écrit sur stdout ou stderr apparaît dans le journal de l'orchestrateur avec les identifiants automatiquement supprimés. Si les sessions restent en attente, vérifiez le corps `/healthz` de l'orchestrateur pour les compteurs de file d'attente, puis ouvrez l'onglet **Activity** de votre environnement sur la [page d'administration **Cloud environments**](https://claude.ai/admin-settings/cloud-environments) : développez une session échouée là pour son erreur de spawn, et sélectionnez **Retry** pour la re-demander.

<h2 id="mcp-servers">
  Serveurs MCP
</h2>

Pour rendre les [serveurs MCP](/docs/fr/mcp) disponibles dans chaque session, ajoutez-les au moment de la construction de l'image avec la même commande `claude mcp add` utilisée sur une installation de bureau. Si votre runner est un processus nu plutôt qu'un conteneur, exécutez la même commande en tant qu'utilisateur du runner sur l'hôte, puis redémarrez le runner : il lit la configuration de l'hôte une fois au démarrage. Le flag `--scope user` est requis ; le scope local par défaut écrit sous une clé par répertoire que le runner ne sème pas dans les sessions. Par exemple, dans votre Dockerfile :

```dockerfile theme={null}
RUN claude mcp add --scope user sidecar -- /usr/local/bin/mcp-sidecar
RUN claude mcp add --scope user --transport http internal http://mcp-gateway.svc.cluster.local:8080
```

Le runner prend un instantané de la configuration de l'hôte une fois au démarrage. L'instantané capture la clé `mcpServers` du `.claude.json` de l'hôte, qui vit à côté plutôt qu'à l'intérieur de `~/.claude/`, et le runner sème uniquement cette clé dans la configuration isolée de chaque session ; l'état du compte et l'historique du projet sont supprimés. Pour confirmer que les serveurs ont atteint les sessions, démarrez une session sur l'environnement et demandez à Claude de lister ses outils MCP ; le runner enregistre également un avertissement de démarrage pour toute entrée capturée dont le `type` n'est pas reconnu et supprime l'entrée, afin que vous puissiez voir pourquoi ce serveur manque des sessions. Quand `SELF_HOSTED_RUNNER_HOST_CONFIG_DIR` est défini, le runner lit `.claude.json` à partir de ce répertoire à la place, donc pointer la variable vers un répertoire vide désactive aussi le semis MCP.

Claude Code charge également les serveurs MCP à partir d'autres sources :

* Le fichier MCP géré au scope d'entreprise [managed MCP file](/docs/fr/managed-mcp) à son chemin système standard : `/etc/claude-code/managed-mcp.json` sur les hôtes runners Linux, `/Library/Application Support/ClaudeCode/managed-mcp.json` sur les hôtes macOS. Utilisez-le pour les flottes verrouillées où seuls les serveurs listés par l'administrateur peuvent charger. Voir [contrôle exclusif avec managed-mcp.json](/docs/fr/managed-mcp#exclusive-control-with-managed-mcp-json) pour les règles de précédence. Quand ce fichier est sur l'hôte runner, Claude Code ignore les serveurs MCP que le plan de contrôle d'Anthropic livre à une session, y compris les connecteurs claude.ai, et les nomme dans un avertissement sur stderr de l'enfant de la session, que le runner enregistre au niveau de journal `debug`. Avant v2.1.229, ces sessions se terminaient au démarrage avec `You cannot dynamically configure MCP servers when an enterprise MCP config is present`.
* La clé [`managedMcpServers`](/docs/fr/settings-reference#managedmcpservers) dans les [paramètres gérés](/docs/fr/managed-settings) sur l'hôte runner : fournit les serveurs HTTP et SSE sans prendre le contrôle exclusif, donc les serveurs des autres sources se chargent toujours. Nécessite Claude Code v2.1.259 ou ultérieur.
* `<repo>/.mcp.json` : scope du projet. Validez le fichier dans le référentiel ; ses serveurs sont pré-approuvés dans les sessions cloud.

Quand la livraison de connecteur est activée pour votre organisation, le plan de contrôle d'Anthropic livre les connecteurs que vous avez configurés sur claude.ai aux sessions créées de manière interactive via la configuration MCP fournie par le serveur, acheminée via `api.anthropic.com`. Les sessions créées par programmation, comme les [envois CLI](/docs/fr/self-hosted-environments-testing#run-the-test-loop), ne reçoivent pas la livraison de connecteur ; donnez-leur des serveurs MCP via l'une des autres sources que cette section énumère à la place. Le token OAuth de l'enfant ne porte pas un scope pour récupérer les connecteurs directement, donc l'enfant ne tente pas cette récupération lui-même ; la livraison est pilotée par le serveur.

`settings.json` ne porte pas de définitions de serveur MCP, et il n'y a pas de champ `mcpServers` de haut niveau dans le schéma des paramètres. Dans les paramètres gérés, fournissez les serveurs avec la clé [`managedMcpServers`](/docs/fr/settings-reference#managedmcpservers) à la place.

Les sessions héritent de l'environnement du runner, donc définissez [`ENABLE_TOOL_SEARCH`](/docs/fr/mcp#scale-with-mcp-tool-search) là pour contrôler la recherche d'outils MCP pour chaque session qu'un runner génère ; la page MCP couvre les valeurs.

<h2 id="prompt-sessions-to-push-their-work">
  Inviter les sessions à envoyer leur travail
</h2>

Les sessions hébergées par Anthropic exécutent un hook [`Stop`](/docs/fr/hooks#stop), le hook Claude Code qui s'exécute quand Claude finit de répondre, qui invite Claude à valider et envoyer son travail. Le runner n'en installe pas. Sans lui, une session qui se termine avec des changements non commis laisse ce travail uniquement sur le disque du runner, et le bouton **Create PR** dans claude.ai/code reste inactif jusqu'à ce que la branche existe sur le distant.

L'implémentation de référence ci-dessous a deux parties. Fusionnez le bloc de paramètres dans `~/.claude/settings.json` sur l'hôte runner, que le runner sème dans chaque session, et enregistrez le script comme `~/.claude/hooks/stop-hook-nudge.sh` sur l'hôte runner et rendez-le exécutable :

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
# Implémentation de référence du hook Stop pour les runners auto-hébergés.
#
# Pousse Claude une fois par tour si le répertoire du projet a des changements
# non commis OU des commits non envoyés, afin que le travail ne soit pas perdu
# quand une session inactive est libérée et afin que le bouton "Create PR" sur
# claude.ai/code s'allume.
#
# Niveau runner (pas de changements de référentiel) : déposez ce fichier à
# ~/.claude/hooks/ sur l'hôte runner et fusionnez le bloc de paramètres du
# hook Stop accompagnant dans ~/.claude/settings.json — le runner sème les deux
# dans chaque session.
# Alternative au niveau du référentiel : validez dans <repo>/.claude/hooks/ et
# changez le chemin de la commande settings.json en $CLAUDE_PROJECT_DIR/.claude/hooks/.
#
# stdin : charge utile JSON du hook (voir https://code.claude.com/docs/en/hooks)
# stdout : {"decision":"block","reason":"..."} pour pousser, ou rien pour permettre l'arrêt.

# Garde de re-entrée : le harnais définit stop_hook_active=true lors de la
# re-invocation du hook Stop après un bloc. Sortez afin que nous ne poussions
# qu'une fois par tour. Le harnais émet du JSON compact (pas d'espace après les
# deux-points), sur lequel ce motif s'appuie ; utilisez jq si vous avez besoin
# d'une vérification tolérante aux espaces.
in=$(cat)
case "$in" in *'"stop_hook_active":true'*) exit 0 ;; esac

d="$CLAUDE_PROJECT_DIR"

# Pas un référentiel git → rien à pousser.
git -C "$d" rev-parse --git-dir >/dev/null 2>&1 || exit 0

# Pas de distant → « envoyer au distant » est insatisfaisable ; sortez.
[ -z "$(git -C "$d" remote 2>/dev/null)" ] && exit 0

# Changements non commis (en attente, non en attente ou non suivis). Excluez
# .claude/ entièrement — les paramètres semés par l'opérateur et l'état
# d'exécution écrit par CLI (verrou du planificateur, worktrees, état de
# routine) vivent là et ni l'un ni l'autre n'est du « travail non commis »
# que le modèle doit envoyer.
s=$(git -C "$d" status --porcelain -- . ':(exclude).claude/' 2>/dev/null)
if [ -n "$s" ]; then
  printf '{"decision":"block","reason":"There are uncommitted changes in the repository. Please commit and push these changes to the remote branch."}'
  exit 0
fi

# Commits non envoyés. Comptez les commits sur HEAD non accessibles à partir
# d'aucune ref de suivi distant ou FETCH_HEAD. Cela fonctionne uniformément
# pour :
#   - checkouts init+fetch (runner par défaut : seul FETCH_HEAD existe)
#   - checkouts basés sur clone (origin/* existent)
#   - le runner par défaut : l'enfant démarre sur la branche de résultat de
#     la session, que le runner crée après le checkout
#   - HEAD détaché, quand une configuration personnalisée ignore cette création
#     de branche
# Sans point de référence du tout (jamais récupéré), restez silencieux plutôt
# que de faux positif sur un tour en lecture seule.
base=""
git -C "$d" rev-parse --verify -q FETCH_HEAD >/dev/null && base="FETCH_HEAD"
if [ -z "$base" ] && [ -z "$(git -C "$d" for-each-ref --count=1 refs/remotes/origin 2>/dev/null)" ]; then
  exit 0
fi
# shellcheck disable=SC2086  # $base est soit "" soit "FETCH_HEAD", division de mot intentionnelle
unpushed=$(git -C "$d" rev-list HEAD --not $base --remotes=origin --count 2>/dev/null) || unpushed=0
if [ "$unpushed" -gt 0 ]; then
  branch=$(git -C "$d" symbolic-ref --short -q HEAD)
  if [ -n "$branch" ]; then
    # $branch est influencée par l'attaquant — git-check-ref-format(1) permet
    # les `` dans les noms de ref. `\` est interdit (règle 10) mais échappé
    # de toute façon comme défense en profondeur bon marché.
    # Échappez les métacaractères JSON avant d'interpoler dans la charge utile
    # construite à la main afin qu'une branche comme x","continue":false ne
    # puisse pas injecter de clés dans le JSON de sortie du hook que le harnais
    # analyse. $unpushed est sûr — la garde -gt ci-dessus rejette tout ce qui
    # n'est pas un entier simple.
    branch_esc=$(printf '%s' "$branch" | sed 's/\\/\\\\/g; s/"/\\"/g')
    printf '{"decision":"block","reason":"There are %s unpushed commit(s) on branch '\''%s'\''. Please push these changes to the remote repository."}' "$unpushed" "$branch_esc"
  else
    printf '{"decision":"block","reason":"There are %s unpushed commit(s) on a detached HEAD. Please create a branch and push it to the remote repository."}' "$unpushed"
  fi
  exit 0
fi

exit 0
```

Le hook pousse Claude à valider et envoyer avant la fin de la session, et reste silencieux quand le répertoire n'est pas un référentiel git ou n'a pas de distant.

<h2 id="permissions-and-tool-approval">
  Permissions et approbation d'outils
</h2>

Une session auto-hébergée n'a pas de terminal attaché, donc une invite de permission sans réponse bloque le tour jusqu'à ce que l'utilisateur réponde dans l'interface utilisateur. Le plan de contrôle d'Anthropic envoie la liste des outils de chaque session et les règles de permission avec la charge utile de travail ; la configuration par défaut pré-approuve les appels d'outils de routine, y compris `Bash`, et les sessions cloud [pré-approuvent les éditions de fichiers quel que soit le mode](/docs/fr/permission-modes#switch-permission-modes). Un appel que rien ne pré-approuve invite via l'interface utilisateur de la session.

<Note>
  Épinglez uniquement le mode auto sur un environnement dont les conteneurs de session s'exécutent avec [sortie réseau par défaut-refuser](/docs/fr/self-hosted-environments-deploy#default-deny-egress) et le reste de la [section de durcissement](/docs/fr/self-hosted-environments-deploy#harden-your-deployment) en place. Les appels d'outils de routine, y compris les demandes réseau `Bash`, s'exécutent sans un humain dans la boucle sur l'ensemble d'outils pré-approuvés par défaut et en mode auto, donc la limite réseau est ce qui limite où ces appels peuvent atteindre.
</Note>

Pour garder les invites au minimum quel que soit ce que le plan de contrôle envoie, épinglez le [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) à partir de votre script wrapper ou du hook [`command`](#command). Le mode auto permet aux sessions de s'exécuter sans invites de permission de routine : un modèle de classificateur séparé examine les actions avant qu'elles ne s'exécutent et bloque celles qu'il rejette, et les règles d'ask explicites forcent toujours une invite ; la page des modes de permission couvre ce que le classificateur vérifie. Le runner ajoute les flags calculés par le serveur avant d'invoquer le wrapper, et pour les flags à valeur unique comme `--permission-mode` l'analyseur honore la dernière occurrence, donc un flag que vous ajoutez après `"$@"` remplace la valeur envoyée par le serveur :

```bash theme={null}
#!/bin/bash
exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@" --permission-mode auto
```

Pour pré-approuver des outils spécifiques à la place, ajoutez `--allowed-tools` avec vos règles, par exemple `--allowed-tools "Bash(bazel *) Bash(yarn *) mcp__internal__*"`. Les flags de liste comme `--allowed-tools` et `--disallowed-tools` s'accumulent entre les occurrences plutôt que de se remplacer, donc vos règles s'appliquent en plus de toutes les règles que le plan de contrôle envoie. Pour réduire, ajoutez `--disallowed-tools`, qui refuse les outils même si une autre règle les permet.

<h3 id="how-each-session’s-config-is-assembled">
  Comment la configuration de chaque session est assemblée
</h3>

Le runner donne à chaque session son propre répertoire de configuration, semé à partir d'un instantané du `~/.claude/` de l'hôte que le runner capture une fois au démarrage : `settings.json`, `CLAUDE.md`, hooks, agents, commandes et skills dans votre image runner s'appliquent à chaque session comme la ligne de base au niveau utilisateur. Si vous modifiez la configuration sur un hôte en cours d'exécution, la modification ne prend effet qu'après le redémarrage du runner.

Définissez `SELF_HOSTED_RUNNER_HOST_CONFIG_DIR` pour semer à partir d'un chemin différent, ou pointez-le vers un répertoire vide pour désactiver le semis.

Le `.claude/settings.json` validé dans le référentiel se superpose comme paramètres du projet. Les sessions lisent également [`managed-settings.json`](/docs/fr/settings#where-settings-live) à partir du chemin système standard dans votre image runner. Que ses clés s'appliquent aux côtés des [paramètres gérés par le serveur](/docs/fr/server-managed-settings) suit [comment Claude Code combine les sources gérées](/docs/fr/managed-settings#how-claude-code-combines-managed-sources) : par défaut, quand votre organisation livre des clés gérées par le serveur, les sessions ignorent le fichier de l'image runner à part les [clés que Claude Code lit à partir de chaque source d'administration](/docs/fr/managed-settings#keys-read-from-every-admin-source), comme le bloc `env`, les verrous de sandbox, les chemins binaires de sandbox et `forceRemoteSettingsRefresh`. Voir [précédence des paramètres](/docs/fr/settings#settings-precedence).

Quand le plan de contrôle d'Anthropic fournit une session avec des [hooks Claude Code](/docs/fr/hooks), le runner les installe aux côtés, pas par-dessus, votre propre configuration. Nécessite Claude Code v2.1.229 ou ultérieur.

* **Où ils atterrissent** : le runner écrit chaque script de hook fourni dans un sous-répertoire réservé `hooks/.ccr-launcher/` du répertoire de configuration de la session et enregistre les scripts dans un fichier de paramètres séparé qu'il transmet à la session avec `--settings`, laissant le `settings.json` semé et vos propres scripts à `hooks/<name>` intacts. Le runner recrée le sous-répertoire réservé pour chaque session et ne sème pas le contenu de l'hôte à `~/.claude/hooks/.ccr-launcher/` dans les sessions.
* **Qui les crée** : le plan de contrôle remplit les scripts à partir de constantes fixes dans son propre déploiement, jamais à partir d'entrées par session ou tierces.
* **Ce qui les gouverne toujours** : les hooks livrés via `--settings` entrent dans la configuration de hook fusionnée ordinaire, pas le niveau géré, donc vos paramètres gérés s'appliquent toujours. `disableAllHooks` les désactive, et ils ne font pas partie des catégories que [`allowManagedHooksOnly`](/docs/fr/settings-reference#allowmanagedhooksonly) garde chargées.

<h3 id="repository-committed-permission-rules">
  Règles de permission validées dans le référentiel
</h3>

Ne mettez pas une entrée `"Edit"`, `"Write"` ou `"NotebookEdit"` nue dans un `permissions.allow` validé dans le référentiel. Une règle d'outil de fichier nue correspond à l'outil quel que soit le chemin, accordant des écritures n'importe où sur l'hôte plutôt que uniquement l'espace de travail, donc la garde de confinement de portée d'écriture du runner signale la session ; avec [`--confine-repo-settings enforce`](/docs/fr/self-hosted-environments-reference#runner-cli-flags) elle refuse de générer la session au lieu de consigner et continuer. Voir la [section de durcissement](/docs/fr/self-hosted-environments-deploy#harden-your-deployment).

Un référentiel n'a besoin d'aucune règle d'outil de fichier du tout : les sessions cloud [pré-approuvent les éditions de fichiers quel que soit le mode](/docs/fr/permission-modes#switch-permission-modes). Si vous validez une règle, limitez-la à l'espace de travail, comme `"Edit(/**)"`; une seule barre oblique de début est relative à la racine du projet, qui est l'espace de travail de la session. Les règles d'outil de fichier nues sont correctes dans le `settings.json` au niveau de l'hôte de l'opérateur, puisque ce fichier n'est pas validé dans le référentiel.

Un `defaultMode` de `auto` n'est honoré que à partir du fichier de paramètres au niveau de l'image ou au niveau utilisateur, donc un référentiel extrait ne peut pas se donner le mode auto. Pour les modes que les sessions cloud acceptent et la syntaxe complète des règles, voir [modes de permission](/docs/fr/permission-modes).

<h2 id="what’s-next">
  Prochaines étapes
</h2>

* [Référence](/docs/fr/self-hosted-environments-reference) : chaque flag CLI, variable d'environnement et métrique
* [Vérifier l'identité de la session](/docs/fr/self-hosted-environments-identity) : valider le token de session à partir de services en dehors du runner
