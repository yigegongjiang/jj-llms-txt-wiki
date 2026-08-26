> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Accélérez les réponses avec le mode rapide

> Obtenez des réponses Opus plus rapides dans Claude Code en activant le mode rapide.

<Note>
  Le mode rapide est en [aperçu de recherche](#research-preview). La fonctionnalité, la tarification et la disponibilité peuvent changer en fonction des commentaires.
</Note>

Le mode rapide est une configuration haute vitesse pour Claude Opus, rendant le modèle jusqu'à 2,5 fois plus rapide à un coût par jeton plus élevé. Activez-le avec `/fast` quand vous avez besoin de vitesse pour un travail interactif comme l'itération rapide ou le débogage en direct, et désactivez-le quand le coût importe plus que la latence.

Le mode rapide n'est pas un modèle différent. Il utilise Claude Opus avec une configuration API différente qui priorise la vitesse plutôt que l'efficacité des coûts. Vous obtenez une qualité et des capacités identiques avec des réponses plus rapides. Le mode rapide est pris en charge sur Opus 4.8 et Opus 4.7. Il n'est pas disponible sur Sonnet, Haiku ou d'autres modèles.

<Warning>
  Le mode rapide pour Opus 4.7 est déprécié depuis le 25 juin 2026 et sera supprimé le 24 juillet 2026. Après la suppression, les demandes de mode rapide sur Opus 4.7 retournent une erreur et ne reviennent pas à la version standard d'Opus 4.7. Migrez vers Opus 4.8 pour conserver l'accélération.
</Warning>

Ce qu'il faut savoir :

* Utilisez `/fast` pour activer/désactiver le mode rapide dans Claude Code CLI. Le mode rapide n'est pas pris en charge dans l'extension VS Code.
* La tarification du mode rapide par MTok entrée/sortie est de 10 $/50 $ sur Opus 4.8 et de 30 $/150 $ sur Opus 4.7.
* Disponible pour tous les utilisateurs de Claude Code sur les plans d'abonnement (Pro/Max/Team/Enterprise) et Claude Console.
* Pour les utilisateurs de Claude Code sur les plans d'abonnement (Pro/Max/Team/Enterprise), le mode rapide est disponible via les crédits d'utilisation uniquement et n'est pas inclus dans les limites de taux d'abonnement.

<h2 id="toggle-fast-mode">
  Activer le mode rapide
</h2>

Activez le mode rapide de l'une de ces deux façons :

* Tapez `/fast` et appuyez sur Tab pour activer ou désactiver
* Définissez `"fastMode": true` dans votre [fichier de paramètres utilisateur](/docs/fr/settings)

Par défaut, le mode rapide que vous activez dans une session interactive persiste entre les sessions. En [mode non-interactif](/docs/fr/headless), avec le drapeau `-p`, `/fast` fonctionne uniquement dans une session lancée avec le mode rapide dans sa valeur [`--settings`](/docs/fr/cli-reference#cli-flags), par exemple `claude -p --settings '{"fastMode": true}'` ; le basculement s'applique alors uniquement à cette session et n'est pas enregistré comme votre paramètre par défaut, et dans toute autre session non-interactive, la commande signale que le mode rapide n'est pas disponible. Vous pouvez configurer le mode rapide pour qu'il se réinitialise à chaque session. Consultez [opt-in par session](#require-per-session-opt-in) pour plus de détails.

Pour la meilleure efficacité des coûts, activez le mode rapide au début d'une session plutôt que de basculer en milieu de conversation. Consultez [comprendre le compromis de coût](#understand-the-cost-tradeoff) pour plus de détails.

Quand vous activez le mode rapide :

* Si vous êtes sur un modèle différent, Claude Code bascule automatiquement vers Opus
* Vous verrez un message de confirmation : « Mode rapide ACTIVÉ »
* Une petite icône `↯` apparaît à côté de l'invite pendant que le mode rapide est actif
* Exécutez `/fast` à nouveau à tout moment pour vérifier si le mode rapide est activé ou désactivé

Quand vous désactivez le mode rapide avec `/fast` à nouveau, vous restez sur Opus. Le modèle ne revient pas à votre modèle précédent. Pour basculer vers un modèle différent, utilisez `/model`.

Basculer vers un modèle qui ne supporte pas le mode rapide désactive le mode rapide. Basculer vers un modèle Opus supporté le réactive quand votre préférence de mode rapide enregistrée est activée, la même préférence qu'une nouvelle session démarre par défaut. Avec [opt-in par session](#require-per-session-opt-in) configuré, basculer vers un modèle supporté ne réactive pas le mode rapide ; exécutez `/fast` pour le réactiver. Le mode rapide ne s'active jamais pour une session dont la préférence enregistrée est désactivée, et l'icône `↯` et la confirmation « Mode rapide ACTIVÉ » apparaissent chaque fois qu'il s'active. Avant v2.1.208, le mode rapide restait désactivé après que vous ayez basculé vers un modèle supporté jusqu'à ce que vous exécutiez `/fast` à nouveau.

Opus 4.8 est le mode rapide par défaut dans Claude Code v2.1.154 et ultérieur. Sur v2.1.142 à v2.1.153, le mode rapide utilise par défaut Opus 4.7.

<h2 id="understand-the-cost-tradeoff">
  Comprendre le compromis de coût
</h2>

Le mode rapide a une tarification par jeton plus élevée que l'Opus standard, avec un multiplicateur variant selon le modèle :

| Modèle   | Entrée (MTok) | Sortie (MTok) |
| -------- | ------------- | ------------- |
| Opus 4.8 | \$10          | \$50          |
| Opus 4.7 | \$30          | \$150         |

La tarification du mode rapide est plate sur toute la fenêtre de contexte de 1 million de jetons. Pour le tarif Opus standard à comparer, consultez la [référence de tarification Claude](https://platform.claude.com/docs/fr/about-claude/pricing).

La première fois que vous activez le mode rapide dans une conversation, vous payez le prix complet du jeton d'entrée non mis en cache du mode rapide pour tout le contexte de la conversation. Plus vous êtes avancé dans une conversation, plus cela coûte cher, donc activer le mode rapide dès le départ est moins cher. Le coût s'applique une seule fois par conversation, donc basculer le mode rapide hors ligne puis le réactiver plus tard ne le répète pas. Pour le mécanisme, consultez [comment le mode rapide interagit avec le cache de prompt](/docs/fr/prompt-caching#turning-on-fast-mode).

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

* **API Anthropic ou abonnement uniquement** : le mode rapide est disponible via l'API Anthropic Console et pour les plans d'abonnement Claude utilisant les crédits d'utilisation. Il n'est pas disponible sur Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry ou Claude Platform sur AWS.
* **Crédits d'utilisation activés** : votre compte doit avoir les crédits d'utilisation activés, ce qui permet la facturation au-delà de l'utilisation incluse dans votre plan. Pour les comptes individuels, activez ceci dans vos [paramètres de facturation Console](https://platform.claude.com/settings/billing). Pour Teams et Enterprise, un administrateur doit activer les crédits d'utilisation pour l'organisation.

<Note>
  L'utilisation du mode rapide est facturée directement à partir des crédits d'utilisation, même si vous avez une utilisation restante sur votre plan. Cela signifie que les jetons du mode rapide ne comptent pas par rapport à l'utilisation incluse de votre plan et sont facturés au tarif du mode rapide à partir du premier jeton.
</Note>

* **Activation par le propriétaire pour Team et Enterprise** : le mode rapide est désactivé par défaut pour les organisations Team et Enterprise. Un propriétaire doit explicitement [activer le mode rapide](#enable-fast-mode-for-your-organization) avant que les utilisateurs puissent y accéder.

<Note>
  Si le mode rapide n'a pas été activé pour votre organisation, la commande `/fast` affichera « Le mode rapide a été désactivé par votre organisation. » Si la liste d'autorisation [`availableModels`](/docs/fr/model-config#restrict-model-selection) de votre organisation exclut le modèle Opus du mode rapide, `/fast` est refusé avec « n'est pas dans les modèles autorisés de votre organisation ». L'exception est une session déjà en cours d'exécution sur un modèle Opus autorisé qui prend en charge le mode rapide : `/fast` active le mode rapide sur votre modèle actuel au lieu de changer de modèle.
</Note>

<h3 id="enable-fast-mode-for-your-organization">
  Activer le mode rapide pour votre organisation
</h3>

Où vous activez le mode rapide dépend du produit que votre organisation utilise :

* **Console** (clients API) : un administrateur l'active dans les [préférences Claude Code](https://platform.claude.com/claude-code/preferences)
* **Claude AI** (Team et Enterprise) : un propriétaire l'active dans [Paramètres d'administration > Claude Code](https://claude.ai/admin-settings/claude-code)

Une autre option pour désactiver complètement le mode rapide est de définir `CLAUDE_CODE_DISABLE_FAST_MODE=1`. Consultez [Variables d'environnement](/docs/fr/env-vars).

<h3 id="require-per-session-opt-in">
  Opt-in par session
</h3>

Par défaut, le mode rapide qu'un utilisateur active dans une session interactive persiste entre les sessions : il reste activé dans les sessions futures. Pour modifier ceci, définissez `fastModePerSessionOptIn` à `true` dans n'importe quel [fichier de paramètres](/docs/fr/settings#settings-files), ce qui fait que chaque session commence avec le mode rapide désactivé et oblige les utilisateurs à l'activer explicitement avec `/fast`. Les propriétaires sur les plans [Team](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_teams#team-&-enterprise) ou [Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_enterprise) peuvent le déployer à l'échelle de l'organisation via les [paramètres gérés par le serveur](/docs/fr/server-managed-settings).

```json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

Ceci est utile pour contrôler les coûts dans les organisations où les utilisateurs exécutent plusieurs sessions simultanées. Les utilisateurs peuvent toujours activer le mode rapide avec `/fast` quand ils ont besoin de vitesse, mais il se réinitialise au début de chaque nouvelle session. La préférence du mode rapide de l'utilisateur est toujours enregistrée, donc supprimer ce paramètre restaure le comportement persistant par défaut.

<h2 id="handle-rate-limits">
  Gérer les limites de taux
</h2>

Le mode rapide a des limites de taux séparées de l'Opus standard. Le mode rapide sur Opus 4.8 et Opus 4.7 partagent le même pool de limites de taux : l'utilisation sur l'un d'entre eux puise dans les mêmes limites. Quand vous atteignez la limite de taux du mode rapide ou que vous manquez de crédits d'utilisation :

1. Le mode rapide bascule automatiquement vers la vitesse standard
2. L'icône `↯` devient grise pour indiquer le refroidissement
3. Vous continuez à travailler à la vitesse et à la tarification standard
4. Quand le refroidissement expire, le mode rapide se réactive automatiquement

Pour désactiver manuellement le mode rapide au lieu d'attendre le refroidissement, exécutez `/fast` à nouveau.

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
