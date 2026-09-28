> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Publier et distribuer un plugin

> Publiez un plugin Claude Code via votre propre marketplace ou la marketplace communautaire d'Anthropic, avec une checklist de pré-lancement et comment les utilisateurs reçoivent les mises à jour.

Publier un plugin Claude Code signifie le lister dans une marketplace, un catalogue JSON qui répertorie les plugins et où récupérer chacun d'eux, afin que d'autres personnes puissent l'installer par nom et recevoir vos mises à jour. Vous pouvez gérer votre propre marketplace ou soumettre votre plugin à la marketplace communautaire d'Anthropic. Pour partager un plugin sans le publier, envoyez aux gens le répertoire du plugin ou un `.zip` de celui-ci à charger eux-mêmes.

Cette page s'adresse à l'auteur d'un plugin fonctionnel qui est prêt à le partager.

<Note>
  Ces cas sont couverts sur d'autres pages :

  * **Votre plugin n'est pas encore terminé** : commencez par [Créer un plugin](/docs/fr/plugins/create)
  * **Vous maintenez une CLI ou un SDK avec un plugin dans une marketplace officielle** : voir [Recommander votre plugin depuis votre CLI](/docs/fr/plugins/cli-hints)
</Note>

Commencez par [Choisir comment distribuer](#choose-how-to-distribute) pour comparer les options de distribution. Si vous connaissez déjà votre route, allez à [Préparer votre plugin pour la sortie](#prepare-your-plugin-for-release), puis suivez la section de votre route pour savoir quoi dire à vos utilisateurs et comment ils reçoivent vos mises à jour.

<h2 id="choose-how-to-distribute">
  Choisir comment distribuer
</h2>

Choisissez une option de distribution en fonction de qui doit installer le plugin :

| Route                                                                         | Qui peut installer                                                                                     | Ce dont vous avez besoin                                                                                  | Les utilisateurs reçoivent-ils vos mises à jour automatiquement ? |
| :---------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------- |
| [Pas de marketplace](#share-a-plugin-without-a-marketplace)                   | Les personnes à qui vous envoyez le dossier du plugin ou un `.zip` de celui-ci                         | Le dossier du plugin                                                                                      | Aucune. Ils chargent la copie que vous avez envoyée               |
| [Votre propre marketplace](#publish-through-your-own-marketplace)             | Quiconque peut accéder au référentiel, qui peut être un référentiel privé que votre équipe peut cloner | Un référentiel git ou un autre hôte avec un `.claude-plugin/marketplace.json` qui répertorie votre plugin | Désactivé                                                         |
| [Marketplace communautaire d'Anthropic](#submit-to-the-community-marketplace) | Quiconque ajoute `anthropics/claude-plugins-community`                                                 | Une soumission via le formulaire de soumission du répertoire de plugins                                   | Désactivé                                                         |

La mise à jour automatique est un paramètre par marketplace du côté de l'utilisateur qui récupère les nouvelles versions en arrière-plan.

<h2 id="prepare-your-plugin-for-release">
  Préparer votre plugin pour la sortie
</h2>

Le nom, la version, la validation et une installation à partir d'une marketplace décident si une sortie fonctionne pour les personnes qui l'installent. Vérifiez-les avant la première sortie et à nouveau avant chaque sortie ultérieure.

<Steps>
  <Step title="Choisir un nom permanent">
    Les utilisateurs installent, activent et configurent votre plugin par `name@marketplace`, donc un plugin renommé est un plugin différent pour chaque installation existante. Choisissez un nom en kebab-case comme `deploy-helper`, car `claude plugin validate` avertit sur d'autres formes, et traitez-le comme permanent. Définissez `displayName` dans `plugin.json` pour le libellé que les utilisateurs voient.
  </Step>

  <Step title="Décider comment vous allez versionner">
    Si vous définissez `version` dans `plugin.json` et que vous poussez ultérieurement des commits sans la modifier, `claude plugin update` affiche `<name> is already at the latest version (1.0.0).` et les utilisateurs conservent l'ancienne copie. Soit vous incrémentez `version` à chaque sortie, soit vous l'omettez dans une marketplace hébergée sur git afin que Claude Code utilise le SHA du commit à la place. Voir [Versions et mises à jour](/docs/fr/plugins/loading#versions-and-updates).
  </Step>

  <Step title="Valider">
    Dans votre shell, exécutez `claude plugin validate --strict ./your-plugin`. Une exécution propre affiche `✔ Validation passed`.

    * **En CI** : conservez `--strict`, qui échoue également l'exécution avec le code de sortie 1 sur les avertissements tels qu'un champ de manifeste inconnu ou une `version` manquante. Supprimez `--strict` si vous avez choisi d'omettre `version` à l'étape précédente.
    * **Chemins** : la validation signale les chemins de composants qui ne commencent pas par `./`. À l'intérieur des commandes hook et des configurations du serveur MCP, référencez les fichiers comme `${CLAUDE_PLUGIN_ROOT}/...`. Voir [règles de chemin](/docs/fr/plugins/manifest-reference#path-rules).
  </Step>

  <Step title="L'installer à partir d'une marketplace locale">
    Dans votre shell, ajoutez une marketplace locale qui répertorie le plugin avec `claude plugin marketplace add ./path-to-marketplace`, installez le plugin à partir de celle-ci, et démarrez une session pour confirmer qu'il se charge.

    * Pour la plus petite marketplace qui fonctionne, voir [Créer une marketplace](/docs/fr/plugins/create-marketplace).
    * Pour savoir si une installation charge votre répertoire source ou une copie en cache, voir [Plugins en place et copiés](/docs/fr/plugins/loading#in-place-and-copied-plugins).
  </Step>

  <Step title="Remplir les métadonnées que les utilisateurs voient">
    Définissez `description`, `author`, `homepage` et `repository` dans `plugin.json`, et ajoutez un `README.md` à la racine du plugin. `homepage` doit être analysable en tant qu'URL. La [référence du manifeste](/docs/fr/plugins/manifest-reference#fields) répertorie tous les champs.
  </Step>

  <Step title="Exécuter votre suite d'évaluation">
    Si vous avez une suite d'évaluation, exécutez `claude plugin eval` dans votre shell. Elle exécute les cas de test du plugin et note les résultats, ce qui détecte les régressions lorsque vous modifiez le plugin. Voir [Tester les plugins avec des évaluations](/docs/fr/plugin-evals).
  </Step>
</Steps>

<h2 id="share-a-plugin-without-a-marketplace">
  Partager un plugin sans marketplace
</h2>

Si le plugin se trouve dans un référentiel git, les gens peuvent le cloner et charger le checkout, ou démarrer Claude Code à partir de leur shell avec `--plugin-url` pointant vers un `.zip` que vous joignez à une sortie. Pour obtenir votre prochaine version, ils tirent ou téléchargent à nouveau. S'il ne se trouve pas dans un référentiel, envoyez-leur le répertoire ou un `.zip` de celui-ci. Ils le chargent de l'une des deux façons suivantes :

* **Pour une session** : ils démarrent Claude Code à partir de leur shell avec `claude --plugin-dir ./deploy-helper`, où le chemin est le clone, le dossier décompressé ou le `.zip` lui-même. Voir [Drapeaux qui chargent un plugin pour une session](/docs/fr/plugins/cli-reference#flags-that-load-a-plugin-for-one-session).
* **Pour chaque session** : ils déplacent le répertoire du plugin, avec son `.claude-plugin/plugin.json`, sous `~/.claude/skills/` afin que Claude Code [le charge dans chaque session](/docs/fr/plugins/loading#find-where-a-plugin-came-from).

L'ajout d'un `.claude-plugin/marketplace.json` à ce même référentiel est ce qui permet aux gens d'installer par nom et de mettre à jour avec une commande ; voir [Publier via votre propre marketplace](#publish-through-your-own-marketplace).

<h3 id="ship-a-plugin-with-your-own-tool">
  Livrer un plugin avec votre propre outil
</h3>

Si vous maintenez une CLI ou un SDK, publiez le plugin dans une marketplace et faites en sorte que votre installateur ou message post-installation exécute ou imprime les deux commandes dont un utilisateur a besoin : `claude plugin marketplace add <source>`, puis `claude plugin install <name>@<marketplace>`. Pour la découverte en session lorsque quelqu'un utilise votre outil, voir [Recommander votre plugin depuis votre CLI](/docs/fr/plugins/cli-hints).

<h2 id="publish-through-your-own-marketplace">
  Publier via votre propre marketplace
</h2>

Votre propre marketplace est un fichier `.claude-plugin/marketplace.json` qui répertorie votre plugin, ajouté à un référentiel git. Une fois le fichier dans le référentiel, le plugin est publié, sans formulaire de soumission. Vous pouvez conserver le fichier dans le propre référentiel du plugin ou dans un référentiel séparé.

<h3 id="add-the-marketplace-file-to-your-repository">
  Ajouter le fichier marketplace à votre référentiel
</h3>

Pour publier à partir du propre référentiel du plugin, enregistrez le fichier marketplace à côté de `plugin.json` dans `.claude-plugin/`, avec une entrée dont la `source` est `"./"`, la racine du référentiel. Donnez à l'entrée le même `name` que `plugin.json`, selon [Garder le nom de l'entrée et le nom du manifeste identiques](/docs/fr/plugins/create-marketplace#keep-the-entry-name-and-the-manifest-name-the-same) :

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Name" },
  "plugins": [
    { "name": "deploy-helper", "source": "./" }
  ]
}
```

Dans votre shell, exécutez `claude plugin validate .` dans le référentiel pour vérifier le fichier avant de le pousser.

[Créer une marketplace](/docs/fr/plugins/create-marketplace) couvre la disposition avec plusieurs plugins dans un référentiel.

<h3 id="control-who-can-install">
  Contrôler qui peut installer
</h3>

Quiconque peut cloner le référentiel peut installer à partir de celui-ci, donc si le référentiel est privé, la marketplace l'est aussi. Pour les hôtes autres qu'un référentiel git, voir [Héberger une marketplace](/docs/fr/plugins/host-marketplace). Pour atteindre tout le monde dans une entreprise, y compris les personnes qui n'utilisent pas git, voir [Déployer dans toute une entreprise](/docs/fr/plugins/host-marketplace#roll-out-to-a-whole-company).

<h3 id="tell-users-how-to-install">
  Dire aux utilisateurs comment installer
</h3>

Dites à vos utilisateurs d'ajouter la marketplace puis d'installer le plugin à partir de leur shell, en remplaçant la source et les noms par les vôtres :

* Ajouter la marketplace une fois : `claude plugin marketplace add your-org/your-marketplace`, où l'argument est un raccourci GitHub `owner/repo`, une URL ou un chemin
* Installer le plugin : `claude plugin install deploy-helper@your-marketplace`
* Ou faire les deux à partir d'une session : `/plugin install deploy-helper --marketplace your-org/your-marketplace`. Nécessite Claude Code v2.1.275 ou ultérieur. Voir [Ajouter une marketplace et installer en une commande](/docs/fr/plugins/install#add-a-marketplace-and-install-in-one-command)

<h3 id="ship-updates-to-users">
  Livrer les mises à jour aux utilisateurs
</h3>

Les utilisateurs reçoivent une sortie lorsqu'ils la demandent ou lorsque la mise à jour automatique est activée pour votre marketplace :

* **Sur demande** : `claude plugin update deploy-helper@your-marketplace` dans le shell de l'utilisateur actualise la marketplace et installe la nouvelle copie lorsque la version de votre plugin a changé
* **Mise à jour automatique** : désactivée par défaut pour votre marketplace. Voir [Activer la mise à jour automatique](/docs/fr/plugins/host-marketplace#turn-on-auto-update). Une fois activée, elle fait la même chose que `claude plugin update` avec un délai après le démarrage de la session

[Installer les plugins](/docs/fr/plugins/install) couvre les commandes du côté utilisateur, et [quand la mise à jour automatique s'exécute](/docs/fr/plugins/loading#when-auto-update-runs) couvre le timing.

<h2 id="submit-to-the-community-marketplace">
  Soumettre à la marketplace communautaire
</h2>

La marketplace communautaire d'Anthropic, `claude-community`, est la marketplace publique qui répertorie les plugins soumis via le formulaire de soumission du répertoire de plugins.

Les utilisateurs ajoutent la marketplace communautaire dans une session Claude Code avec `/plugin marketplace add anthropics/claude-plugins-community` et installent à partir de celle-ci comme `@claude-community`.

Pour savoir comment la marketplace communautaire diffère de la marketplace officielle, voir [Marketplaces d'Anthropic](/docs/fr/plugins/anthropic-marketplaces).

Pour soumettre votre plugin à la marketplace communautaire, utilisez l'un des formulaires intégrés à l'application :

* **claude.ai** : [claude.ai/admin-settings/directory/submissions/plugins/new](https://claude.ai/admin-settings/directory/submissions/plugins/new)
* **Console** : [platform.claude.com/plugins/submit](https://platform.claude.com/plugins/submit)

Le formulaire claude.ai nécessite une organisation Team ou Enterprise et la permission Directory, que les propriétaires détiennent par défaut. Les auteurs individuels qui ne font pas partie d'une organisation Team ou Enterprise peuvent utiliser le formulaire Console à la place.

Dans votre shell, exécutez `claude plugin validate ./your-plugin` localement avant de soumettre, en remplaçant `./your-plugin` par le chemin vers votre répertoire de plugins. Lorsque la validation réussit, Claude Code affiche `✔ Validation passed`, ou `✔ Validation passed with warnings` s'il y a des avertissements. Les avertissements ne font pas échouer la validation ; ajoutez `--strict` pour les traiter comme des erreurs.

Les plugins listés apparaissent dans le catalogue [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community), dans presque tous les cas épinglés à un SHA de commit spécifique.

Il peut y avoir un délai entre la soumission et l'apparition de votre plugin dans `marketplace.json`. Pour vérifier si votre plugin est installable, recherchez son nom dans le [catalogue communautaire](https://github.com/anthropics/claude-plugins-community/blob/main/.claude-plugin/marketplace.json).

La marketplace officielle, `claude-plugins-official`, n'accepte pas les soumissions via ces formulaires. Si vous travaillez avec un contact partenaire d'Anthropic, demandez-lui un listing de marketplace officielle.

<h2 id="ship-updates-renames-and-removals">
  Livrer les mises à jour, les renommages et les suppressions
</h2>

<h3 id="release-a-new-version">
  Sortir une nouvelle version
</h3>

Si vous publiez via votre propre marketplace et que votre `plugin.json` définit `version`, incrémentez-la et poussez. Les utilisateurs qui exécutent `claude plugin update` ou qui ont la mise à jour automatique activée reçoivent alors la nouvelle version, comme décrit sous [Livrer les mises à jour aux utilisateurs](#ship-updates-to-users).

<h3 id="tag-a-release">
  Étiqueter une sortie
</h3>

Étiquetez la sortie dans git lorsque d'autres plugins déclarent une plage de version sur la vôtre, car ces plages se résolvent par rapport aux étiquettes. Sinon, vous n'avez pas besoin d'une étiquette.

Pour étiqueter, exécutez `claude plugin tag` dans votre shell à partir du répertoire du plugin. Elle crée une étiquette `{name}--v{version}`. Ajoutez `--push` pour envoyer l'étiquette à `origin`. La [référence `plugin tag`](/docs/fr/plugins/cli-reference#plugin-tag) répertorie ses drapeaux.

<h3 id="rename-or-remove-a-plugin">
  Renommer ou supprimer un plugin
</h3>

Ne modifiez jamais le `name` d'un plugin publié. Après un renommage, les utilisateurs qui l'ont déjà installé perdent le plugin, car leur installation est enregistrée sous l'ancien nom. Une entrée `renames` dans votre fichier marketplace les migre à la place. Modifiez `displayName` lorsque vous voulez un libellé différent.

Si un renommage est inévitable, utilisez la carte `renames` du fichier marketplace afin que les installations existantes migrent au lieu d'échouer avec [`Plugin "<name>" not found in marketplace`](/docs/fr/plugins/troubleshooting#plugin-not-found-in-marketplace). Pour supprimer un plugin de la marketplace, ou pour les détails complets de `renames`, voir [Renommer ou supprimer un plugin](/docs/fr/plugins/host-marketplace#rename-or-remove-a-plugin) sur la page d'hébergement. La [référence marketplace](/docs/fr/plugins/marketplace-reference#top-level-fields) a le champ.

<h2 id="declare-dependencies">
  Déclarer les dépendances
</h2>

Si votre plugin a besoin d'un autre plugin de la même marketplace pour être activé, listez-le dans le tableau `dependencies` de `plugin.json`. Chaque entrée est un nom nu ou un objet avec une plage de version semver `version`. Lorsqu'un utilisateur installe votre plugin, Claude Code installe et active également la dépendance.

[Dépendances des plugins](/docs/fr/plugins/dependencies) couvre la syntaxe de plage, les dépendances inter-marketplace et comment les utilisateurs élaguent les dépendances dont ils n'ont plus besoin.

<h2 id="next-steps">
  Étapes suivantes
</h2>

* [Héberger et maintenir une marketplace](/docs/fr/plugins/host-marketplace) : sortir de nouvelles versions et tenir les utilisateurs à jour
* [Dépendances des plugins](/docs/fr/plugins/dependencies) : déclarer et versionner les plugins sur lesquels le vôtre dépend
* [Recommander votre plugin depuis votre CLI](/docs/fr/plugins/cli-hints) : inviter les utilisateurs Claude Code de votre CLI à installer le plugin
* [Mesurer le coût et l'utilisation des plugins](/docs/fr/plugins/measure) : voir ce que votre plugin coûte en contexte et si les gens l'utilisent
