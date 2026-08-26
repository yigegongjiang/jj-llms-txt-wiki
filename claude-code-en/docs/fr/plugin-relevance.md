> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Recommander des plugins pour votre organisation

> Ajoutez un bloc de pertinence aux entrées de plugins de la marketplace afin que Claude Code les suggère lorsque le travail d'un utilisateur correspond.

Si vous exploitez une marketplace de plugins pour votre organisation, vous pouvez faire en sorte que Claude Code suggère des plugins spécifiques aux utilisateurs en fonction de ce sur quoi ils travaillent. Ajoutez un bloc `relevance` à l'entrée d'un plugin dans `marketplace.json`, puis autorisez la marketplace dans les paramètres gérés. Lorsque la session d'un utilisateur correspond à l'un des signaux déclarés, Claude Code affiche une suggestion d'installation pour ce plugin.

Les suggestions déclarées par la marketplace sont optionnelles par marketplace via les [paramètres gérés](/docs/fr/settings#settings-files). Aucune déclaration `relevance` d'une marketplace ne produit de suggestions jusqu'à ce qu'un administrateur l'ajoute à la liste d'autorisation, y compris la marketplace officielle d'Anthropic. Claude Code inclut également une suggestion intégrée qui est indépendante de cette liste d'autorisation ; ce conseil et tous les conseils déclarés par la marketplace sont désactivés lorsque [`spinnerTipsEnabled`](/docs/fr/settings#available-settings) est défini sur `false`.

Cette fonctionnalité nécessite Claude Code v2.1.152 ou version ultérieure. Les anciens clients ignorent le champ `relevance`.

Cette page est destinée aux opérateurs de marketplace et aux administrateurs d'entreprise. Si vous cherchez à installer des plugins, consultez [Découvrir et installer des plugins](/docs/fr/discover-plugins).

<h2 id="how-it-works">
  Fonctionnement
</h2>

Chaque entrée de plugin dans `marketplace.json` peut contenir un objet `relevance`. L'objet nomme un sujet et un ou plusieurs signaux. Un signal est un motif que Claude Code teste par rapport à la session actuelle, comme le répertoire de travail ou les fichiers que Claude a lus.

La correspondance des signaux se fait localement sur la machine de l'utilisateur. La correspondance n'ajoute aucun trafic réseau et ne signale pas à Anthropic ou à l'opérateur de la marketplace quels signaux ont correspondu ou leurs valeurs.

Lorsqu'un signal correspond et que le plugin n'est pas déjà installé, Claude Code affiche le plugin à trois endroits :

* **Conseil du spinner** : un message « Vous travaillez avec *sujet* ? Installez le plugin *plugin* » avec la commande `/plugin install` apparaît sous le spinner pendant que Claude répond.
* **Suggestion au démarrage de la session** : si le signal `cwd` correspond au répertoire de travail, une notification `plugin suggestion: <name>@<marketplace> · /plugin` d'une ligne apparaît avant le premier tour. Cette surface nécessite Claude Code v2.1.153 ou version ultérieure.
* **Onglet Discover de `/plugin`** : le plugin est épinglé en haut de la liste Discover avec une annotation telle que « suggéré pour ce répertoire » ou « suggéré pour les commandes stripe ». Cette surface nécessite Claude Code v2.1.154 ou version ultérieure.

Le conseil du spinner et la notification de démarrage de session font partie du système de conseils du spinner. Les deux sont désactivés lorsque l'utilisateur ou le projet définit `spinnerTipsEnabled` sur `false`, ou lorsqu'un `spinnerTipsOverride` personnalisé est configuré avec `excludeDefault`. L'épingle de l'onglet Discover est indépendante des paramètres de conseil.

Claude Code n'installe jamais un plugin automatiquement. L'utilisateur confirme toujours.

<h2 id="add-relevance-to-a-plugin-entry">
  Ajouter la pertinence à une entrée de plugin
</h2>

Ajoutez un objet `relevance` à l'entrée du plugin dans votre `marketplace.json`. L'exemple suivant déclare que le plugin `terraform-helpers` est pertinent lorsque Claude lit un fichier `.tf` ou lorsque Claude exécute `terraform` :

```json theme={null}
{
  "name": "acme-corp-plugins",
  "owner": { "name": "Acme Platform Team" },
  "plugins": [
    {
      "name": "terraform-helpers",
      "source": "./plugins/terraform-helpers",
      "description": "Acme conventions and helpers for Terraform",
      "relevance": {
        "topic": "Terraform",
        "signals": {
          "cli": ["terraform"],
          "filesRead": ["**/*.tf"]
        }
      }
    }
  ]
}
```

Un plugin avec un bloc `relevance` mais sans signal correspondant se comporte comme n'importe quelle autre entrée de marketplace. Il apparaît dans la liste Discover à sa position normale et ne s'affiche jamais comme un conseil du spinner.

<h2 id="field-reference">
  Référence des champs
</h2>

<h3 id="relevance">
  `relevance`
</h3>

| Champ     | Type   | Description                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :-------- | :----- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `topic`   | string | Optionnel. La phrase qui remplit « Vous travaillez avec *sujet* ? » dans le conseil du spinner. Souvent le nom du produit, par exemple `Stripe`. Utilisez un domaine tel que `design` lorsque le nom du plugin ne se lit pas naturellement comme un sujet. Par défaut, le nom du plugin avec chaque segment de tiret en majuscules. La notification de démarrage de session n'utilise pas cette valeur. Maximum 64 caractères. |
| `signals` | object | Les correspondances qui déterminent quand le plugin est pertinent. Au moins un signal est requis pour que le plugin soit suggérable. Voir le tableau ci-dessous.                                                                                                                                                                                                                                                               |

<h3 id="relevance-signals">
  `relevance.signals`
</h3>

| Champ          | Type             | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :------------- | :--------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cwd`          | array of strings | Motifs glob correspondant au répertoire de travail de la session. Correspondance en tant que chemin absolu et, lorsqu'il se trouve dans un référentiel git, en tant que chemin relatif à la racine du référentiel. Normalisé avec barre oblique avant et insensible à la casse. Chaque motif correspond au répertoire lui-même et à tout ce qui se trouve sous lui, donc `infra`, `infra/`, et `infra/**` se comportent de manière identique. C'est le seul signal qui peut correspondre au démarrage de la session, avant le premier tour. Maximum 10 motifs de 256 caractères chacun.                                                                                                                                                                                                                                                                                                                                                               |
| `cli`          | array of strings | Noms de commandes à partir des commandes shell que Claude a exécutées cette session, par exemple `["stripe"]`. S'applique sur chaque plateforme : les commandes exécutées sur Windows via PowerShell ou Git Bash sont enregistrées de la même manière. Claude Code enregistre un nom de commande par invocation d'outil shell : le premier jeton après toute affectation de variable d'environnement de début et `sudo`. Les commandes composées ne contribuent que leur commande de début, donc `cd infra && terraform plan` enregistre `cd`, pas `terraform`. Correspondance exacte. Maximum 10 entrées de 64 caractères chacune.                                                                                                                                                                                                                                                                                                                   |
| `hosts`        | array of strings | Noms d'hôtes vus dans les URL `http://` ou `https://` dans les commandes Bash cette session, par exemple `["api.stripe.com"]`. Nom d'hôte en minuscules uniquement : pas de schéma, port ou chemin. Correspondance exacte insensible à la casse. Maximum 20 entrées de 128 caractères chacune.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `filesRead`    | array of strings | Motifs glob correspondant aux chemins des fichiers que Claude a lus cette session, par exemple `["**/*.tf"]`. Normalisé avec barre oblique avant et insensible à la casse. Maximum 10 motifs de 256 caractères chacun.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `manifestDeps` | array of objects | Dépendances déclarées dans les manifestes de packages que Claude a lus cette session. Chaque entrée est `{ "file": "...", "pattern": "..." }`, où `file` est une expression régulière correspondant au chemin du fichier manifeste tel qu'enregistré dans l'état de la session, généralement un chemin absolu, et `pattern` est une expression régulière correspondant au contenu de ce fichier. Ancrez `file` à la fin, par exemple `[/\\\\]package\\.json$` sous forme échappée JSON, car un motif ancré au début ne correspond jamais à un chemin absolu. Les chemins ne sont pas normalisés par séparateur pour ce signal, donc les chemins Windows utilisent des barres obliques inverses. Les fichiers manifeste plus grands que 512 Ko sont ignorés. Les deux valeurs sont des chaînes source JavaScript `RegExp` d'au maximum 256 caractères. `file` correspond insensible à la casse. `pattern` est sensible à la casse. Maximum 10 entrées. |

Les signaux `cli`, `hosts`, `filesRead`, et `manifestDeps` ont besoin de l'historique de la session, donc ils ne peuvent correspondre que sur le conseil du spinner et l'onglet Discover. Seul `cwd` peut correspondre au démarrage de la session. Les signaux `filesRead` et `manifestDeps` testent l'état du fichier enregistré de la session, qui inclut également les fichiers que Claude a écrits ou modifiés et les fichiers de mémoire `CLAUDE.md` chargés automatiquement.

L'exemple suivant utilise `manifestDeps` pour suggérer un plugin Stripe une fois que Claude a lu un `package.json` qui dépend de `stripe`. Le motif `file` utilise `[/\\\\]` pour qu'il corresponde à la fois aux séparateurs de chemin avec barre oblique avant et barre oblique inverse, et `\\.` pour que le point soit littéral. En JSON, chaque barre oblique inverse dans l'expression régulière est écrite deux fois.

```json theme={null}
{
  "name": "stripe-helpers",
  "source": "./plugins/stripe-helpers",
  "relevance": {
    "topic": "Stripe",
    "signals": {
      "manifestDeps": [
        {
          "file": "[/\\\\]package\\.json$",
          "pattern": "\"stripe\"\\s*:"
        }
      ]
    }
  }
}
```

<Note>
  Les champs inconnus sous `relevance` et `relevance.signals` sont ignorés au moment du chargement afin que les anciens clients Claude Code continuent à charger votre marketplace. Exécutez `claude plugin validate` pour les afficher comme des avertissements.
</Note>

<h2 id="enable-suggestions-in-managed-settings">
  Activer les suggestions dans les paramètres gérés
</h2>

Déclarer `relevance` dans `marketplace.json` ne suffit pas en soi. Un administrateur doit autoriser la marketplace dans les [paramètres gérés](/docs/fr/settings#settings-files) avant que ses suggestions n'apparaissent aux utilisateurs.

Ajoutez le nom de la marketplace à `pluginSuggestionMarketplaces`. Pour toute marketplace autre que la marketplace officielle d'Anthropic, déclarez également la source de la marketplace dans les mêmes paramètres gérés, soit comme entrée de ce nom dans `extraKnownMarketplaces`, soit comme entrée dans `strictKnownMarketplaces`. Le nom autorisé est ignoré si la marketplace enregistrée sur la machine provient d'une source différente. Cela empêche une source non liée de s'enregistrer sous un nom autorisé pour que ses plugins soient suggérés dans toute votre organisation.

Le `managed-settings.json` suivant enregistre une marketplace d'organisation à partir d'un référentiel GitHub et active ses suggestions :

```json theme={null}
{
  "extraKnownMarketplaces": {
    "acme-corp-plugins": {
      "source": {
        "source": "github",
        "repo": "acme-corp/claude-plugins"
      }
    }
  },
  "pluginSuggestionMarketplaces": ["acme-corp-plugins"]
}
```

La marketplace officielle est exempte de l'exigence de déclaration de source car son nom ne peut s'enregistrer que depuis la source officielle d'Anthropic. L'autorisation du nom seul est suffisante :

```json theme={null}
{
  "pluginSuggestionMarketplaces": ["claude-plugins-official"]
}
```

Consultez la [référence des paramètres](/docs/fr/settings) pour `pluginSuggestionMarketplaces` et [`extraKnownMarketplaces`](/docs/fr/settings#extraknownmarketplaces) pour les détails complets de la configuration.

<h2 id="what-the-user-sees">
  Ce que voit l'utilisateur
</h2>

Lorsqu'un signal correspond pendant une session, le conseil du spinner se lit comme suit :

```text theme={null}
Working with Terraform? Install the terraform-helpers plugin:
/plugin install terraform-helpers@acme-corp-plugins
```

Au démarrage de la session, un signal `cwd` correspondant affiche la notification d'une ligne :

```text theme={null}
plugin suggestion: terraform-helpers@acme-corp-plugins · /plugin
```

La suggestion d'un plugin donné apparaît au maximum une fois tous les trois sessions dans le conseil du spinner et la notification de démarrage de session combinés, et aucun ne se répète une fois que le plugin est installé. La notification de démarrage de session s'arrête également d'apparaître après que la suggestion ait été affichée deux fois.

Dans l'onglet Discover de `/plugin`, le plugin est épinglé au-dessus des autres résultats avec une annotation qui nomme le signal correspondant, telle que `suggested for this directory` ou `suggested for terraform commands`. L'onglet Discover épingle un plugin donné une fois ; les visites ultérieures le listent dans l'ordre normal. L'épingle de l'onglet Discover nécessite Claude Code v2.1.154 ou version ultérieure. Sur v2.1.152, seul le conseil du spinner apparaît ; la notification de démarrage de session est ajoutée dans v2.1.153.

<h2 id="validate-your-marketplace">
  Valider votre marketplace
</h2>

Exécutez `claude plugin validate` sur votre répertoire de marketplace pour vérifier le bloc `relevance` avant la publication :

```
claude plugin validate ./my-marketplace
```

Le validateur signale les clés inconnues sous `relevance` et `relevance.signals` comme des avertissements, signale une valeur `relevance` qui n'est pas un objet, et rejette une entrée `signals.hosts` qui inclut un schéma, un port ou un chemin.

<h2 id="see-also">
  Voir aussi
</h2>

* [Créer et distribuer une marketplace de plugins](/docs/fr/plugin-marketplaces) : construisez la marketplace qui héberge vos plugins
* [Recommander votre plugin à partir de votre CLI](/docs/fr/plugin-hints) : invitez les utilisateurs à partir de votre propre CLI au lieu de partir des signaux de session de Claude Code
* [Paramètres](/docs/fr/settings) : référence complète pour `pluginSuggestionMarketplaces` et `extraKnownMarketplaces`
