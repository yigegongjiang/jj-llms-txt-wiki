> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Accélérez les réponses avec le mode rapide

> Obtenez des réponses Opus plus rapides dans Claude Code en activant le mode rapide.

<Note>
  Le mode rapide est en [aperçu de recherche](#research-preview). La fonctionnalité, la tarification et la disponibilité peuvent changer en fonction des commentaires.
</Note>

Le mode rapide est une configuration haute vitesse pour Claude Opus, rendant le modèle jusqu'à 2,5 fois plus rapide à un coût par jeton plus élevé. Activez-le avec `/fast` quand vous avez besoin de vitesse pour un travail interactif comme l'itération rapide ou le débogage en direct, et désactivez-le quand le coût importe plus que la latence.

Le mode rapide n'est pas un modèle différent. Il utilise Claude Opus avec une configuration API différente qui priorise la vitesse plutôt que l'efficacité des coûts. Vous obtenez une qualité et des capacités identiques avec des réponses plus rapides. Le mode rapide est pris en charge sur Opus 5.5, Opus 5 et Opus 4.8. Il n'est pas disponible sur Sonnet, Haiku ou d'autres modèles.

Opus 4.7 ne supporte pas le mode rapide, donc le basculer désactive le mode rapide. Le mode rapide pour Opus 4.7 a été déprécié le 25 juin 2026 et supprimé le 24 juillet 2026.

Ce qu'il faut savoir :

* Utilisez `/fast` pour activer/désactiver le mode rapide dans Claude Code CLI. L'extension [VS Code](/docs/fr/vs-code) offre une commande **Toggle fast mode** quand le modèle sélectionné supporte le mode rapide. Claude Code enregistre ce paramètre dans votre paramètre [`fastMode`](#toggle-fast-mode).
* La tarification du mode rapide par MTok entrée/sortie est de 8 $/40 $ sur Opus 5.5 et de 10 $/50 $ sur Opus 5 et Opus 4.8.
* Disponible pour les utilisateurs de Claude Code sur les plans d'abonnement (Pro/Max/Team/Enterprise) et sur Claude Console. Les organisations Team et Enterprise ont besoin qu'un propriétaire l'active d'abord, et les organisations Console ont besoin d'un accès provisionné d'abord, tous deux décrits sous [Exigences](#requirements).
* Pour les utilisateurs de Claude Code sur les plans d'abonnement (Pro/Max/Team/Enterprise), le mode rapide est disponible via les crédits d'utilisation uniquement et n'est pas inclus dans les limites de taux d'abonnement.

<h2 id="toggle-fast-mode">
  Activer le mode rapide
</h2>

En CLI, activez le mode rapide de l'une de ces deux façons :

* Exécutez `/fast`, appuyez sur Espace pour activer ou désactiver, puis appuyez sur Entrée pour confirmer
* Définissez `"fastMode": true` dans votre [fichier de paramètres utilisateur](/docs/fr/settings)

Par défaut, le mode rapide que vous activez dans une session interactive persiste entre les sessions. Vous pouvez configurer le mode rapide pour qu'il se réinitialise à chaque session. Consultez [opt-in par session](#require-per-session-opt-in) pour plus de détails.

En dehors d'une [session cloud](#use-fast-mode-in-cloud-sessions), en [mode non-interactif](/docs/fr/headless) avec le drapeau `-p`, `/fast` fonctionne uniquement dans une session lancée avec le mode rapide dans sa valeur [`--settings`](/docs/fr/cli-reference#cli-flags), par exemple `claude -p --settings '{"fastMode": true}'` ; le basculement s'applique alors uniquement à cette session et n'est pas enregistré comme votre paramètre par défaut. Le formulaire `-p` nécessite Claude Code v2.1.205 ou ultérieur. Ailleurs en mode non-interactif, la commande signale que le mode rapide n'est pas disponible.

Vous pouvez exécuter `/fast` pendant que Claude travaille, et Claude Code bascule le mode rapide sans attendre la fin du tour. Claude Code termine le tour en cours à sa vitesse d'origine, donc le changement de vitesse prend effet à partir de votre prochain tour. Si votre modèle actuel ne supporte pas le mode rapide, l'activer bascule également votre modèle, et Claude Code utilise le nouveau modèle à partir de sa prochaine requête dans ce tour.

Pour la meilleure efficacité des coûts, activez le mode rapide au début d'une session plutôt que de basculer en milieu de conversation. Consultez [comprendre le compromis de coût](#understand-the-cost-tradeoff) pour plus de détails.

Quand vous activez le mode rapide :

* Si votre modèle actuel ne supporte pas le mode rapide, Claude Code bascule vers Opus
* Vous verrez un message de confirmation : « Mode rapide ACTIVÉ »
* Une petite icône `↯` apparaît à côté de l'invite pendant que le mode rapide est actif
* Exécutez `/fast` à nouveau à tout moment pour vérifier si le mode rapide est activé ou désactivé

Opus 5.5 est le mode rapide par défaut dans Claude Code v2.1.280 et ultérieur. Avant v2.1.280, le mode rapide utilisait par défaut Opus 5 à partir de v2.1.219, Opus 4.8 sur v2.1.154 à v2.1.218, et Opus 4.7 sur v2.1.142 à v2.1.153.

Quand vous désactivez le mode rapide avec `/fast` à nouveau, vous restez sur Opus. Pour basculer vers un modèle différent, utilisez `/model`.

<h3 id="switch-models-while-fast-mode-is-on">
  Basculer les modèles pendant que le mode rapide est activé
</h3>

Le mode rapide suit vos changements de modèle dans les deux directions :

* **Basculer vers un autre** : quand vous basculez vers un modèle qui ne supporte pas le mode rapide, Claude Code désactive le mode rapide. Cela inclut Opus 4.7 ; avant v2.1.221, le mode rapide restait activé après un basculement vers Opus 4.7 et l'API rejetait les requêtes.
* **Basculer vers un modèle supporté** : basculer vers un modèle Opus supporté réactive le mode rapide quand votre préférence de mode rapide enregistrée est activée, la même préférence qu'une nouvelle session démarre par défaut. Un changement de modèle ne réactive jamais le mode rapide pour une session dont la préférence enregistrée est désactivée, et avec [opt-in par session](#require-per-session-opt-in) configuré, basculer vers un modèle supporté ne le réactive pas non plus ; exécutez `/fast` pour le réactiver.

Chaque fois qu'un changement de modèle active ou désactive le mode rapide, Claude Code affiche une confirmation « Mode rapide ACTIVÉ » ou « Mode rapide DÉSACTIVÉ », et l'icône `↯` apparaît pendant que le mode rapide est activé. Cela s'applique que vous basculiez avec `/model`, avec [`/config model=<model>`](/docs/fr/settings), ou depuis un appareil connecté via [Contrôle à distance](/docs/fr/remote-control).

Claude Code renvoie l'état du mode rapide de la session aux appareils connectés via Contrôle à distance après un changement de modèle, une reconnexion, ou une [vérification de disponibilité](#use-fast-mode-behind-proxies-and-llm-gateways) échouée.

<h3 id="use-fast-mode-in-cloud-sessions">
  Utiliser le mode rapide dans les sessions cloud
</h3>

Le mode rapide fonctionne dans les [sessions cloud](/docs/fr/claude-code-on-the-web) quand il est disponible sur votre compte, que la session s'exécute sur l'infrastructure gérée par Anthropic ou un [exécuteur auto-hébergé](/docs/fr/self-hosted-environments). Nécessite Claude Code v2.1.271 ou ultérieur dans l'environnement de la session.

Tapez `/fast on` dans la session pour activer le mode rapide. Il reste activé pour cette session uniquement et n'est pas enregistré comme votre paramètre par défaut. Les [exigences](#requirements) s'appliquent également dans les sessions cloud.

<h2 id="understand-the-cost-tradeoff">
  Comprendre le compromis de coût
</h2>

Le mode rapide a une tarification par jeton plus élevée que l'Opus standard :

| Modèle   | Entrée (MTok) | Sortie (MTok) |
| -------- | ------------- | ------------- |
| Opus 5.5 | \$8           | \$40          |
| Opus 5   | \$10          | \$50          |
| Opus 4.8 | \$10          | \$50          |

La tarification du mode rapide est forfaitaire sur toute la fenêtre de contexte de 1 M de jetons. Pour le taux Opus standard à comparer, consultez la [référence de tarification Claude](https://platform.claude.com/docs/en/about-claude/pricing).

La première fois que vous activez le mode rapide dans une conversation, vous payez le prix complet du jeton d'entrée non mis en cache du mode rapide pour tout le contexte de la conversation. Plus vous êtes avancé dans une conversation, plus cela coûte cher, donc activer le mode rapide dès le départ est moins cher. Le coût s'applique une fois par conversation, donc désactiver le mode rapide et le réactiver plus tard ne le répète pas. Pour le mécanisme, consultez [comment le mode rapide interagit avec le cache de prompt](/docs/fr/prompt-caching#turning-on-fast-mode).

<h3 id="see-where-fast-mode-spend-appears">
  Voir où apparaît la dépense du mode rapide
</h3>

Vous voyez la dépense du mode rapide à un endroit différent selon la façon dont vous vous êtes connecté, alors exécutez d'abord [`/status`](/docs/fr/commands) pour vérifier. S'il affiche une ligne `Login method` telle que `Claude Max account`, vous vous êtes connecté avec un abonnement Claude. S'il affiche une ligne `API key` à la place, vos demandes sont facturées à une organisation Claude Console.

* **Pro et Max** : vous payez le mode rapide à partir de vos crédits d'utilisation. Allez à [**Paramètres > Utilisation**](https://claude.ai/settings/usage) sur claude.ai, où la section **Crédits d'utilisation** affiche combien vous avez dépensé en crédits d'utilisation ce mois-ci. Ce chiffre inclut le mode rapide mais ne le détaille pas séparément.
* **Team et Enterprise** : votre organisation paie votre utilisation du mode rapide à partir de ses crédits d'utilisation. Pour voir votre dépense en crédits d'utilisation, exécutez [`/usage`](/docs/fr/costs#check-your-usage-credits-spend). Pour voir où votre organisation voit cette dépense, consultez [Claude for Teams and Enterprise](/docs/fr/costs#claude-for-teams-and-enterprise).
* **Claude Console** : votre organisation paie le mode rapide avec le reste de son utilisation de l'API. Sur les pages Console [Usage](https://platform.claude.com/usage) et [Cost](https://platform.claude.com/cost), sélectionnez **Speed (Research Preview)** dans le menu **Group by** pour séparer le mode rapide de l'utilisation à vitesse standard. Vous ne voyez cette option que lorsque la plage de dates sélectionnée inclut l'utilisation du mode rapide.

<h2 id="decide-when-to-use-fast-mode">
  Décider quand utiliser le mode rapide
</h2>

Le mode rapide est idéal pour le travail interactif où la latence de réponse importe plus que le coût :

* Itération rapide sur les modifications de code
* Sessions de débogage en direct
* Travail sensible au temps avec des délais serrés

Le mode standard est meilleur pour :

* Les tâches autonomes longues où la vitesse importe moins
* Le traitement par lots ou les pipelines CI/CD
* Les charges de travail sensibles aux coûts

<h3 id="fast-mode-vs-effort-level">
  Mode rapide par rapport au niveau d'effort
</h3>

Le mode rapide et le niveau d'effort affectent tous deux la vitesse de réponse, mais différemment :

| Paramètre                     | Effet                                                                                                           |
| ----------------------------- | --------------------------------------------------------------------------------------------------------------- |
| **Mode rapide**               | Même qualité de modèle, latence inférieure, coût plus élevé                                                     |
| **Niveau d'effort inférieur** | Moins de temps de réflexion, réponses plus rapides, qualité potentiellement inférieure sur les tâches complexes |

Vous pouvez combiner les deux : utilisez le mode rapide avec un [niveau d'effort](/docs/fr/model-config#adjust-effort-level) inférieur pour une vitesse maximale sur les tâches simples.

<h2 id="requirements">
  Exigences
</h2>

Le mode rapide nécessite tous les éléments suivants :

* **API Anthropic ou abonnement uniquement** : le mode rapide est disponible via l'API Anthropic Console et pour les plans d'abonnement Claude utilisant les crédits d'utilisation. Il n'est pas disponible sur Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry ou Claude Platform sur AWS. Les organisations Console doivent également avoir [l'accès au mode rapide provisionné](#enable-fast-mode-for-your-organization).
* **Crédits d'utilisation activés pour les plans d'abonnement** : sur un plan Pro, Max, Team ou Enterprise, votre compte doit avoir les [crédits d'utilisation](/docs/fr/costs#add-usage-credits-to-your-subscription) activés, ce qui permet la facturation au-delà de l'utilisation incluse dans votre plan. Jusqu'à ce qu'ils soient activés, `/fast` affiche « Le mode rapide nécessite des crédits d'utilisation ». La façon dont vous les activez dépend de votre plan :
  * Sur Pro et Max, activez-les dans la section **Crédits d'utilisation** de [**Paramètres > Utilisation**](https://claude.ai/settings/usage) sur claude.ai, ou exécutez `/usage-credits` pour ouvrir cette page.
  * Sur Team et Enterprise, un membre ayant accès à la facturation les active pour l'organisation dans [**Paramètres d'administration > Utilisation**](https://claude.ai/admin-settings/usage), et un membre sans accès exécute `/usage-credits` pour envoyer une demande aux administrateurs de l'organisation.

<Note>
  L'utilisation du mode rapide est facturée directement à partir des crédits d'utilisation, même si vous avez une utilisation restante sur votre plan.
</Note>

* **Organisation Console payante** : les comptes Claude Console n'utilisent pas de crédits d'utilisation, et votre organisation paie le mode rapide par jeton avec le reste de son utilisation API. Sur le plan Evaluation gratuit de la Console, `/fast` affiche « Le mode rapide n'est pas disponible pendant l'évaluation. Veuillez acheter des crédits. » Pour l'effacer, achetez des crédits dans vos [paramètres de facturation Console](https://platform.claude.com/settings/billing).
* **Activation par le propriétaire pour Team et Enterprise** : le mode rapide est désactivé par défaut pour les organisations Team et Enterprise. Un propriétaire doit explicitement [activer le mode rapide](#enable-fast-mode-for-your-organization) avant que les utilisateurs puissent y accéder.

<Note>
  Quatre paramètres d'organisation peuvent empêcher l'activation du mode rapide avec `/fast` :

  * **Mode rapide non activé** : si le mode rapide n'a pas été activé pour votre organisation, l'activation du mode rapide avec `/fast` affiche « Le mode rapide a été désactivé par votre organisation. »
  * **Mode rapide désactivé par les paramètres gérés** : si votre organisation déploie les [paramètres gérés](/docs/fr/managed-settings) qui définissent [`fastMode: false`](/docs/fr/settings-reference#fastmode), l'activation du mode rapide avec `/fast` affiche le même message « Le mode rapide a été désactivé par votre organisation ».
  * **Opt-in par session requis** : les paramètres gérés qui définissent [`fastModePerSessionOptIn: true`](#require-per-session-opt-in) refusent `/fast on` avec le même message partout sauf dans une session de terminal interactive.
  * **Modèle de mode rapide non autorisé** : si la liste d'autorisation [`availableModels`](/docs/fr/model-config#restrict-model-selection) de votre organisation exclut le modèle Opus du mode rapide, l'activation est refusée avec « n'est pas dans les modèles autorisés de votre organisation ». Dans une session déjà en cours d'exécution sur un modèle Opus autorisé qui prend en charge le mode rapide, `/fast` active le mode rapide sur votre modèle actuel au lieu de changer de modèle.
</Note>

<h3 id="enable-fast-mode-for-your-organization">
  Activer le mode rapide pour votre organisation
</h3>

Où vous activez le mode rapide dépend du produit que votre organisation utilise :

* **Console** (clients API) : un administrateur l'active dans les [préférences Claude Code](https://platform.claude.com/claude-code/preferences). Le mode rapide est en [aperçu de recherche](#research-preview), donc votre organisation doit également avoir l'accès au mode rapide provisionné avant que les demandes de mode rapide réussissent. Pour obtenir l'accès, contactez votre gestionnaire de compte ou rejoignez la liste d'attente, comme décrit dans [le mode rapide sur l'API Claude](https://platform.claude.com/docs/en/build-with-claude/fast-mode).

  Sans accès provisionné, l'API rejette chaque demande de mode rapide avec un 429, et Claude Code traite chaque rejet comme une [limite de débit du mode rapide](#handle-rate-limits). Contrairement au délai d'attente d'une limite de débit, les rejets continuent jusqu'à ce que l'accès soit provisionné.
* **Claude AI** (Team et Enterprise) : un propriétaire l'active dans [Paramètres d'administration > Claude Code](https://claude.ai/admin-settings/claude-code)

Une autre option pour désactiver complètement le mode rapide est de définir `CLAUDE_CODE_DISABLE_FAST_MODE=1`. Consultez [Variables d'environnement](/docs/fr/env-vars).

<h3 id="use-fast-mode-behind-proxies-and-llm-gateways">
  Utiliser le mode rapide derrière les proxies et les passerelles LLM
</h3>

Avant d'offrir le mode rapide, Claude Code vérifie la disponibilité du mode rapide de votre organisation avec une demande directe à `api.anthropic.com`. La vérification ne suit pas [`ANTHROPIC_BASE_URL`](/docs/fr/llm-gateway-connect#set-the-base-url-and-credential), donc sur un réseau qui achemine le trafic Claude via une [passerelle LLM](/docs/fr/llm-gateway) et bloque la sortie directe vers `api.anthropic.com`, la vérification échoue même si les demandes d'inférence fonctionnent. La vérification utilise un [proxy HTTP](/docs/fr/network-config#proxy-configuration) configuré, donc un blocage réseau échoue la vérification uniquement où `api.anthropic.com` est inaccessible même via le proxy.

Lorsque la vérification échoue, `/fast` signale « Le mode rapide n'est pas disponible en raison de problèmes de connectivité réseau », et les demandes s'exécutent à vitesse standard, même si votre organisation a activé le mode rapide. Une vérification qui a réussi dans le passé continue de fonctionner à partir de son résultat en cache, donc une vérification bloquée affecte principalement les nouvelles installations.

Le même message de connectivité apparaît sur un réseau ouvert lorsque la vérification atteint `api.anthropic.com` mais présente une credential qu'Anthropic rejette. Une session dont la clé résolue est une credential émise par la passerelle, détenue dans [`ANTHROPIC_API_KEY`](/docs/fr/llm-gateway-connect#set-the-base-url-and-credential) ou produite par un [`apiKeyHelper`](/docs/fr/settings-reference#apikeyhelper), envoie la vérification avec cette clé, et la demande rejetée est signalée comme une défaillance de connectivité.

Pour restaurer le mode rapide, autorisez la sortie directe vers `api.anthropic.com` où un blocage réseau est la cause, ou définissez la variable qui correspond à la façon dont la vérification échoue :

* `CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS=1` traite une vérification échouée comme disponible et honore toujours une réponse « désactivé par votre organisation ». Utilisez-le lorsque votre réseau refuse la connexion, ou lorsqu'Anthropic rejette une credential de passerelle ; l'autorisation ne aide pas le cas de credential, puisque rien n'est bloqué.
* `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK=1` ignore complètement la vérification. Utilisez-le lorsque votre réseau intercepte la demande plutôt que de la refuser.

Deux configurations de passerelle signalent « Le mode rapide a été désactivé par votre organisation » plutôt que le message de connectivité, même si votre organisation a activé le mode rapide :

* Une session qui s'authentifie avec [`ANTHROPIC_AUTH_TOKEN`](/docs/fr/llm-gateway-connect#set-the-base-url-and-credential) seul ignore la vérification : sans connexion claude.ai ou clé API Anthropic, et sans vérification réussie en cache, Claude Code traite le mode rapide comme désactivé par votre organisation sans envoyer la demande.
* Un proxy qui intercepte la vérification et répond avec sa propre page, par exemple un proxy inspectant TLS renvoyant une page de blocage HTTP 200, est lu comme une réponse indiquant que votre organisation a désactivé le mode rapide.

Dans les deux cas, définissez `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK=1` pour restaurer le mode rapide. `CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS` ne s'applique à aucun des deux cas, puisqu'il contourne uniquement les vérifications échouées et les deux produisent une réponse désactivée à la place. L'autorisation de la sortie directe n'aide pas le cas du jeton porteur, qui n'envoie jamais la demande.

Les variables affectent uniquement la vérification côté client. Lorsque votre organisation a désactivé le mode rapide, l'API rejette les demandes de mode rapide qu'elles soient définies ou non. Un rejet de l'API persiste même avec une variable de saut définie. Claude Code réessaie la demande rejetée à vitesse standard, désactive le mode rapide, et `/fast` signale que votre organisation a désactivé le mode rapide.

La définition de `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` supprime également la vérification de disponibilité. Sans vérification réussie précédemment en cache, `/fast` signale « Le mode rapide n'est actuellement pas disponible » ; les deux variables de saut restaurent le mode rapide dans cette configuration également.

<h3 id="require-per-session-opt-in">
  Opt-in par session
</h3>

Par défaut, le mode rapide qu'un utilisateur active dans une session interactive persiste entre les sessions. Pour modifier ceci, définissez `fastModePerSessionOptIn` à `true` dans n'importe quel [fichier de paramètres](/docs/fr/settings#where-settings-live), ce qui fait que chaque session commence avec le mode rapide désactivé et oblige les utilisateurs à l'activer explicitement avec `/fast`. Les propriétaires sur les plans [Team](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_teams#team-&-enterprise) ou [Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_enterprise) peuvent le déployer à l'échelle de l'organisation via les [paramètres gérés par le serveur](/docs/fr/server-managed-settings).

```json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

Ceci est utile pour contrôler les coûts dans les organisations où les utilisateurs exécutent plusieurs sessions simultanées. La préférence du mode rapide de l'utilisateur est toujours enregistrée, donc supprimer ce paramètre restaure le comportement persistant par défaut.

Lorsque les paramètres gérés définissent la clé, `/fast on` fonctionne uniquement dans une session de terminal interactive. Partout ailleurs, y compris le [mode non-interactif](/docs/fr/headless), l'[extension VS Code](/docs/fr/vs-code) et les [sessions cloud](#use-fast-mode-in-cloud-sessions), il est refusé avec un message indiquant que votre organisation a désactivé le mode rapide.

<h2 id="handle-rate-limits">
  Gérer les limites de taux
</h2>

Le mode rapide a des limites de taux séparées de l'Opus standard. Tous les modèles Opus pris en charge partagent un pool de limites de taux en mode rapide : l'utilisation sur l'un d'entre eux puise dans les mêmes limites. Quand vous atteignez la limite de taux du mode rapide :

1. Le mode rapide bascule automatiquement vers la vitesse standard
2. L'icône `↯` devient grise pour indiquer le refroidissement
3. Vous continuez à travailler à la vitesse et à la tarification standard
4. Quand le refroidissement expire, le mode rapide se réactive automatiquement

Pour désactiver manuellement le mode rapide au lieu d'attendre le refroidissement, exécutez `/fast` à nouveau.

Si vous manquez de crédits d'utilisation en cours de session, Claude Code réessaie chaque demande en mode rapide rejetée à la vitesse et à la tarification standard, afin que vous continuiez à travailler, et il n'y a pas de refroidissement. La façon dont vous voyez le rejet dépend du type de session :

* Dans une session interactive, Claude Code affiche une notification « Mode rapide désactivé · crédits d'utilisation épuisés » et désactive le mode rapide pour le reste de la session. Votre préférence de mode rapide enregistrée ne change pas ; exécutez `/fast` pour réactiver le mode rapide.
* En [mode non-interactif](/docs/fr/headless) avec `--output-format stream-json`, et via le SDK Agent, Claude Code émet le même texte sur le flux de messages en tant que message `system` avec le sous-type `notification`, une fois par tour tant que vous n'avez pas de crédits d'utilisation. Le mode rapide reste activé. Nécessite Claude Code v2.1.221 ou version ultérieure.

<h2 id="research-preview">
  Aperçu de recherche
</h2>

Le mode rapide est une fonctionnalité d'aperçu de recherche. Cela signifie :

* La fonctionnalité peut changer en fonction des commentaires
* La disponibilité et la tarification sont sujettes à changement
* La configuration API sous-jacente peut évoluer

Signalez les problèmes ou les commentaires via vos canaux de support Anthropic habituels.

<h2 id="see-also">
  Voir aussi
</h2>

* [Configuration du modèle](/docs/fr/model-config) : basculer les modèles et ajuster les niveaux d'effort
* [Gérer les coûts efficacement](/docs/fr/costs) : suivre l'utilisation des jetons et réduire les coûts
* [Configuration de la ligne d'état](/docs/fr/statusline) : afficher les informations du modèle et du contexte
