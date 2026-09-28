> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configuration du modèle

> Configurez le modèle utilisé par Claude Code, les niveaux d'effort, le contexte étendu et la fenêtre d'auto-compaction

<h2 id="available-models">
  Modèles disponibles
</h2>

Pour le paramètre `model` dans Claude Code, vous pouvez configurer l'un des éléments suivants :

* Un **alias de modèle**
* Un **nom de modèle**
  * API Anthropic : un **[nom de modèle](https://platform.claude.com/docs/en/about-claude/models/overview)** complet
  * Amazon Bedrock : un ARN de profil d'inférence
  * Microsoft Foundry : un nom de déploiement
  * Agent Platform de Google Cloud : un nom de version

Pour obtenir des conseils sur le modèle et le niveau d'effort qui conviennent à différents types de travail, consultez [Choosing a Claude model and effort level in Claude Code](https://claude.com/blog/claude-model-and-effort-level-in-claude-code) sur le blog.

<Note>
  `ANTHROPIC_BASE_URL` change l'endroit où les demandes sont envoyées, non le modèle qui y répond. Pour acheminer Claude via une passerelle LLM, consultez [LLM gateways](/docs/fr/llm-gateway).
</Note>

<h3 id="model-aliases">
  Alias de modèles
</h3>

Utilisez un alias de modèle pour sélectionner les paramètres du modèle sans mémoriser les numéros de version exacts :

| Alias de modèle  | Comportement                                                                                                                                                                                                                                                                                                                                                                             |
| ---------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`default`**    | Valeur spéciale qui efface tout remplacement de modèle et revient à la [valeur par défaut du runtime pour votre compte](#default-model-setting). N'est pas en soi un alias de modèle                                                                                                                                                                                                     |
| **`best`**       | Utilise le modèle auquel l'alias [`fable` se résout](#fable-alias-resolution) où Fable est disponible pour vous, sinon le même modèle que `opus`                                                                                                                                                                                                                                         |
| **`fable`**      | Utilise le [modèle Fable pour votre fournisseur](#fable-alias-resolution) pour vos tâches les plus difficiles et les plus longues                                                                                                                                                                                                                                                        |
| **`sonnet`**     | Utilise le dernier modèle Sonnet pour les tâches de codage quotidiennes                                                                                                                                                                                                                                                                                                                  |
| **`opus`**       | Utilise le dernier modèle Opus pour les tâches de raisonnement complexe                                                                                                                                                                                                                                                                                                                  |
| **`haiku`**      | Utilise le modèle Haiku rapide et efficace pour les tâches simples                                                                                                                                                                                                                                                                                                                       |
| **`sonnet[1m]`** | Utilise Sonnet avec une [fenêtre de contexte de 1 million de jetons](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) pour les longues sessions. Aucun effet lorsque `sonnet` se résout déjà en Sonnet 5 avec sa fenêtre native de 1 M ; derrière une [passerelle LLM](/docs/fr/llm-gateway), sélectionne la fenêtre de 1 M pour Sonnet 5 |
| **`opus[1m]`**   | Utilise Opus avec une [fenêtre de contexte de 1 million de jetons](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) pour les longues sessions                                                                                                                                                                                        |
| **`opusplan`**   | Mode spécial qui utilise `opus` pendant le mode plan, puis bascule vers `sonnet` pour l'exécution                                                                                                                                                                                                                                                                                        |

La version vers laquelle les alias `opus` et `sonnet` se résolvent dépend du fournisseur :

| Fournisseur                                          | `opus`   | `sonnet`   |
| :--------------------------------------------------- | :------- | :--------- |
| API Anthropic                                        | Opus 5.5 | Sonnet 5   |
| [Claude Platform on AWS](/docs/fr/claude-platform-on-aws) | Opus 5.5 | Sonnet 4.6 |
| Amazon Bedrock, Agent Platform de Google Cloud       | Opus 5.5 | Sonnet 4.5 |
| Microsoft Foundry                                    | Opus 4.6 | Sonnet 4.5 |

<span id="fable-alias-resolution" />

Sauf si vous définissez `ANTHROPIC_DEFAULT_FABLE_MODEL`, l'alias `fable` se résout en Fable 5.1, sauf dans les sessions de [passerelle des applications Claude](/docs/fr/claude-apps-gateway), où `fable` et `best` se résolvent en Fable 5. Avant v2.1.257, `fable` se résolvait en Fable 5 sur chaque fournisseur.

Une passerelle qui n'est pas configurée pour servir `claude-fable-5-1` rejette les demandes pour ce modèle. Pour utiliser Fable 5.1 via une passerelle qui le sert, sélectionnez-le avec `/model claude-fable-5-1`.

Lorsqu'un alias se résout en un modèle plus ancien, les modèles plus récents sont disponibles en sélectionnant explicitement le nom de modèle complet ou en définissant `ANTHROPIC_DEFAULT_OPUS_MODEL` ou `ANTHROPIC_DEFAULT_SONNET_MODEL`.

Avant v2.1.280, `opus` se résolvait en Opus 5 sur l'API Anthropic, Claude Platform on AWS, Amazon Bedrock et Agent Platform de Google Cloud à partir de v2.1.219. Avant v2.1.219, `opus` se résolvait en Opus 4.8 sur l'API Anthropic à partir de v2.1.154, et sur Claude Platform on AWS, Amazon Bedrock et Agent Platform de Google Cloud à partir de v2.1.207. Avant v2.1.207, `opus` se résolvait en Opus 4.7 sur Claude Platform on AWS et en Opus 4.6 sur Amazon Bedrock et Agent Platform de Google Cloud.

Les alias pointent vers la version recommandée pour votre fournisseur et se mettent à jour au fil du temps. Pour épingler une version spécifique, utilisez le nom de modèle complet, par exemple `claude-opus-5-5`, ou définissez la variable d'environnement correspondante comme `ANTHROPIC_DEFAULT_OPUS_MODEL`.

<Note>
  Opus 5.5 nécessite Claude Code v2.1.280 ou version ultérieure. Opus 5 nécessite v2.1.219 ou version ultérieure. Sonnet 5 nécessite v2.1.197 ou version ultérieure. Exécutez `claude update` pour mettre à jour.
</Note>

<h3 id="work-with-fable">
  Travailler avec Fable
</h3>

[Claude Fable 5.1](https://platform.claude.com/docs/en/about-claude/models/overview) et Claude Fable 5 sont les modèles les plus puissants de Claude Code, adaptés aux tâches plus importantes qu'une seule séance. Ils maintiennent de longues sessions autonomes, enquêtent avant d'agir et vérifient leur travail plus souvent que les modèles plus petits. Fable 5.1 est la version plus récente.

Aucun des deux modèles Fable n'est la valeur par défaut du type de compte sur aucun plan ou fournisseur. Sélectionnez-en un explicitement :

* **Fable 5.1** : exécutez `/model fable`, ou lancez avec `claude --model fable`. Dans les sessions de [passerelle des applications Claude](/docs/fr/claude-apps-gateway), où l'alias se résout en Fable 5, exécutez `/model claude-fable-5-1` à la place.
* **Fable 5** : sélectionnez-le par ID de modèle. Sur l'API Anthropic, exécutez `/model claude-fable-5` ou lancez avec `claude --model claude-fable-5`. Sur d'autres fournisseurs, utilisez l'ID de modèle Fable 5 de votre fournisseur ou [épinglez-le](#pin-models-for-third-party-deployments) avec `ANTHROPIC_DEFAULT_FABLE_MODEL`.

Si vous vous connectez directement à l'API Anthropic et que vos paramètres utilisateur contiennent `claude-fable-5` ou `claude-fable-5[1m]` comme modèle, par exemple parce que vous avez sélectionné Fable dans le sélecteur `/model` avant v2.1.257, Claude Code change cette valeur enregistrée en alias `fable` ou `fable[1m]` la première fois que vous exécutez v2.1.257 ou version ultérieure. La ligne du modèle de démarrage affiche `(auto-updated)` une fois. Une valeur `claude-fable-5` dans les paramètres du projet, locaux ou gérés reste telle quelle.

Les demandes qu'un modèle Fable signale par ses classificateurs de sécurité, le plus souvent dans les domaines de la cybersécurité et de la biologie, déclenchent un [basculement automatique du modèle](#automatic-model-fallback).

Pour tirer le meilleur parti de Fable :

* **Décrivez le résultat, pas les étapes** : donnez-lui le résultat que vous voulez et laissez-le planifier le chemin. Pour le maintenir en travaillant vers ce résultat, [définissez un objectif](/docs/fr/goal).
* **Donnez-lui des problèmes ambigus** : les enquêtes sur les causes profondes, le débogage des pannes et les décisions architecturales sont les endroits où l'enquête et la vérification supplémentaires sont payantes.
* **Ignorez les rappels de vérification** : il vérifie son propre travail avec moins d'invites, donc les rappels de tester ou de vérifier sont généralement inutiles.
* **Dimensionnez les tâches plus importantes** : donnez-lui du travail que vous diviseriez normalement en morceaux. Il maintient de longues sessions sans perdre le fil.

<Note>
  Fable 5.1 nécessite Claude Code v2.1.257 ou version ultérieure. Si une demande pour celui-ci à partir d'une version plus ancienne échoue, consultez [Claude Code does not support this model](/docs/fr/errors#claude-code-does-not-support-this-model). Exécutez `claude update` pour mettre à jour. Pour la disponibilité en vertu de la rétention zéro des données, consultez [Model availability under ZDR](/docs/fr/zero-data-retention#model-availability-under-zdr).
</Note>

Sur l'API Anthropic, un modèle Fable apparaît dans le sélecteur `/model` sauf si [`availableModels`](#restrict-model-selection) ou [restrictions de modèle de l'organisation](#organization-model-restrictions) l'excluent. Lorsque votre organisation ne peut pas utiliser Fable du tout, par exemple en vertu de la [rétention zéro des données](/docs/fr/zero-data-retention#model-availability-under-zdr), la ligne reste dans le sélecteur grisée, avec une note sur la raison.

<h4 id="fable-and-usage-credits">
  Fable et crédits d'utilisation
</h4>

Selon votre plan et votre niveau de siège, l'utilisation de Fable peut être facturée aux [crédits d'utilisation](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) au lieu de puiser dans les limites incluses de votre plan. Lorsque c'est le cas, le sélecteur `/model` affiche « Requires usage credits » sur la ligne Fable. Pour gérer les crédits d'utilisation, consultez [Add usage credits to your subscription](/docs/fr/costs#add-usage-credits-to-your-subscription).

Dans les sessions interactives, Claude Code affiche une invite de consentement avant qu'une demande Fable ne facture les crédits d'utilisation. Les membres des plans Enterprise avec facturation organisationnelle ne voient pas l'invite. Vous pouvez continuer sur Fable en utilisant les crédits d'utilisation ou basculer vers votre modèle par défaut. Vous pouvez également ignorer l'invite :

* Dans le sélecteur `/model`, vous conservez votre modèle actuel.
* En cours de session, Claude Code continue le tour sur votre modèle par défaut.

Après avoir choisi de continuer sur Fable en utilisant les crédits d'utilisation, Claude Code n'affiche plus l'invite.

Dans une session avec [Remote Control](/docs/fr/remote-control) connecté, une [session en arrière-plan](/docs/fr/agent-view), ou une session d'un coéquipier d'une [équipe d'agents](/docs/fr/agent-teams), personne ne peut être au terminal, donc Claude Code maintient l'invite de consentement en cours de session jusqu'à la date limite [`dialogExpiry`](/docs/fr/settings-reference#dialogexpiry), cinq minutes par défaut. Si personne n'a répondu avant la date limite, Claude Code termine le tour sans envoyer la demande et ajoute un avis à la transcription, que le client Remote Control affiche également. Votre sélection de modèle est inchangée, et Claude Code demande à nouveau le consentement à votre prochain message.

Ce que vous pouvez faire pendant que l'invite attend dépend de la session :

* Avec Remote Control connecté ou dans une session d'un coéquipier, appuyez sur n'importe quelle touche au terminal pour annuler la date limite, et Claude Code attend votre réponse.
* Dans une session en arrière-plan, répondez avant la date limite.
* Si vous envoyez un nouveau message à partir du client distant avant que quelqu'un n'ait tapé au terminal, Claude Code termine le tour de la même manière, et votre nouveau message commence le tour suivant. Après que quelqu'un ait tapé au terminal, Claude Code continue d'attendre la réponse et met en file d'attente votre nouveau message derrière.

En [mode non interactif](/docs/fr/headless) avec l'indicateur `-p` et via le SDK Agent, Claude Code n'affiche jamais l'invite de consentement. Lorsqu'une demande Fable là-bas serait facturée aux crédits d'utilisation, Claude Code la facture sans demander.

<h3 id="setting-your-model">
  Définir votre modèle
</h3>

Vous pouvez configurer votre modèle de plusieurs façons, énumérées par ordre de priorité :

1. **Pendant la session** : utilisez `/model <alias|name>` pour basculer immédiatement, ou exécutez `/model` sans argument pour ouvrir le sélecteur. Consultez [when Claude Code asks you to confirm the switch](/docs/fr/prompt-caching#switching-models)
2. **Au démarrage** : lancez avec `claude --model <alias|name>`
3. **Variable d'environnement** : définissez `ANTHROPIC_MODEL=<alias|name>`
4. **Paramètres** : configurez de manière permanente dans votre fichier de paramètres en utilisant le champ `model`
5. **[Par défaut pour les nouvelles sessions](#set-a-default-model-for-new-sessions)** : définissez `ANTHROPIC_DEFAULT_MODEL=<alias|name>`

`/model` enregistre votre choix comme valeur par défaut pour les nouvelles sessions en écrivant le champ `model` dans vos paramètres utilisateur. Dans le sélecteur :

* `Enter` : basculer le modèle et enregistrer comme valeur par défaut
* `s` : basculer le modèle pour cette session uniquement et laisser votre valeur par défaut inchangée. Pour utiliser une clé différente, reliez [`modelPicker:thisSessionOnly`](/docs/fr/keybindings#model-picker-actions)

Taper `/model <name>` directement se comporte comme `Enter`. Pour basculer pour cette session uniquement, ouvrez le sélecteur avec `/model` et appuyez sur `s` sur la ligne du modèle.

Si vous changez de modèle avec `/model`, le changement atteint également les [sous-agents qui héritent du modèle de la conversation principale](/docs/fr/sub-agents#choose-a-model), car Claude Code résout leur modèle à partir de celui que votre session utilise lorsque Claude les démarre. Basculez vers Opus avant que Claude ne délègue la recherche ou les exécutions de test à l'un d'eux, et ce travail s'exécute également sur Opus. Pour garder un sous-agent personnalisé sur un modèle plus petit, définissez `model` dans sa définition.

Si vous définissez un modèle avec `/model` en [mode non interactif](/docs/fr/headless), avec l'indicateur `-p`, votre choix s'applique à la session actuelle uniquement et n'est pas enregistré comme valeur par défaut ; `/model` dans ce mode nécessite Claude Code v2.1.205 ou version ultérieure. Les paramètres du projet et gérés conservent toujours la priorité et se réappliquent au prochain lancement. Un [modèle par défaut de l'organisation](#organization-default-model) que votre administrateur a configuré pour remplacer la sélection de l'utilisateur se réapplique également au prochain lancement.

Dans v2.1.144 à v2.1.152, `/model` s'appliquait à la session actuelle uniquement et `d` dans le sélecteur enregistrait une valeur par défaut.

L'indicateur `--model` et la variable d'environnement `ANTHROPIC_MODEL` s'appliquent uniquement à la session que vous lancez avec eux. Pour exécuter différents modèles dans différents terminaux en même temps, lancez chacun avec son propre indicateur `--model` plutôt que de basculer avec `/model`.

Les prix dans le sélecteur `/model` apparaissent lorsque Claude Code communique avec l'API Anthropic, directement ou via une [passerelle LLM](/docs/fr/llm-gateway) qui la proxifie, et le prix sur une ligne est le prix du modèle que cette ligne sélectionne. Sur les [fournisseurs tiers](/docs/fr/third-party-integrations) tels qu'Amazon Bedrock et sur la [passerelle des applications Claude](/docs/fr/claude-apps-gateway), votre fournisseur ou passerelle détermine ce que vous payez, donc les lignes du sélecteur n'affichent aucun prix. Le prix est une étiquette d'affichage uniquement ; il n'affecte pas le modèle qu'une ligne sélectionne ou ce que votre fournisseur facture. Avant v2.1.206, [Claude Platform on AWS](/docs/fr/claude-platform-on-aws) et les sessions de passerelle affichaient les prix de liste d'Anthropic, et une ligne pouvait afficher le prix d'un modèle différent de celui qu'elle sélectionnait.

Les sessions reprises commencées avec `claude --resume`, `--continue`, ou le sélecteur `/resume` conservent le modèle qu'elles utilisaient lorsque la transcription a été enregistrée, indépendamment du paramètre `model` actuel. Si le modèle restauré a été retiré ou est exclu par [`availableModels`](#restrict-model-selection), la session revient à l'ordre de priorité normal. Cela empêche le choix `/model` d'une autre session de modifier le modèle à la reprise. Sur les fournisseurs qui utilisent des ID de déploiement spécifiques au fournisseur plutôt que des ID de modèle Anthropic, tels qu'Amazon Bedrock, Agent Platform de Google Cloud et Microsoft Foundry, le modèle de transcription n'est pas restauré du tout et la session résout son modèle via l'ordre de priorité normal.

Un modèle que vous choisissez pour le nouveau lancement avec `--model` ou `ANTHROPIC_MODEL` conserve toujours la priorité sur le modèle restauré. À partir de v2.1.195, une variable de la famille [`ANTHROPIC_DEFAULT_OPUS_MODEL`](#environment-variables) aussi. [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions) peut aussi, selon les conditions énumérées dans sa section.

Lorsque le modèle actif au démarrage provient des paramètres du projet ou gérés plutôt que de votre propre sélection, l'en-tête de démarrage indique quel fichier de paramètres l'a défini. Exécutez `/model` pour remplacer ; le paramètre du projet ou géré se réapplique au prochain lancement. Sur les plates-formes qui intègrent Claude Code et définissent [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/fr/env-vars), la configuration du modèle de l'hôte a priorité sur les paramètres du modèle géré, tandis qu'une liste d'autorisation `availableModels` gérée reste en vigueur sauf si l'hôte en fournit une ; [Exceptions to managed settings precedence](/docs/fr/settings#exceptions-to-managed-settings-precedence) indique quelles clés et variables l'hôte remplace.

Si vous ou votre organisation configurez des crochets [PreModelSwitch](/docs/fr/hooks#premodelswitch), ils s'exécutent avant qu'un changement demandé s'applique et peuvent le bloquer ou vous demander de confirmer.

Lorsque Claude Code ne peut pas déterminer quels crochets PreModelSwitch vos [plugins gérés](/docs/fr/settings-reference#enabledplugins) de l'organisation fournissent, par exemple parce qu'un plugin géré n'a pas pu se charger, il refuse le changement plutôt que de l'appliquer sans vérification, et il vérifie à nouveau à chaque nouvelle tentative. Consultez [Model switch was blocked by a PreModelSwitch hook](/docs/fr/errors#model-switch-was-blocked-by-a-premodelswitch-hook) pour le message et la récupération.

Lorsque vous changez de modèle via la méthode [`setModel()`](/docs/fr/agent-sdk/overview) du SDK Agent ou à partir d'un appareil connecté via [Remote Control](/docs/fr/remote-control), ou une application telle que l'[application de bureau](/docs/fr/desktop) qui exécute l'interface de ligne de commande Claude Code pour vous, Claude Code vérifie que la chaîne est une qu'il reconnaît avant de l'enregistrer. Cette vérification nécessite Claude Code v2.1.200 ou version ultérieure. Vérifier un choix Remote Control nécessite Claude Code v2.1.260 ou version ultérieure sur votre machine. Sur l'API Anthropic, Claude Code reconnaît :

* un alias de modèle
* une entrée du sélecteur `/model`
* tout nom qui commence par `claude-`
* une valeur que vous avez configurée vous-même comme [option de modèle personnalisé](#add-a-custom-model-option) ou dans [`modelOverrides`](#override-model-ids-per-version)

Claude Code rejette une chaîne non reconnue avec `Model "<name>" is not a recognized model id.` et la session conserve son modèle actuel, au lieu d'enregistrer la chaîne et d'échouer à la prochaine demande. Consultez [la référence d'erreur](/docs/fr/errors#model-is-not-a-recognized-model-id) pour les étapes de récupération.

La vérification s'exécute uniquement sur l'API Anthropic. Sur Amazon Bedrock, Agent Platform de Google Cloud, Microsoft Foundry, [Claude Platform on AWS](/docs/fr/claude-platform-on-aws), et derrière une [passerelle LLM](/docs/fr/llm-gateway) ou une `ANTHROPIC_BASE_URL` personnalisée, votre fournisseur ou passerelle définit les noms de modèles, donc Claude Code transmet n'importe quelle chaîne sans la vérifier. La vérification ne couvre pas non plus l'indicateur `--model`, la variable d'environnement `ANTHROPIC_MODEL`, ou le paramètre `model` ; une valeur mal tapée là-bas produit [There's an issue with the selected model](/docs/fr/errors#theres-an-issue-with-the-selected-model) à la première demande à la place. Claude Code peut toujours écrire la [ligne de diagnostic unrecognized-model](/docs/fr/errors#unrecognized-model-id-on-a-request) au moment de la demande, sur chaque fournisseur.

Lorsque le modèle demandé a une date de retraite programmée ou est automatiquement remappé à une version plus récente, Claude Code affiche un avertissement qui nomme le modèle demandé. Les sessions interactives l'affichent comme un avis de démarrage. À partir de v2.1.182, le même avertissement est écrit sur stderr en [mode non interactif](/docs/fr/headless) lors de l'utilisation du format de sortie texte par défaut. La vérification couvre également un `model` défini dans [frontmatter de sous-agent](/docs/fr/sub-agents). L'avertissement stderr est supprimé pour `--output-format json` et `stream-json` ; lisez le modèle réel à partir du champ `modelUsage` du [message de résultat](/docs/fr/headless#get-structured-output) à la place.

Par exemple, démarrez une session sur Opus :

```bash theme={null}
claude --model opus
```

Puis basculez les modèles au sein de la session :

```text theme={null}
/model sonnet
```

Exemple de fichier de paramètres :

```json theme={null}
{
    "permissions": {
        "allow": ["Bash(npm run lint)"]
    },
    "model": "opus"
}
```

<h4 id="set-a-default-model-for-new-sessions">
  Définir un modèle par défaut pour les nouvelles sessions
</h4>

Définissez `ANTHROPIC_DEFAULT_MODEL=<alias|name>` pour choisir le modèle sur lequel vos sessions démarrent par défaut. Nécessite Claude Code v2.1.236 ou version ultérieure.

Claude Code démarre une nouvelle session sur le modèle de la variable uniquement lorsqu'aucun de ceux-ci ne sélectionne un modèle :

* L'indicateur `--model`
* `ANTHROPIC_MODEL`
* Une valeur `model` dans n'importe quel fichier de paramètres, y compris le choix que vous enregistrez avec `/model`
* Un [modèle par défaut de l'organisation](#organization-default-model)

Un choix que vous enregistrez avec `/model` a priorité sur la variable aux lancements ultérieurs aussi. Avec `ANTHROPIC_MODEL` défini à la place, Claude Code revient au modèle de cette variable au prochain lancement, quel que soit ce que vous avez enregistré avec `/model`.

Claude Code résout également l'option Par défaut au modèle de la variable, sauf si un modèle par défaut de l'organisation s'applique. Lorsque l'option Par défaut se résout au modèle de la variable, la ligne Par défaut dans le sélecteur `/model` affiche l'étiquette Set by ANTHROPIC\_DEFAULT\_MODEL.

Claude Code ignore la variable dans ces cas, et l'option Par défaut se résout comme si vous ne l'aviez pas définie :

* Vous l'avez définie sur `default`, `inherit`, `opusplan`, ou `haiku`
* [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) est activé
* [`availableModels`](#restrict-model-selection) ou [restrictions de modèle de l'organisation](#organization-model-restrictions) excluent le modèle
* Le modèle n'est pas disponible pour votre compte

Lorsqu'une nouvelle session démarrerait sur le modèle de la variable, une session que vous reprenez avec `claude --resume`, `--continue`, ou le sélecteur `/resume` démarre également sur celui-ci. Claude Code ne restaure pas le modèle enregistré dans la transcription de cette session. Sinon, Claude Code n'utilise pas la variable lorsque vous [reprenez une session](#setting-your-model).

<h4 id="a-new-session-starts-on-a-different-model-than-you-picked">
  Une nouvelle session démarre sur un modèle différent de celui que vous avez choisi
</h4>

Lorsque vous choisissez un modèle avec `/model` et que votre prochaine session démarre sur quelque chose d'autre, ce sont les causes habituelles :

* **Vous l'avez choisi pour une session.** Appuyer sur `s` dans le sélecteur, lancer avec `--model`, et exécuter `/model` en mode non interactif s'appliquent tous à la session actuelle et laissent votre valeur par défaut enregistrée seule.
* **Quelque chose avec une priorité plus élevée définit le modèle.** Une valeur `model` dans les paramètres du projet ou gérés, `ANTHROPIC_MODEL` dans votre shell, ou un [modèle par défaut de l'organisation](#organization-default-model) que votre administrateur a défini pour remplacer les choix de l'utilisateur s'applique à nouveau à chaque lancement. Votre choix `/model` est toujours enregistré ; il est surclassé. Lorsque les paramètres du projet ou gérés définissent le modèle, l'en-tête de démarrage nomme le fichier.
* **Claude Code n'a pas pu enregistrer votre choix.** `/model` écrit `model` dans `~/.claude/settings.json`. Si vous ne pouvez pas écrire dans ce fichier, par exemple parce qu'un autre outil le génère ou le lie à une copie en lecture seule, le modèle que vous avez choisi dure pour la session et le prochain lancement lit l'ancienne valeur. Définissez `model` dans l'outil qui génère le fichier, ou rendez le fichier accessible en écriture. Consultez [A change you made in Claude Code is lost in new sessions](/docs/fr/settings#a-change-you-made-in-claude-code-is-lost-in-new-sessions).
* **Vous avez repris une session.** Une session que vous reprenez avec `claude --resume` ou `--continue` [conserve généralement le modèle qu'elle utilisait](#setting-your-model) plutôt que votre valeur par défaut actuelle.

<h2 id="restrict-model-selection">
  Restreindre la sélection de modèle
</h2>

Les administrateurs d'entreprise peuvent utiliser `availableModels` dans les [paramètres gérés ou de politique](/docs/fr/managed-settings) pour restreindre les modèles que les utilisateurs peuvent sélectionner. Les entrées correspondent à une famille de modèles telle que `sonnet`, un préfixe de version tel que `claude-sonnet-4-5`, ou un ID de modèle complet tel que `claude-sonnet-4-5-20250929`. Un préfixe de version correspond également aux ID de modèles ultérieurs qui l'étendent avec un autre segment, donc `claude-fable-5` permet à la fois Fable 5 et Fable 5.1, tandis que `claude-fable-5-1` permet uniquement Fable 5.1.

Sur les plateformes qui intègrent Claude Code et définissent [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/fr/env-vars), la configuration de modèle de l'hôte a priorité sur les paramètres de modèle gérés, tandis qu'une liste d'autorisation `availableModels` gérée reste en vigueur sauf si l'hôte fournit la sienne ; [Exceptions à la priorité des paramètres gérés](/docs/fr/settings#exceptions-to-managed-settings-precedence) indique quelles clés et variables l'hôte remplace.

Lorsque `availableModels` est défini, la liste d'autorisation s'applique partout où un utilisateur peut spécifier un modèle :

* **Modèle de session principale** : `/model`, l'indicateur `--model`, la variable d'environnement `ANTHROPIC_MODEL`, le paramètre `model`, [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions), et le modèle restauré lors de la [reprise d'une session](#setting-your-model)
* **Résolution d'alias** : les variables d'environnement `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, `ANTHROPIC_DEFAULT_HAIKU_MODEL`, et `ANTHROPIC_DEFAULT_FABLE_MODEL` ne peuvent pas rediriger un alias autorisé vers un modèle en dehors de la liste
* **Mode rapide** : `/fast` refuse de basculer lorsque cela changerait implicitement vers un modèle Opus en dehors de la liste, avec le message « is not in your organization's allowed models »
* **Modèles de sous-agent et de coéquipier** : le champ `model` dans la [frontmatter de sous-agent](/docs/fr/sub-agents#choose-a-model), le paramètre `model` de l'outil Agent, les [modèles de coéquipier d'équipe d'agent](/docs/fr/agent-teams#specify-teammates-and-models), `CLAUDE_CODE_SUBAGENT_MODEL`, et, sur v2.1.197 et antérieures, le sélecteur de modèle dans l'assistant `/agents`&#x20;
* **Modèles de compétence et de commande** : la frontmatter `model` dans les [compétences et commandes](/docs/fr/skills)
* **Modèle de conseiller** : le paramètre [`advisorModel`](/docs/fr/advisor) configuré et l'indicateur `--advisor`
* **Modèle d'agent en arrière-plan** : le modèle sélectionné dans le [sélecteur de dispatch](/docs/fr/agent-view)

Sur l'API Anthropic et [Claude Platform on AWS](/docs/fr/claude-platform-on-aws), un alias de famille de modèles, `opus`, `sonnet`, `haiku`, ou `fable`, se résout à son modèle habituel lorsque la liste d'autorisation permet ce modèle. Lorsque la liste d'autorisation bloque ce modèle, Claude Code substitue la version la plus récente de la famille que la liste d'autorisation permet et affiche un avis nommant à la fois les modèles demandés et substitués. Avec `["sonnet", "claude-opus-4-6"]`, par exemple, à la fois `/model opus` et `--model opus` sélectionnent Claude Opus 4.6, le plus récent Opus autorisé. Avant v2.1.205, un alias dont la version la plus récente publiée était en dehors de la liste était rejeté ou remplacé comme toute autre sélection bloquée, même lorsque la liste permettait une version plus ancienne.

La substitution a besoin d'une version autorisée sur laquelle atterrir : lorsque la liste d'autorisation ne permet aucune version de la famille de l'alias, l'alias suit le comportement de rejet et de remplacement ci-dessous comme toute autre valeur bloquée.

Claude Code gère toute autre sélection bloquée selon l'endroit où le modèle a été défini :

* **`/model`** : Claude Code rejette le changement avec une erreur
* **Indicateur `--model`, `ANTHROPIC_MODEL`, ou paramètre `model`** : Claude Code remplace la valeur au démarrage par un avis nommant à la fois les modèles demandés et substitués, et la session démarre sur le modèle par défaut
* **[`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions)** : Claude Code ignore la variable
* **Remplacement de sous-agent ou de coéquipier** : Claude Code exécute le sous-agent ou le coéquipier sur un modèle de secours plutôt que d'échouer la demande. Voir [Choisir un modèle](/docs/fr/sub-agents#choose-a-model) pour le secours du sous-agent et [Spécifier les coéquipiers et les modèles](/docs/fr/agent-teams#specify-teammates-and-models) pour le secours du coéquipier.

  Dans les sessions interactives, Claude Code vous avertit lorsqu'il substitue le modèle d'un sous-agent, par ce secours ou par la substitution de version la plus récente autorisée ci-dessus, nommant les modèles demandés et substitués ; il ne signale pas le secours d'un coéquipier.

  Lorsque la substitution de version la plus récente autorisée ci-dessus fonctionne, un alias de famille bloqué la suit à la place. Avant v2.1.222, un alias revenait comme toute autre valeur bloquée sur chaque fournisseur
* **Remplacement de compétence ou de commande** : Claude Code ignore le remplacement, y compris un alias de famille bloqué, et la compétence ou la commande s'exécute sur le modèle de session. Une compétence ou une commande qui [s'exécute dans un sous-agent](/docs/fr/skills#run-skills-in-a-subagent) suit le comportement du sous-agent ci-dessus à la place
* **Paramètre `advisorModel`** : le conseiller est désactivé pour la session
* **Indicateur `--advisor`** : Claude Code se termine avec une erreur au lancement. Dans une [session en arrière-plan](/docs/fr/agent-view), il démarre la session sans le conseiller au lieu de se terminer

Claude Code masque les modèles exclus du sélecteur `/model`. Un ID de modèle complet dans la liste qui n'a pas de ligne de sélecteur intégrée, tel qu'une version plus ancienne que la liste épingle, apparaît dans le sélecteur `/model` comme sa propre ligne étiquetée, sauf si Claude Code remplace les options intégrées par une [lineup `modelPicker`](/docs/fr/settings-reference#modelpicker). Avant v2.1.199, un tel ID n'était sélectionnable qu'en tapant `/model <id>`.

Les changements de modèle que Claude Code effectue en votre nom sont vérifiés de la même manière :

* **[Chaînes de modèle de secours](#fallback-model-chains)** : les entrées en dehors de la liste d'autorisation sont supprimées
* **Mises à niveau du mode plan** : sur l'API Anthropic et Claude Platform on AWS, une mise à niveau telle que [`opusplan`](#opusplan-model-setting) vers un modèle exclu utilise la version la plus récente autorisée de la famille de mise à niveau. Sur les fournisseurs avec des ID de modèle spécifiques au fournisseur, et lorsqu'aucune version n'est autorisée, la mise à niveau est ignorée et la planification continue sur le modèle de la session
* **[Secours de modèle automatique](#automatic-model-fallback)** : un secours dont la cible est exclue ne s'exécute pas, donc la demande signalée se termine par un refus à la place
* **[Classificateur du mode automatique](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode)** : la valeur par défaut Claude Sonnet 5 du classificateur s'applique uniquement lorsque la liste d'autorisation permet Sonnet 5. Lorsqu'il est exclu, le classificateur s'exécute sur le modèle de la session, que la liste d'autorisation gouverne déjà, ou sur un modèle Opus lorsque la session s'exécute sur un [modèle Fable](#work-with-fable). Sur les fournisseurs autres que l'API Anthropic, ce secours Opus s'exécute sur le modèle Opus par défaut du fournisseur sans consulter la liste d'autorisation. Nécessite Claude Code v2.1.210 ou ultérieur
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

| Mécanisme de livraison                                                                            | CLI et IDE | Sessions locales de bureau | Sessions web, mobile et cloud                                                                                                                                                                                                                                                                  | Agent SDK et non-interactif | Cowork              |
| :------------------------------------------------------------------------------------------------ | :--------- | :------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------- | :------------------ |
| [Paramètres gérés par le serveur](/docs/fr/server-managed-settings) depuis la console d'administration | Appliqué   | Appliqué                   | Appliqué                                                                                                                                                                                                                                                                                       | Appliqué                    | Non livré           |
| [Fichiers MDM ou paramètres gérés](/docs/fr/managed-settings#delivery-mechanisms)                      | Appliqué   | Appliqué                   | Non livré dans les environnements hébergés par Anthropic ; dans les [environnements auto-hébergés](/docs/fr/self-hosted-environments), appliqué à partir de l'image du runner selon [comment Claude Code combine les sources gérées](/docs/fr/managed-settings#how-claude-code-combines-managed-sources) | Appliqué                    | Appliqué où déployé |

* Les sessions cloud, sur [Claude Code sur le web](/docs/fr/claude-code-on-the-web) ou dans l'application de bureau, s'exécutent sur des machines virtuelles gérées par Anthropic par défaut : les paramètres déployés sur votre appareil ne les atteignent pas, donc livrez la liste d'autorisation via les paramètres gérés par le serveur. Les sessions que votre organisation achemine vers un [environnement auto-hébergé](/docs/fr/self-hosted-environments) s'exécutent sur votre propre calcul et lisent également le fichier de paramètres gérés dans l'image du runner. [Comment Claude Code combine les sources gérées](/docs/fr/managed-settings#how-claude-code-combines-managed-sources) indique quand ce fichier s'applique. Un changement de modèle en milieu de session dans une session cloud est rejeté lorsque le modèle demandé est exclu par la liste d'autorisation. Lorsque la liste `availableModels` dans vos paramètres gérés par le serveur est non vide, le serveur rejette la demande d'un utilisateur de démarrer une session cloud sur un modèle que la liste exclut.
* Cowork, l'onglet de travail agentique dans l'application Claude Desktop, exécute ses sessions sur Claude Code mais, par conception, ne reçoit pas les paramètres gérés par le serveur de la console d'administration claude.ai. Un fichier de paramètres gérés s'applique aux sessions Cowork lorsqu'il est présent où la session s'exécute ; les sessions Cowork distantes s'exécutent sur des machines virtuelles gérées par Anthropic, où un fichier déployé sur l'appareil n'est pas présent.
* Les sessions sur [fournisseurs tiers](/docs/fr/server-managed-settings#platform-availability) tels qu'Amazon Bedrock, Agent Platform de Google Cloud, Microsoft Foundry, et [Claude Platform on AWS](/docs/fr/claude-platform-on-aws) ne reçoivent pas les paramètres gérés par le serveur, donc livrez la liste d'autorisation via MDM ou des fichiers de paramètres gérés là-bas.
* La livraison gérée par le serveur nécessite également que la session s'authentifie avec une [connexion ou clé éligible](/docs/fr/server-managed-settings#platform-availability). Les flottes qui génèrent des clés uniquement via un script [`apiKeyHelper`](/docs/fr/settings-reference#apikeyhelper) doivent livrer la liste d'autorisation via MDM ou des fichiers de paramètres gérés.
* L'onglet Code du bureau héberge également les [sessions SSH](/docs/fr/desktop#ssh-sessions), qui lisent le fichier de paramètres gérés à partir de l'hôte distant sur lequel elles s'exécutent. Voir [Paramètres gérés du bureau](/docs/fr/desktop#managed-settings).
* Les sélecteurs de modèle sur claude.ai et dans l'application de bureau masquent ou grisent les modèles exclus par la liste d'autorisation de votre organisation. L'état du sélecteur est une commodité pour les utilisateurs ; il n'applique pas la liste d'autorisation.

<h3 id="default-model-behavior">
  Comportement du modèle par défaut
</h3>

En soi, `availableModels` laisse l'option Par défaut sur la [valeur par défaut d'exécution](#default-model-setting) du système pour le compte jusqu'à ce que vous définissiez également [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model). Si ce défaut est un modèle que vous avez l'intention de restreindre, définissez également `enforceAvailableModels`.

Un tableau `availableModels` vide n'engage jamais l'application de l'option Modèle par défaut : avec `availableModels: []`, les sélections de modèles nommés sont bloquées mais le modèle par défaut pour le type de compte reste utilisable quel que soit `enforceAvailableModels`.

<h3 id="enforce-the-allowlist-for-the-default-model">
  Appliquer la liste d'autorisation au modèle par défaut
</h3>

Définissez `enforceAvailableModels: true` aux côtés d'un `availableModels` non vide dans les paramètres gérés pour étendre la liste d'autorisation à l'option Par défaut. Cela nécessite Claude Code v2.1.175 ou ultérieur.

```json theme={null}
{
  "availableModels": ["sonnet", "haiku"],
  "enforceAvailableModels": true
}
```

L'option Par défaut se résout au défaut du type de compte, ou au [modèle par défaut d'organisation](#organization-default-model) lorsqu'un administrateur en a défini un. Lorsque ce modèle n'est pas dans la liste d'autorisation, l'option Par défaut se résout à la place à la première entrée `availableModels` qui nomme un modèle autorisé et disponible, et la ligne Par défaut du sélecteur `/model` affiche ce modèle. Cela s'applique partout où le défaut est atteint : démarrage de session, sélection de Par défaut dans `/model`, le mot-clé `"default"` dans les [chaînes de modèle de secours](#fallback-model-chains), et le secours utilisé lorsqu'une sélection exclue est supprimée.

`enforceAvailableModels` remapte l'option Par défaut uniquement lorsque `availableModels` est non vide. Avec `availableModels: []`, le modèle par défaut pour le type de compte reste utilisable, donc le paramètre ne peut pas verrouiller les utilisateurs hors de chaque modèle. Lorsque `availableModels` est non vide mais qu'aucune entrée ne se résout à un modèle autorisé et disponible, l'application est ignorée et Par défaut se résout au défaut du type de compte, avec un avertissement visible uniquement sous `--debug`. Gardez au moins une entrée garantie disponible dans la liste pour éviter cela.

Déployez les deux clés ensemble dans la source gérée la mieux classée que vous livrez. Par défaut, Claude Code ne lit que cette source, donc une paire placée dans un fichier de paramètres gérés est ignorée lorsque la console d'administration livre des paramètres ; sous la fusion opt-in dans [comment Claude Code combine les sources gérées](/docs/fr/managed-settings#how-claude-code-combines-managed-sources), Claude Code ignore toujours une carte `modelOverrides` d'une source classée en dessous de celle qui définit `availableModels`.

<h3 id="control-the-model-users-run-on">
  Contrôler le modèle sur lequel les utilisateurs s'exécutent
</h3>

Le paramètre `model` est une sélection initiale, pas une application. Il définit quel modèle est actif au démarrage d'une session, mais les utilisateurs peuvent toujours ouvrir `/model` et choisir Par défaut, qui se résout à la [valeur par défaut d'exécution](#default-model-setting) du système quel que soit ce qui est défini pour `model`, sauf si [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) le redirige.

Pour contrôler complètement l'expérience du modèle, combinez ces paramètres :

* **`availableModels`** : restreint les modèles nommés vers lesquels les utilisateurs peuvent basculer
* **`enforceAvailableModels`** : étend la liste d'autorisation `availableModels` à l'option Par défaut, donc Par défaut ne peut pas se résoudre à un modèle en dehors de la liste
* **`model`** : définit la sélection de modèle initiale au démarrage d'une session
* **`ANTHROPIC_DEFAULT_SONNET_MODEL`** / **`ANTHROPIC_DEFAULT_OPUS_MODEL`** / **`ANTHROPIC_DEFAULT_HAIKU_MODEL`** / **`ANTHROPIC_DEFAULT_FABLE_MODEL`** : contrôlez à quoi les alias `sonnet`, `opus`, `haiku`, et `fable` se résolvent, et quelle version la [valeur par défaut du type de compte](#default-model-setting) utilise

Cet exemple démarre les utilisateurs sur Sonnet 4.5, limite le sélecteur à Sonnet et Haiku, et garantit que Par défaut se résout à un modèle sur la liste d'autorisation plutôt que la valeur par défaut du niveau :

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

Sans `enforceAvailableModels` ou le bloc `env`, un utilisateur qui sélectionne Par défaut dans le sélecteur obtient la [valeur par défaut d'exécution](#default-model-setting) plutôt que la version épinglée dans `model`. Les deux paramètres couvrent des portées différentes : `enforceAvailableModels` fait que Par défaut obéit à la liste d'autorisation, tandis que le bloc `env` épingle quelle version un alias autorisé tel que `sonnet` se résout. Utilisez `enforceAvailableModels` seul lorsque restreindre les familles de modèles est suffisant ; ajoutez le bloc `env` lorsque vous avez également besoin d'épingler une version spécifique.

<h3 id="merge-behavior">
  Comportement de fusion
</h3>

Lorsque les paramètres gérés que Claude Code applique définissent `availableModels`, cette liste seule s'applique, à part une [plateforme hôte qui fournit la sienne](/docs/fr/settings#exceptions-to-managed-settings-precedence) : les entrées dans les paramètres utilisateur, projet ou local ne peuvent pas l'étendre, et Claude Code ne fusionne jamais `availableModels` entre les sources gérées non plus ; [comment Claude Code combine les sources gérées](/docs/fr/managed-settings#how-claude-code-combines-managed-sources) indique quelle liste de source s'applique. Sinon, les listes des paramètres utilisateur, projet et local sont [concaténées et dédupliquées](/docs/fr/settings#settings-precedence) comme d'autres paramètres de tableau. Avant Claude Code v2.1.175, les entrées des portées de priorité inférieure fusionnaient dans la liste gérée au lieu d'être remplacées par elle.

Dans la liste effective, une entrée nommant un modèle spécifique dans une famille, qu'il s'agisse d'un préfixe de version ou d'un ID de modèle complet, désactive l'entrée de caractère générique de cette famille : `["sonnet", "claude-sonnet-4-5"]` permet uniquement les versions Sonnet 4.5, pas tous les modèles Sonnet.

<h3 id="mantle-model-ids">
  ID de modèle Mantle
</h3>

Lorsque le [point de terminaison Amazon Bedrock Mantle](/docs/fr/amazon-bedrock#use-the-mantle-endpoint) est activé, les entrées dans `availableModels` qui commencent par `anthropic.` sont ajoutées au sélecteur `/model` comme options personnalisées et acheminées vers le point de terminaison Mantle. Ceci est une exception à la correspondance d'alias décrite dans [Épingler les modèles pour les déploiements tiers](#pin-models-for-third-party-deployments). Le paramètre restreint toujours le sélecteur aux entrées listées, et un ID Mantle intègre un nom de famille, donc il compte comme une entrée spécifique et désactive le caractère générique de cette famille : aux côtés de tous les ID Mantle, listez les préfixes de version ou les ID complets que vous souhaitez garder sélectionnables. Voir [Comportement de fusion](#merge-behavior).

<h3 id="organization-model-restrictions">
  Restrictions de modèle d'organisation
</h3>

Les administrateurs d'organisation sur les plans Claude Enterprise restreignent les modèles que les membres peuvent exécuter en désactivant les modèles individuels dans la console d'administration claude.ai. Cette restriction est livrée avec les droits du compte lorsque Claude Code s'authentifie, séparé de toute liste `availableModels` dans les paramètres, et le serveur applique la même restriction indépendamment lorsqu'une session est créée. Nécessite Claude Code v2.1.187 ou ultérieur.

La restriction s'applique lorsqu'un membre se connecte ou utilise sa propre clé API. Les identifiants d'organisation, tels que les clés de service d'organisation, ne sont pas liés à un utilisateur, donc la restriction ne s'applique pas à eux.

La console Claude n'a pas de contrôle de restriction de modèle. Les organisations sans plan Claude Enterprise, y compris celles dont les membres s'authentifient via l'API Anthropic, restreignent les modèles avec [`availableModels`](#restrict-model-selection) dans les [paramètres gérés](/docs/fr/managed-settings) à la place, en ajoutant [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) pour couvrir l'option Par défaut. [Couverture de surface](#surface-coverage) indique comment chaque surface reçoit et applique ces paramètres.

Un modèle restreint est masqué du sélecteur `/model`. Le sélectionner par nom avec `--model`, la variable d'environnement `ANTHROPIC_MODEL`, ou le paramètre `model` affiche l'avis `Model "<name>" is restricted by your organization's settings. Using <model> instead.` et la session démarre sur un modèle autorisé. Taper `/model <name>` pour un modèle restreint est rejeté avec `Model '<name>' is restricted by your organization's settings. Run /model to choose a different model.` et la session garde son modèle actuel.

Un [alias de famille de modèles](#restrict-model-selection) tel que `opus` se résout à son modèle habituel lorsque l'organisation le permet. Lorsque l'organisation restreint ce modèle, Claude Code substitue la version la plus récente de la famille que l'organisation permet, avec le même avis de substitution. `/model <alias>` est rejeté uniquement lorsque chaque version de sa famille est restreinte ; un alias défini avec `--model`, `ANTHROPIC_MODEL`, ou le paramètre `model` est toujours remplacé au démarrage dans ce cas. Avant v2.1.205, un alias de famille était substitué ou rejeté en fonction de sa version la plus récente publiée seule, même lorsqu'une version plus ancienne était autorisée.

Les restrictions s'appliquent à l'échelle de l'organisation ou par rôle :

* La désactivation d'un modèle au niveau de l'organisation le supprime pour chaque membre.
* L'accès au niveau du rôle accorde différents modèles à différents rôles personnalisés, et un membre qui détient plusieurs rôles peut utiliser n'importe quel modèle qu'un de ses rôles accorde.
* Les modèles Haiku sont toujours disponibles et ne peuvent pas être désactivés, donc chaque membre garde au moins un modèle utilisable.
* Un changement d'accès prend effet sur les nouvelles demandes dans environ une minute ; le sélecteur `/model` le reflète la prochaine fois qu'une session démarre.

Les deux restrictions s'appliquent ensemble : un modèle n'est sélectionnable que lorsqu'il est autorisé par `availableModels` et non restreint par l'organisation. Les restrictions d'organisation atteignent les sessions sur l'API Anthropic et les déploiements de [passerelle LLM](/docs/fr/llm-gateway) uniquement ; sur tout autre fournisseur, utilisez `availableModels` à la place.

<h2 id="organization-default-model">
  Modèle par défaut de l'organisation
</h2>

Les administrateurs d'organisation sur les plans Claude Enterprise peuvent définir un modèle par défaut pour les membres de Claude Code à partir de la console d'administration claude.ai, pour l'ensemble de l'organisation ou par rôle personnalisé. Lorsqu'un modèle est défini, l'option Par défaut se résout à ce modèle. Nécessite Claude Code v2.1.196 ou version ultérieure.

La ligne Par défaut dans le sélecteur `/model` affiche le nom du modèle par défaut de l'organisation avec l'étiquette Org default. L'étiquette indique Org default que l'administrateur ait défini le modèle par défaut pour l'ensemble de l'organisation ou pour votre rôle. Un modèle par défaut de rôle s'applique aux membres de ce rôle personnalisé et prend précédence sur le modèle par défaut à l'échelle de l'organisation ; lorsque plusieurs de vos rôles définissent des modèles par défaut différents, le modèle le plus capable s'applique.

Le modèle par défaut de l'organisation est un point de départ, pas une restriction. Ces sélections prennent précédence sur celui-ci :

* le drapeau `--model` et la variable d'environnement `ANTHROPIC_MODEL`
* une valeur `model` dans les [paramètres gérés](/docs/fr/managed-settings) ou fournie via `--settings`
* une valeur `model` dans vos paramètres utilisateur, projet ou locaux, y compris un modèle que vous enregistrez avec `/model`

Les administrateurs peuvent également configurer le modèle par défaut de l'organisation pour remplacer la sélection de l'utilisateur. Avec le remplacement activé, il prend précédence sur la valeur `model` dans les paramètres utilisateur, projet et locaux, de sorte qu'un modèle que vous enregistrez avec `/model` s'applique pour la session actuelle et le modèle par défaut de l'organisation revient au lancement suivant. Lorsque votre sélection diffère, `/model` affiche `Your organization's default (<model>) applies on restart`. Le drapeau `--model`, `ANTHROPIC_MODEL`, les paramètres gérés et `--settings` prennent toujours précédence même avec le remplacement activé. Le remplacement est disponible pour un ensemble limité d'organisations ; contactez votre équipe de compte Anthropic pour connaître la disponibilité.

Pour limiter les modèles que les membres peuvent sélectionner, utilisez plutôt les [restrictions de modèle de l'organisation](#organization-model-restrictions) ou [`availableModels`](#restrict-model-selection).

Claude Code lit le modèle par défaut de l'organisation une seule fois au démarrage, de sorte qu'un modèle par défaut que l'administrateur modifie en cours de session prend effet au lancement suivant.

Lorsque le modèle par défaut de l'organisation ne remplace pas la sélection de l'utilisateur, le premier lancement interactif après que l'administrateur le modifie efface la clé `model` de vos paramètres utilisateur une seule fois, de sorte que le nouveau modèle par défaut s'applique. Il ne change rien d'autre dans le fichier, et un modèle que vous enregistrez avec `/model` après ce lancement est conservé.

Le modèle par défaut de l'organisation passe par ces vérifications de restriction avant d'être adopté :

* [`availableModels`](#restrict-model-selection) seul ne s'applique pas au modèle par défaut de l'organisation, de sorte qu'un modèle par défaut de l'organisation en dehors de la liste d'autorisation s'applique toujours. Lorsque [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) est également défini, un modèle par défaut de l'organisation en dehors de la liste d'autorisation est remappé à la première entrée de la liste d'autorisation, comme tout autre modèle Par défaut
* un modèle par défaut de l'organisation que les [restrictions de modèle de l'organisation](#organization-model-restrictions) refusent pour votre compte est remplacé par le modèle le plus récent autorisé dans sa famille, ou une famille moins coûteuse lorsque chaque version de celle-ci est restreinte
* un modèle par défaut de l'organisation qui n'est pas disponible pour votre compte du tout est ignoré, et l'option Par défaut se résout comme elle le ferait [sans modèle par défaut de l'organisation](#default-model-setting)

À partir de la v2.1.199, lorsque le modèle par défaut de l'organisation est une famille de modèles différente du modèle par défaut habituel du type de votre compte, le sélecteur `/model` conserve une ligne séparée pour cette famille habituelle, de sorte que vous pouvez toujours basculer vers celle-ci pour une session. Dans les versions v2.1.196 à v2.1.198, cette ligne est absente du sélecteur.

Le modèle par défaut de l'organisation ne s'applique qu'aux sessions authentifiées avec l'API Anthropic. Pour définir un modèle par défaut ailleurs, y compris dans les déploiements de [passerelle LLM](/docs/fr/llm-gateway), utilisez plutôt la clé `model` dans les [paramètres gérés](/docs/fr/managed-settings).

<h2 id="organization-effort-limits">
  Limites d'effort au niveau de l'organisation
</h2>

Votre organisation peut plafonner le [niveau d'effort](#adjust-effort-level) de deux façons. Sur un plan Claude Enterprise, les administrateurs d'organisation définissent des limites d'effort par rôle, décrites ci-dessous. Sur n'importe quel plan et n'importe quel fournisseur, y compris Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry, le paramètre géré [`maxEffortLevel`](/docs/fr/settings-reference#maxeffortlevel) plafonne l'effort côté client. Lorsque les deux s'appliquent à un modèle, le plafond inférieur s'applique.

Les administrateurs d'organisation sur les plans Claude Enterprise peuvent définir un [niveau d'effort](#adjust-effort-level) maximum par modèle pour chaque rôle personnalisé, aux côtés des [restrictions de modèle au niveau de l'organisation](#organization-model-restrictions). Les niveaux au-dessus du plafond ne sont pas proposés dans le sélecteur `/effort`, et nommer un niveau supérieur avec `--effort` ou `/effort` s'exécute au plafond à la place. Dans les sessions interactives et les exécutions en texte brut `--print`, un avertissement nomme les niveaux demandés et appliqués ; avec une sortie `json` ou `stream-json` ou dans les agents en arrière-plan, le plafonnement s'applique silencieusement. Les plafonds sont par modèle, donc changer de modèle peut modifier les niveaux disponibles. Lorsque plusieurs de vos rôles accordent le même modèle, le plafond le moins restrictif s'applique. Nécessite Claude Code v2.1.195 ou version ultérieure.

Les limites d'effort sont livrées avec les [restrictions de modèle au niveau de l'organisation](#organization-model-restrictions) et atteignent les mêmes sessions.

<h2 id="special-model-behavior">
  Comportement spécial des modèles
</h2>

<h3 id="default-model-setting">
  Paramètre de modèle `default`
</h3>

Le comportement de `default` dépend de votre type de compte :

* **Pro, Max, Team, Enterprise et API Anthropic** : par défaut Opus 5.5
* **Claude Platform sur AWS, Amazon Bedrock et Agent Platform de Google Cloud** : par défaut Opus 5.5
* **Microsoft Foundry** : par défaut Sonnet 4.5

Avant la v2.1.280, `default` se résolvait en Sonnet 5 sur Pro et Team Standard, et en Opus 5 sur Max, Team Premium, Enterprise, l'API Anthropic, Claude Platform sur AWS, Amazon Bedrock et Agent Platform de Google Cloud à partir de la v2.1.219. Avant la v2.1.219, `default` se résolvait en Opus 4.8 sur l'API Anthropic, Max, Team Premium et Enterprise avec paiement à l'usage à partir de la v2.1.154, et sur Claude Platform sur AWS, Amazon Bedrock et Agent Platform de Google Cloud à partir de la v2.1.207. Avant la v2.1.207, `default` se résolvait en Opus 4.7 sur Claude Platform sur AWS et en Sonnet 4.5 sur Amazon Bedrock et Agent Platform de Google Cloud.

Quand un administrateur a défini un [modèle par défaut de l'organisation](#organization-default-model), `default` se résout en ce modèle au lieu du modèle par défaut du type de compte ci-dessus. Nécessite Claude Code v2.1.196 ou ultérieure. `default` peut également se résoudre en le modèle que vous avez défini avec [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions), selon les conditions énumérées dans sa section.

Quand les paramètres gérés [appliquent la liste d'autorisation pour le modèle par défaut](#enforce-the-allowlist-for-the-default-model) et que le modèle par défaut du type de compte ne figure pas dans `availableModels`, `default` se résout en Default appliqué au lieu du modèle par défaut du type de compte ci-dessus. Quand les deux s'appliquent, le modèle par défaut de l'organisation remplace d'abord le modèle par défaut du type de compte, puis l'application s'y applique : un modèle par défaut de l'organisation autorisé est conservé, tandis qu'un modèle en dehors de la liste se résout en Default appliqué.

Les modèles Fable ne sont le modèle par défaut du type de compte sur aucun plan ou fournisseur. En choisir un avec `/model` l'enregistre comme modèle sélectionné dans vos paramètres utilisateur, de sorte que les sessions ultérieures commencent dessus. Pour le changement unique que Claude Code apporte à une sélection Fable 5 enregistrée dans la v2.1.257, voir [Travailler avec Fable](#work-with-fable).

<h3 id="opusplan-model-setting">
  Paramètre de modèle `opusplan`
</h3>

L'alias de modèle `opusplan` fournit une approche hybride automatisée :

* **En mode plan** : utilise `opus` pour le raisonnement complexe et les décisions architecturales
* **En mode exécution** : bascule automatiquement vers `sonnet` pour la génération de code et l'implémentation

Cela associe le raisonnement d'Opus pour la planification à l'efficacité de Sonnet pour l'exécution.

La phase Opus en mode plan utilise la même fenêtre de contexte que le paramètre de modèle `opus`, et la phase d'exécution utilise la même fenêtre que `sonnet`. Quand `opus` et `sonnet` se résolvent en modèles qui s'exécutent avec la [fenêtre de contexte de 1M](#extended-context) par défaut, comme le font les modèles actuels sur l'API Anthropic, les deux phases s'exécutent avec elle. Pour demander un contexte de 1M pour les deux phases où ils ne le font pas, [définissez le modèle](#setting-your-model) sur `opusplan[1m]`, par exemple avec `/model opusplan[1m]`. Le définir avec `/model` nécessite Claude Code v2.1.265 ou ultérieure ; sur les versions antérieures, utilisez l'indicateur `--model` ou le paramètre `model` à la place.

Quand [`availableModels`](#restrict-model-selection) exclut le plus récent Opus mais permet une version antérieure, par exemple `["sonnet", "claude-opus-4-6"]`, `opusplan` utilise le plus récent Opus autorisé pour la planification et reste sur Sonnet uniquement quand chaque Opus est exclu. Une session Haiku qui se mettrait normalement à niveau vers Sonnet en mode plan utilise de même le plus récent Sonnet autorisé, et reste sur Haiku uniquement quand chaque Sonnet est exclu. Avant la v2.1.205, le mode plan restait sur le modèle de la session chaque fois que la version la plus récente de la famille de mise à niveau était exclue, même quand la liste d'autorisation permettait une version antérieure.

La substitution d'une version antérieure autorisée s'applique sur l'API Anthropic et [Claude Platform sur AWS](/docs/fr/claude-platform-on-aws). Sur Amazon Bedrock, Agent Platform de Google Cloud, Microsoft Foundry et Mantle, dont les déploiements utilisent des ID de modèle spécifiques au fournisseur, le mode plan reste sur le modèle de la session chaque fois que le modèle de mise à niveau est exclu.

Pour une approche hybride où Claude décide au milieu d'une tâche quand consulter un deuxième modèle plutôt que de basculer à la limite du plan, voir l'[outil advisor](/docs/fr/advisor).

<h3 id="fallback-model-chains">
  Chaînes de modèles de secours
</h3>

Quand le modèle principal est surchargé, indisponible ou retourne une autre erreur serveur non renouvelable, Claude Code peut basculer vers un modèle de secours au lieu d'échouer la demande. Les erreurs d'authentification, de facturation, de limite de débit, de taille de demande et de transport, et un [refus par la vérification de politique de votre organisation](/docs/fr/errors#automatic-retries), ne déclenchent jamais un basculement ; ceux-ci suivent leur gestion normale des tentatives et des erreurs.

Configurez un ou plusieurs modèles de secours et Claude Code les essaie dans l'ordre, affichant un avis quand il bascule. Le basculement dure uniquement pour le tour actuel, de sorte que votre prochain message essaie d'abord le modèle principal à nouveau. Claude Code limite les chaînes à trois modèles après suppression des doublons et ignore les entrées supplémentaires.

Définissez une chaîne pour une session avec l'indicateur `--fallback-model`, qui accepte une liste séparée par des virgules :

```bash theme={null}
claude --fallback-model sonnet,haiku
```

Pour persister une chaîne entre les sessions, définissez `fallbackModel` dans [paramètres](/docs/fr/settings) comme un tableau :

```json theme={null}
{
  "fallbackModel": ["claude-sonnet-5", "claude-haiku-4-5"]
}
```

L'indicateur `--fallback-model` a la priorité sur le paramètre `fallbackModel`. Chaque entrée accepte un nom de modèle ou un alias, et `"default"` se développe en le modèle par défaut.

Claude Code ne confirme pas la chaîne au démarrage et `/status` ne l'affiche pas. L'avis affiché quand un basculement se produit est le premier signe visible qu'un secours est configuré.

Quand une demande bascule, Claude Code essaie chaque entrée dans l'ordre jusqu'à ce que l'une l'accepte. Une entrée qui ne peut pas être atteinte non plus, comme un modèle retiré épinglé dans les paramètres, bascule vers la suivante de la même manière. Claude Code supprime deux types d'entrée avant cette traversée :

* **En dehors de la liste d'autorisation** : Claude Code supprime toute entrée non autorisée par [`availableModels`](#restrict-model-selection) quand il lit la chaîne.
* **Fenêtre de contexte plus petite lors de la compaction** : la chaîne couvre également la [compaction](/docs/fr/context-window#what-survives-compaction), mais Claude Code ne basculera pas vers un modèle avec une fenêtre de contexte plus petite que celle du modèle principal, car résumer là couperait d'abord une partie de la conversation. Si chaque secours est plus petit, la compaction affiche l'erreur d'origine et vous pouvez réessayer.

Claude Code applique également la chaîne aux [sous-agents](/docs/fr/sub-agents). Quand la demande d'un sous-agent bascule, Claude Code essaie vos modèles de secours configurés dans l'ordre, et le sous-agent continue sur le modèle qui accepte la demande. Le modèle de votre session reste inchangé. Avant la v2.1.247, un échec que la chaîne couvre terminait le sous-agent à la place.

<h3 id="automatic-model-fallback">
  Secours automatique du modèle
</h3>

Cette section couvre le secours basé sur le contenu des modèles Fable, Opus 5.5 et Opus 5. Pour le secours basé sur la disponibilité quand un modèle est surchargé ou indisponible, voir [Chaînes de modèles de secours](#fallback-model-chains).

Les modèles Fable, Opus 5.5 et Opus 5 s'exécutent avec des classificateurs de sécurité, qui signalent le plus souvent le contenu de cybersécurité et de biologie. Quand un classificateur signale une demande et que la catégorie signalée a un modèle de secours, Claude Code réexécute la demande sur ce modèle et affiche un avis dans la transcription. Pour ces deux catégories, le modèle de secours dépend du modèle qui a refusé :

* **Fable 5.1, Fable 5 et Opus 5.5** : les demandes signalées pour biologie se réexécutent sur Opus 5, et les demandes signalées pour cybersécurité se réexécutent sur Opus 4.8.
* **Opus 5** : les demandes signalées pour cybersécurité se réexécutent sur Opus 4.8. Les demandes signalées pour biologie se terminent par un refus à la place, car Opus 5 exécute ses propres classificateurs de biologie sans modèle de secours.

Sur Amazon Bedrock, Agent Platform de Google Cloud et Microsoft Foundry, Claude Code résout ces cibles via votre déploiement à la place, et si vous définissez `ANTHROPIC_DEFAULT_OPUS_MODEL`, les catégories qui ont un secours se réexécutent sur le modèle épinglé ; voir [Activer le secours sur Bedrock, Agent Platform et Foundry](#enable-fallback-on-bedrock-agent-platform-and-foundry).

Après un secours, la session continue sur le modèle de secours. Pour revenir à votre modèle d'origine, exécutez [`/model`](#setting-your-model).

Le secours basé sur les catégories nécessite Claude Code v2.1.219 ou ultérieure. Avant la v2.1.219, chaque demande Fable 5 signalée se réexécutait sur le modèle Opus par défaut de votre fournisseur, et Opus 5 n'était pas une source de secours.

Le modèle de secours est vérifié par rapport à [`availableModels`](#restrict-model-selection). Quand il est bloqué, aucun secours ne se produit. Le refus est affiché comme une erreur normale et le modèle de la session reste inchangé.

<h4 id="check-what-triggered-fallback">
  Vérifier ce qui a déclenché le secours
</h4>

Le secours peut se déclencher à la première demande d'une session, avant que vous n'envoyiez quelque chose d'inhabituel, car la première demande porte le contexte de l'espace de travail comme votre contenu CLAUDE.md et l'état git. Un référentiel qui contient du matériel de sécurité ou de biologie peut déclencher le classificateur sur ce contexte seul.

Pour vérifier si les personnalisations sont le déclencheur, démarrez une session avec `claude --safe-mode`, qui désactive les personnalisations comme CLAUDE.md, les skills, les serveurs MCP et les hooks. L'état git et les noms de répertoires ne sont pas des personnalisations et sont toujours inclus.

<h4 id="ask-before-switching">
  Demander avant de basculer
</h4>

Pour décider ce qui se passe chaque fois qu'une demande est signalée, plutôt que de basculer automatiquement, exécutez `/config` et désactivez **Basculer les modèles quand un message est signalé**, ou définissez [`switchModelsOnFlag`](/docs/fr/settings-reference#switchmodelsonflag) sur `false` dans votre fichier de paramètres. Une demande signalée met alors la session en pause avec deux options : basculer vers le modèle de secours, ou modifier l'invite et réessayer sur le modèle actuel.

Certains cas se comportent différemment :

* Quand la catégorie signalée n'a pas de modèle de secours, comme un signal de biologie sur Opus 5, Claude Code n'affiche pas l'invite et la demande se termine par le refus.
* Si les deux modèles signalent la même demande, vous pouvez modifier l'invite et réessayer, ou démarrer une nouvelle session.
* Sur les sessions mobiles [Claude Code sur le web](/docs/fr/claude-code-on-the-web), la modification et la nouvelle tentative ne sont pas prises en charge. Basculez les modèles, ou continuez la session à partir d'un navigateur de bureau ou de l'application de bureau.
* En [mode non interactif](/docs/fr/cli-reference#cli-flags) et les intégrations SDK qui ne peuvent pas afficher l'invite, une demande signalée termine le tour par un refus à la place.
* Quand la cible de secours est bloquée par [`availableModels`](#restrict-model-selection), Claude Code n'affiche pas l'invite. La demande signalée se termine par le refus, de la même manière que le secours automatique quand la cible est bloquée.

<h4 id="enable-fallback-on-bedrock-agent-platform-and-foundry">
  Activer le secours sur Bedrock, Agent Platform et Foundry
</h4>

Sur [Amazon Bedrock](/docs/fr/amazon-bedrock), [Agent Platform de Google Cloud](/docs/fr/google-vertex-ai) et [Microsoft Foundry](/docs/fr/microsoft-foundry), les ID de modèle sont spécifiques au fournisseur, de sorte que le secours automatique ne fonctionne que quand Claude Code peut identifier les deux modèles impliqués :

* Claude Code doit reconnaître le modèle actuel comme une source de secours. Fable 5.1 et Fable 5 sont reconnus quand l'ID de modèle contient `claude-fable-5`, correspond à la valeur de `ANTHROPIC_DEFAULT_FABLE_MODEL`, ou est mappé avec [`modelOverrides`](#override-model-ids-per-version). Opus 5.5 et Opus 5 sont reconnus par leur ID de modèle du fournisseur ou un mappage [`modelOverrides`](#override-model-ids-per-version).
* Le modèle de secours doit se résoudre dans votre déploiement. Si vous définissez `ANTHROPIC_DEFAULT_OPUS_MODEL`, les demandes signalées se réexécutent sur ce modèle pour chaque catégorie qui a un secours ; un signal de biologie sur Opus 5 se termine toujours par un refus. Si vous ne le définissez pas, les demandes signalées pour cybersécurité se réexécutent sur une entrée Opus 4.8 dans la liste des modèles du fournisseur, et les demandes signalées pour biologie d'un modèle Fable ou Opus 5.5 sur une entrée Opus 5.

Si l'un ou l'autre modèle ne peut pas être identifié, Claude Code ne bascule pas automatiquement. La demande signalée se termine par un message de refus, et vous pouvez basculer les modèles avec [`/model`](#setting-your-model) et réessayer. Définir `ANTHROPIC_DEFAULT_FABLE_MODEL` sur votre ID de modèle Fable active la reconnaissance Fable. Définir `ANTHROPIC_DEFAULT_OPUS_MODEL` sur un ID de modèle Opus donne aux catégories signalées une cible de secours, sauf si l'épingle nomme un modèle en dehors de la famille Opus ou le modèle qui a refusé ; alors Claude Code ne bascule pas et le refus tient.

<h4 id="security-research-and-biology-workloads">
  Recherche en sécurité et charges de travail en biologie
</h4>

Les charges de travail en sécurité offensive ou en biologie, y compris les tests de pénétration, les exercices Capture the Flag (CTF) et les bases de code adjacentes à la biologie, déclenchent fréquemment le secours, souvent à la première demande. Pour un travail substantiel en biologie sur Fable 5.1, Fable 5 ou Opus 5.5, Claude Code déplace la session vers Opus 5 à la première demande signalée, et les demandes signalées pour biologie ultérieures se terminent par des refus là, car Opus 5 n'a pas de secours pour la biologie. Sur Opus 5, vous obtenez ces refus à partir de la première demande signalée.

C'est un routage attendu pour ces domaines, pas un signal de compte. Si votre organisation a besoin de la capacité de classe Fable pour ce travail, demandez à votre équipe de compte Anthropic les programmes d'accès de confiance.

<h3 id="adjust-effort-level">
  Ajuster le niveau d'effort
</h3>

Les [niveaux d'effort](https://platform.claude.com/docs/en/build-with-claude/effort) contrôlent le raisonnement adaptatif, qui permet au modèle de décider si et combien penser à chaque étape en fonction de la complexité de la tâche. Un effort inférieur est plus rapide et moins cher pour les tâches simples, tandis qu'un effort supérieur fournit un raisonnement plus profond pour les problèmes complexes.

Les niveaux d'effort disponibles dépendent du modèle. Les modèles non énumérés ici ne supportent pas l'effort :

| Modèle                                           | Niveaux                                 |
| :----------------------------------------------- | :-------------------------------------- |
| Fable 5.1 et Fable 5                             | `low`, `medium`, `high`, `xhigh`, `max` |
| Opus 5.5, Opus 5, Sonnet 5, Opus 4.8 et Opus 4.7 | `low`, `medium`, `high`, `xhigh`, `max` |
| Opus 4.6 et Sonnet 4.6                           | `low`, `medium`, `high`, `max`          |

Si vous définissez un niveau que le modèle actif ne supporte pas, Claude Code revient au niveau le plus élevé supporté au niveau ou en dessous de celui que vous avez défini. Par exemple, `xhigh` s'exécute comme `high` sur Opus 4.6. Votre organisation ou vos propres paramètres peuvent également limiter les niveaux qu'un modèle offre ; voir [Limites d'effort de l'organisation](#organization-effort-limits).

Avec le paramètre [`ultracode`](/docs/fr/settings-reference#ultracode) désactivé, Claude Code résout le niveau d'effort de la session dans cet ordre, en prenant le premier qui s'applique :

1. Un choix explicite : la variable d'environnement [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/fr/env-vars#variables), le lancement avec `--effort`, ou `/effort` dans la session ([un `/effort` non interactif a un effet plus étroit](#non-interactive-effort))
2. Vos paramètres : le niveau que vous avez enregistré pour le modèle ou une clé [`effortLevel`](/docs/fr/settings-reference#effortlevel), avec la priorité entre eux et entre les fichiers de paramètres énoncée à [`modelSettings`](/docs/fr/settings-reference#modelsettings)
3. L'effort par défaut du modèle : `high` sur chaque modèle qui supporte l'effort, sauf qu'Opus 5.5 par défaut à `medium`, Opus 4.7 par défaut à `xhigh`, et, quand votre organisation définit un niveau d'effort par défaut pour son [modèle par défaut de l'organisation](#organization-default-model), ce niveau est le défaut quand vous exécutez ce modèle

Opus 5.5 démarre à `medium` sauf si l'une des sources ci-dessus définit un niveau pour lui, et un `effortLevel` de niveau supérieur dans votre fichier de paramètres utilisateur ne compte pas pour Opus 5.5. Cette clé est la forme plus ancienne que `/effort` écrivait avant que Claude Code enregistre les niveaux par modèle : elle continue à s'appliquer où elle s'appliquait avant, sur Opus 5, Fable 5.1 et les modèles antérieurs, tandis qu'Opus 5.5 et les modèles publiés après lui démarrent à leur propre défaut jusqu'à ce que vous choisissiez un niveau pour eux avec `/effort` ou le sélecteur `/model`. Un `effortLevel` de niveau supérieur dans les paramètres de projet, locaux ou gérés, ou un passé avec `--settings`, s'applique à chaque modèle.

Quand vous définissez `low`, `medium`, `high` ou `xhigh` dans une session interactive sur votre machine, vous choisissez combien de temps cela dure en comment vous le confirmez :

* `Entrée` dans le curseur `/effort` ou le sélecteur `/model`, ou un niveau tapé après `/effort` : enregistrez le niveau comme votre défaut et appliquez-le dans les sessions ultérieures
* `s` dans le curseur `/effort` ou le sélecteur `/model` : appliquez le niveau à cette session uniquement. Nécessite Claude Code v2.1.257 ou ultérieure

Claude Code enregistre le niveau par modèle, sous la clé [`modelSettings`](/docs/fr/settings-reference#modelsettings) dans vos paramètres utilisateur, de sorte que chaque modèle conserve son propre niveau enregistré.

`max` est le niveau de raisonnement le plus profond. À moins que vous ne le définissiez via la variable d'environnement `CLAUDE_CODE_EFFORT_LEVEL`, Claude Code applique `max` à la session actuelle uniquement.

<Note>
  Un niveau que vous choisissez à partir du contrôle d'effort sur un téléphone ou un navigateur connecté via [Contrôle à distance](/docs/fr/remote-control#what-connected-devices-see) s'applique à cette session uniquement.
</Note>

<span id="non-interactive-effort" />

Quand vous définissez un niveau avec `/effort` dans une exécution [`-p`](/docs/fr/headless), Claude Code l'applique à cette session uniquement et ne l'enregistre pas comme votre défaut.

Le menu `/effort` offre également `ultracode`. Ultracode est un paramètre Claude Code plutôt qu'un niveau d'effort du modèle : il envoie `xhigh` au modèle et a en outre Claude orchestrer les [flux de travail dynamiques](/docs/fr/workflows) pour les tâches substantielles. Pour où il peut être défini de manière persistante, voir le paramètre [`ultracode`](/docs/fr/settings-reference#ultracode).

Vous pouvez activer ultracode via l'une des options suivantes :

* **`/effort`** : exécutez `/effort ultracode`, ou sélectionnez-le dans le menu
* **Indicateur `--effort`** : lancez avec `claude --effort ultracode`, qui démarre la session à l'effort `xhigh` avec ultracode activé
* **Paramètre `ultracode`** : définissez [`"ultracode": true`](/docs/fr/settings-reference#ultracode) dans un fichier de paramètres, avec `--settings`, ou dans une demande de contrôle Agent SDK. Une demande [`applyFlagSettings()`](/docs/fr/agent-sdk/typescript#applyflagsettings) accepte également `effortLevel: "ultracode"`
* **Sélecteur `/model`** : déplacez le curseur d'effort vers `ultracode` avec les touches fléchées pendant que vous choisissez un modèle. Claude Code l'active pour la session actuelle, même quand vous enregistrez ce modèle comme votre défaut

Passer `ultracode` à l'indicateur `--effort` ou à la valeur Agent SDK `effortLevel` nécessite Claude Code v2.1.203 ou ultérieure. Avant la v2.1.203, `--effort ultracode` imprimait `Unknown --effort value 'ultracode'` et la session démarrait à l'effort par défaut.

Le paramètre `effortLevel` persisté et la variable d'environnement `CLAUDE_CODE_EFFORT_LEVEL` n'acceptent pas `ultracode`. Quand `CLAUDE_CODE_EFFORT_LEVEL` est défini sur un niveau autre que `xhigh`, les demandes s'exécutent à ce niveau et l'orchestration de flux de travail d'ultracode reste inactive. Sélectionner ultracode affiche alors un avertissement que la variable d'environnement remplace l'effort pour la session.

<span id="when-ultracode-is-available" />

Ultracode n'est pas disponible quand :

* [Les flux de travail sont désactivés](/docs/fr/workflows#turn-workflows-off)
* Le modèle ne supporte pas l'effort `xhigh`
* Un [plafond d'effort](#organization-effort-limits) en dessous de `xhigh` s'applique au modèle

Dans ces cas, `--effort ultracode` démarre la session avec ultracode désactivé, au niveau d'effort le plus élevé que le modèle et tout plafond permettent, jusqu'à `xhigh`.

<h4 id="choose-an-effort-level">
  Choisir un niveau d'effort
</h4>

Chaque niveau échange la dépense de jetons contre la capacité. Le défaut convient à la plupart des tâches de codage ; ajustez quand vous voulez un équilibre différent.

| Niveau      | Quand l'utiliser                                                                                                                                                         |
| :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `low`       | Réservez pour les tâches courtes, délimitées, sensibles à la latence qui ne sont pas sensibles à l'intelligence                                                          |
| `medium`    | Réduit l'utilisation de jetons pour le travail sensible aux coûts qui peut faire des compromis sur une certaine intelligence. Le défaut sur Opus 5.5                     |
| `high`      | Équilibre l'utilisation de jetons et l'intelligence. Le défaut sur chaque modèle sauf Opus 5.5 et Opus 4.7                                                               |
| `xhigh`     | Raisonnement plus profond à une dépense de jetons plus élevée. Le défaut sur Opus 4.7                                                                                    |
| `max`       | Peut améliorer les performances sur les tâches exigeantes mais peut montrer des rendements décroissants et est sujet à la surréflexion. Testez avant d'adopter largement |
| `ultracode` | Un paramètre Claude Code qui planifie un [flux de travail dynamique](/docs/fr/workflows) pour chaque tâche substantielle avec un raisonnement `xhigh` par message             |

L'échelle d'effort est calibrée par modèle, de sorte que le même nom de niveau ne représente pas la même valeur sous-jacente entre les modèles.

<h4 id="use-ultrathink-for-one-off-deep-reasoning">
  Utiliser ultrathink pour un raisonnement profond ponctuel
</h4>

Incluez `ultrathink` n'importe où dans votre invite pour demander un raisonnement plus profond sur ce tour sans changer votre paramètre d'effort de session. Claude Code reconnaît le mot-clé et ajoute une instruction en contexte. Le niveau d'effort envoyé à l'API est inchangé. Claude Code transmet d'autres phrases comme « think », « think hard » et « think more » comme du texte d'invite ordinaire et ne les reconnaît pas comme des mots-clés.

<h4 id="set-the-effort-level">
  Définir le niveau d'effort
</h4>

Vous pouvez changer l'effort via l'une des options suivantes :

* **`/effort`** : exécutez `/effort` sans arguments pour ouvrir un curseur interactif, `/effort` suivi d'un nom de niveau pour le définir directement, ou `/effort auto` pour effacer votre niveau enregistré pour le modèle actif. Vous pouvez l'exécuter pendant que Claude travaille, et une fois que vous confirmez l'[avertissement de cache](/docs/fr/prompt-caching#changing-effort-level), si Claude Code en affiche un, Claude Code applique le nouveau niveau à la demande suivante du tour
* **Dans `/model`** : utilisez les touches fléchées gauche/droite pour ajuster le curseur d'effort lors de la sélection d'un modèle
* **Indicateur `--effort`** : passez un nom de niveau pour le définir pour une seule session lors du lancement de Claude Code
* **Variable d'environnement** : définissez `CLAUDE_CODE_EFFORT_LEVEL` sur un nom de niveau ou `auto`
* **Paramètres** : définissez un niveau par modèle dans [`modelSettings`](/docs/fr/settings-reference#modelsettings), ou définissez [`effortLevel`](/docs/fr/settings-reference#effortlevel) sur `low`, `medium`, `high` ou `xhigh` comme défaut pour les modèles sans un. `max` n'est pas accepté comme niveau dans l'une ou l'autre clé, et `ultracode` a sa propre clé [`ultracode`](/docs/fr/settings-reference#ultracode)
* **À partir d'un appareil connecté** : dans une session [Contrôle à distance](/docs/fr/remote-control#what-connected-devices-see), choisissez un niveau à partir du contrôle d'effort sur votre téléphone ou dans votre navigateur. Le niveau s'applique à la session actuelle uniquement. Nécessite Claude Code v2.1.234 ou ultérieure
* **Frontmatter de skill et de sous-agent** : définissez `effort` dans un fichier markdown [skill](/docs/fr/skills#frontmatter-reference) ou [sous-agent](/docs/fr/sub-agents#supported-frontmatter-fields) pour remplacer le niveau d'effort quand ce skill ou sous-agent s'exécute

L'effort du frontmatter s'applique quand ce skill ou sous-agent est actif, remplaçant le niveau de session mais pas la variable d'environnement. Un [`maxEffortLevel`](/docs/fr/settings-reference#maxeffortlevel) ou un [plafond d'effort de l'organisation](#organization-effort-limits) limite toujours le niveau auquel le skill ou sous-agent s'exécute.

Si vous définissez `effortLevel` dans les [paramètres gérés](/docs/fr/managed-settings), Claude Code l'applique à l'étape des paramètres de l'[ordre de résolution d'effort](#adjust-effort-level), et les utilisateurs peuvent toujours changer le niveau avec `/effort` ou `--effort`. Pour garder les utilisateurs au niveau ou en dessous d'un niveau, définissez [`maxEffortLevel`](/docs/fr/settings-reference#maxeffortlevel).

Le curseur d'effort apparaît dans `/model` quand un modèle supporté est sélectionné. Le niveau d'effort actuel est également affiché dans l'en-tête de session à côté du nom du modèle, par exemple « with low effort », de sorte que vous pouvez confirmer quel paramètre est actif sans ouvrir `/model`. Le pied de page affiche également brièvement le niveau d'effort au démarrage et quand il change.

<h4 id="adaptive-reasoning-and-fixed-thinking-budgets">
  Raisonnement adaptatif et budgets de réflexion fixes
</h4>

Le raisonnement adaptatif rend la réflexion optionnelle à chaque étape, de sorte que Claude peut répondre plus rapidement aux invites de routine et réserver une réflexion plus profonde pour les étapes qui en bénéficient. Si vous voulez que Claude pense plus ou moins souvent que le niveau actuel ne produit, vous pouvez le dire directement dans votre invite ou dans `CLAUDE.md` ; le modèle répond à cette orientation dans son paramètre d'effort.

Les modèles Fable, Sonnet 5 et Opus 4.7 et ultérieur utilisent toujours le raisonnement adaptatif. Le mode de budget de réflexion fixe et `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING` ne s'y appliquent pas.

Sur Opus 4.6 et Sonnet 4.6, vous pouvez définir `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` pour revenir au budget de réflexion fixe précédent contrôlé par `MAX_THINKING_TOKENS`. Voir [variables d'environnement](/docs/fr/env-vars).

<h3 id="extended-thinking">
  Réflexion étendue
</h3>

La réflexion étendue est le raisonnement que Claude émet avant de répondre. Sur les modèles qui supportent le [raisonnement adaptatif](#adjust-effort-level), le niveau d'effort est le contrôle principal pour la quantité de réflexion qui se produit ; les paramètres ci-dessous activent ou désactivent la réflexion et contrôlent comment elle s'affiche. Avec la réflexion désactivée sur l'API Anthropic, Claude Code envoie l'effort `high` au lieu d'un niveau supérieur aux modèles qu'il sait [n'acceptent pas cette combinaison](/docs/fr/errors#effort-isnt-available-with-thinking-turned-off), comme Opus 5.

| Contrôle                                    | Comment le définir                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Basculer pour la session actuelle           | Appuyez sur `Option+T` sur macOS ou `Alt+T` sur Windows et Linux                                                                                                                                                                                                                                                                                                                                                                                          |
| Définir le défaut global                    | Exécutez `/config` et basculez le mode de réflexion. Enregistré comme `alwaysThinkingEnabled` dans `~/.claude/settings.json`                                                                                                                                                                                                                                                                                                                              |
| Désactiver via une variable d'environnement | Définissez [`MAX_THINKING_TOKENS=0`](/docs/fr/env-vars), qui désactive la réflexion sur l'API Anthropic sauf sur Opus 5.5 et les modèles Fable. Sur les [fournisseurs tiers](/docs/fr/third-party-integrations), Claude Code omet le paramètre `thinking` à la place, et les modèles de raisonnement adaptatif peuvent toujours penser. D'autres valeurs s'appliquent uniquement avec un [budget de réflexion fixe](#adaptive-reasoning-and-fixed-thinking-budgets) |

Vous ne pouvez pas désactiver la réflexion sur Opus 5.5 ou les modèles Fable. Le basculement de session, `alwaysThinkingEnabled` et `MAX_THINKING_TOKENS=0` n'ont aucun effet là, et le modèle décide à chaque étape combien penser en fonction du niveau d'effort.

Claude Code réduit la sortie de réflexion par défaut. Appuyez sur `Ctrl+O` pour basculer le mode verbeux et voir le raisonnement en tant que texte gris en italique. Les sessions interactives sur l'API Anthropic reçoivent des blocs de réflexion édités par défaut, donc définissez `showThinkingSummaries: true` dans les [paramètres](/docs/fr/settings) si vous voulez les résumés complets disponibles quand vous développez. Vous êtes facturé pour tous les jetons de réflexion générés, même quand réduits ou édités.

<h3 id="extended-context">
  Contexte étendu
</h3>

Fable 5.1, Fable 5, Sonnet 5, Opus 4.6 et ultérieur, et Sonnet 4.6 supportent une [fenêtre de contexte de 1 million de jetons](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) pour les sessions longues avec de grandes bases de code.

Sur l'API Anthropic, Fable 5.1, Fable 5, Sonnet 5 et Opus 4.7 et ultérieur s'exécutent avec la fenêtre de 1M par défaut. Vous ne sélectionnez pas une variante `[1m]` ou n'activez pas les crédits d'utilisation pour la fenêtre de 1M sur ces modèles. L'utilisation de Fable elle-même peut être facturée aux crédits d'utilisation sur certains plans ; voir [Fable et crédits d'utilisation](#fable-and-usage-credits).

Opus 4.6 et Sonnet 4.6 n'atteignent 1M que via leur variante `[1m]`, et l'accès à cette variante dépend de votre plan. Sur les plans Max, Team et Enterprise, y compris les sièges Team Standard et Team Premium, Opus 4.6 avec un contexte de 1M est inclus avec votre abonnement. Sonnet 4.6 avec un contexte de 1M nécessite des [crédits d'utilisation](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) sur chaque plan d'abonnement, y compris Max.

| Plan                      | Opus 4.6 avec contexte de 1M                                                                                             | Sonnet 4.6 avec contexte de 1M                                                                                           |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------ |
| Max, Team et Enterprise   | Inclus avec l'abonnement                                                                                                 | Nécessite des [crédits d'utilisation](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) |
| Pro                       | Nécessite des [crédits d'utilisation](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) | Nécessite des [crédits d'utilisation](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) |
| API et paiement à l'usage | Accès complet                                                                                                            | Accès complet                                                                                                            |

Claude Code vérifie ces exigences de plan uniquement quand il se connecte directement à l'API Anthropic. Si vous pointez `ANTHROPIC_BASE_URL` vers une [passerelle LLM](/docs/fr/llm-gateway#subscriptions-and-gateways) et votre connexion claude.ai enregistrée reste la credential active, Claude Code ne vérifie pas les crédits d'utilisation de votre plan. Les options `[1m]` restent disponibles dans `/model`, et la passerelle décide si la demande réussit. Avant la v2.1.229, Claude Code rejetait `/model sonnet[1m]` dans cette configuration quand il ne pouvait pas confirmer les crédits d'utilisation sur le compte.

Pour désactiver le contexte de 1M, définissez `CLAUDE_CODE_DISABLE_1M_CONTEXT=1`. Claude Code supprime les variantes de modèle de 1M du sélecteur de modèle. Sur les modèles avec une fenêtre de 1M native, comme Sonnet 5 et les modèles Fable, il traite également le modèle comme ayant une fenêtre de contexte de 200K :

* Avec la compaction automatique activée, les sessions se compactent à la limite de 200K via la [compaction automatique](#set-the-auto-compact-window). Définir la fenêtre de compaction automatique au-dessus de 200K ne lève pas la retenue, car Claude Code limite cette fenêtre à la fenêtre de contexte du modèle.
* Avec la compaction automatique désactivée, les sessions s'arrêtent à la limite de 200K avec l'[erreur de limite de contexte](/docs/fr/errors#prompt-is-too-long) au lieu de se compacter.

Avant la v2.1.223, Claude Code tenait uniquement Sonnet 5, Opus 4.8 et Opus 5 sessions à 200K. Voir [variables d'environnement](/docs/fr/env-vars).

La fenêtre de contexte de 1M utilise la tarification standard du modèle sans prime pour les jetons au-delà de 200K. Pour les plans où le contexte étendu est inclus avec votre abonnement, l'utilisation reste couverte par votre abonnement. Pour les plans qui accèdent au contexte étendu via des crédits d'utilisation, les jetons sont facturés aux crédits d'utilisation.

Si votre compte supporte le contexte de 1M, l'option apparaît dans le sélecteur `/model` dans les dernières versions de Claude Code. Si vous ne la voyez pas, essayez de redémarrer votre session.

Vous pouvez également utiliser le suffixe `[1m]` avec les alias de modèle ou les noms de modèle complets :

```text theme={null}
# Utilisez l'alias opus[1m] ou sonnet[1m]
/model opus[1m]
/model sonnet[1m]

# Ou ajoutez [1m] à un nom de modèle complet
/model claude-opus-4-8[1m]
```

<h4 id="sonnet-5-context-window">
  Fenêtre de contexte Sonnet 5
</h4>

Sur l'API Anthropic, Sonnet 5 s'exécute toujours avec la fenêtre de contexte de 1M. Il n'y a pas de variante de 200K, pas de suffixe `[1m]` à sélectionner, et aucun crédit d'utilisation requis sur aucun plan. Les sessions se compactent automatiquement avant que la fenêtre ne se remplisse, à environ 967K jetons par défaut ; définissez [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/fr/env-vars) pour choisir un seuil différent.

Deux configurations budgètent la fenêtre à 200K à la place :

* **Passerelle LLM** : quand `ANTHROPIC_BASE_URL` pointe vers une [passerelle](/docs/fr/llm-gateway), Claude Code ne peut pas vérifier le support de 1M. Pour utiliser la fenêtre complète, sélectionnez Sonnet 5 (1M context) dans le sélecteur de modèle, qui mappe à `sonnet[1m]`.
* **`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`** : tient les sessions sur chaque modèle avec une fenêtre de 1M native à une fenêtre de 200K ; voir [Contexte étendu](#extended-context) pour comment la retenue est appliquée. Utile pour les déploiements qui ont besoin de limiter le contexte.

<h2 id="context-window-and-auto-compaction">
  Fenêtre de contexte et compaction automatique
</h2>

La fenêtre de compaction automatique détermine le taux de remplissage de la fenêtre de contexte avant que Claude Code compacte la conversation. Pour savoir ce que la compaction conserve et supprime selon chaque mécanisme, consultez [Ce qui survit à la compaction](/docs/fr/context-window#what-survives-compaction).

<h3 id="set-the-auto-compact-window">
  Définir la fenêtre de compaction automatique
</h3>

Vous pouvez définir la fenêtre de compaction automatique à trois endroits :

* **Pour cette session et les sessions ultérieures** : exécutez `/autocompact` avec une valeur, comme `/autocompact 500k`. Claude Code l'enregistre dans vos paramètres utilisateur sous [`autoCompactWindow`](/docs/fr/settings-reference#autocompactwindow) et l'applique à la session actuelle ; si une [portée de paramètres](/docs/fr/settings#settings-precedence) de priorité plus élevée, comme les paramètres gérés, définit la clé, la commande enregistre votre valeur mais la session conserve la fenêtre de cette portée, et la commande vous l'indique. Exécutez `/autocompact auto` pour revenir à la fenêtre optimisée pour votre modèle.
* **Pour un lancement** : passez [`--autocompact`](/docs/fr/cli-reference#cli-flags) au démarrage de Claude Code. Le drapeau remplace votre paramètre enregistré pour ce lancement sans le modifier, et `claude --autocompact auto` exécute la session à la fenêtre optimisée même si votre paramètre enregistré a une valeur. Contrairement à `/autocompact`, le drapeau n'est pas préempté par une portée de paramètres de priorité plus élevée, comme les paramètres gérés.
* **Dans les scripts et les environnements cloud** : définissez [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/fr/env-vars). Tant qu'il est défini, il a la priorité sur la commande, le drapeau et le paramètre, et `/autocompact` signale le remplacement au lieu de modifier la fenêtre.

La commande et le drapeau acceptent une taille de fenêtre de 100 K à 1 M de jetons, sous l'une de ces formes :

* Un nombre de jetons simple, comme `200000`
* Un suffixe `k` ou `M`, comme `500k` ou `1M`
* Un nombre simple de 100 à 1 000, signifiant des milliers, donc `200` définit 200 000

La variable d'environnement accepte uniquement le nombre de jetons simple. Claude Code limite la fenêtre à la fenêtre de contexte du modèle.

<h3 id="default-auto-compact-thresholds">
  Seuils de compaction automatique par défaut
</h3>

Si vous ne définissez pas de fenêtre de compaction automatique, Claude Code compacte lorsque la conversation atteint la limite de contexte du modèle, sauf dans ces sessions :

* Les [sessions cloud](/docs/fr/claude-code-on-the-web) se compactent à mesure que la conversation approche de la limite du modèle
* Sonnet 4.6 et Opus 4.6 sans [contexte étendu](#extended-context) se compactent à la limite de 200 K, tout comme Opus 4.8 et versions ultérieures lorsqu'ils s'exécutent avec une fenêtre de contexte de 200 K, comme sur Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry
* Lorsque vous définissez [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/fr/env-vars), les modèles avec une fenêtre native de 1 M, comme Sonnet 5 et les modèles Fable, se compactent à la limite de 200 K
* Les modèles s'exécutant avec une fenêtre native de 1 M, comme Sonnet 5, les modèles Fable, et Opus 4.7 et versions ultérieures sur l'API Anthropic, se compactent avant que la fenêtre ne se remplisse, à environ 967 K jetons par défaut. Sur Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry, [Épingler les modèles pour les déploiements tiers](#pin-models-for-third-party-deployments) indique quels modèles s'exécutent avec cette fenêtre ; pour les configurations qui budgétisent Sonnet 5 à 200 K à la place, consultez [Fenêtre de contexte Sonnet 5](#sonnet-5-context-window)
* Les sessions sur un ID de modèle que Claude Code ne reconnaît pas, comme un alias de [passerelle LLM](/docs/fr/llm-gateway), se compactent à la fenêtre de contexte que Claude Code suppose pour l'ID ; consultez [Corriger la fenêtre pour une passerelle ou un ID de modèle personnalisé](#correct-the-window-for-a-gateway-or-custom-model-id)

<h3 id="correct-the-window-for-a-gateway-or-custom-model-id">
  Corriger la fenêtre pour une passerelle ou un ID de modèle personnalisé
</h3>

Sur une [passerelle LLM](/docs/fr/llm-gateway) ou un autre déploiement personnalisé, Claude Code peut supposer une fenêtre de contexte pour l'ID de modèle qui diffère de la fenêtre réelle du modèle, qu'il résolve ou non l'ID en un modèle Claude. Définissez [`CLAUDE_CODE_MAX_CONTEXT_TOKENS`](/docs/fr/env-vars) à la fenêtre que Claude Code devrait supposer à la place.

La façon dont la variable s'applique dépend de l'ID. Claude Code traite un ID comme un fournisseur ou une orthographe personnalisée lorsqu'il ne commence pas par `claude-`, en toute casse, ou lorsqu'il porte un suffixe que Claude Code supprime lors de la lecture de l'ID, comme la date `@YYYYMMDD` utilisée sur Google Cloud's Agent Platform. Avant la v2.1.259, Claude Code ne comptait pas un suffixe supprimé, donc un ID `claude-` non reconnu avec un suffixe de date était traité comme un nom `claude-` simple.

Un fournisseur ou une orthographe personnalisée non reconnu, la même orthographe avec `[1m]`, et tous les autres ID sont trois cas distincts :

* Si Claude Code ne peut pas résoudre un fournisseur ou une orthographe personnalisée en un modèle qu'il reconnaît et que l'ID ne contient pas `[1m]`, la variable s'applique directement et la compaction proactive continue à la fenêtre déclarée.
* Si Claude Code ne peut pas résoudre un fournisseur ou une orthographe personnalisée en un modèle qu'il reconnaît et que l'ID contient `[1m]`, en toute casse, Claude Code suppose une fenêtre de 1 M pour celui-ci et la variable ne s'applique pas d'elle-même. Pour corriger la fenêtre tout en maintenant la compaction proactive, définissez également [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/fr/env-vars). Avec cette variable définie, Claude Code dimensionne l'ID comme la même orthographe sans `[1m]`, donc `CLAUDE_CODE_MAX_CONTEXT_TOKENS` s'applique lorsqu'il s'appliquerait à cette orthographe sans étiquette.

  Avec une fenêtre déclarée supérieure à 200 K, Claude Code affiche alors un [avertissement au démarrage](/docs/fr/errors#the-200k-limit-isnt-enforced) indiquant que la limite de 200 K n'est pas appliquée. L'avertissement est attendu dans cette configuration.
* Si l'ID se résout en un modèle que Claude Code reconnaît, ou si l'ID est un nom `claude-` simple sans suffixe pour Claude Code à supprimer, en toute casse, la variable ne prend effet que lorsque vous définissez également [`DISABLE_COMPACT`](/docs/fr/env-vars), ce qui désactive toute compaction.

  Par exemple, un ID qui contient un nom de modèle Claude que Claude Code connaît, comme `anthropic/claude-opus-4-8`, `us.anthropic.claude-…-v1:0`, ou le daté `claude-sonnet-4-5@20250929`, se résout en ce modèle. Cela inclut les ID qui contiennent également `[1m]` : Claude Code résout `claude-opus-4-8[1m]` en Opus 4.8 même avec `CLAUDE_CODE_DISABLE_1M_CONTEXT` défini.

Pour un ID de modèle que Claude Code ne reconnaît pas, définissez [`CLAUDE_CODE_DISABLE_UNKNOWN_MODEL_WINDOW_ENFORCEMENT=1`](/docs/fr/env-vars) pour que Claude Code se compacte uniquement après que l'API rejette la conversation avec une [erreur de longueur excessive que Claude Code reconnaît](/docs/fr/errors#prompt-is-too-long). Claude Code n'exécute pas cette récupération lorsqu'une passerelle [réécrit l'erreur](/docs/fr/llm-gateway-connect#troubleshoot-gateway-errors) avec un libellé que Claude Code ne reconnaît pas.

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

Pour lister plusieurs modèles à la place, dans votre propre ordre et sous les étiquettes que vous choisissez, définissez [`modelPicker`](/docs/fr/settings-reference#modelpicker). Son entrée indique quelles lignes le sélecteur conserve lorsque cet alignement remplace celui intégré.

Cet exemple définit les trois variables pour rendre un déploiement Opus acheminé par passerelle sélectionnable. Claude Code lit les variables d'environnement au démarrage, donc exécutez les exports avant de lancer `claude`, ou redémarrez une session existante pour les récupérer :

```bash theme={null}
export ANTHROPIC_CUSTOM_MODEL_OPTION="my-gateway/claude-opus-5-5"
export ANTHROPIC_CUSTOM_MODEL_OPTION_NAME="Opus via Gateway"
export ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION="Custom deployment routed through the internal LLM gateway"
```

`ANTHROPIC_CUSTOM_MODEL_OPTION_NAME` et `ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION` sont optionnels :

* Si vous omettez le nom, l'entrée affiche le nom du modèle lorsque Claude Code [reconnaît l'ID](#customize-pinned-model-display-and-capabilities), et l'ID du modèle sinon.
* Si vous omettez la description, Claude Code utilise `Custom model (<model-id>)`.

Claude Code répertorie l'entrée personnalisée après les entrées intégrées, et toutes les lignes [`modelPicker`](/docs/fr/settings-reference#modelpicker) que vous ajoutez viennent après.

Claude Code ignore la validation pour l'ID de modèle défini dans `ANTHROPIC_CUSTOM_MODEL_OPTION`, vous pouvez donc utiliser n'importe quelle chaîne que votre point de terminaison API accepte.

Lorsque [`availableModels`](#restrict-model-selection) est défini, incluez également l'ID de modèle personnalisé dans la liste d'autorisation. Sinon, Claude Code filtre l'entrée personnalisée du sélecteur et rejette une sélection `--model` de celui-ci comme tout autre modèle exclu.

Un ID personnalisé qui intègre un nom de famille, tel que `my-gateway/claude-opus-5-5`, compte comme une entrée spécifique pour cette famille et désactive son caractère générique, donc listez également les versions que vous avez l'intention de garder sélectionnables. Voir [Comportement de fusion](#merge-behavior).

<h2 id="environment-variables">
  Variables d'environnement
</h2>

Utilisez les variables d'environnement suivantes pour contrôler les noms de modèle auxquels les alias sont mappés. Chaque valeur doit être un nom de modèle complet, ou l'identifiant équivalent pour votre fournisseur d'API. Pour choisir le modèle sur lequel vos sessions démarrent, définissez [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions), que ce tableau omet.

| Variable d'environnement         | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `ANTHROPIC_DEFAULT_FABLE_MODEL`  | Le modèle à utiliser pour `fable`, et l'ID de modèle que Claude Code reconnaît comme modèle Fable pour le [basculement automatique du modèle](#automatic-model-fallback) sur les fournisseurs tiers                                                                                                                                                                                                                                                                                                                                                          |
| `ANTHROPIC_DEFAULT_OPUS_MODEL`   | Le modèle à utiliser pour `opus`, ou pour `opusplan` lorsque le mode Plan est actif.                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `ANTHROPIC_DEFAULT_SONNET_MODEL` | Le modèle à utiliser pour `sonnet`, ou pour `opusplan` lorsque le mode Plan n'est pas actif.                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `ANTHROPIC_DEFAULT_HAIKU_MODEL`  | Le modèle à utiliser pour `haiku`, ou [fonctionnalité d'arrière-plan](/docs/fr/costs#background-token-usage)                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `CLAUDE_CODE_SUBAGENT_MODEL`     | Le modèle par défaut pour les [subagents](/docs/fr/sub-agents#choose-a-model), les coéquipiers de l'[équipe d'agents](/docs/fr/agent-teams#specify-teammates-and-models) et les agents de [workflow](/docs/fr/workflows) qui ne sont pas assignés à un modèle d'une autre manière. Accepte un alias tel que `haiku` ou un nom de modèle complet. Un modèle par invocation ou le champ `model` d'une définition, y compris `inherit`, prend la priorité. Pour modifier cela, définissez [`CLAUDE_CODE_SUBAGENT_MODEL_FORCE`](/docs/fr/sub-agents#run-every-subagent-on-one-model) |

Remarque : `ANTHROPIC_SMALL_FAST_MODEL` est déprécié au profit de `ANTHROPIC_DEFAULT_HAIKU_MODEL`.

<h3 id="pin-models-for-third-party-deployments">
  Épingler les modèles pour les déploiements tiers
</h3>

Lors du déploiement de Claude Code via [Amazon Bedrock](/docs/fr/amazon-bedrock), [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai), [Microsoft Foundry](/docs/fr/microsoft-foundry), ou [Claude Platform on AWS](/docs/fr/claude-platform-on-aws), épinglez les versions de modèle avant de les déployer auprès des utilisateurs.

Sans épinglage, Claude Code utilise les alias de modèle tels que `fable`, `opus`, `sonnet` et `haiku` qui se résolvent à un ID de modèle par défaut intégré pour chaque fournisseur. Ce défaut peut être en retard par rapport à la dernière version d'Anthropic, et le modèle auquel il pointe peut ne pas encore être activé dans le compte d'un utilisateur. Lorsque le défaut n'est pas disponible, les utilisateurs d'Amazon Bedrock et de Google Cloud's Agent Platform voient un avis et la session revient à une version antérieure du modèle par défaut, ou au modèle Sonnet par défaut lorsque le défaut est un modèle Opus et qu'aucune version Opus n'est disponible. Les utilisateurs de Microsoft Foundry voient des erreurs à la place, car Microsoft Foundry n'a pas de vérification de démarrage équivalente.

Sur Amazon Bedrock et Google Cloud's Agent Platform, un utilisateur qui démarre la session sur une version spécifique de Sonnet ou Opus, par exemple avec `--model`, `ANTHROPIC_MODEL`, ou le paramètre `model`, épingle cette version comme défaut de la session pour l'alias correspondant : la vérification de démarrage ignore le défaut intégré qu'il remplace et n'affiche aucun avis de basculement. Avant v2.1.211, la vérification s'exécutait et pouvait afficher un avis même lorsqu'un modèle de session était explicitement configuré.

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

Pour activer le [contexte étendu](#extended-context) pour un modèle épinglé, ajoutez `[1m]` à l'ID du modèle dans `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, ou `ANTHROPIC_DEFAULT_FABLE_MODEL` :

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8[1m]'
```

Avec le suffixe `[1m]`, la fenêtre de contexte 1M s'applique à toute utilisation de l'alias épinglé, y compris la phase Opus en mode plan de [`opusplan`](#opusplan-model-setting) et les [subagents](/docs/fr/sub-agents#choose-a-model) dont le frontmatter `model` nomme l'alias.

* Claude Code supprime le suffixe avant d'envoyer l'ID du modèle à votre fournisseur.
* N'ajoutez `[1m]` que lorsque le modèle sous-jacent [prend en charge le contexte 1M](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model).
* Le suffixe est lu par variable, et non par modèle. Sur Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry, un ID de modèle sans `[1m]` dans une variable utilise le contexte 200K même si une autre variable définit le même modèle avec le suffixe. Sonnet 5 s'exécute toujours avec la fenêtre 1M sur ces fournisseurs et n'a jamais besoin du suffixe.

<Note>
  Une liste d'autorisation `availableModels` livrée via [MDM ou un fichier de paramètres gérés](/docs/fr/managed-settings#delivery-mechanisms) s'applique toujours lors de l'utilisation de fournisseurs tiers ; les [paramètres gérés par le serveur ne sont pas livrés là](/docs/fr/server-managed-settings#platform-availability).

  Le filtrage correspond à un alias de modèle tel que `opus`, un préfixe de version tel que `claude-opus-4-8`, ou l'ID de modèle complet spécifique au fournisseur. Les préfixes spécifiques au fournisseur tels que `us.anthropic.` ne sont pas supprimés, donc pour autoriser un modèle spécifique, listez son ID complet spécifique au fournisseur, ou mappez-le via [`modelOverrides`](#override-model-ids-per-version). Pour un modèle épinglé, cet ID est la valeur que vous avez définie dans sa variable `ANTHROPIC_DEFAULT_*_MODEL`. Tout suffixe `[1m]` est supprimé de l'entrée de la liste d'autorisation et du modèle demandé avant la correspondance.
</Note>

<h3 id="customize-pinned-model-display-and-capabilities">
  Personnaliser l'affichage et les capacités du modèle épinglé
</h3>

Lorsque vous épinglez un modèle sur un fournisseur tiers, sa ligne dans le sélecteur `/model` affiche le nom du modèle par défaut si Claude Code reconnaît l'ID épinglé, et l'ID brut sinon :

* **Reconnu** : l'ID exact d'un modèle que Claude Code connaît, tel que son ID API Anthropic ou la forme de votre fournisseur ou passerelle, avec ou sans le suffixe `[1m]`. Épinglez `us.anthropic.claude-sonnet-4-5-20250929-v1:0` et la ligne affiche `Sonnet 4.5`.
* **Non reconnu** : tout autre ID, tel qu'un ARN de profil d'inférence d'application ou une version de modèle que Claude Code ne connaît pas, sauf si une entrée [`modelOverrides`](#override-model-ids-per-version) mappe un modèle à cette chaîne exacte. Sur Microsoft Foundry, les noms de déploiement sont définis par l'utilisateur, donc Claude Code ne reconnaît jamais un ID épinglé là, mappé ou non, et la ligne affiche le nom de déploiement par défaut.

Lorsqu'une ligne affiche le nom du modèle, sa description par défaut inclut l'ID épinglé afin que vous puissiez toujours voir quel ID est épinglé.

Claude Code peut également ne pas reconnaître les fonctionnalités qu'un modèle épinglé prend en charge. Vous pouvez définir vous-même le nom d'affichage et la description et déclarer les capacités avec des variables d'environnement complémentaires pour chaque modèle épinglé.

Ces variables prennent effet sur les fournisseurs tiers tels qu'Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry. Les variables `_NAME` et `_DESCRIPTION` prennent également effet lorsque `ANTHROPIC_BASE_URL` pointe vers une [passerelle LLM](/docs/fr/llm-gateway). Elles n'ont aucun effet lors de la connexion directe à `api.anthropic.com`.

| Variable d'environnement                              | Description                                                                                                                                                                                        |
| ----------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_NAME`                   | Nom d'affichage pour le modèle Opus épinglé dans le sélecteur `/model`. Lorsqu'il n'est pas défini, la ligne affiche le nom du modèle si Claude Code reconnaît l'ID épinglé, et l'ID épinglé sinon |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_DESCRIPTION`            | Description d'affichage pour le modèle Opus épinglé dans le sélecteur `/model`. Lorsqu'il n'est pas défini, la ligne affiche une description par défaut qui commence par `Custom Opus model`       |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES` | Liste séparée par des virgules des capacités que le modèle Opus épinglé prend en charge                                                                                                            |

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

Lorsque `_SUPPORTED_CAPABILITIES` est défini, Claude Code active les capacités listées et désactive les capacités non listées pour le modèle épinglé correspondant. Lorsque la variable n'est pas définie, Claude Code revient à la détection intégrée basée sur l'ID du modèle.

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

Sur les plateformes qui intègrent Claude Code et définissent [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/fr/env-vars), la configuration du modèle de l'hôte prend la priorité sur les paramètres de modèle gérés, tandis qu'une liste d'autorisation `availableModels` gérée reste en vigueur sauf si l'hôte en fournit une ; [Exceptions à la priorité des paramètres gérés](/docs/fr/settings#exceptions-to-managed-settings-precedence) indique quelles clés et variables l'hôte remplace.

Les variables d'environnement au niveau de la famille ci-dessus configurent un ID de modèle par alias de famille. Si vous devez mapper plusieurs versions au sein de la même famille à des ID de fournisseur distincts, utilisez plutôt le paramètre `modelOverrides`.

`modelOverrides` mappe les ID de modèle Anthropic individuels aux chaînes spécifiques au fournisseur que Claude Code envoie à l'API de votre fournisseur. Lorsqu'un utilisateur sélectionne un modèle mappé dans le sélecteur `/model`, Claude Code utilise votre valeur configurée au lieu de la valeur par défaut intégrée.

Cela permet aux administrateurs d'entreprise d'acheminer chaque version de modèle vers un ARN de profil d'inférence Amazon Bedrock spécifique, un nom de version Google Cloud's Agent Platform ou un nom de déploiement Microsoft Foundry pour la gouvernance, l'allocation des coûts ou l'acheminement régional.

Définissez `modelOverrides` dans votre [fichier de paramètres](/docs/fr/settings#where-settings-live) :

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

Pour arrêter la ligne de [diagnostic](/docs/fr/errors#unrecognized-model-id-on-a-request) `[claude-code:unrecognized_model]` pour un ID tel qu'un alias de passerelle, ajoutez une entrée avec cet ID comme valeur.

Les remplacements remplacent les ID de modèle intégrés qui soutiennent chaque entrée dans le sélecteur `/model`. Sur Amazon Bedrock, les entrées `modelOverrides` prennent la priorité sur tous les profils d'inférence que Claude Code découvre automatiquement au démarrage. Claude Code transmet les valeurs qui sont déjà natives du fournisseur, telles que les ARN de profil d'inférence Amazon Bedrock ou les noms de déploiement Microsoft Foundry, au fournisseur telles quelles.

Les remplacements s'appliquent également lorsque vous transmettez un ID de modèle Anthropic directement via `--model`, la variable d'environnement `ANTHROPIC_MODEL`, ou une variable d'environnement `ANTHROPIC_DEFAULT_*_MODEL`. Sur Amazon Bedrock, Google Cloud's Agent Platform et [Mantle](/docs/fr/amazon-bedrock#use-the-mantle-endpoint), un ID de modèle Anthropic sans entrée `modelOverrides` se résout au même ID spécifique au fournisseur que la ligne du sélecteur `/model` pour cette version, lorsque le fournisseur prend en charge cette version. Mantle prend en charge un sous-ensemble de versions. Pour un ID de modèle Anthropic en dehors de ce sous-ensemble, Claude Code envoie l'ID brut à Mantle sans le mapper, sauf si une entrée `modelOverrides` le couvre. Avant v2.1.200, `--model` et les valeurs des variables d'environnement atteignaient le fournisseur telles quelles sans passer par la carte de remplacement.

`modelOverrides` fonctionne aux côtés de `availableModels`. La liste d'autorisation est évaluée par rapport à l'ID de modèle Anthropic, et non à la valeur de remplacement, donc une entrée comme `"opus"` dans `availableModels` continue de correspondre même lorsque les versions d'Opus sont mappées à des ARN. Lorsque `enforceAvailableModels` est défini dans les paramètres gérés, la valeur par défaut appliquée se résout via `modelOverrides` à partir des [paramètres gérés](/docs/fr/managed-settings#how-claude-code-combines-managed-sources) uniquement. Le mappage d'un administrateur, tel qu'une version épinglée à un ARN de profil d'inférence, est honoré dans la valeur par défaut appliquée. Les remplacements des paramètres utilisateur ou projet ne l'affectent pas.

Lorsque `availableModels` est défini dans les [paramètres gérés](/docs/fr/managed-settings), seuls les `modelOverrides` de cette source gérée s'appliquent à un ID de modèle Anthropic transmis directement via `--model` ou les variables d'environnement ci-dessus. Claude Code ignore les remplacements dans les paramètres utilisateur ou projet pour ces ID, et ne résout jamais un ID que la liste gérée exclut via `modelOverrides` à partir de n'importe quelle source de paramètres. Cette restriction de source gérée nécessite Claude Code v2.1.200 ou ultérieur. Voir [Restreindre la sélection de modèle](#restrict-model-selection) pour savoir comment les ID bloqués sont gérés.

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

Pour choisir vous-même le TTL du cache pour la conversation principale et pour les subagents séparément, voir [choisir vous-même le TTL](/docs/fr/prompt-caching#choose-the-ttl-yourself). Pour ce qui déclenche un échec du cache, voir [Comment Claude Code utilise la mise en cache des invites](/docs/fr/prompt-caching).
