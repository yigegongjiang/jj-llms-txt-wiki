> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Utiliser Claude Code dans VS Code

> Installez et configurez l'extension Claude Code pour VS Code. Obtenez une assistance de codage IA avec des diffs en ligne, des mentions @, un examen du plan et des raccourcis clavier.

<img src="https://mintcdn.com/claude-code/-YhHHmtSxwr7W8gy/images/vs-code-extension-interface.jpg?fit=max&auto=format&n=-YhHHmtSxwr7W8gy&q=85&s=300652d5678c63905e6b0ea9e50835f8" alt="Éditeur VS Code avec le panneau d'extension Claude Code ouvert sur le côté droit, montrant une conversation avec Claude" width="2500" height="1155" data-path="images/vs-code-extension-interface.jpg" />

L'extension VS Code fournit une interface graphique native pour Claude Code, intégrée directement dans votre IDE. C'est la façon recommandée d'utiliser Claude Code dans VS Code.

Avec l'extension, vous pouvez examiner et modifier les plans de Claude avant de les accepter, accepter automatiquement les modifications au fur et à mesure qu'elles sont apportées, mentionner des fichiers avec des plages de lignes spécifiques à partir de votre sélection, accéder à l'historique des conversations et ouvrir plusieurs conversations dans des onglets ou des fenêtres séparés.

<h2 id="prerequisites">
  Prérequis
</h2>

Avant d'installer, assurez-vous que vous avez :

* VS Code 1.94.0 ou supérieur
* Un compte Anthropic : tout abonnement Claude payant (Pro, Max, Team ou Enterprise) ou un compte Claude Console fonctionne, et aucune clé API n'est requise. Vous vous [connecterez](/docs/fr/authentication#log-in-to-claude-code) avec ce compte lors de la première ouverture de l'extension. Si vous accédez à Claude par l'intermédiaire d'un fournisseur tiers comme Amazon Bedrock ou Google Cloud's Agent Platform, consultez [Utiliser des fournisseurs tiers](#use-third-party-providers) pour les instructions de configuration.

<Tip>
  L'extension inclut sa propre copie du CLI (interface de ligne de commande) pour le panneau de chat. Pour exécuter `claude` dans le terminal intégré de VS Code, vous avez également besoin de l'[installation CLI autonome](/docs/fr/setup). Consultez [Extension VS Code vs. CLI Claude Code](#vs-code-extension-vs-claude-code-cli) pour plus de détails.
</Tip>

<h2 id="install-the-extension">
  Installer l'extension
</h2>

Cliquez sur le lien de votre IDE pour installer directement :

* [Installer pour VS Code](vscode:extension/anthropic.claude-code)
* [Installer pour Cursor](cursor:extension/anthropic.claude-code)

Ou dans VS Code, appuyez sur `Cmd+Shift+X` (Mac) ou `Ctrl+Shift+X` (Windows/Linux) pour ouvrir la vue Extensions, recherchez « Claude Code » et cliquez sur **Installer**.

L'extension s'installe également dans d'autres forks de VS Code comme Devin Desktop ou Kiro. Recherchez « Claude Code » dans la vue Extensions de l'éditeur, ou installez à partir du [registre Open VSX](https://open-vsx.org/extension/Anthropic/claude-code). Si votre éditeur ne peut pas installer l'extension, [installez l'interface CLI](/docs/fr/quickstart) et exécutez `claude` dans son terminal intégré à la place. L'interface CLI fonctionne dans n'importe quel terminal.

<Note>Si l'extension n'apparaît pas après l'installation, redémarrez VS Code ou exécutez « Developer: Reload Window » à partir de la Palette de commandes.</Note>

<h2 id="get-started">
  Commencer
</h2>

Une fois installé, vous pouvez commencer à utiliser Claude Code via l'interface VS Code :

<Steps>
  <Step title="Ouvrir le panneau Claude Code">
    Dans VS Code, l'icône Spark indique Claude Code : <img src="https://mintcdn.com/claude-code/c5r9_6tjPMzFdDDT/images/vs-code-spark-icon.svg?fit=max&auto=format&n=c5r9_6tjPMzFdDDT&q=85&s=3ca45e00deadec8c8f4b4f807da94505" alt="Icône Spark" style={{display: "inline", height: "0.85em", verticalAlign: "middle"}} width="16" height="16" data-path="images/vs-code-spark-icon.svg" />

    Le moyen le plus rapide d'ouvrir Claude est de cliquer sur l'icône Spark dans la **Barre d'outils de l'éditeur** (coin supérieur droit de l'éditeur). L'icône n'apparaît que lorsque vous avez un fichier ouvert.

    <img src="https://mintcdn.com/claude-code/mfM-EyoZGnQv8JTc/images/vs-code-editor-icon.png?fit=max&auto=format&n=mfM-EyoZGnQv8JTc&q=85&s=eb4540325d94664c51776dbbfec4cf02" alt="Éditeur VS Code montrant l'icône Spark dans la Barre d'outils de l'éditeur" width="2796" height="734" data-path="images/vs-code-editor-icon.png" />

    Autres façons d'ouvrir Claude Code :

    * **Barre d'activité** : cliquez sur l'icône Spark dans la barre latérale gauche pour ouvrir la liste des sessions. Cliquez sur n'importe quelle session pour l'ouvrir à votre [emplacement préféré](#extension-settings), ou démarrez-en une nouvelle. Cette icône est toujours visible dans la Barre d'activité.
    * **Palette de commandes** : `Cmd+Shift+P` (Mac) ou `Ctrl+Shift+P` (Windows/Linux), tapez « Claude Code », et sélectionnez une option comme « Open in New Tab »
    * **Barre d'état** : si vous avez défini [`preferredLocation`](#extension-settings) sur `sidebar`, ou ouvert Claude avec **Claude Code: Open in Side Bar**, cliquez sur **✻ Claude Code** dans le coin inférieur droit de la fenêtre. Cela fonctionne même quand aucun fichier n'est ouvert.

    Vous pouvez faire glisser le panneau Claude pour le repositionner n'importe où dans VS Code. Consultez [Personnaliser votre flux de travail](#customize-your-workflow) pour plus de détails.
  </Step>

  <Step title="Se connecter">
    La première fois que vous ouvrez le panneau, un écran de connexion apparaît. Cliquez sur **Sign in** et complétez l'autorisation dans votre navigateur.

    Si vous voyez **Not logged in · Please run /login** plus tard, l'extension rouvre l'écran de connexion automatiquement. Si cela n'apparaît pas, rechargez la fenêtre depuis la Palette de commandes avec **Developer: Reload Window**.

    Si vous avez `ANTHROPIC_API_KEY` défini dans votre shell mais que vous voyez toujours l'invite de connexion, VS Code n'a peut-être pas hérité de votre environnement shell. Lancez VS Code depuis un terminal avec `code .` pour qu'il hérite de vos variables d'environnement, ou connectez-vous avec votre compte Claude à la place.

    Après vous être connecté, une liste de contrôle **Learn Claude Code** apparaît. Parcourez chaque élément en cliquant sur **Show me**, ou fermez-la avec le X. Pour la rouvrir plus tard, décochez **Hide Onboarding** dans les paramètres VS Code sous Extensions → Claude Code.
  </Step>

  <Step title="Envoyer une invite">
    Demandez à Claude de vous aider avec votre code ou vos fichiers, qu'il s'agisse d'expliquer comment quelque chose fonctionne, de déboguer un problème ou de faire des modifications.

    <Tip>Claude voit automatiquement votre texte sélectionné. Appuyez sur `Option+K` (Mac) / `Alt+K` (Windows/Linux) pour insérer également une référence @-mention (comme `@file.ts#5-10`) dans votre invite.</Tip>

    Voici un exemple de question sur une ligne particulière dans un fichier :

    <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-send-prompt.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=ede3ed8d8d5f940e01c5de636d009cfd" alt="Éditeur VS Code avec les lignes 2-3 sélectionnées dans un fichier Python, et le panneau Claude Code montrant une question sur ces lignes avec une référence @-mention" width="3288" height="1876" data-path="images/vs-code-send-prompt.png" />
  </Step>

  <Step title="Examiner les modifications">
    Ce que vous voyez dépend du [mode de permission](/docs/fr/permission-modes#which-mode-a-session-starts-in) affiché au bas de la boîte d'invite :

    * En mode Auto ou Edit automatically, Claude modifie la plupart des fichiers de votre espace de travail sans demander.
    * En mode Manual, quand Claude veut modifier un fichier, il affiche une comparaison côte à côte de l'original et des modifications proposées, puis demande la permission. Vous pouvez accepter, rejeter, ou dire à Claude quoi faire à la place. Si vous modifiez directement le contenu proposé dans la vue diff avant d'accepter, Claude est informé que vous l'avez modifié pour qu'il ne suppose pas que le fichier correspond à sa proposition originale.

          <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-edits.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=e005f9b41c541c5c7c59c082f7c4841c" alt="VS Code montrant un diff des modifications proposées par Claude avec une invite de permission demandant si vous souhaitez faire la modification" width="3292" height="1876" data-path="images/vs-code-edits.png" />

    Pour examiner une modification proposée une à la fois, utilisez les boutons **Accept this change** et **Reject this change** sous chaque modification dans le diff. Rejeter une modification l'annule dans le contenu proposé ; accepter la marque comme examinée. Accepter ou rejeter le fichier entier termine toujours l'examen. Un diff avec plus de 100 modifications s'ouvre sans les boutons par modification, donc examinez-le comme un fichier entier. L'examen par modification nécessite Claude Code v2.1.275 ou ultérieur.

    Les mêmes actions sont disponibles au curseur depuis le menu contextuel de l'éditeur et depuis la Palette de commandes en tant que **Claude Code: Accept Change at Cursor** et **Claude Code: Reject Change at Cursor**.
  </Step>
</Steps>

Pour plus d'idées sur ce que vous pouvez faire avec Claude Code, consultez [Flux de travail courants](/docs/fr/common-workflows).

<Tip>
  Exécutez « Claude Code: Open Walkthrough » depuis la Palette de commandes pour une visite guidée des bases.
</Tip>

<h2 id="use-the-prompt-box">
  Utiliser la boîte de saisie
</h2>

La boîte de saisie prend en charge plusieurs fonctionnalités :

* **Modes de permission** : cliquez sur l'indicateur de mode en bas de la boîte de saisie pour basculer entre les modes de permission. Sur les plans Pro, Max et Team, Auto est le mode de permission intégré au démarrage. Consultez [comment l'extension choisit le mode de permission au démarrage](/docs/fr/permission-modes#switch-permission-modes) pour savoir ce qui change cela, et tous les modes de permission que l'indicateur propose.
  * **Auto** : un classificateur examine la plupart des actions au lieu de vous demander. Consultez [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) pour savoir ce qu'il examine et bloque.
  * **Manual** : Claude demande la permission avant les modifications de fichiers et la plupart des commandes shell.
  * **Plan** : Claude décrit ce qu'il fera et attend l'approbation avant d'apporter des modifications. VS Code ouvre automatiquement le plan en tant que document Markdown complet où vous pouvez ajouter des commentaires en ligne pour donner votre avis avant que Claude ne commence.

    Vous pouvez également taper `/plan` dans la boîte de saisie. Nécessite Claude Code v2.1.280 ou version ultérieure.

    * `/plan` : bascule vers le mode plan. Si vous êtes déjà en mode plan, affiche le plan actuel à la place.
    * `/plan` avec une tâche, telle que `/plan fix the auth bug` : bascule vers le mode plan et commence à planifier cette tâche.
    * `/plan open` : lorsque vous êtes déjà en mode plan, ouvre le fichier de plan dans l'éditeur.
  * **Edit automatically** : Claude apporte des modifications sans demander.
* **Model** : sélectionnez **Switch model…** dans le menu de commande pour changer de modèle en cours de session. Vous pouvez également cliquer sur le nom du modèle en bas de la boîte de saisie pour ouvrir le même sélecteur.

  Lorsque le modèle actuel prend en charge les [niveaux d'effort](/docs/fr/model-config#adjust-effort-level), le sélecteur affiche également une ligne **Effort** et le bouton du nom du modèle affiche le niveau sélectionné. Lorsque vous choisissez un niveau autre que `max`, Claude Code l'enregistre pour le modèle actuel comme valeur par défaut, sous [`modelSettings`](/docs/fr/settings-reference#modelsettings) dans vos paramètres utilisateur ; `max` s'applique uniquement à la session actuelle. Le bouton du nom du modèle et la ligne **Effort** nécessitent Claude Code v2.1.257 ou version ultérieure.
* **Command menu** : cliquez sur `/` ou tapez `/` pour ouvrir le menu de commande. Les options incluent l'attachement de fichiers, le changement de modèle et l'activation de la réflexion étendue.

  La section Personnaliser fournit l'accès aux serveurs MCP, aux commandes, aux styles de sortie, aux hooks, à la mémoire, aux instructions, aux permissions et aux plugins. Les éléments avec une icône de terminal s'ouvrent dans le terminal intégré.

  * Pour parcourir les commandes telles que `/usage` ou [`/remote-control`](/docs/fr/remote-control), sélectionnez **Slash commands** dans la section Personnaliser. Une boîte de dialogue les répertorie avec une zone de filtre. Choisissez-en une pour l'exécuter. Taper `/` dans la boîte de saisie suggère toujours les commandes en ligne. Nécessite Claude Code v2.1.257 ou version ultérieure.

    Taper `/skills` ouvre également cette boîte de dialogue. Chaque ligne de [skill](/docs/fr/skills) affiche sa [visibilité](/docs/fr/skills#override-skill-visibility-from-settings), telle que **On** ou **Name only**. Cliquez sur la visibilité pour la modifier, sauf sur les lignes marquées **locked**, telles que les skills de plugin. Le raccourci `/skills` et les contrôles de visibilité nécessitent Claude Code v2.1.280 ou version ultérieure.
  * Sélectionnez **Output styles** dans la section Personnaliser pour choisir un [style de sortie](/docs/fr/output-styles), y compris vos styles personnalisés. Nécessite Claude Code v2.1.257 ou version ultérieure.

    Pour créer un style personnalisé à la place, sélectionnez **Build a custom style** dans le menu **Output styles**. Claude Code écrit le [fichier de style](/docs/fr/output-styles#create-a-custom-output-style) pour vous au niveau du projet ou de l'utilisateur. Nécessite Claude Code v2.1.261 ou version ultérieure.
  * Sélectionnez **Hooks** dans la section Personnaliser pour afficher les [hooks](/docs/fr/hooks) chargés dans la session, regroupés par événement. Vous pouvez ajouter, modifier ou supprimer les hooks enregistrés dans vos fichiers de paramètres utilisateur, projet et local. Les hooks provenant d'autres sources, telles que les paramètres gérés ou les plugins, sont en lecture seule. Nécessite Claude Code v2.1.269 ou version ultérieure.
  * Sélectionnez **Permissions** dans la section Personnaliser pour afficher les [règles de permission](/docs/fr/permissions) de la session, regroupées en Autoriser, Demander et Refuser. Vous pouvez ajouter des règles à vos paramètres utilisateur, projet ou local et supprimer les règles enregistrées là. Les règles provenant d'autres sources, telles que les paramètres gérés ou les approbations effectuées pour cette session uniquement, sont en lecture seule. Nécessite Claude Code v2.1.269 ou version ultérieure.
  * Sélectionnez **Memory** dans la section Personnaliser pour activer ou désactiver la [mémoire automatique](/docs/fr/memory#auto-memory). Pendant qu'elle est activée, vous pouvez également parcourir les mémoires que Claude a enregistrées et révéler les dossiers qui les stockent dans votre gestionnaire de fichiers. Nécessite Claude Code v2.1.274 ou version ultérieure.

    Cliquez sur une mémoire enregistrée pour la lire dans la boîte de dialogue, où vous pouvez modifier le texte, supprimer la mémoire ou ouvrir son fichier dans l'éditeur. L'affichage, la modification et la suppression d'une mémoire dans la boîte de dialogue nécessitent Claude Code v2.1.275 ou version ultérieure.
  * Sélectionnez **Instructions** dans la section Personnaliser pour modifier les [fichiers CLAUDE.md](/docs/fr/memory#claude-md-files) que Claude lit. Choisissez un fichier pour l'ouvrir dans l'éditeur. Si le fichier n'existe pas encore, Claude Code le crée d'abord. Nécessite Claude Code v2.1.274 ou version ultérieure.
  * Sélectionnez **Status** dans la section Personnaliser, ou tapez `/status`, pour vérifier la version de Claude Code, le compte, le modèle et les détails du serveur MCP de la session. Nécessite Claude Code v2.1.280 ou version ultérieure.
  * Sélectionnez **Sandbox** dans la section Personnaliser, ou tapez `/sandbox`, pour voir si les commandes Bash de Claude s'exécutent [en sandbox](/docs/fr/sandboxing). Vous pouvez basculer le mode sandbox et ajouter des [commandes exclues](/docs/fr/settings-reference#sandbox-excludedcommands) là. Nécessite Claude Code v2.1.280 ou version ultérieure.
  * Sélectionnez **Claude in Chrome** dans la section Personnaliser, ou tapez `/chrome`, pour vérifier et gérer la connexion [Claude in Chrome](/docs/fr/chrome). Les deux nécessitent de se connecter avec un compte claude.ai. Nécessite Claude Code v2.1.280 ou version ultérieure.
  * Sélectionnez **Export conversation** dans la section Contexte, ou tapez `/export`, pour copier la conversation en texte brut ou l'enregistrer dans un fichier. Ajoutez un nom de fichier, tel que `/export notes.txt`, pour ignorer la boîte de dialogue et choisir où enregistrer le fichier. Nécessite Claude Code v2.1.280 ou version ultérieure.
  * La section Paramètres inclut **Enable Remote Control for all sessions**, qui définit [`remoteControlAtStartup`](/docs/fr/settings-reference#remotecontrolatstartup) pour contrôler si les [nouvelles sessions interactives se connectent à Remote Control automatiquement](/docs/fr/remote-control#enable-remote-control-for-all-sessions). Nécessite Claude Code v2.1.203 ou version ultérieure.

    Lorsque vous activez ou désactivez le bouton bascule dans une fenêtre VS Code, la modification s'applique aux sessions déjà ouvertes dans cette fenêtre VS Code, pas seulement aux sessions que vous démarrez par la suite. Si vous le désactivez, les sessions ouvertes se déconnectent. Avec Claude Code v2.1.261 ou version ultérieure, la modification atteint également les sessions ouvertes dans vos autres fenêtres VS Code.
  * La section Paramètres inclut également **Focus view**, qui masque les appels d'outils, les résultats d'outils et la réflexion derrière des lignes extensibles, laissant vos invites et les réponses de Claude. Basculez-le là, avec `Ctrl+Option+F` (Mac) / `Ctrl+Alt+F` (Windows/Linux), ou depuis la Palette de commandes avec **Claude Code: Toggle Focus view**. La modification s'applique à chaque session ouverte et persiste entre les sessions. Nécessite Claude Code v2.1.221 ou version ultérieure.

    La dernière liste de tâches de Claude reste visible, tout comme le texte d'une question en attente que Claude pose ; cela nécessite Claude Code v2.1.225 ou version ultérieure. Pendant que Claude exécute les [sous-agents](/docs/fr/sub-agents), les lignes de progression en direct avec leur dernière activité apparaissent sous le groupe d'appels d'outils qui les a démarrés. Cela nécessite Claude Code v2.1.269 ou version ultérieure.
  * Pour vous déconnecter de votre compte Anthropic, sélectionnez **Sign out** dans la section Paramètres, ou tapez `/logout`. Sur un [fournisseur tiers](#use-third-party-providers), le menu n'offre ni l'un ni l'autre. Nécessite Claude Code v2.1.277 ou version ultérieure.
  * Pour signaler un bogue, cliquez sur **Report a problem** en bas du menu, ou tapez `/bug` ou `/feedback` avec une description facultative qui préremplira le rapport. Lorsque vous soumettez le rapport et que vous êtes connecté à Anthropic sur une connexion propriétaire, Claude Code l'envoie à Anthropic. Sur un fournisseur tiers, ou sans identifiants Anthropic, la boîte de dialogue s'ouvre toujours, mais la soumission affiche une erreur et n'envoie rien : contrairement au `/bug` de la CLI, l'extension n'écrit pas d'archive locale. Nécessite Claude Code v2.1.229 ou version ultérieure.

    Si la politique de votre organisation désactive les commentaires sur les produits, **Report a problem** n'apparaît pas dans le menu, et `/bug` et `/feedback` affichent un avis `Feedback is turned off by your organization's policy or this environment's settings.` au lieu d'ouvrir le rapport.
* **Side questions** : tapez `/btw` suivi d'une question pour poser une question sur votre session [sans l'ajouter à la conversation](/docs/fr/interactive-mode#side-questions-with-%2Fbtw). La réponse s'ouvre dans un panneau à côté du chat, où vous pouvez poser des questions de suivi. Le fil survit aux rechargements de fenêtre. Claude Code conserve les 20 derniers échanges et expire les fils stockés selon le calendrier [`cleanupPeriodDays`](/docs/fr/settings-reference#cleanupperioddays), tant que Claude Code peut [déterminer en toute sécurité la période de rétention](/docs/fr/claude-directory#cleaned-up-automatically). Pour effacer un fil, cliquez sur l'icône de corbeille dans le panneau. Nécessite Claude Code v2.1.227 ou version ultérieure.
* **Copy a response** : survolez une réponse et cliquez sur **Copy response** pour la copier dans votre presse-papiers, ou tapez `/copy` pour copier la dernière réponse. `/copy 2` copie l'avant-dernière. Nécessite Claude Code v2.1.277 ou version ultérieure.
* **Context indicator** : la boîte de saisie affiche la quantité de fenêtre de contexte de Claude que vous utilisez. Claude se compacte automatiquement si nécessaire, ou vous pouvez exécuter `/compact` manuellement.
* **Prompt cache clock** : une icône d'horloge à côté de l'indicateur de contexte estime le temps qu'il reste au [cache de prompt](/docs/fr/prompt-caching) de la conversation avant son expiration. Il compte à rebours à partir de la [durée de vie](/docs/fr/prompt-caching#cache-lifetime) du cache de cinq minutes ou une heure, et chaque réponse qui utilise le cache redémarre le compte à rebours. Outre la compaction, les [actions qui invalident le cache](/docs/fr/prompt-caching#actions-that-invalidate-the-cache) ne réinitialisent pas l'horloge, elle peut donc toujours afficher des minutes restantes après que vous ayez changé de modèle.
  * Jusqu'à ce que le compte à rebours s'écoule, l'icône affiche les minutes restantes, telles que **12m**.
  * Lorsque le compte à rebours s'écoule, les minutes disparaissent et l'icône devient rouge, ou la couleur d'erreur de votre thème, jusqu'à la réponse suivante. Le cache a probablement expiré, attendez-vous donc à une réponse plus lente et plus coûteuse à votre prochain message pendant que le cache se reconstruit. Si la durée de vie de cinq minutes continue de s'écouler entre vos messages, consultez [Choisir le TTL vous-même](/docs/fr/prompt-caching#choose-the-ttl-yourself).
  * Juste après que la conversation soit [compactée](/docs/fr/prompt-caching#compacting-the-conversation), l'icône devient également rouge sans minutes jusqu'à la réponse suivante, car le cache ne couvre pas encore la conversation compactée.
* **Agent map** : lorsque la conversation inclut des [sous-agents](/docs/fr/sub-agents), un nombre d'agents tel que **2 agents** apparaît en bas de la boîte de saisie. Son point indique si un sous-agent travaille ou attend votre permission.

  Cliquez sur le nombre d'agents pour ouvrir la carte des agents, qui dessine les sous-agents de la conversation sous forme d'arbre sous l'agent principal, chacun avec son statut, son temps écoulé et son nombre de jetons. Cliquez sur un sous-agent pour voir son invite et ses appels d'outils, ouvrir sa transcription en lecture seule, ou l'arrêter pendant qu'il s'exécute. Nécessite Claude Code v2.1.269 ou version ultérieure.

  La carte répertorie également les autres [tâches en arrière-plan](/docs/fr/tools-reference#background-commands) de la session, telles que les commandes shell en arrière-plan et les [moniteurs](/docs/fr/tools-reference#monitor-tool), sous les agents. Cliquez sur une ligne pour ouvrir la carte de la tâche et l'arrêter là.

  Pour ouvrir la carte lorsqu'aucun nombre d'agents n'est affiché, par exemple lorsque Claude a démarré un shell en arrière-plan mais pas de sous-agents, tapez `/tasks` dans la boîte de saisie. Les tâches en arrière-plan dans la carte et le `/tasks` tapé nécessitent Claude Code v2.1.277 ou version ultérieure.
* **Extended thinking** : permet à Claude de consacrer plus de temps à raisonner sur des problèmes complexes. Activez-le via le menu de commande (`/`). Le raisonnement de Claude apparaît dans la conversation sous forme de blocs réduits : cliquez sur un bloc pour le lire, ou appuyez sur `Ctrl+O` pour développer ou réduire chaque bloc de réflexion dans la session. Consultez [Extended thinking](/docs/fr/model-config#extended-thinking) pour plus de détails.
* **Multi-line input** : appuyez sur `Shift+Enter` pour ajouter une nouvelle ligne sans envoyer. Cela fonctionne également dans l'entrée en texte libre « Autre » des boîtes de dialogue de question.

<h3 id="reference-files-and-folders">
  Fichiers et dossiers de référence
</h3>

Utilisez les mentions @-mention pour donner à Claude le contexte sur des fichiers ou dossiers spécifiques. Lorsque vous tapez `@` suivi d'un nom de fichier ou de dossier, Claude lit ce contenu et peut répondre à des questions à ce sujet ou y apporter des modifications. Claude Code prend en charge la correspondance floue, vous pouvez donc taper des noms partiels pour trouver ce dont vous avez besoin :

```text wrap theme={null}
Explain the logic in @auth (fuzzy matches auth.js, AuthService.ts, etc.)
What's in @src/components/ (include a trailing slash for folders)
```

Pour les grands PDF, vous pouvez demander à Claude de lire des pages spécifiques au lieu du fichier entier : une seule page, une plage comme les pages 1-10, ou une plage ouverte comme la page 3 et au-delà.

Lorsque vous sélectionnez du texte dans l'éditeur, Claude peut voir votre code en surbrillance automatiquement. Le pied de page de la boîte de saisie affiche le nombre de lignes sélectionnées. Appuyez sur `Option+K` (Mac) / `Alt+K` (Windows/Linux) pour insérer une mention @-mention avec le chemin du fichier et les numéros de ligne (par exemple, `@app.ts#5-10`). Cliquez sur le **X** sur l'indicateur de sélection pour le supprimer afin que Claude ne reçoive pas la sélection. L'indicateur réapparaît lorsque vous sélectionnez un autre texte.

L'extension retient le texte sélectionné de certains fichiers. Lorsque le fichier se trouve dans votre espace de travail et correspond à vos paramètres `files.exclude` ou `search.exclude`, Claude reçoit au maximum le chemin du fichier et non le texte que vous avez sélectionné. Il en va de même pour un fichier ignoré par git, tant que le paramètre `search.useIgnoreFiles` de VS Code et le paramètre [`respectGitIgnore`](#extension-settings) de l'extension sont tous deux activés, ce qui est la valeur par défaut. Ce filtre couvre uniquement le panneau de chat : lorsque Claude Code s'exécute dans le terminal intégré, la CLI envoie votre texte sélectionné quel que soit le fichier, donc ajoutez une [règle de refus `Read`](#the-built-in-ide-mcp-server) pour empêcher que le contenu d'un fichier ne soit envoyé à Claude là.

Claude voit également quel fichier vous avez ouvert dans l'éditeur, même si rien n'est sélectionné, et la boîte de saisie affiche son nom. Pour ajouter uniquement votre texte sélectionné, désactivez le [paramètre Attach Open File](vscode://settings/claudeCode.attachOpenFile). Le paramètre nécessite Claude Code v2.1.271 ou version ultérieure.

Vous pouvez également joindre des images et des fichiers à votre message :

* Pour joindre une image, collez-la depuis votre presse-papiers dans la boîte de saisie.
* Pour joindre des fichiers, maintenez `Shift` enfoncé tout en faisant glisser des fichiers dans la boîte de saisie.
* Pour supprimer une pièce jointe du contexte, cliquez sur le X dessus.

<h3 id="paste-text">
  Coller du texte
</h3>

Le texte que vous collez reste visible dans la boîte de saisie, plutôt que de se réduire à un espace réservé comme il le fait [dans le terminal](/docs/fr/terminal-config#paste-large-content). Dans les sessions où Claude Code [marque le texte collé](/docs/fr/terminal-config#how-claude-treats-pasted-text), Claude voit toujours un grand collage comme du texte que vous avez collé plutôt que tapé.

Claude Code supprime également les [caractères Unicode invisibles](/docs/fr/interactive-mode#invisible-characters-in-prompts) du texte que vous collez dans la boîte de saisie et de tout ce que vous envoyez d'autre :

* Si un avis tel que `Removed 3 invisible characters from the pasted text` apparaît lorsque vous collez, le texte est entré sans ces caractères.
* Si un avis sur les caractères supprimés apparaît lorsque vous envoyez, rien n'a été envoyé. Le texte nettoyé est de retour dans la boîte de saisie. Envoyez à nouveau pour envoyer le texte tel qu'affiché.

<h3 id="resume-past-conversations">
  Reprendre les conversations passées
</h3>

Cliquez sur le bouton **Session history** en haut du panneau Claude Code pour accéder à votre historique de conversation. Vous pouvez rechercher par mot-clé ou parcourir par heure.

Cliquez sur n'importe quelle conversation pour la reprendre avec l'historique complet des messages. Si la conversation est déjà ouverte dans un autre onglet de la fenêtre actuelle, cliquer dessus bascule vers cet onglet. Pour plus d'informations sur la reprise des sessions, consultez [Manage sessions](/docs/fr/sessions).

* **Session titles** : les nouvelles sessions reçoivent des titres générés par l'IA en fonction de votre premier message.
* **Rename and archive** : survolez une session pour révéler ces actions. Renommez-la pour lui donner un titre descriptif, ou archivez-la pour la déplacer vers le groupe **Archived sessions** en bas de la liste.

Par défaut, une session sans activité pendant 14 jours se déplace vers **Archived sessions** automatiquement, sauf si elle est ouverte, non lue ou dans un [groupe](#organize-sessions-into-groups). L'archivage automatique nécessite Claude Code v2.1.265 ou version ultérieure. Pour modifier la période ou la désactiver, ouvrez le [paramètre Archive Inactive Sessions](vscode://settings/claudeCode.archiveInactiveSessions) et sélectionnez un nombre de jours ou **Never**.

Pour restaurer une session archivée, développez **Archived sessions** et cliquez sur **Unarchive session**. Pour restaurer chaque session archivée à la fois, survolez l'en-tête **Archived sessions** dans la liste des sessions dans la barre d'activité et cliquez sur son icône de désarchivage, ce qui nécessite Claude Code v2.1.277 ou version ultérieure. Avant v2.1.257, l'action était **Delete session**, qui masquait une session sans aucun moyen de la restaurer. Les sessions que vous avez supprimées apparaissent alors sous **Archived sessions** après la mise à niveau.

Lorsque la conversation que vous reprenez s'est terminée en mode plan, Claude Code restaure le mode plan. Nécessite Claude Code v2.1.246 ou version ultérieure. Claude Code ne le restaure pas dans deux cas :

* L'extension [choisit le mode de permission au démarrage](/docs/fr/permission-modes#switch-permission-modes) à partir de `claudeCode.initialPermissionMode` ou d'un choix qui se reporte d'une conversation antérieure
* Vous avez `claudeCode.claudeProcessWrapper` configuré

<h3 id="resume-cloud-sessions-from-claude-ai">
  Reprendre les sessions cloud depuis Claude.ai
</h3>

Si vous exécutez des [sessions cloud](/docs/fr/claude-code-on-the-web), vous pouvez les reprendre directement dans VS Code. Cela nécessite de se connecter avec **Claude.ai Subscription**, pas Anthropic Console.

<Steps>
  <Step title="Open session history">
    Cliquez sur le bouton **Session history** en haut du panneau Claude Code.
  </Step>

  <Step title="Select the Web tab">
    La boîte de dialogue affiche deux onglets : Local et Web. Cliquez sur **Web** pour voir les sessions de claude.ai.
  </Step>

  <Step title="Select a session to resume">
    Parcourez ou recherchez vos sessions cloud. Cliquez sur n'importe quelle session pour la télécharger et continuer la conversation localement.
  </Step>
</Steps>

<Note>
  Seules les sessions cloud démarrées avec un référentiel GitHub apparaissent dans l'onglet Web. La reprise charge l'historique de la conversation localement ; les modifications ne sont pas resynchronisées vers claude.ai.
</Note>

<h3 id="check-account-and-usage">
  Vérifier le compte et l'utilisation
</h3>

Exécutez `/usage` pour ouvrir la boîte de dialogue Compte et utilisation. Elle affiche votre compte connecté, et l'utilisation qu'elle signale diffère selon la connexion :

* **claude.ai plan** : barres d'utilisation pour les limites de votre plan, telles que la session actuelle et la semaine. Chaque barre affiche le temps restant avant la réinitialisation de sa limite.

  La boîte de dialogue détaille également ce qui contribue à vos limites de plan. Elle signale les comportements qui représentent 10 % ou plus de l'utilisation récente, tels que les défauts de cache, le contexte long et les sessions lourdes en sous-agents ou hautement parallèles, chacun avec un conseil pour le réduire. Les tableaux d'attribution affichent la quantité d'utilisation provenant de chaque compétence, sous-agent, plugin et serveur MCP.

  Utilisez le bouton bascule Jour et Semaine pour basculer entre les 24 dernières heures et les 7 derniers jours. Les chiffres sont approximatifs et calculés à partir des sessions locales sur cette machine, donc l'utilisation d'autres appareils ou de claude.ai n'est pas incluse.
* **Other sign-ins** : lorsque les limites de plan ne s'appliquent pas à votre connexion, par exemple sur un [fournisseur tiers](#use-third-party-providers) ou avec une clé API, la section Utilisation affiche le coût et l'utilisation des jetons de la session elle-même à la place. Le `/usage` de la CLI affiche les mêmes totaux dans son [bloc Session](/docs/fr/costs#track-your-costs). La liste des sessions dans la barre d'activité affiche également les totaux de la session active sous son en-tête **Account & usage**. Nécessite Claude Code v2.1.277 ou version ultérieure.

Pour plus d'informations sur le suivi et la réduction de l'utilisation, consultez [Track your costs](/docs/fr/costs#track-your-costs).

<h2 id="customize-your-workflow">
  Personnalisez votre flux de travail
</h2>

Vous pouvez repositionner le panneau Claude, exécuter plusieurs conversations, organiser la liste des sessions en groupes ou basculer en mode terminal.

<h3 id="choose-where-claude-lives">
  Choisissez où Claude se trouve
</h3>

Vous pouvez faire glisser le panneau Claude pour le repositionner n'importe où dans VS Code. Saisissez l'onglet ou la barre de titre du panneau et faites-le glisser vers :

* **Barre latérale secondaire** : le côté droit de la fenêtre. Garde Claude visible pendant que vous codez.
* **Barre latérale principale** : la barre latérale gauche avec les icônes pour l'Explorateur, la Recherche, etc.
* **Zone d'éditeur** : ouvre Claude sous forme d'onglet à côté de vos fichiers. Utile pour les tâches secondaires.

Lorsque Claude ouvre un onglet dans un nouveau groupe d'éditeur, l'extension verrouille ce groupe, de sorte que les fichiers que vous ouvrez pendant que l'onglet Claude est actif vont dans un autre groupe au lieu de s'afficher à côté de celui-ci.

Pour empêcher l'extension de verrouiller les groupes, désactivez le [paramètre Lock Editor Groups](vscode://settings/claudeCode.lockEditorGroups). Les groupes déjà verrouillés restent verrouillés jusqu'à ce que vous les déverrouilliez. Le paramètre nécessite Claude Code v2.1.274 ou version ultérieure.

<Tip>
  Utilisez la barre latérale pour votre session Claude principale et ouvrez des onglets supplémentaires pour les tâches secondaires. Claude se souvient de votre emplacement préféré. L'icône de la liste des sessions dans la barre d'activité est séparée du panneau Claude : la liste des sessions est toujours visible dans la barre d'activité, tandis que l'icône du panneau Claude n'y apparaît que lorsque le panneau est ancré à la barre latérale gauche.
</Tip>

Après avoir exécuté **Developer: Reload Window** ou redémarré VS Code, le retour d'une conversation avec son historique dépend de l'endroit où elle était ouverte :

* **Onglet d'éditeur** : la conversation revient avec son onglet.
* **Barre latérale** : la conversation revient si vous avez envoyé un message ou si Claude a répondu dans celle-ci au cours des 10 dernières minutes. Si elle ne revient pas, reprenez la conversation à partir de [Historique des sessions](#resume-past-conversations).

Si le rechargement a interrompu Claude au milieu d'une étape, Claude continue cette étape lorsque la conversation revient, et un avis dans le chat marque la continuation. Nécessite Claude Code v2.1.274 ou version ultérieure. Si l'étape a été interrompue il y a plus d'une heure ou si la session est ouverte ailleurs, la conversation revient inactive à la place.

Pour désactiver la continuation, ouvrez le [paramètre Continue After Reload](vscode://settings/claudeCode.continueAfterReload) et décochez-le.

<h3 id="run-multiple-conversations">
  Exécutez plusieurs conversations
</h3>

Utilisez **Ouvrir dans un nouvel onglet** ou **Ouvrir dans une nouvelle fenêtre** depuis la Palette de commandes pour démarrer des conversations supplémentaires. Chaque conversation maintient son propre historique et contexte, ce qui vous permet de travailler sur différentes tâches en parallèle.

Lors de l'utilisation d'onglets, un petit point coloré sur l'icône d'étincelle indique l'état : bleu signifie qu'une demande de permission est en attente, orange signifie que Claude a terminé pendant que l'onglet était masqué.

<h3 id="organize-sessions-into-groups">
  Organisez les sessions en groupes
</h3>

Dans la liste des sessions de la barre d'activité, vous pouvez regrouper les sessions connexes dans des groupes nommés et réductibles. Nécessite Claude Code v2.1.229 ou version ultérieure.

* **Grouper ou dégrouper une session** : cliquez avec le bouton droit sur une session pour créer un groupe à partir de celle-ci, la déplacer dans un groupe existant ou la supprimer de son groupe. Chaque session appartient à un groupe à la fois, donc la déplacer dans un autre groupe la supprime du premier.
* **Déplacer plusieurs sessions à la fois** : `Cmd`-clic (Mac) / `Ctrl`-clic (Windows/Linux) sur chaque session, ou `Maj`-clic pour sélectionner une plage, puis cliquez avec le bouton droit sur la sélection.
* **Grouper une session à partir de son onglet** : exécutez **Claude Code : Ajouter l'onglet de session au groupe** depuis la Palette de commandes, puis choisissez ou créez un groupe. Nécessite Claude Code v2.1.257 ou version ultérieure.
* **Renommer ou supprimer un groupe** : cliquez avec le bouton droit sur un en-tête de groupe. La suppression d'un groupe supprime uniquement le groupe, et ses sessions reviennent à la liste non groupée.

L'extension enregistre les groupes par dossier d'espace de travail, ils survivent donc aux rechargements de fenêtre et apparaissent dans chaque fenêtre où vous ouvrez le même dossier. Lorsque vous recherchez dans la liste, l'extension affiche les correspondances dans une liste plate unique sur tous les groupes.

<h3 id="switch-to-terminal-mode">
  Basculez en mode terminal
</h3>

Par défaut, l'extension ouvre un panneau de chat graphique. Si vous préférez l'interface de style CLI, ouvrez le [paramètre Utiliser le terminal](vscode://settings/claudeCode.useTerminal) et cochez la case.

Vous pouvez également ouvrir les paramètres VS Code (`Cmd+,` sur Mac ou `Ctrl+,` sur Windows/Linux), accédez à Extensions → Claude Code, et cochez **Utiliser le terminal**.

<h2 id="manage-plugins">
  Gérer les plugins
</h2>

L'extension VS Code inclut une interface graphique pour installer et gérer les [plugins](/docs/fr/plugins/overview). Tapez `/plugins` dans la zone de saisie pour ouvrir l'interface **Gérer les plugins**.

<h3 id="install-plugins">
  Installer les plugins
</h3>

La boîte de dialogue des plugins affiche deux onglets : **Plugins** et **Marketplaces**.

Dans l'onglet Plugins :

* Les **plugins installés** apparaissent en haut avec des commutateurs pour les activer ou les désactiver
* Les **plugins disponibles** de vos marketplaces configurées apparaissent ci-dessous
* Recherchez pour filtrer les plugins par nom ou description
* Cliquez sur **Installer** sur n'importe quel plugin disponible

Lorsque vous installez un plugin, choisissez l'étendue de l'installation :

* **Installer pour vous** : disponible dans tous vos projets (étendue utilisateur)
* **Installer pour ce projet** : partagé avec les collaborateurs du projet (étendue du projet)
* **Installer localement** : uniquement pour vous, uniquement dans ce référentiel (étendue locale)

<h3 id="share-a-plugin-install-link">
  Partager un lien d'installation de plugin
</h3>

Pour envoyer quelqu'un directement à l'installation d'un plugin spécifique, donnez-lui l'URL `install-plugin` de l'extension. L'ouvrir lance ou met au premier plan VS Code, ouvre le panneau Claude Code, et ouvre la boîte de dialogue **Gérer les plugins** sur le choix d'étendue de ce plugin. Rien ne s'installe jusqu'à ce que la personne choisisse une étendue. Si la marketplace du plugin n'est pas encore configurée dans leur Claude Code, la boîte de dialogue leur demande d'abord de l'ajouter.

```text theme={null}
vscode://anthropic.claude-code/install-plugin?plugin=code-review&marketplace=anthropics/claude-plugins-official
```

L'URL prend deux paramètres de requête :

| Paramètre     | Description                                                                                                                                                                                             |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plugin`      | Le nom du plugin tel que sa marketplace le répertorie. Obligatoire.                                                                                                                                     |
| `marketplace` | D'où provient le plugin : un `owner/repo` GitHub, une URL `https://`, ou une URL git SSH telle que `git@github.com:owner/repo.git`. Par défaut `anthropics/claude-plugins-official` lorsqu'il est omis. |

Certaines valeurs que l'[onglet Marketplaces](#manage-marketplaces) accepte ne fonctionnent pas dans un lien, comme un chemin local ou une adresse `http://`. Pour celles-ci, VS Code affiche un message d'erreur et la boîte de dialogue ne s'ouvre pas.

Deux cas se terminent par un message dans la boîte de dialogue au lieu du choix d'étendue :

* **La marketplace ne répertorie pas de plugin avec ce nom** : la boîte de dialogue signale que le plugin n'a pas été trouvé. Vérifiez la valeur `plugin` par rapport au répertoire de la marketplace.
* **Le plugin est déjà installé** : la boîte de dialogue l'indique, et rien ne change.

Les README GitHub, les problèmes et certains autres hôtes Markdown suppriment les liens dont le schéma n'est pas `http` ou `https`, donc un lien `vscode://` y est rendu en texte brut. Mettez l'URL dans un bloc de code sur ces hôtes, comme [Le lien s'affiche en texte brut au lieu d'être cliquable](/docs/fr/deep-links#the-link-renders-as-plain-text-instead-of-being-clickable) le décrit pour les liens `claude-cli://`.

<h3 id="manage-marketplaces">
  Gérer les marketplaces
</h3>

Basculez vers l'onglet **Marketplaces** pour ajouter ou supprimer des sources de plugins :

* Entrez un référentiel GitHub, une URL ou un chemin local pour ajouter une nouvelle marketplace
* Cliquez sur l'icône d'actualisation pour mettre à jour la liste des plugins d'une marketplace
* Cliquez sur l'icône de corbeille pour supprimer une marketplace

Les modifications apportées aux plugins dans la boîte de dialogue s'appliquent immédiatement aux sessions Claude Code ouvertes dans cette fenêtre VS Code. Si la session à partir de laquelle vous avez ouvert la boîte de dialogue ne peut pas recharger ses plugins, la boîte de dialogue vous propose de réessayer ou de redémarrer Claude dans cette session.

<Note>
  La gestion des plugins dans VS Code utilise les mêmes commandes CLI en arrière-plan. Les plugins et les marketplaces que vous configurez dans l'extension sont également disponibles dans la CLI, et vice versa.
</Note>

Pour en savoir plus sur le système de plugins, consultez [Plugins](/docs/fr/plugins/overview) et [Plugin marketplaces](/docs/fr/plugins/overview).

<h2 id="automate-browser-tasks-with-chrome">
  Automatiser les tâches du navigateur avec Chrome
</h2>

Connectez Claude à votre navigateur Chrome pour tester des applications web, déboguer avec les journaux de console et automatiser les flux de travail du navigateur sans quitter VS Code. Cela nécessite l'[extension Claude in Chrome](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) version 1.0.36 ou supérieure.

Tapez `@browser` dans la zone de saisie suivie de ce que vous souhaitez que Claude fasse :

```text wrap theme={null}
@browser go to localhost:3000 and check the console for errors
```

Vous pouvez également ouvrir le menu des pièces jointes pour sélectionner des outils de navigateur spécifiques comme ouvrir un nouvel onglet ou lire le contenu de la page.

Claude ouvre de nouveaux onglets pour les tâches du navigateur et partage l'état de connexion de votre navigateur, ce qui lui permet d'accéder à n'importe quel site auquel vous êtes déjà connecté.

Pour les instructions de configuration, la liste complète des capacités et le dépannage, consultez [Utiliser Claude Code avec Chrome](/docs/fr/chrome).

<h2 id="vs-code-commands-and-shortcuts">
  Commandes et raccourcis VS Code
</h2>

Ouvrez la Palette de commandes (`Cmd+Maj+P` sur Mac ou `Ctrl+Maj+P` sur Windows/Linux) et tapez « Claude Code » pour voir toutes les commandes VS Code disponibles pour l'extension Claude Code.

Certains raccourcis dépendent du panneau qui est « actif » (recevant l'entrée au clavier). Lorsque votre curseur se trouve dans un fichier de code, l'éditeur est actif. Lorsque votre curseur se trouve dans la zone de saisie de Claude, Claude est actif. Utilisez `Cmd+Échap` / `Ctrl+Échap` pour basculer entre eux.

<Note>
  Ce sont des commandes VS Code pour contrôler l'extension. Toutes les commandes Claude Code intégrées ne sont pas disponibles dans l'extension. Consultez [Extension VS Code vs. CLI Claude Code](#vs-code-extension-vs-claude-code-cli) pour plus de détails.
</Note>

| Commande                   | Raccourci                                                | Description                                                                                                                                                                                                                                                                                                                     |
| -------------------------- | -------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Focus Input                | `Cmd+Échap` (Mac) / `Ctrl+Échap` (Windows/Linux)         | Basculer le focus entre l'éditeur et Claude                                                                                                                                                                                                                                                                                     |
| Focus last message         | -                                                        | Déplacer le focus clavier vers le message le plus récent dans la conversation, ou vers une invite de permission en attente, afin que vous puissiez lire à partir de là avec le clavier ou un lecteur d'écran. Non disponible en [mode terminal](#switch-to-terminal-mode). Nécessite Claude Code v2.1.268 ou version ultérieure |
| Open in Side Bar           | -                                                        | Ouvrir Claude dans la barre latérale                                                                                                                                                                                                                                                                                            |
| Open in Terminal           | -                                                        | Ouvrir Claude en mode terminal                                                                                                                                                                                                                                                                                                  |
| Open in New Tab            | `Cmd+Maj+Échap` (Mac) / `Ctrl+Maj+Échap` (Windows/Linux) | Ouvrir une nouvelle conversation en tant qu'onglet d'éditeur                                                                                                                                                                                                                                                                    |
| Open in New Window         | -                                                        | Ouvrir une nouvelle conversation dans une fenêtre séparée                                                                                                                                                                                                                                                                       |
| New Conversation           | `Cmd+N` (Mac) / `Ctrl+N` (Windows/Linux)                 | Démarrer une nouvelle conversation. Nécessite que Claude soit actif et que `enableNewConversationShortcut` soit défini sur `true`                                                                                                                                                                                               |
| Reopen Closed Session      | `Cmd+Maj+T` (Mac) / `Ctrl+Maj+T` (Windows/Linux)         | Rouvrir l'onglet de session Claude fermé le plus récemment. Bascule vers la réouverture normale d'éditeur fermé de VS Code lorsque le dernier onglet fermé n'était pas une session Claude. Désactiver avec `enableReopenClosedSessionShortcut`                                                                                  |
| Insert @-Mention Reference | `Option+K` (Mac) / `Alt+K` (Windows/Linux)               | Insérer une référence au fichier actuel et à la sélection (nécessite que l'éditeur soit actif)                                                                                                                                                                                                                                  |
| Accept Change at Cursor    | -                                                        | Accepter la modification au curseur lors de l'[examen d'une modification proposée](#get-started) une modification à la fois. Nécessite Claude Code v2.1.275 ou version ultérieure                                                                                                                                               |
| Reject Change at Cursor    | -                                                        | Annuler la modification au curseur lors de l'examen d'une modification proposée une modification à la fois. Nécessite Claude Code v2.1.275 ou version ultérieure                                                                                                                                                                |
| Toggle Focus view          | `Ctrl+Option+F` (Mac) / `Ctrl+Alt+F` (Windows/Linux)     | Masquer ou afficher l'activité des outils dans la conversation. Fonctionne lorsqu'un panneau Claude ou une barre latérale est visible. Nécessite Claude Code v2.1.221 ou version ultérieure                                                                                                                                     |
| Rename Session Tab         | -                                                        | Renommer la session dans l'onglet Claude actif. Nécessite Claude Code v2.1.257 ou version ultérieure                                                                                                                                                                                                                            |
| Add Session Tab to Group   | -                                                        | Ajouter la session dans l'onglet Claude actif à un [groupe de sessions](#organize-sessions-into-groups) que vous choisissez ou créez. Nécessite Claude Code v2.1.257 ou version ultérieure                                                                                                                                      |
| Mark Session as Unread     | -                                                        | Marquer la session dans l'onglet Claude actif comme non lue dans la liste des sessions. Nécessite Claude Code v2.1.257 ou version ultérieure                                                                                                                                                                                    |
| Show Logs                  | -                                                        | Afficher les journaux de débogage de l'extension                                                                                                                                                                                                                                                                                |
| Logout                     | -                                                        | Se déconnecter de votre compte Anthropic                                                                                                                                                                                                                                                                                        |

<h3 id="launch-a-vs-code-tab-from-other-tools">
  Lancer un onglet VS Code à partir d'autres outils
</h3>

L'extension enregistre un gestionnaire URI à `vscode://anthropic.claude-code/open`. Utilisez-le pour ouvrir un nouvel onglet Claude Code à partir de vos propres outils : un alias shell, un signet de navigateur ou tout script capable d'ouvrir une URL. Si VS Code n'est pas déjà en cours d'exécution, l'ouverture de l'URL le lance d'abord. Si VS Code est déjà en cours d'exécution, l'URL s'ouvre dans la fenêtre actuellement active.

Invoquez le gestionnaire avec l'ouvreur d'URL de votre système d'exploitation.

<Tabs>
  <Tab title="macOS">
    ```bash theme={null}
    open "vscode://anthropic.claude-code/open"
    ```
  </Tab>

  <Tab title="Linux">
    ```bash theme={null}
    xdg-open "vscode://anthropic.claude-code/open"
    ```

    La commande `xdg-open` provient du paquet `xdg-utils`. Si le shell signale qu'elle n'est pas trouvée, consultez [xdg-open is not found on Linux](/docs/fr/deep-links#xdg-open-is-not-found-on-linux).
  </Tab>

  <Tab title="Windows">
    Dans PowerShell :

    ```powershell theme={null}
    Start-Process "vscode://anthropic.claude-code/open"
    ```

    Dans `cmd.exe`, `start` traite son premier argument entre guillemets comme un titre de fenêtre, donc passez un titre vide avant l'URL :

    ```cmd theme={null}
    start "" "vscode://anthropic.claude-code/open"
    ```
  </Tab>
</Tabs>

Le gestionnaire accepte deux paramètres de requête optionnels :

| Paramètre | Description                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`  | Texte à pré-remplir dans la zone de saisie. Doit être codé en URL. Le prompt est pré-rempli mais non soumis automatiquement.                                                                                                                                                                                                                                                                                                                  |
| `session` | Un ID de session à reprendre au lieu de démarrer une nouvelle conversation. La session doit appartenir à l'espace de travail actuellement ouvert dans VS Code. Si la session n'est pas trouvée, une conversation nouvelle démarre à la place. Si la session est déjà ouverte dans un onglet, cet onglet est actif. Pour capturer un ID de session par programmation, consultez [Continue conversations](/docs/fr/headless#continue-conversations). |

Par exemple, pour ouvrir un onglet pré-rempli avec « review my changes » :

```text theme={null}
vscode://anthropic.claude-code/open?prompt=review%20my%20changes
```

L'extension gère également `vscode://anthropic.claude-code/install-plugin`, qui [ouvre la boîte de dialogue du plugin sur un plugin](#share-a-plugin-install-link). Pour lancer une session terminal au lieu d'un onglet VS Code, utilisez le gestionnaire `claude-cli://` de la CLI. Consultez [Launch sessions from links](/docs/fr/deep-links).

<h2 id="configure-settings">
  Configurer les paramètres
</h2>

L'extension a deux types de paramètres :

* **Paramètres de l'extension** dans VS Code : contrôlent le comportement de l'extension dans VS Code. Ouvrez avec `Cmd+,` (Mac) ou `Ctrl+,` (Windows/Linux), puis allez à Extensions → Claude Code. Vous pouvez également taper `/` et sélectionner **General config…** pour ouvrir les paramètres.
* **Paramètres Claude Code** dans `~/.claude/settings.json` : partagés entre l'extension et CLI. Utilisez-le pour les commandes autorisées, les variables d'environnement, les hooks et les serveurs MCP. Sur les plans Pro, Max et Team, c'est aussi l'une des entrées du mode de permission avec lequel les conversations commencent. [Switch permission modes](/docs/fr/permission-modes#switch-permission-modes) énumère l'ordre. Voir [Settings](/docs/fr/settings) pour plus de détails.

<Tip>
  Ajoutez `"$schema": "https://json.schemastore.org/claude-code-settings.json"` à votre `settings.json` pour obtenir l'autocomplétion et la validation en ligne pour tous les paramètres disponibles directement dans VS Code.
</Tip>

<h3 id="extension-settings">
  Paramètres de l'extension
</h3>

VS Code lit `initialPermissionMode` à partir de vos paramètres utilisateur et ignore les valeurs d'espace de travail. Avant la v2.1.225, VS Code définissait par défaut le paramètre sur `default` et appliquait les valeurs d'espace de travail.

| Paramètre                           | Par défaut | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| ----------------------------------- | ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `useTerminal`                       | `false`    | Lancez Claude en mode terminal au lieu du panneau graphique                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `initialPermissionMode`             | -          | Contrôle les invites d'approbation pour les nouvelles conversations : `default`, `plan`, `acceptEdits` ou `bypassPermissions`. `manual` est un alias pour `default` et sélectionne le mode étiqueté **Manual** dans l'indicateur de mode. Lorsque vous le laissez non défini, l'extension choisit le mode de permission de démarrage comme décrit dans [Switch permission modes](/docs/fr/permission-modes#switch-permission-modes).                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `preferredLocation`                 | `panel`    | Où Claude s'ouvre : `sidebar` (droite) ou `panel` (nouvel onglet)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `lockEditorGroups`                  | `true`     | [Verrouillez les groupes d'éditeurs que Claude démarre pour ses onglets](#choose-where-claude-lives), afin que les fichiers que vous ouvrez lorsqu'un onglet Claude est actif aillent dans un autre groupe. Lorsqu'il est désactivé, l'extension ne verrouille jamais un groupe d'éditeurs. Nécessite Claude Code v2.1.274 ou ultérieur                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `autosave`                          | `true`     | Enregistrement automatique des fichiers avant que Claude les lise ou les écrive                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `attachOpenFile`                    | `true`     | Ajoutez le fichier ouvert dans l'éditeur à vos messages et affichez-le dans la zone de saisie. Lorsqu'il est désactivé, seul votre texte sélectionné est ajouté. Nécessite Claude Code v2.1.271 ou ultérieur                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `useCtrlEnterToSend`                | `false`    | Utilisez Ctrl/Cmd+Entrée au lieu d'Entrée pour envoyer les invites                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `scrollToBottomOnSend`              | `true`     | Faites défiler la conversation vers le bas lorsque vous envoyez un message. Lorsqu'il est désactivé, la conversation reste où vous l'avez laissée. Nécessite Claude Code v2.1.275 ou ultérieur                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `enableNewConversationShortcut`     | `false`    | Activez Cmd/Ctrl+N pour démarrer une nouvelle conversation                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `enableReopenClosedSessionShortcut` | `true`     | Utilisez Cmd/Ctrl+Maj+T pour rouvrir l'onglet de session Claude le plus récemment fermé. Lorsque le dernier onglet fermé n'était pas une session Claude, le raccourci exécute la commande de réouverture d'éditeur fermé normale de VS Code à la place.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `archiveInactiveSessions`           | `14`       | [Archivez une session automatiquement](#resume-past-conversations) après ce nombre de jours sans activité : `1`, `2`, `7` ou `14`. Définissez `0` pour le désactiver. Nécessite Claude Code v2.1.265 ou ultérieur                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `continueAfterReload`               | `true`     | Après un rechargement de fenêtre, Claude [continue l'étape qui a été interrompue](#choose-where-claude-lives) dans la session restaurée. Nécessite Claude Code v2.1.274 ou ultérieur                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `hideOnboarding`                    | `false`    | Masquez la liste de contrôle d'intégration (icône de chapeau de graduation)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `focusView`                         | `false`    | Masquez les appels d'outils, les résultats d'outils et la réflexion derrière des lignes extensibles, en laissant vos invites et les réponses de Claude. La liste de tâches la plus récente de Claude reste visible ; cela nécessite Claude Code v2.1.225 ou ultérieur. Vous pouvez également basculer la vue Focus à partir du menu de commande. Nécessite Claude Code v2.1.221 ou ultérieur                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `respectGitIgnore`                  | `true`     | Excluez les modèles .gitignore des recherches de fichiers et de [contexte de sélection](#reference-files-and-folders)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `usePythonEnvironment`              | `true`     | Activez l'environnement Python de l'espace de travail lors de l'exécution de Claude. Nécessite l'extension Python.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `environmentVariables`              | `[]`       | Définissez les variables d'environnement pour le processus Claude. Utilisez plutôt les paramètres Claude Code pour la configuration partagée.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `disableLoginPrompt`                | `false`    | Ignorez les invites d'authentification (pour les configurations de fournisseur tiers)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `allowDangerouslySkipPermissions`   | `false`    | Ajoute Bypass permissions au sélecteur de mode. Utilisez-le uniquement dans les sandboxes sans accès à Internet.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `claudeProcessWrapper`              | -          | Exécutable utilisé pour lancer le processus Claude. Le chemin binaire fourni est transmis en tant qu'argument lorsqu'il est présent. Définissez-le sur un binaire `claude` installé séparément si la version de l'extension n'en inclut pas un pour votre plateforme. Dans une configuration encapsulée, les conversations commencent en mode Manual sauf si vous définissez `initialPermissionMode` ou avez choisi Manual, Edit automatically ou Auto dans une conversation antérieure, car l'extension ignore les paramètres et les étapes par défaut intégrées là ; voir [Switch permission modes](/docs/fr/permission-modes#switch-permission-modes). Une erreur « Unsupported platform » à l'activation signifie qu'aucun binaire n'est fourni pour votre plateforme ; voir [which platforms have prebuilt binaries](/docs/fr/troubleshoot-install#native-binary-not-found-after-npm-install). |

<h2 id="use-a-screen-reader">
  Utiliser un lecteur d'écran
</h2>

Le panneau de chat de l'extension fonctionne avec les lecteurs d'écran. Vous n'avez rien à activer : l'extension annonce l'activité de la conversation pour chaque utilisateur, sans aucun changement visuel. Ceci est distinct du [mode lecteur d'écran](/docs/fr/accessibility) de la CLI, qui adapte l'interface du terminal.

Le support des lecteurs d'écran dans le panneau de chat nécessite Claude Code v2.1.236 ou une version ultérieure.

Pendant une conversation, l'extension annonce :

* **Les réponses de Claude** : l'extension annonce chaque réponse une seule fois, lorsqu'elle est complète, et reste silencieuse pendant que le texte s'affiche. Votre lecteur d'écran lit les blocs de code comme un résumé du nombre de lignes, lit les liens par leur étiquette, et lit les tableaux cellule par cellule ; la réponse complète reste lisible dans la transcription.
* **Les demandes de permission et les questions** : l'extension annonce une demande lorsque son invite de permission apparaît, en nommant l'outil que Claude souhaite utiliser. Elle annonce de la même manière lorsque Claude vous pose une question et lorsque Claude termine un plan et attend votre examen.
* **Les changements de statut** : l'extension annonce lorsque Claude commence à travailler, lorsque Claude est prêt pour votre entrée, et lorsque Claude Code commence à compacter la conversation.
* **Les erreurs et les invites de modèle** : l'extension annonce les erreurs dans la conversation, et annonce lorsque l'[invite de consentement des crédits d'utilisation](/docs/fr/model-config#fable-and-usage-credits) ou l'[invite de demande signalée](/docs/fr/model-config#ask-before-switching) apparaît.

Pendant que Claude travaille, votre lecteur d'écran lit une étiquette de texte à la place de l'animation du spinner de progression.

Lorsque vous rouvrez une session ou basculez vers une autre, l'extension n'annonce rien : l'historique restauré, les invites de permission en attente, et le statut en cours restent silencieux jusqu'à ce que quelque chose de nouveau se produise.

<h3 id="use-the-chat-panel-from-the-keyboard">
  Utiliser le panneau de chat à partir du clavier
</h3>

Chaque tour dans la transcription commence par un titre masqué visuellement étiqueté avec l'invite qui a démarré le tour, afin que vous puissiez naviguer entre les tours avec la navigation par titre de votre lecteur d'écran.

Au sein d'un tour, votre lecteur d'écran annonce dont le message vous êtes en train de lire au fur et à mesure que vous le parcourez :

* **Vos messages** : « Vous »
* **Les messages de Claude** : « Claude »
* **Les étapes d'outil** : « Claude » plus le nom de l'outil, comme « Claude, Bash »
* **Les blocs de réflexion** : « Claude, réflexion »

Parce que l'extension expose la transcription comme une région étiquetée, vous pouvez également déplacer le focus vers la transcription elle-même avec `Tab` et la lire à votre rythme. Pour déplacer le focus vers le message le plus récent ou une invite de permission en attente à la place, exécutez **Claude Code : Focus last message** à partir de la [Palette de commandes](#vs-code-commands-and-shortcuts).

Lorsqu'une option sur une invite de permission enregistre une règle de permission ou un accès au répertoire, son étiquette se termine en nommant l'endroit où l'approbation est enregistrée, comme « tous les projets » ou « cette session ». Avec cette option ciblée, appuyez sur la touche `Gauche` ou `Droite` pour modifier la destination, et l'extension annonce chaque destination lorsque vous la sélectionnez. Vous pouvez également cliquer sur la destination dans l'étiquette. Les touches fléchées nécessitent Claude Code v2.1.268 ou une version ultérieure.

<h2 id="vs-code-extension-vs-claude-code-cli">
  Extension VS Code vs. Claude Code CLI
</h2>

Claude Code est disponible à la fois en tant qu'extension VS Code (panneau graphique) et CLI (interface de ligne de commande dans le terminal). Certaines fonctionnalités ne sont disponibles que dans la CLI. Si vous avez besoin d'une fonctionnalité réservée à la CLI, exécutez `claude` dans le terminal intégré de VS Code. Cela nécessite l'[installation CLI autonome](/docs/fr/setup) : l'extension n'ajoute pas `claude` à votre PATH. Voir [Exécuter la CLI dans VS Code](#run-cli-in-vs-code).

| Fonctionnalité               | CLI                    | Extension VS Code                                                                                              |
| ---------------------------- | ---------------------- | -------------------------------------------------------------------------------------------------------------- |
| Commandes et skills          | [Toutes](/docs/fr/commands) | Sous-ensemble (tapez `/` pour voir les disponibles)                                                            |
| Configuration du serveur MCP | Oui                    | Oui ([ajouter et gérer les serveurs](#connect-to-external-tools-with-mcp) avec `/mcp` dans le panneau de chat) |
| Checkpoints                  | Oui                    | Oui                                                                                                            |
| Raccourci bash `!`           | Oui                    | Non                                                                                                            |
| Complément de tabulation     | Oui                    | Non                                                                                                            |

<h3 id="rewind-with-checkpoints">
  Rewind avec checkpoints
</h3>

L'extension VS Code prend en charge les checkpoints, qui suivent les modifications de fichiers de Claude et vous permettent de revenir à un état antérieur. Survolez n'importe quel message pour révéler le bouton de rewind, puis choisissez parmi trois options :

* **Fork conversation from here** : démarrer une nouvelle branche de conversation à partir de ce message tout en conservant toutes les modifications de code
* **Rewind code to here** : annuler les modifications de fichiers jusqu'à ce point de la conversation tout en conservant l'historique complet de la conversation
* **Fork conversation and rewind code** : démarrer une nouvelle branche de conversation et annuler les modifications de fichiers jusqu'à ce point

Pour plus de détails sur le fonctionnement des checkpoints et leurs limitations, voir [Checkpointing](/docs/fr/checkpointing).

<h3 id="run-cli-in-vs-code">
  Exécuter la CLI dans VS Code
</h3>

Pour utiliser la CLI tout en restant dans VS Code, ouvrez le terminal intégré (`` Ctrl+` `` sur Windows/Linux ou `` Cmd+` `` sur Mac) et exécutez `claude`. La CLI s'intègre automatiquement à votre IDE pour des fonctionnalités comme l'affichage des différences et le partage des diagnostics.

L'installation de l'extension ne met pas `claude` sur votre PATH shell. L'extension regroupe une copie privée de la CLI pour son panneau de chat, mais taper `claude` dans un terminal nécessite l'[installation CLI autonome](/docs/fr/setup). Exécutez l'installation une fois et les commandes de cette page, y compris `claude mcp add` et `claude --resume`, fonctionnent dans n'importe quel terminal. Si `claude` n'est toujours pas trouvé après l'installation, [vérifiez votre PATH](/docs/fr/troubleshoot-install#verify-your-path).

Si vous utilisez un terminal externe, exécutez `/ide` dans Claude Code pour le connecter à VS Code.

<h3 id="switch-between-extension-and-cli">
  Basculer entre l'extension et la CLI
</h3>

L'extension et la CLI partagent le même historique de conversation. Pour continuer une conversation d'extension dans la CLI, exécutez `claude --resume` dans le terminal. Cela ouvre un sélecteur interactif où vous pouvez rechercher et sélectionner votre conversation.

<h3 id="include-terminal-output-in-prompts">
  Inclure la sortie du terminal dans les invites
</h3>

Référencez la sortie du terminal dans vos invites en utilisant `@terminal:name` où `name` est le titre du terminal. Cela permet à Claude de voir la sortie des commandes, les messages d'erreur ou les journaux sans copier-coller.

<h3 id="monitor-background-processes">
  Surveiller les processus en arrière-plan
</h3>

Tapez `/tasks` dans la zone de saisie d'invite pour ouvrir la [carte des agents](#use-the-prompt-box), qui répertorie les tâches en arrière-plan de la session, comme un serveur de développement que Claude a laissé s'exécuter en tant que commande shell en arrière-plan. Cliquez sur une tâche pour ouvrir sa carte et l'arrêter là. Nécessite Claude Code v2.1.277 ou version ultérieure.

<h3 id="connect-to-external-tools-with-mcp">
  Connecter à des outils externes avec MCP
</h3>

MCP (Model Context Protocol) les serveurs donnent à Claude accès à des outils externes, des bases de données et des API.

Pour gérer les serveurs MCP sans quitter VS Code, tapez `/mcp` dans le panneau de chat. À partir de la boîte de dialogue qui s'ouvre, vous pouvez ajouter des serveurs, supprimer les serveurs enregistrés au niveau local, utilisateur ou projet [scope](/docs/fr/mcp#mcp-installation-scopes), activer ou désactiver les serveurs, vous reconnecter à un serveur et gérer l'authentification OAuth. L'ajout et la suppression de serveurs dans la boîte de dialogue nécessitent Claude Code v2.1.261 ou version ultérieure.

Vous pouvez également exécuter `claude mcp add` dans le terminal intégré de VS Code (`` Ctrl+` `` ou `` Cmd+` ``). La boîte de dialogue et la commande du terminal enregistrent dans la même configuration MCP, et les modifications de l'une ou l'autre prennent effet dans les conversations que vous démarrez par la suite. L'exemple ci-dessous ajoute le serveur MCP distant de GitHub, qui s'authentifie avec un [jeton d'accès personnel](https://github.com/settings/personal-access-tokens) transmis en tant qu'en-tête :

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

Remplacez `YOUR_GITHUB_PAT` par votre jeton d'accès personnel. La commande `claude mcp add` enregistre la configuration sans valider les identifiants, donc une valeur d'espace réservé est acceptée ici mais le serveur ne se connecte pas plus tard. Pour vérifier la connexion, démarrez une nouvelle conversation, tapez `/mcp` et vérifiez que le serveur affiche **Connected**. Un serveur avec de mauvais identifiants affiche **Failed**.

Une fois configuré, demandez à Claude d'utiliser les outils (par exemple, « Review PR #456 »).

Pour trouver les serveurs à connecter, voir [Trouver et créer des serveurs MCP](/docs/fr/mcp#find-and-build-mcp-servers).

<h2 id="work-with-git">
  Travailler avec git
</h2>

Claude Code s'intègre à git pour vous aider avec les workflows de contrôle de version directement dans VS Code. Demandez à Claude de valider les modifications, de créer des demandes de fusion ou de travailler sur plusieurs branches. Pour démarrer Claude dans une arborescence de travail isolée avec ses propres fichiers et branche, consultez [Exécuter des sessions parallèles avec worktrees](/docs/fr/worktrees).

<h3 id="create-commits-and-pull-requests">
  Créer des commits et des demandes de fusion
</h3>

Claude peut indexer les modifications, rédiger des messages de commit et créer des demandes de fusion en fonction de votre travail :

```text wrap theme={null}
commit my changes with a descriptive message
create a pr for this feature
summarize the changes I've made to the auth module
```

Lors de la création de demandes de fusion, Claude génère des descriptions basées sur les modifications de code réelles et peut ajouter du contexte concernant les tests ou les décisions d'implémentation.

<h2 id="use-third-party-providers">
  Utiliser des fournisseurs tiers
</h2>

Par défaut, Claude Code se connecte directement à l'API d'Anthropic. Si votre organisation utilise Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry pour accéder à Claude, configurez l'extension pour utiliser votre fournisseur à la place :

<Steps>
  <Step title="Désactiver l'invite de connexion">
    Ouvrez le [paramètre Désactiver l'invite de connexion](vscode://settings/claudeCode.disableLoginPrompt) et cochez la case.

    Vous pouvez également ouvrir les paramètres VS Code (`Cmd+,` sur Mac ou `Ctrl+,` sur Windows/Linux), rechercher « Claude Code login », et cocher **Désactiver l'invite de connexion**.
  </Step>

  <Step title="Configurer votre fournisseur">
    Suivez le guide de configuration pour votre fournisseur :

    * [Claude Code sur Amazon Bedrock](/docs/fr/amazon-bedrock)
    * [Claude Code sur Google Cloud's Agent Platform](/docs/fr/google-vertex-ai)
    * [Claude Code sur Microsoft Foundry](/docs/fr/microsoft-foundry)

    Ces guides couvrent la configuration de votre fournisseur dans `~/.claude/settings.json`, ce qui garantit que vos paramètres sont partagés entre l'extension VS Code et la CLI.
  </Step>
</Steps>

Sur un fournisseur tiers, l'extension n'offre pas les fonctionnalités qui nécessitent un compte claude.ai, telles que les barres de suivi de l'utilisation du plan, la [dictée vocale](/docs/fr/voice-dictation) et l'onglet Web pour les [sessions cloud](#resume-cloud-sessions-from-claude-ai). Pour ce que la boîte de dialogue Compte et utilisation affiche sur ces connexions, voir [Vérifier le compte et l'utilisation](#check-account-and-usage).

Une connexion claude.ai restante d'une `/login` antérieure reste inutilisée : l'extension ne l'envoie avec aucune demande.

<h2 id="security-and-privacy">
  Sécurité et confidentialité
</h2>

Votre code reste privé. Claude Code traite votre code pour fournir une assistance, mais ne l'utilise pas pour entraîner les modèles. Pour plus de détails sur la gestion des données et comment refuser la journalisation, consultez [Données et confidentialité](/docs/fr/data-usage).

Avec les autorisations de modification automatique activées, Claude Code peut modifier les fichiers de configuration de VS Code (comme `settings.json` ou `tasks.json`) que VS Code peut exécuter automatiquement. Pour réduire les risques lorsque vous travaillez avec du code non approuvé :

* Activez le [Mode restreint de VS Code](https://code.visualstudio.com/docs/editor/workspace-trust#_restricted-mode) pour les espaces de travail non approuvés
* Utilisez le mode Manuel au lieu de Modifier automatiquement ou Auto pour les modifications
* Examinez attentivement les modifications avant de les accepter

<h3 id="the-built-in-ide-mcp-server">
  Le serveur MCP IDE intégré
</h3>

Lorsque l'extension est active, elle exécute un serveur MCP local auquel le CLI se connecte automatiquement. C'est ainsi que le CLI ouvre les diffs dans la visionneuse de diffs native de VS Code, lit votre sélection actuelle pour les mentions `@`, et — lorsque vous travaillez dans un notebook Jupyter — demande à VS Code d'exécuter les cellules.

Le serveur est nommé `ide` et est masqué de `/mcp` car il n'y a rien à configurer. Si votre organisation utilise un hook `PreToolUse` pour créer une liste d'autorisation des outils MCP, vous devrez cependant savoir qu'il existe.

**Contexte de sélection et de fichier ouvert.** Lors de la connexion, le CLI inclut votre sélection d'éditeur actuelle et le chemin du fichier actif comme contexte sur chaque invite que vous envoyez. La transcription affiche une ligne `⧉ Selected N lines from <file>` lorsque cela se produit.

Pour exclure un fichier sensible tel que `.env`, ajoutez une [règle de refus `Read`](/docs/fr/permissions#read-and-edit) pour son chemin. Une règle de refus correspondante empêche à la fois le texte sélectionné et l'avis de fichier ouvert pour ce fichier d'atteindre Claude.

Si vous désactivez le [paramètre Attacher le fichier ouvert](#extension-settings), le CLI reçoit le chemin du fichier actif uniquement lorsque vous avez du texte sélectionné dedans.

**Transport et authentification.** Le serveur se lie à `127.0.0.1` sur un port aléatoire dans la plage 10000–65535, et le port n'est pas configurable. Le transport est un `ws://` non chiffré ; comme la socket est en boucle locale uniquement, tout processus qui pourrait capturer le trafic peut également lire le jeton du fichier de verrouillage, donc TLS n'ajouterait pas de protection. Chaque activation d'extension génère un jeton d'authentification aléatoire frais, l'écrit dans un fichier de verrouillage à `~/.claude/ide/<port>.lock`, et le CLI doit le présenter comme l'en-tête `X-Claude-Code-Ide-Authorization` pour se connecter. Le fichier de verrouillage a des autorisations `0600` dans un répertoire `0700`, donc seul l'utilisateur exécutant VS Code peut le lire. Si `CLAUDE_CONFIG_DIR` est défini, le fichier de verrouillage est écrit dans `$CLAUDE_CONFIG_DIR/ide/` à la place.

**Outils exposés au modèle.** Le serveur héberge une douzaine d'outils, mais seulement deux sont visibles au modèle. Le reste est un RPC interne que le CLI utilise pour sa propre interface utilisateur — ouvrir les diffs, lire les sélections, enregistrer les fichiers — et sont filtrés avant que la liste des outils n'atteigne Claude.

| Nom de l'outil (tel que vu par les hooks) | Ce qu'il fait                                                                                                                                             | Lecture seule |
| ----------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
| `mcp__ide__getDiagnostics`                | Retourne les diagnostics du serveur de langage — les erreurs et avertissements dans le panneau Problèmes de VS Code. Optionnellement limité à un fichier. | Oui           |
| `mcp__ide__executeCode`                   | Exécute le code Python dans le kernel du notebook Jupyter actif. Voir le flux de confirmation ci-dessous.                                                 | Non           |

**L'exécution Jupyter demande toujours d'abord.** `mcp__ide__executeCode` ne peut rien exécuter silencieusement. À chaque appel, le code est inséré comme une nouvelle cellule à la fin du notebook actif, VS Code le fait défiler dans la vue, et un Quick Pick natif vous demande d'**Exécuter** ou d'**Annuler**. Annuler — ou fermer le sélecteur avec `Esc` — retourne une erreur à Claude et rien ne s'exécute. L'outil refuse également catégoriquement lorsqu'il n'y a pas de notebook actif, lorsque l'extension Jupyter (`ms-toolsai.jupyter`) n'est pas installée, ou lorsque le kernel n'est pas Python.

<Note>
  La confirmation Quick Pick est séparée des hooks `PreToolUse`. Une entrée de liste d'autorisation pour `mcp__ide__executeCode` permet à Claude de *proposer* d'exécuter une cellule ; le Quick Pick à l'intérieur de VS Code est ce qui lui permet de l'*exécuter réellement*.
</Note>

<a id="troubleshooting" />

<h2 id="fix-common-issues">
  Résoudre les problèmes courants
</h2>

<h3 id="extension-won’t-install">
  L'extension ne s'installe pas
</h3>

* Assurez-vous que vous disposez d'une version compatible de VS Code (1.94.0 ou ultérieure)
* Vérifiez que VS Code a la permission d'installer des extensions
* Essayez d'installer directement depuis la [Place de marché VS Code](https://marketplace.visualstudio.com/items?itemName=anthropic.claude-code)

<h3 id="spark-icon-not-visible">
  L'icône Spark n'est pas visible
</h3>

L'icône Spark apparaît dans la **Barre d'outils de l'éditeur** (en haut à droite de l'éditeur) lorsque vous avez un fichier ouvert. Si vous ne la voyez pas :

1. **Ouvrez un fichier** : L'icône nécessite qu'un fichier soit ouvert. Avoir simplement un dossier ouvert ne suffit pas.
2. **Vérifiez la version de VS Code** : Nécessite la version 1.94.0 ou ultérieure (Aide → À propos)
3. **Redémarrez VS Code** : Exécutez « Developer: Reload Window » depuis la Palette de commandes
4. **Désactivez les extensions conflictuelles** : Désactivez temporairement les autres extensions IA (Cline, Continue, etc.)
5. **Vérifiez la confiance de l'espace de travail** : L'extension ne fonctionne pas en mode restreint

Sinon, si vous avez défini [`preferredLocation`](#extension-settings) sur `sidebar`, ou ouvert Claude avec **Claude Code: Open in Side Bar**, cliquez sur « ✻ Claude Code » dans la **Barre d'état** (coin inférieur droit). Cela fonctionne même sans fichier ouvert. Vous pouvez également utiliser la **Palette de commandes** (`Cmd+Shift+P` / `Ctrl+Shift+P`) et taper « Claude Code ».

<h3 id="cmd-esc-does-nothing-on-macos">
  Cmd+Esc ne fait rien sur macOS
</h3>

Sur macOS Tahoe et versions ultérieures, le raccourci système Game Overlay est lié à `Cmd+Esc` par défaut et intercepte la frappe avant qu'elle n'atteigne VS Code. Pour libérer le raccourci :

1. Ouvrez Réglages système
2. Allez à Clavier, puis Raccourcis clavier, puis Contrôleurs de jeu
3. Décochez la case Game Overlay

Sinon, reliez l'extension à une touche différente : ouvrez l'[éditeur de raccourcis clavier](https://code.visualstudio.com/docs/configure/keybindings) de VS Code (`Cmd+K Cmd+S`), recherchez `Claude Code: Focus input`, et attribuez une nouvelle liaison.

<h3 id="claude-code-never-responds">
  Claude Code ne répond jamais
</h3>

Si Claude Code ne répond pas à vos invites :

1. **Vérifiez votre connexion Internet** : Assurez-vous que vous disposez d'une connexion Internet stable
2. **Commencez une nouvelle conversation** : Essayez de commencer une nouvelle conversation pour voir si le problème persiste
3. **Essayez l'interface de ligne de commande** : Exécutez `claude` depuis le terminal pour voir si vous obtenez des messages d'erreur plus détaillés

Si les problèmes persistent, [signalez un problème sur GitHub](https://github.com/anthropics/claude-code/issues) avec des détails sur l'erreur.

<h2 id="uninstall-the-extension">
  Désinstaller l'extension
</h2>

Pour désinstaller l'extension Claude Code :

1. Ouvrez la vue Extensions (`Cmd+Shift+X` sur Mac ou `Ctrl+Shift+X` sur Windows/Linux)
2. Recherchez « Claude Code »
3. Cliquez sur **Désinstaller**

Si vous exécutez `claude` dans un terminal intégré VS Code, Claude Code réinstalle l'extension automatiquement. Pour la maintenir désinstallée, désactivez **Auto-install IDE extension** dans `/config`, ou définissez [`autoInstallIdeExtension`](/docs/fr/settings-reference#autoinstallideextension) sur `false`. Vous pouvez également définir la variable d'environnement [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](/docs/fr/env-vars) sur `1`.

Pour supprimer également les données d'extension et réinitialiser tous les paramètres, supprimez le répertoire de stockage de l'extension pour votre plateforme.

Sur macOS :

```bash theme={null}
rm -rf ~/Library/"Application Support"/Code/User/globalStorage/anthropic.claude-code
```

Sur Linux :

```bash theme={null}
rm -rf ~/.config/Code/User/globalStorage/anthropic.claude-code
```

Sur Windows, dans PowerShell :

```powershell theme={null}
Remove-Item -Recurse -Force "$env:APPDATA\Code\User\globalStorage\anthropic.claude-code"
```

Pour obtenir une aide supplémentaire, consultez le [guide de dépannage](/docs/fr/troubleshooting).

<h2 id="next-steps">
  Étapes suivantes
</h2>

Maintenant que vous avez Claude Code configuré dans VS Code :

* [Explorez les flux de travail courants](/docs/fr/common-workflows) pour tirer le meilleur parti de Claude Code
* [Configurez les serveurs MCP](/docs/fr/mcp) pour étendre les capacités de Claude avec des outils externes. Ajoutez et gérez-les avec `/mcp` dans le panneau de chat.
* [Configurez les paramètres Claude Code](/docs/fr/settings) pour personnaliser les commandes autorisées, les hooks et bien d'autres. Ces paramètres sont partagés entre l'extension et le CLI.
