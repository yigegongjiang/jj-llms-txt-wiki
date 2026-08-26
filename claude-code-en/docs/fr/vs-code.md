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

* VS Code 1.98.0 ou supérieur
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

Une fois installée, vous pouvez commencer à utiliser Claude Code via l'interface VS Code :

<Steps>
  <Step title="Ouvrir le panneau Claude Code">
    Dans VS Code, l'icône Spark indique Claude Code : <img src="https://mintcdn.com/claude-code/c5r9_6tjPMzFdDDT/images/vs-code-spark-icon.svg?fit=max&auto=format&n=c5r9_6tjPMzFdDDT&q=85&s=3ca45e00deadec8c8f4b4f807da94505" alt="Icône Spark" style={{display: "inline", height: "0.85em", verticalAlign: "middle"}} width="16" height="16" data-path="images/vs-code-spark-icon.svg" />

    Le moyen le plus rapide d'ouvrir Claude est de cliquer sur l'icône Spark dans la **Barre d'outils de l'éditeur** (coin supérieur droit de l'éditeur). L'icône n'apparaît que lorsque vous avez un fichier ouvert.

    <img src="https://mintcdn.com/claude-code/mfM-EyoZGnQv8JTc/images/vs-code-editor-icon.png?fit=max&auto=format&n=mfM-EyoZGnQv8JTc&q=85&s=eb4540325d94664c51776dbbfec4cf02" alt="Éditeur VS Code montrant l'icône Spark dans la Barre d'outils de l'éditeur" width="2796" height="734" data-path="images/vs-code-editor-icon.png" />

    Autres façons d'ouvrir Claude Code :

    * **Barre d'activité** : cliquez sur l'icône Spark dans la barre latérale gauche pour ouvrir la liste des sessions. Cliquez sur n'importe quelle session pour l'ouvrir en tant qu'onglet d'éditeur complet, ou démarrez-en une nouvelle. Cette icône est toujours visible dans la Barre d'activité.
    * **Palette de commandes** : `Cmd+Shift+P` (Mac) ou `Ctrl+Shift+P` (Windows/Linux), tapez « Claude Code » et sélectionnez une option comme « Ouvrir dans un nouvel onglet »
    * **Barre d'état** : cliquez sur **✱ Claude Code** dans le coin inférieur droit de la fenêtre. Cela fonctionne même quand aucun fichier n'est ouvert.

    Vous pouvez faire glisser le panneau Claude pour le repositionner n'importe où dans VS Code. Consultez [Personnaliser votre flux de travail](#customize-your-workflow) pour plus de détails.
  </Step>

  <Step title="Se connecter">
    La première fois que vous ouvrez le panneau, un écran de connexion apparaît. Cliquez sur **Se connecter** et complétez l'autorisation dans votre navigateur.

    Si vous voyez **Non connecté · Veuillez exécuter /login** plus tard, l'extension rouvre automatiquement l'écran de connexion. Si cela n'apparaît pas, rechargez la fenêtre à partir de la Palette de commandes avec **Developer: Reload Window**.

    Si vous avez `ANTHROPIC_API_KEY` défini dans votre shell mais que vous voyez toujours l'invite de connexion, VS Code n'a peut-être pas hérité de votre environnement shell. Lancez VS Code à partir d'un terminal avec `code .` pour qu'il hérite de vos variables d'environnement, ou connectez-vous avec votre compte Claude à la place.

    Après vous être connecté, une liste de contrôle **Apprendre Claude Code** apparaît. Parcourez chaque élément en cliquant sur **Montrer-moi**, ou fermez-la avec le X. Pour la rouvrir plus tard, décochez **Masquer l'intégration** dans les paramètres VS Code sous Extensions → Claude Code.
  </Step>

  <Step title="Envoyer une invite">
    Demandez à Claude de vous aider avec votre code ou vos fichiers, qu'il s'agisse d'expliquer comment quelque chose fonctionne, de déboguer un problème ou d'apporter des modifications.

    <Tip>Claude voit automatiquement votre texte sélectionné. Appuyez sur `Option+K` (Mac) / `Alt+K` (Windows/Linux) pour insérer également une référence de mention @ (comme `@file.ts#5-10`) dans votre invite.</Tip>

    Voici un exemple de question sur une ligne particulière dans un fichier :

    <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-send-prompt.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=ede3ed8d8d5f940e01c5de636d009cfd" alt="Éditeur VS Code avec les lignes 2-3 sélectionnées dans un fichier Python, et le panneau Claude Code montrant une question sur ces lignes avec une référence de mention @" width="3288" height="1876" data-path="images/vs-code-send-prompt.png" />
  </Step>

  <Step title="Examiner les modifications">
    Lorsque Claude souhaite modifier un fichier, il affiche une comparaison côte à côte de l'original et des modifications proposées, puis demande une permission. Vous pouvez accepter, rejeter ou dire à Claude ce qu'il faut faire à la place. Si vous modifiez le contenu proposé directement dans la vue diff avant d'accepter, Claude est informé que vous l'avez modifié afin qu'il ne suppose pas que le fichier correspond à sa proposition originale.

    <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-edits.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=e005f9b41c541c5c7c59c082f7c4841c" alt="VS Code montrant un diff des modifications proposées par Claude avec une invite de permission demandant si vous souhaitez effectuer la modification" width="3292" height="1876" data-path="images/vs-code-edits.png" />
  </Step>
</Steps>

Pour plus d'idées sur ce que vous pouvez faire avec Claude Code, consultez [Flux de travail courants](/docs/fr/common-workflows).

<Tip>
  Exécutez ' Claude Code: Open Walkthrough ' à partir de la Palette de commandes pour une visite guidée des bases.
</Tip>

<h2 id="use-the-prompt-box">
  Utiliser la zone de saisie
</h2>

La zone de saisie prend en charge plusieurs fonctionnalités :

* **Modes de permission** : cliquez sur l'indicateur de mode en bas de la zone de saisie pour changer de mode, ou définissez la valeur par défaut dans les paramètres VS Code sous `claudeCode.initialPermissionMode`. Consultez [modes de permission](/docs/fr/permission-modes#switch-permission-modes) pour chaque mode que l'indicateur propose.
  * **Manuel** : Claude demande une permission avant les modifications de fichiers et la plupart des commandes shell.
  * **Plan** : Claude décrit ce qu'il fera et attend l'approbation avant d'apporter des modifications. VS Code ouvre automatiquement le plan en tant que document Markdown complet où vous pouvez ajouter des commentaires en ligne pour donner des commentaires avant que Claude ne commence.
  * **Édition automatique** : Claude apporte des modifications sans demander.
* **Menu de commandes** : cliquez sur `/` ou tapez `/` pour ouvrir le menu de commandes. Les options incluent l'attachement de fichiers, le changement de modèles, l'activation de la réflexion étendue, l'affichage de l'utilisation du plan (`/usage`) et le démarrage d'une session [Remote Control](/docs/fr/remote-control) (`/remote-control`). La section Personnaliser fournit l'accès aux serveurs MCP, hooks, mémoire, permissions et plugins. Les éléments avec une icône de terminal s'ouvrent dans le terminal intégré.
  * La section Paramètres inclut **Activer Remote Control pour toutes les sessions**, qui définit [`remoteControlAtStartup`](/docs/fr/settings#available-settings) afin que [chaque nouvelle session interactive se connecte automatiquement à Remote Control](/docs/fr/remote-control#enable-remote-control-for-all-sessions). Nécessite Claude Code v2.1.203 ou version ultérieure.
* **Indicateur de contexte** : la zone de saisie affiche la quantité de fenêtre de contexte de Claude que vous utilisez. Claude se compacte automatiquement si nécessaire, ou vous pouvez exécuter `/compact` manuellement.
* **Réflexion étendue** : permet à Claude de consacrer plus de temps à raisonner sur des problèmes complexes. Activez-la via le menu de commandes (`/`). Le raisonnement de Claude apparaît dans la conversation sous forme de blocs réduits : cliquez sur un bloc pour le lire, ou appuyez sur `Ctrl+O` pour développer ou réduire chaque bloc de réflexion dans la session. Consultez [Réflexion étendue](/docs/fr/model-config#extended-thinking) pour plus de détails.
* **Entrée multiligne** : appuyez sur `Shift+Entrée` pour ajouter une nouvelle ligne sans envoyer. Cela fonctionne également dans l'entrée en texte libre « Autre » des dialogues de question.

<h3 id="reference-files-and-folders">
  Référencer des fichiers et des dossiers
</h3>

Utilisez les mentions @ pour donner à Claude du contexte sur des fichiers ou des dossiers spécifiques. Lorsque vous tapez `@` suivi d'un nom de fichier ou de dossier, Claude lit ce contenu et peut répondre à des questions à ce sujet ou y apporter des modifications. Claude Code prend en charge la correspondance floue, vous pouvez donc taper des noms partiels pour trouver ce dont vous avez besoin :

```text theme={null}
> Explain the logic in @auth (fuzzy matches auth.js, AuthService.ts, etc.)
> What's in @src/components/ (include a trailing slash for folders)
```

Pour les grands PDF, vous pouvez demander à Claude de lire des pages spécifiques au lieu du fichier entier : une seule page, une plage comme les pages 1-10, ou une plage ouverte comme la page 3 et au-delà.

Lorsque vous sélectionnez du texte dans l'éditeur, Claude peut voir votre code en surbrillance automatiquement. Le pied de page de la zone de saisie affiche le nombre de lignes sélectionnées. Appuyez sur `Option+K` (Mac) / `Alt+K` (Windows/Linux) pour insérer une mention @ avec le chemin du fichier et les numéros de ligne (par exemple, `@app.ts#5-10`). Cliquez sur l'indicateur de sélection pour basculer si Claude peut voir votre texte en surbrillance - l'icône en forme de barre oblique signifie que la sélection est masquée à Claude.

Vous pouvez également maintenir `Shift` enfoncé tout en faisant glisser des fichiers dans la zone de saisie pour les ajouter en tant que pièces jointes. Cliquez sur le X sur n'importe quelle pièce jointe pour la supprimer du contexte.

<h3 id="resume-past-conversations">
  Reprendre les conversations passées
</h3>

Cliquez sur le bouton **Historique des sessions** en haut du panneau Claude Code pour accéder à votre historique de conversations. Vous pouvez rechercher par mot-clé ou parcourir par heure (Aujourd'hui, Hier, 7 derniers jours, etc.). Cliquez sur n'importe quelle conversation pour la reprendre avec l'historique complet des messages. Les nouvelles sessions reçoivent des titres générés par l'IA en fonction de votre premier message. Survolez une session pour révéler les actions de renommage et de suppression : renommez pour lui donner un titre descriptif, ou supprimez pour la supprimer de la liste. Pour plus d'informations sur la reprise des sessions, consultez [Gérer les sessions](/docs/fr/sessions).

<h3 id="resume-cloud-sessions-from-claude-ai">
  Reprendre les sessions distantes de Claude.ai
</h3>

Si vous utilisez [Claude Code sur le web](/docs/fr/claude-code-on-the-web), vous pouvez reprendre ces sessions distantes directement dans VS Code. Cela nécessite de se connecter avec **Claude.ai Subscription**, pas Anthropic Console.

<Steps>
  <Step title="Ouvrir l'historique des sessions">
    Cliquez sur le bouton **Historique des sessions** en haut du panneau Claude Code.
  </Step>

  <Step title="Sélectionner l'onglet Distant">
    Le dialogue affiche deux onglets : Local et Distant. Cliquez sur **Distant** pour voir les sessions de claude.ai.
  </Step>

  <Step title="Sélectionner une session à reprendre">
    Parcourez ou recherchez vos sessions distantes. Cliquez sur n'importe quelle session pour la télécharger et continuer la conversation localement.
  </Step>
</Steps>

<Note>
  Seules les sessions web démarrées avec un référentiel GitHub apparaissent dans l'onglet Distant. La reprise charge l'historique de la conversation localement ; les modifications ne sont pas resynchronisées vers claude.ai.
</Note>

<h3 id="check-account-and-usage">
  Vérifier le compte et l'utilisation
</h3>

Exécutez `/usage` à partir du menu de commandes pour ouvrir le dialogue Compte et utilisation. Il affiche votre compte connecté, votre plan et les barres d'utilisation pour la session actuelle et la semaine avec le temps restant avant que chaque limite ne soit réinitialisée.

Le dialogue détaille également ce qui contribue à vos limites de plan. Il signale les comportements qui représentent 10 % ou plus de l'utilisation récente, tels que les défauts de cache, le contexte long et les sessions lourdes en sous-agents ou hautement parallèles, chacun avec un conseil pour le réduire. Les tableaux d'attribution montrent la quantité d'utilisation provenant de chaque skill, sous-agent, plugin et serveur MCP. Nécessite Claude Code v2.1.174 ou version ultérieure.

Utilisez le bouton bascule Jour et Semaine pour basculer entre les 24 dernières heures et les 7 derniers jours. Les chiffres sont approximatifs et calculés à partir des sessions locales sur cette machine, donc l'utilisation d'autres appareils ou de claude.ai n'est pas incluse. Pour plus d'informations sur le suivi et la réduction de l'utilisation, consultez [Suivre vos coûts](/docs/fr/costs#track-your-costs).

<h2 id="customize-your-workflow">
  Personnaliser votre flux de travail
</h2>

Une fois que vous êtes opérationnel, vous pouvez repositionner le panneau Claude, exécuter plusieurs sessions ou passer au mode terminal.

<h3 id="choose-where-claude-lives">
  Choisir où Claude se trouve
</h3>

Vous pouvez faire glisser le panneau Claude pour le repositionner n'importe où dans VS Code. Saisissez l'onglet ou la barre de titre du panneau et faites-le glisser vers :

* **Barre latérale secondaire** : le côté droit de la fenêtre. Garde Claude visible pendant que vous codez.
* **Barre latérale principale** : la barre latérale gauche avec les icônes pour l'Explorateur, la Recherche, etc.
* **Zone d'éditeur** : ouvre Claude en tant qu'onglet à côté de vos fichiers. Utile pour les tâches secondaires.

<Tip>
  Utilisez la barre latérale pour votre session Claude principale et ouvrez des onglets supplémentaires pour les tâches secondaires. Claude se souvient de votre emplacement préféré. L'icône de la liste des sessions de la Barre d'activité est séparée du panneau Claude : la liste des sessions est toujours visible dans la Barre d'activité, tandis que l'icône du panneau Claude n'y apparaît que lorsque le panneau est ancré à la barre latérale gauche.
</Tip>

<h3 id="run-multiple-conversations">
  Exécuter plusieurs conversations
</h3>

Utilisez **Ouvrir dans un nouvel onglet** ou **Ouvrir dans une nouvelle fenêtre** à partir de la Palette de commandes pour démarrer des conversations supplémentaires. Chaque conversation maintient son propre historique et contexte, vous permettant de travailler sur différentes tâches en parallèle.

Lors de l'utilisation d'onglets, un petit point coloré sur l'icône spark indique l'état : bleu signifie qu'une demande de permission est en attente, orange signifie que Claude a terminé pendant que l'onglet était masqué.

<h3 id="switch-to-terminal-mode">
  Passer au mode terminal
</h3>

Par défaut, l'extension ouvre un panneau de chat graphique. Si vous préférez l'interface de style CLI, ouvrez le [paramètre Utiliser le terminal](vscode://settings/claudeCode.useTerminal) et cochez la case.

Vous pouvez également ouvrir les paramètres VS Code (`Cmd+,` sur Mac ou `Ctrl+,` sur Windows/Linux), aller à Extensions → Claude Code et cocher **Utiliser le terminal**.

<h2 id="manage-plugins">
  Gérer les plugins
</h2>

L'extension VS Code inclut une interface graphique pour installer et gérer les [plugins](/docs/fr/plugins). Tapez `/plugins` dans la zone de saisie pour ouvrir l'interface **Gérer les plugins**.

<h3 id="install-plugins">
  Installer les plugins
</h3>

Le dialogue des plugins affiche deux onglets : **Plugins** et **Marchés**.

Dans l'onglet Plugins :

* Les **plugins installés** apparaissent en haut avec des commutateurs pour les activer ou les désactiver
* Les **plugins disponibles** de vos marchés configurés apparaissent ci-dessous
* Recherchez pour filtrer les plugins par nom ou description
* Cliquez sur **Installer** sur n'importe quel plugin disponible

Lorsque vous installez un plugin, choisissez l'étendue de l'installation :

* **Installer pour vous** : disponible dans tous vos projets (étendue utilisateur)
* **Installer pour ce projet** : partagé avec les collaborateurs du projet (étendue du projet)
* **Installer localement** : uniquement pour vous, uniquement dans ce référentiel (étendue locale)

<h3 id="manage-marketplaces">
  Gérer les marchés
</h3>

Basculez vers l'onglet **Marchés** pour ajouter ou supprimer des sources de plugins :

* Entrez un référentiel GitHub, une URL ou un chemin local pour ajouter un nouveau marché
* Cliquez sur l'icône d'actualisation pour mettre à jour la liste des plugins d'un marché
* Cliquez sur l'icône de corbeille pour supprimer un marché

Après avoir apporté des modifications, une bannière vous invite à redémarrer Claude Code pour appliquer les mises à jour.

<Note>
  La gestion des plugins dans VS Code utilise les mêmes commandes CLI sous le capot. Les plugins et les marchés que vous configurez dans l'extension sont également disponibles dans le CLI, et vice versa.
</Note>

Pour plus d'informations sur le système de plugins, consultez [Plugins](/docs/fr/plugins) et [Marchés de plugins](/docs/fr/plugin-marketplaces).

<h2 id="automate-browser-tasks-with-chrome">
  Automatiser les tâches du navigateur avec Chrome
</h2>

Connectez Claude à votre navigateur Chrome pour tester les applications web, déboguer avec les journaux de la console et automatiser les flux de travail du navigateur sans quitter VS Code. Cela nécessite l'extension [Claude in Chrome](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) version 1.0.36 ou supérieure.

Tapez `@browser` dans la zone de saisie suivi de ce que vous voulez que Claude fasse :

```text theme={null}
@browser go to localhost:3000 and check the console for errors
```

Vous pouvez également ouvrir le menu des pièces jointes pour sélectionner des outils de navigateur spécifiques comme ouvrir un nouvel onglet ou lire le contenu de la page.

Claude ouvre de nouveaux onglets pour les tâches du navigateur et partage l'état de connexion de votre navigateur, il peut donc accéder à n'importe quel site auquel vous êtes déjà connecté.

Pour les instructions de configuration, la liste complète des capacités et le dépannage, consultez [Utiliser Claude Code avec Chrome](/docs/fr/chrome).

<h2 id="vs-code-commands-and-shortcuts">
  Commandes et raccourcis VS Code
</h2>

Ouvrez la Palette de commandes (`Cmd+Shift+P` sur Mac ou `Ctrl+Shift+P` sur Windows/Linux) et tapez « Claude Code » pour voir toutes les commandes VS Code disponibles pour l'extension Claude Code.

Certains raccourcis dépendent du panneau qui est « actif » (recevant l'entrée au clavier). Lorsque votre curseur est dans un fichier de code, l'éditeur est actif. Lorsque votre curseur est dans la zone de saisie de Claude, Claude est actif. Utilisez `Cmd+Esc` / `Ctrl+Esc` pour basculer entre eux.

<Note>
  Ce sont des commandes VS Code pour contrôler l'extension. Toutes les commandes Claude Code intégrées ne sont pas disponibles dans l'extension. Consultez [Extension VS Code vs. CLI Claude Code](#vs-code-extension-vs-claude-code-cli) pour plus de détails.
</Note>

| Commande                   | Raccourci                                                | Description                                                                                                                                                                                                                                    |
| -------------------------- | -------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Focus Input                | `Cmd+Esc` (Mac) / `Ctrl+Esc` (Windows/Linux)             | Basculer le focus entre l'éditeur et Claude                                                                                                                                                                                                    |
| Open in Side Bar           | -                                                        | Ouvrir Claude dans la barre latérale gauche                                                                                                                                                                                                    |
| Open in Terminal           | -                                                        | Ouvrir Claude en mode terminal                                                                                                                                                                                                                 |
| Open in New Tab            | `Cmd+Shift+Esc` (Mac) / `Ctrl+Shift+Esc` (Windows/Linux) | Ouvrir une nouvelle conversation en tant qu'onglet d'éditeur                                                                                                                                                                                   |
| Open in New Window         | -                                                        | Ouvrir une nouvelle conversation dans une fenêtre séparée                                                                                                                                                                                      |
| New Conversation           | `Cmd+N` (Mac) / `Ctrl+N` (Windows/Linux)                 | Démarrer une nouvelle conversation. Nécessite que Claude soit actif et `enableNewConversationShortcut` défini sur `true`                                                                                                                       |
| Reopen Closed Session      | `Cmd+Shift+T` (Mac) / `Ctrl+Shift+T` (Windows/Linux)     | Rouvrir l'onglet de session Claude fermé le plus récemment. Bascule vers la réouverture normale d'éditeur fermé de VS Code lorsque le dernier onglet fermé n'était pas une session Claude. Désactiver avec `enableReopenClosedSessionShortcut` |
| Insert @-Mention Reference | `Option+K` (Mac) / `Alt+K` (Windows/Linux)               | Insérer une référence au fichier actuel et à la sélection (nécessite que l'éditeur soit actif)                                                                                                                                                 |
| Show Logs                  | -                                                        | Afficher les journaux de débogage de l'extension                                                                                                                                                                                               |
| Logout                     | -                                                        | Se déconnecter de votre compte Anthropic                                                                                                                                                                                                       |

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

| Paramètre | Description                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`  | Texte à pré-remplir dans la zone de saisie. Doit être codé en URL. L'invite est pré-remplie mais non soumise automatiquement.                                                                                                                                                                                                                                                                                                                       |
| `session` | Un ID de session à reprendre au lieu de démarrer une nouvelle conversation. La session doit appartenir à l'espace de travail actuellement ouvert dans VS Code. Si la session n'est pas trouvée, une conversation nouvelle commence à la place. Si la session est déjà ouverte dans un onglet, cet onglet est actif. Pour capturer un ID de session par programmation, consultez [Continuer les conversations](/docs/fr/headless#continue-conversations). |

Par exemple, pour ouvrir un onglet pré-rempli avec « review my changes » :

```text theme={null}
vscode://anthropic.claude-code/open?prompt=review%20my%20changes
```

Pour lancer une session terminal au lieu d'un onglet VS Code, utilisez le gestionnaire `claude-cli://` de la CLI. Consultez [Lancer des sessions à partir de liens](/docs/fr/deep-links).

<h2 id="configure-settings">
  Configurer les paramètres
</h2>

L'extension a deux types de paramètres :

* **Paramètres d'extension** dans VS Code : contrôlent le comportement de l'extension dans VS Code. Ouvrez avec `Cmd+,` (Mac) ou `Ctrl+,` (Windows/Linux), puis allez à Extensions → Claude Code. Vous pouvez également taper `/` et sélectionner **General Config** pour ouvrir les paramètres.
* **Paramètres Claude Code** dans `~/.claude/settings.json` : partagés entre l'extension et CLI. Utilisez pour les commandes autorisées, les variables d'environnement, les hooks et les serveurs MCP. Consultez [Paramètres](/docs/fr/settings) pour plus de détails.

<Tip>
  Ajoutez `"$schema": "https://json.schemastore.org/claude-code-settings.json"` à votre `settings.json` pour obtenir l'autocomplétion et la validation en ligne pour tous les paramètres disponibles directement dans VS Code.
</Tip>

<h3 id="extension-settings">
  Paramètres d'extension
</h3>

| Paramètre                           | Par défaut | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ----------------------------------- | ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `useTerminal`                       | `false`    | Lancer Claude en mode terminal au lieu du panneau graphique                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `initialPermissionMode`             | `default`  | Contrôle les invites d'approbation pour les nouvelles conversations : `default`, `plan`, `acceptEdits` ou `bypassPermissions`. `manual` est un alias pour `default` et sélectionne le mode étiqueté **Manual** dans l'indicateur de mode. Nécessite Claude Code v2.1.200 ou version ultérieure. Consultez [modes de permission](/docs/fr/permission-modes).                                                                                                                                                              |
| `preferredLocation`                 | `panel`    | Où Claude s'ouvre : `sidebar` (droite) ou `panel` (nouvel onglet)                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `autosave`                          | `true`     | Enregistrement automatique des fichiers avant que Claude ne les lise ou ne les écrive                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `useCtrlEnterToSend`                | `false`    | Utiliser Ctrl/Cmd+Entrée au lieu d'Entrée pour envoyer les invites                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `enableNewConversationShortcut`     | `false`    | Activer Cmd/Ctrl+N pour démarrer une nouvelle conversation                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `enableReopenClosedSessionShortcut` | `true`     | Utiliser Cmd/Ctrl+Maj+T pour rouvrir l'onglet de session Claude fermé le plus récemment. Lorsque le dernier onglet fermé n'était pas une session Claude, le raccourci exécute la commande normale de réouverture d'éditeur fermé de VS Code à la place.                                                                                                                                                                                                                                                             |
| `hideOnboarding`                    | `false`    | Masquer la liste de contrôle d'intégration (icône de chapeau de graduation)                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `respectGitIgnore`                  | `true`     | Exclure les modèles .gitignore des recherches de fichiers                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `usePythonEnvironment`              | `true`     | Activer l'environnement Python de l'espace de travail lors de l'exécution de Claude. Nécessite l'extension Python.                                                                                                                                                                                                                                                                                                                                                                                                  |
| `environmentVariables`              | `[]`       | Définir les variables d'environnement pour le processus Claude. Utilisez plutôt les paramètres Claude Code pour la configuration partagée.                                                                                                                                                                                                                                                                                                                                                                          |
| `disableLoginPrompt`                | `false`    | Ignorer les invites d'authentification (pour les configurations de fournisseur tiers)                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `allowDangerouslySkipPermissions`   | `false`    | Ajoute Bypass permissions au sélecteur de mode. Utilisez uniquement dans les sandboxes sans accès à Internet.                                                                                                                                                                                                                                                                                                                                                                                                       |
| `claudeProcessWrapper`              | -          | Exécutable utilisé pour lancer le processus Claude. Le chemin du binaire fourni est transmis en tant qu'argument si présent. Définissez ceci sur un binaire `claude` installé séparément si la version de l'extension n'en inclut pas un pour votre plateforme. Une erreur « Unsupported platform » à l'activation signifie qu'aucun binaire n'est fourni pour votre plateforme ; consultez [quelles plateformes ont des binaires précompilés](/docs/fr/troubleshoot-install#native-binary-not-found-after-npm-install). |

<h2 id="vs-code-extension-vs-claude-code-cli">
  Extension VS Code vs. CLI Claude Code
</h2>

Claude Code est disponible à la fois en tant qu'extension VS Code (panneau graphique) et en tant que CLI (interface de ligne de commande dans le terminal). Certaines fonctionnalités ne sont disponibles que dans le CLI. Si vous avez besoin d'une fonctionnalité CLI uniquement, exécutez `claude` dans le terminal intégré de VS Code. Cela nécessite l'[installation CLI autonome](/docs/fr/setup) : l'extension n'ajoute pas `claude` à votre PATH. Voir [Exécuter le CLI dans VS Code](#run-cli-in-vs-code).

| Fonctionnalité               | CLI                  | Extension VS Code                                                                                         |
| ---------------------------- | -------------------- | --------------------------------------------------------------------------------------------------------- |
| Commandes et skills          | [Tous](/docs/fr/commands) | Sous-ensemble (tapez `/` pour voir les disponibles)                                                       |
| Configuration du serveur MCP | Oui                  | Partiel (ajouter des serveurs via CLI ; gérer les serveurs existants avec `/mcp` dans le panneau de chat) |
| Checkpoints                  | Oui                  | Oui                                                                                                       |
| Raccourci bash `!`           | Oui                  | Non                                                                                                       |
| Complément de tabulation     | Oui                  | Non                                                                                                       |

<h3 id="rewind-with-checkpoints">
  Rembobiner avec les checkpoints
</h3>

L'extension VS Code prend en charge les checkpoints, qui suivent les modifications de fichiers de Claude et vous permettent de rembobiner à un état précédent. Survolez n'importe quel message pour révéler le bouton de rembobinage, puis choisissez parmi trois options :

* **Fork conversation from here** : démarrer une nouvelle branche de conversation à partir de ce message tout en conservant toutes les modifications de code
* **Rewind code to here** : annuler les modifications de fichiers jusqu'à ce point dans la conversation tout en conservant l'historique complet de la conversation
* **Fork conversation and rewind code** : démarrer une nouvelle branche de conversation et annuler les modifications de fichiers jusqu'à ce point

Pour tous les détails sur le fonctionnement des checkpoints et leurs limitations, consultez [Checkpointing](/docs/fr/checkpointing).

<h3 id="run-cli-in-vs-code">
  Exécuter le CLI dans VS Code
</h3>

Pour utiliser le CLI tout en restant dans VS Code, ouvrez le terminal intégré (`` Ctrl+` `` sur Windows/Linux ou `` Cmd+` `` sur Mac) et exécutez `claude`. Le CLI s'intègre automatiquement à votre IDE pour des fonctionnalités comme l'affichage des diffs et le partage des diagnostics.

L'installation de l'extension ne place pas `claude` sur votre PATH shell. L'extension regroupe une copie privée du CLI pour son panneau de chat, mais taper `claude` dans un terminal nécessite l'[installation CLI autonome](/docs/fr/setup). Exécutez l'installation une fois et les commandes de cette page, y compris `claude mcp add` et `claude --resume`, fonctionnent dans n'importe quel terminal. Si `claude` n'est toujours pas trouvé après l'installation, [vérifiez votre PATH](/docs/fr/troubleshoot-install#verify-your-path).

Si vous utilisez un terminal externe, exécutez `/ide` dans Claude Code pour le connecter à VS Code.

<h3 id="switch-between-extension-and-cli">
  Basculer entre l'extension et le CLI
</h3>

L'extension et le CLI partagent le même historique de conversations. Pour continuer une conversation d'extension dans le CLI, exécutez `claude --resume` dans le terminal. Cela ouvre un sélecteur interactif où vous pouvez rechercher et sélectionner votre conversation.

<h3 id="include-terminal-output-in-prompts">
  Inclure la sortie du terminal dans les invites
</h3>

Référencez la sortie du terminal dans vos invites en utilisant `@terminal:name` où `name` est le titre du terminal. Cela permet à Claude de voir la sortie de la commande, les messages d'erreur ou les journaux sans copier-coller.

<h3 id="monitor-background-processes">
  Surveiller les processus en arrière-plan
</h3>

Lorsque Claude exécute des commandes longues, l'extension affiche la progression dans la barre d'état. Cependant, la visibilité des tâches en arrière-plan est limitée par rapport au CLI. Pour une meilleure visibilité, demandez à Claude de générer la commande afin que vous puissiez l'exécuter dans le terminal intégré de VS Code.

<h3 id="connect-to-external-tools-with-mcp">
  Connecter à des outils externes avec MCP
</h3>

Les serveurs MCP (Model Context Protocol) donnent à Claude accès à des outils externes, des bases de données et des API.

Pour ajouter un serveur MCP, ouvrez le terminal intégré (`` Ctrl+` `` ou `` Cmd+` ``) et exécutez `claude mcp add`. L'exemple ci-dessous ajoute le serveur MCP distant de GitHub, qui s'authentifie avec un [jeton d'accès personnel](https://github.com/settings/personal-access-tokens) transmis en tant qu'en-tête :

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

Une fois configuré, demandez à Claude d'utiliser les outils (par exemple, « Review PR #456 »).

Pour gérer les serveurs MCP sans quitter VS Code, tapez `/mcp` dans le panneau de chat. Le dialogue de gestion MCP vous permet d'activer ou de désactiver les serveurs, de vous reconnecter à un serveur et de gérer l'authentification OAuth. Consultez la [documentation MCP](/docs/fr/mcp) pour les serveurs disponibles.

<h2 id="work-with-git">
  Travailler avec git
</h2>

Claude Code s'intègre à git pour vous aider avec les flux de travail de contrôle de version directement dans VS Code. Demandez à Claude de valider les modifications, de créer des demandes de tirage ou de travailler sur plusieurs branches.

<h3 id="create-commits-and-pull-requests">
  Créer des commits et des demandes de tirage
</h3>

Claude peut mettre en scène les modifications, écrire des messages de commit et créer des demandes de tirage en fonction de votre travail :

```text theme={null}
> commit my changes with a descriptive message
> create a pr for this feature
> summarize the changes I've made to the auth module
```

Lors de la création de demandes de tirage, Claude génère des descriptions basées sur les modifications de code réelles et peut ajouter du contexte sur les tests ou les décisions de mise en œuvre.

<h3 id="use-git-worktrees-for-parallel-tasks">
  Utiliser les git worktrees pour les tâches parallèles
</h3>

Utilisez l'indicateur `--worktree` (`-w`) pour démarrer Claude dans un worktree isolé avec ses propres fichiers et branche :

```bash theme={null}
claude --worktree feature-auth
```

Chaque worktree maintient un état de fichier indépendant tout en partageant l'historique git. Cela empêche les instances de Claude d'interférer les unes avec les autres lorsqu'elles travaillent sur différentes tâches. Pour plus de détails, consultez [Exécuter des sessions parallèles avec Git worktrees](/docs/fr/worktrees).

<h2 id="use-third-party-providers">
  Utiliser des fournisseurs tiers
</h2>

Par défaut, Claude Code se connecte directement à l'API d'Anthropic. Si votre organisation utilise Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry pour accéder à Claude, configurez l'extension pour utiliser votre fournisseur à la place :

<Steps>
  <Step title="Désactiver l'invite de connexion">
    Ouvrez le [paramètre Désactiver l'invite de connexion](vscode://settings/claudeCode.disableLoginPrompt) et cochez la case.

    Vous pouvez également ouvrir les paramètres VS Code (`Cmd+,` sur Mac ou `Ctrl+,` sur Windows/Linux), recherchez ' Claude Code login ' et cochez **Désactiver l'invite de connexion**.
  </Step>

  <Step title="Configurer votre fournisseur">
    Suivez le guide de configuration de votre fournisseur :

    * [Claude Code sur Amazon Bedrock](/docs/fr/amazon-bedrock)
    * [Claude Code sur Google Cloud's Agent Platform](/docs/fr/google-vertex-ai)
    * [Claude Code sur Microsoft Foundry](/docs/fr/microsoft-foundry)

    Ces guides couvrent la configuration de votre fournisseur dans `~/.claude/settings.json`, ce qui garantit que vos paramètres sont partagés entre l'extension VS Code et le CLI.
  </Step>
</Steps>

<h2 id="security-and-privacy">
  Sécurité et confidentialité
</h2>

Votre code reste privé. Claude Code traite votre code pour fournir une assistance mais ne l'utilise pas pour entraîner les modèles. Pour plus de détails sur la gestion des données et comment refuser la journalisation, consultez [Données et confidentialité](/docs/fr/data-usage).

Avec les permissions d'édition automatique activées, Claude Code peut modifier les fichiers de configuration VS Code (comme `settings.json` ou `tasks.json`) que VS Code peut exécuter automatiquement. Pour réduire le risque lorsque vous travaillez avec du code non fiable :

* Activez le [Mode restreint VS Code](https://code.visualstudio.com/docs/editor/workspace-trust#_restricted-mode) pour les espaces de travail non fiables
* Utilisez le mode d'approbation manuelle au lieu de l'acceptation automatique pour les modifications
* Examinez attentivement les modifications avant de les accepter

<h3 id="the-built-in-ide-mcp-server">
  Le serveur MCP IDE intégré
</h3>

Lorsque l'extension est active, elle exécute un serveur MCP local auquel le CLI se connecte automatiquement. C'est ainsi que le CLI ouvre les diffs dans la visionneuse de diffs native de VS Code, lit votre sélection actuelle pour les mentions `@` et — lorsque vous travaillez dans un notebook Jupyter — demande à VS Code d'exécuter les cellules.

Le serveur est nommé `ide` et est masqué de `/mcp` car il n'y a rien à configurer. Cependant, si votre organisation utilise un hook `PreToolUse` pour créer une liste blanche des outils MCP, vous devez savoir qu'il existe.

**Sélection et contexte de fichier ouvert.** Lors de la connexion, le CLI inclut votre sélection d'éditeur actuelle et le chemin du fichier actif comme contexte sur chaque invite que vous envoyez. La transcription affiche une ligne `⧉ Selected N lines from <file>` lorsque cela se produit. Pour exclure un fichier sensible tel que `.env`, ajoutez une [règle de refus `Read`](/docs/fr/permissions#read-and-edit) pour son chemin. Une règle de refus correspondante empêche à la fois le texte sélectionné et l'avis de fichier ouvert pour ce fichier d'atteindre Claude.

**Transport et authentification.** Le serveur se lie à `127.0.0.1` sur un port aléatoire dans la plage 10000–65535, et le port n'est pas configurable. Le transport est un `ws://` non chiffré ; comme la socket est en boucle locale uniquement, tout processus qui pourrait capturer le trafic peut également lire le jeton du fichier de verrouillage, donc TLS n'ajouterait pas de protection. Chaque activation d'extension génère un jeton d'authentification aléatoire frais, l'écrit dans un fichier de verrouillage à `~/.claude/ide/<port>.lock`, et le CLI doit le présenter comme l'en-tête `X-Claude-Code-Ide-Authorization` pour se connecter. Le fichier de verrouillage a les permissions `0600` dans un répertoire `0700`, donc seul l'utilisateur exécutant VS Code peut le lire. Si `CLAUDE_CONFIG_DIR` est défini, le fichier de verrouillage est écrit à `$CLAUDE_CONFIG_DIR/ide/` à la place.

**Outils exposés au modèle.** Le serveur héberge une douzaine d'outils, mais seulement deux sont visibles au modèle. Le reste est un RPC interne que le CLI utilise pour sa propre interface utilisateur — ouvrir les diffs, lire les sélections, enregistrer les fichiers — et sont filtrés avant que la liste des outils n'atteigne Claude.

| Nom de l'outil (tel que vu par les hooks) | Ce qu'il fait                                                                                                                                             | Lecture seule |
| ----------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
| `mcp__ide__getDiagnostics`                | Retourne les diagnostics du serveur de langage — les erreurs et avertissements dans le panneau Problèmes de VS Code. Optionnellement limité à un fichier. | Oui           |
| `mcp__ide__executeCode`                   | Exécute le code Python dans le kernel du notebook Jupyter actif. Consultez le flux de confirmation ci-dessous.                                            | Non           |

**L'exécution Jupyter demande toujours d'abord.** `mcp__ide__executeCode` ne peut rien exécuter silencieusement. À chaque appel, le code est inséré en tant que nouvelle cellule à la fin du notebook actif, VS Code le fait défiler dans la vue, et un Quick Pick natif vous demande d'**Exécuter** ou d'**Annuler**. L'annulation — ou le rejet du sélecteur avec `Esc` — retourne une erreur à Claude et rien ne s'exécute. L'outil refuse également catégoriquement lorsqu'il n'y a pas de notebook actif, lorsque l'extension Jupyter (`ms-toolsai.jupyter`) n'est pas installée, ou lorsque le kernel n'est pas Python.

<Note>
  Le Quick Pick de confirmation est séparé des hooks `PreToolUse`. Une entrée de liste blanche pour `mcp__ide__executeCode` permet à Claude de *proposer* d'exécuter une cellule ; le Quick Pick dans VS Code est ce qui lui permet de l'*exécuter réellement*.
</Note>

<a id="troubleshooting" />

<h2 id="fix-common-issues">
  Corriger les problèmes courants
</h2>

<h3 id="extension-won’t-install">
  L'extension ne s'installe pas
</h3>

* Assurez-vous que vous avez une version compatible de VS Code (1.98.0 ou ultérieure)
* Vérifiez que VS Code a la permission d'installer des extensions
* Essayez d'installer directement à partir de la [Place de marché VS Code](https://marketplace.visualstudio.com/items?itemName=anthropic.claude-code)

<h3 id="spark-icon-not-visible">
  L'icône Spark n'est pas visible
</h3>

L'icône Spark apparaît dans la **Barre d'outils de l'éditeur** (coin supérieur droit de l'éditeur) lorsque vous avez un fichier ouvert. Si vous ne la voyez pas :

1. **Ouvrir un fichier** : L'icône nécessite qu'un fichier soit ouvert. Avoir juste un dossier ouvert ne suffit pas.
2. **Vérifier la version de VS Code** : Nécessite 1.98.0 ou supérieur (Aide → À propos)
3. **Redémarrer VS Code** : Exécutez « Developer: Reload Window » à partir de la Palette de commandes
4. **Désactiver les extensions conflictuelles** : Désactivez temporairement les autres extensions IA (Cline, Continue, etc.)
5. **Vérifier la confiance de l'espace de travail** : L'extension ne fonctionne pas en Mode restreint

Vous pouvez également cliquer sur « ✱ Claude Code » dans la **Barre d'état** (coin inférieur droit). Cela fonctionne même sans fichier ouvert. Vous pouvez également utiliser la **Palette de commandes** (`Cmd+Shift+P` / `Ctrl+Shift+P`) et taper « Claude Code ».

<h3 id="cmd-esc-does-nothing-on-macos">
  Cmd+Esc ne fait rien sur macOS
</h3>

Sur macOS Tahoe et versions ultérieures, le raccourci système Game Overlay est lié à `Cmd+Esc` par défaut et intercepte la frappe avant qu'elle n'atteigne VS Code. Pour libérer le raccourci :

1. Ouvrez Paramètres système
2. Allez à Clavier, puis Raccourcis clavier, puis Contrôleurs de jeu
3. Décochez la case Game Overlay

Vous pouvez également réassigner l'extension à une touche différente : ouvrez l'[éditeur de raccourcis clavier](https://code.visualstudio.com/docs/configure/keybindings) de VS Code (`Cmd+K Cmd+S`), recherchez `Claude Code: Focus input`, et assignez une nouvelle liaison.

<h3 id="claude-code-never-responds">
  Claude Code ne répond jamais
</h3>

Si Claude Code ne répond pas à vos invites :

1. **Vérifier votre connexion Internet** : Assurez-vous que vous avez une connexion Internet stable
2. **Démarrer une nouvelle conversation** : Essayez de démarrer une nouvelle conversation pour voir si le problème persiste
3. **Essayer le CLI** : Exécutez `claude` à partir du terminal pour voir si vous obtenez des messages d'erreur plus détaillés

Si les problèmes persistent, [déposez un problème sur GitHub](https://github.com/anthropics/claude-code/issues) avec des détails sur l'erreur.

<h2 id="uninstall-the-extension">
  Désinstaller l'extension
</h2>

Pour désinstaller l'extension Claude Code :

1. Ouvrez la vue Extensions (`Cmd+Shift+X` sur Mac ou `Ctrl+Shift+X` sur Windows/Linux)
2. Recherchez « Claude Code »
3. Cliquez sur **Désinstaller**

L'exécution de `claude` dans un terminal intégré VS Code réinstalle l'extension automatiquement. Pour la maintenir désinstallée, désactivez **Auto-install IDE extension** dans `/config`, ou définissez [`autoInstallIdeExtension`](/docs/fr/settings#global-config-settings) sur `false`. Vous pouvez également définir la variable d'environnement [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](/docs/fr/env-vars) sur `1`.

Pour également supprimer les données d'extension et réinitialiser tous les paramètres, supprimez le répertoire de stockage de l'extension pour votre plateforme.

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

Pour une aide supplémentaire, consultez le [guide de dépannage](/docs/fr/troubleshooting).

<h2 id="next-steps">
  Étapes suivantes
</h2>

Maintenant que vous avez Claude Code configuré dans VS Code :

* [Explorez les flux de travail courants](/docs/fr/common-workflows) pour tirer le meilleur parti de Claude Code
* [Configurez les serveurs MCP](/docs/fr/mcp) pour étendre les capacités de Claude avec des outils externes. Ajoutez des serveurs en utilisant le CLI, puis gérez-les avec `/mcp` dans le panneau de chat.
* [Configurez les paramètres Claude Code](/docs/fr/settings) pour personnaliser les commandes autorisées, les hooks et bien d'autres. Ces paramètres sont partagés entre l'extension et le CLI.
