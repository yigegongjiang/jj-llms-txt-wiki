> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Comment Claude Code utilise le prompt caching

> Claude Code gère le prompt caching automatiquement. Découvrez pourquoi un changement de modèle déclenche un tour lent sans cache, ce que coûte `/compact`, pourquoi les modifications de CLAUDE.md ne s'appliquent pas en cours de session, et comment vérifier votre taux de cache hit.

Le prompt caching rend Claude Code plus rapide et plus rentable. Sans caching, l'API retraiterait votre historique complet à chaque tour. Avec le caching, elle réutilise ce qu'elle a déjà traité, facture la relecture au [taux de cache](https://platform.claude.com/docs/en/about-claude/pricing), et ne traite complètement que ce qui a changé.

Claude Code gère le prompt caching pour vous, sauf si vous le [désactivez](#disable-prompt-caching). Il est néanmoins utile de comprendre comment fonctionne le prompt caching, car certaines actions invalident le cache et rendent la réponse suivante plus lente et plus coûteuse pendant qu'il se reconstruit. Cette page couvre les actions qui le font, pourquoi certains paramètres attendent un redémarrage pour s'appliquer, et comment vérifier les performances du cache quand l'utilisation semble élevée.

<h2 id="how-the-cache-is-organized">
  Comment le cache est organisé
</h2>

Chaque fois que vous envoyez un message dans Claude Code, il effectue une nouvelle requête API. Le modèle ne se souvient de rien entre les requêtes, donc Claude Code renvoie le contexte complet : l'invite système, votre contexte de projet, tous les messages et résultats d'outils précédents, et votre nouveau message. Le nouveau contenu est ajouté à la fin, ce qui signifie que la plupart de chaque requête est identique à celle précédente. La mise en cache des invites est la façon dont l'API évite de retraiter la partie qui n'a pas changé.

L'API met en cache en faisant correspondre le début de chaque requête, appelé le préfixe, avec le contenu qu'elle a récemment traité. À un tour normal, le préfixe est la requête entière précédente et seul l'échange le plus récent est nouveau. La correspondance est exacte, donc une modification n'importe où dans le préfixe recalcule tout ce qui suit. Il n'y a pas de mise en cache par fichier ou par segment. Consultez [comment fonctionne la mise en cache des invites](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#how-prompt-caching-works) dans la référence API pour le mécanisme sous-jacent.

<img src="https://mintcdn.com/claude-code/VbDJw--l6T9a9Wvm/images/prompt-caching-prefix.svg?fit=max&auto=format&n=VbDJw--l6T9a9Wvm&q=85&s=f2e8f0b8298a50305fe428ca3f1d1594" className="dark:hidden" alt="Quatre tours affichés sous forme de barres horizontales croissantes. La requête de chaque tour contient tout ce qui provient du tour précédent plus l'échange le plus récent ajouté à la fin. Aux tours deux et trois, le préfixe inchangé est lu à partir du cache et seul le nouvel échange est traité. Au tour quatre, l'invite système a changé, donc le préfixe ne correspond plus et la requête entière est retraitée et écrite." width="720" height="454" data-path="images/prompt-caching-prefix.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/prompt-caching-prefix-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=297dc1c639f0915cae858d0c4b6f3be5" className="hidden dark:block" alt="Quatre tours affichés sous forme de barres horizontales croissantes. La requête de chaque tour contient tout ce qui provient du tour précédent plus l'échange le plus récent ajouté à la fin. Aux tours deux et trois, le préfixe inchangé est lu à partir du cache et seul le nouvel échange est traité. Au tour quatre, l'invite système a changé, donc le préfixe ne correspond plus et la requête entière est retraitée et écrite." width="720" height="454" data-path="images/prompt-caching-prefix-dark.svg" />

Pour tirer le meilleur parti de la correspondance des préfixes, Claude Code organise chaque requête de sorte que le contenu qui change rarement entre les tours vient en premier :

| Couche             | Contenu                                                      | Change quand                                         |
| ------------------ | ------------------------------------------------------------ | ---------------------------------------------------- |
| Invite système     | Instructions principales, définitions d'outils               | L'ensemble des définitions d'outils chargées change  |
| Contexte du projet | CLAUDE.md, mémoire automatique, règles non délimitées        | La session commence, ou après `/clear` ou `/compact` |
| Conversation       | Vos messages, les réponses de Claude, les résultats d'outils | À chaque tour                                        |

Une modification de la couche de conversation laisse l'invite système et le contexte du projet en cache. Une modification de l'invite système invalide tout, car tout le contenu ultérieur se trouve maintenant derrière un préfixe différent. La troisième colonne donne les déclencheurs courants plutôt qu'une liste exhaustive, et les sections ci-dessous couvrent l'ensemble complet.

La règle de correspondance des préfixes explique la plupart des comportements sur cette page. Le [mode Plan](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode) et le [chargement des compétences](/docs/fr/skills), par exemple, ajoutent leurs instructions sous forme de messages de conversation, de sorte que le préfixe en cache reste intact.

Deux paramètres n'apparaissent pas dans le tableau des couches mais affectent toujours ce qui reste en cache :

* **Modèle** : chaque modèle a son propre cache. Changer de modèle recalcule la requête entière même lorsque le contenu est identique. Consultez [Changer de modèle](#switching-models) ci-dessous.
* **Niveau d'effort** : sur la plupart des modèles, chaque niveau d'effort a son propre cache, donc changer d'effort en cours de session recalcule la requête entière. Sur Opus 5.5 et Fable 5.1 avec une clé API ou un abonnement Claude, le cache reste intact par défaut. Consultez [Changer le niveau d'effort](#changing-effort-level) ci-dessous.

<Tip>
  Choisissez votre modèle et votre niveau d'effort au début d'une session, puis réservez `/compact` pour les pauses naturelles entre les tâches. Moins vous apportez de modifications en cours de tâche, plus votre taux de succès du cache est élevé.
</Tip>

<h3 id="where-the-cache-lives">
  Où vit le cache
</h3>

La mise en cache se produit côté serveur, dans l'infrastructure qui sert votre modèle. L'endroit où cela se trouve dépend de la façon dont vous vous authentifiez :

* **Clé API, abonnement Claude, ou [Claude Platform on AWS](/docs/fr/claude-platform-on-aws)** : le cache se trouve dans l'infrastructure d'Anthropic, accessible via l'[API Claude](https://platform.claude.com/docs)
* **Amazon Bedrock ou Agent Platform de Google Cloud** : le cache se trouve dans l'infrastructure de service de votre fournisseur cloud
* **Microsoft Foundry** : dépend de l'[option d'hébergement](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options) du déploiement. Les déploiements hébergés sur Azure sont servis sur l'infrastructure Azure ; les déploiements hébergés sur Anthropic sont servis sur l'infrastructure d'Anthropic
* **`ANTHROPIC_BASE_URL` personnalisé ou [passerelle LLM](/docs/fr/llm-gateway)** : le cache se trouve là où vos requêtes sont transférées, et le fonctionnement de la mise en cache dépend de la passerelle

Claude Code ajoute également le contexte système en cours de conversation, comme les avis de modification de fichiers, et marque ce bloc pour la mise en cache sur chaque fournisseur et connexion sauf si vous définissez [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/fr/llm-gateway-protocol#disable-pre-release-capabilities), auquel cas ce bloc est envoyé sans cache.

Au point de terminaison propre du fournisseur, Amazon Bedrock et son [point de terminaison Mantle](/docs/fr/amazon-bedrock#use-the-mantle-endpoint), Agent Platform de Google Cloud, et Microsoft Foundry mettent en cache le bloc de la même manière que l'API Claude.

Lorsque vos requêtes passent par une [passerelle LLM](/docs/fr/llm-gateway), un `ANTHROPIC_BASE_URL` personnalisé, ou un remplacement d'URL de base du fournisseur cloud tel que [`ANTHROPIC_BEDROCK_BASE_URL`](/docs/fr/env-vars), ce qui reste en cache dépend de la façon dont la passerelle gère les [marqueurs `cache_control`](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#explicit-cache-breakpoints) que Claude Code envoie :

* **Les transmet inchangés** : le bloc et votre conversation se mettent en cache de la même manière qu'au point de terminaison propre du fournisseur.
* **Rejette la requête marquée avec une erreur `400` nommant `cache_control`** : Claude Code renvoie la requête avec le marqueur déplacé du bloc vers votre dernier message de conversation, et le garde là pour le reste de la conversation. Le bloc est facturé comme entrée non mise en cache ; votre conversation reste mise en cache.
* **Supprime les marqueurs tout en retournant le succès** : l'historique de votre conversation entière est facturé comme entrée non mise en cache à chaque tour. Une passerelle qui convertit le contenu du système sous forme de bloc en une chaîne simple supprime le marqueur de la même manière.

Pour ce que chaque fournisseur stocke et traite, consultez [utilisation des données](/docs/fr/data-usage). Où que le cache se trouve, les entrées expirent après une période d'inactivité, et [Durée de vie du cache](#cache-lifetime) ci-dessous couvre le TTL et comment l'étendre.

<h2 id="actions-that-invalidate-the-cache">
  Actions qui invalident le cache
</h2>

Ces actions font que la prochaine requête manque une partie ou la totalité du cache. Vous voyez un tour plus lent et plus coûteux une seule fois, après quoi le nouveau préfixe est mis en cache. La plupart d'entre elles sont évitables en cours de tâche une fois que vous savez qu'elles ont un coût. Un changement de modèle peut sembler gratuit jusqu'à ce que vous remarquiez le tour plus lent qui suit.

* [Changement de modèles](#switching-models)
* [Modification du niveau d'effort](#changing-effort-level)
* [Activation du mode rapide](#turning-on-fast-mode)
* [Connexion ou déconnexion d'un serveur MCP](#connecting-or-disconnecting-an-mcp-server)
* [Activation ou désactivation d'un plugin](#enabling-or-disabling-a-plugin)
* [Refus d'un outil entier](#denying-an-entire-tool)
* [Compactage de la conversation](#compacting-the-conversation)
* [Accumulation de nombreuses images](#accumulating-many-images)
* [Mise à niveau de Claude Code](#upgrading-claude-code)

<h3 id="switching-models">
  Changement de modèles
</h3>

Chaque modèle a son propre cache. Basculer avec [`/model`](/docs/fr/model-config#setting-your-model) signifie que la prochaine requête lit l'intégralité de l'historique de conversation sans aucun cache hit, même si le contenu est identique.

Lorsque vous exécutez `/model` au terminal, Claude Code vous demande de confirmer le changement uniquement tant que le cache est encore chaud et que le nouveau modèle n'est pas celui qui a produit la dernière réponse. Le cache reste chaud pendant un [cache TTL](#cache-lifetime) après que Claude Code a envoyé une dernière requête dans cette conversation ou que Claude a répondu. Une fois ce délai écoulé, le cache a expiré, donc Claude Code bascule sans demander.

Avant la v2.1.238, Claude Code ne vérifiait pas le cache TTL et demandait même après l'expiration du cache.

Vous pouvez également exiger cette confirmation ou l'ignorer avec un [hook PreModelSwitch](/docs/fr/hooks#premodelswitch-decision-control).

Le [paramètre de modèle `opusplan`](/docs/fr/model-config#opusplan-model-setting) se résout en Opus pendant le mode plan et en Sonnet pendant l'exécution, donc chaque basculement du mode plan est un changement de modèle et démarre un nouveau cache.

[Le basculement automatique du modèle](/docs/fr/model-config#automatic-model-fallback) sur les modèles Fable, Opus 5.5 et Opus 5 est également un changement de modèle. Lorsqu'un classificateur de sécurité signale une requête dans une catégorie qui a un modèle de secours, Claude Code réexécute la requête sur ce modèle et la session continue là.

Lorsque le frontmatter d'une skill ou d'une commande nomme un [`model`](/docs/fr/skills#frontmatter-reference) autre que le modèle actuel de la session, ce tour est également un changement de modèle : la prochaine requête lit l'intégralité de l'historique de conversation sans aucun cache hit. Le modèle de session reprend à votre prochaine invite. Une skill `context: fork` définit le [modèle du sous-agent forké](/docs/fr/skills#run-skills-in-a-subagent) à la place.

<h3 id="changing-effort-level">
  Modification du niveau d'effort
</h3>

Sur la plupart des modèles, modifier le [niveau d'effort](/docs/fr/model-config#adjust-effort-level) en cours de session signifie que la prochaine requête lit l'intégralité de l'historique de conversation sans aucun cache hit. Tant que le cache est encore chaud, Claude Code vous demande de confirmer le changement d'abord.

Sur Opus 5.5 et Fable 5.1 avec une clé API ou un abonnement Claude, modifier l'effort conserve le cache, et Claude Code applique le nouveau niveau sans demander. Cela ne s'applique pas sur Amazon Bedrock, sur la plateforme Agent de Google Cloud, ou sur une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway), ou lorsque vous définissez [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/fr/llm-gateway-protocol#disable-pre-release-capabilities) ou que votre organisation a une configuration HIPAA.

Avant la v2.1.260, modifier l'effort sur Fable 5.1 avec une clé API ou un abonnement Claude invalidait également le cache.

<h3 id="turning-on-fast-mode">
  Activation du mode rapide
</h3>

L'activation du [mode rapide](/docs/fr/fast-mode) ajoute un en-tête de requête qui fait partie de la clé de cache, donc la première requête que Claude Code envoie avec le mode rapide activé lit l'intégralité de l'historique de conversation sans aucun cache hit. Claude Code définit cet en-tête une fois au démarrage d'un tour et le conserve pour tout le tour, donc lorsque vous activez le mode rapide pendant que Claude travaille, le cache miss de l'en-tête se produit à la première requête de votre tour suivant. Ces jetons d'entrée non mis en cache sont facturés aux [tarifs du mode rapide](/docs/fr/fast-mode#understand-the-cost-tradeoff), c'est pourquoi l'activation au début d'une session coûte moins cher que l'activation profondément dans une longue session. Si votre modèle actuel ne supporte pas le mode rapide, l'activation du mode rapide [bascule également votre modèle](#switching-models), et ce basculement démarre un nouveau cache à partir de la prochaine requête du tour en cours.

Le coût s'applique une fois par conversation. Après le premier tour en mode rapide, Claude Code continue d'envoyer l'en-tête et varie uniquement le paramètre de vitesse de la requête, qui ne fait pas partie de la clé de cache. Désactiver le mode rapide, le [basculement automatique vers la vitesse standard](/docs/fr/fast-mode#handle-rate-limits) après une limite de débit, et le réactiver plus tard conservent tous le cache. Si vous [manquez de crédits d'utilisation](/docs/fr/fast-mode#handle-rate-limits) en cours de session, Claude Code réessaie chaque requête en mode rapide rejetée à la vitesse standard de la même manière, donc ce basculement conserve également le cache. `/clear` et `/compact` réinitialisent cela, puisqu'ils reconstruisent le cache à ces points de toute façon.

<h3 id="connecting-or-disconnecting-an-mcp-server">
  Connexion ou déconnexion d'un serveur MCP
</h3>

Les définitions d'outils se trouvent dans la couche d'invite système, donc le cache s'invalide lorsque l'ensemble des définitions d'outils dans la requête change entre les tours. Basculer l'[outil conseiller](/docs/fr/advisor) est une exception : sa définition se trouve après le point de rupture du cache, donc l'activation ou la désactivation de `/advisor` conserve le préfixe mis en cache intact. Qu'un changement de [serveur MCP](/docs/fr/mcp) fasse cela dépend de si ses outils sont différés par la [recherche d'outils](/docs/fr/mcp#scale-with-mcp-tool-search) ou chargés dans le préfixe :

* **Outils différés**, la valeur par défaut sur les modèles supportés : un serveur se connectant, se déconnectant, ou changeant sa liste d'outils n'ajoute que du nouveau contenu et ne perturbe rien de déjà mis en cache.
* **Outils chargés dans le préfixe** : tout changement les invalide. Cela se produit lorsque la [recherche d'outils n'est pas disponible ou est désactivée](/docs/fr/mcp#configure-tool-search), par exemple sur les modèles de la plateforme Agent de Google Cloud antérieurs à la génération Claude 4.5, avec une passerelle `ANTHROPIC_BASE_URL` personnalisée, ou sur un [déploiement Microsoft Foundry hébergé sur Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options) une fois que Claude Code détecte que le déploiement rejette la recherche d'outils. Cela se produit également pour un serveur ou un outil marqué [`alwaysLoad`](/docs/fr/mcp#exempt-a-server-from-deferral), et pour les définitions conservées en avant par le [chargement basé sur le seuil](/docs/fr/mcp#configure-tool-search).

Lorsque les outils se chargent dans le préfixe, la cause la plus courante d'une invalidation est un serveur se connectant ou se déconnectant en cours de session, ce qui peut se produire sans aucune action de votre part : le processus d'un serveur stdio se termine, une session HTTP expire, ou un serveur [se reconnecte automatiquement après une défaillance transitoire](/docs/fr/mcp#automatic-reconnection). Un serveur connecté peut également envoyer une [mise à jour d'outil dynamique](/docs/fr/mcp#dynamic-tool-updates) qui change sa liste d'outils.

Éditer votre configuration MCP ne change pas le cache en soi. La nouvelle configuration ne prend effet qu'après un redémarrage, c'est à ce moment que le serveur se connecte ou se déconnecte.

<h3 id="enabling-or-disabling-a-plugin">
  Activation ou désactivation d'un plugin
</h3>

Lorsque vous activez ou désactivez un [plugin](/docs/fr/plugins/overview), ce que le changement coûte dépend des types de composants que le plugin fournit. Les cas ci-dessous couvrent chaque type de composant, quand Claude Code applique le changement, et ce qui se passe lorsque vous désactivez un plugin à nouveau dans la même session.

<h4 id="plugin-components-that-keep-the-cache">
  Composants de plugin qui conservent le cache
</h4>

Claude Code n'invalide jamais le cache pour les skills, commandes, agents, hooks, moniteurs ou thèmes d'un plugin. Il ajoute leur contenu après la conversation existante, donc la prochaine requête paie pour ce contenu et lit toujours tout ce qui le précède à partir du cache.

<h4 id="plugins-that-provide-mcp-servers">
  Plugins qui fournissent des serveurs MCP
</h4>

Lorsque vous activez ou désactivez un plugin qui fournit des [serveurs MCP](/docs/fr/plugins/components#mcp-servers), Claude Code suit les mêmes règles que lorsque vous [connectez ou déconnectez un serveur MCP](#connecting-or-disconnecting-an-mcp-server) :

* Si Claude Code diffère les outils du serveur, il conserve le cache.
* Si Claude Code les charge dans le préfixe, la prochaine requête relit l'intégralité de la conversation.

<h4 id="code-intelligence-plugins">
  Plugins d'intelligence de code
</h4>

Lorsque vous activez un [plugin d'intelligence de code](/docs/fr/plugins/code-intelligence), Claude obtient l'[outil LSP](/docs/fr/tools-reference#lsp-tool-behavior).

<h4 id="when-plugin-changes-apply">
  Quand les changements de plugin s'appliquent
</h4>

Un changement que vous effectuez dans le menu `/plugin` passe par [`/reload-plugins`](/docs/fr/plugins/cli-reference#reload-plugins), que Claude Code exécute pour vous lorsque vous fermez le menu. Vous payez le coût, qu'il s'agisse d'annonces ajoutées ou d'une relecture complète, au premier tour après l'application du changement. Claude Code peut également appliquer un changement de son propre chef :

* Pour un plugin avec une source `command`, Claude Code [peut recharger le plugin lui-même](/docs/fr/plugins/loading#when-a-command-source-re-runs).
* Lorsque vous [installez un plugin à partir de l'interface `/plugin`](/docs/fr/plugins/install#install-a-plugin), Claude Code peut l'activer pendant l'installation. Le résumé d'installation vous indique s'il l'a fait.
* Lorsque vous [déplacez la session avec `/cd`](/docs/fr/permissions#move-the-session-to-another-directory) sur v2.1.246 ou ultérieur, Claude Code applique les plugins que les paramètres du nouveau répertoire activent dans le cadre du déplacement, sans l'avertissement de relecture complète qui retient un `/reload-plugins`.
* Dans les sessions interactives, lorsque vous ajoutez ou supprimez un plugin dans un [dossier de plugins](/docs/fr/plugins/create#load-a-directory-or-archive-for-one-session) que vous avez transmis avec `--plugin-dir`, le changement s'applique immédiatement. Si l'appliquer déclencherait une relecture complète, Claude Code retient le changement à la place et affiche un avis pour exécuter `/reload-plugins`. Nécessite Claude Code v2.1.265 ou ultérieur.

Lorsque `/reload-plugins` s'exécute et que la recharge déclencherait une relecture complète, Claude Code affiche un avertissement et n'applique pas la recharge. Exécutez `/reload-plugins --force` pour l'appliquer de toute façon.

`/reload-plugins` s'exécute également dans les sessions sans terminal interactif, comme l'application de bureau, le SDK Agent, et le [mode non interactif](/docs/fr/headless) avec `-p`, lorsque vous le tapez directement dans la session. Nécessite Claude Code v2.1.260 ou ultérieur.

Dans ces sessions, la recharge applique tout sauf les changements de serveur MCP du plugin, qui [prennent effet dans votre prochaine session](/docs/fr/plugins/cli-reference#reload-plugins) et ne coûtent donc jamais une relecture complète en cours de session.

<h4 id="plugins-you-enable-and-then-disable-in-one-session">
  Plugins que vous activez puis désactivez dans une session
</h4>

Lorsque vous désactivez un plugin que vous avez activé plus tôt dans la session, Claude Code restaure la forme de requête précédente. Si ce préfixe se trouve toujours dans sa [durée de vie du cache](#cache-lifetime), la prochaine requête lit l'entrée de cache plus ancienne au lieu de la reconstruire.

<h3 id="denying-an-entire-tool">
  Refus d'un outil entier
</h3>

Si vous ajoutez un nom d'outil nu comme `Bash` ou `WebFetch` comme [règle de refus](/docs/fr/permissions#manage-permissions), Claude ne peut pas appeler cet outil à partir de votre prochaine requête, que vous ajoutiez la règle via `/permissions` ou en [éditant directement un fichier de paramètres](/docs/fr/settings#when-edits-take-effect). Cela inclut une règle que vous ajoutez via `/permissions` au milieu d'un tour.

Lorsque la [recherche d'outils](/docs/fr/mcp#scale-with-mcp-tool-search) est active, ce qui est la valeur par défaut sur les modèles supportés, les définitions d'outils de la requête ne changent pas et le préfixe mis en cache survit. Lorsque la recherche d'outils n'est pas disponible ou est désactivée, Claude Code supprime la définition de la prochaine requête, ce qui invalide le cache, et il en va de même pour la suppression de la règle plus tard.

Seule une règle de refus qui correspond à la position du nom d'outil bloque un outil de cette manière : un nom d'outil nu, la forme équivalente `Bash(*)`, ou un [glob de nom d'outil](/docs/fr/permissions#tool-name-wildcards) comme `"*"`. Un glob qui correspond uniquement aux outils MCP, comme `"mcp__*"`, bloque ces outils de la même manière. Les règles de refus délimitées comme `Bash(rm *)`, et toutes les règles d'autorisation et de demande, ne changent pas les outils que Claude voit. Claude Code les vérifie lorsque Claude tente un appel, laissant le préfixe intact.

<h3 id="compacting-the-conversation">
  Compactage de la conversation
</h3>

Le [compactage](/docs/fr/context-window#what-survives-compaction) remplace votre historique de messages par un résumé. Par conception, cela invalide la couche de conversation, puisque la prochaine requête a un nouvel historique plus court qui ne partage pas de préfixe avec l'ancien. Claude Code réutilise la couche d'invite système sauf si la conversation a été [reprise tout en conservant une invite système qui aurait autrement changé](#resuming-a-session) ; dans ce cas, le premier compactage bascule vers l'invite actuelle et cette couche se reconstruit une fois. Il recharge le contexte du projet à partir du disque, qui ne cache les hits que si CLAUDE.md et la mémoire sont inchangés depuis le début de la session.

Pour produire le résumé, Claude Code envoie une requête séparée avec la même invite système, les mêmes outils et le même historique que votre conversation, plus une instruction de résumé ajoutée comme dernier message utilisateur. Tant que le cache est chaud, cette requête lit votre préfixe à partir du cache, donc un `/compact` en cours de session coûte une fraction de ce que la taille du contexte suggère et passe la plupart de son temps à générer le résumé.

Après une pause plus longue que la [durée de vie du cache](#cache-lifetime), il n'y a pas de cache à lire, donc la requête de résumé retraite l'historique complet en tant qu'entrée non mise en cache. C'est pourquoi `/compact` coûte le plus lorsque vous [reprenez une ancienne session](/docs/fr/sessions#resume-from-a-summary). Dans les deux cas, chaud et froid, le tour après compactage reconstruit le cache de conversation pour seulement le résumé beaucoup plus court, donc ce tour n'est pas la partie lente.

<Tip>
  Le compactage joue en votre faveur lorsque le contexte que vous rejetez est du contenu dont vous n'avez plus besoin. Pour choisir quand son surcoût se produit, exécutez `/compact` à une pause naturelle dans votre travail, par exemple entre les tâches, au lieu d'attendre que le compactage automatique se déclenche en cours de tâche. Si vous avez suivi un chemin que vous voulez abandonner entièrement, [`/rewind`](#rewinding-the-conversation) à un tour antérieur à la place. Le rembobinage tronque jusqu'à un préfixe qui est déjà mis en cache, plutôt que d'en construire un nouveau comme le fait le compactage.
</Tip>

<h3 id="accumulating-many-images">
  Accumulation de nombreuses images
</h3>

L'API limite le nombre d'images et de PDF que chaque requête peut contenir. Pour les chiffres actuels, voir [Limites de requête](https://platform.claude.com/docs/en/build-with-claude/vision#request-limits) dans la documentation de l'API. Claude Code limite également la taille totale des images et des PDF dans une requête, donc les grandes captures d'écran atteignent la limite avec moins d'images que les petites.

Lorsque la prochaine requête dépasserait l'une ou l'autre limite, Claude Code supprime un lot des images et des PDF les plus anciens de ce qu'il envoie, ce qui laisse de la place pour plus avant qu'il n'ait besoin d'en supprimer à nouveau. Claude ne peut plus voir les images supprimées. Si Claude en a besoin à nouveau, partagez-la à nouveau.

La suppression d'images change les messages qui les contenaient, donc la prochaine requête retraite la conversation à partir du plus ancien de ces messages. Parce que Claude Code supprime un lot à la fois, vous voyez un tour plus lent par lot plutôt qu'un avec chaque nouvelle capture d'écran.

<h3 id="upgrading-claude-code">
  Mise à niveau de Claude Code
</h3>

Une nouvelle version de Claude Code met généralement à jour l'invite système ou les définitions d'outils, donc la première conversation que vous démarrez après une mise à niveau construit son cache à partir du début. La [mise à jour automatique](/docs/fr/setup#auto-updates) télécharge les nouvelles versions en arrière-plan mais les applique au prochain lancement, jamais en cours de session, donc vous voyez cela comme un premier tour non mis en cache après redémarrage plutôt qu'une surprise pendant une session. Définissez `DISABLE_AUTOUPDATER=1` pour contrôler quand les mises à niveau s'appliquent.

<Note>
  Pour ce qu'il en coûte de reprendre une conversation que vous avez commencée avant la mise à niveau, voir [Reprise d'une session](#resuming-a-session).
</Note>

<h2 id="actions-that-keep-the-cache">
  Actions qui conservent le cache
</h2>

Ces actions ajoutent à la fin de la conversation ou ne touchent pas du tout à la requête. Certaines d'entre elles, comme l'édition de CLAUDE.md, conservent le cache pour la même raison que le changement n'atteint pas la session en cours jusqu'à `/clear`, `/compact` ou un redémarrage.

* [Édition de fichiers dans votre référentiel](#editing-files-in-your-repository)
* [Édition de CLAUDE.md en cours de session](#editing-claude-md-mid-session)
* [Modification du mode de permission](#changing-permission-mode)
* [Modification du style de sortie](#changing-output-style)
* [Invocation de skills et de commandes](#invoking-skills-and-commands)
* [Exécution de `/recap`](#running-%2Frecap)
* [Rembobinage de la conversation](#rewinding-the-conversation)
* [Génération d'un sous-agent](#subagents-and-the-cache)

<h3 id="editing-files-in-your-repository">
  Édition de fichiers dans votre référentiel
</h3>

Le contenu des fichiers entre en contexte uniquement lorsque Claude les lit, et les lectures s'ajoutent à la conversation. L'édition d'un fichier que Claude a précédemment lu ne change pas rétroactivement la lecture antérieure dans l'historique. Au lieu de cela, Claude Code ajoute un `<system-reminder>` notant que le fichier a changé, et Claude le relit si nécessaire.

<h3 id="editing-claude-md-mid-session">
  Édition de CLAUDE.md en cours de session
</h3>

Vos fichiers CLAUDE.md au niveau du projet-root et au niveau utilisateur sont lus une fois au démarrage de la session et conservés en mémoire. Les éditer en cours de session n'invalide pas le cache, mais l'édition ne s'applique pas non plus. Claude continue de travailler avec la version qui a été chargée au démarrage de la session. Le nouveau contenu se charge au prochain `/clear`, `/compact` ou redémarrage.

[Les fichiers CLAUDE.md imbriqués dans les sous-répertoires](/docs/fr/memory) et [les règles avec frontmatter `paths:`](/docs/fr/memory#path-specific-rules) se chargent plus tard, lorsque Claude lit pour la première fois un fichier correspondant. L'édition d'un avant qu'il ne se charge prend effet. Après son chargement, le contenu fait partie de l'historique de la conversation, donc une édition en cours de session ne le change pas rétroactivement.

<h3 id="changing-permission-mode">
  Modification du mode de permission
</h3>

Le passage entre [les modes de permission](/docs/fr/permission-modes), par exemple de Manuel à accepter les éditions, ne change pas l'invite système ou les définitions d'outils, donc les changements de mode sont sûrs pour le cache. L'exception est le mode plan avec le paramètre de modèle [`opusplan`](/docs/fr/model-config#opusplan-model-setting), qui bascule le modèle entre Opus et Sonnet lorsque vous entrez ou quittez le mode plan. Cela rend le basculement de mode un [changement de modèle](#switching-models).

<h3 id="changing-output-style">
  Modification du style de sortie
</h3>

Lorsque vous changez [les styles de sortie](/docs/fr/output-styles) en cours de session avec [`/output-style`](/docs/fr/output-styles#change-your-output-style), `/config`, ou le paramètre `outputStyle`, Claude utilise le nouveau style à partir de votre prochain message. Claude Code fournit les instructions du nouveau style sous forme de message dans la conversation, donc cette requête lit toujours l'invite système et la conversation antérieure à partir du cache.

Avant la v2.1.251, un changement de style en cours de session conservait le cache mais ne s'appliquait pas jusqu'à ce que vous exécutiez `/clear` ou démarriez une nouvelle session.

<h3 id="invoking-skills-and-commands">
  Invocation de skills et de commandes
</h3>

[Les skills](/docs/fr/skills) et [les commandes](/docs/fr/commands) injectent leurs instructions sous forme de messages utilisateur au point d'invocation. Rien d'antérieur dans la conversation ne change. Un skill ou une commande dont le frontmatter nomme un `model` peut être un [changement de modèle](#switching-models) pour ce tour.

<h3 id="running-/recap">
  Exécution de `/recap`
</h3>

[`/recap`](/docs/fr/interactive-mode#session-recap) génère un résumé pour l'affichage dans votre terminal. Contrairement à `/compact`, il ajoute le résumé en tant que sortie de commande plutôt que de remplacer votre historique de messages, donc le préfixe mis en cache reste intact.

<h3 id="rewinding-the-conversation">
  Rembobinage de la conversation
</h3>

[`/rewind`](/docs/fr/checkpointing) tronque votre conversation jusqu'à un tour antérieur. L'historique restant est le même contenu à partir duquel le cache a été construit à ce moment-là, et l'invite système et les couches de contexte du projet sont inchangées, donc la requête suivante atteint l'entrée de cache antérieure. Chaque tour depuis lors a lu ce préfixe, ce qui a maintenu l'entrée active même si le tour original était plus ancien que le TTL.

La restauration des points de contrôle de fichiers aux côtés de la conversation n'a aucun effet séparé sur le cache. Le contenu des fichiers entre en contexte uniquement lorsque Claude les lit, de la même manière que [l'édition de fichiers dans votre référentiel](#editing-files-in-your-repository).

<h2 id="resuming-a-session">
  Reprendre une session
</h2>

Lorsque vous [reprenez une session](/docs/fr/sessions#resume-a-session), Claude Code renvoie l'intégralité de la conversation, et la demande lit à partir du cache la partie de son préfixe qui n'a pas changé et qui se situe toujours dans la [durée de vie du cache](#cache-lifetime). Le tableau des couches en haut de cette page indique ce qui change à chaque couche.

L'invite système changerait après une [mise à niveau de Claude Code](#upgrading-claude-code) ou avec un texte [`--append-system-prompt`](/docs/fr/cli-reference#system-prompt-flags) différent lors de la reprise. Par défaut, la conversation reprise conserve l'invite système avec laquelle elle a commencé, de sorte que son historique se situe toujours derrière la même invite, et la modification prend effet une fois que la conversation est compactée ou dans une nouvelle conversation. [Les indicateurs d'invite système dans les conversations reprises](/docs/fr/cli-reference#system-prompt-flags-in-resumed-conversations) couvre les cas où Claude Code reconstruit l'invite à chaque demande à la place.

<h2 id="cache-lifetime">
  Durée de vie du cache
</h2>

Les préfixes en cache expirent après une période d'inactivité. Chaque requête qui atteint le cache réinitialise le minuteur, de sorte que le cache reste actif tant que vous continuez à travailler. Après un écart assez long, la requête suivante recalcule l'entrée complète et rétablit le cache, ce qui est pourquoi le premier tour après s'être éloigné peut être notablement plus lent.

Sur un plan Pro ou Max, lorsque vous reprenez une grande session après une longue pause, Claude Code [propose de reprendre à partir d'un résumé](/docs/fr/sessions#resume-from-a-summary) afin que les requêtes ultérieures ne portent pas l'historique complet.

Le time to live (TTL) contrôle la durée de l'écart que le cache survit. L'API en offre deux : un TTL de cinq minutes, et un [TTL d'une heure](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#1-hour-cache-duration) qui garde le cache actif pendant les pauses plus longues mais [facture les écritures de cache à un taux plus élevé](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing). Le TTL plus long aide quand vous laissez une session inactive et y revenez, car vous évitez le retraitement qu'un préfixe expiré coûte. Cela coûte plus cher lors de courtes rafales de travail qui ne restent jamais inactives au-delà de cinq minutes, où le taux d'écriture plus élevé s'applique et la durée de vie du cache plus longue reste inutilisée.

<h3 id="which-ttl-each-request-gets">
  Quel TTL chaque requête obtient
</h3>

Claude Code décide du TTL par requête, et chaque requête se situe dans l'un de deux compartiments fixes :

* **Conversation principale** : vos tours interactifs, les exécutions non interactives `-p`, et les tours Agent SDK, plus les assistants que Claude Code exécute en ligne avec eux
* **Tout le reste** : les requêtes que Claude Code effectue en dehors de cette conversation, telles que les [sous-agents](/docs/fr/sub-agents), les [workflows](/docs/fr/workflows), les [coéquipiers](/docs/fr/agent-teams) en processus, les forks, la compaction, et les titres de session

À moins que vous ne choisissiez vous-même un TTL, Claude Code demande le TTL d'une heure uniquement sur un abonnement Claude dans l'utilisation incluse de votre plan. Là, il demande l'heure pour la conversation principale, plus un petit ensemble de requêtes d'assistance que Anthropic contrôle côté serveur. Ce tableau donne le TTL par défaut de chaque compartiment selon les deux types de facturation.

| Compartiment de requête | Abonnement Claude, dans l'utilisation du plan                                                    | Crédits d'utilisation, clé API, ou fournisseur cloud |
| ----------------------- | ------------------------------------------------------------------------------------------------ | ---------------------------------------------------- |
| Conversation principale | Une heure                                                                                        | Cinq minutes                                         |
| Tout le reste           | Cinq minutes, sauf les requêtes d'assistance contrôlées par le serveur, qui obtiennent une heure | Cinq minutes                                         |

Une fois que vous dépassez la limite d'utilisation de votre plan et que Claude Code puise dans les [crédits d'utilisation](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans), vous êtes facturé pour cette utilisation, donc Claude Code baisse la conversation principale au TTL de cinq minutes moins cher. Pour conserver le TTL d'une heure là, [choisissez le TTL vous-même](#choose-the-ttl-yourself).

<h3 id="choose-the-ttl-yourself">
  Choisissez le TTL vous-même
</h3>

Vous pouvez définir un TTL pour l'un ou l'autre compartiment. Chaque contrôle prend `5m` ou `1h`, et Claude Code ignore toute autre valeur.

* **Conversation principale** : le paramètre [`promptCacheTtl`](/docs/fr/settings-reference#promptcachettl), ou la [variable d'environnement](/docs/fr/env-vars) `CLAUDE_CODE_PROMPT_CACHE_TTL`
* **Tout le reste** : le paramètre [`subagentPromptCacheTtl`](/docs/fr/settings-reference#subagentpromptcachettl), ou la variable d'environnement `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`

Les deux paramètres et les deux variables d'environnement nécessitent Claude Code v2.1.242 ou ultérieur. Si vous vous connectez avec une clé API ou utilisez un fournisseur cloud, définissez `promptCacheTtl` sur `1h` pour donner à la conversation principale un cache d'une heure. Les requêtes en dehors de celle-ci conservent la valeur par défaut de cinq minutes jusqu'à ce que vous choisissiez également un TTL pour ce compartiment.

Lorsque plusieurs contrôles s'appliquent, Claude Code prend la première correspondance dans cet ordre :

1. `FORCE_PROMPT_CACHING_5M=1`, qui force cinq minutes pour les deux compartiments
2. La variable d'environnement du compartiment
3. Le paramètre du compartiment
4. Pour les requêtes d'un sous-agent, la valeur `cacheTtl` dans le champ frontmatter [`experimental`](/docs/fr/sub-agents#supported-frontmatter-fields) du sous-agent, qui nécessite Claude Code v2.1.248 ou ultérieur. Claude Code ignore un `1h` là tandis que votre abonnement Claude utilise des crédits d'utilisation
5. `ENABLE_PROMPT_CACHING_1H=1`, qui demande une heure pour les deux compartiments
6. La [valeur par défaut pour le compartiment de la requête](#which-ttl-each-request-gets)

Définissez `FORCE_PROMPT_CACHING_5M=1` quand vous déboguez le comportement du cache, comparez les deux TTL, ou remplacez un TTL plus long défini dans les [paramètres gérés](/docs/fr/managed-settings).

Pour confirmer quel TTL les écritures de cache de votre conversation principale ont utilisé, exécutez `claude -p "hello" --output-format json` et lisez `usage.cache_creation` dans le résultat. Claude Code rapporte les écritures de cache d'une heure sous `ephemeral_1h_input_tokens` et les écritures de cache de cinq minutes sous `ephemeral_5m_input_tokens`.

Via une passerelle LLM que vous définissez avec `ANTHROPIC_BASE_URL`, une partie de la requête d'une heure voyage dans l'en-tête `anthropic-beta`, donc configurez la passerelle pour [transférer cet en-tête inchangé](/docs/fr/llm-gateway-protocol#request-headers). Le TTL d'une heure n'est pas disponible via la [passerelle des applications Claude](/docs/fr/claude-apps-gateway#availability-and-limitations). Sur Amazon Bedrock, le support du prompt caching, la longueur minimale du préfixe cacheable, et la disponibilité du TTL d'une heure varient tous selon le modèle. Si les comptages de jetons de cache restent à zéro, vérifiez les [modèles, régions et limites pris en charge](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models) dans la documentation Amazon Bedrock.

<h2 id="cache-scope">
  Portée du cache
</h2>

Dans Claude Code, le cache est effectivement limité à une machine et un répertoire. Chaque conversation porte le répertoire de travail, la plateforme, le shell, et la version du système d'exploitation, et le prompt système nomme vos chemins de mémoire automatique, donc deux sessions dans des répertoires différents construisent des préfixes différents et manquent le cache de l'autre. Cela inclut les worktrees du même référentiel, puisque chaque worktree a son propre répertoire de travail.

Les sessions que vous exécutez en parallèle dans le même répertoire construisent des préfixes correspondants et lisent le cache de l'autre. Les sessions séquentielles partagent le préfixe uniquement quand l'instantané du statut git au démarrage correspond, puisque chaque conversation porte également la branche et les commits récents de cet instantané.

Le cache API sous-jacent est plus large. Les caches sont isolés entre les organisations, et sur certains fournisseurs, [entre les espaces de travail au sein d'une organisation](https://platform.claude.com/docs/fr/build-with-claude/prompt-caching#cache-storage-and-sharing). Dans ces limites, deux requêtes quelconques avec le même modèle et préfixe lisent le même cache. Pour les appelants du SDK Agent exécutant des flottes de processus automatisés, voir [améliorer le prompt caching entre les utilisateurs et les machines](/docs/fr/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) pour supprimer les sections par machine du prompt système et partager le cache entre les machines.

<h2 id="check-cache-performance">
  Vérifier les performances du cache
</h2>

Les performances du cache s'affichent comme deux comptages de jetons que l'API rapporte sur chaque réponse. Le moyen le plus direct de les regarder en direct est un [script de ligne d'état](/docs/fr/statusline) qui lit l'objet `current_usage` :

| Champ                         | Signification                                                                                                                                                                            |
| ----------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cache_creation_input_tokens` | Jetons écrits dans le cache à ce tour, facturés au taux d'écriture du cache                                                                                                              |
| `cache_read_input_tokens`     | Jetons servis à partir du cache à ce tour, facturés au [taux de jeton en cache](https://platform.claude.com/docs/en/about-claude/pricing) du modèle, inférieur au taux d'entrée standard |

Un ratio lecture-création élevé signifie que le caching fonctionne bien. Si la création reste élevée tour après tour, quelque chose change dans votre préfixe. La section [Actions qui invalident le cache](#actions-that-invalidate-the-cache) énumère les causes habituelles.

Pour un résumé par session, exécutez `/usage`. Après la première réponse de la conversation principale, Claude Code ajoute une [ligne `Prompt cache (main)`](/docs/fr/costs#prompt-cache-statistics) au bloc Session, affichant le ratio de succès de la session, le nombre d'échecs et si le cache est chaud en ce moment. Un script de ligne d'état peut lire les mêmes chiffres à partir de l'objet [`prompt_cache`](/docs/fr/statusline#prompt-cache-fields). Les deux nécessitent Claude Code v2.1.251 ou ultérieur.

La ligne `Prompt cache (main)` nomme également la cause probable du dernier échec lorsque Claude Code peut en identifier une, par exemple `likely cause: tool definitions changed`. Le texte de cause probable nécessite Claude Code v2.1.260 ou ultérieur.

Pour la visibilité dans une organisation, l'exportateur OpenTelemetry rapporte les jetons de lecture et de création du cache par utilisateur et session. Voir [Surveiller l'utilisation](/docs/fr/monitoring-usage) pour la référence des attributs de métrique et d'événement.

<h2 id="subagents-and-the-cache">
  Sous-agents et le cache
</h2>

Un [sous-agent](/docs/fr/sub-agents) démarre sa propre conversation avec son propre prompt système et ensemble d'outils, séparé du parent. Sa première requête ne lit pas le cache du parent, car les deux préfixes diffèrent, et il réchauffe son propre cache à travers ses tours. Les sous-agents se situent en dehors du [bucket TTL](#which-ttl-each-request-gets) de la conversation principale, donc ils obtiennent cinq minutes même sur un abonnement jusqu'à ce que vous [choisissiez un plus long](#choose-the-ttl-yourself).

Le cache du parent n'est pas affecté. Du côté du parent, l'appel du sous-agent et le résultat s'ajoutent à la conversation, laissant le préfixe du parent intact.

Un [fork](/docs/fr/sub-agents#fork-the-current-conversation), en contraste, hérite du prompt système du parent, des outils, et de l'historique de conversation exactement, donc sa première requête lit le cache du parent.

D'autres requêtes peuvent également lire un préfixe qu'une requête antérieure a mis en cache :

* **Copies de session** : une session que vous [copiez avec `/fork`](/docs/fr/agent-view#copy-the-session-with-%2Ffork) reçoit son instruction d'isolation en tant que message à la fin de la conversation copiée, donc le cache que la conversation originale a construit reste intact.
* **Compaction** : l'appel de résumé décrit dans [Compacter la conversation](#compacting-the-conversation) utilise la même approche de partage de préfixe.
* **Sous-agents repris** : quand Claude [reprend un sous-agent](/docs/fr/sub-agents#resume-subagents), la première requête de l'exécution reprise peut lire le cache que l'exécution originale a réchaufé.
* **Fan-outs de workflow** : dans un [fan-out de workflow](/docs/fr/workflows#prompt-caching-in-a-fan-out) d'agents de même préfixe, Claude Code retient tous sauf le premier pendant jusqu'à 5 secondes par défaut, donc leurs premières requêtes peuvent lire le préfixe que le premier agent a mis en cache.

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

Pour définir la politique de caching dans une organisation, mettez l'une de ces variables ou les [variables TTL](#cache-lifetime) dans le bloc `env` des [paramètres gérés](/docs/fr/managed-settings). Pour un usage normal, laissez le caching activé.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Leçons de la construction de Claude Code : Le prompt caching est tout](https://claude.com/blog/lessons-from-building-claude-code-prompt-caching-is-everything) : la justification de la conception pour le mode plan, le chargement d'outils différé, et la compaction
* [Explorer la fenêtre de contexte](/docs/fr/context-window) : ce qui se charge en contexte et quand
* [Réduire l'utilisation des jetons](/docs/fr/costs#reduce-token-usage) : stratégies au-delà du caching pour gérer la taille du contexte
* [Suivre et réduire les coûts](/docs/fr/agent-sdk/cost-tracking) : suivi des jetons de cache et configuration du TTL pour les appelants du SDK Agent
* [Prompt caching](https://platform.claude.com/docs/fr/build-with-claude/prompt-caching) : le mécanisme API sous-jacent, les points d'arrêt, et la tarification
