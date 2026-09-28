> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Application de bureau

> Tirez le meilleur parti de Claude Code Desktop : sessions parallèles avec isolation Git, disposition des volets par glisser-déposer, terminal intégré et éditeur de fichiers, chats latéraux, utilisation informatique, sessions Dispatch depuis votre téléphone, examen visuel des différences, aperçus d'applications, surveillance des PR, connecteurs et configuration d'entreprise.

L'application Claude Desktop a trois onglets : **Chat** pour les conversations, **Cowork** pour [Dispatch et les travaux agentiques plus longs](https://claude.com/product/cowork), et **Code** pour le développement logiciel. Cette page est la référence pour l'onglet Code.

<CardGroup cols={3}>
  <Card title="Télécharger pour macOS" icon="apple" href="https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code&utm_medium=docs">
    Build universel pour Intel et Apple Silicon
  </Card>

  <Card title="Télécharger pour Windows" icon="windows" href="https://claude.ai/api/desktop/win32/x64/setup/latest/redirect?utm_source=claude_code&utm_medium=docs">
    Pour les processeurs x64
  </Card>

  <Card title="Obtenir Claude pour Linux (bêta)" icon="linux" href="/docs/fr/desktop-linux">
    apt ou .deb pour Ubuntu et Debian
  </Card>
</CardGroup>

Pour Windows ARM64, téléchargez l'[installateur ARM64](https://claude.ai/api/desktop/win32/arm64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs). Sur Linux, installez avec apt ; voir [Claude Desktop sur Linux](/docs/fr/desktop-linux).

Après l'installation, lancez Claude, connectez-vous et cliquez sur l'onglet **Code**. Pour une présentation de votre première session, consultez le [guide Démarrer](/docs/fr/desktop-quickstart).

Dans l'onglet Code, chaque conversation est une **session** : elle a son propre historique de chat et dossier de projet, indépendants de toute autre session. La barre latérale répertorie vos sessions et vous permet d'en exécuter plusieurs en parallèle. Au sein d'une session, vous pouvez :

* [Examiner et commenter les différences](#review-changes-with-diff-view), puis [surveiller la PR résultante via CI](#monitor-pull-request-status)
* [Prévisualiser votre application en cours d'exécution](#preview-your-app) dans le volet Navigateur tandis que Claude vérifie ses propres modifications, et [ouvrir des sites externes](#browse-external-sites) à côté
* Regarder Claude [exécuter et tester votre application iOS](/docs/fr/desktop-ios-simulator) dans le volet iOS Simulator
* [Arranger les volets](#arrange-your-workspace) pour le chat, la différence, le navigateur, le terminal et l'éditeur de fichiers côte à côte
* Poser une [question latérale](#ask-a-side-question-without-derailing-the-session) qui utilise le contexte de la session sans la dérailler
* Laisser Claude [vérifier, envoyer un message ou archiver vos autres sessions](#work-across-sessions)
* [Connecter des outils externes](#connect-external-tools) comme GitHub, Slack et Linear
* Laisser Claude [ouvrir des applications et contrôler votre écran](#let-claude-use-your-computer)
* Exécuter sur votre machine, dans le [cloud](#run-long-running-tasks-in-the-cloud), ou via [SSH](#ssh-sessions)

Pour [les travaux récurrents planifiés](/docs/fr/desktop-scheduled-tasks), [les raccourcis clavier](#keyboard-shortcuts), ou [l'envoi de tâches depuis votre téléphone](#sessions-from-dispatch), consultez les pages et sections liées. Si vous utilisez déjà le CLI basé sur le terminal, consultez la [comparaison CLI](#coming-from-the-cli) pour voir ce qui est transféré.

<h2 id="start-a-session">
  Démarrer une session
</h2>

Avant d'envoyer votre premier message, configurez quatre choses dans la zone de prompt :

* **Environnement** : choisissez où Claude s'exécute. Sélectionnez **Local** pour votre machine, **Cloud** pour une [session cloud](#cloud-sessions) qui continue après la fermeture de l'application, une [**connexion SSH**](#ssh-sessions) pour une machine distante que vous gérez, ou sur Windows une [**distribution WSL**](/docs/fr/desktop-wsl). Voir [configuration de l'environnement](#environment-configuration).
* **Dossier du projet** : sélectionnez le dossier ou le référentiel dans lequel Claude travaille. Pour les sessions cloud, vous pouvez ajouter [plusieurs référentiels](#run-long-running-tasks-in-the-cloud).
* **Modèle** : choisissez un [modèle](/docs/fr/model-config#available-models) dans la liste déroulante à côté du bouton d'envoi. Vous pouvez modifier ceci pendant la session.
* **Mode de permission** : choisissez le niveau d'autonomie de Claude à partir du [sélecteur de mode](#choose-a-permission-mode). Vous pouvez modifier ceci pendant la session.

Tapez votre tâche et appuyez sur **Entrée** pour démarrer. Chaque session suit son propre contexte et les modifications indépendamment.

<h2 id="work-with-code">
  Travailler avec le code
</h2>

Donnez à Claude le bon contexte, contrôlez le volume de travail qu'il effectue seul et examinez ce qu'il a modifié.

<h3 id="use-the-prompt-box">
  Utiliser la zone de prompt
</h3>

Tapez ce que vous voulez que Claude fasse et appuyez sur **Entrée** pour envoyer. Claude lit vos fichiers de projet, effectue des modifications et exécute des commandes en fonction de votre [mode de permission](#choose-a-permission-mode). Vous pouvez rediriger Claude à tout moment : cliquez sur le bouton d'arrêt pour interrompre immédiatement, ou tapez une correction et appuyez sur **Entrée** pour l'envoyer sans arrêter l'action en cours. Claude lit la correction dès que l'action actuelle se termine et s'ajuste avant son étape suivante.

Le bouton **+** à côté de la zone de prompt vous donne accès aux pièces jointes de fichiers, [skills](#use-skills), [connecteurs](#connect-external-tools) et [plugins](#install-plugins).

<h3 id="add-files-and-context-to-prompts">
  Ajouter des fichiers et du contexte aux prompts
</h3>

La zone de prompt supporte deux façons d'apporter du contexte externe :

* **Fichiers @mention** : tapez `@` suivi d'un nom de fichier pour ajouter un fichier au contexte de la conversation. Claude peut alors lire et référencer ce fichier. @mention n'est pas disponible dans les sessions cloud ou WSL.
* **Joindre des fichiers** : joignez des images, des PDF et d'autres fichiers à votre prompt en utilisant le bouton de pièce jointe, ou glissez-déposez les fichiers directement dans le prompt. Ceci est utile pour partager des captures d'écran de bugs, des maquettes de conception ou des documents de référence.

<h3 id="choose-a-permission-mode">
  Choisir un mode de permission
</h3>

Les modes de permission contrôlent le niveau d'autonomie de Claude pendant une session : s'il demande avant de modifier des fichiers, d'exécuter des commandes ou les deux. Vous pouvez changer de mode à tout moment en utilisant le sélecteur de mode à côté du bouton d'envoi. Pour approuver chaque modification vous-même, passez à Manuel.

Pour définir un mode par défaut pour les nouvelles sessions locales, ajoutez `permissions.defaultMode` à votre [fichier de paramètres](/docs/fr/settings#where-settings-live). L'application de bureau lit les mêmes fichiers de paramètres que la CLI. Un mode que vous choisissez dans le sélecteur est mémorisé par dossier et prend la priorité sur `defaultMode` pour ce dossier, sauf Plan, qui s'applique à la session actuelle uniquement.

| Mode                           | Clé de paramètres   | Comportement                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ------------------------------ | ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Manuel**                     | `default`           | Claude demande avant de modifier des fichiers ou d'exécuter des commandes. Vous voyez une différence et pouvez accepter ou rejeter chaque modification.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| **Accepter les modifications** | `acceptEdits`       | Claude accepte automatiquement les modifications de fichiers et les commandes courantes du système de fichiers comme `mkdir`, `touch` et `mv`, mais demande toujours avant d'exécuter les autres commandes du terminal. Utilisez ceci quand vous faites confiance aux modifications de fichiers et voulez une itération plus rapide.                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| **Plan**                       | `plan`              | Claude lit les fichiers et exécute les commandes pour explorer, puis propose un plan sans modifier votre code source. Bon pour les tâches complexes où vous voulez examiner l'approche en premier.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| **Auto**                       | `auto`              | Claude exécute toutes les actions avec des vérifications de sécurité en arrière-plan qui vérifient l'alignement avec votre demande. Réduit les invites de permission tout en maintenant la surveillance. Apparaît quand [auto mode est disponible](#auto-mode-availability) ; il n'y a pas de bascule Paramètres séparée pour cela.                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| **Contourner les permissions** | `bypassPermissions` | Claude s'exécute sans invites de permission, sauf pour les [actions qu'aucun mode n'approuve automatiquement](/docs/fr/permission-modes#actions-no-mode-auto-approves), les classificateurs de sécurité quand Claude [agit sur des sites externes](#browse-external-sites), ou les actions de bureau où Claude demande toujours en premier, comme [archiver une session](#work-across-sessions). Équivalent à `--dangerously-skip-permissions` dans la CLI. Sur les plans Pro et Max, activez-le dans vos Paramètres → Claude Code sous « Autoriser le mode de contournement des permissions » ; sur les plans Team et Enterprise, il n'y a pas de bascule Paramètres, et la politique organisationnelle le contrôle à la place. Utilisez uniquement dans les conteneurs sandboxés ou les machines virtuelles. |

Les versions antérieures de l'onglet Code étiquetaient ces modes Demander les permissions, Accepter automatiquement les modifications et Plan mode.

Le mode de permission `dontAsk` est disponible uniquement dans la [CLI](/docs/fr/permission-modes#allow-only-pre-approved-tools-with-dontask-mode).

<span id="auto-mode-availability" />

Auto mode est disponible à tous les utilisateurs sur l'API Anthropic et nécessite Claude Opus 4.6 ou version ultérieure, Sonnet 4.6 ou version ultérieure, ou un [modèle Fable](/docs/fr/model-config#work-with-fable). Les administrateurs d'organisation peuvent désactiver auto mode avec la clé `disableAutoMode` dans les [paramètres gérés](#managed-settings).

Dans les déploiements Enterprise qui acheminent Desktop vers Google Cloud's Agent Platform, auto mode est également disponible par défaut ; voir [Auto mode sur Bedrock, Agent Platform ou Foundry](/docs/fr/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry) pour les modèles supportés.

<Tip title="Bonne pratique">
  Commencez les tâches complexes en Plan pour que Claude trace une approche avant de faire des modifications. Une fois que vous approuvez le plan, passez à Accepter les modifications ou Manuel pour l'exécuter. Voir [explorer d'abord, puis planifier, puis coder](/docs/fr/best-practices#explore-first-then-plan-then-code) pour plus d'informations sur ce flux de travail.
</Tip>

Les sessions cloud supportent Accepter les modifications, Plan et Auto. Accepter les modifications correspond au mode `default` : les sessions cloud pré-approuvent les modifications de fichiers, donc le sélecteur affiche Accepter les modifications au lieu de Manuel. Contourner les permissions n'est pas disponible dans les sessions cloud, y compris les sessions dans un [environnement auto-hébergé](/docs/fr/self-hosted-environments).

Les administrateurs d'entreprise peuvent restreindre les modes de permission disponibles. Voir [configuration d'entreprise](#enterprise-configuration) pour les détails.

<h3 id="preview-your-app">
  Aperçu de votre application
</h3>

Claude peut démarrer un serveur de développement et l'ouvrir dans le volet Navigateur pour vérifier ses modifications. Ceci fonctionne pour les applications web frontend ainsi que les serveurs backend : Claude peut tester les points de terminaison API, afficher les journaux du serveur et itérer sur les problèmes qu'il trouve. Dans la plupart des cas, Claude démarre le serveur automatiquement après la modification des fichiers du projet. Vous pouvez également demander à Claude de prévisualiser à tout moment. Par défaut, Claude [vérifie automatiquement](#auto-verify-changes) les modifications après chaque modification.

Le volet Navigateur peut également ouvrir des fichiers HTML statiques, des PDF, des images et des vidéos de votre projet. Cliquez sur un chemin HTML, PDF, image ou vidéo dans le chat pour l'ouvrir là.

À partir du volet Navigateur, vous pouvez :

* Interagir avec votre application en cours d'exécution directement dans le volet Navigateur
* Regarder Claude vérifier ses propres modifications automatiquement : il prend des captures d'écran, inspecte le DOM, clique sur les éléments, remplit les formulaires et corrige les problèmes qu'il trouve
* Démarrer ou arrêter les serveurs à partir de la liste déroulante des serveurs dans la barre d'outils de la session
* Conserver les cookies et le stockage local entre les redémarrages du serveur en sélectionnant **Conserver les sessions** dans la liste déroulante, afin que vous n'ayez pas à vous reconnecter pendant le développement
* Modifier la configuration du serveur ou arrêter tous les serveurs à la fois

Claude crée la configuration initiale du serveur en fonction de votre projet. Si votre application utilise une commande de développement personnalisée, modifiez `.claude/launch.json` pour correspondre à votre configuration. Voir [Configurer les serveurs d'aperçu](#configure-preview-servers) pour la référence complète.

Pour effacer les données de session enregistrées, ou pour désactiver complètement le Navigateur, utilisez les bascules dans Paramètres → Claude Code.

<h3 id="browse-external-sites">
  Parcourir les sites externes
</h3>

Le volet Navigateur est un navigateur à onglets, vous pouvez donc ouvrir la documentation, les suivi de problèmes ou tout autre site à côté de votre application en cours d'exécution. Pour ouvrir le Navigateur, appuyez sur **Cmd+Maj+B** sur macOS ou **Ctrl+Maj+B** sur Windows, ou sélectionnez-le dans le menu **Affichages**. Quand vous cliquez sur un lien externe dans le chat, un sélecteur offre **Ouvrir dans l'application** pour utiliser le volet Navigateur ou **Navigateur par défaut** pour utiliser le vôtre ; **Cmd**-cliquez sur macOS ou **Ctrl**-cliquez sur Windows ouvre un lien dans votre navigateur système directement. Vous pouvez vous connecter à des sites dans le volet, y compris les flux de connexion popup tels que Google OAuth.

Claude peut lire et interagir avec les pages externes en utilisant les mêmes outils qu'il utilise pour [vérifier votre application](#preview-your-app), avec deux vérifications de sécurité supplémentaires :

* Les classificateurs de sécurité examinent les actions d'écriture de Claude sur les pages externes, telles que cliquer et taper, dans tous les modes de permission. Ce sont les mêmes classificateurs que [auto mode](#choose-a-permission-mode) utilise, et quand ils signalent une action, vous obtenez une invite de permission quel que soit le mode.
* Dans les modes de permission autres que Auto et Contourner les permissions, une vérification de liste d'autorisation de domaine s'applique également avant que Claude ne navigue vers un nouveau site.

<h4 id="approve-claude’s-actions-on-a-site">
  Approuver les actions de Claude sur un site
</h4>

La première fois que Claude agit sur un site externe, une carte de permission apparaît et Claude attend votre choix : **Autoriser une fois**, **Toujours autoriser** ou **Refuser**. **Autoriser une fois** approuve l'action sans rien enregistrer. **Toujours autoriser** enregistre l'approbation pour ce site sur votre appareil, et vous pouvez la révoquer dans Paramètres. Chaque site a besoin de sa propre approbation, y compris les sous-domaines. Vos serveurs de développement locaux et fichiers de projet n'ont pas besoin d'approbation, donc [auto-verify](#auto-verify-changes) continue de fonctionner sans invites.

Même sur un site approuvé, Claude n'achètera pas d'articles, ne créera pas de comptes ou ne contournera pas les CAPTCHA sans votre entrée. La navigation dans le volet Navigateur utilise le même modèle de sécurité que l'[extension Claude dans Chrome](/docs/fr/chrome). Voir [Utiliser Claude dans Chrome en toute sécurité](https://support.claude.com/en/articles/12902428-using-claude-in-chrome-safely) pour savoir comment Claude gère les sites sensibles et les actions risquées.

<h4 id="choose-between-the-browser-and-the-chrome-extension">
  Choisir entre le Navigateur et l'extension Chrome
</h4>

Le volet Navigateur utilise un profil de navigateur propre, séparé de votre navigateur personnel, sans aucune de vos connexions enregistrées ou historique. Utilisez-le pour construire et tester votre application et pour les sites qui n'ont pas besoin de votre identité. Quand vous voulez que Claude agisse en tant que vous dans vos sessions connectées, utilisez l'[extension Claude dans Chrome](/docs/fr/chrome) à la place, qui partage l'état de connexion de votre navigateur.

<h4 id="restrict-external-browsing-for-your-organization">
  Restreindre la navigation externe pour votre organisation
</h4>

Le Navigateur suit les mêmes [contrôles de liste d'autorisation et de liste de blocage de site](https://support.claude.com/en/articles/13065128-claude-in-chrome-admin-controls) que l'extension Claude dans Chrome. Si votre organisation a déjà configuré ces listes pour l'extension, le Navigateur les respecte automatiquement. Les administrateurs peuvent également désactiver les outils de Claude sur les pages externes avec le paramètre géré [`browserExternalPageTools`](#managed-settings). Avec les outils désactivés, les utilisateurs peuvent toujours naviguer vers des sites externes ; les outils de Claude ne peuvent pas les lire ou agir sur eux.

Pour désactiver complètement la navigation externe, définissez le paramètre géré [`disableBrowserExternalNavigation`](#managed-settings) sur `true`. Ceci bloque toute navigation externe dans le Navigateur, y compris les sites sur la liste d'autorisation de votre organisation ; les serveurs de développement localhost et les aperçus de fichiers continuent de fonctionner. Utilisez `browserExternalPageTools` pour laisser les utilisateurs continuer à naviguer sur des sites externes sans les outils de Claude, et `disableBrowserExternalNavigation` pour bloquer les sites externes pour les utilisateurs et Claude.

<h3 id="review-changes-with-diff-view">
  Examiner les modifications avec la vue de différence
</h3>

Après que Claude ait modifié votre code, la vue de différence vous permet d'examiner les modifications fichier par fichier avant de créer une demande de tirage.

Quand Claude modifie des fichiers, un indicateur de statistiques de différence apparaît montrant le nombre de lignes ajoutées et supprimées, comme `+12 -1`. Cliquez sur cet indicateur pour ouvrir la visionneuse de différences, qui affiche une liste de fichiers à gauche et les modifications pour chaque fichier à droite.

Pour commenter des lignes spécifiques, cliquez sur n'importe quelle ligne dans la différence pour ouvrir une boîte de commentaire. Tapez votre retour et appuyez sur **Entrée** pour ajouter le commentaire. Après avoir ajouté des commentaires à plusieurs lignes, soumettez tous les commentaires à la fois :

* **macOS** : appuyez sur **Cmd+Entrée**
* **Windows** : appuyez sur **Ctrl+Entrée**

Claude lit vos commentaires et effectue les modifications demandées, qui apparaissent comme une nouvelle différence que vous pouvez examiner.

<h3 id="review-your-code">
  Examiner votre code
</h3>

Dans la vue de différence, cliquez sur **Examiner le code** dans la barre d'outils en haut à droite pour demander à Claude d'évaluer les modifications avant de les valider. Claude examine les différences actuelles et laisse des commentaires directement dans la vue de différence. Vous pouvez répondre à n'importe quel commentaire ou demander à Claude de réviser.

L'examen se concentre sur les problèmes à haut signal : erreurs de compilation, erreurs logiques définies, vulnérabilités de sécurité et bugs évidents. Il ne signale pas le style, le formatage, les problèmes préexistants ou quoi que ce soit qu'un linter attraperait.

<h3 id="monitor-pull-request-status">
  Surveiller l'état de la demande de tirage
</h3>

Après avoir ouvert une demande de tirage, une barre d'état CI apparaît dans la session. Claude Code utilise la CLI GitHub pour interroger les résultats des vérifications et afficher les défaillances.

* **Correction automatique** : quand activée, Claude tente automatiquement de corriger les vérifications CI défaillantes en lisant la sortie de défaillance et en itérant.
* **Fusion automatique** : quand activée, Claude fusionne la PR une fois que toutes les vérifications réussissent. La méthode de fusion est squash. La fusion automatique doit être [activée dans les paramètres de votre référentiel GitHub](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-auto-merge-for-pull-requests-in-your-repository) pour que cela fonctionne.

Utilisez les bascules **Correction automatique** et **Fusion automatique** dans la barre d'état CI pour activer l'une ou l'autre option. Claude Code envoie également une notification de bureau quand CI se termine. Pour archiver la session automatiquement une fois que la PR fusionne ou se ferme, activez [auto-archive](#work-in-parallel-with-sessions) dans Paramètres → Claude Code.

<Note>
  La surveillance des PR nécessite que la [CLI GitHub (`gh`)](https://cli.github.com/) soit installée et authentifiée sur votre machine. Si `gh` n'est pas installée, Desktop vous invite à l'installer la première fois que vous essayez de créer une PR.
</Note>

<h2 id="arrange-your-workspace">
  Arranger votre espace de travail
</h2>

L'onglet Code est construit autour de volets que vous pouvez arranger dans n'importe quelle disposition : chat, différence, aperçu, terminal, fichier, plan, tâches et sous-agent, ainsi que le [Simulateur iOS](/docs/fr/desktop-ios-simulator) sur macOS. Glissez un volet par son en-tête pour le repositionner, ou glissez un bord de volet pour le redimensionner. Appuyez sur **Cmd+\\** sur macOS ou **Ctrl+\\** sur Windows pour fermer le volet actif. Ouvrez des volets supplémentaires à partir du menu **Vues** dans la barre d'outils de la session.

Pour travailler sur plusieurs écrans, extrayez un volet tel que la différence ou le terminal dans sa propre fenêtre, et redockez-le quand vous avez terminé. Claude continue de travailler dans la fenêtre principale.

<Note>
  La disposition des volets, le terminal, l'éditeur de fichiers et les modes d'affichage dans cette section nécessitent Claude Desktop v1.2581.0 ou une version ultérieure. Ouvrez **Claude → Vérifier les mises à jour** sur macOS ou **Aide → Vérifier les mises à jour** sur Windows pour mettre à jour.
</Note>

<h3 id="run-commands-in-the-terminal">
  Exécuter les commandes dans le terminal
</h3>

Le terminal intégré vous permet d'exécuter les commandes aux côtés de votre session sans basculer vers une autre application. Ouvrez-le à partir du menu **Vues** ou appuyez sur **Ctrl+\`** sur macOS ou Windows. Le terminal s'ouvre dans le répertoire de travail de votre session et partage le même environnement que Claude, donc les commandes comme `npm test` ou `git status` voient les mêmes fichiers que Claude édite. Pour ouvrir un deuxième onglet de terminal, cliquez sur **+** dans l'en-tête du volet de terminal ou cliquez avec le bouton droit sur un dossier dans le chat pour choisir **Ouvrir dans le terminal**. Le terminal est disponible dans les sessions locales uniquement.

<h3 id="open-and-edit-files">
  Ouvrir et modifier les fichiers
</h3>

Cliquez sur un chemin de fichier dans le chat ou la visionneuse de différences pour l'ouvrir dans le volet de fichier. Les chemins HTML, PDF, image et vidéo s'ouvrent dans le [volet d'aperçu](#preview-your-app) à la place. Effectuez des modifications ponctuelles et cliquez sur **Enregistrer** pour les écrire. Si le fichier a changé sur le disque depuis que vous l'avez ouvert, le volet vous avertit et vous permet de remplacer ou d'abandonner. Cliquez sur **Abandonner** pour annuler vos modifications, ou cliquez sur le chemin dans l'en-tête du volet pour copier le chemin absolu.

Le volet de fichier est disponible dans les sessions locales et SSH. Pour les sessions distantes, demandez à Claude de faire la modification.

<h3 id="open-files-in-other-apps">
  Ouvrir les fichiers dans d'autres applications
</h3>

Cliquez avec le bouton droit sur n'importe quel chemin de fichier dans le chat, la visionneuse de différences ou le volet de fichier pour ouvrir un menu contextuel :

* **Joindre comme contexte** : ajouter le fichier à votre prochain prompt
* **Ouvrir dans** : ouvrir le fichier dans un éditeur installé comme VS Code, Cursor ou Zed
* **Afficher dans le Finder** sur macOS, **Afficher dans l'Explorateur** sur Windows : ouvrir le dossier contenant
* **Copier le chemin** : copier le chemin absolu dans votre presse-papiers

<h3 id="switch-view-modes">
  Changer les modes d'affichage
</h3>

Les modes d'affichage contrôlent le niveau de détail qui apparaît dans la transcription du chat. Changez les modes à partir de la liste déroulante **Vue de la transcription** à côté du bouton d'envoi, ou appuyez sur **Ctrl+O** sur macOS ou Windows pour les parcourir. Le mode Thinking apparaît dans la liste déroulante uniquement après que Claude a produit du thinking dans la session que vous consultez.

| Mode         | Ce qu'il affiche                                                                                             |
| ------------ | ------------------------------------------------------------------------------------------------------------ |
| **Normal**   | Les appels d'outils réduits en résumés, avec les réponses texte complètes                                    |
| **Thinking** | Les appels d'outils réduits en résumés, plus le thinking de Claude                                           |
| **Verbose**  | Chaque appel d'outil, lecture de fichier et étape intermédiaire que Claude prend, plus le thinking de Claude |

Utilisez Thinking pour suivre le raisonnement de Claude avec les appels d'outils toujours réduits. Utilisez Verbose lors du débogage pour savoir pourquoi Claude a pris une action particulière. Les versions de Claude Desktop antérieures à 1.46388.1 listent également un mode Résumé, et une session toujours définie sur Résumé s'ouvre en Normal une fois que vous mettez à jour.

<h3 id="keyboard-shortcuts">
  Raccourcis clavier
</h3>

Appuyez sur **Cmd+/** sur macOS ou **Ctrl+/** sur Windows pour voir tous les raccourcis disponibles dans l'onglet Code. Sur Windows, utilisez **Ctrl** à la place de **Cmd** pour les raccourcis ci-dessous. Le cycle des sessions, le basculement du terminal et le basculement du mode d'affichage utilisent **Ctrl** sur chaque plateforme.

| Raccourci                             | Action                                      |
| ------------------------------------- | ------------------------------------------- |
| `Cmd` `/`                             | Afficher les raccourcis clavier             |
| `Cmd` `N`                             | Nouvelle session                            |
| `Cmd` `W`                             | Fermer la session                           |
| `Ctrl` `Tab` / `Ctrl` `Shift` `Tab`   | Session suivante ou précédente              |
| `Cmd` `Shift` `]` / `Cmd` `Shift` `[` | Session suivante ou précédente              |
| `Esc`                                 | Arrêter la réponse de Claude                |
| `Cmd` `Shift` `D`                     | Basculer le volet de différence             |
| `Cmd` `Shift` `B`                     | Basculer le volet d'aperçu                  |
| `Cmd` `Shift` `S`                     | Sélectionner un élément dans l'aperçu       |
| `Ctrl` `` ` ``                        | Basculer le volet de terminal               |
| `Cmd` `\`                             | Fermer le volet actif                       |
| `Cmd` `;`                             | Ouvrir le chat latéral                      |
| `Ctrl` `O`                            | Parcourir les modes d'affichage             |
| `Cmd` `Shift` `M`                     | Ouvrir le menu du mode de permission        |
| `Cmd` `Shift` `I`                     | Ouvrir le menu du modèle                    |
| `Cmd` `Shift` `E`                     | Ouvrir le menu d'effort                     |
| `1`–`9`                               | Sélectionner un élément dans un menu ouvert |

Ces raccourcis s'appliquent uniquement à l'onglet Code. Les raccourcis du [mode interactif](/docs/fr/interactive-mode#keyboard-shortcuts) basés sur le terminal, comme `Shift+Tab` pour parcourir les modes de permission, ne s'appliquent pas dans Desktop.

<h3 id="check-usage">
  Vérifier l'utilisation
</h3>

Cliquez sur l'anneau d'utilisation à côté du sélecteur de modèle pour voir votre utilisation actuelle de la fenêtre de contexte et votre utilisation du plan pour la période. L'utilisation du contexte est par session ; l'utilisation du plan est partagée sur toutes vos surfaces Claude Code.

<h2 id="let-claude-use-your-computer">
  Laisser Claude utiliser votre ordinateur
</h2>

L'utilisation informatique permet à Claude d'ouvrir vos applications, de contrôler votre écran et de travailler directement sur votre machine comme vous le feriez. Demandez à Claude d'interagir avec un outil de bureau qui n'a pas de CLI, ou d'automatiser quelque chose qui ne fonctionne que via une GUI. Pour exécuter et tester des applications iOS, Desktop ouvre le volet dédié [iOS Simulator](/docs/fr/desktop-ios-simulator) au lieu de contrôler votre écran ; le volet fonctionne sans activer l'utilisation informatique.

<Note>
  L'utilisation informatique est un aperçu de recherche sur macOS et Windows qui nécessite un plan Pro ou Max. Elle n'est pas disponible sur les plans Team ou Enterprise. L'application Claude Desktop doit être en cours d'exécution.
</Note>

L'utilisation informatique est désactivée par défaut. [Activez-la dans Paramètres](#enable-computer-use) avant que Claude puisse contrôler votre écran. Sur macOS, vous devez également accorder les permissions d'Accessibilité et d'Enregistrement d'écran.

Sur macOS, l'utilisation informatique peut également s'exécuter en arrière-plan : Claude travaille dans les applications que vous avez approuvées pendant que vous continuez à travailler.

<Warning>
  Contrairement à l'[outil Bash sandboxé](/docs/fr/sandboxing), l'utilisation informatique s'exécute sur votre vrai bureau avec accès à tout ce que vous approuvez. Claude vérifie chaque action et signale les injections de prompt potentielles du contenu à l'écran, mais la limite de confiance est différente. Voir le [guide de sécurité de l'utilisation informatique](https://support.claude.com/en/articles/14128542) pour les meilleures pratiques.
</Warning>

<h3 id="when-computer-use-applies">
  Quand l'utilisation informatique s'applique
</h3>

Claude a plusieurs façons d'interagir avec une application ou un service, et l'utilisation informatique est la plus large et la plus lente. Il essaie d'abord l'outil le plus précis :

* Si vous avez un [connecteur](#connect-external-tools) pour un service, Claude utilise le connecteur.
* Si la tâche est une commande shell, Claude utilise Bash.
* Si la tâche est du travail de navigateur et que vous avez [Claude dans Chrome](/docs/fr/chrome) configuré, Claude utilise cela.
* Si la tâche consiste à exécuter ou tester une application iOS, Claude utilise le [volet iOS Simulator](/docs/fr/desktop-ios-simulator), qui n'utilise pas le contrôle d'écran.
* Si aucun de ceux-ci ne s'applique, Claude utilise l'utilisation informatique.

Les [niveaux d'accès par application](#app-permissions) renforcent ceci : les navigateurs sont limités à la lecture seule, et les terminaux et IDE à clic uniquement, guidant Claude vers l'outil dédié même quand l'utilisation informatique est active. Le contrôle d'écran est réservé aux choses que rien d'autre ne peut atteindre, comme les applications natives, les panneaux de contrôle matériel, ou les outils propriétaires sans API.

<h3 id="enable-computer-use">
  Activer l'utilisation informatique
</h3>

L'utilisation informatique est désactivée par défaut. Si vous demandez à Claude de faire quelque chose qui en a besoin alors qu'elle est désactivée, Claude vous dit qu'il pourrait faire la tâche si vous activez l'utilisation informatique dans Paramètres.

<Steps>
  <Step title="Mettre à jour l'application de bureau">
    Assurez-vous que vous avez la dernière version de Claude Desktop. Sur macOS et Windows, téléchargez ou mettez à jour sur [claude.com/download](https://claude.com/download) ; sur Linux, mettez à jour via votre gestionnaire de paquets ([instructions](/docs/fr/desktop-linux)). Ensuite, redémarrez l'application.
  </Step>

  <Step title="Activer le basculement">
    Dans l'application de bureau, allez à **Paramètres > Général** (sous **Application de bureau**). Trouvez le basculement **Utilisation informatique** et activez-le. Sur Windows, le basculement prend effet immédiatement et la configuration est complète. Sur macOS, continuez à l'étape suivante.

    Si vous ne voyez pas le basculement, confirmez que vous êtes sur macOS ou Windows avec un plan Pro ou Max, puis mettez à jour et redémarrez l'application.
  </Step>

  <Step title="Accorder les permissions macOS">
    Sur macOS, accordez deux permissions système avant que le basculement prenne effet :

    * **Accessibilité** : permet à Claude de cliquer, taper et faire défiler
    * **Enregistrement d'écran** : permet à Claude de voir ce qui est sur votre écran

    La page Paramètres affiche l'état actuel de chaque permission. Si l'une est refusée, cliquez sur le badge pour ouvrir le volet Paramètres système pertinent.
  </Step>
</Steps>

<h3 id="app-permissions">
  Permissions d'application
</h3>

La première fois que Claude doit utiliser une application, une invite apparaît dans votre session. Cliquez sur **Autoriser pour cette session** ou **Refuser**. Les approbations durent pour la session actuelle, ou 30 minutes dans les [sessions générées par Dispatch](#sessions-from-dispatch).

L'invite affiche également quel niveau de contrôle Claude obtient pour cette application. Ces niveaux sont fixés par catégorie d'application et ne peuvent pas être modifiés :

| Niveau           | Ce que Claude peut faire                                                    | S'applique à                        |
| :--------------- | :-------------------------------------------------------------------------- | :---------------------------------- |
| Lecture seule    | Voir l'application dans les captures d'écran                                | Navigateurs, plateformes de trading |
| Clic uniquement  | Cliquer et faire défiler, mais pas taper ou utiliser les raccourcis clavier | Terminaux, IDE                      |
| Contrôle complet | Cliquer, taper, glisser et utiliser les raccourcis clavier                  | Tout le reste                       |

Les applications avec une large portée, comme les terminaux, Finder ou Explorateur de fichiers, et Paramètres système ou Paramètres, affichent un avertissement supplémentaire dans l'invite pour que vous sachiez ce que l'approbation accorde.

Vous pouvez configurer deux paramètres dans **Paramètres > Général** (sous **Application de bureau**) :

* **Applications refusées** : ajoutez des applications ici pour les rejeter sans demander. Claude peut toujours affecter une application refusée indirectement via des actions dans une application autorisée, mais il ne peut pas interagir directement avec l'application refusée.
* **Afficher les applications quand Claude a terminé** : tandis que l'utilisation informatique ne s'exécute pas en arrière-plan, Claude masque vos autres fenêtres pendant qu'il travaille pour qu'il n'interagisse qu'avec l'application approuvée. Quand Claude a terminé, les fenêtres masquées sont restaurées sauf si vous désactivez ce paramètre.

<h2 id="manage-sessions">
  Gérer les sessions
</h2>

Chaque session est une conversation indépendante avec son propre contexte et ses propres modifications. Vous pouvez exécuter plusieurs sessions en parallèle, créer des chats latéraux, laisser Claude vérifier et envoyer des messages à vos autres sessions, envoyer du travail vers le cloud, ou laisser Dispatch démarrer des sessions pour vous depuis votre téléphone.

<h3 id="work-in-parallel-with-sessions">
  Travailler en parallèle avec les sessions
</h3>

Cliquez sur **+ Nouvelle session** dans la barre latérale, ou appuyez sur **Cmd+N** sur macOS ou **Ctrl+N** sur Windows, pour travailler sur plusieurs tâches en parallèle. Appuyez sur **Ctrl+Tab** et **Ctrl+Shift+Tab** pour parcourir les sessions dans la barre latérale. Pour les référentiels Git, sélectionnez l'option **worktree** à côté du nom de la branche pour donner à la session sa propre copie isolée de votre projet en utilisant [Git worktrees](/docs/fr/worktrees), donc les modifications dans une session n'affectent pas les autres sessions jusqu'à ce que vous les validiez.

Pour afficher deux sessions à la fois, maintenez **Cmd** sur macOS ou **Ctrl** sur Windows et cliquez sur une session dans la barre latérale. La session s'ouvre dans un deuxième volet à côté de celui que vous avez déjà ouvert. Pendant que la division est active, cliquer sur une autre session de la barre latérale remplace le volet qui a le focus. Appuyez sur **Cmd+\\** sur macOS ou **Ctrl+\\** sur Windows pour fermer le volet actif et revenir à une seule session.

Les worktrees sont stockés dans `<project-root>/.claude/worktrees/` par défaut. Vous pouvez modifier ceci en un répertoire personnalisé dans Paramètres → Claude Code sous « Emplacement du worktree ». Vous pouvez également définir un préfixe de branche qui est ajouté au début de chaque nom de branche worktree, ce qui est utile pour garder les branches créées par Claude organisées. Pour supprimer un worktree quand vous avez terminé, survolez la session dans la barre latérale et cliquez sur l'icône d'archive. Pour avoir les sessions s'archiver elles-mêmes quand leur demande de tirage fusionne ou se ferme, activez **Auto-archive après fusion ou fermeture de PR** dans Paramètres → Claude Code. L'auto-archive s'applique uniquement aux sessions locales qui ont terminé l'exécution.

Pour inclure les fichiers ignorés par git comme `.env` dans les nouveaux worktrees, créez un [fichier `.worktreeinclude`](/docs/fr/worktrees#copy-gitignored-files-into-worktrees) à la racine de votre projet.

<Note>
  L'isolation des sessions nécessite [Git](https://git-scm.com/downloads). La plupart des Macs incluent Git par défaut. Exécutez `git --version` dans Terminal pour vérifier ; s'il affiche un numéro de version, Git est installé. Si vous rencontrez des erreurs Git, demandez à Claude dans l'[onglet Cowork](https://claude.com/product/cowork) de vous aider à dépanner votre configuration.
</Note>

Utilisez les contrôles en haut de la barre latérale pour filtrer les sessions par statut, projet ou environnement, et pour grouper les sessions par projet. Pour renommer une session, cliquez sur le titre de la session dans la barre d'outils en haut de la session active.

Pour vérifier l'utilisation du contexte, voir [Vérifier l'utilisation](#check-usage). Quand le contexte se remplit, Claude résume automatiquement la conversation et continue de travailler. Vous pouvez également taper `/compact` pour déclencher la compaction plus tôt et libérer de l'espace de contexte. Voir [la fenêtre de contexte](/docs/fr/how-claude-code-works#the-context-window) pour les détails sur le fonctionnement de la compaction.

L'application de bureau envoie une notification du système d'exploitation quand une session Code termine une tâche et que vous ne visualisez pas actuellement cette session. Pour les sessions qui appartiennent à un [projet](/docs/fr/claude-projects#see-what-needs-you-in-overview), vous recevez les notifications du projet à la place.

<h3 id="ask-a-side-question-without-derailing-the-session">
  Poser une question latérale sans dérailler la session
</h3>

Un chat latéral vous permet de poser une question à Claude qui utilise le contexte de votre session mais n'ajoute rien à la conversation principale. Utilisez-le quand vous voulez comprendre un morceau de code, vérifier une hypothèse ou explorer une idée sans détourner la session de son cours.

Appuyez sur **Cmd+;** sur macOS ou **Ctrl+;** sur Windows pour ouvrir un chat latéral, ou tapez `/btw` dans la zone de prompt. Le chat latéral peut lire tout ce qui se trouve dans le fil principal jusqu'à ce point. Quand vous avez terminé, fermez le chat latéral et continuez la session principale où vous l'aviez laissée.

Les chats latéraux sont disponibles dans les sessions locales, SSH et WSL. L'application de bureau n'enregistre pas les chats latéraux sur le disque, donc vous ne pouvez pas revenir à un après avoir fermé l'application.

<h3 id="watch-background-tasks">
  Regarder les tâches en arrière-plan
</h3>

Le volet des tâches affiche le travail en arrière-plan s'exécutant dans la session actuelle : sous-agents, commandes shell en arrière-plan et [flux de travail dynamiques](/docs/fr/workflows). Ouvrez-le à partir du menu **Vues** ou glissez-le dans votre disposition.

Cliquez sur n'importe quelle entrée pour voir sa sortie dans le volet du sous-agent ou l'arrêter. Pour voir ce que font les autres sessions, utilisez la [barre latérale](#work-in-parallel-with-sessions), ou demandez à Claude de [vérifier pour vous](#work-across-sessions).

<h3 id="work-across-sessions">
  Travailler entre les sessions
</h3>

Claude peut lister vos autres sessions de l'onglet Code, lire ce que chacune a fait, et envoyer des messages entre elles. Demandez en langage naturel : « quelle session a touché à la refonte d'authentification ? », « à quelle conclusion la session API est-elle arrivée ? », ou « dites à la session des paiements que le schéma a changé ». Vous pouvez également demander à Claude de renommer ou d'archiver une session. Claude archive une session de la même manière que l'icône d'archive de la barre latérale, donc demandez-lui de nettoyer les sessions dont les PR ont fusionné.

À travers cette surface, Claude ne voit que les sessions que l'application de bureau exécute elle-même : locales, [SSH](#ssh-sessions) et [WSL](/docs/fr/desktop-wsl) sessions dans l'onglet Code. Claude ne voit pas les sessions cloud, ou les sessions que vous avez démarrées à partir du CLI du terminal ou de l'extension VS Code, même dans les worktrees du même projet, donc avec neuf worktrees de terminal ouverts et deux sessions de bureau, Claude répondant dans l'une d'elles signale l'une des autres sessions de bureau. Claude ne liste jamais la session à partir de laquelle vous posez la question. Par défaut, il voit les 20 sessions les plus récemment actives et ignore les sessions archivées sauf si vous les demandez. [La messagerie entre sessions](/docs/fr/cross-session-messaging) permet séparément à Claude de envoyer des messages à [vos autres sessions Claude Code](/docs/fr/cross-session-messaging#see-which-sessions-claude-can-reach), y compris les sessions de terminal.

Quand Claude envoie un message à une autre session à travers cette surface, Claude Code l'affiche là comme une carte étiquetée avec le titre de la session d'envoi et un lien de retour, donc vous pouvez toujours dire d'où vient un message. Si la session réceptrice est en cours de tâche, Claude Code retient le message et Claude le lit une fois le travail actuel terminé. Le Claude récepteur peut répondre, et Claude Code livre la réponse en retour à travers cette surface. Claude ne peut pas livrer à une session archivée, et vous dit quand un message ne passe pas.

Claude Code applique quatre comportements de sécurité entre les sessions :

* Avant d'archiver une session, Claude vous demande d'abord. Vous voyez la carte d'approbation dans chaque mode de permission, y compris les permissions Auto et Bypass.
* À travers cette surface, Claude ne peut pas envoyer de messages entre sessions à partir d'une session que personne ne regarde, comme une exécution de tâche planifiée, et ne peut pas livrer de messages dans une.
* Claude Code vérifie chaque message de cette surface par rapport aux [contrôles entrants](/docs/fr/cross-session-messaging#control-inbound-messages) de la session réceptrice, même quand la session réceptrice n'a pas [la messagerie entre sessions](/docs/fr/cross-session-messaging#availability) elle-même. Si vous définissez [`crossSessionInbound`](/docs/fr/settings-reference#crosssessioninbound) à `refuse` dans la session réceptrice, Claude Code supprime les messages de cette surface. Claude Code signale le refus à l'application de bureau Claude. Avant v2.1.234, Claude Code supprimait chaque message de cette surface vers une session réceptrice sans messagerie entre sessions.
* Claude Code cite chaque message entrant et l'attribue à la session qui l'a envoyé, et Claude suit toujours les paramètres de permission propres de la session réceptrice quand il agit sur un.

Claude peut également suggérer de nouvelles sessions. Quand il remarque quelque chose qui vaut la peine d'être corrigé et qui est hors de portée pour la tâche actuelle, il propose le travail comme une puce de tâche dans le chat. Cliquez sur la puce pour démarrer ce travail dans une nouvelle session avec son propre worktree ; Claude continue votre session actuelle sans interruption.

<h3 id="run-long-running-tasks-in-the-cloud">
  Exécuter les tâches longues dans le cloud
</h3>

Pour les refactorisations importantes, les suites de tests, les migrations ou autres tâches longues, sélectionnez **Cloud** au lieu de **Local** au démarrage d'une session. Les sessions cloud s'exécutent sur l'infrastructure gérée par Anthropic par défaut et continuent même si vous fermez l'application ou arrêtez votre ordinateur. Revenez à tout moment pour voir la progression ou orienter Claude dans une direction différente. Vous pouvez également surveiller les sessions cloud à partir de [claude.ai/code](https://claude.ai/code) ou de l'[application Claude mobile](/docs/fr/mobile).

Les sessions cloud supportent également plusieurs référentiels. Après avoir sélectionné un environnement cloud, cliquez sur le bouton **+** à côté du référentiel sélectionné pour ajouter des référentiels supplémentaires à la session. Chaque référentiel obtient son propre sélecteur de branche. Ceci est utile pour les tâches qui s'étendent sur plusieurs bases de code, comme la mise à jour d'une bibliothèque partagée et ses consommateurs.

Voir [Utiliser Claude Code dans le cloud](/docs/fr/claude-code-on-the-web) pour plus d'informations sur le fonctionnement des sessions cloud. Quand un corps de travail nécessite de nombreuses sessions cloud, sélectionnez **Projets** dans la barre latérale pour créer un [projet](/docs/fr/claude-projects), où Claude démarre et suit les sessions pour vous à partir d'une conversation.

<h3 id="continue-in-another-surface">
  Continuer sur une autre surface
</h3>

Le menu **Continuer dans**, accessible à partir de l'icône VS Code en bas à droite de la barre d'outils de la session, vous permet de déplacer votre session vers une autre surface :

* **Claude Code sur le Web** : envoie votre session locale pour continuer à s'exécuter dans le cloud. Desktop pousse votre branche, génère un résumé de la conversation et crée une nouvelle session cloud avec le contexte complet. Vous pouvez ensuite choisir d'archiver la session locale ou de la conserver. Ceci nécessite un arbre de travail propre et n'est pas disponible pour les sessions SSH.
* **Votre IDE** : ouvre votre projet dans un IDE supporté au répertoire de travail actuel.

<h3 id="sessions-from-dispatch">
  Sessions depuis Dispatch
</h3>

[Dispatch](https://support.claude.com/en/articles/13947068) est une conversation persistante avec Claude qui vit dans l'onglet [Cowork](https://claude.com/product/cowork). Vous envoyez un message à Dispatch avec une tâche, et il décide comment la gérer.

Une tâche peut devenir une session Code de deux façons : vous en demandez une directement, comme « ouvrir une session Claude Code et corriger le bug de connexion », ou Dispatch décide que la tâche est du travail de développement et en génère une automatiquement. Les tâches qui routent généralement vers Code incluent la correction de bugs, la mise à jour des dépendances, l'exécution de tests ou l'ouverture de demandes de tirage. La recherche, l'édition de documents et le travail sur feuille de calcul restent dans Cowork.

De toute façon, la session Code apparaît dans la barre latérale de l'onglet Code avec un badge **Dispatch**. Vous recevez une notification push sur votre téléphone quand elle se termine ou a besoin de votre approbation.

Si vous avez [l'utilisation informatique](#let-claude-use-your-computer) activée, les sessions Code générées par Dispatch peuvent l'utiliser aussi. Les approbations d'application dans ces sessions expirent après 30 minutes et re-demandent, plutôt que de durer la session complète comme les sessions Code régulières.

Pour la configuration, l'appairage et les paramètres Dispatch, voir l'[article d'aide Dispatch](https://support.claude.com/en/articles/13947068). Dispatch nécessite un plan Pro ou Max et n'est pas disponible sur les plans Team ou Enterprise.

Dispatch est l'une de plusieurs façons de travailler avec Claude quand vous êtes loin de votre terminal. Pour une comparaison avec les autres options, voir [Plateformes et intégrations](/docs/fr/platforms#work-when-you-are-away-from-your-terminal).

<h2 id="extend-claude-code">
  Étendre Claude Code
</h2>

Connectez les services externes, ajoutez des flux de travail réutilisables, personnalisez le comportement de Claude et configurez les serveurs d'aperçu. Pour gérer les connecteurs, les skills et les plugins au même endroit, cliquez sur **Personnaliser** dans la barre latérale. L'onglet [Cowork](https://claude.com/product/cowork) dans l'application de bureau récupère ses skills, plugins et connecteurs à partir de cette configuration Personnaliser, qui se synchronise via votre compte claude.ai, et non à partir du répertoire `~/.claude` de la CLI.

Claude Code charge également les skills et les plugins activés pour votre compte claude.ai dans les sessions de terminal où vous vous connectez avec le même compte. Voir [Skills synchronisés à partir de claude.ai](/docs/fr/skills#how-synced-skills-behave) et [Plugins synchronisés à partir de claude.ai](/docs/fr/plugins/loading#synced-plugins).

<h3 id="connect-external-tools">
  Connecter les outils externes
</h3>

Pour les sessions locales et [SSH](#ssh-sessions), cliquez sur le bouton **+** à côté de la zone de prompt et sélectionnez **Connecteurs** pour ajouter des intégrations comme Google Calendar, Slack, GitHub, Linear, Notion et bien d'autres. Vous pouvez ajouter des connecteurs avant ou pendant une session. Le bouton **+** n'est pas disponible dans les sessions cloud ou WSL, mais les [routines](/docs/fr/routines) configurent les connecteurs au moment de la création de la routine.

Pour gérer ou déconnecter les connecteurs, allez à Paramètres → Connecteurs dans l'application de bureau, ou sélectionnez **Gérer les connecteurs** à partir du menu Connecteurs dans la zone de prompt.

Une fois connecté, Claude peut lire votre calendrier, envoyer des messages, créer des problèmes et interagir avec vos outils directement. Vous pouvez demander à Claude quels connecteurs sont configurés dans votre session.

Les connecteurs sont [des serveurs MCP](/docs/fr/mcp) avec un flux de configuration graphique. Utilisez-les pour une intégration rapide avec les services supportés. Pour les intégrations non listées dans Connecteurs, ajoutez les serveurs MCP manuellement via [fichiers de paramètres](/docs/fr/mcp#installing-mcp-servers). Vous pouvez également [créer des connecteurs personnalisés](https://support.claude.com/en/articles/11175166-getting-started-with-custom-connectors-using-remote-mcp).

<h3 id="use-skills">
  Utiliser les skills
</h3>

[Les skills](/docs/fr/skills) étendent ce que Claude peut faire. Claude les charge automatiquement quand ils sont pertinents, ou vous pouvez en invoquer un directement : tapez `/` dans la zone de prompt ou cliquez sur le bouton **+** et sélectionnez **Slash commands** pour parcourir ce qui est disponible. Ceci inclut [les commandes intégrées](/docs/fr/commands), vos [skills personnalisés](/docs/fr/skills#create-your-first-skill), les skills du projet à partir de votre base de code et les skills de tout [plugin installé](/docs/fr/plugins/install). Sélectionnez-en un et il apparaît en surbrillance dans le champ d'entrée. Tapez votre tâche après et envoyez comme d'habitude.

Vous pouvez envoyer une commande pendant que Claude travaille, de la même manière que tout autre message, et la session revient à l'inactivité une fois que le tour est terminé. Avant la v2.1.206, une commande envoyée en cours de tour pouvait laisser la session affichée comme en cours d'exécution et les messages que vous avez envoyés après n'étaient pas livrés.

Les sessions locales chargent vos skills personnels à partir de `~/.claude/skills/`. Une session [SSH](#ssh-sessions) lit `~/.claude/skills/` à partir du répertoire personnel de l'hôte distant, et non à partir de votre machine.

Les sessions locales et cloud chargent également les skills activés pour votre compte claude.ai. Les sessions cloud les chargent à la place de `~/.claude/skills/`, comme [Skills dans les sessions Cowork et cloud](/docs/fr/skills#skills-in-cowork-and-cloud-sessions) le décrit.

<h3 id="install-plugins">
  Installer les plugins
</h3>

[Les plugins](/docs/fr/plugins/overview) sont des packages réutilisables qui ajoutent des skills, des agents, des hooks, des serveurs MCP et des configurations LSP à Claude Code. Vous pouvez installer les plugins à partir de l'application de bureau sans utiliser le terminal.

Pour les sessions locales et [SSH](#ssh-sessions), cliquez sur le bouton **+** à côté de la zone de prompt et sélectionnez **Plugins** pour voir vos plugins installés et leurs skills. Pour ajouter un plugin, sélectionnez **Ajouter un plugin** à partir du sous-menu pour ouvrir le navigateur de plugins, qui affiche les plugins disponibles à partir de vos [marketplaces](/docs/fr/plugins/overview) configurés, y compris le marketplace officiel d'Anthropic. Sélectionnez **Gérer les plugins** pour activer, désactiver ou désinstaller les plugins.

Vous pouvez limiter les plugins à votre compte utilisateur, un projet spécifique ou local uniquement. Si votre organisation gère les plugins de manière centralisée, ces plugins sont disponibles dans les sessions de bureau de la même manière qu'ils le sont dans la CLI.

Le navigateur de plugins n'est pas disponible dans les sessions cloud, et les plugins que vous installez à partir de l'application de bureau ne sont pas disponibles pour les sessions cloud. Une session cloud n'installe pas non plus les plugins que le fichier `.claude/settings.json` du référentiel déclare, comme [Ce qui est reporté de votre configuration](/docs/fr/cloud-environments#what-carries-over-from-your-setup) l'explique. Les plugins ne sont pas disponibles dans les sessions WSL. Pour la référence complète des plugins, y compris la création de vos propres plugins, voir [plugins](/docs/fr/plugins/overview).

<h3 id="configure-preview-servers">
  Configurer les serveurs d'aperçu
</h3>

Claude détecte automatiquement votre configuration de serveur de développement et stocke la configuration dans `.claude/launch.json` à la racine du dossier que vous avez sélectionné au démarrage de la session. L'aperçu utilise ce dossier comme répertoire de travail, donc si vous avez sélectionné un dossier parent, les sous-dossiers avec leurs propres serveurs de développement ne seront pas détectés automatiquement. Pour travailler avec le serveur d'un sous-dossier, soit démarrez une session dans ce dossier directement, soit ajoutez une configuration manuellement.

Pour personnaliser le démarrage de votre serveur, par exemple pour utiliser `yarn dev` au lieu de `npm run dev` ou pour modifier le port, modifiez le fichier manuellement ou cliquez sur **Modifier la configuration** dans la liste déroulante du serveur pour l'ouvrir dans votre éditeur de code. Le fichier supporte JSON avec commentaires.

```json theme={null}
{
  "version": "0.0.1",
  "configurations": [
    {
      "name": "my-app",
      "runtimeExecutable": "npm",
      "runtimeArgs": ["run", "dev"],
      "port": 3000
    }
  ]
}
```

Vous pouvez définir plusieurs configurations pour exécuter différents serveurs à partir du même projet, comme un frontend et une API. Voir les [exemples](#examples) ci-dessous.

<h4 id="auto-verify-changes">
  Vérification automatique des modifications
</h4>

Quand `autoVerify` est activé, Claude vérifie automatiquement les modifications de code après la modification des fichiers. Il prend des captures d'écran, vérifie les erreurs et confirme que les modifications fonctionnent avant de terminer sa réponse.

La vérification automatique est activée par défaut. Désactivez-la par projet en ajoutant `"autoVerify": false` à `.claude/launch.json`, ou basculez-la à partir du menu déroulant du serveur.

```json theme={null}
{
  "version": "0.0.1",
  "autoVerify": false,
  "configurations": [
    {
      "name": "my-app",
      "runtimeExecutable": "npm",
      "runtimeArgs": ["run", "dev"],
      "port": 3000
    }
  ]
}
```

Quand désactivée, les outils d'aperçu sont toujours disponibles et vous pouvez demander à Claude de vérifier à tout moment. La vérification automatique la rend automatique après chaque modification.

<h4 id="configuration-fields">
  Champs de configuration
</h4>

Chaque entrée dans le tableau `configurations` accepte les champs suivants :

| Champ               | Type      | Description                                                                                                                                                                                                                                                                                                                           |
| ------------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`              | string    | Un identifiant unique pour ce serveur                                                                                                                                                                                                                                                                                                 |
| `runtimeExecutable` | string    | La commande à exécuter, comme `npm`, `yarn` ou `node`                                                                                                                                                                                                                                                                                 |
| `runtimeArgs`       | string\[] | Arguments passés à `runtimeExecutable`, comme `["run", "dev"]`                                                                                                                                                                                                                                                                        |
| `port`              | number    | Le port sur lequel votre serveur écoute. Par défaut 3000                                                                                                                                                                                                                                                                              |
| `cwd`               | string    | Répertoire de travail relatif à la racine de votre projet. Par défaut la racine du projet. Utilisez `${workspaceFolder}` pour référencer la racine du projet explicitement                                                                                                                                                            |
| `env`               | object    | Variables d'environnement supplémentaires comme paires clé-valeur, comme `{ "NODE_ENV": "development" }`. Ne mettez pas de secrets ici car ce fichier est validé dans votre référentiel. Pour passer les secrets à votre serveur de développement, définissez-les dans l'[éditeur d'environnement local](#local-sessions) à la place. |
| `autoPort`          | boolean   | Comment gérer les conflits de port. Voir [Conflits de port](#port-conflicts)                                                                                                                                                                                                                                                          |
| `program`           | string    | Un script à exécuter avec `node`. Voir [quand utiliser `program` vs `runtimeExecutable`](#when-to-use-program-vs-runtimeexecutable)                                                                                                                                                                                                   |
| `args`              | string\[] | Arguments passés à `program`. Utilisé uniquement quand `program` est défini                                                                                                                                                                                                                                                           |
| `url`               | string    | L'adresse que l'aperçu ouvre au lieu de `http://localhost:<port>`. Voir [ouvrir l'aperçu à une URL spécifique](#open-the-preview-at-a-specific-url)                                                                                                                                                                                   |

<a id="when-to-use-program-vs-runtimeexecutable" />

<h5 id="when-to-use-program-vs-runtimeexecutable">
  Quand utiliser `program` vs `runtimeExecutable`
</h5>

Utilisez `runtimeExecutable` avec `runtimeArgs` pour démarrer un serveur de développement via un gestionnaire de packages. Par exemple, `"runtimeExecutable": "npm"` avec `"runtimeArgs": ["run", "dev"]` exécute `npm run dev`.

Utilisez `program` quand vous avez un script autonome que vous voulez exécuter avec `node` directement. Par exemple, `"program": "server.js"` exécute `node server.js`. Passez des drapeaux supplémentaires avec `args`.

<a id="open-the-preview-at-a-specific-url" />

<h5 id="open-the-preview-at-a-specific-url">
  Ouvrir l'aperçu à une URL spécifique
</h5>

Par défaut, l'aperçu ouvre `http://localhost:<port>`. Définissez `url` quand votre serveur a besoin d'une adresse différente. Les cas courants sont les serveurs qui nécessitent HTTPS local, les applications qui utilisent des sous-domaines `*.localhost` et les applications qui vous connectent via une redirection.

```json theme={null}
{
  "version": "0.0.1",
  "configurations": [
    {
      "name": "my-app",
      "runtimeExecutable": "npm",
      "runtimeArgs": ["run", "dev"],
      "port": 8443,
      "url": "https://localhost:8443"
    }
  ]
}
```

Les adresses localhost s'ouvrent directement, exactement comme l'adresse de port par défaut. Ceci inclut `localhost`, n'importe quel sous-domaine `*.localhost`, `127.0.0.1` et `::1`. Pour la sécurité, une `url` localhost doit être juste l'origine de votre serveur, sans chemin ni requête. Son port doit correspondre au port de l'entrée. Pour afficher une page spécifique, demandez à Claude de naviguer là-bas après l'ouverture de l'aperçu. Une `url` localhost avec un chemin, une requête ou un port non correspondant est signalée comme une erreur de configuration qui nomme l'url et affiche la correction.

Pour toute autre adresse, Desktop demande votre permission la première fois que l'aperçu l'ouvre, de la même manière que lorsque vous naviguez vers un nouveau site dans l'aperçu. Les adresses externes peuvent inclure des chemins. Choisissez **Toujours autoriser** pour ignorer l'invite pour ce site à l'avenir. Les politiques organisationnelles qui restreignent les sites externes dans l'aperçu s'appliquent toujours.

Pour prévisualiser un serveur que vous exécutez déjà vous-même, définissez `url` sans commande. Claude attache l'aperçu à votre serveur en cours d'exécution au lieu d'en démarrer un :

```json theme={null}
{
  "version": "0.0.1",
  "configurations": [
    {
      "name": "my-app",
      "url": "https://app.localhost:3000"
    }
  ]
}
```

L'`url` doit être `http` ou `https`, et ne doit pas contenir de nom d'utilisateur ou de mot de passe.

<h4 id="port-conflicts">
  Conflits de port
</h4>

Le champ `autoPort` contrôle ce qui se passe quand votre port préféré est déjà utilisé :

* **`true`** : Claude trouve et utilise un port libre automatiquement. Approprié pour la plupart des serveurs de développement.
* **`false`** : Claude échoue avec une erreur. Utilisez ceci quand votre serveur doit utiliser un port spécifique, comme pour les rappels OAuth ou les listes blanches CORS.
* **Non défini (par défaut)** : Claude demande si le serveur a besoin de ce port exact, puis enregistre votre réponse.

Quand Claude choisit un port différent, il passe le port assigné à votre serveur via la variable d'environnement `PORT`.

<h4 id="examples">
  Exemples
</h4>

Ces configurations montrent les configurations courantes pour différents types de projets :

<Tabs>
  <Tab title="Next.js">
    Cette configuration exécute une application Next.js en utilisant Yarn sur le port 3000 :

    ```json theme={null}
    {
      "version": "0.0.1",
      "configurations": [
        {
          "name": "web",
          "runtimeExecutable": "yarn",
          "runtimeArgs": ["dev"],
          "port": 3000
        }
      ]
    }
    ```
  </Tab>

  <Tab title="Plusieurs serveurs">
    Pour un monorepo avec un serveur frontend et API, définissez plusieurs configurations. Le frontend utilise `autoPort: true` pour qu'il choisisse un port libre si 3000 est pris, tandis que le serveur API nécessite le port 8080 exactement :

    ```json theme={null}
    {
      "version": "0.0.1",
      "configurations": [
        {
          "name": "frontend",
          "runtimeExecutable": "npm",
          "runtimeArgs": ["run", "dev"],
          "cwd": "apps/web",
          "port": 3000,
          "autoPort": true
        },
        {
          "name": "api",
          "runtimeExecutable": "npm",
          "runtimeArgs": ["run", "start"],
          "cwd": "server",
          "port": 8080,
          "env": { "NODE_ENV": "development" },
          "autoPort": false
        }
      ]
    }
    ```
  </Tab>

  <Tab title="Script Node.js">
    Pour exécuter un script Node.js directement au lieu d'utiliser une commande du gestionnaire de packages, utilisez le champ `program` :

    ```json theme={null}
    {
      "version": "0.0.1",
      "configurations": [
        {
          "name": "server",
          "program": "server.js",
          "args": ["--verbose"],
          "port": 4000
        }
      ]
    }
    ```
  </Tab>
</Tabs>

<h2 id="environment-configuration">
  Configuration de l'environnement
</h2>

L'environnement que vous choisissez au [démarrage d'une session](#start-a-session) détermine où Claude s'exécute et comment vous vous connectez :

* **Local** : s'exécute sur votre machine avec accès direct à vos fichiers
* **Cloud** : s'exécute sur l'infrastructure gérée par Anthropic par défaut. Les sessions continuent même si vous fermez l'application.
* **SSH** : s'exécute sur une machine distante à laquelle vous vous connectez via SSH, comme vos propres serveurs, des machines virtuelles cloud ou des conteneurs de développement
* **WSL** (Windows) : s'exécute à l'intérieur d'une [distribution WSL 2](/docs/fr/desktop-wsl) sur votre machine, en utilisant sa chaîne d'outils Linux et ses chemins natifs

<h3 id="local-sessions">
  Sessions locales
</h3>

L'application de bureau n'hérite pas toujours de votre environnement shell complet. Sur macOS, quand vous lancez l'application à partir du Dock ou du Finder, elle lit votre profil shell, comme `~/.zshrc` ou `~/.bashrc`, pour extraire `PATH` et un ensemble fixe de variables Claude Code, mais les autres variables que vous exportez là ne sont pas récupérées. Sur Windows, l'application hérite des variables d'environnement utilisateur et système mais ne lit pas les profils PowerShell.

Pour définir les variables d'environnement pour les sessions locales et les serveurs de développement sur n'importe quelle plateforme, ouvrez la liste déroulante d'environnement dans la zone de prompt, survolez **Local** et cliquez sur l'icône d'engrenage pour ouvrir l'éditeur d'environnement local. Les variables que vous enregistrez ici sont stockées chiffrées sur votre machine et s'appliquent à chaque session locale et serveur d'aperçu que vous démarrez. Vous pouvez également ajouter des variables à la clé `env` dans votre fichier `~/.claude/settings.json`, bien que celles-ci n'atteignent que les sessions Claude et non les serveurs de développement. Voir [variables d'environnement](/docs/fr/env-vars) pour la liste complète des variables supportées.

[La réflexion étendue](/docs/fr/model-config#extended-thinking) est activée par défaut, ce qui améliore les performances sur les tâches de raisonnement complexe mais utilise des tokens supplémentaires. Sur l'API Anthropic, définissez `MAX_THINKING_TOKENS` à `0` dans l'éditeur d'environnement local pour désactiver la réflexion ; cela n'a aucun effet sur Opus 5.5 ou les modèles Fable, qui utilisent toujours la réflexion étendue. Avec la réflexion désactivée sur l'API Anthropic, Claude Code envoie l'effort `high` au lieu d'un niveau supérieur aux modèles qu'il sait [ne pas accepter cette combinaison](/docs/fr/errors#effort-isnt-available-with-thinking-turned-off), comme Opus 5.

Sur les modèles avec [raisonnement adaptatif](/docs/fr/model-config#adjust-effort-level), les valeurs `MAX_THINKING_TOKENS` autres que `0` sont ignorées car le raisonnement adaptatif contrôle la profondeur de la réflexion à la place. Sur Opus 4.6 et Sonnet 4.6, définissez `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING` à `1` pour utiliser un budget de réflexion fixe ; les modèles Fable, Sonnet 5, et Opus 4.7 et versions ultérieures utilisent toujours le raisonnement adaptatif et n'ont pas de mode de budget fixe.

<h4 id="local-sessions-on-managed-devices">
  Sessions locales sur les appareils gérés
</h4>

Votre administrateur peut désactiver les sessions locales avec le [paramètre géré `disableDesktopLocalSessions`](#managed-settings). Quand il l'a fait, **Local** reste dans la liste déroulante d'environnement mais est grisé et ne peut pas être sélectionné, avec une infobulle indiquant que votre organisation l'a désactivé, et sur Windows l'entrée [WSL](/docs/fr/desktop-wsl), dont la disponibilité sur les appareils gérés est [gouvernée séparément](/docs/fr/admin-setup#wsl-sessions-in-claude-code-desktop), est grisée de la même manière. Les nouvelles sessions utilisent par défaut la première connexion SSH si une est configurée, et Desktop affiche un message indiquant que les sessions locales ne sont pas disponibles sur cet appareil si vous essayez de continuer une session existante. Choisissez plutôt un environnement [SSH](#ssh-sessions) ou [cloud](#cloud-sessions), ou contactez votre équipe informatique.

<h3 id="cloud-sessions">
  Sessions cloud
</h3>

Les sessions cloud continuent en arrière-plan même si vous fermez l'application. L'utilisation compte vers les limites de votre [plan d'abonnement](/docs/fr/costs) sans frais de calcul séparés.

Vous pouvez créer des environnements cloud personnalisés avec différents niveaux d'accès réseau et variables d'environnement. Quand vous démarrez une session cloud, ouvrez la liste déroulante d'environnement dans la zone de prompt pour les gérer :

* **Ajouter un environnement** : sélectionnez **Ajouter un environnement cloud**
* **Modifier ou archiver l'un de vos propres environnements** : survolez-le et cliquez sur l'icône d'engrenage

Voir [Configurer les environnements cloud](/docs/fr/cloud-environments) pour les détails sur la configuration de l'accès réseau et des variables d'environnement.

<h3 id="ssh-sessions">
  Sessions SSH
</h3>

Les sessions SSH vous permettent d'exécuter Claude Code sur une machine distante tout en utilisant l'application de bureau comme votre interface. Ceci est utile pour travailler avec des bases de code qui vivent sur des machines virtuelles cloud, des conteneurs de développement ou des serveurs avec du matériel ou des dépendances spécifiques.

Pour ajouter une connexion SSH, cliquez sur la liste déroulante d'environnement avant de démarrer une session et sélectionnez **+ Ajouter une connexion SSH**. La boîte de dialogue demande :

* **Nom** : une étiquette conviviale pour cette connexion
* **Hôte SSH** : `user@hostname` ou un hôte défini dans `~/.ssh/config`
* **Port SSH** : par défaut 22 s'il est laissé vide, ou utilise le port de votre configuration SSH
* **Fichier d'identité** : chemin vers votre clé privée, comme `~/.ssh/id_rsa`. Laissez vide pour utiliser la clé par défaut ou votre configuration SSH.

Une fois ajoutée, la connexion apparaît dans la liste déroulante d'environnement. Sélectionnez-la pour démarrer une session sur cette machine. Claude s'exécute sur la machine distante avec accès à ses fichiers et outils.

La machine distante doit exécuter Linux ou macOS. Desktop installe Claude Code sur la machine distante automatiquement la première fois que vous vous connectez. Une fois connecté, les sessions SSH supportent les modes de permission, les connecteurs, les plugins et les serveurs MCP.

<h4 id="pre-configure-ssh-connections-for-your-team">
  Pré-configurer les connexions SSH pour votre équipe
</h4>

Les administrateurs peuvent distribuer les connexions SSH aux membres de l'équipe en ajoutant `sshConfigs` à un fichier de [paramètres gérés](/docs/fr/managed-settings). Les connexions définies de cette manière apparaissent dans la liste déroulante d'environnement de chaque utilisateur automatiquement et sont affichées comme gérées, de sorte que les utilisateurs peuvent les sélectionner mais ne peuvent pas les modifier ou les supprimer dans l'application.

L'exemple suivant pré-configure une seule connexion :

```json theme={null}
{
  "sshConfigs": [
    {
      "id": "shared-dev-vm",
      "name": "Shared Dev VM",
      "sshHost": "user@dev.example.com",
      "sshPort": 22,
      "sshIdentityFile": "~/.ssh/id_ed25519"
    }
  ]
}
```

Chaque entrée nécessite `id`, `name` et `sshHost`. Les champs `sshPort` et `sshIdentityFile` sont optionnels. Les utilisateurs peuvent également ajouter `sshConfigs` à leur propre `~/.claude/settings.json`, qui est l'endroit où les connexions ajoutées via la boîte de dialogue sont stockées.

<h4 id="restrict-which-ssh-hosts-users-can-connect-to">
  Restreindre les hôtes SSH auxquels les utilisateurs peuvent se connecter
</h4>

Les administrateurs peuvent limiter les sessions SSH de Desktop à un ensemble approuvé d'hôtes en ajoutant `sshHostAllowlist` à un fichier de [paramètres gérés](/docs/fr/managed-settings). Lorsqu'il est défini, les utilisateurs ne peuvent se connecter qu'à des hôtes dont le nom d'hôte résolu correspond à l'un des modèles. Définissez-le sur un tableau vide pour désactiver complètement les sessions SSH.

L'exemple suivant autorise les connexions à n'importe quel hôte sous `devboxes.example.com` et à un seul hôte bastion nommé :

```json theme={null}
{
  "sshHostAllowlist": ["*.devboxes.example.com", "bastion.example.com"]
}
```

Les modèles sont insensibles à la casse. `*` correspond à n'importe quel hôte, et `*.example.com` correspond à `example.com` et à n'importe quel sous-domaine. Tout le reste est une correspondance exacte. La vérification s'exécute sur le nom d'hôte après la résolution `~/.ssh/config` via `ssh -G`, de sorte que les alias `Host` et les entrées `ProxyCommand`/`ProxyJump` sont autorisés tant que le `HostName` résolu correspond.

`sshHostAllowlist` est lu uniquement à partir des paramètres gérés ; les valeurs dans les paramètres utilisateur ou projet sont ignorées. Seule l'application Claude Desktop honore ce paramètre ; la CLI Claude Code et les extensions IDE ne le lisent pas, et il ne restreint pas les commandes `ssh` exécutées via l'outil Bash. Il gouverne les hôtes auxquels l'application Desktop se connecte, pas la sortie réseau, donc associez-le aux contrôles réseau ou zero-trust de votre organisation si vous avez besoin d'une limite stricte.

<h2 id="enterprise-configuration">
  Configuration d'entreprise
</h2>

Les organisations sur les plans Team ou Enterprise peuvent gérer le comportement de l'application de bureau via les contrôles de la console d'administration, les fichiers de paramètres gérés et les politiques de gestion des appareils.

<h3 id="admin-console-controls">
  Contrôles de la console d'administration
</h3>

Ces paramètres sont configurés via la [console de paramètres d'administration](https://claude.ai/admin-settings/claude-code) :

* **Code dans le bureau** : contrôlez si les utilisateurs de votre organisation peuvent accéder à Claude Code dans l'application de bureau
* **Code sur le web** : activez ou désactivez les [sessions cloud](/docs/fr/claude-code-on-the-web) pour votre organisation
* **Remote Control** : activez ou désactivez [Remote Control](/docs/fr/remote-control) pour votre organisation
* **Désactiver le mode Contourner les permissions** : empêchez les utilisateurs de votre organisation d'activer le mode de contournement des permissions

<h3 id="managed-settings">
  Paramètres gérés
</h3>

Les paramètres gérés remplacent les paramètres du projet et de l'utilisateur et s'appliquent aux sessions Claude Code dans Desktop. Vous pouvez définir ces clés dans le fichier [paramètres gérés](/docs/fr/managed-settings) de votre organisation ou les pousser à distance via la console d'administration.

| Clé                                        | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissions.disableBypassPermissionsMode` | définissez sur `"disable"` pour empêcher les utilisateurs d'activer le mode de contournement des permissions.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `disableAutoMode`                          | définissez sur `"disable"` pour supprimer le mode [Auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) du sélecteur de mode. Également accepté sous `permissions`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `autoMode`                                 | personnalisez ce que le classificateur du mode auto fait confiance et bloque dans votre organisation. Voir [Configurer le mode auto](/docs/fr/auto-mode-config).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `browserExternalPageTools`                 | définissez sur `"disabled"` pour empêcher Claude d'utiliser des outils pour lire ou agir sur des pages externes dans le [volet Navigateur](#browse-external-sites). Les utilisateurs peuvent toujours naviguer vers des sites externes eux-mêmes, et les aperçus des serveurs de développement locaux ne sont pas affectés.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `disableMobileSimulatorTools`              | définissez sur `true` pour bloquer les outils de Claude pour contrôler et capturer les appareils dans le [volet iOS Simulator](/docs/fr/desktop-ios-simulator#turn-off-simulator-access). Le volet reste utilisable pour les appuis de l'utilisateur ; seul l'accès de Claude est supprimé. La valeur doit être le booléen JSON `true` ; la chaîne `"true"` est ignorée.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `disableBrowserExternalNavigation`         | définissez sur `true` pour désactiver complètement la navigation externe dans le [volet Navigateur](#browse-external-sites). Ni les utilisateurs ni Claude ne peuvent naviguer vers des sites externes, et les aperçus des serveurs de développement localhost ne sont pas affectés. La valeur doit être le booléen JSON `true` ; la chaîne `"true"` est ignorée.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `sshConfigs`                               | pré-configurez les [connexions SSH](#pre-configure-ssh-connections-for-your-team) qui apparaissent dans la liste déroulante de l'environnement. Les utilisateurs ne peuvent pas modifier ou supprimer les connexions gérées.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `sshHostAllowlist`                         | restreignez les [sessions SSH](#restrict-which-ssh-hosts-users-can-connect-to) aux hôtes dont le nom d'hôte résolu correspond à l'un de ces modèles. Un tableau vide désactive les sessions SSH. Lecture à partir des paramètres gérés uniquement.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `disableDesktopLocalSessions`              | définissez sur `true` pour désactiver les [sessions Code qui s'exécutent sur l'appareil](#local-sessions-on-managed-devices), en laissant les sessions SSH vers d'autres hôtes et les sessions cloud disponibles. La valeur doit être le booléen JSON `true`. Lecture à partir des paramètres gérés uniquement. Nécessite Claude Desktop v1.37937.0 ou version ultérieure.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `managedMcpServers`                        | poussez les configurations du serveur MCP à tous les utilisateurs. Disponible dans les déploiements Desktop tiers (3P) uniquement. Dans chaque entrée, définissez un transport de `"http"`, `"sse"` ou `"stdio"`, les détails de connexion et optionnellement une carte `toolPolicy` pour restreindre les outils de ce serveur que les utilisateurs peuvent invoquer. Livrez-le via le fichier de paramètres gérés, MDM ou la politique de bloc [`desktop`](/docs/fr/claude-apps-gateway-config#claude-desktop-overlay) d'une passerelle d'applications Claude, car les déploiements tiers ne reçoivent pas les paramètres de la console d'administration. Pour le livrer via la passerelle, vous avez besoin de Claude Code v2.1.232 ou version ultérieure sur le serveur de la passerelle. Ceci est la clé propre de l'application de bureau ; Claude Code lit un [paramètre géré de même nom](/docs/fr/managed-mcp#provide-servers-through-managed-settings) qui lui est propre, avec une forme d'entrée différente. |

Les paramètres gérés qui atteignent une session Desktop dépendent de l'endroit où cette session s'exécute. Les restrictions de modèle telles que [`availableModels`](/docs/fr/model-config#restrict-model-selection) sont appliquées dans les sessions Claude Code de Desktop de la même manière que dans le CLI du terminal ; voir [couverture de surface](/docs/fr/model-config#surface-coverage).

* **Sessions locales sur cette machine** : un fichier de paramètres gérés déployé sur le disque s'applique. Les paramètres gérés poussés à distance via la console d'administration atteignent également ces sessions sur l'API d'Anthropic lorsque la session s'authentifie avec une [connexion ou clé éligible](/docs/fr/server-managed-settings#platform-availability), en suivant la même [précédence des paramètres](/docs/fr/settings#settings-precedence) que le CLI du terminal.
* **[Sessions cloud](#cloud-sessions)** : reçoivent les [paramètres gérés par le serveur](/docs/fr/server-managed-settings) ; les fichiers déployés sur l'appareil ne les atteignent pas, car ils s'exécutent sur des machines virtuelles gérées par Anthropic. Les sessions acheminées vers un [environnement auto-hébergé](/docs/fr/self-hosted-environments) lisent également le fichier de paramètres gérés dans l'image du runner. [Comment Claude Code combine les sources gérées](/docs/fr/managed-settings#how-claude-code-combines-managed-sources) indique quand ce fichier s'applique.
* **[Sessions SSH](#ssh-sessions)** : la session lit le fichier de paramètres gérés à partir de l'hôte distant. Desktop lui-même lit `sshConfigs`, `sshHostAllowlist` et `disableDesktopLocalSessions` à partir des paramètres gérés de la machine locale.
* **Sessions [Cowork](https://claude.com/docs/cowork/overview)** : dans une session Cowork sur cette machine, Claude Code ne récupère jamais les paramètres de la console d'administration, même lorsque l'utilisateur se connecte avec un compte Team ou Enterprise, et lit la politique déployée sur la machine sauf si votre configuration Claude Desktop définit `requireCoworkFullVmSandbox`. Les sessions Cowork distantes ne reçoivent ni l'une ni l'autre. Voir [où et quand une politique s'applique](/docs/fr/managed-settings#where-and-when-a-policy-applies) pour savoir quels fichiers d'appareil atteignent Cowork, et [règles de permissions MCP](/docs/fr/permissions#mcp) pour savoir comment les règles `Bash` et `WebFetch` s'appliquent aux outils de Cowork.

Dans les sessions locales et SSH, l'application de bureau fournit les connecteurs claude.ai connectés de chaque utilisateur à Claude Code directement. Aucun paramètre MCP ou `managed-mcp.json` n'atteint ces connecteurs, quelle que soit la source de paramètres ou l'emplacement du fichier que vous utilisez. Pour bloquer les outils d'un connecteur dans ces sessions, utilisez les [contrôles des outils de connecteur](/docs/fr/mcp#organization-controls-on-connector-tools) de votre organisation. [Comment les connecteurs atteignent Claude Code](/docs/fr/mcp#how-connectors-reach-claude-code) montre quels paramètres gouvernent les connecteurs dans chaque type de session.

`permissions.disableBypassPermissionsMode` et `disableAutoMode` fonctionnent également dans les paramètres utilisateur et projet, mais les placer dans les paramètres gérés empêche les utilisateurs de les remplacer.

Pour les clés de permission, de plugin et de livraison que seule une source gérée peut définir, voir [Clés que seuls les paramètres gérés peuvent définir](/docs/fr/managed-settings#managed-only-settings).

<h3 id="device-management-policies">
  Politiques de gestion des appareils
</h3>

Les équipes informatiques peuvent gérer l'application de bureau via MDM sur macOS ou la politique de groupe sur Windows. Les politiques disponibles incluent l'activation ou la désactivation de la fonctionnalité Claude Code, le contrôle des mises à jour automatiques et la définition d'une URL de déploiement personnalisée.

* **macOS** : configurez via le domaine de préférence `com.anthropic.claudefordesktop` en utilisant des outils comme Jamf ou Kandji
* **Windows** : configurez via le registre à `SOFTWARE\Policies\Claude`

<h3 id="network-access-requirements">
  Exigences d'accès réseau
</h3>

Desktop charge son code d'application et le contenu utilisateur à partir des hôtes CDN d'Anthropic.

```text theme={null}
anthropic.com
*.anthropic.com
claude.ai
*.claude.ai
claude.com
*.claude.com
claude.app
*.claude.app
*.claudeusercontent.com
*.claudemcpcontent.com
```

Le trafic est HTTPS sur le port 443 sauf si vous configurez un port personnalisé pour [OTLP](/docs/fr/monitoring-usage), une passerelle LLM ou un serveur MCP.

Pour les serveurs proxy, les autorités de certificats personnalisées, mTLS et les domaines dont le CLI autonome a besoin, voir [configuration réseau](/docs/fr/network-config).

Pour réduire le nombre de caractères génériques du pare-feu, autorisez plutôt ces hôtes Anthropic. Certains sous-domaines sont générés dynamiquement et doivent rester des caractères génériques.

```text theme={null}
anthropic.com
api.anthropic.com
a-api.anthropic.com
a-cdn.anthropic.com
s-cdn.anthropic.com
assets-proxy.anthropic.com
claude.ai
a.claude.ai
a-cdn.claude.ai
assets.claude.ai
downloads.claude.ai
*.livepreview.claude.ai
claude.com
platform.claude.com
*.livepreview.claude.app
*.claudeusercontent.com
*.claudemcpcontent.com
```

Si votre organisation a [l'allowlisting IP](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting) activé pour Claude, acheminez `bridge.claudeusercontent.com` via le même proxy de sortie que `claude.ai` et `api.anthropic.com`. Si vous ne pouvez pas l'acheminer de cette façon, ajoutez l'adresse de sortie que votre proxy utilise pour cet hôte à la liste d'allowlist IP de votre organisation, mais uniquement lorsque cette adresse est dédiée à votre organisation : une plage de sortie de proxy partagée admet également les autres clients du fournisseur de proxy.

Anthropic vérifie les connexions à cet hôte par rapport à la liste d'allowlist IP de votre organisation en utilisant l'adresse à partir de laquelle elles arrivent. Si votre proxy envoie le trafic pour cela via une adresse qui ne figure pas sur cette liste d'allowlist, Claude dans Chrome et d'autres fonctionnalités qui se connectent via le pont cessent de fonctionner tandis que le reste de l'application continue de fonctionner.

Un [artifact](/docs/fr/artifacts) qui charge une police de caractères à partir de [Google Fonts](/docs/fr/artifacts#improve-the-visual-design) demande également `fonts.googleapis.com` et `fonts.gstatic.com`. Les deux hôtes sont optionnels. Si vous les bloquez, les artifacts s'affichent dans les polices de secours. Bloquez avec un rejet rapide plutôt qu'une suppression silencieuse pour que la demande de police échoue immédiatement au lieu de retarder le premier rendu de la page.

Les artifacts peuvent également charger des bibliothèques JavaScript, telles que React ou un package de graphiques, à partir de `cdnjs.cloudflare.com`, `cdn.jsdelivr.net`, `cdn.tailwindcss.com`, `code.jquery.com` et `unpkg.com`, et d'aucun autre hôte externe. Si vous bloquez ces hôtes, les parties d'un artifact qui dépendent d'une bibliothèque ne fonctionnent pas, et contrairement à une police bloquée, une bibliothèque bloquée n'a pas de secours. Bloquez avec un rejet rapide ici aussi, pour qu'une demande de bibliothèque bloquée échoue immédiatement plutôt que de rester suspendue jusqu'à l'expiration du délai.

<h3 id="authentication-and-sso">
  Authentification et SSO
</h3>

Les organisations d'entreprise peuvent exiger SSO pour tous les utilisateurs. Voir [authentification](/docs/fr/authentication) pour les détails au niveau du plan et [Configuration de SSO](https://support.claude.com/en/articles/13132885-setting-up-single-sign-on-sso) pour la configuration SAML ; la configuration OIDC est couverte dans le [Guide de l'administrateur Claude Enterprise](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide).

<h3 id="data-handling">
  Gestion des données
</h3>

Claude Code traite votre code localement dans les sessions locales, ou dans les sessions cloud sur l'infrastructure gérée par Anthropic, sauf si votre organisation les achemine vers un [environnement auto-hébergé](/docs/fr/self-hosted-environments). Les sessions cloud, y compris dans un environnement auto-hébergé, envoient les conversations et le contexte du code à l'API d'Anthropic pour le traitement ; les sessions locales et SSH les envoient à quel que soit le [fournisseur de modèle](#feature-comparison) que votre déploiement configure, l'API d'Anthropic par défaut. Voir [gestion des données](/docs/fr/data-usage) pour les détails sur la rétention des données, la confidentialité et la conformité.

<h3 id="deployment">
  Déploiement
</h3>

Desktop peut être distribué via les outils de déploiement d'entreprise :

* **macOS** : distribuez via MDM comme Jamf ou Kandji en utilisant l'installateur `.dmg`
* **Windows** : déployez via le package MSIX. Voir [Déployer Claude Desktop pour Windows](https://support.claude.com/en/articles/12622703-deploy-claude-desktop-for-windows) pour les options de déploiement d'entreprise, y compris l'installation silencieuse

Pour les domaines à ajouter à la liste blanche dans votre pare-feu, voir [exigences d'accès réseau](#network-access-requirements) ci-dessus. Pour les paramètres de proxy, les autorités de certificats personnalisées et les passerelles LLM, voir [configuration réseau](/docs/fr/network-config).

Pour la référence complète de la configuration d'entreprise, voir le [guide de configuration d'entreprise](https://support.claude.com/en/articles/12622667-enterprise-configuration).

<h2 id="coming-from-the-cli">
  Venant de la CLI ?
</h2>

Si vous utilisez déjà la CLI Claude Code, Desktop exécute le même moteur sous-jacent avec une interface graphique. Vous pouvez exécuter les deux simultanément sur la même machine, même sur le même projet. Chacun maintient un historique de session séparé, et vous pouvez amener une session CLI dans Desktop. Ils partagent la configuration et la mémoire du projet via les fichiers CLAUDE.md.

Pour déplacer une session CLI dans Desktop, exécutez `/desktop` dans le terminal. Claude enregistre votre session et l'ouvre dans l'application de bureau, puis quitte la CLI. Cette commande est disponible sur macOS et Windows x64 quand vous êtes connecté avec un abonnement Claude. Elle n'est pas disponible avec l'authentification par clé API ou sur Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry.

Pour reprendre une session CLI depuis Desktop, tapez `/resume` dans la zone de saisie. Desktop affiche les sessions que vous avez démarrées à partir de la CLI, et vous pouvez les rechercher par titre, dossier ou branche et prévisualiser où chacune s'est arrêtée. Sélectionnez une session et elle continue dans l'application avec sa conversation complète et son contexte.

<Tip>
  Quand utiliser Desktop vs CLI : utilisez Desktop quand vous voulez gérer les sessions parallèles dans une fenêtre, arranger les volets côte à côte ou examiner les modifications visuellement. Utilisez la CLI quand vous avez besoin de scripts, d'automatisation ou préférez un flux de travail terminal.
</Tip>

<h3 id="cli-flag-equivalents">
  Équivalents des drapeaux CLI
</h3>

Ce tableau montre l'équivalent de l'application de bureau pour les drapeaux CLI courants. Les drapeaux non listés n'ont pas d'équivalent de bureau car ils sont conçus pour les scripts ou l'automatisation.

| CLI                                            | Équivalent de bureau                                                                                                                                                                                                                   |
| ---------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--model sonnet`                               | Liste déroulante de modèle à côté du bouton d'envoi                                                                                                                                                                                    |
| `--resume`, `--continue`                       | Cliquez sur une session dans la barre latérale, ou tapez `/resume` dans la zone de saisie pour reprendre une session que vous avez démarrée à partir de la CLI                                                                         |
| `--permission-mode`                            | Sélecteur de mode à côté du bouton d'envoi                                                                                                                                                                                             |
| `--dangerously-skip-permissions`               | Contourner les permissions. Sur les plans Pro et Max, activez-le dans Paramètres → Claude Code → « Autoriser le mode de contournement des permissions » ; sur les plans Team et Enterprise, la politique organisationnelle le contrôle |
| `--add-dir`                                    | Ajouter plusieurs référentiels avec le bouton **+** dans les sessions cloud                                                                                                                                                            |
| `--allowedTools`, `--disallowedTools`          | Aucun équivalent par session. Les règles de permission dans les [fichiers de paramètres](/docs/fr/settings) s'appliquent toujours.                                                                                                          |
| `--verbose`                                    | Mode d'affichage [Verbose](#switch-view-modes) dans la liste déroulante Vue de la transcription                                                                                                                                        |
| `--print`, `--output-format`                   | Non disponible. Desktop est interactif uniquement.                                                                                                                                                                                     |
| Variable d'environnement `ANTHROPIC_MODEL`     | Liste déroulante de modèle à côté du bouton d'envoi                                                                                                                                                                                    |
| Variable d'environnement `MAX_THINKING_TOKENS` | Définissez dans l'éditeur d'environnement local. Voir [configuration de l'environnement](#environment-configuration).                                                                                                                  |

<h3 id="shared-configuration">
  Configuration partagée
</h3>

Desktop et CLI lisent les mêmes fichiers de configuration, donc votre configuration se transfère :

* Les fichiers **[CLAUDE.md](/docs/fr/memory)** et `CLAUDE.local.md` dans votre projet sont utilisés par les deux
* Les **[serveurs MCP](/docs/fr/mcp)** configurés dans `~/.claude.json` ou `.mcp.json` fonctionnent dans les deux
* Les **[hooks](/docs/fr/hooks)** et **[skills](/docs/fr/skills)** définis dans les paramètres s'appliquent aux deux
* Les **[paramètres](/docs/fr/settings)** dans `~/.claude.json` et `~/.claude/settings.json` sont partagés. Les règles de permission, les outils autorisés et d'autres paramètres dans `settings.json` s'appliquent aux sessions Desktop.
* **Modèles** : les mêmes [modèles](/docs/fr/model-config#available-models) sont disponibles dans les deux. Dans Desktop, sélectionnez le modèle à partir de la liste déroulante à côté du bouton d'envoi. Vous pouvez modifier le modèle pendant la session à partir de la même liste déroulante.

<h4 id="mcp-servers-from-the-claude-desktop-chat-app">
  Serveurs MCP de l'application de chat Claude Desktop
</h4>

L'application Desktop charge les serveurs MCP de `claude_desktop_config.json` dans les sessions de l'onglet Code local, aux côtés des serveurs de `~/.claude.json` et `.mcp.json`. Un serveur défini dans `claude_desktop_config.json` est disponible à la fois dans la surface de chat Desktop et dans les sessions de l'onglet Code local.

Si vous définissez le même nom de serveur dans `claude_desktop_config.json` et dans `~/.claude.json` ou `.mcp.json`, l'onglet Code dans les sessions locales se connecte une fois et utilise la définition `claude_desktop_config.json`.

L'application réachemine également les serveurs stdio de `~/.claude.json` vers la CLI intégrée dans les sessions locales. Quand le niveau supérieur de `~/.claude.json` (portée utilisateur) et `.mcp.json` définissent le même nom de serveur stdio, l'onglet Code utilise la définition `~/.claude.json`, s'écartant de la [hiérarchie de portée](/docs/fr/mcp#scope-hierarchy-and-precedence) CLI.

<Note>
  La CLI autonome ne lit pas `claude_desktop_config.json`. Sur macOS et WSL, exécutez `claude mcp add-from-claude-desktop` pour copier ces serveurs dans `~/.claude.json`. Voir [Importer les serveurs MCP de Claude Desktop](/docs/fr/mcp#import-mcp-servers-from-claude-desktop) pour le flux d'importation et les options de portée.
</Note>

<h3 id="feature-comparison">
  Comparaison des fonctionnalités
</h3>

Ce tableau compare les capacités principales entre la CLI et Desktop. Pour une liste complète des drapeaux CLI, voir la [référence CLI](/docs/fr/cli-reference).

| Fonctionnalité                                     | CLI                                                                                          | Desktop                                                                                                                                                                                                                                                                                                                                                                                     |
| -------------------------------------------------- | -------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Modes de permission                                | Tous les modes y compris `dontAsk`                                                           | Manuel, Accepter les modifications, Plan et Auto. Le mode Contourner les permissions apparaît dans le sélecteur de mode une fois activé : via le bouton bascule Paramètres sur les plans Pro et Max, ou via la politique organisationnelle sur les plans Team et Enterprise                                                                                                                 |
| [Fournisseurs tiers](/docs/fr/third-party-integrations) | Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry                             | API d'Anthropic par défaut. Pour le routage par passerelle, voir [connecter l'application de bureau à une passerelle](/docs/fr/llm-gateway-connect#desktop-app). Pour exécuter l'onglet Code sur Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry ou une passerelle LLM auto-hébergée, voir [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview). |
| [Serveurs MCP](/docs/fr/mcp)                            | Configurer dans les fichiers de paramètres                                                   | Interface utilisateur Connecteurs pour les sessions locales et SSH, ou fichiers de paramètres                                                                                                                                                                                                                                                                                               |
| [Plugins](/docs/fr/plugins/overview)                    | Commande `/plugin`                                                                           | Interface utilisateur du gestionnaire de plugins                                                                                                                                                                                                                                                                                                                                            |
| Fichiers @mention                                  | Basé sur le texte                                                                            | Avec autocomplétion ; sessions locales et SSH uniquement                                                                                                                                                                                                                                                                                                                                    |
| Pièces jointes de fichiers                         | Non disponible                                                                               | Images, PDF                                                                                                                                                                                                                                                                                                                                                                                 |
| Isolation des sessions                             | Drapeau [`--worktree`](/docs/fr/cli-reference)                                                    | Option **worktree** au démarrage d'une session                                                                                                                                                                                                                                                                                                                                              |
| Sessions multiples                                 | Terminaux séparés                                                                            | Onglets de barre latérale                                                                                                                                                                                                                                                                                                                                                                   |
| Tâches récurrentes                                 | Tâches cron, pipelines CI                                                                    | [Tâches planifiées](/docs/fr/desktop-scheduled-tasks)                                                                                                                                                                                                                                                                                                                                            |
| Utilisation informatique                           | [Activer via `/mcp`](/docs/fr/computer-use) sur macOS                                             | [Contrôle d'application et d'écran](#let-claude-use-your-computer) sur macOS et Windows                                                                                                                                                                                                                                                                                                     |
| Simulateur iOS                                     | Piloter le simulateur via [utilisation informatique](/docs/fr/computer-use#test-a-simulator-flow) | [Volet Simulateur iOS](/docs/fr/desktop-ios-simulator) s'ouvre automatiquement                                                                                                                                                                                                                                                                                                                   |
| Intégration Dispatch                               | Non disponible                                                                               | [Sessions Dispatch](#sessions-from-dispatch) dans la barre latérale                                                                                                                                                                                                                                                                                                                         |
| Scripts et automatisation                          | [`--print`](/docs/fr/cli-reference), [Agent SDK](/docs/fr/headless)                                    | Non disponible                                                                                                                                                                                                                                                                                                                                                                              |

<h3 id="what’s-not-available-in-desktop">
  Ce qui n'est pas disponible dans Desktop
</h3>

Les fonctionnalités suivantes ne sont pas disponibles dans Desktop, sauf indication contraire :

* **Fournisseurs tiers** : Desktop se connecte à l'API d'Anthropic par défaut. Pour router Desktop via une passerelle, ou pour exécuter l'onglet Code sur Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry ou une passerelle LLM auto-hébergée, suivez les liens dans la [ligne Fournisseurs tiers](#feature-comparison).
* **Linux (bêta)** : l'utilisation informatique n'est pas encore disponible dans l'application de bureau Linux. Voir [Claude Desktop sur Linux](/docs/fr/desktop-linux).
* **Suggestions de code en ligne** : Desktop ne fournit pas de suggestions de style autocomplétion. Il fonctionne via des prompts conversationnels et des modifications de code explicites.
* **Équipes d'agents** : les équipes coordonnées, où Claude en tant que chef d'équipe assigne des tâches aux coéquipiers à partir d'une liste de tâches partagée, sont disponibles dans la [CLI](/docs/fr/agent-teams), pas dans Desktop. Pour le travail multi-agents dans une seule session, utilisez les [flux de travail dynamiques](/docs/fr/workflows), qui s'exécutent dans Desktop ; Claude peut également [envoyer des messages et gérer vos autres sessions](#work-across-sessions) directement.
* **Commandes de dialogue terminal** : les commandes intégrées qui ouvrent un panneau interactif dans le terminal se comportent différemment dans l'onglet Code. Modifiez les [fichiers de paramètres](/docs/fr/settings) directement pour gérer les règles de permission et la configuration, ou exécutez les commandes à partir de la CLI autonome.
  * Les commandes sans forme d'argument, telles que `/permissions`, répondent avec `n'est pas disponible dans cet environnement`.
  * `/config` ouvre Paramètres → Claude Code. Le texte après la commande est ignoré, donc `/config theme=dark` ne définit pas le thème.

<h2 id="troubleshooting">
  Dépannage
</h2>

Les sections ci-dessous couvrent les problèmes spécifiques à l'application de bureau. Pour les erreurs API d'exécution qui apparaissent dans le chat comme `API Error: 500`, `529 Overloaded`, `429` ou `Prompt is too long`, voir la [référence des erreurs](/docs/fr/errors). Ces erreurs et leurs correctifs sont les mêmes sur la CLI, le bureau et le web.

<h3 id="check-your-version">
  Vérifier votre version
</h3>

Pour voir quelle version de l'application de bureau vous exécutez :

* **macOS** : cliquez sur **Claude** dans la barre de menu, puis **À propos de Claude**
* **Windows** : cliquez sur **Aide**, puis **À propos**

Cliquez sur le numéro de version pour le copier dans votre presse-papiers.

<h3 id="403-or-authentication-errors-in-the-code-tab">
  Erreurs 403 ou d'authentification dans l'onglet Code
</h3>

Si vous voyez `Error 403: Forbidden` ou d'autres défaillances d'authentification lors de l'utilisation de l'onglet Code :

1. Déconnectez-vous et reconnectez-vous à partir du menu de l'application. C'est le correctif le plus courant.
2. Vérifiez que vous avez un abonnement payant actif : Pro, Max, Team ou Enterprise.
3. Si la CLI fonctionne mais Desktop ne fonctionne pas, quittez complètement l'application de bureau, pas seulement fermez la fenêtre, puis rouvrez et reconnectez-vous.
4. Vérifiez votre connexion Internet et vos paramètres de proxy.

<h3 id="blank-or-stuck-screen-on-launch">
  Écran blanc ou bloqué au lancement
</h3>

Si l'application s'ouvre mais affiche un écran blanc ou ne répond pas :

1. Redémarrez l'application.
2. Vérifiez les mises à jour en attente. Sur macOS et Windows, l'application se met à jour automatiquement au lancement ; sur Linux, mettez à jour via apt comme décrit dans [Claude Desktop sur Linux](/docs/fr/desktop-linux).
3. Sur un réseau géré, confirmez que votre pare-feu autorise les hôtes CDN dans les [exigences d'accès réseau](#network-access-requirements).
4. Sur Windows, vérifiez l'Observateur d'événements pour les journaux de crash sous **Journaux Windows → Application**.

<h3 id="failed-to-load-session">
  « Impossible de charger la session »
</h3>

Si vous voyez `Failed to load session`, le dossier sélectionné peut ne plus exister, un référentiel Git peut nécessiter Git LFS qui n'est pas installé, ou les permissions de fichier peuvent empêcher l'accès. Essayez de sélectionner un dossier différent ou redémarrez l'application.

<h3 id="session-not-finding-installed-tools">
  Session ne trouvant pas les outils installés
</h3>

Si Claude ne peut pas trouver des outils comme `npm`, `node` ou d'autres commandes CLI, vérifiez que les outils fonctionnent dans votre terminal régulier, vérifiez que votre profil shell configure correctement PATH et redémarrez l'application de bureau pour recharger les variables d'environnement.

<h3 id="git-and-git-lfs-errors">
  Erreurs Git et Git LFS
</h3>

Les sessions qui s'exécutent dans leur propre worktree ont besoin de Git. Si vous voyez « Git is required », installez [Git](https://git-scm.com/downloads), ou [Git pour Windows](https://git-scm.com/downloads/win) sur Windows, et réessayez. Sur Windows, les versions de Claude Desktop antérieures à 1.49585.0 demandaient Git avant de démarrer une session locale ; si vous voyez cette invite et n'utilisez pas de worktrees, mettez à jour l'application.

Si vous voyez « Git LFS is required by this repository but is not installed », installez Git LFS à partir de [git-lfs.com](https://git-lfs.com/), exécutez `git lfs install` et redémarrez l'application.

<h3 id="mcp-servers-not-working-on-windows">
  Les serveurs MCP ne fonctionnent pas sur Windows
</h3>

Si les bascules du serveur MCP ne répondent pas ou que les serveurs ne se connectent pas sur Windows, vérifiez que le serveur est correctement configuré dans vos paramètres, redémarrez l'application, vérifiez que le processus du serveur s'exécute dans le Gestionnaire des tâches et examinez les journaux du serveur pour les erreurs de connexion.

<h3 id="app-won’t-quit">
  L'application ne veut pas quitter
</h3>

* **macOS** : appuyez sur Cmd+Q. Si l'application ne répond pas, utilisez Forcer à quitter avec Cmd+Option+Esc, sélectionnez Claude et cliquez sur Forcer à quitter.
* **Windows** : utilisez le Gestionnaire des tâches avec Ctrl+Maj+Esc pour terminer le processus Claude.

<h3 id="windows-specific-issues">
  Problèmes spécifiques à Windows
</h3>

* **PATH non mis à jour après l'installation** : ouvrez une nouvelle fenêtre de terminal. Les mises à jour PATH s'appliquent uniquement aux nouvelles sessions de terminal.
* **Erreur d'installation simultanée** : si vous voyez une erreur concernant une autre installation en cours mais qu'il n'y en a pas, essayez d'exécuter l'installateur en tant qu'administrateur.

<h3 id="branch-doesn’t-exist-yet-when-opening-in-cli">
  « La branche n'existe pas encore » lors de l'ouverture dans la CLI
</h3>

Les sessions cloud peuvent créer des branches qui n'existent pas sur votre machine locale. Cliquez sur le nom de la branche dans la barre d'outils de la session pour le copier, puis récupérez-le localement :

```bash theme={null}
git fetch origin <branch-name>
git checkout <branch-name>
```

<h3 id="still-stuck">
  Toujours bloqué ?
</h3>

* Ouvrez Aide → Obtenir de l'aide dans l'application de bureau, ou visitez le [centre de support Claude](https://support.claude.com/) directement
* Pour les problèmes qui se reproduisent également dans la CLI autonome `claude`, recherchez ou signalez un bug sur [GitHub Issues](https://github.com/anthropics/claude-code/issues)

Lors du signalement d'un problème, incluez la version de votre application de bureau, votre système d'exploitation, le message d'erreur exact et les journaux pertinents. Sur macOS, vérifiez Console.app. Sur Windows, vérifiez Observateur d'événements → Journaux Windows → Application. Examinez les extraits de journal avant de les publier sur un problème public ; ils peuvent inclure des chemins de fichiers et d'autres détails de votre environnement.
