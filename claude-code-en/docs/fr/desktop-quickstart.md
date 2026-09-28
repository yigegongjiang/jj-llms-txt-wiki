> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Démarrer avec l'application de bureau

> Installez Claude Code sur le bureau et commencez votre première session de codage

L'application de bureau vous donne accès à Claude Code avec une interface graphique conçue pour exécuter plusieurs sessions côte à côte : une barre latérale pour gérer les travaux parallèles, une disposition glisser-déposer avec un terminal intégré et un éditeur de fichiers, un examen des différences visuelles, un aperçu en direct de l'application, la surveillance des PR GitHub avec fusion automatique, et les tâches planifiées. Aucun terminal requis.

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

<Note>
  Claude Code nécessite un [abonnement Pro, Max, Team ou Enterprise](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=desktop_quickstart_pricing).
</Note>

Cette page vous guide dans l'installation de l'application et le démarrage de votre première session. Si vous êtes déjà configuré, consultez [Utiliser Claude Code Desktop](/docs/fr/desktop) pour la référence complète.

L'application de bureau a trois onglets :

* **Chat** : Conversation générale sans accès aux fichiers, similaire à claude.ai.
* **Cowork** : Un agent autonome en arrière-plan qui travaille sur des tâches dans une machine virtuelle en sandbox avec son propre environnement, fonctionnant indépendamment pendant que vous faites autre chose. Les sessions Cowork sur l'appareil exécutent la VM sur votre ordinateur ; les sessions Cowork distantes s'exécutent sur une VM gérée par Anthropic à la place.
* **Code** : Un assistant de codage interactif avec accès direct à vos fichiers locaux. Selon le mode de permission, vous approuvez chaque modification au fur et à mesure que Claude la propose ou vous examinez les modifications après que Claude les ait apportées.

Chat et Cowork sont couverts dans le [Centre d'aide Claude](https://support.claude.com/) ; l'installation et le déploiement de l'application de bureau sont couverts dans les [articles d'assistance Claude Desktop](https://support.claude.com/en/collections/16163169-claude-desktop). Cette page se concentre sur l'onglet **Code**.

<h2 id="install">
  Installer
</h2>

<Steps>
  <Step title="Installer et se connecter">
    Sur macOS et Windows, téléchargez le programme d'installation à partir des liens ci-dessus et exécutez-le. Sur Linux, suivez les étapes d'installation dans [Claude Desktop sur Linux](/docs/fr/desktop-linux). Lancez Claude à partir de votre dossier Applications sur macOS, du menu Démarrer sur Windows, ou de votre lanceur d'applications sur Linux, puis connectez-vous avec votre compte Anthropic.
  </Step>

  <Step title="Ouvrir l'onglet Code">
    Cliquez sur l'onglet **Code** en haut au centre. Si cliquer sur Code vous invite à mettre à niveau, vous devez d'abord [vous abonner à un plan payant](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=desktop_quickstart_upgrade). S'il vous invite à vous connecter en ligne, complétez la connexion et redémarrez l'application. Si vous voyez une erreur 403, consultez [dépannage de l'authentification](/docs/fr/desktop#403-or-authentication-errors-in-the-code-tab).
  </Step>
</Steps>

L'application de bureau inclut Claude Code. Vous n'avez pas besoin d'installer Node.js ou la CLI séparément. Pour utiliser `claude` depuis le terminal, installez la CLI séparément. Consultez [Démarrer avec la CLI](/docs/fr/quickstart).

<h2 id="start-your-first-session">
  Démarrez votre première session
</h2>

Avec l'onglet Code ouvert, choisissez un projet et donnez quelque chose à faire à Claude.

<Steps>
  <Step title="Choisissez un environnement et un dossier">
    Sélectionnez **Local** pour exécuter Claude sur votre machine en utilisant directement vos fichiers. Cliquez sur **Select folder** et choisissez votre répertoire de projet.

    <Tip>
      Commencez par un petit projet que vous connaissez bien. C'est le moyen le plus rapide de voir ce que Claude Code peut faire.
    </Tip>

    Vous pouvez également sélectionner :

    * **Cloud** : Exécutez des sessions dans le cloud qui continuent même si vous fermez l'application. Consultez [Utiliser Claude Code dans le cloud](/docs/fr/claude-code-on-the-web) pour savoir comment fonctionnent les sessions cloud.
    * **SSH** : Connectez-vous à une machine distante via SSH, comme vos propres serveurs, des machines virtuelles cloud ou des conteneurs de développement. Desktop installe Claude Code sur la machine distante automatiquement la première fois que vous vous connectez.
    * **WSL** (Windows) : Exécutez la session à l'intérieur d'une [distribution WSL 2](/docs/fr/desktop-wsl) ; Claude Code, les outils et git s'exécutent du côté Linux avec des chemins natifs.
  </Step>

  <Step title="Choisissez un modèle">
    Sélectionnez un modèle dans la liste déroulante à côté du bouton d'envoi. Consultez [models](/docs/fr/model-config#available-models) pour une comparaison des modèles disponibles. Vous pouvez modifier le modèle ultérieurement à partir de la même liste déroulante.
  </Step>

  <Step title="Dites à Claude ce qu'il faut faire">
    Tapez ce que vous voulez que Claude fasse :

    * `Find a TODO comment and fix it`
    * `Add tests for the main function`
    * `Create a CLAUDE.md with instructions for this codebase`

    Une [session](/docs/fr/desktop#work-in-parallel-with-sessions) est une conversation avec Claude à propos de votre code. Chaque session suit son propre contexte et ses modifications.
  </Step>

  <Step title="Examinez et acceptez les modifications">
    Ce qui se passe ensuite dépend du [mode de permission](/docs/fr/desktop#choose-a-permission-mode) affiché dans le sélecteur à côté du bouton d'envoi :

    * **Auto ou Accept edits** : Claude applique ses modifications de fichier, et un indicateur tel que `+12 -1` apparaît pour que vous puissiez les examiner dans la vue diff
    * **Manual** : Claude propose chaque modification et attend votre approbation avant de l'appliquer. Vos fichiers ne sont pas modifiés tant que vous n'acceptez pas, et si vous rejetez une modification, Claude vous demande comment vous aimeriez procéder à la place

    En mode Manual, vous verrez :

    1. Une [vue diff](/docs/fr/desktop#review-changes-with-diff-view) montrant exactement ce qui changera dans chaque fichier
    2. Des boutons Accept/Reject pour approuver ou refuser chaque modification
    3. Des mises à jour en temps réel au fur et à mesure que Claude traite votre demande
  </Step>
</Steps>

<h2 id="now-what">
  Et maintenant ?
</h2>

Vous avez effectué votre première modification. Pour la référence complète sur tout ce que Desktop peut faire, consultez [Utiliser Claude Code Desktop](/docs/fr/desktop). Voici quelques éléments à essayer ensuite.

**Interrompre et rediriger.** Vous pouvez rediriger Claude à tout moment. Cliquez sur le bouton d'arrêt pour interrompre immédiatement, ou tapez une correction et appuyez sur **Entrée** pour l'envoyer sans arrêter l'action en cours. De toute façon, vous n'avez pas besoin d'attendre qu'elle se termine ou de recommencer.

**Donner plus de contexte à Claude.** Tapez `@filename` dans la zone de saisie pour extraire un fichier spécifique dans la conversation, joignez des images et des PDF à l'aide du bouton de pièce jointe, ou glissez-déposez des fichiers directement dans l'invite. Plus Claude a de contexte, meilleurs sont les résultats. Consultez [Ajouter des fichiers et du contexte](/docs/fr/desktop#add-files-and-context-to-prompts).

**Utiliser les skills pour les tâches répétables.** Tapez `/` ou cliquez sur **+** → **Slash commands** pour parcourir les [commandes intégrées](/docs/fr/commands), les [skills personnalisés](/docs/fr/skills) et les skills de plugin. Les skills sont des invites réutilisables que vous pouvez invoquer chaque fois que vous en avez besoin, comme des listes de contrôle d'examen de code ou des étapes de déploiement.

**Examiner les modifications avant de valider.** Après que Claude modifie les fichiers, un indicateur `+12 -1` apparaît. Cliquez dessus pour ouvrir la [vue diff](/docs/fr/desktop#review-changes-with-diff-view), examinez les modifications fichier par fichier et commentez des lignes spécifiques. Claude lit vos commentaires et révise. Cliquez sur **Review code** pour que Claude évalue lui-même les diffs et laisse des suggestions en ligne.

**Ajuster le contrôle que vous avez.** Votre [mode de permission](/docs/fr/desktop#choose-a-permission-mode) définit le degré de liberté de Claude sans demander votre approbation :

* **Auto** : un classificateur examine les actions en arrière-plan et bloque les actions risquées au lieu de vous les demander.
* **Manual** : Claude demande avant de modifier les fichiers ou d'exécuter des commandes.
* **Accept edits** : Claude accepte automatiquement les modifications de fichiers pour une itération plus rapide.
* **Plan** : Claude propose une approche sans modifier aucun fichier, ce qui est utile avant une refonte majeure.

**Ajouter des plugins pour plus de capacités.** Cliquez sur le bouton **+** à côté de la zone de saisie et sélectionnez **Plugins** pour parcourir et installer des [plugins](/docs/fr/desktop#install-plugins) qui ajoutent des skills, des agents, des serveurs MCP et bien plus.

**Organiser votre espace de travail.** Glissez les volets de chat, diff, terminal, fichier et navigateur dans la disposition que vous souhaitez. Ouvrez le terminal avec **Ctrl+\`** pour exécuter des commandes aux côtés de votre session, ou cliquez sur un chemin de fichier pour l'ouvrir dans le volet de fichier. Consultez [Organiser votre espace de travail](/docs/fr/desktop#arrange-your-workspace).

**Prévisualiser votre application.** Lorsque vous exécutez votre serveur de développement sur le bureau, votre application s'ouvre dans le volet Navigateur, qui peut également [ouvrir des sites externes](/docs/fr/desktop#browse-external-sites). Claude peut afficher l'application en cours d'exécution, tester les points de terminaison, inspecter les journaux et itérer sur ce qu'il voit. Consultez [Prévisualiser votre application](/docs/fr/desktop#preview-your-app).

**Suivre votre demande de tirage.** Après l'ouverture d'une PR, Claude Code surveille les résultats des vérifications CI et peut corriger automatiquement les défaillances ou fusionner la PR une fois que toutes les vérifications sont réussies. Consultez [Surveiller l'état de la demande de tirage](/docs/fr/desktop#monitor-pull-request-status).

**Mettre Claude sur un calendrier.** Configurez des [tâches planifiées](/docs/fr/desktop-scheduled-tasks) pour exécuter Claude automatiquement de manière récurrente : un examen de code quotidien chaque matin, un audit de dépendances hebdomadaire, ou un briefing qui extrait de vos outils connectés.

**Augmenter l'échelle quand vous êtes prêt.** Ouvrez des [sessions parallèles](/docs/fr/desktop#work-in-parallel-with-sessions) à partir de la barre latérale pour travailler sur plusieurs tâches à la fois, chacune dans son propre Git worktree, et ouvrez le [volet des tâches](/docs/fr/desktop#watch-background-tasks) pour surveiller les sous-agents et les commandes en arrière-plan qu'une session exécute. Ouvrez un [side chat](/docs/fr/desktop#ask-a-side-question-without-derailing-the-session) pour poser une question sans dérailler le fil principal. Envoyez des [travaux de longue durée vers le cloud](/docs/fr/desktop#run-long-running-tasks-in-the-cloud) pour qu'ils continuent même si vous fermez l'application, ou [continuez une session sur le web ou dans votre IDE](/docs/fr/desktop#continue-in-another-surface) si une tâche prend plus de temps que prévu. [Connectez des outils externes](/docs/fr/desktop#extend-claude-code) comme GitHub, Slack et Linear pour réunir votre flux de travail.

<h2 id="what’s-next">
  Étapes suivantes
</h2>

* [Utiliser Claude Code Desktop](/docs/fr/desktop) : modes de permission, sessions parallèles, vue de diff, connecteurs et configuration d'entreprise
* [Vous venez de la CLI ?](/docs/fr/desktop#coming-from-the-cli) : exécutez Desktop et la CLI sur le même projet, et comparez les fonctionnalités, les équivalents de drapeaux et ce qui n'est pas disponible dans Desktop
* [Dépannage](/docs/fr/desktop#troubleshooting) : solutions aux erreurs courantes et aux problèmes de configuration
* [Bonnes pratiques](/docs/fr/best-practices) : conseils pour rédiger des invites efficaces et tirer le meilleur parti de Claude Code
* [Flux de travail courants](/docs/fr/common-workflows) : tutoriels pour le débogage, la refactorisation, les tests et bien d'autres
