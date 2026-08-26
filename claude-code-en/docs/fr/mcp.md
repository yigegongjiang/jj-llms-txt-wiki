> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Connecter Claude Code aux outils via MCP

> Découvrez comment connecter Claude Code à vos outils avec le Model Context Protocol.

Claude Code peut se connecter à des centaines d'outils externes et de sources de données via le [Model Context Protocol (MCP)](https://modelcontextprotocol.io/introduction), une norme open source pour les intégrations IA-outils. Les serveurs MCP donnent à Claude Code accès à vos outils, bases de données et API.

Connectez un serveur lorsque vous vous trouvez à copier des données dans le chat à partir d'un autre outil, comme un suivi de problèmes ou un tableau de bord de surveillance. Une fois connecté, Claude peut lire et agir sur ce système directement au lieu de travailler à partir de ce que vous collez.

Si vous connectez votre premier serveur, commencez par le [guide de démarrage MCP](/docs/fr/mcp-quickstart) pour une procédure pas à pas. Cette page est la référence complète.

<h2 id="what-you-can-do-with-mcp">
  Ce que vous pouvez faire avec MCP
</h2>

Avec les serveurs MCP connectés, vous pouvez demander à Claude Code de :

* **Implémenter des fonctionnalités à partir de suivi de problèmes** : « Ajouter la fonctionnalité décrite dans le problème JIRA ENG-4521 et créer une PR sur GitHub. »
* **Analyser les données de surveillance** : « Vérifier Sentry et Statsig pour vérifier l'utilisation de la fonctionnalité décrite dans ENG-4521. »
* **Interroger les bases de données** : « Trouver les e-mails de 10 utilisateurs aléatoires qui ont utilisé la fonctionnalité ENG-4521, en fonction de notre base de données PostgreSQL. »
* **Intégrer les conceptions** : « Mettre à jour notre modèle d'e-mail standard en fonction des nouvelles conceptions Figma qui ont été publiées sur Slack »
* **Automatiser les flux de travail** : « Créer des brouillons Gmail invitant ces 10 utilisateurs à une session de rétroaction sur la nouvelle fonctionnalité. »
* **Réagir aux événements externes** : Un serveur MCP peut également agir comme un [canal](/docs/fr/channels) qui pousse des messages dans votre session, afin que Claude réagisse aux messages Telegram, aux discussions Discord ou aux événements webhook pendant que vous êtes absent.

<h2 id="find-and-build-mcp-servers">
  Trouver et créer des serveurs MCP
</h2>

Parcourez les connecteurs vérifiés dans le [Répertoire Anthropic](https://claude.ai/directory). Les connecteurs du répertoire utilisent la même infrastructure MCP que Claude Code, vous pouvez donc ajouter n'importe quel serveur distant répertorié avec `claude mcp add`.

<Warning>
  Vérifiez que vous faites confiance à chaque serveur avant de le connecter. Les serveurs qui récupèrent du contenu externe peuvent vous exposer à un [risque d'injection de prompt](/docs/fr/security#protect-against-prompt-injection).
</Warning>

Pour créer votre propre serveur, consultez le [guide du serveur MCP](https://modelcontextprotocol.io/docs/develop/build-server) pour les principes fondamentaux du protocole et la [documentation de création de connecteurs Claude](https://claude.com/docs/connectors/building) pour l'authentification, les tests et la soumission au répertoire.

Vous pouvez également faire en sorte que Claude crée un serveur pour vous avec le plugin officiel [`mcp-server-dev`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/mcp-server-dev).

<Steps>
  <Step title="Installer le plugin">
    Dans une session Claude Code, exécutez :

    ```
    /plugin install mcp-server-dev@claude-plugins-official
    ```

    Si Claude Code signale que le marketplace n'est pas trouvé, exécutez d'abord `/plugin marketplace add anthropics/claude-plugins-official`, puis réessayez l'installation. Une fois installé, exécutez `/reload-plugins` pour l'activer dans la session actuelle.
  </Step>

  <Step title="Exécuter la compétence de création">
    ```
    /mcp-server-dev:build-mcp-server
    ```

    Claude vous pose des questions sur votre cas d'usage et crée un serveur HTTP distant ou un serveur stdio local.
  </Step>
</Steps>

<h2 id="installing-mcp-servers">
  Installation des serveurs MCP
</h2>

Les serveurs MCP peuvent être configurés de plusieurs façons selon vos besoins :

<h3 id="option-1-add-a-remote-http-server">
  Option 1 : Ajouter un serveur HTTP distant
</h3>

Les serveurs HTTP sont l'option recommandée pour se connecter aux serveurs MCP distants. C'est le transport le plus largement supporté pour les services basés sur le cloud.

```bash theme={null}
# Syntaxe de base
claude mcp add --transport http <name> <url>

# Exemple réel : Se connecter à Notion
claude mcp add --transport http notion https://mcp.notion.com/mcp

# Exemple avec jeton Bearer
claude mcp add --transport http secure-api https://api.example.com/mcp \
  --header "Authorization: Bearer your-token"
```

Lors de la configuration des serveurs MCP via JSON dans `.mcp.json`, `~/.claude.json`, ou `claude mcp add-json`, le champ `type` accepte `streamable-http` comme alias pour `http`. La spécification MCP utilise le nom `streamable-http` pour ce transport, donc les configurations copiées à partir de la documentation du serveur fonctionnent sans modification.

Une entrée JSON qui a une `url` mais pas de `type` est une erreur de configuration, car Claude Code lit une entrée sans `type` comme un serveur stdio. Claude Code ignore ce serveur et signale `MCP server "<name>" has a "url" but no "type"; add "type": "http" (or "sse" / "ws") to this entry`. Avant la v2.1.202, Claude Code signalait cette mauvaise configuration comme `command: expected string, received undefined`.

<h3 id="option-2-add-a-remote-sse-server">
  Option 2 : Ajouter un serveur SSE distant
</h3>

<Warning>
  Le transport SSE (Server-Sent Events) est déprécié. Utilisez plutôt les serveurs HTTP, si disponibles.
</Warning>

```bash theme={null}
# Syntaxe de base
claude mcp add --transport sse <name> <url>

# Exemple réel : Se connecter à Asana
claude mcp add --transport sse asana https://mcp.asana.com/sse

# Exemple avec en-tête d'authentification
claude mcp add --transport sse private-api https://api.company.com/sse \
  --header "X-API-Key: your-key-here"
```

<h3 id="option-3-add-a-local-stdio-server">
  Option 3 : Ajouter un serveur stdio local
</h3>

Les serveurs Stdio s'exécutent en tant que processus locaux sur votre machine. Ils sont idéaux pour les outils qui ont besoin d'un accès direct au système ou de scripts personnalisés.

Claude Code définit `CLAUDE_PROJECT_DIR` dans l'environnement du serveur généré à la racine du projet, afin que votre serveur puisse résoudre les chemins relatifs au projet sans dépendre du répertoire de travail. C'est le même répertoire que les hooks reçoivent dans leur variable `CLAUDE_PROJECT_DIR`. Lisez-le depuis l'intérieur de votre processus serveur, par exemple `process.env.CLAUDE_PROJECT_DIR` en Node ou `os.environ["CLAUDE_PROJECT_DIR"]` en Python.

`CLAUDE_PROJECT_DIR` est la racine du projet stable et ne change pas lorsque vous ajoutez ou supprimez des répertoires de travail en cours de session. Un serveur qui limite son propre accès au système de fichiers à un ensemble de répertoires autorisés devrait implémenter la demande MCP `roots/list` à la place. Claude Code répond à `roots/list` avec le répertoire de lancement de la session plus chaque [répertoire de travail supplémentaire](/docs/fr/permissions#working-directories) que vous avez accordé avec `--add-dir`, `/add-dir`, ou le paramètre `additionalDirectories`. Claude Code envoie `notifications/roots/list_changed` lorsque cet ensemble change. Avant la v2.1.203, `roots/list` retournait uniquement le répertoire de lancement et Claude Code n'envoyait pas `notifications/roots/list_changed`.

Cette variable est définie dans l'environnement du serveur, pas dans l'environnement propre de Claude Code, donc la référencer via l'expansion `${VAR}` dans un fichier `.mcp.json` de portée projet ou utilisateur `command` ou `args` nécessite une valeur par défaut telle que `${CLAUDE_PROJECT_DIR:-.}`. Les configurations MCP fournies par les plugins remplacent `${CLAUDE_PROJECT_DIR}` directement et n'ont pas besoin de la valeur par défaut.

```bash theme={null}
# Syntaxe de base
claude mcp add [options] <name> -- <command> [args...]

# Exemple réel : Ajouter un serveur Airtable
claude mcp add --env AIRTABLE_API_KEY=YOUR_KEY --transport stdio airtable \
  -- npx -y airtable-mcp-server
```

<Note>
  **Important : Séparer les arguments du serveur avec `--`**

  Pour les serveurs stdio, le `--` (double tiret) sépare les propres options de Claude, telles que `--transport`, `--env`, et `--scope`, de la commande et des arguments qui exécutent le serveur. Tout ce qui suit `--` est transmis au serveur sans modification.

  Par exemple :

  * `claude mcp add --transport stdio myserver -- npx server` → exécute `npx server`
  * `claude mcp add --env KEY=value --transport stdio myserver -- python server.py --port 8080` → exécute `python server.py --port 8080` avec `KEY=value` dans l'environnement

  Sans `--`, Claude Code essaierait d'analyser les drapeaux du serveur, comme `--port` ci-dessus, comme ses propres options.

  `--env` accepte plusieurs paires `KEY=value`. Si le nom du serveur vient directement après `--env`, la CLI lit le nom comme une autre paire et le rejette, donc placez au moins une autre option entre `--env` et le nom du serveur, comme dans les exemples ci-dessus.
</Note>

<h3 id="option-4-add-a-remote-websocket-server">
  Option 4 : Ajouter un serveur WebSocket distant
</h3>

Les serveurs WebSocket maintiennent une connexion bidirectionnelle persistante, ce qui convient aux serveurs MCP distants qui poussent des événements vers Claude sans être sollicités. Utilisez plutôt HTTP lorsque votre serveur répond uniquement aux demandes, car HTTP supporte OAuth et le drapeau `claude mcp add --transport`, tandis que WebSocket ne supporte ni l'un ni l'autre.

Configurez les serveurs WebSocket dans `.mcp.json` ou avec `claude mcp add-json` :

```bash theme={null}
claude mcp add-json events-server \
  '{"type":"ws","url":"wss://mcp.example.com/socket","headers":{"Authorization":"Bearer YOUR_TOKEN"}}'
```

L'entrée `type: "ws"` accepte les mêmes champs `url`, `headers`, `headersHelper`, `timeout`, et `alwaysLoad` que `http`. L'authentification est basée sur les en-têtes uniquement, donc passez un jeton statique dans `headers` ou générez-en un au moment de la connexion avec [`headersHelper`](#use-dynamic-headers-for-custom-authentication). Le drapeau `claude mcp add --transport` n'accepte pas `ws`.

<h3 id="managing-your-servers">
  Gestion de vos serveurs
</h3>

Une fois configurés, vous pouvez gérer vos serveurs MCP avec ces commandes :

```bash theme={null}
# Lister tous les serveurs configurés
claude mcp list

# Obtenir les détails d'un serveur spécifique
claude mcp get github

# Supprimer un serveur
claude mcp remove github

# (dans Claude Code) Vérifier l'état du serveur
/mcp
```

Les serveurs de portée projet à partir de `.mcp.json` qui attendent votre approbation apparaissent dans `claude mcp list` comme `⏸ Approbation en attente`. Exécutez `claude` de manière interactive pour les examiner et les approuver. `claude mcp get <name>` affiche les serveurs en attente comme `⏸ Approbation en attente` et les serveurs rejetés comme `✗ Rejeté`.

À partir de la v2.1.196, `claude mcp list` et `claude mcp get` lisent les approbations `.mcp.json` uniquement à partir des fichiers de paramètres qui ne sont pas archivés dans le référentiel jusqu'à ce que vous fassiez confiance à l'espace de travail en exécutant `claude` et en acceptant la boîte de dialogue de confiance de l'espace de travail. Un référentiel cloné ne peut pas approuver ses propres serveurs : [`enableAllProjectMcpServers` ou `enabledMcpjsonServers`](/docs/fr/settings#available-settings) archivé dans le `.claude/settings.json` du projet est ignoré dans un dossier non approuvé, et le serveur reste à `⏸ Approbation en attente` au lieu d'être connecté et vérifié.

Les approbations de ces sources s'appliquent toujours dans un dossier non approuvé :

* votre `~/.claude/settings.json` utilisateur
* paramètres gérés
* paramètres passés avec `--settings`

Les approbations dans un `.claude/settings.local.json` non suivi s'appliquent également, mais uniquement après que vous acceptiez une boîte de dialogue de confiance pour ce dossier ou l'un de ses répertoires parents : Claude Code exécute git pour vérifier si le fichier est suivi, et il n'exécute cette vérification que dans un dossier approuvé. Dans un dossier que vous n'avez jamais approuvé, les approbations du fichier attendent la boîte de dialogue de confiance sauf si le dossier est votre répertoire de configuration personnel : votre répertoire personnel, ou un répertoire dont vous avez défini le `.claude` comme [`CLAUDE_CONFIG_DIR`](/docs/fr/env-vars). Avant la v2.1.207, un `.claude/settings.local.json` non suivi approuvait les serveurs dans un dossier que vous n'aviez jamais approuvé.

Une entrée `disabledMcpjsonServers` dans n'importe quel fichier de paramètres rejette toujours le serveur.

Le panneau `/mcp` affiche le nombre d'outils à côté de chaque serveur connecté et signale les serveurs qui annoncent la capacité des outils mais n'exposent aucun outil.

Un serveur distant dont la configuration a une `url` vide s'affiche comme `not configured` dans `/mcp`, dans `claude mcp list`, et dans le [gestionnaire `/plugin`](/docs/fr/plugins), et Claude Code ne tente pas de s'y connecter. Un plugin peut inclure une entrée d'espace réservé comme celle-ci pour un connecteur que vous configurez ultérieurement, afin que Claude Code ne la signale pas comme une erreur ou un problème de configuration. La vue détaillée du serveur dans `/mcp` lit `No URL configured for this server` ; définissez la `url` de l'entrée pour vous connecter. Avant la v2.1.208, Claude Code signalait une `url` vide comme un problème de configuration avec une invite de reconnexion.

Si votre demande a besoin d'outils d'un serveur qui se connecte toujours en arrière-plan, Claude attend que ce serveur se connecte avant de continuer. Avec la [recherche d'outils](#scale-with-mcp-tool-search) activée, qui est l'option par défaut, l'attente se produit à l'intérieur de l'appel `ToolSearch`. Dans les configurations sans recherche d'outils, telles que la plateforme Agent de Google Cloud, une `ANTHROPIC_BASE_URL` personnalisée, ou `ENABLE_TOOL_SEARCH=false`, Claude utilise plutôt l'outil `WaitForMcpServers`.

Certains noms de serveur sont réservés aux serveurs intégrés de Claude Code : `workspace`, `claude-in-chrome`, `computer-use`, `Claude Preview`, et `Claude Browser`. Si votre configuration définit un serveur avec un nom réservé, Claude Code le saute au chargement et affiche un avertissement vous demandant de le renommer. `claude mcp add` rejette un nom réservé avec une erreur.

`Claude Preview` et `Claude Browser` nomment tous deux le serveur intégré que le [volet d'aperçu de l'application de bureau Claude Code](/docs/fr/desktop#preview-your-app) utilise. Avant la v2.1.205, `Claude Browser` n'était pas réservé, donc un serveur configuré par l'utilisateur pouvait s'enregistrer sous ce nom.

<h3 id="dynamic-tool-updates">
  Mises à jour dynamiques des outils
</h3>

Claude Code supporte les notifications MCP `list_changed`, permettant aux serveurs MCP de mettre à jour dynamiquement leurs outils, prompts et ressources disponibles sans vous obliger à vous déconnecter et reconnecter. Lorsqu'un serveur MCP envoie une notification `list_changed`, Claude Code actualise automatiquement les capacités disponibles de ce serveur.

<h3 id="automatic-reconnection">
  Reconnexion automatique
</h3>

Si un serveur HTTP ou SSE se déconnecte en cours de session, Claude Code se reconnecte automatiquement avec un backoff exponentiel : jusqu'à cinq tentatives, en commençant par un délai d'une seconde et en doublant à chaque fois. Le serveur apparaît comme en attente dans `/mcp` pendant que la reconnexion est en cours. Après cinq tentatives échouées, le serveur est marqué comme échoué et vous pouvez réessayer manuellement à partir de `/mcp`. Les serveurs Stdio sont des processus locaux et ne sont pas reconnectés automatiquement.

Le même backoff s'applique lorsqu'un serveur HTTP ou SSE échoue sa connexion initiale au démarrage. À partir de la v2.1.121, Claude Code réessaie la connexion initiale jusqu'à trois fois sur les erreurs transitoires telles qu'une réponse 5xx, une connexion refusée ou un délai d'expiration, puis marque le serveur comme échoué s'il ne peut toujours pas se connecter. Les erreurs d'authentification et les erreurs de non-trouvé ne sont pas réessayées car elles nécessitent une modification de la configuration pour être résolues.

Lorsqu'un serveur configuré échoue à se connecter, Claude Code indique à Claude quel serveur a échoué et son erreur de connexion, y compris dans les résultats `ToolSearch` qui ne trouvent aucun outil correspondant, afin que Claude signale l'échec de la connexion dans sa réponse. Nécessite la [recherche d'outils](#scale-with-mcp-tool-search), qui est activée par défaut. Dans les configurations sans recherche d'outils, telles qu'une `ANTHROPIC_BASE_URL` personnalisée, `ENABLE_TOOL_SEARCH=false`, ou un modèle qui ne supporte pas la recherche d'outils, et sur Amazon Bedrock, la plateforme Agent de Google Cloud, et Microsoft Foundry, Claude Code ne signale pas les échecs de connexion du serveur à Claude. Avant la v2.1.205, Claude Code ne transmettait pas les erreurs de connexion à Claude, et Claude pouvait répondre comme si les outils du serveur échoué n'avaient jamais été configurés.

À partir de la v2.1.191, les demandes de découverte de capacités qui s'exécutent après une connexion réussie, telles que `tools/list`, `prompts/list`, et `resources/list`, réessaient également les erreurs réseau transitoires et les erreurs du serveur jusqu'à trois fois avec un backoff court. Les erreurs d'authentification, les réponses 4xx et les délais d'expiration des demandes ne sont pas réessayés.

<h3 id="push-messages-with-channels">
  Pousser des messages avec des canaux
</h3>

Un serveur MCP peut également pousser des messages directement dans votre session afin que Claude puisse réagir aux événements externes comme les résultats CI, les alertes de surveillance ou les messages de chat. Pour activer cela, votre serveur déclare la capacité `claude/channel` et vous l'activez avec le drapeau `--channels` au démarrage. Consultez [Canaux](/docs/fr/channels) pour utiliser un canal officiellement supporté, ou [Référence des canaux](/docs/fr/channels-reference) pour créer le vôtre.

<Tip>
  Conseils :

  * Utilisez le drapeau `-s` ou `--scope` pour spécifier où la configuration est stockée :
    * `local` (par défaut) : Disponible uniquement pour vous dans le projet actuel. Les versions antérieures appelaient cette portée `project`
    * `project` : Partagé avec tous les membres du projet via le fichier `.mcp.json`
    * `user` : Disponible pour vous dans tous les projets. Les versions antérieures appelaient cette portée `global`
  * Définissez les variables d'environnement avec les drapeaux `-e` ou `--env` (par exemple, `-e KEY=value`)
  * Les drapeaux `--transport` et `--header` acceptent également les formes courtes `-t` et `-H`
  * Configurez le délai d'expiration du démarrage du serveur MCP en utilisant la variable d'environnement `MCP_TIMEOUT` (par exemple, `MCP_TIMEOUT=10000 claude` définit un délai d'expiration de 10 secondes)
  * Définissez un délai d'expiration d'exécution d'outil par serveur en ajoutant un champ `timeout` en millisecondes à l'entrée `.mcp.json` de ce serveur, par exemple `"timeout": 600000` pour dix minutes. Cela remplace la variable d'environnement `MCP_TOOL_TIMEOUT` pour ce serveur uniquement
  * Claude Code affiche un avertissement lorsque la sortie de l'outil MCP dépasse 10 000 jetons et limite la sortie à 25 000 jetons par défaut. Pour augmenter cette limite, définissez la variable d'environnement `MAX_MCP_OUTPUT_TOKENS` (par exemple, `MAX_MCP_OUTPUT_TOKENS=50000`) ; le seuil d'avertissement est fixe. Consultez [Limites et avertissements de sortie MCP](#mcp-output-limits-and-warnings)
  * Utilisez `/mcp` pour vous authentifier auprès des serveurs distants qui nécessitent une authentification OAuth 2.0
</Tip>

Le `timeout` par serveur est une limite stricte en temps réel par appel d'outil, et les notifications de progression du serveur ne l'étendent pas. Les valeurs inférieures à 1000 sont ignorées et passent à `MCP_TOOL_TIMEOUT`, ou à sa valeur par défaut d'environ 28 heures lorsque cette variable n'est pas définie. Pour un serveur HTTP, SSE, ou [connecteur claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai) il y a aussi un deuxième minuteur par requête qui couvre chaque requête jusqu'au premier octet de réponse du serveur. Ce minuteur est de 60 secondes sauf si vous définissez le `timeout` par serveur ou `MCP_TOOL_TIMEOUT` ; définir l'un ou l'autre à 60 secondes ou plus augmente le minuteur par requête à cette valeur, une valeur inférieure ne le raccourcit pas, et la valeur par défaut de 28 heures d'un `MCP_TOOL_TIMEOUT` non défini ne l'alimente jamais. Les serveurs Stdio et WebSocket n'ont pas de minuteur par requête. Avant la v2.1.162, les valeurs inférieures à 1000 étaient arrondies à une seconde.

Un `timeout` par serveur d'au moins 1000 agit également comme un plancher sur le délai d'expiration d'inactivité décrit ci-dessous : Claude Code n'interrompt jamais les appels d'outils de ce serveur pour inactivité plus tôt que le `timeout` par serveur. Nécessite Claude Code v2.1.203 ou ultérieur.

Un appel d'outil à un serveur MCP qui n'envoie aucune réponse et aucune notification de progression pendant la fenêtre d'inactivité s'interrompt avec une erreur au lieu d'attendre la limite en temps réel. Le délai d'expiration d'inactivité nécessite Claude Code v2.1.187 ou ultérieur. Il s'applique à tous les types de serveurs sauf les serveurs IDE et les serveurs en processus du SDK. La fenêtre d'inactivité par défaut est de cinq minutes pour les serveurs HTTP, SSE, WebSocket, et [connecteur claude.ai](#use-mcp-servers-from-claude-ai), et de 30 minutes pour les serveurs stdio. Avant la v2.1.203, les serveurs stdio étaient exempts du délai d'expiration d'inactivité.

Définissez la variable d'environnement [`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](/docs/fr/env-vars) en millisecondes pour modifier la fenêtre d'inactivité, ou définissez-la à `0` pour désactiver la vérification.

<h3 id="plugin-provided-mcp-servers">
  Serveurs MCP fournis par les plugins
</h3>

Les [plugins](/docs/fr/plugins) peuvent regrouper des serveurs MCP, fournissant automatiquement des outils et des intégrations lorsque le plugin est activé. Les serveurs MCP des plugins fonctionnent de manière identique aux serveurs configurés par l'utilisateur.

**Comment fonctionnent les serveurs MCP des plugins** :

* Les plugins définissent les serveurs MCP dans `.mcp.json` à la racine du plugin ou en ligne dans `plugin.json`
* Lorsqu'un plugin est activé, ses serveurs MCP démarrent automatiquement
* Les outils MCP des plugins apparaissent aux côtés des outils MCP configurés manuellement
* Les serveurs des plugins sont gérés via l'installation du plugin, pas via les commandes `/mcp`

**Exemple de configuration MCP du plugin** :

Dans `.mcp.json` à la racine du plugin :

```json theme={null}
{
  "mcpServers": {
    "database-tools": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"],
      "env": {
        "DB_URL": "${DB_URL}"
      }
    }
  }
}
```

Ou en ligne dans `plugin.json` :

```json theme={null}
{
  "name": "my-plugin",
  "mcpServers": {
    "plugin-api": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/api-server",
      "args": ["--port", "8080"]
    }
  }
}
```

**Fonctionnalités MCP du plugin** :

* **Cycle de vie automatique** : Au démarrage de la session, les serveurs des plugins activés se connectent automatiquement. Si vous activez ou désactivez un plugin pendant une session, exécutez `/reload-plugins` pour connecter ou déconnecter ses serveurs MCP
* **Variables d'environnement** : utilisez `${CLAUDE_PLUGIN_ROOT}` pour les fichiers du plugin groupés, `${CLAUDE_PLUGIN_DATA}` pour l'[état persistant](/docs/fr/plugins-reference#persistent-data-directory) qui survit aux mises à jour du plugin, et `${CLAUDE_PROJECT_DIR}` pour la racine du projet stable. La substitution s'applique à :
  * serveurs `stdio` : `command`, `args`, `env`
  * serveurs `http`, `sse`, et `ws` : `url`, `headers`, et `headersHelper`. Avant la v2.1.195, `headersHelper` transmettait l'espace réservé comme une chaîne littérale
* **Accès aux variables d'environnement utilisateur** : Accès aux mêmes variables d'environnement que les serveurs configurés manuellement
* **Types de transport multiples** : Support des transports stdio, SSE, HTTP et WebSocket, bien que le support des transports peut varier selon le serveur

**Affichage des serveurs MCP du plugin** :

```bash theme={null}
# Dans Claude Code, voir tous les serveurs MCP y compris ceux du plugin
/mcp
```

Les serveurs des plugins apparaissent dans la liste avec des indicateurs montrant qu'ils proviennent des plugins.

**Noms des outils MCP du plugin** :

Les outils d'un serveur MCP groupé par un plugin incluent à la fois le nom du plugin et la clé du serveur dans leur nom appelable. La forme complète est `mcp__plugin_<plugin-name>_<server-name>__<tool-name>`, où tout caractère en dehors de `A-Z`, `a-z`, `0-9`, `_`, et `-` est remplacé par `_`. Pour le serveur `database-tools` groupé dans un plugin nommé `my-plugin`, un outil `query` est appelable comme :

```
mcp__plugin_my-plugin_database-tools__query
```

Utilisez ce nom complet lors du référencement de l'outil dans les [règles de permission](/docs/fr/permissions), la liste `allowed-tools` d'une compétence, le [champ `tools` d'un sous-agent](/docs/fr/sub-agents#available-tools), ou un [correspondant de hook](/docs/fr/hooks#match-mcp-tools). Un correspondant de hook écrit contre la clé du serveur nu, tel que `mcp__database-tools__.*`, ne se déclenche jamais pour un serveur groupé par un plugin.

Le serveur lui-même s'enregistre sous le nom délimité `plugin:<plugin-name>:<server-name>`, tel que `plugin:my-plugin:database-tools`. Utilisez ce nom où un nom de serveur configuré est attendu, tel que le [champ `server` d'un hook `mcp_tool`](/docs/fr/hooks#mcp-tool-hook-fields).

**Avantages des serveurs MCP du plugin** :

* **Distribution groupée** : Outils et serveurs emballés ensemble
* **Configuration automatique** : Aucune configuration MCP manuelle nécessaire
* **Cohérence d'équipe** : Tout le monde obtient les mêmes outils lorsque le plugin est installé

Consultez la [référence des composants du plugin](/docs/fr/plugins-reference#mcp-servers) pour plus de détails sur le regroupement des serveurs MCP avec les plugins.

<h2 id="mcp-installation-scopes">
  Portées d'installation MCP
</h2>

Les serveurs MCP peuvent être configurés à trois portées différentes. La portée que vous choisissez contrôle les projets dans lesquels le serveur se charge et si la configuration est partagée avec votre équipe. Les administrateurs peuvent également déployer des serveurs au niveau de l'entreprise via la [configuration gérée](#managed-mcp-configuration).

| Portée                     | Se charge dans           | Partagé avec l'équipe           | Stocké dans                       |
| -------------------------- | ------------------------ | ------------------------------- | --------------------------------- |
| [Local](#local-scope)      | Projet actuel uniquement | Non                             | `~/.claude.json`                  |
| [Projet](#project-scope)   | Projet actuel uniquement | Oui, via le contrôle de version | `.mcp.json` à la racine du projet |
| [Utilisateur](#user-scope) | Tous vos projets         | Non                             | `~/.claude.json`                  |

<h3 id="local-scope">
  Portée locale
</h3>

La portée locale est la portée par défaut. Un serveur à portée locale se charge uniquement dans le projet où vous l'avez ajouté et reste privé pour vous. Claude Code le stocke dans `~/.claude.json` sous le chemin de ce projet, donc le même serveur n'apparaîtra pas dans vos autres projets. Utilisez la portée locale pour les serveurs de développement personnels, les configurations expérimentales ou les serveurs avec des identifiants que vous ne voulez pas dans le contrôle de version.

<Note>
  Le terme « portée locale » pour les serveurs MCP diffère des paramètres locaux généraux. Les serveurs MCP à portée locale sont stockés dans `~/.claude.json` (votre répertoire personnel), tandis que les paramètres locaux généraux utilisent `.claude/settings.local.json` (dans le répertoire du projet). Consultez [Paramètres](/docs/fr/settings#settings-files) pour plus de détails sur les emplacements des fichiers de paramètres.
</Note>

```bash theme={null}
# Ajouter un serveur à portée locale (par défaut)
claude mcp add --transport http stripe https://mcp.stripe.com

# Spécifier explicitement la portée locale
claude mcp add --transport http stripe --scope local https://mcp.stripe.com
```

La commande écrit le serveur dans l'entrée de votre projet actuel dans `~/.claude.json`. L'exemple ci-dessous montre le résultat lorsque vous l'exécutez à partir de `/path/to/your/project` :

```json theme={null}
{
  "projects": {
    "/path/to/your/project": {
      "mcpServers": {
        "stripe": {
          "type": "http",
          "url": "https://mcp.stripe.com"
        }
      }
    }
  }
}
```

<h3 id="project-scope">
  Portée du projet
</h3>

Les serveurs à portée de projet permettent la collaboration d'équipe en stockant les configurations dans un fichier `.mcp.json` à la racine de votre projet. Ce fichier est conçu pour être archivé dans le contrôle de version, garantissant que tous les membres de l'équipe ont accès aux mêmes outils et services MCP. Lorsque vous ajoutez un serveur à portée de projet, Claude Code crée ou met à jour automatiquement ce fichier avec la structure de configuration appropriée.

```bash theme={null}
# Ajouter un serveur à portée de projet
claude mcp add --transport http paypal --scope project https://mcp.paypal.com/mcp
```

Le fichier `.mcp.json` résultant suit un format standardisé :

```json theme={null}
{
  "mcpServers": {
    "shared-server": {
      "command": "/path/to/server",
      "args": [],
      "env": {}
    }
  }
}
```

Pour des raisons de sécurité, Claude Code demande une approbation avant d'utiliser les serveurs à portée de projet à partir des fichiers `.mcp.json`. Si vous devez réinitialiser ces choix d'approbation, utilisez la commande `claude mcp reset-project-choices`.

<h3 id="user-scope">
  Portée utilisateur
</h3>

Les serveurs à portée utilisateur sont stockés dans `~/.claude.json` et offrent une accessibilité inter-projets, les rendant disponibles dans tous les projets de votre machine tout en restant privés pour votre compte utilisateur. Cette portée fonctionne bien pour les serveurs utilitaires personnels, les outils de développement ou les services que vous utilisez fréquemment dans différents projets.

```bash theme={null}
# Ajouter un serveur utilisateur
claude mcp add --transport http hubspot --scope user https://mcp.hubspot.com/anthropic
```

<h3 id="scope-hierarchy-and-precedence">
  Hiérarchie de portée et précédence
</h3>

Lorsque le même serveur est défini à plus d'un endroit, Claude Code s'y connecte une fois, en utilisant la définition de la source avec la plus haute priorité. L'entrée de serveur entière de cette source est utilisée ; les champs ne sont pas fusionnés entre les portées.

1. Portée locale
2. Portée du projet
3. Portée utilisateur
4. [Serveurs fournis par les plugins](/docs/fr/plugins)
5. [Connecteurs claude.ai](#use-mcp-servers-from-claude-ai)

Les trois portées correspondent aux doublons par nom. Les plugins et les connecteurs correspondent par point de terminaison, donc celui qui pointe vers la même URL ou commande qu'un serveur ci-dessus est traité comme un doublon.

<h3 id="environment-variable-expansion-in-mcp-json">
  Expansion des variables d'environnement dans `.mcp.json`
</h3>

Claude Code supporte l'expansion des variables d'environnement dans les fichiers `.mcp.json`, permettant aux équipes de partager des configurations tout en maintenant la flexibilité pour les chemins spécifiques à la machine et les valeurs sensibles comme les clés API.

**Syntaxe supportée :**

* `${VAR}` : se développe à la valeur de la variable d'environnement `VAR`
* `${VAR:-default}` : se développe à `VAR` si défini, sinon utilise `default`

**Emplacements d'expansion :**
Les variables d'environnement peuvent être développées dans :

* `command` : le chemin de l'exécutable du serveur
* `args` : arguments de la ligne de commande
* `env` : variables d'environnement passées au serveur
* `url` : pour les types de serveur HTTP
* `headers` : pour l'authentification du serveur HTTP

**Exemple avec expansion de variable :**

```json theme={null}
{
  "mcpServers": {
    "api-server": {
      "type": "http",
      "url": "${API_BASE_URL:-https://api.example.com}/mcp",
      "headers": {
        "Authorization": "Bearer ${API_KEY}"
      }
    }
  }
}
```

Si une variable d'environnement requise n'est pas définie et n'a pas de valeur par défaut, Claude Code laisse le texte littéral `${VAR}` dans la valeur et signale un avertissement de variable manquante pour ce serveur. La configuration se charge toujours, donc définissez la variable ou ajoutez un fallback `:-default` pour que le serveur démarre avec la valeur que vous avez l'intention d'utiliser.

<h2 id="practical-examples">
  Exemples pratiques
</h2>

<h3 id="example-monitor-errors-with-sentry">
  Exemple : Surveiller les erreurs avec Sentry
</h3>

```bash theme={null}
claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
```

Authentifiez-vous avec votre compte Sentry :

```text theme={null}
/mcp
```

Ensuite, déboguez les problèmes de production :

```text theme={null}
Quelles sont les erreurs les plus courantes au cours des 24 dernières heures ?
```

```text theme={null}
Montrez-moi la trace de pile pour l'erreur ID abc123
```

```text theme={null}
Quel déploiement a introduit ces nouvelles erreurs ?
```

<h3 id="example-connect-to-github-for-code-reviews">
  Exemple : Se connecter à GitHub pour les révisions de code
</h3>

Le serveur MCP distant de GitHub s'authentifie avec un jeton d'accès personnel GitHub transmis en tant qu'en-tête. Pour en obtenir un, ouvrez vos [paramètres de jeton GitHub](https://github.com/settings/personal-access-tokens), générez un nouveau jeton à granularité fine avec accès aux référentiels avec lesquels vous souhaitez que Claude travaille, puis ajoutez le serveur :

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

Ensuite, travaillez avec GitHub :

```text theme={null}
Examinez la PR #456 et suggérez des améliorations
```

```text theme={null}
Créer un nouveau problème pour le bogue que nous venons de trouver
```

```text theme={null}
Montrez-moi toutes les PR ouvertes qui me sont assignées
```

<h3 id="example-query-your-postgresql-database">
  Exemple : Interroger votre base de données PostgreSQL
</h3>

```bash theme={null}
claude mcp add --transport stdio db -- npx -y @bytebase/dbhub \
  --dsn "postgresql://readonly:pass@prod.db.com:5432/analytics"
```

Ensuite, interrogez votre base de données naturellement :

```text theme={null}
Quel est notre revenu total ce mois-ci ?
```

```text theme={null}
Montrez-moi le schéma de la table des commandes
```

```text theme={null}
Trouver les clients qui n'ont pas effectué d'achat depuis 90 jours
```

<h2 id="authenticate-with-remote-mcp-servers">
  S'authentifier auprès des serveurs MCP distants
</h2>

De nombreux serveurs MCP basés sur le cloud nécessitent une authentification. Claude Code supporte OAuth 2.0 pour les connexions sécurisées.

Claude Code marque un serveur distant comme nécessitant une authentification lorsque le serveur répond avec `401 Unauthorized` ou `403 Forbidden`. Pour un serveur auprès duquel vous ne vous êtes pas connecté, l'un ou l'autre code de statut le signale dans `/mcp` afin que vous puissiez compléter le flux OAuth.

Lorsqu'une demande à un serveur OAuth auprès duquel vous vous êtes déjà connecté retourne `401 Unauthorized`, Claude Code actualise le jeton stocké, se reconnecte et réessaie la demande une fois. Il signale le serveur dans `/mcp` uniquement si cette nouvelle tentative échoue également. Avant la v2.1.206, une actualisation de jeton qui échouait pour une raison transitoire, comme une erreur réseau, signalait un serveur OAuth comme nécessitant une authentification pour le reste de la session même si son jeton d'actualisation était toujours valide.

À partir de la v2.1.195, lorsqu'une actualisation de jeton échoue parce que le serveur rejette le jeton d'actualisation stocké, Claude Code affiche immédiatement un avis pointant vers `/mcp`. Le menu du serveur connecté là-bas offre Re-authenticate, afin que vous puissiez vous connecter à nouveau avant que le prochain appel d'outil échoue.

Un serveur personnalisé qui retourne un en-tête `WWW-Authenticate` pointant vers son serveur d'autorisation obtient la même découverte automatique que tout autre serveur distant.

À partir de la v2.1.193, Claude Code affiche également un avis de démarrage lorsqu'un ou plusieurs serveurs configurés nécessitent une authentification, vous n'avez donc pas besoin d'ouvrir `/mcp` pour découvrir quels serveurs nécessitent une connexion.

En mode non interactif, il n'y a pas de panneau `/mcp`, donc Claude Code ne peut pas exécuter le flux OAuth pour vous. À partir de la v2.1.196, lorsqu'un serveur configuré nécessite une authentification lors d'une exécution `claude -p` ou Agent SDK avec [recherche d'outils](#scale-with-mcp-tool-search) activée, ce qui est la valeur par défaut, Claude Code indique à Claude que les outils du serveur ne sont pas disponibles jusqu'à ce que vous l'autorisiez. Claude peut alors nommer le serveur qui nécessite une connexion au lieu de répondre comme si le serveur n'était pas configuré. Complétez la connexion à partir d'une session interactive avec `/mcp` ou `claude mcp login <name>`.

Si vous avez configuré `headers.Authorization` pour le serveur et que le serveur rejette cet en-tête, Claude Code signale la connexion comme échouée au lieu de revenir à OAuth. Vérifiez que le jeton est valide pour le point de terminaison MCP, ou supprimez l'en-tête pour utiliser le flux OAuth.

<Steps>
  <Step title="Ajouter le serveur qui nécessite une authentification">
    Par exemple :

    ```bash theme={null}
    claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
    ```
  </Step>

  <Step title="Utiliser la commande /mcp dans Claude Code">
    Dans Claude Code, utilisez la commande :

    ```text theme={null}
    /mcp
    ```

    Ensuite, suivez les étapes dans votre navigateur pour vous connecter.
  </Step>
</Steps>

<Tip>
  Conseils :

  * Les jetons d'authentification sont stockés de manière sécurisée et actualisés automatiquement
  * Utilisez « Clear authentication » dans le menu `/mcp` pour révoquer l'accès
  * Si votre navigateur ne s'ouvre pas automatiquement, copiez l'URL fournie et ouvrez-la manuellement
  * Si la redirection du navigateur échoue avec une erreur de connexion après l'authentification, collez l'URL de rappel complète de la barre d'adresse de votre navigateur dans l'invite d'URL qui apparaît dans Claude Code
  * L'authentification OAuth fonctionne avec les serveurs HTTP
</Tip>

<h3 id="authenticate-from-the-command-line">
  S'authentifier à partir de la ligne de commande
</h3>

À partir de la v2.1.186, `claude mcp login <name>` exécute le flux OAuth d'un serveur configuré directement depuis votre shell, vous n'avez donc pas besoin d'ouvrir le panneau `/mcp` dans une session.

```bash theme={null}
claude mcp login sentry
```

Pour effacer les identifiants stockés ultérieurement, exécutez `claude mcp logout <name>`.

À partir de la v2.1.191, la commande détecte lorsqu'aucun navigateur local n'est disponible, par exemple lors d'une session SSH ou sur Linux sans serveur d'affichage, et imprime l'URL d'autorisation au lieu d'essayer d'ouvrir un navigateur. Ouvrez l'URL sur votre machine locale, puis collez l'URL de redirection complète de la barre d'adresse de votre navigateur à l'invite. La commande a besoin d'un terminal interactif pour l'étape de collage, donc connectez-vous avec `ssh -t`. Passez `--no-browser` pour forcer l'invite d'URL même lorsqu'un navigateur local est détecté.

```bash theme={null}
claude mcp login sentry --no-browser
```

<h3 id="use-a-fixed-oauth-callback-port">
  Utiliser un port de rappel OAuth fixe
</h3>

Certains serveurs MCP nécessitent un URI de redirection spécifique enregistré à l'avance. Par défaut, Claude Code choisit un port disponible aléatoire pour le rappel OAuth. Utilisez `--callback-port` pour fixer le port afin qu'il corresponde à un URI de redirection pré-enregistré de la forme `http://localhost:PORT/callback`.

Vous pouvez utiliser `--callback-port` seul (avec l'enregistrement dynamique du client) ou ensemble avec `--client-id` (avec les identifiants pré-configurés).

```bash theme={null}
# Port de rappel fixe avec enregistrement dynamique du client
claude mcp add --transport http \
  --callback-port 8080 \
  my-server https://mcp.example.com/mcp
```

<h3 id="use-pre-configured-oauth-credentials">
  Utiliser les identifiants OAuth pré-configurés
</h3>

Certains serveurs MCP ne supportent pas la configuration OAuth automatique via l'enregistrement dynamique du client. Si vous voyez une erreur comme « Incompatible auth server: does not support dynamic client registration », le serveur nécessite des identifiants pré-configurés. Claude Code supporte également les serveurs qui utilisent un document de métadonnées d'ID client (CIMD) au lieu de l'enregistrement dynamique du client, et les découvre automatiquement. Si la découverte automatique échoue, enregistrez d'abord une application OAuth via le portail des développeurs du serveur, puis fournissez les identifiants lors de l'ajout du serveur.

<Steps>
  <Step title="Enregistrer une application OAuth auprès du serveur">
    Créez une application via le portail des développeurs du serveur et notez votre ID client et votre secret client.

    De nombreux serveurs nécessitent également un URI de redirection. Si c'est le cas, choisissez un port et enregistrez un URI de redirection au format `http://localhost:PORT/callback`. Utilisez ce même port avec `--callback-port` à l'étape suivante.
  </Step>

  <Step title="Ajouter le serveur avec vos identifiants">
    Choisissez l'une des méthodes suivantes. Le port utilisé pour `--callback-port` peut être n'importe quel port disponible. Il doit simplement correspondre à l'URI de redirection que vous avez enregistré à l'étape précédente.

    <Tabs>
      <Tab title="claude mcp add">
        Utilisez `--client-id` pour passer l'ID client de votre application. Le drapeau `--client-secret` demande le secret avec une entrée masquée :

        ```bash theme={null}
        claude mcp add --transport http \
          --client-id your-client-id --client-secret --callback-port 8080 \
          my-server https://mcp.example.com/mcp
        ```
      </Tab>

      <Tab title="claude mcp add-json">
        Incluez l'objet `oauth` dans la configuration JSON et passez `--client-secret` comme drapeau séparé :

        ```bash theme={null}
        claude mcp add-json my-server \
          '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"clientId":"your-client-id","callbackPort":8080}}' \
          --client-secret
        ```
      </Tab>

      <Tab title="claude mcp add-json (port de rappel uniquement)">
        Utilisez `--callback-port` sans ID client pour fixer le port tout en utilisant l'enregistrement dynamique du client :

        ```bash theme={null}
        claude mcp add-json my-server \
          '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"callbackPort":8080}}'
        ```
      </Tab>

      <Tab title="CI / variable d'environnement">
        Définissez le secret via une variable d'environnement pour ignorer l'invite interactive :

        ```bash theme={null}
        MCP_CLIENT_SECRET=your-secret claude mcp add --transport http \
          --client-id your-client-id --client-secret --callback-port 8080 \
          my-server https://mcp.example.com/mcp
        ```
      </Tab>
    </Tabs>
  </Step>

  <Step title="S'authentifier dans Claude Code">
    Exécutez `/mcp` dans Claude Code et suivez le flux de connexion du navigateur.
  </Step>
</Steps>

<Tip>
  Conseils :

  * Le secret client est stocké de manière sécurisée dans votre trousseau système (macOS) ou un fichier d'identifiants, pas dans votre configuration
  * Si le serveur utilise un client OAuth public sans secret, utilisez uniquement `--client-id` sans `--client-secret`
  * `--callback-port` peut être utilisé avec ou sans `--client-id`
  * Ces drapeaux s'appliquent uniquement aux transports HTTP et SSE. Ils n'ont aucun effet sur les serveurs stdio
  * Utilisez `claude mcp get <name>` pour vérifier que les identifiants OAuth sont configurés pour un serveur
</Tip>

<h3 id="override-oauth-metadata-discovery">
  Remplacer la découverte des métadonnées OAuth
</h3>

Pointez Claude Code vers une URL de métadonnées spécifique du serveur d'autorisation OAuth pour contourner la chaîne de découverte par défaut. Définissez `authServerMetadataUrl` lorsque les points de terminaison standard du serveur MCP génèrent des erreurs, ou lorsque vous souhaitez acheminer la découverte via un proxy interne. Par défaut, Claude Code vérifie d'abord les métadonnées de ressource protégée RFC 9728 à `/.well-known/oauth-protected-resource`, puis revient aux métadonnées du serveur d'autorisation RFC 8414 à `/.well-known/oauth-authorization-server`.

Définissez `authServerMetadataUrl` dans l'objet `oauth` de la configuration de votre serveur dans `.mcp.json` :

```json theme={null}
{
  "mcpServers": {
    "my-server": {
      "type": "http",
      "url": "https://mcp.example.com/mcp",
      "oauth": {
        "authServerMetadataUrl": "https://auth.example.com/.well-known/openid-configuration"
      }
    }
  }
}
```

L'URL doit utiliser `https://`. Les `scopes_supported` de l'URL des métadonnées remplacent les portées que le serveur en amont annonce.

<h3 id="restrict-oauth-scopes">
  Restreindre les portées OAuth
</h3>

Définissez `oauth.scopes` pour épingler les portées que Claude Code demande pendant le flux d'autorisation. C'est la façon supportée de restreindre un serveur MCP à un sous-ensemble approuvé par l'équipe de sécurité lorsque le serveur d'autorisation en amont annonce plus de portées que vous ne souhaitez accorder. La valeur est une seule chaîne séparée par des espaces, correspondant au format du paramètre `scope` dans RFC 6749 §3.3.

```json theme={null}
{
  "mcpServers": {
    "slack": {
      "type": "http",
      "url": "https://mcp.slack.com/mcp",
      "oauth": {
        "scopes": "channels:read chat:write search:read"
      }
    }
  }
}
```

`oauth.scopes` a la priorité sur `authServerMetadataUrl` et les portées que le serveur découvre à `/.well-known`. Laissez-le non défini pour laisser le serveur MCP déterminer l'ensemble de portées demandées.

À partir de la v2.1.196, lorsque `oauth.scopes` n'est pas défini, Claude Code demande la portée fournie par l'en-tête `WWW-Authenticate` du serveur ou ses métadonnées de ressource protégée, et n'envoie aucun paramètre `scope` lorsque ni l'un ni l'autre ne fournit de portée. Il ne demande plus le catalogue complet `scopes_supported` à partir des métadonnées du serveur d'autorisation découvertes automatiquement. Demander ce catalogue a fait que les fournisseurs d'identité qui annoncent des portées réservées aux administrateurs ou des portées de modèle rejettent la demande d'autorisation avec une erreur `invalid_scope`. Les métadonnées récupérées à partir d'une `authServerMetadataUrl` configurée fournissent toujours ses `scopes_supported` comme portées demandées.

Si le serveur d'autorisation annonce `offline_access` dans `scopes_supported`, Claude Code l'ajoute aux portées épinglées afin que le jeton d'accès puisse être actualisé sans une nouvelle connexion au navigateur.

Si le serveur retourne ultérieurement un 403 `insufficient_scope` pour un appel d'outil, Claude Code se réauthentifie avec les mêmes portées épinglées. Élargissez `oauth.scopes` lorsqu'un outil dont vous avez besoin nécessite une portée en dehors de l'épingle.

<h3 id="use-dynamic-headers-for-custom-authentication">
  Utiliser des en-têtes dynamiques pour l'authentification personnalisée
</h3>

Si votre serveur MCP utilise un schéma d'authentification autre que OAuth (tel que Kerberos, jetons de courte durée ou un SSO interne), utilisez `headersHelper` pour générer des en-têtes de requête au moment de la connexion. Claude Code exécute la commande et fusionne sa sortie dans les en-têtes de connexion.

```json theme={null}
{
  "mcpServers": {
    "internal-api": {
      "type": "http",
      "url": "https://mcp.internal.example.com",
      "headersHelper": "/opt/bin/get-mcp-auth-headers.sh"
    }
  }
}
```

La commande peut également être en ligne :

```json theme={null}
{
  "mcpServers": {
    "internal-api": {
      "type": "http",
      "url": "https://mcp.internal.example.com",
      "headersHelper": "echo '{\"Authorization\": \"Bearer '\"$(get-token)\"'\"}'"
    }
  }
}
```

**Exigences :**

* La commande doit écrire un objet JSON de paires clé-valeur de chaîne sur stdout
* La commande s'exécute dans un shell avec un délai d'expiration de 10 secondes, à partir du répertoire de travail actuel de la session. Utilisez un chemin absolu ou une commande sur `PATH` pour le script
* Les en-têtes dynamiques remplacent tous les `headers` statiques portant le même nom

L'assistant s'exécute à nouveau à chaque connexion (au démarrage de la session et à la reconnexion). Il n'y a pas de mise en cache, donc votre script est responsable de toute réutilisation de jetons.

À partir de la v2.1.193, si un appel d'outil retourne `401 Unauthorized` ou `403 Forbidden`, Claude Code réexécute automatiquement l'assistant, se reconnecte avec les en-têtes frais et réessaie l'appel une fois. Claude Code marque le serveur comme nécessitant une authentification dans `/mcp` uniquement si cette nouvelle tentative échoue également.

Claude Code définit ces variables d'environnement lors de l'exécution de l'assistant :

| Variable                      | Valeur                                                                                                                     |
| :---------------------------- | :------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_MCP_SERVER_NAME` | le nom du serveur MCP                                                                                                      |
| `CLAUDE_CODE_MCP_SERVER_URL`  | l'URL du serveur MCP                                                                                                       |
| `CLAUDE_PLUGIN_ROOT`          | le répertoire racine du plugin. Défini uniquement lorsqu'un [plugin](/docs/fr/plugins-reference#mcp-servers) fournit le serveur |

Utilisez-les pour écrire un script d'assistant unique qui sert plusieurs serveurs MCP.

Pour un serveur fourni par un plugin, l'assistant s'exécute également avec son répertoire de travail défini à la racine du plugin, donc un chemin `headersHelper` relatif se résout à l'intérieur du répertoire du plugin plutôt que par rapport au répertoire de travail de la session. Nécessite Claude Code v2.1.195 ou ultérieur.

Un `headersHelper` fourni par un plugin ne peut pas référencer les valeurs [`${user_config.*}`](/docs/fr/plugins-reference#user-configuration) du plugin, car la commande s'exécute via un shell. Claude Code signale le serveur comme mal configuré avec une [erreur](/docs/fr/errors#plugin-command-references-user-config) et ne substitue pas la valeur. Mettez `${user_config.KEY}` dans le champ `headers` du serveur à la place, qui n'est pas analysé par shell, ou faites en sorte que le script d'assistant lise la valeur à partir de son propre environnement ou d'un fichier de configuration. Avant la v2.1.207, `headersHelper` substituait les valeurs `${user_config.*}`.

<Note>
  `headersHelper` exécute des commandes shell arbitraires. Lorsqu'il est défini à portée de projet ou locale, il ne s'exécute qu'après que vous ayez accepté la boîte de dialogue de confiance de l'espace de travail.
</Note>

<h2 id="add-mcp-servers-from-json-configuration">
  Ajouter des serveurs MCP à partir de la configuration JSON
</h2>

Si vous avez une configuration JSON pour un serveur MCP, vous pouvez l'ajouter directement :

<Steps>
  <Step title="Ajouter un serveur MCP à partir de JSON">
    ```bash theme={null}
    # Syntaxe de base
    claude mcp add-json <name> '<json>'

    # Exemple : Ajouter un serveur HTTP avec configuration JSON
    claude mcp add-json weather-api '{"type":"http","url":"https://api.weather.com/mcp","headers":{"Authorization":"Bearer token"}}'

    # Exemple : Ajouter un serveur stdio avec configuration JSON
    claude mcp add-json local-weather '{"type":"stdio","command":"/path/to/weather-cli","args":["--api-key","abc123"],"env":{"CACHE_DIR":"/tmp"}}'

    # Exemple : Ajouter un serveur HTTP avec identifiants OAuth pré-configurés
    claude mcp add-json my-server '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"clientId":"your-client-id","callbackPort":8080}}' --client-secret
    ```
  </Step>

  <Step title="Vérifier que le serveur a été ajouté">
    ```bash theme={null}
    claude mcp get weather-api
    ```
  </Step>
</Steps>

<Tip>
  Conseils :

  * Assurez-vous que le JSON est correctement échappé dans votre shell
  * Le JSON doit se conformer au schéma de configuration du serveur MCP
  * Vous pouvez utiliser `--scope user` pour ajouter le serveur à votre configuration utilisateur au lieu de celle spécifique au projet
</Tip>

<h2 id="import-mcp-servers-from-claude-desktop">
  Importer les serveurs MCP à partir de Claude Desktop
</h2>

Si vous avez déjà configuré des serveurs MCP dans Claude Desktop, vous pouvez les importer :

<Steps>
  <Step title="Importer les serveurs à partir de Claude Desktop">
    ```bash theme={null}
    # Syntaxe de base 
    claude mcp add-from-claude-desktop 
    ```
  </Step>

  <Step title="Sélectionner les serveurs à importer">
    Après avoir exécuté la commande, vous verrez une boîte de dialogue interactive qui vous permet de sélectionner les serveurs que vous souhaitez importer.
  </Step>

  <Step title="Vérifier que les serveurs ont été importés">
    ```bash theme={null}
    claude mcp list 
    ```
  </Step>
</Steps>

Les noms de serveurs ajoutés via les commandes `claude mcp` ne peuvent contenir que des lettres, des chiffres, des tirets et des traits de soulignement. Claude Desktop n'applique pas cette restriction, donc un serveur Claude Desktop dont le nom contient un autre caractère, comme un espace, ne peut pas être importé. L'importation signale chaque nom qu'elle rejette et importe toujours les autres serveurs que vous avez sélectionnés. Avant la v2.1.205, le premier nom invalide arrêtait l'importation et aucun des serveurs sélectionnés n'était ajouté.

<Tip>
  Conseils :

  * Cette fonctionnalité ne fonctionne que sur macOS et Windows Subsystem for Linux (WSL)
  * Elle lit le fichier de configuration de Claude Desktop à partir de son emplacement standard sur ces plates-formes
  * Utilisez le drapeau `--scope user` pour ajouter les serveurs à votre configuration utilisateur
  * Les serveurs importés conservent les mêmes noms que dans Claude Desktop lorsque le nom ne contient que des lettres, des chiffres, des tirets et des traits de soulignement. Claude Code signale un serveur dont le nom contient un autre caractère et l'ignore
  * Si des serveurs portant les mêmes noms existent déjà, ils recevront un suffixe numérique (par exemple, `server_1`)
</Tip>

<h2 id="use-mcp-servers-from-claude-ai">
  Utiliser les serveurs MCP à partir de claude.ai
</h2>

Si vous vous êtes connecté à Claude Code avec un compte [claude.ai](https://claude.ai), les serveurs MCP que vous avez ajoutés dans claude.ai, connus sous le nom de [connecteurs](https://claude.com/docs/connectors), sont automatiquement disponibles dans Claude Code :

<Steps>
  <Step title="Configurer les serveurs MCP dans claude.ai">
    Ajoutez les serveurs à [claude.ai/customize/connectors](https://claude.ai/customize/connectors). Sur les plans Team et Enterprise, seuls les administrateurs peuvent ajouter des serveurs.
  </Step>

  <Step title="Authentifier le serveur MCP">
    Complétez les étapes d'authentification requises dans claude.ai.
  </Step>

  <Step title="Afficher et gérer les serveurs dans Claude Code">
    Dans Claude Code, utilisez la commande :

    ```text theme={null}
    /mcp
    ```

    Les serveurs de claude.ai apparaissent dans la liste avec des indicateurs montrant qu'ils proviennent de claude.ai.
  </Step>
</Steps>

À partir de la v2.1.161, les connecteurs auxquels vous ne vous êtes jamais connecté sont réduits derrière une ligne `Show unused connectors` à la fin de la section claude.ai, de sorte qu'une liste fournie par l'organisation ne remplit pas le panneau. Sélectionnez la ligne pour les développer. Un connecteur auquel vous vous êtes connecté précédemment reste visible même s'il nécessite actuellement une réauthentification.

Les connecteurs de claude.ai sont récupérés uniquement lorsque votre [méthode d'authentification](/docs/fr/authentication#authentication-precedence) active est votre abonnement claude.ai. Ils ne sont pas chargés lorsque `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, `apiKeyHelper`, ou un fournisseur tiers tel que Amazon Bedrock ou Google Cloud's Agent Platform est actif, même si vous avez précédemment exécuté `/login`.

Si `/mcp` ne répertorie pas un connecteur que vous avez ajouté, exécutez `/status` pour confirmer quelle méthode d'authentification est active, désactivez cette variable d'environnement ou supprimez le paramètre `apiKeyHelper`, puis exécutez `/login` pour sélectionner votre compte claude.ai.

Un serveur que vous avez ajouté dans Claude Code prend [précédence](#scope-hierarchy-and-precedence) sur un connecteur claude.ai qui pointe vers la même URL. Quand cela se produit, `/mcp` répertorie le connecteur comme masqué et montre comment supprimer le doublon si vous préférez utiliser le connecteur.

Certains connecteurs hébergés par Anthropic, tels que Microsoft 365, Gmail et Google Calendar, ne supportent pas OAuth local à partir de Claude Code car le fournisseur d'identité en amont n'accepte que l'URL de redirection que claude.ai a enregistrée. À partir de la v2.1.162, l'authentification de l'un de ces hôtes dans `/mcp` affiche un message vous dirigeant pour le connecter à Paramètres → Connecteurs sur claude.ai à la place. Une fois connecté là-bas, le connecteur apparaît dans Claude Code automatiquement.

<h3 id="organization-controls-on-connector-tools">
  Contrôles organisationnels sur les outils de connecteur
</h3>

Votre organisation peut définir des contrôles par outil sur les [connecteurs claude.ai](https://claude.com/docs/connectors). Claude Code lit ces paramètres au démarrage et les applique localement. Exécutez `/mcp` pour voir quel paramètre s'applique à chaque outil sur un connecteur.

* **Outil défini sur `ask`** : Claude Code vous demande à chaque appel avec la raison `Your organization requires approval for this tool`. L'invite apparaît même dans les modes de permission `acceptEdits`, `auto` et `bypassPermissions` [modes de permission](/docs/fr/permissions#permission-modes), et n'offre jamais une option pour mémoriser votre choix. Les [règles d'autorisation](/docs/fr/permissions) qui correspondent à l'outil ne contournent pas non plus l'invite. En mode `dontAsk`, qui ne demande jamais, Claude Code refuse l'appel à la place.
* **Outil défini sur `blocked`** : Claude Code filtre l'outil avant que Claude ne le voie, donc il n'apparaît jamais dans la liste des outils.

L'application de ces contrôles nécessite Claude Code v2.1.129 ou ultérieur. Les versions antérieures ignorent les paramètres et appliquent le flux de permission standard.

<h3 id="disable-claude-ai-connectors">
  Désactiver les connecteurs claude.ai
</h3>

Pour désactiver les serveurs MCP de claude.ai dans Claude Code, définissez [`disableClaudeAiConnectors`](/docs/fr/settings#available-settings) sur `true` dans n'importe quelle portée de paramètres :

```json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

Ce paramètre utilise la sémantique any-source-true : `true` dans n'importe quelle source de paramètres prend précédence. Un fichier `.claude/settings.json` de projet enregistré peut exclure un référentiel des connecteurs cloud, mais un `false` au niveau du projet ne peut pas réactiver les connecteurs qu'un `true` au niveau utilisateur ou politique a désactivés. Les serveurs transmis explicitement via `--mcp-config` ne sont pas affectés.

Vous pouvez également définir la variable d'environnement `ENABLE_CLAUDEAI_MCP_SERVERS` sur `false`, qui a le même effet pour la session shell actuelle :

```bash theme={null}
ENABLE_CLAUDEAI_MCP_SERVERS=false claude
```

Pour bloquer les connecteurs claude.ai individuels au lieu de tous les bloquer, ajoutez-les à [`deniedMcpServers`](/docs/fr/managed-mcp) par nom ou par modèle d'URL. Par exemple, une entrée `serverName` de `« claude.ai Slack »` bloque le connecteur Slack. Pour activer ou désactiver un connecteur pour le projet actuel uniquement, utilisez le panneau `/mcp`.

<Note>
  Ces paramètres côté client régissent les sessions Claude Code locales. Dans les sessions [Claude Code sur le web](/docs/fr/claude-code-on-the-web), les connecteurs claude.ai sont provisionnés par l'hôte distant et arrivent en tant qu'entrées `--mcp-config` explicites, donc `disableClaudeAiConnectors` ne s'applique pas là. Les URL des connecteurs sont également réécrites via le proxy de session, donc un modèle `serverUrl` de `deniedMcpServers` ciblant l'URL du fournisseur ne correspondra pas. Gérez les connecteurs qu'une session cloud peut utiliser à partir de vos paramètres d'organisation claude.ai.
</Note>

<h2 id="use-claude-code-as-an-mcp-server">
  Utiliser Claude Code comme serveur MCP
</h2>

Vous pouvez utiliser Claude Code lui-même comme serveur MCP auquel d'autres applications peuvent se connecter :

```bash theme={null}
# Démarrer Claude en tant que serveur MCP stdio
claude mcp serve
```

Vous pouvez l'utiliser dans Claude Desktop en ajoutant cette configuration à claude\_desktop\_config.json :

```json theme={null}
{
  "mcpServers": {
    "claude-code": {
      "type": "stdio",
      "command": "claude",
      "args": ["mcp", "serve"],
      "env": {}
    }
  }
}
```

<Warning>
  **Configuration du chemin de l'exécutable** : Le champ `command` doit référencer l'exécutable Claude Code. Si la commande `claude` n'est pas dans le PATH de votre système, vous devrez spécifier le chemin complet de l'exécutable.

  Pour trouver le chemin complet :

  ```bash theme={null}
  which claude
  ```

  Ensuite, utilisez le chemin complet dans votre configuration :

  ```json theme={null}
  {
    "mcpServers": {
      "claude-code": {
        "type": "stdio",
        "command": "/full/path/to/claude",
        "args": ["mcp", "serve"],
        "env": {}
      }
    }
  }
  ```

  Sans le chemin d'exécutable correct, vous rencontrerez des erreurs comme `spawn claude ENOENT`.
</Warning>

<Tip>
  Conseils :

  * Le serveur fournit l'accès aux outils de Claude comme View, Edit, LS, etc.
  * Dans Claude Desktop, essayez de demander à Claude de lire les fichiers dans un répertoire, de faire des modifications, et plus encore.
  * Ce serveur MCP expose uniquement les outils de Claude Code à votre client MCP, donc votre propre client est responsable de l'implémentation de la confirmation de l'utilisateur pour les appels d'outils individuels.
</Tip>

<h2 id="mcp-output-limits-and-warnings">
  Limites de sortie MCP et avertissements
</h2>

Lorsque les outils MCP produisent de grandes sorties, Claude Code aide à gérer l'utilisation des jetons pour éviter de surcharger votre contexte de conversation :

* **Seuil d'avertissement de sortie** : Claude Code affiche un avertissement lorsque la sortie de tout outil MCP dépasse 10 000 jetons
* **Limite configurable** : vous pouvez ajuster le nombre maximum de jetons de sortie MCP autorisés en utilisant la variable d'environnement `MAX_MCP_OUTPUT_TOKENS`
* **Limite par défaut** : la limite maximale par défaut est de 25 000 jetons
* **Portée** : la variable d'environnement s'applique aux outils qui ne déclarent pas leur propre limite. Les outils qui définissent [`anthropic/maxResultSizeChars`](#raise-the-limit-for-a-specific-tool) utilisent cette valeur à la place pour le contenu texte, indépendamment de ce que `MAX_MCP_OUTPUT_TOKENS` est défini. Les outils qui retournent des données d'image sont toujours soumis à `MAX_MCP_OUTPUT_TOKENS`

Pour augmenter la limite pour les outils qui produisent de grandes sorties :

```bash theme={null}
export MAX_MCP_OUTPUT_TOKENS=50000
claude
```

Ceci est particulièrement utile lorsque vous travaillez avec des serveurs MCP qui :

* Interrogent de grands ensembles de données ou des bases de données
* Génèrent des rapports ou des documentations détaillés
* Traitent des fichiers journaux ou des informations de débogage étendus

<h3 id="raise-the-limit-for-a-specific-tool">
  Augmenter la limite pour un outil spécifique
</h3>

Si vous créez un serveur MCP, vous pouvez permettre aux outils individuels de retourner des résultats plus grands que le seuil par défaut de persistance sur disque en définissant `_meta["anthropic/maxResultSizeChars"]` dans l'entrée de l'outil dans la réponse `tools/list`. Claude Code augmente le seuil de cet outil à la valeur annotée, jusqu'à un plafond dur de 500 000 caractères.

Ceci est utile pour les outils qui retournent des sorties intrinsèquement grandes mais nécessaires, telles que les schémas de base de données ou les arbres de fichiers complets. Sans l'annotation, les résultats qui dépassent le seuil par défaut sont persistés sur disque et remplacés par une référence de fichier dans la conversation.

```json theme={null}
{
  "name": "get_schema",
  "description": "Returns the full database schema",
  "_meta": {
    "anthropic/maxResultSizeChars": 200000
  }
}
```

L'annotation s'applique indépendamment de `MAX_MCP_OUTPUT_TOKENS` pour le contenu texte, donc les utilisateurs n'ont pas besoin d'augmenter la variable d'environnement pour les outils qui la déclarent. Les outils qui retournent des données d'image sont toujours soumis à la limite de jetons.

<Warning>
  Si vous rencontrez fréquemment des avertissements de sortie avec des serveurs MCP spécifiques que vous ne contrôlez pas, envisagez d'augmenter la limite `MAX_MCP_OUTPUT_TOKENS`. Vous pouvez également demander à l'auteur du serveur d'ajouter l'annotation `anthropic/maxResultSizeChars` ou de paginer ses réponses. L'annotation n'a aucun effet sur les outils qui retournent du contenu d'image ; pour ceux-ci, augmenter `MAX_MCP_OUTPUT_TOKENS` est la seule option.
</Warning>

<h2 id="tool-input-schemas-with-a-root-level-combinator">
  Schémas d'entrée d'outil avec un combinateur au niveau racine
</h2>

Certains serveurs MCP déclarent le schéma d'entrée d'un outil comme une union JSON Schema, avec `anyOf`, `oneOf`, ou `allOf` au niveau supérieur du schéma. L'API Claude n'accepte pas ces mots-clés à la racine du schéma. Elle accepte les combinateurs imbriqués dans `properties`, que Claude Code envoie sans modification.

À partir de Claude Code v2.1.195, les outils avec un combinateur au niveau racine restent disponibles. Avant d'envoyer l'outil à l'API, Claude Code aplatit le schéma en un seul objet et ajoute une phrase à la description de l'outil qui indique à Claude quels groupes de paramètres vont ensemble :

* `allOf` : les propriétés de chaque branche sont fusionnées, et la liste `required` de chaque branche s'applique toujours
* `anyOf` et `oneOf` : les propriétés de chaque branche sont fusionnées, et la liste `required` de chaque branche est décrite dans la description de l'outil au lieu d'être appliquée par le schéma

Votre serveur reçoit les arguments que Claude a choisis, donc continuez à valider la combinaison côté serveur.

Lorsque Claude Code ne peut pas produire un schéma que l'API accepte, ou sur un déploiement qui ne reçoit pas la configuration distante qui active la réécriture, comme une machine hors ligne, il saute cet outil, enregistre la raison dans le journal du serveur, et laisse les autres outils du serveur disponibles. Les versions antérieures à v2.1.195 sautent chaque outil dont le schéma d'entrée a un `anyOf`, `oneOf`, ou `allOf` au niveau racine.

<h2 id="require-approval-for-a-specific-tool">
  Exiger l'approbation pour un outil spécifique
</h2>

Si vous créez un serveur MCP, vous pouvez marquer un outil comme nécessitant une approbation explicite à chaque appel en définissant `_meta["anthropic/requiresUserInteraction"]` sur `true` dans l'entrée de l'outil dans la réponse `tools/list`. La valeur doit être le booléen JSON `true` ; toute autre valeur est ignorée.

Claude Code affiche l'invite de permission de cet outil à chaque appel, même en modes `acceptEdits`, `auto`, et `bypassPermissions` [permission modes](/docs/fr/permissions#permission-modes), et n'offre pas d'option « ne pas demander à nouveau » pour celui-ci. Les [règles d'autorisation](/docs/fr/permissions#permission-rule-syntax) qui correspondent à l'outil ne sautent pas non plus l'invite. En mode `dontAsk`, qui ne demande jamais, Claude Code refuse l'appel à la place.

L'invite doit atteindre une personne. En mode non interactif avec [`--permission-prompt-tool`](/docs/fr/cli-reference#cli-flags), un résultat `allow` de l'outil d'invite de permission pour un outil signalé est converti en refus avec le message `MCP tool requires user interaction; not supported via --permission-prompt-tool`. Le callback [`canUseTool`](/docs/fr/agent-sdk/permissions) du SDK Agent reçoit ces appels et peut les approuver, car l'hôte du SDK est censé les montrer à un utilisateur.

Utilisez ceci pour les outils dont l'invite de permission est elle-même le point, comme une étape de consentement ou d'octroi d'accès où l'approbation automatique signifierait qu'aucun humain n'a jamais accepté. Les autres outils du même serveur conservent leur comportement de permission normal.

L'entrée `tools/list` suivante marque un outil comme nécessitant toujours une approbation.

```json theme={null}
{
  "name": "grant_access",
  "description": "Requests access to a protected resource",
  "_meta": {
    "anthropic/requiresUserInteraction": true
  }
}
```

L'annotation `anthropic/requiresUserInteraction` nécessite Claude Code v2.1.199 ou ultérieur. Les versions antérieures l'ignorent et appliquent le flux de permission standard.

Lorsqu'une session est connectée à [Remote Control](/docs/fr/remote-control) ou à un hôte SDK, Claude Code marque la demande de permission comme nécessitant une interaction utilisateur, de sorte que le client affiche l'invite de permission de l'outil pour que vous répondiez au lieu d'une action d'approbation en un clic.

<h2 id="respond-to-mcp-elicitation-requests">
  Répondre aux demandes d'élicitation MCP
</h2>

Les serveurs MCP peuvent demander une entrée structurée de votre part au cours d'une tâche en utilisant l'élicitation. Lorsqu'un serveur a besoin d'informations qu'il ne peut pas obtenir par lui-même, Claude Code affiche une boîte de dialogue interactive et transmet votre réponse au serveur. Aucune configuration n'est requise de votre côté : les boîtes de dialogue d'élicitation apparaissent automatiquement lorsqu'un serveur les demande.

Les serveurs peuvent demander une entrée de deux façons :

* **Mode formulaire** : Claude Code affiche une boîte de dialogue avec des champs de formulaire définis par le serveur (par exemple, une invite de nom d'utilisateur et de mot de passe). Remplissez les champs et soumettez.
* **Mode URL** : Claude Code ouvre une URL de navigateur pour l'authentification ou l'approbation. Complétez le flux dans le navigateur, puis confirmez dans l'interface de ligne de commande.

Pour répondre automatiquement aux demandes d'élicitation sans afficher de boîte de dialogue, utilisez le [hook `Elicitation`](/docs/fr/hooks#elicitation).

Si vous créez un serveur MCP qui utilise l'élicitation, consultez la [spécification d'élicitation MCP](https://modelcontextprotocol.io/docs/learn/client-concepts#elicitation) pour les détails du protocole et les exemples de schéma.

<h2 id="use-mcp-resources">
  Utiliser les ressources MCP
</h2>

Les serveurs MCP peuvent exposer des ressources que vous pouvez référencer en utilisant des mentions @, similaire à la façon dont vous référencez les fichiers.

<h3 id="reference-mcp-resources">
  Référencer les ressources MCP
</h3>

<Steps>
  <Step title="Lister les ressources disponibles">
    Tapez `@` dans votre prompt pour voir les ressources disponibles de tous les serveurs MCP connectés. Les ressources apparaissent aux côtés des fichiers dans le menu d'autocomplétion.
  </Step>

  <Step title="Référencer une ressource spécifique">
    Utilisez le format `@server:protocol://resource/path` pour référencer une ressource :

    ```text theme={null}
    Pouvez-vous analyser @github:issue://123 et suggérer un correctif ?
    ```

    ```text theme={null}
    Veuillez examiner la documentation API à @docs:file://api/authentication
    ```
  </Step>

  <Step title="Références de ressources multiples">
    Vous pouvez référencer plusieurs ressources dans un seul prompt :

    ```text theme={null}
    Comparez @postgres:schema://users avec @docs:file://database/user-model
    ```
  </Step>
</Steps>

<Tip>
  Conseils :

  * Les ressources sont automatiquement récupérées et incluses en tant que pièces jointes lorsqu'elles sont référencées
  * Les chemins des ressources sont recherchables par correspondance floue dans l'autocomplétion de mention @
  * Claude Code fournit automatiquement des outils pour lister et lire les ressources MCP lorsque les serveurs les supportent
  * Les ressources peuvent contenir n'importe quel type de contenu fourni par le serveur MCP (texte, JSON, données structurées, etc.)
</Tip>

<h2 id="scale-with-mcp-tool-search">
  Mettre à l'échelle avec la recherche d'outils MCP
</h2>

La recherche d'outils maintient l'utilisation du contexte MCP faible en différant les définitions d'outils jusqu'à ce que Claude en ait besoin. Seuls les noms d'outils et les instructions du serveur se chargent au démarrage de la session, donc l'ajout de plus de serveurs MCP a un impact minimal sur votre fenêtre de contexte. Claude Code n'impose pas de limite fixe d'outils par serveur ; la limite pratique est votre budget de fenêtre de contexte.

<h3 id="how-it-works">
  Comment cela fonctionne
</h3>

La recherche d'outils est activée par défaut. Les outils MCP sont différés plutôt que chargés dans le contexte à l'avance, et Claude utilise un outil de recherche pour découvrir les outils pertinents lorsqu'une tâche en a besoin. Seuls les outils que Claude utilise réellement entrent dans le contexte. De votre point de vue, les outils MCP fonctionnent exactement comme avant.

Si vous préférez le chargement basé sur un seuil, définissez `ENABLE_TOOL_SEARCH=auto` pour charger les schémas à l'avance lorsqu'ils s'ajustent dans 10 % de la fenêtre de contexte et différer uniquement le débordement. Consultez [Configurer la recherche d'outils](#configure-tool-search) pour toutes les options.

<h3 id="for-mcp-server-authors">
  Pour les auteurs de serveurs MCP
</h3>

Si vous créez un serveur MCP, le champ des instructions du serveur devient plus utile avec la recherche d'outils activée. Les instructions du serveur aident Claude à comprendre quand rechercher vos outils, similaire à la façon dont les [skills](/docs/fr/skills) fonctionnent.

Ajoutez des instructions de serveur claires et descriptives qui expliquent :

* Quelle catégorie de tâches vos outils gèrent
* Quand Claude doit rechercher vos outils
* Les capacités clés de votre serveur

Claude Code tronque les descriptions d'outils et les instructions du serveur à 2 Ko chacune. Gardez-les concis pour éviter la troncature, et mettez les détails critiques près du début.

<h3 id="configure-tool-search">
  Configurer la recherche d'outils
</h3>

La recherche d'outils est activée par défaut : les outils MCP sont différés et découverts à la demande. Claude Code la désactive par défaut sur la plateforme Agent de Google Cloud. Elle est également désactivée lorsque `ANTHROPIC_BASE_URL` pointe vers un hôte non-propriétaire, car la plupart des proxies ne transfèrent pas les blocs `tool_reference`. Définissez `ENABLE_TOOL_SEARCH` explicitement pour remplacer l'un ou l'autre des mécanismes de secours.

La définition de [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/fr/env-vars) maintient la recherche d'outils désactivée, et `ENABLE_TOOL_SEARCH` ne peut pas la remplacer. La variable supprime l'en-tête bêta que les définitions d'outils `defer_loading` et les blocs de contenu `tool_reference` nécessitent.

La recherche d'outils nécessite un modèle qui supporte les blocs `tool_reference` : Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5, et les modèles ultérieurs. Consultez [la compatibilité des modèles dans la documentation de l'API](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-search-tool#model-compatibility) pour la liste actuelle. Sur la plateforme Agent de Google Cloud, la recherche d'outils est supportée pour Claude Sonnet 4.5 et ultérieur et Claude Opus 4.5 et ultérieur.

Contrôlez le comportement de la recherche d'outils avec la variable d'environnement `ENABLE_TOOL_SEARCH` :

| Valeur       | Comportement                                                                                                                                                                                                                                                                                                           |
| :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| (non défini) | Tous les outils MCP différés et chargés à la demande. Revient au chargement à l'avance sur la plateforme Agent de Google Cloud ou lorsque `ANTHROPIC_BASE_URL` est un hôte non-propriétaire                                                                                                                            |
| `true`       | Tous les outils MCP différés. Claude Code envoie l'en-tête bêta même sur la plateforme Agent de Google Cloud et via les proxies. Les demandes échouent sur les modèles de la plateforme Agent de Google Cloud antérieurs à Sonnet 4.5 ou Opus 4.5, ou sur les proxies qui ne supportent pas les blocs `tool_reference` |
| `auto`       | Mode seuil : les outils se chargent à l'avance s'ils s'ajustent dans 10 % de la fenêtre de contexte, différés sinon                                                                                                                                                                                                    |
| `auto:N`     | Mode seuil avec un pourcentage personnalisé, où `N` est 0-100. Par exemple, `auto:5` pour 5 %                                                                                                                                                                                                                          |
| `false`      | Tous les outils MCP chargés à l'avance, pas de différé                                                                                                                                                                                                                                                                 |

```bash theme={null}
# Utiliser un seuil personnalisé de 5 %
ENABLE_TOOL_SEARCH=auto:5 claude

# Désactiver complètement la recherche d'outils
ENABLE_TOOL_SEARCH=false claude
```

Ou définissez la valeur dans le champ `env` de votre [settings.json](/docs/fr/settings#available-settings).

Vous pouvez également désactiver l'outil `ToolSearch` spécifiquement :

```json theme={null}
{
  "permissions": {
    "deny": ["ToolSearch"]
  }
}
```

<h3 id="exempt-a-server-from-deferral">
  Exempter un serveur du différé
</h3>

Si les outils d'un serveur doivent toujours être visibles pour Claude sans une étape de recherche, définissez `alwaysLoad` à `true` dans la configuration de ce serveur. Chaque outil de ce serveur se charge alors dans le contexte au démarrage de la session indépendamment du paramètre `ENABLE_TOOL_SEARCH`. Utilisez ceci pour un petit nombre d'outils que Claude doit utiliser à chaque tour, car chaque outil à l'avance consomme du contexte qui serait autrement disponible pour votre conversation.

L'entrée `.mcp.json` suivante exempte un serveur HTTP tout en laissant les autres serveurs différés :

```json theme={null}
{
  "mcpServers": {
    "core-tools": {
      "type": "http",
      "url": "https://mcp.example.com/mcp",
      "alwaysLoad": true
    }
  }
}
```

Le champ `alwaysLoad` est disponible sur tous les types de serveurs et nécessite Claude Code v2.1.121 ou ultérieur. Un serveur MCP peut également marquer les outils individuels comme toujours chargés en incluant `"anthropic/alwaysLoad": true` dans l'objet `_meta` de l'outil, ce qui a le même effet pour cet outil uniquement.

La définition de `alwaysLoad: true` bloque également le démarrage jusqu'à ce que le serveur se connecte, limité au délai d'expiration de connexion standard de 5 secondes. Cela s'applique même lorsque MCP startup est autrement [non-bloquant par défaut](/docs/fr/env-vars), car les outils doivent être présents lors de la construction de la première invite. Les autres serveurs continuent à se connecter en arrière-plan.

<h2 id="use-mcp-prompts-as-commands">
  Utiliser les prompts MCP comme commandes
</h2>

Les serveurs MCP peuvent exposer des prompts qui deviennent disponibles en tant que commandes dans Claude Code.

<h3 id="execute-mcp-prompts">
  Exécuter les prompts MCP
</h3>

<Steps>
  <Step title="Découvrir les prompts disponibles">
    Tapez `/` pour voir toutes les commandes disponibles, y compris celles des serveurs MCP. Les prompts MCP apparaissent au format `/mcp__servername__promptname`.
  </Step>

  <Step title="Exécuter un prompt sans arguments">
    ```text theme={null}
    /mcp__github__list_prs
    ```
  </Step>

  <Step title="Exécuter un prompt avec des arguments">
    De nombreux prompts acceptent des arguments. Passez-les séparés par des espaces après la commande :

    ```text theme={null}
    /mcp__github__pr_review 456
    ```

    ```text theme={null}
    /mcp__jira__create_issue "Bug in login flow" high
    ```
  </Step>
</Steps>

<Tip>
  Conseils :

  * Les prompts MCP sont découverts dynamiquement à partir des serveurs connectés
  * Les arguments sont analysés en fonction des paramètres définis du prompt
  * Les résultats du prompt sont injectés directement dans la conversation
  * Les noms de serveur et de prompt sont normalisés, avec les espaces convertis en traits de soulignement
</Tip>

<h2 id="managed-mcp-configuration">
  Configuration MCP gérée
</h2>

Pour les organisations qui ont besoin d'un contrôle centralisé sur les serveurs MCP auxquels les utilisateurs peuvent se connecter, consultez [Configuration MCP gérée](/docs/fr/managed-mcp). Elle couvre le déploiement d'un ensemble fixe de serveurs avec `managed-mcp.json`, la restriction des serveurs avec `allowedMcpServers` et `deniedMcpServers`, et ce que les utilisateurs voient lorsqu'un serveur est bloqué.
