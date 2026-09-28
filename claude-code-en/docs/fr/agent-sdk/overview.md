> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Présentation du SDK Agent

> Créez des agents IA de production avec Claude Code en tant que bibliothèque

Un agent est une application qui complète une tâche en planifiant ses propres étapes et en appelant des outils qui lisent des fichiers, exécutent des commandes ou modifient du code. Le SDK Agent vous offre les mêmes outils, [boucle d'agent](/docs/fr/agent-sdk/agent-loop), et gestion du contexte qui alimentent Claude Code, programmables en Python et TypeScript.

<h2 id="compare-the-agent-sdk-to-other-claude-tools">
  Comparer le SDK Agent à d'autres outils Claude
</h2>

Le SDK Agent, la CLI, le SDK Client et les Agents Gérés diffèrent par qui exécute l'agent, ce qui est intégré et comment vous y accédez. Trouvez la ligne qui correspond à la façon dont vous souhaitez construire et exécuter le vôtre.

| Vous souhaitez                                                                                                        | Utilisez                                                                          | Ce que vous obtenez                                                                                                                                                                                                                                                                                                                                                                                                              |
| --------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Intégrer l'agent Claude Code dans votre propre application Python ou TypeScript, dans un processus que vous exploitez | **SDK Agent**                                                                     | Une bibliothèque qui exécute le binaire Claude Code, avec les [capacités](#capabilities) de Claude Code, telles que les outils intégrés, les permissions, les sessions et les hooks.                                                                                                                                                                                                                                             |
| Faire du développement interactif ou exécuter des tâches ponctuelles depuis un terminal                               | [**Claude Code CLI**](/docs/fr/overview)                                               | L'interface de terminal, conçue pour une utilisation interactive quotidienne.                                                                                                                                                                                                                                                                                                                                                    |
| Appeler l'API Claude directement depuis votre propre code                                                             | [**SDK Client**](https://platform.claude.com/docs/en/cli-sdks-libraries/overview) | Accès direct à l'API Claude à partir de n'importe quel langage du SDK client. Vous écrivez vous-même la boucle d'outils, ou laissez le [tool runner](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-runner) bêta du SDK client la piloter.                                                                                                                                                                   |
| Faire héberger l'agent par Anthropic, configuré via l'API Claude                                                      | [**Agents Gérés**](https://platform.claude.com/docs/en/managed-agents/overview)   | Un harnais d'agent hébergé qui exécute la boucle d'agent, avec des sessions dans un sandbox cloud géré par Anthropic ou un [sandbox auto-hébergé](https://platform.claude.com/docs/en/managed-agents/self-hosted-sandboxes) sur votre propre infrastructure. Utilisez-le à partir du [SDK pour votre langage](https://platform.claude.com/docs/en/managed-agents/quickstart#install-the-sdk), de la CLI `ant`, ou de l'API REST. |

Pour piloter la même boucle d'agent à partir d'un langage autre que Python ou TypeScript, [exécutez la CLI en tant que sous-processus](/docs/fr/headless) avec le drapeau `-p` et `--output-format json`.

<h2 id="capabilities">
  Capacités
</h2>

Ces capacités de Claude Code sont disponibles dans le SDK :

| Capacité                     | Ce qu'elle fait                                                                                                 | En savoir plus                                                                                                                                                                                                            |
| ---------------------------- | --------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Outils intégrés              | Lire, écrire, modifier des fichiers, exécuter des commandes et rechercher sur le web                            | [Référence des outils](/docs/fr/tools-reference)                                                                                                                                                                               |
| Hooks                        | Exécuter du code personnalisé à des points clés du cycle de vie de l'agent                                      | [Hooks](/docs/fr/agent-sdk/hooks)                                                                                                                                                                                              |
| Sous-agents                  | Générer des agents spécialisés pour des sous-tâches ciblées                                                     | [Sous-agents](/docs/fr/agent-sdk/subagents)                                                                                                                                                                                    |
| MCP                          | Connecter des outils externes et des sources de données via le Model Context Protocol                           | [MCP](/docs/fr/agent-sdk/mcp)                                                                                                                                                                                                  |
| Permissions                  | Contrôler quels outils s'exécutent automatiquement, lesquels nécessitent une approbation                        | [Permissions](/docs/fr/agent-sdk/permissions)                                                                                                                                                                                  |
| Sessions                     | Maintenir le contexte sur plusieurs échanges, reprendre ou diviser plus tard                                    | [Sessions](/docs/fr/agent-sdk/sessions)                                                                                                                                                                                        |
| Skills, commandes et mémoire | Charger automatiquement à partir du répertoire `.claude/` de votre projet et de `~/.claude/`, comme Claude Code | [Skills](/docs/fr/agent-sdk/skills), [Commandes](/docs/fr/agent-sdk/skills#commands-in-agent-sdk-sessions), [Mémoire](/docs/fr/agent-sdk/modifying-system-prompts), [Chargement de la configuration](/docs/fr/agent-sdk/claude-code-features) |
| Plugins                      | Empaqueter les skills, les agents, les hooks et les serveurs MCP, et les charger par chemin local               | [Plugins](/docs/fr/agent-sdk/plugins)                                                                                                                                                                                          |

<h2 id="get-started">
  Commencer
</h2>

Suivez le [Démarrage rapide](/docs/fr/agent-sdk/quickstart) pour installer le SDK, définir votre clé API et créer votre premier agent, un agent qui trouve et corrige les bugs dans le code existant.

<Note>
  Sauf approbation préalable, Anthropic n'autorise pas les développeurs tiers à proposer la connexion claude.ai ou les limites de débit pour leurs produits, y compris les agents construits sur le SDK Agent Claude. Utilisez plutôt les méthodes d'authentification par clé API décrites dans le [Démarrage rapide](/docs/fr/agent-sdk/quickstart).
</Note>

<h2 id="changelog">
  Journal des modifications
</h2>

Consultez le journal des modifications complet pour les mises à jour du SDK, les corrections de bugs et les nouvelles fonctionnalités :

* **SDK TypeScript** : [voir CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md)
* **SDK Python** : [voir CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md)

<h2 id="report-bugs">
  Signaler les bugs
</h2>

Si vous rencontrez des bugs ou des problèmes avec le SDK Agent :

* **SDK TypeScript** : [signaler les problèmes sur GitHub](https://github.com/anthropics/claude-agent-sdk-typescript/issues)
* **SDK Python** : [signaler les problèmes sur GitHub](https://github.com/anthropics/claude-agent-sdk-python/issues)

<h2 id="branding-guidelines">
  Directives de marque
</h2>

Pour les partenaires intégrant le SDK Claude Agent, l'utilisation de la marque Claude est facultative. Lorsque vous référencez Claude dans votre produit :

**Autorisé :**

* « Claude Agent », préféré pour les menus déroulants
* « Claude », lorsque vous êtes déjà dans un menu étiqueté « Agents »
* « \{YourAgentName} Powered by Claude », si vous avez un nom d'agent existant

**Non autorisé :**

* « Claude Code » ou « Claude Code Agent »
* Art ASCII ou éléments visuels de marque Claude Code qui imitent Claude Code

Votre produit doit conserver sa propre marque et ne pas sembler être Claude Code ou un produit Anthropic. Pour des questions sur la conformité de la marque, contactez l'équipe [ventes](https://www.anthropic.com/contact-sales) d'Anthropic.

<h2 id="license-and-terms">
  Licence et conditions
</h2>

L'utilisation du SDK Claude Agent est régie par les [Conditions commerciales d'Anthropic](https://www.anthropic.com/legal/commercial-terms), y compris lorsque vous l'utilisez pour alimenter des produits et services que vous mettez à disposition de vos propres clients et utilisateurs finaux, sauf dans la mesure où un composant ou une dépendance spécifique est couvert par une licence différente comme indiqué dans le fichier LICENSE de ce composant.

<h2 id="next-steps">
  Prochaines étapes
</h2>

Ces ressources couvrent des détails techniques plus approfondis et des projets d'exemple pour construire avec le SDK Agent.

* [Guide de démarrage](/docs/fr/agent-sdk/quickstart) : créez votre premier agent qui trouve et corrige les bugs
* [Guide de migration](/docs/fr/agent-sdk/migration-guide) : migrez des packages Claude Code SDK vers le SDK Agent
* [Boucle d'agent](/docs/fr/agent-sdk/agent-loop) : comment Claude planifie, appelle les outils et décide quand une tâche est terminée
* [Agents d'exemple](https://github.com/anthropics/claude-agent-sdk-demos) : applications de démonstration pour le développement local
* [SDK TypeScript](/docs/fr/agent-sdk/typescript) : référence API TypeScript complète et exemples
* [SDK Python](/docs/fr/agent-sdk/python) : référence API Python complète et exemples
* [Conception du harnais d'agent](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code) : comment l'équipe Claude Code utilise les workflows dynamiques pour orchestrer de nombreux sous-agents à la fois
