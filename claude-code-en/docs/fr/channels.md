> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Envoyer des événements dans une session active avec les canaux

> Utilisez les canaux pour envoyer des messages, des alertes et des webhooks dans votre session Claude Code à partir d'un serveur MCP. Transférez les résultats CI, les messages de chat et les événements de surveillance pour que Claude puisse réagir en votre absence.

<Note>
  Les canaux sont en [aperçu de recherche](#research-preview). Ils nécessitent une authentification Anthropic via claude.ai ou une clé API Console, et ne sont pas disponibles sur Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry. Les organisations Team et Enterprise doivent [les activer explicitement](#enterprise-controls).
</Note>

Un canal est un serveur MCP qui envoie des événements dans votre session Claude Code active, afin que Claude puisse réagir aux choses qui se produisent lorsque vous n'êtes pas au terminal. Les canaux peuvent être bidirectionnels : Claude lit l'événement et répond via le même canal, comme un pont de chat. Les événements n'arrivent que lorsque la session est ouverte, donc pour une configuration toujours active, vous exécutez Claude dans un processus d'arrière-plan ou un terminal persistant.

Contrairement aux intégrations qui lancent une nouvelle session cloud ou attendent d'être interrogées, l'événement arrive dans la session que vous avez déjà ouverte : voir [comment les canaux se comparent](#how-channels-compare).

Vous installez un canal en tant que plugin et le configurez avec vos propres identifiants. Telegram, Discord et iMessage sont inclus dans l'aperçu de recherche.

Lorsque Claude répond via un canal, vous voyez le message entrant dans votre terminal mais pas le texte de la réponse. Le terminal affiche l'appel d'outil et une confirmation (comme « envoyé »), et la réponse réelle apparaît sur l'autre plateforme.

Si vous gérez une organisation Team, Enterprise ou Console, consultez [Activer les canaux pour votre organisation](#enterprise-controls). Pour créer votre propre canal, consultez la [référence Canaux](/docs/fr/channels-reference).

<h2 id="supported-channels">
  Canaux pris en charge
</h2>

Chaque canal pris en charge est un plugin qui nécessite [Bun](https://bun.sh). Pour une démonstration pratique du flux de plugin avant de connecter une plateforme réelle, essayez le [démarrage rapide fakechat](#quickstart).

<Tabs>
  <Tab title="Telegram">
    Consultez la [source complète du plugin Telegram](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram).

    <Steps>
      <Step title="Créer un bot Telegram">
        Ouvrez [BotFather](https://t.me/BotFather) dans Telegram et envoyez `/newbot`. Donnez-lui un nom d'affichage et un nom d'utilisateur unique se terminant par `bot`. Copiez le jeton que BotFather retourne.
      </Step>

      <Step title="Installer le plugin">
        Dans Claude Code, exécutez :

        ```
        /plugin install telegram@claude-plugins-official
        ```

        Si Claude Code signale que le plugin n'est trouvé dans aucune marketplace, votre marketplace est soit manquante, soit obsolète. Exécutez `/plugin marketplace update claude-plugins-official` pour l'actualiser, ou `/plugin marketplace add anthropics/claude-plugins-official` si vous ne l'avez pas encore ajoutée. Ensuite, réessayez l'installation.

        Après l'installation, exécutez `/reload-plugins` pour activer la commande de configuration du plugin.
      </Step>

      <Step title="Configurer votre jeton">
        Exécutez la commande de configuration avec le jeton de BotFather :

        ```
        /telegram:configure <token>
        ```

        Cela l'enregistre dans `~/.claude/channels/telegram/.env`. Vous pouvez également définir `TELEGRAM_BOT_TOKEN` dans votre environnement shell avant de lancer Claude Code.
      </Step>

      <Step title="Redémarrer avec les canaux activés">
        Quittez Claude Code et redémarrez avec l'indicateur de canal. Cela démarre le plugin Telegram, qui commence à interroger les messages de votre bot :

        ```bash theme={null}
        claude --channels plugin:telegram@claude-plugins-official
        ```
      </Step>

      <Step title="Appairer votre compte">
        Ouvrez Telegram et envoyez n'importe quel message à votre bot. Le bot répond avec un code d'appairage.

        <Note>Si votre bot ne répond pas, assurez-vous que Claude Code s'exécute avec `--channels` à partir de l'étape précédente. Le bot ne peut répondre que lorsque le canal est actif.</Note>

        De retour dans Claude Code, exécutez :

        ```
        /telegram:access pair <code>
        ```

        Ensuite, verrouillez l'accès pour que seul votre compte puisse envoyer des messages :

        ```
        /telegram:access policy allowlist
        ```
      </Step>
    </Steps>
  </Tab>

  <Tab title="Discord">
    Consultez la [source complète du plugin Discord](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord).

    <Steps>
      <Step title="Créer un bot Discord">
        Allez au [Portail des développeurs Discord](https://discord.com/developers/applications), cliquez sur **New Application** et nommez-le. Dans la section **Bot**, créez un nom d'utilisateur, puis cliquez sur **Reset Token** et copiez le jeton.
      </Step>

      <Step title="Activer Message Content Intent">
        Dans les paramètres de votre bot, faites défiler jusqu'à **Privileged Gateway Intents** et activez **Message Content Intent**.
      </Step>

      <Step title="Inviter le bot à votre serveur">
        Allez à **OAuth2 > URL Generator**. Sélectionnez la portée `bot` et activez ces permissions :

        * View Channels
        * Send Messages
        * Send Messages in Threads
        * Read Message History
        * Attach Files
        * Add Reactions

        Ouvrez l'URL générée pour ajouter le bot à votre serveur.
      </Step>

      <Step title="Installer le plugin">
        Dans Claude Code, exécutez :

        ```
        /plugin install discord@claude-plugins-official
        ```

        Si Claude Code signale que le plugin n'est trouvé dans aucune marketplace, votre marketplace est soit manquante, soit obsolète. Exécutez `/plugin marketplace update claude-plugins-official` pour l'actualiser, ou `/plugin marketplace add anthropics/claude-plugins-official` si vous ne l'avez pas encore ajoutée. Ensuite, réessayez l'installation.

        Après l'installation, exécutez `/reload-plugins` pour activer la commande de configuration du plugin.
      </Step>

      <Step title="Configurer votre jeton">
        Exécutez la commande de configuration avec le jeton du bot que vous avez copié :

        ```
        /discord:configure <token>
        ```

        Cela l'enregistre dans `~/.claude/channels/discord/.env`. Vous pouvez également définir `DISCORD_BOT_TOKEN` dans votre environnement shell avant de lancer Claude Code.
      </Step>

      <Step title="Redémarrer avec les canaux activés">
        Quittez Claude Code et redémarrez avec l'indicateur de canal. Cela connecte le plugin Discord pour que votre bot puisse recevoir et répondre aux messages :

        ```bash theme={null}
        claude --channels plugin:discord@claude-plugins-official
        ```
      </Step>

      <Step title="Appairer votre compte">
        Envoyez un DM à votre bot sur Discord. Le bot répond avec un code d'appairage.

        <Note>Si votre bot ne répond pas, assurez-vous que Claude Code s'exécute avec `--channels` à partir de l'étape précédente. Le bot ne peut répondre que lorsque le canal est actif.</Note>

        De retour dans Claude Code, exécutez :

        ```
        /discord:access pair <code>
        ```

        Ensuite, verrouillez l'accès pour que seul votre compte puisse envoyer des messages :

        ```
        /discord:access policy allowlist
        ```
      </Step>
    </Steps>
  </Tab>

  <Tab title="iMessage">
    Consultez la [source complète du plugin iMessage](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage).

    Le canal iMessage lit votre base de données Messages directement et envoie les réponses via AppleScript. Il nécessite macOS et n'a besoin d'aucun jeton de bot ou service externe.

    <Steps>
      <Step title="Accorder l'accès complet au disque">
        La base de données Messages à `~/Library/Messages/chat.db` est protégée par macOS. La première fois que le serveur la lit, macOS demande l'accès : cliquez sur **Allow**. L'invite nomme l'application qui a lancé Bun, comme Terminal, iTerm ou votre IDE.

        Si l'invite n'apparaît pas ou si vous avez cliqué sur Don't Allow, accordez l'accès manuellement sous **System Settings > Privacy & Security > Full Disk Access** et ajoutez votre terminal. Sans cela, le serveur se ferme immédiatement avec `authorization denied`.
      </Step>

      <Step title="Installer le plugin">
        Dans Claude Code, exécutez :

        ```
        /plugin install imessage@claude-plugins-official
        ```

        Si Claude Code signale que le plugin n'est trouvé dans aucune marketplace, votre marketplace est soit manquante, soit obsolète. Exécutez `/plugin marketplace update claude-plugins-official` pour l'actualiser, ou `/plugin marketplace add anthropics/claude-plugins-official` si vous ne l'avez pas encore ajoutée. Ensuite, réessayez l'installation.
      </Step>

      <Step title="Redémarrer avec les canaux activés">
        Quittez Claude Code et redémarrez avec l'indicateur de canal :

        ```bash theme={null}
        claude --channels plugin:imessage@claude-plugins-official
        ```
      </Step>

      <Step title="Vous envoyer un message">
        Ouvrez Messages sur n'importe quel appareil connecté à votre Apple ID et envoyez-vous un message. Il atteint Claude immédiatement : l'auto-chat contourne le contrôle d'accès sans configuration.

        <Note>La première réponse que Claude envoie déclenche une invite d'automatisation macOS demandant si votre terminal peut contrôler Messages. Cliquez sur **OK**.</Note>
      </Step>

      <Step title="Autoriser d'autres expéditeurs">
        Par défaut, seuls vos propres messages passent. Pour laisser un autre contact atteindre Claude, ajoutez son identifiant :

        ```
        /imessage:access allow +15551234567
        ```

        Les identifiants sont des numéros de téléphone au format `+country` ou des e-mails Apple ID comme `user@example.com`.
      </Step>
    </Steps>
  </Tab>
</Tabs>

Vous pouvez également [créer votre propre canal](/docs/fr/channels-reference) pour les systèmes qui n'ont pas encore de plugin.

<h2 id="quickstart">
  Démarrage rapide
</h2>

Fakechat est un canal de démonstration officiellement pris en charge qui exécute une interface de chat sur localhost, sans rien à authentifier et aucun service externe à configurer.

Une fois que vous installez et activez fakechat, vous pouvez taper dans le navigateur et le message arrive dans votre session Claude Code. Claude répond, et la réponse réapparaît dans le navigateur. Après avoir testé l'interface fakechat, essayez [Telegram](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram), [Discord](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord) ou [iMessage](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage).

Pour essayer la démo fakechat, vous aurez besoin de :

* Claude Code [installé et authentifié](/docs/fr/quickstart#step-1-install-claude-code) avec un compte claude.ai ou une clé API Console
* [Bun](https://bun.sh) installé. Les plugins de canal pré-construits sont des scripts Bun. Vérifiez avec `bun --version` ; si cela échoue, [installez Bun](https://bun.sh/docs/installation).
* **Organisations Team, Enterprise ou Console gérées** : votre administrateur doit [activer les canaux](#enterprise-controls) dans les paramètres gérés

<Steps>
  <Step title="Installer le plugin de canal fakechat">
    Démarrez une session Claude Code et exécutez la commande d'installation :

    ```text theme={null}
    /plugin install fakechat@claude-plugins-official
    ```

    Si Claude Code signale que le plugin n'est trouvé dans aucune marketplace, votre marketplace est soit manquante, soit obsolète. Exécutez `/plugin marketplace update claude-plugins-official` pour l'actualiser, ou `/plugin marketplace add anthropics/claude-plugins-official` si vous ne l'avez pas encore ajoutée. Ensuite, réessayez l'installation.
  </Step>

  <Step title="Redémarrer avec le canal activé">
    Quittez Claude Code, puis redémarrez avec `--channels` et passez le plugin fakechat que vous avez installé :

    ```bash theme={null}
    claude --channels plugin:fakechat@claude-plugins-official
    ```

    Le serveur fakechat démarre automatiquement.

    <Tip>
      Vous pouvez passer plusieurs plugins à `--channels`, séparés par des espaces.
    </Tip>
  </Step>

  <Step title="Envoyer un message">
    Ouvrez l'interface fakechat à [http://localhost:8787](http://localhost:8787) et tapez un message :

    ```text theme={null}
    hey, what's in my working directory?
    ```

    Le message arrive dans votre session Claude Code en tant qu'événement `<channel source="fakechat">`. Claude le lit, fait le travail et appelle l'outil `reply` de fakechat. La réponse s'affiche dans l'interface de chat.
  </Step>
</Steps>

Si Claude rencontre une invite de permission alors que vous êtes loin du terminal, la session s'interrompt jusqu'à ce que vous répondiez. Les serveurs de canaux qui déclarent la [capacité de relais de permission](/docs/fr/channels-reference#relay-permission-prompts) peuvent vous transférer ces invites pour que vous puissiez approuver ou refuser à distance. Pour une utilisation sans surveillance, [`--dangerously-skip-permissions`](/docs/fr/permission-modes#skip-all-checks-with-bypasspermissions-mode) contourne la plupart des invites, mais utilisez-le uniquement dans les environnements auxquels vous faites confiance. Les règles d'ask explicites, les outils de connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) et les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool) continuent à afficher des invites.

Lorsque vous exécutez des canaux en mode non interactif avec `-p`, les outils qui nécessitent une entrée de terminal, tels que les questions à choix multiples et l'approbation du mode plan, sont désactivés pour que la session ne s'arrête jamais en attente d'entrée.

<h2 id="security">
  Sécurité
</h2>

Chaque plugin de canal approuvé maintient une liste blanche d'expéditeurs : seuls les identifiants que vous avez ajoutés peuvent envoyer des messages, et tous les autres sont silencieusement supprimés.

Telegram et Discord amorçent la liste par appairage :

1. Trouvez votre bot dans Telegram ou Discord et envoyez-lui n'importe quel message
2. Le bot répond avec un code d'appairage
3. Dans votre session Claude Code, approuvez le code lorsque vous y êtes invité
4. Votre identifiant d'expéditeur est ajouté à la liste blanche

iMessage fonctionne différemment : vous envoyer un message contourne automatiquement la barrière, et vous ajoutez d'autres contacts par identifiant avec `/imessage:access allow`.

En plus de cela, vous contrôlez quels serveurs sont activés à chaque session avec `--channels`, et votre organisation contrôle la disponibilité avec [`channelsEnabled`](#enterprise-controls) sur les plans Team et Enterprise de claude.ai et sur les organisations Console qui déploient des paramètres gérés.

Être dans `.mcp.json` ne suffit pas pour envoyer des messages : un serveur doit également être nommé dans `--channels`.

La liste blanche contrôle également le [relais de permission](/docs/fr/channels-reference#relay-permission-prompts) si le canal le déclare. Quiconque peut répondre via le canal peut approuver ou refuser l'utilisation d'outils dans votre session, donc n'ajoutez à la liste blanche que les expéditeurs auxquels vous faites confiance avec cette autorité.

<h2 id="enterprise-controls">
  Contrôles d'entreprise
</h2>

Les administrateurs contrôlent la disponibilité via deux [paramètres gérés](/docs/fr/settings) que les utilisateurs ne peuvent pas modifier. La valeur par défaut dépend de la façon dont vous vous authentifiez :

* **claude.ai Team et Enterprise** : les canaux sont bloqués jusqu'à ce qu'un propriétaire les active.
* **Anthropic Console avec authentification par clé API** : les canaux sont autorisés par défaut. Vous n'avez besoin de ce paramètre que si votre organisation déploie des paramètres gérés.

Dans tous les cas, aucun canal ne s'exécute jusqu'à ce qu'un utilisateur l'active pour la session avec `--channels`.

| Paramètre               | Objectif                                                                                                                                                                                                                                                                                                                           | Lorsque non configuré                                                                                                                                                                                       |
| :---------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `channelsEnabled`       | Commutateur maître. Doit être `true` pour que tout canal puisse livrer des messages. Défini via le bouton bascule de la [console Admin claude.ai](https://claude.ai/admin-settings/claude-code) ou directement dans les paramètres gérés. Bloque tous les canaux, y compris l'indicateur de développement lorsqu'il est désactivé. | claude.ai Team et Enterprise : canaux bloqués. Console : canaux autorisés sauf si votre organisation déploie des paramètres gérés, auquel cas les canaux sont bloqués jusqu'à ce que cette clé soit définie |
| `allowedChannelPlugins` | Quels plugins peuvent s'enregistrer une fois les canaux activés. Remplace la liste maintenue par Anthropic lorsqu'elle est définie. S'applique uniquement lorsque `channelsEnabled` est `true`.                                                                                                                                    | La liste par défaut d'Anthropic s'applique                                                                                                                                                                  |

Les utilisateurs Pro et Max sans organisation ignorent complètement ces vérifications : les canaux sont disponibles et les utilisateurs optent pour chaque session avec `--channels`.

<h3 id="enable-channels-for-your-organization">
  Activer les canaux pour votre organisation
</h3>

Activez les canaux pour votre organisation à partir de [**claude.ai → Admin settings → Claude Code → Channels**](https://claude.ai/admin-settings/claude-code), ce qui nécessite le rôle de propriétaire, ou en définissant `channelsEnabled` sur `true` dans les paramètres gérés.

Une fois activés, les utilisateurs de votre organisation peuvent utiliser `--channels` pour opter pour les serveurs de canaux dans les sessions individuelles. Si le paramètre est désactivé ou non défini, le serveur MCP se connecte toujours et ses outils fonctionnent, mais les messages de canal n'arriveront pas. Un avertissement au démarrage indique à l'utilisateur de demander à un administrateur d'activer le paramètre.

<h3 id="restrict-which-channel-plugins-can-run">
  Restreindre les plugins de canal qui peuvent s'exécuter
</h3>

Par défaut, tout plugin sur la liste blanche maintenue par Anthropic peut s'enregistrer en tant que canal. Les administrateurs sur les plans Team et Enterprise peuvent remplacer cette liste blanche par la leur en définissant `allowedChannelPlugins` dans les paramètres gérés. Utilisez ceci pour restreindre les plugins officiels autorisés, approuver les canaux de votre propre marketplace interne, ou les deux. Chaque entrée nomme un plugin et la marketplace d'où il provient :

```json theme={null}
{
  "channelsEnabled": true,
  "allowedChannelPlugins": [
    { "marketplace": "claude-plugins-official", "plugin": "telegram" },
    { "marketplace": "claude-plugins-official", "plugin": "discord" },
    { "marketplace": "acme-corp-plugins", "plugin": "internal-alerts" }
  ]
}
```

Lorsque `allowedChannelPlugins` est défini, il remplace complètement la liste blanche d'Anthropic : seuls les plugins listés peuvent s'enregistrer. Laissez-le non défini pour revenir à la liste blanche par défaut d'Anthropic. Un tableau vide bloque tous les plugins de canal de la liste blanche, mais `--dangerously-load-development-channels` peut toujours le contourner pour les tests locaux. Pour bloquer complètement les canaux, y compris l'indicateur de développement, laissez plutôt `channelsEnabled` non défini.

Ce paramètre nécessite `channelsEnabled: true`. Si un utilisateur transmet un plugin à `--channels` qui ne figure pas sur votre liste, Claude Code démarre normalement mais le canal ne s'enregistre pas, et l'avis de démarrage explique que le plugin ne figure pas sur la liste approuvée de l'organisation.

<h2 id="research-preview">
  Aperçu de recherche
</h2>

Les canaux sont une fonctionnalité d'aperçu de recherche. La disponibilité est déployée progressivement, et la syntaxe de l'indicateur `--channels` et le contrat de protocole peuvent changer en fonction des commentaires.

Pendant l'aperçu, `--channels` n'accepte que les plugins d'une liste blanche maintenue par Anthropic, ou de la liste blanche de votre organisation si un administrateur a défini [`allowedChannelPlugins`](#restrict-which-channel-plugins-can-run). Les plugins de canal dans [claude-plugins-official](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins) sont l'ensemble approuvé par défaut. Si vous passez quelque chose qui ne figure pas sur la liste blanche effective, Claude Code démarre normalement mais le canal ne s'enregistre pas, et l'avis de démarrage vous indique pourquoi.

Pour tester un canal que vous créez, utilisez `--dangerously-load-development-channels`. Consultez [Test during the research preview](/docs/fr/channels-reference#test-during-the-research-preview) pour des informations sur le test des canaux personnalisés que vous créez.

Signalez les problèmes ou les commentaires sur le [référentiel GitHub Claude Code](https://github.com/anthropics/claude-code/issues).

<h2 id="how-channels-compare">
  Comment les canaux se comparent
</h2>

Plusieurs fonctionnalités de Claude Code se connectent à des systèmes en dehors du terminal, chacune adaptée à un type de travail différent :

| Fonctionnalité                                       | Ce qu'elle fait                                                                           | Bonne pour                                                                    |
| ---------------------------------------------------- | ----------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| [Claude Code sur le web](/docs/fr/claude-code-on-the-web) | Exécute les tâches dans un nouveau bac à sable cloud, cloné à partir de GitHub            | Déléguer le travail asynchrone autonome que vous vérifiez plus tard           |
| [Claude dans Slack](/docs/fr/slack)                       | Lance une session web à partir d'une mention `@Claude` dans un canal ou un fil            | Démarrer les tâches directement à partir du contexte de conversation d'équipe |
| Serveur [MCP](/docs/fr/mcp) standard                      | Claude l'interroge pendant une tâche ; rien n'est envoyé à la session                     | Donner à Claude un accès à la demande pour lire ou interroger un système      |
| [Remote Control](/docs/fr/remote-control)                 | Vous pilotez votre session locale à partir de claude.ai ou de l'application mobile Claude | Diriger une session en cours pendant que vous êtes loin de votre bureau       |

Les canaux comblent le vide dans cette liste en envoyant des événements de sources non-Claude dans votre session locale déjà active.

* **Pont de chat** : posez une question à Claude à partir de votre téléphone via Telegram, Discord ou iMessage, et la réponse revient dans le même chat pendant que le travail s'exécute sur votre machine par rapport à vos fichiers réels.
* **[Récepteur webhook](/docs/fr/channels-reference#example-build-a-webhook-receiver)** : un webhook de CI, votre suivi d'erreurs, un pipeline de déploiement ou un autre service externe arrive où Claude a déjà vos fichiers ouverts et se souvient de ce que vous déboguiez.

<h2 id="next-steps">
  Étapes suivantes
</h2>

Une fois que vous avez un canal en cours d'exécution, explorez ces fonctionnalités connexes :

* [Créer votre propre canal](/docs/fr/channels-reference) pour les systèmes qui n'ont pas encore de plugins
* [Remote Control](/docs/fr/remote-control) pour piloter une session locale à partir de votre téléphone au lieu de transférer des événements dans celle-ci
* [Tâches planifiées](/docs/fr/scheduled-tasks) pour interroger selon un minuteur au lieu de réagir aux événements envoyés
