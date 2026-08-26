> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Escalader les décisions difficiles avec l'outil advisor

> Associez votre modèle principal à un modèle advisor plus puissant que Claude consulte aux moments clés pendant une tâche.

<Note>
  L'outil advisor est expérimental et nécessite l'API Anthropic. Il n'est pas disponible sur Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform ou Microsoft Foundry. Le comportement, la tarification et la disponibilité peuvent changer.
</Note>

L'outil advisor permet à Claude de consulter un deuxième modèle, généralement plus puissant, aux moments clés pendant une tâche, par exemple avant de s'engager dans une approche, lorsqu'il est bloqué par une erreur récurrente, ou avant de déclarer une tâche terminée. L'advisor reçoit la conversation complète, y compris chaque appel d'outil et résultat, et retourne des conseils que Claude applique avant de continuer.

L'advisor s'exécute côté serveur sur l'infrastructure d'Anthropic en tant qu'[outil serveur](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool), disponible pour les comptes d'abonnement et facturés à l'API. Vous choisissez quel modèle agit comme advisor, et Claude décide quand l'appeler.

Cette page couvre comment activer l'advisor, quels appairages de modèles sont acceptés, ce que Claude affiche pendant une consultation, et comment l'utilisation de l'advisor est facturée.

<h2 id="when-to-use-the-advisor">
  Quand utiliser l'advisor
</h2>

L'advisor convient aux tâches longues et multi-étapes où la plupart des tours sont routiniers mais la qualité du plan détermine le résultat. Les exemples incluent les grandes refactorisations, les sessions de débogage où une erreur se reproduit constamment, et les tâches que vous voulez vérifier indépendamment avant que Claude les déclare terminées.

Il ajoute moins de valeur sur les tâches courtes où il y a peu à planifier, ou sur le travail où chaque tour a besoin du modèle le plus puissant. Pour ceux-ci, [changez le modèle principal](/docs/fr/model-config#setting-your-model) à la place, ou consultez [comment l'advisor se compare avec opusplan et les sous-agents](#compare-with-related-features) pour d'autres façons d'obtenir un deuxième avis.

<h2 id="enable-the-advisor">
  Activer l'advisor
</h2>

Vous pouvez définir le modèle advisor de trois façons :

* **Commande `/advisor`** : définir ou modifier l'advisor en milieu de session et l'enregistrer comme valeur par défaut
* **Paramètre `advisorModel`** : configurer une valeur par défaut persistante dans votre [fichier de paramètres](/docs/fr/settings)
* **Drapeau `--advisor`** : définir l'advisor pour une seule session au lancement

Si l'une de ces options définit un modèle advisor, l'advisor est activé pour les sessions dont le modèle principal [le supporte](#choose-an-advisor-model). Pour arrêter de l'utiliser, consultez [Désactiver l'advisor](#turn-the-advisor-off).

<Note>
  Pour utiliser Fable 5 comme advisor, vous avez besoin de Claude Code v2.1.170 ou ultérieur et de l'[accès à Fable 5](/docs/fr/model-config#work-with-fable-5) pour votre organisation.
</Note>

<h3 id="use-the-/advisor-command">
  Utiliser la commande `/advisor`
</h3>

Exécutez `/advisor` sans arguments pour ouvrir un sélecteur listant les modèles advisor disponibles, ou passez le modèle directement :

```
/advisor opus
```

Votre sélection est enregistrée dans `advisorModel` dans vos paramètres utilisateur et persiste entre les sessions. Si la liste d'autorisation [`availableModels`](/docs/fr/model-config#restrict-model-selection) de votre organisation exclut le modèle advisor enregistré, l'advisor n'est pas invoqué jusqu'à ce que vous choisissiez un modèle autorisé avec `/advisor`. Si votre modèle principal actuel ne supporte pas l'advisor, la sélection est toujours enregistrée et s'active lorsque vous basculez vers un [modèle principal compatible](#choose-an-advisor-model) avec [`/model`](/docs/fr/model-config#setting-your-model).

<h3 id="set-advisormodel-in-settings">
  Définir `advisorModel` dans les paramètres
</h3>

Pour configurer l'advisor comme valeur par défaut sans ouvrir une session, définissez-le dans votre fichier de paramètres :

```json theme={null}
{
  "advisorModel": "opus"
}
```

<h3 id="use-the-advisor-flag">
  Utiliser le drapeau `--advisor`
</h3>

Pour définir l'advisor pour une seule session sans modifier votre paramètre enregistré, lancez avec le drapeau :

```bash theme={null}
claude --advisor opus
```

Le drapeau a la priorité sur le paramètre `advisorModel` pour cette session. Il se termine avec une erreur si le modèle principal de la session ne supporte pas l'advisor, ou si le modèle advisor demandé est exclu par la liste d'autorisation [`availableModels`](/docs/fr/model-config#restrict-model-selection) de votre organisation.

<h2 id="choose-an-advisor-model">
  Choisir un modèle advisor
</h2>

L'advisor doit être au moins aussi capable que le modèle principal. Les advisors acceptés pour chaque modèle principal sont :

| Modèle principal      | Advisors acceptés         | Notes                                                                                                                                                                      |
| --------------------- | ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Haiku 4.5             | Fable, Opus, Sonnet       | Haiku peut appeler l'advisor mais ne peut pas en être un                                                                                                                   |
| Sonnet 4.6            | Fable, Opus, Sonnet       |                                                                                                                                                                            |
| Sonnet 5              | Fable, Opus, Sonnet 5     | Un advisor Sonnet 4.6 est rejeté                                                                                                                                           |
| Opus 4.6              | Fable, Opus, Sonnet 5     | Sonnet 5 et Opus 4.6 sont classés comme également capables, donc un Opus 4.6 principal accepte un advisor Sonnet 5                                                         |
| Opus 4.7 ou ultérieur | Fable, Opus 4.7, Opus 4.8 | Opus 4.7 et Opus 4.8 sont classés comme également capables, donc l'un accepte l'autre comme advisor. Un Opus 4.7 principal avec un advisor Opus 4.6 ou Sonnet 5 est rejeté |
| Fable 5 (v2.1.170+)   | Fable                     | Un advisor Opus ou Sonnet est rejeté                                                                                                                                       |

Fable 5 nécessite Claude Code v2.1.170 ou ultérieur et l'accès à Fable 5, qu'il agisse comme modèle principal ou advisor.

Définissez l'advisor comme `opus`, `sonnet`, ou `fable`. Ces alias se résolvent à la dernière version de chaque modèle. Vous pouvez également passer un ID de modèle complet tel que `claude-opus-4-8`.

Les sous-agents héritent de l'advisor configuré et appliquent la même vérification d'appairage par rapport à leur propre modèle.

Claude Code valide l'appairage avant d'envoyer une requête :

* Si l'advisor est moins capable que le modèle principal, l'advisor n'est pas attaché aux requêtes du modèle principal. La sortie de la commande `/advisor` et une notification le montrent. Les sous-agents dont le modèle propre satisfait l'appairage peuvent toujours utiliser l'advisor.
* Si le modèle principal ou l'advisor est un modèle que Claude Code ne reconnaît pas, l'advisor n'est pas attaché.

<h3 id="common-model-pairings">
  Appairages de modèles courants
</h3>

Tout appairage accepté fonctionne. Ces combinaisons équilibrent le coût par rapport à la capacité de différentes façons :

| Appairage                         | Quand l'utiliser                                                                                                                                                                               |
| --------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Sonnet principal + advisor Opus   | Sonnet gère le travail routinier et escalade la planification, les défaillances ambiguës et les vérifications d'achèvement à Opus                                                              |
| Sonnet principal + advisor Fable  | Conseils Fable 5 aux points de décision sans exécuter Fable 5 partout. Nécessite v2.1.170 ou ultérieur et l'accès à Fable 5                                                                    |
| Haiku principal + advisor Opus    | Modèle principal au coût le plus bas avec une planification puissante. Attendez-vous à un coût plus élevé que Haiku seul mais inférieur au basculement du modèle principal vers Sonnet ou Opus |
| Opus principal + advisor Opus     | Un deuxième Opus examine le premier. Utile pour les tâches à enjeux élevés où une vérification indépendante importe plus que le coût                                                           |
| Fable principal + advisor Fable   | Appairage de plus haute capacité lorsque Fable 5 est disponible (v2.1.170+). Fable est un niveau supérieur à Opus et Sonnet, donc c'est le seul advisor accepté pour un modèle principal Fable |
| Sonnet principal + advisor Sonnet | Un deuxième avis à coût inférieur pour attraper les oublis routiniers                                                                                                                          |

<h2 id="when-claude-consults-the-advisor">
  Quand Claude consulte l'advisor
</h2>

Claude décide quand appeler l'advisor. Il tend à consulter avant de s'engager dans une approche, lorsqu'une erreur se reproduit constamment, et avant de déclarer une tâche terminée, mais le timing est piloté par le modèle plutôt que basé sur des règles.

Vous pouvez demander une consultation dans votre invite de la même façon que vous demanderiez n'importe quel outil, par exemple `consulter l'advisor avant de continuer`. Il n'y a pas de paramètre pour limiter ou forcer les appels advisor ; si vous voulez que Claude consulte plus ou moins souvent pendant une tâche, dites-le dans vos instructions.

<h2 id="what-you-see-during-a-session">
  Ce que vous voyez pendant une session
</h2>

Quand Claude appelle l'advisor, la transcription affiche une ligne `Advising` avec le nom du modèle advisor pendant que l'appel est en cours. Quand le résultat revient, la ligne confirme que l'advisor a examiné la conversation. Appuyez sur `Ctrl+O` pour l'agrandir et lire les conseils complets de l'advisor.

Claude suit généralement les conseils de l'advisor, mais s'adapte lorsque ses propres preuves contredisent une affirmation spécifique : si une étape recommandée échoue lorsqu'elle est essayée, ou si le contenu du fichier contredit le conseil, Claude met en évidence le conflit plutôt que de suivre le conseil inconditionnellement.

L'advisor reçoit toujours la conversation complète, et Claude contrôle le timing. Pour plus de contrôle ou une configuration différente, consultez [comment l'advisor se compare avec les sous-agents et opusplan](#compare-with-related-features).

<h2 id="cost">
  Coût
</h2>

Chaque appel advisor envoie la conversation au modèle advisor, donc il consomme des tokens aux tarifs du modèle advisor en plus de l'utilisation de votre modèle principal. Avec la facturation API, les tokens advisor sont facturés aux tarifs d'entrée et de sortie du modèle advisor. Sur les plans d'abonnement, l'utilisation de l'advisor compte vers les limites d'utilisation de votre plan.

Claude appelle l'advisor aux points de décision plutôt que sur chaque tour, donc associer un modèle principal plus rapide avec un advisor plus puissant coûte généralement moins cher que d'exécuter le modèle plus puissant partout. L'utilisation de l'advisor compte vers les totaux de session affichés par [`/usage`](/docs/fr/costs#track-your-costs).

Pour savoir comment les tokens advisor sont signalés dans les réponses API, consultez [Utilisation et facturation](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool#usage-and-billing) dans la documentation de l'API Claude.

<h2 id="impact-on-prompt-caching">
  Impact sur la mise en cache des invites
</h2>

Activer ou désactiver l'advisor en milieu de session n'invalide pas le [cache d'invite](/docs/fr/prompt-caching) de votre modèle principal. Contrairement au [changement de modèle ou de niveau d'effort](/docs/fr/prompt-caching#actions-that-invalidate-the-cache), basculer `/advisor` garde le préfixe en cache intact, et les conseils retournés par l'advisor sont mis en cache dans la transcription sur les tours ultérieurs.

La propre lecture de la conversation par le modèle advisor n'est pas mise en cache. Chaque appel advisor traite la transcription complète à nouveau, sans réutilisation entre les appels.

<h2 id="requirements">
  Exigences
</h2>

L'outil advisor nécessite tous les éléments suivants :

* **API Anthropic uniquement** : l'advisor est un outil exécuté par le serveur. Il n'est pas disponible sur Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform, ou Microsoft Foundry. Via une [passerelle LLM](/docs/fr/llm-gateway) configurée avec `ANTHROPIC_BASE_URL`, la disponibilité dépend de si la passerelle transfère la requête intacte à l'API Anthropic.
* **Modèle principal supporté** : Opus 4.6 ou ultérieur, Sonnet 4.6 ou ultérieur, ou Haiku 4.5. Fable 5 se qualifie également sur Claude Code v2.1.170 ou ultérieur.

<h2 id="turn-the-advisor-off">
  Désactiver l'advisor
</h2>

Pour arrêter d'utiliser l'advisor et effacer votre `advisorModel` enregistré, exécutez `/advisor off` ou choisissez **No advisor** dans le sélecteur `/advisor` :

```
/advisor off
```

Pour désactiver l'outil advisor entièrement, définissez `CLAUDE_CODE_DISABLE_ADVISOR_TOOL=1`. La commande `/advisor` devient indisponible et tout `advisorModel` configuré est ignoré. Le drapeau `--advisor` est accepté mais n'a aucun effet ; les scripts existants qui le transmettent continuent de fonctionner sans erreurs. Consultez [Variables d'environnement](/docs/fr/env-vars).

<h2 id="compare-with-related-features">
  Comparer avec les fonctionnalités connexes
</h2>

L'advisor est l'une de plusieurs façons de combiner les forces des modèles. Choisissez en fonction de quand vous voulez qu'un deuxième modèle soit impliqué.

| Approche                                                         | Quand le modèle plus puissant s'exécute                                                                                                           | Comment il démarre                             |
| ---------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------- |
| Outil advisor                                                    | Aux points de décision en milieu de tâche                                                                                                         | Claude l'appelle quand il a besoin de conseils |
| [`opusplan`](/docs/fr/model-config#opusplan-model-setting)            | Pendant le mode plan quand [autorisé par `availableModels`](/docs/fr/model-config#restrict-model-selection), puis bascule vers Sonnet pour l'exécution | Vous entrez en mode plan                       |
| [Sous-agents](/docs/fr/sub-agents#choose-a-model) avec `model` défini | Pour l'ensemble de la sous-tâche déléguée                                                                                                         | Claude délègue, ou vous invoquez le sous-agent |
| [`/model`](/docs/fr/model-config#setting-your-model)                  | Pour tous les tours ultérieurs                                                                                                                    | Vous changez de modèle                         |

<h2 id="see-also">
  Voir aussi
</h2>

* [Configuration du modèle](/docs/fr/model-config) : changer de modèles, définir les niveaux d'effort, et utiliser `opusplan`
* [Gérer les coûts efficacement](/docs/fr/costs) : suivre l'utilisation des tokens entre les modèles
* [Outil advisor dans l'API Claude](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool) : comprendre l'outil serveur sous-jacent, ou l'utiliser directement à partir de l'API Messages
* [La stratégie advisor](https://claude.com/blog/the-advisor-strategy) : pourquoi associer un modèle principal rapide avec un advisor plus puissant fonctionne
