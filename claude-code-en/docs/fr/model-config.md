> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configuration du modèle

> Découvrez la configuration du modèle Claude Code, y compris les alias de modèle comme `opusplan`

<h2 id="available-models">
  Modèles disponibles
</h2>

Pour le paramètre `model` dans Claude Code, vous pouvez configurer l'un des éléments suivants :

* Un **alias de modèle**
* Un **nom de modèle**
  * API Anthropic : un **[nom de modèle](https://platform.claude.com/docs/fr/about-claude/models/overview)** complet
  * Amazon Bedrock : un ARN de profil d'inférence
  * Microsoft Foundry : un nom de déploiement
  * Plateforme Agent de Google Cloud : un nom de version

Pour obtenir des conseils sur le modèle et le niveau d'effort qui conviennent à différents types de travail, consultez [Choisir un modèle Claude et un niveau d'effort dans Claude Code](https://claude.com/blog/claude-model-and-effort-level-in-claude-code) sur le blog.

<Note>
  `ANTHROPIC_BASE_URL` change l'endroit où les demandes sont envoyées, et non le modèle qui y répond. Pour acheminer Claude via une passerelle LLM, consultez la [passerelle LLM](/docs/fr/llm-gateway).
</Note>

<h3 id="model-aliases">
  Alias de modèle
</h3>

Les alias de modèle offrent un moyen pratique de sélectionner les paramètres du modèle sans avoir à mémoriser les numéros de version exacts :

| Alias de modèle  | Comportement                                                                                                                                                                                                                                                                                                                                                                    |
| ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`default`**    | Valeur spéciale qui efface tout remplacement de modèle et revient au modèle recommandé pour votre type de compte, ou au [modèle par défaut de l'organisation](#organization-default-model) lorsqu'un administrateur en a défini un. N'est pas en soi un alias de modèle                                                                                                         |
| **`best`**       | Utilise Fable 5 où votre organisation y a accès, sinon le dernier modèle Opus                                                                                                                                                                                                                                                                                                   |
| **`fable`**      | Utilise Claude Fable 5 pour vos tâches les plus difficiles et les plus longues                                                                                                                                                                                                                                                                                                  |
| **`sonnet`**     | Utilise le dernier modèle Sonnet pour les tâches de codage quotidiennes                                                                                                                                                                                                                                                                                                         |
| **`opus`**       | Utilise le dernier modèle Opus pour les tâches de raisonnement complexe                                                                                                                                                                                                                                                                                                         |
| **`haiku`**      | Utilise le modèle Haiku rapide et efficace pour les tâches simples                                                                                                                                                                                                                                                                                                              |
| **`sonnet[1m]`** | Utilise Sonnet avec une [fenêtre de contexte de 1 million de tokens](https://platform.claude.com/docs/fr/build-with-claude/context-windows#context-window-sizes-by-model) pour les sessions longues. Sans effet lorsque `sonnet` se résout déjà en Sonnet 5 avec sa fenêtre 1M native ; derrière une [passerelle LLM](/docs/fr/llm-gateway), sélectionne la fenêtre 1M pour Sonnet 5 |
| **`opus[1m]`**   | Utilise Opus avec une [fenêtre de contexte de 1 million de tokens](https://platform.claude.com/docs/fr/build-with-claude/context-windows#context-window-sizes-by-model) pour les sessions longues                                                                                                                                                                               |
| **`opusplan`**   | Mode spécial qui utilise `opus` pendant le mode plan, puis bascule vers `sonnet` pour l'exécution                                                                                                                                                                                                                                                                               |

La version à laquelle les alias `opus` et `sonnet` se résolvent dépend du fournisseur :

| Fournisseur                                          | `opus`   | `sonnet`   |
| :--------------------------------------------------- | :------- | :--------- |
| API Anthropic                                        | Opus 4.8 | Sonnet 5   |
| [Claude Platform on AWS](/docs/fr/claude-platform-on-aws) | Opus 4.8 | Sonnet 4.6 |
| Amazon Bedrock, Plateforme Agent de Google Cloud     | Opus 4.8 | Sonnet 4.5 |
| Microsoft Foundry                                    | Opus 4.6 | Sonnet 4.5 |

Lorsqu'un alias se résout en un modèle plus ancien, les modèles plus récents sont disponibles en sélectionnant explicitement le nom de modèle complet ou en définissant `ANTHROPIC_DEFAULT_OPUS_MODEL` ou `ANTHROPIC_DEFAULT_SONNET_MODEL`.

Avant la v2.1.207, `opus` se résolvait en Opus 4.7 sur Claude Platform on AWS et en Opus 4.6 sur Amazon Bedrock et Plateforme Agent de Google Cloud.

Les alias pointent vers la version recommandée pour votre fournisseur et se mettent à jour au fil du temps. Pour épingler une version spécifique, utilisez le nom de modèle complet, par exemple `claude-opus-4-8`, ou définissez la variable d'environnement correspondante comme `ANTHROPIC_DEFAULT_OPUS_MODEL`.

<Note>
  Sonnet 5 nécessite Claude Code v2.1.197 ou version ultérieure. Opus 4.8 nécessite la v2.1.154 ou version ultérieure. Exécutez `claude update` pour mettre à niveau.
</Note>

<h3 id="work-with-fable-5">
  Travailler avec Fable 5
</h3>

[Claude Fable 5](https://platform.claude.com/docs/fr/about-claude/models/introducing-claude-fable-5-and-claude-mythos-5) est le modèle le plus capable dans Claude Code, adapté aux tâches plus grandes qu'une seule séance. Il soutient les sessions autonomes longues, enquête avant d'agir et vérifie son travail plus souvent que les modèles plus petits.

Fable 5 n'est pas le modèle par défaut. Sélectionnez-le avec `/model fable`. Les demandes que ses classificateurs de sécurité signalent, le plus souvent dans les domaines de la cybersécurité et de la biologie, déclenchent un [basculement automatique du modèle](#automatic-model-fallback).

Pour tirer le meilleur parti de Fable 5 :

* **Décrivez le résultat, pas les étapes** : donnez-lui le résultat que vous voulez et laissez-le planifier le chemin. Pour le maintenir en fonctionnement jusqu'à ce que ce résultat soit atteint, [définissez un objectif](/docs/fr/goal).
* **Donnez-lui des problèmes ambigus** : les enquêtes sur les causes profondes, le débogage des pannes et les décisions architecturales sont les endroits où l'enquête et la vérification supplémentaires sont payantes.
* **Ignorez les rappels de vérification** : il vérifie son propre travail avec moins d'invites, donc les rappels de tester ou de vérifier sont généralement inutiles.
* **Dimensionnez les tâches plus grandes** : donnez-lui du travail que vous diviseriez normalement en morceaux. Il maintient les sessions longues sans perdre le fil.

<Note>
  Fable 5 nécessite Claude Code v2.1.170 ou version ultérieure. Les versions antérieures n'affichent pas Fable 5 dans le sélecteur de modèle et ne peuvent pas le sélectionner. Exécutez `claude update` pour mettre à niveau. Fable 5 n'est pas disponible sous [rétention de données zéro](/docs/fr/zero-data-retention), où le sélecteur `/model` l'omet ou l'affiche désactivé.
</Note>

<h3 id="setting-your-model">
  Définir votre modèle
</h3>

Vous pouvez configurer votre modèle de plusieurs façons, énumérées par ordre de priorité :

1. **Pendant la session** : utilisez `/model <alias|name>` pour basculer immédiatement, ou exécutez `/model` sans argument pour ouvrir le sélecteur. Le sélecteur demande une confirmation lorsque la conversation a une sortie antérieure, car la réponse suivante relit l'historique complet sans contexte en cache
2. **Au démarrage** : lancez avec `claude --model <alias|name>`
3. **Variable d'environnement** : définissez `ANTHROPIC_MODEL=<alias|name>`
4. **Paramètres** : configurez de manière permanente dans votre fichier de paramètres en utilisant le champ `model`

À partir de la v2.1.153, `/model` enregistre votre choix comme valeur par défaut pour les nouvelles sessions en écrivant le champ `model` dans vos paramètres utilisateur. Dans le sélecteur :

* `Enter` : basculer le modèle et enregistrer comme valeur par défaut
* `s` : basculer le modèle pour cette session uniquement

Taper `/model <name>` directement se comporte comme `Enter`. Un modèle défini avec `/model` en [mode non interactif](/docs/fr/headless), avec l'indicateur `-p`, s'applique uniquement à la session actuelle et n'est pas enregistré comme valeur par défaut. Les paramètres du projet et gérés ont toujours la priorité et se réappliquent au prochain lancement. Un [modèle par défaut de l'organisation](#organization-default-model) que votre administrateur a configuré pour remplacer la sélection de l'utilisateur se réapplique également au prochain lancement.

Dans les versions v2.1.144 à v2.1.152, `/model` s'appliquait uniquement à la session actuelle et `d` dans le sélecteur enregistrait une valeur par défaut.

L'indicateur `--model` et la variable d'environnement `ANTHROPIC_MODEL` s'appliquent uniquement à la session que vous lancez avec eux. Pour exécuter différents modèles dans différents terminaux en même temps, lancez chacun avec son propre indicateur `--model` plutôt que de basculer avec `/model`.

Les prix dans le sélecteur `/model` apparaissent lorsque Claude Code communique avec l'API Anthropic, directement ou via une [passerelle LLM](/docs/fr/llm-gateway) qui la proxifie, et le prix sur une ligne est le prix du modèle que cette ligne sélectionne. Sur les [fournisseurs tiers](/docs/fr/third-party-integrations) tels qu'Amazon Bedrock et sur la [passerelle des applications Claude](/docs/fr/claude-apps-gateway), votre fournisseur ou passerelle détermine ce que vous payez, donc les lignes du sélecteur n'affichent aucun prix. Le prix est une étiquette d'affichage uniquement ; il n'affecte pas le modèle qu'une ligne sélectionne ou ce que votre fournisseur facture. Avant la v2.1.206, [Claude Platform on AWS](/docs/fr/claude-platform-on-aws) et les sessions de passerelle affichaient les prix de liste d'Anthropic, et une ligne pouvait afficher le prix d'un modèle différent de celui qu'elle sélectionnait.

Les sessions reprises démarrées avec `claude --resume`, `--continue`, ou le sélecteur `/resume` conservent le modèle qu'elles utilisaient lorsque la transcription a été enregistrée, indépendamment du paramètre `model` actuel. Si ce modèle a été retiré ou est exclu par [`availableModels`](#restrict-model-selection), la session revient à l'ordre de priorité normal. Cela empêche le choix `/model` d'une autre session de modifier le modèle à la reprise.

Un modèle que vous choisissez pour le nouveau lancement avec `--model` ou `ANTHROPIC_MODEL` a toujours la priorité sur le modèle restauré. À partir de la v2.1.195, il en va de même pour une variable de famille [`ANTHROPIC_DEFAULT_OPUS_MODEL`](#environment-variables).

Lorsque le modèle actif au démarrage provient des paramètres du projet ou gérés plutôt que de votre propre sélection, l'en-tête de démarrage indique quel fichier de paramètres l'a défini. Exécutez `/model` pour remplacer ; le paramètre du projet ou géré se réapplique au prochain lancement.

Lorsqu'un changement de modèle est demandé via la méthode `setModel()` du [SDK Agent](/docs/fr/agent-sdk/overview) ou par une application telle que l'[application de bureau](/docs/fr/desktop) qui exécute le CLI Claude Code pour vous, Claude Code vérifie que la chaîne est celle qu'il reconnaît avant de l'enregistrer. Cette vérification nécessite Claude Code v2.1.200 ou version ultérieure. Sur l'API Anthropic, Claude Code reconnaît :

* un alias de modèle
* une entrée du sélecteur `/model`
* tout nom qui commence par `claude-`
* une valeur que vous avez configurée vous-même comme [option de modèle personnalisé](#add-a-custom-model-option) ou dans [`modelOverrides`](#override-model-ids-per-version)

Claude Code rejette une chaîne non reconnue avec `Model "<name>" is not a recognized model id.` et la session conserve son modèle actuel, au lieu d'enregistrer la chaîne et d'échouer à la prochaine demande. Consultez la [référence d'erreur](/docs/fr/errors#model-is-not-a-recognized-model-id) pour les étapes de récupération.

La vérification s'exécute uniquement sur l'API Anthropic. Sur Amazon Bedrock, Plateforme Agent de Google Cloud, Microsoft Foundry, [Claude Platform on AWS](/docs/fr/claude-platform-on-aws), et derrière une [passerelle LLM](/docs/fr/llm-gateway) ou une `ANTHROPIC_BASE_URL` personnalisée, votre fournisseur ou passerelle définit les noms de modèle, donc Claude Code transmet toute chaîne sans la vérifier. La vérification ne couvre pas non plus l'indicateur `--model`, la variable d'environnement `ANTHROPIC_MODEL`, ou le paramètre `model` ; une valeur mal orthographiée là produit [There's an issue with the selected model](/docs/fr/errors#theres-an-issue-with-the-selected-model) à la première demande à la place.

Lorsque le modèle demandé a une date de retrait programmée ou est automatiquement remappé à une version plus récente, Claude Code affiche un avertissement qui nomme le modèle demandé. Les sessions interactives l'affichent comme un avis de démarrage. À partir de la v2.1.182, le même avertissement est écrit dans stderr en [mode non interactif](/docs/fr/headless) lors de l'utilisation du format de sortie texte par défaut. La vérification couvre également un `model` défini dans [frontmatter de sous-agent](/docs/fr/sub-agents). L'avertissement stderr est supprimé pour `--output-format json` et `stream-json` ; lisez le modèle réel à partir du champ `modelUsage` du [message de résultat](/docs/fr/headless#get-structured-output) à la place.

Exemple d'utilisation :

```bash theme={null}
# Démarrer avec Opus
claude --model opus

# Basculer vers Sonnet pendant la session
/model sonnet
```

Exemple de fichier de paramètres :

```json theme={null}
{
    "permissions": {
        ...
    },
    "model": "opus"
}
```

<h2 id="restrict-model-selection">
  Restreindre la sélection du modèle
</h2>

Les administrateurs d'entreprise peuvent utiliser `availableModels` dans les [paramètres gérés ou de politique](/docs/fr/settings#settings-files) pour restreindre les modèles que les utilisateurs peuvent sélectionner. Les entrées correspondent à une famille de modèles telle que `sonnet`, un préfixe de version tel que `claude-sonnet-4-5`, ou un ID de modèle complet tel que `claude-sonnet-4-5-20250929`.

Lorsque `availableModels` est défini, la liste d'autorisation s'applique partout où un utilisateur peut spécifier un modèle :

* **Modèle de session principale** : `/model`, le drapeau `--model`, la variable d'environnement `ANTHROPIC_MODEL`, le paramètre `model`, et le modèle restauré lors de la [reprise d'une session](#setting-your-model)
* **Résolution d'alias** : les variables d'environnement `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, `ANTHROPIC_DEFAULT_HAIKU_MODEL`, et `ANTHROPIC_DEFAULT_FABLE_MODEL` ne peuvent pas rediriger un alias autorisé vers un modèle en dehors de la liste
* **Mode rapide** : `/fast` refuse de basculer lorsque cela changerait implicitement vers un modèle Opus en dehors de la liste, avec le message « is not in your organization's allowed models »
* **Modèles de sous-agent** : le champ `model` dans le frontmatter du [sous-agent](/docs/fr/sub-agents#choose-a-model), le paramètre `model` de l'outil Agent, `CLAUDE_CODE_SUBAGENT_MODEL`, et, sur la v2.1.197 et antérieures, le sélecteur de modèle dans l'assistant `/agents`&#x20;
* **Modèles de compétence et de commande** : le frontmatter `model` dans les [compétences et commandes](/docs/fr/skills)
* **Modèle de conseiller** : le paramètre [`advisorModel`](/docs/fr/advisor) configuré et le drapeau `--advisor`
* **Modèle d'agent d'arrière-plan** : le modèle sélectionné dans le [sélecteur de dispatch](/docs/fr/agent-view)

Sur l'API Anthropic et [Claude Platform sur AWS](/docs/fr/claude-platform-on-aws), un alias de famille de modèles, `opus`, `sonnet`, `haiku`, ou `fable`, se résout à la version la plus récente de sa famille que la liste d'autorisation permet. Lorsque la liste d'autorisation épingle des versions spécifiques, par exemple `["sonnet", "claude-opus-4-6"]`, à la fois `/model opus` et `--model opus` sélectionnent Claude Opus 4.6, la version Opus la plus récente autorisée, et affichent un avis nommant à la fois les modèles demandés et substitués. Avant la v2.1.205, un alias dont la version la plus récente était en dehors de la liste était rejeté ou remplacé comme toute autre sélection bloquée, même lorsque la liste permettait une version antérieure.

Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, et [Mantle](/docs/fr/amazon-bedrock#use-the-mantle-endpoint) utilisent des ID de déploiement spécifiques au fournisseur plutôt que des ID de modèle Anthropic, de sorte qu'un alias bloqué là suit le comportement de rejet et de remplacement ci-dessous.

Claude Code gère toute autre sélection bloquée selon l'endroit où le modèle a été défini :

* **`/model`** : le basculement est rejeté avec une erreur
* **Drapeau `--model`, `ANTHROPIC_MODEL`, ou paramètre `model`** : la valeur est remplacée au démarrage par un avertissement nommant à la fois les modèles demandés et substitués, et la session démarre sur le modèle par défaut
* **Remplacement de sous-agent, de compétence ou de commande** : le remplacement revient au modèle hérité ou par défaut plutôt que d'échouer la demande
* **Paramètre `advisorModel`** : le conseiller est désactivé pour la session
* **Drapeau `--advisor`** : Claude Code quitte avec une erreur au lancement

Les modèles exclus sont masqués du sélecteur `/model`. Un ID de modèle complet dans la liste qui n'a pas de ligne de sélecteur intégrée, comme une version antérieure que la liste épingle, apparaît dans le sélecteur `/model` comme sa propre ligne étiquetée. Avant la v2.1.199, un tel ID n'était sélectionnable qu'en tapant `/model <id>`.

Les changements de modèle que Claude Code effectue en votre nom sont vérifiés de la même manière :

* **[Chaînes de modèle de secours](#fallback-model-chains)** : les éléments en dehors de la liste d'autorisation sont supprimés
* **Mises à niveau en mode plan** : sur l'API Anthropic et Claude Platform sur AWS, une mise à niveau telle que [`opusplan`](#opusplan-model-setting) vers un modèle exclu utilise la version la plus récente autorisée de la famille de mise à niveau. Sur les fournisseurs avec des ID de modèle spécifiques au fournisseur, et lorsqu'aucune version n'est autorisée, la mise à niveau est ignorée et la planification continue sur le modèle de la session
* **[Repli automatique de modèle](#automatic-model-fallback)** : un repli dont la cible est exclue ne s'exécute pas, de sorte que la demande signalée se termine par un refus
* **[Mode rapide](/docs/fr/fast-mode)** : l'activation du mode rapide est refusée lorsque le modèle sur lequel la session s'exécuterait ensuite est en dehors de la liste d'autorisation

```json theme={null}
{
  "availableModels": ["sonnet", "haiku"]
}
```

<h3 id="surface-coverage">
  Couverture de surface
</h3>

Chaque surface applique la liste d'autorisation qu'elle reçoit. Le mécanisme de livraison qui atteint chaque surface diffère :

| Mécanisme de livraison                                                                            | CLI et IDE | Sessions locales de bureau | Sessions web, mobile et cloud | Agent SDK et non-interactif | Cowork              |
| :------------------------------------------------------------------------------------------------ | :--------- | :------------------------- | :---------------------------- | :-------------------------- | :------------------ |
| [Paramètres gérés par le serveur](/docs/fr/server-managed-settings) depuis la console d'administration | Appliqué   | Appliqué                   | Appliqué                      | Appliqué                    | Non livré           |
| [Fichiers de paramètres gérés ou MDM](/docs/fr/settings#settings-files)                                | Appliqué   | Appliqué                   | Non livré                     | Appliqué                    | Appliqué où déployé |

* Les sessions cloud, sur [Claude Code sur le web](/docs/fr/claude-code-on-the-web) ou dans l'application de bureau, s'exécutent sur des machines virtuelles gérées par Anthropic : les paramètres déployés sur votre appareil ne les atteignent pas, donc livrez la liste d'autorisation via les paramètres gérés par le serveur. Un changement de modèle en milieu de session dans une session cloud est rejeté lorsque le modèle demandé est exclu par la liste d'autorisation. Le rejet côté serveur à la création de session s'applique aux [restrictions de modèle d'organisation](#organization-model-restrictions), pas à la clé de paramètres `availableModels`.
* Cowork, l'onglet de travail agentique dans l'application Claude Desktop, n'est pas une surface Claude Code et ne reçoit pas les paramètres gérés par le serveur par conception. Un fichier de paramètres gérés s'applique aux sessions Cowork lorsqu'il est présent où la session s'exécute ; les sessions Cowork distantes s'exécutent sur des machines virtuelles gérées par Anthropic, où un fichier déployé sur l'appareil n'est pas présent.
* Les sessions sur les [fournisseurs tiers](/docs/fr/server-managed-settings#platform-availability) tels que Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, et [Claude Platform sur AWS](/docs/fr/claude-platform-on-aws) ne reçoivent pas les paramètres gérés par le serveur, donc livrez la liste d'autorisation via MDM ou des fichiers de paramètres gérés là-bas.
* La livraison gérée par le serveur nécessite également que la session s'authentifie avec une connexion d'organisation ou une clé API directement configurée. Les flottes qui génèrent des clés uniquement via un script [`apiKeyHelper`](/docs/fr/settings#available-settings) doivent livrer la liste d'autorisation via MDM ou des fichiers de paramètres gérés.
* L'onglet Code de bureau héberge également les [sessions SSH](/docs/fr/desktop#ssh-sessions), qui lisent le fichier de paramètres gérés depuis l'hôte distant sur lequel elles s'exécutent. Voir [Paramètres gérés de bureau](/docs/fr/desktop#managed-settings).
* Les sélecteurs de modèle sur claude.ai et dans l'application de bureau masquent ou grisent les modèles exclus par la liste d'autorisation de votre organisation. L'état du sélecteur est une commodité pour les utilisateurs ; l'application se fait dans la session.

<h3 id="default-model-behavior">
  Comportement du modèle par défaut
</h3>

L'option Par défaut dans le sélecteur de modèle n'est pas affectée par `availableModels` sauf si [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) est également défini. En soi, `availableModels` laisse Par défaut disponible, se résolvant au [défaut d'exécution](#default-model-setting) du système pour le compte. Si ce défaut est un modèle que vous avez l'intention de restreindre, définissez également `enforceAvailableModels`.

Un tableau `availableModels` vide n'engage jamais l'application du modèle par défaut : avec `availableModels: []`, les sélections de modèle nommées sont bloquées mais le modèle Par défaut pour le type de compte reste utilisable indépendamment de `enforceAvailableModels`.

<h3 id="enforce-the-allowlist-for-the-default-model">
  Appliquer la liste d'autorisation au modèle par défaut
</h3>

Définissez `enforceAvailableModels: true` aux côtés d'une liste `availableModels` non vide dans les paramètres gérés pour étendre la liste d'autorisation à l'option Par défaut. Cela nécessite Claude Code v2.1.175 ou ultérieur.

```json theme={null}
{
  "availableModels": ["sonnet", "haiku"],
  "enforceAvailableModels": true
}
```

L'option Par défaut se résout à la valeur par défaut du type de compte, ou au [modèle par défaut de l'organisation](#organization-default-model) lorsqu'un administrateur en a défini un. Lorsque ce modèle n'est pas dans la liste d'autorisation, l'option Par défaut se résout à la première entrée `availableModels` qui nomme un modèle autorisé et disponible, et la ligne Par défaut du sélecteur `/model` affiche ce modèle. Cela s'applique partout où la valeur par défaut est atteinte : démarrage de session, sélection de Par défaut dans `/model`, le mot-clé `"default"` dans les [chaînes de modèle de secours](#fallback-model-chains), et le repli utilisé lorsqu'une sélection exclue est supprimée.

`enforceAvailableModels` n'a aucun effet lorsque `availableModels` n'est pas défini ou vide : avec `availableModels: []`, le modèle Par défaut pour le type de compte reste utilisable, de sorte que le paramètre ne peut pas verrouiller les utilisateurs hors de chaque modèle. Lorsque `availableModels` est non vide mais qu'aucune entrée ne se résout à un modèle autorisé et disponible, l'application se dégrade et Par défaut revient à la valeur par défaut du type de compte, avec un avertissement visible uniquement sous `--debug`. Conservez au moins une entrée garantie disponible dans la liste pour éviter cela.

Déployez les deux clés dans la [source gérée de plus haute priorité](/docs/fr/settings#settings-precedence) : les sources gérées déployées par l'administrateur ne fusionnent pas, de sorte qu'une paire placée dans un fichier de paramètres gérés est ignorée lorsque la console d'administration livre des paramètres.

<h3 id="control-the-model-users-run-on">
  Contrôler le modèle sur lequel les utilisateurs s'exécutent
</h3>

Le paramètre `model` est une sélection initiale, pas une application. Il définit quel modèle est actif au démarrage d'une session, mais les utilisateurs peuvent toujours ouvrir `/model` et choisir Par défaut, qui se résout au [défaut d'exécution](#default-model-setting) du système indépendamment de ce que `model` est défini, sauf si [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) le redirige.

Pour contrôler complètement l'expérience du modèle, combinez ces paramètres :

* **`availableModels`** : restreint les modèles nommés vers lesquels les utilisateurs peuvent basculer
* **`enforceAvailableModels`** : étend la liste d'autorisation `availableModels` à l'option Par défaut, de sorte que Par défaut ne peut pas se résoudre à un modèle en dehors de la liste
* **`model`** : définit la sélection de modèle initiale au démarrage d'une session
* **`ANTHROPIC_DEFAULT_SONNET_MODEL`** / **`ANTHROPIC_DEFAULT_OPUS_MODEL`** / **`ANTHROPIC_DEFAULT_HAIKU_MODEL`** / **`ANTHROPIC_DEFAULT_FABLE_MODEL`** : contrôlent ce vers quoi l'option Par défaut et les alias `sonnet`, `opus`, `haiku` et `fable` se résolvent

Cet exemple démarre les utilisateurs sur Sonnet 4.5, limite le sélecteur à Sonnet et Haiku, et garantit que Par défaut se résout à un modèle sur la liste d'autorisation plutôt qu'à la valeur par défaut du niveau :

```json theme={null}
{
  "model": "claude-sonnet-4-5",
  "availableModels": ["claude-sonnet-4-5", "haiku"],
  "enforceAvailableModels": true,
  "env": {
    "ANTHROPIC_DEFAULT_SONNET_MODEL": "claude-sonnet-4-5"
  }
}
```

Sans `enforceAvailableModels` ou le bloc `env`, un utilisateur qui sélectionne Par défaut dans le sélecteur obtiendrait la dernière version pour son niveau, contournant l'épinglage de version dans `model` et `availableModels`. Les deux paramètres couvrent des portées différentes : `enforceAvailableModels` fait que Par défaut obéit à la liste d'autorisation, tandis que le bloc `env` épingle la version à laquelle un alias autorisé tel que `sonnet` se résout. Utilisez `enforceAvailableModels` seul lorsque restreindre les familles de modèles est suffisant ; ajoutez le bloc `env` lorsque vous devez également épingler une version spécifique.

<h3 id="merge-behavior">
  Comportement de fusion
</h3>

Lorsque la [source de paramètres gérés de plus haute priorité](/docs/fr/server-managed-settings#settings-precedence) définit `availableModels`, cette liste seule s'applique : les entrées dans les paramètres utilisateur, projet ou local ne peuvent pas l'étendre, et les sources gérées déployées par l'administrateur ne fusionnent pas les unes avec les autres, de sorte qu'une liste déployée dans un fichier de paramètres gérés est ignorée lorsque les paramètres gérés par le serveur livrent des clés. Sinon, les listes des paramètres utilisateur, projet et local sont [concaténées et dédupliquées](/docs/fr/settings#settings-precedence) comme d'autres paramètres de tableau. À partir de Claude Code v2.1.175, la liste gérée remplace les entrées de priorité inférieure ; les versions antérieures les fusionnent.

Dans la liste effective, une entrée nommant un modèle spécifique dans une famille, qu'il s'agisse d'un préfixe de version ou d'un ID de modèle complet, désactive l'entrée générique de cette famille : `["sonnet", "claude-sonnet-4-5"]` permet uniquement les versions Sonnet 4.5, pas tous les modèles Sonnet.

<h3 id="mantle-model-ids">
  ID de modèle Mantle
</h3>

Lorsque le [point de terminaison Bedrock Mantle](/docs/fr/amazon-bedrock#use-the-mantle-endpoint) est activé, les entrées dans `availableModels` qui commencent par `anthropic.` sont ajoutées au sélecteur `/model` en tant qu'options personnalisées et acheminées vers le point de terminaison Mantle. Ceci est une exception à la correspondance d'alias décrite dans [Épingler les modèles pour les déploiements tiers](#pin-models-for-third-party-deployments). Le paramètre restreint toujours le sélecteur aux entrées listées, et un ID Mantle intègre un nom de famille, de sorte qu'il compte comme une entrée spécifique et désactive la générique de cette famille : aux côtés de tous les ID Mantle, listez les préfixes de version ou les ID complets que vous voulez garder sélectionnables. Voir [Comportement de fusion](#merge-behavior).

<h3 id="organization-model-restrictions">
  Restrictions de modèle d'organisation
</h3>

Les administrateurs d'organisation sur les plans Claude Enterprise restreignent les modèles que les membres peuvent exécuter en désactivant les modèles individuels dans la console d'administration claude.ai. Cette restriction est livrée avec les droits du compte lorsque Claude Code s'authentifie, séparée de toute liste `availableModels` dans les paramètres, et le serveur applique la même restriction indépendamment lorsqu'une session est créée. Nécessite Claude Code v2.1.187 ou ultérieur.

La restriction s'applique lorsqu'un membre se connecte ou utilise sa propre clé API. Les identifiants d'organisation, tels que les clés de service d'organisation, ne sont pas liés à un utilisateur, de sorte que la restriction ne s'applique pas à eux.

La Console Claude n'a pas de contrôle de restriction de modèle. Les organisations sans plan Claude Enterprise, y compris celles dont les membres s'authentifient via l'API Anthropic, restreignent les modèles avec [`availableModels`](#restrict-model-selection) dans les [paramètres gérés](/docs/fr/settings#settings-files) à la place, en ajoutant [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) pour couvrir l'option Par défaut. Ces paramètres sont appliqués par Claude Code lui-même, pas par le serveur.

Un modèle restreint est masqué du sélecteur `/model`. Le sélectionner par nom avec `--model`, la variable d'environnement `ANTHROPIC_MODEL`, ou le paramètre `model` affiche l'avis `Model "<name>" is restricted by your organization's settings. Using <model> instead.` et la session démarre sur un modèle autorisé. Taper `/model <name>` pour un modèle restreint est rejeté avec `Model '<name>' is restricted by your organization's settings. Run /model to choose a different model.` et la session conserve son modèle actuel.

Un [alias de famille de modèles](#restrict-model-selection) tel que `opus` se résout à la version la plus récente de sa famille que l'organisation permet, avec le même avis de substitution. `/model <alias>` est rejeté uniquement lorsque chaque version de sa famille est restreinte ; un alias défini avec `--model`, `ANTHROPIC_MODEL`, ou le paramètre `model` est toujours remplacé au démarrage dans ce cas. Avant la v2.1.205, un alias de famille était substitué ou rejeté en fonction de sa version la plus récente seule, même lorsqu'une version antérieure était autorisée.

Les restrictions s'appliquent au niveau de l'organisation ou par rôle :

* La désactivation d'un modèle au niveau de l'organisation le supprime pour chaque membre.
* L'accès au niveau du rôle accorde différents modèles à différents rôles personnalisés, et un membre qui détient plusieurs rôles peut utiliser n'importe quel modèle qu'un de ses rôles accorde.
* Les modèles Haiku sont toujours disponibles et ne peuvent pas être désactivés, de sorte que chaque membre conserve au moins un modèle utilisable.
* Un changement d'accès prend effet sur les nouvelles demandes dans environ une minute ; le sélecteur `/model` le reflète la prochaine fois qu'une session démarre.

Les deux restrictions s'appliquent ensemble : un modèle est sélectionnable uniquement lorsqu'il est autorisé par `availableModels` et non restreint par l'organisation. Les restrictions d'organisation sont livrées aux sessions sur l'API Anthropic et les déploiements de [passerelle LLM](/docs/fr/llm-gateway). Les sessions sur Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, et Claude Platform sur AWS ne les reçoivent pas, de sorte qu'utilisez `availableModels` sur ces fournisseurs à la place.

<h2 id="organization-default-model">
  Modèle par défaut de l'organisation
</h2>

Les administrateurs d'organisation sur les plans Claude Enterprise peuvent définir un modèle par défaut pour les membres de Claude Code à partir de la console d'administration claude.ai, pour l'ensemble de l'organisation ou par rôle personnalisé. Lorsqu'un est défini, l'option Par défaut se résout à ce modèle au lieu de la [valeur par défaut du type de compte](#default-model-setting). Nécessite Claude Code v2.1.196 ou ultérieur.

La ligne Par défaut dans le sélecteur `/model` affiche le nom du modèle par défaut de l'organisation avec l'étiquette Org default. L'étiquette lit Org default que l'administrateur ait défini la valeur par défaut pour l'ensemble de l'organisation ou pour votre rôle. Une valeur par défaut de rôle couvre les membres de ce rôle personnalisé et prend la priorité sur la valeur par défaut à l'échelle de l'organisation ; lorsque plusieurs de vos rôles définissent des valeurs par défaut différentes, le modèle le plus capable s'applique.

Le modèle par défaut de l'organisation est un point de départ, pas une restriction, et toute autre sélection de modèle prend la priorité sur celui-ci :

* le drapeau `--model` et la variable d'environnement `ANTHROPIC_MODEL`
* une valeur `model` dans les [paramètres gérés](/docs/fr/settings#settings-files) ou fournie via `--settings`
* une valeur `model` dans vos paramètres utilisateur, projet ou local, y compris un modèle que vous enregistrez avec `/model`

Les administrateurs peuvent également configurer le modèle par défaut de l'organisation pour remplacer la sélection de l'utilisateur. Avec le remplacement activé, il prend la priorité sur la valeur `model` dans les paramètres utilisateur, projet et local, de sorte qu'un modèle que vous enregistrez avec `/model` s'applique pour la session actuelle et le modèle par défaut de l'organisation revient au prochain lancement. Lorsque votre sélection diffère, `/model` affiche `Your organization's default (<model>) applies on restart`. Le drapeau `--model`, `ANTHROPIC_MODEL`, les paramètres gérés et `--settings` ont toujours la priorité même avec le remplacement activé. Le remplacement est disponible pour un ensemble limité d'organisations ; demandez à votre équipe de compte Anthropic la disponibilité.

Pour limiter les modèles que les membres peuvent sélectionner, utilisez plutôt les [restrictions de modèle d'organisation](#organization-model-restrictions) ou [`availableModels`](#restrict-model-selection).

Claude Code lit le modèle par défaut de l'organisation une fois au démarrage, de sorte qu'une valeur par défaut que l'administrateur change en milieu de session prend effet au prochain lancement.

Lorsque le modèle par défaut de l'organisation ne remplace pas la sélection de l'utilisateur, le premier lancement interactif après que l'administrateur le change efface la clé `model` de vos paramètres utilisateur une fois, de sorte que la nouvelle valeur par défaut s'applique. Il ne change rien d'autre dans le fichier, et un modèle que vous enregistrez avec `/model` après ce lancement est conservé.

Le modèle par défaut de l'organisation passe par les mêmes vérifications de restriction que tout autre modèle Par défaut avant d'être adopté :

* [`availableModels`](#restrict-model-selection) en soi ne contraint jamais l'option Par défaut, de sorte qu'un modèle par défaut de l'organisation en dehors de la liste d'autorisation s'applique toujours. Lorsque [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) est également défini, un modèle par défaut de l'organisation en dehors de la liste d'autorisation est remappé à la première entrée de la liste d'autorisation, comme tout autre Par défaut
* un modèle par défaut de l'organisation que les [restrictions de modèle d'organisation](#organization-model-restrictions) refusent pour votre compte est remplacé par le modèle le plus récent autorisé dans sa famille, ou une famille moins coûteuse lorsque chaque version de celle-ci est restreinte
* un modèle par défaut de l'organisation qui n'est pas disponible pour votre compte du tout, comme Fable 5 sous [rétention de données zéro](/docs/fr/zero-data-retention), est ignoré, et l'option Par défaut se résout à la valeur par défaut du type de compte

À partir de la v2.1.199, lorsque le modèle par défaut de l'organisation est une famille de modèles différente de la valeur par défaut habituelle du type de compte, le sélecteur `/model` conserve une ligne séparée pour cette famille habituelle, de sorte que vous pouvez toujours basculer vers elle pour une session. Dans les versions v2.1.196 à v2.1.198, cette ligne est manquante du sélecteur.

Le modèle par défaut de l'organisation est livré aux sessions authentifiées avec l'API Anthropic. Les sessions sur les déploiements de [passerelle LLM](/docs/fr/llm-gateway), Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, et Claude Platform sur AWS ne le reçoivent pas. Pour définir une valeur par défaut sur ces déploiements, utilisez plutôt la clé `model` dans les [paramètres gérés](/docs/fr/settings#settings-files).

<h2 id="organization-effort-limits">
  Limites d'effort de l'organisation
</h2>

Les administrateurs d'organisation sur les plans Claude Enterprise peuvent définir un [niveau d'effort](#adjust-effort-level) maximum par modèle pour chaque rôle personnalisé, aux côtés des [restrictions de modèle d'organisation](#organization-model-restrictions) au niveau du rôle. Les niveaux au-dessus du plafond ne sont pas offerts dans le sélecteur `/effort`, et nommer un niveau supérieur avec `--effort` ou `/effort` s'exécute au plafond à la place. Dans les sessions interactives et les exécutions `--print` en texte brut, un avertissement nomme les niveaux demandés et appliqués ; avec une sortie `json` ou `stream-json` ou dans les agents d'arrière-plan, le serrage s'applique silencieusement. Les plafonds sont par modèle, de sorte que le changement de modèles peut modifier les niveaux disponibles. Lorsque plusieurs de vos rôles accordent le même modèle, le plafond le moins restrictif s'applique. Nécessite Claude Code v2.1.195 ou ultérieur.

Les limites d'effort sont livrées avec les [restrictions de modèle d'organisation](#organization-model-restrictions) et suivent la même disponibilité du fournisseur : les sessions sur Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, et Claude Platform sur AWS ne les reçoivent pas.

<h2 id="special-model-behavior">
  Comportement spécial du modèle
</h2>

<h3 id="default-model-setting">
  Paramètre de modèle `default`
</h3>

Le comportement de `default` dépend de votre type de compte :

* **Max, Team Premium, Enterprise pay-as-you-go, et API Anthropic** : par défaut Opus 4.8
* **Claude Platform sur AWS, Amazon Bedrock et Google Cloud's Agent Platform** : par défaut Opus 4.8
* **Pro, Team Standard, et sièges d'abonnement Enterprise** : par défaut Sonnet 5
* **Microsoft Foundry** : par défaut Sonnet 4.5

Enterprise pay-as-you-go signifie une organisation Enterprise facturée à l'utilisation plutôt que par siège d'abonnement.

Avant v2.1.207, `default` se résolvait en Opus 4.7 sur Claude Platform sur AWS et en Sonnet 4.5 sur Amazon Bedrock et Google Cloud's Agent Platform.

Lorsqu'un administrateur a défini un [modèle par défaut de l'organisation](#organization-default-model), `default` se résout à ce modèle au lieu de la valeur par défaut du type de compte ci-dessus. Nécessite Claude Code v2.1.196 ou ultérieur.

Lorsque les paramètres gérés [appliquent la liste d'autorisation pour le modèle par défaut](#enforce-the-allowlist-for-the-default-model) et que le modèle par défaut du type de compte n'est pas dans `availableModels`, `default` se résout vers le modèle par défaut appliqué au lieu de la valeur par défaut du type de compte ci-dessus. Lorsque les deux s'appliquent, le modèle par défaut de l'organisation remplace d'abord la valeur par défaut du type de compte et l'application s'applique ensuite à celui-ci : un modèle par défaut de l'organisation autorisé est conservé, tandis qu'un en dehors de la liste se résout vers le Par défaut appliqué.

Fable 5 n'est le modèle par défaut sur aucun type de compte. Les sessions utilisent Fable 5 uniquement après que vous l'ayez choisi, avec `/model fable`, un paramètre `model`, ou l'alias `best` où Fable 5 est disponible. Le choisir avec `/model` l'enregistre comme modèle sélectionné dans vos paramètres utilisateur, de sorte que les sessions ultérieures commencent sur Fable 5 jusqu'à ce que vous changiez de modèles.

<h3 id="opusplan-model-setting">
  Paramètre de modèle `opusplan`
</h3>

L'alias de modèle `opusplan` fournit une approche hybride automatisée :

* **En mode plan** : utilise `opus` pour le raisonnement complexe et les décisions architecturales
* **En mode exécution** : bascule automatiquement vers `sonnet` pour la génération de code et l'implémentation

Cela associe le raisonnement d'Opus pour la planification avec l'efficacité de Sonnet pour l'exécution.

La phase Opus en mode plan utilise la même fenêtre de contexte que le paramètre de modèle `opus`. Sur les niveaux d'abonnement où Opus est [automatiquement mis à niveau vers un contexte 1M](#extended-context), `opusplan` reçoit la mise à niveau en mode plan également. Pour forcer un contexte 1M pour les deux phases lorsque vous n'êtes pas sur un niveau de mise à niveau automatique, définissez le modèle sur `opusplan[1m]`.

Lorsque [`availableModels`](#restrict-model-selection) exclut le nouvel Opus mais permet une version antérieure, par exemple `["sonnet", "claude-opus-4-6"]`, `opusplan` utilise le nouvel Opus autorisé pour la planification et reste sur Sonnet uniquement lorsque chaque Opus est exclu. Une session Haiku qui se mettrait normalement à niveau vers Sonnet en mode plan utilise de même le nouvel Sonnet autorisé, et reste sur Haiku uniquement lorsque chaque Sonnet est exclu. Avant v2.1.205, le mode plan restait sur le modèle de la session chaque fois que la version la plus récente de la famille de mise à niveau était exclue, même lorsque la liste d'autorisation permettait une version antérieure.

La substitution d'une version antérieure autorisée s'applique sur l'API Anthropic et [Claude Platform sur AWS](/docs/fr/claude-platform-on-aws). Sur Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry et Mantle, dont les déploiements utilisent des ID de modèle spécifiques au fournisseur, le mode plan reste sur le modèle de la session chaque fois que le modèle de mise à niveau est exclu.

Pour une approche hybride où Claude décide en cours de tâche quand consulter un deuxième modèle plutôt que de basculer à la limite du plan, voir l'[outil advisor](/docs/fr/advisor).

<h3 id="fallback-model-chains">
  Chaînes de modèles de secours
</h3>

Lorsque le modèle principal est surchargé, indisponible ou retourne une autre erreur serveur non renouvelable, Claude Code peut basculer vers un modèle de secours au lieu d'échouer la demande. Les erreurs d'authentification, de facturation, de limite de débit, de taille de demande et de transport ne déclenchent jamais un basculement ; celles-ci suivent leur gestion normale des tentatives et des erreurs.

Configurez un ou plusieurs modèles de secours et Claude Code les essaie dans l'ordre, affichant un avis lors du basculement. Le basculement dure uniquement pour le tour actuel, de sorte que votre message suivant essaie d'abord le modèle principal à nouveau. Les chaînes sont limitées à trois modèles après suppression des doublons, et les entrées supplémentaires sont ignorées.

Définissez une chaîne pour une session avec le drapeau `--fallback-model`, qui accepte une liste séparée par des virgules :

```bash theme={null}
claude --fallback-model sonnet,haiku
```

Pour persister une chaîne entre les sessions, définissez `fallbackModel` dans [paramètres](/docs/fr/settings) comme un tableau :

```json theme={null}
{
  "fallbackModel": ["claude-sonnet-5", "claude-haiku-4-5"]
}
```

Le drapeau `--fallback-model` prend la priorité sur le paramètre `fallbackModel`. Chaque élément accepte un nom de modèle ou un alias, et `"default"` se développe vers le modèle par défaut.

Deux cas entraînent le saut d'un élément :

* **Modèle indisponible** : un modèle qui ne peut pas être atteint, comme un modèle retiré épinglé dans les paramètres, est ignoré et Claude Code continue vers l'élément suivant.
* **En dehors de la liste d'autorisation** : un élément non autorisé par [`availableModels`](#restrict-model-selection) est supprimé lors de la lecture de la chaîne et n'est jamais essayé.

<h3 id="automatic-model-fallback">
  Basculement automatique du modèle
</h3>

Cette section couvre le basculement basé sur le contenu de Fable 5. Pour le basculement basé sur la disponibilité lorsqu'un modèle est surchargé ou indisponible, voir [Chaînes de modèles de secours](#fallback-model-chains).

Fable 5 s'exécute avec des classificateurs de sécurité pour le contenu de cybersécurité et de biologie. Lorsqu'un classificateur signale une demande, Claude Code réexécute cette demande sur le modèle Opus par défaut de votre fournisseur et affiche un avis dans la transcription. Sur l'API Anthropic, les déploiements de [passerelle LLM](/docs/fr/llm-gateway) et [Claude Platform sur AWS](/docs/fr/claude-platform-on-aws), ce modèle est Opus 4.8. Sur la [passerelle des applications Claude](/docs/fr/claude-apps-gateway), c'est Opus 4.7 sauf si vous pointez l'[alias `opus`](#environment-variables) vers un autre modèle.

La session continue ensuite sur ce modèle Opus. Pour revenir à Fable 5, exécutez `/model fable`.

La cible de basculement est vérifiée par rapport à [`availableModels`](#restrict-model-selection). Lorsqu'elle est bloquée, aucun basculement ne se produit. Le refus apparaît comme une erreur normale et le modèle de la session reste inchangé.

<h4 id="check-what-triggered-fallback">
  Vérifier ce qui a déclenché le basculement
</h4>

Le basculement peut se déclencher sur la première demande d'une session, avant que vous n'envoyiez quelque chose d'inhabituel, car la première demande porte le contexte de l'espace de travail tel que votre contenu CLAUDE.md et l'état git. Un référentiel qui contient du matériel de sécurité ou de biologie peut déclencher le classificateur sur ce contexte seul.

Pour vérifier si les personnalisations sont le déclencheur, démarrez une session avec `claude --safe-mode`, qui désactive les personnalisations telles que CLAUDE.md, les skills, les serveurs MCP et les hooks. L'état git et les noms de répertoires ne sont pas des personnalisations et sont toujours inclus.

<h4 id="ask-before-switching">
  Demander avant de basculer
</h4>

Pour décider ce qui se passe chaque fois qu'une demande est signalée, plutôt que de basculer automatiquement, exécutez `/config` et désactivez « switch models when a message is flagged ». Une demande signalée met alors la session en pause avec deux options : basculer vers le modèle Opus, ou modifier l'invite et réessayer sur Fable 5.

Certains cas se comportent différemment :

* Si les deux modèles signalent la même demande, vous pouvez modifier l'invite et réessayer, ou démarrer une nouvelle session.
* Sur les sessions mobiles [Claude Code sur le web](/docs/fr/claude-code-on-the-web), la modification et la nouvelle tentative ne sont pas prises en charge. Basculez les modèles, ou continuez la session à partir d'un navigateur de bureau ou de l'application de bureau.
* En [mode non interactif](/docs/fr/cli-reference#cli-flags) et les intégrations SDK qui ne peuvent pas afficher l'invite, une demande signalée termine le tour avec un refus à la place.
* Lorsque la cible de basculement est bloquée par [`availableModels`](#restrict-model-selection), l'invite n'est pas affichée. La demande signalée se termine par le refus, de la même manière que le basculement automatique lorsque la cible est bloquée.

<h4 id="enable-fallback-on-bedrock-agent-platform-and-foundry">
  Activer le basculement sur Bedrock, Agent Platform et Foundry
</h4>

Sur [Amazon Bedrock](/docs/fr/amazon-bedrock), [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai) et [Microsoft Foundry](/docs/fr/microsoft-foundry), les ID de modèle sont spécifiques au fournisseur, de sorte que le basculement automatique ne fonctionne que lorsque Claude Code peut identifier les deux modèles impliqués :

* Claude Code doit reconnaître le modèle actuel comme Fable 5 : l'ID de modèle contient `claude-fable-5`, correspond à la valeur de `ANTHROPIC_DEFAULT_FABLE_MODEL`, ou est mappé avec [`modelOverrides`](#override-model-ids-per-version).
* La cible de basculement doit se résoudre en un modèle Opus : la valeur de `ANTHROPIC_DEFAULT_OPUS_MODEL` si définie, sinon une entrée Opus 4.8 dans la liste des modèles du fournisseur.

Si l'un ou l'autre modèle ne peut pas être identifié, Claude Code ne bascule pas automatiquement. La demande signalée se termine par un message de refus, et vous pouvez basculer les modèles avec [`/model`](#setting-your-model) et réessayer. Pour activer le basculement automatique sur ces fournisseurs, définissez `ANTHROPIC_DEFAULT_FABLE_MODEL` sur votre ID de modèle Fable 5 et `ANTHROPIC_DEFAULT_OPUS_MODEL` sur votre ID de modèle Opus 4.8.

<h4 id="security-research-and-biology-workloads">
  Charges de travail de recherche en sécurité et de biologie
</h4>

Les charges de travail en sécurité offensive ou en biologie, y compris les tests de pénétration, les exercices Capture the Flag (CTF) et les bases de code adjacentes à la biologie, déclenchent fréquemment le basculement, souvent sur la première demande. Pour un travail de biologie substantiel, attendez-vous à ce que presque toutes les demandes soient réacheminées.

C'est le routage attendu pour ces domaines, pas un drapeau de compte. Si votre organisation a besoin de la capacité de classe Fable pour ce travail, demandez à votre équipe de compte Anthropic les programmes d'accès de confiance.

<h3 id="adjust-effort-level">
  Ajuster le niveau d'effort
</h3>

Les [niveaux d'effort](https://platform.claude.com/docs/en/build-with-claude/effort) contrôlent le raisonnement adaptatif, qui permet au modèle de décider si et combien réfléchir à chaque étape en fonction de la complexité de la tâche. Un effort inférieur est plus rapide et moins cher pour les tâches simples, tandis qu'un effort supérieur fournit un raisonnement plus profond pour les problèmes complexes.

Les niveaux d'effort disponibles dépendent du modèle. Les modèles non listés ici ne prennent pas en charge l'effort :

| Modèle                         | Niveaux                                 |
| :----------------------------- | :-------------------------------------- |
| Fable 5                        | `low`, `medium`, `high`, `xhigh`, `max` |
| Sonnet 5, Opus 4.8 et Opus 4.7 | `low`, `medium`, `high`, `xhigh`, `max` |
| Opus 4.6 et Sonnet 4.6         | `low`, `medium`, `high`, `max`          |

Si vous définissez un niveau que le modèle actif ne prend pas en charge, Claude Code revient au niveau le plus élevé pris en charge au niveau ou en dessous de celui que vous avez défini. Par exemple, `xhigh` s'exécute comme `high` sur Opus 4.6. Votre organisation peut également limiter les niveaux disponibles pour un modèle ; voir [Limites d'effort de l'organisation](#organization-effort-limits).

L'effort par défaut est `high` sur Fable 5, Sonnet 5, Opus 4.8, Opus 4.6 et Sonnet 4.6, et `xhigh` sur Opus 4.7.

Lorsque vous exécutez Fable 5, Opus 4.8 ou Opus 4.7 pour la première fois, Claude Code applique l'effort par défaut de ce modèle même si vous aviez précédemment défini un niveau différent pour un autre modèle : `high` sur Fable 5 et Opus 4.8, et `xhigh` sur Opus 4.7. Exécutez `/effort` à nouveau pour choisir un niveau différent après le changement. Cet effort par défaut est conservé entre les sessions jusqu'à ce que vous fassiez un choix d'effort explicite, comme exécuter `/effort` dans une session interactive ou lancer avec `--effort`.

`low`, `medium`, `high` et `xhigh` persistent entre les sessions lorsque vous les définissez dans une session interactive. Un niveau défini avec `/effort` en [mode non interactif](/docs/fr/headless), avec le drapeau `-p`, s'applique à la session actuelle uniquement et n'est pas enregistré comme votre valeur par défaut. Un `/effort` non interactif ne peut pas non plus libérer la retenue de la valeur par défaut du modèle ci-dessus : sur Fable 5, Opus 4.8 et Opus 4.7, il signale `Not applied` et la session reste à l'effort par défaut du modèle, donc passez `--effort` au lancement à la place. `max` fournit le raisonnement le plus profond sans contrainte sur les dépenses en tokens et s'applique à la session actuelle uniquement, sauf lorsqu'il est défini via la variable d'environnement `CLAUDE_CODE_EFFORT_LEVEL`.

Le menu `/effort` offre également `ultracode`. Ultracode est un paramètre de Claude Code plutôt qu'un niveau d'effort du modèle : il envoie `xhigh` au modèle et a également Claude orchestrer les [flux de travail dynamiques](/docs/fr/workflows) pour les tâches substantielles. Il s'applique à la session actuelle uniquement.

Vous pouvez activer ultracode par l'une des méthodes suivantes :

* **`/effort`** : exécutez `/effort ultracode`, ou sélectionnez-le dans le menu
* **Drapeau `--effort`** : lancez avec `claude --effort ultracode`, qui démarre la session avec un effort `xhigh` et ultracode activé
* **`--settings` ou une demande de contrôle du SDK Agent** : passez `"ultracode": true`. Une demande [`applyFlagSettings()`](/docs/fr/agent-sdk/typescript#applyflagsettings) accepte également `effortLevel: "ultracode"`

Passer `ultracode` au drapeau `--effort` ou à la valeur `effortLevel` du SDK Agent nécessite Claude Code v2.1.203 ou ultérieur. Avant v2.1.203, `--effort ultracode` affichait `Unknown --effort value 'ultracode'` et la session démarrait avec l'effort par défaut.

Le paramètre `effortLevel` persisté et la variable d'environnement `CLAUDE_CODE_EFFORT_LEVEL` n'acceptent pas `ultracode`.

Lorsque ultracode n'est pas disponible, par exemple lorsque les [flux de travail sont désactivés](/docs/fr/workflows#turn-workflows-off), `--effort ultracode` définit uniquement l'effort `xhigh`.

<h4 id="choose-an-effort-level">
  Choisir un niveau d'effort
</h4>

Chaque niveau échange les dépenses en tokens contre la capacité. La valeur par défaut convient à la plupart des tâches de codage ; ajustez lorsque vous souhaitez un équilibre différent.

| Niveau      | Quand l'utiliser                                                                                                                                                                    |
| :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `low`       | Réservez aux tâches courtes, délimitées, sensibles à la latence qui ne sont pas sensibles à l'intelligence                                                                          |
| `medium`    | Réduit l'utilisation des tokens pour le travail sensible aux coûts qui peut faire des compromis sur l'intelligence                                                                  |
| `high`      | Équilibre l'utilisation des tokens et l'intelligence. Par défaut sur Fable 5, Sonnet 5, Opus 4.8, Opus 4.6 et Sonnet 4.6                                                            |
| `xhigh`     | Raisonnement plus profond avec dépenses en tokens plus élevées. Par défaut sur Opus 4.7                                                                                             |
| `max`       | Peut améliorer les performances sur les tâches exigeantes mais peut montrer des rendements décroissants et est sujet à la surréflexion. Testez avant d'adopter largement            |
| `ultracode` | Un paramètre de Claude Code qui planifie un [flux de travail dynamique](/docs/fr/workflows) pour chaque tâche substantielle avec un raisonnement `xhigh` par message. Session uniquement |

L'échelle d'effort est calibrée par modèle, donc le même nom de niveau ne représente pas la même valeur sous-jacente entre les modèles.

<h4 id="use-ultrathink-for-one-off-deep-reasoning">
  Utiliser ultrathink pour un raisonnement profond ponctuel
</h4>

Incluez `ultrathink` n'importe où dans votre invite pour demander un raisonnement plus profond à ce tour sans modifier votre paramètre d'effort de session. Claude Code reconnaît le mot-clé et ajoute une instruction en contexte. Le niveau d'effort envoyé à l'API reste inchangé. D'autres phrases telles que « think », « think hard » et « think more » sont transmises comme du texte d'invite ordinaire et ne sont pas reconnues comme des mots-clés.

<h4 id="set-the-effort-level">
  Définir le niveau d'effort
</h4>

Vous pouvez modifier l'effort par l'une des méthodes suivantes :

* **`/effort`** : exécutez `/effort` sans arguments pour ouvrir un curseur interactif, `/effort` suivi d'un nom de niveau pour le définir directement, ou `/effort auto` pour réinitialiser à la valeur par défaut du modèle
* **Dans `/model`** : utilisez les touches fléchées gauche/droite pour ajuster le curseur d'effort lors de la sélection d'un modèle
* **Drapeau `--effort`** : passez un nom de niveau pour le définir pour une seule session lors du lancement de Claude Code
* **Variable d'environnement** : définissez `CLAUDE_CODE_EFFORT_LEVEL` sur un nom de niveau ou `auto`
* **Paramètres** : définissez `effortLevel` sur `low`, `medium`, `high` ou `xhigh` dans votre fichier de paramètres. `max` et `ultracode` sont [session uniquement](#adjust-effort-level) et ne sont pas acceptés ici
* **Frontmatter de skill et de subagent** : définissez `effort` dans un fichier markdown de [skill](/docs/fr/skills#frontmatter-reference) ou de [subagent](/docs/fr/sub-agents#supported-frontmatter-fields) pour remplacer le niveau d'effort lorsque ce skill ou subagent s'exécute

La variable d'environnement prend la priorité sur toutes les autres méthodes, puis votre niveau configuré, puis la valeur par défaut du modèle. L'effort du frontmatter s'applique lorsque ce skill ou subagent est actif, remplaçant le niveau de session mais pas la variable d'environnement.

Le curseur d'effort apparaît dans `/model` lorsqu'un modèle pris en charge est sélectionné. Le niveau d'effort actuel est également affiché à côté du logo et du spinner, par exemple « with low effort », vous pouvez donc confirmer quel paramètre est actif sans ouvrir `/model`.

<h4 id="adaptive-reasoning-and-fixed-thinking-budgets">
  Raisonnement adaptatif et budgets de réflexion fixes
</h4>

Le raisonnement adaptatif rend la réflexion optionnelle à chaque étape, donc Claude peut répondre plus rapidement aux invites de routine et réserver une réflexion plus profonde pour les étapes qui en bénéficient. Si vous souhaitez que Claude réfléchisse plus ou moins souvent que le niveau actuel ne le produit, vous pouvez le dire directement dans votre invite ou dans `CLAUDE.md` ; le modèle répond à cette orientation dans son paramètre d'effort.

Fable 5, Sonnet 5, et Opus 4.7 et versions ultérieures utilisent toujours le raisonnement adaptatif. Le mode de budget de réflexion fixe et `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING` ne s'appliquent pas à eux.

Sur Opus 4.6 et Sonnet 4.6, vous pouvez définir `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` pour revenir au budget de réflexion fixe précédent contrôlé par `MAX_THINKING_TOKENS`. Voir [variables d'environnement](/docs/fr/env-vars).

<h3 id="extended-thinking">
  Réflexion étendue
</h3>

La réflexion étendue est le raisonnement que Claude émet avant de répondre. Sur les modèles qui prennent en charge le [raisonnement adaptatif](#adjust-effort-level), le niveau d'effort est le contrôle principal de la quantité de réflexion qui se produit ; les paramètres ci-dessous activent ou désactivent la réflexion et contrôlent son affichage.

| Contrôle                              | Comment le définir                                                                                                                                                                                                                                                                                                                                                                                                             |
| :------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Basculer pour la session actuelle     | Appuyez sur `Option+T` sur macOS ou `Alt+T` sur Windows et Linux                                                                                                                                                                                                                                                                                                                                                               |
| Définir la valeur par défaut globale  | Exécutez `/config` et basculez le mode de réflexion. Enregistré en tant que `alwaysThinkingEnabled` dans `~/.claude/settings.json`                                                                                                                                                                                                                                                                                             |
| Désactiver indépendamment de l'effort | Définissez [`MAX_THINKING_TOKENS=0`](/docs/fr/env-vars), qui désactive la réflexion sur l'API Anthropic sauf sur Fable 5. Sur les [fournisseurs tiers](/docs/fr/third-party-integrations) cela omet le paramètre `thinking` à la place, et les modèles de raisonnement adaptatif peuvent toujours réfléchir. D'autres valeurs s'appliquent uniquement avec un [budget de réflexion fixe](#adaptive-reasoning-and-fixed-thinking-budgets) |

La réflexion ne peut pas être désactivée sur Fable 5. Le basculement de session, `alwaysThinkingEnabled` et `MAX_THINKING_TOKENS=0` n'ont aucun effet là, et Fable 5 décide à chaque étape combien réfléchir en fonction du niveau d'effort.

La sortie de réflexion est réduite par défaut. Appuyez sur `Ctrl+O` pour basculer le mode verbeux et voir le raisonnement en tant que texte gris en italique. Les sessions interactives sur l'API Anthropic reçoivent des blocs de réflexion masqués par défaut, donc définissez `showThinkingSummaries: true` dans les [paramètres](/docs/fr/settings) si vous souhaitez que les résumés complets soient disponibles lorsque vous développez. Vous êtes facturé pour tous les tokens de réflexion générés, même lorsqu'ils sont réduits ou masqués.

<h3 id="extended-context">
  Contexte étendu
</h3>

Fable 5, Sonnet 5, Opus 4.6 et versions ultérieures, et Sonnet 4.6, prennent en charge une [fenêtre de contexte de 1 million de tokens](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) pour les sessions longues avec de grandes bases de code.

La disponibilité varie selon le modèle et le plan. Sur l'API Anthropic, Fable 5, Sonnet 5, Opus 4.8 et Opus 4.7 s'exécutent toujours avec la fenêtre 1M. Sur les plans Max, Team et Enterprise, Opus est automatiquement mis à niveau vers un contexte 1M sans configuration supplémentaire. Cela s'applique aux sièges Team Standard et Team Premium. Sonnet 4.6 avec contexte 1M ne fait pas partie de la mise à niveau automatique et nécessite des [crédits d'utilisation](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) sur tous les plans d'abonnement, y compris Max.

| Plan                    | Opus avec contexte 1M                                                                                                    | Sonnet 4.6 avec contexte 1M                                                                                              |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------ |
| Max, Team et Enterprise | Inclus dans l'abonnement                                                                                                 | Nécessite des [crédits d'utilisation](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) |
| Pro                     | Nécessite des [crédits d'utilisation](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) | Nécessite des [crédits d'utilisation](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) |
| API et pay-as-you-go    | Accès complet                                                                                                            | Accès complet                                                                                                            |

Pour désactiver complètement le contexte 1M, définissez `CLAUDE_CODE_DISABLE_1M_CONTEXT=1`. Cela supprime les variantes de modèle 1M du sélecteur de modèle. Voir [variables d'environnement](/docs/fr/env-vars).

La fenêtre de contexte 1M utilise la tarification standard du modèle sans prime pour les tokens au-delà de 200 K. Pour les plans où le contexte étendu est inclus dans votre abonnement, l'utilisation reste couverte par votre abonnement. Pour les plans qui accèdent au contexte étendu via des crédits d'utilisation, les tokens sont facturés aux crédits d'utilisation.

Si votre compte prend en charge le contexte 1M, l'option apparaît dans le sélecteur de modèle (`/model`) dans les dernières versions de Claude Code. Si vous ne la voyez pas, essayez de redémarrer votre session.

Vous pouvez également utiliser le suffixe `[1m]` avec les alias de modèle ou les noms de modèle complets :

```bash theme={null}
# Utiliser l'alias opus[1m] ou sonnet[1m]
/model opus[1m]
/model sonnet[1m]

# Ou ajouter [1m] à un nom de modèle complet
/model claude-opus-4-8[1m]
```

<h4 id="sonnet-5-context-window">
  Fenêtre de contexte de Sonnet 5
</h4>

Sur l'API Anthropic, Sonnet 5 s'exécute toujours avec la fenêtre de contexte 1M. Il n'existe pas de variante 200K, pas de suffixe `[1m]` à sélectionner, et aucun crédit d'utilisation n'est requis sur aucun plan. Les sessions se compactent automatiquement avant que la fenêtre ne se remplisse, à environ 967K tokens par défaut ; définissez [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/fr/env-vars) pour choisir un seuil différent.

Deux configurations limitent la fenêtre à 200K à la place et compactent automatiquement à cette limite :

* **Passerelle LLM** : lorsque `ANTHROPIC_BASE_URL` pointe vers une [passerelle](/docs/fr/llm-gateway), Claude Code ne peut pas vérifier la prise en charge du contexte 1M. Pour utiliser la fenêtre complète, sélectionnez Sonnet 5 (1M context) dans le sélecteur de modèle, qui correspond à `sonnet[1m]`.
* **`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`** : traite les sessions Sonnet 5 comme ayant une fenêtre de 200K, pour les déploiements qui doivent limiter le contexte.

<h2 id="checking-your-current-model">
  Vérifier votre modèle actuel
</h2>

Vous pouvez voir quel modèle vous utilisez actuellement de plusieurs façons :

* Dans la [ligne d'état](/docs/fr/statusline), si vous en avez une configurée
* Dans `/status`, qui affiche également vos informations de compte

<h2 id="add-a-custom-model-option">
  Ajouter une option de modèle personnalisé
</h2>

Utilisez `ANTHROPIC_CUSTOM_MODEL_OPTION` pour ajouter une seule entrée personnalisée au sélecteur `/model` sans remplacer les alias intégrés. Ceci est utile pour tester les ID de modèle que Claude Code ne répertorie pas par défaut. Pour les déploiements de passerelle LLM, Claude Code peut remplir le sélecteur à partir du point de terminaison `/v1/models` de la passerelle lorsque `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1` est défini, donc cette variable n'est nécessaire que lorsque la découverte est désactivée ou ne retourne pas le modèle que vous souhaitez. Voir [découverte du modèle de passerelle](/docs/fr/llm-gateway-protocol#model-discovery).

Cet exemple définit les trois variables pour rendre un déploiement Opus acheminé par passerelle sélectionnable :

```bash theme={null}
export ANTHROPIC_CUSTOM_MODEL_OPTION="my-gateway/claude-opus-4-8"
export ANTHROPIC_CUSTOM_MODEL_OPTION_NAME="Opus via Gateway"
export ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION="Custom deployment routed through the internal LLM gateway"
```

L'entrée personnalisée apparaît au bas du sélecteur `/model`. `ANTHROPIC_CUSTOM_MODEL_OPTION_NAME` et `ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION` sont optionnels. S'ils sont omis, l'ID du modèle est utilisé comme nom et la description par défaut est `Custom model (<model-id>)`.

Claude Code ignore la validation pour l'ID de modèle défini dans `ANTHROPIC_CUSTOM_MODEL_OPTION`, vous pouvez donc utiliser n'importe quelle chaîne que votre point de terminaison API accepte. Lorsque [`availableModels`](#restrict-model-selection) est défini, incluez également l'ID de modèle personnalisé dans la liste d'autorisation : l'entrée personnalisée est filtrée du sélecteur et une sélection `--model` de celui-ci est rejetée comme tout autre modèle exclu. Un ID personnalisé qui intègre un nom de famille, tel que `my-gateway/claude-opus-4-8`, compte comme une entrée spécifique pour cette famille et désactive son caractère générique, donc listez également les versions que vous avez l'intention de garder sélectionnables. Voir [Comportement de fusion](#merge-behavior).

<h2 id="environment-variables">
  Variables d'environnement
</h2>

Vous pouvez utiliser les variables d'environnement suivantes pour contrôler les noms de modèle auxquels les alias sont mappés. Chaque valeur doit être un nom de modèle complet, ou l'identifiant équivalent pour votre fournisseur d'API.

| Variable d'environnement         | Description                                                                                                                                                                                                                                                                                                                                                                                                                            |
| -------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_DEFAULT_FABLE_MODEL`  | Le modèle à utiliser pour `fable`, et l'ID de modèle que Claude Code reconnaît comme Fable 5 pour le [basculement automatique du modèle](#automatic-model-fallback) sur les fournisseurs tiers                                                                                                                                                                                                                                         |
| `ANTHROPIC_DEFAULT_OPUS_MODEL`   | Le modèle à utiliser pour `opus`, ou pour `opusplan` lorsque le mode Plan est actif.                                                                                                                                                                                                                                                                                                                                                   |
| `ANTHROPIC_DEFAULT_SONNET_MODEL` | Le modèle à utiliser pour `sonnet`, ou pour `opusplan` lorsque le mode Plan n'est pas actif.                                                                                                                                                                                                                                                                                                                                           |
| `ANTHROPIC_DEFAULT_HAIKU_MODEL`  | Le modèle à utiliser pour `haiku`, ou [fonctionnalité d'arrière-plan](/docs/fr/costs#background-token-usage)                                                                                                                                                                                                                                                                                                                                |
| `CLAUDE_CODE_SUBAGENT_MODEL`     | Le modèle à utiliser pour tous les [subagents](/docs/fr/sub-agents#choose-a-model), les [équipes d'agents](/docs/fr/agent-teams), et les agents qu'un [workflow](/docs/fr/workflows) exécute. Accepte un alias tel que `haiku` ou un nom de modèle complet, et remplace le paramètre `model` par invocation et le frontmatter `model` de la définition du subagent. Définissez sur `inherit` pour utiliser la résolution de modèle normale à la place |

Remarque : `ANTHROPIC_SMALL_FAST_MODEL` est déprécié au profit de `ANTHROPIC_DEFAULT_HAIKU_MODEL`.

<h3 id="pin-models-for-third-party-deployments">
  Épingler les modèles pour les déploiements tiers
</h3>

Lors du déploiement de Claude Code via [Amazon Bedrock](/docs/fr/amazon-bedrock), [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai), [Microsoft Foundry](/docs/fr/microsoft-foundry), ou [Claude Platform on AWS](/docs/fr/claude-platform-on-aws), épinglez les versions de modèle avant de les déployer auprès des utilisateurs.

Sans épinglage, Claude Code utilise les alias de modèle tels que `fable`, `opus`, `sonnet` et `haiku` qui se résolvent à un ID de modèle par défaut intégré pour chaque fournisseur. Ce défaut peut être en retard par rapport à la dernière version d'Anthropic, et le modèle auquel il pointe peut ne pas encore être activé dans le compte d'un utilisateur. Lorsque le défaut n'est pas disponible, les utilisateurs d'Amazon Bedrock et de Google Cloud's Agent Platform voient un avis et la session revient à une version antérieure du modèle par défaut, ou au modèle Sonnet par défaut lorsque le défaut est un modèle Opus et qu'aucune version Opus n'est disponible. Les utilisateurs de Microsoft Foundry voient des erreurs à la place, car Microsoft Foundry n'a pas de vérification de démarrage équivalente.

<Warning>
  Définissez les variables d'environnement de modèle sur des ID de version spécifiques dans le cadre de votre configuration initiale. L'épinglage vous permet de contrôler quand vos utilisateurs passent à un nouveau modèle.
</Warning>

Utilisez les variables d'environnement suivantes avec des ID de modèle spécifiques à la version pour votre fournisseur :

| Fournisseur                   | Exemple                                                              |
| :---------------------------- | :------------------------------------------------------------------- |
| Amazon Bedrock                | `export ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'` |
| Google Cloud's Agent Platform | `export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'`              |
| Microsoft Foundry             | `export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'`              |

Appliquez le même modèle pour `ANTHROPIC_DEFAULT_FABLE_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL` et `ANTHROPIC_DEFAULT_HAIKU_MODEL`. Pour les ID de modèle actuels et hérités sur tous les fournisseurs, voir [Aperçu des modèles](https://platform.claude.com/docs/en/about-claude/models/overview). Pour mettre à niveau les utilisateurs vers une nouvelle version de modèle, mettez à jour ces variables d'environnement et redéployez.

Pour activer le [contexte étendu](#extended-context) pour un modèle épinglé, ajoutez `[1m]` à l'ID du modèle dans `ANTHROPIC_DEFAULT_OPUS_MODEL` ou `ANTHROPIC_DEFAULT_SONNET_MODEL` :

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8[1m]'
```

Le suffixe `[1m]` applique la fenêtre de contexte 1M à toute utilisation des alias `opus` et `sonnet`, y compris la phase Opus en mode plan de [`opusplan`](#opusplan-model-setting).

* Claude Code supprime le suffixe avant d'envoyer l'ID du modèle à votre fournisseur.
* N'ajoutez `[1m]` que lorsque le modèle sous-jacent [prend en charge le contexte 1M](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model).
* Le suffixe est lu par variable, et non par modèle. Sur Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry, un ID de modèle sans `[1m]` dans une variable utilise le contexte 200K même si une autre variable définit le même modèle avec le suffixe. Sonnet 5 s'exécute toujours avec la fenêtre 1M sur ces fournisseurs et n'a jamais besoin du suffixe.

<Note>
  Une liste d'autorisation `availableModels` livrée via [MDM ou un fichier de paramètres gérés](/docs/fr/settings#settings-files) s'applique toujours lors de l'utilisation de fournisseurs tiers ; les [paramètres gérés par le serveur ne sont pas livrés là](/docs/fr/server-managed-settings#platform-availability). Le filtrage correspond à un alias de modèle tel que `opus`, un préfixe de version tel que `claude-opus-4-8`, ou l'ID de modèle complet spécifique au fournisseur. Les préfixes spécifiques au fournisseur tels que `us.anthropic.` ne sont pas supprimés, donc pour autoriser un modèle spécifique, listez le même ID spécifique au fournisseur que le sélecteur affiche, ou mappez-le via [`modelOverrides`](#override-model-ids-per-version). Tout suffixe `[1m]` est supprimé de l'entrée de la liste d'autorisation et du modèle demandé avant la correspondance.
</Note>

<h3 id="customize-pinned-model-display-and-capabilities">
  Personnaliser l'affichage et les capacités du modèle épinglé
</h3>

Lorsque vous épinglez un modèle sur un fournisseur tiers, l'ID spécifique au fournisseur apparaît tel quel dans le sélecteur `/model` et Claude Code peut ne pas reconnaître les fonctionnalités que le modèle prend en charge. Vous pouvez remplacer le nom d'affichage et déclarer les capacités avec des variables d'environnement complémentaires pour chaque modèle épinglé.

Ces variables prennent effet sur les fournisseurs tiers tels qu'Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry. Les variables `_NAME` et `_DESCRIPTION` prennent également effet lorsque `ANTHROPIC_BASE_URL` pointe vers une [passerelle LLM](/docs/fr/llm-gateway). Elles n'ont aucun effet lors de la connexion directe à `api.anthropic.com`.

| Variable d'environnement                              | Description                                                                                                                                |
| ----------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_NAME`                   | Nom d'affichage pour le modèle Opus épinglé dans le sélecteur `/model`. Par défaut, l'ID du modèle lorsqu'il n'est pas défini              |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_DESCRIPTION`            | Description d'affichage pour le modèle Opus épinglé dans le sélecteur `/model`. Par défaut, `Custom Opus model` lorsqu'il n'est pas défini |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES` | Liste séparée par des virgules des capacités que le modèle Opus épinglé prend en charge                                                    |

Les mêmes suffixes `_NAME`, `_DESCRIPTION` et `_SUPPORTED_CAPABILITIES` sont disponibles pour `ANTHROPIC_DEFAULT_SONNET_MODEL`, `ANTHROPIC_DEFAULT_HAIKU_MODEL`, `ANTHROPIC_DEFAULT_FABLE_MODEL` et `ANTHROPIC_CUSTOM_MODEL_OPTION`.

Claude Code active les fonctionnalités comme les [niveaux d'effort](#adjust-effort-level) et la [réflexion étendue](#extended-thinking) en faisant correspondre l'ID du modèle à des modèles connus. Les ID spécifiques au fournisseur tels que les ARN Amazon Bedrock ou les noms de déploiement personnalisés ne correspondent souvent pas à ces modèles, laissant les fonctionnalités prises en charge désactivées. Définissez `_SUPPORTED_CAPABILITIES` pour indiquer à Claude Code les fonctionnalités que le modèle prend réellement en charge :

| Valeur de capacité     | Active                                                                                                |
| ---------------------- | ----------------------------------------------------------------------------------------------------- |
| `effort`               | [Niveaux d'effort](#adjust-effort-level) et la commande `/effort`                                     |
| `xhigh_effort`         | Le niveau d'effort `xhigh`                                                                            |
| `max_effort`           | Le niveau d'effort `max`                                                                              |
| `thinking`             | [Réflexion étendue](#extended-thinking)                                                               |
| `adaptive_thinking`    | Raisonnement adaptatif qui alloue dynamiquement la réflexion en fonction de la complexité de la tâche |
| `interleaved_thinking` | Réflexion entre les appels d'outils                                                                   |

Lorsque `_SUPPORTED_CAPABILITIES` est défini, les capacités listées sont activées et les capacités non listées sont désactivées pour le modèle épinglé correspondant. Lorsque la variable n'est pas définie, Claude Code revient à la détection intégrée basée sur l'ID du modèle.

Cet exemple épingle Opus à un ARN de modèle personnalisé Amazon Bedrock, définit un nom convivial et déclare ses capacités :

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='arn:aws:bedrock:us-east-1:123456789012:custom-model/abc'
export ANTHROPIC_DEFAULT_OPUS_MODEL_NAME='Opus via Bedrock'
export ANTHROPIC_DEFAULT_OPUS_MODEL_DESCRIPTION='Opus 4.7 routed through a Bedrock custom endpoint'
export ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES='effort,xhigh_effort,max_effort,thinking,adaptive_thinking,interleaved_thinking'
```

<h3 id="override-model-ids-per-version">
  Remplacer les ID de modèle par version
</h3>

Les variables d'environnement au niveau de la famille ci-dessus configurent un ID de modèle par alias de famille. Si vous devez mapper plusieurs versions au sein de la même famille à des ID de fournisseur distincts, utilisez plutôt le paramètre `modelOverrides`.

`modelOverrides` mappe les ID de modèle Anthropic individuels aux chaînes spécifiques au fournisseur que Claude Code envoie à l'API de votre fournisseur. Lorsqu'un utilisateur sélectionne un modèle mappé dans le sélecteur `/model`, Claude Code utilise votre valeur configurée au lieu de la valeur par défaut intégrée.

Cela permet aux administrateurs d'entreprise d'acheminer chaque version de modèle vers un ARN de profil d'inférence Amazon Bedrock spécifique, un nom de version Google Cloud's Agent Platform ou un nom de déploiement Microsoft Foundry pour la gouvernance, l'allocation des coûts ou l'acheminement régional.

Définissez `modelOverrides` dans votre [fichier de paramètres](/docs/fr/settings#settings-files) :

```json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-7": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-prod",
    "claude-opus-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-46-prod",
    "claude-sonnet-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/sonnet-prod"
  }
}
```

Les clés doivent être des ID de modèle Anthropic tels que listés dans l'[Aperçu des modèles](https://platform.claude.com/docs/en/about-claude/models/overview). Pour les ID de modèle datés, incluez le suffixe de date exactement tel qu'il apparaît là. Les clés inconnues sont ignorées.

Les remplacements remplacent les ID de modèle intégrés qui soutiennent chaque entrée dans le sélecteur `/model`. Sur Amazon Bedrock, les remplacements prennent la priorité sur tous les profils d'inférence que Claude Code découvre automatiquement au démarrage. Claude Code transmet les valeurs qui sont déjà natives du fournisseur, telles que les ARN de profil d'inférence Amazon Bedrock ou les noms de déploiement Microsoft Foundry, au fournisseur telles quelles.

Les remplacements s'appliquent également lorsque vous transmettez un ID de modèle Anthropic directement via `--model`, la variable d'environnement `ANTHROPIC_MODEL`, ou une variable d'environnement `ANTHROPIC_DEFAULT_*_MODEL`. Sur Amazon Bedrock, Google Cloud's Agent Platform et [Mantle](/docs/fr/amazon-bedrock#use-the-mantle-endpoint), un ID de modèle Anthropic sans entrée `modelOverrides` se résout au même ID spécifique au fournisseur que la ligne du sélecteur `/model` pour cette version, lorsque le fournisseur prend en charge cette version. Mantle prend en charge un sous-ensemble de versions. Pour un ID de modèle Anthropic en dehors de ce sous-ensemble, Claude Code envoie l'ID brut à Mantle sans le mapper, sauf si une entrée `modelOverrides` le couvre. Avant v2.1.200, `--model` et les valeurs des variables d'environnement atteignaient le fournisseur telles quelles sans passer par la carte de remplacement.

`modelOverrides` fonctionne aux côtés de `availableModels`. La liste d'autorisation est évaluée par rapport à l'ID de modèle Anthropic, et non à la valeur de remplacement, donc une entrée comme `"opus"` dans `availableModels` continue de correspondre même lorsque les versions d'Opus sont mappées à des ARN. Lorsque `enforceAvailableModels` est défini dans les paramètres gérés, la valeur par défaut appliquée se résout via `modelOverrides` à partir de la [source gérée de plus haute priorité](/docs/fr/server-managed-settings#settings-precedence) uniquement. Le mappage d'un administrateur, tel qu'une version épinglée à un ARN de profil d'inférence, est honoré dans la valeur par défaut appliquée. Les remplacements des paramètres utilisateur ou projet ne l'affectent pas.

Lorsque `availableModels` est défini dans les [paramètres gérés](/docs/fr/settings#settings-files), seuls les `modelOverrides` de cette source gérée s'appliquent à un ID de modèle Anthropic transmis directement via `--model` ou les variables d'environnement ci-dessus. Claude Code ignore les remplacements dans les paramètres utilisateur ou projet pour ces ID, et ne résout jamais un ID que la liste gérée exclut via `modelOverrides` à partir de n'importe quelle source de paramètres. Cette restriction de source gérée nécessite Claude Code v2.1.200 ou ultérieur. Voir [Restreindre la sélection de modèle](#restrict-model-selection) pour savoir comment les ID bloqués sont gérés.

<h3 id="prompt-caching-configuration">
  Configuration de la mise en cache des invites
</h3>

Claude Code utilise automatiquement la [mise en cache des invites](/docs/fr/prompt-caching) pour optimiser les performances et réduire les coûts. Vous pouvez désactiver la mise en cache des invites globalement ou pour des niveaux de modèle spécifiques :

| Variable d'environnement        | Description                                                                                                                            |
| ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `DISABLE_PROMPT_CACHING`        | Définissez sur `1` pour désactiver la mise en cache des invites pour tous les modèles. Prend la priorité sur les paramètres par modèle |
| `DISABLE_PROMPT_CACHING_HAIKU`  | Définissez sur `1` pour désactiver la mise en cache des invites pour les modèles Haiku uniquement                                      |
| `DISABLE_PROMPT_CACHING_SONNET` | Définissez sur `1` pour désactiver la mise en cache des invites pour les modèles Sonnet uniquement                                     |
| `DISABLE_PROMPT_CACHING_OPUS`   | Définissez sur `1` pour désactiver la mise en cache des invites pour les modèles Opus uniquement                                       |
| `DISABLE_PROMPT_CACHING_FABLE`  | Définissez sur `1` pour désactiver la mise en cache des invites pour les modèles Fable uniquement                                      |

Pour modifier le TTL du cache ou découvrir ce qui déclenche un échec du cache, voir [Comment Claude Code utilise la mise en cache des invites](/docs/fr/prompt-caching).
