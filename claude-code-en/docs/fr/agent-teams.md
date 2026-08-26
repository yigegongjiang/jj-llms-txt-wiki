> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Orchestrer des équipes de sessions Claude Code

> Coordonnez plusieurs instances Claude Code travaillant ensemble en tant qu'équipe, avec des tâches partagées, la messagerie inter-agents et la gestion centralisée.

<Warning>
  Les équipes d'agents sont expérimentales et désactivées par défaut. Activez-les en ajoutant `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` à votre [settings.json](/docs/fr/settings) ou à votre environnement. Sans cette variable, aucune équipe n'est configurée au démarrage de la session, aucun répertoire d'équipe n'est écrit, et Claude ne crée pas ou ne propose pas de coéquipiers. Les équipes d'agents ont des [limitations connues](#limitations) concernant la reprise de session, la coordination des tâches et le comportement d'arrêt.
</Warning>

Les équipes d'agents vous permettent de coordonner plusieurs instances Claude Code travaillant ensemble. Une session agit comme chef d'équipe, coordonnant le travail, assignant des tâches et synthétisant les résultats. Les coéquipiers travaillent indépendamment, chacun dans sa propre fenêtre de contexte, et communiquent directement les uns avec les autres.

Contrairement aux [subagents](/docs/fr/sub-agents), qui s'exécutent au sein d'une seule session et ne peuvent que rendre compte à l'agent principal, vous pouvez également interagir directement avec les coéquipiers individuels sans passer par le chef.

<Note>
  Cette page décrit les équipes d'agents à partir de la v2.1.178. Avec `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` défini, la création d'un coéquipier n'a plus besoin d'une étape de configuration, et le nettoyage se fait automatiquement à la fermeture de la session. Avant la v2.1.178, vous demandiez à Claude de créer et de nommer une équipe en premier, et Claude utilisait les outils `TeamCreate` et `TeamDelete` pour la configurer et la supprimer. Ces deux outils n'existent plus. L'entrée `team_name` sur l'outil Agent est acceptée mais ignorée, et le champ `team_name` dans les [payloads de hook](/docs/fr/hooks#taskcreated) `TaskCreated`, `TaskCompleted` et `TeammateIdle` porte le nom dérivé de la session et est déprécié.
</Note>

<h2 id="when-to-use-agent-teams">
  Quand utiliser les équipes d'agents
</h2>

Les équipes d'agents sont les plus efficaces pour les tâches où l'exploration parallèle ajoute une réelle valeur. Consultez les [exemples de cas d'usage](#use-case-examples) pour des scénarios complets. Les cas d'usage les plus solides sont :

* **Recherche et examen** : plusieurs coéquipiers peuvent enquêter sur différents aspects d'un problème simultanément, puis partager et contester les conclusions les uns des autres
* **Nouveaux modules ou fonctionnalités** : les coéquipiers peuvent chacun posséder une partie distincte sans se marcher dessus
* **Débogage avec hypothèses concurrentes** : les coéquipiers testent différentes théories en parallèle et convergent vers la réponse plus rapidement
* **Coordination inter-couches** : les modifications qui s'étendent sur le frontend, le backend et les tests, chacun possédé par un coéquipier différent

Les équipes d'agents ajoutent une surcharge de coordination et utilisent considérablement plus de tokens qu'une seule session. Elles fonctionnent mieux lorsque les coéquipiers peuvent opérer indépendamment. Pour les tâches séquentielles, les modifications du même fichier ou le travail avec de nombreuses dépendances, une seule session ou les [subagents](/docs/fr/sub-agents) sont plus efficaces.

<h3 id="compare-with-subagents">
  Comparer avec les subagents
</h3>

Les équipes d'agents et les [subagents](/docs/fr/sub-agents) vous permettent tous deux de paralléliser le travail, mais ils fonctionnent différemment. Choisissez en fonction de la nécessité pour vos travailleurs de communiquer les uns avec les autres :

<Frame caption="Les subagents ne rendent compte que des résultats à l'agent principal et ne se parlent jamais. Dans les équipes d'agents, les coéquipiers partagent une liste de tâches, revendiquent du travail et communiquent directement les uns avec les autres.">
  <img src="https://mintcdn.com/claude-code/nsvRFSDNfpSU5nT7/images/subagents-vs-agent-teams-light.png?fit=max&auto=format&n=nsvRFSDNfpSU5nT7&q=85&s=2f8db9b4f3705dd3ab931fbe2d96e42a" className="dark:hidden" alt="Diagramme comparant les architectures des subagents et des équipes d'agents. Les subagents sont générés par l'agent principal, font du travail et rendent compte des résultats. Les équipes d'agents se coordonnent via une liste de tâches partagée, avec les coéquipiers communiquant directement les uns avec les autres." width="4245" height="1615" data-path="images/subagents-vs-agent-teams-light.png" />

  <img src="https://mintcdn.com/claude-code/nsvRFSDNfpSU5nT7/images/subagents-vs-agent-teams-dark.png?fit=max&auto=format&n=nsvRFSDNfpSU5nT7&q=85&s=d573a037540f2ada6a9ae7d8285b46fd" className="hidden dark:block" alt="Diagramme comparant les architectures des subagents et des équipes d'agents. Les subagents sont générés par l'agent principal, font du travail et rendent compte des résultats. Les équipes d'agents se coordonnent via une liste de tâches partagée, avec les coéquipiers communiquant directement les uns avec les autres." width="4245" height="1615" data-path="images/subagents-vs-agent-teams-dark.png" />
</Frame>

|                    | Subagents                                                          | Équipes d'agents                                                |
| :----------------- | :----------------------------------------------------------------- | :-------------------------------------------------------------- |
| **Contexte**       | Fenêtre de contexte propre ; les résultats reviennent à l'appelant | Fenêtre de contexte propre ; complètement indépendant           |
| **Communication**  | Rendre compte uniquement à l'agent principal                       | Les coéquipiers se messagent directement                        |
| **Coordination**   | L'agent principal gère tout le travail                             | Liste de tâches partagée avec auto-coordination                 |
| **Meilleur pour**  | Les tâches ciblées où seul le résultat compte                      | Le travail complexe nécessitant discussion et collaboration     |
| **Coût en tokens** | Inférieur : les résultats sont résumés au contexte principal       | Supérieur : chaque coéquipier est une instance Claude distincte |

Utilisez les subagents lorsque vous avez besoin de travailleurs rapides et ciblés qui rendent compte. Utilisez les équipes d'agents lorsque les coéquipiers doivent partager les conclusions, se contester mutuellement et se coordonner de manière autonome.

<h2 id="enable-agent-teams">
  Activer les équipes d'agents
</h2>

Les équipes d'agents sont désactivées par défaut. Activez-les en définissant la variable d'environnement `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` sur `1`, soit dans votre environnement shell, soit via [settings.json](/docs/fr/settings) :

```json settings.json theme={null}
{
  "env": {
    "CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS": "1"
  }
}
```

<h2 id="start-your-first-agent-team">
  Démarrer votre première équipe d'agents
</h2>

Après avoir activé les équipes d'agents, décrivez la tâche et les coéquipiers que vous souhaitez en langage naturel. Claude les crée et coordonne le travail en fonction de votre prompt.

Cet exemple fonctionne bien car les trois rôles sont indépendants et peuvent explorer le problème sans attendre les uns les autres :

```text theme={null}
Je conçois un outil CLI qui aide les développeurs à suivre les commentaires TODO dans
leur base de code. Créez trois coéquipiers pour explorer cela sous différents angles :
un sur l'UX, un sur l'architecture technique, un jouant l'avocat du diable.
```

À partir de là, Claude remplit une [liste de tâches partagée](/docs/fr/interactive-mode#task-list), crée des coéquipiers pour chaque perspective, les fait explorer le problème, et synthétise les conclusions une fois terminé.

Le terminal du chef liste les coéquipiers dans le panneau d'agents en dessous de l'entrée du prompt. À partir du panneau :

* **Flèches haut et bas** : sélectionner un coéquipier
* **Entrée** : ouvrir la transcription du coéquipier sélectionné et lui envoyer un message directement
* **Échap** : interrompre le tour actuel du coéquipier sélectionné

À partir de la v2.1.199, la ligne d'un coéquipier inactif reste dans le panneau tant que n'importe quel coéquipier ou sous-agent travaille encore, vous pouvez donc la sélectionner pour examiner sa transcription ou lui envoyer plus de travail. Une fois que chaque agent du panneau est inactif, les lignes inactives se masquent après 30 secondes et réapparaissent au prochain tour du coéquipier ; le coéquipier continue de fonctionner et reste adressable pendant qu'il est masqué. Dans les versions v2.1.181 à v2.1.198, une ligne inactif s'est masquée 30 secondes après la fin de son propre tour, même si d'autres coéquipiers travaillaient encore ; les lignes inactives ne sont pas masquées dans les versions antérieures à v2.1.181.

Lorsque plus de trois coéquipiers sont inactifs à la fois, les lignes au-delà des trois premières se réduisent en une seule ligne qui compte les coéquipiers réduits, comme `2 agents inactifs` quand cinq sont inactifs. Sélectionnez-la et appuyez sur Entrée pour développer les lignes réduites, ou appuyez sur Échap pour les réduire à nouveau. Les coéquipiers qui travaillent, les coéquipiers qui ont échoué, et le coéquipier que vous consultez conservent toujours leurs propres lignes.

Si vous souhaitez que chaque coéquipier soit dans son propre volet divisé, consultez [Choisir un mode d'affichage](#choose-a-display-mode).

<h2 id="control-your-agent-team">
  Contrôler votre équipe d'agents
</h2>

Dites au chef ce que vous voulez en langage naturel. Il gère la coordination d'équipe, l'assignation de tâches et la délégation en fonction de vos instructions.

<h3 id="choose-a-display-mode">
  Choisir un mode d'affichage
</h3>

Les équipes d'agents supportent deux modes d'affichage :

* **In-process** : tous les coéquipiers s'exécutent dans votre terminal principal. Utilisez les touches fléchées haut et bas dans le panneau d'agent pour sélectionner un coéquipier, puis appuyez sur Entrée pour l'afficher et tapez pour lui envoyer un message directement. Fonctionne dans n'importe quel terminal, aucune configuration supplémentaire requise.
* **Volets divisés** : chaque coéquipier obtient son propre volet. Vous pouvez voir la sortie de tout le monde à la fois et cliquer dans un volet pour interagir directement. Nécessite tmux ou iTerm2.

<Note>
  `tmux` a des limitations connues sur certains systèmes d'exploitation et fonctionne traditionnellement mieux sur macOS. L'utilisation de `tmux -CC` dans iTerm2 est le point d'entrée suggéré dans `tmux`.
</Note>

La valeur par défaut est `"in-process"`. Avant la v2.1.179, la valeur par défaut était `"auto"`, donc les sessions mises à niveau qui ouvraient précédemment des volets divisés restent maintenant dans un terminal sauf si vous définissez le mode explicitement. Définissez `"auto"` pour activer les volets divisés lorsque vous êtes déjà en train de s'exécuter dans une session tmux ou si votre terminal est iTerm2, en revenant à in-process sinon. Le paramètre `"tmux"` active le mode volets divisés et détecte automatiquement s'il faut utiliser tmux ou iTerm2 en fonction de votre terminal.

À partir de la v2.1.186, définissez `"iterm2"` pour utiliser explicitement les volets divisés natifs d'iTerm2. Ce mode nécessite le CLI [`it2`](https://github.com/mkusaka/it2) et affiche une erreur avec la commande d'installation si `it2` est manquant. L'invite de configuration qui propose d'installer `it2` ou de basculer vers tmux apparaît sous `"auto"` ou `"tmux"` lorsque votre terminal est iTerm2 et que tmux est disponible comme solution de secours.

Pour remplacer la valeur par défaut, définissez [`teammateMode`](/docs/fr/settings#available-settings) dans `~/.claude/settings.json` :

```json theme={null}
{
  "teammateMode": "auto"
}
```

Pour définir le mode pour une seule session, passez-le en tant que drapeau :

```bash theme={null}
claude --teammate-mode auto
```

Le mode volets divisés nécessite soit [tmux](https://github.com/tmux/tmux/wiki) soit iTerm2 avec le CLI [`it2`](https://github.com/mkusaka/it2). Pour installer manuellement :

* **tmux** : installez via le gestionnaire de paquets de votre système. Consultez le [wiki tmux](https://github.com/tmux/tmux/wiki/Installing) pour les instructions spécifiques à la plateforme.
* **iTerm2** : installez le CLI [`it2`](https://github.com/mkusaka/it2), puis activez l'API Python dans **iTerm2 → Paramètres → Général → Magie → Activer l'API Python**.

<h3 id="specify-teammates-and-models">
  Spécifier les coéquipiers et les modèles
</h3>

Claude décide du nombre de coéquipiers à générer en fonction de votre tâche, ou vous pouvez spécifier exactement ce que vous voulez :

```text theme={null}
Générez 4 coéquipiers pour refactoriser ces modules en parallèle. Utilisez
Sonnet pour chaque coéquipier.
```

Les coéquipiers n'héritent pas de la sélection `/model` du chef par défaut. Pour modifier le modèle utilisé lorsque l'invite ne spécifie pas un, définissez **Modèle de coéquipier par défaut** dans `/config`. Choisissez **Par défaut (modèle du chef)** pour que les coéquipiers suivent le modèle actuel du chef.

Les coéquipiers héritent du [niveau d'effort](/docs/fr/model-config#adjust-effort-level) du chef. En mode volets divisés, cela s'applique à partir de la v2.1.186 ; les versions antérieures ne transmettaient pas l'effort de session du chef aux coéquipiers en mode volets divisés.

<h3 id="require-plan-approval-for-teammates">
  Exiger l'approbation du plan pour les coéquipiers
</h3>

Pour les tâches complexes ou risquées, vous pouvez exiger que les coéquipiers planifient avant de mettre en œuvre. Le coéquipier travaille en mode plan en lecture seule jusqu'à ce que le chef approuve son approche :

```text theme={null}
Générez un coéquipier architecte pour refactoriser le module d'authentification.
Exigez l'approbation du plan avant qu'il ne fasse des modifications.
```

Lorsqu'un coéquipier termine la planification, il envoie une demande d'approbation du plan au chef. Le chef examine le plan et l'approuve ou le rejette avec des commentaires. S'il est rejeté, le coéquipier reste en mode plan, révise en fonction des commentaires et resoumis. Une fois approuvé, le coéquipier quitte le mode plan et commence la mise en œuvre.

Le chef prend les décisions d'approbation de manière autonome. Pour influencer le jugement du chef, donnez-lui des critères dans votre invite, tels que « n'approuvez que les plans qui incluent la couverture de test » ou « rejetez les plans qui modifient le schéma de base de données ».

<h3 id="talk-to-teammates-directly">
  Parler directement aux coéquipiers
</h3>

Chaque coéquipier est une session Claude Code complète et indépendante. Vous pouvez envoyer un message à n'importe quel coéquipier directement pour donner des instructions supplémentaires, poser des questions de suivi ou rediriger son approche.

* **Mode in-process** : utilisez les touches fléchées haut et bas dans le panneau d'agent pour sélectionner un coéquipier, puis appuyez sur Entrée pour afficher sa session et tapez pour lui envoyer un message. Appuyez sur `x` sur un coéquipier sélectionné pour l'arrêter. Appuyez sur Ctrl+T pour basculer la liste des tâches.
* **Mode volets divisés** : cliquez dans le volet d'un coéquipier pour interagir directement avec sa session. Chaque coéquipier a une vue complète de son propre terminal.

Pendant que vous visualisez un coéquipier in-process, le texte brut et les [skills](/docs/fr/skills) vont à ce coéquipier, mais les commandes intégrées s'exécutent toujours dans la session du chef.

Le modèle et le mode rapide d'un coéquipier sont fixés lorsqu'il est généré, donc `/model` et `/fast` ne changent que les paramètres du chef. À partir de la v2.1.199, taper l'une ou l'autre commande en visualisant un coéquipier affiche un avis indiquant que le changement s'applique au chef ; les versions antérieures l'appliquaient au chef sans indication. `/effort` s'applique toujours aux tours ultérieurs du coéquipier visualisé, car les coéquipiers suivent le [niveau d'effort](/docs/fr/model-config#adjust-effort-level) du chef.

<h3 id="assign-and-claim-tasks">
  Assigner et revendiquer des tâches
</h3>

La liste de tâches partagée coordonne le travail dans l'équipe. Le chef crée des tâches et les coéquipiers les accomplissent. Les tâches ont trois états : en attente, en cours et terminées. Les tâches peuvent également dépendre d'autres tâches : une tâche en attente avec des dépendances non résolues ne peut pas être revendiquée jusqu'à ce que ces dépendances soient complétées.

Le chef peut assigner des tâches explicitement, ou les coéquipiers peuvent les revendiquer eux-mêmes :

* **Le chef assigne** : dites au chef quelle tâche donner à quel coéquipier
* **Auto-revendication** : après avoir terminé une tâche, un coéquipier choisit la prochaine tâche non assignée et non bloquée de sa propre initiative

La revendication de tâche utilise le verrouillage de fichiers pour prévenir les conditions de course lorsque plusieurs coéquipiers tentent de revendiquer la même tâche simultanément.

<h3 id="shut-down-teammates">
  Arrêter les coéquipiers
</h3>

Pour terminer gracieusement la session d'un coéquipier, référencez-le par son nom. Par exemple, avec un coéquipier nommé chercheur :

```text theme={null}
Demandez au coéquipier chercheur d'arrêter
```

Le chef envoie une demande d'arrêt. Le coéquipier peut approuver, quittant gracieusement, ou rejeter avec une explication.

Les répertoires partagés de l'équipe sont nettoyés automatiquement lorsque la session se termine, il n'y a donc pas d'étape de nettoyage séparé. Consultez [Architecture](#architecture) pour voir quels répertoires sont supprimés et lesquels persistent pour les sessions reprises.

<h3 id="enforce-quality-gates-with-hooks">
  Appliquer des portes de qualité avec des hooks
</h3>

Utilisez les [hooks](/docs/fr/hooks) pour appliquer des règles lorsque les coéquipiers terminent le travail ou que les tâches sont créées ou complétées :

* [`TeammateIdle`](/docs/fr/hooks#teammateidle) : s'exécute lorsqu'un coéquipier est sur le point de devenir inactif. Quittez avec le code 2 pour envoyer des commentaires et garder le coéquipier au travail.
* [`TaskCreated`](/docs/fr/hooks#taskcreated) : s'exécute lorsqu'une tâche est en cours de création. Quittez avec le code 2 pour empêcher la création et envoyer des commentaires.
* [`TaskCompleted`](/docs/fr/hooks#taskcompleted) : s'exécute lorsqu'une tâche est marquée comme complète. Quittez avec le code 2 pour empêcher la complétion et envoyer des commentaires.

<h2 id="how-agent-teams-work">
  Comment fonctionnent les équipes d'agents
</h2>

Cette section couvre l'architecture et la mécanique derrière les équipes d'agents. Si vous souhaitez commencer à les utiliser, consultez [Contrôler votre équipe d'agents](#control-your-agent-team) ci-dessus.

<h3 id="how-claude-starts-agent-teams">
  Comment Claude démarre les équipes d'agents
</h3>

Une équipe d'agents se forme lorsque le premier coéquipier est généré, la session principale agissant comme le chef. Il y a deux façons dont les coéquipiers sont générés :

* **Vous demandez des coéquipiers** : donnez à Claude une tâche qui bénéficie du travail parallèle et demandez explicitement des coéquipiers. Claude les génère en fonction de vos instructions.
* **Claude propose des coéquipiers** : si Claude détermine que votre tâche bénéficierait du travail parallèle, il peut suggérer de générer des coéquipiers. Vous confirmez avant qu'il ne procède.

Dans les deux cas, vous restez maître. Claude ne générera pas de coéquipiers sans votre approbation.

<h3 id="architecture">
  Architecture
</h3>

Une équipe d'agents se compose de :

| Composant             | Rôle                                                                                  |
| :-------------------- | :------------------------------------------------------------------------------------ |
| **Chef d'équipe**     | La session Claude Code principale qui génère les coéquipiers et coordonne le travail  |
| **Coéquipiers**       | Des instances Claude Code distinctes qui travaillent chacune sur des tâches assignées |
| **Liste de tâches**   | Liste partagée d'éléments de travail que les coéquipiers revendiquent et complètent   |
| **Boîte aux lettres** | Système de messagerie pour la communication entre agents                              |

Consultez [Choisir un mode d'affichage](#choose-a-display-mode) pour les options de configuration d'affichage. Les messages des coéquipiers arrivent au chef automatiquement.

La boîte aux lettres de chaque agent est un fichier JSON à `~/.claude/teams/{team-name}/inboxes/{agent-name}.json`. Claude Code valide chaque entrée lorsqu'il lit un fichier de boîte aux lettres. Les entrées qui ne correspondent pas au format de message sont signalées comme des erreurs et supprimées du fichier ; les messages valides sont toujours livrés. Avant la v2.1.207, une seule entrée de boîte aux lettres malformée causait une erreur répétée chaque seconde et bloquait la livraison pour cette boîte aux lettres jusqu'à ce que vous supprimiez le fichier manuellement.

Le système gère automatiquement les dépendances de tâches. Lorsqu'un coéquipier complète une tâche dont d'autres tâches dépendent, les tâches bloquées se débloquent sans intervention manuelle.

Les équipes et les tâches sont stockées localement sous un nom dérivé de la session. Le nom est `session-` suivi des huit premiers caractères de l'ID de session :

* **Configuration d'équipe** : `~/.claude/teams/{team-name}/config.json`
* **Liste de tâches** : `~/.claude/tasks/{team-name}/`

Claude Code génère automatiquement ces deux éléments au démarrage de la session et les met à jour à mesure que les coéquipiers rejoignent, deviennent inactifs ou partent. Le répertoire de configuration d'équipe est supprimé lorsque la session se termine. Le répertoire de liste de tâches persiste localement et n'est jamais téléchargé, donc les sessions reprises conservent leurs tâches. La rétention est régie par le même [`cleanupPeriodDays`](/docs/fr/settings#available-settings) que vous contrôlez déjà pour les transcriptions de session.

La configuration d'équipe contient l'état d'exécution tel que les ID de session et les ID de volet tmux, donc ne l'éditez pas à la main ou ne la pré-créez pas : vos modifications sont écrasées lors de la prochaine mise à jour d'état.

Pour définir des rôles de coéquipiers réutilisables, utilisez plutôt les [définitions de subagents](#use-subagent-definitions-for-teammates).

La configuration d'équipe contient un tableau `members` avec le nom de chaque coéquipier, l'ID d'agent et le type d'agent. Les coéquipiers peuvent lire ce fichier pour découvrir les autres membres de l'équipe.

Il n'y a pas d'équivalent au niveau du projet de la configuration d'équipe. Un fichier comme `.claude/teams/teams.json` dans votre répertoire de projet n'est pas reconnu comme configuration ; Claude le traite comme un fichier ordinaire.

<h3 id="use-subagent-definitions-for-teammates">
  Utiliser les définitions de subagents pour les coéquipiers
</h3>

Lors de la génération d'un coéquipier, vous pouvez référencer un type de [subagent](/docs/fr/sub-agents) de n'importe quelle [portée de subagent](/docs/fr/sub-agents#choose-the-subagent-scope) : projet, utilisateur, plugin ou défini par CLI. Cela vous permet de définir un rôle une fois, comme un examinateur de sécurité ou un exécuteur de tests, et de le réutiliser à la fois comme subagent délégué et comme coéquipier d'équipe d'agents.

Pour utiliser une définition de subagent, mentionnez-la par nom lorsque vous demandez à Claude de générer le coéquipier :

```text theme={null}
Générez un coéquipier utilisant le type d'agent security-reviewer pour auditer le module d'authentification.
```

Le coéquipier honore les restrictions de la liste d'outils de cette définition et le modèle, et le corps de la définition est ajouté au prompt système du coéquipier en tant qu'instructions supplémentaires plutôt que de le remplacer. Les outils de coordination d'équipe tels que `SendMessage` et les outils de gestion des tâches sont toujours disponibles pour un coéquipier même lorsque `tools` restreint d'autres outils.

<Note>
  Les champs frontmatter `skills` et `mcpServers` dans une définition de subagent ne sont pas appliqués lorsque cette définition s'exécute en tant que coéquipier. Les coéquipiers chargent les skills et les serveurs MCP à partir de vos paramètres de projet et d'utilisateur, comme une session régulière.
</Note>

<h3 id="permissions">
  Permissions
</h3>

Les coéquipiers commencent avec les paramètres de permission du chef. Si le chef s'exécute avec `--dangerously-skip-permissions`, tous les coéquipiers le font aussi. Après la génération, vous pouvez modifier les modes de coéquipiers individuels, mais vous ne pouvez pas définir les modes par coéquipier au moment de la génération.

Lorsqu'un agent envoie un message à un autre via `SendMessage`, l'agent destinataire est informé qu'il provient d'une autre session Claude, et non de vous. Un coéquipier ne peut pas approuver une invite de permission ou fournir un consentement en votre nom, et un coéquipier auquel une action a été refusée ne peut pas la relayer à un autre coéquipier pour contourner la vérification. En [mode automatique](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode), le classificateur traite une approbation relayée par un autre agent comme une entrée non fiable plutôt que comme une confirmation de votre part.

Les invites de permission des coéquipiers remontent à la session chef, donc approuvez-les vous-même là-bas. [L'approbation du plan](#require-plan-approval-for-teammates) est l'exception conçue : la session chef accorde les approbations de plan des coéquipiers sans une invite séparée pour vous.

<h3 id="context-and-communication">
  Contexte et communication
</h3>

Chaque coéquipier a sa propre fenêtre de contexte. Lorsqu'il est généré, un coéquipier charge le même contexte de projet qu'une session régulière : CLAUDE.md, serveurs MCP et skills. Il reçoit également le prompt de génération du chef. L'historique de conversation du chef ne se transporte pas.

**Comment les coéquipiers partagent les informations :**

* **Livraison automatique de messages** : lorsque les coéquipiers envoient des messages, ils sont livrés automatiquement aux destinataires. Le chef n'a pas besoin d'interroger les mises à jour.
* **Notifications d'inactivité** : lorsqu'un coéquipier termine et s'arrête, il notifie automatiquement le chef. À partir de la v2.1.198, un coéquipier dont le tour se termine sur une erreur API notifie le chef qu'il a échoué et inclut le texte d'erreur, au lieu d'apparaître comme terminé normalement.
* **Liste de tâches partagée** : tous les agents peuvent voir l'état des tâches et revendiquer le travail disponible.
* **Messagerie des coéquipiers** : envoyer un message à un coéquipier spécifique par son nom. Pour atteindre tout le monde, envoyez un message par destinataire.

Le chef assigne à chaque coéquipier un nom lorsqu'il le génère, et n'importe quel coéquipier peut envoyer un message à n'importe quel autre par ce nom. Pour obtenir des noms prévisibles que vous pouvez référencer dans les prompts ultérieurs, dites au chef comment appeler chaque coéquipier dans votre instruction de génération.

<h3 id="token-usage">
  Utilisation des tokens
</h3>

Les équipes d'agents utilisent considérablement plus de tokens qu'une seule session. Chaque coéquipier a sa propre fenêtre de contexte, et l'utilisation des tokens augmente avec le nombre de coéquipiers actifs. Pour la recherche, l'examen et le travail sur les nouvelles fonctionnalités, les tokens supplémentaires en valent généralement la peine. Pour les tâches de routine, une seule session est plus rentable. Consultez les [coûts des tokens des équipes d'agents](/docs/fr/costs#agent-team-token-costs) pour les conseils d'utilisation.

<h2 id="use-case-examples">
  Exemples de cas d'usage
</h2>

Ces exemples montrent comment les équipes d'agents gèrent les tâches où l'exploration parallèle ajoute de la valeur.

<h3 id="run-a-parallel-code-review">
  Exécuter un examen de code parallèle
</h3>

Un seul examinateur tend à graviter vers un type de problème à la fois. Diviser les critères d'examen en domaines indépendants signifie que la sécurité, l'impact sur les performances et la couverture de test reçoivent tous une attention approfondie simultanément. Le prompt assigne à chaque coéquipier une lentille distincte pour qu'ils ne se chevauchent pas :

```text theme={null}
Spawn three teammates to review PR #142:
- One focused on security implications
- One checking performance impact
- One validating test coverage
Have them each review and report findings.
```

Chaque examinateur travaille à partir de la même PR mais applique un filtre différent. Le chef synthétise les conclusions de tous les trois après qu'ils aient terminé.

<h3 id="investigate-with-competing-hypotheses">
  Enquêter avec des hypothèses concurrentes
</h3>

Lorsque la cause première est peu claire, un seul agent tend à trouver une explication plausible et s'arrête. Le prompt combat cela en rendant les coéquipiers explicitement adversaires : le travail de chacun n'est pas seulement d'enquêter sur sa propre théorie mais de contester les autres.

```text theme={null}
Users report the app exits after one message instead of staying connected.
Spawn 5 agent teammates to investigate different hypotheses. Have them talk to
each other to try to disprove each other's theories, like a scientific
debate. Update the findings doc with whatever consensus emerges.
```

La structure du débat est le mécanisme clé ici. L'enquête séquentielle souffre de l'ancrage : une fois qu'une théorie est explorée, l'enquête ultérieure est biaisée vers elle.

Avec plusieurs enquêteurs indépendants essayant activement de réfuter les uns les autres, la théorie qui survit est beaucoup plus susceptible d'être la cause première réelle.

<h2 id="best-practices">
  Meilleures pratiques
</h2>

<h3 id="give-teammates-enough-context">
  Donner aux coéquipiers suffisamment de contexte
</h3>

Les coéquipiers chargent automatiquement le contexte du projet, y compris CLAUDE.md, serveurs MCP et skills, mais ils n'héritent pas de l'historique de conversation du chef. Consultez [Contexte et communication](#context-and-communication) pour les détails. Incluez les détails spécifiques à la tâche dans le prompt de génération :

```text theme={null}
Générez un coéquipier examinateur de sécurité avec le prompt : « Examinez le module d'authentification
à src/auth/ pour les vulnérabilités de sécurité. Concentrez-vous sur la gestion des tokens, la gestion
des sessions et la validation des entrées. L'application utilise des tokens JWT stockés dans
des cookies httpOnly. Signalez tout problème avec les évaluations de gravité. »
```

<h3 id="choose-an-appropriate-team-size">
  Choisir une taille d'équipe appropriée
</h3>

Il n'y a pas de limite stricte au nombre de coéquipiers, mais des contraintes pratiques s'appliquent :

* **Les coûts des tokens augmentent linéairement** : chaque coéquipier a sa propre fenêtre de contexte et consomme des tokens indépendamment. Consultez les [coûts des tokens des équipes d'agents](/docs/fr/costs#agent-team-token-costs) pour les détails.
* **La surcharge de coordination augmente** : plus de coéquipiers signifie plus de communication, de coordination de tâches et de risques de conflits
* **Rendements décroissants** : au-delà d'un certain point, les coéquipiers supplémentaires n'accélèrent pas le travail proportionnellement

Commencez avec 3 à 5 coéquipiers pour la plupart des flux de travail. Cela équilibre le travail parallèle avec une coordination gérable. Les exemples de ce guide utilisent 3 à 5 coéquipiers car cette plage fonctionne bien dans différents types de tâches.

Avoir 5 à 6 [tâches](/docs/fr/agent-teams#architecture) par coéquipier garde tout le monde productif sans changement de contexte excessif. Si vous avez 15 tâches indépendantes, 3 coéquipiers est un bon point de départ.

Augmentez l'échelle uniquement lorsque le travail bénéficie véritablement d'avoir des coéquipiers travaillant simultanément. Trois coéquipiers ciblés surpassent souvent cinq dispersés.

<h3 id="size-tasks-appropriately">
  Dimensionner les tâches de manière appropriée
</h3>

* **Trop petites** : la surcharge de coordination dépasse le bénéfice
* **Trop grandes** : les coéquipiers travaillent trop longtemps sans points de contrôle, augmentant le risque d'effort gaspillé
* **Juste bien** : des unités autonomes qui produisent un livrable clair, comme une fonction, un fichier de test ou un examen

<Tip>
  Le chef divise le travail en tâches et les assigne aux coéquipiers automatiquement. S'il ne crée pas assez de tâches, demandez-lui de diviser le travail en morceaux plus petits. Avoir 5 à 6 tâches par coéquipier garde tout le monde productif et permet au chef de réassigner le travail si quelqu'un est bloqué.
</Tip>

<h3 id="wait-for-teammates-to-finish">
  Attendre que les coéquipiers terminent
</h3>

Parfois, le chef commence à mettre en œuvre des tâches lui-même au lieu d'attendre les coéquipiers. Si vous remarquez cela :

```text theme={null}
Attendez que vos coéquipiers complètent leurs tâches avant de procéder
```

<h3 id="start-with-research-and-review">
  Commencer par la recherche et l'examen
</h3>

Si vous êtes nouveau aux équipes d'agents, commencez par des tâches qui ont des limites claires et ne nécessitent pas d'écrire du code : examiner une PR, rechercher une bibliothèque ou enquêter sur un bug. Ces tâches montrent la valeur de l'exploration parallèle sans les défis de coordination qui accompagnent la mise en œuvre parallèle.

<h3 id="avoid-file-conflicts">
  Éviter les conflits de fichiers
</h3>

Deux coéquipiers éditant le même fichier entraîne des écrasements. Divisez le travail pour que chaque coéquipier possède un ensemble de fichiers différent.

<h3 id="monitor-and-steer">
  Surveiller et diriger
</h3>

Vérifiez la progression des coéquipiers, redirigez les approches qui ne fonctionnent pas et synthétisez les conclusions au fur et à mesure qu'elles arrivent. Laisser une équipe s'exécuter sans surveillance pendant trop longtemps augmente le risque d'effort gaspillé.

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="teammates-not-appearing">
  Les coéquipiers n'apparaissent pas
</h3>

Si les coéquipiers n'apparaissent pas après avoir demandé à Claude de créer une équipe :

* En mode in-process, les coéquipiers apparaissent dans le panneau d'agent sous l'entrée de prompt. Utilisez les touches fléchées haut et bas pour en sélectionner un, puis appuyez sur Entrée pour l'afficher.
* Une ligne de coéquipier qui a disparu après être restée inactive a été masquée, non arrêtée. Les lignes inactives se masquent 30 secondes après que le panneau entier devienne inactif et réapparaissent au prochain tour du coéquipier. Quand plus de trois coéquipiers sont inactifs, leurs lignes excédentaires s'effondrent en une seule ligne `N idle agents` que Entrée développe. Envoyez un message au coéquipier par son nom pour ramener une ligne masquée.
* Vérifiez que la tâche que vous avez donnée à Claude était suffisamment complexe pour justifier une équipe. Claude décide s'il faut générer des coéquipiers en fonction de la tâche.
* Si vous avez explicitement demandé des volets divisés, assurez-vous que tmux est installé et disponible dans votre PATH :
  ```bash theme={null}
  which tmux
  ```
* Pour iTerm2, vérifiez que le CLI `it2` est installé et que l'API Python est activée dans les préférences d'iTerm2.

<h3 id="too-many-permission-prompts">
  Trop de demandes de permission
</h3>

Les demandes de permission des coéquipiers remontent au chef, ce qui peut créer des frictions. Pré-approuvez les opérations courantes dans vos [paramètres de permission](/docs/fr/permissions) avant de générer les coéquipiers pour réduire les interruptions.

<h3 id="teammates-stopping-on-errors">
  Les coéquipiers s'arrêtent sur les erreurs
</h3>

Les coéquipiers peuvent s'arrêter après avoir rencontré des erreurs au lieu de se rétablir. Vérifiez leur sortie en sélectionnant le coéquipier dans le panneau d'agent et en appuyant sur Entrée en mode in-process, ou en cliquant sur le volet en mode divisé, puis :

* Donnez-leur des instructions supplémentaires directement
* Générez un coéquipier de remplacement pour continuer le travail

À partir de la v2.1.198, un message du chef ou d'un autre coéquipier réveille un coéquipier in-process qui attend de réessayer une demande API échouée, il réessaie donc immédiatement au lieu d'attendre le délai de réessai complet.

<h3 id="lead-shuts-down-before-work-is-done">
  Le chef s'arrête avant que le travail ne soit terminé
</h3>

Le chef peut décider que l'équipe est terminée avant que toutes les tâches ne soient réellement complètes. Si cela se produit, dites-lui de continuer. Vous pouvez également dire au chef d'attendre que les coéquipiers terminent avant de procéder s'il commence à faire du travail au lieu de déléguer.

<h3 id="orphaned-tmux-sessions">
  Sessions tmux orphelines
</h3>

Si une session tmux persiste après la fin de l'équipe, elle peut ne pas avoir été complètement nettoyée. Listez les sessions et tuez celle créée par l'équipe :

```bash theme={null}
tmux ls
tmux kill-session -t <session-name>
```

<h2 id="limitations">
  Limitations
</h2>

Les équipes d'agents sont expérimentales. Les limitations actuelles à connaître :

* **Pas de reprise de session avec les coéquipiers in-process** : `/resume` et `/rewind` ne restaurent pas les coéquipiers in-process. Après la reprise d'une session, le chef peut tenter d'envoyer un message aux coéquipiers qui n'existent plus. Si cela se produit, dites au chef de générer de nouveaux coéquipiers.
* **L'état des tâches peut être en retard** : les coéquipiers échouent parfois à marquer les tâches comme complètes, ce qui bloque les tâches dépendantes. Si une tâche semble bloquée, vérifiez si le travail est réellement terminé et mettez à jour l'état de la tâche manuellement ou dites au chef de pousser le coéquipier.
* **L'arrêt peut être lent** : les coéquipiers terminent leur demande actuelle ou appel d'outil avant de s'arrêter, ce qui peut prendre du temps.
* **Une équipe par session** : une session a exactement une équipe, limitée à cette session. Vous ne pouvez pas créer d'équipes nommées supplémentaires ou partager une équipe entre les sessions.
* **Pas d'équipes imbriquées** : les coéquipiers ne peuvent pas générer leurs propres coéquipiers. Seul le chef peut gérer l'équipe.
* **Pas de sous-agents d'arrière-plan à partir de coéquipiers in-process** : les propres sous-agents d'un coéquipier in-process s'exécutent au premier plan. Demander un sous-agent d'arrière-plan, que ce soit avec `run_in_background` ou une définition de sous-agent qui définit `background: true`, retourne une erreur, car le travail d'arrière-plan d'un coéquipier ne peut pas survivre au processus du chef. Les sous-agents lancés à partir de la conversation principale suivent la [valeur par défaut d'arrière-plan](/docs/fr/sub-agents#run-subagents-in-foreground-or-background).
* **Le chef est fixe** : la session principale est le chef pour sa durée de vie. Vous ne pouvez pas promouvoir un coéquipier en chef ou transférer le leadership.
* **Permissions définies au moment de la génération** : tous les coéquipiers commencent avec le mode de permission du chef. Vous pouvez modifier les modes de coéquipiers individuels après la génération, mais vous ne pouvez pas définir les modes par coéquipier au moment de la génération.
* **Les volets divisés nécessitent tmux ou iTerm2** : le mode in-process par défaut fonctionne dans n'importe quel terminal. Le mode volets divisés n'est pas supporté dans le terminal intégré de VS Code, Windows Terminal ou Ghostty.

<Tip>
  **`CLAUDE.md` fonctionne normalement** : les coéquipiers lisent les fichiers `CLAUDE.md` de leur répertoire de travail. Utilisez ceci pour fournir des conseils spécifiques au projet à tous les coéquipiers.
</Tip>

<h2 id="next-steps">
  Prochaines étapes
</h2>

Explorez les approches connexes pour le travail parallèle et la délégation :

* **Délégation légère** : les [subagents](/docs/fr/sub-agents) génèrent des agents auxiliaires pour la recherche ou la vérification au sein de votre session, mieux pour les tâches qui n'ont pas besoin de coordination inter-agents
* **Sessions parallèles manuelles** : les [Git worktrees](/docs/fr/worktrees) vous permettent d'exécuter plusieurs sessions Claude Code vous-même sans coordination d'équipe automatisée
* **Comparer les approches** : consultez la comparaison [subagent vs équipe d'agents](/docs/fr/features-overview#compare-similar-features) pour une répartition côte à côte
