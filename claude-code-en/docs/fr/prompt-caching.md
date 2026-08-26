> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Comment Claude Code utilise le prompt caching

> Claude Code gère le prompt caching automatiquement. Découvrez pourquoi un changement de modèle déclenche un tour lent sans cache, ce que coûte `/compact`, pourquoi les modifications de CLAUDE.md ne s'appliquent pas en cours de session, et comment vérifier votre taux de cache hit.

Le prompt caching rend Claude Code plus rapide et plus rentable. Sans caching, l'API retraiterait votre historique complet à chaque tour. Avec le caching, elle réutilise ce qu'elle a déjà traité et ne fait du nouveau travail que pour ce qui a changé.

Claude Code gère le prompt caching pour vous, sauf si vous le [désactivez](#disable-prompt-caching). Il est néanmoins utile de comprendre comment fonctionne le prompt caching, car certaines actions invalident le cache et rendent la réponse suivante plus lente et plus coûteuse pendant qu'il se reconstruit. Cette page couvre les actions qui le font, pourquoi certains paramètres attendent un redémarrage pour s'appliquer, et comment vérifier les performances du cache quand l'utilisation semble élevée.

<h2 id="how-the-cache-is-organized">
  Comment le cache est organisé
</h2>

Chaque fois que vous envoyez un message dans Claude Code, il effectue une nouvelle requête API. Le modèle ne se souvient de rien entre les requêtes, donc Claude Code renvoie le contexte complet : le prompt système, votre contexte de projet, chaque message antérieur et résultat d'outil, et votre nouveau message. Le nouveau contenu est ajouté à la fin, ce qui signifie que la plupart de chaque requête est identique à celle précédente. Le prompt caching est la façon dont l'API évite de retraiter la partie qui n'a pas changé.

L'API met en cache en faisant correspondre le début de chaque requête, appelé le préfixe, avec le contenu qu'elle a récemment traité. À un tour normal, le préfixe est la requête entière précédente et seul l'échange le plus récent est nouveau. La correspondance est exacte, donc un changement n'importe où dans le préfixe recalcule tout ce qui suit. Il n'y a pas de caching par fichier ou par segment. Voir [comment fonctionne le prompt caching](https://platform.claude.com/docs/fr/build-with-claude/prompt-caching#how-prompt-caching-works) dans la référence API pour le mécanisme sous-jacent.

<img src="https://mintcdn.com/claude-code/VbDJw--l6T9a9Wvm/images/prompt-caching-prefix.svg?fit=max&auto=format&n=VbDJw--l6T9a9Wvm&q=85&s=f2e8f0b8298a50305fe428ca3f1d1594" className="dark:hidden" alt="Quatre tours affichés sous forme de barres horizontales croissantes. La requête de chaque tour contient tout ce qui provient du tour précédent plus l'échange le plus récent ajouté à la fin. Aux tours deux et trois, le préfixe inchangé est lu à partir du cache et seul le nouvel échange est traité. Au tour quatre, le prompt système a changé, donc le préfixe ne correspond plus et la requête entière est retraitée et écrite." width="720" height="454" data-path="images/prompt-caching-prefix.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/prompt-caching-prefix-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=297dc1c639f0915cae858d0c4b6f3be5" className="hidden dark:block" alt="Quatre tours affichés sous forme de barres horizontales croissantes. La requête de chaque tour contient tout ce qui provient du tour précédent plus l'échange le plus récent ajouté à la fin. Aux tours deux et trois, le préfixe inchangé est lu à partir du cache et seul le nouvel échange est traité. Au tour quatre, le prompt système a changé, donc le préfixe ne correspond plus et la requête entière est retraitée et écrite." width="720" height="454" data-path="images/prompt-caching-prefix-dark.svg" />

Pour tirer le meilleur parti de la correspondance de préfixe, Claude Code organise chaque requête de sorte que le contenu qui change rarement entre les tours vient en premier :

| Couche             | Contenu                                                         | Change quand                                                                         |
| ------------------ | --------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| Prompt système     | Instructions principales, définitions d'outils, style de sortie | L'ensemble des définitions d'outils chargées change, ou Claude Code est mis à niveau |
| Contexte du projet | CLAUDE.md, mémoire automatique, règles non délimitées           | La session démarre, ou après `/clear` ou `/compact`                                  |
| Conversation       | Vos messages, les réponses de Claude, les résultats d'outils    | À chaque tour                                                                        |

Un changement à la couche conversation laisse le prompt système et le contexte du projet en cache. Un changement au prompt système invalide tout, car tout le contenu ultérieur se trouve maintenant derrière un préfixe différent. La troisième colonne donne les déclencheurs courants plutôt qu'une liste exhaustive, et les sections ci-dessous couvrent l'ensemble complet, y compris le contenu tel que le style de sortie qui est fixé au démarrage de la session.

La règle de correspondance de préfixe explique la plupart des comportements sur cette page. Le [mode plan](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode) et le [chargement de compétences](/docs/fr/skills), par exemple, ajoutent leurs instructions comme messages de conversation, de sorte que le préfixe en cache reste intact.

Deux paramètres ne font pas du tout partie du texte du prompt, donc ils n'apparaissent pas dans le tableau des couches, mais tous deux font partie de la clé de cache :

* **Modèle** : chaque modèle a son propre cache. Changer de modèle recalcule la requête entière même quand le contenu est identique. Voir [Changer de modèle](#switching-models) ci-dessous.
* **Niveau d'effort** : chaque niveau d'effort a son propre cache pour le même modèle. Le changer en cours de session recalcule la requête entière, et Claude Code vous demande de confirmer avant d'appliquer le changement. Voir [Changer le niveau d'effort](#changing-effort-level) ci-dessous.

<Tip>
  Choisissez votre modèle et votre niveau d'effort au début d'une session, puis réservez `/compact` pour les pauses naturelles entre les tâches. Moins vous faites de changements en cours de tâche, plus votre taux de cache hit est élevé.
</Tip>

<h3 id="where-the-cache-lives">
  Où vit le cache
</h3>

Le caching se produit côté serveur, dans l'infrastructure qui sert votre modèle. L'endroit où cela se trouve dépend de la façon dont vous vous authentifiez :

* **Clé API, abonnement Claude, ou [Claude Platform on AWS](/docs/fr/claude-platform-on-aws)** : le cache vit dans l'infrastructure d'Anthropic, accessible via l'[API Claude](https://platform.claude.com/docs)
* **Amazon Bedrock ou Google Cloud's Agent Platform** : le cache vit dans l'infrastructure de service de votre fournisseur cloud
* **Microsoft Foundry** : les requêtes sont acheminées vers l'infrastructure d'Anthropic
* **`ANTHROPIC_BASE_URL` personnalisé ou [passerelle LLM](/docs/fr/llm-gateway)** : le cache vit là où vos requêtes sont transférées, et le fonctionnement du caching dépend de la passerelle

Pour ce que chaque fournisseur stocke et traite, voir [utilisation des données](/docs/fr/data-usage). Où que le cache vive, les entrées expirent après une période d'inactivité, et [Durée de vie du cache](#cache-lifetime) ci-dessous couvre le TTL et comment l'étendre.

<h2 id="actions-that-invalidate-the-cache">
  Actions qui invalident le cache
</h2>

Ces actions font que la requête suivante manque une partie ou la totalité du cache. Vous voyez un tour plus lent et plus coûteux une seule fois, après quoi le nouveau préfixe est mis en cache. La plupart d'entre elles sont évitables en cours de tâche une fois que vous savez qu'elles ont un coût. Un changement de modèle peut sembler gratuit jusqu'à ce que vous remarquiez le tour plus lent qui suit.

* [Changer de modèle](#switching-models)
* [Modifier le niveau d'effort](#changing-effort-level)
* [Activer le mode rapide](#turning-on-fast-mode)
* [Connecter ou déconnecter un serveur MCP](#connecting-or-disconnecting-an-mcp-server)
* [Activer ou désactiver un plugin](#enabling-or-disabling-a-plugin)
* [Refuser un outil entier](#denying-an-entire-tool)
* [Compacter la conversation](#compacting-the-conversation)
* [Mettre à niveau Claude Code](#upgrading-claude-code)

<h3 id="switching-models">
  Changer de modèle
</h3>

Chaque modèle a son propre cache. Changer avec [`/model`](/docs/fr/model-config#setting-your-model) signifie que la requête suivante lit l'historique de conversation entier sans cache hits, même si le contenu est identique.

Le [paramètre de modèle `opusplan`](/docs/fr/model-config#opusplan-model-setting) se résout en Opus pendant le mode plan et Sonnet pendant l'exécution, donc chaque basculement de mode plan est un changement de modèle et démarre un cache frais.

Le [basculement automatique du modèle](/docs/fr/model-config#automatic-model-fallback) sur Fable 5 est également un changement de modèle. Quand un classificateur de sécurité signale une requête, Claude Code la réexécute sur le modèle Opus par défaut et la session continue là.

<h3 id="changing-effort-level">
  Modifier le niveau d'effort
</h3>

Le cache est indexé par [niveau d'effort](/docs/fr/model-config#adjust-effort-level) ainsi que par modèle, donc changer avec `/effort` signifie que la requête suivante lit l'historique de conversation entier sans cache hits. Une fois qu'une conversation a commencé, Claude Code affiche une boîte de dialogue de confirmation avant d'appliquer un changement d'effort qui invaliderait le cache. Un changement qui se résout au même niveau déjà en vigueur, comme définir explicitement la valeur par défaut du modèle, ignore la boîte de dialogue et conserve le cache.

<h3 id="turning-on-fast-mode">
  Activer le mode rapide
</h3>

L'activation du [mode rapide](/docs/fr/fast-mode) ajoute un en-tête de requête qui fait partie de la clé de cache, donc la requête suivante lit l'historique de conversation entier sans cache hits. Ces jetons d'entrée non mis en cache sont facturés aux [tarifs du mode rapide](/docs/fr/fast-mode#understand-the-cost-tradeoff), c'est pourquoi l'activer au début d'une session coûte moins cher que de l'activer profondément dans une longue session. L'activation du mode rapide à partir d'un modèle non-Opus [bascule également votre modèle](#switching-models), ce qui démarre un cache frais en soi.

Le coût s'applique une fois par conversation. Après le premier tour en mode rapide, Claude Code continue d'envoyer l'en-tête et varie uniquement le paramètre de vitesse de la requête, qui ne fait pas partie de la clé de cache. Désactiver le mode rapide, le [basculement automatique vers la vitesse standard](/docs/fr/fast-mode#handle-rate-limits) après une limite de débit, et le réactiver plus tard conservent tous le cache. `/clear` et `/compact` réinitialisent cela, puisqu'ils reconstruisent le cache à ces points de toute façon.

<h3 id="connecting-or-disconnecting-an-mcp-server">
  Connecter ou déconnecter un serveur MCP
</h3>

Les définitions d'outils se trouvent dans la couche du prompt système, donc le cache s'invalide quand l'ensemble des définitions d'outils dans la requête change entre les tours. Basculer l'[outil conseiller](/docs/fr/advisor) est une exception : sa définition se trouve après le point de rupture du cache, donc activer ou désactiver `/advisor` conserve le préfixe mis en cache intact. Qu'un changement de [serveur MCP](/docs/fr/mcp) fasse cela dépend de si ses outils sont différés par la [recherche d'outils](/docs/fr/mcp#scale-with-mcp-tool-search) ou chargés dans le préfixe :

* **Outils différés**, la valeur par défaut sur les modèles supportés : un serveur qui se connecte, se déconnecte, ou change sa liste d'outils n'ajoute que du nouveau contenu et ne perturbe rien de ce qui est déjà en cache.
* **Outils chargés dans le préfixe** : tout changement à leur égard invalide le cache. Cela se produit quand la [recherche d'outils n'est pas disponible ou est désactivée](/docs/fr/mcp#configure-tool-search), comme sur Google Cloud's Agent Platform ou avec une passerelle `ANTHROPIC_BASE_URL` personnalisée. Cela se produit également pour un serveur ou un outil marqué [`alwaysLoad`](/docs/fr/mcp#exempt-a-server-from-deferral), et pour les définitions conservées en avant par le [chargement basé sur le seuil](/docs/fr/mcp#configure-tool-search).

Quand les outils se chargent dans le préfixe, la cause la plus courante d'une invalidation est un serveur qui se connecte ou se déconnecte en cours de session, ce qui peut se produire sans aucune action de votre part : le processus d'un serveur stdio se termine, une session HTTP expire, ou un serveur se [reconnecte automatiquement après une défaillance transitoire](/docs/fr/mcp#automatic-reconnection). Un serveur connecté peut également envoyer une [mise à jour d'outil dynamique](/docs/fr/mcp#dynamic-tool-updates) qui change sa liste d'outils.

Éditer votre configuration MCP ne change pas le cache en soi. La nouvelle configuration ne prend effet qu'après un redémarrage, c'est à ce moment que le serveur se connecte ou se déconnecte.

<h3 id="enabling-or-disabling-a-plugin">
  Activer ou désactiver un plugin
</h3>

Les [plugins](/docs/fr/plugins) regroupent plusieurs types de composants, et le coût d'un changement dépend des composants que le plugin fournit. Les skills, les commandes, les agents, les hooks, les serveurs LSP, les moniteurs et les thèmes n'invalident jamais le cache : tout ce qu'ils ajoutent à la requête est ajouté après la conversation existante, donc la requête suivante paie pour le nouveau contenu mais lit toujours tout ce qui le précède à partir du cache.

L'exception est un plugin qui fournit des [serveurs MCP](/docs/fr/plugins-reference#mcp-servers). Activer ou désactiver l'un d'eux suit les mêmes règles que [connecter ou déconnecter un serveur MCP](#connecting-or-disconnecting-an-mcp-server) : le cache survit quand les outils du serveur sont différés, et la requête suivante relit la conversation entière quand ils se chargent dans le préfixe.

Les changements de plugin s'appliquent quand vous exécutez [`/reload-plugins`](/docs/fr/discover-plugins#apply-plugin-changes-without-restarting) ou démarrez une nouvelle session. Le coût, qu'il s'agisse d'annonces ajoutées ou d'une relecture complète, s'affiche au premier tour après le rechargement, pas quand vous exécutez `/plugin install`, `/plugin enable`, ou `/plugin disable`. À partir de la v2.1.163, quand un rechargement déclencherait la relecture complète, `/reload-plugins` affiche un avertissement et n'applique pas le rechargement. Passez `--force` pour appliquer de toute façon.

Désactiver un plugin que vous avez activé plus tôt dans la session restaure la forme de requête précédente. Si ce préfixe se trouve toujours dans sa [durée de vie du cache](#cache-lifetime), la requête suivante lit l'entrée de cache plus ancienne au lieu de la reconstruire.

<h3 id="denying-an-entire-tool">
  Refuser un outil entier
</h3>

Ajouter un nom d'outil simple comme `Bash` ou `WebFetch` comme [règle de refus](/docs/fr/permissions#manage-permissions) supprime cet outil du contexte de Claude entièrement. Les définitions d'outils intégrés se chargent dans la couche du prompt système, donc ajouter ou supprimer l'une de ces règles en cours de session invalide le cache. Le changement prend effet au tour suivant, que vous l'ajoutiez via `/permissions` ou en [éditant un fichier de paramètres directement](/docs/fr/settings#when-edits-take-effect).

Seul un nom d'outil simple, la forme équivalente `Bash(*)`, ou un [glob de nom d'outil](/docs/fr/permissions#tool-name-wildcards) comme `"*"` a cet effet. Un glob qui correspond uniquement aux outils MCP, comme `"mcp__*"`, supprime ces outils de la même manière mais laisse le cache intact quand les outils correspondants sont [différés](#connecting-or-disconnecting-an-mcp-server), la valeur par défaut, puisque les définitions différées n'étaient jamais dans le préfixe mis en cache. Les règles de refus délimitées comme `Bash(rm *)`, et toutes les règles d'autorisation et de demande, ne changent pas les outils que Claude voit. Claude Code les vérifie quand Claude tente un appel, laissant le préfixe intact.

<h3 id="compacting-the-conversation">
  Compacter la conversation
</h3>

La [compaction](/docs/fr/context-window#what-survives-compaction) remplace votre historique de messages par un résumé. Par conception, cela invalide la couche conversation, puisque la requête suivante a un nouvel historique plus court qui ne partage pas de préfixe avec l'ancien. Claude Code réutilise la couche du prompt système et recharge le contexte du projet à partir du disque, qui ne cache que si CLAUDE.md et la mémoire sont inchangés depuis le début de la session.

Pour produire le résumé, Claude Code envoie une requête unique avec le même prompt système, les mêmes outils et le même historique que votre conversation, plus une instruction de résumé ajoutée comme dernier message utilisateur. Parce qu'elle partage votre préfixe, cette requête lit le cache existant plutôt que de retraiter l'historique complet. La plupart du temps de compaction va à la génération du résumé, pas à un cache miss. Le tour qui suit reconstruit le cache de conversation uniquement pour le résumé beaucoup plus court, donc le tour post-compaction n'est pas la partie lente.

<Tip>
  La compaction joue en votre faveur quand le contexte que vous abandonnez est du contenu dont vous n'avez plus besoin. Pour choisir quand son surcoût se produit, exécutez `/compact` à une pause naturelle dans votre travail, comme entre les tâches, au lieu d'attendre que la compaction automatique se déclenche en cours de tâche. Si vous avez suivi un chemin que vous voulez abandonner entièrement, [`/rewind`](#rewinding-the-conversation) à un tour antérieur à la place. Le rembobinage tronque jusqu'à un préfixe qui est déjà en cache, plutôt que de construire un nouveau comme le fait la compaction.
</Tip>

<h3 id="upgrading-claude-code">
  Mettre à niveau Claude Code
</h3>

Une nouvelle version de Claude Code met généralement à jour le prompt système ou les définitions d'outils, donc la première requête après une mise à niveau reconstruit le cache à partir du début. La [mise à jour automatique](/docs/fr/setup#auto-updates) télécharge les nouvelles versions en arrière-plan mais les applique au prochain lancement, jamais en cours de session, donc vous voyez cela comme un premier tour sans cache après redémarrage plutôt qu'une surprise pendant une session. Définissez `DISABLE_AUTOUPDATER=1` pour contrôler quand les mises à niveau s'appliquent.

<Note>
  [Reprendre une session](/docs/fr/sessions#resume-a-session) après une mise à niveau retraite l'historique de conversation entier sans cache hits, puisque l'historique se trouve maintenant derrière un prompt système différent. Le coût s'adapte à la longueur de la conversation reprise, donc le premier tour de retour dans une longue session peut être la requête la plus coûteuse que vous envoyez.
</Note>

<h2 id="actions-that-keep-the-cache">
  Actions qui conservent le cache
</h2>

Ces actions ajoutent soit à la fin de la conversation, soit ne touchent pas du tout la requête. Certaines d'entre elles, comme éditer CLAUDE.md ou changer le style de sortie, sont aussi pourquoi un changement de paramètre attend un redémarrage pour s'appliquer.

* [Éditer des fichiers dans votre référentiel](#editing-files-in-your-repository)
* [Éditer CLAUDE.md en cours de session](#editing-claude-md-mid-session)
* [Changer le style de sortie](#changing-output-style)
* [Changer le mode de permission](#changing-permission-mode)
* [Invoquer des compétences et des commandes](#invoking-skills-and-commands)
* [Exécuter `/recap`](#running-%2Frecap)
* [Rembobiner la conversation](#rewinding-the-conversation)
* [Générer un sous-agent](#subagents-and-the-cache)

<h3 id="editing-files-in-your-repository">
  Éditer des fichiers dans votre référentiel
</h3>

Le contenu des fichiers entre en contexte uniquement quand Claude les lit, et les lectures s'ajoutent à la conversation. Éditer un fichier que Claude a précédemment lu ne change pas rétroactivement la lecture antérieure dans l'historique. Au lieu de cela, Claude Code ajoute un `<system-reminder>` notant que le fichier a changé, et Claude le relit si nécessaire.

<h3 id="editing-claude-md-mid-session">
  Éditer CLAUDE.md en cours de session
</h3>

Vos fichiers CLAUDE.md au niveau de la racine du projet et au niveau utilisateur sont lus une fois au démarrage de la session et conservés en mémoire. Les éditer en cours de session n'invalide pas le cache, mais l'édition ne s'applique pas non plus. Claude continue de travailler avec la version qui a été chargée au démarrage de la session. Le nouveau contenu se charge au prochain `/clear`, `/compact`, ou redémarrage.

Les [fichiers CLAUDE.md imbriqués dans les sous-répertoires](/docs/fr/memory) et les [règles avec frontmatter `paths:`](/docs/fr/memory#path-specific-rules) se chargent plus tard, quand Claude lit pour la première fois un fichier correspondant. Éditer un avant qu'il se charge prend effet. Après son chargement, le contenu fait partie de l'historique de conversation, donc une édition en cours de session ne le change pas rétroactivement.

<h3 id="changing-output-style">
  Changer le style de sortie
</h3>

Le [style de sortie](/docs/fr/output-styles) fait partie du prompt système, que Claude Code lit une fois au démarrage de la session. Le changer via `/config` ou le paramètre `outputStyle` en cours de session n'invalide pas le cache, mais le changement ne s'applique pas non plus. Claude continue d'utiliser le style qui a été chargé au démarrage de la session. Le nouveau style se charge au prochain `/clear` ou redémarrage.

<h3 id="changing-permission-mode">
  Changer le mode de permission
</h3>

Basculer entre les [modes de permission](/docs/fr/permission-modes), comme du défaut à accepter les éditions, ne change pas le prompt système ou les définitions d'outils, donc les changements de mode sont sûrs pour le cache. L'exception est le mode plan avec le paramètre de modèle [`opusplan`](/docs/fr/model-config#opusplan-model-setting), qui bascule le modèle entre Opus et Sonnet quand vous entrez ou quittez le mode plan. Cela rend le basculement de mode un [changement de modèle](#switching-models).

<h3 id="invoking-skills-and-commands">
  Invoquer des compétences et des commandes
</h3>

Les [compétences](/docs/fr/skills) et les [commandes](/docs/fr/commands) injectent leurs instructions comme messages utilisateur au point d'invocation. Rien d'antérieur dans la conversation ne change.

<h3 id="running-/recap">
  Exécuter `/recap`
</h3>

[`/recap`](/docs/fr/interactive-mode#session-recap) génère un résumé pour l'affichage dans votre terminal. Contrairement à `/compact`, il ajoute le résumé comme sortie de commande plutôt que de remplacer votre historique de messages, de sorte que le préfixe en cache reste intact.

<h3 id="rewinding-the-conversation">
  Rembobiner la conversation
</h3>

[`/rewind`](/docs/fr/checkpointing) tronque votre conversation jusqu'à un tour antérieur. L'historique restant est le même contenu à partir duquel le cache a été construit à ce moment, et les couches du prompt système et du contexte du projet sont inchangées, donc la requête suivante atteint l'entrée de cache antérieure. Chaque tour depuis a lu ce préfixe, ce qui a gardé l'entrée active même si le tour original était plus loin que le TTL.

Restaurer les points de contrôle de fichiers aux côtés de la conversation n'a aucun effet séparé sur le cache. Le contenu des fichiers entre en contexte uniquement quand Claude les lit, comme [éditer des fichiers dans votre référentiel](#editing-files-in-your-repository).

<h2 id="cache-lifetime">
  Durée de vie du cache
</h2>

Les préfixes en cache expirent après une période d'inactivité. Chaque requête qui atteint le cache réinitialise le minuteur, de sorte que le cache reste actif tant que vous continuez à travailler. Après un écart assez long, la requête suivante recalcule l'entrée complète et rétablit le cache, ce qui est pourquoi le premier tour après s'être éloigné peut être notablement plus lent.

Le time to live (TTL) contrôle la durée de l'écart que le cache survit. L'API en offre deux : un TTL de cinq minutes, et un [TTL d'une heure](https://platform.claude.com/docs/fr/build-with-claude/prompt-caching#1-hour-cache-duration) qui garde le cache actif pendant les pauses plus longues mais [facture les écritures de cache à un taux plus élevé](https://platform.claude.com/docs/fr/build-with-claude/prompt-caching#pricing). Claude Code choisit le TTL pour vous en fonction de la façon dont vous vous authentifiez, et vous pouvez le remplacer avec des variables d'environnement.

<h3 id="on-a-claude-subscription">
  Sur un abonnement Claude
</h3>

Sur un abonnement Claude, Claude Code demande automatiquement le TTL d'une heure. L'utilisation est incluse dans votre plan plutôt que facturée par jeton, donc le TTL plus long ne vous coûte rien de plus et affecte uniquement la durée pendant laquelle votre cache reste actif.

Si vous avez dépassé la limite d'utilisation de votre plan et que Claude Code puise dans les [crédits d'utilisation](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans), vous êtes facturé pour cette utilisation, donc Claude Code baisse automatiquement le TTL à cinq minutes.

<h3 id="on-an-api-key-or-third-party-provider">
  Sur une clé API ou un fournisseur tiers
</h3>

Sur une clé API, Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, ou Claude Platform on AWS, vous payez les tarifs par jeton, donc le TTL reste aux cinq minutes moins chers par défaut. Pour opter pour le [TTL d'une heure](https://platform.claude.com/docs/fr/build-with-claude/prompt-caching#1-hour-cache-duration), définissez `ENABLE_PROMPT_CACHING_1H=1`.

Sur Amazon Bedrock, le support du prompt caching, la longueur minimale du préfixe cacheable, et la disponibilité du TTL d'une heure varient tous selon le modèle. Si les comptages de jetons de cache restent à zéro, vérifiez les [modèles, régions et limites pris en charge](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models) dans la documentation Amazon Bedrock.

<h3 id="override-the-ttl">
  Remplacer le TTL
</h3>

Définissez `FORCE_PROMPT_CACHING_5M=1` pour forcer le TTL de cinq minutes indépendamment de l'authentification. Ceci est utile quand vous déboguez le comportement du cache, comparez les deux TTL, ou remplacez un `ENABLE_PROMPT_CACHING_1H` défini dans les [paramètres gérés](/docs/fr/settings#settings-files).

<h2 id="cache-scope">
  Portée du cache
</h2>

Dans Claude Code, le cache est effectivement limité à une machine et un répertoire. Le prompt système intègre le répertoire de travail, la plateforme, le shell, la version du système d'exploitation, et les chemins de mémoire automatique, donc deux sessions dans des répertoires différents construisent des préfixes différents et manquent le cache de l'autre. Cela inclut les worktrees du même référentiel, puisque chaque worktree a son propre répertoire de travail.

Les sessions que vous exécutez en parallèle dans le même répertoire construisent des préfixes correspondants et lisent le cache de l'autre. Les sessions séquentielles partagent le préfixe uniquement quand l'instantané du statut git au démarrage correspond, puisque le prompt système capture également la branche et les commits récents.

Le cache API sous-jacent est plus large. Les caches sont isolés entre les organisations, et sur certains fournisseurs, [entre les espaces de travail au sein d'une organisation](https://platform.claude.com/docs/fr/build-with-claude/prompt-caching#cache-storage-and-sharing). Dans ces limites, deux requêtes quelconques avec le même modèle et préfixe lisent le même cache. Pour les appelants du SDK Agent exécutant des flottes de processus automatisés, voir [améliorer le prompt caching entre les utilisateurs et les machines](/docs/fr/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) pour supprimer les sections par machine du prompt système et partager le cache entre les machines.

<h2 id="check-cache-performance">
  Vérifier les performances du cache
</h2>

Les performances du cache s'affichent comme deux comptages de jetons que l'API rapporte sur chaque réponse. Le moyen le plus direct de les regarder en direct est un [script de ligne d'état](/docs/fr/statusline) qui lit l'objet `current_usage` :

| Champ                         | Signification                                                                                |
| ----------------------------- | -------------------------------------------------------------------------------------------- |
| `cache_creation_input_tokens` | Jetons écrits dans le cache à ce tour, facturés au taux d'écriture du cache                  |
| `cache_read_input_tokens`     | Jetons servis à partir du cache à ce tour, facturés à environ 10 % du taux d'entrée standard |

Un ratio lecture-création élevé signifie que le caching fonctionne bien. Si la création reste élevée tour après tour, quelque chose change dans votre préfixe. La section [Actions qui invalident le cache](#actions-that-invalidate-the-cache) énumère les causes habituelles.

Pour la visibilité dans une organisation, l'exportateur OpenTelemetry rapporte les jetons de lecture et de création du cache par utilisateur et session. Voir [Surveiller l'utilisation](/docs/fr/monitoring-usage) pour la référence des attributs de métrique et d'événement.

<h2 id="subagents-and-the-cache">
  Sous-agents et le cache
</h2>

Un [sous-agent](/docs/fr/sub-agents) démarre sa propre conversation avec son propre prompt système et ensemble d'outils, séparé du parent. Il construit son propre cache, en commençant sans cache hits à son premier appel et en se réchauffant à travers ses propres tours. Les sous-agents utilisent le TTL de cinq minutes même sur un abonnement, puisque le TTL d'une heure automatique s'applique à la conversation principale.

Le cache du parent n'est pas affecté. Du côté du parent, l'appel du sous-agent et le résultat s'ajoutent à la conversation, laissant le préfixe du parent intact.

Un [fork](/docs/fr/sub-agents#fork-the-current-conversation), en contraste, hérite du prompt système du parent, des outils, et de l'historique de conversation exactement, donc sa première requête lit le cache du parent. L'appel de résumé de compaction décrit dans [Compacter la conversation](#compacting-the-conversation) utilise la même approche de partage de préfixe.

<h2 id="disable-prompt-caching">
  Désactiver le prompt caching
</h2>

Désactiver le caching est occasionnellement utile quand on débogue le comportement du caching avec un modèle ou un fournisseur spécifique. Pour l'éteindre, définissez l'une de ces variables d'environnement à `1` :

| Variable                        | Effet                             |
| ------------------------------- | --------------------------------- |
| `DISABLE_PROMPT_CACHING`        | Désactiver pour tous les modèles  |
| `DISABLE_PROMPT_CACHING_HAIKU`  | Désactiver pour Haiku uniquement  |
| `DISABLE_PROMPT_CACHING_SONNET` | Désactiver pour Sonnet uniquement |
| `DISABLE_PROMPT_CACHING_OPUS`   | Désactiver pour Opus uniquement   |
| `DISABLE_PROMPT_CACHING_FABLE`  | Désactiver pour Fable uniquement  |

Pour définir la politique de caching dans une organisation, mettez l'une de ces variables ou les [variables TTL](#cache-lifetime) dans le bloc `env` des [paramètres gérés](/docs/fr/settings#settings-files). Pour un usage normal, laissez le caching activé.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Leçons de la construction de Claude Code : Le prompt caching est tout](https://claude.com/blog/lessons-from-building-claude-code-prompt-caching-is-everything) : la justification de la conception pour le mode plan, le chargement d'outils différé, et la compaction
* [Explorer la fenêtre de contexte](/docs/fr/context-window) : ce qui se charge en contexte et quand
* [Réduire l'utilisation des jetons](/docs/fr/costs#reduce-token-usage) : stratégies au-delà du caching pour gérer la taille du contexte
* [Suivre et réduire les coûts](/docs/fr/agent-sdk/cost-tracking) : suivi des jetons de cache et configuration du TTL pour les appelants du SDK Agent
* [Prompt caching](https://platform.claude.com/docs/fr/build-with-claude/prompt-caching) : le mécanisme API sous-jacent, les points d'arrêt, et la tarification
