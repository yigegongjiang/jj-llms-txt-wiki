> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Installer et gérer les plugins

> Installez les plugins Claude Code à partir d'une marketplace sur n'importe quelle surface que vous utilisez, choisissez une portée d'installation et mettez-les à jour ou supprimez-les ultérieurement.

L'installation d'un plugin ajoute ses skills, agents, hooks et serveurs MCP à Claude Code sur votre machine.

Cette page s'adresse à toute personne utilisant des plugins sur sa propre machine ou son compte, que ce soit dans le terminal, l'application de bureau, un IDE ou une session cloud : elle couvre l'installation, le choix d'une portée, l'ajout de marketplaces et la mise à jour des plugins.

<Note>
  Ces cas sont couverts sur d'autres pages :

  * **Vous utilisez le chat claude.ai ou Cowork, pas Claude Code** : consultez [Plugins sur claude.ai et dans Cowork](https://claude.com/docs/plugins/overview)
  * **Claude Code a affiché une erreur** : trouvez-la dans [Dépanner les plugins](/docs/fr/plugins/troubleshooting)
</Note>

Commencez par [Installer un plugin](#install-a-plugin). Si quelqu'un vous a envoyé une commande d'installation dont le nom `@` n'est pas `claude-plugins-official`, [ajoutez d'abord cette marketplace](#add-a-marketplace).

<h2 id="install-a-plugin">
  Installer un plugin
</h2>

À titre d'exemple, cette section installe [`commit-commands`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/commit-commands) à partir de [la marketplace officielle d'Anthropic](/docs/fr/plugins/anthropic-marketplaces), qui ajoute des commandes pour valider, pousser et ouvrir des demandes de tirage.

Les mêmes étapes installent n'importe quel autre plugin : remplacez son nom et le nom de sa marketplace partout où `commit-commands` et `claude-plugins-official` apparaissent. Si ce plugin provient d'une marketplace différente, [ajoutez d'abord la marketplace](#add-a-marketplace).

Choisissez l'onglet correspondant à l'endroit où vous exécutez Claude Code.

<Tabs>
  <Tab title="Terminal">
    Démarrez Claude Code avec `claude` dans votre projet, puis :

    <Steps>
      <Step title="Ouvrez les détails du plugin avec la commande d'installation">
        Exécutez `/plugin install` avec le nom du plugin et la marketplace. Dans une session, cette commande n'installe pas immédiatement : elle ouvre le panneau `/plugin` sur les détails de ce plugin afin que vous puissiez l'examiner et choisir d'abord une portée.

        ```text theme={null}
        /plugin install commit-commands@claude-plugins-official
        ```

        Pour parcourir à la place, exécutez `/plugin` sans nom de plugin : le panneau s'ouvre sur l'onglet **Discover**, qui répertorie les plugins de chaque marketplace que vous avez ajoutée, et vous pouvez taper pour rechercher, puis appuyer sur **Entrée** sur un plugin pour ouvrir ses détails.
      </Step>

      <Step title="Examinez ce que le plugin ajoute">
        Le volet des détails affiche la description du plugin. Il peut également afficher :

        * **Will install** : les commandes, agents, skills, hooks et serveurs MCP et LSP que le plugin ajoute.
        * **Last updated** : affiché pour un plugin dans la marketplace officielle d'Anthropic.
        * **Context cost** : pour un plugin dans la marketplace officielle d'Anthropic, deux estimations de tokens. **Every turn** est ce que le plugin ajoute à chaque message que vous envoyez, et **When invoked** est ce que ses skills et agents ajoutent une fois que Claude les charge. Les estimations apparaissent lorsque vous ouvrez le plugin en nommant sa marketplace, comme le fait la commande de l'étape 1, ou à partir de l'onglet **Marketplaces**. Le volet des détails auquel vous accédez à partir de la liste **Discover** ne les affiche pas.

        Les plugins d'une marketplace locale ou personnalisée peuvent afficher `Components will be discovered at installation` à la place.

        Un plugin peut exécuter des hooks et des serveurs MCP, alors lisez le volet avant d'installer. Consultez [Plugin security and trust](/docs/fr/plugins/security).
      </Step>

      <Step title="Choisissez une portée">
        Sélectionnez l'une des trois options d'installation :

        * **Install for you (user scope)** : vous obtenez le plugin dans chaque projet sur cette machine
        * **Install for all collaborators on this repository (project scope)** : il est activé pour tous ceux qui travaillent dans ce référentiel
        * **Install for you, in this repo only (local scope)** : vous l'obtenez dans ce référentiel uniquement

        [Choisir une portée d'installation](#choose-an-install-scope) indique quel fichier de paramètres chacun écrit et lequel s'applique lorsque le même plugin est défini à plusieurs niveaux.

        Après avoir sélectionné une portée, Claude Code installe le plugin ainsi que toutes les dépendances qu'il déclare, puis imprime un résumé d'installation.
      </Step>

      <Step title="Lisez le résumé d'installation">
        La dernière phrase du résumé vous indique si le plugin est utilisable dans cette session :

        * **Active now** : `Plugin is now active.` Aucun rechargement n'est nécessaire.
        * **Reload needed** : `Run /reload-plugins to activate.` Le panneau se ferme et Claude Code exécute ce rechargement pour vous. Si le rechargement [invaliderait le cache du prompt](/docs/fr/prompt-caching#enabling-or-disabling-a-plugin), il vous avertit et laisse le plugin en attente à la place. Exécutez `/reload-plugins --force` pour l'activer quand même, ce qui coûte une demande non mise en cache.
        * **Load failed** : `The plugin couldn't be loaded`. Ouvrez l'onglet **Errors** dans `/plugin` pour connaître la raison, puis consultez [After install: plugin not working](/docs/fr/plugins/troubleshooting#plugin-installed-but-not-working).
      </Step>

      <Step title="Confirmez que le plugin fonctionne">
        Tapez `/` et recherchez les skills du plugin sous son nom, sous la forme `/<plugin>:<skill>`. Pour `commit-commands`, `/commit-commands:commit` apparaît. Deux autres endroits répertorient également le plugin :

        * Ouvrez l'onglet **Installed** dans `/plugin`, qui répertorie le plugin avec sa portée.
        * Dans votre shell, exécutez `claude plugin list`, qui imprime la même liste avec les lignes `Version`, `Scope` et `Status`.

        Si `/commit-commands:commit` n'apparaît pas, consultez [After install: plugin not working](/docs/fr/plugins/troubleshooting#plugin-installed-but-not-working).
      </Step>
    </Steps>

    L'installation à partir de n'importe quelle autre marketplace nécessite une étape supplémentaire d'abord : [ajoutez la marketplace](#add-a-marketplace). Claude Code ajoute la marketplace officielle d'Anthropic pour vous la première fois que vous démarrez une session de terminal interactive, c'est pourquoi l'exemple ignore cette étape. Si vous avez trouvé un plugin sur [claude.com/marketplace](https://claude.com/marketplace), son bouton **Claude Code** copie la commande d'installation sous sa [forme shell](#install-from-your-shell), `claude plugin install <name>@claude-plugins-official`.
  </Tab>

  <Tab title="Application de bureau">
    Dans une session locale ou SSH dans l'onglet **Code** de l'application de bureau :

    <Steps>
      <Step title="Ouvrez le navigateur de plugins">
        Cliquez sur le bouton **+** à côté de la zone de saisie et sélectionnez **Plugins**, puis **Add plugin**. Le navigateur de plugins s'ouvre avec les plugins de vos marketplaces.
      </Step>

      <Step title="Sélectionnez le plugin">
        Trouvez `commit-commands` et sélectionnez-le.
      </Step>

      <Step title="Choisissez une portée">
        Choisissez une [portée](#choose-an-install-scope) : votre compte utilisateur, ce projet ou local uniquement.
      </Step>
    </Steps>

    Pour activer, désactiver ou désinstaller ultérieurement, utilisez **+ > Plugins > Manage plugins**. Le navigateur de plugins n'est pas disponible dans les sessions cloud de l'application de bureau. Consultez [Install plugins in the desktop app](/docs/fr/desktop#install-plugins).
  </Tab>

  <Tab title="VS Code">
    Dans le panneau Claude Code dans VS Code :

    <Steps>
      <Step title="Ouvrez Manage plugins">
        Tapez `/plugins` dans la zone de saisie pour ouvrir **Manage plugins**.
      </Step>

      <Step title="Installez le plugin">
        Sur l'onglet **Plugins**, recherchez `commit-commands` et cliquez sur **Install**. Si l'onglet ne répertorie aucun plugin, ajoutez d'abord `anthropics/claude-plugins-official` sur l'onglet **Marketplaces**.
      </Step>

      <Step title="Choisissez une portée">
        Choisissez une [portée](#choose-an-install-scope) : **Install for you**, **Install for this project** ou **Install locally**.
      </Step>
    </Steps>

    Vos modifications s'appliquent aux sessions ouvertes sans redémarrage. Consultez [Manage plugins in VS Code](/docs/fr/vs-code#manage-plugins).
  </Tab>

  <Tab title="Session cloud">
    Une [session cloud](/docs/fr/cloud-environments), y compris [le navigateur à claude.ai/code](/docs/fr/claude-code-on-the-web), n'a pas de navigateur de plugins et ne charge pas les plugins que vous avez installés sur votre propre machine ou ceux que le `.claude/settings.json` de votre référentiel active. Pour les plugins que votre organisation distribue via les paramètres gérés, consultez [Manage plugins for your organization](/docs/fr/plugins/org).

    Consultez [quelles parties de votre configuration sont également disponibles dans une session cloud](/docs/fr/cloud-environments#what-carries-over-from-your-setup) pour le reste de votre configuration.
  </Tab>
</Tabs>

<h3 id="choose-an-install-scope">
  Choisir une portée d'installation
</h3>

La portée d'installation d'un plugin détermine qui obtient le plugin et quel fichier de paramètres l'enregistre comme activé :

* **User scope** : le plugin est activé pour vous dans chaque projet sur cette machine. L'entrée va dans `enabledPlugins` dans `~/.claude/settings.json`.
* **Project scope** : le plugin est activé pour tous ceux qui travaillent dans ce référentiel. L'entrée va dans `.claude/settings.json`, que vous validez.
* **Local scope** : le plugin est activé pour vous dans ce référentiel uniquement. L'entrée va dans `.claude/settings.local.json`.

Certains plugins sont définis par leur auteur pour démarrer désactivés, via le champ [`defaultEnabled`](/docs/fr/plugins/manifest-reference#defaultenabled). Un tel plugin est installé mais reste désactivé jusqu'à ce que vous l'activiez avec `claude plugin enable <name>` dans votre shell, ou à partir de l'onglet **Installed** de `/plugin` dans une session.

Lorsque le même plugin est défini à plusieurs portées, le paramètre local remplace le paramètre du projet, et le paramètre du projet remplace le paramètre utilisateur. Consultez [Find where a plugin is enabled](/docs/fr/plugins/loading#find-where-a-plugin-is-enabled) pour la règle complète.

Le terminal, les sessions locales de l'application de bureau et l'extension VS Code sur un ordinateur lisent les mêmes fichiers de paramètres, donc un plugin que vous installez à la portée utilisateur dans l'un d'eux est disponible dans les deux autres.

<h3 id="other-places-you-run-claude-code">
  JetBrains, exécutions non interactives et Agent SDK
</h3>

Certains endroits où vous exécutez Claude Code n'ont pas leur propre navigateur de plugins :

* **JetBrains IDEs** : le plugin JetBrains exécute Claude Code dans le terminal de l'IDE, donc utilisez les étapes de l'onglet **Terminal** là-bas.
* **`claude -p` et autres exécutions non interactives** : `/plugin` ne s'exécute pas, et Claude répond `Plugin isn't available in this environment.` Les plugins que vous avez déjà installés se chargent. Installez et gérez-les à partir de votre shell avec les [commandes `claude plugin`](#install-from-your-shell).
* **Agent SDK** : chargez les plugins via l'option plugin du SDK. Consultez [Load plugins in the Agent SDK](/docs/fr/agent-sdk/plugins).

Si Claude Code signale qu'un plugin activé dans le `.claude/settings.json` du référentiel n'est pas installé, consultez [Enabled in project settings but not installed](/docs/fr/plugins/loading#enabled-in-project-settings-but-not-installed).

<Tip>
  Si vous êtes un auteur de plugin testant une copie de votre plugin sur disque, démarrez Claude Code à partir de votre shell avec `--plugin-dir` pour le charger pour une session au lieu de l'installer. Consultez [Flags that load a plugin for one session](/docs/fr/plugins/cli-reference#flags-that-load-a-plugin-for-one-session).
</Tip>

<h3 id="plugins-from-your-claude-ai-account">
  Plugins de votre compte claude.ai
</h3>

Votre compte claude.ai est une source séparée de plugins, aux côtés des marketplaces à partir desquelles vous installez :

* **Ce qui arrive** : chaque plugin que vous activez pour votre compte claude.ai, et chaque plugin que votre organisation active pour ses membres. Dans une session de terminal, ils se synchronisent en arrière-plan chaque fois que vous démarrez Claude Code en étant connecté avec ce compte ; dans les sessions Cowork, ils se téléchargent au démarrage de la session.
* **Où vous les voyez** : dans `/plugin` et `claude plugin list` sous l'ID `<name>@synced`. Vous pouvez en désactiver un à votre propre portée sauf si votre organisation l'exige.
* **Ce qui ne va pas dans l'autre sens** : les plugins que vous installez avec `/plugin` ou `claude plugin install` restent sur cette machine et ne sont pas ajoutés à votre compte claude.ai.

Pour le calendrier de synchronisation, les exigences de connexion et la désactivation de la synchronisation, consultez [Plugins synced from claude.ai](/docs/fr/plugins/loading#synced-plugins).

<h3 id="install-from-your-shell">
  Installer à partir de votre shell
</h3>

Exécutez `claude plugin install` dans votre shell pour installer un plugin sans démarrer une session Claude Code, par exemple à partir d'un script de configuration.

* **Portée** : portée utilisateur par défaut. Passez `--scope project` ou `--scope local` pour le modifier.
* **Quand les plugins se chargent** : les plugins qu'il installe se chargent la prochaine fois que vous démarrez Claude Code, ou lorsque vous exécutez `/reload-plugins` dans une session déjà ouverte.
* **La marketplace doit d'abord être ajoutée** : sur une machine où personne n'a ouvert une session Claude Code interactive, la marketplace officielle n'est pas enregistrée, donc un script qui installe à partir de celle-ci exécute `claude plugin marketplace add anthropics/claude-plugins-official` avant l'installation.

```bash theme={null}
claude plugin install formatter@your-org --scope project
```

La commande imprime `Successfully installed plugin: formatter@your-org (scope: project)` quand elle se termine.

Certains plugins s'installent en exécutant une commande que leur marketplace nomme, appelée une [`command` source](/docs/fr/plugins/marketplace-reference#command-plugin-source). Claude Code vous montre cette commande et vous demande de l'accepter avant qu'elle ne s'exécute. Un script n'a personne pour répondre à cette invite, donc passez `--yes` là pour l'accepter.

Pour chaque drapeau `claude plugin install`, consultez [plugin install](/docs/fr/plugins/cli-reference#plugin-install).

<h2 id="add-a-marketplace">
  Ajouter une marketplace
</h2>

Vous n'avez besoin de cette section que lorsque le plugin que vous voulez n'est pas dans la marketplace officielle d'Anthropic, par exemple celui qu'un collègue a publié ou celui de la marketplace communautaire d'Anthropic.

Une marketplace est un catalogue de plugins, et Claude Code doit connaître une marketplace avant de pouvoir installer à partir de celle-ci. Vous ajoutez une marketplace une fois. Après cela, ses plugins apparaissent sur l'onglet **Discover** et s'installent avec `/plugin install <plugin>@<marketplace>` dans une session ou `claude plugin install <plugin>@<marketplace>` dans votre shell, où `<marketplace>` est le nom sous lequel la marketplace s'est enregistrée. Pour faire les deux en une seule étape, consultez [Add a marketplace and install in one command](#add-a-marketplace-and-install-in-one-command).

Dans une session Claude Code, exécutez `/plugin marketplace add` suivi de la source de la marketplace : un référentiel GitHub, un référentiel git sur n'importe quel hôte, un répertoire ou fichier local, ou un `marketplace.json` hébergé.

| Source                                  | Ce que vous tapez                                                                                                                                                                                                                                  | Exemple                                                                                                                                 |
| :-------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------- |
| Référentiel GitHub                      | `owner/repo`. Ajoutez `#ref` pour épingler une branche ou une balise.                                                                                                                                                                              | `/plugin marketplace add anthropics/claude-code`, ou `/plugin marketplace add your-org/plugins#v1.2.0` pour épingler la balise `v1.2.0` |
| Référentiel Git sur n'importe quel hôte | L'URL de clonage complète. Ajoutez `#ref` pour épingler une branche ou une balise.                                                                                                                                                                 | `/plugin marketplace add https://gitlab.example.com/your-group/your-marketplace.git#v1.0.0`                                             |
| Répertoire ou fichier local             | Un chemin relatif ou absolu vers un répertoire qui contient `.claude-plugin/marketplace.json`, ou vers le fichier JSON lui-même. Commencez un chemin relatif par `./` ou `../`, car Claude Code lit un `name/name` nu comme un référentiel GitHub. | `/plugin marketplace add ./my-marketplace`                                                                                              |
| `marketplace.json` hébergé              | Son URL `https://`                                                                                                                                                                                                                                 | `/plugin marketplace add https://example.com/marketplace.json`                                                                          |

À partir de votre shell, `claude plugin marketplace add` prend les mêmes sources.

<Tip>
  `/plugin market` fonctionne également comme une forme plus courte de `/plugin marketplace`.
</Tip>

Incluez le préfixe `https://` sur chaque URL, ou utilisez la forme `git@host:path` pour SSH. Si vous tapez un `gitlab.example.com/your-group/your-marketplace.git` nu, Claude Code le lit comme un raccourci GitHub `owner/repo` et le rejette.

Quand la commande réussit, elle imprime `Successfully added marketplace: <name>`, et les plugins de la marketplace apparaissent sur l'onglet **Discover** la prochaine fois que vous ouvrez `/plugin`, sans rechargement nécessaire. S'il échoue, faites correspondre le message d'erreur dans [Troubleshoot plugins](/docs/fr/plugins/troubleshooting#add-a-marketplace).

<h3 id="add-a-marketplace-and-install-in-one-command">
  Ajouter une marketplace et installer en une seule commande
</h3>

Pour installer un plugin à partir d'une marketplace que vous n'avez pas encore ajoutée, exécutez `/plugin install` dans une session Claude Code et nommez la source de la marketplace avec `--marketplace`. Nécessite Claude Code v2.1.275 ou ultérieur.

```text theme={null}
/plugin install deploy-helper --marketplace your-org/plugins
```

La source prend [les mêmes formes que `/plugin marketplace add`](#add-a-marketplace), telles que GitHub `owner/repo`, une URL git ou un chemin local, sauf qu'elle ne peut pas contenir d'espaces. Donnez le nom du plugin seul, sans suffixe `@marketplace`.

Si vous n'avez pas encore ajouté cette marketplace, Claude Code affiche la source qu'il a résolue et vous demande de confirmer avant de l'ajouter. Une fois la marketplace ajoutée, les détails du plugin s'ouvrent et vous choisissez une [portée d'installation](#install-a-plugin). Si la source correspond à une marketplace que vous avez déjà ajoutée, Claude Code ignore la confirmation et ouvre les détails du plugin dans cette marketplace.

<h3 id="add-a-private-marketplace">
  Ajouter une marketplace privée
</h3>

Une marketplace privée est celle dans un référentiel auquel vous avez besoin d'identifiants pour cloner, sur GitHub ou n'importe quel autre hôte git. Vous l'ajoutez avec la même commande `/plugin marketplace add` ou `claude plugin marketplace add` qu'une marketplace publique. Claude Code la clone avec les identifiants git déjà sur votre machine et ne demande jamais, donc chaque façon de se connecter a une exigence :

* **HTTPS** : vos assistants d'identifiants git s'appliquent, donc l'accès que vous avez configuré avec `gh auth login`, le Keychain macOS ou `git-credential-store` fonctionne. Les invites interactives sont supprimées, donc un hôte auquel vous ne vous êtes jamais authentifié échoue au lieu de demander un mot de passe.
* **SSH** : l'hôte doit déjà être dans votre fichier `known_hosts` et la clé doit fonctionner sans invite de phrase secrète, car les invites d'empreinte d'hôte et de phrase secrète sont également supprimées.
* **Raccourci GitHub `owner/repo`** : Claude Code vérifie si votre clé SSH s'authentifie à `github.com`, puis clone sur SSH si c'est le cas et sur HTTPS sinon. Définissez [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/fr/env-vars#variables) pour ignorer cette vérification et toujours cloner sur HTTPS.

Les mêmes identifiants s'appliquent lorsque vous exécutez `/plugin install`, `/plugin marketplace update` et `claude plugin update`.

Sur un hôte GitHub Enterprise Server, consultez [Plugin marketplaces on GHES](/docs/fr/github-enterprise-server#plugin-marketplaces-on-ghes) pour les identifiants que chaque opération nécessite.

Si votre organisation enregistre la marketplace pour vous via les paramètres gérés, vous ne l'ajoutez pas vous-même. Consultez [Pre-install and require plugins](/docs/fr/plugins/org#pre-install-and-require-plugins).

<h3 id="add-from-claude-ai">
  Ajouter une marketplace à partir de claude.ai
</h3>

Dans les sessions de terminal où [les plugins se synchronisent à partir de votre compte claude.ai](/docs/fr/plugins/loading#synced-plugins), claude.ai peut également répertorier les marketplaces de plugins pour vous, telles que la bibliothèque de plugins de votre organisation et vos propres téléchargements claude.ai. Vous en ajoutez une par son nom plutôt que par une source. L'ajout d'une marketplace à partir de claude.ai nécessite Claude Code v2.1.273 ou ultérieur.

Ajoutez une marketplace claude.ai à partir du panneau `/plugin` ou à partir de votre shell :

* **À l'intérieur d'une session** : exécutez `/plugin` et allez à l'onglet **Marketplaces**, qui répertorie les marketplaces à partir de claude.ai. Sélectionnez-en une là pour l'ajouter.
* **À partir de votre shell** : exécutez `claude plugin marketplace list`, qui les imprime dans une section `From claude.ai:`. Ensuite, exécutez `claude plugin marketplace add` avec le drapeau `--claudeai` et le nom affiché dans la liste.

Par exemple, cette commande ajoute une marketplace nommée `claudeai-organization-library` :

```bash theme={null}
claude plugin marketplace add --claudeai claudeai-organization-library
```

Claude Code enregistre la marketplace sous un nom local qui commence par `claudeai-`, dérivé du nom que claude.ai la répertorie. Par exemple, une marketplace répertoriée comme « Organization library » devient `claudeai-organization-library`. Installez ses plugins par ce nom, par exemple avec `claude plugin install <plugin>@claudeai-organization-library`.

Si vous vous déconnectez ou vous connectez à une organisation claude.ai différente, la marketplace reste configurée mais n'affiche aucun plugin, et les plugins que vous avez déjà installés à partir de celle-ci continuent de se charger.

La section `From claude.ai:` peut également répertorier les marketplaces basées sur git partagées via claude.ai, et elle imprime une source pour chacune d'elles. Ajoutez-les par cette source comme dans [Add a marketplace](#add-a-marketplace), pas avec `--claudeai`.

<h2 id="manage-installed-plugins">
  Gérer les plugins installés
</h2>

L'onglet **Installed** dans `/plugin` répertorie vos plugins avec des actions pour activer, désactiver, mettre à jour ou désinstaller chacun. Dans une session Claude Code, exécutez `/plugin` et appuyez sur **Tab** pour l'atteindre, ou exécutez `/plugin enable`, `/plugin disable` ou `/plugin uninstall` pour ouvrir le panneau et effectuer ce changement là. Les plugins désactivés sont regroupés sous un en-tête réduit en bas de la liste. Utilisez ces touches sur la liste :

* Tapez pour filtrer par nom ou description.
* Appuyez sur **Espace** pour activer ou désactiver le plugin sélectionné, et **f** pour le marquer comme favori.
* Appuyez sur **Entrée** pour ouvrir les détails d'un plugin. Le menu là offre **Disable plugin** ou **Enable plugin**, **Update now** et **Uninstall**. Les plugins qui prennent des paramètres offrent également **Configure options**.

L'onglet peut également afficher les plugins à la portée **Managed**. Votre organisation les a installés via les [paramètres gérés](/docs/fr/settings#settings-files), et vous ne pouvez pas les activer, désactiver ou les désinstaller ici.

Pour un plugin synchronisé que votre organisation exige sur claude.ai, consultez [Manage plugins synced from claude.ai](#manage-plugins-synced-from-claude-ai).

Lorsque vous fermez le panneau `/plugin` avec des modifications en attente que vous avez apportées, Claude Code exécute `/reload-plugins` pour vous pour les appliquer. Si le rechargement [invaliderait le cache du prompt](/docs/fr/prompt-caching#enabling-or-disabling-a-plugin), il vous avertit et laisse les modifications en attente à la place. Exécutez `/reload-plugins --force` pour les appliquer quand même.

<h3 id="manage-plugins-synced-from-claude-ai">
  Gérer les plugins synchronisés à partir de claude.ai
</h3>

L'onglet **Installed** dans `/plugin` répertorie également les [plugins synchronisés à partir de votre compte claude.ai](/docs/fr/plugins/loading#synced-plugins), avec `synced` comme source. Les plugins synchronisés apparaissent dans les sessions de terminal sur Claude Code v2.1.273 ou ultérieur.

* **Activer ou désactiver** : utilisez l'onglet **Installed**, sauf si votre organisation a marqué le plugin comme requis.
* **Supprimer** : désactivez le plugin sur claude.ai.

Lorsque Claude Code synchronise un plugin ajouté, mis à jour ou supprimé dans une session interactive, vous voyez `Plugins changed. Run /reload-plugins to activate.` Exécutez `/reload-plugins` pour charger le changement dans cette session, ou laissez-le pour la prochaine fois que vous démarrez Claude Code.

<h3 id="uninstall-a-plugin-the-project-enables">
  Désinstaller un plugin que le projet active
</h3>

Lorsque vous choisissez **Uninstall** pour un plugin que le `.claude/settings.json` de ce référentiel active, que ce soit à partir de l'onglet **Installed** ou avec `/plugin uninstall`, Claude Code vous demande si vous voulez le désactiver pour vous ou le désinstaller pour tout le monde :

* **Disable for me** : appuyez sur **y**. Claude Code écrit `false` pour le plugin dans votre `.claude/settings.local.json` et le laisse installé pour le projet.
* **Uninstall for everyone** : appuyez sur **u**. Claude Code supprime le plugin du `.claude/settings.json` partagé.

<h3 id="see-what-an-installed-plugin-adds-to-your-sessions">
  Voir ce qu'un plugin installé ajoute à vos sessions
</h3>

Dans votre shell, exécutez `claude plugin details <name>` pour un plugin installé. La ligne `Always-on` est le nombre de tokens que le plugin ajoute à chaque session où il est activé, et les lignes par composant montrent quel skill ou agent contribue le plus. Pour la sortie complète et ce que chaque chiffre signifie, consultez [Measure what a plugin costs](/docs/fr/plugins/measure#measure-what-a-plugin-costs).

<h3 id="find-plugins-you-no-longer-use">
  Trouver les plugins que vous n'utilisez plus
</h3>

Sur l'onglet **Installed** dans `/plugin`, les plugins que vous avez installés vous-même et que vous n'avez pas utilisés récemment apparaissent sous un en-tête **Not used recently**, et les détails de chaque plugin affichent une ligne **Last used**. Utilisez cet en-tête et cette ligne pour trouver les plugins qui ajoutent toujours le coût de démarrage et de contexte, puis désactivez ou désinstallez-les.

<h3 id="plugins-with-dependencies">
  Plugins avec dépendances
</h3>

Un plugin peut déclarer d'autres plugins dont il dépend. Lorsque vous installez, désactivez ou désinstallez un tel plugin à partir d'une marketplace, Claude Code agit également sur ces dépendances :

* **Install** : Claude Code installe également et active les dépendances déclarées du plugin à la même portée. Le message de succès les répertorie.
* **Enable** : Claude Code active également les dépendances du plugin qui sont installées mais désactivées. Si une dépendance déclarée n'est pas installée, l'activation échoue et le message vous dit de l'installer d'abord.
* **Disable** : quand un autre plugin activé a toujours besoin de celui que vous avez nommé, Claude Code refuse et imprime une commande chaînée qui désactive les deux dans le bon ordre.
* **Uninstall** : les dépendances auto-installées restent jusqu'à ce que vous exécutiez `claude plugin prune` dans votre shell ; consultez [plugin prune](/docs/fr/plugins/cli-reference#plugin-prune).

Si vous avez chargé le plugin avec `--plugin-dir` à la place, consultez [Test a plugin and its dependency locally](/docs/fr/plugins/dependencies#test-a-plugin-and-its-dependency-locally).

<h3 id="manage-plugins-from-your-shell">
  Gérer les plugins à partir de votre shell
</h3>

Vous pouvez également gérer les plugins sans démarrer une session Claude Code. Dans votre shell, exécutez `claude plugin install`, `enable`, `disable` ou `uninstall` comme des commandes de terminal ordinaires ; elles modifient les mêmes paramètres que le panneau `/plugin`. Chacun prend `--scope` pour cibler une portée, et utilise une portée par défaut lorsque vous l'omettez :

* `enable` et `disable` agissent sur la portée la plus spécifique dont les paramètres répertorient déjà le plugin.
* `install` et `uninstall` agissent sur la portée utilisateur.

Par exemple, ces commandes désactivent et réactivent un plugin, puis le désinstallent à la portée du projet :

```bash theme={null}
claude plugin disable formatter@your-org
claude plugin enable formatter@your-org
claude plugin uninstall formatter@your-org --scope project
```

<h2 id="keep-plugins-updated">
  Garder les plugins à jour
</h2>

Les plugins se mettent à jour automatiquement lorsque la marketplace dont ils proviennent a la mise à jour automatique activée. Après le démarrage d'une session, Claude Code actualise ces marketplaces et met à jour les copies sur disque des plugins que vous avez installés à partir de celles-ci.

La session en cours conserve les versions qu'elle a déjà chargées. Après une mise à jour, vous voyez `Plugin updated: <name> · Run /reload-plugins to apply`, et la session suivante charge automatiquement les nouvelles versions.

Ce sont les paramètres par défaut de mise à jour automatique pour chaque type de marketplace :

* **On by default** : `claude-plugins-official` et les autres [noms de marketplace officiels](/docs/fr/plugins/security#official-marketplace-names) sauf `knowledge-work-plugins` et `first-party-plugins`, plus les [marketplaces ajoutées à partir de claude.ai](#add-from-claude-ai).
* **Off by default** : chaque autre marketplace, y compris la marketplace communautaire, les marketplaces tierces et les marketplaces de développement local.

Pour quand la mise à jour automatique s'exécute, quels plugins elle ignore et les variables d'environnement qui la désactivent, consultez [When auto-update runs](/docs/fr/plugins/loading#when-auto-update-runs).

<h3 id="turn-auto-update-on-or-off-for-a-marketplace">
  Activer ou désactiver la mise à jour automatique pour une marketplace
</h3>

Dans une session Claude Code, exécutez `/plugin` et allez à l'onglet **Marketplaces**. Sélectionnez la marketplace, puis sélectionnez **Enable auto-update** ou **Disable auto-update**.

<h3 id="update-one-plugin-now">
  Mettre à jour un plugin maintenant
</h3>

Dans une session, ouvrez le plugin sur l'onglet **Installed** dans `/plugin` et sélectionnez **Update now**, ou dans votre shell exécutez `claude plugin update <plugin>@<marketplace>`.

<h3 id="auto-update-from-a-private-marketplace">
  Mise à jour automatique à partir d'une marketplace privée
</h3>

Pour une marketplace privée, consultez [What background auto-update does with credentials](/docs/fr/plugins/host-marketplace#what-background-auto-update-does-with-credentials) pour savoir comment les mises à jour automatiques en arrière-plan s'authentifient sur SSH et HTTPS, et [Troubleshoot plugins](/docs/fr/plugins/troubleshooting#add-a-marketplace) pour les messages que vous voyez quand elles échouent.

<h2 id="manage-marketplaces">
  Gérer les marketplaces
</h2>

L'onglet **Marketplaces** dans `/plugin` répertorie chaque marketplace que vous avez enregistrée, ainsi que sa source. Sélectionnez-en une pour parcourir ses plugins, mettre à jour son annonce, activer ou désactiver la mise à jour automatique, ou la supprimer.

Vous pouvez également répertorier, mettre à jour et supprimer les marketplaces avec des commandes, à partir de votre shell ou à l'intérieur d'une session :

| Action                                    | Dans votre shell                          | À l'intérieur d'une session         |
| :---------------------------------------- | :---------------------------------------- | :---------------------------------- |
| Répertorier les marketplaces              | `claude plugin marketplace list`          | `/plugin marketplace list`          |
| Mettre à jour l'annonce d'une marketplace | `claude plugin marketplace update <name>` | `/plugin marketplace update <name>` |
| Supprimer une marketplace                 | `claude plugin marketplace remove <name>` | `/plugin marketplace remove <name>` |

Lorsque vous supprimez une marketplace, Claude Code désinstalle chaque plugin que vous avez installé à partir de celle-ci et supprime leurs entrées `enabledPlugins` de vos fichiers de paramètres. L'onglet **Marketplaces** nomme ces plugins avant de vous demander de confirmer.

<h2 id="next-steps">
  Étapes suivantes
</h2>

* [Les marketplaces d'Anthropic](/docs/fr/plugins/anthropic-marketplaces) : comment les marketplaces officielles, communautaires et de démonstration diffèrent et où parcourir chacune d'elles
* [Référence de chargement des plugins](/docs/fr/plugins/loading) : pourquoi un plugin s'est chargé, ne s'est pas chargé, ou n'a pas changé après une mise à jour
* [Sécurité et confiance des plugins](/docs/fr/plugins/security) : ce qu'il faut examiner avant d'installer un plugin à partir d'une marketplace que vous ne connaissez pas
* [Dépanner les plugins](/docs/fr/plugins/troubleshooting) : messages d'erreur d'installation et de marketplace avec leurs corrections
* [Créer un plugin](/docs/fr/plugins/create) : créez le vôtre
