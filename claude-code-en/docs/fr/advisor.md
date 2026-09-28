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

Chacune de ces options active l'advisor pour les sessions dont le modèle principal [le supporte](#choose-an-advisor-model). Après le démarrage de la session, Claude Code affiche une notification `Advisor Tool (experimental) is on and may use more tokens · /advisor`. Pour arrêter d'utiliser l'advisor, consultez [Désactiver l'advisor](#turn-the-advisor-off).

Sur certains plans, Fable en tant qu'advisor nécessite également votre [consentement unique pour facturer l'utilisation de Fable aux crédits d'utilisation](/docs/fr/model-config#fable-and-usage-credits). Pour savoir ce qui se passe avant que vous ayez donné ce consentement, consultez [Advisor Fable et crédits d'utilisation](#fable-advisor-and-usage-credits).

<h3 id="use-the-/advisor-command">
  Utiliser la commande `/advisor`
</h3>

Exécutez `/advisor` sans arguments pour ouvrir un sélecteur listant les modèles advisor disponibles, ou passez le modèle directement :

```
/advisor opus
```

La commande confirme avec `Advisor set to` suivi du nom du modèle advisor. Votre sélection est enregistrée dans `advisorModel` dans vos paramètres utilisateur et persiste entre les sessions, sauf dans les cas que l'[entrée `advisorModel`](/docs/fr/settings-reference#advisormodel) liste comme s'appliquant à la session actuelle uniquement.

La commande fonctionne également là où il n'y a pas de sélecteur de terminal : en [mode non interactif](/docs/fr/headless) avec `-p`, dans le SDK Agent, dans l'application de bureau, et sur [Remote Control](/docs/fr/remote-control). Cela nécessite Claude Code v2.1.260 ou version ultérieure. Sur ces surfaces :

* Exécutez `/advisor` sans argument pour afficher le modèle advisor actuel et les alias qu'il accepte.
* Exécutez `/advisor` avec un modèle, tel que `/advisor opus`, pour le définir.
* Exécutez `/advisor off` pour le désactiver.

Claude Code n'invoque pas un advisor enregistré que la liste d'autorisation [`availableModels`](/docs/fr/model-config#restrict-model-selection) de votre organisation exclut. Pour utiliser l'advisor, choisissez un modèle autorisé avec `/advisor`.

Claude Code enregistre toujours un advisor que votre modèle principal actuel ne supporte pas. Cet advisor s'active après que vous basculiez vers un [modèle principal compatible](#choose-an-advisor-model) avec [`/model`](/docs/fr/model-config#setting-your-model). Si l'API a déjà refusé l'advisor enregistré dans la conversation actuelle, il reste désactivé jusqu'à `/clear` ou `/compact`, même après que vous ayez changé de modèles.

Sur certains plans, Fable en tant qu'advisor nécessite également votre [consentement unique pour facturer l'utilisation de Fable aux crédits d'utilisation](/docs/fr/model-config#fable-and-usage-credits). Pour savoir ce que `/advisor fable` fait avant que vous ayez donné ce consentement, consultez [Advisor Fable et crédits d'utilisation](#fable-advisor-and-usage-credits).

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

Claude Code utilise le drapeau à la place du paramètre `advisorModel` pour cette session. Il ne liste pas `--advisor` dans `claude --help`. Claude Code se termine avec une erreur au lancement si :

* Le modèle principal de la session ne supporte pas l'advisor
* Le modèle demandé, tel que Haiku, ne peut pas agir comme advisor
* La liste d'autorisation [`availableModels`](/docs/fr/model-config#restrict-model-selection) de votre organisation exclut le modèle demandé
* Vous avez demandé Fable et votre compte nécessite toujours le [consentement des crédits d'utilisation](#fable-advisor-and-usage-credits)

Si vous démarrez une [session en arrière-plan](/docs/fr/agent-view) avec `--advisor` et que l'une de ces conditions s'applique, Claude Code démarre la session sans l'advisor au lieu de se terminer.

<h2 id="choose-an-advisor-model">
  Choisir un modèle advisor
</h2>

L'advisor doit être au moins aussi capable que le modèle principal. Les advisors acceptés pour chaque modèle principal sont :

| Modèle principal     | Advisors acceptés                      | Notes                                                                                     |
| -------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------- |
| Haiku 4.5            | Fable, Opus, Sonnet                    | Haiku peut appeler l'advisor mais ne peut pas en être un                                  |
| Sonnet 4.6           | Fable, Opus, Sonnet                    |                                                                                           |
| Sonnet 5             | Fable, Opus 4.7 ou ultérieur, Sonnet 5 | Un advisor Sonnet 4.6 est rejeté, et l'API refuse un advisor Opus 4.6                     |
| Opus 4.6             | Fable, Opus, Sonnet 5                  | Un advisor Sonnet 4.6 est rejeté                                                          |
| Opus 4.7 ou Opus 4.8 | Fable, et Opus 4.7 ou ultérieur        | Un advisor Opus 4.6 ou Sonnet est rejeté                                                  |
| Opus 5.5 ou Opus 5   | Fable, et Opus 5 ou ultérieur          | Un advisor Opus 4.6 ou Sonnet est rejeté, et l'API refuse un advisor Opus 4.7 ou Opus 4.8 |
| Fable 5              | Fable 5.1 ou Fable 5                   | Un advisor Opus ou Sonnet est rejeté                                                      |
| Fable 5.1            | Fable 5.1                              | Un advisor Opus ou Sonnet est rejeté, et l'API refuse un advisor Fable 5                  |

Fable 5.1 nécessite Claude Code v2.1.257 ou ultérieur. Les deux modèles Fable nécessitent [l'accès à Fable](/docs/fr/model-config#work-with-fable).

Définissez l'advisor comme `fable`, `opus`, ou `sonnet`. Ces alias se résolvent à la version par défaut intégrée de Claude Code pour chaque famille de modèles, qui avance avec les nouvelles versions de Claude Code. Vous pouvez également passer un ID de modèle complet tel que `claude-opus-5-5`.

Les sous-agents héritent de l'advisor configuré et appliquent la même vérification d'appairage par rapport à leur propre modèle.

Claude Code valide l'appairage avant d'envoyer une requête, et l'API le valide à nouveau :

* Pour un advisor que le tableau liste comme rejeté, Claude Code ne l'attache pas aux requêtes du modèle principal. La sortie de la commande `/advisor` et une notification le montrent. Les sous-agents dont le modèle propre satisfait l'appairage peuvent toujours utiliser l'advisor.
* Pour un advisor que le tableau liste comme refusé par l'API, Claude Code l'attache et l'API le refuse. Claude Code renvoie ensuite cette requête sans l'advisor, et le reste de la conversation s'exécute sans un, donc vous ne voyez aucune erreur et n'obtenez aucun appel advisor. Choisissez un advisor accepté avec `/advisor` ; le changement prend effet après `/clear` ou `/compact` et dans les nouvelles sessions.
* Si le modèle principal ou l'advisor est un modèle que Claude Code ne reconnaît pas, l'advisor n'est pas attaché.

<h3 id="fable-advisor-and-usage-credits">
  Advisor Fable et crédits d'utilisation
</h3>

Sur certains plans, l'utilisation de Fable est facturée aux crédits d'utilisation, et Fable en tant qu'advisor est facturé de la même manière. Si votre compte nécessite le [consentement unique pour facturer l'utilisation de Fable aux crédits d'utilisation](/docs/fr/model-config#fable-and-usage-credits), Claude Code le demande lorsque vous sélectionnez un modèle Fable avec `/model` et n'applique pas Fable en tant qu'advisor jusqu'à ce que vous ayez accepté ce consentement.

Avant d'avoir accepté, Claude Code ne sauvegarde pas Fable en tant qu'advisor lorsque vous tapez `/advisor fable` ou choisissez Fable dans le sélecteur `/advisor`. Il vous pointe vers `/model fable` à la place. Avec `claude --advisor fable`, Claude Code quitte au lancement avec un message qui pointe vers `/model fable`. Dans une [session en arrière-plan](#use-the-advisor-flag), il démarre la session sans l'advisor au lieu de quitter. Avec Fable déjà sauvegardé en tant que votre `advisorModel`, Claude Code envoie les requêtes sans l'advisor. Dans une session interactive dont le modèle principal supporte l'advisor, il affiche également une notification qui pointe vers `/model fable`.

Pour accepter le consentement, exécutez `/model fable` et choisissez de continuer sur Fable. Claude Code enregistre le consentement et [sauvegarde Fable en tant que votre modèle sélectionné](/docs/fr/model-config#default-model-setting). Ensuite, sélectionnez Fable en tant qu'advisor.

<h3 id="common-model-pairings">
  Appairages de modèles courants
</h3>

Tout appairage accepté fonctionne. Ces combinaisons équilibrent le coût par rapport à la capacité de différentes façons :

| Appairage                         | Quand l'utiliser                                                                                                                                                                               |
| --------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Sonnet principal + advisor Opus   | Sonnet gère le travail routinier et escalade la planification, les défaillances ambiguës et les vérifications d'achèvement à Opus                                                              |
| Sonnet principal + advisor Fable  | Conseils Fable aux points de décision sans exécuter Fable partout. Nécessite l'accès à Fable                                                                                                   |
| Haiku principal + advisor Opus    | Modèle principal au coût le plus bas avec une planification puissante. Attendez-vous à un coût plus élevé que Haiku seul mais inférieur au basculement du modèle principal vers Sonnet ou Opus |
| Opus principal + advisor Opus     | Un deuxième Opus examine le premier. Utile pour les tâches à enjeux élevés où une vérification indépendante importe plus que le coût                                                           |
| Fable principal + advisor Fable   | Appairage de plus haute capacité lorsque Fable est disponible. Claude Code n'applique pas un advisor Opus ou Sonnet à un modèle principal Fable                                                |
| Sonnet principal + advisor Sonnet | Un deuxième avis à coût inférieur pour attraper les oublis routiniers                                                                                                                          |

<h2 id="when-claude-consults-the-advisor">
  Quand Claude consulte l'advisor
</h2>

Claude décide quand appeler l'advisor. Il tend à consulter avant de s'engager dans une approche, lorsqu'une erreur se reproduit constamment, et avant de déclarer une tâche terminée, mais le timing est piloté par le modèle plutôt que basé sur des règles.

Vous pouvez demander une consultation dans votre invite de la même façon que vous demanderiez n'importe quel outil, par exemple `consulter l'advisor avant de continuer`. Il n'y a pas de paramètre pour limiter ou forcer les appels advisor ; si vous voulez que Claude consulte plus ou moins souvent pendant une tâche, dites-le dans vos instructions.

<h2 id="what-you-see-during-a-session">
  Ce que vous voyez pendant une session
</h2>

Quand Claude appelle l'advisor, la transcription affiche une ligne `Advising` avec le nom du modèle advisor pendant que l'appel est en cours. Quand le résultat revient, la ligne indique si l'advisor a fourni des conseils :

* **Reviewed** : la ligne confirme que l'advisor a examiné la conversation. Quand l'advisor a fourni des conseils lisibles, appuyez sur `Ctrl+O` pour les lire.
* **Declined** : la ligne affiche `Advisor declined to advise on this request`. Si l'advisor a fourni une raison, appuyez sur `Ctrl+O` pour la lire.

Claude suit généralement les conseils de l'advisor, mais s'adapte lorsque ses propres preuves contredisent une affirmation spécifique : si une étape recommandée échoue lorsqu'elle est essayée, ou si le contenu du fichier contredit le conseil, Claude met en évidence le conflit plutôt que de suivre le conseil inconditionnellement.

L'advisor reçoit toujours la conversation complète, et Claude contrôle le timing. Pour plus de contrôle ou une configuration différente, consultez [comment l'advisor se compare avec les sous-agents et opusplan](#compare-with-related-features).

<h2 id="cost">
  Coût
</h2>

Quand Claude appelle l'advisor, le modèle advisor lit la conversation, donc chaque appel consomme des tokens aux tarifs du modèle advisor en plus de l'utilisation de votre modèle principal. La façon dont ces tokens advisor sont facturés dépend de votre mode de paiement :

* **Facturation API** : vous payez les tarifs d'entrée et de sortie du modèle advisor pour les tokens advisor
* **Plans d'abonnement** : l'utilisation de l'advisor compte vers les limites d'utilisation de votre plan, sauf qu'un advisor Fable est facturé aux [crédits d'utilisation](/docs/fr/model-config#fable-and-usage-credits) sur les plans où l'utilisation de Fable l'est

Si votre compte nécessite le consentement des crédits d'utilisation, un advisor Fable ne facture rien avant que vous le donniez, car Claude Code [n'applique pas la sélection](#fable-advisor-and-usage-credits) jusqu'à ce moment.

Claude appelle l'advisor aux points de décision plutôt que sur chaque tour, donc associer un modèle principal plus rapide avec un advisor plus puissant coûte généralement moins cher que d'exécuter le modèle plus puissant partout. L'utilisation de l'advisor compte vers les totaux de session affichés par [`/usage`](/docs/fr/costs#track-your-costs).

Pour savoir comment les tokens advisor sont signalés dans les réponses API, consultez [Utilisation et facturation](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool#usage-and-billing) dans la documentation de l'API Claude.

<h2 id="impact-on-prompt-caching">
  Impact sur la mise en cache des invites
</h2>

Activer ou désactiver l'advisor en milieu de session n'invalide pas le [cache d'invite](/docs/fr/prompt-caching) de votre modèle principal. Contrairement au [changement de modèle](/docs/fr/prompt-caching#switching-models), basculer `/advisor` garde le préfixe en cache intact, et les conseils retournés par l'advisor sont mis en cache dans la transcription sur les tours ultérieurs.

La propre lecture de la conversation par le modèle advisor n'est pas mise en cache. Chaque appel advisor traite la transcription complète à nouveau, sans réutilisation entre les appels.

<h2 id="requirements">
  Exigences
</h2>

L'outil advisor nécessite tous les éléments suivants :

* **API Anthropic uniquement** : l'advisor est un outil exécuté par le serveur. Il n'est pas disponible sur Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform, ou Microsoft Foundry. Via une [passerelle LLM](/docs/fr/llm-gateway) configurée avec `ANTHROPIC_BASE_URL`, la disponibilité dépend de si la passerelle transfère la requête intacte à l'API Anthropic. Si la passerelle ou son amont ne reconnaît pas l'outil advisor, consultez [Nouvelle tentative automatique et transfert d'erreur](/docs/fr/llm-gateway-protocol#automatic-retry-and-error-forwarding) pour voir comment Claude Code répond.
* **Modèle principal supporté** : Fable, Opus 4.6 ou ultérieur, Sonnet 4.6 ou ultérieur, ou Haiku 4.5. Voir [Choisir un modèle advisor](#choose-an-advisor-model) pour savoir quels advisors chacun accepte.
* **Récupération de feature flag** : Claude Code active l'advisor via un feature flag qu'il récupère auprès d'Anthropic. Dans une session où une variable qui désactive la récupération de flag est définie, comme `DISABLE_TELEMETRY`, l'advisor reste désactivé. Voir [Fonctionnalités nécessitant la récupération de feature flag](/docs/fr/env-vars#features-that-need-feature-flag-fetching).

<h2 id="turn-the-advisor-off">
  Désactiver l'advisor
</h2>

Pour arrêter d'utiliser l'advisor, exécutez `/advisor off` ou choisissez **No advisor** dans le sélecteur `/advisor` :

```
/advisor off
```

Pour désactiver l'outil advisor entièrement, définissez `CLAUDE_CODE_DISABLE_ADVISOR_TOOL=1`. La commande `/advisor` devient indisponible et tout `advisorModel` configuré est ignoré. Le drapeau `--advisor` est accepté mais n'a aucun effet. Consultez [Variables d'environnement](/docs/fr/env-vars).

<h2 id="compare-with-related-features">
  Comparer avec les fonctionnalités connexes
</h2>

L'advisor est l'une de plusieurs façons de combiner les forces des modèles. Choisissez en fonction de quand vous voulez qu'un deuxième modèle soit impliqué.

| Approche                                                         | Quand le modèle plus puissant s'exécute                                                                                                           | Comment il démarre                             |
| ---------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------- |
| Outil advisor                                                    | Aux points de décision en milieu de tâche                                                                                                         | Claude l'appelle quand il a besoin de conseils |
| [`opusplan`](/docs/fr/model-config#opusplan-model-setting)            | Pendant le mode plan quand [autorisé par `availableModels`](/docs/fr/model-config#restrict-model-selection), puis bascule vers Sonnet pour l'exécution | Vous entrez en mode plan                       |
| [Sous-agents](/docs/fr/sub-agents#choose-a-model) avec `model` défini | Pour l'ensemble de la sous-tâche déléguée                                                                                                         | Claude délègue, ou vous invoquez le sous-agent |
| [`/model`](/docs/fr/model-config#setting-your-model)                  | À partir de la prochaine requête                                                                                                                  | Vous changez de modèle                         |

<h2 id="see-also">
  Voir aussi
</h2>

* [Configuration du modèle](/docs/fr/model-config) : changer de modèles, définir les niveaux d'effort, et utiliser `opusplan`
* [Gérer les coûts efficacement](/docs/fr/costs) : suivre l'utilisation des tokens entre les modèles
* [Outil advisor dans l'API Claude](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool) : comprendre l'outil serveur sous-jacent, ou l'utiliser directement à partir de l'API Messages
* [La stratégie advisor](https://claude.com/blog/the-advisor-strategy) : pourquoi associer un modèle principal rapide avec un advisor plus puissant fonctionne
