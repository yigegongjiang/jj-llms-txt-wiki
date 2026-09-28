> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Sécurité et confiance des plugins

> Décidez si vous faites confiance à un plugin avant de l'installer, de ce qu'un plugin peut faire sur votre machine à la façon de l'examiner et de le supprimer.

Un plugin Claude Code que vous installez peut exécuter du code arbitraire sur votre machine avec vos privilèges utilisateur.

Vous installez un plugin à partir d'une marketplace, qui est le catalogue que Claude Code récupère. Certains noms de marketplace sont [réservés aux propres marketplaces d'Anthropic](#marketplace-tiers), et toute autre marketplace est tierce. Le nom d'une marketplace vous indique qui publie le catalogue, pas ce que chaque plugin qu'elle contient fait, donc [examinez un plugin avant de l'installer](#review-a-plugin-before-you-install) quelle que soit la marketplace d'où il provient.

Lisez cette page si vous décidez d'installer un plugin, ou si vous examinez les outils avant que votre équipe puisse les utiliser.

<Note>
  Ces cas sont couverts sur d'autres pages :

  * **Modèle de sécurité propre de Claude Code** : voir [Sécurité](/docs/fr/security)
  * **Restriction ou obligation des plugins pour une organisation** : voir [Gérer les plugins pour votre organisation](/docs/fr/plugins/org)
  * **Les plugins `security-guidance` ou `claude-security`** : cette page ne concerne pas ces plugins. Voir [`security-guidance`](/docs/fr/security-guidance) et [`claude-security`](/docs/fr/claude-security)
</Note>

Commencez par [ce qu'un plugin peut faire](#understand-what-a-plugin-can-do) et [quelles marketplaces sont celles d'Anthropic](#marketplace-tiers), puis [examinez le plugin avant de l'installer](#review-a-plugin-before-you-install).

<h2 id="understand-what-a-plugin-can-do">
  Comprendre ce qu'un plugin peut faire
</h2>

Un plugin peut contenir du contenu qui exécute du code sur votre machine avec vos privilèges utilisateur et du contenu qui entre dans le contexte de Claude en tant qu'instructions, donc [examinez un plugin avant de l'installer](#review-a-plugin-before-you-install). Voici ce qu'un plugin installé peut faire :

* **Hooks** : les [hooks](/docs/fr/hooks) d'un plugin s'exécutent en tant que commandes shell à des points du cycle de vie de Claude Code, comme avant ou après un appel d'outil.
* **Serveurs MCP et LSP** : Claude Code se connecte aux [serveurs MCP](/docs/fr/mcp) qu'un plugin activé déclare et donne à Claude leurs outils. Un serveur MCP stdio s'exécute en tant que processus que Claude Code démarre sur votre machine. Claude Code démarre également les serveurs de langage que le plugin déclare.
* **Répertoire `bin/`** : Claude Code ajoute le répertoire `bin/` de chaque plugin activé au `PATH` du shell de l'outil Bash, afin que les commandes Bash de Claude puissent exécuter n'importe quel exécutable qui s'y trouve.
* **Skills, commandes et agents** : ceux-ci entrent dans le contexte de Claude en tant qu'instructions, ils influencent donc ce que Claude fait avec les outils qu'il a déjà.
* **Mises à jour** : quand la mise à jour automatique est activée pour la marketplace à partir de laquelle vous avez installé un plugin, Claude Code met à jour ce plugin en arrière-plan, donc les fichiers que vous avez examinés peuvent changer sur le disque. [Quand la mise à jour automatique s'exécute](/docs/fr/plugins/loading#when-auto-update-runs) indique le calendrier. Pour activer ou désactiver la mise à jour automatique par marketplace, voir [Garder les plugins à jour](/docs/fr/plugins/install#keep-plugins-updated).

Les [règles de permission](/docs/fr/permissions) et le [sandbox](/docs/fr/sandboxing) de Claude Code couvrent les appels d'outils que Claude fait, pas le code qu'un plugin exécute par lui-même :

* **Hooks et processus serveur** : les hooks de commande exécutent des commandes shell avec vos permissions utilisateur complètes. Claude Code exécute les hooks et les serveurs MCP en dehors du sandbox.
* **Appels d'outils de Claude** : un appel à l'un des outils MCP du plugin, et une commande Bash qui exécute un exécutable du `bin/` du plugin, sont des appels d'outils, donc vos règles de permission s'y appliquent.

L'installation d'un plugin l'active également, sauf si son manifeste ou son entrée de marketplace définit [`defaultEnabled: false`](/docs/fr/plugins/install#choose-an-install-scope) et que vous ne l'avez pas activé vous-même.

Pour supprimer un plugin auquel vous ne faites plus confiance, voir [Supprimer un plugin auquel vous ne faites plus confiance](#remove-a-plugin-you-no-longer-trust).

<h2 id="marketplace-tiers">
  Identifier les marketplaces d'Anthropic par nom
</h2>

Le nom d'une marketplace la place dans l'un des trois niveaux : officiel, communautaire ou tiers. Claude Code n'accepte les noms officiels et communautaires que pour les marketplaces provenant de repositories `github.com/anthropics/`, donc une marketplace tierce ne peut pas se présenter comme une marketplace d'Anthropic. Une marketplace qu'un collègue ou votre organisation publie est tierce.

Le tableau liste les noms qui se situent dans chaque niveau :

| Niveau        | Quelles marketplaces                                                                              |
| :------------ | :------------------------------------------------------------------------------------------------ |
| Officiel      | Les [noms de marketplace officiels](#official-marketplace-names), comme `claude-plugins-official` |
| Communautaire | `claude-community`, `claude-plugins-community`, et `healthcare`                                   |
| Tiers         | Toute autre marketplace                                                                           |

Quand le catalogue `claude-community` épingle un plugin à un SHA de commit, ce qu'il fait pour presque chaque entrée, Claude Code refuse d'installer un commit différent.

<h3 id="official-marketplace-names">
  Noms de marketplace officiels
</h3>

Ces noms de marketplace constituent le niveau officiel :

* `claude-plugins-official`
* `claude-code-marketplace`
* `claude-code-plugins`
* `anthropic-marketplace`
* `anthropic-plugins`
* `agent-skills`
* `anthropic-agent-skills`
* `life-sciences`
* `knowledge-work-plugins`
* `claude-for-legal`
* `claude-for-financial-services`
* `financial-services-plugins`
* `first-party-plugins`
* `claude-tag-plugins`

Pour savoir comment les marketplaces officielles, communautaires et de démonstration diffèrent et où parcourir ce que chacune liste, voir [Les marketplaces d'Anthropic](/docs/fr/plugins/anthropic-marketplaces).

<h2 id="review-a-plugin-before-you-install">
  Examiner un plugin avant de l'installer
</h2>

Avant d'installer un plugin, regardez ce qu'il ajoute et d'où il provient.

<Steps>
  <Step title="Vérifier la source de la marketplace">
    Dans votre shell, exécutez `claude plugin marketplace list` pour imprimer la source à partir de laquelle chaque marketplace a été ajoutée, comme un repository GitHub ou un répertoire.
  </Step>

  <Step title="Lire le volet de détails">
    Dans une session Claude Code, exécutez `/plugin` et sélectionnez le plugin. Le volet de détails affiche une section **Will install** (Sera installé) listant les commandes, agents, skills, hooks et serveurs MCP et LSP du plugin. Pour un plugin pour lequel Anthropic n'a pas publié de données de composant, la section affiche ce que l'entrée de marketplace déclare, ou une note : `Components will be discovered at installation` (Les composants seront découverts à l'installation) pour un plugin stocké dans la marketplace, ou `Component summary not available for remote plugin` (Résumé des composants non disponible pour le plugin distant) pour un plugin récupéré ailleurs.
  </Step>

  <Step title="Lire la source du plugin">
    Dans le volet de détails, sélectionnez **Open homepage** (Ouvrir la page d'accueil) ou **View on GitHub** (Voir sur GitHub) sous les options d'installation. Si le volet n'offre ni l'un ni l'autre, ouvrez le repository de marketplace que vous avez trouvé à la première étape. Trouvez le répertoire du plugin là-bas. La section **Will install** (Sera installé) montre qu'un hook existe mais pas ce qu'il exécute, donc lisez ces fichiers dans le répertoire du plugin :

    * **`hooks/hooks.json`** : la commande que chaque hook exécute
    * **`.mcp.json`** : la commande ou l'URL de chaque serveur
    * **`bin/`** : chaque fichier du répertoire
  </Step>

  <Step title="Lister ce que le plugin contient">
    Clonez le repository qui contient le répertoire du plugin, puis exécutez `claude --plugin-dir <plugin directory> plugin details <plugin name>` dans votre shell pour voir ce que Claude Code trouve dedans. La commande lit les fichiers du plugin sans démarrer une session et imprime un `Component inventory` (Inventaire des composants) listant les skills et commandes du plugin, les agents, les hooks avec l'événement de chaque hook, et les serveurs MCP et LSP.
  </Step>
</Steps>

Après avoir installé un plugin, exécutez `claude plugin details <plugin name>` dans votre shell pour imprimer le même `Component inventory` (Inventaire des composants) pour la copie installée sous `~/.claude/plugins/cache/<marketplace>/<plugin>/<version>/`.

<h3 id="remove-a-plugin-you-no-longer-trust">
  Supprimer un plugin auquel vous ne faites plus confiance
</h3>

Dans votre shell, exécutez [`claude plugin uninstall <plugin>`](/docs/fr/plugins/cli-reference#plugin-uninstall) avec le `--scope` auquel vous l'avez installé. Ensuite, vérifiez ce que la désinstallation a supprimé et ce qu'elle a laissé :

* **Données persistantes** : quand c'était le dernier scope auquel le plugin était installé, la désinstallation supprime également le répertoire de données persistantes du plugin, sauf si vous passez `--keep-data`.
* **Fichiers en cache** : les fichiers du plugin restent sur le disque sous `~/.claude/plugins/cache/` pendant 14 jours avant qu'un [balayage en arrière-plan les supprime](/docs/fr/plugins/loading#cleanup-of-previous-versions). Après avoir désinstallé votre dernier plugin, les répertoires orphelins restent jusqu'à ce que vous en installiez un autre. Pour supprimer les fichiers maintenant, supprimez vous-même le répertoire du plugin sous `~/.claude/plugins/cache/<marketplace>/<plugin>/`.
* **La marketplace** : si vous ne faites pas confiance au propriétaire de la marketplace non plus, [supprimez la marketplace](/docs/fr/plugins/install#manage-marketplaces) aussi, ce qui désinstalle chaque plugin que vous avez installé à partir de celle-ci.

<h2 id="recognize-when-claude-code-refuses-or-warns">
  Reconnaître quand Claude Code refuse ou avertit
</h2>

Le volet de détails que vous ouvrez à partir de l'onglet **Discover** (Découvrir) ou **Marketplaces** dans `/plugin` affiche le même avertissement de confiance pour chaque plugin. Claude Code refuse au lieu d'avertir dans des cas comme ceux sous [Sources de marketplace non fiables et vérifications d'intégrité échouées](#untrusted-marketplace-sources-and-failed-integrity-checks).

<h3 id="trust-warning-before-you-install">
  Avertissement de confiance avant l'installation
</h3>

L'avertissement lit la même chose quelle que soit la marketplace d'où provient le plugin :

```text theme={null}
Make sure you trust a plugin before installing, updating, or using it. Anthropic does not control what MCP servers, files, or other software are included in plugins and cannot verify that they will work as intended or that they won't change. See each plugin's homepage for more information.
```

Si votre organisation définit `pluginTrustMessage` dans [les paramètres gérés](/docs/fr/plugins/org), Claude Code ajoute ce texte à l'avertissement.

<h3 id="untrusted-marketplace-sources-and-failed-integrity-checks">
  Sources de marketplace non fiables et vérifications d'intégrité échouées
</h3>

Claude Code refuse de charger une marketplace ou d'installer un plugin dans ces cas, chacun avec son propre message d'erreur :

* **Source de marketplace non fiable** : quand une marketplace utilise un nom officiel ou communautaire mais que sa source est en dehors de `github.com/anthropics/`, Claude Code arrête le chargement de la marketplace et les plugins que vous avez installés à partir de celle-ci. L'erreur est [Marketplace is registered from an untrusted source](/docs/fr/errors#marketplace-is-registered-from-an-untrusted-source).
* **Intégrité de l'archive** : quand une entrée de marketplace épingle une [source `archive`](/docs/fr/plugins/marketplace-reference#archive-plugin-source) à un digest `sha256` et que le digest du fichier téléchargé ne correspond pas, Claude Code refuse l'installation. L'erreur est [Plugin archive integrity check failed](/docs/fr/errors#plugin-archive-integrity-check-failed).

L'épingle `sha256` est séparée de l'épingle de commit SHA du catalogue communautaire, qui sélectionne le commit git à extraire.

<h2 id="enforce-plugin-controls-for-your-organization">
  Appliquer les contrôles de plugin pour votre organisation
</h2>

Avec les [paramètres gérés](/docs/fr/plugins/org), un administrateur peut appliquer ces contrôles de plugin :

* Lister en blanc ou en noir les sources de marketplace
* Forcer l'activation des plugins
* Désactiver les drapeaux `--plugin-dir` et `--plugin-url` et la variable `CLAUDE_CODE_PLUGIN_DIRS`
* Limiter les hooks à ceux des paramètres gérés et des plugins activés de force
* Empêcher les plugins des comptes claude.ai des membres de se charger dans Claude Code, avec [`syncClaudeAiPlugins`](/docs/fr/plugins/org#control-matrix)

La [matrice de contrôle](/docs/fr/plugins/org#control-matrix) indique ce que chaque clé fait et ne couvre pas.

<h2 id="find-plugins-in-telemetry">
  Trouver les plugins dans la télémétrie
</h2>

Si votre organisation exporte les événements [OpenTelemetry](/docs/fr/monitoring-usage) de Claude Code vers son propre backend, les [niveaux de marketplace](#marketplace-tiers) décident quels noms de plugin y apparaissent :

* **[Événement Plugin loaded](/docs/fr/monitoring-usage#plugin-loaded-event)** : l'événement rapporte les noms de plugin et de marketplace du niveau officiel tels qu'ils sont. Pour les niveaux communautaire et tiers, `plugin.name` et `marketplace.name` sont la chaîne littérale `third-party` sauf si vous définissez `OTEL_LOG_TOOL_DETAILS=1`.
* **Scope du plugin** : le `plugin.scope` de l'événement chargé rapporte toujours d'où provient le plugin, comme `org` pour un plugin que vos paramètres gérés activent ou `user-local` pour tout autre plugin tiers. L'[événement Plugin loaded](/docs/fr/monitoring-usage#plugin-loaded-event) liste chaque valeur.
* **[Événement Plugin installed](/docs/fr/monitoring-usage#plugin-installed-event)** : sauf si vous définissez `OTEL_LOG_TOOL_DETAILS=1`, l'événement omet les champs de nom pour les plugins non officiels au lieu de rapporter `third-party`.
* **[API Claude Code Analytics](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list)** : Claude Code rapporte les plugins des niveaux officiel et communautaire par nom et rapporte chaque autre plugin comme `third-party`.

<h2 id="next-steps">
  Étapes suivantes
</h2>

* [Gérer les plugins pour votre organisation](/docs/fr/plugins/org) : restreindre les marketplaces à partir desquelles les utilisateurs peuvent installer et exiger celles auxquelles vous faites confiance
* [Installer et gérer les plugins](/docs/fr/plugins/install) : examinez le volet de détails d'un plugin avant de choisir un scope
* [Les marketplaces d'Anthropic](/docs/fr/plugins/anthropic-marketplaces) : quels noms de marketplace sont ceux d'Anthropic
* [Sécurité](/docs/fr/security) : le propre modèle de sécurité de Claude Code
