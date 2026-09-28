> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Créer un plugin Claude Code

> Créez votre premier plugin Claude Code à partir d'un répertoire vide, testez-le sans marketplace et convertissez une configuration .claude/ existante.

Un plugin est un répertoire de skills, d'agents, de hooks et de serveurs MCP, plus un fichier `plugin.json`, appelé le manifeste, qui nomme le plugin. Claude Code charge le répertoire comme une unité, ce qui vous permet de le partager avec vos coéquipiers, de l'installer dans plusieurs projets ou de le publier sur une marketplace.

Cette page s'adresse aux personnes qui écrivent leurs propres plugins.

<Note>
  Ces cas sont couverts sur d'autres pages :

  * **Installer le plugin de quelqu'un d'autre** : voir [Installer des plugins](/docs/fr/plugins/install)
  * **Vous ne savez pas si vous avez besoin d'un plugin** : voir [Décider si vous avez besoin d'un plugin](/docs/fr/plugins/overview#decide-whether-you-need-a-plugin) dans l'aperçu
  * **Les utilisateurs de votre plugin sont sur claude.ai ou dans Cowork** : le même dossier s'installe là avec un sous-ensemble différent de composants. Voir [Plugins sur claude.ai et dans Cowork](https://claude.com/docs/plugins/overview)
</Note>

Commencez par la section qui correspond à ce que vous avez déjà :

* **Rien encore** : suivez [Créer votre premier plugin](#create-your-first-plugin), puis [Développer sans marketplace](#develop-without-a-marketplace) et [Tester et déboguer](#test-and-debug).
* **Fichiers sous `.claude/` déjà présents** : faites la procédure pas à pas du premier plugin une fois pour apprendre la disposition, puis suivez [Convertir une configuration `.claude/` existante](#convert-an-existing-claude-setup).

<h2 id="decide-when-to-use-a-plugin">
  Décider quand utiliser un plugin
</h2>

Les skills, agents, hooks et serveurs MCP fonctionnent tous de manière autonome dans votre projet ou répertoire personnel. Conservez cette configuration autonome tant qu'elle ne concerne qu'un seul projet ou que vous seul. Créez un plugin quand vous voulez partager la configuration avec vos coéquipiers, l'installer dans plusieurs projets ou publier des versions.

Quand vous déplacez les skills, agents, hooks et configuration MCP autonomes dans un plugin, leur emplacement et leurs noms changent :

* **Où vont les fichiers** : sous le répertoire propre du plugin, appelé la racine du plugin, comme `skills/`, `agents/`, `hooks/hooks.json` et `.mcp.json`.
* **Comment ils sont nommés** : les skills et agents du plugin reçoivent le nom du plugin comme préfixe, par exemple `/my-plugin:hello`, de sorte que deux plugins peuvent chacun fournir un skill `hello` sans collision.

Pour déplacer une configuration existante dans un plugin, voir [Convertir une configuration `.claude/` existante](#convert-an-existing-claude-setup).

<h2 id="create-your-first-plugin">
  Créer votre premier plugin
</h2>

Dans cette procédure pas à pas, vous créez un plugin dont le seul composant est un skill, un salut, et vous l'exécutez avec `--plugin-dir`, qui charge un plugin pour une session sans l'installer. Un plugin peut contenir n'importe quel mélange de [composants](/docs/fr/plugins/components), tels que des skills, des agents, des hooks et des serveurs MCP, et aucun n'est requis ; un skill est le plus petit exemple qui montre la disposition.

Vous avez besoin de Claude Code [installé et connecté](/docs/fr/quickstart#step-1-install-claude-code).

Ouvrez un terminal dans le répertoire où vous voulez conserver le plugin, par exemple `~/projects`, et exécutez les commandes de ces étapes à partir de là. Vous pouvez conserver un plugin n'importe où, car vous passez son chemin à Claude Code quand vous démarrez une session.

<Steps>
  <Step title="Créer le répertoire du plugin">
    Créez le répertoire du plugin, avec un dossier `.claude-plugin/` à l'intérieur pour contenir le manifeste :

    ```bash theme={null}
    mkdir -p my-first-plugin/.claude-plugin
    ```
  </Step>

  <Step title="Écrire le manifeste">
    Le [manifeste](/docs/fr/plugins/manifest-reference) est un fichier JSON nommé `plugin.json` qui indique à Claude Code le nom du plugin et le décrit. Enregistrez celui-ci comme `my-first-plugin/.claude-plugin/plugin.json` :

    ```json my-first-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-first-plugin",
      "description": "A greeting plugin to learn the basics",
      "version": "1.0.0",
      "author": {
        "name": "Your Name"
      }
    }
    ```

    Les quatre champs font ceci :

    * **`name`** : requis. Il identifie le plugin et devient le préfixe sur chaque skill et agent que le plugin fournit. Ne mettez pas d'espaces dedans.
    * **`description`** : le texte que les utilisateurs voient pour le plugin dans `/plugin`.
    * **`version`** : optionnel. Le définir maintient les utilisateurs sur cette version jusqu'à ce que vous la changiez ; [Publier une nouvelle version](/docs/fr/plugins/host-marketplace#release-a-new-version) indique quand le définir ou l'omettre.
    * **`author`** : qui créditer. `name` est requis à l'intérieur ; `email` et `url` sont optionnels.

    Tous les autres champs sont sur la [référence du manifeste](/docs/fr/plugins/manifest-reference#fields).

    Seul `plugin.json` va à l'intérieur de `.claude-plugin/`. Le skill que vous ajoutez ensuite va directement sous `my-first-plugin/`, à côté de ce dossier.
  </Step>

  <Step title="Ajouter un skill">
    Le seul composant de ce plugin est un skill. Chaque skill est un répertoire sous `skills/` qui contient un fichier `SKILL.md`. Créez le répertoire du skill :

    ```bash theme={null}
    mkdir -p my-first-plugin/skills/hello
    ```

    Ensuite, créez `my-first-plugin/skills/hello/SKILL.md` avec ce contenu :

    ```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
    ---
    name: hello
    description: Greet the user with a friendly message
    disable-model-invocation: true
    ---

    Greet the user warmly and ask how you can help them today.
    ```

    La ligne `disable-model-invocation: true` signifie que Claude n'exécute pas le skill de lui-même, donc seul vous le déclenchez. Supprimez cette ligne d'un skill que vous voulez que Claude exécute de lui-même. La commande du skill combine le nom du plugin et le nom du skill, donc vous exécutez celui-ci comme `/my-first-plugin:hello`. Pour les autres champs du frontmatter, voir la [référence du frontmatter du skill](/docs/fr/skills#frontmatter-reference).
  </Step>

  <Step title="Valider le plugin">
    Vérifiez le manifeste et le frontmatter du skill avant d'exécuter quoi que ce soit :

    ```bash theme={null}
    claude plugin validate ./my-first-plugin
    ```

    La commande imprime le chemin du manifeste qu'elle a vérifié et `✔ Validation passed`. Si elle imprime `✘ Validation failed` à la place, chaque ligne au-dessus de cette ligne de résultat nomme le champ à corriger. Recherchez chaque message sous [`claude plugin validate` rapporte des erreurs](/docs/fr/plugins/troubleshooting#claude-plugin-validate-reports-errors).
  </Step>

  <Step title="Exécuter Claude Code avec le plugin">
    Démarrez une session avec le plugin chargé :

    ```bash theme={null}
    claude --plugin-dir ./my-first-plugin
    ```

    Une fois Claude Code démarré, exécutez le skill :

    ```text theme={null}
    /my-first-plugin:hello
    ```

    Claude répond avec un salut.
  </Step>
</Steps>

Le plugin ne se charge que dans les sessions que vous démarrez avec `--plugin-dir`. Pour continuer à travailler dessus sans le drapeau, ou pour tester une version `.zip`, voir [Développer sans marketplace](#develop-without-a-marketplace).

<h3 id="share-the-plugin">
  Partager votre plugin
</h3>

Un plugin que vous avez créé avec [Créer votre premier plugin](#create-your-first-plugin) n'existe que sur votre machine. Quand il est prêt pour d'autres personnes, il y a trois façons de le leur faire parvenir :

* **L'envoyer à quelques personnes directement** : donnez-leur le répertoire du plugin ou un `.zip` de celui-ci, et rien n'a besoin d'être publié. Voir [Partager un plugin sans marketplace](/docs/fr/plugins/publish#share-a-plugin-without-a-marketplace).
* **Le lister dans votre propre marketplace** : les coéquipiers ajoutent votre marketplace une fois et installent le plugin par nom, et ils reçoivent vos mises à jour. Voir [Publier via votre propre marketplace](/docs/fr/plugins/publish#publish-through-your-own-marketplace).
* **Le soumettre à la marketplace communautaire d'Anthropic** : une fois qu'il est listé, quiconque ajoute cette marketplace peut l'installer. Voir [Soumettre à la marketplace communautaire](/docs/fr/plugins/publish#submit-to-the-community-marketplace).

<h3 id="plugin-layout">
  Disposition du plugin
</h3>

Chaque type de [composant](/docs/fr/plugins/components), tel que les skills, agents, hooks et serveurs MCP, va dans un répertoire fixe sous la racine du plugin, qui est le répertoire que vous passez à `--plugin-dir`. Ajoutez uniquement les répertoires que vous utilisez. Pour cliquer dans un répertoire de plugin complet et lire ce que chaque fichier fait, ouvrez l'[explorateur de plugin](/docs/fr/plugins/components#explore-the-plugin-directory).

Le tableau liste les répertoires par lesquels la plupart des plugins commencent, et la [disposition complète](/docs/fr/plugins/manifest-reference#standard-layout) liste le reste.

| Emplacement                  | Contenu                                                                                                                                          |
| :--------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------- |
| `.claude-plugin/plugin.json` | Le manifeste. Quand vous chargez un plugin avec `--plugin-dir` et qu'il n'a pas de manifeste, Claude Code nomme le plugin d'après son répertoire |
| `skills/`                    | Un répertoire `<name>/SKILL.md` par skill                                                                                                        |
| `commands/`                  | Fichiers Markdown plats, la forme plus ancienne des skills. Utilisez `skills/` pour les nouveaux plugins                                         |
| `agents/`                    | Un fichier Markdown par sous-agent                                                                                                               |
| `hooks/hooks.json`           | Configuration des hooks : une clé `"hooks"` de niveau supérieur dont la valeur a la même forme que `hooks` dans un fichier de paramètres         |
| `.mcp.json`                  | Définitions du serveur MCP                                                                                                                       |

<Warning>
  Seul `plugin.json` va à l'intérieur de `.claude-plugin/`. Les composants enregistrés là ne se chargent pas.

  La racine du plugin est le répertoire propre du plugin, pas `~/.claude/` lui-même. Un `.mcp.json` enregistré à `~/.claude/.mcp.json` ne se charge pas.
</Warning>

<h2 id="develop-without-a-marketplace">
  Développer sans marketplace
</h2>

Vous n'avez pas besoin d'une [marketplace](/docs/fr/plugins/overview#get-plugins-from-a-marketplace) pour exécuter un plugin que vous écrivez. Chargez-le directement à partir du disque ou d'une URL à la place :

* [`--plugin-dir`](#load-a-directory-or-archive-for-one-session) : charge un répertoire ou une archive `.zip` pour une session.
* [`--plugin-url`](#fetch-an-archive-from-a-url-for-one-session) : récupère une archive `.zip` à partir d'une URL pour une session.
* [`claude plugin init`](#scaffold-a-plugin-that-loads-every-session) : crée un plugin sous `~/.claude/skills/` qui se charge à chaque session.

Si deux plugins chargés de différentes façons partagent un nom, voir [Conflits de noms](/docs/fr/plugins/loading#name-conflicts) pour savoir lequel Claude Code conserve.

<h3 id="load-a-directory-or-archive-for-one-session">
  Charger un plugin pour une session
</h3>

Vous pouvez charger un plugin pour une seule session de trois façons : à partir d'un répertoire ou d'une archive `.zip` sur le disque avec `--plugin-dir`, à partir d'une URL avec `--plugin-url`, ou à partir d'une variable d'environnement quand vous ne pouvez pas ajouter un drapeau. Chaque plugin se charge pour cette session uniquement, et rien n'est écrit dans vos paramètres pour celui-ci. Quand vous modifiez les fichiers du plugin pendant la session, exécutez `/reload-plugins` pour charger les modifications.

<h4 id="from-a-directory-or-zip">
  À partir d'un répertoire ou `.zip`
</h4>

Quand vous démarrez `claude` à partir de votre shell, passez `--plugin-dir` avec le répertoire racine du plugin ou une archive `.zip` de celui-ci. Répétez le drapeau pour charger plusieurs plugins :

```bash theme={null}
claude --plugin-dir ./my-first-plugin --plugin-dir ./other-plugin.zip
```

<h4 id="load-a-folder-of-plugins">
  À partir d'un dossier de plugins
</h4>

Pour charger plusieurs plugins à partir d'un seul endroit, passez un dossier qui les contient, par exemple `--plugin-dir ./plugins`. Charger un dossier de plugins nécessite Claude Code v2.1.265 ou ultérieur.

Si le dossier n'a pas de répertoire `.claude-plugin/` et pas de composants de plugin au niveau supérieur, Claude Code le traite comme un dossier de plugins. Chaque sous-dossier immédiat qui a un manifeste `.claude-plugin/plugin.json` se charge alors comme un plugin séparé. Tout le reste dans le dossier est ignoré sans erreur, y compris un sous-dossier qui n'a pas de manifeste. Si un plugin dans le dossier ne se charge pas, vérifiez que son sous-dossier a un `.claude-plugin/plugin.json`.

Dans une session interactive, vous pouvez également ajouter et supprimer des plugins dans le dossier après le démarrage :

* Un sous-dossier que vous ajoutez se charge comme un nouveau plugin une fois que son manifeste existe.
* Quand vous supprimez un sous-dossier, son plugin se décharge.

Un message apparaît dans la session pour chacun de ces changements. Si charger ou décharger un plugin en milieu de conversation [invaliderait le cache de prompt](/docs/fr/prompt-caching#enabling-or-disabling-a-plugin), le changement est retenu à la place, et le message vous dit d'exécuter `/reload-plugins` pour l'appliquer.

<h4 id="fetch-an-archive-from-a-url-for-one-session">
  À partir d'une URL
</h4>

Quand vous démarrez `claude` à partir de votre shell, passez `--plugin-url` avec l'adresse d'une archive `.zip`, par exemple un artefact de build que votre CI publie :

```bash theme={null}
claude --plugin-url https://example.com/my-first-plugin.zip
```

Claude Code télécharge l'archive au démarrage. Pour en charger plusieurs, répétez le drapeau ou passez les URL séparées par des espaces dans un argument entre guillemets.

Pointez le drapeau uniquement vers des archives que vous contrôlez ou en lesquelles vous avez confiance.

Si Claude Code ne peut pas récupérer l'archive, ou si l'archive est invalide, il démarre sans le plugin et enregistre une erreur de chargement de plugin que vous pouvez examiner dans l'onglet **Errors** du gestionnaire `/plugin`.

<h4 id="from-an-environment-variable">
  À partir d'une variable d'environnement
</h4>

Pour charger des plugins dans une session où vous ne pouvez pas ajouter le drapeau `--plugin-dir`, listez leurs chemins absolus dans la variable d'environnement [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/fr/env-vars#variables) à la place. Claude Code charge chaque chemin comme il charge un chemin `--plugin-dir`. Ces plugins se chargent en plus de ceux que vous passez avec `--plugin-dir`. [Les paramètres de projet et locaux ne peuvent pas définir cette variable](/docs/fr/settings-reference#variables-claude-code-ignores-in-env). `CLAUDE_CODE_PLUGIN_DIRS` nécessite Claude Code v2.1.280 ou ultérieur.

Les paramètres gérés peuvent désactiver `--plugin-dir` et `CLAUDE_CODE_PLUGIN_DIRS`. Voir [Drapeaux qui chargent un plugin pour une session](/docs/fr/plugins/cli-reference#flags-that-load-a-plugin-for-one-session). Pour tester un plugin avec un plugin dont il dépend, voir [Tester un plugin et sa dépendance localement](/docs/fr/plugins/dependencies#test-a-plugin-and-its-dependency-locally).

<h3 id="scaffold-a-plugin-that-loads-every-session">
  Faire charger un plugin à chaque session
</h3>

Votre répertoire de skills personnel est `~/.claude/skills/`. Claude Code charge n'importe quel dossier là qui contient un `.claude-plugin/plugin.json` comme un plugin à chaque session, sans drapeau et sans étape d'installation. `claude plugin init` en crée un pour vous.

<h4 id="scaffold-the-plugin-with-claude-plugin-init">
  Créer le plugin avec `claude plugin init`
</h4>

`claude plugin init` écrit un plugin de démarrage sous `~/.claude/skills/`. Nécessite Claude Code v2.1.157 ou ultérieur. Créez-en un à partir de votre shell :

```bash theme={null}
claude plugin init my-tool
```

La commande crée `~/.claude/skills/my-tool/` avec un `.claude-plugin/plugin.json` et un `SKILL.md` racine. Elle imprime `✔ Created plugin "my-tool" at ~/.claude/skills/my-tool` suivi de `It will auto-load next session as my-tool@skills-dir. Run /reload-plugins to load it now.`

Passez `--with skills` pour que `claude plugin init` crée un skill sous `skills/` pour vous. Les autres valeurs `--with` sont sur la [référence des commandes de plugin](/docs/fr/plugins/cli-reference#plugin-init).

<h4 id="skill-names-in-a-scaffolded-plugin">
  Nommer les skills du plugin
</h4>

Le skill racine à `~/.claude/skills/my-tool/SKILL.md` est aussi un skill personnel, donc vous l'invoquez comme `/my-tool`, pas `/my-tool:my-tool`. Les skills que vous ajoutez sous `skills/` à l'intérieur du plugin reçoivent le préfixe du nom du plugin, par exemple `/my-tool:example`.

<h4 id="stop-loading-the-plugin">
  Arrêter de charger le plugin
</h4>

Pour arrêter de charger un plugin créé, supprimez son répertoire, ou exécutez `claude plugin disable my-tool@skills-dir` dans votre shell avec le nom `my-tool@skills-dir` que `claude plugin init` a imprimé. Dans l'ID `my-tool@skills-dir`, `skills-dir` se tient à la place où un nom de marketplace serait, car le plugin se charge à partir de votre répertoire de skills plutôt que d'une marketplace.

<h4 id="load-a-plugin-for-everyone-in-one-repository">
  Partager le plugin via un référentiel
</h4>

`claude plugin init` écrit le plugin dans votre répertoire de skills personnel à `~/.claude/skills/`, donc il se charge pour vous dans chaque projet. Pour faire charger un plugin pour tout le monde dans un référentiel, créez la même disposition vous-même à `<project>/.claude/skills/<name>/`, y compris son `.claude-plugin/plugin.json`. Voir [Plugins partagés via un référentiel](/docs/fr/plugins/loading#plugins-shared-through-a-repository) pour les conditions sous lesquelles Claude Code le charge.

<h2 id="test-and-debug">
  Tester et déboguer
</h2>

Quand un changement à votre plugin ne s'affiche pas, travaillez à travers ces vérifications dans l'ordre. Chacune vous dit ce que Claude Code a fait avec le plugin :

1. Dans votre shell, exécutez `claude plugin validate <path>`. Il vérifie le manifeste et le frontmatter de chaque fichier de skill, agent et command, et quitte avec `0` sur `Validation passed`. Ajoutez `--strict` pour échouer aussi sur les avertissements. Les codes de sortie et la gestion des répertoires sont sur la [référence des commandes de plugin](/docs/fr/plugins/cli-reference#plugin-validate).
2. Dans la session en cours, exécutez `/reload-plugins` pour appliquer les modifications que vous avez apportées sur le disque. Il imprime une ligne `Reloaded:` avec des comptages. Ensuite, confirmez qu'un skill s'est chargé en tapant sa commande `/plugin-name:skill`, ou en trouvant le plugin dans l'onglet **Installed** de `/plugin`.
3. Dans la même session, exécutez `/plugin`. L'onglet **Installed** liste votre plugin et, dans les détails du plugin, les composants que Claude Code a trouvés. L'onglet **Errors** liste ce qui n'a pas pu se charger et pourquoi, par exemple un chemin dans votre manifeste qui n'existe pas.
4. De retour dans votre shell, exécutez `claude plugin list`. Il imprime les plugins de session uniquement et du répertoire de skills dans leurs propres sections avec `Status: ✔ loaded` ou l'erreur de chargement. Pour inclure le plugin que vous développez, passez `--plugin-dir` avec son chemin avant `plugin list`.

Pour vérifier un serveur MCP, exécutez `/mcp` dans la session pour voir l'état du serveur. Quand le serveur est sain, `/mcp` le liste comme connecté. Si ce n'est pas le cas, voir [Serveurs MCP qui ne démarrent pas](/docs/fr/plugins/troubleshooting#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start).

Pour vérifier un hook, déclenchez l'événement qu'il correspond. Par exemple, demandez à Claude d'éditer un fichier pour déclencher un hook `PostToolUse`. Ensuite, lisez le [journal de débogage](/docs/fr/hooks#debug-hooks), qui montre quels hooks ont correspondu, leurs codes de sortie et leur sortie.

Les sections suivantes couvrent les défaillances que vous êtes le plus susceptible de rencontrer lors du développement, et la [page de dépannage](/docs/fr/plugins/troubleshooting#build-a-plugin) a l'entrée complète pour chacune.

<h3 id="a-component-path-isn’t-found">
  Un chemin de composant n'est pas trouvé
</h3>

L'onglet **Errors** de `/plugin` affiche `<component> path not found: <path>`, par exemple `commands path not found`. Un chemin de composant dans votre manifeste, tel que `commands`, `skills`, `agents` ou `hooks`, ne pointe vers rien. Corrigez le chemin ou créez le répertoire, puis exécutez `/reload-plugins` dans la session. Voir [`commands path not found`](/docs/fr/plugins/troubleshooting#commands-path-not-found).

<h3 id="plugin-dir-at-a-marketplace-root-doesn’t-load-the-plugins-under-plugins/">
  `--plugin-dir` à la racine d'une marketplace ne charge pas les plugins sous `plugins/`
</h3>

`--plugin-dir` prend le répertoire racine du plugin, celui qui contient `.claude-plugin/plugin.json` et les répertoires de composants tels que `skills/`. Si vous le pointez à la racine d'une marketplace à la place, Claude Code ne lit pas `marketplace.json`, donc un plugin sous `plugins/` ne se charge pas, et vous ne voyez pas d'erreur. Pointez le drapeau vers le dossier d'un plugin, ou ajoutez la marketplace. Voir [l'entrée de dépannage](/docs/fr/plugins/troubleshooting#plugin-dir-loads-a-plugin-with-no-components).

<h3 id="the-plugin-loads-but-its-skills-are-missing">
  Le plugin se charge mais ses skills manquent
</h3>

Le répertoire `skills/` est à l'intérieur de `.claude-plugin/`, ou une entrée `skills` dans le manifeste pointe vers un fichier. Déplacez `skills/` à la racine du plugin, pointez chaque entrée `skills` vers un répertoire qui contient `SKILL.md`, et exécutez `/reload-plugins` dans la session. Voir [Le plugin se charge mais ses skills manquent](/docs/fr/plugins/troubleshooting#plugin-loads-but-its-skills-are-missing).

<h3 id="the-userconfig-dialog-never-appears">
  Le dialogue `userConfig` n'apparaît jamais
</h3>

Le dialogue pour les options [`userConfig`](/docs/fr/plugins/components#user-configuration) de votre plugin fait partie de l'installation via `/plugin` dans une session. Charger avec `--plugin-dir` ne l'affiche pas, et `claude plugin install` dans le shell non plus. Avec le plugin chargé, exécutez `/plugin configure <plugin-name>` dans la session pour l'ouvrir. Voir [Le dialogue `userConfig` n'apparaît jamais](/docs/fr/plugins/troubleshooting#the-userconfig-dialog-never-appears).

<h3 id="check-that-the-plugin-changes-claude’s-behavior">
  Vérifier que le plugin change le comportement de Claude
</h3>

Un plugin qui se charge sans erreurs peut toujours échouer à diriger Claude de la façon que vous avez l'intention. `claude plugin eval`, que vous exécutez dans votre shell, exécute vos cas de test avec et sans le plugin et note la différence. Voir [Tester les plugins avec des evals](/docs/fr/plugin-evals), en commençant par [Créer votre première suite d'eval](/docs/fr/plugin-evals#create-your-first-eval-suite).

<h2 id="convert-an-existing-claude-setup">
  Convertir une configuration `.claude/` existante
</h2>

Si vous avez déjà des skills, agents ou hooks sous le répertoire `.claude/` d'un projet, vous pouvez les déplacer dans un plugin sans les réécrire.

Exécutez les commandes de ces étapes à partir de la racine du projet, qui est le répertoire qui contient `.claude/`, car les chemins `cp` sont relatifs à celui-ci.

<Steps>
  <Step title="Créer la structure du plugin">
    Créez le répertoire du plugin et son dossier `.claude-plugin/` à côté de `.claude/`. Vous pouvez déplacer le plugin n'importe où après.

    ```bash theme={null}
    mkdir -p my-plugin/.claude-plugin
    ```

    Créez `my-plugin/.claude-plugin/plugin.json` :

    ```json my-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-plugin",
      "description": "Migrated from standalone configuration",
      "version": "1.0.0"
    }
    ```
  </Step>

  <Step title="Copier vos fichiers existants">
    Copiez chaque répertoire de configuration que vous avez à la racine du plugin, et ignorez la commande pour tout répertoire que vous n'avez pas.

    ```bash theme={null}
    cp -r .claude/commands my-plugin/
    ```

    ```bash theme={null}
    cp -r .claude/agents my-plugin/
    ```

    ```bash theme={null}
    cp -r .claude/skills my-plugin/
    ```

    Exécutez `ls -a my-plugin` pour confirmer que chaque répertoire que vous avez copié apparaît à côté de `.claude-plugin`.
  </Step>

  <Step title="Déplacer vos hooks">
    Si vous avez des hooks dans `.claude/settings.json` ou `.claude/settings.local.json`, créez un répertoire de hooks :

    ```bash theme={null}
    mkdir -p my-plugin/hooks
    ```

    Créez `my-plugin/hooks/hooks.json` et copiez l'objet `hooks` de votre fichier de paramètres dedans. Le format est le même.

    Cet exemple montre la forme avec un hook qui exécute un linter sur chaque fichier que Claude écrit ou édite. Remplacez l'exemple par votre propre objet `hooks`.

    ```json my-plugin/hooks/hooks.json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npm run lint:fix" }]
          }
        ]
      }
    }
    ```
  </Step>

  <Step title="Tester le plugin migré">
    Chargez le plugin pour une session :

    ```bash theme={null}
    claude --plugin-dir ./my-plugin
    ```

    Vérifiez chaque composant sous son nouveau nom :

    * **Skills** : exécutez `/my-plugin:deploy` pour un skill qui était `/deploy`.
    * **Sous-agents** : demandez à Claude d'utiliser l'agent `my-plugin:reviewer` pour un agent qui était `reviewer`.
    * **Hooks** : déclenchez l'événement que chaque hook correspond.

    Si quelque chose manque, travaillez à travers [Tester et déboguer](#test-and-debug).
  </Step>
</Steps>

Tant que les originaux sont toujours sous `.claude/`, ils restent chargés à côté des copies du plugin :

* **Skills et agents** : les deux ensembles ne se heurtent pas, car les skills et agents du plugin portent le préfixe `my-plugin:`. `/deploy` et `/my-plugin:deploy` fonctionnent tous les deux, et Claude voit `reviewer` et `my-plugin:reviewer` comme deux sous-agents.
* **Hooks** : les hooks n'ont pas de préfixe, donc un hook qui est à la fois dans votre fichier de paramètres et dans `hooks/hooks.json` s'exécute deux fois chaque fois que son événement se déclenche.

Après avoir confirmé que le plugin fonctionne, supprimez les originaux de `.claude/` et supprimez l'objet `hooks` de votre fichier de paramètres.

<h2 id="next-steps">
  Étapes suivantes
</h2>

* [Composants de plugin](/docs/fr/plugins/components) : ajoutez des agents, des hooks, des serveurs MCP, des serveurs LSP et une configuration utilisateur à votre plugin
* [Tester les plugins avec des evals](/docs/fr/plugin-evals) : écrivez des cas d'eval et exécutez-les avec `claude plugin eval` pour vérifier la fiabilité avec laquelle le plugin guide le comportement de Claude
* [Publier un plugin](/docs/fr/plugins/publish) : versionnez-le, mettez-le dans une marketplace et soumettez-le à la marketplace communautaire
* [Plugins sur claude.ai et dans Cowork](https://claude.com/docs/plugins/overview) : le même dossier de plugin s'installe sur claude.ai et dans Cowork. Certains composants sont uniquement Claude Code
* [Référence du manifeste de plugin](/docs/fr/plugins/manifest-reference) : chaque champ `plugin.json`, règle de chemin et répertoire
* [Skills](/docs/fr/skills) : écrivez les skills que votre plugin fournit
* [Plugins d'Anthropic dans le référentiel claude-code](https://github.com/anthropics/claude-code/tree/main/plugins) : des exemples complets de la disposition sur cette page, tels que `feature-dev` et `code-review`
