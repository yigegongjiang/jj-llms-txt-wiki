> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Référence des outils

> Référence complète des outils que Claude Code peut utiliser, y compris les exigences de permission et le comportement par outil.

Claude Code a accès à un ensemble d'outils intégrés qui l'aident à comprendre et modifier votre base de code. Les noms des outils sont les chaînes exactes que vous utilisez dans les [règles de permission](/docs/fr/permissions#tool-specific-permission-rules), les [listes d'outils des sous-agents](/docs/fr/sub-agents), et les [correspondances de hooks](/docs/fr/hooks).

Pour contrôler les outils que Claude peut utiliser et quand il demande d'abord, configurez les [règles de permission](/docs/fr/permissions#tool-specific-permission-rules) dans vos paramètres, les [hooks](/docs/fr/hooks), ou la [liste d'outils d'un sous-agent](/docs/fr/sub-agents#supported-frontmatter-fields). Consultez [Configurer les outils avec les règles de permission et les hooks](#configure-tools-with-permission-rules-and-hooks) pour chaque endroit qui accepte un nom d'outil.

Pour ajouter des outils personnalisés, connectez un [serveur MCP](/docs/fr/mcp). Pour étendre Claude avec des flux de travail réutilisables basés sur des invites, écrivez une [skill](/docs/fr/skills), qui s'exécute via l'outil `Skill` existant plutôt que d'ajouter une nouvelle entrée d'outil.

<Info>
  Sur les plans Pro, Max et Team, Claude Code démarre les sessions en [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode), où un classificateur décide la plupart de ces invites à votre place. La colonne `Permission required` indique si l'outil demande en [mode Manuel](/docs/fr/permission-modes) pour les chemins à l'intérieur du répertoire de travail. Les outils d'accès aux fichiers marqués Non, y compris `Read`, `Grep` et `Glob`, demandent toujours pour les chemins en dehors du [répertoire de travail et des répertoires supplémentaires](/docs/fr/permissions#working-directories). `Bash` est marqué Oui mais exécute un ensemble intégré de [commandes en lecture seule](/docs/fr/permissions#read-only-commands) sans demander.
</Info>

| Outil                  | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | Permission requise |
| :--------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------- |
| `Agent`                | Crée un [sous-agent](/docs/fr/sub-agents) avec sa propre fenêtre de contexte pour gérer une tâche. Avec les [équipes d'agents](/docs/fr/agent-teams) activées, un appel qui porte un `name` peut lancer un [coéquipier](/docs/fr/agent-teams#how-claude-starts-agent-teams) à la place. Consultez [Comportement de l'outil Agent](#agent-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | Non                |
| `Artifact`             | Publie un fichier HTML ou Markdown en tant qu'[artifact](/docs/fr/artifacts) : une page privée et interactive sur claude.ai. Vous pouvez la partager avec un lien public, ou au sein de votre organisation sur les plans Team et Enterprise, où le partage public nécessite qu'un propriétaire l'[active](/docs/fr/artifacts#control-public-sharing). Nécessite un plan Pro, Max, Team ou Enterprise et l'authentification `/login` ; consultez [Disponibilité](/docs/fr/artifacts#availability)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Oui                |
| `AskUserQuestion`      | Pose des questions à choix multiples pour recueillir les exigences ou clarifier l'ambiguïté. Les questions restent ouvertes jusqu'à ce que vous y répondiez par défaut. Consultez [Comportement de l'outil AskUserQuestion](#askuserquestion-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | Non                |
| `Bash`                 | Exécute des commandes shell dans votre environnement. Consultez [Comportement de l'outil Bash](#bash-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | Oui                |
| `CronCreate`           | Planifie une invite récurrente ou ponctuelle dans la session actuelle. Les tâches sont limitées à la session et restaurées sur `--resume` ou `--continue` si non expirées. Consultez [tâches planifiées](/docs/fr/scheduled-tasks)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Non                |
| `CronDelete`           | Annule une tâche planifiée par ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Non                |
| `CronList`             | Liste toutes les tâches planifiées dans la session                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Non                |
| `Edit`                 | Effectue des modifications ciblées sur des fichiers spécifiques. Consultez [Comportement de l'outil Edit](#edit-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Oui                |
| `EndConversation`      | Termine la session, dans les rares cas d'entrée abusive soutenue ou quand vous demandez à Claude de démontrer l'outil. Nécessite Claude Code v2.1.213 ou ultérieur. Consultez [Comportement de l'outil EndConversation](#endconversation-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Non                |
| `EnterPlanMode`        | Bascule en mode plan pour concevoir une approche avant de coder                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Non                |
| `EnterWorktree`        | Crée un [git worktree](/docs/fr/worktrees) isolé et y bascule. Passez un `path` pour basculer dans un worktree existant au lieu d'en créer un nouveau. À la première entrée, la cible peut être un worktree du référentiel actuel ou, dans un espace de travail multi-référentiel, d'un référentiel imbriqué à l'intérieur. Avant v2.1.203, un worktree d'un référentiel imbriqué était rejeté. Un `path` en dehors de `.claude/worktrees/` demande votre approbation avant d'entrer, car il déplace le répertoire de travail de la session et l'accès en écriture à cet emplacement. La création de nouveaux worktrees et les chemins sous `.claude/worktrees/` ne demandent pas. Avant v2.1.206, Claude entrait dans les chemins en dehors de `.claude/worktrees/` sans demander. À partir d'une session worktree, ou d'un sous-agent avec un répertoire de travail épinglé tel que [`isolation: worktree`](/docs/fr/sub-agents#supported-frontmatter-fields), seule la forme `path` est disponible et la cible doit être sous `.claude/worktrees/` du référentiel de la session                              | Oui                |
| `ExitPlanMode`         | Présente un plan pour approbation et quitte le mode plan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | Oui                |
| `ExitWorktree`         | Quitte une session worktree et revient au répertoire d'origine. Non disponible pour les sous-agents qui s'exécutent déjà dans leur propre répertoire de travail, comme avec [`isolation: worktree`](/docs/fr/sub-agents#supported-frontmatter-fields)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Non                |
| `Glob`                 | Trouve des fichiers basés sur la correspondance de motifs. Absent par défaut sur macOS, Linux et WSL. Consultez [Comportement de l'outil Glob](#glob-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | Non                |
| `Grep`                 | Recherche des motifs dans le contenu des fichiers. Absent par défaut sur macOS, Linux et WSL. Consultez [Comportement de l'outil Grep](#grep-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | Non                |
| `ListAgents`           | Liste les agents que Claude peut contacter avec `SendMessage` : les sous-agents de la session, les [coéquipiers d'équipe d'agents](/docs/fr/agent-teams), vos autres sessions Claude Code locales, et, tandis que cette session est connectée à [Remote Control](/docs/fr/remote-control), vos sessions [Claude Code sur le web](/docs/fr/claude-code-on-the-web) et vos sessions Remote Control sur d'autres machines. Soutient la commande `/list-agents`. Consultez [messagerie inter-sessions](/docs/fr/cross-session-messaging). Nécessite Claude Code v2.1.224 ou ultérieur, et n'apparaît que dans les sessions où la [messagerie inter-sessions est activée](/docs/fr/cross-session-messaging#availability). Les lignes de coéquipiers et la première ligne affichant le nom de cette session nécessitent v2.1.239 ou ultérieur                                                                                                                                                                                                                                                                                        | Non                |
| `ListMcpResourcesTool` | Liste les ressources exposées par les [serveurs MCP](/docs/fr/mcp) connectés                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Non                |
| `LSP`                  | Intelligence du code via les serveurs de langage : accéder aux définitions, trouver les références, signaler les erreurs de type et les avertissements. Consultez [Comportement de l'outil LSP](#lsp-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | Non                |
| `Monitor`              | Exécute une commande en arrière-plan et renvoie chaque ligne de sortie à Claude, afin qu'il puisse réagir aux entrées de journal, aux modifications de fichiers ou à l'état interrogé en milieu de conversation. Peut également ouvrir une WebSocket et traiter chaque message entrant comme un événement. Consultez [Outil Monitor](#monitor-tool)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | Oui                |
| `NotebookEdit`         | Modifie les cellules du notebook Jupyter. Consultez [Comportement de l'outil NotebookEdit](#notebookedit-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Oui                |
| `PowerShell`           | Exécute les commandes PowerShell en mode natif. Consultez [Outil PowerShell](#powershell-tool) pour la disponibilité                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Oui                |
| `PushNotification`     | Envoie une notification de bureau, et une notification push téléphonique quand [Remote Control](/docs/fr/remote-control) est connecté, afin qu'une tâche longue ou une [tâche planifiée](/docs/fr/scheduled-tasks) puisse vous atteindre quand vous vous éloignez. La livraison push s'effectue via l'infrastructure hébergée par Anthropic, qui n'est pas accessible depuis Amazon Bedrock, Claude Platform sur AWS, Google Cloud's Agent Platform, ou Microsoft Foundry                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Non                |
| `Read`                 | Lit le contenu des fichiers. Consultez [Comportement de l'outil Read](#read-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | Non                |
| `ReadMcpResourceTool`  | Lit une ressource MCP spécifique par URI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | Non                |
| `RemoteTrigger`        | Crée, met à jour, exécute et liste les [Routines](/docs/fr/routines) sur claude.ai. Soutient la commande `/schedule`. La [référence d'entrée `RemoteTrigger`](/docs/fr/agent-sdk/typescript#remotetrigger) documente chaque action et les politiques organisationnelles qui suppriment l'outil. Les routines vivent sur claude.ai et nécessitent un plan Pro, Max, Team ou Enterprise, donc cet outil n'est pas accessible depuis Amazon Bedrock, Claude Platform sur AWS, Google Cloud's Agent Platform, ou Microsoft Foundry                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Non                |
| `ReportFindings`       | Signale les résultats de l'examen du code sous forme de liste structurée, avec un fichier, un résumé et un scénario d'échec par résultat, afin que Claude Code puisse les rendre au lieu de les imprimer en tant que texte. Claude l'appelle quand les instructions actives d'examen du code le lui demandent. Nécessite Claude Code v2.1.196 ou ultérieur. À partir de v2.1.199, un résultat peut également porter un slug `category` optionnel, tel que `correctness` ou `test-coverage`, affiché à côté de l'emplacement du fichier dans la liste rendue                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | Non                |
| `ScheduleWakeup`       | Reprogramme la prochaine itération d'une [`/loop` auto-rythmée](/docs/fr/scheduled-tasks#let-claude-choose-the-interval). Claude l'appelle à la fin de chaque itération pour choisir quand la prochaine s'exécute, entre une minute et une heure ; vous ne l'appelez pas directement. Pour terminer la boucle à la place, Claude l'appelle avec `stop: true`, ce qui annule le réveil en attente. Le champ `stop` nécessite Claude Code v2.1.202 ou ultérieur. Le réveil en attente apparaît dans `session_crons` dans [Entrée du hook Stop](/docs/fr/hooks#stop-input)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Non                |
| `SendFeedback`         | Rédige un rapport de rétroaction sur Claude Code, couvrant un problème de produit ou le comportement de Claude lui-même dans la session, et le met en file d'attente sur votre machine pour que vous le révisiez. Claude Code n'envoie rien jusqu'à ce que vous choisissiez d'envoyer le brouillon. Consultez [Comportement de l'outil SendFeedback](#sendfeedback-tool-behavior). Nécessite Claude Code v2.1.238 ou ultérieur                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Non                |
| `SendMessage`          | Envoie un message à un autre agent : un [coéquipier d'équipe d'agents](/docs/fr/agent-teams), un [sous-agent qu'il reprend](/docs/fr/sub-agents#resume-subagents) par ID ou nom d'agent, ou l'une de vos autres sessions Claude Code, sur cette machine ou au-delà. La messagerie d'autres sessions nécessite Claude Code v2.1.224 ou ultérieur. La [messagerie inter-sessions](/docs/fr/cross-session-messaging) couvre les sessions que Claude peut atteindre, [à quoi ressemble un message quand il arrive](/docs/fr/cross-session-messaging#what-a-message-looks-like), et [comment Claude reçoit un avis quand une autre session devient inactive](/docs/fr/cross-session-messaging#get-a-notice-when-another-session-goes-idle). Claude peut inclure une entrée `summary` optionnelle, généralement 5-10 mots, que Claude Code affiche comme un aperçu d'une ligne. Quand Claude l'omet sur un [message en texte brut](/docs/fr/cross-session-messaging#limitations), Claude Code utilise la première ligne du message comme résumé. Claude Code tronque un résumé plus long que 200 caractères avec des points de suspension | Non                |
| `SendUserFile`         | Envoie les fichiers de la session vers vous avec une légende optionnelle, afin qu'un rapport généré, un diagramme, une capture d'écran ou un artefact construit atteigne votre appareil au lieu d'être seulement mentionné dans la transcription. À partir de v2.1.196, l'entrée `display` optionnelle contrôle la présentation : `render` ouvre le fichier en ligne dans le client, `attach` affiche une carte de téléchargement uniquement, et quand elle n'est pas définie, le client décide par type de fichier. Disponible quand un client [Remote Control](/docs/fr/remote-control) est connecté ou dans une [session cloud](/docs/fr/claude-code-on-the-web). La livraison s'effectue via l'infrastructure hébergée par Anthropic, donc l'outil n'est pas disponible sur Amazon Bedrock, Google Cloud's Agent Platform, ou Microsoft Foundry                                                                                                                                                                                                                                                             | Non                |
| `ShareOnboardingGuide` | Télécharge `ONBOARDING.md` et retourne un lien de partage que les coéquipiers peuvent ouvrir dans Claude Code. Appelé depuis `/team-onboarding` après que le guide soit écrit. Disponible pour les abonnés claude.ai sur les plans Pro, Max, Team et Enterprise                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Oui                |
| `Skill`                | Exécute une [skill](/docs/fr/skills#control-who-invokes-a-skill) dans la conversation principale                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | Oui                |
| `SubagentHandback`     | Livre le rapport final d'un sous-agent à la conversation qui reçoit le résultat de ce sous-agent. Fourni uniquement en [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode), aux sous-agents que l'outil Agent exécute localement autres que les [forks](/docs/fr/sub-agents#fork-the-current-conversation), et disponible dans le CLI terminal, les extensions IDE, les sessions cloud et le SDK Agent ; le classificateur examine le rapport avant qu'il soit livré. Nécessite Claude Code v2.1.271 ou ultérieur                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | Non                |
| `TaskCreate`           | Crée une nouvelle tâche dans la liste des tâches. Fourni par défaut uniquement sur les modèles listés sous [Disponibilité de l'outil Task](#task-tool-availability), et sur d'autres modèles quand vous acceptez                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Non                |
| `TaskGet`              | Récupère les détails complets d'une tâche spécifique. Fourni par défaut uniquement sur les modèles listés sous [Disponibilité de l'outil Task](#task-tool-availability), et sur d'autres modèles quand vous acceptez                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Non                |
| `TaskList`             | Liste toutes les tâches avec leur statut actuel. Fourni par défaut uniquement sur les modèles listés sous [Disponibilité de l'outil Task](#task-tool-availability), et sur d'autres modèles quand vous acceptez                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Non                |
| `TaskOutput`           | Récupère la sortie d'une tâche en arrière-plan. Déprécié en faveur de `Read` sur le chemin du fichier de sortie de la tâche. Quand aucune tâche ne correspond à l'ID, l'erreur liste les agents d'arrière-plan en cours d'exécution par ID et description. Avant v2.1.203, l'erreur nommait seulement l'ID manquant                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | Non                |
| `TaskStop`             | Arrête une tâche d'arrière-plan en cours d'exécution par ID. Il accepte également un [coéquipier d'équipe d'agents](/docs/fr/agent-teams) ou un agent d'arrière-plan nommé par ID ou nom d'agent. Avant v2.1.198, il acceptait seulement un ID de tâche d'arrière-plan. Quand aucune tâche ne correspond à l'ID, l'erreur liste les agents d'arrière-plan en cours d'exécution par ID et description, y compris les agents qu'un autre agent a créés. Avant v2.1.203, l'erreur listait les coéquipiers et agents nommés en cours d'exécution mais pas les agents d'arrière-plan qu'un autre agent a créés, donc ceux-ci ne pouvaient pas être identifiés ou arrêtés à partir de la conversation principale                                                                                                                                                                                                                                                                                                                                                                                                 | Non                |
| `TaskUpdate`           | Met à jour le statut de la tâche, les dépendances, les détails, ou supprime les tâches. Fourni par défaut uniquement sur les modèles listés sous [Disponibilité de l'outil Task](#task-tool-availability), et sur d'autres modèles quand vous acceptez                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Non                |
| `TodoWrite`            | Gère la liste de contrôle des tâches de la session. Désactivé par défaut en faveur de `TaskCreate`, `TaskGet`, `TaskList` et `TaskUpdate`. Définissez `CLAUDE_CODE_ENABLE_TASKS=0` pour le réactiver dans les [sessions qui ont les outils de suivi des tâches](#task-tool-availability)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | Non                |
| `ToolSearch`           | Recherche et charge les outils différés quand la [recherche d'outils](/docs/fr/mcp#scale-with-mcp-tool-search) est activée                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Non                |
| `WaitForMcpServers`    | Attend un ou plusieurs [serveurs MCP](/docs/fr/mcp) qui se connectent toujours en arrière-plan, afin qu'une demande puisse utiliser leurs outils sans redémarrer la session. Claude l'appelle quand un serveur nécessaire n'est pas encore connecté. N'apparaît que quand la [recherche d'outils](/docs/fr/mcp#scale-with-mcp-tool-search) est désactivée, car `ToolSearch` gère l'attente quand elle est activée                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Non                |
| `WebFetch`             | Récupère le contenu d'une URL spécifiée. Consultez [Comportement de l'outil WebFetch](#webfetch-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Oui                |
| `WebSearch`            | Effectue des recherches web. Consultez [Comportement de l'outil WebSearch](#websearch-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Oui                |
| `Workflow`             | Exécute un [flux de travail dynamique](/docs/fr/workflows) : un script qui orchestre de nombreux sous-agents en arrière-plan et retourne un résultat consolidé                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Oui                |
| `Write`                | Crée ou remplace les fichiers. Consultez [Comportement de l'outil Write](#write-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Oui                |

<h2 id="configure-tools-with-permission-rules-and-hooks">
  Configurer les outils avec des règles de permission et des hooks
</h2>

Pour la plupart, Claude décide quand utiliser ces outils et vous n'avez pas besoin de les nommer vous-même lors de l'interaction avec Claude. Vous référencez les noms d'outils directement lors de la définition des permissions et d'autres configurations :

* dans [`permissions.allow`](/docs/fr/settings-reference#permissions-allow) et [`permissions.deny`](/docs/fr/settings-reference#permissions-deny) dans les paramètres, et l'interface `/permissions`
* dans les drapeaux CLI [`--allowedTools` et `--disallowedTools`](/docs/fr/cli-reference)
* dans les options [`allowedTools` et `disallowedTools`](/docs/fr/agent-sdk/permissions#allow-and-deny-rules) du SDK Agent
* dans le frontmatter [`allowed-tools`](/docs/fr/skills#frontmatter-reference) d'une skill
* dans la condition [`if`](/docs/fr/hooks-guide#filter-by-tool-name-and-arguments-with-the-if-field) d'un hook

Tous ces éléments acceptent le même format de règle, `ToolName(specifier)`. Le spécificateur dépend de l'outil, et plusieurs outils partagent un format :

| Format de règle                | S'applique à              | Détails                                                                             |
| :----------------------------- | :------------------------ | :---------------------------------------------------------------------------------- |
| `Bash(npm run *)`              | Bash, Monitor             | [Correspondance de motif de commande](/docs/fr/permissions#bash)                         |
| `PowerShell(Get-ChildItem *)`  | PowerShell                | [Correspondance de motif de commande](/docs/fr/permissions#powershell)                   |
| `Read(~/secrets/**)`           | Read, Grep, Glob, LSP     | [Correspondance de motif de chemin](/docs/fr/permissions#read-and-edit)                  |
| `Edit(/src/**)`                | Edit, Write, NotebookEdit | [Correspondance de motif de chemin](/docs/fr/permissions#read-and-edit)                  |
| `Skill(deploy *)`              | Skill                     | [Correspondance de nom de skill](/docs/fr/skills#restrict-claude%E2%80%99s-skill-access) |
| `Agent(Explore)`               | Agent                     | [Correspondance de type de subagent](/docs/fr/permissions#agent-subagents)               |
| `WebFetch(domain:example.com)` | WebFetch                  | [Correspondance de domaine](/docs/fr/permissions#webfetch)                               |
| `WebSearch`                    | WebSearch                 | Pas de spécificateur ; autoriser ou refuser l'outil dans son ensemble               |

Les outils non listés ici, tels que `ExitPlanMode` ou `ShareOnboardingGuide`, acceptent uniquement le nom d'outil nu sans spécificateur.

Une règle d'autorisation `Edit(...)` accorde également l'accès en lecture au même chemin, vous n'avez donc pas besoin d'une règle `Read(...)` correspondante. Une règle de refus `Read(...)` bloque également les outils Edit et Write sur le même chemin, y compris la création d'un nouveau fichier à cet endroit, car les deux outils modifient le contenu que Claude doit pouvoir relire. La vérification de refus `Read` nécessite Claude Code v2.1.208 ou ultérieur sur les modifications, et v2.1.228 ou ultérieur sur les écritures.

Les champs `matcher` des hooks utilisent des noms d'outils nus, pas le format entre parenthèses. Voir [motifs de correspondance](/docs/fr/hooks#matcher-patterns) pour les règles de correspondance. Pour les noms de champs que chaque outil transmet à `tool_input` dans les hooks, voir la [référence d'entrée PreToolUse](/docs/fr/hooks#pretooluse-input).

<h2 id="agent-tool-behavior">
  Comportement de l'outil Agent
</h2>

L'outil Agent lance un sous-agent dans une fenêtre de contexte séparée. Le sous-agent travaille de manière autonome sur sa tâche, puis retourne son résultat à la conversation parent. Le parent ne voit pas les appels d'outils intermédiaires ou les résultats du sous-agent, seulement ce résultat final. Avec les [équipes d'agents](/docs/fr/agent-teams) activées, un appel qui porte un `name` peut lancer un [coéquipier](/docs/fr/agent-teams#how-claude-starts-agent-teams) à la place, qui rend compte via des messages d'équipe plutôt que de retourner un résultat.

Pour limiter le nombre de tours qu'un sous-agent exécute, définissez `maxTurns` dans la [définition du sous-agent](/docs/fr/sub-agents#supported-frontmatter-fields). Lorsque le sous-agent atteint la limite, Claude Code marque le résultat retourné comme résultat partiel, et Claude peut [reprendre le sous-agent](/docs/fr/sub-agents#resume-subagents) pour continuer.

Le même outil Agent lance également des [sous-agents dupliqués](/docs/fr/sub-agents#fork-the-current-conversation) partout où le [mode fork](/docs/fr/sub-agents#turn-fork-mode-on-or-off) est activé. Un fork hérite de la conversation parent complète au lieu de commencer à zéro, s'exécute en arrière-plan en dehors des [cas qui restent au premier plan](/docs/fr/sub-agents#run-subagents-in-foreground-or-background), et affiche toujours les invites de permission dans votre terminal. Le reste de cette section décrit les sous-agents non-fork.

Les outils qu'un sous-agent non-fork peut utiliser dépendent des champs `tools` et `disallowedTools` dans la [définition du sous-agent](/docs/fr/sub-agents) :

* **Aucun champ défini** : le sous-agent hérite de chaque [outil disponible pour les sous-agents](/docs/fr/sub-agents#available-tools).
* **`tools` uniquement** : le sous-agent obtient uniquement les outils listés.
* **`disallowedTools` uniquement** : le sous-agent obtient chaque outil parent sauf ceux listés.
* **Les deux définis** : `disallowedTools` a la priorité. Un outil listé dans les deux est supprimé.

Dans tous les cas, l'ensemble résolu est limité aux [outils disponibles pour les sous-agents](/docs/fr/sub-agents#available-tools) : un outil qui n'est pas disponible pour les sous-agents n'est jamais accordé, même s'il est listé dans `tools`. Lorsque les conditions dans l'entrée de la table d'outils `SubagentHandback` sont remplies, Claude Code donne également au sous-agent cet outil, même si vous l'omettez de `tools` ou le listez dans `disallowedTools`.

Si chaque entrée dans la liste `tools` d'un sous-agent ne correspond pas à un outil utilisable, l'outil Agent retourne généralement une erreur nommant les entrées au lieu de lancer le sous-agent ; voir [Agent would be spawned with zero tools](/docs/fr/errors#agent-would-be-spawned-with-zero-tools) pour le message et comment corriger chaque entrée.

Le lancement du sous-agent ne demande pas lui-même la permission. Claude Code vérifie les appels d'outils du sous-agent par rapport à vos règles de permission au fur et à mesure de son exécution.

L'endroit où vous voyez les invites de permission d'un sous-agent dépend de s'il s'exécute au premier plan ou en arrière-plan. Claude Code exécute les sous-agents en arrière-plan par défaut, en dehors des [cas qui s'exécutent au premier plan](/docs/fr/sub-agents#run-subagents-in-foreground-or-background).

* **Sous-agents au premier plan** affichent les mêmes invites de permission que vous verriez dans la conversation principale, au moment où chaque appel d'outil se produit.
* **Sous-agents en arrière-plan** affichent les invites de permission dans votre session principale à partir de v2.1.186. L'invite nomme quel sous-agent demande, et appuyer sur Échap refuse cet appel d'outil unique sans arrêter le sous-agent. Avant v2.1.186, les sous-agents en arrière-plan refusaient automatiquement tout appel d'outil qui demanderait autrement la permission et continuaient sans cet outil.

Pour [limiter ce qu'un sous-agent peut atteindre](/docs/fr/sub-agents#control-subagent-capabilities) en premier lieu, réduisez son champ `tools`, par exemple en laissant Bash hors de la liste, ou définissez des règles de refus dans vos paramètres.

<h2 id="askuserquestion-tool-behavior">
  Comportement de l'outil AskUserQuestion
</h2>

Claude utilise `AskUserQuestion` pour vous poser des questions à choix multiples lorsqu'il a besoin d'une décision ou d'une clarification. Répondez en choisissant une option, ou tapez votre propre texte via la ligne `Other` ou le champ des notes.

Lorsque vous répondez en tapant votre propre texte, Claude Code relaye la réponse avec une formulation neutre afin que Claude suive ce que vous avez écrit, y compris une demande d'attendre ou d'expliquer d'abord.

<h3 id="question-auto-continue-timeout">
  Délai d'expiration de la continuation automatique des questions
</h3>

Les questions restent ouvertes jusqu'à ce que vous y répondiez. Si vous souhaitez qu'une question que vous laissez sans réponse se ferme finalement et permette à Claude de continuer sans vous, définissez le paramètre [`askUserQuestionTimeout`](/docs/fr/settings-reference#askuserquestiontimeout) sur `60s`, `5m`, ou `10m`, soit dans votre fichier `settings.json` utilisateur, soit à partir de la ligne **Question auto-continue timeout** dans `/config`.

Après qu'une question soit restée aussi longtemps sans entrée, la boîte de dialogue se ferme d'elle-même : elle soumet toutes les options que vous aviez déjà sélectionnées et indique à Claude que vous êtes peut-être absent de votre clavier, afin que Claude procède selon son propre jugement et puisse poser à nouveau la question plus tard. Vous voyez un compte à rebours pour les 20 dernières secondes. Appuyez sur n'importe quelle touche pour redémarrer le minuteur ; sur les terminaux qui signalent le focus, le passage à la fenêtre le redémarre également.

Le délai d'expiration s'applique uniquement aux questions à choix multiples de `AskUserQuestion` ; les invites de permission, y compris l'approbation du plan, ne se résolvent jamais automatiquement en cas d'inactivité.

<h2 id="bash-tool-behavior">
  Comportement de l'outil Bash
</h2>

L'outil Bash exécute chaque commande dans un processus séparé.

<h3 id="what-persists-between-commands">
  Ce qui persiste entre les commandes
</h3>

* Quand Claude exécute `cd` dans la session principale, le nouveau répertoire de travail est conservé pour les commandes Bash ultérieures tant qu'il reste à l'intérieur du répertoire du projet ou d'un [répertoire de travail supplémentaire](/docs/fr/permissions#working-directories) que vous avez ajouté avec `--add-dir`, `/add-dir`, ou `additionalDirectories` dans les paramètres. Cela inclut les commandes que Claude exécute en réponse à vos messages ultérieurs.
  * Les sessions de sous-agent ne conservent jamais les changements de répertoire de travail.
  * Si `cd` aboutit en dehors de ces répertoires, Claude Code réinitialise le répertoire du projet et ajoute `Shell cwd was reset to <dir>` au résultat de l'outil.
  * Pour désactiver cette conservation afin que chaque commande Bash commence dans le répertoire du projet, définissez `CLAUDE_BASH_MAINTAIN_PROJECT_WORKING_DIR=1`.
* Les variables d'environnement ne persistent pas. Un `export` dans une commande ne sera pas disponible dans la suivante.
* Les alias et les fonctions shell définis dans votre fichier de démarrage shell sont disponibles. Au démarrage de la session, Claude Code source `~/.zshrc`, `~/.bashrc`, ou `~/.profile` selon votre shell, capture les alias, fonctions et options shell résultants, et les applique à chaque commande Bash.

Activez votre virtualenv ou votre environnement conda avant de lancer Claude Code. Pour que les variables d'environnement persistent entre les commandes Bash, définissez [`CLAUDE_ENV_FILE`](/docs/fr/env-vars) sur un script shell avant de lancer Claude Code, ou utilisez un [hook SessionStart](/docs/fr/hooks#persist-environment-variables) pour le remplir dynamiquement.

<h3 id="timeout-and-output-limits">
  Limites de délai d'attente et de sortie
</h3>

Chaque commande s'exécute sous un délai d'attente, et Claude le gère : quand il a besoin de plus que la valeur par défaut pour une commande, il transmet le paramètre `timeout` avec cet appel — vous ne définissez jamais un délai d'attente par commande. Deux [variables d'environnement](/docs/fr/env-vars) limitent ce que Claude obtient :

* `BASH_DEFAULT_TIMEOUT_MS` — la valeur par défaut quand Claude ne transmet pas de délai d'attente ; deux minutes par défaut
* `BASH_MAX_TIMEOUT_MS` — avec la valeur par défaut, définit le plafond qui limite ce que Claude demande : le plafond effectif est le plus grand des deux, dix minutes par défaut

<h4 id="output-limits">
  Limites de sortie
</h4>

Claude Code diffuse la sortie d'une commande vers un fichier de travail au fur et à mesure que la commande s'exécute ; une commande dont la sortie dépasse 5 Go est arrêtée. Quand la commande se termine, Claude Code relit la sortie à partir de ce fichier, jusqu'à la fenêtre de relecture décrite ci-dessous. La quantité de sortie qui atteint Claude en ligne dépend de la façon dont Claude Code traite le résultat comme un échec :

| Résultat | Ce que Claude obtient                                                                                                                                                                                                                                                                    |
| :------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Valide   | En ligne jusqu'à environ 30 000 caractères par défaut ; au-delà, le chemin d'un fichier enregistré dans le répertoire de session et tronqué au-delà de 64 Mio, plus un aperçu des 2 000 premiers caractères au maximum, et Claude lit ou recherche le fichier quand il a besoin du reste |
| Échec    | En ligne jusqu'à environ 10 000 caractères ; au-delà, un extrait début-fin de cette taille coupé de la fenêtre de relecture, sans chemin de fichier                                                                                                                                      |

Une commande qui se termine avec le code 1 compte comme un résultat valide pour l'outil Bash uniquement quand Claude Code reconnaît le code de sortie 1 comme un résultat bénin pour cette commande : `grep`, `rg`, `egrep`, `fgrep`, `find`, `diff`, `test`, et `[`, plus `git diff` et `git grep`. Toute autre commande qui se termine avec le code 1 compte comme un échec, même quand le code 1 est un résultat informatif bénin : pas de correspondances pour `pgrep` et `jq -e`, fichiers qui diffèrent pour `cmp`.

[`BASH_MAX_OUTPUT_LENGTH`](/docs/fr/env-vars) définit le nombre de caractères de sortie que Claude Code relit à partir du fichier de travail dans le résultat d'une commande : 30 000 par défaut, jusqu'à un plafond dur de 150 000. Augmentez-le quand vos commandes dépassent régulièrement cette fenêtre, comme une compilation détaillée ou un journal de suite de tests complet. L'augmenter élargit la fenêtre de relecture, qui est aussi la fenêtre à partir de laquelle l'extrait d'une commande défaillante est coupé. Elle n'augmente pas les plafonds en ligne : un résultat valide au-delà du plafond en ligne arrive comme un chemin de fichier plus aperçu indépendamment de cette variable.

Pour modifier la quantité de résultat valide que Claude reçoit en ligne, définissez plutôt le paramètre [`bashOutputMaxChars`](/docs/fr/settings-reference#bashoutputmaxchars), jusqu'à 128 000 caractères. Il dimensionne le plafond en ligne et la fenêtre de relecture ensemble, et Claude Code ignore alors `BASH_MAX_OUTPUT_LENGTH`. Nécessite Claude Code v2.1.261 ou ultérieur.

<h3 id="background-commands">
  Commandes en arrière-plan
</h3>

Pour les processus de longue durée tels que les serveurs de développement ou les compilations de surveillance, Claude peut définir `run_in_background: true` pour démarrer la commande en tant que tâche en arrière-plan et continuer à travailler pendant qu'elle s'exécute. Listez et arrêtez les tâches en arrière-plan avec `/tasks`. Après en avoir arrêté une là, ou à partir d'un client connecté tel que l'application de bureau, Claude continue au lieu d'attendre. Si un sous-agent a démarré la commande, c'est ce sous-agent qui continue.

Une commande qu'un [sous-agent au premier plan](/docs/fr/sub-agents#run-subagents-in-foreground-or-background) a démarrée s'arrête quand ce sous-agent donne sa réponse finale. Une commande que la conversation principale ou un sous-agent en arrière-plan a démarrée continue à s'exécuter après une réponse finale. En mode non interactif avec l'indicateur `-p`, [les commandes en arrière-plan se terminent peu après le résultat final de l'exécution](/docs/fr/headless#background-tasks-at-exit).

Quand une commande atteint son délai d'attente sans se terminer, Claude Code la déplace en arrière-plan au lieu de l'arrêter, sauf si la commande commence par `sleep`. Claude continue à travailler pendant que la commande continue. Claude Code applique les mêmes règles de durée de vie à une commande déplacée qu'à toute autre commande en arrière-plan, donc elle arrête toujours la commande d'un sous-agent au premier plan à la réponse finale de ce sous-agent. La définition de [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS=1`](/docs/fr/env-vars#variables) désactive la mise en arrière-plan automatique ainsi que le reste de la fonctionnalité de tâche en arrière-plan.

Le résultat d'une commande déplacée en arrière-plan indique ce qui s'est passé :

* Quand le délai d'attente déclenche le déplacement, le résultat le rapporte explicitement : `Command did not complete within its 120s timeout and was moved to the background`, avec les secondes correspondant au délai d'attente qui s'appliquait, suivi de l'ID de tâche et du chemin du fichier dans lequel la sortie est écrite.
* Un `cd`, `pushd`, `popd`, ou `chdir` à l'intérieur d'une commande qui est déplacée en arrière-plan ne se reporte jamais : le résultat indique `Session cwd remains <dir>; directory changes made by the backgrounded command do not apply to subsequent commands.`, donc Claude n'agit pas sur un changement de répertoire qui ne s'est pas produit.

<h3 id="memory-limit-on-linux-and-wsl">
  Limite de mémoire sur Linux et WSL
</h3>

Sur Linux et WSL, définissez [`CLAUDE_CODE_TOOL_MEMORY_LIMIT`](/docs/fr/env-vars#variables) sur une taille telle que `4G` pour limiter la mémoire que les commandes des outils Bash, PowerShell et [Monitor](#monitor-tool) peuvent utiliser, afin qu'une compilation qui s'échappe ne prenne pas la mémoire dont le reste de la session a besoin. Nécessite Claude Code v2.1.233 ou ultérieur. Avant v2.1.246, les commandes de l'outil Monitor s'exécutaient en dehors du plafond.

* Écrivez la taille comme un nombre d'octets ou avec un suffixe `K`, `M`, `G`, ou `T`. Définissez `0`, `off`, `false`, `no`, ou `none` pour désactiver le plafond. Claude Code ignore toute autre valeur qu'il ne peut pas lire comme une taille, telle que `4e9`.
* Claude Code compte toutes les commandes Bash, PowerShell et Monitor d'une session par rapport au plafond unique, pas chaque commande sur sa propre.
* Claude Code applique le plafond avec un cgroup de mémoire. Quand il ne peut pas configurer le cgroup, les commandes s'exécutent sans plafond, et le journal de débogage de `claude --debug` explique pourquoi.
* Après que le premier processus que Claude Code a démarré a activé le plafond, ou l'a désactivé en raison d'une valeur off ou d'une configuration de cgroup échouée, Claude Code maintient ce résultat jusqu'à ce que vous relancez. Pour appliquer une valeur modifiée ou supprimée, ou une configuration corrigée, lancez `claude` à nouveau.
* Quand les commandes ne peuvent pas rester sous le plafond, le noyau arrête une commande, et rien dans son résultat ne nomme le plafond.

Claude Code peut aussi compter d'autres types de processus qu'il démarre par rapport à la même limite. Définissez [`CLAUDE_CODE_TOOL_MEMORY_CGROUP_EXCLUDE`](/docs/fr/env-vars#variables) sur une liste séparée par des virgules des types à exempter du plafond ; Claude Code applique le plafond à chaque type qui n'est pas sur votre liste. Définissez-le sur `none` pour plafonner chaque type, ou sur `all-new` pour plafonner uniquement les commandes des outils Bash, PowerShell et Monitor. Nécessite Claude Code v2.1.246 ou ultérieur. Les types que vous pouvez nommer :

* `mcp`: serveurs [MCP](/docs/fr/mcp) locaux
* `lsp`: [serveurs de langage](#lsp-tool-behavior)
* `hooks`: commandes [hook](/docs/fr/hooks)
* `plugin` : commandes que les [plugins](/docs/fr/plugins/overview) exécutent
* `helper`: les propres commandes d'assistance de Claude Code, telles que `git`
* `agent`: processus Claude Code enfants, tels que [les coéquipiers agents](/docs/fr/agent-teams)

Quoi que vous listiez, ces règles s'appliquent :

* **Noms inconnus** : Claude Code ignore les noms qu'il ne reconnaît pas
* **Bash, PowerShell et Monitor** : Claude Code maintient les commandes des outils Bash, PowerShell et Monitor sous le plafond quoi que vous listiez
* **Variable non définie** : Claude Code prend l'ensemble des autres types plafonnés à partir de la configuration qu'Anthropic livre depuis le serveur, et cet ensemble peut changer au fil du temps, donc définissez la variable quand vous avez besoin d'un ensemble qui ne change pas
* **Hooks de permission-gating** : même avec chaque type plafonné, Claude Code exempte du plafond un hook qui peut bloquer ou modifier le résultat d'une action, et tout serveur MCP que ce hook appelle, donc le noyau arrêtant un hook de permission-gating ne peut pas permettre l'action qu'il bloquait

<h2 id="edit-tool-behavior">
  Comportement de l'outil Edit
</h2>

L'outil Edit effectue un remplacement de chaîne exact. Il prend une `old_string` et une `new_string` et remplace la première par la seconde. Il n'utilise pas d'expressions régulières ni de correspondance approximative.

Trois vérifications doivent réussir pour qu'une modification s'applique. Avant l'une d'elles, un chemin correspondant à une [règle de refus `Read`](/docs/fr/permissions#tool-specific-permission-rules) est refusé, y compris la création d'un nouveau fichier à cet endroit. Le refus nécessite Claude Code v2.1.208 ou une version ultérieure.

* **Read-before-edit** : Claude lit le fichier dans la conversation actuelle avant de le modifier, et une lecture interrompue avec un avis [`PARTIAL view`](#read-tool-behavior) ne compte pas. Claude Opus 4.6, Claude Haiku 4.5 et les modèles plus anciens nécessitent toujours la lecture. Les modèles plus récents peuvent modifier un fichier non lu lorsque la lecture ne nécessiterait pas une invite de permission et que l'outil Read est disponible.
* **Match** : `old_string` doit apparaître dans le fichier exactement tel qu'écrit. Une seule différence d'espace blanc ou d'indentation suffit à manquer la correspondance.
* **Unicité** : `old_string` doit apparaître exactement une fois. Lorsqu'il apparaît plus d'une fois, Claude fournit soit une chaîne plus longue avec suffisamment de contexte environnant pour identifier une occurrence, soit définit `replace_all: true` pour les remplacer tous.

Un fichier qui a changé sur le disque après la dernière lecture par Claude peut toujours être modifié lorsque `old_string` correspond exactement au contenu actuel et sans ambiguïté et que Claude Code peut lire le fichier sans invite. La correspondance avec le contenu actuel du fichier maintient la sécurité, et le résultat note que le fichier contient d'autres modifications afin que Claude le relise avant les modifications qui dépendent du contenu environnant. Dans tout autre cas, comme une `old_string` obsolète ou une qui correspond à plus d'une occurrence sans `replace_all`, Claude lit le fichier à nouveau avant la modification. La gestion assouplie des fichiers non lus et modifiés nécessite Claude Code v2.1.208 ou une version ultérieure ; avant cela, Claude Code refusait toute modification d'un fichier qu'il n'avait pas lu dans la conversation ou qui avait changé sur le disque après la lecture.

Afficher un fichier avec Bash satisfait également l'exigence read-before-edit lorsque la commande est `cat`, `nl`, `bat`, `batcat`, `head`, `tail`, `sed -n 'X,Yp'`, `grep`, `egrep`, `fgrep`, ou `rg` sur un seul fichier sans pipes ni redirections. La sortie redirigée et les autres commandes Bash ne comptent pas vers la vérification read-before-edit.

Afficher un fichier avec Bash affecte uniquement l'admissibilité de la modification, pas les permissions. Consultez [Règles de permission Read et Edit](/docs/fr/permissions#read-and-edit) pour savoir quelles commandes Bash vos règles de refus `Read` et `Edit` couvrent.

<h2 id="endconversation-tool-behavior">
  Comportement de l'outil EndConversation
</h2>

L'outil EndConversation termine la session actuelle. Claude l'utilise uniquement dans deux situations :

* en dernier recours contre une entrée abusive soutenue, après des tentatives de redirection de la conversation qui ont échoué et après un avertissement clair dans un message antérieur
* lorsque vous demandez explicitement de voir l'outil démontré et confirmez que vous souhaitez terminer la session

La frustration générale, les jurons ou une tâche qui se déroule mal ne sont pas qualifiants, tout comme les demandes de contenu nuisible, que Claude refuse au lieu de terminer la session. Claude Code suit la même approche que claude.ai, qui peut [terminer un sous-ensemble rare de conversations](https://www.anthropic.com/research/end-subset-conversations).

Après que Claude termine une session interactive, la session se verrouille. Les nouvelles invites et la plupart des commandes retournent `Claude ended this conversation. Start a new session (or /clear) to continue.`, et seules `/clear`, `/resume`, `/help`, `/exit` et `/feedback` continuent de fonctionner. Claude Code enregistre la fin dans la transcription de la session, donc reprendre une session terminée restaure le verrouillage ; l'historique de la session n'est pas supprimé.

Reprendre une session terminée en [mode non-interactif](/docs/fr/headless) avec l'indicateur `-p` génère une erreur et se termine avec le code 1, donc un script ne lit pas l'exécution terminée comme un succès.

L'outil ne demande jamais la permission, et les [hooks PreToolUse](/docs/fr/hooks#pretooluse) ne s'exécutent pas pour celui-ci. Tant que tout autre outil reste disponible, vous ne pouvez pas non plus le bloquer : les [règles de refus et de demande](/docs/fr/permissions#tool-specific-permission-rules) nommant `EndConversation` n'ont aucun effet, et ni `--disallowedTools` ni une liste `--tools` ne peuvent le supprimer. L'exemption est délibérée : l'outil ne fait rien d'autre que de terminer la conversation, ne lisant ni ne modifiant jamais les fichiers ou les données, et une sauvegarde de ce type ne tient que si la session à laquelle elle s'applique ne peut pas la désactiver. Lorsque vos règles de refus suppriment tous les autres outils et correspondent également à `EndConversation`, comme le fait `"*"`, Claude Code le supprime également plutôt que de le laisser comme seul outil, sauf si une règle d'autorisation nomme `EndConversation` explicitement. Une liste de refus qui supprime tous les autres outils sans correspondre à `EndConversation` le laisse en place.

Les [sous-agents](/docs/fr/sub-agents) n'obtiennent jamais l'outil. Les tâches en arrière-plan qui partagent la liste d'outils de la conversation principale le voient, mais l'appeler là ne termine rien.

L'outil n'apparaît que lorsque tous les éléments suivants sont vrais :

* **Version** : Claude Code v2.1.213 ou version ultérieure.
* **Modèle** : le modèle de la session est Claude Opus 4.8, Claude Sonnet 5, Claude Fable 5, ou une version ultérieure de l'une de ces familles.
* **Surface** : une session de terminal interactive, y compris une session `claude` dans le terminal intégré d'un IDE, ce qui est la façon dont le [plugin JetBrains](/docs/fr/jetbrains) l'exécute. Les autres surfaces n'incluent pas l'outil, comme :
  * les exécutions non-interactives `-p`
  * les sessions via les packages TypeScript et Python du [SDK Agent](/docs/fr/agent-sdk/overview)
  * le panneau de l'[extension VS Code](/docs/fr/vs-code), qui regroupe sa propre CLI
  * [GitHub Actions](/docs/fr/github-actions)
  * [Claude Code sur le web](/docs/fr/claude-code-on-the-web)
* **Mode de démarrage** : pas une session [`--bare`](/docs/fr/headless#start-faster-with-bare-mode). Le mode bare charge uniquement les outils shell et fichier, donc l'outil n'est jamais enregistré là.
* **Fournisseur** : non disponible sur [Amazon Bedrock](/docs/fr/amazon-bedrock), [Claude Platform on AWS](/docs/fr/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai), ou [Microsoft Foundry](/docs/fr/microsoft-foundry), ou sur les sessions connectées via une [passerelle cloud](/docs/fr/claude-apps-gateway).

<h2 id="glob-tool-behavior">
  Comportement de l'outil Glob
</h2>

L'outil Glob trouve des fichiers par motif de nom. Sur Windows, il fait partie de l'ensemble d'outils par défaut. Sur macOS, Linux et WSL, Claude Code laisse Glob et [Grep](#grep-tool-behavior) en dehors de l'ensemble d'outils par défaut, et Claude effectue des recherches avec `find` et `grep` via l'outil Bash à la place. Dans le shell de Claude, ces deux commandes exécutent des versions intégrées de `bfs` et `ugrep`, et les recherches atteignent vos hooks et règles de permission en tant qu'appels `Bash`.

Sur macOS, Linux et WSL, vous récupérez les outils Glob et Grep dans ces cas :

* Vous nommez `Glob` ou `Grep` dans [`--tools` ou `--allowedTools`](/docs/fr/cli-reference#cli-flags) lorsque vous démarrez la session, ou dans les options équivalentes du [SDK Agent](/docs/fr/agent-sdk/overview). Avec `--tools`, vous obtenez ceux que vous listez, et nommer l'un ou l'autre outil dans `--allowedTools` restaure les deux. Une règle d'autorisation dans un fichier de paramètres n'a pas cet effet.
* Une [règle de refus](/docs/fr/permissions#match-all-uses-of-a-tool) de permissions, l'indicateur `--disallowedTools`, ou [`--restricted`](/docs/fr/cli-reference#cli-flags) supprime `Bash` de la session.
* Un [sous-agent](/docs/fr/sub-agents#available-tools) liste `Glob` ou `Grep` dans son champ `tools` et laisse de côté `Bash`. Les outils listés reviennent pour ce sous-agent uniquement, ou pour la session entière lorsqu'il s'exécute en tant qu'agent de session principal via [`--agent`](/docs/fr/sub-agents#invoke-subagents-explicitly) ou le paramètre `agent`.

Glob supporte la syntaxe glob standard incluant `**` pour la correspondance de répertoire récursive :

* `**/*.js` correspond à tous les fichiers `.js` à n'importe quelle profondeur
* `src/**/*.ts` correspond à tous les fichiers `.ts` sous `src/`
* `*.{json,yaml}` correspond aux fichiers `.json` et `.yaml` dans le répertoire actuel

Les résultats sont triés par heure de modification et limités à 100 fichiers. Si la limite est atteinte, Claude voit un drapeau de troncature dans le résultat et peut affiner le motif.

Glob ne respecte pas `.gitignore` par défaut, donc il trouve les fichiers gitignorés aux côtés des fichiers suivis. Cela diffère de [Grep](#grep-tool-behavior), qui ignore les fichiers gitignorés. Pour faire respecter `.gitignore` à Glob, définissez `CLAUDE_CODE_GLOB_NO_IGNORE=false` avant de lancer Claude Code.

Claude Code décide de la permission pour un appel Glob avant de vérifier si le répertoire de recherche existe. Il exécute toujours la vérification de permission de lecture pour un `path` manquant en dehors des [répertoires de travail](/docs/fr/permissions#working-directories), donc une invite de permission pour un chemin ne signifie pas que le chemin existe.

Une valeur `pattern` ou `path` qui contient un octet nul retourne une erreur demandant à Claude de la supprimer.&#x20;

<h2 id="grep-tool-behavior">
  Comportement de l'outil Grep
</h2>

L'outil Grep recherche des motifs dans le contenu des fichiers. Où [Glob](#glob-tool-behavior) trouve des fichiers par nom, Grep trouve des lignes à l'intérieur d'eux. Sur macOS, Linux et WSL, Grep est absent par défaut dans les mêmes conditions que Glob. Voir [Comportement de l'outil Glob](#glob-tool-behavior) pour savoir quand les deux outils sont disponibles.

Grep est construit sur [ripgrep](https://github.com/BurntSushi/ripgrep) et utilise la syntaxe regex de ripgrep, pas grep POSIX. Les motifs qui incluent des métacaractères regex ont besoin d'échappement. Par exemple, trouver `interface{}` dans le code Go prend le motif `interface\{\}`.

Un motif, un glob, ou un type de fichier que ripgrep rejette retourne une erreur qui inclut le diagnostic de ripgrep, afin que Claude puisse corriger l'entrée et rechercher à nouveau. Avant la v2.1.208, Claude Code signalait une entrée rejetée comme `No files found` au lieu d'une erreur, même lorsque le texte recherché existait dans les fichiers cibles.

Trois modes de sortie contrôlent ce qui revient :

* `files_with_matches` : chemins de fichiers uniquement, pas de contenu de ligne. C'est la valeur par défaut.
* `content` : lignes correspondantes avec numéro de fichier et de ligne. Lorsque le paramètre `offset` de l'outil pointe au-delà de la dernière correspondance pour un motif qui a des correspondances, Grep retourne `No entries at this offset`, afin que Claude élargisse ou réinitialise l'offset au lieu de conclure que le motif ne correspond pas.
* `count` : nombre de correspondances par fichier, suivi d'un total sur tous les fichiers correspondants. Le total couvre chaque correspondance même lorsque les paramètres `head_limit` ou `offset` de l'outil tronquent les entrées par fichier listées. Avant la v2.1.208, le total ne sommait que les entrées listées.

Claude peut limiter les résultats par fichier avec le paramètre `glob`, tel que `**/*.tsx`, ou par langage avec le paramètre `type`, tel que `py` ou `rust`. Par défaut, les motifs correspondent dans une seule ligne. Claude peut définir `multiline: true` pour correspondre à travers les limites de ligne.

Grep respecte `.gitignore`, donc les fichiers gitignorés sont ignorés. Pour rechercher un fichier gitignore, Claude transmet son chemin directement.

Claude Code décide de la permission pour un appel Grep avant de vérifier si le `path` de recherche existe. Il exécute toujours la vérification de permission de lecture pour un `path` manquant en dehors des [répertoires de travail](/docs/fr/permissions#working-directories), donc une invite de permission pour un chemin ne signifie pas que le chemin existe.

<h2 id="lsp-tool-behavior">
  Comportement de l'outil LSP
</h2>

L'outil LSP donne à Claude l'intelligence du code à partir d'un serveur de langage en cours d'exécution. Après chaque modification de fichier, il signale automatiquement les erreurs de type et les avertissements afin que Claude puisse corriger les problèmes sans une étape de compilation séparée. Claude peut également l'appeler directement pour naviguer dans le code :

* Accéder à la définition d'un symbole
* Trouver toutes les références à un symbole
* Obtenir les informations de type à une position
* Lister les symboles dans un fichier
* Rechercher un symbole par nom dans l'espace de travail
* Trouver les implémentations d'une interface
* Tracer les hiérarchies d'appels

Claude Code maintient l'outil inactif jusqu'à ce que vous installiez un [plugin d'intelligence du code](/docs/fr/plugins/code-intelligence) pour votre langage. Dans les [sessions cloud](/docs/fr/claude-code-on-the-web), Claude Code ne démarre pas les serveurs de langage du plugin, donc l'outil LSP reste inactif là. Claude Code prend la configuration du serveur de langage à partir du plugin, et vous installez le binaire du serveur vous-même.

Claude Code retourne un résultat d'erreur pour chaque appel LSP sur un fichier dont il ne peut pas démarrer le serveur de langage.

<h2 id="monitor-tool">
  Outil Monitor
</h2>

L'outil Monitor permet à Claude de surveiller quelque chose en arrière-plan et de réagir quand cela change, sans mettre en pause la conversation. Demandez à Claude de :

* Suivre un fichier journal et signaler les erreurs au fur et à mesure qu'elles apparaissent
* Interroger une PR ou un travail CI et signaler quand son statut change
* Surveiller un répertoire pour les modifications de fichiers
* Suivre la sortie de tout script de longue durée que vous lui pointez
* Se connecter à un flux WebSocket et signaler chaque message à son arrivée

Pour la plupart des surveillances, Claude écrit un petit script, l'exécute en arrière-plan, et reçoit chaque ligne de sortie à son arrivée. Pour un serveur qui pousse déjà des événements, Claude peut ouvrir une [connexion WebSocket](#websocket-source) au lieu d'exécuter un script.

Vous continuez à travailler dans la même session et Claude intervient quand un événement arrive.

Chaque surveillance que Claude démarre a une échéance : 5 minutes par défaut, au maximum 30 minutes, et au maximum 10 minutes dans une exécution [non-interactive](/docs/fr/headless) donnée avec un seul prompt avec `-p`.

À l'échéance, la surveillance se termine. Claude reçoit un avis, il peut donc redémarrer la surveillance si elle est toujours nécessaire.

Arrêtez une surveillance en demandant à Claude de l'annuler ou en terminant la session. Quand vous arrêtez un [sous-agent](/docs/fr/sub-agents) qui a démarré des moniteurs, par exemple à partir de `/tasks`, ces moniteurs s'arrêtent avec lui.

Quand Monitor exécute une commande, il utilise les mêmes [règles de permission que Bash](/docs/fr/permissions#tool-specific-permission-rules), donc les motifs `allow` et `deny` que vous avez définis pour Bash s'appliquent ici aussi. Tandis que le [mode automatique](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) est actif, Claude Code met de côté les règles d'autorisation qui nomment `Monitor` lui-même, ainsi que les autres [règles d'autorisation larges qu'il abandonne](/docs/fr/permission-modes#how-the-classifier-evaluates-actions), donc le classificateur examine les commandes Monitor de la même manière qu'il examine les commandes Bash.

La [source WebSocket](#websocket-source) a sa propre invite d'approbation, que le classificateur décide également en mode automatique.

L'outil n'est pas disponible sur Amazon Bedrock, Google Cloud's Agent Platform, ou Microsoft Foundry. Il n'est également pas disponible quand `DISABLE_TELEMETRY` ou `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` est défini.

Les plugins peuvent déclarer des moniteurs qui démarrent automatiquement quand le plugin est actif, au lieu de demander à Claude de les démarrer. Voir [moniteurs de plugin](/docs/fr/plugins/components#monitors).

<h3 id="websocket-source">
  Source WebSocket
</h3>

<Note>
  La source WebSocket nécessite Claude Code v2.1.195 ou version ultérieure.
</Note>

Quand un serveur pousse déjà des événements sur un WebSocket, Claude peut s'y connecter directement au lieu d'écrire un script d'interrogation. Chaque type d'activité de socket devient soit un événement, soit termine la surveillance :

* **Messages texte** : chacun devient un événement, même quand le message s'étend sur plusieurs lignes.
* **Messages binaires** : non transmis. Claude reçoit une ligne d'espace réservé telle que `[binary frame, 512 bytes]` à la place.
* **Messages plus grands que 1 MiB** : la surveillance se termine, donc abonnez-vous à un flux filtré où il en existe un.
* **Fermeture de socket** : la surveillance se termine et Claude reçoit le code de fermeture.

Une surveillance WebSocket prend une entrée `ws` à la place de `command`, et un seul appel Monitor ne peut pas combiner les deux. L'entrée `ws` a deux champs :

| Champ       | Requis | Description                                                                                                                                                                     |
| :---------- | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `url`       | Oui    | Le point de terminaison auquel se connecter. Doit être une URL `ws://` ou `wss://` sans identifiants ou espaces intégrés, utilisant uniquement des caractères ASCII             |
| `protocols` | Non    | Noms de sous-protocole WebSocket à proposer lors de la poignée de main. Chaque entrée doit être un jeton de sous-protocole valide, et la liste ne peut pas contenir de doublons |

L'échéance `timeout_ms` s'applique également à une surveillance WebSocket : la surveillance se termine à l'échéance, et `TaskStop` l'annule plus tôt.

L'ouverture d'un WebSocket demande une approbation ; en [mode automatique](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) le classificateur décide à la place. L'invite n'offre pas d'option pour ignorer les futures invites pour le même hôte.

Claude Code refuse les URL qui pointent vers une adresse privée, link-local, ou de métadonnées cloud, y compris les noms d'hôte qui se résolvent en une. Il refuse également les hôtes dans `sandbox.network.deniedDomains`, et quand [`allowManagedDomainsOnly`](/docs/fr/settings-reference#sandbox-network-allowmanageddomainsonly) est défini dans les paramètres gérés, tout hôte en dehors de la liste d'autorisation gérée.

<h2 id="notebookedit-tool-behavior">
  Comportement de l'outil NotebookEdit
</h2>

NotebookEdit modifie un notebook Jupyter une cellule à la fois, ciblant les cellules par leur `cell_id`. Il n'effectue pas de remplacement de chaîne à travers le notebook comme le fait [Edit](#edit-tool-behavior) sur les fichiers simples.

Trois modes de modification contrôlent ce qui arrive à la cellule cible :

* `replace` : remplace la source de la cellule. C'est la valeur par défaut.
* `insert` : ajoute une nouvelle cellule après la cible. Sans `cell_id`, la nouvelle cellule va au début du notebook. Nécessite que `cell_type` soit défini à `code` ou `markdown`.
* `delete` : supprime la cellule cible.

Les règles de permission utilisent le format de chemin `Edit(...)`. Une règle comme `Edit(notebooks/**)` couvre les appels NotebookEdit sur les fichiers dans ce répertoire.

<h2 id="powershell-tool">
  Outil PowerShell
</h2>

L'outil PowerShell permet à Claude d'exécuter des commandes PowerShell en mode natif. Sous Windows, cela signifie que les commandes s'exécutent dans PowerShell au lieu d'être acheminées via Git Bash. La disponibilité de l'outil dépend de votre plateforme :

* **Windows sans Git Bash** : l'outil est activé automatiquement.
* **Windows avec Git Bash installé** : l'outil est activé par défaut pour les comptes claude.ai et Console ; définissez `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` pour l'activer dans les sessions Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry, ou `0` pour le désactiver.
* **Linux, macOS et WSL** : l'outil est optionnel.

Vos [hooks PreToolUse](/docs/fr/hooks#powershell) reçoivent la chaîne de commande de l'outil dans `tool_input.command`, avec les mêmes champs que l'outil Bash.

Faites correspondre `Bash|PowerShell` dans les hooks qui inspectent les commandes shell ; la [section d'entrée du hook PowerShell](/docs/fr/hooks#powershell) explique pourquoi la correspondance de `Bash` seul ne suffit pas.

<h3 id="enable-the-powershell-tool">
  Activer l'outil PowerShell
</h3>

Définissez `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` dans votre environnement ou dans `settings.json` :

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_USE_POWERSHELL_TOOL": "1"
  }
}
```

Sous Windows, définissez la variable à `0` pour désactiver l'outil. Sous Linux, macOS et WSL, l'outil nécessite PowerShell 7 ou version ultérieure : installez `pwsh` et assurez-vous qu'il se trouve dans votre `PATH`.

Sous Windows, Claude Code détecte automatiquement `pwsh.exe` pour PowerShell 7+ avec un repli sur `powershell.exe` pour PowerShell 5.1. Lorsque l'outil est activé, Claude traite PowerShell comme le shell principal. L'outil Bash reste disponible pour les scripts POSIX lorsque Git Bash est installé.

Claude Code lance PowerShell avec `-ExecutionPolicy Bypass` au niveau du processus uniquement, de sorte que les scripts `.ps1` et les importations de modules fonctionnent sur les installations Windows par défaut sans modifier la politique de la machine. Le contournement au niveau du processus ne remplace pas la stratégie de groupe `MachinePolicy` ou `UserPolicy`, de sorte que les politiques d'entreprise s'appliquent toujours. Pour respecter la politique d'exécution effective de la machine à la place, définissez `CLAUDE_CODE_POWERSHELL_RESPECT_EXECUTION_POLICY=1`.

<h3 id="shell-selection-in-settings-hooks-and-skills">
  Sélection du shell dans les paramètres, les hooks et les skills
</h3>

Trois paramètres supplémentaires contrôlent l'endroit où PowerShell est utilisé :

* `"defaultShell": "powershell"` dans [`settings.json`](/docs/fr/settings-reference#all-settings) : achemine les commandes interactives `!` via PowerShell. Nécessite que l'outil PowerShell soit activé.
* `"shell": "powershell"` sur les [hooks de commande](/docs/fr/hooks#command-hook-fields) individuels : exécute ce hook dans PowerShell. Les hooks lancent PowerShell directement, de sorte que cela fonctionne indépendamment de `CLAUDE_CODE_USE_POWERSHELL_TOOL`.
* `shell: powershell` dans le [frontmatter du skill](/docs/fr/skills#frontmatter-reference) : exécute les blocs `` !`command` `` dans PowerShell. Nécessite que l'outil PowerShell soit activé.

Le même comportement de réinitialisation du répertoire de travail de la session principale décrit dans la section de l'outil Bash s'applique aux commandes PowerShell, y compris la variable d'environnement `CLAUDE_BASH_MAINTAIN_PROJECT_WORKING_DIR`.

À partir de la v2.1.196, le code de sortie 1 de `grep`, `rg`, `egrep`, `fgrep`, `findstr` et `git grep` signifie aucune correspondance. Le code de sortie 1 de `git diff` signifie que des différences existent. Aucun de ces résultats n'est signalé à Claude comme un échec de commande. Pour `robocopy`, les codes de sortie 0 à 7 sont des résultats informationnels, tels que des fichiers copiés ou des fichiers supplémentaires détectés. Les codes de sortie 8 ou supérieurs comptent comme des échecs.

<h3 id="windows-encoding-and-exit-codes">
  Encodage Windows et codes de sortie
</h3>

Sous Windows, les comportements d'encodage PowerShell et de code de sortie suivants nécessitent Claude Code v2.1.214 ou version ultérieure :

* La redirection avec `>` et `>>` écrit des fichiers UTF-8 sur PowerShell 5.1
* Claude Code encode le texte acheminé vers l'entrée standard d'une commande native en UTF-8
* Claude Code capture la sortie d'erreur sans séquences d'échappement ANSI
* Une commande dont le processus enfant attend sur l'entrée standard reçoit une fin de fichier au lieu de se bloquer
* Le code de sortie 1 de `where.exe` signifie aucune correspondance, et de `fc.exe` et `diff.exe` signifie que les fichiers diffèrent, de sorte que lorsque la commande produit une sortie, Claude Code traite ce code de sortie comme une réponse négative valide plutôt qu'une erreur de commande. Claude Code signale toujours une forme silencieuse, telle que `where.exe /Q` ou une redirection vers `$null`, comme un échec au code de sortie 1

Avant la v2.1.214, `>` sur PowerShell 5.1 écrivait des fichiers UTF-16LE, l'entrée acheminée non-ASCII arrivait sous la forme `?`, et les scripts Python pouvaient planter avec une `UnicodeEncodeError` lors de l'impression de caractères non-ASCII.

<h3 id="preview-limitations">
  Limitations de l'aperçu
</h3>

L'outil PowerShell présente les limitations connues suivantes pendant l'aperçu :

* Les profils PowerShell ne sont pas chargés
* Sous Windows, le sandboxing n'est pas pris en charge

<h2 id="read-tool-behavior">
  Comportement de l'outil Read
</h2>

L'outil Read prend un chemin de fichier et retourne le contenu avec les numéros de ligne. Claude est configuré pour toujours passer des chemins absolus.

Par défaut, Read retourne le fichier depuis le début. Quand une lecture de fichier complet dépasse la limite de tokens, Read retourne la première page avec un avis `PARTIAL view` qui indique à Claude combien du fichier il a reçu et comment lire plus avec `offset` et `limit`. Une lecture qui passe un `offset` ou `limit` explicite et dépasse toujours la limite de tokens retourne une erreur.

Une lecture avec un `limit` explicite s'arrête dès que les lignes sélectionnées dépassent ce que la limite de tokens pourrait jamais contenir et retourne une erreur sans charger le reste de la plage. L'erreur indique à Claude d'utiliser un `limit` plus petit, ou de rechercher du contenu spécifique avec [Grep](#grep-tool-behavior) à la place quand une seule ligne est aussi grande. Avant v2.1.208, Claude Code chargeait toute la plage en mémoire avant de la rejeter, donc lire un fichier avec une seule ligne extrêmement longue pouvait le faire manquer de mémoire.

Lire un fichier vide retourne un avis que le fichier existe mais que son contenu est vide, et un `offset` au-delà de la dernière ligne retourne un avis donnant le nombre de lignes du fichier. Avant v2.1.208, lire un fichier vide retournait l'avis de fin de fichier à la place.

Read gère plusieurs types de fichiers au-delà du texte brut :

* **Images** : PNG, JPG et autres formats d'image sont retournés comme contenu visuel que Claude peut voir, pas comme des octets bruts. Claude Code redimensionne et récompresse les grandes images pour s'adapter aux limites de taille d'image du modèle avant de les envoyer, donc Claude peut voir une version réduite d'une grande capture d'écran. À partir de v2.1.196, une image qui est toujours plus grande que 500 KB après ce redimensionnement est réencodée en JPEG à qualité réduite avec ses dimensions en pixels inchangées. Si Claude manque des détails fins au niveau des pixels dans une grande image, demandez-lui de d'abord recadrer la région d'intérêt, par exemple avec ImageMagick via Bash.
* **PDFs** : Claude lit les fichiers `.pdf` courts en entier. Pour les PDFs plus longs que 10 pages, il lit par plages avec un paramètre `pages`, tel que `« 1-5 »`, jusqu'à 20 pages à la fois.
* **Notebooks Jupyter** : les fichiers `.ipynb` retournent toutes les cellules avec leurs résultats, y compris le code, le markdown et les visualisations. Claude Code refuse de lire un fichier notebook de plus de 100 MB ; l'erreur indique à Claude comment lire une portion du notebook à la place, comme une tranche de cellules, avec une commande shell.

Read lit uniquement les fichiers, pas les répertoires. Claude liste le contenu des répertoires avec une commande shell telle que `ls`.

<h2 id="sendfeedback-tool-behavior">
  Comportement de l'outil SendFeedback
</h2>

Les commentaires rédigés par Claude constituent un rapport de commentaires sur Claude Code que Claude rédige pour vous. Cela nécessite Claude Code v2.1.238 ou une version ultérieure. Claude Code enregistre chaque brouillon sur votre machine sous `~/.claude/feedback/drafts/`, et rien ne parvient à Anthropic tant que vous ne l'envoyez pas. Claude rédige un brouillon avec l'outil SendFeedback quand :

* Un outil ou une commande continue d'échouer
* Il ne peut pas vous aider avec quelque chose que vous avez demandé
* Vous signalez une erreur qu'il a commise, ou il en remarque une
* Vous lui demandez de soumettre un commentaire

<h3 id="what-you-see-when-claude-drafts">
  Ce que vous voyez quand Claude rédige
</h3>

Après que Claude mette un brouillon en file d'attente, vous voyez une carte au-dessus de votre invite avec le titre du brouillon. Appuyez sur `1` pour examiner le brouillon, appuyez sur `2` deux fois pour l'envoyer tel quel, ou appuyez sur `0` pour le rejeter. Un brouillon rejeté reste dans votre file d'attente. Après avoir rejeté une carte, Claude Code vous demande si vous souhaitez désactiver les commentaires rédigés par Claude. Il cesse de poser la question une fois que vous avez refusé deux fois.

Par défaut, vous voyez au maximum trois cartes dans une session ; Anthropic peut ajuster cette limite depuis le serveur sans publication. Après la limite, et chaque fois que vous définissez [`feedbackDrafts`](/docs/fr/settings-reference#feedbackdrafts) sur `quiet`, vous voyez uniquement un nombre de brouillons en file d'attente dans le pied de page de l'invite.

<h3 id="review-and-edit-a-draft">
  Examiner et modifier un brouillon
</h3>

Exécutez `/feedback` sans argument pour ouvrir votre file d'attente. Elle répertorie tous les brouillons en file d'attente de toutes vos sessions, y compris les brouillons dont vous avez rejeté les cartes ou que vous n'avez jamais vus. Sélectionnez un brouillon pour l'ouvrir pour examen, où vous pouvez :

* Modifier le titre, la zone et les détails
* Définir **Envoyer la transcription** sur `yes` ou `no`. Quand la transcription de la session où Claude a mis le brouillon en file d'attente est toujours disponible, elle commence à `yes`, ce qui envoie cette conversation à Anthropic ; `no` envoie uniquement le rapport
* Envoyer le brouillon, le supprimer ou le laisser dans la file d'attente pour plus tard

Pour rédiger un rapport vous-même à la place, appuyez sur `w` pour la boîte de dialogue de commentaires standard. `/feedback` avec du texte après, et `/bug`, ouvrent cette boîte de dialogue directement.

<h3 id="send-a-draft">
  Envoyer un brouillon
</h3>

Quand vous envoyez un brouillon, Claude Code le soumet de la même manière qu'un rapport `/feedback`, avec la même [rétention](/docs/fr/data-usage#feedback-using-the-%2Ffeedback-command), et supprime le brouillon de votre machine. Quand vous envoyez depuis la carte, elle affiche `✓ Sent` ; quand vous envoyez depuis la file d'attente, elle se ferme avec un ID de reçu.

Le rapport contient :

* Votre titre, zone et détails
* Les informations d'environnement, telles que votre version de Claude Code, votre système d'exploitation et votre modèle
* Les ID des demandes API récentes
* La transcription de la conversation, quand vous avez laissé **Envoyer la transcription** sur `yes` dans l'écran d'examen. L'envoi depuis la carte n'inclut jamais la transcription

Claude Code conserve votre répertoire de travail dans le brouillon local afin de pouvoir trouver la transcription, et n'envoie pas le répertoire.

Dans les [organisations avec rétention zéro des données](/docs/fr/zero-data-retention#features-disabled-under-zdr), Claude Code laisse l'outil de côté, comme il le fait pour `/feedback`. Si une session dans une telle organisation offre toujours l'outil, les brouillons restent sur votre machine, et l'envoi échoue avec `Feedback collection is not available for organizations with custom data retention policies.`

<h3 id="discard-or-keep-a-draft">
  Supprimer ou conserver un brouillon
</h3>

Quand vous supprimez un brouillon, Claude Code le supprime de votre machine. Un brouillon que vous laissez dans la file d'attente expire après 30 jours, ou après [`cleanupPeriodDays`](/docs/fr/settings-reference#cleanupperioddays) quand c'est plus court. La file d'attente contient 10 brouillons dans toutes vos sessions, et quand Claude met un onzième en file d'attente, Claude Code supprime le plus ancien. Quand vous exécutez `/exit` avec des brouillons de la session toujours dans la file d'attente, Claude Code vous demande si vous souhaitez les examiner ou les supprimer avant de quitter.

<h3 id="turn-claude-drafted-feedback-off">
  Désactiver les commentaires rédigés par Claude
</h3>

Définissez **Claude-drafted feedback** sur `off` dans `/config`, ce qui écrit le paramètre [`feedbackDrafts`](/docs/fr/settings-reference#feedbackdrafts), ou définissez [`CLAUDE_CODE_SEND_FEEDBACK=0`](/docs/fr/env-vars) pour une session. Avec l'un ou l'autre, Claude ne peut pas mettre de brouillons en file d'attente. Pour continuer à rédiger sans cartes, définissez `feedbackDrafts` sur `quiet` à la place. Les administrateurs peuvent définir `feedbackDrafts` dans les [paramètres gérés](/docs/fr/managed-settings), ce qui prend précédence sur votre propre paramètre.

<h3 id="sessions-without-claude-drafted-feedback">
  Sessions sans commentaires rédigés par Claude
</h3>

Claude Code inclut l'outil dans les sessions de terminal interactives sur votre propre machine qui utilisent l'API Claude plutôt qu'un fournisseur cloud. Il laisse l'outil de côté dans :

* Les exécutions non interactives `-p` et les sessions [Agent SDK](/docs/fr/agent-sdk/overview), qui n'ont pas d'écran pour examiner la file d'attente
* Les sessions cloud telles que [Claude Code sur le web](/docs/fr/claude-code-on-the-web), qui ne peuvent pas écrire dans la file d'attente sur votre machine
* Les sessions sur [Amazon Bedrock](/docs/fr/amazon-bedrock), [Claude Platform on AWS](/docs/fr/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai), ou [Microsoft Foundry](/docs/fr/microsoft-foundry)
* Les sessions où vous avez défini [`CLAUDE_CODE_SEND_FEEDBACK=0`](/docs/fr/env-vars) ou [`DISABLE_FEEDBACK_COMMAND=1`](/docs/fr/env-vars), défini `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` sur une valeur non vide, ou désactivé [feature-flag fetching](/docs/fr/env-vars#features-that-need-feature-flag-fetching)
* Les organisations qui ont désactivé les commentaires sur les produits, et les [organisations avec rétention zéro des données](/docs/fr/zero-data-retention#features-disabled-under-zdr)

<h2 id="task-tool-availability">
  Disponibilité de l'outil Task
</h2>

Les outils de suivi des tâches, `TaskCreate`, `TaskGet`, `TaskUpdate`, `TaskList` et `TodoWrite`, sont disponibles par défaut uniquement sur les modèles Claude 3.x, Opus 4 à 4.7, Sonnet 4 à 4.6 et Haiku 4.5. Partout où les outils sont disponibles, vous obtenez les quatre outils Task, ou `TodoWrite` uniquement lorsque vous définissez [`CLAUDE_CODE_ENABLE_TASKS=0`](/docs/fr/env-vars).

Sur tous les autres modèles, Claude Code exclut les outils sauf si vous les activez explicitement. Il en va de même pour un ID de modèle que Claude Code ne reconnaît pas, comme un nom de modèle personnalisé servi via une [passerelle LLM](/docs/fr/llm-gateway). Sur les modèles plus récents, Claude garde une trace du travail multi-étapes sans liste de contrôle écrite, et les définitions et rappels des outils consomment du contexte. Sans les outils, Claude n'ajoute rien à la [liste des tâches](/docs/fr/interactive-mode#task-list) pendant qu'il travaille.

Si vous souhaitez utiliser ces outils sur un modèle qui ne les possède pas par défaut, faites l'une des choses suivantes :

* Exportez [`CLAUDE_CODE_ENABLE_TODO_TOOLS=1`](/docs/fr/env-vars) avant de démarrer Claude Code, par exemple `CLAUDE_CODE_ENABLE_TODO_TOOLS=1 claude`. Claude Code fournit alors les mêmes outils sur chaque modèle et chaque fournisseur
* Nommez l'un des outils dans [`--allowedTools`](/docs/fr/cli-reference#cli-flags), par exemple `claude --allowedTools TaskCreate`
* Listez les outils dans [`--tools`](/docs/fr/cli-reference#cli-flags), ce qui restreint les outils intégrés de la session à ceux qu'il nomme. Incluez les outils que vous souhaitez utiliser aux côtés des autres outils intégrés que vous utilisez
* Dans l'Agent SDK, les options [`allowedTools` et `tools`](/docs/fr/agent-sdk/todo-tracking#model-availability) fonctionnent de la même manière que les deux drapeaux

Dans les [sessions en arrière-plan](/docs/fr/agent-view) et dans [Claude Code sur le web](/docs/fr/claude-code-on-the-web), Claude Code fournit les mêmes outils sur chaque modèle, listés ou non.

Claude Code donne à un sous-agent les outils uniquement lorsque votre session les possède, même lorsque le sous-agent exécute un modèle différent. Un coéquipier [équipe d'agents](/docs/fr/agent-teams) en processus suit votre session de la même manière, tandis qu'un coéquipier dans son propre [volet divisé](/docs/fr/agent-teams#choose-a-display-mode) s'exécute en tant que processus Claude Code séparé, donc son propre modèle décide. Sans les outils Task, un agent coordonne avec son équipe par le biais de messages au lieu de la [liste des tâches partagée](/docs/fr/agent-teams#assign-and-claim-tasks).

L'ensemble par défaut décrit ici s'applique dans Claude Code v2.1.268 et versions ultérieures.

<h2 id="webfetch-tool-behavior">
  Comportement de l'outil WebFetch
</h2>

WebFetch prend une URL et une invite décrivant ce qu'il faut extraire. Il récupère la page, convertit la réponse en Markdown lorsque le serveur retourne du HTML, et exécute l'invite contre le contenu en utilisant un modèle petit et rapide. Pour la plupart des récupérations, Claude reçoit la réponse de ce modèle, pas la page brute. L'étape de conversion n'est pas configurable.

Cela rend WebFetch lossy par conception. L'invite d'extraction détermine ce qui atteint Claude, donc un résultat qui dit qu'une page ne mentionne pas quelque chose peut seulement signifier que l'invite ne l'a pas demandé. Demandez à Claude de récupérer à nouveau avec une invite plus spécifique, ou utilisez `curl` via Bash pour la page non traitée.

Quelques comportements façonnent la réponse que Claude reçoit :

* WebFetch refuse `localhost` et tout autre nom d'hôte sans point, comme un nom d'intranet nu, avant de faire une demande. L'[erreur qu'il retourne](/docs/fr/errors#webfetch-cannot-fetch-localhost) dit à Claude d'atteindre les serveurs locaux avec `curl` via Bash à la place.
* Les URL HTTP sont automatiquement mises à niveau vers HTTPS.
* Les grandes pages sont tronquées à une limite de caractères fixe avant le traitement.
* WebFetch met en cache chaque réponse pendant 15 minutes par défaut, donc les récupérations répétées de la même URL reviennent rapidement. Sur Claude Code v2.1.233 ou ultérieur, définissez [`CLAUDE_CODE_WEBFETCH_CACHE_TTL_MS`](/docs/fr/env-vars#variables) pour modifier la durée pendant laquelle WebFetch conserve chaque réponse.
* Une page qui n'a pas terminé le téléchargement dans les cinq minutes, y compris les redirections que WebFetch suit, échoue avec une erreur de délai. Sur Claude Code v2.1.268 ou ultérieur, définissez [`CLAUDE_CODE_WEBFETCH_DEADLINE_MS`](/docs/fr/env-vars#variables) pour modifier la limite, ou à `0` pour la supprimer.
* Lorsqu'une URL redirige vers un hôte différent, WebFetch retourne un résultat texte qui nomme l'URL d'origine et la cible de redirection au lieu de la suivre. Claude récupère ensuite la nouvelle URL avec un deuxième appel WebFetch.
* Lorsque l'étape d'extraction atteint une API surchargée, Claude Code la réessaie avec backoff ; une récupération qui échoue toujours retourne un résultat d'erreur. Avant v2.1.212, le texte d'erreur API pouvait atteindre Claude comme s'il s'agissait du contenu de la page extraite.

En modes Manual et `acceptEdits` [permission](/docs/fr/permission-modes), WebFetch demande avant de récupérer, sauf pour les domaines que vos [règles de permission](/docs/fr/permissions#manage-permissions) permettent ou refusent déjà et un ensemble intégré de domaines de documentation préapprouvés qui récupèrent sans invite. Quelles que soient les règles que vous autorisez, une récupération passe également d'abord la [vérification de sécurité du domaine WebFetch](/docs/fr/data-usage#webfetch-domain-safety-check) ; cette section couvre ce que la vérification envoie et le paramètre qui la contourne. L'invite offre trois options :

* **Oui** : approuve cette récupération uniquement. L'appel WebFetch suivant demande à nouveau, même pour le même domaine.
* **Oui, et ne me demande plus pour `<domain>`** : approuve la récupération et enregistre une règle d'autorisation `WebFetch(domain:...)` pour ce domaine dans `.claude/settings.local.json` pour ce référentiel. Voir [comment les approbations enregistrées persistent](/docs/fr/permissions#permission-system). Lorsque votre organisation définit [`allowManagedPermissionRulesOnly`](/docs/fr/permissions#managed-only-settings), Claude Code masque cette option.
* **Non, et dis à Claude ce qu'il faut faire différemment** : rejette la récupération.

Pour autoriser un domaine à l'avance sans invite, ajoutez une règle d'autorisation comme `WebFetch(domain:example.com)` ; `WebFetch(domain:*)` autorise chaque domaine. Les modes de permission `auto` et `bypassPermissions` [permission modes](/docs/fr/permissions#permission-modes) ignorent l'invite, sauf pour un domaine qu'une règle `ask` explicite correspond.

Une règle `WebFetch(domain:...)` explicite dans `deny`, `ask`, ou `allow` prend précédence sur l'ensemble préapprouvé, vous pouvez donc bloquer un domaine préapprouvé ou exiger une invite pour celui-ci.

WebFetch définit un en-tête `User-Agent` commençant par `Claude-User`, et un en-tête `Accept` qui préfère Markdown à HTML afin que les serveurs qui supportent la négociation de contenu puissent retourner Markdown directement.

Les commandes en sandbox n'héritent pas de l'ensemble intégré de domaines de documentation préapprouvés de WebFetch. Pour permettre à une commande en sandbox d'atteindre un domaine sans invite, ajoutez le domaine à [`allowedDomains`](/docs/fr/settings-reference#sandbox-network-alloweddomains) ou autorisez-le avec une règle `WebFetch(domain:...)`, que le [sandbox honore également](/docs/fr/sandboxing#network-isolation). WebFetch ne lit jamais la liste d'autorisation du sandbox en retour, donc ajouter un domaine à un sandbox ou à une liste d'autorisation du réseau de l'organisation n'empêche pas WebFetch de demander pour celui-ci.

<h2 id="websearch-tool-behavior">
  Comportement de l'outil WebSearch
</h2>

WebSearch exécute une requête sur le backend de [recherche web](https://platform.claude.com/docs/en/agents-and-tools/tool-use/web-search-tool) d'Anthropic et retourne les titres et les URL des résultats. Il ne récupère pas les pages de résultats. Pour lire une page que Claude trouve dans les résultats de recherche, il effectue un suivi avec [WebFetch](#webfetch-tool-behavior).

L'outil peut émettre jusqu'à huit recherches backend par appel, en affinant la recherche en interne avant de retourner les résultats. Claude peut limiter les résultats avec `allowed_domains` pour inclure uniquement certains hôtes, ou `blocked_domains` pour les exclure. Les deux listes ne peuvent pas être combinées dans un seul appel.

Lorsque la requête de recherche atteint une API surchargée, Claude Code la réessaie avec backoff ; un appel qui échoue toujours retourne un résultat d'erreur. Avant la v2.1.212, le texte d'erreur de l'API pouvait atteindre Claude comme s'il s'agissait de résultats de recherche.

Les règles de permission WebSearch ne prennent pas de spécificateur. Une entrée `WebSearch` simple dans `allow` ou `deny` est la seule forme.

Le backend de recherche n'est pas configurable. Pour effectuer une recherche avec un fournisseur différent, ajoutez un [serveur MCP](/docs/fr/mcp) qui expose un outil de recherche.

<Note>
  WebSearch est disponible sur l'API Claude et [Claude Platform sur AWS](/docs/fr/claude-platform-on-aws). Sur Microsoft Foundry, cela nécessite un [déploiement hébergé sur Anthropic](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options) : les déploiements hébergés sur Azure ne supportent pas les outils côté serveur, donc l'appel WebSearch échoue. Sur Agent Platform de Google Cloud, cela fonctionne avec Claude 4 et les modèles ultérieurs, y compris Opus, Sonnet et Haiku. Amazon Bedrock n'expose pas l'outil de recherche web côté serveur.
</Note>

<h3 id="session-search-limit">
  Limite de recherche de session
</h3>

Une session peut effectuer au maximum 200 appels WebSearch, comptabilisés dans la conversation principale et chaque [sous-agent](/docs/fr/sub-agents) qu'elle génère, donc les recherches effectuées par les fan-outs de recherche parallèle comptent contre la même limite. La limite nécessite Claude Code v2.1.212 ou ultérieur. Lorsque Claude atteint la limite, les appels ultérieurs retournent un avis indiquant à Claude de continuer avec les informations qu'il a déjà rassemblées, plutôt qu'une erreur qui inviterait à une nouvelle tentative. Vous ne voyez pas l'avis : un appel plafonné apparaît dans la conversation comme une recherche qui n'a rien fait, et si Claude a besoin de plus de recherches, l'avis lui indique de vous demander de relever la limite.

Définissez la variable d'environnement [`CLAUDE_CODE_MAX_WEB_SEARCHES_PER_SESSION`](/docs/fr/env-vars) pour modifier le plafond ; elle accepte un nombre entier positif, donc le plafond peut être augmenté mais pas désactivé. L'exécution de [`/clear`](/docs/fr/commands#all-commands) réinitialise le compteur. Si un travail qui peut toujours générer des [sous-agents](/docs/fr/sub-agents) survit au clear, comme un workflow en cours d'exécution, le compteur est reporté à la place.

<h2 id="write-tool-behavior">
  Comportement de l'outil Write
</h2>

L'outil Write crée un nouveau fichier ou remplace un fichier existant par le contenu complet fourni. Il n'ajoute pas ou ne fusionne pas.

Le fait que Claude doive lire un fichier existant dans la conversation actuelle avant de le remplacer dépend du modèle et du fichier :

* Claude Opus 4.6, Claude Haiku 4.5 et les modèles plus anciens exigent toujours la lecture, donc une Write vers un fichier existant non lu échoue avec une erreur.
* Les modèles plus récents peuvent remplacer un fichier qu'ils n'ont jamais lu cette session dans les mêmes conditions que [read-before-edit](#edit-tool-behavior) : le lire ne nécessiterait pas une invite de permission et l'outil Read est disponible.
* Les notebooks Jupyter et les fichiers que Claude a lus partiellement avec un avis [`PARTIAL view`](#read-tool-behavior) exigent la lecture sur tous les modèles.

Cette contrainte ne s'applique pas aux nouveaux fichiers. Avant v2.1.228, tous les modèles exigeaient la lecture avant de remplacer un fichier existant.

Afficher le fichier avec Bash satisfait également cette exigence selon les mêmes règles décrites dans [Edit tool behavior](#edit-tool-behavior).

Pour les modifications partielles d'un fichier existant, Claude utilise Edit au lieu de Write.

<h2 id="check-which-tools-are-available">
  Vérifier quels outils sont disponibles
</h2>

Votre ensemble d'outils exact dépend de votre fournisseur, de votre plateforme et de vos paramètres. Pour vérifier ce qui est chargé dans une session en cours d'exécution, demandez directement à Claude :

```text theme={null}
What tools do you have access to?
```

Claude donne un résumé conversationnel. Pour les noms d'outils MCP exacts, exécutez `/mcp`.

<Note>
  L'[outil advisor](/docs/fr/advisor) est un [outil serveur](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool) que l'API exécute, plutôt qu'un outil que Claude Code implémente. Il n'a pas de nom que vous pouvez référencer dans les règles de permission ou les correspondances de hook.
</Note>

<h2 id="see-also">
  Voir aussi
</h2>

* [Serveurs MCP](/docs/fr/mcp) : ajouter des outils personnalisés en connectant des serveurs externes
* [Permissions](/docs/fr/permissions) : système de permission, syntaxe des règles, et motifs spécifiques aux outils
* [Subagents](/docs/fr/sub-agents) : configurer l'accès aux outils pour les subagents
* [Hooks](/docs/fr/hooks-guide) : exécuter des commandes personnalisées avant ou après l'exécution des outils
