> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Se connecter aux serveurs MCP

> Ajoutez un serveur MCP à Claude Code, vérifiez la connexion et trouvez la configuration sur le disque.

Le [Model Context Protocol (MCP)](https://modelcontextprotocol.io/introduction) permet à Claude Code d'utiliser des outils au-delà de son ensemble intégré, comme la recherche dans un suivi de problèmes, l'interrogation d'une base de données ou le contrôle d'un navigateur web. Ces outils proviennent de serveurs MCP, qui s'exécutent sur votre machine ou en tant que services hébergés.

Ce guide vous montre comment connecter un serveur MCP de bout en bout avec l'interface de ligne de commande Claude Code. À la fin, vous aurez un serveur connecté et réactif, vous saurez où sa configuration se trouve sur le disque et vous saurez comment corriger les erreurs de connexion les plus courantes.

<Note>
  Vous pouvez également ajouter des serveurs MCP à partir d'autres surfaces, notamment l'application de bureau, VS Code et le web. Voir [Se connecter à partir d'autres surfaces](#connect-from-other-surfaces).
</Note>

Pour tous les moyens de connecter et configurer les serveurs MCP dans Claude Code, consultez la [référence MCP](/docs/fr/mcp).

<h2 id="before-you-begin">
  Avant de commencer
</h2>

Assurez-vous que vous avez :

* [Claude Code installé](/docs/fr/quickstart) et authentifié
* Un terminal ouvert dans un répertoire de projet. N'importe quel répertoire fonctionne, y compris un répertoire vide.

<h2 id="add-and-verify-a-server">
  Ajouter et vérifier un serveur
</h2>

L'exemple ci-dessous se connecte au [serveur MCP de documentation Claude Code](https://code.claude.com/docs/mcp), un serveur hébergé avec recherche en texte intégral sur la documentation Claude Code. Il ne nécessite pas d'authentification ni de configuration spéciale, il fonctionne donc bien comme premier serveur pour tester le flux de configuration.

Les étapes sont les mêmes pour n'importe quel serveur : l'ajouter, vérifier l'état de la connexion, puis l'utiliser dans une session, avec une étape de nettoyage optionnelle à la fin. Certains serveurs ajoutent une étape, comme une connexion au navigateur, présentée dans [Exemples de serveurs MCP supplémentaires](#additional-mcp-server-examples). Pour plus de serveurs à connecter, parcourez le [Répertoire Anthropic](/docs/fr/mcp#find-and-build-mcp-servers).

<Steps>
  <Step title="Ajouter le serveur MCP">
    Enregistrez le serveur avec Claude Code. Exécutez ceci dans votre terminal, pas à l'intérieur d'une session `claude` : vous configurez le serveur avant de démarrer une conversation.

    ```bash theme={null}
    claude mcp add --transport http claude-code-docs https://code.claude.com/docs/mcp
    ```

    Les parties de la commande :

    * `claude mcp add` : enregistre un serveur avec Claude Code.
    * `--transport http` : le serveur est hébergé à une URL plutôt que d'être exécuté en tant que processus local.
    * `claude-code-docs` : un nom que vous inventez. Appeler le même serveur `docs` fonctionnerait de manière identique. Claude Code utilise le nom que vous choisissez pour étiqueter les outils du serveur dans la sortie de Claude et pour faire référence au serveur dans des commandes comme `claude mcp remove`.
    * `https://code.claude.com/docs/mcp` : l'URL où le serveur est hébergé.

    La commande affiche une confirmation comme `Added HTTP MCP server claude-code-docs with URL: https://code.claude.com/docs/mcp to local config`. La partie `local config` signifie que le serveur est enregistré pour vous, dans ce projet : si vous démarrez Claude Code dans un projet différent, ce serveur n'est pas actif là-bas. Pour enregistrer un serveur une fois pour tous vos projets, ajoutez-le à la portée utilisateur, couverte dans [Modifier la portée du serveur](#change-server-scope).
  </Step>

  <Step title="Vérifier l'état de la connexion">
    Confirmez que le serveur apparaît dans votre liste de serveurs et vérifiez son état :

    ```bash theme={null}
    claude mcp list
    ```

    Le serveur apparaît avec un indicateur d'état :

    | État                               | Signification                                                                                                                                                                                                |
    | :--------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `✓ Connected`                      | Prêt à être utilisé. C'est ce que vous devriez voir pour `claude-code-docs`                                                                                                                                  |
    | `! Connected · tools fetch failed` | Le serveur s'est connecté mais n'a pas pu lister ses outils. Exécutez `claude mcp get <name>` pour le détail de l'erreur                                                                                     |
    | `! Needs authentication`           | Le serveur est accessible mais nécessite une connexion au navigateur, ou un jeton transmis avec `--header`. Voir [Connecter un serveur qui nécessite une connexion](#connect-a-server-that-requires-sign-in) |
    | `✗ Failed to connect`              | Le serveur n'a pas répondu. Voir [Dépannage](#troubleshooting)                                                                                                                                               |
    | `✗ Connection error`               | La tentative de connexion a levé une erreur. Voir [Dépannage](#troubleshooting)                                                                                                                              |
    | `⏸ Pending approval`               | Un serveur à portée de projet que vous n'avez pas encore approuvé. Voir [Modifier .mcp.json directement](#edit-mcp-json-directly)                                                                            |
  </Step>

  <Step title="Utiliser le serveur">
    Démarrez une session et demandez à Claude d'utiliser le nouveau serveur par son nom :

    ```bash theme={null}
    claude
    ```

    ```text theme={null}
    Use the claude-code-docs server to look up what MCP_TIMEOUT does
    ```

    <Info>
      Vous n'avez normalement pas besoin de nommer un serveur dans votre invite, car Claude choisit les outils pertinents de lui-même. Le nommer ici garantit que la démonstration passe par le nouveau serveur plutôt que par un autre outil, comme la récupération web, qui pourrait répondre à la même question.
    </Info>

    La première fois que Claude appelle le serveur, il demande la permission d'utiliser le nouvel outil. Approuvez-le pour continuer. L'appel d'outil dans la sortie de Claude est étiqueté avec le nom du serveur, ce qui vous permet de confirmer que la réponse provient du serveur MCP plutôt que de la connaissance intégrée de Claude.
  </Step>

  <Step title="Supprimer le serveur">
    Cette étape est optionnelle. Lorsque vous avez terminé les expériences, vous pouvez supprimer le serveur :

    ```bash theme={null}
    claude mcp remove claude-code-docs
    ```

    <Note>
      Chaque serveur connecté prend de l'espace dans la [fenêtre de contexte de Claude](/docs/fr/how-claude-code-works#the-context-window) car les noms d'outils et les instructions du serveur se chargent dans chaque session. La suppression des serveurs que vous n'utilisez plus garde cet espace libre.
    </Note>
  </Step>
</Steps>

<h2 id="where-servers-are-saved">
  Où les serveurs sont enregistrés
</h2>

La commande `claude mcp add` écrit les détails du serveur dans un fichier de configuration. Par défaut, elle enregistre le serveur à la portée `local` : privé pour vous, actif uniquement dans le projet actuel. Passez `--scope user` pour l'enregistrer une fois pour tous vos projets, ou `--scope project` pour le partager avec vos coéquipiers. [Modifier la portée du serveur](#change-server-scope) explique les deux.

<Note>
  `claude mcp add` fonctionne de la même manière dans chaque shell, y compris PowerShell et Command Prompt. À l'intérieur d'une session `claude`, utilisez la commande `/mcp` pour vérifier et gérer les serveurs que vous avez déjà ajoutés.
</Note>

Il existe d'autres façons d'ajouter un serveur, chacune couverte plus loin sur cette page :

* [Ajouter un serveur local](#add-a-local-server) : exécutez un programme sur votre machine au lieu de vous connecter à une URL.
* [Modifier `.mcp.json` directement](#edit-mcp-json-directly) : écrivez l'entrée JSON vous-même au lieu d'utiliser la commande.
* [Connecter un serveur qui nécessite une connexion](#connect-a-server-that-requires-sign-in) : ajoutez un serveur hébergé qui nécessite une connexion au navigateur avant que ses outils fonctionnent.

<h3 id="find-your-configuration-on-disk">
  Trouvez votre configuration sur le disque
</h3>

La commande `claude mcp add` écrit le serveur dans l'une des trois portées, stockées dans deux fichiers, selon l'indicateur `--scope`. Vous n'avez pas besoin de modifier ces fichiers directement, mais savoir où ils se trouvent aide au débogage et au contrôle de version.

| Portée    | Fichier                                                        | Disponible pour                                           |
| :-------- | :------------------------------------------------------------- | :-------------------------------------------------------- |
| `local`   | `~/.claude.json`, sous l'entrée de ce projet                   | Seulement vous, seulement ce projet. La valeur par défaut |
| `project` | `.mcp.json` à la racine de votre projet                        | Tous ceux qui clonent le projet                           |
| `user`    | `~/.claude.json`, sous la clé `mcpServers` de niveau supérieur | Seulement vous, tous les projets                          |

Sous Windows, `~/.claude.json` se résout en `%USERPROFILE%\.claude.json`, généralement `C:\Users\YourName\.claude.json`. Si vous avez défini [`CLAUDE_CONFIG_DIR`](/docs/fr/env-vars), Claude Code lit `.claude.json` à partir de ce répertoire à la place.

Exécutez `claude mcp get claude-code-docs` pour voir quelle portée contient la définition d'un serveur. Pour savoir comment les portées interagissent lorsque le même serveur est défini dans plus d'une, voir [Portées d'installation MCP](/docs/fr/mcp#mcp-installation-scopes).

<h2 id="change-server-scope">
  Modifier la portée du serveur
</h2>

La portée d'un serveur est fixée lorsque vous l'ajoutez, donc modifier la portée signifie supprimer l'entrée et la rajouter à la nouvelle. Les deux cas ci-dessous commencent par supprimer l'entrée locale de la première procédure pas à pas, afin que le serveur n'ait qu'une seule définition. Si vous l'avez déjà supprimé à la fin de cette procédure pas à pas, ignorez cette commande :

```bash theme={null}
claude mcp remove claude-code-docs --scope local
```

<h3 id="use-a-server-in-all-your-projects">
  Utiliser un serveur dans tous vos projets
</h3>

Rajoutez le serveur à la portée `user` pour le rendre actif dans chaque projet que vous ouvrez, toujours privé pour vous :

```bash theme={null}
claude mcp add --scope user --transport http claude-code-docs https://code.claude.com/docs/mcp
```

<h3 id="share-a-server-with-your-team">
  Partager un serveur avec votre équipe
</h3>

Rajoutez le serveur à la portée `project`, qui écrit dans `.mcp.json` à la racine du projet :

```bash theme={null}
claude mcp add --scope project --transport http claude-code-docs https://code.claude.com/docs/mcp
```

Validez `.mcp.json` dans le contrôle de version. Les coéquipiers qui clonent le référentiel et démarrent Claude Code voient une invite pour approuver le serveur, puis il se connecte pour eux aussi.

<h2 id="additional-mcp-server-examples">
  Exemples de serveurs MCP supplémentaires
</h2>

La première procédure pas à pas utilisait un serveur hébergé qui se connecte sans aucune connexion. Les exemples ci-dessous couvrent les deux autres formes courantes, avec le même flux d'ajout, de vérification et d'utilisation.

<h3 id="add-a-local-server">
  Ajouter un serveur local
</h3>

Un serveur stdio local est un programme que Claude Code démarre en tant que sous-processus sur votre machine, plutôt qu'un service auquel il accède via une URL. Utilisez-en un pour les outils qui ont besoin d'accès à des ressources locales comme un navigateur, votre système de fichiers ou une socket de base de données.

Le [serveur MCP Playwright](https://github.com/microsoft/playwright-mcp) est un bon à essayer : il donne à Claude un navigateur qu'il peut naviguer, cliquer et lire, et il ne nécessite aucun compte. Il s'exécute via `npx`, il nécessite donc [Node.js](https://nodejs.org/en/download) 18 ou version ultérieure.

<Steps>
  <Step title="Ajouter le serveur Playwright">
    Enregistrez le serveur avec la commande que Claude Code doit exécuter pour le démarrer :

    ```bash theme={null}
    claude mcp add playwright -- npx -y @playwright/mcp@latest
    ```

    Cette commande diffère de l'exemple hébergé de trois façons :

    * Il n'y a pas d'indicateur `--transport`, car les serveurs locaux utilisent le transport `stdio` par défaut.
    * Tout ce qui suit le séparateur `--` est la commande que Claude Code exécute pour démarrer le serveur.
    * `-y` indique à `npx` d'installer le package sans demander confirmation.

    Playwright pilote quel que soit le Chrome déjà installé sur votre machine. Pour utiliser un navigateur différent, ajoutez `--browser` avec le nom du navigateur, par exemple `--browser firefox`, après `@playwright/mcp@latest`.
  </Step>

  <Step title="Vérifier la connexion">
    La confirmation `Added` signifie que l'entrée a été enregistrée, pas que la commande s'exécute. Vérifiez la connexion :

    ```bash theme={null}
    claude mcp list
    ```

    La première vérification peut afficher `✗ Failed to connect` pendant que `npx` télécharge le package, attendez donc un moment et exécutez-la à nouveau.
  </Step>

  <Step title="Utiliser le navigateur">
    Donnez à Claude une tâche qui nécessite le navigateur :

    ```text theme={null}
    Use playwright to open https://example.com and tell me the page title
    ```

    Une fenêtre de navigateur s'ouvre pour que vous puissiez la regarder fonctionner, et les appels d'outils dans la sortie de Claude sont étiquetés avec le nom du serveur `playwright` et l'action, comme `browser_navigate`.

    Essayez de le pointer vers votre serveur de développement local pour vérifier qu'une page s'affiche toujours après une modification, ou faites-le parcourir un rapport de bogue étape par étape.
  </Step>
</Steps>

<h3 id="connect-a-server-that-requires-sign-in">
  Connecter un serveur qui nécessite une connexion
</h3>

Les services hébergés comme Sentry, Linear et Notion exécutent leurs serveurs MCP derrière OAuth : vous ajoutez l'URL du serveur, puis vous vous connectez via votre navigateur.

Les étapes ci-dessous utilisent Sentry comme exemple. Pour connecter un service différent, remplacez son URL, que vous pouvez trouver dans le [Répertoire Anthropic](/docs/fr/mcp#find-and-build-mcp-servers) ou la documentation du service.

<Steps>
  <Step title="Ajouter le serveur">
    La commande `add` est la même que pour le serveur de documentation, avec l'URL de Sentry :

    ```bash theme={null}
    claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
    ```

    Après l'ajout, `claude mcp list` affiche le serveur avec `! Needs authentication`. C'est attendu : l'étape suivante complète la connexion.
  </Step>

  <Step title="S'authentifier dans votre navigateur">
    Démarrez une session Claude Code et ouvrez le panneau MCP :

    ```text theme={null}
    /mcp
    ```

    Sélectionnez `sentry` dans la liste, appuyez sur Entrée et choisissez `Authenticate`. Votre navigateur s'ouvre sur la page de connexion de Sentry. Approuvez la connexion là-bas.

    De retour dans Claude Code, l'état du serveur passe à connecté. Si la connexion échoue ou que le navigateur ne s'ouvre pas, voir [Dépannage](#troubleshooting).
  </Step>

  <Step title="Utiliser le serveur">
    Demandez à Claude quelque chose qui nécessite le service, comme `What Sentry projects do I have access to?`, et recherchez les appels d'outils étiquetés avec le nom du serveur `sentry` dans sa sortie.
  </Step>
</Steps>

Les serveurs qui s'authentifient avec un jeton statique au lieu d'OAuth prennent le jeton au moment de l'ajout avec `--header "Authorization: Bearer <token>"`. Voir l'[exemple GitHub](/docs/fr/mcp#example-connect-to-github-for-code-reviews) pour une version travaillée.

<h2 id="edit-mcp-json-directly">
  Modifier .mcp.json directement
</h2>

Chaque fichier du [tableau des portées](#find-your-configuration-on-disk) utilise le même format JSON pour les entrées de serveur. Cette section modifie `.mcp.json`, le fichier à portée de projet. C'est celui qui vaut le plus la peine d'être écrit à la main car il est archivé dans le référentiel, où il sert également de configuration en tant que code pour votre équipe.

Créez `.mcp.json` à la racine de votre projet. L'exemple ci-dessous définit les deux serveurs de ce guide, le serveur de documentation hébergé accessible via HTTP et le serveur Playwright en tant que processus `stdio` local :

```json theme={null}
{
  "mcpServers": {
    "claude-code-docs": {
      "type": "http",
      "url": "https://code.claude.com/docs/mcp"
    },
    "playwright": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "@playwright/mcp@latest"]
    }
  }
}
```

Les champs diffèrent selon le type de serveur :

* Pour les serveurs HTTP, `url` est le point de terminaison auquel Claude Code se connecte.
* Pour les serveurs stdio, `command` et `args` sont le programme qu'il exécute.

Après avoir enregistré le fichier, démarrez une nouvelle session Claude Code dans le projet. Claude Code lit `.mcp.json` au démarrage.

La première fois que Claude Code voit un serveur à portée de projet, il vous demande de l'approuver. L'invite existe pour qu'un référentiel que vous clonez ne puisse pas lancer de processus sur votre machine sans votre consentement. Approuvez l'invite, ou exécutez `/mcp` pour approuver plus tard si vous l'avez manquée.

Une fois approuvé, exécutez `/mcp` et vérifiez que les serveurs s'affichent comme connectés. Si l'un affiche une erreur à la place, voir [Dépannage](#troubleshooting).

<h2 id="connect-from-other-surfaces">
  Se connecter à partir d'autres surfaces
</h2>

Ce guide utilise les commandes CLI `claude mcp`, mais chaque surface Claude Code peut se connecter aux serveurs MCP :

* **Application de bureau Claude Code** : ajoutez des serveurs via l'[interface utilisateur des connecteurs](/docs/fr/desktop#connect-external-tools).
* **Application de chat Claude Desktop** : une application distincte de Claude Code. Pour copier les serveurs de son `claude_desktop_config.json` dans l'interface de ligne de commande, exécutez `claude mcp add-from-claude-desktop` sur macOS ou WSL.
* **VS Code** : voir [Se connecter aux outils externes avec MCP](/docs/fr/vs-code#connect-to-external-tools-with-mcp).
* **Claude Code sur le web** : lit `.mcp.json` à partir de votre référentiel. Voir [Modifier .mcp.json directement](#edit-mcp-json-directly).
* **Claude.ai** : les connecteurs que vous ajoutez à [claude.ai/customize/connectors](https://claude.ai/customize/connectors) se chargent automatiquement dans l'interface de ligne de commande lorsque vous vous connectez avec ce compte. Voir [Utiliser les serveurs MCP de Claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai).

<h2 id="troubleshooting">
  Dépannage
</h2>

Si un serveur ne se connecte pas, vérifiez son état avec `/mcp` à l'intérieur d'une session ou `claude mcp list` à partir de votre shell, puis faites correspondre le symptôme ci-dessous. Le panneau `/mcp` vous permet également de vous reconnecter ou de vous authentifier sans quitter la session.

<AccordionGroup>
  <Accordion title="/mcp shows No MCP servers configured">
    Claude Code n'a trouvé aucun serveur pour le répertoire actuel. Les causes les plus courantes :

    * Vous avez exécuté `claude mcp add` à partir d'un projet différent. Les serveurs à portée locale sont liés au projet où vous les avez ajoutés : la racine du référentiel, ou le répertoire exact si vous n'étiez pas dans un référentiel git. Rajoutez le serveur à partir du projet dans lequel vous êtes maintenant, ou ajoutez-le avec `--scope user` pour qu'il ne soit pas lié à un projet.
    * Vous avez modifié un fichier de configuration au mauvais chemin. Les fichiers corrects sont `~/.claude.json` et `<project>/.mcp.json`. Claude Code ne lit pas les chemins tels que `~/.claude/.mcp.json`, `~/.claude/config/mcp.json`, `~/.claude/mcp.json`, ou `%APPDATA%\Claude\mcp.json`. Pour les serveurs à portée utilisateur, exécutez `claude mcp add --scope user`, qui écrit dans la clé `mcpServers` dans `~/.claude.json` ; pour les serveurs à portée projet, modifiez `.mcp.json` à la racine du projet.
  </Accordion>

  <Accordion title="Status shows Failed to connect or Connection error">
    Les deux états signifient que le serveur n'a pas démarré ou que l'URL n'a pas répondu. Ils peuvent également apparaître pour les serveurs HTTP qui s'attendent à un jeton plutôt qu'à la connexion au navigateur couverte dans [Connecter un serveur qui nécessite une connexion](#connect-a-server-that-requires-sign-in).

    À partir de la v2.1.191, un serveur HTTP qui retourne `404 Not Found` affiche `MCP endpoint not found at <url>. Check the URL in your MCP config.` lorsque vous sélectionnez le serveur dans `/mcp`, avec l'URL que Claude Code a essayée. Les versions antérieures affichent un message générique `Error POSTing to endpoint` sans l'URL. Comparez l'URL au chemin du point de terminaison MCP documenté du serveur, puis exécutez `claude mcp remove <name>` et rajoutez avec l'URL correcte.

    Pour les serveurs HTTP, confirmez que l'URL est accessible à partir de votre machine :

    ```bash theme={null}
    curl -I https://mcp.sentry.dev/mcp
    ```

    Dans PowerShell, utilisez `curl.exe` au lieu de `curl` pour que la demande aille au vrai binaire curl plutôt qu'à l'alias `Invoke-WebRequest`.

    La réponse vous indique quel type de problème vous avez :

    * Un `404` ou `405` : le serveur est actif. De nombreux points de terminaison MCP répondent uniquement aux demandes POST, donc cela confirme toujours que l'URL est accessible à partir de votre machine.
    * Un `401` ou `403` : le serveur est actif et vous devez vous authentifier. Utilisez la connexion au navigateur dans [Connecter un serveur qui nécessite une connexion](#connect-a-server-that-requires-sign-in), ou pour les serveurs qui prennent un jeton à la place, comme celui de GitHub, transmettez-le avec `--header "Authorization: Bearer <token>"` sur la commande `claude mcp add`.
    * Aucune réponse du tout : vérifiez l'URL et votre réseau.

    Pour les serveurs stdio, exécutez la commande configurée directement dans votre terminal pour voir l'erreur sous-jacente. Pour le serveur Playwright de ce guide, exécutez :

    ```bash theme={null}
    npx -y @playwright/mcp@latest
    ```

    Ce qui se passe ensuite vous indique où se trouve le problème :

    * La commande démarre et attend l'entrée : le serveur lui-même fonctionne. Exécutez `claude mcp get <name>` et confirmez que la commande affichée là correspond à ce que vous venez d'exécuter. Si la commande affichée diffère de ce que vous avez tapé, vous avez probablement omis le séparateur `--` avant la commande du serveur. Supprimez le serveur et rajoutez-le avec `--` en place. Si vous avez écrit `.mcp.json` à la main, vérifiez sa syntaxe et son emplacement.
    * La commande génère une erreur : le message indique ce qui manque, comme Node.js ou un navigateur.
  </Accordion>

  <Accordion title="Connection timed out at startup">
    Le serveur a pris plus que le délai d'expiration de démarrage par défaut de 30 secondes. La première exécution d'un serveur stdio peut être lente pendant que `npx` télécharge le package. Augmentez la limite avec la variable d'environnement [`MCP_TIMEOUT`](/docs/fr/env-vars), en millisecondes :

    ```bash theme={null}
    MCP_TIMEOUT=60000 claude
    ```

    Dans PowerShell, définissez la variable avant la commande sur la même ligne :

    ```powershell theme={null}
    $env:MCP_TIMEOUT = "60000"; claude
    ```
  </Accordion>

  <Accordion title="Server already exists">
    Vous avez déjà ajouté un serveur avec ce nom à la même portée. Supprimez l'entrée existante ou choisissez un nom différent :

    ```bash theme={null}
    claude mcp remove claude-code-docs
    ```

    Si le nom existe à plus d'une portée, `remove` signale `exists in multiple scopes`. Passez `--scope` pour choisir quelle copie supprimer, par exemple `claude mcp remove claude-code-docs --scope local`.
  </Accordion>

  <Accordion title="Server connects but no tools appear">
    Exécutez `/mcp` à l'intérieur d'une session et sélectionnez le serveur pour voir sa liste d'outils. Si la liste est vide, le serveur a démarré mais n'a enregistré aucun outil, ce qui signifie généralement qu'il manque une variable d'environnement requise comme une clé API.

    Transmettez la variable avec `--env KEY=value` sur `claude mcp add`, ou dans le champ `env` de l'entrée `.mcp.json` du serveur. La documentation du serveur énumère les variables dont il a besoin.
  </Accordion>

  <Accordion title="Changes to .mcp.json don't take effect">
    Claude Code lit `.mcp.json` au démarrage de la session. Quittez et redémarrez la session après avoir modifié le fichier.

    Si vos serveurs n'apparaissent toujours pas, exécutez `/mcp` et recherchez un avertissement d'analyse. Claude Code ignore les entrées mal formées et affiche le champ offensant là.

    Si vous avez précédemment rejeté le serveur lorsqu'on vous l'a demandé, réinitialisez les approbations du projet :

    ```bash theme={null}
    claude mcp reset-project-choices
    ```
  </Accordion>

  <Accordion title="OAuth sign-in fails or browser doesn't open">
    Exécutez `/mcp`, sélectionnez le serveur et choisissez `Authenticate` à nouveau. Si le navigateur ne s'ouvre pas automatiquement, copiez l'URL affichée dans le terminal et ouvrez-la manuellement. Voir [S'authentifier avec les serveurs MCP distants](/docs/fr/mcp#authenticate-with-remote-mcp-servers) pour les ports de rappel fixes et les identifiants préconfigurés.
  </Accordion>
</AccordionGroup>

<h2 id="next-steps">
  Étapes suivantes
</h2>

Avec un serveur connecté, explorez le reste de ce que MCP permet :

* [Trouver plus de serveurs MCP](/docs/fr/mcp#find-and-build-mcp-servers) dans le Répertoire Anthropic
* [Partager les serveurs avec votre équipe](/docs/fr/mcp#mcp-installation-scopes) en utilisant les portées d'installation
* [Gérer l'accès MCP pour une organisation](/docs/fr/managed-mcp) avec les paramètres gérés et les contrôles de politique
* [Référencer les ressources MCP](/docs/fr/mcp#use-mcp-resources) dans les invites avec les mentions @
* [Exécuter les invites MCP en tant que commandes](/docs/fr/mcp#use-mcp-prompts-as-commands) à partir du menu `/`
* [Créer votre propre serveur](https://modelcontextprotocol.io/quickstart/server) avec le SDK MCP
