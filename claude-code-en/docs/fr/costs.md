> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gérer les coûts efficacement

> Suivez l'utilisation des tokens, définissez des limites de dépenses pour l'équipe, et réduisez les coûts de Claude Code grâce à la gestion du contexte, la sélection du modèle, les paramètres de réflexion étendue et les hooks de prétraitement.

Claude Code facture selon la consommation de tokens API. Pour les tarifs des plans d'abonnement (Pro, Max, Team, Enterprise), consultez [claude.com/pricing](https://claude.com/pricing). Les coûts par développeur varient considérablement en fonction de la sélection du modèle, de la taille de la base de code et des modèles d'utilisation tels que l'exécution de plusieurs instances ou l'automatisation.

Dans les déploiements d'entreprise, le coût moyen est d'environ 13 $par développeur par jour actif et de 150 à 250$ par développeur par mois, les coûts restant en dessous de 30 \$ par jour actif pour 90 % des utilisateurs. Pour estimer les dépenses de votre équipe, commencez par un petit groupe pilote et utilisez les outils de suivi ci-dessous pour établir une base de référence avant un déploiement plus large.

Cette page explique comment [suivre vos coûts](#track-your-costs), [gérer les coûts pour votre organisation](#manage-costs-for-your-organization) et [réduire l'utilisation des tokens](#reduce-token-usage).

<h2 id="track-your-costs">
  Suivre vos coûts
</h2>

<h3 id="using-the-/usage-command">
  Utiliser la commande `/usage`
</h3>

<Note>
  Le bloc Session dans `/usage` affiche l'utilisation des tokens API et est destiné aux utilisateurs d'API. Les abonnés Claude Max et Pro ont l'utilisation incluse dans leur abonnement, donc le chiffre du coût de session n'est pas pertinent à des fins de facturation. Les abonnés voient les barres d'utilisation du plan, les statistiques d'activité et une ventilation de l'utilisation sur le même écran.
</Note>

Le bloc Session en haut de `/usage` affiche des statistiques détaillées sur l'utilisation des tokens pour votre session actuelle. Le chiffre en dollars est une estimation calculée localement à partir des décomptes de tokens et peut différer de votre facture réelle. Pour une facturation fiable, consultez la page Utilisation dans la [Console Claude](https://platform.claude.com/usage).

```text theme={null}
Total cost:            $0.55
Total duration (API):  6m 19.7s
Total duration (wall): 6h 33m 10.2s
Total code changes:    0 lines added, 0 lines removed
```

Sur un plan Pro, Max, Team ou Enterprise, `/usage` affiche également une ventilation de ce qui compte par rapport à vos limites de plan. Il attribue l'utilisation récente aux skills, subagents, plugins et serveurs MCP individuels, chacun étant affiché en pourcentage du total. Appuyez sur `d` ou `w` pour basculer entre les 24 dernières heures et les 7 derniers jours. Les chiffres sont approximatifs et calculés à partir de l'historique des sessions locales sur cette machine, donc l'utilisation d'autres appareils ou de claude.ai n'est pas incluse.

Lorsque la demande de vos limites de plan échoue, le plus souvent parce que le point de terminaison d'utilisation est limité en débit, `/usage` affiche les dernières barres d'utilisation qu'il a chargées sur cette machine au cours des 60 dernières minutes, ainsi qu'une note « Affichage de la dernière utilisation connue » indiquant depuis combien de temps ces données ont été récupérées. Appuyez sur `r` pour réessayer ; une nouvelle tentative réussie remplace les dernières barres connues par des données actualisées. Sans un instantané des 60 dernières minutes, `/usage` signale que le point de terminaison d'utilisation est limité en débit et propose le même raccourci de nouvelle tentative. Avant la v2.1.208, une demande limitée en débit dans une session qui n'avait pas encore chargé l'utilisation affichait toujours l'erreur sans barres.

Dans l'[extension VS Code](/docs/fr/vs-code#check-account-and-usage), la même ventilation apparaît dans la boîte de dialogue Compte et utilisation avec un bouton bascule Jour et Semaine. Nécessite Claude Code v2.1.174 ou version ultérieure.

<h3 id="set-a-spend-limit-on-pro-and-max">
  Définir une limite de dépenses sur Pro et Max
</h3>

Sur les plans Pro et Max, la commande `/usage-credits` ouvre une boîte de dialogue dans la CLI où vous gérez les [crédits d'utilisation](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans). À partir de la boîte de dialogue, vous pouvez :

* Activer les crédits d'utilisation pour votre compte
* Acheter plus de crédits d'utilisation, soit un bundle répertorié, soit un montant personnalisé
* Définir, modifier ou supprimer votre limite de dépenses mensuelles
* Configurer le rechargement automatique, qui achète automatiquement plus de crédits d'utilisation lorsque votre solde tombe en dessous d'un seuil que vous définissez

Sur les versions de Claude Code antérieures à la v2.1.207 et sur les comptes où la boîte de dialogue dans la CLI n'est pas disponible, `/usage-credits` ouvre la page de facturation des crédits d'utilisation dans votre navigateur à la place. Sur les plans Team et Enterprise, les membres ayant accès à la facturation obtiennent la même page de navigateur, et les membres sans accès à la facturation envoient une demande depuis la CLI demandant à leur administrateur d'activer les crédits d'utilisation ou d'augmenter la limite.

La modification de la limite de dépenses mensuelles nécessite un accès à la facturation sur le compte. Si vous atteignez la limite alors que vous avez toujours des crédits d'utilisation disponibles, Claude Code vous invite à augmenter ou supprimer la limite afin que vous puissiez continuer sans quitter la CLI.

Les montants que vous tapez dans la boîte de dialogue, tels qu'un montant d'achat personnalisé, la limite de dépenses mensuelles ou le seuil et la cible de rechargement automatique, doivent être des chiffres, éventuellement suivis d'un point et d'une ou deux décimales, par exemple `20` ou `20.50`. Toute autre entrée, y compris les virgules, affiche une erreur en ligne et n'est pas enregistrée. Les versions antérieures à la v2.1.207 n'affichent pas la boîte de dialogue et ouvrent la page de facturation à la place.

Claude Code vous demande de taper `yes` pour confirmer chaque achat et chaque modification de rechargement automatique, quel que soit le montant, et la confirmation d'achat affiche le total après impôts que vous approuvez. La modification de la limite de dépenses mensuelles demande la même confirmation tapée uniquement au-dessus de 1 000 \$, ou au-dessus de 1 000 unités d'une devise de facturation non-USD. Avant la v2.1.208, les achats et les modifications de rechargement automatique utilisaient également ce seuil, donc les montants plus petits passaient par le flux de dialogue standard sans l'étape supplémentaire `yes` tapée.

Les champs de montant s'ouvrent préremplis avec une valeur suggérée, et le premier chiffre que vous tapez remplace la suggestion au lieu de s'y ajouter. L'écran qui active les crédits d'utilisation s'ouvre avec Annuler sélectionné, donc les activer nécessite une sélection délibérée plutôt qu'une Entrée accidentelle. Les deux nécessitent Claude Code v2.1.208 ou version ultérieure.

<h2 id="manage-costs-for-your-organization">
  Gérer les coûts pour votre organisation
</h2>

Les contrôles dont vous disposez dépendent de la façon dont votre organisation accède à Claude Code : un plan Claude for Teams ou Enterprise, la Claude Console, ou un fournisseur cloud. Sur les plans Teams et Enterprise, l'utilisation est prélevée sur l'allocation de siège de chaque membre. Sur la Console et chez les fournisseurs cloud, l'utilisation est facturée par token à votre organisation. Si votre organisation mélange les méthodes de connexion, chaque développeur est mesuré selon celle avec laquelle il s'est authentifié.

Le tableau mappe chaque configuration à l'endroit où vous voyez les dépenses, où vous les plafonnez, et comment vous extrayez les chiffres par utilisateur.

| Votre configuration                                                                     | Voir les dépenses                                                                                                                                          | Plafonner les dépenses                                   | Rapports par utilisateur                                                                                                                                                                                                              |
| :-------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [Claude for Teams ou Enterprise](#claude-for-teams-and-enterprise)                      | [Rapport de dépenses dans l'analyse organisationnelle](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) | Limites de dépenses dans les paramètres d'administration | [CSV du rapport de dépenses](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) ; [API d'analyse Enterprise](https://platform.claude.com/docs/en/api/admin/analytics) sur Enterprise |
| [Claude Console (API)](#claude-console)                                                 | [Page d'utilisation de la Console](https://platform.claude.com/usage)                                                                                      | Limites de dépenses de l'espace de travail               | [Tableau de bord de la Console](https://platform.claude.com/claude-code), [API d'analyse Claude Code](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api)                                                |
| [Amazon Bedrock, Google Cloud's Agent Platform, ou Microsoft Foundry](#cloud-providers) | Votre console de facturation cloud                                                                                                                         | Les contrôles budgétaires de votre cloud                 | [OpenTelemetry](/docs/fr/monitoring-usage) ou une [passerelle LLM](/docs/fr/llm-gateway)                                                                                                                                                        |

[L'export OpenTelemetry](/docs/fr/monitoring-usage) fonctionne sur chaque configuration et est la seule option qui diffuse les métriques de tokens et de coûts par utilisateur dans votre propre pile d'observabilité en temps quasi réel.

<h3 id="claude-for-teams-and-enterprise">
  Claude for Teams et Enterprise
</h3>

Sur les plans Claude for Teams et Enterprise, l'utilisation de Claude Code par chaque membre est prélevée sur une allocation par siège qui se réinitialise sur une fenêtre glissante de cinq heures et une fenêtre hebdomadaire. L'allocation est partagée avec Claude chat et Cowork, et sa taille dépend du [niveau de siège](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan) (Standard ou Premium). Vos contrôles se trouvent dans la console d'administration claude.ai, pas dans la Claude Console.

* **Voir les dépenses** : le [rapport de dépenses dans l'analyse organisationnelle](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) affiche les dépenses estimées par utilisateur et par modèle, avec export CSV, mis à jour quotidiennement. Le rapport couvre les dépenses en crédits d'utilisation et apparaît une fois que les crédits d'utilisation sont activés. L'utilisation dans l'allocation de siège n'est pas mesurée en dollars.
* **Voir l'adoption** : le [tableau de bord d'analyse](https://claude.ai/analytics/claude-code) affiche les utilisateurs actifs quotidiens, les sessions et les métriques de contribution, avec export CSV des données de contribution. Voir [suivre l'utilisation de l'équipe avec l'analyse](/docs/fr/analytics).
* **Plafonner les dépenses** : l'allocation de siège est le plafond par défaut. Pour permettre aux membres de continuer au-delà, activez les [crédits d'utilisation](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) et définissez les limites de dépenses au niveau de l'organisation, du groupe ou du membre individuel.
* **Extraire les chiffres par utilisateur** : sur le plan Enterprise, l'[API d'analyse Enterprise](https://platform.claude.com/docs/en/api/admin/analytics) retourne les rapports d'utilisation et de coûts par utilisateur sur toutes les surfaces Claude, y compris Claude Code. Un propriétaire principal crée une clé avec la portée `read:analytics` à [claude.ai/analytics/api-keys](https://claude.ai/analytics/api-keys). Sur le plan Teams, exportez le [CSV du rapport de dépenses](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans), qui répertorie l'utilisation des tokens et les dépenses estimées par utilisateur et par modèle.

Le [guide de consommation Claude Enterprise](https://support.claude.com/en/articles/14782391-claude-enterprise-consumption-guide) est la référence de planification pour les administrateurs. Il explique comment la consommation diffère entre Claude chat, Claude Code et Cowork, et donne des points de départ en dollars par utilisateur pour la budgétisation. Budgétisez davantage pour un siège de codage que pour un siège de chat : chaque tour de Claude Code contient le contenu des fichiers, les appels d'outils et le raisonnement multi-étapes, donc une session de débogage peut consommer plus qu'une journée de chat.

<h3 id="claude-console">
  Claude Console
</h3>

Les organisations API gèrent les dépenses de Claude Code via les [espaces de travail](https://platform.claude.com/docs/en/build-with-claude/workspaces). Vous pouvez [définir les limites de dépenses de l'espace de travail](https://platform.claude.com/docs/en/build-with-claude/workspaces#workspace-limits) sur la dépense totale de Claude Code et [afficher les rapports de coûts et d'utilisation](https://platform.claude.com/docs/en/build-with-claude/workspaces#usage-and-cost-tracking) dans la Console.

<Note>
  Lorsque vous authentifiez pour la première fois Claude Code avec votre compte Claude Console, un espace de travail appelé « Claude Code » est automatiquement créé pour vous. Cet espace de travail fournit un suivi et une gestion centralisés des coûts pour toute l'utilisation de Claude Code dans votre organisation. Vous ne pouvez pas créer de clés API pour cet espace de travail ; il est exclusivement destiné à l'authentification et à l'utilisation de Claude Code.

  Pour les organisations avec des limites de débit personnalisées, le trafic Claude Code dans cet espace de travail compte vers les limites de débit API globales de votre organisation. Vous pouvez définir une [limite de débit d'espace de travail](https://platform.claude.com/docs/fr/api/rate-limits#setting-lower-limits-for-workspaces) sur la page Limites de cet espace de travail dans la Console Claude pour limiter la part de Claude Code et protéger les autres charges de travail de production.
</Note>

Pour les rapports par utilisateur, le [tableau de bord de la Console](https://platform.claude.com/claude-code) affiche les dépenses et les lignes acceptées par membre, et l'[API d'analyse Claude Code](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) retourne les mêmes métriques quotidiennes par utilisateur par programmation avec une [clé API Admin](https://platform.claude.com/settings/admin-keys). Voir [analyse pour les clients API](/docs/fr/analytics#access-analytics-for-api-customers).

<h4 id="rate-limit-recommendations">
  Recommandations de limite de débit
</h4>

Lors de la configuration de Claude Code pour les équipes, tenez compte de ces recommandations de Token Par Minute (TPM) et Requête Par Minute (RPM) par utilisateur en fonction de la taille de votre organisation :

| Taille de l'équipe   | TPM par utilisateur | RPM par utilisateur |
| -------------------- | ------------------- | ------------------- |
| 1-5 utilisateurs     | 200 000-300 000     | 5-7                 |
| 5-20 utilisateurs    | 100 000-150 000     | 2,5-3,5             |
| 20-50 utilisateurs   | 50 000-75 000       | 1,25-1,75           |
| 50-100 utilisateurs  | 25 000-35 000       | 0,62-0,87           |
| 100-500 utilisateurs | 15 000-20 000       | 0,37-0,47           |
| 500+ utilisateurs    | 10 000-15 000       | 0,25-0,35           |

Par exemple, si vous avez 200 utilisateurs, vous pourriez demander 20 000 TPM pour chaque utilisateur, soit 4 millions de TPM au total (200 × 20 000 = 4 millions).

Le TPM par utilisateur diminue à mesure que la taille de l'équipe augmente, car moins d'utilisateurs ont tendance à utiliser Claude Code simultanément dans les grandes organisations. Ces limites de débit s'appliquent au niveau de l'organisation, et non par utilisateur individuel, ce qui signifie que les utilisateurs individuels peuvent temporairement consommer plus que leur part calculée lorsque d'autres n'utilisent pas activement le service.

<Note>
  Si vous anticipez des scénarios avec une utilisation concurrente inhabituellement élevée (comme des sessions de formation en direct avec de grands groupes), vous pourriez avoir besoin d'allocations TPM plus élevées par utilisateur.
</Note>

<h3 id="cloud-providers">
  Fournisseurs cloud
</h3>

Sur Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry, Claude Code est facturé par token à votre compte cloud, et les contrôles de dépenses se trouvent dans la console de facturation de votre fournisseur cloud. Claude Code n'envoie pas de métriques de votre cloud vers Anthropic, donc les [tableaux de bord d'analyse](/docs/fr/analytics) et l'API d'analyse Claude Code ne couvrent pas cette utilisation.

Pour l'attribution des coûts par utilisateur, vous avez trois options :

* **OpenTelemetry** : [exporter les métriques](/docs/fr/monitoring-usage) de la machine de chaque développeur vers votre propre pile d'observabilité. Cela vous donne les décomptes de tokens par utilisateur, les coûts et l'activité des outils quel que soit le fournisseur.
* **Une passerelle d'applications Claude** : une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) auto-hébergée fournit l'attribution d'utilisation par utilisateur, les métriques OTLP avec les décomptes de tokens, et les [limites de dépenses par utilisateur](/docs/fr/claude-apps-gateway-spend-limits) sur ces fournisseurs.
* **Une passerelle LLM** : acheminez tout le trafic Claude Code via un proxy qui suit les dépenses par clé. Plusieurs grandes entreprises ont signalé l'utilisation de [LiteLLM](/docs/fr/llm-gateway), un outil open-source qui [suit les dépenses par clé](https://docs.litellm.ai/docs/proxy/virtual_keys#tracking-spend). Ce projet n'est pas affilié à Anthropic et n'a pas été audité pour la sécurité.

<h3 id="when-a-developer-asks-about-a-limit">
  Quand un développeur pose des questions sur une limite
</h3>

Les développeurs apportent généralement les questions de limite à leur administrateur, il est donc utile de savoir quel plafond ils ont atteint. Les trois situations signifient des choses différentes :

* **« Vous avez atteint votre limite de session » ou « Vous avez atteint votre limite hebdomadaire »** : une fenêtre d'utilisation basée sur le siège sur un plan d'abonnement. Ces fenêtres sont partagées sur tous les modèles, donc changer de modèle avec `/model` ne restaure pas l'accès, bien que cela permette au développeur de continuer après le message « Vous avez atteint votre limite Opus ». Le message affiche quand la fenêtre se réinitialise, et le développeur peut exécuter `/usage-credits` pour demander une utilisation au-delà de l'allocation si vous avez activé les [crédits d'utilisation](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans). Voir [erreurs de limite d'utilisation](/docs/fr/errors#youve-hit-your-session-limit).
* **Un avertissement de contexte ou d'auto-compactage** : pas une limite d'utilisation. La conversation s'est rapprochée de la taille d'entrée maximale du modèle, et Claude Code résume l'historique plus ancien pour libérer de l'espace. Pointez le développeur vers [réduire l'utilisation des tokens](#reduce-token-usage).
* **Des dépenses inhabituellement élevées sur un plan API ou fournisseur cloud** : généralement tracées jusqu'à des sessions longues qui n'ont jamais été effacées ou à Opus laissé comme modèle par défaut. Les habitudes à impact le plus élevé à partager sont l'effacement entre les tâches non liées et l'adaptation du modèle au travail, tous deux couverts dans [réduire l'utilisation des tokens](#reduce-token-usage).

<h3 id="agent-team-token-costs">
  Coûts en tokens des équipes d'agents
</h3>

Les [équipes d'agents](/docs/fr/agent-teams) lancent plusieurs instances de Claude Code, chacune avec sa propre fenêtre de contexte. L'utilisation des tokens augmente avec le nombre de coéquipiers actifs et la durée d'exécution de chacun.

Pour maintenir les coûts des équipes d'agents gérables :

* Utilisez Sonnet pour les coéquipiers. Il équilibre la capacité et le coût pour les tâches de coordination.
* Gardez les équipes petites. Chaque coéquipier exécute sa propre fenêtre de contexte, donc l'utilisation des tokens est à peu près proportionnelle à la taille de l'équipe.
* Gardez les invites de génération concentrées. Les coéquipiers chargent CLAUDE.md, les serveurs MCP et les skills automatiquement, mais tout ce qui se trouve dans l'invite de génération s'ajoute à leur contexte dès le départ.
* Arrêtez les coéquipiers lorsque leur travail est terminé. Chaque coéquipier actif continue à consommer des tokens jusqu'à ce qu'il se termine ou que la session se termine.
* Les équipes d'agents sont désactivées par défaut. Définissez `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` dans votre [settings.json](/docs/fr/settings) ou dans l'environnement pour les activer. Voir [activer les équipes d'agents](/docs/fr/agent-teams#enable-agent-teams).

<h2 id="reduce-token-usage">
  Réduire l'utilisation des tokens
</h2>

Les coûts des tokens augmentent avec la taille du contexte : plus Claude traite de contexte, plus vous utilisez de tokens. Claude Code optimise automatiquement les coûts grâce à la [mise en cache des invites](/docs/fr/prompt-caching), qui réduit les coûts pour le contenu répété comme les invites système, et à la compaction automatique, qui résume l'historique des conversations en approchant les limites du contexte.

Les stratégies suivantes vous aident à maintenir le contexte petit et à réduire les coûts par message.

<h3 id="manage-context-proactively">
  Gérer le contexte de manière proactive
</h3>

Utilisez `/usage` pour vérifier votre utilisation actuelle des tokens, ou [configurez votre ligne d'état](/docs/fr/statusline#context-window-usage) pour l'afficher en continu.

* **Effacer entre les tâches** : Utilisez `/clear` pour recommencer à zéro lorsque vous passez à un travail non lié. Le contexte obsolète gaspille des tokens à chaque message suivant. Utilisez `/rename` avant d'effacer pour pouvoir facilement retrouver la session plus tard, puis `/resume` pour y revenir.
* **Ajouter des instructions de compaction personnalisées** : `/compact Focus on code samples and API usage` indique à Claude ce qu'il faut préserver lors de la résumé.

Vous pouvez également personnaliser le comportement de compaction dans votre fichier CLAUDE.md à la racine de votre projet :

```markdown theme={null}
# Compact instructions

When you are using compact, please focus on test output and code changes
```

<h3 id="choose-the-right-model">
  Choisir le bon modèle
</h3>

Sonnet gère bien la plupart des tâches de codage et coûte moins cher qu'Opus. Réservez Opus pour les décisions architecturales complexes ou le raisonnement multi-étapes. Utilisez `/model` pour changer de modèle en cours de session, ou définissez une valeur par défaut dans `/config`. Pour les tâches simples de subagent, spécifiez `model: haiku` dans votre [configuration de subagent](/docs/fr/sub-agents#choose-a-model).

<h3 id="reduce-mcp-server-overhead">
  Réduire la surcharge des serveurs MCP
</h3>

Les définitions d'outils MCP sont [reportées par défaut](/docs/fr/mcp#scale-with-mcp-tool-search), donc seuls les noms d'outils entrent en contexte jusqu'à ce que Claude utilise un outil spécifique. Exécutez `/context` pour voir ce qui consomme de l'espace.

* **Préférez les outils CLI lorsqu'ils sont disponibles** : Les outils comme `gh`, `aws`, `gcloud` et `sentry-cli` sont plus efficaces en contexte que les serveurs MCP car ils n'ajoutent pas de liste d'outils par outil. Claude peut exécuter les commandes CLI directement.
* **Désactiver les serveurs inutilisés** : Exécutez `/mcp` pour voir les serveurs configurés et désactiver ceux que vous n'utilisez pas activement.

<h3 id="install-code-intelligence-plugins-for-typed-languages">
  Installer des plugins d'intelligence de code pour les langages typés
</h3>

Les [plugins d'intelligence de code](/docs/fr/discover-plugins#code-intelligence) donnent à Claude une navigation de symboles précise au lieu d'une recherche basée sur le texte, réduisant les lectures de fichiers inutiles lors de l'exploration de code inconnu. Un seul appel « aller à la définition » remplace ce qui pourrait autrement être une recherche grep suivie de la lecture de plusieurs fichiers candidats. Les serveurs de langage installés signalent également automatiquement les erreurs de type après les modifications, donc Claude détecte les erreurs sans exécuter un compilateur.

<h3 id="offload-processing-to-hooks-and-skills">
  Déléguer le traitement aux hooks et aux skills
</h3>

Les [hooks](/docs/fr/hooks) personnalisés peuvent prétraiter les données avant que Claude ne les voie. Au lieu que Claude lise un fichier journal de 10 000 lignes pour trouver les erreurs, un hook peut rechercher `ERROR` et retourner uniquement les lignes correspondantes, réduisant le contexte de dizaines de milliers de tokens à des centaines.

Une [skill](/docs/fr/skills) peut donner à Claude des connaissances de domaine pour qu'il n'ait pas à explorer. Par exemple, une skill « codebase-overview » pourrait décrire l'architecture de votre projet, les répertoires clés et les conventions de nommage. Lorsque Claude invoque la skill, il obtient ce contexte immédiatement au lieu de dépenser des tokens pour lire plusieurs fichiers pour comprendre la structure.

Par exemple, ce hook PreToolUse filtre la sortie des tests pour afficher uniquement les échecs :

<Tabs>
  <Tab title="settings.json">
    Ajoutez ceci à votre [settings.json](/docs/fr/settings#settings-files) pour exécuter le hook avant chaque commande Bash :

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "~/.claude/hooks/filter-test-output.sh"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="filter-test-output.sh">
    Le hook appelle ce script. Créez le dossier avec `mkdir -p ~/.claude/hooks`, enregistrez le script ci-dessous sous `~/.claude/hooks/filter-test-output.sh` et rendez-le exécutable avec `chmod +x ~/.claude/hooks/filter-test-output.sh`. Il vérifie si la commande est un exécuteur de test et la modifie pour afficher uniquement les échecs :

    ```bash theme={null}
    #!/bin/bash
    input=$(cat)
    cmd=$(echo "$input" | jq -r '.tool_input.command')

    # If running tests, filter to show only failures
    if [[ "$cmd" =~ ^(npm test|pytest|go test) ]]; then
      filtered_cmd="$cmd 2>&1 | grep -A 5 -E '(FAIL|ERROR|error:)' | head -100"
      echo "{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"permissionDecision\":\"allow\",\"updatedInput\":{\"command\":\"$filtered_cmd\"}}}"
    else
      echo "{}"
    fi
    ```
  </Tab>
</Tabs>

<h3 id="move-instructions-from-claude-md-to-skills">
  Déplacer les instructions de CLAUDE.md vers les skills
</h3>

Votre fichier [CLAUDE.md](/docs/fr/memory) est chargé en contexte au démarrage de la session. S'il contient des instructions détaillées pour des flux de travail spécifiques (comme les révisions de PR ou les migrations de base de données), ces tokens sont présents même lorsque vous faites un travail non lié. Les [skills](/docs/fr/skills) se chargent à la demande uniquement lorsqu'elles sont invoquées, donc déplacer les instructions spécialisées dans les skills maintient votre contexte de base plus petit. Visez à garder CLAUDE.md en dessous de 200 lignes en incluant uniquement les éléments essentiels.

<h3 id="adjust-extended-thinking">
  Ajuster la réflexion étendue
</h3>

La réflexion étendue est activée par défaut car elle améliore considérablement les performances sur les tâches complexes de planification et de raisonnement. Les tokens de réflexion sont facturés comme des tokens de sortie, et le budget par défaut peut être des dizaines de milliers de tokens par requête selon le modèle. Pour les tâches plus simples où un raisonnement approfondi n'est pas nécessaire, vous pouvez réduire les coûts en abaissant le [niveau d'effort](/docs/fr/model-config#adjust-effort-level) avec `/effort` ou dans `/model`, en désactivant la réflexion dans `/config`, ou, sur les modèles avec un [budget de réflexion fixe](/docs/fr/model-config#adaptive-reasoning-and-fixed-thinking-budgets), en abaissant le budget en définissant la [variable d'environnement](/docs/fr/env-vars) `MAX_THINKING_TOKENS`, par exemple `MAX_THINKING_TOKENS=8000`. Les modèles de raisonnement adaptatif ignorent les budgets non nuls, donc utilisez plutôt les niveaux d'effort. La désactivation de la réflexion n'est pas disponible sur Fable 5, qui utilise toujours la réflexion étendue.

<h3 id="delegate-verbose-operations-to-subagents">
  Déléguer les opérations détaillées aux subagents
</h3>

L'exécution de tests, la récupération de documentation ou le traitement de fichiers journaux peuvent consommer un contexte important. Déléguez-les aux [subagents](/docs/fr/sub-agents#isolate-high-volume-operations) pour que la sortie détaillée reste dans le contexte du subagent tandis que seul un résumé revient à votre conversation principale.

<h3 id="manage-agent-team-costs">
  Gérer les coûts des équipes d'agents
</h3>

Les équipes d'agents utilisent environ 7 fois plus de tokens que les sessions standard lorsque les coéquipiers s'exécutent en mode plan, car chaque coéquipier maintient sa propre fenêtre de contexte et s'exécute en tant qu'instance Claude distincte. Gardez les tâches d'équipe petites et autonomes pour limiter l'utilisation des tokens par coéquipier. Voir [équipes d'agents](/docs/fr/agent-teams) pour plus de détails.

<h3 id="write-specific-prompts">
  Écrire des invites spécifiques
</h3>

Les demandes vagues comme « améliorer cette base de code » déclenchent une analyse large. Les demandes spécifiques comme « ajouter la validation des entrées à la fonction de connexion dans auth.ts » permettent à Claude de travailler efficacement avec des lectures de fichiers minimales.

<h3 id="work-efficiently-on-complex-tasks">
  Travailler efficacement sur des tâches complexes
</h3>

Pour un travail plus long ou plus complexe, ces habitudes aident à éviter les tokens gaspillés en prenant la mauvaise direction :

* **Utilisez le mode plan pour les tâches complexes** : Appuyez sur Maj+Tab pour entrer en [mode plan](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode) avant l'implémentation. Claude explore la base de code et propose une approche pour votre approbation, évitant les retouches coûteuses lorsque la direction initiale est mauvaise.
* **Corriger la trajectoire tôt** : Si Claude commence à aller dans la mauvaise direction, appuyez sur Échap pour arrêter immédiatement. Utilisez `/rewind` ou appuyez deux fois sur Échap pour restaurer la conversation et le code à un point de contrôle précédent.
* **Donner des cibles de vérification** : Incluez des cas de test, collez des captures d'écran ou définissez la sortie attendue dans votre invite. Lorsque Claude peut vérifier son propre travail, il détecte les problèmes avant que vous ayez besoin de demander des corrections.
* **Tester de manière progressive** : Écrivez un fichier, testez-le, puis continuez. Cela détecte les problèmes tôt lorsqu'ils sont bon marché à corriger.

<h2 id="background-token-usage">
  Utilisation des tokens en arrière-plan
</h2>

Claude Code utilise des tokens pour certaines fonctionnalités en arrière-plan même lorsqu'il est inactif :

* **Résumé des conversations** : Les tâches en arrière-plan qui résument les conversations précédentes pour la fonctionnalité `claude --resume`
* **Traitement des commandes** : Certaines commandes comme `/usage` peuvent générer des requêtes pour vérifier l'état

Ces processus en arrière-plan consomment une petite quantité de tokens (généralement moins de 0,04 \$ par session) même sans interaction active.

<h2 id="understanding-changes-in-claude-code-behavior">
  Comprendre les changements dans le comportement de Claude Code
</h2>

Claude Code reçoit régulièrement des mises à jour qui peuvent modifier le fonctionnement des fonctionnalités, y compris la génération de rapports de coûts. Exécutez `claude --version` pour vérifier votre version actuelle. Pour des questions de facturation spécifiques, contactez le support Anthropic via votre [compte Console](https://platform.claude.com/login).
