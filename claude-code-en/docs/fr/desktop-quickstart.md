> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Démarrer avec l'application de bureau

> Installez Claude Code sur le bureau et commencez votre première session de codage

L'application de bureau vous donne accès à Claude Code avec une interface graphique conçue pour exécuter plusieurs sessions côte à côte : une barre latérale pour gérer les travaux parallèles, une disposition glisser-déposer avec un terminal intégré et un éditeur de fichiers, un examen des différences visuelles, un aperçu en direct de l'application, la surveillance des PR GitHub avec fusion automatique, et les tâches planifiées. Aucun terminal requis.

<CardGroup cols={3}>
  <Card title="Download for macOS" icon="apple" href="https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code&utm_medium=docs">
    Universal build for Intel and Apple Silicon
  </Card>

  <Card title="Download for Windows" icon="windows" href="https://claude.ai/api/desktop/win32/x64/setup/latest/redirect?utm_source=claude_code&utm_medium=docs">
    For x64 processors
  </Card>

  <Card title="Get Claude for Linux (beta)" icon="linux" href="/docs/en/desktop-linux">
    apt or .deb for Ubuntu and Debian
  </Card>
</CardGroup>

For Windows ARM64, download the [ARM64 installer](https://claude.ai/api/desktop/win32/arm64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs). On Linux, install with apt; see [Claude Desktop on Linux](/docs/en/desktop-linux).

<Note>
  Claude Code nécessite un [abonnement Pro, Max, Team ou Enterprise](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=desktop_quickstart_pricing).
</Note>

Cette page vous guide dans l'installation de l'application et le démarrage de votre première session. Si vous êtes déjà configuré, consultez [Utiliser Claude Code Desktop](/docs/fr/desktop) pour la référence complète.

L'application de bureau a trois onglets :

* **Chat** : Conversation générale sans accès aux fichiers, similaire à claude.ai.
* **Cowork** : Un agent autonome en arrière-plan qui travaille sur des tâches dans une machine virtuelle en sandbox avec son propre environnement, fonctionnant indépendamment pendant que vous faites autre chose. Les sessions Cowork sur l'appareil exécutent la VM sur votre ordinateur ; les sessions Cowork distantes s'exécutent sur une VM gérée par Anthropic à la place.
* **Code** : Un assistant de codage interactif avec accès direct à vos fichiers locaux. Vous examinez et approuvez chaque modification en temps réel.

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
  Commencer votre première session
</h2>

Avec l'onglet Code ouvert, choisissez un projet et donnez à Claude quelque chose à faire.

<Steps>
  <Step title="Choisir un environnement et un dossier">
    Sélectionnez **Local** pour exécuter Claude sur votre machine en utilisant vos fichiers directement. Cliquez sur **Sélectionner le dossier** et choisissez votre répertoire de projet.

    <Tip>
      Commencez par un petit projet que vous connaissez bien. C'est le moyen le plus rapide de voir ce que Claude Code peut faire. Sur Windows, [Git](https://git-scm.com/downloads/win) doit être installé pour que les sessions locales fonctionnent. La plupart des Mac incluent Git par défaut.
    </Tip>

    Vous pouvez également sélectionner :

    * **Remote** : Exécutez les sessions sur l'infrastructure cloud d'Anthropic qui continue même si vous fermez l'application. Les sessions distantes utilisent la même infrastructure que [Claude Code sur le web](/docs/fr/claude-code-on-the-web).
    * **SSH** : Connectez-vous à une machine distante via SSH, comme vos propres serveurs, VM cloud ou conteneurs de développement. Desktop installe Claude Code sur la machine distante automatiquement la première fois que vous vous connectez.
    * **WSL** (Windows) : Exécutez la session à l'intérieur d'une [distribution WSL 2](/docs/fr/desktop-wsl) ; Claude Code, les outils et git s'exécutent du côté Linux avec des chemins natifs.
  </Step>

  <Step title="Choisir un modèle">
    Sélectionnez un modèle dans la liste déroulante à côté du bouton d'envoi. Consultez [modèles](/docs/fr/model-config#available-models) pour une comparaison des modèles disponibles. Vous pouvez changer le modèle plus tard à partir de la même liste déroulante.
  </Step>

  <Step title="Dire à Claude ce qu'il faut faire">
    Tapez ce que vous voulez que Claude fasse :

    * `Trouver un commentaire TODO et le corriger`
    * `Ajouter des tests pour la fonction principale`
    * `Créer un CLAUDE.md avec des instructions pour cette base de code`

    Une [session](/docs/fr/desktop#work-in-parallel-with-sessions) est une conversation avec Claude sur votre code. Chaque session suit son propre contexte et ses modifications, vous pouvez donc travailler sur plusieurs tâches sans qu'elles n'interfèrent les unes avec les autres.
  </Step>

  <Step title="Examiner et accepter les modifications">
    Par défaut, l'onglet Code démarre en [mode Demander les permissions](/docs/fr/desktop#choose-a-permission-mode), où Claude propose des modifications et attend votre approbation avant de les appliquer. Vous verrez :

    1. Une [vue de différence](/docs/fr/desktop#review-changes-with-diff-view) montrant exactement ce qui changera dans chaque fichier
    2. Des boutons Accepter/Rejeter pour approuver ou refuser chaque modification
    3. Des mises à jour en temps réel pendant que Claude travaille sur votre demande

    Si vous rejetez une modification, Claude vous demandera comment vous aimeriez procéder différemment. Vos fichiers ne sont pas modifiés tant que vous n'acceptez pas.
  </Step>
</Steps>

<h2 id="now-what">
  Et maintenant ?
</h2>

Vous avez fait votre première modification. Pour la référence complète sur tout ce que Desktop peut faire, consultez [Utiliser Claude Code Desktop](/docs/fr/desktop). Voici quelques choses à essayer ensuite.

**Interrompre et diriger.** Vous pouvez rediriger Claude à tout moment. Cliquez sur le bouton d'arrêt pour interrompre immédiatement, ou tapez une correction et appuyez sur **Entrée** pour l'envoyer sans arrêter l'action en cours. De toute façon, vous n'avez pas besoin d'attendre qu'elle se termine ou de recommencer.

**Donner à Claude plus de contexte.** Tapez `@filename` dans la boîte de saisie pour extraire un fichier spécifique dans la conversation, joignez des images et des PDF en utilisant le bouton de pièce jointe, ou glissez-déposez des fichiers directement dans la saisie. Plus Claude a de contexte, meilleurs sont les résultats. Consultez [Ajouter des fichiers et du contexte](/docs/fr/desktop#add-files-and-context-to-prompts).

**Utiliser les skills pour les tâches répétables.** Tapez `/` ou cliquez sur **+** → **Slash commands** pour parcourir les [commandes intégrées](/docs/fr/commands), les [skills personnalisés](/docs/fr/skills) et les skills de plugin. Les skills sont des invites réutilisables que vous pouvez invoquer chaque fois que vous en avez besoin, comme des listes de contrôle d'examen de code ou des étapes de déploiement.

**Examiner les modifications avant de valider.** Après que Claude ait modifié les fichiers, un indicateur `+12 -1` apparaît. Cliquez dessus pour ouvrir la [vue de différence](/docs/fr/desktop#review-changes-with-diff-view), examinez les modifications fichier par fichier et commentez des lignes spécifiques. Claude lit vos commentaires et révise. Cliquez sur **Examiner le code** pour que Claude évalue lui-même les différences et laisse des suggestions en ligne.

**Ajuster le contrôle que vous avez.** Votre [mode de permission](/docs/fr/desktop#choose-a-permission-mode) définit le niveau de contrôle que Claude peut exercer sans demander d'approbation :

* **Manuel** : la valeur par défaut. Claude demande avant de modifier les fichiers ou d'exécuter des commandes.
* **Accepter les modifications** : Claude accepte automatiquement les modifications de fichiers pour une itération plus rapide.
* **Plan** : Claude propose une approche sans modifier aucun fichier, ce qui est utile avant une grande refonte.

**Ajouter des plugins pour plus de capacités.** Cliquez sur le bouton **+** à côté de la boîte de saisie et sélectionnez **Plugins** pour parcourir et installer les [plugins](/docs/fr/desktop#install-plugins) qui ajoutent des skills, des agents, des MCP servers et bien plus.

**Arranger votre espace de travail.** Glissez-déposez les volets de chat, de différence, de terminal, de fichier et de navigateur dans la disposition que vous souhaitez. Ouvrez le terminal avec **Ctrl+\`** pour exécuter des commandes aux côtés de votre session, ou cliquez sur un chemin de fichier pour l'ouvrir dans le volet de fichier. Consultez [Arranger votre espace de travail](/docs/fr/desktop#arrange-your-workspace).

**Prévisualiser votre application.** Lorsque vous exécutez votre serveur de développement dans le bureau, votre application s'ouvre dans le volet Navigateur, qui peut également [ouvrir des sites externes](/docs/fr/desktop#browse-external-sites). Claude peut voir l'application en cours d'exécution, tester les points de terminaison, inspecter les journaux et itérer sur ce qu'il voit. Consultez [Prévisualiser votre application](/docs/fr/desktop#preview-your-app).

**Suivre votre demande de tirage.** Après avoir ouvert une PR, Claude Code surveille les résultats des vérifications CI et peut corriger automatiquement les défaillances ou fusionner la PR une fois que toutes les vérifications sont réussies. Consultez [Surveiller l'état de la demande de tirage](/docs/fr/desktop#monitor-pull-request-status).

**Mettre Claude sur un calendrier.** Configurez les [tâches planifiées](/docs/fr/desktop-scheduled-tasks) pour exécuter Claude automatiquement de manière récurrente : un examen de code quotidien chaque matin, un audit de dépendances hebdomadaire ou un briefing qui extrait de vos outils connectés.

**Augmenter l'échelle quand vous êtes prêt.** Ouvrez les [sessions parallèles](/docs/fr/desktop#work-in-parallel-with-sessions) à partir de la barre latérale pour travailler sur plusieurs tâches à la fois, chacune dans son propre Git worktree, et ouvrez le [volet des tâches](/docs/fr/desktop#watch-background-tasks) pour regarder les sous-agents et les commandes en arrière-plan qu'une session exécute. Ouvrez un [chat latéral](/docs/fr/desktop#ask-a-side-question-without-derailing-the-session) pour poser une question sans dérailler le fil principal. Envoyez les [travaux de longue durée vers le cloud](/docs/fr/desktop#run-long-running-tasks-remotely) pour qu'ils continuent même si vous fermez l'application, ou [continuez une session sur le web ou dans votre IDE](/docs/fr/desktop#continue-in-another-surface) si une tâche prend plus de temps que prévu. [Connectez les outils externes](/docs/fr/desktop#extend-claude-code) comme GitHub, Slack et Linear pour réunir votre flux de travail.

<h2 id="coming-from-the-cli">
  Venant de la CLI ?
</h2>

Desktop exécute le même moteur que la CLI avec une interface graphique. Vous pouvez exécuter les deux simultanément sur le même projet, et ils partagent la configuration (fichiers CLAUDE.md, MCP servers, hooks, skills et paramètres). Pour une comparaison complète des fonctionnalités, des équivalents de drapeaux et de ce qui n'est pas disponible dans Desktop, consultez [Comparaison CLI](/docs/fr/desktop#coming-from-the-cli).

<h2 id="what’s-next">
  Prochaines étapes
</h2>

* [Utiliser Claude Code Desktop](/docs/fr/desktop) : modes de permission, sessions parallèles, vue de différence, connecteurs et configuration d'entreprise
* [Dépannage](/docs/fr/desktop#troubleshooting) : solutions aux erreurs courantes et problèmes de configuration
* [Meilleures pratiques](/docs/fr/best-practices) : conseils pour rédiger des invites efficaces et tirer le meilleur parti de Claude Code
* [Flux de travail courants](/docs/fr/common-workflows) : tutoriels pour le débogage, la refonte, les tests et bien plus
