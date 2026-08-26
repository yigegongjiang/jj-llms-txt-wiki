> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code dans Slack

> Déléguez les tâches de codage directement depuis votre espace de travail Slack

<Note>
  Claude Code dans Slack est remplacé par [Claude Tag](https://claude.com/product/tag) pour les espaces de travail Team et Enterprise. Claude Tag exécute @Claude en tant qu'identité partagée de votre organisation avec un accès configuré par l'administrateur, sous la même application Slack, il n'y a donc rien à réinstaller et les configurations existantes continuent de fonctionner pendant la transition. Pour basculer un espace de travail, consultez [Migrer depuis la version antérieure de Claude dans Slack](https://claude.com/docs/claude-tag/admins/migrate-from-earlier).
</Note>

Claude Code dans Slack apporte la puissance de Claude Code directement dans votre espace de travail Slack. Lorsque vous mentionnez `@Claude` avec une tâche de codage, Claude détecte automatiquement l'intention et crée une session Claude Code sur le web, vous permettant de déléguer le travail de développement sans quitter vos conversations d'équipe.

Cette intégration est construite sur l'application Claude existante pour Slack, mais ajoute un routage intelligent vers Claude Code sur le web pour les demandes liées au codage. Chaque session s'exécute sous votre propre compte Claude, en utilisant vos référentiels connectés et vos limites de plan.

<h2 id="use-cases">
  Cas d'usage
</h2>

* **Enquête et correction de bugs** : Demandez à Claude d'enquêter et de corriger les bugs dès qu'ils sont signalés dans les canaux Slack.
* **Révisions de code rapides et modifications** : Faites en sorte que Claude implémente de petites fonctionnalités ou refactorise le code en fonction des commentaires de l'équipe.
* **Débogage collaboratif** : Lorsque les discussions d'équipe fournissent un contexte crucial (par exemple, les reproductions d'erreurs ou les rapports d'utilisateurs), Claude peut utiliser ces informations pour éclairer son approche du débogage.
* **Exécution de tâches parallèles** : Lancez des tâches de codage dans Slack tout en continuant d'autres travaux, en recevant des notifications à la fin.

<h2 id="prerequisites">
  Conditions préalables
</h2>

Avant d'utiliser Claude Code dans Slack, assurez-vous d'avoir les éléments suivants :

| Condition              | Détails                                                                                             |
| :--------------------- | :-------------------------------------------------------------------------------------------------- |
| Plan Claude            | Pro, Max, Team ou Enterprise avec accès à Claude Code (sièges premium ou sièges Chat + Claude Code) |
| Claude Code sur le web | L'accès à [Claude Code sur le web](/docs/fr/claude-code-on-the-web) doit être activé                     |
| Compte GitHub          | Connecté à Claude Code sur le web avec au moins un référentiel authentifié                          |
| Authentification Slack | Votre compte Slack lié à votre compte Claude via l'application Claude                               |

<h2 id="setting-up-claude-code-in-slack">
  Configuration de Claude Code dans Slack
</h2>

<Steps>
  <Step title="Installer l'application Claude dans Slack">
    Un administrateur de l'espace de travail doit installer l'application Claude à partir de la Slack App Marketplace. Visitez la [Slack App Marketplace](https://slack.com/marketplace/A08SF47R6P4) et cliquez sur ' Ajouter à Slack ' pour commencer le processus d'installation.
  </Step>

  <Step title="Connectez votre compte Claude">
    Une fois l'application installée, authentifiez votre compte Claude individuel :

    1. Ouvrez l'application Claude dans Slack en cliquant sur « Claude » dans votre section Applications
    2. Accédez à l'onglet Accueil de l'application
    3. Cliquez sur « Connecter » pour lier votre compte Slack à votre compte Claude
    4. Complétez le flux d'authentification dans votre navigateur
  </Step>

  <Step title="Configurez Claude Code sur le web">
    Assurez-vous que votre Claude Code sur le web est correctement configuré :

    * Visitez [claude.ai/code](https://claude.ai/code) et connectez-vous avec le même compte que celui que vous avez connecté à Slack
    * Connectez votre compte GitHub s'il n'est pas déjà connecté
    * Authentifiez au moins un référentiel avec lequel vous souhaitez que Claude travaille
  </Step>

  <Step title="Choisissez votre mode de routage">
    Après avoir connecté vos comptes, configurez la façon dont Claude gère vos messages dans Slack. Accédez à l'Accueil de l'application Claude dans Slack pour trouver le paramètre **Mode de routage**.

    | Mode                | Comportement                                                                                                                                                                                                                                                                            |
    | :------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | **Code uniquement** | Claude achemine toutes les @mentions vers les sessions Claude Code. Idéal pour les équipes utilisant Claude dans Slack exclusivement pour les tâches de développement.                                                                                                                  |
    | **Code + Chat**     | Claude analyse chaque message et achemine intelligemment entre Claude Code (pour les tâches de codage) et Claude Chat (pour la rédaction, l'analyse et les questions générales). Idéal pour les équipes qui souhaitent un point d'entrée @Claude unique pour tous les types de travail. |

    <Note>
      En mode Code + Chat, si Claude achemine un message vers Chat mais que vous vouliez une session de codage, vous pouvez cliquer sur « Réessayer en tant que Code » pour créer une session Claude Code à la place. De même, s'il est acheminé vers Code mais que vous vouliez une session Chat, vous pouvez choisir cette option dans ce fil.
    </Note>
  </Step>

  <Step title="Ajoutez Claude aux canaux">
    Claude n'est pas automatiquement ajouté à aucun canal après l'installation. Pour utiliser Claude dans un canal, invitez-le en tapant `/invite @Claude` dans ce canal. Claude ne peut répondre aux @mentions que dans les canaux où il a été ajouté.
  </Step>
</Steps>

<h2 id="how-it-works">
  Fonctionnement
</h2>

<h3 id="automatic-detection">
  Détection automatique
</h3>

Lorsque vous mentionnez @Claude dans un canal ou un fil Slack, Claude analyse automatiquement votre message pour déterminer s'il s'agit d'une tâche de codage. Si Claude détecte une intention de codage, il acheminera votre demande vers Claude Code sur le web au lieu de répondre en tant qu'assistant de chat ordinaire.

Vous pouvez également dire explicitement à Claude de traiter une demande en tant que tâche de codage, même s'il ne la détecte pas automatiquement.

<Note>
  Claude Code dans Slack ne fonctionne que dans les canaux (publics ou privés). Il ne fonctionne pas dans les messages directs (DM).
</Note>

<h3 id="context-gathering">
  Collecte de contexte
</h3>

**À partir des fils** : Lorsque vous @mentionnez Claude dans un fil, il recueille le contexte de tous les messages de ce fil pour comprendre la conversation complète.

**À partir des canaux** : Lorsqu'il est mentionné directement dans un canal, Claude examine les messages récents du canal pour un contexte pertinent.

Ce contexte aide Claude à comprendre le problème, à sélectionner le référentiel approprié et à éclairer son approche de la tâche.

<Warning>
  Lorsque @Claude est invoqué dans Slack, Claude a accès au contexte de la conversation pour mieux comprendre votre demande. Claude peut suivre les directives d'autres messages dans le contexte, donc les utilisateurs doivent s'assurer d'utiliser Claude uniquement dans des conversations Slack de confiance.
</Warning>

<h3 id="session-flow">
  Flux de session
</h3>

1. **Initiation** : Vous @mentionnez Claude avec une demande de codage
2. **Détection** : Claude analyse votre message et détecte l'intention de codage
3. **Création de session** : Une nouvelle session Claude Code est créée sur claude.ai/code
4. **Mises à jour de progression** : Claude publie des mises à jour de statut dans votre fil Slack au fur et à mesure de la progression du travail
5. **Achèvement** : Une fois terminé, Claude vous @mentionne avec un résumé et des boutons d'action
6. **Révision** : Cliquez sur ' Afficher la session ' pour voir la transcription complète, ou ' Créer une PR ' pour ouvrir une demande de tirage

<h2 id="user-interface-elements">
  Éléments de l'interface utilisateur
</h2>

<h3 id="app-home">
  Accueil de l'application
</h3>

L'onglet Accueil de l'application affiche votre statut de connexion et vous permet de connecter ou de déconnecter votre compte Claude de Slack.

<h3 id="message-actions">
  Actions de message
</h3>

* **Afficher la session** : Ouvre la session Claude Code complète dans votre navigateur où vous pouvez voir tout le travail effectué, continuer la session ou faire des demandes supplémentaires.
* **Créer une PR** : Crée une demande de tirage directement à partir des modifications de la session.
* **Réessayer en tant que Code** : Si Claude a initialement répondu en tant qu'assistant de chat mais que vous vouliez une session de codage, cliquez sur ce bouton pour réessayer la demande en tant que tâche Claude Code.
* **Changer de référentiel** : Vous permet de sélectionner un référentiel différent si Claude a mal choisi.

<h3 id="repository-selection">
  Sélection du référentiel
</h3>

Claude sélectionne automatiquement un référentiel en fonction du contexte de votre conversation Slack. Si plusieurs référentiels pourraient s'appliquer, Claude peut afficher une liste déroulante vous permettant de choisir le bon.

<h2 id="access-and-permissions">
  Accès et permissions
</h2>

<h3 id="user-level-access">
  Accès au niveau utilisateur
</h3>

| Type d'accès                    | Condition                                                                                    |
| :------------------------------ | :------------------------------------------------------------------------------------------- |
| Sessions Claude Code            | Chaque utilisateur exécute les sessions sous son propre compte Claude                        |
| Utilisation et limites de débit | Les sessions comptent par rapport aux limites du plan de l'utilisateur individuel            |
| Accès au référentiel            | Les utilisateurs ne peuvent accéder qu'aux référentiels qu'ils ont personnellement connectés |
| Historique des sessions         | Les sessions apparaissent dans votre historique Claude Code sur claude.ai/code               |

<h3 id="workspace-level-access">
  Accès au niveau de l'espace de travail
</h3>

Les administrateurs de l'espace de travail Slack contrôlent si l'application Claude est disponible dans leur espace de travail :

| Contrôle                      | Description                                                                                                                                               |
| :---------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Installation de l'application | Les administrateurs de l'espace de travail décident d'installer ou non l'application Claude à partir de la Slack App Marketplace                          |
| Distribution Enterprise Grid  | Pour les organisations Enterprise Grid, les administrateurs de l'organisation peuvent contrôler quels espaces de travail ont accès à l'application Claude |
| Suppression de l'application  | La suppression de l'application d'un espace de travail révoque immédiatement l'accès pour tous les utilisateurs de cet espace de travail                  |

<h3 id="channel-based-access-control">
  Contrôle d'accès basé sur les canaux
</h3>

Claude n'est pas automatiquement ajouté à aucun canal après l'installation. Les utilisateurs doivent explicitement inviter Claude aux canaux où ils souhaitent l'utiliser :

* **Invitation requise** : Tapez `/invite @Claude` dans n'importe quel canal pour ajouter Claude à ce canal
* **L'appartenance au canal contrôle l'accès** : Claude ne peut répondre aux @mentions que dans les canaux où il a été ajouté
* **Contrôle d'accès via les canaux** : Les administrateurs peuvent contrôler qui utilise Claude Code en gérant les canaux auxquels Claude est invité et qui a accès à ces canaux
* **Support des canaux privés** : Claude fonctionne dans les canaux publics et privés, donnant aux équipes la flexibilité de contrôler la visibilité

Ce modèle basé sur les canaux permet aux équipes de restreindre l'utilisation de Claude Code à des canaux spécifiques, fournissant une couche supplémentaire de contrôle d'accès au-delà des permissions au niveau de l'espace de travail.

<h2 id="what’s-accessible-where">
  Ce qui est accessible où
</h2>

**Dans Slack** : Vous verrez les mises à jour de statut, les résumés d'achèvement et les boutons d'action. La transcription complète est préservée et toujours accessible.

**Sur le web** : La session Claude Code complète avec l'historique complet de la conversation, tous les changements de code, les opérations de fichiers et la possibilité de continuer la session ou de créer des demandes de tirage.

Pour les comptes Enterprise et Team, les sessions créées à partir de Claude dans Slack sont automatiquement visibles pour l'organisation. Consultez [Partage de sessions Claude Code sur le web](/docs/fr/claude-code-on-the-web#share-sessions) pour plus de détails.

<h2 id="best-practices">
  Meilleures pratiques
</h2>

<h3 id="writing-effective-requests">
  Rédaction de demandes efficaces
</h3>

* **Soyez précis** : Incluez les noms de fichiers, les noms de fonctions ou les messages d'erreur si pertinent.
* **Fournissez du contexte** : Mentionnez le référentiel ou le projet s'il n'est pas clair à partir de la conversation.
* **Définissez le succès** : Expliquez à quoi ressemble ' terminé ' — Claude devrait-il écrire des tests ? Mettre à jour la documentation ? Créer une PR ?
* **Utilisez les fils** : Répondez dans les fils lors de la discussion de bugs ou de fonctionnalités afin que Claude puisse recueillir le contexte complet.

<h3 id="when-to-use-slack-vs-web">
  Quand utiliser Slack par rapport au web
</h3>

**Utilisez Slack quand** : Le contexte existe déjà dans une discussion Slack, vous souhaitez lancer une tâche de manière asynchrone, ou vous collaborez avec des coéquipiers qui ont besoin de visibilité.

**Utilisez le web directement quand** : Vous devez télécharger des fichiers, souhaitez une interaction en temps réel pendant le développement, ou travaillez sur des tâches plus longues et plus complexes.

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="claude-code-is-not-enabled-for-your-account">
  ' Claude Code n'est pas activé pour votre compte '
</h3>

Cette erreur signifie que votre compte Claude n'a pas encore d'environnement cloud, et non qu'un administrateur doit activer quelque chose. Connectez-vous à [claude.ai/code](https://claude.ai/code) une fois avec le même compte que celui que vous avez connecté à Slack. La première visite crée votre environnement cloud par défaut, et l'erreur disparaît lors de votre prochaine mention. Chaque utilisateur doit faire cela individuellement.

<h3 id="sessions-not-starting">
  Les sessions ne démarrent pas
</h3>

1. Vérifiez que votre compte Claude est connecté dans l'Accueil de l'application Claude
2. Vérifiez que vous avez accès à Claude Code sur le web activé
3. Assurez-vous que vous avez au moins un référentiel GitHub connecté à Claude Code

<h3 id="repository-not-showing">
  Le référentiel ne s'affiche pas
</h3>

1. Connectez le référentiel dans Claude Code sur le web à [claude.ai/code](https://claude.ai/code)
2. Vérifiez vos permissions GitHub pour ce référentiel
3. Essayez de déconnecter et de reconnecter votre compte GitHub

<h3 id="wrong-repository-selected">
  Mauvais référentiel sélectionné
</h3>

1. Cliquez sur le bouton ' Changer de référentiel ' pour sélectionner un référentiel différent
2. Incluez le nom du référentiel dans votre demande pour une sélection plus précise

<h3 id="authentication-errors">
  Erreurs d'authentification
</h3>

1. Déconnectez et reconnectez votre compte Claude dans l'Accueil de l'application
2. Assurez-vous que vous êtes connecté au bon compte Claude dans votre navigateur
3. Vérifiez que votre plan Claude inclut l'accès à Claude Code

<h3 id="session-expiration">
  Expiration de la session
</h3>

1. Les sessions restent accessibles dans votre historique Claude Code sur le web
2. Vous pouvez continuer ou référencer les sessions passées à partir de [claude.ai/code](https://claude.ai/code)

<h2 id="current-limitations">
  Limitations actuelles
</h2>

* **GitHub uniquement** : Prend actuellement en charge les référentiels sur GitHub.
* **Une PR à la fois** : Chaque session peut créer une demande de tirage.
* **Les limites de débit s'appliquent** : Les sessions utilisent les limites de débit du plan Claude individuel.
* **Accès web requis** : Les utilisateurs doivent avoir accès à Claude Code sur le web ; ceux qui ne l'ont pas recevront uniquement des réponses de chat Claude standard.

<h2 id="related-resources">
  Ressources connexes
</h2>

<CardGroup>
  <Card title="Claude Code sur le web" icon="globe" href="/docs/fr/claude-code-on-the-web">
    En savoir plus sur Claude Code sur le web
  </Card>

  <Card title="Claude pour Slack" icon="slack" href="https://claude.com/claude-and-slack">
    Documentation générale de Claude pour Slack
  </Card>

  <Card title="Claude Tag" icon="users" href="https://claude.com/docs/claude-tag/overview">
    @Claude géré par l'organisation dans Slack avec accès configuré par l'administrateur
  </Card>

  <Card title="Slack App Marketplace" icon="store" href="https://slack.com/marketplace/A08SF47R6P4">
    Installez l'application Claude à partir de la Slack Marketplace
  </Card>

  <Card title="Centre d'aide Claude" icon="circle-question" href="https://support.claude.com">
    Obtenir un support supplémentaire
  </Card>
</CardGroup>
