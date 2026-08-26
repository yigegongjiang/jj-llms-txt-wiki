> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Continuer les sessions locales depuis n'importe quel appareil avec Remote Control

> Continuez une session Claude Code locale depuis votre téléphone, tablette ou n'importe quel navigateur en utilisant Remote Control. Fonctionne avec claude.ai/code et l'application Claude mobile.

<Note>
  Remote Control est en aperçu de recherche et disponible sur tous les plans. Sur Team et Enterprise, il est désactivé par défaut jusqu'à ce qu'un administrateur active le bouton Remote Control dans les [paramètres d'administration Claude Code](https://claude.ai/admin-settings/claude-code).
</Note>

Remote Control connecte [claude.ai/code](https://claude.ai/code) ou l'application Claude pour [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) et [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) à une session Claude Code s'exécutant sur votre machine. Commencez une tâche à votre bureau, puis reprenez-la depuis votre téléphone sur le canapé ou un navigateur sur un autre ordinateur.

Lorsque vous démarrez une session Remote Control sur votre machine, Claude continue à s'exécuter localement à tout moment, donc votre exécution de code et votre accès au système de fichiers restent sur votre machine. Avec Remote Control, vous pouvez :

* **Utiliser votre environnement local complet à distance** : votre système de fichiers, [serveurs MCP](/docs/fr/mcp), outils et configuration de projet restent tous disponibles, et taper `@` complète automatiquement les chemins de fichiers de votre projet local
* **Travailler depuis les deux surfaces à la fois** : la conversation et la progression des [sous-agents](/docs/fr/sub-agents) et des [flux de travail dynamiques](/docs/fr/workflows) restent synchronisés sur tous les appareils connectés, vous pouvez donc envoyer des messages depuis votre terminal, navigateur et téléphone de manière interchangeable. Avant la v2.1.207, les sessions hébergées par l'[application de bureau](/docs/fr/desktop) n'envoyaient pas la progression des sous-agents ou des flux de travail aux appareils connectés.
* **Envoyer des images et des fichiers depuis votre téléphone ou navigateur** : lorsque vous ajoutez une pièce jointe dans l'application Claude ou sur claude.ai/code, Claude Code la télécharge sur votre machine et la transmet à Claude en tant que référence de fichier `@`, avec ou sans légende. Avant la v2.1.202, Claude Code pouvait supprimer une pièce jointe envoyée sans légende avant qu'elle n'atteigne la session.
* **Survivre aux interruptions** : si votre ordinateur portable s'endort ou votre réseau tombe en panne, la session se reconnecte automatiquement lorsque votre machine revient en ligne. Claude Code met en file d'attente les mises à jour de statut des sous-agents et des flux de travail pendant que la connexion se rétablit et les livre une fois qu'elle se rétablit. Avant la v2.1.207, une mise à jour envoyée lors d'une reconnexion ou d'une actualisation des identifiants pouvait être perdue, donc l'appareil connecté continuait à afficher une tâche terminée comme en cours d'exécution.

Contrairement à [Claude Code sur le web](/docs/fr/claude-code-on-the-web), qui s'exécute sur l'infrastructure cloud, les sessions Remote Control s'exécutent directement sur votre machine et interagissent avec votre système de fichiers local. Les interfaces web et mobile ne sont qu'une fenêtre dans cette session locale.

Cette page couvre la configuration, comment démarrer et se connecter aux sessions, et comment Remote Control se compare à Claude Code sur le web.

<h2 id="requirements">
  Conditions requises
</h2>

Avant d'utiliser Remote Control, confirmez que votre environnement répond à ces conditions :

* **Abonnement** : disponible sur les plans Pro, Max, Team et Enterprise. Les clés API ne sont pas prises en charge. Sur Team et Enterprise, un propriétaire doit d'abord activer le bouton Remote Control dans les [paramètres d'administration Claude Code](https://claude.ai/admin-settings/claude-code).
* **Authentification** : exécutez `claude` et utilisez `/login` pour vous connecter via claude.ai si vous ne l'avez pas déjà fait.
* **Point de terminaison API** : non disponible sur Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry. À partir de la v2.1.196, Remote Control est également désactivé lorsque [`ANTHROPIC_BASE_URL`](/docs/fr/env-vars) pointe vers un hôte autre que `api.anthropic.com`, tel qu'une [passerelle LLM](/docs/fr/llm-gateway) ou un proxy. Déconfigurez la variable pour utiliser Remote Control.
* **Confiance de l'espace de travail** : exécutez `claude` dans votre répertoire de projet au moins une fois pour accepter la boîte de dialogue de confiance de l'espace de travail.

<h2 id="start-a-remote-control-session">
  Démarrer une session Remote Control
</h2>

Vous pouvez démarrer une session Remote Control à partir de la CLI ou de l'extension VS Code. La CLI offre trois modes d'invocation ; VS Code utilise la commande `/remote-control`.

<Tabs>
  <Tab title="Mode serveur">
    Accédez à votre répertoire de projet et exécutez :

    ```bash theme={null}
    claude remote-control
    ```

    Le processus reste en cours d'exécution dans votre terminal en mode serveur, en attente de connexions distantes. Il affiche une URL de session que vous pouvez utiliser pour [vous connecter depuis un autre appareil](#connect-from-another-device), et vous pouvez appuyer sur la barre d'espace pour afficher un code QR pour un accès rapide depuis votre téléphone. Pendant qu'une session distante est active, le terminal affiche l'état de la connexion et l'activité des outils.

    Drapeaux disponibles :

    | Drapeau                                         | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
    | ----------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `--name "My Project"`                           | Définissez un titre de session personnalisé visible dans la liste des sessions sur claude.ai/code.                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
    | `--remote-control-session-name-prefix <prefix>` | Préfixe pour les noms de session générés automatiquement lorsqu'aucun nom explicite n'est défini. Par défaut, le nom d'hôte de votre machine, produisant des noms comme `myhost-graceful-unicorn`. Définissez `CLAUDE_REMOTE_CONTROL_SESSION_NAME_PREFIX` pour le même effet.                                                                                                                                                                                                                                                                                                    |
    | `-c`, `--continue`                              | Reprendre la session Remote Control la plus récente démarrée à partir de ce répertoire au lieu d'en créer une nouvelle. Ne peut pas être combiné avec `--session-id`, `--spawn`, `--capacity`, ou `--create-session-in-dir`. Nécessite Claude Code v2.1.200 ou version ultérieure ; les versions antérieures rejettent le drapeau comme argument inconnu.                                                                                                                                                                                                                        |
    | `--session-id <id>`                             | Reprendre une session Remote Control spécifique par son ID. Ne peut pas être combiné avec `--continue`, `--spawn`, `--capacity`, ou `--create-session-in-dir`. Nécessite Claude Code v2.1.200 ou version ultérieure ; les versions antérieures rejettent le drapeau comme argument inconnu.                                                                                                                                                                                                                                                                                      |
    | `--spawn <mode>`                                | Comment le serveur crée les sessions.<br />• `same-dir` (par défaut) : toutes les sessions partagent le répertoire de travail actuel, elles peuvent donc entrer en conflit si elles modifient les mêmes fichiers.<br />• `worktree` : chaque session à la demande obtient sa propre [git worktree](/docs/fr/worktrees). Nécessite un référentiel git.<br />• `session` : mode session unique. Sert exactement une session et rejette les connexions supplémentaires. Défini au démarrage uniquement.<br />Appuyez sur `w` à l'exécution pour basculer entre `same-dir` et `worktree`. |
    | `--capacity <N>`                                | Nombre maximum de sessions concurrentes. La valeur par défaut est 32. Ne peut pas être utilisé avec `--spawn=session`.                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
    | `--[no-]create-session-in-dir`                  | Pré-créer une session dans le répertoire actuel au démarrage du serveur, afin que vous ayez un endroit où taper immédiatement. En mode `worktree`, cette session reste dans le répertoire actuel tandis que les sessions à la demande obtiennent des worktrees isolés. Activé par défaut ; passez `--no-create-session-in-dir` pour démarrer sans aucune.                                                                                                                                                                                                                        |
    | `--verbose`                                     | Afficher les journaux de connexion et de session détaillés.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
    | `--sandbox` / `--no-sandbox`                    | Activer ou désactiver le [sandboxing](/docs/fr/sandboxing) pour l'isolation du système de fichiers et du réseau. Désactivé par défaut.                                                                                                                                                                                                                                                                                                                                                                                                                                                |
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

    Les drapeaux `--verbose`, `--sandbox` et `--no-sandbox` ne sont pas disponibles avec cette commande.
  </Tab>

  <Tab title="VS Code">
    Dans l'[extension VS Code Claude Code](/docs/fr/vs-code), tapez `/remote-control` ou `/rc` dans la zone de saisie, ou ouvrez le menu de commande avec `/` et sélectionnez-le.

    ```text theme={null}
    /remote-control
    ```

    Une bannière apparaît au-dessus de la zone de saisie montrant l'état de la connexion. Une fois connecté, cliquez sur **Open in browser** dans la bannière pour accéder directement à la session, ou trouvez-la dans la liste des sessions sur [claude.ai/code](https://claude.ai/code). L'URL de la session est également affichée dans la conversation.

    Pour vous déconnecter, cliquez sur l'icône de fermeture sur la bannière ou exécutez `/remote-control` à nouveau.

    Contrairement à la CLI, la commande VS Code n'accepte pas d'argument de nom et n'affiche pas de code QR. Le titre de la session est dérivé de votre historique de conversation ou de votre premier message.
  </Tab>
</Tabs>

<h3 id="check-connection-status">
  Vérifier l'état de la connexion
</h3>

Dans une session de terminal interactive, un indicateur `/rc active` se trouve dans le pied de page sous la zone de saisie tandis que la connexion est active, et est masqué si le terminal est trop étroit pour le contenir. Le texte de l'indicateur est un lien vers la session sur claude.ai. Sélectionnez-le avec la flèche vers le bas et appuyez sur Entrée, ou exécutez `/remote-control` à nouveau, pour ouvrir un panneau d'état avec l'URL de la session et un code QR que vous pouvez utiliser pour [vous connecter depuis un autre appareil](#connect-from-another-device).

Si la connexion échoue, une notification apparaît avec la raison de l'échec et l'indicateur disparaît du pied de page. Exécutez `/remote-control` à nouveau pour réessayer.

<h3 id="connect-from-another-device">
  Se connecter depuis un autre appareil
</h3>

Une fois qu'une session Remote Control est active, vous avez plusieurs façons de vous connecter depuis un autre appareil :

* **Ouvrez l'URL de la session** dans n'importe quel navigateur pour accéder directement à la session sur [claude.ai/code](https://claude.ai/code).
* **Scannez le code QR** affiché à côté de l'URL de la session pour l'ouvrir directement dans l'application Claude. Avec `claude remote-control`, appuyez sur la barre d'espace pour basculer l'affichage du code QR.
* **Ouvrez [claude.ai/code](https://claude.ai/code) ou l'application Claude** et trouvez la session par nom dans la liste des sessions. Dans l'application mobile Claude, appuyez sur **Code** dans la navigation pour accéder à la liste des sessions. Les sessions Remote Control affichent une icône d'ordinateur avec un point d'état vert lorsqu'elles sont en ligne.

Lorsque vous vous connectez, l'appareil affiche tous les sous-agents et les workflows que la session exécute déjà en arrière-plan. Avant v2.1.208, un appareil se connectant à une session hébergée dans un terminal interactif n'affichait pas les sous-agents et les workflows qui étaient déjà en cours d'exécution jusqu'à ce que l'un d'eux démarre ou s'arrête.

Le titre de la session distante est choisi dans cet ordre :

1. Le nom que vous avez passé à `--name`, `--remote-control`, ou `/remote-control`
2. Le titre que vous avez défini avec `/rename`
3. Le dernier message significatif dans l'historique de conversation existant
4. Un nom généré automatiquement comme `myhost-graceful-unicorn`, où `myhost` est le nom d'hôte de votre machine ou le préfixe que vous avez défini avec `--remote-control-session-name-prefix`

Si vous n'avez pas défini de nom explicite, le titre se met à jour pour refléter votre message une fois que vous en envoyez un. À partir de Claude Code v2.1.176, les titres générés automatiquement correspondent à la langue de votre conversation, ou au paramètre [`language`](/docs/fr/settings#available-settings) s'il est configuré. Renommer une session depuis claude.ai ou l'application Claude met également à jour le titre local affiché dans `claude --resume`.

Si l'environnement a déjà une session active, vous serez invité à choisir si vous souhaitez la continuer ou en démarrer une nouvelle.

Si vous n'avez pas encore l'application Claude, utilisez la commande `/mobile` dans Claude Code pour afficher un code QR de téléchargement pour [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) ou [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude).

<h3 id="enable-remote-control-for-all-sessions">
  Activer Remote Control pour toutes les sessions
</h3>

Remote Control ne s'active que lorsque vous exécutez explicitement `claude remote-control`, `claude --remote-control`, ou `/remote-control`, sauf si l'auto-connexion est activée. Pour l'activer automatiquement pour chaque session interactive, exécutez `/config` dans Claude Code et définissez **Enable Remote Control for all sessions** sur `true`. Définissez-le sur `false` pour ne jamais l'auto-connecter, ou laissez-le non défini pour suivre la valeur par défaut de votre organisation. Dans l'application Desktop, vous pouvez également basculer ceci depuis **Settings → Claude Code → Enable remote control by default**. Dans l'[extension VS Code](/docs/fr/vs-code#use-the-prompt-box), le même bouton bascule apparaît comme **Enable Remote Control for all sessions** dans la section Paramètres du menu de commande ; nécessite Claude Code v2.1.203 ou version ultérieure.

Avec ce paramètre activé, chaque processus Claude Code interactif enregistre une session distante. Si vous exécutez plusieurs instances, chacune obtient son propre environnement et sa propre session. Pour exécuter plusieurs sessions concurrentes à partir d'un seul processus, utilisez plutôt le [mode serveur](#start-a-remote-control-session).

<h2 id="connection-and-security">
  Connexion et sécurité
</h2>

Votre session Claude Code locale effectue uniquement des requêtes HTTPS sortantes et n'ouvre jamais de ports entrants sur votre machine. Lorsque vous démarrez Remote Control, il s'enregistre auprès de l'API Anthropic et interroge le travail. Lorsque vous vous connectez depuis un autre appareil, le serveur achemine les messages entre le client web ou mobile et votre session locale sur une connexion en continu.

Tout le trafic passe par l'API Anthropic sur TLS, le même transport de sécurité que n'importe quelle session Claude Code. La connexion utilise plusieurs identifiants de courte durée, chacun limité à un seul objectif et expirant indépendamment.

Pendant que Remote Control est connecté, la transcription de session, y compris vos messages, les réponses de Claude et l'activité des outils, est stockée sur les serveurs Anthropic. La transcription stockée maintient la conversation synchronisée sur vos appareils et permet à la session de se reconnecter après une interruption réseau. L'exécution et l'accès au système de fichiers restent sur votre machine, et les transcriptions stockées sont conservées selon la politique de [Utilisation des données](/docs/fr/data-usage).

Pour désactiver complètement Remote Control, utilisez le paramètre [`disableRemoteControl`](/docs/fr/settings#available-settings). Les organisations ayant des exigences de conformité telles que Zero Data Retention ne peuvent pas activer Remote Control.

<h2 id="trusted-devices">
  Appareils de confiance
</h2>

<Note>
  Trusted Devices est actuellement en version bêta. Les fonctionnalités et les capacités peuvent évoluer à mesure que l'expérience est affinée.

  Trusted Devices est disponible sur les plans Team et Enterprise. Il est désactivé par défaut jusqu'à ce qu'un administrateur l'active.
</Note>

Trusted Devices est un paramètre à l'échelle de l'organisation qui exige que les membres vérifient leur appareil avant de pouvoir afficher ou contrôler les sessions Remote Control depuis claude.ai, les applications Claude mobiles ou Claude Desktop. Il lie l'accès à Remote Control à un appareil connu et à une authentification récente, pas seulement à un compte connecté.

Lorsque le paramètre est activé, l'interaction avec une session Remote Control nécessite les deux éléments suivants :

* **Un appareil inscrit** : chaque navigateur, téléphone ou application de bureau qu'un membre utilise pour Remote Control enregistre sa propre accréditation. L'inscription n'est proposée que peu de temps après une connexion complète, de sorte qu'un appareil rejoint la liste de confiance dans le cadre d'une authentification réelle plutôt que silencieusement en arrière-plan.
* **Une connexion récente** : la connexion du membre ne doit pas dépasser 18 heures. Au lieu de se connecter à nouveau chaque jour, les membres confirment leur présence avec Face ID, Touch ID, Windows Hello ou une clé d'accès. Cette étape biométrique actualise la session immédiatement.

Les vérifications biométriques s'exécutent sur l'appareil via le système d'exploitation ou le navigateur, le même mécanisme que la connexion par clé d'accès. Anthropic ne reçoit ni ne stocke jamais les empreintes digitales, les données faciales ou toute autre information biométrique. Seule la clé publique de l'appareil et les métadonnées de base telles que le nom d'affichage, la plateforme et l'heure d'inscription sont stockées.

Le paramètre s'applique uniquement à Remote Control. Le chat Claude régulier, Claude Code dans le terminal et l'utilisation de l'API ne sont pas affectés.

<h3 id="enable-trusted-devices-for-your-organization">
  Activer Trusted Devices pour votre organisation
</h3>

Les administrateurs activent le paramètre à partir de la console d'administration Claude Code.

<Steps>
  <Step title="Ouvrir les paramètres d'administration Claude Code">
    Allez à [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code). Le bouton **Require trusted devices** apparaît sous le paramètre Remote Control.
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

<h2 id="remote-control-vs-claude-code-on-the-web">
  Remote Control vs Claude Code sur le web
</h2>

Remote Control et [Claude Code sur le web](/docs/fr/claude-code-on-the-web) utilisent tous deux l'interface claude.ai/code. La différence clé est l'endroit où la session s'exécute : Remote Control s'exécute sur votre machine, donc vos serveurs MCP locaux, outils et configuration de projet restent disponibles. Claude Code sur le web s'exécute dans l'infrastructure cloud gérée par Anthropic.

Utilisez Remote Control lorsque vous êtes au milieu d'un travail local et que vous souhaitez continuer depuis un autre appareil. Utilisez Claude Code sur le web lorsque vous souhaitez lancer une tâche sans aucune configuration locale, travailler sur un référentiel que vous n'avez pas cloné, ou exécuter plusieurs tâches en parallèle.

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
* **Le processus local doit continuer à s'exécuter** : Remote Control s'exécute en tant que processus local. Si vous fermez le terminal, quittez VS Code, ou arrêtez autrement le processus `claude`, la session se termine.
* **Panne réseau prolongée** : si votre machine est allumée mais incapable d'atteindre le réseau pendant plus de dix minutes environ, la session expire et le processus se termine. Exécutez `claude remote-control` à nouveau pour démarrer une nouvelle session.
* **Ultraplan déconnecte Remote Control** : le démarrage d'une session [ultraplan](/docs/fr/ultraplan) déconnecte toute session Remote Control active car les deux fonctionnalités occupent l'interface claude.ai/code et une seule peut être connectée à la fois.
* **Certaines commandes sont locales uniquement** : les commandes qui s'exécutent uniquement dans l'interface du terminal, telles que `/plugin` ou `/resume`, fonctionnent uniquement à partir de la CLI locale, que vous transmettiez un argument ou non. Les commandes suivantes fonctionnent à partir du mobile et du web :
  * Commandes de sortie textuelle : `/compact`, `/clear`, `/context`, `/usage`, `/exit`, `/usage-credits` (exécute le formulaire textuel au lieu d'ouvrir la boîte de dialogue dans la CLI), `/recap`, `/reload-plugins`
  * `/model`, `/effort`, `/fast`, `/color`, et `/rename` : transmettez la valeur en tant qu'argument, par exemple `/model sonnet` ou `/effort high`. À partir du mobile et du web, `/model` et `/effort` prennent l'argument à la place du sélecteur du terminal ou du curseur.
  * `/mcp`, à partir de la v2.1.166 : à partir de l'application mobile, retourne un résumé textuel de l'état du serveur au lieu d'ouvrir le sélecteur. Sur le web, `/mcp` seul ouvre un répertoire des [connecteurs claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai) au lieu de retourner le résumé. Les [sous-commandes](/docs/fr/commands#all-commands) `reconnect`, `enable`, et `disable` fonctionnent à partir des deux. Contrairement à la CLI locale, `/mcp reconnect` sans nom de serveur reconnecte tous les serveurs qui ont échoué ou nécessitent une authentification.
  * `/config`, à partir de la v2.1.181 : à partir de l'application mobile, transmettez `key=value` pour définir un paramètre, ou exécutez-le sans argument pour lister les clés que vous pouvez définir. Sur le web, `/config` ouvre la section Claude Code de vos paramètres à la place, et ignore le texte après la commande.

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="remote-control-requires-a-claude-ai-subscription">
  « Remote Control requires a claude.ai subscription »
</h3>

Vous n'êtes pas authentifié avec un compte claude.ai. Exécutez `claude auth login` et choisissez l'option claude.ai. Si `ANTHROPIC_API_KEY` est défini dans votre environnement, désactivez-le d'abord.

Avant la v2.1.206, l'exécution de `/remote-control` alors que vous étiez déconnecté signalait `Unknown command: /remote-control` au lieu de ce message.

<h3 id="remote-control-requires-a-full-scope-login-token">
  « Remote Control requires a full-scope login token »
</h3>

Vous êtes authentifié avec un jeton de longue durée de `claude setup-token` ou la variable d'environnement `CLAUDE_CODE_OAUTH_TOKEN`. Ces jetons sont limités à l'inférence uniquement et ne peuvent pas établir de sessions Remote Control. Exécutez `claude auth login` pour vous authentifier avec un jeton de session à portée complète à la place.

<h3 id="unable-to-determine-your-organization-for-remote-control-eligibility">
  « Unable to determine your organization for Remote Control eligibility »
</h3>

Vos informations de compte en cache sont obsolètes ou incomplètes. Exécutez `claude auth login` pour les actualiser.

<h3 id="remote-control-is-not-yet-enabled-for-your-account">
  « Remote Control is not yet enabled for your account »
</h3>

Le déploiement de Remote Control n'a pas atteint votre compte, ou vos droits en cache sont obsolètes. Si vous avez récemment changé de plan, exécutez `claude auth logout` puis `claude auth login` pour les actualiser. Exécutez `claude doctor` pour voir quel contrôle d'admissibilité individuel a échoué. Les conflits de variables d'environnement, les vérifications inaccessibles et la politique organisationnelle produisent chacun leur propre message, donc cette erreur signifie que la porte de déploiement elle-même.

<h3 id="couldn’t-verify-remote-control-eligibility">
  « Couldn't verify Remote Control eligibility »
</h3>

Claude Code n'a pas pu atteindre le service de drapeau de fonctionnalité pour vérifier si Remote Control est activé pour votre compte, généralement parce que vous êtes hors ligne ou qu'un proxy bloque la requête. Réessayez une fois que vous avez accès au réseau, ou exécutez `claude doctor` pour plus de détails. Le message associé « Couldn't verify your organization's Remote Control policy » a la même cause et le même correctif. Les deux messages ont été ajoutés dans la v2.1.178.

<h3 id="remote-control-is-only-available-when-using-claude-via-api-anthropic-com">
  « Remote Control is only available when using Claude via api.anthropic.com »
</h3>

La session ne communique pas directement avec l'API Anthropic, il n'y a donc pas de backend claude.ai pour l'associer. Cela se produit sur Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry. À partir de la v2.1.196, cela se produit également lorsque [`ANTHROPIC_BASE_URL`](/docs/fr/env-vars) pointe vers un hôte autre que `api.anthropic.com`, comme une [passerelle LLM](/docs/fr/llm-gateway) ou un proxy, même si vous vous connectez avec claude.ai. Désactivez `ANTHROPIC_BASE_URL` et redémarrez la session pour utiliser Remote Control.

<h3 id="remote-control-is-disabled-by-your-organization’s-policy">
  « Remote Control is disabled by your organization's policy »
</h3>

Cette erreur a quatre causes distinctes. Exécutez d'abord `/status` pour voir quelle méthode de connexion et quel abonnement vous utilisez.

* **Vous êtes authentifié avec une clé API ou un compte Console** : Remote Control nécessite OAuth claude.ai. Exécutez `/login` et choisissez l'option claude.ai. Si `ANTHROPIC_API_KEY` est défini dans votre environnement, désactivez-le.
* **Un propriétaire n'a pas activé cette fonctionnalité pour votre organisation** : Remote Control est désactivé par défaut sur les plans Team et Enterprise. Un propriétaire peut l'activer sur [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) en activant le bouton **Remote Control**. Ce bouton est un paramètre d'organisation côté serveur.
* **Le bouton d'administration est grisé** : votre organisation a une configuration de rétention des données ou de conformité incompatible avec Remote Control. Cela ne peut pas être modifié à partir du panneau d'administration. Contactez le support Anthropic pour discuter des options.
* **L'erreur mentionne `disableRemoteControl`** : votre administrateur informatique a désactivé Remote Control sur cet appareil via les [paramètres gérés](/docs/fr/settings#settings-files), indépendamment du bouton à l'échelle de l'organisation.

<h3 id="remote-credentials-fetch-failed">
  « Remote credentials fetch failed »
</h3>

Claude Code n'a pas pu obtenir une accréditation de courte durée auprès de l'API Anthropic pour établir la connexion. Réexécutez avec `--verbose` pour voir l'erreur complète :

```bash theme={null}
claude remote-control --verbose
```

Causes courantes :

* Non connecté : exécutez `claude` et utilisez `/login` pour vous authentifier avec votre compte claude.ai. L'authentification par clé API n'est pas prise en charge pour Remote Control.
* Problème de réseau ou de proxy : un pare-feu ou un proxy peut bloquer la requête HTTPS sortante. Remote Control nécessite l'accès à l'API Anthropic sur le port 443.
* Échec de la création de session : si vous voyez également `Session creation failed — see debug log`, l'échec s'est produit plus tôt dans la configuration. Vérifiez que votre abonnement est actif.

<h3 id="couldn’t-reconnect-to-your-remote-control-session">
  « Couldn't reconnect to your Remote Control session »
</h3>

Lorsque vous reprenez une conversation avec `claude --resume` ou `claude --continue`, Claude Code se reconnecte à la session Remote Control enregistrée dans cette conversation. Ce message signifie que la reconnexion a échoué pour une raison qui peut être temporaire, comme une interruption réseau ou une erreur serveur, donc Claude Code ne peut pas confirmer si la session distante existe toujours. Lorsque le serveur confirme que la session précédente n'existe plus, Claude Code crée une nouvelle session Remote Control sans afficher ce message.

Votre session locale continue de s'exécuter sans Remote Control. Exécutez `/remote-control` pour réessayer la connexion, ou démarrez Claude Code sans `--resume` pour créer une nouvelle session Remote Control.

Avant la v2.1.200, un échec de reconnexion créait une nouvelle session Remote Control au lieu d'afficher ce message, ce qui laissait des sessions supplémentaires dans la liste des sessions sur claude.ai/code.

<h3 id="your-organization-requires-trusted-devices-for-remote-control-but-this-device-is-not-enrolled">
  « Your organization requires Trusted Devices for Remote Control, but this device is not enrolled »
</h3>

Votre organisation a [Trusted Devices](#trusted-devices) activé et cette machine ne s'est pas encore inscrite. Exécutez `/login` dans Claude Code. L'inscription se fait dans le cadre de la connexion, et il n'y a pas de commande d'inscription séparée.

<h3 id="session-expired-for-trusted-device-check">
  « session expired for trusted-device check »
</h3>

Votre connexion a plus de 18 heures. Exécutez `/login` dans Claude Code, ou confirmez avec Face ID, Touch ID, Windows Hello ou une clé d'accès lorsque claude.ai ou l'application mobile vous le demande. Voir [Trusted Devices](#trusted-devices).

<h2 id="choose-the-right-approach">
  Choisir la bonne approche
</h2>

Claude Code offers several ways to work when you're not at your terminal. They differ in what triggers the work, where Claude runs, and how much you need to set up.

|                                                          | Trigger                                                                                        | Claude runs on                                                                               | Setup                                                                                                                                | Best for                                                      |
| :------------------------------------------------------- | :--------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------ |
| [Dispatch](/docs/en/desktop#sessions-from-dispatch)           | Message a task from the Claude mobile app                                                      | Your machine (Desktop)                                                                       | [Pair the mobile app with Desktop](https://support.claude.com/en/articles/13947068)                                                  | Delegating work while you're away, minimal setup              |
| [Remote Control](/docs/en/remote-control)                     | Drive a running session from [claude.ai/code](https://claude.ai/code) or the Claude mobile app | Your machine (CLI or VS Code)                                                                | Run `claude remote-control`                                                                                                          | Steering in-progress work from another device                 |
| [Channels](/docs/en/channels)                                 | Push events from a chat app like Telegram or Discord, or your own server                       | Your machine (CLI)                                                                           | [Install a channel plugin](/docs/en/channels#quickstart) or [build your own](/docs/en/channels-reference)                                      | Reacting to external events like CI failures or chat messages |
| [Slack](/docs/en/slack)                                       | Mention `@Claude` in a team channel                                                            | Anthropic cloud                                                                              | [Install the Slack app](/docs/en/slack#setting-up-claude-code-in-slack) with [Claude Code on the web](/docs/en/claude-code-on-the-web) enabled | PRs and reviews from team chat                                |
| [Self-hosted environments](/docs/en/self-hosted-environments) | Start a [cloud session](/docs/en/claude-code-on-the-web) and pick your organization's environment   | Your organization's infrastructure                                                           | [Deploy runners](/docs/en/self-hosted-environments-quickstart), on Team and Enterprise plans                                              | Cloud sessions that must run inside your network              |
| [Scheduled tasks](/docs/en/scheduled-tasks)                   | Set a schedule                                                                                 | [CLI](/docs/en/scheduled-tasks), [Desktop](/docs/en/desktop-scheduled-tasks), or [cloud](/docs/en/routines) | Pick a frequency                                                                                                                     | Recurring automation like daily reviews                       |

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Claude Code sur le web](/docs/fr/claude-code-on-the-web) : exécutez des sessions dans des environnements cloud gérés par Anthropic au lieu de sur votre machine
* [Ultraplan](/docs/fr/ultraplan) : lancez une session de planification cloud depuis votre terminal et examinez le plan dans votre navigateur
* [Canaux](/docs/fr/channels) : transférez Telegram, Discord ou iMessage dans une session afin que Claude réagisse aux messages pendant que vous êtes absent
* [Dispatch](/docs/fr/desktop#sessions-from-dispatch) : envoyez un message avec une tâche depuis votre téléphone et il peut générer une session Desktop pour la gérer
* [Authentification](/docs/fr/authentication) : configurez `/login` et gérez les identifiants pour claude.ai
* [Référence CLI](/docs/fr/cli-reference) : liste complète des drapeaux et commandes incluant `claude remote-control`
* [Sécurité](/docs/fr/security) : comment les sessions Remote Control s'intègrent dans le modèle de sécurité Claude Code
* [Utilisation des données](/docs/fr/data-usage) : quelles données circulent via l'API Anthropic lors des sessions locales et distantes
