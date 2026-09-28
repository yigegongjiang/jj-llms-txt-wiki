> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Recommander des plugins pour votre organisation

> Ajoutez un bloc de pertinence aux entrées de plugins de la marketplace afin que Claude Code les suggère lorsque le travail d'un utilisateur correspond, et autorisez la marketplace dans les paramètres gérés.

Claude Code peut suggérer l'installation d'un plugin depuis la marketplace de votre organisation lorsque la session d'un utilisateur correspond aux signaux que vous définissez pour ce plugin. Les signaux incluent le répertoire de travail, les fichiers que Claude a lus et les commandes que Claude a exécutées. Vous les définissez en ajoutant un bloc `relevance` à l'entrée du plugin dans `marketplace.json`.

Un opérateur de marketplace écrit les entrées `relevance`. Un administrateur autorise ensuite la marketplace dans les paramètres gérés. Les utilisateurs ne voient aucune suggestion d'une marketplace tant qu'elle n'est pas autorisée.

<Note>
  Ces cas sont couverts sur d'autres pages :

  * **Vous souhaitez installer des plugins** : consultez [Installer et gérer les plugins](/docs/fr/plugins/install)
  * **Vous souhaitez désactiver les suggestions** : consultez [Comprendre le fonctionnement de la pertinence des plugins](#understand-how-plugin-relevance-works)
</Note>

Commencez par les sections correspondant à votre rôle :

* **Opérateurs de marketplace** : lisez [comment fonctionnent les suggestions](#understand-how-plugin-relevance-works), puis [ajoutez la pertinence à une entrée de plugin](#add-relevance-to-a-plugin-entry) et [validez votre marketplace](#validate-your-marketplace)
* **Administrateurs** : [activez les suggestions dans les paramètres gérés](#enable-suggestions-in-managed-settings)

<h2 id="understand-how-plugin-relevance-works">
  Comprendre le fonctionnement de la pertinence des plugins
</h2>

Chaque entrée de plugin dans `marketplace.json` peut inclure un objet `relevance`. L'objet nomme un sujet et un ou plusieurs signaux. Un signal est un motif que Claude Code teste par rapport à la session actuelle, comme le répertoire de travail ou les fichiers que Claude a lus.

La correspondance des signaux se fait localement sur la machine de l'utilisateur et n'ajoute aucun trafic réseau. Claude Code ne signale pas à Anthropic ou à l'opérateur de marketplace quels signaux ont correspondu ou leurs valeurs.

Lorsqu'un signal correspond et que le plugin n'est pas déjà installé, Claude Code suggère le plugin aux endroits suivants :

* **Spinner tip** : un message avec la commande `/plugin install` apparaît sous le spinner pendant que Claude répond.
* **Notification au démarrage de la session** : si un signal `cwd` correspond au répertoire de travail, une notification d'une ligne apparaît avant que l'utilisateur n'envoie un premier message.
* **Onglet Discover de `/plugin`** : le plugin est épinglé en haut de la liste Discover.

[Aperçu de ce que l'utilisateur voit](#preview-what-the-user-sees) montre le texte exact de chacun et la fréquence à laquelle ils se répètent.

Claude Code n'installe jamais le plugin automatiquement. L'utilisateur confirme toujours.

Le spinner tip et la notification au démarrage de la session cessent tous deux d'apparaître lorsque l'utilisateur ou le projet définit [`spinnerTipsEnabled`](/docs/fr/settings-reference#spinnertipsenabled) sur `false`, ou lorsqu'un [`spinnerTipsOverride`](/docs/fr/settings-reference#spinnertipsoverride) avec `excludeDefault` remplace les conseils intégrés. L'épingle de l'onglet Discover n'est affectée par aucun de ces paramètres.

<h2 id="add-relevance-to-a-plugin-entry">
  Ajouter la pertinence à une entrée de plugin
</h2>

Ajoutez un objet `relevance` à l'entrée du plugin dans votre `marketplace.json`. L'exemple suivant déclare que le plugin `terraform-helpers` est pertinent lorsque Claude lit un fichier `.tf` ou exécute `terraform` :

```json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "plugins": [
    {
      "name": "terraform-helpers",
      "source": "./plugins/terraform-helpers",
      "description": "Your organization's Terraform conventions and helpers",
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

Tant qu'aucun de ses signaux ne correspond, le plugin conserve sa position normale dans la liste Discover et n'apparaît pas comme un spinner tip.

Pour vérifier le bloc avant la publication, [validez votre marketplace](#validate-your-marketplace).

<h2 id="field-reference">
  Référence des champs
</h2>

L'objet `relevance` et son objet `signals` imbriqué acceptent les champs des tableaux suivants.

Les clients plus anciens chargent toujours une marketplace qui utilise des champs `relevance` qu'ils ne reconnaissent pas, car les champs inconnus sous `relevance` et `relevance.signals` sont ignorés au moment du chargement. Un champ reconnu dont la valeur dépasse sa limite dans la [référence des champs](#field-reference) invalide l'entrée de plugin entière, et les utilisateurs ne peuvent pas installer ce plugin depuis la marketplace tant que vous ne le corrigez pas ; `claude plugin validate` signale les mêmes limites.

<h3 id="relevance">
  `relevance`
</h3>

| Champ     | Type   | Description                                                                                                                                                                                        |
| :-------- | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `topic`   | string | Optionnel. La phrase qui remplit « Travail avec *topic* ? » dans le spinner tip. Par défaut, le nom du plugin avec chaque segment de tiret en majuscules. Maximum 64 caractères.                   |
| `signals` | object | Les correspondances qui déterminent quand le plugin est pertinent. Claude Code suggère le plugin uniquement si au moins un signal est défini. Consultez [`relevance.signals`](#relevance-signals). |

Le `topic` est souvent le nom du produit, par exemple `Terraform`. Utilisez un domaine tel que `design` lorsque le nom du plugin ne semble pas naturel comme sujet.

<h3 id="relevance-signals">
  `relevance.signals`
</h3>

L'objet `signals` accepte les champs suivants.

| Champ          | Type             | Description                                                                                                                                                                                                                                                                                 | Limite                                                                                                             |
| :------------- | :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------- |
| `cwd`          | array of strings | Motifs Glob correspondant au répertoire de travail de la session. Consultez [correspondance du répertoire de travail](#working-directory-matching).                                                                                                                                         | 10 motifs de 256 caractères chacun                                                                                 |
| `cli`          | array of strings | Noms de commandes des commandes shell que Claude a exécutées cette session, par exemple `["terraform"]`. Correspondance exacte. Consultez [correspondance des noms de commandes](#command-name-matching).                                                                                   | 10 entrées de 64 caractères chacune                                                                                |
| `hosts`        | array of strings | Noms d'hôtes vus dans les URL `http://` ou `https://` dans les commandes Bash cette session, par exemple `["registry.terraform.io"]`. Nom d'hôte nu en minuscules uniquement : pas de schéma, port ou chemin. Correspondance exacte insensible à la casse.                                  | 20 entrées de 128 caractères chacune                                                                               |
| `filesRead`    | array of strings | Motifs Glob correspondant aux chemins des fichiers que Claude a lus cette session, par exemple `["**/*.tf"]`. Normalisé par barre oblique avant et insensible à la casse.                                                                                                                   | 10 motifs de 256 caractères chacun                                                                                 |
| `manifestDeps` | array of objects | Dépendances déclarées dans les manifestes de packages que Claude a lus cette session. Chaque entrée est `{ "file": "...", "pattern": "..." }`, où les deux valeurs sont des expressions régulières. Consultez [correspondance des dépendances de manifeste](#manifest-dependency-matching). | 10 entrées, chaque valeur au maximum 256 caractères. Les fichiers de manifeste plus grands que 512 Ko sont ignorés |

Les signaux `filesRead` et `manifestDeps` correspondent également aux fichiers que Claude a écrits ou modifiés cette session et aux fichiers de mémoire `CLAUDE.md` chargés automatiquement du projet.

<h4 id="working-directory-matching">
  Correspondance du répertoire de travail
</h4>

`cwd` est le seul signal qui peut correspondre au démarrage de la session, avant que l'utilisateur n'envoie un premier message.

Claude Code correspond à chaque motif `cwd` comme suit :

* Le motif est comparé au répertoire de travail en tant que chemin absolu. Lorsque la session se trouve dans un référentiel git, il est également comparé au chemin du répertoire de travail par rapport à la racine du référentiel.
* La correspondance est normalisée par barre oblique avant et insensible à la casse.
* Chaque motif correspond au répertoire lui-même et à tout ce qui se trouve sous lui, donc `infra`, `infra/` et `infra/**` se comportent de manière identique.

<h4 id="command-name-matching">
  Correspondance des noms de commandes
</h4>

Claude Code enregistre un nom de commande pour chaque commande shell que Claude exécute : le premier jeton après toute assignation de variable d'environnement de début et `sudo`. Les commandes composées ne contribuent que leur commande de début, donc `cd infra && terraform plan` enregistre `cd`, pas `terraform`.

<h4 id="manifest-dependency-matching">
  Correspondance des dépendances de manifeste
</h4>

Chaque entrée `manifestDeps` associe deux chaînes source JavaScript `RegExp` :

* `file` : comparée insensible à la casse au chemin du fichier de manifeste. Le chemin est généralement absolu, donc ancrez le motif à la fin plutôt qu'au début. Les chemins ne sont pas normalisés par séparateur pour ce signal, donc les chemins Windows utilisent des barres obliques inverses.
* `pattern` : comparée sensible à la casse au contenu de ce fichier.

L'exemple suivant utilise `manifestDeps` pour suggérer votre plugin une fois que Claude a lu un `package.json` qui dépend du package npm de votre SDK, nommé `your-sdk` ici.

```json theme={null}
{
  "name": "your-plugin",
  "source": "./plugins/your-plugin",
  "relevance": {
    "signals": {
      "manifestDeps": [
        {
          "file": "[/\\\\]package\\.json$",
          "pattern": "\"your-sdk\"\\s*:"
        }
      ]
    }
  }
}
```

Dans cet exemple, le motif `file` utilise `[/\\\\]` pour qu'il corresponde à la fois aux séparateurs de chemin barre oblique avant et barre oblique inverse, et `\\.` pour que le point soit littéral. En JSON, chaque barre oblique inverse dans l'expression régulière est écrite deux fois.

<h2 id="validate-your-marketplace">
  Valider votre marketplace
</h2>

Dans votre shell, exécutez `claude plugin validate` sur votre répertoire de marketplace pour vérifier le bloc `relevance` avant la publication :

```bash theme={null}
claude plugin validate ./my-marketplace
```

Le validateur signale les erreurs et les avertissements sur le bloc `relevance`, y compris ceux-ci :

* Signale les clés inconnues sous `relevance` et `relevance.signals` comme des avertissements
* Signale une valeur `relevance` qui n'est pas un objet
* Rejette une entrée `signals.hosts` qui inclut un schéma, un port ou un chemin

Chaque résultat s'affiche avec le chemin du champ qu'il concerne, et la sortie se termine par `Validation passed`, `Validation passed with warnings` ou `Validation failed`.

<h2 id="enable-suggestions-in-managed-settings">
  Activer les suggestions dans les paramètres gérés
</h2>

Les utilisateurs ne voient aucune suggestion d'une marketplace tant qu'un administrateur ne l'autorise pas dans les [paramètres gérés](/docs/fr/plugins/org), même lorsque son `marketplace.json` déclare `relevance`.

Pour autoriser une marketplace, modifiez vos paramètres gérés comme suit :

* Ajoutez le nom de la marketplace à `pluginSuggestionMarketplaces`.
* Pour toute marketplace autre que la marketplace officielle d'Anthropic, déclarez également la source de la marketplace, soit comme entrée de ce nom dans [`extraKnownMarketplaces`](/docs/fr/plugins/org#require-a-marketplace-and-its-plugins), soit comme entrée dans [`strictKnownMarketplaces`](/docs/fr/plugins/org#allowlist-with-strictknownmarketplaces).

Sur une machine où la marketplace n'est pas enregistrée, ou est enregistrée sous le nom autorisé à partir d'une source différente, aucune suggestion de celle-ci n'apparaît. La vérification de la source empêche une source non liée de s'enregistrer sous un nom autorisé pour que ses plugins soient suggérés dans toute votre organisation.

Le `managed-settings.json` suivant enregistre une marketplace d'organisation à partir d'un référentiel GitHub et active ses suggestions :

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": {
        "source": "github",
        "repo": "your-org/your-marketplace"
      }
    }
  },
  "pluginSuggestionMarketplaces": ["your-marketplace"]
}
```

Le nom de la marketplace officielle ne peut s'enregistrer que depuis la source Anthropic officielle, il n'a donc besoin d'aucune déclaration de source. Pour la marketplace officielle, autorisez le nom seul :

```json theme={null}
{
  "pluginSuggestionMarketplaces": ["claude-plugins-official"]
}
```

<h2 id="preview-what-the-user-sees">
  Aperçu de ce que l'utilisateur voit
</h2>

Lorsque le signal `relevance` d'un plugin correspond pendant une session, le conseil sous le spinner se lit comme suit :

```text theme={null}
Working with Terraform? Install the terraform-helpers plugin:
/plugin install terraform-helpers@your-marketplace
```

Lorsqu'un signal `cwd` correspond au démarrage de la session, la notification d'une ligne se lit comme suit :

```text theme={null}
plugin suggestion: terraform-helpers@your-marketplace · /plugin
```

Dans l'onglet Discover de `/plugin`, le plugin est épinglé au-dessus des autres résultats avec une annotation qui nomme le signal correspondant, tel que `suggested for this directory` ou `suggested for terraform commands`.

Claude Code limite la fréquence à laquelle il suggère un plugin donné :

* La suggestion apparaît au maximum une fois tous les trois sessions, combinant le conseil du spinner et la notification de démarrage de session.
* La notification de démarrage de session cesse d'apparaître une fois que le conseil du spinner et la notification ont montré le plugin un total combiné de deux fois.
* Ni le conseil du spinner ni la notification de démarrage de session ne se répètent une fois que le plugin est installé.
* L'onglet Discover épingle le plugin la première fois que l'utilisateur ouvre l'onglet tandis que les signaux du plugin correspondent. Claude Code enregistre cela dans `~/.claude.json`, de sorte que chaque fois ultérieure que l'utilisateur ouvre `/plugin` sur cette machine, le plugin apparaît dans l'ordre normal.

<h2 id="see-also">
  Voir aussi
</h2>

* [Héberger une marketplace](/docs/fr/plugins/host-marketplace) : exécutez la marketplace qui héberge vos plugins
* [Référence de marketplace](/docs/fr/plugins/marketplace-reference#plugin-entries) : chaque champ qu'une entrée de plugin accepte
* [Recommander votre plugin depuis votre CLI](/docs/fr/plugins/cli-hints) : invitez les utilisateurs depuis votre propre CLI au lieu des signaux de session de Claude Code
* [Gérer les plugins pour votre organisation](/docs/fr/plugins/org) : `extraKnownMarketplaces`, `strictKnownMarketplaces` et le reste des clés de politique de plugins
