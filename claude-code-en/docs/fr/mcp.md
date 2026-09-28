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

    Si l'installation échoue, faites correspondre le message que Claude Code signale :

    * `Marketplace "claude-plugins-official" not found` : ajoutez le marketplace avec `/plugin marketplace add anthropics/claude-plugins-official`, puis réessayez l'installation.
    * Le plugin est [introuvable dans le marketplace](/docs/fr/plugins/install#install-a-plugin) : vérifiez le nom du plugin.

    Si le résumé de l'installation signale `Run /reload-plugins to activate.`, Claude Code exécute ensuite ce rechargement pour vous. Si le rechargement vous avertit que votre prochain message relierait la conversation, exécutez `/reload-plugins --force`.
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

Les serveurs HTTP sont l'option recommandée pour se connecter à des serveurs MCP distants. C'est le transport le plus largement supporté pour les services basés sur le cloud.

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

Dans les exécutions `--output-format stream-json`, Claude Code signale également une entrée `--mcp-config` ignorée dans le champ [`mcp_server_errors`](/docs/fr/headless#stream-responses) de l'événement `system/init`, afin que les scripts puissent détecter que le serveur n'a jamais été chargé. Cela nécessite Claude Code v2.1.219 ou ultérieur.

<h3 id="option-2-add-a-remote-sse-server">
  Option 2 : Ajouter un serveur SSE distant
</h3>

<Warning>
  Le transport SSE (Server-Sent Events) est déprécié. Utilisez plutôt des serveurs HTTP, si disponibles.
</Warning>

Certains services exposent toujours uniquement un point de terminaison SSE. Ajoutez-les avec la même commande `claude mcp add --transport http <name> <url>` que [un serveur HTTP](#option-1-add-a-remote-http-server). Claude Code essaie d'abord le transport HTTP et bascule vers SSE lorsque le serveur ne l'accepte pas. Le basculement automatique nécessite Claude Code v2.1.265 ou ultérieur.

Sur une version antérieure, ou pour vous connecter directement via SSE, passez plutôt `--transport sse` :

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

Les serveurs stdio s'exécutent en tant que processus locaux sur votre machine. Ils sont idéaux pour les outils qui ont besoin d'un accès direct au système ou de scripts personnalisés.

Claude Code définit `CLAUDE_PROJECT_DIR` dans l'environnement du serveur généré pour la racine du projet, afin que votre serveur puisse résoudre les chemins relatifs au projet sans dépendre du répertoire de travail. C'est le même répertoire que les hooks reçoivent dans leur variable `CLAUDE_PROJECT_DIR`. Lisez-le depuis l'intérieur de votre processus serveur, par exemple `process.env.CLAUDE_PROJECT_DIR` en Node ou `os.environ["CLAUDE_PROJECT_DIR"]` en Python.

`CLAUDE_PROJECT_DIR` est la racine de projet stable et ne change pas lorsque vous ajoutez ou supprimez des répertoires de travail en cours de session. Un serveur qui limite son propre accès au système de fichiers à un ensemble de répertoires autorisés devrait implémenter la demande MCP `roots/list` à la place. Claude Code répond à `roots/list` avec le répertoire de lancement de la session plus chaque [répertoire de travail supplémentaire](/docs/fr/permissions#working-directories) que vous avez accordé avec `--add-dir`, `/add-dir`, ou le paramètre `additionalDirectories`. Claude Code envoie `notifications/roots/list_changed` lorsque cet ensemble change. Avant la v2.1.203, `roots/list` retournait uniquement le répertoire de lancement et Claude Code n'envoyait pas `notifications/roots/list_changed`.

Cette variable est définie dans l'environnement du serveur, pas dans l'environnement de Claude Code lui-même, donc la référencer via l'expansion `${VAR}` dans la `command` ou `args` d'une entrée `.mcp.json` scoped au projet ou d'une entrée serveur locale ou utilisateur dans `~/.claude.json` nécessite une valeur par défaut telle que `${CLAUDE_PROJECT_DIR:-.}`. Les configurations MCP fournies par les plugins substituent `${CLAUDE_PROJECT_DIR}` directement et n'ont pas besoin de la valeur par défaut.

```bash theme={null}
# Syntaxe de base
claude mcp add [options] <name> -- <command> [args...]

# Exemple réel : Ajouter un serveur Airtable
claude mcp add --env AIRTABLE_API_KEY=YOUR_KEY --transport stdio airtable \
  -- npx -y airtable-mcp-server
```

<Note>
  **Important : Séparez les arguments du serveur avec `--`**

  Pour les serveurs stdio, le `--` (double tiret) sépare les options propres à Claude, telles que `--transport`, `--env`, et `--scope`, de la commande et des arguments qui exécutent le serveur. Tout ce qui suit `--` est transmis au serveur sans modification.

  Par exemple :

  * `claude mcp add --transport stdio myserver -- npx server` → exécute `npx server`
  * `claude mcp add --env KEY=value --transport stdio myserver -- python server.py --port 8080` → exécute `python server.py --port 8080` avec `KEY=value` dans l'environnement

  Sans `--`, Claude Code essaierait d'analyser les drapeaux du serveur, comme `--port` ci-dessus, comme ses propres options.

  `--env` accepte plusieurs paires `KEY=value`. Si le nom du serveur vient directement après `--env`, la CLI lit le nom comme une autre paire et le rejette, donc placez au moins une autre option, telle que `--transport stdio`, entre `--env` et le nom du serveur.
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

<h3 id="add-a-server-from-setup-instructions-written-for-another-client">
  Ajouter un serveur à partir d'instructions de configuration écrites pour un autre client
</h3>

Les serveurs MCP ne sont pas spécifiques à Claude Code, donc les instructions de configuration d'un serveur peuvent être écrites pour Claude Desktop, Cursor, ou un autre client MCP et ne pas donner de commande `claude mcp add`. Pour ajouter le serveur quand même, cherchez dans ces instructions une URL, une commande de lancement, ou un bloc JSON :

* **Une URL** telle que `https://mcp.example.com/mcp` : le serveur est distant.
* **Une commande de lancement** telle que `npx -y @example/mcp-server` : le serveur s'exécute sur votre machine.
* **Un bloc JSON `mcpServers`** : configuration écrite pour le fichier de paramètres d'un autre client.

Chacun est l'une des entrées que les quatre options dans [Installation des serveurs MCP](#installing-mcp-servers) prennent. Trouvez la forme que vous avez ci-dessous pour la transformer en commande que Claude Code accepte. Chaque commande écrit dans la [portée locale](#local-scope) sauf si vous ajoutez `--scope project` ou `--scope user`.

<h4 id="from-a-url">
  À partir d'une URL
</h4>

Une URL signifie que le serveur est distant. Pour un point de terminaison `https://`, ajoutez-le avec `--transport http`, ou suivez [Option 2](#option-2-add-a-remote-sse-server) lorsque les instructions indiquent que le point de terminaison utilise SSE. Pour un point de terminaison `wss://`, utilisez plutôt [Option 4](#option-4-add-a-remote-websocket-server), car `--transport` n'accepte pas `ws` :

```bash theme={null}
claude mcp add --transport http example https://mcp.example.com/mcp
```

Si les instructions donnent également une clé API ou un en-tête de jeton, passez-le avec `--header` comme montré dans [Option 1](#option-1-add-a-remote-http-server).

<h4 id="from-an-npx-uvx-or-binary-command">
  À partir d'une commande `npx`, `uvx`, ou binaire
</h4>

Une commande de lancement signifie que le serveur s'exécute en tant que processus stdio local. Mettez la commande entière après `--`, afin que Claude Code transmette les drapeaux tels que `-y` à la commande qui démarre le serveur au lieu de les lire comme ses propres options. Passez toutes les variables d'environnement que les instructions demandent avec `--env`, après le nom du serveur et avant `--` :

```bash theme={null}
claude mcp add example --env API_KEY=your-key -- npx -y @example/mcp-server
```

[Option 3](#option-3-add-a-local-stdio-server) couvre le séparateur `--` en détail.

<h4 id="from-an-mcpservers-json-block">
  À partir d'un bloc JSON `mcpServers`
</h4>

Un bloc `mcpServers` écrit pour un autre client MCP, tel que Claude Desktop, utilise la clé wrapper et la forme d'entrée que Claude Code lit. Passez à `claude mcp add-json` l'objet à l'intérieur de `mcpServers`, pas le wrapper. Deux entrées ont besoin d'une réparation d'abord :

* **Une `url` sans `type`** : ajoutez `"type": "http"`, `"type": "sse"`, ou `"type": "ws"` pour correspondre au point de terminaison. Claude Code lit une entrée sans `type` comme un serveur stdio, donc une entrée `url` sans `type` échoue.
* **Une clé avec des caractères autres que des lettres, des chiffres, des tirets, et des traits de soulignement** : choisissez un nom de serveur qui utilise uniquement ces caractères. Sinon, la clé est le nom du serveur.

Par exemple, ce bloc :

```json theme={null}
{
  "mcpServers": {
    "example": {
      "command": "npx",
      "args": ["-y", "@example/mcp-server"]
    }
  }
}
```

devient cette commande :

```bash theme={null}
claude mcp add-json example '{"command":"npx","args":["-y","@example/mcp-server"]}'
```

[Ajouter des serveurs MCP à partir de la configuration JSON](#add-mcp-servers-from-json-configuration) couvre l'échappement du shell et le drapeau `--scope` pour `add-json`. Pour partager le serveur avec votre équipe à la place, ajoutez `--scope project`, ou ajoutez l'entrée sous `mcpServers` dans `.mcp.json` à la racine de votre projet et validez-la. [Portée du projet](#project-scope) couvre comment Claude Code charge et approuve ce fichier.

Chaque commande `claude mcp add` et `claude mcp add-json` imprime une ligne `Added ...`. Pour vérifier que Claude Code s'est connecté, exécutez `claude mcp get <name>` ; [État du serveur](#server-status) couvre les statuts qu'il affiche et l'étape d'approbation pour les serveurs `.mcp.json`.

<h3 id="managing-your-servers">
  Gestion de vos serveurs
</h3>

Une fois configurés, vous pouvez gérer vos serveurs MCP avec ces commandes :

```bash theme={null}
# Lister tous les serveurs configurés
claude mcp list

# Obtenir les détails d'un serveur spécifique
claude mcp get notion

# Supprimer un serveur
claude mcp remove notion

# (dans Claude Code) Vérifier l'état du serveur
/mcp
```

Lorsque vous supprimez un serveur distant, Claude Code supprime également les jetons OAuth et l'enregistrement du client qu'il a stockés pour ce serveur.

<h4 id="server-status">
  État du serveur
</h4>

`claude mcp add` confirme un ajout réussi en imprimant une ligne `Added ...`, ce qui signifie que la configuration a été écrite. `claude mcp list` affiche ensuite un statut de santé à côté de chaque serveur qu'il liste, tel que `✔ Connected`, `! Needs authentication`, ou `✘ Failed to connect`. Un statut d'échec signifie que Claude Code n'a pas pu se connecter à ce serveur, pas que la commande list a échoué.

Les statuts dans cette liste signalent une décision de configuration plutôt qu'une tentative de connexion, donc Claude Code les imprime sans se connecter au serveur :

* ``⏸ Pending approval (run `claude` to approve)`` : un serveur scoped au projet à partir de `.mcp.json` que vous n'avez pas encore approuvé. Claude Code l'affiche à la fois dans `claude mcp list` et `claude mcp get <name>`. Exécutez `claude` de manière interactive pour l'examiner et l'approuver.
* `✘ Rejected (see disabledMcpjsonServers in settings)` : un serveur `.mcp.json` qu'une entrée [`disabledMcpjsonServers`](/docs/fr/settings-reference#disabledmcpjsonservers) rejette. Claude Code l'affiche uniquement dans `claude mcp get <name>`.
* `⊘ Disabled for this project (re-enable via /mcp)` : un serveur que la liste [`disabledMcpServers`](#disable-a-server-without-removing-it) du projet nomme. Claude Code l'affiche à la fois dans `claude mcp list` et `claude mcp get <name>`. Réactivez le serveur à partir du panneau `/mcp`. Avant la v2.1.238, les deux commandes se connectaient à un serveur désactivé pour le vérifier et signalaient le résultat de la connexion.

Les serveurs WebSocket n'apparaissent pas dans la sortie de `claude mcp list`. Utilisez `claude mcp get <name>` ou le panneau `/mcp` pour les vérifier.

<h4 id="project-server-approvals-and-workspace-trust">
  Approbations des serveurs de projet et confiance de l'espace de travail
</h4>

À partir de la v2.1.196, `claude mcp list` et `claude mcp get` lisent les approbations `.mcp.json` uniquement à partir des fichiers de paramètres qui ne sont pas validés dans le référentiel jusqu'à ce que vous fassiez confiance à l'espace de travail en exécutant `claude` et en acceptant la boîte de dialogue de confiance de l'espace de travail. Un référentiel cloné ne peut pas approuver ses propres serveurs : [`enableAllProjectMcpServers`](/docs/fr/settings-reference#enableallprojectmcpservers) ou [`enabledMcpjsonServers`](/docs/fr/settings-reference#enabledmcpjsonservers) validés dans le `.claude/settings.json` du projet est ignoré dans un dossier non approuvé, et le serveur reste à `⏸ Pending approval` au lieu d'être connecté et vérifié.

Les approbations de ces sources s'appliquent toujours dans un dossier non approuvé :

* votre `~/.claude/settings.json` utilisateur
* paramètres gérés
* paramètres passés avec `--settings`

Claude Code applique également les approbations à partir d'un `.claude/settings.local.json` non suivi, mais il exécute git pour vérifier si le fichier est suivi, et il n'exécute cette vérification que dans un [dossier approuvé](/docs/fr/permissions#project-allow-rules-and-workspace-trust). Dans un dossier que vous n'avez jamais approuvé, Claude Code attend la boîte de dialogue de confiance avant d'appliquer les approbations du fichier, sauf si le dossier est votre répertoire de configuration personnel : votre répertoire personnel, ou un répertoire dont vous avez défini le `.claude` comme [`CLAUDE_CONFIG_DIR`](/docs/fr/env-vars). Avant la v2.1.207, Claude Code appliquait les approbations à partir d'un `.claude/settings.local.json` non suivi même dans un dossier que vous n'aviez jamais approuvé.

Une entrée `disabledMcpjsonServers` dans n'importe quel fichier de paramètres rejette toujours le serveur.

<h4 id="server-status-detail">
  Détail de l'état du serveur
</h4>

Dans `/mcp`, y compris le menu d'un serveur là-bas, et dans le gestionnaire [`/plugin`](/docs/fr/plugins/install), un serveur HTTP ou SSE distant que vous avez utilisé auparavant peut afficher un statut `cached` tel que `cached 2h ago · connects on first use · 5 tools`. Claude Code a chargé la liste d'outils du serveur à partir de son cache de découverte, enregistré dans une session précédente, au lieu de se connecter au démarrage, et Claude Code connecte le serveur la première fois que Claude appelle l'un des outils du serveur. Les outils sont disponibles à partir de votre premier message, donc vous n'avez rien à faire. Le cache de découverte et son statut `cached` nécessitent Claude Code v2.1.221 ou ultérieur.

Le cache de découverte est désactivé par défaut sauf si un déploiement progressif l'a activé pour votre compte. Définissez [`MCP_DISCOVERY_CACHE=1`](/docs/fr/env-vars) pour l'activer, ou `0` pour le garder désactivé même lorsque le déploiement l'a activé. Avant la v2.1.238, le cache était activé par défaut.

Deux actions dans le menu d'un serveur dans `/mcp` affectent également l'entrée du cache de ce serveur :

* **Reconnect** : sur un serveur `cached`, Claude Code le connecte maintenant plutôt que lors de son premier appel d'outil et conserve l'entrée. Sur un serveur connecté ou défaillant, Claude Code le reconnecte et supprime également l'entrée.
* **Clear authentication** : Claude Code révoque l'authentification du serveur et supprime également l'entrée.

Après suppression de l'entrée, Claude Code récupère la liste d'outils du serveur à partir du serveur au lieu du cache.

Lorsque le statut d'un serveur est `✘ Failed to connect`, `claude mcp list` ajoute le détail de l'échec à cette ligne de statut, et `claude mcp get <name>` l'affiche sur une ligne `Issue:` : le code de statut HTTP ou le code d'erreur, plus tout texte d'erreur que le serveur a retourné. La vue détaillée du serveur dans `/mcp` inclut le même texte signalé par le serveur dans sa ligne `Issue:`. Claude Code rédige le texte ressemblant à des identifiants de ce détail et n'inclut jamais l'URL du serveur développée, qui peut contenir des secrets. Claude Code n'ajoute aucun détail à un statut `✘ Connection error`, car le texte d'exception qu'il imprimerait là peut intégrer cette URL. Avant la v2.1.219, les deux commandes affichaient uniquement le statut d'échec nu, sans le code de statut ou le texte d'erreur du serveur.

Lorsque vous terminez l'authentification à partir de `/mcp` et que la connexion échoue toujours avec un code de statut HTTP ou un code d'erreur de transport, Claude Code ajoute ce code et l'origine de l'URL du serveur au message qu'il imprime après la tentative. L'origine est le schéma et l'hôte, plus le port lorsque l'URL en nomme un, tel que `https://mcp.example.com`.

* Le chemin et la requête n'apparaissent jamais dans ce message.
* Pour un serveur dans la portée [scope](#mcp-installation-scopes) locale, de projet, ou utilisateur ou dans la configuration MCP gérée, l'origine affiche l'hôte tel qu'écrit dans cette configuration, donc une référence `${VAR}` dans l'hôte n'est pas développée dans le message.
* Pour un échec sans code de statut ou d'erreur, Claude Code affiche le texte d'erreur sans l'origine.

Un serveur distant dont la configuration a une `url` vide s'affiche comme `not configured` dans `/mcp`, dans `claude mcp list`, et dans le gestionnaire [`/plugin`](/docs/fr/plugins/install), et Claude Code ne tente pas de s'y connecter. Un plugin peut inclure une entrée d'espace réservé comme celle-ci pour un connecteur que vous configurez plus tard, afin que Claude Code ne le signale pas comme une erreur ou un problème de configuration. La vue détaillée du serveur dans `/mcp` lit `No URL configured for this server` ; définissez la `url` de l'entrée pour la connecter. Avant la v2.1.208, Claude Code signalait une `url` vide comme un problème de configuration avec une invite de reconnexion.

<h4 id="configuration-warnings">
  Avertissements de configuration
</h4>

Claude Code avertit des problèmes de configuration ci-dessous. Chaque entrée indique ce que Claude Code vérifie et comment effacer l'avertissement :

* **Espaces blancs cachés** : Claude Code avertit lorsqu'une valeur de configuration MCP porte des espaces blancs cachés en début ou en fin, ce qui provient souvent du collage d'un jeton avec une nouvelle ligne de fin. Claude Code vérifie `command`, `url`, chaque entrée `args`, et les valeurs et noms de clés sous `env` et `headers`. Claude Code affiche l'avertissement dans la sortie de `claude mcp list` et dans `/mcp`, en nommant les champs affectés sans répéter leurs valeurs, par exemple `Leading or trailing whitespace in: headers.Authorization`. Claude Code ne supprime pas les espaces blancs et utilise les valeurs exactement telles qu'écrites, donc modifiez la configuration pour les supprimer.
* **Même nom dans plus d'une portée** : si vous définissez le même nom de serveur dans plus d'une [scope](#mcp-installation-scopes) avec des points de terminaison différents, Claude Code avertit du conflit dans la sortie de `claude mcp list` et dans `/mcp`. Claude Code stocke les connexions OAuth par point de terminaison, donc lorsque vous authentifiez la définition qui se charge dans un projet, vous devez toujours vous connecter séparément dans un projet où une définition différente se charge. Conservez le point de terminaison que vous voulez et supprimez les autres avec `claude mcp remove <name> --scope <scope>`. Dans l'avertissement, Claude Code cite le point de terminaison de chaque portée tel qu'écrit dans votre configuration, avec les références [`${VAR}`](#environment-variable-expansion-in-mcp-json) non développées, donc il n'affiche jamais une valeur résolue telle qu'une clé API.
* **Noms réservés** : Claude Code réserve les noms de ses serveurs intégrés, y compris `workspace`, `claude-in-chrome`, `computer-use`, `Claude Preview`, et `Claude Browser`. Si votre configuration définit un serveur avec un nom réservé, Claude Code le saute au moment du chargement et affiche un avertissement vous demandant de le renommer. `claude mcp add` rejette un nom réservé avec une erreur. `Claude Preview` et `Claude Browser` nomment tous deux le serveur intégré que le [volet d'aperçu de l'application de bureau Claude Code](/docs/fr/desktop#preview-your-app) utilise. Avant la v2.1.205, `Claude Browser` n'était pas réservé, donc un serveur configuré par l'utilisateur pouvait s'enregistrer sous ce nom.
* **Variable d'environnement manquante** : si une référence [`${VAR}`](#environment-variable-expansion-in-mcp-json) dans la configuration d'un serveur nomme une variable qui n'est pas définie et n'a pas de `:-default`, Claude Code avertit dans la sortie de `claude mcp list` et dans `/mcp`, en nommant la variable, et charge toujours le serveur avec le texte `${VAR}` non développé. Définissez la variable ou ajoutez un fallback `${VAR:-default}`. Dans l'`url` et les `headers` d'un serveur distant, certaines variables d'identifiants [lisent comme vides](#credential-variables-that-read-as-empty) à la place, sans avertissement.

<h4 id="tool-availability">
  Disponibilité des outils
</h4>

Le panneau `/mcp` affiche le nombre d'outils à côté de chaque serveur connecté et signale les serveurs qui annoncent la capacité des outils mais n'exposent aucun outil.

Si votre demande a besoin d'outils d'un serveur qui se connecte toujours en arrière-plan, Claude attend que ce serveur se connecte avant de continuer. La façon dont l'attente se produit dépend de votre configuration :

* **Avec [recherche d'outils](#scale-with-mcp-tool-search), la valeur par défaut** : l'attente se produit à l'intérieur de l'appel `ToolSearch`.
* **Sans recherche d'outils** : Claude utilise plutôt l'outil `WaitForMcpServers`. Les configurations sans recherche d'outils incluent une `ANTHROPIC_BASE_URL` personnalisée, `ENABLE_TOOL_SEARCH=false`, et un modèle antérieur à la génération Claude 4.5 sur la plateforme Agent de Google Cloud.
* **Sur un déploiement Microsoft Foundry [hébergé sur Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options)** : Claude commence sur le chemin de recherche d'outils plutôt qu'avec `WaitForMcpServers`, car Claude Code découvre le rejet côté serveur du déploiement uniquement à partir de l'API. Après que Claude Code bascule ce déploiement vers [chargement en amont](#scale-with-mcp-tool-search), les outils d'un serveur qui termine la connexion deviennent disponibles à la demande suivante de Claude.

Avec la recherche d'outils activée, lorsqu'un serveur termine la connexion pendant que Claude travaille, Claude Code liste les noms d'outils du serveur à Claude à sa demande suivante dans le même tour. Claude peut alors rechercher et appeler ces outils sans attendre votre message suivant.

<h3 id="disable-a-server-without-removing-it">
  Désactiver un serveur sans le supprimer
</h3>

Basculez un serveur dans le panneau `/mcp` pour arrêter Claude Code de s'y connecter sans perdre sa configuration. Claude Code liste toujours le serveur dans `/mcp`, marqué comme désactivé.

Lorsque vous basculez un serveur, Claude Code enregistre votre choix par projet dans `~/.claude.json`, dans l'une de deux listes qui couvrent des ensembles disjoints de serveurs :

* `disabledMcpServers` : une liste d'exclusion pour les serveurs configurés par l'utilisateur, les serveurs de plugins, les serveurs que votre organisation [fournit via les paramètres gérés](/docs/fr/managed-mcp#provide-servers-through-managed-settings), les connecteurs claude.ai que Claude Code [récupère lui-même](#how-connectors-reach-claude-code), et les serveurs intégrés qui sont activés par défaut. Claude Code ne se connecte pas à un serveur que vous listez ici. Lorsque vous désactivez un connecteur claude.ai avec le basculement `/mcp` par projet décrit dans [Désactiver les connecteurs claude.ai](#disable-claude-ai-connectors), Claude Code l'écrit dans cette liste sous son nom d'affichage, par exemple `claude.ai Slack`.
* `enabledMcpServers` : une liste d'inclusion pour les serveurs intégrés qui sont désactivés par défaut, tels que `computer-use`. Claude Code se connecte à un serveur désactivé par défaut uniquement lorsque vous le listez ici.

Claude Code consulte exactement l'une des deux listes pour chaque serveur, donc ni l'une ni l'autre ne remplace l'autre. Si vous ajoutez un serveur régulier à `enabledMcpServers`, ou un serveur intégré désactivé par défaut à `disabledMcpServers`, Claude Code ignore l'entrée.

`disabledMcpServers` et `enabledMcpServers` ne sont pas liés à [`enabledMcpjsonServers`](/docs/fr/settings-reference#enabledmcpjsonservers) et [`disabledMcpjsonServers`](/docs/fr/settings-reference#disabledmcpjsonservers), qui contrôlent l'approbation des serveurs définis dans le fichier `.mcp.json` d'un projet.

<h3 id="mcp-client-runtimes">
  Runtimes clients MCP
</h3>

Claude Code se connecte aux serveurs MCP via l'un de deux runtimes clients. Le runtime v1 est construit sur MCP TypeScript SDK 1.x. Le runtime v2 est le même code sur [MCP TypeScript SDK 2.0](https://ts.sdk.modelcontextprotocol.io/v2/), qui ajoute la révision de protocole MCP 2026-07-28. Le reste de cette page s'applique aux deux runtimes, sauf si une section nomme le runtime v2.

Claude Code choisit un runtime chaque fois que vous le démarrez et le conserve jusqu'à ce que vous quittiez. Dans les sessions où il [récupère les drapeaux de fonctionnalité](/docs/fr/env-vars#features-that-need-feature-flag-fetching), il utilise le runtime v2 sur Claude Code v2.1.232 ou ultérieur.

Dans les sessions où il ne récupère pas les drapeaux de fonctionnalité, Claude Code utilise le runtime v2 par défaut sur Claude Code v2.1.274 ou ultérieur :

* Sessions sur Amazon Bedrock, Claude Platform sur AWS, plateforme Agent de Google Cloud, ou Microsoft Foundry, sauf si une plateforme hôte qui intègre Claude Code définit [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/fr/env-vars)
* Sessions connectées via une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway)
* Sessions où vous désactivez la télémétrie ou la récupération des drapeaux de fonctionnalité, par exemple avec `DISABLE_TELEMETRY`

Sur v2, Claude Code fait également :

* Demande aux serveurs HTTP s'ils supportent la révision plus récente, et l'utilise avec ceux qui le font. Il demande également aux serveurs connecteurs claude.ai dans les sessions où il récupère les drapeaux de fonctionnalité. Pour qu'il demande aux serveurs stdio, ou aux serveurs connecteurs dans chaque session, définissez [`MCP_PROTOCOL_NEGOTIATION`](/docs/fr/env-vars) à `auto`. Il se connecte à tous les autres serveurs comme v1 le fait.
* Reçoit les notifications `list_changed` des serveurs sur la révision plus récente sur un [flux qu'il maintient ouvert](#notification-streams-on-the-v2-runtime).
* N'enregistre pas un serveur [channel](#push-messages-with-channels) qui se connecte sur la révision plus récente, car cette révision ne peut pas transporter les messages de canal.
* Échoue une [connexion OAuth MCP](#authenticate-with-remote-mcp-servers) dont la réponse d'autorisation nomme un émetteur inattendu.

Anthropic peut garder un serveur spécifique sur le protocole antérieur, ou hors de ce flux, avec un drapeau de fonctionnalité que Claude Code récupère.

Pour choisir le runtime vous-même, définissez [`MCP_SDK_GENERATION`](/docs/fr/env-vars) à `v1` ou `v2`. Pour décider si Claude Code demande, définissez [`MCP_PROTOCOL_NEGOTIATION`](/docs/fr/env-vars) à `auto` ou `legacy`.

<h3 id="dynamic-tool-updates">
  Mises à jour dynamiques des outils
</h3>

Claude Code supporte les notifications MCP `list_changed`, permettant aux serveurs MCP de mettre à jour dynamiquement leurs outils, invites, et ressources disponibles sans vous obliger à vous déconnecter et reconnecter. Lorsqu'un serveur MCP envoie une notification `list_changed`, Claude Code actualise automatiquement les capacités disponibles de ce serveur.

Si une demande d'actualisation échoue, Claude Code conserve les outils, invites, et ressources précédemment découverts du serveur jusqu'à ce qu'une actualisation ultérieure réussisse. Avant la v2.1.214, une erreur transitoire lors de l'actualisation remplaçait les outils, invites, et ressources du serveur par une liste vide.

<h4 id="notification-streams-on-the-v2-runtime">
  Flux de notification sur le runtime v2
</h4>

Sur le [runtime v2](#mcp-client-runtimes), Claude Code reçoit les notifications `list_changed` d'un serveur sur la révision de protocole plus récente sur un flux qu'il maintient ouvert. Lorsque le flux se ferme, Claude Code le rouvre, avec deux limites :

* **Le flux se ferme à nouveau dans les 10 secondes** : Claude Code le rouvre jusqu'à trois fois, puis s'arrête pour cette connexion.
* **Le flux reste ouvert plus de 10 secondes, puis se ferme**, comme les flux vers les hôtes sans serveur le font couramment : après cinq réouvertures en une heure, Claude Code attend environ six heures avant la suivante.

Jusqu'à ce que le flux se rouvre, vous conservez les outils, invites, et ressources du serveur dernièrement récupérés. Pour récupérer ses modifications plus tôt, reconnectez le serveur à partir de `/mcp`.

<h3 id="automatic-reconnection">
  Reconnexion automatique
</h3>

Claude Code reconnecte un serveur distant qui se déconnecte en cours de session et réessaie la première connexion d'un serveur HTTP ou SSE après une erreur transitoire. Les serveurs stdio sont des processus locaux, et Claude Code ne les reconnecte pas automatiquement.

<h4 id="mid-session-drops-of-a-remote-server">
  Déconnexions en cours de session d'un serveur distant
</h4>

Claude Code reconnecte un serveur distant déconnecté avec un backoff exponentiel : jusqu'à cinq tentatives, en commençant par un délai d'une seconde et en le doublant à chaque fois. Ce que vous voyez dépend de la façon dont vous exécutez Claude Code :

* **Dans une session interactive** : `/mcp` affiche le serveur comme en attente pendant que Claude Code se reconnecte. Après cinq tentatives échouées, Claude Code marque le serveur comme défaillant, ou comme ayant besoin d'authentification lorsque le serveur a besoin d'être autorisé à nouveau. Lorsqu'il marque le serveur comme défaillant, vous voyez une notification `MCP server "<name>" disconnected · open /mcp to reconnect`. Vous pouvez réessayer manuellement à partir de `/mcp`.
* **Dans les exécutions [`claude -p`](/docs/fr/headless) et les sessions [Agent SDK](/docs/fr/agent-sdk/overview)** : Claude Code se reconnecte selon le même calendrier, sans panneau `/mcp` pour afficher les tentatives.

<h4 id="failed-first-connections">
  Échecs de première connexion
</h4>

Lorsque la première connexion d'un serveur HTTP ou SSE échoue avec une erreur transitoire, telle qu'une réponse 5xx, une connexion refusée, ou un délai d'expiration, Claude Code réessaie jusqu'à trois fois. Si la connexion échoue toujours, Claude Code marque le serveur comme défaillant. Claude Code réessaie de cette façon au démarrage et lorsqu'un serveur est ajouté en cours de session. Cela inclut un serveur que Claude Code ajoute à une [session cloud](/docs/fr/claude-code-on-the-web) à partir de sa configuration et un serveur que vous ajoutez avec la méthode [`setMcpServers()`](/docs/fr/agent-sdk/typescript) du Agent SDK.

Claude Code ne réessaie pas dans ces cas :

* La première connexion d'un serveur WebSocket
* Une erreur d'authentification ou non trouvée, car elle nécessite une modification de configuration pour être résolue. Lorsqu'un [`headersHelper`](#use-dynamic-headers-for-custom-authentication) est la seule source du serveur de l'en-tête `Authorization`, Claude Code réessaie quand même une erreur d'authentification, car il réexécute l'assistant à chaque tentative et peut récupérer une identifiant frais

<h4 id="failed-discovery-requests">
  Demandes de découverte échouées
</h4>

Après qu'un serveur se connecte, Claude Code lui envoie des demandes de découverte de capacités telles que `tools/list`, `prompts/list`, et `resources/list`. Claude Code réessaie ces demandes jusqu'à trois fois avec un backoff court après une erreur réseau ou serveur transitoire. Il ne réessaie pas les erreurs d'authentification, les réponses 4xx, ou les délais d'expiration de demande.

<h4 id="how-claude-learns-that-a-server-failed">
  Comment Claude apprend qu'un serveur a échoué
</h4>

Que Claude Code dise à Claude qu'un serveur configuré n'a pas pu se connecter dépend de la [recherche d'outils](#scale-with-mcp-tool-search), qui est activée par défaut :

* Avec la recherche d'outils, Claude Code dit à Claude quel serveur a échoué et son erreur de connexion, donc Claude signale l'échec de la connexion dans sa réponse. Claude Code inclut les mêmes informations dans les résultats `ToolSearch` qui ne trouvent aucun outil correspondant.
* Dans toute [configuration sans recherche d'outils](#configure-tool-search), Claude Code ne signale pas les échecs de connexion du serveur à Claude.

<h3 id="push-messages-with-channels">
  Pousser des messages avec des canaux
</h3>

Un serveur MCP peut également pousser des messages directement dans votre session afin que Claude puisse réagir à des événements externes comme les résultats CI, les alertes de surveillance, ou les messages de chat. Pour activer cela, votre serveur déclare la capacité `claude/channel` et vous l'acceptez avec le drapeau `--channels` au démarrage. Voir [Canaux](/docs/fr/channels) pour utiliser un canal officiellement supporté, ou [Référence des canaux](/docs/fr/channels-reference) pour construire le vôtre.

Sur le [runtime v2](#mcp-client-runtimes), si vous définissez [`MCP_PROTOCOL_NEGOTIATION`](/docs/fr/env-vars) à `auto` et qu'un serveur de canal négocie la révision de protocole MCP 2026-07-28, il ne peut pas livrer les messages de canal, donc Claude Code ne l'enregistre pas comme un canal. Laisser la variable non définie, ou la définir à `legacy`, garde les serveurs stdio sur la poignée de main antérieure.

<Tip>
  Conseils :

  * Utilisez le drapeau `-s` ou `--scope` pour spécifier où la configuration est stockée :
    * `local` (par défaut) : disponible uniquement pour vous dans le projet actuel
    * `project` : partagé avec tout le monde dans le projet via le fichier `.mcp.json`
    * `user` : disponible pour vous dans tous les projets
  * Définissez les variables d'environnement avec les drapeaux `-e` ou `--env` (par exemple, `-e KEY=value`)
  * Les drapeaux `--transport` et `--header` acceptent également les formes courtes `-t` et `-H`
  * Configurez le délai d'expiration de démarrage du serveur MCP en utilisant la variable d'environnement `MCP_TIMEOUT` (par exemple, `MCP_TIMEOUT=10000 claude` définit un délai d'expiration de 10 secondes)
  * Définissez un délai d'expiration d'exécution d'outil par serveur en ajoutant un champ `timeout` en millisecondes à l'entrée `.mcp.json` de ce serveur, par exemple `"timeout": 600000` pour dix minutes. Cela remplace la variable d'environnement `MCP_TOOL_TIMEOUT` pour ce serveur uniquement
  * Claude Code affiche un avertissement lorsque la sortie de l'outil MCP dépasse 10 000 jetons et limite la sortie à 25 000 jetons par défaut. Pour augmenter la limite, définissez la variable d'environnement `MAX_MCP_OUTPUT_TOKENS` (par exemple, `MAX_MCP_OUTPUT_TOKENS=50000`) ; le seuil d'avertissement est fixe. Voir [Limites et avertissements de sortie MCP](#mcp-output-limits-and-warnings)
  * Utilisez `/mcp` pour vous authentifier auprès des serveurs distants qui nécessitent une authentification OAuth 2.0
</Tip>

Le `timeout` par serveur est une limite de temps mur dur par appel d'outil, et les notifications de progression du serveur ne l'étendent pas. Les valeurs inférieures à 1000 sont ignorées et tombent à `MCP_TOOL_TIMEOUT`, ou à sa valeur par défaut d'environ 28 heures lorsque cette variable n'est pas définie. Pour un serveur HTTP, SSE, ou [connecteur claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai), il y a également un deuxième minuteur par demande qui couvre chaque demande jusqu'au premier octet de réponse du serveur. Claude Code définit ce minuteur à la plus grande de trois valeurs : 60 secondes, le délai d'expiration de l'outil qui s'applique au serveur, et `MCP_TIMEOUT`. La valeur par défaut de 28 heures d'un `MCP_TOOL_TIMEOUT` non défini n'entre pas dans cette comparaison, et une valeur inférieure à 60 secondes ne raccourcit pas le minuteur. Les serveurs stdio et WebSocket n'ont pas de minuteur par demande.

Un `timeout` par serveur d'au moins 1000 agit également comme un plancher sur le délai d'inactivité décrit ci-dessous : Claude Code n'abandonne jamais les appels d'outil de ce serveur pour inactivité plus tôt que le `timeout` par serveur. Nécessite Claude Code v2.1.203 ou ultérieur.

Un appel d'outil à un serveur MCP qui n'envoie aucune réponse et aucune notification de progression pendant la fenêtre d'inactivité abandonne avec une erreur au lieu d'attendre la limite de temps mur. Il s'applique à tous les types de serveurs sauf les serveurs IDE et les serveurs en processus du SDK. La fenêtre d'inactivité par défaut est de cinq minutes pour les serveurs HTTP, SSE, WebSocket, et [connecteur claude.ai](#use-mcp-servers-from-claude-ai), et de 30 minutes pour les serveurs stdio. Avant la v2.1.203, les serveurs stdio étaient exempts du délai d'inactivité.

Définissez la variable d'environnement [`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](/docs/fr/env-vars) en millisecondes pour changer la fenêtre d'inactivité, ou définissez-la à `0` pour désactiver la vérification.

Ces délais d'expiration limitent la durée pendant laquelle un appel peut s'exécuter, pas toujours la durée pendant laquelle il bloque la session : un appel principal de conversation qui s'exécute au-delà de deux minutes se déplace d'abord vers une tâche en arrière-plan. Voir [Mise en arrière-plan automatique des appels d'outil longs](#automatic-backgrounding-of-long-tool-calls).

<h3 id="automatic-backgrounding-of-long-tool-calls">
  Mise en arrière-plan automatique des appels d'outil longs
</h3>

Un appel d'outil MCP dans la conversation principale qui s'exécute toujours après deux minutes se déplace vers une tâche en arrière-plan au lieu de bloquer la session. Claude reçoit l'ID de la tâche immédiatement et continue de travailler, et le résultat arrive comme une notification de tâche lorsque l'appel se règle. La mise en arrière-plan automatique nécessite Claude Code v2.1.212 ou ultérieur.

La tâche apparaît dans [`/tasks`](/docs/fr/commands#all-commands), où vous pouvez également l'arrêter, et elle ne survit pas à la sortie de la session. Les limites par appel s'appliquent toujours pendant que l'appel s'exécute en arrière-plan : la limite de temps mur définie par le `timeout` par serveur ou [`MCP_TOOL_TIMEOUT`](/docs/fr/env-vars), et le délai d'inactivité défini par [`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](/docs/fr/env-vars).

Définissez la variable d'environnement [`CLAUDE_CODE_MCP_AUTO_BACKGROUND_MS`](/docs/fr/env-vars) en millisecondes pour changer le seuil, ou définissez-la à `0` pour désactiver la mise en arrière-plan automatique. Définir `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` à `1` la désactive également, ainsi que toutes les autres fonctionnalités de tâche en arrière-plan.

Certains appels ne se déplacent jamais vers l'arrière-plan :

* Appels à partir de [sous-agents](/docs/fr/sub-agents) ; Claude Code met en arrière-plan uniquement les appels de conversation principale
* Appels aux serveurs IDE
* Appels en [mode non interactif](/docs/fr/headless), sauf si `CLAUDE_AUTO_BACKGROUND_TASKS` est défini à `1`, car une exécution unique peut se terminer avant l'arrivée du résultat

Un appel en attente d'une [boîte de dialogue d'élicitation](#respond-to-mcp-elicitation-requests) ouverte n'est pas mis en arrière-plan pendant que la boîte de dialogue est ouverte ; le serveur est bloqué sur votre entrée, pas lent, donc Claude Code diffère le déplacement jusqu'à la fermeture de la boîte de dialogue.

<h3 id="plugin-provided-mcp-servers">
  Serveurs MCP fournis par les plugins
</h3>

[Les plugins](/docs/fr/plugins/overview) peuvent regrouper les serveurs MCP qui fournissent des outils et des intégrations lorsque vous activez le plugin. Les serveurs MCP des plugins fonctionnent de manière identique aux serveurs configurés par l'utilisateur.

**Comment fonctionnent les serveurs MCP des plugins** :

* Les plugins définissent les serveurs MCP dans `.mcp.json` à la racine du plugin ou en ligne dans `plugin.json`
* Lorsque vous activez un plugin, Claude Code démarre automatiquement ses serveurs MCP
* Claude Code propose les outils MCP des plugins aux côtés des outils MCP configurés manuellement
* Vous ajoutez et supprimez les serveurs de plugins en installant ou en désinstallant le plugin, pas avec les commandes `/mcp`. Vous pouvez toujours [basculer un serveur de plugin installé](#disable-a-server-without-removing-it) dans `/mcp`, ce qui arrête Claude Code de s'y connecter sans supprimer le plugin

**Exemple de configuration MCP de plugin** :

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

**Fonctionnalités MCP des plugins** :

* **Cycle de vie automatique** : les serveurs se connectent et se déconnectent à ces points :
  * Au démarrage de la session, Claude Code connecte automatiquement les serveurs des plugins activés. Dans `/mcp`, un serveur de plugin distant (HTTP ou SSE) que vous avez utilisé auparavant peut afficher le statut [`cached`](#server-status-detail) à la place ; Claude Code le connecte lorsque Claude appelle pour la première fois l'un de ses outils
  * Si vous activez ou désactivez un plugin pendant une session, Claude Code connecte ou déconnecte ses serveurs MCP lorsque la modification s'applique. [Appliquer les modifications de plugin sans redémarrer](/docs/fr/plugins/cli-reference#reload-plugins) décrit quand c'est le cas. Dans une session sans terminal interactif, `/reload-plugins` ne connecte ni ne déconnecte les serveurs MCP des plugins ; ces modifications prennent effet dans votre session suivante
  * Lorsque vous rechargez, Claude Code conserve les connexions actives des serveurs de plugins dont la configuration est inchangée, et fait de même lorsque vous [remplacez la liste des serveurs MCP de la session](/docs/fr/agent-sdk/typescript#mcpsetserversresult) à partir du Agent SDK sans les nommer
  * Lorsque vous [déplacez la session avec `/cd`](/docs/fr/permissions#move-the-session-to-another-directory) sur v2.1.246 ou ultérieur, Claude Code connecte les serveurs des plugins que les paramètres du nouveau répertoire activent et déconnecte les serveurs des plugins qui ne sont plus activés, afin que vous n'ayez pas besoin d'exécuter `/reload-plugins` après le déplacement
  * Dans les [sessions cloud](/docs/fr/claude-code-on-the-web), un appel MCP à un serveur de plugin qui n'est pas encore connecté, tel que juste après le réveil d'une session inactive, démarre le serveur à la demande et attend qu'il se connecte
* **Espaces réservés de chemin** : `${CLAUDE_PLUGIN_ROOT}` se résout au répertoire d'installation du plugin, `${CLAUDE_PLUGIN_DATA}` à son répertoire d'[état persistant](/docs/fr/plugins/components#path-variables-and-persistent-data), et `${CLAUDE_PROJECT_DIR}` à la racine de projet stable. La substitution s'applique à :
  * serveurs `stdio` : `command`, `args`, `env`
  * serveurs `http`, `sse`, et `ws` : `url`, `headers`, et `headersHelper`. Avant la v2.1.195, `headersHelper` transmettait l'espace réservé comme une chaîne littérale
* **Accès à l'environnement utilisateur** : accès aux mêmes variables d'environnement que les serveurs configurés manuellement
* **Types de transport multiples** : support pour les transports stdio, SSE, HTTP, et WebSocket, bien que le support de transport puisse varier selon le serveur

Les serveurs de plugins apparaissent dans `/mcp` avec des indicateurs montrant qu'ils proviennent de plugins.

**Noms d'outils MCP des plugins** :

Les outils d'un serveur MCP regroupé dans un plugin incluent à la fois le nom du plugin et la clé du serveur dans leur nom appelable. La forme complète est `mcp__plugin_<plugin-name>_<server-name>__<tool-name>`, où tout caractère en dehors de `A-Z`, `a-z`, `0-9`, `_`, et `-` est remplacé par `_`. Pour le serveur `database-tools` regroupé dans un plugin nommé `my-plugin`, un outil `query` est appelable comme :

```
mcp__plugin_my-plugin_database-tools__query
```

Utilisez ce nom complet lorsque vous référencez l'outil dans les [règles de permission](/docs/fr/permissions), la liste `allowed-tools` d'une compétence, le champ `tools` d'un [sous-agent](/docs/fr/sub-agents#available-tools), ou un [matcher de hook](/docs/fr/hooks#match-mcp-tools). Un matcher de hook écrit contre la clé de serveur nue, tel que `mcp__database-tools__.*`, ne se déclenche jamais pour un serveur regroupé dans un plugin.

Le serveur lui-même s'enregistre sous le nom scoped `plugin:<plugin-name>:<server-name>`, tel que `plugin:my-plugin:database-tools`. Utilisez ce nom où un nom de serveur configuré est attendu, tel que le champ `server` d'un [hook `mcp_tool`](/docs/fr/hooks#mcp-tool-hook-fields).

Voir la [référence des composants de plugin](/docs/fr/plugins/components#mcp-servers) pour les détails sur le regroupement des serveurs MCP avec les plugins.

<h2 id="mcp-installation-scopes">
  Portées d'installation MCP
</h2>

Les serveurs MCP peuvent être configurés à trois portées différentes. La portée que vous choisissez contrôle les projets dans lesquels le serveur se charge et si la configuration est partagée avec votre équipe. Les administrateurs peuvent également déployer ou fournir des serveurs pour chaque utilisateur via la [configuration gérée](#managed-mcp-configuration).

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
  Le terme « portée locale » pour les serveurs MCP diffère des paramètres locaux généraux. Les serveurs MCP à portée locale sont stockés dans `~/.claude.json` (votre répertoire personnel), tandis que les paramètres locaux généraux utilisent `.claude/settings.local.json` (dans le répertoire du projet). Consultez [Paramètres](/docs/fr/settings#where-settings-live) pour plus de détails sur les emplacements des fichiers de paramètres.
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

Les serveurs à portée de projet permettent la collaboration d'équipe en stockant les configurations dans un fichier `.mcp.json` à la racine de votre projet. Lorsque vous ajoutez un serveur à portée de projet, Claude Code crée ou met à jour automatiquement ce fichier avec la structure de configuration appropriée. Archivez `.mcp.json` dans le contrôle de version pour que tous les membres de votre équipe obtiennent les mêmes outils et services MCP.

```bash theme={null}
# Ajouter un serveur à portée de projet
claude mcp add --transport http shared-server --scope project https://example.com/mcp
```

Le fichier `.mcp.json` résultant suit un format standardisé :

```json theme={null}
{
  "mcpServers": {
    "shared-server": {
      "type": "http",
      "url": "https://example.com/mcp"
    }
  }
}
```

Pour des raisons de sécurité, Claude Code demande une approbation dans les sessions interactives avant d'utiliser les serveurs à portée de projet à partir des fichiers `.mcp.json`. Pour réinitialiser ces choix d'approbation, exécutez `claude mcp reset-project-choices`.

Dans les exécutions `claude -p`, les sessions du [SDK Agent](/docs/fr/headless) et les [sessions cloud](/docs/fr/claude-code-on-the-web), Claude Code ne peut pas afficher cette invite : il charge les serveurs à portée de projet sans demander. Claude Code ignore également l'invite dans une session que vous démarrez en mode `bypassPermissions` avec [`skipDangerousModePermissionPrompt`](/docs/fr/settings-reference#skipdangerousmodepermissionprompt) défini dans vos paramètres utilisateur ou dans les paramètres gérés. Pour garder un serveur à l'écart de toute façon :

* Ajoutez-le à [`disabledMcpjsonServers`](/docs/fr/settings-reference#disabledmcpjsonservers), qui le bloque dans tous les modes de permission.
* Excluez entièrement les paramètres du projet avec [`--setting-sources`](/docs/fr/cli-reference#cli-flags) ou l'option `settingSources` du SDK.
* Démarrez la session avec [`--strict-mcp-config`](/docs/fr/cli-reference#cli-flags). Claude Code utilise alors uniquement les serveurs MCP que vous transmettez avec `--mcp-config`. Ignorer l'invite d'approbation pour les serveurs à portée de projet que Claude Code ne charge pas nécessite Claude Code v2.1.246 ou ultérieur ; avant v2.1.246, une session stricte attendait toujours l'approbation pour eux, ce qui laissait les sessions en arrière-plan en attente au démarrage. Consultez [Contrôle exclusif avec managed-mcp.json](/docs/fr/managed-mcp#exclusive-control-with-managed-mcp-json) pour voir ce que le drapeau fait sous un fichier MCP géré.

[Approbations des serveurs de projet et confiance de l'espace de travail](#project-server-approvals-and-workspace-trust) couvre la façon dont les approbations validées dans le référentiel interagissent avec la confiance de l'espace de travail.

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
4. [Serveurs fournis par les plugins](/docs/fr/plugins/components#mcp-servers)
5. [Connecteurs claude.ai](#use-mcp-servers-from-claude-ai)

Les trois portées correspondent aux doublons par nom. Les plugins et les connecteurs correspondent par point de terminaison, donc celui qui pointe vers la même URL ou commande qu'un serveur ci-dessus est traité comme un doublon.

Un serveur que votre organisation fournit via le paramètre géré [`managedMcpServers`](/docs/fr/managed-mcp#provide-servers-through-managed-settings) se classe au-dessus de tous ceux-ci, donc lorsque l'un d'eux le duplique, Claude Code se connecte à la définition de l'organisation. Nécessite Claude Code v2.1.259 ou ultérieur.

Si vous ouvrez une session locale dans l'[onglet Code de l'application de bureau](/docs/fr/desktop#mcp-servers-from-the-claude-desktop-chat-app) avec le même nom de serveur stdio au niveau supérieur de `~/.claude.json` (portée utilisateur) et dans `.mcp.json`, l'onglet Code utilise la définition `~/.claude.json`.

<h3 id="environment-variable-expansion-in-mcp-json">
  Expansion des variables d'environnement dans `.mcp.json`
</h3>

Claude Code supporte l'expansion des variables d'environnement dans les fichiers `.mcp.json`, permettant aux équipes de partager des configurations tout en maintenant la flexibilité pour les chemins spécifiques à la machine et les valeurs sensibles comme les clés API.

<h4 id="supported-syntax">
  Syntaxe supportée
</h4>

* `${VAR}` : se développe à la valeur de la variable d'environnement `VAR`
* `${VAR:-default}` : se développe à `VAR` si défini, sinon utilise `default`

<h4 id="expansion-locations">
  Emplacements d'expansion
</h4>

Les variables d'environnement peuvent être développées dans :

* `command` : le chemin de l'exécutable du serveur
* `args` : arguments de la ligne de commande
* `env` : variables d'environnement passées au serveur
* `url` : pour les types de serveur HTTP
* `headers` : pour l'authentification du serveur HTTP

<h4 id="example-with-variable-expansion">
  Exemple avec expansion de variable
</h4>

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

<h4 id="unset-variables-without-a-default">
  Variables non définies sans valeur par défaut
</h4>

Si une variable d'environnement requise n'est pas définie et n'a pas de valeur par défaut, la configuration se charge toujours : Claude Code signale un avertissement de variable manquante pour ce serveur dans la sortie `claude mcp list` et utilise le texte non développé `${VAR}` tel quel. Définissez la variable ou ajoutez un fallback `:-default` pour que le serveur démarre avec la valeur que vous avez l'intention d'utiliser. Dans l'`url` et les `headers` d'un serveur distant, certaines variables d'identification [se lisent comme vides](#credential-variables-that-read-as-empty) à la place, sans avertissement.

<h4 id="credential-variables-that-read-as-empty">
  Variables d'identification qui se lisent comme vides
</h4>

Dans l'`url` et les `headers` d'un serveur distant, Claude Code lit les variables d'identification de votre environnement comme vides plutôt que de les développer. Cela empêche le `.mcp.json` d'un projet ou un plugin d'envoyer vos identifiants Claude Code ou de fournisseur cloud à un serveur qu'il nomme. Si vous écrivez `Bearer ${ANTHROPIC_AUTH_TOKEN}`, le serveur reçoit `Bearer ` sans identifiant et rejette la demande, généralement avec un `401`. Claude Code signale cela comme une connexion échouée.

Les noms couverts sont :

* Les identifiants propres de Claude Code, tels que `ANTHROPIC_API_KEY` et `ANTHROPIC_AUTH_TOKEN`
* Les identifiants de votre fournisseur cloud, tels que `AWS_BEARER_TOKEN_BEDROCK`
* Autres identifiants que votre environnement porte, tels que `HTTPS_PROXY` et `NPM_TOKEN`

Un nom couvert se lit comme vide que vous ayez défini la variable ou non, et un fallback `:-default` sur celui-ci est ignoré. Une URL de base de fournisseur telle que `ANTHROPIC_BASE_URL` se développe toujours, donc `"url": "${ANTHROPIC_BASE_URL}/mcp"` fonctionne, sauf si la valeur de l'URL elle-même intègre un identifiant tel qu'un nom d'utilisateur et un mot de passe.

Un nom en dehors de cet ensemble, tel que `API_KEY`, se développe tel qu'écrit. Pour donner au serveur l'un des identifiants couverts, copiez-le dans une variable avec un nom de votre choix et référencez ce nom à la place.

Lorsque l'`url` ou les `headers` d'un serveur distant référencent une variable couverte que vous avez définie, Claude Code la nomme dans une ligne de journal de débogage. Pour lire la ligne, exécutez `claude --debug-file /tmp/claude-debug.log` et recherchez dans ce fichier `never expanded toward a remote server`.

<h4 id="how-references-appear-in-/mcp-and-cli-output">
  Comment les références apparaissent dans `/mcp` et la sortie CLI
</h4>

Pour un serveur dans la [portée](#mcp-installation-scopes) locale, de projet ou utilisateur, les surfaces suivantes affichent une référence `${VAR}` par nom plutôt que comme sa valeur résolue :

* L'URL ou la ligne de commande dans la vue de détail `/mcp` d'un serveur
* Sortie `claude mcp list` et `claude mcp get`

La vue de détail `/mcp` affiche les références de cette façon dans Claude Code v2.1.268 ou ultérieur.

Pour un serveur que votre organisation fournit via le paramètre `managedMcpServers`, ces surfaces affichent [uniquement l'hôte de l'URL](/docs/fr/managed-mcp#what-users-can-see-and-change).

Pour vérifier ce que `claude mcp list`, `claude mcp get` et `/mcp` affichent lorsqu'une connexion échoue, consultez [Détail du statut du serveur](#server-status-detail).

<h2 id="practical-examples">
  Exemples pratiques
</h2>

<h3 id="example-connect-to-github-for-code-reviews">
  Exemple : Se connecter à GitHub pour les révisions de code
</h3>

Le serveur MCP distant de GitHub s'authentifie avec un jeton d'accès personnel GitHub transmis en tant qu'en-tête. Pour en obtenir un, ouvrez vos [paramètres de jeton GitHub](https://github.com/settings/personal-access-tokens), générez un nouveau jeton à granularité fine avec accès aux référentiels avec lesquels vous souhaitez que Claude travaille, puis ajoutez le serveur :

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

Remplacez `YOUR_GITHUB_PAT` par votre jeton d'accès personnel. La commande `claude mcp add` enregistre la configuration sans valider les identifiants, donc une valeur d'espace réservé est acceptée ici mais le serveur ne parvient pas à se connecter ultérieurement. Pour vérifier la connexion, exécutez `/mcp` et vérifiez que le serveur affiche `connected`. Un serveur avec de mauvais identifiants affiche `failed`, et le détail de l'échec inclut le statut HTTP que le serveur a renvoyé, comme un 401.

Ensuite, travaillez avec GitHub :

```text wrap theme={null}
Examinez la PR #456 et suggérez des améliorations
```

```text wrap theme={null}
Créer un nouveau problème pour le bogue que nous venons de trouver
```

```text wrap theme={null}
Montrez-moi toutes les PR ouvertes qui me sont assignées
```

<h3 id="example-query-your-postgresql-database">
  Exemple : Interroger votre base de données PostgreSQL
</h3>

[DBHub](https://github.com/bytebase/dbhub), le package `@bytebase/dbhub`, est un serveur MCP qui connecte Claude à une base de données relationnelle via la chaîne de connexion que vous transmettez dans `--dsn`. Utilisez un utilisateur de base de données en lecture seule dans la chaîne de connexion afin que les requêtes que Claude exécute ne puissent pas modifier les données :

```bash theme={null}
claude mcp add --transport stdio db -- npx -y @bytebase/dbhub \
  --dsn "postgresql://readonly:pass@prod.db.com:5432/analytics"
```

Pour confirmer que le serveur démarre, exécutez `/mcp` et vérifiez que `db` affiche `connected`.

Ensuite, interrogez votre base de données naturellement :

```text wrap theme={null}
Quel est notre revenu total ce mois-ci ?
```

```text wrap theme={null}
Montrez-moi le schéma de la table des commandes
```

```text wrap theme={null}
Trouver les clients qui n'ont pas effectué d'achat depuis 90 jours
```

<h2 id="authenticate-with-remote-mcp-servers">
  S'authentifier auprès des serveurs MCP distants
</h2>

De nombreux serveurs MCP basés sur le cloud nécessitent une authentification. Claude Code supporte OAuth 2.0 pour les connexions sécurisées.

Claude Code marque un serveur distant comme nécessitant une authentification lorsque le serveur répond avec `401 Unauthorized` ou `403 Forbidden`. Ce que Claude Code affiche dépend du serveur :

* Pour un serveur auprès duquel vous ne vous êtes pas connecté, l'un ou l'autre code de statut le signale dans `/mcp` afin que vous puissiez compléter le flux OAuth.
* Pour un [connecteur claude.ai](#use-mcp-servers-from-claude-ai), un `401` causé par le rejet de votre jeton de session par claude.ai ne signale pas le connecteur, car la réautorisation du connecteur ne peut pas corriger votre connexion. Claude Code affiche plutôt l'[état de rejet du jeton de session](/docs/fr/errors#claude-ai-rejected-the-session-token).
* Pour un serveur dont vous avez configuré l'en-tête `Authorization`, dans `headers` ou via un [`headersHelper`](#use-dynamic-headers-for-custom-authentication), un `401` ou `403` lors de la connexion ne signale pas le serveur, car l'identifiant à corriger est celui que vous avez configuré. Claude Code signale plutôt la connexion comme échouée. Si vous avez défini cet en-tête à partir d'une référence `${VAR}`, vérifiez si cette variable est l'une que Claude Code [lit comme vide](#credential-variables-that-read-as-empty).
* Pour un connecteur [livré à une session cloud](#how-connectors-reach-claude-code), Claude Code n'exécute pas de flux de connexion, car le proxy de la session s'authentifie auprès du connecteur avec l'autorisation que vous avez accordée dans claude.ai. Lorsqu'un connecteur là-bas a besoin d'être autorisé à nouveau, reconnectez-le à [claude.ai/customize/connectors](https://claude.ai/customize/connectors) plutôt que depuis la session.

Lorsqu'une demande à un serveur OAuth auprès duquel vous vous êtes déjà connecté retourne `401 Unauthorized`, Claude Code actualise le jeton stocké, se reconnecte et réessaie la demande une fois. Il signale le serveur dans `/mcp` uniquement si cette nouvelle tentative échoue également. Avant la v2.1.206, une actualisation de jeton qui échouait pour une raison transitoire, comme une erreur réseau, signalait un serveur OAuth comme nécessitant une authentification pour le reste de la session même si son jeton d'actualisation était toujours valide.

Lorsque le serveur rejette le jeton d'actualisation stocké, Claude Code affiche immédiatement un avis pointant vers `/mcp`. Ouvrez `/mcp` et sélectionnez **Re-authenticate** sur le serveur pour vous connecter à nouveau avant que le prochain appel d'outil échoue.

Un serveur personnalisé qui retourne un en-tête `WWW-Authenticate` pointant vers son serveur d'autorisation obtient la même découverte automatique que tout autre serveur distant.

Claude Code affiche également un avis de démarrage lorsqu'un ou plusieurs serveurs configurés nécessitent une authentification, vous n'avez donc pas besoin d'ouvrir `/mcp` pour découvrir quels serveurs nécessitent une connexion. L'avis nécessite Claude Code v2.1.193 ou ultérieur. Il compte uniquement les serveurs auprès desquels vous pouvez vous connecter à partir de Claude Code. Avant la v2.1.218, il comptait également les [connecteurs claude.ai](#use-mcp-servers-from-claude-ai) qui n'étaient pas connectés dans claude.ai, que vous pouvez connecter uniquement à partir des paramètres claude.ai.

L'avis annonce chaque serveur une fois et l'exclut du décompte aux lancements ultérieurs jusqu'à ce que ce serveur se soit connecté et ait besoin d'une connexion à nouveau. `/mcp` liste toujours chaque serveur qui nécessite une connexion.

En mode non interactif, il n'y a pas de panneau `/mcp`, donc Claude Code ne peut pas exécuter le flux OAuth pour vous. À partir de la v2.1.196, lorsqu'un serveur configuré nécessite une authentification lors d'une exécution `claude -p` ou Agent SDK avec [recherche d'outils](#scale-with-mcp-tool-search) activée, ce qui est la valeur par défaut, Claude Code indique à Claude que les outils du serveur ne sont pas disponibles jusqu'à ce que vous l'autorisiez. Claude peut alors nommer le serveur qui nécessite une connexion au lieu de répondre comme si le serveur n'était pas configuré. Complétez la connexion à partir d'une session interactive avec `/mcp` ou `claude mcp login <name>`.

Si vous avez configuré `headers.Authorization` pour le serveur et que le serveur rejette cet en-tête, Claude Code signale la connexion comme échouée au lieu de revenir à OAuth. Vérifiez que le jeton est valide pour le point de terminaison MCP, ou supprimez l'en-tête pour utiliser le flux OAuth.

<Steps>
  <Step title="Ajouter le serveur qui nécessite une authentification">
    Si vous avez déjà ajouté le serveur `sentry` dans le [démarrage rapide MCP](/docs/fr/mcp-quickstart#connect-a-server-that-requires-sign-in), ignorez cette étape : exécuter `claude mcp add` à nouveau avec le même nom de serveur au même scope échoue avec `MCP server sentry already exists in local config`. Sinon, exécutez :

    ```bash theme={null}
    claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
    ```
  </Step>

  <Step title="Utiliser la commande /mcp dans Claude Code">
    Dans Claude Code, utilisez la commande :

    ```text wrap theme={null}
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

La commande `claude mcp login <name>` exécute le flux OAuth d'un serveur configuré directement depuis votre shell, vous n'avez donc pas besoin d'ouvrir le panneau `/mcp` dans une session.

```bash theme={null}
claude mcp login sentry
```

Pour effacer les identifiants stockés ultérieurement, exécutez `claude mcp logout <name>`.

`claude mcp login` détecte lorsqu'aucun navigateur local n'est disponible, par exemple lors d'une session SSH ou sur Linux sans serveur d'affichage, et imprime l'URL d'autorisation au lieu d'essayer d'ouvrir un navigateur. Ouvrez l'URL sur votre machine locale, puis collez l'URL de redirection complète de la barre d'adresse de votre navigateur à l'invite. La commande a besoin d'un terminal interactif pour l'étape de collage, donc connectez-vous avec `ssh -t`. Passez `--no-browser` pour forcer l'invite d'URL même lorsqu'un navigateur local est détecté.

```bash theme={null}
claude mcp login sentry --no-browser
```

<h3 id="use-a-fixed-oauth-callback-port">
  Utiliser un port de rappel OAuth fixe
</h3>

Certains serveurs MCP nécessitent un URI de redirection spécifique enregistré à l'avance. Par défaut, Claude Code choisit un port disponible aléatoire pour le rappel OAuth. Utilisez `--callback-port` pour fixer le port afin qu'il corresponde à un URI de redirection pré-enregistré de la forme `http://localhost:PORT/callback`. Si la connexion échoue sur Claude Code v2.1.229 avec une erreur de non-concordance d'URI de redirection, consultez la note de version sous [Utiliser les identifiants OAuth pré-configurés](#use-pre-configured-oauth-credentials).

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

    Dans la v2.1.229, Claude Code envoyait `http://127.0.0.1:PORT/callback` à la place, et les serveurs qui correspondent exactement à l'URI de redirection enregistré rejetaient la connexion avec une erreur de non-concordance d'URI de redirection. Claude Code v2.1.231 a restauré la forme `localhost`. Pour récupérer sur la v2.1.229, mettez à niveau Claude Code, ou ajoutez temporairement la forme `http://127.0.0.1:PORT/callback` aux URI de redirection enregistrés du serveur.
  </Step>

  <Step title="Ajouter le serveur avec vos identifiants">
    Choisissez l'une des méthodes suivantes. Le port utilisé pour `--callback-port` peut être n'importe quel port disponible. Il doit correspondre à l'URI de redirection que vous avez enregistré à l'étape précédente.

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
  * Vous pouvez définir le secret client uniquement lorsque vous ajoutez le serveur. Lorsque vous vous authentifiez avec `claude mcp login` ou à partir de `/mcp`, Claude Code utilise le secret stocké et ne demande pas de secret ou ne lit pas `MCP_CLIENT_SECRET`
  * Pour ajouter ou modifier le secret ultérieurement, supprimez le serveur avec `claude mcp remove <name>`, puis ajoutez-le à nouveau avec `--client-secret` et le même `--scope`
  * Si le serveur utilise un client OAuth public sans secret, utilisez uniquement `--client-id` sans `--client-secret`
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

Si le serveur retourne ultérieurement un 403 `insufficient_scope` pour un appel d'outil, l'appel échoue avec un message [`needs additional permissions`](/docs/fr/errors#mcp-server-needs-you-to-sign-in-again) qui nomme la portée que le serveur demande. Le serveur s'affiche comme nécessitant une authentification dans `/mcp`.

Si cette portée ne figure pas dans votre `oauth.scopes` épinglé, ajoutez-la, puis exécutez `/mcp` et authentifiez le serveur à nouveau. Claude Code demande les portées épinglées plutôt que la portée que le serveur a nommée, donc si vous vous authentifiez à nouveau sans l'ajouter, le jeton que vous obtenez ne l'a toujours pas.

<h3 id="use-dynamic-headers-for-custom-authentication">
  Utiliser des en-têtes dynamiques pour l'authentification personnalisée
</h3>

Si votre serveur MCP utilise un schéma d'authentification autre que OAuth, tel que Kerberos, jetons de courte durée ou un SSO interne, utilisez `headersHelper` pour générer des en-têtes de requête au moment de la connexion. Claude Code exécute la commande et fusionne sa sortie dans les en-têtes de connexion.

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
* Claude Code exécute la commande dans un shell et abandonne après 10 secondes
* Claude Code choisit le répertoire de travail de la commande en [fonction de l'endroit où vous avez configuré le serveur](#where-the-helper-runs), donc fournissez le script comme chemin absolu ou mettez-le sur `PATH`
* Les en-têtes dynamiques remplacent tous les `headers` statiques portant le même nom

Claude Code exécute l'assistant à nouveau à chaque connexion, au démarrage de la session et à la reconnexion, une fois que la [règle de confiance pour les serveurs de portée projet et locale](#trust-a-folder-before-its-headershelper-runs) le permet. Il ne met pas en cache le résultat, donc votre script est responsable de toute réutilisation de jetons.

Si un appel d'outil retourne `401 Unauthorized` ou `403 Forbidden`, Claude Code réexécute automatiquement l'assistant selon la même règle, se reconnecte avec les en-têtes frais et réessaie l'appel une fois. Claude Code marque le serveur comme nécessitant une authentification dans `/mcp` uniquement si cette nouvelle tentative échoue également.

Lorsque la sortie de l'assistant inclut un en-tête `Authorization`, Claude Code utilise cet identifiant comme authentification du serveur et ne revient pas à OAuth pour le serveur.

Si le serveur rejette l'identifiant de l'assistant lors de la connexion, Claude Code signale la connexion comme échouée plutôt que de marquer le serveur comme nécessitant une authentification. Corrigez l'identifiant que votre assistant retourne, puis reconnectez-vous à partir de `/mcp` pour réexécuter l'assistant.

Claude Code définit ces variables d'environnement lors de l'exécution de l'assistant :

| Variable                      | Valeur                                                                                                                      |
| :---------------------------- | :-------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_MCP_SERVER_NAME` | le nom du serveur MCP                                                                                                       |
| `CLAUDE_CODE_MCP_SERVER_URL`  | l'URL du serveur MCP                                                                                                        |
| `CLAUDE_PLUGIN_ROOT`          | le répertoire racine du plugin. Défini uniquement lorsqu'un [plugin](/docs/fr/plugins/components#mcp-servers) fournit le serveur |

Utilisez-les pour écrire un script d'assistant unique qui sert plusieurs serveurs MCP.

Un `headersHelper` fourni par un plugin ne peut pas référencer les valeurs [`${user_config.*}`](/docs/fr/plugins/manifest-reference#user-configuration) du plugin, car la commande s'exécute via un shell. Claude Code signale le serveur comme mal configuré avec une [erreur](/docs/fr/errors#plugin-command-references-user-config) et ne substitue pas la valeur. Mettez `${user_config.KEY}` dans le champ `headers` du serveur à la place, qui n'est pas analysé par shell, ou faites en sorte que le script d'assistant lise la valeur à partir d'un fichier de configuration. Avant la v2.1.207, `headersHelper` substituait les valeurs `${user_config.*}`.

<h4 id="where-the-helper-runs">
  Où l'assistant s'exécute
</h4>

Claude Code choisit le répertoire de travail de la commande `headersHelper` à partir de la configuration qui déclare le serveur. Un `cd` que Claude exécute dans Bash ne le déplace pas, et [`/cd`](/docs/fr/permissions#move-the-session-to-another-directory) ne le déplace que pour les serveurs qui s'exécutent à partir du répertoire de travail principal de la session. Chaque ligne ci-dessous donne le répertoire contre lequel un chemin relatif dans votre commande `headersHelper` se résout.

| Où vous avez configuré le serveur                                                                                                                                                                                   | Répertoire de travail                                                                                       |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------- |
| Un [plugin](/docs/fr/plugins/components#mcp-servers)                                                                                                                                                                     | Le répertoire racine du plugin. Nécessite Claude Code v2.1.195 ou ultérieur                                 |
| Un `.mcp.json` de projet ou un serveur de [portée locale](#local-scope)                                                                                                                                             | Le répertoire du projet dans lequel le serveur est déclaré                                                  |
| Un fichier agent dans votre projet, un serveur de l'option `mcpServers` du SDK ou de la méthode `setMcpServers()`, ou [`--mcp-config`](/docs/fr/cli-reference)                                                           | Le [répertoire de travail principal](/docs/fr/permissions#working-directories) de la session                     |
| [Portée utilisateur](#user-scope), [MCP géré](/docs/fr/managed-mcp), un [connecteur claude.ai](#use-mcp-servers-from-claude-ai), ou un fichier agent en dehors de votre projet, y compris un d'un répertoire `--add-dir` | Votre répertoire de configuration, `~/.claude` sauf si vous avez défini [`CLAUDE_CONFIG_DIR`](/docs/fr/env-vars) |

Avant la v2.1.238, Claude Code exécutait également les assistants des serveurs de portée utilisateur, gérés et connecteur claude.ai, et des fichiers agent en dehors de votre projet, à partir du répertoire à partir duquel vous l'aviez démarré.

<h4 id="which-variables-a-helper-can-read">
  Quelles variables un assistant peut lire
</h4>

Un `headersHelper` qu'un référentiel ou un plugin fournit est une commande que vous n'avez pas écrite, donc Claude Code l'exécute sans les variables d'identifiant de votre environnement, telles que `ANTHROPIC_API_KEY`. L'endroit où vous avez configuré le serveur détermine si cela s'applique :

* **Supprimées** : un serveur dans un `.mcp.json` de projet ou dans un plugin, et un serveur en ligne dans un fichier agent de votre projet ou d'un répertoire `--add-dir`
* **Non supprimées** : un serveur à [portée utilisateur](#user-scope) ou [portée locale](#local-scope), dans [MCP géré](/docs/fr/managed-mcp), à partir d'un [connecteur claude.ai](#use-mcp-servers-from-claude-ai), ou fourni par le SDK ou [`--mcp-config`](/docs/fr/cli-reference), et un serveur en ligne dans un fichier agent de `~/.claude/agents/`, à partir des paramètres gérés, ou passé avec `--agents`

À part les variables `GIT_CONFIG_KEY_<n>` de Git, Claude Code supprime chaque variable de votre environnement dont le nom ressemble à un identifiant, comme un nom avec `TOKEN`, `SECRET`, `PASSWORD`, `KEY` ou `AUTH` dedans en l'une ou l'autre casse, donc `ANTHROPIC_API_KEY` et `MY_REGISTRY_TOKEN` sont tous deux supprimés. Claude Code supprime également une liste fixe de variables d'identifiant dont les noms ne suivent pas ce modèle, telles que `ANTHROPIC_CUSTOM_HEADERS`.

Lorsque cela s'applique à votre assistant, faites en sorte que le script lise son identifiant à partir d'un fichier ou d'un magasin d'identifiants. Si l'`url` du serveur [développe l'une de ces variables](#environment-variable-expansion-in-mcp-json), la valeur `CLAUDE_CODE_MCP_SERVER_URL` que l'assistant reçoit a cette partie remplacée par `REDACTED` également.

<h4 id="trust-a-folder-before-its-headershelper-runs">
  Faire confiance à un dossier avant que son headersHelper s'exécute
</h4>

Claude Code exécute un `headersHelper` comme une commande shell arbitraire. Pour un serveur dans un `.mcp.json` de projet ou à [portée locale](#local-scope), il exécute l'assistant uniquement après que vous ayez accepté la [boîte de dialogue de confiance](/docs/fr/permissions#project-allow-rules-and-workspace-trust) pour le répertoire du projet dans lequel le serveur est déclaré. Avant la v2.1.238, une session `claude -p` ou SDK exécutait ces assistants sans vérifier la confiance, et une session interactive les exécutait une fois que vous aviez fait confiance à un dossier parent.

* **Confiance qui ne compte pas** : la confiance d'un dossier parent, et la confiance automatique qu'une session `claude -p` ou SDK obtient pour les [hooks dans les fichiers de paramètres](/docs/fr/permissions#what-runs-before-you-trust-a-folder)
* **Jusqu'à ce que vous fassiez confiance au dossier** : Claude Code connecte le serveur avec ses `headers` statiques seuls. Dans une session `claude -p` ou SDK, il imprime également une ligne [`headersHelper not run`](/docs/fr/errors#headershelper-not-run) par serveur sur stderr, vous indiquant comment accorder la confiance.
* **Confiance sans boîte de dialogue** : définissez `projects["<path>"].hasTrustDialogAccepted` à `true` dans `~/.claude.json`. `<path>` est le dossier sur lequel [Règles d'autorisation de projet et confiance de l'espace de travail](/docs/fr/permissions#project-allow-rules-and-workspace-trust) dit que Claude Code base la confiance.

Claude Code applique la même règle à un serveur déclaré en ligne dans un [fichier agent](/docs/fr/sub-agents#scope-mcp-servers-to-a-subagent), en vérifiant d'où ce fichier agent provient : votre projet, pour un fichier dans son répertoire `.claude/agents/`, ou un répertoire `--add-dir`. Jusqu'à ce que vous [fassiez confiance à ce projet ou répertoire lui-même](/docs/fr/permissions#what-runs-before-you-trust-a-folder), Claude Code ne charge pas le serveur du tout, donc son assistant ne s'exécute jamais non plus.

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
  Utiliser les serveurs MCP depuis claude.ai
</h2>

Si vous vous êtes connecté à Claude Code avec un compte [claude.ai](https://claude.ai), les serveurs MCP que vous avez ajoutés dans claude.ai, connus sous le nom de [connecteurs](https://claude.com/docs/connectors), sont automatiquement disponibles dans Claude Code :

<Steps>
  <Step title="Configurer les serveurs MCP dans claude.ai">
    Ajoutez des serveurs à [claude.ai/customize/connectors](https://claude.ai/customize/connectors). Sur les plans Team et Enterprise, seuls les administrateurs peuvent ajouter des serveurs.
  </Step>

  <Step title="Authentifier le serveur MCP">
    Complétez les étapes d'authentification requises dans claude.ai.
  </Step>

  <Step title="Afficher et gérer les serveurs dans Claude Code">
    Dans Claude Code, utilisez la commande :

    ```text wrap theme={null}
    /mcp
    ```

    Les serveurs de claude.ai apparaissent dans la liste avec des indicateurs montrant qu'ils proviennent de claude.ai.
  </Step>
</Steps>

Claude Code marque un connecteur comme `managed` dans `/mcp` et dans le gestionnaire [`/plugin`](/docs/fr/plugins/install) lorsque votre organisation gère son authentification dans claude.ai. Le statut managed ne change pas la façon dont Claude Code se connecte au connecteur ou applique les [contrôles d'outils](#organization-controls-on-connector-tools) de votre organisation.

Les connecteurs auxquels vous ne vous êtes jamais connecté sont réduits derrière une ligne `Show unused connectors` à la fin de la section claude.ai, de sorte qu'une liste fournie par l'organisation ne remplit pas le panneau. Sélectionnez la ligne pour les développer. Un connecteur auquel vous vous êtes connecté avant reste visible même s'il a actuellement besoin d'une nouvelle authentification.

Les connecteurs de claude.ai ne sont récupérés que lorsque votre [méthode d'authentification](/docs/fr/authentication#authentication-precedence) active est une connexion par abonnement claude.ai. Ils ne sont pas chargés, même si vous avez précédemment exécuté `/login`, lorsque :

* `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, ou `apiKeyHelper` est actif
* Un fournisseur tiers tel qu'Amazon Bedrock ou Agent Platform de Google Cloud est actif
* `ANTHROPIC_PROFILE`, les variables de fédération, ou un [profil Anthropic](/docs/fr/authentication#anthropic-profiles-and-federation-credentials) actif fournit les identifiants
* `CLAUDE_CODE_OAUTH_TOKEN` contient un jeton de [`claude setup-token`](/docs/fr/authentication#generate-a-long-lived-token), qui ne peut faire que des demandes de modèle

Si `/mcp` ne liste pas un connecteur que vous avez ajouté, exécutez `/status` pour confirmer quelle méthode d'authentification est active. Désinscrivez cette variable d'environnement, supprimez le paramètre `apiKeyHelper`, ou [désactivez le profil](/docs/fr/authentication#anthropic-profiles-and-federation-credentials), puis exécutez `/login` pour sélectionner votre compte claude.ai.

Si un problème réseau temporaire empêche la liste des connecteurs de se charger au démarrage de votre session, Claude Code réessaie la récupération jusqu'à trois fois en arrière-plan, et les connecteurs apparaissent une fois qu'une tentative réussit. S'ils n'ont toujours pas apparu, redémarrez Claude Code pour récupérer la liste à nouveau.

Si `/mcp` affiche un connecteur comme `connected · session token rejected`, ou sa vue détaillée affiche [`claude.ai rejected the session token`](/docs/fr/errors#claude-ai-rejected-the-session-token), claude.ai a rejeté le jeton de votre connexion Claude Code, généralement parce que la connexion a expiré et n'a pas pu être actualisée. Autoriser à nouveau le connecteur n'efface pas cet état, car l'autorisation propre du connecteur dans claude.ai n'est pas ce qui a été rejeté. Pour l'effacer :

1. Exécutez `/login` pour vous reconnecter.
2. Reconnectez le connecteur depuis `/mcp`.

Avant la v2.1.222, Claude Code marquait les connecteurs comme ayant besoin d'authentification, et les autoriser ne le résolvait pas.

Un serveur que vous avez ajouté dans Claude Code prend [précédence](#scope-hierarchy-and-precedence) sur un connecteur claude.ai qui pointe vers la même URL. Quand cela se produit, `/mcp` liste le connecteur comme caché et montre comment supprimer le doublon si vous préférez utiliser le connecteur.

Certains connecteurs hébergés par Anthropic, tels que Microsoft 365, Gmail et Google Calendar, ne supportent pas OAuth local depuis Claude Code car le fournisseur d'identité en amont n'accepte que l'URL de redirection que claude.ai a enregistrée. Quand un serveur que vous avez ajouté avec `claude mcp add` ou dans `.mcp.json` pointe vers l'un de ces hôtes et que vous vous y connectez depuis `/mcp` ou avec `claude mcp login`, Claude Code affiche [`is Anthropic-hosted and doesn't support local OAuth`](/docs/fr/errors#anthropic-hosted-and-doesnt-support-local-oauth), vous dirigeant pour connecter le service à [claude.ai/customize/connectors](https://claude.ai/customize/connectors) à la place.

Après avoir supprimé votre entrée avec `claude mcp remove <name>` et connecté le service sur claude.ai, le connecteur apparaît dans Claude Code automatiquement.

<h3 id="how-connectors-reach-claude-code">
  Comment les connecteurs atteignent Claude Code
</h3>

Les paramètres qui gouvernent un connecteur claude.ai dépendent de l'endroit où votre session s'exécute, car seules certaines sessions récupèrent les connecteurs de claude.ai elles-mêmes. Chaque ligne ci-dessous nomme comment les connecteurs arrivent dans un type de session et ce qui les contrôle là. Les [sessions WSL](/docs/fr/desktop-wsl#what-works-in-a-wsl-session) de l'application de bureau n'ont pas de ligne car les connecteurs ne sont pas encore disponibles dans celles-ci.

| Où la session s'exécute                                                                                                   | Comment les connecteurs arrivent               | Ce qui les gouverne                                                                                                                                                                                                                              |
| :------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Sessions Terminal, [VS Code](/docs/fr/vs-code), [JetBrains](/docs/fr/jetbrains), et [Agent SDK](/docs/fr/agent-sdk/claude-code-features) | Claude Code les récupère depuis claude.ai      | Les paramètres de cette section et [configuration MCP gérée](/docs/fr/managed-mcp)                                                                                                                                                                    |
| [Sessions cloud](/docs/fr/claude-code-on-the-web)                                                                              | L'hôte distant les transmet                    | Vos paramètres d'organisation claude.ai, plus les paramètres de [liste blanche et liste noire](/docs/fr/managed-mcp#policy-based-control-with-allowlists-and-denylists) qui atteignent la session et tout `managed-mcp.json` sur l'hôte qui l'exécute |
| Les sessions locales et SSH de l'[application de bureau](/docs/fr/desktop)                                                     | L'application de bureau les livre en processus | Les entrées `blocked` dans les [contrôles d'outils de connecteur](#organization-controls-on-connector-tools) de votre organisation                                                                                                               |

[`disableClaudeAiConnectors`](#disable-claude-ai-connectors), `ENABLE_CLAUDEAI_MCP_SERVERS`, et [`allowAllClaudeAiMcps`](/docs/fr/settings-reference#allowallclaudeaimcps) agissent uniquement sur la première ligne, les connecteurs que Claude Code récupère lui-même. Les deux autres lignes en diffèrent de ces façons :

* **Sessions cloud** : les entrées `allowedMcpServers` et `deniedMcpServers` qui atteignent la session, par exemple via les [paramètres gérés par le serveur](/docs/fr/server-managed-settings), filtrent également les connecteurs livrés. Le proxy de la session réécrit l'URL de chaque connecteur, donc un motif `serverUrl` écrit pour l'URL propre du connecteur ne le correspond pas. Pour admettre les connecteurs livrés aux côtés d'une liste blanche d'URL dans un environnement auto-hébergé, ajoutez les entrées `serverUrl` listées sous [Le trafic des connecteurs quitte votre réseau](/docs/fr/self-hosted-environments-deploy#connector-traffic-leaves-your-network). Claude Code supprime les connecteurs livrés quand un `managed-mcp.json` est présent sur l'hôte qui exécute la session, comme un [hôte de runner auto-hébergé](/docs/fr/self-hosted-environments-configuration#mcp-servers), que vous ayez défini `allowAllClaudeAiMcps` ou non.
* **Sessions locales et SSH de l'application de bureau** : l'application de bureau enregistre les connecteurs en tant que serveurs `type: "sdk"` en processus, et aucun paramètre MCP ou `managed-mcp.json` ne les atteint. Un utilisateur garde un connecteur hors de ses propres sessions en le déconnectant à [claude.ai/customize/connectors](https://claude.ai/customize/connectors). Une organisation bloque les [outils](#organization-controls-on-connector-tools) d'un connecteur ou désactive entièrement [Claude Code dans l'application de bureau](/docs/fr/desktop#admin-console-controls).

<h3 id="organization-controls-on-connector-tools">
  Contrôles d'organisation sur les outils de connecteur
</h3>

Votre organisation peut définir des contrôles par outil sur les [connecteurs claude.ai](https://claude.com/docs/connectors). Claude Code lit ces paramètres au démarrage et les applique localement, sauf dans les [sessions locales et SSH](#how-connectors-reach-claude-code) de l'application de bureau. Là, l'application de bureau retient les outils `blocked` avant de livrer un connecteur, et le paramètre `ask` n'atteint pas Claude Code, donc il applique les [règles de permission](/docs/fr/permissions) ordinaires de la session à ces outils au lieu de demander à chaque appel. Dans les sessions où Claude Code récupère les connecteurs lui-même, exécutez `/mcp` pour voir quel paramètre s'applique à chaque outil sur un connecteur.

* **Outil défini sur `ask`** : Claude Code demande à chaque appel avec la raison `Your organization requires approval for this tool`. L'invite apparaît même dans les modes de permission `acceptEdits`, `auto`, et `bypassPermissions` [modes de permission](/docs/fr/permissions#permission-modes), et n'offre jamais une option pour mémoriser votre choix. Les [règles Allow](/docs/fr/permissions) qui correspondent à l'outil ne sautent pas non plus l'invite. En mode `dontAsk`, qui ne demande jamais, Claude Code refuse l'appel à la place.
* **Outil défini sur `blocked`** : Claude Code filtre l'outil avant que Claude ne le voie, donc il n'apparaît jamais dans la liste des outils. L'application de bureau et le chat claude.ai appliquent le même paramètre `blocked`, donc Claude ne peut pas utiliser l'outil là non plus, et vous ne pouvez pas retenir un outil des sessions de l'application de bureau tout en le gardant disponible dans le chat. L'application de bureau saute un connecteur dont tous les outils sont bloqués.

<h3 id="disable-claude-ai-connectors">
  Désactiver les connecteurs claude.ai
</h3>

Claude Code applique [`disableClaudeAiConnectors`](/docs/fr/settings-reference#disableclaudeaiconnectors) uniquement aux connecteurs qu'il [récupère lui-même](#how-connectors-reach-claude-code), pas aux connecteurs qu'un hôte cloud ou l'application de bureau livre. Pour désactiver les connecteurs qu'il récupère, définissez le paramètre sur `true` dans n'importe quelle portée de paramètres :

```json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

Ce paramètre utilise la sémantique any-source-true : `true` dans n'importe quelle source de paramètres prend précédence. Un `.claude/settings.json` de projet enregistré peut exclure un référentiel des connecteurs que Claude Code récupère lui-même, mais un `false` au niveau du projet ne peut pas réactiver les connecteurs qu'un `true` au niveau utilisateur ou politique a désactivés. Les serveurs passés explicitement via `--mcp-config` ne sont pas affectés.

Vous pouvez également définir la variable d'environnement `ENABLE_CLAUDEAI_MCP_SERVERS` sur `false`, qui a le même effet pour la session shell actuelle :

```bash theme={null}
ENABLE_CLAUDEAI_MCP_SERVERS=false claude
```

Pour bloquer les connecteurs claude.ai individuels au lieu de tous les bloquer, ajoutez-les à [`deniedMcpServers`](/docs/fr/managed-mcp) par nom ou par motif d'URL. Par exemple, une entrée `serverName` de `"claude.ai Slack"` bloque le connecteur Slack. Vous pouvez également exécuter `/mcp` pour basculer n'importe quel connecteur que Claude Code récupère activé ou désactivé pour le projet actuel uniquement.

<h2 id="use-claude-code-as-an-mcp-server">
  Utiliser Claude Code comme serveur MCP
</h2>

Vous pouvez utiliser Claude Code lui-même comme serveur MCP que d'autres applications peuvent connecter :

```bash theme={null}
# Démarrer Claude en tant que serveur MCP stdio
claude mcp serve
```

La commande n'affiche rien au démarrage. Un serveur MCP stdio communique via stdin et stdout, donc un terminal silencieux et bloqué signifie que le serveur est en cours d'exécution et attend qu'un client se connecte.

Vous pouvez utiliser ceci dans Claude Desktop en ajoutant cette configuration à claude\_desktop\_config.json :

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
  **Configuration du chemin exécutable** : le champ `command` doit référencer l'exécutable Claude Code. Si la commande `claude` ne se trouve pas dans le PATH de votre système, vous devrez spécifier le chemin complet vers l'exécutable.

  Pour trouver le chemin complet :

  ```bash theme={null}
  which claude
  ```

  Utilisez ensuite le chemin complet dans votre configuration :

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

  Sans le chemin exécutable correct, vous rencontrerez des erreurs comme `spawn claude ENOENT`.
</Warning>

<Tip>
  Conseils :

  * Dans Claude Desktop, essayez de demander à Claude de lire les fichiers d'un répertoire, de faire des modifications, et bien plus.
  * Ce serveur MCP expose uniquement les outils de Claude Code à votre client MCP, donc votre propre client est responsable de l'implémentation de la confirmation de l'utilisateur pour les appels d'outils individuels.
</Tip>

<h2 id="mcp-output-limits-and-warnings">
  Limites de sortie MCP et avertissements
</h2>

Lorsque les outils MCP produisent des sorties volumineuses, Claude Code aide à gérer l'utilisation des tokens pour éviter de surcharger le contexte de votre conversation :

* **Seuil d'avertissement de sortie** : Claude Code affiche un avertissement lorsque la sortie de tout outil MCP dépasse 10 000 tokens
* **Limite configurable** : vous pouvez ajuster le nombre maximum de tokens de sortie MCP autorisés à l'aide de la variable d'environnement `MAX_MCP_OUTPUT_TOKENS`
* **Limite par défaut** : le maximum par défaut est de 25 000 tokens
* **Portée** : la variable d'environnement s'applique aux outils qui ne déclarent pas leur propre limite. Les outils qui définissent [`anthropic/maxResultSizeChars`](#raise-the-limit-for-a-specific-tool) utilisent cette valeur à la place pour le contenu textuel, indépendamment de la valeur définie pour `MAX_MCP_OUTPUT_TOKENS`. Les outils qui retournent des données d'image restent soumis à `MAX_MCP_OUTPUT_TOKENS`
* **Au-delà de la limite** : lorsqu'un résultat sans contenu d'image dépasse la limite, Claude Code l'enregistre dans un fichier et le remplace dans la conversation par un message qui indique le chemin du fichier, de sorte que Claude lit le fichier lorsqu'il a besoin du contenu. Le fichier se trouve dans le répertoire `tool-results` de la session sous [`~/.claude/projects/`](/docs/fr/claude-directory#cleaned-up-automatically).

Pour augmenter la limite pour les outils qui produisent des sorties volumineuses :

```bash theme={null}
export MAX_MCP_OUTPUT_TOKENS=50000
claude
```

<h3 id="raise-the-limit-for-a-specific-tool">
  Augmenter la limite pour un outil spécifique
</h3>

Si vous créez un serveur MCP, vous pouvez permettre aux outils individuels de retourner des résultats plus volumineux que le seuil de persistance sur disque par défaut en définissant `_meta["anthropic/maxResultSizeChars"]` dans l'entrée de réponse `tools/list` de l'outil. Claude Code augmente le seuil de cet outil à la valeur annotée, jusqu'à un plafond maximal de 500 000 caractères.

Ceci est utile pour les outils qui retournent des sorties intrinsèquement volumineuses mais nécessaires, telles que les schémas de base de données ou les arbres de fichiers complets. Sans l'annotation, les résultats qui dépassent le seuil par défaut sont persistés sur disque et remplacés par une référence de fichier dans la conversation.

```json theme={null}
{
  "name": "get_schema",
  "description": "Returns the full database schema",
  "_meta": {
    "anthropic/maxResultSizeChars": 200000
  }
}
```

L'annotation s'applique indépendamment de `MAX_MCP_OUTPUT_TOKENS` pour le contenu textuel, de sorte que les utilisateurs n'ont pas besoin d'augmenter la variable d'environnement pour les outils qui la déclarent. Les outils qui retournent des données d'image restent soumis à la limite de tokens.

<Warning>
  Si vous rencontrez fréquemment des avertissements de sortie avec des serveurs MCP spécifiques que vous ne contrôlez pas, envisagez d'augmenter la limite `MAX_MCP_OUTPUT_TOKENS`. Vous pouvez également demander à l'auteur du serveur d'ajouter l'annotation `anthropic/maxResultSizeChars` ou de paginer ses réponses. L'annotation n'a aucun effet sur les outils qui retournent du contenu d'image ; pour ceux-ci, augmenter `MAX_MCP_OUTPUT_TOKENS` est la seule option.
</Warning>

<h2 id="tool-input-schemas-with-a-root-level-combinator">
  Schémas d'entrée d'outil avec un combinateur au niveau racine
</h2>

Certains serveurs MCP déclarent le schéma d'entrée d'un outil comme une union JSON Schema, avec `anyOf`, `oneOf`, ou `allOf` au niveau supérieur du schéma. L'API Claude n'accepte pas ces mots-clés à la racine du schéma. Elle accepte les combinateurs imbriqués dans `properties`, que Claude Code envoie inchangés.

Les outils avec un combinateur au niveau racine restent disponibles. Avant d'envoyer l'outil à l'API, Claude Code aplatit le schéma en un seul objet et ajoute une phrase au début de la description de l'outil qui indique à Claude quels groupes de paramètres vont ensemble :

* `allOf` : les propriétés de chaque branche sont fusionnées, et la liste `required` de chaque branche s'applique toujours
* `anyOf` et `oneOf` : les propriétés de chaque branche sont fusionnées, et la liste `required` de chaque branche est décrite dans la description de l'outil au lieu d'être appliquée par le schéma

Votre serveur reçoit les arguments que Claude a choisis, alors continuez à valider la combinaison côté serveur.

Quand Claude Code ne peut pas produire un schéma que l'API accepte, ou sur un déploiement qui ne reçoit pas la configuration distante qui active la réécriture, il ignore cet outil, enregistre la raison dans le journal du serveur, et laisse les autres outils du serveur disponibles. Les versions antérieures à v2.1.195 ignorent chaque outil dont le schéma d'entrée a un `anyOf`, `oneOf`, ou `allOf` au niveau racine.

<h2 id="tools-with-invalid-input-schemas">
  Outils avec des schémas d'entrée invalides
</h2>

L'API Claude vérifie le schéma d'entrée de chaque outil dans une requête et rejette la requête entière lorsqu'un schéma échoue, donc un seul outil MCP avec un schéma malformé ferait échouer chaque requête qui l'inclut avec une erreur 400. Claude Code exécute deux des vérifications de l'API lui-même lorsqu'il charge les outils d'un serveur et exclut chaque outil qui échouerait à ces vérifications, de sorte que les autres outils du serveur continuent de fonctionner :

* Les noms de propriétés au niveau supérieur doivent faire entre 1 et 64 caractères et utiliser uniquement des lettres ASCII et des chiffres, `_`, `.` et `-`
* Le schéma doit être valide par rapport au méta-schéma JSON Schema draft 2020-12. Claude Code applique cette vérification aux schémas qui ne déclarent pas de `$schema` et aux schémas qui déclarent draft 2020-12. Un schéma qui déclare un autre dialecte ignore cette vérification, bien que la vérification du nom de propriété ci-dessus s'applique toujours

Claude Code exécute les vérifications après la [réécriture du combinateur au niveau racine](#tool-input-schemas-with-a-root-level-combinator), sur le schéma qu'il enverrait réellement.

Lorsque Claude Code exclut un outil, il enregistre la raison dans le journal du serveur et indique à Claude quels outils il a exclus et pourquoi, afin que vous puissiez demander à Claude pourquoi un outil est manquant. Si vous corrigez le schéma sur le serveur, l'outil réapparaît la prochaine fois que Claude Code charge les outils du serveur.

Claude Code active l'exclusion via un drapeau de fonctionnalité qu'il récupère auprès d'Anthropic. Sur un [déploiement où la récupération de drapeaux est désactivée](/docs/fr/env-vars#features-that-need-feature-flag-fetching), ou sur une machine dont les drapeaux n'ont jamais été reçus, comme une machine isolée du réseau, Claude Code exécute toujours les vérifications et enregistre dans le journal du serveur quel outil serait rejeté, mais envoie le schéma de l'outil à l'API de toute façon. L'API rejette une requête qui inclut ce schéma avec [une erreur 400 nommant l'outil par sa position](/docs/fr/errors#tool-input-schema-is-invalid). Avant la v2.1.216, aucun déploiement n'exécutait ces vérifications.

La [gestion du combinateur au niveau racine](#tool-input-schemas-with-a-root-level-combinator) est séparée et conserve son propre comportement lorsque la récupération de drapeaux est désactivée ou que les drapeaux n'ont jamais été reçus.

<h2 id="require-approval-for-a-specific-tool">
  Exiger une approbation pour un outil spécifique
</h2>

Si vous créez un serveur MCP, vous pouvez marquer un outil comme nécessitant une approbation explicite à chaque appel en définissant `_meta["anthropic/requiresUserInteraction"]` sur `true` dans l'entrée de réponse `tools/list` de l'outil. La valeur doit être le booléen JSON `true` ; toute autre valeur est ignorée.

Claude Code affiche l'invite de permission de cet outil à chaque appel, même dans les [modes de permission](/docs/fr/permissions#permission-modes) `acceptEdits`, `auto` et `bypassPermissions`, et n'offre pas d'option « ne plus demander » pour celui-ci. Les [règles d'autorisation](/docs/fr/permissions#permission-rule-syntax) qui correspondent à l'outil ne contournent pas non plus l'invite. En mode `dontAsk`, qui ne demande jamais, Claude Code refuse l'appel à la place.

L'invite doit atteindre une personne. En mode non interactif avec [`--permission-prompt-tool`](/docs/fr/cli-reference#cli-flags), un résultat `allow` de l'outil d'invite de permission pour un outil signalé est converti en refus avec le message `MCP tool requires user interaction; not supported via --permission-prompt-tool`. Le callback [`canUseTool`](/docs/fr/agent-sdk/permissions) du SDK Agent reçoit bien ces appels et peut les approuver, car votre application SDK est censée les montrer à un utilisateur.

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

L'annotation `anthropic/requiresUserInteraction` nécessite Claude Code v2.1.199 ou version ultérieure. Les versions antérieures l'ignorent et appliquent le flux de permission standard.

Certaines surfaces, comme [Remote Control](/docs/fr/remote-control) et les applications construites sur le [SDK Agent](/docs/fr/agent-sdk/overview), vous permettent normalement d'approuver les appels d'outils en un seul geste. Pour un outil marqué avec cette annotation, Claude Code retient l'action en un geste et affiche l'invite de permission complète de l'outil à la place, de sorte que l'approbation provient toujours d'une personne répondant à l'invite plutôt que d'un geste.

Claude Code retient l'approbation en un geste de la même manière pour toute demande de permission que seul le dialogue du terminal peut afficher complètement, comme celle qui porte un avertissement de sécurité ou une option d'autorisation permanente que la surface distante ne peut pas afficher. Vous répondez à cette demande dans le dialogue du terminal plutôt que depuis Remote Control. Nécessite Claude Code v2.1.214 ou version ultérieure.

<h2 id="respond-to-mcp-elicitation-requests">
  Répondre aux demandes d'élicitation MCP
</h2>

Les serveurs MCP peuvent vous demander une entrée structurée au cours d'une tâche en utilisant l'élicitation. Lorsqu'un serveur a besoin d'informations qu'il ne peut pas obtenir seul, Claude Code affiche un dialogue interactif et transmet votre réponse au serveur. Aucune configuration n'est requise de votre côté : les dialogues d'élicitation apparaissent automatiquement lorsqu'un serveur les demande.

Les serveurs peuvent demander une entrée de deux façons :

* **Mode formulaire** : Claude Code affiche un dialogue avec des champs de formulaire définis par le serveur (par exemple, une invite de nom d'utilisateur et de mot de passe). Remplissez les champs et soumettez.
* **Mode URL** : Claude Code ouvre une URL de navigateur pour l'authentification ou l'approbation. Complétez le flux dans le navigateur, puis confirmez dans l'interface de ligne de commande.

En mode URL, Claude Code transmet l'URL en tant qu'argument de ligne de commande au gestionnaire d'URL de votre système, et limite la longueur de cet argument. Lorsque l'URL, une fois échappée pour la ligne de commande, dépasse cette limite, vous ne pouvez que refuser la demande. Chaque caractère qui doit être échappé, tel que `%` ou `&`, compte quatre fois vers la limite : son propre caractère plus trois caractères d'échappement. Une URL sans aucun d'entre eux atteint la limite à environ 8 000 caractères. Une URL construite largement à partir d'échappements de pourcentage, où chaque tiers caractère est un `%`, l'atteint à environ 4 000.

Pour répondre automatiquement aux demandes d'élicitation sans afficher de dialogue, utilisez le hook [`Elicitation`](/docs/fr/hooks#elicitation).

Si vous créez un serveur MCP qui utilise l'élicitation, consultez la [spécification d'élicitation MCP](https://modelcontextprotocol.io/docs/learn/client-concepts#elicitation) pour les détails du protocole et les exemples de schéma.

<h2 id="use-mcp-resources">
  Utiliser les ressources MCP
</h2>

Les serveurs MCP peuvent exposer des ressources que vous pouvez référencer en utilisant des mentions @, de la même manière que vous référencez des fichiers.

<h3 id="reference-mcp-resources">
  Référencer les ressources MCP
</h3>

<Steps>
  <Step title="Lister les ressources disponibles">
    Tapez `@` dans votre invite pour voir les ressources disponibles de tous les serveurs MCP connectés. Les ressources apparaissent aux côtés des fichiers dans le menu d'autocomplétion.
  </Step>

  <Step title="Référencer une ressource spécifique">
    Utilisez le format `@server:protocol://resource/path` pour référencer une ressource :

    ```text wrap theme={null}
    Can you analyze @github:issue://123 and suggest a fix?
    ```

    ```text wrap theme={null}
    Please review the API documentation at @docs:file://api/authentication
    ```
  </Step>

  <Step title="Références de ressources multiples">
    Vous pouvez référencer plusieurs ressources dans une seule invite :

    ```text wrap theme={null}
    Compare @postgres:schema://users with @docs:file://database/user-model
    ```
  </Step>
</Steps>

<Tip>
  Conseils :

  * Les ressources sont automatiquement récupérées et incluses en tant que pièces jointes lorsqu'elles sont référencées
  * Les chemins de ressources sont recherchables de manière floue dans l'autocomplétion de mention @
  * Claude Code fournit automatiquement des outils pour lister et lire les ressources MCP lorsque les serveurs les supportent
  * Les ressources peuvent contenir n'importe quel type de contenu fourni par le serveur MCP (texte, JSON, données structurées, etc.)
</Tip>

<h2 id="scale-with-mcp-tool-search">
  Mettre à l'échelle avec la recherche d'outils MCP
</h2>

La recherche d'outils maintient l'utilisation du contexte MCP faible en reportant les définitions d'outils jusqu'à ce que Claude en ait besoin. Seuls les noms d'outils et les instructions du serveur se chargent au démarrage de la session, donc l'ajout de plus de serveurs MCP a un impact minimal sur votre fenêtre de contexte. Claude Code n'impose pas de limite fixe d'outils par serveur ; la limite pratique est votre budget de fenêtre de contexte.

<Note>
  La recherche d'outils n'est pas prise en charge sur les [déploiements Microsoft Foundry hébergés sur Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options), qui la rejettent côté serveur : Claude Code détecte le rejet et charge les outils MCP en amont pour ce déploiement à la place. [`ENABLE_TOOL_SEARCH`](#configure-tool-search) ne peut pas contourner cela, puisque le rejet provient du déploiement lui-même.
</Note>

<h3 id="for-mcp-server-authors">
  Pour les auteurs de serveurs MCP
</h3>

Si vous créez un serveur MCP, le champ des instructions du serveur devient plus utile avec la recherche d'outils activée. Les instructions du serveur aident Claude à comprendre quand rechercher vos outils, de la même manière que les [skills](/docs/fr/skills) fonctionnent.

Ajoutez des instructions de serveur claires et descriptives qui expliquent :

* Quelle catégorie de tâches vos outils gèrent
* Quand Claude doit rechercher vos outils
* Les capacités clés que votre serveur fournit

Claude Code tronque chaque description d'outil et les instructions de chaque serveur à 2 048 caractères par défaut. Gardez-les concises, et placez les détails critiques près du début.

Pour modifier la limite pour chaque serveur MCP dans votre session, définissez [`CLAUDE_CODE_MAX_MCP_DESCRIPTION_LENGTH`](/docs/fr/env-vars#variables) sur un nombre de caractères. Cette variable nécessite Claude Code v2.1.280 ou ultérieur.

<h3 id="configure-tool-search">
  Configurer la recherche d'outils
</h3>

La recherche d'outils est activée par défaut : les outils MCP sont reportés et découverts à la demande. Claude Code la désactive quand `ANTHROPIC_BASE_URL` pointe vers un hôte non-propriétaire, puisque la plupart des proxies ne transmettent pas les blocs `tool_reference`. Définissez `ENABLE_TOOL_SEARCH` explicitement pour contourner ce repli.

Définir [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/fr/env-vars) maintient la recherche d'outils désactivée. Vous ne pouvez pas la contourner en définissant `ENABLE_TOOL_SEARCH` vous-même. Votre organisation peut maintenir la recherche d'outils activée via les [paramètres gérés](/docs/fr/managed-settings), sur Claude Code v2.1.227 ou ultérieur. [Désactiver les capacités de pré-version](/docs/fr/llm-gateway-protocol#disable-pre-release-capabilities) couvre où le remplacement s'applique et ce que la variable supprime.

La recherche d'outils nécessite un modèle qui prend en charge les blocs `tool_reference` : Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5, et les modèles ultérieurs. Consultez la [compatibilité des modèles dans la documentation de l'API](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-search-tool#model-compatibility) pour la liste actuelle.

Sur la plateforme Agent de Google Cloud, Claude Code décide par génération de modèle :

* **Claude Opus 4.5, Sonnet 4.5, Haiku 4.5, et ultérieur** : la recherche d'outils est activée par défaut, comme sur l'API Anthropic.
* **Modèles Agent Platform antérieurs** : Claude Code charge tous les outils MCP en amont, car leurs piles de service rejettent l'en-tête bêta requis. `ENABLE_TOOL_SEARCH=true` ne contourne pas cela.

Avant v2.1.221, Claude Code désactivait la recherche d'outils pour tous les modèles sur la plateforme Agent de Google Cloud sauf si vous définissiez `ENABLE_TOOL_SEARCH=true`.

Contrôlez le comportement de la recherche d'outils avec la variable d'environnement `ENABLE_TOOL_SEARCH` :

| Valeur       | Comportement                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| (non défini) | Tous les outils MCP reportés et chargés à la demande. Revient au chargement en amont sur les modèles de la plateforme Agent de Google Cloud antérieurs à la génération Claude 4.5, quand `ANTHROPIC_BASE_URL` est un hôte non-propriétaire, ou sur un déploiement Microsoft Foundry hébergé sur Azure                                                                                                                                                                    |
| `true`       | Tous les outils MCP reportés, sauf sur un déploiement Microsoft Foundry hébergé sur Azure, où le rejet côté serveur force toujours le chargement en amont, et sur les modèles de la plateforme Agent de Google Cloud antérieurs à la génération Claude 4.5, où Claude Code continue de charger les outils en amont. Claude Code envoie l'en-tête bêta via les proxies, et les demandes échouent sur les proxies qui ne prennent pas en charge les blocs `tool_reference` |
| `auto`       | Mode seuil : Claude Code charge les outils qu'il reporterait autrement en amont tant que leurs définitions totalisent moins de 10 % de la fenêtre de contexte, et les reporte tous une fois que les définitions atteignent 10 %                                                                                                                                                                                                                                          |
| `auto:N`     | Mode seuil avec un pourcentage personnalisé, où `N` est 0-100. Par exemple, `auto:5` pour 5 %                                                                                                                                                                                                                                                                                                                                                                            |
| `false`      | Tous les outils MCP chargés en amont, pas de report                                                                                                                                                                                                                                                                                                                                                                                                                      |

```bash theme={null}
# Utiliser un seuil personnalisé de 5 %
ENABLE_TOOL_SEARCH=auto:5 claude

# Désactiver complètement la recherche d'outils
ENABLE_TOOL_SEARCH=false claude
```

Ou définissez la valeur dans le champ `env` de votre [settings.json](/docs/fr/settings-reference#env).

Vous pouvez également désactiver l'outil `ToolSearch` spécifiquement :

```json theme={null}
{
  "permissions": {
    "deny": ["ToolSearch"]
  }
}
```

<h3 id="exempt-a-server-from-deferral">
  Exempter un serveur du report
</h3>

Si les outils d'un serveur doivent toujours être visibles pour Claude sans une étape de recherche, définissez `alwaysLoad` à `true` dans la configuration de ce serveur. Chaque outil de ce serveur se charge alors dans le contexte au démarrage de la session indépendamment du paramètre `ENABLE_TOOL_SEARCH`. Utilisez ceci pour un petit nombre d'outils que Claude doit utiliser à chaque tour, puisque chaque outil en amont consomme du contexte qui serait autrement disponible pour votre conversation.

L'entrée `.mcp.json` suivante exempte un serveur HTTP tout en laissant les autres serveurs reportés :

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

Le champ `alwaysLoad` est disponible sur tous les types de serveurs. Un serveur MCP peut également marquer les outils individuels comme toujours chargés en incluant `"anthropic/alwaysLoad": true` dans l'objet `_meta` de l'outil, ce qui a le même effet pour cet outil uniquement.

Définir `alwaysLoad: true` fait également attendre le démarrage des outils du serveur, limité au délai d'expiration de connexion standard de 5 secondes, puisqu'ils doivent être présents lors de la construction de la première invite. Un serveur distant avec une entrée [`cached`](#server-status-detail) valide fournit ses outils à partir du cache sans se connecter, donc il ne retarde pas le démarrage. Les autres serveurs se connectent en arrière-plan par défaut ; définissez [`MCP_CONNECTION_NONBLOCKING=0`](/docs/fr/env-vars) pour faire attendre le démarrage pour eux aussi.

<h2 id="use-mcp-prompts-as-commands">
  Utiliser les invites MCP comme commandes
</h2>

Les serveurs MCP peuvent exposer des invites qui deviennent disponibles en tant que commandes dans Claude Code.

<h3 id="execute-mcp-prompts">
  Exécuter les invites MCP
</h3>

<Steps>
  <Step title="Découvrir les invites disponibles">
    Tapez `/` pour voir les commandes disponibles, y compris celles des serveurs MCP. Claude Code répertorie chaque invite MCP sous la forme `/servername:promptname (MCP)`. Taper `/mcp__servername__promptname` l'exécute également.
  </Step>

  <Step title="Exécuter une invite sans arguments">
    ```text wrap theme={null}
    /mcp__github__list_prs
    ```
  </Step>

  <Step title="Exécuter une invite avec des arguments">
    De nombreuses invites acceptent des arguments. Passez-les séparés par des espaces après la commande. Claude Code divise les arguments sur les espaces, donc chaque argument est un seul jeton :

    ```text wrap theme={null}
    /mcp__github__pr_review 456
    ```

    ```text wrap theme={null}
    /mcp__jira__create_issue login-bug high
    ```
  </Step>
</Steps>

<Tip>
  Conseils :

  * Les invites MCP sont découvertes dynamiquement à partir des serveurs connectés
  * Les arguments sont analysés en fonction des paramètres définis de l'invite
  * Les résultats de l'invite sont injectés directement dans la conversation
  * Sous la forme `/mcp__servername__promptname`, Claude Code remplace tout caractère du nom du serveur en dehors de `A-Z`, `a-z`, `0-9`, `_` et `-` par `_`, et utilise le nom de l'invite tel que le serveur le déclare
</Tip>

<h2 id="managed-mcp-configuration">
  Configuration MCP gérée
</h2>

Pour les organisations qui ont besoin d'un contrôle centralisé sur les serveurs MCP auxquels les utilisateurs peuvent se connecter, consultez [Configuration MCP gérée](/docs/fr/managed-mcp). Elle couvre le déploiement d'un ensemble fixe de serveurs avec `managed-mcp.json`, la fourniture de serveurs à chaque utilisateur avec `managedMcpServers`, la restriction des serveurs avec `allowedMcpServers` et `deniedMcpServers`, et ce que les utilisateurs voient lorsqu'un serveur est bloqué.
