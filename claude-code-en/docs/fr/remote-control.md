> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Continuer les sessions locales depuis n'importe quel appareil avec Remote Control

> Continuez une session Claude Code locale depuis votre téléphone, tablette ou n'importe quel navigateur en utilisant Remote Control. Fonctionne avec claude.ai/code et l'application Claude mobile.

<Note>
  Remote Control est disponible sur tous les plans. Sur Team et Enterprise, il est désactivé par défaut jusqu'à ce qu'un propriétaire active le bouton Remote Control dans les [paramètres d'administration Claude Code](https://claude.ai/admin-settings/claude-code).
</Note>

Remote Control connecte [claude.ai/code](https://claude.ai/code) ou l'application Claude pour [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) et [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) à une session Claude Code s'exécutant sur votre machine. Commencez une tâche à votre bureau, puis reprenez-la depuis votre téléphone sur le canapé ou un navigateur sur un autre ordinateur.

Lorsque vous démarrez une session Remote Control sur votre machine, Claude continue à s'exécuter localement à tout moment, donc votre exécution de code et votre accès au système de fichiers restent sur votre machine. Avec Remote Control, vous pouvez :

* **Utiliser votre environnement local complet à distance** : votre système de fichiers, [serveurs MCP](/docs/fr/mcp), outils et configuration de projet restent tous disponibles, et taper `@` complète automatiquement les chemins de fichiers de votre projet local.
* **Travailler depuis les deux surfaces à la fois** : la conversation et la progression des [sous-agents](/docs/fr/sub-agents) et des [flux de travail dynamiques](/docs/fr/workflows) restent synchronisés sur tous les appareils connectés, vous pouvez donc envoyer des messages depuis votre terminal, navigateur et téléphone de manière interchangeable.
* **Envoyer des images et des fichiers depuis votre téléphone ou navigateur** : joignez une photo ou un fichier dans l'application Claude ou sur claude.ai/code, avec ou sans légende. Claude voit les photos jointes directement comme faisant partie de votre message. Claude Code télécharge les autres fichiers sur votre machine et les transmet à Claude en tant que références de fichier `@`.
* **Survivre aux interruptions** : si votre ordinateur portable s'endort ou votre réseau tombe en panne, Claude Code se reconnecte automatiquement lorsque votre machine revient en ligne. Pendant que la connexion se rétablit, Claude Code met en file d'attente les messages, les invites de permission et les mises à jour de statut des sous-agents et des flux de travail, et les livre une fois que la connexion se rétablit.

Contrairement à [Claude Code sur le web](/docs/fr/claude-code-on-the-web), qui s'exécute sur l'infrastructure cloud, les sessions Remote Control s'exécutent directement sur votre machine et interagissent avec votre système de fichiers local. Les interfaces web et mobile ne sont qu'une fenêtre dans cette session locale.

Cette page couvre la configuration, comment démarrer et se connecter aux sessions, et comment Remote Control se compare à Claude Code sur le web.

<h2 id="requirements">
  Conditions requises
</h2>

Avant d'utiliser Remote Control, confirmez que votre environnement répond à ces conditions :

* **Abonnement** : disponible sur les plans Pro, Max, Team et Enterprise. Les clés API ne sont pas prises en charge. Sur Team et Enterprise, un propriétaire doit d'abord activer le bouton Remote Control dans les [paramètres d'administration Claude Code](https://claude.ai/admin-settings/claude-code).
* **Authentification** : exécutez `claude` et utilisez `/login` pour vous connecter via claude.ai si vous ne l'avez pas déjà fait. Sans une connexion éligible, `claude remote-control` se termine avec une erreur, tandis que `claude --remote-control` démarre toujours une session interactive et affiche une notification d'échec de Remote Control peu de temps après le lancement.
* **Point de terminaison API** : non disponible dans l'une de ces configurations :
  * Vous utilisez Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry.
  * Vous pointez [`ANTHROPIC_BASE_URL`](/docs/fr/env-vars) vers un hôte autre que `api.anthropic.com`, tel qu'une [passerelle LLM](/docs/fr/llm-gateway) ou un proxy. Déconfigurez la variable pour utiliser Remote Control. Avant la v2.1.196, Claude Code autorisait Remote Control avec une `ANTHROPIC_BASE_URL` personnalisée.
  * Vous vous connectez via une passerelle [Claude apps gateway](/docs/fr/claude-apps-gateway) d'entreprise.
* **Évaluation des drapeaux de fonctionnalité** : [`DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` et `DISABLE_GROWTHBOOK`](/docs/fr/env-vars) désactivent chacun l'évaluation des drapeaux de fonctionnalité dont dépend la disponibilité de Remote Control. Déconfigurez la variable partout où elle est définie, dans votre environnement shell ou dans le bloc `env` d'un [fichier `settings.json`](/docs/fr/settings-reference#all-settings), pour utiliser Remote Control.
* **Confiance de l'espace de travail** : exécutez `claude` dans votre répertoire de projet au moins une fois pour accepter la boîte de dialogue de confiance de l'espace de travail. La boîte de dialogue de confiance au démarrage ne sauvegarde jamais la confiance pour votre répertoire personnel, donc démarrez Remote Control à partir d'un répertoire de projet.

<h2 id="start-a-remote-control-session">
  Démarrer une session Remote Control
</h2>

Vous pouvez démarrer une session Remote Control à partir de la CLI ou de l'extension VS Code. La CLI offre trois modes d'invocation ; VS Code utilise la commande `/remote-control`.

<Tabs>
  <Tab title="Mode serveur">
    Dans votre répertoire de projet, exécutez :

    ```bash theme={null}
    claude remote-control
    ```

    Jusqu'à ce que vous acceptiez la confirmation unique de Remote Control, `claude remote-control` explique ce qu'il fait et demande `Enable Remote Control? (y/n)` avant de démarrer le serveur. Répondez `y` pour accepter et démarrer le serveur. Si vous refusez, Claude Code se ferme sans démarrer le serveur et demande à nouveau la prochaine fois que vous exécutez la commande.

    Le processus reste en cours d'exécution dans votre terminal en mode serveur, en attente de connexions distantes. Il affiche une URL de session que vous pouvez utiliser pour [vous connecter depuis un autre appareil](#connect-from-another-device), et vous pouvez appuyer sur la barre d'espace pour afficher un code QR pour un accès rapide depuis votre téléphone. Pendant qu'une session distante est active, le terminal affiche l'état de la connexion et l'activité des outils.

    Drapeaux disponibles :

    | Drapeau                                         | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
    | ----------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `--name "My Project"`                           | Définissez un titre de session personnalisé visible dans la liste des sessions sur claude.ai/code.                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
    | `--remote-control-session-name-prefix <prefix>` | Préfixe pour les noms de session générés automatiquement lorsqu'aucun nom explicite n'est défini. Par défaut, le nom d'hôte de votre machine, produisant des noms comme `myhost-graceful-unicorn`. Définissez `CLAUDE_REMOTE_CONTROL_SESSION_NAME_PREFIX` pour le même effet.                                                                                                                                                                                                                                                                                                    |
    | `-c`, `--continue`                              | Reprendre la session que le dernier serveur dans ce répertoire a démarrée, au lieu d'en créer une nouvelle. Voir [Reprendre les sessions après l'arrêt du serveur](#resume-sessions-after-stopping-the-server). Ne peut pas être combiné avec `--session-id`, `--spawn`, `--capacity`, ou `--create-session-in-dir`. Nécessite Claude Code v2.1.200 ou version ultérieure ; les versions antérieures rejettent le drapeau comme argument inconnu.                                                                                                                                |
    | `--session-id <id>`                             | Reprendre une session par son ID. Voir [Reprendre les sessions après l'arrêt du serveur](#resume-sessions-after-stopping-the-server). Ne peut pas être combiné avec `--continue`, `--spawn`, `--capacity`, ou `--create-session-in-dir`. Nécessite Claude Code v2.1.200 ou version ultérieure ; les versions antérieures rejettent le drapeau comme argument inconnu.                                                                                                                                                                                                            |
    | `--spawn <mode>`                                | Comment le serveur crée les sessions.<br />• `same-dir` (par défaut) : toutes les sessions partagent le répertoire de travail actuel, elles peuvent donc entrer en conflit si elles modifient les mêmes fichiers.<br />• `worktree` : chaque session à la demande obtient sa propre [git worktree](/docs/fr/worktrees). Nécessite un référentiel git.<br />• `session` : mode session unique. Sert exactement une session et rejette les connexions supplémentaires. Défini au démarrage uniquement.<br />Appuyez sur `w` à l'exécution pour basculer entre `same-dir` et `worktree`. |
    | `--capacity <N>`                                | Nombre maximum de sessions concurrentes. La valeur par défaut est 32. Ne peut pas être utilisé avec `--spawn=session`.                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
    | `--[no-]create-session-in-dir`                  | Pré-créer une session dans le répertoire actuel au démarrage du serveur, afin que vous ayez un endroit où taper immédiatement. En mode `worktree`, cette session reste dans le répertoire actuel tandis que les sessions à la demande obtiennent des worktrees isolés. Activé par défaut. Si vous passez `--no-create-session-in-dir` pour démarrer sans aucune, Claude Code archive les sessions du serveur lorsque vous l'arrêtez, il n'y a donc rien à [reprendre](#resume-sessions-after-stopping-the-server).                                                               |
    | `--permission-mode <mode>`                      | Définissez le [mode de permission](/docs/fr/permission-modes) de démarrage pour les sessions du serveur, tel que `acceptEdits`. Accepte `manual` comme alias pour `default` ; un mode non reconnu arrête le serveur au démarrage et liste les modes valides.                                                                                                                                                                                                                                                                                                                          |
    | `--debug-file <path>`                           | Écrire les journaux de débogage dans le fichier donné.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
    | `--verbose`                                     | Afficher les journaux de connexion et de session détaillés.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
    | `--sandbox` / `--no-sandbox`                    | Activer ou désactiver le [sandboxing](/docs/fr/sandboxing) pour l'isolation du système de fichiers et du réseau. Désactivé par défaut.                                                                                                                                                                                                                                                                                                                                                                                                                                                |

    Donnez ces drapeaux après `remote-control`.

    Si vous passez un drapeau global `claude` avant `remote-control`, ou si un script wrapper en ajoute un, Claude Code ne porte pas le drapeau aux sessions que le serveur crée. Claude Code laisse passer le drapeau uniquement lorsque le supprimer ne change pas ce que ces sessions peuvent faire, comme `--verbose` ou `--model`. Pour tout autre drapeau, comme `--settings`, Claude Code [refuse de démarrer](/docs/fr/errors#not-carried-over-to-the-sessions-remote-control-starts) et nomme le drapeau à supprimer. Avant v2.1.248, toute option avant `remote-control` faisait que Claude Code rejette les drapeaux après avec une erreur `unknown option`.

    Claude Code vérifie l'admissibilité de Remote Control avant d'imprimer l'aide, donc `claude remote-control --help` retourne une erreur au lieu de cette liste de drapeaux lorsque vous n'êtes pas connecté avec un compte admissible.
  </Tab>

  <Tab title="Session interactive">
    Pour démarrer une session Claude Code interactive normale avec Remote Control activé, utilisez le drapeau `--remote-control` (ou `--rc`) :

    ```bash theme={null}
    claude --remote-control
    ```

    Passez éventuellement un nom pour la session :

    ```bash theme={null}
    claude --remote-control "My Project"
    ```

    Cela vous donne une session interactive complète dans votre terminal que vous pouvez également contrôler depuis claude.ai ou l'application Claude. Contrairement à `claude remote-control` (mode serveur), vous pouvez taper des messages localement tandis que la session est également disponible à distance.
  </Tab>

  <Tab title="À partir d'une session existante">
    Si vous êtes déjà dans une session Claude Code et que vous souhaitez la continuer à distance, utilisez la commande `/remote-control` (ou `/rc`) :

    ```text theme={null}
    /remote-control
    ```

    Passez un nom comme argument pour définir un titre de session personnalisé :

    ```text theme={null}
    /remote-control My Project
    ```

    Cela démarre une session Remote Control qui reprend votre historique de conversation actuel.

    Jusqu'à ce que vous acceptiez la confirmation unique de Remote Control, une boîte de dialogue apparaît avant que `/remote-control` se connecte. Sélectionnez **Enable Remote Control** pour accepter et vous connecter. Si vous sélectionnez **Never mind** ou appuyez sur Échap, Claude Code ne se connecte pas et demande à nouveau la prochaine fois que vous exécutez `/remote-control`.

    Les drapeaux `--verbose`, `--sandbox` et `--no-sandbox` ne sont pas disponibles avec cette commande.
  </Tab>

  <Tab title="VS Code">
    Dans l'[extension VS Code Claude Code](/docs/fr/vs-code), tapez `/remote-control` ou `/rc` dans la zone de saisie.

    ```text theme={null}
    /remote-control
    ```

    Pendant que Remote Control est activé, Claude Code affiche un indicateur **Remote Control** dans le pied de page de la zone de saisie. Une fois la session connectée, cliquez sur l'indicateur pour accéder directement à la session, ou trouvez-la dans la liste des sessions sur [claude.ai/code](https://claude.ai/code). Claude Code affiche également l'URL de la session dans la conversation. Pour vous déconnecter, exécutez `/remote-control` à nouveau.

    Contrairement à la CLI, la commande VS Code n'accepte pas d'argument de nom et n'affiche pas de code QR. Le titre de la session est dérivé de votre historique de conversation ou de votre premier message.
  </Tab>
</Tabs>

<h3 id="check-connection-status">
  Vérifier l'état de la connexion
</h3>

Dans une session interactive, tandis que Remote Control est connecté, le terminal affiche un indicateur `/rc active` qui renvoie à la session sur claude.ai. L'indicateur est masqué lorsque le terminal est trop étroit pour le contenir. Pour voir l'URL de la session et un code QR pour [vous connecter depuis un autre appareil](#connect-from-another-device), exécutez `/remote-control` à nouveau pour ouvrir le panneau d'état. Le panneau vous permet également de déconnecter Remote Control tandis que votre session locale continue de s'exécuter.

<span id="session-ended-elsewhere" />Si la connexion échoue dans une session interactive, l'indicateur change pour afficher l'échec, et Claude Code affiche la raison dans une notification et l'ajoute à la conversation. Exécutez `/remote-control` pour vous reconnecter, sauf si la raison indique que la session a changé ailleurs :

* **Une autre connexion a repris cette session** : un autre appareil ou une autre session Claude Code l'a maintenant. Exécutez `/remote-control` uniquement si vous souhaitez la reprendre.
* **Cette session a été terminée ou archivée depuis un autre appareil ou une autre application** : exécutez `/remote-control` uniquement si vous la souhaitez. Claude Code rouvre une session archivée.
* **Le serveur ne signale plus cette session** : elle peut avoir été supprimée depuis un autre appareil ou une autre application.

<h3 id="session-url-reminders">
  Rappels d'URL de session
</h3>

Pendant que Remote Control est connecté, Claude Code vous rappelle l'URL de la session lorsque passer à votre téléphone ou navigateur aide le plus, afin que vous n'ayez pas à trouver le lien dans `/remote-control`. Un rappel apparaît au-dessus de la zone de saisie à l'un de ces moments :

* **Tour long** : lorsqu'un tour s'exécute plus longtemps qu'un seuil réglé par le serveur, Claude Code affiche une notification **Still working** avec un lien **Check in from your phone**, afin que vous puissiez suivre le tour depuis votre téléphone ou navigateur au lieu d'attendre au terminal. Claude Code la supprime lorsque le tour se termine.
* **Invites de permission répétées** : après avoir répondu à plusieurs [invites de permission](/docs/fr/permissions) dans une session, une notification **Approve tool calls from your phone** affiche l'URL de la session. Claude Code la supprime lorsque votre prochain tour commence.

Les rappels peuvent apparaître dans n'importe quelle session connectée, y compris celles où Remote Control [se connecte automatiquement](#enable-remote-control-for-all-sessions). Ils n'apparaissent pas à chaque fois que ces conditions se produisent, et chacun n'apparaît que quelques fois au total dans les sessions. Vous ne pouvez pas les configurer ou les désactiver ; chacun s'efface de lui-même.

<h3 id="connect-from-another-device">
  Se connecter depuis un autre appareil
</h3>

Une fois qu'une session Remote Control est active, vous avez plusieurs façons de vous connecter depuis un autre appareil :

* **Ouvrez l'URL de la session** dans n'importe quel navigateur pour accéder directement à la session sur [claude.ai/code](https://claude.ai/code).
* **Scannez le code QR** affiché à côté de l'URL de la session pour l'ouvrir directement dans l'application Claude. Avec `claude remote-control`, appuyez sur la barre d'espace pour basculer l'affichage du code QR.
* **Ouvrez [claude.ai/code](https://claude.ai/code) ou l'application Claude** et trouvez la session par nom dans la liste des sessions. Dans l'application mobile Claude, appuyez sur **Code** dans la navigation pour accéder à la liste des sessions. Les sessions Remote Control affichent une icône d'ordinateur avec un point d'état vert lorsqu'elles sont en ligne.

Lorsque vous vous connectez, l'appareil affiche tous les sous-agents et les workflows que la session exécute déjà en arrière-plan. Arrêtez l'un d'eux depuis l'appareil, et Claude Code arrête cette tâche sur votre machine.

Le titre de la session distante est choisi dans cet ordre :

1. Le nom que vous avez passé à `--name`, `--remote-control`, ou `/remote-control`
2. Le titre que vous avez défini avec `/rename`
3. Le dernier message significatif dans l'historique de conversation existant
4. Un nom généré automatiquement comme `myhost-graceful-unicorn`, où `myhost` est le nom d'hôte de votre machine ou le préfixe que vous avez défini avec `--remote-control-session-name-prefix`

Si vous n'avez pas défini de nom explicite, Claude Code met à jour le titre pour refléter votre message une fois que vous en envoyez un. Claude Code fait correspondre les titres générés automatiquement à la langue de votre conversation, ou au paramètre [`language`](/docs/fr/settings-reference#language) s'il est configuré.

Lorsque vous renommez une session depuis claude.ai ou l'application Claude, Claude Code met également à jour le titre local affiché dans `claude --resume`. Claude Code applique le même renommage au nom de session affiché sur la barre de saisie, et dans la liste `claude agents` lorsque la session [s'exécute en arrière-plan](/docs/fr/agent-view). Avant v2.1.221, renommer depuis la liste des sessions sur claude.ai ou dans l'application Claude mettait à jour uniquement le titre, et la CLI conservait son nom de session précédent ; `/rename`, qui s'exécute dans la CLI elle-même, définissait le nom sur n'importe quelle version.

Si vous n'avez pas encore l'application Claude, exécutez `/mobile` dans Claude Code pour afficher un code QR pour [claude.ai/mobile](https://claude.ai/mobile), qui ouvre le bon app store pour votre téléphone.

<h3 id="what-connected-devices-see">
  Ce que les appareils connectés voient
</h3>

Un appareil connecté affiche la conversation dans votre terminal au fur et à mesure. Ces cas vont au-delà des messages ordinaires :

* **Compaction et `/clear`** : tandis que Claude Code [compacte la conversation](/docs/fr/context-window#what-survives-compaction), les appareils connectés affichent la progression et ensuite où la conversation a été compactée. Lorsque vous exécutez `/clear`, la conversation se réinitialise également sur les appareils connectés.
* **Basculer les conversations avec `/resume`** : l'appareil connecté ne reçoit pas le titre de la conversation basculée ou l'historique antérieur, mais les nouveaux messages dans les deux sens vont vers et depuis la conversation ouverte dans votre terminal. Pour travailler à nouveau sur la conversation d'origine depuis l'appareil, exécutez `/resume` dans votre terminal et revenez à celle-ci.
* **Tirer une session avec `/teleport`** : lorsque vous tirez une [session Claude Code sur le web](/docs/fr/claude-code-on-the-web) dans votre terminal avec `/teleport`, l'appareil connecté ne reçoit pas l'historique antérieur de la conversation tirée. Les nouveaux messages dans les deux sens vont vers et depuis la conversation tirée, qui est maintenant celle ouverte dans votre terminal.
* **Messages de vos autres sessions** : avec la [messagerie entre sessions](/docs/fr/cross-session-messaging), la même connexion porte les messages entre vos propres sessions sur différentes machines et depuis vos sessions [Claude Code sur le web](/docs/fr/claude-code-on-the-web), via les serveurs Anthropic comme le reste du trafic Remote Control. [Envoyer des messages aux sessions sur d'autres machines](/docs/fr/cross-session-messaging#message-sessions-on-other-machines) couvre les règles de livraison et [Contrôler les messages entrants](/docs/fr/cross-session-messaging#control-inbound-messages) couvre les contrôles entrants. Nécessite Claude Code v2.1.224 ou version ultérieure.
* **Messages que vous envoyez en milieu de tour** : lorsque vous envoyez un message depuis un appareil connecté avant la fin du tour actuel, Claude Code le met en file d'attente et le conserve dans la transcription de l'appareil après la fin de ce tour.
* **Diff de vos modifications** : lorsque le répertoire de la session se trouve dans un référentiel git, le volet diff d'un appareil connecté affiche vos modifications. L'appareil demande le diff sur la connexion, et Claude Code le calcule sur votre machine. Sur une branche qui a des commits en avance sur la branche par défaut du référentiel, le volet affiche les modifications depuis que la branche s'en est séparée, y compris vos modifications non validées. Sur la branche par défaut elle-même, ou sur une branche qui n'est pas en avance sur elle, le volet affiche uniquement vos modifications non validées. Avant v2.1.247, Claude Code signalait le diff aux appareils connectés uniquement dans les sessions servies par `claude remote-control`.
* **Modèle** : lorsque vous choisissez un [modèle](/docs/fr/model-config) depuis un appareil connecté, Claude Code exécute la session sur ce modèle. Le sélecteur `/model` du terminal, `/status` et `/config` affichent ce modèle. Nécessite Claude Code v2.1.238 ou version ultérieure.
  * Un modèle que vous choisissez depuis le contrôle de modèle de l'appareil s'applique uniquement à la session actuelle. Lorsque vous envoyez `/model <name>` depuis l'appareil à une session interactive, Claude Code définit également votre défaut pour les nouvelles sessions.
  * Si vous envoyez un nom que Claude Code ne reconnaît pas, comme un nom d'affichage où un ID de modèle est attendu, Claude Code [refuse le choix](/docs/fr/errors#model-is-not-a-recognized-model-id) et la session conserve son modèle actuel. Avant v2.1.260, Claude Code enregistrait un choix non reconnu depuis le contrôle de modèle de l'appareil, et votre prochain message échouait.
* **Niveau d'effort** : lorsque vous définissez le [niveau d'effort](/docs/fr/model-config#adjust-effort-level) depuis un appareil connecté, avec `/effort` ou le contrôle d'effort de l'appareil, Claude Code l'applique à la session sur votre machine, et claude.ai/code affiche le niveau que la session utilise. Si vous avez épinglé un niveau avec `CLAUDE_CODE_EFFORT_LEVEL`, la session conserve ce niveau, et Claude Code refuse un choix différent du contrôle d'effort. Choisir un niveau depuis le contrôle d'effort nécessite Claude Code v2.1.234 ou version ultérieure sur votre machine.
* **Reconnexion après une défaillance de connexion** : exécutez `/remote-control` pour vous reconnecter. Si la compaction a réécrit la conversation ou que vous avez basculé les conversations avec `/resume` entre-temps, Claude Code archive la session serveur qu'il utilisait au lieu de la laisser dans la liste des sessions. Vous pouvez toujours la trouver en [filtrant les sessions archivées](/docs/fr/claude-code-on-the-web#archive-sessions). Basculer les conversations tandis qu'un appareil est toujours connecté n'archive pas la session.

<h3 id="enable-remote-control-for-all-sessions">
  Activer Remote Control pour toutes les sessions
</h3>

Remote Control ne s'active que lorsque vous exécutez explicitement `claude remote-control`, `claude --remote-control`, ou `/remote-control`, sauf si l'auto-connexion est activée. Pour l'activer automatiquement pour chaque session interactive, exécutez `/config` dans Claude Code et définissez **Enable Remote Control for all sessions**. Le bouton bascule prend trois valeurs :

* **`true`** : se connecter automatiquement au démarrage d'une session interactive.
* **`false`** : désactiver l'auto-connexion, bien qu'un `true` des [paramètres gérés](/docs/fr/managed-settings) le surclasse, car Claude Code enregistre le choix dans vos paramètres utilisateur. Un `false` dans les paramètres de projet ou locaux (`.claude/settings.json`, `.claude/settings.local.json`) désactive l'auto-connexion même sur un `true` géré.
* **`default`** : effacer votre choix et suivre la valeur par défaut de votre organisation si elle est définie, sinon la valeur par défaut actuelle de Claude Code.

Le même bouton bascule apparaît en dehors de la CLI :

* **Application Desktop** : **Settings > Claude Code > Enable remote control by default**.
* **Extension VS Code** : **Enable Remote Control for all sessions** dans la section Paramètres du [menu de commande](/docs/fr/vs-code#use-the-prompt-box). Nécessite Claude Code v2.1.203 ou version ultérieure.

Pour activer l'auto-connexion à partir d'un fichier de paramètres à la place, définissez [`remoteControlAtStartup`](/docs/fr/settings-reference#remotecontrolatstartup) sur `true` dans votre fichier utilisateur `~/.claude/settings.json` ou dans les [paramètres gérés](/docs/fr/managed-settings). Dans les paramètres de projet ou locaux (`.claude/settings.json`, `.claude/settings.local.json`), Claude Code honore un `false` et désactive l'auto-connexion pour ce référentiel, mais ignore un `true`, afin qu'un fichier archivé ne puisse pas activer Remote Control pour tous ceux qui ouvrent le référentiel.

L'auto-connexion se connecte avec votre propre compte claude.ai, donc une session qu'elle démarre n'apparaît que dans les applications Claude de votre propre compte et n'accorde l'accès à personne d'autre.

Avec ce paramètre activé, chaque processus Claude Code interactif enregistre une session distante. Si vous exécutez plusieurs instances, chacune obtient sa propre session distante. Pour exécuter plusieurs sessions concurrentes à partir d'un seul processus, utilisez plutôt le [mode serveur](#start-a-remote-control-session).

<h3 id="resume-sessions-after-stopping-the-server">
  Reprendre les sessions après l'arrêt du serveur
</h3>

Lorsque vous arrêtez `claude remote-control` avec Ctrl+C, les sessions qu'il servait cessent de répondre depuis votre téléphone ou navigateur. Tant que vous n'exécutiez pas un autre `claude remote-control` dans le même répertoire et que vous n'avez pas démarré celui-ci avec `--no-create-session-in-dir`, Claude Code ne les archive pas. Pour les ramener, exécutez l'une de ces commandes dans le même répertoire :

* **`claude remote-control`** : ramène chaque session que le serveur servait.
* **`claude remote-control --continue`** : ramène uniquement la session que le serveur a démarrée avec, et se termine lorsque cette session se termine. Si ce répertoire n'a pas d'enregistrement, Claude Code utilise le plus récent des autres git worktrees de ce référentiel.
* **`claude remote-control --session-id <id>`** : ramène uniquement la session dont vous passez l'ID, et se termine lorsque cette session se termine. L'ID est la partie de l'URL de la session sur claude.ai/code entre `/code/` et tout `?`.

Ces commandes fonctionnent pendant environ quatre heures après l'arrêt du serveur. Après cela, exécutez `claude remote-control` pour démarrer une nouvelle session. Si vous avez archivé une session entre-temps, `--continue` et `--session-id` la désarchive sur Claude Code v2.1.228 ou version ultérieure.

Pour ramener une session que vous avez démarrée avec `claude --remote-control` ou `/remote-control`, reprenez la conversation avec `claude --continue` ou `claude --resume`. Que Claude Code se reconnecte, et à quelle session, dépend de l'[enregistrement de reconnexion](#resume-outcomes) de la conversation.

Si vous reprenez la conversation dans un deuxième terminal tandis que le premier a toujours Remote Control activé, Claude Code affiche un avis dans le deuxième terminal et laisse Remote Control désactivé à la place de prendre la session au premier. Tandis que Remote Control reste désactivé là, Claude dans ce terminal ne voit pas [vos sessions sur d'autres machines](/docs/fr/cross-session-messaging#see-which-sessions-claude-can-reach), et elles ne peuvent pas le joindre. Exécutez `/remote-control` dans le deuxième terminal pour déplacer Remote Control vers celui-ci.

Lorsque vous reprenez une conversation dans Claude Desktop ou une extension IDE qui avait Remote Control activé, Claude Code la réattache à la session claude.ai existante au lieu d'en ajouter une nouvelle à la liste des sessions.

<h2 id="connection-and-security">
  Connexion et sécurité
</h2>

Votre session Claude Code locale effectue uniquement des requêtes HTTPS sortantes et n'ouvre jamais de ports entrants sur votre machine. Lorsque vous démarrez Remote Control, il s'enregistre auprès de l'API Anthropic et interroge le travail. Lorsque vous vous connectez depuis un autre appareil, le serveur achemine les messages entre le client web ou mobile et votre session locale sur une connexion en continu.

Tout le trafic passe par l'API Anthropic sur TLS, le même transport de sécurité que n'importe quelle session Claude Code. La connexion utilise plusieurs identifiants de courte durée, chacun limité à un seul objectif et expirant indépendamment. Lorsque la credential d'enregistrement d'un serveur `claude remote-control` expire, le serveur s'enregistre à nouveau auprès de l'API Anthropic et continue de servir ses sessions.

Pendant que Remote Control est connecté, la transcription de session, y compris vos messages, les réponses de Claude et l'activité des outils, est stockée sur les serveurs Anthropic. La transcription stockée maintient la conversation synchronisée sur vos appareils et permet à la session de se reconnecter après une interruption réseau. L'exécution et l'accès au système de fichiers restent sur votre machine, et les transcriptions stockées sont conservées selon la politique de [Utilisation des données](/docs/fr/data-usage).

Pour désactiver complètement Remote Control, utilisez le paramètre [`disableRemoteControl`](/docs/fr/settings-reference#disableremotecontrol). Les organisations ayant des exigences de conformité telles que Zero Data Retention ne peuvent pas activer Remote Control.

<h2 id="trusted-devices">
  Appareils de confiance
</h2>

<Note>
  Trusted Devices est actuellement en version bêta. Les fonctionnalités et les capacités peuvent évoluer à mesure que l'expérience est affinée.

  Trusted Devices est disponible sur les plans Pro, Max, Team et Enterprise et est désactivé par défaut. Sur les plans Team et Enterprise, un propriétaire l'active pour l'organisation. Sur les plans Pro et Max, vous activez vous-même **Require trusted devices** dans vos paramètres, sur la page Cowork ou Account.
</Note>

Trusted Devices exige que chaque membre de votre organisation, ou vous seul sur un plan Pro ou Max, vérifie son appareil avant de pouvoir afficher ou contrôler les sessions Remote Control depuis claude.ai, les applications Claude mobiles ou Claude Desktop. Il lie l'accès à Remote Control à un appareil connu et à une authentification récente, pas seulement à un compte connecté.

Lorsque le paramètre est activé, l'interaction avec une session Remote Control nécessite les deux éléments suivants :

* **Un appareil inscrit** : chaque navigateur, téléphone ou application de bureau qu'un membre utilise pour Remote Control enregistre sa propre accréditation. L'inscription n'est proposée que peu de temps après une connexion complète, de sorte qu'un appareil rejoint la liste de confiance dans le cadre d'une authentification réelle plutôt que silencieusement en arrière-plan.
* **Une connexion récente** : la connexion du membre ne doit pas dépasser 18 heures. Au lieu de se connecter à nouveau chaque jour, les membres confirment leur présence avec Face ID, Touch ID, Windows Hello ou une clé d'accès. Cette étape biométrique actualise la session immédiatement.

Les vérifications biométriques s'exécutent sur l'appareil via le système d'exploitation ou le navigateur, le même mécanisme que la connexion par clé d'accès. Anthropic ne reçoit ni ne stocke jamais les empreintes digitales, les données faciales ou toute autre information biométrique. Seule la clé publique de l'appareil et les métadonnées de base telles que le nom d'affichage, la plateforme et l'heure d'inscription sont stockées.

Le paramètre s'applique uniquement à Remote Control. Le chat Claude régulier, Claude Code dans le terminal et l'utilisation de l'API ne sont pas affectés.

<h3 id="enable-trusted-devices-for-your-organization">
  Activer Trusted Devices pour une organisation Team ou Enterprise
</h3>

Un propriétaire active le paramètre à partir des paramètres d'organisation claude.ai.

<Steps>
  <Step title="Accéder à la page Capabilities">
    Allez à [**Organization settings > Capabilities > Remote sessions**](https://claude.ai/admin-settings/capabilities). Le bouton bascule **Require trusted devices** apparaît dans cette section.
  </Step>

  <Step title="Activer Require trusted devices">
    Le paramètre s'applique à chaque membre de l'organisation et aux sessions Remote Control démarrées après son activation. Les sessions qui s'exécutaient déjà avant l'activation du bouton ne sont pas rétroactivement protégées et continuent sans l'exigence d'appareil jusqu'à leur fin. La portée par équipe ou par projet n'est pas disponible.
  </Step>

  <Step title="Informer les membres de ce qu'ils doivent attendre">
    La première fois qu'un membre affiche ou contrôle une nouvelle session Remote Control depuis un navigateur, un téléphone ou une application de bureau après l'activation du paramètre, il est invité à inscrire cet appareil. Les informer à l'avance évite la confusion.
  </Step>
</Steps>

<h3 id="what-members-see">
  Ce que les membres voient
</h3>

L'inscription est une étape unique par appareil. Après cela, le seul changement visible est une invite biométrique occasionnelle.

* **Première utilisation sur chaque appareil** : le membre est invité à s'inscrire. Si sa connexion n'est pas récente, il se connecte d'abord via votre flux normal, y compris SSO s'il est configuré, puis confirme l'inscription.
* **Au quotidien** : les membres avec un appareil inscrit et une connexion récente ne voient aucune invite. Lorsque la connexion dépasse 18 heures, l'interaction Remote Control suivante affiche une seule invite Face ID, Touch ID, Windows Hello ou clé d'accès.
* **Appareils non inscrits** : les sessions Remote Control ne peuvent pas être affichées ou contrôlées jusqu'à ce que l'appareil soit inscrit. Le chat Claude régulier sur cet appareil n'est pas affecté.
* **Pas d'authentificateur de plateforme** : les membres sur une machine sans Face ID, Touch ID ou Windows Hello peuvent utiliser une clé de sécurité matérielle ou se connecter à nouveau au lieu de faire une étape supplémentaire.
* **Dans le terminal** : la machine exécutant Claude Code reçoit sa propre accréditation automatiquement lorsque le développeur se connecte à la CLI. Il n'y a pas d'étape d'inscription séparée dans le terminal.

<h3 id="manage-enrolled-devices">
  Gérer les appareils inscrits
</h3>

Les membres peuvent examiner et révoquer leurs propres appareils à partir des paramètres de compte.

Ouvrez [claude.ai/settings/account](https://claude.ai/settings/account#trusted-devices) et trouvez la section **Trusted devices** pour voir chaque appareil inscrit avec son nom, sa plateforme et sa date d'inscription. La suppression d'un appareil révoque son accréditation immédiatement, et l'appareil peut se réinscrire plus tard après une nouvelle connexion. Les accréditations expirent également d'elles-mêmes si elles ne sont pas renouvelées, de sorte qu'un appareil inutilisé disparaît automatiquement de la liste de confiance.

Pour un appareil perdu ou volé, le membre le supprime de cette page. Si le membre ne peut pas se connecter, un administrateur peut utiliser **Sign out everywhere** dans la console d'administration pour révoquer chaque session et appareil inscrit pour ce membre, après quoi le membre réinscrit les appareils qu'il possède toujours.

<h2 id="remote-control-vs-cloud-sessions">
  Remote Control vs cloud sessions
</h2>

Remote Control et [cloud sessions](/docs/fr/claude-code-on-the-web) utilisent tous deux l'interface claude.ai/code. La différence clé est l'endroit où la session s'exécute : Remote Control s'exécute sur votre machine, donc vos serveurs MCP locaux, outils et configuration de projet restent disponibles. Une cloud session s'exécute sur l'infrastructure cloud, gérée par défaut par Anthropic.

Utilisez Remote Control lorsque vous êtes au milieu d'un travail local et que vous souhaitez continuer depuis un autre appareil. Utilisez une cloud session lorsque vous souhaitez lancer une tâche sans aucune configuration locale, travailler sur un référentiel que vous n'avez pas cloné, ou exécuter plusieurs tâches en parallèle.

<h2 id="mobile-push-notifications">
  Notifications push mobiles
</h2>

Lorsque Remote Control est actif, Claude peut envoyer des notifications push à votre téléphone.

Claude décide quand envoyer une notification. Il en envoie généralement une lorsqu'une tâche longue se termine ou lorsqu'il a besoin d'une décision de votre part pour continuer. Vous pouvez également demander une notification dans votre message, par exemple `notify me when the tests finish`. Au-delà des deux boutons marche/arrêt ci-dessous, il n'y a pas de configuration par événement.

Pour configurer les notifications push mobiles :

<Steps>
  <Step title="Installer l'application Claude mobile">
    Téléchargez l'application Claude pour [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) ou [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude).
  </Step>

  <Step title="Connectez-vous avec votre compte Claude Code">
    Utilisez le même compte et la même organisation que vous utilisez pour Claude Code dans le terminal.
  </Step>

  <Step title="Autoriser les notifications">
    Acceptez l'invite de permission de notification du système d'exploitation.
  </Step>

  <Step title="Activer les notifications dans Claude Code">
    Dans votre terminal, exécutez `/config` et activez **Push when Claude decides** pour les notifications proactives, **Push when actions required** pour les invites de permission et les questions, ou les deux.
  </Step>
</Steps>

Si les notifications n'arrivent pas :

* Si `/config` affiche **No mobile registered**, ouvrez l'application Claude sur votre téléphone pour qu'elle puisse actualiser son jeton push. L'avertissement disparaît la prochaine fois que Remote Control se connecte.
* Sur iOS, les modes Focus et les résumés de notifications peuvent supprimer ou retarder les notifications. Vérifiez Paramètres → Notifications → Claude.
* Sur Android, l'optimisation agressive de la batterie peut retarder la livraison. Exemptez l'application Claude de l'optimisation de la batterie dans les paramètres système.

Claude Code ignore les notifications push mobiles pendant que vous tapez ou que vous êtes concentré sur le terminal connecté. À partir de la v2.1.181, vous pouvez définir [`CLAUDE_CLIENT_PRESENCE_FILE`](/docs/fr/env-vars) sur un chemin de fichier marqueur pour étendre cela à tout moment où vous êtes à la machine, même dans une autre fenêtre : les notifications sont ignorées tant que le fichier existe. Configurez un écouteur de verrouillage d'écran ou un outil similaire pour créer le fichier lorsque votre écran se déverrouille et le supprimer lorsque votre écran se verrouille.

<h2 id="limitations">
  Limitations
</h2>

* **Une session distante par processus interactif** : en dehors du mode serveur, chaque instance Claude Code prend en charge une session distante à la fois. Utilisez le [mode serveur](#start-a-remote-control-session) pour exécuter plusieurs sessions concurrentes à partir d'un seul processus.
* **Le processus local doit continuer à s'exécuter** : Remote Control s'exécute en tant que processus local. Si vous fermez le terminal, quittez VS Code, ou arrêtez autrement le processus `claude`, la session se met hors ligne jusqu'à ce que vous la [rétablissiez](#resume-sessions-after-stopping-the-server). Sauf si Claude est en train d'effectuer une tâche, claude.ai et l'application Claude affichent la session comme hors ligne quelques secondes après la fermeture du processus. Pour maintenir une session en cours d'exécution sur une machine distante après vous être déconnecté de SSH, démarrez-la dans `tmux` ou `screen`.
* **Sessions plantées en mode serveur** : si une session servie par `claude remote-control` plante, envoyez-lui un message à partir d'un appareil connecté. Claude Code la sert à nouveau. Vous n'avez pas besoin de redémarrer le serveur. Nécessite Claude Code v2.1.238 ou ultérieur.
* **Refus HTTP 403 sur une session connectée** : une fois qu'une session interactive est connectée, Claude Code continue à réessayer pendant trois minutes au maximum lorsque quelque chose entre votre machine et les serveurs d'Anthropic répond avec HTTP 403, ce qui peut se produire après un changement de VPN ou de réseau. Si les refus durent plus longtemps, Claude Code se déconnecte, et la raison indique ce qui a refusé : un edge réseau, ou un proxy, VPN, ou pare-feu sur votre propre réseau.
* **Panne réseau prolongée** : si votre machine est allumée mais incapable d'atteindre le réseau, ce que vous faites ensuite dépend du mode :
  * **Mode serveur** : Claude Code abandonne après environ 10 minutes et le processus `claude remote-control` se termine. Exécutez `claude remote-control` à nouveau pour démarrer une nouvelle session.
  * **Session interactive** : continuez à travailler localement. Claude Code réessaye aussi longtemps que la panne dure et se reconnecte automatiquement lorsque le réseau revient.
* **Battements de cœur de présence échouant** : si une session interactive se déconnecte avec `could not reach the Remote Control server for about 30 minutes`, exécutez `/remote-control` pour vous reconnecter. Claude Code affiche ce message uniquement lorsque les battements de cœur de présence de la session ont échoué tandis que le reste de la connexion est resté actif ; il réenregistre la session pendant environ 30 minutes avant de se déconnecter.
* **Les dialogues transférés expirent** : Claude Code maintient les invites de permission et les questions `AskUserQuestion` ouvertes jusqu'à ce que vous y répondiez. Lorsque Claude Code transfère un autre type de dialogue à la session distante, comme l'invite de sélection de modèle affichée après un refus de sécurité, il attend cinq minutes par défaut, puis ferme le dialogue et continue avec la valeur par défaut sans action du dialogue. Définissez [`dialogExpiry`](/docs/fr/settings-reference#dialogexpiry) pour ajuster ou désactiver la limite. Nécessite Claude Code v2.1.224 ou ultérieur.
* **L'invite de consentement Fable usage-credits n'est pas transférée** : Claude Code affiche l'invite de consentement [Fable usage-credits](/docs/fr/model-config#fable-and-usage-credits) en milieu de session uniquement où la session s'exécute, pas sur votre appareil. Lorsque la session s'exécute dans un terminal et que personne là-bas ne répond avant que Claude Code ne ferme l'invite, le tour se termine sans envoyer la demande ; voir [L'invite de confirmation n'a pas reçu de réponse](/docs/fr/errors#the-prompt-to-confirm-went-unanswered).
* **Certaines commandes sont locales uniquement** : les commandes qui s'exécutent uniquement dans l'interface du terminal, telles que `/plugin` ou `/resume`, fonctionnent uniquement à partir de la CLI locale, que vous transmettiez un argument ou non. Les commandes suivantes fonctionnent à partir du mobile et du web :
  * Commandes de sortie textuelle : `/compact`, `/clear`, `/context`, `/usage`, `/exit`, `/usage-credits`, `/recap`, et `/reload-plugins`. `/usage-credits` affiche l'URL de facturation au lieu d'ouvrir un navigateur. `/reload-plugins` fonctionne uniquement lorsque la session s'exécute dans un terminal interactif ; une session sans celui-ci le refuse.
  * `/model`, `/effort`, `/fast`, `/color`, et `/rename` : transmettez la valeur en tant qu'argument, par exemple `/model sonnet` ou `/effort high`. À partir du mobile et du web, `/model` et `/effort` prennent l'argument à la place du sélecteur du terminal ou du curseur.
  * `/mcp` : à partir de l'application mobile, retourne un résumé textuel de l'état du serveur au lieu d'ouvrir le sélecteur. Sur le web, `/mcp` seul ouvre un répertoire des [connecteurs claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai) au lieu de retourner le résumé. Les sous-commandes `reconnect`, `enable`, et `disable` [](/docs/fr/commands#all-commands) fonctionnent à partir des deux. Contrairement à la CLI locale, `/mcp reconnect` sans nom de serveur reconnecte tous les serveurs qui ont échoué ou nécessitent une authentification.
  * `/config` : à partir de l'application mobile, transmettez `key=value` pour définir un paramètre, ou exécutez-le sans argument pour lister les clés que vous pouvez définir. Sur le web, `/config` ouvre la section Claude Code de vos paramètres à la place, et ignore le texte après la commande.
  * Sur Team et Enterprise, `/usage-credits` à partir du mobile ou du web n'envoie pas une [demande usage-credits à votre administrateur](/docs/fr/costs#add-usage-credits-to-your-subscription). L'envoi nécessite une confirmation qui n'apparaît que dans la CLI interactive, donc la commande vous dit de l'exécuter là à la place. Avant la v2.1.211, le formulaire textuel envoyait la demande sans confirmation.
  * `/autocompact`, à partir de la v2.1.221 : transmettez la taille de la fenêtre en tant qu'argument, par exemple `/autocompact 500k`. Sans argument, il affiche la taille de fenêtre actuelle sous forme de texte au lieu d'ouvrir le dialogue que la commande affiche dans une session de terminal.
  * `/advisor`, à partir de la v2.1.260 : transmettez le modèle en tant qu'argument, par exemple `/advisor opus`, ou transmettez `off` pour désactiver le conseiller. Les deux formes s'appliquent à la session actuelle uniquement et laissent votre valeur par défaut enregistrée inchangée. Sans argument, il affiche le conseiller actuel sous forme de texte au lieu d'ouvrir le sélecteur.
  * `/output-style`, à partir de la v2.1.269 : transmettez le nom du style en tant qu'argument, par exemple `/output-style concise`, ou exécutez-le sans argument pour lister les styles. À partir du mobile et du web, vous pouvez lister et sélectionner uniquement les [styles intégrés](/docs/fr/output-styles#built-in-output-styles). Pour utiliser un [style personnalisé](/docs/fr/output-styles#create-a-custom-output-style), sélectionnez-le dans la session elle-même.

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="remote-control-requires-a-claude-ai-subscription">
  « Remote Control requires a claude.ai subscription »
</h3>

Vous n'êtes pas connecté avec un compte claude.ai, ou une autre credential prend la priorité sur votre connexion. Le message prend l'une de ces formes :

* Déconnecté, depuis `/remote-control` ou `--remote-control` : « Remote Control requires a claude.ai subscription. » ou « /remote-control requires a claude.ai subscription. »
* Déconnecté, depuis `claude remote-control` : « You must be logged in to use Remote Control. Remote Control is only available with claude.ai subscriptions. »
* Connecté, mais une clé API ou un token est en cours d'utilisation : « Remote Control requires claude.ai subscription auth. » suivi de la credential en cours d'utilisation, comme « ANTHROPIC\_API\_KEY is set, so this session is using API-key auth ». Un paramètre `apiKeyHelper` et `ANTHROPIC_AUTH_TOKEN` sont nommés de la même manière.

Exécutez `claude auth login` et choisissez l'option claude.ai. Si le message nomme `ANTHROPIC_API_KEY` ou `ANTHROPIC_AUTH_TOKEN`, supprimez-le partout où il est défini : votre environnement shell ou le bloc `env` d'un [fichier de paramètres](/docs/fr/settings-reference#env). S'il nomme `apiKeyHelper`, supprimez ce paramètre.

Avant la v2.1.206, l'exécution de `/remote-control` alors que vous étiez déconnecté signalait « Unknown command: /remote-control » au lieu de ce message.

<h3 id="remote-control-requires-a-full-scope-login-token">
  « Remote Control requires a full-scope login token »
</h3>

Vous êtes authentifié avec un token de longue durée depuis `claude setup-token` ou la variable d'environnement `CLAUDE_CODE_OAUTH_TOKEN`. Ces tokens ne peuvent faire que des demandes de modèle, ils ne peuvent donc pas établir de sessions Remote Control. Exécutez `claude auth login` pour vous authentifier avec un token de session à portée complète à la place.

<h3 id="unable-to-determine-your-organization-for-remote-control-eligibility">
  « Unable to determine your organization for Remote Control eligibility »
</h3>

Vos informations de compte en cache sont obsolètes ou incomplètes. Exécutez `claude auth login` pour les actualiser.

<h3 id="remote-control-isn’t-enabled-for-this-account">
  « Remote Control isn't enabled for this account »
</h3>

Claude Code a vérifié la disponibilité de Remote Control pour le compte auquel vous êtes connecté et la vérification a indiqué que c'était désactivé. La cause habituelle est des droits en cache qui sont obsolètes après un changement de plan. Exécutez `claude auth logout` puis `claude auth login` pour les actualiser, et mettez à jour Claude Code si vous utilisez une ancienne version.

Exécutez `claude doctor` pour voir quelle vérification d'éligibilité individuelle a échoué. Les conflits de variables d'environnement, les vérifications inaccessibles et le paramètre Remote Control de votre organisation produisent chacun leur propre message, donc cette erreur signifie que la vérification au niveau du compte elle-même.

Avant la v2.1.239, ce message disait « Remote Control is not yet enabled for your account ». Avant la v2.1.154, une variable qui désactive l'évaluation des feature flags, comme `DISABLE_TELEMETRY` ou `DO_NOT_TRACK`, produisait également ce message ; l'entrée « Remote Control requires feature-flag evaluation » ci-dessous couvre cette configuration.

<h3 id="couldn’t-verify-remote-control-eligibility">
  « Couldn't verify Remote Control eligibility »
</h3>

Claude Code n'a pas pu atteindre le service de feature flags pour vérifier si Remote Control est activé pour votre compte, généralement parce que vous êtes hors ligne ou qu'un proxy bloque la demande. Réessayez une fois que vous avez accès au réseau, ou exécutez `claude doctor` pour plus de détails. Le message connexe « Couldn't verify your organization's Remote Control policy » signifie que Claude Code n'a pas pu lire cette politique, et a la même solution. Les deux messages ont été ajoutés dans la v2.1.178.

<h3 id="remote-control-requires-feature-flag-evaluation">
  « Remote Control requires feature-flag evaluation »
</h3>

L'une de ces variables est définie : [`DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`, ou `DISABLE_GROWTHBOOK`](/docs/fr/env-vars). Chacune d'elles désactive l'évaluation des feature flags dont dépend la disponibilité de Remote Control, et le message complet nomme la variable que Claude Code a trouvée. Annulez la définition de cette variable partout où elle est définie, dans votre environnement shell ou dans le bloc `env` d'un [fichier `settings.json`](/docs/fr/settings-reference#all-settings). Sur les versions antérieures à 2.1.154, la même configuration produit « Remote Control is not yet enabled for your account » à la place.

<h3 id="remote-control-is-only-available-when-using-claude-via-api-anthropic-com">
  « Remote Control is only available when using Claude via api.anthropic.com »
</h3>

La session ne communique pas directement avec l'API Anthropic, il n'y a donc pas de backend claude.ai pour l'associer. Cela se produit sur Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry. Cela se produit également lorsque [`ANTHROPIC_BASE_URL`](/docs/fr/env-vars) pointe vers un hôte autre que `api.anthropic.com`, comme une [passerelle LLM](/docs/fr/llm-gateway) ou un proxy, même si vous vous connectez avec claude.ai. Avant la v2.1.196, Claude Code n'affichait pas ce message pour un `ANTHROPIC_BASE_URL` personnalisé. Consultez la [référence des erreurs](/docs/fr/errors#remote-control-requires-the-anthropic-api) pour la liste complète des causes.

Le message nomme ce qui a routé la session loin de l'API Anthropic, comme `CLAUDE_CODE_USE_BEDROCK` ou un `ANTHROPIC_BASE_URL` personnalisé. Si vous avez une connexion claude.ai éligible, annulez la définition de la variable nommée, supprimez-la de la clé `env` dans [les paramètres](/docs/fr/settings) si vous l'avez définie là, et redémarrez la session. Avant la v2.1.219, le message était seulement la phrase dans l'en-tête de cette section, donc sur les versions plus anciennes, vérifiez vous-même votre environnement pour les variables de fournisseur comme `CLAUDE_CODE_USE_BEDROCK` et `CLAUDE_CODE_USE_VERTEX`, et pour `ANTHROPIC_BASE_URL`.

<h3 id="remote-control-is-disabled-by-your-organization’s-policy">
  « Remote Control is disabled by your organization's policy »
</h3>

Une politique bloque Remote Control, ou Claude Code n'a pas pu charger la politique de votre organisation sur cette machine et garde Remote Control désactivé en attendant. Vérifiez ces causes dans l'ordre :

* **L'erreur mentionne `disableRemoteControl`** : votre administrateur informatique a désactivé Remote Control sur cet appareil via les [paramètres gérés](/docs/fr/managed-settings), indépendamment du bouton bascule à l'échelle de l'organisation et de la façon dont vous êtes connecté.
* **Votre plan claude.ai est Pro ou Max** : Claude Code est toujours connecté sous une organisation Team ou Enterprise d'une connexion antérieure, il vérifie donc la politique Remote Control de cette organisation. Exécutez `/status` pour voir quel plan et quelle organisation votre connexion utilise. Exécutez `claude auth logout` puis `claude auth login` pour vous reconnecter sous votre plan actuel.
* **La politique de l'organisation n'a pas été chargée sur cette machine** : exécutez `claude doctor` et lisez la ligne `Organization policy`. Si la ligne montre que la politique n'est pas chargée, c'est ce qui garde Remote Control désactivé. Avant la v2.1.261, `claude doctor` n'imprimait pas cette ligne.
* **Le message ne dit pas de contacter votre administrateur d'organisation** : votre organisation a une configuration HIPAA qui est incompatible avec Remote Control, et `/status` liste `HIPAA` dans sa ligne `Compliance`. Dans cet état, le bouton bascule Remote Control du panneau d'administration est grisé, donc un Owner ne peut pas le modifier là. Contactez le support Anthropic pour discuter des options. Avant la v2.1.267, ce cas affichait « Remote Control isn't available for your organization due to its compliance policy » à la place.
* **Sinon, un Owner ne l'a pas activé pour votre organisation** : Remote Control est désactivé par défaut sur les plans Team et Enterprise. Un Owner peut l'activer sur [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) en activant le bouton bascule **Remote Control**. Ce bouton bascule est un paramètre d'organisation côté serveur.

<h3 id="remote-credentials-fetch-failed">
  « Remote credentials fetch failed »
</h3>

Claude Code n'a pas pu obtenir une credential de courte durée de l'API Anthropic pour établir la connexion. Réexécutez avec `--verbose` pour voir l'erreur complète :

```bash theme={null}
claude remote-control --verbose
```

Causes courantes :

* Non connecté : exécutez `claude` et utilisez `/login` pour vous authentifier avec votre compte claude.ai. L'authentification par clé API n'est pas prise en charge pour Remote Control.
* Problème de réseau ou de proxy : un pare-feu ou un proxy peut bloquer la demande HTTPS sortante. Remote Control nécessite l'accès à l'API Anthropic sur le port 443.
* Échec de la création de session : si vous voyez également « Session creation failed — see debug log », l'échec s'est produit plus tôt dans la configuration. Vérifiez que votre abonnement est actif.

Un token de connexion obsolète ne cause pas cette erreur. Lorsque l'API Anthropic rejette le token enregistré, par exemple parce qu'un autre processus Claude Code l'a déjà actualisé, Claude Code actualise le token et réessaie de lui-même. Avant la v2.1.224, un token obsolète échouait au démarrage de Remote Control avec ce message, donc les sessions définies pour [se connecter automatiquement](#enable-remote-control-for-all-sessions) pouvaient échouer par intermittence au lancement.

<h3 id="couldn’t-reconnect-to-your-remote-control-session">
  « Couldn't reconnect to your Remote Control session »
</h3>

Lorsque vous reprenez une conversation avec `claude --resume` ou `claude --continue`, Claude Code se reconnecte à la session Remote Control enregistrée dans cette conversation. Ce message signifie que la reconnexion a échoué pour une raison qui peut être temporaire, comme une interruption réseau ou une erreur serveur, donc Claude Code ne peut pas confirmer si la session distante existe toujours.

Exécutez `/remote-control` pour réessayer la connexion, ou démarrez une nouvelle session avec `claude --remote-control` pour créer une nouvelle session Remote Control. Votre session locale continue de s'exécuter sans Remote Control en attendant.

<span id="resume-outcomes" />Lorsque vous reprenez, vous pouvez également obtenir l'un de ces résultats au lieu de ce message :

* **Le serveur signale que la session enregistrée a disparu, ou l'enregistrement de reconnexion nomme un compte différent** : Claude Code se fie à ce que l'enregistrement de reconnexion de la conversation dit :
  * **L'enregistrement nomme votre compte connecté** : Claude Code démarre une session de remplacement avec un nom généré automatiquement et laisse les messages antérieurs de la conversation en dehors. Vous obtenez cela après avoir supprimé la session de claude.ai ou de l'application Claude, par exemple.
  * **L'enregistrement nomme un compte différent** : Claude Code démarre une nouvelle session sans les messages antérieurs de la conversation et sans afficher de message, que la session enregistrée existe toujours ou non.
  * **L'enregistrement ne dit pas quel compte possédait la session, ou Claude Code ne peut pas lire votre connexion enregistrée** : Claude Code affiche [`Previous session is unavailable — run /remote-control to start a new one`](#previous-session-is-unavailable) au lieu de ce message, ne démarre rien, et supprime l'enregistrement de la conversation.
* **Vous avez désactivé Remote Control avant de reprendre** : sauf si l'application hébergeant Claude Code lui avait dit que l'application possédait la session claude.ai, Claude Code a supprimé l'enregistrement de reconnexion lorsque vous avez désactivé Remote Control depuis le [panneau de statut](#check-connection-status) de la CLI, l'extension VS Code, ou un hôte construit sur le [SDK Agent](/docs/fr/agent-sdk/overview), donc il ne se reconnecte pas. Lorsqu'une application propriétaire l'a désactivé, Claude Code a conservé l'enregistrement et se reconnecte.
* **Un autre Claude Code sur cette machine possède toujours la session** : vous voyez un avis qui commence par « Remote Control not started here », et Claude Code [laisse Remote Control désactivé dans la session reprise](#resume-sessions-after-stopping-the-server). Exécutez `/remote-control` là pour le déplacer.

<span id="reconnect-history" />Avant la v2.1.232, Claude Code répondait différemment lorsque le serveur signalait que la session enregistrée avait disparu. De la v2.1.227 à la v2.1.231, Claude Code refusait de démarrer un remplacement même lorsque l'enregistrement correspondait à votre compte. Jusqu'à la v2.1.226, Claude Code démarrait un remplacement que l'enregistrement corresponde ou non à votre compte, et dans la v2.1.224 à la v2.1.226 le créait sous le compte connecté sur cette machine, jamais un autre compte, sans télécharger les messages antérieurs de la conversation vers celui-ci. Avant la v2.1.200, Claude Code créait une nouvelle session après tout échec de reconnexion.

<h3 id="previous-session-is-unavailable">
  « Previous session is unavailable — run /remote-control to start a new one »
</h3>

Claude Code n'a pas pu ramener la session Remote Control précédente et s'est arrêté au lieu de démarrer une nouvelle de lui-même. Vous pouvez voir ce message après avoir repris une conversation avec `claude --resume` ou `claude --continue`, ou après que Claude Code [se reconnecte de lui-même suite à une déconnexion](/docs/fr/errors#remote-control-couldnt-refresh-your-login).

Exécutez `/remote-control` pour démarrer une nouvelle session Remote Control sous la connexion actuelle ; votre session locale continue de s'exécuter sans Remote Control en attendant. Le message connexe « Remote Control could not verify the signed-in account — run /remote-control to reconnect » a la même solution ; Claude Code l'affiche lorsque le compte connecté a changé ou n'a pas pu être lu entre sa validation et la reconnexion. Si vous exécutez `/remote-control` après « Previous session is unavailable » sans redémarrer Claude Code d'abord, Claude Code laisse les messages antérieurs de la conversation en dehors de la nouvelle session.

À la reprise, Claude Code [démarre une nouvelle session à sa place](#resume-outcomes) seulement si l'enregistrement de reconnexion de la conversation nomme le compte qui possédait la session, parce que le serveur signale une session que vous avez supprimée et une session possédée par un autre compte de la même manière. Claude Code avant la v2.1.227 n'a pas enregistré ce compte, et Claude Code ne peut pas vérifier l'enregistrement lorsqu'il ne peut pas lire votre connexion enregistrée. Claude Code avant la v2.1.232 affichait « Remote Control could not resume the previous session under the current login — run /remote-control to start fresh » à la place, dans [un ensemble différent de cas](#reconnect-history).

<h3 id="remote-control-got-an-unexpected-server-response">
  « Remote Control got an unexpected server response »
</h3>

Le serveur Remote Control a accepté une demande mais a répondu d'une forme que cette version de Claude Code ne pouvait pas lire, lors de la création de la session distante ou de la récupération de ses credentials. Réessayer sur la même version échoue de la même manière. Exécutez `claude update`, puis exécutez `/remote-control` pour vous reconnecter. Ce message a été ajouté dans la v2.1.225.

<h3 id="your-organization-requires-trusted-devices-for-remote-control-but-this-device-is-not-enrolled">
  « Your organization requires Trusted Devices for Remote Control, but this device is not enrolled »
</h3>

Votre organisation a [Trusted Devices](#trusted-devices) activé et cette machine ne s'est pas encore inscrite. Exécutez `/login` dans Claude Code. L'inscription se fait dans le cadre de la connexion, et il n'y a pas de commande d'inscription séparée.

<h3 id="session-expired-for-trusted-device-check">
  « session expired for trusted-device check »
</h3>

Votre connexion a plus de 18 heures. Exécutez `/login` dans Claude Code, ou confirmez avec Face ID, Touch ID, Windows Hello, ou une passkey lorsque claude.ai ou l'application mobile vous le demande. Consultez [Trusted Devices](#trusted-devices).

<h2 id="choose-the-right-approach">
  Choisir la bonne approche
</h2>

Claude Code offre plusieurs façons de travailler quand vous n'êtes pas à votre terminal. Elles diffèrent par ce qui déclenche le travail, où Claude s'exécute et la quantité de configuration dont vous avez besoin.

|                                                              | Déclencheur                                                                                                 | Claude s'exécute sur                                                                         | Configuration                                                                                                                               | Idéal pour                                                                   |
| :----------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------- |
| [Dispatch](/docs/fr/desktop#sessions-from-dispatch)               | Envoyer une tâche depuis l'application mobile Claude                                                        | Votre machine (Desktop)                                                                      | [Associer l'application mobile à Desktop](https://support.claude.com/en/articles/13947068)                                                  | Déléguer du travail quand vous êtes absent, configuration minimale           |
| [Remote Control](/docs/fr/remote-control)                         | Piloter une session en cours depuis [claude.ai/code](https://claude.ai/code) ou l'application mobile Claude | Votre machine (CLI ou VS Code)                                                               | Exécuter `claude remote-control`                                                                                                            | Diriger le travail en cours depuis un autre appareil                         |
| [Channels](/docs/fr/channels)                                     | Envoyer des événements depuis une application de chat comme Telegram ou Discord, ou votre propre serveur    | Votre machine (CLI)                                                                          | [Installer un plugin de canal](/docs/fr/channels#quickstart) ou [créer le vôtre](/docs/fr/channels-reference)                                         | Réagir à des événements externes comme les échecs CI ou les messages de chat |
| [Slack](/docs/fr/slack)                                           | Mentionner `@Claude` dans un canal d'équipe                                                                 | Cloud Anthropic                                                                              | [Installer l'application Slack](/docs/fr/slack#setting-up-claude-code-in-slack) avec [Claude Code sur le web](/docs/fr/claude-code-on-the-web) activé | PRs et révisions depuis le chat d'équipe                                     |
| [Environnements auto-hébergés](/docs/fr/self-hosted-environments) | Démarrer une [session cloud](/docs/fr/claude-code-on-the-web) et choisir l'environnement de votre organisation   | Infrastructure de votre organisation                                                         | [Déployer des runners](/docs/fr/self-hosted-environments-quickstart), sur les plans Team et Enterprise                                           | Sessions cloud qui doivent s'exécuter dans votre réseau                      |
| [Tâches planifiées](/docs/fr/scheduled-tasks)                     | Définir un calendrier                                                                                       | [CLI](/docs/fr/scheduled-tasks), [Desktop](/docs/fr/desktop-scheduled-tasks), ou [cloud](/docs/fr/routines) | Choisir une fréquence                                                                                                                       | Automatisation récurrente comme les révisions quotidiennes                   |

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Claude Code sur le web](/docs/fr/claude-code-on-the-web) : exécutez des sessions dans le cloud au lieu de sur votre machine, configurées via des [environnements cloud](/docs/fr/cloud-environments)
* [Messagerie inter-sessions](/docs/fr/cross-session-messaging) : permettez à Claude d'envoyer des messages à vos sessions sur d'autres machines ou sur vos [sessions cloud](/docs/fr/claude-code-on-the-web)
* [Canaux](/docs/fr/channels) : transférez Telegram, Discord ou iMessage dans une session afin que Claude réagisse aux messages pendant que vous êtes absent
* [Dispatch](/docs/fr/desktop#sessions-from-dispatch) : envoyez un message avec une tâche depuis votre téléphone et il peut générer une session Desktop pour la gérer
* [Authentification](/docs/fr/authentication) : configurez `/login` et gérez les identifiants pour claude.ai
* [Référence CLI](/docs/fr/cli-reference) : liste complète des drapeaux et commandes incluant `claude remote-control`
* [Sécurité](/docs/fr/security) : comment les sessions Remote Control s'intègrent dans le modèle de sécurité Claude Code
* [Utilisation des données](/docs/fr/data-usage) : quelles données circulent via l'API Anthropic lors des sessions locales, Remote Control et cloud
