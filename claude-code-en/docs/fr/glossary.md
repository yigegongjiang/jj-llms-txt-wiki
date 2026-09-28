> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Glossaire

> Définitions de la terminologie Claude Code. Découvrez ce que signifient agentic loop, compaction, CLAUDE.md, hooks, subagents, MCP et autres concepts fondamentaux.

Ce glossaire définit la terminologie de Claude Code. Chaque entrée renvoie à la page où le concept est couvert en détail. Pour les concepts au niveau du modèle comme les tokens, la température et RAG, consultez le [glossaire de la plateforme](https://platform.claude.com/docs/fr/about-claude/glossary). Pour les termes Claude Desktop tels que desktop extension, MCPB et DXT, consultez le [Centre d'aide Claude](https://support.claude.com/).

<h2 id="a">
  A
</h2>

<h3 id="agents-md">
  AGENTS.md
</h3>

Un fichier markdown d'instructions de projet que vous écrivez pour les agents de codage IA. Si votre référentiel en contient un et aucun [CLAUDE.md](#claude-md), Claude le lit comme vos instructions de projet sans que vous ayez besoin d'ajouter un deuxième fichier. Vous pouvez modifier le paramètre **Project instructions** dans `/config` pour que Claude lise les deux fichiers ou seulement `CLAUDE.md`. La lecture directe de `AGENTS.md` nécessite Claude Code v2.1.277 ou ultérieur. Dans certaines sessions, Claude [ne peut pas lire `AGENTS.md`](/docs/fr/memory#when-agents-md-support-is-unavailable), donc [importez-le à partir d'un `CLAUDE.md`](/docs/fr/memory#share-one-file-with-other-coding-tools) à la place.

En savoir plus : [AGENTS.md](/docs/fr/memory#agents-md)

<h3 id="agent-teams">
  Agent teams
</h3>

Plusieurs sessions Claude Code indépendantes coordonnées par un chef d'équipe, avec une liste de tâches partagée et une messagerie pair à pair. Contrairement aux [subagents](#subagent), qui s'exécutent au sein d'une seule session et ne rendent compte qu'au parent, les coéquipiers ont chacun leur propre fenêtre de contexte et vous pouvez interagir directement avec n'importe lequel d'entre eux. Les agent teams sont expérimentaux et désactivés par défaut ; voir [Enable agent teams](/docs/fr/agent-teams#enable-agent-teams).

En savoir plus : [Run agent teams](/docs/fr/agent-teams)

<h3 id="agentic-coding">
  Agentic coding
</h3>

Un flux de travail où l'IA peut lire des fichiers, exécuter des commandes et apporter des modifications de manière autonome pendant que vous regardez, redirigez ou vous éloignez, par opposition aux assistants basés sur le chat qui répondent uniquement avec du texte que vous devez appliquer vous-même. Claude Code est agentic car il dispose de [tools](#tool) qui lui permettent d'agir, pas seulement de conseiller.

En savoir plus : [How Claude Code works](/docs/fr/how-claude-code-works)

<h3 id="agentic-harness">
  Agentic harness
</h3>

Les outils, la gestion du contexte et l'environnement d'exécution qui transforment un modèle de langage en un agent de codage capable. Claude Code est le harness ; Claude est le modèle à l'intérieur. Le harness fournit l'accès aux fichiers, l'exécution du shell, la gestion des permissions, le chargement de la mémoire et la boucle qui enchaîne les actions ensemble.

En savoir plus : [How Claude Code works](/docs/fr/how-claude-code-works)

<h3 id="agentic-loop">
  Agentic loop
</h3>

Le cycle que Claude parcourt pour chaque tâche : rassembler le contexte, agir, vérifier les résultats et répéter jusqu'à ce que ce soit fait. Chaque utilisation d'outil retourne des informations qui informent l'étape suivante. Vous pouvez interrompre la boucle à tout moment pour rediriger. La plupart des points d'extension, y compris les [hooks](#hook), les [skills](#skill) et [MCP](#mcp-model-context-protocol), se connectent à des phases spécifiques de cette boucle.

En savoir plus : [How Claude Code works](/docs/fr/how-claude-code-works#the-agentic-loop)

<h3 id="artifact">
  Artifact
</h3>

Une page web en direct et interactive que Claude Code publie à partir de votre session vers une URL privée sur claude.ai, afin que vous puissiez voir la sortie visuellement ou la partager au lieu de lire du texte de terminal. La page se met à jour sur place lorsque la session est republié. Les artifacts que vous créez à partir de Claude Code apparaissent dans la même galerie que les artifacts créés dans les conversations claude.ai. Le partage dépend de votre plan : sur Pro et Max, un lien public que n'importe qui peut ouvrir ; sur Team et Enterprise, le partage au sein de votre organisation, plus les liens publics une fois qu'un propriétaire les active.

En savoir plus : [Share session output as artifacts](/docs/fr/artifacts)

<h3 id="auto-memory">
  Auto memory
</h3>

Des notes que Claude écrit pour lui-même en fonction de vos corrections et préférences, stockées par référentiel git sous `~/.claude/projects/`. Tous les worktrees du même référentiel partagent un répertoire de mémoire automatique. Les 200 premières lignes ou 25 KB de l'index `MEMORY.md` se chargent au début de chaque session. Auto memory est l'équivalent écrit par Claude de [CLAUDE.md](#claude-md), que vous écrivez.

En savoir plus : [Auto memory](/docs/fr/memory#auto-memory)

<h3 id="auto-mode">
  Auto mode
</h3>

Un [permission mode](#permission-mode) où un modèle de classificateur séparé examine les actions au lieu de vous, de sorte que Claude Code exécute la plupart d'entre elles sans vous demander. Claude Code vous demande toujours avant les actions qui correspondent à vos règles d'ask explicites. Sur les plans Pro, Max et Team, auto mode est le [permission mode de démarrage intégré](/docs/fr/permission-modes#which-mode-a-session-starts-in) pour les sessions de terminal interactif et VS Code. Le classificateur bloque l'escalade de portée, l'infrastructure non fiable et [prompt injection](#prompt-injection). Les résultats des outils sont supprimés de ce qu'il voit, de sorte que le contenu hostile dans un fichier ou une page web ne peut pas le manipuler directement.

En savoir plus : [Eliminate prompts with auto mode](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode)

<h2 id="b">
  B
</h2>

<h3 id="bare-mode">
  Bare mode
</h3>

Avec `--bare`, Claude Code démarre sans charger les hooks, skills, commandes personnalisées, subagents, plugins, serveurs MCP, auto memory ou CLAUDE.md, à l'exception des skills dans un répertoire que vous transmettez avec `--add-dir`. Recommandé pour CI et les appels scriptés où vous avez besoin du même résultat sur chaque machine.

En savoir plus : [Démarrer plus rapidement avec bare mode](/docs/fr/headless#start-faster-with-bare-mode)

<h3 id="bundled-skills">
  Bundled skills
</h3>

Des playbooks basés sur des invites inclus avec Claude Code, tels que `/batch`, `/code-review`, `/debug` et `/loop`. Contrairement aux commandes intégrées, qui exécutent une logique fixe, les bundled skills donnent à Claude une invite détaillée et lui permettent d'orchestrer le travail, afin qu'ils puissent générer des agents, lire des fichiers et s'adapter à votre base de code.

En savoir plus : [Bundled skills](/docs/fr/skills#bundled-skills)

<h2 id="c">
  C
</h2>

<h3 id="channel">
  Channel
</h3>

Un [serveur MCP](#mcp-model-context-protocol) qui pousse des événements dans votre session en cours afin que Claude puisse réagir aux choses qui se produisent pendant que vous êtes loin du terminal. Les channels peuvent être bidirectionnels : Claude lit un événement entrant et répond via le même channel. Telegram, Discord et iMessage sont inclus dans l'aperçu de recherche.

En savoir plus : [Channels](/docs/fr/channels)

<h3 id="checkpoint">
  Checkpoint
</h3>

Un point de restauration créé à chaque invite que vous envoyez qui démarre un tour. Claude Code crée des instantanés des fichiers avant chaque modification afin qu'un checkpoint puisse les restaurer. Appuyez sur `Esc` deux fois ou exécutez `/rewind` pour restaurer le code, la conversation ou les deux à un point antérieur, ou pour résumer une partie de la conversation à partir d'un message sélectionné. Les checkpoints sont sauvegardés avec la conversation, donc une session reprise peut toujours `/rewind` vers eux. Ils sont séparés de git et ne suivent pas les modifications apportées via l'outil Bash.

En savoir plus : [Checkpointing](/docs/fr/checkpointing)

<h3 id="claude-directory">
  `.claude` directory
</h3>

Le répertoire où Claude Code lit la configuration au niveau du projet : paramètres, hooks, skills, subagents, règles et auto memory. Un projet a `.claude/` à sa racine ; vos paramètres par défaut au niveau utilisateur se trouvent à `~/.claude/`.

En savoir plus : [The `.claude` directory](/docs/fr/claude-directory)

<h3 id="claude-md">
  CLAUDE.md
</h3>

Un fichier markdown d'instructions persistantes que vous écrivez pour Claude, chargé au début de chaque session en tant que message utilisateur après l'invite système. Mettez les conventions de projet, les notes d'architecture et les règles « toujours faire X » ici. Project-root CLAUDE.md survit à [compaction](#compaction) et est relu à nouveau à partir du disque après.

Vous pouvez placer CLAUDE.md au niveau du projet dans `./CLAUDE.md` ou `./.claude/CLAUDE.md`, au niveau utilisateur dans `~/.claude/CLAUDE.md`, ou comme [managed policy](#managed-settings) pour votre organisation. Tous les fichiers découverts sont concaténés dans le contexte plutôt que de se remplacer les uns les autres, ordonnés du champ d'application le plus large au plus spécifique. Claude Code peut également charger les fichiers [AGENTS.md](#agents-md) d'un projet, seuls ou aux côtés de CLAUDE.md.

En savoir plus : [CLAUDE.md files](/docs/fr/memory#claude-md-files)

<h3 id="cloud-session">
  Cloud session
</h3>

Une session Claude Code qui continue de s'exécuter après avoir fermé votre ordinateur portable, car elle s'exécute sur une infrastructure cloud au lieu de votre machine : gérée par Anthropic par défaut, ou un [environnement auto-hébergé](/docs/fr/self-hosted-environments) que votre organisation exploite. Vous en démarrez une à partir de claude.ai/code, l'application mobile Claude, l'application Desktop avec **Cloud** sélectionné, `claude --cloud`, ou une [routine](/docs/fr/routines). Une session dans votre terminal, IDE ou l'application Desktop avec **Local** sélectionné est une session locale ; pour accéder à une session locale à partir d'un autre appareil, utilisez [Remote Control](#remote-control).

En savoir plus : [Use Claude Code in the cloud](/docs/fr/claude-code-on-the-web)

<h3 id="command">
  Command
</h3>

Une instruction réutilisable que vous invoquez en tapant `/name` dans l'invite. Les commandes intégrées telles que `/clear`, `/model` et `/compact` contrôlent la session. Vous pouvez définir vos propres commandes en tant que fichiers dans `.claude/commands/`, ou les installer à partir d'un [plugin](#plugin). Les [Skills](#skill) sont la méthode recommandée pour empaqueter les commandes multi-étapes.

Deux autres utilisations du mot ne sont pas liées : les sous-commandes CLI `claude` telles que `claude mcp add`, listées dans la [référence CLI](/docs/fr/cli-reference#cli-commands), et le champ `command` d'une entrée de [serveur MCP](#mcp-server) stdio, qui spécifie l'exécutable que Claude Code lance pour démarrer le serveur.

En savoir plus : [Commands](/docs/fr/commands) · [Skills](/docs/fr/skills)

<h3 id="compaction">
  Compaction
</h3>

Résumé automatique de votre conversation lorsque la [context window](#context-window) approche de sa limite. Les résultats des outils plus anciens sont d'abord effacés, puis la conversation est résumée. Project-root CLAUDE.md et auto memory survivent à la compaction et se rechargent à partir du disque ; les instructions données uniquement dans la conversation peuvent être perdues. Exécutez `/compact` pour déclencher manuellement, éventuellement avec un focus comme `/compact focus on the API changes`.

En savoir plus : [What survives compaction](/docs/fr/context-window#what-survives-compaction) · [When context fills up](/docs/fr/how-claude-code-works#when-context-fills-up)

<h3 id="connector">
  Connector
</h3>

Un [serveur MCP](#mcp-server) ajouté à votre compte claude.ai plutôt que configuré dans Claude Code. Lorsque vous vous connectez à Claude Code avec ce compte, vos connecteurs apparaissent dans `/mcp` aux côtés des serveurs que vous avez ajoutés localement. Les organisations peuvent également provisionner des connecteurs et définir des contrôles par outil sur eux.

En savoir plus : [Use MCP servers from claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai)

<h3 id="context-window">
  Context window
</h3>

La mémoire de travail d'une session, contenant l'historique des conversations, le contenu des fichiers, les résultats des commandes, CLAUDE.md, auto memory, les skills chargés et les instructions système. Au fur et à mesure que vous travaillez, le contexte se remplit jusqu'à ce que [compaction](#compaction) le résume. Exécutez `/context` pour voir ce qui utilise l'espace. Pour le concept de modèle sous-jacent, consultez le [glossaire de la plateforme](https://platform.claude.com/docs/fr/about-claude/glossary#context-window).

En savoir plus : [Explore the context window](/docs/fr/context-window)

<h2 id="d">
  D
</h2>

<h3 id="dispatch">
  Dispatch
</h3>

Un routeur de tâches initié par téléphone qui génère une session Claude Code dans l'application Desktop lorsque vous envoyez une tâche de codage depuis l'application mobile Claude. Votre invite s'achemine automatiquement vers le bon outil. Disponible sur les plans Pro et Max.

En savoir plus : [Sessions from Dispatch](/docs/fr/desktop#sessions-from-dispatch)

<h2 id="e">
  E
</h2>

<h3 id="effort-level">
  Effort level
</h3>

Un paramètre qui contrôle le raisonnement adaptatif, ce qui permet au modèle de décider s'il doit réfléchir et dans quelle mesure à chaque étape. Un effort plus élevé signifie plus de tokens de réflexion et un raisonnement plus profond ; un effort plus faible est plus rapide et moins cher. L'effort est pris en charge sur les modèles Fable, sur Opus 4.6 et versions ultérieures, ainsi que sur Sonnet 4.6 et versions ultérieures.

En savoir plus : [Adjust effort level](/docs/fr/model-config#adjust-effort-level)

<h3 id="extended-thinking">
  Extended thinking
</h3>

Un raisonnement étape par étape visible que le modèle effectue avant de répondre. Vous pouvez l'ajuster avec le [effort level](#effort-level), ou plafonner les tokens de réflexion avec `MAX_THINKING_TOKENS` sur les modèles avec un budget de réflexion fixe. La réflexion apparaît en texte gris italique dans le terminal.

En savoir plus : [Use extended thinking](/docs/fr/model-config#extended-thinking)

<h2 id="f">
  F
</h2>

<h3 id="frontmatter">
  Frontmatter
</h3>

Un bloc de paramètres YAML au tout début d'un fichier Markdown, entre une ligne d'ouverture `---` et une ligne de fermeture `---`. Les Skills, subagents, output styles et rules lisent chacun leur configuration à partir du frontmatter, comme la `description` d'une skill ou les `tools` d'un subagent, et traitent tout ce qui suit le `---` de fermeture comme les instructions. Le `---` d'ouverture doit être la première ligne du fichier. Chaque type de fichier accepte son propre ensemble de champs.

En savoir plus : [Skill frontmatter](/docs/fr/skills#frontmatter-reference), [Subagent frontmatter](/docs/fr/sub-agents#supported-frontmatter-fields), [Output style frontmatter](/docs/fr/output-styles#frontmatter), [Rule frontmatter](/docs/fr/memory#rules-frontmatter-reference)

<h2 id="h">
  H
</h2>

<h3 id="hook">
  Hook
</h3>

Un gestionnaire défini par l'utilisateur qui s'exécute automatiquement à un point spécifique du cycle de vie de Claude Code, par exemple avant l'exécution d'un outil, après une modification de fichier ou au démarrage de la session. Les gestionnaires peuvent être une commande shell, un point de terminaison HTTP, un outil MCP, une invite LLM ou un subagent. Les hooks sont déterministes : ils se déclenchent à des points de cycle de vie fixes plutôt qu'à la discrétion du modèle.

Une configuration de hook a trois niveaux :

* **Hook event** : le point du cycle de vie
* **Matcher** : filtre les événements qui le déclenchent
* **Hook handler** : ce qui s'exécute

En savoir plus : [Get started with hooks](/docs/fr/hooks-guide) · [Hooks reference](/docs/fr/hooks)

<h2 id="m">
  M
</h2>

<h3 id="managed-settings">
  Managed settings
</h3>

Les paramètres gérés sont des paramètres appliqués à l'échelle de l'organisation par l'informatique ou DevOps, fournis par les serveurs d'Anthropic via la console d'administration ou déployés sur les appareils à un chemin au niveau du système d'exploitation en dehors de `~/.claude`. Les paramètres utilisateur et projet ne peuvent pas remplacer les paramètres gérés. La livraison gérée par le serveur s'applique sur les [configurations éligibles](/docs/fr/server-managed-settings#platform-availability) ; consultez les [Considérations de sécurité](/docs/fr/server-managed-settings#security-considerations). Utilisez ceci pour les politiques de sécurité, les exigences de conformité ou les outils standardisés sur une flotte.

En savoir plus : [Server-managed settings](/docs/fr/server-managed-settings) · [Settings files](/docs/fr/settings#where-settings-live)

<h3 id="mcp-model-context-protocol">
  MCP (Model Context Protocol)
</h3>

Une norme ouverte pour connecter les outils d'IA aux sources de données et services externes. Les serveurs MCP donnent à Claude de nouveaux outils pour Slack, Jira, les bases de données, les navigateurs et des centaines d'autres intégrations. Vous connectez les serveurs via `/mcp` ou en les ajoutant à `.mcp.json`. Pour le protocole lui-même, consultez le [glossaire de la plateforme](https://platform.claude.com/docs/fr/about-claude/glossary#mcp-model-context-protocol).

En savoir plus : [Model Context Protocol](/docs/fr/mcp)

<h3 id="mcp-server">
  MCP server
</h3>

Un programme qui donne à Claude des outils, des invites ou des ressources via [MCP](#mcp-model-context-protocol). Vous ajoutez des serveurs avec `claude mcp add`, dans `.mcp.json`, via un [plugin](#plugin), ou en tant que connecteur [connector](#connector) de claude.ai. Un serveur stdio local s'exécute en tant que processus que Claude Code démarre à partir des champs `command` et `args` de sa configuration, qui n'ont rien à voir avec les [commandes](#command) que vous tapez à l'invite.

En savoir plus : [Model Context Protocol](/docs/fr/mcp)

<h3 id="mcp-tool-search">
  MCP Tool Search
</h3>

Un mécanisme d'économie de contexte qui reporte les schémas d'outils MCP jusqu'à ce qu'ils soient nécessaires. Seuls les noms d'outils et les instructions du serveur se chargent au démarrage ; Claude récupère le schéma complet à la demande lorsqu'il décide d'utiliser un outil spécifique. Cela empêche les serveurs MCP inactifs de consommer beaucoup de contexte.

En savoir plus : [Scale with MCP Tool Search](/docs/fr/mcp#scale-with-mcp-tool-search)

<h2 id="n">
  N
</h2>

<h3 id="non-interactive-mode">
  Non-interactive mode
</h3>

Un mode qui exécute une seule invite et se ferme sans session conversationnelle, invoqué avec `-p` ou `--print`. Utilisé pour CI, les scripts et le piping. L'exécution est toujours enregistrée en tant que session reprise sauf si vous passez `--no-session-persistence`. Le [Agent SDK](/docs/fr/agent-sdk/overview) est l'équivalent Python et TypeScript. Anciennement appelé headless mode.

En savoir plus : [Run Claude Code programmatically](/docs/fr/headless)

<h2 id="o">
  O
</h2>

<h3 id="output-style">
  Output style
</h3>

Une configuration qui modifie les instructions que Claude Code donne à Claude, pour définir le comportement, le ton ou le format de la réponse. Contrairement à [CLAUDE.md](#claude-md), qui ajoute le contexte du projet aux côtés des instructions par défaut de Claude Code, un output style personnalisé peut remplacer les instructions d'ingénierie logicielle par défaut.

En savoir plus : [Output styles](/docs/fr/output-styles)

<h2 id="p">
  P
</h2>

<h3 id="permission-mode">
  Mode de permission
</h3>

Le comportement d'approbation de base pour la session. Basculez avec `Maj+Tab` dans l'interface de ligne de commande ou utilisez le sélecteur de mode dans VS Code, l'application de bureau et claude.ai. Les modes disponibles sont `default`, `acceptEdits`, `plan`, `auto`, `dontAsk` et `bypassPermissions`.

Le mode `default` est étiqueté Manuel dans l'interface de ligne de commande, dans les extensions VS Code et JetBrains, et dans l'application de bureau, et Claude Code accepte `manual` comme alias pour la valeur.

En savoir plus : [Choisir un mode de permission](/docs/fr/permission-modes)

<h3 id="permission-rule">
  Règle de permission
</h3>

Une entrée de paramètres qui autorise, demande ou refuse une invocation d'outil en fonction du nom de l'outil et du modèle d'argument. Les règles sont évaluées deny→ask→allow, le premier correspondance gagne. Les règles de permission sont des contrôles granulaires superposés au-dessus du [mode de permission](#permission-mode) plus large.

En savoir plus : [Configurer les permissions](/docs/fr/permissions)

<h3 id="plan-mode">
  Plan mode
</h3>

Un [mode de permission](#permission-mode) où Claude recherche et propose des modifications sans modifier vos fichiers source. Il peut lire, rechercher et exécuter des commandes d'exploration, puis présente un plan pour approbation avant de toucher à quoi que ce soit. Entrez en plan mode avec `/plan` ou en appuyant sur `Maj+Tab`.

En savoir plus : [Analyser avant de modifier avec plan mode](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode)

<h3 id="plugin">
  Plugin
</h3>

Un ensemble de skills, hooks, subagents et serveurs MCP emballés comme une unité installable unique. Les skills de plugin sont espacés de noms comme `plugin-name:skill-name` afin que plusieurs plugins coexistent. Distribuez les plugins dans les équipes via un [marketplace](/docs/fr/plugins/overview).

En savoir plus : [Plugins](/docs/fr/plugins/overview)

<h3 id="project-trust">
  Confiance du projet
</h3>

Un dialogue acceptant un répertoire avant que Claude Code ne charge sa configuration. L'acceptation est enregistrée par répertoire de projet, sauf votre répertoire personnel, où la confiance est maintenue pour la session actuelle uniquement et l'invite réapparaît à chaque lancement. Jusqu'à ce que vous fassiez confiance à un répertoire, Claude Code retient une partie du contenu que son référentiel fournit, comme les règles d'autorisation du projet et les marketplaces de `.claude/settings.json`. [Ce qui s'exécute avant de faire confiance à un dossier](/docs/fr/permissions#what-runs-before-you-trust-a-folder) énumère chaque type de contenu, y compris ce qu'une session `-p` exécute sans dialogue.

En savoir plus : [Le répertoire `.claude`](/docs/fr/claude-directory)

<h3 id="prompt-injection">
  Injection de prompt
</h3>

Des instructions hostiles intégrées dans un fichier, une page web ou un résultat d'outil qui tentent de rediriger Claude vers des actions que vous n'avez jamais demandées. Les défenses de Claude Code incluent le système de permission, la détection d'injection de commande et la vérification de confiance. [Le mode Auto](#auto-mode) ajoute une sonde côté serveur qui analyse les résultats des outils pour détecter le contenu suspect et un classificateur qui examine les actions avec les résultats des outils supprimés, afin que le texte injecté ne puisse pas le manipuler directement.

En savoir plus : [Protéger contre l'injection de prompt](/docs/fr/security#protect-against-prompt-injection)

<h2 id="r">
  R
</h2>

<h3 id="remote-control">
  Remote Control
</h3>

Un moyen de continuer une session Claude Code locale depuis votre téléphone ou navigateur via claude.ai. Votre exécution de code et vos fichiers restent sur votre machine ; seule l'interface est distante. Différent d'une [session cloud](/docs/fr/claude-code-on-the-web), qui s'exécute dans un sandbox cloud.

En savoir plus : [Remote Control](/docs/fr/remote-control)

<h3 id="rules">
  Rules
</h3>

Des fichiers d'instructions modulaires dans `.claude/rules/` qui se chargent aux côtés de CLAUDE.md. Une règle peut être délimitée par chemin avec le frontmatter YAML `paths:` afin qu'elle ne se charge que lorsque Claude lit un fichier correspondant, gardant le contexte maigre jusqu'à ce qu'il soit pertinent.

En savoir plus : [Organize rules with `.claude/rules/`](/docs/fr/memory#organize-rules-with-claude/rules/)

<h2 id="s">
  S
</h2>

<h3 id="sandboxing">
  Sandboxing
</h3>

Isolation du système de fichiers et du réseau au niveau du système d'exploitation pour l'outil Bash. Les commandes s'exécutent à l'intérieur d'une limite que vous définissez à l'avance, afin que Claude puisse travailler librement sans invites d'approbation par commande. Le sandboxing est une couche séparée des [permission rules](#permission-rule).

En savoir plus : [Sandboxing](/docs/fr/sandboxing)

<h3 id="session">
  Session
</h3>

Une conversation liée à votre répertoire actuel, avec sa propre [context window](#context-window) indépendante. Les sessions peuvent être reprises avec `claude -c`, bifurquées avec `--fork-session` pour préserver l'historique sous un nouvel ID de session, ou exécutées en parallèle sur les terminaux. L'exécution de `/clear` démarre une nouvelle session ; la session précédente reste stockée et est disponible via `/resume`. La transcription de chaque session est stockée sous `~/.claude/projects/`.

En savoir plus : [Work with sessions](/docs/fr/how-claude-code-works#work-with-sessions)

<h3 id="settings-layers">
  Settings layers
</h3>

La hiérarchie à partir de laquelle Claude Code lit la configuration, par ordre de priorité du plus élevé au plus bas : [managed policy](#managed-settings), arguments de ligne de commande, paramètres locaux à `.claude/settings.local.json`, paramètres de projet à `.claude/settings.json`, puis paramètres utilisateur à `~/.claude/settings.json`. Les tableaux fusionnent entre les couches ; les scalaires à une couche supérieure remplacent les couches inférieures. Voir [Settings precedence](/docs/fr/settings#settings-precedence).

En savoir plus : [Settings files](/docs/fr/settings#where-settings-live)

<h3 id="skill">
  Skill
</h3>

Un fichier `SKILL.md` contenant des instructions, des connaissances ou un flux de travail que Claude ajoute à sa boîte à outils. Claude charge une skill automatiquement lorsqu'elle est pertinente, ou vous l'invoquez directement avec `/skill-name`. Les skills suivent la norme Agent Skills ouverte ; Claude Code l'étend avec le contrôle d'invocation et l'exécution de subagent.

Les skills sont le successeur recommandé aux commandes personnalisées. Un fichier à `.claude/commands/deploy.md` et un à `.claude/skills/deploy/SKILL.md` créent tous deux `/deploy` et fonctionnent de la même manière ; les fichiers de commande existants continuent de fonctionner.

En savoir plus : [Extend Claude with skills](/docs/fr/skills)

<h3 id="subagent">
  Subagent
</h3>

Un assistant IA spécialisé qui s'exécute dans sa propre fenêtre de contexte avec une invite système personnalisée, un accès à des outils spécifiques et des permissions indépendantes. Il travaille sur une tâche déléguée et retourne un résumé à la conversation principale. Utilisez les subagents pour garder les grandes explorations hors de votre contexte principal ou pour exécuter des recherches parallèles. Un subagent reste à l'intérieur de la session qui l'a créé. Pour transmettre les résultats entre des sessions distinctes que vous exécutez vous-même, utilisez [cross-session messaging](/docs/fr/cross-session-messaging).

Les subagents intégrés incluent Explore, Plan et à usage général.

En savoir plus : [Create custom subagents](/docs/fr/sub-agents)

<h3 id="surface">
  Surface
</h3>

N'importe quel endroit où vous accédez à Claude Code : la CLI, VS Code, JetBrains, Desktop ou claude.ai. Toutes les surfaces partagent le même moteur. Les sessions sur votre machine lisent votre CLAUDE.md local, vos paramètres et vos skills ; les [cloud sessions](/docs/fr/cloud-environments#what-carries-over-from-your-setup) commencent par un clone frais de votre référentiel et ne lisent pas `~/.claude/` sur votre machine. Slack et l'extension Chrome sont des intégrations qui se connectent à une surface plutôt que des surfaces elles-mêmes.

En savoir plus : [Platforms and integrations](/docs/fr/platforms)

<h2 id="t">
  T
</h2>

<h3 id="teleport">
  Teleport
</h3>

Une commande, `/teleport`, qui tire une session Claude Code cloud dans votre terminal local. Claude récupère la branche, charge l'historique des conversations et reprend à partir du dernier état de la session cloud. La direction inverse est `--cloud`, qui envoie une tâche locale pour s'exécuter sur le cloud.

En savoir plus : [From cloud to terminal](/docs/fr/claude-code-on-the-web#from-cloud-to-terminal)

<h3 id="tool">
  Tool
</h3>

Une action que Claude peut prendre : lire un fichier, modifier le code, exécuter une commande shell, rechercher sur le web, générer un subagent. Les tools sont ce qui rend Claude Code agentic. Sans eux, Claude ne peut que répondre avec du texte. Chaque utilisation d'outil retourne un résultat qui informe la décision suivante de Claude dans la [agentic loop](#agentic-loop).

En savoir plus : [Tools available to Claude](/docs/fr/tools-reference)

<h3 id="turn">
  Turn
</h3>

Une réponse complète de Claude au sein d'une [session](#session). Un turn commence quand vous envoyez un message et se termine quand Claude finit de répondre, avec un nombre quelconque d'appels [tool](#tool) entre les deux. Les [stop hooks](#hook) se déclenchent à la fin de chaque turn. Une session se compose de nombreux turns, et la [agentic loop](#agentic-loop) décrit ce qui se passe à l'intérieur d'un.

En savoir plus : [How Claude Code works](/docs/fr/how-claude-code-works#the-agentic-loop)

<h2 id="v">
  V
</h2>

<h3 id="verification-loop">
  Verification loop
</h3>

Comment une session sait que le travail est réellement terminé plutôt que simplement plausible. Vous donnez à Claude une vérification qu'il peut exécuter, comme une suite de tests, une compilation ou une comparaison de captures d'écran, et Claude itère jusqu'à ce que la vérification réussisse au lieu de s'arrêter après une tentative. Une verification loop est la condition préalable pour [`/goal`](/docs/fr/goal), les exécutions sans surveillance et les [dynamic workflows](/docs/fr/workflows) : sans elle, la seule chose qui décide que l'agent a terminé est l'agent lui-même.

En savoir plus : [Donnez à Claude un moyen de vérifier son travail](/docs/fr/best-practices#give-claude-a-way-to-verify-its-work)

<h2 id="w">
  W
</h2>

<h3 id="worktree-isolation">
  Worktree isolation
</h3>

Un mode d'isolation qui exécute Claude dans un worktree git séparé sous `.claude/worktrees/`, activé avec le drapeau `-w` ou `isolation: worktree` dans la configuration du subagent. Les modifications restent sur une branche séparée dans un répertoire séparé, afin que les agents parallèles ne se remplacent pas les fichiers les uns des autres.

En savoir plus : [Run parallel sessions with git worktrees](/docs/fr/worktrees)

***

<h2 id="deprecated-and-renamed-terms">
  Termes dépréciés et renommés
</h2>

Ces termes apparaissent dans les documents plus anciens, les articles de blog et le contenu communautaire. Utilisez le nom actuel lors de la recherche sur ce site.

| Old term                                                                | Now called                                    | Notes                                                                         |
| ----------------------------------------------------------------------- | --------------------------------------------- | ----------------------------------------------------------------------------- |
| Headless mode                                                           | [Non-interactive mode](#non-interactive-mode) | Same `-p` flag, same behavior                                                 |
| Web session; "Claude Code on the web" as the name for any cloud session | [Cloud session](#cloud-session)               | "Claude Code on the web" now names only the browser surface at claude.ai/code |
| Custom commands                                                         | [Skills](#skill)                              | `.claude/commands/` files still work                                          |
| Slash commands                                                          | Commands                                      | "Slash" dropped from product copy                                             |
