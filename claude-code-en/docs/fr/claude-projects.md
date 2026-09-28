> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Laissez Claude coordonner le travail en cours avec Projects

> Donnez à Claude un ensemble de travaux connexes dans une conversation et laissez-le coordonner des sessions cloud parallèles qui partagent des référentiels, des instructions et la mémoire.

<Note>
  Projects est en bêta publique sur les plans Pro et Max et se déploie progressivement, en commençant par les comptes qui ont utilisé les [sessions cloud](/docs/fr/claude-code-on-the-web) et qui n'ont pas de projets existants dans le chat claude.ai ou Cowork. Il n'est pas encore disponible sur les plans Team ou Enterprise. Si **Projects** n'apparaît pas dans la barre latérale sur [claude.ai/code](https://claude.ai/code) ou dans l'onglet Code de l'[application de bureau](/docs/fr/desktop), le déploiement n'a pas encore atteint votre compte, et vous pouvez [rejoindre la liste d'attente](https://claude.com/form/projects). [Exécuter des agents en parallèle](/docs/fr/agents) énumère ce que vous pouvez utiliser en attendant.
</Note>

Un projet est une conversation en cours unique où Claude coordonne un flux de travaux connexes pour vous. Vous lui dites ce qui doit être fait et il démarre un thread pour chaque tâche.

Chaque thread est généralement une [session cloud](/docs/fr/claude-code-on-the-web) : Claude Code s'exécutant dans le cloud plutôt que sur votre machine. Quand une tâche a besoin de quelque chose que seul votre ordinateur possède, vous pouvez demander à Claude d'exécuter ce thread sur votre ordinateur à la place via [Remote Control](/docs/fr/remote-control). Les threads s'exécutent en parallèle et vous pouvez les vérifier et les diriger depuis votre téléphone. Les threads cloud continuent après que vous ayez fermé l'ordinateur portable.

Sans projet, l'exécution de plusieurs sessions signifie faire la coordination vous-même : vous décidez sur quoi chacun travaille, répétez le même contexte au début de chacun, et vérifiez lequel a terminé ou a besoin d'une réponse. Avec un projet, vous pouvez plutôt :

* **Envoyer le travail à un seul endroit** : collez un rapport de bug, une trace de pile ou une liste de tâches dans la conversation chaque fois qu'une apparaît. Claude démarre un thread pour chaque élément de travail ou le transmet au thread déjà en train de travailler dans ce domaine, et répond aux questions rapides sur place.
* **Définir le contexte une seule fois** : chaque nouveau thread commence avec les instructions du projet, donc une règle que vous énoncez une seule fois, comme la branche à cibler, atteint tous les threads.
* **Partez et revenez au travail terminé** : quand vous revenez une heure plus tard ou le lendemain matin, le volet **Overview** montre quels threads ont terminé, quelles pull requests sont prêtes pour examen, et quel thread attend votre réponse.

Si vous connaissez déjà le travail que vous voulez qu'un projet exécute, allez directement à [Créer un projet](#create-a-project).

<h2 id="when-to-use-a-project">
  Quand utiliser un projet
</h2>

Un projet vaut la peine d'être créé quand le travail a un objectif qui dépasse une session et continue à produire des tâches. Ces types de travaux conviennent bien à un projet :

* **Un objectif sur plusieurs référentiels** : « Mettre à jour chaque service avec la nouvelle configuration lint. » Claude peut exécuter un thread par référentiel, chacun avec sa propre pull request, et le volet [**Overview**](#see-what-needs-you-in-overview) montre lesquels sont prêts pour examen.
* **Un domaine que vous continuez à alimenter** : les bugs, les traces de pile et les demandes d'examen pour un service, collés dans la conversation au fur et à mesure qu'ils vous parviennent. Une mise en garde que vous dites à Claude de mémoriser après une correction est dans la [mémoire du projet](#give-a-project-standing-context) pour la prochaine.
* **Une construction ou une migration plus grande qu'une session** : « Construire ce que `docs/spec.md` décrit » ou « Migrer l'application hors de l'ORM obsolète. » Le travail se divise en threads qui prennent chacun une partie, les décisions que vous demandez à Claude de mémoriser au début atteignent les threads ultérieurs, et la spécification change et les bugs que vous trouvez pendant la construction vont dans la même conversation.
* **Un travail qui n'est pas du code** : un dossier de contrats ou une exportation de tickets d'assistance sur laquelle vous revenez continuellement avec de nouvelles questions, comme « trouver les dix erreurs d'intégration les plus courantes dans ces tickets. » Téléchargez les documents au lieu d'ajouter un référentiel, et les threads livrent chaque rapport en tant que fichier sur l'onglet [**Library**](#see-what-needs-you-in-overview) du projet.

Dans n'importe lequel d'entre eux, vous pouvez envoyer un lot de tâches, dire à Claude de commencer sans vous demander de confirmer, vous éloigner et trouver les threads qui vous attendent sous [**Waiting on you**](#see-what-needs-you-in-overview) quand vous êtes de retour, ou demander à Claude de mettre une partie du travail sur un calendrier en tant que [routine](/docs/fr/routines). Si c'est votre situation, [créez un projet](#create-a-project).

<h3 id="when-something-else-fits-better">
  Quand quelque chose d'autre convient mieux
</h3>

Les threads cloud fonctionnent sur les référentiels GitHub et sur les fichiers, dossiers et dossiers Google Drive que vous téléchargez vers le projet, pas sur les fichiers ou outils qui n'existent que sur votre machine. Si une tâche nécessite votre machine, demandez à Claude d'exécuter son thread là-bas via [Remote Control](/docs/fr/remote-control). [Limitations](#limitations) énumère ce dont cela a besoin. Quelque chose d'autre convient mieux dans ces cas :

* **Une tâche qui tient dans une session** : « Corriger le test de connexion instable. » Démarrez une [session cloud](/docs/fr/claude-code-on-the-web) vous-même.
* **Un travail où chaque tâche nécessite votre machine** : une base de données locale, un émulateur d'appareil, ou une API derrière votre VPN. Utilisez une session locale, ou [agent view](/docs/fr/agent-view) pour en exécuter plusieurs à la fois. Si le travail ne nécessite que des fichiers locaux, téléchargez-les vers le projet à la place.
* **Une tâche qui se répète selon un calendrier sans conversation autour** : « Publier un rapport de dépendance chaque lundi. » Créez une [routine](/docs/fr/routines) seule.
* **Plusieurs personnes donnant du travail à Claude et le dirigeant ensemble dans un canal Slack** : voir [Claude Tag](https://claude.com/docs/claude-tag/overview).

Un projet utilise les mêmes limites de plan que vos autres sessions Claude Code et les utilise plus rapidement. [Utilisation et coût](#usage-and-cost) couvre ce qui utilise votre plan et comment le réduire.

<h2 id="how-a-project-is-organized">
  Comment un projet est organisé
</h2>

Un projet est une conversation de coordination unique avec Claude plus les threads qu'il démarre pour faire le travail. Voici ses parties :

* **La conversation du projet** : une session longue durée unique où Claude agit en tant que coordinateur. Il prend ce que vous envoyez, décide ce qui devient un thread, et garde une trace de chaque thread qu'il a démarré. Il voit ce que les threads rapportent, pas chaque étape qu'ils prennent.
* **Threads** : les travailleurs. Chacun est une session distincte avec sa propre fenêtre de contexte qui fait un morceau de travail et rapporte à la conversation quand il termine. Un thread cloud travaille sur sa propre branche et ouvre une pull request quand le travail l'exige.
* **Ce que chaque thread cloud commence avec** :
  * Les référentiels et fichiers du projet, plus ses [instructions et mémoire](#give-a-project-standing-context)
  * Le `CLAUDE.md` et skills dans [chacun des référentiels du projet](#what-threads-pick-up-from-your-repositories), et dans un projet avec un référentiel, les règles de permission et les hooks de ce référentiel aussi
  * Les [connecteurs](#get-skills-plugins-connectors-and-tools-into-threads) sur votre compte claude.ai
  * Un [environnement cloud](#choose-an-environment-for-threads) qui définit son accès réseau, les variables d'environnement, les identifiants API et les outils installés
* **Le volet Overview** : où vous [voyez tous les threads à la fois](#see-what-needs-you-in-overview) et lesquels vous attendent. Ses autres onglets sont **Library** pour les fichiers que vous avez ajoutés et les fichiers que les threads ont produits, **Pull requests** pour ceux que les threads ont ouverts, et **Routines** pour le travail programmé dans le projet.

Les threads cloud ne reprennent rien de la configuration Claude Code sur votre propre machine. [Obtenir des skills, des plugins, des connecteurs et des outils dans les threads](#get-skills-plugins-connectors-and-tools-into-threads) couvre comment leur donner ce qui leur manquerait autrement.

Voici comment ces parties se connectent, de vous à travers la conversation aux threads qui font le travail, avec **Overview** qui suit leur état :

<Frame>
  <img src="https://mintcdn.com/claude-code/e8CLbxM17eD7cAiv/images/claude-projects-overview.svg?fit=max&auto=format&n=e8CLbxM17eD7cAiv&q=85&s=dbf446f69f0bbdb9961d21af207cb93b" className="dark:hidden" alt="Diagramme d'un projet. Vous écrivez dans la conversation du projet, où Claude répond ou démarre un thread. Chaque thread cloud travaille sur sa propre branche et pull request. Le volet Overview énumère les threads par état, comme prêt pour examen, en attente de vous et en cours." width="600" height="250" data-path="images/claude-projects-overview.svg" />

  <img src="https://mintcdn.com/claude-code/e8CLbxM17eD7cAiv/images/claude-projects-overview-dark.svg?fit=max&auto=format&n=e8CLbxM17eD7cAiv&q=85&s=549a5ba9fea8433729babc37a1f6e9c8" className="hidden dark:block" alt="Diagramme d'un projet. Vous écrivez dans la conversation du projet, où Claude répond ou démarre un thread. Chaque thread cloud travaille sur sa propre branche et pull request. Le volet Overview énumère les threads par état, comme prêt pour examen, en attente de vous et en cours." width="600" height="250" data-path="images/claude-projects-overview-dark.svg" />
</Frame>

<h2 id="create-a-project">
  Créer un projet
</h2>

Vous créez et utilisez des projets à [claude.ai/code](https://claude.ai/code), dans l'onglet Code de l'application de bureau, ou dans l'application mobile Claude pour [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) et [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude). Dans le navigateur et l'application de bureau, il y a deux façons de démarrer un projet :

* **À partir de zéro**, quand vous connaissez le flux de travail que vous voulez que Claude exécute : ouvrez la boîte de dialogue **New project** et nommez-la. [Démarrer un nouveau projet à partir de zéro](#start-a-new-project-from-scratch) parcourt la boîte de dialogue.
* **À partir d'une session cloud qui fait déjà le travail** : choisissez **Continue as a project** dans le menu de cette session, et Claude propose la configuration du projet à partir de ce que la session faisait. Voir [Démarrer à partir d'une session cloud existante](#start-from-an-existing-cloud-session).

De toute façon, [vérifiez d'abord les prérequis](#check-the-prerequisites).

<h3 id="check-the-prerequisites">
  Vérifier les prérequis
</h3>

Avant de créer un projet, vérifiez votre plan, votre configuration GitHub et ce que le travail doit atteindre :

* **Plan** : vous êtes sur Pro ou Max et **Projects** s'affiche dans votre barre latérale.
* **GitHub, si le projet travaillera sur du code** : votre code est sur github.com plutôt que sur GitHub Enterprise Server, GitLab ou Bitbucket, votre compte GitHub connecté y a accès en push, et l'application Claude GitHub est installée dessus. Si vous avez connecté GitHub avec [`/web-setup`](/docs/fr/web-quickstart#connect-from-your-terminal), ce token permet à vos autres sessions cloud d'atteindre un référentiel mais n'est pas suffisant pour les threads du projet, qui ont besoin de l'application Claude GitHub. [Configurer l'accès GitHub](#set-up-github-access) a les étapes.
* **Réseau, identifiants et outils** : ceux-ci proviennent de l'[environnement cloud](#choose-an-environment-for-threads) du projet. L'environnement par défaut atteint déjà [les registres de packages courants](/docs/fr/cloud-environments#default-allowed-domains), donc vérifiez ceci seulement si le travail a besoin d'autres domaines, d'un secret ou d'un outil qui n'est pas préinstallé. Si le travail a besoin d'un serveur MCP, vérifiez qu'il s'affiche comme connecté dans vos [connecteurs claude.ai](https://claude.ai/customize/connectors).

<h3 id="start-a-new-project-from-scratch">
  Démarrer un nouveau projet à partir de zéro
</h3>

Démarrer un projet à partir de zéro signifie ouvrir la boîte de dialogue **New project**, nommer le flux de travail et éventuellement lui donner un objectif et les référentiels et fichiers sur lesquels il travaille. Seul le nom est requis, vous pouvez donc créer le projet en premier et remplir le reste au fur et à mesure que le travail prend forme.

<Steps>
  <Step title="Ouvrir Projects">
    À [claude.ai/code](https://claude.ai/code) ou dans l'onglet Code de l'application de bureau, sélectionnez **Projects** dans la barre latérale gauche, puis sélectionnez **New project**. Dans un navigateur, vous pouvez également aller directement à [claude.ai/code/projects/browse](https://claude.ai/code/projects/browse).
  </Step>

  <Step title="Remplir la boîte de dialogue New project">
    Limitez le projet à un flux de travail que vous continuerez à ajouter, comme tout ce qu'il faut pour maintenir une API sous sa cible de latence. [Quand utiliser un projet](#when-to-use-a-project) a plus d'exemples. Ensuite, remplissez les champs de la boîte de dialogue :

    * **Name** : comment le projet apparaît dans la liste **Projects**.
    * **Goal** (optionnel) : une ligne de ce que vous essayez d'accomplir, comme « Maintenir la latence p95 de l'API sous 200 ms ». Claude dans la conversation travaille vers cela. Sans objectif, Claude travaille à partir des tâches que vous envoyez, et vous pouvez ajouter un objectif plus tard dans **Project settings > General**.
    * **Context** (optionnel) : les référentiels GitHub sur lesquels ce projet travaille, plus tous les fichiers, dossiers ou dossiers Google Drive que les threads doivent lire. Cliquez sur **Add** pour chacun. Ajoutez les référentiels dont la plupart des tâches ont besoin plutôt que tous ceux que le travail pourrait toucher ; [Décider quels référentiels ajouter](#decide-which-repositories-to-add) couvre le choix, et vous pouvez en ajouter d'autres plus tard dans **Project settings > Environment**.

    Les règles permanentes sur la façon dont les threads doivent fonctionner vont dans [les instructions du projet](#give-a-project-standing-context), que vous définissez après que le projet existe.
  </Step>

  <Step title="Créer le projet">
    Cliquez sur **Create project**. La conversation du projet s'ouvre avec une boîte de message en bas, où vous décrivez le travail pour Claude.

    Sur votre premier projet, Claude prend un tour de son propre chef dès que le projet est créé, sauf si vous envoyez d'abord un message. Ce tour utilise votre plan. En cela, Claude peut :

    * Démarrer un thread qui explore le référentiel sans rien changer et propose les prochaines étapes, si le projet a un référentiel qu'il peut lire.
    * Publier **Setup recommendations** tirées de vos sessions cloud récentes : référentiels à ajouter, routines à créer et threads qu'il pourrait démarrer. Chaque référentiel et routine recommandés commencent activés. Désactivez ceux que vous ne voulez pas, puis cliquez sur **Update setup** pour ajouter le reste, ou ignorez les recommandations et décrivez le travail vous-même.
  </Step>
</Steps>

Le projet est maintenant listé sous **Projects** dans la barre latérale, et sa conversation est ouverte. [Votre premier lot](#your-first-batch) couvre ce qu'il faut configurer avant de lui envoyer du travail.

<h3 id="start-from-an-existing-cloud-session">
  Démarrer à partir d'une session cloud existante
</h3>

Si vous avez déjà une session cloud qui fait du travail qui appartient à un projet, ouvrez le menu de la session dans la barre latérale et choisissez **Continue as a project** ou **Move to project** :

* **Continue as a project** crée un nouveau projet nommé d'après la session et l'ouvre. Claude lit la session et publie **Setup recommendations** dans la conversation pour que vous confirmiez. La session d'origine reste dans votre liste de sessions, et si elle était au milieu d'un tour, elle continue à s'exécuter, donc arrêtez-la vous-même si vous ne voulez pas que les deux fonctionnent à la fois. Si vous utilisez la bannière **Set up project** qui peut apparaître au-dessus de la boîte de message de la session cloud à la place, le résultat est le même, sauf que le tour en cours de la session s'arrête une fois que le projet s'ouvre.
* **Move to project** apporte le travail de la session dans un projet existant. Il publie un message dans la conversation de ce projet demandant à Claude de lire la session et de continuer là où elle s'était arrêtée, et le nouveau travail continue dans les propres threads du projet. La session d'origine reste dans votre liste de sessions, inchangée.

<h3 id="set-up-github-access">
  Configurer l'accès GitHub
</h3>

La plupart de la configuration GitHub se fait une fois, pas par projet. Vous connectez votre compte GitHub à Claude une fois, et l'application Claude GitHub est installée une fois par référentiel, ou une fois pour toute une organisation GitHub si vous lui donnez tous les référentiels. Vous revenez à ces étapes quand vous ajoutez un référentiel que l'application Claude GitHub ne couvre pas encore ou un dans une organisation GitHub qui applique SSO.

<Steps>
  <Step title="Connecter votre compte GitHub">
    Si vous n'avez pas utilisé claude.ai/code avant, votre première visite vous guide à travers la connexion de GitHub ; voir [Connecter GitHub](/docs/fr/web-quickstart#connect-github). Sinon, utilisez l'une des [options d'authentification GitHub](/docs/fr/claude-code-on-the-web#github-authentication-options).
  </Step>

  <Step title="Installer l'application Claude GitHub sur les référentiels du projet">
    Installez l'[application Claude GitHub](https://github.com/apps/claude) et accordez-lui les référentiels que le projet utilisera. Sur un référentiel appartenant à une organisation GitHub, seul un propriétaire d'organisation peut terminer l'installation ; si vous n'en êtes pas un, GitHub envoie au propriétaire une demande d'installation et le projet ne peut pas utiliser le référentiel jusqu'à ce qu'il l'approuve.
  </Step>

  <Step title="Autoriser SSO pour les organisations qui l'appliquent">
    Si une organisation GitHub applique SAML SSO, reconnectez GitHub et autorisez l'application Claude pour cette organisation. Jusqu'à ce que vous le fassiez, les référentiels privés de cette organisation n'apparaissent pas dans la boîte de dialogue **New project** ou **Project settings > Environment**.
  </Step>
</Steps>

Quand l'une de ces étapes est incomplète, la boîte de dialogue **New project** et la page du projet nomment l'étape manquante et créent un lien vers où vous la terminez. Terminez l'étape là, puis cliquez sur **Check again** si la boîte de dialogue l'offre. Si un référentiel manque toujours de la liste après, ouvrez l'installation de l'application Claude GitHub sur GitHub, à [github.com/settings/installations](https://github.com/settings/installations) pour un compte personnel, et confirmez que le référentiel est listé sous **Repository access**. Pour les messages d'erreur qu'un thread ou le projet rapporte quand l'accès est toujours mauvais, voir [Erreurs d'accès au référentiel](#repository-access-errors).

<h2 id="work-in-a-project">
  Travailler dans un projet
</h2>

Donnez du travail à Claude à travers la conversation du projet : des tâches une à la fois ou plusieurs à la fois, plus des mises à jour et des pensées vagues au fur et à mesure qu'elles arrivent. Claude achemine chaque message, et les threads font le travail et rapportent.

<h3 id="your-first-batch">
  Votre premier lot
</h3>

Avant d'envoyer à un nouveau projet un lot de travail, configurez-le pour que les premiers threads reviennent comme vous le souhaitez :

1. [Écrire les instructions du projet](#write-project-instructions) : le brief que chaque thread commence, comme la branche à cibler, comment un thread vérifie son travail et ce qui a besoin de votre approbation.
2. Envoyez un petit morceau du vrai travail, ou démarrez l'un des threads que Claude a suggérés, et ouvrez le thread quand il termine pour voir comment il rapporte et ce qu'il a fait sur sa branche. S'il a supposé quelque chose de mal ou ne pouvait pas atteindre ce dont il avait besoin, [Les threads ont deviné ou se sont arrêtés au lieu de demander](#threads-guessed-or-stalled-instead-of-asking) couvre où corriger cela.
3. Vérifiez **Thread model** et **Thread effort** dans **Project settings > General**. Un nouveau projet exécute chaque thread sur Opus à effort élevé, ce qui utilise votre plan le plus rapidement ; [Choisir des modèles et laisser Claude gérer le contexte](#choose-models-and-let-claude-manage-context) couvre les alternatives.
4. Demandez à Claude de [proposer des threads avant de les démarrer et d'en exécuter quelques-uns à la fois](#tune-how-claude-runs-a-project), et abandonnez ces limites une fois que quelques threads reviennent comme vous le souhaitez.

<h3 id="send-work-and-read-results">
  Envoyer du travail et lire les résultats
</h3>

Claude décide où va chaque message que vous envoyez dans la conversation :

* Une question rapide obtient généralement une réponse dans la conversation.
* Le nouveau travail va à un nouveau thread ou à un thread qui travaille déjà dans ce domaine, et Claude vous dit lequel. Chaque nouveau thread s'affiche sous votre message sous forme de carte : une boîte avec le titre et l'état du thread, que vous cliquez pour ouvrir le thread.
* Plusieurs tâches non liées dans un message deviennent des threads séparés.

Si Claude achemine quelque chose différemment de ce que vous vouliez, dites-le. [Affiner la façon dont Claude exécute un projet](#tune-how-claude-runs-a-project) énumère les choses que vous pouvez lui dire, comme réutiliser un thread existant pour les suites ou répondre sur place au lieu de démarrer un thread.

Les résultats complets d'un thread restent dans le thread, et vous ouvrez sa carte dans la conversation pour les lire. Les fichiers qu'un thread a produits sont également sur l'onglet **Library** dans **Overview**.

Parfois, Claude propose des threads au lieu de les démarrer, dans une liste **Suggested threads**. Cliquez sur la flèche d'une suggestion pour démarrer ce thread. Quand plusieurs sont listés, un bouton sous la liste les démarre tous.

<h3 id="review-a-thread’s-pull-request">
  Examiner la pull request d'un thread
</h3>

Quand un thread change du code, voici ce qu'il fait sauf si vous lui dites autrement :

* **Branch** : travaille sur une nouvelle branche, commencée à partir de la branche par défaut du référentiel.
* **Pull request** : en ouvre une quand vous le demandez, et peut en ouvrir une seule pour une correction de bug ou un autre changement concret.
* **Après l'ouverture** : surveille la pull request avec [auto-fix](/docs/fr/claude-code-on-the-web#auto-fix-pull-requests) activé, que auto-fix soit activé ou non pour vos autres sessions cloud. Il pousse des corrections quand CI échoue, traite les commentaires d'examen, et répond dans le thread quand les vérifications réussissent et la pull request est prête pour vous.

Quand un thread a poussé une branche ou ouvert une pull request, sa carte dans la conversation peut afficher un bouton pour l'étape suivante :

* **Resolve conflicts**, **Fix CI**, **Address comments**, et **Merge it** envoient cette instruction au thread en tant que message de vous, vous pouvez donc inviter le thread vous-même au lieu d'attendre qu'il réagisse à la pull request.
* **Review PR** ouvre la pull request sur GitHub.
* **Create PR** apparaît quand un thread inactif a poussé une branche mais n'a pas ouvert de pull request. Le cliquer crée la pull request à partir de cette branche directement plutôt que d'envoyer au thread une instruction pour en ouvrir une.

Pour changer quand les threads ouvrent les pull requests, par exemple seulement quand vous le demandez, ou quelle branche ils commencent, dites-le dans la tâche ou dans [les instructions du projet](#write-project-instructions).

<h3 id="see-what-needs-you-in-overview">
  Voir ce qui vous attend dans Overview
</h3>

Le volet **Overview** à côté de la conversation suit les threads du projet. Il est déjà ouvert la première fois que vous ouvrez un nouveau projet. Le bouton **Overview** dans l'en-tête du projet le ferme et le rouvre, et affiche un point quand un thread vous attend.

Dans l'application de bureau, vous obtenez également une notification de bureau quand Claude publie dans la conversation, un thread atteint une erreur ou un thread a besoin de votre entrée, vous n'avez donc pas besoin de garder le projet ouvert pour le découvrir. Pour obtenir également une notification chaque fois qu'un thread termine un tour, ou pour les désactiver pour un projet, choisissez **Notifications** dans le menu de la barre latérale du projet. Ces notifications sont uniquement de bureau : dans un navigateur, vérifiez le point sur le bouton **Overview**.

L'onglet **Threads** du volet groupe les threads par état :

| Groupe               | Ce qui s'y trouve                                                                                                                                                                                                                                                |
| :------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Ready for review** | Threads dont la pull request est ouverte et en attente d'examen                                                                                                                                                                                                  |
| **Waiting on you**   | Threads qui ont besoin de votre réponse ou approbation, ou qui ont échoué                                                                                                                                                                                        |
| **Working**          | Threads toujours en cours d'exécution                                                                                                                                                                                                                            |
| **Landing**          | Threads dont la pull request est approuvée ou en attente de fusion                                                                                                                                                                                               |
| **Idle**             | Threads qui ont terminé et n'attendent rien                                                                                                                                                                                                                      |
| **Resolved**         | Threads marqués comme terminés : par vous depuis le menu du thread, par Claude une fois que vous avez pris la dernière étape, comme fusionner sa pull request, ou automatiquement après une semaine sans activité. Vous pouvez en rouvrir un depuis le même menu |

Les autres onglets du volet sont **Library** pour les fichiers et dossiers que vous avez ajoutés et les fichiers que les threads ont produits, **Pull requests** une fois que les threads en ont ouvert, et **Routines** pour les [routines](/docs/fr/routines) que Claude a configurées à partir de ce projet.

<h3 id="open-a-thread-when-you-need-control">
  Ouvrir un thread quand vous avez besoin de contrôle
</h3>

Cliquez sur la carte d'un thread dans la conversation ou sa ligne dans **Overview** pour ouvrir sa transcription dans le volet Overview. De là, vous pouvez :

* Lire ce que Claude a fait, étape par étape.
* Diriger la tâche en écrivant dans la boîte de message propre du thread. Un message là va directement à ce thread, tandis qu'une suite dans la conversation du projet ne l'atteint que quand Claude correspond la suite à ce thread.
* Répondre à une invite de permission que le thread attend.
* Interrompre le thread avec **Stop**, qui remplace le bouton d'envoi pendant que le thread fonctionne, ou en appuyant sur Échap.

<h3 id="choose-models-and-let-claude-manage-context">
  Choisir des modèles et laisser Claude gérer le contexte
</h3>

Définissez les modèles et l'effort dans **Project settings > General**. Un nouveau projet exécute Opus partout, avec un [effort](/docs/fr/model-config#adjust-effort-level) élevé pour les threads et un effort faible pour la conversation :

* **Thread model** et **Thread effort** s'appliquent aux threads. Pour utiliser un modèle différent pour une tâche, demandez-le dans la tâche ; pour un thread déjà en cours d'exécution, utilisez le sélecteur de modèle de ce thread.
* **Coordinator model** et **Coordinator effort** s'appliquent à Claude dans la conversation du projet.

Vous ne gérez pas les fenêtres de contexte dans un projet. Les threads se compactent automatiquement, et la conversation fonctionne à partir des messages récents, des threads récents et de la mémoire du projet plutôt que de son historique complet, elle continue donc aussi longtemps que le projet s'exécute. Mettez tout ce qui ne doit jamais être supprimé dans la [mémoire du projet](#give-a-project-standing-context). Si un thread dépasse sa fenêtre de contexte, il affiche [Claude a manqué de contexte à ce tour](#context-limit).

<h3 id="tune-how-claude-runs-a-project">
  Affiner la façon dont Claude exécute un projet
</h3>

Dites à Claude dans la conversation combien de threads exécuter à la fois, quand publier des mises à jour et quand ouvrir des pull requests. Si Claude coordonne d'une manière que vous ne voulez pas, dites-le. Par exemple, vous pouvez dire :

* « Proposer des threads et attendre mon approbation avant de les démarrer » ou « Démarrer ceux-ci maintenant sans me demander de confirmer »
* « Exécuter au maximum deux threads à la fois » ou « Réutiliser un thread existant pour les suites dans le même domaine »
* « Publier des mises à jour plus courtes » ou « Publier seulement quand quelque chose se termine ou est bloqué »
* « Donnez-moi une mise à jour d'état sur chaque thread »
* « Faire cette tâche avec un modèle plus petit »
* « N'ouvrez pas de pull request jusqu'à ce que j'aie vu le plan »
* « Dites-moi ce qui ne va pas dans ces référentiels et ne corrigez rien pour l'instant », quand vous voulez examiner les résultats avant qu'aucun d'eux ne devienne un thread
* « Répondre à cela ici au lieu de démarrer un thread », quand Claude démarre un thread pour quelque chose que vous aviez l'intention comme une question rapide

Claude enregistre les préférences comme celles-ci dans la [mémoire du projet](#give-a-project-standing-context) de lui-même et les suit dans les threads ultérieurs. Ce sont des instructions que Claude respecte, pas des paramètres appliqués, donc une limite de thread que vous donnez de cette façon n'est pas un plafond dur. Ajoutez-en une aux instructions du projet quand vous voulez qu'elle soit formulée exactement et appliquée à chaque thread dès le départ.

<h3 id="unblock-a-thread-waiting-on-approval">
  Débloquer un thread en attente d'approbation
</h3>

Les threads s'exécutent en [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) quand le modèle du thread le supporte, donc la plupart des appels d'outils s'exécutent sans vous demander. Quand un thread a besoin de votre approbation, l'invite est à l'intérieur de ce thread et le thread attend jusqu'à ce que vous y répondiez. Dire à Claude dans la conversation du projet d'aller de l'avant ne l'atteint pas.

Chaque approbation couvre cette invite, ou le reste de ce thread si vous choisissez l'option plus large. Pour laisser chaque thread exécuter certaines commandes sans demander, ou pour en bloquer certaines, ajoutez des [règles de permission](/docs/fr/permissions) au `.claude/settings.json` du référentiel. Les threads ne les appliquent que dans un projet avec un référentiel ; voir [Ce que les threads reprennent de vos référentiels](#what-threads-pick-up-from-your-repositories).

<h2 id="give-a-project-standing-context">
  Donner un contexte permanent à un projet
</h2>

La mémoire du projet, les instructions du projet et les référentiels, fichiers et environnement du projet portent le contexte sur les threads. Vous définissez chacun une fois.

| Contexte                                | Ce qu'il porte                                                                                                                                                                                                                                 | Comment vous le définissez                                                                                                                                                                                                            |
| :-------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Mémoire du projet                       | Notes que Claude garde sur le projet, comme les exigences, les décisions et les pièges, stockées sous forme de fichiers. Chaque thread cloud lit le fichier d'index `MEMORY.md` au démarrage et ouvre les autres fichiers quand il en a besoin | Demandez à Claude dans la conversation du projet ou n'importe quel thread cloud de mémoriser une exigence, une décision ou un piège, ou d'en oublier un. Lisez, modifiez et supprimez les fichiers dans **Project settings > Memory** |
| Instructions du projet                  | Texte envoyé à chaque nouveau thread et à Claude dans la conversation du projet, jusqu'à 16 000 caractères. [Écrire les instructions du projet](#write-project-instructions) couvre ce qu'il faut y mettre                                     | **Project settings > Memory > Project instructions**, ou demandez à Claude de changer les instructions                                                                                                                                |
| Référentiels, fichiers et environnement | Les référentiels que chaque thread cloud clone, les dossiers et fichiers qu'il peut lire sous `/mnt/project-files`, et l'environnement cloud dans lequel il s'exécute                                                                          | Référentiels et environnement dans **Project settings > Environment**, ou demandez à Claude dans la conversation d'ajouter un référentiel au projet. Fichiers et dossiers depuis **Add** sur l'onglet **Library** dans **Overview**   |

**Project settings > Memory** énumère ces fichiers sous **Auto memory**, car Claude les écrit lui-même en travaillant dans le projet. Ils sont distincts de la [mémoire automatique](/docs/fr/memory) que Claude Code garde sur votre machine, même si les deux utilisent un index `MEMORY.md`. La mémoire du projet est également distincte des fichiers `CLAUDE.md` dans les référentiels du projet. Chaque thread cloud lit toujours ces fichiers `CLAUDE.md` à partir de son clone au démarrage, donc mettez les instructions sur un référentiel dans son `CLAUDE.md` et les notes sur le projet dans la mémoire du projet.

<h3 id="write-project-instructions">
  Écrire les instructions du projet
</h3>

Les instructions du projet sont le brief que chaque nouveau thread commence. Cliquez sur l'icône d'engrenage dans l'en-tête du projet pour ouvrir **Project settings**, puis allez à **Memory > Project instructions**. Un brief utile couvre :

* À quoi sert le projet
* Où le travail se fait : quels référentiels, quelle branche commencer, comment nommer les pull requests
* Comment un thread vérifie son propre travail avant de l'appeler terminé
* Quoi faire quand quelque chose dont il a besoin manque
* Ce qui a besoin de votre approbation en premier

Par exemple :

```text theme={null}
Ce projet maintient la latence p95 de l'API des paiements sous 200 ms : profilage, corrections de requêtes et de cache, et les mises à jour de dépendances qui les accompagnent, dans le référentiel payments-api.

- Brancher à partir de main et ouvrir une pull request brouillon par thread.
- Avant d'appeler le travail terminé, exécutez `make test` et `make lint` et collez les lignes de résumé dans votre message final.
- Si vous ne pouvez pas atteindre quelque chose dont vous avez besoin, comme un référentiel, un secret, une API ou un connecteur, dites exactement ce qui manque dans votre premier message et arrêtez. Ne substituez pas, ne simulez pas et ne devinez pas.
- Ne fusionnez pas, ne forcez pas de push et ne changez pas la configuration CI sans me le demander dans le thread.
```

Les règles sur un référentiel, comme ses commandes de construction, appartiennent au `CLAUDE.md` de ce référentiel, que chaque thread cloud lit quand le référentiel fait partie du projet. Une fois que le travail est en cours, quand vous corrigez un thread, dites aussi à Claude de mémoriser la correction : elle va dans la [mémoire du projet](#give-a-project-standing-context) et les threads ultérieurs commencent avec elle.

<h3 id="decide-which-repositories-to-add">
  Décider quels référentiels ajouter
</h3>

Les référentiels que vous ajoutez à un projet viennent avec tout ce qu'ils contiennent, leur code, `CLAUDE.md` et skills, dans chaque thread cloud. Les référentiels que vous n'ajoutez pas sont toujours à portée : un thread cloud peut en ajouter un à lui-même quand sa tâche en a besoin. La plupart des projets utilisent les deux :

* **L'ajouter au projet**, dans la boîte de dialogue **New project**, dans **Project settings > Environment**, ou en demandant à Claude dans la conversation de l'ajouter au projet. Chaque thread cloud à partir de là clone et commence avec son `CLAUDE.md` et ses skills chargés, que la tâche le touche ou non. Passer d'un référentiel à plusieurs change également ce que les threads prennent du `.claude/settings.json` de chaque référentiel ; voir [Ce que les threads reprennent de vos référentiels](#what-threads-pick-up-from-your-repositories).
* **Le laisser de côté et laisser les threads l'ajouter quand nécessaire.** Un thread cloud dont la tâche a besoin d'un référentiel que le projet n'a pas peut l'ajouter à lui-même, et une note dans le thread dit qu'il a été ajouté à ce thread seulement. Le clone se produit à mi-chemin de la tâche, donc le `CLAUDE.md` et les skills de ce référentiel n'étaient pas là quand le thread a commencé. Le thread suivant commence sans lui. Un référentiel ajouté de cette façon a besoin des mêmes [prérequis](#check-the-prerequisites) qu'un référentiel de projet : l'application Claude GitHub installée dessus et l'accès en push de votre compte GitHub.

Un projet n'a pas besoin d'un référentiel du tout. Ses threads cloud peuvent toujours faire de la recherche, écrire des documents et écrire et exécuter du code dans leur propre sandbox, et ils livrent des fichiers à l'onglet **Library**. N'importe quel thread cloud peut aussi ajouter un référentiel à lui-même quand une tâche l'exige.

Une fois que le projet a des référentiels, Claude ne peut ajouter que des référentiels d'un propriétaire GitHub que le projet utilise déjà, qu'il en ajoute un au projet ou qu'un thread en ajoute un à lui-même. Pour apporter un référentiel d'un propriétaire différent, ajoutez-le au projet vous-même dans **Project settings > Environment**.

Pour un projet qui s'étend sur plusieurs référentiels, comme une fonctionnalité avec du code serveur, web, mobile et de bureau, ajoutez le ou les deux référentiels que presque chaque tâche touche et nommez les autres dans [les instructions du projet](#write-project-instructions) pour que Claude sache où le reste du code vit. Les threads cloud commencent alors petit et tirent les autres référentiels seulement pour les tâches qui en ont besoin.

<h3 id="what-threads-pick-up-from-your-repositories">
  Ce que les threads reprennent de vos référentiels
</h3>

Chaque thread cloud clone chaque référentiel du projet et charge `CLAUDE.md` et les skills de tous. Les règles de permission, les hooks et `env` viennent seulement du `.claude/settings.json` dans le répertoire où le thread commence : à l'intérieur du référentiel quand le projet en a un, et au-dessus des clones quand il en a plusieurs, où aucun fichier du référentiel n'est lu pour eux.

| Dans chaque référentiel                                                   | Un référentiel                                                                                                                            | Plusieurs référentiels                                                        |
| :------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------- |
| `CLAUDE.md`                                                               | Chargé au démarrage du thread                                                                                                             | Chargé à partir de chaque référentiel au démarrage du thread                  |
| Skills, agents et commandes sous `.claude/`                               | Chargés                                                                                                                                   | Chargés à partir de chaque référentiel                                        |
| Plugins activés dans `.claude/settings.json`                              | Non chargés. Ajoutez le plugin dans **Project settings > Plugins** à la place                                                             | Non chargés. Ajoutez le plugin dans **Project settings > Plugins** à la place |
| Règles de permission, hooks et `env` définis dans `.claude/settings.json` | S'appliquent au thread, sauf les clés `env` que [aucune session cloud n'honore](/docs/fr/cloud-environments#what-carries-over-from-your-setup) | Ne s'appliquent pas                                                           |

Dans un projet avec plusieurs référentiels, chaque clone est attaché au thread en tant que [répertoire supplémentaire](/docs/fr/memory#load-from-additional-directories) avec le chargement de `CLAUDE.md` activé, c'est pourquoi le `CLAUDE.md` et les skills de chaque référentiel se chargent au démarrage même si le thread commence au-dessus d'eux. Dans un tel projet, mettez les règles permanentes dans les instructions du projet et donnez aux threads les variables d'environnement via l'[environnement cloud](#choose-an-environment-for-threads).

<h3 id="choose-an-environment-for-threads">
  Choisir un environnement pour les threads
</h3>

Chaque nouveau thread cloud démarre dans l'[environnement cloud](/docs/fr/cloud-environments) du projet. L'environnement définit quels domaines les threads peuvent atteindre, quelles variables d'environnement ils ont, quels identifiants API sont ajoutés à leurs demandes et ce que le script de configuration installe avant que Claude commence. Les threads cloud utilisent un environnement par défaut hébergé par Anthropic jusqu'à ce que vous en choisissiez un dans **Project settings > Environment**.

Si les threads cloud ont besoin d'atteindre une API interne ou un registre de packages privé, ou ont besoin d'un token que votre machine détient normalement, changez l'environnement plutôt que le projet : voir [Accès réseau](/docs/fr/cloud-environments#network-access), [Ajouter des identifiants API](/docs/fr/cloud-environments#add-api-credentials) et [Scripts de configuration](/docs/fr/cloud-environments#setup-scripts).

<h3 id="get-skills-plugins-connectors-and-tools-into-threads">
  Obtenir des skills, des plugins, des connecteurs et des outils dans les threads
</h3>

Les threads cloud n'ont pas les skills, les serveurs MCP, les plugins et les outils installés seulement sur votre machine. Un thread que Claude exécute sur votre machine via [Remote Control](/docs/fr/remote-control) utilise ce qui y est installé. Pour rendre chacun de ceux-ci disponible aux threads cloud :

* Skills, subagents et commandes : validez-les dans un référentiel que vous avez ajouté au projet, par exemple un skill à `.claude/skills/<skill-name>/SKILL.md`. Chaque thread cloud clone chaque référentiel du projet et charge `.claude/skills/`, `.claude/agents/` et `.claude/commands/` à partir de chacun d'eux, donc un skill validé dans un référentiel est disponible dans chaque thread cloud. Les threads cloud chargent également les skills que vous activez pour votre compte claude.ai.
* Plugins : ajoutez-les dans **Project settings > Plugins** ; ils se chargent dans chaque nouveau thread cloud. Les plugins qu'un référentiel déclare dans son `.claude/settings.json` [ne se chargent pas dans les threads cloud](/docs/fr/cloud-environments#what-carries-over-from-your-setup).
* Serveurs MCP : les threads cloud obtiennent leurs outils MCP à partir des connecteurs sur votre compte claude.ai, qui sont des serveurs MCP que vous connectez une fois à [claude.ai/customize/connectors](https://claude.ai/customize/connectors) ou via le lien **Manage connectors** dans **Project settings > Environment**. Chaque thread cloud peut tous les utiliser sans configuration par projet. La conversation du projet elle-même n'a pas de connecteurs, donc envoyez le travail qui en a besoin en tant que tâche pour un thread cloud. Dans un projet avec un référentiel, les threads cloud chargent également les serveurs MCP à partir du [`.mcp.json`](/docs/fr/cloud-environments#what-carries-over-from-your-setup) de ce référentiel. [Comment les connecteurs atteignent Claude Code](/docs/fr/mcp#how-connectors-reach-claude-code) énumère les règles pour les sessions cloud et les paramètres qui désactivent les connecteurs.
* Outils en ligne de commande et packages : installez-les dans le [script de configuration](/docs/fr/cloud-environments#setup-scripts) de l'environnement.

Pour voir quels connecteurs un thread cloud en cours d'exécution a à claude.ai/code, ouvrez le thread et sélectionnez **Connectors** dans le menu **+** à côté de sa boîte de message. Désactiver un connecteur là le supprime de ce thread et enregistre cela comme votre paramètre par défaut du compte, donc les nouveaux threads et les chats claude.ai commencent sans lui jusqu'à ce que vous le réactiviez. Un thread cloud reprend un connecteur que vous ajoutez ou reconnectez après le prochain message que vous lui envoyez.

<h2 id="project-settings-reference">
  Référence des paramètres du projet
</h2>

Vous changez les paramètres du projet à claude.ai/code ou dans l'application de bureau, pas dans `settings.json`. Ouvrez **Project settings** depuis **Settings** dans le menu de la barre latérale du projet ou depuis l'icône d'engrenage dans l'en-tête du projet.

Les paramètres s'enregistrent au fur et à mesure que vous les modifiez ; un champ de texte que vous modifiez, comme l'objectif ou les instructions, affiche **Save changes** et **Discard** jusqu'à ce que vous le quittiez. Les modifications apportées aux instructions, référentiels, plugins et environnement dans **Project settings** atteignent les nouveaux threads, pas les threads déjà en cours d'exécution.

| Paramètre                        | Section     | Ce qu'il contrôle                                                                                                           |
| :------------------------------- | :---------- | :-------------------------------------------------------------------------------------------------------------------------- |
| Nom, icône et objectif           | General     | Le nom et l'icône du projet dans la barre latérale, et son objectif d'une ligne                                             |
| Modèle et effort du coordinateur | General     | Le modèle et le [niveau d'effort](/docs/fr/model-config#adjust-effort-level) pour Claude dans la conversation du projet          |
| Modèle et effort du thread       | General     | Le modèle et le niveau d'effort pour les threads                                                                            |
| Instructions du projet           | Memory      | [Règles permanentes](#give-a-project-standing-context) que chaque nouveau thread reçoit                                     |
| Référentiels du projet           | Environment | Les référentiels que les nouveaux threads clonent                                                                           |
| Environnement cloud              | Environment | L'[environnement cloud](#choose-an-environment-for-threads) dans lequel les nouveaux threads s'exécutent                    |
| Connecteurs                      | Environment | Un lien pour gérer les connecteurs claude.ai que les threads obtiennent                                                     |
| Plugins                          | Plugins     | Les plugins qui se chargent dans chaque nouveau thread                                                                      |
| Usage                            | Usage       | [Utilisation des tokens](#usage-and-cost) par thread et par modèle                                                          |
| Memory                           | Memory      | Les [fichiers de mémoire](#give-a-project-standing-context) du projet                                                       |
| Restart Claude                   | General     | Redémarre la conversation du projet quand [Claude cesse de répondre là](#claude-hasnt-responded)                            |
| Pause, Archive, Delete           | General     | Arrête, masque ou supprime le projet ; voir [Pause, archive ou suppression d'un projet](#pause-archive-or-delete-a-project) |

<h3 id="pause-archive-or-delete-a-project">
  Pause, archive ou suppression d'un projet
</h3>

Les trois contrôles sont en bas de **Project settings > General** :

* **Pause** : arrête tout à la fois. Chaque thread en cours d'exécution et la conversation sont interrompus, aucun nouveau thread ne démarre, les routines ne s'exécutent pas, et le projet n'accepte pas de messages jusqu'à ce que vous le repreniez. Cliquez sur **Resume** au même endroit ou sur la bannière au-dessus de la boîte de message du projet ; un thread en pause continue quand vous lui envoyez un message après cela.
* **Archive** : masque le projet de la barre latérale et archive ses threads, ce qui arrête tout thread qui était en cours d'exécution ou surveillait une pull request. Les routines du projet ne s'exécutent pas pendant qu'il est archivé. Pour ramener le projet, ouvrez-le à partir de la page Projects et cliquez sur **Unarchive**. Ses threads restent archivés jusqu'à ce que vous les désarchiviez individuellement à partir de la liste des sessions.
* **Delete** : supprime définitivement le projet ainsi que ses threads, sa mémoire et ses fichiers, et désactive les routines du projet. Cela ne peut pas être annulé. Les branches et pull requests que les threads ont poussées vers GitHub ne sont pas affectées.

<h2 id="usage-and-cost">
  Utilisation et coût
</h2>

L'utilisation du projet compte par rapport aux mêmes [limites de plan](/docs/fr/errors#youve-hit-your-session-limit) que vos autres sessions Claude Code, et un projet ne peut pas dépenser au-delà de ces limites de lui-même.

Un thread qui atteint la limite de votre plan attend et continue de lui-même quand la limite se réinitialise, donc le travail que vous avez laissé en cours d'exécution commence à utiliser votre prochaine fenêtre d'utilisation sans message de vous. [Un thread a atteint la limite d'utilisation](#usage-limit-reached) couvre ce que vous voyez, comment l'arrêter et le seul cas qui n'attend pas.

Le travail dépasse vos limites de plan seulement si vous avez activé les [crédits d'utilisation](/docs/fr/costs#add-usage-credits-to-your-subscription) pour votre compte. Un thread ne peut pas les activer pour vous.

<h3 id="what-draws-on-your-plan">
  Ce qui utilise votre plan
</h3>

Un projet utilise vos limites plus rapidement qu'une seule session, et sur un plan Pro en particulier, vous devriez vous attendre à atteindre votre limite plus tôt les jours où vous en exécutez un. Ce sont les parties d'un projet qui utilisent votre plan :

* **Threads en cours d'exécution** : chacun est une session complète, et plusieurs peuvent s'exécuter à la fois. Il n'y a pas de nombre fixe ; Claude démarre autant que le travail l'exige, et une limite que vous [demandez](#tune-how-claude-runs-a-project) est une préférence plutôt qu'un plafond. La limite appliquée est 200 nouveaux threads par jour sur vos projets.
* **La conversation** : Claude utilise des tokens en lisant ce que les threads rapportent et en décidant quoi faire ensuite.
* **Threads surveillant une pull request** : un thread inactif se réveille et utilise votre plan à nouveau quand CI échoue ou un commentaire d'examen arrive sur sa pull request. Pour arrêter cela, demandez dans le thread de cesser de surveiller la pull request.

Un projet sans threads en cours d'exécution, sans pull requests surveillées et sans nouveaux messages n'utilise pas votre plan pendant qu'il reste inactif, et un projet archivé non plus.

<h3 id="see-and-reduce-a-project’s-usage">
  Voir et réduire l'utilisation d'un projet
</h3>

Ouvrez l'onglet **Usage** dans **Project settings** pour voir l'utilisation des tokens par thread et par modèle, et combien est allé à la conversation du projet. Pour la réduire :

* Une suite acheminée vers un thread qui a été inactif plus longtemps que la [durée de vie du cache](/docs/fr/prompt-caching#cache-lifetime), une heure sur Pro et Max dans les limites de votre plan, relit la conversation entière de ce thread avant de faire quoi que ce soit. Pour le nouveau travail, demander à Claude de démarrer un thread frais peut utiliser moins que de relancer un grand ancien.
* Pour le travail qui n'a pas besoin du plus grand modèle, [choisissez un modèle plus petit ou un niveau d'effort inférieur](#choose-models-and-let-claude-manage-context) pour les threads, la conversation ou les deux.
* Demandez à Claude dans la conversation du projet d'exécuter moins de threads à la fois, ou de répondre aux petites questions lui-même au lieu de démarrer un thread.

<h2 id="how-projects-relate-to-other-claude-code-features">
  Comment les projets se rapportent à d'autres fonctionnalités de Claude Code
</h2>

Plusieurs fonctionnalités de Claude Code permettent à plus d'une session de fonctionner en même temps, donc exécuter du travail en parallèle n'est pas en soi à quoi sert un projet. Dans un projet, Claude démarre et suit les sessions au lieu de vous, et chacun commence à partir des mêmes instructions. Voici comment chaque fonctionnalité voisine se connecte à un projet :

* **Claude Tag** : [Claude Tag](https://claude.com/docs/claude-tag/overview) est Claude dans les canaux Slack de votre équipe, sur les plans Team et Enterprise. N'importe qui dans un canal peut lui donner du travail, tout le monde dans le canal le voit et le dirige, et il utilise les connexions qu'un administrateur a configurées pour ce canal. Un projet est le vôtre seul : vous êtes le seul à lui envoyer du travail ou à voir ses threads, il utilise votre propre accès GitHub et vos connecteurs, et il est sur Pro et Max. [Comment Claude Tag diffère de Cowork et Claude Code](https://claude.com/docs/claude-tag/concepts/how-it-works#how-claude-tag-differs-from-cowork-and-claude-code) a le côté à côté.
* **Sessions cloud** : chaque thread est une [session cloud](/docs/fr/claude-code-on-the-web) sauf si vous demandez à Claude de l'exécuter sur votre machine. De toute façon, Claude la démarre et la suit au lieu de vous. Une session cloud que vous avez démarrée vous-même peut devenir un projet ou en alimenter un via [**Continue as a project** ou **Move to project**](#start-from-an-existing-cloud-session).
* **Routines** : quand vous demandez du travail programmé dans un projet, Claude crée une [routine](/docs/fr/routines) qui s'exécute en tant que threads dans ce projet et apparaît sur son onglet **Routines**. Les routines que vous créez en dehors d'un projet continuent à fonctionner seules.
* **Sessions locales et agent view** : une session que vous démarrez vous-même dans votre terminal, IDE ou l'environnement local de l'application de bureau ne peut pas être ajoutée à un projet. Un projet atteint votre machine uniquement en exécutant un thread là-bas via [Remote Control](/docs/fr/remote-control). [Agent view](/docs/fr/agent-view) est un écran pour suivre plusieurs sessions locales que vous avez démarrées vous-même ; il n'a pas de coordinateur.
* **Worktrees** : un [worktree](/docs/fr/worktrees) donne à chaque session locale sa propre copie de travail d'un référentiel pour que les sessions parallèles sur votre machine ne s'écrasent pas mutuellement. Les threads cloud n'en ont pas besoin : chacun clone ses référentiels dans son propre sandbox cloud et travaille sur sa propre branche.
* **Équipes d'agents** : une [équipe d'agents](/docs/fr/agent-teams) est une session qui démarre des sessions de coéquipiers pour une seule tâche, sur votre machine ou à l'intérieur d'une session cloud, et se termine avec cette tâche.
* **Projects dans le chat claude.ai et Cowork** : l'[expérience Projects antérieure](https://support.claude.com/en/articles/9517075-what-are-projects), qui groupe les conversations et les fichiers de référence sans threads ni coordinateur. Ces projets continuent à fonctionner comme ils le font aujourd'hui jusqu'à ce que l'expérience repensée les atteigne.

[Exécuter des agents en parallèle](/docs/fr/agents) compare ces options côte à côte.

<h2 id="limitations">
  Limitations
</h2>

* Les projets sont disponibles à claude.ai/code, dans l'application de bureau et dans l'application mobile Claude, pas dans le CLI du terminal ou via Amazon Bedrock, la plateforme d'agents de Google Cloud ou Microsoft Foundry. La commande [`claude project`](/docs/fr/cli-reference) du CLI, qui gère l'état local de Claude Code pour un répertoire, n'est pas liée.
* Les threads du projet sont des [sessions cloud](/docs/fr/claude-code-on-the-web), ou des sessions sur votre propre machine via [Remote Control](/docs/fr/remote-control), avec Anthropic comme fournisseur de modèle dans les deux cas. [Sécurité](/docs/fr/security) et [Utilisation des données](/docs/fr/data-usage) couvrent comment les sessions cloud sont isolées et ce qui est conservé, et [Connexion et sécurité](/docs/fr/remote-control#connection-and-security) couvre comment un thread sur votre machine se connecte et ce qui est stocké.
* Vous ne pouvez pas ajouter une session que vous avez démarrée vous-même sur votre machine à un projet. Pour permettre à un projet d'exécuter un thread sur votre machine, connectez le dossier dans lequel il doit fonctionner via [Remote Control](/docs/fr/remote-control#requirements) : activez Remote Control sous **Paramètres > Claude Code** dans l'application de bureau Claude, ou exécutez `claude remote-control` dans le dossier et laissez-le s'exécuter. Cette machine doit avoir Claude Code v2.1.280 ou version ultérieure. Un projet ne peut pas non plus exécuter un thread sur votre machine tandis que **Require trusted devices** est activé dans vos paramètres claude.ai.
* Le sandbox d'un thread cloud se met en pause entre les tours et reprend quand le thread continue. Si le sandbox ne peut pas être repris, le thread continue à partir d'un clone frais, donc les modifications non validées peuvent être perdues. Sur les tâches longues, demandez à Claude de valider et de pousser le travail en cours.
* Un projet appartient à un utilisateur. Vous ne pouvez pas partager un projet ou ses threads avec un autre utilisateur, et les transcriptions de threads n'ont pas l'option de partage que les autres sessions cloud ont. Il n'y a pas de contrôles au niveau de l'organisation pour les projets pendant la bêta.
* Un thread appartient au seul projet qui l'a démarré. Vous ne pouvez pas déplacer ou copier un thread vers un autre projet, ou le déplacer pour qu'il soit seul. [**Move to project**](#start-from-an-existing-cloud-session) va seulement dans l'autre sens : il apporte le travail d'une session cloud dans un projet.

<h2 id="troubleshooting">
  Dépannage
</h2>

Pour les invites de configuration GitHub dans la boîte de dialogue **New project**, voir [Configurer l'accès GitHub](#set-up-github-access).

<h3 id="a-thread-looks-stuck">
  Un thread semble bloqué
</h3>

Claude ne publie pas chaque étape qu'un thread prend, donc un thread qui s'affiche comme en cours d'exécution sans nouveaux messages dans la conversation du projet fonctionne généralement toujours. Un nouveau thread cloud provisionne également son [environnement cloud](/docs/fr/cloud-environments) avant que Claude commence, donc sa première mise à jour prend un moment. Ouvrez le thread pour lire sa transcription. Si le thread attend une invite de permission, répondez-y là.

<h3 id="threads-guessed-or-stalled-instead-of-asking">
  Les threads ont deviné ou se sont arrêtés au lieu de demander
</h3>

Quand plusieurs threads reviennent ayant supposé quelque chose de mal, contourné l'accès manquant ou arrêté avec « bloqué », la cause est généralement le même écart dans la configuration du projet plutôt qu'un problème avec chaque tâche. Triez les threads qui sont solides avant de corriger quoi que ce soit :

1. Demandez à Claude dans la conversation : « Pour chaque thread ouvert, énumérez ce que vous lui avez demandé de faire, ce qu'il a supposé ou ne pouvait pas atteindre, et sur quoi il attend. » Claude lit chaque thread et répond dans la conversation.
2. Pour les threads qui ont commencé à partir d'une mauvaise hypothèse, ouvrez le thread depuis **Overview** et marquez-le résolu depuis son menu, ou dites-lui quoi faire à la place dans sa boîte de message. Sa branche et toute pull request restent sur GitHub jusqu'à ce que vous les supprimiez.
3. Corrigez l'écart une seule fois, dans [les instructions du projet](#give-a-project-standing-context) ou l'[environnement](#choose-an-environment-for-threads), puis envoyez un thread avant d'envoyer le reste du travail à nouveau en tant que nouveaux threads.

<h3 id="claude-hasnt-responded">
  Claude n'a pas répondu
</h3>

La conversation du projet affiche une bannière « Claude hasn't responded » quand Claude fonctionne mais ses réponses n'atteignent pas le projet. Cliquez sur **Restart Claude** sur la bannière, ou allez à **Project settings > General** et cliquez sur **Restart** dans la ligne **Restart Claude**. Claude se reconnecte à la conversation ; toute réponse qu'il était en train d'écrire est perdue, et les threads ne sont pas affectés.

<h3 id="repository-access-errors">
  Erreurs d'accès au référentiel
</h3>

Trois messages signifient qu'un thread ou le projet ne peut pas atteindre l'un de ses référentiels. Un thread cloud du projet a besoin des [prérequis GitHub](#check-the-prerequisites) même quand vos autres sessions cloud clonent le même référentiel sans problème.

* **« Couldn't start the session — Claude doesn't have GitHub access to this project's repository »**, rapporté avant le démarrage du thread, quand l'application Claude GitHub n'est pas installée sur ce référentiel, est suspendue ou n'est pas liée au compte GitHub que vous avez connecté.
* **« Unable to access your repository »**, rapporté par un thread quand son clone échoue : GitHub a rejeté le clone, le référentiel n'a pas été trouvé sous le nom que le projet a, ou la branche à partir de laquelle le thread a été demandé de commencer n'existe pas.
* **« Claude can't access »** un référentiel, affiché quand vous enregistrez les référentiels dans la boîte de dialogue **New project** ou **Project settings**. Le message continue avec un lien d'installation et un lien de reconnexion. Utilisez le lien d'installation si l'application Claude GitHub n'est pas sur ce référentiel, et le lien de reconnexion si elle l'est, puisque l'application GitHub peut être installée sur GitHub sans être liée au compte que vous avez connecté à Claude. Si le message dit que l'application GitHub est suspendue ou n'inclut pas ce référentiel, suivez son lien vers GitHub pour corriger cela.

Pour corriger l'un d'eux, cliquez sur le bouton que le message offre, comme **Install GitHub App** ou **Select repositories on GitHub**, puis **Check again**. Quand le bloc est du côté de l'organisation GitHub, comme un propriétaire qui n'a pas approuvé l'application ou une liste d'autorisation IP qui exclut Claude, le message affiche un lien **See how to fix** à la place. S'il n'y a pas de bouton, suivez [Configurer l'accès GitHub](#set-up-github-access), puis envoyez un autre message pour réessayer.

<h3 id="usage-limit-reached">
  Un thread a atteint la limite d'utilisation
</h3>

Quand un thread ou la conversation du projet atteint la limite de cinq heures ou hebdomadaire de votre plan, il continue à réessayer de lui-même et continue quand la limite se réinitialise. Pendant qu'il attend, le thread affiche **Service is busy** avec « Claude is still retrying and will continue automatically. » Vous n'avez rien à faire pour que le travail continue. Si vous préférez qu'il n'utilise pas votre prochaine fenêtre d'utilisation, cliquez sur **Stop** dans le thread, ou [mettez le projet en pause](#pause-archive-or-delete-a-project) pour tenir chaque thread. Un thread qu'une routine a démarré n'attend pas : son tour s'arrête avec une erreur de limite, et vous lui envoyez un message après que la limite se réinitialise.

[Erreurs de limite d'utilisation](/docs/fr/errors#youve-hit-your-session-limit) expliquent les limites et quand elles se réinitialisent.

<h3 id="additional-usage-credits-are-required">
  Des crédits d'utilisation supplémentaires sont requis
</h3>

Un thread ou la conversation du projet a fait une demande que votre plan ne couvre que avec des crédits d'utilisation, comme une à un modèle ou une taille de contexte que votre plan n'inclut pas, et les crédits d'utilisation ne sont pas activés pour votre compte. [Ajouter des crédits d'utilisation à votre abonnement](/docs/fr/costs#add-usage-credits-to-your-subscription) couvre qui peut les activer ou les acheter sur chaque plan. Une fois que les crédits sont disponibles, envoyez un autre message pour réessayer.

<h3 id="context-limit">
  Autres messages
</h3>

Ces messages nomment leur propre cause. Le tableau donne la prochaine étape pour chacun.

| Message                                                                                            | Quoi faire                                                                                                                                                                                                                                         |
| :------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| « Unable to connect to repository » avec « Claude couldn't reach GitHub to fetch your repository » | Attendez un moment, puis envoyez un autre message pour réessayer                                                                                                                                                                                   |
| « Unable to connect to repository » avec « Claude couldn't access your repository or environment » | Votre compte GitHub a besoin d'accès en push au référentiel, et l'environnement doit toujours exister. Vérifiez les deux dans **Project settings > Environment**, puis réessayez                                                                   |
| « Couldn't show the setup proposal »                                                               | L'application que vous avez ouverte est plus ancienne que les **Setup recommendations** que Claude a envoyées. Actualisez la page ou redémarrez l'application de bureau, ou demandez à Claude de proposer à nouveau la configuration               |
| « The project's environment was removed »                                                          | Choisissez un environnement différent dans **Project settings > Environment** ; le changement s'applique aux nouveaux threads                                                                                                                      |
| « Setup script failed »                                                                            | Cliquez sur **Edit setup script** sur l'erreur, corrigez le script dans l'environnement, puis envoyez un autre message. [Setup script failed](/docs/fr/web-quickstart#setup-script-failed) énumère les causes courantes                                 |
| « Claude ran out of context on this turn »                                                         | Le thread a rempli sa fenêtre de contexte. Si le message dit que le thread continue dans une session frais, il continue de lui-même ; sinon demandez à Claude dans la conversation du projet de démarrer un nouveau thread pour le travail restant |
| « Reached the turn limit »                                                                         | Le thread a atteint le plafond sur les étapes pour un message défini par [`CLAUDE_CODE_MAX_TURNS`](/docs/fr/env-vars). Envoyez un autre message pour continuer, ou augmentez ou supprimez cette variable où elle est définie                            |

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Utiliser Claude Code dans le cloud](/docs/fr/claude-code-on-the-web) : comment fonctionnent les sessions cloud derrière chaque thread, y compris les options d'accès GitHub et l'auto-fix sur les pull requests
* [Configurer les environnements cloud](/docs/fr/cloud-environments) : changez ce que les threads peuvent atteindre sur le réseau, donnez-leur des variables d'environnement et des identifiants API, et installez des outils avec un script de configuration
* [Automatiser le travail avec les routines](/docs/fr/routines) : calendriers, déclencheurs et gestion des routines, y compris celles que Claude crée à partir d'un projet
* [Gérer plusieurs agents avec agent view](/docs/fr/agent-view) : exécutez et suivez plusieurs sessions sur votre propre machine quand le travail a besoin d'outils ou de services que seule votre machine peut atteindre
* [Projects redesigned: from folder to conversation](https://claude.com/blog/projects-redesigned) : l'annonce du lancement, avec la réflexion derrière la transformation d'un projet en conversation avec Claude
