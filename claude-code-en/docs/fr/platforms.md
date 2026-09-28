> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plateformes et intégrations

> Choisissez où exécuter Claude Code et ce que vous y connecter. Comparez le CLI, Desktop, VS Code, JetBrains, le web et les intégrations comme Chrome, Slack et CI/CD.

Claude Code exécute le même moteur sous-jacent partout, mais chaque surface est adaptée à une façon différente de travailler. Cette page vous aide à choisir la bonne plateforme pour votre flux de travail et à connecter les outils que vous utilisez déjà.

<h2 id="where-to-run-claude-code">
  Où exécuter Claude Code
</h2>

Choisissez une plateforme en fonction de votre façon de travailler et de l'endroit où se trouve votre projet.

| Plateforme                        | Idéale pour                                                                                                              | Ce que vous obtenez                                                                                                                                                                                        |
| :-------------------------------- | :----------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [CLI](/docs/fr/quickstart)             | Flux de travail en terminal, scripts, serveurs distants                                                                  | Ensemble complet de fonctionnalités, [Agent SDK](/docs/fr/headless), [utilisation de l'ordinateur](/docs/fr/computer-use) sur macOS (Pro et Max), fournisseurs tiers                                                 |
| [Desktop](/docs/fr/desktop)            | Examen visuel, sessions parallèles, configuration gérée                                                                  | Visionneuse de différences, aperçu de l'application, [utilisation de l'ordinateur](/docs/fr/desktop#let-claude-use-your-computer) et [Dispatch](/docs/fr/desktop#sessions-from-dispatch) sur Pro et Max              |
| [VS Code](/docs/fr/vs-code)            | Travailler dans VS Code sans basculer vers un terminal                                                                   | Différences intégrées, terminal intégré, contexte de fichier                                                                                                                                               |
| [JetBrains](/docs/fr/jetbrains)        | Travailler dans IntelliJ, PyCharm, WebStorm ou d'autres IDE JetBrains                                                    | Visionneuse de différences, partage de sélection, session de terminal                                                                                                                                      |
| [Web](/docs/fr/claude-code-on-the-web) | Tâches longues qui ne nécessitent pas beaucoup de direction, ou travail qui devrait continuer quand vous êtes hors ligne | Cloud, géré par Anthropic par défaut ; continue après votre déconnexion                                                                                                                                    |
| [Mobile](/docs/fr/mobile)              | Démarrer et surveiller les tâches loin de votre ordinateur                                                               | Sessions cloud depuis l'application Claude pour iOS et Android, [Remote Control](/docs/fr/remote-control) pour les sessions locales, [Dispatch](/docs/fr/desktop#sessions-from-dispatch) vers Desktop sur Pro et Max |

Le CLI est la surface la plus complète pour le travail natif en terminal : les scripts et l'Agent SDK sont exclusifs au CLI. Les fournisseurs tiers fonctionnent également dans [VS Code](/docs/fr/vs-code#use-third-party-providers) et dans [JetBrains](/docs/fr/feature-availability#features-available-on-every-provider), qui exécute le CLI dans le terminal de votre IDE. Les déploiements [Desktop](/docs/fr/desktop) d'entreprise prennent en charge Google Cloud's Agent Platform, et Desktop prend en charge les [fournisseurs de passerelle](/docs/fr/llm-gateway-connect#desktop-app) ; pour Amazon Bedrock ou Microsoft Foundry, utilisez le CLI ou une extension IDE, ou [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview), qui exécute l'onglet Code sur ces fournisseurs. Desktop et les extensions IDE échangent certaines fonctionnalités exclusives au CLI contre un examen visuel et une intégration plus étroite de l'éditeur. Le web s'exécute dans le cloud, donc les tâches continuent après votre déconnexion. Mobile est un client léger dans ces mêmes sessions cloud ou dans une session locale via Remote Control, et peut envoyer des tâches à Desktop avec Dispatch.

Vous pouvez mélanger les surfaces sur le même projet. La configuration, la mémoire du projet et les serveurs MCP sont partagés entre les surfaces locales.

<h2 id="connect-your-tools">
  Connectez vos outils
</h2>

Les intégrations permettent à Claude de travailler avec des services en dehors de votre base de code.

| Intégration                                      | Ce qu'elle fait                                                                                                | Utilisez-la pour                                                                                   |
| :----------------------------------------------- | :------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------- |
| [Chrome](/docs/fr/chrome)                             | Contrôle votre navigateur avec vos sessions connectées                                                         | Tester les applications web, remplir les formulaires, automatiser les sites sans API               |
| [GitHub Actions](/docs/fr/github-actions)             | Exécute Claude dans votre pipeline CI                                                                          | Examens automatisés des PR, triage des problèmes, maintenance programmée                           |
| [GitLab CI/CD](/docs/fr/gitlab-ci-cd)                 | Identique à GitHub Actions pour GitLab                                                                         | Automatisation pilotée par CI sur GitLab                                                           |
| [Code Review](/docs/fr/code-review)                   | Examine automatiquement chaque PR                                                                              | Détecter les bogues avant l'examen humain                                                          |
| [Slack](/docs/fr/slack)                               | Répond aux mentions `@Claude` dans vos canaux                                                                  | Transformer les rapports de bogues en demandes de tirage à partir du chat d'équipe                 |
| [Claude Tag](https://claude.com/docs/claude-tag) | Exécute `@Claude` en tant qu'identité partagée de votre organisation avec accès configuré par l'administrateur | Accès d'équipe partagé sur les plans Team et Enterprise, au lieu de sessions Slack par utilisateur |

Pour les intégrations non listées ici, les [serveurs MCP](/docs/fr/mcp) et les [connecteurs](/docs/fr/desktop#connect-external-tools) vous permettent de connecter presque n'importe quoi : Linear, Notion, Google Drive ou vos propres API internes.

<h2 id="work-when-you-are-away-from-your-terminal">
  Travaillez quand vous êtes loin de votre terminal
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

Si vous ne savez pas par où commencer, [installez le CLI](/docs/fr/quickstart) et exécutez-le dans un répertoire de projet. Si vous préférez ne pas utiliser un terminal, [Desktop](/docs/fr/desktop-quickstart) vous donne le même moteur avec une interface graphique.

<h2 id="related-resources">
  Ressources connexes
</h2>

<h3 id="platforms">
  Plateformes
</h3>

* [Démarrage rapide CLI](/docs/fr/quickstart) : installez et exécutez votre première commande dans le terminal
* [Desktop](/docs/fr/desktop) : examen visuel des différences, sessions parallèles, utilisation de l'ordinateur et Dispatch
* [VS Code](/docs/fr/vs-code) : l'extension Claude Code dans votre éditeur
* [JetBrains](/docs/fr/jetbrains) : l'extension pour IntelliJ, PyCharm et autres IDE JetBrains
* [Web](/docs/fr/claude-code-on-the-web) : sessions cloud depuis votre navigateur sur claude.ai/code qui continuent à s'exécuter quand vous vous déconnectez
* [Projects](/docs/fr/claude-projects) : une conversation où Claude coordonne de nombreuses sessions cloud pour un ensemble de travail et vous rend compte
* [Mobile](/docs/fr/mobile) : l'application Claude pour [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) et [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) pour démarrer et surveiller les tâches loin de votre ordinateur

<h3 id="integrations">
  Intégrations
</h3>

* [Chrome](/docs/fr/chrome) : automatisez les tâches du navigateur avec vos sessions connectées
* [Utilisation de l'ordinateur](/docs/fr/computer-use) : permettez à Claude d'ouvrir des applications et de contrôler votre écran sur macOS
* [GitHub Actions](/docs/fr/github-actions) : exécutez Claude dans votre pipeline CI
* [GitLab CI/CD](/docs/fr/gitlab-ci-cd) : la même chose pour GitLab
* [Code Review](/docs/fr/code-review) : examen automatique à chaque demande de tirage
* [Slack](/docs/fr/slack) : envoyez des tâches à partir du chat d'équipe, récupérez les PR en retour
* [Claude Tag](https://claude.com/docs/claude-tag) : exécutez `@Claude` comme identité partagée de votre organisation sur les plans Team et Enterprise

<h3 id="remote-access">
  Accès à distance
</h3>

* [Dispatch](/docs/fr/desktop#sessions-from-dispatch) : envoyez un message avec une tâche depuis votre téléphone et il peut générer une session Desktop
* [Remote Control](/docs/fr/remote-control) : pilotez une session en cours depuis votre téléphone ou navigateur
* [Channels](/docs/fr/channels) : poussez les événements des applications de chat ou de vos propres serveurs dans une session
* [Scheduled tasks](/docs/fr/scheduled-tasks) : exécutez les invites selon un calendrier récurrent
