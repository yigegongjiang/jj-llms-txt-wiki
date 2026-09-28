> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Utiliser Claude Code dans le cloud

> Exécutez les sessions Claude Code dans le cloud depuis votre navigateur, téléphone, application de bureau ou terminal, déplacez-les avec --cloud et --teleport, et corrigez automatiquement les demandes de tirage.

<Note>
  Les sessions cloud sont disponibles sur les plans Pro, Max et Team, ainsi que pour les utilisateurs Enterprise disposant de sièges premium ou de sièges Chat + Claude Code.
</Note>

Une session cloud est une session Claude Code qui s'exécute sur l'infrastructure cloud au lieu de sur votre machine. Par défaut, elle s'exécute sur l'infrastructure gérée par Anthropic, ou sur l'[environnement auto-hébergé](/docs/fr/self-hosted-environments) de votre organisation lorsqu'elle y est acheminée. La session continue de s'exécuter après que vous fermiez votre ordinateur portable, et vous pouvez la vérifier ou la diriger depuis n'importe quel appareil.

Vous pouvez démarrer une session cloud à partir de l'une de ces surfaces :

* **Navigateur** : [claude.ai/code](https://claude.ai/code), également appelé Claude Code sur le web
* **Mobile** : l'onglet **Code** dans l'[application Claude](/docs/fr/mobile)
* **Application de bureau** : sélectionnez **Cloud** au lieu de **Local** lorsque vous [démarrez une session](/docs/fr/desktop#run-long-running-tasks-in-the-cloud)
* **Terminal** : [`claude --cloud`](#from-terminal-to-cloud)
* **Routines** : les [exécutions planifiées et déclenchées](/docs/fr/routines) s'exécutent chacune en tant que session cloud

Pour que Claude démarre et suive de nombreuses sessions cloud pour un corps de travail, utilisez un [projet](/docs/fr/claude-projects). Une session dans votre terminal, votre IDE ou l'application de bureau avec **Local** sélectionné s'exécute sur votre propre machine à la place. Pour diriger l'une de ces sessions locales depuis votre téléphone ou navigateur, utilisez [Contrôle à distance](/docs/fr/remote-control).

<Tip>
  Nouveau sur les sessions cloud ? Commencez par [Démarrer](/docs/fr/web-quickstart) pour connecter votre compte GitHub et soumettre votre première tâche.
</Tip>

Cette page couvre :

* [Environnements cloud](#cloud-environments) : où les sessions s'exécutent et où configurer cela
* [Options d'authentification GitHub](#github-authentication-options) : deux façons de connecter GitHub
* [Déplacer les tâches entre le terminal et le cloud](#move-tasks-between-terminal-and-cloud) avec `--cloud` et `--teleport`
* [Travailler avec les sessions](#work-with-sessions) : modes de permission, examen, partage, archivage, suppression
* [Correction automatique des demandes de tirage](#auto-fix-pull-requests) : répondre automatiquement aux défaillances CI et aux commentaires d'examen
* [Sécurité et isolation](#security-and-isolation) : comment les sessions sont isolées
* [Limitations](#limitations) : limites de débit et restrictions de plateforme

<h2 id="cloud-environments">
  Environnements cloud
</h2>

Chaque session cloud s'exécute dans un [environnement cloud](/docs/fr/cloud-environments), la configuration enregistrée qui contrôle l'accès réseau, les variables d'environnement et les scripts de configuration. Si vous n'avez pas encore d'environnement, l'intégration configure un environnement **Default** avec [accès réseau **Trusted**](/docs/fr/cloud-environments#access-levels), soit en le créant pour vous, soit en vous demandant de le créer. Consultez [L'environnement Default](/docs/fr/cloud-environments#the-default-environment) pour savoir lequel se produit sur votre plan et comment les sessions choisissent un environnement lorsque vous en avez plus d'un.

Les mêmes environnements s'appliquent partout où vous démarrez une session cloud : le web, le terminal, [Claude Tag](https://claude.com/docs/claude-tag/overview), [routines](/docs/fr/routines) et les applications mobile et Desktop. Les sessions de canal Claude Tag utilisent uniquement les environnements au niveau de l'organisation, soit les [environnements partagés](/docs/fr/cloud-environments#organization-shared-environments), soit les [environnements auto-hébergés](/docs/fr/self-hosted-environments).

Consultez [Configurer les environnements cloud](/docs/fr/cloud-environments) pour modifier ce qu'un environnement permet, définir des variables ou ajouter un script de configuration, et [Outils installés](/docs/fr/cloud-environments#installed-tools) pour ce que les sessions incluent sans aucune configuration.

<h2 id="github-authentication-options">
  Options d'authentification GitHub
</h2>

Les sessions cloud ont besoin d'accès à vos référentiels GitHub pour cloner le code et pousser les branches. Vous pouvez accorder l'accès de deux façons :

| Méthode                | Comment vous vous connectez                                                                             | Référentiels que les sessions peuvent atteindre                                                                      | Idéal pour                                                                |
| :--------------------- | :------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------ |
| **Application GitHub** | Autorisez l'application Claude GitHub lors de [l'intégration web](/docs/fr/web-quickstart)                   | N'importe quel référentiel public, et les référentiels privés sur lesquels l'application Claude GitHub est installée | Intégration web ; équipes qui veulent [Auto-fix](#auto-fix-pull-requests) |
| **`/web-setup`**       | Exécutez `/web-setup` dans votre terminal pour envoyer votre jeton CLI `gh` local à votre compte Claude | N'importe quel référentiel auquel votre jeton `gh` peut accéder, que l'application soit installée ou non             | Développeurs individuels qui utilisent déjà `gh`                          |

L'installation de l'application Claude GitHub sur un référentiel active également [Auto-fix](#auto-fix-pull-requests) pour les demandes de tirage qu'il contient.

Les threads dans un [projet](/docs/fr/claude-projects) ont besoin que l'application soit installée sur chaque référentiel qu'ils clonent, quelle que soit la méthode de connexion utilisée. Consultez [Configurer l'accès GitHub](/docs/fr/claude-projects#set-up-github-access).

Pour savoir comment `/schedule` vérifie l'accès au référentiel avant de créer une routine, consultez [Référentiels et permissions de branche](/docs/fr/routines#repositories-and-branch-permissions). Consultez [Connecter depuis votre terminal](/docs/fr/web-quickstart#connect-from-your-terminal) pour la procédure pas à pas de `/web-setup`, y compris ce que `/web-setup` stocke et comment le supprimer.

La configuration web rapide est un paramètre d'organisation qui permet aux membres de connecter GitHub avec `/web-setup`, ignore l'invite d'installation de l'application Claude GitHub lors de l'intégration web, et fait que l'intégration web crée l'[environnement **Default**](/docs/fr/cloud-environments#the-default-environment) pour eux au lieu d'afficher le formulaire d'environnement. Sur les plans Team et Enterprise, elle est désactivée par défaut, ce qui masque `/web-setup`. Un [Propriétaire](/docs/fr/server-managed-settings#access-control) l'active avec le bouton bascule **Quick web setup** à [**Paramètres d'administration > Claude Code**](https://claude.ai/admin-settings/claude-code).

<Note>
  Les organisations avec [Zéro rétention de données](/docs/fr/zero-data-retention) activée ne peuvent pas utiliser `/web-setup` ou d'autres fonctionnalités de session cloud.
</Note>

<h2 id="move-tasks-between-terminal-and-cloud">
  Déplacer les tâches entre le terminal et le cloud
</h2>

Ces workflows nécessitent que le [Claude Code CLI](/docs/fr/quickstart) soit connecté au même compte claude.ai. Vous pouvez démarrer de nouvelles sessions cloud depuis votre terminal, ou extraire des sessions cloud dans votre terminal pour continuer localement. Les sessions cloud persistent même si vous fermez votre ordinateur portable, et vous pouvez les surveiller de n'importe où, y compris depuis l'application mobile Claude.

<Note>
  Depuis le CLI, le transfert de session est unidirectionnel : vous pouvez extraire des sessions cloud dans votre terminal avec `--teleport`, mais vous ne pouvez pas envoyer une session terminal existante vers le cloud. Le flag `--cloud` avec une description de tâche crée une nouvelle session cloud pour votre référentiel actuel ; avec `-p` et un ID de session ou une URL claude.ai/code, il [met plutôt en file d'attente un message dans cette session existante](/docs/fr/claude-code-on-the-web#send-follow-ups-from-the-cli). L'[application de bureau](/docs/fr/desktop#continue-in-another-surface) fournit un menu **Continuer dans** qui peut envoyer une session locale vers le cloud.
</Note>

<h3 id="from-terminal-to-cloud">
  Du terminal vers le cloud
</h3>

Démarrez une session cloud depuis la ligne de commande avec le flag `--cloud` :

```bash theme={null}
claude --cloud "Fix the authentication bug in src/auth/login.ts"
```

Cela crée une nouvelle session cloud sur claude.ai. La VM cloud clone le remote GitHub du répertoire actuel à votre branche actuelle, pas votre checkout local, donc poussez d'abord si vous avez des commits locaux. Consultez [Envoyer des référentiels locaux sans GitHub](#send-local-repositories-without-github) pour les cas où Claude Code télécharge votre référentiel local au lieu de le cloner.

`--cloud` fonctionne avec un seul référentiel à la fois. La tâche s'exécute dans le cloud tandis que vous continuez à travailler localement. L'ancienne orthographe `--remote` fonctionne toujours comme alias déprécié pour `--cloud`.

Pendant que le conteneur cloud démarre, le CLI affiche une liste de contrôle en direct des étapes de configuration, telles que le clonage du référentiel et l'exécution de votre [script de configuration](/docs/fr/cloud-environments#setup-scripts). Il met en file d'attente les messages que vous tapez pendant le provisionnement et les envoie une fois que la session est prête.

<Note>
  `--cloud` crée des sessions cloud. `--remote-control` n'est pas lié : il vous permet de surveiller et de diriger une session CLI locale depuis claude.ai ou l'application Claude. Voir [Remote Control](/docs/fr/remote-control).
</Note>

Ouvrez la session sur claude.ai ou l'application mobile Claude pour vérifier la progression ou interagir directement. De là, vous pouvez diriger Claude, fournir des commentaires ou répondre à des questions comme dans n'importe quelle autre conversation.

Si Claude pose une question et que la session reste inactive, vous pouvez toujours répondre quand vous revenez, jusqu'à [l'expiration de l'environnement](#environment-expired), et la session continue à partir de votre réponse.

<h4 id="tips-for-cloud-tasks">
  Conseils pour les tâches cloud
</h4>

**Planifiez localement, exécutez dans le cloud** : pour les tâches complexes, démarrez Claude en mode plan pour collaborer sur l'approche, puis envoyez le travail vers le cloud :

```bash theme={null}
claude --permission-mode plan
```

En mode plan, Claude lit les fichiers, exécute des commandes pour explorer et propose un plan sans modifier le code source. Une fois que vous êtes satisfait, enregistrez le plan dans le référentiel, validez et poussez afin que la VM cloud puisse le cloner. Ensuite, démarrez une session cloud pour l'exécution autonome :

```bash theme={null}
claude --cloud "Execute the migration plan in docs/migration-plan.md"
```

**Exécutez les tâches en parallèle** : chaque commande `--cloud` crée sa propre session cloud qui s'exécute indépendamment. Vous pouvez démarrer plusieurs tâches et elles s'exécuteront toutes simultanément dans des sessions séparées :

```bash theme={null}
claude --cloud "Fix the flaky test in auth.spec.ts"
claude --cloud "Update the API documentation"
claude --cloud "Refactor the logger to use structured output"
```

Quand une session se termine, vous pouvez créer une PR depuis claude.ai/code ou [téléporter](#from-cloud-to-terminal) la session dans votre terminal pour continuer à travailler.

<h4 id="send-local-repositories-without-github">
  Envoyer des référentiels locaux sans GitHub
</h4>

Quand vous exécutez `claude --cloud` depuis un référentiel qui n'a pas de remote git, ou depuis un référentiel github.com sur lequel l'application Claude GitHub n'est pas installée, Claude Code regroupe votre référentiel local et le télécharge directement vers la session cloud. Cela s'applique même si vous avez connecté GitHub avec `/web-setup`. Le bundle inclut l'historique complet de votre référentiel sur toutes les branches, plus les modifications non validées des fichiers suivis.

Sur macOS, Linux et WSL, Claude Code exclut les modifications non validées des fichiers nommés comme des identifiants ou des clés de l'upload et nomme les fichiers qu'il a laissés de côté. Cela couvre les fichiers `.env`, les fichiers Terraform `*.tfvars` et les fichiers clés tels que `id_rsa` et `*.pem`. La session démarre avec la version validée de chacun, ou sans le fichier si aucun n'est validé. Dans une worktree liée, un submodule ou une disposition similaire, Claude Code télécharge ces modifications avec le reste et nomme les fichiers qu'il télécharge.

Pour télécharger un bundle même quand Claude Code clonerait autrement depuis le remote, définissez `CCR_FORCE_BUNDLE=1` :

```bash theme={null}
CCR_FORCE_BUNDLE=1 claude --cloud "Run the test suite and fix any failures"
```

Les référentiels regroupés doivent respecter ces limites :

* Le répertoire doit être un référentiel git avec au moins un commit
* Le référentiel regroupé doit être inférieur à 100 MB. Les référentiels plus grands reviennent à regrouper uniquement la branche actuelle, puis à un seul snapshot aplati de l'arborescence de travail, et échouent uniquement si le snapshot est toujours trop volumineux
* Les fichiers non suivis ne sont pas inclus ; exécutez `git add` sur les fichiers que vous voulez que la session cloud voie
* Les sessions créées à partir d'un bundle ne peuvent repousser vers un remote GitHub que si votre [connexion GitHub](#github-authentication-options) a accès en push à ce référentiel

<h3 id="send-follow-ups-from-the-cli">
  Envoyer des messages de suivi depuis le CLI
</h3>

Une fois qu'une session cloud est en cours d'exécution, où qu'elle s'exécute, envoyez-lui un message de suivi depuis le CLI `claude` sur n'importe quelle machine où vous êtes connecté avec `claude auth login`. Le CLI s'authentifie avec vos identifiants de compte Anthropic et n'envoie aucun état de session local, donc la commande n'a pas besoin de s'exécuter depuis la machine qui a démarré la session, et c'est la même dans chaque shell, y compris PowerShell.

La commande publie un message et se termine :

```bash theme={null}
claude -p "your message" --cloud <session-id>
```

Le CLI met le message en file d'attente dans la session et se termine sans attendre une réponse. Utilisez-le pour diriger une session longue, mettre en file d'attente l'étape suivante tandis que la session actuelle se termine toujours, ou envoyer des messages de suivi depuis un [script CI](/docs/fr/self-hosted-environments-testing#run-the-test-loop). Vous pouvez également rediriger le message sur stdin au lieu de le passer en tant qu'argument : `echo "your message" | claude -p --cloud <session-id>`.

Pour `<session-id>`, passez l'ID nu, tel que `session_...` ou `cse_...`, ou l'URL `claude.ai/code/<id>` de la session, avec ou sans le schéma ou la chaîne de requête. Trouvez l'ID dans votre liste de sessions sur claude.ai/code.

<Note>
  `--cloud` nécessite un compte Anthropic. Il n'est pas disponible quand Claude Code est configuré pour Amazon Bedrock, Google Cloud's Agent Platform ou un autre fournisseur tiers. Une [passerelle LLM](/docs/fr/llm-gateway) configurée uniquement via `ANTHROPIC_BASE_URL` ne compte pas comme un fournisseur tiers pour cette vérification, mais vous devez toujours vous connecter avec `claude auth login`. La politique `allow_remote_sessions` de votre organisation doit également être activée. Un propriétaire peut l'activer dans les paramètres d'administration Claude Code sur claude.ai/admin-settings/claude-code.
</Note>

<h4 id="output-and-errors">
  Sortie et erreurs
</h4>

En cas de succès, la commande imprime l'ID de session et un lien pour afficher la session :

```
Sent to cloud session.
Session ID: session_01DiUkqY2kzbUbDmW1w96rfi
View: https://claude.ai/code/session_01DiUkqY2kzbUbDmW1w96rfi?from=cli&m=0
```

Passez `--output-format json` pour un résultat lisible par machine : `{ok, session_id, url}` en cas de succès, ou `{ok: false, session_id, error}` quand l'envoi échoue, par exemple quand la session est manquante ou archivée. Les erreurs de configuration, telles qu'un fournisseur non pris en charge ou une politique organisationnelle désactivée, s'impriment sur stderr sans JSON. `--output-format stream-json` n'est pas pris en charge avec `--cloud <session-id>`.

Le CLI préfixe les erreurs avec `Error: `. Un échec de livraison est enveloppé comme `failed to send message to cloud session <id>: <reason>`.

| Message                                                                                                                     | Ce que cela signifie                                                                                                                                                                                                                                                                                                                                           |
| --------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Cloud sessions aren't available with <provider>. They run on Anthropic's infrastructure and require an Anthropic account.` | Claude Code est configuré pour un fournisseur tiers. Le message nomme le fournisseur avec l'étiquette que votre configuration utilise, telle que `Amazon Bedrock` ou `Google Vertex AI`. Supprimez la configuration de ce fournisseur, par exemple en désactivant `CLAUDE_CODE_USE_BEDROCK`, et connectez-vous avec un compte Anthropic (`claude auth login`). |
| `Cloud sessions are disabled by your organization's policy. Contact your organization admin to enable them.`                | La politique organisationnelle `allow_remote_sessions` est désactivée.                                                                                                                                                                                                                                                                                         |
| `Couldn't verify your organization's policy for cloud sessions. Check your network connection and try again.`               | Claude Code n'a pas pu récupérer la politique de votre organisation, il refuse donc l'envoi plutôt que d'assumer que les sessions cloud sont autorisées. Vérifiez votre connexion réseau et réessayez.                                                                                                                                                         |
| `Attaching to an existing cloud session is not enabled for your account.`                                                   | Vous avez exécuté `--cloud <session-id>` sans `-p`. Envoyez le message avec `claude -p "your message" --cloud <session-id>`.                                                                                                                                                                                                                                   |
| `Session not found: <id>`                                                                                                   | L'ID ou l'URL ne correspond pas à une session à laquelle vous pouvez accéder. Vérifiez-le par rapport à l'URL claude.ai/code de la session.                                                                                                                                                                                                                    |
| `cloud session <id> is archived and cannot accept new messages`                                                             | La session a été archivée. Démarrez une nouvelle session à la place.                                                                                                                                                                                                                                                                                           |

<h3 id="from-cloud-to-terminal">
  Du cloud vers le terminal
</h3>

Extrayez une session cloud dans votre terminal en utilisant l'une de ces options :

* **Utilisation de `--teleport`** : depuis la ligne de commande, exécutez `claude --teleport` pour un sélecteur de session interactif, ou `claude --teleport <session-id>` pour reprendre une session spécifique directement. Si vous avez des modifications non validées, vous serez invité à les ranger d'abord.
* **Utilisation de `/teleport`** : à l'intérieur d'une session CLI existante, exécutez `/teleport` ou `/tp` pour ouvrir le même sélecteur de session sans redémarrer Claude Code.
* **Depuis `/tasks`** : exécutez `/tasks` pour voir vos sessions en arrière-plan, puis appuyez sur `t` pour vous téléporter dans l'une d'elles.
* **Depuis claude.ai/code** : sélectionnez **Ouvrir dans > Terminal** dans le menu de session pour copier une commande que vous pouvez coller dans votre terminal.
* **Depuis l'intérieur de la session cloud** : tapez `/teleport` et Claude Code répond avec la commande exacte `claude --teleport <session-id>` pour cette session, prête à s'exécuter à partir d'un checkout du référentiel. Nécessite Claude Code v2.1.223 ou ultérieur dans l'environnement de la session.

Quand vous téléportez une session, Claude vérifie que vous êtes dans le bon référentiel, récupère et vérifie la branche de la session cloud, et charge l'historique complet de la conversation dans votre terminal. Le terminal obtient sa propre copie de la session : le nouveau travail là-bas reste local et n'apparaît pas dans la session cloud sur claude.ai ou l'application mobile Claude. Pour continuer à diriger depuis votre téléphone après la téléportation, démarrez [`/remote-control`](/docs/fr/remote-control) dans la session locale.

`--teleport` est distinct de `--resume`. `--resume` rouvre une conversation à partir de l'historique local de cette machine et ne liste pas les sessions cloud ; `--teleport` extrait une session cloud et sa branche.

<h4 id="teleport-requirements">
  Exigences de téléportation
</h4>

Teleport vérifie ces exigences avant de reprendre une session. Si une exigence n'est pas satisfaite, vous verrez une erreur ou serez invité à résoudre le problème.

| Exigence            | Détails                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| État git propre     | Votre répertoire de travail ne doit avoir aucune modification non validée. Teleport vous invite à ranger les modifications si nécessaire.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| Référentiel correct | Vous devez exécuter `--teleport` à partir d'un checkout du même référentiel, pas d'une fork. Si vous l'exécutez à partir d'un checkout d'un référentiel différent, Claude Code affiche une erreur qui nomme à la fois le référentiel de la session et le référentiel de votre checkout. Avant la v2.1.219, l'erreur ne nommait pas le référentiel de votre checkout. Si Claude Code ne peut pas analyser votre remote en un nom d'hôte, par exemple un alias d'hôte SSH comme `git@work:owner/repo.git`, il vous demande de confirmer et accepte le checkout quand le propriétaire du remote et le nom du référentiel correspondent au référentiel de la session. |
| Branche disponible  | La branche de la session cloud doit avoir été poussée vers le remote. Teleport la récupère et la vérifie automatiquement.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| Même compte         | Vous devez être authentifié au même compte claude.ai utilisé dans la session cloud.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |

<h4 id="teleport-is-unavailable">
  `--teleport` n'est pas disponible
</h4>

Teleport nécessite l'authentification par abonnement claude.ai. Si vous êtes authentifié via clé API, exécutez `/login` pour vous connecter avec votre compte claude.ai à la place. Si l'erreur nomme votre fournisseur à la place, les sessions cloud ne sont pas disponibles via les fournisseurs tiers ; voir le [tableau d'erreurs](#output-and-errors). Si vous êtes déjà connecté via claude.ai et que `--teleport` n'est toujours pas disponible, votre organisation a peut-être désactivé les sessions cloud.

<h2 id="work-with-sessions">
  Travailler avec les sessions
</h2>

Les sessions apparaissent dans la barre latérale sur claude.ai/code. De là, vous pouvez examiner les modifications, partager avec vos coéquipiers, archiver le travail terminé ou supprimer définitivement les sessions.

<h3 id="take-back-a-queued-message">
  Annuler un message en attente
</h3>

Si vous envoyez un message alors que Claude travaille, le message est mis en attente jusqu'à ce que Claude le lise. Pour annuler un message en attente, cliquez sur le ✕ qui s'y trouve. Le texte revient à la boîte de message pour que vous puissiez le modifier ou envoyer quelque chose d'autre.

Si Claude a déjà lu le message, il reste dans la conversation.

<h3 id="manage-context">
  Gérer le contexte
</h3>

Les sessions cloud prennent en charge les [commandes intégrées](/docs/fr/commands) qui produisent une sortie textuelle. Les commandes qui s'exécutent uniquement dans l'interface du terminal, telles que `/plugin` ou `/resume`, ne sont pas disponibles. Les commandes qui ouvrent un sélecteur ou un panneau dans le terminal se comportent différemment dans les sessions cloud :

* **`/model`, `/effort`, `/color` et `/rename`** : transmettez la valeur en tant qu'argument, par exemple `/model sonnet`, au lieu d'ouvrir le sélecteur du terminal ou le curseur. Les formes d'argument nécessitent Claude Code v2.1.205 ou une version ultérieure dans l'environnement de la session et suivent les [notes de disponibilité](/docs/fr/commands#all-commands) de chaque commande.
* **`/fast`** : bascule le [mode rapide](/docs/fr/fast-mode#use-fast-mode-in-cloud-sessions) pour la session lorsque le mode rapide est [disponible sur votre compte](/docs/fr/fast-mode#requirements). Nécessite Claude Code v2.1.271 ou une version ultérieure dans l'environnement de la session.
* **`/config`** : dans votre navigateur sur claude.ai/code, ouvre la section Claude Code de vos paramètres au lieu de définir une valeur, et le texte après la commande, y compris `key=value`, est ignoré. Pour modifier un paramètre pour une session cloud, définissez une [variable d'environnement](/docs/fr/cloud-environments#set-environment-variables) sur l'environnement, ou dans une session avec un référentiel, validez la clé dans le fichier `.claude/settings.json` de ce référentiel. [Les paramètres dans les sessions cloud](/docs/fr/settings#settings-in-cloud-sessions) énumère ce que chaque session lit.

Pour la gestion du contexte spécifiquement :

| Commande   | Fonctionne dans les sessions cloud | Notes                                                                                                                                 |
| :--------- | :--------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------ |
| `/compact` | Oui                                | Résume la conversation pour libérer du contexte. Accepte les instructions de focus optionnelles comme `/compact keep the test output` |
| `/context` | Oui                                | Affiche ce qui se trouve actuellement dans la fenêtre de contexte                                                                     |
| `/clear`   | Non                                | Démarrez plutôt une nouvelle session à partir de la barre latérale                                                                    |

La compaction automatique s'exécute automatiquement lorsque la fenêtre de contexte approche de sa capacité. Les sessions cloud définissent [`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`](/docs/fr/env-vars) elles-mêmes, de sorte que la compaction se déclenche à mi-chemin dans la [fenêtre de compaction automatique](/docs/fr/model-config#set-the-auto-compact-window) plutôt que lorsque la fenêtre se remplit. Cette valeur remplace celle que vous ajoutez dans vos [variables d'environnement](/docs/fr/cloud-environments#set-environment-variables), donc ajouter la variable là-bas ne change pas le moment où la compaction se déclenche.

Pour modifier la fenêtre de compaction automatique à la place, définissez [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/fr/env-vars) dans vos variables d'environnement, ou exécutez [`/autocompact`](/docs/fr/commands#all-commands) avec un nombre de jetons dans une session où la variable n'est pas définie.

Les [sous-agents](/docs/fr/sub-agents) fonctionnent de la même manière qu'en local. Claude peut les générer avec l'outil Agent pour déléguer la recherche ou le travail parallèle à une fenêtre de contexte séparée, en gardant la conversation principale plus légère. Les sous-agents définis dans le répertoire `.claude/agents/` de votre référentiel sont détectés automatiquement.

Les [équipes d'agents](/docs/fr/agent-teams) sont désactivées par défaut mais peuvent être activées en ajoutant `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` à vos [variables d'environnement](/docs/fr/cloud-environments#set-environment-variables).

<h3 id="permission-modes-in-cloud-sessions">
  Modes de permission dans les sessions cloud
</h3>

Vous choisissez le [mode de permission](/docs/fr/permission-modes) d'une session cloud à partir du [menu déroulant de mode](/docs/fr/permission-modes#switch-permission-modes), à la fois lorsque vous créez la tâche et pendant que la session s'exécute. Lorsque vous rouvrez une session dont l'[environnement hébergé par Anthropic a expiré](#environment-expired), ou que vous envoyez un message à une session qu'un exécuteur auto-hébergé a [libérée pendant qu'elle était inactive](/docs/fr/self-hosted-environments-reference#runner-cli-flags), Claude Code reprend la session dans le mode de permission dans lequel elle se trouvait.

<h3 id="review-changes">
  Examiner les modifications
</h3>

Chaque session affiche un indicateur de diff avec les lignes ajoutées et supprimées, comme `+42 -18`. Sélectionnez-le pour ouvrir la vue diff, laisser des commentaires en ligne sur des lignes spécifiques et les envoyer à Claude avec votre prochain message.

La vue diff compare les modifications de la session par rapport à sa branche de base par défaut. Pour comparer avec n'importe quelle autre branche du référentiel, sélectionnez **Comparer avec** et choisissez-en une.

Claude Code calcule ces diffs, y compris les diffs par fichier affichés lors des modifications de Claude, à partir du contenu brut des blobs git, de sorte que les pilotes diff et les filtres `textconv` configurés dans le référentiel ne s'appliquent pas. Pour un fichier dans un référentiel qui n'est pas l'un des checkouts de la session, comme un cloné à l'intérieur de l'espace de travail pendant la session, le diff par fichier affiche la modification de Claude elle-même plutôt qu'une comparaison git.

Consultez [Examiner et itérer](/docs/fr/web-quickstart#review-and-iterate) pour la procédure complète incluant la création de PR. Pour que Claude surveille automatiquement la PR pour les défaillances CI et les commentaires d'examen, consultez [Correction automatique des demandes de tirage](#auto-fix-pull-requests).

<h3 id="share-sessions">
  Partager les sessions
</h3>

Pour partager une session, basculez sa visibilité selon les types de compte ci-dessous. Après cela, partagez le lien de session tel quel. Les destinataires voient l'état le plus récent lorsqu'ils ouvrent le lien, mais leur vue ne se met pas à jour en temps réel.

<h4 id="share-from-an-enterprise-or-team-account">
  Partager à partir d'un compte Enterprise ou Team
</h4>

Pour les comptes Enterprise et Team, les deux options de visibilité sont **Privé** et **Équipe**. La visibilité Équipe rend la session visible aux autres membres de votre organisation claude.ai. Les sessions [Claude dans Slack](/docs/fr/slack) sont automatiquement partagées avec la visibilité Équipe.

La vérification de l'accès au référentiel est activée par défaut, en fonction du compte GitHub connecté au compte du destinataire. Le nom d'affichage de votre compte est visible à tous les destinataires ayant accès.

<h4 id="share-from-a-max-or-pro-account">
  Partager à partir d'un compte Max ou Pro
</h4>

Pour les comptes Max et Pro, les deux options de visibilité sont **Privé** et **Public**. La visibilité Public rend la session visible à tout utilisateur connecté à claude.ai.

Vérifiez votre session pour le contenu sensible avant de partager. Les sessions peuvent contenir du code et des identifiants provenant de référentiels GitHub privés. La vérification de l'accès au référentiel n'est pas activée par défaut.

Pour exiger que les destinataires aient accès au référentiel, ou pour masquer votre nom des sessions partagées, accédez à [**Paramètres > Claude Code > Paramètres de partage**](https://claude.ai/settings/claude-code).

<h3 id="archive-sessions">
  Archiver les sessions
</h3>

Vous pouvez archiver les sessions pour garder votre liste de sessions organisée. Les sessions archivées sont masquées de la liste de sessions par défaut mais peuvent être visualisées en filtrant les sessions archivées.

Pour archiver une session, survolez la session dans la barre latérale et sélectionnez l'icône d'archive.

<h3 id="delete-sessions">
  Supprimer les sessions
</h3>

La suppression d'une session supprime définitivement la session et ses données. Cette action ne peut pas être annulée. Vous pouvez supprimer une session de deux façons :

* **À partir de la barre latérale** : filtrez les sessions archivées, puis survolez la session que vous souhaitez supprimer et sélectionnez l'icône de suppression
* **À partir du menu de session** : ouvrez une session, sélectionnez la liste déroulante à côté du titre de la session, et sélectionnez **Supprimer**

Vous serez invité à confirmer avant la suppression d'une session.

<h2 id="auto-fix-pull-requests">
  Correction automatique des demandes de tirage
</h2>

Claude peut surveiller une demande de tirage et répondre automatiquement aux défaillances CI et aux commentaires d'examen. Claude s'abonne aux événements GitHub sur la PR, et lorsqu'une vérification échoue ou qu'un examinateur laisse un commentaire, Claude enquête et pousse une correction si elle est claire.

<Note>
  Auto-fix nécessite que l'application Claude GitHub soit installée sur votre référentiel. Si vous ne l'avez pas déjà fait, installez-la à partir de la [page de l'application GitHub](https://github.com/apps/claude).
</Note>

Il existe plusieurs façons d'activer auto-fix selon d'où provient la PR et quel appareil vous utilisez :

* **PR créées dans une session cloud** : ouvrez la session sur claude.ai/code, ouvrez la barre d'état CI et sélectionnez **Auto-fix**
* **À partir de votre terminal** : exécutez [`/autofix-pr`](/docs/fr/commands) sur la branche de la PR. Claude Code détecte la PR ouverte avec `gh`, génère une session cloud et active auto-fix en une seule étape
* **À partir de l'application mobile** : dites à Claude de corriger automatiquement la PR, par exemple « regardez cette PR et corrigez les défaillances CI ou les commentaires d'examen »
* **N'importe quelle PR existante** : collez l'URL de la PR dans une session et dites à Claude de la corriger automatiquement

Auto-fix est un bouton bascule par PR. Pour arrêter la surveillance, ouvrez la barre d'état CI dans la session sur claude.ai/code et désactivez le bouton bascule **Auto-fix**, ou dites à Claude d'arrêter de surveiller la PR.

<h3 id="how-claude-responds-to-pr-activity">
  Comment Claude répond à l'activité PR
</h3>

Lorsque auto-fix est actif, Claude reçoit les événements GitHub pour la PR, y compris les nouveaux commentaires d'examen et les défaillances de vérification CI. Pour chaque événement, Claude enquête et décide comment procéder :

* **Corrections claires** : si Claude est confiant dans une correction et qu'elle n'entre pas en conflit avec les instructions antérieures, Claude apporte la modification, la pousse et explique ce qui a été fait dans la session
* **Demandes ambiguës** : si le commentaire d'un examinateur peut être interprété de plusieurs façons ou implique quelque chose d'architecturalement significatif, Claude vous demande avant d'agir
* **Événements en double ou sans action** : si un événement est un doublon ou ne nécessite aucune modification, Claude le note dans la session et continue

GitHub n'émet pas de webhook lorsque la branche de base avance et crée un conflit de fusion, donc auto-fix ne peut pas réagir aux conflits de son propre chef. Pour résoudre un conflit, ouvrez la session et demandez à Claude de rebaser.

Claude peut répondre aux fils de commentaires d'examen sur GitHub dans le cadre de leur résolution. Ces réponses sont publiées en utilisant votre compte GitHub, elles apparaissent donc sous votre nom d'utilisateur, mais chaque réponse est étiquetée comme provenant de Claude Code pour que les examinateurs sachent qu'elle a été écrite par l'agent et non par vous directement.

<Warning>
  Si votre référentiel utilise une automatisation déclenchée par commentaire comme Atlantis, Terraform Cloud ou des GitHub Actions personnalisées qui s'exécutent sur les événements `issue_comment`, sachez que Claude peut répondre en votre nom, ce qui peut déclencher ces flux de travail. Examinez l'automatisation de votre référentiel avant d'activer auto-fix et envisagez de désactiver auto-fix pour les référentiels où un commentaire PR peut déployer une infrastructure ou exécuter des opérations privilégiées.
</Warning>

<h2 id="security-and-isolation">
  Sécurité et isolation
</h2>

Chaque session cloud est séparée de votre machine et des autres sessions par plusieurs couches :

* **Machines virtuelles isolées** : chaque session s'exécute dans une VM isolée gérée par Anthropic. Les sessions que votre organisation achemine vers un [environnement auto-hébergé](/docs/fr/self-hosted-environments) s'exécutent sur votre propre infrastructure à la place, où l'isolation est la responsabilité de votre déploiement
* <span id="default-allowed-domains" />**Contrôles d'accès réseau** : dans les environnements hébergés par Anthropic, l'accès réseau est limité par défaut et peut être désactivé. Consultez [Accès réseau](/docs/fr/cloud-environments#network-access) pour les niveaux d'accès, les [domaines autorisés par défaut](/docs/fr/cloud-environments#default-allowed-domains), et le trafic qui ne passe pas par la liste d'autorisation. Dans un environnement auto-hébergé, vous restreignez la sortie de session à votre propre limite réseau. Lors de l'exécution avec l'accès réseau désactivé, Claude Code peut toujours communiquer avec l'API Anthropic, ce qui peut permettre aux données de quitter la VM.
* **Protection des identifiants** : dans les environnements hébergés par Anthropic, les identifiants git et les clés de signature restent en dehors du sandbox, et un proxy authentifie au nom de la session avec des identifiants limités. Dans un environnement auto-hébergé, votre déploiement fournit les identifiants git ; consultez [Configurer git](/docs/fr/self-hosted-environments-deploy#configure-git)
* **Identifiants API** : dans les environnements hébergés par Anthropic sur les plans Pro et Max, les clés que vous [ajoutez à un environnement cloud](/docs/fr/cloud-environments#add-api-credentials) restent en dehors du sandbox de la même manière, attachées aux demandes correspondantes après qu'elles quittent la session. Un environnement auto-hébergé n'a pas d'identifiants API, et les plans Team et Enterprise ne les ont pas encore
* **Analyse sécurisée** : le code est analysé et modifié dans l'environnement isolé de la session avant la création de PR

<h2 id="troubleshooting">
  Dépannage
</h2>

Pour les erreurs d'API d'exécution qui apparaissent dans la conversation comme `API Error: 500`, `529 Overloaded`, `429` ou `Prompt is too long`, consultez la [référence des erreurs](/docs/fr/errors). Ces erreurs et leurs corrections sont partagées avec le CLI et l'application Desktop. Les sections ci-dessous couvrent les problèmes spécifiques aux sessions cloud.

<h3 id="session-creation-failed">
  Échec de la création de session
</h3>

Si une nouvelle session ne démarre pas avec `Session creation failed` ou stagne à la mise en service, Claude Code n'a pas pu allouer une VM pour la session.

* Vérifiez [status.claude.com](https://status.claude.com) pour les incidents de session cloud
* Réessayez après une minute, car la capacité est mise en service à la demande
* Confirmez que votre connexion GitHub peut atteindre le référentiel en suivant [Aucun référentiel n'apparaît après la connexion à GitHub](/docs/fr/web-quickstart#no-repositories-appear-after-connecting-github)

<h3 id="unable-to-get-organization-uuid">
  Impossible d'obtenir l'UUID de l'organisation
</h3>

`claude --cloud` et `claude --teleport` nécessitent une connexion avec un compte claude.ai. Si vous vous authentifiez avec une clé API, ou si vos détails de compte stockés sont obsolètes, ces commandes échouent avec `Unable to get organization UUID` ou un message indiquant que l'authentification par clé API n'est pas suffisante. Avec l'authentification par clé API ou les détails de compte obsolètes, l'exécution de `claude --teleport` sans ID de session affiche `Error loading Claude Code sessions` dans le sélecteur de session au lieu de l'un ou l'autre message, et le même correctif s'applique.

Exécutez `/login` pour vous connecter avec votre compte claude.ai, puis réessayez la commande. Si le message nomme votre fournisseur à la place, consultez le [tableau d'erreurs](#output-and-errors) : les sessions cloud ne sont pas disponibles via les fournisseurs tiers.

<h3 id="remote-control-session-expired-or-access-denied">
  Session Remote Control expirée ou accès refusé
</h3>

`--teleport` se connecte via la même infrastructure de session Remote Control que les sessions cloud, donc les erreurs d'authentification et d'expiration de session apparaissent avec la terminologie Remote Control. Vous pouvez voir `Remote Control session expired` ou `Access denied`. Le jeton de connexion est de courte durée et limité à votre compte.

* Exécutez `/login` localement pour actualiser vos identifiants, puis reconnectez-vous
* Confirmez que vous êtes connecté au même compte qui possède la session
* Si vous voyez `Remote Control may not be available for this organization`, un Propriétaire n'a pas activé les sessions cloud pour votre organisation

<h3 id="environment-expired">
  Environnement expiré
</h3>

Les sessions cloud s'arrêtent après une période d'inactivité et la VM de la session est réclamée. Une session est considérée comme inactive pendant qu'elle attend votre approbation d'un appel d'outil [connecteur MCP](/docs/fr/cloud-environments#network-access) ou votre connexion à un serveur MCP, et elle peut expirer pendant cette attente.

Rouvrez la session à partir de [claude.ai/code](https://claude.ai/code) pour mettre en service une VM fraîche avec votre historique de conversation restauré. Le travail en arrière-plan qui était toujours en cours d'exécution lorsque la VM a été réclamée, comme les sous-agents et les commandes shell, n'est pas restauré.

<h2 id="limitations">
  Limitations
</h2>

Avant de compter sur les sessions cloud pour un flux de travail, tenez compte de ces contraintes :

* **Limites de débit** : les sessions cloud partagent les limites de débit avec tous les autres usages de Claude et Claude Code au sein de votre compte. L'exécution de plusieurs tâches en parallèle consomme proportionnellement plus de limites de débit. Il n'y a pas de frais de calcul séparé pour la VM cloud.
* **Authentification du référentiel** : vous ne pouvez déplacer une session cloud vers votre terminal que lorsque vous êtes authentifié au même compte
* **Restrictions de plateforme** : le clonage du référentiel et la création de demandes de tirage nécessitent GitHub. Les instances [GitHub Enterprise Server](/docs/fr/github-enterprise-server) auto-hébergées sont prises en charge pour les plans Team et Enterprise. Vous pouvez envoyer un référentiel GitLab, Bitbucket ou autre référentiel non-GitHub à une session cloud en tant que [paquet local](#send-local-repositories-without-github) en définissant `CCR_FORCE_BUNDLE=1`, mais la session ne peut pas pousser les résultats vers ce serveur distant
* **Liste d'autorisation IP de l'organisation** : les sessions cloud appellent l'API Anthropic à partir de l'infrastructure gérée par Anthropic, pas de votre réseau, tandis que les sessions dans un [environnement auto-hébergé](/docs/fr/self-hosted-environments) l'appellent à partir de votre propre réseau. Si votre organisation a [l'autorisation IP](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting) activée, chaque session cloud hébergée par Anthropic échoue avec une erreur d'authentification. Il en va de même pour [Code Review](/docs/fr/code-review) et pour les [routines](/docs/fr/routines) qui s'exécutent sur les environnements hébergés par Anthropic ; une routine acheminée vers un environnement auto-hébergé appelle l'API à partir de votre propre réseau. Contactez [le support Anthropic](https://support.claude.com/) pour exempter les services hébergés par Anthropic de la liste d'autorisation IP de votre organisation.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Environnements cloud](/docs/fr/cloud-environments) : configurez l'accès réseau, les variables d'environnement et les scripts de configuration pour les sessions cloud
* [Projects](/docs/fr/claude-projects) : une conversation où Claude coordonne les sessions cloud parallèles sur vos référentiels et vous rend compte
* [Ultrareview](/docs/fr/ultrareview) : exécutez un examen de code multi-agent approfondi dans un sandbox cloud
* [Routines](/docs/fr/routines) : automatisez le travail selon un calendrier, via un appel API ou en réponse aux événements GitHub
* [Configuration des hooks](/docs/fr/hooks) : exécutez les scripts aux événements du cycle de vie de la session
* [Tous les paramètres](/docs/fr/settings-reference) : toutes les options de configuration
* [Sécurité](/docs/fr/security) : garanties d'isolation et gestion des données
* [Utilisation des données](/docs/fr/data-usage) : ce qu'Anthropic conserve des sessions cloud
* [Claude Tag](https://claude.com/docs/claude-tag/overview) : un @Claude géré par l'organisation dans Slack qui s'exécute sur la même infrastructure cloud
