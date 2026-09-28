> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Créer une marketplace

> Créez une marketplace de plugins à partir d'un fichier marketplace.json et testez-la localement avant de l'héberger.

Une marketplace de plugins est un répertoire ou un dépôt contenant un fichier `.claude-plugin/marketplace.json` qui répertorie vos plugins et indique où récupérer chacun d'eux. Vous poussez le répertoire vers un hôte git, et toute personne ayant accès l'enregistre dans Claude Code avec une seule commande et installe vos plugins à partir de celui-ci.

Créez votre propre marketplace lorsque vous souhaitez qu'un groupe que vous choisissez, comme votre équipe ou votre organisation, installe vos plugins et continue à recevoir vos mises à jour à partir d'un catalogue que vous contrôlez. Le dépôt peut être privé, il peut répertorier autant de plugins que vous le souhaitez, et un administrateur peut [l'exiger sur chaque machine](/docs/fr/plugins/org).

<Note>
  Ces cas sont couverts sur d'autres pages :

  * **Partager un plugin avec quelques personnes** : envoyez-leur le répertoire du plugin ou un `.zip` de celui-ci. Voir [Partager un plugin sans marketplace](/docs/fr/plugins/publish#share-a-plugin-without-a-marketplace).
  * **Proposer un plugin à tout le monde** : soumettez-le à la marketplace communautaire d'Anthropic. Voir [Soumettre à la marketplace communautaire](/docs/fr/plugins/publish#submit-to-the-community-marketplace).
  * **Utiliser un plugin vous-même** : chargez-le avec `--plugin-dir` ou enregistrez-le dans votre répertoire de compétences. Voir [Développer sans marketplace](/docs/fr/plugins/create#develop-without-a-marketplace).
</Note>

Commencez par [Créer une marketplace](#create-a-marketplace) pour en construire une sur votre propre machine et installer un plugin à partir de celle-ci, puis [ajoutez d'autres entrées de plugins](#add-plugin-entries).

<h2 id="create-a-marketplace">
  Créer une marketplace
</h2>

Les étapes suivantes créent une marketplace sur votre machine, ajoutent un plugin à celle-ci, l'enregistrent dans Claude Code et installent le plugin à partir de celle-ci. C'est la boucle complète, et c'est la même boucle que vos utilisateurs parcourent une fois que vous hébergez la marketplace quelque part où ils peuvent y accéder. Exécutez chaque commande dans votre shell, à partir du répertoire où vous souhaitez que `my-marketplace/` soit créé.

Vous avez besoin d'un plugin à répertorier. L'exemple utilise `my-first-plugin` de [Créer votre premier plugin](/docs/fr/plugins/create#create-your-first-plugin), un plugin avec une compétence que vous exécutez en tant que `/my-first-plugin:hello` ; construisez-le d'abord si vous n'avez pas encore de plugin. Pour utiliser un plugin qui vous appartient à la place, remplacez son répertoire et son `name` partout où les étapes disent `my-first-plugin`. Pour savoir ce qu'un répertoire de plugins peut contenir, voir l'[explorateur de répertoire de plugins](/docs/fr/plugins/components#explore-the-plugin-directory).

<Steps>
  <Step title="Configurer le répertoire de la marketplace">
    Une marketplace est un répertoire avec un fichier `.claude-plugin/marketplace.json`, plus les plugins qu'il répertorie. Créez le répertoire de la marketplace et son dossier `.claude-plugin/`, puis copiez votre plugin sous `plugins/` :

    ```bash theme={null}
    mkdir -p my-marketplace/.claude-plugin my-marketplace/plugins
    cp -r my-first-plugin my-marketplace/plugins/
    ```

    Vérifiez que le plugin est valide là où il se trouve maintenant, afin que toute erreur ultérieure concerne la marketplace et non le plugin :

    ```bash theme={null}
    claude plugin validate ./my-marketplace/plugins/my-first-plugin
    ```

    La dernière ligne de la sortie lit `✔ Validation passed`.
  </Step>

  <Step title="Créer le fichier de marketplace">
    Enregistrez `marketplace.json` à `my-marketplace/.claude-plugin/marketplace.json`. Le fichier nécessite un `name`, un `owner` et un tableau `plugins`.

    Chaque objet dans `plugins` est une entrée de plugin et a besoin d'un `name` et d'une `source`. Écrivez la `source` de l'entrée comme un chemin à partir de la racine de la marketplace. La racine est `my-marketplace/`, le répertoire qui contient `.claude-plugin/`.

    ```json my-marketplace/.claude-plugin/marketplace.json theme={null}
    {
      "name": "my-marketplace",
      "description": "Plugins for my team",
      "owner": {
        "name": "Your Name"
      },
      "plugins": [
        {
          "name": "my-first-plugin",
          "source": "./plugins/my-first-plugin",
          "description": "A greeting plugin to learn the basics"
        }
      ]
    }
    ```
  </Step>

  <Step title="Valider la marketplace">
    Exécutez `claude plugin validate` sur le répertoire de la marketplace pour vérifier la syntaxe JSON, les champs obligatoires et chaque entrée de plugin dans son `.claude-plugin/marketplace.json`.

    ```bash theme={null}
    claude plugin validate ./my-marketplace
    ```

    Pour le fichier tel qu'écrit à l'étape 2, la dernière ligne de la sortie lit `✔ Validation passed`.
  </Step>

  <Step title="Ajouter la marketplace et installer le plugin">
    Enregistrez le répertoire en tant que marketplace.

    ```bash theme={null}
    claude plugin marketplace add ./my-marketplace
    ```

    La commande affiche `✔ Successfully added marketplace: my-marketplace (declared in user settings)`, ce qui signifie que la marketplace est enregistrée dans votre fichier de paramètres utilisateur.

    Installez le plugin. L'ID d'installation est le `name` de l'entrée, un `@` et le `name` de la marketplace.

    ```bash theme={null}
    claude plugin install my-first-plugin@my-marketplace
    ```

    La commande affiche `✔ Successfully installed plugin: my-first-plugin@my-marketplace (scope: user)`.

    À l'intérieur d'une session, `/plugin marketplace add ./my-marketplace` enregistre la marketplace de la même manière. `/plugin install my-first-plugin@my-marketplace` ouvre les détails du plugin dans le panneau `/plugin`, où vous l'installez. Pour ce flux, voir [Installer et gérer les plugins](/docs/fr/plugins/install).
  </Step>

  <Step title="Confirmer que le plugin a été chargé">
    Listez les plugins installés.

    ```bash theme={null}
    claude plugin list
    ```

    La sortie répertorie `my-first-plugin@my-marketplace` avec `Status: ✔ enabled`.

    Pour voir ce que le plugin a chargé, affichez ses détails.

    ```bash theme={null}
    claude plugin details my-first-plugin
    ```

    La section `Component inventory` lit `Skills (1)  hello`.

    Pour exécuter la compétence, démarrez une session et entrez `/my-first-plugin:hello`. Claude vous salue. La commande a le nom du plugin comme préfixe, comme le fait le nom de chaque compétence de plugin.
  </Step>
</Steps>

<h2 id="add-plugin-entries">
  Ajouter des entrées de plugins
</h2>

Chaque plugin que vous distribuez est un objet dans le tableau `plugins` de `marketplace.json`. Pour ajouter un deuxième plugin, ajoutez un deuxième objet. Ces champs couvrent la plupart des entrées :

* `name` : l'identifiant que les gens tapent avant `@` lorsqu'ils installent. Il ne peut pas contenir d'espaces.
* `source` : où Claude Code récupère le plugin. Écrivez une chaîne de chemin relatif pour un plugin à l'intérieur du répertoire de la marketplace, comme dans [la procédure pas à pas](#create-a-marketplace), ou un objet source pour un plugin en dehors de celui-ci. Voir [Choisir une source de plugin](#choose-a-plugin-source).
* `description` : la ligne que les gens voient à côté du plugin lorsqu'ils parcourent votre marketplace dans `/plugin`.

Pour la liste complète des champs, voir [Entrées de plugins](/docs/fr/plugins/marketplace-reference#plugin-entries).

Une entrée peut également définir n'importe quel champ [`plugin.json`](/docs/fr/plugins/manifest-reference). Pour savoir quand les champs `plugin.json` d'une entrée s'appliquent à un plugin qui a son propre `plugin.json`, voir [Entrée et plugin.json](/docs/fr/plugins/marketplace-reference#entry-and-plugin-json).

<h2 id="rules-for-plugin-entries">
  Règles pour les entrées de plugins
</h2>

La plupart des installations échouées à partir d'une nouvelle marketplace proviennent d'un chemin relatif écrit à partir du mauvais répertoire, ou d'un nom d'entrée qui diffère du `name` dans le `plugin.json` du plugin.

<h3 id="write-relative-paths-from-the-marketplace-root">
  Écrire les chemins relatifs à partir de la racine de la marketplace
</h3>

La racine de la marketplace est le répertoire qui contient `.claude-plugin/`. Dans [la procédure pas à pas](#create-a-marketplace), c'est `my-marketplace/`, donc la `source` de l'entrée est `"./plugins/my-first-plugin"`. Le chemin ne commence pas à l'intérieur de `.claude-plugin/`, donc n'utilisez pas `..` pour le quitter.

Un chemin avec `..` et un chemin vers un répertoire manquant échouent à des commandes différentes :

* **Un chemin avec `..`** : `claude plugin validate` signale l'entrée comme invalide. Le message commence par `Path contains "..": ./../plugins/my-first-plugin`.
* **Un chemin vers un répertoire qui n'existe pas** : `claude plugin validate` réussit. `claude plugin install` échoue avec `Source path does not exist: <path>`, et `<path>` est l'emplacement absolu que Claude Code a vérifié.

<h3 id="keep-the-entry-name-and-the-manifest-name-the-same">
  Garder le nom de l'entrée et le nom du manifeste identiques
</h3>

Un plugin de marketplace a un `name` d'entrée dans `marketplace.json` et un `name` dans son propre `plugin.json`, appelé le nom du manifeste. Chaque nom apparaît à des endroits différents :

* **Nom de l'entrée** : l'ID d'installation, `<entry-name>@<marketplace>`. C'est ce que les gens tapent pour installer, ce que `claude plugin list` affiche, et la clé que Claude Code écrit sous [`enabledPlugins`](/docs/fr/settings-reference#enabledplugins) dans leur fichier de paramètres.
* **Nom du manifeste** : le préfixe sur les compétences du plugin, et le nom que `claude plugin details` prend.

Lorsque les deux noms diffèrent et que quelqu'un installe par le nom du manifeste, Claude Code signale `Plugin "<manifest-name>" not found in marketplace "<marketplace>"`. Gardez les deux noms identiques. Pour plus d'informations sur la façon dont Claude Code utilise les deux noms, voir [Référence de chargement des plugins](/docs/fr/plugins/loading#find-where-a-plugin-came-from).

<h2 id="choose-a-plugin-source">
  Choisir une source de plugin
</h2>

Chaque entrée de plugin dans `marketplace.json` a une `source` qui indique à Claude Code où récupérer ce plugin. Choisissez la source en fonction de l'endroit où les fichiers du plugin sont stockés. Le tableau répertorie les sources que la plupart des propriétaires de marketplace utilisent.

| Source         | Utilisez-la quand                                                                         | Valeur `source` minimale                                                                  |
| :------------- | :---------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------- |
| Chemin relatif | Les fichiers du plugin se trouvent à l'intérieur du répertoire de la marketplace lui-même | `"./plugins/my-first-plugin"`                                                             |
| `github`       | Le plugin est son propre dépôt GitHub                                                     | `{ "source": "github", "repo": "your-org/my-first-plugin" }`                              |
| `git-subdir`   | Le plugin est un sous-répertoire d'un autre dépôt, comme un monorepo                      | `{ "source": "git-subdir", "url": "your-org/monorepo", "path": "tools/my-first-plugin" }` |

Dans une source `git-subdir`, `url` prend une URL git ou un raccourci GitHub `owner/repo`.

Un plugin peut également provenir de l'un de ces types de source :

* `url` : un dépôt git par URL, sur n'importe quel hôte
* `archive` : un fichier zip téléchargé via HTTPS
* `npm` : un package npm
* `command` : un répertoire produit en exécutant une commande sur la machine où le plugin est installé

Pour les champs de chaque type de source, et pour épingler une source basée sur git à une `ref` ou `sha`, voir [Sources de plugins](/docs/fr/plugins/marketplace-reference#plugin-sources).

<h2 id="validate-and-test">
  Valider et tester
</h2>

À mesure que vous ajoutez des plugins, exécutez `claude plugin validate ./my-marketplace` dans votre shell après chaque modification, et installez à partir de la marketplace sur votre propre machine avant de la partager. La validation et l'installation détectent des problèmes différents.

<h3 id="problems-that-validation-reports">
  Problèmes que la validation signale
</h3>

`claude plugin validate` lit uniquement les fichiers à l'intérieur du répertoire de la marketplace. Il signale :

* Les erreurs de syntaxe JSON, comme `json: Invalid JSON syntax: <reason>`
* Les champs obligatoires manquants, tels que `owner: Invalid input`
* Un nom de marketplace avec des espaces, des caractères non-ASCII, ou une forme qui imite une marketplace officielle d'Anthropic, comme `claude-official`
* Une `source` relative qui contient `..`
* Les champs inconnus au niveau supérieur ou dans une entrée de plugin, comme des avertissements
* Les problèmes dans le `plugin.json` de chaque plugin à chemin relatif, comme `plugins[N] plugin.json → <field>: <message>`

Pour chaque message que `validate` peut imprimer, voir [Messages de validation](/docs/fr/plugins/marketplace-reference#validation-messages). Pour ses drapeaux et codes de sortie, voir [`plugin validate`](/docs/fr/plugins/cli-reference#plugin-validate).

<h3 id="problems-that-surface-when-you-add-or-install">
  Problèmes qui apparaissent lorsque vous ajoutez ou installez
</h3>

Les problèmes que `claude plugin validate` ne signale pas apparaissent lorsque vous ajoutez la marketplace ou installez à partir de celle-ci :

* **Lorsque vous ajoutez la marketplace** : les [noms de marketplace officiels](/docs/fr/plugins/marketplace-reference#reserved-names) exacts, tels que `claude-plugins-official`, passent la validation. Lorsque vous ajoutez une marketplace avec l'un de ces noms, Claude Code la refuse avec un message qui commence par `The name '<name>' is reserved for official Anthropic marketplaces`.
* **Lorsque vous installez un plugin** :
  * Claude Code récupère d'abord une source `github`, `git-subdir` ou autre source distante lorsque vous installez le plugin, donc un `repo` ou `path` incorrect apparaît alors.
  * Une `source` relative dont le répertoire n'existe pas échoue également à l'installation, avec `Source path does not exist: <path>`.

<h3 id="test-an-edit-to-a-plugin">
  Tester une modification d'un plugin
</h3>

Dans [la procédure pas à pas](#create-a-marketplace), vous avez ajouté `my-marketplace` à partir d'un répertoire local avec une `source` à chemin relatif. Avec cette configuration, Claude Code lit les fichiers du plugin directement à partir de `my-marketplace/plugins/`. Vos modifications prennent effet au prochain démarrage de session ou lorsque vous exécutez `/reload-plugins` dans une session, sans modification de la `version` du plugin.

Les personnes qui installent à partir de votre marketplace hébergée obtiennent une copie dans le cache des plugins à la place. Pour savoir comment elles reçoivent une nouvelle version, voir [Garder les utilisateurs à jour](/docs/fr/plugins/host-marketplace#keep-users-up-to-date).

<h3 id="remove-the-marketplace-to-start-over">
  Supprimer la marketplace pour recommencer
</h3>

Pour tout supprimer et recommencer, exécutez `claude plugin marketplace remove my-marketplace` dans votre shell. La commande supprime la marketplace et désinstalle ses plugins.

<h2 id="host-your-marketplace">
  Héberger votre marketplace
</h2>

Une fois que vous pouvez installer un plugin à partir de la marketplace sur votre propre machine, comme dans [Créer une marketplace](#create-a-marketplace), poussez le répertoire de la marketplace vers un hôte git.

Vos coéquipiers exécutent ensuite `claude plugin marketplace add <owner>/<repo>` dans leur shell pour un dépôt GitHub, ou la même commande avec l'URL du dépôt. Ils installent ensuite un plugin par nom comme dans [la procédure pas à pas](#create-a-marketplace).

Pour l'accès aux dépôts privés, les mises à jour, le versioning, et le renommage ou la suppression d'entrées, voir [Héberger et maintenir une marketplace](/docs/fr/plugins/host-marketplace).

<h2 id="next-steps">
  Étapes suivantes
</h2>

* [Héberger et maintenir une marketplace](/docs/fr/plugins/host-marketplace) : choisissez un hôte, gardez les utilisateurs à jour, et renommez ou supprimez les plugins en toute sécurité
* [Référence de marketplace](/docs/fr/plugins/marketplace-reference) : champs `marketplace.json` et types de sources
* [Gérer les plugins pour votre organisation](/docs/fr/plugins/org) : exigez votre marketplace et ses plugins sur chaque machine
* [Suggérer les plugins par pertinence](/docs/fr/plugins/relevance) : faites en sorte que Claude Code suggère un plugin de votre marketplace lorsqu'une session correspond
